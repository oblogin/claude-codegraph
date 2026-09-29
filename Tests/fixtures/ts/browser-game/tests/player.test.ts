import { describe, expect, it } from "vitest";
import { Player } from "../src/game/actors/player";
import { Hud } from "../src/game/ui/hud";

describe("Player", () => {
  it("jumps only when grounded", () => {
    const player = new Player();
    player.jump();
    expect(player.grounded).toBe(false);
    const vy = player.vel.y;
    player.jump();
    expect(player.vel.y).toBe(vy);
  });

  it("moves on update", () => {
    const player = new Player();
    player.vel.x = 10;
    player.update(0.5);
    expect(player.pos.x).toBe(5);
  });
});

describe("Hud", () => {
  it("shows elapsed time", () => {
    const hud = new Hud();
    hud.onUpdate(61);
    hud.draw("x");
    expect(hud.lines[1]).toBe("1:01");
  });
});
