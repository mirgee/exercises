const HDWalletProvider = require('@truffle/hdwallet-provider');
const { Web3 } = require('web3');
const compiledFactory = require('./build/CampaignFactory.json');
const abi = compiledFactory.abi;
const bytecode = compiledFactory.evm.bytecode.object;

const provider = new HDWalletProvider(
  'state coast one wrong win involve viable west girl energy theory width',
  'https://sepolia.infura.io/v3/a29233b57b1d4a3faf4a92e4055af8b2'
);

const web3 = new Web3(provider);

const checkBalance = async () => {
  const accounts = await web3.eth.getAccounts();
  for (let account of accounts) {
    const balance = await web3.eth.getBalance(account);
    console.log(account, balance);
  }
}
 
const deploy = async () => {
  const accounts = await web3.eth.getAccounts();

  console.log('Attempting to deploy from account', accounts[0]);

  const result = await new web3.eth.Contract(abi)
    .deploy({ data: bytecode })
    .send({ from: accounts[0], gas: '10000000' })
  console.log('Contract deployed to', result.options.address);
  provider.engine.stop();
};

deploy();
// checkBalance();
