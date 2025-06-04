use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();

    rsx! {
        div { id: "home-container",
            p { id: "home-title", "Bank of Skibidi" }
            div { id: "home-div",
                button {
                    id: "home-login",
                    onclick: move |_| {
                        navigator.push(Route::Login {});
                    },
                    "Login"
                },
                button {
                    id: "home-register",
                    onclick: move |_| {
                        navigator.push(Route::Register {});
                    },
                    "Register"
                }
            }
        }
    }
}
