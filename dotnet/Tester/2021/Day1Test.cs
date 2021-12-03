using FluentAssertions;

using NUnit.Framework;

using Solver._2021;

namespace Tester._2021
{
    public class Day1Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"199
200
208
210
200
207
240
269
260
263
");

            solver = new Day1(fileSystem);
        }

        [Test]
        public void Part1()
        {
            Solve1Int().Should().Be(7);
        }

        [Test]
        public void Part2()
        {
            Solve2Int().Should().Be(5);
        }
    }
}
