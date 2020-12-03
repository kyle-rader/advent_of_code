using System.IO.Abstractions;

namespace SolverBase
{
    public abstract class Base : ISolver
    {
        private readonly IFileSystem fileSystem;

        public Base(IFileSystem fileSystem)
        {
            this.fileSystem = fileSystem;
        }

        public abstract string Solve(string inputFile);
    }
}
