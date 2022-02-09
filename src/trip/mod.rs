pub mod atw;
pub mod gaming;
pub mod tech;

use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{BufWriter, Write};

use crate::{root, Htmlize, Page};

fn path_str_to_components(path: &str) -> Vec<String> {
    let mut components = Vec::new();

    // Paths that start with a "/" character will get broken up such
    // that the first entry in the return value is an empty string.
    let mut component = "".to_string();

    for c in path.chars() {
        if c == '/' {
            components.push(component.clone());
            component.clear();
        } else {
            component.push(c);
        }
    }

    if component.len() > 0 {
        components.push(component);
    }

    components
}

fn slice_before_match_from_end_ci<'a>(s: &'a str, m: &str) -> &'a str {
    let i = s.len() - m.len();

    //TODO: Can lowercasing a string change the length of it?
    //      Is there a better way of doing string operations like this in Rust?
    // Should not affect this project, but good things to research.
    let l = s.to_lowercase();
    let lm = m.to_lowercase();
    if l.len() > i && &l[i..] == lm {
        return &s[..i];
    }

    s
}

fn htmlize_boilerplate_header_and_navigation(
    page: &Page,
    output: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
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
        &format!(
            r#"
<!DOCTYPE html>
<html>
    <head>
        <title>fairchild.tech</title>
        <link rel="stylesheet" type="text/css" href="{}css/simple.css" />
    </head>
    <body>
        <div class="navbar-container">
            <div class="navbar">fairchild.tech</div>
            <div class="navbar-path">"#,
            root()
        )
        .into_bytes(),
    )?;

    // "First block" - takes care of /home.
    output.write_all(
        &format!(
            r#"
                /<a href="{}index.html">home</a>"#,
            root()
        )
        .into_bytes(),
    )?;
    // "Middle block" - takes care of tricky variable navigation.
    let name = slice_before_match_from_end_ci(&page.name, ".html");
    let components = path_str_to_components(&page.path);
    let mut path_to_component = String::new();
    for component in components.iter() {
        // The directories are structured such that every directory has a
        // file named after it. When we are on that page, we don't want to
        // show the directory name twice in our navigation, so we just skip it.
        if component == name {
            continue;
        }

        path_to_component = path_to_component + component + "/";
        // With the exception of index.html at home, there is always a file named after
        // the folder that it is in which acts as the landing page for that folder.
        let path_to_html = root() + &path_to_component + component + ".html";
        output.write_all(
            &format!(
                r#"
                /<a href="{}">{}</a>"#,
                path_to_html, component
            )
            .into_bytes(),
        )?;
    }
    // "Last block" - takes care of the page we are currently on.
    output.write_all(
        &format!(
            r#"
                /{}"#,
            name
        )
        .into_bytes(),
    )?;

    // Close off the common navigation bar / header HTML.
    output.write_all(
        br#"
            </div>
            <div style="clear: both;"></div>
        </div>"#,
    )?;

    Ok(())
}

pub struct TripIndexPage {
    pub name: String,
    pub path: String,
}

impl TripIndexPage {
    pub fn new(name: &str, path: &str) -> TripIndexPage {
        TripIndexPage {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

pub struct TripIndexHeader {}

impl Htmlize for TripIndexHeader {
    fn htmlize(&self, page: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        htmlize_boilerplate_header_and_navigation(page, output)?;

        output.write_all(
            br#"
        <div class="world-menu">"#,
        )?;

        Ok(())
    }
}

pub struct TripIndexFooter {}

impl Htmlize for TripIndexFooter {
    fn htmlize(&self, page: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        output.write_all(
            br#"
        </div>
    </body>
</html>"#,
        )?;

        Ok(())
    }
}

pub struct TripIndexCard {
    link: String,
    title: String,
    subtitle: String,
    image: String,
}

pub struct TripIndexContent {
    content: Vec<TripIndexCard>,
}

impl TripIndexContent {
    pub fn new(content: Vec<(&str, &str, &str, &str)>) -> TripIndexContent {
        TripIndexContent {
            content: content
                .iter()
                .map(|s| TripIndexCard {
                    link: s.0.to_string(),
                    title: s.1.to_string(),
                    subtitle: s.2.to_string(),
                    image: s.3.to_string(),
                })
                .collect(),
        }
    }
}

impl Htmlize for TripIndexContent {
    fn htmlize(&self, page: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        for c in self.content.iter() {
            let link = root() + &page.path + "/" + &c.link;
            let image = root() + "from-bucket/" + &c.image;

            output.write_all(
                &format!(
                    r#"
        <a class="card-link" href="{}">
            <div class="world-menu-card">
                <img class="world-menu-card-image" src="{}" />
                <div class="world-menu-text-box">
                    <div class="world-menu-title">{}</div>
                    <div class="world-menu-subtitle">{}</div>
                </div>
            </div>
        </a>
                    "#,
                    link, image, &c.title, &c.subtitle
                )
                .into_bytes(),
            )?;
        }

        Ok(())
    }
}

// pub fn atw_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
//     Ok((
//         Page::TripIndex(TripIndexPage::new("atw.html", "atw")),
//         vec![
//             ihdr(),
//             icnt(vec![
//                 (
//                     "washington/washington.html",
//                     "Washington",
//                     "Grandma Fairchild, Elin",
//                     "grandma/Hike_with_grandma.jpg",
//                 ),
//                 ("south_korea/south_korea.html", "South Korea", "Seoul", ""),
//                 (
//                     "vietname/vietnam.html",
//                     "Vietnam",
//                     "Hanoi, Ho Chi Minh City",
//                     "",
//                 ),
//                 (
//                     "cambodia/cambodia.html",
//                     "Cambodia",
//                     "Siem Reap, Phnom Penh",
//                     "",
//                 ),
//                 (
//                     "thailand/thailand.html",
//                     "Thailand",
//                     "Bangkok, Chiang Mai, Chiang Rai, Railay",
//                     "",
//                 ),
//                 (
//                     "england/england.html",
//                     "England",
//                     "London, Seaford, Bath, York, Malham",
//                     "",
//                 ),
//                 ("ireland/ireland.html", "Ireland", "The Entire Island", ""),
//                 (
//                     "scotland/scotland.html",
//                     "Scotland",
//                     "Glasgow, Islay, Oban, Glencoe, Edinburgh",
//                     "",
//                 ),
//                 ("portugal/portugal.html", "Portugal", "Porto, Lisbon", ""),
//                 (
//                     "spain/spain.html",
//                     "Spain",
//                     "Andalucia, Madrid, Barcelona",
//                     "",
//                 ),
//                 (
//                     "greece/greece.html",
//                     "Greece",
//                     "Athens, Hydra, Santorini, Paros",
//                     "",
//                 ),
//                 (
//                     "czech_republic/czech_republic.html",
//                     "Czech Republic",
//                     "Prague",
//                     "",
//                 ),
//             ]),
//             iftr(),
//         ],
//     ))
// }

pub struct LocationPage {
    pub name: String,
    pub path: String,
}

impl LocationPage {
    pub fn new(name: &str, path: &str) -> LocationPage {
        LocationPage {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

pub struct LocationHeader {
    title: String,
}

impl LocationHeader {
    pub fn new(title: &str) -> LocationHeader {
        LocationHeader {
            title: title.to_string(),
        }
    }
}

impl Htmlize for LocationHeader {
    fn htmlize(&self, page: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        htmlize_boilerplate_header_and_navigation(page, output)?;

        output.write_all(
            &format!(
                r#"
        <div class="region-menu">
            <div class="page-title">
                <div class="page-title-text-container">
                    <div class="page-title-text">{}</div>
                </div>
            </div>
            <div class="spacer"></div>
        "#,
                self.title
            )
            .into_bytes(),
        )?;

        Ok(())
    }
}

pub struct LocationFooter {}

impl Htmlize for LocationFooter {
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

struct LocationContentSection {
    link: String,
    title: String,
    image: String,
    text: String,
}

pub struct LocationContent {
    content: Vec<LocationContentSection>,
}

impl LocationContent {
    pub fn new(content: Vec<(&str, &str, &str, &str)>) -> LocationContent {
        LocationContent {
            content: content
                .iter()
                .map(|s| LocationContentSection {
                    link: s.0.to_string(),
                    title: s.1.to_string(),
                    image: s.2.to_string(),
                    text: s.3.to_string(),
                })
                .collect(),
        }
    }

    fn htmlize_image(
        &self,
        _: &Page,
        output: &mut BufWriter<File>,
        c: &LocationContentSection,
    ) -> Result<(), Box<dyn Error>> {
        let image_path = root() + "from-bucket/" + &c.image;
        output.write_all(
            &format!(
                r#"
                <img class="region-menu-card-image" src="{}" />"#,
                image_path
            )
            .into_bytes(),
        )?;
        Ok(())
    }
    fn htmlize_text(
        &self,
        _: &Page,
        output: &mut BufWriter<File>,
        c: &LocationContentSection,
    ) -> Result<(), Box<dyn Error>> {
        output.write_all(
            &format!(
                r#"
                <div class="region-menu-text-box">
                    <div class="region-menu-title">{}</div>
                    <div class="region-menu-text">{}</div>
                </div>"#,
                c.title, c.text
            )
            .into_bytes(),
        )?;
        Ok(())
    }
}

impl Htmlize for LocationContent {
    fn htmlize(&self, page: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        let mut left = true;
        for c in self.content.iter() {
            let link = root() + &page.path + "/" + &c.link;

            output.write_all(
                &format!(
                    r#"
        <a class="card-link" href="{}">
            <div class="region-menu-card">"#,
                    link
                )
                .into_bytes(),
            )?;

            if left {
                self.htmlize_image(page, output, c)?;
                self.htmlize_text(page, output, c)?;
            } else {
                self.htmlize_text(page, output, c)?;
                self.htmlize_image(page, output, c)?;
            }
            left = !left;

            output.write_all(
                br#"
            </div>
        </a>
        <div class="spacer"></div>"#,
            )?;
        }

        Ok(())
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

impl Htmlize for TripDayHeader {
    fn htmlize(&self, page: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        htmlize_boilerplate_header_and_navigation(page, output)?;

        // OK, that was it for the navigation, now we can just close out the simple pieces.
        output.write_all(
            &format!(
                r#"
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

fn ends_with_tag(register: &String, tag: &str) -> bool {
    // Make this check case insensitive, some of the old HTML pages had tags with wrong casing.
    let register = register.to_lowercase();
    let tag = tag.to_lowercase();

    // zip returns None when either iterator returns None, so we need to make sure
    // that we iterate at least over the entire tag.
    // That just requires that the number of characters in the register is
    // greater than or equal to the number of the characters in the tag.
    if register.chars().count() < tag.chars().count() {
        return false;
    }

    for c in register.chars().rev().zip(tag.chars().rev()) {
        if c.0 != c.1 {
            return false;
        }
    }

    true
}

pub fn gen_from_bucket_html(path: &str) -> Result<Vec<Box<dyn Htmlize>>, Box<dyn Error>> {
    const INVALID_TITLE: &str = "INVALID_TITLE";

    #[derive(PartialEq)]
    enum ParseMode {
        Title,
        SecTitle,
        Text,
        Image,
        Searching,
    }

    let mut ret: Vec<Box<dyn Htmlize>> = vec![];

    let mut mode = ParseMode::Searching;
    let mut content = TripDayContentSection::new(INVALID_TITLE, vec![]);
    let file = read_to_string(path)?;
    let mut register = String::new();
    for c in file.chars() {
        // We want to remove any newline or extra space characters from the file.
        // This is just to help us get a consistent look in the generated files.
        if c == '\n'
            || c == '\r'
            || c == '\t'
            || (c == ' ' && register.chars().last().unwrap_or('x') == ' ')
        {
            continue;
        }

        register.push(c);

        match mode {
            ParseMode::Searching => {
                if ends_with_tag(&register, "<h1>") {
                    mode = ParseMode::Title;
                } else if ends_with_tag(&register, "<h3>") {
                    mode = ParseMode::SecTitle;
                    // Push the previous content, if valid.
                    if content.title != INVALID_TITLE {
                        ret.push(Box::new(content));
                    }
                    content = TripDayContentSection::new("", vec![]);
                } else if ends_with_tag(&register, "<p>") {
                    mode = ParseMode::Text;

                    // Handle untitled content.
                    if content.title == INVALID_TITLE {
                        content.title = "".to_string();
                    }
                } else if ends_with_tag(&register, "<img src=\"") {
                    mode = ParseMode::Image;
                    
                    // Handle untitled content.
                    if content.title == INVALID_TITLE {
                        content.title = "".to_string();
                    }
                }

                if mode != ParseMode::Searching {
                    register.clear();
                }
            }
            ParseMode::Title => {
                if ends_with_tag(&register, "</h1>") {
                    ret.push(hdr(slice_before_match_from_end_ci(&register, "</h1>")));
                    register.clear();
                    mode = ParseMode::Searching;
                }
            }
            ParseMode::SecTitle => {
                if ends_with_tag(&register, "</h3>") {
                    content.title = slice_before_match_from_end_ci(&register, "</h3>").to_string();
                    register.clear();
                    mode = ParseMode::Searching;
                }
            }
            ParseMode::Text => {
                if ends_with_tag(&register, "</p>") {
                    content
                        .content
                        .push(slice_before_match_from_end_ci(&register, "</p>").to_string());
                    register.clear();
                    mode = ParseMode::Searching;
                }
            }
            ParseMode::Image => {
                if ends_with_tag(&register, "\">") {
                    content
                        .content
                        .push(slice_before_match_from_end_ci(&register, "\">").to_string());
                    register.clear();
                    mode = ParseMode::Searching;
                }
            }
        }
    }

    // After we finished the file, we will usually have a last valid content and a footer to add.
    if content.title != INVALID_TITLE {
        ret.push(Box::new(content));
    }
    ret.push(ftr());

    Ok(ret)
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
            return Ok(());
        }

        if self.is_image(index) {
            // There is no previous image, we need to open the image collection container.
            if index == 0 || !self.is_image(index - 1) {
                output.write_all(
                    br#"
            <div class="page-content-images">"#,
                )?;
            }

            let path = root() + "from-bucket/" + &page.resource_path + "/";

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

            let path = root() + "from-bucket/" + &page.resource_path + "/";

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
            if ext == "jpg" || ext == "gif" || ext == "png" {
                return true
            }
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
        if self.title != "" {
            output.write_all(
                &format!(
                    r#"
                <div class="page-content-header">{}</div>"#,
                    self.title
                )
                .into_bytes(),
            )?;
        }

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
pub fn lhdr(title: &str) -> Box<LocationHeader> {
    Box::new(LocationHeader::new(title))
}
pub fn lftr() -> Box<LocationFooter> {
    Box::new(LocationFooter {})
}
pub fn lcnt(content: Vec<(&str, &str, &str, &str)>) -> Box<LocationContent> {
    Box::new(LocationContent::new(content))
}
pub fn ihdr() -> Box<TripIndexHeader> {
    Box::new(TripIndexHeader {})
}
pub fn iftr() -> Box<TripIndexFooter> {
    Box::new(TripIndexFooter {})
}
pub fn icnt(content: Vec<(&str, &str, &str, &str)>) -> Box<TripIndexContent> {
    Box::new(TripIndexContent::new(content))
}
