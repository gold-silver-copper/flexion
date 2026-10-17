//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 20 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"amidah	1	amidoth
bacigalupo	1	bacigalupi
cassano	1	cassani
chumash	1	chumashim
dionysos	1	dionysoi
genseric	1	m
hora	1	horae
humash	1	humashim
kallikantzaros	1	kallikantzaroi
lieuchieu	1	lieuchieux
mebarak	1	mubaraks
nunakput	1	m
petorca	1	f
rathwa	1	rathva
snapewife	1	snapewives
superman	1	supermen
toxotes	1	toxotai
trumpenvolk	1	trumpenvoelker
tyche	1	tychai
uccello	1	ucellos
"#,
    ),
};
