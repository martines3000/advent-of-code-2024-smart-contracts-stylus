use std::fs;

// e2e module
use e2e::{eyre::Result, tokio, Account, ReceiptExt};

// AOC2024 ABI
use abi::Solution;
mod abi;

// Test Day 25 Part 1 (2024)
#[e2e::test]
async fn day_25_1(account: Account) -> Result<()> {
    let contract_addr = account.as_deployer().deploy().await?.address()?;
    let instance = Solution::new(contract_addr, &account.wallet);

    // Read input
    let input = fs::read_to_string("tests/input.txt").unwrap();

    let Solution::solvepart1Return { result } = instance.solvepart1(input).call().await?;

    println!("Result Day 25 part 1: {}", result);

    assert!(result == 1406);
    Ok(())
}

// Test Day 25 Part 2 (2024)
#[e2e::test]
async fn day_25_2(account: Account) -> Result<()> {
    let contract_addr = account.as_deployer().deploy().await?.address()?;
    let instance = Solution::new(contract_addr, &account.wallet);

    // Read input
    let input = fs::read_to_string("tests/input.txt").unwrap();

    let Solution::solvepart2Return { result } = instance.solvepart2(input).call().await?;

    println!("Result Day 25 part 2: {}", result);

    assert!(result == 1006101);
    Ok(())
}
