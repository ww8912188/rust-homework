const sleepSeconds = s => {
  // console.log(`sleep for ${s} seconds`)
  return new Promise((resolve) => setTimeout(resolve, s * 1000))
}

module.exports = {
  sleepSeconds,
}
