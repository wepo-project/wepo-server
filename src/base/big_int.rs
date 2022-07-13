use std::error::Error;

use serde::{Serialize, Deserialize, de::Visitor};
use tokio_postgres::types::{ToSql, Type, to_sql_checked, accepts, FromSql, IsNull};
use actix_web::web::BytesMut;


#[derive(Debug)]
pub struct BigInt(i64);

impl BigInt {
    pub fn new(num: i64) -> Self {
        Self(num)
    }
    // pub fn optional(num: Option<i64>) -> Option<Self> {
    //     num.map(|num| Self::new(num))
    // }
    pub fn inner(&self) -> &i64 {
        &self.0
    }
}

impl ToString for BigInt {
    fn to_string(&self) -> String {
        self.0.to_string()
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
        formatter.write_str("big int")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        let num = v.parse::<i64>().map_err(|_e| serde::de::Error::custom("parse to i64 failed"))?;
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
        let num = i64::from_sql(ty, raw)?;
        Ok(BigInt::new(num))
    }

    accepts!(INT8);
}