using System.Collections.Generic;

namespace Fx.Traps
{
    // Trap: instance Write(string), same name as the static GameLog.Write.
    public class AuditLog
    {
        private readonly List<string> _lines = new List<string>();

        public void Write(string message)
        {
            _lines.Add(message);
        }
    }
}
