
fn yahtzee_upper(five_dice_roll: Vec<usize>) -> usize {
    return 0;
}

// Counts the occurrences of each number one to six storing the value indexed at the number - 1
fn count_nums_one_to_six(five_dice_roll: Vec<usize>) -> Vec<usize> {
    let num_values = 6; // 1-6 inclusive
    // Initialize the count of each number 1-6 to zero
    let mut occurrences = vec![0; num_values];
    for a_roll_value in five_dice_roll {
        let value_index = a_roll_value - 1;
        let previous_count = occurrences.get(value_index).unwrap();
        occurrences[value_index] = previous_count + 1;
    }
    return occurrences;
}


#[cfg(test)]
mod tests {
    use crate::project_381_yahtzee::yahtzee::{yahtzee_upper, count_nums_one_to_six};

    #[test]
    fn test_yahtzee_upper() {
        assert_eq!(yahtzee_upper(vec![2, 3, 5, 5, 6]), 10);
        assert_eq!(yahtzee_upper(vec![1, 1, 1, 1, 3]), 4);
        assert_eq!(yahtzee_upper(vec![1, 1, 1, 3, 3]), 6);
        assert_eq!(yahtzee_upper(vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(yahtzee_upper(vec![6, 6, 6, 6, 6]), 30);
    }

    #[test]
    fn test_count_nums_one_to_six() {
        assert_eq!(count_nums_one_to_six(vec![2, 3, 5, 5, 6]), vec![0, 1, 1, 0, 2, 1]);
        assert_eq!(count_nums_one_to_six(vec![1, 1, 1, 1, 3]), vec![4, 0, 1, 0, 0, 0]);
        assert_eq!(count_nums_one_to_six(vec![1, 1, 1, 3, 3]), vec![3, 0, 2, 0, 0, 0]);
    }
}