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
        no_merge = [(0, 2, 1), (2, 1, 2)]
        before_merge = [(0, 2, 1), (0, 2, 2)]
        after_merge = [(0, 2, 3)]
        self.assertEqual(set(no_merge), set(merge_blobs(no_merge)))
        self.assertEqual(set(after_merge), set(merge_blobs(before_merge)))

    def test_update_observed_blobs(self):
        observed_blobs = [(1, 1, 1), (1, 2, 3), (3, 5, 7), (2, 2, 1), (5, 7, 8)]
        a_new_blob_1 = (1, 1, 3)
        self.assertEqual(set([(1, 1, 4), (1, 2, 3), (3, 5, 7), (2, 2, 1), (5, 7, 8)]),
                         set(update_observed_blobs(a_new_blob_1, observed_blobs)))
        a_new_blob_2 = (3, 5, 11)
        self.assertEqual(set([(1, 1, 1), (1, 2, 3), (3, 5, 18), (2, 2, 1), (5, 7, 8)]),
                         set(update_observed_blobs(a_new_blob_2, observed_blobs)))
        unseen_blob = (2, 3, 1)
        self.assertEqual(set([(1, 1, 1), (1, 2, 3), (2, 3, 1), (3, 5, 7), (2, 2, 1), (5, 7, 8)]),
                         set(update_observed_blobs(unseen_blob, observed_blobs)))

    def test_positions_equal(self):
        self.assertTrue(positions_equal((1, 1, 1), (1, 1, 4)))
        self.assertFalse(positions_equal((1, 9, 1), (1, 1, 1)))
        self.assertFalse(positions_equal((2, 1, 1), (3, 1, 1)))

    def test_merge_two_blobs(self):
        self.assertEqual((1, 1, 7), merge_two_blobs((1, 1, 3), (1, 1, 4)))

    def test_remove_this_blob(self):
        self.assertEqual([(1, 1, 1), (1, 2, 2)], remove_this_blob((1, 2, 3),
                                                                  [(1, 1, 1), (1, 2, 2), (1, 2, 3)]))

    def test_blobs_equal(self):
        self.assertTrue(blobs_equal((1, 1, 1), (1, 1, 1)))
        self.assertFalse(blobs_equal((1, 2, 1), (1, 1, 1)))
        self.assertFalse(blobs_equal((1, 1, 1), (1, 1, 2)))

    def test_find_smaller_blobs(self):
        many_blobs = [
            (1, 2, 3),
            (2, 5, 6),
            (4, 8, 2),
            (7, 9, 4),
            (3, 5, 4),
            (2, 2, 4)
        ]
        self.assertEqual(set([((1, 2, 3), 3), ((4, 8, 2), 3), ((7, 9, 4), 4), ((3, 5, 4), 1), ((2, 2, 4), 3)]),
                         set(find_smaller_blobs((4, 5, 5), many_blobs))
                         )

    def test_get_blob_distance(self):
        comparison_blob = (4, 5, 5)
        self.assertEqual(3, get_blob_distance((1, 2, 3), comparison_blob))
        self.assertEqual(3, get_blob_distance((2, 2, 4), comparison_blob))
        self.assertEqual(2, get_blob_distance((2, 5, 6), comparison_blob))
        self.assertEqual(4, get_blob_distance((7, 9, 4), comparison_blob))


if __name__ == '__main__':
    unittest.main()
