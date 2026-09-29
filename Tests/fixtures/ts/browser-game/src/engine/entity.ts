export abstract class Entity {
  private static nextId = 1;

  readonly id: number = Entity.nextId++;
  active = true;

  update(dt: number): void {
    if (!this.active) {
      return;
    }
    this.onUpdate(dt);
  }

  protected abstract onUpdate(dt: number): void;
}
