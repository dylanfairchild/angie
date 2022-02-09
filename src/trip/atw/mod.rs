use std::error::Error;

use crate::trip::{gen_from_bucket_html, icnt, iftr, ihdr, lcnt, lftr, lhdr};
use crate::{Htmlize, Page};

// Aggregation function for the entire atw page family.
pub fn atw() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    let mut res: Vec<(Page, Vec<Box<dyn Htmlize>>)> = Vec::new();

    res.push(index()?);
    res.append(&mut washington()?);
    res.append(&mut south_korea()?);
    res.append(&mut vietnam()?);
    res.append(&mut cambodia()?);
    res.append(&mut thailand()?);
    res.append(&mut england()?);
    res.append(&mut ireland()?);
    res.append(&mut scotland()?);
    res.append(&mut portugal()?);
    res.append(&mut spain()?);
    res.append(&mut greece()?);
    res.append(&mut czech_republic()?);

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
                (
                    "south_korea/south_korea.html",
                    "South Korea",
                    "Seoul",
                    "seoul_day_2/temple.jpg",
                ),
                (
                    "vietnam/vietnam.html",
                    "Vietnam",
                    "Hanoi, Ho Chi Minh City",
                    "ho_chi_minh_arrival/banh_mi_pupper.jpg",
                ),
                (
                    "cambodia/cambodia.html",
                    "Cambodia",
                    "Siem Reap, Phnom Penh",
                    "siem_reap_arrival/me_lotus.jpg",
                ),
                (
                    "thailand/thailand.html",
                    "Thailand",
                    "Bangkok, Chiang Mai, Chiang Rai, Railay",
                    "railay_day_2/walking_to_climb_spot.jpg",
                ),
                (
                    "england/england.html",
                    "England",
                    "London, Seaford, Bath, York, Malham",
                    "malham_arrival/end1.jpg",
                ),
                (
                    "ireland/ireland.html",
                    "Ireland",
                    "The Entire Island",
                    "dublin_day_2/guinness_group_selfie.jpg",
                ),
                (
                    "scotland/scotland.html",
                    "Scotland",
                    "Glasgow, Islay, Oban, Glencoe, Edinburgh",
                    "glencoe_arrival/pap17.jpg",
                ),
                (
                    "portugal/portugal.html",
                    "Portugal",
                    "Porto, Lisbon",
                    "porto_day_2/sunset4.jpg",
                ),
                (
                    "spain/spain.html",
                    "Spain",
                    "Andalucia, Madrid, Barcelona",
                    "seville_arrival/dinner1.jpg",
                ),
                (
                    "greece/greece.html",
                    "Greece",
                    "Athens, Hydra, Santorini, Paros",
                    "paros_day_2/b10.jpg",
                ),
                (
                    "czech_republic/czech_republic.html",
                    "Czech Republic",
                    "Prague",
                    "prague_arrival/explore3.jpg",
                ),
            ]),
            iftr(),
        ],
    ))
}

fn washington() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok(vec![
        washington_index()?,
        preparation()?,
        grandma()?,
        elin()?,
    ])
}
fn south_korea() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok(vec![
        south_korea_index()?,
        seoul_arrival()?,
        seoul_day_2()?,
        seoul_day_3()?,
    ])
}
fn vietnam() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok(vec![
        vietnam_index()?,
        hanoi_arrival()?,
        hanoi_day_2()?,
        ho_chi_minh_arrival()?,
        ho_chi_minh_day_2()?,
    ])
}
fn cambodia() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok(vec![
        cambodia_index()?,
        siem_reap_arrival()?,
        siem_reap_day_2()?,
        siem_reap_day_3()?,
        siem_reap_day_4()?,
        phnom_penh_arrival()?,
        phnom_penh_day_2()?,
    ])
}
fn thailand() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok(vec![
        thailand_index()?,
        bangkok_arrival()?,
        bangkok_day_2()?,
        bangkok_day_3()?,
        bangkok_day_4()?,
        chiang_mai_arrival()?,
        chiang_mai_day_2()?,
        chiang_rai_arrival()?,
        chiang_rai_day_2()?,
        chiang_rai_day_3()?,
        railay_arrival()?,
        railay_day_2()?,
        railay_day_3()?,
        railay_day_4()?,
        railay_day_5()?,
        railay_departure_travel_day()?,
    ])
}
fn england() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok(vec![
        england_index()?,
        london_arrival()?,
        london_day_2()?,
        london_day_3()?,
        london_day_4()?,
        london_day_5()?,
        york_arrival()?,
        york_day_2()?,
        malham_arrival()?,
        malham_day_2()?,
    ])
}
fn ireland() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok(vec![
        ireland_index()?,
        ireland_arrival()?,
        ireland_day_2()?,
        ireland_day_3()?,
        ireland_day_4()?,
        ireland_day_5()?,
        ireland_day_6()?,
        ireland_day_7()?,
        ireland_day_8()?,
        ireland_day_9()?,
        ireland_day_10()?,
        ireland_day_11()?,
    ])
}
fn scotland() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok(vec![
        scotland_index()?,
        scotland_arrival()?,
        scotland_day_2()?,
        islay_arrival()?,
        islay_day_2()?,
        islay_day_3()?,
        oban_arrival()?,
        glencoe_arrival()?,
        glencoe_day_2()?,
        edinburgh_arrival()?,
        edinburgh_day_2()?,
    ])
}
fn portugal() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok(vec![
        portugal_index()?,
        porto_arrival()?,
        porto_day_2()?,
        porto_day_3()?,
        lisbon_arrival()?,
        lisbon_day_2()?,
        lisbon_day_3()?,
        lisbon_day_4()?,
    ])
}
fn spain() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok(vec![
        spain_index()?,
        seville_arrival()?,
        seville_day_2()?,
        seville_day_3()?,
        nerja_arrival()?,
        nerja_day_2()?,
        granada_arrival()?,
        granada_day_2()?,
        cuenca_arrival()?,
        madrid_arrival()?,
        madrid_day_2()?,
        madrid_day_3()?,
        barcelona_arrival()?,
        barcelona_day_2()?,
        barcelona_day_3()?,
    ])
}
fn greece() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok(vec![
        greece_index()?,
        athens_arrival()?,
        athens_day_2()?,
        athens_day_3()?,
        santorini_arrival()?,
        santorini_day_2()?,
        santorini_day_3()?,
        paros_arrival()?,
        paros_day_2()?,
    ])
}
fn czech_republic() -> Result<Vec<(Page, Vec<Box<dyn Htmlize>>)>, Box<dyn Error>> {
    Ok(vec![
        czech_republic_index()?,
        prague_arrival()?,
        prague_day_2()?,
        prague_day_3()?,
    ])
}

// Washington
fn washington_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("washington.html", "atw/washington"),
        vec![
            lhdr("Washington"),
            lcnt(vec![
                (
                    "preparation.html",
                    "Preparing for the Trip",
                    "preparation/steph_stuff_packed.jpg",
                    "A quick primer on how to get 3 months of stuff into one backpack!"
                ),
                (
                    "grandma.html",
                    "Visiting Grandma Fairchild",
                    "grandma/Snoquamie_group_shot.jpg",
                    "We kicked off our trip with a visit to Grandma Fairchild and the rest of the Washington Fairchilds."
                ),
                (
                    "elin.html",
                    "Elin & the Fremont Fair",
                    "fremont/Fremont_cars.jpg",
                    "Before we left the USA we met up with Elin to visit the one-and-only Fremont Fair.",
                )
            ]),
            lftr()
        ]
    ))
}
fn preparation() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/preparation/preparation.html")?;
    Ok((
        Page::new_override_resource("preparation.html", "atw/washington", "preparation"),
        content,
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
        Page::new_override_resource("elin.html", "atw/washington", "fremont"),
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
                (
                    "seoul_arrival.html",
                    "Seoul Arrival",
                    "seoul_arrival/Netflix_dumplings.jpg",
                    "After our long cross-Pacific flight we arrived in Seoul exhausted.
                We started by finding the hotel, then headed out to Kwanjang
                Market to try some authentic Korean food.",
                ),
                (
                    "seoul_day_2.html",
                    "Seoul Day 2",
                    "seoul_day_2/secret_garden.jpg",
                    r#"Our first full day in Seoul we decided to soak up some of the
                history of Seoul and hit palaces, hanok villages, and Buddhist
                temples. Of course, then we found our way to the "food street"!"#,
                ),
                (
                    "seoul_day_3.html",
                    "Seoul Day 3",
                    "seoul_day_3/palace_lake.jpg",
                    "We could not hit all the shrines and palaces yesterday, so for
                today we set out for Jongmyo Shrine and Gyeongbokgung Palace. We
                also climbed to the top of N. Seoul Tower to get a birds eye view
                of the city (and some chicken and beer)!",
                ),
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
                (
                    "hanoi_arrival.html",
                    "Hanoi Arrival",
                    "hanoi_arrival/Hoan_Kiem_lake.jpg",
                    r#"After our cross-Pacific "layover" in South Korea, it was time to
                travel onto Vietnam. Our first stop was the beautiful city of
                Hanoi, which was an excellent introduction to Vietnam. This is a
                city where you can feel the culture, and the heat!"#,
                ),
                (
                    "hanoi_day_2.html",
                    "Hanoi Day 2",
                    "hanoi_day_2/one_pillar_pagoda.jpg",
                    "This was our only full day in Hanoi, so we braved the heat and the
                motorbikes (traffic is crazy here) to visit some of the major
                attractions like the Ho Chi Minh Mausoleum. We also found one of
                Steph's favorite things about Vietnam, egg coffee.",
                ),
                (
                    "ho_chi_minh_city_arrival.html",
                    "Ho Chi Minh City Arrival",
                    "ho_chi_minh_arrival/uncle_ho_statue.jpg",
                    "After a couple days in Hanoi it was time to head over to Ho Chi
                    Minh City and check out another part of Vietnam. Compared to
                    Hanoi, Ho Chi Minh City is far more industrial. On day one, we
                    sampled some outstanding craft beers and pho, hit some major
                    attractions, and got a taste of the night life.",
                ),
                (
                    "ho_chi_minh_city_day_2.html",
                    "Ho Chi Minh City Day 2",
                    "ho_chi_minh_day_2/netflix_banh_mi.jpg",
                    r#"On day two we answered the question "How does Vietnam remember the
                    Vietnam War?" and the answers we got were chilling. At the War
                    Remnants Museum we got an alternative look at the atrocities
                    committed during the war. Afterward we toured around town a bit
                    and capped off Vietnam with an excellent meal at a restaurant
                    recommended by Cam Ly!"#,
                ),
            ]),
            lftr(),
        ],
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
        Page::new_override_resource(
            "ho_chi_minh_arrival.html",
            "atw/vietnam",
            "ho_chi_minh_arrival",
        ),
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
        Page::new_override_resource(
            "siem_reap_arrival.html",
            "atw/cambodia",
            "siem_reap_arrival",
        ),
        content,
    ))
}
fn siem_reap_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/siem_reap_day_2/angkor.html")?;
    Ok((
        Page::new_override_resource("siem_reap_day_2.html", "atw/cambodia", "siem_reap_day_2"),
        content,
    ))
}
fn siem_reap_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/siem_reap_day_3/angkor.html")?;
    Ok((
        Page::new_override_resource("siem_reap_day_3.html", "atw/cambodia", "siem_reap_day_3"),
        content,
    ))
}
fn siem_reap_day_4() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/siem_reap_day_4/angkor.html")?;
    Ok((
        Page::new_override_resource("siem_reap_day_4.html", "atw/cambodia", "siem_reap_day_4"),
        content,
    ))
}
fn phnom_penh_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/phnom_penh_arrival/phnom_penh.html")?;
    Ok((
        Page::new_override_resource(
            "phnom_penh_arrival.html",
            "atw/cambodia",
            "phnom_penh_arrival",
        ),
        content,
    ))
}
fn phnom_penh_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/phnom_penh_day_2/phnom_penh.html")?;
    Ok((
        Page::new_override_resource("phnom_penh_day_2.html", "atw/cambodia", "phnom_penh_day_2"),
        content,
    ))
}

// Thailand
fn thailand_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("thailand.html", "atw/thailand"),
        vec![
            lhdr("Thailand"),
            lcnt(vec![
                ("bangkok_arrival.html", "Bangkok Arrival", "bangkok_arrival/Saranrom Palace.jpg", "The plane from Phnom Penh, finding our hotel, and an amazing riverside dinner with a view of Wat Arun."),
                ("bangkok_day_2.html", "Bangkok Day 2", "bangkok_day_2/khon_dance_battle.jpg", "We got busy around the city visiting temples and monuments, found our way to a free performance based on the Ramakein epic, braved the water taxi, and lived up the backpacker experience on Khao San road."),
                ("bangkok_day_3.html", "Bangkok Day 3", "bangkok_day_3/wat_arun.jpg", "After dealing with the aftermath of eating the street food we visited Wat Arun across the river. We decided to go safe for dinner and visited a delicious restaurant owned by a Michelin star chef."),
                ("bangkok_day_4.html", "Bangkok Day 4", "bangkok_day_4/tree_head.jpg", "Tour day! We joined a bus tour to see the Sumemr Palace, Ayothaya Floating Market, and Ayutthaya Ruins (a World Heritage site with an amazing buddha statue head in a tree)."),
                ("chiang_mai_arrival.html", "Chiang Mai Arrival", "chiang_mai_arrival/live_music.jpg", "We said goodbye to Bangkok and journeyed up to Chiang Mai in northern Thailand. The biggest hit today was the night market and singing Country Roads with the live band (a Thai hit apparently)."),
                ("chiang_mai_day_2.html", "Chiang Mai Day 2", "chiang_mai_day_2/inside_city_pillar2.jpg", "Visiting temples mixed with travel stress. Besides finally securing some tickets for a bus trip to Chiang Rai, Dylan got the privilege of visiting the exclusive men-only \"city pillar\"."),
                ("chiang_rai_arrival.html", "Chiang Rai Arrival", "chiang_rai_arrival/white_temple2.jpg", "After a miserable, miserable, MISERABLE bus ride, we hit up the first of the famous Chiang Rai temples; the White Temple."),
                ("chiang_rai_day_2.html", "Chiang Rai Day 2", "chiang_rai_day_2/blue2.jpg", "We did not want to leave any color of temple off of our list, so we visited the Blue Temple today. Luckily we caught a tuk tuk back, because the heat was deadly!"),
                ("chiang_rai_day_3.html", "Chiang Rai Day 3", "chiang_rai_day_3/looking_from_rock.jpg", "Emboldened by our tuk tuk success on the previous day we got a half-day from a nice old man that picked up the trade in retirement. We visited the Khun Korn waterfall, a beautiful hike just outside of town, and Singha Park. Then we enjoyed some more of the best burgers Thailand has to offer."),
                ("railay_arrival.html", "Railay Arrival", "railay_arrival/heading_toward_storm.jpg", "Out of the heat of northern Thailand, we ventured to the rain of southern Thailand. We took a small boat out to our resort on Railay Peninsula for a week of relaxation (and rock climbing)."),
                ("railay_day_2.html", "Railay Day 2", "railay_day_2/walking_to_climb_spot.jpg", "Climb day! We hired a guide to take us rock climbing, one of the activities Railay is famous for. The cliffs were beautiful, and the rain let up for long enough that we got plenty of mostly-dry climbs in!"),
                ("railay_day_3.html", "Railay Day 3", "railay_day_3/penis_cave.jpg", "We went for a bit of exploring on our own today and enjoyed the natural beauty of the peninsula. Steph also found the \"penis cave\"."),
                ("railay_day_4.html", "Railay Day 4", "railay_day_4/monkeys_rails.jpg", "A day of pouring rain, we settled in to a day of reading, posting, and watching the monkeys. By night we decided to venture around the beach to visit a neighboring area on the peninsula which was equally beautiful."),
                ("railay_day_5.html", "Railay Day 5", "railay_day_5/beach_morning.jpg", "Finally the sun came out and we got a little time on the beach. When the rain came back we settled into some more reading, posting, and eating."),
                ("railay_departure.html", "Railay Departure", "railay_departure_travel_day/farewell_selfie.jpg", "Our last day in Asia gone, it was time to make the 32-hour trip to London."),
            ]),
            lftr(),
        ],
    ))
}
fn bangkok_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/bangkok_arrival/bangkok.html")?;
    Ok((
        Page::new_override_resource("bangkok_arrival.html", "atw/thailand", "bangkok_arrival"),
        content,
    ))
}
fn bangkok_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/bangkok_day_2/bangkok.html")?;
    Ok((
        Page::new_override_resource("bangkok_day_2.html", "atw/thailand", "bangkok_day_2"),
        content,
    ))
}
fn bangkok_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/bangkok_day_3/bangkok.html")?;
    Ok((
        Page::new_override_resource("bangkok_day_3.html", "atw/thailand", "bangkok_day_3"),
        content,
    ))
}
fn bangkok_day_4() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/bangkok_day_4/bangkok.html")?;
    Ok((
        Page::new_override_resource("bangkok_day_4.html", "atw/thailand", "bangkok_day_4"),
        content,
    ))
}
fn chiang_mai_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/chiang_mai_arrival/chiang_mai.html")?;
    Ok((
        Page::new_override_resource(
            "chiang_mai_arrival.html",
            "atw/thailand",
            "chiang_mai_arrival",
        ),
        content,
    ))
}
fn chiang_mai_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/chiang_mai_day_2/chiang_mai.html")?;
    Ok((
        Page::new_override_resource("chiang_mai_day_2.html", "atw/thailand", "chiang_mai_day_2"),
        content,
    ))
}
fn chiang_rai_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/chiang_rai_arrival/chiang_rai.html")?;
    Ok((
        Page::new_override_resource(
            "chiang_rai_arrival.html",
            "atw/thailand",
            "chiang_rai_arrival",
        ),
        content,
    ))
}
fn chiang_rai_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/chiang_rai_day_2/chiang_rai.html")?;
    Ok((
        Page::new_override_resource("chiang_rai_day_2.html", "atw/thailand", "chiang_rai_day_2"),
        content,
    ))
}
fn chiang_rai_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/chiang_rai_day_3/chiang_rai.html")?;
    Ok((
        Page::new_override_resource("chiang_rai_day_3.html", "atw/thailand", "chiang_rai_day_3"),
        content,
    ))
}
fn railay_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/railay_arrival/railay.html")?;
    Ok((
        Page::new_override_resource("railay_arrival.html", "atw/thailand", "railay_arrival"),
        content,
    ))
}
fn railay_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/railay_day_2/railay.html")?;
    Ok((
        Page::new_override_resource("railay_day_2.html", "atw/thailand", "railay_day_2"),
        content,
    ))
}
fn railay_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/railay_day_3/railay.html")?;
    Ok((
        Page::new_override_resource("railay_day_3.html", "atw/thailand", "railay_day_3"),
        content,
    ))
}
fn railay_day_4() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/railay_day_4/railay.html")?;
    Ok((
        Page::new_override_resource("railay_day_4.html", "atw/thailand", "railay_day_4"),
        content,
    ))
}
fn railay_day_5() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/railay_day_5/railay.html")?;
    Ok((
        Page::new_override_resource("railay_day_5.html", "atw/thailand", "railay_day_5"),
        content,
    ))
}
fn railay_departure_travel_day() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/railay_departure_travel_day/railay.html")?;
    Ok((
        Page::new_override_resource(
            "railay_departure.html",
            "atw/thailand",
            "railay_departure_travel_day",
        ),
        content,
    ))
}

// England
fn england_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("england.html", "atw/england"),
        vec![
            lhdr("England"),
            lcnt(vec![
                (
                    "london_arrival.html",
                    "London Arrival",
                    "london_arrival/london_eye.jpg",
                    "Eager to get out of the heat of Thailand, we arrived to 100°F days in London! Even so, we were glad to be back to Western food and darker beers.",
                ),
                (
                    "london_day_2.html",
                    "London Day 2",
                    "london_day_2/laughing_selfie_cliffs.jpg",
                    "Treating London as home-base, we took off on the first of our south England day trips. This one was a beautiful walk along the Seven Sisters between Seaford and Eastbourne. These are the white-chalk cliffs that Dover is famous for, but more easily viewable from land.",
                ),
                (
                    "london_day_3.html",
                    "London Day 3",
                    "london_day_3/tower_bridge_close.jpg",
                    "You know Stephanie cannot resist a good museum, and London is chock full of them! Today we visited the British Museum, walked the Thames, and relaxed in the park.",
                ),
                (
                    "london_day_4.html",
                    "London Day 4",
                    "london_day_4/national_gallery.jpg",
                    "Why just visit one museum when we can go to two? After enjoying an English breakfast and watching the changing of the guard at Buckingham Palace we visited Westminster Abbey and the National Gallery.",
                ),
                (
                    "london_day_5.html",
                    "London Day 5",
                    "london_day_5/cute_park.jpg",
                    "Eager to get back out into the countryside and away from the heat of the city we day-tripped down to Bath. This is a beautiful quaint English town famous for its well preserved Roman baths. They should also be famous for their amazing Indian food!",
                ),
                (
                    "york_arrival.html",
                    "York Arrival",
                    "york_arrival/york1.jpg",
                    "This is a bit of a time-travel to keep England together on the site, but after a bit of a loop around Ireland and Scotland we landed back on our own in York. This is a busy modern town built within an ancient wall, and is quite a unique experience.",
                ),
                ("york_day_2.html", "York Day 2", "york_day_2/beer.jpg", "Today we walked the wall around York and visited the beautiful cathedral. We also enjoyed an authentic British afternoon tea and visited the Shambles, which inspired Diagon Alley from Harry Potter. To cap off the day we found the true treasure of York, the Kings Arms pub. To quote a Google review \"the beer was so cheap that they were buying strangers drinks\"."),
                (
                    "malham_arrival.html",
                    "Malham Arrival",
                    "malham_arrival/end1.jpg",
                    "We wanted to finish off our time in England by visiting the true countryside, and we got everything we could have wanted in Malham. This beautiful little town caught our attention because it features some of the best \"pub walks\" in England. I could not recommend it more highly.",
                ),
                (
                    "malham_day_2.html",
                    "Malham Day 2",
                    "malham_day_2/tarn12.jpg",
                    "For our last day in Malham we took the long trek out to the tarn and really soaked up the English countryside. After a long walk in the brisk weather we warmed up back at the pub.",
                ),
            ]),
            lftr(),
        ],
    ))
}
fn london_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/london_arrival/london.html")?;
    Ok((
        Page::new_override_resource("london_arrival.html", "atw/england", "london_arrival"),
        content,
    ))
}
fn london_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/london_day_2/london.html")?;
    Ok((
        Page::new_override_resource("london_day_2.html", "atw/england", "london_day_2"),
        content,
    ))
}
fn london_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/london_day_3/london.html")?;
    Ok((
        Page::new_override_resource("london_day_3.html", "atw/england", "london_day_3"),
        content,
    ))
}
fn london_day_4() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/london_day_4/london.html")?;
    Ok((
        Page::new_override_resource("london_day_4.html", "atw/england", "london_day_4"),
        content,
    ))
}
fn london_day_5() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/london_day_5/london.html")?;
    Ok((
        Page::new_override_resource("london_day_5.html", "atw/england", "london_day_5"),
        content,
    ))
}
fn york_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/york_arrival/york.html")?;
    Ok((
        Page::new_override_resource("york_arrival.html", "atw/england", "york_arrival"),
        content,
    ))
}
fn york_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/york_day_2/york.html")?;
    Ok((
        Page::new_override_resource("york_day_2.html", "atw/england", "york_day_2"),
        content,
    ))
}
fn malham_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/malham_arrival/malham.html")?;
    Ok((
        Page::new_override_resource("malham_arrival.html", "atw/england", "malham_arrival"),
        content,
    ))
}
fn malham_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/malham_day_2/malham.html")?;
    Ok((
        Page::new_override_resource("malham_day_2.html", "atw/england", "malham_day_2"),
        content,
    ))
}

// Ireland
fn ireland_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("ireland.html", "atw/ireland"),
        vec![
            lhdr("Ireland"),
            lcnt(vec![
                (
                    "ireland_arrival.html",
                    "Ireland Arrival",
                    "dublin_arrival/dinner1.jpg",
                    "Most of the rest of the Fairchild-Fisher-Bregy clan was set to meet us for the Ireland and Scotland legs of the trip. Where better to start than Dublin!",
                ),
                (
                    "ireland_day_2.html",
                    "Ireland Day 2",
                    "dublin_day_2/for_strength.jpg",
                    "They say when in Rome, do as the Romans do. Apparently when in Dublin, you drink. We had to run to visit all the breweries and distilleries we hit today!",
                ),
                (
                    "ireland_day_3.html",
                    "Ireland Day 3",
                    "dublin_day_3/flowers.jpg",
                    "A more relaxed day to soak up some culture at Trinity College and some whisky at Teeling Distillery.",
                ),
                (
                    "ireland_day_4.html",
                    "Ireland Day 4",
                    "ireland_day_4/causewayselfieline.jpg",
                    "Today begins the saga of the Ireland minibus with our tireless driver Randy and navigator Cole. We visited our cousin Spencer in Belfast, stopped off at Bushmills, took a quick trip around the Giant's Causeway, and ended the night in a cozy (very cold) campsite.",
                ),
                (
                    "ireland_day_5.html",
                    "Ireland Day 5",
                    "ireland_day_5/bayview3.jpg",
                    "After a whirlwind first day in the minibus we decided to do a mega-trek the second day and ended up in Dungloe at Casey's Viking Hotel. The town was having a nice party so we joined in!",
                ),
                (
                    "ireland_day_6.html",
                    "Ireland Day 6",
                    "ireland_day_6/pcastle 13.jpg",
                    "From Dungloe we ventured to Galway, but stopped off at Donegal castle and Parke's Castle (both very nice).",
                ),
                (
                    "ireland_day_7.html",
                    "Ireland Day 7",
                    "ireland_day_7/hookers.jpg",
                    "Galway was having a party when we showed up, so we enjoyed some Galway Hookers (on tap) and joined in the fun.",
                ),
                (
                    "ireland_day_8.html",
                    "Ireland Day 8",
                    "ireland_day_8/moher6.jpg",
                    "From Galway our next destination was Blarney, to go kiss a rock, but we were side tracked by two beautiful attractions: Dunguaire Castle and the Cliffs of Moher.",
                ),
                (
                    "ireland_day_9.html",
                    "Ireland Day 9",
                    "ireland_day_9/kiss1.jpg",
                    "We climbed the castle to kiss the Blarney stone and get the gift of gab. Also, the grounds are just gorgeous.",
                ),
                (
                    "ireland_day_10.html",
                    "Ireland Day 10",
                    "ireland_day_10/place1.jpg",
                    "It had been awhile since we visited a brewery so we headed on to Kilkenny to see what Smithwick's had to offer. Then we wandered our way out to our vacation rental, a beautiful house up in the nearby hills.",
                ),
                (
                    "ireland_day_11.html",
                    "Ireland Day 11",
                    "ireland_day_11/dylan3.jpg",
                    "Our final day in Ireland we visited the Kilkenny castle, climbed a church tower, and visited the aptly named Dylan Whisky Bar.",
                ),
            ]),
            lftr(),
        ],
    ))
}
fn ireland_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/dublin_arrival/dublin.html")?;
    Ok((
        Page::new_override_resource("ireland_arrival.html", "atw/ireland", "dublin_arrival"),
        content,
    ))
}
fn ireland_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/dublin_day_2/dublin.html")?;
    Ok((
        Page::new_override_resource("ireland_day_2.html", "atw/ireland", "dublin_day_2"),
        content,
    ))
}
fn ireland_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/dublin_day_3/dublin.html")?;
    Ok((
        Page::new_override_resource("ireland_day_3.html", "atw/ireland", "dublin_day_3"),
        content,
    ))
}
fn ireland_day_4() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_4/limavady.html")?;
    Ok((
        Page::new_override_resource("ireland_day_4.html", "atw/ireland", "ireland_day_4"),
        content,
    ))
}
fn ireland_day_5() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_5/donegal.html")?;
    Ok((
        Page::new_override_resource("ireland_day_5.html", "atw/ireland", "ireland_day_5"),
        content,
    ))
}
fn ireland_day_6() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_6/galway.html")?;
    Ok((
        Page::new_override_resource("ireland_day_6.html", "atw/ireland", "ireland_day_6"),
        content,
    ))
}
fn ireland_day_7() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_7/galway.html")?;
    Ok((
        Page::new_override_resource("ireland_day_7.html", "atw/ireland", "ireland_day_7"),
        content,
    ))
}
fn ireland_day_8() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_8/blarney.html")?;
    Ok((
        Page::new_override_resource("ireland_day_8.html", "atw/ireland", "ireland_day_8"),
        content,
    ))
}
fn ireland_day_9() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_9/blarney.html")?;
    Ok((
        Page::new_override_resource("ireland_day_9.html", "atw/ireland", "ireland_day_9"),
        content,
    ))
}
fn ireland_day_10() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_10/kilkenny.html")?;
    Ok((
        Page::new_override_resource("ireland_day_10.html", "atw/ireland", "ireland_day_10"),
        content,
    ))
}
fn ireland_day_11() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ireland_day_11/kilkenny.html")?;
    Ok((
        Page::new_override_resource("ireland_day_11.html", "atw/ireland", "ireland_day_11"),
        content,
    ))
}

// Scotland
fn scotland_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("scotland.html", "atw/scotland"),
        vec![
            lhdr("Scotland"),
            lcnt(vec![
                (
                    "scotland_arrival.html",
                    "Scotland Arrival",
                    "scotland_arrival/oorwullie.jpg",
                    "A short flight from Dublin and the team all made it to Glasgow to begin the Scotland tour. We spent the first day walking the town and enjoying the culture.",
                ),
                (
                    "scotland_day_2.html",
                    "Scotland Day 2",
                    "scotland_day_2/statue.jpg",
                    "For our second day in Scotland everyone was getting thirsty again so we trekked out to Tennent's. We also found an amazing Lebowski themed bar. Otherwise we spent the day relaxing around town, enjoying parks, cemeteries, churches, and the river.",
                ),
                (
                    "islay_arrival.html",
                    "Islay Arrival",
                    "islay_arrival/port2.jpg",
                    "After a few days in the big city of Glasgow it was time to head out to the small island of Islay to start tasting some of the Scotch the country is famous for. We got to enjoy some Scottish summer weather and a ferry ride along the way.",
                ),
                (
                    "islay_day_2.html",
                    "Islay Day 2",
                    "islay_day_2/laph9.jpg",
                    "Our first day out to the distilleries we hit Laphroaig (which had a particularly excellent tour guide) and bottled our own scotch to take home. Afterward we visited Ardbeg for lunch (and some more scotch), Islay Ales, and Bruichladdich Distillery.",
                ),
                (
                    "islay_day_3.html",
                    "Islay Day 3",
                    "islay_day_3/lagoutside2.jpg",
                    "Another day in Islay, another day of drinking Scotch. Today we visited Lagavulin, Bruichladdich Distillery (again), and Kilchoman. Also, sadly, Elin left us on a wee little plane to head on home.",
                ),
                (
                    "oban_arrival.html",
                    "Oban Arrival",
                    "oban_arrival/mccaig6.jpg",
                    "Unfortunately, after Islay the band had to split up. Steph and I continued on touring the highlands, while the rest of the Fairchild-Fishers short-cut to Edinburgh then caught a flight home. Our next stop was Oban, a beautiful little town on the water.",
                ),
                (
                    "glencoe_arrival.html",
                    "Glencoe Arrival",
                    "glencoe_arrival/pap5.jpg",
                    "Not wanting to leave the highlands without a bit of a hike, we took on the Pap of Glencoe. This one might have been a bit more than we bargained for, but we conquered the mountain.",
                ),
                (
                    "glencoe_day_2.html",
                    "Glencoe Day 2",
                    "glencoe_day_2/quarry3.jpg",
                    "A day for easy walks and relaxation in the beautiful environment of the highlands.",
                ),
                (
                    "edinburgh_arrival.html",
                    "Edinburgh Arrival",
                    "edinburgh_arrival/hillview.jpg",
                    "We took a scenic route through the highlands on our way to Edinburgh, our last stop in Scotland. We summitted Arthur's Seat when we arrived to get a beautiful view of the town.",
                ),
                (
                    "edinburgh_day_2.html",
                    "Edinburgh Day 2",
                    "edinburgh_day_2/firework2.jpg",
                    "Edinburgh happened to be in the middle of the Fringe festival, famous for its buskers. We took a day touring the down, including a self-run Harry Potter tour, and enjoying the festivities.",
                ),
            ]),
            lftr(),
        ],
    ))
}
fn scotland_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/scotland_arrival/glasgow.html")?;
    Ok((
        Page::new_override_resource("scotland_arrival.html", "atw/scotland", "scotland_arrival"),
        content,
    ))
}
fn scotland_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/scotland_day_2/glasgow.html")?;
    Ok((
        Page::new_override_resource("scotland_day_2.html", "atw/scotland", "scotland_day_2"),
        content,
    ))
}
fn islay_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/islay_arrival/islay.html")?;
    Ok((
        Page::new_override_resource("islay_arrival.html", "atw/scotland", "islay_arrival"),
        content,
    ))
}
fn islay_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/islay_day_2/islay.html")?;
    Ok((
        Page::new_override_resource("islay_day_2.html", "atw/scotland", "islay_day_2"),
        content,
    ))
}
fn islay_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/islay_day_3/islay.html")?;
    Ok((
        Page::new_override_resource("islay_day_3.html", "atw/scotland", "islay_day_3"),
        content,
    ))
}
fn oban_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/oban_arrival/oban.html")?;
    Ok((
        Page::new_override_resource("oban_arrival.html", "atw/scotland", "oban_arrival"),
        content,
    ))
}
fn glencoe_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/glencoe_arrival/glencoe.html")?;
    Ok((
        Page::new_override_resource("glencoe_arrival.html", "atw/scotland", "glencoe_arrival"),
        content,
    ))
}
fn glencoe_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/glencoe_day_2/glencoe.html")?;
    Ok((
        Page::new_override_resource("glencoe_day_2.html", "atw/scotland", "glencoe_day_2"),
        content,
    ))
}
fn edinburgh_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/edinburgh_arrival/edinburgh.html")?;
    Ok((
        Page::new_override_resource(
            "edinburgh_arrival.html",
            "atw/scotland",
            "edinburgh_arrival",
        ),
        content,
    ))
}
fn edinburgh_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/edinburgh_day_2/edinburgh.html")?;
    Ok((
        Page::new_override_resource("edinburgh_day_2.html", "atw/scotland", "edinburgh_day_2"),
        content,
    ))
}

// Portugal
fn portugal_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("portugal.html", "atw/portugal"),
        vec![
            lhdr("Portugal"),
            lcnt(vec![
                (
                    "porto_arrival.html",
                    "Porto Arrival",
                    "porto_arrival/porto2.jpg",
                    "It finally happened, we lost our bags. After a very stressful trip in, and a sprint around the Amsterdam airport, we did our best to enjoy the city at night in Porto.",
                ),
                (
                    "porto_day_2.html",
                    "Porto Day 2",
                    "porto_day_2/topbridge1.jpg",
                    "We took a day staying close to the hotel to retrieve our bags and saw some of the historical sites and beautiful artwork around the city. By the afternoon we were reunited with our belongings and we enjoyed a relaxed sunset on the river.",
                ),
                (
                    "porto_day_3.html",
                    "Porto Day 3",
                    "porto_day_3/sunset.jpg",
                    "Ready to devote an entire day to crawling the port cellars we ventured across the river. We hit Offley, Taylor's, Calem, and Croft, and learned a lot about what makes port port. We also picked up a new addiction to port.",
                ),
                (
                    "lisbon_arrival.html",
                    "Lisbon Arrival",
                    "lisbon_arrival/fadostreets.jpg",
                    "The travel hijinks continue in Portugal and we got booted from a train on our way to Lisbon. Luckily we figured out how to get the rest of the way there and were down in Alfama by nightfall. This is a neighborhood with charm, delicious food, and good music.",
                ),
                (
                    "lisbon_day_2.html",
                    "Lisbon Day 2",
                    "lisbon_day_2/tower1.jpg",
                    "We focused on hitting the historical highlights of Lisbon on our first full day and visited Castelo de S. Jorge, Praca do Comercio, and the Belem Tower. We also sampled some of the best natas we have found and stopped off at a Banksy exhibit. This was a big walking day!",
                ),
                (
                    "lisbon_day_3.html",
                    "Lisbon Day 3",
                    "lisbon_day_3/castle2.jpg",
                    "Sintra was the target for today, a quick train ride out of town from Lisbon. Sintra boasts some beautiful historic tourist attractions like the Quinta da Regeleira and Castelos dos Mouros.",
                ),
                (
                    "lisbon_day_4.html",
                    "Lisbon Day 4",
                    "lisbon_day_4/park1.jpg",
                    "For our final day in Lisbon we relaxed at the Eduardo VII park and caught up on some reading and posting. We also treated ourselves to a nice dinner at a corner restaurant in Alfama.",
                ),
            ]),
            lftr(),
        ],
    ))
}
fn porto_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/porto_arrival/porto.html")?;
    Ok((
        Page::new_override_resource("porto_arrival.html", "atw/portugal", "porto_arrival"),
        content,
    ))
}
fn porto_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/porto_day_2/porto.html")?;
    Ok((
        Page::new_override_resource("porto_day_2.html", "atw/portugal", "porto_day_2"),
        content,
    ))
}
fn porto_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/porto_day_3/porto.html")?;
    Ok((
        Page::new_override_resource("porto_day_3.html", "atw/portugal", "porto_day_3"),
        content,
    ))
}
fn lisbon_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/lisbon_arrival/lisbon.html")?;
    Ok((
        Page::new_override_resource("lisbon_arrival.html", "atw/portugal", "lisbon_arrival"),
        content,
    ))
}
fn lisbon_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/lisbon_day_2/lisbon.html")?;
    Ok((
        Page::new_override_resource("lisbon_day_2.html", "atw/portugal", "lisbon_day_2"),
        content,
    ))
}
fn lisbon_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/lisbon_day_3/lisbon.html")?;
    Ok((
        Page::new_override_resource("lisbon_day_3.html", "atw/portugal", "lisbon_day_3"),
        content,
    ))
}
fn lisbon_day_4() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/lisbon_day_4/lisbon.html")?;
    Ok((
        Page::new_override_resource("lisbon_day_4.html", "atw/portugal", "lisbon_day_4"),
        content,
    ))
}

// Spain
fn spain_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("spain.html", "atw/spain"),
        vec![
            lhdr("Spain"),
            lcnt(vec![
                (
                    "seville_arrival.html",
                    "Seville Arrival",
                    "seville_arrival/gardens2.jpg",
                    "From Lisbon it was a quick flight over to Seville and the beginning of our trip through Andalucia. On our first day we visited the Real Alcazar, a beautiful historic complex used for filming Game of Thrones, and had some of the best tapas... probably in the world.",
                ),
                (
                    "seville_day_2.html",
                    "Seville Day 2",
                    "seville_day_2/plaza4.jpg",
                    "Another day of active wandering through Seville and enjoying all of the history and architecture in the city. A highlight was the Plaza España which besides being impressive on its own showed some interesting motifs from the other regions of Spain we planned to visit.",
                ),
                (
                    "seville_day_3.html",
                    "Seville Day 3",
                    "seville_day_3/mosque2.jpg",
                    "Having seen all there was to see in Seville we headed off to Cordoba by train to see the famous mosque.",
                ),
                (
                    "nerja_arrival.html",
                    "Nerja Arrival",
                    "ronda_nerja_arrival/ronda3.jpg",
                    "We picked up a car to tour the rest of Andalucia and started our trek to Nerja through Ronda. Ronda is a beautiful city built across a gorge, the downtown is remarkable. We finished the day at Nerja with a dip in the crystal clear Mediterranean.",
                ),
                (
                    "nerja_day_2.html",
                    "Nerja Day 2",
                    "nerja_day_2/caves1.jpg",
                    "Other than the beach, Nerja's main attraction is its caves. We decided \"why not\" so went on the tour, and they were pretty impressive. Afterward we spent the rest of the day relaxing on the beach.",
                ),
                (
                    "granada_arrival.html",
                    "Granada Arrival",
                    "granada_arrival/flamenco2.jpg",
                    "We travelled on to Granada, home of the Alhambra and one of the major attractions we were looking forward to in Spain. After we arrived we filled the afternoon with a Flamenco show, which was a very cool experience, and some tapas. Granada does tapas better than any other city we visited in Spain.",
                ),
                (
                    "granada_day_2.html",
                    "Granada Day 2",
                    "granada_day_2/a12.jpg",
                    "All Alhambra, all day (most day). There is a reason this is one of the most visited tourist destinations in the world. The architecture and gardens are outstanding. We finished the day with a tapas tour around Granada, there is something magical about this city.",
                ),
                (
                    "cuenca_arrival.html",
                    "Cuenca Arrival",
                    "cuenca_arrival/explore4.jpg",
                    "We wanted to take advantage of the Parador system in Spain, which converts historic buildings into high-end hotels. We picked a converted monastery in Cuenca, a beautiful town built on a cliff. Although it is a quiet spot, it was an excellent pick for relaxing in a beautiful place at a luxurious hotel.",
                ),
                (
                    "madrid_arrival.html",
                    "Madrid Arrival",
                    "toledo_madrid/toledo6.jpg",
                    "On our way to Madrid we stopped at Toledo, a city that focuses on the medieval era of Spain's history. Although very tourist heavy it was a fun spot to walk around, and at times felt like a big renaissance fair.",
                ),
                (
                    "madrid_day_2.html",
                    "Madrid Day 2",
                    "madrid_day_2/park5.jpg",
                    "Having given back the car and settled back into the big-city life, Steph was ready to hit the museums. Madrid is famous for the Prado, which was our first stop, then we visited Sofia Reina to see the Guernica. Fully museumed-out, we relaxed the rest of the day at Retiro Park. We also found a gem of a beer-shop, Cerveceria 100 Montaditos. Cheap and delicious!",
                ),
                (
                    "madrid_day_3.html",
                    "Madrid Day 3",
                    "madrid_day_3/templedebod.jpg",
                    "Having seem museums the day before, we dedicated our last day in Madrid to our special brand of rapid wandering. We hit all the major sites including the Debod Temple, an ancient Egyptian temple transplanted to Madrid.",
                ),
                (
                    "barcelona_arrival.html",
                    "Barcelona Arrival",
                    "barcelona_arrival/walk1.jpg",
                    "From Madrid we took the train to Barcelona. We hid from the rain the first night, but enjoyed some cookies, pizza, and sweet wine in the Gothic Quarter.",
                ),
                (
                    "barcelona_day_2.html",
                    "Barcelona Day 2",
                    "barcelona_day_2/park6.jpg",
                    "Our second day in Barcelona was gorgeous, and we dug deep into rapid wandering. We saw one of the best fountains of all time at Ciutadella Park, had some amazing Doner Kebab, did some window shopping of La Sagrada Familia (which we had tickets to visit the next day) and otherwise enjoyed just about every site we could find.",
                ),
                (
                    "barcelona_day_3.html",
                    "Barcelona Day 3",
                    "barcelona_day_3/lunch1.jpg",
                    "Raining again but we enjoyed a delicious seafood lunch at a nearby market and struck out for La Sagrada Familia. We also made sure we didn't leave Spain without some empanadas!",
                ),
            ]),
            lftr(),
        ],
    ))
}
fn seville_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/seville_arrival/seville.html")?;
    Ok((
        Page::new_override_resource("seville_arrival.html", "atw/spain", "seville_arrival"),
        content,
    ))
}
fn seville_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/seville_day_2/seville.html")?;
    Ok((
        Page::new_override_resource("seville_day_2.html", "atw/spain", "seville_day_2"),
        content,
    ))
}
fn seville_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/seville_day_3/seville.html")?;
    Ok((
        Page::new_override_resource("seville_day_3.html", "atw/spain", "seville_day_3"),
        content,
    ))
}
fn nerja_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ronda_nerja_arrival/nerja.html")?;
    Ok((
        Page::new_override_resource("nerja_arrival.html", "atw/spain", "ronda_nerja_arrival"),
        content,
    ))
}
fn nerja_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/nerja_day_2/nerja.html")?;
    Ok((
        Page::new_override_resource("nerja_day_2.html", "atw/spain", "nerja_day_2"),
        content,
    ))
}
fn granada_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/granada_arrival/granada.html")?;
    Ok((
        Page::new_override_resource("granada_arrival.html", "atw/spain", "granada_arrival"),
        content,
    ))
}
fn granada_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/granada_day_2/granada.html")?;
    Ok((
        Page::new_override_resource("granada_day_2.html", "atw/spain", "granada_day_2"),
        content,
    ))
}
fn cuenca_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/cuenca_arrival/cuenca.html")?;
    Ok((
        Page::new_override_resource("cuenca_arrival.html", "atw/spain", "cuenca_arrival"),
        content,
    ))
}
fn madrid_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/toledo_madrid/madrid.html")?;
    Ok((
        Page::new_override_resource("madrid_arrival.html", "atw/spain", "toledo_madrid"),
        content,
    ))
}
fn madrid_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/madrid_day_2/madrid.html")?;
    Ok((
        Page::new_override_resource("madrid_day_2.html", "atw/spain", "madrid_day_2"),
        content,
    ))
}
fn madrid_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/madrid_day_3/madrid.html")?;
    Ok((
        Page::new_override_resource("madrid_day_3.html", "atw/spain", "madrid_day_3"),
        content,
    ))
}
fn barcelona_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/barcelona_arrival/barcelona.html")?;
    Ok((
        Page::new_override_resource("barcelona_arrival.html", "atw/spain", "barcelona_arrival"),
        content,
    ))
}
fn barcelona_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/barcelona_day_2/barcelona.html")?;
    Ok((
        Page::new_override_resource("barcelona_day_2.html", "atw/spain", "barcelona_day_2"),
        content,
    ))
}
fn barcelona_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/barcelona_day_3/barcelona.html")?;
    Ok((
        Page::new_override_resource("barcelona_day_3.html", "atw/spain", "barcelona_day_3"),
        content,
    ))
}

// Greece
fn greece_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("greece.html", "atw/greece"),
        vec![
            lhdr("Greece"),
            lcnt(vec![
                (
                    "athens_arrival.html",
                    "Athens Arrival",
                    "athens_arrival/group.jpg",
                    "Spain behind is it was time to meet up with some more Fairchilds over in Greece. Cole and my Dad joined us for this leg of the adventure in Athens. We met up in the afternoon so we just focused on finding some good food and ice cream.",
                ),
                (
                    "athens_day_2.html",
                    "Athens Day 2",
                    "athens_day_2/a15.jpg",
                    "For our main day in Athens we hit the Greek ruins. The Acropolis and Parthenon was the highlight, but we also hit the Temple of Zeus, Hadrian's Arch, the ancient Agora and others. It was very cool to see in person the things we learned about in books at school.",
                ),
                (
                    "athens_day_3.html",
                    "Athens Day 3",
                    "athens_day_3/h20.jpg",
                    "Having city'd ourselves out, we planned a last minute day trip to Hydra for our last day in Athens. Although the scheduling was a bit hectic with the high winds, the island was absolutely gorgeous. I am not sure anything place is as beautiful as the Greek islands.",
                ),
                (
                    "santorini_arrival.html",
                    "Santorini Arrival",
                    "ferry_day/ferry1.jpg",
                    "Unfortunately the high winds meant only the largest ferries were running, which caused us booking headache but also was not the most enjoyable ride. We finally made it to Santorini after an eternity on a big boat.",
                ),
                (
                    "santorini_day_2.html",
                    "Santorini Day 2",
                    "santorini_day_1/beach5.jpg",
                    "Our first day in Santorini was focused on recovery from that ferry ride, so we headed down to the black sand beach for some rest and relaxation. Afterward we got some gyros from Lucky Souvlaki in Fira where we were staying.",
                ),
                (
                    "santorini_day_3.html",
                    "Santorini Day 3",
                    "santorini_day_2/o20.jpg",
                    "Steph and I took a scenic hike out to Oia and got to enjoy the views from the island (and of the island). Oia is a beautiful town perched down on one end of the island, and home to one of my favorite swimming spots of all time, Amoudi Bay.",
                ),
                (
                    "paros_arrival.html",
                    "Paros Arrival",
                    "paros_arrival/p11.jpg",
                    "Our last Greek island to visit was Paros, which was a beautiful quiet little island primarily offering a spot to relax on the beach. We stayed in a nice apartment overlooking the ocean and enjoyed roaming around town.",
                ),
                ("paros_day_2.html", "Paros Day 2", "paros_day_2/b15.jpg", "For our last day on the island we hit Kolimpithres Beach and enjoyed the water and a little bouldering. Afterward Steph and I had to say goodbye and catch a ferry back to Athens for our next flight."),
            ]),
            lftr(),
        ],
    ))
}
fn athens_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/athens_arrival/athens.html")?;
    Ok((
        Page::new_override_resource("athens_arrival.html", "atw/greece", "athens_arrival"),
        content,
    ))
}
fn athens_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/athens_day_2/athens.html")?;
    Ok((
        Page::new_override_resource("athens_day_2.html", "atw/greece", "athens_day_2"),
        content,
    ))
}
fn athens_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/athens_day_3/athens.html")?;
    Ok((
        Page::new_override_resource("athens_day_3.html", "atw/greece", "athens_day_3"),
        content,
    ))
}
fn santorini_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/ferry_day/ferry.html")?;
    Ok((
        Page::new_override_resource("santorini_arrival.html", "atw/greece", "ferry_day"),
        content,
    ))
}
fn santorini_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/santorini_day_1/santorini.html")?;
    Ok((
        Page::new_override_resource("santorini_day_2.html", "atw/greece", "santorini_day_1"),
        content,
    ))
}
fn santorini_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/santorini_day_2/santorini.html")?;
    Ok((
        Page::new_override_resource("santorini_day_3.html", "atw/greece", "santorini_day_2"),
        content,
    ))
}
fn paros_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/paros_arrival/paros.html")?;
    Ok((
        Page::new_override_resource("paros_arrival.html", "atw/greece", "paros_arrival"),
        content,
    ))
}
fn paros_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/paros_day_2/paros.html")?;
    Ok((
        Page::new_override_resource("paros_day_2.html", "atw/greece", "paros_day_2"),
        content,
    ))
}

// Czech Republic
fn czech_republic_index() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::new("czech_republic.html", "atw/czech_republic"),
        vec![
            lhdr("Czech Republic"),
            lcnt(vec![
                (
                    "prague_arrival.html",
                    "Prague Arrival",
                    "prague_arrival/explore4.jpg",
                    "Prague, the city so nice I have visited it twice. After a last delicious meal in Greece (at the airport of all places) we made our way back up north so Steph could see Prague. After a little hotel hassle we were out on the town enjoying the sunset and some pilsners.",
                ),
                (
                    "prague_day_2.html",
                    "Prague Day 2",
                    "prague_day_2/a1.jpg",
                    "For our second day in town we did what we do best, wandered around and drank beer.",
                ),
                (
                    "prague_day_3.html",
                    "Prague Day 3",
                    "prague_day_3/wenceslo4.jpg",
                    "Our last day of the trip, bittersweet. We visited the castle up on the hill and enjoyed some more wandering and drinking. We also stopped in for the authentic Prague absinthe bar experience and a nice early dinner. Altogether an excellent way to cap off a journey around the world.",
                ),
            ]),
            lftr(),
        ],
    ))
}
fn prague_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/prague_arrival/prague.html")?;
    Ok((
        Page::new_override_resource(
            "prague_arrival.html",
            "atw/czech_republic",
            "prague_arrival",
        ),
        content,
    ))
}
fn prague_day_2() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/prague_day_2/prague.html")?;
    Ok((
        Page::new_override_resource("prague_day_2.html", "atw/czech_republic", "prague_day_2"),
        content,
    ))
}
fn prague_day_3() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    let content = gen_from_bucket_html("from-bucket/prague_day_3/prague.html")?;
    Ok((
        Page::new_override_resource("prague_day_3.html", "atw/czech_republic", "prague_day_3"),
        content,
    ))
}
