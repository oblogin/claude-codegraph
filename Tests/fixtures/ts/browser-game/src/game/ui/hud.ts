import { format } from "./format";

function clamp(text: string, maxLength: number): string {
  return text.length > maxLength ? text.slice(0, maxLength) : text;
}

export class Hud {
  private elapsed = 0;
  lines: string[] = [];

  onUpdate(dt: number): void {
    this.elapsed += dt;
  }

  draw(label: string): void {
    this.lines = [clamp(label, 24), format(this.elapsed)];
  }
}
