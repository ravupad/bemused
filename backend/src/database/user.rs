use crate::database::{execute, query, CN};
use crate::error::ErrorCode;
use crate::Result;

const CHECK_USER_NAME: &str = "
    select id from users where username = $1";
const INSERT_USER: &str = "
    insert into users (username, password) values ($1, $2)";
const GET_USER: &str = "
    select id, password from users where username = $1";

pub fn add_user(cn: CN, username: &str, password: &str) -> Result<()> {
    query(&cn, CHECK_USER_NAME, &[&username])
        .and_then(|rows| {
            if rows.is_empty() {
                Ok(())
            } else {
                ErrorCode::UserNameTaken.default().err()
            }
        })
        .and_then(|_| execute(&cn, INSERT_USER, &[&username, &password]))
        .map(|_| ())
}

pub fn match_password(cn: CN, username: &str, password: &str) -> Result<i64> {
    query(&cn, GET_USER, &[&username]).and_then(|rows| {
        let row = match rows.len() {
            0 => return ErrorCode::UserNotFound.default().err(),
            1 => rows.get(0),
            _ => return ErrorCode::DatabaseError.default().err(),
        };
        if row.get::<_, String>(1).as_str() == password {
            Ok(row.get::<_, i64>(0))
        } else {
            ErrorCode::WrongPassword.default().err()
        }
    })
}
