namespace Fx.Inheritance
{
    public class Square : Shape
    {
        protected readonly double Side;

        public Square(double side)
        {
            Side = side;
        }

        public override double Area()
        {
            return Side * Side;
        }
    }
}
