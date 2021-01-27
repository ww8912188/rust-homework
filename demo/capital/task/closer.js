const Agenda = require('agenda');
const Constant = require('../common/Constant')
const TxService = require('../service/tx')
const AddressService = require('../service/address')
const { refreshAddressBalance } = require('../controller/dot')
const { sleepSeconds } = require('../util/time')
const { ApiRx } = require('@polkadot/api')
const logger = require('../util/logger').of('Outer')

const connection_options = {
  db: {
    address: Constant.DB_URL,
    collection: 'closer',
  },
}

// 初始化agenda
let agenda = new Agenda(connection_options)
agenda
  .name('AGENDA - ' + process.pid)
  .defaultConcurrency(1)
  .defaultLockLifetime(10000)

const doCloser = async () => {
  try {
    let addrs = await AddressService.find({})
    let addresses = []
    addrs.map(o => addresses.push(o.address))
    if (addresses.length > 0) {
      logger.tag('to-be-refreshed-addresses').logObj(addresses)
      await refreshAddressBalance(addresses)
    }
  } catch (err) {
    logger.error(err)
  }
}

// 定义任务
agenda.define('doCloser', { priority: 'high', concurrency: 1 }, (job, done) => {
  (async () => {
    await doCloser();
  })().then(done, done);
})

// 配置任务(需要在ready事件中完成)
agenda.on('ready', () => {
  agenda.every(`${Constant.CLOSER_INTERVAL} seconds`, 'doCloser', {}, { timezone: 'Asia/Shanghai' })
  agenda.start()
})
// 设置监听
agenda.on('start', (job) => {

})

agenda.on('complete', (job) => {

})

agenda.on('success', (job) => {

})

agenda.on('fail', (job) => {
  console.log('检测到job失败: ', job.attrs.name)
  console.log('失败时间: ', job.attrs.failedAt)
  console.log('失败原因: ', job.attrs.failReason)
  agenda.stop()
})
// 最后，优雅的退出方案
function graceful() {
  agenda.stop(() => {
    console.log('检测到退出')
    process.exit(0);
  });
}

process.on('SIGTERM', graceful);
process.on('SIGINT', graceful);
