import pytest


def SOLUTION_NAME(text):
    return None


@pytest.mark.parametrize("exp, text", [
    (0, ""),
])
def test_SOLUTION_NAME(exp, text):
    assert exp == SOLUTION_NAME(text)
