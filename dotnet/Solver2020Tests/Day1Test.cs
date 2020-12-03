using FluentAssertions;

using NUnit.Framework;

using Solver2020;

using System;
using System.IO.Abstractions.TestingHelpers;

namespace Solver2020Tests
{
    public class Day1Test
    {
        private MockFileSystem fileSystem;
        private Day1 subject;

        [SetUp]
        public void Setup()
        {
            this.fileSystem = new MockFileSystem();
            this.subject = new Day1(fileSystem);
        }

        [Test]
        public void ItWorks()
        {
            Action tester = () => this.subject.Solve("input.txt");
            tester.Should().Throw<Exception>();
        }
    }
}