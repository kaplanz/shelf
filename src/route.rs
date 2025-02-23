use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::Query;
use log::{error, trace};
use tokio::sync::RwLock;

use crate::dbase::{Content, Database};
use crate::types::{Bookmark, Filter, Set};

pub async fn root(
    Query(filter): Query<Filter>,
    State(data): State<Arc<RwLock<Database>>>,
) -> Json<Content> {
    // Run query on database
    let data: Content = data
        .read()
        .await
        .view()
        .iter()
        .filter(|item| filter.check(item))
        .cloned()
        .collect();
    // Reply with matched items
    trace!("query matched {} bookmarks", data.len());
    Json::from(data)
}

pub async fn push(State(data): State<Arc<RwLock<Database>>>, Json(item): Json<Bookmark>) {
    // Push item to database
    trace!("push: {item:?}");
    data.write().await.push(item);
}

pub async fn sync(State(data): State<Arc<RwLock<Database>>>) -> impl IntoResponse {
    // Sync the database
    trace!("sync database");
    data.write()
        .await
        .sync()
        .await
        .inspect_err(|err| error!("{err}"))
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn tags(
    Query(filter): Query<Filter>,
    State(data): State<Arc<RwLock<Database>>>,
) -> Json<Set<String>> {
    // Collect all tags
    let data: Set<String> = data
        .read()
        .await
        .view()
        .iter()
        .filter(|item| filter.check(item))
        .flat_map(|item| item.tags.iter())
        .cloned()
        .collect();
    // Reply with tags
    trace!("query matched {} tags", data.len());
    Json::from(data)
}

pub async fn categories(
    Query(filter): Query<Filter>,
    State(data): State<Arc<RwLock<Database>>>,
) -> Json<Set<String>> {
    // Collect all categories
    let data: Set<String> = data
        .read()
        .await
        .view()
        .iter()
        .filter(|item| filter.check(item))
        .flat_map(|item| item.categories.iter())
        .cloned()
        .collect();
    // Reply with categories
    trace!("query matched {} categories", data.len());
    Json::from(data)
}
