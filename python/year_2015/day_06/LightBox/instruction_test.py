import pytest
from typing import Tuple, List

from .instruction import Instruction, _parse_number_list


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


@pytest.mark.parametrize("raw_instruction", [
    "this is not an instruction",
    "2,5,9 through 3,8.4",
    "turn on -1,5 through 5, 10",
    "turn off 1, -5 through 5,10",
    "toggle",
])
def test_unparsable_instruction_raises_value_error(raw_instruction: str):
    with pytest.raises(ValueError) as ex_info:
        Instruction.parse(raw_instruction)
    assert 'could not be parsed' in str(ex_info.value)


@pytest.mark.parametrize("raw_instruction", [
    "turn off 1,5 through 5,-10",
    "turn off 0 through 4, 5",
    "toggle 45, 35 through 9 ",
])
def test_instruction_must_have_valid_dimensions(raw_instruction: str):
    with pytest.raises(ValueError) as ex_info:
        Instruction.parse(raw_instruction)
    assert 'must have valid coordinate pair!' in str(ex_info.value)


@pytest.mark.parametrize("exp, numbers", [
    ([1, 2, 4], "1, 2, 4"),
    ([1, 2], "1, 2, "),
])
def test_parse_number_list(exp: List[int], numbers: str):
    assert exp == _parse_number_list(numbers)