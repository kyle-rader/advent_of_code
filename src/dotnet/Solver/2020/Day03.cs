﻿using System.IO.Abstractions;
using System.Linq;

namespace Solver._2020
{
    public class Day03 : Base
    {
        public Day03(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public override double Solve()
        {
            char[][] map = InputLines().Select(line => line.ToCharArray()).ToArray();

            return TreesOnPath(map, 3);
        }

        public override double Solve2()
        {
            char[][] map = InputLines().Select(line => line.ToCharArray()).ToArray();

            return new double[]
            {
                TreesOnPath(map, 1, 1),
                TreesOnPath(map, 3, 1),
                TreesOnPath(map, 5, 1),
                TreesOnPath(map, 7, 1),
                TreesOnPath(map, 1, 2),
            }.Aggregate(1.0, (acc, x) => acc * x);
        }

        public static int TreesOnPath(char[][] map, int reach, int depth = 1)
        {
            int trees = 0;
            int xMod = map[0].Length;
            for (int i = 0; i * depth < map.Length; i++)
            {
                trees += map[i * depth][(i * reach) % xMod] == '#' ? 1 : 0;
            }
            return trees;
        }
    }
}
