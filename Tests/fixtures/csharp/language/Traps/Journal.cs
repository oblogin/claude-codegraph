using System.Collections.Generic;

namespace Fx.Traps
{
    // Trap: has Save(string) and Load() like IStorage, but does not implement it.
    public class Journal
    {
        private readonly List<string> _entries = new List<string>();

        public void Save(string data)
        {
            _entries.Add(data);
        }

        public string Load()
        {
            return string.Join("\n", _entries);
        }
    }
}
