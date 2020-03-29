use crate::error::Error;
use crate::error::ErrorCode;
use crate::Result;
use r2d2_postgres::PostgresConnectionManager;

const CHECK_USER_NAME: &str = "
    select id from users where username = $1";
const INSERT_USER: &str = "
    insert into users (username, password) values ($1, $2)";
const GET_USER: &str = "
    select id, password from users where username = $1";

pub fn add_user(
    pool: &r2d2::Pool<PostgresConnectionManager>,
    username: &str,
    password: &str,
) -> Result<()> {
    let cn = &pool.get().map_err(Error::from)?;
    cn.query(CHECK_USER_NAME, &[&username])
        .map_err(Error::from)
        .and_then(|rows| {
            if rows.is_empty() {
                Ok(())
            } else {
                ErrorCode::UserNameTaken.default().err()
            }
        })
        .and_then(|_| {
            cn.execute(INSERT_USER, &[&username, &password])
                .map_err(Error::from)
        })
        .map(|_| ())
}

pub fn match_password(
    pool: &r2d2::Pool<PostgresConnectionManager>,
    username: &str,
    password: &str,
) -> Result<i64> {
    let cn = &pool.get().map_err(Error::from)?;
    cn.query(GET_USER, &[&username])
        .map_err(Error::from)
        .and_then(|rows| {
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
