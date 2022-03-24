#![deny(warnings)]

use warp::{hyper::header::HeaderName, Filter};

pub(crate) fn cross_origin_embedder_policy(reply: impl warp::Reply) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        HeaderName::from_static("cross-origin-embedder-policy"),
        "require-corp",
    )
}

pub(crate) fn cross_origin_opener_policy(reply: impl warp::Reply) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        HeaderName::from_static("cross-origin-opener-policy"),
        "same-origin",
    )
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let dir = std::env::current_dir().unwrap();
    let mut file = dir.clone();
    file.push("index.html");

    let dir_filter = warp::fs::dir(dir)
        .map(cross_origin_embedder_policy)
        .map(cross_origin_opener_policy);

    let file_filter = warp::fs::file(file)
        .map(cross_origin_embedder_policy)
        .map(cross_origin_opener_policy);

    let filter = dir_filter.or(file_filter);

    warp::serve(filter).run(([127, 0, 0, 1], 3030)).await;
}
