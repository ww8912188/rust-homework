const mongoose = require('mongoose');
module.exports = mongoose.model('Address', {
  address: { type: String },
  account: { type: String },
  mnemonic: { type: String },
  balance: { type: String, default: '0' },
}); 