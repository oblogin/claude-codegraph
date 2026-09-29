export type RenderContext = {
  layer: number;
  commands: string[];
};

export interface Drawable {
  draw(ctx: RenderContext): void;
}
