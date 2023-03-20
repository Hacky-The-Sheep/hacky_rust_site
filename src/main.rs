use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let items = vec!["Page One ", "Page Two ", "Page Three "];
    html! {
        <>
            <h1>{ "Welcome!" }</h1>
            <p>{ "This landing page was written in Rust 🚀 using Yew" }</p>
            <h2>{ "This site is dedicated to Our Lady of Perpetual Help" }</h2>
            <>
            <figure class="ol-figure">
                <img class="hacky" src="https://raw.githubusercontent.com/Hacky-The-Sheep/hacky_rust_site/main/media/OLPH.jpeg"/>
                <figcaption class="ol_fig"> { "Pray for us! "}</figcaption>
            </figure>
            </>
            // <img class="hacky" src="https://raw.githubusercontent.com/Hacky-The-Sheep/hacky_rust_site/main/media/OLPH.jpeg"/>
            <br/>
            <p>{ "The page will be added on as I continue to learn Rust and Web development. Adding to it, I believe I can keep this site
            to simply a page, with links to other platforms. Although, I think that adding links to single (smaller) sites I find for others
            to view will still be a goal. Just need to determine how to add that to the page... 🤔 

            Maybe something like:" }</p>
            <div class="sample_items">
                {
                    items.into_iter().map(|items| {
                        html!{<div key={items}>{ format!("• {items}") }</div>}
                    }).collect::<Html>()
                }
            </div>
            <p>{ "Or, I can host the sites on SurrealDB and do a call to pull a few of them at random. That might be an issue
            with scaling in the future but I only have 3 sites in mind right now so that will be future Hackys problem I suppose."}</p>
            <a class="cat" href="https://github.com/catppuccin/catppuccin">{"Themed with Catppuccin 🐱"}</a>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
