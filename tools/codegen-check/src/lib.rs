//! Code generated (by `build.rs`) from the test fixtures' models; the test
//! checks that it answers exactly as the in-memory models do.

#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod english {
    include!(concat!(env!("OUT_DIR"), "/english.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod russian {
    include!(concat!(env!("OUT_DIR"), "/russian.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod latin {
    include!(concat!(env!("OUT_DIR"), "/latin.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod latin_front {
    include!(concat!(env!("OUT_DIR"), "/latin_front.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod polish {
    include!(concat!(env!("OUT_DIR"), "/polish.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod german {
    include!(concat!(env!("OUT_DIR"), "/german.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod german_joins {
    include!(concat!(env!("OUT_DIR"), "/german_joins.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod ancient_greek {
    include!(concat!(env!("OUT_DIR"), "/ancient_greek.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod sanskrit {
    include!(concat!(env!("OUT_DIR"), "/sanskrit.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod spanish {
    include!(concat!(env!("OUT_DIR"), "/spanish.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod french {
    include!(concat!(env!("OUT_DIR"), "/french.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod italian {
    include!(concat!(env!("OUT_DIR"), "/italian.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod russian_laws {
    include!(concat!(env!("OUT_DIR"), "/russian_laws.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod polish_laws {
    include!(concat!(env!("OUT_DIR"), "/polish_laws.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod german_laws {
    include!(concat!(env!("OUT_DIR"), "/german_laws.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod ancient_greek_laws {
    include!(concat!(env!("OUT_DIR"), "/ancient_greek_laws.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod russian_derived {
    include!(concat!(env!("OUT_DIR"), "/russian_derived.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod latin_derived {
    include!(concat!(env!("OUT_DIR"), "/latin_derived.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod ancient_greek_derived {
    include!(concat!(env!("OUT_DIR"), "/ancient_greek_derived.rs"));
}
#[allow(clippy::approx_constant, clippy::excessive_precision, clippy::unreadable_literal, dead_code)]
#[rustfmt::skip]
mod handmade {
    include!(concat!(env!("OUT_DIR"), "/handmade.rs"));
}

/// The generated model of fixture case `module`.
pub fn model(module: &str) -> Option<&'static flexion::Model<'static>> {
    use std::sync::OnceLock;
    static MODELS: OnceLock<Vec<(&'static str, flexion::Model<'static>)>> = OnceLock::new();
    let models = MODELS.get_or_init(|| {
        vec![
            ("english", english::build()),
            ("russian", russian::build()),
            ("latin", latin::build()),
            ("latin_front", latin_front::build()),
            ("polish", polish::build()),
            ("german", german::build()),
            ("german_joins", german_joins::build()),
            ("ancient_greek", ancient_greek::build()),
            ("sanskrit", sanskrit::build()),
            ("spanish", spanish::build()),
            ("french", french::build()),
            ("italian", italian::build()),
            ("russian_laws", russian_laws::build()),
            ("polish_laws", polish_laws::build()),
            ("german_laws", german_laws::build()),
            ("ancient_greek_laws", ancient_greek_laws::build()),
            ("russian_derived", russian_derived::build()),
            ("latin_derived", latin_derived::build()),
            ("ancient_greek_derived", ancient_greek_derived::build()),
            ("handmade", handmade::build()),
        ]
    });
    models.iter().find(|(m, _)| *m == module).map(|(_, m)| m)
}
