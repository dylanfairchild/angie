use std::error::Error;

use crate::trip::{hdr, ftr, sec, lhdr, lftr, lcnt};
use crate::{Htmlize, Page};

pub fn tech() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    let mut res: Vec<(Page, Vec<Box<dyn Htmlize>>)> = Vec::new();

    res.push(index()?);

    Ok(res)
}

fn index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("tech.html", "tech"),
        vec![
            lhdr("Tech"),
            lcnt(vec![
                (
                    "introducing_angie.html",
                    "Introducing Angie",
                    "../images/angie.png",
                    "Our family site has long been overdue for refresh, and this time I decided to learn Rust along the way.
                    I built a single-purpose static site generation tool to generate the site which I call Angie. Is it perfect? No! ... but it has been a good tool for learning!"
                )
            ]),
            lftr()
        ]
    ))
}