using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace Solver._2015
{
    public class Day2 : Base
    {
        public Day2(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public struct Box
        {
            public int[] Sides;
            public int WrappingPaper;
            public int Ribbon;

            public Box(int[] dims)
            {
                int l = dims[0];
                int w = dims[1];
                int h = dims[2];

                Sides = new[] { l * w, w * h, h * l };
                Array.Sort(dims);
                Array.Sort(Sides);

                WrappingPaper = Sides.Select(i => i * 2).Aggregate(0, (acc, i) => acc + i) + Sides[0];
                Ribbon = (dims[0] * 2) + (dims[1] * 2) + dims.Aggregate(1, (acc, i) => acc * i);
            }
        }

        IEnumerable<Box> Boxes() =>
            InputLines()
            .Select((line) =>
            {
                int[] dims = line.SplitNoEmpties(new[] { "x" }).Select(int.Parse).ToArray();
                return new Box(dims);
            });

        public override double Solve()
        {
            return Boxes().Aggregate(0, (acc, b) => acc + b.WrappingPaper);
        }

        public override double Solve2()
        {
            return Boxes().Aggregate(0, (acc, b) => acc + b.Ribbon);
        }
    }
}
