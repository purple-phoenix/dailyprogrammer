use std::collections::HashMap;

pub fn yahtzee_upper(dice_roll: Vec<usize>) -> usize {
    let occurrences = count_nums_x_to_y(dice_roll);
    let mut current_best = 0; //Some result will be better than 0
    for occurrence in occurrences {
        let die_value = occurrence.0;
        let num_times = occurrence.1;
        let score = die_value * num_times;
        if score > current_best {
            current_best = score;
        }
    }
    return current_best;
}


fn count_nums_x_to_y(dice_roll: Vec<(usize)>) -> HashMap<usize, usize>{
    let mut occurrences: HashMap<usize, usize> = HashMap::new();
    for a_roll_value in dice_roll {
        // If entry exists increment, else initialize with zero and increment to one
        *occurrences.entry(a_roll_value).or_insert(0) += 1;
    }
    return occurrences;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test_yahtzee_upper() {
        assert_eq!(yahtzee_upper(vec![2, 3, 5, 5, 6]), 10);
        assert_eq!(yahtzee_upper(vec![1, 1, 1, 1, 3]), 4);
        assert_eq!(yahtzee_upper(vec![1, 1, 1, 3, 3]), 6);
        assert_eq!(yahtzee_upper(vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(yahtzee_upper(vec![6, 6, 6, 6, 6]), 30);
        assert_eq!(yahtzee_upper(vec![1654, 1654, 50995, 30864, 1654, 50995, 22747,
                                      1654, 1654, 1654, 1654, 1654, 30864, 4868, 1654, 4868, 1654,
                                      30864, 4868, 30864]),
                   123456
                                      )
    }

    #[test]
    fn test_count_nums_x_to_y() {
        let mut map1 = HashMap::new();
        map1.insert(2, 1);
        map1.insert(3, 1);
        map1.insert(5, 2);
        map1.insert(6, 1);
        assert_eq!(count_nums_x_to_y(vec![2, 3, 5, 5, 6]), map1);
        let mut map2 = HashMap::new();
        map2.insert(1, 4);
        map2.insert(3, 1);
        assert_eq!(count_nums_x_to_y(vec![1, 1, 1, 1, 3]), map2);
        let mut map3 = HashMap::new();
        map3.insert(1, 3);
        map3.insert(3, 2);
        assert_eq!(count_nums_x_to_y(vec![1, 1, 1, 3, 3]), map3);
    }
}