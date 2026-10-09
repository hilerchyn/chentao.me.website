use topcoat::{
    Result,
    router::page,
    view::{View, view},
};

#[page]
async fn about() -> Result<impl View> {
    Ok(view! {
        <p>"About"</p>
    })
}
