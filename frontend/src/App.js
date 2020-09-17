import React, { Component } from 'react';
import Label from './Label'
import './App.css';
import { getAccounts, doTransfer } from './service'
import TextareaAutosize from '@material-ui/core/TextareaAutosize';
import Button from '@material-ui/core/Button';

class App extends Component {
  state = {
    accounts: [],
    obj: ''
  }


  async componentWillMount() {
    let { data: accounts } = await getAccounts()
    this.setState({ accounts })
  }

  handleObjChange = e => {
    this.setState({
      obj: e.target.value
    })
  }

  handleClick = async () => {
    let { obj } = this.state
    try {
      let up = JSON.parse(obj)
      let ret = await doTransfer(up)
      console.log(ret)
    } catch (err) {
      alert(err)
    }
  }

  render() {
    let { accounts } = this.state
    return (
      <div className="App">
        <div className="App-header" style={{ display: 'flex', flexDirection: 'row' }}>
          <div>
            {
              accounts.map((o, index) => <Label key={index} account={o} />)
            }
          </div>
          <div style={{ marginLeft: '10px', display: 'flex', flexDirection: 'column' }}>
            <div style={{ display: 'flex' }}>Transfer:</div>
            <TextareaAutosize rows={8} onChange={e => this.handleObjChange(e)} />
            <Button style={{ marginTop: '10px' }} variant="contained" onClick={this.handleClick}>GO</Button>
          </div>
        </div>
      </div>
    )
  }
}

export default App;
