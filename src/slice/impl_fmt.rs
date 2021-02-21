use crate::{
    marker::{DebugTransparent, DisplayTransparent},
    Strong, Validator
};
use std::fmt;

impl<Ctx: Validator + DebugTransparent> fmt::Debug for Strong<Ctx> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, formatter)
    }
}

impl<Ctx: Validator + DisplayTransparent> fmt::Display for Strong<Ctx> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.inner)
    }
}
