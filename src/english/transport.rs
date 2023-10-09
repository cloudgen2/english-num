use super::Thing;
use crate::thisis::ThisIs;
use crate::entities::Sex;
use crate::entities::Transport;

pub fn to_thing<'a>(num: u32, transport: Transport) -> Thing<'a> {
    let mut result: Thing;
    match transport {
        Transport::Ambulance => result = Thing::new(Sex::Any, true, "ambulance", "ambulances"),
        Transport::Bus => result = Thing::new(Sex::Any, false, "bus", "buses"),
        Transport::Car => result = Thing::new(Sex::Any, false, "car", "cars"),
        Transport::Taxi => result = Thing::new(Sex::Any, false, "taxi", "taxis"),
        Transport::Any => result = Thing::new(Sex::Any, false, "mean of transport", "transports")
    }
    result.set_num(num);
    result 
}