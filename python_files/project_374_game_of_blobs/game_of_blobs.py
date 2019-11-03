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
    pass


def clockwise_prioritization(blob: Blob, unit_directions_to_largest_blobs: List[Blob]) -> Blob:
    pass


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
