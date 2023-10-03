use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Food;

pub fn to_thing<'a>(num: u32, food: Food) -> Thing<'a> {
    let mut result: Thing;
    match food {
        Food::Bread => result = Thing::new( Sex::Any, false, "piece of bread", "pieces of bread"),
        Food::Croissant => result = Thing::new( Sex::Any, false, "croissant", "croissants"),
        Food::Cake => result = Thing::new( Sex::Any, false, "piece of cake", "pieces of cake"),
        Food::Pizza => result = Thing::new( Sex::Any, false, "piece of pizza", "pieces of pizza"),
        Food::Rice => result = Thing::new( Sex::Any, false, "bowl of rice", "bowls of rice"),
        Food::Soup => result = Thing::new( Sex::Any, false, "dish of soup", "dishes of soup"),
        Food::Any => result = Thing::new( Sex::Any, false, "food","foods")
    }
    result.set_num(num);
    result 
}