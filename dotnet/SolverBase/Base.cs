using System.IO.Abstractions;

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
    }
}
