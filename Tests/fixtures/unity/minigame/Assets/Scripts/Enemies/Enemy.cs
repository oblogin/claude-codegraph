using UnityEngine;

namespace Game.Enemies
{
    public class Enemy : MonoBehaviour
    {
        [SerializeField] private float _speed = 3f;
        // Slimes hop. Same YAML key as the old name of PlayerController._jumpHeight.
        [SerializeField] private float jumpForce = 4f;
        [SerializeReference] private IEnemyBehaviour _behaviour = new PatrolBehaviour();

        private readonly CombatStats _stats = new CombatStats();

        public float Speed => _speed;

        private void Awake()
        {
            _behaviour?.Begin(this);
        }

        private void Update()
        {
            _behaviour?.Update(this);
        }

        // Called by the Animation Event "OnAttackHit" of Assets/Animations/Enemy_Attack.anim; no C# callers.
        public void OnAttackHit()
        {
            // CombatStats.OnAttackHit, not a recursive call.
            _stats.OnAttackHit(jumpForce);
        }
    }
}
