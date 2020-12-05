using System.IO.Abstractions;
using System.Linq;

namespace Solver
{
    public class Day3 : Base
    {
        public Day3(IFileSystem fileSystem) : base(fileSystem) { }

        public override string Solve(string inputFile)
        {
            char[][] map = InputItems(inputFile).Select(line => line.ToCharArray()).ToArray();

            return TreesOnPath(map, 3).ToString();
        }

        public override string Solve2(string inputFile)
        {
            char[][] map = InputItems(inputFile).Select(line => line.ToCharArray()).ToArray();

            return new double[]
            {
                TreesOnPath(map, 1, 1),
                TreesOnPath(map, 3, 1),
                TreesOnPath(map, 5, 1),
                TreesOnPath(map, 7, 1),
                TreesOnPath(map, 1, 2),
            }.Aggregate(1.0, (acc, x) => acc * x).ToString();
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
