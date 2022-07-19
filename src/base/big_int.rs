use std::{error::Error, fmt::Display, num::ParseIntError, ops::Deref};

use log::info;
use serde::{Serialize, Deserialize, de::Visitor};
use tokio_postgres::types::{ToSql, Type, to_sql_checked, accepts, FromSql, IsNull};
use actix_web::web::BytesMut;


#[derive(Debug, Clone, Copy)]
pub struct BigInt(i64);

impl BigInt {
    pub fn new(num: i64) -> Self {
        Self(num)
    }
    pub fn inner(&self) -> &i64 {
        &self.0
    }
}

impl TryFrom<String> for BigInt {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::new(value.parse::<i64>()?))
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl Serialize for BigInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for BigInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_string(BigIntVistor)
    }
}

struct BigIntVistor;

impl<'de> Visitor<'de> for BigIntVistor {
    type Value = BigInt;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("BigInt(i64)")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        let num = v.parse::<i64>().map_err(|_e| serde::de::Error::custom("parse to i64 failed"))?;
        Ok(BigInt::new(num))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        let num = v.parse::<i64>().map_err(|_e| serde::de::Error::custom("parse to i64 failed"))?;
        Ok(BigInt::new(num))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        let str = String::from_utf8_lossy(v).to_string();
        let num = str.parse::<i64>().map_err(|_e| serde::de::Error::custom("parse to i64 failed"))?;
        Ok(BigInt::new(num))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(BigInt::new(v))
    }
}

impl ToSql for BigInt {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized {
        self.0.to_sql(ty, out)
    }

    accepts!(INT8);

    to_sql_checked!();
}

impl<'a> FromSql<'a> for BigInt {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let num: i64 = match ty {
            &Type::INT8 => i64::from_sql(ty, raw)?,
            &Type::TEXT => String::from_sql(ty, raw)?.parse::<i64>()?,
            _ => unreachable!("accepts type unmatch")
        };
        Ok(BigInt::new(num))
    }

    accepts!(INT8, TEXT);
}

impl Deref for BigInt {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}