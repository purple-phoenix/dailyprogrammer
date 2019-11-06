from typing import Tuple, List
from random import randint
from functools import reduce
from operator import add

# Number of die and number of sides per die
DiceRoll = Tuple[int, int]


DiceInput = List[DiceRoll]


def dice_roll(input: List[str]) -> List[int]:
    dice_input = convert_input_str_to_dice_input(input)
    return roll_dice(dice_input)


def roll_dice(dice_input: DiceInput) -> List[int]:
    return list(map(roll_die, dice_input))


def roll_die(dice_roll: DiceRoll) -> int:
    num_dice = dice_roll[0]
    num_sides = dice_roll[1]
    list_of_die_values = [randint(1, num_sides) for _ in num_dice]
    return sum(list_of_die_values)


def convert_input_str_to_dice_input(input_str: List[str]) -> DiceInput:
    return list(map(lambda a_input_str: split_str_by_to_int(a_input_str, 'd'), input_str))


# Splits by first instance of splitter
def split_str_by_to_int(input_str: str, splitting_char: str) -> Tuple[str, str]:
    split_str = input_str.split(splitting_char)
    return int(split_str[0]), int("".join(split_str[1:]))
