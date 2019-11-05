from typing import Dict, List


def balanced(string: str) -> bool:
    char_dict = num_each_char(string)
    x_in_dict = "x" in char_dict
    y_in_dict = "y" in char_dict
    if not x_in_dict or not y_in_dict:
        # If either is missing, then they are balanced if both are missing
        return not x_in_dict and not y_in_dict
    else:
        num_x = char_dict["x"]
        num_y = char_dict["y"]
        return num_x == num_y


def balanced_bonus(string: str) -> bool:
    char_dict = num_each_char(string)
    return all_chars_equal(char_dict)


def all_chars_equal(char_dict: Dict[str, int]) -> bool:
    return all_chars_equal_helper([val for val in char_dict.values()])


def all_chars_equal_helper(char_nums: List[int]) -> bool:
    if not char_nums:
        return True
    else:
        first_num = char_nums[0]
        rest_of_nums = char_nums[1:]
        return nums_are_equal_to(rest_of_nums, first_num)


def nums_are_equal_to(nums: List[int], num: int) -> bool:
    if not nums:
        return True
    else:
        next_num = nums[0]
        if next_num != num:
            return False
        else:
            rest_of_nums = nums[1:]
            return nums_are_equal_to(rest_of_nums, num)


def num_each_char(string: str) -> Dict[str, int]:
    return num_each_char_helper([char for char in string],  {})


def num_each_char_helper(char_stream: List[str], accum: Dict[str, int]) -> Dict:
    if not char_stream:
        return accum
    else:
        this_char = char_stream[0]
        rest_chars = char_stream[1:]
        if this_char in accum:
            # Not pure functional but I think this is fine
            # it's about knowing when to break the rules
            current_count = accum[this_char]
            accum[this_char] = current_count + 1
            return num_each_char_helper(rest_chars, accum)
        else:
            # First instance of this character
            accum[this_char] = 1
            return num_each_char_helper(rest_chars, accum)

