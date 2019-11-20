

pub fn increment_digits(num: i32) -> i32 {
    let digits = get_digits(num);
    println!("{:?}", digits);
    let incremented_digits: Vec<i32> = digits.into_iter()
        .map(|digit| digit + 1).rev().collect();
    println!("Incremented digits {:?}", incremented_digits);
    let mut accum = 0;
    let mut placeholder = 1;
    for inc_digit in incremented_digits {
        accum += inc_digit * placeholder;
        if inc_digit == 10 {
            placeholder *= 100;
        }
        else {
            placeholder *= 10;
        }

    }
    return accum;
}

fn get_digits(num: i32) -> Vec<i32> {
    let largest_digit = get_largest_digit_place(num);
    return get_digit(num, largest_digit, vec![])
}

fn get_digit(mut num: i32, place_holder: usize, mut accum: Vec<i32>) -> Vec<i32>{

    if place_holder == 0 {
        return accum;
    }

    if num == 0 {
        accum.push(0);
        return get_digit(num, place_holder / 10, accum);
    }
    else if num > 0 {
        let mut num_digits_in_this_place = 0;
        while num - (place_holder as i32) >= 0 {
            num_digits_in_this_place += 1;
            num -= place_holder as i32;
        }
        accum.push(num_digits_in_this_place);
        return get_digit(num, place_holder / 10, accum);
    }
    else {
        //num is negative
        let mut num_digits_in_this_place = 0;
        while num + (place_holder as i32) <= 0 {
            num_digits_in_this_place -= 1;
            num += place_holder as i32;
        }
        accum.push(num_digits_in_this_place);
        return get_digit(num, place_holder /10, accum);
    }

}

fn get_largest_digit_place(num: i32) -> usize {
    if num == 0 {
        return 1;
    }
    else if num > 0 {
        let mut init_digit_place_holder = 1;
        while num - init_digit_place_holder >= 0 {
            init_digit_place_holder *= 10;
            println!("{}", init_digit_place_holder)
        }
        return (init_digit_place_holder / 10) as usize;
    }
    else {
        //num is negative
        let mut init_digit_place_holder = 1;
        while num + init_digit_place_holder <= 0 {
            init_digit_place_holder *= 10;
        }
        return (init_digit_place_holder / 10) as usize;
    }


}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_increment_digits() {
        assert_eq!(increment_digits(998), 10109);
        assert_eq!(increment_digits(0), 1);
        assert_eq!(increment_digits(-998), -887);
    }
    #[test]
    fn test_get_digits() {
        assert_eq!(get_digits(998), vec![9, 9, 8]);
        assert_eq!(get_digits(0), vec![0]);
        assert_eq!(get_digits(100), vec![1, 0, 0]);
        assert_eq!(get_digits(-998), vec![-9, -9, -8])
    }
    #[test]
    fn test_get_largest_digit_place() {
        assert_eq!(get_largest_digit_place(998), 100);
        assert_eq!(get_largest_digit_place(0), 1);
        assert_eq!(get_largest_digit_place(7), 1);
        assert_eq!(get_largest_digit_place(-998), 100);
        assert_eq!(get_largest_digit_place(100), 100);
    }

}