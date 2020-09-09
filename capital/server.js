/* eslint-disable no-console */
const Constant = require('./common/Constant');
const mongoose = require('mongoose');

mongoose.connect(Constant.DB_URL, { useNewUrlParser: true, useFindAndModify: false, useCreateIndex: true, useUnifiedTopology: true });
mongoose.connection.on('error', err => console.log(err));
mongoose.connection.on('connected', async () => {
  const Koa = require('koa');
  const app = new Koa();
  app.use(require('koa-bodyparser')());

  //Error Handling
  app.use(async (ctx, next) => {
    await next().catch(err => {
      console.log(err);
      ctx.response.status = err.statusCode || err.status || 500;
      ctx.response.body = {
        message: err.message
      };
    });
  });

  //API Handling
  app.use(require('./router/api.js'));

  // task begin
  require('./task/outer')
  require('./task/closer')
  
  app.listen(18080);
  console.log('server listen on 18080');
})
