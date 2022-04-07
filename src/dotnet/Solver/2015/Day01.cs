using System.IO.Abstractions;
using System.Linq;

namespace Solver._2015
{
    public class Day01 : Base
    {
        public Day01(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public override double Solve()
        {
            return Input
                .ToCharArray()
                .Aggregate(
                    0,
                    (acc, c) => acc + (c == '(' ? 1 : -1),
                    (final) => final);
        }

        public override double Solve2()
        {
            int floor = 0, i = 1;
            foreach (var c in Input.ToCharArray())
            {
                floor += c == '(' ? 1 : -1;
                if (floor < 0) break;
                i++;
            }
            return i;
        }
    }
}
