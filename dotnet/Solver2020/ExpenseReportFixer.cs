using System;

namespace Solver2020
{
    class ExpenseReportFixer
    {
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
