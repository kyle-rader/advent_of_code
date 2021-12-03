using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace Solver._2021
{
    public class Day2 : Base
    {
        public Day2(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        private struct Position
        {
            public int Depth;
            public int PositionHorizontal;
            public int Aim;

            public void Add(Position other)
            {
                Depth += other.Depth;
                PositionHorizontal += other.PositionHorizontal;
            }
        }

        private IEnumerable<Position> Steps(IEnumerable<string> stepsRaw)
        {
            foreach(var line in stepsRaw)
            {
                var parts = line.Split();
                int value = int.Parse(parts[1]);
                if (parts[0] == "forward")
                {
                    yield return new Position() { PositionHorizontal = value };
                }
                if (parts[0] == "up")
                {
                    yield return new Position() { Depth = -value };
                }
                if (parts[0] == "down")
                {
                    yield return new Position { Depth = value };
                }
            }
        }

        public override double Solve()
        {
            var input = InputLines();
            Position position = new Position();
            foreach (var step in Steps(input))
                position.Add(step);

            return position.Depth * position.PositionHorizontal;
        }

        public override double Solve2()
        {
            var input = InputLines();
            Position p = new Position();
            foreach (var line in input)
            {
                var parts = line.Split();
                if (parts == null || parts.Length != 2)
                {
                    throw new ArgumentException($"Unparsable command! '{line}'");
                }

                int value = int.Parse(parts[1]);
                if (parts[0] == "forward")
                {
                    p.PositionHorizontal += value;
                    p.Depth += p.Aim * value;
                }
                else if (parts[0] == "up")
                {
                    p.Aim -= value;
                }
                else if (parts[0] == "down")
                {
                    p.Aim += value;
                }
                else
                {
                    throw new ArgumentException($"unkown command {parts[0]}");
                }

            }
            return p.Depth * p.PositionHorizontal;
        }
    }
}
