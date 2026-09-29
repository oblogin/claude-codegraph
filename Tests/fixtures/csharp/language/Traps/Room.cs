namespace Fx.Traps
{
    // Trap: has Area() but is not a Shape.
    public class Room
    {
        public double Width { get; set; }

        public double Length { get; set; }

        public double Area()
        {
            return Width * Length;
        }
    }
}
