use mysql::*;
use mysql::prelude::*;
use crate::ConnectionDatabaseConfig;

pub struct Database {
    pub conn: PooledConn,
    pub name: String,
}
impl Database {
    pub fn new(database_config: &ConnectionDatabaseConfig) -> Self {
        let opts: OptsBuilder = OptsBuilder::new()
            .ip_or_hostname(Some(database_config.hostname.clone()))  
            .tcp_port(database_config.port) 
            .user(Some(database_config.username.clone()))
            .pass(Some(database_config.password.clone()))
            .db_name(Some(database_config.database.clone()));
        let pool: Pool = Pool::new(opts).unwrap();
        let conn: PooledConn = pool.get_conn().unwrap();
        Self { conn, name: database_config.database.clone() }
    }
    pub fn query_columns(&mut self, table: &str) -> Vec<String> {
        let query: String = format!(
            "SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = '{}' AND TABLE_SCHEMA = '{}'",
            table, self.name
        );
        self.conn.query_map(
            query, 
            |column_name| column_name
        ).unwrap()
    }
    pub fn query_table_unbuffered(
        &mut self, 
        table: &str, 
        columns: &str, 
        condition: &Option<String>
    ) -> Result<QueryResult<'_, '_, '_, Binary>> {
        let mut query = format!("SELECT {} FROM {}", columns, table);
        if let Some(cond) = condition {
            query.push_str(&format!(" WHERE {}", cond));
        }
        self.conn.exec_iter(query, ())
    }
}
