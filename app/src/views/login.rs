use api::auth::ServerLoginToken;
use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager};
use gloo_storage::{LocalStorage, Storage};

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    fn soft_can_login(email: String, password: String) -> bool {
        if email.is_empty() || password.is_empty() {
            println!("empty fields");
            return false;
        }

        true
    }

    let mut toast: Signal<ToastManager> = use_context();

    rsx! {
        main {
            class: "main",
            section {
                class: "section",
                form {
                    class: "gapLarge column centerColumn bg2 round paddingMedium widthFit",
                    h1 { class: "textLarge", "Login" },
                    div {
                        class: "column gapMedium",
                        div {
                            class: "column gapSmall",
                            label { class: "textSmall", "Enter your email" },
                            input {
                                class: "input bg3 borderBg4",
                                placeholder: "Your email",
                                type: "email",
                                oninput: move |e| {
                                    email.set(e.value().clone())
                                },
                            },
                        },
                        div {
                            class: "column gapSmall",
                            label { class: "textSmall", "Enter your password" },
                            input {
                                class: "input bg3 borderBg4",
                                placeholder: "Your password",
                                type: "password",
                                oninput: move |e| {
                                    password.set(e.value().clone())
                                },
                            },
                        },
                    },
                    button {
                        class: "button buttonBg3",
                        type: "submit",
                        disabled: !soft_can_login(email(), password()),
                        onclick: move |_| async move {
                            if !soft_can_login(email(), password()) {
                                return
                            }

                            let login_result = api::auth::login(email(), password()).await;
                            println!("login result {:#?}", login_result);

                            let Ok(login_result) = login_result else {
                                return
                            };

                            let _ = toast.write().popup(ToastInfo::simple("Login successful"));

                            /* let window = web_sys::window().unwrap();
                            let local_storage = window.local_storage().unwrap().unwrap();
                            local_storage.set_item("token", &login_result.token).unwrap(); */

                            /* LocalStorage::set("login_token", &login_result.token).unwrap();
                            let token_check: ServerLoginToken = LocalStorage::get("login_token").unwrap();
                            println!("Token check: {:#?}", token_check); */
                        },
                        "Login"
                    },
                }
            }
        }
    }
}
