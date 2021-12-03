using System.IO.Abstractions;
using System.Linq;

namespace Solver._2020
{
    public class Day1 : Base
    {
        public Day1(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public override double Solve()
        {
            int sum = 2020;
            int[] a = InputInts().ToArray();

            return FindSumGroupProduct(a, sum, 2);
        }

        public override double Solve2()
        {
            int sum = 2020;
            int[] a = InputInts().ToArray();

            return FindSumGroupProduct(a, sum, 3);
        }

        public static int FindSumGroupProduct(int[] a, int targetSum, int acc, int startAt, int remaining)
        {
            for (int i = startAt; i < a.Length; i++)
            {
                int groupSum = a[i] + acc;
                if (groupSum > targetSum) continue;

                if (remaining == 1)
                {
                    if (groupSum == targetSum) return a[i];
                    else continue;
                }

                int subGroupProduct = FindSumGroupProduct(a, targetSum, groupSum, i + 1, remaining - 1);
                if (subGroupProduct >= 0)
                {
                    return a[i] * subGroupProduct;
                }
            }

            return -1;
        }

        public static int FindSumGroupProduct(int[] a, int targetSum, int groupSize)
        {
            return FindSumGroupProduct(a, targetSum, 0, 0, groupSize);
        }
    }
}
