import type { KillEvent } from "./game";

export class AudioBus {
  readonly played: string[] = [];

  playHit(event: KillEvent): void {
    this.played.push(`hit:${event.points}`);
  }

  playGameOver(event: { score: number }): void {
    console.log(`game over: ${event.score}`);
  }
}
