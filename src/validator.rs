use std::fmt::Debug;

pub trait Validator {
    type Err: Debug;
    #[allow(unused_variables)]
    fn validate(raw: &str) -> Result<(), Self::Err> { Ok(()) }
}
