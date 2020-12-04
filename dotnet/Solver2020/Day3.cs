using SolverBase;

using System.IO.Abstractions;
using System.Linq;

namespace Solver2020
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
            throw new System.NotImplementedException();
        }

        public static int TreesOnPath(char[][] map, int reach)
        {
            int trees = 0;
            int xMod = map[0].Length;
            for (int i = 0; i < map.Length; i++)
            {
                trees += map[i][(i * reach) % xMod] == '#' ? 1 : 0;
            }
            return trees;
        }
    }
}
