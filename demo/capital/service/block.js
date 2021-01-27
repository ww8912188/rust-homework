const { ObjectId } = require('mongoose').Types;
const Block = require('../schema/Block');
const Constant = require('../common/Constant');
const Pager = require('../common/Pager');
const BasicService = require('./base')

class BlockService extends BasicService {
  constructor() { }
}

module.exports = new BlockService(Block);
