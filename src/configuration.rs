//! in src/configuration.rs

// manging configuration with 'config' - application settings
// as a Rust type it must implement Serde's Deserialize trait
#[derive(serde::Deserialize)]
pub struct Settings {}
