use crate::{
    marker::{DebugTransparent, DisplayTransparent},
    StrongBuf, Validator
};
use std::fmt;

impl<Ctx: Validator + DebugTransparent> fmt::Debug for StrongBuf<Ctx> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, formatter)
    }
}

impl<Ctx: Validator + DisplayTransparent> fmt::Display for StrongBuf<Ctx> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.inner)
    }
}
