from typing import Tuple, List
from random import randint
from functools import reduce
from operator import add

# Number of die and number of sides per die
DiceRoll = Tuple[int, int]


DiceInput = List[DiceRoll]


def roll_dice(dice_input: DiceInput) -> List[int]:
    return list(map(roll_die, dice_input))


def roll_die(dice_roll: DiceRoll) -> int:
    num_dice = dice_roll[0]
    num_sides = dice_roll[1]
    list_of_die_values = num_dice * [randint(1, num_sides)]
    return sum(list_of_die_values)


def convert_input_str_to_dice_input(input_str: List[str]) -> DiceInput:
    return list(map(lambda a_input_str: split_str_by_to_int(a_input_str, 'd'), input_str))


# Splits by first instance of splitter
def split_str_by_to_int(input_str: str, splitting_char: str) -> Tuple[str, str]:
    split_str = input_str.split(splitting_char)
    return int(split_str[0]), int("".join(split_str[1:]))
