use crate::config::Config;
use crate::utils::get_object_hashes;

pub async fn sync(contract_address: Option<String>) -> anyhow::Result<()> {
    let contract_address = contract_address.unwrap_or(Config::get_contract_address());

    println!("Syncing repository...");

    // get objects
    let (objects, paths) = get_object_hashes("./.git/objects").await?;

    // ask contract for objects
    let contract =
        crate::contract_interaction::ContractInteraction::new_with_address(&contract_address);

    println!("{}", contract_address);

    let flag_vec = contract.check_objects(objects.clone()).await?;

    let mut ipfs_hashes = Vec::new();

    for (i, flag) in flag_vec.iter().enumerate() {
        if !flag {
            let ipfs_hash = crate::ipfs::load_to_ipfs(paths[i].as_str()).await?;

            ipfs_hashes.push(ipfs_hash.clone().into_bytes());
        }
    }

    contract.add_objects(objects, ipfs_hashes.clone()).await?;

    println!(
        "Repository synced successfully with {} objects added",
        ipfs_hashes.len()
    );

    Ok(())
}
