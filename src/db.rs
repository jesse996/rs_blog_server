use diesel::r2d2::{ConnectionManager, Pool};

use actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::PgConnection;

mod user;

//database actor
pub struct Dba(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for Dba {
    type Context = SyncContext<Self>;
}

pub type DbAddr = Addr<Dba>;

pub fn init_dba() -> DbAddr {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manage = ConnectionManager::<PgConnection>::new(db_url);
    let thread_num = 16;
    let conn = Pool::builder()
        .max_size(thread_num)
        .build(manage)
        .expect("Fail to create db pool");
    SyncArbiter::start(thread_num as usize, move || Dba(conn.clone()))
}
