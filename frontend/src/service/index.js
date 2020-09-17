const { get, post } = require('./wrap')

export const getAccounts = async () => {
  let out = await get('/api/v1/accounts');
  return out
}

export const doTransfer = async payload => {
  let out = await post('/api/v1/transfer', payload)
  return out
}
