using Game.Core;
using Game.Shared;

namespace Game.Tools
{
    public static class SettingsTool
    {
        public static string Apply()
        {
            // Game.Tools does not reference Game.Gameplay: this is Assets/Tools/Settings.cs.
            string value = Settings.Load("quality");
            Log.Info(value);
            return value;
        }
    }
}
