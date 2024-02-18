use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, Plugins};

pub fn md_to_html(md: &str) -> String {
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;
    options.render.target_blank = true;
    options.parse.smart = true;

    let mut plugins = Plugins::default();
    let adapter = SyntectAdapterBuilder::new()
        //.theme("base16-ocean.dark")
        .css()
        .build();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);
    markdown_to_html_with_plugins(md, &options, &plugins)
        .trim()
        .to_string()
}
