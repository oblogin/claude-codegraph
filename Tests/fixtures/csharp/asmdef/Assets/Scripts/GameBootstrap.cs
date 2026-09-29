using Game.Core;
using Game.Gameplay;
using Game.Shared;
using UnityEngine;

namespace Game.Boot
{
    public class GameBootstrap : MonoBehaviour
    {
        public static string Run()
        {
            // Game.Shared.Settings from Game.Gameplay (Game.Tools is not autoReferenced).
            string mode = Settings.Load("start");
            Log.Info(mode);
            // Global Tween from Assembly-CSharp-firstpass: a type of the global namespace
            // is found before the types imported by "using Game.Gameplay".
            Tween.To(null, 0.5f);
            return mode;
        }

        private void Awake()
        {
            Run();
        }
    }
}
