// Plain JavaScript module consumed from TypeScript (allowJs).

/**
 * @param {() => void} callback
 * @param {number} intervalMs
 * @returns {number}
 */
export function startTimer(callback, intervalMs) {
  return window.setInterval(callback, intervalMs);
}

/** @param {number | undefined} id */
export function stopTimer(id) {
  if (id !== undefined) {
    window.clearInterval(id);
  }
}
