using System;
using System.Collections.Generic;
using System.IO.Abstractions;
using System.Linq;

namespace SolverBase
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

        protected IEnumerable<string> InputItems(string inputFile) => fileSystem.File
                .ReadAllText(inputFile)
                .Split(new[] { "\n", "\r\n", " " }, StringSplitOptions.RemoveEmptyEntries);

        protected string[] InputItemsStrings(string fileName) => InputItems(fileName).ToArray();

        protected int[] InputItemsInts(string inputFile) => InputItems(inputFile)
                .Select(line => int.Parse(line))
                .ToArray();
    }
}
