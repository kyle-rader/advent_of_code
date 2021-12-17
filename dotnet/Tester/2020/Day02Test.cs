using FluentAssertions;

using NUnit.Framework;

using Solver._2020;

namespace Tester._2020
{
    public class Day02Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc");

            solver = new Day02(fileSystem, INPUT_FILE);
        }

        [Test]
        public void Part1Example()
        {
            Solve1Int().Should().Be(2);
        }

        [Test]
        public void Part2Example()
        {
            Solve2Int().Should().Be(1);
        }
    }
}
