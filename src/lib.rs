// Allow `cargo stylus export-abi` to generate a main function.
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::StorageU256};

/// The storage macro allows this struct to be used in persistent
/// storage. It accepts fields that implement the StorageType trait. Built-in
/// storage types for Solidity ABI primitives are found under
/// stylus_sdk::storage.
#[storage]
/// The entrypoint macro defines where Stylus execution begins. External methods
/// are exposed by annotating an impl for this struct with #[external] as seen
/// below.
#[entrypoint]
pub struct Counter {
    number: StorageU256,
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Counter {
    /// Gets the number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Sets a number in storage to a user-specified value.
    pub fn mul_number(&mut self, new_number: U256) {
        self.number.set(new_number * self.number.get());
    }

    /// Sets a number in storage to a user-specified value.
    pub fn add_number(&mut self, new_number: U256) {
        self.number.set(new_number + self.number.get());
    }

    /// Increments `number` and updates its value in storage.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }
}
