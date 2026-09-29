export class Vector {
  constructor(
    public x: number,
    public y: number,
  ) {}

  static zero(): Vector {
    return new Vector(0, 0);
  }

  add(other: Vector): Vector {
    return new Vector(this.x + other.x, this.y + other.y);
  }

  scale(k: number): Vector {
    return new Vector(this.x * k, this.y * k);
  }
}
