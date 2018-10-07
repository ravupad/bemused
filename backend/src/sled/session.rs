use crate::error::Error;
use crate::Result;
use sled::{Db, Tree};
use std::convert::TryInto;
use std::sync::Arc;

pub struct Session {
    tree: Arc<Tree>,
}

impl Session {
    pub fn new(db: &Db) -> Self {
        Session {
            tree: db.open_tree("session").unwrap(),
        }
    }

    pub fn save(&self, session_id: &str, user_id: i64) -> Result<()> {
        self.tree
            .set(session_id.as_bytes(), user_id.to_be_bytes().to_vec())
            .map(|_| ())
            .map_err(Error::from)
    }

    pub fn get(&self, session_id: &str) -> Result<Option<i64>> {
        self.tree
            .get(session_id.as_bytes())
            .map(|optional| {
                optional.map(|vec| i64::from_be_bytes(vec.as_ref().try_into().unwrap()))
            })
            .map_err(Error::from)
    }

    pub fn del(&self, session_id: &str) -> Result<()> {
        self.tree
            .del(session_id.as_bytes())
            .map(|_| ())
            .map_err(Error::from)
    }
}
