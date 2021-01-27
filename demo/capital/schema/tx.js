const mongoose = require('mongoose');
module.exports = mongoose.model('Tx', {
  uid: { type: String, unique: true },
  from: String,
  to: String,
  value: Number,
  status: String,
  hash: String,
})