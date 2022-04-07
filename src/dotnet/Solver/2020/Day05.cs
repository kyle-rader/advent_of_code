using System;
using System.Collections.Generic;
using System.IO.Abstractions;
using System.Linq;

namespace Solver._2020
{
    public class Day05 : Base
    {
        public Day05(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public override double Solve()
        {
            return InputLines().Select(x => BoardingPassId(x)).Max();
        }

        public override double Solve2()
        {
            var ids = new HashSet<int>(InputLines().Select(x => BoardingPassId(x)));

            bool[] filled = new bool[] { ids.Contains(0), ids.Contains(1) };
            int i;
            for (i = 2; i < 1023; i++)
            {
                if (ids.Contains(i) && !filled[1] && filled[0]) break;
                filled[0] = filled[1];
                filled[1] = ids.Contains(i);
            }
            return i - 1;
        }

        public static int BoardingPassId(string input)
        {
            return Convert.ToInt32(input
                .Replace('F', '0')
                .Replace('B', '1')
                .Replace('L', '0')
                .Replace('R', '1'), 2);
        }
    }
}
