import React from "react";
import { Word } from "./Word";


export interface CrosswordGridProps {
  words: Word[];
  hideLetters?: boolean
}

interface GridElem {
  char: string,
  /* Id is the number in the upper-left corner. */
  id: string | null,
}


interface LetterBoxProps {
  mainLetter: string;
  cornerLetter: string;
}

const LetterBox: React.FC<LetterBoxProps> = ({ mainLetter, cornerLetter }) => {
  return (
    <div style={styles.container}>
      <span style={styles.mainLetter}>{mainLetter}</span>
      <span style={styles.cornerLetter}>{cornerLetter}</span>
    </div>
  );
};

const styles: Record<string, React.CSSProperties> = {
  container: {
    position: "relative",
  },
  mainLetter: {
    //position: "absolute",
  },
  cornerLetter: {
    position: "absolute",
    top: "-0.3em",
    left: "-0.7em",
    fontSize: "0.7em",
  },
};



export const CrosswordGrid: React.FC<CrosswordGridProps> = ({ words, hideLetters=false }) => {
  let maxX = 0, maxY = 0;
  words.forEach(({ x, y, word, orientation }) => {
    if (orientation === "hor") {
      maxX = Math.max(maxX, x + word.length);
      maxY = Math.max(maxY, y + 1);
    } else {
      maxX = Math.max(maxX, x + 1);
      maxY = Math.max(maxY, y + word.length);
    }
  });

  const grid: (GridElem | null)[][] = Array.from({ length: maxY }, () =>
    Array.from({ length: maxX }, () => null)
  );

  const merge_grid_elem = ({char, id, x, y}: {char: string, id: string | null, x: number, y: number}) => {
    const old_grid_elem = grid[y][x]
    if (old_grid_elem === null) {
      grid[y][x] = {char, id}
    } else {
      let final_id = old_grid_elem.id;
      if (final_id === null) {
        final_id = id
      }
      grid[y][x] = {char, id: final_id}
    }
  }

  words.forEach(({ x, y, word, orientation, id }) => {
    for (let i = 0; i < word.length; i++) {
      const grid_id = (i === 0) ? ("" + id) : null; // Id only on first letter.
      if (orientation === "hor") {
        merge_grid_elem({x: x+i, y: y, char: word[i], id: grid_id})
      } else if (orientation === "ver" ) {
        merge_grid_elem({x, y: y+i, char: word[i], id: grid_id})
      } else {
        throw Error("bad orientation")
      }
    }
  });

  const grid_elem_to_letter_box = (letter: GridElem | null) => {
    const mainLetter = (hideLetters ? null : letter?.char) || "";
    const cornerLetter = letter?.id || "";
    return <LetterBox
      mainLetter={mainLetter}
      cornerLetter={cornerLetter}
    />
  }
        // border: "2px solid black",
        // backgroundColor: "#eee"
        // padding: "10px",

  return (
    <div
      style={{
        display: "grid",
        gridTemplateColumns: `repeat(${maxX}, 30px)`,
        gridTemplateRows: `repeat(${maxY}, 30px)`,
        gap: "2px",
      }}
    >
      {grid.map((row, rowIndex) =>
        row.map((letter, colIndex) => (
          <div
            key={`${rowIndex}-${colIndex}`}
            style={{
              width: "30px",
              height: "30px",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              border: letter ? "1px solid black": "",
              fontSize: "18px",
              fontWeight: "bold",
            }}
          >
            {grid_elem_to_letter_box(letter)}
          </div>
        ))
      )}
    </div>
  );
};
