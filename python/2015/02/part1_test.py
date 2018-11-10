import pytest


def wrapping_paper_needed(text: str):
    dimensions = text.split()
    total = 0

    for dim in dimensions:
        (l, w, h) = map(int, dim.split('x'))
        s1 = l * w
        s2 = w * h
        s3 = h * l

        total += 2*(s1 + s2 + s3) + min(s1, s2, s3)


    return total


INPUT3 = """2x3x4
1x1x10
"""


@pytest.mark.parametrize("exp, text", [
    (58, "2x3x4"),
    (43, "1x1x10"),
    (101, INPUT3),
])
def test_wrapping_paper_needed(exp, text):
    assert exp == wrapping_paper_needed(text)
