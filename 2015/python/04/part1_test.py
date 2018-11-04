import pytest
import hashlib


def advent_coin(text):
    number = 0
    found = False
    while not found:
        number += 1
        hash: str = hashlib.md5(f"{text}{number}".encode('utf-8')).hexdigest()
        found = hash.startswith("00000")
    return number


@pytest.mark.parametrize("exp, text", [
    (609043, "abcdef"),
    (1048970, "pqrstuv"),
])
def test_advent_coin(exp, text):
    assert exp == advent_coin(text)
