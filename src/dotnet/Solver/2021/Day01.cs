using System.IO.Abstractions;
using System.Linq;
using System.Collections.Generic;

namespace Solver._2021
{
    public class Day01 : Base
    {
        public Day01(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

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

        private IEnumerable<int> SlidingWindowSums(int[] depths, int windowSize)
        {
            int n = depths.Length - 2;
            for (int i = 0; i < n; i++)
            {
                int windowSum = 0;
                for (int j = 0; j < windowSize; j++)
                {
                    windowSum += depths[i + j];
                }
                yield return windowSum;
            }
        }

        public override double Solve()
        {
            int[] depths = InputInts().ToArray();
            return CountIncreases(depths);
        }

        public override double Solve2()
        {
            int[] depths = InputInts().ToArray();
            var sums = SlidingWindowSums(depths, 3);
            return CountIncreases(sums.ToArray());
        }
    }
}
