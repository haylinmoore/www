use super::{base, PageContext};
use crate::words::Post;
use crate::{ClientState, SiteState};
use axum::extract::{Extension, State};
use maud::{html, Markup, PreEscaped};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn home(
    State(state): State<Arc<RwLock<SiteState>>>,
    Extension(client_state): Extension<ClientState>,
) -> Markup {
    let state = state.read().await;

    let things = state.things[0..5].to_vec();
    let words: Vec<Post>;

    let word_count = state.words.len();

    if word_count > 5 {
        words = state.words[0..5].to_vec();
    } else {
        words = state.words.clone();
    }

    let content = html! {
        div class="pure-g hero section" style="position: relative" {
            div class="pure-u-1 pure-u-md-2-3 hero-text" style="margin-bottom: 32px;" {
                h1 { "Hampton Moore" }
                p {
                    "I am an network automations engineer, software developer, and student (CS BS @ UMass Amherst).
                    Since Summer 2022 I have been working at "
                    a href="https://arista.com" target = "_blank" { "Arista Networks" }
                    " focusing on how to make network automation easier to use and integrate with existing systems.
                    In my free-time I run "
                    a href="https://bgp.tools/as/923" target = "_blank" { "AS923" }
                    " and provide virtual hosting for a few projects."
                }
            }
            div class="pure-u-1 pure-u-md-1-3 hero-img" {
                img class="pure-img" src="/assets/img/hammy.png" alt="Hampton's avatar";
            }
            div class="themecolors" {
                div style="background-color: var(--scheme-blue)" theme="blue" {}
                div style="background-color: var(--scheme-pink)" theme="pink" {}
                div style="background-color: var(--scheme-white); box-sizing: border-box;" theme="white" {}
                div style="background-color: var(--scheme-purple)" theme="purple" {}
                div style="background-color: var(--scheme-red)" theme="red" {}
            }
            script type="text/javascript" {
               (PreEscaped("
                [...document.getElementsByClassName('themecolors')[0].children].forEach((c)=>c.onclick=()=>{
                    theme = c.getAttribute('theme');
                    document.body.setAttribute('colorscheme', theme);
                    document.cookie = `colorscheme=${theme}; expires=${(new Date(Date.now()+ 86400*365*1000)).toUTCString()}; path=/`;
                })
                "))
            }
        }

        div class="pure-g hero section" {
            div class="pure-u-1 pure-u-md-1-2" {
                h3 { "Things I've Made" }
                ul {
                    @for thing in things.clone() {
                        li {
                            (thing.date.format("%Y-%m").to_string()) ": "
                            a target="_blank" href=(thing.link) { (thing.title) }
                            @if let Some(description) = &thing.description {
                                " - "
                                (description)
                            }
                        };
                    }
                    li {
                        a href="/things/" { "See more things" }
                    }
                }
            }
            div class="pure-u-1 pure-u-md-1-2" {
                h3 { "Words I've Written" }
                ul {
                    @for post in words {
                        li {
                            (post.date.format("%Y").to_string()) " "
                            a href=(post.link) { (post.title) }
                            ": "
                            (post.description)
                        };
                    }
                    @if word_count > 5 {
                        li {
                            a href="/posts/" { "See more words" }
                        }
                    }
                }
            }
        }


    };

    base(PageContext {
        title: "Home".to_string(),
        canonical: "/".to_string(),
    }, content, state.clone(), client_state)
}
