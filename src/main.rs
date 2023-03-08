use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Welcome!" }</h1>
            <p>{ "This landing page was written in Rust 🚀 using Yew" }</p>
            <br/>
            <h2>{ "Enjoy this picture of a sheep... Mr Robot Style" }</h2>
            <br/>
            <img class="hacky" src="https://github.com/Hacky-The-Sheep/hacky_rust_site/blob/main/media/Hacky_robot.png?raw=true"/>
            <p>{ "The page will be added on as I continue to learn Rust and Web development" }</p>

        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
