using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace Solver._2021
{
    public class Day06 : Base
    {
        public Day06(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public class LanternFishModel
        {
            public const int MaxSize = 9; // 0 tracks fish at 0, up to 8, means a size of 9.]
            public const int Min = 0;
            public const int TimerNewFish = 8;
            public const int TimerNormal = 6;

            public ulong[] fish = new ulong[MaxSize];

            public LanternFishModel(string input)
            {
                foreach (int fish_val in input.SplitNoEmpties(Commas).AsInt())
                {
                    if (fish_val < Min || fish_val >= MaxSize)
                    {
                        throw new ArgumentOutOfRangeException($"Fish value of {fish_val} is out of the expected bounds!");
                    }

                    fish[fish_val]++;
                }
            }

            public ulong Simulate(int days)
            {
                for (int i = 0; i < days; i++)
                {
                    ulong newFish = fish[0];
                    for (int j = 1; j < MaxSize; j++)
                    {
                        fish[j - 1] = fish[j];
                    }
                    fish[TimerNormal] += (ulong)newFish;
                    fish[TimerNewFish] = (ulong)newFish;
                }
                return fish.Aggregate((ulong)0, (a, val) => a + val);
            }
        }

        public override double Solve()
        {
            var model = new LanternFishModel(Input);
            return model.Simulate(80);
        }

        public override double Solve2()
        {
            var model = new LanternFishModel(Input);
            return model.Simulate(256);
        }
    }
}
