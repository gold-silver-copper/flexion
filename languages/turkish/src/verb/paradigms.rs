//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~mak`, `ayırmak`: the stem and `mak`).
//! Alternatives are separated by `|`, the usual one first. A capital (`E`, `I`, `A`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the conjugation it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem converb_neg_plain_doubled: \"~maya ~maya- like ayırmak\"`: the slot's cells are `ayırmak`'s table's patterns
/// with `~maya ~maya` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the conjugation's it is
/// like, or, in a conjugation like no other, derived):
/// `pos_plain_aor__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_aor_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_aor_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_aor_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_prog__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_prog_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_prog_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_prog_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_evid__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_evid_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_evid_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_evid_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_fut__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_fut_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_fut_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_fut_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_cont__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_cont_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_cont_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_cont_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_necess__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_necess_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_necess_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_plain_necess_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_aor__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_aor_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_aor_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_aor_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_prog__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_prog_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_prog_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_prog_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_evid__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_evid_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_evid_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_evid_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_fut__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_fut_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_fut_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_fut_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_cont__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_cont_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_cont_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_cont_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_necess__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_necess_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_necess_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `pos_abil_necess_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_aor__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_aor_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_aor_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_aor_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_prog__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_prog_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_prog_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_prog_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_evid__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_evid_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_evid_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_evid_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_fut__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_fut_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_fut_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_fut_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_cont__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_cont_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_cont_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_cont_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_necess__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_necess_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_necess_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_plain_necess_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_aor__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_aor_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_aor_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_aor_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_prog__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_prog_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_prog_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_prog_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_evid__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_evid_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_evid_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_evid_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_fut__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_fut_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_fut_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_fut_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_cont__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_cont_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_cont_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_cont_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_necess__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_necess_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_necess_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `neg_abil_necess_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `past_pos_plain__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `past_pos_plain_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `past_pos_plain_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `past_pos_abil__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `past_pos_abil_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `past_pos_abil_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `past_neg_plain__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `past_neg_plain_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `past_neg_plain_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `past_neg_abil__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `past_neg_abil_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `past_neg_abil_cond` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `conditional_pos_plain__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `conditional_pos_plain_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `conditional_pos_plain_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `conditional_pos_abil__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `conditional_pos_abil_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `conditional_pos_abil_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `conditional_neg_plain__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `conditional_neg_plain_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `conditional_neg_plain_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `conditional_neg_abil__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `conditional_neg_abil_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `conditional_neg_abil_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `optative_pos_plain__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `optative_pos_plain_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `optative_pos_plain_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `optative_pos_abil__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `optative_pos_abil_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `optative_pos_abil_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `optative_neg_plain__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `optative_neg_plain_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `optative_neg_plain_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `optative_neg_abil__` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `optative_neg_abil_evid` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `optative_neg_abil_past` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `imperative_pos_plain` (_2sg, _3sg, _2pl, _3pl, _2sg_formal, _2pl_formal, _2sg_urging, _2pl_urging)
/// `imperative_pos_abil` (_2sg, _3sg, _2pl, _3pl, _2sg_formal, _2pl_formal, _2sg_urging, _2pl_urging)
/// `imperative_neg_plain` (_2sg, _3sg, _2pl, _3pl, _2sg_formal, _2pl_formal, _2sg_urging, _2pl_urging)
/// `imperative_neg_abil` (_2sg, _3sg, _2pl, _3pl, _2sg_formal, _2pl_formal, _2sg_urging, _2pl_urging)
/// `infinitive_pos` (plain, abil)
/// `infinitive_neg` (plain, abil)
/// `verbal_noun_pos` (plain, abil)
/// `verbal_noun_neg` (plain, abil)
/// `participle_pos_plain` (an, miş, acak)
/// `participle_pos_abil` (an, miş, acak)
/// `participle_neg_plain` (an, miş, acak)
/// `participle_neg_abil` (an, miş, acak)
/// `relative_pos_plain_dik` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `relative_pos_plain_acak` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `relative_pos_abil_dik` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `relative_pos_abil_acak` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `relative_neg_plain_dik` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `relative_neg_plain_acak` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `relative_neg_abil_dik` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `relative_neg_abil_acak` (_1sg, _2sg, _3sg, _1pl, _2pl, _3pl)
/// `converb_pos_plain` (ken, ince, ip, eli, esiye, dikçe, erek, doubled, cesine)
/// `converb_pos_abil` (ken, ince, ip, eli, esiye, dikçe, erek, doubled, cesine)
/// `converb_neg_plain` (ken, ince, ip, eli, esiye, dikçe, erek, doubled, cesine)
/// `converb_neg_abil` (ken, ince, ip, eli, esiye, dikçe, erek, doubled, cesine)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem converb_neg_plain_doubled: $s1:tt;)?
     $(stem converb_neg_abil_doubled: $s2:tt;)?
     $(pos_plain_aor__: $c0:tt, $c1:tt, $c2:tt, $c3:tt, $c4:tt, $c5:tt;)?
     $(pos_plain_aor_evid: $c6:tt, $c7:tt, $c8:tt, $c9:tt, $c10:tt, $c11:tt;)?
     $(pos_plain_aor_past: $c12:tt, $c13:tt, $c14:tt, $c15:tt, $c16:tt, $c17:tt;)?
     $(pos_plain_aor_cond: $c18:tt, $c19:tt, $c20:tt, $c21:tt, $c22:tt, $c23:tt;)?
     $(pos_plain_prog__: $c24:tt, $c25:tt, $c26:tt, $c27:tt, $c28:tt, $c29:tt;)?
     $(pos_plain_prog_evid: $c30:tt, $c31:tt, $c32:tt, $c33:tt, $c34:tt, $c35:tt;)?
     $(pos_plain_prog_past: $c36:tt, $c37:tt, $c38:tt, $c39:tt, $c40:tt, $c41:tt;)?
     $(pos_plain_prog_cond: $c42:tt, $c43:tt, $c44:tt, $c45:tt, $c46:tt, $c47:tt;)?
     $(pos_plain_evid__: $c48:tt, $c49:tt, $c50:tt, $c51:tt, $c52:tt, $c53:tt;)?
     $(pos_plain_evid_evid: $c54:tt, $c55:tt, $c56:tt, $c57:tt, $c58:tt, $c59:tt;)?
     $(pos_plain_evid_past: $c60:tt, $c61:tt, $c62:tt, $c63:tt, $c64:tt, $c65:tt;)?
     $(pos_plain_evid_cond: $c66:tt, $c67:tt, $c68:tt, $c69:tt, $c70:tt, $c71:tt;)?
     $(pos_plain_fut__: $c72:tt, $c73:tt, $c74:tt, $c75:tt, $c76:tt, $c77:tt;)?
     $(pos_plain_fut_evid: $c78:tt, $c79:tt, $c80:tt, $c81:tt, $c82:tt, $c83:tt;)?
     $(pos_plain_fut_past: $c84:tt, $c85:tt, $c86:tt, $c87:tt, $c88:tt, $c89:tt;)?
     $(pos_plain_fut_cond: $c90:tt, $c91:tt, $c92:tt, $c93:tt, $c94:tt, $c95:tt;)?
     $(pos_plain_cont__: $c96:tt, $c97:tt, $c98:tt, $c99:tt, $c100:tt, $c101:tt;)?
     $(pos_plain_cont_evid: $c102:tt, $c103:tt, $c104:tt, $c105:tt, $c106:tt, $c107:tt;)?
     $(pos_plain_cont_past: $c108:tt, $c109:tt, $c110:tt, $c111:tt, $c112:tt, $c113:tt;)?
     $(pos_plain_cont_cond: $c114:tt, $c115:tt, $c116:tt, $c117:tt, $c118:tt, $c119:tt;)?
     $(pos_plain_necess__: $c120:tt, $c121:tt, $c122:tt, $c123:tt, $c124:tt, $c125:tt;)?
     $(pos_plain_necess_evid: $c126:tt, $c127:tt, $c128:tt, $c129:tt, $c130:tt, $c131:tt;)?
     $(pos_plain_necess_past: $c132:tt, $c133:tt, $c134:tt, $c135:tt, $c136:tt, $c137:tt;)?
     $(pos_plain_necess_cond: $c138:tt, $c139:tt, $c140:tt, $c141:tt, $c142:tt, $c143:tt;)?
     $(pos_abil_aor__: $c144:tt, $c145:tt, $c146:tt, $c147:tt, $c148:tt, $c149:tt;)?
     $(pos_abil_aor_evid: $c150:tt, $c151:tt, $c152:tt, $c153:tt, $c154:tt, $c155:tt;)?
     $(pos_abil_aor_past: $c156:tt, $c157:tt, $c158:tt, $c159:tt, $c160:tt, $c161:tt;)?
     $(pos_abil_aor_cond: $c162:tt, $c163:tt, $c164:tt, $c165:tt, $c166:tt, $c167:tt;)?
     $(pos_abil_prog__: $c168:tt, $c169:tt, $c170:tt, $c171:tt, $c172:tt, $c173:tt;)?
     $(pos_abil_prog_evid: $c174:tt, $c175:tt, $c176:tt, $c177:tt, $c178:tt, $c179:tt;)?
     $(pos_abil_prog_past: $c180:tt, $c181:tt, $c182:tt, $c183:tt, $c184:tt, $c185:tt;)?
     $(pos_abil_prog_cond: $c186:tt, $c187:tt, $c188:tt, $c189:tt, $c190:tt, $c191:tt;)?
     $(pos_abil_evid__: $c192:tt, $c193:tt, $c194:tt, $c195:tt, $c196:tt, $c197:tt;)?
     $(pos_abil_evid_evid: $c198:tt, $c199:tt, $c200:tt, $c201:tt, $c202:tt, $c203:tt;)?
     $(pos_abil_evid_past: $c204:tt, $c205:tt, $c206:tt, $c207:tt, $c208:tt, $c209:tt;)?
     $(pos_abil_evid_cond: $c210:tt, $c211:tt, $c212:tt, $c213:tt, $c214:tt, $c215:tt;)?
     $(pos_abil_fut__: $c216:tt, $c217:tt, $c218:tt, $c219:tt, $c220:tt, $c221:tt;)?
     $(pos_abil_fut_evid: $c222:tt, $c223:tt, $c224:tt, $c225:tt, $c226:tt, $c227:tt;)?
     $(pos_abil_fut_past: $c228:tt, $c229:tt, $c230:tt, $c231:tt, $c232:tt, $c233:tt;)?
     $(pos_abil_fut_cond: $c234:tt, $c235:tt, $c236:tt, $c237:tt, $c238:tt, $c239:tt;)?
     $(pos_abil_cont__: $c240:tt, $c241:tt, $c242:tt, $c243:tt, $c244:tt, $c245:tt;)?
     $(pos_abil_cont_evid: $c246:tt, $c247:tt, $c248:tt, $c249:tt, $c250:tt, $c251:tt;)?
     $(pos_abil_cont_past: $c252:tt, $c253:tt, $c254:tt, $c255:tt, $c256:tt, $c257:tt;)?
     $(pos_abil_cont_cond: $c258:tt, $c259:tt, $c260:tt, $c261:tt, $c262:tt, $c263:tt;)?
     $(pos_abil_necess__: $c264:tt, $c265:tt, $c266:tt, $c267:tt, $c268:tt, $c269:tt;)?
     $(pos_abil_necess_evid: $c270:tt, $c271:tt, $c272:tt, $c273:tt, $c274:tt, $c275:tt;)?
     $(pos_abil_necess_past: $c276:tt, $c277:tt, $c278:tt, $c279:tt, $c280:tt, $c281:tt;)?
     $(pos_abil_necess_cond: $c282:tt, $c283:tt, $c284:tt, $c285:tt, $c286:tt, $c287:tt;)?
     $(neg_plain_aor__: $c288:tt, $c289:tt, $c290:tt, $c291:tt, $c292:tt, $c293:tt;)?
     $(neg_plain_aor_evid: $c294:tt, $c295:tt, $c296:tt, $c297:tt, $c298:tt, $c299:tt;)?
     $(neg_plain_aor_past: $c300:tt, $c301:tt, $c302:tt, $c303:tt, $c304:tt, $c305:tt;)?
     $(neg_plain_aor_cond: $c306:tt, $c307:tt, $c308:tt, $c309:tt, $c310:tt, $c311:tt;)?
     $(neg_plain_prog__: $c312:tt, $c313:tt, $c314:tt, $c315:tt, $c316:tt, $c317:tt;)?
     $(neg_plain_prog_evid: $c318:tt, $c319:tt, $c320:tt, $c321:tt, $c322:tt, $c323:tt;)?
     $(neg_plain_prog_past: $c324:tt, $c325:tt, $c326:tt, $c327:tt, $c328:tt, $c329:tt;)?
     $(neg_plain_prog_cond: $c330:tt, $c331:tt, $c332:tt, $c333:tt, $c334:tt, $c335:tt;)?
     $(neg_plain_evid__: $c336:tt, $c337:tt, $c338:tt, $c339:tt, $c340:tt, $c341:tt;)?
     $(neg_plain_evid_evid: $c342:tt, $c343:tt, $c344:tt, $c345:tt, $c346:tt, $c347:tt;)?
     $(neg_plain_evid_past: $c348:tt, $c349:tt, $c350:tt, $c351:tt, $c352:tt, $c353:tt;)?
     $(neg_plain_evid_cond: $c354:tt, $c355:tt, $c356:tt, $c357:tt, $c358:tt, $c359:tt;)?
     $(neg_plain_fut__: $c360:tt, $c361:tt, $c362:tt, $c363:tt, $c364:tt, $c365:tt;)?
     $(neg_plain_fut_evid: $c366:tt, $c367:tt, $c368:tt, $c369:tt, $c370:tt, $c371:tt;)?
     $(neg_plain_fut_past: $c372:tt, $c373:tt, $c374:tt, $c375:tt, $c376:tt, $c377:tt;)?
     $(neg_plain_fut_cond: $c378:tt, $c379:tt, $c380:tt, $c381:tt, $c382:tt, $c383:tt;)?
     $(neg_plain_cont__: $c384:tt, $c385:tt, $c386:tt, $c387:tt, $c388:tt, $c389:tt;)?
     $(neg_plain_cont_evid: $c390:tt, $c391:tt, $c392:tt, $c393:tt, $c394:tt, $c395:tt;)?
     $(neg_plain_cont_past: $c396:tt, $c397:tt, $c398:tt, $c399:tt, $c400:tt, $c401:tt;)?
     $(neg_plain_cont_cond: $c402:tt, $c403:tt, $c404:tt, $c405:tt, $c406:tt, $c407:tt;)?
     $(neg_plain_necess__: $c408:tt, $c409:tt, $c410:tt, $c411:tt, $c412:tt, $c413:tt;)?
     $(neg_plain_necess_evid: $c414:tt, $c415:tt, $c416:tt, $c417:tt, $c418:tt, $c419:tt;)?
     $(neg_plain_necess_past: $c420:tt, $c421:tt, $c422:tt, $c423:tt, $c424:tt, $c425:tt;)?
     $(neg_plain_necess_cond: $c426:tt, $c427:tt, $c428:tt, $c429:tt, $c430:tt, $c431:tt;)?
     $(neg_abil_aor__: $c432:tt, $c433:tt, $c434:tt, $c435:tt, $c436:tt, $c437:tt;)?
     $(neg_abil_aor_evid: $c438:tt, $c439:tt, $c440:tt, $c441:tt, $c442:tt, $c443:tt;)?
     $(neg_abil_aor_past: $c444:tt, $c445:tt, $c446:tt, $c447:tt, $c448:tt, $c449:tt;)?
     $(neg_abil_aor_cond: $c450:tt, $c451:tt, $c452:tt, $c453:tt, $c454:tt, $c455:tt;)?
     $(neg_abil_prog__: $c456:tt, $c457:tt, $c458:tt, $c459:tt, $c460:tt, $c461:tt;)?
     $(neg_abil_prog_evid: $c462:tt, $c463:tt, $c464:tt, $c465:tt, $c466:tt, $c467:tt;)?
     $(neg_abil_prog_past: $c468:tt, $c469:tt, $c470:tt, $c471:tt, $c472:tt, $c473:tt;)?
     $(neg_abil_prog_cond: $c474:tt, $c475:tt, $c476:tt, $c477:tt, $c478:tt, $c479:tt;)?
     $(neg_abil_evid__: $c480:tt, $c481:tt, $c482:tt, $c483:tt, $c484:tt, $c485:tt;)?
     $(neg_abil_evid_evid: $c486:tt, $c487:tt, $c488:tt, $c489:tt, $c490:tt, $c491:tt;)?
     $(neg_abil_evid_past: $c492:tt, $c493:tt, $c494:tt, $c495:tt, $c496:tt, $c497:tt;)?
     $(neg_abil_evid_cond: $c498:tt, $c499:tt, $c500:tt, $c501:tt, $c502:tt, $c503:tt;)?
     $(neg_abil_fut__: $c504:tt, $c505:tt, $c506:tt, $c507:tt, $c508:tt, $c509:tt;)?
     $(neg_abil_fut_evid: $c510:tt, $c511:tt, $c512:tt, $c513:tt, $c514:tt, $c515:tt;)?
     $(neg_abil_fut_past: $c516:tt, $c517:tt, $c518:tt, $c519:tt, $c520:tt, $c521:tt;)?
     $(neg_abil_fut_cond: $c522:tt, $c523:tt, $c524:tt, $c525:tt, $c526:tt, $c527:tt;)?
     $(neg_abil_cont__: $c528:tt, $c529:tt, $c530:tt, $c531:tt, $c532:tt, $c533:tt;)?
     $(neg_abil_cont_evid: $c534:tt, $c535:tt, $c536:tt, $c537:tt, $c538:tt, $c539:tt;)?
     $(neg_abil_cont_past: $c540:tt, $c541:tt, $c542:tt, $c543:tt, $c544:tt, $c545:tt;)?
     $(neg_abil_cont_cond: $c546:tt, $c547:tt, $c548:tt, $c549:tt, $c550:tt, $c551:tt;)?
     $(neg_abil_necess__: $c552:tt, $c553:tt, $c554:tt, $c555:tt, $c556:tt, $c557:tt;)?
     $(neg_abil_necess_evid: $c558:tt, $c559:tt, $c560:tt, $c561:tt, $c562:tt, $c563:tt;)?
     $(neg_abil_necess_past: $c564:tt, $c565:tt, $c566:tt, $c567:tt, $c568:tt, $c569:tt;)?
     $(neg_abil_necess_cond: $c570:tt, $c571:tt, $c572:tt, $c573:tt, $c574:tt, $c575:tt;)?
     $(past_pos_plain__: $c576:tt, $c577:tt, $c578:tt, $c579:tt, $c580:tt, $c581:tt;)?
     $(past_pos_plain_past: $c582:tt, $c583:tt, $c584:tt, $c585:tt, $c586:tt, $c587:tt;)?
     $(past_pos_plain_cond: $c588:tt, $c589:tt, $c590:tt, $c591:tt, $c592:tt, $c593:tt;)?
     $(past_pos_abil__: $c594:tt, $c595:tt, $c596:tt, $c597:tt, $c598:tt, $c599:tt;)?
     $(past_pos_abil_past: $c600:tt, $c601:tt, $c602:tt, $c603:tt, $c604:tt, $c605:tt;)?
     $(past_pos_abil_cond: $c606:tt, $c607:tt, $c608:tt, $c609:tt, $c610:tt, $c611:tt;)?
     $(past_neg_plain__: $c612:tt, $c613:tt, $c614:tt, $c615:tt, $c616:tt, $c617:tt;)?
     $(past_neg_plain_past: $c618:tt, $c619:tt, $c620:tt, $c621:tt, $c622:tt, $c623:tt;)?
     $(past_neg_plain_cond: $c624:tt, $c625:tt, $c626:tt, $c627:tt, $c628:tt, $c629:tt;)?
     $(past_neg_abil__: $c630:tt, $c631:tt, $c632:tt, $c633:tt, $c634:tt, $c635:tt;)?
     $(past_neg_abil_past: $c636:tt, $c637:tt, $c638:tt, $c639:tt, $c640:tt, $c641:tt;)?
     $(past_neg_abil_cond: $c642:tt, $c643:tt, $c644:tt, $c645:tt, $c646:tt, $c647:tt;)?
     $(conditional_pos_plain__: $c648:tt, $c649:tt, $c650:tt, $c651:tt, $c652:tt, $c653:tt;)?
     $(conditional_pos_plain_evid: $c654:tt, $c655:tt, $c656:tt, $c657:tt, $c658:tt, $c659:tt;)?
     $(conditional_pos_plain_past: $c660:tt, $c661:tt, $c662:tt, $c663:tt, $c664:tt, $c665:tt;)?
     $(conditional_pos_abil__: $c666:tt, $c667:tt, $c668:tt, $c669:tt, $c670:tt, $c671:tt;)?
     $(conditional_pos_abil_evid: $c672:tt, $c673:tt, $c674:tt, $c675:tt, $c676:tt, $c677:tt;)?
     $(conditional_pos_abil_past: $c678:tt, $c679:tt, $c680:tt, $c681:tt, $c682:tt, $c683:tt;)?
     $(conditional_neg_plain__: $c684:tt, $c685:tt, $c686:tt, $c687:tt, $c688:tt, $c689:tt;)?
     $(conditional_neg_plain_evid: $c690:tt, $c691:tt, $c692:tt, $c693:tt, $c694:tt, $c695:tt;)?
     $(conditional_neg_plain_past: $c696:tt, $c697:tt, $c698:tt, $c699:tt, $c700:tt, $c701:tt;)?
     $(conditional_neg_abil__: $c702:tt, $c703:tt, $c704:tt, $c705:tt, $c706:tt, $c707:tt;)?
     $(conditional_neg_abil_evid: $c708:tt, $c709:tt, $c710:tt, $c711:tt, $c712:tt, $c713:tt;)?
     $(conditional_neg_abil_past: $c714:tt, $c715:tt, $c716:tt, $c717:tt, $c718:tt, $c719:tt;)?
     $(optative_pos_plain__: $c720:tt, $c721:tt, $c722:tt, $c723:tt, $c724:tt, $c725:tt;)?
     $(optative_pos_plain_evid: $c726:tt, $c727:tt, $c728:tt, $c729:tt, $c730:tt, $c731:tt;)?
     $(optative_pos_plain_past: $c732:tt, $c733:tt, $c734:tt, $c735:tt, $c736:tt, $c737:tt;)?
     $(optative_pos_abil__: $c738:tt, $c739:tt, $c740:tt, $c741:tt, $c742:tt, $c743:tt;)?
     $(optative_pos_abil_evid: $c744:tt, $c745:tt, $c746:tt, $c747:tt, $c748:tt, $c749:tt;)?
     $(optative_pos_abil_past: $c750:tt, $c751:tt, $c752:tt, $c753:tt, $c754:tt, $c755:tt;)?
     $(optative_neg_plain__: $c756:tt, $c757:tt, $c758:tt, $c759:tt, $c760:tt, $c761:tt;)?
     $(optative_neg_plain_evid: $c762:tt, $c763:tt, $c764:tt, $c765:tt, $c766:tt, $c767:tt;)?
     $(optative_neg_plain_past: $c768:tt, $c769:tt, $c770:tt, $c771:tt, $c772:tt, $c773:tt;)?
     $(optative_neg_abil__: $c774:tt, $c775:tt, $c776:tt, $c777:tt, $c778:tt, $c779:tt;)?
     $(optative_neg_abil_evid: $c780:tt, $c781:tt, $c782:tt, $c783:tt, $c784:tt, $c785:tt;)?
     $(optative_neg_abil_past: $c786:tt, $c787:tt, $c788:tt, $c789:tt, $c790:tt, $c791:tt;)?
     $(imperative_pos_plain: $c792:tt, $c793:tt, $c794:tt, $c795:tt, $c796:tt, $c797:tt, $c798:tt, $c799:tt;)?
     $(imperative_pos_abil: $c800:tt, $c801:tt, $c802:tt, $c803:tt, $c804:tt, $c805:tt, $c806:tt, $c807:tt;)?
     $(imperative_neg_plain: $c808:tt, $c809:tt, $c810:tt, $c811:tt, $c812:tt, $c813:tt, $c814:tt, $c815:tt;)?
     $(imperative_neg_abil: $c816:tt, $c817:tt, $c818:tt, $c819:tt, $c820:tt, $c821:tt, $c822:tt, $c823:tt;)?
     $(infinitive_pos: $c824:tt, $c825:tt;)?
     $(infinitive_neg: $c826:tt, $c827:tt;)?
     $(verbal_noun_pos: $c828:tt, $c829:tt;)?
     $(verbal_noun_neg: $c830:tt, $c831:tt;)?
     $(participle_pos_plain: $c832:tt, $c833:tt, $c834:tt;)?
     $(participle_pos_abil: $c835:tt, $c836:tt, $c837:tt;)?
     $(participle_neg_plain: $c838:tt, $c839:tt, $c840:tt;)?
     $(participle_neg_abil: $c841:tt, $c842:tt, $c843:tt;)?
     $(relative_pos_plain_dik: $c844:tt, $c845:tt, $c846:tt, $c847:tt, $c848:tt, $c849:tt;)?
     $(relative_pos_plain_acak: $c850:tt, $c851:tt, $c852:tt, $c853:tt, $c854:tt, $c855:tt;)?
     $(relative_pos_abil_dik: $c856:tt, $c857:tt, $c858:tt, $c859:tt, $c860:tt, $c861:tt;)?
     $(relative_pos_abil_acak: $c862:tt, $c863:tt, $c864:tt, $c865:tt, $c866:tt, $c867:tt;)?
     $(relative_neg_plain_dik: $c868:tt, $c869:tt, $c870:tt, $c871:tt, $c872:tt, $c873:tt;)?
     $(relative_neg_plain_acak: $c874:tt, $c875:tt, $c876:tt, $c877:tt, $c878:tt, $c879:tt;)?
     $(relative_neg_abil_dik: $c880:tt, $c881:tt, $c882:tt, $c883:tt, $c884:tt, $c885:tt;)?
     $(relative_neg_abil_acak: $c886:tt, $c887:tt, $c888:tt, $c889:tt, $c890:tt, $c891:tt;)?
     $(converb_pos_plain: $c892:tt, $c893:tt, $c894:tt, $c895:tt, $c896:tt, $c897:tt, $c898:tt, $c899:tt, $c900:tt;)?
     $(converb_pos_abil: $c901:tt, $c902:tt, $c903:tt, $c904:tt, $c905:tt, $c906:tt, $c907:tt, $c908:tt, $c909:tt;)?
     $(converb_neg_plain: $c910:tt, $c911:tt, $c912:tt, $c913:tt, $c914:tt, $c915:tt, $c916:tt, $c917:tt, $c918:tt;)?
     $(converb_neg_abil: $c919:tt, $c920:tt, $c921:tt, $c922:tt, $c923:tt, $c924:tt, $c925:tt, $c926:tt, $c927:tt;)?
    ) => {
        Paradigm {
            name: ::std::borrow::Cow::Borrowed($name),
            words: $words,
            lemma: ::std::borrow::Cow::Borrowed($lemma),
            parent: like!($($parent)?),
            reads: view!($($($view)?)?),
            inherent: ::std::borrow::Cow::Borrowed(&[]),
            table: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($c0),)? "\t",
                $(flexion::cell!($c1),)? "\t",
                $(flexion::cell!($c2),)? "\t",
                $(flexion::cell!($c3),)? "\t",
                $(flexion::cell!($c4),)? "\t",
                $(flexion::cell!($c5),)? "\t",
                $(flexion::cell!($c6),)? "\t",
                $(flexion::cell!($c7),)? "\t",
                $(flexion::cell!($c8),)? "\t",
                $(flexion::cell!($c9),)? "\t",
                $(flexion::cell!($c10),)? "\t",
                $(flexion::cell!($c11),)? "\t",
                $(flexion::cell!($c12),)? "\t",
                $(flexion::cell!($c13),)? "\t",
                $(flexion::cell!($c14),)? "\t",
                $(flexion::cell!($c15),)? "\t",
                $(flexion::cell!($c16),)? "\t",
                $(flexion::cell!($c17),)? "\t",
                $(flexion::cell!($c18),)? "\t",
                $(flexion::cell!($c19),)? "\t",
                $(flexion::cell!($c20),)? "\t",
                $(flexion::cell!($c21),)? "\t",
                $(flexion::cell!($c22),)? "\t",
                $(flexion::cell!($c23),)? "\t",
                $(flexion::cell!($c24),)? "\t",
                $(flexion::cell!($c25),)? "\t",
                $(flexion::cell!($c26),)? "\t",
                $(flexion::cell!($c27),)? "\t",
                $(flexion::cell!($c28),)? "\t",
                $(flexion::cell!($c29),)? "\t",
                $(flexion::cell!($c30),)? "\t",
                $(flexion::cell!($c31),)? "\t",
                $(flexion::cell!($c32),)? "\t",
                $(flexion::cell!($c33),)? "\t",
                $(flexion::cell!($c34),)? "\t",
                $(flexion::cell!($c35),)? "\t",
                $(flexion::cell!($c36),)? "\t",
                $(flexion::cell!($c37),)? "\t",
                $(flexion::cell!($c38),)? "\t",
                $(flexion::cell!($c39),)? "\t",
                $(flexion::cell!($c40),)? "\t",
                $(flexion::cell!($c41),)? "\t",
                $(flexion::cell!($c42),)? "\t",
                $(flexion::cell!($c43),)? "\t",
                $(flexion::cell!($c44),)? "\t",
                $(flexion::cell!($c45),)? "\t",
                $(flexion::cell!($c46),)? "\t",
                $(flexion::cell!($c47),)? "\t",
                $(flexion::cell!($c48),)? "\t",
                $(flexion::cell!($c49),)? "\t",
                $(flexion::cell!($c50),)? "\t",
                $(flexion::cell!($c51),)? "\t",
                $(flexion::cell!($c52),)? "\t",
                $(flexion::cell!($c53),)? "\t",
                $(flexion::cell!($c54),)? "\t",
                $(flexion::cell!($c55),)? "\t",
                $(flexion::cell!($c56),)? "\t",
                $(flexion::cell!($c57),)? "\t",
                $(flexion::cell!($c58),)? "\t",
                $(flexion::cell!($c59),)? "\t",
                $(flexion::cell!($c60),)? "\t",
                $(flexion::cell!($c61),)? "\t",
                $(flexion::cell!($c62),)? "\t",
                $(flexion::cell!($c63),)? "\t",
                $(flexion::cell!($c64),)? "\t",
                $(flexion::cell!($c65),)? "\t",
                $(flexion::cell!($c66),)? "\t",
                $(flexion::cell!($c67),)? "\t",
                $(flexion::cell!($c68),)? "\t",
                $(flexion::cell!($c69),)? "\t",
                $(flexion::cell!($c70),)? "\t",
                $(flexion::cell!($c71),)? "\t",
                $(flexion::cell!($c72),)? "\t",
                $(flexion::cell!($c73),)? "\t",
                $(flexion::cell!($c74),)? "\t",
                $(flexion::cell!($c75),)? "\t",
                $(flexion::cell!($c76),)? "\t",
                $(flexion::cell!($c77),)? "\t",
                $(flexion::cell!($c78),)? "\t",
                $(flexion::cell!($c79),)? "\t",
                $(flexion::cell!($c80),)? "\t",
                $(flexion::cell!($c81),)? "\t",
                $(flexion::cell!($c82),)? "\t",
                $(flexion::cell!($c83),)? "\t",
                $(flexion::cell!($c84),)? "\t",
                $(flexion::cell!($c85),)? "\t",
                $(flexion::cell!($c86),)? "\t",
                $(flexion::cell!($c87),)? "\t",
                $(flexion::cell!($c88),)? "\t",
                $(flexion::cell!($c89),)? "\t",
                $(flexion::cell!($c90),)? "\t",
                $(flexion::cell!($c91),)? "\t",
                $(flexion::cell!($c92),)? "\t",
                $(flexion::cell!($c93),)? "\t",
                $(flexion::cell!($c94),)? "\t",
                $(flexion::cell!($c95),)? "\t",
                $(flexion::cell!($c96),)? "\t",
                $(flexion::cell!($c97),)? "\t",
                $(flexion::cell!($c98),)? "\t",
                $(flexion::cell!($c99),)? "\t",
                $(flexion::cell!($c100),)? "\t",
                $(flexion::cell!($c101),)? "\t",
                $(flexion::cell!($c102),)? "\t",
                $(flexion::cell!($c103),)? "\t",
                $(flexion::cell!($c104),)? "\t",
                $(flexion::cell!($c105),)? "\t",
                $(flexion::cell!($c106),)? "\t",
                $(flexion::cell!($c107),)? "\t",
                $(flexion::cell!($c108),)? "\t",
                $(flexion::cell!($c109),)? "\t",
                $(flexion::cell!($c110),)? "\t",
                $(flexion::cell!($c111),)? "\t",
                $(flexion::cell!($c112),)? "\t",
                $(flexion::cell!($c113),)? "\t",
                $(flexion::cell!($c114),)? "\t",
                $(flexion::cell!($c115),)? "\t",
                $(flexion::cell!($c116),)? "\t",
                $(flexion::cell!($c117),)? "\t",
                $(flexion::cell!($c118),)? "\t",
                $(flexion::cell!($c119),)? "\t",
                $(flexion::cell!($c120),)? "\t",
                $(flexion::cell!($c121),)? "\t",
                $(flexion::cell!($c122),)? "\t",
                $(flexion::cell!($c123),)? "\t",
                $(flexion::cell!($c124),)? "\t",
                $(flexion::cell!($c125),)? "\t",
                $(flexion::cell!($c126),)? "\t",
                $(flexion::cell!($c127),)? "\t",
                $(flexion::cell!($c128),)? "\t",
                $(flexion::cell!($c129),)? "\t",
                $(flexion::cell!($c130),)? "\t",
                $(flexion::cell!($c131),)? "\t",
                $(flexion::cell!($c132),)? "\t",
                $(flexion::cell!($c133),)? "\t",
                $(flexion::cell!($c134),)? "\t",
                $(flexion::cell!($c135),)? "\t",
                $(flexion::cell!($c136),)? "\t",
                $(flexion::cell!($c137),)? "\t",
                $(flexion::cell!($c138),)? "\t",
                $(flexion::cell!($c139),)? "\t",
                $(flexion::cell!($c140),)? "\t",
                $(flexion::cell!($c141),)? "\t",
                $(flexion::cell!($c142),)? "\t",
                $(flexion::cell!($c143),)? "\t",
                $(flexion::cell!($c144),)? "\t",
                $(flexion::cell!($c145),)? "\t",
                $(flexion::cell!($c146),)? "\t",
                $(flexion::cell!($c147),)? "\t",
                $(flexion::cell!($c148),)? "\t",
                $(flexion::cell!($c149),)? "\t",
                $(flexion::cell!($c150),)? "\t",
                $(flexion::cell!($c151),)? "\t",
                $(flexion::cell!($c152),)? "\t",
                $(flexion::cell!($c153),)? "\t",
                $(flexion::cell!($c154),)? "\t",
                $(flexion::cell!($c155),)? "\t",
                $(flexion::cell!($c156),)? "\t",
                $(flexion::cell!($c157),)? "\t",
                $(flexion::cell!($c158),)? "\t",
                $(flexion::cell!($c159),)? "\t",
                $(flexion::cell!($c160),)? "\t",
                $(flexion::cell!($c161),)? "\t",
                $(flexion::cell!($c162),)? "\t",
                $(flexion::cell!($c163),)? "\t",
                $(flexion::cell!($c164),)? "\t",
                $(flexion::cell!($c165),)? "\t",
                $(flexion::cell!($c166),)? "\t",
                $(flexion::cell!($c167),)? "\t",
                $(flexion::cell!($c168),)? "\t",
                $(flexion::cell!($c169),)? "\t",
                $(flexion::cell!($c170),)? "\t",
                $(flexion::cell!($c171),)? "\t",
                $(flexion::cell!($c172),)? "\t",
                $(flexion::cell!($c173),)? "\t",
                $(flexion::cell!($c174),)? "\t",
                $(flexion::cell!($c175),)? "\t",
                $(flexion::cell!($c176),)? "\t",
                $(flexion::cell!($c177),)? "\t",
                $(flexion::cell!($c178),)? "\t",
                $(flexion::cell!($c179),)? "\t",
                $(flexion::cell!($c180),)? "\t",
                $(flexion::cell!($c181),)? "\t",
                $(flexion::cell!($c182),)? "\t",
                $(flexion::cell!($c183),)? "\t",
                $(flexion::cell!($c184),)? "\t",
                $(flexion::cell!($c185),)? "\t",
                $(flexion::cell!($c186),)? "\t",
                $(flexion::cell!($c187),)? "\t",
                $(flexion::cell!($c188),)? "\t",
                $(flexion::cell!($c189),)? "\t",
                $(flexion::cell!($c190),)? "\t",
                $(flexion::cell!($c191),)? "\t",
                $(flexion::cell!($c192),)? "\t",
                $(flexion::cell!($c193),)? "\t",
                $(flexion::cell!($c194),)? "\t",
                $(flexion::cell!($c195),)? "\t",
                $(flexion::cell!($c196),)? "\t",
                $(flexion::cell!($c197),)? "\t",
                $(flexion::cell!($c198),)? "\t",
                $(flexion::cell!($c199),)? "\t",
                $(flexion::cell!($c200),)? "\t",
                $(flexion::cell!($c201),)? "\t",
                $(flexion::cell!($c202),)? "\t",
                $(flexion::cell!($c203),)? "\t",
                $(flexion::cell!($c204),)? "\t",
                $(flexion::cell!($c205),)? "\t",
                $(flexion::cell!($c206),)? "\t",
                $(flexion::cell!($c207),)? "\t",
                $(flexion::cell!($c208),)? "\t",
                $(flexion::cell!($c209),)? "\t",
                $(flexion::cell!($c210),)? "\t",
                $(flexion::cell!($c211),)? "\t",
                $(flexion::cell!($c212),)? "\t",
                $(flexion::cell!($c213),)? "\t",
                $(flexion::cell!($c214),)? "\t",
                $(flexion::cell!($c215),)? "\t",
                $(flexion::cell!($c216),)? "\t",
                $(flexion::cell!($c217),)? "\t",
                $(flexion::cell!($c218),)? "\t",
                $(flexion::cell!($c219),)? "\t",
                $(flexion::cell!($c220),)? "\t",
                $(flexion::cell!($c221),)? "\t",
                $(flexion::cell!($c222),)? "\t",
                $(flexion::cell!($c223),)? "\t",
                $(flexion::cell!($c224),)? "\t",
                $(flexion::cell!($c225),)? "\t",
                $(flexion::cell!($c226),)? "\t",
                $(flexion::cell!($c227),)? "\t",
                $(flexion::cell!($c228),)? "\t",
                $(flexion::cell!($c229),)? "\t",
                $(flexion::cell!($c230),)? "\t",
                $(flexion::cell!($c231),)? "\t",
                $(flexion::cell!($c232),)? "\t",
                $(flexion::cell!($c233),)? "\t",
                $(flexion::cell!($c234),)? "\t",
                $(flexion::cell!($c235),)? "\t",
                $(flexion::cell!($c236),)? "\t",
                $(flexion::cell!($c237),)? "\t",
                $(flexion::cell!($c238),)? "\t",
                $(flexion::cell!($c239),)? "\t",
                $(flexion::cell!($c240),)? "\t",
                $(flexion::cell!($c241),)? "\t",
                $(flexion::cell!($c242),)? "\t",
                $(flexion::cell!($c243),)? "\t",
                $(flexion::cell!($c244),)? "\t",
                $(flexion::cell!($c245),)? "\t",
                $(flexion::cell!($c246),)? "\t",
                $(flexion::cell!($c247),)? "\t",
                $(flexion::cell!($c248),)? "\t",
                $(flexion::cell!($c249),)? "\t",
                $(flexion::cell!($c250),)? "\t",
                $(flexion::cell!($c251),)? "\t",
                $(flexion::cell!($c252),)? "\t",
                $(flexion::cell!($c253),)? "\t",
                $(flexion::cell!($c254),)? "\t",
                $(flexion::cell!($c255),)? "\t",
                $(flexion::cell!($c256),)? "\t",
                $(flexion::cell!($c257),)? "\t",
                $(flexion::cell!($c258),)? "\t",
                $(flexion::cell!($c259),)? "\t",
                $(flexion::cell!($c260),)? "\t",
                $(flexion::cell!($c261),)? "\t",
                $(flexion::cell!($c262),)? "\t",
                $(flexion::cell!($c263),)? "\t",
                $(flexion::cell!($c264),)? "\t",
                $(flexion::cell!($c265),)? "\t",
                $(flexion::cell!($c266),)? "\t",
                $(flexion::cell!($c267),)? "\t",
                $(flexion::cell!($c268),)? "\t",
                $(flexion::cell!($c269),)? "\t",
                $(flexion::cell!($c270),)? "\t",
                $(flexion::cell!($c271),)? "\t",
                $(flexion::cell!($c272),)? "\t",
                $(flexion::cell!($c273),)? "\t",
                $(flexion::cell!($c274),)? "\t",
                $(flexion::cell!($c275),)? "\t",
                $(flexion::cell!($c276),)? "\t",
                $(flexion::cell!($c277),)? "\t",
                $(flexion::cell!($c278),)? "\t",
                $(flexion::cell!($c279),)? "\t",
                $(flexion::cell!($c280),)? "\t",
                $(flexion::cell!($c281),)? "\t",
                $(flexion::cell!($c282),)? "\t",
                $(flexion::cell!($c283),)? "\t",
                $(flexion::cell!($c284),)? "\t",
                $(flexion::cell!($c285),)? "\t",
                $(flexion::cell!($c286),)? "\t",
                $(flexion::cell!($c287),)? "\t",
                $(flexion::cell!($c288),)? "\t",
                $(flexion::cell!($c289),)? "\t",
                $(flexion::cell!($c290),)? "\t",
                $(flexion::cell!($c291),)? "\t",
                $(flexion::cell!($c292),)? "\t",
                $(flexion::cell!($c293),)? "\t",
                $(flexion::cell!($c294),)? "\t",
                $(flexion::cell!($c295),)? "\t",
                $(flexion::cell!($c296),)? "\t",
                $(flexion::cell!($c297),)? "\t",
                $(flexion::cell!($c298),)? "\t",
                $(flexion::cell!($c299),)? "\t",
                $(flexion::cell!($c300),)? "\t",
                $(flexion::cell!($c301),)? "\t",
                $(flexion::cell!($c302),)? "\t",
                $(flexion::cell!($c303),)? "\t",
                $(flexion::cell!($c304),)? "\t",
                $(flexion::cell!($c305),)? "\t",
                $(flexion::cell!($c306),)? "\t",
                $(flexion::cell!($c307),)? "\t",
                $(flexion::cell!($c308),)? "\t",
                $(flexion::cell!($c309),)? "\t",
                $(flexion::cell!($c310),)? "\t",
                $(flexion::cell!($c311),)? "\t",
                $(flexion::cell!($c312),)? "\t",
                $(flexion::cell!($c313),)? "\t",
                $(flexion::cell!($c314),)? "\t",
                $(flexion::cell!($c315),)? "\t",
                $(flexion::cell!($c316),)? "\t",
                $(flexion::cell!($c317),)? "\t",
                $(flexion::cell!($c318),)? "\t",
                $(flexion::cell!($c319),)? "\t",
                $(flexion::cell!($c320),)? "\t",
                $(flexion::cell!($c321),)? "\t",
                $(flexion::cell!($c322),)? "\t",
                $(flexion::cell!($c323),)? "\t",
                $(flexion::cell!($c324),)? "\t",
                $(flexion::cell!($c325),)? "\t",
                $(flexion::cell!($c326),)? "\t",
                $(flexion::cell!($c327),)? "\t",
                $(flexion::cell!($c328),)? "\t",
                $(flexion::cell!($c329),)? "\t",
                $(flexion::cell!($c330),)? "\t",
                $(flexion::cell!($c331),)? "\t",
                $(flexion::cell!($c332),)? "\t",
                $(flexion::cell!($c333),)? "\t",
                $(flexion::cell!($c334),)? "\t",
                $(flexion::cell!($c335),)? "\t",
                $(flexion::cell!($c336),)? "\t",
                $(flexion::cell!($c337),)? "\t",
                $(flexion::cell!($c338),)? "\t",
                $(flexion::cell!($c339),)? "\t",
                $(flexion::cell!($c340),)? "\t",
                $(flexion::cell!($c341),)? "\t",
                $(flexion::cell!($c342),)? "\t",
                $(flexion::cell!($c343),)? "\t",
                $(flexion::cell!($c344),)? "\t",
                $(flexion::cell!($c345),)? "\t",
                $(flexion::cell!($c346),)? "\t",
                $(flexion::cell!($c347),)? "\t",
                $(flexion::cell!($c348),)? "\t",
                $(flexion::cell!($c349),)? "\t",
                $(flexion::cell!($c350),)? "\t",
                $(flexion::cell!($c351),)? "\t",
                $(flexion::cell!($c352),)? "\t",
                $(flexion::cell!($c353),)? "\t",
                $(flexion::cell!($c354),)? "\t",
                $(flexion::cell!($c355),)? "\t",
                $(flexion::cell!($c356),)? "\t",
                $(flexion::cell!($c357),)? "\t",
                $(flexion::cell!($c358),)? "\t",
                $(flexion::cell!($c359),)? "\t",
                $(flexion::cell!($c360),)? "\t",
                $(flexion::cell!($c361),)? "\t",
                $(flexion::cell!($c362),)? "\t",
                $(flexion::cell!($c363),)? "\t",
                $(flexion::cell!($c364),)? "\t",
                $(flexion::cell!($c365),)? "\t",
                $(flexion::cell!($c366),)? "\t",
                $(flexion::cell!($c367),)? "\t",
                $(flexion::cell!($c368),)? "\t",
                $(flexion::cell!($c369),)? "\t",
                $(flexion::cell!($c370),)? "\t",
                $(flexion::cell!($c371),)? "\t",
                $(flexion::cell!($c372),)? "\t",
                $(flexion::cell!($c373),)? "\t",
                $(flexion::cell!($c374),)? "\t",
                $(flexion::cell!($c375),)? "\t",
                $(flexion::cell!($c376),)? "\t",
                $(flexion::cell!($c377),)? "\t",
                $(flexion::cell!($c378),)? "\t",
                $(flexion::cell!($c379),)? "\t",
                $(flexion::cell!($c380),)? "\t",
                $(flexion::cell!($c381),)? "\t",
                $(flexion::cell!($c382),)? "\t",
                $(flexion::cell!($c383),)? "\t",
                $(flexion::cell!($c384),)? "\t",
                $(flexion::cell!($c385),)? "\t",
                $(flexion::cell!($c386),)? "\t",
                $(flexion::cell!($c387),)? "\t",
                $(flexion::cell!($c388),)? "\t",
                $(flexion::cell!($c389),)? "\t",
                $(flexion::cell!($c390),)? "\t",
                $(flexion::cell!($c391),)? "\t",
                $(flexion::cell!($c392),)? "\t",
                $(flexion::cell!($c393),)? "\t",
                $(flexion::cell!($c394),)? "\t",
                $(flexion::cell!($c395),)? "\t",
                $(flexion::cell!($c396),)? "\t",
                $(flexion::cell!($c397),)? "\t",
                $(flexion::cell!($c398),)? "\t",
                $(flexion::cell!($c399),)? "\t",
                $(flexion::cell!($c400),)? "\t",
                $(flexion::cell!($c401),)? "\t",
                $(flexion::cell!($c402),)? "\t",
                $(flexion::cell!($c403),)? "\t",
                $(flexion::cell!($c404),)? "\t",
                $(flexion::cell!($c405),)? "\t",
                $(flexion::cell!($c406),)? "\t",
                $(flexion::cell!($c407),)? "\t",
                $(flexion::cell!($c408),)? "\t",
                $(flexion::cell!($c409),)? "\t",
                $(flexion::cell!($c410),)? "\t",
                $(flexion::cell!($c411),)? "\t",
                $(flexion::cell!($c412),)? "\t",
                $(flexion::cell!($c413),)? "\t",
                $(flexion::cell!($c414),)? "\t",
                $(flexion::cell!($c415),)? "\t",
                $(flexion::cell!($c416),)? "\t",
                $(flexion::cell!($c417),)? "\t",
                $(flexion::cell!($c418),)? "\t",
                $(flexion::cell!($c419),)? "\t",
                $(flexion::cell!($c420),)? "\t",
                $(flexion::cell!($c421),)? "\t",
                $(flexion::cell!($c422),)? "\t",
                $(flexion::cell!($c423),)? "\t",
                $(flexion::cell!($c424),)? "\t",
                $(flexion::cell!($c425),)? "\t",
                $(flexion::cell!($c426),)? "\t",
                $(flexion::cell!($c427),)? "\t",
                $(flexion::cell!($c428),)? "\t",
                $(flexion::cell!($c429),)? "\t",
                $(flexion::cell!($c430),)? "\t",
                $(flexion::cell!($c431),)? "\t",
                $(flexion::cell!($c432),)? "\t",
                $(flexion::cell!($c433),)? "\t",
                $(flexion::cell!($c434),)? "\t",
                $(flexion::cell!($c435),)? "\t",
                $(flexion::cell!($c436),)? "\t",
                $(flexion::cell!($c437),)? "\t",
                $(flexion::cell!($c438),)? "\t",
                $(flexion::cell!($c439),)? "\t",
                $(flexion::cell!($c440),)? "\t",
                $(flexion::cell!($c441),)? "\t",
                $(flexion::cell!($c442),)? "\t",
                $(flexion::cell!($c443),)? "\t",
                $(flexion::cell!($c444),)? "\t",
                $(flexion::cell!($c445),)? "\t",
                $(flexion::cell!($c446),)? "\t",
                $(flexion::cell!($c447),)? "\t",
                $(flexion::cell!($c448),)? "\t",
                $(flexion::cell!($c449),)? "\t",
                $(flexion::cell!($c450),)? "\t",
                $(flexion::cell!($c451),)? "\t",
                $(flexion::cell!($c452),)? "\t",
                $(flexion::cell!($c453),)? "\t",
                $(flexion::cell!($c454),)? "\t",
                $(flexion::cell!($c455),)? "\t",
                $(flexion::cell!($c456),)? "\t",
                $(flexion::cell!($c457),)? "\t",
                $(flexion::cell!($c458),)? "\t",
                $(flexion::cell!($c459),)? "\t",
                $(flexion::cell!($c460),)? "\t",
                $(flexion::cell!($c461),)? "\t",
                $(flexion::cell!($c462),)? "\t",
                $(flexion::cell!($c463),)? "\t",
                $(flexion::cell!($c464),)? "\t",
                $(flexion::cell!($c465),)? "\t",
                $(flexion::cell!($c466),)? "\t",
                $(flexion::cell!($c467),)? "\t",
                $(flexion::cell!($c468),)? "\t",
                $(flexion::cell!($c469),)? "\t",
                $(flexion::cell!($c470),)? "\t",
                $(flexion::cell!($c471),)? "\t",
                $(flexion::cell!($c472),)? "\t",
                $(flexion::cell!($c473),)? "\t",
                $(flexion::cell!($c474),)? "\t",
                $(flexion::cell!($c475),)? "\t",
                $(flexion::cell!($c476),)? "\t",
                $(flexion::cell!($c477),)? "\t",
                $(flexion::cell!($c478),)? "\t",
                $(flexion::cell!($c479),)? "\t",
                $(flexion::cell!($c480),)? "\t",
                $(flexion::cell!($c481),)? "\t",
                $(flexion::cell!($c482),)? "\t",
                $(flexion::cell!($c483),)? "\t",
                $(flexion::cell!($c484),)? "\t",
                $(flexion::cell!($c485),)? "\t",
                $(flexion::cell!($c486),)? "\t",
                $(flexion::cell!($c487),)? "\t",
                $(flexion::cell!($c488),)? "\t",
                $(flexion::cell!($c489),)? "\t",
                $(flexion::cell!($c490),)? "\t",
                $(flexion::cell!($c491),)? "\t",
                $(flexion::cell!($c492),)? "\t",
                $(flexion::cell!($c493),)? "\t",
                $(flexion::cell!($c494),)? "\t",
                $(flexion::cell!($c495),)? "\t",
                $(flexion::cell!($c496),)? "\t",
                $(flexion::cell!($c497),)? "\t",
                $(flexion::cell!($c498),)? "\t",
                $(flexion::cell!($c499),)? "\t",
                $(flexion::cell!($c500),)? "\t",
                $(flexion::cell!($c501),)? "\t",
                $(flexion::cell!($c502),)? "\t",
                $(flexion::cell!($c503),)? "\t",
                $(flexion::cell!($c504),)? "\t",
                $(flexion::cell!($c505),)? "\t",
                $(flexion::cell!($c506),)? "\t",
                $(flexion::cell!($c507),)? "\t",
                $(flexion::cell!($c508),)? "\t",
                $(flexion::cell!($c509),)? "\t",
                $(flexion::cell!($c510),)? "\t",
                $(flexion::cell!($c511),)? "\t",
                $(flexion::cell!($c512),)? "\t",
                $(flexion::cell!($c513),)? "\t",
                $(flexion::cell!($c514),)? "\t",
                $(flexion::cell!($c515),)? "\t",
                $(flexion::cell!($c516),)? "\t",
                $(flexion::cell!($c517),)? "\t",
                $(flexion::cell!($c518),)? "\t",
                $(flexion::cell!($c519),)? "\t",
                $(flexion::cell!($c520),)? "\t",
                $(flexion::cell!($c521),)? "\t",
                $(flexion::cell!($c522),)? "\t",
                $(flexion::cell!($c523),)? "\t",
                $(flexion::cell!($c524),)? "\t",
                $(flexion::cell!($c525),)? "\t",
                $(flexion::cell!($c526),)? "\t",
                $(flexion::cell!($c527),)? "\t",
                $(flexion::cell!($c528),)? "\t",
                $(flexion::cell!($c529),)? "\t",
                $(flexion::cell!($c530),)? "\t",
                $(flexion::cell!($c531),)? "\t",
                $(flexion::cell!($c532),)? "\t",
                $(flexion::cell!($c533),)? "\t",
                $(flexion::cell!($c534),)? "\t",
                $(flexion::cell!($c535),)? "\t",
                $(flexion::cell!($c536),)? "\t",
                $(flexion::cell!($c537),)? "\t",
                $(flexion::cell!($c538),)? "\t",
                $(flexion::cell!($c539),)? "\t",
                $(flexion::cell!($c540),)? "\t",
                $(flexion::cell!($c541),)? "\t",
                $(flexion::cell!($c542),)? "\t",
                $(flexion::cell!($c543),)? "\t",
                $(flexion::cell!($c544),)? "\t",
                $(flexion::cell!($c545),)? "\t",
                $(flexion::cell!($c546),)? "\t",
                $(flexion::cell!($c547),)? "\t",
                $(flexion::cell!($c548),)? "\t",
                $(flexion::cell!($c549),)? "\t",
                $(flexion::cell!($c550),)? "\t",
                $(flexion::cell!($c551),)? "\t",
                $(flexion::cell!($c552),)? "\t",
                $(flexion::cell!($c553),)? "\t",
                $(flexion::cell!($c554),)? "\t",
                $(flexion::cell!($c555),)? "\t",
                $(flexion::cell!($c556),)? "\t",
                $(flexion::cell!($c557),)? "\t",
                $(flexion::cell!($c558),)? "\t",
                $(flexion::cell!($c559),)? "\t",
                $(flexion::cell!($c560),)? "\t",
                $(flexion::cell!($c561),)? "\t",
                $(flexion::cell!($c562),)? "\t",
                $(flexion::cell!($c563),)? "\t",
                $(flexion::cell!($c564),)? "\t",
                $(flexion::cell!($c565),)? "\t",
                $(flexion::cell!($c566),)? "\t",
                $(flexion::cell!($c567),)? "\t",
                $(flexion::cell!($c568),)? "\t",
                $(flexion::cell!($c569),)? "\t",
                $(flexion::cell!($c570),)? "\t",
                $(flexion::cell!($c571),)? "\t",
                $(flexion::cell!($c572),)? "\t",
                $(flexion::cell!($c573),)? "\t",
                $(flexion::cell!($c574),)? "\t",
                $(flexion::cell!($c575),)? "\t",
                $(flexion::cell!($c576),)? "\t",
                $(flexion::cell!($c577),)? "\t",
                $(flexion::cell!($c578),)? "\t",
                $(flexion::cell!($c579),)? "\t",
                $(flexion::cell!($c580),)? "\t",
                $(flexion::cell!($c581),)? "\t",
                $(flexion::cell!($c582),)? "\t",
                $(flexion::cell!($c583),)? "\t",
                $(flexion::cell!($c584),)? "\t",
                $(flexion::cell!($c585),)? "\t",
                $(flexion::cell!($c586),)? "\t",
                $(flexion::cell!($c587),)? "\t",
                $(flexion::cell!($c588),)? "\t",
                $(flexion::cell!($c589),)? "\t",
                $(flexion::cell!($c590),)? "\t",
                $(flexion::cell!($c591),)? "\t",
                $(flexion::cell!($c592),)? "\t",
                $(flexion::cell!($c593),)? "\t",
                $(flexion::cell!($c594),)? "\t",
                $(flexion::cell!($c595),)? "\t",
                $(flexion::cell!($c596),)? "\t",
                $(flexion::cell!($c597),)? "\t",
                $(flexion::cell!($c598),)? "\t",
                $(flexion::cell!($c599),)? "\t",
                $(flexion::cell!($c600),)? "\t",
                $(flexion::cell!($c601),)? "\t",
                $(flexion::cell!($c602),)? "\t",
                $(flexion::cell!($c603),)? "\t",
                $(flexion::cell!($c604),)? "\t",
                $(flexion::cell!($c605),)? "\t",
                $(flexion::cell!($c606),)? "\t",
                $(flexion::cell!($c607),)? "\t",
                $(flexion::cell!($c608),)? "\t",
                $(flexion::cell!($c609),)? "\t",
                $(flexion::cell!($c610),)? "\t",
                $(flexion::cell!($c611),)? "\t",
                $(flexion::cell!($c612),)? "\t",
                $(flexion::cell!($c613),)? "\t",
                $(flexion::cell!($c614),)? "\t",
                $(flexion::cell!($c615),)? "\t",
                $(flexion::cell!($c616),)? "\t",
                $(flexion::cell!($c617),)? "\t",
                $(flexion::cell!($c618),)? "\t",
                $(flexion::cell!($c619),)? "\t",
                $(flexion::cell!($c620),)? "\t",
                $(flexion::cell!($c621),)? "\t",
                $(flexion::cell!($c622),)? "\t",
                $(flexion::cell!($c623),)? "\t",
                $(flexion::cell!($c624),)? "\t",
                $(flexion::cell!($c625),)? "\t",
                $(flexion::cell!($c626),)? "\t",
                $(flexion::cell!($c627),)? "\t",
                $(flexion::cell!($c628),)? "\t",
                $(flexion::cell!($c629),)? "\t",
                $(flexion::cell!($c630),)? "\t",
                $(flexion::cell!($c631),)? "\t",
                $(flexion::cell!($c632),)? "\t",
                $(flexion::cell!($c633),)? "\t",
                $(flexion::cell!($c634),)? "\t",
                $(flexion::cell!($c635),)? "\t",
                $(flexion::cell!($c636),)? "\t",
                $(flexion::cell!($c637),)? "\t",
                $(flexion::cell!($c638),)? "\t",
                $(flexion::cell!($c639),)? "\t",
                $(flexion::cell!($c640),)? "\t",
                $(flexion::cell!($c641),)? "\t",
                $(flexion::cell!($c642),)? "\t",
                $(flexion::cell!($c643),)? "\t",
                $(flexion::cell!($c644),)? "\t",
                $(flexion::cell!($c645),)? "\t",
                $(flexion::cell!($c646),)? "\t",
                $(flexion::cell!($c647),)? "\t",
                $(flexion::cell!($c648),)? "\t",
                $(flexion::cell!($c649),)? "\t",
                $(flexion::cell!($c650),)? "\t",
                $(flexion::cell!($c651),)? "\t",
                $(flexion::cell!($c652),)? "\t",
                $(flexion::cell!($c653),)? "\t",
                $(flexion::cell!($c654),)? "\t",
                $(flexion::cell!($c655),)? "\t",
                $(flexion::cell!($c656),)? "\t",
                $(flexion::cell!($c657),)? "\t",
                $(flexion::cell!($c658),)? "\t",
                $(flexion::cell!($c659),)? "\t",
                $(flexion::cell!($c660),)? "\t",
                $(flexion::cell!($c661),)? "\t",
                $(flexion::cell!($c662),)? "\t",
                $(flexion::cell!($c663),)? "\t",
                $(flexion::cell!($c664),)? "\t",
                $(flexion::cell!($c665),)? "\t",
                $(flexion::cell!($c666),)? "\t",
                $(flexion::cell!($c667),)? "\t",
                $(flexion::cell!($c668),)? "\t",
                $(flexion::cell!($c669),)? "\t",
                $(flexion::cell!($c670),)? "\t",
                $(flexion::cell!($c671),)? "\t",
                $(flexion::cell!($c672),)? "\t",
                $(flexion::cell!($c673),)? "\t",
                $(flexion::cell!($c674),)? "\t",
                $(flexion::cell!($c675),)? "\t",
                $(flexion::cell!($c676),)? "\t",
                $(flexion::cell!($c677),)? "\t",
                $(flexion::cell!($c678),)? "\t",
                $(flexion::cell!($c679),)? "\t",
                $(flexion::cell!($c680),)? "\t",
                $(flexion::cell!($c681),)? "\t",
                $(flexion::cell!($c682),)? "\t",
                $(flexion::cell!($c683),)? "\t",
                $(flexion::cell!($c684),)? "\t",
                $(flexion::cell!($c685),)? "\t",
                $(flexion::cell!($c686),)? "\t",
                $(flexion::cell!($c687),)? "\t",
                $(flexion::cell!($c688),)? "\t",
                $(flexion::cell!($c689),)? "\t",
                $(flexion::cell!($c690),)? "\t",
                $(flexion::cell!($c691),)? "\t",
                $(flexion::cell!($c692),)? "\t",
                $(flexion::cell!($c693),)? "\t",
                $(flexion::cell!($c694),)? "\t",
                $(flexion::cell!($c695),)? "\t",
                $(flexion::cell!($c696),)? "\t",
                $(flexion::cell!($c697),)? "\t",
                $(flexion::cell!($c698),)? "\t",
                $(flexion::cell!($c699),)? "\t",
                $(flexion::cell!($c700),)? "\t",
                $(flexion::cell!($c701),)? "\t",
                $(flexion::cell!($c702),)? "\t",
                $(flexion::cell!($c703),)? "\t",
                $(flexion::cell!($c704),)? "\t",
                $(flexion::cell!($c705),)? "\t",
                $(flexion::cell!($c706),)? "\t",
                $(flexion::cell!($c707),)? "\t",
                $(flexion::cell!($c708),)? "\t",
                $(flexion::cell!($c709),)? "\t",
                $(flexion::cell!($c710),)? "\t",
                $(flexion::cell!($c711),)? "\t",
                $(flexion::cell!($c712),)? "\t",
                $(flexion::cell!($c713),)? "\t",
                $(flexion::cell!($c714),)? "\t",
                $(flexion::cell!($c715),)? "\t",
                $(flexion::cell!($c716),)? "\t",
                $(flexion::cell!($c717),)? "\t",
                $(flexion::cell!($c718),)? "\t",
                $(flexion::cell!($c719),)? "\t",
                $(flexion::cell!($c720),)? "\t",
                $(flexion::cell!($c721),)? "\t",
                $(flexion::cell!($c722),)? "\t",
                $(flexion::cell!($c723),)? "\t",
                $(flexion::cell!($c724),)? "\t",
                $(flexion::cell!($c725),)? "\t",
                $(flexion::cell!($c726),)? "\t",
                $(flexion::cell!($c727),)? "\t",
                $(flexion::cell!($c728),)? "\t",
                $(flexion::cell!($c729),)? "\t",
                $(flexion::cell!($c730),)? "\t",
                $(flexion::cell!($c731),)? "\t",
                $(flexion::cell!($c732),)? "\t",
                $(flexion::cell!($c733),)? "\t",
                $(flexion::cell!($c734),)? "\t",
                $(flexion::cell!($c735),)? "\t",
                $(flexion::cell!($c736),)? "\t",
                $(flexion::cell!($c737),)? "\t",
                $(flexion::cell!($c738),)? "\t",
                $(flexion::cell!($c739),)? "\t",
                $(flexion::cell!($c740),)? "\t",
                $(flexion::cell!($c741),)? "\t",
                $(flexion::cell!($c742),)? "\t",
                $(flexion::cell!($c743),)? "\t",
                $(flexion::cell!($c744),)? "\t",
                $(flexion::cell!($c745),)? "\t",
                $(flexion::cell!($c746),)? "\t",
                $(flexion::cell!($c747),)? "\t",
                $(flexion::cell!($c748),)? "\t",
                $(flexion::cell!($c749),)? "\t",
                $(flexion::cell!($c750),)? "\t",
                $(flexion::cell!($c751),)? "\t",
                $(flexion::cell!($c752),)? "\t",
                $(flexion::cell!($c753),)? "\t",
                $(flexion::cell!($c754),)? "\t",
                $(flexion::cell!($c755),)? "\t",
                $(flexion::cell!($c756),)? "\t",
                $(flexion::cell!($c757),)? "\t",
                $(flexion::cell!($c758),)? "\t",
                $(flexion::cell!($c759),)? "\t",
                $(flexion::cell!($c760),)? "\t",
                $(flexion::cell!($c761),)? "\t",
                $(flexion::cell!($c762),)? "\t",
                $(flexion::cell!($c763),)? "\t",
                $(flexion::cell!($c764),)? "\t",
                $(flexion::cell!($c765),)? "\t",
                $(flexion::cell!($c766),)? "\t",
                $(flexion::cell!($c767),)? "\t",
                $(flexion::cell!($c768),)? "\t",
                $(flexion::cell!($c769),)? "\t",
                $(flexion::cell!($c770),)? "\t",
                $(flexion::cell!($c771),)? "\t",
                $(flexion::cell!($c772),)? "\t",
                $(flexion::cell!($c773),)? "\t",
                $(flexion::cell!($c774),)? "\t",
                $(flexion::cell!($c775),)? "\t",
                $(flexion::cell!($c776),)? "\t",
                $(flexion::cell!($c777),)? "\t",
                $(flexion::cell!($c778),)? "\t",
                $(flexion::cell!($c779),)? "\t",
                $(flexion::cell!($c780),)? "\t",
                $(flexion::cell!($c781),)? "\t",
                $(flexion::cell!($c782),)? "\t",
                $(flexion::cell!($c783),)? "\t",
                $(flexion::cell!($c784),)? "\t",
                $(flexion::cell!($c785),)? "\t",
                $(flexion::cell!($c786),)? "\t",
                $(flexion::cell!($c787),)? "\t",
                $(flexion::cell!($c788),)? "\t",
                $(flexion::cell!($c789),)? "\t",
                $(flexion::cell!($c790),)? "\t",
                $(flexion::cell!($c791),)? "\t",
                $(flexion::cell!($c792),)? "\t",
                $(flexion::cell!($c793),)? "\t",
                $(flexion::cell!($c794),)? "\t",
                $(flexion::cell!($c795),)? "\t",
                $(flexion::cell!($c796),)? "\t",
                $(flexion::cell!($c797),)? "\t",
                $(flexion::cell!($c798),)? "\t",
                $(flexion::cell!($c799),)? "\t",
                $(flexion::cell!($c800),)? "\t",
                $(flexion::cell!($c801),)? "\t",
                $(flexion::cell!($c802),)? "\t",
                $(flexion::cell!($c803),)? "\t",
                $(flexion::cell!($c804),)? "\t",
                $(flexion::cell!($c805),)? "\t",
                $(flexion::cell!($c806),)? "\t",
                $(flexion::cell!($c807),)? "\t",
                $(flexion::cell!($c808),)? "\t",
                $(flexion::cell!($c809),)? "\t",
                $(flexion::cell!($c810),)? "\t",
                $(flexion::cell!($c811),)? "\t",
                $(flexion::cell!($c812),)? "\t",
                $(flexion::cell!($c813),)? "\t",
                $(flexion::cell!($c814),)? "\t",
                $(flexion::cell!($c815),)? "\t",
                $(flexion::cell!($c816),)? "\t",
                $(flexion::cell!($c817),)? "\t",
                $(flexion::cell!($c818),)? "\t",
                $(flexion::cell!($c819),)? "\t",
                $(flexion::cell!($c820),)? "\t",
                $(flexion::cell!($c821),)? "\t",
                $(flexion::cell!($c822),)? "\t",
                $(flexion::cell!($c823),)? "\t",
                $(flexion::cell!($c824),)? "\t",
                $(flexion::cell!($c825),)? "\t",
                $(flexion::cell!($c826),)? "\t",
                $(flexion::cell!($c827),)? "\t",
                $(flexion::cell!($c828),)? "\t",
                $(flexion::cell!($c829),)? "\t",
                $(flexion::cell!($c830),)? "\t",
                $(flexion::cell!($c831),)? "\t",
                $(flexion::cell!($c832),)? "\t",
                $(flexion::cell!($c833),)? "\t",
                $(flexion::cell!($c834),)? "\t",
                $(flexion::cell!($c835),)? "\t",
                $(flexion::cell!($c836),)? "\t",
                $(flexion::cell!($c837),)? "\t",
                $(flexion::cell!($c838),)? "\t",
                $(flexion::cell!($c839),)? "\t",
                $(flexion::cell!($c840),)? "\t",
                $(flexion::cell!($c841),)? "\t",
                $(flexion::cell!($c842),)? "\t",
                $(flexion::cell!($c843),)? "\t",
                $(flexion::cell!($c844),)? "\t",
                $(flexion::cell!($c845),)? "\t",
                $(flexion::cell!($c846),)? "\t",
                $(flexion::cell!($c847),)? "\t",
                $(flexion::cell!($c848),)? "\t",
                $(flexion::cell!($c849),)? "\t",
                $(flexion::cell!($c850),)? "\t",
                $(flexion::cell!($c851),)? "\t",
                $(flexion::cell!($c852),)? "\t",
                $(flexion::cell!($c853),)? "\t",
                $(flexion::cell!($c854),)? "\t",
                $(flexion::cell!($c855),)? "\t",
                $(flexion::cell!($c856),)? "\t",
                $(flexion::cell!($c857),)? "\t",
                $(flexion::cell!($c858),)? "\t",
                $(flexion::cell!($c859),)? "\t",
                $(flexion::cell!($c860),)? "\t",
                $(flexion::cell!($c861),)? "\t",
                $(flexion::cell!($c862),)? "\t",
                $(flexion::cell!($c863),)? "\t",
                $(flexion::cell!($c864),)? "\t",
                $(flexion::cell!($c865),)? "\t",
                $(flexion::cell!($c866),)? "\t",
                $(flexion::cell!($c867),)? "\t",
                $(flexion::cell!($c868),)? "\t",
                $(flexion::cell!($c869),)? "\t",
                $(flexion::cell!($c870),)? "\t",
                $(flexion::cell!($c871),)? "\t",
                $(flexion::cell!($c872),)? "\t",
                $(flexion::cell!($c873),)? "\t",
                $(flexion::cell!($c874),)? "\t",
                $(flexion::cell!($c875),)? "\t",
                $(flexion::cell!($c876),)? "\t",
                $(flexion::cell!($c877),)? "\t",
                $(flexion::cell!($c878),)? "\t",
                $(flexion::cell!($c879),)? "\t",
                $(flexion::cell!($c880),)? "\t",
                $(flexion::cell!($c881),)? "\t",
                $(flexion::cell!($c882),)? "\t",
                $(flexion::cell!($c883),)? "\t",
                $(flexion::cell!($c884),)? "\t",
                $(flexion::cell!($c885),)? "\t",
                $(flexion::cell!($c886),)? "\t",
                $(flexion::cell!($c887),)? "\t",
                $(flexion::cell!($c888),)? "\t",
                $(flexion::cell!($c889),)? "\t",
                $(flexion::cell!($c890),)? "\t",
                $(flexion::cell!($c891),)? "\t",
                $(flexion::cell!($c892),)? "\t",
                $(flexion::cell!($c893),)? "\t",
                $(flexion::cell!($c894),)? "\t",
                $(flexion::cell!($c895),)? "\t",
                $(flexion::cell!($c896),)? "\t",
                $(flexion::cell!($c897),)? "\t",
                $(flexion::cell!($c898),)? "\t",
                $(flexion::cell!($c899),)? "\t",
                $(flexion::cell!($c900),)? "\t",
                $(flexion::cell!($c901),)? "\t",
                $(flexion::cell!($c902),)? "\t",
                $(flexion::cell!($c903),)? "\t",
                $(flexion::cell!($c904),)? "\t",
                $(flexion::cell!($c905),)? "\t",
                $(flexion::cell!($c906),)? "\t",
                $(flexion::cell!($c907),)? "\t",
                $(flexion::cell!($c908),)? "\t",
                $(flexion::cell!($c909),)? "\t",
                $(flexion::cell!($c910),)? "\t",
                $(flexion::cell!($c911),)? "\t",
                $(flexion::cell!($c912),)? "\t",
                $(flexion::cell!($c913),)? "\t",
                $(flexion::cell!($c914),)? "\t",
                $(flexion::cell!($c915),)? "\t",
                $(flexion::cell!($c916),)? "\t",
                $(flexion::cell!($c917),)? "\t",
                $(flexion::cell!($c918),)? "\t",
                $(flexion::cell!($c919),)? "\t",
                $(flexion::cell!($c920),)? "\t",
                $(flexion::cell!($c921),)? "\t",
                $(flexion::cell!($c922),)? "\t",
                $(flexion::cell!($c923),)? "\t",
                $(flexion::cell!($c924),)? "\t",
                $(flexion::cell!($c925),)? "\t",
                $(flexion::cell!($c926),)? "\t",
                $(flexion::cell!($c927),)? "\t",
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
            )),
            affix: None,
        }
    };
}

/// The conjugation a conjugation is like, by its name.
macro_rules! like {
    () => {
        None
    };
    ($parent:ident) => {
        Some(Conjugation::$parent as u16)
    };
}

/// The cells a conjugation reads in the one it is like, by the view's name (see
/// the views below); none: the same cells.
macro_rules! view {
    () => {
        None
    };
    ($view:ident) => {
        Some(::std::borrow::Cow::Borrowed(&$view))
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `ayırmak`: 736 words.
    Ayırmak,
    /// Like `vermek`: 483 words.
    Vermek,
    /// Like `patlamak`: 266 words.
    Patlamak,
    /// Like `takip etmek`: 186 words.
    Takipetmek,
    /// Like `beslemek`: 149 words.
    Beslemek,
    /// Like `açmak`: 124 words.
    Açmak,
    /// Like `silmek`: 107 words.
    Silmek,
    /// Like `gitmek`: 53 words.
    Gitmek,
    /// Like `ele vermek`: 33 words.
    Elevermek,
    /// Like `var olmak`: 31 words.
    Varolmak,
    /// Like `taşımak`: 25 words.
    Taşımak,
    /// Like `film çekmek`: 17 words.
    Filmçekmek,
    /// Like `koynuna almak`: 16 words.
    Koynunaalmak,
    /// Like `yürümek`: 16 words.
    Yürümek,
    /// Like `aşk yapmak`: 15 words.
    Aşkyapmak,
    /// Like `tekme atmak`: 12 words.
    Tekmeatmak,
    /// Like `imana gelmek`: 12 words.
    Imanagelmek,
    /// Like `cama çıkmak`: 12 words.
    Camaçıkmak,
    /// Like `ilişkiye girmek`: 10 words.
    Ilişkiyegirmek,
    /// Like `baştan çıkarmak`: 7 words.
    Baştançıkarmak,
    /// Like `almak`: 7 words.
    Almak,
    /// Like `yemek`: 6 words.
    Yemek,
    /// Like `vurmak`: 5 words.
    Vurmak,
    /// Like `yol açmak`: 5 words.
    Yolaçmak,
    /// Like `maytap geçmek`: 4 words.
    Maytapgeçmek,
    /// Like `kafa tutmak`: 4 words.
    Kafatutmak,
    /// Like `bükmek`: 3 words.
    Bükmek,
    /// Like `fındık kırmak`: 3 words.
    Fındıkkırmak,
    /// Like `geç kalmak`: 3 words.
    Geçkalmak,
    /// Like `serbest bırakmak`: 3 words.
    Serbestbırakmak,
    /// Like `görmek`: 3 words.
    Görmek,
    /// Like `götü gitmek`: 3 words.
    Götügitmek,
    /// Like `büyük oynamak`: 3 words.
    Büyükoynamak,
    /// Like `layığını bulmak`: 3 words.
    Layığınıbulmak,
    /// Like `ses kasmak`: 3 words.
    Seskasmak,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 35] = [
        Conjugation::Ayırmak,
        Conjugation::Vermek,
        Conjugation::Patlamak,
        Conjugation::Takipetmek,
        Conjugation::Beslemek,
        Conjugation::Açmak,
        Conjugation::Silmek,
        Conjugation::Gitmek,
        Conjugation::Elevermek,
        Conjugation::Varolmak,
        Conjugation::Taşımak,
        Conjugation::Filmçekmek,
        Conjugation::Koynunaalmak,
        Conjugation::Yürümek,
        Conjugation::Aşkyapmak,
        Conjugation::Tekmeatmak,
        Conjugation::Imanagelmek,
        Conjugation::Camaçıkmak,
        Conjugation::Ilişkiyegirmek,
        Conjugation::Baştançıkarmak,
        Conjugation::Almak,
        Conjugation::Yemek,
        Conjugation::Vurmak,
        Conjugation::Yolaçmak,
        Conjugation::Maytapgeçmek,
        Conjugation::Kafatutmak,
        Conjugation::Bükmek,
        Conjugation::Fındıkkırmak,
        Conjugation::Geçkalmak,
        Conjugation::Serbestbırakmak,
        Conjugation::Görmek,
        Conjugation::Götügitmek,
        Conjugation::Büyükoynamak,
        Conjugation::Layığınıbulmak,
        Conjugation::Seskasmak,
    ];

    /// Its table.
    pub fn paradigm(self) -> Option<&'static Paradigm<'static>> {
        CONJUGATIONS.get(self as usize)
    }
}

impl flexion::Label for Conjugation {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

pub(super) static CONJUGATIONS: [Paradigm<'static>; 35] = [
    // Ayırmak: 736 words (kazanmak, cezalandırmak, yapışmak, azaltmak, bırakmak, boşalmak).
    conjugation!("ayırmak", 736, "~mak";
    stem converb_neg_plain_doubled: "~maya ~maya- like ayırmak";
    stem converb_neg_abil_doubled: "~amaya ~amaya- like ayırmak";
    pos_plain_aor__: "~IrIm", "~IrsIn", "~Ir", _, "~IrsInIz", "~IrlEr";
    pos_plain_aor_evid: "~IrmIşIm", "~IrmIşsIn", "~IrmIş", "~IrmIşIz", "~IrmIşsInIz", "~IrlErmIş";
    pos_plain_aor_past: "~IrdIm", "~IrdIn", "~IrdI", "~IrdIk", "~IrdInIz", "~IrlErdI";
    pos_plain_aor_cond: "~IrsEm", "~IrsEn", "~IrsE", "~IrsEk", "~IrsEnIz", "~IrlErsE";
    pos_plain_prog__: "~Iyorum", _, _, _, _, _;
    pos_plain_evid__: _, "~mIşsIn", "~mIş", _, "~mIşsInIz", "~mIşlEr";
    pos_plain_evid_evid: "~mIşmIşIm", "~mIşmIşsIn", "~mIşmIş", "~mIşmIşIz", "~mIşmIşsInIz", "~mIşlErmIş";
    pos_plain_evid_past: "~mIştIm", "~mIştIn", "~mIştI", "~mIştIk", "~mIştInIz", "~mIşlErdI";
    pos_plain_evid_cond: "~mIşsEm", "~mIşsEn", "~mIşsE", "~mIşsEk", "~mIşsEnIz", "~mIşlErsE";
    pos_plain_fut__: "~EcEğIm", "~EcEksIn", "~EcEk", _, "~EcEksInIz", "~EcEklEr";
    pos_plain_fut_evid: "~EcEkmIşIm", "~EcEkmIşsIn", "~EcEkmIş", "~EcEkmIşIz", "~EcEkmIşsInIz", "~EcEklErmIş";
    pos_plain_fut_past: "~EcEktIm", "~EcEktIn", "~EcEktI", "~EcEktIk", "~EcEktInIz", "~EcEklErdI";
    pos_plain_fut_cond: "~EcEksEm", "~EcEksEn", "~EcEksE", "~EcEksEk", "~EcEksEnIz", "~EcEklErsE";
    pos_abil_aor__: "~Ebilirim", "~Ebilirsin", "~Ebilir", "~Ebiliriz", "~Ebilirsiniz", "~Ebilirler";
    pos_abil_aor_evid: "~Ebilirmişim", "~Ebilirmişsin", "~Ebilirmiş", "~Ebilirmişiz", "~Ebilirmişsiniz", "~Ebilirlermiş";
    pos_abil_aor_past: "~Ebilirdim", "~Ebilirdin", "~Ebilirdi", "~Ebilirdik", "~Ebilirdiniz", "~Ebilirlerdi";
    pos_abil_aor_cond: "~Ebilirsem", "~Ebilirsen", "~Ebilirse", "~Ebilirsek", "~Ebilirseniz", "~Ebilirlerse";
    pos_abil_prog__: "~Ebiliyorum", "~Ebiliyorsun", "~Ebiliyor", "~Ebiliyoruz", "~Ebiliyorsunuz", "~Ebiliyorlar";
    pos_abil_prog_evid: "~Ebiliyormuşum", "~Ebiliyormuşsun", "~Ebiliyormuş", "~Ebiliyormuşuz", "~Ebiliyormuşsunuz", "~Ebiliyorlarmış";
    pos_abil_prog_past: "~Ebiliyordum", "~Ebiliyordun", "~Ebiliyordu", "~Ebiliyorduk", "~Ebiliyordunuz", "~Ebiliyorlardı";
    pos_abil_prog_cond: "~Ebiliyorsam", "~Ebiliyorsan", "~Ebiliyorsa", "~Ebiliyorsak", "~Ebiliyorsanız", "~Ebiliyorlarsa";
    pos_abil_evid__: "~Ebilmişim", "~Ebilmişsin", "~Ebilmiş", "~Ebilmişiz", "~Ebilmişsiniz", "~Ebilmişler";
    pos_abil_evid_evid: "~Ebilmişmişim", "~Ebilmişmişsin", "~Ebilmişmiş", "~Ebilmişmişiz", "~Ebilmişmişsiniz", "~Ebilmişlermiş";
    pos_abil_evid_past: "~Ebilmiştim", "~Ebilmiştin", "~Ebilmişti", "~Ebilmiştik", "~Ebilmiştiniz", "~Ebilmişlerdi";
    pos_abil_evid_cond: "~Ebilmişsem", "~Ebilmişsen", "~Ebilmişse", "~Ebilmişsek", "~Ebilmişseniz", "~Ebilmişlerse";
    pos_abil_fut__: "~Ebileceğim", "~Ebileceksin", "~Ebilecek", "~Ebileceğiz", "~Ebileceksiniz", "~Ebilecekler";
    pos_abil_fut_evid: "~Ebilecekmişim", "~Ebilecekmişsin", "~Ebilecekmiş", "~Ebilecekmişiz", "~Ebilecekmişsiniz", "~Ebileceklermiş";
    pos_abil_fut_past: "~Ebilecektim", "~Ebilecektin", "~Ebilecekti", "~Ebilecektik", "~Ebilecektiniz", "~Ebileceklerdi";
    pos_abil_fut_cond: "~Ebileceksem", "~Ebileceksen", "~Ebilecekse", "~Ebileceksek", "~Ebilecekseniz", "~Ebileceklerse";
    pos_abil_cont__: "~Ebilmekteyim", "~Ebilmektesin", "~Ebilmekte", "~Ebilmekteyiz", "~Ebilmektesiniz", "~Ebilmekteler";
    pos_abil_cont_evid: "~Ebilmekteymişim", "~Ebilmekteymişsin", "~Ebilmekteymiş", "~Ebilmekteymişiz", "~Ebilmekteymişsiniz", "~Ebilmektelermiş";
    pos_abil_cont_past: "~Ebilmekteydim", "~Ebilmekteydin", "~Ebilmekteydi", "~Ebilmekteydik", "~Ebilmekteydiniz", "~Ebilmektelerdi";
    pos_abil_cont_cond: "~Ebilmekteysem", "~Ebilmekteysen", "~Ebilmekteyse", "~Ebilmekteysek", "~Ebilmekteyseniz", "~Ebilmektelerse";
    pos_abil_necess__: "~Ebilmeliyim", "~Ebilmelisin", "~Ebilmeli", "~Ebilmeliyiz", "~Ebilmelisiniz", "~Ebilmeliler";
    pos_abil_necess_evid: "~Ebilmeliymişim", "~Ebilmeliymişsin", "~Ebilmeliymiş", "~Ebilmeliymişiz", "~Ebilmeliymişsiniz", "~Ebilmelilermiş";
    pos_abil_necess_past: "~Ebilmeliydim", "~Ebilmeliydin", "~Ebilmeliydi", "~Ebilmeliydik", "~Ebilmeliydiniz", "~Ebilmelilerdi";
    pos_abil_necess_cond: "~Ebilmeliysem", "~Ebilmeliysen", "~Ebilmeliyse", "~Ebilmeliysek", "~Ebilmeliyseniz", "~Ebilmelilerse";
    neg_plain_prog__: "~mIyorum", "~mIyorsun", "~mIyor", "~mIyoruz", "~mIyorsunuz", "~mIyorlar";
    neg_plain_prog_evid: "~mIyormuşum", "~mIyormuşsun", "~mIyormuş", "~mIyormuşuz", "~mIyormuşsunuz", "~mIyorlarmış";
    neg_plain_prog_past: "~mIyordum", "~mIyordun", "~mIyordu", "~mIyorduk", "~mIyordunuz", "~mIyorlardı";
    neg_plain_prog_cond: "~mIyorsam", "~mIyorsan", "~mIyorsa", "~mIyorsak", "~mIyorsanız", "~mIyorlarsa";
    neg_abil_aor__: "~EmEm", "~EmEzsIn", "~EmEz", "~EmEyIz", "~EmEzsInIz", "~EmEzlEr";
    neg_abil_aor_evid: "~EmEzmIşIm", "~EmEzmIşsIn", "~EmEzmIş", "~EmEzmIşIz", "~EmEzmIşsInIz", "~EmEzlErmIş";
    neg_abil_aor_past: "~EmEzdIm", "~EmEzdIn", "~EmEzdI", "~EmEzdIk", "~EmEzdInIz", "~EmEzlErdI";
    neg_abil_aor_cond: "~EmEzsEm", "~EmEzsEn", "~EmEzsE", "~EmEzsEk", "~EmEzsEnIz", "~EmEzlErsE";
    neg_abil_prog__: "~EmIyorum", "~EmIyorsun", "~EmIyor", "~EmIyoruz", "~EmIyorsunuz", "~EmIyorlar";
    neg_abil_prog_evid: "~EmIyormuşum", "~EmIyormuşsun", "~EmIyormuş", "~EmIyormuşuz", "~EmIyormuşsunuz", "~EmIyorlarmış";
    neg_abil_prog_past: "~EmIyordum", "~EmIyordun", "~EmIyordu", "~EmIyorduk", "~EmIyordunuz", "~EmIyorlardı";
    neg_abil_prog_cond: "~EmIyorsam", "~EmIyorsan", "~EmIyorsa", "~EmIyorsak", "~EmIyorsanız", "~EmIyorlarsa";
    neg_abil_evid__: "~EmEmIşIm", "~EmEmIşsIn", "~EmEmIş", "~EmEmIşIz", "~EmEmIşsInIz", "~EmEmIşlEr";
    neg_abil_evid_evid: "~EmEmIşmIşIm", "~EmEmIşmIşsIn", "~EmEmIşmIş", "~EmEmIşmIşIz", "~EmEmIşmIşsInIz", "~EmEmIşlErmIş";
    neg_abil_evid_past: "~EmEmIştIm", "~EmEmIştIn", "~EmEmIştI", "~EmEmIştIk", "~EmEmIştInIz", "~EmEmIşlErdI";
    neg_abil_evid_cond: "~EmEmIşsEm", "~EmEmIşsEn", "~EmEmIşsE", "~EmEmIşsEk", "~EmEmIşsEnIz", "~EmEmIşlErsE";
    neg_abil_fut__: "~EmEyEcEğIm", "~EmEyEcEksIn", "~EmEyEcEk", "~EmEyEcEğIz", "~EmEyEcEksInIz", "~EmEyEcEklEr";
    neg_abil_fut_evid: "~EmEyEcEkmIşIm", "~EmEyEcEkmIşsIn", "~EmEyEcEkmIş", "~EmEyEcEkmIşIz", "~EmEyEcEkmIşsInIz", "~EmEyEcEklErmIş";
    neg_abil_fut_past: "~EmEyEcEktIm", "~EmEyEcEktIn", "~EmEyEcEktI", "~EmEyEcEktIk", "~EmEyEcEktInIz", "~EmEyEcEklErdI";
    neg_abil_fut_cond: "~EmEyEcEksEm", "~EmEyEcEksEn", "~EmEyEcEksE", "~EmEyEcEksEk", "~EmEyEcEksEnIz", "~EmEyEcEklErsE";
    neg_abil_cont__: "~EmEmEktEyIm", "~EmEmEktEsIn", "~EmEmEktE", "~EmEmEktEyIz", "~EmEmEktEsInIz", "~EmEmEktElEr";
    neg_abil_cont_evid: "~EmEmEktEymIşIm", "~EmEmEktEymIşsIn", "~EmEmEktEymIş", "~EmEmEktEymIşIz", "~EmEmEktEymIşsInIz", "~EmEmEktElErmIş";
    neg_abil_cont_past: "~EmEmEktEydIm", "~EmEmEktEydIn", "~EmEmEktEydI", "~EmEmEktEydIk", "~EmEmEktEydInIz", "~EmEmEktElErdI";
    neg_abil_cont_cond: "~EmEmEktEysEm", "~EmEmEktEysEn", "~EmEmEktEysE", "~EmEmEktEysEk", "~EmEmEktEysEnIz", "~EmEmEktElErsE";
    neg_abil_necess__: "~EmEmElIyIm", "~EmEmElIsIn", "~EmEmElI", "~EmEmElIyIz", "~EmEmElIsInIz", "~EmEmElIlEr";
    neg_abil_necess_evid: "~EmEmElIymIşIm", "~EmEmElIymIşsIn", "~EmEmElIymIş", "~EmEmElIymIşIz", "~EmEmElIymIşsInIz", "~EmEmElIlErmIş";
    neg_abil_necess_past: "~EmEmElIydIm", "~EmEmElIydIn", "~EmEmElIydI", "~EmEmElIydIk", "~EmEmElIydInIz", "~EmEmElIlErdI";
    neg_abil_necess_cond: "~EmEmElIysEm", "~EmEmElIysEn", "~EmEmElIysE", "~EmEmElIysEk", "~EmEmElIysEnIz", "~EmEmElIlErsE";
    past_pos_plain__: "~dIm", _, _, _, _, _;
    past_pos_abil__: "~Ebildim", "~Ebildin", "~Ebildi", "~Ebildik", "~Ebildiniz", "~Ebildiler";
    past_pos_abil_past: "~Ebildiydim", "~Ebildiydin", "~Ebildiydi", "~Ebildiydik", "~Ebildiydiniz", "~Ebildilerdi";
    past_pos_abil_cond: "~Ebildiysem", "~Ebildiysen", "~Ebildiyse", "~Ebildiysek", "~Ebildiyseniz", "~Ebildilerse";
    past_neg_abil__: "~EmEdIm", "~EmEdIn", "~EmEdI", "~EmEdIk", "~EmEdInIz", "~EmEdIlEr";
    past_neg_abil_past: "~EmEdIydIm", "~EmEdIydIn", "~EmEdIydI", "~EmEdIydIk", "~EmEdIydInIz", "~EmEdIlErdI";
    past_neg_abil_cond: "~EmEdIysEm", "~EmEdIysEn", "~EmEdIysE", "~EmEdIysEk", "~EmEdIysEnIz", "~EmEdIlErsE";
    conditional_pos_plain__: "~sEm", "~sEn", "~sE", "~sEk", "~sEnIz", "~sElEr";
    conditional_pos_plain_evid: "~sEymIşIm", "~sEymIşsIn", "~sEymIş", "~sEymIşIz", "~sEymIşsInIz", "~sElErmIş";
    conditional_pos_plain_past: "~sEydIm", "~sEydIn", "~sEydI", "~sEydIk", "~sEydInIz", "~sElErdI";
    conditional_pos_abil__: "~Ebilsem", "~Ebilsen", "~Ebilse", "~Ebilsek", "~Ebilseniz", "~Ebilseler";
    conditional_pos_abil_evid: "~Ebilseymişim", "~Ebilseymişsin", "~Ebilseymiş", "~Ebilseymişiz", "~Ebilseymişsiniz", "~Ebilselermiş";
    conditional_pos_abil_past: "~Ebilseydim", "~Ebilseydin", "~Ebilseydi", "~Ebilseydik", "~Ebilseydiniz", "~Ebilselerdi";
    conditional_neg_abil__: "~EmEsEm", "~EmEsEn", "~EmEsE", "~EmEsEk", "~EmEsEnIz", "~EmEsElEr";
    conditional_neg_abil_evid: "~EmEsEymIşIm", "~EmEsEymIşsIn", "~EmEsEymIş", "~EmEsEymIşIz", "~EmEsEymIşsInIz", "~EmEsElErmIş";
    conditional_neg_abil_past: "~EmEsEydIm", "~EmEsEydIn", "~EmEsEydI", "~EmEsEydIk", "~EmEsEydInIz", "~EmEsElErdI";
    optative_pos_plain__: "~EyIm", "~EsIn", "~E", "~ElIm", "~EsInIz", "~ElEr";
    optative_pos_plain_evid: "~EymIşIm", "~EymIşsIn", "~EymIş", "~EymIşIz", "~EymIşsInIz", "~ElErmIş";
    optative_pos_plain_past: "~EydIm", "~EydIn", "~EydI", "~EydIk", "~EydInIz", "~ElErdI";
    optative_pos_abil__: "~Ebileyim", "~Ebilesin", "~Ebile", "~Ebilelim", "~Ebilesiniz", "~Ebileler";
    optative_pos_abil_evid: "~Ebileymişim", "~Ebileymişsin", "~Ebileymiş", "~Ebileymişiz", "~Ebileymişsiniz", "~Ebilelermiş";
    optative_pos_abil_past: "~Ebileydim", "~Ebileydin", "~Ebileydi", "~Ebileydik", "~Ebileydiniz", "~Ebilelerdi";
    optative_neg_abil__: "~EmEyEyIm", "~EmEyEsIn", "~EmEyE", "~EmEyElIm", "~EmEyEsInIz", "~EmEyElEr";
    optative_neg_abil_evid: "~EmEyEymIşIm", "~EmEyEymIşsIn", "~EmEyEymIş", "~EmEyEymIşIz", "~EmEyEymIşsInIz", "~EmEyElErmIş";
    optative_neg_abil_past: "~EmEyEydIm", "~EmEyEydIn", "~EmEyEydI", "~EmEyEydIk", "~EmEyEydInIz", "~EmEyElErdI";
    imperative_pos_plain: "~", "~sIn", "~In", "~sInlEr", _, "~InIz", "~sEnE", "~sEnIzE";
    imperative_pos_abil: "~Ebil", "~Ebilsin", "~Ebilin", "~Ebilsinler", "~Ebiliniz", _, "~Ebilsene", "~Ebilsenize";
    imperative_neg_plain: _, _, _, _, _, "~mEyInIz", _, _;
    imperative_neg_abil: "~EmE", "~EmEsIn", "~EmEyIn", "~EmEsInlEr", "~EmEyInIz", _, "~EmEsEnE", "~EmEsEnIzE";
    infinitive_pos: "~mak", "~Ebilmek";
    infinitive_neg: _, "~EmEmEk";
    verbal_noun_pos: "~mE", "~Ebilme";
    verbal_noun_neg: _, "~EmEmE";
    participle_pos_plain: "~En", _, _;
    participle_pos_abil: "~Ebilen", _, _;
    participle_neg_plain: _, "~mEmIş", "~mEyEcEk";
    participle_neg_abil: "~EmEyEn", _, _;
    relative_pos_plain_acak: _, _, _, _, _, "~EcEklErI";
    relative_pos_abil_dik: "~Ebildiğim", "~Ebildiğin", "~Ebildiği", "~Ebildiğimiz", "~Ebildiğiniz", "~Ebildikleri";
    relative_pos_abil_acak: _, "~Ebileceğin", "~Ebileceği", "~Ebileceğimiz", "~Ebileceğiniz", "~Ebilecekleri";
    relative_neg_plain_acak: "~mEyEcEğIm", _, _, _, _, _;
    relative_neg_abil_dik: "~EmEdIğIm", "~EmEdIğIn", "~EmEdIğI", "~EmEdIğImIz", "~EmEdIğInIz", "~EmEdIklErI";
    relative_neg_abil_acak: _, "~EmEyEcEğIn", "~EmEyEcEğI", "~EmEyEcEğImIz", "~EmEyEcEğInIz", "~EmEyEcEklErI";
    converb_pos_plain: "~Irken", _, _, "~ElI", "~EsIyE", _, "~ErEk", "~a ~a", "~IrcEsInE";
    converb_pos_abil: "~EbilIrken", "~Ebilince", "~Ebilip", "~Ebileli", "~Ebilesiye", "~Ebildikçe", "~Ebilerek", "~abile ~abile", _;
    converb_neg_abil: "~EmEzken", "~EmEyIncE", "~EmEyIp", "~EmEyElI", "~EmEyEsIyE", "~EmEdIkçE", "~EmEyErEk", _, "~EmEzcEsInE";
    ),
    // Vermek: 483 words (çizmek, direnmek, tüketmek, bilmek, vermek, gelmek). Like Ayırmak, except in 3 cells: infinitive_pos plain, converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("vermek", 483, "~mek", like Ayırmak;
    stem converb_neg_plain_doubled: "~meye ~meye- like ayırmak";
    stem converb_neg_abil_doubled: "~emeye ~emeye- like ayırmak";
    infinitive_pos: "~mek", _;
    converb_pos_plain: _, _, _, _, _, _, _, "~e ~e", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~ebile ~ebile", _;
    ),
    // Patlamak: 266 words (oynamak, ağlamak, zorlamak, sağlamak, atlamak, kapamak).
    conjugation!("patlamak", 266, "~amak";
    stem converb_neg_plain_doubled: "~amaya ~amaya- like ayırmak";
    stem converb_neg_abil_doubled: "~ayamaya ~ayamaya- like ayırmak";
    pos_plain_aor__: "~ErIm", "~ErsIn", "~Er", _, "~ErsInIz", "~ErlEr";
    pos_plain_aor_evid: "~ErmIşIm", "~ErmIşsIn", "~ErmIş", "~ErmIşIz", "~ErmIşsInIz", "~ErlErmIş";
    pos_plain_aor_past: "~ErdIm", "~ErdIn", "~ErdI", "~ErdIk", "~ErdInIz", "~ErlErdI";
    pos_plain_aor_cond: "~ErsEm", "~ErsEn", "~ErsE", "~ErsEk", "~ErsEnIz", "~ErlErsE";
    pos_plain_prog__: "~Iyorum", _, _, _, _, _;
    pos_plain_evid__: _, "~EmIşsIn", "~EmIş", _, "~EmIşsInIz", "~EmIşlEr";
    pos_plain_evid_evid: "~EmIşmIşIm", "~EmIşmIşsIn", "~EmIşmIş", "~EmIşmIşIz", "~EmIşmIşsInIz", "~EmIşlErmIş";
    pos_plain_evid_past: "~EmIştIm", "~EmIştIn", "~EmIştI", "~EmIştIk", "~EmIştInIz", "~EmIşlErdI";
    pos_plain_evid_cond: "~EmIşsEm", "~EmIşsEn", "~EmIşsE", "~EmIşsEk", "~EmIşsEnIz", "~EmIşlErsE";
    pos_plain_fut__: "~EyEcEğIm", "~EyEcEksIn", "~EyEcEk", _, "~EyEcEksInIz", "~EyEcEklEr";
    pos_plain_fut_evid: "~EyEcEkmIşIm", "~EyEcEkmIşsIn", "~EyEcEkmIş", "~EyEcEkmIşIz", "~EyEcEkmIşsInIz", "~EyEcEklErmIş";
    pos_plain_fut_past: "~EyEcEktIm", "~EyEcEktIn", "~EyEcEktI", "~EyEcEktIk", "~EyEcEktInIz", "~EyEcEklErdI";
    pos_plain_fut_cond: "~EyEcEksEm", "~EyEcEksEn", "~EyEcEksE", "~EyEcEksEk", "~EyEcEksEnIz", "~EyEcEklErsE";
    pos_abil_aor__: "~EyEbilirim", "~EyEbilirsin", "~EyEbilir", "~EyEbiliriz", "~EyEbilirsiniz", "~EyEbilirler";
    pos_abil_aor_evid: "~EyEbilirmişim", "~EyEbilirmişsin", "~EyEbilirmiş", "~EyEbilirmişiz", "~EyEbilirmişsiniz", "~EyEbilirlermiş";
    pos_abil_aor_past: "~EyEbilirdim", "~EyEbilirdin", "~EyEbilirdi", "~EyEbilirdik", "~EyEbilirdiniz", "~EyEbilirlerdi";
    pos_abil_aor_cond: "~EyEbilirsem", "~EyEbilirsen", "~EyEbilirse", "~EyEbilirsek", "~EyEbilirseniz", "~EyEbilirlerse";
    pos_abil_prog__: "~EyEbiliyorum", "~EyEbiliyorsun", "~EyEbiliyor", "~EyEbiliyoruz", "~EyEbiliyorsunuz", "~EyEbiliyorlar";
    pos_abil_prog_evid: "~EyEbiliyormuşum", "~EyEbiliyormuşsun", "~EyEbiliyormuş", "~EyEbiliyormuşuz", "~EyEbiliyormuşsunuz", "~EyEbiliyorlarmış";
    pos_abil_prog_past: "~EyEbiliyordum", "~EyEbiliyordun", "~EyEbiliyordu", "~EyEbiliyorduk", "~EyEbiliyordunuz", "~EyEbiliyorlardı";
    pos_abil_prog_cond: "~EyEbiliyorsam", "~EyEbiliyorsan", "~EyEbiliyorsa", "~EyEbiliyorsak", "~EyEbiliyorsanız", "~EyEbiliyorlarsa";
    pos_abil_evid__: "~EyEbilmişim", "~EyEbilmişsin", "~EyEbilmiş", "~EyEbilmişiz", "~EyEbilmişsiniz", "~EyEbilmişler";
    pos_abil_evid_evid: "~EyEbilmişmişim", "~EyEbilmişmişsin", "~EyEbilmişmiş", "~EyEbilmişmişiz", "~EyEbilmişmişsiniz", "~EyEbilmişlermiş";
    pos_abil_evid_past: "~EyEbilmiştim", "~EyEbilmiştin", "~EyEbilmişti", "~EyEbilmiştik", "~EyEbilmiştiniz", "~EyEbilmişlerdi";
    pos_abil_evid_cond: "~EyEbilmişsem", "~EyEbilmişsen", "~EyEbilmişse", "~EyEbilmişsek", "~EyEbilmişseniz", "~EyEbilmişlerse";
    pos_abil_fut__: "~EyEbileceğim", "~EyEbileceksin", "~EyEbilecek", "~EyEbileceğiz", "~EyEbileceksiniz", "~EyEbilecekler";
    pos_abil_fut_evid: "~EyEbilecekmişim", "~EyEbilecekmişsin", "~EyEbilecekmiş", "~EyEbilecekmişiz", "~EyEbilecekmişsiniz", "~EyEbileceklermiş";
    pos_abil_fut_past: "~EyEbilecektim", "~EyEbilecektin", "~EyEbilecekti", "~EyEbilecektik", "~EyEbilecektiniz", "~EyEbileceklerdi";
    pos_abil_fut_cond: "~EyEbileceksem", "~EyEbileceksen", "~EyEbilecekse", "~EyEbileceksek", "~EyEbilecekseniz", "~EyEbileceklerse";
    pos_abil_cont__: "~EyEbilmekteyim", "~EyEbilmektesin", "~EyEbilmekte", "~EyEbilmekteyiz", "~EyEbilmektesiniz", "~EyEbilmekteler";
    pos_abil_cont_evid: "~EyEbilmekteymişim", "~EyEbilmekteymişsin", "~EyEbilmekteymiş", "~EyEbilmekteymişiz", "~EyEbilmekteymişsiniz", "~EyEbilmektelermiş";
    pos_abil_cont_past: "~EyEbilmekteydim", "~EyEbilmekteydin", "~EyEbilmekteydi", "~EyEbilmekteydik", "~EyEbilmekteydiniz", "~EyEbilmektelerdi";
    pos_abil_cont_cond: "~EyEbilmekteysem", "~EyEbilmekteysen", "~EyEbilmekteyse", "~EyEbilmekteysek", "~EyEbilmekteyseniz", "~EyEbilmektelerse";
    pos_abil_necess__: "~EyEbilmeliyim", "~EyEbilmelisin", "~EyEbilmeli", "~EyEbilmeliyiz", "~EyEbilmelisiniz", "~EyEbilmeliler";
    pos_abil_necess_evid: "~EyEbilmeliymişim", "~EyEbilmeliymişsin", "~EyEbilmeliymiş", "~EyEbilmeliymişiz", "~EyEbilmeliymişsiniz", "~EyEbilmelilermiş";
    pos_abil_necess_past: "~EyEbilmeliydim", "~EyEbilmeliydin", "~EyEbilmeliydi", "~EyEbilmeliydik", "~EyEbilmeliydiniz", "~EyEbilmelilerdi";
    pos_abil_necess_cond: "~EyEbilmeliysem", "~EyEbilmeliysen", "~EyEbilmeliyse", "~EyEbilmeliysek", "~EyEbilmeliyseniz", "~EyEbilmelilerse";
    neg_plain_prog__: "~EmIyorum", "~EmIyorsun", "~EmIyor", "~EmIyoruz", "~EmIyorsunuz", "~EmIyorlar";
    neg_plain_prog_evid: "~EmIyormuşum", "~EmIyormuşsun", "~EmIyormuş", "~EmIyormuşuz", "~EmIyormuşsunuz", "~EmIyorlarmış";
    neg_plain_prog_past: "~EmIyordum", "~EmIyordun", "~EmIyordu", "~EmIyorduk", "~EmIyordunuz", "~EmIyorlardı";
    neg_plain_prog_cond: "~EmIyorsam", "~EmIyorsan", "~EmIyorsa", "~EmIyorsak", "~EmIyorsanız", "~EmIyorlarsa";
    neg_abil_aor__: "~EyEmEm", "~EyEmEzsIn", "~EyEmEz", "~EyEmEyIz", "~EyEmEzsInIz", "~EyEmEzlEr";
    neg_abil_aor_evid: "~EyEmEzmIşIm", "~EyEmEzmIşsIn", "~EyEmEzmIş", "~EyEmEzmIşIz", "~EyEmEzmIşsInIz", "~EyEmEzlErmIş";
    neg_abil_aor_past: "~EyEmEzdIm", "~EyEmEzdIn", "~EyEmEzdI", "~EyEmEzdIk", "~EyEmEzdInIz", "~EyEmEzlErdI";
    neg_abil_aor_cond: "~EyEmEzsEm", "~EyEmEzsEn", "~EyEmEzsE", "~EyEmEzsEk", "~EyEmEzsEnIz", "~EyEmEzlErsE";
    neg_abil_prog__: "~EyEmIyorum", "~EyEmIyorsun", "~EyEmIyor", "~EyEmIyoruz", "~EyEmIyorsunuz", "~EyEmIyorlar";
    neg_abil_prog_evid: "~EyEmIyormuşum", "~EyEmIyormuşsun", "~EyEmIyormuş", "~EyEmIyormuşuz", "~EyEmIyormuşsunuz", "~EyEmIyorlarmış";
    neg_abil_prog_past: "~EyEmIyordum", "~EyEmIyordun", "~EyEmIyordu", "~EyEmIyorduk", "~EyEmIyordunuz", "~EyEmIyorlardı";
    neg_abil_prog_cond: "~EyEmIyorsam", "~EyEmIyorsan", "~EyEmIyorsa", "~EyEmIyorsak", "~EyEmIyorsanız", "~EyEmIyorlarsa";
    neg_abil_evid__: "~EyEmEmIşIm", "~EyEmEmIşsIn", "~EyEmEmIş", "~EyEmEmIşIz", "~EyEmEmIşsInIz", "~EyEmEmIşlEr";
    neg_abil_evid_evid: "~EyEmEmIşmIşIm", "~EyEmEmIşmIşsIn", "~EyEmEmIşmIş", "~EyEmEmIşmIşIz", "~EyEmEmIşmIşsInIz", "~EyEmEmIşlErmIş";
    neg_abil_evid_past: "~EyEmEmIştIm", "~EyEmEmIştIn", "~EyEmEmIştI", "~EyEmEmIştIk", "~EyEmEmIştInIz", "~EyEmEmIşlErdI";
    neg_abil_evid_cond: "~EyEmEmIşsEm", "~EyEmEmIşsEn", "~EyEmEmIşsE", "~EyEmEmIşsEk", "~EyEmEmIşsEnIz", "~EyEmEmIşlErsE";
    neg_abil_fut__: "~EyEmEyEcEğIm", "~EyEmEyEcEksIn", "~EyEmEyEcEk", "~EyEmEyEcEğIz", "~EyEmEyEcEksInIz", "~EyEmEyEcEklEr";
    neg_abil_fut_evid: "~EyEmEyEcEkmIşIm", "~EyEmEyEcEkmIşsIn", "~EyEmEyEcEkmIş", "~EyEmEyEcEkmIşIz", "~EyEmEyEcEkmIşsInIz", "~EyEmEyEcEklErmIş";
    neg_abil_fut_past: "~EyEmEyEcEktIm", "~EyEmEyEcEktIn", "~EyEmEyEcEktI", "~EyEmEyEcEktIk", "~EyEmEyEcEktInIz", "~EyEmEyEcEklErdI";
    neg_abil_fut_cond: "~EyEmEyEcEksEm", "~EyEmEyEcEksEn", "~EyEmEyEcEksE", "~EyEmEyEcEksEk", "~EyEmEyEcEksEnIz", "~EyEmEyEcEklErsE";
    neg_abil_cont__: "~EyEmEmEktEyIm", "~EyEmEmEktEsIn", "~EyEmEmEktE", "~EyEmEmEktEyIz", "~EyEmEmEktEsInIz", "~EyEmEmEktElEr";
    neg_abil_cont_evid: "~EyEmEmEktEymIşIm", "~EyEmEmEktEymIşsIn", "~EyEmEmEktEymIş", "~EyEmEmEktEymIşIz", "~EyEmEmEktEymIşsInIz", "~EyEmEmEktElErmIş";
    neg_abil_cont_past: "~EyEmEmEktEydIm", "~EyEmEmEktEydIn", "~EyEmEmEktEydI", "~EyEmEmEktEydIk", "~EyEmEmEktEydInIz", "~EyEmEmEktElErdI";
    neg_abil_cont_cond: "~EyEmEmEktEysEm", "~EyEmEmEktEysEn", "~EyEmEmEktEysE", "~EyEmEmEktEysEk", "~EyEmEmEktEysEnIz", "~EyEmEmEktElErsE";
    neg_abil_necess__: "~EyEmEmElIyIm", "~EyEmEmElIsIn", "~EyEmEmElI", "~EyEmEmElIyIz", "~EyEmEmElIsInIz", "~EyEmEmElIlEr";
    neg_abil_necess_evid: "~EyEmEmElIymIşIm", "~EyEmEmElIymIşsIn", "~EyEmEmElIymIş", "~EyEmEmElIymIşIz", "~EyEmEmElIymIşsInIz", "~EyEmEmElIlErmIş";
    neg_abil_necess_past: "~EyEmEmElIydIm", "~EyEmEmElIydIn", "~EyEmEmElIydI", "~EyEmEmElIydIk", "~EyEmEmElIydInIz", "~EyEmEmElIlErdI";
    neg_abil_necess_cond: "~EyEmEmElIysEm", "~EyEmEmElIysEn", "~EyEmEmElIysE", "~EyEmEmElIysEk", "~EyEmEmElIysEnIz", "~EyEmEmElIlErsE";
    past_pos_plain__: "~EdIm", _, _, _, _, _;
    past_pos_abil__: "~EyEbildim", "~EyEbildin", "~EyEbildi", "~EyEbildik", "~EyEbildiniz", "~EyEbildiler";
    past_pos_abil_past: "~EyEbildiydim", "~EyEbildiydin", "~EyEbildiydi", "~EyEbildiydik", "~EyEbildiydiniz", "~EyEbildilerdi";
    past_pos_abil_cond: "~EyEbildiysem", "~EyEbildiysen", "~EyEbildiyse", "~EyEbildiysek", "~EyEbildiyseniz", "~EyEbildilerse";
    past_neg_abil__: "~EyEmEdIm", "~EyEmEdIn", "~EyEmEdI", "~EyEmEdIk", "~EyEmEdInIz", "~EyEmEdIlEr";
    past_neg_abil_past: "~EyEmEdIydIm", "~EyEmEdIydIn", "~EyEmEdIydI", "~EyEmEdIydIk", "~EyEmEdIydInIz", "~EyEmEdIlErdI";
    past_neg_abil_cond: "~EyEmEdIysEm", "~EyEmEdIysEn", "~EyEmEdIysE", "~EyEmEdIysEk", "~EyEmEdIysEnIz", "~EyEmEdIlErsE";
    conditional_pos_plain__: "~EsEm", "~EsEn", "~EsE", "~EsEk", "~EsEnIz", "~EsElEr";
    conditional_pos_plain_evid: "~EsEymIşIm", "~EsEymIşsIn", "~EsEymIş", "~EsEymIşIz", "~EsEymIşsInIz", "~EsElErmIş";
    conditional_pos_plain_past: "~EsEydIm", "~EsEydIn", "~EsEydI", "~EsEydIk", "~EsEydInIz", "~EsElErdI";
    conditional_pos_abil__: "~EyEbilsem", "~EyEbilsen", "~EyEbilse", "~EyEbilsek", "~EyEbilseniz", "~EyEbilseler";
    conditional_pos_abil_evid: "~EyEbilseymişim", "~EyEbilseymişsin", "~EyEbilseymiş", "~EyEbilseymişiz", "~EyEbilseymişsiniz", "~EyEbilselermiş";
    conditional_pos_abil_past: "~EyEbilseydim", "~EyEbilseydin", "~EyEbilseydi", "~EyEbilseydik", "~EyEbilseydiniz", "~EyEbilselerdi";
    conditional_neg_abil__: "~EyEmEsEm", "~EyEmEsEn", "~EyEmEsE", "~EyEmEsEk", "~EyEmEsEnIz", "~EyEmEsElEr";
    conditional_neg_abil_evid: "~EyEmEsEymIşIm", "~EyEmEsEymIşsIn", "~EyEmEsEymIş", "~EyEmEsEymIşIz", "~EyEmEsEymIşsInIz", "~EyEmEsElErmIş";
    conditional_neg_abil_past: "~EyEmEsEydIm", "~EyEmEsEydIn", "~EyEmEsEydI", "~EyEmEsEydIk", "~EyEmEsEydInIz", "~EyEmEsElErdI";
    optative_pos_plain__: "~EyEyIm", "~EyEsIn", "~EyE", "~EyElIm", "~EyEsInIz", "~EyElEr";
    optative_pos_plain_evid: "~EyEymIşIm", "~EyEymIşsIn", "~EyEymIş", "~EyEymIşIz", "~EyEymIşsInIz", "~EyElErmIş";
    optative_pos_plain_past: "~EyEydIm", "~EyEydIn", "~EyEydI", "~EyEydIk", "~EyEydInIz", "~EyElErdI";
    optative_pos_abil__: "~EyEbileyim", "~EyEbilesin", "~EyEbile", "~EyEbilelim", "~EyEbilesiniz", "~EyEbileler";
    optative_pos_abil_evid: "~EyEbileymişim", "~EyEbileymişsin", "~EyEbileymiş", "~EyEbileymişiz", "~EyEbileymişsiniz", "~EyEbilelermiş";
    optative_pos_abil_past: "~EyEbileydim", "~EyEbileydin", "~EyEbileydi", "~EyEbileydik", "~EyEbileydiniz", "~EyEbilelerdi";
    optative_neg_abil__: "~EyEmEyEyIm", "~EyEmEyEsIn", "~EyEmEyE", "~EyEmEyElIm", "~EyEmEyEsInIz", "~EyEmEyElEr";
    optative_neg_abil_evid: "~EyEmEyEymIşIm", "~EyEmEyEymIşsIn", "~EyEmEyEymIş", "~EyEmEyEymIşIz", "~EyEmEyEymIşsInIz", "~EyEmEyElErmIş";
    optative_neg_abil_past: "~EyEmEyEydIm", "~EyEmEyEydIn", "~EyEmEyEydI", "~EyEmEyEydIk", "~EyEmEyEydInIz", "~EyEmEyElErdI";
    imperative_pos_plain: "~E", "~EsIn", "~EyIn", "~EsInlEr", _, "~EyInIz", "~EsEnE", "~EsEnIzE";
    imperative_pos_abil: "~EyEbil", "~EyEbilsin", "~EyEbilin", "~EyEbilsinler", "~EyEbiliniz", _, "~EyEbilsene", "~EyEbilsenize";
    imperative_neg_plain: _, _, _, _, _, "~EmEyInIz", _, _;
    imperative_neg_abil: "~EyEmE", "~EyEmEsIn", "~EyEmEyIn", "~EyEmEsInlEr", "~EyEmEyInIz", _, "~EyEmEsEnE", "~EyEmEsEnIzE";
    infinitive_pos: "~amak", "~EyEbilmek";
    infinitive_neg: _, "~EyEmEmEk";
    verbal_noun_pos: "~EmE", "~EyEbilme";
    verbal_noun_neg: _, "~EyEmEmE";
    participle_pos_plain: "~EyEn", _, _;
    participle_pos_abil: "~EyEbilen", _, _;
    participle_neg_plain: _, "~EmEmIş", "~EmEyEcEk";
    participle_neg_abil: "~EyEmEyEn", _, _;
    relative_pos_plain_acak: _, _, _, _, _, "~EyEcEklErI";
    relative_pos_abil_dik: "~EyEbildiğim", "~EyEbildiğin", "~EyEbildiği", "~EyEbildiğimiz", "~EyEbildiğiniz", "~EyEbildikleri";
    relative_pos_abil_acak: _, "~EyEbileceğin", "~EyEbileceği", "~EyEbileceğimiz", "~EyEbileceğiniz", "~EyEbilecekleri";
    relative_neg_plain_acak: "~EmEyEcEğIm", _, _, _, _, _;
    relative_neg_abil_dik: "~EyEmEdIğIm", "~EyEmEdIğIn", "~EyEmEdIğI", "~EyEmEdIğImIz", "~EyEmEdIğInIz", "~EyEmEdIklErI";
    relative_neg_abil_acak: _, "~EyEmEyEcEğIn", "~EyEmEyEcEğI", "~EyEmEyEcEğImIz", "~EyEmEyEcEğInIz", "~EyEmEyEcEklErI";
    converb_pos_plain: "~Erken", _, _, "~EyElI", "~EyEsIyE", _, "~EyErEk", "~aya ~aya", "~ErcEsInE";
    converb_pos_abil: "~EyEbilIrken", "~EyEbilince", "~EyEbilip", "~EyEbileli", "~EyEbilesiye", "~EyEbildikçe", "~EyEbilerek", "~ayabile ~ayabile", _;
    converb_neg_abil: "~EyEmEzken", "~EyEmEyIncE", "~EyEmEyIp", "~EyEmEyElI", "~EyEmEyEsIyE", "~EyEmEdIkçE", "~EyEmEyErEk", _, "~EyEmEzcEsInE";
    ),
    // Takipetmek: 186 words (terk etmek, nefret etmek, kabul etmek, tahliye etmek, itiraf etmek, taklit etmek).
    conjugation!("takip etmek", 186, "~tmek";
    stem lemma: "~- like takip etmek";
    stem converb_neg_plain_doubled: "~tmeye Etmeye- like ayırmak";
    stem converb_neg_abil_doubled: "~demeye edemeye- like ayırmak";
    imperative_pos_abil: _, _, _, _, _, derived, _, _;
    imperative_neg_abil: _, _, _, _, _, derived, _, _;
    participle_pos_plain: _, derived, derived;
    participle_pos_abil: _, derived, derived;
    participle_neg_abil: _, derived, derived;
    relative_pos_plain_acak: derived, _, _, _, _, _;
    relative_pos_abil_acak: derived, _, _, _, _, _;
    relative_neg_abil_acak: derived, _, _, _, _, _;
    ),
    // Beslemek: 149 words (ezberlemek, gülümsemek, titremek, ertelemek, istemek, bellemek). Like Patlamak, except in 3 cells: infinitive_pos plain, converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("beslemek", 149, "~emek", like Patlamak;
    stem converb_neg_plain_doubled: "~emeye ~emeye- like ayırmak";
    stem converb_neg_abil_doubled: "~eyemeye ~eyemeye- like ayırmak";
    infinitive_pos: "~emek", _;
    converb_pos_plain: _, _, _, _, _, _, _, "~eye ~eye", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~eyebile ~eyebile", _;
    ),
    // Açmak: 124 words (batmak, koşmak, atmak, yazmak, kurmak, yakmak). Like Ayırmak, except in 26 cells: pos_plain_aor__ _1sg, pos_plain_aor__ _2sg, pos_plain_aor__ _3sg, pos_plain_aor__ _2pl, pos_plain_aor__ _3pl, pos_plain_aor_evid _1sg, ….
    conjugation!("açmak", 124, "~mak", like Ayırmak;
    pos_plain_aor__: "~ErIm", "~ErsIn", "~Er", _, "~ErsInIz", "~ErlEr";
    pos_plain_aor_evid: "~ErmIşIm", "~ErmIşsIn", "~ErmIş", "~ErmIşIz", "~ErmIşsInIz", "~ErlErmIş";
    pos_plain_aor_past: "~ErdIm", "~ErdIn", "~ErdI", "~ErdIk", "~ErdInIz", "~ErlErdI";
    pos_plain_aor_cond: "~ErsEm", "~ErsEn", "~ErsE", "~ErsEk", "~ErsEnIz", "~ErlErsE";
    past_pos_plain__: "~tIm", _, _, _, _, _;
    converb_pos_plain: "~Erken", _, _, _, _, _, _, _, "~ErcEsInE";
    ),
    // Silmek: 107 words (ekmek, silmek, gülmek, bitmek, vazgeçmek, yenmek). Like Açmak, except in 4 cells: past_pos_plain__ _1sg, infinitive_pos plain, converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("silmek", 107, "~mek", like Açmak;
    stem converb_neg_plain_doubled: "~meye ~meye- like ayırmak";
    stem converb_neg_abil_doubled: "~emeye ~emeye- like ayırmak";
    past_pos_plain__: "~dIm", _, _, _, _, _;
    infinitive_pos: "~mek", _;
    converb_pos_plain: _, _, _, _, _, _, _, "~e ~e", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~ebile ~ebile", _;
    ),
    // Gitmek: 53 words (reddetmek, gitmek, etmek, affetmek, seyretmek, keşfetmek). Like Takipetmek, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("gitmek", 53, "~tmek", like Takipetmek;
    stem converb_neg_plain_doubled: "~tmeye ~tmeye- like ayırmak";
    stem converb_neg_abil_doubled: "~demeye ~demeye- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~de ~de", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~debile ~debile", _;
    ),
    // Elevermek: 33 words (söz vermek, ateşe vermek, yanıt vermek, cevap vermek, karar vermek, can vermek). Like Vermek, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("ele vermek", 33, "~mek", like Vermek;
    stem converb_neg_plain_doubled: "~mEyE vErmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE vErEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E vErE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile vErEbile", _;
    ),
    // Varolmak: 31 words (pişman olmak, tıraş olmak, bir avuç toprak olmak, rahmetli olmak, helak olmak, toprak olmak). Like Ayırmak, except in 27 cells: pos_abil_aor__ _1sg, pos_abil_aor__ _2sg, pos_abil_aor__ _3sg, pos_abil_aor__ _1pl, pos_abil_aor__ _2pl, pos_abil_aor__ _3pl, ….
    conjugation!("var olmak", 31, "~mak", like Ayırmak;
    stem converb_neg_plain_doubled: "~mEyE AlmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE AlEmEyE- like ayırmak";
    pos_abil_aor__: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_evid: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_past: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_cond: derived, derived, derived, derived, derived, derived;
    converb_pos_plain: _, _, _, _, _, _, _, "~E AlE", _;
    converb_pos_abil: "~Ebilurken", _, _, _, _, _, _, "~Ebile AlEbile", _;
    ),
    // Taşımak: 25 words (acımak, ağrımak, solumak, tanımak, okumak, korumak).
    conjugation!("taşımak", 25, "~mak";
    stem converb_neg_plain_doubled: "~maya ~maya- like ayırmak";
    stem converb_neg_abil_doubled: "~yamaya ~yamaya- like ayırmak";
    pos_plain_aor__: "~rIm", "~rsIn", "~r", _, "~rsInIz", "~rlEr";
    pos_plain_aor_evid: "~rmIşIm", "~rmIşsIn", "~rmIş", "~rmIşIz", "~rmIşsInIz", "~rlErmIş";
    pos_plain_aor_past: "~rdIm", "~rdIn", "~rdI", "~rdIk", "~rdInIz", "~rlErdI";
    pos_plain_aor_cond: "~rsEm", "~rsEn", "~rsE", "~rsEk", "~rsEnIz", "~rlErsE";
    pos_plain_prog__: "~yorum", _, _, _, _, _;
    pos_plain_evid__: _, "~mIşsIn", "~mIş", _, "~mIşsInIz", "~mIşlEr";
    pos_plain_evid_evid: "~mIşmIşIm", "~mIşmIşsIn", "~mIşmIş", "~mIşmIşIz", "~mIşmIşsInIz", "~mIşlErmIş";
    pos_plain_evid_past: "~mIştIm", "~mIştIn", "~mIştI", "~mIştIk", "~mIştInIz", "~mIşlErdI";
    pos_plain_evid_cond: "~mIşsEm", "~mIşsEn", "~mIşsE", "~mIşsEk", "~mIşsEnIz", "~mIşlErsE";
    pos_plain_fut__: "~yEcEğIm", "~yEcEksIn", "~yEcEk", _, "~yEcEksInIz", "~yEcEklEr";
    pos_plain_fut_evid: "~yEcEkmIşIm", "~yEcEkmIşsIn", "~yEcEkmIş", "~yEcEkmIşIz", "~yEcEkmIşsInIz", "~yEcEklErmIş";
    pos_plain_fut_past: "~yEcEktIm", "~yEcEktIn", "~yEcEktI", "~yEcEktIk", "~yEcEktInIz", "~yEcEklErdI";
    pos_plain_fut_cond: "~yEcEksEm", "~yEcEksEn", "~yEcEksE", "~yEcEksEk", "~yEcEksEnIz", "~yEcEklErsE";
    pos_abil_aor__: "~yEbilirim", "~yEbilirsin", "~yEbilir", "~yEbiliriz", "~yEbilirsiniz", "~yEbilirler";
    pos_abil_aor_evid: "~yEbilirmişim", "~yEbilirmişsin", "~yEbilirmiş", "~yEbilirmişiz", "~yEbilirmişsiniz", "~yEbilirlermiş";
    pos_abil_aor_past: "~yEbilirdim", "~yEbilirdin", "~yEbilirdi", "~yEbilirdik", "~yEbilirdiniz", "~yEbilirlerdi";
    pos_abil_aor_cond: "~yEbilirsem", "~yEbilirsen", "~yEbilirse", "~yEbilirsek", "~yEbilirseniz", "~yEbilirlerse";
    pos_abil_prog__: "~yEbiliyorum", "~yEbiliyorsun", "~yEbiliyor", "~yEbiliyoruz", "~yEbiliyorsunuz", "~yEbiliyorlar";
    pos_abil_prog_evid: "~yEbiliyormuşum", "~yEbiliyormuşsun", "~yEbiliyormuş", "~yEbiliyormuşuz", "~yEbiliyormuşsunuz", "~yEbiliyorlarmış";
    pos_abil_prog_past: "~yEbiliyordum", "~yEbiliyordun", "~yEbiliyordu", "~yEbiliyorduk", "~yEbiliyordunuz", "~yEbiliyorlardı";
    pos_abil_prog_cond: "~yEbiliyorsam", "~yEbiliyorsan", "~yEbiliyorsa", "~yEbiliyorsak", "~yEbiliyorsanız", "~yEbiliyorlarsa";
    pos_abil_evid__: "~yEbilmişim", "~yEbilmişsin", "~yEbilmiş", "~yEbilmişiz", "~yEbilmişsiniz", "~yEbilmişler";
    pos_abil_evid_evid: "~yEbilmişmişim", "~yEbilmişmişsin", "~yEbilmişmiş", "~yEbilmişmişiz", "~yEbilmişmişsiniz", "~yEbilmişlermiş";
    pos_abil_evid_past: "~yEbilmiştim", "~yEbilmiştin", "~yEbilmişti", "~yEbilmiştik", "~yEbilmiştiniz", "~yEbilmişlerdi";
    pos_abil_evid_cond: "~yEbilmişsem", "~yEbilmişsen", "~yEbilmişse", "~yEbilmişsek", "~yEbilmişseniz", "~yEbilmişlerse";
    pos_abil_fut__: "~yEbileceğim", "~yEbileceksin", "~yEbilecek", "~yEbileceğiz", "~yEbileceksiniz", "~yEbilecekler";
    pos_abil_fut_evid: "~yEbilecekmişim", "~yEbilecekmişsin", "~yEbilecekmiş", "~yEbilecekmişiz", "~yEbilecekmişsiniz", "~yEbileceklermiş";
    pos_abil_fut_past: "~yEbilecektim", "~yEbilecektin", "~yEbilecekti", "~yEbilecektik", "~yEbilecektiniz", "~yEbileceklerdi";
    pos_abil_fut_cond: "~yEbileceksem", "~yEbileceksen", "~yEbilecekse", "~yEbileceksek", "~yEbilecekseniz", "~yEbileceklerse";
    pos_abil_cont__: "~yEbilmekteyim", "~yEbilmektesin", "~yEbilmekte", "~yEbilmekteyiz", "~yEbilmektesiniz", "~yEbilmekteler";
    pos_abil_cont_evid: "~yEbilmekteymişim", "~yEbilmekteymişsin", "~yEbilmekteymiş", "~yEbilmekteymişiz", "~yEbilmekteymişsiniz", "~yEbilmektelermiş";
    pos_abil_cont_past: "~yEbilmekteydim", "~yEbilmekteydin", "~yEbilmekteydi", "~yEbilmekteydik", "~yEbilmekteydiniz", "~yEbilmektelerdi";
    pos_abil_cont_cond: "~yEbilmekteysem", "~yEbilmekteysen", "~yEbilmekteyse", "~yEbilmekteysek", "~yEbilmekteyseniz", "~yEbilmektelerse";
    pos_abil_necess__: "~yEbilmeliyim", "~yEbilmelisin", "~yEbilmeli", "~yEbilmeliyiz", "~yEbilmelisiniz", "~yEbilmeliler";
    pos_abil_necess_evid: "~yEbilmeliymişim", "~yEbilmeliymişsin", "~yEbilmeliymiş", "~yEbilmeliymişiz", "~yEbilmeliymişsiniz", "~yEbilmelilermiş";
    pos_abil_necess_past: "~yEbilmeliydim", "~yEbilmeliydin", "~yEbilmeliydi", "~yEbilmeliydik", "~yEbilmeliydiniz", "~yEbilmelilerdi";
    pos_abil_necess_cond: "~yEbilmeliysem", "~yEbilmeliysen", "~yEbilmeliyse", "~yEbilmeliysek", "~yEbilmeliyseniz", "~yEbilmelilerse";
    neg_plain_prog__: "~mIyorum", "~mIyorsun", "~mIyor", "~mIyoruz", "~mIyorsunuz", "~mIyorlar";
    neg_plain_prog_evid: "~mIyormuşum", "~mIyormuşsun", "~mIyormuş", "~mIyormuşuz", "~mIyormuşsunuz", "~mIyorlarmış";
    neg_plain_prog_past: "~mIyordum", "~mIyordun", "~mIyordu", "~mIyorduk", "~mIyordunuz", "~mIyorlardı";
    neg_plain_prog_cond: "~mIyorsam", "~mIyorsan", "~mIyorsa", "~mIyorsak", "~mIyorsanız", "~mIyorlarsa";
    neg_abil_aor__: "~yEmEm", "~yEmEzsIn", "~yEmEz", "~yEmEyIz", "~yEmEzsInIz", "~yEmEzlEr";
    neg_abil_aor_evid: "~yEmEzmIşIm", "~yEmEzmIşsIn", "~yEmEzmIş", "~yEmEzmIşIz", "~yEmEzmIşsInIz", "~yEmEzlErmIş";
    neg_abil_aor_past: "~yEmEzdIm", "~yEmEzdIn", "~yEmEzdI", "~yEmEzdIk", "~yEmEzdInIz", "~yEmEzlErdI";
    neg_abil_aor_cond: "~yEmEzsEm", "~yEmEzsEn", "~yEmEzsE", "~yEmEzsEk", "~yEmEzsEnIz", "~yEmEzlErsE";
    neg_abil_prog__: "~yEmIyorum", "~yEmIyorsun", "~yEmIyor", "~yEmIyoruz", "~yEmIyorsunuz", "~yEmIyorlar";
    neg_abil_prog_evid: "~yEmIyormuşum", "~yEmIyormuşsun", "~yEmIyormuş", "~yEmIyormuşuz", "~yEmIyormuşsunuz", "~yEmIyorlarmış";
    neg_abil_prog_past: "~yEmIyordum", "~yEmIyordun", "~yEmIyordu", "~yEmIyorduk", "~yEmIyordunuz", "~yEmIyorlardı";
    neg_abil_prog_cond: "~yEmIyorsam", "~yEmIyorsan", "~yEmIyorsa", "~yEmIyorsak", "~yEmIyorsanız", "~yEmIyorlarsa";
    neg_abil_evid__: "~yEmEmIşIm", "~yEmEmIşsIn", "~yEmEmIş", "~yEmEmIşIz", "~yEmEmIşsInIz", "~yEmEmIşlEr";
    neg_abil_evid_evid: "~yEmEmIşmIşIm", "~yEmEmIşmIşsIn", "~yEmEmIşmIş", "~yEmEmIşmIşIz", "~yEmEmIşmIşsInIz", "~yEmEmIşlErmIş";
    neg_abil_evid_past: "~yEmEmIştIm", "~yEmEmIştIn", "~yEmEmIştI", "~yEmEmIştIk", "~yEmEmIştInIz", "~yEmEmIşlErdI";
    neg_abil_evid_cond: "~yEmEmIşsEm", "~yEmEmIşsEn", "~yEmEmIşsE", "~yEmEmIşsEk", "~yEmEmIşsEnIz", "~yEmEmIşlErsE";
    neg_abil_fut__: "~yEmEyEcEğIm", "~yEmEyEcEksIn", "~yEmEyEcEk", "~yEmEyEcEğIz", "~yEmEyEcEksInIz", "~yEmEyEcEklEr";
    neg_abil_fut_evid: "~yEmEyEcEkmIşIm", "~yEmEyEcEkmIşsIn", "~yEmEyEcEkmIş", "~yEmEyEcEkmIşIz", "~yEmEyEcEkmIşsInIz", "~yEmEyEcEklErmIş";
    neg_abil_fut_past: "~yEmEyEcEktIm", "~yEmEyEcEktIn", "~yEmEyEcEktI", "~yEmEyEcEktIk", "~yEmEyEcEktInIz", "~yEmEyEcEklErdI";
    neg_abil_fut_cond: "~yEmEyEcEksEm", "~yEmEyEcEksEn", "~yEmEyEcEksE", "~yEmEyEcEksEk", "~yEmEyEcEksEnIz", "~yEmEyEcEklErsE";
    neg_abil_cont__: "~yEmEmEktEyIm", "~yEmEmEktEsIn", "~yEmEmEktE", "~yEmEmEktEyIz", "~yEmEmEktEsInIz", "~yEmEmEktElEr";
    neg_abil_cont_evid: "~yEmEmEktEymIşIm", "~yEmEmEktEymIşsIn", "~yEmEmEktEymIş", "~yEmEmEktEymIşIz", "~yEmEmEktEymIşsInIz", "~yEmEmEktElErmIş";
    neg_abil_cont_past: "~yEmEmEktEydIm", "~yEmEmEktEydIn", "~yEmEmEktEydI", "~yEmEmEktEydIk", "~yEmEmEktEydInIz", "~yEmEmEktElErdI";
    neg_abil_cont_cond: "~yEmEmEktEysEm", "~yEmEmEktEysEn", "~yEmEmEktEysE", "~yEmEmEktEysEk", "~yEmEmEktEysEnIz", "~yEmEmEktElErsE";
    neg_abil_necess__: "~yEmEmElIyIm", "~yEmEmElIsIn", "~yEmEmElI", "~yEmEmElIyIz", "~yEmEmElIsInIz", "~yEmEmElIlEr";
    neg_abil_necess_evid: "~yEmEmElIymIşIm", "~yEmEmElIymIşsIn", "~yEmEmElIymIş", "~yEmEmElIymIşIz", "~yEmEmElIymIşsInIz", "~yEmEmElIlErmIş";
    neg_abil_necess_past: "~yEmEmElIydIm", "~yEmEmElIydIn", "~yEmEmElIydI", "~yEmEmElIydIk", "~yEmEmElIydInIz", "~yEmEmElIlErdI";
    neg_abil_necess_cond: "~yEmEmElIysEm", "~yEmEmElIysEn", "~yEmEmElIysE", "~yEmEmElIysEk", "~yEmEmElIysEnIz", "~yEmEmElIlErsE";
    past_pos_plain__: "~dIm", _, _, _, _, _;
    past_pos_abil__: "~yEbildim", "~yEbildin", "~yEbildi", "~yEbildik", "~yEbildiniz", "~yEbildiler";
    past_pos_abil_past: "~yEbildiydim", "~yEbildiydin", "~yEbildiydi", "~yEbildiydik", "~yEbildiydiniz", "~yEbildilerdi";
    past_pos_abil_cond: "~yEbildiysem", "~yEbildiysen", "~yEbildiyse", "~yEbildiysek", "~yEbildiyseniz", "~yEbildilerse";
    past_neg_abil__: "~yEmEdIm", "~yEmEdIn", "~yEmEdI", "~yEmEdIk", "~yEmEdInIz", "~yEmEdIlEr";
    past_neg_abil_past: "~yEmEdIydIm", "~yEmEdIydIn", "~yEmEdIydI", "~yEmEdIydIk", "~yEmEdIydInIz", "~yEmEdIlErdI";
    past_neg_abil_cond: "~yEmEdIysEm", "~yEmEdIysEn", "~yEmEdIysE", "~yEmEdIysEk", "~yEmEdIysEnIz", "~yEmEdIlErsE";
    conditional_pos_plain__: "~sEm", "~sEn", "~sE", "~sEk", "~sEnIz", "~sElEr";
    conditional_pos_plain_evid: "~sEymIşIm", "~sEymIşsIn", "~sEymIş", "~sEymIşIz", "~sEymIşsInIz", "~sElErmIş";
    conditional_pos_plain_past: "~sEydIm", "~sEydIn", "~sEydI", "~sEydIk", "~sEydInIz", "~sElErdI";
    conditional_pos_abil__: "~yEbilsem", "~yEbilsen", "~yEbilse", "~yEbilsek", "~yEbilseniz", "~yEbilseler";
    conditional_pos_abil_evid: "~yEbilseymişim", "~yEbilseymişsin", "~yEbilseymiş", "~yEbilseymişiz", "~yEbilseymişsiniz", "~yEbilselermiş";
    conditional_pos_abil_past: "~yEbilseydim", "~yEbilseydin", "~yEbilseydi", "~yEbilseydik", "~yEbilseydiniz", "~yEbilselerdi";
    conditional_neg_abil__: "~yEmEsEm", "~yEmEsEn", "~yEmEsE", "~yEmEsEk", "~yEmEsEnIz", "~yEmEsElEr";
    conditional_neg_abil_evid: "~yEmEsEymIşIm", "~yEmEsEymIşsIn", "~yEmEsEymIş", "~yEmEsEymIşIz", "~yEmEsEymIşsInIz", "~yEmEsElErmIş";
    conditional_neg_abil_past: "~yEmEsEydIm", "~yEmEsEydIn", "~yEmEsEydI", "~yEmEsEydIk", "~yEmEsEydInIz", "~yEmEsElErdI";
    optative_pos_plain__: "~yEyIm", "~yEsIn", "~yE", "~yElIm", "~yEsInIz", "~yElEr";
    optative_pos_plain_evid: "~yEymIşIm", "~yEymIşsIn", "~yEymIş", "~yEymIşIz", "~yEymIşsInIz", "~yElErmIş";
    optative_pos_plain_past: "~yEydIm", "~yEydIn", "~yEydI", "~yEydIk", "~yEydInIz", "~yElErdI";
    optative_pos_abil__: "~yEbileyim", "~yEbilesin", "~yEbile", "~yEbilelim", "~yEbilesiniz", "~yEbileler";
    optative_pos_abil_evid: "~yEbileymişim", "~yEbileymişsin", "~yEbileymiş", "~yEbileymişiz", "~yEbileymişsiniz", "~yEbilelermiş";
    optative_pos_abil_past: "~yEbileydim", "~yEbileydin", "~yEbileydi", "~yEbileydik", "~yEbileydiniz", "~yEbilelerdi";
    optative_neg_abil__: "~yEmEyEyIm", "~yEmEyEsIn", "~yEmEyE", "~yEmEyElIm", "~yEmEyEsInIz", "~yEmEyElEr";
    optative_neg_abil_evid: "~yEmEyEymIşIm", "~yEmEyEymIşsIn", "~yEmEyEymIş", "~yEmEyEymIşIz", "~yEmEyEymIşsInIz", "~yEmEyElErmIş";
    optative_neg_abil_past: "~yEmEyEydIm", "~yEmEyEydIn", "~yEmEyEydI", "~yEmEyEydIk", "~yEmEyEydInIz", "~yEmEyElErdI";
    imperative_pos_plain: "~", "~sIn", "~yIn", "~sInlEr", _, "~yInIz", "~sEnE", "~sEnIzE";
    imperative_pos_abil: "~yEbil", "~yEbilsin", "~yEbilin", "~yEbilsinler", "~yEbiliniz", _, "~yEbilsene", "~yEbilsenize";
    imperative_neg_plain: _, _, _, _, _, "~mEyInIz", _, _;
    imperative_neg_abil: "~yEmE", "~yEmEsIn", "~yEmEyIn", "~yEmEsInlEr", "~yEmEyInIz", _, "~yEmEsEnE", "~yEmEsEnIzE";
    infinitive_pos: "~mak", "~yEbilmek";
    infinitive_neg: _, "~yEmEmEk";
    verbal_noun_pos: "~mE", "~yEbilme";
    verbal_noun_neg: _, "~yEmEmE";
    participle_pos_plain: "~yEn", _, _;
    participle_pos_abil: "~yEbilen", _, _;
    participle_neg_plain: _, "~mEmIş", "~mEyEcEk";
    participle_neg_abil: "~yEmEyEn", _, _;
    relative_pos_plain_acak: _, _, _, _, _, "~yEcEklErI";
    relative_pos_abil_dik: "~yEbildiğim", "~yEbildiğin", "~yEbildiği", "~yEbildiğimiz", "~yEbildiğiniz", "~yEbildikleri";
    relative_pos_abil_acak: _, "~yEbileceğin", "~yEbileceği", "~yEbileceğimiz", "~yEbileceğiniz", "~yEbilecekleri";
    relative_neg_plain_acak: "~mEyEcEğIm", _, _, _, _, _;
    relative_neg_abil_dik: "~yEmEdIğIm", "~yEmEdIğIn", "~yEmEdIğI", "~yEmEdIğImIz", "~yEmEdIğInIz", "~yEmEdIklErI";
    relative_neg_abil_acak: _, "~yEmEyEcEğIn", "~yEmEyEcEğI", "~yEmEyEcEğImIz", "~yEmEyEcEğInIz", "~yEmEyEcEklErI";
    converb_pos_plain: "~rken", _, _, "~yElI", "~yEsIyE", _, "~yErEk", "~ya ~ya", "~rcEsInE";
    converb_pos_abil: "~yEbilirken", "~yEbilince", "~yEbilip", "~yEbileli", "~yEbilesiye", "~yEbildikçe", "~yEbilerek", "~yabile ~yabile", _;
    converb_neg_abil: "~yEmEzken", "~yEmEyIncE", "~yEmEyIp", "~yEmEyElI", "~yEmEyEsIyE", "~yEmEdIkçE", "~yEmEyErEk", _, "~yEmEzcEsInE";
    ),
    // Filmçekmek: 17 words (fotoğraf çekmek, otuz bir çekmek, zartayı çekmek, mortoyu çekmek, cızlamı çekmek, cartayı çekmek). Like Açmak, except in 3 cells: infinitive_pos plain, converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("film çekmek", 17, "~mek", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE çEkmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE çEkEmEyE- like ayırmak";
    infinitive_pos: "~mek", _;
    converb_pos_plain: _, _, _, _, _, _, _, "~E çEkE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile çEkEbile", _;
    ),
    // Koynunaalmak: 16 words (almak, nefes almak, duş almak, satın almak, farkına varmak, koynuna almak). Like Varolmak, except in 3 cells: converb_pos_plain doubled, converb_pos_abil ken, converb_pos_abil doubled.
    conjugation!("koynuna almak", 16, "~mak", like Varolmak;
    stem converb_neg_plain_doubled: "~mEyE almEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE alEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E alE", _;
    converb_pos_abil: "~Ebilırken", _, _, _, _, _, _, "~Ebile alEbile", _;
    ),
    // Yürümek: 16 words (yürümek, üşümek, ekşimek, büyümek, çürümek, eskimek). Like Taşımak, except in 3 cells: infinitive_pos plain, converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("yürümek", 16, "~mek", like Taşımak;
    stem converb_neg_plain_doubled: "~meye ~meye- like ayırmak";
    stem converb_neg_abil_doubled: "~yemeye ~yemeye- like ayırmak";
    infinitive_pos: "~mek", _;
    converb_pos_plain: _, _, _, _, _, _, _, "~ye ~ye", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~yebile ~yebile", _;
    ),
    // Aşkyapmak: 15 words (hata yapmak, makyaj yapmak, seks yapmak, dedikodu yapmak, ağda yapmak, sugeçirmez yapmak). Like Açmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("aşk yapmak", 15, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE yapmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE yapEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E yapE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile yapEbile", _;
    ),
    // Tekmeatmak: 12 words (havlu atmak, hava atmak, aşık atmak, kazık atmak, göz atmak, tekme atmak). Like Açmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("tekme atmak", 12, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE atmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE atEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E atE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile atEbile", _;
    ),
    // Imanagelmek: 12 words (üstesinden gelmek, dize gelmek, manasına gelmek, anlamına gelmek, taklaya gelmek, kendine gelmek). Like Vermek, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("imana gelmek", 12, "~mek", like Vermek;
    stem converb_neg_plain_doubled: "~mEyE gElmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE gElEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E gElE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile gElEbile", _;
    ),
    // Camaçıkmak: 12 words (sahip çıkmak, kat çıkmak, karşı çıkmak, dağa çıkmak, eve çıkmak, cama çıkmak). Like Açmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("cama çıkmak", 12, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE çıkmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE çıkEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E çıkE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile çıkEbile", _;
    ),
    // Ilişkiyegirmek: 10 words (bahse girmek, transa girmek, vizyona girmek, gösterime girmek, araya girmek, yürürlüğe girmek). Like Silmek, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("ilişkiye girmek", 10, "~mek", like Silmek;
    stem converb_neg_plain_doubled: "~mEyE gIrmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE gIrEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E gIrE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile gIrEbile", _;
    ),
    // Baştançıkarmak: 7 words (baştan çıkarmak, tepesine çıkarmak, foyasını ortaya çıkarmak, ortaya çıkarmak, göklere çıkarmak, tadını çıkarmak). Like Ayırmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("baştan çıkarmak", 7, "~mak", like Ayırmak;
    stem converb_neg_plain_doubled: "~mEyE çıkErmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE çıkErEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E çıkErE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile çıkErEbile", _;
    ),
    // Almak: 7 words (azmak, kalmak, varmak, sanmak, yammak, çırpmak). Like Koynunaalmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("almak", 7, "~mak", like Koynunaalmak;
    stem converb_neg_plain_doubled: "~maya ~maya- like ayırmak";
    stem converb_neg_abil_doubled: "~amaya ~amaya- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~a ~a", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~abile ~abile", _;
    ),
    // Yemek: 6 words (demek, yemek, dayak yemek, kazık yemek, rüşvet yemek, aklını peynir ekmekle yemek).
    conjugation!("yemek", 6, "~emek";
    stem lemma: "~- like yemek";
    stem converb_neg_plain_doubled: "~Emeye yEmeye- like ayırmak";
    stem converb_neg_abil_doubled: "~iyemeye yiyemeye- like ayırmak";
    imperative_pos_abil: _, _, _, _, _, derived, _, _;
    imperative_neg_abil: _, _, _, _, _, derived, _, _;
    participle_pos_plain: _, derived, derived;
    participle_pos_abil: _, derived, derived;
    participle_neg_abil: _, derived, derived;
    relative_pos_plain_acak: derived, _, _, _, _, _;
    relative_pos_abil_acak: derived, _, _, _, _, _;
    relative_neg_abil_acak: derived, _, _, _, _, _;
    ),
    // Vurmak: 5 words (olmak, vurmak, durmak, bulmak, bulundurmak). Like Almak, except in 1 cell: converb_pos_abil ken.
    conjugation!("vurmak", 5, "~mak", like Almak;
    converb_pos_abil: "~Ebilurken", _, _, _, _, _, _, _, _;
    ),
    // Yolaçmak: 5 words (ateş açmak, yol açmak, iç açmak, kanat açmak, çiçek açmak). Like Açmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("yol açmak", 5, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE açmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE açEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E açE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile açEbile", _;
    ),
    // Maytapgeçmek: 4 words (taşak geçmek, transa geçmek, maytap geçmek, tarihe geçmek). Like Filmçekmek, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("maytap geçmek", 4, "~mek", like Filmçekmek;
    stem converb_neg_plain_doubled: "~mEyE gEçmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE gEçEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E gEçE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile gEçEbile", _;
    ),
    // Kafatutmak: 4 words (kafa tutmak, oruç tutmak, kulak tutmak, gözü tutmak). Like Açmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("kafa tutmak", 4, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE tutmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE tutEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E tutE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile tutEbile", _;
    ),
    // Bükmek: 3 words (bükmek, ölçmek, gömmek). Like Vermek, except in 26 cells: pos_plain_aor__ _1sg, pos_plain_aor__ _2sg, pos_plain_aor__ _3sg, pos_plain_aor__ _2pl, pos_plain_aor__ _3pl, pos_plain_aor_evid _1sg, ….
    conjugation!("bükmek", 3, "~mek", like Vermek;
    pos_plain_aor__: "~irIm", "~irsIn", "~ir", _, "~irsInIz", "~irlEr";
    pos_plain_aor_evid: "~irmIşIm", "~irmIşsIn", "~irmIş", "~irmIşIz", "~irmIşsInIz", "~irlErmIş";
    pos_plain_aor_past: "~irdIm", "~irdIn", "~irdI", "~irdIk", "~irdInIz", "~irlErdI";
    pos_plain_aor_cond: "~irsEm", "~irsEn", "~irsE", "~irsEk", "~irsEnIz", "~irlErsE";
    past_pos_plain__: "~tIm", _, _, _, _, _;
    converb_pos_plain: "~irken", _, _, _, _, _, _, _, "~ircEsInE";
    ),
    // Fındıkkırmak: 3 words (fındık kırmak, iki lafın belini kırmak, pot kırmak). Like Açmak, except in 3 cells: past_pos_plain__ _1sg, converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("fındık kırmak", 3, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE kırmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE kırEmEyE- like ayırmak";
    past_pos_plain__: "~dIm", _, _, _, _, _;
    converb_pos_plain: _, _, _, _, _, _, _, "~E kırE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile kırEbile", _;
    ),
    // Geçkalmak: 3 words (geç kalmak, mutabık kalmak, ramak kalmak). Like Koynunaalmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("geç kalmak", 3, "~mak", like Koynunaalmak;
    stem converb_neg_plain_doubled: "~mEyE kAlmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE kAlEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E kAlE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile kAlEbile", _;
    ),
    // Serbestbırakmak: 3 words (silah bırakmak, tadında bırakmak, serbest bırakmak). Like Ayırmak, except in 3 cells: past_pos_plain__ _1sg, converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("serbest bırakmak", 3, "~mak", like Ayırmak;
    stem converb_neg_plain_doubled: "~mEyE bırEkmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE bırEkEmEyE- like ayırmak";
    past_pos_plain__: "~tIm", _, _, _, _, _;
    converb_pos_plain: _, _, _, _, _, _, _, "~E bırEkE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile bırEkEbile", _;
    ),
    // Görmek: 3 words (görmek, mazur görmek, görüm görmek). Like Varolmak, except in 4 cells: infinitive_pos plain, converb_pos_plain doubled, converb_pos_abil ken, converb_pos_abil doubled.
    conjugation!("görmek", 3, "~mek", like Varolmak;
    stem converb_neg_plain_doubled: "~mEyE görmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE görEmEyE- like ayırmak";
    infinitive_pos: "~mek", _;
    converb_pos_plain: _, _, _, _, _, _, _, "~E görE", _;
    converb_pos_abil: "~Ebilürken", _, _, _, _, _, _, "~Ebile görEbile", _;
    ),
    // Götügitmek: 3 words (götü gitmek, hoşuna gitmek, geberip gitmek). Like Takipetmek, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("götü gitmek", 3, "~tmek", like Takipetmek;
    stem converb_neg_plain_doubled: "~tmeye gitmeye- like ayırmak";
    stem converb_neg_abil_doubled: "~demeye gidemeye- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~de gide", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~debile gidebile", _;
    ),
    // Büyükoynamak: 3 words (büyük oynamak, bahis oynamak, kumar oynamak). Like Patlamak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("büyük oynamak", 3, "~amak", like Patlamak;
    stem converb_neg_plain_doubled: "~amaya Aynamaya- like ayırmak";
    stem converb_neg_abil_doubled: "~ayamaya Aynayamaya- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~aya Aynaya", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~ayabile Aynayabile", _;
    ),
    // Layığınıbulmak: 3 words (teselli bulmak, bokunda boncuk bulmak, layığını bulmak). Like Varolmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("layığını bulmak", 3, "~mak", like Varolmak;
    stem converb_neg_plain_doubled: "~mEyE bulmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE bulEmEyE- like ayırmak";
    converb_pos_plain: _, _, _, _, _, _, _, "~E bulE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile bulEbile", _;
    ),
    // Seskasmak: 3 words (ses kasmak, amuda kalkmak, duyar kasmak). Like Açmak, except in 27 cells: pos_abil_aor__ _1sg, pos_abil_aor__ _2sg, pos_abil_aor__ _3sg, pos_abil_aor__ _1pl, pos_abil_aor__ _2pl, pos_abil_aor__ _3pl, ….
    conjugation!("ses kasmak", 3, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE kAsmEyE- like ayırmak";
    stem converb_neg_abil_doubled: "~EmEyE kAsEmEyE- like ayırmak";
    pos_abil_aor__: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_evid: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_past: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_cond: derived, derived, derived, derived, derived, derived;
    converb_pos_plain: _, _, _, _, _, _, _, "~E kAsE", _;
    converb_pos_abil: "~EbilArken", _, _, _, _, _, _, "~Ebile kAsEbile", _;
    ),
];
