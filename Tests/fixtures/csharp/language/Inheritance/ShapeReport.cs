using System.Collections.Generic;

namespace Fx.Inheritance
{
    public static class ShapeReport
    {
        public static double TotalArea(IEnumerable<Shape> shapes)
        {
            double total = 0;
            foreach (Shape shape in shapes)
            {
                // Virtual call: static target Shape.Area(), runtime target any override.
                total += shape.Area();
            }
            return total;
        }
    }
}
