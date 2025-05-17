use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::{utils, views::{account, recipe::recipes}, Route, LOGIN_TOKEN_GLOBAL};

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
            div {
                class: "row centerColumn gapMedium paddingSmall",
                Link {
                    to: Route::Home {},
                    h1 {
                        class: "textLarge button buttonBg3",
                        "Recipe Me"
                    }
                }
            }
            div {
                class: "row centerColumn gapMedium paddingSmall",

                Link {
                    class: "buttonBg3 button",
                    to: Route::Recipes { query: recipes::Query::default() },
                    "Find recipes"
                }
                if is_logged_in {
                    Link {
                        class: "buttonBg3 button",
                        to: Route::AccountDashboard {  },
                        "Dashboard"
                    }
                    Link {
                        class: "buttonBg3 button",
                        to: Route::AccountRecipes { query: account::recipes::Query::default() },
                        "My Recipes"
                    }
                    Link {
                        class: "buttonBg3 button",
                        to: Route::Account {  },
                        dioxus_free_icons::Icon { icon: ld_icons::LdUserRound }
                    }
                }
                if !is_logged_in {
                    Link {
                        class: "buttonBg3 button",
                        to: Route::Login {  },
                        "Login"
                    }
                    Link {
                        class: "buttonBg3 button borderBg4",
                        to: Route::Signup {  },
                        "Create an account"
                    }
                }
                
            }
        }
    }
}
