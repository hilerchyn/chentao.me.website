use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, component, view},
};

#[tokio::main]
async fn main() {
    topcoat::start(module_router!().build()).await.unwrap();
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
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
