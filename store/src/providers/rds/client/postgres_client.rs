use deadpool_postgres::{Object, Pool, Runtime};
use tokio_postgres::NoTls;

#[allow(dead_code)]
pub struct DataBaseManager {
    pub connection_pool: Pool,
}

impl DataBaseManager {
    pub fn new(config: deadpool_postgres::Config) -> Self {
        let pool = config
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|err| err.to_string())
            .unwrap();
        Self {
            connection_pool: pool,
        }
    }

    pub async fn connection(&self) -> Result<Object, String> {
        self.connection_pool
            .get()
            .await
            .map_err(|err| err.to_string())
    }
}