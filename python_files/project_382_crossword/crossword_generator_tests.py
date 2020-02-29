import unittest

import project_382_crossword.crossword_generator as cg


class TestCrosswordGenerator(unittest.TestCase):

    def test_challenge_example(self):
        dim_len = 15
        across_words = [1, 4, 7, 10, 13, 14, 16, 17, 18, 20, 21, 23, 24, 26, 28, 29, 33, 35, 36, 38, 39, 42, 44, 45, 47,
                        49, 50, 52, 55, 56, 58, 59, 61, 63, 67, 69, 70, 71, 72, 73, 74, 75, 76]
        down_words = [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 19, 22, 25, 27, 29, 30, 31, 32, 34, 37, 40, 41, 43, 46, 48,
                      51, 53, 54, 57, 60, 62, 64, 65, 66, 68]
        """
        
        . . . # # # . . . # . . . # #
        . . . . . # . . . # . . . . #
        . . . . . # . . . # . . . . .
        . . . . . # . . . . # . . . .
        # # # . . . # . . . . # . . .
        . . . . . . . # . . . . . . .
        . . . # # . . . # . . . . . .
        . . . . . # . . . # . . . . . 
        . . . . . . # . . . # # . . .
        . . . . . . . # . . . . . . .
        . . . # . . . . # . . . # # # 
        . . . . # . . . . # . . . . .
        . . . . . # . . . # . . . . .
        # . . . . # . . . # . . . . .  
        # # . . . # . . . # # # . . .
        
        """
        expected_crossword = [
            [True, True, True, False, False, False, True, True, True, False, True, True, True, False, False],
            [True, True, True, True, True, False, True, True, True, False, True, True, True, True, False],
            [True, True, True, True, True, False, True, True, True, False, True, True, True, True, True],
            [True, True, True, True, True, False, True, True, True, True, False, True, True, True, True],
            [False, False, False, True, True, True, False, True, True, True, True, False, True, True, True],
            [True, True, True, True, True, True, True, False, True, True, True, True, True, True, True],
            [True, True, True, False, False, True, True, True, False, True, True, True, True, True, True],
            [True, True, True, True, True, False, True, True, True, False, True, True, True, True, True],
            [True, True, True, True, True, True, False, True, True, True, False, False, True, True, True],
            [True, True, True, True, True, True, True, False, True, True, True, True, True, True, True],
            [True, True, True, False, True, True, True, True, False, True, True, True, False, False, False],
            [True, True, True, True, False, True, True, True, True, False, True, True, True, True, True],
            [True, True, True, True, True, False, True, True, True, False, True, True, True, True, True],
            [False, True, True, True, True, False, True, True, True, False, True, True, True, True, True],
            [False, False, True, True, True, False, True, True, True, False, False, False, True, True, True]

        ]
        self.assertTrue(cg.validate_crossword(expected_crossword, dim_len))

    def test_invalidates_one_eighty_symmetry(self):
        dim_len = 9
        """
            # . . . . . . . #      
            # . . . . . . . #      
            # . . . . . . . #      
            . . . # # # . . .      
            . . . # # # . . .      
            . . . # # # . . .      
            . . . . . . . . .      
            # . . . . . . . #      
            # # . . . . . # #   
        """
        invalid_cross = [
            [False, True, True, True, True, True, True, True, False],
            [False, True, True, True, True, True, True, True, False],
            [False, True, True, True, True, True, True, True, False],
            [True, True, True, False, False, False, True, True, True],
            [True, True, True, False, False, False, True, True, True],
            [True, True, True, False, False, False, True, True, True],
            [True, True, True, True, True, True, True, True, True],
            [False, True, True, True, True, True, True, True, False],
            [False, False, True, True, True, True, True, False, False],
        ]

        self.assertTrue(cg.validate_cross_len(invalid_cross, dim_len))
        self.assertFalse(cg.validate_crossword(invalid_cross, dim_len))

    def test_invalidates_disconnection(self):
        dim_len = 9
        """
            # # . . . . # # #      
            # . . . . . # # #      
            . . . . . . # # #      
            . . . . . # . . .      
            . . . . # . . . .      
            . . . # . . . . .      
            # # # . . . . . .      
            # # # . . . . . #      
            # # # . . . . # #  
        """
        invalid_cross = [
            [False, False, True, True, True, True, False, False, False],
            [False, True, True, True, True, True, False, False, False],
            [True, True, True, True, True, True, False, False, False],
            [True, True, True, True, True, False, True, True, True],
            [True, True, True, True, False, True, True, True, True],
            [True, True, True, False, True, True, True, True, True],
            [False, False, False, True, True, True, True, True, True],
            [False, False, False, True, True, True, True, True, False],
            [False, False, False, True, True, True, True, False, False],
        ]
        self.assertTrue(cg.validate_cross_len(invalid_cross, dim_len))
        self.assertTrue(cg.validate_cross_one_eighty_symmetry(invalid_cross, dim_len))
        self.assertFalse(cg.validate_crossword(invalid_cross, dim_len))


if __name__ == '__main__':
    unittest.main()
