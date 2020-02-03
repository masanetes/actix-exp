use std::env;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};
use actix_web::{web, HttpResponse};

pub type DBPool = Pool<ConnectionManager<MysqlConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

fn init_pool(database_url: &str)-> Result<DBPool, PoolError> {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        Pool::builder().build(manager)
}

pub fn establish_connection() -> DBPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
}

pub fn pool_handler(pool: web::Data<DBPool>) -> Result<DBPooledConnection, HttpResponse> {
    pool.get().map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}
