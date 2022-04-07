using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace Solver._2021
{
    public class Day09 : Base
    {
        public Day09(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }


        public override double Solve()
        {
            var input = InputLines();
            var yMax = input.Count();
            var n = input.First().Length;
            List<int[]> map = new List<int[]>();
            foreach (var line in input)
            {
                map.Add(line.Select(c => c - '0').ToArray());
            }

            Func<(int row, int col), bool> inBounds = (p) =>
            {
                return p.row >= 0 && p.row < yMax && p.col >= 0 && p.col < n;
            };

            Func<int, int, bool> isLowPoint = (row, col) =>
            {
                List<int> points = new List<(int row, int col)>() {
                    (row, col+1),
                    (row, col-1),
                    (row+1, col),
                    (row-1, col),
                }
                .Where(inBounds)
                .Select(p => map[p.row][p.col])
                .ToList();

                var current = map[row][col];
                return points.All(x => current < x);
            };

            int sum = 0;
            for (int row = 0; row < yMax; row++)
            {
                for (int col = 0; col < n; col++)
                {
                    if (isLowPoint(row, col))
                    {
                        sum += map[row][col] + 1;
                    }
                }
            }

            return sum;
        }

        public override double Solve2()
        {
            var input = InputLines();
            var yMax = input.Count();
            var n = input.First().Length;
            List<int[]> map = new List<int[]>();
            foreach (var line in input)
            {
                map.Add(line.Select(c => c - '0').ToArray());
            }

            Func<(int row, int col), bool> inBounds = (p) =>
            {
                return p.row >= 0 && p.row < yMax && p.col >= 0 && p.col < n;
            };

            Func<int, int, bool> isLowPoint = (row, col) =>
            {
                List<int> points = new List<(int row, int col)>() {
                    (row, col+1),
                    (row, col-1),
                    (row+1, col),
                    (row-1, col),
                }
                .Where(inBounds)
                .Select(p => map[p.row][p.col])
                .ToList();

                var current = map[row][col];
                return points.All(x => current < x);
            };

            Func<int, int, int> basinSize = (row, col) =>
            {
                int size = 0;
                var q = new Queue<(int row, int col)>();
                var seen = new HashSet<(int row, int col)>();

                q.Enqueue((row, col));

                while (q.Count > 0)
                {
                    var p = q.Dequeue();
                    if (!inBounds(p)) continue;
                    if (map[p.row][p.col] >= 9) continue;
                    if (seen.Contains(p)) continue;

                    seen.Add(p);

                    // This point is in the basin
                    size++;
                    // continue the search
                    q.Enqueue((p.row, p.col - 1));
                    q.Enqueue((p.row, p.col + 1));
                    q.Enqueue((p.row + 1, p.col));
                    q.Enqueue((p.row - 1, p.col));
                }
                return size;
            };

            List<int> basins = new List<int>();
            for (int row = 0; row < yMax; row++)
            {
                for (int col = 0; col < n; col++)
                {
                    if (isLowPoint(row, col))
                    {
                        basins.Add(basinSize(row, col));
                    }
                }
            }

            basins.Sort();
            basins.Reverse();
            return basins[0] * basins[1] * basins[2];
        }
    }
}
