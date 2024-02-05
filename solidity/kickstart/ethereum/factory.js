import CampaignFactory from './build/CampaignFactory.json';
import web3 from './web3';

const instance = new web3.eth.Contract(
  CampaignFactory.abi,
  '0x68Adfc88aE5C3ec71497770E1c7F4f25A7319A83'
);

export default instance;
