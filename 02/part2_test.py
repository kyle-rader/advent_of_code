import pytest


def ribbon_needed(text: str):
    dimensions = text.split()
    total = 0

    for dim in dimensions:
        (l, w, h) = sorted(map(int, dim.split('x')))

        total += 2*(l + w) + l*w*h

    return total


INPUT3 = """2x3x4
1x1x10
"""


@pytest.mark.parametrize("exp, text", [
    (34, "2x3x4"),
    (14, "1x1x10"),
    (48, INPUT3),
])
def test_ribbon_needed(exp, text):
    assert exp == ribbon_needed(text)
