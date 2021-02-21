use crate::{marker::*, Validator};
use std::marker::PhantomData;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{raw} isn't valid")]
pub struct InvalidEmail {
    raw: String
}

pub enum Email {}

impl PartialEqTransparent for Email {}
impl EqTransparent for Email {}
impl PartialOrdTransparent for Email {}
impl OrdTransparent for Email {}
impl HashTransparent for Email {}

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

impl<T> DefaultTransparent for Name<T> {}
impl<T> PartialEqTransparent for Name<T> {}
impl<T> EqTransparent for Name<T> {}
impl<T> PartialOrdTransparent for Name<T> {}
impl<T> OrdTransparent for Name<T> {}
impl<T> HashTransparent for Name<T> {}

impl<T> Validator for Name<T> {
    type Err = std::convert::Infallible;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Strong as S;

    #[test]
    fn name() {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct UserId(i32);
        struct User<'a> {
            id: UserId,
            name: &'a S<Name<UserId>>
        }
        let name: &S<Name<UserId>> = S::validate("Alice").unwrap();
        let u = User {
            id: UserId(3),
            name
        };
        assert_eq!(u.id, UserId(3));
        assert_eq!(u.name.as_str(), "Alice");
    }

    #[test]
    fn email() {
        assert!(S::<Email>::validate("a").is_err());
        S::<Email>::validate("a@example.com").unwrap();
    }
}
