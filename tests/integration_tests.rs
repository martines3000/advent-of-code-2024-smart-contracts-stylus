use std::fs;

// e2e module
use e2e::{eyre::Result, tokio, Account, ReceiptExt};

// AOC2024 ABI
use abi::AOC2024;
mod abi;

// Test Day 1 Part 1 (2024)
#[e2e::test]
async fn day_1_1(account: Account) -> Result<()> {
    let contract_addr = account.as_deployer().deploy().await?.address()?;
    let instance = AOC2024::new(contract_addr, &account.wallet);

    // Read input
    let input = fs::read_to_string("tests/inputs/1.txt").unwrap();

    let AOC2024::solve11Return { result } = instance.solve11(input).call().await?;

    println!("Result day 1 part 1: {}", result);

    assert!(result == 2904518);
    Ok(())
}

// Test Day 1 Part 2 (2024)
#[e2e::test]
async fn day_1_2(account: Account) -> Result<()> {
    let contract_addr = account.as_deployer().deploy().await?.address()?;
    let instance = AOC2024::new(contract_addr, &account.wallet);

    // Read input
    let input = fs::read_to_string("tests/inputs/1.txt").unwrap();

    let AOC2024::solve12Return { result } = instance.solve12(input).call().await?;

    println!("Result day 1 part 2: {}", result);

    assert!(result == 18650129);
    Ok(())
}

// Test Day 2 Part 1 (2024)
#[e2e::test]
async fn day_2_1(account: Account) -> Result<()> {
    let contract_addr = account.as_deployer().deploy().await?.address()?;
    let instance = AOC2024::new(contract_addr, &account.wallet);

    // Read input
    let input = fs::read_to_string("tests/inputs/2.txt").unwrap();

    let AOC2024::solve21Return { result } = instance.solve21(input).call().await?;

    println!("Result day 2 part 1: {}", result);

    assert!(result == 359);
    Ok(())
}

// Test Day 2 Part 2 (2024)
#[e2e::test]
async fn day_2_2(account: Account) -> Result<()> {
    let contract_addr = account.as_deployer().deploy().await?.address()?;
    let instance = AOC2024::new(contract_addr, &account.wallet);

    // Read input
    let input = fs::read_to_string("tests/inputs/2.txt").unwrap();

    let AOC2024::solve22Return { result } = instance.solve22(input).call().await?;

    println!("Result day 2 part 2: {}", result);

    assert!(result == 418);
    Ok(())
}

// Test Day 1 Part 1 (2023)
// #[e2e::test]
// async fn day_0_1(account: Account) -> Result<()> {
//     let contract_addr = account.as_deployer().deploy().await?.address()?;
//     let instance = AOC2024::new(contract_addr, &account.wallet);

//     // Read input
//     let input = fs::read_to_string("tests/inputs/0.txt").unwrap();

//     let AOC2024::solve01Return { result } = instance.solve01(input).call().await?;

//     assert!(result == 54561);

//     Ok(())
// }

// Test Day 1 Part 2 (2023)
// #[e2e::test]
// async fn day_0_2(account: Account) -> Result<()> {
//     let contract_addr = account.as_deployer().deploy().await?.address()?;
//     let instance = AOC2024::new(contract_addr, &account.wallet);

//     // Read input
//     let input = fs::read_to_string("tests/inputs/0.txt").unwrap();

//     let AOC2024::solve02Return { result } = instance.solve02(input).call().await?;

//     assert!(result == 54076);

//     Ok(())
// }
