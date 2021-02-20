use crate::Validator;
use std::marker::PhantomData;

#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct strong<Ctx: Validator> {
    phantom: PhantomData<Ctx>,
    inner: str
}

impl<Ctx: Validator> strong<Ctx> {
    pub fn validate<S: AsRef<str> + ?Sized>(s: &S) -> Result<&Self, Ctx::Err> {
        Ctx::validate(s.as_ref())?;
        Ok(unsafe { &*(s.as_ref() as *const str as *const Self) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;

    #[test]
    fn validate() {
        enum Name {}
        impl Validator for Name {
            type Err = ();
            fn validate(s: &str) -> Result<(), Self::Err> {
                if s.chars().count() > 4 {
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
        let s = "Bob".to_string();
        let s: Result<&strong<Name>, _> = strong::<Name>::validate(&s);
        assert!(s.is_err());
        let s = "Alice".to_string();
        let _: &strong<Name> = strong::<Name>::validate(&s).unwrap();
    }
}
