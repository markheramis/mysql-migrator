use std::sync::Arc;
use mysql::Pool;
use crate::connection::ConnectionDatabaseConfig;
pub struct Database {
    pub pool: Arc<Pool>,
    pub name: String,
}
impl Database { 
    pub fn new(conf: &ConnectionDatabaseConfig) -> Self {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            conf.username,
            conf.password,
            conf.hostname,
            conf.port,
            conf.database
        );
        let pool: Pool = Pool::new(url.as_str()).unwrap();
        let pool: Arc<Pool> = Arc::new(pool);
        Self {
            pool,
            name: conf.database.clone()
        }
    }
    
    
}