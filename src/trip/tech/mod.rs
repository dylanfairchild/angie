use std::error::Error;

use crate::trip::{ftr, hdr, lcnt, lftr, lhdr, sec};
use crate::{Htmlize, Page};

pub fn tech() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    let mut res: Vec<(Page, Vec<Box<dyn Htmlize>>)> = Vec::new();

    res.push(index()?);
    res.push(introducing_angie()?);

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

fn introducing_angie() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new_override_resource("introducing_angie.html", "tech", "../images"),
        vec![
            hdr("Introducing Angie"),
            sec("A Single-Purpose Static Site Generator in Rust", vec![
                "The mighty fairchild.tech had been collecting dust as a bare-bones HTML listing of images and stories from our trip around the world, and I decided it was time for a refresh. I wanted to have a place to post more types of content and wanted to clean up the look a bit.",
                "Here are some screenshots from our original site.",
                "old_fairchild_tech_screenshot.png",
                "old_fairchild_tech_screenshot_day.png",
                "I had tried this task a few times before and settled on just going bare CSS & HTML instead of diving deep in the Javascript and using a framework like Vue or React which I am used to having at work. I also wanted it to be something as dependency-free as possible and something I developed myself as much as possible.",
                "Before settling on Angie I had tried just writing every new HTML file by hand. I got a few locations into the new site and realized I didn't have the heart for it. Not only was the copy-paste absolutely mind-numbing, the new site was very difficult to modify. If I made a change to how some common component was built I had to go make that tweak in every file. This wasn't going to scale and my days were now full of childcare, so I gave up and the refresh went back into hibernation.",
                "Now, several months later, I have picked back up the torch and decided to pair this project with another long-term goal of mine, learning Rust. Although at the time of this writing the implementation is incredibly naive, poorly organized, and fairly inflexible; I am happy to say I have something working. I have learned a lot about Rust with this simple intro-project, and look forward to improvements to come in the site.",
                "To provide another snapshot into the past, here is what Angie looks like at the time of writing.",
                "angie_fairchild_tech_screenshot.png",
                "angie_fairchild_tech_screenshot_tech.png",
                "angie_fairchild_tech_screenshot_introducing_angie.png",
                "angie_fairchild_tech_screenshot_atw.png",
                "angie_fairchild_tech_screenshot_atw_location.png",
                "angie_fairchild_tech_screenshot_atw_day.png",
            ]),
            ftr(),
        ],
    ))
}
