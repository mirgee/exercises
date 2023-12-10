pub mod contracts;

use ethers::{
    providers::{Http, Provider},
    types::Address,
};

use self::contracts::{campaign::Campaign, campaign_factory::CampaignFactory};

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
