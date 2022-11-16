use crate::zebra::{Zebra, ZebraBuilder};


fn is_immediately_to_the_right(v1: &str, v2: &str) -> bool {
    distance(v1, v2) == 1
}

fn is_next_to(v1: &str, v2: &str) -> bool {
    i32::abs(distance(v1, v2)) == 1
}

fn distance(v1: &str, v2: &str) -> i32 {
    let conv = |v| { String::from(v).parse::<i32>().unwrap()};
    conv(v2) - conv(v1)
}

pub fn init_my_zebra<'a>() -> Zebra<'a> {
    ZebraBuilder::new()
        //    1. There are five houses.
        .set_object_count(5)
        //    2. The Englishman lives in the red house.
        .fact("nationality", "Englishman", "color", "red")
        //    3. The Spaniard owns the dog.
        .fact("nationality", "Spaniard", "pet", "dog")
        //    4. Coffee is drunk in the green house.
        .fact("beverage", "coffee", "color", "green")
        //    5. The Ukrainian drinks tea.
        .fact("nationality", "Ukrainian", "beverage", "tea")
        //    6. The green house is immediately to the right of the ivory house.
        .predicate(
            "color",
            "green",
            "color",
            "ivory",
            "position",
            "position",
            &is_immediately_to_the_right,
        )
        //    7. The Old Gold smoker owns snails.
        .fact("smoke", "OldGold", "pet", "snail")
        //    8. Kools are smoked in the yellow house.
        .fact("smoke", "Kools", "color", "yellow")
        //    9. Milk is drunk in the middle house.
        .fact("beverage", "milk", "position", "3")
        //    10. The Norwegian lives in the first house.
        .fact("nationality", "Norwegian", "position", "1")
        //    11. The man who smokes Chesterfields lives in the house next to the man with the fox.
        .predicate(
            "smoke",
            "Chesterfields",
            "pet",
            "fox",
            "position",
            "position",
            &is_next_to,
        )
        //   12. Kools are smoked in the house next to the house where the horse is kept.
        .predicate(
            "smoke",
            "Kools",
            "pet",
            "horse",
            "position",
            "position",
            &is_next_to,
        )
        //   13. The Lucky Strike smoker drinks orange juice.
        .fact("smoke", "LuckyStrike", "beverage", "juice")
        //    14. The Japanese smokes Parliaments.
        .fact("nationality", "Japanese", "smoke", "Parliaments")
        //    15. The Norwegian lives next to the blue house.
        .predicate(
            "nationality",
            "Norwegian",
            "color",
            "blue",
            "position",
            "position",
            &is_next_to,
        )
        .choice("position", vec!["2", "4", "5"])
        .choice("beverage", vec!["watter"])
        .choice("pet", vec!["zebra"])
        .build()
}


#[test]
pub fn test_distance() {
    assert_eq!(2, distance("1", "3"));
}