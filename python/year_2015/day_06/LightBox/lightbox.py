from collections import defaultdict
from typing import List

from year_2015.day_06.LightBox import Instruction


def follow_instructions(instructions: List[Instruction]):
    lights = defaultdict(lambda: 0)
    brightness = 0

    for inst in instructions:

        if inst.action == 'on':
            for light in inst.lights():
                lights[light] += 1
                brightness += 1

        if inst.action == 'off':
            for light in inst.lights():
                if light in lights and lights[light] > 0:
                    lights[light] -= 1
                    brightness -= 1
                    if lights[light] == 0:
                        del lights[light]

        if inst.action == 'toggle':
            for light in inst.lights():
                lights[light] += 2
                brightness += 2

    return len(lights), brightness
