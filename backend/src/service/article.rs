use crate::database::article;
use crate::model::article::Article;
use crate::server::Server;
use crate::Future;
use futures::future::Future as _;
use std::sync::Arc;

pub fn create(server: Server, user_id: i64, mut article: Article) -> Future<()> {
    article.user_id = user_id;
    let article = Arc::new(article);
    let fut = server
        .database
        .run(move |cn| article::create(cn, &article))
        .map(|_| ());
    Box::new(fut)
}

pub fn get(server: Server, user_id: i64, article_id: String) -> Future<Article> {
    let article_id = Arc::new(article_id);
    let fut = server
        .database
        .run(move |cn| article::get(cn, user_id, &article_id));
    Box::new(fut)
}

pub fn list(server: Server, user_id: i64) -> Future<Vec<Article>> {
    let fut = server.database.run(move |cn| article::list(cn, user_id));
    Box::new(fut)
}

pub fn update(server: Server, user_id: i64, mut article: Article) -> Future<()> {
    article.user_id = user_id;
    let article = Arc::new(article);
    let fut = server.database.run(move |cn| article::update(cn, &article));
    Box::new(fut)
}

pub fn delete(server: Server, user_id: i64, article_id: String) -> Future<()> {
    let article_id = Arc::new(article_id);
    let fut = server
        .database
        .run(move |cn| article::delete(cn, user_id, &article_id));
    Box::new(fut)
}
