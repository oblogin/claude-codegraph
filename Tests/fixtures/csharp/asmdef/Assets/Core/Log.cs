using UnityEngine;

namespace Game.Core
{
    public static class Log
    {
        public static void Info(string message)
        {
            Debug.Log("[Core] " + message);
        }
    }
}
