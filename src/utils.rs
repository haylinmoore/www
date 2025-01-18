use anyhow::Error;
use markdown::{to_html_with_options, CompileOptions, Options};

pub fn md_to_html(md: &str) -> String {
    // Live dangerously / trust the author:
    let result = to_html_with_options(
        md,
        &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,

                ..CompileOptions::default()
            },
            ..Options::default()
        },
    );

    result.unwrap()
}

#[derive(Clone)]
pub struct BuildInfo {
    pub time: String,
    pub commit: String,
    pub branch: String,
}

pub fn build_info() -> BuildInfo {
    let commit = if let Ok(commit) = std::env::var("COMMIT") {
        commit.chars().take(8).collect()
    } else {
        String::from("Unknown")
    };

    let time = std::env::var("TIME")
        .map(|time_str| {
            time_str
                .parse::<u64>()
                .map(|timestamp| {
                    use chrono::{DateTime, TimeZone, Utc};
                    let dt: Option<DateTime<Utc>> = Utc.timestamp_opt(timestamp as i64, 0).latest();
                    if dt.is_none() {
                        return None;
                    }
                    Some(dt.unwrap().format("%Y-%m-%dT%H:%M:%SZ").to_string())
                })
                .unwrap_or(Some(time_str)) // If parsing fails, use original string
        })
        .unwrap_or_else(|_| Some(String::from("Unknown")))
        .unwrap();

    BuildInfo {
        time,
        branch: std::env::var("REF").unwrap_or_else(|_| String::from("Unknown")),
        commit,
    }
}
