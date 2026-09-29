using System;
using UnityEngine;

namespace Game.Enemies
{
    [Serializable]
    public class PatrolBehaviour : IEnemyBehaviour
    {
        [SerializeField] private int _waypointCount = 3;
        private int _current;

        public void Begin(Enemy owner)
        {
            _current = 0;
        }

        // Not a Unity message: PatrolBehaviour is not a MonoBehaviour.
        public void Update(Enemy owner)
        {
            _current = (_current + 1) % _waypointCount;
            owner.transform.Rotate(0f, owner.Speed, 0f);
        }
    }
}
