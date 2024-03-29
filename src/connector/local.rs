use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::embed_migrations;
use std::env;

embed_migrations!("./migrations");

pub struct Connector {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Clone)]
pub struct ConnectorBuilder {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ConnectorBuilder {
    pub fn new() -> ConnectorBuilder {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool_size: u32 = env::var("DATABASE_POOL_SIZE")
            .map(|pool_size_str| {
                pool_size_str
                    .parse::<u32>()
                    .expect("Invalid DATABASE_POOL_SIZE")
            })
            .unwrap_or(15);

        let manager = ConnectionManager::<PgConnection>::new(database_url.clone());

        let builder = ConnectorBuilder {
            pool: Pool::builder()
                .max_size(pool_size)
                .build(manager)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
        };

        let connection = builder
            .pool
            .get()
            .expect("Unable to connect for migrations");

        embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
            .expect("Unable to run migrations");

        builder
    }

    pub fn create(&self) -> Connector {
        Connector {
            pool: self.pool.clone(),
        }
    }
}
