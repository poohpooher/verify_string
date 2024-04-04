mod length;
mod regex;
mod word;

pub use length::*;
pub use regex::*;
pub use word::*;

use dyn_clone::DynClone;

pub trait Verifier: DynClone + Send + Sync {
    fn verify(&self, name: &str) -> Result<(), anyhow::Error>;
}