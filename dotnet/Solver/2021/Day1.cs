using System.IO.Abstractions;
using System.Linq;
using System.Collections.Generic;

namespace Solver._2021
{
    public class Day1 : Base
    {
        public Day1(IFileSystem fileSystem) : base(fileSystem) { }

        private int CountIncreases(int[] items)
        {
            int increases = 0;
            for (int i = 1; i < items.Length; i++)
            {
                if (items[i] > items[i - 1])
                    increases++;
            }
            return increases;
        }

        private IEnumerable<int> SlidingWindowSums(int[] depths)
        {
            int n = depths.Length - 2;
            for (int i = 0; i < n; i++)
            {
                yield return depths[i] + depths[i + 1] + depths[i + 2];
            }
        }

        public override double Solve(string inputFile)
        {
            int[] depths = InputItemsInts(inputFile).ToArray();
            return CountIncreases(depths);
        }

        public override double Solve2(string inputFile)
        {
            int[] depths = InputItemsInts(inputFile).ToArray();
            var sums = SlidingWindowSums(depths);
            return CountIncreases(sums.ToArray());
        }
    }
}
