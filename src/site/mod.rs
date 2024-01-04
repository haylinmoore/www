use maud::{html, Markup};
use crate::SiteState;
pub mod home;

pub fn base(content: Markup, _state: SiteState) -> Markup {
    html! {
        (maud::DOCTYPE)
            html lang="en" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";

                    title { "Ezri" };
                }
                body {
                    (content)
                }
            }
    }
}
