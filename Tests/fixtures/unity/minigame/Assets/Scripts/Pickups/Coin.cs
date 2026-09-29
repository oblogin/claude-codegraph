using UnityEngine;

namespace Game.Pickups
{
    public class Coin : MonoBehaviour
    {
        [SerializeField] private int _value = 1;

        public static int Collected { get; private set; }

        // Unity message (same name as PlayerController.OnTriggerEnter, a different class).
        private void OnTriggerEnter(Collider other)
        {
            if (other.CompareTag("Player")) Collect();
        }

        // Lower-case name: NOT the OnTriggerExit message, the engine never calls it.
        private void onTriggerExit(Collider other)
        {
            Debug.Log("never called: " + other.name);
        }

        public void Collect()
        {
            Collected += _value;
            Destroy(gameObject);
        }
    }
}
