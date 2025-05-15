use dioxus::prelude::*;

use crate::utils::is_logged_in;

#[component]
pub fn AccountDashboard() -> Element {
    /* let is_logged_in = is_logged_in(); */

    rsx! {
        "account dashboard"
    }
}