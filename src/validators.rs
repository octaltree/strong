use crate::{StrongBuf, Validator};
use std::marker::PhantomData;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{raw} isn't valid")]
pub struct InvalidEmail {
    raw: String
}

#[cfg_attr(feature = "diesel", derive(SqlType, QueryId))]
pub enum Email {}

#[cfg(not(feature = "impl_partial_eq_transparent"))]
impl crate::marker::PartialEqTransparent for Email {}
#[cfg(not(feature = "impl_eq_transparent"))]
impl crate::marker::EqTransparent for Email {}
#[cfg(not(feature = "impl_partial_ord_transparent"))]
impl crate::marker::PartialOrdTransparent for Email {}
#[cfg(not(feature = "impl_ord_transparent"))]
impl crate::marker::OrdTransparent for Email {}
#[cfg(not(feature = "impl_hash_transparent"))]
impl crate::marker::HashTransparent for Email {}
#[cfg(not(feature = "impl_debug_transparent"))]
impl crate::marker::DebugTransparent for Email {}
#[cfg(not(feature = "impl_display_transparent"))]
impl crate::marker::DisplayTransparent for Email {}

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

#[cfg_attr(feature = "diesel", derive(SqlType, QueryId))]
pub struct Name<T> {
    phantom: PhantomData<T>
}

#[cfg(not(feature = "impl_partial_eq_transparent"))]
impl<T> crate::marker::PartialEqTransparent for Name<T> {}
#[cfg(not(feature = "impl_eq_transparent"))]
impl<T> crate::marker::EqTransparent for Name<T> {}
#[cfg(not(feature = "impl_partial_ord_transparent"))]
impl<T> crate::marker::PartialOrdTransparent for Name<T> {}
#[cfg(not(feature = "impl_ord_transparent"))]
impl<T> crate::marker::OrdTransparent for Name<T> {}
#[cfg(not(feature = "impl_hash_transparent"))]
impl<T> crate::marker::HashTransparent for Name<T> {}
#[cfg(not(feature = "impl_debug_transparent"))]
impl<T> crate::marker::DebugTransparent for Name<T> {}
#[cfg(not(feature = "impl_display_transparent"))]
impl<T> crate::marker::DisplayTransparent for Name<T> {}

impl<T> Validator for Name<T> {
    type Err = std::convert::Infallible;
}

impl<T> Default for StrongBuf<Name<T>> {
    fn default() -> Self { unsafe { Self::no_validate(String::new()) } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Strong as S, *};

    #[test]
    fn name() {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct UserId(i32);
        struct User<'a> {
            id: UserId,
            name: &'a S<Name<UserId>>
        }
        let name: &S<Name<UserId>> = TryFrom::try_from("Alice").unwrap();
        let u = User {
            id: UserId(3),
            name
        };
        assert_eq!(u.id, UserId(3));
        assert_eq!(u.name.as_str(), "Alice");
    }

    #[test]
    fn email() {
        assert!(StrongBuf::<Email>::validate("a".to_string()).is_err());
        let _: StrongBuf<Email> = TryFrom::try_from("a@example.com".to_string()).unwrap();
    }
}
