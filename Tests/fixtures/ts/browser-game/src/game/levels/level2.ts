import type { Game } from "../game";

export default function buildLevel(game: Game): void {
  game.spawnEnemy(80, 0);
  game.spawnEnemy(160, 0);
  game.spawnEnemy(240, 0);
}
