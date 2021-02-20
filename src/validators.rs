use crate::Validator;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{raw} isn't valid")]
pub struct InvalidEmail {
    raw: String
}

pub enum Email {}

impl Validator for Email {
    type Err = InvalidEmail;

    fn validate(raw: &str) -> Result<(), Self::Err> {
        if validator::validate_email(raw) {
            Ok(())
        } else {
            Err(InvalidEmail { raw: raw.into() })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Strong;

    #[test]
    fn email() {
        assert!(Strong::<Email>::validate("a".to_string()).is_err());
        Strong::<Email>::validate("a@example.com".to_string()).unwrap();
    }
}
