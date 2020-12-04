using NUnit.Framework;

using SolverBase;

using System.IO.Abstractions;
using System.IO.Abstractions.TestingHelpers;

namespace Solver2020Tests
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

        public string Solve1String() => solver.Solve(INPUT_FILE);

        public int Solve1Int() => int.Parse(Solve1String());

        public string Solve2String() => solver.Solve2(INPUT_FILE);

        public int Solve2Int() => int.Parse(Solve2String());
    }
}
