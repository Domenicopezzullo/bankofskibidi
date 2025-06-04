use dioxus::prelude::*;
use crate::utils::{database::add_user, utilities::calculate_hash};



#[component]
pub fn Register() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    rsx! {
        div { id: "register-container",
            p { id: "register-title", "Register" }
            label { id: "register-username-label", r#for: "register-username", "Username" }
            input { onchange: move |e: Event<FormData>| {
                username.set(e.value().clone())
            }, name: "Username", placeholder: "ThatItalianDude", type: "text", id: "register-username" }
            label { id: "register-password-label", r#for: "register-password", "Password" }
            input { onchange: move |e: Event<FormData>| {
                password.set(e.value().clone())
            }, name: "Username", placeholder: "Skibidi@12", type: "password", id: "register-password" }
            button { onclick: move |_| {
                let hashed_pass = calculate_hash(password().as_str()); 
                add_user(username().as_str(), &hashed_pass);
                
            }, id: "register-register-btn", "Register" }
        }
    }
}
