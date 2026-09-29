using System;

namespace Fx.Inheritance
{
    // Abstract base class: Area() is abstract, Describe() is virtual, Print() is not virtual.
    public abstract class Shape
    {
        public abstract double Area();

        public virtual string Describe()
        {
            return "shape with area " + Area();
        }

        public void Print()
        {
            Console.WriteLine(Describe());
        }
    }
}
