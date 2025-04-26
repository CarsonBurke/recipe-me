use dioxus::prelude::*;

use ui::{views::app::App};

mod views;

fn main() {
    dioxus::launch(App);
}