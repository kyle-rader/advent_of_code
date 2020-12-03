using System.IO.Abstractions;

namespace SolverBase
{
    public interface ISolver
    {
        string Solve(string inputFile);

        string Solve2(string inputFile);
    }
}
