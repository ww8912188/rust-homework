const Agenda = require('agenda');
const Constant = require('../common/Constant')
const TxService = require('../service/tx')
const { doTransfer } = require('../controller/dot')
const { sleepSeconds } = require('../util/time')
const logger = require('../util/logger').of('Outer')

const connection_options = {
  db: {
    address: Constant.DB_URL,
    collection: 'outer',
  },
}

// 初始化agenda
let agenda = new Agenda(connection_options)
agenda
  .name('AGENDA - ' + process.pid)
  .defaultConcurrency(1)
  .defaultLockLifetime(10000)

const doPayout = async () => {
  try {
    let txs = await TxService.find({ status: 'init' })
    for (let i = 0; i < txs.length; i++) {
      let { from, to, value, uid } = txs[i]
      logger.info(`found payout tx ${uid}, from ${from}, to ${to}, value ${value}`)
      await doTransfer(from, to, value, uid)
      await sleepSeconds(10)
    }
  } catch (err) {
    logger.error(err)
  }
}

// 定义任务
agenda.define('doPayout', { priority: 'high', concurrency: 1 }, (job, done) => {
  (async () => {
    await doPayout();
  })().then(done, done);
})

// 配置任务(需要在ready事件中完成)
agenda.on('ready', () => {
  agenda.every(`${Constant.OUTER_INTERVAL} seconds`, 'doPayout', {}, { timezone: 'Asia/Shanghai' })
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
