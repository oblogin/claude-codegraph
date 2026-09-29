using System;
using System.Collections.Generic;

namespace Fx.Lambdas
{
    public class Pipeline
    {
        private readonly List<Func<int, int>> _steps = new List<Func<int, int>>();

        public Pipeline Then(Func<int, int> step)
        {
            _steps.Add(step);
            return this;
        }

        public int Run(int input)
        {
            int value = input;
            foreach (Func<int, int> step in _steps)
            {
                value = step(value);
            }
            return value;
        }
    }
}
