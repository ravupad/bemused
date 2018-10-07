pub mod session;

use crate::sled::session::Session;
use sled::Db;

pub struct Sled {
    pub session: Session,
}

impl Sled {
    pub fn new(path: &str) -> Self {
        let db = Db::start_default(path).unwrap();
        let session = Session::new(&db);
        Sled { session }
    }
}
