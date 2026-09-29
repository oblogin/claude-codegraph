namespace Fx.Traps
{
    public class Warehouse
    {
        public void Stock()
        {
            var store = new Store();
            store.Save("crate");
            var journal = new Journal();
            journal.Save("stocked");
        }
    }
}
