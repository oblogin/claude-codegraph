namespace Fx.Events
{
    // Trap: a handler with the same name as HealthBar.OnDamaged, subscribed to Shield.Damaged.
    public class ShieldBar
    {
        private int _shown;

        public void Bind(Shield shield)
        {
            shield.Damaged += OnDamaged;
        }

        private void OnDamaged(int amount)
        {
            _shown -= amount;
        }
    }
}
