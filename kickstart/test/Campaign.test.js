const assert = require('assert');
const ganache = require('ganache');
const { Web3 } = require('web3');
const web3 = new Web3(ganache.provider());

const compiledFactory = require('../ethereum/build/CampaignFactory.json');
const compiledCampaign = require('../ethereum/build/Campaign.json');

let accounts;
let factory;
let campaignAddress;
let campaign;

beforeEach(async () => {
  accounts = await web3.eth.getAccounts();
  factory = await new web3.eth.Contract(compiledFactory.abi)
    .deploy({ data: compiledFactory.evm.bytecode.object })
    .send({ from: accounts[0], gas: '10000000' });
  await factory.methods.createCampaign('100').send({ from: accounts[0], gas: '1000000' });
  [campaignAddress] = await factory.methods.getDeployedCampaigns().call();
  campaign = new web3.eth.Contract(
    compiledCampaign.abi,
    campaignAddress
  );
})

describe('Campaigns', () => {
  it('deploys a contract', () => {
    assert.ok(factory.options.address);
    assert.ok(campaign.options.address);
  });
});

