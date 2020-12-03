using SolverBase;

using System;
using System.IO.Abstractions;
using System.Linq;

namespace Solver2020
{
    public class Day1 : Base, ISolver
    {
        public Day1(IFileSystem fileSystem) : base(fileSystem) { }

        public override string Solve(string inputFile)
        {
            int sum = 2020;
            int[] a = fileSystem.File
                .ReadAllText(inputFile)
                .Split(new[] { "\n", "\r\n", " " }, StringSplitOptions.RemoveEmptyEntries)
                .Select(line => int.Parse(line))
                .ToArray();

            return ExpenseReportFixer.FindSumPairProduct(a, sum).ToString();
        }

        public override string Solve2(string inputFile)
        {
            throw new NotImplementedException();
        }
    }
}
