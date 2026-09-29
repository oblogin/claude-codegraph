using UnityEngine;
using UnityEngine.Serialization;

namespace Game.Player
{
    [RequireComponent(typeof(Health))]
    public class PlayerController : MonoBehaviour
    {
        [SerializeField] private float _speed = 5f;
        [SerializeField, FormerlySerializedAs("jumpForce")] private float _jumpHeight = 2f;
        [SerializeField] private Health _health;

        private Rigidbody _body;

        // Unity message: called by the engine when the component is loaded.
        private void Awake()
        {
            _body = GetComponent<Rigidbody>();
            if (_health == null) _health = GetComponent<Health>();
        }

        // Unity message: called by the engine every frame.
        private void Update()
        {
            var input = new Vector3(Input.GetAxis("Horizontal"), 0f, Input.GetAxis("Vertical"));
            transform.position += input * (_speed * Time.deltaTime);
            if (Input.GetButtonDown("Jump")) Jump();
        }

        private void Jump()
        {
            float velocity = Mathf.Sqrt(2f * Physics.gravity.magnitude * _jumpHeight);
            _body.AddForce(Vector3.up * velocity, ForceMode.VelocityChange);
        }

        // Unity message: called by the physics engine when a trigger collider is entered.
        private void OnTriggerEnter(Collider other)
        {
            if (other.CompareTag("Hazard")) _health.TakeDamage(1);
        }
    }
}
