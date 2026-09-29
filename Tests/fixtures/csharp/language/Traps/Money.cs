namespace Fx.Traps
{
    public class Money
    {
        public Money(decimal amount)
        {
            Amount = amount;
        }

        public decimal Amount { get; }

        // Trap: same name as MathSteps.Double.
        public Money Double()
        {
            return new Money(Amount * 2);
        }
    }
}
