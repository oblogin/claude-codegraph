import { format } from "@util";

export function computeScore(kills: number, seconds: number): number {
  return kills * 100 + Math.max(0, 600 - Math.floor(seconds));
}

export function scoreLabel(score: number): string {
  return `Score: ${format(score)}`;
}
