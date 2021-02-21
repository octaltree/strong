use crate::{StrongBuf, Validator};

/// Implements Default only if [`Validator`] has [`Default`].
impl<Ctx: Validator + Default> Default for StrongBuf<Ctx> {
    /// # Panics
    /// Panics if [`Validator`] rejects blank string.
    fn default() -> Self { StrongBuf::validate(String::new()).unwrap() }
}
