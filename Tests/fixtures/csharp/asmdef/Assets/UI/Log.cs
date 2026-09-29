using UnityEngine;

namespace Game.UI
{
    // Same simple name as Game.Core.Log. Inside namespace Game.UI this type wins over "using Game.Core".
    internal static class Log
    {
        public static void Info(string message)
        {
            Debug.Log("[UI] " + message);
        }
    }
}
