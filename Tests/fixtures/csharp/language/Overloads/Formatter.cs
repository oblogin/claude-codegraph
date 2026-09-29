namespace Fx.Overloads
{
    public class Formatter
    {
        public string Format(int value)
        {
            return value.ToString();
        }

        public string Format(string text)
        {
            return "\"" + text + "\"";
        }

        public string Format(int value, int width)
        {
            return value.ToString().PadLeft(width);
        }
    }
}
