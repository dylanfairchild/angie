use std::error::Error;

use crate::{Htmlize, Page};
use crate::trip::{hdr, ftr, sec, TripDayPage};

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
