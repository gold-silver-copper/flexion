//! What no declension gives a word, as the dictionary writes it: a line per word,
//! sorted, the word, then each stem it lists, by its slot (see `stems.rs`):
//! the slot's name and the stem with the ending table its slot's cells are
//! built with, then each listed form, by its cell's number (see
//! `super::cell`), all tab-separated. They are the answer for that word.

use flexion::Irregular;
use std::borrow::Cow;

/// 0 stems, 144 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"dwieście	6	dwustu	7	dwustu	36	dwustu	37	dwustu
jeden	2	jedna	3	jedna	4	jedno	6	jedni	7	jedni	8	jedne	10	jednego	11	jednego	12	jednej	13	jednej	14	jednego	15	jednego	16	jednych	17	jednych	18	jednych	19	jednych	20	jednemu	21	jednemu	22	jednej	23	jednej	24	jednemu	25	jednemu	26	jednym	27	jednym	28	jednym	29	jednym	30	jednego	32	jedną	33	jedną	36	jednych	37	jednych	50	jednym	51	jednym	52	jednej	53	jednej	54	jednym	55	jednym	56	jednych	57	jednych	58	jednych	59	jednych
osiem	6	ośmiu	7	ośmiu	36	ośmiu	37	ośmiu
osiemset	6	ośmiuset	7	ośmiuset	36	ośmiuset	37	ośmiuset
pińcet	6	pińciuset	7	pińciuset	36	pińciuset	37	pińciuset
siedemset	6	siedmiuset	7	siedmiuset	36	siedmiuset	37	siedmiuset
sto	6	stu	7	stu	36	stu	37	stu
szereg	2	szereg	3	szereg	6	szereg	7	szereg	10	szeregu	11	szeregu	12	szeregu	13	szeregu	14	szeregu	15	szeregu	16	szeregu	17	szeregu	18	szeregu	19	szeregu	20	szeregowi	21	szeregowi	22	szeregowi	23	szeregowi	24	szeregowi	25	szeregowi	26	szeregowi	27	szeregowi	28	szeregowi	29	szeregowi	32	szereg	33	szereg	36	szereg	37	szereg	48	szeregiem	50	szeregu	51	szeregu	52	szeregu	53	szeregu	54	szeregu	55	szeregu	56	szeregu	57	szeregu	58	szeregu	59	szeregu
wtóry	2	wtóra	3	wtóra	4	wtóre	6	wtórzy	7	wtórzy	10	wtórego	11	wtórego	12	wtórej	13	wtórej	14	wtórego	15	wtórego	16	wtórych	17	wtórych	18	wtórych	19	wtórych	20	wtóremu	21	wtóremu	22	wtórej	23	wtórej	24	wtóremu	25	wtóremu	26	wtórym	27	wtórym	28	wtórym	29	wtórym	30	wtórego	32	wtórą	33	wtórą	36	wtórych	37	wtórych	50	wtórym	51	wtórym	52	wtórej	53	wtórej	54	wtórym	55	wtórym	56	wtórych	57	wtórych	58	wtórych	59	wtórych
"#,
    ),
};
