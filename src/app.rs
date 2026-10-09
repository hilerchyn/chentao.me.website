use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    router::{Router, RouterBuilderDiscoverExt, Slot, layout, module_router, page},
    view::{View, component, view},
};

mod about;

static TITLE: &str = "chen.tao's website";

pub fn router() -> Router {
    module_router!()
        .discover()
        .assets(AssetBundle::load().unwrap())
        .build()
}

#[component]
async fn hello(name: &str) -> Result<impl View> {
    Ok(view! {
        <h1 class="text-3xl font-semibold tracking-tight text-stone-950">
            "I'm " (name) "!"
        </h1>
    })
}

#[layout]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html class="h-full">
            <head>
                <title>(TITLE)</title>
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>
            </head>
            <body class="min-h-full bg-stone-50 text-stone-900 antialiased">
                <div class="mx-auto flex min-h-full max-w-2xl flex-col px-6 py-16">
                    <header class="border-b border-stone-200 pb-8">
                        hello(name: "陈涛(chen.tao)")
                        <nav class="mt-6">
                            <ul class="flex gap-6 text-sm font-medium">
                                <li>
                                    <a
                                        class="text-stone-600 underline-offset-4 hover:text-stone-950 hover:underline"
                                        href="/"
                                    >
                                        "home"
                                    </a>
                                </li>
                                <li>
                                    <a
                                        class="text-stone-600 underline-offset-4 hover:text-stone-950 hover:underline"
                                        href="/about"
                                    >
                                        "about"
                                    </a>
                                </li>
                            </ul>
                        </nav>
                    </header>
                    <main class="py-10 text-lg leading-8 text-stone-700">
                        (slot)
                    </main>
                </div>
            </body>
        </html>
    })
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <p>"home"</p>
    })
}
