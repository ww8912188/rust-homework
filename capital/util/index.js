const genUniqueName = () => {
  var resource = "abcdefghzklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
  var s = "";
  var length = 10;
  for (var i = 0; i < length; i++) {
    s += resource.charAt(
      Math.ceil(Math.random() * 1000) % resource.length
    );
  }
  return s;
}

module.exports = {
  genUniqueName
}
