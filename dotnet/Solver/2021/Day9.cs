using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace Solver._2021
{
    public class Day9 : Base
    {
        public Day9(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }


        public override double Solve()
        {
            var input = InputLines();
            var yMax = input.Count();
            var n = input.First().Length;
            List<int[]> map = new List<int[]>();
            foreach (var line in input)
            {
                map.Add(line.Select(c => (int)c - (int)'0').ToArray());
            }

            Func<(int y, int x), bool> inBounds = (p) => {
                return p.y >= 0 && p.y < yMax && p.x >= 0 && p.x < n;
            };

            Func<int, int, bool> isLowPoint = (y, x) => {
                List<int> points = new List<(int y, int x)>() {
                    (x+1, y+1),
                    (x+1, y-1),
                    (x-1, y+1),
                    (x-1, y-1),
                }
                .Where(inBounds)
                .Select(p => map[p.y][p.x])
                .ToList();

                return points.All(val => val < map[y][x]);
            };

            int sum = 0;
            for (int y = 0; y < yMax; y++)
            {
                for (int x = 0; x < n; x++)
                {
                    if (isLowPoint(x, y)) {
                        sum += map[y][x] + 1;
                    }
                }
            }

            return sum;
        }

        public override double Solve2()
        {
            var input = InputLines();
            return -1;
        }
    }
}
