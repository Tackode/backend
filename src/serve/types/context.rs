use crate::connector::ConnectorsBuilders;

pub const CONTENT_LENGTH_LIMIT: u64 = 1024 * 16;

#[derive(Clone)]
pub struct Context {
    pub builders: ConnectorsBuilders,
}
