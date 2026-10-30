use askama::Template;
use axum::{
    extract,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rust_embed::RustEmbed;

// 1. Define your asset folder to embed
#[derive(RustEmbed)]
#[folder = "assets/"] // Path relative to Cargo.toml
struct Asset;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with some routes
    let app = app();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await;
}

// 3. Fallback/catch-all handler for embedded assets
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    // Strip the route prefix if necessary (e.g., "/static/style.css" -> "style.css")
    if path.starts_with("static/") {
        path = path.replacen("static/", "", 1);
    }

    match Asset::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(axum::body::Body::from(content.data))
                .unwrap()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

fn app() -> Router {
    Router::new()
        .route("/greet/{name}", get(greet))
        .route("/static/{*file}", get(static_handler))
        .route("/about", get(handler_about))
        .route("/", get(handler_home))
}

async fn handler_home() -> impl IntoResponse {
    let template = HomeTemplate {};
    HtmlTemplate(template)
}

async fn handler_about() -> impl IntoResponse {
    let template = AboutTemplate {};
    HtmlTemplate(template)
}

async fn greet(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    let template = HelloTemplate { name };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_main() {
        let response = app()
            .oneshot(Request::get("/greet/Foo").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(html, "<h1>Hello, Foo!</h1>");
    }
}
