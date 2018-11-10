import pytest

VOWELS = {'a', 'e', 'i', 'o', 'u'}
BAD_PAIRS = {'ab', 'cd', 'pq', 'xy'}


def is_nice(text):
    last = None
    found_double = False
    vowels = 0

    for c in text.lower():
        if last is not None and f"{last}{c}" in BAD_PAIRS:
            return False
        found_double |= (c == last)
        vowels += 1 if c in VOWELS else 0
        last = c

    return vowels >= 3 and found_double


@pytest.mark.parametrize("exp, text", [
    (True, "ugknbfddgicrmopn"),
    (True, "aaa"),
    (True, "aaa"),
    (False, "jchzalrnumimnmhp"),
    (False, "haegwjzuvuyypxyu"),
    (False, "dvszwmarrgswjxmb"),
    (False, "aaeab"),
    (False, "aeiccd"),
    (False, "aeiccceeepqq"),
    (True, "eightintty"),
])
def test_is_nice(exp, text):
    assert exp == is_nice(text)
