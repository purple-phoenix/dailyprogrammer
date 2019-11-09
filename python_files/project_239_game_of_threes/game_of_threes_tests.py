import unittest
from project_239_game_of_threes.game_of_threes import *

# Private functions are not imported with the wildcard import.
# Explicitly import private functions
#  https://stackoverflow.com/questions/31519815/import-symbols-starting-with-underscore
from project_239_game_of_threes.game_of_threes import _divisible_by_three
from project_239_game_of_threes.game_of_threes import _add_one_divisible_by_three
from project_239_game_of_threes.game_of_threes import _sub_one_divisible_by_three


class TestGameOfThrees(unittest.TestCase):

    def test__divisible_by_three(self):
        self.assertTrue(_divisible_by_three(33))
        self.assertFalse(_divisible_by_three(1))
        self.assertFalse(_divisible_by_three(2))
        self.assertFalse(_divisible_by_three(53))
        self.assertTrue(_divisible_by_three(42))

    def test__add_one_divisible_by_three(self):
        self.assertTrue(_add_one_divisible_by_three(32))
        self.assertFalse(_add_one_divisible_by_three(1))
        self.assertTrue(_add_one_divisible_by_three(2))
        self.assertFalse(_add_one_divisible_by_three(42))
        self.assertFalse(_add_one_divisible_by_three(40))

    def test__sub_one_divisible_by_three(self):
        self.assertFalse(_sub_one_divisible_by_three(32))
        self.assertTrue(_sub_one_divisible_by_three(31))
        self.assertTrue(_sub_one_divisible_by_three(34))
        self.assertFalse(_sub_one_divisible_by_three(30))

    def test_invalidInput(self):
        # 0 is invalid and should return False
        self.assertFalse(game_of_threes(0))

    def test_game_of_threes(self):
        # Should terminate
        self.assertTrue(game_of_threes(31337357))

    def test_game_of_threes_simple(self):
        self.assertTrue(game_of_threes(30))


if __name__ == '__main__':
    unittest.main()