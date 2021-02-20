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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Strong;

    #[test]
    fn name() {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct UserId(i32);
        struct User<'a> {
            id: UserId,
            name: &'a Strong<Name<UserId>>
        }
        let name: &Strong<Name<UserId>> = Strong::validate("Alice").unwrap();
        let u = User {
            id: UserId(3),
            name
        };
        assert_eq!(u.id, UserId(3));
        assert_eq!(u.name.as_str(), "Alice");
    }

    #[test]
    fn email() {
        assert!(Strong::<Email>::validate("a").is_err());
        Strong::<Email>::validate("a@example.com").unwrap();
    }
}
