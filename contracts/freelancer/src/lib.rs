#![no_std]

mod storage;
mod storage_types;
mod events;
mod contract;
mod test;

pub use crate::contract::FreelanceContractClient;