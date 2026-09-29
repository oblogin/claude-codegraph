using UnityEngine;

namespace Game.Pickups
{
    // Added to the MagnetCoin prefab variant through m_AddedComponents.
    public class Magnet : MonoBehaviour
    {
        [SerializeField] private float _radius = 3f;

        private void Update()
        {
            foreach (var hit in Physics.OverlapSphere(transform.position, _radius))
            {
                if (hit.TryGetComponent(out Coin coin)) coin.Collect();
            }
        }
    }
}
