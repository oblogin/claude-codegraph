using UnityEngine;

// Global namespace on purpose: a third-party style plugin compiled into Assembly-CSharp-firstpass.
public static class Tween
{
    public static void To(Transform target, float duration)
    {
        if (target == null) return;
        target.localPosition += Vector3.up * duration;
    }
}
