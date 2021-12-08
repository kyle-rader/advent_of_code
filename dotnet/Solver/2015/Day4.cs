using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;
using System.Security.Cryptography;
using System.Text;
using System.Threading.Tasks;

namespace Solver._2015
{
    public class Day4 : Base
    {
        public Day4(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public int SmallestMd5Number(string input, string prefix)
        {
            int value = 0;
            string locker = "a lock";

            var result = Parallel.For(1, int.MaxValue, (i, state) =>
             {
                 var md5 = MD5.Create();
                 StringBuilder sb = new StringBuilder();
                 var hash = md5.ComputeHash(Encoding.UTF8.GetBytes($"{input}{i}"));
                 foreach (var b in hash)
                 {
                     sb.Append(b.ToString("X2"));
                 }

                 if (sb.ToString().StartsWith(prefix))
                 {
                     lock(locker)
                     {
                         value = i;
                     }
                     state.Break();
                 }
             });
            return value;
        }

        public override double Solve()
        {
            return SmallestMd5Number(Input, "00000");
        }

        public override double Solve2()
        {
            return SmallestMd5Number(Input, "000000");
        }
    }
}
