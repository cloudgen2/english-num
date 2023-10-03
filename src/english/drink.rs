use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Drink;

pub fn to_thing<'a>(num: u32, drink: Drink) -> Thing<'a> {
    let mut result: Thing;
    match drink {
        Drink::Beer => result = Thing::new( Sex::Any, false, "glass of beer", "glasses of beer"),
        Drink::Coffee => result = Thing::new( Sex::Any, false, "cup of coffee", "cups of coffee"),
        Drink::Milk => result = Thing::new( Sex::Any, false, "glass of milk", "glasses of milk"),
        Drink::Tea => result = Thing::new( Sex::Any, false, "cup of tea", "cups of tea"),
        Drink::Water => result = Thing::new( Sex::Any, false, "glass of water", "glasses of water"),
        Drink::Wine => result = Thing::new( Sex::Any, false, "glass of wine", "glasses of wine"),
        Drink::Any => result = Thing::new( Sex::Any, false, "glass of drink", "glasses of drink")
    }
    result.set_num(num);
    result 
}