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
