use topcoat::{
    Result,
    router::page,
    view::{View, view},
};

#[page]
async fn about() -> Result<impl View> {
    Ok(view! {
        <script>"const element = document.getElementById('about');element.classList.add('border-b');"</script>
        <p>"About"</p>
    })
}
