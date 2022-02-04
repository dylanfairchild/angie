use std::error::Error;

use crate::trip::{gen_from_bucket_html, icnt, iftr, ihdr, lcnt, lftr, lhdr};
use crate::{Htmlize, Page};

// Aggregation function for the entire atw page family.
pub fn atw() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    let mut res: Vec<(Page, Vec<Box<dyn Htmlize>>)> = Vec::new();

    res.push(index()?);

    Ok(res)
}

fn index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("atw.html", "atw"),
        vec![
            ihdr(),
            icnt(vec![
                (
                    "washington/washington.html",
                    "Washington",
                    "Grandma Fairchild, Elin",
                    "grandma/Hike_with_grandma.jpg",
                ),
                ("south_korea/south_korea.html", "South Korea", "Seoul", ""),
                (
                    "vietname/vietnam.html",
                    "Vietnam",
                    "Hanoi, Ho Chi Minh City",
                    "",
                ),
                (
                    "cambodia/cambodia.html",
                    "Cambodia",
                    "Siem Reap, Phnom Penh",
                    "",
                ),
                (
                    "thailand/thailand.html",
                    "Thailand",
                    "Bangkok, Chiang Mai, Chiang Rai, Railay",
                    "",
                ),
                (
                    "england/england.html",
                    "England",
                    "London, Seaford, Bath, York, Malham",
                    "",
                ),
                ("ireland/ireland.html", "Ireland", "The Entire Island", ""),
                (
                    "scotland/scotland.html",
                    "Scotland",
                    "Glasgow, Islay, Oban, Glencoe, Edinburgh",
                    "",
                ),
                ("portugal/portugal.html", "Portugal", "Porto, Lisbon", ""),
                (
                    "spain/spain.html",
                    "Spain",
                    "Andalucia, Madrid, Barcelona",
                    "",
                ),
                (
                    "greece/greece.html",
                    "Greece",
                    "Athens, Hydra, Santorini, Paros",
                    "",
                ),
                (
                    "czech_republic/czech_republic.html",
                    "Czech Republic",
                    "Prague",
                    "",
                ),
            ]),
            iftr(),
        ],
    ))
}

fn washington() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok((vec![washington_index()?, grandma()?, elin()?]))
}
fn south_korea() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok((vec![south_korea_index()]))
}
fn vietnam() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok((vec![vietnam_index()]))
}
fn cambodia() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok((vec![cambodia_index()]))
}
fn thailand() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok((vec![thailand_index()]))
}
fn england() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok((vec![england_index()]))
}
fn ireland() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok((vec![ireland_index()]))
}
fn scotland() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok((vec![scotland_index()]))
}
fn portugal() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok((vec![portugal_index()]))
}
fn spain() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok((vec![spain_index()]))
}
fn greece() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok((vec![greece_index()]))
}
fn czech_republic() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok((vec![czech_republic_index()]))
}

// Washington
fn washington_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("washington.html", "atw/washington"),
        vec![
            lhdr("Washington"),
            lcnt(vec![
                (
                    "grandma.html",
                    "Visiting Grandma Fairchild",
                    "",
                    "We kicked off our trip with a visit to Grandma Fairchild and the rest of the Washington Fairchilds."
                ),
                (
                    "elin.html",
                    "Elin & the Fremont Fair",
                    "",
                    "Before we left the USA we met up with Elin to visit the one-and-only Fremont Fair.",
                )
            ]),
            lftr()
        ]
    ))
}
fn grandma() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/grandma/grandma.html")?;
    Ok((
        Page::new_override_resource("grandma.html", "atw/washington", "grandma"),
        content,
    ))
}
fn elin() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/fremont/fremont.html")?;
    Ok((
        Page::new_override_resource("elin.html", "atw/washington", "elin"),
        content,
    ))
}

// South Korea
fn south_korea_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("south_korea.html", "atw/south_korea"),
        vec![
            lhdr("South Korea"),
            lcnt(vec![
                ("seoul_arrival.html", "Seoul Arrival", "", ""),
                ("seoul_day_2.html", "Seoul Day 2", "", ""),
                ("seoul_day_3.html", "Seoul Day 3", "", ""),
            ]),
            lftr(),
        ],
    ))
}
fn seoul_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/seoul_arrival/seoul.html")?;
    Ok((
        Page::new_override_resource("seoul_arrival.html", "atw/south_korea", "seoul_arrival"),
        content,
    ))
}
fn seoul_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/seoul_day_2/seoul.html")?;
    Ok((
        Page::new_override_resource("seoul_day_2.html", "atw/south_korea", "seoul_day_2"),
        content,
    ))
}
fn seoul_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/seoul_day_3/seoul.html")?;
    Ok((
        Page::new_override_resource("seoul_day_3.html", "atw/south_korea", "seoul_day_3"),
        content,
    ))
}

// Vietnam
fn vietnam_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("vietnam.html", "atw/vietnam"),
        vec![
            lhdr("Vietnam"),
            lcnt(vec![
                ("hanoi_arrival.html", "Hanoi Arrival", "", ""),
                ("hanoi_day_2.html", "Hanoi Day 2", "", ""),
                ("ho_chi_minh_city_arrival.html", "Ho Chi Minh City Arrival", "", ""),
                ("ho_chi_minh_city_day_2.html", "Ho Chi Minh City Day 2", "", "")
            ]),
            lftr()
        ]
    ))
}
fn hanoi_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/hanoi_arrival/hanoi.html")?;
    Ok((
        Page::new_override_resource("hanoi_arrival.html", "atw/vietnam", "hanoi_arrival"),
        content,
    ))
}
fn hanoi_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/hanoi_day_2/hanoi.html")?;
    Ok((
        Page::new_override_resource("hanoi_day_2.html", "atw/vietnam", "hanoi_day_2"),
        content,
    ))
}
fn ho_chi_minh_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ho_chi_minh_arrival/hcm.html")?;
    Ok((
        Page::new_override_resource("ho_chi_minh_arrival.html", "atw/vietnam", "ho_chi_minh_arrival"),
        content,
    ))
}
fn ho_chi_minh_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ho_chi_minh_day_2/hcm.html")?;
    Ok((
        Page::new_override_resource("ho_chi_minh_day_2.html", "atw/vietnam", "ho_chi_minh_day_2"),
        content,
    ))
}


// Cambodia
fn cambodia_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("cambodia.html", "atw/cambodia"),
        vec![
            lhdr("Cambodia"),
            lcnt(vec![
                (
                    "siem_reap_arrival.html",
                    "Siem Reap Arrival",
                    "siem_reap_arrival/night_lotus.jpg",
                    "After a few days in Ho Chi Minh City we were ready to visit some
                    of the more rural areas of South East Asia. Up first on our list,
                    the Angkor temple complex in Siem Reap. On our travel day we did
                    not visit the temple complex, but we did explore the local area
                    with a guided bicycle tour from our hostel through the lotus
                    fields and to nearby Buddhist temple.",
                ),
                (
                    "siem_reap_day_2.html",
                    "Siem Reap Day 2",
                    "siem_reap_day_2/sunrise_ish.jpg",
                    "Our first full day in Siem Reap we did the morning Ankor tour
                    through our hostel. This hit the main attractions like Angkor Wat,
                    Bayon, Ta Prohm, and Banteay Kdei. It really feels like you are
                    Indiana Jones exploring these temples, absolutely stunning. We
                    finished off our day with a double-dinner of wood-fired fresh
                    pineapple pizza and a Khmer curry on Pub Street.",
                ),
                (
                    "siem_reap_day_3.html",
                    "Siem Reap Day 3",
                    "siem_reap_day_3/tree_preah.jpg",
                    "After getting a taste of the adventure with the morning tour, we
                    decided to book the afternoon tour and hit most of the rest of the
                    temples. We also had a nice night on pub street with the $0.50
                    drafts and live music!",
                ),
                (
                    "siem_reap_day_4.html",
                    "Siem Reap Day 4",
                    "siem_reap_day_4/restaurant_steph_drinks.jpg",
                    "After a few exhausting days exploring the temple complex we
                    decided to take a day off. Today we got the scattered downpours
                    typical of the region, so we explored around town for some food
                    and otherwise took the opportunity to work on our posts and books.",
                ),
                (
                    "phnom_penh_arrival.html",
                    "Phnom Penh Arrival",
                    "phnom_penh_arrival/noodle_making.jpg",
                    "Today was our travel day to Phnom Penh, and our first non-plane
                    travel day of the trip. We took a bus cross-country from Siem Reap
                    to Phnom Penh and got to soak in the sights of the Cambodian
                    countryside. Altogether it was a really nice bus trip, with a
                    couple stops but nothing too egregious and tourist trappy. When we
                    arrived in Phnom Penh we checked out the rooftop pool on our hotel
                    (super cool!) and had one of the best meals of the trip, David's
                    Homemade Noodles.",
                ),
                (
                    "phnom_penh_day_2.html",
                    "Phnom Penh Day 2",
                    "phnom_penh_day_2/dylan_pool.jpg",
                    "Today we decided to learn about Khmer Rouge, an important period
                    in Cambodian history. Unfortunately, also an extremely dark period
                    of genocide. After visiting the Tuol Sleng Genocide Museum we hit
                    some of the other major attractions in town a long the riverside
                    and made our way to the famous Friends Restauraunt, where Steph
                    ATE BUGS! We spent the afternoon fully enjoying the rooftop pool
                    at the hotel, from sunbathing to the light show on the city
                    skyline at night.",
                ),
            ]),
            lftr(),
        ],
    ))
}
fn siem_reap_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/siem_reap_arrival/angkor.html")?;
    Ok((
        Page::new_override_resource("siem_reap_arrival.html", "atw/cambodia", "siem_reap_arrival"),
        content
    ))
}
fn siem_reap_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/siem_reap_day_2/angkor.html")?;
    Ok((
        Page::new_override_resource("siem_reap_day_2.html", "atw/cambodia", "siem_reap_day_2"),
        content
    ))
}
fn siem_reap_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/siem_reap_day_3/angkor.html")?;
    Ok((
        Page::new_override_resource("siem_reap_day_3.html", "atw/cambodia", "siem_reap_day_3"),
        content
    ))
}
fn siem_reap_day_4() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/siem_reap_day_4/angkor.html")?;
    Ok((
        Page::new_override_resource("siem_reap_day_4.html", "atw/cambodia", "siem_reap_day_4"),
        content
    ))
}
fn phnom_penh_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/phnom_penh_arrival/phnom_penh.html")?;
    Ok((
        Page::new_override_resource("phnom_penh_arrival.html", "atw/cambodia", "phnom_penh_arrival"),
        content
    ))
}
fn phnom_penh_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/phnom_penh_day_2/phnom_penh.html")?;
    Ok((
        Page::new_override_resource("phnom_penh_day_2.html", "atw/cambodia", "phnom_penh_day_2"),
        content
    ))
}

// Thailand
fn thailand_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {}
fn bangkok_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/bangkok_arrival/bangkok.html")?;
    Ok((
        Page::new_override_resource("bangkok_arrival.html", "atw/thailand", "bangkok_arrival"),
        content
    ))
}
fn bangkok_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/bangkok_day_2/bangkok.html")?;
    Ok((
        Page::new_override_resource("bangkok_day_2.html", "atw/thailand", "bangkok_day_2"),
        content
    ))
}
fn bangkok_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/bangkok_day_3/bangkok.html")?;
    Ok((
        Page::new_override_resource("bangkok_day_3.html", "atw/thailand", "bangkok_day_3"),
        content
    ))
}
fn bangkok_day_4() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/bangkok_day_4/bangkok.html")?;
    Ok((
        Page::new_override_resource("bangkok_day_4.html", "atw/thailand", "bangkok_day_4"),
        content
    ))
}
fn chiang_mai_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/chiang_mai_arrival/chiang_mai.html")?;
    Ok((
        Page::new_override_resource("chiang_mai_arrival.html", "atw/thailand", "chiang_mai_arrival"),
        content
    ))
}
fn chiang_mai_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/chiang_mai_day_2/chiang_mai.html")?;
    Ok((
        Page::new_override_resource("chiang_mai_day_2.html", "atw/thailand", "chiang_mai_day_2"),
        content
    ))
}
fn chiang_rai_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/chiang_rai_arrival/chiang_rai.html")?;
    Ok((
        Page::new_override_resource("chiang_rai_arrival.html", "atw/thailand", "chiang_rai_arrival"),
        content
    ))
}
fn chiang_rai_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/chiang_rai_day_2/chiang_rai.html")?;
    Ok((
        Page::new_override_resource("chiang_rai_day_2.html", "atw/thailand", "chiang_rai_day_2"),
        content
    ))
}
fn chiang_rai_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/chiang_rai_day_3/chiang_rai.html")?;
    Ok((
        Page::new_override_resource("chiang_rai_day_3.html", "atw/thailand", "chiang_rai_day_3"),
        content
    ))
}
fn railay_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/railay_arrival/railay.html")?;
    Ok((
        Page::new_override_resource("railay_arrival.html", "atw/thailand", "railay_arrival"),
        content
    ))
}
fn railay_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/railay_day_2/railay.html")?;
    Ok((
        Page::new_override_resource("railay_day_2.html", "atw/thailand", "railay_day_2"),
        content
    ))
}
fn railay_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/railay_day_3/railay.html")?;
    Ok((
        Page::new_override_resource("railay_day_3.html", "atw/thailand", "railay_day_3"),
        content
    ))
}
fn railay_day_4() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/railay_day_4/railay.html")?;
    Ok((
        Page::new_override_resource("railay_day_4.html", "atw/thailand", "railay_day_4"),
        content
    ))
}
fn railay_day_5() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/railay_day_5/railay.html")?;
    Ok((
        Page::new_override_resource("railay_day_5.html", "atw/thailand", "railay_day_5"),
        content
    ))
}
fn railay_departure_travel_day() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/railay_departure_travel_day/railay.html")?;
    Ok((
        Page::new_override_resource("railay_departure.html", "atw/thailand", "railay_departure_travel_day"),
        content
    ))
}

// England
fn england_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {}
fn london_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/london_arrival/london.html")?;
    Ok((
        Page::new_override_resource("london_arrival.html", "atw/england", "london_arrival"),
        content
    ))
}
fn london_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/london_day_2/london.html")?;
    Ok((
        Page::new_override_resource("london_day_2.html", "atw/england", "london_day_2"),
        content
    ))
}
fn london_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/london_day_3/london.html")?;
    Ok((
        Page::new_override_resource("london_day_3.html", "atw/england", "london_day_3"),
        content
    ))
}
fn london_day_4() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/london_day_4/london.html")?;
    Ok((
        Page::new_override_resource("london_day_4.html", "atw/england", "london_day_4"),
        content
    ))
}
fn london_day_5() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/london_day_5/london.html")?;
    Ok((
        Page::new_override_resource("london_day_5.html", "atw/england", "london_day_5"),
        content
    ))
}
fn york_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/york_arrival/york.html")?;
    Ok((
        Page::new_override_resource("york_arrival.html", "atw/england", "york_arrival"),
        content
    ))
}
fn york_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/york_day_2/york.html")?;
    Ok((
        Page::new_override_resource("york_day_2.html", "atw/england", "york_day_2"),
        content
    ))
}
fn malham_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/malham_arrival/malham.html")?;
    Ok((
        Page::new_override_resource("malham_arrival.html", "atw/england", "malham_arrival"),
        content
    ))
}
fn malham_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/malham_day_2/malham.html")?;
    Ok((
        Page::new_override_resource("malham_day_2.html", "atw/england", "malham_day_2"),
        content
    ))
}

// Ireland
fn ireland_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {}
fn ireland_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/dublin_arrival/dublin.html")?;
    Ok((
        Page::new_override_resource("ireland_arrival.html", "atw/ireland", "dublin_arrival"),
        content
    ))
}
fn ireland_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/dublin_day_2/dublin.html")?;
    Ok((
        Page::new_override_resource("ireland_day_1.html", "atw/ireland", "dublin_day_2"),
        content
    ))
}
fn ireland_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/dublin_day_3/dublin.html")?;
    Ok((
        Page::new_override_resource("ireland_day_3.html", "atw/ireland", "dublin_day_3"),
        content
    ))
}
fn ireland_day_4() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_4/limavady.html")?;
    Ok((
        Page::new_override_resource("ireland_day_4.html", "atw/ireland", "ireland_day_4"),
        content
    ))
}
fn ireland_day_5() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_5/donegal.html")?;
    Ok((
        Page::new_override_resource("ireland_day_5.html", "atw/ireland", "ireland_day_5"),
        content
    ))
}
fn ireland_day_6() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_6/galway.html")?;
    Ok((
        Page::new_override_resource("ireland_day_6.html", "atw/ireland", "ireland_day_6"),
        content
    ))
}
fn ireland_day_7() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_7/galway.html")?;
    Ok((
        Page::new_override_resource("ireland_day_7.html", "atw/ireland", "ireland_day_7"),
        content
    ))
}
fn ireland_day_8() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_8/blarney.html")?;
    Ok((
        Page::new_override_resource("ireland_day_8.html", "atw/ireland", "ireland_day_8"),
        content
    ))
}
fn ireland_day_9() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_9/blarney.html")?;
    Ok((
        Page::new_override_resource("ireland_day_9.html", "atw/ireland", "ireland_day_9"),
        content
    ))
}
fn ireland_day_10() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_10/kilkenny.html")?;
    Ok((
        Page::new_override_resource("ireland_day_10.html", "atw/ireland", "ireland_day_10"),
        content
    ))
}
fn ireland_day_11() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_11/kilkenny.html")?;
    Ok((
        Page::new_override_resource("ireland_day_11.html", "atw/ireland", "ireland_day_11"),
        content
    ))
}

// Scotland
fn scotland_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {}
fn scotland_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/scotland_arrival/glasgow.html")?;
    Ok((
        Page::new_override_resource("scotland_arrival.html", "atw/scotland", "scotland_arrival"),
        content
    ))
}
fn scotland_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/scotland_day_2/glasgow.html")?;
    Ok((
        Page::new_override_resource("scotland_day_2.html", "atw/scotland", "scotland_day_2"),
        content
    ))
}
fn islay_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/islay_arrival/islay.html")?;
    Ok((
        Page::new_override_resource("islay_arrival.html", "atw/scotland", "islay_arrival"),
        content
    ))
}
fn islay_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/islay_day_2/islay.html")?;
    Ok((
        Page::new_override_resource("islay_day_2.html", "atw/scotland", "islay_day_2"),
        content
    ))
}
fn islay_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/islay_day_3/islay.html")?;
    Ok((
        Page::new_override_resource("islay_day_3.html", "atw/scotland", "islay_day_3"),
        content
    ))
}
fn oban_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/oban_arrival/oban.html")?;
    Ok((
        Page::new_override_resource("oban_arrival.html", "atw/scotland", "oban_arrival"),
        content
    ))
}
fn glencoe_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/glencoe_arrival/glencoe.html")?;
    Ok((
        Page::new_override_resource("glencoe_arrival.html", "atw/scotland", "glencoe_arrival"),
        content
    ))
}
fn glencoe_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/glencoe_day_2/glencoe.html")?;
    Ok((
        Page::new_override_resource("glencoe_day_2.html", "atw/scotland", "glencoe_day_2"),
        content
    ))
}
fn edinburgh_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/edinburgh_arrival/edinburgh.html")?;
    Ok((
        Page::new_override_resource("edinburgh_arrival.html", "atw/scotland", "edinburgh_arrival"),
        content
    ))
}
fn edinburgh_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/edinburgh_day_2/edinburgh.html")?;
    Ok((
        Page::new_override_resource("edinburgh_day_2.html", "atw/scotland", "edinburgh_day_2"),
        content
    ))
}

// Portugal
fn portugal_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {}

// Spain
fn spain_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {}

// Greece
fn greece_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {}

// Czech Republic
fn czech_republic_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {}
