from typing import List

import pytest

from .instruction import Instruction, _parse_number_list, _coord_to_string


@pytest.mark.parametrize("exp, raw_instruction", [
    (Instruction([489, 959], [759, 964], 'on'), "turn on 489,959 through 759,964"),
    (Instruction([427, 423], [929, 502], 'off'), "turn off 427,423 through 929,502"),
    (Instruction([0, 1, 1], [2, 5, 10], 'off'), "turn off 0,1,1 through 2,5, 10"),
    (Instruction([0, 1], [2, 5], 'toggle'), "toggle 0 , 1 through 2,5"),
    (Instruction([0, 0, 0, 0], [2, 5, 1, 9], 'toggle'), "toggle 0,0,0,0 through 2,5,1,9"),
    (Instruction([0, 0, 0], [2, 5, 1], 'toggle'), "  ToGgLe 0, 0 ,  0 Through 2 , 5, 1 "),
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


two_d_box = [
    '00',
    '01',
    '10',
    '11',
    '20',
    '21']

three_d_box = [
    '000',
    '001',
    '010',
    '011',
    '100',
    '101',
    '110',
    '111']


@pytest.mark.parametrize("exp, instruction", [
    (['0'], Instruction([0], [0], 'on')),
    (['0', '1', '2', '3'], Instruction([0], [3], 'on')),
    (two_d_box, Instruction([0, 0], [2, 1], 'on')),
    (two_d_box, Instruction([0, 1], [2, 0], 'on')),
    (three_d_box, Instruction([0, 0, 0], [1, 1, 1], 'on')),
    (three_d_box, Instruction([0, 1, 1], [1, 0, 0], 'on')),
])
def test_instruction_cells(exp: List[str], instruction: Instruction):
    result = list(instruction.lights())
    for cell in exp:
        assert str(cell) in result


def test_large_instruction_uniqueness():
    inst = Instruction([0,0], [99, 99], 'on')
    lights = list(inst.lights())
    light_set = set()
    for light in lights:
        if light not in light_set:
            light_set.add(light)
        else:
            assert light not in light_set

    assert 100 * 100 == len(lights)


@pytest.mark.parametrize("exp, coords", [
    ('123', [1, 2, 3]),
    ('002306', [0, 23, 6]),
    ('000', [0, 0, 0]),
    ('100', [1, 0, 0]),
    ('101', [1, 0, 1]),
    ('010100000', [10, 100, 0]),
])
def test_coord_to_string(exp, coords):
    assert exp == _coord_to_string(coords)


@pytest.mark.parametrize("coord1, coord2", [
    ([11, 1], [1, 11]),
    ([12, 21], [1, 221]),
    ([12, 21], [1, 221]),
])
def test_coord_to_string_unique(coord1: List[int], coord2: List[int]):
    assert _coord_to_string(coord1) != _coord_to_string(coord2)
