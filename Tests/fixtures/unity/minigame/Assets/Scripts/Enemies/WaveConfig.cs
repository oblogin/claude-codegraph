using System.Collections.Generic;
using UnityEngine;

namespace Game.Enemies
{
    [CreateAssetMenu(menuName = "Game/Wave Config")]
    public class WaveConfig : ScriptableObject
    {
        [SerializeField] private GameObject _enemyPrefab;
        [SerializeField] private int _count = 5;
        [SerializeReference] private List<IEnemyBehaviour> _behaviours = new List<IEnemyBehaviour>();

        public GameObject EnemyPrefab => _enemyPrefab;

        public int Count => _count;

        public IReadOnlyList<IEnemyBehaviour> Behaviours => _behaviours;
    }
}
