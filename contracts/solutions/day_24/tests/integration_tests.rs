use std::fs;

// e2e module
use e2e::{eyre::Result, tokio, Account, ReceiptExt};

// AOC2024 ABI
use abi::Solution;
mod abi;

// Test Day 24 Part 1 (2024)
#[e2e::test]
async fn day_24_1(account: Account) -> Result<()> {
    let contract_addr = account.as_deployer().deploy().await?.address()?;
    let instance = Solution::new(contract_addr, &account.wallet);

    // Read input
    let input = fs::read_to_string("tests/input.txt").unwrap();

    let Solution::solvepart1Return { result } = instance.solvepart1(input).call().await?;

    println!("Result Day 24 part 1: {}", result);

    assert!(result == 59336987801432);
    Ok(())
}

// Test Day 24 Part 2 (2024)
#[e2e::test]
async fn day_24_2(account: Account) -> Result<()> {
    let contract_addr = account.as_deployer().deploy().await?.address()?;
    let instance = Solution::new(contract_addr, &account.wallet);

    // Read input
    let input = fs::read_to_string("tests/input.txt").unwrap();

    let Solution::solvepart2Return { result } = instance.solvepart2(input).call().await?;

    println!("Result Day 24 part 2: {}", result);

    assert!(result == "ctg,dmh,dvq,rpb,rpv,z11,z31,z38");
    Ok(())
}
