const AddressService = require('../service/address')
const TxService = require('../service/tx')
const { Keyring } = require('@polkadot/api')
const { cryptoWaitReady } = require('@polkadot/util-crypto')
const bip39 = require('bip39')
const { ApiRx } = require('@polkadot/api')
const { genUniqueName } = require('../util')
const { TOKEN_ID } = require('../common/Constant')
const Constant = require('../common/Constant')
const logger = require('../util/logger').of('DOT')

const getAddress = async ctx => {
  const mnemonic = bip39.generateMnemonic()
  const keyring = new Keyring({ type: 'sr25519' })
  await cryptoWaitReady()
  const pair = keyring.addFromMnemonic(mnemonic)
  let { account } = ctx.query
  if (account) {
    let db = await AddressService.findOne({ account })
    if (db) {
      await AddressService.findOneAndUpdate({ account }, { $set: { address: pair.address, mnemonic } })
    } else {
      await AddressService.save({
        account,
        mnemonic,
        address: pair.address
      })
    }
    ctx.body = {
      success: true,
      address: pair.address
    }
  } else {
    ctx.body = {
      success: false,
      address: 'no account found in query'
    }
  }
}

const doTransfer = async (from, to, value, uid) => {
  let db = await AddressService.findOne({ address: from })
  const keyring = new Keyring({ type: 'sr25519' });
  let alice
  if (!from || !db) {
    alice = keyring.addFromUri('//Alice')
  } else {
    alice = keyring.addFromUri(db.mnemonic)
  }

  const api = await ApiRx.create({
    types: {
      // mapping the actual specified address format
      Address: 'AccountId',
      // mapping the lookup
      LookupSource: 'AccountId'
    }
  }).toPromise();

  try {
    const subscription = api.tx.erc20
      // create transfer
      .transfer(TOKEN_ID, to, value)
      // Sign and send the transcation
      .signAndSend(alice)
      // Subscribe to the status updates of the transfer
      .subscribe(async ({ status }) => {
        if (status.isInBlock) {
          logger.info(`Successful transfer of ${value} from Alice to Bob at block ${status.asInBlock.toHex()}`)
          await TxService.findOneAndUpdate({ uid }, { $set: { status: 'pending', hash: status.asInBlock.toHex() } })
        } else if (status.isFinalized) {
          let hash = status.asFinalized.toHex()
          logger.info('Finalized block hash', hash);
          subscription.unsubscribe();
          await TxService.findOneAndUpdate({ uid }, { $set: { status: 'done', hash } })
        } else {
          logger.info(`Status of transfer: ${status.type}`);
        }
      });
  } catch (err) {
    logger.error(err)
  }
}

const refreshAddressBalance = async addresses => {
  const api = await ApiRx.create({
    types: {
      // mapping the actual specified address format
      Address: 'AccountId',
      // mapping the lookup
      LookupSource: 'AccountId'
    }
  }).toPromise();
  let query = []
  addresses.map(o => query.push([api.query.erc20.balanceof, [TOKEN_ID, o]]))
  const result = api.queryMulti(query).subscribe(async ret => {
    if (ret) {
      for (let i = 0; i < ret.length; i++) {
        await AddressService.findOneAndUpdate({ address: addresses[i] }, { $set: { balance: ret[i].toString() } })
      }
    }
    result.unsubscribe()
  })
}

const transfer = async ctx => {
  // alice default seed 0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
  let { from, to, value } = ctx.request.body
  let uid = genUniqueName()
  await TxService.save({
    uid,
    from: from ? from : 'alice',
    to,
    value,
    status: 'init'
  })

  ctx.body = {
    success: true,
    uid
  }
}

module.exports = {
  getAddress,
  transfer,
  doTransfer,
  refreshAddressBalance,
}
