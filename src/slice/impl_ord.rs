use crate::{
    marker::{EqTransparent, OrdTransparent, PartialEqTransparent, PartialOrdTransparent},
    Strong, Validator
};
use std::cmp::Ordering;

impl<Ctx: Validator + PartialEqTransparent> PartialEq for Strong<Ctx> {
    #[inline]
    fn eq(&self, other: &Strong<Ctx>) -> bool { self.as_str() == other.as_str() }
}

impl<Ctx: Validator + PartialEqTransparent + EqTransparent> Eq for Strong<Ctx> {}

impl<Ctx: Validator + PartialEqTransparent + PartialOrdTransparent> PartialOrd for Strong<Ctx> {
    #[inline]
    fn partial_cmp(&self, other: &Strong<Ctx>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<
        Ctx: Validator + PartialEqTransparent + EqTransparent + PartialOrdTransparent + OrdTransparent
    > Ord for Strong<Ctx>
{
    #[inline]
    fn cmp(&self, other: &Strong<Ctx>) -> Ordering { self.as_str().cmp(other.as_str()) }
}
