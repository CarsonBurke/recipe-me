use dioxus::prelude::*;
use dioxus_free_icons::icons::{ld_icons, io_icons};

#[component]
pub fn DownloadPage() -> Element {
    rsx! {
        main {
            class: "main",
            section {
                class: "section column centerColumn gapLarge",
                h1 {
                    class: "textXLarge",
                    "Download"
                }
                div {
                    class: "row gapMedium flexWrap",
                    button {
                        class: "paddingLarge round centerRow centerColumn buttonBg2 square widthFit column button",
                        dioxus_free_icons::Icon { icon: io_icons::IoLogoAndroid }
                        p {
                            class: "textLarge",
                            "Android",
                        }
                    }
                    button {
                        class: "paddingLarge round centerRow centerColumn buttonBg2 square widthFit column button",
                        dioxus_free_icons::Icon { icon: io_icons::IoLogoApple }
                        p {
                            class: "textLarge",
                            "IOS",
                        }
                    }
                    button {
                        class: "paddingLarge round centerRow centerColumn buttonBg2 square widthFit column button",
                        dioxus_free_icons::Icon { icon: io_icons::IoLogoTux }
                        p {
                            class: "textLarge",
                            "Linux",
                        }
                    }
                    button {
                        class: "paddingLarge round centerRow centerColumn buttonBg2 square widthFit column button",
                        dioxus_free_icons::Icon { icon: io_icons::IoLogoAppleAppstore }
                        p {
                            class: "textLarge",
                            "MacOS",
                        }
                    }
                    button {
                        class: "paddingLarge round centerRow centerColumn buttonBg2 square widthFit column button",
                        dioxus_free_icons::Icon { icon: io_icons::IoLogoWindows }
                        p {
                            class: "textLarge",
                            "Windows",
                        }
                    }
                }
            }
        }
    }
}