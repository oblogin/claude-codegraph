using UnityEngine;
using UnityEngine.Events;

namespace Game.Player
{
    public class Health : MonoBehaviour
    {
        [SerializeField] private int _maxHp = 10;

        // Persistent listeners are configured in the scene (override in Assets/Scenes/Main.unity).
        public UnityEvent OnDied = new UnityEvent();

        public int Current { get; private set; }

        // Unity message with the same name as PlayerController.Awake (a different method).
        private void Awake()
        {
            Init(_maxHp);
        }

        public void Init(int maxHp)
        {
            _maxHp = maxHp;
            Current = maxHp;
        }

        public void TakeDamage(int amount)
        {
            if (Current <= 0) return;
            Current = Mathf.Max(0, Current - amount);
            if (Current == 0) OnDied.Invoke();
        }

        public void Heal(int amount)
        {
            Current = Mathf.Min(_maxHp, Current + amount);
        }
    }
}
