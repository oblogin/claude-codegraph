namespace Fx.Partial
{
    // Second part of Hero: no base list here, Hero still inherits Character.
    public partial class Hero
    {
        public float Speed { get; private set; } = 1f;

        public int Dash()
        {
            Speed *= 2;
            // Attack() is declared in the other part (Hero.Combat.cs).
            return Attack();
        }
    }
}
