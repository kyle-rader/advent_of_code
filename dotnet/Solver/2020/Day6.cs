using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections.Generic;

namespace Solver._2020
{
    public class Day6 : Base
    {
        public Day6(IFileSystem fileSystem) : base(fileSystem) { }

        public override double Solve(string inputFile)
        {
            return InputItemsByBlankLines(inputFile).
                Aggregate(0,
                          (sum, grp) => new HashSet<char>(grp.Replace("\r\n", "").ToCharArray()).Count() + sum);
        }

        public override double Solve2(string inputFile)
        {
            return InputItemsByBlankLines(inputFile)
                .Aggregate(0, (sum, grp) =>
                    sum + grp
                            .Split(new[] { "\n", "\r\n" }, StringSplitOptions.RemoveEmptyEntries)
                            .Select((p) => new HashSet<char>(p.AsEnumerable()))
                            .Aggregate(null as HashSet<char>,
                                       (intersection, person) =>
                                       {
                                           if (intersection == null)
                                               return person;
                                           else
                                               intersection.IntersectWith(person);
                                           return intersection;
                                       })
                            .Count());
        }
    }
}
