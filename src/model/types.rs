use std::fmt;

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
