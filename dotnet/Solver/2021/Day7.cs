using System.IO.Abstractions;
using System;
using System.Linq;

namespace Solver._2021
{
    public class Day7 : Base
    {
        public Day7(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public override double Solve()
        {
            var input = InputByCommas().AsInt().ToArray();
            Array.Sort(input);
            int alignOn = input[(input.Count() - 1) / 2]; // median
            return input.Aggregate(0, (a, i) => a + Math.Abs(alignOn - i));
        }

        public override double Solve2()
        {
            var input = InputByCommas().AsInt().ToArray();
            double alignOn = (int)input.Average();
            return input.Aggregate(0.0, (a, i) => a + SumOf(Math.Abs(alignOn - i)));
        }

        public static double SumOf(double i)
        {
            return i * (i + 1) / 2.0;
        }
    }
}
