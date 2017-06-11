use r2d2;
use iron::typemap::Key;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use utils::get_env_var;

pub struct PgConnPool(pub r2d2::Pool<PostgresConnectionManager>);
impl PgConnPool {
    pub fn new() -> PgConnPool {
        let database_url = get_env_var("DATABASE_URL".to_string()).unwrap();
        let pg_pool_manager = PostgresConnectionManager::new(database_url, TlsMode::None).unwrap();
        PgConnPool(r2d2::Pool::new(r2d2::Config::default(), pg_pool_manager).unwrap())
    }
}
impl Key for PgConnPool {
    type Value = PgConnPool;
}