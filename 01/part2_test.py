import pytest as pytest


def get_basement_step(directions: str):
    counter = 0

    for i in range(len(directions)):
        counter += 1 if directions[i] == '(' else -1

        if counter < 0:
            return i + 1


@pytest.mark.parametrize("expected, directions", [
    (1, ")"),
    (5, "()())"),
])
def test_get_basement_step(expected, directions):
    assert expected == get_basement_step(directions)
