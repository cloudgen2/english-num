use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Fruit;

pub fn to_thing<'a>(num: u32, fruit: Fruit) -> Thing<'a> {
    let mut result: Thing;
    match fruit {
        Fruit::Apple => result = Thing::new( Sex::Any, true, "apple", "apples"),
        Fruit::Orange => result = Thing::new( Sex::Any, true, "orange", "oranges"),
        Fruit::Banana => result = Thing::new( Sex::Any, false, "banana", "bananas"),
        Fruit::Strawberry => result = Thing::new( Sex::Any, false, "strawberry", "strawberries"),
        Fruit::Pear => result = Thing::new( Sex::Any, false, "pear", "pears" ),
        Fruit::WaterMelon => result = Thing::new( Sex::Any, false, "watermelon", "watermelons" ),
        Fruit::Cherry => result = Thing::new( Sex::Any, false, "cherry", "cherries" ),
        Fruit::Any => result = Thing::new( Sex::Any, false, "fruit", "fruits")
    }
    result.set_num(num);
    result 
}