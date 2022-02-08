use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::{root, Htmlize, Page};

struct IndexPageLink {
    title: String,
    message: String,
    image: String,
    link: String,
}

struct IndexPage {
    links: Vec<IndexPageLink>,
}

impl IndexPage {
    pub fn new(links: Vec<(&str, &str, &str, &str)>) -> IndexPage {
        IndexPage {
            links: links
                .iter()
                .map(|s| IndexPageLink {
                    title: s.0.to_string(),
                    message: s.1.to_string(),
                    image: s.2.to_string(),
                    link: s.3.to_string(),
                })
                .collect(),
        }
    }
}

impl Htmlize for IndexPage {
    fn htmlize(&self, page: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        output.write_all(&format!(r#"
<!DOCTYPE html>
<html>
    <head>
        <title>fairchild.tech</title>
        <link rel="stylesheet" type="text/css" href="{}css/simple.css" />
    </head>
    <body>
        <div class="navbar-container">
            <div class="navbar">fairchild.tech</div>
            <div class="navbar-path">
                /home
            </div>
            <div style="clear: both;"></div>
        </div>
        <div class="index-welcome-message">
            <img class="index-welcome-message-image" src="{}images/welcome_image.jpg" />
            <div class="index-welcome-message-text-title">
                Welcome
            </div>
            <div class="index-welcome-message-text">
                This is a spot for Dylan and the rest of the Fairchild family to share their thoughts, stories, pictures, and what they are working on.
                <br />
                Please check back often to see what is new!
            </div>
        </div>
        <div class="spacer"></div>
        <div class="index-menu">"#, root(), root()).into_bytes())?;

        for link in self.links.iter() {
            output.write_all(
                &format!(
                    r#"
            <a class="card-link" href="{}{}">
                <div class="index-menu-card">
                    <img class="index-menu-card-image" src="{}{}" />
                    <div class="index-menu-text-box">
                        <div class="index-menu-title">{}</div>
                        <div class="index-menu-subtitle">{}</div>
                    </div>
                </div>
            </a>    
            "#,
                    root(),
                    link.link,
                    root(),
                    link.image,
                    link.title,
                    link.message
                )
                .into_bytes(),
            )?;
        }

        output.write_all(
            br#"
        </div>
    </body>
</html>"#,
        )?;

        Ok(())
    }
}

pub fn index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("index.html", "."),
        vec![Box::new(IndexPage::new(vec![
            (
                "Tech",
                "A place to talk about the technology we are using.",
                "images/code.png",
                "tech/tech.html",
            ),
            (
                "Gaming",
                "Reviews, stories, and other throughts on games.",
                "images/TWWH3_Keyart.webp",
                "gaming/gaming.html",
            ),
            (
                "Travel",
                "Pictures and stories from our adventures.",
                "from-bucket/railay_day_2/walking_to_climb_spot.jpg",
                "atw/atw.html",
            ),
        ]))],
    ))
}
