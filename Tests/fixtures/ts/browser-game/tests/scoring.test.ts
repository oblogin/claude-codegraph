import { describe, expect, it } from "vitest";
import { computeScore, scoreLabel } from "@game/scoring";

describe("scoring", () => {
  it("adds a time bonus", () => {
    expect(computeScore(2, 100)).toBe(700);
  });

  it("formats the label with thousands separators", () => {
    expect(scoreLabel(12345)).toBe("Score: 12 345");
  });
});
