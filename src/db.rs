
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
//pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create connection pool")
}
