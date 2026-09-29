import { EventEmitter, Renderer, type Drawable } from "@engine";
import { startTimer, stopTimer } from "../util/timer.js";
import { Enemy } from "./actors/enemy";
import { Player } from "./actors/player";
import { AudioBus } from "./audio";
import { computeScore, scoreLabel } from "./scoring";
import { Hud } from "./ui/hud";

export type KillEvent = { enemy: Enemy; points: number };

export type GameEvents = {
  kill: KillEvent;
  gameover: { score: number };
};

export class Game {
  readonly events = new EventEmitter<GameEvents>();
  readonly player = new Player();
  private readonly enemies: Enemy[] = [];
  private readonly renderer = new Renderer();
  private readonly hud = new Hud();
  private readonly audio = new AudioBus();
  private kills = 0;
  private elapsed = 0;
  private timerId: number | undefined;

  start(): void {
    this.events.on("kill", this.onEnemyKilled);
    this.events.on("kill", (e) => this.audio.playHit(e));
    this.events.on("gameover", this.audio.playGameOver);
    this.timerId = startTimer(() => this.tick(1 / 60), 16);
  }

  stop(): void {
    this.events.off("kill", this.onEnemyKilled);
    stopTimer(this.timerId);
    this.events.emit("gameover", { score: computeScore(this.kills, this.elapsed) });
  }

  spawnEnemy(x: number, y: number): Enemy {
    const enemy = new Enemy(x, y);
    this.enemies.push(enemy);
    return enemy;
  }

  killEnemy(enemy: Enemy): void {
    this.events.emit("kill", { enemy, points: enemy.points });
  }

  tick(dt: number): void {
    this.elapsed += dt;
    this.player.update(dt);
    for (const enemy of this.enemies) {
      enemy.update(dt);
    }
    this.hud.onUpdate(dt);
    const drawables: Drawable[] = [this.player, ...this.enemies];
    this.renderer.drawAll(drawables);
    this.hud.draw(scoreLabel(computeScore(this.kills, this.elapsed)));
  }

  async loadLevel(index: number): Promise<void> {
    if (index === 1) {
      const level = await import("./levels/level1");
      level.default(this);
      return;
    }
    const level = await import(`./levels/level${index}.ts`);
    level.default(this);
  }

  private onEnemyKilled = (e: KillEvent): void => {
    this.kills += 1;
    const i = this.enemies.indexOf(e.enemy);
    if (i >= 0) {
      this.enemies.splice(i, 1);
    }
  };
}
