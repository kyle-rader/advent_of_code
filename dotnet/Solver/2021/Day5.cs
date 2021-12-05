using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace Solver._2021
{
    public class Day5 : Base
    {
        public Day5(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public override double Solve()
        {
            var straightLines = InputLines()
                .Select(l => new Line(l))
                .Where(l => l.x1 == l.x2 || l.y1 == l.y2);
            return DangerousVentsCount(straightLines.SelectMany(l => l.Points()));
        }

        public override double Solve2()
        {
            var points = InputLines().Select(l => new Line(l)).SelectMany(l => l.Points());
            return DangerousVentsCount(points);
        }

        public int DangerousVentsCount(IEnumerable<(int x, int y)> points)
        {
            Dictionary<(int X, int Y), int> floorMap = new Dictionary<(int X, int Y), int>();
            foreach (var p in points)
            {
                if (floorMap.ContainsKey(p))
                {
                    floorMap[p]++;
                }
                else
                {
                    floorMap[p] = 1;
                }
            }
            return floorMap.Values.Where(c => c >= 2).Count();
        }

        public class Line
        {
            public int x1;
            public int y1;
            public int x2;
            public int y2;

            public Line(string v)
            {
                var coords = v
                    .SplitNoEmpties(new[] { "->", "," })
                    .Select(int.Parse)
                    .ToArray();
                x1 = coords[0];
                y1 = coords[1];
                x2 = coords[2];
                y2 = coords[3];
            }

            public IEnumerable<(int X, int Y)> Points()
            {
                if (x1 == x2 | y1 == y2)
                {
                    return StraightPoints();
                }
                else
                {
                    return DiagnoalPoints();
                }
            }

            private IEnumerable<(int X, int Y)> StraightPoints()
            {
                int xMin = Min(x1, x2);
                int xMax = Max(x1, x2);

                int yMin = Min(y1, y2);
                int yMax = Max(y1, y2);

                for (int x = xMin; x <= xMax; x++)
                {
                    for (int y = yMin; y <= yMax; y++)
                    {
                        yield return (x, y);
                    }
                }
            }

            private IEnumerable<(int X, int Y)> DiagnoalPoints()
            {
                int xInc = x1 < x2 ? 1 : -1;
                int yInc = y1 < y2 ? 1 : -1;

                int x = x1, y = y1;
                while (x != x2 && y != y2)
                {
                    yield return (x, y);
                    x += xInc;
                    y += yInc;
                }
                yield return (x, y);
            }

            private int Min(int a, int b) => a <= b? a : b;
            private int Max(int a, int b) => a >= b ? a : b;
        }
    }
}
