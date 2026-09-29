using UnityEngine;

namespace Game.Gameplay
{
    // Same simple name as the global Tween in Assets/Plugins (Assembly-CSharp-firstpass).
    public static class Tween
    {
        public static void To(Transform target, float duration)
        {
            target.localScale = Vector3.one * (1f + duration);
        }
    }
}
