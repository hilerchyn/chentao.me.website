use topcoat::{
    Result,
    router::{Router, page},
    view::{View, component, view},
};

static TITLE: &str = "chen.tao's website";

#[tokio::main]
async fn main() {
    topcoat::start(Router::builder().page(home).page(about).build())
        .await
        .unwrap();
}

#[page("/")]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <title>(TITLE)</title>
            <body>
                hello(name: "陈涛")
            </body>
        </html>
    })
}

#[component]
async fn hello(name: &str) -> Result<impl View> {
    Ok(view! {<h1>"I'm " (name) "!"</h1>})
}

#[page("/about")]
async fn about() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <title>(TITLE)</title>
            <body>
                "About"
            </body>
        </html>
    })
}
