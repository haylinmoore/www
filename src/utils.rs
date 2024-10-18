use markdown::{to_html_with_options, CompileOptions, Options};

pub fn md_to_html(md: &str) -> String {
// Live dangerously / trust the author:
    let result = to_html_with_options(md, &Options {
        compile: CompileOptions {
        allow_dangerous_html: true,
        allow_dangerous_protocol: true,

        ..CompileOptions::default()
        },
        ..Options::default()
    });


    result.unwrap()
}