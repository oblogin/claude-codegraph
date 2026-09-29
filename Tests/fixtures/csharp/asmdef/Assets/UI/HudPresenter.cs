using Game.Core;
using UnityEngine;

namespace Game.UI
{
    public class HudPresenter : MonoBehaviour
    {
        public int Show()
        {
            // Game.UI.Log (a member of the enclosing namespace), not Game.Core.Log.
            Log.Info("hud");
            return Stats.Add(2, 3);
        }
    }
}
