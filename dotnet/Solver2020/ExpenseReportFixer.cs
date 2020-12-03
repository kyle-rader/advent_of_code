using System;

namespace Solver2020
{
    class ExpenseReportFixer
    {
        public static int FindSumPairProduct(int[] a, int sum)
        {
            int i, j = 0;
            for (i = 0; i < a.Length; i++)
            {
                if (a[i] > sum) continue;
                for (j = i + 1; j < a.Length; j++)
                {
                    int pairSum = a[i] + a[j];
                    if (pairSum == sum)
                    {
                        return a[i] * a[j];
                    }
                }
            }

            throw new Exception($"Unable to find pair that sums to {sum}");
        }
    }
}
