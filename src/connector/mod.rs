pub mod email;
pub mod local;

#[derive(Clone)]
pub struct ConnectorBuilder {
    local: local::ConnectorBuilder,
    email: email::ConnectorBuilder,
}

pub struct Connector {
    pub local: local::Connector,
    pub email: email::Connector,
}

impl ConnectorBuilder {
    pub fn new() -> Self {
        ConnectorBuilder {
            local: local::ConnectorBuilder::new(),
            email: email::ConnectorBuilder::new(),
        }
    }

    pub fn create(&self) -> Connector {
        Connector {
            local: self.local.create(),
            email: self.email.create(),
        }
    }
}
