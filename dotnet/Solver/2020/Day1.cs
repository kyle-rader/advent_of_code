using System.IO.Abstractions;
using System.Linq;

namespace Solver._2020
{
    public class Day1 : Base
    {
        public Day1(IFileSystem fileSystem) : base(fileSystem) { }

        public override string Solve(string inputFile)
        {
            int sum = 2020;
            int[] a = InputItemsInts(inputFile).ToArray();

            return FindSumGroupProduct(a, sum, 2).ToString();
        }

        public override string Solve2(string inputFile)
        {
            int sum = 2020;
            int[] a = InputItemsInts(inputFile).ToArray();

            return FindSumGroupProduct(a, sum, 3).ToString();
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
