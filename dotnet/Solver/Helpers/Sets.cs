using System.Collections.Generic;

namespace Solver.Helpers
{
    public class Sets
    {
        public static HashSet<T> From<T>(IEnumerable<T> input)
        {
            return new HashSet<T>(input);
        }
    }
}
