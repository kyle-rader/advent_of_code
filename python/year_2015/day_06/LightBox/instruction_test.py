import pytest
from typing import Tuple, List

from .instruction import Instruction


@pytest.mark.parametrize("exp, raw_instruction", [
    (Instruction([489, 959], [759, 964], 'on'), "turn on 489,959 through 759,964"),
    (Instruction([427, 423], [929, 502], 'off'), "turn off 427,423 through 929,502"),
    (Instruction([0, 1, 1], [2, 5, 10], 'off'), "turn off 0,1,1 through 2,5, 10"),
    (Instruction([0,1], [2,5], 'toggle'), "toggle 0 , 1 through 2,5"),
    (Instruction([0,0,0,0], [2,5,1,9], 'toggle'), "toggle 0,0,0,0 through 2,5,1,9"),
    (Instruction([0,0,0], [2,5,1], 'toggle'), "  ToGgLe 0, 0 ,  0 Through 2 , 5, 1 "),
])
def test_parse_instruction(exp: Instruction, raw_instruction: str):
    result = Instruction.parse(raw_instruction)
    assert exp == result
