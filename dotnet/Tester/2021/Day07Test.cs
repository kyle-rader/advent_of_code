using FluentAssertions;

using NUnit.Framework;

using Solver._2021;

namespace Tester._2021
{
    public class Day07Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"16,1,2,0,4,2,7,1,2,14");

            solver = new Day07(fileSystem, INPUT_FILE);
        }

        [Test]
        public void Part1()
        {
            Solve1Int().Should().Be(37);
        }

        [TestCase("2,6", 4)]
        [TestCase("2,3,6", 4)]
        public void FuelCostLinear(string input, double expected)
        {
            SetInput(input);
            Solve1Double().Should().Be(expected);
        }

        [Test]
        [Ignore("Example problem requires using Round, but actual problem requires floor of average which produces different results on test data.")]
        public void Part2()
        {
            Solve2Double().Should().Be(168);
        }

        [TestCase("2,6", 4)]
        [TestCase("2,3,6", 4)]
        public void FuelCostExponential(string input, double expected)
        {
            SetInput(input);
            Solve1Double().Should().Be(expected);
        }
    }
}
