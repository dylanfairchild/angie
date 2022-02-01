pub mod cambodia;

use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::{Htmlize, Page};

pub struct LocationPage {
    pub name: String,
}

impl LocationPage {
    pub fn new(name: &str) -> LocationPage {
        LocationPage {
            name: name.to_string(),
        }
    }
}

pub struct TripDayPage {
    pub name: String,
    pub path: String,
    pub resource_path: String,
}

impl TripDayPage {
    pub fn new(name: &str, path: &str, resource_path: &str) -> TripDayPage {
        TripDayPage {
            name: name.to_string(),
            path: path.to_string(),
            resource_path: resource_path.to_string(),
        }
    }
}

pub struct TripDayHeader {
    title: String,
}

impl TripDayHeader {
    fn new(title: &str) -> TripDayHeader {
        TripDayHeader {
            title: title.to_string(),
        }
    }
}

fn path_str_to_components(path: &str) -> Vec<String> {
    let mut components = Vec::new();

    // Paths that start with a "/" character will get broken up such
    // that the first entry in the return value is an empty string.
    let mut component = "".to_string();

    for c in path.chars() {
        if c == '/' {
            components.push(component.clone());
            component.clear();
        }
        else {
            component.push(c);
        }
    }

    if component.len() > 0 {
        components.push(component);
    }

    components
}

fn path_to_component_html(components: &Vec<String>, index: usize) -> String {
    for i in 0..index {

    }

    "".to_string()
}

fn slice_before_match_from_end_ci<'a>(s:  &'a str, m: &str) -> &'a str {
    let mut i = s.len() - m.len();

    //TODO: Can lowercasing a string change the length of it?
    //      Is there a better way of doing string operations like this in Rust?
    // Should not affect this project, but good things to research.
    let l = s.to_lowercase();
    let lm = m.to_lowercase();
    if l.len() > i && &l[..l.len()-i] == lm{
        return &s[s.len()-i..];
    }

    s
}

impl Htmlize for TripDayHeader {
    fn htmlize(&self, page: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        // The following code outputs the navigation tree at the top of the page.
        // This is a bit tricky, so here is what it is intended to do:
        // Page given is for siem_reap_arrival.html
        // Path given on the page is... atw/cambodia
        // Navigation should look like... /home/atw/cambodia/siem_reap_arrival, all links except the last
        // Name from page is... siem_reap_arrival.html
        // Location on the server is... /atw/cambodia/siem_reap_arrival.html
        //
        // Components array from path_str_to_components will look like:
        // 0 - atw
        // 1 - cambodia
        // We want to output... /home/atw/cambodia/siem_reap_arrival
        // So, the first block takes care of /home, which is always a static link for any given TripDayHeader.
        // The middle block takes care of /atw/cambodia, which we get from our TripDayPage path.
        // The last block takes care of /siem_reap_arrival, which we get from our TripDayPage name.

        // This beginning is just the simple boilerplate to set up the page and the navigation.
        output.write_all(
                br#"
<!DOCTYPE html>
<html>
    <head>
        <title>fairchild.tech</title>
        <link rel="stylesheet" type="text/css" href="css/simple.css" />
    </head>
    <body>
        <div class="navbar-container">
            <div class="navbar">fairchild.tech</div>
            <div class="navbar-path">"#)?;


        // "First block" - takes care of /home.
        output.write_all(br#"
                /<a href="index.html">home</a>"#)?;
        // "Middle block" - takes care of tricky variable navigation.
        let components = path_str_to_components(page.path());
        let mut path_to_component = String::new();
        for component in components.iter() {
            path_to_component = path_to_component + component + "/";
            // With the exception of index.html at home, there is always a file named after
            // the folder that it is in which acts as the landing page for that folder.
            let path_to_html = String::new() + &path_to_component + component + ".html";
            output.write_all(&format!(r#"
                /<a href="{}">{}</a>"#, path_to_html, component).into_bytes())?;
        }
        // "Last block" - takes care of the page we are currently on.
        // This needs to have the ".html" removed from the name since we specify
        // page names with the extension on creation.
        let name = slice_before_match_from_end_ci(page.name(), ".html");
        output.write_all(&format!(r#"
                /{}"#, name).into_bytes())?;

        // OK, that was it for the navigation, now we can just close out the simple pieces.
        output.write_all(
            &format!(
                r#"
            </div>
            <div style="clear: both;"></div>
        </div>
        <div class="page-container">
            <div class="page-title">
                <div class="page-title-text-container">
                    <div class="page-title-text">{}</div>
                </div>
            </div>"#,
                self.title
            )
            .into_bytes(),
        )?;

        Ok(())
    }
}

pub struct TripDayFooter {}

impl Htmlize for TripDayFooter {
    fn htmlize(&self, _: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        output.write_all(
            br#"
        </div>
    </body>
</html>"#,
        )?;

        Ok(())
    }
}

pub struct TripDayContentSection {
    title: String,
    content: Vec<String>,
}

impl TripDayContentSection {
    fn new(title: &str, content: Vec<&str>) -> TripDayContentSection {
        TripDayContentSection {
            title: title.to_string(),
            content: content.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn htmlize_content(
        &self,
        page: &Page,
        output: &mut BufWriter<File>,
        index: usize,
    ) -> Result<(), Box<dyn Error>> {
        if index >= self.content.len() {
            return Ok(())
        }

        if self.is_image(index) {
            // There is no previous image, we need to open the image collection container.
            if index == 0 || !self.is_image(index - 1) {
                output.write_all(
                    br#"
            <div class="page-content-images">"#,
                )?;
            }

            let path = String::new() + "from-bucket/" + page.resource_path() + "/";

            // Output the image content.
            output.write_all(
                &format!(
                    r#"
                <img class="page-content-image" src="{}{}"/>"#,
                    path, &self.content[index]
                )
                .into_bytes(),
            )?;

            // There is no future image, we need to close the image collection container.
            if index == (self.content.len() - 1) || !self.is_image(index + 1) {
                output.write_all(
                    br#"
            </div>"#,
                )?;
            }
        } else if self.is_video(index) {
            // Output video content.

            let path = String::new() + "from-bucket/" + page.resource_path() + "/";

            output.write_all(
                &format!(
                    r#"
            <video controls>
                    <source src="{}{}" type="video/mp4">
                    The video is not supported by your browser.
            </video>"#,
                    path, &self.content[index]
                )
                .into_bytes(),
            )?;
        } else {
            // Output text content.
            output.write_all(
                &format!(
                    r#"
            <div class="page-content-text">
                {}
            </div>"#,
                    &self.content[index]
                )
                .into_bytes(),
            )?;
        }

        self.htmlize_content(page, output, index + 1)
    }

    fn is_image(&self, index: usize) -> bool {
        if self.content.len() == 0 {
            return false;
        }
        if index >= self.content.len() {
            return false;
        }

        let content = self.content[index].clone();

        if content.len() >= 3 {
            let ext: String = content.chars().skip(content.len() - 3).take(3).collect();
            let ext = ext.to_lowercase();
            return ext == "jpg" || ext == "gif";
        }

        if content.len() >= 4 {
            let ext: String = content.chars().skip(content.len() - 4).take(4).collect();
            let ext = ext.to_lowercase();
            return ext == "jpeg";
        }

        false
    }

    fn is_video(&self, index: usize) -> bool {
        if self.content.len() == 0 {
            return false;
        }
        if index >= self.content.len() {
            return false;
        }

        let content = self.content[index].clone();

        if content.len() >= 3 {
            let ext: String = content.chars().skip(content.len() - 3).take(3).collect();
            let ext = ext.to_lowercase();
            return ext == "mp4";
        }

        false
    }
}

impl Htmlize for TripDayContentSection {
    fn htmlize(&self, page: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        // Title
        output.write_all(
            &format!(
                r#"
            <div class="page-content-header">{}</div>"#,
                self.title
            )
            .into_bytes(),
        )?;

        self.htmlize_content(page, output, 0)?;

        Ok(())
    }
}

// Shorthand functions for reducing verbosity building out the page structure.
pub fn hdr(title: &str) -> Box<TripDayHeader> {
    Box::new(TripDayHeader::new(title))
}
pub fn ftr() -> Box<TripDayFooter> {
    Box::new(TripDayFooter {})
}
pub fn sec(title: &str, content: Vec<&str>) -> Box<TripDayContentSection> {
    Box::new(TripDayContentSection::new(title, content))
}
