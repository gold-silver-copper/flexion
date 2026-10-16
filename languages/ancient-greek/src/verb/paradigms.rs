//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes (`~ω`, `μελλω`: the stem and `ω`). A stem in two pieces (`~~πω`, `πεμπω`) has
//! letters between them that other forms may drop or change.
//! Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~ημένοι ὦσῐ|%ε~ημένοι ὦσῐν`, of
//! `ποιεω`), as the copy laws in `laws.rs` change it. A `ˇ` ends
//! the patterns of a conjugation whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~ωˇ`: `σκᾰνδᾰλῐζω`).

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (if they share them),
/// the conjugation it is like (if any), the stem slots it states with a stem and an
/// ending table
/// (`stem lemma: \"~- like ἐχω\"`: the slot's cells are `ἐχω`'s table's patterns
/// with `~` for their `~`) (see `stems.rs`), then a row of patterns per line
/// (`derived`: the form comes from another cell's, see `relations.rs`; `_`,
/// or a row left out: the form is its stem slot's, or the conjugation's it is
/// like, or, in a conjugation like no other, derived):
/// `ind_act_1` (sg, du, pl)
/// `ind_act_2` (sg, du, pl)
/// `ind_act_3` (sg, du, pl)
/// `ind_mid_1` (sg, du, pl)
/// `ind_mid_2` (sg, du, pl)
/// `ind_mid_3` (sg, du, pl)
/// `ind_pass_1` (sg, du, pl)
/// `ind_pass_2` (sg, du, pl)
/// `ind_pass_3` (sg, du, pl)
/// `subj_act_1` (sg, du, pl)
/// `subj_act_2` (sg, du, pl)
/// `subj_act_3` (sg, du, pl)
/// `subj_mid_1` (sg, du, pl)
/// `subj_mid_2` (sg, du, pl)
/// `subj_mid_3` (sg, du, pl)
/// `subj_pass_1` (sg, du, pl)
/// `subj_pass_2` (sg, du, pl)
/// `subj_pass_3` (sg, du, pl)
/// `opt_act_1` (sg, du, pl)
/// `opt_act_2` (sg, du, pl)
/// `opt_act_3` (sg, du, pl)
/// `opt_mid_1` (sg, du, pl)
/// `opt_mid_2` (sg, du, pl)
/// `opt_mid_3` (sg, du, pl)
/// `opt_pass_1` (sg, du, pl)
/// `opt_pass_2` (sg, du, pl)
/// `opt_pass_3` (sg, du, pl)
/// `imp_act_1` (sg, du, pl)
/// `imp_act_2` (sg, du, pl)
/// `imp_act_3` (sg, du, pl)
/// `imp_mid_1` (sg, du, pl)
/// `imp_mid_2` (sg, du, pl)
/// `imp_mid_3` (sg, du, pl)
/// `imp_pass_1` (sg, du, pl)
/// `imp_pass_2` (sg, du, pl)
/// `imp_pass_3` (sg, du, pl)
/// `imperfect_act_1` (sg, du, pl)
/// `imperfect_act_2` (sg, du, pl)
/// `imperfect_act_3` (sg, du, pl)
/// `imperfect_mid_1` (sg, du, pl)
/// `imperfect_mid_2` (sg, du, pl)
/// `imperfect_mid_3` (sg, du, pl)
/// `imperfect_pass_1` (sg, du, pl)
/// `imperfect_pass_2` (sg, du, pl)
/// `imperfect_pass_3` (sg, du, pl)
/// `future_ind_act_1` (sg, du, pl)
/// `future_ind_act_2` (sg, du, pl)
/// `future_ind_act_3` (sg, du, pl)
/// `future_ind_mid_1` (sg, du, pl)
/// `future_ind_mid_2` (sg, du, pl)
/// `future_ind_mid_3` (sg, du, pl)
/// `future_ind_pass_1` (sg, du, pl)
/// `future_ind_pass_2` (sg, du, pl)
/// `future_ind_pass_3` (sg, du, pl)
/// `future_subj_act_1` (sg, du, pl)
/// `future_subj_act_2` (sg, du, pl)
/// `future_subj_act_3` (sg, du, pl)
/// `future_subj_mid_1` (sg, du, pl)
/// `future_subj_mid_2` (sg, du, pl)
/// `future_subj_mid_3` (sg, du, pl)
/// `future_subj_pass_1` (sg, du, pl)
/// `future_subj_pass_2` (sg, du, pl)
/// `future_subj_pass_3` (sg, du, pl)
/// `future_opt_act_1` (sg, du, pl)
/// `future_opt_act_2` (sg, du, pl)
/// `future_opt_act_3` (sg, du, pl)
/// `future_opt_mid_1` (sg, du, pl)
/// `future_opt_mid_2` (sg, du, pl)
/// `future_opt_mid_3` (sg, du, pl)
/// `future_opt_pass_1` (sg, du, pl)
/// `future_opt_pass_2` (sg, du, pl)
/// `future_opt_pass_3` (sg, du, pl)
/// `future_imp_act_1` (sg, du, pl)
/// `future_imp_act_2` (sg, du, pl)
/// `future_imp_act_3` (sg, du, pl)
/// `future_imp_mid_1` (sg, du, pl)
/// `future_imp_mid_2` (sg, du, pl)
/// `future_imp_mid_3` (sg, du, pl)
/// `future_imp_pass_1` (sg, du, pl)
/// `future_imp_pass_2` (sg, du, pl)
/// `future_imp_pass_3` (sg, du, pl)
/// `aorist_ind_act_1` (sg, du, pl)
/// `aorist_ind_act_2` (sg, du, pl)
/// `aorist_ind_act_3` (sg, du, pl)
/// `aorist_ind_mid_1` (sg, du, pl)
/// `aorist_ind_mid_2` (sg, du, pl)
/// `aorist_ind_mid_3` (sg, du, pl)
/// `aorist_ind_pass_1` (sg, du, pl)
/// `aorist_ind_pass_2` (sg, du, pl)
/// `aorist_ind_pass_3` (sg, du, pl)
/// `aorist_subj_act_1` (sg, du, pl)
/// `aorist_subj_act_2` (sg, du, pl)
/// `aorist_subj_act_3` (sg, du, pl)
/// `aorist_subj_mid_1` (sg, du, pl)
/// `aorist_subj_mid_2` (sg, du, pl)
/// `aorist_subj_mid_3` (sg, du, pl)
/// `aorist_subj_pass_1` (sg, du, pl)
/// `aorist_subj_pass_2` (sg, du, pl)
/// `aorist_subj_pass_3` (sg, du, pl)
/// `aorist_opt_act_1` (sg, du, pl)
/// `aorist_opt_act_2` (sg, du, pl)
/// `aorist_opt_act_3` (sg, du, pl)
/// `aorist_opt_mid_1` (sg, du, pl)
/// `aorist_opt_mid_2` (sg, du, pl)
/// `aorist_opt_mid_3` (sg, du, pl)
/// `aorist_opt_pass_1` (sg, du, pl)
/// `aorist_opt_pass_2` (sg, du, pl)
/// `aorist_opt_pass_3` (sg, du, pl)
/// `aorist_imp_act_1` (sg, du, pl)
/// `aorist_imp_act_2` (sg, du, pl)
/// `aorist_imp_act_3` (sg, du, pl)
/// `aorist_imp_mid_1` (sg, du, pl)
/// `aorist_imp_mid_2` (sg, du, pl)
/// `aorist_imp_mid_3` (sg, du, pl)
/// `aorist_imp_pass_1` (sg, du, pl)
/// `aorist_imp_pass_2` (sg, du, pl)
/// `aorist_imp_pass_3` (sg, du, pl)
/// `perfect_ind_act_1` (sg, du, pl)
/// `perfect_ind_act_2` (sg, du, pl)
/// `perfect_ind_act_3` (sg, du, pl)
/// `perfect_ind_mid_1` (sg, du, pl)
/// `perfect_ind_mid_2` (sg, du, pl)
/// `perfect_ind_mid_3` (sg, du, pl)
/// `perfect_ind_pass_1` (sg, du, pl)
/// `perfect_ind_pass_2` (sg, du, pl)
/// `perfect_ind_pass_3` (sg, du, pl)
/// `perfect_subj_act_1` (sg, du, pl)
/// `perfect_subj_act_2` (sg, du, pl)
/// `perfect_subj_act_3` (sg, du, pl)
/// `perfect_subj_mid_1` (sg, du, pl)
/// `perfect_subj_mid_2` (sg, du, pl)
/// `perfect_subj_mid_3` (sg, du, pl)
/// `perfect_subj_pass_1` (sg, du, pl)
/// `perfect_subj_pass_2` (sg, du, pl)
/// `perfect_subj_pass_3` (sg, du, pl)
/// `perfect_opt_act_1` (sg, du, pl)
/// `perfect_opt_act_2` (sg, du, pl)
/// `perfect_opt_act_3` (sg, du, pl)
/// `perfect_opt_mid_1` (sg, du, pl)
/// `perfect_opt_mid_2` (sg, du, pl)
/// `perfect_opt_mid_3` (sg, du, pl)
/// `perfect_opt_pass_1` (sg, du, pl)
/// `perfect_opt_pass_2` (sg, du, pl)
/// `perfect_opt_pass_3` (sg, du, pl)
/// `perfect_imp_act_1` (sg, du, pl)
/// `perfect_imp_act_2` (sg, du, pl)
/// `perfect_imp_act_3` (sg, du, pl)
/// `perfect_imp_mid_1` (sg, du, pl)
/// `perfect_imp_mid_2` (sg, du, pl)
/// `perfect_imp_mid_3` (sg, du, pl)
/// `perfect_imp_pass_1` (sg, du, pl)
/// `perfect_imp_pass_2` (sg, du, pl)
/// `perfect_imp_pass_3` (sg, du, pl)
/// `pluperfect_act_1` (sg, du, pl)
/// `pluperfect_act_2` (sg, du, pl)
/// `pluperfect_act_3` (sg, du, pl)
/// `pluperfect_mid_1` (sg, du, pl)
/// `pluperfect_mid_2` (sg, du, pl)
/// `pluperfect_mid_3` (sg, du, pl)
/// `pluperfect_pass_1` (sg, du, pl)
/// `pluperfect_pass_2` (sg, du, pl)
/// `pluperfect_pass_3` (sg, du, pl)
/// `future_perfect_ind_act_1` (sg, du, pl)
/// `future_perfect_ind_act_2` (sg, du, pl)
/// `future_perfect_ind_act_3` (sg, du, pl)
/// `future_perfect_ind_mid_1` (sg, du, pl)
/// `future_perfect_ind_mid_2` (sg, du, pl)
/// `future_perfect_ind_mid_3` (sg, du, pl)
/// `future_perfect_ind_pass_1` (sg, du, pl)
/// `future_perfect_ind_pass_2` (sg, du, pl)
/// `future_perfect_ind_pass_3` (sg, du, pl)
/// `future_perfect_subj_act_1` (sg, du, pl)
/// `future_perfect_subj_act_2` (sg, du, pl)
/// `future_perfect_subj_act_3` (sg, du, pl)
/// `future_perfect_subj_mid_1` (sg, du, pl)
/// `future_perfect_subj_mid_2` (sg, du, pl)
/// `future_perfect_subj_mid_3` (sg, du, pl)
/// `future_perfect_subj_pass_1` (sg, du, pl)
/// `future_perfect_subj_pass_2` (sg, du, pl)
/// `future_perfect_subj_pass_3` (sg, du, pl)
/// `future_perfect_opt_act_1` (sg, du, pl)
/// `future_perfect_opt_act_2` (sg, du, pl)
/// `future_perfect_opt_act_3` (sg, du, pl)
/// `future_perfect_opt_mid_1` (sg, du, pl)
/// `future_perfect_opt_mid_2` (sg, du, pl)
/// `future_perfect_opt_mid_3` (sg, du, pl)
/// `future_perfect_opt_pass_1` (sg, du, pl)
/// `future_perfect_opt_pass_2` (sg, du, pl)
/// `future_perfect_opt_pass_3` (sg, du, pl)
/// `future_perfect_imp_act_1` (sg, du, pl)
/// `future_perfect_imp_act_2` (sg, du, pl)
/// `future_perfect_imp_act_3` (sg, du, pl)
/// `future_perfect_imp_mid_1` (sg, du, pl)
/// `future_perfect_imp_mid_2` (sg, du, pl)
/// `future_perfect_imp_mid_3` (sg, du, pl)
/// `future_perfect_imp_pass_1` (sg, du, pl)
/// `future_perfect_imp_pass_2` (sg, du, pl)
/// `future_perfect_imp_pass_3` (sg, du, pl)
/// `infinitive_pres` (act, mid, pass)
/// `infinitive_fut` (act, mid, pass)
/// `infinitive_aor` (act, mid, pass)
/// `infinitive_perf` (act, mid, pass)
/// `infinitive_futp` (act, mid, pass)
/// `participle_pres` (act, mid, pass)
/// `participle_fut` (act, mid, pass)
/// `participle_aor` (act, mid, pass)
/// `participle_perf` (act, mid, pass)
/// `participle_futp` (act, mid, pass)
macro_rules! conjugation {
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)? $(with $affix:literal $(, $other:literal after $letters:literal)?)?)?;
     $(stem lemma: $s0:tt;)?
     $(stem rest____: $s1:tt;)?
     $(stem rest__: $s2:tt;)?
     $(stem rest_______: $s3:tt;)?
     $(stem rest________: $s4:tt;)?
     $(stem rest__________: $s5:tt;)?
     $(stem rest_: $s6:tt;)?
     $(stem rest_____: $s7:tt;)?
     $(stem rest___: $s8:tt;)?
     $(stem rest: $s9:tt;)?
     $(stem rest______: $s10:tt;)?
     $(stem rest___________: $s11:tt;)?
     $(stem rest_________: $s12:tt;)?
     $(ind_act_1: $c0:tt, $c1:tt, $c2:tt;)?
     $(ind_act_2: $c3:tt, $c4:tt, $c5:tt;)?
     $(ind_act_3: $c6:tt, $c7:tt, $c8:tt;)?
     $(ind_mid_1: $c9:tt, $c10:tt, $c11:tt;)?
     $(ind_mid_2: $c12:tt, $c13:tt, $c14:tt;)?
     $(ind_mid_3: $c15:tt, $c16:tt, $c17:tt;)?
     $(ind_pass_1: $c18:tt, $c19:tt, $c20:tt;)?
     $(ind_pass_2: $c21:tt, $c22:tt, $c23:tt;)?
     $(ind_pass_3: $c24:tt, $c25:tt, $c26:tt;)?
     $(subj_act_1: $c27:tt, $c28:tt, $c29:tt;)?
     $(subj_act_2: $c30:tt, $c31:tt, $c32:tt;)?
     $(subj_act_3: $c33:tt, $c34:tt, $c35:tt;)?
     $(subj_mid_1: $c36:tt, $c37:tt, $c38:tt;)?
     $(subj_mid_2: $c39:tt, $c40:tt, $c41:tt;)?
     $(subj_mid_3: $c42:tt, $c43:tt, $c44:tt;)?
     $(subj_pass_1: $c45:tt, $c46:tt, $c47:tt;)?
     $(subj_pass_2: $c48:tt, $c49:tt, $c50:tt;)?
     $(subj_pass_3: $c51:tt, $c52:tt, $c53:tt;)?
     $(opt_act_1: $c54:tt, $c55:tt, $c56:tt;)?
     $(opt_act_2: $c57:tt, $c58:tt, $c59:tt;)?
     $(opt_act_3: $c60:tt, $c61:tt, $c62:tt;)?
     $(opt_mid_1: $c63:tt, $c64:tt, $c65:tt;)?
     $(opt_mid_2: $c66:tt, $c67:tt, $c68:tt;)?
     $(opt_mid_3: $c69:tt, $c70:tt, $c71:tt;)?
     $(opt_pass_1: $c72:tt, $c73:tt, $c74:tt;)?
     $(opt_pass_2: $c75:tt, $c76:tt, $c77:tt;)?
     $(opt_pass_3: $c78:tt, $c79:tt, $c80:tt;)?
     $(imp_act_1: $c81:tt, $c82:tt, $c83:tt;)?
     $(imp_act_2: $c84:tt, $c85:tt, $c86:tt;)?
     $(imp_act_3: $c87:tt, $c88:tt, $c89:tt;)?
     $(imp_mid_1: $c90:tt, $c91:tt, $c92:tt;)?
     $(imp_mid_2: $c93:tt, $c94:tt, $c95:tt;)?
     $(imp_mid_3: $c96:tt, $c97:tt, $c98:tt;)?
     $(imp_pass_1: $c99:tt, $c100:tt, $c101:tt;)?
     $(imp_pass_2: $c102:tt, $c103:tt, $c104:tt;)?
     $(imp_pass_3: $c105:tt, $c106:tt, $c107:tt;)?
     $(imperfect_act_1: $c108:tt, $c109:tt, $c110:tt;)?
     $(imperfect_act_2: $c111:tt, $c112:tt, $c113:tt;)?
     $(imperfect_act_3: $c114:tt, $c115:tt, $c116:tt;)?
     $(imperfect_mid_1: $c117:tt, $c118:tt, $c119:tt;)?
     $(imperfect_mid_2: $c120:tt, $c121:tt, $c122:tt;)?
     $(imperfect_mid_3: $c123:tt, $c124:tt, $c125:tt;)?
     $(imperfect_pass_1: $c126:tt, $c127:tt, $c128:tt;)?
     $(imperfect_pass_2: $c129:tt, $c130:tt, $c131:tt;)?
     $(imperfect_pass_3: $c132:tt, $c133:tt, $c134:tt;)?
     $(future_ind_act_1: $c135:tt, $c136:tt, $c137:tt;)?
     $(future_ind_act_2: $c138:tt, $c139:tt, $c140:tt;)?
     $(future_ind_act_3: $c141:tt, $c142:tt, $c143:tt;)?
     $(future_ind_mid_1: $c144:tt, $c145:tt, $c146:tt;)?
     $(future_ind_mid_2: $c147:tt, $c148:tt, $c149:tt;)?
     $(future_ind_mid_3: $c150:tt, $c151:tt, $c152:tt;)?
     $(future_ind_pass_1: $c153:tt, $c154:tt, $c155:tt;)?
     $(future_ind_pass_2: $c156:tt, $c157:tt, $c158:tt;)?
     $(future_ind_pass_3: $c159:tt, $c160:tt, $c161:tt;)?
     $(future_subj_act_1: $c162:tt, $c163:tt, $c164:tt;)?
     $(future_subj_act_2: $c165:tt, $c166:tt, $c167:tt;)?
     $(future_subj_act_3: $c168:tt, $c169:tt, $c170:tt;)?
     $(future_subj_mid_1: $c171:tt, $c172:tt, $c173:tt;)?
     $(future_subj_mid_2: $c174:tt, $c175:tt, $c176:tt;)?
     $(future_subj_mid_3: $c177:tt, $c178:tt, $c179:tt;)?
     $(future_subj_pass_1: $c180:tt, $c181:tt, $c182:tt;)?
     $(future_subj_pass_2: $c183:tt, $c184:tt, $c185:tt;)?
     $(future_subj_pass_3: $c186:tt, $c187:tt, $c188:tt;)?
     $(future_opt_act_1: $c189:tt, $c190:tt, $c191:tt;)?
     $(future_opt_act_2: $c192:tt, $c193:tt, $c194:tt;)?
     $(future_opt_act_3: $c195:tt, $c196:tt, $c197:tt;)?
     $(future_opt_mid_1: $c198:tt, $c199:tt, $c200:tt;)?
     $(future_opt_mid_2: $c201:tt, $c202:tt, $c203:tt;)?
     $(future_opt_mid_3: $c204:tt, $c205:tt, $c206:tt;)?
     $(future_opt_pass_1: $c207:tt, $c208:tt, $c209:tt;)?
     $(future_opt_pass_2: $c210:tt, $c211:tt, $c212:tt;)?
     $(future_opt_pass_3: $c213:tt, $c214:tt, $c215:tt;)?
     $(future_imp_act_1: $c216:tt, $c217:tt, $c218:tt;)?
     $(future_imp_act_2: $c219:tt, $c220:tt, $c221:tt;)?
     $(future_imp_act_3: $c222:tt, $c223:tt, $c224:tt;)?
     $(future_imp_mid_1: $c225:tt, $c226:tt, $c227:tt;)?
     $(future_imp_mid_2: $c228:tt, $c229:tt, $c230:tt;)?
     $(future_imp_mid_3: $c231:tt, $c232:tt, $c233:tt;)?
     $(future_imp_pass_1: $c234:tt, $c235:tt, $c236:tt;)?
     $(future_imp_pass_2: $c237:tt, $c238:tt, $c239:tt;)?
     $(future_imp_pass_3: $c240:tt, $c241:tt, $c242:tt;)?
     $(aorist_ind_act_1: $c243:tt, $c244:tt, $c245:tt;)?
     $(aorist_ind_act_2: $c246:tt, $c247:tt, $c248:tt;)?
     $(aorist_ind_act_3: $c249:tt, $c250:tt, $c251:tt;)?
     $(aorist_ind_mid_1: $c252:tt, $c253:tt, $c254:tt;)?
     $(aorist_ind_mid_2: $c255:tt, $c256:tt, $c257:tt;)?
     $(aorist_ind_mid_3: $c258:tt, $c259:tt, $c260:tt;)?
     $(aorist_ind_pass_1: $c261:tt, $c262:tt, $c263:tt;)?
     $(aorist_ind_pass_2: $c264:tt, $c265:tt, $c266:tt;)?
     $(aorist_ind_pass_3: $c267:tt, $c268:tt, $c269:tt;)?
     $(aorist_subj_act_1: $c270:tt, $c271:tt, $c272:tt;)?
     $(aorist_subj_act_2: $c273:tt, $c274:tt, $c275:tt;)?
     $(aorist_subj_act_3: $c276:tt, $c277:tt, $c278:tt;)?
     $(aorist_subj_mid_1: $c279:tt, $c280:tt, $c281:tt;)?
     $(aorist_subj_mid_2: $c282:tt, $c283:tt, $c284:tt;)?
     $(aorist_subj_mid_3: $c285:tt, $c286:tt, $c287:tt;)?
     $(aorist_subj_pass_1: $c288:tt, $c289:tt, $c290:tt;)?
     $(aorist_subj_pass_2: $c291:tt, $c292:tt, $c293:tt;)?
     $(aorist_subj_pass_3: $c294:tt, $c295:tt, $c296:tt;)?
     $(aorist_opt_act_1: $c297:tt, $c298:tt, $c299:tt;)?
     $(aorist_opt_act_2: $c300:tt, $c301:tt, $c302:tt;)?
     $(aorist_opt_act_3: $c303:tt, $c304:tt, $c305:tt;)?
     $(aorist_opt_mid_1: $c306:tt, $c307:tt, $c308:tt;)?
     $(aorist_opt_mid_2: $c309:tt, $c310:tt, $c311:tt;)?
     $(aorist_opt_mid_3: $c312:tt, $c313:tt, $c314:tt;)?
     $(aorist_opt_pass_1: $c315:tt, $c316:tt, $c317:tt;)?
     $(aorist_opt_pass_2: $c318:tt, $c319:tt, $c320:tt;)?
     $(aorist_opt_pass_3: $c321:tt, $c322:tt, $c323:tt;)?
     $(aorist_imp_act_1: $c324:tt, $c325:tt, $c326:tt;)?
     $(aorist_imp_act_2: $c327:tt, $c328:tt, $c329:tt;)?
     $(aorist_imp_act_3: $c330:tt, $c331:tt, $c332:tt;)?
     $(aorist_imp_mid_1: $c333:tt, $c334:tt, $c335:tt;)?
     $(aorist_imp_mid_2: $c336:tt, $c337:tt, $c338:tt;)?
     $(aorist_imp_mid_3: $c339:tt, $c340:tt, $c341:tt;)?
     $(aorist_imp_pass_1: $c342:tt, $c343:tt, $c344:tt;)?
     $(aorist_imp_pass_2: $c345:tt, $c346:tt, $c347:tt;)?
     $(aorist_imp_pass_3: $c348:tt, $c349:tt, $c350:tt;)?
     $(perfect_ind_act_1: $c351:tt, $c352:tt, $c353:tt;)?
     $(perfect_ind_act_2: $c354:tt, $c355:tt, $c356:tt;)?
     $(perfect_ind_act_3: $c357:tt, $c358:tt, $c359:tt;)?
     $(perfect_ind_mid_1: $c360:tt, $c361:tt, $c362:tt;)?
     $(perfect_ind_mid_2: $c363:tt, $c364:tt, $c365:tt;)?
     $(perfect_ind_mid_3: $c366:tt, $c367:tt, $c368:tt;)?
     $(perfect_ind_pass_1: $c369:tt, $c370:tt, $c371:tt;)?
     $(perfect_ind_pass_2: $c372:tt, $c373:tt, $c374:tt;)?
     $(perfect_ind_pass_3: $c375:tt, $c376:tt, $c377:tt;)?
     $(perfect_subj_act_1: $c378:tt, $c379:tt, $c380:tt;)?
     $(perfect_subj_act_2: $c381:tt, $c382:tt, $c383:tt;)?
     $(perfect_subj_act_3: $c384:tt, $c385:tt, $c386:tt;)?
     $(perfect_subj_mid_1: $c387:tt, $c388:tt, $c389:tt;)?
     $(perfect_subj_mid_2: $c390:tt, $c391:tt, $c392:tt;)?
     $(perfect_subj_mid_3: $c393:tt, $c394:tt, $c395:tt;)?
     $(perfect_subj_pass_1: $c396:tt, $c397:tt, $c398:tt;)?
     $(perfect_subj_pass_2: $c399:tt, $c400:tt, $c401:tt;)?
     $(perfect_subj_pass_3: $c402:tt, $c403:tt, $c404:tt;)?
     $(perfect_opt_act_1: $c405:tt, $c406:tt, $c407:tt;)?
     $(perfect_opt_act_2: $c408:tt, $c409:tt, $c410:tt;)?
     $(perfect_opt_act_3: $c411:tt, $c412:tt, $c413:tt;)?
     $(perfect_opt_mid_1: $c414:tt, $c415:tt, $c416:tt;)?
     $(perfect_opt_mid_2: $c417:tt, $c418:tt, $c419:tt;)?
     $(perfect_opt_mid_3: $c420:tt, $c421:tt, $c422:tt;)?
     $(perfect_opt_pass_1: $c423:tt, $c424:tt, $c425:tt;)?
     $(perfect_opt_pass_2: $c426:tt, $c427:tt, $c428:tt;)?
     $(perfect_opt_pass_3: $c429:tt, $c430:tt, $c431:tt;)?
     $(perfect_imp_act_1: $c432:tt, $c433:tt, $c434:tt;)?
     $(perfect_imp_act_2: $c435:tt, $c436:tt, $c437:tt;)?
     $(perfect_imp_act_3: $c438:tt, $c439:tt, $c440:tt;)?
     $(perfect_imp_mid_1: $c441:tt, $c442:tt, $c443:tt;)?
     $(perfect_imp_mid_2: $c444:tt, $c445:tt, $c446:tt;)?
     $(perfect_imp_mid_3: $c447:tt, $c448:tt, $c449:tt;)?
     $(perfect_imp_pass_1: $c450:tt, $c451:tt, $c452:tt;)?
     $(perfect_imp_pass_2: $c453:tt, $c454:tt, $c455:tt;)?
     $(perfect_imp_pass_3: $c456:tt, $c457:tt, $c458:tt;)?
     $(pluperfect_act_1: $c459:tt, $c460:tt, $c461:tt;)?
     $(pluperfect_act_2: $c462:tt, $c463:tt, $c464:tt;)?
     $(pluperfect_act_3: $c465:tt, $c466:tt, $c467:tt;)?
     $(pluperfect_mid_1: $c468:tt, $c469:tt, $c470:tt;)?
     $(pluperfect_mid_2: $c471:tt, $c472:tt, $c473:tt;)?
     $(pluperfect_mid_3: $c474:tt, $c475:tt, $c476:tt;)?
     $(pluperfect_pass_1: $c477:tt, $c478:tt, $c479:tt;)?
     $(pluperfect_pass_2: $c480:tt, $c481:tt, $c482:tt;)?
     $(pluperfect_pass_3: $c483:tt, $c484:tt, $c485:tt;)?
     $(future_perfect_ind_act_1: $c486:tt, $c487:tt, $c488:tt;)?
     $(future_perfect_ind_act_2: $c489:tt, $c490:tt, $c491:tt;)?
     $(future_perfect_ind_act_3: $c492:tt, $c493:tt, $c494:tt;)?
     $(future_perfect_ind_mid_1: $c495:tt, $c496:tt, $c497:tt;)?
     $(future_perfect_ind_mid_2: $c498:tt, $c499:tt, $c500:tt;)?
     $(future_perfect_ind_mid_3: $c501:tt, $c502:tt, $c503:tt;)?
     $(future_perfect_ind_pass_1: $c504:tt, $c505:tt, $c506:tt;)?
     $(future_perfect_ind_pass_2: $c507:tt, $c508:tt, $c509:tt;)?
     $(future_perfect_ind_pass_3: $c510:tt, $c511:tt, $c512:tt;)?
     $(future_perfect_subj_act_1: $c513:tt, $c514:tt, $c515:tt;)?
     $(future_perfect_subj_act_2: $c516:tt, $c517:tt, $c518:tt;)?
     $(future_perfect_subj_act_3: $c519:tt, $c520:tt, $c521:tt;)?
     $(future_perfect_subj_mid_1: $c522:tt, $c523:tt, $c524:tt;)?
     $(future_perfect_subj_mid_2: $c525:tt, $c526:tt, $c527:tt;)?
     $(future_perfect_subj_mid_3: $c528:tt, $c529:tt, $c530:tt;)?
     $(future_perfect_subj_pass_1: $c531:tt, $c532:tt, $c533:tt;)?
     $(future_perfect_subj_pass_2: $c534:tt, $c535:tt, $c536:tt;)?
     $(future_perfect_subj_pass_3: $c537:tt, $c538:tt, $c539:tt;)?
     $(future_perfect_opt_act_1: $c540:tt, $c541:tt, $c542:tt;)?
     $(future_perfect_opt_act_2: $c543:tt, $c544:tt, $c545:tt;)?
     $(future_perfect_opt_act_3: $c546:tt, $c547:tt, $c548:tt;)?
     $(future_perfect_opt_mid_1: $c549:tt, $c550:tt, $c551:tt;)?
     $(future_perfect_opt_mid_2: $c552:tt, $c553:tt, $c554:tt;)?
     $(future_perfect_opt_mid_3: $c555:tt, $c556:tt, $c557:tt;)?
     $(future_perfect_opt_pass_1: $c558:tt, $c559:tt, $c560:tt;)?
     $(future_perfect_opt_pass_2: $c561:tt, $c562:tt, $c563:tt;)?
     $(future_perfect_opt_pass_3: $c564:tt, $c565:tt, $c566:tt;)?
     $(future_perfect_imp_act_1: $c567:tt, $c568:tt, $c569:tt;)?
     $(future_perfect_imp_act_2: $c570:tt, $c571:tt, $c572:tt;)?
     $(future_perfect_imp_act_3: $c573:tt, $c574:tt, $c575:tt;)?
     $(future_perfect_imp_mid_1: $c576:tt, $c577:tt, $c578:tt;)?
     $(future_perfect_imp_mid_2: $c579:tt, $c580:tt, $c581:tt;)?
     $(future_perfect_imp_mid_3: $c582:tt, $c583:tt, $c584:tt;)?
     $(future_perfect_imp_pass_1: $c585:tt, $c586:tt, $c587:tt;)?
     $(future_perfect_imp_pass_2: $c588:tt, $c589:tt, $c590:tt;)?
     $(future_perfect_imp_pass_3: $c591:tt, $c592:tt, $c593:tt;)?
     $(infinitive_pres: $c594:tt, $c595:tt, $c596:tt;)?
     $(infinitive_fut: $c597:tt, $c598:tt, $c599:tt;)?
     $(infinitive_aor: $c600:tt, $c601:tt, $c602:tt;)?
     $(infinitive_perf: $c603:tt, $c604:tt, $c605:tt;)?
     $(infinitive_futp: $c606:tt, $c607:tt, $c608:tt;)?
     $(participle_pres: $c609:tt, $c610:tt, $c611:tt;)?
     $(participle_fut: $c612:tt, $c613:tt, $c614:tt;)?
     $(participle_aor: $c615:tt, $c616:tt, $c617:tt;)?
     $(participle_perf: $c618:tt, $c619:tt, $c620:tt;)?
     $(participle_futp: $c621:tt, $c622:tt, $c623:tt;)?
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
            )),
            stems: ::std::borrow::Cow::Borrowed(concat!(
                $(flexion::cell!($s0),)? "\t",
                $(flexion::cell!($s1),)? "\t",
                $(flexion::cell!($s2),)? "\t",
                $(flexion::cell!($s3),)? "\t",
                $(flexion::cell!($s4),)? "\t",
                $(flexion::cell!($s5),)? "\t",
                $(flexion::cell!($s6),)? "\t",
                $(flexion::cell!($s7),)? "\t",
                $(flexion::cell!($s8),)? "\t",
                $(flexion::cell!($s9),)? "\t",
                $(flexion::cell!($s10),)? "\t",
                $(flexion::cell!($s11),)? "\t",
                $(flexion::cell!($s12),)? "\t",
            )),
            affix: affix!($($($affix $(, $other, $letters)?)?)?),
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

/// What a conjugation adds after each pattern of the one it is like (`with "ˇ"`: `ˇ` after every pattern).
macro_rules! affix {
    () => {
        None
    };
    ($affix:literal) => {
        affix!($affix, $affix, "")
    };
    ($affix:literal, $other:literal, $letters:literal) => {
        Some(flexion::Affix {
            pattern: ::std::borrow::Cow::Borrowed($affix),
            other: ::std::borrow::Cow::Borrowed($other),
            letters: ::std::borrow::Cow::Borrowed($letters),
        })
    };
}

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `μελλω`: 234 words.
    Μελλω,
    /// Like `ποιεω`: 199 words.
    Ποιεω,
    /// Like `βουλευω`: 129 words.
    Βουλευω,
    /// Like `ἀξῐοω`: 80 words.
    Ἀξῐοω,
    /// Like `κρῑνομαι`: 63 words.
    Κρῑνομαι,
    /// Like `χρημᾰτῐζω`: 62 words.
    Χρημᾰτῐζω,
    /// Like `δῐκᾰζω`: 60 words.
    Δῐκᾰζω,
    /// Like `φοιτᾰω`: 53 words.
    Φοιτᾰω,
    /// Like `σκοπεω`: 49 words.
    Σκοπεω,
    /// Like `τᾰσσω`: 32 words.
    Τᾰσσω,
    /// Like `ληγω`: 29 words.
    Ληγω,
    /// Like `ψηφῐζω`: 26 words.
    Ψηφῐζω,
    /// Like `στῐζω`: 23 words.
    Στῐζω,
    /// Like `ποικιλλω`: 22 words.
    Ποικιλλω,
    /// Like `κᾰτηχεω`: 22 words.
    Κᾰτηχεω,
    /// Like `φᾰντᾰζω`: 21 words.
    Φᾰντᾰζω,
    /// Like `δᾰπτω`: 20 words.
    Δᾰπτω,
    /// Like `νῑκαω`: 18 words.
    Νῑκαω,
    /// Like `πεμπω`: 18 words.
    Πεμπω,
    /// Like `ὀρχεομαι`: 18 words.
    Ὀρχεομαι,
    /// Like `θωᾰω`: 16 words.
    Θωᾰω,
    /// Like `λογῐζομαι`: 15 words.
    Λογῐζομαι,
    /// Like `πειρᾰζω`: 15 words.
    Πειρᾰζω,
    /// Like `πᾰρῐστημῐ`: 15 words.
    Πᾰρῐστημῐ,
    /// Like `σκᾰνδᾰλῐζω`: 13 words.
    Σκᾰνδᾰλῐζω,
    /// Like `μνημονευω`: 13 words.
    Μνημονευω,
    /// Like `τεχναομαι`: 13 words.
    Τεχναομαι,
    /// Like `δῐοικεω`: 12 words.
    Δῐοικεω,
    /// Like `ῥωννῡμῐ`: 12 words.
    Ῥωννῡμῐ,
    /// Like `πορσῡνω`: 12 words.
    Πορσῡνω,
    /// Like `ἐπᾰϊω`: 12 words.
    Ἐπᾰϊω,
    /// Like `πῠρσαινω`: 11 words.
    Πῠρσαινω,
    /// Like `κᾰθιδρῡω`: 11 words.
    Κᾰθιδρῡω,
    /// Like `σαινω`: 11 words.
    Σαινω,
    /// Like `ἀνᾰγω`: 11 words.
    Ἀνᾰγω,
    /// Like `πῡθω`: 11 words.
    Πῡθω,
    /// Like `δαμνημῐ`: 11 words.
    Δαμνημῐ,
    /// Like `ζευγνῡμῐ`: 10 words.
    Ζευγνῡμῐ,
    /// Like `ὁμᾰρτεω`: 10 words.
    Ὁμᾰρτεω,
    /// Like `χεω`: 10 words.
    Χεω,
    /// Like `ἀφῑημῐ`: 10 words.
    Ἀφῑημῐ,
    /// Like `στραγγᾰλοομαι`: 9 words.
    Στραγγᾰλοομαι,
    /// Like `ζωγρᾰφεω`: 9 words.
    Ζωγρᾰφεω,
    /// Like `φθειρω`: 9 words.
    Φθειρω,
    /// Like `ἐξερχομαι`: 9 words.
    Ἐξερχομαι,
    /// Like `δαινῡμῐ`: 9 words.
    Δαινῡμῐ,
    /// Like `σῑνομαι`: 9 words.
    Σῑνομαι,
    /// Like `ἀθῡρω`: 8 words.
    Ἀθῡρω,
    /// Like `ψῑλοω`: 8 words.
    Ψῑλοω,
    /// Like `φᾰρμᾰσσω`: 8 words.
    Φᾰρμᾰσσω,
    /// Like `δῐωκω`: 8 words.
    Δῐωκω,
    /// Like `σφᾰλλω`: 8 words.
    Σφᾰλλω,
    /// Like `κῠω`: 8 words.
    Κῠω,
    /// Like `ψευδω`: 8 words.
    Ψευδω,
    /// Like `πλᾰσσω`: 8 words.
    Πλᾰσσω,
    /// Like `κερδαινω`: 8 words.
    Κερδαινω,
    /// Like `λῡω`: 8 words.
    Λῡω,
    /// Like `μαχομαι`: 8 words.
    Μαχομαι,
    /// Like `λλῡμῐ`: 8 words.
    Λλῡμῐ,
    /// Like `πρεσβευω`: 7 words.
    Πρεσβευω,
    /// Like `σοφῐζω`: 7 words.
    Σοφῐζω,
    /// Like `βᾰδιζω`: 7 words.
    Βᾰδιζω,
    /// Like `ἀπειπον`: 7 words.
    Ἀπειπον,
    /// Like `τελεω`: 7 words.
    Τελεω,
    /// Like `χραω`: 7 words.
    Χραω,
    /// Like `βουλομαι`: 7 words.
    Βουλομαι,
    /// Like `θλαω`: 7 words.
    Θλαω,
    /// Like `λᾰγοθηρεω`: 7 words.
    Λᾰγοθηρεω,
    /// Like `εἰρω`: 7 words.
    Εἰρω,
    /// Like `λλᾰμβᾰνω`: 7 words.
    Λλᾰμβᾰνω,
    /// Like `θρῠπτω`: 6 words.
    Θρῠπτω,
    /// Like `σειω`: 6 words.
    Σειω,
    /// Like `βαινω`: 6 words.
    Βαινω,
    /// Like `μηκᾰομαι`: 6 words.
    Μηκᾰομαι,
    /// Like `ἐπεξειμι`: 6 words.
    Ἐπεξειμι,
    /// Like `πᾰρεχω`: 6 words.
    Πᾰρεχω,
    /// Like `θηραω`: 6 words.
    Θηραω,
    /// Like `περιειμι`: 6 words.
    Περιειμι,
    /// Like `ἀποκωλῡω`: 6 words.
    Ἀποκωλῡω,
    /// Like `ἡκω`: 5 words.
    Ἡκω,
    /// Like `ἱζω`: 5 words.
    Ἱζω,
    /// Like `φρᾰζω`: 5 words.
    Φρᾰζω,
    /// Like `πρεπω`: 5 words.
    Πρεπω,
    /// Like `στερεω`: 5 words.
    Στερεω,
    /// Like `μᾰρτῠρεω`: 5 words.
    Μᾰρτῠρεω,
    /// Like `ἐγγῐζω`: 5 words.
    Ἐγγῐζω,
    /// Like `ἀναιρεω`: 5 words.
    Ἀναιρεω,
    /// Like `ἀνοικῐζω`: 5 words.
    Ἀνοικῐζω,
    /// Like `κᾰτᾰλᾰμβᾰνω`: 5 words.
    Κᾰτᾰλᾰμβᾰνω,
    /// Like `μετερχομαι`: 5 words.
    Μετερχομαι,
    /// Like `τελλω`: 5 words.
    Τελλω,
    /// Like `φειδομαι`: 5 words.
    Φειδομαι,
    /// Like `σκηπτω`: 5 words.
    Σκηπτω,
    /// Like `ποιμαινω`: 5 words.
    Ποιμαινω,
    /// Like `βᾰθῠνω`: 5 words.
    Βᾰθῠνω,
    /// Like `ἐξᾰκολουθεω`: 5 words.
    Ἐξᾰκολουθεω,
    /// Like `προκᾰλεω`: 5 words.
    Προκᾰλεω,
    /// Like `περαινω`: 5 words.
    Περαινω,
    /// Like `ἡγεομαι`: 4 words.
    Ἡγεομαι,
    /// Like `κοπτω`: 4 words.
    Κοπτω,
    /// Like `ἐπᾰγγελλω`: 4 words.
    Ἐπᾰγγελλω,
    /// Like `λειβω`: 4 words.
    Λειβω,
    /// Like `ἐξᾰττῐκῐζω`: 4 words.
    Ἐξᾰττῐκῐζω,
    /// Like `προφερω`: 4 words.
    Προφερω,
    /// Like `περῐτρεχω`: 4 words.
    Περῐτρεχω,
    /// Like `δερω`: 4 words.
    Δερω,
    /// Like `γελᾰω`: 4 words.
    Γελᾰω,
    /// Like `βῐᾰζομαι`: 4 words.
    Βῐᾰζομαι,
    /// Like `μᾰρνᾰμαι`: 4 words.
    Μᾰρνᾰμαι,
    /// Like `νεκραγωγεω`: 4 words.
    Νεκραγωγεω,
    /// Like `ἀνᾰπτω`: 4 words.
    Ἀνᾰπτω,
    /// Like `πειρᾰω`: 4 words.
    Πειρᾰω,
    /// Like `πᾰρᾰλλᾰσσω`: 4 words.
    Πᾰρᾰλλᾰσσω,
    /// Like `ἀμεργω`: 4 words.
    Ἀμεργω,
    /// Like `πᾰροξῡνω`: 4 words.
    Πᾰροξῡνω,
    /// Like `ἐλᾰσσοω`: 4 words.
    Ἐλᾰσσοω,
    /// Like `ἐρωτᾰω`: 4 words.
    Ἐρωτᾰω,
    /// Like `πᾰρᾰγῐγνομαι`: 4 words.
    Πᾰρᾰγῐγνομαι,
    /// Like `κρουω`: 4 words.
    Κρουω,
    /// Like `πλουτοκρᾰτεομαι`: 4 words.
    Πλουτοκρᾰτεομαι,
    /// Like `κυκλοω`: 4 words.
    Κυκλοω,
    /// Like `ψᾱω`: 4 words.
    Ψᾱω,
    /// Like `πληττω`: 4 words.
    Πληττω,
    /// Like `ἐπαιρω`: 4 words.
    Ἐπαιρω,
    /// Like `ἀπαιτεω`: 4 words.
    Ἀπαιτεω,
    /// Like `ἐξορκῐζω`: 4 words.
    Ἐξορκῐζω,
    /// Like `ὑπᾰνᾰχωρεω`: 4 words.
    Ὑπᾰνᾰχωρεω,
    /// Like `γρᾰφω`: 3 words.
    Γρᾰφω,
    /// Like `ἡλοω`: 3 words.
    Ἡλοω,
    /// Like `πῑαινω`: 3 words.
    Πῑαινω,
    /// Like `θερᾰπευω`: 3 words.
    Θερᾰπευω,
    /// Like `ἀνᾰζω`: 3 words.
    Ἀνᾰζω,
    /// Like `τεινω`: 3 words.
    Τεινω,
    /// Like `πλᾰναω`: 3 words.
    Πλᾰναω,
    /// Like `ὀμνῠω`: 3 words.
    Ὀμνῠω,
    /// Like `προβαλλω`: 3 words.
    Προβαλλω,
    /// Like `ἐπῐτῐθημῐ`: 3 words.
    Ἐπῐτῐθημῐ,
    /// Like `κῠβισταω`: 3 words.
    Κῠβισταω,
    /// Like `φλῡω`: 3 words.
    Φλῡω,
    /// Like `ἐπευχομαι`: 3 words.
    Ἐπευχομαι,
    /// Like `κᾰθελκω`: 3 words.
    Κᾰθελκω,
    /// Like `θλῑβω`: 3 words.
    Θλῑβω,
    /// Like `κᾰτεχω`: 3 words.
    Κᾰτεχω,
    /// Like `σῠνιστημῐ`: 3 words.
    Σῠνιστημῐ,
    /// Like `ἰαινω`: 3 words.
    Ἰαινω,
    /// Like `ἀφῐστημῐ`: 3 words.
    Ἀφῐστημῐ,
    /// Like `δρᾰσσομαι`: 3 words.
    Δρᾰσσομαι,
    /// Like `πᾰρᾰκᾰλεω`: 3 words.
    Πᾰρᾰκᾰλεω,
    /// Like `ἐξαιρω`: 3 words.
    Ἐξαιρω,
    /// Like `ἀντᾰνᾰλῡω`: 3 words.
    Ἀντᾰνᾰλῡω,
    /// Like `ἁμᾰρτᾰνω`: 3 words.
    Ἁμᾰρτᾰνω,
    /// Like `κολᾰπτω`: 3 words.
    Κολᾰπτω,
    /// Like `ἀντῐκᾰθῐστημῐ`: 3 words.
    Ἀντῐκᾰθῐστημῐ,
    /// Like `ἐκγᾰμῐζω`: 3 words.
    Ἐκγᾰμῐζω,
    /// Like `ἀντελπῐζω`: 3 words.
    Ἀντελπῐζω,
    /// Like `σῠναιρεω`: 3 words.
    Σῠναιρεω,
    /// Like `φῡρω`: 3 words.
    Φῡρω,
    /// Like `σταθμαω`: 3 words.
    Σταθμαω,
    /// Like `κνᾰω`: 3 words.
    Κνᾰω,
    /// Like `δηλεομαι`: 3 words.
    Δηλεομαι,
    /// Like `τρωγλοδῠτεω`: 3 words.
    Τρωγλοδῠτεω,
    /// Like `κηδευω`: 3 words.
    Κηδευω,
    /// Like `λεγομαι`: 3 words.
    Λεγομαι,
    /// Like `ρρεω`: 3 words.
    Ρρεω,
    /// Like `σσῳζω`: 3 words.
    Σσῳζω,
    /// Like `ρρηγνῡμῐ`: 3 words.
    Ρρηγνῡμῐ,
    /// Like `θρεομαι`: 3 words.
    Θρεομαι,
    /// Like `κηρῡσσω`: 3 words.
    Κηρῡσσω,
    /// Like `ἐφᾰγον`: 2 words.
    Ἐφᾰγον,
    /// Like `εἰδον`: 2 words.
    Εἰδον,
    /// Like `πτᾰρνῠμαι`: 2 words.
    Πτᾰρνῠμαι,
    /// Like `ξυμπολεμεω`: 2 words.
    Ξυμπολεμεω,
    /// Like `κᾰτακειμαι`: 2 words.
    Κᾰτακειμαι,
    /// Like `ξῠνοιδᾰ`: 2 words.
    Ξῠνοιδᾰ,
    /// Like `συγκειμαι`: 2 words.
    Συγκειμαι,
    /// Like `κορινθιαζομαι`: 2 words.
    Κορινθιαζομαι,
    /// Like `στεφανοω`: 2 words.
    Στεφανοω,
    /// Like `ἐπῐχειρεω`: 2 words.
    Ἐπῐχειρεω,
    /// Like `συνεπομαι`: 2 words.
    Συνεπομαι,
    /// Like `λαλαγεω`: 2 words.
    Λαλαγεω,
    /// Like `ξυνναυμᾰχεω`: 2 words.
    Ξυνναυμᾰχεω,
    /// Like `μετᾰκᾰλεω`: 2 words.
    Μετᾰκᾰλεω,
    /// Like `επῐβουλευω`: 2 words.
    Επῐβουλευω,
    /// Like `ερεω`: 2 words.
    Ερεω,
    /// Like `ἐδω`: 1 words.
    Ἐδω,
    /// Like `αἰσθᾰνομαι`: 1 words.
    Αἰσθᾰνομαι,
    /// Like `τρυπαω`: 1 words.
    Τρυπαω,
    /// Like `μειδᾰω`: 1 words.
    Μειδᾰω,
    /// Like `ἀνᾰχρονῐζομαι`: 1 words.
    Ἀνᾰχρονῐζομαι,
    /// Like `σῠμμᾰχομαι`: 1 words.
    Σῠμμᾰχομαι,
    /// Like `κᾰθημαι`: 1 words.
    Κᾰθημαι,
    /// Like `ἐπῐκᾰλεω`: 1 words.
    Ἐπῐκᾰλεω,
    /// Like `χρη`: 1 words.
    Χρη,
    /// Like `ἠλθον`: 1 words.
    Ἠλθον,
    /// Like `ἀρνυμαι`: 1 words.
    Ἀρνυμαι,
    /// Like `ἐπεφνον`: 1 words.
    Ἐπεφνον,
    /// Like `μετεκῑᾰθον`: 1 words.
    Μετεκῑᾰθον,
    /// Like `πεπνῡμαι`: 1 words.
    Πεπνῡμαι,
    /// Like `περνημῐ`: 1 words.
    Περνημῐ,
    /// Like `ἐπῐστᾰμαι`: 1 words.
    Ἐπῐστᾰμαι,
    /// Like `ἀπαυραω`: 1 words.
    Ἀπαυραω,
    /// Like `προκειμαι`: 1 words.
    Προκειμαι,
    /// Like `ἐπῐπειθομαι`: 1 words.
    Ἐπῐπειθομαι,
    /// Like `ἐπισευω`: 1 words.
    Ἐπισευω,
    /// Like `δειμᾰτοω`: 1 words.
    Δειμᾰτοω,
    /// Like `κορυσσω`: 1 words.
    Κορυσσω,
    /// Like `κεκᾰδον`: 1 words.
    Κεκᾰδον,
    /// Like `κᾰτᾰκλαω`: 1 words.
    Κᾰτᾰκλαω,
    /// Like `ἀγωνιζομαι`: 1 words.
    Ἀγωνιζομαι,
    /// Like `ἀπιστεω`: 1 words.
    Ἀπιστεω,
    /// Like `χᾰριζω`: 1 words.
    Χᾰριζω,
    /// Like `διαφορεω`: 1 words.
    Διαφορεω,
    /// Like `τελεθω`: 1 words.
    Τελεθω,
    /// Like `δῐᾰσευομαι`: 1 words.
    Δῐᾰσευομαι,
    /// Like `φθιω`: 1 words.
    Φθιω,
    /// Like `οικειοομαι`: 1 words.
    Οικειοομαι,
    /// Like `συγκᾰκουργεω`: 1 words.
    Συγκᾰκουργεω,
    /// Like `ἀποστατεω`: 1 words.
    Ἀποστατεω,
    /// Like `γεγονοι`: 1 words.
    Γεγονοι,
    /// Like `γεγενησαι`: 1 words.
    Γεγενησαι,
    /// Like `γεγενηται`: 1 words.
    Γεγενηται,
    /// Like `γεγενησθω`: 1 words.
    Γεγενησθω,
    /// Like `γεγενησο`: 1 words.
    Γεγενησο,
    /// Like `γεγενησθον`: 1 words.
    Γεγενησθον,
    /// Like `γεγενησθων`: 1 words.
    Γεγενησθων,
    /// Like `γεγενηνται`: 1 words.
    Γεγενηνται,
    /// Like `γεγενησθε`: 1 words.
    Γεγενησθε,
    /// Like `γεγενημεθα`: 1 words.
    Γεγενημεθα,
    /// Like `τεθνηκας`: 1 words.
    Τεθνηκας,
    /// Like `τεθνηκῃ`: 1 words.
    Τεθνηκῃ,
    /// Like `τεθνηκοι`: 1 words.
    Τεθνηκοι,
    /// Like `τεθνηκοιη`: 1 words.
    Τεθνηκοιη,
    /// Like `τεθνηκοιμι`: 1 words.
    Τεθνηκοιμι,
    /// Like `τεθνηκοιην`: 1 words.
    Τεθνηκοιην,
    /// Like `τεθνηκοις`: 1 words.
    Τεθνηκοις,
    /// Like `τεθνηκοιης`: 1 words.
    Τεθνηκοιης,
    /// Like `τεθνηκῃς`: 1 words.
    Τεθνηκῃς,
    /// Like `τεθνηκετω`: 1 words.
    Τεθνηκετω,
    /// Like `τεθνηκαμεν`: 1 words.
    Τεθνηκαμεν,
    /// Like `τεθνηκωμεν`: 1 words.
    Τεθνηκωμεν,
    /// Like `τεθνηκοιμεν`: 1 words.
    Τεθνηκοιμεν,
    /// Like `τεθνηκατε`: 1 words.
    Τεθνηκατε,
    /// Like `τεθνηκητε`: 1 words.
    Τεθνηκητε,
    /// Like `τεθνηκοιτε`: 1 words.
    Τεθνηκοιτε,
    /// Like `τεθνηκετε`: 1 words.
    Τεθνηκετε,
    /// Like `τεθνηκασι`: 1 words.
    Τεθνηκασι,
    /// Like `τεθνηκασιν`: 1 words.
    Τεθνηκασιν,
    /// Like `τεθνηκωσι`: 1 words.
    Τεθνηκωσι,
    /// Like `τεθνηκωσιν`: 1 words.
    Τεθνηκωσιν,
    /// Like `τεθνηκοιεν`: 1 words.
    Τεθνηκοιεν,
    /// Like `τεθνηκοντων`: 1 words.
    Τεθνηκοντων,
    /// Like `τεθνηκατον`: 1 words.
    Τεθνηκατον,
    /// Like `τεθνηκητον`: 1 words.
    Τεθνηκητον,
    /// Like `τεθνηκοιτον`: 1 words.
    Τεθνηκοιτον,
    /// Like `τεθνηκοιτην`: 1 words.
    Τεθνηκοιτην,
    /// Like `τεθνηκετον`: 1 words.
    Τεθνηκετον,
    /// Like `τεθνηκετων`: 1 words.
    Τεθνηκετων,
    /// Like `ἀντιπαρατασσομαι`: 1 words.
    Ἀντιπαρατασσομαι,
    /// Like `ἐπᾰνερχομαι`: 1 words.
    Ἐπᾰνερχομαι,
    /// Like `κιδνᾰμαι`: 1 words.
    Κιδνᾰμαι,
    /// Like `ἐπισκεπτομαι`: 1 words.
    Ἐπισκεπτομαι,
    /// Like `κολυμβαω`: 1 words.
    Κολυμβαω,
    /// Like `δῐκειν`: 1 words.
    Δῐκειν,
    /// Like `ὑπονοεω`: 1 words.
    Ὑπονοεω,
    /// Like `ἀντᾰγωνιζομαι`: 1 words.
    Ἀντᾰγωνιζομαι,
    /// Like `διπλᾰσῐοομαι`: 1 words.
    Διπλᾰσῐοομαι,
    /// Like `ἐξικνεομαι`: 1 words.
    Ἐξικνεομαι,
    /// Like `καταπετροω`: 1 words.
    Καταπετροω,
    /// Like `λαλαχευομαι`: 1 words.
    Λαλαχευομαι,
    /// Like `μικρογρᾰφεω`: 1 words.
    Μικρογρᾰφεω,
    /// Like `λῐνοστᾰτεω`: 1 words.
    Λῐνοστᾰτεω,
    /// Like `ἀδησᾰ`: 1 words.
    Ἀδησᾰ,
    /// Like `ὑποκοριζομαι`: 1 words.
    Ὑποκοριζομαι,
    /// Like `ἰχᾰνᾰω`: 1 words.
    Ἰχᾰνᾰω,
    /// Like `ᾰντᾰποκρῑνομαι`: 1 words.
    Ᾰντᾰποκρῑνομαι,
    /// Like `ὑπεκτιθεμαι`: 1 words.
    Ὑπεκτιθεμαι,
    /// Like `μᾰστῐγοω`: 1 words.
    Μᾰστῐγοω,
    /// Like `βᾰρβᾰροομαι`: 1 words.
    Βᾰρβᾰροομαι,
    /// Like `ἐπιμιμνησκομαι`: 1 words.
    Ἐπιμιμνησκομαι,
    /// Like `εργεω`: 1 words.
    Εργεω,
    /// Like `ᾰνθρωπεω`: 1 words.
    Ᾰνθρωπεω,
    /// Like `εισκᾰτοικεω`: 1 words.
    Εισκᾰτοικεω,
    /// Like `εκπλεω`: 1 words.
    Εκπλεω,
    /// Like `δειδῐᾰ`: 1 words.
    Δειδῐᾰ,
    /// Like `λανθανομαι`: 1 words.
    Λανθανομαι,
    /// Like `τρᾰπω`: 1 words.
    Τρᾰπω,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 286] = [
        Conjugation::Μελλω,
        Conjugation::Ποιεω,
        Conjugation::Βουλευω,
        Conjugation::Ἀξῐοω,
        Conjugation::Κρῑνομαι,
        Conjugation::Χρημᾰτῐζω,
        Conjugation::Δῐκᾰζω,
        Conjugation::Φοιτᾰω,
        Conjugation::Σκοπεω,
        Conjugation::Τᾰσσω,
        Conjugation::Ληγω,
        Conjugation::Ψηφῐζω,
        Conjugation::Στῐζω,
        Conjugation::Ποικιλλω,
        Conjugation::Κᾰτηχεω,
        Conjugation::Φᾰντᾰζω,
        Conjugation::Δᾰπτω,
        Conjugation::Νῑκαω,
        Conjugation::Πεμπω,
        Conjugation::Ὀρχεομαι,
        Conjugation::Θωᾰω,
        Conjugation::Λογῐζομαι,
        Conjugation::Πειρᾰζω,
        Conjugation::Πᾰρῐστημῐ,
        Conjugation::Σκᾰνδᾰλῐζω,
        Conjugation::Μνημονευω,
        Conjugation::Τεχναομαι,
        Conjugation::Δῐοικεω,
        Conjugation::Ῥωννῡμῐ,
        Conjugation::Πορσῡνω,
        Conjugation::Ἐπᾰϊω,
        Conjugation::Πῠρσαινω,
        Conjugation::Κᾰθιδρῡω,
        Conjugation::Σαινω,
        Conjugation::Ἀνᾰγω,
        Conjugation::Πῡθω,
        Conjugation::Δαμνημῐ,
        Conjugation::Ζευγνῡμῐ,
        Conjugation::Ὁμᾰρτεω,
        Conjugation::Χεω,
        Conjugation::Ἀφῑημῐ,
        Conjugation::Στραγγᾰλοομαι,
        Conjugation::Ζωγρᾰφεω,
        Conjugation::Φθειρω,
        Conjugation::Ἐξερχομαι,
        Conjugation::Δαινῡμῐ,
        Conjugation::Σῑνομαι,
        Conjugation::Ἀθῡρω,
        Conjugation::Ψῑλοω,
        Conjugation::Φᾰρμᾰσσω,
        Conjugation::Δῐωκω,
        Conjugation::Σφᾰλλω,
        Conjugation::Κῠω,
        Conjugation::Ψευδω,
        Conjugation::Πλᾰσσω,
        Conjugation::Κερδαινω,
        Conjugation::Λῡω,
        Conjugation::Μαχομαι,
        Conjugation::Λλῡμῐ,
        Conjugation::Πρεσβευω,
        Conjugation::Σοφῐζω,
        Conjugation::Βᾰδιζω,
        Conjugation::Ἀπειπον,
        Conjugation::Τελεω,
        Conjugation::Χραω,
        Conjugation::Βουλομαι,
        Conjugation::Θλαω,
        Conjugation::Λᾰγοθηρεω,
        Conjugation::Εἰρω,
        Conjugation::Λλᾰμβᾰνω,
        Conjugation::Θρῠπτω,
        Conjugation::Σειω,
        Conjugation::Βαινω,
        Conjugation::Μηκᾰομαι,
        Conjugation::Ἐπεξειμι,
        Conjugation::Πᾰρεχω,
        Conjugation::Θηραω,
        Conjugation::Περιειμι,
        Conjugation::Ἀποκωλῡω,
        Conjugation::Ἡκω,
        Conjugation::Ἱζω,
        Conjugation::Φρᾰζω,
        Conjugation::Πρεπω,
        Conjugation::Στερεω,
        Conjugation::Μᾰρτῠρεω,
        Conjugation::Ἐγγῐζω,
        Conjugation::Ἀναιρεω,
        Conjugation::Ἀνοικῐζω,
        Conjugation::Κᾰτᾰλᾰμβᾰνω,
        Conjugation::Μετερχομαι,
        Conjugation::Τελλω,
        Conjugation::Φειδομαι,
        Conjugation::Σκηπτω,
        Conjugation::Ποιμαινω,
        Conjugation::Βᾰθῠνω,
        Conjugation::Ἐξᾰκολουθεω,
        Conjugation::Προκᾰλεω,
        Conjugation::Περαινω,
        Conjugation::Ἡγεομαι,
        Conjugation::Κοπτω,
        Conjugation::Ἐπᾰγγελλω,
        Conjugation::Λειβω,
        Conjugation::Ἐξᾰττῐκῐζω,
        Conjugation::Προφερω,
        Conjugation::Περῐτρεχω,
        Conjugation::Δερω,
        Conjugation::Γελᾰω,
        Conjugation::Βῐᾰζομαι,
        Conjugation::Μᾰρνᾰμαι,
        Conjugation::Νεκραγωγεω,
        Conjugation::Ἀνᾰπτω,
        Conjugation::Πειρᾰω,
        Conjugation::Πᾰρᾰλλᾰσσω,
        Conjugation::Ἀμεργω,
        Conjugation::Πᾰροξῡνω,
        Conjugation::Ἐλᾰσσοω,
        Conjugation::Ἐρωτᾰω,
        Conjugation::Πᾰρᾰγῐγνομαι,
        Conjugation::Κρουω,
        Conjugation::Πλουτοκρᾰτεομαι,
        Conjugation::Κυκλοω,
        Conjugation::Ψᾱω,
        Conjugation::Πληττω,
        Conjugation::Ἐπαιρω,
        Conjugation::Ἀπαιτεω,
        Conjugation::Ἐξορκῐζω,
        Conjugation::Ὑπᾰνᾰχωρεω,
        Conjugation::Γρᾰφω,
        Conjugation::Ἡλοω,
        Conjugation::Πῑαινω,
        Conjugation::Θερᾰπευω,
        Conjugation::Ἀνᾰζω,
        Conjugation::Τεινω,
        Conjugation::Πλᾰναω,
        Conjugation::Ὀμνῠω,
        Conjugation::Προβαλλω,
        Conjugation::Ἐπῐτῐθημῐ,
        Conjugation::Κῠβισταω,
        Conjugation::Φλῡω,
        Conjugation::Ἐπευχομαι,
        Conjugation::Κᾰθελκω,
        Conjugation::Θλῑβω,
        Conjugation::Κᾰτεχω,
        Conjugation::Σῠνιστημῐ,
        Conjugation::Ἰαινω,
        Conjugation::Ἀφῐστημῐ,
        Conjugation::Δρᾰσσομαι,
        Conjugation::Πᾰρᾰκᾰλεω,
        Conjugation::Ἐξαιρω,
        Conjugation::Ἀντᾰνᾰλῡω,
        Conjugation::Ἁμᾰρτᾰνω,
        Conjugation::Κολᾰπτω,
        Conjugation::Ἀντῐκᾰθῐστημῐ,
        Conjugation::Ἐκγᾰμῐζω,
        Conjugation::Ἀντελπῐζω,
        Conjugation::Σῠναιρεω,
        Conjugation::Φῡρω,
        Conjugation::Σταθμαω,
        Conjugation::Κνᾰω,
        Conjugation::Δηλεομαι,
        Conjugation::Τρωγλοδῠτεω,
        Conjugation::Κηδευω,
        Conjugation::Λεγομαι,
        Conjugation::Ρρεω,
        Conjugation::Σσῳζω,
        Conjugation::Ρρηγνῡμῐ,
        Conjugation::Θρεομαι,
        Conjugation::Κηρῡσσω,
        Conjugation::Ἐφᾰγον,
        Conjugation::Εἰδον,
        Conjugation::Πτᾰρνῠμαι,
        Conjugation::Ξυμπολεμεω,
        Conjugation::Κᾰτακειμαι,
        Conjugation::Ξῠνοιδᾰ,
        Conjugation::Συγκειμαι,
        Conjugation::Κορινθιαζομαι,
        Conjugation::Στεφανοω,
        Conjugation::Ἐπῐχειρεω,
        Conjugation::Συνεπομαι,
        Conjugation::Λαλαγεω,
        Conjugation::Ξυνναυμᾰχεω,
        Conjugation::Μετᾰκᾰλεω,
        Conjugation::Επῐβουλευω,
        Conjugation::Ερεω,
        Conjugation::Ἐδω,
        Conjugation::Αἰσθᾰνομαι,
        Conjugation::Τρυπαω,
        Conjugation::Μειδᾰω,
        Conjugation::Ἀνᾰχρονῐζομαι,
        Conjugation::Σῠμμᾰχομαι,
        Conjugation::Κᾰθημαι,
        Conjugation::Ἐπῐκᾰλεω,
        Conjugation::Χρη,
        Conjugation::Ἠλθον,
        Conjugation::Ἀρνυμαι,
        Conjugation::Ἐπεφνον,
        Conjugation::Μετεκῑᾰθον,
        Conjugation::Πεπνῡμαι,
        Conjugation::Περνημῐ,
        Conjugation::Ἐπῐστᾰμαι,
        Conjugation::Ἀπαυραω,
        Conjugation::Προκειμαι,
        Conjugation::Ἐπῐπειθομαι,
        Conjugation::Ἐπισευω,
        Conjugation::Δειμᾰτοω,
        Conjugation::Κορυσσω,
        Conjugation::Κεκᾰδον,
        Conjugation::Κᾰτᾰκλαω,
        Conjugation::Ἀγωνιζομαι,
        Conjugation::Ἀπιστεω,
        Conjugation::Χᾰριζω,
        Conjugation::Διαφορεω,
        Conjugation::Τελεθω,
        Conjugation::Δῐᾰσευομαι,
        Conjugation::Φθιω,
        Conjugation::Οικειοομαι,
        Conjugation::Συγκᾰκουργεω,
        Conjugation::Ἀποστατεω,
        Conjugation::Γεγονοι,
        Conjugation::Γεγενησαι,
        Conjugation::Γεγενηται,
        Conjugation::Γεγενησθω,
        Conjugation::Γεγενησο,
        Conjugation::Γεγενησθον,
        Conjugation::Γεγενησθων,
        Conjugation::Γεγενηνται,
        Conjugation::Γεγενησθε,
        Conjugation::Γεγενημεθα,
        Conjugation::Τεθνηκας,
        Conjugation::Τεθνηκῃ,
        Conjugation::Τεθνηκοι,
        Conjugation::Τεθνηκοιη,
        Conjugation::Τεθνηκοιμι,
        Conjugation::Τεθνηκοιην,
        Conjugation::Τεθνηκοις,
        Conjugation::Τεθνηκοιης,
        Conjugation::Τεθνηκῃς,
        Conjugation::Τεθνηκετω,
        Conjugation::Τεθνηκαμεν,
        Conjugation::Τεθνηκωμεν,
        Conjugation::Τεθνηκοιμεν,
        Conjugation::Τεθνηκατε,
        Conjugation::Τεθνηκητε,
        Conjugation::Τεθνηκοιτε,
        Conjugation::Τεθνηκετε,
        Conjugation::Τεθνηκασι,
        Conjugation::Τεθνηκασιν,
        Conjugation::Τεθνηκωσι,
        Conjugation::Τεθνηκωσιν,
        Conjugation::Τεθνηκοιεν,
        Conjugation::Τεθνηκοντων,
        Conjugation::Τεθνηκατον,
        Conjugation::Τεθνηκητον,
        Conjugation::Τεθνηκοιτον,
        Conjugation::Τεθνηκοιτην,
        Conjugation::Τεθνηκετον,
        Conjugation::Τεθνηκετων,
        Conjugation::Ἀντιπαρατασσομαι,
        Conjugation::Ἐπᾰνερχομαι,
        Conjugation::Κιδνᾰμαι,
        Conjugation::Ἐπισκεπτομαι,
        Conjugation::Κολυμβαω,
        Conjugation::Δῐκειν,
        Conjugation::Ὑπονοεω,
        Conjugation::Ἀντᾰγωνιζομαι,
        Conjugation::Διπλᾰσῐοομαι,
        Conjugation::Ἐξικνεομαι,
        Conjugation::Καταπετροω,
        Conjugation::Λαλαχευομαι,
        Conjugation::Μικρογρᾰφεω,
        Conjugation::Λῐνοστᾰτεω,
        Conjugation::Ἀδησᾰ,
        Conjugation::Ὑποκοριζομαι,
        Conjugation::Ἰχᾰνᾰω,
        Conjugation::Ᾰντᾰποκρῑνομαι,
        Conjugation::Ὑπεκτιθεμαι,
        Conjugation::Μᾰστῐγοω,
        Conjugation::Βᾰρβᾰροομαι,
        Conjugation::Ἐπιμιμνησκομαι,
        Conjugation::Εργεω,
        Conjugation::Ᾰνθρωπεω,
        Conjugation::Εισκᾰτοικεω,
        Conjugation::Εκπλεω,
        Conjugation::Δειδῐᾰ,
        Conjugation::Λανθανομαι,
        Conjugation::Τρᾰπω,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 286] = [
    // Μελλω: 234 words (θελω, τῠγχᾰνω, δῐδᾰσκω, πῑνω, εὑρῐσκω, πληθω).
    conjugation!("μελλω", 234, "~ω";
    stem lemma: "~- like ἐχω";
    stem rest____: "~- like ἐχω";
    stem rest__: "~ησ- like ἐχω";
    stem rest_______: "~ηθησ- like ἐχω";
    stem rest________: "~ησ- like ἐχω";
    stem rest__________: "~ηθην- like ἐχω";
    stem rest_: "~ησ- like ἐχω";
    stem rest_____: "~ηθ- like ἐχω";
    stem rest___: "~ηκ- like ἐχω";
    stem rest: "~η- like ἐχω";
    stem rest______: "~η- like ἐχω";
    stem rest___________: "~ηκειν- like ἐχω";
    stem rest_________: "~η- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    future_perfect_ind_pass_1: derived, _, _;
    infinitive_perf: _, _, derived;
    ),
    // Ποιεω: 199 words (φοβεω, βλασφημεω, γᾰμεω, μῑσεω, διακονεω, ποιεω). Like Μελλω, except in 33 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ποιεω", 199, "~εω", like Μελλω;
    stem lemma: "~- like ποιεω";
    stem rest____: "~- like ποιεω";
    stem rest__: "~ησ- like ποιεω";
    stem rest_______: "~ηθησ- like ποιεω";
    stem rest________: "~ησ- like ποιεω";
    stem rest_: "~ησ- like ποιεω";
    stem rest_____: "~ηθ- like ποιεω";
    stem rest___: "~ηκ- like ποιεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_subj_mid_3: _, _, "%ε~ημένοι ὦσῐ|%ε~ημένοι ὦσῐν";
    perfect_subj_pass_3: _, _, derived;
    ),
    // Βουλευω: 129 words (θερᾰπευω, ψῡχω, ἀρχω, πῐστευω, αἰθω, νηστευω). Like Μελλω, except in 65 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("βουλευω", 129, "~ω", like Μελλω;
    stem lemma: "~- like βουλευω";
    stem rest____: "~- like βουλευω";
    stem rest__: "~σ- like ποιεω";
    stem rest_______: "~θησ- like ἐχω";
    stem rest________: "~σ- like ποιεω";
    stem rest__________: "~θην- like ἐχω";
    stem rest_: "~σ- like ποιεω";
    stem rest_____: "~θ- like ἐχω";
    stem rest___: "~κ- like ἐχω";
    stem rest: "~- like βουλευω";
    stem rest______: "~- like ἐχω";
    stem rest___________: "~κειν- like ἐχω";
    stem rest_________: "~- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    future_perfect_ind_mid_1: derived, _, _;
    future_perfect_ind_mid_3: _, derived, _;
    future_perfect_ind_pass_1: _, _, derived;
    future_perfect_ind_pass_2: derived, derived, derived;
    future_perfect_ind_pass_3: derived, derived, derived;
    future_perfect_opt_pass_1: derived, _, derived;
    future_perfect_opt_pass_2: derived, derived, derived;
    future_perfect_opt_pass_3: derived, derived, derived;
    infinitive_perf: _, _, derived;
    infinitive_futp: _, _, derived;
    participle_futp: _, _, derived;
    ),
    // Ἀξῐοω: 80 words (ὁμοιοω, μορφοω, ζηλοω, ναρκοω, κοινοω, ἀλλοιοω). Like Βουλευω, except in 46 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἀξῐοω", 80, "~οω", like Βουλευω;
    stem lemma: "~- like γῡροω";
    stem rest____: "~ου- like γῡροω";
    stem rest__: "~ωσ- like γῡροω";
    stem rest_______: "~ωθησ- like ἐχω";
    stem rest________: "~ωσ- like ποιεω";
    stem rest__________: "~ωθην- like ἐχω";
    stem rest_: "~ωσ- like ποιεω";
    stem rest_____: "~ωθ- like ἐχω";
    stem rest___: "~ωκ- like ἐχω";
    stem rest: "~ω- like βουλευω";
    stem rest______: "~ω- like ἐχω";
    stem rest___________: "~ωκειν- like ἐχω";
    stem rest_________: "~ω- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Κρῑνομαι: 63 words (ἐρχομαι, γεινομαι, γῐγνομαι, δεχομαι, δερκομαι, μεμφομαι). Like Βουλευω, except in 82 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("κρῑνομαι", 63, "~ομαι", like Βουλευω;
    stem rest__: "~σ- like ἑπομαι";
    ind_act_1: "~ομαι", _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: derived, _, derived;
    ind_mid_2: "~ῃ", _, _;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_2: "~ῃ", _, _;
    subj_pass_2: "~ῃ", _, _;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: _, derived, _;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, _;
    future_opt_act_3: derived, derived, _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_subj_act_3: _, derived, _;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    pluperfect_act_1: derived, _, _;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    participle_pres: derived, _, _;
    participle_aor: derived, _, _;
    ),
    // Χρημᾰτῐζω: 62 words (κᾰθᾰρῐζω, βᾰσᾰνῐζω, κᾰπνῐζω, φενᾱκῐζω, γεμῐζω, ἱζω). Like Βουλευω, except in 38 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("χρημᾰτῐζω", 62, "~ζω", like Βουλευω;
    stem lemma: "~ζ- like βουλευω";
    stem rest____: "~ζ- like βουλευω";
    stem rest__: "~- like χρημᾰτῐζω";
    stem rest_______: "~σθησ- like ποιεω";
    stem rest__________: "~σθην- like ἐχω";
    stem rest_____: "~σθ- like ποιεω";
    stem rest___: "~κ- like ποιεω";
    stem rest: "~σ- like χρημᾰτῐζω";
    stem rest___________: "~κ- like χρημᾰτῐζω";
    stem rest_________: "~- like χρημᾰτῐζω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, _, "%ε~δᾰται";
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_pass_3: _, _, derived;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    ),
    // Δῐκᾰζω: 60 words (πυρρᾰζω, βᾰπτῐζω, δᾰμαζω, σῐζω, εὐᾱζω, ἐξῠπνῐζω). Like Χρημᾰτῐζω, except in 67 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("δῐκᾰζω", 60, "~ζω", like Χρημᾰτῐζω;
    stem lemma: "~ζ- like βᾰπτῐζω";
    stem rest____: "~ζ- like ἐχω";
    stem rest__: "~σ- like ἐχω";
    stem rest________: "~σ- like ἐχω";
    stem rest_: "~σ- like ἐχω";
    stem rest___: "~κ- like ἐχω";
    stem rest___________: "~κειν- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_2: _, _, derived;
    future_opt_act_3: _, _, derived;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: derived, derived, derived;
    perfect_subj_act_3: _, _, derived;
    perfect_subj_mid_3: _, _, derived;
    perfect_subj_pass_3: _, _, "%ε~σμένοι ὦσῐ";
    pluperfect_act_2: derived, _, _;
    pluperfect_act_3: derived, _, _;
    future_perfect_ind_mid_1: "%ε~σομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~σεσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~σομεθᾰ";
    future_perfect_ind_pass_2: "%ε~σῃ", "%ε~σεσθον", "%ε~σεσθε";
    future_perfect_ind_pass_3: "%ε~σεται", "%ε~σεσθον", "%ε~σονται";
    future_perfect_opt_pass_1: "%ε~σοιμην", _, "%ε~σοιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~σοιο", "%ε~σοισθον", "%ε~σοισθε";
    future_perfect_opt_pass_3: "%ε~σοιτο", "%ε~σοισθην", "%ε~σοιντο";
    infinitive_futp: _, _, "%ε~σεσθαι";
    participle_futp: _, _, "%ε~σομενος";
    ),
    // Φοιτᾰω: 53 words (ἀγᾰπᾰω, γεννᾰω, κῠβερνᾰω, γλᾰγᾰω, σῑγᾰω, κᾰκκᾰω). Like Μελλω, except in 30 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("φοιτᾰω", 53, "~ᾰω", like Μελλω;
    stem lemma: "~- like φοιτᾰω";
    stem rest____: "~- like φοιτᾰω";
    stem rest__: "~ησ- like ποιεω";
    stem rest________: "~ησ- like ποιεω";
    stem rest_: "~ησ- like ποιεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    ),
    // Σκοπεω: 49 words (ἀγᾰθοποιεω, ἀγνοεω, ἀντλεω, ἀσκεω, οἰκεω, ἀλεω). Like Ποιεω, except in 36 cells: perfect_ind_act_3 du, perfect_ind_mid_3 du, perfect_ind_pass_1 sg, perfect_ind_pass_2 sg, perfect_ind_pass_2 du, perfect_ind_pass_3 sg, ….
    conjugation!("σκοπεω", 49, "~εω", like Ποιεω;
    stem rest___: "~ηκ- like σκοπεω";
    stem rest: "~η- like σκοπεω";
    stem rest______: "~η- like σκοπεω";
    stem rest___________: "~ηκ- like σκοπεω";
    stem rest_________: "~η- like σκοπεω";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    future_perfect_ind_mid_1: derived, _, _;
    future_perfect_ind_mid_3: _, derived, _;
    future_perfect_ind_pass_1: _, _, derived;
    future_perfect_ind_pass_2: derived, derived, derived;
    future_perfect_ind_pass_3: derived, derived, derived;
    future_perfect_opt_pass_1: derived, _, derived;
    future_perfect_opt_pass_2: derived, derived, derived;
    future_perfect_opt_pass_3: derived, derived, derived;
    infinitive_perf: _, _, derived;
    infinitive_futp: _, _, derived;
    participle_futp: _, _, derived;
    ),
    // Τᾰσσω: 32 words (ἀϊσσω, πλησσω, τᾰσσω, μᾰσσω, φῠλᾰσσω, πτῠσσω). Like Βουλευω, except in 65 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("τᾰσσω", 32, "~σσω", like Βουλευω;
    stem lemma: "~σσ- like βουλευω";
    stem rest____: "~σσ- like βουλευω";
    stem rest__: "~ξ- like ποιεω";
    stem rest_______: "~χθησ- like ἐχω";
    stem rest________: "~ξ- like ποιεω";
    stem rest__________: "~χθην- like ἐχω";
    stem rest_: "~ξ- like ποιεω";
    stem rest_____: "~χθ- like ἐχω";
    stem rest___: "~χ- like ἐχω";
    stem rest______: "~- like τᾰσσω";
    stem rest___________: "~χειν- like ἐχω";
    stem rest_________: "~- like τᾰσσω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: "%ε~γμαι", _, _;
    perfect_ind_mid_3: "%ε~κται", derived, _;
    perfect_ind_pass_1: _, _, "%ε~γμεθᾰ";
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_subj_mid_3: _, "%ε~γμένω ἦτον", _;
    perfect_subj_pass_1: "%ε~γμένος ὦ", _, "%ε~γμένοι ὦμεν";
    perfect_subj_pass_2: "%ε~γμένος ᾖς", "%ε~γμένω ἦτον", "%ε~γμένοι ἦτε";
    perfect_subj_pass_3: "%ε~γμένος ᾖ", "%ε~γμένω ἦτον", "%ε~γμένοι ὦσῐ";
    perfect_opt_pass_1: "%ε~γμένος εἴην", _, "%ε~γμένοι εἴημεν";
    perfect_opt_pass_2: "%ε~γμένος εἴης", "%ε~γμένω εἴητον", "%ε~γμένοι εἴητε";
    perfect_opt_pass_3: "%ε~γμένος εἴη", "%ε~γμένω εἰήτην", "%ε~γμένοι εἴησᾰν";
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    participle_perf: _, _, "%ε~γμενος";
    ),
    // Ληγω: 29 words (ἀγω, λεγω, φευγω, τρωγω, δῐᾰλεγω, φλεγω). Like Τᾰσσω, except in 54 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ληγω", 29, "~γω", like Τᾰσσω;
    stem lemma: "~γ- like βᾰπτῐζω";
    stem rest____: "~γ- like ἐχω";
    stem rest__: "~ξ- like ἐχω";
    stem rest________: "~ξ- like ἐχω";
    stem rest_: "~ξ- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_subj_act_3: _, derived, _;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    ),
    // Ψηφῐζω: 26 words (γνωρῐζω, ψηφῐζω, ὁρῐζω, ἁγνῐζω, σχηματῐζω, ἀτενῐζω). Like Χρημᾰτῐζω, except in 17 cells: perfect_ind_act_3 du, perfect_ind_mid_3 du, perfect_ind_pass_1 sg, perfect_ind_pass_2 sg, perfect_ind_pass_2 du, perfect_ind_pass_3 sg, ….
    conjugation!("ψηφῐζω", 26, "~ζω", like Χρημᾰτῐζω;
    stem rest___: "~κ- like σκοπεω";
    stem rest: "~σ- like σκοπεω";
    stem rest______: "~- like ψηφῐζω";
    stem rest___________: "~κ- like σκοπεω";
    stem rest_________: "~- like ψηφῐζω";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Στῐζω: 23 words (βᾰζω, ἁρπᾰζω, ἐλελῐζω, κρᾱζω, στῐζω, σφυζω). Like Ληγω, except in 56 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("στῐζω", 23, "~ζω", like Ληγω;
    stem lemma: "~ζ- like βᾰπτῐζω";
    stem rest____: "~ζ- like ἐχω";
    stem rest___: "~χ- like στηρῐζω";
    stem rest______: "~- like στηρῐζω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: "ἐ~γμαι", _, _;
    perfect_ind_mid_3: "ἐ~κται", derived, _;
    perfect_ind_pass_1: _, _, "ἐ~γμεθᾰ";
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_subj_mid_3: _, "ἐ~γμένω ἦτον", _;
    perfect_subj_pass_1: "ἐ~γμένος ὦ", _, "ἐ~γμένοι ὦμεν";
    perfect_subj_pass_2: "ἐ~γμένος ᾖς", "ἐ~γμένω ἦτον", "ἐ~γμένοι ἦτε";
    perfect_subj_pass_3: "ἐ~γμένος ᾖ", "ἐ~γμένω ἦτον", "ἐ~γμένοι ὦσῐ";
    perfect_opt_pass_1: "ἐ~γμένος εἴην", _, "ἐ~γμένοι εἴημεν";
    perfect_opt_pass_2: "ἐ~γμένος εἴης", "ἐ~γμένω εἴητον", "ἐ~γμένοι εἴητε";
    perfect_opt_pass_3: "ἐ~γμένος εἴη", "ἐ~γμένω εἰήτην", "ἐ~γμένοι εἴησᾰν";
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    participle_perf: _, _, "ἐ~γμενος";
    ),
    // Ποικιλλω: 22 words (ὑω, ἐλπω, ἐρδω, περικλειω, ἑλκῠω, αὐω). Like Βουλευω, except in 64 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ποικιλλω", 22, "~ω", like Βουλευω;
    stem lemma: "~- like ἐχω";
    stem rest____: "~- like ἐλπω";
    stem rest__: "~σ- like ἐχω";
    stem rest_______: "~σθησ- like ἐχω";
    stem rest________: "~σ- like ἐλπω";
    stem rest__________: "~σθην- like ἐλπω";
    stem rest_: "~σ- like ἐχω";
    stem rest_____: "~σθ- like ἐχω";
    stem rest: "~σ- like ἐλπω";
    stem rest______: "~σ- like ἐλπω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_act_3: _, derived, _;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Κᾰτηχεω: 22 words (εὐλογεω, ἠθεω, κᾰτηχεω, ἠχεω, ὀλιγαρχεω, μοναρχεω). Like Σκοπεω, except in 22 cells: imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_3 sg, imperfect_pass_3 pl, perfect_ind_act_3 du, ….
    conjugation!("κᾰτηχεω", 22, "~εω", like Σκοπεω;
    stem rest____: "~- like εὐλογεω";
    stem rest________: "~ησ- like εὐλογεω";
    stem rest__________: "~ηθην- like ἐλπω";
    stem rest___: "~ηκ- like εὐλογεω";
    stem rest: "~η- like εὐλογεω";
    stem rest______: "~η- like εὐλογεω";
    stem rest___________: "~ηκ- like εὐλογεω";
    stem rest_________: "~η- like εὐλογεω";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Φᾰντᾰζω: 21 words (σχῐζω, ὀνομᾰζω, σχεδῐᾰζω, ἁρμοζω, κτῐζω, σπιζω). Like Ψηφῐζω, except in 4 cells: future_ind_act_3 du, future_ind_mid_3 du, future_opt_act_2 pl, future_opt_act_3 pl.
    conjugation!("φᾰντᾰζω", 21, "~ζω", like Ψηφῐζω;
    stem rest__: "~σ- like ποιεω";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_opt_act_2: _, _, derived;
    future_opt_act_3: _, _, derived;
    ),
    // Δᾰπτω: 20 words (κρῠπτω, κᾰμπτω, ἁπτω, κῠπτω, θᾰπτω, ἀποκαλυπτω). Like Βουλευω, except in 56 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("δᾰπτω", 20, "~πτω", like Βουλευω;
    stem lemma: "~πτ- like βᾰπτῐζω";
    stem rest____: "~πτ- like ἐχω";
    stem rest__: "~ψ- like ἐχω";
    stem rest_______: "~φθησ- like ἐχω";
    stem rest________: "~ψ- like ἐχω";
    stem rest__________: "~φθην- like ἐχω";
    stem rest_: "~ψ- like ἐχω";
    stem rest_____: "~φθ- like ἐχω";
    stem rest___: "~φ- like ἐχω";
    stem rest___________: "~φειν- like ἐχω";
    stem rest_________: "~- like ἁπτω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    future_perfect_ind_mid_1: "%ε~φομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~φεσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~φομεθᾰ";
    future_perfect_ind_pass_2: "%ε~φῃ", "%ε~φεσθον", "%ε~φεσθε";
    future_perfect_ind_pass_3: "%ε~φεται", "%ε~φεσθον", "%ε~φονται";
    future_perfect_opt_pass_1: "%ε~φοιμην", _, "%ε~φοιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~φοιο", "%ε~φοισθον", "%ε~φοισθε";
    future_perfect_opt_pass_3: "%ε~φοιτο", "%ε~φοισθην", "%ε~φοιντο";
    infinitive_futp: _, _, "%ε~φεσθαι";
    participle_futp: _, _, "%ε~φομενος";
    ),
    // Νῑκαω: 18 words (νῑκαω, λικμαω, ὀργαω, μελεταω, κομαω, εἰσοραω). Like Φοιτᾰω, except in 30 cells: future_ind_act_3 du, future_ind_mid_3 du, pluperfect_act_1 sg, pluperfect_mid_1 sg, pluperfect_mid_2 sg, pluperfect_mid_2 du, ….
    conjugation!("νῑκαω", 18, "~αω", like Φοιτᾰω;
    stem rest__: "~ησ- like ἐχω";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    future_perfect_ind_mid_1: derived, _, _;
    future_perfect_ind_mid_3: _, derived, _;
    future_perfect_ind_pass_1: _, _, derived;
    future_perfect_ind_pass_2: derived, derived, derived;
    future_perfect_ind_pass_3: derived, derived, derived;
    future_perfect_opt_pass_1: derived, _, derived;
    future_perfect_opt_pass_2: derived, derived, derived;
    future_perfect_opt_pass_3: derived, derived, derived;
    infinitive_futp: _, _, derived;
    participle_futp: _, _, derived;
    ),
    // Πεμπω: 18 words (λᾰμπω, βλεπω, ἐρειπω, λειπω, ἑρπω, πεμπω).
    conjugation!("πεμπω", 18, "~~πω";
    stem lemma: "~~π- like βᾰπτῐζω";
    stem rest____: "~~π- like ἐχω";
    stem rest__: "~~ψ- like ἐχω";
    stem rest_______: "~~φθησ- like ἐχω";
    stem rest________: "~~ψ- like ἐχω";
    stem rest__________: "~~φθην- like ἐχω";
    stem rest_: "~~ψ- like ἐχω";
    stem rest_____: "~~φθ- like ἐχω";
    stem rest___: "~πο~φ- like πεμπω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: "%ε~~μαι", _, _;
    perfect_ind_mid_2: "%ε~~ψαι", "%ε~~φθον", _;
    perfect_ind_mid_3: "%ε~~πται", _, "%ε~~πᾰται";
    perfect_ind_pass_1: _, _, "%ε~~μεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~~φθε";
    perfect_subj_mid_3: _, "%ε~~μένω ἦτον", _;
    perfect_subj_pass_1: "%ε~~μένος ὦ", _, "%ε~~μένοι ὦμεν";
    perfect_subj_pass_2: "%ε~~μένος ᾖς", "%ε~~μένω ἦτον", "%ε~~μένοι ἦτε";
    perfect_subj_pass_3: "%ε~~μένος ᾖ", "%ε~~μένω ἦτον", "%ε~~μένοι ὦσῐ";
    perfect_opt_pass_1: "%ε~~μένος εἴην", _, "%ε~~μένοι εἴημεν";
    perfect_opt_pass_2: "%ε~~μένος εἴης", "%ε~~μένω εἴητον", "%ε~~μένοι εἴητε";
    perfect_opt_pass_3: "%ε~~μένος εἴη", "%ε~~μένω εἰήτην", "%ε~~μένοι εἴησᾰν";
    perfect_imp_mid_2: _, _, "%ε~~φθε";
    perfect_imp_mid_3: _, _, "%ε~~φθων";
    perfect_imp_pass_2: "%ε~~ψο", _, "%ε~~φθε";
    perfect_imp_pass_3: "%ε~~φθω", "%ε~~φθων", "%ε~~φθων";
    pluperfect_act_1: "ἐ~πο~φειν", _, _;
    pluperfect_mid_1: "ἐ%ε~~μην", _, _;
    pluperfect_mid_2: "ἐ%ε~~ψο", "ἐ%ε~~φθον", _;
    pluperfect_mid_3: "ἐ%ε~~πτο", _, "ἐ%ε~~πᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~~φθε";
    pluperfect_pass_3: _, "ἐ%ε~~φθην", _;
    infinitive_perf: _, "%ε~~φθαι", _;
    participle_perf: _, _, "%ε~~μενος";
    ),
    // Ὀρχεομαι: 18 words (ὀρχεομαι, ἀρνεομαι, ἱκνεομαι, ἐπεξηγεομαι, ἀφῐκνεομαι, ὑπῐσχνεομαι). Like Ποιεω, except in 86 cells: ind_act_3 pl, ind_pass_2 sg, subj_act_3 pl, opt_act_1 sg, opt_act_1 pl, opt_act_2 sg, ….
    conjugation!("ὀρχεομαι", 18, "~εομαι", like Ποιεω;
    stem rest__: "~ησ- like ἑπομαι";
    stem rest___: "~ηκ- like ἐχω";
    stem rest: "~η- like βουλευω";
    ind_act_3: _, _, "~ουσῐ";
    ind_pass_2: "~ει", _, _;
    subj_act_3: _, _, "~ωσῐ";
    opt_act_1: "~οιην", _, "~οιμεν";
    opt_act_2: "~οιης", "~οιτον", "~οιτε";
    opt_act_3: "~οιη", "~οιτην", "~οιεν";
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: _, derived, _;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, _;
    future_opt_act_3: derived, derived, _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    perfect_ind_act_3: derived, derived, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_act_3: _, _, derived;
    perfect_subj_mid_3: _, _, derived;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    future_perfect_ind_mid_1: derived, _, _;
    future_perfect_ind_mid_3: _, derived, _;
    future_perfect_ind_pass_1: _, _, derived;
    future_perfect_ind_pass_2: derived, derived, derived;
    future_perfect_ind_pass_3: derived, derived, derived;
    future_perfect_opt_pass_1: derived, _, derived;
    future_perfect_opt_pass_2: derived, derived, derived;
    future_perfect_opt_pass_3: derived, derived, derived;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    infinitive_futp: _, _, derived;
    participle_aor: derived, _, _;
    participle_futp: _, _, derived;
    ),
    // Θωᾰω: 16 words (πεπτω, τᾰττω, ἀλεγω, ϝερρω, φᾱρω, θωαδδω). Like Κρῑνομαι, except in 103 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("θωᾰω", 16, "~ω", like Κρῑνομαι;
    stem rest____: "~- like ἐχω";
    ind_act_1: "~ω", _, "~ομεν";
    ind_act_2: "~εις", "~ετον", _;
    ind_act_3: "~ει", _, "~ουσῐ|~ουσῐν";
    ind_mid_1: "~ομαι", _, _;
    ind_mid_2: "~εαι", _, _;
    ind_pass_1: derived, _, _;
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", _, _;
    subj_act_3: "~ῃ", "~ητον", "~ωσῐ|~ωσῐν";
    subj_mid_2: "~ηαι", _, _;
    subj_pass_2: "~ηαι", _, _;
    opt_act_1: "~οιμῐ", _, "~οιμεν";
    opt_act_2: "~οις", "~οιτον", "~οιτε";
    opt_act_3: "~οι", "~οιτην", "~οιεν";
    imp_act_2: "~ε", _, "~ετε";
    imp_act_3: _, _, "~οντων";
    imp_mid_2: "~εο", _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_mid_1: derived, _, _;
    future_opt_mid_3: derived, derived, _;
    aorist_ind_mid_1: derived, _, _;
    aorist_ind_mid_2: derived, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_mid_1: _, _, derived;
    aorist_subj_mid_2: derived, derived, _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_opt_mid_1: derived, _, _;
    aorist_imp_mid_2: derived, derived, _;
    aorist_imp_mid_3: _, _, derived;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_mid_1: derived, _, _;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_mid_3: _, derived, _;
    perfect_subj_pass_1: derived, _, derived;
    perfect_subj_pass_2: derived, derived, derived;
    perfect_subj_pass_3: derived, derived, derived;
    perfect_opt_pass_1: derived, _, derived;
    perfect_opt_pass_2: derived, derived, derived;
    perfect_opt_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_pres: "~ειν", _, _;
    infinitive_perf: _, derived, _;
    participle_pres: "~ων", _, _;
    participle_aor: _, derived, _;
    participle_perf: _, _, derived;
    ),
    // Λογῐζομαι: 15 words (λογῐζομαι, χᾰρῐζομαι, λᾰζομαι, δῐᾰγωνῐζομαι, λῐᾰζομαι, πλειστηρῐζομαι). Like Χρημᾰτῐζω, except in 49 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("λογῐζομαι", 15, "~ζομαι", like Χρημᾰτῐζω;
    stem lemma: "~ζ- like βᾰπτῐζω";
    stem rest____: "~ζ- like ἐχω";
    stem rest__: "~- like λογῐζομαι";
    stem rest________: "~σ- like ἐχω";
    stem rest_: "~σ- like ἐχω";
    stem rest___: "~κ- like ἐχω";
    stem rest: "~σ- like βουλευω";
    stem rest___________: "~κειν- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_2: _, _, derived;
    future_opt_act_3: _, _, derived;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: derived, derived, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_act_3: _, _, derived;
    perfect_subj_mid_3: _, _, derived;
    pluperfect_act_2: derived, _, _;
    pluperfect_act_3: derived, _, _;
    ),
    // Πειρᾰζω: 15 words (πᾰρουσῐᾰζω, πλεονᾰζω, θειᾰζω, γυμνᾰζω, πειρᾰζω, πλησῐᾰζω). Like Χρημᾰτῐζω, except in 9 cells: future_ind_act_3 du, future_ind_mid_3 du, future_opt_act_2 pl, future_opt_act_3 pl, pluperfect_pass_1 sg, pluperfect_pass_2 sg, ….
    conjugation!("πειρᾰζω", 15, "~ζω", like Χρημᾰτῐζω;
    stem rest__: "~σ- like ποιεω";
    stem rest___________: "~κ- like σκοπεω";
    stem rest_________: "~- like ψηφῐζω";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_opt_act_2: _, _, derived;
    future_opt_act_3: _, _, derived;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    ),
    // Πᾰρῐστημῐ: 15 words (ἐξῐστημῐ, ἐπεντῐθημῐ, τῐθημῐ, πᾰρῐστημῐ, ἀνῐστημῐ, κᾰθῐστημῐ). Like Πεμπω, except in 103 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πᾰρῐστημῐ", 15, "~ῐ~ημῐ", like Πεμπω;
    stem rest____: "~ῑ~- like ἀνῐστημῐ";
    stem rest__: "~ᾰ~ησ- like ἐχω";
    stem rest_______: "~ᾰ~ησ- like ἐχω";
    stem rest________: "~ε~ησ- like ἐλπω";
    stem rest__________: "~ε~ᾰθην- like ἐλπω";
    stem rest_: "~ᾰ~ησ- like ἐχω";
    stem rest_____: "~ᾰ~ᾰθ- like ἐχω";
    stem rest___: "~ε~ᾰκ- like πεμπω";
    stem rest: "~ε~- like ἀνῐστημῐ";
    stem rest______: "~ε~ᾰ- like εὐλογεω";
    stem rest_________: "~ε~ᾰ- like εὐλογεω";
    ind_act_1: "~ῐ~ημῐ", _, "~ῐ~ᾰμεν";
    ind_act_2: "~ῐ~ης", "~ῐ~ᾰτον", _;
    ind_act_3: "~ῐ~ησῐ|~ῐ~ησῐν", _, "~ῐ~ᾱσῐ|~ῐ~ᾱσῐν";
    ind_mid_1: _, _, "~ῐ~ᾰμεθᾰ";
    ind_mid_2: "~ῐ~ᾰσαι", _, _;
    ind_mid_3: _, "~ῐ~ᾰσθον", _;
    ind_pass_1: "~ῐ~ᾰμαι", _, _;
    ind_pass_2: _, "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    ind_pass_3: "~ῐ~ᾰται", "~ῐ~ᾰσθον", "~ῐ~ᾰνται";
    subj_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    subj_act_2: "~ῐ~ῃς", _, _;
    subj_act_3: "~ῐ~ῃ", "~ῐ~ητον", "~ῐ~ωσῐ|~ῐ~ωσῐν";
    subj_mid_1: _, _, "~ῐ~ωμεθᾰ";
    subj_mid_2: _, "~ῐ~ησθον", _;
    subj_mid_3: "~ῐ~ηται", _, "~ῐ~ωνται";
    subj_pass_1: "~ῐ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῐ~ησθε";
    opt_act_1: "~ῐ~αιην", _, "~ῐ~αιμεν|~ῐ~αιημεν";
    opt_act_2: "~ῐ~αιης", "~ῐ~αιτον|~ῐ~αιητον", "~ῐ~αιτε|~ῐ~αιητε";
    opt_act_3: "~ῐ~αιη", "~ῐ~αιτην|~ῐ~αιητην", "~ῐ~αιεν|~ῐ~αιησᾰν";
    opt_mid_1: "~ῐ~αιμην", _, "~ῐ~αιμεθᾰ";
    opt_mid_2: "~ῐ~αιο", "~ῐ~αισθον", "~ῐ~αισθε";
    opt_mid_3: "~ῐ~αιτο", "~ῐ~αισθην", "~ῐ~αιντο";
    imp_act_2: "~ῐ~η", _, "~ῐ~ᾰτε";
    imp_act_3: _, _, "~ῐ~ᾰντων";
    imp_mid_2: "~ῐ~ᾰσο|~ῐ~ω", "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    imp_mid_3: _, _, "~ῐ~ᾰσθων";
    imp_pass_2: _, "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    imp_pass_3: "~ῐ~ᾰσθω", "~ῐ~ᾰσθων", "~ῐ~ᾰσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "~ε~ᾰκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    future_perfect_ind_act_1: "~ε~ηξω", _, "~ε~ηξομεν";
    future_perfect_ind_act_2: "~ε~ηξεις", "~ε~ηξετον", "~ε~ηξετε";
    future_perfect_ind_act_3: "~ε~ηξει", _, "~ε~ηξουσῐ";
    future_perfect_ind_pass_1: derived, _, _;
    future_perfect_opt_act_1: "~ε~ηξοιμῐ", _, "~ε~ηξοιμεν";
    future_perfect_opt_act_2: "~ε~ηξοις", "~ε~ηξοιτον", "~ε~ηξοιτε";
    future_perfect_opt_act_3: "~ε~ηξοι", "~ε~ηξοιτην", "~ε~ηξοιεν";
    infinitive_pres: "~ῐ~ᾰναι", _, "~ῐ~ᾰσθαι";
    infinitive_perf: _, _, derived;
    infinitive_futp: "~ε~ηξειν", _, _;
    participle_pres: "~ῐ~ᾱς", _, "~ῐ~ᾰμενος";
    participle_futp: "~ε~ηξων", _, _;
    ),
    // Σκᾰνδᾰλῐζω: 13 words (σκᾰνδᾰλῐζω, δειγμᾰτῐζω, γογγῠζω, βλυζω, τρῑζω, πλειστηριᾰζω). Like Θερᾰπευω with `ˇ` after each pattern, except in 31 cells: future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, aorist_opt_pass_3 du, ….
    conjugation!("σκᾰνδᾰλῐζω", 13, "~ωˇ", like Θερᾰπευω with "ˇ";
    future_ind_pass_2: derived, _, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: "%ε~κεˇ", _, "%ε~κᾱσῐˇ";
    perfect_subj_act_3: _, _, "%ε~κωσῐˇ";
    perfect_subj_mid_3: _, _, "%ε~μένοι ὦσῐˇ";
    perfect_opt_act_1: "%ε~κοιμῐˇ", _, _;
    perfect_opt_act_2: "%ε~κοιςˇ", _, _;
    perfect_opt_act_3: "%ε~κοιˇ", _, _;
    perfect_imp_act_2: derived, _, _;
    pluperfect_act_1: "ἐ%ε~κεινˇ", _, "ἐ%ε~κεμενˇ";
    pluperfect_act_2: "ἐ%ε~κειςˇ", "ἐ%ε~κετονˇ", "ἐ%ε~κετεˇ";
    pluperfect_act_3: "ἐ%ε~κειˇ", "ἐ%ε~κετηνˇ", "ἐ%ε~κεσᾰνˇ";
    pluperfect_mid_1: "ἐ%ε~μηνˇ", _, "ἐ%ε~μεθᾰˇ";
    pluperfect_mid_2: "ἐ%ε~σοˇ", "ἐ%ε~σθονˇ", "ἐ%ε~σθεˇ";
    pluperfect_mid_3: "ἐ%ε~τοˇ", "ἐ%ε~σθηνˇ", "ἐ%ε~ντοˇ";
    ),
    // Μνημονευω: 13 words (στεφω, ἀγχω, ἀροω, ἀρτῡω, μνημονευω, πταιω). Like Βουλευω, except in 18 cells: perfect_ind_act_3 du, perfect_ind_mid_3 du, perfect_ind_pass_1 sg, perfect_ind_pass_2 sg, perfect_ind_pass_2 du, perfect_ind_pass_3 sg, ….
    conjugation!("μνημονευω", 13, "~ω", like Βουλευω;
    stem rest___: "~κ- like σκοπεω";
    stem rest: "~- like σκοπεω";
    stem rest______: "~- like σκοπεω";
    stem rest_________: "~- like σκοπεω";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "ἐ~κειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Τεχναομαι: 13 words (μναομαι, καυχᾰομαι, ἀραομαι, χρᾰομαι, κτᾰομαι, μᾰσαομαι). Like Νῑκαω, except in 93 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("τεχναομαι", 13, "~αομαι", like Νῑκαω;
    stem rest_______: "~ησ- like ἐχω";
    ind_act_1: "~αομαι", _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: derived, _, derived;
    ind_mid_1: "~ωμαι", _, _;
    ind_pass_1: derived, _, _;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_1: "~ωμαι", _, _;
    subj_mid_2: "~ᾳ", _, _;
    subj_pass_1: derived, _, _;
    subj_pass_2: "~ᾳ", _, _;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imp_mid_3: "~ᾱσθω", _, _;
    imp_pass_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    imperfect_mid_2: _, "ἐ~ᾱσθον", _;
    imperfect_pass_2: _, derived, _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: _, derived, _;
    future_ind_act_3: derived, _, derived;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, _;
    future_opt_act_3: derived, derived, _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_subj_act_3: _, derived, _;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    participle_pres: derived, _, _;
    participle_aor: derived, _, _;
    ),
    // Δῐοικεω: 12 words (δῐοικοδομεω, ἐνοικεω, προσοικεω, κᾰτοικοφθορεω, κατοικονομεω, κᾰτοικοδομεω). Like Πεμπω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("δῐοικεω", 12, "~οι~εω", like Πεμπω;
    stem lemma: "~οι~- like ποιεω";
    stem rest____: "~ῳ~- like εὐλογεω";
    stem rest__: "~οι~ησ- like ποιεω";
    stem rest_______: "~οι~ηθησ- like ποιεω";
    stem rest________: "~ῳ~ησ- like εὐλογεω";
    stem rest__________: "~ῳ~ηθην- like ἐλπω";
    stem rest_: "~οι~ησ- like ποιεω";
    stem rest_____: "~οι~ηθ- like ποιεω";
    stem rest___: "~ῳ~ηκ- like εὐλογεω";
    stem rest: "~ῳ~η- like εὐλογεω";
    stem rest______: "~ῳ~η- like εὐλογεω";
    stem rest___________: "~ῳ~ηκ- like εὐλογεω";
    stem rest_________: "~ῳ~η- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ῥωννῡμῐ: 12 words (ῥωννῡμῐ, ἑννῡμῐ, πετᾰννῡμῐ, σβεννῡμῐ, κερᾰννῡμῐ, ζωννῡμῐ). Like Μνημονευω, except in 52 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ῥωννῡμῐ", 12, "~ννῡμῐ", like Μνημονευω;
    stem lemma: "~νν- like σβεννῡμῐ";
    stem rest____: "~νν- like σβεννῡμῐ";
    stem rest__: "~σ- like ἐχω";
    stem rest_______: "~σθησ- like ἐχω";
    stem rest________: "~σ- like ἐχω";
    stem rest__________: "~σθην- like ἐχω";
    stem rest_: "~σ- like ἐχω";
    stem rest_____: "~σθ- like ἐχω";
    stem rest: "~- like σβεννῡμῐ";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_act_3: _, derived, derived;
    perfect_subj_mid_3: _, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    pluperfect_act_1: derived, _, _;
    ),
    // Πορσῡνω: 12 words (πληθῡνω, ἀμῡνω, αἰσχῡνω, πλῡνω, μολῡνω, ὀξῡνω). Like Ληγω, except in 49 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("πορσῡνω", 12, "~ῡνω", like Ληγω;
    stem lemma: "~ῡν- like βουλευω";
    stem rest____: "~ῡν- like βουλευω";
    stem rest__: "~ῠν- like χρημᾰτῐζω";
    stem rest_______: "~ῠνθησ- like ἐχω";
    stem rest________: "~ῡν- like ποιεω";
    stem rest__________: "~ῡνθην- like ἐχω";
    stem rest_: "~ῡν- like ἐχω";
    stem rest_____: "~ῡνθ- like ἐχω";
    stem rest___: "~ῠγκ- like ἐχω";
    stem rest: "~ῠ- like πορσῡνω";
    stem rest______: "~ῠν- like πορσῡνω";
    stem rest___________: "~ῠγκειν- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_mid_1: "ἐ%ε~ῠσμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ῠνσο", "ἐ%ε~ῠνθον", _;
    pluperfect_mid_3: "ἐ%ε~ῠντο", _, "ἐ%ε~ῠνᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~ῠσμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ῠνθε";
    pluperfect_pass_3: _, "ἐ%ε~ῠνθην", _;
    infinitive_perf: _, _, derived;
    ),
    // Ἐπᾰϊω: 12 words (ὑπᾰρχω, ὑπᾰκουω, δῐᾰνῠω, ἐξᾰρχω, ἐξᾰρτῡω, ἀπᾰρεσκω). Like Πεμπω, except in 54 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἐπᾰϊω", 12, "~ᾰ~ω", like Πεμπω;
    stem lemma: "~ᾰ~- like βουλευω";
    stem rest____: "~η~- like ἐλπω";
    stem rest__: "~ᾰ~σ- like ἐχω";
    stem rest_______: "~ᾰ~σθησ- like ἐχω";
    stem rest________: "~η~σ- like εὐλογεω";
    stem rest__________: "~η~σθην- like ἐλπω";
    stem rest_: "~ᾰ~σ- like ποιεω";
    stem rest_____: "~ᾰ~σθ- like ἐχω";
    stem rest___: "~η~κ- like πεμπω";
    stem rest: "~η~σ- like ἐλπω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_2: "~η~σαι", "~η~σθον", _;
    perfect_ind_mid_3: _, _, "~η~τᾰται";
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, "~η~σθε";
    perfect_ind_pass_3: derived, _, _;
    perfect_imp_mid_2: _, _, "~η~σθε";
    perfect_imp_mid_3: _, _, "~η~σθων";
    perfect_imp_pass_2: "~η~σο", _, "~η~σθε";
    perfect_imp_pass_3: "~η~σθω", "~η~σθων", "~η~σθων";
    pluperfect_act_1: "~η~κειν", _, _;
    pluperfect_mid_1: "~η~σμην", _, _;
    pluperfect_mid_2: "~η~σο", "~η~σθον", _;
    pluperfect_mid_3: "~η~στο", _, "~η~τᾰτο";
    pluperfect_pass_1: _, _, "~η~σμεθᾰ";
    pluperfect_pass_2: _, _, "~η~σθε";
    pluperfect_pass_3: _, "~η~σθην", _;
    infinitive_perf: _, "~η~σθαι", _;
    ),
    // Πῠρσαινω: 11 words (ἐοικᾰ, κειμαι, δυναμαι, οἰδᾰ, δει, ἐτλην). Like Θωᾰω, except in 22 cells: ind_act_3 du, ind_mid_1 sg, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, ….
    conjugation!("πῠρσαινω", 11, "~", like Θωᾰω;
    stem lemma: "~- like βᾰπτῐζω";
    ind_act_3: _, derived, _;
    ind_mid_1: derived, _, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    ),
    // Κᾰθιδρῡω: 11 words (ἰᾱτρευω, ἰσχῡω, ἑλκω, ἱκω, κᾰθιδρῡω, εἰκω). Like Ποικιλλω, except in 45 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("κᾰθιδρῡω", 11, "~ω", like Ποικιλλω;
    stem lemma: "~- like βουλευω";
    stem rest__: "~σ- like ποιεω";
    stem rest________: "~σ- like εὐλογεω";
    stem rest_: "~σ- like ποιεω";
    stem rest___: "~κ- like εὐλογεω";
    stem rest: "~- like εὐλογεω";
    stem rest______: "~- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Σαινω: 11 words (φαινω, ξαινω, ὑφαινω, ὁρμαινω, χᾰλεπαινω, σαινω). Like Ληγω, except in 63 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("σαινω", 11, "~αινω", like Ληγω;
    stem lemma: "~αιν- like βᾰπτῐζω";
    stem rest____: "~αιν- like ἐχω";
    stem rest__: "~ᾰν- like λογῐζομαι";
    stem rest_______: "~ᾰνθησ- like ἐχω";
    stem rest________: "~ην- like ἐχω";
    stem rest__________: "~ᾰνθην- like ἐχω";
    stem rest_: "~ην- like ἐχω";
    stem rest_____: "~ᾰνθ- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_1: "ἐ~ασμαι", _, _;
    perfect_ind_mid_2: "ἐ~ανσαι", "ἐ~ανθον", _;
    perfect_ind_mid_3: "ἐ~ανται", _, "ἐ~ανᾰται";
    perfect_ind_pass_1: _, _, "ἐ~ασμεθᾰ";
    perfect_ind_pass_2: _, _, "ἐ~ανθε";
    perfect_subj_mid_3: _, "ἐ~ασμένω ἦτον", _;
    perfect_subj_pass_1: "ἐ~ασμένος ὦ", _, "ἐ~ασμένοι ὦμεν";
    perfect_subj_pass_2: "ἐ~ασμένος ᾖς", "ἐ~ασμένω ἦτον", "ἐ~ασμένοι ἦτε";
    perfect_subj_pass_3: "ἐ~ασμένος ᾖ", "ἐ~ασμένω ἦτον", "ἐ~ασμένοι ὦσῐ";
    perfect_opt_pass_1: "ἐ~ασμένος εἴην", _, "ἐ~ασμένοι εἴημεν";
    perfect_opt_pass_2: "ἐ~ασμένος εἴης", "ἐ~ασμένω εἴητον", "ἐ~ασμένοι εἴητε";
    perfect_opt_pass_3: "ἐ~ασμένος εἴη", "ἐ~ασμένω εἰήτην", "ἐ~ασμένοι εἴησᾰν";
    perfect_imp_mid_2: _, _, "ἐ~ανθε";
    perfect_imp_mid_3: _, _, "ἐ~ανθων";
    perfect_imp_pass_2: "ἐ~ανσο", _, "ἐ~ανθε";
    perfect_imp_pass_3: "ἐ~ανθω", "ἐ~ανθων", "ἐ~ανθων";
    infinitive_perf: _, "ἐ~ᾱνθαι", _;
    participle_perf: _, _, "ἐ~ασμενος";
    ),
    // Ἀνᾰγω: 11 words (σῠνᾰγω, ἐξᾰγω, ὑπᾰγω, ἀνᾰγω, πᾰρᾰγω, ἐπᾰγω). Like Ληγω, except in 41 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἀνᾰγω", 11, "~ᾰγω", like Ληγω;
    stem lemma: "~ᾰγ- like βουλευω";
    stem rest____: "~ηγ- like ἐλπω";
    stem rest__: "~ᾰξ- like ποιεω";
    stem rest_______: "~ᾰχθησ- like ποιεω";
    stem rest________: "~ηγᾰγ- like ἀνᾰγω";
    stem rest__________: "~ηχθην- like ἐλπω";
    stem rest_: "~ᾰγᾰγ- like ἀνᾰγω";
    stem rest_____: "~ᾰχθ- like ποιεω";
    stem rest___: "~ηχ- like πεμπω";
    stem rest: "~η- like ἀνᾰγω";
    stem rest______: "~η- like ἀνᾰγω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Πῡθω: 11 words (πειθω, περθω, ληθω, κλωθω, κευθω, δῐᾰπερθω). Like Πειρᾰζω, except in 64 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("πῡθω", 11, "~θω", like Πειρᾰζω;
    stem lemma: "~θ- like ἐχω";
    stem rest____: "~θ- like ἐχω";
    stem rest__: "~σ- like ἐχω";
    stem rest: "~σ- like βουλευω";
    stem rest_________: "~σ- like πειθω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_2: derived, _, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_mid_3: _, _, "%ε~θᾰται";
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_act_3: _, derived, derived;
    perfect_subj_mid_3: _, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    pluperfect_act_1: derived, _, _;
    pluperfect_act_2: derived, _, _;
    pluperfect_act_3: derived, _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    ),
    // Δαμνημῐ: 11 words (ἱστημῐ, ἱημῐ, δαμνημῐ, πῐμπλημῐ, φημῐ, διατιθημῐ). Like Πῠρσαινω, except in 62 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("δαμνημῐ", 11, "~ημῐ", like Πῠρσαινω;
    ind_act_1: "~ημῐ", _, "~ᾰμεν";
    ind_act_2: "~ης", "~ᾰτον", _;
    ind_act_3: "~ησῐ", _, "~ᾱσῐ";
    ind_mid_1: _, _, "~ᾰμεθᾰ";
    ind_mid_2: "~ᾰσαι", _, _;
    ind_mid_3: _, "~ᾰσθον", _;
    ind_pass_1: "~ᾰμαι", _, _;
    ind_pass_2: _, "~ᾰσθον", "~ᾰσθε";
    ind_pass_3: "~ᾰται", "~ᾰσθον", "~ᾰνται";
    subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    opt_act_1: "~αιην", _, "~αιμεν|~αιημεν";
    opt_act_2: "~αιης", "~αιτον|~αιητον", "~αιτε|~αιητε";
    opt_act_3: "~αιη", "~αιτην|~αιητην", "~αιεν|~αιησᾰν";
    opt_mid_1: "~αιμην", _, "~αιμεθᾰ";
    opt_mid_2: "~αιο", "~αισθον", "~αισθε";
    opt_mid_3: "~αιτο", "~αισθην", "~αιντο";
    imp_act_2: "~η", _, "~ᾰτε";
    imp_act_3: _, _, "~ᾰντων";
    imp_mid_2: "~ᾰσο", "~ᾰσθον", "~ᾰσθε";
    imp_mid_3: _, _, "~ᾰσθων";
    imp_pass_2: _, "~ᾰσθον", "~ᾰσθε";
    imp_pass_3: "~ᾰσθω", "~ᾰσθων", "~ᾰσθων";
    imperfect_act_1: "ἐ~ην", _, _;
    imperfect_act_2: "ἐ~ης", "ἐ~ᾰτον", "ἐ~ᾰτε";
    imperfect_act_3: "ἐ~η", _, "ἐ~ᾰσᾰν";
    imperfect_mid_1: "ἐ~ᾰμην", _, "ἐ~ᾰμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰσο", _, _;
    imperfect_mid_3: "ἐ~ᾰτο", _, "ἐ~ᾰντο";
    imperfect_pass_2: _, "ἐ~ᾰσθον", "ἐ~ᾰσθε";
    imperfect_pass_3: _, "ἐ~ᾰσθην", _;
    infinitive_pres: "~ᾰναι", _, "~ᾰσθαι";
    participle_pres: "~ᾱς", _, "~ᾰμενος";
    ),
    // Ζευγνῡμῐ: 10 words (πηγνῡμῐ, μῑγνῡμῐ, ζευγνῡμῐ, ἀνοιγνῡμῐ, οἰγνῡμῐ, ὀμοργνῡμῐ). Like Ληγω, except in 27 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ζευγνῡμῐ", 10, "~γνῡμῐ", like Ληγω;
    stem lemma: "~γν- like σβεννῡμῐ";
    stem rest____: "~γν- like σβεννῡμῐ";
    stem rest___: "~χ- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    perfect_ind_act_3: _, derived, _;
    ),
    // Ὁμᾰρτεω: 10 words (πωλεω, αἱρεω, ἀγραυλεω, αἰνεω, βῑνεω, ὑμνεω). Like Μελλω, except in 26 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ὁμᾰρτεω", 10, "~εω", like Μελλω;
    stem lemma: "~ε- like βᾰπτῐζω";
    stem rest____: "~ε- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    ),
    // Χεω: 10 words (πνεω, θεω, πλεω, χεω, νεω, ἀγρεω). Like Πῠρσαινω, except in 58 cells: ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_mid_3 du, ind_pass_2 du, ind_pass_2 pl, ….
    conjugation!("χεω", 10, "~ω", like Πῠρσαινω;
    stem rest__: "~υσ- like ἐχω";
    stem rest__________: "~υσθην- like ἐχω";
    stem rest_____: "~υσθ- like ἐχω";
    stem rest___: "~υκ- like ἐχω";
    stem rest: "~υσ- like βουλευω";
    stem rest______: "~υσ- like χεω";
    ind_act_2: "~ις", "~ιτον", _;
    ind_act_3: "~ι", _, _;
    ind_mid_3: _, "~ισθον", _;
    ind_pass_2: _, "~ισθον", "~ισθε";
    ind_pass_3: "~ιται", "~ισθον", _;
    imp_act_2: "~ι", _, "~ιτε";
    imp_mid_2: _, "~ισθον", "~ισθε";
    imp_pass_2: _, "~ισθον", "~ισθε";
    imp_pass_3: "~ισθω", "~ισθων", _;
    imperfect_act_2: "ἐ~ις", "ἐ~ιτον", "ἐ~ιτε";
    imperfect_act_3: "ἐ~ι", _, _;
    imperfect_mid_3: "ἐ~ιτο", _, _;
    imperfect_pass_2: _, "ἐ~ισθον", "ἐ~ισθε";
    imperfect_pass_3: _, "ἐ~ισθην", _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_ind_act_1: "ἐ~υσᾰ", _, "ἐ~υσᾰμεν";
    aorist_ind_act_2: "ἐ~υσᾰς", "ἐ~υσᾰτον", _;
    aorist_ind_act_3: "ἐ~υσε", _, "ἐ~υσᾰν";
    aorist_subj_act_1: "~υσω", _, _;
    aorist_subj_act_2: "~υσῃς", "~υσητον", _;
    aorist_opt_act_1: "~υσαιμῐ", _, "~υσαιμεν";
    aorist_opt_act_2: "~υσειᾰς", _, _;
    aorist_opt_act_3: "~υσειε", _, "~υσειᾰν";
    aorist_imp_act_2: "~υσον", _, "~υσᾰτε";
    aorist_imp_act_3: _, _, "~υσᾰντων";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_pres: "~ιν", _, "~ισθαι";
    infinitive_aor: "~υσαι", _, _;
    infinitive_perf: _, _, derived;
    participle_aor: "~υσᾱς", _, _;
    ),
    // Ἀφῑημῐ: 10 words (ἀφῑημῐ, σῠνῑημῐ, ἐπᾰνῐημῐ, προϊημῐ, ἀνιημῐ, ξῠνῑημῐ). Like Νῑκαω, except in 107 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀφῑημῐ", 10, "~ῑημῐ", like Νῑκαω;
    stem rest____: "~ῑ- like ἀφῑημῐ";
    stem rest__: "~ησ- like ποιεω";
    stem rest________: "~ηκ- like εὐλογεω";
    stem rest___: "~εικ- like πεμπω";
    stem rest: "~ει- like ἐλπω";
    stem rest______: "~ει- like εὐλογεω";
    ind_act_1: "~ῑημῐ", _, "~ῑεμεν";
    ind_act_2: "~ῑης", "~ῑετον", _;
    ind_act_3: "~ῑησῐ|~ῑησῐν", _, "~ῑᾱσι|~ῑᾱσιν";
    ind_mid_1: _, _, "~ῑεμεθᾰ";
    ind_mid_2: "~ῑεσαι", _, _;
    ind_mid_3: _, "~ῑεσθον", _;
    ind_pass_1: "~ῑεμαι", _, _;
    ind_pass_2: _, "~ῑεσθον", "~ῑεσθε";
    ind_pass_3: "~ῑεται", "~ῑεσθον", "~ῑενται";
    subj_act_1: "~ῑω", _, "~ῑωμεν";
    subj_act_2: "~ῑῃς", _, _;
    subj_act_3: "~ῑῃ", "~ῑητον", "~ῑωσῐ|~ῑωσῐν";
    subj_mid_1: _, _, "~ῑωμεθᾰ";
    subj_mid_2: _, "~ῑησθον", _;
    subj_mid_3: "~ῑηται", _, "~ῑωνται";
    subj_pass_1: "~ῑωμαι", _, _;
    subj_pass_2: _, _, "~ῑησθε";
    opt_act_1: "~ῑειην", _, "~ῑειμεν|~ῑειημεν";
    opt_act_2: "~ῑειης", "~ῑειτον|~ῑειητον", "~ῑειτε|~ῑειητε";
    opt_act_3: "~ῑειη", "~ῑειτην|~ῑειητην", "~ῑειεν|~ῑειησᾰν";
    opt_mid_1: "~ῑειμην", _, "~ῑειμεθᾰ|~ῑοιμεθᾰ";
    opt_mid_2: "~ῑειο", "~ῑεισθον|~ῑοισθον", "~ῑεισθε|~ῑοισθε";
    opt_mid_3: "~ῑειτο|~ῑοιτο", "~ῑεισθην|~ῑοισθην", "~ῑειντο|~ῑοιντο";
    imp_act_2: "~ῑει", _, "~ῑετε";
    imp_act_3: _, _, "~ῑεντων";
    imp_mid_2: "~ῑεσο|~ῑου", "~ῑεσθον", "~ῑεσθε";
    imp_mid_3: _, _, "~ῑεσθων";
    imp_pass_2: _, "~ῑεσθον", "~ῑεσθε";
    imp_pass_3: "~ῑεσθω", "~ῑεσθων", "~ῑεσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ω", _, _;
    aorist_subj_act_2: "~ῃς", "~ητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: _, _, "~ωμεθᾰ";
    aorist_subj_mid_2: "~ῃ", "~ησθον", _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_opt_act_1: "~ειην", _, "~ειμεν";
    aorist_opt_act_2: "~ειης", _, _;
    aorist_opt_act_3: "~ειη", _, "~ειεν";
    aorist_opt_mid_1: "~ειμην", _, _;
    aorist_imp_act_2: "~ες", _, "~ετε";
    aorist_imp_act_3: _, _, "~εντων";
    aorist_imp_mid_2: "~ου", "~εσθον", _;
    aorist_imp_mid_3: _, _, "~εσθων";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_pres: "~ῑεναι", _, "~ῑεσθαι";
    infinitive_aor: "~ειναι", _, _;
    infinitive_perf: _, _, derived;
    participle_pres: "~ῑεις", _, "~ῑεμενος";
    participle_aor: "~εις", "~εμενος", _;
    ),
    // Στραγγᾰλοομαι: 9 words (στραγγᾰλοομαι, χειροομαι, ἀπῐχθῠοομαι, κᾰτοικειοομαι, ἐνᾰντῐοομαι, δεξῐοομαι). Like Ἀξῐοω, except in 89 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("στραγγᾰλοομαι", 9, "~οομαι", like Ἀξῐοω;
    stem rest__: "~ωσ- like ἑπομαι";
    ind_act_1: "~οομαι", _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: derived, _, derived;
    ind_mid_1: "~ουμαι", _, _;
    ind_pass_1: derived, _, _;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_1: "~ωμαι", _, _;
    subj_mid_2: "~οι", _, _;
    subj_pass_1: derived, _, _;
    subj_pass_2: "~οι", _, _;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imp_mid_3: "~ουσθω", _, _;
    imp_pass_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    imperfect_mid_2: _, "ἐ~ουσθον", _;
    imperfect_pass_2: _, derived, _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: _, derived, _;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, _;
    future_opt_act_3: derived, derived, _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_subj_act_3: _, derived, _;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    pluperfect_act_1: derived, _, _;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    participle_pres: derived, _, _;
    participle_aor: derived, _, _;
    ),
    // Ζωγρᾰφεω: 9 words (κᾰταφρονεω, κᾰταργεω, κᾰτηγορεω, λοιπογρᾰφεω, τερᾰτολογεω, τερᾰτουργεω). Like Πεμπω, except in 46 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ζωγρᾰφεω", 9, "~ᾰ~εω", like Πεμπω;
    stem lemma: "~α~- like ποιεω";
    stem rest____: "~α~- like ποιεω";
    stem rest__: "~α~ησ- like ποιεω";
    stem rest_______: "~α~ηθησ- like ποιεω";
    stem rest________: "~α~ησ- like ποιεω";
    stem rest__________: "~α~ηθην- like ἐχω";
    stem rest_: "~α~ησ- like ποιεω";
    stem rest_____: "~α~ηθ- like ποιεω";
    stem rest___: "~α~ηκ- like ἐχω";
    stem rest: "~α~η- like βουλευω";
    stem rest______: "~α~η- like ἐχω";
    stem rest___________: "~α~ηκειν- like ἐχω";
    stem rest_________: "~α~η- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Φθειρω: 9 words (κειρω, πειρω, ἐγειρω, κτεινω, σπειρω, φθειρω). Like Πεμπω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("φθειρω", 9, "~ει~ω", like Πεμπω;
    stem lemma: "~ει~- like βουλευω";
    stem rest____: "~ει~- like βουλευω";
    stem rest__: "~ε~- like λογῐζομαι";
    stem rest_______: "~ᾰ~ησ- like ἐχω";
    stem rest________: "~ει~- like ποιεω";
    stem rest__________: "~ᾰ~ην- like ἐχω";
    stem rest_: "~ει~- like ποιεω";
    stem rest_____: "~ᾰ~- like ἐγειρω";
    stem rest___: "~ᾰ~κ- like στηρῐζω";
    stem rest: "~ᾰ~- like σβεννῡμῐ";
    stem rest______: "~ᾰ~- like ἐγειρω";
    stem rest_________: "~ᾰ~- like ἐγειρω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "ἐ~ᾰ~κειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ἐξερχομαι: 9 words (ἐξερχομαι, εἰσερχομαι, ἀπερχομαι, προσερχομαι, δῐερχομαι, πᾰρερχομαι). Like Θωᾰω, except in 65 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ind_mid_3 du, ….
    conjugation!("ἐξερχομαι", 9, "~ερχομαι", like Θωᾰω;
    stem lemma: "~ερχ- like εἰσερχομαι";
    stem rest____: "~ηρχ- like εἰσερχομαι";
    stem rest__: "~ελευσ- like εἰσερχομαι";
    stem rest________: "~ηλθ- like εἰσερχομαι";
    stem rest_: "~ελθ- like εἰσερχομαι";
    stem rest___: "~εληλ- like εἰσερχομαι";
    ind_act_1: _, _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: derived, _, derived;
    ind_mid_3: _, derived, _;
    ind_pass_1: derived, _, derived;
    ind_pass_2: derived, derived, _;
    ind_pass_3: _, derived, _;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_3: _, derived, _;
    subj_pass_1: derived, _, derived;
    subj_pass_2: _, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imp_mid_2: _, derived, _;
    imp_pass_2: derived, derived, _;
    imp_pass_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    pluperfect_act_1: "~εληλῠθειν", _, _;
    infinitive_pres: derived, _, _;
    participle_pres: derived, _, _;
    ),
    // Δαινῡμῐ: 9 words (δεικνῡμῐ, ὀρνῡμῐ, ὀμνῡμῐ, δαινῡμῐ, στορνῡμῐ, ἐπορνῡμῐ). Like Ῥωννῡμῐ, except in 75 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("δαινῡμῐ", 9, "~νῡμῐ", like Ῥωννῡμῐ;
    stem lemma: "~ν- like σβεννῡμῐ";
    stem rest____: "~ν- like σβεννῡμῐ";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_mid_1: derived, _, _;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_mid_3: _, derived, _;
    perfect_subj_pass_1: derived, _, derived;
    perfect_subj_pass_2: derived, derived, derived;
    perfect_subj_pass_3: derived, derived, derived;
    perfect_opt_pass_1: derived, _, derived;
    perfect_opt_pass_2: derived, derived, derived;
    perfect_opt_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: _, derived, _;
    participle_perf: _, _, derived;
    ),
    // Σῑνομαι: 9 words (ἑζομαι, ἐρομαι, ἁλῐσκομαι, ἁζομαι, ἁλλομαι, κᾰθεζομαι). Like Θωᾰω, except in 59 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ind_pass_1 sg, ….
    conjugation!("σῑνομαι", 9, "~ομαι", like Θωᾰω;
    stem lemma: "~- like σῑνομαι";
    stem rest____: "~- like σῑνομαι";
    stem rest__: "~ησ- like ἑπομαι";
    stem rest________: "~- like ἐχω";
    stem rest_: "~- like ἐχω";
    ind_act_1: _, _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: derived, _, derived;
    ind_pass_1: derived, _, derived;
    ind_pass_2: derived, _, _;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_3: _, derived, _;
    subj_pass_1: derived, _, derived;
    subj_pass_2: _, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imp_pass_2: derived, _, _;
    imp_pass_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    infinitive_pres: derived, _, _;
    participle_pres: derived, _, _;
    ),
    // Ἀθῡρω: 8 words (ἀγρευω, ἀτασθαλλω, ἀμορβευω, ἀτεμβω, ἀφραινω, ἀπῐστευω). Like Μελλω, except in 130 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀθῡρω", 8, "ἀ~ω", like Μελλω;
    stem rest____: "~- like ἀθῡρω";
    stem rest__: "~σ- like ἀθῡρω";
    stem rest________: "~σ- like ἀθῡρω";
    stem rest__________: "~θην- like ἀθῡρω";
    stem rest_: "~σ- like ἀθῡρω";
    stem rest___: "~κ- like ἀθῡρω";
    stem rest______: "~- like ἀθῡρω";
    stem rest_________: "~- like ἀθῡρω";
    ind_act_1: "ἀ~ω", _, "ἀ~ομεν";
    ind_act_2: "ἀ~εις", "ἀ~ετον", _;
    ind_act_3: "ἀ~ει", _, "ἀ~ουσῐ|ἀ~ουσῐν";
    ind_mid_1: _, _, "ἀ~ομεθᾰ";
    ind_mid_2: "ἀ~ῃ", _, _;
    ind_mid_3: _, "ἀ~εσθον", _;
    ind_pass_1: "ἀ~ομαι", _, _;
    ind_pass_2: _, "ἀ~εσθον", "ἀ~εσθε";
    ind_pass_3: "ἀ~εται", "ἀ~εσθον", "ἀ~ονται";
    subj_act_1: "ἀ~ω", _, "ἀ~ωμεν";
    subj_act_2: "ἀ~ῃς", _, _;
    subj_act_3: "ἀ~ῃ", "ἀ~ητον", "ἀ~ωσῐ|ἀ~ωσῐν";
    subj_mid_1: _, _, "ἀ~ωμεθᾰ";
    subj_mid_2: _, "ἀ~ησθον", _;
    subj_mid_3: "ἀ~ηται", _, "ἀ~ωνται";
    subj_pass_1: "ἀ~ωμαι", _, _;
    subj_pass_2: _, _, "ἀ~ησθε";
    opt_act_1: "ἀ~οιμῐ", _, "ἀ~οιμεν";
    opt_act_2: "ἀ~οις", "ἀ~οιτον", "ἀ~οιτε";
    opt_act_3: "ἀ~οι", "ἀ~οιτην", "ἀ~οιεν";
    opt_mid_1: "ἀ~οιμην", _, "ἀ~οιμεθᾰ";
    opt_mid_2: "ἀ~οιο", "ἀ~οισθον", "ἀ~οισθε";
    opt_mid_3: "ἀ~οιτο", "ἀ~οισθην", "ἀ~οιντο";
    imp_act_2: "ἀ~ε", _, "ἀ~ετε";
    imp_act_3: _, _, "ἀ~οντων";
    imp_mid_2: "ἀ~ου", "ἀ~εσθον", "ἀ~εσθε";
    imp_mid_3: _, _, "ἀ~εσθων";
    imp_pass_2: _, "ἀ~εσθον", "ἀ~εσθε";
    imp_pass_3: "ἀ~εσθω", "ἀ~εσθων", "ἀ~εσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: "ἀ~θησομαι", _, _;
    future_ind_pass_3: _, "ἀ~θησεσθον", _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_1: "ἀ~θω", _, _;
    aorist_subj_pass_3: _, "ἀ~θητον", _;
    aorist_imp_pass_2: "ἀ~θητῐ", "ἀ~θητον", "ἀ~θητε";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: "ἠ~μαι", _, _;
    perfect_ind_mid_3: "ἠ~ται", derived, _;
    perfect_ind_pass_1: _, _, "ἠ~μεθᾰ";
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_subj_mid_3: _, "ἠ~μένω ἦτον", _;
    perfect_subj_pass_1: "ἠ~μένος ὦ", _, "ἠ~μένοι ὦμεν";
    perfect_subj_pass_2: "ἠ~μένος ᾖς", "ἠ~μένω ἦτον", "ἠ~μένοι ἦτε";
    perfect_subj_pass_3: "ἠ~μένος ᾖ", "ἠ~μένω ἦτον", "ἠ~μένοι ὦσῐ";
    perfect_opt_pass_1: "ἠ~μένος εἴην", _, "ἠ~μένοι εἴημεν";
    perfect_opt_pass_2: "ἠ~μένος εἴης", "ἠ~μένω εἴητον", "ἠ~μένοι εἴητε";
    perfect_opt_pass_3: "ἠ~μένος εἴη", "ἠ~μένω εἰήτην", "ἠ~μένοι εἴησᾰν";
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "ἠ~κειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    future_perfect_ind_mid_1: "ἠ~σομαι", _, _;
    future_perfect_ind_mid_3: _, "ἠ~σεσθον", _;
    future_perfect_ind_pass_1: _, _, "ἠ~σομεθᾰ";
    future_perfect_ind_pass_2: "ἠ~σῃ", "ἠ~σεσθον", "ἠ~σεσθε";
    future_perfect_ind_pass_3: "ἠ~σεται", "ἠ~σεσθον", "ἠ~σονται";
    future_perfect_opt_pass_1: "ἠ~σοιμην", _, "ἠ~σοιμεθᾰ";
    future_perfect_opt_pass_2: "ἠ~σοιο", "ἠ~σοισθον", "ἠ~σοισθε";
    future_perfect_opt_pass_3: "ἠ~σοιτο", "ἠ~σοισθην", "ἠ~σοιντο";
    infinitive_pres: "ἀ~ειν", _, "ἀ~εσθαι";
    infinitive_perf: _, _, derived;
    infinitive_futp: _, _, "ἠ~σεσθαι";
    participle_pres: "ἀ~ων", _, "ἀ~ομενος";
    participle_perf: _, _, "ἠ~μενος";
    participle_futp: _, _, "ἠ~σομενος";
    ),
    // Ψῑλοω: 8 words (ἱδροω, στενοω, πτεροω, ψῑλοω, κεραυνοω, φορτοω). Like Ἀξῐοω, except in 19 cells: future_ind_act_3 du, future_ind_mid_3 du, perfect_ind_act_3 du, perfect_ind_mid_3 du, perfect_ind_pass_1 sg, perfect_ind_pass_2 sg, ….
    conjugation!("ψῑλοω", 8, "~οω", like Ἀξῐοω;
    stem rest__: "~ωσ- like ποιεω";
    stem rest_______: "~ωθησ- like ποιεω";
    stem rest_____: "~ωθ- like ποιεω";
    stem rest___: "~ωκ- like σκοπεω";
    stem rest: "~ω- like σκοπεω";
    stem rest______: "~ω- like σκοπεω";
    stem rest___________: "~ωκ- like σκοπεω";
    stem rest_________: "~ω- like σκοπεω";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Φᾰρμᾰσσω: 8 words (τᾰρᾰσσω, θωρησσω, κνωσσω, θρῡλῐσσω, παιφᾰσσω, φᾰρμᾰσσω). Like Τᾰσσω, except in 63 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("φᾰρμᾰσσω", 8, "~σσω", like Τᾰσσω;
    stem lemma: "~σσ- like βᾰπτῐζω";
    stem rest____: "~σσ- like ἐχω";
    stem rest__: "~ξ- like ἐχω";
    stem rest________: "~ξ- like φᾰρμᾰσσω";
    stem rest_: "~ξ- like φᾰρμᾰσσω";
    stem rest______: "~- like φᾰρμᾰσσω";
    stem rest_________: "~- like φᾰρμᾰσσω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    future_perfect_ind_mid_1: "%ε~ξομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~ξεσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~ξομεθᾰ";
    future_perfect_ind_pass_2: "%ε~ξῃ", "%ε~ξεσθον", "%ε~ξεσθε";
    future_perfect_ind_pass_3: "%ε~ξεται", "%ε~ξεσθον", "%ε~ξονται";
    future_perfect_opt_pass_1: "%ε~ξοιμην", _, "%ε~ξοιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~ξοιο", "%ε~ξοισθον", "%ε~ξοισθε";
    future_perfect_opt_pass_3: "%ε~ξοιτο", "%ε~ξοισθην", "%ε~ξοιντο";
    infinitive_perf: _, _, derived;
    infinitive_futp: _, _, "%ε~ξεσθαι";
    participle_futp: _, _, "%ε~ξομενος";
    ),
    // Δῐωκω: 8 words (βρῡκω, πεκω, πλεκω, τηκω, κᾰτᾰδῐωκω, δῐωκω). Like Τᾰσσω, except in 50 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("δῐωκω", 8, "~κω", like Τᾰσσω;
    stem lemma: "~κ- like βᾰπτῐζω";
    stem rest____: "~κ- like ἐχω";
    stem rest__: "~ξ- like ἐχω";
    stem rest________: "~ξ- like ἐχω";
    stem rest_: "~ξ- like ἐχω";
    stem rest______: "~- like φᾰρμᾰσσω";
    stem rest_________: "~- like φᾰρμᾰσσω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Σφᾰλλω: 8 words (βᾰλλω, θᾰλλω, πᾰλλω, βδᾰλλω, σφᾰλλω, ψᾰλλω). Like Φθειρω, except in 30 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("σφᾰλλω", 8, "~ᾰ~λω", like Φθειρω;
    stem lemma: "~ᾰ~λ- like βουλευω";
    stem rest____: "~ᾰ~λ- like βουλευω";
    stem rest__: "~ᾰ~- like λογῐζομαι";
    stem rest_______: "~ᾰ~ησησ- like ἐχω";
    stem rest________: "~η~- like ποιεω";
    stem rest_: "~η~- like ποιεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    ),
    // Κῠω: 8 words (φῠω, ἐνδῠω, κῠω, κωλῠω, προμηνῠω, δῐᾰμηνῠω). Like Ληγω, except in 44 cells: ind_act_3 du, ind_pass_1 sg, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, ….
    conjugation!("κῠω", 8, "~ῠω", like Ληγω;
    stem lemma: "~ῠ- like φῠω";
    stem rest____: "~ῠ- like ἐχω";
    stem rest__: "~ῡσ- like ἐχω";
    stem rest_______: "~ῡθησ- like ἐχω";
    stem rest________: "~ῡσ- like ἐχω";
    stem rest__________: "~ῡθην- like ἐχω";
    stem rest_: "~ῡσ- like φῠω";
    stem rest_____: "~ῡθ- like ἐχω";
    stem rest___: "~ῡκ- like ἐχω";
    stem rest: "~ῡ- like βουλευω";
    stem rest______: "~ῡ- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: derived, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: derived, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imp_pass_3: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Ψευδω: 8 words (ᾀδω, ἀειδω, σπευδω, ψευδω, δειδω, ἐρειδω). Like Ῥωννῡμῐ, except in 41 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ψευδω", 8, "~δω", like Ῥωννῡμῐ;
    stem lemma: "~δ- like βᾰπτῐζω";
    stem rest____: "~δ- like ἐχω";
    stem rest: "~σ- like σβεννῡμῐ";
    stem rest______: "~- like ψηφῐζω";
    stem rest_________: "~- like ψηφῐζω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Πλᾰσσω: 8 words (πλᾰσσω, πᾰσσω, βρᾰσσω, ἱμᾰσσω, ἐρεσσω, βλῐσσω). Like Κρῑνομαι, except in 62 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("πλᾰσσω", 8, "~σω", like Κρῑνομαι;
    stem lemma: "~σ- like βουλευω";
    stem rest____: "~σ- like ἐχω";
    stem rest__: "~- like ἐχω";
    stem rest________: "~- like ἐχω";
    stem rest_: "~- like ποιεω";
    stem rest______: "~- like χεω";
    stem rest_________: "~- like πειθω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    future_perfect_ind_mid_1: "%ε~ομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~εσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~ομεθᾰ";
    future_perfect_ind_pass_2: "%ε~ῃ", "%ε~εσθον", "%ε~εσθε";
    future_perfect_ind_pass_3: "%ε~εται", "%ε~εσθον", "%ε~ονται";
    future_perfect_opt_pass_1: "%ε~οιμην", _, "%ε~οιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~οιο", "%ε~οισθον", "%ε~οισθε";
    future_perfect_opt_pass_3: "%ε~οιτο", "%ε~οισθην", "%ε~οιντο";
    infinitive_perf: _, _, derived;
    infinitive_futp: _, _, "%ε~εσθαι";
    participle_futp: _, _, "%ε~ομενος";
    ),
    // Κερδαινω: 8 words (σημαινω, ῥαινω, μιαινω, κερδαινω, μελαινω, εὐφραινω). Like Σαινω, except in 36 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("κερδαινω", 8, "~αινω", like Σαινω;
    stem lemma: "~αιν- like βουλευω";
    stem rest__: "~αν- like λογῐζομαι";
    stem rest_______: "~ανθησ- like ποιεω";
    stem rest________: "~ᾱν- like ἐχω";
    stem rest__________: "~ανθην- like ἐχω";
    stem rest_: "~ᾱν- like ἐχω";
    stem rest_____: "~ανθ- like ἐχω";
    stem rest___: "~ᾰγκ- like ἐχω";
    stem rest: "~ᾰ- like πορσῡνω";
    stem rest______: "~ᾰν- like πορσῡνω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Λῡω: 8 words (λῡω, δῡω, καμμῡω, καταμῡω, κᾰσσῡω, ἐξαναλῡω). Like Κῠω, except in 88 cells: ind_act_3 du, ind_mid_1 sg, ind_pass_1 pl, ind_pass_2 sg, subj_mid_1 sg, subj_mid_2 sg, ….
    conjugation!("λῡω", 8, "~ῡω", like Κῠω;
    stem lemma: "~ῡ- like βουλευω";
    stem rest____: "~ῡ- like βουλευω";
    stem rest__: "~ῡσ- like ποιεω";
    stem rest_______: "~ῠθησ- like ποιεω";
    stem rest________: "~ῡσ- like ποιεω";
    stem rest__________: "~ῠθην- like ἐχω";
    stem rest_: "~ῡσ- like ποιεω";
    stem rest_____: "~ῠθ- like ποιεω";
    stem rest___: "~ῠκ- like ποιεω";
    stem rest______: "~ῠ- like ἐχω";
    stem rest___________: "~ῠκειν- like ἐχω";
    stem rest_________: "~ῠ- like ἐχω";
    ind_act_3: _, derived, _;
    ind_mid_1: derived, _, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_1: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_mid_3: derived, _, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: "%ε~ῠμαι", _, _;
    perfect_ind_mid_3: "%ε~ῠται", derived, _;
    perfect_ind_pass_1: _, _, "%ε~ῠμεθᾰ";
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_subj_mid_3: _, "%ε~ῠμένω ἦτον", "%ε~ῠμένοι ὦσῐ|%ε~ῠμένοι ὦσῐν";
    perfect_subj_pass_1: "%ε~ῠμένος ὦ", _, "%ε~ῠμένοι ὦμεν";
    perfect_subj_pass_2: "%ε~ῠμένος ᾖς", "%ε~ῠμένω ἦτον", "%ε~ῠμένοι ἦτε";
    perfect_subj_pass_3: "%ε~ῠμένος ᾖ", "%ε~ῠμένω ἦτον", derived;
    perfect_opt_pass_1: "%ε~ῠμένος εἴην", _, "%ε~ῠμένοι εἴημεν";
    perfect_opt_pass_2: "%ε~ῠμένος εἴης", "%ε~ῠμένω εἴητον", "%ε~ῠμένοι εἴητε";
    perfect_opt_pass_3: "%ε~ῠμένος εἴη", "%ε~ῠμένω εἰήτην", "%ε~ῠμένοι εἴησᾰν";
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    future_perfect_ind_mid_1: "%ε~ῡσομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~ῡσεσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~ῡσομεθᾰ";
    future_perfect_ind_pass_2: "%ε~ῡσῃ", "%ε~ῡσεσθον", "%ε~ῡσεσθε";
    future_perfect_ind_pass_3: "%ε~ῡσεται", "%ε~ῡσεσθον", "%ε~ῡσονται";
    future_perfect_opt_pass_1: "%ε~ῡσοιμην", _, "%ε~ῡσοιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~ῡσοιο", "%ε~ῡσοισθον", "%ε~ῡσοισθε";
    future_perfect_opt_pass_3: "%ε~ῡσοιτο", "%ε~ῡσοισθην", "%ε~ῡσοιντο";
    infinitive_perf: _, _, derived;
    infinitive_futp: _, _, "%ε~ῡσεσθαι";
    participle_perf: _, _, "%ε~ῠμενος";
    participle_futp: _, _, "%ε~ῡσομενος";
    ),
    // Μαχομαι: 8 words (μαχομαι, πῠνθανομαι, σῠνεργαζομαι, κᾰτεργαζομαι, ἀπεχθανομαι, ἐξεργαζομαι). Like Ἐπᾰϊω, except in 121 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("μαχομαι", 8, "~α~ομαι", like Ἐπᾰϊω;
    stem rest____: "~ᾰ~- like σῑνομαι";
    stem rest__: "~ᾰ~εσ- like ἑπομαι";
    stem rest________: "~ᾰ~εσ- like ἑπομαι";
    stem rest_: "~ᾰ~εσ- like ἑπομαι";
    stem rest: "~εμᾰ~η- like ἐλπω";
    stem rest______: "~εμᾰ~η- like εὐλογεω";
    ind_act_1: "~α~ομαι", _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: derived, _, derived;
    ind_mid_1: "~ᾰ~ομαι", _, _;
    ind_pass_1: derived, _, _;
    ind_pass_2: "~ᾰ~ῃ", _, _;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_1: "~ᾰ~ωμαι", _, _;
    subj_mid_2: "~ᾰ~ῃ", _, _;
    subj_pass_1: derived, _, _;
    subj_pass_2: "~ᾰ~ῃ", _, _;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imp_mid_3: "~ᾰ~εσθω", _, _;
    imp_pass_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: _, derived, _;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, _;
    future_opt_act_3: derived, derived, _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_act_3: _, derived, _;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    infinitive_perf: _, _, derived;
    participle_pres: derived, _, _;
    participle_aor: derived, _, _;
    ),
    // Λλῡμῐ: 8 words (εἰμῐ, δῐδωμῐ, ἀντᾰποδῐδωμῐ, πᾰρᾰδιδωμῐ, ἠμῐ, ὀλλῡμῐ). Like Πῠρσαινω, except in 73 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("λλῡμῐ", 8, "~μῐ", like Πῠρσαινω;
    ind_act_1: "~μῐ", _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: "~σῐ", _, derived;
    ind_mid_1: _, _, derived;
    ind_mid_2: derived, _, _;
    ind_mid_3: _, derived, _;
    ind_pass_1: derived, _, _;
    ind_pass_2: _, derived, derived;
    ind_pass_3: derived, derived, derived;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_1: _, _, derived;
    subj_mid_2: _, derived, _;
    subj_mid_3: derived, _, derived;
    subj_pass_1: derived, _, _;
    subj_pass_2: _, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, derived;
    opt_mid_3: derived, derived, derived;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imp_mid_2: derived, derived, derived;
    imp_mid_3: _, _, derived;
    imp_pass_2: _, derived, derived;
    imp_pass_3: derived, derived, derived;
    imperfect_act_1: "~ν", _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: "~", _, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, _, _;
    imperfect_mid_3: derived, _, derived;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, _;
    infinitive_pres: derived, _, derived;
    participle_pres: derived, _, derived;
    ),
    // Πρεσβευω: 7 words (δρᾱω, χρῑω, κλειω, πρεσβευω, μᾰθητευω, λαμπᾰδευω). Like Βουλευω.
    conjugation!("πρεσβευω", 7, "~ω", like Βουλευω;
    stem rest_______: "~σθησ- like ποιεω";
    stem rest__________: "~σθην- like ἐχω";
    stem rest_____: "~σθ- like ποιεω";
    ),
    // Σοφῐζω: 7 words (σοφῐζω, σῳζω, κεᾰζω, στρουθῐζω, τῠκῐζω, μερῐζω). Like Χρημᾰτῐζω, except in 13 cells: perfect_ind_mid_3 du, perfect_ind_pass_2 sg, perfect_ind_pass_2 du, perfect_ind_pass_3 du, perfect_ind_pass_3 pl, perfect_imp_mid_2 du, ….
    conjugation!("σοφῐζω", 7, "~ζω", like Χρημᾰτῐζω;
    stem rest______: "~σ- like χεω";
    stem rest_________: "~σ- like πειθω";
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Βᾰδιζω: 7 words (εὐαγγελιζω, βᾰδιζω, καθιζω, συνᾱλιζω, βαλλιζω, μακαριζω). Like Πεμπω, except in 70 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("βᾰδιζω", 7, "~~ζω", like Πεμπω;
    stem lemma: "~~ζ- like βουλευω";
    stem rest____: "~~ζ- like βουλευω";
    stem rest__: "~~- like χρημᾰτῐζω";
    stem rest_______: "~~σθησ- like ἐχω";
    stem rest________: "~~σ- like ποιεω";
    stem rest__________: "~~σθην- like ἐχω";
    stem rest_____: "~~σθ- like ἐχω";
    stem rest___: "~~κ- like ἐχω";
    stem rest: "~~σ- like βουλευω";
    stem rest___________: "~~κειν- like ἐχω";
    stem rest_________: "~~σ- like πειθω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_1: "~~σω", _, _;
    aorist_subj_act_2: "~~σῃς", "~~σητον", _;
    aorist_subj_act_3: _, _, "~~σωσῐ|~~σωσῐν";
    aorist_subj_mid_1: _, _, "~~σωμεθᾰ";
    aorist_subj_mid_2: "~~σῃ", "~~σησθον", _;
    aorist_opt_act_1: "~~σαιμῐ", _, "~~σαιμεν";
    aorist_opt_act_2: "~~σειᾰς|~~σαις", _, _;
    aorist_opt_act_3: "~~σειε|~~σειεν|~~σαι", _, "~~σειᾰν|~~σαιεν";
    aorist_opt_mid_1: "~~σαιμην", _, _;
    aorist_imp_act_2: "~~σον", _, "~~σᾰτε";
    aorist_imp_act_3: _, _, "~~σᾰντων";
    aorist_imp_mid_2: "~~σαι", "~~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~σᾰσθων";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_2: "%ε~~σαι", "%ε~~σθον", _;
    perfect_ind_mid_3: _, _, "%ε~~σᾰται";
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, "%ε~~σθε";
    perfect_ind_pass_3: derived, _, _;
    perfect_imp_mid_2: _, _, "%ε~~σθε";
    perfect_imp_mid_3: _, _, "%ε~~σθων";
    perfect_imp_pass_2: "%ε~~σο", _, "%ε~~σθε";
    perfect_imp_pass_3: "%ε~~σθω", "%ε~~σθων", "%ε~~σθων";
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_aor: "~ῑσα~", _, _;
    infinitive_perf: _, "%ε~ῑσθα~", _;
    participle_aor: "~~σᾱς", "~~σᾰμενος", _;
    ),
    // Ἀπειπον: 7 words (προσειπον, ἐξειπον, προειπον, ἀπειπον, ὑπειπον, ἐπειπον). Like Λλῡμῐ, except in 19 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, future_ind_act_3 du, future_ind_mid_3 du, ….
    conjugation!("ἀπειπον", 7, "~ειπον", like Λλῡμῐ;
    stem rest__: "~ερ- like λογῐζομαι";
    stem rest_______: "~ορρηθησ- like ἐχω";
    stem rest________: "~ειπ- like ἀνᾰγω";
    stem rest__________: "~ερρηθην- like ἐλπω";
    stem rest_: "~ειπ- like φᾰρμᾰσσω";
    stem rest_____: "~ορρηθ- like ἐχω";
    stem rest___: "~ειρηκ- like πεμπω";
    stem rest: "~ειρη- like ἐλπω";
    stem rest______: "~ειρη- like εὐλογεω";
    ind_act_1: "~ειπον", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Τελεω: 7 words (ἐμεω, ἀρκεω, τελεω, καταπαιδεραστεω, βδεω, νεικεω). Like Ὀρχεομαι, except in 41 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("τελεω", 7, "~εω", like Ὀρχεομαι;
    stem lemma: "~- like τελεω";
    stem rest__: "~- like λογῐζομαι";
    stem rest_______: "~εσθησ- like ἐχω";
    stem rest________: "~εσ- like ἐχω";
    stem rest__________: "~εσθην- like ἐχω";
    stem rest_: "~εσ- like ἐχω";
    stem rest_____: "~εσθ- like ἐχω";
    stem rest___: "~εκ- like ἐχω";
    stem rest: "~εσ- like βουλευω";
    stem rest______: "~εσ- like χεω";
    stem rest___________: "~εκειν- like ἐχω";
    stem rest_________: "~εσ- like πειθω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Χραω: 7 words (χραω, νωμαω, σκῐαω, ἀπαγχοναω, λοχαω, πασχητῐαω). Like Νῑκαω, except in 5 cells: pluperfect_pass_1 sg, pluperfect_pass_2 sg, pluperfect_pass_2 du, pluperfect_pass_3 sg, pluperfect_pass_3 pl.
    conjugation!("χραω", 7, "~αω", like Νῑκαω;
    stem rest_______: "~ησθησ- like ἐχω";
    stem rest__________: "~ησθην- like ἐχω";
    stem rest_____: "~ησθ- like ἐχω";
    stem rest___________: "~ηκειν- like ἐχω";
    stem rest_________: "~η- like ἐχω";
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    ),
    // Βουλομαι: 7 words (πετομαι, βουλομαι, οἰομαι, οἰχομαι, κελομαι, ἀχθομαι). Like Τεχναομαι, except in 39 cells: ind_pass_1 sg, ind_pass_1 pl, ind_pass_2 sg, subj_mid_3 du, subj_pass_1 sg, subj_pass_1 pl, ….
    conjugation!("βουλομαι", 7, "~ομαι", like Τεχναομαι;
    stem lemma: "~- like οἰομαι";
    stem rest____: "~- like σῑνομαι";
    stem rest__________: "~ηθην- like ἐχω";
    stem rest_____: "~ηθ- like ἐχω";
    ind_pass_1: derived, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: derived, _, derived;
    subj_pass_2: _, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_pass_2: derived, _, _;
    imp_pass_3: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    aorist_ind_mid_1: derived, _, _;
    aorist_ind_mid_2: derived, derived, _;
    aorist_subj_mid_1: _, _, derived;
    aorist_subj_mid_2: derived, derived, _;
    aorist_opt_mid_1: derived, _, _;
    aorist_imp_mid_2: derived, derived, _;
    aorist_imp_mid_3: _, _, derived;
    participle_aor: _, derived, _;
    ),
    // Θλαω: 7 words (ἀγαλλιαω, κρεμαω, θλαω, τηταω, ἐλᾰω, νεᾰω). Like Νῑκαω, except in 44 cells: future_ind_act_3 du, future_ind_mid_3 du, aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, ….
    conjugation!("θλαω", 7, "~αω", like Νῑκαω;
    stem rest__: "~ασ- like ἐχω";
    stem rest_______: "~ασθησ- like ἐχω";
    stem rest________: "~ασ- like ἐχω";
    stem rest__________: "~ασθην- like ἐχω";
    stem rest_____: "~ασθ- like ἐχω";
    stem rest___: "~ακ- like ἐχω";
    stem rest: "~α- like βουλευω";
    stem rest___________: "~ακειν- like ἐχω";
    stem rest_________: "~α- like ἐχω";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_1: "~ασω", _, _;
    aorist_subj_act_2: "~ασῃς", "~ασητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: _, _, "~ασωμεθᾰ";
    aorist_subj_mid_2: "~ασῃ", "~ασησθον", _;
    aorist_opt_act_1: "~ασαιμῐ", _, "~ασαιμεν";
    aorist_opt_act_2: "~ασειᾰς", _, _;
    aorist_opt_act_3: "~ασειε", _, "~ασειᾰν";
    aorist_opt_mid_1: "~ασαιμην", _, _;
    aorist_imp_act_2: "~ᾱσον", _, "~ασᾰτε";
    aorist_imp_act_3: _, _, "~ασᾰντων";
    aorist_imp_mid_2: "~ᾱσαι", "~ασᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ασᾰσθων";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_2: "%ε~ασαι", "%ε~ασθον", _;
    perfect_ind_mid_3: _, _, "%ε~ανται";
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, "%ε~ασθε";
    perfect_ind_pass_3: derived, _, _;
    perfect_imp_mid_2: _, _, "%ε~ασθε";
    perfect_imp_mid_3: _, _, "%ε~ασθων";
    perfect_imp_pass_2: "%ε~ασο", _, "%ε~ασθε";
    perfect_imp_pass_3: "%ε~ασθω", "%ε~ασθων", "%ε~ασθων";
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_aor: "~ᾱσαι", _, _;
    infinitive_perf: _, "%ε~ᾱσθαι", _;
    participle_aor: "~ασᾱς", "~ασᾰμενος", _;
    ),
    // Λᾰγοθηρεω: 7 words (κρατεω, παιδεραστεω, φωνομᾰχεω, τερᾰτοτοκεω, λᾰγοθηρεω, λᾰβρηγορεω). Like Ζωγρᾰφεω, except in 25 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("λᾰγοθηρεω", 7, "~ᾰ~εω", like Ζωγρᾰφεω;
    stem lemma: "~ᾰ~- like ποιεω";
    stem rest__: "~ᾰ~ησ- like ποιεω";
    stem rest_______: "~ᾰ~ηθησ- like ποιεω";
    stem rest_: "~ᾰ~ησ- like ποιεω";
    stem rest_____: "~ᾰ~ηθ- like ποιεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    ),
    // Εἰρω: 7 words (εἰρω, ἐκχεω, ἀειρω, κλῠω, δῐᾰχεω, σευω). Like Κᾰθιδρῡω, except in 69 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("εἰρω", 7, "~ω", like Κᾰθιδρῡω;
    stem lemma: "~- like βᾰπτῐζω";
    stem rest____: "~- like ἀειρω";
    stem rest________: "~- like ἐλπω";
    stem rest_: "~- like ἐχω";
    stem rest___: "~κ- like πεμπω";
    stem rest: "~- like ἐλπω";
    stem rest______: "~- like ἀειρω";
    stem rest_________: "~- like ἀειρω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: _, derived, _;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, _;
    future_opt_act_3: derived, derived, _;
    future_opt_mid_1: derived, _, _;
    future_opt_mid_3: derived, derived, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: derived, derived, derived;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_act_3: _, _, derived;
    perfect_subj_mid_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "~κειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_fut: derived, _, _;
    infinitive_perf: _, _, derived;
    ),
    // Λλᾰμβᾰνω: 7 words (ἐπῐλᾰμβᾰνω, προλᾰμβᾰνω, ὑπολᾰμβᾰνω, ἀπολᾰμβᾰνω, ξῠλλᾰμβᾰνω, προσλαμβᾰνω). Like Πῠρσαινω, except in 63 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("λλᾰμβᾰνω", 7, "~λᾰμβᾰνω", like Πῠρσαινω;
    stem lemma: "~λᾰμβᾰν- like βᾰπτῐζω";
    stem rest____: "~ᾰμβᾰν- like ἐχω";
    stem rest__: "~ληψ- like ἑπομαι";
    stem rest_______: "~ληφθησ- like ἐχω";
    stem rest________: "~ᾰβ- like φᾰρμᾰσσω";
    stem rest__________: "~ηφθην- like ἐχω";
    stem rest_: "~λᾰβ- like φᾰρμᾰσσω";
    stem rest_____: "~ληφθ- like ἐχω";
    stem rest___: "~ηφ- like ἐπῐλᾰμβᾰνω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: "ἐι~ημμαι", _, _;
    perfect_ind_mid_2: "ἐι~ηψαι", "ἐι~ηφθον", _;
    perfect_ind_mid_3: "ἐι~ηπται", _, "ἐι~ηφᾰται";
    perfect_ind_pass_1: _, _, "ἐι~ημμεθᾰ";
    perfect_ind_pass_2: _, _, "ἐι~ηφθε";
    perfect_subj_mid_3: _, "ἐι~ημμένω ἦτον", _;
    perfect_subj_pass_1: "ἐι~ημμένος ὦ", _, "ἐι~ημμένοι ὦμεν";
    perfect_subj_pass_2: "ἐι~ημμένος ᾖς", "ἐι~ημμένω ἦτον", "ἐι~ημμένοι ἦτε";
    perfect_subj_pass_3: "ἐι~ημμένος ᾖ", "ἐι~ημμένω ἦτον", "ἐι~ημμένοι ὦσῐ";
    perfect_opt_pass_1: "ἐι~ημμένος εἴην", _, "ἐι~ημμένοι εἴημεν";
    perfect_opt_pass_2: "ἐι~ημμένος εἴης", "ἐι~ημμένω εἴητον", "ἐι~ημμένοι εἴητε";
    perfect_opt_pass_3: "ἐι~ημμένος εἴη", "ἐι~ημμένω εἰήτην", "ἐι~ημμένοι εἴησᾰν";
    perfect_imp_mid_2: _, _, "ἐι~ηφθε";
    perfect_imp_mid_3: _, _, "ἐι~ηφθων";
    perfect_imp_pass_2: "ἐι~ηψο", _, "ἐι~ηφθε";
    perfect_imp_pass_3: "ἐι~ηφθω", "ἐι~ηφθων", "ἐι~ηφθων";
    infinitive_perf: _, "ἐι~ηφθαι", _;
    participle_perf: _, _, "ἐι~ημμενος";
    ),
    // Θρῠπτω: 6 words (βᾰπτω, σκηπτω, θρῠπτω, γνᾰμπτω, μᾰρπτω, δρῠπτω). Like Δᾰπτω, except in 89 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("θρῠπτω", 6, "~πτω", like Δᾰπτω;
    stem lemma: "~πτ- like βουλευω";
    stem rest____: "~πτ- like βουλευω";
    stem rest__: "~ψ- like ποιεω";
    stem rest_______: "~φησ- like ποιεω";
    stem rest________: "~ψ- like ποιεω";
    stem rest_: "~ψ- like ποιεω";
    stem rest_____: "~φθ- like ποιεω";
    stem rest___: "~πτ- like ἐχω";
    stem rest______: "~- like θρῠπτω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: "%ε~μμαι", _, _;
    perfect_ind_mid_3: "%ε~πται", derived, _;
    perfect_ind_pass_1: _, _, "%ε~μμεθᾰ";
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_subj_mid_3: _, "%ε~μμένω ἦτον", "%ε~μμένοι ὦσῐ|%ε~μμένοι ὦσῐν";
    perfect_subj_pass_1: "%ε~μμένος ὦ", _, "%ε~μμένοι ὦμεν";
    perfect_subj_pass_2: "%ε~μμένος ᾖς", "%ε~μμένω ἦτον", "%ε~μμένοι ἦτε";
    perfect_subj_pass_3: "%ε~μμένος ᾖ", "%ε~μμένω ἦτον", derived;
    perfect_opt_pass_1: "%ε~μμένος εἴην", _, "%ε~μμένοι εἴημεν";
    perfect_opt_pass_2: "%ε~μμένος εἴης", "%ε~μμένω εἴητον", "%ε~μμένοι εἴητε";
    perfect_opt_pass_3: "%ε~μμένος εἴη", "%ε~μμένω εἰήτην", "%ε~μμένοι εἴησᾰν";
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    future_perfect_ind_mid_1: derived, _, _;
    future_perfect_ind_mid_3: _, derived, _;
    future_perfect_ind_pass_1: _, _, derived;
    future_perfect_ind_pass_2: derived, derived, derived;
    future_perfect_ind_pass_3: derived, derived, derived;
    future_perfect_opt_pass_1: derived, _, derived;
    future_perfect_opt_pass_2: derived, derived, derived;
    future_perfect_opt_pass_3: derived, derived, derived;
    infinitive_perf: _, _, derived;
    infinitive_futp: _, _, derived;
    participle_perf: _, _, "%ε~μμενος";
    participle_futp: _, _, derived;
    ),
    // Σειω: 6 words (ζεω, σειω, κελευω, παιω, ἀνῠω, λαυω). Like Πρεσβευω, except in 16 cells: perfect_ind_act_3 du, perfect_ind_mid_3 du, perfect_ind_pass_1 sg, perfect_ind_pass_2 sg, perfect_ind_pass_2 du, perfect_ind_pass_3 sg, ….
    conjugation!("σειω", 6, "~ω", like Πρεσβευω;
    stem rest___: "~κ- like ποιεω";
    stem rest: "~σ- like σειω";
    stem rest______: "~σ- like χεω";
    stem rest_________: "~σ- like πειθω";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Βαινω: 6 words (βαινω, κᾰτᾰβαινω, ἐπεκβαινω, εἰσᾰνᾰβαινω, ἐπαλθαινω, πᾰρᾰβαινω). Like Σαινω, except in 41 cells: future_ind_act_3 du, future_ind_mid_3 du, future_ind_pass_1 sg, future_ind_pass_3 du, aorist_subj_act_1 sg, aorist_subj_act_2 sg, ….
    conjugation!("βαινω", 6, "~αινω", like Σαινω;
    stem rest__: "~ησ- like ἐχω";
    stem rest________: "~η- like βαινω";
    stem rest__________: "~ᾰθην- like ἐχω";
    stem rest_____: "~ᾰθ- like ἐχω";
    stem rest___: "~εβηκ- like πεμπω";
    stem rest: "~εβᾰ- like ἐλπω";
    stem rest______: "~εβᾰ- like εὐλογεω";
    stem rest_________: "~εβᾰ- like σκοπεω";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    aorist_subj_act_1: "~ω", _, _;
    aorist_subj_act_2: "~ῃς", "~ητον", _;
    aorist_subj_mid_1: _, _, "~ησωμεθᾰ";
    aorist_subj_mid_2: "~ησῃ", "~ησησθον", _;
    aorist_opt_act_1: "~αιην", _, "~αιμεν";
    aorist_opt_act_2: "~αιης", _, _;
    aorist_opt_act_3: "~αιη", _, "~αιεν";
    aorist_opt_mid_1: "~ησαιμην", _, _;
    aorist_imp_act_2: "~ηθῐ", _, "~ητε";
    aorist_imp_act_3: _, _, "~ᾰντων";
    aorist_imp_mid_2: "~ησαι", "~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ησᾰσθων";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "ἐ~εβηκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    participle_aor: "~ᾱς", "~ησᾰμενος", _;
    ),
    // Μηκᾰομαι: 6 words (ἰᾰομαι, θεᾰομαι, ἀκροᾰομαι, αἰτῐᾰομαι, ἡσσᾰομαι, μηκᾰομαι). Like Τεχναομαι, except in 19 cells: ind_act_1 sg, imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_2 du, imperfect_pass_3 sg, ….
    conjugation!("μηκᾰομαι", 6, "~ᾰομαι", like Τεχναομαι;
    stem rest____: "~- like μηκᾰομαι";
    stem rest__: "~ᾱσ- like ἑπομαι";
    stem rest_______: "~ᾱθησ- like ἐχω";
    stem rest________: "~ᾱσ- like μηκᾰομαι";
    stem rest__________: "~ᾱθην- like ἐλπω";
    stem rest_: "~ᾱσ- like ἑπομαι";
    stem rest_____: "~ᾱθ- like ἐχω";
    stem rest: "~ᾱ- like ἐλπω";
    stem rest______: "~ᾱ- like εὐλογεω";
    ind_act_1: "~ᾰομαι", _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Ἐπεξειμι: 6 words (ἐξειμι, ἀπειμι, κᾰτειμῐ, προσειμι, προειμι, ἐπεξειμι). Like Λλῡμῐ, except in 32 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐπεξειμι", 6, "~ειμι", like Λλῡμῐ;
    ind_act_1: "~ειμι", _, "~ῐμεν";
    ind_act_2: "~ει", "~ῐτον", _;
    ind_act_3: "~εισῐ|~εισῐν", _, "~ῐᾱσῐ|~ῐᾱσῐν";
    subj_act_1: "~ῐω", _, "~ῐωμεν";
    subj_act_2: "~ῐῃς", "~ῐητον", _;
    subj_act_3: "~ῐῃ", _, "~ῐωσῐ|~ῐωσῐν";
    opt_act_1: "~ῐοιμῐ", _, "~ῐοιμεν";
    opt_act_2: "~ῐοις", "~ῐοιτον", "~ῐοιτε";
    opt_act_3: "~ῐοι", "~ῐοιτην", "~ῐοιεν";
    imp_act_2: "~ῐθῐ", _, "~ῐτε";
    imp_act_3: _, _, "~ῐοντων";
    imperfect_act_1: "~ῃᾰ|~ῃειν", _, "~ῃμεν";
    imperfect_act_2: "~ῃεισθᾰ|~ῃεις", "~ῃτον", "~ῃτε";
    imperfect_act_3: "~ῃειν|~ῃει", _, "~ῃσᾰν|~ῃεσᾰν";
    infinitive_pres: "~ῐεναι", _, _;
    participle_pres: "~ῐων", _, _;
    ),
    // Πᾰρεχω: 6 words (ἐπεχω, προσεχω, πᾰρεχω, ἀπεχω, περιεχω, ἀνεχω). Like Πῠρσαινω, except in 41 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("πᾰρεχω", 6, "~εχω", like Πῠρσαινω;
    stem lemma: "~εχ- like βᾰπτῐζω";
    stem rest____: "~ειχ- like ἀειρω";
    stem rest__: "~εξ- like ἐχω";
    stem rest________: "~εσχ- like ἀνᾰγω";
    stem rest_: "~ᾰσχ- like πᾰρεχω";
    stem rest___: "~εσχηκ- like πεμπω";
    stem rest: "~εσχη- like ἐλπω";
    stem rest______: "~εσχη- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Θηραω: 6 words (ἐαω, θηραω, ἀπερᾱω, ἐξερᾱω, θῡμῐαω, φωραω). Like Νῑκαω, except in 15 cells: future_ind_act_3 du, future_ind_mid_3 du, aorist_subj_act_3 du, aorist_subj_mid_3 du, perfect_ind_act_3 du, perfect_ind_mid_3 du, ….
    conjugation!("θηραω", 6, "~αω", like Νῑκαω;
    stem rest__: "~ᾱσ- like γῡροω";
    stem rest_______: "~ᾱθησ- like ἐχω";
    stem rest________: "~ᾱσ- like ποιεω";
    stem rest__________: "~ᾱθην- like ἐχω";
    stem rest_: "~ᾱσ- like ποιεω";
    stem rest_____: "~ᾱθ- like ἐχω";
    stem rest___: "~ᾱκ- like ἐχω";
    stem rest: "~ᾰ- like βουλευω";
    stem rest______: "~ᾰ- like ἐχω";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Περιειμι: 6 words (πᾰρειμι, ἐπειμι, μετειμι, περιειμι, ἐνειμι, σῠνειμῐ). Like Ἐπεξειμι, except in 37 cells: ind_act_1 pl, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, subj_act_1 sg, subj_act_1 pl, ….
    conjugation!("περιειμι", 6, "~ειμι", like Ἐπεξειμι;
    ind_act_1: _, _, "~εσμεν";
    ind_act_2: _, "~εστον", _;
    ind_act_3: "~εστι", _, "~εισι";
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", "~ητον", _;
    subj_act_3: "~ῃ", _, "~ωσι";
    opt_act_1: "~ειην", _, "~ειημεν";
    opt_act_2: "~ειης", "~ειητον", "~ειητε";
    opt_act_3: "~ειη", "~ειητην", "~ειησαν";
    imp_act_2: "~ισθι", _, "~εστε";
    imp_act_3: _, _, "~εστων";
    imperfect_act_1: "~η", _, "~ημεν";
    imperfect_act_2: "~ησθα", "~ηστον", "~ητε";
    imperfect_act_3: "~ην", _, "~ησαν";
    future_ind_mid_1: "~εσομαι", _, _;
    future_ind_mid_2: "~εσῃ", "~εσεσθον", _;
    future_ind_mid_3: "~εσται", _, _;
    future_opt_mid_1: "~εσοιμην", _, _;
    future_opt_mid_3: "~εσοιτο", "~εσοισθην", _;
    infinitive_pres: "~ειναι", _, _;
    participle_pres: "~ων", _, _;
    ),
    // Ἀποκωλῡω: 6 words (ἀπολεγω, ἀποδῠω, ἐπαπολῡω, ἀποκωλῡω, ὑποφερω, ἀποθραυω). Like Μαχομαι, except in 45 cells: ind_act_3 du, ind_mid_1 sg, ind_pass_1 pl, ind_pass_2 sg, subj_mid_1 sg, subj_mid_2 sg, ….
    conjugation!("ἀποκωλῡω", 6, "~ο~ω", like Μαχομαι;
    stem lemma: "~ο~- like βᾰπτῐζω";
    stem rest____: "~ε~- like ἀειρω";
    stem rest__: "~ο~σ- like ἐχω";
    stem rest_______: "~ο~θησ- like ἐχω";
    stem rest________: "~ε~σ- like ἐλπω";
    stem rest__________: "~ε~θην- like ἐλπω";
    stem rest_: "~ο~σ- like ἐχω";
    stem rest_____: "~ο~θ- like ἐχω";
    stem rest___: "~ο%ε~κ- like πεμπω";
    stem rest: "~ο%ε~- like ἐλπω";
    stem rest______: "~ο%ε~- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_mid_1: derived, _, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_1: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_mid_3: derived, _, _;
    imp_pass_2: derived, _, _;
    imperfect_mid_2: _, derived, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Ἡκω: 5 words (ἡκω, πᾰρηκω, προσηκω, τεθνηκω, κᾰθηκω). Like Δῐωκω, except in 75 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἡκω", 5, "~κω", like Δῐωκω;
    stem lemma: "~κ- like βουλευω";
    stem rest____: "~κ- like ἐλπω";
    stem rest__: "~ξ- like ποιεω";
    stem rest________: "~ξ- like εὐλογεω";
    stem rest_: "~ξ- like ποιεω";
    stem rest___: "~κ- like πεμπω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: derived, _, _;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_mid_3: _, derived, _;
    perfect_subj_pass_1: derived, _, derived;
    perfect_subj_pass_2: derived, derived, derived;
    perfect_subj_pass_3: derived, derived, derived;
    perfect_opt_pass_1: derived, _, derived;
    perfect_opt_pass_2: derived, derived, derived;
    perfect_opt_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    pluperfect_act_1: "~κειν", _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: _, derived, _;
    participle_perf: _, _, derived;
    ),
    // Ἱζω: 5 words (ἑλληνῐζω, ἀφῠπνῐζω, ἡσῠχᾰζω, ῥωμᾱϊζω, ἐξῠβρῐζω). Like Λογῐζομαι, except in 65 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἱζω", 5, "~ζω", like Λογῐζομαι;
    stem lemma: "~ζ- like βουλευω";
    stem rest____: "~ζ- like ἐλπω";
    stem rest________: "~σ- like ἐλπω";
    stem rest__________: "~σθην- like ἐλπω";
    stem rest_: "~σ- like ποιεω";
    stem rest___: "~κ- like πεμπω";
    stem rest______: "~- like ἱζω";
    stem rest_________: "~- like ἱζω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: "~σμαι", _, _;
    perfect_ind_mid_3: "~σται", derived, _;
    perfect_ind_pass_1: _, _, "~σμεθᾰ";
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_subj_mid_3: _, "~σμένω ἦτον", "~σμένοι ὦσῐ|~σμένοι ὦσῐν";
    perfect_subj_pass_1: "~σμένος ὦ", _, "~σμένοι ὦμεν";
    perfect_subj_pass_2: "~σμένος ᾖς", "~σμένω ἦτον", "~σμένοι ἦτε";
    perfect_subj_pass_3: "~σμένος ᾖ", "~σμένω ἦτον", "~σμένοι ὦσῐ";
    perfect_opt_pass_1: "~σμένος εἴην", _, "~σμένοι εἴημεν";
    perfect_opt_pass_2: "~σμένος εἴης", "~σμένω εἴητον", "~σμένοι εἴητε";
    perfect_opt_pass_3: "~σμένος εἴη", "~σμένω εἰήτην", "~σμένοι εἴησᾰν";
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "~κειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    participle_perf: _, _, "~σμενος";
    ),
    // Φρᾰζω: 5 words (φρᾰζω, θαυμᾰζω, θοᾰζω, θωᾰζω, χειμᾰζω). Like Πειρᾰζω, except in 5 cells: pluperfect_pass_1 sg, pluperfect_pass_2 sg, pluperfect_pass_2 du, pluperfect_pass_3 sg, pluperfect_pass_3 pl.
    conjugation!("φρᾰζω", 5, "~ζω", like Πειρᾰζω;
    stem rest___________: "~κ- like χρημᾰτῐζω";
    stem rest_________: "~- like χρημᾰτῐζω";
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    ),
    // Πρεπω: 5 words (τρεπω, πρεπω, πᾰρεκτρεπω, λεπω, ρρεπω). Like Ληγω, except in 50 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("πρεπω", 5, "~επω", like Ληγω;
    stem lemma: "~επ- like βᾰπτῐζω";
    stem rest____: "~επ- like ἐχω";
    stem rest__: "~εψ- like ἐχω";
    stem rest_______: "~απησ- like ἐχω";
    stem rest________: "~εψ- like ἐχω";
    stem rest__________: "~εφθην- like ἐχω";
    stem rest_: "~εψ- like ἐχω";
    stem rest_____: "~εφθ- like ἐχω";
    stem rest___: "~οφ- like ἐχω";
    stem rest: "~α- like πρεπω";
    stem rest___________: "~οφειν- like ἐχω";
    stem rest_________: "~α- like πρεπω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_2: "%ε~αψαι", "%ε~αφθον", _;
    perfect_ind_mid_3: _, _, "%ε~αφᾰται";
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, "%ε~αφθε";
    perfect_ind_pass_3: derived, _, _;
    perfect_imp_mid_2: _, _, "%ε~αφθε";
    perfect_imp_mid_3: _, _, "%ε~αφθων";
    perfect_imp_pass_2: "%ε~αψο", _, "%ε~αφθε";
    perfect_imp_pass_3: "%ε~αφθω", "%ε~αφθων", "%ε~αφθων";
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, "%ε~ᾱφθαι", _;
    ),
    // Στερεω: 5 words (στερεω, σῐδηροφορεω, κορεω, σειρεω, γεωμετρεω). Like Σκοπεω, except in 40 cells: future_ind_act_3 du, future_ind_mid_3 du, future_ind_pass_2 sg, aorist_subj_act_3 du, aorist_subj_act_3 pl, aorist_subj_mid_3 du, ….
    conjugation!("στερεω", 5, "~εω", like Σκοπεω;
    stem rest__: "~- like λογῐζομαι";
    stem rest_______: "~ησ- like ἐχω";
    stem rest________: "~ησ- like ἐχω";
    stem rest_: "~ησ- like ἐχω";
    stem rest___: "~ηκ- like στηρῐζω";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_2: derived, _, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: derived, derived, derived;
    perfect_subj_act_3: _, _, derived;
    perfect_subj_mid_3: _, _, derived;
    perfect_subj_pass_3: _, _, "ἐ~ημένοι ὦσῐ";
    pluperfect_act_1: "ἐ~ηκειν", _, _;
    pluperfect_act_2: derived, _, _;
    pluperfect_act_3: derived, _, _;
    future_perfect_ind_mid_1: "ἐ~ησομαι", _, _;
    future_perfect_ind_mid_3: _, "ἐ~ησεσθον", _;
    future_perfect_ind_pass_1: _, _, "ἐ~ησομεθᾰ";
    future_perfect_ind_pass_2: "ἐ~ησῃ", "ἐ~ησεσθον", "ἐ~ησεσθε";
    future_perfect_ind_pass_3: "ἐ~ησεται", "ἐ~ησεσθον", "ἐ~ησονται";
    future_perfect_opt_pass_1: "ἐ~ησοιμην", _, "ἐ~ησοιμεθᾰ";
    future_perfect_opt_pass_2: "ἐ~ησοιο", "ἐ~ησοισθον", "ἐ~ησοισθε";
    future_perfect_opt_pass_3: "ἐ~ησοιτο", "ἐ~ησοισθην", "ἐ~ησοιντο";
    infinitive_futp: _, _, "ἐ~ησεσθαι";
    participle_futp: _, _, "ἐ~ησομενος";
    ),
    // Μᾰρτῠρεω: 5 words (μᾰρτῠρεω, λοιδορεω, μᾰραυγεω, δονεω, κερτομεω). Like Ποιεω, except in 19 cells: future_perfect_ind_mid_1 sg, future_perfect_ind_mid_3 du, future_perfect_ind_pass_1 pl, future_perfect_ind_pass_2 sg, future_perfect_ind_pass_2 du, future_perfect_ind_pass_2 pl, ….
    conjugation!("μᾰρτῠρεω", 5, "~εω", like Ποιεω;
    stem rest__________: "~ηθην- like ἐλπω";
    stem rest_____: "~ηθ- like ἐλπῐζω";
    future_perfect_ind_mid_1: derived, _, _;
    future_perfect_ind_mid_3: _, derived, _;
    future_perfect_ind_pass_1: _, _, derived;
    future_perfect_ind_pass_2: derived, derived, derived;
    future_perfect_ind_pass_3: derived, derived, derived;
    future_perfect_opt_pass_1: derived, _, derived;
    future_perfect_opt_pass_2: derived, derived, derived;
    future_perfect_opt_pass_3: derived, derived, derived;
    infinitive_futp: _, _, derived;
    participle_futp: _, _, derived;
    ),
    // Ἐγγῐζω: 5 words (ἐγγῐζω, ἐλπῐζω, ἐρεθῐζω, ἐνᾰρῐζω, ἐρῐζω). Like Ἀθῡρω, except in 96 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐγγῐζω", 5, "ἐ~ζω", like Ἀθῡρω;
    stem rest____: "~ζ- like ἀθῡρω";
    stem rest__: "~- like ἐλπῐζω";
    stem rest_______: "~σθησ- like ἐλπῐζω";
    stem rest________: "~σ- like ἐλπῐζω";
    stem rest__________: "~σθην- like ἀθῡρω";
    stem rest_: "~σ- like ἐλπῐζω";
    stem rest_____: "~σθ- like ἐλπῐζω";
    stem rest___: "~κ- like ἐλπῐζω";
    stem rest: "~σ- like ἐλπῐζω";
    stem rest___________: "~κ- like ἐλπῐζω";
    ind_act_1: "ἐ~ζω", _, "ἐ~ζομεν";
    ind_act_2: "ἐ~ζεις", "ἐ~ζετον", _;
    ind_act_3: "ἐ~ζει", _, "ἐ~ζουσῐ|ἐ~ζουσῐν";
    ind_mid_1: _, _, "ἐ~ζομεθᾰ";
    ind_mid_2: "ἐ~ζῃ|ἐ~ζει", _, _;
    ind_mid_3: _, "ἐ~ζεσθον", _;
    ind_pass_1: "ἐ~ζομαι", _, _;
    ind_pass_2: _, "ἐ~ζεσθον", "ἐ~ζεσθε";
    ind_pass_3: "ἐ~ζεται", "ἐ~ζεσθον", "ἐ~ζονται";
    subj_act_1: "ἐ~ζω", _, "ἐ~ζωμεν";
    subj_act_2: "ἐ~ζῃς", _, _;
    subj_act_3: "ἐ~ζῃ", "ἐ~ζητον", "ἐ~ζωσῐ|ἐ~ζωσῐν";
    subj_mid_1: _, _, "ἐ~ζωμεθᾰ";
    subj_mid_2: _, "ἐ~ζησθον", _;
    subj_mid_3: "ἐ~ζηται", _, "ἐ~ζωνται";
    subj_pass_1: "ἐ~ζωμαι", _, _;
    subj_pass_2: _, _, "ἐ~ζησθε";
    opt_act_1: "ἐ~ζοιμῐ", _, "ἐ~ζοιμεν";
    opt_act_2: "ἐ~ζοις", "ἐ~ζοιτον", "ἐ~ζοιτε";
    opt_act_3: "ἐ~ζοι", "ἐ~ζοιτην", "ἐ~ζοιεν";
    opt_mid_1: "ἐ~ζοιμην", _, "ἐ~ζοιμεθᾰ";
    opt_mid_2: "ἐ~ζοιο", "ἐ~ζοισθον", "ἐ~ζοισθε";
    opt_mid_3: "ἐ~ζοιτο", "ἐ~ζοισθην", "ἐ~ζοιντο";
    imp_act_2: "ἐ~ζε", _, "ἐ~ζετε";
    imp_act_3: _, _, "ἐ~ζοντων";
    imp_mid_2: "ἐ~ζου", "ἐ~ζεσθον", "ἐ~ζεσθε";
    imp_mid_3: _, _, "ἐ~ζεσθων";
    imp_pass_2: _, "ἐ~ζεσθον", "ἐ~ζεσθε";
    imp_pass_3: "ἐ~ζεσθω", "ἐ~ζεσθων", "ἐ~ζεσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, _, "ἠ~δᾰται";
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_pass_3: _, _, derived;
    pluperfect_mid_1: "ἠ~σμην", _, _;
    pluperfect_mid_3: "ἠ~στο", _, "ἠ~δᾰτο";
    pluperfect_pass_1: _, _, "ἠ~σμεθᾰ";
    future_perfect_ind_mid_1: derived, _, _;
    future_perfect_ind_mid_3: _, derived, _;
    future_perfect_ind_pass_1: _, _, derived;
    future_perfect_ind_pass_2: derived, derived, derived;
    future_perfect_ind_pass_3: derived, derived, derived;
    future_perfect_opt_pass_1: derived, _, derived;
    future_perfect_opt_pass_2: derived, derived, derived;
    future_perfect_opt_pass_3: derived, derived, derived;
    infinitive_pres: "ἐ~ζειν", _, "ἐ~ζεσθαι";
    infinitive_futp: _, _, derived;
    participle_pres: "ἐ~ζων", _, "ἐ~ζομενος";
    participle_futp: _, _, derived;
    ),
    // Ἀναιρεω: 5 words (ἀναιρεω, ἀφαιρεω, ἐξαιρεω, δῐαιρεω, ὑφαιρεω). Like Πῠρσαινω, except in 71 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἀναιρεω", 5, "~αιρεω", like Πῠρσαινω;
    stem lemma: "~αιρ- like ποιεω";
    stem rest____: "~ῃρ- like εὐλογεω";
    stem rest__: "~αιρησ- like ποιεω";
    stem rest_______: "~αιρεθησ- like ποιεω";
    stem rest________: "~- like ἀναιρεω";
    stem rest__________: "~ῃρεθην- like ἐλπω";
    stem rest_____: "~αιρεθ- like ποιεω";
    stem rest___: "~ῃρηκ- like εὐλογεω";
    stem rest: "~ῃρη- like εὐλογεω";
    stem rest______: "~ῃρη- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_1: "~ελω|~αιρησω", _, "~ελωμεν|~αιρησωμεν";
    aorist_subj_act_2: "~ελῃς|~αιρησῃς", "~ελητον|~αιρησητον", "~ελητε|~αιρησητε";
    aorist_subj_act_3: "~ελῃ|~αιρησῃ", _, "~ελωσῐ|~ελωσῐν|~αιρησωσῐ|~αιρησωσῐν";
    aorist_subj_mid_1: _, _, "~ελωμεθᾰ";
    aorist_subj_mid_2: "~ελῃ", "~ελησθον", _;
    aorist_opt_act_1: "~ελοιμῐ|~αιρησαιμῐ", _, "~ελοιμεν|~αιρησαιμεν";
    aorist_opt_act_2: "~ελοις|~αιρησειᾰς|~αιρησαις", "~ελοιτον|~αιρησαιτον", "~ελοιτε|~αιρησαιτε";
    aorist_opt_act_3: "~ελοι|~αιρησειε|~αιρησειεν|~αιρησαι", "~ελοιτην|~αιρησαιτην", "~ελοιεν|~αιρησειᾰν|~αιρησαιεν";
    aorist_opt_mid_1: "~ελοιμην", _, _;
    aorist_imp_act_2: "~ελε|~αιρησον", "~ελετον|~αιρησᾰτον", "~ελετε|~αιρησᾰτε";
    aorist_imp_act_3: "~ελετω|~αιρησᾰτω", "~ελετων|~αιρησᾰτων", "~ελοντων|~αιρησᾰντων";
    aorist_imp_mid_2: "~ελου", "~ελεσθον", _;
    aorist_imp_mid_3: _, _, "~ελεσθων";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_aor: "~ελειν|~αιρησαι", _, _;
    infinitive_perf: _, _, derived;
    participle_aor: "~ελων|~αιρησᾱς", "~ελομενος", _;
    ),
    // Ἀνοικῐζω: 5 words (ἀνοικῐζω, κᾰτοικῐζω, σῠνοικῐζω, ξῠνοικῐζω, ἀποικῐζω). Like Δῐοικεω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἀνοικῐζω", 5, "~οι~ζω", like Δῐοικεω;
    stem lemma: "~οι~ζ- like βουλευω";
    stem rest____: "~ῳ~ζ- like ἐλπω";
    stem rest__: "~οι~- like χρημᾰτῐζω";
    stem rest_______: "~οι~σθησ- like ποιεω";
    stem rest________: "~ῳ~σ- like εὐλογεω";
    stem rest__________: "~ῳ~σθην- like ἐλπω";
    stem rest_: "~οι~σ- like ποιεω";
    stem rest_____: "~οι~σθ- like ποιεω";
    stem rest___: "~ῳ~κ- like εὐλογεω";
    stem rest: "~ῳ~σ- like εὐλογεω";
    stem rest______: "~ῳ~- like ἱζω";
    stem rest___________: "~ῳ~κ- like εὐλογεω";
    stem rest_________: "~ῳ~- like ἱζω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Κᾰτᾰλᾰμβᾰνω: 5 words (κᾰτᾰλᾰμβᾰνω, ἀνᾰλᾰμβᾰνω, προκᾰτᾰλᾰμβᾰνω, μετᾰλαμβᾰνω, ἐπᾰνᾰλᾰμβᾰνω). Like Πῠρσαινω, except in 45 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("κᾰτᾰλᾰμβᾰνω", 5, "~ᾰλᾰμβᾰνω", like Πῠρσαινω;
    stem lemma: "~ᾰλᾰμβᾰν- like βουλευω";
    stem rest____: "~ελᾰμβᾰν- like ἐλπω";
    stem rest__: "~ᾰληψ- like εἰσερχομαι";
    stem rest_______: "~ᾰληφθησ- like ποιεω";
    stem rest________: "~ελᾰβ- like ἀνᾰλᾰμβᾰνω";
    stem rest__________: "~εληφθην- like ἐλπω";
    stem rest_: "~ᾰλᾰβ- like ἀνᾰγω";
    stem rest_____: "~ᾰληφθ- like ποιεω";
    stem rest___: "~ειληφ- like πεμπω";
    stem rest: "~ειλη- like ἀνᾰλᾰμβᾰνω";
    stem rest______: "~ειλη- like ἀνᾰλᾰμβᾰνω";
    stem rest___________: "~ειληφ- like εὐλογεω";
    stem rest_________: "~ειλη- like ἀνᾰλᾰμβᾰνω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Μετερχομαι: 5 words (προερχομαι, σῠνερχομαι, ἐπεξερχομαι, μετερχομαι, ξῠνερχομαι). Like Ἐξερχομαι, except in 15 cells: ind_mid_2 du, ind_mid_3 du, ind_pass_2 pl, ind_pass_3 sg, ind_pass_3 pl, subj_pass_2 sg, ….
    conjugation!("μετερχομαι", 5, "~ερχομαι", like Ἐξερχομαι;
    stem rest___: "~εληλῠθ- like εὐλογεω";
    ind_mid_2: _, derived, _;
    ind_mid_3: _, "~ερχεσθον", _;
    ind_pass_2: _, _, derived;
    ind_pass_3: derived, _, derived;
    subj_pass_2: derived, _, derived;
    imp_mid_2: _, "~ερχεσθον", _;
    imp_pass_2: _, _, derived;
    imp_pass_3: _, derived, derived;
    perfect_ind_act_3: _, derived, _;
    pluperfect_act_1: "~εληλυθειν", _, _;
    infinitive_pres: _, _, derived;
    participle_pres: _, _, derived;
    ),
    // Τελλω: 5 words (στελλω, ἀγγελλω, τῐλλω, τελλω, σκελλω). Like Βᾰδιζω, except in 60 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("τελλω", 5, "~~λω", like Βᾰδιζω;
    stem lemma: "~~λ- like βᾰπτῐζω";
    stem rest____: "~~λ- like ἐχω";
    stem rest__: "~~- like λογῐζομαι";
    stem rest_______: "~~θησ- like ἐχω";
    stem rest________: "~ι~- like ἐχω";
    stem rest_: "~ι~- like ἐχω";
    stem rest___: "~τα~κ- like πεμπω";
    stem rest: "~τα~- like ἐλπω";
    stem rest_________: "~τα~- like ἐγειρω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_opt_act_2: _, _, derived;
    future_opt_act_3: _, _, derived;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_2: "~τα~σαι", "~τα~θον", _;
    perfect_ind_mid_3: _, _, "~τα~ᾰται";
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, "~τα~θε";
    perfect_ind_pass_3: derived, _, _;
    perfect_imp_mid_2: _, _, "~τα~θε";
    perfect_imp_mid_3: _, _, "~τα~θων";
    perfect_imp_pass_2: "~τα~σο", _, "~τα~θε";
    perfect_imp_pass_3: "~τα~θω", "~τα~θων", "~τα~θων";
    pluperfect_act_1: "ἐ~τα~κειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, "~τᾱ~θαι", _;
    ),
    // Φειδομαι: 5 words (εἰδομαι, ἐλδομαι, ἡδομαι, μηδομαι, φειδομαι). Like Λλῡμῐ, except in 57 cells: ind_act_1 sg, ind_act_3 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ….
    conjugation!("φειδομαι", 5, "~δομαι", like Λλῡμῐ;
    stem rest____: "~δ- like εἰσερχομαι";
    stem rest__: "~σ- like εἰσερχομαι";
    stem rest_______: "~σθησ- like ἐχω";
    stem rest________: "~σ- like ἑπομαι";
    stem rest_: "~σ- like ἑπομαι";
    ind_act_1: "~δομαι", _, _;
    ind_act_3: derived, _, _;
    ind_mid_1: "~δομαι", _, "~δομεθᾰ";
    ind_mid_2: "~δῃ|~δει", _, _;
    ind_mid_3: _, "~δεσθον", _;
    ind_pass_2: "~δῃ", "~δεσθον", "~δεσθε";
    ind_pass_3: "~δεται", "~δεσθον", "~δονται";
    subj_mid_1: "~δωμαι", _, "~δωμεθᾰ";
    subj_mid_2: "~δῃ", "~δησθον", _;
    subj_mid_3: "~δηται", _, "~δωνται";
    subj_pass_2: "~δῃ", _, "~δησθε";
    opt_mid_1: "~δοιμην", _, "~δοιμεθᾰ";
    opt_mid_2: "~δοιο", "~δοισθον", "~δοισθε";
    opt_mid_3: "~δοιτο", "~δοισθην", "~δοιντο";
    imp_mid_2: "~δου", "~δεσθον", "~δεσθε";
    imp_mid_3: "~δεσθω", _, "~δεσθων";
    imp_pass_2: _, "~δεσθον", "~δεσθε";
    imp_pass_3: _, "~δεσθων", "~δεσθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_1: "%ε~σμαι", _, _;
    perfect_ind_mid_2: "%ε~σαι", "%ε~σθον", _;
    perfect_ind_mid_3: "%ε~σται", _, "%ε~δᾰται";
    perfect_subj_mid_3: _, "%ε~σμένω ἦτον", _;
    perfect_imp_mid_2: _, _, "%ε~σθε";
    perfect_imp_mid_3: _, _, "%ε~σθων";
    infinitive_pres: _, _, "~δεσθαι";
    participle_pres: _, _, "~δομενος";
    ),
    // Σκηπτω: 5 words (ῥᾰπτω, ῥῑπτω, σκᾰπτω, ρρῑπτω, ρρᾰπτω). Like Θρῠπτω, except in 34 cells: imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_2 du, imperfect_pass_3 sg, imperfect_pass_3 pl, ….
    conjugation!("σκηπτω", 5, "~πτω", like Θρῠπτω;
    stem rest____: "~πτ- like σκηπτω";
    stem rest________: "~ψ- like ἐχω";
    stem rest__________: "~φην- like ἐχω";
    stem rest_: "~ψ- like ἐχω";
    stem rest_____: "~φ- like ἐγειρω";
    stem rest___: "~φ- like σκοπεω";
    stem rest: "~- like σκηπτω";
    stem rest______: "~- like σκηπτω";
    stem rest_________: "~- like σκηπτω";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_pass_2: derived, _, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "ἐ~φειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ποιμαινω: 5 words (ἐπῐφαινω, ποιμαινω, χλῑαινω, λῑμαινω, ξηραινω). Like Μαχομαι, except in 67 cells: ind_act_3 du, ind_mid_1 sg, ind_pass_1 pl, ind_pass_2 sg, subj_mid_1 sg, subj_mid_2 sg, ….
    conjugation!("ποιμαινω", 5, "~~ινω", like Μαχομαι;
    stem lemma: "~~ιν- like βᾰπτῐζω";
    stem rest____: "~~ιν- like ποιμαινω";
    stem rest__: "~~ν- like λογῐζομαι";
    stem rest________: "~~ν- like ἐχω";
    stem rest__________: "~~νθην- like ἐχω";
    stem rest_____: "~~νθ- like ἐχω";
    stem rest: "~~- like πορσῡνω";
    ind_act_3: _, derived, _;
    ind_mid_1: derived, _, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_1: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_mid_3: derived, _, _;
    imp_pass_2: derived, _, _;
    imperfect_mid_2: _, derived, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_1: "~~νω", _, _;
    aorist_subj_act_2: "~~νῃς", "~~νητον", _;
    aorist_subj_mid_1: _, _, "~~νωμεθᾰ";
    aorist_subj_mid_2: "~~νῃ", "~~νησθον", _;
    aorist_opt_act_1: "~~ναιμῐ", _, "~~ναιμεν";
    aorist_opt_act_2: "~~νειᾰς", _, _;
    aorist_opt_act_3: "~~νειε", _, "~~νειᾰν";
    aorist_opt_mid_1: "~~ναιμην", _, _;
    aorist_imp_act_2: "~~νον", _, "~~νᾰτε";
    aorist_imp_act_3: _, _, "~~νᾰντων";
    aorist_imp_mid_2: "~~ναι", "~~νᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~νᾰσθων";
    perfect_ind_mid_2: "%ε~~νσαι", "%ε~~νθον", _;
    perfect_ind_mid_3: _, _, "%ε~~νᾰται";
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, "%ε~~νθε";
    perfect_ind_pass_3: derived, _, _;
    perfect_imp_mid_2: _, _, "%ε~~νθε";
    perfect_imp_mid_3: _, _, "%ε~~νθων";
    perfect_imp_pass_2: "%ε~~νσο", _, "%ε~~νθε";
    perfect_imp_pass_3: "%ε~~νθω", "%ε~~νθων", "%ε~~νθων";
    infinitive_aor: "~ᾱν~ι", _, _;
    infinitive_perf: _, "%ε~ᾱνθ~ι", _;
    participle_aor: "~~νᾱς", "~~νᾰμενος", _;
    ),
    // Βᾰθῠνω: 5 words (βᾰρῠνω, βᾰθῠνω, πλᾰτῠνω, βρᾰδῠνω, βρᾰχῠνω). Like Ποιμαινω, except in 41 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("βᾰθῠνω", 5, "~ᾰ~ῠνω", like Ποιμαινω;
    stem lemma: "~ᾰ~ῠν- like βᾰπτῐζω";
    stem rest____: "~ᾰ~ῠν- like ἐχω";
    stem rest__: "~ᾰ~ῠν- like λογῐζομαι";
    stem rest________: "~ᾰ~ῡν- like ἐχω";
    stem rest__________: "~ᾰ~ῡνθην- like ἐχω";
    stem rest_: "~ᾰ~ῡν- like ἐχω";
    stem rest_____: "~ᾰ~ῡνθ- like ἐχω";
    stem rest___: "~εβᾱ~ῠγκ- like πεμπω";
    stem rest: "~εβᾰ~ῡσ- like ἐλπω";
    stem rest______: "~εβᾰ~ῡσ- like ἐλπω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Ἐξᾰκολουθεω: 5 words (πᾰρᾰμελεω, σῠνᾰδῐκεω, ξῠνᾰδῐκεω, ἀπᾰσχολεω, ἐξᾰκολουθεω). Like Λᾰγοθηρεω, except in 22 cells: imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_3 sg, imperfect_pass_3 pl, perfect_ind_act_3 du, ….
    conjugation!("ἐξᾰκολουθεω", 5, "~ᾰ~εω", like Λᾰγοθηρεω;
    stem rest____: "~η~- like εὐλογεω";
    stem rest________: "~η~ησ- like εὐλογεω";
    stem rest__________: "~η~ηθην- like ἐλπω";
    stem rest___: "~η~ηκ- like εὐλογεω";
    stem rest: "~η~η- like εὐλογεω";
    stem rest______: "~η~η- like εὐλογεω";
    stem rest___________: "~η~ηκ- like εὐλογεω";
    stem rest_________: "~η~η- like εὐλογεω";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Προκᾰλεω: 5 words (προσκᾰλεω, σῠγκᾰλεω, προκᾰλεω, ἐγκᾰλεω, ἐκκᾰλεω).
    conjugation!("προκᾰλεω", 5, "~~ᾰ~εω";
    stem lemma: "~~ᾰ~- like ποιεω";
    stem rest____: "~υ~ᾰ~- like εὐλογεω";
    stem rest__: "~~ᾰ~εσ- like ποιεω";
    stem rest_______: "~~~ηθησ- like ποιεω";
    stem rest________: "~υ~ᾰ~εσ- like ἐλπω";
    stem rest__________: "~υ~~ηθην- like ἐλπω";
    stem rest_: "~~ᾰ~εσ- like ποιεω";
    stem rest_____: "~~~ηθ- like ποιεω";
    stem rest___: "~~εκ~ηκ- like εὐλογεω";
    stem rest: "~~εκ~η- like εὐλογεω";
    stem rest______: "~~εκ~η- like εὐλογεω";
    stem rest_________: "~υ%ε~~η- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "~υ%ε~~ηκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Περαινω: 5 words (κραινω, ὑγῐαινω, ὑδραινω, περαινω, τετραινω). Like Κερδαινω, except in 24 cells: imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_3 sg, imperfect_pass_3 pl, future_ind_act_3 du, ….
    conjugation!("περαινω", 5, "~αινω", like Κερδαινω;
    stem rest____: "~αιν- like βουλευω";
    stem rest__: "~ᾰν- like χρημᾰτῐζω";
    stem rest_______: "~ᾰνθησ- like ἐχω";
    stem rest________: "~ᾱν- like ποιεω";
    stem rest__________: "~ᾰνθην- like ἐχω";
    stem rest_: "~ᾱν- like ποιεω";
    stem rest_____: "~ᾰνθ- like ἐχω";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_2: derived, _, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_subj_act_3: _, derived, _;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    ),
    // Ἡγεομαι: 4 words (ἡγεομαι, νεομαι, ἐξηγεομαι, εὐνομεομαι). Like Βουλομαι, except in 37 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἡγεομαι", 4, "~εομαι", like Βουλομαι;
    stem rest____: "~- like ἡγεομαι";
    stem rest_______: "~ηθησ- like ἐχω";
    stem rest________: "~ησ- like μηκᾰομαι";
    stem rest__________: "~ηθην- like ἐλπω";
    stem rest_: "~ησ- like ἑπομαι";
    stem rest: "~η- like ἐλπω";
    stem rest______: "~η- like εὐλογεω";
    ind_act_1: "~εομαι", _, _;
    ind_mid_1: "~ουμαι", _, "~ουμεθᾰ";
    ind_mid_2: "~ει", _, _;
    ind_mid_3: _, "~εισθον", _;
    ind_pass_2: _, "~εισθον", "~εισθε";
    ind_pass_3: "~ειται", "~εισθον", "~ουνται";
    imp_mid_2: _, "~εισθον", "~εισθε";
    imp_mid_3: "~εισθω", _, "~εισθων";
    imp_pass_2: _, "~εισθον", "~εισθε";
    imp_pass_3: _, "~εισθων", "~εισθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_pres: _, _, "~εισθαι";
    infinitive_perf: _, _, derived;
    participle_pres: _, _, "~ουμενος";
    ),
    // Κοπτω: 4 words (κοπτω, τῠπτω, βλᾰπτω, σκωπτω). Like Θρῠπτω, except in 36 cells: future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, aorist_opt_pass_3 du, ….
    conjugation!("κοπτω", 4, "~πτω", like Θρῠπτω;
    stem rest_______: "~πησ- like ἐχω";
    stem rest__________: "~πην- like ἐχω";
    stem rest_____: "~π- like ἐγειρω";
    stem rest___: "~φ- like ποιεω";
    stem rest___________: "~φ- like χρημᾰτῐζω";
    future_ind_pass_2: derived, _, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, _, "%ε~πᾰται";
    pluperfect_mid_1: "ἐ%ε~μμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ψο", "ἐ%ε~φθον", _;
    pluperfect_mid_3: "ἐ%ε~πτο", _, "ἐ%ε~πᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~μμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~φθε";
    pluperfect_pass_3: _, "ἐ%ε~φθην", _;
    future_perfect_ind_mid_1: "%ε~ψομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~ψεσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~ψομεθᾰ";
    future_perfect_ind_pass_2: "%ε~ψῃ", "%ε~ψεσθον", "%ε~ψεσθε";
    future_perfect_ind_pass_3: "%ε~ψεται", "%ε~ψεσθον", "%ε~ψονται";
    future_perfect_opt_pass_1: "%ε~ψοιμην", _, "%ε~ψοιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~ψοιο", "%ε~ψοισθον", "%ε~ψοισθε";
    future_perfect_opt_pass_3: "%ε~ψοιτο", "%ε~ψοισθην", "%ε~ψοιντο";
    infinitive_futp: _, _, "%ε~ψεσθαι";
    participle_futp: _, _, "%ε~ψομενος";
    ),
    // Ἐπᾰγγελλω: 4 words (πᾰρᾰγγελλω, ἀπᾰγγελλω, ἐπᾰγγελλω, ἀνᾰγγελλω). Like Προκᾰλεω, except in 51 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἐπᾰγγελλω", 4, "~ᾰ~~λω", like Προκᾰλεω;
    stem lemma: "~ᾰ~~λ- like βουλευω";
    stem rest____: "~η~~λ- like ἐλπω";
    stem rest__: "~ᾰ~~- like χρημᾰτῐζω";
    stem rest_______: "~ᾰ~~θησ- like ποιεω";
    stem rest________: "~η~ι~- like εὐλογεω";
    stem rest__________: "~η~~θην- like ἐλπω";
    stem rest_: "~ᾰ~ι~- like ποιεω";
    stem rest_____: "~ᾰ~~θ- like ποιεω";
    stem rest___: "~η~~κ- like εὐλογεω";
    stem rest: "~η~~- like εὐλογεω";
    stem rest______: "~η~~- like ἀειρω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Λειβω: 4 words (στειβω, λειβω, ἀμειβω, στιλβω). Like Πῠρσαινω, except in 28 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("λειβω", 4, "~βω", like Πῠρσαινω;
    stem lemma: "~β- like βᾰπτῐζω";
    stem rest____: "~β- like ἐχω";
    stem rest________: "~ψ- like ἐχω";
    stem rest_: "~ψ- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    ),
    // Ἐξᾰττῐκῐζω: 4 words (ἐξᾰφᾰνῐζω, σῠνᾰρπᾰζω, δῐᾰρπᾰζω, ἐξᾰττῐκῐζω). Like Ἐπᾰϊω, except in 45 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἐξᾰττῐκῐζω", 4, "~ᾰ~ζω", like Ἐπᾰϊω;
    stem lemma: "~ᾰ~ζ- like βουλευω";
    stem rest____: "~η~ζ- like ἐλπω";
    stem rest__: "~ᾰ~- like λογῐζομαι";
    stem rest_______: "~ᾰ~σθησ- like ποιεω";
    stem rest_____: "~ᾰ~σθ- like ποιεω";
    stem rest___: "~η~κ- like εὐλογεω";
    stem rest: "~η~σ- like εὐλογεω";
    stem rest______: "~η~- like ἱζω";
    stem rest_________: "~η~- like ἱζω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Προφερω: 4 words (προφερω, προσφερω, περῐφερω, ὑπερφερω). Like Πῠρσαινω, except in 31 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("προφερω", 4, "~φερω", like Πῠρσαινω;
    stem lemma: "~φερ- like βουλευω";
    stem rest____: "~εφερ- like ἐλπω";
    stem rest__: "~οισ- like ποιεω";
    stem rest_______: "~οισθησ- like ἐχω";
    stem rest________: "~ηνεγκ- like ἐλπω";
    stem rest__________: "~ηνεχθην- like ἐλπω";
    stem rest_: "~ενεγκ- like ἐχω";
    stem rest_____: "~ενεχθ- like ἐχω";
    stem rest___: "~ενηνεχ- like πεμπω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    ),
    // Περῐτρεχω: 4 words (περῐτρεχω, κᾰτᾰτρεχω, ἐπιτρεχω, ἀνατρεχω). Like Μαχομαι, except in 78 cells: ind_act_3 du, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, subj_act_3 du, ….
    conjugation!("περῐτρεχω", 4, "~τ~εχω", like Μαχομαι;
    stem lemma: "~τ~εχ- like ἐπιτρεχω";
    stem rest____: "~ετ~εχ- like ἐλπω";
    stem rest________: "~εθ~εξ- like ἐλπω";
    stem rest_: "~θ~εξ- like ἐχω";
    stem rest___: "~δεδ~ᾰμηκ- like πεμπω";
    ind_act_3: _, derived, _;
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, _, _;
    ind_mid_3: _, derived, _;
    subj_act_3: _, derived, _;
    subj_mid_1: derived, _, derived;
    subj_mid_2: derived, derived, _;
    subj_mid_3: derived, _, derived;
    subj_pass_2: derived, _, _;
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, derived;
    opt_mid_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_mid_2: derived, derived, derived;
    imp_mid_3: derived, _, derived;
    imperfect_mid_2: _, derived, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_1: "~δ~ᾰμουμαι", _, _;
    future_ind_mid_2: "~δ~ᾰμει", "~δ~ᾰμεισθον", _;
    future_ind_mid_3: "~δ~ᾰμειται", _, _;
    future_opt_mid_1: "~δ~ᾰμοιμην", _, _;
    future_opt_mid_3: "~δ~ᾰμοιτο", "~δ~ᾰμοισθην", _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: derived, _, _;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_mid_3: _, derived, _;
    perfect_subj_pass_1: derived, _, derived;
    perfect_subj_pass_2: derived, derived, derived;
    perfect_subj_pass_3: derived, derived, derived;
    perfect_opt_pass_1: derived, _, derived;
    perfect_opt_pass_2: derived, derived, derived;
    perfect_opt_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    infinitive_pres: _, _, derived;
    infinitive_perf: _, derived, _;
    participle_pres: _, _, derived;
    participle_perf: _, _, derived;
    ),
    // Δερω: 4 words (τρεφω, δερω, δεμω, ξῠμμενω). Like Φθειρω, except in 46 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("δερω", 4, "~ε~ω", like Φθειρω;
    stem lemma: "~ε~- like βᾰπτῐζω";
    stem rest____: "~ε~- like ἐχω";
    stem rest________: "~ει~- like ἐχω";
    stem rest_: "~ει~- like ἐχω";
    stem rest___: "~εδᾰ~κ- like πεμπω";
    stem rest: "~εδᾰ~- like ἐλπω";
    stem rest______: "~εδᾰ~- like ἀειρω";
    stem rest_________: "~εδᾰ~- like ἐγειρω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "ἐ~εδᾰ~κειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Γελᾰω: 4 words (σπᾰω, κλᾰω, γελᾰω, ἀντῐᾰω). Like Θηραω, except in 28 cells: future_ind_act_3 du, future_ind_mid_3 du, aorist_subj_act_3 du, aorist_subj_mid_3 du, perfect_ind_act_1 pl, perfect_ind_act_2 du, ….
    conjugation!("γελᾰω", 4, "~ᾰω", like Θηραω;
    stem rest__: "~ᾰσ- like ποιεω";
    stem rest_______: "~ᾰσθησ- like ἐχω";
    stem rest________: "~ᾰσ- like ποιεω";
    stem rest__________: "~ᾰσθην- like ἐχω";
    stem rest_: "~ᾰσ- like ποιεω";
    stem rest_____: "~ᾰσθ- like ἐχω";
    stem rest: "~ᾰσ- like βουλευω";
    stem rest______: "~ᾰσ- like χεω";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_act_3: _, derived, _;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Βῐᾰζομαι: 4 words (στοχᾰζομαι, ἐργᾰζομαι, ἀσπᾰζομαι, βῐᾰζομαι). Like Κρῑνομαι, except in 34 cells: ind_pass_1 sg, ind_pass_1 pl, ind_pass_2 sg, subj_mid_3 du, subj_pass_1 sg, subj_pass_1 pl, ….
    conjugation!("βῐᾰζομαι", 4, "~ζομαι", like Κρῑνομαι;
    stem lemma: "~ζ- like οἰομαι";
    stem rest____: "~ζ- like σῑνομαι";
    stem rest_______: "~σθησ- like ἐχω";
    stem rest__________: "~σθην- like ἐχω";
    stem rest_____: "~σθ- like ἐχω";
    stem rest: "~σ- like βουλευω";
    stem rest_________: "~- like χρημᾰτῐζω";
    ind_pass_1: derived, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: derived, _, derived;
    subj_pass_2: _, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_pass_2: derived, _, _;
    imp_pass_3: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    perfect_ind_mid_3: _, _, "%ε~δᾰται";
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    ),
    // Μᾰρνᾰμαι: 4 words (ἐρᾰμαι, μᾰρνᾰμαι, ἀγᾰμαι, λῐνᾰμαι). Like Δαμνημῐ, except in 41 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("μᾰρνᾰμαι", 4, "~ᾰμαι", like Δαμνημῐ;
    ind_act_1: "~ᾰμαι", _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: derived, _, derived;
    ind_mid_1: "~ᾰμαι", _, _;
    ind_pass_1: derived, _, _;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_1: "~ωμαι", _, _;
    subj_mid_2: "~ῃ", _, _;
    subj_pass_1: derived, _, _;
    subj_pass_2: "~ῃ", _, _;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imp_mid_3: "~ᾰσθω", _, _;
    imp_pass_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    imperfect_mid_2: _, "ἐ~ᾰσθον", _;
    imperfect_pass_2: _, derived, _;
    infinitive_pres: derived, _, _;
    participle_pres: derived, _, _;
    ),
    // Νεκραγωγεω: 4 words (κατοικεω, νεκραγωγεω, πᾰνηγῠρῐαρχεω, ἐξασθενεω). Like Λᾰγοθηρεω, except in 35 cells: ind_mid_2 sg, imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_3 sg, imperfect_pass_3 pl, ….
    conjugation!("νεκραγωγεω", 4, "~α~εω", like Λᾰγοθηρεω;
    stem rest____: "~ᾰ~- like εὐλογεω";
    stem rest__: "~ᾰ~ησ- like ἐχω";
    stem rest________: "~ᾰ~ησ- like ἐλπω";
    stem rest__________: "~ᾰ~ηθην- like ἐλπω";
    stem rest_: "~ᾰ~ησ- like ἐχω";
    stem rest___: "~ᾰ~ηκ- like πεμπω";
    stem rest: "~ᾰ~η- like ἐλπω";
    stem rest______: "~ᾰ~η- like εὐλογεω";
    stem rest_________: "~ᾰ~η- like εὐλογεω";
    ind_mid_2: "~ᾰ~ει", _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_2: derived, _, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "~ᾰ~ηκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ἀνᾰπτω: 4 words (ἀνᾰπτω, ἐξᾰπτω, καθᾰπτω, ἐπῐσῠνᾰπτω). Like Πῠρσαινω, except in 46 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἀνᾰπτω", 4, "~ᾰπτω", like Πῠρσαινω;
    stem lemma: "~ᾰπτ- like βουλευω";
    stem rest____: "~ηπτ- like ἐλπω";
    stem rest__: "~ᾰψ- like ποιεω";
    stem rest_______: "~ᾰφθησ- like ἐχω";
    stem rest________: "~ηψ- like εὐλογεω";
    stem rest__________: "~ηφθην- like ἐλπω";
    stem rest_: "~ᾰψ- like ποιεω";
    stem rest_____: "~ᾰφθ- like ποιεω";
    stem rest: "~η- like ἀνᾰπτω";
    stem rest______: "~η- like ἀνᾰπτω";
    stem rest_________: "~η- like ἀνᾰπτω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Πειρᾰω: 4 words (κοπῐᾰω, ἀᾰω, πειρᾰω, μειδῐᾰω). Like Θηραω, except in 19 cells: future_ind_act_3 du, future_ind_mid_3 du, future_ind_pass_1 sg, future_ind_pass_3 du, perfect_ind_mid_3 du, perfect_ind_pass_1 sg, ….
    conjugation!("πειρᾰω", 4, "~ᾰω", like Θηραω;
    stem rest__: "~ᾱσ- like ἐχω";
    stem rest: "~ᾱ- like βουλευω";
    stem rest______: "~ᾱ- like ἐχω";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_mid_1: "ἐ%ε~ᾱμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ᾱσο", "ἐ%ε~ᾱσθον", _;
    pluperfect_mid_3: "ἐ%ε~ᾱτο", _, "ἐ%ε~ᾱντο";
    infinitive_perf: _, _, derived;
    ),
    // Πᾰρᾰλλᾰσσω: 4 words (ἀπᾰλλᾰσσω, ὑπᾰλλᾰσσω, ἀντᾰλλᾰσσω, πᾰρᾰλλᾰσσω). Like Ἀποκωλῡω, except in 45 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("πᾰρᾰλλᾰσσω", 4, "~ᾰ~σσω", like Ἀποκωλῡω;
    stem lemma: "~ᾰ~σσ- like βουλευω";
    stem rest____: "~η~σσ- like ἐλπω";
    stem rest__: "~ᾰ~ξ- like ποιεω";
    stem rest_______: "~ᾰ~χθησ- like ποιεω";
    stem rest________: "~η~ξ- like ἐλπω";
    stem rest__________: "~η~- like πᾰρᾰλλᾰσσω";
    stem rest_: "~ᾰ~ξ- like ἐχω";
    stem rest_____: "~ᾰ~- like πᾰρᾰλλᾰσσω";
    stem rest___: "~η~χ- like εὐλογεω";
    stem rest: "~η~- like πᾰρᾰλλᾰσσω";
    stem rest______: "~η~- like πᾰρᾰλλᾰσσω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_imp_pass_2: _, derived, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Ἀμεργω: 4 words (ἐργω, εἰργω, ἀμεργω, κᾰτειργω). Like Ληγω, except in 43 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἀμεργω", 4, "~γω", like Ληγω;
    stem lemma: "~γ- like βουλευω";
    stem rest____: "~γ- like ἀειρω";
    stem rest__: "~ξ- like ποιεω";
    stem rest________: "~ξ- like ἐλπω";
    stem rest__________: "~χθην- like ἐλπω";
    stem rest: "~- like ἀνᾰγω";
    stem rest______: "~- like πᾰρᾰλλᾰσσω";
    stem rest_________: "~- like κᾰτειργω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Πᾰροξῡνω: 4 words (παροτρῡνω, ἐποτρῡνω, πᾰροξῡνω, ἐξοτρῡνω). Like Ἀποκωλῡω, except in 41 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("πᾰροξῡνω", 4, "~ο~ῡνω", like Ἀποκωλῡω;
    stem lemma: "~ο~ῡν- like βᾰπτῐζω";
    stem rest____: "~ω~ῡν- like ἀειρω";
    stem rest__: "~ο~ῠν- like λογῐζομαι";
    stem rest_______: "~ο~ῠνθησ- like ἐχω";
    stem rest________: "~ω~ῡν- like ἐλπω";
    stem rest__________: "~ω~ῠνθην- like ἐλπω";
    stem rest_: "~ο~ῡν- like ἐχω";
    stem rest_____: "~ο~ῠνθ- like ἐχω";
    stem rest___: "~ω~ῠγκ- like πεμπω";
    stem rest: "~ω~ῠ- like ἀνᾰλᾰμβᾰνω";
    stem rest______: "~ω~ῠ- like ἀνᾰπτω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Ἐλᾰσσοω: 4 words (ἐρημοω, ἐλᾰσσοω, ἐλευθεροω, ἐπουλοω). Like Πῠρσαινω, except in 86 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐλᾰσσοω", 4, "ἐ~οω", like Πῠρσαινω;
    stem rest____: "~ου- like ἐλᾰσσοω";
    stem rest__: "~ωσ- like ἐλᾰσσοω";
    stem rest_______: "~ωθησ- like ἐλπῐζω";
    stem rest________: "~ωσ- like ἐλπῐζω";
    stem rest__________: "~ωθην- like ἀθῡρω";
    stem rest_: "~ωσ- like ἐλπῐζω";
    stem rest_____: "~ωθ- like ἐλπῐζω";
    stem rest___: "~ωκ- like ἀθῡρω";
    stem rest: "~ω- like ἐλπῐζω";
    stem rest______: "~ω- like ἀθῡρω";
    stem rest_________: "~ω- like ἀθῡρω";
    ind_act_1: "ἐ~ω", _, "ἐ~ουμεν";
    ind_act_2: "ἐ~οις", "ἐ~ουτον", _;
    ind_act_3: "ἐ~οι", _, "ἐ~ουσῐ|ἐ~ουσῐν";
    ind_mid_1: _, _, "ἐ~ουμεθᾰ";
    ind_mid_2: "ἐ~οι", _, _;
    ind_mid_3: _, "ἐ~ουσθον", _;
    ind_pass_1: "ἐ~ουμαι", _, _;
    ind_pass_2: _, "ἐ~ουσθον", "ἐ~ουσθε";
    ind_pass_3: "ἐ~ουται", "ἐ~ουσθον", "ἐ~ουνται";
    subj_act_1: "ἐ~ω", _, "ἐ~ωμεν";
    subj_act_2: "ἐ~οις", _, _;
    subj_act_3: "ἐ~οι", "ἐ~ωτον", "ἐ~ωσῐ|ἐ~ωσῐν";
    subj_mid_1: _, _, "ἐ~ωμεθᾰ";
    subj_mid_2: _, "ἐ~ωσθον", _;
    subj_mid_3: "ἐ~ωται", _, "ἐ~ωνται";
    subj_pass_1: "ἐ~ωμαι", _, _;
    subj_pass_2: _, _, "ἐ~ωσθε";
    opt_act_1: "ἐ~οιην|ἐ~οιμῐ", _, "ἐ~οιμεν|ἐ~οιημεν";
    opt_act_2: "ἐ~οιης|ἐ~οις", "ἐ~οιτον|ἐ~οιητον", "ἐ~οιτε|ἐ~οιητε";
    opt_act_3: "ἐ~οιη|ἐ~οι", "ἐ~οιτην|ἐ~οιητην", "ἐ~οιεν|ἐ~οιησᾰν";
    opt_mid_1: "ἐ~οιμην", _, "ἐ~οιμεθᾰ";
    opt_mid_2: "ἐ~οιο", "ἐ~οισθον", "ἐ~οισθε";
    opt_mid_3: "ἐ~οιτο", "ἐ~οισθην", "ἐ~οιντο";
    imp_act_2: "ἐ~ου", _, "ἐ~ουτε";
    imp_act_3: _, _, "ἐ~ουντων";
    imp_mid_2: "ἐ~ου", "ἐ~ουσθον", "ἐ~ουσθε";
    imp_mid_3: _, _, "ἐ~ουσθων";
    imp_pass_2: _, "ἐ~ουσθον", "ἐ~ουσθε";
    imp_pass_3: "ἐ~ουσθω", "ἐ~ουσθων", "ἐ~ουσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "ἠ~ωκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_pres: "ἐ~ουν", _, "ἐ~ουσθαι";
    infinitive_perf: _, _, derived;
    participle_pres: "ἐ~ων", _, "ἐ~ουμενος";
    ),
    // Ἐρωτᾰω: 4 words (ἐρωτᾰω, ἐρᾰω, ἐρευνᾰω, ἐμπολᾰω). Like Ληγω, except in 115 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐρωτᾰω", 4, "ἐ~ᾰω", like Ληγω;
    stem rest__: "~ησ- like ἐρωτᾰω";
    stem rest________: "~ησ- like ἀθῡρω";
    stem rest__________: "~ηθην- like ἀθῡρω";
    stem rest_: "~ησ- like ἐρωτᾰω";
    stem rest_____: "~ηθ- like ἐρωτᾰω";
    stem rest___: "~ηκ- like ἀθῡρω";
    stem rest______: "~η- like ἀθῡρω";
    stem rest_________: "~η- like ἀθῡρω";
    ind_act_1: "ἐ~ω", _, "ἐ~ωμεν";
    ind_act_2: "ἐ~ᾳς", "ἐ~ᾱτον", _;
    ind_act_3: "ἐ~ᾳ", _, "ἐ~ωσῐ|ἐ~ωσῐν";
    ind_mid_1: _, _, "ἐ~ωμεθᾰ";
    ind_mid_2: "ἐ~ᾳ", _, _;
    ind_mid_3: _, "ἐ~ᾱσθον", _;
    ind_pass_1: "ἐ~ωμαι", _, _;
    ind_pass_2: _, "ἐ~ᾱσθον", "ἐ~ᾱσθε";
    ind_pass_3: "ἐ~ᾱται", "ἐ~ᾱσθον", "ἐ~ωνται";
    subj_act_1: "ἐ~ω", _, "ἐ~ωμεν";
    subj_act_2: "ἐ~ᾳς", _, _;
    subj_act_3: "ἐ~ᾳ", "ἐ~ᾱτον", "ἐ~ωσῐ|ἐ~ωσῐν";
    subj_mid_1: _, _, "ἐ~ωμεθᾰ";
    subj_mid_2: _, "ἐ~ᾱσθον", _;
    subj_mid_3: "ἐ~ᾱται", _, "ἐ~ωνται";
    subj_pass_1: "ἐ~ωμαι", _, _;
    subj_pass_2: _, _, "ἐ~ᾱσθε";
    opt_act_1: "ἐ~ῳην|ἐ~ῳμῐ", _, "ἐ~ῳμεν|ἐ~ῳημεν";
    opt_act_2: "ἐ~ῳης|ἐ~ῳς", "ἐ~ῳτον|ἐ~ῳητον", "ἐ~ῳτε|ἐ~ῳητε";
    opt_act_3: "ἐ~ῳη|ἐ~ῳ", "ἐ~ῳτην|ἐ~ῳητην", "ἐ~ῳεν|ἐ~ῳησᾰν";
    opt_mid_1: "ἐ~ῳμην", _, "ἐ~ῳμεθᾰ";
    opt_mid_2: "ἐ~ῳο", "ἐ~ῳσθον", "ἐ~ῳσθε";
    opt_mid_3: "ἐ~ῳτο", "ἐ~ῳσθην", "ἐ~ῳντο";
    imp_act_2: "ἐ~ᾱ", _, "ἐ~ᾱτε";
    imp_act_3: _, _, "ἐ~ωντων";
    imp_mid_2: "ἐ~ω", "ἐ~ᾱσθον", "ἐ~ᾱσθε";
    imp_mid_3: _, _, "ἐ~ᾱσθων";
    imp_pass_2: _, "ἐ~ᾱσθον", "ἐ~ᾱσθε";
    imp_pass_3: "ἐ~ᾱσθω", "ἐ~ᾱσθων", "ἐ~ᾱσθων";
    imperfect_act_1: "ἠ~ων", _, _;
    imperfect_act_2: "ἠ~ᾱς", "ἠ~ᾱτον", "ἠ~ᾱτε";
    imperfect_act_3: "ἠ~ᾱ", _, "ἠ~ων";
    imperfect_mid_1: "ἠ~ωμην", _, "ἠ~ωμεθᾰ";
    imperfect_mid_2: "ἠ~ω", _, _;
    imperfect_mid_3: "ἠ~ᾱτο", _, "ἠ~ωντο";
    imperfect_pass_2: _, "ἠ~ᾱσθον", "ἠ~ᾱσθε";
    imperfect_pass_3: _, "ἠ~ᾱσθην", _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: "ἐ~ηθησομαι", _, _;
    future_ind_pass_3: _, "ἐ~ηθησεσθον", _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: "ἠ~ημαι", _, _;
    perfect_ind_mid_3: "ἠ~ηται", derived, _;
    perfect_ind_pass_1: _, _, "ἠ~ημεθᾰ";
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_subj_mid_3: _, "ἠ~ημένω ἦτον", _;
    perfect_subj_pass_1: "ἠ~ημένος ὦ", _, "ἠ~ημένοι ὦμεν";
    perfect_subj_pass_2: "ἠ~ημένος ᾖς", "ἠ~ημένω ἦτον", "ἠ~ημένοι ἦτε";
    perfect_subj_pass_3: "ἠ~ημένος ᾖ", "ἠ~ημένω ἦτον", "ἠ~ημένοι ὦσῐ";
    perfect_opt_pass_1: "ἠ~ημένος εἴην", _, "ἠ~ημένοι εἴημεν";
    perfect_opt_pass_2: "ἠ~ημένος εἴης", "ἠ~ημένω εἴητον", "ἠ~ημένοι εἴητε";
    perfect_opt_pass_3: "ἠ~ημένος εἴη", "ἠ~ημένω εἰήτην", "ἠ~ημένοι εἴησᾰν";
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "ἠ~ηκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_pres: "ἐ~ᾱν", _, "ἐ~ᾱσθαι";
    infinitive_perf: _, _, derived;
    participle_pres: "ἐ~ων", _, "ἐ~ωμενος";
    participle_perf: _, _, "ἠ~ημενος";
    ),
    // Πᾰρᾰγῐγνομαι: 4 words (ὑπογῐγνομαι, πᾰρᾰγῐγνομαι, περῐγῐγνομαι, ῠγγῐγνομαι). Like Προκᾰλεω, except in 118 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ind_pass_1 sg, ….
    conjugation!("πᾰρᾰγῐγνομαι", 4, "~~γῐγ~ομαι", like Προκᾰλεω;
    stem lemma: "~~γῐγ~- like οἰομαι";
    stem rest____: "~νε%ῐ~~- like εἰσερχομαι";
    stem rest__: "~~γε~ησ- like ἑπομαι";
    stem rest_______: "~~γε~ηθησ- like ἐχω";
    stem rest________: "~νε~ε~- like ὑπογῐγνομαι";
    stem rest__________: "~νε~ε~ηθην- like ἐλπω";
    stem rest_: "~~γε~- like ὑπογῐγνομαι";
    stem rest_____: "~~γε~ηθ- like ἐχω";
    stem rest___: "~~γεγο~- like πεμπω";
    stem rest: "~~γεγε~η- like ἐλπω";
    stem rest______: "~~γεγε~η- like εὐλογεω";
    stem rest_________: "~νε~εγε~η- like εὐλογεω";
    ind_act_1: _, _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: derived, _, derived;
    ind_pass_1: derived, _, derived;
    ind_pass_2: derived, _, _;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_3: _, derived, _;
    subj_pass_1: derived, _, derived;
    subj_pass_2: _, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imp_pass_2: derived, _, _;
    imp_pass_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: _, derived, _;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, _;
    future_opt_act_3: derived, derived, _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    perfect_ind_act_3: derived, derived, derived;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_act_3: _, _, derived;
    perfect_subj_mid_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "~νε~εγο~ειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    infinitive_perf: _, _, derived;
    participle_pres: derived, _, _;
    participle_aor: derived, _, _;
    ),
    // Κρουω: 4 words (ἀκουω, κρουω, κονῑω, ρουω). Like Πρεσβευω, except in 16 cells: future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, aorist_opt_pass_3 du, ….
    conjugation!("κρουω", 4, "~ω", like Πρεσβευω;
    stem rest: "~- like χρημᾰτῐζω";
    stem rest_________: "~σ- like κρουω";
    future_ind_pass_2: derived, _, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_pass_3: _, _, derived;
    pluperfect_act_1: "%ε~κειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    ),
    // Πλουτοκρᾰτεομαι: 4 words (πλουτοκρᾰτεομαι, δημοκρᾰτεομαι, εὐλαβεομαι, ἀμηχᾰνοποιεομαι). Like Λᾰγοθηρεω, except in 9 cells: imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_3 sg, imperfect_pass_3 pl, perfect_ind_act_3 du, ….
    conjugation!("πλουτοκρᾰτεομαι", 4, "~ᾰ~εομαι", like Λᾰγοθηρεω;
    stem rest____: "~ᾰ~- like ποιεω";
    stem rest________: "~ᾰ~ησ- like ἐχω";
    stem rest__________: "~ᾰ~ηθην- like ἐχω";
    stem rest___: "~ᾰ~ηκ- like ποιεω";
    stem rest: "~α~η- like χρημᾰτῐζω";
    stem rest___________: "~ᾰ~ηκ- like χρημᾰτῐζω";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_pass_3: _, _, derived;
    ),
    // Κυκλοω: 4 words (κυκλοω, γυμνοω, ἀπολυτροω, ἐκλυτροω). Like Ἀποκωλῡω, except in 41 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("κυκλοω", 4, "~υ~οω", like Ἀποκωλῡω;
    stem lemma: "~ῠ~- like γῡροω";
    stem rest____: "~ῠ~ου- like γῡροω";
    stem rest__: "~ῠ~ωσ- like ἐχω";
    stem rest_______: "~ῠ~ωθησ- like ἐχω";
    stem rest________: "~ῠ~ωσ- like ἐχω";
    stem rest__________: "~ῠ~ωθην- like ἐχω";
    stem rest_: "~ῠ~ωσ- like ἐχω";
    stem rest_____: "~ῠ~ηθ- like ἐχω";
    stem rest___: "~ε%ῠ~ωκ- like πεμπω";
    stem rest: "~ε%ῠ~ω- like ἐλπω";
    stem rest______: "~ε%ῠ~ω- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Ψᾱω: 4 words (δῐψᾱω, νᾱω, ψᾱω, σμᾰω). Like Νῑκαω, except in 61 cells: ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ψᾱω", 4, "~ᾱω", like Νῑκαω;
    stem rest__: "~ησ- like ἐπᾰνᾰχωρεω";
    stem rest: "~η- like σβεννῡμῐ";
    stem rest______: "~η- like σκοπεω";
    ind_act_2: "~ῃς", "~ητον", _;
    ind_act_3: "~ῃ", _, _;
    ind_mid_2: "~ῃ", _, _;
    ind_mid_3: _, "~ησθον", _;
    ind_pass_2: _, "~ησθον", "~ησθε";
    ind_pass_3: "~ηται", "~ησθον", _;
    subj_act_2: "~ῃς", _, _;
    subj_act_3: "~ῃ", "~ητον", _;
    subj_mid_2: _, "~ησθον", _;
    subj_mid_3: "~ηται", _, _;
    subj_pass_2: _, _, "~ησθε";
    imp_act_2: "~η", _, "~ητε";
    imp_mid_2: _, "~ησθον", "~ησθε";
    imp_mid_3: _, _, "~ησθων";
    imp_pass_2: _, "~ησθον", "~ησθε";
    imp_pass_3: "~ησθω", "~ησθων", "~ησθων";
    imperfect_act_2: "ἐ~ης", "ἐ~ητον", "ἐ~ητε";
    imperfect_act_3: "ἐ~η", _, _;
    imperfect_mid_3: "ἐ~ητο", _, _;
    imperfect_pass_2: _, "ἐ~ησθον", "ἐ~ησθε";
    imperfect_pass_3: _, "ἐ~ησθην", _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_act_3: _, derived, _;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_pres: "~ην", _, "~ησθαι";
    infinitive_perf: _, _, derived;
    ),
    // Πληττω: 4 words (φῠλᾰττω, πληττω, σᾰττω, ἀλλᾰττω). Like Ληγω, except in 33 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("πληττω", 4, "~ττω", like Ληγω;
    stem lemma: "~ττ- like βουλευω";
    stem rest____: "~ττ- like βουλευω";
    stem rest__: "~ξ- like ποιεω";
    stem rest_______: "~γησ- like ἐχω";
    stem rest________: "~ξ- like ποιεω";
    stem rest__________: "~γην- like ἐχω";
    stem rest_: "~ξ- like ποιεω";
    stem rest_____: "~γ- like ἐγειρω";
    stem rest___: "~γ- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_subj_mid_3: _, _, "%ε~γμένοι ὦσῐ|%ε~γμένοι ὦσῐν";
    perfect_subj_pass_3: _, _, derived;
    ),
    // Ἐπαιρω: 4 words (κᾰθαιρω, ἐπαιρω, δῐαιρω, ῥῠπαινω). Like Ἐξᾰττῐκῐζω, except in 50 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἐπαιρω", 4, "~αι~ω", like Ἐξᾰττῐκῐζω;
    stem lemma: "~αι~- like βουλευω";
    stem rest____: "~ῃ~- like ἐλπω";
    stem rest__: "~ᾰ~- like χρημᾰτῐζω";
    stem rest_______: "~ᾰ~θησ- like ποιεω";
    stem rest________: "~η~- like εὐλογεω";
    stem rest__________: "~η~θην- like ἐλπω";
    stem rest_: "~ᾰ~- like ποιεω";
    stem rest_____: "~ᾰ~θ- like ποιεω";
    stem rest: "~η~- like εὐλογεω";
    stem rest______: "~η~- like ἀειρω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Ἀπαιτεω: 4 words (ἐπαινεω, ἀπαιτεω, ἐξαιτεω, πᾰραινεω). Like Ἐπαιρω, except in 51 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἀπαιτεω", 4, "~αι~εω", like Ἐπαιρω;
    stem lemma: "~αι~- like ποιεω";
    stem rest____: "~ῃ~- like εὐλογεω";
    stem rest__: "~αι~ησ- like ἐπᾰνᾰχωρεω";
    stem rest_______: "~αι~ηθησ- like ἐχω";
    stem rest________: "~ῃ~ησ- like εὐλογεω";
    stem rest__________: "~ῃ~ηθην- like ἐλπω";
    stem rest_: "~αι~ησ- like ποιεω";
    stem rest_____: "~αι~ηθ- like ποιεω";
    stem rest___: "~ῃ~ηκ- like εὐλογεω";
    stem rest: "~ῃ~η- like εὐλογεω";
    stem rest______: "~ῃ~η- like εὐλογεω";
    stem rest_________: "~ῃ~η- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_2: _, _, derived;
    future_opt_act_3: _, _, derived;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "~ῃ~ηκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ἐξορκῐζω: 4 words (ἀφορῐζω, προορῐζω, ἐξορκῐζω, ἑορτᾰζω). Like Ἀποκωλῡω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἐξορκῐζω", 4, "~ο~ζω", like Ἀποκωλῡω;
    stem lemma: "~ο~ζ- like βουλευω";
    stem rest____: "~ω~ζ- like ἐλπω";
    stem rest__: "~ο~- like χρημᾰτῐζω";
    stem rest_______: "~ο~σθησ- like ποιεω";
    stem rest________: "~ω~σ- like εὐλογεω";
    stem rest__________: "~ω~σθην- like ἐλπω";
    stem rest_: "~ο~σ- like ποιεω";
    stem rest_____: "~ο~σθ- like ποιεω";
    stem rest___: "~ω~κ- like εὐλογεω";
    stem rest: "~ω~σ- like εὐλογεω";
    stem rest______: "~ω~- like ἱζω";
    stem rest___________: "~ω~κ- like εὐλογεω";
    stem rest_________: "~ω~- like ἱζω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ὑπᾰνᾰχωρεω: 4 words (ὑπᾰνᾰχωρεω, δῐασκοπεω, ἐπᾰνᾰχωρεω, κᾰτᾰπονεω). Like Νεκραγωγεω, except in 29 cells: imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_3 sg, imperfect_pass_3 pl, future_ind_act_3 du, ….
    conjugation!("ὑπᾰνᾰχωρεω", 4, "~ᾰ~εω", like Νεκραγωγεω;
    stem rest____: "~ε~- like εὐλογεω";
    stem rest__: "~ᾰ~ησ- like ἐπᾰνᾰχωρεω";
    stem rest________: "~ε~ησ- like εὐλογεω";
    stem rest__________: "~ε~ηθην- like ἐλπω";
    stem rest_: "~ᾰ~ησ- like ποιεω";
    stem rest___: "~ᾰ%ε~ηκ- like εὐλογεω";
    stem rest: "~ᾰ%ε~η- like ἐλπω";
    stem rest______: "~ᾰ%ε~η- like εὐλογεω";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Γρᾰφω: 3 words (γρᾰφω, ἐρεφω, γλῠφω). Like Θρῠπτω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("γρᾰφω", 3, "~φω", like Θρῠπτω;
    stem lemma: "~φ- like ἐχω";
    stem rest____: "~φ- like ἐχω";
    stem rest__: "~ψ- like ἐπᾰνᾰχωρεω";
    stem rest________: "~ψ- like ἐχω";
    stem rest__________: "~φην- like ἐχω";
    stem rest_: "~ψ- like ἐχω";
    stem rest_____: "~φ- like ἐγειρω";
    stem rest___: "~φ- like ἐχω";
    stem rest: "~- like πρεπω";
    stem rest___________: "~φειν- like ἐχω";
    stem rest_________: "~- like πρεπω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_2: derived, _, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_mid_3: _, _, derived;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    ),
    // Ἡλοω: 3 words (κᾰκοω, ἐξῐσοω, ἡλοω). Like Ἀξῐοω, except in 25 cells: imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_3 sg, imperfect_pass_3 pl, future_ind_act_3 du, ….
    conjugation!("ἡλοω", 3, "~οω", like Ἀξῐοω;
    stem rest____: "~ου- like κᾰκοω";
    stem rest__: "~ωσ- like ποιεω";
    stem rest________: "~ωσ- like εὐλογεω";
    stem rest__________: "~ωθην- like ἐλπω";
    stem rest___: "~ωκ- like εὐλογεω";
    stem rest: "~ω- like εὐλογεω";
    stem rest______: "~ω- like εὐλογεω";
    stem rest_________: "~ω- like εὐλογεω";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "~ωκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Πῑαινω: 3 words (μᾰραινω, πημαινω, πῑαινω). Like Περαινω, except in 35 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("πῑαινω", 3, "~αινω", like Περαινω;
    stem lemma: "~αιν- like βᾰπτῐζω";
    stem rest____: "~αιν- like ποιμαινω";
    stem rest__: "~ᾰν- like λογῐζομαι";
    stem rest________: "~ᾱν- like ἐχω";
    stem rest_: "~ᾱν- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_2: _, _, derived;
    future_opt_act_3: _, _, derived;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    ),
    // Θερᾰπευω: 3 words (ἱκετευω, ὀρουω, ὀπῑπευω). Like Βουλευω, except in 78 cells: perfect_ind_act_1 sg, perfect_ind_act_2 sg, perfect_ind_act_2 pl, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_ind_mid_1 pl, ….
    conjugation!("θερᾰπευω", 3, "~ω", like Βουλευω;
    stem rest_______: "~θησ- like ποιεω";
    stem rest_____: "~θ- like ποιεω";
    perfect_ind_act_1: "%ε~κᾰ", _, _;
    perfect_ind_act_2: "%ε~κᾰς", _, "%ε~κᾰτε";
    perfect_ind_act_3: "%ε~κε|%ε~κεν", _, "%ε~κᾱσῐ|%ε~κᾱσῐν";
    perfect_ind_mid_1: _, _, "%ε~μεθᾰ";
    perfect_ind_mid_2: _, _, "%ε~σθε";
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "%ε~κω", _, "%ε~κωμεν";
    perfect_subj_act_2: "%ε~κῃς", "%ε~κητον", "%ε~κητε";
    perfect_subj_act_3: "%ε~κῃ", derived, "%ε~κωσῐ|%ε~κωσῐν";
    perfect_subj_mid_1: "%ε~μένος ὦ", _, "%ε~μένοι ὦμεν";
    perfect_subj_mid_2: "%ε~μένος ᾖς", "%ε~μένω ἦτον", "%ε~μένοι ἦτε";
    perfect_subj_mid_3: "%ε~μένος ᾖ", derived, "%ε~μένοι ὦσῐ|%ε~μένοι ὦσῐν";
    perfect_subj_pass_1: derived, _, derived;
    perfect_subj_pass_2: derived, derived, derived;
    perfect_subj_pass_3: derived, derived, derived;
    perfect_opt_act_1: "%ε~κοιμῐ|%ε~κοιην", _, _;
    perfect_opt_act_2: "%ε~κοις|%ε~κοιης", _, _;
    perfect_opt_act_3: "%ε~κοι|%ε~κοιη", _, _;
    perfect_opt_mid_1: "%ε~μένος εἴην", _, "%ε~μένοι εἴημεν";
    perfect_opt_mid_2: "%ε~μένος εἴης", "%ε~μένω εἴητον", "%ε~μένοι εἴητε";
    perfect_opt_mid_3: "%ε~μένος εἴη", "%ε~μένω εἰήτην", "%ε~μένοι εἴησᾰν";
    perfect_opt_pass_1: derived, _, derived;
    perfect_opt_pass_2: derived, derived, derived;
    perfect_opt_pass_3: derived, derived, derived;
    perfect_imp_act_2: _, _, "%ε~κετε";
    perfect_imp_act_3: "%ε~κετω", "%ε~κετων", _;
    perfect_imp_mid_2: "%ε~σο", _, derived;
    perfect_imp_mid_3: "%ε~σθω", "%ε~σθων", derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: "%ε~κεναι", _, _;
    participle_perf: "%ε~κως", "%ε~μενος", derived;
    ),
    // Ἀνᾰζω: 3 words (ζω, ἀποζω, ἀνᾰζω). Like Περιειμι, except in 31 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀνᾰζω", 3, "~ω", like Περιειμι;
    stem rest__: "~ησ- like ἐχω";
    stem rest________: "~ησ- like ἐχω";
    stem rest_: "~ησ- like ἐχω";
    stem rest___: "~ηκ- like στηρῐζω";
    ind_act_1: "~ω", _, "~ωμεν";
    ind_act_2: "~ῃς", "~ητον", _;
    ind_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    opt_act_1: "~ῳην|~ῳμῐ", _, "~ῳμεν|~ῳημεν";
    opt_act_2: "~ῳης|~ῳς", "~ῳτον|~ῳητον", "~ῳτε|~ῳητε";
    opt_act_3: "~ῳη|~ῳ", "~ῳτην|~ῳητην", "~ῳεν|~ῳησᾰν";
    imp_act_2: "~η", _, "~ητε";
    imp_act_3: _, _, "~ωντων";
    imperfect_act_1: "ἐ~ων", _, "ἐ~ωμεν";
    imperfect_act_2: "ἐ~ης", "ἐ~ητον", "ἐ~ητε";
    imperfect_act_3: "ἐ~η", _, "ἐ~ων";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    infinitive_pres: "~ην", _, _;
    ),
    // Τεινω: 3 words (ἐκτεινω, τεινω, θεινω). Like Βαινω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("τεινω", 3, "~εινω", like Βαινω;
    stem lemma: "~ειν- like βᾰπτῐζω";
    stem rest____: "~ειν- like ἐχω";
    stem rest__: "~εν- like λογῐζομαι";
    stem rest_______: "~ᾰθησ- like ἐχω";
    stem rest________: "~ειν- like ἐχω";
    stem rest_: "~ειν- like ἐχω";
    stem rest___: "~ετᾰκ- like πεμπω";
    stem rest: "~ετᾰ- like ἐλπω";
    stem rest______: "~ετᾰ- like εὐλογεω";
    stem rest_________: "~ετᾰ- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "~ετᾰκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Πλᾰναω: 3 words (δαμναω, πλᾰναω, γαναω). Like Πῠρσαινω, except in 84 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πλᾰναω", 3, "~ναω", like Πῠρσαινω;
    stem rest____: "~ν- like φοιτᾰω";
    stem rest__: "~γησ- like ἑπομαι";
    stem rest________: "~νησ- like ἐχω";
    stem rest__________: "~νηθην- like ἐχω";
    stem rest_: "~νησ- like ἐχω";
    stem rest_____: "~νηθ- like ἐχω";
    stem rest___: "~νηκ- like ἐχω";
    stem rest: "~νη- like ἐχω";
    stem rest______: "~νη- like ἐχω";
    stem rest___________: "~νηκειν- like ἐχω";
    stem rest_________: "~νη- like ἐχω";
    ind_act_1: "~νω", _, "~νωμεν";
    ind_act_2: "~νᾳς", "~νᾱτον", _;
    ind_act_3: "~νᾳ", _, "~νωσῐ";
    ind_mid_1: _, _, "~νωμεθᾰ";
    ind_mid_2: "~νᾳ", _, _;
    ind_mid_3: _, "~νᾱσθον", _;
    ind_pass_1: "~νωμαι", _, _;
    ind_pass_2: _, "~νᾱσθον", "~νᾱσθε";
    ind_pass_3: "~νᾱται", "~νᾱσθον", "~νωνται";
    subj_act_1: "~νω", _, "~νωμεν";
    subj_act_2: "~νᾳς", _, _;
    subj_act_3: "~νᾳ", "~νᾱτον", "~νωσῐ";
    subj_mid_1: _, _, "~νωμεθᾰ";
    subj_mid_2: _, "~νᾱσθον", _;
    subj_mid_3: "~νᾱται", _, "~νωνται";
    subj_pass_1: "~νωμαι", _, _;
    subj_pass_2: _, _, "~νᾱσθε";
    opt_act_1: "~νῳην", _, "~νῳμεν";
    opt_act_2: "~νῳης", "~νῳτον", "~νῳτε";
    opt_act_3: "~νῳη", "~νῳτην", "~νῳεν";
    opt_mid_1: "~νῳμην", _, "~νῳμεθᾰ";
    opt_mid_2: "~νῳο", "~νῳσθον", "~νῳσθε";
    opt_mid_3: "~νῳτο", "~νῳσθην", "~νῳντο";
    imp_act_2: "~νᾱ", _, "~νᾱτε";
    imp_act_3: _, _, "~νωντων";
    imp_mid_2: "~νω", "~νᾱσθον", "~νᾱσθε";
    imp_mid_3: _, _, "~νᾱσθων";
    imp_pass_2: _, "~νᾱσθον", "~νᾱσθε";
    imp_pass_3: "~νᾱσθω", "~νᾱσθων", "~νᾱσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    future_perfect_ind_pass_1: derived, _, _;
    infinitive_pres: "~νᾱν", _, "~νᾱσθαι";
    infinitive_perf: _, _, derived;
    participle_pres: "~νων", _, "~νωμενος";
    ),
    // Ὀμνῠω: 3 words (ὀφελλω, ὀμνῠω, ὀρῠττω). Like Πῠρσαινω, except in 73 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ὀμνῠω", 3, "ὀ~ω", like Πῠρσαινω;
    ind_act_1: "ὀ~ω", _, "ὀ~ομεν";
    ind_act_2: "ὀ~εις", "ὀ~ετον", _;
    ind_act_3: "ὀ~ει", _, "ὀ~ουσῐ|ὀ~ουσῐν";
    ind_mid_1: _, _, "ὀ~ομεθᾰ";
    ind_mid_2: "ὀ~ῃ|ὀ~ει", _, _;
    ind_mid_3: _, "ὀ~εσθον", _;
    ind_pass_1: "ὀ~ομαι", _, _;
    ind_pass_2: _, "ὀ~εσθον", "ὀ~εσθε";
    ind_pass_3: "ὀ~εται", "ὀ~εσθον", "ὀ~ονται";
    subj_act_1: "ὀ~ω", _, "ὀ~ωμεν";
    subj_act_2: "ὀ~ῃς", _, _;
    subj_act_3: "ὀ~ῃ", "ὀ~ητον", "ὀ~ωσῐ|ὀ~ωσῐν";
    subj_mid_1: _, _, "ὀ~ωμεθᾰ";
    subj_mid_2: _, "ὀ~ησθον", _;
    subj_mid_3: "ὀ~ηται", _, "ὀ~ωνται";
    subj_pass_1: "ὀ~ωμαι", _, _;
    subj_pass_2: _, _, "ὀ~ησθε";
    opt_act_1: "ὀ~οιμῐ", _, "ὀ~οιμεν";
    opt_act_2: "ὀ~οις", "ὀ~οιτον", "ὀ~οιτε";
    opt_act_3: "ὀ~οι", "ὀ~οιτην", "ὀ~οιεν";
    opt_mid_1: "ὀ~οιμην", _, "ὀ~οιμεθᾰ";
    opt_mid_2: "ὀ~οιο", "ὀ~οισθον", "ὀ~οισθε";
    opt_mid_3: "ὀ~οιτο", "ὀ~οισθην", "ὀ~οιντο";
    imp_act_2: "ὀ~ε", _, "ὀ~ετε";
    imp_act_3: _, _, "ὀ~οντων";
    imp_mid_2: "ὀ~ου", "ὀ~εσθον", "ὀ~εσθε";
    imp_mid_3: _, _, "ὀ~εσθων";
    imp_pass_2: _, "ὀ~εσθον", "ὀ~εσθε";
    imp_pass_3: "ὀ~εσθω", "ὀ~εσθων", "ὀ~εσθων";
    imperfect_act_1: "ὠ~ον", _, _;
    imperfect_act_2: "ὠ~ες", "ὠ~ετον", "ὠ~ετε";
    imperfect_act_3: "ὠ~ε|ὠ~εν", _, "ὠ~ον";
    imperfect_mid_1: "ὠ~ομην", _, "ὠ~ομεθᾰ";
    imperfect_mid_2: "ὠ~ου", _, _;
    imperfect_mid_3: "ὠ~ετο", _, "ὠ~οντο";
    imperfect_pass_2: _, "ὠ~εσθον", "ὠ~εσθε";
    imperfect_pass_3: _, "ὠ~εσθην", _;
    infinitive_pres: "ὀ~ειν", _, "ὀ~εσθαι";
    participle_pres: "ὀ~ων", _, "ὀ~ομενος";
    ),
    // Προβαλλω: 3 words (προβαλλω, διαβαλλω, ἐμβαλλω). Like Ἐπᾰγγελλω, except in 63 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("προβαλλω", 3, "~~α~λω", like Ἐπᾰγγελλω;
    stem lemma: "~~α~λ- like βᾰπτῐζω";
    stem rest____: "~ε~α~λ- like ἀειρω";
    stem rest__: "~~α~- like λογῐζομαι";
    stem rest________: "~ε~α~- like ἀνᾰγω";
    stem rest_: "~~α~- like φᾰρμᾰσσω";
    stem rest___: "~~εβ~ηκ- like πεμπω";
    stem rest: "~~εβ~η- like ἐλπω";
    stem rest______: "~~εβ~η- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_2: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_2: _, _, derived;
    future_opt_act_3: _, _, derived;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_3: derived, derived, derived;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_act_3: _, _, derived;
    perfect_subj_mid_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Ἐπῐτῐθημῐ: 3 words (ἐπῐτῐθημῐ, συνεπῐτῐθημῐ, ξυνεπῐτῐθημῐ). Like Ἀποκωλῡω, except in 50 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἐπῐτῐθημῐ", 3, "~ῐτῐ~ημῐ", like Ἀποκωλῡω;
    stem lemma: "~ῐ%ῐ~- like ἐπῐτῐθημῐ";
    stem rest____: "~ε%ῐ~- like ἀφῑημῐ";
    stem rest__: "~ῐ~ησ- like ποιεω";
    stem rest_______: "~ῐ%ε~ησ- like ποιεω";
    stem rest________: "~ε~- like ἐπῐτῐθημῐ";
    stem rest_: "~ῐ~- like ἐπῐτῐθημῐ";
    stem rest___: "~ῐ%ε~εικ- like εὐλογεω";
    stem rest: "~ῐ%ε~ει- like εὐλογεω";
    stem rest______: "~ῐ%ε~ει- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_2: derived, _, _;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_imp_mid_3: _, _, derived;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Κῠβισταω: 3 words (μεριμναω, κῠβισταω, λιστραω). Like Ποιμαινω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("κῠβισταω", 3, "~ι~αω", like Ποιμαινω;
    stem lemma: "~ῐ~- like φοιτᾰω";
    stem rest____: "~ῐ~- like φοιτᾰω";
    stem rest__: "~ῐ~ησ- like ἐχω";
    stem rest________: "~ῐ~ησ- like ἐχω";
    stem rest_: "~ῐ~ησ- like ἐχω";
    stem rest___: "~ῐ~ηκ- like ἐχω";
    stem rest: "~ῐ~η- like βουλευω";
    stem rest______: "~ῐ~η- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Φλῡω: 3 words (πτῡω, μῡω, φλῡω). Like Λῡω, except in 33 cells: future_ind_act_3 du, future_ind_mid_3 du, future_ind_pass_2 sg, aorist_subj_act_3 du, aorist_subj_mid_3 du, aorist_subj_pass_3 pl, ….
    conjugation!("φλῡω", 3, "~ῡω", like Λῡω;
    stem rest__: "~ῠσ- like ποιεω";
    stem rest________: "~ῠσ- like ποιεω";
    stem rest_: "~ῠσ- like ποιεω";
    stem rest___: "~εμῡκ- like πεμπω";
    stem rest: "~εμῡ- like φλῡω";
    stem rest______: "~εμῡ- like εὐλογεω";
    stem rest_________: "~εμῠ- like σκοπεω";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_2: derived, _, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: derived, derived, derived;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_act_3: _, _, derived;
    perfect_subj_mid_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "ἐ~εμῠκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    future_perfect_ind_pass_1: derived, _, _;
    infinitive_perf: _, _, derived;
    ),
    // Ἐπευχομαι: 3 words (προσευχομαι, ἀπευχομαι, ἐπευχομαι). Like Μαχομαι, except in 43 cells: ind_pass_1 sg, ind_pass_1 pl, ind_pass_2 sg, subj_mid_3 du, subj_pass_1 sg, subj_pass_1 pl, ….
    conjugation!("ἐπευχομαι", 3, "~ε~χομαι", like Μαχομαι;
    stem lemma: "~ε~χ- like σῑνομαι";
    stem rest____: "~η~χ- like εἰσερχομαι";
    stem rest__: "~ε~ξ- like εἰσερχομαι";
    stem rest________: "~η~ξ- like μηκᾰομαι";
    stem rest_: "~ε~ξ- like ἑπομαι";
    stem rest: "~η~- like ἀνᾰγω";
    stem rest______: "~η~- like πᾰρᾰλλᾰσσω";
    stem rest_________: "~η~- like κᾰτειργω";
    ind_pass_1: derived, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: derived, _, derived;
    subj_pass_2: _, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_pass_2: derived, _, _;
    imp_pass_3: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Κᾰθελκω: 3 words (ἀφελκω, κᾰθελκω, ἐφελκω). Like Ἀποκωλῡω, except in 41 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("κᾰθελκω", 3, "~~κω", like Ἀποκωλῡω;
    stem lemma: "~~κ- like βουλευω";
    stem rest____: "~ι~κ- like ἐλπω";
    stem rest__: "~~ξ- like ἐπᾰνᾰχωρεω";
    stem rest_______: "~~κυσθησ- like ἐχω";
    stem rest________: "~ι~κυσ- like εὐλογεω";
    stem rest__________: "~ι~κυσθην- like ἐλπω";
    stem rest_: "~~κ- like ἐφελκω";
    stem rest_____: "~ι~κυσθ- like ἐχω";
    stem rest___: "~ι~κῠκ- like εὐλογεω";
    stem rest: "~ι~κῠ- like ἐλπω";
    stem rest______: "~ι~κῠ- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Θλῑβω: 3 words (τρῑβω, θλῑβω, φλῑβω). Like Βουλευω, except in 66 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("θλῑβω", 3, "~ῑβω", like Βουλευω;
    stem lemma: "~ῑβ- like βουλευω";
    stem rest____: "~ῑβ- like βουλευω";
    stem rest__: "~ιψ- like ποιεω";
    stem rest_______: "~ῐβησ- like ποιεω";
    stem rest________: "~ιψ- like ποιεω";
    stem rest__________: "~ιφθην- like ἐχω";
    stem rest_____: "~ιφθ- like ἐχω";
    stem rest___: "~ῐφ- like ποιεω";
    stem rest: "~ῑ- like πρεπω";
    stem rest______: "~ῑ- like θρῠπτω";
    stem rest___________: "~ῐφ- like χρημᾰτῐζω";
    stem rest_________: "~ῐ- like πρεπω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_1: "~ιψω", _, _;
    aorist_subj_act_2: "~ιψῃς", "~ιψητον", _;
    aorist_subj_act_3: _, _, "~ιψωσῐ|~ιψωσῐν";
    aorist_subj_mid_1: _, _, "~ιψωμεθᾰ";
    aorist_subj_mid_2: "~ιψῃ", "~ιψησθον", _;
    aorist_opt_act_1: "~ιψαιμῐ", _, "~ιψαιμεν";
    aorist_opt_act_2: "~ιψειᾰς|~ιψαις", _, _;
    aorist_opt_act_3: "~ιψειε|~ιψειεν|~ιψαι", _, "~ιψειᾰν|~ιψαιεν";
    aorist_opt_mid_1: "~ιψαιμην", _, _;
    aorist_imp_act_2: "~ῑψον", _, "~ιψᾰτε";
    aorist_imp_act_3: _, _, "~ιψᾰντων";
    aorist_imp_mid_2: "~ῑψαι", "~ιψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ιψᾰσθων";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_aor: "~ῑψαι", _, _;
    infinitive_perf: _, _, derived;
    participle_aor: "~ιψᾱς", "~ιψᾰμενος", _;
    ),
    // Κᾰτεχω: 3 words (μετεχω, κᾰτεχω, ἀντεχω). Like Πῠρσαινω, except in 42 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("κᾰτεχω", 3, "~τεχω", like Πῠρσαινω;
    stem lemma: "~τεχ- like βουλευω";
    stem rest____: "~τειχ- like ἐλπω";
    stem rest__: "~θεξ- like ποιεω";
    stem rest________: "~τεσχ- like ἀνᾰλᾰμβᾰνω";
    stem rest_: "~τᾰσχ- like φᾰρμᾰσσω";
    stem rest___: "~τεσχηκ- like εὐλογεω";
    stem rest: "~τεσχη- like εὐλογεω";
    stem rest______: "~τεσχη- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Σῠνιστημῐ: 3 words (σῠνιστημῐ, ξῠνιστημῐ, ὀνινημῐ). Like Περῐτρεχω, except in 32 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_act_2 du, subj_mid_2 sg, subj_mid_3 du, ….
    conjugation!("σῠνιστημῐ", 3, "~νι~ημῐ", like Περῐτρεχω;
    stem lemma: "~νι~- like σῠνιστημῐ";
    stem rest____: "~νῑ~- like ἀνῐστημῐ";
    stem rest__: "~~ησ- like ποιεω";
    stem rest________: "~νε~- like σῠνιστημῐ";
    stem rest__________: "~νε~ᾰθην- like ἐλπω";
    stem rest_: "~~- like σῠνιστημῐ";
    stem rest_____: "~~ᾰθ- like ἐγειρω";
    stem rest___: "~νε~ηκ- like πεμπω";
    stem rest___________: "~νει~ηκ- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_act_2: _, derived, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    ),
    // Ἰαινω: 3 words (ἰαινω, σαιρω, ἐναιρω). Like Ἐπαιρω, except in 87 cells: imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_3 sg, imperfect_pass_3 pl, future_ind_act_1 sg, ….
    conjugation!("ἰαινω", 3, "~αι~ω", like Ἐπαιρω;
    stem rest____: "~αι~- like ἀειρω";
    stem rest________: "~η~- like ἐλπω";
    stem rest__________: "~α~θην- like ἐλπω";
    stem rest_: "~η~- like ποιεω";
    stem rest_____: "~α~θ- like ἐχω";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: _, derived, _;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_2: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    future_opt_mid_1: derived, _, _;
    future_opt_mid_3: derived, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_mid_1: derived, _, _;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_3: _, derived, derived;
    perfect_subj_mid_3: _, derived, derived;
    perfect_subj_pass_1: derived, _, derived;
    perfect_subj_pass_2: derived, derived, derived;
    perfect_subj_pass_3: derived, derived, _;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_opt_pass_1: derived, _, derived;
    perfect_opt_pass_2: derived, derived, derived;
    perfect_opt_pass_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    infinitive_fut: derived, _, _;
    infinitive_perf: _, derived, _;
    participle_perf: _, _, derived;
    ),
    // Ἀφῐστημῐ: 3 words (ἐφῐστημῐ, ἀφῐστημῐ, ὑφῐστημῐ). Like Ἰαινω, except in 102 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀφῐστημῐ", 3, "~φῐ~ημῐ", like Ἰαινω;
    stem rest____: "~φῑ~- like ἀνῐστημῐ";
    stem rest__: "~πο~ησ- like ποιεω";
    stem rest_______: "~πο~ησ- like ἐχω";
    stem rest________: "~πε~- like ἐφῐστημῐ";
    stem rest__________: "~πε~ᾰθην- like ἐλπω";
    stem rest_: "~πο~- like ἐφῐστημῐ";
    stem rest_____: "~πο~ᾰθ- like ποιεω";
    stem rest___: "~φε~- like ἐφῐστημῐ";
    stem rest: "~φε~- like ἐφῐστημῐ";
    stem rest______: "~φε~ᾰ- like εὐλογεω";
    ind_act_1: "~φῐ~ημῐ", _, "~φῐ~ᾰμεν";
    ind_act_2: "~φῐ~ης", "~φῐ~ᾰτον", _;
    ind_act_3: "~φῐ~ησῐ|~φῐ~ησῐν", _, "~φῐ~ᾱσῐ|~φῐ~ᾱσῐν";
    ind_mid_1: _, _, "~φῐ~ᾰμεθᾰ";
    ind_mid_2: "~φῐ~ᾰσαι", _, _;
    ind_mid_3: _, "~φῐ~ᾰσθον", _;
    ind_pass_1: "~φῐ~ᾰμαι", _, _;
    ind_pass_2: _, "~φῐ~ᾰσθον", "~φῐ~ᾰσθε";
    ind_pass_3: "~φῐ~ᾰται", "~φῐ~ᾰσθον", "~φῐ~ᾰνται";
    subj_act_1: "~φῐ~ω", _, "~φῐ~ωμεν";
    subj_act_2: "~φῐ~ῃς", _, _;
    subj_act_3: "~φῐ~ῃ", "~φῐ~ητον", "~φῐ~ωσῐ|~φῐ~ωσῐν";
    subj_mid_1: _, _, "~φῐ~ωμεθᾰ";
    subj_mid_2: _, "~φῐ~ησθον", _;
    subj_mid_3: "~φῐ~ηται", _, "~φῐ~ωνται";
    subj_pass_1: "~φῐ~ωμαι", _, _;
    subj_pass_2: _, _, "~φῐ~ησθε";
    opt_act_1: "~φῐ~αιην", _, "~φῐ~αιμεν|~φῐ~αιημεν";
    opt_act_2: "~φῐ~αιης", "~φῐ~αιτον|~φῐ~αιητον", "~φῐ~αιτε|~φῐ~αιητε";
    opt_act_3: "~φῐ~αιη", "~φῐ~αιτην|~φῐ~αιητην", "~φῐ~αιεν|~φῐ~αιησᾰν";
    opt_mid_1: "~φῐ~αιμην", _, "~φῐ~αιμεθᾰ";
    opt_mid_2: "~φῐ~αιο", "~φῐ~αισθον", "~φῐ~αισθε";
    opt_mid_3: "~φῐ~αιτο", "~φῐ~αισθην", "~φῐ~αιντο";
    imp_act_2: "~φῐ~η", _, "~φῐ~ᾰτε";
    imp_act_3: _, _, "~φῐ~ᾰντων";
    imp_mid_2: "~φῐ~ᾰσο|~φῐ~ω", "~φῐ~ᾰσθον", "~φῐ~ᾰσθε";
    imp_mid_3: _, _, "~φῐ~ᾰσθων";
    imp_pass_2: _, "~φῐ~ᾰσθον", "~φῐ~ᾰσθε";
    imp_pass_3: "~φῐ~ᾰσθω", "~φῐ~ᾰσθων", "~φῐ~ᾰσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_2: derived, _, _;
    aorist_subj_mid_3: _, derived, _;
    aorist_imp_mid_3: _, _, derived;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "~φε~ηκειν", _, _;
    future_perfect_ind_act_1: "~φε~ηξω", _, "~φε~ηξομεν";
    future_perfect_ind_act_2: "~φε~ηξεις", "~φε~ηξετον", "~φε~ηξετε";
    future_perfect_ind_act_3: "~φε~ηξει", _, "~φε~ηξουσῐ|~φε~ηξουσῐν";
    future_perfect_ind_pass_1: derived, _, _;
    future_perfect_ind_pass_2: derived, _, _;
    future_perfect_opt_act_1: "~φε~ηξοιμῐ", _, "~φε~ηξοιμεν";
    future_perfect_opt_act_2: "~φε~ηξοις", "~φε~ηξοιτον", "~φε~ηξοιτε";
    future_perfect_opt_act_3: "~φε~ηξοι", "~φε~ηξοιτην", "~φε~ηξοιεν";
    infinitive_pres: "~φῐ~ᾰναι", _, "~φῐ~ᾰσθαι";
    infinitive_perf: _, _, derived;
    infinitive_futp: "~φε~ηξειν", _, _;
    participle_pres: "~φῐ~ᾱς", _, "~φῐ~ᾰμενος";
    participle_futp: "~φε~ηξων", _, _;
    ),
    // Δρᾰσσομαι: 3 words (πλῐσσομαι, δρᾰσσομαι, μῠσσομαι). Like Λλῡμῐ, except in 64 cells: ind_act_1 sg, ind_act_3 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ….
    conjugation!("δρᾰσσομαι", 3, "~σσομαι", like Λλῡμῐ;
    stem rest__: "~ξ- like εἰσερχομαι";
    stem rest________: "~ξ- like ἑπομαι";
    stem rest_: "~ξ- like ἑπομαι";
    stem rest______: "~- like φᾰρμᾰσσω";
    ind_act_1: "~σσομαι", _, _;
    ind_act_3: derived, _, _;
    ind_mid_1: "~σσομαι", _, "~σσομεθᾰ";
    ind_mid_2: "~σσῃ|~σσει", _, _;
    ind_mid_3: _, "~σσεσθον", _;
    subj_mid_1: "~σσωμαι", _, "~σσωμεθᾰ";
    subj_mid_2: "~σσῃ", "~σσησθον", _;
    subj_mid_3: "~σσηται", _, "~σσωνται";
    opt_mid_1: "~σσοιμην", _, "~σσοιμεθᾰ";
    opt_mid_2: "~σσοιο", "~σσοισθον", "~σσοισθε";
    opt_mid_3: "~σσοιτο", "~σσοισθην", "~σσοιντο";
    imp_mid_2: "~σσου", "~σσεσθον", "~σσεσθε";
    imp_mid_3: "~σσεσθω", _, "~σσεσθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    imperfect_mid_1: "ἐ~σσομην", _, "ἐ~σσομεθᾰ";
    imperfect_mid_2: "ἐ~σσου", "ἐ~σσεσθον", _;
    imperfect_mid_3: "ἐ~σσετο", _, "ἐ~σσοντο";
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_1: "%ε~γμαι", _, _;
    perfect_ind_mid_3: "%ε~κται", derived, _;
    perfect_ind_pass_1: _, _, "%ε~γμεθᾰ";
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_subj_mid_3: _, "%ε~γμένω ἦτον", "%ε~γμένοι ὦσῐ|%ε~γμένοι ὦσῐν";
    perfect_subj_pass_1: "%ε~γμένος ὦ", _, "%ε~γμένοι ὦμεν";
    perfect_subj_pass_2: "%ε~γμένος ᾖς", "%ε~γμένω ἦτον", "%ε~γμένοι ἦτε";
    perfect_subj_pass_3: "%ε~γμένος ᾖ", "%ε~γμένω ἦτον", _;
    perfect_opt_pass_1: "%ε~γμένος εἴην", _, "%ε~γμένοι εἴημεν";
    perfect_opt_pass_2: "%ε~γμένος εἴης", "%ε~γμένω εἴητον", "%ε~γμένοι εἴητε";
    perfect_opt_pass_3: "%ε~γμένος εἴη", "%ε~γμένω εἰήτην", "%ε~γμένοι εἴησᾰν";
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    participle_perf: _, _, "%ε~γμενος";
    ),
    // Πᾰρᾰκᾰλεω: 3 words (πᾰρᾰκᾰλεω, ἀνᾰκᾰλεω, κᾰτᾰκᾰλεω). Like Ἀπαιτεω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("πᾰρᾰκᾰλεω", 3, "~ᾰκᾰ~εω", like Ἀπαιτεω;
    stem lemma: "~ᾰκᾰ~- like ποιεω";
    stem rest____: "~εκᾰ~- like εὐλογεω";
    stem rest__: "~ᾰκᾰ~- like χρημᾰτῐζω";
    stem rest_______: "~ᾰκεκ~ησ- like ἐχω";
    stem rest________: "~εκᾰ~εσ- like εὐλογεω";
    stem rest__________: "~εκ~ηθην- like ἐλπω";
    stem rest_: "~ᾰκᾰ~εσ- like ποιεω";
    stem rest_____: "~ᾰκ~ηθ- like ποιεω";
    stem rest___: "~ᾰκεκ~ηκ- like εὐλογεω";
    stem rest: "~ᾰκεκ~η- like εὐλογεω";
    stem rest______: "~ᾰκεκ~η- like εὐλογεω";
    stem rest___________: "~εκεκ~ηκ- like εὐλογεω";
    stem rest_________: "~εκεκ~η- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ἐξαιρω: 3 words (κᾰταιρω, ἀνταιρω, ἐξαιρω). Like Ἐπαιρω, except in 22 cells: aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, aorist_subj_mid_1 pl, aorist_subj_mid_2 sg, ….
    conjugation!("ἐξαιρω", 3, "~αι~ω", like Ἐπαιρω;
    aorist_subj_act_1: "~α~ω", _, _;
    aorist_subj_act_2: "~α~ῃς", "~α~ητον", _;
    aorist_subj_act_3: _, _, "~α~ωσῐ|~α~ωσῐν";
    aorist_subj_mid_1: _, _, "~α~ωμεθᾰ";
    aorist_subj_mid_2: "~α~ῃ", "~α~ησθον", _;
    aorist_opt_act_1: "~α~αιμῐ", _, "~α~αιμεν";
    aorist_opt_act_2: "~α~ειᾰς|~α~αις", _, _;
    aorist_opt_act_3: "~α~ειε|~α~ειεν|~α~αι", _, "~α~ειᾰν|~α~αιεν";
    aorist_opt_mid_1: "~α~αιμην", _, _;
    aorist_imp_act_2: "~α~ον", _, "~α~ᾰτε";
    aorist_imp_act_3: _, _, "~α~ᾰντων";
    aorist_imp_mid_2: "~α~αι", "~α~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~α~ᾰσθων";
    infinitive_aor: "~ᾱ~αι", _, _;
    participle_aor: "~α~ᾱς", "~α~ᾰμενος", _;
    ),
    // Ἀντᾰνᾰλῡω: 3 words (ἀντᾰνᾰλῡω, δῐᾰλῡω, κᾰτᾰλῡω). Like Δῐοικεω, except in 49 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἀντᾰνᾰλῡω", 3, "~ᾰ~ῡω", like Δῐοικεω;
    stem lemma: "~ᾰ~ῡ- like βουλευω";
    stem rest____: "~ε~ῡ- like ἐλπω";
    stem rest__: "~ᾰ~ῡσ- like ποιεω";
    stem rest_______: "~ᾰ~ῠθησ- like ποιεω";
    stem rest________: "~ε~ῡσ- like εὐλογεω";
    stem rest__________: "~ε~ῠθην- like ἐλπω";
    stem rest_: "~ᾰ~ῡσ- like ποιεω";
    stem rest_____: "~ᾰ~ῠθ- like ποιεω";
    stem rest___: "~ᾰ~ελῠκ- like εὐλογεω";
    stem rest: "~ᾰ~ελ- like ἀντᾰνᾰλῡω";
    stem rest______: "~ᾰ~ελῠ- like εὐλογεω";
    stem rest___________: "~ε~ελῠκ- like εὐλογεω";
    stem rest_________: "~ε~ελῠ- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    future_perfect_ind_pass_1: derived, _, _;
    future_perfect_ind_pass_2: derived, _, _;
    infinitive_perf: _, _, derived;
    ),
    // Ἁμᾰρτᾰνω: 3 words (ἁμᾰρτᾰνω, ἀφᾰμᾰρτᾰνω, ἀνᾰβλᾰστᾰνω). Like Ἰαινω, except in 30 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἁμᾰρτᾰνω", 3, "~ᾰ~ᾰνω", like Ἰαινω;
    stem lemma: "~ᾰ~ᾰν- like ἐχω";
    stem rest____: "~η~ᾰν- like ἀειρω";
    stem rest__: "~ᾰ~ησ- like ἐπᾰνᾰχωρεω";
    stem rest_______: "~ᾰ~ηθησ- like ἐχω";
    stem rest________: "~η~- like ἀνᾰγω";
    stem rest__________: "~η~ηθην- like ἐλπω";
    stem rest_: "~ᾰ~- like ἁμᾰρτᾰνω";
    stem rest_____: "~ᾰ~ηθ- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    ),
    // Κολᾰπτω: 3 words (κλεπτω, κᾰλῠπτω, κολᾰπτω). Like Θρῠπτω, except in 15 cells: perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_mid_3 pl, perfect_subj_act_3 du, perfect_opt_act_1 sg, perfect_opt_act_1 pl, ….
    conjugation!("κολᾰπτω", 3, "~πτω", like Θρῠπτω;
    stem rest_______: "~φθησ- like ποιεω";
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_mid_3: _, _, "%ε~πᾰται";
    perfect_subj_act_3: _, derived, _;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    ),
    // Ἀντῐκᾰθῐστημῐ: 3 words (ἐγκᾰθῐστημῐ, μεθῐστημῐ, ἀντῐκᾰθῐστημῐ). Like Ἰαινω, except in 32 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἀντῐκᾰθῐστημῐ", 3, "~θῐ~ημῐ", like Ἰαινω;
    stem lemma: "~θῐ~- like σῠνιστημῐ";
    stem rest____: "~θῑ~- like ἀνῐστημῐ";
    stem rest__: "~τᾰ~ησ- like ποιεω";
    stem rest_______: "~τᾰ~ησ- like ποιεω";
    stem rest________: "~τε~- like ἐφῐστημῐ";
    stem rest__________: "~τε~ᾰθην- like ἐλπω";
    stem rest_: "~τᾰ~ησ- like ἐχω";
    stem rest_____: "~τᾰ~ᾰθ- like ἐχω";
    stem rest___: "~θε~ηκ- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    ),
    // Ἐκγᾰμῐζω: 3 words (ἐκφερω, ἐκγᾰμῐζω, ὑπεκφευγω). Like Ἰαινω, except in 63 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἐκγᾰμῐζω", 3, "~κ~ω", like Ἰαινω;
    stem lemma: "~κ~- like βᾰπτῐζω";
    stem rest____: "~ξε~- like ἀειρω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_ind_mid_1: derived, _, _;
    aorist_ind_mid_2: derived, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: _, _, derived;
    aorist_subj_mid_2: derived, derived, _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_opt_mid_1: derived, _, _;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_mid_2: derived, derived, _;
    aorist_imp_mid_3: _, _, derived;
    aorist_imp_pass_2: derived, derived, derived;
    infinitive_aor: derived, _, _;
    participle_aor: derived, derived, _;
    ),
    // Ἀντελπῐζω: 3 words (προετοιμᾰζω, ἐξετᾰζω, ἀντελπῐζω). Like Ἐξᾰττῐκῐζω, except in 25 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἀντελπῐζω", 3, "~ε~ζω", like Ἐξᾰττῐκῐζω;
    stem lemma: "~ε~ζ- like βουλευω";
    stem rest__: "~ε~- like λογῐζομαι";
    stem rest_______: "~ε~σθησ- like ποιεω";
    stem rest_: "~ε~σ- like ποιεω";
    stem rest_____: "~ε~σθ- like ποιεω";
    stem rest___________: "~η~κ- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    ),
    // Σῠναιρεω: 3 words (κᾰθαιρεω, σῠναιρεω, ξῠναιρεω). Like Ἀναιρεω, except in 30 cells: future_ind_act_3 du, future_ind_mid_3 du, future_ind_pass_1 sg, future_ind_pass_2 sg, future_ind_pass_3 du, aorist_ind_act_2 pl, ….
    conjugation!("σῠναιρεω", 3, "~αιρεω", like Ἀναιρεω;
    stem rest__: "~ελ- like χρημᾰτῐζω";
    stem rest________: "~ειλ- like ἀνᾰλᾰμβᾰνω";
    stem rest_: "~ελ- like ἀνᾰγω";
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_2: derived, _, _;
    future_ind_pass_3: _, derived, _;
    aorist_ind_act_2: _, _, derived;
    aorist_ind_act_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: _, _, derived;
    aorist_subj_act_2: _, _, derived;
    aorist_subj_act_3: derived, derived, _;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, derived;
    aorist_opt_act_2: _, derived, derived;
    aorist_opt_act_3: _, derived, _;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: _, derived, _;
    aorist_imp_act_3: derived, derived, _;
    aorist_imp_pass_2: derived, derived, derived;
    ),
    // Φῡρω: 3 words (σῡρω, τῡφω, φῡρω). Like Ποιμαινω, except in 40 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("φῡρω", 3, "~ῡ~ω", like Ποιμαινω;
    stem lemma: "~ῡ~- like βουλευω";
    stem rest____: "~ῡ~- like βουλευω";
    stem rest__: "~ῠ~σ- like ἐχω";
    stem rest________: "~ῠ~σ- like ἐχω";
    stem rest__________: "~ῠ~θην- like ἐχω";
    stem rest_: "~ῠ~σ- like ἐχω";
    stem rest_____: "~ῠ~θ- like ἐχω";
    stem rest: "~ῠ~- like βουλευω";
    stem rest______: "~ῠ~- like πορσῡνω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Σταθμαω: 3 words (χαλαω, καθοραω, σταθμαω). Like Ὑπᾰνᾰχωρεω, except in 54 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("σταθμαω", 3, "~α~αω", like Ὑπᾰνᾰχωρεω;
    stem lemma: "~ᾰ~- like φοιτᾰω";
    stem rest__: "~ᾰ~ησ- like ἐχω";
    stem rest_______: "~ᾰ~ησθησ- like ἐχω";
    stem rest________: "~ᾰ~ησ- like ἐχω";
    stem rest__________: "~ᾰ~ησθην- like ἐχω";
    stem rest_: "~ᾰ~ησ- like ἐχω";
    stem rest_____: "~ᾰ~ησθ- like ἐχω";
    stem rest___: "~ᾰ~ηκ- like στηρῐζω";
    stem rest: "~ᾰ~η- like σβεννῡμῐ";
    stem rest______: "~ᾰ~η- like σκοπεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, _, _;
    imperfect_mid_3: derived, _, derived;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: derived, derived, derived;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_act_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Κνᾰω: 3 words (βοᾰω, τρῠγᾰω, κνᾰω). Like Χραω, except in 81 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("κνᾰω", 3, "~ᾰω", like Χραω;
    stem rest__: "~ησ- like ποιεω";
    stem rest_______: "~ησθησ- like ποιεω";
    stem rest_____: "~ησθ- like ποιεω";
    stem rest___: "~ηκ- like ποιεω";
    stem rest: "~η- like χρημᾰτῐζω";
    stem rest___________: "~ηκ- like χρημᾰτῐζω";
    ind_act_1: "~ᾰω|~ω", _, "~ᾰομεν|~ωμεν";
    ind_act_2: "~ᾰεις|~ᾳς", "~ᾰετον|~ᾱτον", "~ᾰετε|~ᾱτε";
    ind_act_3: "~ᾰει|~ᾳ", _, "~ᾰουσῐ|~ᾰουσῐν|~ωσῐ|~ωσῐν";
    ind_mid_1: "~ᾰομαι|~ωμαι", _, "~ᾰομεθᾰ|~ωμεθᾰ";
    ind_mid_2: "~ᾰῃ|~ᾰει|~ᾳ", "~ᾰεσθον|~ᾱσθον", "~ᾰεσθε|~ᾱσθε";
    ind_mid_3: "~ᾰεται|~ᾱται", derived, "~ᾰονται|~ωνται";
    ind_pass_1: derived, _, _;
    ind_pass_2: _, derived, derived;
    ind_pass_3: derived, derived, derived;
    subj_act_1: "~ᾰω|~ω", _, "~ᾰωμεν|~ωμεν";
    subj_act_2: "~ᾰῃς|~ᾳς", "~ᾰητον|~ᾱτον", "~ᾰητε|~ᾱτε";
    subj_act_3: "~ᾰῃ|~ᾳ", derived, "~ᾰωσῐ|~ᾰωσῐν|~ωσῐ|~ωσῐν";
    subj_mid_1: "~ᾰωμαι|~ωμαι", _, "~ᾰωμεθᾰ|~ωμεθᾰ";
    subj_mid_2: _, "~ᾰησθον|~ᾱσθον", "~ᾰησθε|~ᾱσθε";
    subj_mid_3: "~ᾰηται|~ᾱται", _, "~ᾰωνται|~ωνται";
    subj_pass_1: derived, _, _;
    subj_pass_2: _, _, derived;
    opt_act_1: "~ᾰοιμῐ|~ῳην|~ῳμῐ", _, "~ᾰοιμεν|~ῳμεν|~ῳημεν";
    opt_act_2: "~ᾰοις|~ῳης|~ῳς", "~ᾰοιτον|~ῳτον|~ῳητον", "~ᾰοιτε|~ῳτε|~ῳητε";
    opt_act_3: "~ᾰοι|~ῳη|~ῳ", "~ᾰοιτην|~ῳτην|~ῳητην", "~ᾰοιεν|~ῳεν|~ῳησᾰν";
    opt_mid_1: "~ᾰοιμην|~ῳμην", _, "~ᾰοιμεθᾰ|~ῳμεθᾰ";
    opt_mid_2: "~ᾰοιο|~ῳο", "~ᾰοισθον|~ῳσθον", "~ᾰοισθε|~ῳσθε";
    opt_mid_3: "~ᾰοιτο|~ῳτο", "~ᾰοισθην|~ῳσθην", "~ᾰοιντο|~ῳντο";
    imp_act_2: "~ᾰε|~ᾱ", _, derived;
    imp_act_3: "~ᾰετω|~ᾱτω", "~ᾰετων|~ᾱτων", "~ᾰοντων|~ωντων";
    imp_mid_2: "~ᾰου|~ω", derived, derived;
    imp_mid_3: "~ᾰεσθω|~ᾱσθω", "~ᾰεσθων|~ᾱσθων", derived;
    imp_pass_2: _, derived, derived;
    imp_pass_3: derived, derived, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_pass_3: _, _, derived;
    infinitive_pres: "~ᾰειν|~ᾱν", "~ᾰεσθαι|~ᾱσθαι", derived;
    participle_pres: "~ᾰων|~ων", "~ᾰομενος|~ωμενος", derived;
    ),
    // Δηλεομαι: 3 words (δηλεομαι, μῑμεομαι, δῐηγεομαι). Like Βουλομαι, except in 36 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 sg, ….
    conjugation!("δηλεομαι", 3, "~εομαι", like Βουλομαι;
    stem rest____: "~- like δηλεομαι";
    stem rest__: "~ησ- like εἰσερχομαι";
    stem rest_______: "~ηθησ- like ἐχω";
    stem rest________: "~ησ- like ἑπομαι";
    stem rest_: "~ησ- like ἑπομαι";
    stem rest: "~η- like σειω";
    stem rest_________: "~η- like ἐχω";
    ind_act_1: "~εομαι", _, _;
    ind_mid_1: "~ουμαι", _, "~ουμεθᾰ";
    ind_mid_2: "~ει|~ῃ", _, _;
    ind_mid_3: _, "~εισθον", _;
    ind_pass_2: "~ει", "~εισθον", "~εισθε";
    ind_pass_3: "~ειται", "~εισθον", "~ουνται";
    imp_mid_2: _, "~εισθον", "~εισθε";
    imp_mid_3: "~εισθω", _, "~εισθων";
    imp_pass_2: _, "~εισθον", "~εισθε";
    imp_pass_3: _, "~εισθων", "~εισθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_pres: _, _, "~εισθαι";
    participle_pres: _, _, "~ουμενος";
    ),
    // Τρωγλοδῠτεω: 3 words (σῠνδεω, σῠμμᾰχεω, τρωγλοδῠτεω). Like Πεμπω, except in 46 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("τρωγλοδῠτεω", 3, "~ῠ~εω", like Πεμπω;
    stem lemma: "~υ~- like τελεω";
    stem rest____: "~υ~- like ποιεω";
    stem rest__: "~υ~ησ- like ἐχω";
    stem rest_______: "~υ~ηθησ- like ἐχω";
    stem rest________: "~υ~ησ- like ἐχω";
    stem rest__________: "~υ~ηθην- like ἐχω";
    stem rest_: "~υ~ησ- like ἐχω";
    stem rest_____: "~υ~ηθ- like ἐχω";
    stem rest___: "~υ~ηκ- like ἐχω";
    stem rest: "~υ~η- like βουλευω";
    stem rest______: "~υ~η- like ἐχω";
    stem rest___________: "~υ~ηκειν- like ἐχω";
    stem rest_________: "~υ~η- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Κηδευω: 3 words (σκῡλευω, κηδευω, τυμβευω). Like Κᾰθιδρῡω, except in 11 cells: imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_3 sg, imperfect_pass_3 pl, pluperfect_act_1 sg, ….
    conjugation!("κηδευω", 3, "~ω", like Κᾰθιδρῡω;
    stem rest____: "~- like βουλευω";
    stem rest_______: "~θησ- like ποιεω";
    stem rest________: "~σ- like ποιεω";
    stem rest__________: "~θην- like ἐχω";
    stem rest_____: "~θ- like ποιεω";
    stem rest_________: "~- like σκοπεω";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    pluperfect_act_1: "ἐ~κειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    ),
    // Λεγομαι: 3 words (φθεγγομαι, ἐρευγομαι, λεγομαι). Like Μᾰρνᾰμαι, except in 40 cells: ind_pass_1 sg, ind_pass_1 pl, ind_pass_2 sg, subj_mid_3 du, subj_pass_1 sg, subj_pass_1 pl, ….
    conjugation!("λεγομαι", 3, "~γομαι", like Μᾰρνᾰμαι;
    stem lemma: "~γ- like σῑνομαι";
    stem rest____: "~γ- like σῑνομαι";
    stem rest__: "~ξ- like ἑπομαι";
    stem rest_______: "~χθησ- like ἐχω";
    stem rest________: "~ξ- like ἑπομαι";
    stem rest__________: "~γην- like ἐχω";
    stem rest_____: "~γ- like ἐγειρω";
    ind_pass_1: derived, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: derived, _, derived;
    subj_pass_2: _, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_pass_2: derived, _, _;
    imp_pass_3: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    perfect_ind_mid_1: "ἐι~γμαι", _, _;
    perfect_ind_mid_2: "ἐι~ξαι", "ἐι~χθον", _;
    perfect_ind_mid_3: "ἐι~κται", _, "ἐι~γᾰται";
    perfect_subj_mid_3: _, "ἐι~γμένω ἦτον", _;
    perfect_imp_mid_2: _, _, "ἐι~χθε";
    perfect_imp_mid_3: _, _, "ἐι~χθων";
    pluperfect_mid_1: "ἐι~γμην", _, _;
    pluperfect_mid_2: "ἐι~ξο", "ἐι~χθον", _;
    pluperfect_mid_3: "ἐι~κτο", _, "ἐι~γᾰτο";
    ),
    // Ρρεω: 3 words (ῥεω, ἐκρεω, ρρεω). Like Ὁμᾰρτεω, except in 70 cells: ind_mid_2 sg, imperfect_pass_2 du, imperfect_pass_2 pl, imperfect_pass_3 du, future_ind_act_3 du, future_ind_mid_3 du, ….
    conjugation!("ρρεω", 3, "~εω", like Ὁμᾰρτεω;
    stem rest__: "~ευσ- like ἐχω";
    stem rest_______: "~ῠησ- like ἐχω";
    stem rest________: "~ευσ- like ἐχω";
    stem rest__________: "~ῠην- like ἐχω";
    stem rest_: "~ευσ- like ἐχω";
    stem rest_____: "~ῠ- like ἐγειρω";
    stem rest___: "~υηκ- like στηρῐζω";
    ind_mid_2: "~εει", _, _;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: derived, _, _;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_mid_3: _, derived, _;
    perfect_subj_pass_1: derived, _, derived;
    perfect_subj_pass_2: derived, derived, derived;
    perfect_subj_pass_3: derived, derived, derived;
    perfect_opt_pass_1: derived, _, derived;
    perfect_opt_pass_2: derived, derived, derived;
    perfect_opt_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    future_perfect_ind_mid_1: derived, _, _;
    future_perfect_ind_mid_3: _, derived, _;
    future_perfect_ind_pass_1: _, _, derived;
    future_perfect_ind_pass_2: derived, derived, derived;
    future_perfect_ind_pass_3: derived, derived, derived;
    future_perfect_opt_pass_1: derived, _, derived;
    future_perfect_opt_pass_2: derived, derived, derived;
    future_perfect_opt_pass_3: derived, derived, derived;
    infinitive_perf: _, derived, _;
    infinitive_futp: _, _, derived;
    participle_perf: _, _, derived;
    participle_futp: _, _, derived;
    ),
    // Σσῳζω: 3 words (ξῠσσῳζω, δῐᾰσῳζω, σσῳζω). Like Ληγω, except in 41 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("σσῳζω", 3, "~σῳζω", like Ληγω;
    stem lemma: "~σῳζ- like βᾰπτῐζω";
    stem rest____: "~ῳζ- like ἐχω";
    stem rest__: "~σωσ- like ἐχω";
    stem rest_______: "~σωθησ- like ἐχω";
    stem rest________: "~ωσ- like ἐχω";
    stem rest__________: "~ωθην- like ἐχω";
    stem rest_: "~σωσ- like ἐχω";
    stem rest_____: "~σωθ- like ἐχω";
    stem rest___: "~σεσωκ- like πεμπω";
    stem rest: "~σεσωσ- like ἐλπω";
    stem rest______: "~σεσωσ- like ἐλπω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Ρρηγνῡμῐ: 3 words (ῥηγνῡμῐ, ξῠρρηγνῡμῐ, ρρηγνῡμῐ). Like Πῠρσαινω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ρρηγνῡμῐ", 3, "~ηγνῡμῐ", like Πῠρσαινω;
    stem lemma: "~ηγν- like σβεννῡμῐ";
    stem rest____: "~ηγν- like σβεννῡμῐ";
    stem rest__: "~ηξ- like ἐχω";
    stem rest________: "~ηξ- like ἐχω";
    stem rest__________: "~ᾰγην- like ἐχω";
    stem rest_: "~ηξ- like ἐχω";
    stem rest_____: "~ᾰγ- like ἐγειρω";
    stem rest___: "~ωγ- like στηρῐζω";
    stem rest: "~ηρρη- like ἀνᾰγω";
    stem rest______: "~ηρρη- like ἀνᾰγω";
    stem rest_________: "~ηρρη- like ῥηγνῡμῐ";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "ἐ~ωγειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Θρεομαι: 3 words (ὠνεομαι, θρεομαι, λογεομαι). Like Δηλεομαι, except in 32 cells: ind_pass_1 sg, ind_pass_1 pl, ind_pass_2 sg, subj_mid_3 du, subj_pass_1 sg, subj_pass_1 pl, ….
    conjugation!("θρεομαι", 3, "~εομαι", like Δηλεομαι;
    stem lemma: "~ε- like οἰομαι";
    stem rest____: "~ε- like σῑνομαι";
    stem rest__: "~ησ- like ἑπομαι";
    stem rest_: "~ησ- like ὠνεομαι";
    stem rest_____: "~ηθ- like ἐρωτᾰω";
    stem rest: "~η- like ἐχω";
    ind_pass_1: derived, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: derived, _, derived;
    subj_pass_2: _, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_pass_2: derived, _, _;
    imp_pass_3: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_mid_3: _, _, derived;
    future_perfect_ind_pass_1: derived, _, _;
    ),
    // Κηρῡσσω: 3 words (πῠρεσσω, κηρῡσσω, ἀλλᾰσσω). Like Τᾰσσω, except in 16 cells: perfect_ind_act_3 du, perfect_ind_mid_3 du, perfect_ind_pass_2 sg, perfect_ind_pass_2 du, perfect_ind_pass_3 du, perfect_ind_pass_3 pl, ….
    conjugation!("κηρῡσσω", 3, "~σσω", like Τᾰσσω;
    stem rest___: "~χ- like ποιεω";
    stem rest______: "~- like φᾰρμᾰσσω";
    stem rest_________: "~- like φᾰρμᾰσσω";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_subj_mid_3: _, _, "%ε~γμένοι ὦσῐ|%ε~γμένοι ὦσῐν";
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ἐφᾰγον: 2 words (ἐφᾰγον, ἐπορον). Like Λλῡμῐ, except in 23 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, ….
    conjugation!("ἐφᾰγον", 2, "ἐ~ον", like Λλῡμῐ;
    ind_act_1: "ἐ~ον", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    aorist_ind_act_1: "ἐ~ον", _, "ἐ~ομεν";
    aorist_ind_act_2: "ἐ~ες", "ἐ~ετον", _;
    aorist_ind_act_3: "ἐ~ε", _, "ἐ~ον";
    aorist_subj_act_1: "~ω", _, _;
    aorist_subj_act_2: "~ῃς", "~ητον", _;
    aorist_opt_act_1: "~οιμῐ", _, "~οιμεν";
    aorist_opt_act_2: "~οις", _, _;
    aorist_opt_act_3: "~οι", _, "~οιεν";
    aorist_imp_act_2: "~ε", _, "~ετε";
    aorist_imp_act_3: _, _, "~οντων";
    infinitive_aor: "~ειν", _, _;
    participle_aor: "~ων", _, _;
    ),
    // Εἰδον: 2 words (εἰπον, εἰδον). Like Ἐφᾰγον, except in 5 cells: ind_act_1 sg, future_ind_act_3 du, future_ind_mid_3 du, aorist_subj_act_3 du, aorist_subj_mid_3 du.
    conjugation!("εἰδον", 2, "ε~ον", like Ἐφᾰγον;
    stem rest__: "~ησ- like ἐχω";
    stem rest________: "~- like εἰδον";
    stem rest_: "~- like φᾰρμᾰσσω";
    ind_act_1: "ε~ον", _, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    ),
    // Πτᾰρνῠμαι: 2 words (πτᾰρνῠμαι, καινῠμαι). Like Ἐφᾰγον, except in 37 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("πτᾰρνῠμαι", 2, "~νῠμαι", like Ἐφᾰγον;
    ind_act_1: "~νῠμαι", _, _;
    ind_mid_1: "~νῠμαι", _, "~νῠμεθᾰ";
    ind_mid_2: "~νῠσαι", _, _;
    ind_mid_3: _, "~νῠσθον", _;
    ind_pass_2: _, "~νῠσθον", "~νῠσθε";
    ind_pass_3: "~νῠται", "~νῠσθον", "~νῠνται";
    subj_mid_1: "~νῠωμαι", _, "~νῠωμεθᾰ";
    subj_mid_2: "~νῠῃ", "~νῠησθον", _;
    subj_mid_3: "~νῠηται", _, "~νῠωνται";
    subj_pass_2: "~νῠῃ", _, "~νῠησθε";
    opt_mid_1: "~νῠοιμην", _, "~νῠοιμεθᾰ";
    opt_mid_2: "~νῠοιο", "~νῠοισθον", "~νῠοισθε";
    opt_mid_3: "~νῠοιτο", "~νῠοισθην", "~νῠοιντο";
    imp_mid_2: "~νῠσο", "~νῠσθον", "~νῠσθε";
    imp_mid_3: "~νῠσθω", _, "~νῠσθων";
    imp_pass_2: _, "~νῠσθον", "~νῠσθε";
    imp_pass_3: _, "~νῠσθων", "~νῠσθων";
    infinitive_pres: _, _, "~νῠσθαι";
    participle_pres: _, _, "~νῠμενος";
    ),
    // Ξυμπολεμεω: 2 words (συμπολεμεω, ξυμπολεμεω). Like Σταθμαω, except in 53 cells: ind_act_3 du, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_1 sg, ind_pass_2 du, ….
    conjugation!("ξυμπολεμεω", 2, "~υμ~εω", like Σταθμαω;
    stem lemma: "~ῠμ~- like ξυνναυμᾰχεω";
    stem rest____: "~ῠνε~- like ξυνναυμᾰχεω";
    stem rest__: "~ῠμ~ησ- like ποιεω";
    stem rest_______: "~ῠμ~ηθησ- like ποιεω";
    stem rest________: "~ῠνε~ησ- like εὐλογεω";
    stem rest__________: "~ῠνε~ηθην- like ἐλπω";
    stem rest_: "~ῠμ~ησ- like ποιεω";
    stem rest_____: "~ῠμ~ηθ- like ποιεω";
    stem rest___: "~ῠμ%ε~ηκ- like εὐλογεω";
    stem rest: "~ῠμ%ε~η- like εὐλογεω";
    stem rest______: "~ῠμ%ε~η- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_mid_1: _, _, derived;
    ind_mid_2: derived, _, _;
    ind_mid_3: _, derived, _;
    ind_pass_1: derived, _, _;
    ind_pass_2: _, derived, derived;
    ind_pass_3: derived, derived, derived;
    subj_act_3: _, derived, _;
    subj_mid_1: _, _, derived;
    subj_mid_2: _, derived, _;
    subj_mid_3: derived, _, derived;
    subj_pass_1: derived, _, _;
    subj_pass_2: _, _, derived;
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, derived;
    opt_mid_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_mid_2: derived, derived, derived;
    imp_mid_3: _, _, derived;
    imp_pass_2: _, derived, derived;
    imp_pass_3: derived, derived, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_pres: _, _, derived;
    infinitive_perf: _, _, derived;
    participle_pres: _, _, derived;
    ),
    // Κᾰτακειμαι: 2 words (κᾰτακειμαι, δῐακειμαι). Like Ἐκγᾰμῐζω, except in 73 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("κᾰτακειμαι", 2, "~α~ειμαι", like Ἐκγᾰμῐζω;
    stem rest____: "~ε~ει- like δῐακειμαι";
    stem rest__: "~ᾰ~εισ- like ἑπομαι";
    ind_act_1: "~α~ειμαι", _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: derived, _, derived;
    ind_mid_1: "~ᾰ~ειμαι", _, "~ᾰ~ειμεθα";
    ind_mid_2: "~ᾰ~εισαι", _, _;
    ind_mid_3: _, "~ᾰ~εισθον", _;
    ind_pass_1: derived, _, _;
    ind_pass_2: _, derived, derived;
    ind_pass_3: derived, derived, derived;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    subj_mid_2: "~ᾰ~εῃ", "~ᾰ~εησθον", _;
    subj_mid_3: "~ᾰ~εηται", _, "~ᾰ~εωνται";
    subj_pass_1: derived, _, _;
    subj_pass_2: _, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~οιο", "~ᾰ~οισθον", "~ᾰ~οισθε";
    opt_mid_3: "~ᾰ~οιτο", "~ᾰ~οισθην", "~ᾰ~οιντο";
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imp_mid_2: "~ᾰ~εισο", "~ᾰ~εισθον", "~ᾰ~εισθον";
    imp_mid_3: "~ᾰ~εισθω", _, "~ᾰ~εισθων";
    imp_pass_2: _, derived, derived;
    imp_pass_3: derived, derived, derived;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    infinitive_pres: derived, _, derived;
    participle_pres: derived, _, derived;
    ),
    // Ξῠνοιδᾰ: 2 words (ξῠνοιδᾰ, προοιδᾰ). Like Τεθνηκοι with `δᾰ` after each pattern, except in 55 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, subj_act_1 sg, ….
    conjugation!("ξῠνοιδᾰ", 2, "~οιδᾰ", like Τεθνηκοι with "δᾰ";
    ind_act_1: _, _, "~ϊσμεν";
    ind_act_2: "~οισθᾰ|~οιδᾰς", "~ϊστον", _;
    ind_act_3: "~οιδε", _, "~ϊσᾱσῐ";
    subj_act_1: "~ειδω", _, "~ειδωμεν";
    subj_act_2: "~ειδῃς", "~ειδητον", _;
    subj_act_3: "~ειδῃ", _, "~ειδωσῐ|~ειδωσῐν";
    opt_act_1: "~ειδειην", _, "~ειδειμεν|~ειδειημεν";
    opt_act_2: "~ειδειης", "~ειδειτον", "~ειδειτε|~ειδειητε";
    opt_act_3: "~ειδειη", "~ειδειτην", "~ειδειεν|~ειδειησᾰν";
    imp_act_2: "~ϊσθῐ", _, "~ϊστε";
    imp_act_3: _, _, "~ϊστων";
    imperfect_act_1: "~ῃδη|~ῃδειν", _, "~ῃσμεν|~ῃδεμεν|~ῃδειμεν";
    imperfect_act_2: "~ῃδησθᾰ|~ῃδεις", "~ῃστον", "~ῃστε|~ῃδειτε|~ῃδετε";
    imperfect_act_3: "~ῃδει|~ῃδειν", _, "~ῃσᾰν|~ῃδεσᾰν";
    future_ind_mid_1: "~εισομαι", _, _;
    future_ind_mid_2: "~εισῃ|~εισει", "~εισεσθον", _;
    future_ind_mid_3: "~εισεται", _, _;
    future_opt_mid_1: "~εισοιμην", _, _;
    future_opt_mid_3: "~εισοιτο", "~εισοισθην", _;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: derived, _, derived;
    perfect_subj_act_3: _, derived, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    infinitive_pres: "~ειδεναι", _, _;
    participle_pres: "~ειδως", _, _;
    ),
    // Συγκειμαι: 2 words (συγκειμαι, ξυγκειμαι). Like Κᾰτακειμαι, except in 26 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_mid_3 pl, ….
    conjugation!("συγκειμαι", 2, "~γ~ειμαι", like Κᾰτακειμαι;
    stem rest____: "~νε~ε- like συγκειμαι";
    stem rest__: "~γ~εισ- like εἰσερχομαι";
    ind_act_1: "~γ~ειμαι", _, _;
    ind_mid_1: "~γ~ειμαι", _, "~γ~ειμεθα";
    ind_mid_2: "~γ~εισαι", _, _;
    ind_mid_3: _, "~γ~εισθον", "~γ~εινται|~γ~εᾰται|~γ~εονται|~γ~ειᾰται";
    subj_mid_1: "~γ~ωμαι", _, "~γ~ωμεθᾰ";
    subj_mid_2: "~γ~εῃ", "~γ~εησθον", _;
    subj_mid_3: "~γ~εηται|~γ~ειται", _, "~γ~εωνται|~γ~ειωνται";
    opt_mid_1: "~γ~οιμην", _, "~γ~οιμεθᾰ";
    opt_mid_2: "~γ~οιο", "~γ~οισθον", "~γ~οισθε";
    opt_mid_3: "~γ~οιτο", "~γ~οισθην", "~γ~οιντο";
    imp_mid_2: "~γ~εισο", "~γ~εισθον", "~γ~εισθον";
    imp_mid_3: "~γ~εισθω", _, "~γ~εισθων";
    future_ind_mid_3: _, derived, _;
    ),
    // Κορινθιαζομαι: 2 words (κορινθιαζομαι, νισσομαι). Like Κᾰτακειμαι, except in 50 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("κορινθιαζομαι", 2, "~ι~ομαι", like Κᾰτακειμαι;
    ind_act_1: "~ι~ομαι", _, _;
    ind_mid_1: "~ῐ~ομαι", _, "~ῐ~ομεθᾰ";
    ind_mid_2: "~ῐ~ῃ", _, _;
    ind_mid_3: _, "~ῐ~εσθον", _;
    ind_pass_2: _, "~ῐ~εσθον", "~ῐ~εσθε";
    ind_pass_3: "~ῐ~εται", "~ῐ~εσθον", "~ῐ~ονται";
    subj_mid_1: "~ῐ~ωμαι", _, "~ῐ~ωμεθᾰ";
    subj_mid_2: "~ῐ~ῃ", "~ῐ~ησθον", _;
    subj_mid_3: "~ῐ~ηται", _, "~ῐ~ωνται";
    subj_pass_2: "~ῐ~ῃ", _, "~ῐ~ησθε";
    opt_mid_1: "~ῐ~οιμην", _, "~ῐ~οιμεθᾰ";
    opt_mid_2: "~ῐ~οιο", "~ῐ~οισθον", "~ῐ~οισθε";
    opt_mid_3: "~ῐ~οιτο", "~ῐ~οισθην", "~ῐ~οιντο";
    imp_mid_2: "~ῐ~ου", "~ῐ~εσθον", "~ῐ~εσθε";
    imp_mid_3: "~ῐ~εσθω", _, "~ῐ~εσθων";
    imp_pass_2: _, "~ῐ~εσθον", "~ῐ~εσθε";
    imp_pass_3: _, "~ῐ~εσθων", "~ῐ~εσθων";
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, _;
    imperfect_mid_3: derived, _, derived;
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, _;
    future_opt_mid_3: derived, derived, _;
    infinitive_pres: _, _, "~ῐ~εσθαι";
    participle_pres: _, _, "~ῐ~ομενος";
    ),
    // Στεφανοω: 2 words (στεφανοω, κᾰτασκηνοω). Like Νεκραγωγεω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("στεφανοω", 2, "~α~οω", like Νεκραγωγεω;
    stem lemma: "~ᾰ~- like γῡροω";
    stem rest____: "~ᾰ~ου- like γῡροω";
    stem rest__: "~ᾰ~ωσ- like ἐπᾰνᾰχωρεω";
    stem rest_______: "~ᾰ~ωθησ- like ἐχω";
    stem rest________: "~ᾰ~ωσ- like ἐχω";
    stem rest__________: "~ᾰ~ωθην- like ἐχω";
    stem rest_: "~ᾰ~ωσ- like ποιεω";
    stem rest_____: "~ᾰ~ωθ- like ἐχω";
    stem rest___: "~ᾰ~ωκ- like στηρῐζω";
    stem rest: "~ᾰ~ω- like σβεννῡμῐ";
    stem rest______: "~ᾰ~ω- like σκοπεω";
    stem rest_________: "~ᾰ~ω- like σκοπεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "ἐ~ᾰ~ωκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ἐπῐχειρεω: 2 words (ἀρῐθμεω, ἐπῐχειρεω). Like Ὑπᾰνᾰχωρεω, except in 46 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἐπῐχειρεω", 2, "~ῐ~εω", like Ὑπᾰνᾰχωρεω;
    stem lemma: "~ι~- like τελεω";
    stem rest__: "~ι~ησ- like ἐχω";
    stem rest_______: "~ι~ηθησ- like ἐχω";
    stem rest________: "~ε~ησ- like ἐλπω";
    stem rest_: "~ι~ησ- like ἐχω";
    stem rest_____: "~ι~ηθ- like ἐχω";
    stem rest___: "~ε~ηκ- like πεμπω";
    stem rest: "~ε~η- like ἐλπω";
    stem rest______: "~ε~η- like εὐλογεω";
    stem rest_________: "~ε~η- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, derived;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: derived, derived, derived;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_act_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "~ε~ηκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Συνεπομαι: 2 words (συνεπομαι, ξυνεπομαι). Like Μᾰρνᾰμαι, except in 45 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("συνεπομαι", 2, "~υνεπομαι", like Μᾰρνᾰμαι;
    stem rest____: "~ῠνειπ- like εἰσερχομαι";
    stem rest__: "~ῠνεψ- like εἰσερχομαι";
    stem rest________: "~ῠνεσπ- like ὑπογῐγνομαι";
    stem rest_: "~ῠσπ- like ὑπογῐγνομαι";
    ind_act_1: "~υνεπομαι", _, _;
    ind_mid_1: "~ῠνεπομαι", _, "~ῠνεπομεθᾰ";
    ind_mid_2: "~ῠνεπῃ|~ῠνεπει", _, _;
    ind_mid_3: _, "~ῠνεπεσθον", _;
    ind_pass_2: _, "~ῠνεπεσθον", "~ῠνεπεσθε";
    ind_pass_3: "~ῠνεπεται", "~ῠνεπεσθον", "~ῠνεπονται";
    subj_mid_1: "~ῠνεπωμαι", _, "~ῠνεπωμεθᾰ";
    subj_mid_2: "~ῠνεπῃ", "~ῠνεπησθον", _;
    subj_mid_3: "~ῠνεπηται", _, "~ῠνεπωνται";
    subj_pass_2: "~ῠνεπῃ", _, "~ῠνεπησθε";
    opt_mid_1: "~ῠνεποιμην", _, "~ῠνεποιμεθᾰ";
    opt_mid_2: "~ῠνεποιο", "~ῠνεποισθον", "~ῠνεποισθε";
    opt_mid_3: "~ῠνεποιτο", "~ῠνεποισθην", "~ῠνεποιντο";
    imp_mid_2: "~ῠνεπου", "~ῠνεπεσθον", "~ῠνεπεσθε";
    imp_mid_3: "~ῠνεπεσθω", _, "~ῠνεπεσθων";
    imp_pass_2: _, "~ῠνεπεσθον", "~ῠνεπεσθε";
    imp_pass_3: _, "~ῠνεπεσθων", "~ῠνεπεσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    infinitive_pres: _, _, "~ῠνεπεσθαι";
    participle_pres: _, _, "~ῠνεπομενος";
    ),
    // Λαλαγεω: 2 words (καναχεω, λαλαγεω). Like Προβαλλω, except in 77 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("λαλαγεω", 2, "~α~α~εω", like Προβαλλω;
    stem lemma: "~ᾰ~ᾰ~- like ποιεω";
    stem rest____: "~ᾰ~ᾰ~- like ποιεω";
    stem rest__: "~ᾰ~ᾰ~ησ- like ποιεω";
    stem rest_______: "~ᾰ~ᾰ~ηθησ- like ἐχω";
    stem rest________: "~ᾰ~ᾰ~ησ- like ποιεω";
    stem rest__________: "~ᾰ~ᾰ~ηθην- like ἐχω";
    stem rest_: "~ᾰ~ᾰ~ηθ- like ἐχω";
    stem rest_____: "~ᾰ~ᾰ~ηθ- like ποιεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_1: _, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_mid_1: derived, _, _;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_3: _, derived, _;
    perfect_subj_mid_3: _, derived, _;
    perfect_subj_pass_1: derived, _, derived;
    perfect_subj_pass_2: derived, derived, derived;
    perfect_subj_pass_3: derived, derived, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_opt_pass_1: derived, _, derived;
    perfect_opt_pass_2: derived, derived, derived;
    perfect_opt_pass_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, _;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    infinitive_perf: _, derived, _;
    participle_perf: _, _, derived;
    ),
    // Ξυνναυμᾰχεω: 2 words (συνναυμᾰχεω, ξυνναυμᾰχεω). Like Λαλαγεω, except in 79 cells: ind_act_3 du, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_1 sg, ind_pass_2 du, ….
    conjugation!("ξυνναυμᾰχεω", 2, "~υ~~εω", like Λαλαγεω;
    stem lemma: "~ῠ~~- like ξυνναυμᾰχεω";
    stem rest____: "~ῠ~ε~- like ξυνναυμᾰχεω";
    stem rest__: "~ῠ~~ησ- like ξυνναυμᾰχεω";
    stem rest________: "~ῠ~ε~ησ- like ξυνναυμᾰχεω";
    stem rest_: "~ῠ~~ησ- like ξυνναυμᾰχεω";
    ind_act_3: _, derived, _;
    ind_mid_1: _, _, derived;
    ind_mid_2: derived, _, _;
    ind_mid_3: _, derived, _;
    ind_pass_1: derived, _, _;
    ind_pass_2: _, derived, derived;
    ind_pass_3: derived, derived, derived;
    subj_act_3: _, derived, _;
    subj_mid_1: _, _, derived;
    subj_mid_2: _, derived, _;
    subj_mid_3: derived, _, derived;
    subj_pass_1: derived, _, _;
    subj_pass_2: _, _, derived;
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, derived;
    opt_mid_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_mid_2: derived, derived, derived;
    imp_mid_3: _, _, derived;
    imp_pass_2: _, derived, derived;
    imp_pass_3: derived, derived, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, _, _;
    imperfect_mid_3: derived, _, derived;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_mid_1: derived, _, _;
    future_opt_mid_3: derived, derived, _;
    aorist_ind_mid_1: derived, _, _;
    aorist_ind_mid_2: derived, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_1: _, _, derived;
    aorist_subj_mid_2: derived, derived, _;
    aorist_subj_pass_1: derived, _, _;
    aorist_subj_pass_3: _, derived, derived;
    aorist_opt_mid_1: derived, _, _;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_mid_2: derived, derived, _;
    aorist_imp_mid_3: _, _, derived;
    aorist_imp_pass_2: derived, derived, derived;
    infinitive_pres: _, _, derived;
    participle_pres: _, _, derived;
    participle_aor: _, derived, _;
    ),
    // Μετᾰκᾰλεω: 2 words (μηχᾰνορρᾰφεω, μετᾰκᾰλεω). Like Προβαλλω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("μετᾰκᾰλεω", 2, "~ᾰ~ᾰ~εω", like Προβαλλω;
    stem lemma: "~α~α~- like ποιεω";
    stem rest____: "~ε~α~- like εὐλογεω";
    stem rest__: "~α~α~εσ- like ἐχω";
    stem rest_______: "~α~α~εθησ- like ἐχω";
    stem rest________: "~ε~α~εσ- like ἐλπω";
    stem rest__________: "~ε~α~εθην- like ἐλπω";
    stem rest_: "~α~α~εσ- like ἐχω";
    stem rest_____: "~α~α~εθ- like ἐχω";
    stem rest___: "~α~εκα~εκ- like πεμπω";
    stem rest: "~α~εκα~ε- like ἐλπω";
    stem rest______: "~α~εκα~ε- like εὐλογεω";
    stem rest_________: "~ε~εκα~ε- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "~ε~εκα~εκειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Επῐβουλευω: 2 words (επῐβουλευω, επῐλῡω). Like Πεμπω, except in 55 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("επῐβουλευω", 2, "ε~ῐ~ω", like Πεμπω;
    stem lemma: "~ῐ~- like επῐλῡω";
    stem rest____: "~ε~- like επῐλῡω";
    stem rest__: "~ῐ~σ- like ερεω";
    stem rest________: "~ε~σ- like επῐλῡω";
    stem rest_: "~ῐ~σ- like επῐλῡω";
    stem rest___: "~ῐ%ε~κ- like επῐλῡω";
    stem rest: "~ῐ%ε~- like επῐλῡω";
    stem rest______: "~ῐ%ε~- like επῐλῡω";
    stem rest_________: "~ε%ε~- like επῐλῡω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: "ἑ~ῐ~θησομαι", _, _;
    future_ind_pass_3: _, "ἑ~ῐ~θησεσθον", _;
    aorist_ind_pass_1: "ἑ~ε~θην", _, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_1: "ἑ~ῐ~θω", _, _;
    aorist_subj_pass_3: _, "ἑ~ῐ~θητον", _;
    aorist_imp_pass_2: "ἑ~ῐ~θητῐ", "ἑ~ῐ~θητον", "ἑ~ῐ~θητε";
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_act_1: "ἑ~ε%ε~κειν", _, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ερεω: 2 words (ερεω, ετεω). Like Λλῡμῐ, except in 25 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ερεω", 2, "ε~εω", like Λλῡμῐ;
    stem lemma: "~- like ερεω";
    stem rest__: "~ησ- like ερεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    ),
    // Ἐδω: 1 words (ἐδω). Like Λλῡμῐ, except in 23 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, future_ind_mid_3 du, perfect_ind_act_3 du, ….
    conjugation!("ἐδω", 1, "~ω", like Λλῡμῐ;
    stem rest__: "~- like εἰσερχομαι";
    stem rest___: "~ηδ- like ἐδω";
    stem rest: "~ηδο- like εὐλογεω";
    stem rest______: "~ηδο- like εὐλογεω";
    stem rest___________: "~ηδοκ- like εὐλογεω";
    stem rest_________: "~ηδο- like εὐλογεω";
    ind_act_1: "~ω", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_act_3: _, derived, _;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Αἰσθᾰνομαι: 1 words (αἰσθᾰνομαι). Like Μᾰρνᾰμαι, except in 141 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("αἰσθᾰνομαι", 1, "αἰ~ᾰνομαι", like Μᾰρνᾰμαι;
    stem rest_____: "~ηθ- like αἰσθᾰνομαι";
    ind_act_1: "αἰ~ᾰνομαι", _, _;
    ind_mid_1: "αἰ~ανομαι", _, "αἰ~ανομεθᾰ";
    ind_mid_2: "αἰ~ανῃ|αἰ~ανει", _, _;
    ind_mid_3: _, "αἰ~ανεσθον", _;
    ind_pass_2: _, "αἰ~ανεσθον", "αἰ~ανεσθε";
    ind_pass_3: "αἰ~ανεται", "αἰ~ανεσθον", "αἰ~ανονται";
    subj_mid_1: "αἰ~ανωμαι", _, "αἰ~ανωμεθᾰ";
    subj_mid_2: "αἰ~ανῃ", "αἰ~ανησθον", _;
    subj_mid_3: "αἰ~ανηται", _, "αἰ~ανωνται";
    subj_pass_2: "αἰ~ανῃ", _, "αἰ~ανησθε";
    opt_mid_1: "αἰ~ανοιμην", _, "αἰ~ανοιμεθᾰ";
    opt_mid_2: "αἰ~ανοιο", "αἰ~ανοισθον", "αἰ~ανοισθε";
    opt_mid_3: "αἰ~ανοιτο", "αἰ~ανοισθην", "αἰ~ανοιντο";
    imp_mid_2: "αἰ~ανου", "αἰ~ανεσθον", "αἰ~ανεσθε";
    imp_mid_3: "αἰ~ανεσθω", _, "αἰ~ανεσθων";
    imp_pass_2: _, "αἰ~ανεσθον", "αἰ~ανεσθε";
    imp_pass_3: _, "αἰ~ανεσθων", "αἰ~ανεσθων";
    imperfect_mid_1: "ᾐ~ανομην", _, "ᾐ~ανομεθᾰ";
    imperfect_mid_2: "ᾐ~ανου", "ᾐ~ανεσθον", _;
    imperfect_mid_3: "ᾐ~ανετο", _, "ᾐ~ανοντο";
    imperfect_pass_2: _, _, "ᾐ~ανεσθε";
    imperfect_pass_3: _, "ᾐ~ανεσθην", _;
    future_ind_mid_1: "αἰ~ησομαι", _, _;
    future_ind_mid_2: "αἰ~ησῃ|αἰ~ησει", "αἰ~ησεσθον", _;
    future_ind_mid_3: "αἰ~ησεται", _, _;
    future_ind_pass_1: "αἰ~ηθησομαι|αἰ~ανθησομαι", _, "αἰ~ηθησομεθᾰ|αἰ~ανθησομεθᾰ";
    future_ind_pass_2: "αἰ~ηθησῃ|αἰ~ηθησει|αἰ~ανθησῃ|αἰ~ανθησει", "αἰ~ηθησεσθον|αἰ~ανθησεσθον", "αἰ~ηθησεσθε|αἰ~ανθησεσθε";
    future_ind_pass_3: "αἰ~ηθησεται|αἰ~ανθησεται", _, "αἰ~ηθησονται|αἰ~ανθησονται";
    future_opt_mid_1: "αἰ~ησοιμην", _, _;
    future_opt_mid_3: "αἰ~ησοιτο", "αἰ~ησοισθην", _;
    future_opt_pass_1: "αἰ~ηθησοιμην|αἰ~ανθησοιμην", _, "αἰ~ηθησοιμεθᾰ|αἰ~ανθησοιμεθᾰ";
    future_opt_pass_2: "αἰ~ηθησοιο|αἰ~ανθησοιο", "αἰ~ηθησοισθον|αἰ~ανθησοισθον", "αἰ~ηθησοισθε|αἰ~ανθησοισθε";
    future_opt_pass_3: "αἰ~ηθησοιτο|αἰ~ανθησοιτο", "αἰ~ηθησοισθην|αἰ~ανθησοισθην", "αἰ~ηθησοιντο|αἰ~ανθησοιντο";
    aorist_ind_mid_1: "ᾐ~ομην|ᾐ~ησᾰμην", _, "ᾐ~ομεθᾰ|ᾐ~ησᾰμεθᾰ";
    aorist_ind_mid_2: "ᾐ~ου|ᾐ~ησω", "ᾐ~εσθον|ᾐ~ησᾰσθον", "ᾐ~εσθε|ᾐ~ησᾰσθε";
    aorist_ind_mid_3: "ᾐ~ετο|ᾐ~ησᾰτο", "ᾐ~εσθην|ᾐ~ησᾰσθην", "ᾐ~οντο|ᾐ~ησᾰντο";
    aorist_ind_pass_1: "ᾐ~ηθην", _, _;
    aorist_subj_mid_1: "αἰ~ωμαι|αἰ~ησωμαι", _, "αἰ~ωμεθᾰ|αἰ~ησωμεθᾰ";
    aorist_subj_mid_2: "αἰ~ῃ|αἰ~ησῃ", "αἰ~ησθον|αἰ~ησησθον", "αἰ~ησθε|αἰ~ησησθε";
    aorist_subj_mid_3: "αἰ~ηται|αἰ~ησηται", _, "αἰ~ωνται|αἰ~ησωνται";
    aorist_opt_mid_1: "αἰ~οιμην|αἰ~ησαιμην", _, "αἰ~οιμεθᾰ|αἰ~ησαιμεθᾰ";
    aorist_opt_mid_2: "αἰ~οιο|αἰ~ησαιο", "αἰ~οισθον|αἰ~ησαισθον", "αἰ~οισθε|αἰ~ησαισθε";
    aorist_opt_mid_3: "αἰ~οιτο|αἰ~ησαιτο", "αἰ~οισθην|αἰ~ησαισθην", "αἰ~οιντο|αἰ~ησαιντο";
    aorist_imp_mid_2: "αἰ~ου|αἰ~ησαι", "αἰ~εσθον|αἰ~ησᾰσθον", "αἰ~εσθε|αἰ~ησᾰσθε";
    aorist_imp_mid_3: "αἰ~εσθω|αἰ~ησᾰσθω", "αἰ~εσθων|αἰ~ησᾰσθων", _;
    perfect_ind_mid_1: "ᾐ~ημαι", _, _;
    perfect_ind_mid_2: "ᾐ~ησαι", "ᾐ~ησθον", _;
    perfect_ind_mid_3: "ᾐ~ηται", _, "ᾐ~ηνται";
    perfect_ind_pass_1: _, _, "ᾐ~ημεθᾰ";
    perfect_ind_pass_2: _, _, "ᾐ~ησθε";
    perfect_subj_mid_3: _, "ᾐ~ημένω ἦτον", "ᾐ~ημένοι ὦσῐ|ᾐ~ημένοι ὦσῐν";
    perfect_subj_pass_1: "ᾐ~ημένος ὦ", _, "ᾐ~ημένοι ὦμεν";
    perfect_subj_pass_2: "ᾐ~ημένος ᾖς", "ᾐ~ημένω ἦτον", "ᾐ~ημένοι ἦτε";
    perfect_subj_pass_3: "ᾐ~ημένος ᾖ", "ᾐ~ημένω ἦτον", _;
    perfect_opt_pass_1: "ᾐ~ημένος εἴην", _, "ᾐ~ημένοι εἴημεν";
    perfect_opt_pass_2: "ᾐ~ημένος εἴης", "ᾐ~ημένω εἴητον", "ᾐ~ημένοι εἴητε";
    perfect_opt_pass_3: "ᾐ~ημένος εἴη", "ᾐ~ημένω εἰήτην", "ᾐ~ημένοι εἴησᾰν";
    perfect_imp_mid_2: _, _, "ᾐ~ησθε";
    perfect_imp_mid_3: _, _, "ᾐ~ησθων";
    perfect_imp_pass_2: "ᾐ~ησο", _, "ᾐ~ησθε";
    perfect_imp_pass_3: "ᾐ~ησθω", "ᾐ~ησθων", "ᾐ~ησθων";
    pluperfect_mid_1: "ᾐ~ημην", _, _;
    pluperfect_mid_2: "ᾐ~ησο", "ᾐ~ησθον", _;
    pluperfect_mid_3: "ᾐ~ητο", _, "ᾐ~ηντο";
    pluperfect_pass_1: _, _, "ᾐ~ημεθᾰ";
    pluperfect_pass_2: _, _, "ᾐ~ησθε";
    pluperfect_pass_3: _, "ᾐ~ησθην", _;
    infinitive_pres: _, _, "αἰ~ανεσθαι";
    infinitive_fut: _, _, "αἰ~ηθησεσθαι|αἰ~ανθησεσθαι";
    infinitive_aor: _, "αἰ~εσθαι|αἰ~ησᾰσθαι", _;
    infinitive_perf: _, "ᾐ~ησθαι", _;
    participle_pres: _, _, "αἰ~ανομενος";
    participle_fut: _, _, "αἰ~ηθησομενος|αἰ~ανθησομενος";
    participle_aor: _, "αἰ~ομενος|αἰ~ησᾰμενος", _;
    participle_perf: _, _, "ᾐ~ημενος";
    ),
    // Τρυπαω: 1 words (τρυπαω). Like Ἐκγᾰμῐζω, except in 45 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("τρυπαω", 1, "~υ~αω", like Ἐκγᾰμῐζω;
    stem lemma: "~ῡ~- like φοιτᾰω";
    stem rest____: "~ῡ~- like φοιτᾰω";
    stem rest________: "~ῡ~ησ- like ποιεω";
    stem rest_: "~ῡ~ησ- like ποιεω";
    stem rest___: "~ῡ~ηκ- like ποιεω";
    stem rest: "~ῡ~η- like χρημᾰτῐζω";
    stem rest______: "~ῡ~η- like ἐχω";
    stem rest___________: "~ῡ~ηκ- like χρημᾰτῐζω";
    stem rest_________: "~ῡ~η- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Μειδᾰω: 1 words (μειδᾰω). Like Λλῡμῐ, except in 4 cells: ind_act_3 du, subj_act_3 du, imp_act_2 du, aorist_subj_act_3 du.
    conjugation!("μειδᾰω", 1, "~ᾰω", like Λλῡμῐ;
    stem lemma: "~ι- like μειδᾰω";
    stem rest____: "~ι- like μειδᾰω";
    stem rest________: "~ιᾱσ- like χᾰριζω";
    stem rest_: "~ιᾱσ- like ξυνναυμᾰχεω";
    ind_act_3: _, derived, _;
    subj_act_3: _, derived, _;
    imp_act_2: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    ),
    // Ἀνᾰχρονῐζομαι: 1 words (ἀνᾰχρονῐζομαι). Like Κᾰτακειμαι, except in 61 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἀνᾰχρονῐζομαι", 1, "~ᾰ~ζομαι", like Κᾰτακειμαι;
    stem rest____: "~ε~ζ- like εἰσερχομαι";
    stem rest__: "~α~- like ὑποκοριζομαι";
    stem rest_______: "~α~σθησ- like ποιεω";
    stem rest________: "~ε~σ- like μηκᾰομαι";
    stem rest__________: "~ε~σθην- like ἐλπω";
    stem rest_: "~α~σ- like ἑπομαι";
    stem rest_____: "~α~σθ- like ποιεω";
    stem rest: "~α%ε~σ- like εὐλογεω";
    stem rest______: "~α%ε~- like ἱζω";
    stem rest_________: "~ε%ε~- like ἱζω";
    ind_act_1: "~ᾰ~ζομαι", _, _;
    ind_mid_1: "~α~ζομαι", _, "~α~ζομεθᾰ";
    ind_mid_2: "~α~ζῃ|~α~ζει", _, _;
    ind_mid_3: _, "~α~ζεσθον", _;
    ind_pass_2: _, "~α~ζεσθον", "~α~ζεσθε";
    ind_pass_3: "~α~ζεται", "~α~ζεσθον", "~α~ζονται";
    subj_mid_1: "~α~ζωμαι", _, "~α~ζωμεθᾰ";
    subj_mid_2: "~α~ζῃ", "~α~ζησθον", _;
    subj_mid_3: "~α~ζηται", _, "~α~ζωνται";
    subj_pass_2: "~α~ζῃ", _, "~α~ζησθε";
    opt_mid_1: "~α~ζοιμην", _, "~α~ζοιμεθᾰ";
    opt_mid_2: "~α~ζοιο", "~α~ζοισθον", "~α~ζοισθε";
    opt_mid_3: "~α~ζοιτο", "~α~ζοισθην", "~α~ζοιντο";
    imp_mid_2: "~α~ζου", "~α~ζεσθον", "~α~ζεσθε";
    imp_mid_3: "~α~ζεσθω", _, "~α~ζεσθων";
    imp_pass_2: _, "~α~ζεσθον", "~α~ζεσθε";
    imp_pass_3: _, "~α~ζεσθων", "~α~ζεσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_pres: _, _, "~α~ζεσθαι";
    infinitive_perf: _, _, derived;
    participle_pres: _, _, "~α~ζομενος";
    ),
    // Σῠμμᾰχομαι: 1 words (σῠμμᾰχομαι). Like Κορινθιαζομαι, except in 39 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("σῠμμᾰχομαι", 1, "~ῠμ~ομαι", like Κορινθιαζομαι;
    stem rest__: "~υμ~- like ὑποκοριζομαι";
    stem rest________: "~υνε~εσ- like μηκᾰομαι";
    stem rest_: "~υμ~εσ- like ἑπομαι";
    ind_act_1: "~ῠμ~ομαι", _, _;
    ind_mid_1: "~υμ~ομαι", _, "~υμ~ομεθᾰ";
    ind_mid_2: "~υμ~ῃ|~υμ~ει", _, _;
    ind_mid_3: _, "~υμ~εσθον", _;
    ind_pass_2: _, "~υμ~εσθον", "~υμ~εσθε";
    ind_pass_3: "~υμ~εται", "~υμ~εσθον", "~υμ~ονται";
    subj_mid_1: "~υμ~ωμαι", _, "~υμ~ωμεθᾰ";
    subj_mid_2: "~υμ~ῃ", "~υμ~ησθον", _;
    subj_mid_3: "~υμ~ηται", _, "~υμ~ωνται";
    subj_pass_2: "~υμ~ῃ", _, "~υμ~ησθε";
    opt_mid_1: "~υμ~οιμην", _, "~υμ~οιμεθᾰ";
    opt_mid_2: "~υμ~οιο", "~υμ~οισθον", "~υμ~οισθε";
    opt_mid_3: "~υμ~οιτο", "~υμ~οισθην", "~υμ~οιντο";
    imp_mid_2: "~υμ~ου", "~υμ~εσθον", "~υμ~εσθε";
    imp_mid_3: "~υμ~εσθω", _, "~υμ~εσθων";
    imp_pass_2: _, "~υμ~εσθον", "~υμ~εσθε";
    imp_pass_3: _, "~υμ~εσθων", "~υμ~εσθων";
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    infinitive_pres: _, _, "~υμ~εσθαι";
    participle_pres: _, _, "~υμ~ομενος";
    ),
    // Κᾰθημαι: 1 words (κᾰθημαι). Like Κᾰτακειμαι, except in 45 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, subj_mid_1 sg, ….
    conjugation!("κᾰθημαι", 1, "~ᾰ~μαι", like Κᾰτακειμαι;
    stem rest_________: "~α~- like κᾰθημαι";
    ind_act_1: "~ᾰ~μαι", _, _;
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, _, _;
    ind_mid_3: _, derived, _;
    subj_mid_1: derived, _, derived;
    subj_mid_2: derived, derived, _;
    subj_mid_3: derived, _, derived;
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, derived;
    opt_mid_3: derived, derived, derived;
    imp_mid_2: derived, derived, derived;
    imp_mid_3: derived, _, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, _;
    imperfect_mid_3: derived, _, derived;
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, _;
    future_opt_mid_3: derived, derived, _;
    pluperfect_pass_1: derived, _, derived;
    pluperfect_pass_2: derived, derived, derived;
    pluperfect_pass_3: derived, derived, derived;
    ),
    // Ἐπῐκᾰλεω: 1 words (ἐπῐκᾰλεω). Like Μετᾰκᾰλεω, except in 42 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἐπῐκᾰλεω", 1, "~ῐ~ᾰ~εω", like Μετᾰκᾰλεω;
    stem lemma: "~ι~α~- like ποιεω";
    stem rest__: "~ι~α~- like χρημᾰτῐζω";
    stem rest_______: "~ι%ε~~ησ- like ποιεω";
    stem rest________: "~ε~α~εσ- like εὐλογεω";
    stem rest__________: "~ε~~ηθην- like ἐλπω";
    stem rest_: "~ι~α~εσ- like ποιεω";
    stem rest_____: "~ι~~ηθ- like ποιεω";
    stem rest___: "~ι%ε~~ηκ- like εὐλογεω";
    stem rest: "~ι%ε~~η- like εὐλογεω";
    stem rest______: "~ι%ε~~η- like εὐλογεω";
    stem rest___________: "~ε%ε~~ηκ- like εὐλογεω";
    stem rest_________: "~ε%ε~~η- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Χρη: 1 words (χρη). Like Λλῡμῐ, except in 13 cells: ind_act_1 sg, ind_act_3 sg, subj_act_3 sg, opt_act_3 sg, imp_act_3 sg, imperfect_act_1 sg, ….
    conjugation!("χρη", 1, "~η", like Λλῡμῐ;
    ind_act_1: "~η", _, _;
    ind_act_3: "~η", _, _;
    subj_act_3: "~ῃ|~ὴ ᾖ", _, _;
    opt_act_3: "~ειη|~ὴ εἴη", _, _;
    imp_act_3: "~εὼν ἔστω|~εὼν ἤτω", _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_3: "~ησει", _, _;
    future_ind_mid_3: "~ησται", _, _;
    future_opt_act_3: "~ησοι", _, _;
    future_opt_mid_3: "~ησοιτο", _, _;
    infinitive_pres: "~ηναι|~ὴ εἶναι", _, _;
    infinitive_fut: "~ησειν", _, _;
    ),
    // Ἠλθον: 1 words (ἠλθον). Like Ἐφᾰγον, except in 15 cells: ind_act_1 sg, aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, aorist_opt_act_1 sg, ….
    conjugation!("ἠλθον", 1, "ἠ~ον", like Ἐφᾰγον;
    stem rest________: "~- like ἠλθον";
    ind_act_1: "ἠ~ον", _, _;
    aorist_subj_act_1: "ἐ~ω", _, _;
    aorist_subj_act_2: "ἐ~ῃς", "ἐ~ητον", _;
    aorist_subj_act_3: _, _, "ἐ~ωσῐ|ἐ~ωσῐν";
    aorist_opt_act_1: "ἐ~οιμῐ", _, "ἐ~οιμεν";
    aorist_opt_act_2: "ἐ~οις", _, _;
    aorist_opt_act_3: "ἐ~οι", _, "ἐ~οιεν";
    aorist_imp_act_2: "ἐ~ε", _, "ἐ~ετε";
    aorist_imp_act_3: _, _, "ἐ~οντων";
    infinitive_aor: "ἐ~ειν", _, _;
    participle_aor: "ἐ~ων", _, _;
    ),
    // Ἀρνυμαι: 1 words (ἀρνυμαι). Like Πτᾰρνῠμαι, except in 21 cells: ind_act_1 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_3 sg, ….
    conjugation!("ἀρνυμαι", 1, "~νυμαι", like Πτᾰρνῠμαι;
    stem rest________: "~- like ὑπογῐγνομαι";
    stem rest_: "~- like ὑπογῐγνομαι";
    ind_act_1: "~νυμαι", _, _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_mid_3: _, derived, _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    infinitive_aor: derived, _, _;
    participle_aor: derived, _, _;
    ),
    // Ἐπεφνον: 1 words (ἐπεφνον). Like Ἐφᾰγον, except in 17 cells: ind_act_1 sg, aorist_subj_act_3 du, perfect_ind_mid_2 sg, perfect_ind_mid_2 du, perfect_ind_mid_3 pl, perfect_ind_pass_1 sg, ….
    conjugation!("ἐπεφνον", 1, "ἐ~νον", like Ἐφᾰγον;
    stem rest_______: "~ησ- like ποιεω";
    stem rest________: "~ν- like ἐπεφνον";
    stem rest_: "~ν- like εἰσερχομαι";
    stem rest: "~α- like εὐλογεω";
    ind_act_1: "ἐ~νον", _, _;
    aorist_subj_act_3: _, derived, _;
    perfect_ind_mid_2: "~ασαι", "~ασθον", _;
    perfect_ind_mid_3: _, _, "~ανται";
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, "~ασθε";
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, _, "~ασθε";
    perfect_imp_mid_3: _, _, "~ασθων";
    perfect_imp_pass_2: "~ασο", _, "~ασθε";
    perfect_imp_pass_3: "~ασθω", "~ασθων", "~ασθων";
    infinitive_perf: _, "~ᾱσθαι", _;
    ),
    // Μετεκῑᾰθον: 1 words (μετεκῑᾰθον). Like Κᾰθημαι, except in 10 cells: ind_act_1 sg, aorist_subj_act_3 du, pluperfect_mid_1 sg, pluperfect_mid_1 pl, pluperfect_mid_2 sg, pluperfect_mid_2 du, ….
    conjugation!("μετεκῑᾰθον", 1, "~ε~ον", like Κᾰθημαι;
    stem rest________: "~ε~- like εἰσερχομαι";
    stem rest_: "~α~- like εἰσερχομαι";
    ind_act_1: "~ε~ον", _, _;
    aorist_subj_act_3: _, derived, _;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    ),
    // Πεπνῡμαι: 1 words (πεπνῡμαι). Like Λλῡμῐ, except in 4 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg.
    conjugation!("πεπνῡμαι", 1, "πε~μαι", like Λλῡμῐ;
    stem rest__________: "~θην- like ἐχω";
    stem rest_____: "~θ- like ποιεω";
    ind_act_1: "πε~μαι", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    ),
    // Περνημῐ: 1 words (περνημῐ). Like Λλῡμῐ, except in 6 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, future_ind_act_3 du, future_ind_mid_3 du.
    conjugation!("περνημῐ", 1, "~νημῐ", like Λλῡμῐ;
    stem rest__: "~- like περνημῐ";
    stem rest_______: "~ᾰθησ- like ποιεω";
    ind_act_1: "~νημῐ", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    ),
    // Ἐπῐστᾰμαι: 1 words (ἐπῐστᾰμαι). Like Κᾰτακειμαι, except in 37 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, subj_mid_1 sg, ….
    conjugation!("ἐπῐστᾰμαι", 1, "ἐ~ῐ~ᾰμαι", like Κᾰτακειμαι;
    stem rest__________: "~ι~ηθην- like ἀθῡρω";
    stem rest_____: "~ι~ηθ- like ἐλπῐζω";
    ind_act_1: "ἐ~ῐ~ᾰμαι", _, _;
    ind_mid_1: "ἐ~ι~ᾰμαι", _, "ἐ~ι~ᾰμεθᾰ";
    ind_mid_2: "ἐ~ι~ᾰσαι|ἐ~ι~ᾳ|ἐ~ι~ῃ", _, _;
    ind_mid_3: _, "ἐ~ι~ᾰσθον", _;
    subj_mid_1: "ἐ~ι~ωμαι|ἐ~ι~ωμαι", _, "ἐ~ι~ωμεθᾰ";
    subj_mid_2: "ἐ~ι~ῃ", "ἐ~ι~ησθον", _;
    subj_mid_3: "ἐ~ι~ηται", _, "ἐ~ι~ωνται";
    opt_mid_1: "ἐ~ι~αιμην", _, "ἐ~ι~αιμεθᾰ";
    opt_mid_2: "ἐ~ι~αιο", "ἐ~ι~αισθον", "ἐ~ι~αισθε";
    opt_mid_3: "ἐ~ι~αιτο", "ἐ~ι~αισθην", "ἐ~ι~αιντο";
    imp_mid_2: "ἐ~ι~ᾰσο|ἐ~ι~ω", "ἐ~ι~ᾰσθον", "ἐ~ι~ᾰσθε";
    imp_mid_3: "ἐ~ι~ᾰσθω", _, "ἐ~ι~ᾰσθων";
    imperfect_mid_1: "ἠ~ι~ᾰμην", _, "ἠ~ι~ᾰμεθᾰ";
    imperfect_mid_2: "ἠ~ῐ~ω|ἠ~ῐ~ᾰσο", "ἠ~ι~ᾰσθον", _;
    imperfect_mid_3: "ἠ~ι~ᾰτο", _, "ἠ~ι~ᾰντο";
    future_ind_mid_1: "ἐ~ι~ησομαι", _, _;
    future_ind_mid_2: "ἐ~ι~ησῃ|ἐ~ι~ησει", "ἐ~ι~ησεσθον", _;
    future_ind_mid_3: "ἐ~ι~ησεται", _, _;
    future_opt_mid_1: "ἐ~ι~ησοιμην", _, _;
    future_opt_mid_3: "ἐ~ι~ησοιτο", "ἐ~ι~ησοισθην", _;
    ),
    // Ἀπαυραω: 1 words (ἀπαυραω). Like Μετεκῑᾰθον, except in 10 cells: ind_act_1 sg, imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_3 sg, imperfect_pass_3 pl, ….
    conjugation!("ἀπαυραω", 1, "~α~αω", like Μετεκῑᾰθον;
    stem rest____: "~η~- like ἀπαυραω";
    stem rest__: "~ο~ησ- like ποιεω";
    stem rest________: "~ο~- like εὐλογεω";
    stem rest_: "~ο~- like ποιεω";
    ind_act_1: "~α~αω", _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    ),
    // Προκειμαι: 1 words (προκειμαι). Like Κᾰτακειμαι, except in 39 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, subj_mid_1 sg, ….
    conjugation!("προκειμαι", 1, "~~ειμαι", like Κᾰτακειμαι;
    ind_act_1: "~~ειμαι", _, _;
    ind_mid_1: "~~ειμαι", _, "~~ειμεθα";
    ind_mid_2: "~~εισαι", _, _;
    ind_mid_3: _, "~~εισθον", _;
    subj_mid_1: "~~ωμαι", _, "~~ωμεθᾰ";
    subj_mid_2: "~~εῃ", "~~εησθον", _;
    subj_mid_3: "~~εηται", _, "~~εωνται|~~ειωνται";
    opt_mid_1: "~~οιμην", _, "~~οιμεθᾰ";
    opt_mid_2: "~~οιο", "~~οισθον", "~~οισθε";
    opt_mid_3: "~~οιτο", "~~οισθην", "~~οιντο";
    imp_mid_2: "~~εισο", "~~εισθον", "~~εισθον";
    imp_mid_3: "~~εισθω", _, "~~εισθων";
    imperfect_mid_1: "~ε~ειμην|~υ~ειμην", _, "~ε~ειμεθα|~υ~ειμεθα";
    imperfect_mid_2: "~ε~εισο|~υ~εισο", "~ε~εισθον|~υ~εισθον", "~ε~εισθε|~υ~εισθε";
    imperfect_mid_3: "~ε~ειτο|~υ~ειτο", "~ε~εισθην|~υ~εισθην", "~ε~εατο|~υ~εατο";
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, _;
    future_opt_mid_3: derived, derived, _;
    ),
    // Ἐπῐπειθομαι: 1 words (ἐπῐπειθομαι). Like Κᾰτακειμαι, except in 25 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, subj_mid_1 sg, ….
    conjugation!("ἐπῐπειθομαι", 1, "~ῐ~θομαι", like Κᾰτακειμαι;
    stem rest____: "~ε~θ- like ὑποκοριζομαι";
    stem rest__: "~ι~σ- like εἰσερχομαι";
    ind_act_1: "~ῐ~θομαι", _, _;
    ind_mid_1: "~ι~θομαι", _, "~ι~θομεθᾰ";
    ind_mid_2: "~ι~θῃ|~ι~θει", _, _;
    ind_mid_3: _, "~ι~θεσθον", _;
    subj_mid_1: "~ι~θωμαι", _, "~ι~θωμεθᾰ";
    subj_mid_2: "~ι~θῃ", "~ι~θησθον", _;
    subj_mid_3: "~ι~θηται", _, "~ι~θωνται";
    opt_mid_1: "~ι~θοιμην", _, "~ι~θοιμεθᾰ";
    opt_mid_2: "~ι~θοιο", "~ι~θοισθον", "~ι~θοισθε";
    opt_mid_3: "~ι~θοιτο", "~ι~θοισθην", "~ι~θοιντο";
    imp_mid_2: "~ι~θου", "~ι~θεσθον", "~ι~θεσθε";
    imp_mid_3: "~ι~θεσθω", _, "~ι~θεσθων";
    future_ind_mid_3: _, derived, _;
    ),
    // Ἐπισευω: 1 words (ἐπισευω). Like Κᾰθημαι, except in 22 cells: ind_act_1 sg, perfect_ind_mid_3 du, perfect_ind_pass_1 sg, perfect_ind_pass_2 sg, perfect_ind_pass_2 du, perfect_ind_pass_3 sg, ….
    conjugation!("ἐπισευω", 1, "~ι~ευω", like Κᾰθημαι;
    stem rest: "~ι~- like ἐπισευω";
    stem rest______: "~ι~ῠ- like εὐλογεω";
    stem rest_________: "~ε~ῠ- like εὐλογεω";
    ind_act_1: "~ι~ευω", _, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_1: derived, _, _;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_mid_1: _, _, derived;
    pluperfect_mid_2: _, _, derived;
    pluperfect_mid_3: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    participle_perf: _, _, derived;
    ),
    // Δειμᾰτοω: 1 words (δειμᾰτοω). Like Ζωγρᾰφεω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("δειμᾰτοω", 1, "~ᾰ~οω", like Ζωγρᾰφεω;
    stem lemma: "~α~- like γῡροω";
    stem rest____: "~α~ου- like γῡροω";
    stem rest__: "~α~ωσ- like ποιεω";
    stem rest_______: "~α~ωθησ- like ποιεω";
    stem rest________: "~α~ωσ- like ποιεω";
    stem rest__________: "~α~ωθην- like ἐχω";
    stem rest_: "~α~ωσ- like ποιεω";
    stem rest_____: "~α~ωθ- like ποιεω";
    stem rest___: "~α~ωκ- like ποιεω";
    stem rest: "~α~ω- like χρημᾰτῐζω";
    stem rest______: "~α~ω- like ἐχω";
    stem rest___________: "~α~ωκ- like χρημᾰτῐζω";
    stem rest_________: "~α~ω- like ἐχω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Κορυσσω: 1 words (κορυσσω). Like Ἐφᾰγον, except in 27 cells: ind_act_1 sg, imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_3 sg, imperfect_pass_3 pl, ….
    conjugation!("κορυσσω", 1, "~υσσω", like Ἐφᾰγον;
    stem rest____: "~ῠσσ- like ἐλπω";
    stem rest________: "~ῠσσ- like ποιεω";
    stem rest_: "~ῠσσ- like ποιεω";
    stem rest______: "~ῠ- like κορυσσω";
    ind_act_1: "~υσσω", _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_pass_1: "%ε~υθμαι", _, "%ε~υθμεθᾰ";
    perfect_ind_pass_3: "%ε~ῠσται", _, _;
    perfect_subj_pass_1: "%ε~υθμένος ὦ", _, "%ε~υθμένοι ὦμεν";
    perfect_subj_pass_2: "%ε~υθμένος ᾖς", "%ε~υθμένω ἦτον", "%ε~υθμένοι ἦτε";
    perfect_subj_pass_3: "%ε~υθμένος ᾖ", "%ε~υθμένω ἦτον", "%ε~υθμένοι ὦσῐ|%ε~υθμένοι ὦσῐν";
    perfect_opt_pass_1: "%ε~υθμένος εἴην", _, "%ε~υθμένοι εἴημεν";
    perfect_opt_pass_2: "%ε~υθμένος εἴης", "%ε~υθμένω εἴητον", "%ε~υθμένοι εἴητε";
    perfect_opt_pass_3: "%ε~υθμένος εἴη", "%ε~υθμένω εἰήτην", "%ε~υθμένοι εἴησᾰν";
    ),
    // Κεκᾰδον: 1 words (κεκᾰδον). Like Μετεκῑᾰθον, except in 8 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 du, future_ind_act_3 sg, future_ind_act_3 pl, ….
    conjugation!("κεκᾰδον", 1, "~ᾰ~ον", like Μετεκῑᾰθον;
    stem rest________: "~ᾰ~- like ἀνᾰλᾰμβᾰνω";
    stem rest_: "~ᾰ~- like ἀνᾰγω";
    ind_act_1: "~ᾰ~ον", _, _;
    future_ind_act_1: "~α~ησω", _, "~α~ησομεν";
    future_ind_act_2: _, "~α~ησετον", _;
    future_ind_act_3: "~α~ησει", _, "~α~ησουσῐ|~α~ησουσῐν";
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    ),
    // Κᾰτᾰκλαω: 1 words (κᾰτᾰκλαω). Like Ξυνναυμᾰχεω, except in 53 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_act_2 du, subj_mid_2 sg, subj_mid_3 du, ….
    conjugation!("κᾰτᾰκλαω", 1, "~ᾰ~ᾰ~αω", like Ξυνναυμᾰχεω;
    stem lemma: "~ᾰ~α~- like φοιτᾰω";
    stem rest____: "~α~ε~- like ἀπαυραω";
    stem rest________: "~ᾰ~ε~ᾰσ- like εὐλογεω";
    stem rest__________: "~α~ε~ασθην- like ἐλπω";
    stem rest_: "~ᾰ~α~ᾰσ- like ποιεω";
    stem rest_____: "~ᾰ~α~ασθ- like ποιεω";
    stem rest: "~ᾰ~ᾰ%ε~ᾰσ- like εὐλογεω";
    stem rest______: "~ᾰ~ᾰ%ε~ᾰσ- like ἐλπω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_act_2: _, derived, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_act_1: _, _, derived;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: _, derived, _;
    future_ind_act_3: derived, _, derived;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, _;
    future_opt_act_3: derived, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_fut: derived, _, _;
    infinitive_perf: _, _, derived;
    ),
    // Ἀγωνιζομαι: 1 words (ἀγωνιζομαι). Like Λλῡμῐ, except in 52 cells: ind_act_1 sg, ind_act_3 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ….
    conjugation!("ἀγωνιζομαι", 1, "ἀ~ιζομαι", like Λλῡμῐ;
    stem rest__: "~ῐ- like ᾰντᾰποκρῑνομαι";
    stem rest_: "~ῐσ- like ᾰντᾰποκρῑνομαι";
    ind_act_1: "ἀ~ιζομαι", _, _;
    ind_act_3: derived, _, _;
    ind_mid_1: "ἀ~ῐζομαι", _, "ἀ~ῐζομεθᾰ";
    ind_mid_2: "ἀ~ῐζῃ|ἀ~ῐζει", _, _;
    ind_mid_3: _, "ἀ~ῐζεσθον", _;
    subj_mid_1: "ἀ~ῐζωμαι", _, "ἀ~ῐζωμεθᾰ";
    subj_mid_2: "ἀ~ῐζῃ", "ἀ~ῐζησθον", _;
    subj_mid_3: "ἀ~ῐζηται", _, "ἀ~ῐζωνται";
    opt_mid_1: "ἀ~ῐζοιμην", _, "ἀ~ῐζοιμεθᾰ";
    opt_mid_2: "ἀ~ῐζοιο", "ἀ~ῐζοισθον", "ἀ~ῐζοισθε";
    opt_mid_3: "ἀ~ῐζοιτο", "ἀ~ῐζοισθην", "ἀ~ῐζοιντο";
    imp_mid_2: "ἀ~ῐζου", "ἀ~ῐζεσθον", "ἀ~ῐζεσθε";
    imp_mid_3: "ἀ~ῐζεσθω", _, "ἀ~ῐζεσθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    imperfect_mid_1: "ἠ~ῐζομην", _, "ἠ~ῐζομεθᾰ";
    imperfect_mid_2: "ἠ~ῐζου", "ἠ~ῐζεσθον", _;
    imperfect_mid_3: "ἠ~ῐζετο", _, "ἠ~ῐζοντο";
    future_ind_mid_3: _, derived, _;
    aorist_ind_mid_1: "ἠ~ῐσᾰμην", _, _;
    aorist_ind_mid_2: "ἠ~ῐσω", "ἠ~ῐσᾰσθον", _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_1: "ἠ~ῐσμαι", _, _;
    perfect_ind_mid_2: "ἠ~ῐσαι", "ἠ~ῐσθον", _;
    perfect_ind_mid_3: "ἠ~ῐσται", _, "ἠ~ῐδᾰται";
    perfect_subj_mid_3: _, "ἠ~ῐσμένω ἦτον", "ἠ~ῐσμένοι ὦσῐ|ἠ~ῐσμένοι ὦσῐν";
    perfect_imp_mid_2: _, _, "ἠ~ῐσθε";
    perfect_imp_mid_3: _, _, "ἠ~ῐσθων";
    pluperfect_mid_1: "ἠ~ῐσμην", _, _;
    pluperfect_mid_2: "ἠ~ῐσο", "ἠ~ῐσθον", _;
    pluperfect_mid_3: "ἠ~ῐστο", _, "ἠ~ῐδᾰτο";
    ),
    // Ἀπιστεω: 1 words (ἀπιστεω). Like Ἐκγᾰμῐζω, except in 30 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἀπιστεω", 1, "ἀ~ι~εω", like Ἐκγᾰμῐζω;
    stem lemma: "~ῐ~- like ᾰνθρωπεω";
    stem rest____: "~ῐ~- like ᾰνθρωπεω";
    stem rest__: "~ῐ~ησ- like ᾰνθρωπεω";
    stem rest_______: "~ῐ~ηθησ- like ᾰντᾰποκρῑνομαι";
    stem rest________: "~ῐ~ησ- like ἐλπῐζω";
    stem rest_: "~ῐ~ησ- like ᾰνθρωπεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    ),
    // Χᾰριζω: 1 words (χᾰριζω). Like Ἐφᾰγον, except in 3 cells: ind_act_1 sg, future_ind_act_3 du, aorist_subj_act_3 du.
    conjugation!("χᾰριζω", 1, "~ιζω", like Ἐφᾰγον;
    stem rest__: "~ῐ- like χᾰριζω";
    stem rest________: "~ῐσ- like χᾰριζω";
    stem rest_: "~ῐσ- like ξυνναυμᾰχεω";
    ind_act_1: "~ιζω", _, _;
    future_ind_act_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    ),
    // Διαφορεω: 1 words (διαφορεω). Like Δῐοικεω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("διαφορεω", 1, "~ια~εω", like Δῐοικεω;
    stem lemma: "~ῐᾰ~- like ποιεω";
    stem rest____: "~ῐε~- like εὐλογεω";
    stem rest__: "~ῐᾰ~ησ- like ποιεω";
    stem rest_______: "~ῐᾰ~ηθησ- like ποιεω";
    stem rest________: "~ῐε~ησ- like εὐλογεω";
    stem rest__________: "~ῐε~ηθην- like ἐλπω";
    stem rest_: "~ῐᾰ~ησ- like ποιεω";
    stem rest_____: "~ῐᾰ~ηθ- like ποιεω";
    stem rest___: "~ῐᾰ%ε~ηκ- like εὐλογεω";
    stem rest: "~ῐᾰ%ε~η- like εὐλογεω";
    stem rest______: "~ῐᾰ%ε~η- like εὐλογεω";
    stem rest___________: "~ῐε%ε~ηκ- like εὐλογεω";
    stem rest_________: "~ῐε%ε~η- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Τελεθω: 1 words (τελεθω). Like Λλῡμῐ, except in 5 cells: ind_act_3 du, subj_mid_3 du, imp_act_2 du, imperfect_act_1 sg, imperfect_act_3 sg.
    conjugation!("τελεθω", 1, "~ω", like Λλῡμῐ;
    stem lemma: "~εσκ- like τελεθω";
    ind_act_3: _, derived, _;
    subj_mid_3: _, derived, _;
    imp_act_2: _, derived, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    ),
    // Δῐᾰσευομαι: 1 words (δῐᾰσευομαι). Like Κορινθιαζομαι, except in 45 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("δῐᾰσευομαι", 1, "~ᾰσ~υομαι", like Κορινθιαζομαι;
    stem rest____: "~~σευ- like εἰσερχομαι";
    stem rest: "~~σσῠ- like ὑποκοριζομαι";
    stem rest______: "~~σσῠ- like δῐᾰσευομαι";
    ind_act_1: "~ᾰσ~υομαι", _, _;
    ind_mid_1: "~ασ~υομαι", _, "~ασ~υομεθᾰ";
    ind_mid_2: "~ασ~υῃ|~ασ~υει", _, _;
    ind_mid_3: _, "~ασ~υεσθον", _;
    ind_pass_2: _, "~ασ~υεσθον", "~ασ~υεσθε";
    ind_pass_3: "~ασ~υεται", "~ασ~υεσθον", "~ασ~υονται";
    subj_mid_1: "~ασ~υωμαι", _, "~ασ~υωμεθᾰ";
    subj_mid_2: "~ασ~υῃ", "~ασ~υησθον", _;
    subj_mid_3: "~ασ~υηται", _, "~ασ~υωνται";
    subj_pass_2: "~ασ~υῃ", _, "~ασ~υησθε";
    opt_mid_1: "~ασ~υοιμην", _, "~ασ~υοιμεθᾰ";
    opt_mid_2: "~ασ~υοιο", "~ασ~υοισθον", "~ασ~υοισθε";
    opt_mid_3: "~ασ~υοιτο", "~ασ~υοισθην", "~ασ~υοιντο";
    imp_mid_2: "~ασ~υου", "~ασ~υεσθον", "~ασ~υεσθε";
    imp_mid_3: "~ασ~υεσθω", _, "~ασ~υεσθων";
    imp_pass_2: _, "~ασ~υεσθον", "~ασ~υεσθε";
    imp_pass_3: _, "~ασ~υεσθων", "~ασ~υεσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    perfect_ind_mid_3: _, derived, _;
    perfect_imp_mid_2: _, derived, _;
    infinitive_pres: _, _, "~ασ~υεσθαι";
    participle_pres: _, _, "~ασ~υομενος";
    ),
    // Φθιω: 1 words (φθιω). Like Πῠρσαινω, except in 105 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("φθιω", 1, "~ιω", like Πῠρσαινω;
    stem lemma: "~ῐν- like βουλευω";
    stem rest____: "~ῐν- like βουλευω";
    stem rest________: "~- like φθιω";
    stem rest__________: "~ῐθην- like ἐχω";
    stem rest_____: "~ῐθ- like ποιεω";
    stem rest___: "~ῐνηκ- like σκοπεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_1: "~ῑσω|~εισω|~ῐνησω", _, "~ῑσομεν|~εισομεν|~ῐνησομεν";
    future_ind_act_2: "~ῑσεις|~εισεις|~ῐνησεις", "~ῑσετον|~εισετον|~ῐνησετον", "~ῑσετε|~εισετε|~ῐνησετε";
    future_ind_act_3: "~ῑσει|~εισει|~ῐνησει", _, "~ῑσουσῐ|~ῑσουσῐν|~εισουσῐ|~εισουσῐν|~ῐνησουσῐ|~ῐνησουσῐν";
    future_ind_mid_1: "~ῑσομαι|~εισομαι|~ῐνησομαι", _, "~ῑσομεθᾰ|~εισομεθᾰ|~ῐνησομεθᾰ";
    future_ind_mid_2: "~ῑσῃ|~ῑσει|~εισῃ|~εισει|~ῐνησῃ|~ῐνησει", "~ῑσεσθον|~εισεσθον|~ῐνησεσθον", "~ῑσεσθε|~εισεσθε|~ῐνησεσθε";
    future_ind_mid_3: "~ῑσεται|~εισεται|~ῐνησεται", _, "~ῑσονται|~εισονται|~ῐνησονται";
    future_opt_act_1: "~ῑσοιμῐ|~εισοιμῐ|~ῐνησοιμῐ", _, "~ῑσοιμεν|~εισοιμεν|~ῐνησοιμεν";
    future_opt_act_2: "~ῑσοις|~εισοις|~ῐνησοις", "~ῑσοιτον|~εισοιτον|~ῐνησοιτον", "~ῑσοιτε|~εισοιτε|~ῐνησοιτε";
    future_opt_act_3: "~ῑσοι|~εισοι|~ῐνησοι", "~ῑσοιτην|~εισοιτην|~ῐνησοιτην", "~ῑσοιεν|~εισοιεν|~ῐνησοιεν";
    future_opt_mid_1: "~ῑσοιμην|~εισοιμην|~ῐνησοιμην", _, "~ῑσοιμεθᾰ|~εισοιμεθᾰ|~ῐνησοιμεθᾰ";
    future_opt_mid_2: "~ῑσοιο|~εισοιο|~ῐνησοιο", "~ῑσοισθον|~εισοισθον|~ῐνησοισθον", "~ῑσοισθε|~εισοισθε|~ῐνησοισθε";
    future_opt_mid_3: "~ῑσοιτο|~εισοιτο|~ῐνησοιτο", "~ῑσοισθην|~εισοισθην|~ῐνησοισθην", "~ῑσοιντο|~εισοιντο|~ῐνησοιντο";
    aorist_subj_act_1: "~ῑσω|~εισω|~ῐνησω", _, "~ῑσωμεν|~εισωμεν|~ῐνησωμεν";
    aorist_subj_act_2: "~ῑσῃς|~εισῃς|~ῐνησῃς", "~ῑσητον|~εισητον|~ῐνησητον", "~ῑσητε|~εισητε|~ῐνησητε";
    aorist_subj_act_3: "~ῑσῃ|~εισῃ|~ῐνησῃ", _, "~ῑσωσῐ|~ῑσωσῐν|~εισωσῐ|~εισωσῐν|~ῐνησωσῐ|~ῐνησωσῐν";
    aorist_subj_mid_1: "~ῑσωμαι|~εισωμαι|~ῐνησωμαι", _, "~ῑσωμεθᾰ|~εισωμεθᾰ|~ῐνησωμεθᾰ";
    aorist_subj_mid_2: _, "~ῑσησθον|~εισησθον|~ῐνησησθον", "~ῑσησθε|~εισησθε|~ῐνησησθε";
    aorist_subj_mid_3: "~ῑσηται|~εισηται|~ῐνησηται", _, "~ῑσωνται|~εισωνται|~ῐνησωνται";
    aorist_opt_act_1: "~ῑσαιμῐ|~εισαιμῐ|~ῐνησαιμῐ", _, "~ῑσαιμεν|~εισαιμεν|~ῐνησαιμεν";
    aorist_opt_act_2: "~ῑσειᾰς|~ῑσαις|~εισειᾰς|~εισαις|~ῐνησειᾰς|~ῐνησαις", "~ῑσαιτον|~εισαιτον|~ῐνησαιτον", "~ῑσαιτε|~εισαιτε|~ῐνησαιτε";
    aorist_opt_act_3: "~ῑσειε|~ῑσειεν|~ῑσαι|~εισειε|~εισειεν|~εισαι|~ῐνησειε|~ῐνησειεν|~ῐνησαι", "~ῑσαιτην|~εισαιτην|~ῐνησαιτην", "~ῑσειᾰν|~ῑσαιεν|~εισειᾰν|~εισαιεν|~ῐνησειᾰν|~ῐνησαιεν";
    aorist_opt_mid_1: "~ῑσαιμην|~εισαιμην|~ῐνησαιμην", _, "~ῑσαιμεθᾰ|~εισαιμεθᾰ|~ῐνησαιμεθᾰ";
    aorist_opt_mid_2: "~ῑσαιο|~εισαιο|~ῐνησαιο", "~ῑσαισθον|~εισαισθον|~ῐνησαισθον", "~ῑσαισθε|~εισαισθε|~ῐνησαισθε";
    aorist_opt_mid_3: "~ῑσαιτο|~εισαιτο|~ῐνησαιτο", "~ῑσαισθην|~εισαισθην|~ῐνησαισθην", "~ῑσαιντο|~εισαιντο|~ῐνησαιντο";
    aorist_imp_act_2: "~ῑσον|~εισον|~ῐνησον", "~ῑσᾰτον|~εισᾰτον|~ῐνησᾰτον", "~ῑσᾰτε|~εισᾰτε|~ῐνησᾰτε";
    aorist_imp_act_3: "~ῑσᾰτω|~εισᾰτω|~ῐνησᾰτω", "~ῑσᾰτων|~εισᾰτων|~ῐνησᾰτων", "~ῑσᾰντων|~εισᾰντων|~ῐνησᾰντων";
    aorist_imp_mid_2: "~ῑσαι|~εισαι|~ῐνησαι", "~ῑσᾰσθον|~εισᾰσθον|~ῐνησᾰσθον", "~ῑσᾰσθε|~εισᾰσθε|~ῐνησᾰσθε";
    aorist_imp_mid_3: "~ῑσᾰσθω|~εισᾰσθω|~ῐνησᾰσθω", "~ῑσᾰσθων|~εισᾰσθων|~ῐνησᾰσθων", _;
    perfect_ind_act_3: _, derived, _;
    infinitive_fut: "~ῑσειν|~εισειν|~ῐνησειν", "~ῑσεσθαι|~εισεσθαι|~ῐνησεσθαι", _;
    infinitive_aor: "~ῑσαι|~εισαι|~ῐνησαι", "~ῑσᾰσθαι|~εισᾰσθαι|~ῐνησᾰσθαι", _;
    participle_fut: "~ῑσων|~εισων|~ῐνησων", "~ῑσομενος|~εισομενος|~ῐνησομενος", _;
    participle_aor: "~ῑσᾱς|~εισᾱς|~ῐνησᾱς", "~ῑσᾰμενος|~εισᾰμενος|~ῐνησᾰμενος", _;
    ),
    // Οικειοομαι: 1 words (οικειοομαι). Like Μᾰρνᾰμαι, except in 45 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("οικειοομαι", 1, "οι~οομαι", like Μᾰρνᾰμαι;
    ind_act_1: "οι~οομαι", _, _;
    ind_mid_1: "οἰ~ουμαι", _, "οἰ~ουμεθᾰ";
    ind_mid_2: "οἰ~οι", _, _;
    ind_mid_3: _, "οἰ~ουσθον", _;
    ind_pass_2: _, "οἰ~ουσθον", "οἰ~ουσθε";
    ind_pass_3: "οἰ~ουται", "οἰ~ουσθον", "οἰ~ουνται";
    subj_mid_1: "οἰ~ωμαι", _, "οἰ~ωμεθᾰ";
    subj_mid_2: "οἰ~οι", "οἰ~ωσθον", _;
    subj_mid_3: "οἰ~ωται", _, "οἰ~ωνται";
    subj_pass_2: "οἰ~οι", _, "οἰ~ωσθε";
    opt_mid_1: "οἰ~οιμην", _, "οἰ~οιμεθᾰ";
    opt_mid_2: "οἰ~οιο", "οἰ~οισθον", "οἰ~οισθε";
    opt_mid_3: "οἰ~οιτο", "οἰ~οισθην", "οἰ~οιντο";
    imp_mid_2: "οἰ~ου", "οἰ~ουσθον", "οἰ~ουσθε";
    imp_mid_3: "οἰ~ουσθω", _, "οἰ~ουσθων";
    imp_pass_2: _, "οἰ~ουσθον", "οἰ~ουσθε";
    imp_pass_3: _, "οἰ~ουσθων", "οἰ~ουσθων";
    imperfect_mid_1: "ᾠ~ουμην", _, "ᾠ~ουμεθᾰ";
    imperfect_mid_2: "ᾠ~ου", "ᾠ~ουσθον", _;
    imperfect_mid_3: "ᾠ~ουτο", _, "ᾠ~ουντο";
    imperfect_pass_2: _, _, "ᾠ~ουσθε";
    imperfect_pass_3: _, "ᾠ~ουσθην", _;
    infinitive_pres: _, _, "οἰ~ουσθαι";
    participle_pres: _, _, "οἰ~ουμενος";
    ),
    // Συγκᾰκουργεω: 1 words (συγκᾰκουργεω). Like Ξυμπολεμεω, except in 49 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_act_2 du, subj_mid_2 sg, subj_mid_3 du, ….
    conjugation!("συγκᾰκουργεω", 1, "~υγ~εω", like Ξυμπολεμεω;
    stem lemma: "~ῠγ~- like ποιεω";
    stem rest____: "~ῠνε~- like εὐλογεω";
    stem rest__: "~ῠγ~ησ- like ποιεω";
    stem rest_______: "~ῠγ~ηθησ- like ποιεω";
    stem rest_: "~ῠγ~ησ- like ποιεω";
    stem rest_____: "~ῠγ~ηθ- like ποιεω";
    stem rest___: "~ῠγ%ε~ηκ- like εὐλογεω";
    stem rest: "~ῠγ%ε~η- like εὐλογεω";
    stem rest______: "~ῠγ%ε~η- like εὐλογεω";
    stem rest___________: "~ῠγ%ε~ηκ- like σκοπεω";
    stem rest_________: "~ῠγ%ε~η- like σκοπεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_act_2: _, derived, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_act_1: _, _, derived;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ἀποστατεω: 1 words (ἀποστατεω). Like Ξυνναυμᾰχεω, except in 35 cells: ind_act_3 du, subj_act_3 du, imp_act_2 du, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 du, ….
    conjugation!("ἀποστατεω", 1, "ἀ~ο~α~εω", like Ξυνναυμᾰχεω;
    stem lemma: "~ο~ᾰ~- like ἀποστατεω";
    stem rest____: "~ε~ᾰ~- like ἀποστατεω";
    ind_act_3: _, derived, _;
    subj_act_3: _, derived, _;
    imp_act_2: _, derived, _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: _, derived, _;
    future_ind_act_3: derived, _, derived;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, _;
    future_opt_act_3: derived, derived, _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    participle_aor: derived, _, _;
    ),
    // Γεγονοι: 1 words (γεγονοι). Like Κᾰθημαι, except in 21 cells: ind_act_1 sg, perfect_ind_act_3 du, perfect_ind_mid_3 du, perfect_ind_pass_1 sg, perfect_ind_pass_2 sg, perfect_ind_pass_2 du, ….
    conjugation!("γεγονοι", 1, "~ο~οι", like Κᾰθημαι;
    stem rest___: "~ο~- like εὐλογεω";
    stem rest: "~ε~η- like εὐλογεω";
    stem rest______: "~ε~η- like εὐλογεω";
    ind_act_1: "~ο~οι", _, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_perf: _, _, derived;
    ),
    // Γεγενησαι: 1 words (γεγενησαι). Like Γεγονοι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησαι", 1, "~ε~ησαι", like Γεγονοι;
    ind_act_1: "~ε~ησαι", _, _;
    ),
    // Γεγενηται: 1 words (γεγενηται). Like Γεγονοι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενηται", 1, "~ε~ηται", like Γεγονοι;
    ind_act_1: "~ε~ηται", _, _;
    ),
    // Γεγενησθω: 1 words (γεγενησθω). Like Γεγονοι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησθω", 1, "~ε~ησθω", like Γεγονοι;
    ind_act_1: "~ε~ησθω", _, _;
    ),
    // Γεγενησο: 1 words (γεγενησο). Like Λλῡμῐ, except in 16 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, perfect_ind_act_3 du, perfect_ind_mid_3 du, ….
    conjugation!("γεγενησο", 1, "~ενησο", like Λλῡμῐ;
    stem rest___: "~ον- like εὐλογεω";
    stem rest: "~ενη- like εὐλογεω";
    stem rest______: "~ενη- like εὐλογεω";
    ind_act_1: "~ενησο", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_perf: _, _, derived;
    ),
    // Γεγενησθον: 1 words (γεγενησθον). Like Γεγονοι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησθον", 1, "~ε~ησθον", like Γεγονοι;
    ind_act_1: "~ε~ησθον", _, _;
    ),
    // Γεγενησθων: 1 words (γεγενησθων). Like Γεγενησθω with `` after each pattern (`ν` after ω), except in 1 cell: perfect_imp_pass_3 sg.
    conjugation!("γεγενησθων", 1, "~ε~ησθων", like Γεγενησθω with "", "ν" after "ω";
    perfect_imp_pass_3: "~ε~ησθω", _, _;
    ),
    // Γεγενηνται: 1 words (γεγενηνται). Like Γεγονοι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενηνται", 1, "~ε~ηνται", like Γεγονοι;
    ind_act_1: "~ε~ηνται", _, _;
    ),
    // Γεγενησθε: 1 words (γεγενησθε). Like Γεγονοι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησθε", 1, "~ε~ησθε", like Γεγονοι;
    ind_act_1: "~ε~ησθε", _, _;
    ),
    // Γεγενημεθα: 1 words (γεγενημεθα). Like Γεγονοι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενημεθα", 1, "~ε~ημεθα", like Γεγονοι;
    ind_act_1: "~ε~ημεθα", _, _;
    ),
    // Τεθνηκας: 1 words (τεθνηκας). Like Λλῡμῐ, except in 5 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, perfect_ind_act_3 du.
    conjugation!("τεθνηκας", 1, "~ας", like Λλῡμῐ;
    stem rest___: "~- like εὐλογεω";
    ind_act_1: "~ας", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    perfect_ind_act_3: _, derived, _;
    ),
    // Τεθνηκῃ: 1 words (τεθνηκῃ). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκῃ", 1, "~ῃ", like Τεθνηκας;
    ind_act_1: "~ῃ", _, _;
    ),
    // Τεθνηκοι: 1 words (τεθνηκοι). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοι", 1, "~οι", like Τεθνηκας;
    ind_act_1: "~οι", _, _;
    ),
    // Τεθνηκοιη: 1 words (τεθνηκοιη). Like Τεθνηκοι with `` after each pattern (`η` after ι), except in 1 cell: perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιη", 1, "~οιη", like Τεθνηκοι with "", "η" after "ι";
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκοιμι: 1 words (τεθνηκοιμι). Like Τεθνηκοι with `` after each pattern (`μι` after ι), except in 1 cell: perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιμι", 1, "~οιμι", like Τεθνηκοι with "", "μι" after "ι";
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκοιην: 1 words (τεθνηκοιην). Like Τεθνηκοι with `` after each pattern (`ην` after ι), except in 1 cell: perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιην", 1, "~οιην", like Τεθνηκοι with "", "ην" after "ι";
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκοις: 1 words (τεθνηκοις). Like Τεθνηκοι with `` after each pattern (`ς` after ι), except in 1 cell: perfect_opt_act_3 sg.
    conjugation!("τεθνηκοις", 1, "~οις", like Τεθνηκοι with "", "ς" after "ι";
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκοιης: 1 words (τεθνηκοιης). Like Τεθνηκοι with `` after each pattern (`ης` after ι), except in 1 cell: perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιης", 1, "~οιης", like Τεθνηκοι with "", "ης" after "ι";
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκῃς: 1 words (τεθνηκῃς). Like Τεθνηκῃ with `` after each pattern (`ς` after ῃ).
    conjugation!("τεθνηκῃς", 1, "~ῃς", like Τεθνηκῃ with "", "ς" after "ῃ";
    ),
    // Τεθνηκετω: 1 words (τεθνηκετω). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκετω", 1, "~ετω", like Τεθνηκας;
    ind_act_1: "~ετω", _, _;
    ),
    // Τεθνηκαμεν: 1 words (τεθνηκαμεν). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκαμεν", 1, "~αμεν", like Τεθνηκας;
    ind_act_1: "~αμεν", _, _;
    ),
    // Τεθνηκωμεν: 1 words (τεθνηκωμεν). Like Ἐδω with `μεν` after each pattern, except in 82 cells: future_ind_mid_1 sg, future_ind_mid_2 sg, future_ind_mid_2 du, future_ind_mid_3 sg, future_opt_mid_1 sg, future_opt_mid_3 sg, ….
    conjugation!("τεθνηκωμεν", 1, "~ωμεν", like Ἐδω with "μεν";
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, _;
    future_opt_mid_3: derived, derived, _;
    perfect_ind_act_1: derived, _, "~ᾰμεν";
    perfect_ind_act_2: derived, "~ᾰτον", derived;
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_ind_mid_1: derived, _, _;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, "~ητον", "~ωσῐ|~ωσῐν";
    perfect_subj_mid_3: _, derived, derived;
    perfect_subj_pass_1: derived, _, derived;
    perfect_subj_pass_2: derived, derived, derived;
    perfect_subj_pass_3: derived, derived, _;
    perfect_opt_act_1: "~οιμῐ|~οιην", _, "~οιμεν";
    perfect_opt_act_2: "~οις|~οιης", "~οιτον", "~οιτε";
    perfect_opt_act_3: "~οι|~οιη", "~οιτην", "~οιεν";
    perfect_opt_pass_1: derived, _, derived;
    perfect_opt_pass_2: derived, derived, derived;
    perfect_opt_pass_3: derived, derived, derived;
    perfect_imp_act_2: "~ε", "~ετον", derived;
    perfect_imp_act_3: derived, derived, "~οντων";
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    pluperfect_act_1: derived, _, _;
    pluperfect_act_2: derived, _, _;
    pluperfect_act_3: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: derived, derived, _;
    participle_perf: derived, _, derived;
    ),
    // Τεθνηκοιμεν: 1 words (τεθνηκοιμεν). Like Τεθνηκοι with `` after each pattern (`μεν` after ι), except in 1 cell: perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιμεν", 1, "~οιμεν", like Τεθνηκοι with "", "μεν" after "ι";
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκατε: 1 words (τεθνηκατε). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκατε", 1, "~ατε", like Τεθνηκας;
    ind_act_1: "~ατε", _, _;
    ),
    // Τεθνηκητε: 1 words (τεθνηκητε). Like Χρη with `τε` after each pattern, except in 27 cells: ind_act_3 sg, subj_act_3 sg, opt_act_3 sg, imp_act_3 sg, future_ind_act_3 sg, future_ind_mid_3 sg, ….
    conjugation!("τεθνηκητε", 1, "~ητε", like Χρη with "τε";
    ind_act_3: derived, _, _;
    subj_act_3: derived, _, _;
    opt_act_3: derived, _, _;
    imp_act_3: derived, _, _;
    future_ind_act_3: derived, _, _;
    future_ind_mid_3: derived, _, _;
    future_opt_act_3: derived, _, _;
    future_opt_mid_3: derived, _, _;
    perfect_ind_act_1: _, _, "~ᾰμεν";
    perfect_ind_act_2: _, "~ᾰτον", _;
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, "~ητον", "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, "~οιμεν";
    perfect_opt_act_2: "~οις|~οιης", "~οιτον", "~οιτε";
    perfect_opt_act_3: "~οι|~οιη", "~οιτην", "~οιεν";
    perfect_imp_act_2: "~ε", "~ετον", _;
    perfect_imp_act_3: _, _, "~οντων";
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    ),
    // Τεθνηκοιτε: 1 words (τεθνηκοιτε). Like Τεθνηκοι with `` after each pattern (`τε` after ι), except in 1 cell: perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιτε", 1, "~οιτε", like Τεθνηκοι with "", "τε" after "ι";
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκετε: 1 words (τεθνηκετε). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκετε", 1, "~ετε", like Τεθνηκας;
    ind_act_1: "~ετε", _, _;
    ),
    // Τεθνηκασι: 1 words (τεθνηκασι). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκασι", 1, "~ασι", like Τεθνηκας;
    ind_act_1: "~ασι", _, _;
    ),
    // Τεθνηκασιν: 1 words (τεθνηκασιν). Like Τεθνηκασι with `` after each pattern (`ν` after ι), except in 1 cell: perfect_opt_act_3 sg.
    conjugation!("τεθνηκασιν", 1, "~ασιν", like Τεθνηκασι with "", "ν" after "ι";
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκωσι: 1 words (τεθνηκωσι). Like Ἐδω with `σι` after each pattern, except in 82 cells: future_ind_mid_1 sg, future_ind_mid_2 sg, future_ind_mid_2 du, future_ind_mid_3 sg, future_opt_mid_1 sg, future_opt_mid_3 sg, ….
    conjugation!("τεθνηκωσι", 1, "~ωσι", like Ἐδω with "σι";
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, _;
    future_opt_mid_3: derived, derived, _;
    perfect_ind_act_1: derived, _, "~ᾰμεν";
    perfect_ind_act_2: derived, "~ᾰτον", derived;
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_ind_mid_1: derived, _, _;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, "~ητον", "~ωσῐ|~ωσῐν";
    perfect_subj_mid_3: _, derived, derived;
    perfect_subj_pass_1: derived, _, derived;
    perfect_subj_pass_2: derived, derived, derived;
    perfect_subj_pass_3: derived, derived, _;
    perfect_opt_act_1: "~οιμῐ|~οιην", _, "~οιμεν";
    perfect_opt_act_2: "~οις|~οιης", "~οιτον", "~οιτε";
    perfect_opt_act_3: "~οι|~οιη", "~οιτην", "~οιεν";
    perfect_opt_pass_1: derived, _, derived;
    perfect_opt_pass_2: derived, derived, derived;
    perfect_opt_pass_3: derived, derived, derived;
    perfect_imp_act_2: "~ε", "~ετον", derived;
    perfect_imp_act_3: derived, derived, "~οντων";
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    pluperfect_act_1: derived, _, _;
    pluperfect_act_2: derived, _, _;
    pluperfect_act_3: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: derived, derived, _;
    participle_perf: derived, _, derived;
    ),
    // Τεθνηκωσιν: 1 words (τεθνηκωσιν). Like Ἐδω with `σιν` after each pattern, except in 82 cells: future_ind_mid_1 sg, future_ind_mid_2 sg, future_ind_mid_2 du, future_ind_mid_3 sg, future_opt_mid_1 sg, future_opt_mid_3 sg, ….
    conjugation!("τεθνηκωσιν", 1, "~ωσιν", like Ἐδω with "σιν";
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, _;
    future_opt_mid_3: derived, derived, _;
    perfect_ind_act_1: derived, _, "~ᾰμεν";
    perfect_ind_act_2: derived, "~ᾰτον", derived;
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_ind_mid_1: derived, _, _;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, "~ητον", "~ωσῐ|~ωσῐν";
    perfect_subj_mid_3: _, derived, derived;
    perfect_subj_pass_1: derived, _, derived;
    perfect_subj_pass_2: derived, derived, derived;
    perfect_subj_pass_3: derived, derived, _;
    perfect_opt_act_1: "~οιμῐ|~οιην", _, "~οιμεν";
    perfect_opt_act_2: "~οις|~οιης", "~οιτον", "~οιτε";
    perfect_opt_act_3: "~οι|~οιη", "~οιτην", "~οιεν";
    perfect_opt_pass_1: derived, _, derived;
    perfect_opt_pass_2: derived, derived, derived;
    perfect_opt_pass_3: derived, derived, derived;
    perfect_imp_act_2: "~ε", "~ετον", derived;
    perfect_imp_act_3: derived, derived, "~οντων";
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    pluperfect_act_1: derived, _, _;
    pluperfect_act_2: derived, _, _;
    pluperfect_act_3: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: derived, derived, _;
    participle_perf: derived, _, derived;
    ),
    // Τεθνηκοιεν: 1 words (τεθνηκοιεν). Like Τεθνηκοι with `` after each pattern (`εν` after ι), except in 1 cell: perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιεν", 1, "~οιεν", like Τεθνηκοι with "", "εν" after "ι";
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκοντων: 1 words (τεθνηκοντων). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοντων", 1, "~οντων", like Τεθνηκας;
    ind_act_1: "~οντων", _, _;
    ),
    // Τεθνηκατον: 1 words (τεθνηκατον). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκατον", 1, "~ατον", like Τεθνηκας;
    ind_act_1: "~ατον", _, _;
    ),
    // Τεθνηκητον: 1 words (τεθνηκητον). Like Χρη with `τον` after each pattern, except in 27 cells: ind_act_3 sg, subj_act_3 sg, opt_act_3 sg, imp_act_3 sg, future_ind_act_3 sg, future_ind_mid_3 sg, ….
    conjugation!("τεθνηκητον", 1, "~ητον", like Χρη with "τον";
    ind_act_3: derived, _, _;
    subj_act_3: derived, _, _;
    opt_act_3: derived, _, _;
    imp_act_3: derived, _, _;
    future_ind_act_3: derived, _, _;
    future_ind_mid_3: derived, _, _;
    future_opt_act_3: derived, _, _;
    future_opt_mid_3: derived, _, _;
    perfect_ind_act_1: _, _, "~ᾰμεν";
    perfect_ind_act_2: _, "~ᾰτον", _;
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, "~ητον", "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, "~οιμεν";
    perfect_opt_act_2: "~οις|~οιης", "~οιτον", "~οιτε";
    perfect_opt_act_3: "~οι|~οιη", "~οιτην", "~οιεν";
    perfect_imp_act_2: "~ε", "~ετον", _;
    perfect_imp_act_3: _, _, "~οντων";
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    ),
    // Τεθνηκοιτον: 1 words (τεθνηκοιτον). Like Τεθνηκοι with `` after each pattern (`τον` after ι), except in 1 cell: perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιτον", 1, "~οιτον", like Τεθνηκοι with "", "τον" after "ι";
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκοιτην: 1 words (τεθνηκοιτην). Like Τεθνηκοι with `` after each pattern (`την` after ι), except in 1 cell: perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιτην", 1, "~οιτην", like Τεθνηκοι with "", "την" after "ι";
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκετον: 1 words (τεθνηκετον). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκετον", 1, "~ετον", like Τεθνηκας;
    ind_act_1: "~ετον", _, _;
    ),
    // Τεθνηκετων: 1 words (τεθνηκετων). Like Τεθνηκετω with `` after each pattern (`ν` after ω).
    conjugation!("τεθνηκετων", 1, "~ετων", like Τεθνηκετω with "", "ν" after "ω";
    ),
    // Ἀντιπαρατασσομαι: 1 words (ἀντιπαρατασσομαι).
    conjugation!("ἀντιπαρατασσομαι", 1, "~ι~α~α~ασσομαι";
    stem rest____: "~ῐ~ᾰ~ε~ᾰσσ- like εἰσερχομαι";
    stem rest__: "~ῐ~ᾰ~ᾰ~ᾰξ- like εἰσερχομαι";
    stem rest_______: "~ῐ~ᾰ~ᾰ~ᾰχθησ- like ποιεω";
    stem rest________: "~ῐ~ᾰ~ε~ᾰξ- like μηκᾰομαι";
    stem rest__________: "~ῐ~ᾰ~ε~ᾰχθην- like ἐλπω";
    stem rest_: "~ῐ~ᾰ~ᾰ~ᾰξ- like ἑπομαι";
    stem rest_____: "~ῐ~ᾰ~ᾰ~ᾰχθ- like ποιεω";
    stem rest: "~ῐ~ᾰ~ᾰ~ετᾰ- like πᾰρᾰλλᾰσσω";
    stem rest______: "~ῐ~ᾰ~ᾰ~ετᾰ- like ἀνᾰγω";
    stem rest_________: "~ῐ~ᾰ~ε~ετᾰ- like ῥηγνῡμῐ";
    ind_act_1: "~ι~α~α~ασσομαι", _, _;
    ind_mid_1: "~ῐ~ᾰ~ᾰ~ᾰσσομαι", _, "~ῐ~ᾰ~ᾰ~ᾰσσομεθᾰ";
    ind_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσῃ|~ῐ~ᾰ~ᾰ~ᾰσσει", _, _;
    ind_mid_3: _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθον", _;
    ind_pass_2: _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθον", "~ῐ~ᾰ~ᾰ~ᾰσσεσθε";
    ind_pass_3: "~ῐ~ᾰ~ᾰ~ᾰσσεται", "~ῐ~ᾰ~ᾰ~ᾰσσεσθον", "~ῐ~ᾰ~ᾰ~ᾰσσονται";
    subj_mid_1: "~ῐ~ᾰ~ᾰ~ᾰσσωμαι", _, "~ῐ~ᾰ~ᾰ~ᾰσσωμεθᾰ";
    subj_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσῃ", "~ῐ~ᾰ~ᾰ~ᾰσσησθον", _;
    subj_mid_3: "~ῐ~ᾰ~ᾰ~ᾰσσηται", _, "~ῐ~ᾰ~ᾰ~ᾰσσωνται";
    subj_pass_2: "~ῐ~ᾰ~ᾰ~ᾰσσῃ", _, "~ῐ~ᾰ~ᾰ~ᾰσσησθε";
    opt_mid_1: "~ῐ~ᾰ~ᾰ~ᾰσσοιμην", _, "~ῐ~ᾰ~ᾰ~ᾰσσοιμεθᾰ";
    opt_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσοιο", "~ῐ~ᾰ~ᾰ~ᾰσσοισθον", "~ῐ~ᾰ~ᾰ~ᾰσσοισθε";
    opt_mid_3: "~ῐ~ᾰ~ᾰ~ᾰσσοιτο", "~ῐ~ᾰ~ᾰ~ᾰσσοισθην", "~ῐ~ᾰ~ᾰ~ᾰσσοιντο";
    imp_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσου", "~ῐ~ᾰ~ᾰ~ᾰσσεσθον", "~ῐ~ᾰ~ᾰ~ᾰσσεσθε";
    imp_mid_3: "~ῐ~ᾰ~ᾰ~ᾰσσεσθω", _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθων";
    imp_pass_2: _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθον", "~ῐ~ᾰ~ᾰ~ᾰσσεσθε";
    imp_pass_3: _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθων", "~ῐ~ᾰ~ᾰ~ᾰσσεσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_pres: _, _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθαι";
    infinitive_perf: _, _, derived;
    participle_pres: _, _, "~ῐ~ᾰ~ᾰ~ᾰσσομενος";
    ),
    // Ἐπᾰνερχομαι: 1 words (ἐπᾰνερχομαι). Like Κορινθιαζομαι, except in 38 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἐπᾰνερχομαι", 1, "~ᾰ~ρχομαι", like Κορινθιαζομαι;
    stem rest________: "~α~λθ- like εἰσερχομαι";
    stem rest_: "~α~λθ- like εἰσερχομαι";
    ind_act_1: "~ᾰ~ρχομαι", _, _;
    ind_mid_1: "~α~ρχομαι", _, "~α~ρχομεθᾰ";
    ind_mid_2: "~α~ρχῃ|~α~ρχει", _, _;
    ind_mid_3: _, "~α~ρχεσθον", _;
    ind_pass_2: _, "~α~ρχεσθον", "~α~ρχεσθε";
    ind_pass_3: "~α~ρχεται", "~α~ρχεσθον", "~α~ρχονται";
    subj_mid_1: "~α~ρχωμαι", _, "~α~ρχωμεθᾰ";
    subj_mid_2: "~α~ρχῃ", "~α~ρχησθον", _;
    subj_mid_3: "~α~ρχηται", _, "~α~ρχωνται";
    subj_pass_2: "~α~ρχῃ", _, "~α~ρχησθε";
    opt_mid_1: "~α~ρχοιμην", _, "~α~ρχοιμεθᾰ";
    opt_mid_2: "~α~ρχοιο", "~α~ρχοισθον", "~α~ρχοισθε";
    opt_mid_3: "~α~ρχοιτο", "~α~ρχοισθην", "~α~ρχοιντο";
    imp_mid_2: "~α~ρχου", "~α~ρχεσθον", "~α~ρχεσθε";
    imp_mid_3: "~α~ρχεσθω", _, "~α~ρχεσθων";
    imp_pass_2: _, "~α~ρχεσθον", "~α~ρχεσθε";
    imp_pass_3: _, "~α~ρχεσθων", "~α~ρχεσθων";
    aorist_subj_act_3: _, derived, _;
    infinitive_pres: _, _, "~α~ρχεσθαι";
    participle_pres: _, _, "~α~ρχομενος";
    ),
    // Κιδνᾰμαι: 1 words (κιδνᾰμαι). Like Κορινθιαζομαι, except in 37 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("κιδνᾰμαι", 1, "~ι~ᾰμαι", like Κορινθιαζομαι;
    ind_act_1: "~ι~ᾰμαι", _, _;
    ind_mid_1: "~ῐ~ᾰμαι", _, "~ῐ~ᾰμεθᾰ";
    ind_mid_2: "~ῐ~ᾰσαι", _, _;
    ind_mid_3: _, "~ῐ~ᾰσθον", _;
    ind_pass_2: _, "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    ind_pass_3: "~ῐ~ᾰται", "~ῐ~ᾰσθον", "~ῐ~ᾰνται";
    opt_mid_1: "~ῐ~αιμην", _, "~ῐ~αιμεθᾰ";
    opt_mid_2: "~ῐ~αιο", "~ῐ~αισθον", "~ῐ~αισθε";
    opt_mid_3: "~ῐ~αιτο", "~ῐ~αισθην", "~ῐ~αιντο";
    imp_mid_2: "~ῐ~ᾰσο|~ῐ~ω", "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    imp_mid_3: "~ῐ~ᾰσθω", _, "~ῐ~ᾰσθων";
    imp_pass_2: _, "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    imp_pass_3: _, "~ῐ~ᾰσθων", "~ῐ~ᾰσθων";
    imperfect_mid_1: "ἐ~ῐ~ᾰμην", _, "ἐ~ῐ~ᾰμεθᾰ";
    imperfect_mid_2: "ἐ~ῐ~ᾰσο", "ἐ~ῐ~ᾰσθον", _;
    imperfect_mid_3: "ἐ~ῐ~ᾰτο", _, "ἐ~ῐ~ᾰντο";
    imperfect_pass_2: _, _, "ἐ~ῐ~ᾰσθε";
    imperfect_pass_3: _, "ἐ~ῐ~ᾰσθην", _;
    infinitive_pres: _, _, "~ῐ~ᾰσθαι";
    participle_pres: _, _, "~ῐ~ᾰμενος";
    ),
    // Ἐπισκεπτομαι: 1 words (ἐπισκεπτομαι). Like Κᾰτακειμαι, except in 59 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἐπισκεπτομαι", 1, "~ι~πτομαι", like Κᾰτακειμαι;
    stem rest____: "~ε~πτ- like εἰσερχομαι";
    stem rest__: "~ῐ~ψ- like εἰσερχομαι";
    stem rest_______: "~ῐ~πησ- like ποιεω";
    stem rest________: "~ε~ψ- like μηκᾰομαι";
    stem rest__________: "~ε~- like ἐπισκεπτομαι";
    stem rest_: "~ῐ~ψ- like ἑπομαι";
    stem rest_____: "~ῐ~- like ἐπισκεπτομαι";
    stem rest: "~ε~- like ἀνᾰπτω";
    stem rest______: "~ε~- like ἀνᾰπτω";
    ind_act_1: "~ι~πτομαι", _, _;
    ind_mid_1: "~ῐ~πτομαι", _, "~ῐ~πτομεθᾰ";
    ind_mid_2: "~ῐ~πτῃ|~ῐ~πτει", _, _;
    ind_mid_3: _, "~ῐ~πτεσθον", _;
    ind_pass_2: _, "~ῐ~πτεσθον", "~ῐ~πτεσθε";
    ind_pass_3: "~ῐ~πτεται", "~ῐ~πτεσθον", "~ῐ~πτονται";
    subj_mid_1: "~ῐ~πτωμαι", _, "~ῐ~πτωμεθᾰ";
    subj_mid_2: "~ῐ~πτῃ", "~ῐ~πτησθον", _;
    subj_mid_3: "~ῐ~πτηται", _, "~ῐ~πτωνται";
    subj_pass_2: "~ῐ~πτῃ", _, "~ῐ~πτησθε";
    opt_mid_1: "~ῐ~πτοιμην", _, "~ῐ~πτοιμεθᾰ";
    opt_mid_2: "~ῐ~πτοιο", "~ῐ~πτοισθον", "~ῐ~πτοισθε";
    opt_mid_3: "~ῐ~πτοιτο", "~ῐ~πτοισθην", "~ῐ~πτοιντο";
    imp_mid_2: "~ῐ~πτου", "~ῐ~πτεσθον", "~ῐ~πτεσθε";
    imp_mid_3: "~ῐ~πτεσθω", _, "~ῐ~πτεσθων";
    imp_pass_2: _, "~ῐ~πτεσθον", "~ῐ~πτεσθε";
    imp_pass_3: _, "~ῐ~πτεσθων", "~ῐ~πτεσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    aorist_subj_pass_3: _, derived, _;
    aorist_imp_pass_2: _, derived, derived;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_pres: _, _, "~ῐ~πτεσθαι";
    infinitive_perf: _, _, derived;
    participle_pres: _, _, "~ῐ~πτομενος";
    ),
    // Κολυμβαω: 1 words (κολυμβαω). Like Μετεκῑᾰθον, except in 5 cells: ind_act_3 du, subj_act_3 du, imp_act_2 du, future_ind_act_3 du, aorist_subj_act_3 du.
    conjugation!("κολυμβαω", 1, "~υ~αω", like Μετεκῑᾰθον;
    stem lemma: "~ῠ~- like μειδᾰω";
    stem rest____: "~ῠ~- like μειδᾰω";
    stem rest__: "~ῠ~ησ- like ξυνναυμᾰχεω";
    stem rest________: "~ῠ~ησ- like χᾰριζω";
    stem rest_: "~ῠ~ησ- like ξυνναυμᾰχεω";
    ind_act_3: _, derived, _;
    subj_act_3: _, derived, _;
    imp_act_2: _, derived, _;
    future_ind_act_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    ),
    // Δῐκειν: 1 words (δῐκειν). Like Ἐφᾰγον, except in 2 cells: ind_act_1 sg, aorist_subj_act_3 du.
    conjugation!("δῐκειν", 1, "~ειν", like Ἐφᾰγον;
    stem rest________: "~- like ἐπεφνον";
    stem rest_: "~- like εἰσερχομαι";
    ind_act_1: "~ειν", _, _;
    aorist_subj_act_3: _, derived, _;
    ),
    // Ὑπονοεω: 1 words (ὑπονοεω). Like Δῐοικεω, except in 60 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ὑπονοεω", 1, "ὑ~ο~εω", like Δῐοικεω;
    stem lemma: "~ο~- like ὑπονοεω";
    stem rest__: "~ο~ησ- like ὑπονοεω";
    stem rest________: "~ε~ησ- like ὑπονοεω";
    stem rest_: "~ο~ησ- like ὑπονοεω";
    stem rest_____: "~ο~ηθ- like ὑπονοεω";
    stem rest___: "~ο%ε~ηκ- like ὑπονοεω";
    stem rest: "~ο%ε~η- like ὑπονοεω";
    stem rest______: "~ο%ε~η- like ὑπονοεω";
    stem rest___________: "~ε%ε~ηκ- like ὑπονοεω";
    stem rest_________: "~ε%ε~η- like ὑπονοεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_act_1: "ῠ~ε~ουν", _, _;
    imperfect_act_2: "ῠ~ε~εις", "ῠ~ε~ειτον", "ῠ~ε~ειτε";
    imperfect_act_3: "ῠ~ε~ει", _, "ῠ~ε~ουν";
    imperfect_mid_1: "ῠ~ε~ουμην", _, "ῠ~ε~ουμεθᾰ";
    imperfect_mid_2: "ῠ~ε~ου", _, _;
    imperfect_mid_3: "ῠ~ε~ειτο", _, "ῠ~ε~ουντο";
    imperfect_pass_2: _, "ῠ~ε~εισθον", "ῠ~ε~εισθε";
    imperfect_pass_3: _, "ῠ~ε~εισθην", _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_ind_pass_1: "ῠ~ο~ηθησομαι", _, _;
    future_ind_pass_2: "ῠ~ο~ηθησῃ|ῠ~ο~ηθησει", _, _;
    future_ind_pass_3: _, "ῠ~ο~ηθησεσθον", _;
    aorist_ind_pass_1: "ῠ~ε~ηθην", _, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ἀντᾰγωνιζομαι: 1 words (ἀντᾰγωνιζομαι). Like Κᾰτακειμαι, except in 61 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἀντᾰγωνιζομαι", 1, "~ᾰ~ιζομαι", like Κᾰτακειμαι;
    stem rest____: "~η~ῐζ- like εἰσερχομαι";
    stem rest__: "~ᾰ~ῐ- like ὑποκοριζομαι";
    stem rest_______: "~ᾰ~ῐσθησ- like ποιεω";
    stem rest________: "~η~ῐσ- like μηκᾰομαι";
    stem rest__________: "~η~ῐσθην- like ἐλπω";
    stem rest_: "~ᾰ~ῐσ- like ἑπομαι";
    stem rest_____: "~ᾰ~ῐσθ- like ποιεω";
    stem rest: "~η~ῐσ- like εὐλογεω";
    stem rest______: "~η~ῐ- like ἱζω";
    stem rest_________: "~η~ῐ- like ἱζω";
    ind_act_1: "~ᾰ~ιζομαι", _, _;
    ind_mid_1: "~ᾰ~ῐζομαι", _, "~ᾰ~ῐζομεθᾰ";
    ind_mid_2: "~ᾰ~ῐζῃ|~ᾰ~ῐζει", _, _;
    ind_mid_3: _, "~ᾰ~ῐζεσθον", _;
    ind_pass_2: _, "~ᾰ~ῐζεσθον", "~ᾰ~ῐζεσθε";
    ind_pass_3: "~ᾰ~ῐζεται", "~ᾰ~ῐζεσθον", "~ᾰ~ῐζονται";
    subj_mid_1: "~ᾰ~ῐζωμαι", _, "~ᾰ~ῐζωμεθᾰ";
    subj_mid_2: "~ᾰ~ῐζῃ", "~ᾰ~ῐζησθον", _;
    subj_mid_3: "~ᾰ~ῐζηται", _, "~ᾰ~ῐζωνται";
    subj_pass_2: "~ᾰ~ῐζῃ", _, "~ᾰ~ῐζησθε";
    opt_mid_1: "~ᾰ~ῐζοιμην", _, "~ᾰ~ῐζοιμεθᾰ";
    opt_mid_2: "~ᾰ~ῐζοιο", "~ᾰ~ῐζοισθον", "~ᾰ~ῐζοισθε";
    opt_mid_3: "~ᾰ~ῐζοιτο", "~ᾰ~ῐζοισθην", "~ᾰ~ῐζοιντο";
    imp_mid_2: "~ᾰ~ῐζου", "~ᾰ~ῐζεσθον", "~ᾰ~ῐζεσθε";
    imp_mid_3: "~ᾰ~ῐζεσθω", _, "~ᾰ~ῐζεσθων";
    imp_pass_2: _, "~ᾰ~ῐζεσθον", "~ᾰ~ῐζεσθε";
    imp_pass_3: _, "~ᾰ~ῐζεσθων", "~ᾰ~ῐζεσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_pres: _, _, "~ᾰ~ῐζεσθαι";
    infinitive_perf: _, _, derived;
    participle_pres: _, _, "~ᾰ~ῐζομενος";
    ),
    // Διπλᾰσῐοομαι: 1 words (διπλᾰσῐοομαι). Like Κορινθιαζομαι, except in 25 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("διπλᾰσῐοομαι", 1, "~ι~οομαι", like Κορινθιαζομαι;
    ind_act_1: "~ι~οομαι", _, _;
    ind_mid_1: "~ῐ~ουμαι", _, "~ῐ~ουμεθᾰ";
    ind_mid_2: "~ῐ~οι", _, _;
    ind_mid_3: _, "~ῐ~ουσθον", _;
    ind_pass_2: _, "~ῐ~ουσθον", "~ῐ~ουσθε";
    ind_pass_3: "~ῐ~ουται", "~ῐ~ουσθον", "~ῐ~ουνται";
    subj_mid_2: "~ῐ~οι", "~ῐ~ωσθον", _;
    subj_mid_3: "~ῐ~ωται", _, _;
    subj_pass_2: "~ῐ~οι", _, "~ῐ~ωσθε";
    imp_mid_2: _, "~ῐ~ουσθον", "~ῐ~ουσθε";
    imp_mid_3: "~ῐ~ουσθω", _, "~ῐ~ουσθων";
    imp_pass_2: _, "~ῐ~ουσθον", "~ῐ~ουσθε";
    imp_pass_3: _, "~ῐ~ουσθων", "~ῐ~ουσθων";
    infinitive_pres: _, _, "~ῐ~ουσθαι";
    participle_pres: _, _, "~ῐ~ουμενος";
    ),
    // Ἐξικνεομαι: 1 words (ἐξικνεομαι). Like Μᾰρνᾰμαι, except in 54 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἐξικνεομαι", 1, "~ικνεομαι", like Μᾰρνᾰμαι;
    stem rest____: "~ῑκν- like ἡγεομαι";
    stem rest__: "~ῐξ- like εἰσερχομαι";
    stem rest________: "~ῑκ- like ὑπογῐγνομαι";
    stem rest_: "~ῐκ- like ὑπογῐγνομαι";
    ind_act_1: "~ικνεομαι", _, _;
    ind_mid_1: "~ῐκνουμαι", _, "~ῐκνουμεθᾰ";
    ind_mid_2: "~ῐκνει|~ῐκνῃ", _, _;
    ind_mid_3: _, "~ῐκνεισθον", _;
    ind_pass_2: _, "~ῐκνεισθον", "~ῐκνεισθε";
    ind_pass_3: "~ῐκνειται", "~ῐκνεισθον", "~ῐκνουνται";
    subj_mid_1: "~ῐκνωμαι", _, "~ῐκνωμεθᾰ";
    subj_mid_2: "~ῐκνῃ", "~ῐκνησθον", _;
    subj_mid_3: "~ῐκνηται", _, "~ῐκνωνται";
    subj_pass_2: "~ῐκνῃ", _, "~ῐκνησθε";
    opt_mid_1: "~ῐκνοιμην", _, "~ῐκνοιμεθᾰ";
    opt_mid_2: "~ῐκνοιο", "~ῐκνοισθον", "~ῐκνοισθε";
    opt_mid_3: "~ῐκνοιτο", "~ῐκνοισθην", "~ῐκνοιντο";
    imp_mid_2: "~ῐκνου", "~ῐκνεισθον", "~ῐκνεισθε";
    imp_mid_3: "~ῐκνεισθω", _, "~ῐκνεισθων";
    imp_pass_2: _, "~ῐκνεισθον", "~ῐκνεισθε";
    imp_pass_3: _, "~ῐκνεισθων", "~ῐκνεισθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_1: "~ῑγμαι", _, _;
    perfect_ind_mid_2: "~ῑξαι", "~ῑχθον", _;
    perfect_ind_mid_3: "~ῑκται", _, "~ῑγᾰται";
    perfect_subj_mid_3: _, "~ῑγμένω ἦτον", "~ῑγμένοι ὦσῐ|~ῑγμένοι ὦσῐν";
    perfect_imp_mid_2: _, _, "~ῑχθε";
    perfect_imp_mid_3: _, _, "~ῑχθων";
    infinitive_pres: _, _, "~ῐκνεισθαι";
    participle_pres: _, _, "~ῐκνουμενος";
    ),
    // Καταπετροω: 1 words (καταπετροω). Like Λαλαγεω, except in 47 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("καταπετροω", 1, "~α~α~οω", like Λαλαγεω;
    stem lemma: "~ᾰ~ᾰ~- like γῡροω";
    stem rest____: "~ᾰ~ε~ου- like κᾰκοω";
    stem rest__: "~ᾰ~ᾰ~ωσ- like ποιεω";
    stem rest_______: "~ᾰ~ᾰ~ωθησ- like ποιεω";
    stem rest________: "~ᾰ~ε~ωσ- like εὐλογεω";
    stem rest__________: "~ᾰ~ε~ωθην- like ἐλπω";
    stem rest_: "~ᾰ~ᾰ~ωσ- like ποιεω";
    stem rest_____: "~ᾰ~ᾰ~ωθ- like ποιεω";
    stem rest___: "~ᾰ~ε~ωκ- like εὐλογεω";
    stem rest: "~ᾰ~ε~ω- like εὐλογεω";
    stem rest______: "~ᾰ~ε~ω- like εὐλογεω";
    stem rest___________: "~ᾰ~ε~ωκ- like εὐλογεω";
    stem rest_________: "~ᾰ~ε~ω- like εὐλογεω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Λαλαχευομαι: 1 words (λαλαχευομαι). Like Ἀποστατεω, except in 62 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("λαλαχευομαι", 1, "~α~α~ομαι", like Ἀποστατεω;
    stem rest____: "~ᾰ~ᾰ~- like λαλαχευομαι";
    stem rest_______: "~ᾰ~ᾰ~θησ- like ποιεω";
    stem rest__________: "~ᾰ~ᾰ~θην- like ἐχω";
    stem rest_____: "~ᾰ~ᾰ~θ- like ποιεω";
    stem rest: "~ε~ᾰλᾰ~- like λαλαχευομαι";
    stem rest______: "~ε~ᾰλᾰ~- like λαλαχευομαι";
    stem rest_________: "~ε~ᾰλᾰ~- like λαλαχευομαι";
    ind_act_1: "~α~α~ομαι", _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: derived, _, derived;
    ind_pass_1: "~ᾰ~ᾰ~ομαι", _, "~ᾰ~ᾰ~ομεθᾰ";
    ind_pass_2: "~ᾰ~ᾰ~ῃ|~ᾰ~ᾰ~ει", "~ᾰ~ᾰ~εσθον", "~ᾰ~ᾰ~εσθε";
    ind_pass_3: "~ᾰ~ᾰ~εται", "~ᾰ~ᾰ~εσθον", "~ᾰ~ᾰ~ονται";
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, _;
    subj_act_3: derived, _, derived;
    subj_pass_1: "~ᾰ~ᾰ~ωμαι", _, "~ᾰ~ᾰ~ωμεθᾰ";
    subj_pass_2: "~ᾰ~ᾰ~ῃ", "~ᾰ~ᾰ~ησθον", "~ᾰ~ᾰ~ησθε";
    subj_pass_3: "~ᾰ~ᾰ~ηται", "~ᾰ~ᾰ~ησθον", "~ᾰ~ᾰ~ωνται";
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    opt_pass_1: "~ᾰ~ᾰ~οιμην", _, "~ᾰ~ᾰ~οιμεθᾰ";
    opt_pass_2: "~ᾰ~ᾰ~οιο", "~ᾰ~ᾰ~οισθον", "~ᾰ~ᾰ~οισθε";
    opt_pass_3: "~ᾰ~ᾰ~οιτο", "~ᾰ~ᾰ~οισθην", "~ᾰ~ᾰ~οιντο";
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imp_pass_2: "~ᾰ~ᾰ~ου", "~ᾰ~ᾰ~εσθον", "~ᾰ~ᾰ~εσθε";
    imp_pass_3: "~ᾰ~ᾰ~εσθω", "~ᾰ~ᾰ~εσθων", "~ᾰ~ᾰ~εσθων";
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    infinitive_pres: derived, _, _;
    participle_pres: derived, _, _;
    ),
    // Μικρογρᾰφεω: 1 words (μικρογρᾰφεω). Like Κορινθιαζομαι, except in 24 cells: ind_act_3 du, ind_mid_1 sg, ind_pass_1 pl, ind_pass_2 sg, subj_mid_1 sg, subj_mid_2 sg, ….
    conjugation!("μικρογρᾰφεω", 1, "~ι~εω", like Κορινθιαζομαι;
    stem lemma: "~ῐ~- like ποιεω";
    ind_act_3: _, derived, _;
    ind_mid_1: derived, _, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_1: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_mid_3: derived, _, _;
    imp_pass_2: derived, _, _;
    ),
    // Λῐνοστᾰτεω: 1 words (λῐνοστᾰτεω). Like Ἐπῐκᾰλεω, except in 28 cells: imperfect_pass_1 sg, imperfect_pass_1 pl, imperfect_pass_2 sg, imperfect_pass_3 sg, imperfect_pass_3 pl, future_ind_act_3 du, ….
    conjugation!("λῐνοστᾰτεω", 1, "~ῐ~ᾰ~εω", like Ἐπῐκᾰλεω;
    stem rest____: "~ι~α~- like ποιεω";
    stem rest__: "~ι~α~ησ- like ποιεω";
    stem rest_______: "~ι~α~ηθησ- like ποιεω";
    stem rest________: "~ι~α~ησ- like ποιεω";
    stem rest__________: "~ι~α~ηθην- like ἐχω";
    stem rest_: "~ι~α~ησ- like ποιεω";
    stem rest_____: "~ι~α~ηθ- like ποιεω";
    stem rest___: "~ελι~α~ηκ- like εὐλογεω";
    stem rest: "~ελι~α~η- like εὐλογεω";
    stem rest______: "~ελι~α~η- like εὐλογεω";
    stem rest___________: "~ελι~α~ηκ- like σκοπεω";
    stem rest_________: "~ελι~α~η- like σκοπεω";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    future_opt_act_2: _, _, derived;
    future_opt_act_3: _, _, derived;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    ),
    // Ἀδησᾰ: 1 words (ἀδησᾰ). Like Ἐφᾰγον, except in 3 cells: ind_act_1 sg, aorist_subj_act_3 du, perfect_ind_act_3 du.
    conjugation!("ἀδησᾰ", 1, "~σᾰ", like Ἐφᾰγον;
    stem rest________: "~σ- like ξυνναυμᾰχεω";
    stem rest_: "~σ- like ξυνναυμᾰχεω";
    stem rest___: "~κ- like εὐλογεω";
    ind_act_1: "~σᾰ", _, _;
    aorist_subj_act_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    ),
    // Ὑποκοριζομαι: 1 words (ὑποκοριζομαι). Like Κᾰτακειμαι, except in 31 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, subj_mid_1 sg, ….
    conjugation!("ὑποκοριζομαι", 1, "~ο~ιζομαι", like Κᾰτακειμαι;
    stem rest____: "~ε~ῐζ- like ὑποκοριζομαι";
    stem rest__: "~ο~ῐ- like ὑποκοριζομαι";
    stem rest________: "~ε~ῐσ- like μηκᾰομαι";
    stem rest_: "~ο~ῐσ- like ἑπομαι";
    stem rest: "~ο%ε~ῐσ- like ὑποκοριζομαι";
    stem rest_________: "~ε%ε~ῐ- like ὑποκοριζομαι";
    ind_act_1: "~ο~ιζομαι", _, _;
    ind_mid_1: "~ο~ῐζομαι", _, "~ο~ῐζομεθᾰ";
    ind_mid_2: "~ο~ῐζῃ|~ο~ῐζει", _, _;
    ind_mid_3: _, "~ο~ῐζεσθον", _;
    subj_mid_1: "~ο~ῐζωμαι", _, "~ο~ῐζωμεθᾰ";
    subj_mid_2: "~ο~ῐζῃ", "~ο~ῐζησθον", _;
    subj_mid_3: "~ο~ῐζηται", _, "~ο~ῐζωνται";
    opt_mid_1: "~ο~ῐζοιμην", _, "~ο~ῐζοιμεθᾰ";
    opt_mid_2: "~ο~ῐζοιο", "~ο~ῐζοισθον", "~ο~ῐζοισθε";
    opt_mid_3: "~ο~ῐζοιτο", "~ο~ῐζοισθην", "~ο~ῐζοιντο";
    imp_mid_2: "~ο~ῐζου", "~ο~ῐζεσθον", "~ο~ῐζεσθε";
    imp_mid_3: "~ο~ῐζεσθω", _, "~ο~ῐζεσθων";
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_2: "~ο%ε~ῐσαι", "~ο%ε~ῐσθον", _;
    perfect_ind_mid_3: _, _, "~ο%ε~ῐδᾰται";
    perfect_imp_mid_2: _, _, "~ο%ε~ῐσθε";
    perfect_imp_mid_3: _, _, "~ο%ε~ῐσθων";
    ),
    // Ἰχᾰνᾰω: 1 words (ἰχᾰνᾰω). Like Μικρογρᾰφεω, except in 21 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ἰχᾰνᾰω", 1, "~ᾰ~ᾰω", like Μικρογρᾰφεω;
    stem lemma: "~α~- like φοιτᾰω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    ),
    // Ᾰντᾰποκρῑνομαι: 1 words (ᾰντᾰποκρῑνομαι). Like Κᾰτακειμαι, except in 60 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ᾰντᾰποκρῑνομαι", 1, "ᾰ~ο~ῑνομαι", like Κᾰτακειμαι;
    stem rest____: "~ε~ῑν- like ᾰντᾰποκρῑνομαι";
    stem rest__: "~ο~ῐν- like ᾰντᾰποκρῑνομαι";
    stem rest_______: "~ο~ῐθησ- like ᾰντᾰποκρῑνομαι";
    stem rest_: "~ο~ῑν- like ᾰντᾰποκρῑνομαι";
    stem rest_____: "~ο~ῐθ- like ᾰντᾰποκρῑνομαι";
    stem rest: "~ο%ε~ῐ- like ᾰντᾰποκρῑνομαι";
    stem rest______: "~ο%ε~ῐ- like ᾰντᾰποκρῑνομαι";
    ind_act_1: "ᾰ~ο~ῑνομαι", _, _;
    ind_mid_1: "ἀ~ο~ῑνομαι", _, "ἀ~ο~ῑνομεθᾰ";
    ind_mid_2: "ἀ~ο~ῑνῃ|ἀ~ο~ῑνει", _, _;
    ind_mid_3: _, "ἀ~ο~ῑνεσθον", _;
    ind_pass_2: _, "ἀ~ο~ῑνεσθον", "ἀ~ο~ῑνεσθε";
    ind_pass_3: "ἀ~ο~ῑνεται", "ἀ~ο~ῑνεσθον", "ἀ~ο~ῑνονται";
    subj_mid_1: "ἀ~ο~ῑνωμαι", _, "ἀ~ο~ῑνωμεθᾰ";
    subj_mid_2: "ἀ~ο~ῑνῃ", "ἀ~ο~ῑνησθον", _;
    subj_mid_3: "ἀ~ο~ῑνηται", _, "ἀ~ο~ῑνωνται";
    subj_pass_2: "ἀ~ο~ῑνῃ", _, "ἀ~ο~ῑνησθε";
    opt_mid_1: "ἀ~ο~ῑνοιμην", _, "ἀ~ο~ῑνοιμεθᾰ";
    opt_mid_2: "ἀ~ο~ῑνοιο", "ἀ~ο~ῑνοισθον", "ἀ~ο~ῑνοισθε";
    opt_mid_3: "ἀ~ο~ῑνοιτο", "ἀ~ο~ῑνοισθην", "ἀ~ο~ῑνοιντο";
    imp_mid_2: "ἀ~ο~ῑνου", "ἀ~ο~ῑνεσθον", "ἀ~ο~ῑνεσθε";
    imp_mid_3: "ἀ~ο~ῑνεσθω", _, "ἀ~ο~ῑνεσθων";
    imp_pass_2: _, "ἀ~ο~ῑνεσθον", "ἀ~ο~ῑνεσθε";
    imp_pass_3: _, "ἀ~ο~ῑνεσθων", "ἀ~ο~ῑνεσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_ind_mid_1: "ἀ~ε~ῑνᾰμην", _, _;
    aorist_ind_mid_2: "ἀ~ε~ῑνω", "ἀ~ε~ῑνᾰσθον", _;
    aorist_ind_pass_1: "ἀ~ε~ῐθην", _, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_pres: _, _, "ἀ~ο~ῑνεσθαι";
    infinitive_perf: _, _, derived;
    participle_pres: _, _, "ἀ~ο~ῑνομενος";
    ),
    // Ὑπεκτιθεμαι: 1 words (ὑπεκτιθεμαι). Like Κᾰτακειμαι, except in 67 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ὑπεκτιθεμαι", 1, "~κτι~εμαι", like Κᾰτακειμαι;
    stem rest____: "~ξε%ῐ~ε- like ὑπεκτιθεμαι";
    stem rest__: "~κ~ησ- like εἰσερχομαι";
    stem rest_______: "~κ%ε~ησ- like ποιεω";
    stem rest__________: "~ξετε~ην- like ἐλπω";
    stem rest_____: "~κτε~- like ὑπεκτιθεμαι";
    stem rest: "~κ%ε~ει- like εὐλογεω";
    stem rest______: "~κ%ε~ει- like εὐλογεω";
    ind_act_1: "~κτι~εμαι", _, _;
    ind_mid_1: "~κ%ῐ~εμαι", _, "~κ%ῐ~εμεθᾰ";
    ind_mid_2: "~κ%ῐ~εσαι", _, _;
    ind_mid_3: _, "~κ%ῐ~εσθον", _;
    ind_pass_2: _, "~κ%ῐ~εσθον", "~κ%ῐ~εσθε";
    ind_pass_3: "~κ%ῐ~εται", "~κ%ῐ~εσθον", "~κ%ῐ~ενται";
    subj_mid_1: "~κ%ῐ~ωμαι", _, "~κ%ῐ~ωμεθᾰ";
    subj_mid_2: "~κ%ῐ~ῃ", "~κ%ῐ~ησθον", _;
    subj_mid_3: "~κ%ῐ~ηται", _, "~κ%ῐ~ωνται";
    subj_pass_2: "~κ%ῐ~ῃ", _, "~κ%ῐ~ησθε";
    opt_mid_1: "~κ%ῐ~ειμην", _, "~κ%ῐ~ειμεθᾰ|~κ%ῐ~οιμεθᾰ";
    opt_mid_2: "~κ%ῐ~ειο", "~κ%ῐ~εισθον|~κ%ῐ~οισθον", "~κ%ῐ~εισθε|~κ%ῐ~οισθε";
    opt_mid_3: "~κ%ῐ~ειτο|~κ%ῐ~οιτο", "~κ%ῐ~εισθην|~κ%ῐ~οισθην", "~κ%ῐ~ειντο|~κ%ῐ~οιντο";
    imp_mid_2: "~κ%ῐ~εσο|~κ%ῐ~ου", "~κ%ῐ~εσθον", "~κ%ῐ~εσθε";
    imp_mid_3: "~κ%ῐ~εσθω", _, "~κ%ῐ~εσθων";
    imp_pass_2: _, "~κ%ῐ~εσθον", "~κ%ῐ~εσθε";
    imp_pass_3: _, "~κ%ῐ~εσθων", "~κ%ῐ~εσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_ind_mid_1: "~ξε~εμην", _, _;
    aorist_ind_mid_2: "~ξε~ου", "~ξε~εσθον", _;
    aorist_subj_mid_1: _, _, "~κ~ωμεθᾰ";
    aorist_subj_mid_2: "~κ~ῃ", "~κ~ησθον", _;
    aorist_opt_mid_1: "~κ~ειμην", _, _;
    aorist_opt_mid_3: "~κ~ειτο|~κ~οιτο", _, _;
    aorist_imp_mid_2: "~κ~ου", "~κ~εσθον", _;
    aorist_imp_mid_3: _, _, "~κ~εσθων";
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_pres: _, _, "~κ%ῐ~εσθαι";
    infinitive_perf: _, _, derived;
    participle_pres: _, _, "~κ%ῐ~εμενος";
    participle_aor: _, "~κ~εμενος", _;
    ),
    // Μᾰστῐγοω: 1 words (μᾰστῐγοω). Like Ἀποστατεω, except in 29 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_act_2 du, subj_mid_2 sg, subj_mid_3 du, ….
    conjugation!("μᾰστῐγοω", 1, "~ᾰ~ῐ~οω", like Ἀποστατεω;
    stem lemma: "~α~ι~- like βουλευω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_act_2: _, derived, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    ),
    // Βᾰρβᾰροομαι: 1 words (βᾰρβᾰροομαι). Like Ἀποστατεω, except in 34 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("βᾰρβᾰροομαι", 1, "~ᾰ~ᾰροομ~ι", like Ἀποστατεω;
    stem rest__________: "~α~~ρωθην- like ἐχω";
    stem rest_____: "~α~~ρωθ- like ποιεω";
    stem rest: "~εβα~~ρω- like ὑποκοριζομαι";
    stem rest______: "~εβα~~ρω- like δῐᾰσευομαι";
    ind_act_1: "~ᾰ~ᾰροομ~ι", _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: derived, _, derived;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, _;
    subj_act_3: derived, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    perfect_ind_mid_3: _, derived, _;
    perfect_imp_mid_2: _, derived, _;
    infinitive_pres: derived, _, _;
    participle_pres: derived, _, _;
    ),
    // Ἐπιμιμνησκομαι: 1 words (ἐπιμιμνησκομαι). Like Κᾰτακειμαι, except in 56 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἐπιμιμνησκομαι", 1, "~ιμι~σκομαι", like Κᾰτακειμαι;
    stem rest____: "~ε%ῐ~σκ- like εἰσερχομαι";
    stem rest__: "~ῐ~σ- like εἰσερχομαι";
    stem rest_______: "~ῐ~σθησ- like ποιεω";
    stem rest________: "~ε~σ- like μηκᾰομαι";
    stem rest__________: "~ε~σθην- like ἐλπω";
    stem rest_: "~ῐ~σ- like ἑπομαι";
    stem rest_____: "~ῐ~σθ- like ποιεω";
    stem rest: "~ῐ%ε~- like εὐλογεω";
    stem rest______: "~ῐ%ε~- like εὐλογεω";
    ind_act_1: "~ιμι~σκομαι", _, _;
    ind_mid_1: "~ῐ%ῐ~σκομαι", _, "~ῐ%ῐ~σκομεθᾰ";
    ind_mid_2: "~ῐ%ῐ~σκῃ|~ῐ%ῐ~σκει", _, _;
    ind_mid_3: _, "~ῐ%ῐ~σκεσθον", _;
    ind_pass_2: _, "~ῐ%ῐ~σκεσθον", "~ῐ%ῐ~σκεσθε";
    ind_pass_3: "~ῐ%ῐ~σκεται", "~ῐ%ῐ~σκεσθον", "~ῐ%ῐ~σκονται";
    subj_mid_1: "~ῐ%ῐ~σκωμαι", _, "~ῐ%ῐ~σκωμεθᾰ";
    subj_mid_2: "~ῐ%ῐ~σκῃ", "~ῐ%ῐ~σκησθον", _;
    subj_mid_3: "~ῐ%ῐ~σκηται", _, "~ῐ%ῐ~σκωνται";
    subj_pass_2: "~ῐ%ῐ~σκῃ", _, "~ῐ%ῐ~σκησθε";
    opt_mid_1: "~ῐ%ῐ~σκοιμην", _, "~ῐ%ῐ~σκοιμεθᾰ";
    opt_mid_2: "~ῐ%ῐ~σκοιο", "~ῐ%ῐ~σκοισθον", "~ῐ%ῐ~σκοισθε";
    opt_mid_3: "~ῐ%ῐ~σκοιτο", "~ῐ%ῐ~σκοισθην", "~ῐ%ῐ~σκοιντο";
    imp_mid_2: "~ῐ%ῐ~σκου", "~ῐ%ῐ~σκεσθον", "~ῐ%ῐ~σκεσθε";
    imp_mid_3: "~ῐ%ῐ~σκεσθω", _, "~ῐ%ῐ~σκεσθων";
    imp_pass_2: _, "~ῐ%ῐ~σκεσθον", "~ῐ%ῐ~σκεσθε";
    imp_pass_3: _, "~ῐ%ῐ~σκεσθων", "~ῐ%ῐ~σκεσθων";
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, derived, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_mid_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_mid_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: derived, derived, derived;
    perfect_subj_pass_3: _, _, derived;
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    infinitive_pres: _, _, "~ῐ%ῐ~σκεσθαι";
    infinitive_perf: _, _, derived;
    participle_pres: _, _, "~ῐ%ῐ~σκομενος";
    ),
    // Εργεω: 1 words (εργεω). Like Τελεθω, except in 60 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("εργεω", 1, "ε~εω", like Τελεθω;
    ind_act_1: "ἐ~ω", _, "ἐ~ουμεν";
    ind_act_2: "ἐ~εις", "ἐ~ειτον", _;
    ind_act_3: "ἐ~ει", _, "ἐ~ουσῐ|ἐ~ουσῐν";
    ind_mid_1: _, _, "ἐ~ουμεθᾰ";
    ind_mid_2: "ἐ~ει|ἐ~ῃ", _, _;
    ind_mid_3: _, "ἐ~εισθον", _;
    ind_pass_1: "ἐ~ουμαι", _, _;
    ind_pass_2: _, "ἐ~εισθον", "ἐ~εισθε";
    ind_pass_3: "ἐ~ειται", "ἐ~εισθον", "ἐ~ουνται";
    subj_act_1: "ἐ~ω", _, "ἐ~ωμεν";
    subj_act_2: "ἐ~ῃς", _, _;
    subj_act_3: "ἐ~ῃ", "ἐ~ητον", "ἐ~ωσῐ|ἐ~ωσῐν";
    subj_mid_1: _, _, "ἐ~ωμεθᾰ";
    subj_mid_2: derived, "ἐ~ησθον", _;
    subj_mid_3: "ἐ~ηται", _, "ἐ~ωνται";
    subj_pass_1: "ἐ~ωμαι", _, _;
    subj_pass_2: _, _, "ἐ~ησθε";
    opt_act_1: "ἐ~οιην|ἐ~οιμῐ", _, "ἐ~οιμεν|ἐ~οιημεν";
    opt_act_2: "ἐ~οιης|ἐ~οις", "ἐ~οιτον|ἐ~οιητον", "ἐ~οιτε|ἐ~οιητε";
    opt_act_3: "ἐ~οιη|ἐ~οι", "ἐ~οιτην|ἐ~οιητην", "ἐ~οιεν|ἐ~οιησᾰν";
    opt_mid_1: "ἐ~οιμην", _, "ἐ~οιμεθᾰ";
    opt_mid_2: "ἐ~οιο", "ἐ~οισθον", "ἐ~οισθε";
    opt_mid_3: "ἐ~οιτο", "ἐ~οισθην", "ἐ~οιντο";
    imp_act_2: "ἐ~ει", _, "ἐ~ειτε";
    imp_act_3: _, _, "ἐ~ουντων";
    imp_mid_2: "ἐ~ου", "ἐ~εισθον", "ἐ~εισθε";
    imp_mid_3: _, _, "ἐ~εισθων";
    imp_pass_2: _, "ἐ~εισθον", "ἐ~εισθε";
    imp_pass_3: "ἐ~εισθω", "ἐ~εισθων", "ἐ~εισθων";
    infinitive_pres: "ἐ~ειν", _, "ἐ~εισθαι";
    participle_pres: "ἐ~ων", _, "ἐ~ουμενος";
    ),
    // Ᾰνθρωπεω: 1 words (ᾰνθρωπεω). Like Ἐρωτᾰω, except in 43 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("ᾰνθρωπεω", 1, "ᾰ~εω", like Ἐρωτᾰω;
    stem lemma: "~- like ᾰνθρωπεω";
    stem rest____: "~- like ᾰνθρωπεω";
    stem rest__: "~ησ- like ᾰνθρωπεω";
    stem rest_______: "~ηθησ- like ᾰντᾰποκρῑνομαι";
    stem rest________: "~ησ- like ἐλπῐζω";
    stem rest_: "~ησ- like ᾰνθρωπεω";
    stem rest_____: "~ηθ- like ᾰντᾰποκρῑνομαι";
    stem rest___: "~ηκ- like ἐλπῐζω";
    stem rest: "~η- like ἐλπῐζω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_pass_1: derived, _, derived;
    imperfect_pass_2: derived, _, _;
    imperfect_pass_3: derived, _, derived;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_3: derived, _, _;
    perfect_subj_pass_3: _, _, derived;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    ),
    // Εισκᾰτοικεω: 1 words (εισκᾰτοικεω). Like Δῐοικεω, except in 75 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("εισκᾰτοικεω", 1, "ε~οι~εω", like Δῐοικεω;
    stem lemma: "~οι~- like ερεω";
    stem rest__: "~οι~ησ- like εισκᾰτοικεω";
    stem rest_______: "~οι~ηθησ- like εισκᾰτοικεω";
    stem rest________: "~ῳ~ησ- like εισκᾰτοικεω";
    stem rest_: "~οι~ησ- like εισκᾰτοικεω";
    stem rest_____: "~οι~ηθ- like εισκᾰτοικεω";
    stem rest___: "~ῳ~ηκ- like εισκᾰτοικεω";
    stem rest______: "~ῳ~η- like επῐλῡω";
    stem rest___________: "~ῳ~ηκ- like εισκᾰτοικεω";
    stem rest_________: "~ῳ~η- like επῐλῡω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    imperfect_act_1: "ἑ~ῳ~ουν", _, _;
    imperfect_act_2: "ἑ~ῳ~εις", "ἑ~ῳ~ειτον", "ἑ~ῳ~ειτε";
    imperfect_act_3: "ἑ~ῳ~ει", _, "ἑ~ῳ~ουν";
    imperfect_mid_1: "ἑ~ῳ~ουμην", _, "ἑ~ῳ~ουμεθᾰ";
    imperfect_mid_2: "ἑ~ῳ~ου", _, _;
    imperfect_mid_3: "ἑ~ῳ~ειτο", _, "ἑ~ῳ~ουντο";
    imperfect_pass_2: _, "ἑ~ῳ~εισθον", "ἑ~ῳ~εισθε";
    imperfect_pass_3: _, "ἑ~ῳ~εισθην", _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    aorist_ind_pass_1: "ἑ~ῳ~ηθην", _, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: "ἑ~ῳ~ημαι", _, _;
    perfect_ind_mid_3: "ἑ~ῳ~ηται", derived, _;
    perfect_ind_pass_1: _, _, "ἑ~ῳ~ημεθᾰ";
    perfect_ind_pass_2: derived, derived, _;
    perfect_ind_pass_3: _, derived, derived;
    perfect_subj_mid_3: _, "ἑ~ῳ~ημένω ἦτον", "ἑ~ῳ~ημένοι ὦσῐ|ἑ~ῳ~ημένοι ὦσῐν";
    perfect_subj_pass_1: "ἑ~ῳ~ημένος ὦ", _, "ἑ~ῳ~ημένοι ὦμεν";
    perfect_subj_pass_2: "ἑ~ῳ~ημένος ᾖς", "ἑ~ῳ~ημένω ἦτον", "ἑ~ῳ~ημένοι ἦτε";
    perfect_subj_pass_3: "ἑ~ῳ~ημένος ᾖ", "ἑ~ῳ~ημένω ἦτον", _;
    perfect_opt_pass_1: "ἑ~ῳ~ημένος εἴην", _, "ἑ~ῳ~ημένοι εἴημεν";
    perfect_opt_pass_2: "ἑ~ῳ~ημένος εἴης", "ἑ~ῳ~ημένω εἴητον", "ἑ~ῳ~ημένοι εἴητε";
    perfect_opt_pass_3: "ἑ~ῳ~ημένος εἴη", "ἑ~ῳ~ημένω εἰήτην", "ἑ~ῳ~ημένοι εἴησᾰν";
    perfect_imp_mid_2: _, derived, _;
    perfect_imp_pass_2: _, derived, _;
    pluperfect_pass_1: derived, _, _;
    pluperfect_pass_2: derived, derived, _;
    pluperfect_pass_3: derived, _, derived;
    infinitive_perf: _, _, derived;
    participle_perf: _, _, "ἑ~ῳ~ημενος";
    ),
    // Εκπλεω: 1 words (εκπλεω). Like Λλῡμῐ, except in 114 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("εκπλεω", 1, "εκ~ω", like Λλῡμῐ;
    stem rest________: "~υσ- like εκπλεω";
    stem rest__________: "~υσθην- like εκπλεω";
    stem rest_: "~υσ- like εκπλεω";
    stem rest_____: "~υσθ- like εκπλεω";
    stem rest___: "~υκ- like εκπλεω";
    ind_act_1: "ἐκ~ω", _, "ἐκ~ομεν";
    ind_act_2: "ἐκ~ις", "ἐκ~ιτον", _;
    ind_act_3: "ἐκ~ι", _, "ἐκ~ουσῐ|ἐκ~ουσῐν";
    ind_mid_1: _, _, "ἐκ~ομεθᾰ";
    ind_mid_3: _, "ἐκ~ισθον", _;
    ind_pass_1: "ἐκ~ομαι", _, _;
    ind_pass_2: _, "ἐκ~ισθον", "ἐκ~ισθε";
    ind_pass_3: "ἐκ~ιται", "ἐκ~ισθον", "ἐκ~ονται";
    subj_act_1: "ἐκ~ω", _, "ἐκ~ωμεν";
    subj_act_2: "ἐκ~ῃς", _, _;
    subj_act_3: "ἐκ~ῃ", "ἐκ~ητον", "ἐκ~ωσῐ|ἐκ~ωσῐν";
    subj_mid_1: _, _, "ἐκ~ωμεθᾰ";
    subj_mid_2: _, "ἐκ~ησθον", _;
    subj_mid_3: "ἐκ~ηται", _, "ἐκ~ωνται";
    subj_pass_1: "ἐκ~ωμαι", _, _;
    subj_pass_2: _, _, "ἐκ~ησθε";
    opt_act_1: "ἐκ~οιμῐ", _, "ἐκ~οιμεν";
    opt_act_2: "ἐκ~οις", "ἐκ~οιτον", "ἐκ~οιτε";
    opt_act_3: "ἐκ~οι", "ἐκ~οιτην", "ἐκ~οιεν";
    opt_mid_1: "ἐκ~οιμην", _, "ἐκ~οιμεθᾰ";
    opt_mid_2: "ἐκ~οιο", "ἐκ~οισθον", "ἐκ~οισθε";
    opt_mid_3: "ἐκ~οιτο", "ἐκ~οισθην", "ἐκ~οιντο";
    imp_act_2: "ἐκ~ι", _, "ἐκ~ιτε";
    imp_act_3: _, _, "ἐκ~οντων";
    imp_mid_2: "ἐκ~ου", "ἐκ~ισθον", "ἐκ~ισθε";
    imp_mid_3: _, _, "ἐκ~εσθων";
    imp_pass_2: _, "ἐκ~ισθον", "ἐκ~ισθε";
    imp_pass_3: "ἐκ~ισθω", "ἐκ~ισθων", "ἐκ~εσθων";
    imperfect_act_1: "ἐξε~ον", _, _;
    imperfect_act_2: "ἐξε~ις", "ἐξε~ιτον", "ἐξε~ιτε";
    imperfect_act_3: "ἐξε~ι|ἐξε~ιν", _, "ἐξε~ον";
    imperfect_mid_1: "ἐξε~ομην", _, "ἐξε~ομεθᾰ";
    imperfect_mid_2: "ἐξε~ου", _, _;
    imperfect_mid_3: "ἐξε~ιτο", _, "ἐξε~οντο";
    imperfect_pass_2: _, "ἐξε~ισθον", "ἐξε~ισθε";
    imperfect_pass_3: _, "ἐξε~ισθην", _;
    future_ind_mid_1: "ἐκ~υσομαι", _, _;
    future_ind_mid_2: "ἐκ~υσῃ|ἐκ~υσει", "ἐκ~υσεσθον", _;
    future_ind_mid_3: "ἐκ~υσεται", _, _;
    future_opt_mid_1: "ἐκ~υσοιμην", _, _;
    future_opt_mid_3: "ἐκ~υσοιτο", "ἐκ~υσοισθην", _;
    aorist_subj_act_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    perfect_ind_mid_1: "ἐκ%ε~υσμαι", _, _;
    perfect_ind_mid_2: "ἐκ%ε~υσαι", "ἐκ%ε~υσθον", _;
    perfect_ind_mid_3: "ἐκ%ε~υσται", _, "ἐκ%ε~υσᾰται";
    perfect_ind_pass_1: _, _, "ἐκ%ε~υσμεθᾰ";
    perfect_ind_pass_2: _, _, "ἐκ%ε~υσθε";
    perfect_subj_mid_3: _, "ἐκ%ε~υσμένω ἦτον", "ἐκ%ε~υσμένοι ὦσῐ|ἐκ%ε~υσμένοι ὦσῐν";
    perfect_subj_pass_1: "ἐκ%ε~υσμένος ὦ", _, "ἐκ%ε~υσμένοι ὦμεν";
    perfect_subj_pass_2: "ἐκ%ε~υσμένος ᾖς", "ἐκ%ε~υσμένω ἦτον", "ἐκ%ε~υσμένοι ἦτε";
    perfect_subj_pass_3: "ἐκ%ε~υσμένος ᾖ", "ἐκ%ε~υσμένω ἦτον", _;
    perfect_opt_pass_1: "ἐκ%ε~υσμένος εἴην", _, "ἐκ%ε~υσμένοι εἴημεν";
    perfect_opt_pass_2: "ἐκ%ε~υσμένος εἴης", "ἐκ%ε~υσμένω εἴητον", "ἐκ%ε~υσμένοι εἴητε";
    perfect_opt_pass_3: "ἐκ%ε~υσμένος εἴη", "ἐκ%ε~υσμένω εἰήτην", "ἐκ%ε~υσμένοι εἴησᾰν";
    perfect_imp_mid_2: _, _, "ἐκ%ε~υσθε";
    perfect_imp_mid_3: _, _, "ἐκ%ε~υσθων";
    perfect_imp_pass_2: "ἐκ%ε~υσο", _, "ἐκ%ε~υσθε";
    perfect_imp_pass_3: "ἐκ%ε~υσθω", "ἐκ%ε~υσθων", "ἐκ%ε~υσθων";
    infinitive_pres: "ἐκ~ιν", _, "ἐκ~ισθαι";
    infinitive_perf: _, "ἐκ%ε~υσθαι", _;
    participle_pres: "ἐκ~ων", _, "ἐκ~ομενος";
    participle_perf: _, _, "ἐκ%ε~υσμενος";
    ),
    // Δειδῐᾰ: 1 words (δειδῐᾰ). Like Μετεκῑᾰθον, except in 4 cells: ind_act_1 sg, aorist_subj_act_3 du, aorist_subj_mid_3 du, perfect_ind_act_3 du.
    conjugation!("δειδῐᾰ", 1, "~~δῐᾰ", like Μετεκῑᾰθον;
    stem rest________: "~~σ- like ποιεω";
    stem rest_: "~~σ- like ποιεω";
    stem rest___: "~δο~κ- like εὐλογεω";
    ind_act_1: "~~δῐᾰ", _, _;
    aorist_subj_act_3: _, derived, _;
    aorist_subj_mid_3: _, derived, _;
    perfect_ind_act_3: _, derived, _;
    ),
    // Λανθανομαι: 1 words (λανθανομαι). Like Μᾰστῐγοω, except in 21 cells: ind_act_3 du, ind_pass_1 pl, ind_pass_2 sg, subj_mid_2 sg, subj_mid_3 du, subj_pass_1 pl, ….
    conjugation!("λανθανομαι", 1, "~α~α~ομαι", like Μᾰστῐγοω;
    stem lemma: "~ᾰ~ᾰ~- like βουλευω";
    ind_act_3: _, derived, _;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_mid_3: _, derived, _;
    subj_pass_1: _, _, derived;
    subj_pass_2: derived, derived, _;
    subj_pass_3: derived, derived, derived;
    opt_pass_1: derived, _, derived;
    opt_pass_2: derived, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: _, derived, _;
    imp_pass_2: derived, _, _;
    ),
    // Τρᾰπω: 1 words (τρᾰπω). Like Λλῡμῐ, except in 6 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, future_ind_act_3 du, future_ind_mid_3 du.
    conjugation!("τρᾰπω", 1, "~ᾰπω", like Λλῡμῐ;
    stem rest__: "~αψ- like τρᾰπω";
    ind_act_1: "~ᾰπω", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_3: _, derived, _;
    future_ind_mid_3: _, derived, _;
    ),
];
