namespace Fx.Extensions
{
    public static class StringExtensions
    {
        public static string Shout(this string text)
        {
            return text.ToUpperInvariant() + "!";
        }
    }
}
