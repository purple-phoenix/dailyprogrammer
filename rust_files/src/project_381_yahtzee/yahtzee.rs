
fn yahtzee_upper(five_dice_roll: Vec<usize>) -> usize {
    return 0;
}


#[cfg(test)]
mod tests {
    use crate::project_381_yahtzee::yahtzee::yahtzee_upper;

    fn test_yahtzee_upper() {
        assert_eq!(yahtzee_upper(vec![2, 3, 5, 5, 6]), 10);
        assert_eq!(yahtzee_upper(vec![1, 1, 1, 1, 3]), 4);
        assert_eq!(yahtzee_upper(vec![1, 1, 1, 3, 3]), 6);
        assert_eq!(yahtzee_upper(vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(yahtzee_upper(vec![6, 6, 6, 6, 6]), 30);
    }
}