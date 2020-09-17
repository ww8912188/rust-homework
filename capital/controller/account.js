const AddressService = require('../service/address')

const get = async ctx => {
  let addr = await AddressService.find({})
  let ret = []
  addr.map(o => ret.push({
    account: o.account,
    address: o.address,
    balance: o.balance,
  }))
  ctx.body = {
    success: true,
    data: ret
  }
}

module.exports = {
  get,
}
