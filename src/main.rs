use dioxus::prelude::*;
mod routes;
mod utils;
use routes::{dashboard::Dashboard, home::Home, login::Login, register::Register};

const CSS: Asset = asset!("./assets/main.css");

#[derive(Routable, Debug, Clone, PartialEq)]
enum Route {
    #[redirect("/", || Route::Home {})]
    #[route("/home")]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/dashboard")]
    Dashboard {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        Router::<Route> {}
    }
}
