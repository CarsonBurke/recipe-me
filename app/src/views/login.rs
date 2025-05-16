use api::auth::ServerLoginToken;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;
use dioxus_sdk::storage::{LocalStorage, use_synced_storage};
use dioxus_toast::{ToastInfo, ToastManager};

use crate::{LOGIN_TOKEN_GLOBAL, Route, constants::LOGIN_TOKEN_KEY};

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let mut is_processing = use_signal(|| false);
    let mut failed_login = use_signal(|| false);

    fn soft_can_login(email: String, password: String) -> bool {
        if email.is_empty() || password.is_empty() {
            println!("empty fields");
            return false;
        }

        true
    }

    let mut toast: Signal<ToastManager> = use_context();

    /* let mut local_token = use_synced_storage::<LocalStorage, Option<ServerLoginToken>>("placeholder".to_string(), || None); */
    /* let mut count_local = use_synced_storage::<LocalStorage, i32>("synced".to_string(), || 0);
    println!("Local token 1: {:#?}", count_local); */

    rsx! {
        main {
            class: "main",
            section {
                class: "section column centerColumn",
                form {
                    onsubmit: move |e| async move {
                        if !soft_can_login(email(), password()) {
                            return
                        }

                        is_processing.set(true);

                        let login_token = api::auth::login(email(), password()).await;
                        println!("login result {:#?}", login_token);

                        let Ok(login_token) = login_token else {
                            let _ = toast.write().popup(ToastInfo::simple("Login failed"));
                            failed_login.set(true);
                            is_processing.set(false);
                            return
                        };

                        let _ = toast.write().popup(ToastInfo::simple("Login successful"));

                        /* let window = web_sys::window().unwrap();
                        let local_storage = window.local_storage().unwrap().unwrap();
                        local_storage.set_item("token", &login_token.token).unwrap(); */

                        /* LocalStorage::set("login_token", &login_token.token);
                        let token_check = LocalStorage::get::<ServerLoginToken>("login_token");
                        println!("Token check: {:#?}", token_check); */

                        let mut local_token = use_synced_storage::<LocalStorage, Option<ServerLoginToken>>(LOGIN_TOKEN_KEY.to_string(), || None);
                        println!("Local token 1: {:#?}", local_token);

                        *local_token.write() = Some(login_token.clone());

                        println!("Local token 2: {:#?}", local_token);

                        *LOGIN_TOKEN_GLOBAL.write() = Some(login_token);

                        is_processing.set(false);

                        // Send the user to their account dashboard

                        let navigator = navigator();
                        navigator.push(Route::AccountDashboard {});
                    },
                    class: "gapLarge column centerColumn bg2 round paddingLarge widthFit",
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
                    if failed_login() {
                        p { class: "textSmall textNegative", "Login failed, please try a different username or password" }
                    }
                    button {
                        class: "button buttonBg3",
                        type: "submit",
                        disabled: !soft_can_login(email(), password()) || is_processing(),
                        if is_processing() {
                            dioxus_free_icons::Icon { icon: dioxus_free_icons::icons::ld_icons::LdLoader}
                            "Trying to log you in..."
                        }
                        else { "Login" }
                    },
                }
            }
        }
    }
}
