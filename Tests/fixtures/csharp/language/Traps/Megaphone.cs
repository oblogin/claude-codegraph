namespace Fx.Traps
{
    // Trap: instance Shout(string), same name as the extension StringExtensions.Shout.
    public class Megaphone
    {
        public string Shout(string text)
        {
            return text + "!!!";
        }
    }
}
