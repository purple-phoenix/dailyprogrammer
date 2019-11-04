import unittest
from project_364_dice_roller.dice_roller import *


class TestDiceRoller(unittest.TestCase):

    def setUp(self) -> None:
        self.input = ["5d12", "6d4", "1d2", "1d8", "3d6", "4d20", "100d100"]
        self.dice_input = [(5, 12), (6, 4), (1, 2), (1, 8), (3, 6), (4, 20), (100, 100)]

    def test_convert_input(self):
        self.assertTrue(len(self.dice_input) == len(self.input))
        self.assertEqual(self.dice_input, convert_input_str_to_dice_input(self.input))

    def test_dice_roll(self):
        dice_output = roll_dice(self.dice_input)
        self.assertTrue(len(dice_output) == len(self.dice_input))

        for index, die_input in enumerate(self.dice_input):
            num_dice = die_input[0]
            num_sides = die_input[1]
            output = dice_output[index]
            self.assertTrue(1*num_dice <= output <= num_sides*num_dice)


if __name__ == '__main__':
    unittest.main()
