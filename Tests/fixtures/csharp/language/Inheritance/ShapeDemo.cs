namespace Fx.Inheritance
{
    public class ShapeDemo
    {
        public double Run()
        {
            var circle = new Circle(2);
            var square = new Square(3);
            double a = circle.Area();
            double b = square.Area();
            circle.Print();
            return a + b + ShapeReport.TotalArea(new Shape[] { circle, square });
        }
    }
}
