mod below100;
use below100::below100;

pub fn below1000(number: u32) -> String {
    let mut result=String::new();
    let diff: u32;
    if number < 100 {
        result.push_str(&below100(number));
    } else if number == 100 {
        result.push_str("one hundred");
    } else if number < 200 {
        result.push_str("one hundred ");
        diff = number - 100;
        result.push_str(&below100(diff));
    } else if number == 200 {
        result.push_str("two hundreds");
    } else if number < 300 {
        result.push_str("two hundred ");
        diff = number - 200;
        result.push_str(&below100(diff));
    } else if number == 300 {
        result.push_str("three hundreds");
    } else if number < 400 {
        result.push_str("three hundred ");
        diff = number - 300;
        result.push_str(&below100(diff));
    } else if number == 400 {
        result.push_str("four hundreds");
    } else if number < 500 {
        result.push_str("four hundred ");
        diff = number - 400;
        result.push_str(&below100(diff));
    } else if number == 500 {
        result.push_str("five hundreds");
    } else if number < 600 {
        result.push_str("five hundred ");
        diff = number - 500;
        result.push_str(&below100(diff));
    } else if number == 600 {
        result.push_str("six hundreds");
    } else if number < 700 {
        result.push_str("six hundred ");
        diff = number - 600;
        result.push_str(&below100(diff));
    } else if number == 700 {
        result.push_str("seven hundreds");
    } else if number < 800 {
        result.push_str("seven hundred ");
        diff = number - 700;
        result.push_str(&below100(diff));
    } else if number == 800 {
        result.push_str("eight hundreds");
    } else if number < 900 {
        result.push_str("eight hundred ");
        diff = number - 800;
        result.push_str(&below100(diff));
    } else if number == 900 {
        result.push_str("nine hundreds");
    } else if number < 1000 {
        result.push_str("nine hundred ");
        diff = number - 900;
        result.push_str(&below100(diff));
    }
    result
}
