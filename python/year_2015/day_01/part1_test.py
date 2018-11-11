import pytest as pytest


def get_floor(directions: str):
    counter = 0
    for char in directions:
        counter += 1 if char == '(' else -1
    return counter


@pytest.mark.parametrize("expected, directions", [
    (0, "(())"),
    (-2, "(())))"),
    (3, "))((((("),
    (3, "(()(()("),
])
def test_get_floor(expected, directions):
    assert expected == get_floor(directions)
