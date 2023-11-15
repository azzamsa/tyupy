#![deny(unsafe_code)]

pub mod out;

pub mod cli;
pub mod config;
pub mod error;
pub mod exit_codes;
pub mod fmt;
pub mod web;

// Aliases
pub use error::Error;
