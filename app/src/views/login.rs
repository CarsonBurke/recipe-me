use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let mut username_or_email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    rsx! {
        main {
            class: "main",
            section {
                class: "section",
                div {
                    class: "gapLarge column centerColumn bg2 round paddingMedium widthFit",
                    h1 { class: "textLarge", "Login" },
                    div {
                        class: "column gapMedium",
                        div {
                            class: "column gapSmall",
                            label { class: "textSmall", "Enter your username or email" },
                            input {
                                class: "input bg3 borderBg4",
                                placeholder: "Username or email",
                                type: "text/email",
                                oninput: move |e| {
                                    username_or_email.set(e.value().clone())
                                },
                            },
                        },
                        div {
                            class: "column gapSmall",
                            label { class: "textSmall", "Enter your password" },
                            input {
                                class: "input bg3 borderBg4",
                                placeholder: "Password",
                                type: "password",
                                oninput: move |e| {
                                    password.set(e.value().clone())
                                },
                            },
                        },
                    },
                    button {
                        class: "button buttonBg3",
                        onclick: move |_| {
                            async move {
                                let login_result = api::login(username_or_email(), password());
                                println!("login result {:#?}", login_result.await);
                            }
                        },
                        "Login"
                    },
                }
            }
        }
    }
}
