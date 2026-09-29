using System;

namespace Fx.Statics
{
    public static class GameLog
    {
        public static int Count { get; private set; }

        public static void Write(string message)
        {
            Count++;
            Console.WriteLine(message);
        }
    }
}
