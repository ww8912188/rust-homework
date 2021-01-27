import React from 'react';
import { shortString } from './util'

function Label(props) {
  let { account, address, balance } = props.account
  return (
    <div style={{ width: "250px", height: "150px", display: 'flex', flexDirection: 'column', border: '1px solid white', padding: '10px' }}>
      <div style={{ display: 'flex', flexDirection: 'row', flexGrow: 1 }}>
        <div>Account:&nbsp;</div>
        <div>{account}</div>
      </div >
      <div style={{ display: 'flex', flexDirection: 'row', flexGrow: 1 }}>
        <div>Address:&nbsp;</div>
        <div title={address}>{shortString(address)}</div>
      </div >
      <div style={{ display: 'flex', flexDirection: 'row', flexGrow: 1 }}>
        <div>Balance:&nbsp;</div>
        <div>{balance}</div>
      </div >
    </div >
  )
}

export default Label;
