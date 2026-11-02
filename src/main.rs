mod blog;
mod i18n;

use askama::Template;
use axum::{
    extract,
    extract::Path,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use blog::{catalog, index_href, kind_href, post_href, KindView, PostView};
use i18n::{Lang, Page, T};
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
        .route("/", get(|| async { handler_home(Lang::Zh) }))
        .route("/en", get(|| async { handler_home(Lang::En) }))
        .route("/about", get(|| async { handler_about(Lang::Zh) }))
        .route("/en/about", get(|| async { handler_about(Lang::En) }))
        .route("/blog", get(|| async { handler_blog(Lang::Zh, None) }))
        .route("/en/blog", get(|| async { handler_blog(Lang::En, None) }))
        .route(
            "/blog/{kind}",
            get(|Path(kind): Path<String>| async move { handler_blog(Lang::Zh, Some(kind)) }),
        )
        .route(
            "/en/blog/{kind}",
            get(|Path(kind): Path<String>| async move { handler_blog(Lang::En, Some(kind)) }),
        )
        .route(
            "/blog/{kind}/{slug}",
            get(|Path((kind, slug)): Path<(String, String)>| async move {
                handler_blog_post(Lang::Zh, kind, slug)
            }),
        )
        .route(
            "/en/blog/{kind}/{slug}",
            get(|Path((kind, slug)): Path<(String, String)>| async move {
                handler_blog_post(Lang::En, kind, slug)
            }),
        )
}

fn handler_home(lang: Lang) -> impl IntoResponse {
    let t = T::new(lang, Page::Home);
    HtmlTemplate(HomeTemplate {
        nav_name: "home",
        switch_href: t.switch_href.to_string(),
        t,
    })
}

fn handler_about(lang: Lang) -> impl IntoResponse {
    let t = T::new(lang, Page::About);
    HtmlTemplate(AboutTemplate {
        nav_name: "about",
        switch_href: t.switch_href.to_string(),
        t,
    })
}

fn handler_blog(lang: Lang, kind: Option<String>) -> Response {
    let cat = catalog();
    if let Some(id) = kind.as_deref() {
        if cat.kind(id).is_none() {
            return StatusCode::NOT_FOUND.into_response();
        }
    }
    let t = T::new(lang, Page::Blog);
    let kind_meta = kind.as_deref().and_then(|id| cat.kind(id));
    let heading = kind_meta
        .map(|k| k.name(lang).to_string())
        .unwrap_or_else(|| t.blog.title.to_string());
    let lead = kind_meta
        .map(|k| k.desc(lang).to_string())
        .unwrap_or_else(|| t.blog.lead.to_string());
    let switch_href = match kind.as_deref() {
        Some(id) => kind_href(lang.other(), id),
        None => index_href(lang.other()).to_string(),
    };
    let alt_zh = match kind.as_deref() {
        Some(id) => kind_href(Lang::Zh, id),
        None => index_href(Lang::Zh).to_string(),
    };
    let alt_en = match kind.as_deref() {
        Some(id) => kind_href(Lang::En, id),
        None => index_href(Lang::En).to_string(),
    };
    HtmlTemplate(BlogTemplate {
        nav_name: "blog",
        switch_href,
        alt_zh_href: alt_zh,
        alt_en_href: alt_en,
        heading,
        lead,
        all_active: kind.is_none(),
        kinds: cat.kind_views(lang, kind.as_deref()),
        posts: cat.post_views(lang, kind.as_deref()),
        t,
    })
    .into_response()
}

fn handler_blog_post(lang: Lang, kind: String, slug: String) -> Response {
    let cat = catalog();
    let Some(post) = cat.post(&kind, &slug, lang) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let kind_meta = cat.kind(&kind);
    let t = T::new(lang, Page::Blog);
    let switch_href = if cat.post(&kind, &slug, lang.other()).is_some() {
        post_href(lang.other(), &kind, &slug)
    } else {
        kind_href(lang.other(), &kind)
    };
    HtmlTemplate(BlogPostTemplate {
        nav_name: "blog",
        switch_href,
        alt_zh_href: post_href(Lang::Zh, &kind, &slug),
        alt_en_href: post_href(Lang::En, &kind, &slug),
        post_title: post.title.clone(),
        summary: post.summary.clone(),
        date: post.date.clone(),
        kind_name: kind_meta
            .map(|k| k.name(lang).to_string())
            .unwrap_or_else(|| kind.clone()),
        kind_href: kind_href(lang, &kind),
        body: post.html(),
        t,
    })
    .into_response()
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
struct HomeTemplate {
    nav_name: &'static str,
    switch_href: String,
    t: T,
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    nav_name: &'static str,
    switch_href: String,
    t: T,
}

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    nav_name: &'static str,
    switch_href: String,
    alt_zh_href: String,
    alt_en_href: String,
    heading: String,
    lead: String,
    all_active: bool,
    kinds: Vec<KindView>,
    posts: Vec<PostView>,
    t: T,
}

#[derive(Template)]
#[template(path = "blog_post.html")]
struct BlogPostTemplate {
    nav_name: &'static str,
    switch_href: String,
    alt_zh_href: String,
    alt_en_href: String,
    post_title: String,
    summary: String,
    date: String,
    kind_name: String,
    kind_href: String,
    body: String,
    t: T,
}

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

    #[tokio::test]
    async fn test_home_page() {
        let response = app()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert!(html.contains("把想法"));
        assert!(html.contains("打开 GitHub"));
        assert!(html.contains("lang=\"zh-CN\""));
        assert!(html.contains("href=\"/en\""));
        assert!(html.contains("/static/css/sketch.css"));
        assert!(html.contains("/static/img/hero-desk.jpg"));
    }

    #[tokio::test]
    async fn test_home_page_en() {
        let response = app()
            .oneshot(Request::get("/en").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert!(html.contains("Sketch the idea"));
        assert!(html.contains("Open GitHub"));
        assert!(html.contains("lang=\"en\""));
        assert!(html.contains("href=\"/\""));
        assert!(html.contains("hreflang=\"zh-CN\""));
    }

    #[tokio::test]
    async fn test_about_page() {
        let response = app()
            .oneshot(Request::get("/about").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert!(html.contains("简历"));
        assert!(html.contains("管理能力"));
        assert!(html.contains("技术能力"));
        assert!(html.contains("lang=\"zh-CN\""));
        assert!(html.contains("href=\"/en/about\""));
        assert!(html.contains("/static/css/sketch.css"));
        assert!(html.contains("/static/img/resume-desk.jpg"));
    }

    #[tokio::test]
    async fn test_about_page_en() {
        let response = app()
            .oneshot(Request::get("/en/about").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert!(html.contains("Resume"));
        assert!(html.contains("Management"));
        assert!(html.contains("Technical"));
        assert!(html.contains("lang=\"en\""));
        assert!(html.contains("href=\"/about\""));
        assert!(html.contains("/static/css/sketch.css"));
    }

    #[tokio::test]
    async fn test_blog_index() {
        let response = app()
            .oneshot(Request::get("/blog").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let html = String::from_utf8(
            response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .to_vec(),
        )
        .unwrap();
        assert!(html.contains("博客"));
        assert!(html.contains("架构设计"));
        assert!(html.contains("编程沉思"));
        assert!(html.contains("读书笔记"));
        assert!(html.contains("项目管理"));
        assert!(html.contains("产品设计"));
        assert!(html.contains("AI探索"));
        assert!(html.contains("先把边界画出来"));
        assert!(html.contains("href=\"/en/blog\""));
        assert!(html.contains("/static/css/sketch.css"));
    }

    #[tokio::test]
    async fn test_blog_index_en() {
        let response = app()
            .oneshot(Request::get("/en/blog").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let html = String::from_utf8(
            response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .to_vec(),
        )
        .unwrap();
        assert!(html.contains("Blog"));
        assert!(html.contains("Architecture"));
        assert!(html.contains("Draw the edges first"));
        assert!(html.contains("lang=\"en\""));
        assert!(html.contains("href=\"/blog\""));
    }

    #[tokio::test]
    async fn test_blog_kind_and_post() {
        let kind = app()
            .oneshot(
                Request::get("/blog/architecture")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(kind.status(), StatusCode::OK);
        let kind_html = String::from_utf8(
            kind.into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .to_vec(),
        )
        .unwrap();
        assert!(kind_html.contains("架构设计"));
        assert!(kind_html.contains("/blog/architecture/lay-the-path"));

        let post = app()
            .oneshot(
                Request::get("/blog/architecture/lay-the-path")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(post.status(), StatusCode::OK);
        let post_html = String::from_utf8(
            post.into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .to_vec(),
        )
        .unwrap();
        assert!(post_html.contains("先把边界画出来"));
        assert!(post_html.contains("href=\"/en/blog/architecture/lay-the-path\""));

        let missing = app()
            .oneshot(Request::get("/blog/no-such-type").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(missing.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_home_assets() {
        for path in [
            "/static/css/sketch.css",
            "/static/js/sketch.js",
            "/static/img/hero-desk.jpg",
            "/static/img/resume-desk.jpg",
            "/static/img/favicon.svg",
        ] {
            let response = app()
                .oneshot(Request::get(path).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK, "missing {path}");
        }
    }
}
