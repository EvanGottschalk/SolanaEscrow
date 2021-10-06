pub mod entrypoint;
pub mod instructions;
pub mod lib;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
