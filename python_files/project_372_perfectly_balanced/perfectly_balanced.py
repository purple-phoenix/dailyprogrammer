from typing import Dict, List


def balanced(string: str) -> bool:
    pass


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

