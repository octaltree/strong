use crate::{marker::HashTransparent, StrongBuf, Validator};
use std::hash::{Hash, Hasher};

impl<Ctx: Validator + HashTransparent> Hash for StrongBuf<Ctx> {
    fn hash<H: Hasher>(&self, h: &mut H) { self.inner.hash(h) }
}
