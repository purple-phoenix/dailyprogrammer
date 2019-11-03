import unittest
from project_374_game_of_blobs.game_of_blobs import *


class TestGameOfBlobs(unittest.TestCase):

    def test_blob_tick(self):
        initial_blobs = [(0, 2, 1), (2, 1, 2)]
        one_tick = [(0, 2, 1), (1, 1, 2)]
        two_tick = [(0, 2, 3)]
        self.assertEqual(one_tick, blob_tick(initial_blobs))
        self.assertEqual(two_tick, blob_tick(one_tick))

    def test_move_blob(self):
        initial_blobs = [(0, 2, 1), (2, 1, 2)]
        one_tick = [(0, 2, 1), (1, 1, 2)]
        two_tick = [(0, 2, 1), (0, 2, 2)]
        self.assertEqual(one_tick, move_blobs(initial_blobs))
        self.assertEqual(two_tick, move_blobs(one_tick))

    def test_merge_blobs(self):
        initial_blobs = [(0, 2, 1), (2, 1, 2)]
        one_tick = [(0, 2, 1), (1, 1, 2)]
        two_tick = [(0, 2, 3)]
        self.assertEqual(one_tick, merge_blobs(initial_blobs))
        self.assertEqual(two_tick, merge_blobs(one_tick))


if __name__ == '__main__':
    unittest.main()
