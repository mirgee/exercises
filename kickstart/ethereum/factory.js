import CampaignFactory from './build/CampaignFactory.json';
import web3 from './web3';

const instance = new web3.eth.Contract(
  CampaignFactory.abi,
  '0xEF57540C95D13f8d6105C105187ABFA2d2CB2a26'
);

export default instance;
