using NUnit.Framework;

using Solver;

using System.IO.Abstractions;
using System.IO.Abstractions.TestingHelpers;

namespace Tester
{
    public class TestBase
    {
        public IFileSystem fileSystem;

        public const string INPUT_FILE = "X:\\input.txt";

        public ISolver solver;

        [SetUp]
        public void SetupBase()
        {
            fileSystem = new MockFileSystem();
            fileSystem.Directory.CreateDirectory("X:");
        }

        public void SetInput(string input)
        {
            fileSystem.File.WriteAllText(INPUT_FILE, input);
        }

        public double Solve1Double() => solver.Solve(INPUT_FILE);

        public int Solve1Int() => (int)solver.Solve(INPUT_FILE);

        public double Solve2Double() => solver.Solve2(INPUT_FILE);

        public int Solve2Int() => (int)solver.Solve2(INPUT_FILE);
    }
}
