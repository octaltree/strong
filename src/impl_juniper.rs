use crate::{StrongBuf, Validator};

pub trait JuniperType: Validator {
    fn name() -> &'static str;
}

impl<__S, Ctx> ::juniper::GraphQLValueAsync<__S> for StrongBuf<Ctx>
where
    Ctx: JuniperType + 'static,
    Self: Sync,
    Self::TypeInfo: Sync,
    Self::Context: Sync,
    __S: ::juniper::ScalarValue + Send + Sync
{
    fn resolve_async<'a>(
        &'a self,
        info: &'a Self::TypeInfo,
        selection_set: Option<&'a [::juniper::Selection<__S>]>,
        executor: &'a ::juniper::Executor<Self::Context, __S>
    ) -> ::juniper::BoxFuture<'a, ::juniper::ExecutionResult<__S>> {
        use ::juniper::futures::future;
        let v = ::juniper::GraphQLValue::<__S>::resolve(self, info, selection_set, executor);
        Box::pin(future::ready(v))
    }
}

impl<__S, Ctx> ::juniper::GraphQLType<__S> for StrongBuf<Ctx>
where
    Ctx: JuniperType + 'static,
    __S: ::juniper::ScalarValue
{
    fn name(_: &Self::TypeInfo) -> Option<&'static str> { Some(<Ctx as JuniperType>::name()) }
    fn meta<'r>(
        info: &Self::TypeInfo,
        registry: &mut ::juniper::Registry<'r, __S>
    ) -> ::juniper::meta::MetaType<'r, __S>
    where
        __S: 'r
    {
        registry.build_scalar_type::<Self>(info).into_meta()
    }
}

impl<__S, Ctx> ::juniper::GraphQLValue<__S> for StrongBuf<Ctx>
where
    Ctx: JuniperType + 'static,
    __S: ::juniper::ScalarValue
{
    type Context = ();
    type TypeInfo = ();
    fn type_name<'__i>(&self, info: &'__i Self::TypeInfo) -> Option<&'__i str> {
        <Self as ::juniper::GraphQLType<__S>>::name(info)
    }
    fn resolve(
        &self,
        info: &(),
        selection: Option<&[::juniper::Selection<__S>]>,
        executor: &::juniper::Executor<Self::Context, __S>
    ) -> ::juniper::ExecutionResult<__S> {
        ::juniper::GraphQLValue::<__S>::resolve(self.as_str(), info, selection, executor)
    }
}

impl<__S, Ctx> ::juniper::ToInputValue<__S> for StrongBuf<Ctx>
where
    Ctx: JuniperType,
    __S: ::juniper::ScalarValue
{
    fn to_input_value(&self) -> ::juniper::InputValue<__S> {
        ::juniper::ToInputValue::<__S>::to_input_value(&self.as_str())
    }
}

impl<__S, Ctx> ::juniper::FromInputValue<__S> for StrongBuf<Ctx>
where
    Ctx: JuniperType,
    __S: ::juniper::ScalarValue
{
    fn from_input_value(v: &::juniper::InputValue<__S>) -> Option<Self> {
        let inner: String = ::juniper::FromInputValue::<__S>::from_input_value(v)?;
        Self::validate(inner).ok()
    }
}

impl<__S, Ctx> ::juniper::ParseScalarValue<__S> for StrongBuf<Ctx>
where
    Ctx: JuniperType,
    __S: ::juniper::ScalarValue
{
    fn from_str(
        value: ::juniper::parser::ScalarToken<'_>
    ) -> ::juniper::ParseScalarResult<'_, __S> {
        <String as ::juniper::ParseScalarValue<__S>>::from_str(value)
    }
}

impl<__S, Ctx> ::juniper::marker::IsOutputType<__S> for StrongBuf<Ctx>
where
    Ctx: JuniperType,
    __S: ::juniper::ScalarValue
{
}
impl<__S, Ctx> ::juniper::marker::IsInputType<__S> for StrongBuf<Ctx>
where
    Ctx: JuniperType + 'static,
    __S: ::juniper::ScalarValue
{
}
