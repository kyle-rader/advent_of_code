using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace Solver._2020
{
    public class Day6 : Base
    {
        public Day6(IFileSystem fileSystem) : base(fileSystem) { }

        public override string Solve(string inputFile)
        {
            return InputItemsByBlankLines(inputFile).
                Aggregate(0,
                          (sum, next) => new HashSet<char>(next.Replace("\r\n", "").ToCharArray()).Count() + sum)
                .ToString();
        }

        public override string Solve2(string inputFile)
        {
            var input = Input(inputFile);
            return null;
        }
    }
}
