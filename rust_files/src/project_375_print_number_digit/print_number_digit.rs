

fn increment_digits(num: i32) -> i32 {
    return num;
}

fn get_digits(num: i32) -> Vec<usize> {
    return vec![]
}

fn get_largest_digit_place(num: i32) -> usize {
    if num == 0 {
        return 1;
    }
    else if num > 0 {
        let mut init_digit_place_holder = 1;
        while num - init_digit_place_holder > 0 {
            init_digit_place_holder *= 10;
            println!("{}", init_digit_place_holder)
        }
        return (init_digit_place_holder / 10) as usize;
    }
    else {
        //num is negative
        let mut init_digit_place_holder = 1;
        while num + init_digit_place_holder < 0 {
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


        //TODO
        //assert_eq!(increment_digits(-998), -887);
    }
    #[test]
    fn test_get_digits() {
        assert_eq!(get_digits(998), vec![9, 9, 8]);
        assert_eq!(get_digits(0), vec![0]);
        //TODO
        //assert_eq!(get_digits(-998), vec![-9, -9, -8])
    }
    #[test]
    fn test_get_largest_digit_place() {
        assert_eq!(get_largest_digit_place(998), 100);
        assert_eq!(get_largest_digit_place(0), 1);
        assert_eq!(get_largest_digit_place(7), 1);
        assert_eq!(get_largest_digit_place(-998), 100)//?
    }

}