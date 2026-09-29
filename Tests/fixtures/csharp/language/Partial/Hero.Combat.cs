namespace Fx.Partial
{
    // First part of Hero: declares the base class and Attack().
    public partial class Hero : Character
    {
        public int Damage { get; private set; } = 3;

        public int Attack()
        {
            return Damage;
        }
    }
}
