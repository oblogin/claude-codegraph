namespace Fx.Statics
{
    public static class MathUtil
    {
        public const int MaxValue = 100;

        public static int Clamp(int value, int min, int max)
        {
            if (value < min) return min;
            return value > max ? max : value;
        }
    }
}
