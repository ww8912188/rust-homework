const axios = require('axios')
const ENV = 'http://127.0.0.1:18080'

export const get = async url => {
  return (await axios.get(ENV + url)).data
}

export const post = async (url, data) => {
  return (await axios.post(ENV + url, data)).data
}
