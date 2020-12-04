using FluentAssertions;

using NUnit.Framework;

using Solver2020;

namespace Solver2020Tests
{
    public class Day2Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc");

            solver = new Day2(fileSystem);
        }

        [Test]
        public void Part1Example()
        {
            Solve1Int().Should().Be(2);
        }

        [Test]
        public void Part2Example()
        {
            //Solve2Int().Should().Be(0);
        }
    }
}
