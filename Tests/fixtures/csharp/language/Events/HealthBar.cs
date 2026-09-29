namespace Fx.Events
{
    public class HealthBar
    {
        private readonly Health _health;
        private bool _visible = true;
        private int _shown;

        public HealthBar(Health health)
        {
            _health = health;
        }

        public bool Visible => _visible;

        public void Bind()
        {
            _health.Damaged += OnDamaged;
            _health.Died += () => Hide();
            _health.OnHealed = Refresh;
        }

        public void Unbind()
        {
            _health.Damaged -= OnDamaged;
        }

        private void OnDamaged(int amount, string source)
        {
            _shown -= amount;
        }

        private void Refresh(int amount)
        {
            _shown += amount;
        }

        private void Hide()
        {
            _visible = false;
        }
    }
}
