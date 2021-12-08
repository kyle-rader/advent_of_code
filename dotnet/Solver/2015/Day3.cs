using System.IO.Abstractions;
using System.Linq;
using System.Collections.Generic;

namespace Solver._2015
{
    public class Day3 : Base
    {
        public Day3(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public class Santa
        {
            int x = 0;
            int y = 0;
            public (int, int) Move(char step)
            {
                switch (step)
                {
                    case '^':
                        y += 1;
                        break;
                    case 'v':
                        y -= 1;
                        break;
                    case '<':
                        x -= 1;
                        break;
                    case '>':
                        x += 1;
                        break;
                }
                return (x, y);
            }
        }

        public override double Solve()
        {
            var input = Input.ToCharArray();
            Dictionary<(int, int), int> houses = new Dictionary<(int, int), int>()
            {
                { (0,0), 1 },
            };
            Santa s1 = new Santa();
            foreach (var step in input)
            {
                var pos = s1.Move(step);
                houses[pos] = houses.GetOrZero(pos) + 1;
            }
            return houses.Count();
        }

        public override double Solve2()
        {
            var input = Input.ToCharArray();
            Dictionary<(int, int), int> houses = new Dictionary<(int, int), int>()
            {
                { (0,0), 2 },
            };
            Santa s1 = new Santa();
            Santa s2 = new Santa();
            for (int i = 0; i < input.Length; i++)
            {
                var step = input[i];
                var pos = (i % 2 == 0) ? s1.Move(step) : s2.Move(step);
                houses[pos] = houses.GetOrZero(pos) + 1;
            }
            return houses.Count();
        }
    }
}
