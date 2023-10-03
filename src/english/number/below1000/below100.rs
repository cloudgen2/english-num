pub fn below100(number: u32) -> String {
    let numbers = vec!["zero","one","two","three","four","five","six","seven","eight","nine","ten","eleven","twelve","thirteen","fourteen","fifteen","sixteen","seventeen","eighteen","nineteen","twenty"];
    let mut result: String;
    let diff: u32;
    if number < 21 {
        result=String::from(numbers[number as usize]);
    } else if number < 30 {
        result=String::from(numbers[20 as usize]);
        diff = number - 20;
        if diff == 1 {
            result.push_str("-");
        } else {
            result.push_str("-");
        }
        result.push_str(numbers[diff as usize]);
    } else if number < 40 {
        result=String::from("thirty");
        diff = number - 30;
        if diff > 0 {
            if diff == 1 {
                result.push_str("-");
            } else {
                result.push_str("-");
            }
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 50 {
        result=String::from("forty");
        diff = number - 40;
        if diff > 0 {
            if diff == 1 {
                result.push_str("-");
            } else {
                result.push_str("-");
            }
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 60 {
        result=String::from("fifty");
        diff = number - 50;
        if diff > 0 {
            if diff == 1 {
                result.push_str("-");
            } else {
                result.push_str("-");
            }
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 70 {
        result=String::from("sixty");
        diff = number - 60;
        if diff > 0 {
            if diff == 1 || diff == 11 {
                result.push_str("-");
            } else {
                result.push_str("-");
            }
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 80 {
        result=String::from("seventy");
        diff = number - 70;
        if diff > 0 {
            if diff == 1 || diff == 11 {
                result.push_str("-");
            } else {
                result.push_str("-");
            }
            result.push_str(numbers[diff as usize]);
        }
    } else if number < 90 {
        result=String::from("eighty");
        diff = number - 80;
        if diff > 0 {
            if diff == 1 || diff == 11 {
                result.push_str("-");
            } else {
                result.push_str("-");
            }
            result.push_str(numbers[diff as usize]);
        }
    } else {
        result=String::from("ninety");
        diff = number - 90;
        if diff > 0 {
            if diff == 1 || diff == 11 {
                result.push_str("-");
            } else {
                result.push_str("-");
            }
            result.push_str(numbers[diff as usize]);
        }
    }
    result
}