use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset},
    router::{Router, RouterBuilderDiscoverExt, Slot, layout, module_router, page},
    view::{View, component, view},
};

mod about;

static TITLE: &str = "chen.tao's website";
const FERRIS: Asset = asset!("statics/style.css");

pub fn router() -> Router {
    module_router!()
        .discover()
        .assets(AssetBundle::load().unwrap())
        .build()
}

#[component]
async fn hello(name: &str) -> Result<impl View> {
    Ok(view! {<h1>"I'm " (name) "!"</h1>})
}

#[layout]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>(TITLE)</title>
                <link rel="stylesheet" href=(FERRIS)>
            </head>
            <body>
                hello(name: "陈涛")
                <ul>
                    <li><a href="/">"home"</a></li>
                    <li><a href="/about">"about"</a></li>
                </ul>
                (slot)
            </body>
        </html>
    })
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        "home"
    })
}
