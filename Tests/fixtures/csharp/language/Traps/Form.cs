namespace Fx.Traps
{
    // Trap: Validate() like IEntity.Validate(), but Form is not an IEntity.
    public class Form
    {
        public string Email { get; set; }

        public bool Validate()
        {
            return !string.IsNullOrEmpty(Email) && Email.Contains("@");
        }
    }
}
