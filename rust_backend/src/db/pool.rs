use dotenv::dotenv;
use r2d2;
use iron::typemap::Key;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
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

pub struct DieselPool(r2d2::Pool<ConnectionManager<PgConnection>>);
impl DieselPool {
    pub fn new() -> DieselPool {
    dotenv().ok();

    let database_url = get_env_var("DATABASE_URL".to_string()).unwrap();
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    DieselPool(r2d2::Pool::new(config, manager).unwrap())
    }
}
impl Key for DieselPool {
    type Value = DieselPool;
}
