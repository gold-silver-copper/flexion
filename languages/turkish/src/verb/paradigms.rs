//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~mek`, `vermek`: the stem and `mek`).
//! Alternatives are separated by `|`, the usual one first. A capital (`E`, `I`)
//! is a class letter: a harmony law in `laws.rs` writes it, by the
//! letter before it.

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the conjugation it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem converb_neg_plain_doubled: \"~meye ~meye- like vermek\"`: the slot's cells are `vermek`'s table's patterns
/// with `~meye ~meye` for their `~`) (see `stems.rs`), then a row of patterns per line
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
    /// Like `vermek`: 476 words.
    Vermek,
    /// Like `nefes aldırmak`: 457 words.
    Nefesaldırmak,
    /// Like `var olmak`: 353 words.
    Varolmak,
    /// Like `patlamak`: 267 words.
    Patlamak,
    /// Like `takip etmek`: 186 words.
    Takipetmek,
    /// Like `beslemek`: 152 words.
    Beslemek,
    /// Like `açmak`: 123 words.
    Açmak,
    /// Like `çekmek`: 104 words.
    Çekmek,
    /// Like `gitmek`: 53 words.
    Gitmek,
    /// Like `ele vermek`: 33 words.
    Elevermek,
    /// Like `silmek`: 15 words.
    Silmek,
    /// Like `aşk yapmak`: 15 words.
    Aşkyapmak,
    /// Like `yürümek`: 13 words.
    Yürümek,
    /// Like `tekme atmak`: 12 words.
    Tekmeatmak,
    /// Like `imana gelmek`: 12 words.
    Imanagelmek,
    /// Like `cama çıkmak`: 12 words.
    Camaçıkmak,
    /// Like `taşımak`: 11 words.
    Taşımak,
    /// Like `ilişkiye girmek`: 10 words.
    Ilişkiyegirmek,
    /// Like `ilerletmek`: 7 words.
    Ilerletmek,
    /// Like `yemek`: 6 words.
    Yemek,
    /// Like `film çekmek`: 5 words.
    Filmçekmek,
    /// Like `vurmak`: 5 words.
    Vurmak,
    /// Like `yol açmak`: 5 words.
    Yolaçmak,
    /// Like `salmak`: 4 words.
    Salmak,
    /// Like `maytap geçmek`: 4 words.
    Maytapgeçmek,
    /// Like `kafa tutmak`: 4 words.
    Kafatutmak,
    /// Like `gömmek`: 3 words.
    Gömmek,
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
    pub const ALL: [Conjugation; 34] = [
        Conjugation::Vermek,
        Conjugation::Nefesaldırmak,
        Conjugation::Varolmak,
        Conjugation::Patlamak,
        Conjugation::Takipetmek,
        Conjugation::Beslemek,
        Conjugation::Açmak,
        Conjugation::Çekmek,
        Conjugation::Gitmek,
        Conjugation::Elevermek,
        Conjugation::Silmek,
        Conjugation::Aşkyapmak,
        Conjugation::Yürümek,
        Conjugation::Tekmeatmak,
        Conjugation::Imanagelmek,
        Conjugation::Camaçıkmak,
        Conjugation::Taşımak,
        Conjugation::Ilişkiyegirmek,
        Conjugation::Ilerletmek,
        Conjugation::Yemek,
        Conjugation::Filmçekmek,
        Conjugation::Vurmak,
        Conjugation::Yolaçmak,
        Conjugation::Salmak,
        Conjugation::Maytapgeçmek,
        Conjugation::Kafatutmak,
        Conjugation::Gömmek,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 34] = [
    // Vermek: 476 words (çizmek, direnmek, tüketmek, bilmek, vermek, gelmek).
    conjugation!("vermek", 476, "~mek";
    stem converb_neg_plain_doubled: "~meye ~meye- like vermek";
    stem converb_neg_abil_doubled: "~emeye ~emeye- like vermek";
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
    pos_plain_cont__: "~mektEyIm", "~mektEsIn", "~mektE", "~mektEyIz", "~mektEsInIz", "~mektElEr";
    pos_plain_cont_evid: "~mektEymIşIm", "~mektEymIşsIn", "~mektEymIş", "~mektEymIşIz", "~mektEymIşsInIz", "~mektElErmIş";
    pos_plain_cont_past: "~mektEydIm", "~mektEydIn", "~mektEydI", "~mektEydIk", "~mektEydInIz", "~mektElErdI";
    pos_plain_cont_cond: "~mektEysEm", "~mektEysEn", "~mektEysE", "~mektEysEk", "~mektEysEnIz", "~mektElErsE";
    pos_plain_necess__: _, _, _, _, _, "~mElIlEr";
    pos_plain_necess_evid: _, _, _, _, _, "~mElIlErmIş";
    pos_plain_necess_past: _, _, _, _, _, "~mElIlErdI";
    pos_plain_necess_cond: "~mElIysEm", "~mElIysEn", "~mElIysE", "~mElIysEk", "~mElIysEnIz", "~mElIlErsE";
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
    neg_plain_aor__: _, _, _, _, _, "~mEzlEr";
    neg_plain_aor_evid: _, _, _, _, _, "~mEzlErmIş";
    neg_plain_aor_past: _, _, _, _, _, "~mEzlErdI";
    neg_plain_aor_cond: "~mEzsEm", "~mEzsEn", "~mEzsE", "~mEzsEk", "~mEzsEnIz", "~mEzlErsE";
    neg_plain_prog__: "~mIyorum", "~mIyorsun", "~mIyor", "~mIyoruz", "~mIyorsunuz", "~mIyorlar";
    neg_plain_prog_evid: "~mIyormuşum", "~mIyormuşsun", "~mIyormuş", "~mIyormuşuz", "~mIyormuşsunuz", "~mIyorlarmış";
    neg_plain_prog_past: "~mIyordum", "~mIyordun", "~mIyordu", "~mIyorduk", "~mIyordunuz", "~mIyorlardı";
    neg_plain_prog_cond: "~mIyorsam", "~mIyorsan", "~mIyorsa", "~mIyorsak", "~mIyorsanız", "~mIyorlarsa";
    neg_plain_evid__: _, _, _, _, _, "~mEmIşlEr";
    neg_plain_evid_evid: _, _, _, _, _, "~mEmIşlErmIş";
    neg_plain_evid_past: _, _, _, _, _, "~mEmIşlErdI";
    neg_plain_evid_cond: "~mEmIşsEm", "~mEmIşsEn", "~mEmIşsE", "~mEmIşsEk", "~mEmIşsEnIz", "~mEmIşlErsE";
    neg_plain_fut__: "~mEyEcEğIm", "~mEyEcEksIn", "~mEyEcEk", "~mEyEcEğIz", "~mEyEcEksInIz", "~mEyEcEklEr";
    neg_plain_fut_evid: "~mEyEcEkmIşIm", "~mEyEcEkmIşsIn", "~mEyEcEkmIş", "~mEyEcEkmIşIz", "~mEyEcEkmIşsInIz", "~mEyEcEklErmIş";
    neg_plain_fut_past: "~mEyEcEktIm", "~mEyEcEktIn", "~mEyEcEktI", "~mEyEcEktIk", "~mEyEcEktInIz", "~mEyEcEklErdI";
    neg_plain_fut_cond: "~mEyEcEksEm", "~mEyEcEksEn", "~mEyEcEksE", "~mEyEcEksEk", "~mEyEcEksEnIz", "~mEyEcEklErsE";
    neg_plain_cont__: "~mEmEktEyIm", "~mEmEktEsIn", "~mEmEktE", "~mEmEktEyIz", "~mEmEktEsInIz", "~mEmEktElEr";
    neg_plain_cont_evid: "~mEmEktEymIşIm", "~mEmEktEymIşsIn", "~mEmEktEymIş", "~mEmEktEymIşIz", "~mEmEktEymIşsInIz", "~mEmEktElErmIş";
    neg_plain_cont_past: "~mEmEktEydIm", "~mEmEktEydIn", "~mEmEktEydI", "~mEmEktEydIk", "~mEmEktEydInIz", "~mEmEktElErdI";
    neg_plain_cont_cond: "~mEmEktEysEm", "~mEmEktEysEn", "~mEmEktEysE", "~mEmEktEysEk", "~mEmEktEysEnIz", "~mEmEktElErsE";
    neg_plain_necess__: "~mEmElIyIm", "~mEmElIsIn", "~mEmElI", "~mEmElIyIz", "~mEmElIsInIz", "~mEmElIlEr";
    neg_plain_necess_evid: "~mEmElIymIşIm", "~mEmElIymIşsIn", "~mEmElIymIş", "~mEmElIymIşIz", "~mEmElIymIşsInIz", "~mEmElIlErmIş";
    neg_plain_necess_past: "~mEmElIydIm", "~mEmElIydIn", "~mEmElIydI", "~mEmElIydIk", "~mEmElIydInIz", "~mEmElIlErdI";
    neg_plain_necess_cond: "~mEmElIysEm", "~mEmElIysEn", "~mEmElIysE", "~mEmElIysEk", "~mEmElIysEnIz", "~mEmElIlErsE";
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
    past_neg_plain__: _, _, _, _, _, "~mEdIlEr";
    past_neg_plain_past: _, _, _, _, _, "~mEdIlErdI";
    past_neg_plain_cond: "~mEdIysEm", "~mEdIysEn", "~mEdIysE", "~mEdIysEk", "~mEdIysEnIz", "~mEdIlErsE";
    past_neg_abil__: "~EmEdIm", "~EmEdIn", "~EmEdI", "~EmEdIk", "~EmEdInIz", "~EmEdIlEr";
    past_neg_abil_past: "~EmEdIydIm", "~EmEdIydIn", "~EmEdIydI", "~EmEdIydIk", "~EmEdIydInIz", "~EmEdIlErdI";
    past_neg_abil_cond: "~EmEdIysEm", "~EmEdIysEn", "~EmEdIysE", "~EmEdIysEk", "~EmEdIysEnIz", "~EmEdIlErsE";
    conditional_pos_plain__: "~sEm", "~sEn", "~sE", "~sEk", "~sEnIz", "~sElEr";
    conditional_pos_plain_evid: "~sEymIşIm", "~sEymIşsIn", "~sEymIş", "~sEymIşIz", "~sEymIşsInIz", "~sElErmIş";
    conditional_pos_plain_past: "~sEydIm", "~sEydIn", "~sEydI", "~sEydIk", "~sEydInIz", "~sElErdI";
    conditional_pos_abil__: "~Ebilsem", "~Ebilsen", "~Ebilse", "~Ebilsek", "~Ebilseniz", "~Ebilseler";
    conditional_pos_abil_evid: "~Ebilseymişim", "~Ebilseymişsin", "~Ebilseymiş", "~Ebilseymişiz", "~Ebilseymişsiniz", "~Ebilselermiş";
    conditional_pos_abil_past: "~Ebilseydim", "~Ebilseydin", "~Ebilseydi", "~Ebilseydik", "~Ebilseydiniz", "~Ebilselerdi";
    conditional_neg_plain__: "~mEsEm", "~mEsEn", "~mEsE", "~mEsEk", "~mEsEnIz", "~mEsElEr";
    conditional_neg_plain_evid: "~mEsEymIşIm", "~mEsEymIşsIn", "~mEsEymIş", "~mEsEymIşIz", "~mEsEymIşsInIz", "~mEsElErmIş";
    conditional_neg_plain_past: "~mEsEydIm", "~mEsEydIn", "~mEsEydI", "~mEsEydIk", "~mEsEydInIz", "~mEsElErdI";
    conditional_neg_abil__: "~EmEsEm", "~EmEsEn", "~EmEsE", "~EmEsEk", "~EmEsEnIz", "~EmEsElEr";
    conditional_neg_abil_evid: "~EmEsEymIşIm", "~EmEsEymIşsIn", "~EmEsEymIş", "~EmEsEymIşIz", "~EmEsEymIşsInIz", "~EmEsElErmIş";
    conditional_neg_abil_past: "~EmEsEydIm", "~EmEsEydIn", "~EmEsEydI", "~EmEsEydIk", "~EmEsEydInIz", "~EmEsElErdI";
    optative_pos_plain__: "~EyIm", "~EsIn", "~E", "~ElIm", "~EsInIz", "~ElEr";
    optative_pos_plain_evid: "~EymIşIm", "~EymIşsIn", "~EymIş", "~EymIşIz", "~EymIşsInIz", "~ElErmIş";
    optative_pos_plain_past: "~EydIm", "~EydIn", "~EydI", "~EydIk", "~EydInIz", "~ElErdI";
    optative_pos_abil__: "~Ebileyim", "~Ebilesin", "~Ebile", "~Ebilelim", "~Ebilesiniz", "~Ebileler";
    optative_pos_abil_evid: "~Ebileymişim", "~Ebileymişsin", "~Ebileymiş", "~Ebileymişiz", "~Ebileymişsiniz", "~Ebilelermiş";
    optative_pos_abil_past: "~Ebileydim", "~Ebileydin", "~Ebileydi", "~Ebileydik", "~Ebileydiniz", "~Ebilelerdi";
    optative_neg_plain__: "~mEyEyIm", "~mEyEsIn", "~mEyE", "~mEyElIm", "~mEyEsInIz", "~mEyElEr";
    optative_neg_plain_evid: "~mEyEymIşIm", "~mEyEymIşsIn", "~mEyEymIş", "~mEyEymIşIz", "~mEyEymIşsInIz", "~mEyElErmIş";
    optative_neg_plain_past: "~mEyEydIm", "~mEyEydIn", "~mEyEydI", "~mEyEydIk", "~mEyEydInIz", "~mEyElErdI";
    optative_neg_abil__: "~EmEyEyIm", "~EmEyEsIn", "~EmEyE", "~EmEyElIm", "~EmEyEsInIz", "~EmEyElEr";
    optative_neg_abil_evid: "~EmEyEymIşIm", "~EmEyEymIşsIn", "~EmEyEymIş", "~EmEyEymIşIz", "~EmEyEymIşsInIz", "~EmEyElErmIş";
    optative_neg_abil_past: "~EmEyEydIm", "~EmEyEydIn", "~EmEyEydI", "~EmEyEydIk", "~EmEyEydInIz", "~EmEyElErdI";
    imperative_pos_plain: "~", "~sIn", "~In", "~sInlEr", _, "~InIz", "~sEnE", "~sEnIzE";
    imperative_pos_abil: "~Ebil", "~Ebilsin", "~Ebilin", "~Ebilsinler", "~Ebiliniz", _, "~Ebilsene", "~Ebilsenize";
    imperative_neg_plain: _, _, _, "~mEsInlEr", _, "~mEyInIz", "~mEsEnE", "~mEsEnIzE";
    imperative_neg_abil: "~EmE", "~EmEsIn", "~EmEyIn", "~EmEsInlEr", "~EmEyInIz", _, "~EmEsEnE", "~EmEsEnIzE";
    infinitive_pos: "~mek", "~Ebilmek";
    infinitive_neg: "~mEmEk", "~EmEmEk";
    verbal_noun_pos: "~mE", "~Ebilme";
    verbal_noun_neg: "~mEmE", "~EmEmE";
    participle_pos_plain: "~En", _, _;
    participle_pos_abil: "~Ebilen", _, _;
    participle_neg_plain: "~mEyEn", "~mEmIş", _;
    participle_neg_abil: "~EmEyEn", _, _;
    relative_pos_plain_acak: _, _, _, _, _, "~EcEklErI";
    relative_pos_abil_dik: "~Ebildiğim", "~Ebildiğin", "~Ebildiği", "~Ebildiğimiz", "~Ebildiğiniz", "~Ebildikleri";
    relative_pos_abil_acak: _, "~Ebileceğin", "~Ebileceği", "~Ebileceğimiz", "~Ebileceğiniz", "~Ebilecekleri";
    relative_neg_plain_dik: _, _, _, _, _, "~mEdIklErI";
    relative_neg_plain_acak: _, "~mEyEcEğIn", "~mEyEcEğI", "~mEyEcEğImIz", "~mEyEcEğInIz", "~mEyEcEklErI";
    relative_neg_abil_dik: "~EmEdIğIm", "~EmEdIğIn", "~EmEdIğI", "~EmEdIğImIz", "~EmEdIğInIz", "~EmEdIklErI";
    relative_neg_abil_acak: _, "~EmEyEcEğIn", "~EmEyEcEğI", "~EmEyEcEğImIz", "~EmEyEcEğInIz", "~EmEyEcEklErI";
    converb_pos_plain: "~Irken", _, _, "~ElI", "~EsIyE", _, "~ErEk", "~e ~e", "~IrcEsInE";
    converb_pos_abil: "~EbilIrken", "~Ebilince", "~Ebilip", "~Ebileli", "~Ebilesiye", "~Ebildikçe", "~Ebilerek", "~ebile ~ebile", _;
    converb_neg_plain: _, "~mEyIncE", _, "~mEyElI", "~mEyEsIyE", "~mEdIkçE", "~mEyErEk", _, "~mEzcEsInE";
    converb_neg_abil: "~EmEzken", "~EmEyIncE", "~EmEyIp", "~EmEyElI", "~EmEyEsIyE", "~EmEdIkçE", "~EmEyErEk", _, "~EmEzcEsInE";
    ),
    // Nefesaldırmak: 457 words (kazanmak, cezalandırmak, boşalmak, heyecanlandırmak, batırmak, kullanmak). Like Vermek, except in 27 cells: pos_plain_cont__ _1sg, pos_plain_cont__ _2sg, pos_plain_cont__ _3sg, pos_plain_cont__ _1pl, pos_plain_cont__ _2pl, pos_plain_cont__ _3pl, ….
    conjugation!("nefes aldırmak", 457, "~mak", like Vermek;
    stem converb_neg_plain_doubled: "~maya ~maya- like vermek";
    stem converb_neg_abil_doubled: "~amaya ~amaya- like vermek";
    pos_plain_cont__: "~maktEyIm", "~maktEsIn", "~maktE", "~maktEyIz", "~maktEsInIz", "~maktElEr";
    pos_plain_cont_evid: "~maktEymIşIm", "~maktEymIşsIn", "~maktEymIş", "~maktEymIşIz", "~maktEymIşsInIz", "~maktElErmIş";
    pos_plain_cont_past: "~maktEydIm", "~maktEydIn", "~maktEydI", "~maktEydIk", "~maktEydInIz", "~maktElErdI";
    pos_plain_cont_cond: "~maktEysEm", "~maktEysEn", "~maktEysE", "~maktEysEk", "~maktEysEnIz", "~maktElErsE";
    infinitive_pos: "~mak", _;
    converb_pos_plain: _, _, _, _, _, _, _, "~a ~a", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~abile ~abile", _;
    ),
    // Varolmak: 353 words (yapışmak, azaltmak, bırakmak, tartışmak, basmak, abartmak). Like Nefesaldırmak, except in 1 cell: past_pos_plain__ _1sg.
    conjugation!("var olmak", 353, "~mak", like Nefesaldırmak;
    past_pos_plain__: "~tIm", _, _, _, _, _;
    ),
    // Patlamak: 267 words (oynamak, ağlamak, zorlamak, sağlamak, atlamak, kapamak).
    conjugation!("patlamak", 267, "~amak";
    stem converb_neg_plain_doubled: "~amaya ~amaya- like vermek";
    stem converb_neg_abil_doubled: "~ayamaya ~ayamaya- like vermek";
    pos_plain_aor__: "~arIm", "~arsIn", "~ar", _, "~arsInIz", "~arlEr";
    pos_plain_aor_evid: "~armIşIm", "~armIşsIn", "~armIş", "~armIşIz", "~armIşsInIz", "~arlErmIş";
    pos_plain_aor_past: "~ardIm", "~ardIn", "~ardI", "~ardIk", "~ardInIz", "~arlErdI";
    pos_plain_aor_cond: "~arsEm", "~arsEn", "~arsE", "~arsEk", "~arsEnIz", "~arlErsE";
    pos_plain_prog__: "~Iyorum", _, _, _, _, _;
    pos_plain_evid__: _, "~amIşsIn", "~amIş", _, "~amIşsInIz", "~amIşlEr";
    pos_plain_evid_evid: "~amIşmIşIm", "~amIşmIşsIn", "~amIşmIş", "~amIşmIşIz", "~amIşmIşsInIz", "~amIşlErmIş";
    pos_plain_evid_past: "~amIştIm", "~amIştIn", "~amIştI", "~amIştIk", "~amIştInIz", "~amIşlErdI";
    pos_plain_evid_cond: "~amIşsEm", "~amIşsEn", "~amIşsE", "~amIşsEk", "~amIşsEnIz", "~amIşlErsE";
    pos_plain_fut__: "~ayEcEğIm", "~ayEcEksIn", "~ayEcEk", _, "~ayEcEksInIz", "~ayEcEklEr";
    pos_plain_fut_evid: "~ayEcEkmIşIm", "~ayEcEkmIşsIn", "~ayEcEkmIş", "~ayEcEkmIşIz", "~ayEcEkmIşsInIz", "~ayEcEklErmIş";
    pos_plain_fut_past: "~ayEcEktIm", "~ayEcEktIn", "~ayEcEktI", "~ayEcEktIk", "~ayEcEktInIz", "~ayEcEklErdI";
    pos_plain_fut_cond: "~ayEcEksEm", "~ayEcEksEn", "~ayEcEksE", "~ayEcEksEk", "~ayEcEksEnIz", "~ayEcEklErsE";
    pos_plain_cont__: "~amaktEyIm", "~amaktEsIn", "~amaktE", "~amaktEyIz", "~amaktEsInIz", "~amaktElEr";
    pos_plain_cont_evid: "~amaktEymIşIm", "~amaktEymIşsIn", "~amaktEymIş", "~amaktEymIşIz", "~amaktEymIşsInIz", "~amaktElErmIş";
    pos_plain_cont_past: "~amaktEydIm", "~amaktEydIn", "~amaktEydI", "~amaktEydIk", "~amaktEydInIz", "~amaktElErdI";
    pos_plain_cont_cond: "~amaktEysEm", "~amaktEysEn", "~amaktEysE", "~amaktEysEk", "~amaktEysEnIz", "~amaktElErsE";
    pos_plain_necess__: _, _, _, _, _, "~amElIlEr";
    pos_plain_necess_evid: _, _, _, _, _, "~amElIlErmIş";
    pos_plain_necess_past: _, _, _, _, _, "~amElIlErdI";
    pos_plain_necess_cond: "~amElIysEm", "~amElIysEn", "~amElIysE", "~amElIysEk", "~amElIysEnIz", "~amElIlErsE";
    pos_abil_aor__: "~ayEbilirim", "~ayEbilirsin", "~ayEbilir", "~ayEbiliriz", "~ayEbilirsiniz", "~ayEbilirler";
    pos_abil_aor_evid: "~ayEbilirmişim", "~ayEbilirmişsin", "~ayEbilirmiş", "~ayEbilirmişiz", "~ayEbilirmişsiniz", "~ayEbilirlermiş";
    pos_abil_aor_past: "~ayEbilirdim", "~ayEbilirdin", "~ayEbilirdi", "~ayEbilirdik", "~ayEbilirdiniz", "~ayEbilirlerdi";
    pos_abil_aor_cond: "~ayEbilirsem", "~ayEbilirsen", "~ayEbilirse", "~ayEbilirsek", "~ayEbilirseniz", "~ayEbilirlerse";
    pos_abil_prog__: "~ayEbiliyorum", "~ayEbiliyorsun", "~ayEbiliyor", "~ayEbiliyoruz", "~ayEbiliyorsunuz", "~ayEbiliyorlar";
    pos_abil_prog_evid: "~ayEbiliyormuşum", "~ayEbiliyormuşsun", "~ayEbiliyormuş", "~ayEbiliyormuşuz", "~ayEbiliyormuşsunuz", "~ayEbiliyorlarmış";
    pos_abil_prog_past: "~ayEbiliyordum", "~ayEbiliyordun", "~ayEbiliyordu", "~ayEbiliyorduk", "~ayEbiliyordunuz", "~ayEbiliyorlardı";
    pos_abil_prog_cond: "~ayEbiliyorsam", "~ayEbiliyorsan", "~ayEbiliyorsa", "~ayEbiliyorsak", "~ayEbiliyorsanız", "~ayEbiliyorlarsa";
    pos_abil_evid__: "~ayEbilmişim", "~ayEbilmişsin", "~ayEbilmiş", "~ayEbilmişiz", "~ayEbilmişsiniz", "~ayEbilmişler";
    pos_abil_evid_evid: "~ayEbilmişmişim", "~ayEbilmişmişsin", "~ayEbilmişmiş", "~ayEbilmişmişiz", "~ayEbilmişmişsiniz", "~ayEbilmişlermiş";
    pos_abil_evid_past: "~ayEbilmiştim", "~ayEbilmiştin", "~ayEbilmişti", "~ayEbilmiştik", "~ayEbilmiştiniz", "~ayEbilmişlerdi";
    pos_abil_evid_cond: "~ayEbilmişsem", "~ayEbilmişsen", "~ayEbilmişse", "~ayEbilmişsek", "~ayEbilmişseniz", "~ayEbilmişlerse";
    pos_abil_fut__: "~ayEbileceğim", "~ayEbileceksin", "~ayEbilecek", "~ayEbileceğiz", "~ayEbileceksiniz", "~ayEbilecekler";
    pos_abil_fut_evid: "~ayEbilecekmişim", "~ayEbilecekmişsin", "~ayEbilecekmiş", "~ayEbilecekmişiz", "~ayEbilecekmişsiniz", "~ayEbileceklermiş";
    pos_abil_fut_past: "~ayEbilecektim", "~ayEbilecektin", "~ayEbilecekti", "~ayEbilecektik", "~ayEbilecektiniz", "~ayEbileceklerdi";
    pos_abil_fut_cond: "~ayEbileceksem", "~ayEbileceksen", "~ayEbilecekse", "~ayEbileceksek", "~ayEbilecekseniz", "~ayEbileceklerse";
    pos_abil_cont__: "~ayEbilmekteyim", "~ayEbilmektesin", "~ayEbilmekte", "~ayEbilmekteyiz", "~ayEbilmektesiniz", "~ayEbilmekteler";
    pos_abil_cont_evid: "~ayEbilmekteymişim", "~ayEbilmekteymişsin", "~ayEbilmekteymiş", "~ayEbilmekteymişiz", "~ayEbilmekteymişsiniz", "~ayEbilmektelermiş";
    pos_abil_cont_past: "~ayEbilmekteydim", "~ayEbilmekteydin", "~ayEbilmekteydi", "~ayEbilmekteydik", "~ayEbilmekteydiniz", "~ayEbilmektelerdi";
    pos_abil_cont_cond: "~ayEbilmekteysem", "~ayEbilmekteysen", "~ayEbilmekteyse", "~ayEbilmekteysek", "~ayEbilmekteyseniz", "~ayEbilmektelerse";
    pos_abil_necess__: "~ayEbilmeliyim", "~ayEbilmelisin", "~ayEbilmeli", "~ayEbilmeliyiz", "~ayEbilmelisiniz", "~ayEbilmeliler";
    pos_abil_necess_evid: "~ayEbilmeliymişim", "~ayEbilmeliymişsin", "~ayEbilmeliymiş", "~ayEbilmeliymişiz", "~ayEbilmeliymişsiniz", "~ayEbilmelilermiş";
    pos_abil_necess_past: "~ayEbilmeliydim", "~ayEbilmeliydin", "~ayEbilmeliydi", "~ayEbilmeliydik", "~ayEbilmeliydiniz", "~ayEbilmelilerdi";
    pos_abil_necess_cond: "~ayEbilmeliysem", "~ayEbilmeliysen", "~ayEbilmeliyse", "~ayEbilmeliysek", "~ayEbilmeliyseniz", "~ayEbilmelilerse";
    neg_plain_aor__: _, _, _, _, _, "~amEzlEr";
    neg_plain_aor_evid: _, _, _, _, _, "~amEzlErmIş";
    neg_plain_aor_past: _, _, _, _, _, "~amEzlErdI";
    neg_plain_aor_cond: "~amEzsEm", "~amEzsEn", "~amEzsE", "~amEzsEk", "~amEzsEnIz", "~amEzlErsE";
    neg_plain_prog__: "~amIyorum", "~amIyorsun", "~amIyor", "~amIyoruz", "~amIyorsunuz", "~amIyorlar";
    neg_plain_prog_evid: "~amIyormuşum", "~amIyormuşsun", "~amIyormuş", "~amIyormuşuz", "~amIyormuşsunuz", "~amIyorlarmış";
    neg_plain_prog_past: "~amIyordum", "~amIyordun", "~amIyordu", "~amIyorduk", "~amIyordunuz", "~amIyorlardı";
    neg_plain_prog_cond: "~amIyorsam", "~amIyorsan", "~amIyorsa", "~amIyorsak", "~amIyorsanız", "~amIyorlarsa";
    neg_plain_evid__: _, _, _, _, _, "~amEmIşlEr";
    neg_plain_evid_evid: _, _, _, _, _, "~amEmIşlErmIş";
    neg_plain_evid_past: _, _, _, _, _, "~amEmIşlErdI";
    neg_plain_evid_cond: "~amEmIşsEm", "~amEmIşsEn", "~amEmIşsE", "~amEmIşsEk", "~amEmIşsEnIz", "~amEmIşlErsE";
    neg_plain_fut__: "~amEyEcEğIm", "~amEyEcEksIn", "~amEyEcEk", "~amEyEcEğIz", "~amEyEcEksInIz", "~amEyEcEklEr";
    neg_plain_fut_evid: "~amEyEcEkmIşIm", "~amEyEcEkmIşsIn", "~amEyEcEkmIş", "~amEyEcEkmIşIz", "~amEyEcEkmIşsInIz", "~amEyEcEklErmIş";
    neg_plain_fut_past: "~amEyEcEktIm", "~amEyEcEktIn", "~amEyEcEktI", "~amEyEcEktIk", "~amEyEcEktInIz", "~amEyEcEklErdI";
    neg_plain_fut_cond: "~amEyEcEksEm", "~amEyEcEksEn", "~amEyEcEksE", "~amEyEcEksEk", "~amEyEcEksEnIz", "~amEyEcEklErsE";
    neg_plain_cont__: "~amEmEktEyIm", "~amEmEktEsIn", "~amEmEktE", "~amEmEktEyIz", "~amEmEktEsInIz", "~amEmEktElEr";
    neg_plain_cont_evid: "~amEmEktEymIşIm", "~amEmEktEymIşsIn", "~amEmEktEymIş", "~amEmEktEymIşIz", "~amEmEktEymIşsInIz", "~amEmEktElErmIş";
    neg_plain_cont_past: "~amEmEktEydIm", "~amEmEktEydIn", "~amEmEktEydI", "~amEmEktEydIk", "~amEmEktEydInIz", "~amEmEktElErdI";
    neg_plain_cont_cond: "~amEmEktEysEm", "~amEmEktEysEn", "~amEmEktEysE", "~amEmEktEysEk", "~amEmEktEysEnIz", "~amEmEktElErsE";
    neg_plain_necess__: "~amEmElIyIm", "~amEmElIsIn", "~amEmElI", "~amEmElIyIz", "~amEmElIsInIz", "~amEmElIlEr";
    neg_plain_necess_evid: "~amEmElIymIşIm", "~amEmElIymIşsIn", "~amEmElIymIş", "~amEmElIymIşIz", "~amEmElIymIşsInIz", "~amEmElIlErmIş";
    neg_plain_necess_past: "~amEmElIydIm", "~amEmElIydIn", "~amEmElIydI", "~amEmElIydIk", "~amEmElIydInIz", "~amEmElIlErdI";
    neg_plain_necess_cond: "~amEmElIysEm", "~amEmElIysEn", "~amEmElIysE", "~amEmElIysEk", "~amEmElIysEnIz", "~amEmElIlErsE";
    neg_abil_aor__: "~ayEmEm", "~ayEmEzsIn", "~ayEmEz", "~ayEmEyIz", "~ayEmEzsInIz", "~ayEmEzlEr";
    neg_abil_aor_evid: "~ayEmEzmIşIm", "~ayEmEzmIşsIn", "~ayEmEzmIş", "~ayEmEzmIşIz", "~ayEmEzmIşsInIz", "~ayEmEzlErmIş";
    neg_abil_aor_past: "~ayEmEzdIm", "~ayEmEzdIn", "~ayEmEzdI", "~ayEmEzdIk", "~ayEmEzdInIz", "~ayEmEzlErdI";
    neg_abil_aor_cond: "~ayEmEzsEm", "~ayEmEzsEn", "~ayEmEzsE", "~ayEmEzsEk", "~ayEmEzsEnIz", "~ayEmEzlErsE";
    neg_abil_prog__: "~ayEmIyorum", "~ayEmIyorsun", "~ayEmIyor", "~ayEmIyoruz", "~ayEmIyorsunuz", "~ayEmIyorlar";
    neg_abil_prog_evid: "~ayEmIyormuşum", "~ayEmIyormuşsun", "~ayEmIyormuş", "~ayEmIyormuşuz", "~ayEmIyormuşsunuz", "~ayEmIyorlarmış";
    neg_abil_prog_past: "~ayEmIyordum", "~ayEmIyordun", "~ayEmIyordu", "~ayEmIyorduk", "~ayEmIyordunuz", "~ayEmIyorlardı";
    neg_abil_prog_cond: "~ayEmIyorsam", "~ayEmIyorsan", "~ayEmIyorsa", "~ayEmIyorsak", "~ayEmIyorsanız", "~ayEmIyorlarsa";
    neg_abil_evid__: "~ayEmEmIşIm", "~ayEmEmIşsIn", "~ayEmEmIş", "~ayEmEmIşIz", "~ayEmEmIşsInIz", "~ayEmEmIşlEr";
    neg_abil_evid_evid: "~ayEmEmIşmIşIm", "~ayEmEmIşmIşsIn", "~ayEmEmIşmIş", "~ayEmEmIşmIşIz", "~ayEmEmIşmIşsInIz", "~ayEmEmIşlErmIş";
    neg_abil_evid_past: "~ayEmEmIştIm", "~ayEmEmIştIn", "~ayEmEmIştI", "~ayEmEmIştIk", "~ayEmEmIştInIz", "~ayEmEmIşlErdI";
    neg_abil_evid_cond: "~ayEmEmIşsEm", "~ayEmEmIşsEn", "~ayEmEmIşsE", "~ayEmEmIşsEk", "~ayEmEmIşsEnIz", "~ayEmEmIşlErsE";
    neg_abil_fut__: "~ayEmEyEcEğIm", "~ayEmEyEcEksIn", "~ayEmEyEcEk", "~ayEmEyEcEğIz", "~ayEmEyEcEksInIz", "~ayEmEyEcEklEr";
    neg_abil_fut_evid: "~ayEmEyEcEkmIşIm", "~ayEmEyEcEkmIşsIn", "~ayEmEyEcEkmIş", "~ayEmEyEcEkmIşIz", "~ayEmEyEcEkmIşsInIz", "~ayEmEyEcEklErmIş";
    neg_abil_fut_past: "~ayEmEyEcEktIm", "~ayEmEyEcEktIn", "~ayEmEyEcEktI", "~ayEmEyEcEktIk", "~ayEmEyEcEktInIz", "~ayEmEyEcEklErdI";
    neg_abil_fut_cond: "~ayEmEyEcEksEm", "~ayEmEyEcEksEn", "~ayEmEyEcEksE", "~ayEmEyEcEksEk", "~ayEmEyEcEksEnIz", "~ayEmEyEcEklErsE";
    neg_abil_cont__: "~ayEmEmEktEyIm", "~ayEmEmEktEsIn", "~ayEmEmEktE", "~ayEmEmEktEyIz", "~ayEmEmEktEsInIz", "~ayEmEmEktElEr";
    neg_abil_cont_evid: "~ayEmEmEktEymIşIm", "~ayEmEmEktEymIşsIn", "~ayEmEmEktEymIş", "~ayEmEmEktEymIşIz", "~ayEmEmEktEymIşsInIz", "~ayEmEmEktElErmIş";
    neg_abil_cont_past: "~ayEmEmEktEydIm", "~ayEmEmEktEydIn", "~ayEmEmEktEydI", "~ayEmEmEktEydIk", "~ayEmEmEktEydInIz", "~ayEmEmEktElErdI";
    neg_abil_cont_cond: "~ayEmEmEktEysEm", "~ayEmEmEktEysEn", "~ayEmEmEktEysE", "~ayEmEmEktEysEk", "~ayEmEmEktEysEnIz", "~ayEmEmEktElErsE";
    neg_abil_necess__: "~ayEmEmElIyIm", "~ayEmEmElIsIn", "~ayEmEmElI", "~ayEmEmElIyIz", "~ayEmEmElIsInIz", "~ayEmEmElIlEr";
    neg_abil_necess_evid: "~ayEmEmElIymIşIm", "~ayEmEmElIymIşsIn", "~ayEmEmElIymIş", "~ayEmEmElIymIşIz", "~ayEmEmElIymIşsInIz", "~ayEmEmElIlErmIş";
    neg_abil_necess_past: "~ayEmEmElIydIm", "~ayEmEmElIydIn", "~ayEmEmElIydI", "~ayEmEmElIydIk", "~ayEmEmElIydInIz", "~ayEmEmElIlErdI";
    neg_abil_necess_cond: "~ayEmEmElIysEm", "~ayEmEmElIysEn", "~ayEmEmElIysE", "~ayEmEmElIysEk", "~ayEmEmElIysEnIz", "~ayEmEmElIlErsE";
    past_pos_plain__: "~adIm", _, _, _, _, _;
    past_pos_abil__: "~ayEbildim", "~ayEbildin", "~ayEbildi", "~ayEbildik", "~ayEbildiniz", "~ayEbildiler";
    past_pos_abil_past: "~ayEbildiydim", "~ayEbildiydin", "~ayEbildiydi", "~ayEbildiydik", "~ayEbildiydiniz", "~ayEbildilerdi";
    past_pos_abil_cond: "~ayEbildiysem", "~ayEbildiysen", "~ayEbildiyse", "~ayEbildiysek", "~ayEbildiyseniz", "~ayEbildilerse";
    past_neg_plain__: _, _, _, _, _, "~amEdIlEr";
    past_neg_plain_past: _, _, _, _, _, "~amEdIlErdI";
    past_neg_plain_cond: "~amEdIysEm", "~amEdIysEn", "~amEdIysE", "~amEdIysEk", "~amEdIysEnIz", "~amEdIlErsE";
    past_neg_abil__: "~ayEmEdIm", "~ayEmEdIn", "~ayEmEdI", "~ayEmEdIk", "~ayEmEdInIz", "~ayEmEdIlEr";
    past_neg_abil_past: "~ayEmEdIydIm", "~ayEmEdIydIn", "~ayEmEdIydI", "~ayEmEdIydIk", "~ayEmEdIydInIz", "~ayEmEdIlErdI";
    past_neg_abil_cond: "~ayEmEdIysEm", "~ayEmEdIysEn", "~ayEmEdIysE", "~ayEmEdIysEk", "~ayEmEdIysEnIz", "~ayEmEdIlErsE";
    conditional_pos_plain__: "~asEm", "~asEn", "~asE", "~asEk", "~asEnIz", "~asElEr";
    conditional_pos_plain_evid: "~asEymIşIm", "~asEymIşsIn", "~asEymIş", "~asEymIşIz", "~asEymIşsInIz", "~asElErmIş";
    conditional_pos_plain_past: "~asEydIm", "~asEydIn", "~asEydI", "~asEydIk", "~asEydInIz", "~asElErdI";
    conditional_pos_abil__: "~ayEbilsem", "~ayEbilsen", "~ayEbilse", "~ayEbilsek", "~ayEbilseniz", "~ayEbilseler";
    conditional_pos_abil_evid: "~ayEbilseymişim", "~ayEbilseymişsin", "~ayEbilseymiş", "~ayEbilseymişiz", "~ayEbilseymişsiniz", "~ayEbilselermiş";
    conditional_pos_abil_past: "~ayEbilseydim", "~ayEbilseydin", "~ayEbilseydi", "~ayEbilseydik", "~ayEbilseydiniz", "~ayEbilselerdi";
    conditional_neg_plain__: "~amEsEm", "~amEsEn", "~amEsE", "~amEsEk", "~amEsEnIz", "~amEsElEr";
    conditional_neg_plain_evid: "~amEsEymIşIm", "~amEsEymIşsIn", "~amEsEymIş", "~amEsEymIşIz", "~amEsEymIşsInIz", "~amEsElErmIş";
    conditional_neg_plain_past: "~amEsEydIm", "~amEsEydIn", "~amEsEydI", "~amEsEydIk", "~amEsEydInIz", "~amEsElErdI";
    conditional_neg_abil__: "~ayEmEsEm", "~ayEmEsEn", "~ayEmEsE", "~ayEmEsEk", "~ayEmEsEnIz", "~ayEmEsElEr";
    conditional_neg_abil_evid: "~ayEmEsEymIşIm", "~ayEmEsEymIşsIn", "~ayEmEsEymIş", "~ayEmEsEymIşIz", "~ayEmEsEymIşsInIz", "~ayEmEsElErmIş";
    conditional_neg_abil_past: "~ayEmEsEydIm", "~ayEmEsEydIn", "~ayEmEsEydI", "~ayEmEsEydIk", "~ayEmEsEydInIz", "~ayEmEsElErdI";
    optative_pos_plain__: "~ayEyIm", "~ayEsIn", "~ayE", "~ayElIm", "~ayEsInIz", "~ayElEr";
    optative_pos_plain_evid: "~ayEymIşIm", "~ayEymIşsIn", "~ayEymIş", "~ayEymIşIz", "~ayEymIşsInIz", "~ayElErmIş";
    optative_pos_plain_past: "~ayEydIm", "~ayEydIn", "~ayEydI", "~ayEydIk", "~ayEydInIz", "~ayElErdI";
    optative_pos_abil__: "~ayEbileyim", "~ayEbilesin", "~ayEbile", "~ayEbilelim", "~ayEbilesiniz", "~ayEbileler";
    optative_pos_abil_evid: "~ayEbileymişim", "~ayEbileymişsin", "~ayEbileymiş", "~ayEbileymişiz", "~ayEbileymişsiniz", "~ayEbilelermiş";
    optative_pos_abil_past: "~ayEbileydim", "~ayEbileydin", "~ayEbileydi", "~ayEbileydik", "~ayEbileydiniz", "~ayEbilelerdi";
    optative_neg_plain__: "~amEyEyIm", "~amEyEsIn", "~amEyE", "~amEyElIm", "~amEyEsInIz", "~amEyElEr";
    optative_neg_plain_evid: "~amEyEymIşIm", "~amEyEymIşsIn", "~amEyEymIş", "~amEyEymIşIz", "~amEyEymIşsInIz", "~amEyElErmIş";
    optative_neg_plain_past: "~amEyEydIm", "~amEyEydIn", "~amEyEydI", "~amEyEydIk", "~amEyEydInIz", "~amEyElErdI";
    optative_neg_abil__: "~ayEmEyEyIm", "~ayEmEyEsIn", "~ayEmEyE", "~ayEmEyElIm", "~ayEmEyEsInIz", "~ayEmEyElEr";
    optative_neg_abil_evid: "~ayEmEyEymIşIm", "~ayEmEyEymIşsIn", "~ayEmEyEymIş", "~ayEmEyEymIşIz", "~ayEmEyEymIşsInIz", "~ayEmEyElErmIş";
    optative_neg_abil_past: "~ayEmEyEydIm", "~ayEmEyEydIn", "~ayEmEyEydI", "~ayEmEyEydIk", "~ayEmEyEydInIz", "~ayEmEyElErdI";
    imperative_pos_plain: "~a", "~asIn", "~ayIn", "~asInlEr", _, "~ayInIz", "~asEnE", "~asEnIzE";
    imperative_pos_abil: "~ayEbil", "~ayEbilsin", "~ayEbilin", "~ayEbilsinler", "~ayEbiliniz", _, "~ayEbilsene", "~ayEbilsenize";
    imperative_neg_plain: _, _, _, "~amEsInlEr", _, "~amEyInIz", "~amEsEnE", "~amEsEnIzE";
    imperative_neg_abil: "~ayEmE", "~ayEmEsIn", "~ayEmEyIn", "~ayEmEsInlEr", "~ayEmEyInIz", _, "~ayEmEsEnE", "~ayEmEsEnIzE";
    infinitive_pos: "~amak", "~ayEbilmek";
    infinitive_neg: "~amEmEk", "~ayEmEmEk";
    verbal_noun_pos: "~amE", "~ayEbilme";
    verbal_noun_neg: "~amEmE", "~ayEmEmE";
    participle_pos_plain: "~ayEn", _, _;
    participle_pos_abil: "~ayEbilen", _, _;
    participle_neg_plain: "~amEyEn", "~amEmIş", _;
    participle_neg_abil: "~ayEmEyEn", _, _;
    relative_pos_plain_acak: _, _, _, _, _, "~ayEcEklErI";
    relative_pos_abil_dik: "~ayEbildiğim", "~ayEbildiğin", "~ayEbildiği", "~ayEbildiğimiz", "~ayEbildiğiniz", "~ayEbildikleri";
    relative_pos_abil_acak: _, "~ayEbileceğin", "~ayEbileceği", "~ayEbileceğimiz", "~ayEbileceğiniz", "~ayEbilecekleri";
    relative_neg_plain_dik: _, _, _, _, _, "~amEdIklErI";
    relative_neg_plain_acak: _, "~amEyEcEğIn", "~amEyEcEğI", "~amEyEcEğImIz", "~amEyEcEğInIz", "~amEyEcEklErI";
    relative_neg_abil_dik: "~ayEmEdIğIm", "~ayEmEdIğIn", "~ayEmEdIğI", "~ayEmEdIğImIz", "~ayEmEdIğInIz", "~ayEmEdIklErI";
    relative_neg_abil_acak: _, "~ayEmEyEcEğIn", "~ayEmEyEcEğI", "~ayEmEyEcEğImIz", "~ayEmEyEcEğInIz", "~ayEmEyEcEklErI";
    converb_pos_plain: "~arken", _, _, "~ayElI", "~ayEsIyE", _, "~ayErEk", "~aya ~aya", "~arcEsInE";
    converb_pos_abil: "~ayEbilIrken", "~ayEbilince", "~ayEbilip", "~ayEbileli", "~ayEbilesiye", "~ayEbildikçe", "~ayEbilerek", "~ayabile ~ayabile", _;
    converb_neg_plain: _, "~amEyIncE", _, "~amEyElI", "~amEyEsIyE", "~amEdIkçE", "~amEyErEk", _, "~amEzcEsInE";
    converb_neg_abil: "~ayEmEzken", "~ayEmEyIncE", "~ayEmEyIp", "~ayEmEyElI", "~ayEmEyEsIyE", "~ayEmEdIkçE", "~ayEmEyErEk", _, "~ayEmEzcEsInE";
    ),
    // Takipetmek: 186 words (terk etmek, nefret etmek, kabul etmek, tahliye etmek, itiraf etmek, taklit etmek).
    conjugation!("takip etmek", 186, "~tmek";
    stem lemma: "~- like takip etmek";
    stem converb_neg_plain_doubled: "~tmeye etmeye- like vermek";
    stem converb_neg_abil_doubled: "~demeye edemeye- like vermek";
    imperative_pos_abil: _, _, _, _, _, derived, _, _;
    imperative_neg_abil: _, _, _, _, _, derived, _, _;
    participle_pos_plain: _, derived, derived;
    participle_pos_abil: _, derived, derived;
    participle_neg_plain: _, _, derived;
    participle_neg_abil: _, derived, derived;
    relative_pos_plain_acak: derived, _, _, _, _, _;
    relative_pos_abil_acak: derived, _, _, _, _, _;
    relative_neg_plain_acak: derived, _, _, _, _, _;
    relative_neg_abil_acak: derived, _, _, _, _, _;
    ),
    // Beslemek: 152 words (ezberlemek, gülümsemek, titremek, ertelemek, istemek, bellemek).
    conjugation!("beslemek", 152, "~emek";
    stem converb_neg_plain_doubled: "~emeye ~emeye- like vermek";
    stem converb_neg_abil_doubled: "~eyemeye ~eyemeye- like vermek";
    pos_plain_aor__: "~erIm", "~ersIn", "~er", _, "~ersInIz", "~erler";
    pos_plain_aor_evid: "~ermIşIm", "~ermIşsIn", "~ermIş", "~ermIşIz", "~ermIşsInIz", "~erlermIş";
    pos_plain_aor_past: "~erdIm", "~erdIn", "~erdI", "~erdIk", "~erdInIz", "~erlerdI";
    pos_plain_aor_cond: "~ersem", "~ersen", "~erse", "~ersek", "~ersenIz", "~erlerse";
    pos_plain_prog__: "~Iyorum", _, _, _, _, _;
    pos_plain_evid__: _, "~emIşsIn", "~emIş", _, "~emIşsInIz", "~emIşlEr";
    pos_plain_evid_evid: "~emIşmIşIm", "~emIşmIşsIn", "~emIşmIş", "~emIşmIşIz", "~emIşmIşsInIz", "~emIşlErmIş";
    pos_plain_evid_past: "~emIştIm", "~emIştIn", "~emIştI", "~emIştIk", "~emIştInIz", "~emIşlErdI";
    pos_plain_evid_cond: "~emIşsEm", "~emIşsEn", "~emIşsE", "~emIşsEk", "~emIşsEnIz", "~emIşlErsE";
    pos_plain_fut__: "~eyeceğIm", "~eyeceksIn", "~eyecek", _, "~eyeceksInIz", "~eyecekler";
    pos_plain_fut_evid: "~eyecekmIşIm", "~eyecekmIşsIn", "~eyecekmIş", "~eyecekmIşIz", "~eyecekmIşsInIz", "~eyeceklermIş";
    pos_plain_fut_past: "~eyecektIm", "~eyecektIn", "~eyecektI", "~eyecektIk", "~eyecektInIz", "~eyeceklerdI";
    pos_plain_fut_cond: "~eyeceksem", "~eyeceksen", "~eyecekse", "~eyeceksek", "~eyeceksenIz", "~eyeceklerse";
    pos_plain_cont__: "~emekteyIm", "~emektesIn", "~emekte", "~emekteyIz", "~emektesInIz", "~emekteler";
    pos_plain_cont_evid: "~emekteymIşIm", "~emekteymIşsIn", "~emekteymIş", "~emekteymIşIz", "~emekteymIşsInIz", "~emektelermIş";
    pos_plain_cont_past: "~emekteydIm", "~emekteydIn", "~emekteydI", "~emekteydIk", "~emekteydInIz", "~emektelerdI";
    pos_plain_cont_cond: "~emekteysem", "~emekteysen", "~emekteyse", "~emekteysek", "~emekteysenIz", "~emektelerse";
    pos_plain_necess__: _, _, _, _, _, "~emelIlEr";
    pos_plain_necess_evid: _, _, _, _, _, "~emelIlErmIş";
    pos_plain_necess_past: _, _, _, _, _, "~emelIlErdI";
    pos_plain_necess_cond: "~emelIysEm", "~emelIysEn", "~emelIysE", "~emelIysEk", "~emelIysEnIz", "~emelIlErsE";
    pos_abil_aor__: "~eyebilirim", "~eyebilirsin", "~eyebilir", "~eyebiliriz", "~eyebilirsiniz", "~eyebilirler";
    pos_abil_aor_evid: "~eyebilirmişim", "~eyebilirmişsin", "~eyebilirmiş", "~eyebilirmişiz", "~eyebilirmişsiniz", "~eyebilirlermiş";
    pos_abil_aor_past: "~eyebilirdim", "~eyebilirdin", "~eyebilirdi", "~eyebilirdik", "~eyebilirdiniz", "~eyebilirlerdi";
    pos_abil_aor_cond: "~eyebilirsem", "~eyebilirsen", "~eyebilirse", "~eyebilirsek", "~eyebilirseniz", "~eyebilirlerse";
    pos_abil_prog__: "~eyebiliyorum", "~eyebiliyorsun", "~eyebiliyor", "~eyebiliyoruz", "~eyebiliyorsunuz", "~eyebiliyorlar";
    pos_abil_prog_evid: "~eyebiliyormuşum", "~eyebiliyormuşsun", "~eyebiliyormuş", "~eyebiliyormuşuz", "~eyebiliyormuşsunuz", "~eyebiliyorlarmış";
    pos_abil_prog_past: "~eyebiliyordum", "~eyebiliyordun", "~eyebiliyordu", "~eyebiliyorduk", "~eyebiliyordunuz", "~eyebiliyorlardı";
    pos_abil_prog_cond: "~eyebiliyorsam", "~eyebiliyorsan", "~eyebiliyorsa", "~eyebiliyorsak", "~eyebiliyorsanız", "~eyebiliyorlarsa";
    pos_abil_evid__: "~eyebilmişim", "~eyebilmişsin", "~eyebilmiş", "~eyebilmişiz", "~eyebilmişsiniz", "~eyebilmişler";
    pos_abil_evid_evid: "~eyebilmişmişim", "~eyebilmişmişsin", "~eyebilmişmiş", "~eyebilmişmişiz", "~eyebilmişmişsiniz", "~eyebilmişlermiş";
    pos_abil_evid_past: "~eyebilmiştim", "~eyebilmiştin", "~eyebilmişti", "~eyebilmiştik", "~eyebilmiştiniz", "~eyebilmişlerdi";
    pos_abil_evid_cond: "~eyebilmişsem", "~eyebilmişsen", "~eyebilmişse", "~eyebilmişsek", "~eyebilmişseniz", "~eyebilmişlerse";
    pos_abil_fut__: "~eyebileceğim", "~eyebileceksin", "~eyebilecek", "~eyebileceğiz", "~eyebileceksiniz", "~eyebilecekler";
    pos_abil_fut_evid: "~eyebilecekmişim", "~eyebilecekmişsin", "~eyebilecekmiş", "~eyebilecekmişiz", "~eyebilecekmişsiniz", "~eyebileceklermiş";
    pos_abil_fut_past: "~eyebilecektim", "~eyebilecektin", "~eyebilecekti", "~eyebilecektik", "~eyebilecektiniz", "~eyebileceklerdi";
    pos_abil_fut_cond: "~eyebileceksem", "~eyebileceksen", "~eyebilecekse", "~eyebileceksek", "~eyebilecekseniz", "~eyebileceklerse";
    pos_abil_cont__: "~eyebilmekteyim", "~eyebilmektesin", "~eyebilmekte", "~eyebilmekteyiz", "~eyebilmektesiniz", "~eyebilmekteler";
    pos_abil_cont_evid: "~eyebilmekteymişim", "~eyebilmekteymişsin", "~eyebilmekteymiş", "~eyebilmekteymişiz", "~eyebilmekteymişsiniz", "~eyebilmektelermiş";
    pos_abil_cont_past: "~eyebilmekteydim", "~eyebilmekteydin", "~eyebilmekteydi", "~eyebilmekteydik", "~eyebilmekteydiniz", "~eyebilmektelerdi";
    pos_abil_cont_cond: "~eyebilmekteysem", "~eyebilmekteysen", "~eyebilmekteyse", "~eyebilmekteysek", "~eyebilmekteyseniz", "~eyebilmektelerse";
    pos_abil_necess__: "~eyebilmeliyim", "~eyebilmelisin", "~eyebilmeli", "~eyebilmeliyiz", "~eyebilmelisiniz", "~eyebilmeliler";
    pos_abil_necess_evid: "~eyebilmeliymişim", "~eyebilmeliymişsin", "~eyebilmeliymiş", "~eyebilmeliymişiz", "~eyebilmeliymişsiniz", "~eyebilmelilermiş";
    pos_abil_necess_past: "~eyebilmeliydim", "~eyebilmeliydin", "~eyebilmeliydi", "~eyebilmeliydik", "~eyebilmeliydiniz", "~eyebilmelilerdi";
    pos_abil_necess_cond: "~eyebilmeliysem", "~eyebilmeliysen", "~eyebilmeliyse", "~eyebilmeliysek", "~eyebilmeliyseniz", "~eyebilmelilerse";
    neg_plain_aor__: _, _, _, _, _, "~emezler";
    neg_plain_aor_evid: _, _, _, _, _, "~emezlermIş";
    neg_plain_aor_past: _, _, _, _, _, "~emezlerdI";
    neg_plain_aor_cond: "~emezsem", "~emezsen", "~emezse", "~emezsek", "~emezsenIz", "~emezlerse";
    neg_plain_prog__: "~emIyorum", "~emIyorsun", "~emIyor", "~emIyoruz", "~emIyorsunuz", "~emIyorlar";
    neg_plain_prog_evid: "~emIyormuşum", "~emIyormuşsun", "~emIyormuş", "~emIyormuşuz", "~emIyormuşsunuz", "~emIyorlarmış";
    neg_plain_prog_past: "~emIyordum", "~emIyordun", "~emIyordu", "~emIyorduk", "~emIyordunuz", "~emIyorlardı";
    neg_plain_prog_cond: "~emIyorsam", "~emIyorsan", "~emIyorsa", "~emIyorsak", "~emIyorsanız", "~emIyorlarsa";
    neg_plain_evid__: _, _, _, _, _, "~ememIşlEr";
    neg_plain_evid_evid: _, _, _, _, _, "~ememIşlErmIş";
    neg_plain_evid_past: _, _, _, _, _, "~ememIşlErdI";
    neg_plain_evid_cond: "~ememIşsEm", "~ememIşsEn", "~ememIşsE", "~ememIşsEk", "~ememIşsEnIz", "~ememIşlErsE";
    neg_plain_fut__: "~emeyeceğIm", "~emeyeceksIn", "~emeyecek", "~emeyeceğIz", "~emeyeceksInIz", "~emeyecekler";
    neg_plain_fut_evid: "~emeyecekmIşIm", "~emeyecekmIşsIn", "~emeyecekmIş", "~emeyecekmIşIz", "~emeyecekmIşsInIz", "~emeyeceklermIş";
    neg_plain_fut_past: "~emeyecektIm", "~emeyecektIn", "~emeyecektI", "~emeyecektIk", "~emeyecektInIz", "~emeyeceklerdI";
    neg_plain_fut_cond: "~emeyeceksem", "~emeyeceksen", "~emeyecekse", "~emeyeceksek", "~emeyeceksenIz", "~emeyeceklerse";
    neg_plain_cont__: "~ememekteyIm", "~ememektesIn", "~ememekte", "~ememekteyIz", "~ememektesInIz", "~ememekteler";
    neg_plain_cont_evid: "~ememekteymIşIm", "~ememekteymIşsIn", "~ememekteymIş", "~ememekteymIşIz", "~ememekteymIşsInIz", "~ememektelermIş";
    neg_plain_cont_past: "~ememekteydIm", "~ememekteydIn", "~ememekteydI", "~ememekteydIk", "~ememekteydInIz", "~ememektelerdI";
    neg_plain_cont_cond: "~ememekteysem", "~ememekteysen", "~ememekteyse", "~ememekteysek", "~ememekteysenIz", "~ememektelerse";
    neg_plain_necess__: "~ememelIyIm", "~ememelIsIn", "~ememelI", "~ememelIyIz", "~ememelIsInIz", "~ememelIlEr";
    neg_plain_necess_evid: "~ememelIymIşIm", "~ememelIymIşsIn", "~ememelIymIş", "~ememelIymIşIz", "~ememelIymIşsInIz", "~ememelIlErmIş";
    neg_plain_necess_past: "~ememelIydIm", "~ememelIydIn", "~ememelIydI", "~ememelIydIk", "~ememelIydInIz", "~ememelIlErdI";
    neg_plain_necess_cond: "~ememelIysEm", "~ememelIysEn", "~ememelIysE", "~ememelIysEk", "~ememelIysEnIz", "~ememelIlErsE";
    neg_abil_aor__: "~eyemem", "~eyemezsIn", "~eyemez", "~eyemeyIz", "~eyemezsInIz", "~eyemezler";
    neg_abil_aor_evid: "~eyemezmIşIm", "~eyemezmIşsIn", "~eyemezmIş", "~eyemezmIşIz", "~eyemezmIşsInIz", "~eyemezlermIş";
    neg_abil_aor_past: "~eyemezdIm", "~eyemezdIn", "~eyemezdI", "~eyemezdIk", "~eyemezdInIz", "~eyemezlerdI";
    neg_abil_aor_cond: "~eyemezsem", "~eyemezsen", "~eyemezse", "~eyemezsek", "~eyemezsenIz", "~eyemezlerse";
    neg_abil_prog__: "~eyemIyorum", "~eyemIyorsun", "~eyemIyor", "~eyemIyoruz", "~eyemIyorsunuz", "~eyemIyorlar";
    neg_abil_prog_evid: "~eyemIyormuşum", "~eyemIyormuşsun", "~eyemIyormuş", "~eyemIyormuşuz", "~eyemIyormuşsunuz", "~eyemIyorlarmış";
    neg_abil_prog_past: "~eyemIyordum", "~eyemIyordun", "~eyemIyordu", "~eyemIyorduk", "~eyemIyordunuz", "~eyemIyorlardı";
    neg_abil_prog_cond: "~eyemIyorsam", "~eyemIyorsan", "~eyemIyorsa", "~eyemIyorsak", "~eyemIyorsanız", "~eyemIyorlarsa";
    neg_abil_evid__: "~eyememIşIm", "~eyememIşsIn", "~eyememIş", "~eyememIşIz", "~eyememIşsInIz", "~eyememIşlEr";
    neg_abil_evid_evid: "~eyememIşmIşIm", "~eyememIşmIşsIn", "~eyememIşmIş", "~eyememIşmIşIz", "~eyememIşmIşsInIz", "~eyememIşlErmIş";
    neg_abil_evid_past: "~eyememIştIm", "~eyememIştIn", "~eyememIştI", "~eyememIştIk", "~eyememIştInIz", "~eyememIşlErdI";
    neg_abil_evid_cond: "~eyememIşsEm", "~eyememIşsEn", "~eyememIşsE", "~eyememIşsEk", "~eyememIşsEnIz", "~eyememIşlErsE";
    neg_abil_fut__: "~eyemeyeceğIm", "~eyemeyeceksIn", "~eyemeyecek", "~eyemeyeceğIz", "~eyemeyeceksInIz", "~eyemeyecekler";
    neg_abil_fut_evid: "~eyemeyecekmIşIm", "~eyemeyecekmIşsIn", "~eyemeyecekmIş", "~eyemeyecekmIşIz", "~eyemeyecekmIşsInIz", "~eyemeyeceklermIş";
    neg_abil_fut_past: "~eyemeyecektIm", "~eyemeyecektIn", "~eyemeyecektI", "~eyemeyecektIk", "~eyemeyecektInIz", "~eyemeyeceklerdI";
    neg_abil_fut_cond: "~eyemeyeceksem", "~eyemeyeceksen", "~eyemeyecekse", "~eyemeyeceksek", "~eyemeyeceksenIz", "~eyemeyeceklerse";
    neg_abil_cont__: "~eyememekteyIm", "~eyememektesIn", "~eyememekte", "~eyememekteyIz", "~eyememektesInIz", "~eyememekteler";
    neg_abil_cont_evid: "~eyememekteymIşIm", "~eyememekteymIşsIn", "~eyememekteymIş", "~eyememekteymIşIz", "~eyememekteymIşsInIz", "~eyememektelermIş";
    neg_abil_cont_past: "~eyememekteydIm", "~eyememekteydIn", "~eyememekteydI", "~eyememekteydIk", "~eyememekteydInIz", "~eyememektelerdI";
    neg_abil_cont_cond: "~eyememekteysem", "~eyememekteysen", "~eyememekteyse", "~eyememekteysek", "~eyememekteysenIz", "~eyememektelerse";
    neg_abil_necess__: "~eyememelIyIm", "~eyememelIsIn", "~eyememelI", "~eyememelIyIz", "~eyememelIsInIz", "~eyememelIlEr";
    neg_abil_necess_evid: "~eyememelIymIşIm", "~eyememelIymIşsIn", "~eyememelIymIş", "~eyememelIymIşIz", "~eyememelIymIşsInIz", "~eyememelIlErmIş";
    neg_abil_necess_past: "~eyememelIydIm", "~eyememelIydIn", "~eyememelIydI", "~eyememelIydIk", "~eyememelIydInIz", "~eyememelIlErdI";
    neg_abil_necess_cond: "~eyememelIysEm", "~eyememelIysEn", "~eyememelIysE", "~eyememelIysEk", "~eyememelIysEnIz", "~eyememelIlErsE";
    past_pos_plain__: "~edIm", _, _, _, _, _;
    past_pos_abil__: "~eyebildim", "~eyebildin", "~eyebildi", "~eyebildik", "~eyebildiniz", "~eyebildiler";
    past_pos_abil_past: "~eyebildiydim", "~eyebildiydin", "~eyebildiydi", "~eyebildiydik", "~eyebildiydiniz", "~eyebildilerdi";
    past_pos_abil_cond: "~eyebildiysem", "~eyebildiysen", "~eyebildiyse", "~eyebildiysek", "~eyebildiyseniz", "~eyebildilerse";
    past_neg_plain__: _, _, _, _, _, "~emedIlEr";
    past_neg_plain_past: _, _, _, _, _, "~emedIlErdI";
    past_neg_plain_cond: "~emedIysEm", "~emedIysEn", "~emedIysE", "~emedIysEk", "~emedIysEnIz", "~emedIlErsE";
    past_neg_abil__: "~eyemedIm", "~eyemedIn", "~eyemedI", "~eyemedIk", "~eyemedInIz", "~eyemedIlEr";
    past_neg_abil_past: "~eyemedIydIm", "~eyemedIydIn", "~eyemedIydI", "~eyemedIydIk", "~eyemedIydInIz", "~eyemedIlErdI";
    past_neg_abil_cond: "~eyemedIysEm", "~eyemedIysEn", "~eyemedIysE", "~eyemedIysEk", "~eyemedIysEnIz", "~eyemedIlErsE";
    conditional_pos_plain__: "~esem", "~esen", "~ese", "~esek", "~esenIz", "~eseler";
    conditional_pos_plain_evid: "~eseymIşIm", "~eseymIşsIn", "~eseymIş", "~eseymIşIz", "~eseymIşsInIz", "~eselermIş";
    conditional_pos_plain_past: "~eseydIm", "~eseydIn", "~eseydI", "~eseydIk", "~eseydInIz", "~eselerdI";
    conditional_pos_abil__: "~eyebilsem", "~eyebilsen", "~eyebilse", "~eyebilsek", "~eyebilseniz", "~eyebilseler";
    conditional_pos_abil_evid: "~eyebilseymişim", "~eyebilseymişsin", "~eyebilseymiş", "~eyebilseymişiz", "~eyebilseymişsiniz", "~eyebilselermiş";
    conditional_pos_abil_past: "~eyebilseydim", "~eyebilseydin", "~eyebilseydi", "~eyebilseydik", "~eyebilseydiniz", "~eyebilselerdi";
    conditional_neg_plain__: "~emesem", "~emesen", "~emese", "~emesek", "~emesenIz", "~emeseler";
    conditional_neg_plain_evid: "~emeseymIşIm", "~emeseymIşsIn", "~emeseymIş", "~emeseymIşIz", "~emeseymIşsInIz", "~emeselermIş";
    conditional_neg_plain_past: "~emeseydIm", "~emeseydIn", "~emeseydI", "~emeseydIk", "~emeseydInIz", "~emeselerdI";
    conditional_neg_abil__: "~eyemesem", "~eyemesen", "~eyemese", "~eyemesek", "~eyemesenIz", "~eyemeseler";
    conditional_neg_abil_evid: "~eyemeseymIşIm", "~eyemeseymIşsIn", "~eyemeseymIş", "~eyemeseymIşIz", "~eyemeseymIşsInIz", "~eyemeselermIş";
    conditional_neg_abil_past: "~eyemeseydIm", "~eyemeseydIn", "~eyemeseydI", "~eyemeseydIk", "~eyemeseydInIz", "~eyemeselerdI";
    optative_pos_plain__: "~eyeyIm", "~eyesIn", "~eye", "~eyelIm", "~eyesInIz", "~eyeler";
    optative_pos_plain_evid: "~eyeymIşIm", "~eyeymIşsIn", "~eyeymIş", "~eyeymIşIz", "~eyeymIşsInIz", "~eyelermIş";
    optative_pos_plain_past: "~eyeydIm", "~eyeydIn", "~eyeydI", "~eyeydIk", "~eyeydInIz", "~eyelerdI";
    optative_pos_abil__: "~eyebileyim", "~eyebilesin", "~eyebile", "~eyebilelim", "~eyebilesiniz", "~eyebileler";
    optative_pos_abil_evid: "~eyebileymişim", "~eyebileymişsin", "~eyebileymiş", "~eyebileymişiz", "~eyebileymişsiniz", "~eyebilelermiş";
    optative_pos_abil_past: "~eyebileydim", "~eyebileydin", "~eyebileydi", "~eyebileydik", "~eyebileydiniz", "~eyebilelerdi";
    optative_neg_plain__: "~emeyeyIm", "~emeyesIn", "~emeye", "~emeyelIm", "~emeyesInIz", "~emeyeler";
    optative_neg_plain_evid: "~emeyeymIşIm", "~emeyeymIşsIn", "~emeyeymIş", "~emeyeymIşIz", "~emeyeymIşsInIz", "~emeyelermIş";
    optative_neg_plain_past: "~emeyeydIm", "~emeyeydIn", "~emeyeydI", "~emeyeydIk", "~emeyeydInIz", "~emeyelerdI";
    optative_neg_abil__: "~eyemeyeyIm", "~eyemeyesIn", "~eyemeye", "~eyemeyelIm", "~eyemeyesInIz", "~eyemeyeler";
    optative_neg_abil_evid: "~eyemeyeymIşIm", "~eyemeyeymIşsIn", "~eyemeyeymIş", "~eyemeyeymIşIz", "~eyemeyeymIşsInIz", "~eyemeyelermIş";
    optative_neg_abil_past: "~eyemeyeydIm", "~eyemeyeydIn", "~eyemeyeydI", "~eyemeyeydIk", "~eyemeyeydInIz", "~eyemeyelerdI";
    imperative_pos_plain: "~e", "~esIn", "~eyIn", "~esInlEr", _, "~eyInIz", "~esene", "~esenIzE";
    imperative_pos_abil: "~eyebil", "~eyebilsin", "~eyebilin", "~eyebilsinler", "~eyebiliniz", _, "~eyebilsene", "~eyebilsenize";
    imperative_neg_plain: _, _, _, "~emesInlEr", _, "~emeyInIz", "~emesene", "~emesenIzE";
    imperative_neg_abil: "~eyeme", "~eyemesIn", "~eyemeyIn", "~eyemesInlEr", "~eyemeyInIz", _, "~eyemesene", "~eyemesenIzE";
    infinitive_pos: "~emek", "~eyebilmek";
    infinitive_neg: "~ememek", "~eyememek";
    verbal_noun_pos: "~eme", "~eyebilme";
    verbal_noun_neg: "~ememe", "~eyememe";
    participle_pos_plain: "~eyen", _, _;
    participle_pos_abil: "~eyebilen", _, _;
    participle_neg_plain: "~emeyen", "~ememIş", _;
    participle_neg_abil: "~eyemeyen", _, _;
    relative_pos_plain_acak: _, _, _, _, _, "~eyeceklerI";
    relative_pos_abil_dik: "~eyebildiğim", "~eyebildiğin", "~eyebildiği", "~eyebildiğimiz", "~eyebildiğiniz", "~eyebildikleri";
    relative_pos_abil_acak: _, "~eyebileceğin", "~eyebileceği", "~eyebileceğimiz", "~eyebileceğiniz", "~eyebilecekleri";
    relative_neg_plain_dik: _, _, _, _, _, "~emedIklErI";
    relative_neg_plain_acak: _, "~emeyeceğIn", "~emeyeceğI", "~emeyeceğImIz", "~emeyeceğInIz", "~emeyeceklerI";
    relative_neg_abil_dik: "~eyemedIğIm", "~eyemedIğIn", "~eyemedIğI", "~eyemedIğImIz", "~eyemedIğInIz", "~eyemedIklErI";
    relative_neg_abil_acak: _, "~eyemeyeceğIn", "~eyemeyeceğI", "~eyemeyeceğImIz", "~eyemeyeceğInIz", "~eyemeyeceklerI";
    converb_pos_plain: "~erken", _, _, "~eyelI", "~eyesIyE", _, "~eyerek", "~eye ~eye", "~ercesInE";
    converb_pos_abil: "~eyebilIrken", "~eyebilince", "~eyebilip", "~eyebileli", "~eyebilesiye", "~eyebildikçe", "~eyebilerek", "~eyebile ~eyebile", _;
    converb_neg_plain: _, "~emeyIncE", _, "~emeyelI", "~emeyesIyE", "~emedIkçE", "~emeyerek", _, "~emezcesInE";
    converb_neg_abil: "~eyemezken", "~eyemeyIncE", "~eyemeyIp", "~eyemeyelI", "~eyemeyesIyE", "~eyemedIkçE", "~eyemeyerek", _, "~eyemezcesInE";
    ),
    // Açmak: 123 words (batmak, koşmak, atmak, yazmak, kurmak, yakmak). Like Varolmak, except in 25 cells: pos_plain_aor__ _1sg, pos_plain_aor__ _2sg, pos_plain_aor__ _3sg, pos_plain_aor__ _2pl, pos_plain_aor__ _3pl, pos_plain_aor_evid _1sg, ….
    conjugation!("açmak", 123, "~mak", like Varolmak;
    pos_plain_aor__: "~ErIm", "~ErsIn", "~Er", _, "~ErsInIz", "~ErlEr";
    pos_plain_aor_evid: "~ErmIşIm", "~ErmIşsIn", "~ErmIş", "~ErmIşIz", "~ErmIşsInIz", "~ErlErmIş";
    pos_plain_aor_past: "~ErdIm", "~ErdIn", "~ErdI", "~ErdIk", "~ErdInIz", "~ErlErdI";
    pos_plain_aor_cond: "~ErsEm", "~ErsEn", "~ErsE", "~ErsEk", "~ErsEnIz", "~ErlErsE";
    converb_pos_plain: "~Erken", _, _, _, _, _, _, _, "~ErcEsInE";
    ),
    // Çekmek: 104 words (ekmek, silmek, gülmek, bitmek, vazgeçmek, yenmek). Like Vermek, except in 26 cells: pos_plain_aor__ _1sg, pos_plain_aor__ _2sg, pos_plain_aor__ _3sg, pos_plain_aor__ _2pl, pos_plain_aor__ _3pl, pos_plain_aor_evid _1sg, ….
    conjugation!("çekmek", 104, "~mek", like Vermek;
    pos_plain_aor__: "~ErIm", "~ErsIn", "~Er", _, "~ErsInIz", "~ErlEr";
    pos_plain_aor_evid: "~ErmIşIm", "~ErmIşsIn", "~ErmIş", "~ErmIşIz", "~ErmIşsInIz", "~ErlErmIş";
    pos_plain_aor_past: "~ErdIm", "~ErdIn", "~ErdI", "~ErdIk", "~ErdInIz", "~ErlErdI";
    pos_plain_aor_cond: "~ErsEm", "~ErsEn", "~ErsE", "~ErsEk", "~ErsEnIz", "~ErlErsE";
    past_pos_plain__: "~tIm", _, _, _, _, _;
    converb_pos_plain: "~Erken", _, _, _, _, _, _, _, "~ErcEsInE";
    ),
    // Gitmek: 53 words (reddetmek, gitmek, etmek, affetmek, seyretmek, keşfetmek). Like Takipetmek, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("gitmek", 53, "~tmek", like Takipetmek;
    stem converb_neg_plain_doubled: "~tmeye ~tmeye- like vermek";
    stem converb_neg_abil_doubled: "~demeye ~demeye- like vermek";
    converb_pos_plain: _, _, _, _, _, _, _, "~de ~de", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~debile ~debile", _;
    ),
    // Elevermek: 33 words (söz vermek, ateşe vermek, yanıt vermek, cevap vermek, karar vermek, can vermek). Like Vermek, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("ele vermek", 33, "~mek", like Vermek;
    stem converb_neg_plain_doubled: "~mEyE vErmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE vErEmEyE- like vermek";
    converb_pos_plain: _, _, _, _, _, _, _, "~E vErE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile vErEbile", _;
    ),
    // Silmek: 15 words (fotoğraf çekmek, zartayı çekmek, mortoyu çekmek, cızlamı çekmek, cartayı çekmek, cavlağı çekmek).
    conjugation!("silmek", 15, "~mek";
    stem converb_neg_plain_doubled: "~meye ~meye- like vermek";
    stem converb_neg_abil_doubled: "~emeye ~emeye- like vermek";
    pos_plain_aor__: "~erIm", "~ersIn", "~er", _, "~ersInIz", "~erler";
    pos_plain_aor_evid: "~ermIşIm", "~ermIşsIn", "~ermIş", "~ermIşIz", "~ermIşsInIz", "~erlermIş";
    pos_plain_aor_past: "~erdIm", "~erdIn", "~erdI", "~erdIk", "~erdInIz", "~erlerdI";
    pos_plain_aor_cond: "~ersem", "~ersen", "~erse", "~ersek", "~ersenIz", "~erlerse";
    pos_plain_prog__: "~Iyorum", _, _, _, _, _;
    pos_plain_evid__: _, "~mIşsIn", "~mIş", _, "~mIşsInIz", "~mIşlEr";
    pos_plain_evid_evid: "~mIşmIşIm", "~mIşmIşsIn", "~mIşmIş", "~mIşmIşIz", "~mIşmIşsInIz", "~mIşlErmIş";
    pos_plain_evid_past: "~mIştIm", "~mIştIn", "~mIştI", "~mIştIk", "~mIştInIz", "~mIşlErdI";
    pos_plain_evid_cond: "~mIşsEm", "~mIşsEn", "~mIşsE", "~mIşsEk", "~mIşsEnIz", "~mIşlErsE";
    pos_plain_fut__: "~eceğIm", "~eceksIn", "~ecek", _, "~eceksInIz", "~ecekler";
    pos_plain_fut_evid: "~ecekmIşIm", "~ecekmIşsIn", "~ecekmIş", "~ecekmIşIz", "~ecekmIşsInIz", "~eceklermIş";
    pos_plain_fut_past: "~ecektIm", "~ecektIn", "~ecektI", "~ecektIk", "~ecektInIz", "~eceklerdI";
    pos_plain_fut_cond: "~eceksem", "~eceksen", "~ecekse", "~eceksek", "~eceksenIz", "~eceklerse";
    pos_plain_cont__: "~mekteyIm", "~mektesIn", "~mekte", "~mekteyIz", "~mektesInIz", "~mekteler";
    pos_plain_cont_evid: "~mekteymIşIm", "~mekteymIşsIn", "~mekteymIş", "~mekteymIşIz", "~mekteymIşsInIz", "~mektelermIş";
    pos_plain_cont_past: "~mekteydIm", "~mekteydIn", "~mekteydI", "~mekteydIk", "~mekteydInIz", "~mektelerdI";
    pos_plain_cont_cond: "~mekteysem", "~mekteysen", "~mekteyse", "~mekteysek", "~mekteysenIz", "~mektelerse";
    pos_plain_necess__: _, _, _, _, _, "~melIlEr";
    pos_plain_necess_evid: _, _, _, _, _, "~melIlErmIş";
    pos_plain_necess_past: _, _, _, _, _, "~melIlErdI";
    pos_plain_necess_cond: "~melIysEm", "~melIysEn", "~melIysE", "~melIysEk", "~melIysEnIz", "~melIlErsE";
    pos_abil_aor__: "~ebilirim", "~ebilirsin", "~ebilir", "~ebiliriz", "~ebilirsiniz", "~ebilirler";
    pos_abil_aor_evid: "~ebilirmişim", "~ebilirmişsin", "~ebilirmiş", "~ebilirmişiz", "~ebilirmişsiniz", "~ebilirlermiş";
    pos_abil_aor_past: "~ebilirdim", "~ebilirdin", "~ebilirdi", "~ebilirdik", "~ebilirdiniz", "~ebilirlerdi";
    pos_abil_aor_cond: "~ebilirsem", "~ebilirsen", "~ebilirse", "~ebilirsek", "~ebilirseniz", "~ebilirlerse";
    pos_abil_prog__: "~ebiliyorum", "~ebiliyorsun", "~ebiliyor", "~ebiliyoruz", "~ebiliyorsunuz", "~ebiliyorlar";
    pos_abil_prog_evid: "~ebiliyormuşum", "~ebiliyormuşsun", "~ebiliyormuş", "~ebiliyormuşuz", "~ebiliyormuşsunuz", "~ebiliyorlarmış";
    pos_abil_prog_past: "~ebiliyordum", "~ebiliyordun", "~ebiliyordu", "~ebiliyorduk", "~ebiliyordunuz", "~ebiliyorlardı";
    pos_abil_prog_cond: "~ebiliyorsam", "~ebiliyorsan", "~ebiliyorsa", "~ebiliyorsak", "~ebiliyorsanız", "~ebiliyorlarsa";
    pos_abil_evid__: "~ebilmişim", "~ebilmişsin", "~ebilmiş", "~ebilmişiz", "~ebilmişsiniz", "~ebilmişler";
    pos_abil_evid_evid: "~ebilmişmişim", "~ebilmişmişsin", "~ebilmişmiş", "~ebilmişmişiz", "~ebilmişmişsiniz", "~ebilmişlermiş";
    pos_abil_evid_past: "~ebilmiştim", "~ebilmiştin", "~ebilmişti", "~ebilmiştik", "~ebilmiştiniz", "~ebilmişlerdi";
    pos_abil_evid_cond: "~ebilmişsem", "~ebilmişsen", "~ebilmişse", "~ebilmişsek", "~ebilmişseniz", "~ebilmişlerse";
    pos_abil_fut__: "~ebileceğim", "~ebileceksin", "~ebilecek", "~ebileceğiz", "~ebileceksiniz", "~ebilecekler";
    pos_abil_fut_evid: "~ebilecekmişim", "~ebilecekmişsin", "~ebilecekmiş", "~ebilecekmişiz", "~ebilecekmişsiniz", "~ebileceklermiş";
    pos_abil_fut_past: "~ebilecektim", "~ebilecektin", "~ebilecekti", "~ebilecektik", "~ebilecektiniz", "~ebileceklerdi";
    pos_abil_fut_cond: "~ebileceksem", "~ebileceksen", "~ebilecekse", "~ebileceksek", "~ebilecekseniz", "~ebileceklerse";
    pos_abil_cont__: "~ebilmekteyim", "~ebilmektesin", "~ebilmekte", "~ebilmekteyiz", "~ebilmektesiniz", "~ebilmekteler";
    pos_abil_cont_evid: "~ebilmekteymişim", "~ebilmekteymişsin", "~ebilmekteymiş", "~ebilmekteymişiz", "~ebilmekteymişsiniz", "~ebilmektelermiş";
    pos_abil_cont_past: "~ebilmekteydim", "~ebilmekteydin", "~ebilmekteydi", "~ebilmekteydik", "~ebilmekteydiniz", "~ebilmektelerdi";
    pos_abil_cont_cond: "~ebilmekteysem", "~ebilmekteysen", "~ebilmekteyse", "~ebilmekteysek", "~ebilmekteyseniz", "~ebilmektelerse";
    pos_abil_necess__: "~ebilmeliyim", "~ebilmelisin", "~ebilmeli", "~ebilmeliyiz", "~ebilmelisiniz", "~ebilmeliler";
    pos_abil_necess_evid: "~ebilmeliymişim", "~ebilmeliymişsin", "~ebilmeliymiş", "~ebilmeliymişiz", "~ebilmeliymişsiniz", "~ebilmelilermiş";
    pos_abil_necess_past: "~ebilmeliydim", "~ebilmeliydin", "~ebilmeliydi", "~ebilmeliydik", "~ebilmeliydiniz", "~ebilmelilerdi";
    pos_abil_necess_cond: "~ebilmeliysem", "~ebilmeliysen", "~ebilmeliyse", "~ebilmeliysek", "~ebilmeliyseniz", "~ebilmelilerse";
    neg_plain_aor__: _, _, _, _, _, "~mezler";
    neg_plain_aor_evid: _, _, _, _, _, "~mezlermIş";
    neg_plain_aor_past: _, _, _, _, _, "~mezlerdI";
    neg_plain_aor_cond: "~mezsem", "~mezsen", "~mezse", "~mezsek", "~mezsenIz", "~mezlerse";
    neg_plain_prog__: "~mIyorum", "~mIyorsun", "~mIyor", "~mIyoruz", "~mIyorsunuz", "~mIyorlar";
    neg_plain_prog_evid: "~mIyormuşum", "~mIyormuşsun", "~mIyormuş", "~mIyormuşuz", "~mIyormuşsunuz", "~mIyorlarmış";
    neg_plain_prog_past: "~mIyordum", "~mIyordun", "~mIyordu", "~mIyorduk", "~mIyordunuz", "~mIyorlardı";
    neg_plain_prog_cond: "~mIyorsam", "~mIyorsan", "~mIyorsa", "~mIyorsak", "~mIyorsanız", "~mIyorlarsa";
    neg_plain_evid__: _, _, _, _, _, "~memIşlEr";
    neg_plain_evid_evid: _, _, _, _, _, "~memIşlErmIş";
    neg_plain_evid_past: _, _, _, _, _, "~memIşlErdI";
    neg_plain_evid_cond: "~memIşsEm", "~memIşsEn", "~memIşsE", "~memIşsEk", "~memIşsEnIz", "~memIşlErsE";
    neg_plain_fut__: "~meyeceğIm", "~meyeceksIn", "~meyecek", "~meyeceğIz", "~meyeceksInIz", "~meyecekler";
    neg_plain_fut_evid: "~meyecekmIşIm", "~meyecekmIşsIn", "~meyecekmIş", "~meyecekmIşIz", "~meyecekmIşsInIz", "~meyeceklermIş";
    neg_plain_fut_past: "~meyecektIm", "~meyecektIn", "~meyecektI", "~meyecektIk", "~meyecektInIz", "~meyeceklerdI";
    neg_plain_fut_cond: "~meyeceksem", "~meyeceksen", "~meyecekse", "~meyeceksek", "~meyeceksenIz", "~meyeceklerse";
    neg_plain_cont__: "~memekteyIm", "~memektesIn", "~memekte", "~memekteyIz", "~memektesInIz", "~memekteler";
    neg_plain_cont_evid: "~memekteymIşIm", "~memekteymIşsIn", "~memekteymIş", "~memekteymIşIz", "~memekteymIşsInIz", "~memektelermIş";
    neg_plain_cont_past: "~memekteydIm", "~memekteydIn", "~memekteydI", "~memekteydIk", "~memekteydInIz", "~memektelerdI";
    neg_plain_cont_cond: "~memekteysem", "~memekteysen", "~memekteyse", "~memekteysek", "~memekteysenIz", "~memektelerse";
    neg_plain_necess__: "~memelIyIm", "~memelIsIn", "~memelI", "~memelIyIz", "~memelIsInIz", "~memelIlEr";
    neg_plain_necess_evid: "~memelIymIşIm", "~memelIymIşsIn", "~memelIymIş", "~memelIymIşIz", "~memelIymIşsInIz", "~memelIlErmIş";
    neg_plain_necess_past: "~memelIydIm", "~memelIydIn", "~memelIydI", "~memelIydIk", "~memelIydInIz", "~memelIlErdI";
    neg_plain_necess_cond: "~memelIysEm", "~memelIysEn", "~memelIysE", "~memelIysEk", "~memelIysEnIz", "~memelIlErsE";
    neg_abil_aor__: "~emem", "~emezsIn", "~emez", "~emeyIz", "~emezsInIz", "~emezler";
    neg_abil_aor_evid: "~emezmIşIm", "~emezmIşsIn", "~emezmIş", "~emezmIşIz", "~emezmIşsInIz", "~emezlermIş";
    neg_abil_aor_past: "~emezdIm", "~emezdIn", "~emezdI", "~emezdIk", "~emezdInIz", "~emezlerdI";
    neg_abil_aor_cond: "~emezsem", "~emezsen", "~emezse", "~emezsek", "~emezsenIz", "~emezlerse";
    neg_abil_prog__: "~emIyorum", "~emIyorsun", "~emIyor", "~emIyoruz", "~emIyorsunuz", "~emIyorlar";
    neg_abil_prog_evid: "~emIyormuşum", "~emIyormuşsun", "~emIyormuş", "~emIyormuşuz", "~emIyormuşsunuz", "~emIyorlarmış";
    neg_abil_prog_past: "~emIyordum", "~emIyordun", "~emIyordu", "~emIyorduk", "~emIyordunuz", "~emIyorlardı";
    neg_abil_prog_cond: "~emIyorsam", "~emIyorsan", "~emIyorsa", "~emIyorsak", "~emIyorsanız", "~emIyorlarsa";
    neg_abil_evid__: "~ememIşIm", "~ememIşsIn", "~ememIş", "~ememIşIz", "~ememIşsInIz", "~ememIşlEr";
    neg_abil_evid_evid: "~ememIşmIşIm", "~ememIşmIşsIn", "~ememIşmIş", "~ememIşmIşIz", "~ememIşmIşsInIz", "~ememIşlErmIş";
    neg_abil_evid_past: "~ememIştIm", "~ememIştIn", "~ememIştI", "~ememIştIk", "~ememIştInIz", "~ememIşlErdI";
    neg_abil_evid_cond: "~ememIşsEm", "~ememIşsEn", "~ememIşsE", "~ememIşsEk", "~ememIşsEnIz", "~ememIşlErsE";
    neg_abil_fut__: "~emeyeceğIm", "~emeyeceksIn", "~emeyecek", "~emeyeceğIz", "~emeyeceksInIz", "~emeyecekler";
    neg_abil_fut_evid: "~emeyecekmIşIm", "~emeyecekmIşsIn", "~emeyecekmIş", "~emeyecekmIşIz", "~emeyecekmIşsInIz", "~emeyeceklermIş";
    neg_abil_fut_past: "~emeyecektIm", "~emeyecektIn", "~emeyecektI", "~emeyecektIk", "~emeyecektInIz", "~emeyeceklerdI";
    neg_abil_fut_cond: "~emeyeceksem", "~emeyeceksen", "~emeyecekse", "~emeyeceksek", "~emeyeceksenIz", "~emeyeceklerse";
    neg_abil_cont__: "~ememekteyIm", "~ememektesIn", "~ememekte", "~ememekteyIz", "~ememektesInIz", "~ememekteler";
    neg_abil_cont_evid: "~ememekteymIşIm", "~ememekteymIşsIn", "~ememekteymIş", "~ememekteymIşIz", "~ememekteymIşsInIz", "~ememektelermIş";
    neg_abil_cont_past: "~ememekteydIm", "~ememekteydIn", "~ememekteydI", "~ememekteydIk", "~ememekteydInIz", "~ememektelerdI";
    neg_abil_cont_cond: "~ememekteysem", "~ememekteysen", "~ememekteyse", "~ememekteysek", "~ememekteysenIz", "~ememektelerse";
    neg_abil_necess__: "~ememelIyIm", "~ememelIsIn", "~ememelI", "~ememelIyIz", "~ememelIsInIz", "~ememelIlEr";
    neg_abil_necess_evid: "~ememelIymIşIm", "~ememelIymIşsIn", "~ememelIymIş", "~ememelIymIşIz", "~ememelIymIşsInIz", "~ememelIlErmIş";
    neg_abil_necess_past: "~ememelIydIm", "~ememelIydIn", "~ememelIydI", "~ememelIydIk", "~ememelIydInIz", "~ememelIlErdI";
    neg_abil_necess_cond: "~ememelIysEm", "~ememelIysEn", "~ememelIysE", "~ememelIysEk", "~ememelIysEnIz", "~ememelIlErsE";
    past_pos_plain__: "~dIm", _, _, _, _, _;
    past_pos_abil__: "~ebildim", "~ebildin", "~ebildi", "~ebildik", "~ebildiniz", "~ebildiler";
    past_pos_abil_past: "~ebildiydim", "~ebildiydin", "~ebildiydi", "~ebildiydik", "~ebildiydiniz", "~ebildilerdi";
    past_pos_abil_cond: "~ebildiysem", "~ebildiysen", "~ebildiyse", "~ebildiysek", "~ebildiyseniz", "~ebildilerse";
    past_neg_plain__: _, _, _, _, _, "~medIlEr";
    past_neg_plain_past: _, _, _, _, _, "~medIlErdI";
    past_neg_plain_cond: "~medIysEm", "~medIysEn", "~medIysE", "~medIysEk", "~medIysEnIz", "~medIlErsE";
    past_neg_abil__: "~emedIm", "~emedIn", "~emedI", "~emedIk", "~emedInIz", "~emedIlEr";
    past_neg_abil_past: "~emedIydIm", "~emedIydIn", "~emedIydI", "~emedIydIk", "~emedIydInIz", "~emedIlErdI";
    past_neg_abil_cond: "~emedIysEm", "~emedIysEn", "~emedIysE", "~emedIysEk", "~emedIysEnIz", "~emedIlErsE";
    conditional_pos_plain__: "~sem", "~sen", "~se", "~sek", "~senIz", "~seler";
    conditional_pos_plain_evid: "~seymIşIm", "~seymIşsIn", "~seymIş", "~seymIşIz", "~seymIşsInIz", "~selermIş";
    conditional_pos_plain_past: "~seydIm", "~seydIn", "~seydI", "~seydIk", "~seydInIz", "~selerdI";
    conditional_pos_abil__: "~ebilsem", "~ebilsen", "~ebilse", "~ebilsek", "~ebilseniz", "~ebilseler";
    conditional_pos_abil_evid: "~ebilseymişim", "~ebilseymişsin", "~ebilseymiş", "~ebilseymişiz", "~ebilseymişsiniz", "~ebilselermiş";
    conditional_pos_abil_past: "~ebilseydim", "~ebilseydin", "~ebilseydi", "~ebilseydik", "~ebilseydiniz", "~ebilselerdi";
    conditional_neg_plain__: "~mesem", "~mesen", "~mese", "~mesek", "~mesenIz", "~meseler";
    conditional_neg_plain_evid: "~meseymIşIm", "~meseymIşsIn", "~meseymIş", "~meseymIşIz", "~meseymIşsInIz", "~meselermIş";
    conditional_neg_plain_past: "~meseydIm", "~meseydIn", "~meseydI", "~meseydIk", "~meseydInIz", "~meselerdI";
    conditional_neg_abil__: "~emesem", "~emesen", "~emese", "~emesek", "~emesenIz", "~emeseler";
    conditional_neg_abil_evid: "~emeseymIşIm", "~emeseymIşsIn", "~emeseymIş", "~emeseymIşIz", "~emeseymIşsInIz", "~emeselermIş";
    conditional_neg_abil_past: "~emeseydIm", "~emeseydIn", "~emeseydI", "~emeseydIk", "~emeseydInIz", "~emeselerdI";
    optative_pos_plain__: "~eyIm", "~esIn", "~e", "~elIm", "~esInIz", "~eler";
    optative_pos_plain_evid: "~eymIşIm", "~eymIşsIn", "~eymIş", "~eymIşIz", "~eymIşsInIz", "~elermIş";
    optative_pos_plain_past: "~eydIm", "~eydIn", "~eydI", "~eydIk", "~eydInIz", "~elerdI";
    optative_pos_abil__: "~ebileyim", "~ebilesin", "~ebile", "~ebilelim", "~ebilesiniz", "~ebileler";
    optative_pos_abil_evid: "~ebileymişim", "~ebileymişsin", "~ebileymiş", "~ebileymişiz", "~ebileymişsiniz", "~ebilelermiş";
    optative_pos_abil_past: "~ebileydim", "~ebileydin", "~ebileydi", "~ebileydik", "~ebileydiniz", "~ebilelerdi";
    optative_neg_plain__: "~meyeyIm", "~meyesIn", "~meye", "~meyelIm", "~meyesInIz", "~meyeler";
    optative_neg_plain_evid: "~meyeymIşIm", "~meyeymIşsIn", "~meyeymIş", "~meyeymIşIz", "~meyeymIşsInIz", "~meyelermIş";
    optative_neg_plain_past: "~meyeydIm", "~meyeydIn", "~meyeydI", "~meyeydIk", "~meyeydInIz", "~meyelerdI";
    optative_neg_abil__: "~emeyeyIm", "~emeyesIn", "~emeye", "~emeyelIm", "~emeyesInIz", "~emeyeler";
    optative_neg_abil_evid: "~emeyeymIşIm", "~emeyeymIşsIn", "~emeyeymIş", "~emeyeymIşIz", "~emeyeymIşsInIz", "~emeyelermIş";
    optative_neg_abil_past: "~emeyeydIm", "~emeyeydIn", "~emeyeydI", "~emeyeydIk", "~emeyeydInIz", "~emeyelerdI";
    imperative_pos_plain: "~", "~sIn", "~In", "~sInlEr", _, "~InIz", "~sene", "~senIzE";
    imperative_pos_abil: "~ebil", "~ebilsin", "~ebilin", "~ebilsinler", "~ebiliniz", _, "~ebilsene", "~ebilsenize";
    imperative_neg_plain: _, _, _, "~mesInlEr", _, "~meyInIz", "~mesene", "~mesenIzE";
    imperative_neg_abil: "~eme", "~emesIn", "~emeyIn", "~emesInlEr", "~emeyInIz", _, "~emesene", "~emesenIzE";
    infinitive_pos: "~mek", "~ebilmek";
    infinitive_neg: "~memek", "~ememek";
    verbal_noun_pos: "~me", "~ebilme";
    verbal_noun_neg: "~meme", "~ememe";
    participle_pos_plain: "~en", _, _;
    participle_pos_abil: "~ebilen", _, _;
    participle_neg_plain: "~meyen", "~memIş", _;
    participle_neg_abil: "~emeyen", _, _;
    relative_pos_plain_acak: _, _, _, _, _, "~eceklerI";
    relative_pos_abil_dik: "~ebildiğim", "~ebildiğin", "~ebildiği", "~ebildiğimiz", "~ebildiğiniz", "~ebildikleri";
    relative_pos_abil_acak: _, "~ebileceğin", "~ebileceği", "~ebileceğimiz", "~ebileceğiniz", "~ebilecekleri";
    relative_neg_plain_dik: _, _, _, _, _, "~medIklErI";
    relative_neg_plain_acak: _, "~meyeceğIn", "~meyeceğI", "~meyeceğImIz", "~meyeceğInIz", "~meyeceklerI";
    relative_neg_abil_dik: "~emedIğIm", "~emedIğIn", "~emedIğI", "~emedIğImIz", "~emedIğInIz", "~emedIklErI";
    relative_neg_abil_acak: _, "~emeyeceğIn", "~emeyeceğI", "~emeyeceğImIz", "~emeyeceğInIz", "~emeyeceklerI";
    converb_pos_plain: "~erken", _, _, "~elI", "~esIyE", _, "~erek", "~e ~e", "~ercesInE";
    converb_pos_abil: "~ebilIrken", "~ebilince", "~ebilip", "~ebileli", "~ebilesiye", "~ebildikçe", "~ebilerek", "~ebile ~ebile", _;
    converb_neg_plain: _, "~meyIncE", _, "~meyelI", "~meyesIyE", "~medIkçE", "~meyerek", _, "~mezcesInE";
    converb_neg_abil: "~emezken", "~emeyIncE", "~emeyIp", "~emeyelI", "~emeyesIyE", "~emedIkçE", "~emeyerek", _, "~emezcesInE";
    ),
    // Aşkyapmak: 15 words (hata yapmak, makyaj yapmak, seks yapmak, dedikodu yapmak, ağda yapmak, sugeçirmez yapmak). Like Açmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("aşk yapmak", 15, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE yEpmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE yEpEmEyE- like vermek";
    converb_pos_plain: _, _, _, _, _, _, _, "~E yEpE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile yapEbile", _;
    ),
    // Yürümek: 13 words (yürümek, üşümek, ekşimek, büyümek, çürümek, eskimek).
    conjugation!("yürümek", 13, "~mek";
    stem converb_neg_plain_doubled: "~meye ~meye- like vermek";
    stem converb_neg_abil_doubled: "~yemeye ~yemeye- like vermek";
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
    pos_plain_cont__: "~mektEyIm", "~mektEsIn", "~mektE", "~mektEyIz", "~mektEsInIz", "~mektElEr";
    pos_plain_cont_evid: "~mektEymIşIm", "~mektEymIşsIn", "~mektEymIş", "~mektEymIşIz", "~mektEymIşsInIz", "~mektElErmIş";
    pos_plain_cont_past: "~mektEydIm", "~mektEydIn", "~mektEydI", "~mektEydIk", "~mektEydInIz", "~mektElErdI";
    pos_plain_cont_cond: "~mektEysEm", "~mektEysEn", "~mektEysE", "~mektEysEk", "~mektEysEnIz", "~mektElErsE";
    pos_plain_necess__: _, _, _, _, _, "~mElIlEr";
    pos_plain_necess_evid: _, _, _, _, _, "~mElIlErmIş";
    pos_plain_necess_past: _, _, _, _, _, "~mElIlErdI";
    pos_plain_necess_cond: "~mElIysEm", "~mElIysEn", "~mElIysE", "~mElIysEk", "~mElIysEnIz", "~mElIlErsE";
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
    neg_plain_aor__: _, _, _, _, _, "~mEzlEr";
    neg_plain_aor_evid: _, _, _, _, _, "~mEzlErmIş";
    neg_plain_aor_past: _, _, _, _, _, "~mEzlErdI";
    neg_plain_aor_cond: "~mEzsEm", "~mEzsEn", "~mEzsE", "~mEzsEk", "~mEzsEnIz", "~mEzlErsE";
    neg_plain_prog__: "~mIyorum", "~mIyorsun", "~mIyor", "~mIyoruz", "~mIyorsunuz", "~mIyorlar";
    neg_plain_prog_evid: "~mIyormuşum", "~mIyormuşsun", "~mIyormuş", "~mIyormuşuz", "~mIyormuşsunuz", "~mIyorlarmış";
    neg_plain_prog_past: "~mIyordum", "~mIyordun", "~mIyordu", "~mIyorduk", "~mIyordunuz", "~mIyorlardı";
    neg_plain_prog_cond: "~mIyorsam", "~mIyorsan", "~mIyorsa", "~mIyorsak", "~mIyorsanız", "~mIyorlarsa";
    neg_plain_evid__: _, _, _, _, _, "~mEmIşlEr";
    neg_plain_evid_evid: _, _, _, _, _, "~mEmIşlErmIş";
    neg_plain_evid_past: _, _, _, _, _, "~mEmIşlErdI";
    neg_plain_evid_cond: "~mEmIşsEm", "~mEmIşsEn", "~mEmIşsE", "~mEmIşsEk", "~mEmIşsEnIz", "~mEmIşlErsE";
    neg_plain_fut__: "~mEyEcEğIm", "~mEyEcEksIn", "~mEyEcEk", "~mEyEcEğIz", "~mEyEcEksInIz", "~mEyEcEklEr";
    neg_plain_fut_evid: "~mEyEcEkmIşIm", "~mEyEcEkmIşsIn", "~mEyEcEkmIş", "~mEyEcEkmIşIz", "~mEyEcEkmIşsInIz", "~mEyEcEklErmIş";
    neg_plain_fut_past: "~mEyEcEktIm", "~mEyEcEktIn", "~mEyEcEktI", "~mEyEcEktIk", "~mEyEcEktInIz", "~mEyEcEklErdI";
    neg_plain_fut_cond: "~mEyEcEksEm", "~mEyEcEksEn", "~mEyEcEksE", "~mEyEcEksEk", "~mEyEcEksEnIz", "~mEyEcEklErsE";
    neg_plain_cont__: "~mEmEktEyIm", "~mEmEktEsIn", "~mEmEktE", "~mEmEktEyIz", "~mEmEktEsInIz", "~mEmEktElEr";
    neg_plain_cont_evid: "~mEmEktEymIşIm", "~mEmEktEymIşsIn", "~mEmEktEymIş", "~mEmEktEymIşIz", "~mEmEktEymIşsInIz", "~mEmEktElErmIş";
    neg_plain_cont_past: "~mEmEktEydIm", "~mEmEktEydIn", "~mEmEktEydI", "~mEmEktEydIk", "~mEmEktEydInIz", "~mEmEktElErdI";
    neg_plain_cont_cond: "~mEmEktEysEm", "~mEmEktEysEn", "~mEmEktEysE", "~mEmEktEysEk", "~mEmEktEysEnIz", "~mEmEktElErsE";
    neg_plain_necess__: "~mEmElIyIm", "~mEmElIsIn", "~mEmElI", "~mEmElIyIz", "~mEmElIsInIz", "~mEmElIlEr";
    neg_plain_necess_evid: "~mEmElIymIşIm", "~mEmElIymIşsIn", "~mEmElIymIş", "~mEmElIymIşIz", "~mEmElIymIşsInIz", "~mEmElIlErmIş";
    neg_plain_necess_past: "~mEmElIydIm", "~mEmElIydIn", "~mEmElIydI", "~mEmElIydIk", "~mEmElIydInIz", "~mEmElIlErdI";
    neg_plain_necess_cond: "~mEmElIysEm", "~mEmElIysEn", "~mEmElIysE", "~mEmElIysEk", "~mEmElIysEnIz", "~mEmElIlErsE";
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
    past_neg_plain__: _, _, _, _, _, "~mEdIlEr";
    past_neg_plain_past: _, _, _, _, _, "~mEdIlErdI";
    past_neg_plain_cond: "~mEdIysEm", "~mEdIysEn", "~mEdIysE", "~mEdIysEk", "~mEdIysEnIz", "~mEdIlErsE";
    past_neg_abil__: "~yEmEdIm", "~yEmEdIn", "~yEmEdI", "~yEmEdIk", "~yEmEdInIz", "~yEmEdIlEr";
    past_neg_abil_past: "~yEmEdIydIm", "~yEmEdIydIn", "~yEmEdIydI", "~yEmEdIydIk", "~yEmEdIydInIz", "~yEmEdIlErdI";
    past_neg_abil_cond: "~yEmEdIysEm", "~yEmEdIysEn", "~yEmEdIysE", "~yEmEdIysEk", "~yEmEdIysEnIz", "~yEmEdIlErsE";
    conditional_pos_plain__: "~sEm", "~sEn", "~sE", "~sEk", "~sEnIz", "~sElEr";
    conditional_pos_plain_evid: "~sEymIşIm", "~sEymIşsIn", "~sEymIş", "~sEymIşIz", "~sEymIşsInIz", "~sElErmIş";
    conditional_pos_plain_past: "~sEydIm", "~sEydIn", "~sEydI", "~sEydIk", "~sEydInIz", "~sElErdI";
    conditional_pos_abil__: "~yEbilsem", "~yEbilsen", "~yEbilse", "~yEbilsek", "~yEbilseniz", "~yEbilseler";
    conditional_pos_abil_evid: "~yEbilseymişim", "~yEbilseymişsin", "~yEbilseymiş", "~yEbilseymişiz", "~yEbilseymişsiniz", "~yEbilselermiş";
    conditional_pos_abil_past: "~yEbilseydim", "~yEbilseydin", "~yEbilseydi", "~yEbilseydik", "~yEbilseydiniz", "~yEbilselerdi";
    conditional_neg_plain__: "~mEsEm", "~mEsEn", "~mEsE", "~mEsEk", "~mEsEnIz", "~mEsElEr";
    conditional_neg_plain_evid: "~mEsEymIşIm", "~mEsEymIşsIn", "~mEsEymIş", "~mEsEymIşIz", "~mEsEymIşsInIz", "~mEsElErmIş";
    conditional_neg_plain_past: "~mEsEydIm", "~mEsEydIn", "~mEsEydI", "~mEsEydIk", "~mEsEydInIz", "~mEsElErdI";
    conditional_neg_abil__: "~yEmEsEm", "~yEmEsEn", "~yEmEsE", "~yEmEsEk", "~yEmEsEnIz", "~yEmEsElEr";
    conditional_neg_abil_evid: "~yEmEsEymIşIm", "~yEmEsEymIşsIn", "~yEmEsEymIş", "~yEmEsEymIşIz", "~yEmEsEymIşsInIz", "~yEmEsElErmIş";
    conditional_neg_abil_past: "~yEmEsEydIm", "~yEmEsEydIn", "~yEmEsEydI", "~yEmEsEydIk", "~yEmEsEydInIz", "~yEmEsElErdI";
    optative_pos_plain__: "~yEyIm", "~yEsIn", "~yE", "~yElIm", "~yEsInIz", "~yElEr";
    optative_pos_plain_evid: "~yEymIşIm", "~yEymIşsIn", "~yEymIş", "~yEymIşIz", "~yEymIşsInIz", "~yElErmIş";
    optative_pos_plain_past: "~yEydIm", "~yEydIn", "~yEydI", "~yEydIk", "~yEydInIz", "~yElErdI";
    optative_pos_abil__: "~yEbileyim", "~yEbilesin", "~yEbile", "~yEbilelim", "~yEbilesiniz", "~yEbileler";
    optative_pos_abil_evid: "~yEbileymişim", "~yEbileymişsin", "~yEbileymiş", "~yEbileymişiz", "~yEbileymişsiniz", "~yEbilelermiş";
    optative_pos_abil_past: "~yEbileydim", "~yEbileydin", "~yEbileydi", "~yEbileydik", "~yEbileydiniz", "~yEbilelerdi";
    optative_neg_plain__: "~mEyEyIm", "~mEyEsIn", "~mEyE", "~mEyElIm", "~mEyEsInIz", "~mEyElEr";
    optative_neg_plain_evid: "~mEyEymIşIm", "~mEyEymIşsIn", "~mEyEymIş", "~mEyEymIşIz", "~mEyEymIşsInIz", "~mEyElErmIş";
    optative_neg_plain_past: "~mEyEydIm", "~mEyEydIn", "~mEyEydI", "~mEyEydIk", "~mEyEydInIz", "~mEyElErdI";
    optative_neg_abil__: "~yEmEyEyIm", "~yEmEyEsIn", "~yEmEyE", "~yEmEyElIm", "~yEmEyEsInIz", "~yEmEyElEr";
    optative_neg_abil_evid: "~yEmEyEymIşIm", "~yEmEyEymIşsIn", "~yEmEyEymIş", "~yEmEyEymIşIz", "~yEmEyEymIşsInIz", "~yEmEyElErmIş";
    optative_neg_abil_past: "~yEmEyEydIm", "~yEmEyEydIn", "~yEmEyEydI", "~yEmEyEydIk", "~yEmEyEydInIz", "~yEmEyElErdI";
    imperative_pos_plain: "~", "~sIn", "~yIn", "~sInlEr", _, "~yInIz", "~sEnE", "~sEnIzE";
    imperative_pos_abil: "~yEbil", "~yEbilsin", "~yEbilin", "~yEbilsinler", "~yEbiliniz", _, "~yEbilsene", "~yEbilsenize";
    imperative_neg_plain: _, _, _, "~mEsInlEr", _, "~mEyInIz", "~mEsEnE", "~mEsEnIzE";
    imperative_neg_abil: "~yEmE", "~yEmEsIn", "~yEmEyIn", "~yEmEsInlEr", "~yEmEyInIz", _, "~yEmEsEnE", "~yEmEsEnIzE";
    infinitive_pos: "~mek", "~yEbilmek";
    infinitive_neg: "~mEmEk", "~yEmEmEk";
    verbal_noun_pos: "~mE", "~yEbilme";
    verbal_noun_neg: "~mEmE", "~yEmEmE";
    participle_pos_plain: "~yEn", _, _;
    participle_pos_abil: "~yEbilen", _, _;
    participle_neg_plain: "~mEyEn", "~mEmIş", _;
    participle_neg_abil: "~yEmEyEn", _, _;
    relative_pos_plain_acak: _, _, _, _, _, "~yEcEklErI";
    relative_pos_abil_dik: "~yEbildiğim", "~yEbildiğin", "~yEbildiği", "~yEbildiğimiz", "~yEbildiğiniz", "~yEbildikleri";
    relative_pos_abil_acak: _, "~yEbileceğin", "~yEbileceği", "~yEbileceğimiz", "~yEbileceğiniz", "~yEbilecekleri";
    relative_neg_plain_dik: _, _, _, _, _, "~mEdIklErI";
    relative_neg_plain_acak: _, "~mEyEcEğIn", "~mEyEcEğI", "~mEyEcEğImIz", "~mEyEcEğInIz", "~mEyEcEklErI";
    relative_neg_abil_dik: "~yEmEdIğIm", "~yEmEdIğIn", "~yEmEdIğI", "~yEmEdIğImIz", "~yEmEdIğInIz", "~yEmEdIklErI";
    relative_neg_abil_acak: _, "~yEmEyEcEğIn", "~yEmEyEcEğI", "~yEmEyEcEğImIz", "~yEmEyEcEğInIz", "~yEmEyEcEklErI";
    converb_pos_plain: "~rken", _, _, "~yElI", "~yEsIyE", _, "~yErEk", "~ye ~ye", "~rcEsInE";
    converb_pos_abil: "~yEbilirken", "~yEbilince", "~yEbilip", "~yEbileli", "~yEbilesiye", "~yEbildikçe", "~yEbilerek", "~yebile ~yebile", _;
    converb_neg_plain: _, "~mEyIncE", _, "~mEyElI", "~mEyEsIyE", "~mEdIkçE", "~mEyErEk", _, "~mEzcEsInE";
    converb_neg_abil: "~yEmEzken", "~yEmEyIncE", "~yEmEyIp", "~yEmEyElI", "~yEmEyEsIyE", "~yEmEdIkçE", "~yEmEyErEk", _, "~yEmEzcEsInE";
    ),
    // Tekmeatmak: 12 words (havlu atmak, hava atmak, aşık atmak, kazık atmak, göz atmak, tekme atmak). Like Açmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("tekme atmak", 12, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE EtmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE EtEmEyE- like vermek";
    converb_pos_plain: _, _, _, _, _, _, _, "~E EtE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile atEbile", _;
    ),
    // Imanagelmek: 12 words (üstesinden gelmek, dize gelmek, manasına gelmek, anlamına gelmek, taklaya gelmek, kendine gelmek). Like Vermek, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("imana gelmek", 12, "~mek", like Vermek;
    stem converb_neg_plain_doubled: "~mEyE gElmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE gElEmEyE- like vermek";
    converb_pos_plain: _, _, _, _, _, _, _, "~E gElE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile gElEbile", _;
    ),
    // Camaçıkmak: 12 words (sahip çıkmak, kat çıkmak, karşı çıkmak, dağa çıkmak, eve çıkmak, cama çıkmak). Like Açmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("cama çıkmak", 12, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE çıkmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE çıkEmEyE- like vermek";
    converb_pos_plain: _, _, _, _, _, _, _, "~E çıkE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile çıkEbile", _;
    ),
    // Taşımak: 11 words (solumak, okumak, korumak, uyumak, meydan okumak, dokumak). Like Yürümek, except in 27 cells: pos_plain_cont__ _1sg, pos_plain_cont__ _2sg, pos_plain_cont__ _3sg, pos_plain_cont__ _1pl, pos_plain_cont__ _2pl, pos_plain_cont__ _3pl, ….
    conjugation!("taşımak", 11, "~mak", like Yürümek;
    stem converb_neg_plain_doubled: "~maya ~maya- like vermek";
    stem converb_neg_abil_doubled: "~yamaya ~yamaya- like vermek";
    pos_plain_cont__: "~maktEyIm", "~maktEsIn", "~maktE", "~maktEyIz", "~maktEsInIz", "~maktElEr";
    pos_plain_cont_evid: "~maktEymIşIm", "~maktEymIşsIn", "~maktEymIş", "~maktEymIşIz", "~maktEymIşsInIz", "~maktElErmIş";
    pos_plain_cont_past: "~maktEydIm", "~maktEydIn", "~maktEydI", "~maktEydIk", "~maktEydInIz", "~maktElErdI";
    pos_plain_cont_cond: "~maktEysEm", "~maktEysEn", "~maktEysE", "~maktEysEk", "~maktEysEnIz", "~maktElErsE";
    infinitive_pos: "~mak", _;
    converb_pos_plain: _, _, _, _, _, _, _, "~ya ~ya", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~yabile ~yabile", _;
    ),
    // Ilişkiyegirmek: 10 words (bahse girmek, transa girmek, vizyona girmek, gösterime girmek, araya girmek, yürürlüğe girmek). Like Çekmek, except in 3 cells: past_pos_plain__ _1sg, converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("ilişkiye girmek", 10, "~mek", like Çekmek;
    stem converb_neg_plain_doubled: "~mEyE gIrmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE gIrEmEyE- like vermek";
    past_pos_plain__: "~dIm", _, _, _, _, _;
    converb_pos_plain: _, _, _, _, _, _, _, "~E gIrE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile gIrEbile", _;
    ),
    // Ilerletmek: 7 words (sahneletmek, normalleşmek, mahkemeleşmek, halleşmek, helalleşmek, kristalleşmek). Like Silmek, except in 26 cells: pos_plain_aor__ _1sg, pos_plain_aor__ _2sg, pos_plain_aor__ _3sg, pos_plain_aor__ _2pl, pos_plain_aor__ _3pl, pos_plain_aor_evid _1sg, ….
    conjugation!("ilerletmek", 7, "~mek", like Silmek;
    pos_plain_aor__: "~IrIm", "~IrsIn", "~Ir", _, "~IrsInIz", "~IrlEr";
    pos_plain_aor_evid: "~IrmIşIm", "~IrmIşsIn", "~IrmIş", "~IrmIşIz", "~IrmIşsInIz", "~IrlErmIş";
    pos_plain_aor_past: "~IrdIm", "~IrdIn", "~IrdI", "~IrdIk", "~IrdInIz", "~IrlErdI";
    pos_plain_aor_cond: "~IrsEm", "~IrsEn", "~IrsE", "~IrsEk", "~IrsEnIz", "~IrlErsE";
    past_pos_plain__: "~tIm", _, _, _, _, _;
    converb_pos_plain: "~Irken", _, _, _, _, _, _, _, "~IrcEsInE";
    ),
    // Yemek: 6 words (demek, yemek, dayak yemek, kazık yemek, rüşvet yemek, aklını peynir ekmekle yemek).
    conjugation!("yemek", 6, "~emek";
    stem lemma: "~- like yemek";
    stem converb_neg_plain_doubled: "~emeye yemeye- like vermek";
    stem converb_neg_abil_doubled: "~iyemeye yiyemeye- like vermek";
    imperative_pos_abil: _, _, _, _, _, derived, _, _;
    imperative_neg_abil: _, _, _, _, _, derived, _, _;
    participle_pos_plain: _, derived, derived;
    participle_pos_abil: _, derived, derived;
    participle_neg_plain: _, _, derived;
    participle_neg_abil: _, derived, derived;
    relative_pos_plain_acak: derived, _, _, _, _, _;
    relative_pos_abil_acak: derived, _, _, _, _, _;
    relative_neg_plain_acak: derived, _, _, _, _, _;
    relative_neg_abil_acak: derived, _, _, _, _, _;
    ),
    // Filmçekmek: 5 words (otuz bir çekmek, iç çekmek, sineye çekmek, film çekmek, şınav çekmek). Like Silmek, except in 13 cells: imperative_pos_abil _2pl_formal, imperative_neg_abil _2pl_formal, participle_pos_plain miş, participle_pos_plain acak, participle_pos_abil miş, participle_pos_abil acak, ….
    conjugation!("film çekmek", 5, "~mek", like Silmek;
    stem lemma: "~- like film çekmek";
    stem converb_neg_plain_doubled: "~meye çekmeye- like vermek";
    stem converb_neg_abil_doubled: "~emeye çekemeye- like vermek";
    imperative_pos_abil: _, _, _, _, _, derived, _, _;
    imperative_neg_abil: _, _, _, _, _, derived, _, _;
    participle_pos_plain: _, derived, derived;
    participle_pos_abil: _, derived, derived;
    participle_neg_plain: _, _, derived;
    participle_neg_abil: _, derived, derived;
    relative_pos_plain_acak: derived, _, _, _, _, _;
    relative_pos_abil_acak: derived, _, _, _, _, _;
    relative_neg_plain_acak: derived, _, _, _, _, _;
    relative_neg_abil_acak: derived, _, _, _, _, _;
    ),
    // Vurmak: 5 words (olmak, vurmak, durmak, bulmak, bulundurmak). Like Nefesaldırmak, except in 25 cells: pos_abil_aor__ _1sg, pos_abil_aor__ _2sg, pos_abil_aor__ _3sg, pos_abil_aor__ _1pl, pos_abil_aor__ _2pl, pos_abil_aor__ _3pl, ….
    conjugation!("vurmak", 5, "~mak", like Nefesaldırmak;
    pos_abil_aor__: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_evid: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_past: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_cond: derived, derived, derived, derived, derived, derived;
    converb_pos_abil: "~Ebilurken", _, _, _, _, _, _, _, _;
    ),
    // Yolaçmak: 5 words (ateş açmak, yol açmak, iç açmak, kanat açmak, çiçek açmak). Like Açmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("yol açmak", 5, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE EçmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE EçEmEyE- like vermek";
    converb_pos_plain: _, _, _, _, _, _, _, "~E EçE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile açEbile", _;
    ),
    // Salmak: 4 words (kovmak, ovmak, savmak, başından savmak).
    conjugation!("salmak", 4, "~mak";
    stem converb_neg_plain_doubled: "~maya ~maya- like vermek";
    stem converb_neg_abil_doubled: "~amaya ~amaya- like vermek";
    pos_plain_aor__: "~arIm", "~arsIn", "~ar", _, "~arsInIz", "~arlEr";
    pos_plain_aor_evid: "~armIşIm", "~armIşsIn", "~armIş", "~armIşIz", "~armIşsInIz", "~arlErmIş";
    pos_plain_aor_past: "~ardIm", "~ardIn", "~ardI", "~ardIk", "~ardInIz", "~arlErdI";
    pos_plain_aor_cond: "~arsEm", "~arsEn", "~arsE", "~arsEk", "~arsEnIz", "~arlErsE";
    pos_plain_prog__: "~Iyorum", _, _, _, _, _;
    pos_plain_evid__: _, "~mIşsIn", "~mIş", _, "~mIşsInIz", "~mIşlEr";
    pos_plain_evid_evid: "~mIşmIşIm", "~mIşmIşsIn", "~mIşmIş", "~mIşmIşIz", "~mIşmIşsInIz", "~mIşlErmIş";
    pos_plain_evid_past: "~mIştIm", "~mIştIn", "~mIştI", "~mIştIk", "~mIştInIz", "~mIşlErdI";
    pos_plain_evid_cond: "~mIşsEm", "~mIşsEn", "~mIşsE", "~mIşsEk", "~mIşsEnIz", "~mIşlErsE";
    pos_plain_fut__: "~acEğIm", "~acEksIn", "~acEk", _, "~acEksInIz", "~acEklEr";
    pos_plain_fut_evid: "~acEkmIşIm", "~acEkmIşsIn", "~acEkmIş", "~acEkmIşIz", "~acEkmIşsInIz", "~acEklErmIş";
    pos_plain_fut_past: "~acEktIm", "~acEktIn", "~acEktI", "~acEktIk", "~acEktInIz", "~acEklErdI";
    pos_plain_fut_cond: "~acEksEm", "~acEksEn", "~acEksE", "~acEksEk", "~acEksEnIz", "~acEklErsE";
    pos_plain_cont__: "~maktEyIm", "~maktEsIn", "~maktE", "~maktEyIz", "~maktEsInIz", "~maktElEr";
    pos_plain_cont_evid: "~maktEymIşIm", "~maktEymIşsIn", "~maktEymIş", "~maktEymIşIz", "~maktEymIşsInIz", "~maktElErmIş";
    pos_plain_cont_past: "~maktEydIm", "~maktEydIn", "~maktEydI", "~maktEydIk", "~maktEydInIz", "~maktElErdI";
    pos_plain_cont_cond: "~maktEysEm", "~maktEysEn", "~maktEysE", "~maktEysEk", "~maktEysEnIz", "~maktElErsE";
    pos_plain_necess__: _, _, _, _, _, "~malIlEr";
    pos_plain_necess_evid: _, _, _, _, _, "~malIlErmIş";
    pos_plain_necess_past: _, _, _, _, _, "~malIlErdI";
    pos_plain_necess_cond: "~malIysEm", "~malIysEn", "~malIysE", "~malIysEk", "~malIysEnIz", "~malIlErsE";
    pos_abil_aor__: "~abilirim", "~abilirsin", "~abilir", "~abiliriz", "~abilirsiniz", "~abilirler";
    pos_abil_aor_evid: "~abilirmişim", "~abilirmişsin", "~abilirmiş", "~abilirmişiz", "~abilirmişsiniz", "~abilirlermiş";
    pos_abil_aor_past: "~abilirdim", "~abilirdin", "~abilirdi", "~abilirdik", "~abilirdiniz", "~abilirlerdi";
    pos_abil_aor_cond: "~abilirsem", "~abilirsen", "~abilirse", "~abilirsek", "~abilirseniz", "~abilirlerse";
    pos_abil_prog__: "~abiliyorum", "~abiliyorsun", "~abiliyor", "~abiliyoruz", "~abiliyorsunuz", "~abiliyorlar";
    pos_abil_prog_evid: "~abiliyormuşum", "~abiliyormuşsun", "~abiliyormuş", "~abiliyormuşuz", "~abiliyormuşsunuz", "~abiliyorlarmış";
    pos_abil_prog_past: "~abiliyordum", "~abiliyordun", "~abiliyordu", "~abiliyorduk", "~abiliyordunuz", "~abiliyorlardı";
    pos_abil_prog_cond: "~abiliyorsam", "~abiliyorsan", "~abiliyorsa", "~abiliyorsak", "~abiliyorsanız", "~abiliyorlarsa";
    pos_abil_evid__: "~abilmişim", "~abilmişsin", "~abilmiş", "~abilmişiz", "~abilmişsiniz", "~abilmişler";
    pos_abil_evid_evid: "~abilmişmişim", "~abilmişmişsin", "~abilmişmiş", "~abilmişmişiz", "~abilmişmişsiniz", "~abilmişlermiş";
    pos_abil_evid_past: "~abilmiştim", "~abilmiştin", "~abilmişti", "~abilmiştik", "~abilmiştiniz", "~abilmişlerdi";
    pos_abil_evid_cond: "~abilmişsem", "~abilmişsen", "~abilmişse", "~abilmişsek", "~abilmişseniz", "~abilmişlerse";
    pos_abil_fut__: "~abileceğim", "~abileceksin", "~abilecek", "~abileceğiz", "~abileceksiniz", "~abilecekler";
    pos_abil_fut_evid: "~abilecekmişim", "~abilecekmişsin", "~abilecekmiş", "~abilecekmişiz", "~abilecekmişsiniz", "~abileceklermiş";
    pos_abil_fut_past: "~abilecektim", "~abilecektin", "~abilecekti", "~abilecektik", "~abilecektiniz", "~abileceklerdi";
    pos_abil_fut_cond: "~abileceksem", "~abileceksen", "~abilecekse", "~abileceksek", "~abilecekseniz", "~abileceklerse";
    pos_abil_cont__: "~abilmekteyim", "~abilmektesin", "~abilmekte", "~abilmekteyiz", "~abilmektesiniz", "~abilmekteler";
    pos_abil_cont_evid: "~abilmekteymişim", "~abilmekteymişsin", "~abilmekteymiş", "~abilmekteymişiz", "~abilmekteymişsiniz", "~abilmektelermiş";
    pos_abil_cont_past: "~abilmekteydim", "~abilmekteydin", "~abilmekteydi", "~abilmekteydik", "~abilmekteydiniz", "~abilmektelerdi";
    pos_abil_cont_cond: "~abilmekteysem", "~abilmekteysen", "~abilmekteyse", "~abilmekteysek", "~abilmekteyseniz", "~abilmektelerse";
    pos_abil_necess__: "~abilmeliyim", "~abilmelisin", "~abilmeli", "~abilmeliyiz", "~abilmelisiniz", "~abilmeliler";
    pos_abil_necess_evid: "~abilmeliymişim", "~abilmeliymişsin", "~abilmeliymiş", "~abilmeliymişiz", "~abilmeliymişsiniz", "~abilmelilermiş";
    pos_abil_necess_past: "~abilmeliydim", "~abilmeliydin", "~abilmeliydi", "~abilmeliydik", "~abilmeliydiniz", "~abilmelilerdi";
    pos_abil_necess_cond: "~abilmeliysem", "~abilmeliysen", "~abilmeliyse", "~abilmeliysek", "~abilmeliyseniz", "~abilmelilerse";
    neg_plain_aor__: _, _, _, _, _, "~mazlEr";
    neg_plain_aor_evid: _, _, _, _, _, "~mazlErmIş";
    neg_plain_aor_past: _, _, _, _, _, "~mazlErdI";
    neg_plain_aor_cond: "~mazsEm", "~mazsEn", "~mazsE", "~mazsEk", "~mazsEnIz", "~mazlErsE";
    neg_plain_prog__: "~mIyorum", "~mIyorsun", "~mIyor", "~mIyoruz", "~mIyorsunuz", "~mIyorlar";
    neg_plain_prog_evid: "~mIyormuşum", "~mIyormuşsun", "~mIyormuş", "~mIyormuşuz", "~mIyormuşsunuz", "~mIyorlarmış";
    neg_plain_prog_past: "~mIyordum", "~mIyordun", "~mIyordu", "~mIyorduk", "~mIyordunuz", "~mIyorlardı";
    neg_plain_prog_cond: "~mIyorsam", "~mIyorsan", "~mIyorsa", "~mIyorsak", "~mIyorsanız", "~mIyorlarsa";
    neg_plain_evid__: _, _, _, _, _, "~mamIşlEr";
    neg_plain_evid_evid: _, _, _, _, _, "~mamIşlErmIş";
    neg_plain_evid_past: _, _, _, _, _, "~mamIşlErdI";
    neg_plain_evid_cond: "~mamIşsEm", "~mamIşsEn", "~mamIşsE", "~mamIşsEk", "~mamIşsEnIz", "~mamIşlErsE";
    neg_plain_fut__: "~mayEcEğIm", "~mayEcEksIn", "~mayEcEk", "~mayEcEğIz", "~mayEcEksInIz", "~mayEcEklEr";
    neg_plain_fut_evid: "~mayEcEkmIşIm", "~mayEcEkmIşsIn", "~mayEcEkmIş", "~mayEcEkmIşIz", "~mayEcEkmIşsInIz", "~mayEcEklErmIş";
    neg_plain_fut_past: "~mayEcEktIm", "~mayEcEktIn", "~mayEcEktI", "~mayEcEktIk", "~mayEcEktInIz", "~mayEcEklErdI";
    neg_plain_fut_cond: "~mayEcEksEm", "~mayEcEksEn", "~mayEcEksE", "~mayEcEksEk", "~mayEcEksEnIz", "~mayEcEklErsE";
    neg_plain_cont__: "~mamEktEyIm", "~mamEktEsIn", "~mamEktE", "~mamEktEyIz", "~mamEktEsInIz", "~mamEktElEr";
    neg_plain_cont_evid: "~mamEktEymIşIm", "~mamEktEymIşsIn", "~mamEktEymIş", "~mamEktEymIşIz", "~mamEktEymIşsInIz", "~mamEktElErmIş";
    neg_plain_cont_past: "~mamEktEydIm", "~mamEktEydIn", "~mamEktEydI", "~mamEktEydIk", "~mamEktEydInIz", "~mamEktElErdI";
    neg_plain_cont_cond: "~mamEktEysEm", "~mamEktEysEn", "~mamEktEysE", "~mamEktEysEk", "~mamEktEysEnIz", "~mamEktElErsE";
    neg_plain_necess__: "~mamElIyIm", "~mamElIsIn", "~mamElI", "~mamElIyIz", "~mamElIsInIz", "~mamElIlEr";
    neg_plain_necess_evid: "~mamElIymIşIm", "~mamElIymIşsIn", "~mamElIymIş", "~mamElIymIşIz", "~mamElIymIşsInIz", "~mamElIlErmIş";
    neg_plain_necess_past: "~mamElIydIm", "~mamElIydIn", "~mamElIydI", "~mamElIydIk", "~mamElIydInIz", "~mamElIlErdI";
    neg_plain_necess_cond: "~mamElIysEm", "~mamElIysEn", "~mamElIysE", "~mamElIysEk", "~mamElIysEnIz", "~mamElIlErsE";
    neg_abil_aor__: "~amEm", "~amEzsIn", "~amEz", "~amEyIz", "~amEzsInIz", "~amEzlEr";
    neg_abil_aor_evid: "~amEzmIşIm", "~amEzmIşsIn", "~amEzmIş", "~amEzmIşIz", "~amEzmIşsInIz", "~amEzlErmIş";
    neg_abil_aor_past: "~amEzdIm", "~amEzdIn", "~amEzdI", "~amEzdIk", "~amEzdInIz", "~amEzlErdI";
    neg_abil_aor_cond: "~amEzsEm", "~amEzsEn", "~amEzsE", "~amEzsEk", "~amEzsEnIz", "~amEzlErsE";
    neg_abil_prog__: "~amIyorum", "~amIyorsun", "~amIyor", "~amIyoruz", "~amIyorsunuz", "~amIyorlar";
    neg_abil_prog_evid: "~amIyormuşum", "~amIyormuşsun", "~amIyormuş", "~amIyormuşuz", "~amIyormuşsunuz", "~amIyorlarmış";
    neg_abil_prog_past: "~amIyordum", "~amIyordun", "~amIyordu", "~amIyorduk", "~amIyordunuz", "~amIyorlardı";
    neg_abil_prog_cond: "~amIyorsam", "~amIyorsan", "~amIyorsa", "~amIyorsak", "~amIyorsanız", "~amIyorlarsa";
    neg_abil_evid__: "~amEmIşIm", "~amEmIşsIn", "~amEmIş", "~amEmIşIz", "~amEmIşsInIz", "~amEmIşlEr";
    neg_abil_evid_evid: "~amEmIşmIşIm", "~amEmIşmIşsIn", "~amEmIşmIş", "~amEmIşmIşIz", "~amEmIşmIşsInIz", "~amEmIşlErmIş";
    neg_abil_evid_past: "~amEmIştIm", "~amEmIştIn", "~amEmIştI", "~amEmIştIk", "~amEmIştInIz", "~amEmIşlErdI";
    neg_abil_evid_cond: "~amEmIşsEm", "~amEmIşsEn", "~amEmIşsE", "~amEmIşsEk", "~amEmIşsEnIz", "~amEmIşlErsE";
    neg_abil_fut__: "~amEyEcEğIm", "~amEyEcEksIn", "~amEyEcEk", "~amEyEcEğIz", "~amEyEcEksInIz", "~amEyEcEklEr";
    neg_abil_fut_evid: "~amEyEcEkmIşIm", "~amEyEcEkmIşsIn", "~amEyEcEkmIş", "~amEyEcEkmIşIz", "~amEyEcEkmIşsInIz", "~amEyEcEklErmIş";
    neg_abil_fut_past: "~amEyEcEktIm", "~amEyEcEktIn", "~amEyEcEktI", "~amEyEcEktIk", "~amEyEcEktInIz", "~amEyEcEklErdI";
    neg_abil_fut_cond: "~amEyEcEksEm", "~amEyEcEksEn", "~amEyEcEksE", "~amEyEcEksEk", "~amEyEcEksEnIz", "~amEyEcEklErsE";
    neg_abil_cont__: "~amEmEktEyIm", "~amEmEktEsIn", "~amEmEktE", "~amEmEktEyIz", "~amEmEktEsInIz", "~amEmEktElEr";
    neg_abil_cont_evid: "~amEmEktEymIşIm", "~amEmEktEymIşsIn", "~amEmEktEymIş", "~amEmEktEymIşIz", "~amEmEktEymIşsInIz", "~amEmEktElErmIş";
    neg_abil_cont_past: "~amEmEktEydIm", "~amEmEktEydIn", "~amEmEktEydI", "~amEmEktEydIk", "~amEmEktEydInIz", "~amEmEktElErdI";
    neg_abil_cont_cond: "~amEmEktEysEm", "~amEmEktEysEn", "~amEmEktEysE", "~amEmEktEysEk", "~amEmEktEysEnIz", "~amEmEktElErsE";
    neg_abil_necess__: "~amEmElIyIm", "~amEmElIsIn", "~amEmElI", "~amEmElIyIz", "~amEmElIsInIz", "~amEmElIlEr";
    neg_abil_necess_evid: "~amEmElIymIşIm", "~amEmElIymIşsIn", "~amEmElIymIş", "~amEmElIymIşIz", "~amEmElIymIşsInIz", "~amEmElIlErmIş";
    neg_abil_necess_past: "~amEmElIydIm", "~amEmElIydIn", "~amEmElIydI", "~amEmElIydIk", "~amEmElIydInIz", "~amEmElIlErdI";
    neg_abil_necess_cond: "~amEmElIysEm", "~amEmElIysEn", "~amEmElIysE", "~amEmElIysEk", "~amEmElIysEnIz", "~amEmElIlErsE";
    past_pos_plain__: "~dIm", _, _, _, _, _;
    past_pos_abil__: "~abildim", "~abildin", "~abildi", "~abildik", "~abildiniz", "~abildiler";
    past_pos_abil_past: "~abildiydim", "~abildiydin", "~abildiydi", "~abildiydik", "~abildiydiniz", "~abildilerdi";
    past_pos_abil_cond: "~abildiysem", "~abildiysen", "~abildiyse", "~abildiysek", "~abildiyseniz", "~abildilerse";
    past_neg_plain__: _, _, _, _, _, "~madIlEr";
    past_neg_plain_past: _, _, _, _, _, "~madIlErdI";
    past_neg_plain_cond: "~madIysEm", "~madIysEn", "~madIysE", "~madIysEk", "~madIysEnIz", "~madIlErsE";
    past_neg_abil__: "~amEdIm", "~amEdIn", "~amEdI", "~amEdIk", "~amEdInIz", "~amEdIlEr";
    past_neg_abil_past: "~amEdIydIm", "~amEdIydIn", "~amEdIydI", "~amEdIydIk", "~amEdIydInIz", "~amEdIlErdI";
    past_neg_abil_cond: "~amEdIysEm", "~amEdIysEn", "~amEdIysE", "~amEdIysEk", "~amEdIysEnIz", "~amEdIlErsE";
    conditional_pos_plain__: "~sam", "~san", "~sa", "~sak", "~sanIz", "~salEr";
    conditional_pos_plain_evid: "~saymIşIm", "~saymIşsIn", "~saymIş", "~saymIşIz", "~saymIşsInIz", "~salErmIş";
    conditional_pos_plain_past: "~saydIm", "~saydIn", "~saydI", "~saydIk", "~saydInIz", "~salErdI";
    conditional_pos_abil__: "~abilsem", "~abilsen", "~abilse", "~abilsek", "~abilseniz", "~abilseler";
    conditional_pos_abil_evid: "~abilseymişim", "~abilseymişsin", "~abilseymiş", "~abilseymişiz", "~abilseymişsiniz", "~abilselermiş";
    conditional_pos_abil_past: "~abilseydim", "~abilseydin", "~abilseydi", "~abilseydik", "~abilseydiniz", "~abilselerdi";
    conditional_neg_plain__: "~masEm", "~masEn", "~masE", "~masEk", "~masEnIz", "~masElEr";
    conditional_neg_plain_evid: "~masEymIşIm", "~masEymIşsIn", "~masEymIş", "~masEymIşIz", "~masEymIşsInIz", "~masElErmIş";
    conditional_neg_plain_past: "~masEydIm", "~masEydIn", "~masEydI", "~masEydIk", "~masEydInIz", "~masElErdI";
    conditional_neg_abil__: "~amEsEm", "~amEsEn", "~amEsE", "~amEsEk", "~amEsEnIz", "~amEsElEr";
    conditional_neg_abil_evid: "~amEsEymIşIm", "~amEsEymIşsIn", "~amEsEymIş", "~amEsEymIşIz", "~amEsEymIşsInIz", "~amEsElErmIş";
    conditional_neg_abil_past: "~amEsEydIm", "~amEsEydIn", "~amEsEydI", "~amEsEydIk", "~amEsEydInIz", "~amEsElErdI";
    optative_pos_plain__: "~ayIm", "~asIn", "~a", "~alIm", "~asInIz", "~alEr";
    optative_pos_plain_evid: "~aymIşIm", "~aymIşsIn", "~aymIş", "~aymIşIz", "~aymIşsInIz", "~alErmIş";
    optative_pos_plain_past: "~aydIm", "~aydIn", "~aydI", "~aydIk", "~aydInIz", "~alErdI";
    optative_pos_abil__: "~abileyim", "~abilesin", "~abile", "~abilelim", "~abilesiniz", "~abileler";
    optative_pos_abil_evid: "~abileymişim", "~abileymişsin", "~abileymiş", "~abileymişiz", "~abileymişsiniz", "~abilelermiş";
    optative_pos_abil_past: "~abileydim", "~abileydin", "~abileydi", "~abileydik", "~abileydiniz", "~abilelerdi";
    optative_neg_plain__: "~mayEyIm", "~mayEsIn", "~mayE", "~mayElIm", "~mayEsInIz", "~mayElEr";
    optative_neg_plain_evid: "~mayEymIşIm", "~mayEymIşsIn", "~mayEymIş", "~mayEymIşIz", "~mayEymIşsInIz", "~mayElErmIş";
    optative_neg_plain_past: "~mayEydIm", "~mayEydIn", "~mayEydI", "~mayEydIk", "~mayEydInIz", "~mayElErdI";
    optative_neg_abil__: "~amEyEyIm", "~amEyEsIn", "~amEyE", "~amEyElIm", "~amEyEsInIz", "~amEyElEr";
    optative_neg_abil_evid: "~amEyEymIşIm", "~amEyEymIşsIn", "~amEyEymIş", "~amEyEymIşIz", "~amEyEymIşsInIz", "~amEyElErmIş";
    optative_neg_abil_past: "~amEyEydIm", "~amEyEydIn", "~amEyEydI", "~amEyEydIk", "~amEyEydInIz", "~amEyElErdI";
    imperative_pos_plain: "~", "~sIn", "~In", "~sInlEr", _, "~InIz", "~sanE", "~sanIzE";
    imperative_pos_abil: "~abil", "~abilsin", "~abilin", "~abilsinler", "~abiliniz", _, "~abilsene", "~abilsenize";
    imperative_neg_plain: _, _, _, "~masInlEr", _, "~mayInIz", "~masEnE", "~masEnIzE";
    imperative_neg_abil: "~amE", "~amEsIn", "~amEyIn", "~amEsInlEr", "~amEyInIz", _, "~amEsEnE", "~amEsEnIzE";
    infinitive_pos: "~mak", "~abilmek";
    infinitive_neg: "~mamEk", "~amEmEk";
    verbal_noun_pos: "~ma", "~abilme";
    verbal_noun_neg: "~mamE", "~amEmE";
    participle_pos_plain: "~an", _, _;
    participle_pos_abil: "~abilen", _, _;
    participle_neg_plain: "~mayEn", "~mamIş", _;
    participle_neg_abil: "~amEyEn", _, _;
    relative_pos_plain_acak: _, _, _, _, _, "~acEklErI";
    relative_pos_abil_dik: "~abildiğim", "~abildiğin", "~abildiği", "~abildiğimiz", "~abildiğiniz", "~abildikleri";
    relative_pos_abil_acak: _, "~abileceğin", "~abileceği", "~abileceğimiz", "~abileceğiniz", "~abilecekleri";
    relative_neg_plain_dik: _, _, _, _, _, "~madIklErI";
    relative_neg_plain_acak: _, "~mayEcEğIn", "~mayEcEğI", "~mayEcEğImIz", "~mayEcEğInIz", "~mayEcEklErI";
    relative_neg_abil_dik: "~amEdIğIm", "~amEdIğIn", "~amEdIğI", "~amEdIğImIz", "~amEdIğInIz", "~amEdIklErI";
    relative_neg_abil_acak: _, "~amEyEcEğIn", "~amEyEcEğI", "~amEyEcEğImIz", "~amEyEcEğInIz", "~amEyEcEklErI";
    converb_pos_plain: "~arken", _, _, "~alI", "~asIyE", _, "~arEk", "~a ~a", "~arcEsInE";
    converb_pos_abil: "~abilIrken", "~abilince", "~abilip", "~abileli", "~abilesiye", "~abildikçe", "~abilerek", "~abile ~abile", _;
    converb_neg_plain: _, "~mayIncE", _, "~mayElI", "~mayEsIyE", "~madIkçE", "~mayErEk", _, "~mazcEsInE";
    converb_neg_abil: "~amEzken", "~amEyIncE", "~amEyIp", "~amEyElI", "~amEyEsIyE", "~amEdIkçE", "~amEyErEk", _, "~amEzcEsInE";
    ),
    // Maytapgeçmek: 4 words (taşak geçmek, transa geçmek, maytap geçmek, tarihe geçmek). Like Çekmek, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("maytap geçmek", 4, "~mek", like Çekmek;
    stem converb_neg_plain_doubled: "~mEyE gEçmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE gEçEmEyE- like vermek";
    converb_pos_plain: _, _, _, _, _, _, _, "~E gEçE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile gEçEbile", _;
    ),
    // Kafatutmak: 4 words (kafa tutmak, oruç tutmak, kulak tutmak, gözü tutmak). Like Açmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("kafa tutmak", 4, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE tutmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE tutEmEyE- like vermek";
    converb_pos_plain: _, _, _, _, _, _, _, "~E tutE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile tutEbile", _;
    ),
    // Gömmek: 3 words (bükmek, ölçmek, gömmek). Like Çekmek, except in 25 cells: pos_plain_aor__ _1sg, pos_plain_aor__ _2sg, pos_plain_aor__ _3sg, pos_plain_aor__ _2pl, pos_plain_aor__ _3pl, pos_plain_aor_evid _1sg, ….
    conjugation!("gömmek", 3, "~mek", like Çekmek;
    pos_plain_aor__: "~irIm", "~irsIn", "~ir", _, "~irsInIz", "~irlEr";
    pos_plain_aor_evid: "~irmIşIm", "~irmIşsIn", "~irmIş", "~irmIşIz", "~irmIşsInIz", "~irlErmIş";
    pos_plain_aor_past: "~irdIm", "~irdIn", "~irdI", "~irdIk", "~irdInIz", "~irlErdI";
    pos_plain_aor_cond: "~irsEm", "~irsEn", "~irsE", "~irsEk", "~irsEnIz", "~irlErsE";
    converb_pos_plain: "~irken", _, _, _, _, _, _, _, "~ircEsInE";
    ),
    // Geçkalmak: 3 words (geç kalmak, mutabık kalmak, ramak kalmak). Like Vurmak, except in 3 cells: converb_pos_plain doubled, converb_pos_abil ken, converb_pos_abil doubled.
    conjugation!("geç kalmak", 3, "~mak", like Vurmak;
    stem converb_neg_plain_doubled: "~mEyE kElmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE kElEmEyE- like vermek";
    converb_pos_plain: _, _, _, _, _, _, _, "~E kElE", _;
    converb_pos_abil: "~Ebilırken", _, _, _, _, _, _, "~Ebile kalEbile", _;
    ),
    // Serbestbırakmak: 3 words (silah bırakmak, tadında bırakmak, serbest bırakmak). Like Varolmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("serbest bırakmak", 3, "~mak", like Varolmak;
    stem converb_neg_plain_doubled: "~mEyE bırEkmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE bırEkEmEyE- like vermek";
    converb_pos_plain: _, _, _, _, _, _, _, "~E bırEkE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile bırEkEbile", _;
    ),
    // Görmek: 3 words (görmek, mazur görmek, görüm görmek). Like Vermek, except in 27 cells: pos_abil_aor__ _1sg, pos_abil_aor__ _2sg, pos_abil_aor__ _3sg, pos_abil_aor__ _1pl, pos_abil_aor__ _2pl, pos_abil_aor__ _3pl, ….
    conjugation!("görmek", 3, "~mek", like Vermek;
    stem converb_neg_plain_doubled: "~mEyE görmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE görEmEyE- like vermek";
    pos_abil_aor__: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_evid: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_past: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_cond: derived, derived, derived, derived, derived, derived;
    converb_pos_plain: _, _, _, _, _, _, _, "~E görE", _;
    converb_pos_abil: "~Ebilürken", _, _, _, _, _, _, "~Ebile görEbile", _;
    ),
    // Götügitmek: 3 words (götü gitmek, hoşuna gitmek, geberip gitmek). Like Takipetmek, except in 210 cells: pos_plain_cont__ _1sg, pos_plain_cont__ _2sg, pos_plain_cont__ _3sg, pos_plain_cont__ _1pl, pos_plain_cont__ _2pl, pos_plain_cont__ _3pl, ….
    conjugation!("götü gitmek", 3, "~tmek", like Takipetmek;
    stem converb_neg_plain_doubled: "~tmeye gitmeye- like vermek";
    stem converb_neg_abil_doubled: "~demeye gidemeye- like vermek";
    pos_plain_cont__: "~tmektEyIm", "~tmektEsIn", "~tmektE", "~tmektEyIz", "~tmektEsInIz", "~tmektElEr";
    pos_plain_cont_evid: "~tmektEymIşIm", "~tmektEymIşsIn", "~tmektEymIş", "~tmektEymIşIz", "~tmektEymIşsInIz", "~tmektElErmIş";
    pos_plain_cont_past: "~tmektEydIm", "~tmektEydIn", "~tmektEydI", "~tmektEydIk", "~tmektEydInIz", "~tmektElErdI";
    pos_plain_cont_cond: "~tmektEysEm", "~tmektEysEn", "~tmektEysE", "~tmektEysEk", "~tmektEysEnIz", "~tmektElErsE";
    pos_plain_necess__: _, _, _, _, _, "~tmElIlEr";
    pos_plain_necess_evid: _, _, _, _, _, "~tmElIlErmIş";
    pos_plain_necess_past: _, _, _, _, _, "~tmElIlErdI";
    pos_plain_necess_cond: "~tmElIysEm", "~tmElIysEn", "~tmElIysE", "~tmElIysEk", "~tmElIysEnIz", "~tmElIlErsE";
    neg_plain_aor__: _, _, _, _, _, "~tmEzlEr";
    neg_plain_aor_evid: _, _, _, _, _, "~tmEzlErmIş";
    neg_plain_aor_past: _, _, _, _, _, "~tmEzlErdI";
    neg_plain_aor_cond: "~tmEzsEm", "~tmEzsEn", "~tmEzsE", "~tmEzsEk", "~tmEzsEnIz", "~tmEzlErsE";
    neg_plain_evid__: _, _, _, _, _, "~tmEmIşlEr";
    neg_plain_evid_evid: _, _, _, _, _, "~tmEmIşlErmIş";
    neg_plain_evid_past: _, _, _, _, _, "~tmEmIşlErdI";
    neg_plain_evid_cond: "~tmEmIşsEm", "~tmEmIşsEn", "~tmEmIşsE", "~tmEmIşsEk", "~tmEmIşsEnIz", "~tmEmIşlErsE";
    neg_plain_fut__: "~tmEyEcEğIm", "~tmEyEcEksIn", "~tmEyEcEk", "~tmEyEcEğIz", "~tmEyEcEksInIz", "~tmEyEcEklEr";
    neg_plain_fut_evid: "~tmEyEcEkmIşIm", "~tmEyEcEkmIşsIn", "~tmEyEcEkmIş", "~tmEyEcEkmIşIz", "~tmEyEcEkmIşsInIz", "~tmEyEcEklErmIş";
    neg_plain_fut_past: "~tmEyEcEktIm", "~tmEyEcEktIn", "~tmEyEcEktI", "~tmEyEcEktIk", "~tmEyEcEktInIz", "~tmEyEcEklErdI";
    neg_plain_fut_cond: "~tmEyEcEksEm", "~tmEyEcEksEn", "~tmEyEcEksE", "~tmEyEcEksEk", "~tmEyEcEksEnIz", "~tmEyEcEklErsE";
    neg_plain_cont__: "~tmEmEktEyIm", "~tmEmEktEsIn", "~tmEmEktE", "~tmEmEktEyIz", "~tmEmEktEsInIz", "~tmEmEktElEr";
    neg_plain_cont_evid: "~tmEmEktEymIşIm", "~tmEmEktEymIşsIn", "~tmEmEktEymIş", "~tmEmEktEymIşIz", "~tmEmEktEymIşsInIz", "~tmEmEktElErmIş";
    neg_plain_cont_past: "~tmEmEktEydIm", "~tmEmEktEydIn", "~tmEmEktEydI", "~tmEmEktEydIk", "~tmEmEktEydInIz", "~tmEmEktElErdI";
    neg_plain_cont_cond: "~tmEmEktEysEm", "~tmEmEktEysEn", "~tmEmEktEysE", "~tmEmEktEysEk", "~tmEmEktEysEnIz", "~tmEmEktElErsE";
    neg_plain_necess__: "~tmEmElIyIm", "~tmEmElIsIn", "~tmEmElI", "~tmEmElIyIz", "~tmEmElIsInIz", "~tmEmElIlEr";
    neg_plain_necess_evid: "~tmEmElIymIşIm", "~tmEmElIymIşsIn", "~tmEmElIymIş", "~tmEmElIymIşIz", "~tmEmElIymIşsInIz", "~tmEmElIlErmIş";
    neg_plain_necess_past: "~tmEmElIydIm", "~tmEmElIydIn", "~tmEmElIydI", "~tmEmElIydIk", "~tmEmElIydInIz", "~tmEmElIlErdI";
    neg_plain_necess_cond: "~tmEmElIysEm", "~tmEmElIysEn", "~tmEmElIysE", "~tmEmElIysEk", "~tmEmElIysEnIz", "~tmEmElIlErsE";
    past_neg_plain__: _, _, _, _, _, "~tmEdIlEr";
    past_neg_plain_past: _, _, _, _, _, "~tmEdIlErdI";
    past_neg_plain_cond: "~tmEdIysEm", "~tmEdIysEn", "~tmEdIysE", "~tmEdIysEk", "~tmEdIysEnIz", "~tmEdIlErsE";
    conditional_pos_plain__: "~tsEm", "~tsEn", "~tsE", "~tsEk", "~tsEnIz", "~tsElEr";
    conditional_pos_plain_evid: "~tsEymIşIm", "~tsEymIşsIn", "~tsEymIş", "~tsEymIşIz", "~tsEymIşsInIz", "~tsElErmIş";
    conditional_pos_plain_past: "~tsEydIm", "~tsEydIn", "~tsEydI", "~tsEydIk", "~tsEydInIz", "~tsElErdI";
    conditional_neg_plain__: "~tmEsEm", "~tmEsEn", "~tmEsE", "~tmEsEk", "~tmEsEnIz", "~tmEsElEr";
    conditional_neg_plain_evid: "~tmEsEymIşIm", "~tmEsEymIşsIn", "~tmEsEymIş", "~tmEsEymIşIz", "~tmEsEymIşsInIz", "~tmEsElErmIş";
    conditional_neg_plain_past: "~tmEsEydIm", "~tmEsEydIn", "~tmEsEydI", "~tmEsEydIk", "~tmEsEydInIz", "~tmEsElErdI";
    optative_neg_plain__: "~tmEyEyIm", "~tmEyEsIn", "~tmEyE", "~tmEyElIm", "~tmEyEsInIz", "~tmEyElEr";
    optative_neg_plain_evid: "~tmEyEymIşIm", "~tmEyEymIşsIn", "~tmEyEymIş", "~tmEyEymIşIz", "~tmEyEymIşsInIz", "~tmEyElErmIş";
    optative_neg_plain_past: "~tmEyEydIm", "~tmEyEydIn", "~tmEyEydI", "~tmEyEydIk", "~tmEyEydInIz", "~tmEyElErdI";
    imperative_pos_plain: _, _, _, _, _, _, "~tsEnE", "~tsEnIzE";
    imperative_neg_plain: _, _, _, "~tmEsInlEr", _, "~tmEyInIz", "~tmEsEnE", "~tmEsEnIzE";
    infinitive_neg: "~tmEmEk", _;
    verbal_noun_pos: "~tmE", _;
    verbal_noun_neg: "~tmEmE", _;
    participle_neg_plain: "~tmEyEn", "~tmEmIş", _;
    relative_neg_plain_dik: _, _, _, _, _, "~tmEdIklErI";
    relative_neg_plain_acak: _, "~tmEyEcEğIn", "~tmEyEcEğI", "~tmEyEcEğImIz", "~tmEyEcEğInIz", "~tmEyEcEklErI";
    converb_pos_plain: _, _, _, _, _, _, _, "~de gide", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~debile gidebile", _;
    converb_neg_plain: _, "~tmEyIncE", _, "~tmEyElI", "~tmEyEsIyE", "~tmEdIkçE", "~tmEyErEk", _, "~tmEzcEsInE";
    ),
    // Büyükoynamak: 3 words (büyük oynamak, bahis oynamak, kumar oynamak).
    conjugation!("büyük oynamak", 3, "~amak";
    stem lemma: "~- like büyük oynamak";
    stem converb_neg_plain_doubled: "~amaya oynamaya- like vermek";
    stem converb_neg_abil_doubled: "~ayamaya oynayamaya- like vermek";
    imperative_pos_abil: _, _, _, _, _, derived, _, _;
    imperative_neg_abil: _, _, _, _, _, derived, _, _;
    participle_pos_plain: _, derived, derived;
    participle_pos_abil: _, derived, derived;
    participle_neg_plain: _, _, derived;
    participle_neg_abil: _, derived, derived;
    relative_pos_plain_acak: derived, _, _, _, _, _;
    relative_pos_abil_acak: derived, _, _, _, _, _;
    relative_neg_plain_acak: derived, _, _, _, _, _;
    relative_neg_abil_acak: derived, _, _, _, _, _;
    ),
    // Layığınıbulmak: 3 words (teselli bulmak, bokunda boncuk bulmak, layığını bulmak). Like Vurmak, except in 2 cells: converb_pos_plain doubled, converb_pos_abil doubled.
    conjugation!("layığını bulmak", 3, "~mak", like Vurmak;
    stem converb_neg_plain_doubled: "~mEyE bulmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE bulEmEyE- like vermek";
    converb_pos_plain: _, _, _, _, _, _, _, "~E bulE", _;
    converb_pos_abil: _, _, _, _, _, _, _, "~Ebile bulEbile", _;
    ),
    // Seskasmak: 3 words (ses kasmak, amuda kalkmak, duyar kasmak). Like Açmak, except in 27 cells: pos_abil_aor__ _1sg, pos_abil_aor__ _2sg, pos_abil_aor__ _3sg, pos_abil_aor__ _1pl, pos_abil_aor__ _2pl, pos_abil_aor__ _3pl, ….
    conjugation!("ses kasmak", 3, "~mak", like Açmak;
    stem converb_neg_plain_doubled: "~mEyE kEsmEyE- like vermek";
    stem converb_neg_abil_doubled: "~EmEyE kEsEmEyE- like vermek";
    pos_abil_aor__: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_evid: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_past: derived, derived, derived, derived, derived, derived;
    pos_abil_aor_cond: derived, derived, derived, derived, derived, derived;
    converb_pos_plain: _, _, _, _, _, _, _, "~E kEsE", _;
    converb_pos_abil: "~Ebilarken", _, _, _, _, _, _, "~Ebile kasEbile", _;
    ),
];
