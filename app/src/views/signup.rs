use api::auth::{ServerLoginToken, create_account};
use dioxus::prelude::*;
use dioxus_sdk::storage::{LocalStorage, use_synced_storage};
use dioxus_toast::{ToastInfo, ToastManager};

use crate::{LOGIN_TOKEN_GLOBAL, Route, constants::LOGIN_TOKEN_KEY};

#[component]
pub fn Signup() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut confirm_password = use_signal(|| "".to_string());

    let mut failed = use_signal(|| false);
    let mut is_processing = use_signal(|| false);

    fn soft_can_create_account(
        username: String,
        email: String,
        password: String,
        confirm_password: String,
    ) -> bool {
        if username.is_empty()
            || email.is_empty()
            || password.is_empty()
            || confirm_password.is_empty()
        {
            println!("empty fields");
            return false;
        }

        if password != confirm_password {
            println!("passwords do not match");
            return false;
        }

        if password.len() < 6 {
            println!("Password is too short");
            return false;
        }

        true
    }

    let mut toast: Signal<ToastManager> = use_context();

    rsx! {
        main {
            class: "main",
            section {
                class: "section column centerColumn",
                form {
                    onsubmit: move |_| async move {
                        println!("Username: {}", username());

                        if !soft_can_create_account(username(), email(), password(), confirm_password()) {
                            return;
                        }
                        println!("passed soft checks");

                        is_processing.set(true);

                        println!("client side account create");
                        let login_token = create_account(username.to_string(), email.to_string(), password.to_string()).await;
                        println!("Potential login token {:?}", login_token);

                        let Ok(login_token) = login_token else {
                            let _ = toast.write().popup(ToastInfo::simple("Signup failed"));
                            failed.set(true);
                            is_processing.set(false);
                            return
                        };

                        let _ = toast.write().popup(ToastInfo::simple("Successfully created account"));

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
                    h1 { class: "textLarge", "Create an account" },
                    div {
                        class: "column gapMedium centerColumn",
                        div {
                            class: "column gapSmall",
                            label { class: "textSmall", "Enter a username" },
                            input {
                                class: "input bg3 borderBg4",
                                placeholder: "Username",
                                type: "text/username",
                                oninput: move |e| {
                                    username.set(e.value().clone())
                                },
                            },
                        },
                        div {
                            class: "column gapSmall",
                            label { class: "textSmall", "Enter an email" },
                            input {
                                class: "input bg3 borderBg4",
                                placeholder: "Email",
                                type: "email",
                                oninput: move |e| {
                                    email.set(e.value().clone())
                                },
                            },
                        },
                        div {
                            class: "column gapSmall",
                            div { 
                                class: "row gapMedium", label { class: "textSmall", "Enter a password" }, 
                                if !password().is_empty() { 
                                    p { class: "textSmall", {format!("{}/6", password().len())} } 
                                }
                        },
                            input {
                                class: "input bg3 borderBg4",
                                placeholder: "Password",
                                type: "password",
                                oninput: move |e| {
                                    password.set(e.value().clone())
                                },
                            },
                        },
                        div {
                            class: "column gapSmall",
                            label { class: "textSmall", "Repeat your password" },
                            input {
                                class: "input bg3 borderBg4",
                                placeholder: "Repeat password",
                                type: "password",
                                oninput: move |e| {
                                    confirm_password.set(e.value().clone())
                                },
                            },
                        },
                        {
                            let password = password();
                            let confirm_password = confirm_password();
                            if password.len() > 0 && confirm_password.len() > 0 {
                                if password != confirm_password {
                                    rsx!{
                                        p {
                                            class: "textSmall textNegative",
                                            "Passwords don't match"
                                        }
                                    }
                                }
                                else {
                                    rsx!{
                                        p {
                                            class: "textSmall textPositive",
                                            "Passwords match"
                                        }
                                    }
                                }
                            }
                            else {
                                rsx ! {}
                            }
                        }
                    },
                    if failed() {
                        p { class: "textSmall textNegative", "Signup failed, please try a different email" }
                    }
                    button {
                        class: "button buttonBg3",
                        type: "submit",
                        disabled: !soft_can_create_account(username(), email(), password(), confirm_password()) || is_processing(),
                        if is_processing() {
                            dioxus_free_icons::Icon { icon: dioxus_free_icons::icons::ld_icons::LdLoader}
                            "Trying to create your account..."
                        }
                        else { "Create new account" }
                    },
                }
            }
        }
    }
}
