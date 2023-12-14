pub mod contracts;

use ethers::{
    providers::{Http, Provider},
    types::Address,
};
use web3::{transports::eip_1193, Web3};
use web3::{transports::eip_1193::Eip1193, Transport};

use self::contracts::{campaign::Campaign, campaign_factory::CampaignFactory};

pub async fn create_web3_instance_1() -> Web3<Eip1193> {
    let provider = eip_1193::Provider::default().unwrap().unwrap();
    let transport = eip_1193::Eip1193::new(provider);

    let web3 = web3::Web3::new(transport.clone());

    web3
}

pub fn create_web3_instance() -> Provider<Http> {
    let provider =
        Provider::<Http>::try_from("https://sepolia.infura.io/v3/a29233b57b1d4a3faf4a92e4055af8b2")
            .unwrap();
    provider
}

pub fn get_factory_contract() -> CampaignFactory<Provider<Http>> {
    let address: Address = "0x68Adfc88aE5C3ec71497770E1c7F4f25A7319A83"
        .parse()
        .unwrap();
    CampaignFactory::new(address, create_web3_instance().into())
}

pub fn get_campaign_contract(address: Address) -> Campaign<Provider<Http>> {
    Campaign::new(address, create_web3_instance().into())
}
