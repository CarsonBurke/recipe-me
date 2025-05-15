use api::auth::create_account;
use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager};

#[component]
pub fn Signup() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut confirm_password = use_signal(|| "".to_string());

    fn soft_can_create_account(username: String, email: String, password: String, confirm_password: String) -> bool {
        if username.is_empty() || email.is_empty() || password.is_empty() || confirm_password.is_empty() {
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
                class: "section",
                div {
                    class: "gapLarge column centerColumn bg2 round paddingMedium widthFit",
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
                            label { class: "textSmall", "Enter a password" },
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
                    button {
                        class: "button buttonBg3",
                        disabled: !soft_can_create_account(username(), email(), password(), confirm_password()),
                        onclick: move |_| async move {
                            println!("Username: {}", username());

                            if !soft_can_create_account(username(), email(), password(), confirm_password()) {
                                return;
                            }
                            println!("passed soft checks");
                            
                            println!("client side account create");
                            let token = create_account(username.to_string(), email.to_string(), password.to_string()).await;
                            println!("Potential login token {:?}", token);

                            if let Ok(token) = token {
                                let _ = toast.write().popup(ToastInfo::simple("Successfully created account"));
                            }
                        },
                        "Create new account"
                    },
                }
            }
        }
    }
}

