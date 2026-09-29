import { Game } from "@game/game";

const game = new Game();

function onKeyDown(event: KeyboardEvent): void {
  if (event.key === " ") {
    game.player.jump();
  }
  if (event.key === "Escape") {
    game.stop();
  }
}

window.addEventListener("keydown", onKeyDown);
game.start();
void game.loadLevel(1);
