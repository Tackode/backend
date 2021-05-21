use std::env;

#[derive(Clone)]
pub struct GaugeConfiguration {
    pub alert: i64,
    pub warning: i64,
}

pub struct Connector {
    pub gauge: GaugeConfiguration,
}

#[derive(Clone)]
pub struct ConnectorBuilder {
    gauge: GaugeConfiguration,
}

impl ConnectorBuilder {
    pub fn new() -> ConnectorBuilder {
        let alert_gauge: i64 = env::var("ALERT_GAUGE")
            .map(|alert_gauge_str| alert_gauge_str.parse::<i64>().expect("Invalid ALERT_GAUGE"))
            .unwrap_or(90);

        let warning_gauge: i64 = env::var("WARNING_GAUGE")
            .map(|warning_gauge_str| {
                warning_gauge_str
                    .parse::<i64>()
                    .expect("Invalid WARNING_GAUGE")
            })
            .unwrap_or(80);

        let builder = ConnectorBuilder {
            gauge: GaugeConfiguration {
                alert: alert_gauge,
                warning: warning_gauge,
            },
        };

        builder
    }

    pub fn create(&self) -> Connector {
        Connector {
            gauge: self.gauge.clone(),
        }
    }
}
