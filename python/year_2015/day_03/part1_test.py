import pytest


def _house_key(x:int, y:int):
    return f"{x},{y}"


def house_count(text):
    x = 0
    y = 0
    houses = set()
    houses.add(_house_key(x, y))
    for dir in text:
        if dir == ">":
            x += 1
        elif dir == "<":
            x -= 1
        elif dir == "^":
            y += 1
        elif dir == "v":
            y -= 1
        else:
            raise Exception("Unknown direction")

        houses.add(_house_key(x, y))

    return len(houses)


@pytest.mark.parametrize("exp, text", [
    (2, ">"),
    (4, "^>v<"),
    (2, "^v^v^v^v^v"),
    (3, ">v"),
])
def test_house_count(exp, text):
    assert exp == house_count(text)
