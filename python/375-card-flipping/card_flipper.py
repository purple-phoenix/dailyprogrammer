from typing import Dict, List, Union, Optional

Card = bool

Move = int

Game = List[Optional[Card]]


def is_face_up(card: Card) -> bool:
    return card


def can_flip(card: Card) -> bool:
    return is_face_up(card)


def make_move(game: Game, index: int) -> Optional[Game]:
    # Flip card
    # flip neighbors
    return None

def flip_neighbors(game: Optional[Game], index: int) -> Optional[Game]:
    if game is None:
        return None
    return list(map(flip_neighbor(), zip_game_with_index(index, game)))


def zip_game_with_index(index, game):
    incremental_indexes = list(range(0, len(game)))
    list_of_index = [index] * len(game)
    return list(zip(list_of_index, incremental_indexes, game))



def should_flip_neighbor(flipped_index, neighbor_index) -> bool:
    pass

def flip_neighbor(flipped_index, neighbor_index, neighbor_value) -> Optional[Card]:
    pass


def flip_card(game: Game, index: int) -> Optional[Game]:
    if index < 0:
        print("Invalid Index! Must be greater than 0.")
        return None
    elif index > len(game) - 1:
        print("Invalid Index! Must be less than the size of the game - 1")
        return None



