import type { Game } from "../game";

export default function buildLevel(game: Game): void {
  game.spawnEnemy(100, 0);
  game.spawnEnemy(200, 0);
}
