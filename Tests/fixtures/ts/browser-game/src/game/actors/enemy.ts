import * as math from "@engine/math";
import { randomInt } from "@util";
import { Actor } from "./actor";

export class Enemy extends Actor {
  readonly points = randomInt(10, 50);

  protected override onUpdate(dt: number): void {
    const dir = this.pos.x > 0 ? -1 : 1;
    this.vel.x = math.clampValue(this.vel.x + dir * 10, -40, 40);
    super.onUpdate(dt);
  }
}
