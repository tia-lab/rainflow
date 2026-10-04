mod config;
mod core;
mod errors;
mod types;

pub use config::RAINFLOW_MAX_INPUT_LEN;
pub use core::{RainflowCycle, rainflow_cycles};
pub use errors::MathError;
pub use types::MathResult;
#[cfg(test)]
mod tests;
