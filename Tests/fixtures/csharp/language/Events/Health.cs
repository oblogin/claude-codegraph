using System;

namespace Fx.Events
{
    public delegate void DamageHandler(int amount, string source);

    public class Health
    {
        private int _hp = 100;

        public event DamageHandler Damaged;

        public event Action Died;

        // Plain delegate field (not an event): assigned with '=' and invoked like a method.
        public Action<int> OnHealed;

        public void TakeDamage(int amount, string source)
        {
            _hp -= amount;
            Damaged?.Invoke(amount, source);
            if (_hp <= 0)
            {
                Died?.Invoke();
            }
        }

        public void Heal(int amount)
        {
            _hp += amount;
            if (OnHealed == null) return;
            OnHealed(amount);
        }
    }
}
