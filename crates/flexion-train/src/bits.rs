//! Description length: what a grammar costs to write down, in bits, so that
//! every structural choice (a paradigm, a table, a referral, a listed form)
//! is kept only where it pays for itself.
//!
//! A printed item is a stream of tokens: letters of its patterns and forms,
//! and references to cells, paradigms or other items. A token costs log₂ of
//! the number of choices it is made from (Rasin, Berger, Lan & Katzir 2021
//! fn. 12: log₂ n, not rounded up). An item also pays a fixed cost for its
//! name and evidence ([`ITEM`]).

/// The fixed cost of an item: its name and its evidence (word counts).
pub const ITEM: f64 = 32.0;

/// The prices of a part of speech's tokens.
#[derive(Debug, Clone, Copy)]
pub struct Prices {
    /// One letter (of the letters its forms are written with, plus the end).
    pub letter: f64,
    /// One cell (of the part of speech's cells).
    pub cell: f64,
}

impl Prices {
    /// The prices for forms written with `letters` distinct letters and a
    /// part of speech of `cells` cells.
    pub fn new(letters: usize, cells: usize) -> Prices {
        Prices {
            letter: log2(letters + 1),
            cell: log2(cells.max(1)),
        }
    }

    /// A pattern or a form: its letters and its end (a stem slot `~` is a
    /// letter like any other).
    pub fn text(&self, text: &str) -> f64 {
        // The count is small; the conversion is exact.
        #[allow(clippy::cast_precision_loss)]
        let n = (text.chars().count() + 1) as f64;
        n * self.letter
    }

    /// A word's form of a cell, listed: which cell, and the form.
    pub fn listed(&self, form: &str) -> f64 {
        self.cell + self.text(form)
    }
}

/// log₂ of `n` choices (0 for one or none).
pub fn log2(n: usize) -> f64 {
    // A count of choices; the conversion is exact below 2⁵³.
    #[allow(clippy::cast_precision_loss)]
    let n = n as f64;
    if n <= 1.0 { 0.0 } else { n.log2() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_listed_form_costs_more_than_its_pattern() {
        let p = Prices::new(30, 12);
        // Stating `~ам` for every word is cheaper than listing `книгам`.
        assert!(p.text("~ам") < p.listed("книгам"));
        assert_eq!(log2(1), 0.0);
        assert_eq!(log2(8), 3.0);
    }
}
