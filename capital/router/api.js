const Router = require('koa-router');
const Constant = require('../common/Constant');
const Dot = require('../controller/dot.js')
const router = new Router();

router.prefix('/api/v1');
router.get('/ping', ctx => {
  ctx.body = {
    success: true
  }
})
router.get('/address', Dot.getAddress);
router.post('/transfer', Dot.transfer);

module.exports = router.routes();
