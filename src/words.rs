use super::utils;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use std::fs;

#[derive(Clone)]
pub enum PostType {
    Post,
    Link,
}

#[derive(Clone)]
pub struct Post {
    pub slug: String,
    pub link: String,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub description: String,
    pub tags: Vec<String>,
    pub r#type: PostType,
    pub body: String,
}

pub fn get(posts: Vec<Post>, slug: &str) -> Option<Post> {
    for post in posts {
        if post.slug == slug {
            return Some(post);
        }
    }
    None
}

pub fn init(dir: &str) -> Vec<Post> {
    let mut posts_list = vec![];

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_file() {
                let filename = path.file_stem().unwrap().to_str().unwrap().to_string();

                // Here we read the raw file to be processed
                let raw = fs::read_to_string(path).unwrap();

                // yaml frontmatter parsing
                let matter = Matter::<YAML>::new();
                let result = matter.parse(&raw);

                let Some(result_map) = result.data.as_ref() else {
                    panic!("Error parsing YAML")
                };
                let Ok(result_map) = result_map.as_hashmap() else {
                    panic!("Error getting hashmap from Pod")
                };

                let title = result_map["title"].as_string().unwrap();
                let description = result_map["description"].as_string().unwrap();

                // see if tags["Tags"] is exists
                let mut tags: Vec<String> = Vec::new();
                if result_map.contains_key("tags") {
                    let taglist = result_map["tags"].as_vec().unwrap();
                    for tag in taglist {
                        tags.push(tag.as_string().unwrap());
                    }
                }

                let date_str = result_map["date"].as_string().unwrap();
                // The date_str will be displayed on the homepage, blogindex, and blog pages.
                // First we parse our text into NaiveDate
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();
                let date = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                // Now we make this a Fixed DateTime with Eastern time
                let timezone_east = FixedOffset::east_opt(8 * 60 * 60).unwrap();
                let date = timezone_east.from_local_datetime(&date).unwrap();

                // If there is a link we don't load body text
                let link = result_map.get("link").map(|s| s.as_string().unwrap());

                if let Some(link) = link {
                    let post = Post {
                        slug: link.clone(),
                        link,
                        title,
                        date,
                        description,
                        tags,
                        r#type: PostType::Link,
                        body: "".to_string(),
                    };
                    posts_list.push(post);
                    continue;
                }

                // the markdown without the frontmatter, parsed to html
                let body = utils::md_to_html(&result.content);
                let slug = filename.replace(".md", "");
                let post = Post {
                    link: format!("/posts/{}/", slug),
                    slug,
                    title,
                    date,
                    description,
                    tags,
                    body,
                    r#type: PostType::Post,
                };
                posts_list.push(post);
            }
        }
    }
    posts_list.sort_by(|a, b| b.date.cmp(&a.date));
    posts_list
}
