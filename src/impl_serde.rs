use crate::{Strong, StrongBuf, Validator};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<'de, Ctx> Deserialize<'de> for StrongBuf<Ctx>
where
    Ctx: Validator
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Self::validate(s).map_err(serde::de::Error::custom)
    }
}

impl<'de, Ctx> Deserialize<'de> for &'de Strong<Ctx>
where
    Ctx: Validator
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let s = <&str>::deserialize(deserializer)?;
        Strong::validate(s).map_err(serde::de::Error::custom)
    }
}

impl<Ctx> Serialize for Strong<Ctx>
where
    Ctx: Validator
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<Ctx> Serialize for StrongBuf<Ctx>
where
    Ctx: Validator
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(self.as_str())
    }
}
