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
    pass


def merge_blobs(blobs: List[Blob]) -> List[Blob]:
    return merge_blobs(blobs, [])


def merge_blobs_helper(blobs: List[Blob], observed_blobs: List[Blob]) -> List[Blob]:
    if not blobs:
        return observed_blobs
    else:
        blob = blobs[0]
        unobserved_blobs = blobs[1:]
        updated_observed_blobs = update_observed_blobs(blob, observed_blobs)
        return merge_blobs_helper(unobserved_blobs, updated_observed_blobs)


def update_observed_blobs(new_blob: Blob, already_observed_blobs) -> List[Blob]:
    pass

