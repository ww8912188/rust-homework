const { ObjectId } = require('mongoose').Types;
const Tx = require('../schema/tx');
const Constant = require('../common/Constant');
const Pager = require('../common/Pager');
const BasicService = require('./base')

class TxService extends BasicService {
  
}

module.exports = new TxService(Tx);
