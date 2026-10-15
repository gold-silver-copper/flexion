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
        law5 = ["ὀ" => "ὠ"];
        law6 = ["ἀ" => "ἠ", "α" => "η", "ᾰ" => "η", "ᾱ" => "η", "ᾳ" => "η"];
        law9 = ["ῥ" => "ἐρρ"];
        law10 = ["οἰ" => "ᾠ"];
        law12 = ["ἁ" => "ἡ", "ᾰ" => "η", "ᾱ" => "η", "ᾳ" => "η", "ὁ" => "ὡ"];
        law13 = ["σκ" => "ἐσκ"];
        law14 = ["ὀ" => "ὠ", "ο" => "ω"];
        law15 = ["ε" => "ε"];
        law16 = ["ἱ" => "ἱ", "ι" => "ι", "ϊ" => "ι", "ῐ" => "ι", "ῑ" => "ι"];
        law17 = ["ὑ" => "ὑ", "υ" => "υ", "ϋ" => "υ", "ῠ" => "υ", "ῡ" => "υ"];
        law19 = ["ἰ" => "ἰ", "ι" => "ι", "ϊ" => "ι", "ῐ" => "ι", "ῑ" => "ι"];
        law20 = ["ὑ" => "ὑ", "υ" => "υ", "ϋ" => "υ", "ῠ" => "υ", "ῡ" => "υ"];
        law22 = ["ζ" => "ἐζ"];
        law23 = ["ἐ" => "ἠ", "ε" => "η"];
        law25 = ["αἰ" => "ᾐ"];
        law26 = ["ζ" => "ἐζ"];
        law28 = ["σκ" => "ἐσκ"];
        law29 = ["ὑ" => "ὑ", "υ" => "υ", "ϋ" => "υ", "ῠ" => "υ", "ῡ" => "υ"];
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
        // ἐ‹ὀ → ὠ: 28 words (ὀρχεομαι, ὀνομᾰζω, ὀζω, ὀρεγω), 0 forms wrong.
        law5: "^ἐ‹{x:[ὀ]}" => "{x|law5}";
        // ἐ%ε‹ἀ → ἠ (α → η, ᾰ → η, ᾱ → η, ᾳ → η): 9 words (ἀγᾰπᾰω, ἀσπᾰζομαι, ἀμαυροω, ἀρτᾰω), 0 forms wrong.
        law6: "^ἐ°ἀε‹{x:[ἀ|α|ᾰ|ᾱ|ᾳ]}" => "{x|law6}";
        // ζ+σ → +σ: 14 words (σκᾰνδᾰλῐζω, δειγμᾰτῐζω, γογγῠζω, βλυζω), 0 forms wrong.
        law7: "ζ+σ" => "+σ";
        // χ+σ → +ξ: 14 words (ψῡχω, δεχομαι, βρεχω, εὐχομαι), 0 forms wrong.
        law8: "χ+σ" => "+ξ";
        // ἐ‹ῥ → ἐρρ: 15 words (ῥεω, ῥηγνῡμῐ, ῥωννῡμῐ, ῥοφεω), 0 forms wrong.
        law9: "^ἐ‹{x:[ῥ]}" => "{x|law9}";
        // ἐ‹οἰ → ᾠ: 13 words (οἰφω, οἰκεω, οἰδεω, οἰκοδομεω), 0 forms wrong.
        law10: "^ἐ‹{x:[οἰ]}" => "{x|law10}";
        // φ+σ → +ψ: 10 words (μεμφομαι, οἰφω, ἐρεφω, νῑφω), 0 forms wrong.
        law11: "φ+σ" => "+ψ";
        // ἐ‹ἁ → ἡ (ᾰ → η, ᾱ → η, ᾳ → η, ὁ → ὡ): 20 words (ἁπτω, ἁρπᾰζω, ἁγῐζω, ἁλῐζω), 0 forms wrong.
        law12: "^ἐ‹{x:[ἁ|ᾰ|ᾱ|ᾳ|ὁ]}" => "{x|law12}";
        // %ε‹σκ → ἐσκ: 10 words (σκοπεω, σκηπτω, σκευᾰζω, σκεπᾰζω), 0 forms wrong.
        law13: "^°σε‹{x:[σκ]}" => "{x|law13}";
        // %ε‹ὀ → ὠ (ο → ω): 7 words (ὀρχεομαι, ὀζω, ὀρεγω, ὀφειλω), 0 forms wrong.
        law14: "^°ὀε‹{x:[ὀ|ο]}" => "{x|law14}";
        // ἐ‹ε → ε: 7 words (εἰδομαι, εὑω, εὐδοκεω, εἰσοραω), 0 forms wrong.
        law15: "^ἐ‹{x:[ε]}" => "{x|law15}";
        // ἐ‹ἱ → ἱ (ι → ι, ϊ → ι, ῐ → ι, ῑ → ι): 8 words (ἱημῐ, ἱδροω, ἱζω, ἱκνεομαι), 0 forms wrong.
        law16: "^ἐ‹{x:[ἱ|ι|ϊ|ῐ|ῑ]}" => "{x|law16}";
        // ἐ‹ὑ → ὑ (υ → υ, ϋ → υ, ῠ → υ, ῡ → υ): 9 words (ὑλακτεω, ὑφαινω, ὑμνεω, ὑστερεω), 0 forms wrong.
        law17: "^ἐ‹{x:[ὑ|υ|ϋ|ῠ|ῡ]}" => "{x|law17}";
        // κ+σ → +ξ: 7 words (δερκομαι, ἑλκω, ἱκω, εἰκω), 0 forms wrong.
        law18: "κ+σ" => "+ξ";
        // ἐ‹ἰ → ἰ (ι → ι, ϊ → ι, ῐ → ι, ῑ → ι): 8 words (ἰδῑω, ἰθῡνω, ἰαυω, ἰσχω), 0 forms wrong.
        law19: "^ἐ‹{x:[ἰ|ι|ϊ|ῐ|ῑ]}" => "{x|law19}";
        // %ε‹ὑ → ὑ (υ → υ, ϋ → υ, ῠ → υ, ῡ → υ): 7 words (ὑμνεω, ὑστερεω, ὑβρῐζω, ὑᾰλῐζω), 0 forms wrong.
        law20: "^°ὑε‹{x:[ὑ|υ|ϋ|ῠ|ῡ]}" => "{x|law20}";
        // ζ+θ → +σθ: 6 words (σκᾰνδᾰλῐζω, βλυζω, λᾰλᾰζω, μηδῐζω), 0 forms wrong.
        law21: "ζ+θ" => "+σθ";
        // %ε‹ζ → ἐζ: 7 words (ζεω, ζητεω, ζηλοω, ζευγνῡμῐ), 0 forms wrong.
        law22: "^°ζε‹{x:[ζ]}" => "{x|law22}";
        // %ε‹ἐ → ἠ (ε → η): 5 words (ἐρεφω, ἐλεεω, ἐπειγω, ἐρημοω), 0 forms wrong.
        law23: "^°ἐε‹{x:[ἐ|ε]}" => "{x|law23}";
        // ν+σ → ν+: 6 words (γεινομαι, ἰθῡνω, δῐᾰβαινω, πᾰλῡνω), 10 forms wrong.
        law24: "ν+σ" => "ν+";
        // ἐ‹αἰ → ᾐ: 6 words (αἰθω, αἰτεω, αἰσχῡνω, αἰδεομαι), 0 forms wrong.
        law25: "^ἐ‹{x:[αἰ]}" => "{x|law25}";
        // ἐ%ε‹ζ → ἐζ: 6 words (ζεω, ζητεω, ζηλοω, ζῠγοω), 0 forms wrong.
        law26: "^ἐ°ζε‹{x:[ζ]}" => "{x|law26}";
        // [φ|χ|ν|γ]+ντ → +ᾰτ: 5 words (δεχομαι, ἐρεφω, τευχω, ἀπολεγω), 0 forms wrong.
        law27: "{x:[φ|χ|ν|γ]}+ντ" => "{x}+ᾰτ";
        // ἐ%ε‹σκ → ἐσκ: 5 words (σκοπεω, σκευᾰζω, σκεπᾰζω, σκορπῐζω), 0 forms wrong.
        law28: "^ἐ°σε‹{x:[σκ]}" => "{x|law28}";
        // ἐ%ε‹ὑ → ὑ (υ → υ, ϋ → υ, ῠ → υ, ῡ → υ): 5 words (ὑμνεω, ὑστερεω, ὑβρῐζω, ὑᾰλῐζω), 0 forms wrong.
        law29: "^ἐ°ὑε‹{x:[ὑ|υ|ϋ|ῠ|ῡ]}" => "{x|law29}";
        // [υ|ι|ε|ῡ|ῠ|ῑ|ᾱ|ὑ|ω]+θ → +σθ after [α|ζ|ξ]: 7 words (παυω, ξῡω, ζωννῡμῐ, πταιω), 0 forms wrong.
        law30: "{x:[υ|ι|ε|ῡ|ῠ|ῑ|ᾱ|ὑ|ω]}+θ" => "{x}+σθ" / "{[α|ζ|ξ]}" _ "";
    }
};
