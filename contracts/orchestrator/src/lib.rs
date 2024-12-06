#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
// #![no_std]
extern crate alloc;
use stylus_sdk::prelude::*;

/// The storage macro allows this struct to be used in persistent
/// storage. It accepts fields that implement the StorageType trait. Built-in
/// storage types for Solidity ABI primitives are found under
/// stylus_sdk::storage.
#[storage]
/// The entrypoint macro defines where Stylus execution begins. External methodsf
/// are exposed by annotating an impl for this struct with #[external] as seen
/// below.
#[entrypoint]
pub struct Orchestrator {}

#[public]
impl Orchestrator {}
