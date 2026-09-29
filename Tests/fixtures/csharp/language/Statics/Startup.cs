using Fx.Traps;

namespace Fx.Statics
{
    public static class Startup
    {
        public static int Boot()
        {
            GameLog.Write("boot");
            var audit = new AuditLog();
            // Trap: AuditLog.Write is an instance method with the same name as GameLog.Write.
            audit.Write("boot");
            return GameLog.Count;
        }
    }
}
