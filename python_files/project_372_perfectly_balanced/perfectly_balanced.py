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
    pass


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

