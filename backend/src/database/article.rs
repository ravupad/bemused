use crate::error::Error;
use crate::error::ErrorCode;
use crate::model::article::Article;
use crate::Result;
use r2d2_postgres::PostgresConnectionManager;

const INSERT: &str = "
insert into article(id, user_id, title, text, tags)
values ($1, $2, $3, $4, $5)";

const GET: &str = "
select title, text, tags from article
where id = $1 and user_id = $2";

const LIST: &str = "
select id, title, tags from article
where user_id = $1";

const UPDATE: &str = "
update article
set title = $3, text = $4, tags = $5
where id = $1 and user_id = $2";

const DELETE: &str = "
delete from article
where id = $1 and user_id = $2";

pub fn create(pool: &r2d2::Pool<PostgresConnectionManager>, article: Article) -> Result<()> {
    pool.get()
        .map_err(Error::from)?
        .execute(
            INSERT,
            &[
                &article.id,
                &article.user_id,
                &article.title,
                &article.text,
                &from_tag(&article.tags),
            ],
        )
        .map_err(Error::from)
        .map(|_| ())
}

pub fn get(
    pool: &r2d2::Pool<PostgresConnectionManager>,
    user_id: i64,
    id: &str,
) -> Result<Article> {
    pool.get()
        .map_err(Error::from)?
        .query(GET, &[&id, &user_id])
        .map_err(Error::from)
        .and_then(|rows| {
            let row = match rows.len() {
                1 => rows.get(0),
                _ => return ErrorCode::ResourceNotFound.default().err(),
            };
            let article = Article {
                id: id.to_string(),
                user_id,
                title: row.get(0),
                text: row.get(1),
                tags: to_tag(&row.get::<_, String>(2)),
            };
            Ok(article)
        })
}

pub fn list(pool: &r2d2::Pool<PostgresConnectionManager>, user_id: i64) -> Result<Vec<Article>> {
    pool.get()
        .map_err(Error::from)?
        .query(LIST, &[&user_id])
        .map_err(Error::from)
        .map(|rows| {
            rows.iter()
                .map(|row| Article {
                    id: row.get(0),
                    user_id,
                    title: row.get(1),
                    text: String::new(),
                    tags: to_tag(&row.get::<_, String>(2)),
                })
                .collect()
        })
}

pub fn update(pool: &r2d2::Pool<PostgresConnectionManager>, article: Article) -> Result<()> {
    pool.get()
        .map_err(Error::from)?
        .execute(
            UPDATE,
            &[
                &article.id,
                &article.user_id,
                &article.title,
                &article.text,
                &from_tag(&article.tags),
            ],
        )
        .map_err(Error::from)
        .map(|_| ())
}

pub fn delete(pool: &r2d2::Pool<PostgresConnectionManager>, user_id: i64, id: &str) -> Result<()> {
    pool.get()
        .map_err(Error::from)?
        .execute(DELETE, &[&id, &user_id])
        .map_err(Error::from)
        .map(|_| ())
}

fn from_tag(tags: &[String]) -> String {
    let mut result;
    if tags.len() > 1 {
        result = tags[0].to_string();
    } else {
        return String::new();
    }
    for tag in tags {
        result = result + "," + tag;
    }
    result
}

fn to_tag(tag: &str) -> Vec<String> {
    tag.split(',').map(|tag| tag.to_string()).collect()
}
