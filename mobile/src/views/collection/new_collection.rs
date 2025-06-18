use dioxus::prelude::*;

use crate::{server::collection::new_collection, Route};

#[component]
pub fn NewCollection() -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());

    let mut is_processing = use_signal(|| false);
    let mut failed = use_signal(|| false);

    fn soft_can_create(name: String) -> bool {
        !name.is_empty()
    }

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                form {
                    onsubmit: move |_| async move {

                        if !soft_can_create(name()) {
                            return
                        }

                        is_processing.set(true);
                        
                        let create_result = new_collection(name(), description()).await;
                        
                        let Ok(_) = create_result else {
                            is_processing.set(false);
                            failed.set(true);
                            return
                        };

                        is_processing.set(false);

                        navigator().push(Route::Collections { public: false });
                    },
                    class: "column gapLarge paddingLarge round bg2 centerColumn",
                    h1 {class: "textLarge", "New collection" },
                    input { class: "input bg3 borderBg4", oninput: move |e| name.set(e.value().clone()), placeholder: "Collection name" }
                    textarea { class: "input bg3 borderBg4", oninput: move |e| description.set(e.value().clone()), placeholder: "Collection description" }
                    button {
                        disabled: !soft_can_create(name()) || is_processing(), 
                        type: "submit", 
                        class: "button buttonBg3", 
                        if is_processing() { "..." }
                        else { "Create collection"}
                    }
                }
            }
        }
    }
}
