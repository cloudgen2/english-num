mod below1000;
use below1000::below1000;

pub fn all_num(number: u32) -> String {
    let mut result=String::new();
    let diff: u32;
    if number<1000 {
        result.push_str(&below1000( number));
    } else if number == 1000 {
        result.push_str("one thousand");
    } else if number < 2000 {
        result.push_str("one thousand ");
        diff = number - 1000;
        result.push_str(&below1000( diff));
    } else if number == 2000 {
        result.push_str("two thousand");
    } else if number < 3000 {
        result.push_str("two thousand ");
        diff = number - 2000;
        result.push_str(&below1000( diff));
    } else if number == 3000 {  
        result.push_str("three thousand");
    } else if number < 4000 {
        result.push_str("three thousand ");
        diff = number - 3000;
        result.push_str(&below1000( diff));
    } else if number == 4000 {
        result.push_str("four thousand");
    } else if number < 5000 {
        result.push_str("four thousand ");
        diff = number - 4000;
        result.push_str(&below1000( diff));
    } else if number == 5000 {
        result.push_str("five thousand");
    } else if number < 6000 {
        result.push_str("five thousand ");
        diff = number - 5000; 
        result.push_str(&below1000( diff));
    } else if number == 6000 {
        result.push_str("six thousand");
    } else if number < 7000 {
        result.push_str("six thousand ");
        diff = number - 6000;
        result.push_str(&below1000( diff));
    } else if number == 7000 {
        result.push_str("seven thousand");
    } else if number < 8000 {
        result.push_str("seven thousand ");
        diff = number - 7000;
        result.push_str(&below1000( diff));
    } else if number == 8000 {
        result.push_str("eight thousand");
    } else if number < 9000 {
        result.push_str("eight thousand ");
        diff = number - 8000;
        result.push_str(&below1000(diff));
    } else if number == 9000 {
        result.push_str("nine thousand");
    } else if number < 10000 {
        result.push_str("nine thousand ");
        diff = number - 9000;
        result.push_str(&below1000( diff ));
    }
    result
}