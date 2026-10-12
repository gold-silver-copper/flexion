//! The dictionary's forms no conjugation gives, as it writes them: a line per form,
//! the word, its cell's number (see `super::cell`) and the form, sorted by word
//! and cell. They are the answer for that word and cell.

use flexion::Irregular;
use std::borrow::Cow;

/// 22 forms.
pub(super) const IRREGULAR: Irregular<'static> = Irregular {
    table: Cow::Borrowed(
        r#"алчу	3	алкая
вмклокочусь	1	всклокочутся
движу	3	двигая
долродаюсь	3	допродаваясь
еду	3	едучи
жаждусь	3	жаждаясь
застонаю	1	застонут
колышу	3	колыхая
колышусь	3	колыхаясь
лежу	3	лёжа
наплещу	1	паплещут
обмохначюсь	1	обмохнатятся
объемлю	1	объемлят
отстрекочу	1	отастрекочут
отфренджу	1	отфрендят
переравлю	1	перетравят
побижусь	1	пообидятся
полулежу	3	полулёжа
поотверчу	1	поовертят
сахарю	1	сахарат
умерщвлю	1	умертвят
хочу	3	желая
"#,
    ),
};
