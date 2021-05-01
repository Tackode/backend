use crate::connector::ConnectorBuilder;
use crate::serve;

pub async fn run(builder: ConnectorBuilder) {
    serve::run(builder).await;
}
