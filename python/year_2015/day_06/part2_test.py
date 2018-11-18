from year_2015.day_06.LightBox import Instruction, follow_instructions


inst1 = """
turn on 0,0 through 99,99
turn off 0,0 through 0,0
toggle 99,99 through 99,99
turn on 49,49 through 50,50
"""

def test_part2():
    inst = [Instruction.parse(l.strip()) for l in inst1.splitlines(keepends=False) if l.strip()]

    (lights_on, brightness) = follow_instructions(inst)

    assert 9999 == lights_on
    assert 10005 == brightness
