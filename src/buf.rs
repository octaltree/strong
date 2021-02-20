use crate::Validator;
use std::marker::PhantomData;

pub struct Strong<Ctx: Validator> {
    phantom: PhantomData<Ctx>,
    inner: String
}

impl<Ctx: Validator> Strong<Ctx> {
    /// creates from [`String`].
    pub fn validate(s: String) -> Result<Self, Ctx::Err> {
        Ctx::validate(&s)?;
        Ok(Self {
            inner: s,
            phantom: PhantomData
        })
    }

    /// creates new blank [`String`] and validtes it.
    pub fn with_capacity(capacity: usize) -> Result<Self, Ctx::Err> {
        let s = String::with_capacity(capacity);
        Self::validate(s)
    }
}
