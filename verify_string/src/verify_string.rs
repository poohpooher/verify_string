use log::error;
use crate::Verifier;

#[derive(Default)]
pub struct VerifyString{
    pub verifies : Vec<Box<dyn Verifier>>
}


impl VerifyString {
    pub fn with_verifier(&mut self, verifier: impl Verifier + 'static) -> &mut Self {
        self.verifies.push(Box::new(verifier));
        self
    }

    pub fn verify(&self, name: &str) -> bool {
        for verifier in &self.verifies {
            if let Err(e) = verifier.verify(name) {
                error!("{}", e);
                return false;
            }
        }

        true
    }
}
