use dioxus::prelude::*;

use ui::{views::app::App};

mod views;

fn main() {
    dioxus::launch(App);
}

// use dioxus::prelude::*;

// use views::{Blog, Home};

// mod views;

// fn main() {
//     dioxus::launch(App);
// }

// #[derive(Debug, Clone, Routable, PartialEq)]
// #[rustfmt::skip]
// pub enum Route {
//     #[layout(WebNavbar)]
//     #[route("/")]
//     Home {},
//     #[route("/blog/:id")]
//     Blog { id: i32 },
// }

// #[component]
// pub fn App() -> Element {

//     rsx! {
//         // Global app resources

//         Router::<Route> {}
//     }
// }

// /// A web-specific Router around the shared `Navbar` component
// /// which allows us to use the web-specific `Route` enum.
// #[component]
// fn WebNavbar() -> Element {
//     rsx! {
//         div {
//             "Hi"
//         }

//         Outlet::<Route> {}
//     }
// }