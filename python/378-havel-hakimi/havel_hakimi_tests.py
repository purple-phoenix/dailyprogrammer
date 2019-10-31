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
