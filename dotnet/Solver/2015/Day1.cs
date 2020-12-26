using System.IO.Abstractions;
using System.Linq;

namespace Solver._2015
{
    public class Day1 : Base
    {
        public Day1(IFileSystem fileSystem) : base(fileSystem) { }

        public override double Solve(string inputFile)
        {
            return Input(inputFile)
                .ToCharArray()
                .Aggregate(
                    0,
                    (acc, c) => acc + (c == '(' ? 1 : -1),
                    (final) => final);
        }

        public override double Solve2(string inputFile)
        {
            int floor = 0, i = 1;
            foreach (var c in Input(inputFile).ToCharArray())
            {
                floor += c == '(' ? 1 : -1;
                if (floor < 0) break;
                i++;
            }
            return i;
        }
    }
}
