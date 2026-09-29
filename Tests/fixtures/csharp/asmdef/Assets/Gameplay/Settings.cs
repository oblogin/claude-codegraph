namespace Game.Shared
{
    // Same full name (Game.Shared.Settings) as Assets/Tools/Settings.cs, but another assembly.
    // Assembly-CSharp sees this one: Game.Gameplay is autoReferenced, Game.Tools is not.
    public static class Settings
    {
        public static string Load(string key)
        {
            return "gameplay:" + key;
        }
    }
}
