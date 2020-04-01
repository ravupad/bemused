use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager as PCM;
use tokio::sync::mpsc::channel as mchannel;
use tokio::sync::mpsc::Receiver as MReceiver;
use tokio::sync::mpsc::Sender as MSender;
type F = Box<dyn FnOnce(Pool<PCM>) -> () + Send>;

pub struct Database {
    tx: MSender<F>,
}

impl Database {
    pub fn new(pool: Pool<PCM>) -> Self {
        let (tx, rx) = mchannel::<F>(100);
        let future = start_future(rx, pool);
        std::thread::spawn(move || tokio::runtime::Runtime::new().unwrap().block_on(future));
        Database { tx }
    }

    pub async fn run<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&Pool<PCM>) -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let f2 = move |pool| {
            tx.send(f(&pool)).map_err(|_| ()).unwrap_or(());
        };
        let f3 = Box::new(f2);
        self.tx.clone().send(f3).await.map_err(|_| ()).unwrap_or(());
        rx.await.unwrap()
    }
}

async fn start_future(mut rx: MReceiver<F>, pool: Pool<PCM>) {
    loop {
        if let Some(req) = rx.recv().await {
            req(pool.clone());
        } else {
            break;
        }
    }
}
