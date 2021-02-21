use crate::{marker::CloneTransparent, StrongBuf, Validator};

/// Implements Clone only if [`Validator`] has [`Clone`].
impl<Ctx: Validator + CloneTransparent> Clone for StrongBuf<Ctx> {
    /// # Panics
    /// Panics if [`Validator`] rejects cloned string.
    #[inline]
    fn clone(&self) -> Self { Self::validate(self.inner.clone()).unwrap() }
}
