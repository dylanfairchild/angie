use std::error::Error;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufWriter, Write};

mod trip;
use trip::cambodia::{
    cambodia, phnom_penh_arrival, phnom_penh_day_2, siem_reap_arrival, siem_reap_day_2,
    siem_reap_day_3, siem_reap_day_4,
};
use trip::{LocationPage, TripDayPage};

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

pub enum Page {
    Index,
    Location(LocationPage),
    TripDay(TripDayPage),
}

fn open_for_writing(path: &str, name: &str) -> Result<BufWriter<File>, Box<dyn Error>> {
    create_dir_all(path)?;

    let target = String::new() + path + "/" + name;
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&target)?;
    Ok(BufWriter::new(file))
}

//TODO:
// For running via cargo run and debugging on the local filesystem, this is fine.
// When we need to put this into production the root should be changed.
pub fn root() -> String {
    std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/"
}

impl Page {
    fn open(&self) -> Result<BufWriter<File>, Box<dyn Error>> {
        match self {
            Page::Index => open_for_writing(".", "index.html"),
            Page::Location(p) => open_for_writing(&p.path, &p.name),
            Page::TripDay(p) => open_for_writing(&p.path, &p.name),
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
            Page::Location(p) => &p.path,
            _ => "",
        }
    }

    fn resource_path(&self) -> &str {
        match self {
            Page::TripDay(p) => &p.resource_path,
            _ => "",
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

fn index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((Page::Index, vec![]))
}

fn gen() -> Result<(), Box<dyn Error>> {
    let site = vec![
        index()?,
        cambodia()?,
        siem_reap_arrival()?,
        siem_reap_day_2()?,
        siem_reap_day_3()?,
        siem_reap_day_4()?,
        phnom_penh_arrival()?,
        phnom_penh_day_2()?,
    ];

    for page in site {
        let mut writer = page.0.open()?;
        for element in page.1 {
            element.htmlize(&page.0, &mut writer)?;
        }
    }

    Ok(())
}
