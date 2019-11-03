from typing import Tuple, List

# Blobs are represented as a x position, y position and size
# Positions are zero indexed, Blobs of size zero represented empty spaces
Blob = Tuple[int, int, int]


# One tick of the game, blobs move, then
def blob_tick(blobs: List[Blob]) -> List[Blob]:
    unmerged_blobs = move_blobs(blobs)
    merged_blobs = merge_blobs(unmerged_blobs)
    return merged_blobs


def move_blobs(blobs: List[Blob]) -> List[Blob]:
    return list(map(lambda ablob: move_blob_helper(ablob, blobs)))


def move_blob_helper(blob: Blob, all_blobs: List[Blob]) -> Blob:
    other_blobs = remove_this_blob(blob, all_blobs)
    return find_unit_path_to_largest_blob(blob, other_blobs)


def remove_this_blob(blob: Blob, all_blobs: List[Blob]) -> List[Blob]:
    pass


# Returns the moved blob. New blob is one unit closer to the largest blob. Ties going Clockwise
def find_unit_path_to_largest_blob(blob: Blob, other_blobs: List[Blob]) -> Blob:
    pass


def clockwise_prioritization(blob: Blob, unit_directions_to_largest_blobs: List[Blob]) -> Blob:
    pass


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
