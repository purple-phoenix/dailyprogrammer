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

    def test_get_largest_smaller_blobs(self):
        smaller_blobs = [((1, 2, 3), 3), ((4, 8, 2), 3), ((7, 9, 4), 4), ((3, 5, 4), 1), ((2, 2, 4), 3)]
        largest_smaller_blobs_and_distances = [((7, 9, 4), 4), ((3, 5, 4), 1), ((2, 2, 4), 3)]
        self.assertEqual(largest_smaller_blobs_and_distances, get_largest_blobs(smaller_blobs))
        self.assertEqual([], get_largest_blobs([]))

    def test_blob_is_larger(self):
        self.assertTrue(blob_is_larger(((1, 2, 3), 4), ((1, 2, 2), 4)))
        self.assertFalse(blob_is_larger(((1, 2, 3), 4), ((1, 2, 3), 3)))

    def test_blob_is_same_size(self):
        self.assertFalse(blob_is_same_size(((1, 2, 3), 4), ((1, 2, 2), 4)))
        self.assertTrue(blob_is_same_size(((1, 2, 3), 4), ((1, 2, 3), 3)))

    def test_get_closest_smaller_blobs(self):
        smaller_blobs = [((1, 2, 3), 3), ((4, 8, 2), 3), ((7, 9, 4), 4), ((3, 5, 4), 1), ((2, 2, 4), 3)]

        closest_smaller_blobs = [((3, 5, 4), 1)]
        self.assertEqual(closest_smaller_blobs, get_closest_blobs(smaller_blobs))

    def test_blob_is_closer(self):
        self.assertTrue(blob_is_closer(((1, 2, 3), 1), ((2, 3, 4), 2)))
        self.assertFalse(blob_is_closer(((1, 2, 3), 1), ((2, 3, 4), 1)))
        self.assertFalse(blob_is_closer(((1, 2, 3), 2), ((2, 3, 4), 1)))

    def test_blob_is_same_distance(self):
        self.assertTrue(blob_is_same_distance(((1, 2, 3), 1), ((2, 3, 4), 1)))
        self.assertFalse(blob_is_same_distance(((1, 2, 3), 1), ((2, 3, 4), 2)))
        self.assertFalse(blob_is_same_distance(((1, 2, 3), 2), ((2, 3, 4), 1)))

    def test_clockwise_prioritization(self):
        comparison_blob = (1, 1, 1)
        twelve_o_clock = (0, 1, 1)
        one_o_clock = (0, 2, 1)
        six_o_clock = (2, 1, 1)
        nine_o_clock = (1, 0, 1)
        eleven_o_clock = (0, 0, 1)
        eight_o_clock = (2, 0, 1)
        blobs_to_move_towards = [eleven_o_clock,
                                 one_o_clock,
                                 eight_o_clock,
                                 twelve_o_clock,
                                 six_o_clock
                                 ]
        self.assertEqual(twelve_o_clock, clockwise_prioritization(comparison_blob, blobs_to_move_towards))
        blobs_to_move_towards_2 = [eleven_o_clock,
                                   one_o_clock,
                                   eight_o_clock,
                                   six_o_clock
                                   ]
        self.assertEqual(one_o_clock, clockwise_prioritization(comparison_blob, blobs_to_move_towards_2))
        blobs_to_move_towards_3 = [six_o_clock, nine_o_clock]
        self.assertEqual(six_o_clock, clockwise_prioritization(comparison_blob, blobs_to_move_towards_3))

    def test_more_clockwise(self):
        comparison_blob = (1, 1, 1)
        twelve_o_clock = (0, 1, 1)
        one_o_clock = (0, 2, 1)
        six_o_clock = (2, 1, 1)
        eight_o_clock = (2, 0, 1)
        eleven_o_clock = (0, 0, 1)
        five_o_clock = (2, 2, 1)
        self.assertTrue(more_clockwise(comparison_blob, twelve_o_clock, one_o_clock))
        self.assertTrue(more_clockwise(comparison_blob, one_o_clock, six_o_clock))
        self.assertFalse(more_clockwise(comparison_blob, eight_o_clock, six_o_clock))
        self.assertTrue(more_clockwise(comparison_blob, twelve_o_clock, eleven_o_clock))
        self.assertTrue(more_clockwise(comparison_blob, five_o_clock, eleven_o_clock))
        self.assertTrue(more_clockwise(comparison_blob, one_o_clock, five_o_clock))


if __name__ == '__main__':
    unittest.main()
