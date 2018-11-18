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


@pytest.mark.parametrize("exp_lights_on, exp_brightness, instructions", [
    (4, 8, instruction_set1),
    (30, 70, instruction_set2),
])
def test_follow_instructions_set1(exp_lights_on, exp_brightness, instructions):
    (lights_on, brightness) = follow_instructions(instructions)
    assert exp_lights_on == lights_on
    assert exp_brightness == brightness
