using Fx.Traps;

namespace Fx.Extensions
{
    public class Greeter
    {
        private readonly Megaphone _megaphone = new Megaphone();

        public string Greet(string name)
        {
            string loud = name.Shout();
            // Trap: Megaphone.Shout(string) is an instance method, not the extension method.
            return loud + _megaphone.Shout(name);
        }
    }
}
