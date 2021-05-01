use crate::connector::ConnectorBuilder;

pub const CONTENT_LENGTH_LIMIT: u64 = 1024 * 16;

#[derive(Clone)]
pub struct Context {
    pub builder: ConnectorBuilder,
}
