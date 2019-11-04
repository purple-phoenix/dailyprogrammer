from typing import Tuple, List


# Number of die and number of sides per die
DiceRoll = Tuple[int, int]


DiceInput = List[DiceRoll]


def roll_dice(dice_input: DiceInput) -> List[int]:
    pass


def convert_input_str_to_dice_input(input_str: List[str]) -> DiceInput:
    return list(map(lambda a_input_str: split_str_by_to_int(a_input_str, 'd'), input_str))


# Splits by first instance of splitter
def split_str_by_to_int(input_str: str, splitting_char: str) -> Tuple[str, str]:
    split_str = input_str.split(splitting_char)
    return int(split_str[0]), int("".join(split_str[1:]))