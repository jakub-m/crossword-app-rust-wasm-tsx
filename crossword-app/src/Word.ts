type Orientation = "hor" | "ver";

interface Word {
  id: number;
  x: number;
  y: number;
  word: string;
  orientation: Orientation;
  definition?: string;
}

export { Orientation, Word };
