import type { RenderContext } from "@engine";
import { clampValue, Vector } from "@engine/math";
import { Actor } from "./actor";

export const MAX_SPEED = 200;

export class Player extends Actor {
  grounded = true;

  constructor() {
    super(0, 0);
  }

  jump(): void {
    if (!this.grounded) {
      return;
    }
    this.vel = this.vel.add(new Vector(0, -300));
    this.grounded = false;
  }

  protected override onUpdate(dt: number): void {
    super.onUpdate(dt);
    this.vel.x = clampValue(this.vel.x, -MAX_SPEED, MAX_SPEED);
  }

  override draw(ctx: RenderContext): void {
    super.draw(ctx);
    ctx.commands.push("player-marker");
  }
}
