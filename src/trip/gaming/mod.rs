use std::error::Error;

use crate::trip::{hdr, ftr, sec, lhdr, lftr, lcnt};
use crate::{Htmlize, Page};

pub fn gaming() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    let mut res: Vec<(Page, Vec<Box<dyn Htmlize>>)> = Vec::new();

    res.push(index()?);

    Ok(res)
}

fn index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("gaming.html", "gaming"),
        vec![
            lhdr("Gaming"),
            lcnt(vec![
                (
                    "kislev.html",
                    "Defenders of Order: A Kislev Campaign",
                    "../images/TWWH3_Keyart.webp",
                    "Highlights from my first Total War: Warhammer III campaign as the Ice Queen of Kislev."
                )
            ]),
            lftr()
        ]
    ))
}