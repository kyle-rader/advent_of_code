using FluentAssertions;

using NUnit.Framework;

using Solver._2021;

using System.Linq;

namespace Tester._2021
{
    public class Day6Test : TestBase
    {
        Day6.LanternFishModel fishModel;

        [SetUp]
        public void Setup()
        {
            SetInput(@"3,4,3,1,2");
            solver = new Day6(fileSystem, INPUT_FILE);

            fishModel = new Day6.LanternFishModel("3,4,3,1,2");
        }

        [Test]
        public void LanternFishModel_New()
        {
            fishModel.fish.Should().BeEquivalentTo(new[]
            {
                0, // no 0
                1, // one 1
                1, // one 2
                2, // two 3's
                1, // one 4
                0, // no 5
                0, // no 6
                0, // no 7
                0, // no 8
            });
        }

        [Test]
        public void LanternFishModel_Simulate()
        {
            fishModel.Simulate(1);
            fishModel.fish.Should().BeEquivalentTo(new[]
            {
                1,
                1,
                2,
                1,
                0,
                0,
                0,
                0,
                0,
            });
        }

        [Test]
        public void Part1()
        {
            Solve1Int().Should().Be(5934);
        }

        [Test]
        public void Part2()
        {
            Solve2Double().Should().Be(26984457539);
        }
    }
}
