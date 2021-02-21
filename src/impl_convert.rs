//! impl From, TryFrom
use crate::{StrongBuf, Validator};
use std::{convert::TryFrom, str::FromStr};

impl<Ctx: Validator> TryFrom<String> for StrongBuf<Ctx> {
    type Error = Ctx::Err;
    fn try_from(s: String) -> Result<Self, Self::Error> { Self::validate(s) }
}

impl<Ctx: Validator> FromStr for StrongBuf<Ctx> {
    type Err = Ctx::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Self::validate(s.to_string()) }
}

// TODO: Box, Rc, Arc, Cow
