from typing import Dict, List, Union, Optional, Tuple, Callable

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


def find_winning_moves(game: Game) -> Optional[List[Move]]:
    print("Initial Game State")
    print(repr_game(game))
    return find_winning_moves_helper(game, [])


def find_winning_moves_helper(game: Game, moves_so_far: List[Move]):
    print("Moves so far")
    print(repr_moves(moves_so_far))
    print("Updated Game State")
    print(repr_game(game))
    if game_is_won(game):
        print("Won Game!\n\n\n")
        return moves_so_far

    elif is_unwinnable_game(game):
        print("Game unwinnable!")
        return None

    all_moves_with_games = find_all_moves(game)
    # No valid moves for this game state
    # Cannot win
    if not all_moves_with_games:
        return None
    valid_moves, new_games = list(zip(*all_moves_with_games))

    # Python equivalent of a lazy first in this map
    for x in filter(lambda result: result is not None,
                    (map(
                        lambda valid_move, new_game_state:
                        find_winning_moves_helper(new_game_state, moves_so_far + [valid_move]),
                        valid_moves, new_games
                    ))):
        return x


def game_is_won(game: Game):
    if not game:
        return True

    first_card = game[0]
    if first_card is not None:
        return False

    rest_of_game = game[1:]
    return game_is_won(rest_of_game)


def is_unwinnable_game(game: Game):
    return is_unwinnable_game_helper(game, [])


def is_unwinnable_game_helper(game: Game, seen_cards: Game):
    # An empty game is considered won
    if not game:
        return False
    # Game is winnable as long as there is an odd number of face up cards
    # on both sides of every None
    elif num_none(game) == 0:
        return num_face_up(game) % 2 == 0
    elif len(game) == 1:
        # If a game is a single card then it is only
        # winnable if face up. So it is unwinnable if face down i.e not face up
        first_card = game[0]
        return not is_face_up(first_card)
    elif len(game) == 2:
        # If a game is two cards, then it is unwinnable if
        # they are either both face up or both face down
        first_card = game[0]
        second_card = game[1]
        return first_card == second_card
    else:
        # In the general case we attempt to divide and conquer
        # when a card is None, the cards to the left and right can be
        # thought of as separate games. This game is winnable if and only if
        # the cards to left and the right are winnable in isolation since they are divided by
        # a card which has already been taken
        first_card = game[0]
        if first_card is None:
            return is_unwinnable_game(seen_cards) or is_unwinnable_game(game[1:])
        else:
            return is_unwinnable_game_helper(game[1:], seen_cards + [first_card])


def num_face_up(game: Game):
    return count_game_condition(game, lambda card: is_face_up(card))


def num_none(game: Game):
    return count_game_condition(game, lambda card: card is None)


def count_game_condition(game: Game, condition: Callable[[Card], bool]) -> int:
    if not game:
        return 0
    else:
        card = game[0]
        if condition(card):
            return 1 + count_game_condition(game[1:], condition)
        else:
            return count_game_condition(game[1:], condition)


def repr_game(game: Game):
    if not game:
        return ""
    card = game[0]
    return repr_card(card) + repr_game(game[1:])


def repr_card(card: Card):
    if card is None:
        return "*"
    elif is_face_up(card):
        return "1"
    else:
        return "0"


def repr_moves(moves: List[Move]):
    if not moves:
        return ""
    move = moves[0]
    rest_of_moves = moves[1:]
    if not rest_of_moves:
        return str(move)
    else:
        return str(move) + " " + repr_moves(rest_of_moves)
