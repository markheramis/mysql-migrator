use mysql::*;
use mysql::prelude::*;

use crate::DatabaseConfig;

pub struct Database {
    pub conn: PooledConn,
    pub name: String,
}
impl Database {
    pub fn new(config: &DatabaseConfig) -> Self {
        let opts: OptsBuilder = OptsBuilder::new()
            .ip_or_hostname(Some(config.host.clone()))  
            .tcp_port(config.port) 
            .user(Some(config.user.clone()))
            .pass(Some(config.pass.clone()))
            .db_name(Some(config.name.clone()));
        let pool: Pool = Pool::new(opts).unwrap();
        let conn: PooledConn = pool.get_conn().unwrap();
        Self { conn, name: config.name.clone() }
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
