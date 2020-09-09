const mongoose = require('mongoose');
module.exports = mongoose.model('Block', {
  name: String,
  scaned: { type: Boolean, default: false }
}); 