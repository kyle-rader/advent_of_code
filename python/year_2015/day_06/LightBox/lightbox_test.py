from typing import List
import pytest

from year_2015.day_06.LightBox import Instruction
from year_2015.day_06.LightBox import follow_instructions

instruction_set1 = [
    Instruction([0, 0], [1, 1], 'on'),
    Instruction([0, 0], [0, 1], 'toggle'),
]

instruction_set2 = [
    Instruction([0, 0], [0, 9], 'on'),
    Instruction([0, 0], [2, 9], 'toggle'),
]


@pytest.mark.parametrize("exp, instructions", [
    (2, instruction_set1),
    (20, instruction_set2),
])
def test_follow_instructions_set1(exp, instructions):
    assert exp == follow_instructions(instructions)
