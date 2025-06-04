use std::hash;

use dioxus::prelude::*;
use crate::utils::{database::add_user, utilities::calculate_hash};



#[component]
pub fn Register() -> Element {
    let username = use_signal(|| "");
    let password = use_signal(|| "");
    rsx! {
        div { id: "register-container",
            p { id: "register-title", "Register" }
            label { id: "register-username-label", r#for: "register-username", "Username" }
            input { name: "Username", placeholder: "ThatItalianDude", type: "text", id: "register-username" }
            label { id: "register-password-label", r#for: "register-password", "Password" }
            input { name: "Username", placeholder: "Skibidi@12", type: "password", id: "register-password" }
            button { onclick: move |_| {
                let hashed_pass = calculate_hash(password()); 
                add_user(username(), &hashed_pass);
            }, id: "register-register-btn", "Register" }
        }
    }
}
