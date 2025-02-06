use std::cmp;

use super::Layout;

fn compare_area(some: &Layout, other: &Layout) -> cmp::Ordering {
    some.area().cmp(&other.area()).reverse()
}

fn compare_crossings(some: &Layout, other: &Layout) -> cmp::Ordering {
    some.crossings_count().cmp(&&other.crossings_count())
}

pub struct UltimateComparator;

pub struct CrosswordGenerator {
    comparators: Vec<Box<dyn Fn(&Layout, &Layout) -> cmp::Ordering>>,
}

pub enum GeneratorMode {
    /// Automatic mode generates crossword deterministically regardless of the input order.
    /// Ideally, the crossword would be "optimal" w.r.t. to some measure (it's not, it's greedy).
    Automatic,
    /// Build the next state of the Layout based on the order of inputs. This is more useful
    /// for interactive development.
    InputOrder,
}

impl CrosswordGenerator {
    pub fn new(
        comparators: Vec<Box<dyn Fn(&Layout, &Layout) -> cmp::Ordering>>,
    ) -> CrosswordGenerator {
        CrosswordGenerator { comparators }
    }

    pub fn generate_crossword(&self, words: &Vec<&str>, mode: GeneratorMode) -> Layout {
        let mut layout = Layout::new();
        let mut words = words.clone();
        if let GeneratorMode::Automatic = mode {
            // Place larger first. Otherwise one will end up with a tiny shape that cannot be extended.
            words.sort_by(|a, b| b.len().cmp(&a.len()));
        }

        while !words.is_empty() {
            // eprintln!("Now word: {word}");
            let mut best_layout_with_word: Option<(Layout, usize)> = None;
            let word_iter: Box<dyn Iterator<Item = &&str>> = match mode {
                GeneratorMode::Automatic => Box::new(words.iter()),
                GeneratorMode::InputOrder => Box::new(words.iter().take(1)),
            };
            for (i_word, word) in word_iter.enumerate() {
                eprintln!("now try word {word}");
                for (pos, orientation) in layout.get_possible_start_positions(word) {
                    eprintln!("Now trying {word:?} {pos} {orientation}");
                    let mut updated_layout = layout.clone();
                    updated_layout.insert_at(word, pos, orientation).unwrap();
                    best_layout_with_word = if let Some((best_layout, best_i_word)) =
                        best_layout_with_word
                    {
                        // Try if this results in better layout:
                        // - Choosing random among equal
                        // - Use different metric, e.g. how "squarish" the layout is
                        if self.compare(&updated_layout, &best_layout) == cmp::Ordering::Greater {
                            eprintln!(
                                "Tried this word {word:?} and is better {pos} {orientation}"
                            );
                            //eprintln!("Before\n{}\n", best_layout);
                            //eprintln!("Updated:\n{}\n", updated_layout);
                            Some((updated_layout, i_word))
                        } else {
                            // eprintln!("Tried this and is same or worse:\n{}", updated_layout);
                            Some((best_layout, best_i_word))
                        }
                    } else {
                        eprintln!(
                            "This is the best because no other candidate pos={pos} or={orientation}:\n{}",
                            updated_layout
                        );
                        Some((updated_layout, i_word))
                    };
                }
            }
            if let Some((best_layout, best_i_word)) = best_layout_with_word {
                layout = best_layout;
                words.remove(best_i_word);
            } else {
                eprintln!("Failed to insert words: {words:?}");
                break;
            }
        }
        layout.normalize()
    }

    fn compare(&self, candidate: &Layout, current_best: &Layout) -> cmp::Ordering {
        for comparator in &self.comparators {
            match comparator(candidate, current_best) {
                cmp::Ordering::Less => return cmp::Ordering::Less,
                cmp::Ordering::Equal => (),
                cmp::Ordering::Greater => return cmp::Ordering::Greater,
            }
        }
        cmp::Ordering::Equal
    }
}

pub fn generate_crossword(words: &Vec<&str>, mode: GeneratorMode) -> Layout {
    let comparators: Vec<Box<dyn Fn(&Layout, &Layout) -> cmp::Ordering>>;
    comparators = match mode {
        // In automatic mode, we don't want to optimize for area because we would end up with tiny initial crossword that would not extend.
        GeneratorMode::Automatic => vec![Box::new(compare_crossings)],
        GeneratorMode::InputOrder => vec![Box::new(compare_crossings), Box::new(compare_area)],
    };
    CrosswordGenerator::new(comparators).generate_crossword(words, mode)
}
