use derive_builder::Builder;
use crate::Verifier;

/// ProfanityVerifier is a struct that contains a list of profanities that are used to check if a string contains any of the profanities.
#[derive(Debug, Clone, Builder)]
pub struct WordVerifier {
    profanities: Vec<String>,
}

impl Verifier for WordVerifier {
    fn verify(&self, name: &str) -> Result<(), anyhow::Error> {
        for profanity in &self.profanities {
            if name.contains(profanity) {
                return Err(anyhow::anyhow!("Contains profanity: {}", profanity));
            }
        }

        Ok(())
    }


}