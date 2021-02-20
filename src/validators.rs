use crate::Validator;
use std::marker::PhantomData;
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

pub struct Name<T> {
    phantom: PhantomData<T>
}

impl<T> Validator for Name<T> {
    type Err = std::convert::Infallible;
    fn validate(s: &str) -> Result<(), Self::Err> { Ok(()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Strong, StrongBuf};

    #[test]
    fn name() {
        #[derive(Clone, Copy)]
        struct UserId(i32);
        struct User<'a> {
            id: UserId,
            name: &'a strong<Name<UserId>>
        }
        let name = Strong::<Name<UserId>>::validate("Alice").unwrap();
        let _ = User {
            id: UserId(3),
            name
        };
    }

    #[test]
    fn email() {
        assert!(Strong::<Email>::validate("a").is_err());
        Strong::<Email>::validate("a@example.com").unwrap();
    }
}
