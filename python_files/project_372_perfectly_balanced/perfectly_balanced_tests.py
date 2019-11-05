import unittest
from project_372_perfectly_balanced.perfectly_balanced import *


class TestPerfectlyBalanced(unittest.TestCase):
    def test_balanced(self):
        self.assertTrue(balanced("xxxyyy"))
        self.assertTrue(balanced("yyyxxx"))
        self.assertFalse(balanced("xxxyyyy"))
        self.assertTrue(balanced("yyxyxxyxxyyyyxxxyxyx"))
        self.assertFalse(balanced("xyxxxxyyyxyxxyxxyy"))
        self.assertTrue(balanced(""))
        self.assertFalse(balanced("x"))

    def test_balanced_bonus(self):
        self.assertTrue(balanced_bonus("xxxyyyzzz"))
        self.assertTrue(balanced_bonus("abccbaabccba"))
        self.assertFalse(balanced_bonus("xxxyyyzzzz"))
        self.assertTrue(balanced_bonus("abcdefghijklmnopqrstuvwxyz"))
        self.assertFalse(balanced_bonus("pqq"))
        self.assertFalse(balanced_bonus("fdedfdeffeddefeeeefddf"))
        self.assertTrue(balanced_bonus("www"))
        self.assertTrue(balanced_bonus("x"))
        self.assertTrue(balanced_bonus(""))

    def test_num_each_char(self):
        self.assertEqual({"a": 3, "b": 4, "y": 1}, num_each_char("abybbaba"))
        self.assertEqual({}, num_each_char(""))
        self.assertEqual({"a": 1}, num_each_char("a"))


if __name__ == '__main__':
    unittest.main()
