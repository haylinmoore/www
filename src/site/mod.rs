use crate::{ClientState, SiteState};
use maud::{html, Markup};
pub mod home;
pub mod things;
pub mod words;

pub fn base(title: String, content: Markup, _state: SiteState, client: ClientState) -> Markup {
    let description = "Hampton Moore";
    let title = format!("{} | Hampton Moore", title);

    let commit = if let Ok(commit) = std::env::var("COMMIT") {
        commit[..8].to_string()
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

                    title { (title) };
                    meta name="description" content=(description);
                    meta name="author" content="Hampton Moore";

                    link rel="manifest" href="/assets/favicon/site.webmanifest";

                    meta name="theme-color" content="#19191e";

                    meta property="og:type" content="website";
                    meta property="og:title" content=(title);
                    meta property="og:description" content=(description);
                    meta property="og:theme-color" content="#19191e";
                }

                body colorscheme=(client.theme) {
                    div class="main" {
                        (content);
                        div class="footer" {
                            div class="badges" {
                                a target="_blank" href="https://social.treehouse.systems/@hammy" {
                                    img src="/assets/img/badges/mastodon.gif" alt="mastodon";
                                }

                                a target="_blank" href="https://github.com/hamptonmoore" {
                                    img src="/assets/img/badges/github.png" alt="github";
                                }
                                img src="/assets/img/badges/penguins.gif" alt="Powered by Penguins";
                                img src="/assets/img/badges/debian.gif" alt="Powered by Debian";
                                // a target="_blank" href="https://infernocomms.com" {
                                //     img src="/assets/img/badges/infernocomms.png" alt="Inferno Communications";
                                // }
                                a target="_blank" href="https://xenyth.net/" {
                                    img src="/assets/img/badges/xenyth.png" alt="xenyth cloud";
                                }
                                a target="_blank" href="https://umass.edu" {
                                    img src="/assets/img/badges/umass.gif" alt="umass";
                                }
                                img src="/assets/img/badges/hammy.gif" alt="hammy";
                                a target="_blank" href="https://ezrizhu.com/" {
                                    img src="/assets/img/badges/ezri.png" alt="Ezri";
                                }
                                img src="/assets/img/badges/yoshi-egg-crack.gif" alt="Yoshi!";
                                // img src="/assets/img/badges/gothtml.png" alt="gothtml";
                                a target="_blank" href="https://arc.net/gift/73b9fff4" {
                                    img src="/assets/img/badges/arc.gif" alt="arc";
                                }
                                a target="_blank" href="https://open.spotify.com/playlist/0tEjCoXGaAOH45dTTVvwWl" {
                                    img src="/assets/img/badges/tf.gif" alt="trans-fem hyperpop";
                                }
                                a target="_blank" href="https://code.visualstudio.com" {
                                    img src="/assets/img/badges/vscode.gif" alt="vscode";
                                }
                                img src="/assets/img/badges/runrust.png" alt="Rust!";
                                a target="_blank" href="https://yesterweb.org/no-to-web3/" {
                                    img src="/assets/img/badges/roly-saynotoweb3.gif" alt="say no to web3";
                                }
                                a target="_blank" href="http://jigsaw.w3.org/css-validator/check/referer" {
                                    img src="/assets/img/badges/vcss-blue.gif" alt="Valid CSS!";
                                }
                            }

                            p {
                                // "Auto refreshed: " (last_updated)
                                // br;
                                "Source code "
                                a target="_blank" href="https://github.com/hamptonmoore/www" { "available here" }
                                ", released under the "
                                a target="_blank" href="https://github.com/hamptonmoore/www/blob/main/COPYING" { "GNU AGPLv3 license" }
                                br;
                                "All opinions here are my own and do not reflect the views of my employers or university: future, past, and present."
                                br;
                                (build_info)
                            }
                        }
                    }

                }
            }
    }
}
