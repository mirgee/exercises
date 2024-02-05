use ethers::{prelude::Abigen, solc::Solc};

fn generate_binding(
    contract_name: &str,
    file_name: &str,
    contract_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating bindings for {contract_path}\n");

    let contracts = Solc::default().compile_source(&contract_path)?;
    let abi = contracts
        .get(&contract_path, &contract_name)
        .unwrap()
        .abi
        .unwrap();
    let abi = serde_json::to_string(abi).unwrap();

    let bindings = Abigen::new(&contract_name, abi)?.generate()?;

    let output_path = format!("./src/ethereum/contracts/{}.rs", file_name);
    bindings.write_to_file(output_path)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_binding("Campaign", "campaign", "./ethereum/contracts/Campaign.sol")?;
    generate_binding(
        "CampaignFactory",
        "campaign_factory",
        "./ethereum/contracts/Campaign.sol",
    )?;

    Ok(())
}
