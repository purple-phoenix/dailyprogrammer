import unittest
from card_flipper import *


class TestCardFlipper(unittest.TestCase):

    def setUp(self):
        self.face_up_card = Card(True)
        self.face_down_card = Card(False)

        self.example_game = [
            self.face_down_card,
            self.face_up_card,
            self.face_down_card,
            self.face_down_card,
            self.face_up_card,
            self.face_up_card,
            self.face_down_card
        ]

        self.example_game_flipped_at_one = [
            self.face_down_card,
            None,
            self.face_down_card,
            self.face_down_card,
            self.face_up_card,
            self.face_up_card,
            self.face_down_card
        ]

        self.example_game_flipped_at_one_flipped_neighbors = [
            self.face_up_card,
            None,
            self.face_up_card,
            self.face_down_card,
            self.face_up_card,
            self.face_up_card,
            self.face_down_card
        ]

    def test_is_face_up(self):
        self.assertTrue(is_face_up(self.face_up_card))
        self.assertFalse(is_face_up(self.face_down_card))

    def test_flip_card(self):
        self.assertEquals(None, flip_card([], -1))
        self.assertEquals(None, flip_card([], 1))
        self.assertEquals(self.example_game_flipped_at_one,
                          flip_card(self.example_game, 1))
        self.assertEquals(self.example_game, flip_card(self.example_game, 0))

    def test_flip_neighbors(self):
        self.assertEquals(self.example_game_flipped_at_one_flipped_neighbors,
                          flip_neighbors(self.example_game_flipped_at_one, 1)
                          )
        self.assertEquals([None, True, None], flip_neighbors([None, True, None], 1))
        self.assertEquals([None, False, True], flip_neighbors([None, False, False], 1))
        self.assertEquals(None, flip_neighbors(None, 0))

    def test_make_move(self):
        self.assertEquals(self.example_game_flipped_at_one_flipped_neighbors,
                          make_move(self.example_game, 1))
        self.assertEquals(None, make_move([False, True, False], 0))


    def test_should_flip_neighbor(self):
        self.assertTrue(should_flip_neighbor(0, 1))
        self.assertTrue(should_flip_neighbor(1, 2))
        self.assertTrue(should_flip_neighbor(2, 1))
        self.assertFalse(should_flip_neighbor(0, 0))
        self.assertFalse(should_flip_neighbor(0, 2))

    def test_flip_neighbor(self):
        self.assertEquals(True, flip_neighbor(0, 2, True))
        self.assertEquals(None, flip_neighbor(0, 1, None))
        self.assertEquals(True, flip_neighbor(0, 1, False))
        self.assertEquals(False, flip_neighbor(1, 2, True))

    def test_find_all_moves(self):
        self.assertEquals([(0, [None, None, None])], find_all_moves([True, None, None]))
        self.assertEquals([(0, [None, False, True]), (1, [False, None, False]), (2, [True, False, None])],
                          find_all_moves([True, True, True]))
        self.assertEquals(
            [
                (0,[None, True, False, True, None, False, True]),
                (3,[True, False, True, None, None, False, True]),
                (6,[True, False, False, True, None, True, None])],
                          find_all_moves([True, False, False, True, None, False, True]))

    def test_find_winning_moves(self):
        self.assertEquals([1, 0, 2, 3, 5, 4, 6],
                          find_winning_moves([False, True, False, False, True, True, False])
                          )

        self.assertEquals(None,
                          find_winning_moves([False, True, False, False, True, True, False, False, True, True, True])
                          )

        self.assertEquals([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
                          find_winning_moves([True,
                                              False,
                                              False,
                                              False,
                                              False,
                                              True,
                                              True,
                                              False,
                                              False,
                                              True,
                                              False,
                                              True,
                                              False,
                                              False,
                                              False
                                              ])
                          )

    def test_game_is_won(self):
        self.assertTrue(game_is_won([]))
        self.assertTrue(game_is_won([None]))
        self.assertTrue(game_is_won([None, None, None]))
        self.assertFalse(game_is_won([None, False, True]))
        self.assertFalse(game_is_won([None, None, None, True, None]))
        self.assertFalse(game_is_won([None, False, None]))

    def test_is_unwinnable_game(self):
        self.assertFalse(is_unwinnable_game([]))

        self.assertTrue(is_unwinnable_game([False]))
        self.assertFalse(is_unwinnable_game([True]))

        self.assertFalse(is_unwinnable_game([True, False]))
        self.assertFalse(is_unwinnable_game([False, True]))
        self.assertTrue(is_unwinnable_game([False, False]))
        self.assertTrue(is_unwinnable_game([True, True]))

        self.assertTrue(is_unwinnable_game([False, True, True, None, False, False]))
        self.assertFalse(is_unwinnable_game([False, True, False, False, True, True, False]))

        self.assertTrue(is_unwinnable_game([False, True, False, False, True, True, False, False, True, True, True]))

        self.assertFalse(is_unwinnable_game([None, None, True, False, True, True, False]))
