use std::cmp::{max, min};
use std::hash::Hash;
use std::{collections::HashMap, fmt, ops};

/// Layout of the words: position and orientation per word.
#[derive(Clone)]
pub struct Layout {
    /// The words with positions.
    positioned_words: Vec<WordPosition>,
    /// A map with characters. This map is fully defined by positioned_words and is merely an optimization
    /// over creating such a map each time it's needed.
    char_map: CharMap,
    /// Count times when a word cut other word. The n_crossings is a number of characters cut.
    n_crossings: usize,
}

type LayoutResult<T = ()> = Result<T, ()>;

impl Layout {
    pub fn new() -> Layout {
        Layout {
            positioned_words: Vec::new(),
            char_map: CharMap::new(),
            n_crossings: 0,
        }
    }

    pub fn get_word_positions(&self) -> &Vec<WordPosition> {
        &self.positioned_words
    }

    /// Assign a number per word. The (id, orientation) is unique, there can be two words starting at the same
    /// field of the grid with different orientation. Theoretically there can be two words totally overlapping
    /// with the same orientation and id, like "mass" and "massage".
    pub fn get_words_with_ids(&self) -> Vec<(&WordPosition, usize)> {
        let mut words = self.positioned_words.clone();
        words.sort_by(|a, b| {
            if a.pos.y == b.pos.y {
                a.pos.x.cmp(&b.pos.x)
            } else {
                a.pos.y.cmp(&b.pos.y)
            }
        });
        let mut pos_to_id: HashMap<XY, usize> = HashMap::new();
        for w in words.iter() {
            if !pos_to_id.contains_key(&w.pos) {
                pos_to_id.insert(w.pos, pos_to_id.len() + 1);
            }
        }

        self.positioned_words
            .iter()
            .map(|wp| (wp, *pos_to_id.get(&wp.pos).unwrap()))
            .collect()
    }

    pub fn area(&self) -> u32 {
        if self.char_map.top_left.is_none() && self.char_map.bottom_right.is_none() {
            return 0;
        }
        let top_left = self.char_map.top_left.unwrap();
        let bottom_right = self.char_map.bottom_right.unwrap();
        let dim = bottom_right - top_left + XY::one();
        assert!(dim.x > 0);
        assert!(dim.y > 0);
        return (dim.x * dim.y) as u32;
    }

    pub fn crossings_count(&self) -> usize {
        self.n_crossings
    }

    /// # Returns
    /// If the returned value is Err it means that there was a conflict on insertion. If Ok, the
    /// number in Ok tells how many other words did this word cut.
    pub fn insert_at<A: Into<XY>>(
        &mut self,
        word: &str,
        pos: A,
        orientation: Orientation,
    ) -> LayoutResult {
        let pos: XY = pos.into();
        self.positioned_words.push(WordPosition {
            word: word.to_owned(),
            pos,
            orientation,
        });
        let wp = self.positioned_words.last().unwrap();
        match self
            .char_map
            .insert_word(wp.word.as_str(), wp.pos, wp.orientation)
        {
            Ok(n_crossings) => {
                self.n_crossings += n_crossings;
                Ok(())
            }
            Err(_) => Err(()),
        }
    }

    /// Given a word, iterate over possible positions of the word. A possible start position is a position that does
    /// not conflict with other letters and does not make the word "stick" side by side with other word.
    ///
    /// # Known bugs
    /// - two same words like "bob" and "bob" will overlap instead of forming two crossing bobs.
    pub fn get_possible_start_positions(&self, word: &str) -> Vec<(XY, Orientation)> {
        eprintln!("get_possible_start_positions for {word}");
        // eprintln!("{:?}", &self.positioned_words);
        if self.positioned_words.is_empty() {
            eprintln!("first word at (0,0) is {word}");
            return vec![
                (XY::zero(), Orientation::Horiz),
                (XY::zero(), Orientation::Vert),
            ];
        }

        let mut word_positions: Vec<(XY, Orientation)> = Vec::new();
        for (i, curr_char) in word.chars().enumerate() {
            // Given a character and position, check if the word starting at that position (vertically
            // or horizontally) would conflict with any other word on the layout, i.e. would two different
            // letters land on the same field..
            let i = i as i32;
            for orient in [Orientation::Horiz, Orientation::Vert] {
                let start_delta = orient.step() * (-i);
                if let Some(pos_on_layout_vec) = self.char_map.char_to_pos.get(&curr_char) {
                    for pos_on_layout in pos_on_layout_vec {
                        let word_pos = start_delta + *pos_on_layout;
                        eprintln!("Consider potential possible start position {word_pos}");
                        if self.would_conflict_with_other_char(word, &word_pos, orient) {
                            eprintln!("Position {word_pos} would conflict other char");
                            continue;
                        }
                        if self.would_envelope_overlap(word.chars().count(), &word_pos, orient) {
                            eprintln!("Position {word_pos} would make envelope overlap");
                            continue;
                        }
                        eprintln!("Add potential position for {word}: {word_pos} {orient}");
                        word_positions.push((word_pos, orient));
                    }
                };
            }
        }
        word_positions
    }

    /// Tell if a word put at specific position would result in letters conflicting, i.e. would
    /// different letters land on the same field of the layout.
    fn would_conflict_with_other_char(&self, word: &str, pos: &XY, orient: Orientation) -> bool {
        let mut pos = *pos;
        for c in word.chars() {
            if let Some(char_on_layout) = self.char_map.pos_to_char.get(&pos) {
                if *char_on_layout != c {
                    return true;
                }
            }
            pos = pos + orient.step();
        }
        return false;
    }

    /// Check if the word would touch other existing word.
    ///
    /// For word brown:
    ///
    /// ```text
    ///
    ///    brown
    ///    ^x   ^x+n
    ///   ^x-1
    ///
    /// ```
    fn would_envelope_overlap(
        &self,
        word_len: usize,
        tentative_start_pos: &XY,
        orient: Orientation,
    ) -> bool {
        let n = word_len as i32;
        let tentative_start_pos = *tentative_start_pos;

        // Prevent one word to be a continuation of the other word.
        let tips = [-1, n];
        for i in tips {
            let p = (orient.step() * i) + tentative_start_pos;
            // eprintln!("would_envelope_overlap({tentative_start_pos}) i={i} p={p}");
            if self.char_map.is_pos_taken(&p) {
                return true;
            }
        }

        // Prevent two words to "stick" side by side.
        for i in 0..n {
            let pos_in_word = (orient.step() * i) + tentative_start_pos;
            if self.char_map.is_pos_taken(&pos_in_word) {
                continue;
            }
            for b in orient.band() {
                let p = pos_in_word + b;
                if self.char_map.is_pos_taken(&p) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn normalize(self) -> Layout {
        let mut words = self.positioned_words.iter();
        let mut min_x: i32;
        let mut min_y: i32;
        if let Some(wp) = words.next() {
            min_x = wp.pos.x;
            min_y = wp.pos.y;
        } else {
            return self;
        }
        for wp in words {
            min_x = min(min_x, wp.pos.x);
            min_y = min(min_y, wp.pos.y);
        }

        let offset = XY {
            x: -min_x,
            y: -min_y,
        };
        let mut layout = Layout::new();
        for wp in self.positioned_words.iter() {
            layout
                .insert_at(&wp.word, wp.pos + offset, wp.orientation)
                .unwrap();
        }
        for wp in &layout.positioned_words {
            assert!(wp.pos.x >= 0);
            assert!(wp.pos.y >= 0);
        }
        layout
    }
}

#[derive(Clone, Debug)]
pub struct WordPosition {
    pub word: String,
    pub pos: XY,
    pub orientation: Orientation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct XY {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for XY {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "XY{{{},{}}}", self.x, self.y)
    }
}

impl XY {
    fn zero() -> XY {
        XY { x: 0, y: 0 }
    }
    fn one() -> XY {
        XY { x: 1, y: 1 }
    }
}

impl ops::Add<XY> for XY {
    type Output = XY;

    fn add(self, rhs: XY) -> Self::Output {
        XY {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<XY> for XY {
    type Output = XY;

    fn sub(self, rhs: XY) -> Self::Output {
        XY {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<i32> for XY {
    type Output = XY;

    fn mul(self, rhs: i32) -> Self::Output {
        XY {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl<T: Into<i32>> From<(T, T)> for XY {
    fn from(value: (T, T)) -> Self {
        return XY {
            x: value.0.into(),
            y: value.1.into(),
        };
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    /// Left-to-right
    Horiz,
    /// Top-to-bottom
    Vert,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Orientation::Horiz => write!(f, "Hor"),
            Orientation::Vert => write!(f, "Ver"),
        }
    }
}

impl Orientation {
    fn step(&self) -> XY {
        match self {
            Orientation::Horiz => XY { x: 1, y: 0 },
            Orientation::Vert => XY { x: 0, y: 1 },
        }
    }

    fn band(&self) -> [XY; 2] {
        let s = match self {
            Orientation::Horiz => XY { x: 0, y: 1 },
            Orientation::Vert => XY { x: 1, y: 0 },
        };
        [s, XY::zero() - s]
    }
}

impl fmt::Display for Layout {
    /// A "fill character" passed in formatter will be used to fill in the empty spaces on the layout.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char_map = self.char_map.normalize();
        let dim = if let Some(xy) = char_map.bottom_right {
            xy + XY::one()
        } else {
            return write!(f, "[]");
        };

        let empty_char = f.fill();

        let new_row = || (0..dim.x).map(|_| empty_char).collect::<Vec<char>>();
        let mut grid: Vec<Vec<char>> = (0..dim.y).map(|_| new_row()).collect();

        for (pos, c) in char_map.pos_to_char.iter() {
            grid[pos.y as usize][pos.x as usize] = *c
        }

        let mut rows = grid.iter();
        if let Some(row) = rows.next() {
            for c in row.iter() {
                write!(f, "{}", c)?;
            }
        }
        for row in rows {
            write!(f, "\n")?;
            for c in row.iter() {
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}

/// Represents a grid with characters on it.
#[derive(Clone, Debug)]
struct CharMap {
    pos_to_char: HashMap<XY, char>,
    char_to_pos: HashMap<char, Vec<XY>>,
    top_left: Option<XY>,
    /// Bottom-right corner inside the layout rectangle.
    bottom_right: Option<XY>,
    /// Indicates if there are conflicting letters, i.e. two words positioned in a way that the overlapping
    /// letters are different.
    has_conflict: bool,
}

const CONFLICT_CHAR: char = '!';

enum CharInsertResult {
    /// First means that the inserted character was the first character at position.
    First,
    /// Taken means that there was already a character at the position.
    Taken,
}

impl CharMap {
    fn new() -> CharMap {
        CharMap {
            pos_to_char: HashMap::new(),
            char_to_pos: HashMap::new(),
            top_left: None,
            bottom_right: None,
            has_conflict: false,
        }
    }

    fn insert_word(&mut self, word: &str, pos: XY, orient: Orientation) -> LayoutResult<usize> {
        let mut pos = pos;
        let mut is_conflict = false;
        let mut crossing_count: usize = 0;
        for curr_char in word.chars() {
            match self.insert_char(pos, curr_char) {
                Ok(r) => match r {
                    CharInsertResult::First => (),
                    CharInsertResult::Taken => crossing_count += 1,
                },
                Err(_) => is_conflict = true,
            }
            pos = pos + orient.step();
        }
        match is_conflict {
            true => Err(()),
            false => Ok(crossing_count),
        }
    }

    fn insert_char(&mut self, pos: XY, curr_char: char) -> LayoutResult<CharInsertResult> {
        match self.pos_to_char.get(&pos) {
            Some(char_at_pos) => {
                if *char_at_pos == curr_char {
                    // Same char, all good, carry on.
                    Ok(CharInsertResult::Taken)
                } else {
                    self.has_conflict = true;
                    self.pos_to_char.insert(pos, CONFLICT_CHAR);
                    // On conflict, char_to_pos breaks. It does not strictly reflect the positions of the characters.
                    self.update_corners(pos);
                    Err(())
                }
            }
            None => {
                let prev = self.pos_to_char.insert(pos, curr_char);
                let positions_for_char = if let Some(v) = self.char_to_pos.get_mut(&curr_char) {
                    v
                } else {
                    self.char_to_pos.insert(curr_char, Vec::new());
                    self.char_to_pos.get_mut(&curr_char).unwrap()
                };
                positions_for_char.push(pos);
                self.update_corners(pos);
                assert_eq!(prev, None);
                Ok(CharInsertResult::First)
            }
        }
    }

    fn update_corners(&mut self, pos: XY) {
        self.top_left = Some(if let Some(top_left) = self.top_left {
            XY {
                x: min(pos.x, top_left.x),
                y: min(pos.y, top_left.y),
            }
        } else {
            pos
        });

        self.bottom_right = Some(if let Some(bottom_right) = self.bottom_right {
            XY {
                x: max(pos.x, bottom_right.x),
                y: max(pos.y, bottom_right.y),
            }
        } else {
            pos
        });
    }

    fn normalize(&self) -> CharMap {
        let mut grid = CharMap::new();
        if self.is_empty() {
            return grid;
        }
        let top_left = self.top_left.unwrap();
        let bottom_right = self.bottom_right.unwrap();
        for (pos, c) in self.pos_to_char.iter() {
            let _ = grid.insert_char(*pos - top_left, *c);
        }
        assert_eq!(grid.top_left.unwrap(), XY { x: 0, y: 0 });
        assert_eq!(grid.bottom_right.unwrap(), bottom_right - top_left);
        grid
    }

    fn is_empty(&self) -> bool {
        if self.pos_to_char.is_empty() {
            assert!(self.char_to_pos.is_empty());
            assert!(self.top_left.is_none());
            assert!(self.bottom_right.is_none());
            true
        } else {
            assert!(!self.char_to_pos.is_empty());
            assert!(self.top_left.is_some());
            assert!(self.bottom_right.is_some());
            false
        }
    }

    fn is_pos_taken(&self, pos: &XY) -> bool {
        self.pos_to_char.contains_key(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::Layout;
    use super::Orientation;

    #[test]
    fn test_insert_and_display() {
        let mut layout = Layout::new();
        layout.insert_at("xab", (0, 0), Orientation::Horiz).unwrap();
        layout.insert_at("xyz", (0, 0), Orientation::Vert).unwrap();
        let actual = format!("{:_>0}", layout);
        let expected = "
            xab
            y__
            z__
        ";
        let expected = expected.trim().replace(" ", "");
        eprintln!("expected:\n{}\n", expected);
        eprintln!("actual:\n{}\n", actual);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_insert_conflict() {
        let mut layout = Layout::new();
        let _ = layout.insert_at("xab", (0, 0), Orientation::Horiz);
        let _ = layout.insert_at("xyz", (2, 0), Orientation::Vert);
        let formatted = format!("{:_>0}", layout);
        eprintln!("{}", formatted);
        let expected = "
            xa!
            __y
            __z
        ";
        let expected = expected.trim().replace(" ", "");
        assert_eq!(formatted, expected)
    }

    #[test]
    fn test_count_crossings() {
        let mut layout = Layout::new();
        layout.insert_at("abc", (0, 0), Orientation::Horiz).unwrap();
        assert_eq!(layout.n_crossings, 0);
        layout.insert_at("abc", (0, 0), Orientation::Vert).unwrap();
        assert_eq!(layout.n_crossings, 1);
        layout.insert_at("cba", (2, 0), Orientation::Vert).unwrap();
        assert_eq!(layout.n_crossings, 2);
        layout.insert_at("cba", (0, 2), Orientation::Horiz).unwrap();
        assert_eq!(layout.n_crossings, 4);
        layout
            .insert_at("xabcx", (-1, 0), Orientation::Horiz)
            .unwrap();
        assert_eq!(layout.n_crossings, 7);
    }

    #[test]
    fn test_layouts_produce_same_output() {
        let mut layout1 = Layout::new();
        layout1
            .insert_at("xab", (-9, 4), Orientation::Horiz)
            .unwrap();
        layout1
            .insert_at("xyz", (-9, 4), Orientation::Vert)
            .unwrap();
        let formatted1 = format!("{:_>0}", layout1);
        eprintln!("{}", formatted1);

        let mut layout2 = Layout::new();
        layout2
            .insert_at("xab", (5, -2), Orientation::Horiz)
            .unwrap();
        layout2
            .insert_at("xyz", (5, -2), Orientation::Vert)
            .unwrap();
        let formatted2 = format!("{:_>0}", layout2);
        eprintln!("{}", formatted2);

        assert_eq!(formatted1, formatted2)
    }
}
