use crate::config::Config;
use crate::utils::get_object_hashes;
use colored::*;
use ethcontract::jsonrpc::serde::__private::from_utf8_lossy;
use ethcontract::U256;

pub async fn push() -> anyhow::Result<()> {
    let contract_address = Config::get_contract_address();

    let count_up = sync_up(contract_address.clone()).await?;

    println!("{}", format!("Synced up: {}", count_up).green());

    Ok(())
}

pub async fn pull() -> anyhow::Result<()> {
    let contract_address = Config::get_contract_address();

    let count_down = sync_down(contract_address.clone()).await?;

    println!("{}", format!("Synced down: {}", count_down).blue());

    Ok(())
}

pub async fn sync_up(contract_address: String) -> anyhow::Result<usize> {
    let (objects, paths) = get_object_hashes("./.git/objects").await?;

    let contract =
        crate::contract_interaction::ContractInteraction::new_with_address(&contract_address);

    println!(
        "{}",
        format!("Using contract address: {}", contract_address).yellow()
    );

    let flag_vec = contract.check_objects(objects.clone()).await?;

    let mut ipfs_hashes = Vec::new();

    for (i, flag) in flag_vec.iter().enumerate() {
        if !flag {
            let ipfs_hash = crate::ipfs::load_to_ipfs(paths[i].as_str()).await?;
            ipfs_hashes.push(ipfs_hash.clone().into_bytes());

            contract
                .save_object(objects[i].clone(), ipfs_hash.into_bytes())
                .await?;

            println!(
                "{}",
                format!("Uploaded and saved object: {}", objects[i]).cyan()
            );
        }
    }

    println!(
        "{}",
        format!("Total objects synced up: {}", ipfs_hashes.len()).green()
    );
    Ok(ipfs_hashes.len())
}

pub async fn sync_down(contract_address: String) -> anyhow::Result<usize> {
    let contract =
        crate::contract_interaction::ContractInteraction::new_with_address(&contract_address);

    let mut count = 0usize;

    let len_of_objects = contract.get_objects_length().await?.as_u64();

    println!(
        "{}",
        format!("Length of objects in contract: {}", len_of_objects).yellow()
    );

    for index in 0..len_of_objects {
        let object = contract.get_object_by_id(U256::from(index)).await?;
        let ipfs_hash = object.ipfs_url;
        let (prefix, hash) = object.hash.split_at(2);
        let file_path = format!("./.git/objects/{}/{}", prefix, hash);

        if std::path::Path::new(&file_path).exists() {
            println!("{}", format!("File already exists: {}", file_path).blue());
            continue;
        }

        crate::ipfs::download_from_ipfs(from_utf8_lossy(ipfs_hash.as_slice()).as_ref(), &file_path)
            .await?;

        println!(
            "{}",
            format!("Downloaded and saved file: {}", file_path).cyan()
        );

        count += 1;
    }

    println!("{}", format!("Total objects synced down: {}", count).blue());
    Ok(count)
}
