use maud::{html, Markup};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::extract::State;
use crate::SiteState;
use crate::words::Post;
use super::base;

pub async fn home(State(state): State<Arc<RwLock<SiteState>>>) -> Markup {
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
        div class="pure-g hero section" {
            div class="pure-u-1 pure-u-md-2-3 hero-text" {
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
        }

        div class="pure-g hero section" {
            div class="pure-u-1 pure-u-md-1-2" {
                h3 { "Things I've Made" }
                ul {
                    @for thing in things.clone() {
                        li { 
                            (thing.date.format("%Y-%m").to_string()) ": "
                            a href=(thing.link) { (thing.title) }
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

    base("Home".to_owned(), content, state.clone())
}
