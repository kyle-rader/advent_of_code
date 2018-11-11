import pytest


def is_nice(text: str):
    text = text.lower()
    last_pair = None
    pairs = set()
    found_sandwich = False
    found_pair = False

    for i, c in enumerate(text):
        if i > 1:
            found_sandwich |= c == text[i-2]

        if i < len(text)-1:
            pair = f"{c}{text[i+1]}"
            if pair in pairs and pair != last_pair:
                found_pair = True
            last_pair = pair
            pairs.add(last_pair)

        if found_pair and found_sandwich:
            return True

    return found_pair and found_sandwich


@pytest.mark.parametrize("exp, text", [
    (True, "qjhvhtzxzqqjkmpb"),
    (True, "xxyxx"),
    (False, "uurcxstgmygtbstg"),
    (False, "ieodomkazucvgmuy"),
    (False, "dvszwmarrgswjxmb"),
])
def test_is_nice(exp, text):
    assert exp == is_nice(text)
