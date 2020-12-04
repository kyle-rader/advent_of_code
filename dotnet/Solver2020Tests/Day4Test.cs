using FluentAssertions;

using NUnit.Framework;

using Solver2020;

namespace Solver2020Tests
{
    public class Day4Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"");

            solver = new Day4(fileSystem);
        }

        [Test]
        public void Part1Example()
        {
            //Solve1Int().Should().Be(0);
        }

        [Test]
        public void Part2Example()
        {
            //Solve2Double().Should().Be(0);
        }
    }
}
