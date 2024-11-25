use std::fs;

// e2e module
use e2e::{eyre::Result, tokio, Account, ReceiptExt};

// AOC2024 ABI
use abi::AOC2024;
mod abi;

// Test Day 1 Part 1 (2023)
#[e2e::test]
async fn day_0_1(account: Account) -> Result<()> {
    let contract_addr = account.as_deployer().deploy().await?.address()?;
    let instance = AOC2024::new(contract_addr, &account.wallet);

    // Read input
    let input = fs::read_to_string("tests/inputs/0_1.txt").unwrap();

    let AOC2024::solve01Return { result } = instance.solve01(input).call().await?;

    assert!(result == 54561);

    Ok(())
}

// Test Day 1 Part 2 (2023)
#[e2e::test]
async fn day_0_2(account: Account) -> Result<()> {
    let contract_addr = account.as_deployer().deploy().await?.address()?;
    let instance = AOC2024::new(contract_addr, &account.wallet);

    // Read input
    let input = fs::read_to_string("tests/inputs/0_1.txt").unwrap();

    let AOC2024::solve02Return { result } = instance.solve02(input).call().await?;

    assert!(result == 54076);

    Ok(())
}
