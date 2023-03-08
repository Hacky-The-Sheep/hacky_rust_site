use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Welcome!" }</h1>
            <p>{ "This landing page was written in Rust 🚀 using Yew" }</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
