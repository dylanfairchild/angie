use std::error::Error;

use crate::trip::{hdr, ftr, sec, lhdr, lftr, lcnt};
use crate::{Htmlize, Page};

pub fn gaming() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    let mut res: Vec<(Page, Vec<Box<dyn Htmlize>>)> = Vec::new();

    res.push(index()?);
    res.push(kislev()?);

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

fn kislev() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new_override_resource("kislev.html", "gaming", "../images"),
        vec![
            hdr("Defenders of Order: A Kislev Campaign"),
            sec("Goals", vec![
                "For my first campaign in Total War: Warhammer III I decided to play the \"good guys\" and hold the forces of Chaos at bay. My goals in this campaign are to unify Kislev and save the world!"
            ]),
            sec("Turn 1 through XX", vec![
                "Placeholder for gameplay recap."
            ]),
            ftr()
        ]
    ))
}