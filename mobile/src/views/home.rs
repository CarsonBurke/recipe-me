use dioxus::prelude::*;

use crate::server;

#[component]
pub fn Home() -> Element {

    let x = use_server_future(|| {
        async move {
            server::ping_self().await
        }
    }).unwrap();

    println!("x: {:?}", x);

    rsx! {
        h1 { "Home" }
    }
}
