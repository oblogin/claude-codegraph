// Plain JavaScript module that imports a TypeScript module without an extension.
import { format } from "./format";

/** @param {number} score */
export function logScore(score) {
  console.log(`[score] ${format(score)}`);
}
