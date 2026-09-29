using System;
using UnityEngine;

namespace Game.Enemies
{
    // Moved from namespace Game.AI without [MovedFrom]: old [SerializeReference] data still names Game.AI.
    [Serializable]
    public class ChaseBehaviour : IEnemyBehaviour
    {
        [SerializeField] private float _range = 8f;

        public void Begin(Enemy owner)
        {
        }

        public void Update(Enemy owner)
        {
            float step = Mathf.Min(_range, owner.Speed) * Time.deltaTime;
            owner.transform.position += owner.transform.forward * step;
        }
    }
}
