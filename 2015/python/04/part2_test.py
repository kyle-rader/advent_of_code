import pytest
import hashlib


def advent_coin(text):
    number: int = 0
    found: bool = False
    while not found:
        number += 1
        hash: str = hashlib.md5(f"{text}{number}".encode('utf-8')).hexdigest()
        found = hash.startswith("000000")
    return number


@pytest.mark.parametrize("exp, text", [
])
def test_advent_coin(exp, text):
    assert exp == advent_coin(text)
