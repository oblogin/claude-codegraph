namespace Game.Shared
{
    // Same full name as Assets/Gameplay/Settings.cs. Only Game.Tools itself sees this one.
    public static class Settings
    {
        public static string Load(string key)
        {
            return "tools:" + key;
        }
    }
}
