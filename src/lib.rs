use std::error::Error;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufWriter, Write};

mod trip;
use trip::atw::atw;

mod index;
use index::index;
// use trip::cambodia::{
//     cambodia, phnom_penh_arrival, phnom_penh_day_2, siem_reap_arrival, siem_reap_day_2,
//     siem_reap_day_3, siem_reap_day_4,
// };
// use trip::{atw_index, LocationPage, TripDayPage, TripIndexPage};

pub struct Config {}

impl Config {
    pub fn new(args: &[String]) -> Config {
        Config {}
    }
}

pub fn run(_: Config) -> Result<(), Box<dyn Error>> {
    gen()?;
    Ok(())
}

// pub enum Page {
//     Index,
//     TripIndex(TripIndexPage),
//     Location(LocationPage),
//     TripDay(TripDayPage),
// }

pub struct Page {
    name: String,
    path: String,
    resource_path: String,
}

impl Page {
    fn new(name: &str, path: &str) -> Page {
        Page {
            name: name.to_string(),
            path: path.to_string(),
            resource_path: String::new()
        }
    }

    fn new_override_resource(name: &str, path: &str, resource_path: &str) -> Page {
        Page {
            name: name.to_string(),
            path: path.to_string(),
            resource_path: resource_path.to_string()
        }
    }

    fn open(&self) -> Result<BufWriter<File>, Box<dyn Error>> {
        create_dir_all(&self.path)?;

        let target = String::new() + &self.path + "/" + &self.name;
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&target)?;
        Ok(BufWriter::new(file))
    }
}

// fn open_for_writing(path: &str, name: &str) -> Result<BufWriter<File>, Box<dyn Error>> {
//     create_dir_all(path)?;

//     let target = String::new() + path + "/" + name;
//     let file = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .truncate(true)
//         .open(&target)?;
//     Ok(BufWriter::new(file))
// }

//TODO:
// For running via cargo run and debugging on the local filesystem, this is fine.
// When we need to put this into production the root should be changed.
pub fn root() -> String {
    std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/"
}

// impl Page {
//     fn open(&self) -> Result<BufWriter<File>, Box<dyn Error>> {
//         match self {
//             Page::Index => open_for_writing(".", "index.html"),
//             Page::Location(p) => open_for_writing(&p.path, &p.name),
//             Page::TripDay(p) => open_for_writing(&p.path, &p.name),
//             Page::TripIndex(p) => open_for_writing(&p.path, &p.name)
//         }
//     }

//     fn name(&self) -> &str {
//         match self {
//             Page::Index => "index.html",
//             Page::TripIndex(p) => &p.name,
//             Page::Location(p) => &p.name,
//             Page::TripDay(p) => &p.name,
//         }
//     }

//     fn path(&self) -> &str {
//         match self {
//             Page::TripDay(p) => &p.path,
//             Page::Location(p) => &p.path,
//             Page::TripIndex(p) => &p.path,
//             _ => "",
//         }
//     }

//     fn resource_path(&self) -> &str {
//         match self {
//             Page::TripDay(p) => &p.resource_path,
//             _ => "",
//         }
//     }
// }

trait Htmlize {
    // Generates HTML for the given component.
    // See https://stackoverflow.com/questions/64942754/how-can-a-trait-object-take-a-trait-with-generic-methods-as-an-argument
    // for why this doesn't take &impl Write and suggestions about how to make it work if it ever matters.
    //fn htmlize(&self, config: &PageConfig, output: &impl Write) -> Result<(), Box<dyn Error>>;
    fn htmlize(&self, config: &Page, output: &mut BufWriter<File>) -> Result<(), Box<dyn Error>>;
}

fn gen() -> Result<(), Box<dyn Error>> {
    let mut site = vec![
        index()?,
    ];
    site.append(&mut atw()?);

    for page in site {
        let mut writer = match page.0.open()  {
            Ok(v) => v,
            Err(e) => {
                println!("Failure opening page: {}, path: {}", page.0.name, page.0.path);
                return Err(e);
            }
        };

        for element in page.1 {
            element.htmlize(&page.0, &mut writer)?;
        }
    }

    Ok(())
}
