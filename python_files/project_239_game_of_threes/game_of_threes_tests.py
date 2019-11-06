import unittest
from project_239_game_of_threes.game_of_threes import *


class TestGameOfThrees(unittest.TestCase):

    def test_divisible_by_three(self):
        self.assertTrue(divisible_by_three(33))
        self.assertFalse(divisible_by_three(1))
        self.assertFalse(divisible_by_three(2))
        self.assertFalse(divisible_by_three(53))
        self.assertTrue(divisible_by_three(42))

    def test_add_one_divisible_by_three(self):
        self.assertTrue(add_one_divisible_by_three(32))
        self.assertFalse(add_one_divisible_by_three(1))
        self.assertTrue(add_one_divisible_by_three(2))
        self.assertFalse(add_one_divisible_by_three(42))
        self.assertFalse(add_one_divisible_by_three(40))

    def test_sub_one_divisible_by_three(self):
        self.assertFalse(sub_one_divisible_by_three(32))
        self.assertTrue(sub_one_divisible_by_three(31))
        self.assertTrue(sub_one_divisible_by_three(34))
        self.assertFalse(sub_one_divisible_by_three(30))

    def test_game_of_threes(self):
        # Should terminate
        game_of_threes(31337357)


if __name__ == '__main__':
    unittest.main()
