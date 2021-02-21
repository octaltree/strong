use crate::{marker::HashTransparent, Strong, Validator};
use std::hash::{Hash, Hasher};

impl<Ctx: Validator + HashTransparent> Hash for Strong<Ctx> {
    fn hash<H: Hasher>(&self, h: &mut H) { self.inner.hash(h) }
}
