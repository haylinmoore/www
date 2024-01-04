use maud::{html, Markup};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::extract::State;
use crate::SiteState;
use super::base;
use rand::Rng;

pub async fn home(State(state): State<Arc<RwLock<SiteState>>>) -> Markup {
    let state = state.read().await;
    let workstation = state.workstation.clone();
    let steam = state.steam.clone();
    let discord = state.discord.clone();
    let cloud = state.cloud.clone();

    let mut rng = rand::thread_rng();
    let img = if rng.gen() {
        "ezri.webp"
    } else {
        "pixel.webp"
    };
    let img = format!("/assets/img/{}", img);

    let content = html! {
        div class="pure-g hero" {
            div class="pure-u-1 pure-u-md-2-3" {
                h1 { "Ezri (they/any)" }
                a href="https://en.pronouns.page/terminology#nonbinary" {
                    img class="flag" src="/assets/img/Nonbinary.webp" alt="Nonbinary flag";
                }
                a href="https://en.pronouns.page/terminology#pansexual" {
                    img class="flag" src="/assets/img/Pansexual.webp" alt="Pansexual flag";
                }
                p { "I am a computer science major that runs a small hosting service with it's own ASN. I currently work in academia as a research assistant." }
                p { "This website is a more casual version of my " a href="https://ezrizhu.com" { "professional website" } "." }
            }
            div class="pure-u-1 pure-u-md-1-3" {
                img class="pure-img" src=(img) alt="Ezri's avatar";
            }
        }

        h3 { "Socials" }

        div class="pure-g hero" {
            div class="pure-u-1 pure-u-md-1-2" {
                p {
                    b { "Fediverse: " }
                    a href="https://sleepless.cafe/ezri" {
                        "@ezri@sleepless.cafe"
                    }
                    br;
                    b { "Matrix: " }
                    a href="https://matrix.to/#/@ezri:envs.net" {
                        "@ezri:envs.net"
                    }
                    br;
                    b { "Twitter: " }
                    a href="https://twitter.com/finnekit" {
                        "@finnekit"
                    }
                    br;
                    b { "BSky: " }
                    a href="https://bsky.app/profile/ezrizhu.com" {
                        "@ezrizhu.com"
                    }
                    br;
                    b { "GitHub: "}
                    a href="https://github.com/ezrizhu" {
                        "@ezrizhu"
                    }
                }
            }
            div class="pure-u-1 pure-u-md-1-2" {
                p {
                    b { "Location: " }
                    "NYC"
                        br;
                    b { "Email: " }
                    a href="mailto:me@ezri.pet" {
                        "me@ezri.pet"
                    }
                    br;
                    b { "pronouns.page: " }
                    a href="https://en.pronouns.page/@finnekit" {
                        "@finnekit"
                    }
                    br;
                    b { "Telegram: " }
                    a href="https://t.me/finnekit" {
                        "@finnekit"
                    }
                    br;
                    b { "Irc: " }
                    "ezri on libera, hackint" 
                }
            }
        }

        div class="pure-g hero" {
            div class="pure-u-1 pure-u-md-1-2" {
                h3 { "Discord" }
                p {
                    "Username: finnekit"
                        br;
                    "Custom status: " (discord.custom_status)
                        br;
                    "Web client: " 
                        @if discord.status_web == "" {
                            "offline"
                        } @else {
                            (discord.status_web)
                        }
                    br;

                    "Mobile client: " 
                        @if discord.status_mobile == "" {
                            "offline"
                        } @else {
                            (discord.status_mobile)
                        }
                    br;

                    "Desktop client: " 
                        @if discord.status_desk == "" {
                            "offline"
                        } @else {
                            (discord.status_desk)
                        }
                }
            }

            div class="pure-u-1 pure-u-md-1-2" {
                h3 { "Steam" }
                p {
                    "Profile: " a href=(steam.profile_url) { (steam.persona_name) }
                    br;
                    "Currently " (steam.persona_state)
                        @if steam.is_gaming {
                            br;
                            p { "Playing " a href=(steam.game_url) { (steam.game_extra_info) } }
                        };
                    br;
                    "Last logoff " (steam.last_logoff)
                }
            }
        }


        h3 { "Workstation status" }
        pre { (workstation) };

        h3 { "EzriCloud" }
        p {
            "AS: 206628"
                br;
            "Status: "
                @if cloud.is_down {
                    "Down since" (cloud.down_since)
                } @else {
                    "All systems operational"
                }
            br;
            "nic-hdl: EZRI-RIPE, ZHUEZ-ARIN"
        }
    };

    base(content, state.clone())
}
