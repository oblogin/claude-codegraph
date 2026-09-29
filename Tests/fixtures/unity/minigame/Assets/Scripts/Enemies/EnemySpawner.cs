using UnityEngine;
using UnityEngine.AddressableAssets;

namespace Game.Enemies
{
    public class EnemySpawner : MonoBehaviour
    {
        // Addressables reference: serialized as m_AssetGUID, without a {fileID, guid} pair.
        [SerializeField] private AssetReferenceGameObject _bossRef;
        [SerializeField] private WaveConfig _wave;

        private void Start()
        {
            SpawnSlime();
        }

        public GameObject SpawnSlime()
        {
            // Literal path: only Resources folders are searched -> Assets/Resources/Enemies/Slime.prefab.
            var prefab = Resources.Load<GameObject>("Enemies/Slime");
            return Instantiate(prefab, transform.position, Quaternion.identity);
        }

        public GameObject SpawnByName(string enemyName)
        {
            // Non-literal path: the loaded asset is unknown statically.
            var prefab = Resources.Load<GameObject>("Enemies/" + enemyName);
            return prefab != null ? Instantiate(prefab) : null;
        }

        public void SpawnBossFromReference()
        {
            _bossRef.InstantiateAsync(transform.position, Quaternion.identity);
        }

        public void SpawnBossByAddress()
        {
            // Addressables address "Enemies/Boss" (see Default Local Group.asset) -> Boss.prefab.
            Addressables.LoadAssetAsync<GameObject>("Enemies/Boss").Completed += handle => Instantiate(handle.Result);
        }

        public GameObject SpawnBossFromResources()
        {
            // Bug kept on purpose: Boss.prefab is Addressable and not inside a Resources folder,
            // so this returns null. It is NOT a reference to Assets/Prefabs/Enemies/Boss.prefab.
            return Resources.Load<GameObject>("Enemies/Boss");
        }
    }
}
