use crate::{marker::DefaultTransparent, StrongBuf, Validator};

/// Implements Default only if [`Validator`] has [`Default`].
impl<Ctx: DefaultTransparent> Default for StrongBuf<Ctx>
where
    Ctx: Validator
{
    /// # Panics
    /// Panics if [`Validator`] rejects blank string.
    fn default() -> Self { StrongBuf::validate(String::new()).unwrap() }
}
