// use std::fs;
use std::error::Error;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

mod trip;
use trip::{LocationPage, TripDayPage};
use trip::cambodia::siem_reap_arrival;

// use xml::name::OwnedName;
// use xml::reader::{EventReader, XmlEvent};

// Need to:
// 1. Read the XML file in
// 2. Parse the XML
// 3. Step through all the keys in the XML, output HTML into a file for each of them
//    a. Some keys are going to require we create a new file
pub struct Config {
    index: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let mut index = "index.aml".to_string();
        if args.len() >= 2 {
            index = args[1].clone();
        }

        Config { index }
    }
}

pub fn run(_: Config) -> Result<(), Box<dyn Error>> {
    // gen(config.index)
    gen()?;
    Ok(())
}

// Prints the header for the site, which includes the following:
// 1. Any HTML boilerplate, e.g. the HTML DOCTYPE and CSS/JS files, etc.
// 2. Navigation for this page.
fn print_header<W: Write>(mut output: W, _: &str) -> Result<W, Box<dyn Error>> {
    output.write_all(b"<!DOCTYPE html>")?;
    output.write_all(b"<html>")?;

    Ok(output)
}

fn print_footer<W: Write>(mut output: W) -> Result<W, Box<dyn Error>> {
    output.write_all(b"</html>")?;

    Ok(output)
}

struct IndexPage {
    links: Vec<String>,
}

impl IndexPage {
    fn new(links: Vec<&str>) -> IndexPage {
        IndexPage {
            links: vec![links.iter().map(|s| s.to_string()).collect()],
        }
    }

    fn to_html(&self, mut output: Box<dyn Write>) -> Result<Box<dyn Write>, Box<dyn Error>> {
        // // There is only one IndexPage, and that is index.html.
        // let file = OpenOptions::new()
        //     .write(true)
        //     .create(true)
        //     .truncate(true)
        //     .open("index.html")?;
        // let file = BufWriter::new(file);

        // let file = print_header(file, "/home")?;

        // // Custom HTML for the index page.

        // print_footer(file)?;

        // Ok(())

        output.write_all(b"<!DOCTYPE html>")?;
        output.write_all(b"<html>")?;

        // Custom HTML for the index page.

        output.write_all(b"</html>");

        Ok(output)
    }
}

struct TripsLocationPage {
    name: String,
    nav: String,
    sections: Vec<TripsLocationSection>,
}

impl TripsLocationPage {
    fn new(name: &str, nav: &str, sections: Vec<TripsLocationSection>) -> TripsLocationPage {
        TripsLocationPage {
            name: name.to_string(),
            nav: nav.to_string(),
            sections: sections,
        }
    }

    fn to_html(&self, mut output: Box<dyn Write>) -> Result<Box<dyn Write>, Box<dyn Error>> {
        // let file = OpenOptions::new()
        //     .write(true)
        //     .create(true)
        //     .truncate(true)
        //     .open(&self.name)?;
        // let file = BufWriter::new(file);

        // let mut file = print_header(file, &self.nav)?;

        // for section in self.sections.iter() {
        //     file = section.to_html(file)?;
        // }

        // print_footer(file)?;

        output.write_all(b"<!DOCTYPE html>")?;
        output.write_all(b"<html>")?;

        Ok(output)
    }
}

struct TripsLocationPageEnd;

impl TripsLocationPageEnd {
    fn new() -> TripsLocationPageEnd {
        TripsLocationPageEnd {}
    }

    fn to_html(&self, mut output: Box<dyn Write>) -> Result<Box<dyn Write>, Box<dyn Error>> {
        output.write_all(b"</html>")?;

        Ok(output)
    }
}

struct TripsLocationSection {
    title: String,
    contents: Vec<TripsLocationContent>,
}

impl TripsLocationSection {
    fn new(title: &str, contents: Vec<TripsLocationContent>) -> TripsLocationSection {
        TripsLocationSection {
            title: title.to_string(),
            contents: contents,
        }
    }

    fn to_html(&self, mut output: Box<dyn Write>) -> Result<Box<dyn Write>, Box<dyn Error>> {
        output.write_all(
            &format!(
                r#"
<div class="page-title">
    <div class="page-title-text-container">
        <div class="page-title-text">{}</div>
    </div>
</div>
"#,
                self.title
            )
            .into_bytes(),
        )?;

        for content in self.contents.iter() {
            output = content.to_html(output)?;
        }

        Ok(output)
    }
}

struct TripsLocationContent {
    texts: Vec<String>,
    images: Vec<String>,
}

impl TripsLocationContent {
    fn new(texts: Vec<&str>, images: Vec<&str>) -> TripsLocationContent {
        TripsLocationContent {
            texts: texts.iter().map(|s| s.to_string()).collect(),
            images: images.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn to_html(&self, mut output: Box<dyn Write>) -> Result<Box<dyn Write>, Box<dyn Error>> {
        for text in self.texts.iter() {
            output.write_all(
                &format!(r#"<div class="page-content-text">{}</div>"#, text).into_bytes(),
            )?;
        }
        if self.images.len() > 0 {
            output.write_all(br#"<div class="page-content-images">"#)?;
            for image in self.images.iter() {
                output.write_all(
                    &format!(
                        r#"<img class="page-content-image" src="/images/{}"/>"#,
                        image
                    )
                    .into_bytes(),
                )?;
            }
            output.write_all(br#"</div>"#)?;
        }

        Ok(output)
    }
}

// Data structure jabbering:
// 1. Can make the page without any sections
//      --> Then we can give a reference to the containing page to each sub-content
// 2. Can make the page have the to_html and know how to render sub-contents
//      --> Then
// 3. Can make the page pass-down a reference to itself when required to methods on sub-contents
//      --> Get the code-smell of passing through references multi-layers
// 4. Is there something natural we can do with navigation context?
// 5. Is there something we can do where all pages get jammed into a common structure & they can
//    reference their location by their navigation?
//      --> Sub-contents won't contain data on their navigation as defined (its not necessary to
//          define their contents); so we would still end up with pass-through of location data
// 6. What if we just dumped everything into a vector, then everything could get its location data
//      --> Vector of to_html-able things (actually can just slap them all in an enum and operate)
//          https://stackoverflow.com/questions/40559931/vector-store-mixed-types-of-data-in-rust
//      --> Page goes in first, section next, contents next, etc. etc.
//      --> If you want to find the page you are on, just keep walking backwards to the page
//      --> If you want to find the section, keep walking backwards to the section

// enum AngieDataContainer {
//     IndexPage(IndexPage),
//     TripPage(TripsLocationPage),
//     TripPageEnd(TripsLocationPageEnd),
//     TripSection(TripsLocationSection),
//     TripContent(TripsLocationContent),
// }

// impl AngieDataContainer {
//     fn output_stuff(&self, output: Box<dyn Write>) -> Result<Box<dyn Write>, Box<dyn Error>> {
//         match self {
//             AngieDataContainer::IndexPage(p) => {
//                 // Make new writer at new page
//                 let file = OpenOptions::new()
//                     .write(true)
//                     .create(true)
//                     .truncate(true)
//                     .open("index.html")?;
//                 let file = Box::new(BufWriter::new(file));
//                 let file = p.to_html(file)?;

//                 Ok(file)
//             },
//             AngieDataContainer::TripPage(p) => {
//                 // Make new writer for new page
//                 let file = OpenOptions::new()
//                     .write(true)
//                     .create(true)
//                     .truncate(true)
//                     .open(&p.name)?;
//                 let file = Box::new(BufWriter::new(file));
//                 let file = p.to_html(file)?;

//                 Ok(file)
//             }
//             AngieDataContainer::TripPageEnd(p) => {
//                 p.to_html(output)
//             }
//             AngieDataContainer::TripSection(p) => {
//                 p.to_html(output)
//             }
//             AngieDataContainer::TripContent(p) => {
//                 p.to_html(output)
//             }
//         }
//     }
// }

// fn gen() -> Result<(), Box<dyn Error>> {
//     let site: Vec<AngieDataContainer> = vec![
//         AngieDataContainer::IndexPage(IndexPage::new(vec![
//             "washington.html",
//             "south_korea.html",
//             "vietnam.html",
//             "cambodia.html",
//             "thailand.html",
//             "england.html",
//             "ireland.html",
//             "scotland.html",
//             "portugal.html",
//             "spain.html",
//             "greece.html",
//             "czech_republic.html",
//         ])),
//         AngieDataContainer::TripPage(TripsLocationPage::new(
//             "siem_reap_arrival",
//             "/home/cambodia/siem_reap_arrival",
//             vec![]
//         )),
//         AngieDataContainer::TripSection(TripsLocationSection::new(
//             "Welcome to Siem Reap",
//             vec![]
//         )),
//         AngieDataContainer::TripContent(TripsLocationContent::new(
//             vec![
//                 "We got up pretty early to catch a flight to Cambodia. It was a short flight and a very small airplane!",
//             ],
//             vec!["cambodia_air.jpg"],
//         ))
//     ];

//     // We need something which implements Write to pass in to kick this entire thing
//     // off, so we just kludge it here by opening "index.html".
//     // ... this doesn't really make sense so we should come up with something better.
//     let output = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .truncate(true)
//         .open("index.html")?;
//     let output = Box::new(BufWriter::new(output));
//     let mut writer: Box<dyn Write> = output;
//     for e in site {
//         writer = e.to_html(writer)?;
//     }

//     // let index = IndexPage::new(vec!["cambodia.html".to_string()]);
//     // index.to_html()?;

//     // let siem_reap_arrival = TripsLocationPage::new(
//     //     "siem_reap_arrival",
//     //     "/home/cambodia/siem_reap_arrival",
//     //     vec![TripsLocationSection::new(
//     //         "Welcome to Siem Reap",
//     //         vec![TripsLocationContent::new(
//     //             vec![
//     //                 "We got up pretty early to catch a flight to Cambodia. It was a short flight and a very small airplane!",
//     //             ],
//     //             vec!["cambodia_air.jpg"],
//     //         )],
//     //     )],
//     // );
//     // siem_reap_arrival.to_html()?;

//     Ok(())
// }

// struct LocationPage {
//     name: String,
// }

// impl LocationPage {
//     fn new(name: &str) -> LocationPage {
//         LocationPage {
//             name: name.to_string(),
//         }
//     }
// }

// struct TripDayPage {
//     name: String,
// }

// impl TripDayPage {
//     fn new(name: &str) -> TripDayPage {
//         TripDayPage {
//             name: name.to_string(),
//         }
//     }
// }

pub enum Page {
    Index,
    Location(LocationPage),
    TripDay(TripDayPage),
}

fn open_for_writing(name: &str) -> Result<BufWriter<File>, Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(name)?;
    Ok(BufWriter::new(file))
}

impl Page {
    fn open(&self) -> Result<BufWriter<File>, Box<dyn Error>> {
        match self {
            Page::Index => open_for_writing("index.html"),
            Page::Location(p) => open_for_writing(&p.name),
            Page::TripDay(p) => open_for_writing(&p.name),
        }
    }

    fn name(&self) -> &str {
        match self {
            Page::Index => "index.html",
            Page::Location(p) => &p.name,
            Page::TripDay(p) => &p.name,
        }
    }

    fn path(&self) -> &str {
        match self {
            Page::TripDay(p) => &p.path,
            _ => ""
        }
    }

    fn resource_path(&self) -> &str {
        match self {
            Page::TripDay(p) => &p.resource_path,
            _ => ""
        }
    }
}

trait Htmlize {
    // Generates HTML for the given component.
    // See https://stackoverflow.com/questions/64942754/how-can-a-trait-object-take-a-trait-with-generic-methods-as-an-argument
    // for why this doesn't take &impl Write and suggestions about how to make it work if it ever matters.
    //fn htmlize(&self, config: &PageConfig, output: &impl Write) -> Result<(), Box<dyn Error>>;
    fn htmlize(&self, config: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>>;
}

// struct TripDayHeader {}

// impl Htmlize for TripDayHeader {
//     fn htmlize(&self, _: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
//         output.write_all(b"<!DOCTYPE html>")?;
//         output.write_all(b"<html>")?;

//         Ok(())
//     }
// }

// struct TripDaySection {
//     title: String,
// }

// impl TripDaySection {
//     fn new(title: &str) -> TripDaySection {
//         TripDaySection {
//             title: title.to_string(),
//         }
//     }
// }

// impl Htmlize for TripDaySection {
//     fn htmlize(&self, _: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
//         output.write_all(
//             &format!(
//                 r#"
// <div class="page-title">
//     <div class="page-title-text-container">
//         <div class="page-title-text">{}</div>
//     </div>
// </div>"#,
//                 self.title
//             )
//             .into_bytes(),
//         )?;

//         Ok(())
//     }
// }

// struct TripDayContent {
//     texts: Vec<String>,
//     images: Vec<String>,
// }

// impl TripDayContent {
//     fn new(texts: Vec<&str>, images: Vec<&str>) -> TripDayContent {
//         TripDayContent {
//             texts: texts.iter().map(|s| s.to_string()).collect(),
//             images: images.iter().map(|s| s.to_string()).collect(),
//         }
//     }
// }

// impl Htmlize for TripDayContent {
//     fn htmlize(&self, _: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
//         for text in self.texts.iter() {
//             output.write_all(
//                 &format!(r#"<div class="page-content-text">{}</div>"#, text).into_bytes(),
//             )?;
//         }
//         if self.images.len() > 0 {
//             output.write_all(br#"<div class="page-content-images">"#)?;
//             for image in self.images.iter() {
//                 output.write_all(
//                     &format!(
//                         r#"<img class="page-content-image" src="/images/{}"/>"#,
//                         image
//                     )
//                     .into_bytes(),
//                 )?;
//             }
//             output.write_all(br#"</div>"#)?;
//         }

//         Ok(())
//     }
// }

// Instead of building the whole site in one mega-structure, lets only put a single page of
// HTML in at a time. Then we know where our Page structure is, its always the first element.
// We don't need to walk backwards up the array to find our page, we can just go to arr[0]

fn index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((Page::Index, vec![]))
}

// // macro: Angie Trip Header
// // Match anything, output shorthand for building the trip header entry for the vector.
// macro_rules! ath {
//     () => {
//         Box::new(TripDayHeader {})
//     }
// }

// // macro: Angie Trip Day Section
// // Match anything, output shorthand for building the TripDaySection entry for the vector.
// macro_rules! atds {
//     ($x:expr) => {
//         Box::new(TripDaySection::new($x))
//     }
// }

// // macro: Angie Trip Day Content
// // Match anything, output shorthand for building TripDayContent entry for the vector.
// macro_rules! atdc {
//     ( $($x:expr),* ) => {
//         ()
//     };
//     ( $(I$x:expr),* )=> {
//         ()
//     }
// }

// // Shorthand functions for reducing verbosity building out the page structure.
// fn hdr() -> Box<TripDayHeader> {
//     Box::new(TripDayHeader {})
// }
// fn sec(title: &str) -> Box<TripDaySection> {
//     Box::new(TripDaySection::new(title))
// }
// fn cnt(text: Vec<&str>, images: Vec<&str>) -> Box<TripDayContent> {
//     Box::new(TripDayContent::new(text, images))
// }

// fn siem_reap_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
//     Ok((
//         Page::TripDay(TripDayPage::new("siem_reap_arrival.html")),
//         vec![
//             //Box::new(TripDayHeader {}),
//             hdr(),
//             //Box::new(TripDaySection::new("Welcome to Siem Reap")),
//             sec("Welcome to Siem Reap"),
//             cnt(vec!["We got up pretty early to catch a flight to Cambodia. It was a short flight and a very small airplane!",],
//                 vec!["cambodia_air.jpg"]),
//             // Box::new(
//             //     TripDayContent::new(
//             //         vec!["We got up pretty early to catch a flight to Cambodia. It was a short flight and a very small airplane!",],
//             //         vec!["cambodia_air.jpg"],
//             //     )  
//             // )
//         ],
//     ))
// }

fn gen() -> Result<(), Box<dyn Error>> {
    let site = vec![index()?, siem_reap_arrival()?];

    for page in site {
        let mut writer = page.0.open()?;
        for element in page.1 {
            element.htmlize(&page.0, &mut writer)?;
        }
    }

    Ok(())
}

// fn gen(target: String) -> Result<(), io::Error> {
//     // // The details of reading a file and error handling are exhaustively handled here.
//     // // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
//     // //
//     // // Having read that, lets just use the one-shot for reading a file.
//     // println!("{}", target);
//     // let contents = fs::read_to_string(target)?;

//     // println!("{}", contents);

//     let mut output = target.clone();
//     output.truncate(output.len() - 3); // Remove "aml" extension
//     output.push_str("html");
//     let output = File::open(output)?;
//     let mut output = BufWriter::new(output);

//     let input = File::open(target)?;
//     let input = BufReader::new(input);

//     let parser = EventReader::new(input);

//     for e in parser {
//         match e {
//             Ok(XmlEvent::StartElement {name, ..}) => {
//                 let to_write = start(&name).into_bytes();
//                 match output.write(&to_write) {
//                     Ok(sz) => {
//                         if sz < to_write.len() {
//                             panic!("Wrote only {} of {} bytes", sz, to_write.len());
//                         }
//                     }
//                     Err(err) => {
//                         panic!("Failed to write {}", err)
//                     }
//                 }
//                 println!("StartElement {}", name);
//                 // Handle element starting
//             }
//             Ok(XmlEvent::EndElement { name }) => {
//                 println!("EndElement {}", name);
//             }
//             Ok(XmlEvent::CData(d)) => {
//                 println!("CData {}", d);
//             }
//             Ok(XmlEvent::Comment(d)) => {
//                 println!("Comment {}", d);
//             }
//             Ok(XmlEvent::Characters(d)) => {
//                 println!("Characters {}", d);
//             }
//             Err(e) => {
//                 println!("Error: {}", e);
//                 break;
//             }
//             _ => {}
//         }
//     }

//     Ok(())
// }

// fn start(tag: &OwnedName) -> String
// {
//     "Yep".to_string()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::path::PathBuf;

//     fn test_aml(target: &str) -> String {
//         let mut path = PathBuf::new();
//         path.push(env!("CARGO_MANIFEST_DIR"));
//         path.push("res");
//         path.push("test");
//         path.push(target);

//         path.to_string_lossy().to_string()
//     }

//     #[test]
//     #[should_panic]
//     fn gen_file_not_found() {
//         gen("does_not_exist.aml").unwrap();
//     }

//     #[test]
//     #[should_panic]
//     fn gen_file_argument_empty() {
//         gen("").unwrap();
//     }

//     #[test]
//     fn gen_simple() {
//         let mut path = PathBuf::new();
//         path.push(env!("CARGO_MANIFEST_DIR"));
//         path.push("res");
//         path.push("test");
//         path.push("simple.aml");

//         gen(&test_aml("simple.aml")).unwrap();
//     }
// }
