use std::error::Error;

use crate::trip::{ftr, hdr, lcnt, lftr, lhdr, sec, gen_from_bucket_html, LocationPage, TripDayPage};
use crate::{Htmlize, Page};

pub fn cambodia() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::Location(LocationPage::new("cambodia.html", "atw/cambodia")),
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

pub fn siem_reap_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {

    let content = gen_from_bucket_html("from-bucket/siem_reap_arrival/angkor.html")?;
    Ok((
        Page::TripDay(TripDayPage::new("siem_reap_arrival.html", "atw/cambodia", "siem_reap_arrival")),
        content
    ))
}

pub fn siem_reap_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {

    let content = gen_from_bucket_html("from-bucket/siem_reap_day_2/angkor.html")?;
    Ok((
        Page::TripDay(TripDayPage::new("siem_reap_day_2.html", "atw/cambodia", "siem_reap_day_2")),
        content
    ))
}

pub fn siem_reap_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {

    let content = gen_from_bucket_html("from-bucket/siem_reap_day_3/angkor.html")?;
    Ok((
        Page::TripDay(TripDayPage::new("siem_reap_day_3.html", "atw/cambodia", "siem_reap_day_3")),
        content
    ))
}

pub fn siem_reap_day_4() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {

    let content = gen_from_bucket_html("from-bucket/siem_reap_day_4/angkor.html")?;
    Ok((
        Page::TripDay(TripDayPage::new("siem_reap_day_4.html", "atw/cambodia", "siem_reap_day_4")),
        content
    ))
}

pub fn phnom_penh_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {

    let content = gen_from_bucket_html("from-bucket/phnom_penh_arrival/phnom_penh.html")?;
    Ok((
        Page::TripDay(TripDayPage::new("phnom_penh_arrival.html", "atw/cambodia", "phnom_penh_arrival")),
        content
    ))
}

pub fn phnom_penh_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {

    let content = gen_from_bucket_html("from-bucket/phnom_penh_day_2/phnom_penh.html")?;
    Ok((
        Page::TripDay(TripDayPage::new("phnom_penh_day_2.html", "atw/cambodia", "phnom_penh_day_2")),
        content
    ))
}
