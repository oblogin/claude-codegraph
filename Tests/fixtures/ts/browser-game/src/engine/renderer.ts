import type { Drawable, RenderContext } from "./interfaces";

export class Renderer {
  private readonly ctx: RenderContext = { layer: 0, commands: [] };

  drawAll(items: Drawable[]): void {
    this.ctx.commands.length = 0;
    for (const item of items) {
      item.draw(this.ctx);
    }
  }
}
