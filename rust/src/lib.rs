#![cfg_attr(not(feature = "std"), no_std)]

pub mod convert;
pub mod schemas;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;
