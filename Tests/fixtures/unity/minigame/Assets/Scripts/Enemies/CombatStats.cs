namespace Game.Enemies
{
    // Plain C# class, not a MonoBehaviour: Animation Events cannot call its methods.
    public class CombatStats
    {
        public int Hits { get; private set; }

        public float LastForce { get; private set; }

        public void OnAttackHit(float force)
        {
            Hits++;
            LastForce = force;
        }
    }
}
