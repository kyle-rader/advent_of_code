using NUnit.Framework;

using System.Collections.Generic;
using System.IO.Abstractions;
using System.IO.Abstractions.TestingHelpers;

namespace Solver2020Tests
{
    public class TestBase
    {
        public IFileSystem fileSystem;

        public const string INPUT_FILE = "X:\\input.txt";

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
    }
}
