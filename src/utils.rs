use comrak::{markdown_to_html, ComrakOptions};
use kuchiki::traits::*;
use std::fs::{self,File};
use std::io;
use std::io::prelude::*;


pub fn md_to_html(md: &str) -> String {
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;
    options.parse.smart = true;
    let md = markdown_to_html(&md, &options);
    let md = add_target_blank_to_links(md);
    md.trim().to_string()
}

pub fn add_target_blank_to_links(html: String) -> String {
    // Parse the HTML document
    let document = kuchiki::parse_html().one(html);

    // Get all `a` elements in the document
    let a_elements = document.select("a").unwrap();
    
    for a in a_elements {
        let mut attrs = a.attributes.borrow_mut();
        // Add the target "_blank" attribute
        attrs.insert("target", "_blank".to_string());
    }
    
    // Serialize the modified document back to a string
    let body = document.select_first("body").unwrap();
    let mut output = vec![];
    for child in body.as_node().children() {
        child.serialize(&mut output).unwrap();
    }
    String::from_utf8(output).unwrap()
}
