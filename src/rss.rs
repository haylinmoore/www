use chrono::Utc;
use xml::writer::{EmitterConfig, XmlEvent};
use axum::{
    body,
    extract::State,
    response::Response,
};
use crate::SiteState;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn get(State(state): State<Arc<RwLock<SiteState>>>) -> Response {
    let mut buf: Vec<u8> = Vec::new();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut buf);

    let state = state.read().await;

    let pubdate = Utc::now().to_rfc2822();
    let link = format!("https://{}/posts/", state.name.domain());
    let feeds: Vec<XmlEvent> = vec![
        XmlEvent::StartDocument{
            version: xml::common::XmlVersion::Version10,
            encoding: Some("UTF-8"),
            standalone: Some(true),
        },
        XmlEvent::start_element("rss").attr("version", "2.0").into(),
        XmlEvent::start_element("channel").into(),

        XmlEvent::start_element("title").into(),
        XmlEvent::characters(&state.name.uppercase_str()).into(),
        XmlEvent::characters("'s blog").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("link").into(),
        XmlEvent::characters(&link).into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("description").into(),
        XmlEvent::characters(&state.name.uppercase_str()).into(),
        XmlEvent::characters("'s writings, rambles, and sometimes tutorials").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("language").into(),
        XmlEvent::characters("en-us").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("pubDate").into(),
        XmlEvent::characters(&pubdate).into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("generator").into(),
        XmlEvent::characters("https://github.com/hamptonmoore/www").into(),
        XmlEvent::end_element().into(),

        XmlEvent::start_element("ttl").into(),
        XmlEvent::characters("1440").into(),
        XmlEvent::end_element().into(),
    ];

    for feed in feeds {
        writer.write(feed).unwrap();
    }

    for post in &state.words {
        let title = post.title.clone();
        let link = format!("https://{}/posts/{}", state.name.domain(), post.slug);
        let date = post.date.to_rfc2822();
        let content = post.body.clone();

        let feeds: Vec<XmlEvent> = vec![
            XmlEvent::start_element("item").into(),

            XmlEvent::start_element("title").into(),
            XmlEvent::characters(&title).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("guid").into(),
            XmlEvent::characters(&link).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("link").into(),
            XmlEvent::characters(&link).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("description").into(),
            XmlEvent::cdata(&content).into(),
            XmlEvent::end_element().into(),

            XmlEvent::start_element("pubDate").into(),
            XmlEvent::characters(&date).into(),
            XmlEvent::end_element().into(),

            XmlEvent::end_element().into(),
        ];
        for feed in feeds {
            writer.write(feed).unwrap();
        }
    }

    let end: XmlEvent = XmlEvent::end_element().into();
    writer.write(end.clone()).unwrap();
    writer.write(end).unwrap();

    Response::builder()
        .header("Content-Type", "application/rss+xml")
        .body(body::Body::from(buf))
        .unwrap()
}