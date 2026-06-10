use std::{
    fmt::{self, Display, Formatter},
    num::NonZeroU8,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct ConfigVersion(NonZeroU8);

impl ConfigVersion {
    pub const V1: Self = Self(NonZeroU8::new(1).unwrap());
    pub const CURRENT: Self = Self::V1;

    pub fn new(version: u8) -> Option<Self> {
        NonZeroU8::new(version).map(Self)
    }

    pub fn get(self) -> u8 {
        self.0.get()
    }
}

#[derive(Debug)]
pub struct UnknownConfigVersion(u8);

impl Display for UnknownConfigVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "unknown config version: {}", self.0)
    }
}

impl TryFrom<u8> for ConfigVersion {
    type Error = UnknownConfigVersion;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::V1),
            value => Err(UnknownConfigVersion(value)),
        }
    }
}

impl<'de> Deserialize<'de> for ConfigVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;

        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
