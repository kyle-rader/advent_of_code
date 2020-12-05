using System;
using System.Collections.Generic;
using System.IO.Abstractions;
using System.Linq;

namespace Solver
{
    public abstract class Base : ISolver
    {
        protected readonly IFileSystem fileSystem;

        public Base(IFileSystem fileSystem)
        {
            this.fileSystem = fileSystem;
        }

        public abstract string Solve(string inputFile);

        public abstract string Solve2(string inputFile);

        public string Input(string inputFile) => fileSystem.File.ReadAllText(inputFile);

        protected IEnumerable<string> InputItems(string inputFile) => Input(inputFile)
                .Split(new[] { "\n", "\r\n" }, StringSplitOptions.RemoveEmptyEntries);

        protected string[] InputItemsStrings(string inputFile) => InputItems(inputFile).ToArray();

        protected int[] InputItemsInts(string inputFile) => InputItems(inputFile)
                .Select(line => int.Parse(line))
                .ToArray();
    }
}
