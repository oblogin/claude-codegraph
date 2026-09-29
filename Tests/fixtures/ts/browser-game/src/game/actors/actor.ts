import { Entity, Vector, type Drawable, type RenderContext } from "@engine";

export class Actor extends Entity implements Drawable {
  pos: Vector;
  vel: Vector = Vector.zero();

  constructor(x: number, y: number) {
    super();
    this.pos = new Vector(x, y);
  }

  protected onUpdate(dt: number): void {
    this.pos = this.pos.add(this.vel.scale(dt));
  }

  draw(ctx: RenderContext): void {
    ctx.commands.push(`rect ${this.pos.x} ${this.pos.y}`);
  }
}
