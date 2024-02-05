import "./App.css";
import React, { useEffect, useState } from "react";
import web3 from "./web3";
import lottery from './lottery';

function App() {
  const [manager, setManager] = useState('');
  const [players, setPlayers] = useState([]);
  const [balance, setBalance] = useState('');
  const [value, setValue] = useState(0);
  const [message, setMessage] = useState('');

  useEffect(() => {
    async function fetchManager() {
      const manager = await lottery.methods.manager().call();
      setManager(manager);
      return manager;
    }

    async function fetchPlayers(manager) {
      const players = await lottery.methods.getPlayers().call({
        from: manager
      });
      setPlayers(players);
    }

    async function getBalance() {
      const balance = await web3.eth.getBalance(lottery.options.address);
      setBalance(balance);
    }

    fetchManager()
      .then(manager => fetchPlayers(manager));
    getBalance();
  }, []);

  const onSubmit = async (event) => {
    event.preventDefault();
    
    const accounts = await web3.eth.getAccounts();

    setMessage('Wating on transaction success');

    await lottery.methods.enter().send({
      from: accounts[0],
      value: web3.utils.toWei(value, 'ether')
    })

    setMessage('You have been entered!');
  }

  const onClick = async (event) => {
    event.preventDefault();
    
    const accounts = await web3.eth.getAccounts();

    setMessage('Wating on transaction success');

    await lottery.methods.pickWinner().send({
      from: accounts[0],
      value: web3.utils.toWei(value, 'ether')
    })

    setMessage('A winner has been picked!');
  }

  return (
    <div>
      <h2>Lottery Contract</h2>
      <p>
        This contract is managed by {manager}.
        There is currently {players.length} people competing to win
        {web3.utils.fromWei(balance, 'ether')} ether.
      </p>

      <hr />

      <form onSubmit={onSubmit}>
      <h4>Want to try your luck?</h4>
      <div>
        <label>Amount of ether to enter:</label>
        <input
          value={value}
          onChange={e => setValue(e.target.value)}
        />
      </div>
      <button>Enter</button>
      </form>

      <hr />

      <h4>Ready to pick a winner?</h4>
      <button onClick={onClick}>Pick a winner</button>
      
      <hr />

      <h1>{message}</h1>
    </div>
  );
}
export default App;
