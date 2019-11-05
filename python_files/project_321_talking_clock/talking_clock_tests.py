import unittest
from project_321_talking_clock.talking_clock import *


class TestTalkingClock(unittest.TestCase):

    def test_make_clock_time(self):
        self.assertEqual((0, 0, 0, 0), make_clock_time("00:00"))
        self.assertEqual((0, 1, 3, 0), make_clock_time("01:30"))
        self.assertEqual((1, 2, 0, 5), make_clock_time("12:05"))
        self.assertEqual((1, 4, 0, 1), make_clock_time("14:01"))
        self.assertEqual((2, 0, 2, 9), make_clock_time("20:29"))
        self.assertEqual((2, 1, 0, 0), make_clock_time("21:00"))

    def test_clock_time_to_english(self):
        self.assertEqual("It's twelve am", make_clock_statement((0, 0, 0, 0)))
        self.assertEqual("It's one thirty am", make_clock_statement((0, 1, 3, 0)))
        self.assertEqual("It's twelve oh five pm", make_clock_statement((1, 2, 0, 5)))
        self.assertEqual("It's two oh one pm", make_clock_statement((1, 4, 0, 1)))
        self.assertEqual("It's eight twenty nine pm", make_clock_statement((2, 0, 2, 9)))
        self.assertEqual("It's nine pm", make_clock_statement((2, 1, 0, 0)))

    def test_clock_num_to_str(self):
        self.assertEqual("one", clock_num_to_str(1, False))
        self.assertEqual("twenty", clock_num_to_str(20, False))
        self.assertEqual("five", clock_num_to_str(5, False))
        self.assertEqual("thirty", clock_num_to_str(30, False))
        self.assertEqual("oh", clock_num_to_str(0, False))
        self.assertEqual("twelve", clock_num_to_str(0, True))


if __name__ == '__main__':
    unittest.main()
