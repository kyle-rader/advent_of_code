import pytest


class Position:
    def __init__(self, x:int, y:int):
        self.x = x
        self.y = y

    def move(self, direction: str):
        if direction == ">":
            self.x += 1
        elif direction == "<":
            self.x -= 1
        elif direction == "^":
            self.y += 1
        elif direction == "v":
            self.y -= 1
        else:
            raise Exception(f"Unknown direction: {direction}")

    def key(self):
        return f"{self.x},{self.y}"


def house_count(text):
    santa = Position(0, 0)
    robo_santa = Position(0, 0)
    houses = set()
    houses.add(santa.key())

    for i, direction in enumerate(text):
        if i % 2 == 0:
            santa.move(direction)
            houses.add(santa.key())
        else:
            robo_santa.move(direction)
            houses.add(robo_santa.key())

    return len(houses)


@pytest.mark.parametrize("exp, text", [
    (3, "^v"),
    (3, "^>v<"),
    (11, "^v^v^v^v^v"),
])
def test_house_count(exp, text):
    assert exp == house_count(text)
