use crate::{marker::HashTransparent, Strong, StrongBuf, Validator};
use std::hash::{Hash, Hasher};

impl<Ctx: HashTransparent> Hash for Strong<Ctx>
where
    Ctx: Validator
{
    fn hash<H: Hasher>(&self, h: &mut H) { self.as_str().hash(h) }
}

impl<Ctx: HashTransparent> Hash for StrongBuf<Ctx>
where
    Ctx: Validator
{
    fn hash<H: Hasher>(&self, h: &mut H) { self.as_str().hash(h) }
}
