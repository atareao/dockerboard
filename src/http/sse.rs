use tokio_stream::StreamExt;
use docker_api::{Docker, models::EventMessage, opts::EventsOptsBuilder};
use axum::{
    headers,
    TypedHeader,
    extract::State,
    response::sse::{Event, Sse},
    routing,
    Router,
};
use futures::stream::{self, Stream, TryStreamExt};
use std::{convert::Infallible, path::PathBuf, time::Duration};

use crate::AppState;
use std::sync::Arc;


pub fn router() -> Router<Arc<AppState>>{
    Router::new()
        .route("/sse",
            routing::get(sse_handler)
        )
}

async fn sse_handler(
    State(app_state): State<Arc<AppState>>,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    //) -> impl Stream<Item = Result<models::EventMessage>> + Unpin + 'docker {
) -> Sse<impl Stream<Item = Result<EventMessage>>> {
//) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let uri = app_state.config.get_docker_uri().unwrap();
    let docker = Docker::new(uri).unwrap();
    let stream2 = docker.events(&EventsOptsBuilder::default().build()).map(
        |item| {

        }item.unwrap()
    )
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream2).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
