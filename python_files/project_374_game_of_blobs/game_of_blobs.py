from typing import Tuple, List, Optional, Callable

# Blobs are represented as a x position, y position and size
# Positions are zero indexed, Blobs of size zero represented empty spaces
Blob = Tuple[int, int, int]


# One tick of the game, blobs move, then
def blob_tick(blobs: List[Blob]) -> List[Blob]:
    unmerged_blobs = move_blobs(blobs)
    merged_blobs = merge_blobs(unmerged_blobs)
    return merged_blobs


def move_blobs(blobs: List[Blob]) -> List[Blob]:
    return list(map(
        lambda a_blob: move_blob_helper(a_blob, blobs),
        blobs))


def move_blob_helper(blob: Blob, all_blobs: List[Blob]) -> Blob:
    other_blobs = remove_this_blob(blob, all_blobs)
    return move_blob_toward_largest_smaller_blob(blob, other_blobs)


def remove_this_blob(blob: Blob, all_blobs: List[Blob]) -> List[Blob]:
    if not all_blobs:
        return []
    else:
        this_blob = all_blobs[0]
        other_blobs = all_blobs[1:]
        if blobs_equal(this_blob, blob):
            return other_blobs
        else:
            return [this_blob] + remove_this_blob(blob, other_blobs)


# Returns the moved blob. New blob is one unit closer to the largest blob. Ties going Clockwise
def move_blob_toward_largest_smaller_blob(blob: Blob, other_blobs: List[Blob]) -> Blob:
    smaller_blobs_with_distances = find_smaller_blobs(blob, other_blobs)
    closest_smaller_blobs = get_closest_blobs(smaller_blobs_with_distances)
    largest_closest_smaller_blobs = get_largest_blobs(closest_smaller_blobs)
    # Get just the Blobs no distances
    blobs_to_move_towards = list(map(lambda blob_and_distance: blob_and_distance[0], largest_closest_smaller_blobs))
    moved_blob = clockwise_prioritization(blob, blobs_to_move_towards)
    return moved_blob


def get_closest_blobs(smaller_blobs_and_distances: List[Tuple[Blob, int]]) -> List[Blob]:
    return get_conditionest_blobs_helper(blob_is_closer, blob_is_same_distance, smaller_blobs_and_distances, [])


def blob_is_closer(blob_and_distance: Tuple[Blob, int], current_best: Tuple[Blob, int]) -> bool:
    return blob_and_distance[1] < current_best[1]


def blob_is_same_distance(blob_and_distance: Tuple[Blob, int], current_best: Tuple[Blob, int]) -> bool:
    return blob_and_distance[1] == current_best[1]


def get_largest_blobs(smaller_blobs_and_distances: List[Tuple[Blob, int]]) -> List[Tuple[Blob, int]]:

    return get_conditionest_blobs_helper(blob_is_larger,
                                         blob_is_same_size,
                                         smaller_blobs_and_distances,
                                         [])


def blob_is_larger(blob_and_distance: Tuple[Blob, int], current_best: Tuple[Blob, int]) -> bool:
    return blob_and_distance[0][2] > current_best[0][2]


def blob_is_same_size(blob_and_distance: Tuple[Blob, int], current_best: Tuple[Blob, int]) -> bool:
    return blob_and_distance[0][2] == current_best[0][2]


# I admit this is not very Pythonic. But I didn't want to rewrite code once I discovered that the priority for movement
# was for closeness before largeness. So instead I abstracted out the separate conditions and made a generic function
# which both implementations would call. It's a bit obtuse, but technically more DRY. I'll leave comments
# best I can
def get_conditionest_blobs_helper(condition_for_replace: Callable[[Tuple[Blob, int], Tuple[Blob, int]], bool],
                                  condition_for_extension: Callable[[Tuple[Blob, int], Tuple[Blob, int]], bool],
                                  blobs_and_distances: List[Tuple[Blob, int]],
                                  conditionest_blobs_so_far: List[Tuple[Blob, int]]
                                  ) -> List[Tuple[Blob, int]]:
    # Base case is simple. If there are no more blobs and distances to check then we return what we have so far
    if not blobs_and_distances:
        return conditionest_blobs_so_far
    else:
        this_blob_and_distance = blobs_and_distances[0]
        rest_of_blobs = blobs_and_distances[1:]
        # If we have not checked any blobs then we pass this one to start the accumulator
        if not conditionest_blobs_so_far:
            return get_conditionest_blobs_helper(condition_for_replace,
                                                 condition_for_extension,
                                                 rest_of_blobs,
                                                 [this_blob_and_distance])
        else:
            # Get first in list then get size. All entries in this list will satisfy condition based on elif
            # That is all entries in conditionest_blobs_so_far satisfy the condition_for_extension therefore
            # we only need to check the first one against the abstract condition
            blob_and_distance_to_check_against = conditionest_blobs_so_far[0]
            if condition_for_replace(this_blob_and_distance, blob_and_distance_to_check_against):
                # If this one satisfies the condition for replacement, then we start over on the accumulation
                # If this one satisfies the condition for replacement, then we start over on the accumulation
                return get_conditionest_blobs_helper(condition_for_replace,
                                                     condition_for_extension,
                                                     rest_of_blobs,
                                                     [this_blob_and_distance])
            elif condition_for_extension(this_blob_and_distance, blob_and_distance_to_check_against):
                # Otherwise we check whether it satisfies the condition for extension, if so add to the accumulation
                return get_conditionest_blobs_helper(condition_for_replace,
                                                     condition_for_extension,
                                                     rest_of_blobs,
                                                     conditionest_blobs_so_far + [this_blob_and_distance])
            else:
                # Otherwise keep going with what we have so far through the rest of the unobserved list
                return get_conditionest_blobs_helper(condition_for_replace,
                                                     condition_for_extension,
                                                     rest_of_blobs,
                                                     conditionest_blobs_so_far)


def clockwise_prioritization(blob: Blob, potential_blobs_to_move_towards: List[Blob]) -> Blob:
    return clockwise_prioritization(blob, potential_blobs_to_move_towards, None)


def clockwise_prior_helper(blob: Blob, potential_blobs: List[Blob], most_clockwise_so_far) -> Blob:
    if most_clockwise_so_far is None and not potential_blobs:
        return blob # Don't move
    elif not potential_blobs:
        return most_clockwise_so_far # Found most clockwise
    else:
        first_potential_blob = potential_blobs[0]
        rest_potential_blobs = potential_blobs[1:]
        if most_clockwise_so_far is None:
            return clockwise_prior_helper(blob, rest_potential_blobs, first_potential_blob)
        elif more_clockwise(blob, first_potential_blob, most_clockwise_so_far):
            return clockwise_prior_helper(blob, rest_potential_blobs, first_potential_blob)
        else:
            return clockwise_prior_helper(blob, rest_potential_blobs, most_clockwise_so_far)


def more_clockwise(blob: Blob, pot_blob: Blob, current_best: Blob) -> bool:
    pot_x_diff = pot_blob[0] - blob[0]
    pot_y_diff = pot_blob[1] - blob[1]

    current_best_x_diff = current_best[0] - blob[0]
    current_best_y_diff = current_best[1] - blob[1]
    # If potential is at 12 then only not better if current best is too (which can't happen based on the game)
    if pot_y_diff == 0 and pot_x_diff < 0:
        return not (current_best_y_diff == 0 and current_best_x_diff < 0)
    # If not at 12 then check if pot y is greater than zero (right of six o'clock) and current best is not)
    elif pot_y_diff > 0 and current_best_x_diff < 0:
        return True
    elif pot_x_diff < 0:
        # Both are right of six, check if pot x is less than current best x(closer to 3)
        return pot_x_diff < current_best_x_diff
    else:
        # Both are left of six, check if post x is greater than current best x
        return pot_x_diff > current_best_x_diff

# Find smaller blobs and how far away they are from this blob
def find_smaller_blobs(blob: Blob, other_blobs: List[Blob]) -> List[Tuple[Blob, int]]:
    smaller_blobs = list(filter(lambda other_blob: other_blob[2] < blob[2], other_blobs))
    mapped_blobs_with_distances = list(map(lambda smaller_blob: (smaller_blob, get_blob_distance(smaller_blob, blob))
                                           , smaller_blobs))
    return mapped_blobs_with_distances


def merge_blobs(blobs: List[Blob]) -> List[Blob]:
    return merge_blobs_helper(blobs, [])


def merge_blobs_helper(blobs: List[Blob], observed_blobs: List[Blob]) -> List[Blob]:
    if not blobs:
        return observed_blobs
    else:
        blob = blobs[0]
        unobserved_blobs = blobs[1:]
        updated_observed_blobs = update_observed_blobs(blob, observed_blobs)
        return merge_blobs_helper(unobserved_blobs, updated_observed_blobs)


def update_observed_blobs(new_blob: Blob, already_observed_blobs) -> List[Blob]:
    if not already_observed_blobs:
        return [new_blob]
    observed_blob = already_observed_blobs[0]
    rest_of_observed_blobs = already_observed_blobs[1:]
    if positions_equal(new_blob, observed_blob):
        merged_blob = merge_two_blobs(new_blob, observed_blob)
        return update_observed_blobs(merged_blob, rest_of_observed_blobs)
    else:
        return [observed_blob] + update_observed_blobs(new_blob, rest_of_observed_blobs)


def positions_equal(blob_a: Blob, blob_b: Blob) -> bool:
    return blob_a[0] == blob_b[0] and blob_a[1] == blob_b[1]


def merge_two_blobs(blob_a: Blob, blob_b: Blob) -> Blob:
    return blob_a[0], blob_b[1], (blob_a[2] + blob_b[2])


def blobs_equal(blob_a: Blob, blob_b: Blob) -> bool:
    return positions_equal(blob_a, blob_b) and blob_a[2] == blob_b[2]


def get_blob_distance(blob_a: Blob, blob_b: Blob) -> int:
    x_distance = abs(blob_a[0] - blob_b[0])
    y_distance = abs(blob_a[1] - blob_b[1])
    return max(x_distance, y_distance)
