use crate::database::article;
use crate::model::article::Article;
use crate::server::Server;
use std::sync::Arc;

pub fn create(server: Server, user_id: i64, mut article: Article) -> crate::Result<()> {
    article.user_id = user_id;
    article::create(&server.pool, article).map(|_| ())
}

pub fn get(server: Server, user_id: i64, article_id: String) -> crate::Result<Article> {
    let article_id = Arc::new(article_id);
    article::get(&server.pool, user_id, &article_id)
}

pub fn list(server: Server, user_id: i64) -> crate::Result<Vec<Article>> {
    article::list(&server.pool, user_id)
}

pub fn update(server: Server, user_id: i64, mut article: Article) -> crate::Result<()> {
    article.user_id = user_id;
    article::update(&server.pool, article)
}

pub fn delete(server: Server, user_id: i64, article_id: String) -> crate::Result<()> {
    article::delete(&server.pool, user_id, &article_id)
}
