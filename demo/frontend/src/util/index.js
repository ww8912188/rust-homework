const shortString = str => {
  let l = str.length
  return str.substr(0, 4) + '....' + str.substr(l - 4, l)
}

module.exports = {
  shortString
}
