using FluentAssertions;

using NUnit.Framework;

using Solver2020;

namespace Solver2020Tests
{
    public class Day1Test : TestBase
    {
        private Day1 subject;

        [SetUp]
        public void Setup()
        {
            subject = new Day1(fileSystem);
        }

        [Test]
        public void Part1Example()
        {
            SetInput(@"1721
979
366
299
675
1456");
            int.Parse(subject.Solve(INPUT_FILE)).Should().Be(514579);
        }
    }
}