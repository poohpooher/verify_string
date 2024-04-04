use std::ops::RangeBounds;
use derive_builder::Builder;
use dyn_clone::DynClone;
use crate::verifier::Verifier;

/// Range-based string length verification
#[derive(Debug, Clone, Builder)]
pub struct LengthVerifier<T>
where T: Iterator<Item=usize> + RangeBounds<usize> + DynClone + Clone {
    range: T
}

impl<T> LengthVerifier<T>
where T: Iterator<Item=usize> + RangeBounds<usize> + DynClone + Clone {
    pub fn new(range: T) -> Self {
        Self {
            range
        }
    }
}

impl<T> Verifier for LengthVerifier<T>
where T: Iterator<Item=usize> + RangeBounds<usize> + DynClone + Clone {

    fn verify(&self, name: &str) -> Result<(), anyhow::Error> {
        let len = name.chars().count();

        if self.range.contains(&len) {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Length is out of range: {}", len))
        }
    }
}