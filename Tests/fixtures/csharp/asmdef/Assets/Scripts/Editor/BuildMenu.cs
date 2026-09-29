using Game.Boot;
using Game.Core;
using UnityEditor;

namespace Game.EditorTools
{
    public static class BuildMenu
    {
        [MenuItem("Game/Run Bootstrap")]
        public static void Build()
        {
            // Assembly-CSharp-Editor sees Assembly-CSharp and every autoReferenced assembly.
            string mode = GameBootstrap.Run();
            Log.Info("editor: " + mode);
        }
    }
}
