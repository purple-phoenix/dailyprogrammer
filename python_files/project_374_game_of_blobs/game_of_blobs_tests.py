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

    def test_update_observed_blobs(self):
        observed_blobs = [(1, 1, 1), (1, 2, 3), (3, 5, 7), (2, 2, 1), (5, 7, 8)]
        a_new_blob_1 = (1, 1, 3)
        self.assertEqual([(1, 1, 4), (1, 2, 3), (3, 5, 7), (2, 2, 1), (5, 7, 8)],
                         update_observed_blobs(a_new_blob_1, observed_blobs))
        a_new_blob_2 = (3, 5, 11)
        self.assertEqual([(1, 1, 4), (1, 2, 3), (3, 5, 18), (2, 2, 1), (5, 7, 8)],
                         update_observed_blobs(a_new_blob_2, observed_blobs))
        unseen_blob = (2, 3, 1)
        self.assertEqual([(1, 1, 4), (1, 2, 3), (2, 3, 1), (3, 5, 18), (2, 2, 1), (5, 7, 8)],
                         update_observed_blobs(unseen_blob, observed_blobs))

    def test_positions_equal(self):
        self.assertTrue(positions_equal((1, 1, 1), (1, 1, 4)))
        self.assertFalse(positions_equal((1, 9, 1), (1, 1, 1)))
        self.assertFalse(positions_equal((2, 1, 1), (3, 1, 1)))


if __name__ == '__main__':
    unittest.main()
