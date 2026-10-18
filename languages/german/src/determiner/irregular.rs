//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 39 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"der	1	die	2	das	7	der|deren|derer	11	den|denen
derjenige	1	diejenige	2	dasjenige	4	desjenigen	8	demjenigen	11	denjenigen
derselbe	1	dieselbe	2	dasselbe	4	desselben	8	demselben	11	denselben
einer	0	einer|eine	2	eines|eins|eine	3	einen	4	eines|einen	5	einer|einen	6	eines|einen	7	einen	8	einem|einen	9	einer|einen	10	einem|einen	14	eines|eins|eine
unßer	1	unßere|unßre	3	unßere|unßre	4	unßeres|unßres	5	unßerer|unßrer	6	unßeres|unßres	7	unßerer|unßrer	8	unßerem|unßrem	9	unßerer|unßrer	10	unßerem|unßrem	11	unßeren|unßren	12	unßeren|unßren	13	unßere|unßre	14	unßeres|unßres	15	unßere|unßre
"#,
    ),
};
