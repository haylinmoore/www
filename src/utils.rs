use comrak::{markdown_to_html, ComrakOptions};

pub fn md_to_html(md: &str) -> String {
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;
    options.render.target_blank = true;
    options.parse.smart = true;
    let md = markdown_to_html(&md, &options);
    md.trim().to_string()
}