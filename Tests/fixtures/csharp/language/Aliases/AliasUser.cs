using Fx.Traps;
using Store = Fx.Interfaces.MemoryStore;
using Log = Fx.Statics.GameLog;
using static Fx.Statics.MathUtil;

namespace Fx.Aliases
{
    public class AliasUser
    {
        public int Run()
        {
            // 'Store' is the alias (MemoryStore): an alias wins over Fx.Traps.Store imported by 'using Fx.Traps'.
            var store = new Store();
            store.Save("alias");
            Log.Write("saved");
            int clamped = Clamp(150, 0, MaxValue);
            int limited = Limits.Clamp(150, 0, 10);
            return clamped + limited;
        }
    }
}
