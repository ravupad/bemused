use sled::Db;
use crate::error::Error;
use crate::Result;
use crate::utils::sled::in_transaction;

pub trait Entity {
    fn name() -> &'static str;
    fn body(&self) -> Vec<u8>;
    fn indexes() -> Vec<(&'static str, bool, Box<dyn Fn(&Self) -> Vec<u8>>)>;
}

fn create<T: Entity>(sled: &Db, t: &mut T) -> Result<u64> {
    let tree = sled.open_tree(T::name()).map_err(Error::from)?;
    tree.transaction(move |transactional_tree| {
        let id = in_transaction::generate_id(transactional_tree)?;
        let key = in_transaction::serialize(&(true, id))?;
        transactional_tree.insert(&key[..], t.body())?;
        for idx in T::indexes() {
            let index_name = idx.0;
            let unique = idx.1;
            let index_key = idx.2(&t);
            let (key, val) = if unique {
                (in_transaction::serialize(&(false, index_name, index_key))?, &key[..])
            } else {
                (in_transaction::serialize(&(false, index_name, index_key, &key[..]))?, &key[..])
            };
            transactional_tree.insert(key, val)?;
        }
        Ok(id)
    }).map_err(Error::sled)
}

fn read<T: Entity>(sled: &Db, id: u64) -> Result<Option<T>> {

}

fn update(sled: &Db, t: &mut T) {

}

fn delete() {

}

