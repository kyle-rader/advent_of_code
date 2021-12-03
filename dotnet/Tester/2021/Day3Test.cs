using FluentAssertions;

using NUnit.Framework;

using Solver._2021;

using System.Collections.Generic;
using System.Linq;

namespace Tester._2021
{
    public class Day3Test : TestBase
    {
        private const string Input = @"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

        [SetUp]
        public void Setup()
        {
            SetInput(Input);

            solver = new Day3(fileSystem, INPUT_FILE);
        }

        [Test]
        public void Part1()
        {
            Solve1Int().Should().Be(198);
        }

        [Test]
        public void Part2()
        {
            Solve2Int().Should().Be(230);
        }

        [Test]
        public void Co2GeneratorRating()
        {
            Day3 solver = (Day3)this.solver;
            List<string> input = Input.Split(new[] { "\n", "\r\n" }, System.StringSplitOptions.RemoveEmptyEntries).ToList();

            solver
                .Co2GeneratorRating(input)
                .Should()
                .Be(23);
        }

        [Test]
        public void Co2ScrubberRating()
        {
            Day3 solver = (Day3)this.solver;
            List<string> input = Input.Split(new[] { "\n", "\r\n" }, System.StringSplitOptions.RemoveEmptyEntries).ToList();

            solver
                .Co2ScrubberRating(input)
                .Should()
                .Be(10);
        }
    }
}
