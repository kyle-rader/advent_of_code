import pytest

from year_2015.day_07.circuit import Circuit

TEST_STATE = {
    'i': 1,
    'j': 4,
    'k': 5,
    'l': 16,
    'm': 32,
    'x': 123,
    'y': 456,
}


@pytest.mark.parametrize('expected, expression', [
    (0, "1 AND 2"),
    (5, "7 AND 5"),
    (72, "345  AND  234"),
    (3, "1 OR 2"),
    (15, "12 OR 3"),
    (10, "5 LSHIFT 1"),
    (20, "5 LSHIFT 2"),
    (40, "5 LSHIFT 3"),
    (5, "40 RSHIFT 3"),
    (4, "8 RSHIFT 1"),
    (4, "32 RSHIFT 3"),
    (4, "32 RSHIFT 3"),
    (4, "32 RSHIFT 3"),
    (65412, "NOT 123"),
    (72, "x AND y"),
    (507, "x OR y"),
    (492, "x LSHIFT 2"),
    (0, "m AND l"),
    (3, "i OR 2"),
    (65412, "NOT x"),
    (65079, "NOT y"),
    (123, "123"),
])
def test_circuit_eval(expected: int, expression: str):
    subject = Circuit()
    subject.state = TEST_STATE

    assert expected == subject.eval(expression)


@pytest.mark.parametrize("expected_signal_in_a, steps", [
    (72, "123 -> x \n 456 -> y \n x AND y -> a"),
    (507, "123 -> x \n 456 -> y \n x OR y -> a"),
    (65028, "123 -> x \n 456 -> y \n x OR y -> b \n NOT b -> a"),
    (63, "123 -> x \n 456 -> y \n x OR y -> b \n b RSHIFT 3 -> a"),
])
def test_circuit_execute_steps(expected_signal_in_a: int, steps: str):
    subject = Circuit(steps)
    assert expected_signal_in_a == subject['a']
