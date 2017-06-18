use dotenv::dotenv;
use r2d2;
use iron::typemap::Key;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use utils::get_env_var;

pub struct DieselPool(pub r2d2::Pool<ConnectionManager<PgConnection>>);
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
