namespace Fx.Traps
{
    // Trap: static Clamp with the same signature as MathUtil.Clamp.
    public static class Limits
    {
        public static int Clamp(int value, int low, int high)
        {
            return value < low ? low : (value > high ? high : value);
        }
    }
}
