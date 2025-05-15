use dioxus::prelude::*;

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
                        disabled: !soft_can_login(email(), password()),
                        onclick: move |_| async move {
                            if !soft_can_login(email(), password()) {
                                return
                            }

                            let login_result = api::auth::login(email(), password()).await;
                            println!("login result {:#?}", login_result);
                        },
                        "Login"
                    },
                }
            }
        }
    }
}
