use axum::http::Request;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultOnRequest, DefaultOnResponse, MakeSpan, TraceLayer},
};
use tracing::{span, Level, Span};
use uuid::Uuid;

#[derive(Clone, Copy)]
pub(crate) struct MakeSpanWithId;

impl<B> MakeSpan<B> for MakeSpanWithId {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        span!(
            Level::INFO,
            "request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
            id = %Uuid::new_v4(),
        )
    }
}

/// Add a layer for logging
pub(crate) fn layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, MakeSpanWithId> {
    TraceLayer::new_for_http()
        .make_span_with(MakeSpanWithId)
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}
