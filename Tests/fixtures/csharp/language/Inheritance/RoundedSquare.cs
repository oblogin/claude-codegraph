namespace Fx.Inheritance
{
    // Two levels deep: RoundedSquare -> Square -> Shape.
    public class RoundedSquare : Square
    {
        private readonly double _cornerRadius;

        public RoundedSquare(double side, double cornerRadius) : base(side)
        {
            _cornerRadius = cornerRadius;
        }

        public override double Area()
        {
            // base.Area() is Square.Area(), not the abstract Shape.Area().
            double cut = (4 - System.Math.PI) * _cornerRadius * _cornerRadius;
            return base.Area() - cut;
        }

        // Trap: 'new' hides Shape.Describe() instead of overriding it.
        public new string Describe()
        {
            return "rounded square";
        }
    }
}
