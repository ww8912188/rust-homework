const { ObjectId } = require('mongoose').Types;
const Address = require('../schema/Address');
const Constant = require('../common/Constant');
const Pager = require('../common/Pager');
const BasicService = require('./base')

class AddressService extends BasicService {
  
}

module.exports = new AddressService(Address);
