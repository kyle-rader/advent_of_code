using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections.Generic;

namespace Solver._2020
{
    public class Day6 : Base
    {
        public Day6(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public override double Solve()
        {
            return InputLinesByBlankLines().
                Aggregate(0,
                          (sum, grp) => new HashSet<char>(grp.Replace("\r\n", "").ToCharArray()).Count() + sum);
        }

        public override double Solve2()
        {
            return InputLinesByBlankLines()
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
