use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/rating_static.css");

#[component]
/// [rating]: a float between 0 and 5
pub fn RatingStatic(rating: f32) -> Element {

    let rating_percent = rating / 5. * 100.;

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        div {
            class: "ratingStatic row centerColumn",
            background_image: format!("linear-gradient(to right, yellow {rating_percent}%, var(--bg3) {rating_percent}%)"),
            div {
                class: "ratingStaticStar",
                "★"
            }
            div {
                class: "ratingStaticStar",
                "★"
            }
            div {
                class: "ratingStaticStar",
                "★"
            }
            div {
                class: "ratingStaticStar",
                "★"
            }
            div {
                class: "ratingStaticStar",
                "★"
            }
        }
    }
}