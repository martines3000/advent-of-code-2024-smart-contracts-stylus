// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
// #![no_std]
extern crate alloc;
use alloc::string::String;
use stylus_sdk::{
    alloy_primitives::Address,
    call::Call,
    console, evm, msg,
    prelude::*,
    storage::{StorageAddress, StorageBool, StorageMap},
};

#[storage]
#[entrypoint]
pub struct Orchestrator {
    initialized: StorageBool,
    owner: StorageAddress,
    day_to_solution: StorageMap<u32, StorageAddress>,
}

sol_interface! {
  interface Solution {
    function solvepart1(string calldata input) external view returns (uint32 result);
    function solvepart2(string calldata input) external view returns (uint32 result);
  }
}

#[public]
impl Orchestrator {
    // State variables are initialized in an `init` function.
    pub fn init(&mut self) {
        // We check if contract has been initialized before.
        // We return if so, we initialize if not.
        let initialized = self.initialized.get();
        if initialized {
            panic!("Contract already initialized");
        }
        self.initialized.set(true);

        // We set the contract owner to the caller,
        // which we get from the global msg module
        self.owner.set(msg::sender());
    }

    pub fn set_solution(&mut self, day: u32, address: String) {
        if msg::sender() != self.owner.get() {
            panic!("Only owner can set solution");
        }

        self.day_to_solution
            .insert(day, Address::parse_checksummed(address, None).unwrap());
    }

    pub fn solve(&mut self, day: u32, part: u32, input: String) -> u32 {
        let solution = self.day_to_solution.get(day);

        if solution.is_zero() {
            panic!("No solution for this day");
        }

        let solution = Solution::new(solution);

        let config = Call::new().gas(evm::gas_left());

        let result = match part {
            1 => solution.solvepart_1(config, input),
            2 => solution.solvepart_2(config, input),
            _ => panic!("Invalid part"),
        };

        result.unwrap()
    }
}
