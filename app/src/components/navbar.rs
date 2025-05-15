use dioxus::prelude::*;

use crate::{utils, views::recipe::recipes, Route, LOGIN_TOKEN_GLOBAL};

const CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {

    let is_logged_in = utils::is_logged_in().unwrap_or(false);
    /* let login_token = LOGIN_TOKEN_GLOBAL(); */
    println!("is logged in: {:#?}", is_logged_in);

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        nav {
            id: "navbar",
            class: "navbar paddingSectionSide row bg3 gapSmall",
            Link {
                to: Route::Home {},
                h1 {
                    class: "textLarge button buttonBg3",
                    "Recipe Me"
                }
            }
            div {
                class: "row centerColumn gapMedium",
                Link {
                    class: "buttonBg3 button",
                    to: Route::Home {},
                    "Home"
                }
                Link {
                    class: "buttonBg3 button",
                    to: Route::Recipes { query: recipes::Query::default() },
                    "Recipes"
                }
                if is_logged_in {
                    Link {
                        class: "buttonBg3 button",
                        to: Route::AccountDashboard {  },
                        "Dashboard"
                    }
                    Link {
                        class: "buttonBg3 button",
                        to: Route::AccountRecipes {  },
                        "My Recipes"
                    }
                    Link {
                        class: "buttonBg3 button",
                        to: Route::Account {  },
                        "Account"
                    }
                }
                if !is_logged_in {
                    Link {
                        class: "buttonBg3 button",
                        to: Route::Login {  },
                        "Login"
                    }
                    Link {
                        class: "buttonBg3 button",
                        to: Route::Signup {  },
                        "Create an account"
                    }
                }
            }
        }
    }
}
