using FluentAssertions;

using NUnit.Framework;

using Solver._2021;

namespace Tester._2021
{
    public class Day05Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
");

            solver = new Day05(fileSystem, INPUT_FILE);
        }

        [Test]
        public void Line_Points()
        {
            var subject = new Day05.Line("0,9 -> 5,9");
            subject.Points().Should().BeEquivalentTo(new[]
            {
                (0, 9),
                (1, 9),
                (2, 9),
                (3, 9),
                (4, 9),
                (5, 9),
            });
        }

        [Test]
        public void Part1()
        {
            Solve1Int().Should().Be(5);
        }

        [Test]
        public void Part1_1()
        {
            SetInput(@"0,0 -> 5,5
0,0 -> 0,4
0,0 -> 0,2
");
            Solve1Int().Should().Be(3);
        }

        [Test]
        public void Part2()
        {
            Solve2Int().Should().Be(12);
        }

        [Test]
        public void DiagnoalPoints()
        {
            var subject = new Day05.Line("0,0 -> 2,2");
            subject.Points().Should().BeEquivalentTo(new[]
            {
                (0,0),
                (1,1),
                (2,2),
            });
        }
    }
}
