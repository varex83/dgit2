use crate::utils::get_object_hashes;
use anyhow::Result;
use colored::Colorize;
use ethcontract::U256;

pub async fn status() -> Result<()> {
    println!("Checking repository status...");

    let contract_address = crate::config::Config::get_contract_address();

    let contract =
        crate::contract_interaction::ContractInteraction::new_with_address(&contract_address);

    let mut count_down = 0usize;

    let len_of_objects = contract.get_objects_length().await?.as_u64();

    for index in 0..len_of_objects {
        let object = contract.get_object_by_id(U256::from(index)).await?;
        let (prefix, hash) = object.hash.split_at(2);
        let file_path = format!("./.git/objects/{}/{}", prefix, hash);

        if std::path::Path::new(&file_path).exists() {
            continue;
        }

        count_down += 1;
    }

    let (objects, _) = get_object_hashes("./.git/objects").await?;

    let flag_vec = contract.check_objects(objects.clone()).await?;

    let count_up = flag_vec.iter().filter(|&flag| !flag).count();

    println!(
        "{}",
        format!("Total objects to upload: {}", count_up).green()
    );

    println!(
        "{}",
        format!("Total objects to download: {}", count_down).yellow()
    );

    Ok(())
}
