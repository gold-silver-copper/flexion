//! The dictionary's forms no declension gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 38 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"erstbester	0	am erstbesten
sagend	0	ist sagend
schaffend	0	ist schaffend
schauernd	0	ist schauernd
scherend	0	ist scherend
schielend	0	ist schielend
schimpfend	0	ist schimpfend
schlafend	0	ist schlafend
schleichend	0	ist schleichend
schleppend	0	ist schleppend
schlotternd	0	ist schlotternd
schnaufend	0	ist schnaufend
schreiend	0	ist schreiend
schwarzgebrannt	0	ist schwarzgebrannt
schwarzgefahren	0	ist schwarzgefahren
schweigend	0	ist schweigend
schwingend	0	ist schwingend
schwächend	0	ist schwächend
sehend	0	ist sehend
seiend	0	ist seiend
sichergestellt	0	ist sichergestellt
sichernd	0	ist sichernd
sicherstellend	0	ist sicherstellend
sollend	0	ist sollend
sperrend	0	ist sperrend
spielend	0	ist spielend
sprechend	0	ist sprechend
sprenkelnd	0	ist sprenkelnd
springend	0	ist springend
spähend	0	ist spähend
teilnehmend	0	ist teilnehmend
tobend	0	ist tobend
tosend	0	ist tosend
trinkend	0	ist trinkend
tränend	0	ist tränend
träumend	0	ist träumend
tuend	0	ist tuend
tötend	0	ist tötend
"#,
    ),
};
