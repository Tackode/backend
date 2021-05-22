use std::fmt;
use std::io::Write;

use chrono_tz::Tz;
use diesel::backend::Backend;
use diesel::deserialize;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::types::FromSql;

#[derive(Debug, PartialEq, diesel_derive_enum::DbEnum)]
#[DieselType = "Gauge_level"]
pub enum GaugeLevel {
    Alert,
    Warning,
    Safe,
    Unknown,
}

impl fmt::Display for GaugeLevel {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GaugeLevel::Alert => "alert",
            GaugeLevel::Warning => "warning",
            GaugeLevel::Safe => "safe",
            GaugeLevel::Unknown => "unknown",
        })?;
        Ok(())
    }
}

pub fn join_gauge_levels(gauge_levels: Vec<GaugeLevel>) -> String {
    gauge_levels
        .into_iter()
        .map(|level| format!("'{}'", level))
        .collect::<Vec<String>>()
        .join(",")
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Text"]
pub struct Timezone {
    pub tz: Tz,
}

impl From<Tz> for Timezone {
    fn from(tz: Tz) -> Self {
        Timezone { tz }
    }
}

impl<DB> ToSql<Text, DB> for Timezone
where
    DB: Backend,
    String: ToSql<Text, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        self.tz.to_string().to_sql(out)
    }
}

impl<DB> FromSql<Text, DB> for Timezone
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let tz_str = String::from_sql(bytes)?;
        let tz = tz_str.parse::<Tz>()?;
        Ok(Timezone { tz })
    }
}
