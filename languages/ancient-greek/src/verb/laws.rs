//! Sound laws where a stem meets its ending (`+`), where a form's first
//! literal meets the stem (`‹`), on a letter a `%` copies (`°`), on a class
//! letter an ending writes (harmony, by the nearest letter of a tier before
//! it as written), or inside a stem that alternates (`ˇ`, by what follows
//! it): every form a conjugation builds goes through them, in order, so one conjugation
//! serves words whose stems change its endings. Each with the words whose
//! forms it gives, and how many of the dictionary's forms it gets wrong
//! (those are listed in `irregular.rs`). Written in `sound-law`'s notation
//! (`"target" => "replacement" / "before" _ "after"`; `{x|law3}`, a
//! captured letter through that law's map) and compiled when this crate is
//! built.

use flexion::sound_law::Program;

/// The laws, in order.
pub(super) const LAWS: Program<'static> = flexion::laws! {
    classes {}
    maps {
        law0 = ["ἀ" => "ἠ", "ᾰ" => "η", "ᾱ" => "η", "ᾳ" => "η"];
        law1 = ["ἀ" => "ἠ", "α" => "η", "ᾰ" => "η", "ᾱ" => "η", "ᾳ" => "η"];
        law5 = ["ὀ" => "ὠ", "ῥ" => "ἐρρ"];
        law6 = ["ἀ" => "ἠ", "α" => "η", "ᾰ" => "η", "ᾱ" => "η", "ᾳ" => "η"];
        law9 = ["οἰ" => "ᾠ", "ἁ" => "ἡ", "ᾰ" => "η", "ᾱ" => "η", "ᾳ" => "η", "ὁ" => "ὡ", "ε" => "ε", "ἱ" => "ἱ", "ι" => "ι", "ϊ" => "ι", "ῐ" => "ι", "ῑ" => "ι", "ὑ" => "ὑ", "υ" => "υ", "ϋ" => "υ", "ῠ" => "υ", "ῡ" => "υ"];
        law11 = ["σκ" => "ἐσκ"];
        law12 = ["ὀ" => "ὠ", "ο" => "ω"];
        law14 = ["ἰ" => "ἰ", "ι" => "ι", "ϊ" => "ι", "ῐ" => "ι", "ῑ" => "ι"];
        law15 = ["ὑ" => "ὑ", "υ" => "υ", "ϋ" => "υ", "ῠ" => "υ", "ῡ" => "υ"];
        law17 = ["ζ" => "ἐζ"];
        law18 = ["ἐ" => "ἠ", "ε" => "η"];
        law20 = ["αἰ" => "ᾐ"];
        law21 = ["ζ" => "ἐζ"];
        law23 = ["σκ" => "ἐσκ"];
        law24 = ["ὑ" => "ὑ", "υ" => "υ", "ϋ" => "υ", "ῠ" => "υ", "ῡ" => "υ"];
    }
    rules {
        // ἐ‹ἀ → ἠ (ᾰ → η, ᾱ → η, ᾳ → η): 88 words (ἀγω, ἀγᾰπᾰω, ἀκουω, ἀγᾰθοποιεω), 0 forms wrong.
        law0: "^ἐ‹{x:[ἀ|ᾰ|ᾱ|ᾳ]}" => "{x|law0}";
        // %ε‹ἀ → ἠ (α → η, ᾰ → η, ᾱ → η, ᾳ → η): 16 words (ἀγω, ἀγᾰπᾰω, ἀκουω, ἀρκεω), 0 forms wrong.
        law1: "^°ἀε‹{x:[ἀ|α|ᾰ|ᾱ|ᾳ]}" => "{x|law1}";
        // copy of φ → π: 33 words (φοβεω, φῐλεω, φενᾱκῐζω, φῠω), 0 forms wrong.
        law2: "°φ" => "π";
        // copy of θ → τ: 32 words (θελω, θερᾰπευω, ἐπεντῐθημῐ, θῡω), 0 forms wrong.
        law3: "°θ" => "τ";
        // copy of χ → κ: 30 words (χᾰρῐζομαι, χειρουργεω, χορευω, χρῑω), 0 forms wrong.
        law4: "°χ" => "κ";
        // ἐ‹ὀ → ὠ (ῥ → ἐρρ): 43 words (ὀρχεομαι, ὀνομᾰζω, ὀζω, ὀρεγω), 0 forms wrong.
        law5: "^ἐ‹{x:[ὀ|ῥ]}" => "{x|law5}";
        // ἐ%ε‹ἀ → ἠ (α → η, ᾰ → η, ᾱ → η, ᾳ → η): 9 words (ἀγᾰπᾰω, ἀσπᾰζομαι, ἀμαυροω, ἀρτᾰω), 0 forms wrong.
        law6: "^ἐ°ἀε‹{x:[ἀ|α|ᾰ|ᾱ|ᾳ]}" => "{x|law6}";
        // ζ+σ → +σ: 14 words (σκᾰνδᾰλῐζω, δειγμᾰτῐζω, γογγῠζω, βλυζω), 0 forms wrong.
        law7: "ζ+σ" => "+σ";
        // χ+σ → +ξ: 14 words (ψῡχω, δεχομαι, βρεχω, εὐχομαι), 0 forms wrong.
        law8: "χ+σ" => "+ξ";
        // ἐ‹οἰ → ᾠ (ἁ → ἡ, ᾰ → η, ᾱ → η, ᾳ → η, ὁ → ὡ, ε → ε, ἱ → ἱ, ι → ι, ϊ → ι, ῐ → ι, ῑ → ι, ὑ → ὑ, υ → υ, ϋ → υ, ῠ → υ, ῡ → υ): 57 words (οἰφω, οἰκεω, οἰδεω, οἰκοδομεω), 0 forms wrong.
        law9: "^ἐ‹{x:[οἰ|ἁ|ᾰ|ᾱ|ᾳ|ὁ|ε|ἱ|ι|ϊ|ῐ|ῑ|ὑ|υ|ϋ|ῠ|ῡ]}" => "{x|law9}";
        // φ+σ → +ψ: 10 words (μεμφομαι, οἰφω, ἐρεφω, νῑφω), 0 forms wrong.
        law10: "φ+σ" => "+ψ";
        // %ε‹σκ → ἐσκ: 10 words (σκοπεω, σκηπτω, σκευᾰζω, σκεπᾰζω), 0 forms wrong.
        law11: "^°σε‹{x:[σκ]}" => "{x|law11}";
        // %ε‹ὀ → ὠ (ο → ω): 7 words (ὀρχεομαι, ὀζω, ὀρεγω, ὀφειλω), 0 forms wrong.
        law12: "^°ὀε‹{x:[ὀ|ο]}" => "{x|law12}";
        // κ+σ → +ξ: 7 words (δερκομαι, ἑλκω, ἱκω, εἰκω), 0 forms wrong.
        law13: "κ+σ" => "+ξ";
        // ἐ‹ἰ → ἰ (ι → ι, ϊ → ι, ῐ → ι, ῑ → ι): 8 words (ἰδῑω, ἰθῡνω, ἰαυω, ἰσχω), 0 forms wrong.
        law14: "^ἐ‹{x:[ἰ|ι|ϊ|ῐ|ῑ]}" => "{x|law14}";
        // %ε‹ὑ → ὑ (υ → υ, ϋ → υ, ῠ → υ, ῡ → υ): 7 words (ὑμνεω, ὑστερεω, ὑβρῐζω, ὑᾰλῐζω), 0 forms wrong.
        law15: "^°ὑε‹{x:[ὑ|υ|ϋ|ῠ|ῡ]}" => "{x|law15}";
        // ζ+θ → +σθ: 6 words (σκᾰνδᾰλῐζω, βλυζω, λᾰλᾰζω, μηδῐζω), 0 forms wrong.
        law16: "ζ+θ" => "+σθ";
        // %ε‹ζ → ἐζ: 7 words (ζεω, ζητεω, ζηλοω, ζευγνῡμῐ), 0 forms wrong.
        law17: "^°ζε‹{x:[ζ]}" => "{x|law17}";
        // %ε‹ἐ → ἠ (ε → η): 5 words (ἐρεφω, ἐλεεω, ἐπειγω, ἐρημοω), 0 forms wrong.
        law18: "^°ἐε‹{x:[ἐ|ε]}" => "{x|law18}";
        // ν+σ → ν+: 6 words (γεινομαι, ἰθῡνω, δῐᾰβαινω, πᾰλῡνω), 10 forms wrong.
        law19: "ν+σ" => "ν+";
        // ἐ‹αἰ → ᾐ: 6 words (αἰθω, αἰτεω, αἰσχῡνω, αἰδεομαι), 0 forms wrong.
        law20: "^ἐ‹{x:[αἰ]}" => "{x|law20}";
        // ἐ%ε‹ζ → ἐζ: 6 words (ζεω, ζητεω, ζηλοω, ζῠγοω), 0 forms wrong.
        law21: "^ἐ°ζε‹{x:[ζ]}" => "{x|law21}";
        // [φ|χ|ν|γ]+ντ → +ᾰτ: 5 words (δεχομαι, ἐρεφω, τευχω, ἀπολεγω), 0 forms wrong.
        law22: "{x:[φ|χ|ν|γ]}+ντ" => "{x}+ᾰτ";
        // ἐ%ε‹σκ → ἐσκ: 5 words (σκοπεω, σκευᾰζω, σκεπᾰζω, σκορπῐζω), 0 forms wrong.
        law23: "^ἐ°σε‹{x:[σκ]}" => "{x|law23}";
        // ἐ%ε‹ὑ → ὑ (υ → υ, ϋ → υ, ῠ → υ, ῡ → υ): 5 words (ὑμνεω, ὑστερεω, ὑβρῐζω, ὑᾰλῐζω), 0 forms wrong.
        law24: "^ἐ°ὑε‹{x:[ὑ|υ|ϋ|ῠ|ῡ]}" => "{x|law24}";
        // [υ|ι|ε|ῡ|ῠ|ῑ|ᾱ|ὑ|ω]+θ → +σθ after [α|ζ|ξ]: 7 words (παυω, ξῡω, ζωννῡμῐ, πταιω), 0 forms wrong.
        law25: "{x:[υ|ι|ε|ῡ|ῠ|ῑ|ᾱ|ὑ|ω]}+θ" => "{x}+σθ" / "{[α|ζ|ξ]}" _ "";
    }
};
