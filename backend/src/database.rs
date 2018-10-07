pub mod article;
pub mod task;
pub mod user;

use crate::Result;
use crate::{error::Error, Future};
use futures::future::{self, Future as _, IntoFuture};
use futures::{
    sink::Sink,
    stream::Stream,
    sync::{mpsc, oneshot},
};
use postgres::rows::Rows;
use postgres::types::ToSql;
use r2d2_postgres::PostgresConnectionManager;
use std::thread;
use tokio_core::reactor::Core;

type MG = PostgresConnectionManager;
type PL = r2d2::Pool<MG>;
type CN = r2d2::PooledConnection<MG>;
type FP = Box<FnOnce(PL) -> () + Send>;

pub struct Database(mpsc::Sender<FP>);

impl Database {
    pub fn new(pool: PL) -> Database {
        let (tx, rx) = mpsc::channel(1);
        let future = rx.for_each(move |cmd: FP| {
            cmd(pool.clone());
            future::ok(())
        });
        thread::spawn(move || {
            let mut core = Core::new().expect("core fail");
            core.run(future).expect("core fail")
        });
        Database(tx)
    }

    pub fn run<F, T>(&self, f: F) -> Future<T>
    where
        F: FnMut(CN) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        let mut tx = Some(tx);
        let cmd = Box::new(move |pool: PL| {
            let conn = pool.get().map_err(Error::from);
            let result = conn.and_then(f);
            tx.take().map(|tx| tx.send(result)).is_some();
        });
        Box::new(
            self.0
                .clone()
                .send(cmd)
                .map_err(|e| Error::internal(&e))
                .and_then(|_| rx.map_err(|e| Error::internal(&e)))
                .and_then(|res| res.into_future()),
        )
    }
}

pub fn query(cn: &CN, query: &str, args: &[&dyn ToSql]) -> Result<Rows> {
    cn.query(query, args).map_err(Error::from)
}

pub fn execute(cn: &CN, query: &str, args: &[&dyn ToSql]) -> Result<u64> {
    cn.execute(query, args).map_err(Error::from)
}
