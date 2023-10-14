use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Animal;

pub fn to_thing<'a>(num: u32, animal: Animal) -> Thing<'a> {
    let mut result: Thing;
    match animal {
        Animal::Bird => result = Thing::new( Sex::Any, false, "bird", "birds"),
        Animal::Cat => result = Thing::new( Sex::Any, false, "cat", "cats"),
        Animal::Dog => result = Thing::new( Sex::Any, false, "dog", "dogs"),
        Animal::Fish => result = Thing::new( Sex::Any, false, "fish", "fishes" ),
        Animal::Horse => result = Thing::new( Sex::Any, false,  "horse", "horses"),
        Animal::Rabbit => result = Thing::new( Sex::Any, false, "rabbit", "rabbits"),
        Animal::Pig => result = Thing::new( Sex::Any, false, "pig", "pigs"),
        Animal::Any => result = Thing::new( Sex::Any, true, "animal", "animals")
    }
    result.set_num(num);
    result 
}
