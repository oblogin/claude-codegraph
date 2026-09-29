using System;

namespace Fx.Inheritance
{
    public class Circle : Shape
    {
        private readonly double _radius;

        public Circle(double radius)
        {
            _radius = radius;
        }

        public override double Area()
        {
            return Math.PI * _radius * _radius;
        }

        public override string Describe()
        {
            return "circle, " + base.Describe();
        }
    }
}
