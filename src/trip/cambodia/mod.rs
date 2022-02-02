use std::error::Error;

use crate::trip::{ftr, hdr, lcnt, lftr, lhdr, sec, LocationPage, TripDayPage};
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
                    and otherwise took the opportunity to work on our posts and books."
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
                    Homemade Noodles."
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
                    skyline at night."
                )
            ]),
            lftr(),
        ],
    ))
}

pub fn siem_reap_arrival() -> Result<(Page, Vec<Box<dyn Htmlize>>), Box<dyn Error>> {
    Ok((
        Page::TripDay(TripDayPage::new("siem_reap_arrival.html", "atw/cambodia", "siem_reap_arrival")),
        vec![
            hdr("Onto Cambodia"),
            sec("Welcome to Siem Reap",
                vec![
                    "We got up pretty early to catch a flight to Cambodia. It was a short flight and a very small
                    airplane!",
                    "cambodia_air.jpg", 
                    "From the airport, we took a quick cab ride to where we are staying. The place, Onederz, is a hostel that also
                    has
                    some private rooms. It's turned out to be really fun to get the backpacker/hostel environment
                    without having to stay in big dorm rooms.",
                    "Our room wasn't ready yet, so we headed to nearby Pub Street to get some food and beers.",
                    "pub_street.jpg",
                    "Cambodia has adopted USD as the country's de facto currency, so it's been really easy to pay
                    for things. Most dishes on Pub Street are $5-6, but the standard price for a pint of draft Angkor
                    beer at all of the bars and restaurants is $0.50! We have done our best to take advantage of
                    the cheap beer!",
                    "food_day1.jpg",
                    "The street is pretty cute too!",
                    "pub_street_umbrellas.jpg",
                ]),
            sec("Bike Tour",
                vec![
                    "We weren't ready to venture to Angkor Wat yet, so we decided to sign up for a bike tour through
                    the hostel.",
                    "We biked to a local village, which had a pretty temple and a very big pig roaming around.",
                    "bike_tour_village.jpg",
                    "pig.jpg",
                    "It started pouring while we were at the village, so we waited under the eaves of the temple
                    for a little while. Eventually someone from the hostel brought some ponchos for us to continue
                    our ride.",
                    "rain_coat.jpg",
                    "The next stop on our bike tour was a lotus field. We
                    watched our guide demonstrate lotus folding and had a beer to keep hydrated.",
                    "lotus_field.jpg",
                    "lotus_folding.jpg",
                    "me_lotus.jpg",
                    "beer_lotus.jpg",
                    "We then sat in these floating huts as the sun set. It's definitely beauitful here!",
                    "night_lotus.jpg"
                ]),
            sec("Back to Pub Street!",
                vec![
                    "After our bike tour, we headed back to Pub Street to grab some food and check out the night market.",
                    "back_to_pub.jpg",
                    "night_market.jpg",
                    "After eating traditional Khmer food for lunch, we decided to try the Cambodian spin on doner kebab
                    for dinner.
                    It was a little sweeter than what we have had in Europe, but it was tasty!",
                    "kebab.jpg",
                    "And embracing the sweetness of everything in Southeast Asia, we decided to get rolled ice cream
                    from one of the many stands on Pub Street. It wasn't the best ice cream we've had, but it was a fun
                    novelty. And we decided that making it would be a great way to keep our arms in shape!",
                    "ice_cream_roll.jpg",
                    "yum.jpg"
                ]),
            ftr()
        ],
    ))
}
