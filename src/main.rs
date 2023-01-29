use yew::prelude::*;
use stylist::{css, Style};
use yew::prelude::*;


#[function_component]
fn App() -> Html {
    
    let s = css!(
        r#"
            color : ${color};
            span, ${sel_div} {
                background-color: blue;

            }


            @media screen and ${breakpoint} {
                display: flex;

            }
        "#,
        color = "red",
        sel_div = "div.selected",
        breakpoint = "(max-width: 500px)",
    );

    let style = Style::new(s).expect("msg");

    return html! {

        <>
        <div class={style}> {"Hello Word"} </div>
        </>
    };
}

// We load "App" and render it.
fn main() {
    yew::Renderer::<App>::new().render();
}
