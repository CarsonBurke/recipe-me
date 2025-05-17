use dioxus::prelude::*;
use dioxus_free_icons::icons::{io_icons, ld_icons};

use crate::{views::recipe::recipes, Route};

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {

    const BACKGROUND: Asset = asset!("/assets/images/heroBg.png");

    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }

        section {
            class: "hero column centerColumn",
            img { src: "{BACKGROUND}", class: "heroBg" }
            header {
                class: "paddingLarge heroHeader centerColumn column gapLarge bg2 round",
                div {
                    class: "column centerColumn gapXSmall",
                    h1 { 
                        class: "textXXLarge",
                        "Recipe me"
                    }
                    p {
                        class: "textLarge",
                        "Your ultimate recipe companion"
                    }
                }
                div {
                    class: "row flexWrap gapMedium",
                    Link {
                        class: "button buttonBg3",
                        to: Route::Recipes { query: recipes::Query::default() },
                        "Find recipes"
                    }
                    Link {
                        class: "button borderBg3 buttonBg2",
                        to: Route::DownloadPage {  },
                        "Download"
                    }
                }
                Link {
                    class: "row gapSmall textSmall",
                    to: Route::DownloadPage {  },
                    dioxus_free_icons::Icon { icon: io_icons::IoLogoWindows }
                    dioxus_free_icons::Icon { icon: io_icons::IoLogoAndroid }
                    dioxus_free_icons::Icon { icon: io_icons::IoLogoApple }
                    dioxus_free_icons::Icon { icon: io_icons::IoLogoAppleAppstore }
                    dioxus_free_icons::Icon { icon: io_icons::IoLogoTux }
                }
            }
            div {
                class: "column gapSmall centerColumn",
                p { class: "textSmall", "Learn more"}
                p {
                    class: "textMedium animateFloatUpDown square round row centerRow centerColumn paddingMedium",
                    dioxus_free_icons::Icon { icon: ld_icons::LdChevronDown }
                }
            }
        }
    }
}
