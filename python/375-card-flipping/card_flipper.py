from typing import Dict, List, Union, Optional, Tuple

Card = bool

Move = int

Game = List[Optional[Card]]


def is_face_up(card: Card) -> bool:
    return card


def can_flip(card: Card) -> bool:
    return is_face_up(card)


def make_move(game: Game, index: int) -> Optional[Game]:
    #  Flip card, then flip its neighbors if possible
    #  else return None
    game_after_flipping = flip_card(game, index)
    if game_after_flipping != game:
        # Then this was a valid move
        return flip_neighbors(game_after_flipping, index)
    else:
        return None

def flip_neighbors(game: Optional[Game], index: int) -> Optional[Game]:
    if game is None:
        return None
    neighbor_indexes = list(range(0, len(game)))
    flipped_indexes = [index] * len(game)
    return list(map(flip_neighbor, flipped_indexes, neighbor_indexes, game))


def should_flip_neighbor(flipped_index, neighbor_index) -> bool:
    return neighbor_index == flipped_index + 1 or neighbor_index == flipped_index - 1

def flip_neighbor(flipped_index, neighbor_index, neighbor_value) -> Optional[Card]:
    if neighbor_value is None:
        return None
    elif should_flip_neighbor(flipped_index, neighbor_index):
        return not neighbor_value

    return neighbor_value


def flip_card(game: Game, index: int) -> Optional[Game]:
    if index < 0:
        print("Invalid Index! Must be greater than 0.")
        return None
    elif index > len(game) - 1:
        print("Invalid Index! Must be less than the size of the game - 1")
        return None
    return flip_card_helper(game, index, 0)

def flip_card_helper(game: Game, index: int, new_index: int):
    if not game:
        return []
    else:
        card = game[0]
        if index == new_index and can_flip(card):
            return [None] + flip_card_helper(game[1:], index, new_index + 1)
        else:
            return [card] + flip_card_helper(game[1:], index, new_index + 1)


def find_all_moves(game: Game) -> List[Tuple[Move, Game]]:
    return find_all_moves_helper(game, 0)

def find_all_moves_helper(game: Game, move_counter: int):
    if move_counter == len(game):
        return []

    maybe_this_move = make_move(game, move_counter)
    if maybe_this_move is not None:
        valid_move = maybe_this_move
        return [(move_counter, valid_move)] + find_all_moves_helper(game, move_counter + 1)
    else:
        return find_all_moves_helper(game, move_counter + 1)



