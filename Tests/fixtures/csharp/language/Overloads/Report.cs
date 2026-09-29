namespace Fx.Overloads
{
    public class Report
    {
        private readonly Formatter _formatter = new Formatter();

        public string Build()
        {
            string a = _formatter.Format(42);
            string b = _formatter.Format("total");
            string c = _formatter.Format(7, 3);
            return a + b + c;
        }
    }
}
