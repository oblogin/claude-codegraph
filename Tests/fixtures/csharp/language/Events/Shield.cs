using System;

namespace Fx.Events
{
    // Trap: another event named Damaged, on an unrelated class.
    public class Shield
    {
        public event Action<int> Damaged;

        public void Hit(int amount)
        {
            Damaged?.Invoke(amount);
        }
    }
}
