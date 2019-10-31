import unittest
from havel_hakimi import *

class TestHavelHakimi(unittest.TestCase):

    def setUp(self):
        pass

    def test_warmup1(self):
        self.assertEquals([5,3,2,6,2,7,2,5],
            warmup1([5,3,0,2,6,2,0,7,2,5]))
        self.assertEquals([4,1,3],
                          warmup1([4, 0, 0, 1, 3]))
        self.assertEquals([1,2,3],
                          warmup1([1,2,3]))
        self.assertEquals([],
                          warmup1([0,0,0]))
        self.assertEquals([], warmup1([]))

    def test_warmup2(self):
        self.assertEquals([5,4,3,2,1], warmup2([5,1,3,4,2]))
        self.assertEquals([4,0,0,0,0], warmup2([0,0,0,4,0]))
        self.assertEquals([1], warmup2([1]))
        self.assertEquals([], warmup2([]))

    def test_warmup3(self):
        self.assertEquals(False, warmup3(7, [6,5,5,3,2,2,2]))
        self.assertEquals(False, warmup3(5, [5,5,5,5,5]))
        self.assertEquals(True, warmup3(5, [5,5,5,5]))
        self.assertEquals(True, warmup3(3, [1,1]))
        self.assertEquals(True, warmup3(1, []))
        self.assertEquals(False, warmup3(0, []))

    def test_warmup4(self):
        self.assertEquals([4,3,2,1,1], warmup4(4, [5,4,3,2,1]))
        self.assertEquals([13, 12, 12, 12, 11, 9, 7, 7, 6, 6, 5, 6, 4, 4, 2],
                          warmup4(11, [14, 13, 13, 13, 12, 10, 8, 8, 7, 7, 6, 6, 4, 4, 2]))
        self.assertEquals([9, 10, 10], warmup4(1, [10,10,10]))
        self.assertEquals([9,9,9], warmup4(3, [10,10,10]))
        self.assertEquals([0], warmup4(1, [1]))

    def test_havel_hakimi(self):
        self.assertFalse(havel_hakimi([5,3,0,2,6,0,7,2,5]))
        self.assertFalse(havel_hakimi([4,2,0,1,5,0]))
        self.assertTrue(havel_hakimi([3,1,2,3,1,0]))
        self.assertTrue(havel_hakimi(
            [16, 9, 9, 15, 9, 7, 9, 11, 17, 11, 4, 9, 12, 14, 14, 12, 17, 0, 3, 16]))
        self.assertFalse(havel_hakimi(
            [15, 18, 6, 13, 12, 4, 4, 14, 1, 6, 18, 2, 6, 16, 0, 9, 10, 7, 12, 3]))
        self.assertFalse(havel_hakimi(
            [6, 0, 10, 10, 10, 5, 8, 3, 0, 14, 16, 2, 13, 1, 2, 13, 6, 15, 5, 1]))
        self.assertFalse(havel_hakimi([2,2,0]))
        self.assertFalse(havel_hakimi([3,2,1]))
        self.assertTrue(havel_hakimi(1,1))
        self.assertFalse([1])
        self.assertTrue(havel_hakimi([]))
