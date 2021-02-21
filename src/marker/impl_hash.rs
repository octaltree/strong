use crate::{marker::HashTransparent, Strong, StrongBuf, Validator};
use std::hash::{Hash, Hasher};

impl<Ctx: Validator + HashTransparent> Hash for Strong<Ctx> {
    fn hash<H: Hasher>(&self, h: &mut H) { self.as_str().hash(h) }
}

impl<Ctx: Validator + HashTransparent> Hash for StrongBuf<Ctx> {
    fn hash<H: Hasher>(&self, h: &mut H) { self.as_str().hash(h) }
}
