using Game.Core;
using UnityEngine;

namespace Game.Gameplay
{
    public class Weapon : MonoBehaviour
    {
        public int Fire()
        {
            Log.Info("fire");
            // Game.Gameplay.Tween: asmdef assemblies cannot see Assembly-CSharp-firstpass.
            Tween.To(transform, 0.2f);
            return Stats.Add(1, 2);
        }
    }
}
