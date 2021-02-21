use crate::{
    marker::{EqTransparent, OrdTransparent, PartialEqTransparent, PartialOrdTransparent},
    Strong, Validator
};
use std::cmp::Ordering;

impl<Ctx: Validator + PartialEqTransparent> PartialEq for Strong<Ctx> {
    #[inline]
    fn eq(&self, other: &Strong<Ctx>) -> bool { self.inner == other.inner }
}

impl<Ctx: Validator + PartialEqTransparent + EqTransparent> Eq for Strong<Ctx> {}

impl<Ctx: Validator + PartialEqTransparent + PartialOrdTransparent> PartialOrd for Strong<Ctx> {
    #[inline]
    fn partial_cmp(&self, other: &Strong<Ctx>) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<
        Ctx: Validator + PartialEqTransparent + EqTransparent + PartialOrdTransparent + OrdTransparent
    > Ord for Strong<Ctx>
{
    #[inline]
    fn cmp(&self, other: &Strong<Ctx>) -> Ordering { self.inner.cmp(&other.inner) }
}
