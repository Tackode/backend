pub mod configuration;
pub mod email;
pub mod local;

#[derive(Clone)]
pub struct ConnectorBuilder {
    configuration: configuration::ConnectorBuilder,
    local: local::ConnectorBuilder,
    email: email::ConnectorBuilder,
}

pub struct Connector {
    pub configuration: configuration::Connector,
    pub local: local::Connector,
    pub email: email::Connector,
}

impl ConnectorBuilder {
    pub fn new() -> Self {
        ConnectorBuilder {
            configuration: configuration::ConnectorBuilder::new(),
            local: local::ConnectorBuilder::new(),
            email: email::ConnectorBuilder::new(),
        }
    }

    pub fn create(&self) -> Connector {
        Connector {
            configuration: self.configuration.create(),
            local: self.local.create(),
            email: self.email.create(),
        }
    }
}
