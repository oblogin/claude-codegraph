using System.Text;

namespace Fx.Events
{
    public class CombatLog
    {
        private readonly StringBuilder _text = new StringBuilder();

        public void Attach(Health health, Shield shield)
        {
            // Lambda handlers: the calls inside belong to Attach().
            health.Damaged += (amount, source) => Write(source + " -" + amount);
            shield.Damaged += amount => Write("shield -" + amount);
        }

        private void Write(string line)
        {
            _text.AppendLine(line);
        }
    }
}
