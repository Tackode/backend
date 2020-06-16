pub mod email;
pub mod local;

#[derive(Clone)]
pub struct ConnectorsBuilders {
    local: local::ConnectorBuilder,
    email: email::ConnectorBuilder,
}

pub struct Connectors {
    pub local: local::Connector,
    pub email: email::Connector,
}

impl ConnectorsBuilders {
    pub fn new() -> Self {
        ConnectorsBuilders {
            local: local::ConnectorBuilder::new(),
            email: email::ConnectorBuilder::new(),
        }
    }

    pub fn create(&self) -> Connectors {
        Connectors {
            local: self.local.create(),
            email: self.email.create(),
        }
    }
}
