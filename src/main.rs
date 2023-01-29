use yew::prelude::*;


#[function_component]
fn App() -> Html {
    return html! {
        <>
        <div> { "Hello Word" } </div>
        </>
    };
}

// We load "App" and render it.
fn main() {
    #[cfg(feature = "render")]
    yew::Renderer::<App>::new().render();
}
