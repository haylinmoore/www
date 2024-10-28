use crate::{ClientState, SiteState};
use maud::{html, Markup};
pub mod error404;
pub mod home;
pub mod nginx;
pub mod things;
pub mod words;

pub struct PageContext {
    title: String,
    canonical: String,
}

pub fn base(
    context: PageContext,
    content: Markup,
    state: SiteState,
    client: ClientState,
) -> Markup {
    let title = format!("{} | {}", context.title, state.name.uppercase_full_str());

    let commit = if let Ok(commit) = std::env::var("COMMIT") {
        commit.chars().take(8).collect()
    } else {
        String::from("Unknown")
    };

    let build_info = format!(
        "{} {}:{}",
        std::env::var("TIME").unwrap_or_else(|_| String::from("Unknown")),
        std::env::var("REF").unwrap_or_else(|_| String::from("Unknown")),
        commit
    );

    html! {
        (maud::DOCTYPE)
            html lang="en" {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    link rel="stylesheet" href="/assets/css/pure-min.css";
                    link rel="stylesheet" href="/assets/css/main.css";
                    link rel="stylesheet" href="/assets/css/grids-responsive-min.css";
                    link rel="alternate" title=(format!("{}'s Blog", state.name.uppercase_str())) type="application/rss+xml" href="/feed.xml";
                    link rel="canonical" href=(format!("https://hayl.in{}", context.canonical));
                    link rel="icon" type="image/png" href="/assets/img/favicon.png";

                    title { (title) };
                    meta name="author" content=(state.name.uppercase_full_str());

                    meta name="theme-color" content="#19191e";

                    meta property="og:type" content="website";
                    meta property="og:title" content=(title);
                    meta property="og:description" content=(state.name.uppercase_full_str());
                    meta property="og:theme-color" content="#19191e";
                }

                body colorscheme=(client.theme) {
                    div class="main" {
                        (content);
                        div class="footer" {
                            div class="badges" {
                                @for badge in &state.badges {
                                    // check if the link is Some
                                    @if let Some(link) = &badge.link {
                                        a href=(link) target="_blank" {
                                            img loading="lazy" src=(badge.src) alt=(badge.alt);
                                        }
                                    } @else {
                                        img loading="lazy" src=(badge.src) alt=(badge.alt);
                                    }
                                }
                            }

                            p {
                                @if let Some(webring) = &state.webring {
                                    a href=(webring.prev.url) { (webring.prev.name) } (" <- ")
                                    a href=("https://github.com/umaring/umaring") { "UMass Ring" }
                                    (" -> ") a href=(webring.next.url) { (webring.next.name) }
                                    br;
                                }
                                "Source code "
                                a target="_blank" href="https://github.com/haylinmoore/www" { "available here" }
                                ", released under the "
                                a target="_blank" href="https://github.com/haylinmoore/www/blob/main/COPYING" { "GNU AGPLv3 license" }
                                br;
                                "All opinions here are my own and do not reflect the views of my employers or university: future, past, and present."
                                br;
                                (build_info)
                                br;
                            }
                        }
                    }

                }
            }
    }
}

pub fn four04(path: String, state: SiteState, client: ClientState) -> Markup {
    let content = html! {
        div class="pure-g hero section" {
            div class="pure-u-1" {
                h1 { "404" }
                p { "Page not found" }
            }
        }
    };

    base(
        PageContext {
            title: "404".to_string(),
            canonical: format!("/{}", path),
        },
        content,
        state,
        client,
    )
}
