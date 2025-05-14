use api::create_account;
use dioxus::prelude::*;

#[component]
pub fn Signup() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut confirm_password = use_signal(|| "".to_string());

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
                                    password.set(e.value().clone())
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
                                            class: "textSmall",
                                            "Passwords don't match"
                                        }
                                    }
                                }
                                else {
                                    rsx!{
                                        p {
                                            class: "textSmall",
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
                        onclick: move |_| {
                            if password() != confirm_password() {
                                return;
                            }

                            if password().len() < 6 {
                                return;
                            }

                            async move {
                                let token = create_account(username.to_string(), email.to_string(), password.to_string()).await;
                                println!("Potential login token {:?}", token);
                            };
                        },
                        "Create new account"
                    },
                }
            }
        }
    }
}
