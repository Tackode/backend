#[derive(Debug, PartialEq, diesel_derive_enum::DbEnum)]
#[DieselType = "Gauge_level"]
pub enum GaugeLevel {
    Alert,
    Warning,
    Safe,
    Unknown,
}
