//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it. A `ˇ` ends
//! the patterns of a conjugation whose stems alternate: the laws in `laws.rs`
//! for alternating stems change them (`~nˇ`: `katu`, `kadun`).

use flexion::Paradigm;

/// Writes one conjugation: the word it is named after, how many words follow it,
/// the lemma's pattern, its words' inherent features (`[gender: Neuter]`, if
/// they share them), the conjugation it is like (if any), the stem slots
/// it states with a stem and an ending table (`stem aorist: "~σ- like λυω"`:
/// the slot's cells are `λυω`'s table's patterns with `~σ` for their `~`; see
/// `stems.rs`), then a row of patterns per line (`derived`: the form comes
/// from another cell's, see `relations.rs`; `_`, or a row left out: the
/// form is its stem slot's, or the conjugation's it is like, or, in a conjugation like
/// no other, derived):
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
     $(stem present: $s0:tt;)?
     $(stem imperfect: $s1:tt;)?
     $(stem future: $s2:tt;)?
     $(stem future_pass: $s3:tt;)?
     $(stem aorist_ind: $s4:tt;)?
     $(stem aorist_ind_pass: $s5:tt;)?
     $(stem aorist: $s6:tt;)?
     $(stem aorist_pass: $s7:tt;)?
     $(stem perfect_act: $s8:tt;)?
     $(stem perfect: $s9:tt;)?
     $(stem pluperfect_act: $s10:tt;)?
     $(stem pluperfect: $s11:tt;)?
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

/// What a conjugation adds after each pattern of the one it is like (`with "~ь", "~я"
/// after "йлмть"`: `~ь`, or `~я` where that pattern ends in `й`, `л`, `м`, `т` or `ь`).
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
    /// Like `ἐχω`: 235 words.
    Ἐχω,
    /// Like `ποιεω`: 204 words.
    Ποιεω,
    /// Like `βουλευω`: 122 words.
    Βουλευω,
    /// Like `γῡροω`: 80 words.
    Γῡροω,
    /// Like `χρημᾰτῐζω`: 63 words.
    Χρημᾰτῐζω,
    /// Like `ἑπομαι`: 61 words.
    Ἑπομαι,
    /// Like `βᾰπτῐζω`: 60 words.
    Βᾰπτῐζω,
    /// Like `δοκᾰω`: 54 words.
    Δοκᾰω,
    /// Like `δῐκεω`: 49 words.
    Δῐκεω,
    /// Like `τᾰσσω`: 32 words.
    Τᾰσσω,
    /// Like `ληγω`: 30 words.
    Ληγω,
    /// Like `ψηφῐζω`: 27 words.
    Ψηφῐζω,
    /// Like `σῠνᾰρπᾰζω`: 24 words.
    Σῠνᾰρπᾰζω,
    /// Like `εὐλογεω`: 22 words.
    Εὐλογεω,
    /// Like `ἁπτω`: 21 words.
    Ἁπτω,
    /// Like `ἐλπω`: 20 words.
    Ἐλπω,
    /// Like `κτῐζω`: 20 words.
    Κτῐζω,
    /// Like `νῑκαω`: 18 words.
    Νῑκαω,
    /// Like `πᾰραιτεομαι`: 18 words.
    Πᾰραιτεομαι,
    /// Like `πορσῡνω`: 18 words.
    Πορσῡνω,
    /// Like `πεμπω`: 17 words.
    Πεμπω,
    /// Like `θωᾰω`: 16 words.
    Θωᾰω,
    /// Like `πειρᾰζω`: 15 words.
    Πειρᾰζω,
    /// Like `ἀνῐστημῐ`: 15 words.
    Ἀνῐστημῐ,
    /// Like `λογῐζομαι`: 14 words.
    Λογῐζομαι,
    /// Like `σκᾰνδᾰλῐζω`: 13 words.
    Σκᾰνδᾰλῐζω,
    /// Like `μνημονευω`: 13 words.
    Μνημονευω,
    /// Like `ἀειρω`: 13 words.
    Ἀειρω,
    /// Like `ἀραομαι`: 13 words.
    Ἀραομαι,
    /// Like `δῐοικεω`: 12 words.
    Δῐοικεω,
    /// Like `φαινω`: 12 words.
    Φαινω,
    /// Like `σβεννῡμῐ`: 12 words.
    Σβεννῡμῐ,
    /// Like `ἐπᾰϊω`: 12 words.
    Ἐπᾰϊω,
    /// Like `κειμαι`: 11 words.
    Κειμαι,
    /// Like `κᾰθιδρῡω`: 11 words.
    Κᾰθιδρῡω,
    /// Like `ἐγειρω`: 11 words.
    Ἐγειρω,
    /// Like `πειθω`: 11 words.
    Πειθω,
    /// Like `ἱημῐ`: 11 words.
    Ἱημῐ,
    /// Like `μῑγνῡμῐ`: 10 words.
    Μῑγνῡμῐ,
    /// Like `αἱρεω`: 10 words.
    Αἱρεω,
    /// Like `οἰομαι`: 10 words.
    Οἰομαι,
    /// Like `χεω`: 10 words.
    Χεω,
    /// Like `ἀφῑημῐ`: 10 words.
    Ἀφῑημῐ,
    /// Like `ἐνᾰντῐοομαι`: 9 words.
    Ἐνᾰντῐοομαι,
    /// Like `κᾰταφρονεω`: 9 words.
    Κᾰταφρονεω,
    /// Like `εἰσερχομαι`: 9 words.
    Εἰσερχομαι,
    /// Like `ὀμνῡμῐ`: 9 words.
    Ὀμνῡμῐ,
    /// Like `ἀθῡρω`: 8 words.
    Ἀθῡρω,
    /// Like `ψῑλοω`: 8 words.
    Ψῑλοω,
    /// Like `ἀνᾰγω`: 8 words.
    Ἀνᾰγω,
    /// Like `φᾰρμᾰσσω`: 8 words.
    Φᾰρμᾰσσω,
    /// Like `δῐωκω`: 8 words.
    Δῐωκω,
    /// Like `φῠω`: 8 words.
    Φῠω,
    /// Like `ἀρδω`: 8 words.
    Ἀρδω,
    /// Like `πλᾰσσω`: 8 words.
    Πλᾰσσω,
    /// Like `πᾰρεχω`: 8 words.
    Πᾰρεχω,
    /// Like `κρῑνομαι`: 8 words.
    Κρῑνομαι,
    /// Like `πῠνθανομαι`: 8 words.
    Πῠνθανομαι,
    /// Like `εἰμῐ`: 8 words.
    Εἰμῐ,
    /// Like `πρεσβευω`: 7 words.
    Πρεσβευω,
    /// Like `σῳζω`: 7 words.
    Σῳζω,
    /// Like `σφᾰλλω`: 7 words.
    Σφᾰλλω,
    /// Like `καθιζω`: 7 words.
    Καθιζω,
    /// Like `ἀπειπον`: 7 words.
    Ἀπειπον,
    /// Like `χραω`: 7 words.
    Χραω,
    /// Like `λῡω`: 7 words.
    Λῡω,
    /// Like `θλαω`: 7 words.
    Θλαω,
    /// Like `ἐπῐλᾰμβᾰνω`: 7 words.
    Ἐπῐλᾰμβᾰνω,
    /// Like `τελεω`: 6 words.
    Τελεω,
    /// Like `μωραινω`: 6 words.
    Μωραινω,
    /// Like `κατοικεω`: 6 words.
    Κατοικεω,
    /// Like `λᾰγοθηρεω`: 6 words.
    Λᾰγοθηρεω,
    /// Like `σειω`: 6 words.
    Σειω,
    /// Like `μηκᾰομαι`: 6 words.
    Μηκᾰομαι,
    /// Like `ἐπεξειμι`: 6 words.
    Ἐπεξειμι,
    /// Like `ἐαω`: 6 words.
    Ἐαω,
    /// Like `περιειμι`: 6 words.
    Περιειμι,
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
    /// Like `ἐλπῐζω`: 5 words.
    Ἐλπῐζω,
    /// Like `ἀναιρεω`: 5 words.
    Ἀναιρεω,
    /// Like `σῠνοικῐζω`: 5 words.
    Σῠνοικῐζω,
    /// Like `θρῠπτω`: 5 words.
    Θρῠπτω,
    /// Like `τρεφω`: 5 words.
    Τρεφω,
    /// Like `μετερχομαι`: 5 words.
    Μετερχομαι,
    /// Like `ἀποκωλῡω`: 5 words.
    Ἀποκωλῡω,
    /// Like `σκηπτω`: 5 words.
    Σκηπτω,
    /// Like `εἰδομαι`: 5 words.
    Εἰδομαι,
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
    /// Like `βαινω`: 4 words.
    Βαινω,
    /// Like `τῠπτω`: 4 words.
    Τῠπτω,
    /// Like `ἐπᾰγγελλω`: 4 words.
    Ἐπᾰγγελλω,
    /// Like `ἀμειβω`: 4 words.
    Ἀμειβω,
    /// Like `προσφερω`: 4 words.
    Προσφερω,
    /// Like `ἐπιτρεχω`: 4 words.
    Ἐπιτρεχω,
    /// Like `σπᾰω`: 4 words.
    Σπᾰω,
    /// Like `σῠνᾰγω`: 4 words.
    Σῠνᾰγω,
    /// Like `σᾰττω`: 4 words.
    Σᾰττω,
    /// Like `ἐργᾰζομαι`: 4 words.
    Ἐργᾰζομαι,
    /// Like `ἐρᾰμαι`: 4 words.
    Ἐρᾰμαι,
    /// Like `ἀνᾰπτω`: 4 words.
    Ἀνᾰπτω,
    /// Like `δῡω`: 4 words.
    Δῡω,
    /// Like `πειρᾰω`: 4 words.
    Πειρᾰω,
    /// Like `πᾰρᾰλλᾰσσω`: 4 words.
    Πᾰρᾰλλᾰσσω,
    /// Like `κᾰτειργω`: 4 words.
    Κᾰτειργω,
    /// Like `στελλω`: 4 words.
    Στελλω,
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
    /// Like `δημοκρᾰτεομαι`: 4 words.
    Δημοκρᾰτεομαι,
    /// Like `ἀνᾰλᾰμβᾰνω`: 4 words.
    Ἀνᾰλᾰμβᾰνω,
    /// Like `γυμνοω`: 4 words.
    Γυμνοω,
    /// Like `δῐαιρω`: 4 words.
    Δῐαιρω,
    /// Like `προβαλλω`: 4 words.
    Προβαλλω,
    /// Like `ἀφορῐζω`: 4 words.
    Ἀφορῐζω,
    /// Like `κᾰκοω`: 3 words.
    Κᾰκοω,
    /// Like `πῑαινω`: 3 words.
    Πῑαινω,
    /// Like `θερᾰπευω`: 3 words.
    Θερᾰπευω,
    /// Like `ἀποζω`: 3 words.
    Ἀποζω,
    /// Like `πλᾰναω`: 3 words.
    Πλᾰναω,
    /// Like `δῐᾰρπᾰζω`: 3 words.
    Δῐᾰρπᾰζω,
    /// Like `ὀμνῠω`: 3 words.
    Ὀμνῠω,
    /// Like `ψᾱω`: 3 words.
    Ψᾱω,
    /// Like `ἐπῐτῐθημῐ`: 3 words.
    Ἐπῐτῐθημῐ,
    /// Like `ἐπαινεω`: 3 words.
    Ἐπαινεω,
    /// Like `λιστραω`: 3 words.
    Λιστραω,
    /// Like `φλῡω`: 3 words.
    Φλῡω,
    /// Like `ἐπευχομαι`: 3 words.
    Ἐπευχομαι,
    /// Like `ἐφελκω`: 3 words.
    Ἐφελκω,
    /// Like `τρῑβω`: 3 words.
    Τρῑβω,
    /// Like `ἀντεχω`: 3 words.
    Ἀντεχω,
    /// Like `ποιμαινω`: 3 words.
    Ποιμαινω,
    /// Like `σῠνιστημῐ`: 3 words.
    Σῠνιστημῐ,
    /// Like `ἰαινω`: 3 words.
    Ἰαινω,
    /// Like `ἐφῐστημῐ`: 3 words.
    Ἐφῐστημῐ,
    /// Like `δρᾰσσομαι`: 3 words.
    Δρᾰσσομαι,
    /// Like `πᾰρᾰκᾰλεω`: 3 words.
    Πᾰρᾰκᾰλεω,
    /// Like `ἐξαιρω`: 3 words.
    Ἐξαιρω,
    /// Like `δῐᾰλῡω`: 3 words.
    Δῐᾰλῡω,
    /// Like `ἁμᾰρτᾰνω`: 3 words.
    Ἁμᾰρτᾰνω,
    /// Like `κᾰμπτω`: 3 words.
    Κᾰμπτω,
    /// Like `μεθῐστημῐ`: 3 words.
    Μεθῐστημῐ,
    /// Like `ἐκφερω`: 3 words.
    Ἐκφερω,
    /// Like `ἐξετᾰζω`: 3 words.
    Ἐξετᾰζω,
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
    /// Like `λευκαινω`: 3 words.
    Λευκαινω,
    /// Like `σῠνδεω`: 3 words.
    Σῠνδεω,
    /// Like `κηδευω`: 3 words.
    Κηδευω,
    /// Like `λεγομαι`: 3 words.
    Λεγομαι,
    /// Like `ῥεω`: 3 words.
    Ῥεω,
    /// Like `δῐᾰσῳζω`: 3 words.
    Δῐᾰσῳζω,
    /// Like `κᾰτᾰβῐβᾰζω`: 3 words.
    Κᾰτᾰβῐβᾰζω,
    /// Like `ῥηγνῡμῐ`: 3 words.
    Ῥηγνῡμῐ,
    /// Like `ὠνεομαι`: 3 words.
    Ὠνεομαι,
    /// Like `κᾰτᾰπονεω`: 3 words.
    Κᾰτᾰπονεω,
    /// Like `κηρῡσσω`: 3 words.
    Κηρῡσσω,
    /// Like `ἐπορον`: 2 words.
    Ἐπορον,
    /// Like `εἰδον`: 2 words.
    Εἰδον,
    /// Like `καινῠμαι`: 2 words.
    Καινῠμαι,
    /// Like `ξυμπολεμεω`: 2 words.
    Ξυμπολεμεω,
    /// Like `δῐακειμαι`: 2 words.
    Δῐακειμαι,
    /// Like `ξῠνοιδᾰ`: 2 words.
    Ξῠνοιδᾰ,
    /// Like `συγκειμαι`: 2 words.
    Συγκειμαι,
    /// Like `νισσομαι`: 2 words.
    Νισσομαι,
    /// Like `κᾰτασκηνοω`: 2 words.
    Κᾰτασκηνοω,
    /// Like `γεγονοι`: 2 words.
    Γεγονοι,
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
    /// Like `επῐλῡω`: 2 words.
    Επῐλῡω,
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
    pub const ALL: [Conjugation; 287] = [
        Conjugation::Ἐχω,
        Conjugation::Ποιεω,
        Conjugation::Βουλευω,
        Conjugation::Γῡροω,
        Conjugation::Χρημᾰτῐζω,
        Conjugation::Ἑπομαι,
        Conjugation::Βᾰπτῐζω,
        Conjugation::Δοκᾰω,
        Conjugation::Δῐκεω,
        Conjugation::Τᾰσσω,
        Conjugation::Ληγω,
        Conjugation::Ψηφῐζω,
        Conjugation::Σῠνᾰρπᾰζω,
        Conjugation::Εὐλογεω,
        Conjugation::Ἁπτω,
        Conjugation::Ἐλπω,
        Conjugation::Κτῐζω,
        Conjugation::Νῑκαω,
        Conjugation::Πᾰραιτεομαι,
        Conjugation::Πορσῡνω,
        Conjugation::Πεμπω,
        Conjugation::Θωᾰω,
        Conjugation::Πειρᾰζω,
        Conjugation::Ἀνῐστημῐ,
        Conjugation::Λογῐζομαι,
        Conjugation::Σκᾰνδᾰλῐζω,
        Conjugation::Μνημονευω,
        Conjugation::Ἀειρω,
        Conjugation::Ἀραομαι,
        Conjugation::Δῐοικεω,
        Conjugation::Φαινω,
        Conjugation::Σβεννῡμῐ,
        Conjugation::Ἐπᾰϊω,
        Conjugation::Κειμαι,
        Conjugation::Κᾰθιδρῡω,
        Conjugation::Ἐγειρω,
        Conjugation::Πειθω,
        Conjugation::Ἱημῐ,
        Conjugation::Μῑγνῡμῐ,
        Conjugation::Αἱρεω,
        Conjugation::Οἰομαι,
        Conjugation::Χεω,
        Conjugation::Ἀφῑημῐ,
        Conjugation::Ἐνᾰντῐοομαι,
        Conjugation::Κᾰταφρονεω,
        Conjugation::Εἰσερχομαι,
        Conjugation::Ὀμνῡμῐ,
        Conjugation::Ἀθῡρω,
        Conjugation::Ψῑλοω,
        Conjugation::Ἀνᾰγω,
        Conjugation::Φᾰρμᾰσσω,
        Conjugation::Δῐωκω,
        Conjugation::Φῠω,
        Conjugation::Ἀρδω,
        Conjugation::Πλᾰσσω,
        Conjugation::Πᾰρεχω,
        Conjugation::Κρῑνομαι,
        Conjugation::Πῠνθανομαι,
        Conjugation::Εἰμῐ,
        Conjugation::Πρεσβευω,
        Conjugation::Σῳζω,
        Conjugation::Σφᾰλλω,
        Conjugation::Καθιζω,
        Conjugation::Ἀπειπον,
        Conjugation::Χραω,
        Conjugation::Λῡω,
        Conjugation::Θλαω,
        Conjugation::Ἐπῐλᾰμβᾰνω,
        Conjugation::Τελεω,
        Conjugation::Μωραινω,
        Conjugation::Κατοικεω,
        Conjugation::Λᾰγοθηρεω,
        Conjugation::Σειω,
        Conjugation::Μηκᾰομαι,
        Conjugation::Ἐπεξειμι,
        Conjugation::Ἐαω,
        Conjugation::Περιειμι,
        Conjugation::Ἡκω,
        Conjugation::Ἱζω,
        Conjugation::Φρᾰζω,
        Conjugation::Πρεπω,
        Conjugation::Στερεω,
        Conjugation::Μᾰρτῠρεω,
        Conjugation::Ἐλπῐζω,
        Conjugation::Ἀναιρεω,
        Conjugation::Σῠνοικῐζω,
        Conjugation::Θρῠπτω,
        Conjugation::Τρεφω,
        Conjugation::Μετερχομαι,
        Conjugation::Ἀποκωλῡω,
        Conjugation::Σκηπτω,
        Conjugation::Εἰδομαι,
        Conjugation::Βᾰθῠνω,
        Conjugation::Ἐξᾰκολουθεω,
        Conjugation::Προκᾰλεω,
        Conjugation::Περαινω,
        Conjugation::Ἡγεομαι,
        Conjugation::Βαινω,
        Conjugation::Τῠπτω,
        Conjugation::Ἐπᾰγγελλω,
        Conjugation::Ἀμειβω,
        Conjugation::Προσφερω,
        Conjugation::Ἐπιτρεχω,
        Conjugation::Σπᾰω,
        Conjugation::Σῠνᾰγω,
        Conjugation::Σᾰττω,
        Conjugation::Ἐργᾰζομαι,
        Conjugation::Ἐρᾰμαι,
        Conjugation::Ἀνᾰπτω,
        Conjugation::Δῡω,
        Conjugation::Πειρᾰω,
        Conjugation::Πᾰρᾰλλᾰσσω,
        Conjugation::Κᾰτειργω,
        Conjugation::Στελλω,
        Conjugation::Πᾰροξῡνω,
        Conjugation::Ἐλᾰσσοω,
        Conjugation::Ἐρωτᾰω,
        Conjugation::Πᾰρᾰγῐγνομαι,
        Conjugation::Κρουω,
        Conjugation::Δημοκρᾰτεομαι,
        Conjugation::Ἀνᾰλᾰμβᾰνω,
        Conjugation::Γυμνοω,
        Conjugation::Δῐαιρω,
        Conjugation::Προβαλλω,
        Conjugation::Ἀφορῐζω,
        Conjugation::Κᾰκοω,
        Conjugation::Πῑαινω,
        Conjugation::Θερᾰπευω,
        Conjugation::Ἀποζω,
        Conjugation::Πλᾰναω,
        Conjugation::Δῐᾰρπᾰζω,
        Conjugation::Ὀμνῠω,
        Conjugation::Ψᾱω,
        Conjugation::Ἐπῐτῐθημῐ,
        Conjugation::Ἐπαινεω,
        Conjugation::Λιστραω,
        Conjugation::Φλῡω,
        Conjugation::Ἐπευχομαι,
        Conjugation::Ἐφελκω,
        Conjugation::Τρῑβω,
        Conjugation::Ἀντεχω,
        Conjugation::Ποιμαινω,
        Conjugation::Σῠνιστημῐ,
        Conjugation::Ἰαινω,
        Conjugation::Ἐφῐστημῐ,
        Conjugation::Δρᾰσσομαι,
        Conjugation::Πᾰρᾰκᾰλεω,
        Conjugation::Ἐξαιρω,
        Conjugation::Δῐᾰλῡω,
        Conjugation::Ἁμᾰρτᾰνω,
        Conjugation::Κᾰμπτω,
        Conjugation::Μεθῐστημῐ,
        Conjugation::Ἐκφερω,
        Conjugation::Ἐξετᾰζω,
        Conjugation::Σῠναιρεω,
        Conjugation::Φῡρω,
        Conjugation::Σταθμαω,
        Conjugation::Κνᾰω,
        Conjugation::Δηλεομαι,
        Conjugation::Λευκαινω,
        Conjugation::Σῠνδεω,
        Conjugation::Κηδευω,
        Conjugation::Λεγομαι,
        Conjugation::Ῥεω,
        Conjugation::Δῐᾰσῳζω,
        Conjugation::Κᾰτᾰβῐβᾰζω,
        Conjugation::Ῥηγνῡμῐ,
        Conjugation::Ὠνεομαι,
        Conjugation::Κᾰτᾰπονεω,
        Conjugation::Κηρῡσσω,
        Conjugation::Ἐπορον,
        Conjugation::Εἰδον,
        Conjugation::Καινῠμαι,
        Conjugation::Ξυμπολεμεω,
        Conjugation::Δῐακειμαι,
        Conjugation::Ξῠνοιδᾰ,
        Conjugation::Συγκειμαι,
        Conjugation::Νισσομαι,
        Conjugation::Κᾰτασκηνοω,
        Conjugation::Γεγονοι,
        Conjugation::Ἐπῐχειρεω,
        Conjugation::Συνεπομαι,
        Conjugation::Λαλαγεω,
        Conjugation::Ξυνναυμᾰχεω,
        Conjugation::Μετᾰκᾰλεω,
        Conjugation::Επῐλῡω,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 287] = [
    // Ἐχω: 235 words (θελω, τῠγχᾰνω, δῐδᾰσκω, πῑνω, εὑρῐσκω, πληθω).
    conjugation!("ἐχω", 235, "~ω";
    stem future_pass: "~ηθησ- like ἐχω";
    stem aorist_ind: "~ησ- like ἐχω";
    stem aorist_ind_pass: "~ηθην- like ἐχω";
    stem aorist_pass: "~ηθ- like ἐχω";
    stem pluperfect_act: "~ηκειν- like ἐχω";
    ind_act_1: "~ω", _, "~ομεν";
    ind_act_2: "~εις", "~ετον", _;
    ind_act_3: "~ει", _, "~ουσῐ|~ουσῐν";
    ind_mid_1: _, _, "~ομεθᾰ";
    ind_mid_2: "~ῃ", _, _;
    ind_mid_3: _, "~εσθον", _;
    ind_pass_1: "~ομαι", _, _;
    ind_pass_2: _, "~εσθον", "~εσθε";
    ind_pass_3: "~εται", "~εσθον", "~ονται";
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", _, _;
    subj_act_3: "~ῃ", "~ητον", "~ωσῐ|~ωσῐν";
    subj_mid_1: _, _, "~ωμεθᾰ";
    subj_mid_2: _, "~ησθον", _;
    subj_mid_3: "~ηται", _, "~ωνται";
    subj_pass_1: "~ωμαι", _, _;
    subj_pass_2: _, _, "~ησθε";
    opt_act_1: "~οιμῐ", _, "~οιμεν";
    opt_act_2: "~οις", "~οιτον", _;
    opt_act_3: "~οι", "~οιτην", "~οιεν";
    opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    opt_mid_2: "~οιο", _, "~οισθε";
    opt_mid_3: "~οιτο", "~οισθην", "~οιντο";
    opt_pass_2: _, "~οισθον", _;
    imp_act_2: "~ε", _, "~ετε";
    imp_act_3: _, _, "~οντων";
    imp_mid_2: "~ου", "~εσθον", "~εσθε";
    imp_mid_3: _, _, "~εσθων";
    imp_pass_2: _, "~εσθον", "~εσθε";
    imp_pass_3: "~εσθω", "~εσθων", "~εσθων";
    imperfect_act_1: "ἐ~ον", _, _;
    imperfect_act_2: "ἐ~ες", "ἐ~ετον", "ἐ~ετε";
    imperfect_act_3: "ἐ~ε", _, "ἐ~ον";
    imperfect_mid_1: "ἐ~ομην", _, "ἐ~ομεθᾰ";
    imperfect_mid_2: "ἐ~ου", _, _;
    imperfect_mid_3: "ἐ~ετο", _, "ἐ~οντο";
    imperfect_pass_2: _, "ἐ~εσθον", "ἐ~εσθε";
    imperfect_pass_3: _, "ἐ~εσθην", _;
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", _;
    future_ind_act_3: _, _, "~ησουσῐ";
    future_ind_mid_1: "~ησομαι", _, _;
    future_ind_mid_2: "~ησῃ", "~ησεσθον", _;
    future_ind_mid_3: "~ησεται", _, _;
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: "~ησοις", "~ησοιτον", _;
    future_opt_act_3: "~ησοι", "~ησοιτην", _;
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ησοισθην", _;
    aorist_subj_act_1: "~ησω", _, _;
    aorist_subj_act_2: "~ησῃς", "~ησητον", _;
    aorist_subj_mid_1: "~ησωμαι", _, _;
    aorist_subj_mid_2: "~ησῃ", "~ησησθον", _;
    aorist_opt_act_1: "~ησαιμῐ", _, "~ησαιμεν";
    aorist_opt_act_2: "~ησειᾰς", _, _;
    aorist_opt_act_3: "~ησειε", _, "~ησειᾰν";
    aorist_opt_mid_1: "~ησαιμην", _, _;
    aorist_imp_act_2: "~ησον", _, "~ησᾰτε";
    aorist_imp_act_3: _, _, "~ησᾰντων";
    aorist_imp_mid_2: "~ησαι", "~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ησᾰσθων";
    perfect_ind_act_1: "%ε~ηκᾰ", _, "%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ηκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ησαι", "%ε~ησθον", _;
    perfect_ind_mid_3: "%ε~ηται", _, "%ε~ηνται";
    perfect_ind_pass_1: "%ε~ημαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ησθε";
    perfect_subj_act_1: "%ε~ηκω", _, _;
    perfect_subj_act_2: "%ε~ηκῃς", "%ε~ηκητον", "%ε~ηκητε";
    perfect_subj_act_3: "%ε~ηκῃ", _, "%ε~ηκωσῐ";
    perfect_imp_act_2: "%ε~ηκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~ησθε";
    perfect_imp_mid_3: _, _, "%ε~ησθων";
    perfect_imp_pass_2: "%ε~ησο", _, "%ε~ησθε";
    perfect_imp_pass_3: "%ε~ησθω", "%ε~ησθων", "%ε~ησθων";
    pluperfect_mid_1: "ἐ%ε~ημην", _, _;
    pluperfect_mid_2: "ἐ%ε~ησο", "ἐ%ε~ησθον", _;
    pluperfect_mid_3: "ἐ%ε~ητο", _, "ἐ%ε~ηντο";
    pluperfect_pass_1: _, _, "ἐ%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~ησθην", _;
    future_perfect_ind_mid_1: "%ε~ησομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~ησεσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~ησομεθᾰ";
    future_perfect_ind_pass_2: "%ε~ησῃ", "%ε~ησεσθον", "%ε~ησεσθε";
    future_perfect_ind_pass_3: "%ε~ησεται", "%ε~ησεσθον", "%ε~ησονται";
    future_perfect_opt_pass_1: "%ε~ησοιμην", _, "%ε~ησοιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~ησοιο", "%ε~ησοισθον", "%ε~ησοισθε";
    future_perfect_opt_pass_3: "%ε~ησοιτο", "%ε~ησοισθην", "%ε~ησοιντο";
    infinitive_pres: "~ειν", _, "~εσθαι";
    infinitive_fut: "~ησειν", _, _;
    infinitive_aor: "~ησαι", _, _;
    infinitive_perf: _, "%ε~ησθαι", _;
    infinitive_futp: _, _, "%ε~ησεσθαι";
    participle_pres: "~ων", _, "~ομενος";
    participle_aor: "~ησᾱς", "~ησᾰμενος", _;
    participle_perf: "%ε~ηκως", _, "%ε~ημενος";
    participle_futp: _, _, "%ε~ησομενος";
    ),
    // Ποιεω: 204 words (φοβεω, βλασφημεω, γᾰμεω, μῑσεω, διακονεω, ποιεω). Like Ἐχω, except in 57 cells: ind_act_1 pl, ind_act_2 du, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_1 sg, ….
    conjugation!("ποιεω", 204, "~εω", like Ἐχω;
    stem aorist_ind: "~ησ- like ποιεω";
    stem aorist_pass: "~ηθ- like ποιεω";
    ind_act_1: _, _, "~ουμεν";
    ind_act_2: _, "~ειτον", _;
    ind_mid_1: _, _, "~ουμεθᾰ";
    ind_mid_2: "~ει|~ῃ", _, _;
    ind_mid_3: _, "~εισθον", _;
    ind_pass_1: "~ουμαι", _, _;
    ind_pass_2: _, "~εισθον", "~εισθε";
    ind_pass_3: "~ειται", "~εισθον", "~ουνται";
    opt_act_1: "~οιην|~οιμῐ", _, "~οιμεν|~οιημεν";
    opt_act_2: "~οιης|~οις", "~οιτον|~οιητον", "~οιτε|~οιητε";
    opt_act_3: "~οιη|~οι", "~οιτην|~οιητην", "~οιεν|~οιησᾰν";
    imp_act_2: "~ει", _, "~ειτε";
    imp_act_3: _, _, "~ουντων";
    imp_mid_2: _, "~εισθον", "~εισθε";
    imp_mid_3: _, _, "~εισθων";
    imp_pass_2: _, "~εισθον", "~εισθε";
    imp_pass_3: "~εισθω", "~εισθων", "~εισθων";
    imperfect_act_1: "ἐ~ουν", _, _;
    imperfect_act_2: "ἐ~εις", "ἐ~ειτον", "ἐ~ειτε";
    imperfect_act_3: "ἐ~ει", _, "ἐ~ουν";
    imperfect_mid_1: "ἐ~ουμην", _, "ἐ~ουμεθᾰ";
    imperfect_mid_3: "ἐ~ειτο", _, "ἐ~ουντο";
    imperfect_pass_2: _, "ἐ~εισθον", "ἐ~εισθε";
    imperfect_pass_3: _, "ἐ~εισθην", _;
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    future_ind_mid_2: "~ησῃ|~ησει", _, _;
    aorist_subj_act_3: _, _, "~ησωσῐ|~ησωσῐν";
    aorist_opt_act_2: "~ησειᾰς|~ησαις", _, _;
    aorist_opt_act_3: "~ησειε|~ησειεν|~ησαι", _, "~ησειᾰν|~ησαιεν";
    perfect_ind_act_3: "%ε~ηκε|%ε~ηκεν", _, "%ε~ηκᾱσῐ|%ε~ηκᾱσῐν";
    perfect_subj_act_3: _, _, "%ε~ηκωσῐ|%ε~ηκωσῐν";
    perfect_opt_act_1: "%ε~ηκοιμῐ|%ε~ηκοιην", _, _;
    perfect_opt_act_2: "%ε~ηκοις|%ε~ηκοιης", _, _;
    perfect_opt_act_3: "%ε~ηκοι|%ε~ηκοιη", _, _;
    infinitive_pres: _, _, "~εισθαι";
    participle_pres: _, _, "~ουμενος";
    ),
    // Βουλευω: 122 words (γρᾰφω, θερᾰπευω, ψῡχω, ἀρχω, πῐστευω, αἰθω). Like Ἐχω, except in 98 cells: ind_mid_2 sg, imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, ….
    conjugation!("βουλευω", 122, "~ω", like Ἐχω;
    stem future_pass: "~θησ- like ἐχω";
    stem aorist_ind: "~σ- like ποιεω";
    stem aorist_ind_pass: "~θην- like ἐχω";
    stem aorist_pass: "~θ- like ἐχω";
    stem pluperfect_act: "~κειν- like ἐχω";
    ind_mid_2: "~ῃ|~ει", _, _;
    imperfect_act_3: "ἐ~ε|ἐ~εν", _, _;
    future_ind_act_1: "~σω", _, "~σομεν";
    future_ind_act_2: "~σεις", "~σετον", _;
    future_ind_act_3: _, _, "~σουσῐ|~σουσῐν";
    future_ind_mid_1: "~σομαι", _, _;
    future_ind_mid_2: "~σῃ|~σει", "~σεσθον", _;
    future_ind_mid_3: "~σεται", _, _;
    future_opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    future_opt_act_2: "~σοις", "~σοιτον", _;
    future_opt_act_3: "~σοι", "~σοιτην", _;
    future_opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    future_opt_mid_3: _, "~σοισθην", _;
    aorist_subj_act_1: "~σω", _, _;
    aorist_subj_act_2: "~σῃς", "~σητον", _;
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_subj_mid_1: "~σωμαι", _, _;
    aorist_subj_mid_2: "~σῃ", "~σησθον", _;
    aorist_opt_act_1: "~σαιμῐ", _, "~σαιμεν";
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", _, "~σειᾰν|~σαιεν";
    aorist_opt_mid_1: "~σαιμην", _, _;
    aorist_imp_act_2: "~σον", _, "~σᾰτε";
    aorist_imp_act_3: _, _, "~σᾰντων";
    aorist_imp_mid_2: "~σαι", "~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~σᾰσθων";
    perfect_ind_act_1: "%ε~κᾰ", _, "%ε~κᾰμεν";
    perfect_ind_act_2: _, "%ε~κᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~κᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~μεθᾰ";
    perfect_ind_mid_2: "%ε~σαι", "%ε~σθον", _;
    perfect_ind_mid_3: "%ε~ται", _, "%ε~νται";
    perfect_ind_pass_1: "%ε~μαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~σθε";
    perfect_subj_act_1: "%ε~κω", _, _;
    perfect_subj_act_2: "%ε~κῃς", "%ε~κητον", "%ε~κητε";
    perfect_subj_act_3: "%ε~κῃ", _, "%ε~κωσῐ";
    perfect_imp_act_2: "%ε~κε", _, _;
    perfect_imp_mid_2: _, _, "%ε~σθε";
    perfect_imp_mid_3: _, _, "%ε~σθων";
    perfect_imp_pass_2: "%ε~σο", _, "%ε~σθε";
    perfect_imp_pass_3: "%ε~σθω", "%ε~σθων", "%ε~σθων";
    pluperfect_mid_1: "ἐ%ε~μην", _, _;
    pluperfect_mid_2: "ἐ%ε~σο", "ἐ%ε~σθον", _;
    pluperfect_mid_3: "ἐ%ε~το", _, "ἐ%ε~ντο";
    pluperfect_pass_1: _, _, "ἐ%ε~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~σθε";
    pluperfect_pass_3: _, "ἐ%ε~σθην", _;
    future_perfect_ind_mid_1: derived, _, _;
    future_perfect_ind_mid_3: _, derived, _;
    future_perfect_ind_pass_1: _, _, derived;
    future_perfect_ind_pass_2: derived, derived, derived;
    future_perfect_ind_pass_3: derived, derived, derived;
    future_perfect_opt_pass_1: derived, _, derived;
    future_perfect_opt_pass_2: derived, derived, derived;
    future_perfect_opt_pass_3: derived, derived, derived;
    infinitive_fut: "~σειν", _, _;
    infinitive_aor: "~σαι", _, _;
    infinitive_perf: _, "%ε~σθαι", _;
    infinitive_futp: _, _, derived;
    participle_aor: "~σᾱς", "~σᾰμενος", _;
    participle_perf: "%ε~κως", _, "%ε~μενος";
    participle_futp: _, _, derived;
    ),
    // Γῡροω: 80 words (ὁμοιοω, μορφοω, ζηλοω, ναρκοω, κοινοω, ἀλλοιοω). Like Βουλευω, except in 131 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_mid_1 pl, ind_mid_2 sg, ….
    conjugation!("γῡροω", 80, "~οω", like Βουλευω;
    stem future_pass: "~ωθησ- like ἐχω";
    stem aorist_ind: "~ωσ- like ποιεω";
    stem aorist_ind_pass: "~ωθην- like ἐχω";
    stem aorist_pass: "~ωθ- like ἐχω";
    stem pluperfect_act: "~ωκειν- like ἐχω";
    ind_act_1: _, _, "~ουμεν";
    ind_act_2: "~οις", "~ουτον", _;
    ind_act_3: "~οι", _, _;
    ind_mid_1: _, _, "~ουμεθᾰ";
    ind_mid_2: "~οι", _, _;
    ind_mid_3: _, "~ουσθον", _;
    ind_pass_1: "~ουμαι", _, _;
    ind_pass_2: _, "~ουσθον", "~ουσθε";
    ind_pass_3: "~ουται", "~ουσθον", "~ουνται";
    subj_act_2: "~οις", _, _;
    subj_act_3: "~οι", "~ωτον", _;
    subj_mid_2: _, "~ωσθον", _;
    subj_mid_3: "~ωται", _, _;
    subj_pass_2: _, _, "~ωσθε";
    opt_act_1: "~οιην|~οιμῐ", _, "~οιμεν|~οιημεν";
    opt_act_2: "~οιης|~οις", "~οιτον|~οιητον", "~οιτε|~οιητε";
    opt_act_3: "~οιη|~οι", "~οιτην|~οιητην", "~οιεν|~οιησᾰν";
    imp_act_2: "~ου", _, "~ουτε";
    imp_act_3: _, _, "~ουντων";
    imp_mid_2: _, "~ουσθον", "~ουσθε";
    imp_mid_3: _, _, "~ουσθων";
    imp_pass_2: _, "~ουσθον", "~ουσθε";
    imp_pass_3: "~ουσθω", "~ουσθων", "~ουσθων";
    imperfect_act_1: "ἐ~ουν", _, _;
    imperfect_act_2: "ἐ~ους", "ἐ~ουτον", "ἐ~ουτε";
    imperfect_act_3: "ἐ~ου", _, "ἐ~ουν";
    imperfect_mid_1: "ἐ~ουμην", _, "ἐ~ουμεθᾰ";
    imperfect_mid_3: "ἐ~ουτο", _, "ἐ~ουντο";
    imperfect_pass_2: _, "ἐ~ουσθον", "ἐ~ουσθε";
    imperfect_pass_3: _, "ἐ~ουσθην", _;
    future_ind_act_1: "~ωσω", _, "~ωσομεν";
    future_ind_act_2: "~ωσεις", "~ωσετον", _;
    future_ind_act_3: _, _, "~ωσουσῐ";
    future_ind_mid_1: "~ωσομαι", _, _;
    future_ind_mid_2: "~ωσῃ|~ωσει", "~ωσεσθον", _;
    future_ind_mid_3: "~ωσεται", _, _;
    future_opt_act_1: "~ωσοιμῐ", _, "~ωσοιμεν";
    future_opt_act_2: "~ωσοις", "~ωσοιτον", _;
    future_opt_act_3: "~ωσοι", "~ωσοιτην", _;
    future_opt_mid_1: "~ωσοιμην", _, "~ωσοιμεθᾰ";
    future_opt_mid_3: _, "~ωσοισθην", _;
    aorist_subj_act_1: "~ωσω", _, _;
    aorist_subj_act_2: "~ωσῃς", "~ωσητον", _;
    aorist_subj_act_3: _, _, "~ωσωσῐ|~ωσωσῐν";
    aorist_subj_mid_1: "~ωσωμαι", _, _;
    aorist_subj_mid_2: "~ωσῃ", "~ωσησθον", _;
    aorist_opt_act_1: "~ωσαιμῐ", _, "~ωσαιμεν";
    aorist_opt_act_2: "~ωσειᾰς|~ωσαις", _, _;
    aorist_opt_act_3: "~ωσειε|~ωσειεν|~ωσαι", _, "~ωσειᾰν|~ωσαιεν";
    aorist_opt_mid_1: "~ωσαιμην", _, _;
    aorist_imp_act_2: "~ωσον", _, "~ωσᾰτε";
    aorist_imp_act_3: _, _, "~ωσᾰντων";
    aorist_imp_mid_2: "~ωσαι", "~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ωσᾰσθων";
    perfect_ind_act_1: "%ε~ωκᾰ", _, "%ε~ωκᾰμεν";
    perfect_ind_act_2: _, "%ε~ωκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ωκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~ωμεθᾰ";
    perfect_ind_mid_2: "%ε~ωσαι", "%ε~ωσθον", _;
    perfect_ind_mid_3: "%ε~ωται", _, "%ε~ωνται";
    perfect_ind_pass_1: "%ε~ωμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ωσθε";
    perfect_subj_act_1: "%ε~ωκω", _, _;
    perfect_subj_act_2: "%ε~ωκῃς", "%ε~ωκητον", "%ε~ωκητε";
    perfect_subj_act_3: "%ε~ωκῃ", _, "%ε~ωκωσῐ";
    perfect_imp_act_2: "%ε~ωκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~ωσθε";
    perfect_imp_mid_3: _, _, "%ε~ωσθων";
    perfect_imp_pass_2: "%ε~ωσο", _, "%ε~ωσθε";
    perfect_imp_pass_3: "%ε~ωσθω", "%ε~ωσθων", "%ε~ωσθων";
    pluperfect_mid_1: "ἐ%ε~ωμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ωσο", "ἐ%ε~ωσθον", _;
    pluperfect_mid_3: "ἐ%ε~ωτο", _, "ἐ%ε~ωντο";
    pluperfect_pass_1: _, _, "ἐ%ε~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ωσθε";
    pluperfect_pass_3: _, "ἐ%ε~ωσθην", _;
    infinitive_pres: "~ουν", _, "~ουσθαι";
    infinitive_fut: "~ωσειν", _, _;
    infinitive_aor: "~ωσαι", _, _;
    infinitive_perf: _, "%ε~ωσθαι", _;
    participle_pres: _, _, "~ουμενος";
    participle_aor: "~ωσᾱς", "~ωσᾰμενος", _;
    participle_perf: "%ε~ωκως", _, "%ε~ωμενος";
    ),
    // Χρημᾰτῐζω: 63 words (κᾰθᾰρῐζω, βᾰσᾰνῐζω, κᾰπνῐζω, φενᾱκῐζω, γεμῐζω, ἱζω). Like Βουλευω, except in 108 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("χρημᾰτῐζω", 63, "~ζω", like Βουλευω;
    stem future_pass: "~σθησ- like χρημᾰτῐζω";
    stem aorist_ind_pass: "~σθην- like ἐχω";
    stem aorist_pass: "~σθ- like ποιεω";
    stem pluperfect_act: "~κ- like χρημᾰτῐζω";
    ind_act_1: "~ζω", _, "~ζομεν";
    ind_act_2: "~ζεις", "~ζετον", _;
    ind_act_3: "~ζει", _, "~ζουσῐ|~ζουσῐν";
    ind_mid_1: _, _, "~ζομεθᾰ";
    ind_mid_2: "~ζῃ|~ζει", _, _;
    ind_mid_3: _, "~ζεσθον", _;
    ind_pass_1: "~ζομαι", _, _;
    ind_pass_2: _, "~ζεσθον", "~ζεσθε";
    ind_pass_3: "~ζεται", "~ζεσθον", "~ζονται";
    subj_act_1: "~ζω", _, "~ζωμεν";
    subj_act_2: "~ζῃς", _, _;
    subj_act_3: "~ζῃ", "~ζητον", "~ζωσῐ|~ζωσῐν";
    subj_mid_1: _, _, "~ζωμεθᾰ";
    subj_mid_2: _, "~ζησθον", _;
    subj_mid_3: "~ζηται", _, "~ζωνται";
    subj_pass_1: "~ζωμαι", _, _;
    subj_pass_2: _, _, "~ζησθε";
    opt_act_1: "~ζοιμῐ", _, "~ζοιμεν";
    opt_act_2: "~ζοις", "~ζοιτον", _;
    opt_act_3: "~ζοι", "~ζοιτην", "~ζοιεν";
    opt_mid_1: "~ζοιμην", _, "~ζοιμεθᾰ";
    opt_mid_2: "~ζοιο", _, "~ζοισθε";
    opt_mid_3: "~ζοιτο", "~ζοισθην", "~ζοιντο";
    opt_pass_2: _, "~ζοισθον", _;
    imp_act_2: "~ζε", _, "~ζετε";
    imp_act_3: _, _, "~ζοντων";
    imp_mid_2: "~ζου", "~ζεσθον", "~ζεσθε";
    imp_mid_3: _, _, "~ζεσθων";
    imp_pass_2: _, "~ζεσθον", "~ζεσθε";
    imp_pass_3: "~ζεσθω", "~ζεσθων", "~ζεσθων";
    imperfect_act_1: "ἐ~ζον", _, _;
    imperfect_act_2: "ἐ~ζες", "ἐ~ζετον", "ἐ~ζετε";
    imperfect_act_3: "ἐ~ζε|ἐ~ζεν", _, "ἐ~ζον";
    imperfect_mid_1: "ἐ~ζομην", _, "ἐ~ζομεθᾰ";
    imperfect_mid_2: "ἐ~ζου", _, _;
    imperfect_mid_3: "ἐ~ζετο", _, "ἐ~ζοντο";
    imperfect_pass_2: _, "ἐ~ζεσθον", "ἐ~ζεσθε";
    imperfect_pass_3: _, "ἐ~ζεσθην", _;
    future_ind_act_1: "~ω", _, "~ουμεν";
    future_ind_act_2: "~εις", "~ειτον", _;
    future_ind_act_3: _, _, "~ουσῐ|~ουσῐν";
    future_ind_mid_1: "~ουμαι", _, _;
    future_ind_mid_2: "~ει|~ῃ", "~εισθον", _;
    future_ind_mid_3: "~ειται", _, _;
    future_opt_act_1: "~οιην|~οιμῐ", _, "~οιμεν|~οιημεν";
    future_opt_act_2: "~οιης|~οις", "~οιτον|~οιητον", "~οιτε|~οιητε";
    future_opt_act_3: "~οιη|~οι", "~οιτην|~οιητην", "~οιεν|~οιησᾰν";
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_3: _, "~οισθην", _;
    perfect_ind_act_3: "%ε~κε|%ε~κεν", _, "%ε~κᾱσῐ|%ε~κᾱσῐν";
    perfect_ind_mid_1: _, _, "%ε~σμεθᾰ";
    perfect_ind_mid_3: "%ε~σται", _, "%ε~δᾰται";
    perfect_ind_pass_1: "%ε~σμαι", _, _;
    perfect_subj_act_3: _, _, "%ε~κωσῐ|%ε~κωσῐν";
    perfect_opt_act_1: "%ε~κοιμῐ|%ε~κοιην", _, _;
    perfect_opt_act_2: "%ε~κοις|%ε~κοιης", _, _;
    perfect_opt_act_3: "%ε~κοι|%ε~κοιη", _, _;
    pluperfect_mid_1: "ἐ%ε~σμην", _, _;
    pluperfect_mid_3: "ἐ%ε~στο", _, "ἐ%ε~δᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~σμεθᾰ";
    infinitive_pres: "~ζειν", _, "~ζεσθαι";
    infinitive_fut: "~ειν", _, _;
    participle_pres: "~ζων", _, "~ζομενος";
    participle_perf: _, _, "%ε~σμενος";
    ),
    // Ἑπομαι: 61 words (ἐρχομαι, γῐγνομαι, δεχομαι, δερκομαι, μεμφομαι, μᾰντευομαι). Like Βουλευω, except in 79 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἑπομαι", 61, "~ομαι", like Βουλευω;
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
    opt_act_2: derived, derived, _;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, _;
    future_ind_act_3: _, _, derived;
    future_ind_mid_2: "~σῃ", _, _;
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
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_imp_act_2: derived, _, _;
    pluperfect_act_1: derived, _, _;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    participle_pres: derived, _, _;
    participle_aor: derived, _, _;
    participle_perf: derived, _, _;
    ),
    // Βᾰπτῐζω: 60 words (πυρρᾰζω, βᾰπτῐζω, δᾰμαζω, σῐζω, εὐᾱζω, ἐξῠπνῐζω). Like Χρημᾰτῐζω, except in 63 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("βᾰπτῐζω", 60, "~ζω", like Χρημᾰτῐζω;
    stem aorist_ind: "~σ- like ἐχω";
    stem pluperfect_act: "~κειν- like ἐχω";
    ind_act_3: _, _, "~ζουσῐ";
    ind_mid_2: "~ζῃ", _, _;
    subj_act_3: _, _, "~ζωσῐ";
    imperfect_act_3: "ἐ~ζε", _, _;
    future_ind_act_1: "~σω", _, "~σομεν";
    future_ind_act_2: "~σεις", "~σετον", _;
    future_ind_act_3: _, _, "~σουσῐ";
    future_ind_mid_1: "~σομαι", _, _;
    future_ind_mid_2: "~σῃ", "~σεσθον", _;
    future_ind_mid_3: "~σεται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    future_opt_act_2: "~σοις", "~σοιτον", derived;
    future_opt_act_3: "~σοι", "~σοιτην", derived;
    future_opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    future_opt_mid_3: _, "~σοισθην", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_2: "~σειᾰς", _, _;
    aorist_opt_act_3: "~σειε", _, "~σειᾰν";
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: derived, _, "%ε~κᾱσῐ";
    perfect_subj_act_3: _, _, "%ε~κωσῐ";
    perfect_opt_act_1: derived, _, _;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: derived, _, _;
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
    infinitive_fut: "~σειν", _, _;
    infinitive_futp: _, _, "%ε~σεσθαι";
    participle_futp: _, _, "%ε~σομενος";
    ),
    // Δοκᾰω: 54 words (ἀγᾰπᾰω, γεννᾰω, κῠβερνᾰω, γλᾰγᾰω, σῑγᾰω, κᾰκκᾰω). Like Ἐχω, except in 70 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ind_mid_1 pl, ….
    conjugation!("δοκᾰω", 54, "~ᾰω", like Ἐχω;
    stem aorist_ind: "~ησ- like ποιεω";
    ind_act_1: _, _, "~ωμεν";
    ind_act_2: "~ᾳς", "~ᾱτον", _;
    ind_act_3: "~ᾳ", _, "~ωσῐ|~ωσῐν";
    ind_mid_1: _, _, "~ωμεθᾰ";
    ind_mid_2: "~ᾳ", _, _;
    ind_mid_3: _, "~ᾱσθον", _;
    ind_pass_1: "~ωμαι", _, _;
    ind_pass_2: _, "~ᾱσθον", "~ᾱσθε";
    ind_pass_3: "~ᾱται", "~ᾱσθον", "~ωνται";
    subj_act_2: "~ᾳς", _, _;
    subj_act_3: "~ᾳ", "~ᾱτον", _;
    subj_mid_2: _, "~ᾱσθον", _;
    subj_mid_3: "~ᾱται", _, _;
    subj_pass_2: _, _, "~ᾱσθε";
    opt_act_1: "~ῳην|~ῳμῐ", _, "~ῳμεν|~ῳημεν";
    opt_act_2: "~ῳης|~ῳς", "~ῳτον|~ῳητον", "~ῳτε|~ῳητε";
    opt_act_3: "~ῳη|~ῳ", "~ῳτην|~ῳητην", "~ῳεν|~ῳησᾰν";
    opt_mid_1: "~ῳμην", _, "~ῳμεθᾰ";
    opt_mid_2: "~ῳο", _, "~ῳσθε";
    opt_mid_3: "~ῳτο", "~ῳσθην", "~ῳντο";
    opt_pass_2: _, "~ῳσθον", _;
    imp_act_2: "~ᾱ", _, "~ᾱτε";
    imp_act_3: _, _, "~ωντων";
    imp_mid_2: "~ω", "~ᾱσθον", "~ᾱσθε";
    imp_mid_3: _, _, "~ᾱσθων";
    imp_pass_2: _, "~ᾱσθον", "~ᾱσθε";
    imp_pass_3: "~ᾱσθω", "~ᾱσθων", "~ᾱσθων";
    imperfect_act_1: "ἐ~ων", _, _;
    imperfect_act_2: "ἐ~ᾱς", "ἐ~ᾱτον", "ἐ~ᾱτε";
    imperfect_act_3: "ἐ~ᾱ", _, "ἐ~ων";
    imperfect_mid_1: "ἐ~ωμην", _, "ἐ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ω", _, _;
    imperfect_mid_3: "ἐ~ᾱτο", _, "ἐ~ωντο";
    imperfect_pass_2: _, "ἐ~ᾱσθον", "ἐ~ᾱσθε";
    imperfect_pass_3: _, "ἐ~ᾱσθην", _;
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    aorist_subj_act_3: _, _, "~ησωσῐ|~ησωσῐν";
    aorist_opt_act_2: "~ησειᾰς|~ησαις", _, _;
    aorist_opt_act_3: "~ησειε|~ησειεν|~ησαι", _, "~ησειᾰν|~ησαιεν";
    infinitive_pres: "~ᾱν", _, "~ᾱσθαι";
    participle_pres: _, _, "~ωμενος";
    ),
    // Δῐκεω: 49 words (ἀγᾰθοποιεω, ἀγνοεω, ἀντλεω, ἀσκεω, οἰκεω, ἀλεω). Like Ποιεω, except in 59 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_ind_mid_1 pl, ….
    conjugation!("δῐκεω", 49, "~εω", like Ποιεω;
    stem future_pass: "~ηθησ- like χρημᾰτῐζω";
    stem pluperfect_act: "~ηκ- like δῐκεω";
    perfect_ind_act_1: "ἐ~ηκᾰ", _, "ἐ~ηκᾰμεν";
    perfect_ind_act_2: _, "ἐ~ηκᾰτον", _;
    perfect_ind_act_3: "ἐ~ηκε|ἐ~ηκεν", _, "ἐ~ηκᾱσῐ|ἐ~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "ἐ~ημεθᾰ";
    perfect_ind_mid_2: "ἐ~ησαι", "ἐ~ησθον", _;
    perfect_ind_mid_3: "ἐ~ηται", _, "ἐ~ηνται";
    perfect_ind_pass_1: "ἐ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~ησθε";
    perfect_subj_act_1: "ἐ~ηκω", _, _;
    perfect_subj_act_2: "ἐ~ηκῃς", "ἐ~ηκητον", "ἐ~ηκητε";
    perfect_subj_act_3: "ἐ~ηκῃ", _, "ἐ~ηκωσῐ|ἐ~ηκωσῐν";
    perfect_opt_act_1: "ἐ~ηκοιμῐ|ἐ~ηκοιην", _, _;
    perfect_opt_act_2: "ἐ~ηκοις|ἐ~ηκοιης", _, _;
    perfect_opt_act_3: "ἐ~ηκοι|ἐ~ηκοιη", _, _;
    perfect_imp_act_2: "ἐ~ηκε", _, _;
    perfect_imp_mid_2: _, _, "ἐ~ησθε";
    perfect_imp_mid_3: _, _, "ἐ~ησθων";
    perfect_imp_pass_2: "ἐ~ησο", _, "ἐ~ησθε";
    perfect_imp_pass_3: "ἐ~ησθω", "ἐ~ησθων", "ἐ~ησθων";
    pluperfect_mid_1: "ἐ~ημην", _, _;
    pluperfect_mid_2: "ἐ~ησο", "ἐ~ησθον", _;
    pluperfect_mid_3: "ἐ~ητο", _, "ἐ~ηντο";
    pluperfect_pass_1: _, _, "ἐ~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ησθε";
    pluperfect_pass_3: _, "ἐ~ησθην", _;
    future_perfect_ind_mid_1: derived, _, _;
    future_perfect_ind_mid_3: _, derived, _;
    future_perfect_ind_pass_1: _, _, derived;
    future_perfect_ind_pass_2: derived, derived, derived;
    future_perfect_ind_pass_3: derived, derived, derived;
    future_perfect_opt_pass_1: derived, _, derived;
    future_perfect_opt_pass_2: derived, derived, derived;
    future_perfect_opt_pass_3: derived, derived, derived;
    infinitive_perf: _, "ἐ~ησθαι", _;
    infinitive_futp: _, _, derived;
    participle_perf: "ἐ~ηκως", _, "ἐ~ημενος";
    participle_futp: _, _, derived;
    ),
    // Τᾰσσω: 32 words (ἀϊσσω, πλησσω, τᾰσσω, μᾰσσω, φῠλᾰσσω, πτῠσσω). Like Βουλευω, except in 149 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("τᾰσσω", 32, "~σσω", like Βουλευω;
    stem future_pass: "~χθησ- like ἐχω";
    stem aorist_ind: "~ξ- like ποιεω";
    stem aorist_ind_pass: "~χθην- like ἐχω";
    stem aorist_pass: "~χθ- like ἐχω";
    stem pluperfect_act: "~χειν- like ἐχω";
    ind_act_1: "~σσω", _, "~σσομεν";
    ind_act_2: "~σσεις", "~σσετον", _;
    ind_act_3: "~σσει", _, "~σσουσῐ|~σσουσῐν";
    ind_mid_1: _, _, "~σσομεθᾰ";
    ind_mid_2: "~σσῃ|~σσει", _, _;
    ind_mid_3: _, "~σσεσθον", _;
    ind_pass_1: "~σσομαι", _, _;
    ind_pass_2: _, "~σσεσθον", "~σσεσθε";
    ind_pass_3: "~σσεται", "~σσεσθον", "~σσονται";
    subj_act_1: "~σσω", _, "~σσωμεν";
    subj_act_2: "~σσῃς", _, _;
    subj_act_3: "~σσῃ", "~σσητον", "~σσωσῐ|~σσωσῐν";
    subj_mid_1: _, _, "~σσωμεθᾰ";
    subj_mid_2: _, "~σσησθον", _;
    subj_mid_3: "~σσηται", _, "~σσωνται";
    subj_pass_1: "~σσωμαι", _, _;
    subj_pass_2: _, _, "~σσησθε";
    opt_act_1: "~σσοιμῐ", _, "~σσοιμεν";
    opt_act_2: "~σσοις", "~σσοιτον", _;
    opt_act_3: "~σσοι", "~σσοιτην", "~σσοιεν";
    opt_mid_1: "~σσοιμην", _, "~σσοιμεθᾰ";
    opt_mid_2: "~σσοιο", _, "~σσοισθε";
    opt_mid_3: "~σσοιτο", "~σσοισθην", "~σσοιντο";
    opt_pass_2: _, "~σσοισθον", _;
    imp_act_2: "~σσε", _, "~σσετε";
    imp_act_3: _, _, "~σσοντων";
    imp_mid_2: "~σσου", "~σσεσθον", "~σσεσθε";
    imp_mid_3: _, _, "~σσεσθων";
    imp_pass_2: _, "~σσεσθον", "~σσεσθε";
    imp_pass_3: "~σσεσθω", "~σσεσθων", "~σσεσθων";
    imperfect_act_1: "ἐ~σσον", _, _;
    imperfect_act_2: "ἐ~σσες", "ἐ~σσετον", "ἐ~σσετε";
    imperfect_act_3: "ἐ~σσε|ἐ~σσεν", _, "ἐ~σσον";
    imperfect_mid_1: "ἐ~σσομην", _, "ἐ~σσομεθᾰ";
    imperfect_mid_2: "ἐ~σσου", _, _;
    imperfect_mid_3: "ἐ~σσετο", _, "ἐ~σσοντο";
    imperfect_pass_2: _, "ἐ~σσεσθον", "ἐ~σσεσθε";
    imperfect_pass_3: _, "ἐ~σσεσθην", _;
    future_ind_act_1: "~ξω", _, "~ξομεν";
    future_ind_act_2: "~ξεις", "~ξετον", _;
    future_ind_act_3: _, _, "~ξουσῐ|~ξουσῐν";
    future_ind_mid_1: "~ξομαι", _, _;
    future_ind_mid_2: "~ξῃ|~ξει", "~ξεσθον", _;
    future_ind_mid_3: "~ξεται", _, _;
    future_opt_act_1: "~ξοιμῐ", _, "~ξοιμεν";
    future_opt_act_2: "~ξοις", "~ξοιτον", _;
    future_opt_act_3: "~ξοι", "~ξοιτην", _;
    future_opt_mid_1: "~ξοιμην", _, "~ξοιμεθᾰ";
    future_opt_mid_3: _, "~ξοισθην", _;
    aorist_subj_act_1: "~ξω", _, _;
    aorist_subj_act_2: "~ξῃς", "~ξητον", _;
    aorist_subj_act_3: _, _, "~ξωσῐ|~ξωσῐν";
    aorist_subj_mid_1: "~ξωμαι", _, _;
    aorist_subj_mid_2: "~ξῃ", "~ξησθον", _;
    aorist_opt_act_1: "~ξαιμῐ", _, "~ξαιμεν";
    aorist_opt_act_2: "~ξειᾰς|~ξαις", _, _;
    aorist_opt_act_3: "~ξειε|~ξειεν|~ξαι", _, "~ξειᾰν|~ξαιεν";
    aorist_opt_mid_1: "~ξαιμην", _, _;
    aorist_imp_act_2: "~ξον", _, "~ξᾰτε";
    aorist_imp_act_3: _, _, "~ξᾰντων";
    aorist_imp_mid_2: "~ξαι", "~ξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ξᾰσθων";
    perfect_ind_act_1: "%ε~χᾰ", _, "%ε~χᾰμεν";
    perfect_ind_act_2: _, "%ε~χᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~χᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~γμεθᾰ";
    perfect_ind_mid_2: "%ε~ξαι", "%ε~χθον", _;
    perfect_ind_mid_3: "%ε~κται", _, "%ε~γᾰται";
    perfect_ind_pass_1: "%ε~γμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~χθε";
    perfect_subj_act_1: "%ε~χω", _, _;
    perfect_subj_act_2: "%ε~χῃς", "%ε~χητον", "%ε~χητε";
    perfect_subj_act_3: "%ε~χῃ", _, "%ε~χωσῐ";
    perfect_imp_act_2: "%ε~χε", _, _;
    perfect_imp_mid_2: _, _, "%ε~χθε";
    perfect_imp_mid_3: _, _, "%ε~χθων";
    perfect_imp_pass_2: "%ε~ξο", _, "%ε~χθε";
    perfect_imp_pass_3: "%ε~χθω", "%ε~χθων", "%ε~χθων";
    pluperfect_mid_1: "ἐ%ε~γμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ξο", "ἐ%ε~χθον", _;
    pluperfect_mid_3: "ἐ%ε~κτο", _, "ἐ%ε~γᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~γμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~χθε";
    pluperfect_pass_3: _, "ἐ%ε~χθην", _;
    infinitive_pres: "~σσειν", _, "~σσεσθαι";
    infinitive_fut: "~ξειν", _, _;
    infinitive_aor: "~ξαι", _, _;
    infinitive_perf: _, "%ε~χθαι", _;
    participle_pres: "~σσων", _, "~σσομενος";
    participle_aor: "~ξᾱς", "~ξᾰμενος", _;
    participle_perf: "%ε~χως", _, "%ε~γμενος";
    ),
    // Ληγω: 30 words (ἀγω, λεγω, φευγω, τρωγω, δῐᾰλεγω, φλεγω). Like Τᾰσσω, except in 99 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ληγω", 30, "~γω", like Τᾰσσω;
    stem aorist_ind: "~ξ- like ἐχω";
    ind_act_1: "~γω", _, "~γομεν";
    ind_act_2: "~γεις", "~γετον", _;
    ind_act_3: "~γει", _, "~γουσῐ";
    ind_mid_1: _, _, "~γομεθᾰ";
    ind_mid_2: "~γῃ", _, _;
    ind_mid_3: _, "~γεσθον", _;
    ind_pass_1: "~γομαι", _, _;
    ind_pass_2: _, "~γεσθον", "~γεσθε";
    ind_pass_3: "~γεται", "~γεσθον", "~γονται";
    subj_act_1: "~γω", _, "~γωμεν";
    subj_act_2: "~γῃς", _, _;
    subj_act_3: "~γῃ", "~γητον", "~γωσῐ";
    subj_mid_1: _, _, "~γωμεθᾰ";
    subj_mid_2: _, "~γησθον", _;
    subj_mid_3: "~γηται", _, "~γωνται";
    subj_pass_1: "~γωμαι", _, _;
    subj_pass_2: _, _, "~γησθε";
    opt_act_1: "~γοιμῐ", _, "~γοιμεν";
    opt_act_2: "~γοις", "~γοιτον", _;
    opt_act_3: "~γοι", "~γοιτην", "~γοιεν";
    opt_mid_1: "~γοιμην", _, "~γοιμεθᾰ";
    opt_mid_2: "~γοιο", _, "~γοισθε";
    opt_mid_3: "~γοιτο", "~γοισθην", "~γοιντο";
    opt_pass_2: _, "~γοισθον", _;
    imp_act_2: "~γε", _, "~γετε";
    imp_act_3: _, _, "~γοντων";
    imp_mid_2: "~γου", "~γεσθον", "~γεσθε";
    imp_mid_3: _, _, "~γεσθων";
    imp_pass_2: _, "~γεσθον", "~γεσθε";
    imp_pass_3: "~γεσθω", "~γεσθων", "~γεσθων";
    imperfect_act_1: "ἐ~γον", _, _;
    imperfect_act_2: "ἐ~γες", "ἐ~γετον", "ἐ~γετε";
    imperfect_act_3: "ἐ~γε", _, "ἐ~γον";
    imperfect_mid_1: "ἐ~γομην", _, "ἐ~γομεθᾰ";
    imperfect_mid_2: "ἐ~γου", _, _;
    imperfect_mid_3: "ἐ~γετο", _, "ἐ~γοντο";
    imperfect_pass_2: _, "ἐ~γεσθον", "ἐ~γεσθε";
    imperfect_pass_3: _, "ἐ~γεσθην", _;
    future_ind_act_3: _, _, "~ξουσῐ";
    future_ind_mid_2: "~ξῃ", _, _;
    aorist_subj_act_3: _, _, derived;
    aorist_opt_act_2: "~ξειᾰς", _, _;
    aorist_opt_act_3: "~ξειε", _, "~ξειᾰν";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_imp_act_2: derived, _, _;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_pres: "~γειν", _, "~γεσθαι";
    participle_pres: "~γων", _, "~γομενος";
    participle_perf: derived, _, _;
    ),
    // Ψηφῐζω: 27 words (γνωρῐζω, ψηφῐζω, ὁρῐζω, ἁγνῐζω, σχηματῐζω, ἀτενῐζω). Like Χρημᾰτῐζω, except in 40 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_ind_mid_1 pl, ….
    conjugation!("ψηφῐζω", 27, "~ζω", like Χρημᾰτῐζω;
    stem pluperfect_act: "~κ- like δῐκεω";
    perfect_ind_act_1: "ἐ~κᾰ", _, "ἐ~κᾰμεν";
    perfect_ind_act_2: _, "ἐ~κᾰτον", _;
    perfect_ind_act_3: "ἐ~κε|ἐ~κεν", _, "ἐ~κᾱσῐ|ἐ~κᾱσῐν";
    perfect_ind_mid_1: _, _, "ἐ~σμεθᾰ";
    perfect_ind_mid_2: "ἐ~σαι", "ἐ~σθον", _;
    perfect_ind_mid_3: "ἐ~σται", _, "ἐ~δᾰται";
    perfect_ind_pass_1: "ἐ~σμαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~σθε";
    perfect_subj_act_1: "ἐ~κω", _, _;
    perfect_subj_act_2: "ἐ~κῃς", "ἐ~κητον", "ἐ~κητε";
    perfect_subj_act_3: "ἐ~κῃ", _, "ἐ~κωσῐ|ἐ~κωσῐν";
    perfect_opt_act_1: "ἐ~κοιμῐ|ἐ~κοιην", _, _;
    perfect_opt_act_2: "ἐ~κοις|ἐ~κοιης", _, _;
    perfect_opt_act_3: "ἐ~κοι|ἐ~κοιη", _, _;
    perfect_imp_act_2: "ἐ~κε", _, _;
    perfect_imp_mid_2: _, _, "ἐ~σθε";
    perfect_imp_mid_3: _, _, "ἐ~σθων";
    perfect_imp_pass_2: "ἐ~σο", _, "ἐ~σθε";
    perfect_imp_pass_3: "ἐ~σθω", "ἐ~σθων", "ἐ~σθων";
    pluperfect_mid_1: "ἐ~σμην", _, _;
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", _;
    pluperfect_mid_3: "ἐ~στο", _, "ἐ~δᾰτο";
    pluperfect_pass_1: _, _, "ἐ~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~σθε";
    pluperfect_pass_3: _, "ἐ~σθην", _;
    infinitive_perf: _, "ἐ~σθαι", _;
    participle_perf: "ἐ~κως", _, "ἐ~σμενος";
    ),
    // Σῠνᾰρπᾰζω: 24 words (βᾰζω, ἁρπᾰζω, ἐλελῐζω, κρᾱζω, στῐζω, σφυζω). Like Ληγω, except in 100 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("σῠνᾰρπᾰζω", 24, "~ζω", like Ληγω;
    ind_act_1: "~ζω", _, "~ζομεν";
    ind_act_2: "~ζεις", "~ζετον", _;
    ind_act_3: "~ζει", _, "~ζουσῐ";
    ind_mid_1: _, _, "~ζομεθᾰ";
    ind_mid_2: "~ζῃ", _, _;
    ind_mid_3: _, "~ζεσθον", _;
    ind_pass_1: "~ζομαι", _, _;
    ind_pass_2: _, "~ζεσθον", "~ζεσθε";
    ind_pass_3: "~ζεται", "~ζεσθον", "~ζονται";
    subj_act_1: "~ζω", _, "~ζωμεν";
    subj_act_2: "~ζῃς", _, _;
    subj_act_3: "~ζῃ", "~ζητον", "~ζωσῐ";
    subj_mid_1: _, _, "~ζωμεθᾰ";
    subj_mid_2: _, "~ζησθον", _;
    subj_mid_3: "~ζηται", _, "~ζωνται";
    subj_pass_1: "~ζωμαι", _, _;
    subj_pass_2: _, _, "~ζησθε";
    opt_act_1: "~ζοιμῐ", _, "~ζοιμεν";
    opt_act_2: "~ζοις", "~ζοιτον", _;
    opt_act_3: "~ζοι", "~ζοιτην", "~ζοιεν";
    opt_mid_1: "~ζοιμην", _, "~ζοιμεθᾰ";
    opt_mid_2: "~ζοιο", _, "~ζοισθε";
    opt_mid_3: "~ζοιτο", "~ζοισθην", "~ζοιντο";
    opt_pass_2: _, "~ζοισθον", _;
    imp_act_2: "~ζε", _, "~ζετε";
    imp_act_3: _, _, "~ζοντων";
    imp_mid_2: "~ζου", "~ζεσθον", "~ζεσθε";
    imp_mid_3: _, _, "~ζεσθων";
    imp_pass_2: _, "~ζεσθον", "~ζεσθε";
    imp_pass_3: "~ζεσθω", "~ζεσθων", "~ζεσθων";
    imperfect_act_1: "ἐ~ζον", _, _;
    imperfect_act_2: "ἐ~ζες", "ἐ~ζετον", "ἐ~ζετε";
    imperfect_act_3: "ἐ~ζε", _, "ἐ~ζον";
    imperfect_mid_1: "ἐ~ζομην", _, "ἐ~ζομεθᾰ";
    imperfect_mid_2: "ἐ~ζου", _, _;
    imperfect_mid_3: "ἐ~ζετο", _, "ἐ~ζοντο";
    imperfect_pass_2: _, "ἐ~ζεσθον", "ἐ~ζεσθε";
    imperfect_pass_3: _, "ἐ~ζεσθην", _;
    perfect_ind_act_1: "ἐ~χᾰ", _, "ἐ~χᾰμεν";
    perfect_ind_act_2: _, "ἐ~χᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~χᾱσῐ";
    perfect_ind_mid_1: _, _, "ἐ~γμεθᾰ";
    perfect_ind_mid_2: "ἐ~ξαι", "ἐ~χθον", _;
    perfect_ind_mid_3: "ἐ~κται", _, "ἐ~γᾰται";
    perfect_ind_pass_1: "ἐ~γμαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~χθε";
    perfect_subj_act_1: "ἐ~χω", _, _;
    perfect_subj_act_2: "ἐ~χῃς", "ἐ~χητον", "ἐ~χητε";
    perfect_subj_act_3: "ἐ~χῃ", _, "ἐ~χωσῐ";
    perfect_imp_act_2: "ἐ~χε", _, _;
    perfect_imp_mid_2: _, _, "ἐ~χθε";
    perfect_imp_mid_3: _, _, "ἐ~χθων";
    perfect_imp_pass_2: "ἐ~ξο", _, "ἐ~χθε";
    perfect_imp_pass_3: "ἐ~χθω", "ἐ~χθων", "ἐ~χθων";
    infinitive_pres: "~ζειν", _, "~ζεσθαι";
    infinitive_perf: _, "ἐ~χθαι", _;
    participle_pres: "~ζων", _, "~ζομενος";
    participle_perf: "ἐ~χως", _, "ἐ~γμενος";
    ),
    // Εὐλογεω: 22 words (εὐλογεω, ἠθεω, κᾰτηχεω, ἠχεω, ὀλιγαρχεω, μοναρχεω). Like Δῐκεω, except in 54 cells: imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, imperfect_act_3 pl, ….
    conjugation!("εὐλογεω", 22, "~εω", like Δῐκεω;
    stem aorist_ind: "~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ηθην- like εὐλογεω";
    stem pluperfect_act: "~ηκ- like εὐλογεω";
    imperfect_act_1: "~ουν", _, _;
    imperfect_act_2: "~εις", "~ειτον", "~ειτε";
    imperfect_act_3: "~ει", _, "~ουν";
    imperfect_mid_1: "~ουμην", _, "~ουμεθᾰ";
    imperfect_mid_2: "~ου", _, _;
    imperfect_mid_3: "~ειτο", _, "~ουντο";
    imperfect_pass_2: _, "~εισθον", "~εισθε";
    imperfect_pass_3: _, "~εισθην", _;
    perfect_ind_act_1: "~ηκᾰ", _, "~ηκᾰμεν";
    perfect_ind_act_2: _, "~ηκᾰτον", _;
    perfect_ind_act_3: "~ηκε|~ηκεν", _, "~ηκᾱσῐ|~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ημεθᾰ";
    perfect_ind_mid_2: "~ησαι", "~ησθον", _;
    perfect_ind_mid_3: "~ηται", _, "~ηνται";
    perfect_ind_pass_1: "~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ησθε";
    perfect_subj_act_1: "~ηκω", _, _;
    perfect_subj_act_2: "~ηκῃς", "~ηκητον", "~ηκητε";
    perfect_subj_act_3: "~ηκῃ", _, "~ηκωσῐ|~ηκωσῐν";
    perfect_opt_act_1: "~ηκοιμῐ|~ηκοιην", _, _;
    perfect_opt_act_2: "~ηκοις|~ηκοιης", _, _;
    perfect_opt_act_3: "~ηκοι|~ηκοιη", _, _;
    perfect_imp_act_2: "~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~ησθε";
    perfect_imp_mid_3: _, _, "~ησθων";
    perfect_imp_pass_2: "~ησο", _, "~ησθε";
    perfect_imp_pass_3: "~ησθω", "~ησθων", "~ησθων";
    pluperfect_mid_1: "~ημην", _, _;
    pluperfect_mid_2: "~ησο", "~ησθον", _;
    pluperfect_mid_3: "~ητο", _, "~ηντο";
    pluperfect_pass_1: _, _, "~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ησθε";
    pluperfect_pass_3: _, "~ησθην", _;
    infinitive_perf: _, "~ησθαι", _;
    participle_perf: "~ηκως", _, "~ημενος";
    ),
    // Ἁπτω: 21 words (κρῠπτω, ἁπτω, κῠπτω, θᾰπτω, ἀποκαλυπτω, σῠναπτω). Like Βουλευω, except in 152 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἁπτω", 21, "~πτω", like Βουλευω;
    stem future_pass: "~φθησ- like ἐχω";
    stem aorist_ind: "~ψ- like ἐχω";
    stem aorist_ind_pass: "~φθην- like ἐχω";
    stem aorist_pass: "~φθ- like ἐχω";
    stem pluperfect_act: "~φειν- like ἐχω";
    ind_act_1: "~πτω", _, "~πτομεν";
    ind_act_2: "~πτεις", "~πτετον", _;
    ind_act_3: "~πτει", _, "~πτουσῐ";
    ind_mid_1: _, _, "~πτομεθᾰ";
    ind_mid_2: "~πτῃ", _, _;
    ind_mid_3: _, "~πτεσθον", _;
    ind_pass_1: "~πτομαι", _, _;
    ind_pass_2: _, "~πτεσθον", "~πτεσθε";
    ind_pass_3: "~πτεται", "~πτεσθον", "~πτονται";
    subj_act_1: "~πτω", _, "~πτωμεν";
    subj_act_2: "~πτῃς", _, _;
    subj_act_3: "~πτῃ", "~πτητον", "~πτωσῐ";
    subj_mid_1: _, _, "~πτωμεθᾰ";
    subj_mid_2: _, "~πτησθον", _;
    subj_mid_3: "~πτηται", _, "~πτωνται";
    subj_pass_1: "~πτωμαι", _, _;
    subj_pass_2: _, _, "~πτησθε";
    opt_act_1: "~πτοιμῐ", _, "~πτοιμεν";
    opt_act_2: "~πτοις", "~πτοιτον", _;
    opt_act_3: "~πτοι", "~πτοιτην", "~πτοιεν";
    opt_mid_1: "~πτοιμην", _, "~πτοιμεθᾰ";
    opt_mid_2: "~πτοιο", _, "~πτοισθε";
    opt_mid_3: "~πτοιτο", "~πτοισθην", "~πτοιντο";
    opt_pass_2: _, "~πτοισθον", _;
    imp_act_2: "~πτε", _, "~πτετε";
    imp_act_3: _, _, "~πτοντων";
    imp_mid_2: "~πτου", "~πτεσθον", "~πτεσθε";
    imp_mid_3: _, _, "~πτεσθων";
    imp_pass_2: _, "~πτεσθον", "~πτεσθε";
    imp_pass_3: "~πτεσθω", "~πτεσθων", "~πτεσθων";
    imperfect_act_1: "ἐ~πτον", _, _;
    imperfect_act_2: "ἐ~πτες", "ἐ~πτετον", "ἐ~πτετε";
    imperfect_act_3: "ἐ~πτε", _, "ἐ~πτον";
    imperfect_mid_1: "ἐ~πτομην", _, "ἐ~πτομεθᾰ";
    imperfect_mid_2: "ἐ~πτου", _, _;
    imperfect_mid_3: "ἐ~πτετο", _, "ἐ~πτοντο";
    imperfect_pass_2: _, "ἐ~πτεσθον", "ἐ~πτεσθε";
    imperfect_pass_3: _, "ἐ~πτεσθην", _;
    future_ind_act_1: "~ψω", _, "~ψομεν";
    future_ind_act_2: "~ψεις", "~ψετον", _;
    future_ind_act_3: _, _, "~ψουσῐ";
    future_ind_mid_1: "~ψομαι", _, _;
    future_ind_mid_2: "~ψῃ", "~ψεσθον", _;
    future_ind_mid_3: "~ψεται", _, _;
    future_opt_act_1: "~ψοιμῐ", _, "~ψοιμεν";
    future_opt_act_2: "~ψοις", "~ψοιτον", _;
    future_opt_act_3: "~ψοι", "~ψοιτην", _;
    future_opt_mid_1: "~ψοιμην", _, "~ψοιμεθᾰ";
    future_opt_mid_3: _, "~ψοισθην", _;
    aorist_subj_act_1: "~ψω", _, _;
    aorist_subj_act_2: "~ψῃς", "~ψητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~ψωμαι", _, _;
    aorist_subj_mid_2: "~ψῃ", "~ψησθον", _;
    aorist_opt_act_1: "~ψαιμῐ", _, "~ψαιμεν";
    aorist_opt_act_2: "~ψειᾰς", _, _;
    aorist_opt_act_3: "~ψειε", _, "~ψειᾰν";
    aorist_opt_mid_1: "~ψαιμην", _, _;
    aorist_imp_act_2: "~ψον", _, "~ψᾰτε";
    aorist_imp_act_3: _, _, "~ψᾰντων";
    aorist_imp_mid_2: "~ψαι", "~ψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ψᾰσθων";
    perfect_ind_act_1: "%ε~φᾰ", _, "%ε~φᾰμεν";
    perfect_ind_act_2: _, "%ε~φᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~φᾱσῐ";
    perfect_subj_act_1: "%ε~φω", _, _;
    perfect_subj_act_2: "%ε~φῃς", "%ε~φητον", "%ε~φητε";
    perfect_subj_act_3: "%ε~φῃ", _, "%ε~φωσῐ";
    perfect_imp_act_2: "%ε~φε", _, _;
    pluperfect_mid_1: "%ε~μην", _, _;
    pluperfect_mid_2: "%ε~σο", "%ε~σθον", _;
    pluperfect_mid_3: "%ε~το", _, "%ε~ντο";
    pluperfect_pass_1: _, _, "%ε~μεθᾰ";
    pluperfect_pass_2: _, _, "%ε~σθε";
    pluperfect_pass_3: _, "%ε~σθην", _;
    future_perfect_ind_mid_1: "%ε~φομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~φεσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~φομεθᾰ";
    future_perfect_ind_pass_2: "%ε~φῃ", "%ε~φεσθον", "%ε~φεσθε";
    future_perfect_ind_pass_3: "%ε~φεται", "%ε~φεσθον", "%ε~φονται";
    future_perfect_opt_pass_1: "%ε~φοιμην", _, "%ε~φοιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~φοιο", "%ε~φοισθον", "%ε~φοισθε";
    future_perfect_opt_pass_3: "%ε~φοιτο", "%ε~φοισθην", "%ε~φοιντο";
    infinitive_pres: "~πτειν", _, "~πτεσθαι";
    infinitive_fut: "~ψειν", _, _;
    infinitive_aor: "~ψαι", _, _;
    infinitive_futp: _, _, "%ε~φεσθαι";
    participle_pres: "~πτων", _, "~πτομενος";
    participle_aor: "~ψᾱς", "~ψᾰμενος", _;
    participle_perf: "%ε~φως", _, _;
    participle_futp: _, _, "%ε~φομενος";
    ),
    // Ἐλπω: 20 words (ὑω, ἐλπω, ἐρδω, περικλειω, ἑλκῠω, αὐω). Like Βουλευω, except in 58 cells: ind_mid_2 sg, imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("ἐλπω", 20, "~ω", like Βουλευω;
    stem future_pass: "~σθησ- like ἐχω";
    stem aorist_ind: "~σ- like ἐλπω";
    stem aorist_ind_pass: "~σθην- like εὐλογεω";
    stem aorist_pass: "~σθ- like ἐχω";
    ind_mid_2: "~ῃ", _, _;
    imperfect_act_1: "~ον", _, _;
    imperfect_act_2: "~ες", "~ετον", "~ετε";
    imperfect_act_3: "~ε|~εν", _, "~ον";
    imperfect_mid_1: "~ομην", _, "~ομεθᾰ";
    imperfect_mid_2: "~ου", _, _;
    imperfect_mid_3: "~ετο", _, "~οντο";
    imperfect_pass_2: _, "~εσθον", "~εσθε";
    imperfect_pass_3: _, "~εσθην", _;
    future_ind_act_3: _, _, "~σουσῐ";
    future_ind_mid_2: "~σῃ", _, _;
    aorist_subj_act_3: _, _, derived;
    aorist_opt_act_2: "~σειᾰς", _, _;
    aorist_opt_act_3: "~σειε", _, "~σειᾰν";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_1: _, _, "~σμεθᾰ";
    perfect_ind_mid_2: "~σαι", "~σθον", _;
    perfect_ind_mid_3: "~σται", _, "~σᾰται";
    perfect_ind_pass_1: "~σμαι", _, _;
    perfect_ind_pass_2: _, _, "~σθε";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_imp_act_2: derived, _, _;
    perfect_imp_mid_2: _, _, "~σθε";
    perfect_imp_mid_3: _, _, "~σθων";
    perfect_imp_pass_2: "~σο", _, "~σθε";
    perfect_imp_pass_3: "~σθω", "~σθων", "~σθων";
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: _, "~σθαι", _;
    participle_perf: derived, _, "~σμενος";
    ),
    // Κτῐζω: 20 words (σχῐζω, ὀνομᾰζω, σχεδῐᾰζω, ἁρμοζω, κτῐζω, σπιζω). Like Ψηφῐζω, except in 21 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_3 pl, future_ind_mid_1 sg, ….
    conjugation!("κτῐζω", 20, "~ζω", like Ψηφῐζω;
    future_ind_act_1: "~σω", _, "~σομεν";
    future_ind_act_2: "~σεις", "~σετον", _;
    future_ind_act_3: _, _, "~σουσῐ|~σουσῐν";
    future_ind_mid_1: "~σομαι", _, _;
    future_ind_mid_2: "~σῃ|~σει", "~σεσθον", _;
    future_ind_mid_3: "~σεται", _, _;
    future_opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    future_opt_act_2: "~σοις", "~σοιτον", derived;
    future_opt_act_3: "~σοι", "~σοιτην", derived;
    future_opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    future_opt_mid_3: _, "~σοισθην", _;
    infinitive_fut: "~σειν", _, _;
    ),
    // Νῑκαω: 18 words (νῑκαω, λικμαω, ὀργαω, μελεταω, κομαω, εἰσοραω). Like Δοκᾰω, except in 29 cells: future_ind_act_3 pl, pluperfect_act_1 sg, pluperfect_mid_1 sg, pluperfect_mid_2 sg, pluperfect_mid_2 du, pluperfect_mid_3 sg, ….
    conjugation!("νῑκαω", 18, "~αω", like Δοκᾰω;
    future_ind_act_3: _, _, "~ησουσῐ";
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
    // Πᾰραιτεομαι: 18 words (ὀρχεομαι, ἀρνεομαι, ἱκνεομαι, ἐπεξηγεομαι, ἀφῐκνεομαι, ὑπῐσχνεομαι). Like Ποιεω, except in 84 cells: ind_act_3 pl, ind_pass_2 sg, subj_act_3 pl, opt_act_1 sg, opt_act_1 pl, opt_act_2 sg, ….
    conjugation!("πᾰραιτεομαι", 18, "~εομαι", like Ποιεω;
    ind_act_3: _, _, "~ουσῐ";
    ind_pass_2: "~ει", _, _;
    subj_act_3: _, _, "~ωσῐ";
    opt_act_1: "~οιην", _, "~οιμεν";
    opt_act_2: "~οιης", "~οιτον", derived;
    opt_act_3: "~οιη", "~οιτην", "~οιεν";
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, _;
    future_ind_act_3: _, _, derived;
    future_ind_mid_2: "~ησῃ", _, _;
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
    perfect_ind_act_3: derived, _, "%ε~ηκᾱσῐ";
    perfect_subj_act_3: _, _, "%ε~ηκωσῐ";
    perfect_opt_act_1: derived, _, _;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: derived, _, _;
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
    // Πορσῡνω: 18 words (πληθῡνω, ἀμῡνω, εὐρῡνω, δῐευθῡνω, εὐθῡνω, αἰσχῡνω). Like Ληγω, except in 150 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πορσῡνω", 18, "~ῡνω", like Ληγω;
    stem future_pass: "~ῠνθησ- like ἐχω";
    stem aorist_ind: "~ῡν- like ποιεω";
    stem aorist_ind_pass: "~ῡνθην- like ἐχω";
    stem aorist_pass: "~ῡνθ- like ἐχω";
    stem pluperfect_act: "~ῠγκειν- like ἐχω";
    ind_act_1: "~ῡνω", _, "~ῡνομεν";
    ind_act_2: "~ῡνεις", "~ῡνετον", _;
    ind_act_3: "~ῡνει", _, "~ῡνουσῐ|~ῡνουσῐν";
    ind_mid_1: _, _, "~ῡνομεθᾰ";
    ind_mid_2: "~ῡνῃ|~ῡνει", _, _;
    ind_mid_3: _, "~ῡνεσθον", _;
    ind_pass_1: "~ῡνομαι", _, _;
    ind_pass_2: _, "~ῡνεσθον", "~ῡνεσθε";
    ind_pass_3: "~ῡνεται", "~ῡνεσθον", "~ῡνονται";
    subj_act_1: "~ῡνω", _, "~ῡνωμεν";
    subj_act_2: "~ῡνῃς", _, _;
    subj_act_3: "~ῡνῃ", "~ῡνητον", "~ῡνωσῐ|~ῡνωσῐν";
    subj_mid_1: _, _, "~ῡνωμεθᾰ";
    subj_mid_2: _, "~ῡνησθον", _;
    subj_mid_3: "~ῡνηται", _, "~ῡνωνται";
    subj_pass_1: "~ῡνωμαι", _, _;
    subj_pass_2: _, _, "~ῡνησθε";
    opt_act_1: "~ῡνοιμῐ", _, "~ῡνοιμεν";
    opt_act_2: "~ῡνοις", "~ῡνοιτον", _;
    opt_act_3: "~ῡνοι", "~ῡνοιτην", "~ῡνοιεν";
    opt_mid_1: "~ῡνοιμην", _, "~ῡνοιμεθᾰ";
    opt_mid_2: "~ῡνοιο", _, "~ῡνοισθε";
    opt_mid_3: "~ῡνοιτο", "~ῡνοισθην", "~ῡνοιντο";
    opt_pass_2: _, "~ῡνοισθον", _;
    imp_act_2: "~ῡνε", _, "~ῡνετε";
    imp_act_3: _, _, "~ῡνοντων";
    imp_mid_2: "~ῡνου", "~ῡνεσθον", "~ῡνεσθε";
    imp_mid_3: _, _, "~ῡνεσθων";
    imp_pass_2: _, "~ῡνεσθον", "~ῡνεσθε";
    imp_pass_3: "~ῡνεσθω", "~ῡνεσθων", "~ῡνεσθων";
    imperfect_act_1: "ἐ~ῡνον", _, _;
    imperfect_act_2: "ἐ~ῡνες", "ἐ~ῡνετον", "ἐ~ῡνετε";
    imperfect_act_3: "ἐ~ῡνε|ἐ~ῡνεν", _, "ἐ~ῡνον";
    imperfect_mid_1: "ἐ~ῡνομην", _, "ἐ~ῡνομεθᾰ";
    imperfect_mid_2: "ἐ~ῡνου", _, _;
    imperfect_mid_3: "ἐ~ῡνετο", _, "ἐ~ῡνοντο";
    imperfect_pass_2: _, "ἐ~ῡνεσθον", "ἐ~ῡνεσθε";
    imperfect_pass_3: _, "ἐ~ῡνεσθην", _;
    future_ind_act_1: "~ῠνω", _, "~ῠνουμεν";
    future_ind_act_2: "~ῠνεις", "~ῠνειτον", _;
    future_ind_act_3: _, _, "~ῠνουσῐ|~ῠνουσῐν";
    future_ind_mid_1: "~ῠνουμαι", _, _;
    future_ind_mid_2: "~ῠνει|~ῠνῃ", "~ῠνεισθον", _;
    future_ind_mid_3: "~ῠνειται", _, _;
    future_opt_act_1: "~ῠνοιην|~ῠνοιμῐ", _, "~ῠνοιμεν|~ῠνοιημεν";
    future_opt_act_2: "~ῠνοιης|~ῠνοις", "~ῠνοιτον|~ῠνοιητον", "~ῠνοιτε|~ῠνοιητε";
    future_opt_act_3: "~ῠνοιη|~ῠνοι", "~ῠνοιτην|~ῠνοιητην", "~ῠνοιεν|~ῠνοιησᾰν";
    future_opt_mid_1: "~ῠνοιμην", _, "~ῠνοιμεθᾰ";
    future_opt_mid_3: _, "~ῠνοισθην", _;
    aorist_subj_act_1: "~ῡνω", _, _;
    aorist_subj_act_2: "~ῡνῃς", "~ῡνητον", _;
    aorist_subj_mid_1: "~ῡνωμαι", _, _;
    aorist_subj_mid_2: "~ῡνῃ", "~ῡνησθον", _;
    aorist_opt_act_1: "~ῡναιμῐ", _, "~ῡναιμεν";
    aorist_opt_act_2: "~ῡνειᾰς", _, _;
    aorist_opt_act_3: "~ῡνειε", _, "~ῡνειᾰν";
    aorist_opt_mid_1: "~ῡναιμην", _, _;
    aorist_imp_act_2: "~ῡνον", _, "~ῡνᾰτε";
    aorist_imp_act_3: _, _, "~ῡνᾰντων";
    aorist_imp_mid_2: "~ῡναι", "~ῡνᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῡνᾰσθων";
    perfect_ind_act_1: "%ε~ῠγκᾰ", _, "%ε~ῠγκᾰμεν";
    perfect_ind_act_2: _, "%ε~ῠγκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ῠγκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~ῠσμεθᾰ";
    perfect_ind_mid_2: "%ε~ῠνσαι", "%ε~ῠνθον", _;
    perfect_ind_mid_3: "%ε~ῠνται", _, "%ε~ῠνᾰται";
    perfect_ind_pass_1: "%ε~ῠσμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ῠνθε";
    perfect_subj_act_1: "%ε~ῠγκω", _, _;
    perfect_subj_act_2: "%ε~ῠγκῃς", "%ε~ῠγκητον", "%ε~ῠγκητε";
    perfect_subj_act_3: "%ε~ῠγκῃ", _, "%ε~ῠγκωσῐ";
    perfect_imp_act_2: "%ε~ῠγκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~ῠνθε";
    perfect_imp_mid_3: _, _, "%ε~ῠνθων";
    perfect_imp_pass_2: "%ε~ῠνσο", _, "%ε~ῠνθε";
    perfect_imp_pass_3: "%ε~ῠνθω", "%ε~ῠνθων", "%ε~ῠνθων";
    pluperfect_mid_1: "ἐ%ε~ῠσμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ῠνσο", "ἐ%ε~ῠνθον", _;
    pluperfect_mid_3: "ἐ%ε~ῠντο", _, "ἐ%ε~ῠνᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~ῠσμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ῠνθε";
    pluperfect_pass_3: _, "ἐ%ε~ῠνθην", _;
    infinitive_pres: "~ῡνειν", _, "~ῡνεσθαι";
    infinitive_fut: "~ῠνειν", _, _;
    infinitive_aor: "~ῡναι", _, _;
    infinitive_perf: _, "%ε~ῠνθαι", _;
    participle_pres: "~ῡνων", _, "~ῡνομενος";
    participle_aor: "~ῡνᾱς", "~ῡνᾰμενος", _;
    participle_perf: "%ε~ῠγκως", _, "%ε~ῠσμενος";
    ),
    // Πεμπω: 17 words (λᾰμπω, βλεπω, ἐρειπω, λειπω, ἑρπω, πεμπω).
    conjugation!("πεμπω", 17, "~~πω";
    stem future_pass: "~~φθησ- like ἐχω";
    stem aorist_ind: "~~ψ- like ἐχω";
    stem aorist_ind_pass: "~~φθην- like ἐχω";
    stem aorist_pass: "~~φθ- like ἐχω";
    ind_act_1: "~~πω", _, "~~πομεν";
    ind_act_2: "~~πεις", "~~πετον", _;
    ind_act_3: "~~πει", _, "~~πουσῐ";
    ind_mid_1: _, _, "~~πομεθᾰ";
    ind_mid_2: "~~πῃ", _, _;
    ind_mid_3: _, "~~πεσθον", _;
    ind_pass_1: "~~πομαι", _, _;
    ind_pass_2: _, "~~πεσθον", "~~πεσθε";
    ind_pass_3: "~~πεται", "~~πεσθον", "~~πονται";
    subj_act_1: "~~πω", _, "~~πωμεν";
    subj_act_2: "~~πῃς", _, _;
    subj_act_3: "~~πῃ", "~~πητον", "~~πωσῐ";
    subj_mid_1: _, _, "~~πωμεθᾰ";
    subj_mid_2: _, "~~πησθον", _;
    subj_mid_3: "~~πηται", _, "~~πωνται";
    subj_pass_1: "~~πωμαι", _, _;
    subj_pass_2: _, _, "~~πησθε";
    opt_act_1: "~~ποιμῐ", _, "~~ποιμεν";
    opt_act_2: "~~ποις", "~~ποιτον", _;
    opt_act_3: "~~ποι", "~~ποιτην", "~~ποιεν";
    opt_mid_1: "~~ποιμην", _, "~~ποιμεθᾰ";
    opt_mid_2: "~~ποιο", _, "~~ποισθε";
    opt_mid_3: "~~ποιτο", "~~ποισθην", "~~ποιντο";
    opt_pass_2: _, "~~ποισθον", _;
    imp_act_2: "~~πε", _, "~~πετε";
    imp_act_3: _, _, "~~ποντων";
    imp_mid_2: "~~που", "~~πεσθον", "~~πεσθε";
    imp_mid_3: _, _, "~~πεσθων";
    imp_pass_2: _, "~~πεσθον", "~~πεσθε";
    imp_pass_3: "~~πεσθω", "~~πεσθων", "~~πεσθων";
    imperfect_act_1: "ἐ~~πον", _, _;
    imperfect_act_2: "ἐ~~πες", "ἐ~~πετον", "ἐ~~πετε";
    imperfect_act_3: "ἐ~~πε", _, "ἐ~~πον";
    imperfect_mid_1: "ἐ~~πομην", _, "ἐ~~πομεθᾰ";
    imperfect_mid_2: "ἐ~~που", _, _;
    imperfect_mid_3: "ἐ~~πετο", _, "ἐ~~ποντο";
    imperfect_pass_2: _, "ἐ~~πεσθον", "ἐ~~πεσθε";
    imperfect_pass_3: _, "ἐ~~πεσθην", _;
    future_ind_act_1: "~~ψω", _, "~~ψομεν";
    future_ind_act_2: "~~ψεις", "~~ψετον", _;
    future_ind_act_3: _, _, "~~ψουσῐ";
    future_ind_mid_1: "~~ψομαι", _, _;
    future_ind_mid_2: "~~ψῃ", "~~ψεσθον", _;
    future_ind_mid_3: "~~ψεται", _, _;
    future_opt_act_1: "~~ψοιμῐ", _, "~~ψοιμεν";
    future_opt_act_2: "~~ψοις", "~~ψοιτον", _;
    future_opt_act_3: "~~ψοι", "~~ψοιτην", _;
    future_opt_mid_1: "~~ψοιμην", _, "~~ψοιμεθᾰ";
    future_opt_mid_3: _, "~~ψοισθην", _;
    aorist_subj_act_1: "~~ψω", _, _;
    aorist_subj_act_2: "~~ψῃς", "~~ψητον", _;
    aorist_subj_mid_1: "~~ψωμαι", _, _;
    aorist_subj_mid_2: "~~ψῃ", "~~ψησθον", _;
    aorist_opt_act_1: "~~ψαιμῐ", _, "~~ψαιμεν";
    aorist_opt_act_2: "~~ψειᾰς", _, _;
    aorist_opt_act_3: "~~ψειε", _, "~~ψειᾰν";
    aorist_opt_mid_1: "~~ψαιμην", _, _;
    aorist_imp_act_2: "~~ψον", _, "~~ψᾰτε";
    aorist_imp_act_3: _, _, "~~ψᾰντων";
    aorist_imp_mid_2: "~~ψαι", "~~ψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~ψᾰσθων";
    perfect_ind_act_1: "~πο~φᾰ", _, "~πο~φᾰμεν";
    perfect_ind_act_2: _, "~πο~φᾰτον", _;
    perfect_ind_act_3: _, _, "~πο~φᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~~μεθᾰ";
    perfect_ind_mid_2: "%ε~~ψαι", "%ε~~φθον", _;
    perfect_ind_mid_3: "%ε~~πται", _, "%ε~~πᾰται";
    perfect_ind_pass_1: "%ε~~μαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~~φθε";
    perfect_subj_act_1: "~πο~φω", _, _;
    perfect_subj_act_2: "~πο~φῃς", "~πο~φητον", "~πο~φητε";
    perfect_subj_act_3: "~πο~φῃ", _, "~πο~φωσῐ";
    perfect_imp_act_2: "~πο~φε", _, _;
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
    infinitive_pres: "~~πειν", _, "~~πεσθαι";
    infinitive_fut: "~~ψειν", _, _;
    infinitive_aor: "~~ψαι", _, _;
    infinitive_perf: _, "%ε~~φθαι", _;
    participle_pres: "~~πων", _, "~~πομενος";
    participle_aor: "~~ψᾱς", "~~ψᾰμενος", _;
    participle_perf: "~πο~φως", _, "%ε~~μενος";
    ),
    // Θωᾰω: 16 words (πεπτω, τᾰττω, ἀλεγω, ϝερρω, φᾱρω, θωαδδω). Like Ἑπομαι, except in 86 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("θωᾰω", 16, "~ω", like Ἑπομαι;
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
    opt_act_2: "~οις", "~οιτον", _;
    opt_act_3: "~οι", "~οιτην", "~οιεν";
    imp_act_2: "~ε", _, "~ετε";
    imp_act_3: _, _, "~οντων";
    imp_mid_2: "~εο", _, _;
    imperfect_act_1: "ἐ~ον", _, _;
    imperfect_act_2: "ἐ~ες", "ἐ~ετον", "ἐ~ετε";
    imperfect_act_3: "ἐ~ε", _, "ἐ~ον";
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_3: _, derived, _;
    aorist_ind_mid_1: derived, _, _;
    aorist_ind_mid_2: derived, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_mid_1: derived, _, _;
    aorist_subj_mid_2: derived, derived, _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_mid_1: derived, _, _;
    aorist_imp_mid_2: derived, derived, _;
    aorist_imp_mid_3: _, _, derived;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
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
    // Πειρᾰζω: 15 words (πᾰρουσῐᾰζω, πλεονᾰζω, θειᾰζω, γυμνᾰζω, πειρᾰζω, πλησῐᾰζω). Like Χρημᾰτῐζω, except in 29 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_3 pl, future_ind_mid_1 sg, ….
    conjugation!("πειρᾰζω", 15, "~ζω", like Χρημᾰτῐζω;
    stem pluperfect_act: "~κ- like δῐκεω";
    future_ind_act_1: "~σω", _, "~σομεν";
    future_ind_act_2: "~σεις", "~σετον", _;
    future_ind_act_3: _, _, "~σουσῐ|~σουσῐν";
    future_ind_mid_1: "~σομαι", _, _;
    future_ind_mid_2: "~σῃ|~σει", "~σεσθον", _;
    future_ind_mid_3: "~σεται", _, _;
    future_opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    future_opt_act_2: "~σοις", "~σοιτον", derived;
    future_opt_act_3: "~σοι", "~σοιτην", derived;
    future_opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    future_opt_mid_3: _, "~σοισθην", _;
    pluperfect_mid_1: "ἐ~σμην", _, _;
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", _;
    pluperfect_mid_3: "ἐ~στο", _, "ἐ~δᾰτο";
    pluperfect_pass_1: _, _, "ἐ~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~σθε";
    pluperfect_pass_3: _, "ἐ~σθην", _;
    infinitive_fut: "~σειν", _, _;
    ),
    // Ἀνῐστημῐ: 15 words (ἐξῐστημῐ, ἐπεντῐθημῐ, τῐθημῐ, πᾰρῐστημῐ, ἀνῐστημῐ, κᾰθῐστημῐ). Like Πεμπω, except in 186 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀνῐστημῐ", 15, "~ῐ~ημῐ", like Πεμπω;
    stem future_pass: "~ᾰ~ησ- like ἐχω";
    stem aorist_ind: "~ε~ησ- like ἐλπω";
    stem aorist_ind_pass: "~ε~ᾰθην- like εὐλογεω";
    stem aorist_pass: "~ᾰ~ᾰθ- like ἐχω";
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
    opt_mid_2: "~ῐ~αιο", _, "~ῐ~αισθε";
    opt_mid_3: "~ῐ~αιτο", "~ῐ~αισθην", "~ῐ~αιντο";
    opt_pass_2: _, "~ῐ~αισθον", _;
    imp_act_2: "~ῐ~η", _, "~ῐ~ᾰτε";
    imp_act_3: _, _, "~ῐ~ᾰντων";
    imp_mid_2: "~ῐ~ᾰσο|~ῐ~ω", "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    imp_mid_3: _, _, "~ῐ~ᾰσθων";
    imp_pass_2: _, "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    imp_pass_3: "~ῐ~ᾰσθω", "~ῐ~ᾰσθων", "~ῐ~ᾰσθων";
    imperfect_act_1: "~ῑ~ην", _, _;
    imperfect_act_2: "~ῑ~ης", "~ῑ~ᾰτον", "~ῑ~ᾰτε";
    imperfect_act_3: "~ῑ~η", _, "~ῑ~ᾰσᾰν";
    imperfect_mid_1: "~ῑ~ᾰμην", _, "~ῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~ῑ~ᾰσο", _, _;
    imperfect_mid_3: "~ῑ~ᾰτο", _, "~ῑ~ᾰντο";
    imperfect_pass_2: _, "~ῑ~ᾰσθον", "~ῑ~ᾰσθε";
    imperfect_pass_3: _, "~ῑ~ᾰσθην", _;
    future_ind_act_1: "~ᾰ~ησω", _, "~ᾰ~ησομεν";
    future_ind_act_2: "~ᾰ~ησεις", "~ᾰ~ησετον", _;
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ";
    future_ind_mid_1: "~ᾰ~ησομαι", _, _;
    future_ind_mid_2: "~ᾰ~ησῃ", "~ᾰ~ησεσθον", _;
    future_ind_mid_3: "~ᾰ~ησεται", _, _;
    future_opt_act_1: "~ᾰ~ησοιμῐ", _, "~ᾰ~ησοιμεν";
    future_opt_act_2: "~ᾰ~ησοις", "~ᾰ~ησοιτον", _;
    future_opt_act_3: "~ᾰ~ησοι", "~ᾰ~ησοιτην", _;
    future_opt_mid_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~ησοισθην", _;
    aorist_subj_act_1: "~ᾰ~ησω", _, _;
    aorist_subj_act_2: "~ᾰ~ησῃς", "~ᾰ~ησητον", _;
    aorist_subj_mid_1: "~ᾰ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ησῃ", "~ᾰ~ησησθον", _;
    aorist_opt_act_1: "~ᾰ~ησαιμῐ", _, "~ᾰ~ησαιμεν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς", _, _;
    aorist_opt_act_3: "~ᾰ~ησειε", _, "~ᾰ~ησειᾰν";
    aorist_opt_mid_1: "~ᾰ~ησαιμην", _, _;
    aorist_imp_act_2: "~ᾰ~ησον", _, "~ᾰ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ησᾰντων";
    aorist_imp_mid_2: "~ᾰ~ησαι", "~ᾰ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ησᾰσθων";
    perfect_ind_act_1: "~ε~ᾰκᾰ", _, "~ε~ᾰκᾰμεν";
    perfect_ind_act_2: _, "~ε~ᾰκᾰτον", _;
    perfect_ind_act_3: _, _, "~ε~ᾰκᾱσῐ";
    perfect_ind_mid_1: _, _, "~ε~ᾰμεθᾰ";
    perfect_ind_mid_2: "~ε~ᾰσαι", "~ε~ᾰσθον", _;
    perfect_ind_mid_3: "~ε~ᾰται", _, "~ε~ᾰνται";
    perfect_ind_pass_1: "~ε~ᾰμαι", _, _;
    perfect_ind_pass_2: _, _, "~ε~ᾰσθε";
    perfect_subj_act_1: "~ε~ᾰκω", _, _;
    perfect_subj_act_2: "~ε~ᾰκῃς", "~ε~ᾰκητον", "~ε~ᾰκητε";
    perfect_subj_act_3: "~ε~ᾰκῃ", _, "~ε~ᾰκωσῐ";
    perfect_imp_act_2: "~ε~ᾰκε", _, _;
    perfect_imp_mid_2: _, _, "~ε~ᾰσθε";
    perfect_imp_mid_3: _, _, "~ε~ᾰσθων";
    perfect_imp_pass_2: "~ε~ᾰσο", _, "~ε~ᾰσθε";
    perfect_imp_pass_3: "~ε~ᾰσθω", "~ε~ᾰσθων", "~ε~ᾰσθων";
    pluperfect_act_1: "~ε~ᾰκειν", _, _;
    pluperfect_mid_1: "~ε~ᾰμην", _, _;
    pluperfect_mid_2: "~ε~ᾰσο", "~ε~ᾰσθον", _;
    pluperfect_mid_3: "~ε~ᾰτο", _, "~ε~ᾰντο";
    pluperfect_pass_1: _, _, "~ε~ᾰμεθᾰ";
    pluperfect_pass_2: _, _, "~ε~ᾰσθε";
    pluperfect_pass_3: _, "~ε~ᾰσθην", _;
    future_perfect_ind_act_1: "~ε~ηξω", _, "~ε~ηξομεν";
    future_perfect_ind_act_2: "~ε~ηξεις", "~ε~ηξετον", "~ε~ηξετε";
    future_perfect_ind_act_3: "~ε~ηξει", _, "~ε~ηξουσῐ";
    future_perfect_ind_mid_1: "~ε~ηξομαι", _, _;
    future_perfect_ind_mid_3: _, "~ε~ηξεσθον", _;
    future_perfect_ind_pass_1: _, _, "~ε~ηξομεθᾰ";
    future_perfect_ind_pass_2: "~ε~ηξῃ", "~ε~ηξεσθον", "~ε~ηξεσθε";
    future_perfect_ind_pass_3: "~ε~ηξεται", "~ε~ηξεσθον", "~ε~ηξονται";
    future_perfect_opt_act_1: "~ε~ηξοιμῐ", _, "~ε~ηξοιμεν";
    future_perfect_opt_act_2: "~ε~ηξοις", "~ε~ηξοιτον", "~ε~ηξοιτε";
    future_perfect_opt_act_3: "~ε~ηξοι", "~ε~ηξοιτην", "~ε~ηξοιεν";
    future_perfect_opt_pass_1: "~ε~ηξοιμην", _, "~ε~ηξοιμεθᾰ";
    future_perfect_opt_pass_2: "~ε~ηξοιο", "~ε~ηξοισθον", "~ε~ηξοισθε";
    future_perfect_opt_pass_3: "~ε~ηξοιτο", "~ε~ηξοισθην", "~ε~ηξοιντο";
    infinitive_pres: "~ῐ~ᾰναι", _, "~ῐ~ᾰσθαι";
    infinitive_fut: "~ᾰ~ησειν", _, _;
    infinitive_aor: "~ᾰ~ησαι", _, _;
    infinitive_perf: _, "~ε~ᾰσθαι", _;
    infinitive_futp: "~ε~ηξειν", _, "~ε~ηξεσθαι";
    participle_pres: "~ῐ~ᾱς", _, "~ῐ~ᾰμενος";
    participle_aor: "~ᾰ~ησᾱς", "~ᾰ~ησᾰμενος", _;
    participle_perf: "~ε~ᾰκως", _, "~ε~ᾰμενος";
    participle_futp: "~ε~ηξων", _, "~ε~ηξομενος";
    ),
    // Λογῐζομαι: 14 words (λογῐζομαι, χᾰρῐζομαι, λᾰζομαι, δῐᾰγωνῐζομαι, λῐᾰζομαι, πλειστηρῐζομαι). Like Χρημᾰτῐζω, except in 33 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_3 pl, future_ind_mid_2 sg, ….
    conjugation!("λογῐζομαι", 14, "~ζομαι", like Χρημᾰτῐζω;
    stem aorist_ind: "~σ- like ἐχω";
    stem pluperfect_act: "~κειν- like ἐχω";
    ind_act_3: _, _, "~ζουσῐ";
    ind_mid_2: "~ζῃ", _, _;
    subj_act_3: _, _, "~ζωσῐ";
    imperfect_act_3: "ἐ~ζε", _, _;
    future_ind_act_3: _, _, "~ουσῐ";
    future_ind_mid_2: "~ει", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~οιην", _, "~οιμεν";
    future_opt_act_2: "~οιης", "~οιτον", derived;
    future_opt_act_3: "~οιη", "~οιτην", derived;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_2: "~σειᾰς", _, _;
    aorist_opt_act_3: "~σειε", _, "~σειᾰν";
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: derived, _, "%ε~κᾱσῐ";
    perfect_subj_act_3: _, _, "%ε~κωσῐ";
    perfect_opt_act_1: derived, _, _;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: derived, _, _;
    pluperfect_act_2: derived, _, _;
    pluperfect_act_3: derived, _, _;
    ),
    // Σκᾰνδᾰλῐζω: 13 words (σκᾰνδᾰλῐζω, δειγμᾰτῐζω, γογγῠζω, βλυζω, τρῑζω, πλειστηριᾰζω). Like Θερᾰπευω with `ˇ` after each pattern, except in 30 cells: future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, aorist_opt_pass_3 du, ….
    conjugation!("σκᾰνδᾰλῐζω", 13, "~ωˇ", like Θερᾰπευω with "ˇ";
    future_ind_pass_2: derived, _, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: "%ε~κεˇ", _, "%ε~κᾱσῐˇ";
    perfect_subj_act_3: _, _, "%ε~κωσῐˇ";
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
    // Μνημονευω: 13 words (στεφω, ἀγχω, ἀροω, ἀρτῡω, μνημονευω, πταιω). Like Βουλευω, except in 41 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_ind_mid_1 pl, ….
    conjugation!("μνημονευω", 13, "~ω", like Βουλευω;
    stem future_pass: "~θησ- like χρημᾰτῐζω";
    stem aorist_pass: "~θ- like ποιεω";
    perfect_ind_act_1: "ἐ~κᾰ", _, "ἐ~κᾰμεν";
    perfect_ind_act_2: _, "ἐ~κᾰτον", _;
    perfect_ind_act_3: "ἐ~κε|ἐ~κεν", _, "ἐ~κᾱσῐ|ἐ~κᾱσῐν";
    perfect_ind_mid_1: _, _, "ἐ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~σαι", "ἐ~σθον", _;
    perfect_ind_mid_3: "ἐ~ται", _, "ἐ~νται";
    perfect_ind_pass_1: "ἐ~μαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~σθε";
    perfect_subj_act_1: "ἐ~κω", _, _;
    perfect_subj_act_2: "ἐ~κῃς", "ἐ~κητον", "ἐ~κητε";
    perfect_subj_act_3: "ἐ~κῃ", _, "ἐ~κωσῐ|ἐ~κωσῐν";
    perfect_opt_act_1: "ἐ~κοιμῐ|ἐ~κοιην", _, _;
    perfect_opt_act_2: "ἐ~κοις|ἐ~κοιης", _, _;
    perfect_opt_act_3: "ἐ~κοι|ἐ~κοιη", _, _;
    perfect_imp_act_2: "ἐ~κε", _, _;
    perfect_imp_mid_2: _, _, "ἐ~σθε";
    perfect_imp_mid_3: _, _, "ἐ~σθων";
    perfect_imp_pass_2: "ἐ~σο", _, "ἐ~σθε";
    perfect_imp_pass_3: "ἐ~σθω", "ἐ~σθων", "ἐ~σθων";
    pluperfect_act_1: "ἐ~κειν", _, _;
    pluperfect_mid_1: "ἐ~μην", _, _;
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", _;
    pluperfect_mid_3: "ἐ~το", _, "ἐ~ντο";
    pluperfect_pass_1: _, _, "ἐ~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~σθε";
    pluperfect_pass_3: _, "ἐ~σθην", _;
    infinitive_perf: _, "ἐ~σθαι", _;
    participle_perf: "ἐ~κως", _, "ἐ~μενος";
    ),
    // Ἀειρω: 13 words (κρῑνω, κλῑνω, εἰρω, ἐκχεω, ἀειρω, ἰθῡνω). Like Θωᾰω, except in 80 cells: ind_act_3 pl, ind_mid_1 sg, ind_mid_2 sg, ind_pass_1 sg, subj_act_3 pl, subj_mid_2 sg, ….
    conjugation!("ἀειρω", 13, "~ω", like Θωᾰω;
    stem aorist_ind: "~- like ἐλπω";
    ind_act_3: _, _, "~ουσῐ";
    ind_mid_1: derived, _, _;
    ind_mid_2: "~ῃ", _, _;
    ind_pass_1: "~ομαι", _, _;
    subj_act_3: _, _, "~ωσῐ";
    subj_mid_2: derived, _, _;
    subj_pass_2: derived, _, _;
    imp_mid_2: "~ου", _, _;
    imperfect_act_1: "~ον", _, _;
    imperfect_act_2: "~ες", "~ετον", "~ετε";
    imperfect_act_3: "~ε", _, "~ον";
    imperfect_mid_1: "~ομην", _, "~ομεθᾰ";
    imperfect_mid_2: "~ου", _, _;
    imperfect_mid_3: "~ετο", _, "~οντο";
    imperfect_pass_2: _, "~εσθον", "~εσθε";
    imperfect_pass_3: _, "~εσθην", _;
    aorist_subj_act_1: "~ω", _, _;
    aorist_subj_act_2: "~ῃς", "~ητον", _;
    aorist_subj_mid_1: "~ωμαι", _, _;
    aorist_subj_mid_2: "~ῃ", "~ησθον", _;
    aorist_opt_act_1: "~αιμῐ", _, "~αιμεν";
    aorist_opt_act_2: "~ειᾰς", _, _;
    aorist_opt_act_3: "~ειε", _, "~ειᾰν";
    aorist_opt_mid_1: "~αιμην", _, _;
    aorist_imp_act_2: "~ον", _, "~ᾰτε";
    aorist_imp_act_3: _, _, "~ᾰντων";
    aorist_imp_mid_2: "~αι", "~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰσθων";
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: _, "~κᾰτον", _;
    perfect_ind_act_3: _, _, "~κᾱσῐ";
    perfect_ind_mid_1: _, _, "~μεθᾰ";
    perfect_ind_mid_2: "~σαι", "~θον", _;
    perfect_ind_mid_3: "~ται", _, "~ᾰται";
    perfect_ind_pass_1: "~μαι", _, _;
    perfect_ind_pass_2: _, _, "~θε";
    perfect_subj_act_1: "~κω", _, _;
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ";
    perfect_imp_act_2: "~κε", _, _;
    perfect_imp_mid_2: _, _, "~θε";
    perfect_imp_mid_3: _, _, "~θων";
    perfect_imp_pass_2: "~σο", _, "~θε";
    perfect_imp_pass_3: "~θω", "~θων", "~θων";
    pluperfect_act_1: "~κειν", _, _;
    pluperfect_mid_1: "~μην", _, _;
    pluperfect_mid_2: "~σο", "~θον", _;
    pluperfect_mid_3: "~το", _, "~ᾰτο";
    pluperfect_pass_1: _, _, "~μεθᾰ";
    pluperfect_pass_2: _, _, "~θε";
    pluperfect_pass_3: _, "~θην", _;
    infinitive_aor: "~αι", _, _;
    infinitive_perf: _, "~θαι", _;
    participle_aor: "~ᾱς", "~ᾰμενος", _;
    participle_perf: "~κως", _, "~μενος";
    ),
    // Ἀραομαι: 13 words (μναομαι, καυχᾰομαι, ἀραομαι, χρᾰομαι, κτᾰομαι, μᾰσαομαι). Like Νῑκαω, except in 91 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀραομαι", 13, "~αομαι", like Νῑκαω;
    stem future_pass: "~ησ- like ἐχω";
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
    future_ind_act_2: derived, derived, _;
    future_ind_act_3: _, _, derived;
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
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_imp_act_2: derived, _, _;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    participle_pres: derived, _, _;
    participle_aor: derived, _, _;
    participle_perf: derived, _, _;
    ),
    // Δῐοικεω: 12 words (δῐοικοδομεω, ἐνοικεω, προσοικεω, κᾰτοικοφθορεω, κατοικονομεω, κᾰτοικοδομεω). Like Πεμπω, except in 154 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("δῐοικεω", 12, "~οι~εω", like Πεμπω;
    stem future_pass: "~οι~ηθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ῳ~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ῳ~ηθην- like εὐλογεω";
    stem aorist_pass: "~οι~ηθ- like ποιεω";
    stem pluperfect_act: "~ῳ~ηκ- like εὐλογεω";
    ind_act_1: "~οι~ω", _, "~οι~ουμεν";
    ind_act_2: "~οι~εις", "~οι~ειτον", _;
    ind_act_3: "~οι~ει", _, "~οι~ουσῐ|~οι~ουσῐν";
    ind_mid_1: _, _, "~οι~ουμεθᾰ";
    ind_mid_2: "~οι~ει|~οι~ῃ", _, _;
    ind_mid_3: _, "~οι~εισθον", _;
    ind_pass_1: "~οι~ουμαι", _, _;
    ind_pass_2: _, "~οι~εισθον", "~οι~εισθε";
    ind_pass_3: "~οι~ειται", "~οι~εισθον", "~οι~ουνται";
    subj_act_1: "~οι~ω", _, "~οι~ωμεν";
    subj_act_2: "~οι~ῃς", _, _;
    subj_act_3: "~οι~ῃ", "~οι~ητον", "~οι~ωσῐ|~οι~ωσῐν";
    subj_mid_1: _, _, "~οι~ωμεθᾰ";
    subj_mid_2: _, "~οι~ησθον", _;
    subj_mid_3: "~οι~ηται", _, "~οι~ωνται";
    subj_pass_1: "~οι~ωμαι", _, _;
    subj_pass_2: _, _, "~οι~ησθε";
    opt_act_1: "~οι~οιην|~οι~οιμῐ", _, "~οι~οιμεν|~οι~οιημεν";
    opt_act_2: "~οι~οιης|~οι~οις", "~οι~οιτον|~οι~οιητον", "~οι~οιτε|~οι~οιητε";
    opt_act_3: "~οι~οιη|~οι~οι", "~οι~οιτην|~οι~οιητην", "~οι~οιεν|~οι~οιησᾰν";
    opt_mid_1: "~οι~οιμην", _, "~οι~οιμεθᾰ";
    opt_mid_2: "~οι~οιο", _, "~οι~οισθε";
    opt_mid_3: "~οι~οιτο", "~οι~οισθην", "~οι~οιντο";
    opt_pass_2: _, "~οι~οισθον", _;
    imp_act_2: "~οι~ει", _, "~οι~ειτε";
    imp_act_3: _, _, "~οι~ουντων";
    imp_mid_2: "~οι~ου", "~οι~εισθον", "~οι~εισθε";
    imp_mid_3: _, _, "~οι~εισθων";
    imp_pass_2: _, "~οι~εισθον", "~οι~εισθε";
    imp_pass_3: "~οι~εισθω", "~οι~εισθων", "~οι~εισθων";
    imperfect_act_1: "~ῳ~ουν", _, _;
    imperfect_act_2: "~ῳ~εις", "~ῳ~ειτον", "~ῳ~ειτε";
    imperfect_act_3: "~ῳ~ει", _, "~ῳ~ουν";
    imperfect_mid_1: "~ῳ~ουμην", _, "~ῳ~ουμεθᾰ";
    imperfect_mid_2: "~ῳ~ου", _, _;
    imperfect_mid_3: "~ῳ~ειτο", _, "~ῳ~ουντο";
    imperfect_pass_2: _, "~ῳ~εισθον", "~ῳ~εισθε";
    imperfect_pass_3: _, "~ῳ~εισθην", _;
    future_ind_act_1: "~οι~ησω", _, "~οι~ησομεν";
    future_ind_act_2: "~οι~ησεις", "~οι~ησετον", _;
    future_ind_act_3: _, _, "~οι~ησουσῐ|~οι~ησουσῐν";
    future_ind_mid_1: "~οι~ησομαι", _, _;
    future_ind_mid_2: "~οι~ησῃ|~οι~ησει", "~οι~ησεσθον", _;
    future_ind_mid_3: "~οι~ησεται", _, _;
    future_opt_act_1: "~οι~ησοιμῐ", _, "~οι~ησοιμεν";
    future_opt_act_2: "~οι~ησοις", "~οι~ησοιτον", _;
    future_opt_act_3: "~οι~ησοι", "~οι~ησοιτην", _;
    future_opt_mid_1: "~οι~ησοιμην", _, "~οι~ησοιμεθᾰ";
    future_opt_mid_3: _, "~οι~ησοισθην", _;
    aorist_subj_act_1: "~οι~ησω", _, _;
    aorist_subj_act_2: "~οι~ησῃς", "~οι~ησητον", _;
    aorist_subj_act_3: _, _, "~οι~ησωσῐ|~οι~ησωσῐν";
    aorist_subj_mid_1: "~οι~ησωμαι", _, _;
    aorist_subj_mid_2: "~οι~ησῃ", "~οι~ησησθον", _;
    aorist_opt_act_1: "~οι~ησαιμῐ", _, "~οι~ησαιμεν";
    aorist_opt_act_2: "~οι~ησειᾰς|~οι~ησαις", _, _;
    aorist_opt_act_3: "~οι~ησειε|~οι~ησειεν|~οι~ησαι", _, "~οι~ησειᾰν|~οι~ησαιεν";
    aorist_opt_mid_1: "~οι~ησαιμην", _, _;
    aorist_imp_act_2: "~οι~ησον", _, "~οι~ησᾰτε";
    aorist_imp_act_3: _, _, "~οι~ησᾰντων";
    aorist_imp_mid_2: "~οι~ησαι", "~οι~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~οι~ησᾰσθων";
    perfect_ind_act_1: "~ῳ~ηκᾰ", _, "~ῳ~ηκᾰμεν";
    perfect_ind_act_2: _, "~ῳ~ηκᾰτον", _;
    perfect_ind_act_3: "~ῳ~ηκε|~ῳ~ηκεν", _, "~ῳ~ηκᾱσῐ|~ῳ~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ῳ~ημεθᾰ";
    perfect_ind_mid_2: "~ῳ~ησαι", "~ῳ~ησθον", _;
    perfect_ind_mid_3: "~ῳ~ηται", _, "~ῳ~ηνται";
    perfect_ind_pass_1: "~ῳ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ῳ~ησθε";
    perfect_subj_act_1: "~ῳ~ηκω", _, _;
    perfect_subj_act_2: "~ῳ~ηκῃς", "~ῳ~ηκητον", "~ῳ~ηκητε";
    perfect_subj_act_3: "~ῳ~ηκῃ", _, "~ῳ~ηκωσῐ|~ῳ~ηκωσῐν";
    perfect_opt_act_1: "~ῳ~ηκοιμῐ|~ῳ~ηκοιην", _, _;
    perfect_opt_act_2: "~ῳ~ηκοις|~ῳ~ηκοιης", _, _;
    perfect_opt_act_3: "~ῳ~ηκοι|~ῳ~ηκοιη", _, _;
    perfect_imp_act_2: "~ῳ~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~ῳ~ησθε";
    perfect_imp_mid_3: _, _, "~ῳ~ησθων";
    perfect_imp_pass_2: "~ῳ~ησο", _, "~ῳ~ησθε";
    perfect_imp_pass_3: "~ῳ~ησθω", "~ῳ~ησθων", "~ῳ~ησθων";
    pluperfect_mid_1: "~ῳ~ημην", _, _;
    pluperfect_mid_2: "~ῳ~ησο", "~ῳ~ησθον", _;
    pluperfect_mid_3: "~ῳ~ητο", _, "~ῳ~ηντο";
    pluperfect_pass_1: _, _, "~ῳ~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ῳ~ησθε";
    pluperfect_pass_3: _, "~ῳ~ησθην", _;
    infinitive_pres: "~οι~ειν", _, "~οι~εισθαι";
    infinitive_fut: "~οι~ησειν", _, _;
    infinitive_aor: "~οι~ησαι", _, _;
    infinitive_perf: _, "~ῳ~ησθαι", _;
    participle_pres: "~οι~ων", _, "~οι~ουμενος";
    participle_aor: "~οι~ησᾱς", "~οι~ησᾰμενος", _;
    participle_perf: "~ῳ~ηκως", _, "~ῳ~ημενος";
    ),
    // Φαινω: 12 words (σημαινω, φαινω, ξαινω, ὑφαινω, ὁρμαινω, χᾰλεπαινω). Like Ληγω, except in 128 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("φαινω", 12, "~αινω", like Ληγω;
    stem future_pass: "~ᾰνθησ- like ἐχω";
    stem aorist_ind: "~ην- like ἐχω";
    stem aorist_ind_pass: "~ᾰνθην- like ἐχω";
    stem aorist_pass: "~ᾰνθ- like ἐχω";
    ind_act_1: "~αινω", _, "~αινομεν";
    ind_act_2: "~αινεις", "~αινετον", _;
    ind_act_3: "~αινει", _, "~αινουσῐ";
    ind_mid_1: _, _, "~αινομεθᾰ";
    ind_mid_2: "~αινῃ", _, _;
    ind_mid_3: _, "~αινεσθον", _;
    ind_pass_1: "~αινομαι", _, _;
    ind_pass_2: _, "~αινεσθον", "~αινεσθε";
    ind_pass_3: "~αινεται", "~αινεσθον", "~αινονται";
    subj_act_1: "~αινω", _, "~αινωμεν";
    subj_act_2: "~αινῃς", _, _;
    subj_act_3: "~αινῃ", "~αινητον", "~αινωσῐ";
    subj_mid_1: _, _, "~αινωμεθᾰ";
    subj_mid_2: _, "~αινησθον", _;
    subj_mid_3: "~αινηται", _, "~αινωνται";
    subj_pass_1: "~αινωμαι", _, _;
    subj_pass_2: _, _, "~αινησθε";
    opt_act_1: "~αινοιμῐ", _, "~αινοιμεν";
    opt_act_2: "~αινοις", "~αινοιτον", _;
    opt_act_3: "~αινοι", "~αινοιτην", "~αινοιεν";
    opt_mid_1: "~αινοιμην", _, "~αινοιμεθᾰ";
    opt_mid_2: "~αινοιο", _, "~αινοισθε";
    opt_mid_3: "~αινοιτο", "~αινοισθην", "~αινοιντο";
    opt_pass_2: _, "~αινοισθον", _;
    imp_act_2: "~αινε", _, "~αινετε";
    imp_act_3: _, _, "~αινοντων";
    imp_mid_2: "~αινου", "~αινεσθον", "~αινεσθε";
    imp_mid_3: _, _, "~αινεσθων";
    imp_pass_2: _, "~αινεσθον", "~αινεσθε";
    imp_pass_3: "~αινεσθω", "~αινεσθων", "~αινεσθων";
    imperfect_act_1: "ἐ~αινον", _, _;
    imperfect_act_2: "ἐ~αινες", "ἐ~αινετον", "ἐ~αινετε";
    imperfect_act_3: "ἐ~αινε", _, "ἐ~αινον";
    imperfect_mid_1: "ἐ~αινομην", _, "ἐ~αινομεθᾰ";
    imperfect_mid_2: "ἐ~αινου", _, _;
    imperfect_mid_3: "ἐ~αινετο", _, "ἐ~αινοντο";
    imperfect_pass_2: _, "ἐ~αινεσθον", "ἐ~αινεσθε";
    imperfect_pass_3: _, "ἐ~αινεσθην", _;
    future_ind_act_1: "~ᾰνω", _, "~ᾰνουμεν";
    future_ind_act_2: "~ᾰνεις", "~ᾰνειτον", _;
    future_ind_act_3: _, _, "~ᾰνουσῐ";
    future_ind_mid_1: "~ᾰνουμαι", _, _;
    future_ind_mid_2: "~ᾰνει", "~ᾰνεισθον", _;
    future_ind_mid_3: "~ᾰνειται", _, _;
    future_opt_act_1: "~ᾰνοιην", _, "~ᾰνοιμεν";
    future_opt_act_2: "~ᾰνοιης", "~ᾰνοιτον", _;
    future_opt_act_3: "~ᾰνοιη", "~ᾰνοιτην", _;
    future_opt_mid_1: "~ᾰνοιμην", _, "~ᾰνοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰνοισθην", _;
    aorist_subj_act_1: "~ηνω", _, _;
    aorist_subj_act_2: "~ηνῃς", "~ηνητον", _;
    aorist_subj_mid_1: "~ηνωμαι", _, _;
    aorist_subj_mid_2: "~ηνῃ", "~ηνησθον", _;
    aorist_opt_act_1: "~ηναιμῐ", _, "~ηναιμεν";
    aorist_opt_act_2: "~ηνειᾰς", _, _;
    aorist_opt_act_3: "~ηνειε", _, "~ηνειᾰν";
    aorist_opt_mid_1: "~ηναιμην", _, _;
    aorist_imp_act_2: "~ηνον", _, "~ηνᾰτε";
    aorist_imp_act_3: _, _, "~ηνᾰντων";
    aorist_imp_mid_2: "~ηναι", "~ηνᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ηνᾰσθων";
    perfect_ind_mid_1: _, _, "ἐ~ασμεθᾰ";
    perfect_ind_mid_2: "ἐ~ανσαι", "ἐ~ανθον", _;
    perfect_ind_mid_3: "ἐ~ανται", _, "ἐ~ανᾰται";
    perfect_ind_pass_1: "ἐ~ασμαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~ανθε";
    perfect_imp_mid_2: _, _, "ἐ~ανθε";
    perfect_imp_mid_3: _, _, "ἐ~ανθων";
    perfect_imp_pass_2: "ἐ~ανσο", _, "ἐ~ανθε";
    perfect_imp_pass_3: "ἐ~ανθω", "ἐ~ανθων", "ἐ~ανθων";
    infinitive_pres: "~αινειν", _, "~αινεσθαι";
    infinitive_fut: "~ᾰνειν", _, _;
    infinitive_aor: "~ηναι", _, _;
    infinitive_perf: _, "ἐ~ᾱνθαι", _;
    participle_pres: "~αινων", _, "~αινομενος";
    participle_aor: "~ηνᾱς", "~ηνᾰμενος", _;
    participle_perf: _, _, "ἐ~ασμενος";
    ),
    // Σβεννῡμῐ: 12 words (ῥωννῡμῐ, ἑννῡμῐ, πετᾰννῡμῐ, σβεννῡμῐ, κερᾰννῡμῐ, ζωννῡμῐ). Like Ἐλπω, except in 96 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("σβεννῡμῐ", 12, "~ννῡμῐ", like Ἐλπω;
    stem aorist_ind: "~σ- like ἐχω";
    stem aorist_ind_pass: "~σθην- like ἐχω";
    ind_act_1: "~ννῡμῐ", _, "~ννῠμεν";
    ind_act_2: "~ννῡς", "~ννῠτον", _;
    ind_act_3: "~ννῡσῐ", _, "~ννῠᾱσῐ";
    ind_mid_1: _, _, "~ννῠμεθᾰ";
    ind_mid_2: "~ννῠσαι", _, _;
    ind_mid_3: _, "~ννῠσθον", _;
    ind_pass_1: "~ννῠμαι", _, _;
    ind_pass_2: _, "~ννῠσθον", "~ννῠσθε";
    ind_pass_3: "~ννῠται", "~ννῠσθον", "~ννῠνται";
    subj_act_1: "~ννῠω", _, "~ννῠωμεν";
    subj_act_2: "~ννῠῃς", _, _;
    subj_act_3: "~ννῠῃ", "~ννῠητον", "~ννῠωσῐ";
    subj_mid_1: _, _, "~ννῠωμεθᾰ";
    subj_mid_2: _, "~ννῠησθον", _;
    subj_mid_3: "~ννῠηται", _, "~ννῠωνται";
    subj_pass_1: "~ννῠωμαι", _, _;
    subj_pass_2: _, _, "~ννῠησθε";
    opt_act_1: "~ννῠοιμῐ", _, "~ννῠοιμεν";
    opt_act_2: "~ννῠοις", "~ννῠοιτον", _;
    opt_act_3: "~ννῠοι", "~ννῠοιτην", "~ννῠοιεν";
    opt_mid_1: "~ννῠοιμην", _, "~ννῠοιμεθᾰ";
    opt_mid_2: "~ννῠοιο", _, "~ννῠοισθε";
    opt_mid_3: "~ννῠοιτο", "~ννῠοισθην", "~ννῠοιντο";
    opt_pass_2: _, "~ννῠοισθον", _;
    imp_act_2: "~ννῡ", _, "~ννῠτε";
    imp_act_3: _, _, "~ννῠντων";
    imp_mid_2: "~ννῠσο", "~ννῠσθον", "~ννῠσθε";
    imp_mid_3: _, _, "~ννῠσθων";
    imp_pass_2: _, "~ννῠσθον", "~ννῠσθε";
    imp_pass_3: "~ννῠσθω", "~ννῠσθων", "~ννῠσθων";
    imperfect_act_1: "ἐ~ννῡν", _, _;
    imperfect_act_2: "ἐ~ννῡς", "ἐ~ννῠτον", "ἐ~ννῠτε";
    imperfect_act_3: "ἐ~ννῡ", _, "ἐ~ννῠσᾰν";
    imperfect_mid_1: "ἐ~ννῠμην", _, "ἐ~ννῠμεθᾰ";
    imperfect_mid_2: "ἐ~ννῠσο", _, _;
    imperfect_mid_3: "ἐ~ννῠτο", _, "ἐ~ννῠντο";
    imperfect_pass_2: _, "ἐ~ννῠσθον", "ἐ~ννῠσθε";
    imperfect_pass_3: _, "ἐ~ννῠσθην", _;
    perfect_ind_mid_1: _, _, "ἐ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~σαι", "ἐ~σθον", _;
    perfect_ind_mid_3: "ἐ~ται", _, "ἐ~νται";
    perfect_ind_pass_1: "ἐ~μαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~σθε";
    perfect_imp_mid_2: _, _, "ἐ~σθε";
    perfect_imp_mid_3: _, _, "ἐ~σθων";
    perfect_imp_pass_2: "ἐ~σο", _, "ἐ~σθε";
    perfect_imp_pass_3: "ἐ~σθω", "ἐ~σθων", "ἐ~σθων";
    pluperfect_mid_1: "ἐ~μην", _, _;
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", _;
    pluperfect_mid_3: "ἐ~το", _, "ἐ~ντο";
    pluperfect_pass_1: _, _, "ἐ~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~σθε";
    pluperfect_pass_3: _, "ἐ~σθην", _;
    infinitive_pres: "~ννῠναι", _, "~ννῠσθαι";
    infinitive_perf: _, "ἐ~σθαι", _;
    participle_pres: "~ννῡς", _, "~ννῠμενος";
    participle_perf: _, _, "ἐ~μενος";
    ),
    // Ἐπᾰϊω: 12 words (ὑπᾰρχω, ὑπᾰκουω, δῐᾰνῠω, ἐξᾰρχω, ἐξᾰρτῡω, ἀπᾰρεσκω). Like Πεμπω, except in 150 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐπᾰϊω", 12, "~ᾰ~ω", like Πεμπω;
    stem future_pass: "~ᾰ~σθησ- like ἐχω";
    stem aorist_ind: "~η~σ- like εὐλογεω";
    stem aorist_ind_pass: "~η~σθην- like εὐλογεω";
    stem aorist_pass: "~ᾰ~σθ- like ἐχω";
    ind_act_1: "~ᾰ~ω", _, "~ᾰ~ομεν";
    ind_act_2: "~ᾰ~εις", "~ᾰ~ετον", _;
    ind_act_3: "~ᾰ~ει", _, "~ᾰ~ουσῐ|~ᾰ~ουσῐν";
    ind_mid_1: _, _, "~ᾰ~ομεθᾰ";
    ind_mid_2: "~ᾰ~ῃ|~ᾰ~ει", _, _;
    ind_mid_3: _, "~ᾰ~εσθον", _;
    ind_pass_1: "~ᾰ~ομαι", _, _;
    ind_pass_2: _, "~ᾰ~εσθον", "~ᾰ~εσθε";
    ind_pass_3: "~ᾰ~εται", "~ᾰ~εσθον", "~ᾰ~ονται";
    subj_act_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    subj_act_2: "~ᾰ~ῃς", _, _;
    subj_act_3: "~ᾰ~ῃ", "~ᾰ~ητον", "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    subj_mid_1: _, _, "~ᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ησθον", _;
    subj_mid_3: "~ᾰ~ηται", _, "~ᾰ~ωνται";
    subj_pass_1: "~ᾰ~ωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~ησθε";
    opt_act_1: "~ᾰ~οιμῐ", _, "~ᾰ~οιμεν";
    opt_act_2: "~ᾰ~οις", "~ᾰ~οιτον", _;
    opt_act_3: "~ᾰ~οι", "~ᾰ~οιτην", "~ᾰ~οιεν";
    opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~οιο", _, "~ᾰ~οισθε";
    opt_mid_3: "~ᾰ~οιτο", "~ᾰ~οισθην", "~ᾰ~οιντο";
    opt_pass_2: _, "~ᾰ~οισθον", _;
    imp_act_2: "~ᾰ~ε", _, "~ᾰ~ετε";
    imp_act_3: _, _, "~ᾰ~οντων";
    imp_mid_2: "~ᾰ~ου", "~ᾰ~εσθον", "~ᾰ~εσθε";
    imp_mid_3: _, _, "~ᾰ~εσθων";
    imp_pass_2: _, "~ᾰ~εσθον", "~ᾰ~εσθε";
    imp_pass_3: "~ᾰ~εσθω", "~ᾰ~εσθων", "~ᾰ~εσθων";
    imperfect_act_1: "~η~ον", _, _;
    imperfect_act_2: "~η~ες", "~η~ετον", "~η~ετε";
    imperfect_act_3: "~η~ε|~η~εν", _, "~η~ον";
    imperfect_mid_1: "~η~ομην", _, "~η~ομεθᾰ";
    imperfect_mid_2: "~η~ου", _, _;
    imperfect_mid_3: "~η~ετο", _, "~η~οντο";
    imperfect_pass_2: _, "~η~εσθον", "~η~εσθε";
    imperfect_pass_3: _, "~η~εσθην", _;
    future_ind_act_1: "~ᾰ~σω", _, "~ᾰ~σομεν";
    future_ind_act_2: "~ᾰ~σεις", "~ᾰ~σετον", _;
    future_ind_act_3: _, _, "~ᾰ~σουσῐ";
    future_ind_mid_1: "~ᾰ~σομαι", _, _;
    future_ind_mid_2: "~ᾰ~σῃ", "~ᾰ~σεσθον", _;
    future_ind_mid_3: "~ᾰ~σεται", _, _;
    future_opt_act_1: "~ᾰ~σοιμῐ", _, "~ᾰ~σοιμεν";
    future_opt_act_2: "~ᾰ~σοις", "~ᾰ~σοιτον", _;
    future_opt_act_3: "~ᾰ~σοι", "~ᾰ~σοιτην", _;
    future_opt_mid_1: "~ᾰ~σοιμην", _, "~ᾰ~σοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~σοισθην", _;
    aorist_subj_act_1: "~ᾰ~σω", _, _;
    aorist_subj_act_2: "~ᾰ~σῃς", "~ᾰ~σητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~σωσῐ|~ᾰ~σωσῐν";
    aorist_subj_mid_1: "~ᾰ~σωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~σῃ", "~ᾰ~σησθον", _;
    aorist_opt_act_1: "~ᾰ~σαιμῐ", _, "~ᾰ~σαιμεν";
    aorist_opt_act_2: "~ᾰ~σειᾰς|~ᾰ~σαις", _, _;
    aorist_opt_act_3: "~ᾰ~σειε|~ᾰ~σειεν|~ᾰ~σαι", _, "~ᾰ~σειᾰν|~ᾰ~σαιεν";
    aorist_opt_mid_1: "~ᾰ~σαιμην", _, _;
    aorist_imp_act_2: "~ᾰ~σον", _, "~ᾰ~σᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~σᾰντων";
    aorist_imp_mid_2: "~ᾰ~σαι", "~ᾰ~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~σᾰσθων";
    perfect_ind_act_1: "~η~κᾰ", _, "~η~κᾰμεν";
    perfect_ind_act_2: _, "~η~κᾰτον", _;
    perfect_ind_act_3: _, _, "~η~κᾱσῐ";
    perfect_ind_mid_1: _, _, "~η~σμεθᾰ";
    perfect_ind_mid_2: "~η~σαι", "~η~σθον", _;
    perfect_ind_mid_3: "~η~σται", _, "~η~τᾰται";
    perfect_ind_pass_1: "~η~σμαι", _, _;
    perfect_ind_pass_2: _, _, "~η~σθε";
    perfect_subj_act_1: "~η~κω", _, _;
    perfect_subj_act_2: "~η~κῃς", "~η~κητον", "~η~κητε";
    perfect_subj_act_3: "~η~κῃ", _, "~η~κωσῐ";
    perfect_imp_act_2: "~η~κε", _, _;
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
    infinitive_pres: "~ᾰ~ειν", _, "~ᾰ~εσθαι";
    infinitive_fut: "~ᾰ~σειν", _, _;
    infinitive_aor: "~ᾰ~σαι", _, _;
    infinitive_perf: _, "~η~σθαι", _;
    participle_pres: "~ᾰ~ων", _, "~ᾰ~ομενος";
    participle_aor: "~ᾰ~σᾱς", "~ᾰ~σᾰμενος", _;
    participle_perf: "~η~κως", _, "~η~σμενος";
    ),
    // Κειμαι: 11 words (ἐοικᾰ, κειμαι, δυναμαι, οἰδᾰ, δει, ἐτλην). Like Θωᾰω, except in 8 cells: ind_act_3 pl, ind_mid_1 sg, ind_mid_2 sg, ind_pass_1 sg, subj_act_3 pl, subj_mid_2 sg, ….
    conjugation!("κειμαι", 11, "~", like Θωᾰω;
    ind_act_3: _, _, "~ουσῐ";
    ind_mid_1: derived, _, _;
    ind_mid_2: "~ῃ", _, _;
    ind_pass_1: "~ομαι", _, _;
    subj_act_3: _, _, "~ωσῐ";
    subj_mid_2: derived, _, _;
    subj_pass_2: derived, _, _;
    imp_mid_2: "~ου", _, _;
    ),
    // Κᾰθιδρῡω: 11 words (ἰᾱτρευω, ἰσχῡω, ἑλκω, ἱκω, κᾰθιδρῡω, εἰκω). Like Ἐλπω, except in 36 cells: ind_mid_2 sg, future_ind_act_3 pl, future_ind_mid_2 sg, future_ind_pass_1 sg, future_ind_pass_3 du, aorist_ind_pass_1 sg, ….
    conjugation!("κᾰθιδρῡω", 11, "~ω", like Ἐλπω;
    stem aorist_ind: "~σ- like εὐλογεω";
    ind_mid_2: "~ῃ|~ει", _, _;
    future_ind_act_3: _, _, "~σουσῐ|~σουσῐν";
    future_ind_mid_2: "~σῃ|~σει", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", _, "~σειᾰν|~σαιεν";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: _, "~κᾰτον", _;
    perfect_ind_act_3: "~κε|~κεν", _, "~κᾱσῐ|~κᾱσῐν";
    perfect_ind_mid_1: _, _, "~μεθᾰ";
    perfect_ind_mid_3: "~ται", _, "~νται";
    perfect_ind_pass_1: "~μαι", _, _;
    perfect_subj_act_1: "~κω", _, _;
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ|~κωσῐν";
    perfect_opt_act_1: "~κοιμῐ|~κοιην", _, _;
    perfect_opt_act_2: "~κοις|~κοιης", _, _;
    perfect_opt_act_3: "~κοι|~κοιη", _, _;
    perfect_imp_act_2: "~κε", _, _;
    participle_perf: "~κως", _, "~μενος";
    ),
    // Ἐγειρω: 11 words (κειρω, τεινω, πειρω, θεινω, ἐγειρω, κτεινω). Like Πεμπω, except in 149 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐγειρω", 11, "~ει~ω", like Πεμπω;
    stem future_pass: "~ᾰ~ησ- like ἐχω";
    stem aorist_ind: "~ει~- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~ην- like ἐχω";
    stem aorist_pass: "~ᾰ~- like ἐγειρω";
    ind_act_1: "~ει~ω", _, "~ει~ομεν";
    ind_act_2: "~ει~εις", "~ει~ετον", _;
    ind_act_3: "~ει~ει", _, "~ει~ουσῐ";
    ind_mid_1: _, _, "~ει~ομεθᾰ";
    ind_mid_2: "~ει~ῃ", _, _;
    ind_mid_3: _, "~ει~εσθον", _;
    ind_pass_1: "~ει~ομαι", _, _;
    ind_pass_2: _, "~ει~εσθον", "~ει~εσθε";
    ind_pass_3: "~ει~εται", "~ει~εσθον", "~ει~ονται";
    subj_act_1: "~ει~ω", _, "~ει~ωμεν";
    subj_act_2: "~ει~ῃς", _, _;
    subj_act_3: "~ει~ῃ", "~ει~ητον", "~ει~ωσῐ";
    subj_mid_1: _, _, "~ει~ωμεθᾰ";
    subj_mid_2: _, "~ει~ησθον", _;
    subj_mid_3: "~ει~ηται", _, "~ει~ωνται";
    subj_pass_1: "~ει~ωμαι", _, _;
    subj_pass_2: _, _, "~ει~ησθε";
    opt_act_1: "~ει~οιμῐ", _, "~ει~οιμεν";
    opt_act_2: "~ει~οις", "~ει~οιτον", _;
    opt_act_3: "~ει~οι", "~ει~οιτην", "~ει~οιεν";
    opt_mid_1: "~ει~οιμην", _, "~ει~οιμεθᾰ";
    opt_mid_2: "~ει~οιο", _, "~ει~οισθε";
    opt_mid_3: "~ει~οιτο", "~ει~οισθην", "~ει~οιντο";
    opt_pass_2: _, "~ει~οισθον", _;
    imp_act_2: "~ει~ε", _, "~ει~ετε";
    imp_act_3: _, _, "~ει~οντων";
    imp_mid_2: "~ει~ου", "~ει~εσθον", "~ει~εσθε";
    imp_mid_3: _, _, "~ει~εσθων";
    imp_pass_2: _, "~ει~εσθον", "~ει~εσθε";
    imp_pass_3: "~ει~εσθω", "~ει~εσθων", "~ει~εσθων";
    imperfect_act_1: "ἐ~ει~ον", _, _;
    imperfect_act_2: "ἐ~ει~ες", "ἐ~ει~ετον", "ἐ~ει~ετε";
    imperfect_act_3: "ἐ~ει~ε", _, "ἐ~ει~ον";
    imperfect_mid_1: "ἐ~ει~ομην", _, "ἐ~ει~ομεθᾰ";
    imperfect_mid_2: "ἐ~ει~ου", _, _;
    imperfect_mid_3: "ἐ~ει~ετο", _, "ἐ~ει~οντο";
    imperfect_pass_2: _, "ἐ~ει~εσθον", "ἐ~ει~εσθε";
    imperfect_pass_3: _, "ἐ~ει~εσθην", _;
    future_ind_act_1: "~ε~ω", _, "~ε~ουμεν";
    future_ind_act_2: "~ε~εις", "~ε~ειτον", _;
    future_ind_act_3: _, _, "~ε~ουσῐ";
    future_ind_mid_1: "~ε~ουμαι", _, _;
    future_ind_mid_2: "~ε~ει", "~ε~εισθον", _;
    future_ind_mid_3: "~ε~ειται", _, _;
    future_opt_act_1: "~ε~οιην", _, "~ε~οιμεν";
    future_opt_act_2: "~ε~οιης", "~ε~οιτον", _;
    future_opt_act_3: "~ε~οιη", "~ε~οιτην", _;
    future_opt_mid_1: "~ε~οιμην", _, "~ε~οιμεθᾰ";
    future_opt_mid_3: _, "~ε~οισθην", _;
    aorist_subj_act_1: "~ει~ω", _, _;
    aorist_subj_act_2: "~ει~ῃς", "~ει~ητον", _;
    aorist_subj_mid_1: "~ει~ωμαι", _, _;
    aorist_subj_mid_2: "~ει~ῃ", "~ει~ησθον", _;
    aorist_opt_act_1: "~ει~αιμῐ", _, "~ει~αιμεν";
    aorist_opt_act_2: "~ει~ειᾰς", _, _;
    aorist_opt_act_3: "~ει~ειε", _, "~ει~ειᾰν";
    aorist_opt_mid_1: "~ει~αιμην", _, _;
    aorist_imp_act_2: "~ει~ον", _, "~ει~ᾰτε";
    aorist_imp_act_3: _, _, "~ει~ᾰντων";
    aorist_imp_mid_2: "~ει~αι", "~ει~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ει~ᾰσθων";
    perfect_ind_act_1: "ἐ~ᾰ~κᾰ", _, "ἐ~ᾰ~κᾰμεν";
    perfect_ind_act_2: _, "ἐ~ᾰ~κᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~ᾰ~κᾱσῐ";
    perfect_ind_mid_1: _, _, "ἐ~ᾰ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰ~σαι", "ἐ~ᾰ~θον", _;
    perfect_ind_mid_3: "ἐ~ᾰ~ται", _, "ἐ~ᾰ~ᾰται";
    perfect_ind_pass_1: "ἐ~ᾰ~μαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~ᾰ~θε";
    perfect_subj_act_1: "ἐ~ᾰ~κω", _, _;
    perfect_subj_act_2: "ἐ~ᾰ~κῃς", "ἐ~ᾰ~κητον", "ἐ~ᾰ~κητε";
    perfect_subj_act_3: "ἐ~ᾰ~κῃ", _, "ἐ~ᾰ~κωσῐ";
    perfect_imp_act_2: "ἐ~ᾰ~κε", _, _;
    perfect_imp_mid_2: _, _, "ἐ~ᾰ~θε";
    perfect_imp_mid_3: _, _, "ἐ~ᾰ~θων";
    perfect_imp_pass_2: "ἐ~ᾰ~σο", _, "ἐ~ᾰ~θε";
    perfect_imp_pass_3: "ἐ~ᾰ~θω", "ἐ~ᾰ~θων", "ἐ~ᾰ~θων";
    pluperfect_act_1: "ἐ~ᾰ~κειν", _, _;
    pluperfect_mid_1: "ἐ~ᾰ~μην", _, _;
    pluperfect_mid_2: "ἐ~ᾰ~σο", "ἐ~ᾰ~θον", _;
    pluperfect_mid_3: "ἐ~ᾰ~το", _, "ἐ~ᾰ~ᾰτο";
    pluperfect_pass_1: _, _, "ἐ~ᾰ~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ᾰ~θε";
    pluperfect_pass_3: _, "ἐ~ᾰ~θην", _;
    infinitive_pres: "~ει~ειν", _, "~ει~εσθαι";
    infinitive_fut: "~ε~ειν", _, _;
    infinitive_aor: "~ει~αι", _, _;
    infinitive_perf: _, "ἐ~ᾰ~θαι", _;
    participle_pres: "~ει~ων", _, "~ει~ομενος";
    participle_aor: "~ει~ᾱς", "~ει~ᾰμενος", _;
    participle_perf: "ἐ~ᾰ~κως", _, "ἐ~ᾰ~μενος";
    ),
    // Πειθω: 11 words (πειθω, περθω, ληθω, κλωθω, κευθω, δῐᾰπερθω). Like Σβεννῡμῐ, except in 100 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πειθω", 11, "~θω", like Σβεννῡμῐ;
    stem aorist_ind: "~σ- like ποιεω";
    ind_act_1: "~θω", _, "~θομεν";
    ind_act_2: "~θεις", "~θετον", _;
    ind_act_3: "~θει", _, "~θουσῐ|~θουσῐν";
    ind_mid_1: _, _, "~θομεθᾰ";
    ind_mid_2: "~θῃ", _, _;
    ind_mid_3: _, "~θεσθον", _;
    ind_pass_1: "~θομαι", _, _;
    ind_pass_2: _, "~θεσθον", "~θεσθε";
    ind_pass_3: "~θεται", "~θεσθον", "~θονται";
    subj_act_1: "~θω", _, "~θωμεν";
    subj_act_2: "~θῃς", _, _;
    subj_act_3: "~θῃ", "~θητον", "~θωσῐ|~θωσῐν";
    subj_mid_1: _, _, "~θωμεθᾰ";
    subj_mid_2: _, "~θησθον", _;
    subj_mid_3: "~θηται", _, "~θωνται";
    subj_pass_1: "~θωμαι", _, _;
    subj_pass_2: _, _, "~θησθε";
    opt_act_1: "~θοιμῐ", _, "~θοιμεν";
    opt_act_2: "~θοις", "~θοιτον", _;
    opt_act_3: "~θοι", "~θοιτην", "~θοιεν";
    opt_mid_1: "~θοιμην", _, "~θοιμεθᾰ";
    opt_mid_2: "~θοιο", _, "~θοισθε";
    opt_mid_3: "~θοιτο", "~θοισθην", "~θοιντο";
    opt_pass_2: _, "~θοισθον", _;
    imp_act_2: "~θε", _, "~θετε";
    imp_act_3: _, _, "~θοντων";
    imp_mid_2: "~θου", "~θεσθον", "~θεσθε";
    imp_mid_3: _, _, "~θεσθων";
    imp_pass_2: _, "~θεσθον", "~θεσθε";
    imp_pass_3: "~θεσθω", "~θεσθων", "~θεσθων";
    imperfect_act_1: "ἐ~θον", _, _;
    imperfect_act_2: "ἐ~θες", "ἐ~θετον", "ἐ~θετε";
    imperfect_act_3: "ἐ~θε", _, "ἐ~θον";
    imperfect_mid_1: "ἐ~θομην", _, "ἐ~θομεθᾰ";
    imperfect_mid_2: "ἐ~θου", _, _;
    imperfect_mid_3: "ἐ~θετο", _, "ἐ~θοντο";
    imperfect_pass_2: _, "ἐ~θεσθον", "ἐ~θεσθε";
    imperfect_pass_3: _, "ἐ~θεσθην", _;
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", _, "~σειᾰν|~σαιεν";
    perfect_ind_mid_1: _, _, "%ε~σμεθᾰ";
    perfect_ind_mid_2: "%ε~σαι", "%ε~σθον", _;
    perfect_ind_mid_3: "%ε~σται", _, "%ε~θᾰται";
    perfect_ind_pass_1: "%ε~σμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~σθε";
    perfect_imp_mid_2: _, _, "%ε~σθε";
    perfect_imp_mid_3: _, _, "%ε~σθων";
    perfect_imp_pass_2: "%ε~σο", _, "%ε~σθε";
    perfect_imp_pass_3: "%ε~σθω", "%ε~σθων", "%ε~σθων";
    pluperfect_mid_1: "ἐ%ε~σμην", _, _;
    pluperfect_mid_2: "ἐ%ε~σο", "ἐ%ε~σθον", _;
    pluperfect_mid_3: "ἐ%ε~στο", _, "ἐ%ε~σᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~σθε";
    pluperfect_pass_3: _, "ἐ%ε~σθην", _;
    infinitive_pres: "~θειν", _, "~θεσθαι";
    infinitive_perf: _, "%ε~σθαι", _;
    participle_pres: "~θων", _, "~θομενος";
    participle_perf: _, _, "%ε~σμενος";
    ),
    // Ἱημῐ: 11 words (ἱστημῐ, ἱημῐ, δαμνημῐ, πῐμπλημῐ, φημῐ, διατιθημῐ). Like Χρη with `` after each pattern (`μῐ` after η), except in 76 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ind_mid_1 pl, ….
    conjugation!("ἱημῐ", 11, "~ημῐ", like Χρη with "", "μῐ" after "η";
    ind_act_1: _, _, "~ᾰμεν";
    ind_act_2: "~ης", "~ᾰτον", _;
    ind_act_3: "~ησῐ", _, "~ᾱσῐ";
    ind_mid_1: _, _, "~ᾰμεθᾰ";
    ind_mid_2: "~ᾰσαι", _, _;
    ind_mid_3: _, "~ᾰσθον", _;
    ind_pass_1: "~ᾰμαι", _, _;
    ind_pass_2: _, "~ᾰσθον", "~ᾰσθε";
    ind_pass_3: "~ᾰται", "~ᾰσθον", "~ᾰνται";
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", _, _;
    subj_act_3: _, "~ητον", "~ωσῐ|~ωσῐν";
    subj_mid_1: _, _, "~ωμεθᾰ";
    subj_mid_2: _, "~ησθον", _;
    subj_mid_3: "~ηται", _, "~ωνται";
    subj_pass_1: "~ωμαι", _, _;
    subj_pass_2: _, _, "~ησθε";
    opt_act_1: "~αιην", _, "~αιμεν|~αιημεν";
    opt_act_2: "~αιης", "~αιτον|~αιητον", "~αιτε|~αιητε";
    opt_act_3: "~αιη", "~αιτην|~αιητην", "~αιεν|~αιησᾰν";
    opt_mid_1: "~αιμην", _, "~αιμεθᾰ";
    opt_mid_2: "~αιο", _, "~αισθε";
    opt_mid_3: "~αιτο", "~αισθην", "~αιντο";
    opt_pass_2: _, "~αισθον", _;
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
    future_ind_act_3: derived, _, _;
    future_ind_mid_3: derived, _, _;
    future_opt_act_3: derived, _, _;
    future_opt_mid_3: derived, _, _;
    infinitive_pres: "~ᾰναι", _, "~ᾰσθαι";
    infinitive_fut: derived, _, _;
    participle_pres: "~ᾱς", _, "~ᾰμενος";
    ),
    // Μῑγνῡμῐ: 10 words (πηγνῡμῐ, μῑγνῡμῐ, ζευγνῡμῐ, ἀνοιγνῡμῐ, οἰγνῡμῐ, ὀμοργνῡμῐ). Like Ληγω, except in 84 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("μῑγνῡμῐ", 10, "~γνῡμῐ", like Ληγω;
    ind_act_1: "~γνῡμῐ", _, "~γνῠμεν";
    ind_act_2: "~γνῡς", "~γνῠτον", _;
    ind_act_3: "~γνῡσῐ", _, "~γνῠᾱσῐ";
    ind_mid_1: _, _, "~γνῠμεθᾰ";
    ind_mid_2: "~γνῠσαι", _, _;
    ind_mid_3: _, "~γνῠσθον", _;
    ind_pass_1: "~γνῠμαι", _, _;
    ind_pass_2: _, "~γνῠσθον", "~γνῠσθε";
    ind_pass_3: "~γνῠται", "~γνῠσθον", "~γνῠνται";
    subj_act_1: "~γνῠω", _, "~γνῠωμεν";
    subj_act_2: "~γνῠῃς", _, _;
    subj_act_3: "~γνῠῃ", "~γνῠητον", "~γνῠωσῐ";
    subj_mid_1: _, _, "~γνῠωμεθᾰ";
    subj_mid_2: _, "~γνῠησθον", _;
    subj_mid_3: "~γνῠηται", _, "~γνῠωνται";
    subj_pass_1: "~γνῠωμαι", _, _;
    subj_pass_2: _, _, "~γνῠησθε";
    opt_act_1: "~γνῠοιμῐ", _, "~γνῠοιμεν";
    opt_act_2: "~γνῠοις", "~γνῠοιτον", _;
    opt_act_3: "~γνῠοι", "~γνῠοιτην", "~γνῠοιεν";
    opt_mid_1: "~γνῠοιμην", _, "~γνῠοιμεθᾰ";
    opt_mid_2: "~γνῠοιο", _, "~γνῠοισθε";
    opt_mid_3: "~γνῠοιτο", "~γνῠοισθην", "~γνῠοιντο";
    opt_pass_2: _, "~γνῠοισθον", _;
    imp_act_2: "~γνῡ", _, "~γνῠτε";
    imp_act_3: _, _, "~γνῠντων";
    imp_mid_2: "~γνῠσο", "~γνῠσθον", "~γνῠσθε";
    imp_mid_3: _, _, "~γνῠσθων";
    imp_pass_2: _, "~γνῠσθον", "~γνῠσθε";
    imp_pass_3: "~γνῠσθω", "~γνῠσθων", "~γνῠσθων";
    imperfect_act_1: "ἐ~γνῡν", _, _;
    imperfect_act_2: "ἐ~γνῡς", "ἐ~γνῠτον", "ἐ~γνῠτε";
    imperfect_act_3: "ἐ~γνῡ", _, "ἐ~γνῠσᾰν";
    imperfect_mid_1: "ἐ~γνῠμην", _, "ἐ~γνῠμεθᾰ";
    imperfect_mid_2: "ἐ~γνῠσο", _, _;
    imperfect_mid_3: "ἐ~γνῠτο", _, "ἐ~γνῠντο";
    imperfect_pass_2: _, "ἐ~γνῠσθον", "ἐ~γνῠσθε";
    imperfect_pass_3: _, "ἐ~γνῠσθην", _;
    perfect_ind_act_1: "%ε~χᾰ", _, "%ε~χᾰμεν";
    perfect_ind_act_2: _, "%ε~χᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~χᾱσῐ";
    perfect_subj_act_1: "%ε~χω", _, _;
    perfect_subj_act_2: "%ε~χῃς", "%ε~χητον", "%ε~χητε";
    perfect_subj_act_3: "%ε~χῃ", _, "%ε~χωσῐ";
    perfect_imp_act_2: "%ε~χε", _, _;
    infinitive_pres: "~γνῠναι", _, "~γνῠσθαι";
    participle_pres: "~γνῡς", _, "~γνῠμενος";
    participle_perf: "%ε~χως", _, _;
    ),
    // Αἱρεω: 10 words (πωλεω, αἱρεω, ἀγραυλεω, αἰνεω, βῑνεω, ὑμνεω). Like Ἐχω, except in 72 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("αἱρεω", 10, "~εω", like Ἐχω;
    ind_act_1: "~εω", _, "~εομεν";
    ind_act_2: "~εεις", "~εετον", _;
    ind_act_3: "~εει", _, "~εουσῐ";
    ind_mid_1: _, _, "~εομεθᾰ";
    ind_mid_2: "~εῃ", _, _;
    ind_mid_3: _, "~εεσθον", _;
    ind_pass_1: "~εομαι", _, _;
    ind_pass_2: _, "~εεσθον", "~εεσθε";
    ind_pass_3: "~εεται", "~εεσθον", "~εονται";
    subj_act_1: "~εω", _, "~εωμεν";
    subj_act_2: "~εῃς", _, _;
    subj_act_3: "~εῃ", "~εητον", "~εωσῐ";
    subj_mid_1: _, _, "~εωμεθᾰ";
    subj_mid_2: _, "~εησθον", _;
    subj_mid_3: "~εηται", _, "~εωνται";
    subj_pass_1: "~εωμαι", _, _;
    subj_pass_2: _, _, "~εησθε";
    opt_act_1: "~εοιμῐ", _, "~εοιμεν";
    opt_act_2: "~εοις", "~εοιτον", _;
    opt_act_3: "~εοι", "~εοιτην", "~εοιεν";
    opt_mid_1: "~εοιμην", _, "~εοιμεθᾰ";
    opt_mid_2: "~εοιο", _, "~εοισθε";
    opt_mid_3: "~εοιτο", "~εοισθην", "~εοιντο";
    opt_pass_2: _, "~εοισθον", _;
    imp_act_2: "~εε", _, "~εετε";
    imp_act_3: _, _, "~εοντων";
    imp_mid_2: "~εου", "~εεσθον", "~εεσθε";
    imp_mid_3: _, _, "~εεσθων";
    imp_pass_2: _, "~εεσθον", "~εεσθε";
    imp_pass_3: "~εεσθω", "~εεσθων", "~εεσθων";
    imperfect_act_1: "ἐ~εον", _, _;
    imperfect_act_2: "ἐ~εες", "ἐ~εετον", "ἐ~εετε";
    imperfect_act_3: "ἐ~εε", _, "ἐ~εον";
    imperfect_mid_1: "ἐ~εομην", _, "ἐ~εομεθᾰ";
    imperfect_mid_2: "ἐ~εου", _, _;
    imperfect_mid_3: "ἐ~εετο", _, "ἐ~εοντο";
    imperfect_pass_2: _, "ἐ~εεσθον", "ἐ~εεσθε";
    imperfect_pass_3: _, "ἐ~εεσθην", _;
    infinitive_pres: "~εειν", _, "~εεσθαι";
    participle_pres: "~εων", _, "~εομενος";
    ),
    // Οἰομαι: 10 words (πετομαι, βουλομαι, οἰομαι, οἰχομαι, ἁλῐσκομαι, ἁζομαι). Like Ἑπομαι, except in 52 cells: ind_mid_1 sg, ind_pass_1 sg, subj_mid_1 sg, subj_pass_1 sg, imp_mid_3 sg, imp_pass_3 sg, ….
    conjugation!("οἰομαι", 10, "~ομαι", like Ἑπομαι;
    stem aorist_ind_pass: "~ηθην- like ἐχω";
    stem aorist_pass: "~ηθ- like ἐχω";
    ind_mid_1: "~ομαι", _, _;
    ind_pass_1: derived, _, _;
    subj_mid_1: "~ωμαι", _, _;
    subj_pass_1: derived, _, _;
    imp_mid_3: "~εσθω", _, _;
    imp_pass_3: derived, _, _;
    imperfect_mid_2: _, "ἐ~εσθον", _;
    imperfect_pass_2: _, derived, _;
    future_ind_mid_1: "~ησομαι", _, _;
    future_ind_mid_2: "~ησῃ", "~ησεσθον", _;
    future_ind_mid_3: "~ησεται", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ησοισθην", _;
    aorist_ind_mid_1: derived, _, _;
    aorist_ind_mid_2: derived, derived, _;
    aorist_subj_mid_1: derived, _, _;
    aorist_subj_mid_2: derived, derived, _;
    aorist_opt_mid_1: derived, _, _;
    aorist_imp_mid_2: derived, derived, _;
    aorist_imp_mid_3: _, _, derived;
    perfect_ind_mid_1: _, _, "%ε~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ησαι", "%ε~ησθον", _;
    perfect_ind_mid_3: "%ε~ηται", _, "%ε~ηνται";
    perfect_ind_pass_1: "%ε~ημαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ησθε";
    perfect_imp_mid_2: _, _, "%ε~ησθε";
    perfect_imp_mid_3: _, _, "%ε~ησθων";
    perfect_imp_pass_2: "%ε~ησο", _, "%ε~ησθε";
    perfect_imp_pass_3: "%ε~ησθω", "%ε~ησθων", "%ε~ησθων";
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: _, "%ε~ησθαι", _;
    participle_aor: _, derived, _;
    participle_perf: _, _, "%ε~ημενος";
    ),
    // Χεω: 10 words (πνεω, θεω, πλεω, χεω, νεω, ἀγρεω). Like Κειμαι, except in 92 cells: ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_mid_3 du, ind_pass_2 du, ind_pass_2 pl, ….
    conjugation!("χεω", 10, "~ω", like Κειμαι;
    stem aorist_ind_pass: "~υσθην- like ἐχω";
    stem aorist_pass: "~υσθ- like ἐχω";
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
    future_ind_act_1: "~υσω", _, "~υσομεν";
    future_ind_act_2: "~υσεις", "~υσετον", _;
    future_ind_act_3: _, _, "~υσουσῐ";
    future_ind_mid_1: "~υσομαι", _, _;
    future_ind_mid_2: "~υσῃ", "~υσεσθον", _;
    future_ind_mid_3: "~υσεται", _, _;
    future_opt_act_1: "~υσοιμῐ", _, "~υσοιμεν";
    future_opt_act_2: "~υσοις", "~υσοιτον", _;
    future_opt_act_3: "~υσοι", "~υσοιτην", _;
    future_opt_mid_1: "~υσοιμην", _, "~υσοιμεθᾰ";
    future_opt_mid_3: _, "~υσοισθην", _;
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
    perfect_ind_act_1: "%ε~υκᾰ", _, "%ε~υκᾰμεν";
    perfect_ind_act_2: _, "%ε~υκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~υκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~υσμεθᾰ";
    perfect_ind_mid_2: "%ε~υσαι", "%ε~υσθον", _;
    perfect_ind_mid_3: "%ε~υσται", _, "%ε~υσᾰται";
    perfect_ind_pass_1: "%ε~υσμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~υσθε";
    perfect_subj_act_1: "%ε~υκω", _, _;
    perfect_subj_act_2: "%ε~υκῃς", "%ε~υκητον", "%ε~υκητε";
    perfect_subj_act_3: "%ε~υκῃ", _, "%ε~υκωσῐ";
    perfect_imp_act_2: "%ε~υκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~υσθε";
    perfect_imp_mid_3: _, _, "%ε~υσθων";
    perfect_imp_pass_2: "%ε~υσο", _, "%ε~υσθε";
    perfect_imp_pass_3: "%ε~υσθω", "%ε~υσθων", "%ε~υσθων";
    infinitive_pres: "~ιν", _, "~ισθαι";
    infinitive_fut: "~υσειν", _, _;
    infinitive_aor: "~υσαι", _, _;
    infinitive_perf: _, "%ε~υσθαι", _;
    participle_aor: "~υσᾱς", _, _;
    participle_perf: "%ε~υκως", _, "%ε~υσμενος";
    ),
    // Ἀφῑημῐ: 10 words (ἀφῑημῐ, σῠνῑημῐ, ἐπᾰνῐημῐ, προϊημῐ, ἀνιημῐ, ξῠνῑημῐ). Like Νῑκαω, except in 134 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀφῑημῐ", 10, "~ῑημῐ", like Νῑκαω;
    stem aorist_ind: "~ηκ- like εὐλογεω";
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
    opt_pass_2: _, derived, _;
    imp_act_2: "~ῑει", _, "~ῑετε";
    imp_act_3: _, _, "~ῑεντων";
    imp_mid_2: "~ῑεσο|~ῑου", "~ῑεσθον", "~ῑεσθε";
    imp_mid_3: _, _, "~ῑεσθων";
    imp_pass_2: _, "~ῑεσθον", "~ῑεσθε";
    imp_pass_3: "~ῑεσθω", "~ῑεσθων", "~ῑεσθων";
    imperfect_act_1: "~ῑην", _, _;
    imperfect_act_2: "~ῑεις", "~ῑετον", "~ῑετε";
    imperfect_act_3: "~ῑει", _, "~ῑεσᾰν";
    imperfect_mid_1: "~ῑεμην", _, "~ῑεμεθᾰ";
    imperfect_mid_2: "~ῑεσο", _, _;
    imperfect_mid_3: "~ῑετο", _, "~ῑεντο";
    imperfect_pass_2: _, "~ῑεσθον", "~ῑεσθε";
    imperfect_pass_3: _, "~ῑεσθην", _;
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    future_ind_mid_2: "~ησῃ|~ησει", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ω", _, _;
    aorist_subj_act_2: "~ῃς", "~ητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~ωμαι", _, _;
    aorist_subj_mid_2: "~ῃ", "~ησθον", _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_1: "~ειην", _, "~ειμεν";
    aorist_opt_act_2: "~ειης", _, _;
    aorist_opt_act_3: "~ειη", _, "~ειεν";
    aorist_opt_mid_1: "~ειμην", _, _;
    aorist_imp_act_2: "~ες", _, "~ετε";
    aorist_imp_act_3: _, _, "~εντων";
    aorist_imp_mid_2: "~ου", "~εσθον", _;
    aorist_imp_mid_3: _, _, "~εσθων";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: "~εικᾰ", _, "~εικᾰμεν";
    perfect_ind_act_2: _, "~εικᾰτον", _;
    perfect_ind_act_3: _, _, "~εικᾱσῐ";
    perfect_ind_mid_1: _, _, "~ειμεθᾰ";
    perfect_ind_mid_2: "~εισαι", "~εισθον", _;
    perfect_ind_mid_3: "~ειται", _, "~εινται";
    perfect_ind_pass_1: "~ειμαι", _, _;
    perfect_ind_pass_2: _, _, "~εισθε";
    perfect_subj_act_1: "~εικω", _, _;
    perfect_subj_act_2: "~εικῃς", "~εικητον", "~εικητε";
    perfect_subj_act_3: "~εικῃ", _, "~εικωσῐ";
    perfect_imp_act_2: "~εικε", _, _;
    perfect_imp_mid_2: _, _, "~εισθε";
    perfect_imp_mid_3: _, _, "~εισθων";
    perfect_imp_pass_2: "~εισο", _, "~εισθε";
    perfect_imp_pass_3: "~εισθω", "~εισθων", "~εισθων";
    infinitive_pres: "~ῑεναι", _, "~ῑεσθαι";
    infinitive_aor: "~ειναι", _, _;
    infinitive_perf: _, "~εισθαι", _;
    participle_pres: "~ῑεις", _, "~ῑεμενος";
    participle_aor: "~εις", "~εμενος", _;
    participle_perf: "~εικως", _, "~ειμενος";
    ),
    // Ἐνᾰντῐοομαι: 9 words (στραγγᾰλοομαι, χειροομαι, ἀπῐχθῠοομαι, κᾰτοικειοομαι, ἐνᾰντῐοομαι, δεξῐοομαι). Like Οἰομαι, except in 71 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἐνᾰντῐοομαι", 9, "~οομαι", like Οἰομαι;
    stem future_pass: "~ωθησ- like ἐχω";
    stem aorist_ind: "~ωσ- like ἑπομαι";
    stem aorist_ind_pass: "~ωθην- like ἐχω";
    stem aorist_pass: "~ωθ- like ἐχω";
    ind_act_1: "~οομαι", _, _;
    ind_mid_1: "~ουμαι", _, "~ουμεθᾰ";
    ind_mid_2: "~οι", _, _;
    ind_mid_3: _, "~ουσθον", _;
    ind_pass_2: _, "~ουσθον", "~ουσθε";
    ind_pass_3: "~ουται", "~ουσθον", "~ουνται";
    subj_mid_2: "~οι", "~ωσθον", _;
    subj_mid_3: "~ωται", _, _;
    subj_pass_2: "~οι", _, "~ωσθε";
    imp_mid_2: _, "~ουσθον", "~ουσθε";
    imp_mid_3: "~ουσθω", _, "~ουσθων";
    imp_pass_2: _, "~ουσθον", "~ουσθε";
    imp_pass_3: _, "~ουσθων", "~ουσθων";
    imperfect_mid_1: "ἐ~ουμην", _, "ἐ~ουμεθᾰ";
    imperfect_mid_2: _, "ἐ~ουσθον", _;
    imperfect_mid_3: "ἐ~ουτο", _, "ἐ~ουντο";
    imperfect_pass_2: _, _, "ἐ~ουσθε";
    imperfect_pass_3: _, "ἐ~ουσθην", _;
    future_ind_mid_1: "~ωσομαι", _, _;
    future_ind_mid_2: "~ωσῃ", "~ωσεσθον", _;
    future_ind_mid_3: "~ωσεται", _, _;
    future_opt_mid_1: "~ωσοιμην", _, "~ωσοιμεθᾰ";
    future_opt_mid_3: _, "~ωσοισθην", _;
    aorist_subj_mid_1: "~ωσωμαι", _, _;
    aorist_subj_mid_2: "~ωσῃ", "~ωσησθον", _;
    aorist_opt_mid_1: "~ωσαιμην", _, _;
    aorist_imp_mid_2: "~ωσαι", "~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ωσᾰσθων";
    perfect_ind_mid_1: _, _, "%ε~ωμεθᾰ";
    perfect_ind_mid_2: "%ε~ωσαι", "%ε~ωσθον", _;
    perfect_ind_mid_3: "%ε~ωται", _, "%ε~ωνται";
    perfect_ind_pass_1: "%ε~ωμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ωσθε";
    perfect_imp_mid_2: _, _, "%ε~ωσθε";
    perfect_imp_mid_3: _, _, "%ε~ωσθων";
    perfect_imp_pass_2: "%ε~ωσο", _, "%ε~ωσθε";
    perfect_imp_pass_3: "%ε~ωσθω", "%ε~ωσθων", "%ε~ωσθων";
    pluperfect_mid_1: "ἐ%ε~ωμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ωσο", "ἐ%ε~ωσθον", _;
    pluperfect_mid_3: "ἐ%ε~ωτο", _, "ἐ%ε~ωντο";
    pluperfect_pass_1: _, _, "ἐ%ε~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ωσθε";
    pluperfect_pass_3: _, "ἐ%ε~ωσθην", _;
    infinitive_pres: _, _, "~ουσθαι";
    infinitive_perf: _, "%ε~ωσθαι", _;
    participle_pres: _, _, "~ουμενος";
    participle_aor: _, "~ωσᾰμενος", _;
    participle_perf: _, _, "%ε~ωμενος";
    ),
    // Κᾰταφρονεω: 9 words (κᾰταφρονεω, κᾰταργεω, κᾰτηγορεω, λοιπογρᾰφεω, τερᾰτολογεω, τερᾰτουργεω). Like Πεμπω, except in 150 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("κᾰταφρονεω", 9, "~ᾰ~εω", like Πεμπω;
    stem future_pass: "~α~ηθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~α~ησ- like ποιεω";
    stem aorist_ind_pass: "~α~ηθην- like ἐχω";
    stem aorist_pass: "~α~ηθ- like ποιεω";
    stem pluperfect_act: "~α~ηκειν- like ἐχω";
    ind_act_1: "~α~ω", _, "~α~ουμεν";
    ind_act_2: "~α~εις", "~α~ειτον", _;
    ind_act_3: "~α~ει", _, "~α~ουσῐ|~α~ουσῐν";
    ind_mid_1: _, _, "~α~ουμεθᾰ";
    ind_mid_2: "~α~ει|~α~ῃ", _, _;
    ind_mid_3: _, "~α~εισθον", _;
    ind_pass_1: "~α~ουμαι", _, _;
    ind_pass_2: _, "~α~εισθον", "~α~εισθε";
    ind_pass_3: "~α~ειται", "~α~εισθον", "~α~ουνται";
    subj_act_1: "~α~ω", _, "~α~ωμεν";
    subj_act_2: "~α~ῃς", _, _;
    subj_act_3: "~α~ῃ", "~α~ητον", "~α~ωσῐ|~α~ωσῐν";
    subj_mid_1: _, _, "~α~ωμεθᾰ";
    subj_mid_2: _, "~α~ησθον", _;
    subj_mid_3: "~α~ηται", _, "~α~ωνται";
    subj_pass_1: "~α~ωμαι", _, _;
    subj_pass_2: _, _, "~α~ησθε";
    opt_act_1: "~α~οιην|~α~οιμῐ", _, "~α~οιμεν|~α~οιημεν";
    opt_act_2: "~α~οιης|~α~οις", "~α~οιτον|~α~οιητον", "~α~οιτε|~α~οιητε";
    opt_act_3: "~α~οιη|~α~οι", "~α~οιτην|~α~οιητην", "~α~οιεν|~α~οιησᾰν";
    opt_mid_1: "~α~οιμην", _, "~α~οιμεθᾰ";
    opt_mid_2: "~α~οιο", _, "~α~οισθε";
    opt_mid_3: "~α~οιτο", "~α~οισθην", "~α~οιντο";
    opt_pass_2: _, "~α~οισθον", _;
    imp_act_2: "~α~ει", _, "~α~ειτε";
    imp_act_3: _, _, "~α~ουντων";
    imp_mid_2: "~α~ου", "~α~εισθον", "~α~εισθε";
    imp_mid_3: _, _, "~α~εισθων";
    imp_pass_2: _, "~α~εισθον", "~α~εισθε";
    imp_pass_3: "~α~εισθω", "~α~εισθων", "~α~εισθων";
    imperfect_act_1: "ἐ~α~ουν", _, _;
    imperfect_act_2: "ἐ~α~εις", "ἐ~α~ειτον", "ἐ~α~ειτε";
    imperfect_act_3: "ἐ~α~ει", _, "ἐ~α~ουν";
    imperfect_mid_1: "ἐ~α~ουμην", _, "ἐ~α~ουμεθᾰ";
    imperfect_mid_2: "ἐ~α~ου", _, _;
    imperfect_mid_3: "ἐ~α~ειτο", _, "ἐ~α~ουντο";
    imperfect_pass_2: _, "ἐ~α~εισθον", "ἐ~α~εισθε";
    imperfect_pass_3: _, "ἐ~α~εισθην", _;
    future_ind_act_1: "~α~ησω", _, "~α~ησομεν";
    future_ind_act_2: "~α~ησεις", "~α~ησετον", _;
    future_ind_act_3: _, _, "~α~ησουσῐ|~α~ησουσῐν";
    future_ind_mid_1: "~α~ησομαι", _, _;
    future_ind_mid_2: "~α~ησῃ|~α~ησει", "~α~ησεσθον", _;
    future_ind_mid_3: "~α~ησεται", _, _;
    future_opt_act_1: "~α~ησοιμῐ", _, "~α~ησοιμεν";
    future_opt_act_2: "~α~ησοις", "~α~ησοιτον", _;
    future_opt_act_3: "~α~ησοι", "~α~ησοιτην", _;
    future_opt_mid_1: "~α~ησοιμην", _, "~α~ησοιμεθᾰ";
    future_opt_mid_3: _, "~α~ησοισθην", _;
    aorist_subj_act_1: "~α~ησω", _, _;
    aorist_subj_act_2: "~α~ησῃς", "~α~ησητον", _;
    aorist_subj_act_3: _, _, "~α~ησωσῐ|~α~ησωσῐν";
    aorist_subj_mid_1: "~α~ησωμαι", _, _;
    aorist_subj_mid_2: "~α~ησῃ", "~α~ησησθον", _;
    aorist_opt_act_1: "~α~ησαιμῐ", _, "~α~ησαιμεν";
    aorist_opt_act_2: "~α~ησειᾰς|~α~ησαις", _, _;
    aorist_opt_act_3: "~α~ησειε|~α~ησειεν|~α~ησαι", _, "~α~ησειᾰν|~α~ησαιεν";
    aorist_opt_mid_1: "~α~ησαιμην", _, _;
    aorist_imp_act_2: "~α~ησον", _, "~α~ησᾰτε";
    aorist_imp_act_3: _, _, "~α~ησᾰντων";
    aorist_imp_mid_2: "~α~ησαι", "~α~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~α~ησᾰσθων";
    perfect_ind_act_1: "%ε~α~ηκᾰ", _, "%ε~α~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~α~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~α~ηκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~α~ημεθᾰ";
    perfect_ind_mid_2: "%ε~α~ησαι", "%ε~α~ησθον", _;
    perfect_ind_mid_3: "%ε~α~ηται", _, "%ε~α~ηνται";
    perfect_ind_pass_1: "%ε~α~ημαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~α~ησθε";
    perfect_subj_act_1: "%ε~α~ηκω", _, _;
    perfect_subj_act_2: "%ε~α~ηκῃς", "%ε~α~ηκητον", "%ε~α~ηκητε";
    perfect_subj_act_3: "%ε~α~ηκῃ", _, "%ε~α~ηκωσῐ";
    perfect_imp_act_2: "%ε~α~ηκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~α~ησθε";
    perfect_imp_mid_3: _, _, "%ε~α~ησθων";
    perfect_imp_pass_2: "%ε~α~ησο", _, "%ε~α~ησθε";
    perfect_imp_pass_3: "%ε~α~ησθω", "%ε~α~ησθων", "%ε~α~ησθων";
    pluperfect_mid_1: "ἐ%ε~α~ημην", _, _;
    pluperfect_mid_2: "ἐ%ε~α~ησο", "ἐ%ε~α~ησθον", _;
    pluperfect_mid_3: "ἐ%ε~α~ητο", _, "ἐ%ε~α~ηντο";
    pluperfect_pass_1: _, _, "ἐ%ε~α~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~α~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~α~ησθην", _;
    infinitive_pres: "~α~ειν", _, "~α~εισθαι";
    infinitive_fut: "~α~ησειν", _, _;
    infinitive_aor: "~α~ησαι", _, _;
    infinitive_perf: _, "%ε~α~ησθαι", _;
    participle_pres: "~α~ων", _, "~α~ουμενος";
    participle_aor: "~α~ησᾱς", "~α~ησᾰμενος", _;
    participle_perf: "%ε~α~ηκως", _, "%ε~α~ημενος";
    ),
    // Εἰσερχομαι: 9 words (ἐξερχομαι, εἰσερχομαι, ἀπερχομαι, προσερχομαι, δῐερχομαι, πᾰρερχομαι). Like Οἰομαι, except in 102 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_3 du, ….
    conjugation!("εἰσερχομαι", 9, "~ερχομαι", like Οἰομαι;
    stem aorist_ind: "~ηλθ- like εἰσερχομαι";
    ind_act_1: "~ερχομαι", _, _;
    ind_mid_1: "~ερχομαι", _, "~ερχομεθᾰ";
    ind_mid_2: "~ερχῃ|~ερχει", "~ερχεσθον", _;
    ind_mid_3: _, derived, _;
    ind_pass_2: _, derived, "~ερχεσθε";
    ind_pass_3: "~ερχεται", derived, "~ερχονται";
    subj_mid_1: "~ερχωμαι", _, "~ερχωμεθᾰ";
    subj_mid_2: "~ερχῃ", "~ερχησθον", _;
    subj_mid_3: "~ερχηται", _, "~ερχωνται";
    subj_pass_2: "~ερχῃ", _, "~ερχησθε";
    opt_mid_1: "~ερχοιμην", _, "~ερχοιμεθᾰ";
    opt_mid_2: "~ερχοιο", _, "~ερχοισθε";
    opt_mid_3: "~ερχοιτο", "~ερχοισθην", "~ερχοιντο";
    opt_pass_2: _, "~ερχοισθον", _;
    imp_mid_2: "~ερχου", derived, "~ερχεσθε";
    imp_mid_3: "~ερχεσθω", _, "~ερχεσθων";
    imp_pass_2: _, derived, "~ερχεσθε";
    imp_pass_3: _, "~ερχεσθων", "~ερχεσθων";
    imperfect_mid_1: "~ηρχομην", _, "~ηρχομεθᾰ";
    imperfect_mid_2: "~ηρχου", "~ηρχεσθον", _;
    imperfect_mid_3: "~ηρχετο", _, "~ηρχοντο";
    imperfect_pass_2: _, _, "~ηρχεσθε";
    imperfect_pass_3: _, "~ηρχεσθην", _;
    future_ind_mid_1: "~ελευσομαι", _, _;
    future_ind_mid_2: "~ελευσῃ|~ελευσει", "~ελευσεσθον", _;
    future_ind_mid_3: "~ελευσεται", _, _;
    future_opt_mid_1: "~ελευσοιμην", _, "~ελευσοιμεθᾰ";
    future_opt_mid_3: _, "~ελευσοισθην", _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ελθω", _, _;
    aorist_subj_act_2: "~ελθῃς", "~ελθητον", _;
    aorist_subj_act_3: _, _, "~ελθωσῐ|~ελθωσῐν";
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_1: "~ελθοιμῐ", _, "~ελθοιμεν";
    aorist_opt_act_2: "~ελθοις", _, _;
    aorist_opt_act_3: "~ελθοι", _, "~ελθοιεν";
    aorist_imp_act_2: "~ελθε", _, "~ελθετε";
    aorist_imp_act_3: _, _, "~ελθοντων";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: "~εληλῠθᾰ", _, "~εληλυμεν";
    perfect_ind_act_2: _, "~εληλῠθᾰτον", _;
    perfect_ind_act_3: _, _, "~εληλῠθᾱσῐ";
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "~εληλῠθω", _, _;
    perfect_subj_act_2: "~εληλῠθῃς", "~εληλῠθητον", "~εληλῠθητε";
    perfect_subj_act_3: "~εληλῠθῃ", _, "~εληλῠθωσῐ";
    perfect_imp_act_2: "~εληλῠθε", _, _;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    pluperfect_act_1: "~εληλῠθειν", _, _;
    infinitive_pres: _, _, "~ερχεσθαι";
    infinitive_aor: "~ελθειν", _, _;
    infinitive_perf: _, derived, _;
    participle_pres: _, _, "~ερχομενος";
    participle_aor: "~ελθων", _, _;
    participle_perf: "~εληλῠθως", _, derived;
    ),
    // Ὀμνῡμῐ: 9 words (δεικνῡμῐ, ὀρνῡμῐ, ὀμνῡμῐ, δαινῡμῐ, στορνῡμῐ, ἐπορνῡμῐ). Like Σβεννῡμῐ, except in 104 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ὀμνῡμῐ", 9, "~νῡμῐ", like Σβεννῡμῐ;
    ind_act_1: "~νῡμῐ", _, "~νῠμεν";
    ind_act_2: "~νῡς", "~νῠτον", _;
    ind_act_3: "~νῡσῐ", _, "~νῠᾱσῐ";
    ind_mid_1: _, _, "~νῠμεθᾰ";
    ind_mid_2: "~νῠσαι", _, _;
    ind_mid_3: _, "~νῠσθον", _;
    ind_pass_1: "~νῠμαι", _, _;
    ind_pass_2: _, "~νῠσθον", "~νῠσθε";
    ind_pass_3: "~νῠται", "~νῠσθον", "~νῠνται";
    subj_act_1: "~νῠω", _, "~νῠωμεν";
    subj_act_2: "~νῠῃς", _, _;
    subj_act_3: "~νῠῃ", "~νῠητον", "~νῠωσῐ";
    subj_mid_1: _, _, "~νῠωμεθᾰ";
    subj_mid_2: _, "~νῠησθον", _;
    subj_mid_3: "~νῠηται", _, "~νῠωνται";
    subj_pass_1: "~νῠωμαι", _, _;
    subj_pass_2: _, _, "~νῠησθε";
    opt_act_1: "~νῠοιμῐ", _, "~νῠοιμεν";
    opt_act_2: "~νῠοις", "~νῠοιτον", _;
    opt_act_3: "~νῠοι", "~νῠοιτην", "~νῠοιεν";
    opt_mid_1: "~νῠοιμην", _, "~νῠοιμεθᾰ";
    opt_mid_2: "~νῠοιο", _, "~νῠοισθε";
    opt_mid_3: "~νῠοιτο", "~νῠοισθην", "~νῠοιντο";
    opt_pass_2: _, "~νῠοισθον", _;
    imp_act_2: "~νῡ", _, "~νῠτε";
    imp_act_3: _, _, "~νῠντων";
    imp_mid_2: "~νῠσο", "~νῠσθον", "~νῠσθε";
    imp_mid_3: _, _, "~νῠσθων";
    imp_pass_2: _, "~νῠσθον", "~νῠσθε";
    imp_pass_3: "~νῠσθω", "~νῠσθων", "~νῠσθων";
    imperfect_act_1: "ἐ~νῡν", _, _;
    imperfect_act_2: "ἐ~νῡς", "ἐ~νῠτον", "ἐ~νῠτε";
    imperfect_act_3: "ἐ~νῡ", _, "ἐ~νῠσᾰν";
    imperfect_mid_1: "ἐ~νῠμην", _, "ἐ~νῠμεθᾰ";
    imperfect_mid_2: "ἐ~νῠσο", _, _;
    imperfect_mid_3: "ἐ~νῠτο", _, "ἐ~νῠντο";
    imperfect_pass_2: _, "ἐ~νῠσθον", "ἐ~νῠσθε";
    imperfect_pass_3: _, "ἐ~νῠσθην", _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
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
    infinitive_pres: "~νῠναι", _, "~νῠσθαι";
    infinitive_perf: _, derived, _;
    participle_pres: "~νῡς", _, "~νῠμενος";
    participle_perf: _, _, derived;
    ),
    // Ἀθῡρω: 8 words (ἀγρευω, ἀτασθαλλω, ἀμορβευω, ἀτεμβω, ἀφραινω, ἀπῐστευω). Like Ἐχω, except in 175 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀθῡρω", 8, "ἀ~ω", like Ἐχω;
    stem aorist_ind: "~σ- like ἀθῡρω";
    stem aorist_ind_pass: "~θην- like ἀθῡρω";
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
    opt_act_2: "ἀ~οις", "ἀ~οιτον", _;
    opt_act_3: "ἀ~οι", "ἀ~οιτην", "ἀ~οιεν";
    opt_mid_1: "ἀ~οιμην", _, "ἀ~οιμεθᾰ";
    opt_mid_2: "ἀ~οιο", _, "ἀ~οισθε";
    opt_mid_3: "ἀ~οιτο", "ἀ~οισθην", "ἀ~οιντο";
    opt_pass_2: _, "ἀ~οισθον", _;
    imp_act_2: "ἀ~ε", _, "ἀ~ετε";
    imp_act_3: _, _, "ἀ~οντων";
    imp_mid_2: "ἀ~ου", "ἀ~εσθον", "ἀ~εσθε";
    imp_mid_3: _, _, "ἀ~εσθων";
    imp_pass_2: _, "ἀ~εσθον", "ἀ~εσθε";
    imp_pass_3: "ἀ~εσθω", "ἀ~εσθων", "ἀ~εσθων";
    imperfect_act_1: "ἠ~ον", _, _;
    imperfect_act_2: "ἠ~ες", "ἠ~ετον", "ἠ~ετε";
    imperfect_act_3: "ἠ~ε|ἠ~εν", _, "ἠ~ον";
    imperfect_mid_1: "ἠ~ομην", _, "ἠ~ομεθᾰ";
    imperfect_mid_2: "ἠ~ου", _, _;
    imperfect_mid_3: "ἠ~ετο", _, "ἠ~οντο";
    imperfect_pass_2: _, "ἠ~εσθον", "ἠ~εσθε";
    imperfect_pass_3: _, "ἠ~εσθην", _;
    future_ind_act_1: "ἀ~σω", _, "ἀ~σομεν";
    future_ind_act_2: "ἀ~σεις", "ἀ~σετον", _;
    future_ind_act_3: _, _, "ἀ~σουσῐ";
    future_ind_mid_1: "ἀ~σομαι", _, _;
    future_ind_mid_2: "ἀ~σῃ", "ἀ~σεσθον", _;
    future_ind_mid_3: "ἀ~σεται", _, _;
    future_ind_pass_1: "ἀ~θησομαι", _, _;
    future_ind_pass_3: _, "ἀ~θησεσθον", _;
    future_opt_act_1: "ἀ~σοιμῐ", _, "ἀ~σοιμεν";
    future_opt_act_2: "ἀ~σοις", "ἀ~σοιτον", _;
    future_opt_act_3: "ἀ~σοι", "ἀ~σοιτην", _;
    future_opt_mid_1: "ἀ~σοιμην", _, "ἀ~σοιμεθᾰ";
    future_opt_mid_3: _, "ἀ~σοισθην", _;
    aorist_subj_act_1: "ἀ~σω", _, _;
    aorist_subj_act_2: "ἀ~σῃς", "ἀ~σητον", _;
    aorist_subj_mid_1: "ἀ~σωμαι", _, _;
    aorist_subj_mid_2: "ἀ~σῃ", "ἀ~σησθον", _;
    aorist_subj_pass_3: "ἀ~θῃ", "ἀ~θητον", _;
    aorist_opt_act_1: "ἀ~σαιμῐ", _, "ἀ~σαιμεν";
    aorist_opt_act_2: "ἀ~σειᾰς", _, _;
    aorist_opt_act_3: "ἀ~σειε", _, "ἀ~σειᾰν";
    aorist_opt_mid_1: "ἀ~σαιμην", _, _;
    aorist_imp_act_2: "ἀ~σον", _, "ἀ~σᾰτε";
    aorist_imp_act_3: _, _, "ἀ~σᾰντων";
    aorist_imp_mid_2: "ἀ~σαι", "ἀ~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἀ~σᾰσθων";
    aorist_imp_pass_2: "ἀ~θητῐ", "ἀ~θητον", "ἀ~θητε";
    perfect_ind_act_1: "ἠ~κᾰ", _, "ἠ~κᾰμεν";
    perfect_ind_act_2: _, "ἠ~κᾰτον", _;
    perfect_ind_act_3: _, _, "ἠ~κᾱσῐ";
    perfect_ind_mid_1: _, _, "ἠ~μεθᾰ";
    perfect_ind_mid_2: "ἠ~σαι", "ἠ~σθον", _;
    perfect_ind_mid_3: "ἠ~ται", _, "ἠ~νται";
    perfect_ind_pass_1: "ἠ~μαι", _, _;
    perfect_ind_pass_2: _, _, "ἠ~σθε";
    perfect_subj_act_1: "ἠ~κω", _, _;
    perfect_subj_act_2: "ἠ~κῃς", "ἠ~κητον", "ἠ~κητε";
    perfect_subj_act_3: "ἠ~κῃ", _, "ἠ~κωσῐ";
    perfect_imp_act_2: "ἠ~κε", _, _;
    perfect_imp_mid_2: _, _, "ἠ~σθε";
    perfect_imp_mid_3: _, _, "ἠ~σθων";
    perfect_imp_pass_2: "ἠ~σο", _, "ἠ~σθε";
    perfect_imp_pass_3: "ἠ~σθω", "ἠ~σθων", "ἠ~σθων";
    pluperfect_act_1: "ἠ~κειν", _, _;
    pluperfect_mid_1: "ἠ~μην", _, _;
    pluperfect_mid_2: "ἠ~σο", "ἠ~σθον", _;
    pluperfect_mid_3: "ἠ~το", _, "ἠ~ντο";
    pluperfect_pass_1: _, _, "ἠ~μεθᾰ";
    pluperfect_pass_2: _, _, "ἠ~σθε";
    pluperfect_pass_3: _, "ἠ~σθην", _;
    future_perfect_ind_mid_1: "ἠ~σομαι", _, _;
    future_perfect_ind_mid_3: _, "ἠ~σεσθον", _;
    future_perfect_ind_pass_1: _, _, "ἠ~σομεθᾰ";
    future_perfect_ind_pass_2: "ἠ~σῃ", "ἠ~σεσθον", "ἠ~σεσθε";
    future_perfect_ind_pass_3: "ἠ~σεται", "ἠ~σεσθον", "ἠ~σονται";
    future_perfect_opt_pass_1: "ἠ~σοιμην", _, "ἠ~σοιμεθᾰ";
    future_perfect_opt_pass_2: "ἠ~σοιο", "ἠ~σοισθον", "ἠ~σοισθε";
    future_perfect_opt_pass_3: "ἠ~σοιτο", "ἠ~σοισθην", "ἠ~σοιντο";
    infinitive_pres: "ἀ~ειν", _, "ἀ~εσθαι";
    infinitive_fut: "ἀ~σειν", _, _;
    infinitive_aor: "ἀ~σαι", _, _;
    infinitive_perf: _, "ἠ~σθαι", _;
    infinitive_futp: _, _, "ἠ~σεσθαι";
    participle_pres: "ἀ~ων", _, "ἀ~ομενος";
    participle_aor: "ἀ~σᾱς", "ἀ~σᾰμενος", _;
    participle_perf: "ἠ~κως", _, "ἠ~μενος";
    participle_futp: _, _, "ἠ~σομενος";
    ),
    // Ψῑλοω: 8 words (ἱδροω, στενοω, πτεροω, ψῑλοω, κεραυνοω, φορτοω). Like Γῡροω, except in 41 cells: future_ind_act_3 pl, perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, ….
    conjugation!("ψῑλοω", 8, "~οω", like Γῡροω;
    stem future_pass: "~ωθησ- like χρημᾰτῐζω";
    stem aorist_pass: "~ωθ- like ποιεω";
    stem pluperfect_act: "~ωκ- like δῐκεω";
    future_ind_act_3: _, _, "~ωσουσῐ|~ωσουσῐν";
    perfect_ind_act_1: "ἐ~ωκᾰ", _, "ἐ~ωκᾰμεν";
    perfect_ind_act_2: _, "ἐ~ωκᾰτον", _;
    perfect_ind_act_3: "ἐ~ωκε|ἐ~ωκεν", _, "ἐ~ωκᾱσῐ|ἐ~ωκᾱσῐν";
    perfect_ind_mid_1: _, _, "ἐ~ωμεθᾰ";
    perfect_ind_mid_2: "ἐ~ωσαι", "ἐ~ωσθον", _;
    perfect_ind_mid_3: "ἐ~ωται", _, "ἐ~ωνται";
    perfect_ind_pass_1: "ἐ~ωμαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~ωσθε";
    perfect_subj_act_1: "ἐ~ωκω", _, _;
    perfect_subj_act_2: "ἐ~ωκῃς", "ἐ~ωκητον", "ἐ~ωκητε";
    perfect_subj_act_3: "ἐ~ωκῃ", _, "ἐ~ωκωσῐ|ἐ~ωκωσῐν";
    perfect_opt_act_1: "ἐ~ωκοιμῐ|ἐ~ωκοιην", _, _;
    perfect_opt_act_2: "ἐ~ωκοις|ἐ~ωκοιης", _, _;
    perfect_opt_act_3: "ἐ~ωκοι|ἐ~ωκοιη", _, _;
    perfect_imp_act_2: "ἐ~ωκε", _, _;
    perfect_imp_mid_2: _, _, "ἐ~ωσθε";
    perfect_imp_mid_3: _, _, "ἐ~ωσθων";
    perfect_imp_pass_2: "ἐ~ωσο", _, "ἐ~ωσθε";
    perfect_imp_pass_3: "ἐ~ωσθω", "ἐ~ωσθων", "ἐ~ωσθων";
    pluperfect_mid_1: "ἐ~ωμην", _, _;
    pluperfect_mid_2: "ἐ~ωσο", "ἐ~ωσθον", _;
    pluperfect_mid_3: "ἐ~ωτο", _, "ἐ~ωντο";
    pluperfect_pass_1: _, _, "ἐ~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ωσθε";
    pluperfect_pass_3: _, "ἐ~ωσθην", _;
    infinitive_perf: _, "ἐ~ωσθαι", _;
    participle_perf: "ἐ~ωκως", _, "ἐ~ωμενος";
    ),
    // Ἀνᾰγω: 8 words (ἐξᾰγω, ὑπᾰγω, ἀνᾰγω, πᾰρᾰγω, ἐπᾰγω, κᾰτᾰγω). Like Ληγω, except in 141 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀνᾰγω", 8, "~ᾰγω", like Ληγω;
    stem future_pass: "~ᾰχθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ηγᾰγ- like ἀνᾰγω";
    stem aorist_ind_pass: "~ηχθην- like εὐλογεω";
    stem aorist_pass: "~ᾰχθ- like ἐχω";
    ind_act_1: "~ᾰγω", _, "~ᾰγομεν";
    ind_act_2: "~ᾰγεις", "~ᾰγετον", _;
    ind_act_3: "~ᾰγει", _, "~ᾰγουσῐ|~ᾰγουσῐν";
    ind_mid_1: _, _, "~ᾰγομεθᾰ";
    ind_mid_2: "~ᾰγῃ|~ᾰγει", _, _;
    ind_mid_3: _, "~ᾰγεσθον", _;
    ind_pass_1: "~ᾰγομαι", _, _;
    ind_pass_2: _, "~ᾰγεσθον", "~ᾰγεσθε";
    ind_pass_3: "~ᾰγεται", "~ᾰγεσθον", "~ᾰγονται";
    subj_act_1: "~ᾰγω", _, "~ᾰγωμεν";
    subj_act_2: "~ᾰγῃς", _, _;
    subj_act_3: "~ᾰγῃ", "~ᾰγητον", "~ᾰγωσῐ|~ᾰγωσῐν";
    subj_mid_1: _, _, "~ᾰγωμεθᾰ";
    subj_mid_2: _, "~ᾰγησθον", _;
    subj_mid_3: "~ᾰγηται", _, "~ᾰγωνται";
    subj_pass_1: "~ᾰγωμαι", _, _;
    subj_pass_2: _, _, "~ᾰγησθε";
    opt_act_1: "~ᾰγοιμῐ", _, "~ᾰγοιμεν";
    opt_act_2: "~ᾰγοις", "~ᾰγοιτον", _;
    opt_act_3: "~ᾰγοι", "~ᾰγοιτην", "~ᾰγοιεν";
    opt_mid_1: "~ᾰγοιμην", _, "~ᾰγοιμεθᾰ";
    opt_mid_2: "~ᾰγοιο", _, "~ᾰγοισθε";
    opt_mid_3: "~ᾰγοιτο", "~ᾰγοισθην", "~ᾰγοιντο";
    opt_pass_2: _, "~ᾰγοισθον", _;
    imp_act_2: "~ᾰγε", _, "~ᾰγετε";
    imp_act_3: _, _, "~ᾰγοντων";
    imp_mid_2: "~ᾰγου", "~ᾰγεσθον", "~ᾰγεσθε";
    imp_mid_3: _, _, "~ᾰγεσθων";
    imp_pass_2: _, "~ᾰγεσθον", "~ᾰγεσθε";
    imp_pass_3: "~ᾰγεσθω", "~ᾰγεσθων", "~ᾰγεσθων";
    imperfect_act_1: "~ηγον", _, _;
    imperfect_act_2: "~ηγες", "~ηγετον", "~ηγετε";
    imperfect_act_3: "~ηγε|~ηγεν", _, "~ηγον";
    imperfect_mid_1: "~ηγομην", _, "~ηγομεθᾰ";
    imperfect_mid_2: "~ηγου", _, _;
    imperfect_mid_3: "~ηγετο", _, "~ηγοντο";
    imperfect_pass_2: _, "~ηγεσθον", "~ηγεσθε";
    imperfect_pass_3: _, "~ηγεσθην", _;
    future_ind_act_1: "~ᾰξω", _, "~ᾰξομεν";
    future_ind_act_2: "~ᾰξεις", "~ᾰξετον", _;
    future_ind_act_3: _, _, "~ᾰξουσῐ|~ᾰξουσῐν";
    future_ind_mid_1: "~ᾰξομαι", _, _;
    future_ind_mid_2: "~ᾰξῃ|~ᾰξει", "~ᾰξεσθον", _;
    future_ind_mid_3: "~ᾰξεται", _, _;
    future_opt_act_1: "~ᾰξοιμῐ", _, "~ᾰξοιμεν";
    future_opt_act_2: "~ᾰξοις", "~ᾰξοιτον", _;
    future_opt_act_3: "~ᾰξοι", "~ᾰξοιτην", _;
    future_opt_mid_1: "~ᾰξοιμην", _, "~ᾰξοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰξοισθην", _;
    aorist_subj_act_1: "~ᾰγᾰγω", _, _;
    aorist_subj_act_2: "~ᾰγᾰγῃς", "~ᾰγᾰγητον", _;
    aorist_subj_act_3: _, _, "~ᾰγᾰγωσῐ|~ᾰγᾰγωσῐν";
    aorist_subj_mid_1: "~ᾰγᾰγωμαι", _, _;
    aorist_subj_mid_2: "~ᾰγᾰγῃ", "~ᾰγᾰγησθον", _;
    aorist_opt_act_1: "~ᾰγᾰγοιμῐ", _, "~ᾰγᾰγοιμεν";
    aorist_opt_act_2: "~ᾰγᾰγοις", _, _;
    aorist_opt_act_3: "~ᾰγᾰγοι", _, "~ᾰγᾰγοιεν";
    aorist_opt_mid_1: "~ᾰγᾰγοιμην", _, _;
    aorist_imp_act_2: "~ᾰγᾰγε", _, "~ᾰγᾰγετε";
    aorist_imp_act_3: _, _, "~ᾰγᾰγοντων";
    aorist_imp_mid_2: "~ᾰγᾰγου", "~ᾰγᾰγεσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰγᾰγεσθων";
    perfect_ind_act_1: "~ηχᾰ", _, "~ηχᾰμεν";
    perfect_ind_act_2: _, "~ηχᾰτον", _;
    perfect_ind_act_3: _, _, "~ηχᾱσῐ";
    perfect_ind_mid_1: _, _, "~ηγμεθᾰ";
    perfect_ind_mid_2: "~ηξαι", "~ηχθον", _;
    perfect_ind_mid_3: "~ηκται", _, "~ηγᾰται";
    perfect_ind_pass_1: "~ηγμαι", _, _;
    perfect_ind_pass_2: _, _, "~ηχθε";
    perfect_subj_act_1: "~ηχω", _, _;
    perfect_subj_act_2: "~ηχῃς", "~ηχητον", "~ηχητε";
    perfect_subj_act_3: "~ηχῃ", _, "~ηχωσῐ";
    perfect_imp_act_2: "~ηχε", _, _;
    perfect_imp_mid_2: _, _, "~ηχθε";
    perfect_imp_mid_3: _, _, "~ηχθων";
    perfect_imp_pass_2: "~ηξο", _, "~ηχθε";
    perfect_imp_pass_3: "~ηχθω", "~ηχθων", "~ηχθων";
    infinitive_pres: "~ᾰγειν", _, "~ᾰγεσθαι";
    infinitive_fut: "~ᾰξειν", _, _;
    infinitive_aor: "~ᾰγᾰγειν", _, _;
    infinitive_perf: _, "~ηχθαι", _;
    participle_pres: "~ᾰγων", _, "~ᾰγομενος";
    participle_aor: "~ᾰγᾰγων", "~ᾰγᾰγομενος", _;
    participle_perf: "~ηχως", _, "~ηγμενος";
    ),
    // Φᾰρμᾰσσω: 8 words (τᾰρᾰσσω, θωρησσω, κνωσσω, θρῡλῐσσω, παιφᾰσσω, φᾰρμᾰσσω). Like Τᾰσσω, except in 44 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_3 pl, future_ind_mid_2 sg, ….
    conjugation!("φᾰρμᾰσσω", 8, "~σσω", like Τᾰσσω;
    stem aorist_ind: "~ξ- like φᾰρμᾰσσω";
    ind_act_3: _, _, "~σσουσῐ";
    ind_mid_2: "~σσῃ", _, _;
    subj_act_3: _, _, "~σσωσῐ";
    imperfect_act_3: "ἐ~σσε", _, _;
    future_ind_act_3: _, _, "~ξουσῐ";
    future_ind_mid_2: "~ξῃ", _, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: _, _, "~ξωμεθᾰ";
    aorist_opt_act_1: "~ξοιμῐ", _, "~ξοιμεν";
    aorist_opt_act_2: "~ξοις", _, _;
    aorist_opt_act_3: "~ξοι", _, "~ξοιεν";
    aorist_opt_mid_1: "~ξοιμην", _, _;
    aorist_imp_act_2: "~ξε", _, "~ξετε";
    aorist_imp_act_3: _, _, "~ξοντων";
    aorist_imp_mid_2: "~ξου", "~ξεσθον", _;
    aorist_imp_mid_3: _, _, "~ξεσθων";
    perfect_ind_mid_3: _, _, "%ε~χᾰται";
    pluperfect_mid_3: _, _, "ἐ%ε~χᾰτο";
    future_perfect_ind_mid_1: "%ε~ξομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~ξεσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~ξομεθᾰ";
    future_perfect_ind_pass_2: "%ε~ξῃ", "%ε~ξεσθον", "%ε~ξεσθε";
    future_perfect_ind_pass_3: "%ε~ξεται", "%ε~ξεσθον", "%ε~ξονται";
    future_perfect_opt_pass_1: "%ε~ξοιμην", _, "%ε~ξοιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~ξοιο", "%ε~ξοισθον", "%ε~ξοισθε";
    future_perfect_opt_pass_3: "%ε~ξοιτο", "%ε~ξοισθην", "%ε~ξοιντο";
    infinitive_aor: "~ξειν", _, _;
    infinitive_futp: _, _, "%ε~ξεσθαι";
    participle_aor: "~ξων", "~ξομενος", _;
    participle_futp: _, _, "%ε~ξομενος";
    ),
    // Δῐωκω: 8 words (βρῡκω, πεκω, πλεκω, τηκω, κᾰτᾰδῐωκω, δῐωκω). Like Τᾰσσω, except in 86 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("δῐωκω", 8, "~κω", like Τᾰσσω;
    stem aorist_ind: "~ξ- like ἐχω";
    ind_act_1: "~κω", _, "~κομεν";
    ind_act_2: "~κεις", "~κετον", _;
    ind_act_3: "~κει", _, "~κουσῐ";
    ind_mid_1: _, _, "~κομεθᾰ";
    ind_mid_2: "~κῃ", _, _;
    ind_mid_3: _, "~κεσθον", _;
    ind_pass_1: "~κομαι", _, _;
    ind_pass_2: _, "~κεσθον", "~κεσθε";
    ind_pass_3: "~κεται", "~κεσθον", "~κονται";
    subj_act_1: "~κω", _, "~κωμεν";
    subj_act_2: "~κῃς", _, _;
    subj_act_3: "~κῃ", "~κητον", "~κωσῐ";
    subj_mid_1: _, _, "~κωμεθᾰ";
    subj_mid_2: _, "~κησθον", _;
    subj_mid_3: "~κηται", _, "~κωνται";
    subj_pass_1: "~κωμαι", _, _;
    subj_pass_2: _, _, "~κησθε";
    opt_act_1: "~κοιμῐ", _, "~κοιμεν";
    opt_act_2: "~κοις", "~κοιτον", _;
    opt_act_3: "~κοι", "~κοιτην", "~κοιεν";
    opt_mid_1: "~κοιμην", _, "~κοιμεθᾰ";
    opt_mid_2: "~κοιο", _, "~κοισθε";
    opt_mid_3: "~κοιτο", "~κοισθην", "~κοιντο";
    opt_pass_2: _, "~κοισθον", _;
    imp_act_2: "~κε", _, "~κετε";
    imp_act_3: _, _, "~κοντων";
    imp_mid_2: "~κου", "~κεσθον", "~κεσθε";
    imp_mid_3: _, _, "~κεσθων";
    imp_pass_2: _, "~κεσθον", "~κεσθε";
    imp_pass_3: "~κεσθω", "~κεσθων", "~κεσθων";
    imperfect_act_1: "ἐ~κον", _, _;
    imperfect_act_2: "ἐ~κες", "ἐ~κετον", "ἐ~κετε";
    imperfect_act_3: "ἐ~κε", _, "ἐ~κον";
    imperfect_mid_1: "ἐ~κομην", _, "ἐ~κομεθᾰ";
    imperfect_mid_2: "ἐ~κου", _, _;
    imperfect_mid_3: "ἐ~κετο", _, "ἐ~κοντο";
    imperfect_pass_2: _, "ἐ~κεσθον", "ἐ~κεσθε";
    imperfect_pass_3: _, "ἐ~κεσθην", _;
    future_ind_act_3: _, _, "~ξουσῐ";
    future_ind_mid_2: "~ξῃ", _, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_2: "~ξειᾰς", _, _;
    aorist_opt_act_3: "~ξειε", _, "~ξειᾰν";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_mid_3: _, _, "%ε~χᾰται";
    pluperfect_mid_3: _, _, "ἐ%ε~χᾰτο";
    infinitive_pres: "~κειν", _, "~κεσθαι";
    participle_pres: "~κων", _, "~κομενος";
    ),
    // Φῠω: 8 words (φῠω, ἐνδῠω, κῠω, κωλῠω, προμηνῠω, δῐᾰμηνῠω). Like Ληγω, except in 143 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("φῠω", 8, "~ῠω", like Ληγω;
    stem future_pass: "~ῡθησ- like ἐχω";
    stem aorist_ind: "~ῡσ- like ἐχω";
    stem aorist_ind_pass: "~ῡθην- like ἐχω";
    stem aorist_pass: "~ῡθ- like ἐχω";
    ind_act_1: "~ῠω", _, "~ῠομεν";
    ind_act_2: "~ῠεις", "~ῠετον", _;
    ind_act_3: "~ῠει", _, "~ῠουσῐ|~ῠουσῐν";
    ind_mid_1: "~ῠομαι", _, "~ῠομεθᾰ";
    ind_mid_2: "~ῠῃ|~ῠει", _, _;
    ind_mid_3: _, "~ῠεσθον", _;
    ind_pass_1: derived, _, _;
    ind_pass_2: _, "~ῠεσθον", "~ῠεσθε";
    ind_pass_3: "~ῠεται", "~ῠεσθον", "~ῠονται";
    subj_act_1: "~ῠω", _, "~ῠωμεν";
    subj_act_2: "~ῠῃς", _, _;
    subj_act_3: "~ῠῃ", "~ῠητον", "~ῠωσῐ|~ῠωσῐν";
    subj_mid_1: "~ῠωμαι", _, "~ῠωμεθᾰ";
    subj_mid_2: _, "~ῠησθον", _;
    subj_mid_3: "~ῠηται", _, "~ῠωνται";
    subj_pass_1: derived, _, _;
    subj_pass_2: _, _, "~ῠησθε";
    opt_act_1: "~ῠοιμῐ", _, "~ῠοιμεν";
    opt_act_2: "~ῠοις", "~ῠοιτον", _;
    opt_act_3: "~ῠοι", "~ῠοιτην", "~ῠοιεν";
    opt_mid_1: "~ῠοιμην", _, "~ῠοιμεθᾰ";
    opt_mid_2: "~ῠοιο", _, "~ῠοισθε";
    opt_mid_3: "~ῠοιτο", "~ῠοισθην", "~ῠοιντο";
    opt_pass_2: _, "~ῠοισθον", _;
    imp_act_2: "~ῠε", _, "~ῠετε";
    imp_act_3: _, _, "~ῠοντων";
    imp_mid_2: "~ῠου", "~ῠεσθον", "~ῠεσθε";
    imp_mid_3: "~ῠεσθω", _, "~ῠεσθων";
    imp_pass_2: _, "~ῠεσθον", "~ῠεσθε";
    imp_pass_3: derived, "~ῠεσθων", "~ῠεσθων";
    imperfect_act_1: "ἐ~ῠον", _, _;
    imperfect_act_2: "ἐ~ῠες", "ἐ~ῠετον", "ἐ~ῠετε";
    imperfect_act_3: "ἐ~ῠε", _, "ἐ~ῠον";
    imperfect_mid_1: "ἐ~ῠομην", _, "ἐ~ῠομεθᾰ";
    imperfect_mid_2: "ἐ~ῠου", _, _;
    imperfect_mid_3: "ἐ~ῠετο", _, "ἐ~ῠοντο";
    imperfect_pass_2: _, "ἐ~ῠεσθον", "ἐ~ῠεσθε";
    imperfect_pass_3: _, "ἐ~ῠεσθην", _;
    future_ind_act_1: "~ῡσω", _, "~ῡσομεν";
    future_ind_act_2: "~ῡσεις", "~ῡσετον", _;
    future_ind_act_3: _, _, "~ῡσουσῐ";
    future_ind_mid_1: "~ῡσομαι", _, _;
    future_ind_mid_2: "~ῡσῃ", "~ῡσεσθον", _;
    future_ind_mid_3: "~ῡσεται", _, _;
    future_opt_act_1: "~ῡσοιμῐ", _, "~ῡσοιμεν";
    future_opt_act_2: "~ῡσοις", "~ῡσοιτον", _;
    future_opt_act_3: "~ῡσοι", "~ῡσοιτην", _;
    future_opt_mid_1: "~ῡσοιμην", _, "~ῡσοιμεθᾰ";
    future_opt_mid_3: _, "~ῡσοισθην", _;
    aorist_subj_act_1: "~ῡσω", _, _;
    aorist_subj_act_2: "~ῡσῃς", "~ῡσητον", _;
    aorist_subj_mid_1: "~ῡσωμαι", _, _;
    aorist_subj_mid_2: "~ῡσῃ", "~ῡσησθον", _;
    aorist_opt_act_1: "~ῡσαιμῐ", _, "~ῡσαιμεν";
    aorist_opt_act_2: "~ῡσειᾰς|~ῡσαις", _, _;
    aorist_opt_act_3: "~ῡσειε|~ῡσειεν|~ῡσαι", _, "~ῡσειᾰν|~ῡσαιεν";
    aorist_opt_mid_1: "~ῡσαιμην", _, _;
    aorist_imp_act_2: "~ῡσον", _, "~ῡσᾰτε";
    aorist_imp_act_3: _, _, "~ῡσᾰντων";
    aorist_imp_mid_2: "~ῡσαι", "~ῡσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῡσᾰσθων";
    perfect_ind_act_1: "%ε~ῡκᾰ", _, "%ε~ῡκᾰμεν";
    perfect_ind_act_2: _, "%ε~ῡκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ῡκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~ῡμεθᾰ";
    perfect_ind_mid_2: "%ε~ῡσαι", "%ε~ῡσθον", _;
    perfect_ind_mid_3: "%ε~ῡται", _, "%ε~ῡνται";
    perfect_ind_pass_1: "%ε~ῡμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ῡσθε";
    perfect_subj_act_1: "%ε~ῡκω", _, _;
    perfect_subj_act_2: "%ε~ῡκῃς", "%ε~ῡκητον", "%ε~ῡκητε";
    perfect_subj_act_3: "%ε~ῡκῃ", _, "%ε~ῡκωσῐ";
    perfect_imp_act_2: "%ε~ῡκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~ῡσθε";
    perfect_imp_mid_3: _, _, "%ε~ῡσθων";
    perfect_imp_pass_2: "%ε~ῡσο", _, "%ε~ῡσθε";
    perfect_imp_pass_3: "%ε~ῡσθω", "%ε~ῡσθων", "%ε~ῡσθων";
    infinitive_pres: "~ῠειν", _, "~ῠεσθαι";
    infinitive_fut: "~ῡσειν", _, _;
    infinitive_aor: "~ῡσαι", _, _;
    infinitive_perf: _, "%ε~ῡσθαι", _;
    participle_pres: "~ῠων", _, "~ῠομενος";
    participle_aor: "~ῡσᾱς", "~ῡσᾰμενος", _;
    participle_perf: "%ε~ῡκως", _, "%ε~ῡμενος";
    ),
    // Ἀρδω: 8 words (ᾀδω, ἀειδω, σπευδω, ψευδω, δειδω, ἐρειδω). Like Σβεννῡμῐ, except in 81 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀρδω", 8, "~δω", like Σβεννῡμῐ;
    ind_act_1: "~δω", _, "~δομεν";
    ind_act_2: "~δεις", "~δετον", _;
    ind_act_3: "~δει", _, "~δουσῐ";
    ind_mid_1: _, _, "~δομεθᾰ";
    ind_mid_2: "~δῃ", _, _;
    ind_mid_3: _, "~δεσθον", _;
    ind_pass_1: "~δομαι", _, _;
    ind_pass_2: _, "~δεσθον", "~δεσθε";
    ind_pass_3: "~δεται", "~δεσθον", "~δονται";
    subj_act_1: "~δω", _, "~δωμεν";
    subj_act_2: "~δῃς", _, _;
    subj_act_3: "~δῃ", "~δητον", "~δωσῐ";
    subj_mid_1: _, _, "~δωμεθᾰ";
    subj_mid_2: _, "~δησθον", _;
    subj_mid_3: "~δηται", _, "~δωνται";
    subj_pass_1: "~δωμαι", _, _;
    subj_pass_2: _, _, "~δησθε";
    opt_act_1: "~δοιμῐ", _, "~δοιμεν";
    opt_act_2: "~δοις", "~δοιτον", _;
    opt_act_3: "~δοι", "~δοιτην", "~δοιεν";
    opt_mid_1: "~δοιμην", _, "~δοιμεθᾰ";
    opt_mid_2: "~δοιο", _, "~δοισθε";
    opt_mid_3: "~δοιτο", "~δοισθην", "~δοιντο";
    opt_pass_2: _, "~δοισθον", _;
    imp_act_2: "~δε", _, "~δετε";
    imp_act_3: _, _, "~δοντων";
    imp_mid_2: "~δου", "~δεσθον", "~δεσθε";
    imp_mid_3: _, _, "~δεσθων";
    imp_pass_2: _, "~δεσθον", "~δεσθε";
    imp_pass_3: "~δεσθω", "~δεσθων", "~δεσθων";
    imperfect_act_1: "ἐ~δον", _, _;
    imperfect_act_2: "ἐ~δες", "ἐ~δετον", "ἐ~δετε";
    imperfect_act_3: "ἐ~δε", _, "ἐ~δον";
    imperfect_mid_1: "ἐ~δομην", _, "ἐ~δομεθᾰ";
    imperfect_mid_2: "ἐ~δου", _, _;
    imperfect_mid_3: "ἐ~δετο", _, "ἐ~δοντο";
    imperfect_pass_2: _, "ἐ~δεσθον", "ἐ~δεσθε";
    imperfect_pass_3: _, "ἐ~δεσθην", _;
    perfect_ind_mid_1: _, _, "ἐ~σμεθᾰ";
    perfect_ind_mid_3: "ἐ~σται", _, "ἐ~δᾰται";
    perfect_ind_pass_1: "ἐ~σμαι", _, _;
    pluperfect_mid_1: "ἐ~σμην", _, _;
    pluperfect_mid_3: "ἐ~στο", _, "ἐ~δᾰτο";
    pluperfect_pass_1: _, _, "ἐ~σμεθᾰ";
    infinitive_pres: "~δειν", _, "~δεσθαι";
    participle_pres: "~δων", _, "~δομενος";
    participle_perf: _, _, "ἐ~σμενος";
    ),
    // Πλᾰσσω: 8 words (πλᾰσσω, πᾰσσω, βρᾰσσω, ἱμᾰσσω, ἐρεσσω, βλῐσσω). Like Ἑπομαι, except in 151 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πλᾰσσω", 8, "~σω", like Ἑπομαι;
    stem aorist_ind: "~- like ἐχω";
    ind_act_1: "~σω", _, "~σομεν";
    ind_act_2: "~σεις", "~σετον", _;
    ind_act_3: "~σει", _, "~σουσῐ|~σουσῐν";
    ind_mid_1: _, _, "~σομεθᾰ";
    ind_mid_2: "~σῃ|~σει", _, _;
    ind_mid_3: _, "~σεσθον", _;
    ind_pass_1: "~σομαι", _, _;
    ind_pass_2: _, "~σεσθον", "~σεσθε";
    ind_pass_3: "~σεται", "~σεσθον", "~σονται";
    subj_act_1: "~σω", _, "~σωμεν";
    subj_act_2: "~σῃς", _, _;
    subj_act_3: "~σῃ", "~σητον", "~σωσῐ|~σωσῐν";
    subj_mid_1: _, _, "~σωμεθᾰ";
    subj_mid_2: derived, "~σησθον", _;
    subj_mid_3: "~σηται", _, "~σωνται";
    subj_pass_1: "~σωμαι", _, _;
    subj_pass_2: derived, _, "~σησθε";
    opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    opt_act_2: "~σοις", "~σοιτον", _;
    opt_act_3: "~σοι", "~σοιτην", "~σοιεν";
    opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    opt_mid_2: "~σοιο", _, "~σοισθε";
    opt_mid_3: "~σοιτο", "~σοισθην", "~σοιντο";
    opt_pass_2: _, "~σοισθον", _;
    imp_act_2: "~σε", _, "~σετε";
    imp_act_3: _, _, "~σοντων";
    imp_mid_2: "~σου", "~σεσθον", "~σεσθε";
    imp_mid_3: _, _, "~σεσθων";
    imp_pass_2: _, "~σεσθον", "~σεσθε";
    imp_pass_3: "~σεσθω", "~σεσθων", "~σεσθων";
    imperfect_act_1: "ἐ~σον", _, _;
    imperfect_act_2: "ἐ~σες", "ἐ~σετον", "ἐ~σετε";
    imperfect_act_3: "ἐ~σε", _, "ἐ~σον";
    imperfect_mid_1: "ἐ~σομην", _, "ἐ~σομεθᾰ";
    imperfect_mid_2: "ἐ~σου", _, _;
    imperfect_mid_3: "ἐ~σετο", _, "ἐ~σοντο";
    imperfect_pass_2: _, "ἐ~σεσθον", "ἐ~σεσθε";
    imperfect_pass_3: _, "ἐ~σεσθην", _;
    future_ind_act_1: "~ω", _, "~ομεν";
    future_ind_act_2: "~εις", "~ετον", _;
    future_ind_act_3: _, _, "~ουσῐ";
    future_ind_mid_1: "~ομαι", _, _;
    future_ind_mid_2: "~ῃ", "~εσθον", _;
    future_ind_mid_3: "~εται", _, _;
    future_opt_act_1: "~οιμῐ", _, "~οιμεν";
    future_opt_act_2: "~οις", "~οιτον", _;
    future_opt_act_3: "~οι", "~οιτην", _;
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_3: _, "~οισθην", _;
    aorist_subj_act_1: "~ω", _, _;
    aorist_subj_act_2: "~ῃς", "~ητον", _;
    aorist_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    aorist_subj_mid_1: "~ωμαι", _, _;
    aorist_subj_mid_2: "~ῃ", "~ησθον", _;
    aorist_opt_act_1: "~αιμῐ", _, "~αιμεν";
    aorist_opt_act_2: "~ειᾰς|~αις", _, _;
    aorist_opt_act_3: "~ειε|~ειεν|~αι", _, "~ειᾰν|~αιεν";
    aorist_opt_mid_1: "~αιμην", _, _;
    aorist_imp_act_2: "~ον", _, "~ᾰτε";
    aorist_imp_act_3: _, _, "~ᾰντων";
    aorist_imp_mid_2: "~αι", "~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰσθων";
    perfect_ind_mid_2: "%ε~αι", "%ε~θον", _;
    perfect_ind_mid_3: _, _, "%ε~ᾰται";
    perfect_ind_pass_2: _, _, "%ε~θε";
    perfect_imp_mid_2: _, _, "%ε~θε";
    perfect_imp_mid_3: _, _, "%ε~θων";
    perfect_imp_pass_2: "%ε~ο", _, "%ε~θε";
    perfect_imp_pass_3: "%ε~θω", "%ε~θων", "%ε~θων";
    pluperfect_mid_2: "ἐ%ε~ο", "ἐ%ε~θον", _;
    pluperfect_mid_3: _, _, "ἐ%ε~ᾰτο";
    pluperfect_pass_2: _, _, "ἐ%ε~θε";
    pluperfect_pass_3: _, "ἐ%ε~θην", _;
    future_perfect_ind_mid_1: "%ε~ομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~εσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~ομεθᾰ";
    future_perfect_ind_pass_2: "%ε~ῃ", "%ε~εσθον", "%ε~εσθε";
    future_perfect_ind_pass_3: "%ε~εται", "%ε~εσθον", "%ε~ονται";
    future_perfect_opt_pass_1: "%ε~οιμην", _, "%ε~οιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~οιο", "%ε~οισθον", "%ε~οισθε";
    future_perfect_opt_pass_3: "%ε~οιτο", "%ε~οισθην", "%ε~οιντο";
    infinitive_pres: "~σειν", _, "~σεσθαι";
    infinitive_fut: "~ειν", _, _;
    infinitive_aor: "~αι", _, _;
    infinitive_perf: _, "%ε~θαι", _;
    infinitive_futp: _, _, "%ε~εσθαι";
    participle_pres: "~σων", _, "~σομενος";
    participle_aor: "~ᾱς", "~ᾰμενος", _;
    participle_futp: _, _, "%ε~ομενος";
    ),
    // Πᾰρεχω: 8 words (ἐπεχω, προσεχω, πᾰρεχω, ἀπεχω, περιεχω, ἀνεχω). Like Κειμαι, except in 145 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πᾰρεχω", 8, "~εχω", like Κειμαι;
    stem aorist_ind: "~εσχ- like πᾰρεχω";
    stem aorist_ind_pass: "~εσχεθην- like εὐλογεω";
    stem aorist_pass: "~σχεθ- like ἐχω";
    ind_act_1: "~εχω", _, "~εχομεν";
    ind_act_2: "~εχεις", "~εχετον", _;
    ind_act_3: "~εχει", _, "~εχουσῐ|~εχουσῐν";
    ind_mid_1: _, _, "~εχομεθᾰ";
    ind_mid_2: "~εχῃ|~εχει", _, _;
    ind_mid_3: _, "~εχεσθον", _;
    ind_pass_1: "~εχομαι", _, _;
    ind_pass_2: _, "~εχεσθον", "~εχεσθε";
    ind_pass_3: "~εχεται", "~εχεσθον", "~εχονται";
    subj_act_1: "~εχω", _, "~εχωμεν";
    subj_act_2: "~εχῃς", _, _;
    subj_act_3: "~εχῃ", "~εχητον", "~εχωσῐ|~εχωσῐν";
    subj_mid_1: _, _, "~εχωμεθᾰ";
    subj_mid_2: _, "~εχησθον", _;
    subj_mid_3: "~εχηται", _, "~εχωνται";
    subj_pass_1: "~εχωμαι", _, _;
    subj_pass_2: _, _, "~εχησθε";
    opt_act_1: "~εχοιμῐ", _, "~εχοιμεν";
    opt_act_2: "~εχοις", "~εχοιτον", _;
    opt_act_3: "~εχοι", "~εχοιτην", "~εχοιεν";
    opt_mid_1: "~εχοιμην", _, "~εχοιμεθᾰ";
    opt_mid_2: "~εχοιο", _, "~εχοισθε";
    opt_mid_3: "~εχοιτο", "~εχοισθην", "~εχοιντο";
    opt_pass_2: _, "~εχοισθον", _;
    imp_act_2: "~εχε", _, "~εχετε";
    imp_act_3: _, _, "~εχοντων";
    imp_mid_2: "~εχου", "~εχεσθον", "~εχεσθε";
    imp_mid_3: _, _, "~εχεσθων";
    imp_pass_2: _, "~εχεσθον", "~εχεσθε";
    imp_pass_3: "~εχεσθω", "~εχεσθων", "~εχεσθων";
    imperfect_act_1: "~ειχον", _, _;
    imperfect_act_2: "~ειχες", "~ειχετον", "~ειχετε";
    imperfect_act_3: "~ειχε", _, "~ειχον";
    imperfect_mid_1: "~ειχομην", _, "~ειχομεθᾰ";
    imperfect_mid_2: "~ειχου", _, _;
    imperfect_mid_3: "~ειχετο", _, "~ειχοντο";
    imperfect_pass_2: _, "~ειχεσθον", "~ειχεσθε";
    imperfect_pass_3: _, "~ειχεσθην", _;
    future_ind_act_1: "~εξω", _, "~εξομεν";
    future_ind_act_2: "~εξεις", "~εξετον", _;
    future_ind_act_3: _, _, "~εξουσῐ";
    future_ind_mid_1: "~εξομαι", _, _;
    future_ind_mid_2: "~εξῃ", "~εξεσθον", _;
    future_ind_mid_3: "~εξεται", _, _;
    future_opt_act_1: "~εξοιμῐ", _, "~εξοιμεν";
    future_opt_act_2: "~εξοις", "~εξοιτον", _;
    future_opt_act_3: "~εξοι", "~εξοιτην", _;
    future_opt_mid_1: "~εξοιμην", _, "~εξοιμεθᾰ";
    future_opt_mid_3: _, "~εξοισθην", _;
    aorist_subj_act_1: "~σχω", _, _;
    aorist_subj_act_2: "~σχῃς", "~σχητον", _;
    aorist_subj_act_3: _, _, "~σχωσῐ|~σχωσῐν";
    aorist_subj_mid_1: "~σχωμαι", _, _;
    aorist_subj_mid_2: "~σχῃ", "~σχησθον", _;
    aorist_opt_act_1: "~σχοιμῐ", _, "~σχοιμεν";
    aorist_opt_act_2: "~σχοις", _, _;
    aorist_opt_act_3: "~σχοι", _, "~σχοιεν";
    aorist_opt_mid_1: "~σχοιμην", _, _;
    aorist_imp_act_2: "~σχε", _, "~σχετε";
    aorist_imp_act_3: _, _, "~σχοντων";
    aorist_imp_mid_2: "~σχου", "~σχεσθον", _;
    aorist_imp_mid_3: _, _, "~σχεσθων";
    perfect_ind_act_1: "~εσχηκᾰ", _, "~εσχηκᾰμεν";
    perfect_ind_act_2: _, "~εσχηκᾰτον", _;
    perfect_ind_act_3: "~εσχηκε|~εσχηκεν", _, "~εσχηκᾱσῐ|~εσχηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~εσχημεθᾰ";
    perfect_ind_mid_2: "~εσχησαι", "~εσχησθον", _;
    perfect_ind_mid_3: "~εσχηται", _, "~εσχηνται";
    perfect_ind_pass_1: "~εσχημαι", _, _;
    perfect_ind_pass_2: _, _, "~εσχησθε";
    perfect_subj_act_1: "~εσχηκω", _, _;
    perfect_subj_act_2: "~εσχηκῃς", "~εσχηκητον", "~εσχηκητε";
    perfect_subj_act_3: "~εσχηκῃ", _, "~εσχηκωσῐ|~εσχηκωσῐν";
    perfect_opt_act_1: "~εσχηκοιμῐ|~εσχηκοιην", _, _;
    perfect_opt_act_2: "~εσχηκοις|~εσχηκοιης", _, _;
    perfect_opt_act_3: "~εσχηκοι|~εσχηκοιη", _, _;
    perfect_imp_act_2: "~εσχηκε", _, _;
    perfect_imp_mid_2: _, _, "~εσχησθε";
    perfect_imp_mid_3: _, _, "~εσχησθων";
    perfect_imp_pass_2: "~εσχησο", _, "~εσχησθε";
    perfect_imp_pass_3: "~εσχησθω", "~εσχησθων", "~εσχησθων";
    infinitive_pres: "~εχειν", _, "~εχεσθαι";
    infinitive_fut: "~εξειν", _, _;
    infinitive_aor: "~σχειν", _, _;
    infinitive_perf: _, "~εσχησθαι", _;
    participle_pres: "~εχων", _, "~εχομενος";
    participle_aor: "~σχων", "~σχομενος", _;
    participle_perf: "~εσχηκως", _, "~εσχημενος";
    ),
    // Κρῑνομαι: 8 words (γεινομαι, ἑζομαι, ἐρομαι, κᾰθεζομαι, σῑνομαι, ἐπερομαι). Like Οἰομαι, except in 43 cells: aorist_ind_pass_1 sg, aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_mid_1 sg, aorist_subj_mid_2 sg, ….
    conjugation!("κρῑνομαι", 8, "~ομαι", like Οἰομαι;
    stem aorist_ind: "~- like ἐχω";
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ω", _, _;
    aorist_subj_act_2: "~ῃς", "~ητον", _;
    aorist_subj_mid_1: "~ωμαι", _, _;
    aorist_subj_mid_2: "~ῃ", "~ησθον", _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_1: "~αιμῐ", _, "~αιμεν";
    aorist_opt_act_2: "~ειᾰς", _, _;
    aorist_opt_act_3: "~ειε", _, "~ειᾰν";
    aorist_opt_mid_1: "~αιμην", _, _;
    aorist_imp_act_2: "~ον", _, "~ᾰτε";
    aorist_imp_act_3: _, _, "~ᾰντων";
    aorist_imp_mid_2: "~αι", "~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰσθων";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    infinitive_aor: "~αι", _, _;
    infinitive_perf: _, derived, _;
    participle_aor: "~ᾱς", "~ᾰμενος", _;
    participle_perf: _, _, derived;
    ),
    // Πῠνθανομαι: 8 words (μαχομαι, πῠνθανομαι, σῠνεργαζομαι, κᾰτεργαζομαι, ἀπεχθανομαι, ἐξεργαζομαι). Like Ἐπᾰϊω, except in 140 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πῠνθανομαι", 8, "~α~ομαι", like Ἐπᾰϊω;
    stem aorist_ind: "~ᾰ~εσ- like ἑπομαι";
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
    opt_act_2: derived, derived, _;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imp_mid_3: "~ᾰ~εσθω", _, _;
    imp_pass_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    imperfect_mid_1: "ἐ~ᾰ~ομην", _, "ἐ~ᾰ~ομεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~ου", "ἐ~ᾰ~εσθον", _;
    imperfect_mid_3: "ἐ~ᾰ~ετο", _, "ἐ~ᾰ~οντο";
    imperfect_pass_2: _, derived, "ἐ~ᾰ~εσθε";
    imperfect_pass_3: _, "ἐ~ᾰ~εσθην", _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, _;
    future_ind_act_3: _, _, derived;
    future_ind_mid_1: "~ᾰ~εσομαι", _, _;
    future_ind_mid_2: "~ᾰ~εσῃ", "~ᾰ~εσεσθον", _;
    future_ind_mid_3: "~ᾰ~εσεται", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, _;
    future_opt_act_3: derived, derived, _;
    future_opt_mid_1: "~ᾰ~εσοιμην", _, "~ᾰ~εσοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~εσοισθην", _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~ᾰ~εσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~εσῃ", "~ᾰ~εσησθον", _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_opt_mid_1: "~ᾰ~εσαιμην", _, _;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_mid_2: "~ᾰ~εσαι", "~ᾰ~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~εσᾰσθων";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_1: _, _, "~εμᾰ~ημεθᾰ";
    perfect_ind_mid_2: "~εμᾰ~ησαι", "~εμᾰ~ησθον", _;
    perfect_ind_mid_3: "~εμᾰ~ηται", _, "~εμᾰ~ηνται";
    perfect_ind_pass_1: "~εμᾰ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~εμᾰ~ησθε";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_imp_act_2: derived, _, _;
    perfect_imp_mid_2: _, _, "~εμᾰ~ησθε";
    perfect_imp_mid_3: _, _, "~εμᾰ~ησθων";
    perfect_imp_pass_2: "~εμᾰ~ησο", _, "~εμᾰ~ησθε";
    perfect_imp_pass_3: "~εμᾰ~ησθω", "~εμᾰ~ησθων", "~εμᾰ~ησθων";
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
    infinitive_perf: _, "~εμᾰ~ησθαι", _;
    participle_pres: derived, _, _;
    participle_aor: derived, "~ᾰ~εσᾰμενος", _;
    participle_perf: derived, _, "~εμᾰ~ημενος";
    ),
    // Εἰμῐ: 8 words (εἰμῐ, δῐδωμῐ, ἀντᾰποδῐδωμῐ, πᾰρᾰδιδωμῐ, ἠμῐ, ὀλλῡμῐ). Like Κειμαι, except in 72 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("εἰμῐ", 8, "~μῐ", like Κειμαι;
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
    opt_act_2: derived, derived, _;
    opt_act_3: derived, derived, derived;
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, _, derived;
    opt_mid_3: derived, derived, derived;
    opt_pass_2: _, derived, _;
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
    stem future_pass: "~σθησ- like χρημᾰτῐζω";
    stem aorist_ind_pass: "~σθην- like ἐχω";
    stem aorist_pass: "~σθ- like ποιεω";
    ),
    // Σῳζω: 7 words (σοφῐζω, σῳζω, κεᾰζω, στρουθῐζω, τῠκῐζω, μερῐζω). Like Χρημᾰτῐζω, except in 2 cells: perfect_ind_mid_3 pl, pluperfect_mid_3 pl.
    conjugation!("σῳζω", 7, "~ζω", like Χρημᾰτῐζω;
    perfect_ind_mid_3: _, _, "%ε~σᾰται";
    pluperfect_mid_3: _, _, "ἐ%ε~σᾰτο";
    ),
    // Σφᾰλλω: 7 words (θᾰλλω, πᾰλλω, βδᾰλλω, σφᾰλλω, ψᾰλλω, ἰᾰλλω). Like Ἐγειρω, except in 113 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("σφᾰλλω", 7, "~ᾰ~λω", like Ἐγειρω;
    stem future_pass: "~ᾰ~ησησ- like ἐχω";
    stem aorist_ind: "~η~- like ποιεω";
    ind_act_1: "~ᾰ~λω", _, "~ᾰ~λομεν";
    ind_act_2: "~ᾰ~λεις", "~ᾰ~λετον", _;
    ind_act_3: "~ᾰ~λει", _, "~ᾰ~λουσῐ|~ᾰ~λουσῐν";
    ind_mid_1: _, _, "~ᾰ~λομεθᾰ";
    ind_mid_2: "~ᾰ~λῃ|~ᾰ~λει", _, _;
    ind_mid_3: _, "~ᾰ~λεσθον", _;
    ind_pass_1: "~ᾰ~λομαι", _, _;
    ind_pass_2: _, "~ᾰ~λεσθον", "~ᾰ~λεσθε";
    ind_pass_3: "~ᾰ~λεται", "~ᾰ~λεσθον", "~ᾰ~λονται";
    subj_act_1: "~ᾰ~λω", _, "~ᾰ~λωμεν";
    subj_act_2: "~ᾰ~λῃς", _, _;
    subj_act_3: "~ᾰ~λῃ", "~ᾰ~λητον", "~ᾰ~λωσῐ|~ᾰ~λωσῐν";
    subj_mid_1: _, _, "~ᾰ~λωμεθᾰ";
    subj_mid_2: _, "~ᾰ~λησθον", _;
    subj_mid_3: "~ᾰ~ληται", _, "~ᾰ~λωνται";
    subj_pass_1: "~ᾰ~λωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~λησθε";
    opt_act_1: "~ᾰ~λοιμῐ", _, "~ᾰ~λοιμεν";
    opt_act_2: "~ᾰ~λοις", "~ᾰ~λοιτον", _;
    opt_act_3: "~ᾰ~λοι", "~ᾰ~λοιτην", "~ᾰ~λοιεν";
    opt_mid_1: "~ᾰ~λοιμην", _, "~ᾰ~λοιμεθᾰ";
    opt_mid_2: "~ᾰ~λοιο", _, "~ᾰ~λοισθε";
    opt_mid_3: "~ᾰ~λοιτο", "~ᾰ~λοισθην", "~ᾰ~λοιντο";
    opt_pass_2: _, "~ᾰ~λοισθον", _;
    imp_act_2: "~ᾰ~λε", _, "~ᾰ~λετε";
    imp_act_3: _, _, "~ᾰ~λοντων";
    imp_mid_2: "~ᾰ~λου", "~ᾰ~λεσθον", "~ᾰ~λεσθε";
    imp_mid_3: _, _, "~ᾰ~λεσθων";
    imp_pass_2: _, "~ᾰ~λεσθον", "~ᾰ~λεσθε";
    imp_pass_3: "~ᾰ~λεσθω", "~ᾰ~λεσθων", "~ᾰ~λεσθων";
    imperfect_act_1: "ἐ~ᾰ~λον", _, _;
    imperfect_act_2: "ἐ~ᾰ~λες", "ἐ~ᾰ~λετον", "ἐ~ᾰ~λετε";
    imperfect_act_3: "ἐ~ᾰ~λε|ἐ~ᾰ~λεν", _, "ἐ~ᾰ~λον";
    imperfect_mid_1: "ἐ~ᾰ~λομην", _, "ἐ~ᾰ~λομεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~λου", _, _;
    imperfect_mid_3: "ἐ~ᾰ~λετο", _, "ἐ~ᾰ~λοντο";
    imperfect_pass_2: _, "ἐ~ᾰ~λεσθον", "ἐ~ᾰ~λεσθε";
    imperfect_pass_3: _, "ἐ~ᾰ~λεσθην", _;
    future_ind_act_1: "~ᾰ~ω", _, "~ᾰ~ουμεν";
    future_ind_act_2: "~ᾰ~εις", "~ᾰ~ειτον", _;
    future_ind_act_3: _, _, "~ᾰ~ουσῐ";
    future_ind_mid_1: "~ᾰ~ουμαι", _, _;
    future_ind_mid_2: "~ᾰ~ει", "~ᾰ~εισθον", _;
    future_ind_mid_3: "~ᾰ~ειται", _, _;
    future_opt_act_1: "~ᾰ~οιην", _, "~ᾰ~οιμεν";
    future_opt_act_2: "~ᾰ~οιης", "~ᾰ~οιτον", _;
    future_opt_act_3: "~ᾰ~οιη", "~ᾰ~οιτην", _;
    future_opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~οισθην", _;
    aorist_subj_act_1: "~η~ω", _, _;
    aorist_subj_act_2: "~η~ῃς", "~η~ητον", _;
    aorist_subj_act_3: _, _, "~η~ωσῐ|~η~ωσῐν";
    aorist_subj_mid_1: "~η~ωμαι", _, _;
    aorist_subj_mid_2: "~η~ῃ", "~η~ησθον", _;
    aorist_opt_act_1: "~η~αιμῐ", _, "~η~αιμεν";
    aorist_opt_act_2: "~η~ειᾰς|~η~αις", _, _;
    aorist_opt_act_3: "~η~ειε|~η~ειεν|~η~αι", _, "~η~ειᾰν|~η~αιεν";
    aorist_opt_mid_1: "~η~αιμην", _, _;
    aorist_imp_act_2: "~η~ον", _, "~η~ᾰτε";
    aorist_imp_act_3: _, _, "~η~ᾰντων";
    aorist_imp_mid_2: "~η~αι", "~η~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~η~ᾰσθων";
    infinitive_pres: "~ᾰ~λειν", _, "~ᾰ~λεσθαι";
    infinitive_fut: "~ᾰ~ειν", _, _;
    infinitive_aor: "~η~αι", _, _;
    participle_pres: "~ᾰ~λων", _, "~ᾰ~λομενος";
    participle_aor: "~η~ᾱς", "~η~ᾰμενος", _;
    ),
    // Καθιζω: 7 words (εὐαγγελιζω, βᾰδιζω, καθιζω, συνᾱλιζω, βαλλιζω, μακαριζω). Like Πεμπω, except in 151 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("καθιζω", 7, "~~ζω", like Πεμπω;
    stem future_pass: "~~σθησ- like ἐχω";
    stem aorist_ind: "~~σ- like ποιεω";
    stem aorist_ind_pass: "~~σθην- like ἐχω";
    stem aorist_pass: "~~σθ- like ἐχω";
    stem pluperfect_act: "~~κειν- like ἐχω";
    ind_act_1: "~~ζω", _, "~~ζομεν";
    ind_act_2: "~~ζεις", "~~ζετον", _;
    ind_act_3: "~~ζει", _, "~~ζουσῐ|~~ζουσῐν";
    ind_mid_1: _, _, "~~ζομεθᾰ";
    ind_mid_2: "~~ζῃ|~~ζει", _, _;
    ind_mid_3: _, "~~ζεσθον", _;
    ind_pass_1: "~~ζομαι", _, _;
    ind_pass_2: _, "~~ζεσθον", "~~ζεσθε";
    ind_pass_3: "~~ζεται", "~~ζεσθον", "~~ζονται";
    subj_act_1: "~~ζω", _, "~~ζωμεν";
    subj_act_2: "~~ζῃς", _, _;
    subj_act_3: "~~ζῃ", "~~ζητον", "~~ζωσῐ|~~ζωσῐν";
    subj_mid_1: _, _, "~~ζωμεθᾰ";
    subj_mid_2: _, "~~ζησθον", _;
    subj_mid_3: "~~ζηται", _, "~~ζωνται";
    subj_pass_1: "~~ζωμαι", _, _;
    subj_pass_2: _, _, "~~ζησθε";
    opt_act_1: "~~ζοιμῐ", _, "~~ζοιμεν";
    opt_act_2: "~~ζοις", "~~ζοιτον", _;
    opt_act_3: "~~ζοι", "~~ζοιτην", "~~ζοιεν";
    opt_mid_1: "~~ζοιμην", _, "~~ζοιμεθᾰ";
    opt_mid_2: "~~ζοιο", _, "~~ζοισθε";
    opt_mid_3: "~~ζοιτο", "~~ζοισθην", "~~ζοιντο";
    opt_pass_2: _, "~~ζοισθον", _;
    imp_act_2: "~~ζε", _, "~~ζετε";
    imp_act_3: _, _, "~~ζοντων";
    imp_mid_2: "~~ζου", "~~ζεσθον", "~~ζεσθε";
    imp_mid_3: _, _, "~~ζεσθων";
    imp_pass_2: _, "~~ζεσθον", "~~ζεσθε";
    imp_pass_3: "~~ζεσθω", "~~ζεσθων", "~~ζεσθων";
    imperfect_act_1: "ἐ~~ζον", _, _;
    imperfect_act_2: "ἐ~~ζες", "ἐ~~ζετον", "ἐ~~ζετε";
    imperfect_act_3: "ἐ~~ζε|ἐ~~ζεν", _, "ἐ~~ζον";
    imperfect_mid_1: "ἐ~~ζομην", _, "ἐ~~ζομεθᾰ";
    imperfect_mid_2: "ἐ~~ζου", _, _;
    imperfect_mid_3: "ἐ~~ζετο", _, "ἐ~~ζοντο";
    imperfect_pass_2: _, "ἐ~~ζεσθον", "ἐ~~ζεσθε";
    imperfect_pass_3: _, "ἐ~~ζεσθην", _;
    future_ind_act_1: "~~ω", _, "~~ουμεν";
    future_ind_act_2: "~~εις", "~~ειτον", _;
    future_ind_act_3: _, _, "~~ουσῐ|~~ουσῐν";
    future_ind_mid_1: "~~ουμαι", _, _;
    future_ind_mid_2: "~~ει|~~ῃ", "~~εισθον", _;
    future_ind_mid_3: "~~ειται", _, _;
    future_opt_act_1: "~~οιην|~~οιμῐ", _, "~~οιμεν|~~οιημεν";
    future_opt_act_2: "~~οιης|~~οις", "~~οιτον|~~οιητον", "~~οιτε|~~οιητε";
    future_opt_act_3: "~~οιη|~~οι", "~~οιτην|~~οιητην", "~~οιεν|~~οιησᾰν";
    future_opt_mid_1: "~~οιμην", _, "~~οιμεθᾰ";
    future_opt_mid_3: _, "~~οισθην", _;
    aorist_subj_act_1: "~~σω", _, _;
    aorist_subj_act_2: "~~σῃς", "~~σητον", _;
    aorist_subj_act_3: _, _, "~~σωσῐ|~~σωσῐν";
    aorist_subj_mid_1: "~~σωμαι", _, _;
    aorist_subj_mid_2: "~~σῃ", "~~σησθον", _;
    aorist_opt_act_1: "~~σαιμῐ", _, "~~σαιμεν";
    aorist_opt_act_2: "~~σειᾰς|~~σαις", _, _;
    aorist_opt_act_3: "~~σειε|~~σειεν|~~σαι", _, "~~σειᾰν|~~σαιεν";
    aorist_opt_mid_1: "~~σαιμην", _, _;
    aorist_imp_act_2: "~~σον", _, "~~σᾰτε";
    aorist_imp_act_3: _, _, "~~σᾰντων";
    aorist_imp_mid_2: "~~σαι", "~~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~σᾰσθων";
    perfect_ind_act_1: "%ε~~κᾰ", _, "%ε~~κᾰμεν";
    perfect_ind_act_2: _, "%ε~~κᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~~κᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~~σμεθᾰ";
    perfect_ind_mid_2: "%ε~~σαι", "%ε~~σθον", _;
    perfect_ind_mid_3: "%ε~~σται", _, "%ε~~σᾰται";
    perfect_ind_pass_1: "%ε~~σμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~~σθε";
    perfect_subj_act_1: "%ε~~κω", _, _;
    perfect_subj_act_2: "%ε~~κῃς", "%ε~~κητον", "%ε~~κητε";
    perfect_subj_act_3: "%ε~~κῃ", _, "%ε~~κωσῐ";
    perfect_imp_act_2: "%ε~~κε", _, _;
    perfect_imp_mid_2: _, _, "%ε~~σθε";
    perfect_imp_mid_3: _, _, "%ε~~σθων";
    perfect_imp_pass_2: "%ε~~σο", _, "%ε~~σθε";
    perfect_imp_pass_3: "%ε~~σθω", "%ε~~σθων", "%ε~~σθων";
    pluperfect_mid_1: "ἐ%ε~~σμην", _, _;
    pluperfect_mid_2: "ἐ%ε~~σο", "ἐ%ε~~σθον", _;
    pluperfect_mid_3: "ἐ%ε~~στο", _, "ἐ%ε~~σᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~~σθε";
    pluperfect_pass_3: _, "ἐ%ε~~σθην", _;
    infinitive_pres: "~~ζειν", _, "~~ζεσθαι";
    infinitive_fut: "~~ειν", _, _;
    infinitive_aor: "~ῑσα~", _, _;
    infinitive_perf: _, "%ε~ῑσθα~", _;
    participle_pres: "~~ζων", _, "~~ζομενος";
    participle_aor: "~~σᾱς", "~~σᾰμενος", _;
    participle_perf: "%ε~~κως", _, "%ε~~σμενος";
    ),
    // Ἀπειπον: 7 words (προσειπον, ἐξειπον, προειπον, ἀπειπον, ὑπειπον, ἐπειπον). Like Εἰμῐ, except in 72 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("ἀπειπον", 7, "~ειπον", like Εἰμῐ;
    stem future_pass: "~ορρηθησ- like ἐχω";
    stem aorist_ind: "~ειπ- like ἀνᾰγω";
    stem aorist_ind_pass: "~ερρηθην- like εὐλογεω";
    stem aorist_pass: "~ορρηθ- like ἐχω";
    ind_act_1: "~ειπον", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_1: "~ερω", _, "~ερουμεν";
    future_ind_act_2: "~ερεις", "~ερειτον", _;
    future_ind_act_3: _, _, "~ερουσῐ";
    future_ind_mid_1: "~ερουμαι", _, _;
    future_ind_mid_2: "~ερει", "~ερεισθον", _;
    future_ind_mid_3: "~ερειται", _, _;
    future_opt_act_1: "~εροιην", _, "~εροιμεν";
    future_opt_act_2: "~εροιης", "~εροιτον", _;
    future_opt_act_3: "~εροιη", "~εροιτην", _;
    future_opt_mid_1: "~εροιμην", _, "~εροιμεθᾰ";
    future_opt_mid_3: _, "~εροισθην", _;
    aorist_subj_act_1: "~ειπω", _, _;
    aorist_subj_act_2: "~ειπῃς", "~ειπητον", _;
    aorist_subj_mid_1: "~ειπωμαι", _, _;
    aorist_subj_mid_2: "~ειπῃ", "~ειπησθον", _;
    aorist_opt_act_1: "~ειποιμῐ", _, "~ειποιμεν";
    aorist_opt_act_2: "~ειποις", _, _;
    aorist_opt_act_3: "~ειποι", _, "~ειποιεν";
    aorist_opt_mid_1: "~ειποιμην", _, _;
    aorist_imp_act_2: "~ειπε", _, "~ειπετε";
    aorist_imp_act_3: _, _, "~ειποντων";
    aorist_imp_mid_2: "~ειπου", "~ειπεσθον", _;
    aorist_imp_mid_3: _, _, "~ειπεσθων";
    perfect_ind_act_1: "~ειρηκᾰ", _, "~ειρηκᾰμεν";
    perfect_ind_act_2: _, "~ειρηκᾰτον", _;
    perfect_ind_act_3: _, _, "~ειρηκᾱσῐ";
    perfect_ind_mid_1: _, _, "~ειρημεθᾰ";
    perfect_ind_mid_2: "~ειρησαι", "~ειρησθον", _;
    perfect_ind_mid_3: "~ειρηται", _, "~ειρηνται";
    perfect_ind_pass_1: "~ειρημαι", _, _;
    perfect_ind_pass_2: _, _, "~ειρησθε";
    perfect_subj_act_1: "~ειρηκω", _, _;
    perfect_subj_act_2: "~ειρηκῃς", "~ειρηκητον", "~ειρηκητε";
    perfect_subj_act_3: "~ειρηκῃ", _, "~ειρηκωσῐ";
    perfect_imp_act_2: "~ειρηκε", _, _;
    perfect_imp_mid_2: _, _, "~ειρησθε";
    perfect_imp_mid_3: _, _, "~ειρησθων";
    perfect_imp_pass_2: "~ειρησο", _, "~ειρησθε";
    perfect_imp_pass_3: "~ειρησθω", "~ειρησθων", "~ειρησθων";
    infinitive_fut: "~ερειν", _, _;
    infinitive_aor: "~ειπειν", _, _;
    infinitive_perf: _, "~ειρησθαι", _;
    participle_aor: "~ειπων", "~ειπομενος", _;
    participle_perf: "~ειρηκως", _, "~ειρημενος";
    ),
    // Χραω: 7 words (χραω, νωμαω, σκῐαω, ἀπαγχοναω, λοχαω, πασχητῐαω). Like Νῑκαω, except in 8 cells: pluperfect_mid_1 sg, pluperfect_mid_2 sg, pluperfect_mid_2 du, pluperfect_mid_3 sg, pluperfect_mid_3 pl, pluperfect_pass_1 pl, ….
    conjugation!("χραω", 7, "~αω", like Νῑκαω;
    stem future_pass: "~ησθησ- like ἐχω";
    stem aorist_ind_pass: "~ησθην- like ἐχω";
    stem aorist_pass: "~ησθ- like ἐχω";
    stem pluperfect_act: "~ηκειν- like ἐχω";
    pluperfect_mid_1: "ἐ%ε~ημην", _, _;
    pluperfect_mid_2: "ἐ%ε~ησο", "ἐ%ε~ησθον", _;
    pluperfect_mid_3: "ἐ%ε~ητο", _, "ἐ%ε~ηντο";
    pluperfect_pass_1: _, _, "ἐ%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~ησθην", _;
    ),
    // Λῡω: 7 words (λῡω, βλῡω, ἐξαναλῡω, ἐπαναλῡω, ἐπαπολῡω, περῐλῡω). Like Φῠω, except in 134 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("λῡω", 7, "~ῡω", like Φῠω;
    stem future_pass: "~ῠθησ- like ἐχω";
    stem aorist_ind_pass: "~ῠθην- like ἐχω";
    stem aorist_pass: "~ῠθ- like ἐχω";
    ind_act_1: "~ῡω", _, "~ῡομεν";
    ind_act_2: "~ῡεις", "~ῡετον", _;
    ind_act_3: "~ῡει", _, "~ῡουσῐ";
    ind_mid_1: derived, _, "~ῡομεθᾰ";
    ind_mid_2: "~ῡῃ", _, _;
    ind_mid_3: _, "~ῡεσθον", _;
    ind_pass_1: "~ῡομαι", _, _;
    ind_pass_2: _, "~ῡεσθον", "~ῡεσθε";
    ind_pass_3: "~ῡεται", "~ῡεσθον", "~ῡονται";
    subj_act_1: "~ῡω", _, "~ῡωμεν";
    subj_act_2: "~ῡῃς", _, _;
    subj_act_3: "~ῡῃ", "~ῡητον", "~ῡωσῐ";
    subj_mid_1: derived, _, "~ῡωμεθᾰ";
    subj_mid_2: _, "~ῡησθον", _;
    subj_mid_3: "~ῡηται", _, "~ῡωνται";
    subj_pass_1: "~ῡωμαι", _, _;
    subj_pass_2: _, _, "~ῡησθε";
    opt_act_1: "~ῡοιμῐ", _, "~ῡοιμεν";
    opt_act_2: "~ῡοις", "~ῡοιτον", _;
    opt_act_3: "~ῡοι", "~ῡοιτην", "~ῡοιεν";
    opt_mid_1: "~ῡοιμην", _, "~ῡοιμεθᾰ";
    opt_mid_2: "~ῡοιο", _, "~ῡοισθε";
    opt_mid_3: "~ῡοιτο", "~ῡοισθην", "~ῡοιντο";
    opt_pass_2: _, "~ῡοισθον", _;
    imp_act_2: "~ῡε", _, "~ῡετε";
    imp_act_3: _, _, "~ῡοντων";
    imp_mid_2: "~ῡου", "~ῡεσθον", "~ῡεσθε";
    imp_mid_3: derived, _, "~ῡεσθων";
    imp_pass_2: _, "~ῡεσθον", "~ῡεσθε";
    imp_pass_3: "~ῡεσθω", "~ῡεσθων", "~ῡεσθων";
    imperfect_act_1: "ἐ~ῡον", _, _;
    imperfect_act_2: "ἐ~ῡες", "ἐ~ῡετον", "ἐ~ῡετε";
    imperfect_act_3: "ἐ~ῡε", _, "ἐ~ῡον";
    imperfect_mid_1: "ἐ~ῡομην", _, "ἐ~ῡομεθᾰ";
    imperfect_mid_2: "ἐ~ῡου", _, _;
    imperfect_mid_3: "ἐ~ῡετο", _, "ἐ~ῡοντο";
    imperfect_pass_2: _, "ἐ~ῡεσθον", "ἐ~ῡεσθε";
    imperfect_pass_3: _, "ἐ~ῡεσθην", _;
    aorist_opt_act_2: "~ῡσειᾰς", _, _;
    aorist_opt_act_3: "~ῡσειε", _, "~ῡσειᾰν";
    perfect_ind_act_1: "~ελῠκᾰ", _, "~ελῠκᾰμεν";
    perfect_ind_act_2: _, "~ελῠκᾰτον", _;
    perfect_ind_act_3: _, _, "~ελῠκᾱσῐ";
    perfect_ind_mid_1: _, _, "~ελῠμεθᾰ";
    perfect_ind_mid_2: "~ελῠσαι", "~ελῠσθον", _;
    perfect_ind_mid_3: "~ελῠται", _, "~ελῠνται";
    perfect_ind_pass_1: "~ελῠμαι", _, _;
    perfect_ind_pass_2: _, _, "~ελῠσθε";
    perfect_subj_act_1: "~ελῠκω", _, _;
    perfect_subj_act_2: "~ελῠκῃς", "~ελῠκητον", "~ελῠκητε";
    perfect_subj_act_3: "~ελῠκῃ", _, "~ελῠκωσῐ";
    perfect_imp_act_2: "~ελῠκε", _, _;
    perfect_imp_mid_2: _, _, "~ελῠσθε";
    perfect_imp_mid_3: _, _, "~ελῠσθων";
    perfect_imp_pass_2: "~ελῠσο", _, "~ελῠσθε";
    perfect_imp_pass_3: "~ελῠσθω", "~ελῠσθων", "~ελῠσθων";
    pluperfect_act_1: "ἐ~ελῠκειν", _, _;
    pluperfect_mid_1: "ἐ~ελῠμην", _, _;
    pluperfect_mid_2: "ἐ~ελῠσο", "ἐ~ελῠσθον", _;
    pluperfect_mid_3: "ἐ~ελῠτο", _, "ἐ~ελῠντο";
    pluperfect_pass_1: _, _, "ἐ~ελῠμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ελῠσθε";
    pluperfect_pass_3: _, "ἐ~ελῠσθην", _;
    future_perfect_ind_mid_1: "~ελῡσομαι", _, _;
    future_perfect_ind_mid_3: _, "~ελῡσεσθον", _;
    future_perfect_ind_pass_1: _, _, "~ελῡσομεθᾰ";
    future_perfect_ind_pass_2: "~ελῡσῃ", "~ελῡσεσθον", "~ελῡσεσθε";
    future_perfect_ind_pass_3: "~ελῡσεται", "~ελῡσεσθον", "~ελῡσονται";
    future_perfect_opt_pass_1: "~ελῡσοιμην", _, "~ελῡσοιμεθᾰ";
    future_perfect_opt_pass_2: "~ελῡσοιο", "~ελῡσοισθον", "~ελῡσοισθε";
    future_perfect_opt_pass_3: "~ελῡσοιτο", "~ελῡσοισθην", "~ελῡσοιντο";
    infinitive_pres: "~ῡειν", _, "~ῡεσθαι";
    infinitive_perf: _, "~ελῠσθαι", _;
    infinitive_futp: _, _, "~ελῡσεσθαι";
    participle_pres: "~ῡων", _, "~ῡομενος";
    participle_perf: "~ελῠκως", _, "~ελῠμενος";
    participle_futp: _, _, "~ελῡσομενος";
    ),
    // Θλαω: 7 words (ἀγαλλιαω, κρεμαω, θλαω, τηταω, ἐλᾰω, νεᾰω). Like Νῑκαω, except in 77 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_3 pl, future_ind_mid_1 sg, ….
    conjugation!("θλαω", 7, "~αω", like Νῑκαω;
    stem future_pass: "~ασθησ- like ἐχω";
    stem aorist_ind: "~ασ- like ἐχω";
    stem aorist_ind_pass: "~ασθην- like ἐχω";
    stem aorist_pass: "~ασθ- like ἐχω";
    stem pluperfect_act: "~ακειν- like ἐχω";
    future_ind_act_1: "~ασω", _, "~ασομεν";
    future_ind_act_2: "~ασεις", "~ασετον", _;
    future_ind_act_3: _, _, "~ασουσῐ";
    future_ind_mid_1: "~ασομαι", _, _;
    future_ind_mid_2: "~ασῃ", "~ασεσθον", _;
    future_ind_mid_3: "~ασεται", _, _;
    future_opt_act_1: "~ασοιμῐ", _, "~ασοιμεν";
    future_opt_act_2: "~ασοις", "~ασοιτον", _;
    future_opt_act_3: "~ασοι", "~ασοιτην", _;
    future_opt_mid_1: "~ασοιμην", _, "~ασοιμεθᾰ";
    future_opt_mid_3: _, "~ασοισθην", _;
    aorist_subj_act_1: "~ασω", _, _;
    aorist_subj_act_2: "~ασῃς", "~ασητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~ασωμαι", _, _;
    aorist_subj_mid_2: "~ασῃ", "~ασησθον", _;
    aorist_opt_act_1: "~ασαιμῐ", _, "~ασαιμεν";
    aorist_opt_act_2: "~ασειᾰς", _, _;
    aorist_opt_act_3: "~ασειε", _, "~ασειᾰν";
    aorist_opt_mid_1: "~ασαιμην", _, _;
    aorist_imp_act_2: "~ᾱσον", _, "~ασᾰτε";
    aorist_imp_act_3: _, _, "~ασᾰντων";
    aorist_imp_mid_2: "~ᾱσαι", "~ασᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ασᾰσθων";
    perfect_ind_act_1: "%ε~ακᾰ", _, "%ε~ακᾰμεν";
    perfect_ind_act_2: _, "%ε~ακᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ακᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~αμεθᾰ";
    perfect_ind_mid_2: "%ε~ασαι", "%ε~ασθον", _;
    perfect_ind_mid_3: "%ε~αται", _, "%ε~ανται";
    perfect_ind_pass_1: "%ε~αμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ασθε";
    perfect_subj_act_1: "%ε~ακω", _, _;
    perfect_subj_act_2: "%ε~ακῃς", "%ε~ακητον", "%ε~ακητε";
    perfect_subj_act_3: "%ε~ακῃ", _, "%ε~ακωσῐ";
    perfect_imp_act_2: "%ε~ακε", _, _;
    perfect_imp_mid_2: _, _, "%ε~ασθε";
    perfect_imp_mid_3: _, _, "%ε~ασθων";
    perfect_imp_pass_2: "%ε~ασο", _, "%ε~ασθε";
    perfect_imp_pass_3: "%ε~ασθω", "%ε~ασθων", "%ε~ασθων";
    pluperfect_mid_1: "ἐ%ε~αμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ασο", "ἐ%ε~ασθον", _;
    pluperfect_mid_3: "ἐ%ε~ατο", _, "ἐ%ε~αντο";
    pluperfect_pass_1: _, _, "ἐ%ε~αμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ασθε";
    pluperfect_pass_3: _, "ἐ%ε~ασθην", _;
    infinitive_fut: "~ασειν", _, _;
    infinitive_aor: "~ᾱσαι", _, _;
    infinitive_perf: _, "%ε~ᾱσθαι", _;
    participle_aor: "~ασᾱς", "~ασᾰμενος", _;
    participle_perf: "%ε~ακως", _, "%ε~αμενος";
    ),
    // Ἐπῐλᾰμβᾰνω: 7 words (ἐπῐλᾰμβᾰνω, προλᾰμβᾰνω, ὑπολᾰμβᾰνω, ἀπολᾰμβᾰνω, ξῠλλᾰμβᾰνω, προσλαμβᾰνω). Like Κειμαι, except in 128 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐπῐλᾰμβᾰνω", 7, "~λᾰμβᾰνω", like Κειμαι;
    stem future_pass: "~ληφθησ- like ἐχω";
    stem aorist_ind: "~ᾰβ- like φᾰρμᾰσσω";
    stem aorist_ind_pass: "~ηφθην- like ἐχω";
    stem aorist_pass: "~ληφθ- like ἐχω";
    ind_act_1: "~λᾰμβᾰνω", _, "~λᾰμβᾰνομεν";
    ind_act_2: "~λᾰμβᾰνεις", "~λᾰμβᾰνετον", _;
    ind_act_3: "~λᾰμβᾰνει", _, "~λᾰμβᾰνουσῐ";
    ind_mid_1: _, _, "~λᾰμβᾰνομεθᾰ";
    ind_mid_2: "~λᾰμβᾰνῃ", _, _;
    ind_mid_3: _, "~λᾰμβᾰνεσθον", _;
    ind_pass_1: "~λᾰμβᾰνομαι", _, _;
    ind_pass_2: _, "~λᾰμβᾰνεσθον", "~λᾰμβᾰνεσθε";
    ind_pass_3: "~λᾰμβᾰνεται", "~λᾰμβᾰνεσθον", "~λᾰμβᾰνονται";
    subj_act_1: "~λᾰμβᾰνω", _, "~λᾰμβᾰνωμεν";
    subj_act_2: "~λᾰμβᾰνῃς", _, _;
    subj_act_3: "~λᾰμβᾰνῃ", "~λᾰμβᾰνητον", "~λᾰμβᾰνωσῐ";
    subj_mid_1: _, _, "~λᾰμβᾰνωμεθᾰ";
    subj_mid_2: _, "~λᾰμβᾰνησθον", _;
    subj_mid_3: "~λᾰμβᾰνηται", _, "~λᾰμβᾰνωνται";
    subj_pass_1: "~λᾰμβᾰνωμαι", _, _;
    subj_pass_2: _, _, "~λᾰμβᾰνησθε";
    opt_act_1: "~λᾰμβᾰνοιμῐ", _, "~λᾰμβᾰνοιμεν";
    opt_act_2: "~λᾰμβᾰνοις", "~λᾰμβᾰνοιτον", _;
    opt_act_3: "~λᾰμβᾰνοι", "~λᾰμβᾰνοιτην", "~λᾰμβᾰνοιεν";
    opt_mid_1: "~λᾰμβᾰνοιμην", _, "~λᾰμβᾰνοιμεθᾰ";
    opt_mid_2: "~λᾰμβᾰνοιο", _, "~λᾰμβᾰνοισθε";
    opt_mid_3: "~λᾰμβᾰνοιτο", "~λᾰμβᾰνοισθην", "~λᾰμβᾰνοιντο";
    opt_pass_2: _, "~λᾰμβᾰνοισθον", _;
    imp_act_2: "~λᾰμβᾰνε", _, "~λᾰμβᾰνετε";
    imp_act_3: _, _, "~λᾰμβᾰνοντων";
    imp_mid_2: "~λᾰμβᾰνου", "~λᾰμβᾰνεσθον", "~λᾰμβᾰνεσθε";
    imp_mid_3: _, _, "~λᾰμβᾰνεσθων";
    imp_pass_2: _, "~λᾰμβᾰνεσθον", "~λᾰμβᾰνεσθε";
    imp_pass_3: "~λᾰμβᾰνεσθω", "~λᾰμβᾰνεσθων", "~λᾰμβᾰνεσθων";
    imperfect_act_1: "ἐ~ᾰμβᾰνον", _, _;
    imperfect_act_2: "ἐ~ᾰμβᾰνες", "ἐ~ᾰμβᾰνετον", "ἐ~ᾰμβᾰνετε";
    imperfect_act_3: "ἐ~ᾰμβᾰνε", _, "ἐ~ᾰμβᾰνον";
    imperfect_mid_1: "ἐ~ᾰμβᾰνομην", _, "ἐ~ᾰμβᾰνομεθᾰ";
    imperfect_mid_2: "ἐ~ᾰμβᾰνου", _, _;
    imperfect_mid_3: "ἐ~ᾰμβᾰνετο", _, "ἐ~ᾰμβᾰνοντο";
    imperfect_pass_2: _, "ἐ~ᾰμβᾰνεσθον", "ἐ~ᾰμβᾰνεσθε";
    imperfect_pass_3: _, "ἐ~ᾰμβᾰνεσθην", _;
    future_ind_mid_1: "~ληψομαι", _, _;
    future_ind_mid_2: "~ληψῃ", "~ληψεσθον", _;
    future_ind_mid_3: "~ληψεται", _, _;
    future_opt_mid_1: "~ληψοιμην", _, "~ληψοιμεθᾰ";
    future_opt_mid_3: _, "~ληψοισθην", _;
    aorist_subj_act_1: "~λᾰβω", _, _;
    aorist_subj_act_2: "~λᾰβῃς", "~λᾰβητον", _;
    aorist_subj_mid_1: "~λᾰβωμαι", _, _;
    aorist_subj_mid_2: "~λᾰβῃ", "~λᾰβησθον", _;
    aorist_opt_act_1: "~λᾰβοιμῐ", _, "~λᾰβοιμεν";
    aorist_opt_act_2: "~λᾰβοις", _, _;
    aorist_opt_act_3: "~λᾰβοι", _, "~λᾰβοιεν";
    aorist_opt_mid_1: "~λᾰβοιμην", _, _;
    aorist_imp_act_2: "~λᾰβε", _, "~λᾰβετε";
    aorist_imp_act_3: _, _, "~λᾰβοντων";
    aorist_imp_mid_2: "~λᾰβου", "~λᾰβεσθον", _;
    aorist_imp_mid_3: _, _, "~λᾰβεσθων";
    perfect_ind_act_1: "ἐι~ηφᾰ", _, "ἐι~ηφᾰμεν";
    perfect_ind_act_2: _, "ἐι~ηφᾰτον", _;
    perfect_ind_act_3: _, _, "ἐι~ηφᾱσῐ";
    perfect_ind_mid_1: _, _, "ἐι~ημμεθᾰ";
    perfect_ind_mid_2: "ἐι~ηψαι", "ἐι~ηφθον", _;
    perfect_ind_mid_3: "ἐι~ηπται", _, "ἐι~ηφᾰται";
    perfect_ind_pass_1: "ἐι~ημμαι", _, _;
    perfect_ind_pass_2: _, _, "ἐι~ηφθε";
    perfect_subj_act_1: "ἐι~ηφω", _, _;
    perfect_subj_act_2: "ἐι~ηφῃς", "ἐι~ηφητον", "ἐι~ηφητε";
    perfect_subj_act_3: "ἐι~ηφῃ", _, "ἐι~ηφωσῐ";
    perfect_imp_act_2: "ἐι~ηφε", _, _;
    perfect_imp_mid_2: _, _, "ἐι~ηφθε";
    perfect_imp_mid_3: _, _, "ἐι~ηφθων";
    perfect_imp_pass_2: "ἐι~ηψο", _, "ἐι~ηφθε";
    perfect_imp_pass_3: "ἐι~ηφθω", "ἐι~ηφθων", "ἐι~ηφθων";
    infinitive_pres: "~λᾰμβᾰνειν", _, "~λᾰμβᾰνεσθαι";
    infinitive_aor: "~λᾰβειν", _, _;
    infinitive_perf: _, "ἐι~ηφθαι", _;
    participle_pres: "~λᾰμβᾰνων", _, "~λᾰμβᾰνομενος";
    participle_aor: "~λᾰβων", "~λᾰβομενος", _;
    participle_perf: "ἐι~ηφως", _, "ἐι~ημμενος";
    ),
    // Τελεω: 6 words (ἐμεω, ἀρκεω, τελεω, βδεω, νεικεω, δῐᾰρκεω). Like Πᾰραιτεομαι, except in 78 cells: ind_mid_2 sg, ind_pass_2 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, ….
    conjugation!("τελεω", 6, "~εω", like Πᾰραιτεομαι;
    stem future_pass: "~εσθησ- like ἐχω";
    stem aorist_ind: "~εσ- like ἐχω";
    stem aorist_ind_pass: "~εσθην- like ἐχω";
    stem aorist_pass: "~εσθ- like ἐχω";
    stem pluperfect_act: "~εκειν- like ἐχω";
    ind_mid_2: "~ει", _, _;
    ind_pass_2: derived, _, _;
    future_ind_act_1: "~ω", _, "~ουμεν";
    future_ind_act_2: "~εις", "~ειτον", _;
    future_ind_act_3: _, _, "~ουσῐ";
    future_ind_mid_1: "~ουμαι", _, _;
    future_ind_mid_2: "~ει", "~εισθον", _;
    future_ind_mid_3: "~ειται", _, _;
    future_opt_act_1: "~οιην", _, "~οιμεν";
    future_opt_act_2: "~οιης", "~οιτον", _;
    future_opt_act_3: "~οιη", "~οιτην", _;
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_3: _, "~οισθην", _;
    aorist_subj_act_1: "~εσω", _, _;
    aorist_subj_act_2: "~εσῃς", "~εσητον", _;
    aorist_subj_mid_1: "~εσωμαι", _, _;
    aorist_subj_mid_2: "~εσῃ", "~εσησθον", _;
    aorist_opt_act_1: "~εσαιμῐ", _, "~εσαιμεν";
    aorist_opt_act_2: "~εσειᾰς", _, _;
    aorist_opt_act_3: "~εσειε", _, "~εσειᾰν";
    aorist_opt_mid_1: "~εσαιμην", _, _;
    aorist_imp_act_2: "~εσον", _, "~εσᾰτε";
    aorist_imp_act_3: _, _, "~εσᾰντων";
    aorist_imp_mid_2: "~εσαι", "~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~εσᾰσθων";
    perfect_ind_act_1: "%ε~εκᾰ", _, "%ε~εκᾰμεν";
    perfect_ind_act_2: _, "%ε~εκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~εκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~εσμεθᾰ";
    perfect_ind_mid_2: "%ε~εσαι", "%ε~εσθον", _;
    perfect_ind_mid_3: "%ε~εσται", _, "%ε~εσᾰται";
    perfect_ind_pass_1: "%ε~εσμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~εσθε";
    perfect_subj_act_1: "%ε~εκω", _, _;
    perfect_subj_act_2: "%ε~εκῃς", "%ε~εκητον", "%ε~εκητε";
    perfect_subj_act_3: "%ε~εκῃ", _, "%ε~εκωσῐ";
    perfect_imp_act_2: "%ε~εκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~εσθε";
    perfect_imp_mid_3: _, _, "%ε~εσθων";
    perfect_imp_pass_2: "%ε~εσο", _, "%ε~εσθε";
    perfect_imp_pass_3: "%ε~εσθω", "%ε~εσθων", "%ε~εσθων";
    pluperfect_mid_1: "ἐ%ε~εσμην", _, _;
    pluperfect_mid_2: "ἐ%ε~εσο", "ἐ%ε~εσθον", _;
    pluperfect_mid_3: "ἐ%ε~εστο", _, "ἐ%ε~εσᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~εσμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~εσθε";
    pluperfect_pass_3: _, "ἐ%ε~εσθην", _;
    infinitive_fut: "~ειν", _, _;
    infinitive_aor: "~εσαι", _, _;
    infinitive_perf: _, "%ε~εσθαι", _;
    participle_aor: "~εσᾱς", "~εσᾰμενος", _;
    participle_perf: "%ε~εκως", _, "%ε~εσμενος";
    ),
    // Μωραινω: 6 words (ῥαινω, μιαινω, κερδαινω, μελαινω, εὐφραινω, μωραινω). Like Φαινω, except in 72 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("μωραινω", 6, "~αινω", like Φαινω;
    stem future_pass: "~ανθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ᾱν- like ἐχω";
    stem aorist_ind_pass: "~ανθην- like ἐχω";
    stem aorist_pass: "~ανθ- like ἐχω";
    ind_act_3: _, _, "~αινουσῐ|~αινουσῐν";
    ind_mid_2: "~αινῃ|~αινει", _, _;
    subj_act_3: _, _, "~αινωσῐ|~αινωσῐν";
    imperfect_act_3: "ἐ~αινε|ἐ~αινεν", _, _;
    future_ind_act_1: "~ανω", _, "~ανουμεν";
    future_ind_act_2: "~ανεις", "~ανειτον", _;
    future_ind_act_3: _, _, "~ανουσῐ";
    future_ind_mid_1: "~ανουμαι", _, _;
    future_ind_mid_2: "~ανει", "~ανεισθον", _;
    future_ind_mid_3: "~ανειται", _, _;
    future_opt_act_1: "~ανοιην", _, "~ανοιμεν";
    future_opt_act_2: "~ανοιης", "~ανοιτον", _;
    future_opt_act_3: "~ανοιη", "~ανοιτην", _;
    future_opt_mid_1: "~ανοιμην", _, "~ανοιμεθᾰ";
    future_opt_mid_3: _, "~ανοισθην", _;
    aorist_subj_act_1: "~ᾱνω", _, _;
    aorist_subj_act_2: "~ᾱνῃς", "~ᾱνητον", _;
    aorist_subj_mid_1: "~ᾱνωμαι", _, _;
    aorist_subj_mid_2: "~ᾱνῃ", "~ᾱνησθον", _;
    aorist_opt_act_1: "~ᾱναιμῐ", _, "~ᾱναιμεν";
    aorist_opt_act_2: "~ᾱνειᾰς", _, _;
    aorist_opt_act_3: "~ᾱνειε", _, "~ᾱνειᾰν";
    aorist_opt_mid_1: "~ᾱναιμην", _, _;
    aorist_imp_act_2: "~ᾱνον", _, "~ᾱνᾰτε";
    aorist_imp_act_3: _, _, "~ᾱνᾰντων";
    aorist_imp_mid_2: "~ᾱναι", "~ᾱνᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾱνᾰσθων";
    perfect_ind_act_1: "%ε~ᾰγκᾰ", _, "%ε~ᾰγκᾰμεν";
    perfect_ind_act_2: _, "%ε~ᾰγκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ᾰγκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~ᾰσμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾰνσαι", "%ε~ᾰνθον", _;
    perfect_ind_mid_3: "%ε~ᾰνται", _, "%ε~ᾰνᾰται";
    perfect_ind_pass_1: "%ε~ᾰσμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ᾰνθε";
    perfect_subj_act_1: "%ε~ᾰγκω", _, _;
    perfect_subj_act_2: "%ε~ᾰγκῃς", "%ε~ᾰγκητον", "%ε~ᾰγκητε";
    perfect_subj_act_3: "%ε~ᾰγκῃ", _, "%ε~ᾰγκωσῐ";
    perfect_imp_act_2: "%ε~ᾰγκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~ᾰνθε";
    perfect_imp_mid_3: _, _, "%ε~ᾰνθων";
    perfect_imp_pass_2: "%ε~ᾰνσο", _, "%ε~ᾰνθε";
    perfect_imp_pass_3: "%ε~ᾰνθω", "%ε~ᾰνθων", "%ε~ᾰνθων";
    infinitive_fut: "~ανειν", _, _;
    infinitive_aor: "~ᾱναι", _, _;
    infinitive_perf: _, "%ε~ᾰνθαι", _;
    participle_aor: "~ᾱνᾱς", "~ᾱνᾰμενος", _;
    participle_perf: "%ε~ᾰγκως", _, "%ε~ᾰσμενος";
    ),
    // Κατοικεω: 6 words (κατοικεω, νεκραγωγεω, δῐασκοπεω, παιδεραστεω, πᾰνηγῠρῐαρχεω, ἐξασθενεω). Like Ἐπᾰϊω, except in 124 cells: ind_act_1 pl, ind_act_2 du, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_1 sg, ….
    conjugation!("κατοικεω", 6, "~α~εω", like Ἐπᾰϊω;
    stem future_pass: "~ᾰ~ηθησ- like ἐχω";
    stem aorist_ind: "~ᾰ~ησ- like ἐλπω";
    stem aorist_ind_pass: "~ᾰ~ηθην- like εὐλογεω";
    stem aorist_pass: "~ᾰ~ηθ- like ἐχω";
    ind_act_1: _, _, "~ᾰ~ουμεν";
    ind_act_2: _, "~ᾰ~ειτον", _;
    ind_mid_1: _, _, "~ᾰ~ουμεθᾰ";
    ind_mid_2: "~ᾰ~ει", _, _;
    ind_mid_3: _, "~ᾰ~εισθον", _;
    ind_pass_1: "~ᾰ~ουμαι", _, _;
    ind_pass_2: _, "~ᾰ~εισθον", "~ᾰ~εισθε";
    ind_pass_3: "~ᾰ~ειται", "~ᾰ~εισθον", "~ᾰ~ουνται";
    opt_act_1: "~ᾰ~οιην|~ᾰ~οιμῐ", _, "~ᾰ~οιμεν|~ᾰ~οιημεν";
    opt_act_2: "~ᾰ~οιης|~ᾰ~οις", "~ᾰ~οιτον|~ᾰ~οιητον", "~ᾰ~οιτε|~ᾰ~οιητε";
    opt_act_3: "~ᾰ~οιη|~ᾰ~οι", "~ᾰ~οιτην|~ᾰ~οιητην", "~ᾰ~οιεν|~ᾰ~οιησᾰν";
    imp_act_2: "~ᾰ~ει", _, "~ᾰ~ειτε";
    imp_act_3: _, _, "~ᾰ~ουντων";
    imp_mid_2: _, "~ᾰ~εισθον", "~ᾰ~εισθε";
    imp_mid_3: _, _, "~ᾰ~εισθων";
    imp_pass_2: _, "~ᾰ~εισθον", "~ᾰ~εισθε";
    imp_pass_3: "~ᾰ~εισθω", "~ᾰ~εισθων", "~ᾰ~εισθων";
    imperfect_act_1: "~ᾰ~ουν", _, _;
    imperfect_act_2: "~ᾰ~εις", "~ᾰ~ειτον", "~ᾰ~ειτε";
    imperfect_act_3: "~ᾰ~ει", _, "~ᾰ~ουν";
    imperfect_mid_1: "~ᾰ~ουμην", _, "~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "~ᾰ~ου", _, _;
    imperfect_mid_3: "~ᾰ~ειτο", _, "~ᾰ~ουντο";
    imperfect_pass_2: _, "~ᾰ~εισθον", "~ᾰ~εισθε";
    imperfect_pass_3: _, "~ᾰ~εισθην", _;
    future_ind_act_1: "~ᾰ~ησω", _, "~ᾰ~ησομεν";
    future_ind_act_2: "~ᾰ~ησεις", "~ᾰ~ησετον", _;
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ";
    future_ind_mid_1: "~ᾰ~ησομαι", _, _;
    future_ind_mid_2: "~ᾰ~ησῃ", "~ᾰ~ησεσθον", _;
    future_ind_mid_3: "~ᾰ~ησεται", _, _;
    future_opt_act_1: "~ᾰ~ησοιμῐ", _, "~ᾰ~ησοιμεν";
    future_opt_act_2: "~ᾰ~ησοις", "~ᾰ~ησοιτον", _;
    future_opt_act_3: "~ᾰ~ησοι", "~ᾰ~ησοιτην", _;
    future_opt_mid_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~ησοισθην", _;
    aorist_subj_act_1: "~ᾰ~ησω", _, _;
    aorist_subj_act_2: "~ᾰ~ησῃς", "~ᾰ~ησητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~ᾰ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ησῃ", "~ᾰ~ησησθον", _;
    aorist_opt_act_1: "~ᾰ~ησαιμῐ", _, "~ᾰ~ησαιμεν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς", _, _;
    aorist_opt_act_3: "~ᾰ~ησειε", _, "~ᾰ~ησειᾰν";
    aorist_opt_mid_1: "~ᾰ~ησαιμην", _, _;
    aorist_imp_act_2: "~ᾰ~ησον", _, "~ᾰ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ησᾰντων";
    aorist_imp_mid_2: "~ᾰ~ησαι", "~ᾰ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ησᾰσθων";
    perfect_ind_act_1: "~ᾰ~ηκᾰ", _, "~ᾰ~ηκᾰμεν";
    perfect_ind_act_2: _, "~ᾰ~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "~ᾰ~ηκᾱσῐ";
    perfect_ind_mid_1: _, _, "~ᾰ~ημεθᾰ";
    perfect_ind_mid_2: "~ᾰ~ησαι", "~ᾰ~ησθον", _;
    perfect_ind_mid_3: "~ᾰ~ηται", _, "~ᾰ~ηνται";
    perfect_ind_pass_1: "~ᾰ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ᾰ~ησθε";
    perfect_subj_act_1: "~ᾰ~ηκω", _, _;
    perfect_subj_act_2: "~ᾰ~ηκῃς", "~ᾰ~ηκητον", "~ᾰ~ηκητε";
    perfect_subj_act_3: "~ᾰ~ηκῃ", _, "~ᾰ~ηκωσῐ";
    perfect_imp_act_2: "~ᾰ~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~ᾰ~ησθε";
    perfect_imp_mid_3: _, _, "~ᾰ~ησθων";
    perfect_imp_pass_2: "~ᾰ~ησο", _, "~ᾰ~ησθε";
    perfect_imp_pass_3: "~ᾰ~ησθω", "~ᾰ~ησθων", "~ᾰ~ησθων";
    pluperfect_act_1: "~ᾰ~ηκειν", _, _;
    pluperfect_mid_1: "~ᾰ~ημην", _, _;
    pluperfect_mid_2: "~ᾰ~ησο", "~ᾰ~ησθον", _;
    pluperfect_mid_3: "~ᾰ~ητο", _, "~ᾰ~ηντο";
    pluperfect_pass_1: _, _, "~ᾰ~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ᾰ~ησθε";
    pluperfect_pass_3: _, "~ᾰ~ησθην", _;
    infinitive_pres: _, _, "~ᾰ~εισθαι";
    infinitive_fut: "~ᾰ~ησειν", _, _;
    infinitive_aor: "~ᾰ~ησαι", _, _;
    infinitive_perf: _, "~ᾰ~ησθαι", _;
    participle_pres: _, _, "~ᾰ~ουμενος";
    participle_aor: "~ᾰ~ησᾱς", "~ᾰ~ησᾰμενος", _;
    participle_perf: "~ᾰ~ηκως", _, "~ᾰ~ημενος";
    ),
    // Λᾰγοθηρεω: 6 words (κρατεω, φωνομᾰχεω, τερᾰτοτοκεω, λᾰγοθηρεω, λᾰβρηγορεω, λᾰρκοφορεω). Like Κατοικεω, except in 57 cells: ind_mid_2 sg, imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("λᾰγοθηρεω", 6, "~ᾰ~εω", like Κατοικεω;
    stem future_pass: "~ᾰ~ηθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~α~ησ- like ποιεω";
    stem aorist_ind_pass: "~α~ηθην- like ἐχω";
    stem aorist_pass: "~ᾰ~ηθ- like ποιεω";
    stem pluperfect_act: "~α~ηκειν- like ἐχω";
    ind_mid_2: "~ᾰ~ει|~ᾰ~ῃ", _, _;
    imperfect_act_1: "ἐ~α~ουν", _, _;
    imperfect_act_2: "ἐ~α~εις", "ἐ~α~ειτον", "ἐ~α~ειτε";
    imperfect_act_3: "ἐ~α~ει", _, "ἐ~α~ουν";
    imperfect_mid_1: "ἐ~α~ουμην", _, "ἐ~α~ουμεθᾰ";
    imperfect_mid_2: "ἐ~α~ου", _, _;
    imperfect_mid_3: "ἐ~α~ειτο", _, "ἐ~α~ουντο";
    imperfect_pass_2: _, "ἐ~α~εισθον", "ἐ~α~εισθε";
    imperfect_pass_3: _, "ἐ~α~εισθην", _;
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ|~ᾰ~ησουσῐν";
    future_ind_mid_2: "~ᾰ~ησῃ|~ᾰ~ησει", _, _;
    aorist_subj_act_3: _, _, "~ᾰ~ησωσῐ|~ᾰ~ησωσῐν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς|~ᾰ~ησαις", _, _;
    aorist_opt_act_3: "~ᾰ~ησειε|~ᾰ~ησειεν|~ᾰ~ησαι", _, "~ᾰ~ησειᾰν|~ᾰ~ησαιεν";
    perfect_ind_act_1: "%ε~α~ηκᾰ", _, "%ε~α~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~α~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~α~ηκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~α~ημεθᾰ";
    perfect_ind_mid_2: "%ε~α~ησαι", "%ε~α~ησθον", _;
    perfect_ind_mid_3: "%ε~α~ηται", _, "%ε~α~ηνται";
    perfect_ind_pass_1: "%ε~α~ημαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~α~ησθε";
    perfect_subj_act_1: "%ε~α~ηκω", _, _;
    perfect_subj_act_2: "%ε~α~ηκῃς", "%ε~α~ηκητον", "%ε~α~ηκητε";
    perfect_subj_act_3: "%ε~α~ηκῃ", _, "%ε~α~ηκωσῐ";
    perfect_imp_act_2: "%ε~α~ηκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~α~ησθε";
    perfect_imp_mid_3: _, _, "%ε~α~ησθων";
    perfect_imp_pass_2: "%ε~α~ησο", _, "%ε~α~ησθε";
    perfect_imp_pass_3: "%ε~α~ησθω", "%ε~α~ησθων", "%ε~α~ησθων";
    pluperfect_mid_1: "ἐ%ε~α~ημην", _, _;
    pluperfect_mid_2: "ἐ%ε~α~ησο", "ἐ%ε~α~ησθον", _;
    pluperfect_mid_3: "ἐ%ε~α~ητο", _, "ἐ%ε~α~ηντο";
    pluperfect_pass_1: _, _, "ἐ%ε~α~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~α~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~α~ησθην", _;
    infinitive_perf: _, "%ε~α~ησθαι", _;
    participle_perf: "%ε~α~ηκως", _, "%ε~α~ημενος";
    ),
    // Σειω: 6 words (ζεω, σειω, κελευω, παιω, ἀνῠω, λαυω). Like Πρεσβευω, except in 15 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_ind_mid_1 pl, perfect_ind_mid_3 sg, perfect_ind_mid_3 pl, perfect_ind_pass_1 sg, ….
    conjugation!("σειω", 6, "~ω", like Πρεσβευω;
    perfect_ind_act_3: "%ε~κε|%ε~κεν", _, "%ε~κᾱσῐ|%ε~κᾱσῐν";
    perfect_ind_mid_1: _, _, "%ε~σμεθᾰ";
    perfect_ind_mid_3: "%ε~σται", _, "%ε~σᾰται";
    perfect_ind_pass_1: "%ε~σμαι", _, _;
    perfect_subj_act_3: _, _, "%ε~κωσῐ|%ε~κωσῐν";
    perfect_opt_act_1: "%ε~κοιμῐ|%ε~κοιην", _, _;
    perfect_opt_act_2: "%ε~κοις|%ε~κοιης", _, _;
    perfect_opt_act_3: "%ε~κοι|%ε~κοιη", _, _;
    pluperfect_mid_1: "ἐ%ε~σμην", _, _;
    pluperfect_mid_3: "ἐ%ε~στο", _, "ἐ%ε~σᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~σμεθᾰ";
    participle_perf: _, _, "%ε~σμενος";
    ),
    // Μηκᾰομαι: 6 words (ἰᾰομαι, θεᾰομαι, ἀκροᾰομαι, αἰτῐᾰομαι, ἡσσᾰομαι, μηκᾰομαι). Like Ἀραομαι, except in 40 cells: ind_act_1 sg, imperfect_mid_1 sg, imperfect_mid_1 pl, imperfect_mid_2 sg, imperfect_mid_2 du, imperfect_mid_3 sg, ….
    conjugation!("μηκᾰομαι", 6, "~ᾰομαι", like Ἀραομαι;
    stem future_pass: "~ᾱθησ- like ἐχω";
    stem aorist_ind: "~ᾱσ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ᾱθην- like εὐλογεω";
    stem aorist_pass: "~ᾱθ- like ἐχω";
    ind_act_1: "~ᾰομαι", _, _;
    imperfect_mid_1: "~ωμην", _, "~ωμεθᾰ";
    imperfect_mid_2: "~ω", "~ᾱσθον", _;
    imperfect_mid_3: "~ᾱτο", _, "~ωντο";
    imperfect_pass_2: _, _, "~ᾱσθε";
    imperfect_pass_3: _, "~ᾱσθην", _;
    future_ind_mid_1: "~ᾱσομαι", _, _;
    future_ind_mid_2: "~ᾱσῃ", "~ᾱσεσθον", _;
    future_ind_mid_3: "~ᾱσεται", _, _;
    future_opt_mid_1: "~ᾱσοιμην", _, "~ᾱσοιμεθᾰ";
    future_opt_mid_3: _, "~ᾱσοισθην", _;
    aorist_subj_mid_1: "~ᾱσωμαι", _, _;
    aorist_subj_mid_2: "~ᾱσῃ", "~ᾱσησθον", _;
    aorist_opt_mid_1: "~ᾱσαιμην", _, _;
    aorist_imp_mid_2: "~ᾱσαι", "~ᾱσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾱσᾰσθων";
    perfect_ind_mid_1: _, _, "~ᾱμεθᾰ";
    perfect_ind_mid_2: "~ᾱσαι", "~ᾱσθον", _;
    perfect_ind_mid_3: "~ᾱται", _, "~ᾱνται";
    perfect_ind_pass_1: "~ᾱμαι", _, _;
    perfect_ind_pass_2: _, _, "~ᾱσθε";
    perfect_imp_mid_2: _, _, "~ᾱσθε";
    perfect_imp_mid_3: _, _, "~ᾱσθων";
    perfect_imp_pass_2: "~ᾱσο", _, "~ᾱσθε";
    perfect_imp_pass_3: "~ᾱσθω", "~ᾱσθων", "~ᾱσθων";
    infinitive_perf: _, "~ᾱσθαι", _;
    participle_aor: _, "~ᾱσᾰμενος", _;
    participle_perf: _, _, "~ᾱμενος";
    ),
    // Ἐπεξειμι: 6 words (ἐξειμι, ἀπειμι, κᾰτειμῐ, προσειμι, προειμι, ἐπεξειμι). Like Εἰμῐ, except in 31 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐπεξειμι", 6, "~ειμι", like Εἰμῐ;
    ind_act_1: "~ειμι", _, "~ῐμεν";
    ind_act_2: "~ει", "~ῐτον", _;
    ind_act_3: "~εισῐ|~εισῐν", _, "~ῐᾱσῐ|~ῐᾱσῐν";
    subj_act_1: "~ῐω", _, "~ῐωμεν";
    subj_act_2: "~ῐῃς", "~ῐητον", _;
    subj_act_3: "~ῐῃ", _, "~ῐωσῐ|~ῐωσῐν";
    opt_act_1: "~ῐοιμῐ", _, "~ῐοιμεν";
    opt_act_2: "~ῐοις", "~ῐοιτον", _;
    opt_act_3: "~ῐοι", "~ῐοιτην", "~ῐοιεν";
    imp_act_2: "~ῐθῐ", _, "~ῐτε";
    imp_act_3: _, _, "~ῐοντων";
    imperfect_act_1: "~ῃᾰ|~ῃειν", _, "~ῃμεν";
    imperfect_act_2: "~ῃεισθᾰ|~ῃεις", "~ῃτον", "~ῃτε";
    imperfect_act_3: "~ῃειν|~ῃει", _, "~ῃσᾰν|~ῃεσᾰν";
    infinitive_pres: "~ῐεναι", _, _;
    participle_pres: "~ῐων", _, _;
    ),
    // Ἐαω: 6 words (ἐαω, θηραω, ἀπερᾱω, ἐξερᾱω, θῡμῐαω, φωραω). Like Νῑκαω, except in 69 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_3 pl, future_ind_mid_1 sg, ….
    conjugation!("ἐαω", 6, "~αω", like Νῑκαω;
    stem future_pass: "~ᾱθησ- like ἐχω";
    stem aorist_ind: "~ᾱσ- like ποιεω";
    stem aorist_ind_pass: "~ᾱθην- like ἐχω";
    stem aorist_pass: "~ᾱθ- like ἐχω";
    future_ind_act_1: "~ᾱσω", _, "~ᾱσομεν";
    future_ind_act_2: "~ᾱσεις", "~ᾱσετον", _;
    future_ind_act_3: _, _, "~ᾱσουσῐ";
    future_ind_mid_1: "~ᾱσομαι", _, _;
    future_ind_mid_2: "~ᾱσῃ|~ᾱσει", "~ᾱσεσθον", _;
    future_ind_mid_3: "~ᾱσεται", _, _;
    future_opt_act_1: "~ᾱσοιμῐ", _, "~ᾱσοιμεν";
    future_opt_act_2: "~ᾱσοις", "~ᾱσοιτον", _;
    future_opt_act_3: "~ᾱσοι", "~ᾱσοιτην", _;
    future_opt_mid_1: "~ᾱσοιμην", _, "~ᾱσοιμεθᾰ";
    future_opt_mid_3: _, "~ᾱσοισθην", _;
    aorist_subj_act_1: "~ᾱσω", _, _;
    aorist_subj_act_2: "~ᾱσῃς", "~ᾱσητον", _;
    aorist_subj_act_3: _, _, "~ᾱσωσῐ|~ᾱσωσῐν";
    aorist_subj_mid_1: "~ᾱσωμαι", _, _;
    aorist_subj_mid_2: "~ᾱσῃ", "~ᾱσησθον", _;
    aorist_opt_act_1: "~ᾱσαιμῐ", _, "~ᾱσαιμεν";
    aorist_opt_act_2: "~ᾱσειᾰς|~ᾱσαις", _, _;
    aorist_opt_act_3: "~ᾱσειε|~ᾱσειεν|~ᾱσαι", _, "~ᾱσειᾰν|~ᾱσαιεν";
    aorist_opt_mid_1: "~ᾱσαιμην", _, _;
    aorist_imp_act_2: "~ᾱσον", _, "~ᾱσᾰτε";
    aorist_imp_act_3: _, _, "~ᾱσᾰντων";
    aorist_imp_mid_2: "~ᾱσαι", "~ᾱσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾱσᾰσθων";
    perfect_ind_act_1: "%ε~ᾱκᾰ", _, "%ε~ᾱκᾰμεν";
    perfect_ind_act_2: _, "%ε~ᾱκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ᾱκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~ᾰμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾰσαι", "%ε~ᾰσθον", _;
    perfect_ind_mid_3: "%ε~ᾰται", _, "%ε~ᾰνται";
    perfect_ind_pass_1: "%ε~ᾰμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ᾰσθε";
    perfect_subj_act_1: "%ε~ᾱκω", _, _;
    perfect_subj_act_2: "%ε~ᾱκῃς", "%ε~ᾱκητον", "%ε~ᾱκητε";
    perfect_subj_act_3: "%ε~ᾱκῃ", _, "%ε~ᾱκωσῐ";
    perfect_imp_act_2: "%ε~ᾱκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~ᾰσθε";
    perfect_imp_mid_3: _, _, "%ε~ᾰσθων";
    perfect_imp_pass_2: "%ε~ᾰσο", _, "%ε~ᾰσθε";
    perfect_imp_pass_3: "%ε~ᾰσθω", "%ε~ᾰσθων", "%ε~ᾰσθων";
    infinitive_fut: "~ᾱσειν", _, _;
    infinitive_aor: "~ᾱσαι", _, _;
    infinitive_perf: _, "%ε~ᾰσθαι", _;
    participle_aor: "~ᾱσᾱς", "~ᾱσᾰμενος", _;
    participle_perf: "%ε~ᾱκως", _, "%ε~ᾰμενος";
    ),
    // Περιειμι: 6 words (πᾰρειμι, ἐπειμι, μετειμι, περιειμι, ἐνειμι, σῠνειμῐ). Like Ἐπεξειμι, except in 36 cells: ind_act_1 pl, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, subj_act_1 sg, subj_act_1 pl, ….
    conjugation!("περιειμι", 6, "~ειμι", like Ἐπεξειμι;
    ind_act_1: _, _, "~εσμεν";
    ind_act_2: _, "~εστον", _;
    ind_act_3: "~εστι", _, "~εισι";
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", "~ητον", _;
    subj_act_3: "~ῃ", _, "~ωσι";
    opt_act_1: "~ειην", _, "~ειημεν";
    opt_act_2: "~ειης", "~ειητον", _;
    opt_act_3: "~ειη", "~ειητην", "~ειησαν";
    imp_act_2: "~ισθι", _, "~εστε";
    imp_act_3: _, _, "~εστων";
    imperfect_act_1: "~η", _, "~ημεν";
    imperfect_act_2: "~ησθα", "~ηστον", "~ητε";
    imperfect_act_3: "~ην", _, "~ησαν";
    future_ind_mid_1: "~εσομαι", _, _;
    future_ind_mid_2: "~εσῃ", "~εσεσθον", _;
    future_ind_mid_3: "~εσται", _, _;
    future_opt_mid_1: "~εσοιμην", _, "~εσοιμεθᾰ";
    future_opt_mid_3: _, "~εσοισθην", _;
    infinitive_pres: "~ειναι", _, _;
    participle_pres: "~ων", _, _;
    ),
    // Ἡκω: 5 words (ἡκω, πᾰρηκω, προσηκω, τεθνηκω, κᾰθηκω). Like Δῐωκω, except in 62 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, ….
    conjugation!("ἡκω", 5, "~κω", like Δῐωκω;
    stem aorist_ind: "~ξ- like εὐλογεω";
    ind_act_3: _, _, "~κουσῐ|~κουσῐν";
    ind_mid_2: "~κῃ|~κει", _, _;
    subj_act_3: _, _, "~κωσῐ|~κωσῐν";
    imperfect_act_1: "~κον", _, _;
    imperfect_act_2: "~κες", "~κετον", "~κετε";
    imperfect_act_3: "~κε|~κεν", _, "~κον";
    imperfect_mid_1: "~κομην", _, "~κομεθᾰ";
    imperfect_mid_2: "~κου", _, _;
    imperfect_mid_3: "~κετο", _, "~κοντο";
    imperfect_pass_2: _, "~κεσθον", "~κεσθε";
    imperfect_pass_3: _, "~κεσθην", _;
    future_ind_act_3: _, _, "~ξουσῐ|~ξουσῐν";
    future_ind_mid_2: "~ξῃ|~ξει", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    aorist_subj_act_3: _, _, "~ξωσῐ|~ξωσῐν";
    aorist_opt_act_2: "~ξειᾰς|~ξαις", _, _;
    aorist_opt_act_3: "~ξειε|~ξειεν|~ξαι", _, "~ξειᾰν|~ξαιεν";
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: _, "~κᾰτον", _;
    perfect_ind_act_3: _, _, "~κᾱσῐ";
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "~κω", _, _;
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ";
    perfect_imp_act_2: "~κε", _, _;
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
    participle_perf: "~κως", _, derived;
    ),
    // Ἱζω: 5 words (ἑλληνῐζω, ἀφῠπνῐζω, ἡσῠχᾰζω, ῥωμᾱϊζω, ἐξῠβρῐζω). Like Λογῐζομαι, except in 58 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, ….
    conjugation!("ἱζω", 5, "~ζω", like Λογῐζομαι;
    stem aorist_ind: "~σ- like ἐλπω";
    stem aorist_ind_pass: "~σθην- like εὐλογεω";
    ind_act_3: _, _, "~ζουσῐ|~ζουσῐν";
    ind_mid_2: "~ζῃ|~ζει", _, _;
    subj_act_3: _, _, "~ζωσῐ|~ζωσῐν";
    imperfect_act_1: "~ζον", _, _;
    imperfect_act_2: "~ζες", "~ζετον", "~ζετε";
    imperfect_act_3: "~ζε|~ζεν", _, "~ζον";
    imperfect_mid_1: "~ζομην", _, "~ζομεθᾰ";
    imperfect_mid_2: "~ζου", _, _;
    imperfect_mid_3: "~ζετο", _, "~ζοντο";
    imperfect_pass_2: _, "~ζεσθον", "~ζεσθε";
    imperfect_pass_3: _, "~ζεσθην", _;
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", _, "~σειᾰν|~σαιεν";
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: _, "~κᾰτον", _;
    perfect_ind_act_3: _, _, "~κᾱσῐ";
    perfect_ind_mid_1: _, _, "~σμεθᾰ";
    perfect_ind_mid_2: "~σαι", "~σθον", _;
    perfect_ind_mid_3: "~σται", _, "~δᾰται";
    perfect_ind_pass_1: "~σμαι", _, _;
    perfect_ind_pass_2: _, _, "~σθε";
    perfect_subj_act_1: "~κω", _, _;
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ";
    perfect_imp_act_2: "~κε", _, _;
    perfect_imp_mid_2: _, _, "~σθε";
    perfect_imp_mid_3: _, _, "~σθων";
    perfect_imp_pass_2: "~σο", _, "~σθε";
    perfect_imp_pass_3: "~σθω", "~σθων", "~σθων";
    pluperfect_act_1: "~κειν", _, _;
    pluperfect_mid_1: "~σμην", _, _;
    pluperfect_mid_2: "~σο", "~σθον", _;
    pluperfect_mid_3: "~στο", _, "~δᾰτο";
    pluperfect_pass_1: _, _, "~σμεθᾰ";
    pluperfect_pass_2: _, _, "~σθε";
    pluperfect_pass_3: _, "~σθην", _;
    infinitive_perf: _, "~σθαι", _;
    participle_perf: "~κως", _, "~σμενος";
    ),
    // Φρᾰζω: 5 words (φρᾰζω, θαυμᾰζω, θοᾰζω, θωᾰζω, χειμᾰζω). Like Πειρᾰζω, except in 8 cells: pluperfect_mid_1 sg, pluperfect_mid_2 sg, pluperfect_mid_2 du, pluperfect_mid_3 sg, pluperfect_mid_3 pl, pluperfect_pass_1 pl, ….
    conjugation!("φρᾰζω", 5, "~ζω", like Πειρᾰζω;
    stem pluperfect_act: "~κ- like χρημᾰτῐζω";
    pluperfect_mid_1: "ἐ%ε~σμην", _, _;
    pluperfect_mid_2: "ἐ%ε~σο", "ἐ%ε~σθον", _;
    pluperfect_mid_3: "ἐ%ε~στο", _, "ἐ%ε~δᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~σθε";
    pluperfect_pass_3: _, "ἐ%ε~σθην", _;
    ),
    // Πρεπω: 5 words (τρεπω, πρεπω, πᾰρεκτρεπω, λεπω, ρρεπω). Like Ληγω, except in 148 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πρεπω", 5, "~επω", like Ληγω;
    stem future_pass: "~απησ- like ἐχω";
    stem aorist_ind: "~εψ- like ἐχω";
    stem aorist_ind_pass: "~εφθην- like ἐχω";
    stem aorist_pass: "~εφθ- like ἐχω";
    stem pluperfect_act: "~οφειν- like ἐχω";
    ind_act_1: "~επω", _, "~επομεν";
    ind_act_2: "~επεις", "~επετον", _;
    ind_act_3: "~επει", _, "~επουσῐ";
    ind_mid_1: _, _, "~επομεθᾰ";
    ind_mid_2: "~επῃ", _, _;
    ind_mid_3: _, "~επεσθον", _;
    ind_pass_1: "~επομαι", _, _;
    ind_pass_2: _, "~επεσθον", "~επεσθε";
    ind_pass_3: "~επεται", "~επεσθον", "~επονται";
    subj_act_1: "~επω", _, "~επωμεν";
    subj_act_2: "~επῃς", _, _;
    subj_act_3: "~επῃ", "~επητον", "~επωσῐ";
    subj_mid_1: _, _, "~επωμεθᾰ";
    subj_mid_2: _, "~επησθον", _;
    subj_mid_3: "~επηται", _, "~επωνται";
    subj_pass_1: "~επωμαι", _, _;
    subj_pass_2: _, _, "~επησθε";
    opt_act_1: "~εποιμῐ", _, "~εποιμεν";
    opt_act_2: "~εποις", "~εποιτον", _;
    opt_act_3: "~εποι", "~εποιτην", "~εποιεν";
    opt_mid_1: "~εποιμην", _, "~εποιμεθᾰ";
    opt_mid_2: "~εποιο", _, "~εποισθε";
    opt_mid_3: "~εποιτο", "~εποισθην", "~εποιντο";
    opt_pass_2: _, "~εποισθον", _;
    imp_act_2: "~επε", _, "~επετε";
    imp_act_3: _, _, "~εποντων";
    imp_mid_2: "~επου", "~επεσθον", "~επεσθε";
    imp_mid_3: _, _, "~επεσθων";
    imp_pass_2: _, "~επεσθον", "~επεσθε";
    imp_pass_3: "~επεσθω", "~επεσθων", "~επεσθων";
    imperfect_act_1: "ἐ~επον", _, _;
    imperfect_act_2: "ἐ~επες", "ἐ~επετον", "ἐ~επετε";
    imperfect_act_3: "ἐ~επε", _, "ἐ~επον";
    imperfect_mid_1: "ἐ~επομην", _, "ἐ~επομεθᾰ";
    imperfect_mid_2: "ἐ~επου", _, _;
    imperfect_mid_3: "ἐ~επετο", _, "ἐ~εποντο";
    imperfect_pass_2: _, "ἐ~επεσθον", "ἐ~επεσθε";
    imperfect_pass_3: _, "ἐ~επεσθην", _;
    future_ind_act_1: "~εψω", _, "~εψομεν";
    future_ind_act_2: "~εψεις", "~εψετον", _;
    future_ind_act_3: _, _, "~εψουσῐ";
    future_ind_mid_1: "~εψομαι", _, _;
    future_ind_mid_2: "~εψῃ", "~εψεσθον", _;
    future_ind_mid_3: "~εψεται", _, _;
    future_opt_act_1: "~εψοιμῐ", _, "~εψοιμεν";
    future_opt_act_2: "~εψοις", "~εψοιτον", _;
    future_opt_act_3: "~εψοι", "~εψοιτην", _;
    future_opt_mid_1: "~εψοιμην", _, "~εψοιμεθᾰ";
    future_opt_mid_3: _, "~εψοισθην", _;
    aorist_subj_act_1: "~εψω", _, _;
    aorist_subj_act_2: "~εψῃς", "~εψητον", _;
    aorist_subj_mid_1: "~εψωμαι", _, _;
    aorist_subj_mid_2: "~εψῃ", "~εψησθον", _;
    aorist_opt_act_1: "~εψαιμῐ", _, "~εψαιμεν";
    aorist_opt_act_2: "~εψειᾰς", _, _;
    aorist_opt_act_3: "~εψειε", _, "~εψειᾰν";
    aorist_opt_mid_1: "~εψαιμην", _, _;
    aorist_imp_act_2: "~εψον", _, "~εψᾰτε";
    aorist_imp_act_3: _, _, "~εψᾰντων";
    aorist_imp_mid_2: "~εψαι", "~εψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~εψᾰσθων";
    perfect_ind_act_1: "%ε~οφᾰ", _, "%ε~οφᾰμεν";
    perfect_ind_act_2: _, "%ε~οφᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~οφᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~αμμεθᾰ";
    perfect_ind_mid_2: "%ε~αψαι", "%ε~αφθον", _;
    perfect_ind_mid_3: "%ε~απται", _, "%ε~αφᾰται";
    perfect_ind_pass_1: "%ε~αμμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~αφθε";
    perfect_subj_act_1: "%ε~οφω", _, _;
    perfect_subj_act_2: "%ε~οφῃς", "%ε~οφητον", "%ε~οφητε";
    perfect_subj_act_3: "%ε~οφῃ", _, "%ε~οφωσῐ";
    perfect_imp_act_2: "%ε~οφε", _, _;
    perfect_imp_mid_2: _, _, "%ε~αφθε";
    perfect_imp_mid_3: _, _, "%ε~αφθων";
    perfect_imp_pass_2: "%ε~αψο", _, "%ε~αφθε";
    perfect_imp_pass_3: "%ε~αφθω", "%ε~αφθων", "%ε~αφθων";
    pluperfect_mid_1: "ἐ%ε~αμμην", _, _;
    pluperfect_mid_2: "ἐ%ε~αψο", "ἐ%ε~αφθον", _;
    pluperfect_mid_3: "ἐ%ε~απτο", _, "ἐ%ε~αφᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~αμμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~αφθε";
    pluperfect_pass_3: _, "ἐ%ε~αφθην", _;
    infinitive_pres: "~επειν", _, "~επεσθαι";
    infinitive_fut: "~εψειν", _, _;
    infinitive_aor: "~εψαι", _, _;
    infinitive_perf: _, "%ε~ᾱφθαι", _;
    participle_pres: "~επων", _, "~επομενος";
    participle_aor: "~εψᾱς", "~εψᾰμενος", _;
    participle_perf: "%ε~οφως", _, "%ε~αμμενος";
    ),
    // Στερεω: 5 words (στερεω, σῐδηροφορεω, κορεω, σειρεω, γεωμετρεω). Like Δῐκεω, except in 58 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_3 pl, future_ind_mid_1 sg, ….
    conjugation!("στερεω", 5, "~εω", like Δῐκεω;
    stem future_pass: "~ησ- like ἐχω";
    stem aorist_ind: "~ησ- like ἐχω";
    future_ind_act_1: "~ω", _, "~ουμεν";
    future_ind_act_2: "~εις", "~ειτον", _;
    future_ind_act_3: _, _, "~ουσῐ";
    future_ind_mid_1: "~ουμαι", _, _;
    future_ind_mid_2: "~ει", "~εισθον", _;
    future_ind_mid_3: "~ειται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~οιην", _, "~οιμεν";
    future_opt_act_2: "~οιης", "~οιτον", _;
    future_opt_act_3: "~οιη", "~οιτην", _;
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_3: _, "~οισθην", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_2: "~ησειᾰς", _, _;
    aorist_opt_act_3: "~ησειε", _, "~ησειᾰν";
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: derived, _, "ἐ~ηκᾱσῐ";
    perfect_subj_act_3: _, _, "ἐ~ηκωσῐ";
    perfect_opt_act_1: derived, _, _;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: derived, _, _;
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
    infinitive_fut: "~ειν", _, _;
    infinitive_futp: _, _, "ἐ~ησεσθαι";
    participle_futp: _, _, "ἐ~ησομενος";
    ),
    // Μᾰρτῠρεω: 5 words (μᾰρτῠρεω, λοιδορεω, μᾰραυγεω, δονεω, κερτομεω). Like Ποιεω, except in 19 cells: future_perfect_ind_mid_1 sg, future_perfect_ind_mid_3 du, future_perfect_ind_pass_1 pl, future_perfect_ind_pass_2 sg, future_perfect_ind_pass_2 du, future_perfect_ind_pass_2 pl, ….
    conjugation!("μᾰρτῠρεω", 5, "~εω", like Ποιεω;
    stem future_pass: "~ηθησ- like χρημᾰτῐζω";
    stem aorist_ind_pass: "~ηθην- like εὐλογεω";
    stem aorist_pass: "~ηθ- like ἐλπῐζω";
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
    // Ἐλπῐζω: 5 words (ἐγγῐζω, ἐλπῐζω, ἐρεθῐζω, ἐνᾰρῐζω, ἐρῐζω). Like Ἀθῡρω, except in 152 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐλπῐζω", 5, "ἐ~ζω", like Ἀθῡρω;
    stem future_pass: "~σθησ- like ἐλπῐζω";
    stem aorist_ind: "~σ- like ἐλπῐζω";
    stem aorist_ind_pass: "~σθην- like ἀθῡρω";
    stem aorist_pass: "~σθ- like ἐλπῐζω";
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
    opt_act_2: "ἐ~ζοις", "ἐ~ζοιτον", _;
    opt_act_3: "ἐ~ζοι", "ἐ~ζοιτην", "ἐ~ζοιεν";
    opt_mid_1: "ἐ~ζοιμην", _, "ἐ~ζοιμεθᾰ";
    opt_mid_2: "ἐ~ζοιο", _, "ἐ~ζοισθε";
    opt_mid_3: "ἐ~ζοιτο", "ἐ~ζοισθην", "ἐ~ζοιντο";
    opt_pass_2: _, "ἐ~ζοισθον", _;
    imp_act_2: "ἐ~ζε", _, "ἐ~ζετε";
    imp_act_3: _, _, "ἐ~ζοντων";
    imp_mid_2: "ἐ~ζου", "ἐ~ζεσθον", "ἐ~ζεσθε";
    imp_mid_3: _, _, "ἐ~ζεσθων";
    imp_pass_2: _, "ἐ~ζεσθον", "ἐ~ζεσθε";
    imp_pass_3: "ἐ~ζεσθω", "ἐ~ζεσθων", "ἐ~ζεσθων";
    imperfect_act_1: "ἠ~ζον", _, _;
    imperfect_act_2: "ἠ~ζες", "ἠ~ζετον", "ἠ~ζετε";
    imperfect_act_3: "ἠ~ζε|ἠ~ζεν", _, "ἠ~ζον";
    imperfect_mid_1: "ἠ~ζομην", _, "ἠ~ζομεθᾰ";
    imperfect_mid_2: "ἠ~ζου", _, _;
    imperfect_mid_3: "ἠ~ζετο", _, "ἠ~ζοντο";
    imperfect_pass_2: _, "ἠ~ζεσθον", "ἠ~ζεσθε";
    imperfect_pass_3: _, "ἠ~ζεσθην", _;
    future_ind_act_1: "ἐ~ω", _, "ἐ~ουμεν";
    future_ind_act_2: "ἐ~εις", "ἐ~ειτον", _;
    future_ind_act_3: _, _, "ἐ~ουσῐ|ἐ~ουσῐν";
    future_ind_mid_1: "ἐ~ουμαι", _, _;
    future_ind_mid_2: "ἐ~ει|ἐ~ῃ", "ἐ~εισθον", _;
    future_ind_mid_3: "ἐ~ειται", _, _;
    future_opt_act_1: "ἐ~οιην|ἐ~οιμῐ", _, "ἐ~οιμεν|ἐ~οιημεν";
    future_opt_act_2: "ἐ~οιης|ἐ~οις", "ἐ~οιτον|ἐ~οιητον", "ἐ~οιτε|ἐ~οιητε";
    future_opt_act_3: "ἐ~οιη|ἐ~οι", "ἐ~οιτην|ἐ~οιητην", "ἐ~οιεν|ἐ~οιησᾰν";
    future_opt_mid_1: "ἐ~οιμην", _, "ἐ~οιμεθᾰ";
    future_opt_mid_3: _, "ἐ~οισθην", _;
    aorist_subj_act_1: "ἐ~σω", _, _;
    aorist_subj_act_2: "ἐ~σῃς", "ἐ~σητον", _;
    aorist_subj_act_3: _, _, "ἐ~σωσῐ|ἐ~σωσῐν";
    aorist_subj_mid_1: "ἐ~σωμαι", _, _;
    aorist_subj_mid_2: "ἐ~σῃ", "ἐ~σησθον", _;
    aorist_opt_act_1: "ἐ~σαιμῐ", _, "ἐ~σαιμεν";
    aorist_opt_act_2: "ἐ~σειᾰς|ἐ~σαις", _, _;
    aorist_opt_act_3: "ἐ~σειε|ἐ~σειεν|ἐ~σαι", _, "ἐ~σειᾰν|ἐ~σαιεν";
    aorist_opt_mid_1: "ἐ~σαιμην", _, _;
    aorist_imp_act_2: "ἐ~σον", _, "ἐ~σᾰτε";
    aorist_imp_act_3: _, _, "ἐ~σᾰντων";
    aorist_imp_mid_2: "ἐ~σαι", "ἐ~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἐ~σᾰσθων";
    perfect_ind_act_3: "ἠ~κε|ἠ~κεν", _, "ἠ~κᾱσῐ|ἠ~κᾱσῐν";
    perfect_ind_mid_1: _, _, "ἠ~σμεθᾰ";
    perfect_ind_mid_3: "ἠ~σται", _, "ἠ~δᾰται";
    perfect_ind_pass_1: "ἠ~σμαι", _, _;
    perfect_subj_act_3: _, _, "ἠ~κωσῐ|ἠ~κωσῐν";
    perfect_opt_act_1: "ἠ~κοιμῐ|ἠ~κοιην", _, _;
    perfect_opt_act_2: "ἠ~κοις|ἠ~κοιης", _, _;
    perfect_opt_act_3: "ἠ~κοι|ἠ~κοιη", _, _;
    pluperfect_act_1: "ἠ~κειν|ἠ~κη", _, _;
    pluperfect_act_2: "ἠ~κεις|ἠ~κης", _, _;
    pluperfect_act_3: "ἠ~κει|ἠ~κειν", _, _;
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
    infinitive_fut: "ἐ~ειν", _, _;
    infinitive_aor: "ἐ~σαι", _, _;
    infinitive_futp: _, _, derived;
    participle_pres: "ἐ~ζων", _, "ἐ~ζομενος";
    participle_aor: "ἐ~σᾱς", "ἐ~σᾰμενος", _;
    participle_perf: _, _, "ἠ~σμενος";
    participle_futp: _, _, derived;
    ),
    // Ἀναιρεω: 5 words (ἀναιρεω, ἀφαιρεω, ἐξαιρεω, δῐαιρεω, ὑφαιρεω). Like Ληγω, except in 155 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀναιρεω", 5, "~αιρεω", like Ληγω;
    stem future_pass: "~αιρεθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~- like ἀναιρεω";
    stem aorist_ind_pass: "~ῃρεθην- like εὐλογεω";
    stem aorist_pass: "~αιρεθ- like ποιεω";
    ind_act_1: "~αιρω", _, "~αιρουμεν";
    ind_act_2: "~αιρεις", "~αιρειτον", _;
    ind_act_3: "~αιρει", _, "~αιρουσῐ|~αιρουσῐν";
    ind_mid_1: _, _, "~αιρουμεθᾰ";
    ind_mid_2: "~αιρει|~αιρῃ", _, _;
    ind_mid_3: _, "~αιρεισθον", _;
    ind_pass_1: "~αιρουμαι", _, _;
    ind_pass_2: _, "~αιρεισθον", "~αιρεισθε";
    ind_pass_3: "~αιρειται", "~αιρεισθον", "~αιρουνται";
    subj_act_1: "~αιρω", _, "~αιρωμεν";
    subj_act_2: "~αιρῃς", _, _;
    subj_act_3: "~αιρῃ", "~αιρητον", "~αιρωσῐ|~αιρωσῐν";
    subj_mid_1: _, _, "~αιρωμεθᾰ";
    subj_mid_2: _, "~αιρησθον", _;
    subj_mid_3: "~αιρηται", _, "~αιρωνται";
    subj_pass_1: "~αιρωμαι", _, _;
    subj_pass_2: _, _, "~αιρησθε";
    opt_act_1: "~αιροιην|~αιροιμῐ", _, "~αιροιμεν|~αιροιημεν";
    opt_act_2: "~αιροιης|~αιροις", "~αιροιτον|~αιροιητον", "~αιροιτε|~αιροιητε";
    opt_act_3: "~αιροιη|~αιροι", "~αιροιτην|~αιροιητην", "~αιροιεν|~αιροιησᾰν";
    opt_mid_1: "~αιροιμην", _, "~αιροιμεθᾰ";
    opt_mid_2: "~αιροιο", _, "~αιροισθε";
    opt_mid_3: "~αιροιτο", "~αιροισθην", "~αιροιντο";
    opt_pass_2: _, "~αιροισθον", _;
    imp_act_2: "~αιρει", _, "~αιρειτε";
    imp_act_3: _, _, "~αιρουντων";
    imp_mid_2: "~αιρου", "~αιρεισθον", "~αιρεισθε";
    imp_mid_3: _, _, "~αιρεισθων";
    imp_pass_2: _, "~αιρεισθον", "~αιρεισθε";
    imp_pass_3: "~αιρεισθω", "~αιρεισθων", "~αιρεισθων";
    imperfect_act_1: "~ῃρουν", _, _;
    imperfect_act_2: "~ῃρεις", "~ῃρειτον", "~ῃρειτε";
    imperfect_act_3: "~ῃρει", _, "~ῃρουν";
    imperfect_mid_1: "~ῃρουμην", _, "~ῃρουμεθᾰ";
    imperfect_mid_2: "~ῃρου", _, _;
    imperfect_mid_3: "~ῃρειτο", _, "~ῃρουντο";
    imperfect_pass_2: _, "~ῃρεισθον", "~ῃρεισθε";
    imperfect_pass_3: _, "~ῃρεισθην", _;
    future_ind_act_1: "~αιρησω", _, "~αιρησομεν";
    future_ind_act_2: "~αιρησεις", "~αιρησετον", _;
    future_ind_act_3: _, _, "~αιρησουσῐ|~αιρησουσῐν";
    future_ind_mid_1: "~αιρησομαι", _, _;
    future_ind_mid_2: "~αιρησῃ|~αιρησει", "~αιρησεσθον", _;
    future_ind_mid_3: "~αιρησεται", _, _;
    future_opt_act_1: "~αιρησοιμῐ", _, "~αιρησοιμεν";
    future_opt_act_2: "~αιρησοις", "~αιρησοιτον", _;
    future_opt_act_3: "~αιρησοι", "~αιρησοιτην", _;
    future_opt_mid_1: "~αιρησοιμην", _, "~αιρησοιμεθᾰ";
    future_opt_mid_3: _, "~αιρησοισθην", _;
    aorist_subj_act_1: "~ελω|~αιρησω", _, "~ελωμεν|~αιρησωμεν";
    aorist_subj_act_2: "~ελῃς|~αιρησῃς", "~ελητον|~αιρησητον", "~ελητε|~αιρησητε";
    aorist_subj_act_3: "~ελῃ|~αιρησῃ", _, "~ελωσῐ|~ελωσῐν|~αιρησωσῐ|~αιρησωσῐν";
    aorist_subj_mid_1: "~ελωμαι", _, _;
    aorist_subj_mid_2: "~ελῃ", "~ελησθον", _;
    aorist_opt_act_1: "~ελοιμῐ|~αιρησαιμῐ", _, "~ελοιμεν|~αιρησαιμεν";
    aorist_opt_act_2: "~ελοις|~αιρησειᾰς|~αιρησαις", "~ελοιτον|~αιρησαιτον", "~ελοιτε|~αιρησαιτε";
    aorist_opt_act_3: "~ελοι|~αιρησειε|~αιρησειεν|~αιρησαι", "~ελοιτην|~αιρησαιτην", "~ελοιεν|~αιρησειᾰν|~αιρησαιεν";
    aorist_opt_mid_1: "~ελοιμην", _, _;
    aorist_imp_act_2: "~ελε|~αιρησον", "~ελετον|~αιρησᾰτον", "~ελετε|~αιρησᾰτε";
    aorist_imp_act_3: "~ελετω|~αιρησᾰτω", "~ελετων|~αιρησᾰτων", "~ελοντων|~αιρησᾰντων";
    aorist_imp_mid_2: "~ελου", "~ελεσθον", _;
    aorist_imp_mid_3: _, _, "~ελεσθων";
    perfect_ind_act_1: "~ῃρηκᾰ", _, "~ῃρηκᾰμεν";
    perfect_ind_act_2: _, "~ῃρηκᾰτον", _;
    perfect_ind_act_3: "~ῃρηκε|~ῃρηκεν", _, "~ῃρηκᾱσῐ|~ῃρηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ῃρημεθᾰ";
    perfect_ind_mid_2: "~ῃρησαι", "~ῃρησθον", _;
    perfect_ind_mid_3: "~ῃρηται", _, "~ῃρηνται";
    perfect_ind_pass_1: "~ῃρημαι", _, _;
    perfect_ind_pass_2: _, _, "~ῃρησθε";
    perfect_subj_act_1: "~ῃρηκω", _, _;
    perfect_subj_act_2: "~ῃρηκῃς", "~ῃρηκητον", "~ῃρηκητε";
    perfect_subj_act_3: "~ῃρηκῃ", _, "~ῃρηκωσῐ|~ῃρηκωσῐν";
    perfect_opt_act_1: "~ῃρηκοιμῐ|~ῃρηκοιην", _, _;
    perfect_opt_act_2: "~ῃρηκοις|~ῃρηκοιης", _, _;
    perfect_opt_act_3: "~ῃρηκοι|~ῃρηκοιη", _, _;
    perfect_imp_act_2: "~ῃρηκε", _, _;
    perfect_imp_mid_2: _, _, "~ῃρησθε";
    perfect_imp_mid_3: _, _, "~ῃρησθων";
    perfect_imp_pass_2: "~ῃρησο", _, "~ῃρησθε";
    perfect_imp_pass_3: "~ῃρησθω", "~ῃρησθων", "~ῃρησθων";
    infinitive_pres: "~αιρειν", _, "~αιρεισθαι";
    infinitive_fut: "~αιρησειν", _, _;
    infinitive_aor: "~ελειν|~αιρησαι", _, _;
    infinitive_perf: _, "~ῃρησθαι", _;
    participle_pres: "~αιρων", _, "~αιρουμενος";
    participle_aor: "~ελων|~αιρησᾱς", "~ελομενος", _;
    participle_perf: "~ῃρηκως", _, "~ῃρημενος";
    ),
    // Σῠνοικῐζω: 5 words (ἀνοικῐζω, κᾰτοικῐζω, σῠνοικῐζω, ξῠνοικῐζω, ἀποικῐζω). Like Πεμπω, except in 155 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("σῠνοικῐζω", 5, "~οι~ζω", like Πεμπω;
    stem future_pass: "~οι~σθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ῳ~σ- like εὐλογεω";
    stem aorist_ind_pass: "~ῳ~σθην- like εὐλογεω";
    stem aorist_pass: "~οι~σθ- like ποιεω";
    stem pluperfect_act: "~ῳ~κ- like εὐλογεω";
    ind_act_1: "~οι~ζω", _, "~οι~ζομεν";
    ind_act_2: "~οι~ζεις", "~οι~ζετον", _;
    ind_act_3: "~οι~ζει", _, "~οι~ζουσῐ|~οι~ζουσῐν";
    ind_mid_1: _, _, "~οι~ζομεθᾰ";
    ind_mid_2: "~οι~ζῃ|~οι~ζει", _, _;
    ind_mid_3: _, "~οι~ζεσθον", _;
    ind_pass_1: "~οι~ζομαι", _, _;
    ind_pass_2: _, "~οι~ζεσθον", "~οι~ζεσθε";
    ind_pass_3: "~οι~ζεται", "~οι~ζεσθον", "~οι~ζονται";
    subj_act_1: "~οι~ζω", _, "~οι~ζωμεν";
    subj_act_2: "~οι~ζῃς", _, _;
    subj_act_3: "~οι~ζῃ", "~οι~ζητον", "~οι~ζωσῐ|~οι~ζωσῐν";
    subj_mid_1: _, _, "~οι~ζωμεθᾰ";
    subj_mid_2: _, "~οι~ζησθον", _;
    subj_mid_3: "~οι~ζηται", _, "~οι~ζωνται";
    subj_pass_1: "~οι~ζωμαι", _, _;
    subj_pass_2: _, _, "~οι~ζησθε";
    opt_act_1: "~οι~ζοιμῐ", _, "~οι~ζοιμεν";
    opt_act_2: "~οι~ζοις", "~οι~ζοιτον", _;
    opt_act_3: "~οι~ζοι", "~οι~ζοιτην", "~οι~ζοιεν";
    opt_mid_1: "~οι~ζοιμην", _, "~οι~ζοιμεθᾰ";
    opt_mid_2: "~οι~ζοιο", _, "~οι~ζοισθε";
    opt_mid_3: "~οι~ζοιτο", "~οι~ζοισθην", "~οι~ζοιντο";
    opt_pass_2: _, "~οι~ζοισθον", _;
    imp_act_2: "~οι~ζε", _, "~οι~ζετε";
    imp_act_3: _, _, "~οι~ζοντων";
    imp_mid_2: "~οι~ζου", "~οι~ζεσθον", "~οι~ζεσθε";
    imp_mid_3: _, _, "~οι~ζεσθων";
    imp_pass_2: _, "~οι~ζεσθον", "~οι~ζεσθε";
    imp_pass_3: "~οι~ζεσθω", "~οι~ζεσθων", "~οι~ζεσθων";
    imperfect_act_1: "~ῳ~ζον", _, _;
    imperfect_act_2: "~ῳ~ζες", "~ῳ~ζετον", "~ῳ~ζετε";
    imperfect_act_3: "~ῳ~ζε|~ῳ~ζεν", _, "~ῳ~ζον";
    imperfect_mid_1: "~ῳ~ζομην", _, "~ῳ~ζομεθᾰ";
    imperfect_mid_2: "~ῳ~ζου", _, _;
    imperfect_mid_3: "~ῳ~ζετο", _, "~ῳ~ζοντο";
    imperfect_pass_2: _, "~ῳ~ζεσθον", "~ῳ~ζεσθε";
    imperfect_pass_3: _, "~ῳ~ζεσθην", _;
    future_ind_act_1: "~οι~ω", _, "~οι~ουμεν";
    future_ind_act_2: "~οι~εις", "~οι~ειτον", _;
    future_ind_act_3: _, _, "~οι~ουσῐ|~οι~ουσῐν";
    future_ind_mid_1: "~οι~ουμαι", _, _;
    future_ind_mid_2: "~οι~ει|~οι~ῃ", "~οι~εισθον", _;
    future_ind_mid_3: "~οι~ειται", _, _;
    future_opt_act_1: "~οι~οιην|~οι~οιμῐ", _, "~οι~οιμεν|~οι~οιημεν";
    future_opt_act_2: "~οι~οιης|~οι~οις", "~οι~οιτον|~οι~οιητον", "~οι~οιτε|~οι~οιητε";
    future_opt_act_3: "~οι~οιη|~οι~οι", "~οι~οιτην|~οι~οιητην", "~οι~οιεν|~οι~οιησᾰν";
    future_opt_mid_1: "~οι~οιμην", _, "~οι~οιμεθᾰ";
    future_opt_mid_3: _, "~οι~οισθην", _;
    aorist_subj_act_1: "~οι~σω", _, _;
    aorist_subj_act_2: "~οι~σῃς", "~οι~σητον", _;
    aorist_subj_act_3: _, _, "~οι~σωσῐ|~οι~σωσῐν";
    aorist_subj_mid_1: "~οι~σωμαι", _, _;
    aorist_subj_mid_2: "~οι~σῃ", "~οι~σησθον", _;
    aorist_opt_act_1: "~οι~σαιμῐ", _, "~οι~σαιμεν";
    aorist_opt_act_2: "~οι~σειᾰς|~οι~σαις", _, _;
    aorist_opt_act_3: "~οι~σειε|~οι~σειεν|~οι~σαι", _, "~οι~σειᾰν|~οι~σαιεν";
    aorist_opt_mid_1: "~οι~σαιμην", _, _;
    aorist_imp_act_2: "~οι~σον", _, "~οι~σᾰτε";
    aorist_imp_act_3: _, _, "~οι~σᾰντων";
    aorist_imp_mid_2: "~οι~σαι", "~οι~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~οι~σᾰσθων";
    perfect_ind_act_1: "~ῳ~κᾰ", _, "~ῳ~κᾰμεν";
    perfect_ind_act_2: _, "~ῳ~κᾰτον", _;
    perfect_ind_act_3: "~ῳ~κε|~ῳ~κεν", _, "~ῳ~κᾱσῐ|~ῳ~κᾱσῐν";
    perfect_ind_mid_1: _, _, "~ῳ~σμεθᾰ";
    perfect_ind_mid_2: "~ῳ~σαι", "~ῳ~σθον", _;
    perfect_ind_mid_3: "~ῳ~σται", _, "~ῳ~δᾰται";
    perfect_ind_pass_1: "~ῳ~σμαι", _, _;
    perfect_ind_pass_2: _, _, "~ῳ~σθε";
    perfect_subj_act_1: "~ῳ~κω", _, _;
    perfect_subj_act_2: "~ῳ~κῃς", "~ῳ~κητον", "~ῳ~κητε";
    perfect_subj_act_3: "~ῳ~κῃ", _, "~ῳ~κωσῐ|~ῳ~κωσῐν";
    perfect_opt_act_1: "~ῳ~κοιμῐ|~ῳ~κοιην", _, _;
    perfect_opt_act_2: "~ῳ~κοις|~ῳ~κοιης", _, _;
    perfect_opt_act_3: "~ῳ~κοι|~ῳ~κοιη", _, _;
    perfect_imp_act_2: "~ῳ~κε", _, _;
    perfect_imp_mid_2: _, _, "~ῳ~σθε";
    perfect_imp_mid_3: _, _, "~ῳ~σθων";
    perfect_imp_pass_2: "~ῳ~σο", _, "~ῳ~σθε";
    perfect_imp_pass_3: "~ῳ~σθω", "~ῳ~σθων", "~ῳ~σθων";
    pluperfect_mid_1: "~ῳ~σμην", _, _;
    pluperfect_mid_2: "~ῳ~σο", "~ῳ~σθον", _;
    pluperfect_mid_3: "~ῳ~στο", _, "~ῳ~δᾰτο";
    pluperfect_pass_1: _, _, "~ῳ~σμεθᾰ";
    pluperfect_pass_2: _, _, "~ῳ~σθε";
    pluperfect_pass_3: _, "~ῳ~σθην", _;
    infinitive_pres: "~οι~ζειν", _, "~οι~ζεσθαι";
    infinitive_fut: "~οι~ειν", _, _;
    infinitive_aor: "~οι~σαι", _, _;
    infinitive_perf: _, "~ῳ~σθαι", _;
    participle_pres: "~οι~ζων", _, "~οι~ζομενος";
    participle_aor: "~οι~σᾱς", "~οι~σᾰμενος", _;
    participle_perf: "~ῳ~κως", _, "~ῳ~σμενος";
    ),
    // Θρῠπτω: 5 words (βᾰπτω, θρῠπτω, γνᾰμπτω, μᾰρπτω, δρῠπτω). Like Ἁπτω, except in 66 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_3 pl, future_ind_mid_2 sg, ….
    conjugation!("θρῠπτω", 5, "~πτω", like Ἁπτω;
    stem future_pass: "~φησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ψ- like ποιεω";
    stem aorist_pass: "~φθ- like ποιεω";
    ind_act_3: _, _, "~πτουσῐ|~πτουσῐν";
    ind_mid_2: "~πτῃ|~πτει", _, _;
    subj_act_3: _, _, "~πτωσῐ|~πτωσῐν";
    imperfect_act_3: "ἐ~πτε|ἐ~πτεν", _, _;
    future_ind_act_3: _, _, "~ψουσῐ|~ψουσῐν";
    future_ind_mid_2: "~ψῃ|~ψει", _, _;
    aorist_subj_act_3: _, _, "~ψωσῐ|~ψωσῐν";
    aorist_opt_act_2: "~ψειᾰς|~ψαις", _, _;
    aorist_opt_act_3: "~ψειε|~ψειεν|~ψαι", _, "~ψειᾰν|~ψαιεν";
    perfect_ind_act_1: "%ε~πτᾰ", _, "%ε~πτᾰμεν";
    perfect_ind_act_2: _, "%ε~πτᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~πτᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~μμεθᾰ";
    perfect_ind_mid_2: "%ε~ψαι", "%ε~φθον", _;
    perfect_ind_mid_3: "%ε~πται", _, "%ε~φᾰται";
    perfect_ind_pass_1: "%ε~μμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~φθε";
    perfect_subj_act_1: "%ε~πτω", _, _;
    perfect_subj_act_2: "%ε~πτῃς", "%ε~πτητον", "%ε~πτητε";
    perfect_subj_act_3: "%ε~πτῃ", _, "%ε~πτωσῐ";
    perfect_imp_act_2: "%ε~πτε", _, _;
    perfect_imp_mid_2: _, _, "%ε~φθε";
    perfect_imp_mid_3: _, _, "%ε~φθων";
    perfect_imp_pass_2: "%ε~ψο", _, "%ε~φθε";
    perfect_imp_pass_3: "%ε~φθω", "%ε~φθων", "%ε~φθων";
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
    infinitive_perf: _, "%ε~φθαι", _;
    infinitive_futp: _, _, derived;
    participle_perf: "%ε~πτως", _, "%ε~μμενος";
    participle_futp: _, _, derived;
    ),
    // Τρεφω: 5 words (νεμω, τρεφω, δερω, δεμω, ξῠμμενω). Like Ἐγειρω, except in 109 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("τρεφω", 5, "~ε~ω", like Ἐγειρω;
    ind_act_1: "~ε~ω", _, "~ε~ομεν";
    ind_act_2: "~ε~εις", "~ε~ετον", _;
    ind_act_3: "~ε~ει", _, "~ε~ουσῐ";
    ind_mid_1: _, _, "~ε~ομεθᾰ";
    ind_mid_2: "~ε~ῃ", _, _;
    ind_mid_3: _, "~ε~εσθον", _;
    ind_pass_1: "~ε~ομαι", _, _;
    ind_pass_2: _, "~ε~εσθον", "~ε~εσθε";
    ind_pass_3: "~ε~εται", "~ε~εσθον", "~ε~ονται";
    subj_act_1: "~ε~ω", _, "~ε~ωμεν";
    subj_act_2: "~ε~ῃς", _, _;
    subj_act_3: "~ε~ῃ", "~ε~ητον", "~ε~ωσῐ";
    subj_mid_1: _, _, "~ε~ωμεθᾰ";
    subj_mid_2: _, "~ε~ησθον", _;
    subj_mid_3: "~ε~ηται", _, "~ε~ωνται";
    subj_pass_1: "~ε~ωμαι", _, _;
    subj_pass_2: _, _, "~ε~ησθε";
    opt_act_1: "~ε~οιμῐ", _, "~ε~οιμεν";
    opt_act_2: "~ε~οις", "~ε~οιτον", _;
    opt_act_3: "~ε~οι", "~ε~οιτην", "~ε~οιεν";
    opt_mid_1: "~ε~οιμην", _, "~ε~οιμεθᾰ";
    opt_mid_2: "~ε~οιο", _, "~ε~οισθε";
    opt_mid_3: "~ε~οιτο", "~ε~οισθην", "~ε~οιντο";
    opt_pass_2: _, "~ε~οισθον", _;
    imp_act_2: "~ε~ε", _, "~ε~ετε";
    imp_act_3: _, _, "~ε~οντων";
    imp_mid_2: "~ε~ου", "~ε~εσθον", "~ε~εσθε";
    imp_mid_3: _, _, "~ε~εσθων";
    imp_pass_2: _, "~ε~εσθον", "~ε~εσθε";
    imp_pass_3: "~ε~εσθω", "~ε~εσθων", "~ε~εσθων";
    imperfect_act_1: "ἐ~ε~ον", _, _;
    imperfect_act_2: "ἐ~ε~ες", "ἐ~ε~ετον", "ἐ~ε~ετε";
    imperfect_act_3: "ἐ~ε~ε", _, "ἐ~ε~ον";
    imperfect_mid_1: "ἐ~ε~ομην", _, "ἐ~ε~ομεθᾰ";
    imperfect_mid_2: "ἐ~ε~ου", _, _;
    imperfect_mid_3: "ἐ~ε~ετο", _, "ἐ~ε~οντο";
    imperfect_pass_2: _, "ἐ~ε~εσθον", "ἐ~ε~εσθε";
    imperfect_pass_3: _, "ἐ~ε~εσθην", _;
    perfect_ind_act_1: "~εδᾰ~κᾰ", _, "~εδᾰ~κᾰμεν";
    perfect_ind_act_2: _, "~εδᾰ~κᾰτον", _;
    perfect_ind_act_3: _, _, "~εδᾰ~κᾱσῐ";
    perfect_ind_mid_1: _, _, "~εδᾰ~μεθᾰ";
    perfect_ind_mid_2: "~εδᾰ~σαι", "~εδᾰ~θον", _;
    perfect_ind_mid_3: "~εδᾰ~ται", _, "~εδᾰ~ᾰται";
    perfect_ind_pass_1: "~εδᾰ~μαι", _, _;
    perfect_ind_pass_2: _, _, "~εδᾰ~θε";
    perfect_subj_act_1: "~εδᾰ~κω", _, _;
    perfect_subj_act_2: "~εδᾰ~κῃς", "~εδᾰ~κητον", "~εδᾰ~κητε";
    perfect_subj_act_3: "~εδᾰ~κῃ", _, "~εδᾰ~κωσῐ";
    perfect_imp_act_2: "~εδᾰ~κε", _, _;
    perfect_imp_mid_2: _, _, "~εδᾰ~θε";
    perfect_imp_mid_3: _, _, "~εδᾰ~θων";
    perfect_imp_pass_2: "~εδᾰ~σο", _, "~εδᾰ~θε";
    perfect_imp_pass_3: "~εδᾰ~θω", "~εδᾰ~θων", "~εδᾰ~θων";
    pluperfect_act_1: "ἐ~εδᾰ~κειν", _, _;
    pluperfect_mid_1: "ἐ~εδᾰ~μην", _, _;
    pluperfect_mid_2: "ἐ~εδᾰ~σο", "ἐ~εδᾰ~θον", _;
    pluperfect_mid_3: "ἐ~εδᾰ~το", _, "ἐ~εδᾰ~ᾰτο";
    pluperfect_pass_1: _, _, "ἐ~εδᾰ~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~εδᾰ~θε";
    pluperfect_pass_3: _, "ἐ~εδᾰ~θην", _;
    infinitive_pres: "~ε~ειν", _, "~ε~εσθαι";
    infinitive_perf: _, "~εδᾰ~θαι", _;
    participle_pres: "~ε~ων", _, "~ε~ομενος";
    participle_perf: "~εδᾰ~κως", _, "~εδᾰ~μενος";
    ),
    // Μετερχομαι: 5 words (προερχομαι, σῠνερχομαι, ἐπεξερχομαι, μετερχομαι, ξῠνερχομαι). Like Εἰσερχομαι, except in 22 cells: ind_mid_2 du, ind_mid_3 du, ind_pass_2 pl, ind_pass_3 sg, ind_pass_3 pl, subj_pass_2 sg, ….
    conjugation!("μετερχομαι", 5, "~ερχομαι", like Εἰσερχομαι;
    ind_mid_2: _, derived, _;
    ind_mid_3: _, "~ερχεσθον", _;
    ind_pass_2: _, _, derived;
    ind_pass_3: derived, _, derived;
    subj_pass_2: derived, _, derived;
    opt_pass_2: _, derived, _;
    imp_mid_2: _, "~ερχεσθον", _;
    imp_pass_2: _, _, derived;
    imp_pass_3: _, derived, derived;
    perfect_ind_act_1: _, _, "~εληλῠθᾰμεν";
    perfect_ind_act_3: "~εληλῠθε|~εληλῠθεν", _, "~εληλῠθᾱσῐ|~εληλῠθᾱσῐν";
    perfect_subj_act_3: _, _, "~εληλῠθωσῐ|~εληλῠθωσῐν";
    perfect_opt_act_1: "~εληλῠθοιμῐ|~εληλῠθοιην", _, _;
    perfect_opt_act_2: "~εληλῠθοις|~εληλῠθοιης", _, _;
    perfect_opt_act_3: "~εληλῠθοι|~εληλῠθοιη", _, _;
    pluperfect_act_1: "~εληλυθειν", _, _;
    infinitive_pres: _, _, derived;
    participle_pres: _, _, derived;
    ),
    // Ἀποκωλῡω: 5 words (ἀποδῠω, ἀποκωλῡω, προκῐνδῡνευω, ὑποφερω, ἀποθραυω). Like Πῠνθανομαι, except in 147 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀποκωλῡω", 5, "~ο~ω", like Πῠνθανομαι;
    stem future_pass: "~ο~θησ- like ἐχω";
    stem aorist_ind: "~ε~σ- like ἐλπω";
    stem aorist_ind_pass: "~ε~θην- like εὐλογεω";
    stem aorist_pass: "~ο~θ- like ἐχω";
    ind_act_1: "~ο~ω", _, "~ο~ομεν";
    ind_act_2: "~ο~εις", "~ο~ετον", _;
    ind_act_3: "~ο~ει", _, "~ο~ουσῐ";
    ind_mid_1: derived, _, "~ο~ομεθᾰ";
    ind_mid_2: "~ο~ῃ", _, _;
    ind_mid_3: _, "~ο~εσθον", _;
    ind_pass_1: "~ο~ομαι", _, _;
    ind_pass_2: derived, "~ο~εσθον", "~ο~εσθε";
    ind_pass_3: "~ο~εται", "~ο~εσθον", "~ο~ονται";
    subj_act_1: "~ο~ω", _, "~ο~ωμεν";
    subj_act_2: "~ο~ῃς", _, _;
    subj_act_3: "~ο~ῃ", "~ο~ητον", "~ο~ωσῐ";
    subj_mid_1: derived, _, "~ο~ωμεθᾰ";
    subj_mid_2: derived, "~ο~ησθον", _;
    subj_mid_3: "~ο~ηται", _, "~ο~ωνται";
    subj_pass_1: "~ο~ωμαι", _, _;
    subj_pass_2: derived, _, "~ο~ησθε";
    opt_act_1: "~ο~οιμῐ", _, "~ο~οιμεν";
    opt_act_2: "~ο~οις", "~ο~οιτον", _;
    opt_act_3: "~ο~οι", "~ο~οιτην", "~ο~οιεν";
    opt_mid_1: "~ο~οιμην", _, "~ο~οιμεθᾰ";
    opt_mid_2: "~ο~οιο", _, "~ο~οισθε";
    opt_mid_3: "~ο~οιτο", "~ο~οισθην", "~ο~οιντο";
    opt_pass_2: _, "~ο~οισθον", _;
    imp_act_2: "~ο~ε", _, "~ο~ετε";
    imp_act_3: _, _, "~ο~οντων";
    imp_mid_2: "~ο~ου", "~ο~εσθον", "~ο~εσθε";
    imp_mid_3: derived, _, "~ο~εσθων";
    imp_pass_2: _, "~ο~εσθον", "~ο~εσθε";
    imp_pass_3: "~ο~εσθω", "~ο~εσθων", "~ο~εσθων";
    imperfect_act_1: "~ε~ον", _, _;
    imperfect_act_2: "~ε~ες", "~ε~ετον", "~ε~ετε";
    imperfect_act_3: "~ε~ε", _, "~ε~ον";
    imperfect_mid_1: "~ε~ομην", _, "~ε~ομεθᾰ";
    imperfect_mid_2: "~ε~ου", derived, _;
    imperfect_mid_3: "~ε~ετο", _, "~ε~οντο";
    imperfect_pass_2: _, "~ε~εσθον", "~ε~εσθε";
    imperfect_pass_3: _, "~ε~εσθην", _;
    future_ind_act_1: "~ο~σω", _, "~ο~σομεν";
    future_ind_act_2: "~ο~σεις", "~ο~σετον", _;
    future_ind_act_3: _, _, "~ο~σουσῐ";
    future_ind_mid_1: "~ο~σομαι", _, _;
    future_ind_mid_2: "~ο~σῃ", "~ο~σεσθον", _;
    future_ind_mid_3: "~ο~σεται", _, _;
    future_opt_act_1: "~ο~σοιμῐ", _, "~ο~σοιμεν";
    future_opt_act_2: "~ο~σοις", "~ο~σοιτον", _;
    future_opt_act_3: "~ο~σοι", "~ο~σοιτην", _;
    future_opt_mid_1: "~ο~σοιμην", _, "~ο~σοιμεθᾰ";
    future_opt_mid_3: _, "~ο~σοισθην", _;
    aorist_subj_act_1: "~ο~σω", _, _;
    aorist_subj_act_2: "~ο~σῃς", "~ο~σητον", _;
    aorist_subj_mid_1: "~ο~σωμαι", _, _;
    aorist_subj_mid_2: "~ο~σῃ", "~ο~σησθον", _;
    aorist_opt_act_1: "~ο~σαιμῐ", _, "~ο~σαιμεν";
    aorist_opt_act_2: "~ο~σειᾰς", _, _;
    aorist_opt_act_3: "~ο~σειε", _, "~ο~σειᾰν";
    aorist_opt_mid_1: "~ο~σαιμην", _, _;
    aorist_imp_act_2: "~ο~σον", _, "~ο~σᾰτε";
    aorist_imp_act_3: _, _, "~ο~σᾰντων";
    aorist_imp_mid_2: "~ο~σαι", "~ο~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ο~σᾰσθων";
    perfect_ind_act_1: "~ο%ε~κᾰ", _, "~ο%ε~κᾰμεν";
    perfect_ind_act_2: _, "~ο%ε~κᾰτον", _;
    perfect_ind_act_3: _, _, "~ο%ε~κᾱσῐ";
    perfect_ind_mid_1: _, _, "~ο%ε~μεθᾰ";
    perfect_ind_mid_2: "~ο%ε~σαι", "~ο%ε~σθον", _;
    perfect_ind_mid_3: "~ο%ε~ται", _, "~ο%ε~νται";
    perfect_ind_pass_1: "~ο%ε~μαι", _, _;
    perfect_ind_pass_2: _, _, "~ο%ε~σθε";
    perfect_subj_act_1: "~ο%ε~κω", _, _;
    perfect_subj_act_2: "~ο%ε~κῃς", "~ο%ε~κητον", "~ο%ε~κητε";
    perfect_subj_act_3: "~ο%ε~κῃ", _, "~ο%ε~κωσῐ";
    perfect_imp_act_2: "~ο%ε~κε", _, _;
    perfect_imp_mid_2: _, _, "~ο%ε~σθε";
    perfect_imp_mid_3: _, _, "~ο%ε~σθων";
    perfect_imp_pass_2: "~ο%ε~σο", _, "~ο%ε~σθε";
    perfect_imp_pass_3: "~ο%ε~σθω", "~ο%ε~σθων", "~ο%ε~σθων";
    infinitive_pres: "~ο~ειν", _, "~ο~εσθαι";
    infinitive_fut: "~ο~σειν", _, _;
    infinitive_aor: "~ο~σαι", _, _;
    infinitive_perf: _, "~ο%ε~σθαι", _;
    participle_pres: "~ο~ων", _, "~ο~ομενος";
    participle_aor: "~ο~σᾱς", "~ο~σᾰμενος", _;
    participle_perf: "~ο%ε~κως", _, "~ο%ε~μενος";
    ),
    // Σκηπτω: 5 words (ῥᾰπτω, ῥῑπτω, σκᾰπτω, ρρῑπτω, ρρᾰπτω). Like Θρῠπτω, except in 54 cells: imperfect_mid_2 du, imperfect_pass_2 du, future_ind_pass_2 sg, aorist_subj_act_3 pl, aorist_subj_pass_3 pl, aorist_opt_act_2 sg, ….
    conjugation!("σκηπτω", 5, "~πτω", like Θρῠπτω;
    stem aorist_ind: "~ψ- like ἐχω";
    stem aorist_ind_pass: "~φην- like ἐχω";
    stem aorist_pass: "~φ- like ἐγειρω";
    imperfect_mid_2: _, "ἐ~πτεσθον", _;
    imperfect_pass_2: _, derived, _;
    future_ind_pass_2: derived, _, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_2: "~ψειᾰς", _, _;
    aorist_opt_act_3: "~ψειε", _, "~ψειᾰν";
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_1: "ἐ~φᾰ", _, "ἐ~φᾰμεν";
    perfect_ind_act_2: _, "ἐ~φᾰτον", _;
    perfect_ind_act_3: "ἐ~φε|ἐ~φεν", _, "ἐ~φᾱσῐ|ἐ~φᾱσῐν";
    perfect_ind_mid_1: _, _, "ἐ~μμεθᾰ";
    perfect_ind_mid_2: "ἐ~ψαι", "ἐ~φθον", _;
    perfect_ind_mid_3: "ἐ~πται", _, "ἐ~φᾰται";
    perfect_ind_pass_1: "ἐ~μμαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~φθε";
    perfect_subj_act_1: "ἐ~φω", _, _;
    perfect_subj_act_2: "ἐ~φῃς", "ἐ~φητον", "ἐ~φητε";
    perfect_subj_act_3: "ἐ~φῃ", _, "ἐ~φωσῐ|ἐ~φωσῐν";
    perfect_opt_act_1: "ἐ~φοιμῐ|ἐ~φοιην", _, _;
    perfect_opt_act_2: "ἐ~φοις|ἐ~φοιης", _, _;
    perfect_opt_act_3: "ἐ~φοι|ἐ~φοιη", _, _;
    perfect_imp_act_2: "ἐ~φε", _, _;
    perfect_imp_mid_2: _, _, "ἐ~φθε";
    perfect_imp_mid_3: _, _, "ἐ~φθων";
    perfect_imp_pass_2: "ἐ~ψο", _, "ἐ~φθε";
    perfect_imp_pass_3: "ἐ~φθω", "ἐ~φθων", "ἐ~φθων";
    pluperfect_act_1: "ἐ~φειν", _, _;
    pluperfect_mid_1: "ἐ~μμην", _, _;
    pluperfect_mid_2: "ἐ~ψο", "ἐ~φθον", _;
    pluperfect_mid_3: "ἐ~πτο", _, "ἐ~φᾰτο";
    pluperfect_pass_1: _, _, "ἐ~μμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~φθε";
    pluperfect_pass_3: _, "ἐ~φθην", _;
    infinitive_perf: _, "ἐ~φθαι", _;
    participle_perf: "ἐ~φως", _, "ἐ~μμενος";
    ),
    // Εἰδομαι: 5 words (εἰδομαι, ἐλδομαι, ἡδομαι, μηδομαι, φειδομαι). Like Εἰμῐ, except in 56 cells: ind_act_1 sg, ind_act_3 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ….
    conjugation!("εἰδομαι", 5, "~δομαι", like Εἰμῐ;
    stem future_pass: "~σθησ- like ἐχω";
    stem aorist_ind: "~σ- like μηκᾰομαι";
    stem aorist_ind_pass: "~σθην- like εὐλογεω";
    stem aorist_pass: "~σθ- like ἐχω";
    ind_act_1: "~δομαι", _, _;
    ind_act_3: derived, _, _;
    ind_mid_1: "~δομαι", _, "~δομεθᾰ";
    ind_mid_2: "~δῃ|~δει", _, _;
    ind_mid_3: _, "~δεσθον", _;
    ind_pass_2: _, "~δεσθον", "~δεσθε";
    ind_pass_3: "~δεται", "~δεσθον", "~δονται";
    subj_mid_1: "~δωμαι", _, "~δωμεθᾰ";
    subj_mid_2: "~δῃ", "~δησθον", _;
    subj_mid_3: "~δηται", _, "~δωνται";
    subj_pass_2: "~δῃ", _, "~δησθε";
    opt_mid_1: "~δοιμην", _, "~δοιμεθᾰ";
    opt_mid_2: "~δοιο", _, "~δοισθε";
    opt_mid_3: "~δοιτο", "~δοισθην", "~δοιντο";
    opt_pass_2: _, "~δοισθον", _;
    imp_mid_2: "~δου", "~δεσθον", "~δεσθε";
    imp_mid_3: "~δεσθω", _, "~δεσθων";
    imp_pass_2: _, "~δεσθον", "~δεσθε";
    imp_pass_3: _, "~δεσθων", "~δεσθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    imperfect_mid_1: "~δομην", _, "~δομεθᾰ";
    imperfect_mid_2: "~δου", "~δεσθον", _;
    imperfect_mid_3: "~δετο", _, "~δοντο";
    imperfect_pass_2: _, _, "~δεσθε";
    imperfect_pass_3: _, "~δεσθην", _;
    aorist_subj_mid_1: "~σωμαι", _, _;
    aorist_subj_mid_2: "~σῃ", "~σησθον", _;
    aorist_opt_mid_1: "~σαιμην", _, _;
    aorist_imp_mid_2: "~σαι", "~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~σᾰσθων";
    infinitive_pres: _, _, "~δεσθαι";
    participle_pres: _, _, "~δομενος";
    participle_aor: _, "~σᾰμενος", _;
    ),
    // Βᾰθῠνω: 5 words (βᾰρῠνω, βᾰθῠνω, πλᾰτῠνω, βρᾰδῠνω, βρᾰχῠνω). Like Ἀποκωλῡω, except in 142 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("βᾰθῠνω", 5, "~ᾰ~ῠνω", like Ἀποκωλῡω;
    stem aorist_ind: "~ᾰ~ῡν- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~ῡνθην- like ἐχω";
    stem aorist_pass: "~ᾰ~ῡνθ- like ἐχω";
    ind_act_1: "~ᾰ~ῠνω", _, "~ᾰ~ῠνομεν";
    ind_act_2: "~ᾰ~ῠνεις", "~ᾰ~ῠνετον", _;
    ind_act_3: "~ᾰ~ῠνει", _, "~ᾰ~ῠνουσῐ";
    ind_mid_1: _, _, "~ᾰ~ῠνομεθᾰ";
    ind_mid_2: "~ᾰ~ῠνῃ", _, _;
    ind_mid_3: _, "~ᾰ~ῠνεσθον", _;
    ind_pass_1: "~ᾰ~ῠνομαι", _, _;
    ind_pass_2: _, "~ᾰ~ῠνεσθον", "~ᾰ~ῠνεσθε";
    ind_pass_3: "~ᾰ~ῠνεται", "~ᾰ~ῠνεσθον", "~ᾰ~ῠνονται";
    subj_act_1: "~ᾰ~ῠνω", _, "~ᾰ~ῠνωμεν";
    subj_act_2: "~ᾰ~ῠνῃς", _, _;
    subj_act_3: "~ᾰ~ῠνῃ", "~ᾰ~ῠνητον", "~ᾰ~ῠνωσῐ";
    subj_mid_1: _, _, "~ᾰ~ῠνωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ῠνησθον", _;
    subj_mid_3: "~ᾰ~ῠνηται", _, "~ᾰ~ῠνωνται";
    subj_pass_1: "~ᾰ~ῠνωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~ῠνησθε";
    opt_act_1: "~ᾰ~ῠνοιμῐ", _, "~ᾰ~ῠνοιμεν";
    opt_act_2: "~ᾰ~ῠνοις", "~ᾰ~ῠνοιτον", _;
    opt_act_3: "~ᾰ~ῠνοι", "~ᾰ~ῠνοιτην", "~ᾰ~ῠνοιεν";
    opt_mid_1: "~ᾰ~ῠνοιμην", _, "~ᾰ~ῠνοιμεθᾰ";
    opt_mid_2: "~ᾰ~ῠνοιο", _, "~ᾰ~ῠνοισθε";
    opt_mid_3: "~ᾰ~ῠνοιτο", "~ᾰ~ῠνοισθην", "~ᾰ~ῠνοιντο";
    opt_pass_2: _, "~ᾰ~ῠνοισθον", _;
    imp_act_2: "~ᾰ~ῠνε", _, "~ᾰ~ῠνετε";
    imp_act_3: _, _, "~ᾰ~ῠνοντων";
    imp_mid_2: "~ᾰ~ῠνου", "~ᾰ~ῠνεσθον", "~ᾰ~ῠνεσθε";
    imp_mid_3: _, _, "~ᾰ~ῠνεσθων";
    imp_pass_2: _, "~ᾰ~ῠνεσθον", "~ᾰ~ῠνεσθε";
    imp_pass_3: "~ᾰ~ῠνεσθω", "~ᾰ~ῠνεσθων", "~ᾰ~ῠνεσθων";
    imperfect_act_1: "ἐ~ᾰ~ῠνον", _, _;
    imperfect_act_2: "ἐ~ᾰ~ῠνες", "ἐ~ᾰ~ῠνετον", "ἐ~ᾰ~ῠνετε";
    imperfect_act_3: "ἐ~ᾰ~ῠνε", _, "ἐ~ᾰ~ῠνον";
    imperfect_mid_1: "ἐ~ᾰ~ῠνομην", _, "ἐ~ᾰ~ῠνομεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~ῠνου", _, _;
    imperfect_mid_3: "ἐ~ᾰ~ῠνετο", _, "ἐ~ᾰ~ῠνοντο";
    imperfect_pass_2: _, "ἐ~ᾰ~ῠνεσθον", "ἐ~ᾰ~ῠνεσθε";
    imperfect_pass_3: _, "ἐ~ᾰ~ῠνεσθην", _;
    future_ind_act_1: "~ᾰ~ῠνω", _, "~ᾰ~ῠνουμεν";
    future_ind_act_2: "~ᾰ~ῠνεις", "~ᾰ~ῠνειτον", _;
    future_ind_act_3: _, _, "~ᾰ~ῠνουσῐ";
    future_ind_mid_1: "~ᾰ~ῠνουμαι", _, _;
    future_ind_mid_2: "~ᾰ~ῠνει", "~ᾰ~ῠνεισθον", _;
    future_ind_mid_3: "~ᾰ~ῠνειται", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: "~ᾰ~ῠνοιην", _, "~ᾰ~ῠνοιμεν";
    future_opt_act_2: "~ᾰ~ῠνοιης", "~ᾰ~ῠνοιτον", _;
    future_opt_act_3: "~ᾰ~ῠνοιη", "~ᾰ~ῠνοιτην", _;
    future_opt_mid_1: "~ᾰ~ῠνοιμην", _, "~ᾰ~ῠνοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~ῠνοισθην", _;
    aorist_subj_act_1: "~ᾰ~ῡνω", _, _;
    aorist_subj_act_2: "~ᾰ~ῡνῃς", "~ᾰ~ῡνητον", _;
    aorist_subj_mid_1: "~ᾰ~ῡνωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ῡνῃ", "~ᾰ~ῡνησθον", _;
    aorist_opt_act_1: "~ᾰ~ῡναιμῐ", _, "~ᾰ~ῡναιμεν";
    aorist_opt_act_2: "~ᾰ~ῡνειᾰς", _, _;
    aorist_opt_act_3: "~ᾰ~ῡνειε", _, "~ᾰ~ῡνειᾰν";
    aorist_opt_mid_1: "~ᾰ~ῡναιμην", _, _;
    aorist_imp_act_2: "~ᾰ~ῡνον", _, "~ᾰ~ῡνᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ῡνᾰντων";
    aorist_imp_mid_2: "~ᾰ~ῡναι", "~ᾰ~ῡνᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ῡνᾰσθων";
    perfect_ind_act_1: "~εβᾱ~ῠγκᾰ", _, "~εβᾱ~ῠγκᾰμεν";
    perfect_ind_act_2: _, "~εβᾱ~ῠγκᾰτον", _;
    perfect_ind_act_3: _, _, "~εβᾱ~ῠγκᾱσῐ";
    perfect_ind_mid_1: _, _, "~εβᾰ~ῡσμεθᾰ";
    perfect_ind_mid_2: "~εβᾰ~ῡσαι", "~εβᾰ~ῡσθον", _;
    perfect_ind_mid_3: "~εβᾰ~ῡσται", _, "~εβᾰ~ῡσᾰται";
    perfect_ind_pass_1: "~εβᾰ~ῡσμαι", _, _;
    perfect_ind_pass_2: _, _, "~εβᾰ~ῡσθε";
    perfect_subj_act_1: "~εβᾱ~ῠγκω", _, _;
    perfect_subj_act_2: "~εβᾱ~ῠγκῃς", "~εβᾱ~ῠγκητον", "~εβᾱ~ῠγκητε";
    perfect_subj_act_3: "~εβᾱ~ῠγκῃ", _, "~εβᾱ~ῠγκωσῐ";
    perfect_imp_act_2: "~εβᾱ~ῠγκε", _, _;
    perfect_imp_mid_2: _, _, "~εβᾰ~ῡσθε";
    perfect_imp_mid_3: _, _, "~εβᾰ~ῡσθων";
    perfect_imp_pass_2: "~εβᾰ~ῡσο", _, "~εβᾰ~ῡσθε";
    perfect_imp_pass_3: "~εβᾰ~ῡσθω", "~εβᾰ~ῡσθων", "~εβᾰ~ῡσθων";
    infinitive_pres: "~ᾰ~ῠνειν", _, "~ᾰ~ῠνεσθαι";
    infinitive_fut: "~ᾰ~ῠνειν", _, _;
    infinitive_aor: "~ᾰ~ῡναι", _, _;
    infinitive_perf: _, "~εβᾰ~ῡσθαι", _;
    participle_pres: "~ᾰ~ῠνων", _, "~ᾰ~ῠνομενος";
    participle_aor: "~ᾰ~ῡνᾱς", "~ᾰ~ῡνᾰμενος", _;
    participle_perf: "~εβᾱ~ῠγκως", _, "~εβᾰ~ῡσμενος";
    ),
    // Ἐξᾰκολουθεω: 5 words (πᾰρᾰμελεω, σῠνᾰδῐκεω, ξῠνᾰδῐκεω, ἀπᾰσχολεω, ἐξᾰκολουθεω). Like Λᾰγοθηρεω, except in 54 cells: imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, imperfect_act_3 pl, ….
    conjugation!("ἐξᾰκολουθεω", 5, "~ᾰ~εω", like Λᾰγοθηρεω;
    stem aorist_ind: "~η~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~η~ηθην- like εὐλογεω";
    stem pluperfect_act: "~η~ηκ- like εὐλογεω";
    imperfect_act_1: "~η~ουν", _, _;
    imperfect_act_2: "~η~εις", "~η~ειτον", "~η~ειτε";
    imperfect_act_3: "~η~ει", _, "~η~ουν";
    imperfect_mid_1: "~η~ουμην", _, "~η~ουμεθᾰ";
    imperfect_mid_2: "~η~ου", _, _;
    imperfect_mid_3: "~η~ειτο", _, "~η~ουντο";
    imperfect_pass_2: _, "~η~εισθον", "~η~εισθε";
    imperfect_pass_3: _, "~η~εισθην", _;
    perfect_ind_act_1: "~η~ηκᾰ", _, "~η~ηκᾰμεν";
    perfect_ind_act_2: _, "~η~ηκᾰτον", _;
    perfect_ind_act_3: "~η~ηκε|~η~ηκεν", _, "~η~ηκᾱσῐ|~η~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~η~ημεθᾰ";
    perfect_ind_mid_2: "~η~ησαι", "~η~ησθον", _;
    perfect_ind_mid_3: "~η~ηται", _, "~η~ηνται";
    perfect_ind_pass_1: "~η~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~η~ησθε";
    perfect_subj_act_1: "~η~ηκω", _, _;
    perfect_subj_act_2: "~η~ηκῃς", "~η~ηκητον", "~η~ηκητε";
    perfect_subj_act_3: "~η~ηκῃ", _, "~η~ηκωσῐ|~η~ηκωσῐν";
    perfect_opt_act_1: "~η~ηκοιμῐ|~η~ηκοιην", _, _;
    perfect_opt_act_2: "~η~ηκοις|~η~ηκοιης", _, _;
    perfect_opt_act_3: "~η~ηκοι|~η~ηκοιη", _, _;
    perfect_imp_act_2: "~η~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~η~ησθε";
    perfect_imp_mid_3: _, _, "~η~ησθων";
    perfect_imp_pass_2: "~η~ησο", _, "~η~ησθε";
    perfect_imp_pass_3: "~η~ησθω", "~η~ησθων", "~η~ησθων";
    pluperfect_mid_1: "~η~ημην", _, _;
    pluperfect_mid_2: "~η~ησο", "~η~ησθον", _;
    pluperfect_mid_3: "~η~ητο", _, "~η~ηντο";
    pluperfect_pass_1: _, _, "~η~ημεθᾰ";
    pluperfect_pass_2: _, _, "~η~ησθε";
    pluperfect_pass_3: _, "~η~ησθην", _;
    infinitive_perf: _, "~η~ησθαι", _;
    participle_perf: "~η~ηκως", _, "~η~ημενος";
    ),
    // Προκᾰλεω: 5 words (προσκᾰλεω, σῠγκᾰλεω, προκᾰλεω, ἐγκᾰλεω, ἐκκᾰλεω).
    conjugation!("προκᾰλεω", 5, "~~ᾰ~εω";
    stem future_pass: "~~~ηθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~υ~ᾰ~εσ- like ἐλπω";
    stem aorist_ind_pass: "~υ~~ηθην- like εὐλογεω";
    stem aorist_pass: "~~~ηθ- like ποιεω";
    ind_act_1: "~~ᾰ~ω", _, "~~ᾰ~ουμεν";
    ind_act_2: "~~ᾰ~εις", "~~ᾰ~ειτον", _;
    ind_act_3: "~~ᾰ~ει", _, "~~ᾰ~ουσῐ|~~ᾰ~ουσῐν";
    ind_mid_1: _, _, "~~ᾰ~ουμεθᾰ";
    ind_mid_2: "~~ᾰ~ει|~~ᾰ~ῃ", _, _;
    ind_mid_3: _, "~~ᾰ~εισθον", _;
    ind_pass_1: "~~ᾰ~ουμαι", _, _;
    ind_pass_2: _, "~~ᾰ~εισθον", "~~ᾰ~εισθε";
    ind_pass_3: "~~ᾰ~ειται", "~~ᾰ~εισθον", "~~ᾰ~ουνται";
    subj_act_1: "~~ᾰ~ω", _, "~~ᾰ~ωμεν";
    subj_act_2: "~~ᾰ~ῃς", _, _;
    subj_act_3: "~~ᾰ~ῃ", "~~ᾰ~ητον", "~~ᾰ~ωσῐ|~~ᾰ~ωσῐν";
    subj_mid_1: _, _, "~~ᾰ~ωμεθᾰ";
    subj_mid_2: _, "~~ᾰ~ησθον", _;
    subj_mid_3: "~~ᾰ~ηται", _, "~~ᾰ~ωνται";
    subj_pass_1: "~~ᾰ~ωμαι", _, _;
    subj_pass_2: _, _, "~~ᾰ~ησθε";
    opt_act_1: "~~ᾰ~οιην|~~ᾰ~οιμῐ", _, "~~ᾰ~οιμεν|~~ᾰ~οιημεν";
    opt_act_2: "~~ᾰ~οιης|~~ᾰ~οις", "~~ᾰ~οιτον|~~ᾰ~οιητον", "~~ᾰ~οιτε|~~ᾰ~οιητε";
    opt_act_3: "~~ᾰ~οιη|~~ᾰ~οι", "~~ᾰ~οιτην|~~ᾰ~οιητην", "~~ᾰ~οιεν|~~ᾰ~οιησᾰν";
    opt_mid_1: "~~ᾰ~οιμην", _, "~~ᾰ~οιμεθᾰ";
    opt_mid_2: "~~ᾰ~οιο", _, "~~ᾰ~οισθε";
    opt_mid_3: "~~ᾰ~οιτο", "~~ᾰ~οισθην", "~~ᾰ~οιντο";
    opt_pass_2: _, "~~ᾰ~οισθον", _;
    imp_act_2: "~~ᾰ~ει", _, "~~ᾰ~ειτε";
    imp_act_3: _, _, "~~ᾰ~ουντων";
    imp_mid_2: "~~ᾰ~ου", "~~ᾰ~εισθον", "~~ᾰ~εισθε";
    imp_mid_3: _, _, "~~ᾰ~εισθων";
    imp_pass_2: _, "~~ᾰ~εισθον", "~~ᾰ~εισθε";
    imp_pass_3: "~~ᾰ~εισθω", "~~ᾰ~εισθων", "~~ᾰ~εισθων";
    imperfect_act_1: "~υ~ᾰ~ουν", _, _;
    imperfect_act_2: "~υ~ᾰ~εις", "~υ~ᾰ~ειτον", "~υ~ᾰ~ειτε";
    imperfect_act_3: "~υ~ᾰ~ει", _, "~υ~ᾰ~ουν";
    imperfect_mid_1: "~υ~ᾰ~ουμην", _, "~υ~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "~υ~ᾰ~ου", _, _;
    imperfect_mid_3: "~υ~ᾰ~ειτο", _, "~υ~ᾰ~ουντο";
    imperfect_pass_2: _, "~υ~ᾰ~εισθον", "~υ~ᾰ~εισθε";
    imperfect_pass_3: _, "~υ~ᾰ~εισθην", _;
    future_ind_act_1: "~~ᾰ~εσω", _, "~~ᾰ~εσομεν";
    future_ind_act_2: "~~ᾰ~εσεις", "~~ᾰ~εσετον", _;
    future_ind_act_3: _, _, "~~ᾰ~εσουσῐ|~~ᾰ~εσουσῐν";
    future_ind_mid_1: "~~ᾰ~εσομαι", _, _;
    future_ind_mid_2: "~~ᾰ~εσῃ|~~ᾰ~εσει", "~~ᾰ~εσεσθον", _;
    future_ind_mid_3: "~~ᾰ~εσεται", _, _;
    future_opt_act_1: "~~ᾰ~εσοιμῐ", _, "~~ᾰ~εσοιμεν";
    future_opt_act_2: "~~ᾰ~εσοις", "~~ᾰ~εσοιτον", _;
    future_opt_act_3: "~~ᾰ~εσοι", "~~ᾰ~εσοιτην", _;
    future_opt_mid_1: "~~ᾰ~εσοιμην", _, "~~ᾰ~εσοιμεθᾰ";
    future_opt_mid_3: _, "~~ᾰ~εσοισθην", _;
    aorist_subj_act_1: "~~ᾰ~εσω", _, _;
    aorist_subj_act_2: "~~ᾰ~εσῃς", "~~ᾰ~εσητον", _;
    aorist_subj_act_3: _, _, "~~ᾰ~εσωσῐ|~~ᾰ~εσωσῐν";
    aorist_subj_mid_1: "~~ᾰ~εσωμαι", _, _;
    aorist_subj_mid_2: "~~ᾰ~εσῃ", "~~ᾰ~εσησθον", _;
    aorist_opt_act_1: "~~ᾰ~εσαιμῐ", _, "~~ᾰ~εσαιμεν";
    aorist_opt_act_2: "~~ᾰ~εσειᾰς|~~ᾰ~εσαις", _, _;
    aorist_opt_act_3: "~~ᾰ~εσειε|~~ᾰ~εσειεν|~~ᾰ~εσαι", _, "~~ᾰ~εσειᾰν|~~ᾰ~εσαιεν";
    aorist_opt_mid_1: "~~ᾰ~εσαιμην", _, _;
    aorist_imp_act_2: "~~ᾰ~εσον", _, "~~ᾰ~εσᾰτε";
    aorist_imp_act_3: _, _, "~~ᾰ~εσᾰντων";
    aorist_imp_mid_2: "~~ᾰ~εσαι", "~~ᾰ~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~ᾰ~εσᾰσθων";
    perfect_ind_act_1: "~~εκ~ηκᾰ", _, "~~εκ~ηκᾰμεν";
    perfect_ind_act_2: _, "~~εκ~ηκᾰτον", _;
    perfect_ind_act_3: "~~εκ~ηκε|~~εκ~ηκεν", _, "~~εκ~ηκᾱσῐ|~~εκ~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~~εκ~ημεθᾰ";
    perfect_ind_mid_2: "~~εκ~ησαι", "~~εκ~ησθον", _;
    perfect_ind_mid_3: "~~εκ~ηται", _, "~~εκ~ηνται";
    perfect_ind_pass_1: "~~εκ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~~εκ~ησθε";
    perfect_subj_act_1: "~~εκ~ηκω", _, _;
    perfect_subj_act_2: "~~εκ~ηκῃς", "~~εκ~ηκητον", "~~εκ~ηκητε";
    perfect_subj_act_3: "~~εκ~ηκῃ", _, "~~εκ~ηκωσῐ|~~εκ~ηκωσῐν";
    perfect_opt_act_1: "~~εκ~ηκοιμῐ|~~εκ~ηκοιην", _, _;
    perfect_opt_act_2: "~~εκ~ηκοις|~~εκ~ηκοιης", _, _;
    perfect_opt_act_3: "~~εκ~ηκοι|~~εκ~ηκοιη", _, _;
    perfect_imp_act_2: "~~εκ~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~~εκ~ησθε";
    perfect_imp_mid_3: _, _, "~~εκ~ησθων";
    perfect_imp_pass_2: "~~εκ~ησο", _, "~~εκ~ησθε";
    perfect_imp_pass_3: "~~εκ~ησθω", "~~εκ~ησθων", "~~εκ~ησθων";
    pluperfect_act_1: "~υ%ε~~ηκειν", _, _;
    pluperfect_mid_1: "~υ%ε~~ημην", _, _;
    pluperfect_mid_2: "~υ%ε~~ησο", "~υ%ε~~ησθον", _;
    pluperfect_mid_3: "~υ%ε~~ητο", _, "~υ%ε~~ηντο";
    pluperfect_pass_1: _, _, "~υ%ε~~ημεθᾰ";
    pluperfect_pass_2: _, _, "~υ%ε~~ησθε";
    pluperfect_pass_3: _, "~υ%ε~~ησθην", _;
    infinitive_pres: "~~ᾰ~ειν", _, "~~ᾰ~εισθαι";
    infinitive_fut: "~~ᾰ~εσειν", _, _;
    infinitive_aor: "~~ᾰ~εσαι", _, _;
    infinitive_perf: _, "~~εκ~ησθαι", _;
    participle_pres: "~~ᾰ~ων", _, "~~ᾰ~ουμενος";
    participle_aor: "~~ᾰ~εσᾱς", "~~ᾰ~εσᾰμενος", _;
    participle_perf: "~~εκ~ηκως", _, "~~εκ~ημενος";
    ),
    // Περαινω: 5 words (κραινω, ὑγῐαινω, ὑδραινω, περαινω, τετραινω). Like Μωραινω, except in 38 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_3 pl, future_ind_mid_1 sg, ….
    conjugation!("περαινω", 5, "~αινω", like Μωραινω;
    stem future_pass: "~ᾰνθησ- like ἐχω";
    stem aorist_ind: "~ᾱν- like ποιεω";
    stem aorist_ind_pass: "~ᾰνθην- like ἐχω";
    stem aorist_pass: "~ᾰνθ- like ἐχω";
    future_ind_act_1: "~ᾰνω", _, "~ᾰνουμεν";
    future_ind_act_2: "~ᾰνεις", "~ᾰνειτον", _;
    future_ind_act_3: _, _, "~ᾰνουσῐ|~ᾰνουσῐν";
    future_ind_mid_1: "~ᾰνουμαι", _, _;
    future_ind_mid_2: "~ᾰνει|~ᾰνῃ", "~ᾰνεισθον", _;
    future_ind_mid_3: "~ᾰνειται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~ᾰνοιην|~ᾰνοιμῐ", _, "~ᾰνοιμεν|~ᾰνοιημεν";
    future_opt_act_2: "~ᾰνοιης|~ᾰνοις", "~ᾰνοιτον|~ᾰνοιητον", "~ᾰνοιτε|~ᾰνοιητε";
    future_opt_act_3: "~ᾰνοιη|~ᾰνοι", "~ᾰνοιτην|~ᾰνοιητην", "~ᾰνοιεν|~ᾰνοιησᾰν";
    future_opt_mid_1: "~ᾰνοιμην", _, "~ᾰνοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰνοισθην", _;
    aorist_subj_act_3: _, _, "~ᾱνωσῐ|~ᾱνωσῐν";
    aorist_opt_act_2: "~ᾱνειᾰς|~ᾱναις", _, _;
    aorist_opt_act_3: "~ᾱνειε|~ᾱνειεν|~ᾱναι", _, "~ᾱνειᾰν|~ᾱναιεν";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_imp_act_2: derived, _, _;
    infinitive_fut: "~ᾰνειν", _, _;
    participle_perf: derived, _, _;
    ),
    // Ἡγεομαι: 4 words (ἡγεομαι, νεομαι, ἐξηγεομαι, εὐνομεομαι). Like Οἰομαι, except in 52 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἡγεομαι", 4, "~εομαι", like Οἰομαι;
    stem future_pass: "~ηθησ- like ἐχω";
    stem aorist_ind: "~ησ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ηθην- like εὐλογεω";
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
    imperfect_mid_1: "~ουμην", _, "~ουμεθᾰ";
    imperfect_mid_2: "~ου", "~εισθον", _;
    imperfect_mid_3: "~ειτο", _, "~ουντο";
    imperfect_pass_2: _, _, "~εισθε";
    imperfect_pass_3: _, "~εισθην", _;
    aorist_subj_mid_1: "~ησωμαι", _, _;
    aorist_subj_mid_2: "~ησῃ", "~ησησθον", _;
    aorist_opt_mid_1: "~ησαιμην", _, _;
    aorist_imp_mid_2: "~ησαι", "~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ησᾰσθων";
    perfect_ind_mid_1: _, _, "~ημεθᾰ";
    perfect_ind_mid_2: "~ησαι", "~ησθον", _;
    perfect_ind_mid_3: "~ηται", _, "~ηνται";
    perfect_ind_pass_1: "~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ησθε";
    perfect_imp_mid_2: _, _, "~ησθε";
    perfect_imp_mid_3: _, _, "~ησθων";
    perfect_imp_pass_2: "~ησο", _, "~ησθε";
    perfect_imp_pass_3: "~ησθω", "~ησθων", "~ησθων";
    infinitive_pres: _, _, "~εισθαι";
    infinitive_perf: _, "~ησθαι", _;
    participle_pres: _, _, "~ουμενος";
    participle_aor: _, "~ησᾰμενος", _;
    participle_perf: _, _, "~ημενος";
    ),
    // Βαινω: 4 words (βαινω, ἐπεκβαινω, εἰσᾰνᾰβαινω, ἐπαλθαινω). Like Φαινω, except in 76 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_3 pl, future_ind_mid_1 sg, ….
    conjugation!("βαινω", 4, "~αινω", like Φαινω;
    stem future_pass: "~ηθησ- like ἐχω";
    stem aorist_ind: "~η- like βαινω";
    stem aorist_ind_pass: "~ᾰθην- like ἐχω";
    stem aorist_pass: "~ᾰθ- like ἐχω";
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", _;
    future_ind_act_3: _, _, "~ησουσῐ";
    future_ind_mid_1: "~ησομαι", _, _;
    future_ind_mid_2: "~ησῃ", "~ησεσθον", _;
    future_ind_mid_3: "~ησεται", _, _;
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: "~ησοις", "~ησοιτον", _;
    future_opt_act_3: "~ησοι", "~ησοιτην", _;
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ησοισθην", _;
    aorist_subj_act_1: "~ω", _, _;
    aorist_subj_act_2: "~ῃς", "~ητον", _;
    aorist_subj_mid_1: "~ησωμαι", _, _;
    aorist_subj_mid_2: "~ησῃ", "~ησησθον", _;
    aorist_opt_act_1: "~αιην", _, "~αιμεν";
    aorist_opt_act_2: "~αιης", _, _;
    aorist_opt_act_3: "~αιη", _, "~αιεν";
    aorist_opt_mid_1: "~ησαιμην", _, _;
    aorist_imp_act_2: "~ηθῐ", _, "~ητε";
    aorist_imp_act_3: _, _, "~ᾰντων";
    aorist_imp_mid_2: "~ησαι", "~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ησᾰσθων";
    perfect_ind_act_1: "~εβηκᾰ", _, "~εβηκᾰμεν";
    perfect_ind_act_2: _, "~εβηκᾰτον", _;
    perfect_ind_act_3: _, _, "~εβηκᾱσῐ";
    perfect_ind_mid_1: _, _, "~εβᾰμεθᾰ";
    perfect_ind_mid_2: "~εβᾰσαι", "~εβᾰσθον", _;
    perfect_ind_mid_3: "~εβᾰται", _, "~εβᾰνται";
    perfect_ind_pass_1: "~εβᾰμαι", _, _;
    perfect_ind_pass_2: _, _, "~εβᾰσθε";
    perfect_subj_act_1: "~εβηκω", _, _;
    perfect_subj_act_2: "~εβηκῃς", "~εβηκητον", "~εβηκητε";
    perfect_subj_act_3: "~εβηκῃ", _, "~εβηκωσῐ";
    perfect_imp_act_2: "~εβηκε", _, _;
    perfect_imp_mid_2: _, _, "~εβᾰσθε";
    perfect_imp_mid_3: _, _, "~εβᾰσθων";
    perfect_imp_pass_2: "~εβᾰσο", _, "~εβᾰσθε";
    perfect_imp_pass_3: "~εβᾰσθω", "~εβᾰσθων", "~εβᾰσθων";
    pluperfect_act_1: "ἐ~εβηκειν", _, _;
    pluperfect_mid_1: "ἐ~εβᾰμην", _, _;
    pluperfect_mid_2: "ἐ~εβᾰσο", "ἐ~εβᾰσθον", _;
    pluperfect_mid_3: "ἐ~εβᾰτο", _, "ἐ~εβᾰντο";
    pluperfect_pass_1: _, _, "ἐ~εβᾰμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~εβᾰσθε";
    pluperfect_pass_3: _, "ἐ~εβᾰσθην", _;
    infinitive_fut: "~ησειν", _, _;
    infinitive_perf: _, "~εβᾰσθαι", _;
    participle_aor: "~ᾱς", "~ησᾰμενος", _;
    participle_perf: "~εβηκως", _, "~εβᾰμενος";
    ),
    // Τῠπτω: 4 words (κοπτω, τῠπτω, βλᾰπτω, σκωπτω). Like Θρῠπτω, except in 51 cells: future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, aorist_opt_pass_3 du, ….
    conjugation!("τῠπτω", 4, "~πτω", like Θρῠπτω;
    stem future_pass: "~πησ- like ἐχω";
    stem aorist_ind_pass: "~πην- like ἐχω";
    stem aorist_pass: "~π- like ἐγειρω";
    stem pluperfect_act: "~φ- like χρημᾰτῐζω";
    future_ind_pass_2: derived, _, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_1: "%ε~φᾰ", _, "%ε~φᾰμεν";
    perfect_ind_act_2: _, "%ε~φᾰτον", _;
    perfect_ind_act_3: "%ε~φε|%ε~φεν", _, "%ε~φᾱσῐ|%ε~φᾱσῐν";
    perfect_ind_mid_3: _, _, "%ε~πᾰται";
    perfect_subj_act_1: "%ε~φω", _, _;
    perfect_subj_act_2: "%ε~φῃς", "%ε~φητον", "%ε~φητε";
    perfect_subj_act_3: "%ε~φῃ", _, "%ε~φωσῐ|%ε~φωσῐν";
    perfect_opt_act_1: "%ε~φοιμῐ|%ε~φοιην", _, _;
    perfect_opt_act_2: "%ε~φοις|%ε~φοιης", _, _;
    perfect_opt_act_3: "%ε~φοι|%ε~φοιη", _, _;
    perfect_imp_act_2: "%ε~φε", _, _;
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
    participle_perf: "%ε~φως", _, _;
    participle_futp: _, _, "%ε~ψομενος";
    ),
    // Ἐπᾰγγελλω: 4 words (πᾰρᾰγγελλω, ἀπᾰγγελλω, ἐπᾰγγελλω, ἀνᾰγγελλω). Like Προκᾰλεω, except in 157 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐπᾰγγελλω", 4, "~ᾰ~~λω", like Προκᾰλεω;
    stem future_pass: "~ᾰ~~θησ- like χρημᾰτῐζω";
    stem aorist_ind: "~η~ι~- like εὐλογεω";
    stem aorist_ind_pass: "~η~~θην- like εὐλογεω";
    stem aorist_pass: "~ᾰ~~θ- like ποιεω";
    ind_act_1: "~ᾰ~~λω", _, "~ᾰ~~λομεν";
    ind_act_2: "~ᾰ~~λεις", "~ᾰ~~λετον", _;
    ind_act_3: "~ᾰ~~λει", _, "~ᾰ~~λουσῐ|~ᾰ~~λουσῐν";
    ind_mid_1: _, _, "~ᾰ~~λομεθᾰ";
    ind_mid_2: "~ᾰ~~λῃ|~ᾰ~~λει", _, _;
    ind_mid_3: _, "~ᾰ~~λεσθον", _;
    ind_pass_1: "~ᾰ~~λομαι", _, _;
    ind_pass_2: _, "~ᾰ~~λεσθον", "~ᾰ~~λεσθε";
    ind_pass_3: "~ᾰ~~λεται", "~ᾰ~~λεσθον", "~ᾰ~~λονται";
    subj_act_1: "~ᾰ~~λω", _, "~ᾰ~~λωμεν";
    subj_act_2: "~ᾰ~~λῃς", _, _;
    subj_act_3: "~ᾰ~~λῃ", "~ᾰ~~λητον", "~ᾰ~~λωσῐ|~ᾰ~~λωσῐν";
    subj_mid_1: _, _, "~ᾰ~~λωμεθᾰ";
    subj_mid_2: _, "~ᾰ~~λησθον", _;
    subj_mid_3: "~ᾰ~~ληται", _, "~ᾰ~~λωνται";
    subj_pass_1: "~ᾰ~~λωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~~λησθε";
    opt_act_1: "~ᾰ~~λοιμῐ", _, "~ᾰ~~λοιμεν";
    opt_act_2: "~ᾰ~~λοις", "~ᾰ~~λοιτον", derived;
    opt_act_3: "~ᾰ~~λοι", "~ᾰ~~λοιτην", "~ᾰ~~λοιεν";
    opt_mid_1: "~ᾰ~~λοιμην", _, "~ᾰ~~λοιμεθᾰ";
    opt_mid_2: "~ᾰ~~λοιο", _, "~ᾰ~~λοισθε";
    opt_mid_3: "~ᾰ~~λοιτο", "~ᾰ~~λοισθην", "~ᾰ~~λοιντο";
    opt_pass_2: _, "~ᾰ~~λοισθον", _;
    imp_act_2: "~ᾰ~~λε", _, "~ᾰ~~λετε";
    imp_act_3: _, _, "~ᾰ~~λοντων";
    imp_mid_2: "~ᾰ~~λου", "~ᾰ~~λεσθον", "~ᾰ~~λεσθε";
    imp_mid_3: _, _, "~ᾰ~~λεσθων";
    imp_pass_2: _, "~ᾰ~~λεσθον", "~ᾰ~~λεσθε";
    imp_pass_3: "~ᾰ~~λεσθω", "~ᾰ~~λεσθων", "~ᾰ~~λεσθων";
    imperfect_act_1: "~η~~λον", _, _;
    imperfect_act_2: "~η~~λες", "~η~~λετον", "~η~~λετε";
    imperfect_act_3: "~η~~λε|~η~~λεν", _, "~η~~λον";
    imperfect_mid_1: "~η~~λομην", _, "~η~~λομεθᾰ";
    imperfect_mid_2: "~η~~λου", _, _;
    imperfect_mid_3: "~η~~λετο", _, "~η~~λοντο";
    imperfect_pass_2: _, "~η~~λεσθον", "~η~~λεσθε";
    imperfect_pass_3: _, "~η~~λεσθην", _;
    future_ind_act_1: "~ᾰ~~ω", _, "~ᾰ~~ουμεν";
    future_ind_act_2: "~ᾰ~~εις", "~ᾰ~~ειτον", _;
    future_ind_act_3: _, _, "~ᾰ~~ουσῐ|~ᾰ~~ουσῐν";
    future_ind_mid_1: "~ᾰ~~ουμαι", _, _;
    future_ind_mid_2: "~ᾰ~~ει|~ᾰ~~ῃ", "~ᾰ~~εισθον", _;
    future_ind_mid_3: "~ᾰ~~ειται", _, _;
    future_opt_act_1: "~ᾰ~~οιην|~ᾰ~~οιμῐ", _, "~ᾰ~~οιμεν|~ᾰ~~οιημεν";
    future_opt_act_2: "~ᾰ~~οιης|~ᾰ~~οις", "~ᾰ~~οιτον|~ᾰ~~οιητον", "~ᾰ~~οιτε|~ᾰ~~οιητε";
    future_opt_act_3: "~ᾰ~~οιη|~ᾰ~~οι", "~ᾰ~~οιτην|~ᾰ~~οιητην", "~ᾰ~~οιεν|~ᾰ~~οιησᾰν";
    future_opt_mid_1: "~ᾰ~~οιμην", _, "~ᾰ~~οιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~~οισθην", _;
    aorist_subj_act_1: "~ᾰ~ι~ω", _, _;
    aorist_subj_act_2: "~ᾰ~ι~ῃς", "~ᾰ~ι~ητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~ι~ωσῐ|~ᾰ~ι~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ι~ωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ι~ῃ", "~ᾰ~ι~ησθον", _;
    aorist_opt_act_1: "~ᾰ~ι~αιμῐ", _, "~ᾰ~ι~αιμεν";
    aorist_opt_act_2: "~ᾰ~ι~ειᾰς|~ᾰ~ι~αις", _, _;
    aorist_opt_act_3: "~ᾰ~ι~ειε|~ᾰ~ι~ειεν|~ᾰ~ι~αι", _, "~ᾰ~ι~ειᾰν|~ᾰ~ι~αιεν";
    aorist_opt_mid_1: "~ᾰ~ι~αιμην", _, _;
    aorist_imp_act_2: "~ᾰ~ι~ον", _, "~ᾰ~ι~ᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ι~ᾰντων";
    aorist_imp_mid_2: "~ᾰ~ι~αι", "~ᾰ~ι~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ι~ᾰσθων";
    perfect_ind_act_1: "~η~~κᾰ", _, "~η~~κᾰμεν";
    perfect_ind_act_2: _, "~η~~κᾰτον", _;
    perfect_ind_act_3: "~η~~κε|~η~~κεν", _, "~η~~κᾱσῐ|~η~~κᾱσῐν";
    perfect_ind_mid_1: _, _, "~η~~μεθᾰ";
    perfect_ind_mid_2: "~η~~σαι", "~η~~θον", _;
    perfect_ind_mid_3: "~η~~ται", _, "~η~~ᾰται";
    perfect_ind_pass_1: "~η~~μαι", _, _;
    perfect_ind_pass_2: _, _, "~η~~θε";
    perfect_subj_act_1: "~η~~κω", _, _;
    perfect_subj_act_2: "~η~~κῃς", "~η~~κητον", "~η~~κητε";
    perfect_subj_act_3: "~η~~κῃ", _, "~η~~κωσῐ|~η~~κωσῐν";
    perfect_opt_act_1: "~η~~κοιμῐ|~η~~κοιην", _, _;
    perfect_opt_act_2: "~η~~κοις|~η~~κοιης", _, _;
    perfect_opt_act_3: "~η~~κοι|~η~~κοιη", _, _;
    perfect_imp_act_2: "~η~~κε", _, _;
    perfect_imp_mid_2: _, _, "~η~~θε";
    perfect_imp_mid_3: _, _, "~η~~θων";
    perfect_imp_pass_2: "~η~~σο", _, "~η~~θε";
    perfect_imp_pass_3: "~η~~θω", "~η~~θων", "~η~~θων";
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_pres: "~ᾰ~~λειν", _, "~ᾰ~~λεσθαι";
    infinitive_fut: "~ᾰ~~ειν", _, _;
    infinitive_aor: "~ᾰ~ι~αι", _, _;
    infinitive_perf: _, "~η~~θαι", _;
    participle_pres: "~ᾰ~~λων", _, "~ᾰ~~λομενος";
    participle_aor: "~ᾰ~ι~ᾱς", "~ᾰ~ι~ᾰμενος", _;
    participle_perf: "~η~~κως", _, "~η~~μενος";
    ),
    // Ἀμειβω: 4 words (στειβω, λειβω, ἀμειβω, στιλβω). Like Κειμαι, except in 93 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀμειβω", 4, "~βω", like Κειμαι;
    stem aorist_ind: "~ψ- like ἐχω";
    ind_act_1: "~βω", _, "~βομεν";
    ind_act_2: "~βεις", "~βετον", _;
    ind_act_3: "~βει", _, "~βουσῐ";
    ind_mid_1: _, _, "~βομεθᾰ";
    ind_mid_2: "~βῃ", _, _;
    ind_mid_3: _, "~βεσθον", _;
    ind_pass_1: "~βομαι", _, _;
    ind_pass_2: _, "~βεσθον", "~βεσθε";
    ind_pass_3: "~βεται", "~βεσθον", "~βονται";
    subj_act_1: "~βω", _, "~βωμεν";
    subj_act_2: "~βῃς", _, _;
    subj_act_3: "~βῃ", "~βητον", "~βωσῐ";
    subj_mid_1: _, _, "~βωμεθᾰ";
    subj_mid_2: _, "~βησθον", _;
    subj_mid_3: "~βηται", _, "~βωνται";
    subj_pass_1: "~βωμαι", _, _;
    subj_pass_2: _, _, "~βησθε";
    opt_act_1: "~βοιμῐ", _, "~βοιμεν";
    opt_act_2: "~βοις", "~βοιτον", _;
    opt_act_3: "~βοι", "~βοιτην", "~βοιεν";
    opt_mid_1: "~βοιμην", _, "~βοιμεθᾰ";
    opt_mid_2: "~βοιο", _, "~βοισθε";
    opt_mid_3: "~βοιτο", "~βοισθην", "~βοιντο";
    opt_pass_2: _, "~βοισθον", _;
    imp_act_2: "~βε", _, "~βετε";
    imp_act_3: _, _, "~βοντων";
    imp_mid_2: "~βου", "~βεσθον", "~βεσθε";
    imp_mid_3: _, _, "~βεσθων";
    imp_pass_2: _, "~βεσθον", "~βεσθε";
    imp_pass_3: "~βεσθω", "~βεσθων", "~βεσθων";
    imperfect_act_1: "ἐ~βον", _, _;
    imperfect_act_2: "ἐ~βες", "ἐ~βετον", "ἐ~βετε";
    imperfect_act_3: "ἐ~βε", _, "ἐ~βον";
    imperfect_mid_1: "ἐ~βομην", _, "ἐ~βομεθᾰ";
    imperfect_mid_2: "ἐ~βου", _, _;
    imperfect_mid_3: "ἐ~βετο", _, "ἐ~βοντο";
    imperfect_pass_2: _, "ἐ~βεσθον", "ἐ~βεσθε";
    imperfect_pass_3: _, "ἐ~βεσθην", _;
    aorist_subj_act_1: "~ψω", _, _;
    aorist_subj_act_2: "~ψῃς", "~ψητον", _;
    aorist_subj_mid_1: "~ψωμαι", _, _;
    aorist_subj_mid_2: "~ψῃ", "~ψησθον", _;
    aorist_opt_act_1: "~ψαιμῐ", _, "~ψαιμεν";
    aorist_opt_act_2: "~ψειᾰς", _, _;
    aorist_opt_act_3: "~ψειε", _, "~ψειᾰν";
    aorist_opt_mid_1: "~ψαιμην", _, _;
    aorist_imp_act_2: "~ψον", _, "~ψᾰτε";
    aorist_imp_act_3: _, _, "~ψᾰντων";
    aorist_imp_mid_2: "~ψαι", "~ψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ψᾰσθων";
    infinitive_pres: "~βειν", _, "~βεσθαι";
    infinitive_aor: "~ψαι", _, _;
    participle_pres: "~βων", _, "~βομενος";
    participle_aor: "~ψᾱς", "~ψᾰμενος", _;
    ),
    // Προσφερω: 4 words (προφερω, προσφερω, περῐφερω, ὑπερφερω). Like Κειμαι, except in 124 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("προσφερω", 4, "~φερω", like Κειμαι;
    stem future_pass: "~οισθησ- like ἐχω";
    stem aorist_ind: "~ηνεγκ- like ἐλπω";
    stem aorist_ind_pass: "~ηνεχθην- like εὐλογεω";
    stem aorist_pass: "~ενεχθ- like ἐχω";
    ind_act_1: "~φερω", _, "~φερομεν";
    ind_act_2: "~φερεις", "~φερετον", _;
    ind_act_3: "~φερει", _, "~φερουσῐ|~φερουσῐν";
    ind_mid_1: _, _, "~φερομεθᾰ";
    ind_mid_2: "~φερῃ|~φερει", _, _;
    ind_mid_3: _, "~φερεσθον", _;
    ind_pass_1: "~φερομαι", _, _;
    ind_pass_2: _, "~φερεσθον", "~φερεσθε";
    ind_pass_3: "~φερεται", "~φερεσθον", "~φερονται";
    subj_act_1: "~φερω", _, "~φερωμεν";
    subj_act_2: "~φερῃς", _, _;
    subj_act_3: "~φερῃ", "~φερητον", "~φερωσῐ|~φερωσῐν";
    subj_mid_1: _, _, "~φερωμεθᾰ";
    subj_mid_2: _, "~φερησθον", _;
    subj_mid_3: "~φερηται", _, "~φερωνται";
    subj_pass_1: "~φερωμαι", _, _;
    subj_pass_2: _, _, "~φερησθε";
    opt_act_1: "~φεροιμῐ", _, "~φεροιμεν";
    opt_act_2: "~φεροις", "~φεροιτον", _;
    opt_act_3: "~φεροι", "~φεροιτην", "~φεροιεν";
    opt_mid_1: "~φεροιμην", _, "~φεροιμεθᾰ";
    opt_mid_2: "~φεροιο", _, "~φεροισθε";
    opt_mid_3: "~φεροιτο", "~φεροισθην", "~φεροιντο";
    opt_pass_2: _, "~φεροισθον", _;
    imp_act_2: "~φερε", _, "~φερετε";
    imp_act_3: _, _, "~φεροντων";
    imp_mid_2: "~φερου", "~φερεσθον", "~φερεσθε";
    imp_mid_3: _, _, "~φερεσθων";
    imp_pass_2: _, "~φερεσθον", "~φερεσθε";
    imp_pass_3: "~φερεσθω", "~φερεσθων", "~φερεσθων";
    imperfect_act_1: "~εφερον", _, _;
    imperfect_act_2: "~εφερες", "~εφερετον", "~εφερετε";
    imperfect_act_3: "~εφερε|~εφερεν", _, "~εφερον";
    imperfect_mid_1: "~εφερομην", _, "~εφερομεθᾰ";
    imperfect_mid_2: "~εφερου", _, _;
    imperfect_mid_3: "~εφερετο", _, "~εφεροντο";
    imperfect_pass_2: _, "~εφερεσθον", "~εφερεσθε";
    imperfect_pass_3: _, "~εφερεσθην", _;
    future_ind_act_1: "~οισω", _, "~οισομεν";
    future_ind_act_2: "~οισεις", "~οισετον", _;
    future_ind_act_3: _, _, "~οισουσῐ|~οισουσῐν";
    future_ind_mid_1: "~οισομαι", _, _;
    future_ind_mid_2: "~οισῃ|~οισει", "~οισεσθον", _;
    future_ind_mid_3: "~οισεται", _, _;
    future_opt_act_1: "~οισοιμῐ", _, "~οισοιμεν";
    future_opt_act_2: "~οισοις", "~οισοιτον", _;
    future_opt_act_3: "~οισοι", "~οισοιτην", _;
    future_opt_mid_1: "~οισοιμην", _, "~οισοιμεθᾰ";
    future_opt_mid_3: _, "~οισοισθην", _;
    aorist_subj_act_1: "~ενεγκω", _, _;
    aorist_subj_act_2: "~ενεγκῃς", "~ενεγκητον", _;
    aorist_subj_mid_1: "~ενεγκωμαι", _, _;
    aorist_subj_mid_2: "~ενεγκῃ", "~ενεγκησθον", _;
    aorist_opt_act_1: "~ενεγκαιμῐ", _, "~ενεγκαιμεν";
    aorist_opt_act_2: "~ενεγκειᾰς", _, _;
    aorist_opt_act_3: "~ενεγκειε", _, "~ενεγκειᾰν";
    aorist_opt_mid_1: "~ενεγκαιμην", _, _;
    aorist_imp_act_2: "~ενεγκον", _, "~ενεγκᾰτε";
    aorist_imp_act_3: _, _, "~ενεγκᾰντων";
    aorist_imp_mid_2: "~ενεγκαι", "~ενεγκᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ενεγκᾰσθων";
    perfect_ind_act_1: "~ενηνεχᾰ", _, "~ενηνεχᾰμεν";
    perfect_ind_act_2: _, "~ενηνεχᾰτον", _;
    perfect_ind_act_3: _, _, "~ενηνεχᾱσῐ";
    perfect_subj_act_1: "~ενηνεχω", _, _;
    perfect_subj_act_2: "~ενηνεχῃς", "~ενηνεχητον", "~ενηνεχητε";
    perfect_subj_act_3: "~ενηνεχῃ", _, "~ενηνεχωσῐ";
    perfect_imp_act_2: "~ενηνεχε", _, _;
    infinitive_pres: "~φερειν", _, "~φερεσθαι";
    infinitive_fut: "~οισειν", _, _;
    infinitive_aor: "~ενεγκαι", _, _;
    participle_pres: "~φερων", _, "~φερομενος";
    participle_aor: "~ενεγκᾱς", "~ενεγκᾰμενος", _;
    participle_perf: "~ενηνεχως", _, _;
    ),
    // Ἐπιτρεχω: 4 words (περῐτρεχω, κᾰτᾰτρεχω, ἐπιτρεχω, ἀνατρεχω). Like Πῠνθανομαι, except in 149 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐπιτρεχω", 4, "~τ~εχω", like Πῠνθανομαι;
    stem aorist_ind: "~εθ~εξ- like ἐλπω";
    ind_act_1: "~τ~εχω", _, "~τ~εχομεν";
    ind_act_2: "~τ~εχεις", "~τ~εχετον", _;
    ind_act_3: "~τ~εχει", _, "~τ~εχουσῐ";
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, _, _;
    ind_mid_3: _, derived, _;
    ind_pass_1: "~τ~εχομαι", _, "~τ~εχομεθᾰ";
    ind_pass_2: "~τ~εχῃ", "~τ~εχεσθον", "~τ~εχεσθε";
    ind_pass_3: "~τ~εχεται", "~τ~εχεσθον", "~τ~εχονται";
    subj_act_1: "~τ~εχω", _, "~τ~εχωμεν";
    subj_act_2: "~τ~εχῃς", "~τ~εχητον", _;
    subj_act_3: "~τ~εχῃ", _, "~τ~εχωσῐ";
    subj_mid_1: derived, _, derived;
    subj_mid_2: derived, derived, _;
    subj_mid_3: derived, _, derived;
    subj_pass_1: "~τ~εχωμαι", _, "~τ~εχωμεθᾰ";
    subj_pass_2: derived, "~τ~εχησθον", "~τ~εχησθε";
    subj_pass_3: "~τ~εχηται", "~τ~εχησθον", "~τ~εχωνται";
    opt_act_1: "~τ~εχοιμῐ", _, "~τ~εχοιμεν";
    opt_act_2: "~τ~εχοις", "~τ~εχοιτον", _;
    opt_act_3: "~τ~εχοι", "~τ~εχοιτην", "~τ~εχοιεν";
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, _, derived;
    opt_mid_3: derived, derived, derived;
    opt_pass_1: "~τ~εχοιμην", _, "~τ~εχοιμεθᾰ";
    opt_pass_2: "~τ~εχοιο", "~τ~εχοισθον", "~τ~εχοισθε";
    opt_pass_3: "~τ~εχοιτο", "~τ~εχοισθην", "~τ~εχοιντο";
    imp_act_2: "~τ~εχε", _, "~τ~εχετε";
    imp_act_3: _, _, "~τ~εχοντων";
    imp_mid_2: derived, derived, derived;
    imp_mid_3: derived, _, derived;
    imp_pass_2: "~τ~εχου", "~τ~εχεσθον", "~τ~εχεσθε";
    imp_pass_3: "~τ~εχεσθω", "~τ~εχεσθων", "~τ~εχεσθων";
    imperfect_act_1: "~ετ~εχον", _, _;
    imperfect_act_2: "~ετ~εχες", "~ετ~εχετον", "~ετ~εχετε";
    imperfect_act_3: "~ετ~εχε|~ετ~εχεν", _, "~ετ~εχον";
    imperfect_mid_1: "~ετ~εχομην", _, "~ετ~εχομεθᾰ";
    imperfect_mid_2: "~ετ~εχου", derived, _;
    imperfect_mid_3: "~ετ~εχετο", _, "~ετ~εχοντο";
    imperfect_pass_2: _, "~ετ~εχεσθον", "~ετ~εχεσθε";
    imperfect_pass_3: _, "~ετ~εχεσθην", _;
    future_ind_mid_1: "~δ~ᾰμουμαι", _, _;
    future_ind_mid_2: "~δ~ᾰμει", "~δ~ᾰμεισθον", _;
    future_ind_mid_3: "~δ~ᾰμειται", _, _;
    future_opt_mid_1: "~δ~ᾰμοιμην", _, "~δ~ᾰμοιμεθᾰ";
    future_opt_mid_3: _, "~δ~ᾰμοισθην", _;
    aorist_subj_act_1: "~θ~εξω", _, _;
    aorist_subj_act_2: "~θ~εξῃς", "~θ~εξητον", _;
    aorist_subj_mid_1: "~θ~εξωμαι", _, _;
    aorist_subj_mid_2: "~θ~εξῃ", "~θ~εξησθον", _;
    aorist_opt_act_1: "~θ~εξαιμῐ", _, "~θ~εξαιμεν";
    aorist_opt_act_2: "~θ~εξειᾰς", _, _;
    aorist_opt_act_3: "~θ~εξειε", _, "~θ~εξειᾰν";
    aorist_opt_mid_1: "~θ~εξαιμην", _, _;
    aorist_imp_act_2: "~θ~εξον", _, "~θ~εξᾰτε";
    aorist_imp_act_3: _, _, "~θ~εξᾰντων";
    aorist_imp_mid_2: "~θ~εξαι", "~θ~εξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~θ~εξᾰσθων";
    perfect_ind_act_1: "~δεδ~ᾰμηκᾰ", _, "~δεδ~ᾰμηκᾰμεν";
    perfect_ind_act_2: _, "~δεδ~ᾰμηκᾰτον", _;
    perfect_ind_act_3: _, _, "~δεδ~ᾰμηκᾱσῐ";
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "~δεδ~ᾰμηκω", _, _;
    perfect_subj_act_2: "~δεδ~ᾰμηκῃς", "~δεδ~ᾰμηκητον", "~δεδ~ᾰμηκητε";
    perfect_subj_act_3: "~δεδ~ᾰμηκῃ", _, "~δεδ~ᾰμηκωσῐ";
    perfect_imp_act_2: "~δεδ~ᾰμηκε", _, _;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    infinitive_pres: "~τ~εχειν", _, derived;
    infinitive_aor: "~θ~εξαι", _, _;
    infinitive_perf: _, derived, _;
    participle_pres: "~τ~εχων", _, derived;
    participle_aor: "~θ~εξᾱς", "~θ~εξᾰμενος", _;
    participle_perf: "~δεδ~ᾰμηκως", _, derived;
    ),
    // Σπᾰω: 4 words (σπᾰω, κλᾰω, γελᾰω, ἀντῐᾰω). Like Ἐαω, except in 58 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_3 pl, future_ind_mid_1 sg, ….
    conjugation!("σπᾰω", 4, "~ᾰω", like Ἐαω;
    stem future_pass: "~ᾰσθησ- like ἐχω";
    stem aorist_ind: "~ᾰσ- like ποιεω";
    stem aorist_ind_pass: "~ᾰσθην- like ἐχω";
    stem aorist_pass: "~ᾰσθ- like ἐχω";
    future_ind_act_1: "~ᾰσω", _, "~ᾰσομεν";
    future_ind_act_2: "~ᾰσεις", "~ᾰσετον", _;
    future_ind_act_3: _, _, "~ᾰσουσῐ|~ᾰσουσῐν";
    future_ind_mid_1: "~ᾰσομαι", _, _;
    future_ind_mid_2: "~ᾰσῃ|~ᾰσει", "~ᾰσεσθον", _;
    future_ind_mid_3: "~ᾰσεται", _, _;
    future_opt_act_1: "~ᾰσοιμῐ", _, "~ᾰσοιμεν";
    future_opt_act_2: "~ᾰσοις", "~ᾰσοιτον", _;
    future_opt_act_3: "~ᾰσοι", "~ᾰσοιτην", _;
    future_opt_mid_1: "~ᾰσοιμην", _, "~ᾰσοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰσοισθην", _;
    aorist_subj_act_1: "~ᾰσω", _, _;
    aorist_subj_act_2: "~ᾰσῃς", "~ᾰσητον", _;
    aorist_subj_act_3: _, _, "~ᾰσωσῐ|~ᾰσωσῐν";
    aorist_subj_mid_1: "~ᾰσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰσῃ", "~ᾰσησθον", _;
    aorist_opt_act_1: "~ᾰσαιμῐ", _, "~ᾰσαιμεν";
    aorist_opt_act_2: "~ᾰσειᾰς|~ᾰσαις", _, _;
    aorist_opt_act_3: "~ᾰσειε|~ᾰσειεν|~ᾰσαι", _, "~ᾰσειᾰν|~ᾰσαιεν";
    aorist_opt_mid_1: "~ᾰσαιμην", _, _;
    aorist_imp_act_2: "~ᾰσον", _, "~ᾰσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰσᾰντων";
    aorist_imp_mid_2: "~ᾰσαι", "~ᾰσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰσᾰσθων";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_1: _, _, "%ε~ᾰσμεθᾰ";
    perfect_ind_mid_3: "%ε~ᾰσται", _, "%ε~ᾰσᾰται";
    perfect_ind_pass_1: "%ε~ᾰσμαι", _, _;
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_imp_act_2: derived, _, _;
    infinitive_fut: "~ᾰσειν", _, _;
    infinitive_aor: "~ᾰσαι", _, _;
    participle_aor: "~ᾰσᾱς", "~ᾰσᾰμενος", _;
    participle_perf: derived, _, "%ε~ᾰσμενος";
    ),
    // Σῠνᾰγω: 4 words (σῠνᾰγω, προᾰγω, ξῠνᾰγω, προσᾰγω). Like Ἀνᾰγω, except in 58 cells: future_ind_act_3 pl, future_ind_mid_2 sg, future_ind_pass_2 sg, aorist_subj_act_1 sg, aorist_subj_act_1 pl, aorist_subj_act_2 sg, ….
    conjugation!("σῠνᾰγω", 4, "~ᾰγω", like Ἀνᾰγω;
    stem aorist_ind: "~η- like σῠνᾰγω";
    stem aorist_pass: "~ᾰχθ- like ποιεω";
    future_ind_act_3: _, _, "~ᾰξουσῐ";
    future_ind_mid_2: "~ᾰξῃ", _, _;
    future_ind_pass_2: derived, _, _;
    aorist_subj_act_1: "~ᾰξω|~ᾰγᾰγω", _, "~ᾰξωμεν|~ᾰγᾰγωμεν";
    aorist_subj_act_2: "~ᾰξῃς|~ᾰγᾰγῃς", "~ᾰξητον|~ᾰγᾰγητον", "~ᾰξητε|~ᾰγᾰγητε";
    aorist_subj_act_3: "~ᾰξῃ|~ᾰγᾰγῃ", _, "~ᾰξωσῐ|~ᾰξωσῐν|~ᾰγᾰγωσῐ|~ᾰγᾰγωσῐν";
    aorist_subj_mid_1: "~ᾰξωμαι|~ᾰγᾰγωμαι", _, "~ᾰξωμεθᾰ|~ᾰγᾰγωμεθᾰ";
    aorist_subj_mid_2: derived, "~ᾰξησθον|~ᾰγᾰγησθον", "~ᾰξησθε|~ᾰγᾰγησθε";
    aorist_subj_mid_3: "~ᾰξηται|~ᾰγᾰγηται", _, "~ᾰξωνται|~ᾰγᾰγωνται";
    aorist_opt_act_1: "~ᾰξαιμῐ|~ᾰγᾰγοιμῐ", _, "~ᾰξαιμεν|~ᾰγᾰγοιμεν";
    aorist_opt_act_2: "~ᾰξειᾰς|~ᾰξαις|~ᾰγᾰγοις", "~ᾰξαιτον|~ᾰγᾰγοιτον", "~ᾰξαιτε|~ᾰγᾰγοιτε";
    aorist_opt_act_3: "~ᾰξειε|~ᾰξειεν|~ᾰξαι|~ᾰγᾰγοι", "~ᾰξαιτην|~ᾰγᾰγοιτην", "~ᾰξειᾰν|~ᾰξαιεν|~ᾰγᾰγοιεν";
    aorist_opt_mid_1: "~ᾰξαιμην|~ᾰγᾰγοιμην", _, "~ᾰξαιμεθᾰ|~ᾰγᾰγοιμεθᾰ";
    aorist_opt_mid_2: "~ᾰξαιο|~ᾰγᾰγοιο", "~ᾰξαισθον|~ᾰγᾰγοισθον", "~ᾰξαισθε|~ᾰγᾰγοισθε";
    aorist_opt_mid_3: "~ᾰξαιτο|~ᾰγᾰγοιτο", "~ᾰξαισθην|~ᾰγᾰγοισθην", "~ᾰξαιντο|~ᾰγᾰγοιντο";
    aorist_imp_act_2: "~ᾰξον|~ᾰγᾰγε", "~ᾰξᾰτον|~ᾰγᾰγετον", "~ᾰξᾰτε|~ᾰγᾰγετε";
    aorist_imp_act_3: "~ᾰξᾰτω|~ᾰγᾰγετω", "~ᾰξᾰτων|~ᾰγᾰγετων", "~ᾰξᾰντων|~ᾰγᾰγοντων";
    aorist_imp_mid_2: "~ᾰξαι|~ᾰγᾰγου", "~ᾰξᾰσθον|~ᾰγᾰγεσθον", "~ᾰξᾰσθε|~ᾰγᾰγεσθε";
    aorist_imp_mid_3: "~ᾰξᾰσθω|~ᾰγᾰγεσθω", "~ᾰξᾰσθων|~ᾰγᾰγεσθων", derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    infinitive_aor: "~ᾰξαι|~ᾰγᾰγειν", "~ᾰξᾰσθαι|~ᾰγᾰγεσθαι", _;
    infinitive_perf: _, derived, _;
    participle_aor: "~ᾰξᾱς|~ᾰγᾰγων", "~ᾰξᾰμενος|~ᾰγᾰγομενος", _;
    participle_perf: _, _, derived;
    ),
    // Σᾰττω: 4 words (φῠλᾰττω, πληττω, σᾰττω, ἀλλᾰττω). Like Ληγω, except in 86 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("σᾰττω", 4, "~ττω", like Ληγω;
    stem future_pass: "~γησ- like ἐχω";
    stem aorist_ind_pass: "~γην- like ἐχω";
    stem aorist_pass: "~γ- like ἐγειρω";
    ind_act_1: "~ττω", _, "~ττομεν";
    ind_act_2: "~ττεις", "~ττετον", _;
    ind_act_3: "~ττει", _, "~ττουσῐ|~ττουσῐν";
    ind_mid_1: _, _, "~ττομεθᾰ";
    ind_mid_2: "~ττῃ|~ττει", _, _;
    ind_mid_3: _, "~ττεσθον", _;
    ind_pass_1: "~ττομαι", _, _;
    ind_pass_2: _, "~ττεσθον", "~ττεσθε";
    ind_pass_3: "~ττεται", "~ττεσθον", "~ττονται";
    subj_act_1: "~ττω", _, "~ττωμεν";
    subj_act_2: "~ττῃς", _, _;
    subj_act_3: "~ττῃ", "~ττητον", "~ττωσῐ|~ττωσῐν";
    subj_mid_1: _, _, "~ττωμεθᾰ";
    subj_mid_2: _, "~ττησθον", _;
    subj_mid_3: "~ττηται", _, "~ττωνται";
    subj_pass_1: "~ττωμαι", _, _;
    subj_pass_2: _, _, "~ττησθε";
    opt_act_1: "~ττοιμῐ", _, "~ττοιμεν";
    opt_act_2: "~ττοις", "~ττοιτον", _;
    opt_act_3: "~ττοι", "~ττοιτην", "~ττοιεν";
    opt_mid_1: "~ττοιμην", _, "~ττοιμεθᾰ";
    opt_mid_2: "~ττοιο", _, "~ττοισθε";
    opt_mid_3: "~ττοιτο", "~ττοισθην", "~ττοιντο";
    opt_pass_2: _, "~ττοισθον", _;
    imp_act_2: "~ττε", _, "~ττετε";
    imp_act_3: _, _, "~ττοντων";
    imp_mid_2: "~ττου", "~ττεσθον", "~ττεσθε";
    imp_mid_3: _, _, "~ττεσθων";
    imp_pass_2: _, "~ττεσθον", "~ττεσθε";
    imp_pass_3: "~ττεσθω", "~ττεσθων", "~ττεσθων";
    imperfect_act_1: "ἐ~ττον", _, _;
    imperfect_act_2: "ἐ~ττες", "ἐ~ττετον", "ἐ~ττετε";
    imperfect_act_3: "ἐ~ττε|ἐ~ττεν", _, "ἐ~ττον";
    imperfect_mid_1: "ἐ~ττομην", _, "ἐ~ττομεθᾰ";
    imperfect_mid_2: "ἐ~ττου", _, _;
    imperfect_mid_3: "ἐ~ττετο", _, "ἐ~ττοντο";
    imperfect_pass_2: _, "ἐ~ττεσθον", "ἐ~ττεσθε";
    imperfect_pass_3: _, "ἐ~ττεσθην", _;
    future_ind_act_3: _, _, "~ξουσῐ|~ξουσῐν";
    future_ind_mid_2: "~ξῃ|~ξει", _, _;
    perfect_ind_act_1: "%ε~γᾰ", _, "%ε~γᾰμεν";
    perfect_ind_act_2: _, "%ε~γᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~γᾱσῐ";
    perfect_subj_act_1: "%ε~γω", _, _;
    perfect_subj_act_2: "%ε~γῃς", "%ε~γητον", "%ε~γητε";
    perfect_subj_act_3: "%ε~γῃ", _, "%ε~γωσῐ";
    perfect_imp_act_2: "%ε~γε", _, _;
    infinitive_pres: "~ττειν", _, "~ττεσθαι";
    participle_pres: "~ττων", _, "~ττομενος";
    participle_perf: "%ε~γως", _, _;
    ),
    // Ἐργᾰζομαι: 4 words (στοχᾰζομαι, ἐργᾰζομαι, ἀσπᾰζομαι, βῐᾰζομαι). Like Ἑπομαι, except in 58 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_1 sg, ….
    conjugation!("ἐργᾰζομαι", 4, "~ζομαι", like Ἑπομαι;
    stem future_pass: "~σθησ- like ἐχω";
    stem aorist_ind_pass: "~σθην- like ἐχω";
    stem aorist_pass: "~σθ- like ἐχω";
    ind_act_1: "~ζομαι", _, _;
    ind_mid_1: "~ζομαι", _, "~ζομεθᾰ";
    ind_mid_2: "~ζῃ", _, _;
    ind_mid_3: _, "~ζεσθον", _;
    ind_pass_1: derived, _, _;
    ind_pass_2: _, "~ζεσθον", "~ζεσθε";
    ind_pass_3: "~ζεται", "~ζεσθον", "~ζονται";
    subj_mid_1: "~ζωμαι", _, "~ζωμεθᾰ";
    subj_mid_2: "~ζῃ", "~ζησθον", _;
    subj_mid_3: "~ζηται", _, "~ζωνται";
    subj_pass_1: derived, _, _;
    subj_pass_2: "~ζῃ", _, "~ζησθε";
    opt_mid_1: "~ζοιμην", _, "~ζοιμεθᾰ";
    opt_mid_2: "~ζοιο", _, "~ζοισθε";
    opt_mid_3: "~ζοιτο", "~ζοισθην", "~ζοιντο";
    opt_pass_2: _, "~ζοισθον", _;
    imp_mid_2: "~ζου", "~ζεσθον", "~ζεσθε";
    imp_mid_3: "~ζεσθω", _, "~ζεσθων";
    imp_pass_2: _, "~ζεσθον", "~ζεσθε";
    imp_pass_3: derived, "~ζεσθων", "~ζεσθων";
    imperfect_mid_1: "ἐ~ζομην", _, "ἐ~ζομεθᾰ";
    imperfect_mid_2: "ἐ~ζου", "ἐ~ζεσθον", _;
    imperfect_mid_3: "ἐ~ζετο", _, "ἐ~ζοντο";
    imperfect_pass_2: _, derived, "ἐ~ζεσθε";
    imperfect_pass_3: _, "ἐ~ζεσθην", _;
    perfect_ind_mid_1: _, _, "%ε~σμεθᾰ";
    perfect_ind_mid_3: "%ε~σται", _, "%ε~δᾰται";
    perfect_ind_pass_1: "%ε~σμαι", _, _;
    pluperfect_mid_1: "ἐ%ε~σμην", _, _;
    pluperfect_mid_3: "ἐ%ε~στο", _, "ἐ%ε~δᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~σμεθᾰ";
    infinitive_pres: _, _, "~ζεσθαι";
    participle_pres: _, _, "~ζομενος";
    participle_perf: _, _, "%ε~σμενος";
    ),
    // Ἐρᾰμαι: 4 words (ἐρᾰμαι, μᾰρνᾰμαι, ἀγᾰμαι, λῐνᾰμαι). Like Ἱημῐ, except in 41 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐρᾰμαι", 4, "~ᾰμαι", like Ἱημῐ;
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
    // Ἀνᾰπτω: 4 words (ἀνᾰπτω, ἐξᾰπτω, καθᾰπτω, ἐπῐσῠνᾰπτω). Like Ληγω, except in 137 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀνᾰπτω", 4, "~ᾰπτω", like Ληγω;
    stem future_pass: "~ᾰφθησ- like ἐχω";
    stem aorist_ind: "~ηψ- like εὐλογεω";
    stem aorist_ind_pass: "~ηφθην- like εὐλογεω";
    stem aorist_pass: "~ᾰφθ- like ποιεω";
    ind_act_1: "~ᾰπτω", _, "~ᾰπτομεν";
    ind_act_2: "~ᾰπτεις", "~ᾰπτετον", _;
    ind_act_3: "~ᾰπτει", _, "~ᾰπτουσῐ|~ᾰπτουσῐν";
    ind_mid_1: _, _, "~ᾰπτομεθᾰ";
    ind_mid_2: "~ᾰπτῃ|~ᾰπτει", _, _;
    ind_mid_3: _, "~ᾰπτεσθον", _;
    ind_pass_1: "~ᾰπτομαι", _, _;
    ind_pass_2: _, "~ᾰπτεσθον", "~ᾰπτεσθε";
    ind_pass_3: "~ᾰπτεται", "~ᾰπτεσθον", "~ᾰπτονται";
    subj_act_1: "~ᾰπτω", _, "~ᾰπτωμεν";
    subj_act_2: "~ᾰπτῃς", _, _;
    subj_act_3: "~ᾰπτῃ", "~ᾰπτητον", "~ᾰπτωσῐ|~ᾰπτωσῐν";
    subj_mid_1: _, _, "~ᾰπτωμεθᾰ";
    subj_mid_2: _, "~ᾰπτησθον", _;
    subj_mid_3: "~ᾰπτηται", _, "~ᾰπτωνται";
    subj_pass_1: "~ᾰπτωμαι", _, _;
    subj_pass_2: _, _, "~ᾰπτησθε";
    opt_act_1: "~ᾰπτοιμῐ", _, "~ᾰπτοιμεν";
    opt_act_2: "~ᾰπτοις", "~ᾰπτοιτον", _;
    opt_act_3: "~ᾰπτοι", "~ᾰπτοιτην", "~ᾰπτοιεν";
    opt_mid_1: "~ᾰπτοιμην", _, "~ᾰπτοιμεθᾰ";
    opt_mid_2: "~ᾰπτοιο", _, "~ᾰπτοισθε";
    opt_mid_3: "~ᾰπτοιτο", "~ᾰπτοισθην", "~ᾰπτοιντο";
    opt_pass_2: _, "~ᾰπτοισθον", _;
    imp_act_2: "~ᾰπτε", _, "~ᾰπτετε";
    imp_act_3: _, _, "~ᾰπτοντων";
    imp_mid_2: "~ᾰπτου", "~ᾰπτεσθον", "~ᾰπτεσθε";
    imp_mid_3: _, _, "~ᾰπτεσθων";
    imp_pass_2: _, "~ᾰπτεσθον", "~ᾰπτεσθε";
    imp_pass_3: "~ᾰπτεσθω", "~ᾰπτεσθων", "~ᾰπτεσθων";
    imperfect_act_1: "~ηπτον", _, _;
    imperfect_act_2: "~ηπτες", "~ηπτετον", "~ηπτετε";
    imperfect_act_3: "~ηπτε|~ηπτεν", _, "~ηπτον";
    imperfect_mid_1: "~ηπτομην", _, "~ηπτομεθᾰ";
    imperfect_mid_2: "~ηπτου", _, _;
    imperfect_mid_3: "~ηπτετο", _, "~ηπτοντο";
    imperfect_pass_2: _, "~ηπτεσθον", "~ηπτεσθε";
    imperfect_pass_3: _, "~ηπτεσθην", _;
    future_ind_act_1: "~ᾰψω", _, "~ᾰψομεν";
    future_ind_act_2: "~ᾰψεις", "~ᾰψετον", _;
    future_ind_act_3: _, _, "~ᾰψουσῐ|~ᾰψουσῐν";
    future_ind_mid_1: "~ᾰψομαι", _, _;
    future_ind_mid_2: "~ᾰψῃ|~ᾰψει", "~ᾰψεσθον", _;
    future_ind_mid_3: "~ᾰψεται", _, _;
    future_opt_act_1: "~ᾰψοιμῐ", _, "~ᾰψοιμεν";
    future_opt_act_2: "~ᾰψοις", "~ᾰψοιτον", _;
    future_opt_act_3: "~ᾰψοι", "~ᾰψοιτην", _;
    future_opt_mid_1: "~ᾰψοιμην", _, "~ᾰψοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰψοισθην", _;
    aorist_subj_act_1: "~ᾰψω", _, _;
    aorist_subj_act_2: "~ᾰψῃς", "~ᾰψητον", _;
    aorist_subj_act_3: _, _, "~ᾰψωσῐ|~ᾰψωσῐν";
    aorist_subj_mid_1: "~ᾰψωμαι", _, _;
    aorist_subj_mid_2: "~ᾰψῃ", "~ᾰψησθον", _;
    aorist_opt_act_1: "~ᾰψαιμῐ", _, "~ᾰψαιμεν";
    aorist_opt_act_2: "~ᾰψειᾰς|~ᾰψαις", _, _;
    aorist_opt_act_3: "~ᾰψειε|~ᾰψειεν|~ᾰψαι", _, "~ᾰψειᾰν|~ᾰψαιεν";
    aorist_opt_mid_1: "~ᾰψαιμην", _, _;
    aorist_imp_act_2: "~ᾰψον", _, "~ᾰψᾰτε";
    aorist_imp_act_3: _, _, "~ᾰψᾰντων";
    aorist_imp_mid_2: "~ᾰψαι", "~ᾰψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰψᾰσθων";
    perfect_ind_mid_1: _, _, "~ημμεθᾰ";
    perfect_ind_mid_2: "~ηψαι", "~ηφθον", _;
    perfect_ind_mid_3: "~ηπται", _, "~ηπᾰται";
    perfect_ind_pass_1: "~ημμαι", _, _;
    perfect_ind_pass_2: _, _, "~ηφθε";
    perfect_imp_mid_2: _, _, "~ηφθε";
    perfect_imp_mid_3: _, _, "~ηφθων";
    perfect_imp_pass_2: "~ηψο", _, "~ηφθε";
    perfect_imp_pass_3: "~ηφθω", "~ηφθων", "~ηφθων";
    pluperfect_mid_1: "~ημμην", _, _;
    pluperfect_mid_2: "~ηψο", "~ηφθον", _;
    pluperfect_mid_3: "~ηπτο", _, "~ηπᾰτο";
    pluperfect_pass_1: _, _, "~ημμεθᾰ";
    pluperfect_pass_2: _, _, "~ηφθε";
    pluperfect_pass_3: _, "~ηφθην", _;
    infinitive_pres: "~ᾰπτειν", _, "~ᾰπτεσθαι";
    infinitive_fut: "~ᾰψειν", _, _;
    infinitive_aor: "~ᾰψαι", _, _;
    infinitive_perf: _, "~ηφθαι", _;
    participle_pres: "~ᾰπτων", _, "~ᾰπτομενος";
    participle_aor: "~ᾰψᾱς", "~ᾰψᾰμενος", _;
    participle_perf: _, _, "~ημμενος";
    ),
    // Δῡω: 4 words (δῡω, καμμῡω, καταμῡω, κᾰσσῡω). Like Λῡω, except in 69 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_3 pl, future_ind_mid_2 sg, ….
    conjugation!("δῡω", 4, "~ῡω", like Λῡω;
    stem future_pass: "~ῠθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ῡσ- like ποιεω";
    stem aorist_pass: "~ῠθ- like ποιεω";
    stem pluperfect_act: "~ῠκειν- like ἐχω";
    ind_act_3: _, _, "~ῡουσῐ|~ῡουσῐν";
    ind_mid_2: "~ῡῃ|~ῡει", _, _;
    subj_act_3: _, _, "~ῡωσῐ|~ῡωσῐν";
    imperfect_act_3: "ἐ~ῡε|ἐ~ῡεν", _, _;
    future_ind_act_3: _, _, "~ῡσουσῐ|~ῡσουσῐν";
    future_ind_mid_2: "~ῡσῃ|~ῡσει", _, _;
    aorist_subj_act_3: _, _, "~ῡσωσῐ|~ῡσωσῐν";
    aorist_opt_act_2: "~ῡσειᾰς|~ῡσαις", _, _;
    aorist_opt_act_3: "~ῡσειε|~ῡσειεν|~ῡσαι", _, "~ῡσειᾰν|~ῡσαιεν";
    perfect_ind_act_1: "%ε~ῠκᾰ", _, "%ε~ῠκᾰμεν";
    perfect_ind_act_2: _, "%ε~ῠκᾰτον", _;
    perfect_ind_act_3: "%ε~ῠκε|%ε~ῠκεν", _, "%ε~ῠκᾱσῐ|%ε~ῠκᾱσῐν";
    perfect_ind_mid_1: _, _, "%ε~ῠμεθᾰ";
    perfect_ind_mid_2: "%ε~ῠσαι", "%ε~ῠσθον", _;
    perfect_ind_mid_3: "%ε~ῠται", _, "%ε~ῠνται";
    perfect_ind_pass_1: "%ε~ῠμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ῠσθε";
    perfect_subj_act_1: "%ε~ῠκω", _, _;
    perfect_subj_act_2: "%ε~ῠκῃς", "%ε~ῠκητον", "%ε~ῠκητε";
    perfect_subj_act_3: "%ε~ῠκῃ", _, "%ε~ῠκωσῐ|%ε~ῠκωσῐν";
    perfect_opt_act_1: "%ε~ῠκοιμῐ|%ε~ῠκοιην", _, _;
    perfect_opt_act_2: "%ε~ῠκοις|%ε~ῠκοιης", _, _;
    perfect_opt_act_3: "%ε~ῠκοι|%ε~ῠκοιη", _, _;
    perfect_imp_act_2: "%ε~ῠκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~ῠσθε";
    perfect_imp_mid_3: _, _, "%ε~ῠσθων";
    perfect_imp_pass_2: "%ε~ῠσο", _, "%ε~ῠσθε";
    perfect_imp_pass_3: "%ε~ῠσθω", "%ε~ῠσθων", "%ε~ῠσθων";
    pluperfect_mid_1: "ἐ%ε~ῠμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ῠσο", "ἐ%ε~ῠσθον", _;
    pluperfect_mid_3: "ἐ%ε~ῠτο", _, "ἐ%ε~ῠντο";
    pluperfect_pass_1: _, _, "ἐ%ε~ῠμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ῠσθε";
    pluperfect_pass_3: _, "ἐ%ε~ῠσθην", _;
    future_perfect_ind_mid_1: "%ε~ῡσομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~ῡσεσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~ῡσομεθᾰ";
    future_perfect_ind_pass_2: "%ε~ῡσῃ", "%ε~ῡσεσθον", "%ε~ῡσεσθε";
    future_perfect_ind_pass_3: "%ε~ῡσεται", "%ε~ῡσεσθον", "%ε~ῡσονται";
    future_perfect_opt_pass_1: "%ε~ῡσοιμην", _, "%ε~ῡσοιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~ῡσοιο", "%ε~ῡσοισθον", "%ε~ῡσοισθε";
    future_perfect_opt_pass_3: "%ε~ῡσοιτο", "%ε~ῡσοισθην", "%ε~ῡσοιντο";
    infinitive_perf: _, "%ε~ῠσθαι", _;
    infinitive_futp: _, _, "%ε~ῡσεσθαι";
    participle_perf: "%ε~ῠκως", _, "%ε~ῠμενος";
    participle_futp: _, _, "%ε~ῡσομενος";
    ),
    // Πειρᾰω: 4 words (κοπῐᾰω, ἀᾰω, πειρᾰω, μειδῐᾰω). Like Ἐαω, except in 24 cells: future_ind_mid_2 sg, future_ind_pass_1 sg, future_ind_pass_3 du, perfect_ind_mid_1 pl, perfect_ind_mid_2 sg, perfect_ind_mid_2 du, ….
    conjugation!("πειρᾰω", 4, "~ᾰω", like Ἐαω;
    future_ind_mid_2: "~ᾱσῃ", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    perfect_ind_mid_1: _, _, "%ε~ᾱμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾱσαι", "%ε~ᾱσθον", _;
    perfect_ind_mid_3: "%ε~ᾱται", _, "%ε~ᾱνται";
    perfect_ind_pass_1: "%ε~ᾱμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ᾱσθε";
    perfect_imp_mid_2: _, _, "%ε~ᾱσθε";
    perfect_imp_mid_3: _, _, "%ε~ᾱσθων";
    perfect_imp_pass_2: "%ε~ᾱσο", _, "%ε~ᾱσθε";
    perfect_imp_pass_3: "%ε~ᾱσθω", "%ε~ᾱσθων", "%ε~ᾱσθων";
    pluperfect_mid_1: "ἐ%ε~ᾱμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ᾱσο", "ἐ%ε~ᾱσθον", _;
    pluperfect_mid_3: "ἐ%ε~ᾱτο", _, "ἐ%ε~ᾱντο";
    infinitive_perf: _, "%ε~ᾱσθαι", _;
    participle_perf: _, _, "%ε~ᾱμενος";
    ),
    // Πᾰρᾰλλᾰσσω: 4 words (ἀπᾰλλᾰσσω, ὑπᾰλλᾰσσω, ἀντᾰλλᾰσσω, πᾰρᾰλλᾰσσω). Like Ἀποκωλῡω, except in 168 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πᾰρᾰλλᾰσσω", 4, "~ᾰ~σσω", like Ἀποκωλῡω;
    stem future_pass: "~ᾰ~χθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~η~ξ- like ἐλπω";
    stem aorist_ind_pass: "~η~- like πᾰρᾰλλᾰσσω";
    ind_act_1: "~ᾰ~σσω", _, "~ᾰ~σσομεν";
    ind_act_2: "~ᾰ~σσεις", "~ᾰ~σσετον", _;
    ind_act_3: "~ᾰ~σσει", _, "~ᾰ~σσουσῐ|~ᾰ~σσουσῐν";
    ind_mid_1: _, _, "~ᾰ~σσομεθᾰ";
    ind_mid_2: "~ᾰ~σσῃ|~ᾰ~σσει", _, _;
    ind_mid_3: _, "~ᾰ~σσεσθον", _;
    ind_pass_1: "~ᾰ~σσομαι", _, _;
    ind_pass_2: _, "~ᾰ~σσεσθον", "~ᾰ~σσεσθε";
    ind_pass_3: "~ᾰ~σσεται", "~ᾰ~σσεσθον", "~ᾰ~σσονται";
    subj_act_1: "~ᾰ~σσω", _, "~ᾰ~σσωμεν";
    subj_act_2: "~ᾰ~σσῃς", _, _;
    subj_act_3: "~ᾰ~σσῃ", "~ᾰ~σσητον", "~ᾰ~σσωσῐ|~ᾰ~σσωσῐν";
    subj_mid_1: _, _, "~ᾰ~σσωμεθᾰ";
    subj_mid_2: _, "~ᾰ~σσησθον", _;
    subj_mid_3: "~ᾰ~σσηται", _, "~ᾰ~σσωνται";
    subj_pass_1: "~ᾰ~σσωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~σσησθε";
    opt_act_1: "~ᾰ~σσοιμῐ", _, "~ᾰ~σσοιμεν";
    opt_act_2: "~ᾰ~σσοις", "~ᾰ~σσοιτον", _;
    opt_act_3: "~ᾰ~σσοι", "~ᾰ~σσοιτην", "~ᾰ~σσοιεν";
    opt_mid_1: "~ᾰ~σσοιμην", _, "~ᾰ~σσοιμεθᾰ";
    opt_mid_2: "~ᾰ~σσοιο", _, "~ᾰ~σσοισθε";
    opt_mid_3: "~ᾰ~σσοιτο", "~ᾰ~σσοισθην", "~ᾰ~σσοιντο";
    opt_pass_2: _, "~ᾰ~σσοισθον", _;
    imp_act_2: "~ᾰ~σσε", _, "~ᾰ~σσετε";
    imp_act_3: _, _, "~ᾰ~σσοντων";
    imp_mid_2: "~ᾰ~σσου", "~ᾰ~σσεσθον", "~ᾰ~σσεσθε";
    imp_mid_3: _, _, "~ᾰ~σσεσθων";
    imp_pass_2: _, "~ᾰ~σσεσθον", "~ᾰ~σσεσθε";
    imp_pass_3: "~ᾰ~σσεσθω", "~ᾰ~σσεσθων", "~ᾰ~σσεσθων";
    imperfect_act_1: "~η~σσον", _, _;
    imperfect_act_2: "~η~σσες", "~η~σσετον", "~η~σσετε";
    imperfect_act_3: "~η~σσε|~η~σσεν", _, "~η~σσον";
    imperfect_mid_1: "~η~σσομην", _, "~η~σσομεθᾰ";
    imperfect_mid_2: "~η~σσου", _, _;
    imperfect_mid_3: "~η~σσετο", _, "~η~σσοντο";
    imperfect_pass_2: _, "~η~σσεσθον", "~η~σσεσθε";
    imperfect_pass_3: _, "~η~σσεσθην", _;
    future_ind_act_1: "~ᾰ~ξω", _, "~ᾰ~ξομεν";
    future_ind_act_2: "~ᾰ~ξεις", "~ᾰ~ξετον", _;
    future_ind_act_3: _, _, "~ᾰ~ξουσῐ|~ᾰ~ξουσῐν";
    future_ind_mid_1: "~ᾰ~ξομαι", _, _;
    future_ind_mid_2: "~ᾰ~ξῃ|~ᾰ~ξει", "~ᾰ~ξεσθον", _;
    future_ind_mid_3: "~ᾰ~ξεται", _, _;
    future_opt_act_1: "~ᾰ~ξοιμῐ", _, "~ᾰ~ξοιμεν";
    future_opt_act_2: "~ᾰ~ξοις", "~ᾰ~ξοιτον", _;
    future_opt_act_3: "~ᾰ~ξοι", "~ᾰ~ξοιτην", _;
    future_opt_mid_1: "~ᾰ~ξοιμην", _, "~ᾰ~ξοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~ξοισθην", _;
    aorist_subj_act_1: "~ᾰ~ξω", _, _;
    aorist_subj_act_2: "~ᾰ~ξῃς", "~ᾰ~ξητον", _;
    aorist_subj_mid_1: "~ᾰ~ξωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ξῃ", "~ᾰ~ξησθον", _;
    aorist_subj_pass_1: "~ᾰ~χθω|~ᾰ~γω", _, "~ᾰ~χθωμεν|~ᾰ~γωμεν";
    aorist_subj_pass_2: "~ᾰ~χθῃς|~ᾰ~γῃς", "~ᾰ~χθητον|~ᾰ~γητον", "~ᾰ~χθητε|~ᾰ~γητε";
    aorist_subj_pass_3: "~ᾰ~χθῃ|~ᾰ~γῃ", derived, "~ᾰ~χθωσῐ|~ᾰ~χθωσῐν|~ᾰ~γωσῐ|~ᾰ~γωσῐν";
    aorist_opt_act_1: "~ᾰ~ξαιμῐ", _, "~ᾰ~ξαιμεν";
    aorist_opt_act_2: "~ᾰ~ξειᾰς", _, _;
    aorist_opt_act_3: "~ᾰ~ξειε", _, "~ᾰ~ξειᾰν";
    aorist_opt_mid_1: "~ᾰ~ξαιμην", _, _;
    aorist_opt_pass_1: "~ᾰ~χθειην|~ᾰ~γειην", _, "~ᾰ~χθειμεν|~ᾰ~χθειημεν|~ᾰ~γειμεν|~ᾰ~γειημεν";
    aorist_opt_pass_2: "~ᾰ~χθειης|~ᾰ~γειης", "~ᾰ~χθειτον|~ᾰ~χθειητον|~ᾰ~γειτον|~ᾰ~γειητον", "~ᾰ~χθειτε|~ᾰ~χθειητε|~ᾰ~γειτε|~ᾰ~γειητε";
    aorist_opt_pass_3: "~ᾰ~χθειη|~ᾰ~γειη", "~ᾰ~χθειτην|~ᾰ~χθειητην|~ᾰ~γειτην|~ᾰ~γειητην", "~ᾰ~χθειεν|~ᾰ~χθειησᾰν|~ᾰ~γειεν|~ᾰ~γειησᾰν";
    aorist_imp_act_2: "~ᾰ~ξον", _, "~ᾰ~ξᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ξᾰντων";
    aorist_imp_mid_2: "~ᾰ~ξαι", "~ᾰ~ξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ξᾰσθων";
    aorist_imp_pass_2: "~ᾰ~χθητῐ|~ᾰ~γηθῐ", derived, derived;
    aorist_imp_pass_3: "~ᾰ~χθητω|~ᾰ~γητω", "~ᾰ~χθητων|~ᾰ~γητων", "~ᾰ~χθεντων|~ᾰ~γεντων";
    perfect_ind_act_1: "~η~χᾰ", _, "~η~χᾰμεν";
    perfect_ind_act_2: _, "~η~χᾰτον", _;
    perfect_ind_act_3: "~η~χε|~η~χεν", _, "~η~χᾱσῐ|~η~χᾱσῐν";
    perfect_ind_mid_1: _, _, "~η~γμεθᾰ";
    perfect_ind_mid_2: "~η~ξαι", "~η~χθον", _;
    perfect_ind_mid_3: "~η~κται", _, "~η~χᾰται";
    perfect_ind_pass_1: "~η~γμαι", _, _;
    perfect_ind_pass_2: _, _, "~η~χθε";
    perfect_subj_act_1: "~η~χω", _, _;
    perfect_subj_act_2: "~η~χῃς", "~η~χητον", "~η~χητε";
    perfect_subj_act_3: "~η~χῃ", _, "~η~χωσῐ|~η~χωσῐν";
    perfect_opt_act_1: "~η~χοιμῐ|~η~χοιην", _, _;
    perfect_opt_act_2: "~η~χοις|~η~χοιης", _, _;
    perfect_opt_act_3: "~η~χοι|~η~χοιη", _, _;
    perfect_imp_act_2: "~η~χε", _, _;
    perfect_imp_mid_2: _, _, "~η~χθε";
    perfect_imp_mid_3: _, _, "~η~χθων";
    perfect_imp_pass_2: "~η~ξο", _, "~η~χθε";
    perfect_imp_pass_3: "~η~χθω", "~η~χθων", "~η~χθων";
    infinitive_pres: "~ᾰ~σσειν", _, "~ᾰ~σσεσθαι";
    infinitive_fut: "~ᾰ~ξειν", _, _;
    infinitive_aor: "~ᾰ~ξαι", _, "~ᾰ~χθηναι|~ᾰ~γηναι";
    infinitive_perf: _, "~η~χθαι", _;
    participle_pres: "~ᾰ~σσων", _, "~ᾰ~σσομενος";
    participle_aor: "~ᾰ~ξᾱς", "~ᾰ~ξᾰμενος", "~ᾰ~χθεις|~ᾰ~γεις";
    participle_perf: "~η~χως", _, "~η~γμενος";
    ),
    // Κᾰτειργω: 4 words (ἐργω, εἰργω, ἀμεργω, κᾰτειργω). Like Ληγω, except in 43 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, ….
    conjugation!("κᾰτειργω", 4, "~γω", like Ληγω;
    stem aorist_ind: "~ξ- like ἐλπω";
    stem aorist_ind_pass: "~χθην- like εὐλογεω";
    ind_act_3: _, _, "~γουσῐ|~γουσῐν";
    ind_mid_2: "~γῃ|~γει", _, _;
    subj_act_3: _, _, "~γωσῐ|~γωσῐν";
    imperfect_act_1: "~γον", _, _;
    imperfect_act_2: "~γες", "~γετον", "~γετε";
    imperfect_act_3: "~γε", _, "~γον";
    imperfect_mid_1: "~γομην", _, "~γομεθᾰ";
    imperfect_mid_2: "~γου", _, _;
    imperfect_mid_3: "~γετο", _, "~γοντο";
    imperfect_pass_2: _, "~γεσθον", "~γεσθε";
    imperfect_pass_3: _, "~γεσθην", _;
    future_ind_act_3: _, _, "~ξουσῐ|~ξουσῐν";
    future_ind_mid_2: "~ξῃ|~ξει", _, _;
    perfect_ind_mid_1: _, _, "~γμεθᾰ";
    perfect_ind_mid_2: "~ξαι", "~χθον", _;
    perfect_ind_mid_3: "~κται", _, "~χᾰται";
    perfect_ind_pass_1: "~γμαι", _, _;
    perfect_ind_pass_2: _, _, "~χθε";
    perfect_imp_mid_2: _, _, "~χθε";
    perfect_imp_mid_3: _, _, "~χθων";
    perfect_imp_pass_2: "~ξο", _, "~χθε";
    perfect_imp_pass_3: "~χθω", "~χθων", "~χθων";
    pluperfect_mid_1: "~γμην", _, _;
    pluperfect_mid_2: "~ξο", "~χθον", _;
    pluperfect_mid_3: "~κτο", _, "~χᾰτο";
    pluperfect_pass_1: _, _, "~γμεθᾰ";
    pluperfect_pass_2: _, _, "~χθε";
    pluperfect_pass_3: _, "~χθην", _;
    infinitive_perf: _, "~χθαι", _;
    participle_perf: _, _, "~γμενος";
    ),
    // Στελλω: 4 words (στελλω, ἀγγελλω, τελλω, σκελλω). Like Καθιζω, except in 147 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("στελλω", 4, "~~λω", like Καθιζω;
    stem future_pass: "~~θησ- like ἐχω";
    stem aorist_ind: "~ι~- like ἐχω";
    ind_act_1: "~~λω", _, "~~λομεν";
    ind_act_2: "~~λεις", "~~λετον", _;
    ind_act_3: "~~λει", _, "~~λουσῐ";
    ind_mid_1: _, _, "~~λομεθᾰ";
    ind_mid_2: "~~λῃ", _, _;
    ind_mid_3: _, "~~λεσθον", _;
    ind_pass_1: "~~λομαι", _, _;
    ind_pass_2: _, "~~λεσθον", "~~λεσθε";
    ind_pass_3: "~~λεται", "~~λεσθον", "~~λονται";
    subj_act_1: "~~λω", _, "~~λωμεν";
    subj_act_2: "~~λῃς", _, _;
    subj_act_3: "~~λῃ", "~~λητον", "~~λωσῐ";
    subj_mid_1: _, _, "~~λωμεθᾰ";
    subj_mid_2: _, "~~λησθον", _;
    subj_mid_3: "~~ληται", _, "~~λωνται";
    subj_pass_1: "~~λωμαι", _, _;
    subj_pass_2: _, _, "~~λησθε";
    opt_act_1: "~~λοιμῐ", _, "~~λοιμεν";
    opt_act_2: "~~λοις", "~~λοιτον", _;
    opt_act_3: "~~λοι", "~~λοιτην", "~~λοιεν";
    opt_mid_1: "~~λοιμην", _, "~~λοιμεθᾰ";
    opt_mid_2: "~~λοιο", _, "~~λοισθε";
    opt_mid_3: "~~λοιτο", "~~λοισθην", "~~λοιντο";
    opt_pass_2: _, "~~λοισθον", _;
    imp_act_2: "~~λε", _, "~~λετε";
    imp_act_3: _, _, "~~λοντων";
    imp_mid_2: "~~λου", "~~λεσθον", "~~λεσθε";
    imp_mid_3: _, _, "~~λεσθων";
    imp_pass_2: _, "~~λεσθον", "~~λεσθε";
    imp_pass_3: "~~λεσθω", "~~λεσθων", "~~λεσθων";
    imperfect_act_1: "ἐ~~λον", _, _;
    imperfect_act_2: "ἐ~~λες", "ἐ~~λετον", "ἐ~~λετε";
    imperfect_act_3: "ἐ~~λε", _, "ἐ~~λον";
    imperfect_mid_1: "ἐ~~λομην", _, "ἐ~~λομεθᾰ";
    imperfect_mid_2: "ἐ~~λου", _, _;
    imperfect_mid_3: "ἐ~~λετο", _, "ἐ~~λοντο";
    imperfect_pass_2: _, "ἐ~~λεσθον", "ἐ~~λεσθε";
    imperfect_pass_3: _, "ἐ~~λεσθην", _;
    future_ind_act_3: _, _, "~~ουσῐ";
    future_ind_mid_2: "~~ει", _, _;
    future_opt_act_1: "~~οιην", _, "~~οιμεν";
    future_opt_act_2: "~~οιης", "~~οιτον", derived;
    future_opt_act_3: "~~οιη", "~~οιτην", derived;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ι~ω", _, _;
    aorist_subj_act_2: "~ι~ῃς", "~ι~ητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~ι~ωμαι", _, _;
    aorist_subj_mid_2: "~ι~ῃ", "~ι~ησθον", _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_1: "~ι~αιμῐ", _, "~ι~αιμεν";
    aorist_opt_act_2: "~ι~ειᾰς", _, _;
    aorist_opt_act_3: "~ι~ειε", _, "~ι~ειᾰν";
    aorist_opt_mid_1: "~ι~αιμην", _, _;
    aorist_imp_act_2: "~ι~ον", _, "~ι~ᾰτε";
    aorist_imp_act_3: _, _, "~ι~ᾰντων";
    aorist_imp_mid_2: "~ι~αι", "~ι~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ι~ᾰσθων";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: "~τα~κᾰ", _, "~τα~κᾰμεν";
    perfect_ind_act_2: _, "~τα~κᾰτον", _;
    perfect_ind_act_3: _, _, "~τα~κᾱσῐ";
    perfect_ind_mid_1: _, _, "~τα~μεθᾰ";
    perfect_ind_mid_2: "~τα~σαι", "~τα~θον", _;
    perfect_ind_mid_3: "~τα~ται", _, "~τα~ᾰται";
    perfect_ind_pass_1: "~τα~μαι", _, _;
    perfect_ind_pass_2: _, _, "~τα~θε";
    perfect_subj_act_1: "~τα~κω", _, _;
    perfect_subj_act_2: "~τα~κῃς", "~τα~κητον", "~τα~κητε";
    perfect_subj_act_3: "~τα~κῃ", _, "~τα~κωσῐ";
    perfect_imp_act_2: "~τα~κε", _, _;
    perfect_imp_mid_2: _, _, "~τα~θε";
    perfect_imp_mid_3: _, _, "~τα~θων";
    perfect_imp_pass_2: "~τα~σο", _, "~τα~θε";
    perfect_imp_pass_3: "~τα~θω", "~τα~θων", "~τα~θων";
    pluperfect_act_1: "ἐ~τα~κειν", _, _;
    pluperfect_mid_1: "ἐ~τα~μην", _, _;
    pluperfect_mid_2: "ἐ~τα~σο", "ἐ~τα~θον", _;
    pluperfect_mid_3: "ἐ~τα~το", _, "ἐ~τα~ᾰτο";
    pluperfect_pass_1: _, _, "ἐ~τα~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~τα~θε";
    pluperfect_pass_3: _, "ἐ~τα~θην", _;
    infinitive_pres: "~~λειν", _, "~~λεσθαι";
    infinitive_aor: "~ι~αι", _, _;
    infinitive_perf: _, "~τᾱ~θαι", _;
    participle_pres: "~~λων", _, "~~λομενος";
    participle_aor: "~ι~ᾱς", "~ι~ᾰμενος", _;
    participle_perf: "~τα~κως", _, "~τα~μενος";
    ),
    // Πᾰροξῡνω: 4 words (παροτρῡνω, ἐποτρῡνω, πᾰροξῡνω, ἐξοτρῡνω). Like Ἀποκωλῡω, except in 140 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πᾰροξῡνω", 4, "~ο~ῡνω", like Ἀποκωλῡω;
    stem future_pass: "~ο~ῠνθησ- like ἐχω";
    stem aorist_ind: "~ω~ῡν- like ἐλπω";
    stem aorist_ind_pass: "~ω~ῠνθην- like εὐλογεω";
    stem aorist_pass: "~ο~ῠνθ- like ἐχω";
    ind_act_1: "~ο~ῡνω", _, "~ο~ῡνομεν";
    ind_act_2: "~ο~ῡνεις", "~ο~ῡνετον", _;
    ind_act_3: "~ο~ῡνει", _, "~ο~ῡνουσῐ";
    ind_mid_1: _, _, "~ο~ῡνομεθᾰ";
    ind_mid_2: "~ο~ῡνῃ", _, _;
    ind_mid_3: _, "~ο~ῡνεσθον", _;
    ind_pass_1: "~ο~ῡνομαι", _, _;
    ind_pass_2: _, "~ο~ῡνεσθον", "~ο~ῡνεσθε";
    ind_pass_3: "~ο~ῡνεται", "~ο~ῡνεσθον", "~ο~ῡνονται";
    subj_act_1: "~ο~ῡνω", _, "~ο~ῡνωμεν";
    subj_act_2: "~ο~ῡνῃς", _, _;
    subj_act_3: "~ο~ῡνῃ", "~ο~ῡνητον", "~ο~ῡνωσῐ";
    subj_mid_1: _, _, "~ο~ῡνωμεθᾰ";
    subj_mid_2: _, "~ο~ῡνησθον", _;
    subj_mid_3: "~ο~ῡνηται", _, "~ο~ῡνωνται";
    subj_pass_1: "~ο~ῡνωμαι", _, _;
    subj_pass_2: _, _, "~ο~ῡνησθε";
    opt_act_1: "~ο~ῡνοιμῐ", _, "~ο~ῡνοιμεν";
    opt_act_2: "~ο~ῡνοις", "~ο~ῡνοιτον", _;
    opt_act_3: "~ο~ῡνοι", "~ο~ῡνοιτην", "~ο~ῡνοιεν";
    opt_mid_1: "~ο~ῡνοιμην", _, "~ο~ῡνοιμεθᾰ";
    opt_mid_2: "~ο~ῡνοιο", _, "~ο~ῡνοισθε";
    opt_mid_3: "~ο~ῡνοιτο", "~ο~ῡνοισθην", "~ο~ῡνοιντο";
    opt_pass_2: _, "~ο~ῡνοισθον", _;
    imp_act_2: "~ο~ῡνε", _, "~ο~ῡνετε";
    imp_act_3: _, _, "~ο~ῡνοντων";
    imp_mid_2: "~ο~ῡνου", "~ο~ῡνεσθον", "~ο~ῡνεσθε";
    imp_mid_3: _, _, "~ο~ῡνεσθων";
    imp_pass_2: _, "~ο~ῡνεσθον", "~ο~ῡνεσθε";
    imp_pass_3: "~ο~ῡνεσθω", "~ο~ῡνεσθων", "~ο~ῡνεσθων";
    imperfect_act_1: "~ω~ῡνον", _, _;
    imperfect_act_2: "~ω~ῡνες", "~ω~ῡνετον", "~ω~ῡνετε";
    imperfect_act_3: "~ω~ῡνε", _, "~ω~ῡνον";
    imperfect_mid_1: "~ω~ῡνομην", _, "~ω~ῡνομεθᾰ";
    imperfect_mid_2: "~ω~ῡνου", _, _;
    imperfect_mid_3: "~ω~ῡνετο", _, "~ω~ῡνοντο";
    imperfect_pass_2: _, "~ω~ῡνεσθον", "~ω~ῡνεσθε";
    imperfect_pass_3: _, "~ω~ῡνεσθην", _;
    future_ind_act_1: "~ο~ῠνω", _, "~ο~ῠνουμεν";
    future_ind_act_2: "~ο~ῠνεις", "~ο~ῠνειτον", _;
    future_ind_act_3: _, _, "~ο~ῠνουσῐ";
    future_ind_mid_1: "~ο~ῠνουμαι", _, _;
    future_ind_mid_2: "~ο~ῠνει", "~ο~ῠνεισθον", _;
    future_ind_mid_3: "~ο~ῠνειται", _, _;
    future_opt_act_1: "~ο~ῠνοιην", _, "~ο~ῠνοιμεν";
    future_opt_act_2: "~ο~ῠνοιης", "~ο~ῠνοιτον", _;
    future_opt_act_3: "~ο~ῠνοιη", "~ο~ῠνοιτην", _;
    future_opt_mid_1: "~ο~ῠνοιμην", _, "~ο~ῠνοιμεθᾰ";
    future_opt_mid_3: _, "~ο~ῠνοισθην", _;
    aorist_subj_act_1: "~ο~ῡνω", _, _;
    aorist_subj_act_2: "~ο~ῡνῃς", "~ο~ῡνητον", _;
    aorist_subj_mid_1: "~ο~ῡνωμαι", _, _;
    aorist_subj_mid_2: "~ο~ῡνῃ", "~ο~ῡνησθον", _;
    aorist_opt_act_1: "~ο~ῡναιμῐ", _, "~ο~ῡναιμεν";
    aorist_opt_act_2: "~ο~ῡνειᾰς", _, _;
    aorist_opt_act_3: "~ο~ῡνειε", _, "~ο~ῡνειᾰν";
    aorist_opt_mid_1: "~ο~ῡναιμην", _, _;
    aorist_imp_act_2: "~ο~ῡνον", _, "~ο~ῡνᾰτε";
    aorist_imp_act_3: _, _, "~ο~ῡνᾰντων";
    aorist_imp_mid_2: "~ο~ῡναι", "~ο~ῡνᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ο~ῡνᾰσθων";
    perfect_ind_act_1: "~ω~ῠγκᾰ", _, "~ω~ῠγκᾰμεν";
    perfect_ind_act_2: _, "~ω~ῠγκᾰτον", _;
    perfect_ind_act_3: _, _, "~ω~ῠγκᾱσῐ";
    perfect_ind_mid_1: _, _, "~ω~ῠμμεθᾰ";
    perfect_ind_mid_2: "~ω~ῠψαι", "~ω~ῠφθον", _;
    perfect_ind_mid_3: "~ω~ῠπται", _, "~ω~ῠπᾰται";
    perfect_ind_pass_1: "~ω~ῠμμαι", _, _;
    perfect_ind_pass_2: _, _, "~ω~ῠφθε";
    perfect_subj_act_1: "~ω~ῠγκω", _, _;
    perfect_subj_act_2: "~ω~ῠγκῃς", "~ω~ῠγκητον", "~ω~ῠγκητε";
    perfect_subj_act_3: "~ω~ῠγκῃ", _, "~ω~ῠγκωσῐ";
    perfect_imp_act_2: "~ω~ῠγκε", _, _;
    perfect_imp_mid_2: _, _, "~ω~ῠφθε";
    perfect_imp_mid_3: _, _, "~ω~ῠφθων";
    perfect_imp_pass_2: "~ω~ῠψο", _, "~ω~ῠφθε";
    perfect_imp_pass_3: "~ω~ῠφθω", "~ω~ῠφθων", "~ω~ῠφθων";
    infinitive_pres: "~ο~ῡνειν", _, "~ο~ῡνεσθαι";
    infinitive_fut: "~ο~ῠνειν", _, _;
    infinitive_aor: "~ο~ῡναι", _, _;
    infinitive_perf: _, "~ω~ῠφθαι", _;
    participle_pres: "~ο~ῡνων", _, "~ο~ῡνομενος";
    participle_aor: "~ο~ῡνᾱς", "~ο~ῡνᾰμενος", _;
    participle_perf: "~ω~ῠγκως", _, "~ω~ῠμμενος";
    ),
    // Ἐλᾰσσοω: 4 words (ἐρημοω, ἐλᾰσσοω, ἐλευθεροω, ἐπουλοω). Like Βουλευω, except in 151 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐλᾰσσοω", 4, "ἐ~οω", like Βουλευω;
    stem future_pass: "~ωθησ- like ἐλπῐζω";
    stem aorist_ind: "~ωσ- like ἐλπῐζω";
    stem aorist_ind_pass: "~ωθην- like ἀθῡρω";
    stem aorist_pass: "~ωθ- like ἐλπῐζω";
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
    opt_mid_2: "ἐ~οιο", _, "ἐ~οισθε";
    opt_mid_3: "ἐ~οιτο", "ἐ~οισθην", "ἐ~οιντο";
    opt_pass_2: _, "ἐ~οισθον", _;
    imp_act_2: "ἐ~ου", _, "ἐ~ουτε";
    imp_act_3: _, _, "ἐ~ουντων";
    imp_mid_2: "ἐ~ου", "ἐ~ουσθον", "ἐ~ουσθε";
    imp_mid_3: _, _, "ἐ~ουσθων";
    imp_pass_2: _, "ἐ~ουσθον", "ἐ~ουσθε";
    imp_pass_3: "ἐ~ουσθω", "ἐ~ουσθων", "ἐ~ουσθων";
    imperfect_act_1: "ἠ~ουν", _, _;
    imperfect_act_2: "ἠ~ους", "ἠ~ουτον", "ἠ~ουτε";
    imperfect_act_3: "ἠ~ου", _, "ἠ~ουν";
    imperfect_mid_1: "ἠ~ουμην", _, "ἠ~ουμεθᾰ";
    imperfect_mid_2: "ἠ~ου", _, _;
    imperfect_mid_3: "ἠ~ουτο", _, "ἠ~ουντο";
    imperfect_pass_2: _, "ἠ~ουσθον", "ἠ~ουσθε";
    imperfect_pass_3: _, "ἠ~ουσθην", _;
    future_ind_act_1: "ἐ~ωσω", _, "ἐ~ωσομεν";
    future_ind_act_2: "ἐ~ωσεις", "ἐ~ωσετον", _;
    future_ind_act_3: _, _, "ἐ~ωσουσῐ|ἐ~ωσουσῐν";
    future_ind_mid_1: "ἐ~ωσομαι", _, _;
    future_ind_mid_2: "ἐ~ωσῃ|ἐ~ωσει", "ἐ~ωσεσθον", _;
    future_ind_mid_3: "ἐ~ωσεται", _, _;
    future_opt_act_1: "ἐ~ωσοιμῐ", _, "ἐ~ωσοιμεν";
    future_opt_act_2: "ἐ~ωσοις", "ἐ~ωσοιτον", _;
    future_opt_act_3: "ἐ~ωσοι", "ἐ~ωσοιτην", _;
    future_opt_mid_1: "ἐ~ωσοιμην", _, "ἐ~ωσοιμεθᾰ";
    future_opt_mid_3: _, "ἐ~ωσοισθην", _;
    aorist_subj_act_1: "ἐ~ωσω", _, _;
    aorist_subj_act_2: "ἐ~ωσῃς", "ἐ~ωσητον", _;
    aorist_subj_act_3: _, _, "ἐ~ωσωσῐ|ἐ~ωσωσῐν";
    aorist_subj_mid_1: "ἐ~ωσωμαι", _, _;
    aorist_subj_mid_2: "ἐ~ωσῃ", "ἐ~ωσησθον", _;
    aorist_opt_act_1: "ἐ~ωσαιμῐ", _, "ἐ~ωσαιμεν";
    aorist_opt_act_2: "ἐ~ωσειᾰς|ἐ~ωσαις", _, _;
    aorist_opt_act_3: "ἐ~ωσειε|ἐ~ωσειεν|ἐ~ωσαι", _, "ἐ~ωσειᾰν|ἐ~ωσαιεν";
    aorist_opt_mid_1: "ἐ~ωσαιμην", _, _;
    aorist_imp_act_2: "ἐ~ωσον", _, "ἐ~ωσᾰτε";
    aorist_imp_act_3: _, _, "ἐ~ωσᾰντων";
    aorist_imp_mid_2: "ἐ~ωσαι", "ἐ~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἐ~ωσᾰσθων";
    perfect_ind_act_1: "ἠ~ωκᾰ", _, "ἠ~ωκᾰμεν";
    perfect_ind_act_2: _, "ἠ~ωκᾰτον", _;
    perfect_ind_act_3: _, _, "ἠ~ωκᾱσῐ";
    perfect_ind_mid_1: _, _, "ἠ~ωμεθᾰ";
    perfect_ind_mid_2: "ἠ~ωσαι", "ἠ~ωσθον", _;
    perfect_ind_mid_3: "ἠ~ωται", _, "ἠ~ωνται";
    perfect_ind_pass_1: "ἠ~ωμαι", _, _;
    perfect_ind_pass_2: _, _, "ἠ~ωσθε";
    perfect_subj_act_1: "ἠ~ωκω", _, _;
    perfect_subj_act_2: "ἠ~ωκῃς", "ἠ~ωκητον", "ἠ~ωκητε";
    perfect_subj_act_3: "ἠ~ωκῃ", _, "ἠ~ωκωσῐ";
    perfect_imp_act_2: "ἠ~ωκε", _, _;
    perfect_imp_mid_2: _, _, "ἠ~ωσθε";
    perfect_imp_mid_3: _, _, "ἠ~ωσθων";
    perfect_imp_pass_2: "ἠ~ωσο", _, "ἠ~ωσθε";
    perfect_imp_pass_3: "ἠ~ωσθω", "ἠ~ωσθων", "ἠ~ωσθων";
    pluperfect_act_1: "ἠ~ωκειν", _, _;
    pluperfect_mid_1: "ἠ~ωμην", _, _;
    pluperfect_mid_2: "ἠ~ωσο", "ἠ~ωσθον", _;
    pluperfect_mid_3: "ἠ~ωτο", _, "ἠ~ωντο";
    pluperfect_pass_1: _, _, "ἠ~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἠ~ωσθε";
    pluperfect_pass_3: _, "ἠ~ωσθην", _;
    infinitive_pres: "ἐ~ουν", _, "ἐ~ουσθαι";
    infinitive_fut: "ἐ~ωσειν", _, _;
    infinitive_aor: "ἐ~ωσαι", _, _;
    infinitive_perf: _, "ἠ~ωσθαι", _;
    participle_pres: "ἐ~ων", _, "ἐ~ουμενος";
    participle_aor: "ἐ~ωσᾱς", "ἐ~ωσᾰμενος", _;
    participle_perf: "ἠ~ωκως", _, "ἠ~ωμενος";
    ),
    // Ἐρωτᾰω: 4 words (ἐρωτᾰω, ἐρᾰω, ἐρευνᾰω, ἐμπολᾰω). Like Ληγω, except in 152 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐρωτᾰω", 4, "ἐ~ᾰω", like Ληγω;
    stem aorist_ind: "~ησ- like ἀθῡρω";
    stem aorist_ind_pass: "~ηθην- like ἀθῡρω";
    stem aorist_pass: "~ηθ- like ἐρωτᾰω";
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
    opt_mid_2: "ἐ~ῳο", _, "ἐ~ῳσθε";
    opt_mid_3: "ἐ~ῳτο", "ἐ~ῳσθην", "ἐ~ῳντο";
    opt_pass_2: _, "ἐ~ῳσθον", _;
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
    future_ind_act_1: "ἐ~ησω", _, "ἐ~ησομεν";
    future_ind_act_2: "ἐ~ησεις", "ἐ~ησετον", _;
    future_ind_act_3: _, _, "ἐ~ησουσῐ";
    future_ind_mid_1: "ἐ~ησομαι", _, _;
    future_ind_mid_2: "ἐ~ησῃ", "ἐ~ησεσθον", _;
    future_ind_mid_3: "ἐ~ησεται", _, _;
    future_ind_pass_1: "ἐ~ηθησομαι", _, _;
    future_ind_pass_3: _, "ἐ~ηθησεσθον", _;
    future_opt_act_1: "ἐ~ησοιμῐ", _, "ἐ~ησοιμεν";
    future_opt_act_2: "ἐ~ησοις", "ἐ~ησοιτον", _;
    future_opt_act_3: "ἐ~ησοι", "ἐ~ησοιτην", _;
    future_opt_mid_1: "ἐ~ησοιμην", _, "ἐ~ησοιμεθᾰ";
    future_opt_mid_3: _, "ἐ~ησοισθην", _;
    aorist_subj_act_1: "ἐ~ησω", _, _;
    aorist_subj_act_2: "ἐ~ησῃς", "ἐ~ησητον", _;
    aorist_subj_mid_1: "ἐ~ησωμαι", _, _;
    aorist_subj_mid_2: "ἐ~ησῃ", "ἐ~ησησθον", _;
    aorist_opt_act_1: "ἐ~ησαιμῐ", _, "ἐ~ησαιμεν";
    aorist_opt_act_2: "ἐ~ησειᾰς", _, _;
    aorist_opt_act_3: "ἐ~ησειε", _, "ἐ~ησειᾰν";
    aorist_opt_mid_1: "ἐ~ησαιμην", _, _;
    aorist_imp_act_2: "ἐ~ησον", _, "ἐ~ησᾰτε";
    aorist_imp_act_3: _, _, "ἐ~ησᾰντων";
    aorist_imp_mid_2: "ἐ~ησαι", "ἐ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἐ~ησᾰσθων";
    perfect_ind_act_1: "ἠ~ηκᾰ", _, "ἠ~ηκᾰμεν";
    perfect_ind_act_2: _, "ἠ~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "ἠ~ηκᾱσῐ";
    perfect_ind_mid_1: _, _, "ἠ~ημεθᾰ";
    perfect_ind_mid_2: "ἠ~ησαι", "ἠ~ησθον", _;
    perfect_ind_mid_3: "ἠ~ηται", _, "ἠ~ηνται";
    perfect_ind_pass_1: "ἠ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "ἠ~ησθε";
    perfect_subj_act_1: "ἠ~ηκω", _, _;
    perfect_subj_act_2: "ἠ~ηκῃς", "ἠ~ηκητον", "ἠ~ηκητε";
    perfect_subj_act_3: "ἠ~ηκῃ", _, "ἠ~ηκωσῐ";
    perfect_imp_act_2: "ἠ~ηκε", _, _;
    perfect_imp_mid_2: _, _, "ἠ~ησθε";
    perfect_imp_mid_3: _, _, "ἠ~ησθων";
    perfect_imp_pass_2: "ἠ~ησο", _, "ἠ~ησθε";
    perfect_imp_pass_3: "ἠ~ησθω", "ἠ~ησθων", "ἠ~ησθων";
    pluperfect_act_1: "ἠ~ηκειν", _, _;
    pluperfect_mid_1: "ἠ~ημην", _, _;
    pluperfect_mid_2: "ἠ~ησο", "ἠ~ησθον", _;
    pluperfect_mid_3: "ἠ~ητο", _, "ἠ~ηντο";
    pluperfect_pass_1: _, _, "ἠ~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἠ~ησθε";
    pluperfect_pass_3: _, "ἠ~ησθην", _;
    infinitive_pres: "ἐ~ᾱν", _, "ἐ~ᾱσθαι";
    infinitive_fut: "ἐ~ησειν", _, _;
    infinitive_aor: "ἐ~ησαι", _, _;
    infinitive_perf: _, "ἠ~ησθαι", _;
    participle_pres: "ἐ~ων", _, "ἐ~ωμενος";
    participle_aor: "ἐ~ησᾱς", "ἐ~ησᾰμενος", _;
    participle_perf: "ἠ~ηκως", _, "ἠ~ημενος";
    ),
    // Πᾰρᾰγῐγνομαι: 4 words (ὑπογῐγνομαι, πᾰρᾰγῐγνομαι, περῐγῐγνομαι, ῠγγῐγνομαι). Like Προκᾰλεω, except in 174 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πᾰρᾰγῐγνομαι", 4, "~~γῐγ~ομαι", like Προκᾰλεω;
    stem future_pass: "~~γε~ηθησ- like ἐχω";
    stem aorist_ind: "~νε~ε~- like πᾰρᾰγῐγνομαι";
    stem aorist_ind_pass: "~νε~ε~ηθην- like εὐλογεω";
    stem aorist_pass: "~~γε~ηθ- like ἐχω";
    ind_act_1: "~~γῐγ~ομαι", _, derived;
    ind_act_2: derived, derived, _;
    ind_act_3: derived, _, derived;
    ind_mid_1: "~~γῐγ~ομαι", _, "~~γῐγ~ομεθᾰ";
    ind_mid_2: "~~γῐγ~ῃ", _, _;
    ind_mid_3: _, "~~γῐγ~εσθον", _;
    ind_pass_1: derived, _, _;
    ind_pass_2: _, "~~γῐγ~εσθον", "~~γῐγ~εσθε";
    ind_pass_3: "~~γῐγ~εται", "~~γῐγ~εσθον", "~~γῐγ~ονται";
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_1: "~~γῐγ~ωμαι", _, "~~γῐγ~ωμεθᾰ";
    subj_mid_2: "~~γῐγ~ῃ", "~~γῐγ~ησθον", _;
    subj_mid_3: "~~γῐγ~ηται", _, "~~γῐγ~ωνται";
    subj_pass_1: derived, _, _;
    subj_pass_2: "~~γῐγ~ῃ", _, "~~γῐγ~ησθε";
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    opt_mid_1: "~~γῐγ~οιμην", _, "~~γῐγ~οιμεθᾰ";
    opt_mid_2: "~~γῐγ~οιο", _, "~~γῐγ~οισθε";
    opt_mid_3: "~~γῐγ~οιτο", "~~γῐγ~οισθην", "~~γῐγ~οιντο";
    opt_pass_2: _, "~~γῐγ~οισθον", _;
    imp_act_2: derived, _, derived;
    imp_act_3: _, _, derived;
    imp_mid_2: "~~γῐγ~ου", "~~γῐγ~εσθον", "~~γῐγ~εσθε";
    imp_mid_3: "~~γῐγ~εσθω", _, "~~γῐγ~εσθων";
    imp_pass_2: _, "~~γῐγ~εσθον", "~~γῐγ~εσθε";
    imp_pass_3: derived, "~~γῐγ~εσθων", "~~γῐγ~εσθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    imperfect_mid_1: "~νε%ῐ~~ομην", _, "~νε%ῐ~~ομεθᾰ";
    imperfect_mid_2: "~νε%ῐ~~ου", "~νε%ῐ~~εσθον", _;
    imperfect_mid_3: "~νε%ῐ~~ετο", _, "~νε%ῐ~~οντο";
    imperfect_pass_2: _, derived, "~νε%ῐ~~εσθε";
    imperfect_pass_3: _, "~νε%ῐ~~εσθην", _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, _;
    future_ind_act_3: _, _, derived;
    future_ind_mid_1: "~~γε~ησομαι", _, _;
    future_ind_mid_2: "~~γε~ησῃ", "~~γε~ησεσθον", _;
    future_ind_mid_3: "~~γε~ησεται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, _;
    future_opt_act_3: derived, derived, _;
    future_opt_mid_1: "~~γε~ησοιμην", _, "~~γε~ησοιμεθᾰ";
    future_opt_mid_3: _, "~~γε~ησοισθην", _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~~γε~ωμαι", _, _;
    aorist_subj_mid_2: "~~γε~ῃ", "~~γε~ησθον", _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_opt_mid_1: "~~γε~οιμην", _, _;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_mid_2: "~~γε~ου", "~~γε~εσθον", _;
    aorist_imp_mid_3: _, _, "~~γε~εσθων";
    perfect_ind_act_1: "~~γεγο~ᾰ", _, "~~γεγο~ᾰμεν";
    perfect_ind_act_2: _, "~~γεγο~ᾰτον", _;
    perfect_ind_act_3: derived, _, "~~γεγο~ᾱσῐ";
    perfect_ind_mid_1: _, _, "~~γεγε~ημεθᾰ";
    perfect_ind_mid_2: "~~γεγε~ησαι", "~~γεγε~ησθον", _;
    perfect_ind_mid_3: "~~γεγε~ηται", _, "~~γεγε~ηνται";
    perfect_ind_pass_1: "~~γεγε~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~~γεγε~ησθε";
    perfect_subj_act_1: "~~γεγο~ω", _, _;
    perfect_subj_act_2: "~~γεγο~ῃς", "~~γεγο~ητον", "~~γεγο~ητε";
    perfect_subj_act_3: "~~γεγο~ῃ", _, "~~γεγο~ωσῐ";
    perfect_opt_act_1: derived, _, _;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: derived, _, _;
    perfect_imp_act_2: "~~γεγο~ε", _, _;
    perfect_imp_mid_2: _, _, "~~γεγε~ησθε";
    perfect_imp_mid_3: _, _, "~~γεγε~ησθων";
    perfect_imp_pass_2: "~~γεγε~ησο", _, "~~γεγε~ησθε";
    perfect_imp_pass_3: "~~γεγε~ησθω", "~~γεγε~ησθων", "~~γεγε~ησθων";
    pluperfect_act_1: "~νε~εγο~ειν", _, _;
    pluperfect_mid_1: "~νε~εγε~ημην", _, _;
    pluperfect_mid_2: "~νε~εγε~ησο", "~νε~εγε~ησθον", _;
    pluperfect_mid_3: "~νε~εγε~ητο", _, "~νε~εγε~ηντο";
    pluperfect_pass_1: _, _, "~νε~εγε~ημεθᾰ";
    pluperfect_pass_2: _, _, "~νε~εγε~ησθε";
    pluperfect_pass_3: _, "~νε~εγε~ησθην", _;
    infinitive_pres: derived, _, "~~γῐγ~εσθαι";
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    infinitive_perf: _, "~~γεγε~ησθαι", _;
    participle_pres: derived, _, "~~γῐγ~ομενος";
    participle_aor: derived, "~~γε~ομενος", _;
    participle_perf: "~~γεγο~ως", _, "~~γεγε~ημενος";
    ),
    // Κρουω: 4 words (ἀκουω, κρουω, κονῑω, ρουω). Like Πρεσβευω, except in 16 cells: future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, aorist_opt_pass_3 du, ….
    conjugation!("κρουω", 4, "~ω", like Πρεσβευω;
    future_ind_pass_2: derived, _, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    pluperfect_act_1: "%ε~κειν", _, _;
    pluperfect_mid_1: "%ε~σμην", _, _;
    pluperfect_mid_2: "%ε~σο", "%ε~σθον", _;
    pluperfect_mid_3: "%ε~στο", _, "%ε~σᾰτο";
    pluperfect_pass_1: _, _, "%ε~σμεθᾰ";
    pluperfect_pass_2: _, _, "%ε~σθε";
    pluperfect_pass_3: _, "%ε~σθην", _;
    ),
    // Δημοκρᾰτεομαι: 4 words (πλουτοκρᾰτεομαι, δημοκρᾰτεομαι, εὐλαβεομαι, ἀμηχᾰνοποιεομαι). Like Λᾰγοθηρεω, except in 30 cells: imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, imperfect_act_3 pl, ….
    conjugation!("δημοκρᾰτεομαι", 4, "~ᾰ~εομαι", like Λᾰγοθηρεω;
    stem aorist_ind: "~ᾰ~ησ- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~ηθην- like ἐχω";
    stem pluperfect_act: "~ᾰ~ηκ- like χρημᾰτῐζω";
    imperfect_act_1: "ἐ~ᾰ~ουν", _, _;
    imperfect_act_2: "ἐ~ᾰ~εις", "ἐ~ᾰ~ειτον", "ἐ~ᾰ~ειτε";
    imperfect_act_3: "ἐ~ᾰ~ει", _, "ἐ~ᾰ~ουν";
    imperfect_mid_1: "ἐ~ᾰ~ουμην", _, "ἐ~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~ου", _, _;
    imperfect_mid_3: "ἐ~ᾰ~ειτο", _, "ἐ~ᾰ~ουντο";
    imperfect_pass_2: _, "ἐ~ᾰ~εισθον", "ἐ~ᾰ~εισθε";
    imperfect_pass_3: _, "ἐ~ᾰ~εισθην", _;
    perfect_ind_act_1: "%ε~ᾰ~ηκᾰ", _, "%ε~ᾰ~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~ᾰ~ηκᾰτον", _;
    perfect_ind_act_3: "%ε~ᾰ~ηκε|%ε~ᾰ~ηκεν", _, "%ε~ᾰ~ηκᾱσῐ|%ε~ᾰ~ηκᾱσῐν";
    perfect_subj_act_1: "%ε~ᾰ~ηκω", _, _;
    perfect_subj_act_2: "%ε~ᾰ~ηκῃς", "%ε~ᾰ~ηκητον", "%ε~ᾰ~ηκητε";
    perfect_subj_act_3: "%ε~ᾰ~ηκῃ", _, "%ε~ᾰ~ηκωσῐ|%ε~ᾰ~ηκωσῐν";
    perfect_opt_act_1: "%ε~ᾰ~ηκοιμῐ|%ε~ᾰ~ηκοιην", _, _;
    perfect_opt_act_2: "%ε~ᾰ~ηκοις|%ε~ᾰ~ηκοιης", _, _;
    perfect_opt_act_3: "%ε~ᾰ~ηκοι|%ε~ᾰ~ηκοιη", _, _;
    perfect_imp_act_2: "%ε~ᾰ~ηκε", _, _;
    participle_perf: "%ε~ᾰ~ηκως", _, _;
    ),
    // Ἀνᾰλᾰμβᾰνω: 4 words (κᾰτᾰλᾰμβᾰνω, ἀνᾰλᾰμβᾰνω, μετᾰλαμβᾰνω, ἐπᾰνᾰλᾰμβᾰνω). Like Ἀειρω, except in 141 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀνᾰλᾰμβᾰνω", 4, "~ᾰλᾰμβᾰνω", like Ἀειρω;
    stem future_pass: "~ᾰληφθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ελᾰβ- like πᾰρεχω";
    stem aorist_ind_pass: "~εληφθην- like εὐλογεω";
    stem aorist_pass: "~ᾰληφθ- like ποιεω";
    stem pluperfect_act: "~ειληφ- like εὐλογεω";
    ind_act_1: "~ᾰλᾰμβᾰνω", _, "~ᾰλᾰμβᾰνομεν";
    ind_act_2: "~ᾰλᾰμβᾰνεις", "~ᾰλᾰμβᾰνετον", _;
    ind_act_3: "~ᾰλᾰμβᾰνει", _, "~ᾰλᾰμβᾰνουσῐ|~ᾰλᾰμβᾰνουσῐν";
    ind_mid_1: _, _, "~ᾰλᾰμβᾰνομεθᾰ";
    ind_mid_2: "~ᾰλᾰμβᾰνῃ|~ᾰλᾰμβᾰνει", _, _;
    ind_mid_3: _, "~ᾰλᾰμβᾰνεσθον", _;
    ind_pass_1: "~ᾰλᾰμβᾰνομαι", _, _;
    ind_pass_2: _, "~ᾰλᾰμβᾰνεσθον", "~ᾰλᾰμβᾰνεσθε";
    ind_pass_3: "~ᾰλᾰμβᾰνεται", "~ᾰλᾰμβᾰνεσθον", "~ᾰλᾰμβᾰνονται";
    subj_act_1: "~ᾰλᾰμβᾰνω", _, "~ᾰλᾰμβᾰνωμεν";
    subj_act_2: "~ᾰλᾰμβᾰνῃς", _, _;
    subj_act_3: "~ᾰλᾰμβᾰνῃ", "~ᾰλᾰμβᾰνητον", "~ᾰλᾰμβᾰνωσῐ|~ᾰλᾰμβᾰνωσῐν";
    subj_mid_1: _, _, "~ᾰλᾰμβᾰνωμεθᾰ";
    subj_mid_2: _, "~ᾰλᾰμβᾰνησθον", _;
    subj_mid_3: "~ᾰλᾰμβᾰνηται", _, "~ᾰλᾰμβᾰνωνται";
    subj_pass_1: "~ᾰλᾰμβᾰνωμαι", _, _;
    subj_pass_2: _, _, "~ᾰλᾰμβᾰνησθε";
    opt_act_1: "~ᾰλᾰμβᾰνοιμῐ", _, "~ᾰλᾰμβᾰνοιμεν";
    opt_act_2: "~ᾰλᾰμβᾰνοις", "~ᾰλᾰμβᾰνοιτον", _;
    opt_act_3: "~ᾰλᾰμβᾰνοι", "~ᾰλᾰμβᾰνοιτην", "~ᾰλᾰμβᾰνοιεν";
    opt_mid_1: "~ᾰλᾰμβᾰνοιμην", _, "~ᾰλᾰμβᾰνοιμεθᾰ";
    opt_mid_2: "~ᾰλᾰμβᾰνοιο", _, "~ᾰλᾰμβᾰνοισθε";
    opt_mid_3: "~ᾰλᾰμβᾰνοιτο", "~ᾰλᾰμβᾰνοισθην", "~ᾰλᾰμβᾰνοιντο";
    opt_pass_2: _, "~ᾰλᾰμβᾰνοισθον", _;
    imp_act_2: "~ᾰλᾰμβᾰνε", _, "~ᾰλᾰμβᾰνετε";
    imp_act_3: _, _, "~ᾰλᾰμβᾰνοντων";
    imp_mid_2: "~ᾰλᾰμβᾰνου", "~ᾰλᾰμβᾰνεσθον", "~ᾰλᾰμβᾰνεσθε";
    imp_mid_3: _, _, "~ᾰλᾰμβᾰνεσθων";
    imp_pass_2: _, "~ᾰλᾰμβᾰνεσθον", "~ᾰλᾰμβᾰνεσθε";
    imp_pass_3: "~ᾰλᾰμβᾰνεσθω", "~ᾰλᾰμβᾰνεσθων", "~ᾰλᾰμβᾰνεσθων";
    imperfect_act_1: "~ελᾰμβᾰνον", _, _;
    imperfect_act_2: "~ελᾰμβᾰνες", "~ελᾰμβᾰνετον", "~ελᾰμβᾰνετε";
    imperfect_act_3: "~ελᾰμβᾰνε|~ελᾰμβᾰνεν", _, "~ελᾰμβᾰνον";
    imperfect_mid_1: "~ελᾰμβᾰνομην", _, "~ελᾰμβᾰνομεθᾰ";
    imperfect_mid_2: "~ελᾰμβᾰνου", _, _;
    imperfect_mid_3: "~ελᾰμβᾰνετο", _, "~ελᾰμβᾰνοντο";
    imperfect_pass_2: _, "~ελᾰμβᾰνεσθον", "~ελᾰμβᾰνεσθε";
    imperfect_pass_3: _, "~ελᾰμβᾰνεσθην", _;
    future_ind_mid_1: "~ᾰληψομαι", _, _;
    future_ind_mid_2: "~ᾰληψῃ|~ᾰληψει", "~ᾰληψεσθον", _;
    future_ind_mid_3: "~ᾰληψεται", _, _;
    future_opt_mid_1: "~ᾰληψοιμην", _, "~ᾰληψοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰληψοισθην", _;
    aorist_subj_act_1: "~ᾰλᾰβω", _, _;
    aorist_subj_act_2: "~ᾰλᾰβῃς", "~ᾰλᾰβητον", _;
    aorist_subj_act_3: _, _, "~ᾰλᾰβωσῐ|~ᾰλᾰβωσῐν";
    aorist_subj_mid_1: "~ᾰλᾰβωμαι", _, _;
    aorist_subj_mid_2: "~ᾰλᾰβῃ", "~ᾰλᾰβησθον", _;
    aorist_opt_act_1: "~ᾰλᾰβοιμῐ", _, "~ᾰλᾰβοιμεν";
    aorist_opt_act_2: "~ᾰλᾰβοις", _, _;
    aorist_opt_act_3: "~ᾰλᾰβοι", _, "~ᾰλᾰβοιεν";
    aorist_opt_mid_1: "~ᾰλᾰβοιμην", _, _;
    aorist_imp_act_2: "~ᾰλᾰβε", _, "~ᾰλᾰβετε";
    aorist_imp_act_3: _, _, "~ᾰλᾰβοντων";
    aorist_imp_mid_2: "~ᾰλᾰβου", "~ᾰλᾰβεσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰλᾰβεσθων";
    perfect_ind_act_1: "~ειληφᾰ", _, "~ειληφᾰμεν";
    perfect_ind_act_2: _, "~ειληφᾰτον", _;
    perfect_ind_act_3: "~ειληφε|~ειληφεν", _, "~ειληφᾱσῐ|~ειληφᾱσῐν";
    perfect_ind_mid_1: _, _, "~ειλημμεθᾰ";
    perfect_ind_mid_2: "~ειληψαι", "~ειληφθον", _;
    perfect_ind_mid_3: "~ειληπται", _, "~ειληφᾰται";
    perfect_ind_pass_1: "~ειλημμαι", _, _;
    perfect_ind_pass_2: _, _, "~ειληφθε";
    perfect_subj_act_1: "~ειληφω", _, _;
    perfect_subj_act_2: "~ειληφῃς", "~ειληφητον", "~ειληφητε";
    perfect_subj_act_3: "~ειληφῃ", _, "~ειληφωσῐ|~ειληφωσῐν";
    perfect_opt_act_1: "~ειληφοιμῐ|~ειληφοιην", _, _;
    perfect_opt_act_2: "~ειληφοις|~ειληφοιης", _, _;
    perfect_opt_act_3: "~ειληφοι|~ειληφοιη", _, _;
    perfect_imp_act_2: "~ειληφε", _, _;
    perfect_imp_mid_2: _, _, "~ειληφθε";
    perfect_imp_mid_3: _, _, "~ειληφθων";
    perfect_imp_pass_2: "~ειληψο", _, "~ειληφθε";
    perfect_imp_pass_3: "~ειληφθω", "~ειληφθων", "~ειληφθων";
    pluperfect_mid_1: "~ειλημμην", _, _;
    pluperfect_mid_2: "~ειληψο", "~ειληφθον", _;
    pluperfect_mid_3: "~ειληπτο", _, "~ειληφᾰτο";
    pluperfect_pass_1: _, _, "~ειλημμεθᾰ";
    pluperfect_pass_2: _, _, "~ειληφθε";
    pluperfect_pass_3: _, "~ειληφθην", _;
    infinitive_pres: "~ᾰλᾰμβᾰνειν", _, "~ᾰλᾰμβᾰνεσθαι";
    infinitive_aor: "~ᾰλᾰβειν", _, _;
    infinitive_perf: _, "~ειληφθαι", _;
    participle_pres: "~ᾰλᾰμβᾰνων", _, "~ᾰλᾰμβᾰνομενος";
    participle_aor: "~ᾰλᾰβων", "~ᾰλᾰβομενος", _;
    participle_perf: "~ειληφως", _, "~ειλημμενος";
    ),
    // Γυμνοω: 4 words (κυκλοω, γυμνοω, ἀπολυτροω, ἐκλυτροω). Like Ἀποκωλῡω, except in 141 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("γυμνοω", 4, "~υ~οω", like Ἀποκωλῡω;
    stem future_pass: "~ῠ~ωθησ- like ἐχω";
    stem aorist_ind: "~ῠ~ωσ- like ἐχω";
    stem aorist_ind_pass: "~ῠ~ωθην- like ἐχω";
    stem aorist_pass: "~ῠ~ηθ- like ἐχω";
    ind_act_1: "~ῠ~ω", _, "~ῠ~ουμεν";
    ind_act_2: "~ῠ~οις", "~ῠ~ουτον", _;
    ind_act_3: "~ῠ~οι", _, "~ῠ~ουσῐ|~ῠ~ουσῐν";
    ind_mid_1: _, _, "~ῠ~ουμεθᾰ";
    ind_mid_2: "~ῠ~οι", _, _;
    ind_mid_3: _, "~ῠ~ουσθον", _;
    ind_pass_1: "~ῠ~ουμαι", _, _;
    ind_pass_2: _, "~ῠ~ουσθον", "~ῠ~ουσθε";
    ind_pass_3: "~ῠ~ουται", "~ῠ~ουσθον", "~ῠ~ουνται";
    subj_act_1: "~ῠ~ω", _, "~ῠ~ωμεν";
    subj_act_2: "~ῠ~οις", _, _;
    subj_act_3: "~ῠ~οι", "~ῠ~ωτον", "~ῠ~ωσῐ|~ῠ~ωσῐν";
    subj_mid_1: _, _, "~ῠ~ωμεθᾰ";
    subj_mid_2: _, "~ῠ~ωσθον", _;
    subj_mid_3: "~ῠ~ωται", _, "~ῠ~ωνται";
    subj_pass_1: "~ῠ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῠ~ωσθε";
    opt_act_1: "~ῠ~οιην|~ῠ~οιμῐ", _, "~ῠ~οιμεν|~ῠ~οιημεν";
    opt_act_2: "~ῠ~οιης|~ῠ~οις", "~ῠ~οιτον|~ῠ~οιητον", "~ῠ~οιτε|~ῠ~οιητε";
    opt_act_3: "~ῠ~οιη|~ῠ~οι", "~ῠ~οιτην|~ῠ~οιητην", "~ῠ~οιεν|~ῠ~οιησᾰν";
    opt_mid_1: "~ῠ~οιμην", _, "~ῠ~οιμεθᾰ";
    opt_mid_2: "~ῠ~οιο", _, "~ῠ~οισθε";
    opt_mid_3: "~ῠ~οιτο", "~ῠ~οισθην", "~ῠ~οιντο";
    opt_pass_2: _, "~ῠ~οισθον", _;
    imp_act_2: "~ῠ~ου", _, "~ῠ~ουτε";
    imp_act_3: _, _, "~ῠ~ουντων";
    imp_mid_2: "~ῠ~ου", "~ῠ~ουσθον", "~ῠ~ουσθε";
    imp_mid_3: _, _, "~ῠ~ουσθων";
    imp_pass_2: _, "~ῠ~ουσθον", "~ῠ~ουσθε";
    imp_pass_3: "~ῠ~ουσθω", "~ῠ~ουσθων", "~ῠ~ουσθων";
    imperfect_act_1: "ἐ~ῠ~ουν", _, _;
    imperfect_act_2: "ἐ~ῠ~ους", "ἐ~ῠ~ουτον", "ἐ~ῠ~ουτε";
    imperfect_act_3: "ἐ~ῠ~ου", _, "ἐ~ῠ~ουν";
    imperfect_mid_1: "ἐ~ῠ~ουμην", _, "ἐ~ῠ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ῠ~ου", _, _;
    imperfect_mid_3: "ἐ~ῠ~ουτο", _, "ἐ~ῠ~ουντο";
    imperfect_pass_2: _, "ἐ~ῠ~ουσθον", "ἐ~ῠ~ουσθε";
    imperfect_pass_3: _, "ἐ~ῠ~ουσθην", _;
    future_ind_act_1: "~ῠ~ωσω", _, "~ῠ~ωσομεν";
    future_ind_act_2: "~ῠ~ωσεις", "~ῠ~ωσετον", _;
    future_ind_act_3: _, _, "~ῠ~ωσουσῐ";
    future_ind_mid_1: "~ῠ~ωσομαι", _, _;
    future_ind_mid_2: "~ῠ~ωσῃ", "~ῠ~ωσεσθον", _;
    future_ind_mid_3: "~ῠ~ωσεται", _, _;
    future_opt_act_1: "~ῠ~ωσοιμῐ", _, "~ῠ~ωσοιμεν";
    future_opt_act_2: "~ῠ~ωσοις", "~ῠ~ωσοιτον", _;
    future_opt_act_3: "~ῠ~ωσοι", "~ῠ~ωσοιτην", _;
    future_opt_mid_1: "~ῠ~ωσοιμην", _, "~ῠ~ωσοιμεθᾰ";
    future_opt_mid_3: _, "~ῠ~ωσοισθην", _;
    aorist_subj_act_1: "~ῠ~ωσω", _, _;
    aorist_subj_act_2: "~ῠ~ωσῃς", "~ῠ~ωσητον", _;
    aorist_subj_mid_1: "~ῠ~ωσωμαι", _, _;
    aorist_subj_mid_2: "~ῠ~ωσῃ", "~ῠ~ωσησθον", _;
    aorist_opt_act_1: "~ῠ~ωσαιμῐ", _, "~ῠ~ωσαιμεν";
    aorist_opt_act_2: "~ῠ~ωσειᾰς", _, _;
    aorist_opt_act_3: "~ῠ~ωσειε", _, "~ῠ~ωσειᾰν";
    aorist_opt_mid_1: "~ῠ~ωσαιμην", _, _;
    aorist_imp_act_2: "~ῠ~ωσον", _, "~ῠ~ωσᾰτε";
    aorist_imp_act_3: _, _, "~ῠ~ωσᾰντων";
    aorist_imp_mid_2: "~ῠ~ωσαι", "~ῠ~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῠ~ωσᾰσθων";
    perfect_ind_act_1: "~ε%ῠ~ωκᾰ", _, "~ε%ῠ~ωκᾰμεν";
    perfect_ind_act_2: _, "~ε%ῠ~ωκᾰτον", _;
    perfect_ind_act_3: _, _, "~ε%ῠ~ωκᾱσῐ";
    perfect_ind_mid_1: _, _, "~ε%ῠ~ωμεθᾰ";
    perfect_ind_mid_2: "~ε%ῠ~ωσαι", "~ε%ῠ~ωσθον", _;
    perfect_ind_mid_3: "~ε%ῠ~ωται", _, "~ε%ῠ~ωνται";
    perfect_ind_pass_1: "~ε%ῠ~ωμαι", _, _;
    perfect_ind_pass_2: _, _, "~ε%ῠ~ωσθε";
    perfect_subj_act_1: "~ε%ῠ~ωκω", _, _;
    perfect_subj_act_2: "~ε%ῠ~ωκῃς", "~ε%ῠ~ωκητον", "~ε%ῠ~ωκητε";
    perfect_subj_act_3: "~ε%ῠ~ωκῃ", _, "~ε%ῠ~ωκωσῐ";
    perfect_imp_act_2: "~ε%ῠ~ωκε", _, _;
    perfect_imp_mid_2: _, _, "~ε%ῠ~ωσθε";
    perfect_imp_mid_3: _, _, "~ε%ῠ~ωσθων";
    perfect_imp_pass_2: "~ε%ῠ~ωσο", _, "~ε%ῠ~ωσθε";
    perfect_imp_pass_3: "~ε%ῠ~ωσθω", "~ε%ῠ~ωσθων", "~ε%ῠ~ωσθων";
    infinitive_pres: "~ῠ~ουν", _, "~ῠ~ουσθαι";
    infinitive_fut: "~ῠ~ωσειν", _, _;
    infinitive_aor: "~ῠ~ωσαι", _, _;
    infinitive_perf: _, "~ε%ῠ~ωσθαι", _;
    participle_pres: "~ῠ~ων", _, "~ῠ~ουμενος";
    participle_aor: "~ῠ~ωσᾱς", "~ῠ~ωσᾰμενος", _;
    participle_perf: "~ε%ῠ~ωκως", _, "~ε%ῠ~ωμενος";
    ),
    // Δῐαιρω: 4 words (κᾰθαιρω, ἐπαιρω, δῐαιρω, ῥῠπαινω). Like Ἐπᾰϊω, except in 144 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("δῐαιρω", 4, "~αι~ω", like Ἐπᾰϊω;
    stem future_pass: "~ᾰ~θησ- like χρημᾰτῐζω";
    stem aorist_ind: "~η~- like εὐλογεω";
    stem aorist_ind_pass: "~η~θην- like εὐλογεω";
    stem aorist_pass: "~ᾰ~θ- like ποιεω";
    ind_act_1: "~αι~ω", _, "~αι~ομεν";
    ind_act_2: "~αι~εις", "~αι~ετον", _;
    ind_act_3: "~αι~ει", _, "~αι~ουσῐ|~αι~ουσῐν";
    ind_mid_1: _, _, "~αι~ομεθᾰ";
    ind_mid_2: "~αι~ῃ|~αι~ει", _, _;
    ind_mid_3: _, "~αι~εσθον", _;
    ind_pass_1: "~αι~ομαι", _, _;
    ind_pass_2: _, "~αι~εσθον", "~αι~εσθε";
    ind_pass_3: "~αι~εται", "~αι~εσθον", "~αι~ονται";
    subj_act_1: "~αι~ω", _, "~αι~ωμεν";
    subj_act_2: "~αι~ῃς", _, _;
    subj_act_3: "~αι~ῃ", "~αι~ητον", "~αι~ωσῐ|~αι~ωσῐν";
    subj_mid_1: _, _, "~αι~ωμεθᾰ";
    subj_mid_2: _, "~αι~ησθον", _;
    subj_mid_3: "~αι~ηται", _, "~αι~ωνται";
    subj_pass_1: "~αι~ωμαι", _, _;
    subj_pass_2: _, _, "~αι~ησθε";
    opt_act_1: "~αι~οιμῐ", _, "~αι~οιμεν";
    opt_act_2: "~αι~οις", "~αι~οιτον", _;
    opt_act_3: "~αι~οι", "~αι~οιτην", "~αι~οιεν";
    opt_mid_1: "~αι~οιμην", _, "~αι~οιμεθᾰ";
    opt_mid_2: "~αι~οιο", _, "~αι~οισθε";
    opt_mid_3: "~αι~οιτο", "~αι~οισθην", "~αι~οιντο";
    opt_pass_2: _, "~αι~οισθον", _;
    imp_act_2: "~αι~ε", _, "~αι~ετε";
    imp_act_3: _, _, "~αι~οντων";
    imp_mid_2: "~αι~ου", "~αι~εσθον", "~αι~εσθε";
    imp_mid_3: _, _, "~αι~εσθων";
    imp_pass_2: _, "~αι~εσθον", "~αι~εσθε";
    imp_pass_3: "~αι~εσθω", "~αι~εσθων", "~αι~εσθων";
    imperfect_act_1: "~ῃ~ον", _, _;
    imperfect_act_2: "~ῃ~ες", "~ῃ~ετον", "~ῃ~ετε";
    imperfect_act_3: "~ῃ~ε|~ῃ~εν", _, "~ῃ~ον";
    imperfect_mid_1: "~ῃ~ομην", _, "~ῃ~ομεθᾰ";
    imperfect_mid_2: "~ῃ~ου", _, _;
    imperfect_mid_3: "~ῃ~ετο", _, "~ῃ~οντο";
    imperfect_pass_2: _, "~ῃ~εσθον", "~ῃ~εσθε";
    imperfect_pass_3: _, "~ῃ~εσθην", _;
    future_ind_act_1: "~ᾰ~ω", _, "~ᾰ~ουμεν";
    future_ind_act_2: "~ᾰ~εις", "~ᾰ~ειτον", _;
    future_ind_act_3: _, _, "~ᾰ~ουσῐ|~ᾰ~ουσῐν";
    future_ind_mid_1: "~ᾰ~ουμαι", _, _;
    future_ind_mid_2: "~ᾰ~ει|~ᾰ~ῃ", "~ᾰ~εισθον", _;
    future_ind_mid_3: "~ᾰ~ειται", _, _;
    future_opt_act_1: "~ᾰ~οιην|~ᾰ~οιμῐ", _, "~ᾰ~οιμεν|~ᾰ~οιημεν";
    future_opt_act_2: "~ᾰ~οιης|~ᾰ~οις", "~ᾰ~οιτον|~ᾰ~οιητον", "~ᾰ~οιτε|~ᾰ~οιητε";
    future_opt_act_3: "~ᾰ~οιη|~ᾰ~οι", "~ᾰ~οιτην|~ᾰ~οιητην", "~ᾰ~οιεν|~ᾰ~οιησᾰν";
    future_opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~οισθην", _;
    aorist_subj_act_1: "~ᾰ~ω", _, _;
    aorist_subj_act_2: "~ᾰ~ῃς", "~ᾰ~ητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ῃ", "~ᾰ~ησθον", _;
    aorist_opt_act_1: "~ᾰ~αιμῐ", _, "~ᾰ~αιμεν";
    aorist_opt_act_2: "~ᾰ~ειᾰς|~ᾰ~αις", _, _;
    aorist_opt_act_3: "~ᾰ~ειε|~ᾰ~ειεν|~ᾰ~αι", _, "~ᾰ~ειᾰν|~ᾰ~αιεν";
    aorist_opt_mid_1: "~ᾰ~αιμην", _, _;
    aorist_imp_act_2: "~ᾰ~ον", _, "~ᾰ~ᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ᾰντων";
    aorist_imp_mid_2: "~ᾰ~αι", "~ᾰ~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ᾰσθων";
    perfect_ind_act_3: "~η~κε|~η~κεν", _, "~η~κᾱσῐ|~η~κᾱσῐν";
    perfect_ind_mid_1: _, _, "~η~μεθᾰ";
    perfect_ind_mid_2: _, "~η~θον", _;
    perfect_ind_mid_3: "~η~ται", _, "~η~ᾰται";
    perfect_ind_pass_1: "~η~μαι", _, _;
    perfect_ind_pass_2: _, _, "~η~θε";
    perfect_subj_act_3: _, _, "~η~κωσῐ|~η~κωσῐν";
    perfect_opt_act_1: "~η~κοιμῐ|~η~κοιην", _, _;
    perfect_opt_act_2: "~η~κοις|~η~κοιης", _, _;
    perfect_opt_act_3: "~η~κοι|~η~κοιη", _, _;
    perfect_imp_mid_2: _, _, "~η~θε";
    perfect_imp_mid_3: _, _, "~η~θων";
    perfect_imp_pass_2: _, _, "~η~θε";
    perfect_imp_pass_3: "~η~θω", "~η~θων", "~η~θων";
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_pres: "~αι~ειν", _, "~αι~εσθαι";
    infinitive_fut: "~ᾰ~ειν", _, _;
    infinitive_aor: "~ᾰ~αι", _, _;
    infinitive_perf: _, "~η~θαι", _;
    participle_pres: "~αι~ων", _, "~αι~ομενος";
    participle_aor: "~ᾰ~ᾱς", "~ᾰ~ᾰμενος", _;
    participle_perf: _, _, "~η~μενος";
    ),
    // Προβαλλω: 4 words (βᾰλλω, προβαλλω, διαβαλλω, τῐλλω). Like Κρῑνομαι, except in 125 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("προβαλλω", 4, "~λω", like Κρῑνομαι;
    stem future_pass: "~ησησ- like ἐχω";
    stem aorist_ind_pass: "~ην- like ἐχω";
    stem aorist_pass: "~- like ἐγειρω";
    ind_act_1: "~λω", _, "~λομεν";
    ind_act_2: "~λεις", "~λετον", _;
    ind_act_3: "~λει", _, "~λουσῐ";
    ind_mid_1: derived, _, "~λομεθᾰ";
    ind_mid_2: "~λῃ", _, _;
    ind_mid_3: _, "~λεσθον", _;
    ind_pass_1: "~λομαι", _, _;
    ind_pass_2: _, "~λεσθον", "~λεσθε";
    ind_pass_3: "~λεται", "~λεσθον", "~λονται";
    subj_act_1: "~λω", _, "~λωμεν";
    subj_act_2: "~λῃς", _, _;
    subj_act_3: "~λῃ", "~λητον", "~λωσῐ";
    subj_mid_1: derived, _, "~λωμεθᾰ";
    subj_mid_2: derived, "~λησθον", _;
    subj_mid_3: "~ληται", _, "~λωνται";
    subj_pass_1: "~λωμαι", _, _;
    subj_pass_2: derived, _, "~λησθε";
    opt_act_1: "~λοιμῐ", _, "~λοιμεν";
    opt_act_2: "~λοις", "~λοιτον", _;
    opt_act_3: "~λοι", "~λοιτην", "~λοιεν";
    opt_mid_1: "~λοιμην", _, "~λοιμεθᾰ";
    opt_mid_2: "~λοιο", _, "~λοισθε";
    opt_mid_3: "~λοιτο", "~λοισθην", "~λοιντο";
    opt_pass_2: _, "~λοισθον", _;
    imp_act_2: "~λε", _, "~λετε";
    imp_act_3: _, _, "~λοντων";
    imp_mid_2: "~λου", "~λεσθον", "~λεσθε";
    imp_mid_3: derived, _, "~λεσθων";
    imp_pass_2: _, "~λεσθον", "~λεσθε";
    imp_pass_3: "~λεσθω", "~λεσθων", "~λεσθων";
    imperfect_act_1: "ἐ~λον", _, _;
    imperfect_act_2: "ἐ~λες", "ἐ~λετον", "ἐ~λετε";
    imperfect_act_3: "ἐ~λε", _, "ἐ~λον";
    imperfect_mid_1: "ἐ~λομην", _, "ἐ~λομεθᾰ";
    imperfect_mid_2: "ἐ~λου", derived, _;
    imperfect_mid_3: "ἐ~λετο", _, "ἐ~λοντο";
    imperfect_pass_2: _, "ἐ~λεσθον", "ἐ~λεσθε";
    imperfect_pass_3: _, "ἐ~λεσθην", _;
    future_ind_act_1: "~ω", _, "~ουμεν";
    future_ind_act_2: "~εις", "~ειτον", _;
    future_ind_act_3: _, _, "~ουσῐ";
    future_ind_mid_1: "~ουμαι", _, _;
    future_ind_mid_2: "~ει", "~εισθον", _;
    future_ind_mid_3: "~ειται", _, _;
    future_opt_act_1: "~οιην", _, "~οιμεν";
    future_opt_act_2: "~οιης", "~οιτον", _;
    future_opt_act_3: "~οιη", "~οιτην", _;
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_3: _, "~οισθην", _;
    perfect_ind_act_1: "%ε~κᾰ", _, "%ε~κᾰμεν";
    perfect_ind_act_2: _, "%ε~κᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~κᾱσῐ";
    perfect_ind_mid_1: _, _, "ἐ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~σαι", "ἐ~θον", _;
    perfect_ind_mid_3: "ἐ~ται", _, "ἐ~ᾰται";
    perfect_ind_pass_1: "ἐ~μαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~θε";
    perfect_subj_act_1: "%ε~κω", _, _;
    perfect_subj_act_2: "%ε~κῃς", "%ε~κητον", "%ε~κητε";
    perfect_subj_act_3: "%ε~κῃ", _, "%ε~κωσῐ";
    perfect_imp_act_2: "%ε~κε", _, _;
    perfect_imp_mid_2: _, _, "ἐ~θε";
    perfect_imp_mid_3: _, _, "ἐ~θων";
    perfect_imp_pass_2: "ἐ~σο", _, "ἐ~θε";
    perfect_imp_pass_3: "ἐ~θω", "ἐ~θων", "ἐ~θων";
    infinitive_pres: "~λειν", _, "~λεσθαι";
    infinitive_fut: "~ειν", _, _;
    infinitive_perf: _, "ἐ~θαι", _;
    participle_pres: "~λων", _, "~λομενος";
    participle_perf: "%ε~κως", _, "ἐ~μενος";
    ),
    // Ἀφορῐζω: 4 words (ἀφορῐζω, προορῐζω, ἐξορκῐζω, ἑορτᾰζω). Like Ἀποκωλῡω, except in 137 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀφορῐζω", 4, "~ο~ζω", like Ἀποκωλῡω;
    stem future_pass: "~ο~σθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ω~σ- like εὐλογεω";
    stem aorist_ind_pass: "~ω~σθην- like εὐλογεω";
    stem aorist_pass: "~ο~σθ- like ποιεω";
    stem pluperfect_act: "~ω~κ- like εὐλογεω";
    ind_act_1: "~ο~ζω", _, "~ο~ζομεν";
    ind_act_2: "~ο~ζεις", "~ο~ζετον", _;
    ind_act_3: "~ο~ζει", _, "~ο~ζουσῐ|~ο~ζουσῐν";
    ind_mid_1: _, _, "~ο~ζομεθᾰ";
    ind_mid_2: "~ο~ζῃ|~ο~ζει", _, _;
    ind_mid_3: _, "~ο~ζεσθον", _;
    ind_pass_1: "~ο~ζομαι", _, _;
    ind_pass_2: _, "~ο~ζεσθον", "~ο~ζεσθε";
    ind_pass_3: "~ο~ζεται", "~ο~ζεσθον", "~ο~ζονται";
    subj_act_1: "~ο~ζω", _, "~ο~ζωμεν";
    subj_act_2: "~ο~ζῃς", _, _;
    subj_act_3: "~ο~ζῃ", "~ο~ζητον", "~ο~ζωσῐ|~ο~ζωσῐν";
    subj_mid_1: _, _, "~ο~ζωμεθᾰ";
    subj_mid_2: _, "~ο~ζησθον", _;
    subj_mid_3: "~ο~ζηται", _, "~ο~ζωνται";
    subj_pass_1: "~ο~ζωμαι", _, _;
    subj_pass_2: _, _, "~ο~ζησθε";
    opt_act_1: "~ο~ζοιμῐ", _, "~ο~ζοιμεν";
    opt_act_2: "~ο~ζοις", "~ο~ζοιτον", _;
    opt_act_3: "~ο~ζοι", "~ο~ζοιτην", "~ο~ζοιεν";
    opt_mid_1: "~ο~ζοιμην", _, "~ο~ζοιμεθᾰ";
    opt_mid_2: "~ο~ζοιο", _, "~ο~ζοισθε";
    opt_mid_3: "~ο~ζοιτο", "~ο~ζοισθην", "~ο~ζοιντο";
    opt_pass_2: _, "~ο~ζοισθον", _;
    imp_act_2: "~ο~ζε", _, "~ο~ζετε";
    imp_act_3: _, _, "~ο~ζοντων";
    imp_mid_2: "~ο~ζου", "~ο~ζεσθον", "~ο~ζεσθε";
    imp_mid_3: _, _, "~ο~ζεσθων";
    imp_pass_2: _, "~ο~ζεσθον", "~ο~ζεσθε";
    imp_pass_3: "~ο~ζεσθω", "~ο~ζεσθων", "~ο~ζεσθων";
    imperfect_act_1: "~ω~ζον", _, _;
    imperfect_act_2: "~ω~ζες", "~ω~ζετον", "~ω~ζετε";
    imperfect_act_3: "~ω~ζε|~ω~ζεν", _, "~ω~ζον";
    imperfect_mid_1: "~ω~ζομην", _, "~ω~ζομεθᾰ";
    imperfect_mid_2: "~ω~ζου", _, _;
    imperfect_mid_3: "~ω~ζετο", _, "~ω~ζοντο";
    imperfect_pass_2: _, "~ω~ζεσθον", "~ω~ζεσθε";
    imperfect_pass_3: _, "~ω~ζεσθην", _;
    future_ind_act_1: "~ο~ω", _, "~ο~ουμεν";
    future_ind_act_2: "~ο~εις", "~ο~ειτον", _;
    future_ind_act_3: _, _, "~ο~ουσῐ|~ο~ουσῐν";
    future_ind_mid_1: "~ο~ουμαι", _, _;
    future_ind_mid_2: "~ο~ει|~ο~ῃ", "~ο~εισθον", _;
    future_ind_mid_3: "~ο~ειται", _, _;
    future_opt_act_1: "~ο~οιην|~ο~οιμῐ", _, "~ο~οιμεν|~ο~οιημεν";
    future_opt_act_2: "~ο~οιης|~ο~οις", "~ο~οιτον|~ο~οιητον", "~ο~οιτε|~ο~οιητε";
    future_opt_act_3: "~ο~οιη|~ο~οι", "~ο~οιτην|~ο~οιητην", "~ο~οιεν|~ο~οιησᾰν";
    future_opt_mid_1: "~ο~οιμην", _, "~ο~οιμεθᾰ";
    future_opt_mid_3: _, "~ο~οισθην", _;
    aorist_subj_act_3: _, _, "~ο~σωσῐ|~ο~σωσῐν";
    aorist_opt_act_2: "~ο~σειᾰς|~ο~σαις", _, _;
    aorist_opt_act_3: "~ο~σειε|~ο~σειεν|~ο~σαι", _, "~ο~σειᾰν|~ο~σαιεν";
    perfect_ind_act_1: "~ω~κᾰ", _, "~ω~κᾰμεν";
    perfect_ind_act_2: _, "~ω~κᾰτον", _;
    perfect_ind_act_3: "~ω~κε|~ω~κεν", _, "~ω~κᾱσῐ|~ω~κᾱσῐν";
    perfect_ind_mid_1: _, _, "~ω~σμεθᾰ";
    perfect_ind_mid_2: "~ω~σαι", "~ω~σθον", _;
    perfect_ind_mid_3: "~ω~σται", _, "~ω~δᾰται";
    perfect_ind_pass_1: "~ω~σμαι", _, _;
    perfect_ind_pass_2: _, _, "~ω~σθε";
    perfect_subj_act_1: "~ω~κω", _, _;
    perfect_subj_act_2: "~ω~κῃς", "~ω~κητον", "~ω~κητε";
    perfect_subj_act_3: "~ω~κῃ", _, "~ω~κωσῐ|~ω~κωσῐν";
    perfect_opt_act_1: "~ω~κοιμῐ|~ω~κοιην", _, _;
    perfect_opt_act_2: "~ω~κοις|~ω~κοιης", _, _;
    perfect_opt_act_3: "~ω~κοι|~ω~κοιη", _, _;
    perfect_imp_act_2: "~ω~κε", _, _;
    perfect_imp_mid_2: _, _, "~ω~σθε";
    perfect_imp_mid_3: _, _, "~ω~σθων";
    perfect_imp_pass_2: "~ω~σο", _, "~ω~σθε";
    perfect_imp_pass_3: "~ω~σθω", "~ω~σθων", "~ω~σθων";
    pluperfect_mid_1: "~ω~σμην", _, _;
    pluperfect_mid_2: "~ω~σο", "~ω~σθον", _;
    pluperfect_mid_3: "~ω~στο", _, "~ω~δᾰτο";
    pluperfect_pass_1: _, _, "~ω~σμεθᾰ";
    pluperfect_pass_2: _, _, "~ω~σθε";
    pluperfect_pass_3: _, "~ω~σθην", _;
    infinitive_pres: "~ο~ζειν", _, "~ο~ζεσθαι";
    infinitive_fut: "~ο~ειν", _, _;
    infinitive_perf: _, "~ω~σθαι", _;
    participle_pres: "~ο~ζων", _, "~ο~ζομενος";
    participle_perf: "~ω~κως", _, "~ω~σμενος";
    ),
    // Κᾰκοω: 3 words (κᾰκοω, ἐξῐσοω, ἡλοω). Like Γῡροω, except in 56 cells: imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, imperfect_act_3 pl, ….
    conjugation!("κᾰκοω", 3, "~οω", like Γῡροω;
    stem aorist_ind: "~ωσ- like εὐλογεω";
    stem aorist_ind_pass: "~ωθην- like εὐλογεω";
    imperfect_act_1: "~ουν", _, _;
    imperfect_act_2: "~ους", "~ουτον", "~ουτε";
    imperfect_act_3: "~ου", _, "~ουν";
    imperfect_mid_1: "~ουμην", _, "~ουμεθᾰ";
    imperfect_mid_2: "~ου", _, _;
    imperfect_mid_3: "~ουτο", _, "~ουντο";
    imperfect_pass_2: _, "~ουσθον", "~ουσθε";
    imperfect_pass_3: _, "~ουσθην", _;
    future_ind_act_3: _, _, "~ωσουσῐ|~ωσουσῐν";
    perfect_ind_act_1: "~ωκᾰ", _, "~ωκᾰμεν";
    perfect_ind_act_2: _, "~ωκᾰτον", _;
    perfect_ind_act_3: "~ωκε|~ωκεν", _, "~ωκᾱσῐ|~ωκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ωμεθᾰ";
    perfect_ind_mid_2: "~ωσαι", "~ωσθον", _;
    perfect_ind_mid_3: "~ωται", _, "~ωνται";
    perfect_ind_pass_1: "~ωμαι", _, _;
    perfect_ind_pass_2: _, _, "~ωσθε";
    perfect_subj_act_1: "~ωκω", _, _;
    perfect_subj_act_2: "~ωκῃς", "~ωκητον", "~ωκητε";
    perfect_subj_act_3: "~ωκῃ", _, "~ωκωσῐ|~ωκωσῐν";
    perfect_opt_act_1: "~ωκοιμῐ|~ωκοιην", _, _;
    perfect_opt_act_2: "~ωκοις|~ωκοιης", _, _;
    perfect_opt_act_3: "~ωκοι|~ωκοιη", _, _;
    perfect_imp_act_2: "~ωκε", _, _;
    perfect_imp_mid_2: _, _, "~ωσθε";
    perfect_imp_mid_3: _, _, "~ωσθων";
    perfect_imp_pass_2: "~ωσο", _, "~ωσθε";
    perfect_imp_pass_3: "~ωσθω", "~ωσθων", "~ωσθων";
    pluperfect_act_1: "~ωκειν", _, _;
    pluperfect_mid_1: "~ωμην", _, _;
    pluperfect_mid_2: "~ωσο", "~ωσθον", _;
    pluperfect_mid_3: "~ωτο", _, "~ωντο";
    pluperfect_pass_1: _, _, "~ωμεθᾰ";
    pluperfect_pass_2: _, _, "~ωσθε";
    pluperfect_pass_3: _, "~ωσθην", _;
    infinitive_perf: _, "~ωσθαι", _;
    participle_perf: "~ωκως", _, "~ωμενος";
    ),
    // Πῑαινω: 3 words (μᾰραινω, πημαινω, πῑαινω). Like Περαινω, except in 33 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, ….
    conjugation!("πῑαινω", 3, "~αινω", like Περαινω;
    stem aorist_ind: "~ᾱν- like ἐχω";
    ind_act_3: _, _, "~αινουσῐ";
    ind_mid_2: "~αινῃ", _, _;
    subj_act_3: _, _, "~αινωσῐ";
    imperfect_act_1: "ε~αινον", _, _;
    imperfect_act_2: "ε~αινες", "ε~αινετον", "ε~αινετε";
    imperfect_act_3: "ε~αινε", _, "ε~αινον";
    imperfect_mid_1: "ε~αινομην", _, "ε~αινομεθᾰ";
    imperfect_mid_2: "ε~αινου", _, _;
    imperfect_mid_3: "ε~αινετο", _, "ε~αινοντο";
    imperfect_pass_2: _, "ε~αινεσθον", "ε~αινεσθε";
    imperfect_pass_3: _, "ε~αινεσθην", _;
    future_ind_act_3: _, _, "~ᾰνουσῐ";
    future_ind_mid_2: "~ᾰνει", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: "~ᾰνοιην", _, "~ᾰνοιμεν";
    future_opt_act_2: "~ᾰνοιης", "~ᾰνοιτον", derived;
    future_opt_act_3: "~ᾰνοιη", "~ᾰνοιτην", derived;
    aorist_subj_act_3: _, _, derived;
    aorist_opt_act_2: "~ᾱνειᾰς", _, _;
    aorist_opt_act_3: "~ᾱνειε", _, "~ᾱνειᾰν";
    ),
    // Θερᾰπευω: 3 words (ἱκετευω, ὀρουω, ὀπῑπευω). Like Βουλευω, except in 45 cells: perfect_ind_act_2 sg, perfect_ind_act_2 pl, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_ind_mid_1 sg, perfect_ind_mid_2 pl, ….
    conjugation!("θερᾰπευω", 3, "~ω", like Βουλευω;
    stem future_pass: "~θησ- like χρημᾰτῐζω";
    stem aorist_pass: "~θ- like ποιεω";
    perfect_ind_act_2: "%ε~κᾰς", _, "%ε~κᾰτε";
    perfect_ind_act_3: "%ε~κε|%ε~κεν", _, "%ε~κᾱσῐ|%ε~κᾱσῐν";
    perfect_ind_mid_1: "%ε~μαι", _, _;
    perfect_ind_mid_2: _, _, "%ε~σθε";
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: _, _, "%ε~κωμεν";
    perfect_subj_act_3: _, _, "%ε~κωσῐ|%ε~κωσῐν";
    perfect_opt_act_1: "%ε~κοιμῐ|%ε~κοιην", _, "%ε~κοιμεν";
    perfect_opt_act_2: "%ε~κοις|%ε~κοιης", "%ε~κοιτον", "%ε~κοιτε";
    perfect_opt_act_3: "%ε~κοι|%ε~κοιη", "%ε~κοιτην", "%ε~κοιεν";
    perfect_imp_act_2: _, "%ε~κετον", "%ε~κετε";
    perfect_imp_act_3: "%ε~κετω", "%ε~κετων", "%ε~κοντων";
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
    participle_perf: _, "%ε~μενος", derived;
    ),
    // Ἀποζω: 3 words (ζω, ἀποζω, ἀνᾰζω). Like Περιειμι, except in 78 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀποζω", 3, "~ω", like Περιειμι;
    stem aorist_ind: "~ησ- like ἐχω";
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
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", _;
    future_ind_act_3: _, _, "~ησουσῐ";
    future_ind_mid_1: "~ησομαι", _, _;
    future_ind_mid_2: "~ησῃ", "~ησεσθον", _;
    future_ind_mid_3: "~ησεται", _, _;
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: "~ησοις", "~ησοιτον", _;
    future_opt_act_3: "~ησοι", "~ησοιτην", _;
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ησοισθην", _;
    aorist_subj_act_1: "~ησω", _, _;
    aorist_subj_act_2: "~ησῃς", "~ησητον", _;
    aorist_subj_mid_1: "~ησωμαι", _, _;
    aorist_subj_mid_2: "~ησῃ", "~ησησθον", _;
    aorist_opt_act_1: "~ησαιμῐ", _, "~ησαιμεν";
    aorist_opt_act_2: "~ησειᾰς", _, _;
    aorist_opt_act_3: "~ησειε", _, "~ησειᾰν";
    aorist_opt_mid_1: "~ησαιμην", _, _;
    aorist_imp_act_2: "~ησον", _, "~ησᾰτε";
    aorist_imp_act_3: _, _, "~ησᾰντων";
    aorist_imp_mid_2: "~ησαι", "~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ησᾰσθων";
    perfect_ind_act_1: "ἐ~ηκᾰ", _, "ἐ~ηκᾰμεν";
    perfect_ind_act_2: _, "ἐ~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~ηκᾱσῐ";
    perfect_subj_act_1: "ἐ~ηκω", _, _;
    perfect_subj_act_2: "ἐ~ηκῃς", "ἐ~ηκητον", "ἐ~ηκητε";
    perfect_subj_act_3: "ἐ~ηκῃ", _, "ἐ~ηκωσῐ";
    perfect_imp_act_2: "ἐ~ηκε", _, _;
    infinitive_pres: "~ην", _, _;
    infinitive_fut: "~ησειν", _, _;
    infinitive_aor: "~ησαι", _, _;
    participle_aor: "~ησᾱς", "~ησᾰμενος", _;
    participle_perf: "ἐ~ηκως", _, _;
    ),
    // Πλᾰναω: 3 words (δαμναω, πλᾰναω, γαναω). Like Ἀειρω, except in 155 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πλᾰναω", 3, "~ναω", like Ἀειρω;
    stem aorist_ind: "~νησ- like ἐχω";
    stem aorist_ind_pass: "~νηθην- like ἐχω";
    stem aorist_pass: "~νηθ- like ἐχω";
    stem pluperfect_act: "~νηκειν- like ἐχω";
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
    opt_act_2: "~νῳης", "~νῳτον", _;
    opt_act_3: "~νῳη", "~νῳτην", "~νῳεν";
    opt_mid_1: "~νῳμην", _, "~νῳμεθᾰ";
    opt_mid_2: "~νῳο", _, "~νῳσθε";
    opt_mid_3: "~νῳτο", "~νῳσθην", "~νῳντο";
    opt_pass_2: _, "~νῳσθον", _;
    imp_act_2: "~νᾱ", _, "~νᾱτε";
    imp_act_3: _, _, "~νωντων";
    imp_mid_2: "~νω", "~νᾱσθον", "~νᾱσθε";
    imp_mid_3: _, _, "~νᾱσθων";
    imp_pass_2: _, "~νᾱσθον", "~νᾱσθε";
    imp_pass_3: "~νᾱσθω", "~νᾱσθων", "~νᾱσθων";
    imperfect_act_1: "ἐ~νων", _, _;
    imperfect_act_2: "ἐ~νᾱς", "ἐ~νᾱτον", "ἐ~νᾱτε";
    imperfect_act_3: "ἐ~νᾱ", _, "ἐ~νων";
    imperfect_mid_1: "ἐ~νωμην", _, "ἐ~νωμεθᾰ";
    imperfect_mid_2: "ἐ~νω", _, _;
    imperfect_mid_3: "ἐ~νᾱτο", _, "ἐ~νωντο";
    imperfect_pass_2: _, "ἐ~νᾱσθον", "ἐ~νᾱσθε";
    imperfect_pass_3: _, "ἐ~νᾱσθην", _;
    future_ind_mid_1: "~γησομαι", _, _;
    future_ind_mid_2: "~γησῃ", "~γησεσθον", _;
    future_ind_mid_3: "~γησεται", _, _;
    future_opt_mid_1: "~γησοιμην", _, "~γησοιμεθᾰ";
    future_opt_mid_3: _, "~γησοισθην", _;
    aorist_subj_act_1: "~νησω", _, _;
    aorist_subj_act_2: "~νησῃς", "~νησητον", _;
    aorist_subj_mid_1: "~νησωμαι", _, _;
    aorist_subj_mid_2: "~νησῃ", "~νησησθον", _;
    aorist_opt_act_1: "~νησαιμῐ", _, "~νησαιμεν";
    aorist_opt_act_2: "~νησειᾰς", _, _;
    aorist_opt_act_3: "~νησειε", _, "~νησειᾰν";
    aorist_opt_mid_1: "~νησαιμην", _, _;
    aorist_imp_act_2: "~νησον", _, "~νησᾰτε";
    aorist_imp_act_3: _, _, "~νησᾰντων";
    aorist_imp_mid_2: "~νησαι", "~νησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~νησᾰσθων";
    perfect_ind_act_1: "%ε~νηκᾰ", _, "%ε~νηκᾰμεν";
    perfect_ind_act_2: _, "%ε~νηκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~νηκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~νημεθᾰ";
    perfect_ind_mid_2: "%ε~νησαι", "%ε~νησθον", _;
    perfect_ind_mid_3: "%ε~νηται", _, "%ε~νηνται";
    perfect_ind_pass_1: "%ε~νημαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~νησθε";
    perfect_subj_act_1: "%ε~νηκω", _, _;
    perfect_subj_act_2: "%ε~νηκῃς", "%ε~νηκητον", "%ε~νηκητε";
    perfect_subj_act_3: "%ε~νηκῃ", _, "%ε~νηκωσῐ";
    perfect_imp_act_2: "%ε~νηκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~νησθε";
    perfect_imp_mid_3: _, _, "%ε~νησθων";
    perfect_imp_pass_2: "%ε~νησο", _, "%ε~νησθε";
    perfect_imp_pass_3: "%ε~νησθω", "%ε~νησθων", "%ε~νησθων";
    pluperfect_mid_1: "ἐ%ε~νημην", _, _;
    pluperfect_mid_2: "ἐ%ε~νησο", "ἐ%ε~νησθον", _;
    pluperfect_mid_3: "ἐ%ε~νητο", _, "ἐ%ε~νηντο";
    pluperfect_pass_1: _, _, "ἐ%ε~νημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~νησθε";
    pluperfect_pass_3: _, "ἐ%ε~νησθην", _;
    future_perfect_ind_mid_1: "%ε~νησομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~νησεσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~νησομεθᾰ";
    future_perfect_ind_pass_2: "%ε~νησῃ", "%ε~νησεσθον", "%ε~νησεσθε";
    future_perfect_ind_pass_3: "%ε~νησεται", "%ε~νησεσθον", "%ε~νησονται";
    future_perfect_opt_pass_1: "%ε~νησοιμην", _, "%ε~νησοιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~νησοιο", "%ε~νησοισθον", "%ε~νησοισθε";
    future_perfect_opt_pass_3: "%ε~νησοιτο", "%ε~νησοισθην", "%ε~νησοιντο";
    infinitive_pres: "~νᾱν", _, "~νᾱσθαι";
    infinitive_aor: "~νησαι", _, _;
    infinitive_perf: _, "%ε~νησθαι", _;
    infinitive_futp: _, _, "%ε~νησεσθαι";
    participle_pres: "~νων", _, "~νωμενος";
    participle_aor: "~νησᾱς", "~νησᾰμενος", _;
    participle_perf: "%ε~νηκως", _, "%ε~νημενος";
    participle_futp: _, _, "%ε~νησομενος";
    ),
    // Δῐᾰρπᾰζω: 3 words (ἐξᾰφᾰνῐζω, δῐᾰρπᾰζω, ἐξᾰττῐκῐζω). Like Ἐπᾰϊω, except in 101 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("δῐᾰρπᾰζω", 3, "~ᾰ~ζω", like Ἐπᾰϊω;
    stem future_pass: "~ᾰ~σθησ- like χρημᾰτῐζω";
    stem aorist_pass: "~ᾰ~σθ- like ποιεω";
    stem pluperfect_act: "~η~κ- like εὐλογεω";
    ind_act_1: "~ᾰ~ζω", _, "~ᾰ~ζομεν";
    ind_act_2: "~ᾰ~ζεις", "~ᾰ~ζετον", _;
    ind_act_3: "~ᾰ~ζει", _, "~ᾰ~ζουσῐ|~ᾰ~ζουσῐν";
    ind_mid_1: _, _, "~ᾰ~ζομεθᾰ";
    ind_mid_2: "~ᾰ~ζῃ|~ᾰ~ζει", _, _;
    ind_mid_3: _, "~ᾰ~ζεσθον", _;
    ind_pass_1: "~ᾰ~ζομαι", _, _;
    ind_pass_2: _, "~ᾰ~ζεσθον", "~ᾰ~ζεσθε";
    ind_pass_3: "~ᾰ~ζεται", "~ᾰ~ζεσθον", "~ᾰ~ζονται";
    subj_act_1: "~ᾰ~ζω", _, "~ᾰ~ζωμεν";
    subj_act_2: "~ᾰ~ζῃς", _, _;
    subj_act_3: "~ᾰ~ζῃ", "~ᾰ~ζητον", "~ᾰ~ζωσῐ|~ᾰ~ζωσῐν";
    subj_mid_1: _, _, "~ᾰ~ζωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ζησθον", _;
    subj_mid_3: "~ᾰ~ζηται", _, "~ᾰ~ζωνται";
    subj_pass_1: "~ᾰ~ζωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~ζησθε";
    opt_act_1: "~ᾰ~ζοιμῐ", _, "~ᾰ~ζοιμεν";
    opt_act_2: "~ᾰ~ζοις", "~ᾰ~ζοιτον", _;
    opt_act_3: "~ᾰ~ζοι", "~ᾰ~ζοιτην", "~ᾰ~ζοιεν";
    opt_mid_1: "~ᾰ~ζοιμην", _, "~ᾰ~ζοιμεθᾰ";
    opt_mid_2: "~ᾰ~ζοιο", _, "~ᾰ~ζοισθε";
    opt_mid_3: "~ᾰ~ζοιτο", "~ᾰ~ζοισθην", "~ᾰ~ζοιντο";
    opt_pass_2: _, "~ᾰ~ζοισθον", _;
    imp_act_2: "~ᾰ~ζε", _, "~ᾰ~ζετε";
    imp_act_3: _, _, "~ᾰ~ζοντων";
    imp_mid_2: "~ᾰ~ζου", "~ᾰ~ζεσθον", "~ᾰ~ζεσθε";
    imp_mid_3: _, _, "~ᾰ~ζεσθων";
    imp_pass_2: _, "~ᾰ~ζεσθον", "~ᾰ~ζεσθε";
    imp_pass_3: "~ᾰ~ζεσθω", "~ᾰ~ζεσθων", "~ᾰ~ζεσθων";
    imperfect_act_1: "~η~ζον", _, _;
    imperfect_act_2: "~η~ζες", "~η~ζετον", "~η~ζετε";
    imperfect_act_3: "~η~ζε|~η~ζεν", _, "~η~ζον";
    imperfect_mid_1: "~η~ζομην", _, "~η~ζομεθᾰ";
    imperfect_mid_2: "~η~ζου", _, _;
    imperfect_mid_3: "~η~ζετο", _, "~η~ζοντο";
    imperfect_pass_2: _, "~η~ζεσθον", "~η~ζεσθε";
    imperfect_pass_3: _, "~η~ζεσθην", _;
    future_ind_act_1: "~ᾰ~ω", _, "~ᾰ~ουμεν";
    future_ind_act_2: "~ᾰ~εις", "~ᾰ~ειτον", _;
    future_ind_act_3: _, _, "~ᾰ~ουσῐ|~ᾰ~ουσῐν";
    future_ind_mid_1: "~ᾰ~ουμαι", _, _;
    future_ind_mid_2: "~ᾰ~ει|~ᾰ~ῃ", "~ᾰ~εισθον", _;
    future_ind_mid_3: "~ᾰ~ειται", _, _;
    future_opt_act_1: "~ᾰ~οιην|~ᾰ~οιμῐ", _, "~ᾰ~οιμεν|~ᾰ~οιημεν";
    future_opt_act_2: "~ᾰ~οιης|~ᾰ~οις", "~ᾰ~οιτον|~ᾰ~οιητον", "~ᾰ~οιτε|~ᾰ~οιητε";
    future_opt_act_3: "~ᾰ~οιη|~ᾰ~οι", "~ᾰ~οιτην|~ᾰ~οιητην", "~ᾰ~οιεν|~ᾰ~οιησᾰν";
    future_opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~οισθην", _;
    perfect_ind_act_3: "~η~κε|~η~κεν", _, "~η~κᾱσῐ|~η~κᾱσῐν";
    perfect_ind_mid_3: _, _, "~η~δᾰται";
    perfect_subj_act_3: _, _, "~η~κωσῐ|~η~κωσῐν";
    perfect_opt_act_1: "~η~κοιμῐ|~η~κοιην", _, _;
    perfect_opt_act_2: "~η~κοις|~η~κοιης", _, _;
    perfect_opt_act_3: "~η~κοι|~η~κοιη", _, _;
    pluperfect_mid_3: _, _, "~η~δᾰτο";
    infinitive_pres: "~ᾰ~ζειν", _, "~ᾰ~ζεσθαι";
    infinitive_fut: "~ᾰ~ειν", _, _;
    participle_pres: "~ᾰ~ζων", _, "~ᾰ~ζομενος";
    ),
    // Ὀμνῠω: 3 words (ὀφελλω, ὀμνῠω, ὀρῠττω). Like Κειμαι, except in 72 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ὀμνῠω", 3, "ὀ~ω", like Κειμαι;
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
    opt_act_2: "ὀ~οις", "ὀ~οιτον", _;
    opt_act_3: "ὀ~οι", "ὀ~οιτην", "ὀ~οιεν";
    opt_mid_1: "ὀ~οιμην", _, "ὀ~οιμεθᾰ";
    opt_mid_2: "ὀ~οιο", _, "ὀ~οισθε";
    opt_mid_3: "ὀ~οιτο", "ὀ~οισθην", "ὀ~οιντο";
    opt_pass_2: _, "ὀ~οισθον", _;
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
    // Ψᾱω: 3 words (δῐψᾱω, νᾱω, ψᾱω). Like Νῑκαω, except in 64 cells: ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ψᾱω", 3, "~ᾱω", like Νῑκαω;
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
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_1: _, _, "ἐ~ημεθᾰ";
    perfect_ind_mid_2: "ἐ~ησαι", "ἐ~ησθον", _;
    perfect_ind_mid_3: "ἐ~ηται", _, "ἐ~ηνται";
    perfect_ind_pass_1: "ἐ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~ησθε";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_imp_act_2: derived, _, _;
    perfect_imp_mid_2: _, _, "ἐ~ησθε";
    perfect_imp_mid_3: _, _, "ἐ~ησθων";
    perfect_imp_pass_2: "ἐ~ησο", _, "ἐ~ησθε";
    perfect_imp_pass_3: "ἐ~ησθω", "ἐ~ησθων", "ἐ~ησθων";
    infinitive_pres: "~ην", _, "~ησθαι";
    infinitive_perf: _, "ἐ~ησθαι", _;
    participle_perf: derived, _, "ἐ~ημενος";
    ),
    // Ἐπῐτῐθημῐ: 3 words (ἐπῐτῐθημῐ, συνεπῐτῐθημῐ, ξυνεπῐτῐθημῐ). Like Ἀποκωλῡω, except in 177 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐπῐτῐθημῐ", 3, "~ῐτῐ~ημῐ", like Ἀποκωλῡω;
    stem future_pass: "~ῐ%ε~ησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ε~- like ἐπῐτῐθημῐ";
    ind_act_1: "~ῐ%ῐ~ημῐ", _, "~ῐ%ῐ~εμεν";
    ind_act_2: "~ῐ%ῐ~ης", "~ῐ%ῐ~ετον", _;
    ind_act_3: "~ῐ%ῐ~ησῐ|~ῐ%ῐ~ησῐν", _, "~ῐ%ῐ~εᾱσῐ|~ῐ%ῐ~εᾱσῐν";
    ind_mid_1: _, _, "~ῐ%ῐ~εμεθᾰ";
    ind_mid_2: "~ῐ%ῐ~εσαι", _, _;
    ind_mid_3: _, "~ῐ%ῐ~εσθον", _;
    ind_pass_1: "~ῐ%ῐ~εμαι", _, _;
    ind_pass_2: _, "~ῐ%ῐ~εσθον", "~ῐ%ῐ~εσθε";
    ind_pass_3: "~ῐ%ῐ~εται", "~ῐ%ῐ~εσθον", "~ῐ%ῐ~ενται";
    subj_act_1: "~ῐ%ῐ~ω", _, "~ῐ%ῐ~ωμεν";
    subj_act_2: "~ῐ%ῐ~ῃς", _, _;
    subj_act_3: "~ῐ%ῐ~ῃ", "~ῐ%ῐ~ητον", "~ῐ%ῐ~ωσῐ|~ῐ%ῐ~ωσῐν";
    subj_mid_1: _, _, "~ῐ%ῐ~ωμεθᾰ";
    subj_mid_2: _, "~ῐ%ῐ~ησθον", _;
    subj_mid_3: "~ῐ%ῐ~ηται", _, "~ῐ%ῐ~ωνται";
    subj_pass_1: "~ῐ%ῐ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῐ%ῐ~ησθε";
    opt_act_1: "~ῐ%ῐ~ειην", _, "~ῐ%ῐ~ειμεν|~ῐ%ῐ~ειημεν";
    opt_act_2: "~ῐ%ῐ~ειης", "~ῐ%ῐ~ειτον|~ῐ%ῐ~ειητον", "~ῐ%ῐ~ειτε|~ῐ%ῐ~ειητε";
    opt_act_3: "~ῐ%ῐ~ειη", "~ῐ%ῐ~ειτην|~ῐ%ῐ~ειητην", "~ῐ%ῐ~ειεν|~ῐ%ῐ~ειησᾰν";
    opt_mid_1: "~ῐ%ῐ~ειμην", _, "~ῐ%ῐ~ειμεθᾰ|~ῐ%ῐ~οιμεθᾰ";
    opt_mid_2: "~ῐ%ῐ~ειο", "~ῐ%ῐ~εισθον|~ῐ%ῐ~οισθον", "~ῐ%ῐ~εισθε|~ῐ%ῐ~οισθε";
    opt_mid_3: "~ῐ%ῐ~ειτο|~ῐ%ῐ~οιτο", "~ῐ%ῐ~εισθην|~ῐ%ῐ~οισθην", "~ῐ%ῐ~ειντο|~ῐ%ῐ~οιντο";
    opt_pass_2: _, derived, _;
    imp_act_2: "~ῐ%ῐ~ει", _, "~ῐ%ῐ~ετε";
    imp_act_3: _, _, "~ῐ%ῐ~εντων";
    imp_mid_2: "~ῐ%ῐ~εσο|~ῐ%ῐ~ου", "~ῐ%ῐ~εσθον", "~ῐ%ῐ~εσθε";
    imp_mid_3: _, _, "~ῐ%ῐ~εσθων";
    imp_pass_2: _, "~ῐ%ῐ~εσθον", "~ῐ%ῐ~εσθε";
    imp_pass_3: "~ῐ%ῐ~εσθω", "~ῐ%ῐ~εσθων", "~ῐ%ῐ~εσθων";
    imperfect_act_1: "~ε%ῐ~ην", _, _;
    imperfect_act_2: "~ε%ῐ~εις", "~ε%ῐ~ετον", "~ε%ῐ~ετε";
    imperfect_act_3: "~ε%ῐ~ει", _, "~ε%ῐ~εσᾰν";
    imperfect_mid_1: "~ε%ῐ~εμην", _, "~ε%ῐ~εμεθᾰ";
    imperfect_mid_2: "~ε%ῐ~εσο", _, _;
    imperfect_mid_3: "~ε%ῐ~ετο", _, "~ε%ῐ~εντο";
    imperfect_pass_2: _, "~ε%ῐ~εσθον", "~ε%ῐ~εσθε";
    imperfect_pass_3: _, "~ε%ῐ~εσθην", _;
    future_ind_act_1: "~ῐ~ησω", _, "~ῐ~ησομεν";
    future_ind_act_2: "~ῐ~ησεις", "~ῐ~ησετον", _;
    future_ind_act_3: _, _, "~ῐ~ησουσῐ|~ῐ~ησουσῐν";
    future_ind_mid_1: "~ῐ~ησομαι", _, _;
    future_ind_mid_2: "~ῐ~ησῃ|~ῐ~ησει", "~ῐ~ησεσθον", _;
    future_ind_mid_3: "~ῐ~ησεται", _, _;
    future_opt_act_1: "~ῐ~ησοιμῐ", _, "~ῐ~ησοιμεν";
    future_opt_act_2: "~ῐ~ησοις", "~ῐ~ησοιτον", _;
    future_opt_act_3: "~ῐ~ησοι", "~ῐ~ησοιτην", _;
    future_opt_mid_1: "~ῐ~ησοιμην", _, "~ῐ~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ῐ~ησοισθην", _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ῐ~ηκω|~ῐ~ω", _, "~ῐ~ηκωμεν|~ῐ~ωμεν";
    aorist_subj_act_2: "~ῐ~ηκῃς|~ῐ~ῃς", "~ῐ~ηκητον|~ῐ~ητον", "~ῐ~ηκητε|~ῐ~ητε";
    aorist_subj_act_3: "~ῐ~ηκῃ|~ῐ~ῃ", _, "~ῐ~ηκωσῐ|~ῐ~ηκωσῐν|~ῐ~ωσῐ|~ῐ~ωσῐν";
    aorist_subj_mid_1: "~ῐ~ηκωμαι|~ῐ~ωμαι", _, "~ῐ~ηκωμεθᾰ|~ῐ~ωμεθᾰ";
    aorist_subj_mid_2: derived, "~ῐ~ηκησθον|~ῐ~ησθον", "~ῐ~ηκησθε|~ῐ~ησθε";
    aorist_subj_mid_3: "~ῐ~ηκηται|~ῐ~ηται", _, "~ῐ~ηκωνται|~ῐ~ωνται";
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_1: "~ῐ~ηκαιμῐ|~ῐ~ειην", _, "~ῐ~ηκαιμεν|~ῐ~ειμεν|~ῐ~ειημεν";
    aorist_opt_act_2: "~ῐ~ηκειᾰς|~ῐ~ηκαις|~ῐ~ειης", "~ῐ~ηκαιτον|~ῐ~ειτον|~ῐ~ειητον", "~ῐ~ηκαιτε|~ῐ~ειτε|~ῐ~ειητε";
    aorist_opt_act_3: "~ῐ~ηκειε|~ῐ~ηκειεν|~ῐ~ηκαι|~ῐ~ειη", "~ῐ~ηκαιτην|~ῐ~ειτην|~ῐ~ειητην", "~ῐ~ηκειᾰν|~ῐ~ηκαιεν|~ῐ~ειεν|~ῐ~ειησᾰν";
    aorist_opt_mid_1: "~ῐ~ηκαιμην|~ῐ~ειμην", _, "~ῐ~ηκαιμεθᾰ|~ῐ~ειμεθᾰ";
    aorist_opt_mid_2: "~ῐ~ηκαιο|~ῐ~ειο", "~ῐ~ηκαισθον|~ῐ~εισθον", "~ῐ~ηκαισθε|~ῐ~εισθε";
    aorist_opt_mid_3: "~ῐ~ηκαιτο|~ῐ~ειτο|~ῐ~οιτο", "~ῐ~ηκαισθην|~ῐ~εισθην", "~ῐ~ηκαιντο|~ῐ~ειντο";
    aorist_imp_act_2: "~ῐ~ηκον|~ῐ~ες", "~ῐ~ηκᾰτον|~ῐ~ετον", "~ῐ~ηκᾰτε|~ῐ~ετε";
    aorist_imp_act_3: "~ῐ~ηκᾰτω|~ῐ~ετω", "~ῐ~ηκᾰτων|~ῐ~ετων", "~ῐ~ηκᾰντων|~ῐ~εντων";
    aorist_imp_mid_2: "~ῐ~ηκαι|~ῐ~ου", "~ῐ~ηκᾰσθον|~ῐ~εσθον", "~ῐ~ηκᾰσθε|~ῐ~εσθε";
    aorist_imp_mid_3: "~ῐ~ηκᾰσθω|~ῐ~εσθω", "~ῐ~ηκᾰσθων|~ῐ~εσθων", derived;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: "~ῐ%ε~εικᾰ", _, "~ῐ%ε~εικᾰμεν";
    perfect_ind_act_2: _, "~ῐ%ε~εικᾰτον", _;
    perfect_ind_act_3: "~ῐ%ε~εικε|~ῐ%ε~εικεν", _, "~ῐ%ε~εικᾱσῐ|~ῐ%ε~εικᾱσῐν";
    perfect_ind_mid_1: _, _, "~ῐ%ε~ειμεθᾰ";
    perfect_ind_mid_2: "~ῐ%ε~εισαι", "~ῐ%ε~εισθον", _;
    perfect_ind_mid_3: "~ῐ%ε~ειται", _, "~ῐ%ε~εινται";
    perfect_ind_pass_1: "~ῐ%ε~ειμαι", _, _;
    perfect_ind_pass_2: _, _, "~ῐ%ε~εισθε";
    perfect_subj_act_1: "~ῐ%ε~εικω", _, _;
    perfect_subj_act_2: "~ῐ%ε~εικῃς", "~ῐ%ε~εικητον", "~ῐ%ε~εικητε";
    perfect_subj_act_3: "~ῐ%ε~εικῃ", _, "~ῐ%ε~εικωσῐ|~ῐ%ε~εικωσῐν";
    perfect_opt_act_1: "~ῐ%ε~εικοιμῐ|~ῐ%ε~εικοιην", _, _;
    perfect_opt_act_2: "~ῐ%ε~εικοις|~ῐ%ε~εικοιης", _, _;
    perfect_opt_act_3: "~ῐ%ε~εικοι|~ῐ%ε~εικοιη", _, _;
    perfect_imp_act_2: "~ῐ%ε~εικε", _, _;
    perfect_imp_mid_2: _, _, "~ῐ%ε~εισθε";
    perfect_imp_mid_3: _, _, "~ῐ%ε~εισθων";
    perfect_imp_pass_2: "~ῐ%ε~εισο", _, "~ῐ%ε~εισθε";
    perfect_imp_pass_3: "~ῐ%ε~εισθω", "~ῐ%ε~εισθων", "~ῐ%ε~εισθων";
    infinitive_pres: "~ῐ%ῐ~εναι", _, "~ῐ%ῐ~εσθαι";
    infinitive_fut: "~ῐ~ησειν", _, _;
    infinitive_aor: "~ῐ~ηκαι|~ῐ~ειναι", "~ῐ~ηκᾰσθαι|~ῐ~εσθαι", _;
    infinitive_perf: _, "~ῐ%ε~εισθαι", _;
    participle_pres: "~ῐ%ῐ~εις", _, "~ῐ%ῐ~εμενος";
    participle_aor: "~ῐ~ηκᾱς|~ῐ~εις", "~ῐ~ηκᾰμενος|~ῐ~εμενος", _;
    participle_perf: "~ῐ%ε~εικως", _, "~ῐ%ε~ειμενος";
    ),
    // Ἐπαινεω: 3 words (ἐπαινεω, καταπαιδεραστεω, πᾰραινεω). Like Κᾰταφρονεω, except in 110 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, opt_act_1 sg, opt_act_1 pl, opt_act_2 sg, ….
    conjugation!("ἐπαινεω", 3, "~α~εω", like Κᾰταφρονεω;
    stem future_pass: "~α~εθησησ- like ἐχω";
    stem aorist_ind: "~ε~εσ- like ἐλπω";
    stem aorist_ind_pass: "~ε~εθην- like εὐλογεω";
    stem aorist_pass: "~α~εθ- like ἐχω";
    ind_act_3: _, _, "~α~ουσῐ";
    ind_mid_2: "~α~ει", _, _;
    subj_act_3: _, _, "~α~ωσῐ";
    opt_act_1: "~α~οιην", _, "~α~οιμεν";
    opt_act_2: "~α~οιης", "~α~οιτον", derived;
    opt_act_3: "~α~οιη", "~α~οιτην", "~α~οιεν";
    imperfect_act_1: "~ε~ουν", _, _;
    imperfect_act_2: "~ε~εις", "~ε~ειτον", "~ε~ειτε";
    imperfect_act_3: "~ε~ει", _, "~ε~ουν";
    imperfect_mid_1: "~ε~ουμην", _, "~ε~ουμεθᾰ";
    imperfect_mid_2: "~ε~ου", _, _;
    imperfect_mid_3: "~ε~ειτο", _, "~ε~ουντο";
    imperfect_pass_2: _, "~ε~εισθον", "~ε~εισθε";
    imperfect_pass_3: _, "~ε~εισθην", _;
    future_ind_act_1: "~α~εσω", _, "~α~εσομεν";
    future_ind_act_2: "~α~εσεις", "~α~εσετον", _;
    future_ind_act_3: _, _, "~α~εσουσῐ";
    future_ind_mid_1: "~α~εσομαι", _, _;
    future_ind_mid_2: "~α~εσῃ", "~α~εσεσθον", _;
    future_ind_mid_3: "~α~εσεται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~α~εσοιμῐ", _, "~α~εσοιμεν";
    future_opt_act_2: "~α~εσοις", "~α~εσοιτον", _;
    future_opt_act_3: "~α~εσοι", "~α~εσοιτην", _;
    future_opt_mid_1: "~α~εσοιμην", _, "~α~εσοιμεθᾰ";
    future_opt_mid_3: _, "~α~εσοισθην", _;
    aorist_subj_act_1: "~α~εσω", _, _;
    aorist_subj_act_2: "~α~εσῃς", "~α~εσητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~α~εσωμαι", _, _;
    aorist_subj_mid_2: "~α~εσῃ", "~α~εσησθον", _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_1: "~α~εσαιμῐ", _, "~α~εσαιμεν";
    aorist_opt_act_2: "~α~εσειᾰς", _, _;
    aorist_opt_act_3: "~α~εσειε", _, "~α~εσειᾰν";
    aorist_opt_mid_1: "~α~εσαιμην", _, _;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: "~α~εσον", _, "~α~εσᾰτε";
    aorist_imp_act_3: _, _, "~α~εσᾰντων";
    aorist_imp_mid_2: "~α~εσαι", "~α~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~α~εσᾰσθων";
    perfect_ind_act_1: "~α%ε~ηκᾰ", _, "~α%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "~α%ε~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "~α%ε~ηκᾱσῐ";
    perfect_ind_mid_1: _, _, "~α%ε~ημεθᾰ";
    perfect_ind_mid_2: "~α%ε~ησαι", "~α%ε~ησθον", _;
    perfect_ind_mid_3: "~α%ε~ηται", _, "~α%ε~ηνται";
    perfect_ind_pass_1: "~α%ε~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~α%ε~ησθε";
    perfect_subj_act_1: "~α%ε~ηκω", _, _;
    perfect_subj_act_2: "~α%ε~ηκῃς", "~α%ε~ηκητον", "~α%ε~ηκητε";
    perfect_subj_act_3: "~α%ε~ηκῃ", _, "~α%ε~ηκωσῐ";
    perfect_imp_act_2: "~α%ε~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~α%ε~ησθε";
    perfect_imp_mid_3: _, _, "~α%ε~ησθων";
    perfect_imp_pass_2: "~α%ε~ησο", _, "~α%ε~ησθε";
    perfect_imp_pass_3: "~α%ε~ησθω", "~α%ε~ησθων", "~α%ε~ησθων";
    pluperfect_act_1: "~ε%ε~ηκειν", _, _;
    pluperfect_mid_1: "~ε%ε~ημην", _, _;
    pluperfect_mid_2: "~ε%ε~ησο", "~ε%ε~ησθον", _;
    pluperfect_mid_3: "~ε%ε~ητο", _, "~ε%ε~ηντο";
    pluperfect_pass_1: _, _, "~ε%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ε%ε~ησθε";
    pluperfect_pass_3: _, "~ε%ε~ησθην", _;
    infinitive_fut: "~α~εσειν", _, _;
    infinitive_aor: "~α~εσαι", _, _;
    infinitive_perf: _, "~α%ε~ησθαι", _;
    participle_aor: "~α~εσᾱς", "~α~εσᾰμενος", _;
    participle_perf: "~α%ε~ηκως", _, "~α%ε~ημενος";
    ),
    // Λιστραω: 3 words (μεριμναω, κῠβισταω, λιστραω). Like Βᾰθῠνω, except in 147 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("λιστραω", 3, "~ι~αω", like Βᾰθῠνω;
    stem aorist_ind: "~ῐ~ησ- like ἐχω";
    ind_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    ind_act_2: "~ῐ~ᾳς", "~ῐ~ᾱτον", _;
    ind_act_3: "~ῐ~ᾳ", _, "~ῐ~ωσῐ|~ῐ~ωσῐν";
    ind_mid_1: _, _, "~ῐ~ωμεθᾰ";
    ind_mid_2: "~ῐ~ᾳ", _, _;
    ind_mid_3: _, "~ῐ~ᾱσθον", _;
    ind_pass_1: "~ῐ~ωμαι", _, _;
    ind_pass_2: _, "~ῐ~ᾱσθον", "~ῐ~ᾱσθε";
    ind_pass_3: "~ῐ~ᾱται", "~ῐ~ᾱσθον", "~ῐ~ωνται";
    subj_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    subj_act_2: "~ῐ~ᾳς", _, _;
    subj_act_3: "~ῐ~ᾳ", "~ῐ~ᾱτον", "~ῐ~ωσῐ|~ῐ~ωσῐν";
    subj_mid_1: _, _, "~ῐ~ωμεθᾰ";
    subj_mid_2: _, "~ῐ~ᾱσθον", _;
    subj_mid_3: "~ῐ~ᾱται", _, "~ῐ~ωνται";
    subj_pass_1: "~ῐ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῐ~ᾱσθε";
    opt_act_1: "~ῐ~ῳην|~ῐ~ῳμῐ", _, "~ῐ~ῳμεν|~ῐ~ῳημεν";
    opt_act_2: "~ῐ~ῳης|~ῐ~ῳς", "~ῐ~ῳτον|~ῐ~ῳητον", "~ῐ~ῳτε|~ῐ~ῳητε";
    opt_act_3: "~ῐ~ῳη|~ῐ~ῳ", "~ῐ~ῳτην|~ῐ~ῳητην", "~ῐ~ῳεν|~ῐ~ῳησᾰν";
    opt_mid_1: "~ῐ~ῳμην", _, "~ῐ~ῳμεθᾰ";
    opt_mid_2: "~ῐ~ῳο", _, "~ῐ~ῳσθε";
    opt_mid_3: "~ῐ~ῳτο", "~ῐ~ῳσθην", "~ῐ~ῳντο";
    opt_pass_2: _, "~ῐ~ῳσθον", _;
    imp_act_2: "~ῐ~ᾱ", _, "~ῐ~ᾱτε";
    imp_act_3: _, _, "~ῐ~ωντων";
    imp_mid_2: "~ῐ~ω", "~ῐ~ᾱσθον", "~ῐ~ᾱσθε";
    imp_mid_3: _, _, "~ῐ~ᾱσθων";
    imp_pass_2: _, "~ῐ~ᾱσθον", "~ῐ~ᾱσθε";
    imp_pass_3: "~ῐ~ᾱσθω", "~ῐ~ᾱσθων", "~ῐ~ᾱσθων";
    imperfect_act_1: "ἐ~ῐ~ων", _, _;
    imperfect_act_2: "ἐ~ῐ~ᾱς", "ἐ~ῐ~ᾱτον", "ἐ~ῐ~ᾱτε";
    imperfect_act_3: "ἐ~ῐ~ᾱ", _, "ἐ~ῐ~ων";
    imperfect_mid_1: "ἐ~ῐ~ωμην", _, "ἐ~ῐ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ῐ~ω", _, _;
    imperfect_mid_3: "ἐ~ῐ~ᾱτο", _, "ἐ~ῐ~ωντο";
    imperfect_pass_2: _, "ἐ~ῐ~ᾱσθον", "ἐ~ῐ~ᾱσθε";
    imperfect_pass_3: _, "ἐ~ῐ~ᾱσθην", _;
    future_ind_act_1: "~ῐ~ησω", _, "~ῐ~ησομεν";
    future_ind_act_2: "~ῐ~ησεις", "~ῐ~ησετον", _;
    future_ind_act_3: _, _, "~ῐ~ησουσῐ";
    future_ind_mid_1: "~ῐ~ησομαι", _, _;
    future_ind_mid_2: "~ῐ~ησῃ", "~ῐ~ησεσθον", _;
    future_ind_mid_3: "~ῐ~ησεται", _, _;
    future_opt_act_1: "~ῐ~ησοιμῐ", _, "~ῐ~ησοιμεν";
    future_opt_act_2: "~ῐ~ησοις", "~ῐ~ησοιτον", _;
    future_opt_act_3: "~ῐ~ησοι", "~ῐ~ησοιτην", _;
    future_opt_mid_1: "~ῐ~ησοιμην", _, "~ῐ~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ῐ~ησοισθην", _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ῐ~ησω", _, _;
    aorist_subj_act_2: "~ῐ~ησῃς", "~ῐ~ησητον", _;
    aorist_subj_mid_1: "~ῐ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ῐ~ησῃ", "~ῐ~ησησθον", _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_1: "~ῐ~ησαιμῐ", _, "~ῐ~ησαιμεν";
    aorist_opt_act_2: "~ῐ~ησειᾰς", _, _;
    aorist_opt_act_3: "~ῐ~ησειε", _, "~ῐ~ησειᾰν";
    aorist_opt_mid_1: "~ῐ~ησαιμην", _, _;
    aorist_imp_act_2: "~ῐ~ησον", _, "~ῐ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῐ~ησᾰντων";
    aorist_imp_mid_2: "~ῐ~ησαι", "~ῐ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῐ~ησᾰσθων";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: "%ε~ῐ~ηκᾰ", _, "%ε~ῐ~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~ῐ~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ῐ~ηκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~ῐ~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ῐ~ησαι", "%ε~ῐ~ησθον", _;
    perfect_ind_mid_3: "%ε~ῐ~ηται", _, "%ε~ῐ~ηνται";
    perfect_ind_pass_1: "%ε~ῐ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ῐ~ησθε";
    perfect_subj_act_1: "%ε~ῐ~ηκω", _, _;
    perfect_subj_act_2: "%ε~ῐ~ηκῃς", "%ε~ῐ~ηκητον", "%ε~ῐ~ηκητε";
    perfect_subj_act_3: "%ε~ῐ~ηκῃ", _, "%ε~ῐ~ηκωσῐ";
    perfect_imp_act_2: "%ε~ῐ~ηκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~ῐ~ησθε";
    perfect_imp_mid_3: _, _, "%ε~ῐ~ησθων";
    perfect_imp_pass_2: "%ε~ῐ~ησο", _, "%ε~ῐ~ησθε";
    perfect_imp_pass_3: "%ε~ῐ~ησθω", "%ε~ῐ~ησθων", "%ε~ῐ~ησθων";
    infinitive_pres: "~ῐ~ᾱν", _, "~ῐ~ᾱσθαι";
    infinitive_fut: "~ῐ~ησειν", _, _;
    infinitive_aor: "~ῐ~ησαι", _, _;
    infinitive_perf: _, "%ε~ῐ~ησθαι", _;
    participle_pres: "~ῐ~ων", _, "~ῐ~ωμενος";
    participle_aor: "~ῐ~ησᾱς", "~ῐ~ησᾰμενος", _;
    participle_perf: "%ε~ῐ~ηκως", _, "%ε~ῐ~ημενος";
    ),
    // Φλῡω: 3 words (πτῡω, μῡω, φλῡω). Like Λῡω, except in 101 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("φλῡω", 3, "~ῡω", like Λῡω;
    stem aorist_ind: "~ῠσ- like ποιεω";
    ind_act_3: _, _, "~ῡουσῐ|~ῡουσῐν";
    ind_mid_2: "~ῡῃ|~ῡει", _, _;
    subj_act_3: _, _, "~ῡωσῐ|~ῡωσῐν";
    imperfect_act_3: "ἐ~ῡε|ἐ~ῡεν", _, _;
    future_ind_act_1: "~ῠσω", _, "~ῠσομεν";
    future_ind_act_2: "~ῠσεις", "~ῠσετον", _;
    future_ind_act_3: _, _, "~ῠσουσῐ|~ῠσουσῐν";
    future_ind_mid_1: "~ῠσομαι", _, _;
    future_ind_mid_2: "~ῠσῃ|~ῠσει", "~ῠσεσθον", _;
    future_ind_mid_3: "~ῠσεται", _, _;
    future_opt_act_1: "~ῠσοιμῐ", _, "~ῠσοιμεν";
    future_opt_act_2: "~ῠσοις", "~ῠσοιτον", _;
    future_opt_act_3: "~ῠσοι", "~ῠσοιτην", _;
    future_opt_mid_1: "~ῠσοιμην", _, "~ῠσοιμεθᾰ";
    future_opt_mid_3: _, "~ῠσοισθην", _;
    aorist_subj_act_1: "~ῠσω", _, _;
    aorist_subj_act_2: "~ῠσῃς", "~ῠσητον", _;
    aorist_subj_act_3: _, _, "~ῠσωσῐ|~ῠσωσῐν";
    aorist_subj_mid_1: "~ῠσωμαι", _, _;
    aorist_subj_mid_2: "~ῠσῃ", "~ῠσησθον", _;
    aorist_opt_act_1: "~ῠσαιμῐ", _, "~ῠσαιμεν";
    aorist_opt_act_2: "~ῠσειᾰς|~ῠσαις", _, _;
    aorist_opt_act_3: "~ῠσειε|~ῠσειεν|~ῠσαι", _, "~ῠσειᾰν|~ῠσαιεν";
    aorist_opt_mid_1: "~ῠσαιμην", _, _;
    aorist_imp_act_2: "~ῠσον", _, "~ῠσᾰτε";
    aorist_imp_act_3: _, _, "~ῠσᾰντων";
    aorist_imp_mid_2: "~ῠσαι", "~ῠσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῠσᾰσθων";
    perfect_ind_act_1: "~εμῡκᾰ", _, "~εμῡκᾰμεν";
    perfect_ind_act_2: _, "~εμῡκᾰτον", _;
    perfect_ind_act_3: _, _, "~εμῡκᾱσῐ";
    perfect_ind_mid_1: _, _, "~εμῡμεθᾰ";
    perfect_ind_mid_2: "~εμῡσαι", "~εμῡσθον", _;
    perfect_ind_mid_3: "~εμῡται", _, "~εμῡνται";
    perfect_ind_pass_1: "~εμῡμαι", _, _;
    perfect_ind_pass_2: _, _, "~εμῡσθε";
    perfect_subj_act_1: "~εμῡκω", _, _;
    perfect_subj_act_2: "~εμῡκῃς", "~εμῡκητον", "~εμῡκητε";
    perfect_subj_act_3: "~εμῡκῃ", _, "~εμῡκωσῐ";
    perfect_imp_act_2: "~εμῡκε", _, _;
    perfect_imp_mid_2: _, _, "~εμῡσθε";
    perfect_imp_mid_3: _, _, "~εμῡσθων";
    perfect_imp_pass_2: "~εμῡσο", _, "~εμῡσθε";
    perfect_imp_pass_3: "~εμῡσθω", "~εμῡσθων", "~εμῡσθων";
    pluperfect_act_1: "ἐ~εμῠκειν", _, _;
    pluperfect_mid_1: "ἐ~εμῠμην", _, _;
    pluperfect_mid_2: "ἐ~εμῠσο", "ἐ~εμῠσθον", _;
    pluperfect_mid_3: "ἐ~εμῠτο", _, "ἐ~εμῠντο";
    pluperfect_pass_1: _, _, "ἐ~εμῠμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~εμῠσθε";
    pluperfect_pass_3: _, "ἐ~εμῠσθην", _;
    future_perfect_ind_mid_1: "~εμῡσομαι", _, _;
    future_perfect_ind_mid_3: _, "~εμῡσεσθον", _;
    future_perfect_ind_pass_1: _, _, "~εμῡσομεθᾰ";
    future_perfect_ind_pass_2: "~εμῡσῃ", "~εμῡσεσθον", "~εμῡσεσθε";
    future_perfect_ind_pass_3: "~εμῡσεται", "~εμῡσεσθον", "~εμῡσονται";
    future_perfect_opt_pass_1: "~εμῡσοιμην", _, "~εμῡσοιμεθᾰ";
    future_perfect_opt_pass_2: "~εμῡσοιο", "~εμῡσοισθον", "~εμῡσοισθε";
    future_perfect_opt_pass_3: "~εμῡσοιτο", "~εμῡσοισθην", "~εμῡσοιντο";
    infinitive_fut: "~ῠσειν", _, _;
    infinitive_aor: "~ῠσαι", _, _;
    infinitive_perf: _, "~εμῡσθαι", _;
    infinitive_futp: _, _, "~εμῡσεσθαι";
    participle_aor: "~ῠσᾱς", "~ῠσᾰμενος", _;
    participle_perf: "~εμῡκως", _, "~εμῡμενος";
    participle_futp: _, _, "~εμῡσομενος";
    ),
    // Ἐπευχομαι: 3 words (προσευχομαι, ἀπευχομαι, ἐπευχομαι). Like Πῠνθανομαι, except in 85 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 sg, ….
    conjugation!("ἐπευχομαι", 3, "~ε~χομαι", like Πῠνθανομαι;
    stem aorist_ind: "~η~ξ- like μηκᾰομαι";
    ind_act_1: "~ε~χομαι", _, _;
    ind_mid_1: "~ε~χομαι", _, "~ε~χομεθᾰ";
    ind_mid_2: "~ε~χῃ|~ε~χει", _, _;
    ind_mid_3: _, "~ε~χεσθον", _;
    ind_pass_2: derived, "~ε~χεσθον", "~ε~χεσθε";
    ind_pass_3: "~ε~χεται", "~ε~χεσθον", "~ε~χονται";
    subj_mid_1: "~ε~χωμαι", _, "~ε~χωμεθᾰ";
    subj_mid_2: "~ε~χῃ", "~ε~χησθον", _;
    subj_mid_3: "~ε~χηται", _, "~ε~χωνται";
    subj_pass_2: "~ε~χῃ", _, "~ε~χησθε";
    opt_mid_1: "~ε~χοιμην", _, "~ε~χοιμεθᾰ";
    opt_mid_2: "~ε~χοιο", _, "~ε~χοισθε";
    opt_mid_3: "~ε~χοιτο", "~ε~χοισθην", "~ε~χοιντο";
    opt_pass_2: _, "~ε~χοισθον", _;
    imp_mid_2: "~ε~χου", "~ε~χεσθον", "~ε~χεσθε";
    imp_mid_3: "~ε~χεσθω", _, "~ε~χεσθων";
    imp_pass_2: _, "~ε~χεσθον", "~ε~χεσθε";
    imp_pass_3: _, "~ε~χεσθων", "~ε~χεσθων";
    imperfect_mid_1: "~η~χομην", _, "~η~χομεθᾰ";
    imperfect_mid_2: "~η~χου", "~η~χεσθον", _;
    imperfect_mid_3: "~η~χετο", _, "~η~χοντο";
    imperfect_pass_2: _, _, "~η~χεσθε";
    imperfect_pass_3: _, "~η~χεσθην", _;
    future_ind_mid_1: "~ε~ξομαι", _, _;
    future_ind_mid_2: "~ε~ξῃ|~ε~ξει", "~ε~ξεσθον", _;
    future_ind_mid_3: "~ε~ξεται", _, _;
    future_opt_mid_1: "~ε~ξοιμην", _, "~ε~ξοιμεθᾰ";
    future_opt_mid_3: _, "~ε~ξοισθην", _;
    aorist_subj_mid_1: "~ε~ξωμαι", _, _;
    aorist_subj_mid_2: "~ε~ξῃ", "~ε~ξησθον", _;
    aorist_opt_mid_1: "~ε~ξαιμην", _, _;
    aorist_imp_mid_2: "~ε~ξαι", "~ε~ξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ε~ξᾰσθων";
    perfect_ind_mid_1: _, _, "~η~γμεθᾰ";
    perfect_ind_mid_2: "~η~ξαι", "~η~χθον", _;
    perfect_ind_mid_3: "~η~κται", _, "~η~χᾰται";
    perfect_ind_pass_1: "~η~γμαι", _, _;
    perfect_ind_pass_2: _, _, "~η~χθε";
    perfect_imp_mid_2: _, _, "~η~χθε";
    perfect_imp_mid_3: _, _, "~η~χθων";
    perfect_imp_pass_2: "~η~ξο", _, "~η~χθε";
    perfect_imp_pass_3: "~η~χθω", "~η~χθων", "~η~χθων";
    pluperfect_mid_1: "~η~γμην", _, _;
    pluperfect_mid_2: "~η~ξο", "~η~χθον", _;
    pluperfect_mid_3: "~η~κτο", _, "~η~χᾰτο";
    pluperfect_pass_1: _, _, "~η~γμεθᾰ";
    pluperfect_pass_2: _, _, "~η~χθε";
    pluperfect_pass_3: _, "~η~χθην", _;
    infinitive_pres: _, _, "~ε~χεσθαι";
    infinitive_perf: _, "~η~χθαι", _;
    participle_pres: _, _, "~ε~χομενος";
    participle_aor: _, "~ε~ξᾰμενος", _;
    participle_perf: _, _, "~η~γμενος";
    ),
    // Ἐφελκω: 3 words (ἀφελκω, κᾰθελκω, ἐφελκω). Like Ἀποκωλῡω, except in 145 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐφελκω", 3, "~~κω", like Ἀποκωλῡω;
    stem future_pass: "~~κυσθησ- like ἐχω";
    stem aorist_ind: "~ι~κυσ- like εὐλογεω";
    stem aorist_ind_pass: "~ι~κυσθην- like εὐλογεω";
    stem aorist_pass: "~ι~κυσθ- like ἐχω";
    ind_act_1: "~~κω", _, "~~κομεν";
    ind_act_2: "~~κεις", "~~κετον", _;
    ind_act_3: "~~κει", _, "~~κουσῐ|~~κουσῐν";
    ind_mid_1: _, _, "~~κομεθᾰ";
    ind_mid_2: "~~κῃ|~~κει", _, _;
    ind_mid_3: _, "~~κεσθον", _;
    ind_pass_1: "~~κομαι", _, _;
    ind_pass_2: _, "~~κεσθον", "~~κεσθε";
    ind_pass_3: "~~κεται", "~~κεσθον", "~~κονται";
    subj_act_1: "~~κω", _, "~~κωμεν";
    subj_act_2: "~~κῃς", _, _;
    subj_act_3: "~~κῃ", "~~κητον", "~~κωσῐ|~~κωσῐν";
    subj_mid_1: _, _, "~~κωμεθᾰ";
    subj_mid_2: _, "~~κησθον", _;
    subj_mid_3: "~~κηται", _, "~~κωνται";
    subj_pass_1: "~~κωμαι", _, _;
    subj_pass_2: _, _, "~~κησθε";
    opt_act_1: "~~κοιμῐ", _, "~~κοιμεν";
    opt_act_2: "~~κοις", "~~κοιτον", _;
    opt_act_3: "~~κοι", "~~κοιτην", "~~κοιεν";
    opt_mid_1: "~~κοιμην", _, "~~κοιμεθᾰ";
    opt_mid_2: "~~κοιο", _, "~~κοισθε";
    opt_mid_3: "~~κοιτο", "~~κοισθην", "~~κοιντο";
    opt_pass_2: _, "~~κοισθον", _;
    imp_act_2: "~~κε", _, "~~κετε";
    imp_act_3: _, _, "~~κοντων";
    imp_mid_2: "~~κου", "~~κεσθον", "~~κεσθε";
    imp_mid_3: _, _, "~~κεσθων";
    imp_pass_2: _, "~~κεσθον", "~~κεσθε";
    imp_pass_3: "~~κεσθω", "~~κεσθων", "~~κεσθων";
    imperfect_act_1: "~ι~κον", _, _;
    imperfect_act_2: "~ι~κες", "~ι~κετον", "~ι~κετε";
    imperfect_act_3: "~ι~κε|~ι~κεν", _, "~ι~κον";
    imperfect_mid_1: "~ι~κομην", _, "~ι~κομεθᾰ";
    imperfect_mid_2: "~ι~κου", _, _;
    imperfect_mid_3: "~ι~κετο", _, "~ι~κοντο";
    imperfect_pass_2: _, "~ι~κεσθον", "~ι~κεσθε";
    imperfect_pass_3: _, "~ι~κεσθην", _;
    future_ind_act_1: "~~ξω", _, "~~ξομεν";
    future_ind_act_2: "~~ξεις", "~~ξετον", _;
    future_ind_act_3: _, _, "~~ξουσῐ|~~ξουσῐν";
    future_ind_mid_1: "~~ξομαι", _, _;
    future_ind_mid_2: "~~ξῃ", "~~ξεσθον", _;
    future_ind_mid_3: "~~ξεται", _, _;
    future_opt_act_1: "~~ξοιμῐ", _, "~~ξοιμεν";
    future_opt_act_2: "~~ξοις", "~~ξοιτον", _;
    future_opt_act_3: "~~ξοι", "~~ξοιτην", _;
    future_opt_mid_1: "~~ξοιμην", _, "~~ξοιμεθᾰ";
    future_opt_mid_3: _, "~~ξοισθην", _;
    aorist_subj_act_1: "~~κυσω", _, _;
    aorist_subj_act_2: "~~κυσῃς", "~~κυσητον", _;
    aorist_subj_act_3: _, _, "~~κυσωσῐ|~~κυσωσῐν";
    aorist_subj_mid_1: "~~κυσωμαι", _, _;
    aorist_subj_mid_2: "~~κυσῃ", "~~κυσησθον", _;
    aorist_opt_act_1: "~~κυσαιμῐ", _, "~~κυσαιμεν";
    aorist_opt_act_2: "~~κυσειᾰς|~~κυσαις", _, _;
    aorist_opt_act_3: "~~κυσειε|~~κυσειεν|~~κυσαι", _, "~~κυσειᾰν|~~κυσαιεν";
    aorist_opt_mid_1: "~~κυσαιμην", _, _;
    aorist_imp_act_2: "~~κυσον", _, "~~κυσᾰτε";
    aorist_imp_act_3: _, _, "~~κυσᾰντων";
    aorist_imp_mid_2: "~~κυσαι", "~~κυσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~κυσᾰσθων";
    perfect_ind_act_1: "~ι~κῠκᾰ", _, "~ι~κῠκᾰμεν";
    perfect_ind_act_2: _, "~ι~κῠκᾰτον", _;
    perfect_ind_act_3: "~ι~κῠκε|~ι~κῠκεν", _, "~ι~κῠκᾱσῐ|~ι~κῠκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ι~κῠμεθᾰ";
    perfect_ind_mid_2: "~ι~κῠσαι", "~ι~κῠσθον", _;
    perfect_ind_mid_3: "~ι~κῠται", _, "~ι~κῠνται";
    perfect_ind_pass_1: "~ι~κῠμαι", _, _;
    perfect_ind_pass_2: _, _, "~ι~κῠσθε";
    perfect_subj_act_1: "~ι~κῠκω", _, _;
    perfect_subj_act_2: "~ι~κῠκῃς", "~ι~κῠκητον", "~ι~κῠκητε";
    perfect_subj_act_3: "~ι~κῠκῃ", _, "~ι~κῠκωσῐ|~ι~κῠκωσῐν";
    perfect_opt_act_1: "~ι~κῠκοιμῐ|~ι~κῠκοιην", _, _;
    perfect_opt_act_2: "~ι~κῠκοις|~ι~κῠκοιης", _, _;
    perfect_opt_act_3: "~ι~κῠκοι|~ι~κῠκοιη", _, _;
    perfect_imp_act_2: "~ι~κῠκε", _, _;
    perfect_imp_mid_2: _, _, "~ι~κῠσθε";
    perfect_imp_mid_3: _, _, "~ι~κῠσθων";
    perfect_imp_pass_2: "~ι~κῠσο", _, "~ι~κῠσθε";
    perfect_imp_pass_3: "~ι~κῠσθω", "~ι~κῠσθων", "~ι~κῠσθων";
    infinitive_pres: "~~κειν", _, "~~κεσθαι";
    infinitive_fut: "~~ξειν", _, _;
    infinitive_aor: "~~κῡσαι", _, _;
    infinitive_perf: _, "~ι~κῠσθαι", _;
    participle_pres: "~~κων", _, "~~κομενος";
    participle_aor: "~~κυσᾱς", "~~κυσᾰμενος", _;
    participle_perf: "~ι~κῠκως", _, "~ι~κῠμενος";
    ),
    // Τρῑβω: 3 words (τρῑβω, θλῑβω, φλῑβω). Like Βουλευω, except in 153 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("τρῑβω", 3, "~ῑβω", like Βουλευω;
    stem future_pass: "~ῐβησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ιψ- like ποιεω";
    stem aorist_ind_pass: "~ιφθην- like ἐχω";
    stem aorist_pass: "~ιφθ- like ἐχω";
    stem pluperfect_act: "~ῐφ- like χρημᾰτῐζω";
    ind_act_1: "~ῑβω", _, "~ῑβομεν";
    ind_act_2: "~ῑβεις", "~ῑβετον", _;
    ind_act_3: "~ῑβει", _, "~ῑβουσῐ|~ῑβουσῐν";
    ind_mid_1: _, _, "~ῑβομεθᾰ";
    ind_mid_2: "~ῑβῃ|~ῑβει", _, _;
    ind_mid_3: _, "~ῑβεσθον", _;
    ind_pass_1: "~ῑβομαι", _, _;
    ind_pass_2: _, "~ῑβεσθον", "~ῑβεσθε";
    ind_pass_3: "~ῑβεται", "~ῑβεσθον", "~ῑβονται";
    subj_act_1: "~ῑβω", _, "~ῑβωμεν";
    subj_act_2: "~ῑβῃς", _, _;
    subj_act_3: "~ῑβῃ", "~ῑβητον", "~ῑβωσῐ|~ῑβωσῐν";
    subj_mid_1: _, _, "~ῑβωμεθᾰ";
    subj_mid_2: _, "~ῑβησθον", _;
    subj_mid_3: "~ῑβηται", _, "~ῑβωνται";
    subj_pass_1: "~ῑβωμαι", _, _;
    subj_pass_2: _, _, "~ῑβησθε";
    opt_act_1: "~ῑβοιμῐ", _, "~ῑβοιμεν";
    opt_act_2: "~ῑβοις", "~ῑβοιτον", _;
    opt_act_3: "~ῑβοι", "~ῑβοιτην", "~ῑβοιεν";
    opt_mid_1: "~ῑβοιμην", _, "~ῑβοιμεθᾰ";
    opt_mid_2: "~ῑβοιο", _, "~ῑβοισθε";
    opt_mid_3: "~ῑβοιτο", "~ῑβοισθην", "~ῑβοιντο";
    opt_pass_2: _, "~ῑβοισθον", _;
    imp_act_2: "~ῑβε", _, "~ῑβετε";
    imp_act_3: _, _, "~ῑβοντων";
    imp_mid_2: "~ῑβου", "~ῑβεσθον", "~ῑβεσθε";
    imp_mid_3: _, _, "~ῑβεσθων";
    imp_pass_2: _, "~ῑβεσθον", "~ῑβεσθε";
    imp_pass_3: "~ῑβεσθω", "~ῑβεσθων", "~ῑβεσθων";
    imperfect_act_1: "ἐ~ῑβον", _, _;
    imperfect_act_2: "ἐ~ῑβες", "ἐ~ῑβετον", "ἐ~ῑβετε";
    imperfect_act_3: "ἐ~ῑβε|ἐ~ῑβεν", _, "ἐ~ῑβον";
    imperfect_mid_1: "ἐ~ῑβομην", _, "ἐ~ῑβομεθᾰ";
    imperfect_mid_2: "ἐ~ῑβου", _, _;
    imperfect_mid_3: "ἐ~ῑβετο", _, "ἐ~ῑβοντο";
    imperfect_pass_2: _, "ἐ~ῑβεσθον", "ἐ~ῑβεσθε";
    imperfect_pass_3: _, "ἐ~ῑβεσθην", _;
    future_ind_act_1: "~ιψω", _, "~ιψομεν";
    future_ind_act_2: "~ιψεις", "~ιψετον", _;
    future_ind_act_3: _, _, "~ιψουσῐ|~ιψουσῐν";
    future_ind_mid_1: "~ιψομαι", _, _;
    future_ind_mid_2: "~ιψῃ|~ιψει", "~ιψεσθον", _;
    future_ind_mid_3: "~ιψεται", _, _;
    future_opt_act_1: "~ιψοιμῐ", _, "~ιψοιμεν";
    future_opt_act_2: "~ιψοις", "~ιψοιτον", _;
    future_opt_act_3: "~ιψοι", "~ιψοιτην", _;
    future_opt_mid_1: "~ιψοιμην", _, "~ιψοιμεθᾰ";
    future_opt_mid_3: _, "~ιψοισθην", _;
    aorist_subj_act_1: "~ιψω", _, _;
    aorist_subj_act_2: "~ιψῃς", "~ιψητον", _;
    aorist_subj_act_3: _, _, "~ιψωσῐ|~ιψωσῐν";
    aorist_subj_mid_1: "~ιψωμαι", _, _;
    aorist_subj_mid_2: "~ιψῃ", "~ιψησθον", _;
    aorist_opt_act_1: "~ιψαιμῐ", _, "~ιψαιμεν";
    aorist_opt_act_2: "~ιψειᾰς|~ιψαις", _, _;
    aorist_opt_act_3: "~ιψειε|~ιψειεν|~ιψαι", _, "~ιψειᾰν|~ιψαιεν";
    aorist_opt_mid_1: "~ιψαιμην", _, _;
    aorist_imp_act_2: "~ῑψον", _, "~ιψᾰτε";
    aorist_imp_act_3: _, _, "~ιψᾰντων";
    aorist_imp_mid_2: "~ῑψαι", "~ιψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ιψᾰσθων";
    perfect_ind_act_1: "%ε~ῐφᾰ", _, "%ε~ῐφᾰμεν";
    perfect_ind_act_2: _, "%ε~ῐφᾰτον", _;
    perfect_ind_act_3: "%ε~ῐφε|%ε~ῐφεν", _, "%ε~ῐφᾱσῐ|%ε~ῐφᾱσῐν";
    perfect_ind_mid_1: _, _, "%ε~ῑμμεθᾰ";
    perfect_ind_mid_2: "%ε~ῑψαι", "%ε~ῑφθον", _;
    perfect_ind_mid_3: "%ε~ῑπται", _, "%ε~ῑφᾰται";
    perfect_ind_pass_1: "%ε~ῑμμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ῑφθε";
    perfect_subj_act_1: "%ε~ῐφω", _, _;
    perfect_subj_act_2: "%ε~ῐφῃς", "%ε~ῐφητον", "%ε~ῐφητε";
    perfect_subj_act_3: "%ε~ῐφῃ", _, "%ε~ῐφωσῐ|%ε~ῐφωσῐν";
    perfect_opt_act_1: "%ε~ῐφοιμῐ|%ε~ῐφοιην", _, _;
    perfect_opt_act_2: "%ε~ῐφοις|%ε~ῐφοιης", _, _;
    perfect_opt_act_3: "%ε~ῐφοι|%ε~ῐφοιη", _, _;
    perfect_imp_act_2: "%ε~ῐφε", _, _;
    perfect_imp_mid_2: _, _, "%ε~ῑφθε";
    perfect_imp_mid_3: _, _, "%ε~ῑφθων";
    perfect_imp_pass_2: "%ε~ῑψο", _, "%ε~ῑφθε";
    perfect_imp_pass_3: "%ε~ῑφθω", "%ε~ῑφθων", "%ε~ῑφθων";
    pluperfect_mid_1: "ἐ%ε~ῐμμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ῐψο", "ἐ%ε~ῐφθον", _;
    pluperfect_mid_3: "ἐ%ε~ῐπτο", _, "ἐ%ε~ῐφᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~ῐμμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ῐφθε";
    pluperfect_pass_3: _, "ἐ%ε~ῐφθην", _;
    infinitive_pres: "~ῑβειν", _, "~ῑβεσθαι";
    infinitive_fut: "~ιψειν", _, _;
    infinitive_aor: "~ῑψαι", _, _;
    infinitive_perf: _, "%ε~ῑφθαι", _;
    participle_pres: "~ῑβων", _, "~ῑβομενος";
    participle_aor: "~ιψᾱς", "~ιψᾰμενος", _;
    participle_perf: "%ε~ῐφως", _, "%ε~ῑμμενος";
    ),
    // Ἀντεχω: 3 words (μετεχω, κᾰτεχω, ἀντεχω). Like Κειμαι, except in 144 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀντεχω", 3, "~τεχω", like Κειμαι;
    stem aorist_ind: "~τεσχ- like πᾰρεχω";
    ind_act_1: "~τεχω", _, "~τεχομεν";
    ind_act_2: "~τεχεις", "~τεχετον", _;
    ind_act_3: "~τεχει", _, "~τεχουσῐ|~τεχουσῐν";
    ind_mid_1: _, _, "~τεχομεθᾰ";
    ind_mid_2: "~τεχῃ|~τεχει", _, _;
    ind_mid_3: _, "~τεχεσθον", _;
    ind_pass_1: "~τεχομαι", _, _;
    ind_pass_2: _, "~τεχεσθον", "~τεχεσθε";
    ind_pass_3: "~τεχεται", "~τεχεσθον", "~τεχονται";
    subj_act_1: "~τεχω", _, "~τεχωμεν";
    subj_act_2: "~τεχῃς", _, _;
    subj_act_3: "~τεχῃ", "~τεχητον", "~τεχωσῐ|~τεχωσῐν";
    subj_mid_1: _, _, "~τεχωμεθᾰ";
    subj_mid_2: _, "~τεχησθον", _;
    subj_mid_3: "~τεχηται", _, "~τεχωνται";
    subj_pass_1: "~τεχωμαι", _, _;
    subj_pass_2: _, _, "~τεχησθε";
    opt_act_1: "~τεχοιμῐ", _, "~τεχοιμεν";
    opt_act_2: "~τεχοις", "~τεχοιτον", _;
    opt_act_3: "~τεχοι", "~τεχοιτην", "~τεχοιεν";
    opt_mid_1: "~τεχοιμην", _, "~τεχοιμεθᾰ";
    opt_mid_2: "~τεχοιο", _, "~τεχοισθε";
    opt_mid_3: "~τεχοιτο", "~τεχοισθην", "~τεχοιντο";
    opt_pass_2: _, "~τεχοισθον", _;
    imp_act_2: "~τεχε", _, "~τεχετε";
    imp_act_3: _, _, "~τεχοντων";
    imp_mid_2: "~τεχου", "~τεχεσθον", "~τεχεσθε";
    imp_mid_3: _, _, "~τεχεσθων";
    imp_pass_2: _, "~τεχεσθον", "~τεχεσθε";
    imp_pass_3: "~τεχεσθω", "~τεχεσθων", "~τεχεσθων";
    imperfect_act_1: "~τειχον", _, _;
    imperfect_act_2: "~τειχες", "~τειχετον", "~τειχετε";
    imperfect_act_3: "~τειχε|~τειχεν", _, "~τειχον";
    imperfect_mid_1: "~τειχομην", _, "~τειχομεθᾰ";
    imperfect_mid_2: "~τειχου", _, _;
    imperfect_mid_3: "~τειχετο", _, "~τειχοντο";
    imperfect_pass_2: _, "~τειχεσθον", "~τειχεσθε";
    imperfect_pass_3: _, "~τειχεσθην", _;
    future_ind_act_1: "~θεξω", _, "~θεξομεν";
    future_ind_act_2: "~θεξεις", "~θεξετον", _;
    future_ind_act_3: _, _, "~θεξουσῐ|~θεξουσῐν";
    future_ind_mid_1: "~θεξομαι", _, _;
    future_ind_mid_2: "~θεξῃ|~θεξει", "~θεξεσθον", _;
    future_ind_mid_3: "~θεξεται", _, _;
    future_opt_act_1: "~θεξοιμῐ", _, "~θεξοιμεν";
    future_opt_act_2: "~θεξοις", "~θεξοιτον", _;
    future_opt_act_3: "~θεξοι", "~θεξοιτην", _;
    future_opt_mid_1: "~θεξοιμην", _, "~θεξοιμεθᾰ";
    future_opt_mid_3: _, "~θεξοισθην", _;
    aorist_subj_act_1: "~τᾰσχω", _, _;
    aorist_subj_act_2: "~τᾰσχῃς", "~τᾰσχητον", _;
    aorist_subj_mid_1: "~τᾰσχωμαι", _, _;
    aorist_subj_mid_2: "~τᾰσχῃ", "~τᾰσχησθον", _;
    aorist_opt_act_1: "~τᾰσχοιμῐ", _, "~τᾰσχοιμεν";
    aorist_opt_act_2: "~τᾰσχοις", _, _;
    aorist_opt_act_3: "~τᾰσχοι", _, "~τᾰσχοιεν";
    aorist_opt_mid_1: "~τᾰσχοιμην", _, _;
    aorist_imp_act_2: "~τᾰσχε", _, "~τᾰσχετε";
    aorist_imp_act_3: _, _, "~τᾰσχοντων";
    aorist_imp_mid_2: "~τᾰσχου", "~τᾰσχεσθον", _;
    aorist_imp_mid_3: _, _, "~τᾰσχεσθων";
    perfect_ind_act_1: "~τεσχηκᾰ", _, "~τεσχηκᾰμεν";
    perfect_ind_act_2: _, "~τεσχηκᾰτον", _;
    perfect_ind_act_3: "~τεσχηκε|~τεσχηκεν", _, "~τεσχηκᾱσῐ|~τεσχηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~τεσχημεθᾰ";
    perfect_ind_mid_2: "~τεσχησαι", "~τεσχησθον", _;
    perfect_ind_mid_3: "~τεσχηται", _, "~τεσχηνται";
    perfect_ind_pass_1: "~τεσχημαι", _, _;
    perfect_ind_pass_2: _, _, "~τεσχησθε";
    perfect_subj_act_1: "~τεσχηκω", _, _;
    perfect_subj_act_2: "~τεσχηκῃς", "~τεσχηκητον", "~τεσχηκητε";
    perfect_subj_act_3: "~τεσχηκῃ", _, "~τεσχηκωσῐ|~τεσχηκωσῐν";
    perfect_opt_act_1: "~τεσχηκοιμῐ|~τεσχηκοιην", _, _;
    perfect_opt_act_2: "~τεσχηκοις|~τεσχηκοιης", _, _;
    perfect_opt_act_3: "~τεσχηκοι|~τεσχηκοιη", _, _;
    perfect_imp_act_2: "~τεσχηκε", _, _;
    perfect_imp_mid_2: _, _, "~τεσχησθε";
    perfect_imp_mid_3: _, _, "~τεσχησθων";
    perfect_imp_pass_2: "~τεσχησο", _, "~τεσχησθε";
    perfect_imp_pass_3: "~τεσχησθω", "~τεσχησθων", "~τεσχησθων";
    infinitive_pres: "~τεχειν", _, "~τεχεσθαι";
    infinitive_fut: "~θεξειν", _, _;
    infinitive_aor: "~τᾰσχειν", _, _;
    infinitive_perf: _, "~τεσχησθαι", _;
    participle_pres: "~τεχων", _, "~τεχομενος";
    participle_aor: "~τᾰσχων", "~τᾰσχομενος", _;
    participle_perf: "~τεσχηκως", _, "~τεσχημενος";
    ),
    // Ποιμαινω: 3 words (ἐπῐφαινω, ποιμαινω, χλῑαινω). Like Πῠνθανομαι, except in 135 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ποιμαινω", 3, "~~ινω", like Πῠνθανομαι;
    stem aorist_ind: "~~ν- like ἐχω";
    stem aorist_ind_pass: "~~νθην- like ἐχω";
    stem aorist_pass: "~~νθ- like ἐχω";
    ind_act_1: "~~ινω", _, "~~ινομεν";
    ind_act_2: "~~ινεις", "~~ινετον", _;
    ind_act_3: "~~ινει", _, "~~ινουσῐ";
    ind_mid_1: derived, _, "~~ινομεθᾰ";
    ind_mid_2: "~~ινῃ", _, _;
    ind_mid_3: _, "~~ινεσθον", _;
    ind_pass_1: "~~ινομαι", _, _;
    ind_pass_2: derived, "~~ινεσθον", "~~ινεσθε";
    ind_pass_3: "~~ινεται", "~~ινεσθον", "~~ινονται";
    subj_act_1: "~~ινω", _, "~~ινωμεν";
    subj_act_2: "~~ινῃς", _, _;
    subj_act_3: "~~ινῃ", "~~ινητον", "~~ινωσῐ";
    subj_mid_1: derived, _, "~~ινωμεθᾰ";
    subj_mid_2: derived, "~~ινησθον", _;
    subj_mid_3: "~~ινηται", _, "~~ινωνται";
    subj_pass_1: "~~ινωμαι", _, _;
    subj_pass_2: derived, _, "~~ινησθε";
    opt_act_1: "~~ινοιμῐ", _, "~~ινοιμεν";
    opt_act_2: "~~ινοις", "~~ινοιτον", _;
    opt_act_3: "~~ινοι", "~~ινοιτην", "~~ινοιεν";
    opt_mid_1: "~~ινοιμην", _, "~~ινοιμεθᾰ";
    opt_mid_2: "~~ινοιο", _, "~~ινοισθε";
    opt_mid_3: "~~ινοιτο", "~~ινοισθην", "~~ινοιντο";
    opt_pass_2: _, "~~ινοισθον", _;
    imp_act_2: "~~ινε", _, "~~ινετε";
    imp_act_3: _, _, "~~ινοντων";
    imp_mid_2: "~~ινου", "~~ινεσθον", "~~ινεσθε";
    imp_mid_3: derived, _, "~~ινεσθων";
    imp_pass_2: _, "~~ινεσθον", "~~ινεσθε";
    imp_pass_3: "~~ινεσθω", "~~ινεσθων", "~~ινεσθων";
    imperfect_act_1: "ε~~ινον", _, _;
    imperfect_act_2: "ε~~ινες", "ε~~ινετον", "ε~~ινετε";
    imperfect_act_3: "ε~~ινε", _, "ε~~ινον";
    imperfect_mid_1: "ε~~ινομην", _, "ε~~ινομεθᾰ";
    imperfect_mid_2: "ε~~ινου", derived, _;
    imperfect_mid_3: "ε~~ινετο", _, "ε~~ινοντο";
    imperfect_pass_2: _, "ε~~ινεσθον", "ε~~ινεσθε";
    imperfect_pass_3: _, "ε~~ινεσθην", _;
    future_ind_act_1: "~~νω", _, "~~νουμεν";
    future_ind_act_2: "~~νεις", "~~νειτον", _;
    future_ind_act_3: _, _, "~~νουσῐ";
    future_ind_mid_1: "~~νουμαι", _, _;
    future_ind_mid_2: "~~νει", "~~νεισθον", _;
    future_ind_mid_3: "~~νειται", _, _;
    future_opt_act_1: "~~νοιην", _, "~~νοιμεν";
    future_opt_act_2: "~~νοιης", "~~νοιτον", _;
    future_opt_act_3: "~~νοιη", "~~νοιτην", _;
    future_opt_mid_1: "~~νοιμην", _, "~~νοιμεθᾰ";
    future_opt_mid_3: _, "~~νοισθην", _;
    aorist_subj_act_1: "~~νω", _, _;
    aorist_subj_act_2: "~~νῃς", "~~νητον", _;
    aorist_subj_mid_1: "~~νωμαι", _, _;
    aorist_subj_mid_2: "~~νῃ", "~~νησθον", _;
    aorist_opt_act_1: "~~ναιμῐ", _, "~~ναιμεν";
    aorist_opt_act_2: "~~νειᾰς", _, _;
    aorist_opt_act_3: "~~νειε", _, "~~νειᾰν";
    aorist_opt_mid_1: "~~ναιμην", _, _;
    aorist_imp_act_2: "~~νον", _, "~~νᾰτε";
    aorist_imp_act_3: _, _, "~~νᾰντων";
    aorist_imp_mid_2: "~~ναι", "~~νᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~νᾰσθων";
    perfect_ind_mid_1: _, _, "%ε~~σμεθᾰ";
    perfect_ind_mid_2: "%ε~~νσαι", "%ε~~νθον", _;
    perfect_ind_mid_3: "%ε~~νται", _, "%ε~~νᾰται";
    perfect_ind_pass_1: "%ε~~σμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~~νθε";
    perfect_imp_mid_2: _, _, "%ε~~νθε";
    perfect_imp_mid_3: _, _, "%ε~~νθων";
    perfect_imp_pass_2: "%ε~~νσο", _, "%ε~~νθε";
    perfect_imp_pass_3: "%ε~~νθω", "%ε~~νθων", "%ε~~νθων";
    infinitive_pres: "~~ινειν", _, "~~ινεσθαι";
    infinitive_fut: "~~νειν", _, _;
    infinitive_aor: "~ᾱν~ι", _, _;
    infinitive_perf: _, "%ε~ᾱνθ~ι", _;
    participle_pres: "~~ινων", _, "~~ινομενος";
    participle_aor: "~~νᾱς", "~~νᾰμενος", _;
    participle_perf: _, _, "%ε~~σμενος";
    ),
    // Σῠνιστημῐ: 3 words (σῠνιστημῐ, ξῠνιστημῐ, ὀνινημῐ). Like Βᾰθῠνω, except in 141 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("σῠνιστημῐ", 3, "~νι~ημῐ", like Βᾰθῠνω;
    stem aorist_ind: "~νε~- like σῠνιστημῐ";
    stem aorist_ind_pass: "~νε~ᾰθην- like εὐλογεω";
    stem aorist_pass: "~~ᾰθ- like ἐγειρω";
    stem pluperfect_act: "~νει~ηκ- like εὐλογεω";
    ind_act_1: "~νι~ημῐ", _, "~νι~ᾰμεν";
    ind_act_2: "~νι~ης", "~νι~ᾰτον", _;
    ind_act_3: "~νι~ησῐ|~νι~ησῐν", _, "~νι~ᾱσῐ|~νι~ᾱσῐν";
    ind_mid_1: _, _, "~νι~ᾰμεθᾰ";
    ind_mid_2: "~νι~ᾰσαι", _, _;
    ind_mid_3: _, "~νι~ᾰσθον", _;
    ind_pass_1: "~νι~ᾰμαι", _, _;
    ind_pass_2: _, "~νι~ᾰσθον", "~νι~ᾰσθε";
    ind_pass_3: "~νι~ᾰται", "~νι~ᾰσθον", "~νι~ᾰνται";
    subj_act_1: "~νι~ω", _, "~νι~ωμεν";
    subj_act_2: "~νι~ῃς", _, _;
    subj_act_3: "~νι~ῃ", "~νι~ητον", "~νι~ωσῐ|~νι~ωσῐν";
    subj_mid_1: _, _, "~νι~ωμεθᾰ";
    subj_mid_2: _, "~νι~ησθον", _;
    subj_mid_3: "~νι~ηται", _, "~νι~ωνται";
    subj_pass_1: "~νι~ωμαι", _, _;
    subj_pass_2: _, _, "~νι~ησθε";
    opt_act_1: "~νι~αιην", _, "~νι~αιμεν|~νι~αιημεν";
    opt_act_2: "~νι~αιης", "~νι~αιτον|~νι~αιητον", "~νι~αιτε|~νι~αιητε";
    opt_act_3: "~νι~αιη", "~νι~αιτην|~νι~αιητην", "~νι~αιεν|~νι~αιησᾰν";
    opt_mid_1: "~νι~αιμην", _, "~νι~αιμεθᾰ";
    opt_mid_2: "~νι~αιο", _, "~νι~αισθε";
    opt_mid_3: "~νι~αιτο", "~νι~αισθην", "~νι~αιντο";
    opt_pass_2: _, "~νι~αισθον", _;
    imp_act_2: "~νι~η", _, "~νι~ᾰτε";
    imp_act_3: _, _, "~νι~ᾰντων";
    imp_mid_2: "~νι~ᾰσο|~νι~ω", "~νι~ᾰσθον", "~νι~ᾰσθε";
    imp_mid_3: _, _, "~νι~ᾰσθων";
    imp_pass_2: _, "~νι~ᾰσθον", "~νι~ᾰσθε";
    imp_pass_3: "~νι~ᾰσθω", "~νι~ᾰσθων", "~νι~ᾰσθων";
    imperfect_act_1: "~νῑ~ην", _, _;
    imperfect_act_2: "~νῑ~ης", "~νῑ~ᾰτον", "~νῑ~ᾰτε";
    imperfect_act_3: "~νῑ~η", _, "~νῑ~ᾰσᾰν";
    imperfect_mid_1: "~νῑ~ᾰμην", _, "~νῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~νῑ~ᾰσο", _, _;
    imperfect_mid_3: "~νῑ~ᾰτο", _, "~νῑ~ᾰντο";
    imperfect_pass_2: _, "~νῑ~ᾰσθον", "~νῑ~ᾰσθε";
    imperfect_pass_3: _, "~νῑ~ᾰσθην", _;
    future_ind_act_1: "~~ησω", _, "~~ησομεν";
    future_ind_act_2: "~~ησεις", "~~ησετον", _;
    future_ind_act_3: _, _, "~~ησουσῐ|~~ησουσῐν";
    future_ind_mid_1: "~~ησομαι", _, _;
    future_ind_mid_2: "~~ησῃ|~~ησει", "~~ησεσθον", _;
    future_ind_mid_3: "~~ησεται", _, _;
    future_opt_act_1: "~~ησοιμῐ", _, "~~ησοιμεν";
    future_opt_act_2: "~~ησοις", "~~ησοιτον", _;
    future_opt_act_3: "~~ησοι", "~~ησοιτην", _;
    future_opt_mid_1: "~~ησοιμην", _, "~~ησοιμεθᾰ";
    future_opt_mid_3: _, "~~ησοισθην", _;
    aorist_subj_act_1: "~~ω", _, _;
    aorist_subj_act_2: "~~ῃς", "~~ητον", _;
    aorist_subj_mid_1: "~~ωμαι", _, _;
    aorist_subj_mid_2: "~~ῃ", "~~ησθον", _;
    aorist_opt_act_1: "~~αιην", _, "~~αιμεν";
    aorist_opt_act_2: "~~αιης", _, _;
    aorist_opt_act_3: "~~αιη", _, "~~αιεν";
    aorist_opt_mid_1: "~~αιμην", _, _;
    aorist_imp_act_2: "~~ηθῐ", _, "~~ητε";
    aorist_imp_act_3: _, _, "~~ᾰντων";
    aorist_imp_mid_2: "~~ω", "~~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~ᾰσθων";
    perfect_ind_act_1: "~νε~ηκᾰ", _, "~νε~ηκᾰμεν";
    perfect_ind_act_2: _, "~νε~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "~νε~ηκᾱσῐ";
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "~νε~ηκω", _, _;
    perfect_subj_act_2: "~νε~ηκῃς", "~νε~ηκητον", "~νε~ηκητε";
    perfect_subj_act_3: "~νε~ηκῃ", _, "~νε~ηκωσῐ";
    perfect_imp_act_2: "~νε~ηκε", _, _;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    infinitive_pres: "~νι~ᾰναι", _, "~νι~ᾰσθαι";
    infinitive_fut: "~~ησειν", _, _;
    infinitive_aor: "~~ηναι", _, _;
    infinitive_perf: _, derived, _;
    participle_pres: "~νι~ᾱς", _, "~νι~ᾰμενος";
    participle_aor: "~~ᾱς", "~~ᾰμενος", _;
    participle_perf: "~νε~ηκως", _, derived;
    ),
    // Ἰαινω: 3 words (ἰαινω, σαιρω, ἐναιρω). Like Δῐαιρω, except in 98 cells: imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, imperfect_act_3 pl, ….
    conjugation!("ἰαινω", 3, "~αι~ω", like Δῐαιρω;
    stem aorist_ind: "~η~- like ἐλπω";
    stem aorist_ind_pass: "~α~θην- like εὐλογεω";
    stem aorist_pass: "~α~θ- like ἐχω";
    imperfect_act_1: "~αι~ον", _, _;
    imperfect_act_2: "~αι~ες", "~αι~ετον", "~αι~ετε";
    imperfect_act_3: "~αι~ε", _, "~αι~ον";
    imperfect_mid_1: "~αι~ομην", _, "~αι~ομεθᾰ";
    imperfect_mid_2: "~αι~ου", _, _;
    imperfect_mid_3: "~αι~ετο", _, "~αι~οντο";
    imperfect_pass_2: _, "~αι~εσθον", "~αι~εσθε";
    imperfect_pass_3: _, "~αι~εσθην", _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, _;
    future_ind_act_3: _, _, derived;
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_2: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_3: _, derived, _;
    aorist_subj_act_1: "~η~ω", _, _;
    aorist_subj_act_2: "~η~ῃς", "~η~ητον", _;
    aorist_subj_act_3: _, _, "~η~ωσῐ|~η~ωσῐν";
    aorist_subj_mid_1: "~η~ωμαι", _, _;
    aorist_subj_mid_2: "~η~ῃ", "~η~ησθον", _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_1: "~η~αιμῐ", _, "~η~αιμεν";
    aorist_opt_act_2: "~η~ειᾰς|~η~αις", _, _;
    aorist_opt_act_3: "~η~ειε|~η~ειεν|~η~αι", _, "~η~ειᾰν|~η~αιεν";
    aorist_opt_mid_1: "~η~αιμην", _, _;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: "~η~ον", _, "~η~ᾰτε";
    aorist_imp_act_3: _, _, "~η~ᾰντων";
    aorist_imp_mid_2: "~η~αι", "~η~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~η~ᾰσθων";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, _;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: derived, _, _;
    perfect_imp_act_2: derived, _, _;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    infinitive_fut: derived, _, _;
    infinitive_aor: "~η~αι", _, _;
    infinitive_perf: _, derived, _;
    participle_aor: "~η~ᾱς", "~η~ᾰμενος", _;
    participle_perf: derived, _, derived;
    ),
    // Ἐφῐστημῐ: 3 words (ἐφῐστημῐ, ἀφῐστημῐ, ὑφῐστημῐ). Like Ἀποκωλῡω, except in 205 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐφῐστημῐ", 3, "~φῐ~ημῐ", like Ἀποκωλῡω;
    stem future_pass: "~πο~ησ- like ἐχω";
    stem aorist_ind: "~πε~- like ἐφῐστημῐ";
    stem aorist_ind_pass: "~πε~ᾰθην- like εὐλογεω";
    stem aorist_pass: "~πο~ᾰθ- like ποιεω";
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
    opt_mid_2: "~φῐ~αιο", _, "~φῐ~αισθε";
    opt_mid_3: "~φῐ~αιτο", "~φῐ~αισθην", "~φῐ~αιντο";
    opt_pass_2: _, "~φῐ~αισθον", _;
    imp_act_2: "~φῐ~η", _, "~φῐ~ᾰτε";
    imp_act_3: _, _, "~φῐ~ᾰντων";
    imp_mid_2: "~φῐ~ᾰσο|~φῐ~ω", "~φῐ~ᾰσθον", "~φῐ~ᾰσθε";
    imp_mid_3: _, _, "~φῐ~ᾰσθων";
    imp_pass_2: _, "~φῐ~ᾰσθον", "~φῐ~ᾰσθε";
    imp_pass_3: "~φῐ~ᾰσθω", "~φῐ~ᾰσθων", "~φῐ~ᾰσθων";
    imperfect_act_1: "~φῑ~ην", _, _;
    imperfect_act_2: "~φῑ~ης", "~φῑ~ᾰτον", "~φῑ~ᾰτε";
    imperfect_act_3: "~φῑ~η", _, "~φῑ~ᾰσᾰν";
    imperfect_mid_1: "~φῑ~ᾰμην", _, "~φῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~φῑ~ᾰσο", _, _;
    imperfect_mid_3: "~φῑ~ᾰτο", _, "~φῑ~ᾰντο";
    imperfect_pass_2: _, "~φῑ~ᾰσθον", "~φῑ~ᾰσθε";
    imperfect_pass_3: _, "~φῑ~ᾰσθην", _;
    future_ind_act_1: "~πο~ησω", _, "~πο~ησομεν";
    future_ind_act_2: "~πο~ησεις", "~πο~ησετον", _;
    future_ind_act_3: _, _, "~πο~ησουσῐ|~πο~ησουσῐν";
    future_ind_mid_1: "~πο~ησομαι", _, _;
    future_ind_mid_2: "~πο~ησῃ|~πο~ησει", "~πο~ησεσθον", _;
    future_ind_mid_3: "~πο~ησεται", _, _;
    future_opt_act_1: "~πο~ησοιμῐ", _, "~πο~ησοιμεν";
    future_opt_act_2: "~πο~ησοις", "~πο~ησοιτον", _;
    future_opt_act_3: "~πο~ησοι", "~πο~ησοιτην", _;
    future_opt_mid_1: "~πο~ησοιμην", _, "~πο~ησοιμεθᾰ";
    future_opt_mid_3: _, "~πο~ησοισθην", _;
    aorist_subj_act_1: "~πο~ησω|~πο~ω", _, "~πο~ησωμεν|~πο~ωμεν";
    aorist_subj_act_2: "~πο~ησῃς|~πο~ῃς", "~πο~ησητον|~πο~ητον", "~πο~ησητε|~πο~ητε";
    aorist_subj_act_3: "~πο~ησῃ|~πο~ῃ", _, "~πο~ησωσῐ|~πο~ησωσῐν|~πο~ωσῐ|~πο~ωσῐν";
    aorist_subj_mid_1: "~πο~ησωμαι|~πο~ωμαι", _, "~πο~ησωμεθᾰ|~πο~ωμεθᾰ";
    aorist_subj_mid_2: derived, "~πο~ησησθον|~πο~ησθον", "~πο~ησησθε|~πο~ησθε";
    aorist_subj_mid_3: "~πο~ησηται|~πο~ηται", _, "~πο~ησωνται|~πο~ωνται";
    aorist_opt_act_1: "~πο~ησαιμῐ|~πο~αιην", _, "~πο~ησαιμεν|~πο~αιμεν|~πο~αιημεν";
    aorist_opt_act_2: "~πο~ησειᾰς|~πο~ησαις|~πο~αιης", "~πο~ησαιτον|~πο~αιτον|~πο~αιητον", "~πο~ησαιτε|~πο~αιτε|~πο~αιητε";
    aorist_opt_act_3: "~πο~ησειε|~πο~ησειεν|~πο~ησαι|~πο~αιη", "~πο~ησαιτην|~πο~αιτην|~πο~αιητην", "~πο~ησειᾰν|~πο~ησαιεν|~πο~αιεν|~πο~αιησᾰν";
    aorist_opt_mid_1: "~πο~ησαιμην|~πο~αιμην", _, "~πο~ησαιμεθᾰ|~πο~αιμεθᾰ";
    aorist_opt_mid_2: "~πο~ησαιο|~πο~αιο", "~πο~ησαισθον|~πο~αισθον", "~πο~ησαισθε|~πο~αισθε";
    aorist_opt_mid_3: "~πο~ησαιτο|~πο~αιτο", "~πο~ησαισθην|~πο~αισθην", "~πο~ησαιντο|~πο~αιντο";
    aorist_imp_act_2: "~πο~ησον|~πο~ηθῐ", "~πο~ησᾰτον|~πο~ητον", "~πο~ησᾰτε|~πο~ητε";
    aorist_imp_act_3: "~πο~ησᾰτω|~πο~ητω", "~πο~ησᾰτων|~πο~ητων", "~πο~ησᾰντων|~πο~ᾰντων";
    aorist_imp_mid_2: "~πο~ησαι|~πο~ω", "~πο~ησᾰσθον|~πο~ᾰσθον", "~πο~ησᾰσθε|~πο~ᾰσθε";
    aorist_imp_mid_3: "~πο~ησᾰσθω|~πο~ᾰσθω", "~πο~ησᾰσθων|~πο~ᾰσθων", derived;
    perfect_ind_act_1: "~φε~ᾰκᾰ|~φε~ηκᾰ", _, "~φε~ᾰκᾰμεν";
    perfect_ind_act_2: "~φε~ᾰκᾰς|~φε~ηκᾰς", "~φε~ᾰκᾰτον", _;
    perfect_ind_act_3: "~φε~ᾰκε|~φε~ᾰκεν|~φε~ηκε|~φε~ηκεν", _, "~φε~ᾰκᾱσῐ";
    perfect_ind_mid_1: _, _, "~φε~ᾰμεθᾰ";
    perfect_ind_mid_2: "~φε~ᾰσαι", "~φε~ᾰσθον", _;
    perfect_ind_mid_3: "~φε~ᾰται", _, "~φε~ᾰνται";
    perfect_ind_pass_1: "~φε~ᾰμαι", _, _;
    perfect_ind_pass_2: _, _, "~φε~ᾰσθε";
    perfect_subj_act_1: "~φε~ᾰκω", _, _;
    perfect_subj_act_2: "~φε~ᾰκῃς", "~φε~ᾰκητον", "~φε~ᾰκητε";
    perfect_subj_act_3: "~φε~ᾰκῃ", _, "~φε~ᾰκωσῐ";
    perfect_imp_act_2: "~φε~ᾰκε", _, _;
    perfect_imp_mid_2: _, _, "~φε~ᾰσθε";
    perfect_imp_mid_3: _, _, "~φε~ᾰσθων";
    perfect_imp_pass_2: "~φε~ᾰσο", _, "~φε~ᾰσθε";
    perfect_imp_pass_3: "~φε~ᾰσθω", "~φε~ᾰσθων", "~φε~ᾰσθων";
    pluperfect_act_1: "~φε~ηκειν", _, _;
    future_perfect_ind_act_1: "~φε~ηξω", _, "~φε~ηξομεν";
    future_perfect_ind_act_2: "~φε~ηξεις", "~φε~ηξετον", "~φε~ηξετε";
    future_perfect_ind_act_3: "~φε~ηξει", _, "~φε~ηξουσῐ|~φε~ηξουσῐν";
    future_perfect_ind_mid_1: "~φε~ηξομαι", _, _;
    future_perfect_ind_mid_2: "~φε~ηξῃ|~φε~ηξει", _, _;
    future_perfect_ind_mid_3: _, "~φε~ηξεσθον", _;
    future_perfect_ind_pass_1: _, _, "~φε~ηξομεθᾰ";
    future_perfect_ind_pass_2: _, "~φε~ηξεσθον", "~φε~ηξεσθε";
    future_perfect_ind_pass_3: "~φε~ηξεται", "~φε~ηξεσθον", "~φε~ηξονται";
    future_perfect_opt_act_1: "~φε~ηξοιμῐ", _, "~φε~ηξοιμεν";
    future_perfect_opt_act_2: "~φε~ηξοις", "~φε~ηξοιτον", "~φε~ηξοιτε";
    future_perfect_opt_act_3: "~φε~ηξοι", "~φε~ηξοιτην", "~φε~ηξοιεν";
    future_perfect_opt_pass_1: "~φε~ηξοιμην", _, "~φε~ηξοιμεθᾰ";
    future_perfect_opt_pass_2: "~φε~ηξοιο", "~φε~ηξοισθον", "~φε~ηξοισθε";
    future_perfect_opt_pass_3: "~φε~ηξοιτο", "~φε~ηξοισθην", "~φε~ηξοιντο";
    infinitive_pres: "~φῐ~ᾰναι", _, "~φῐ~ᾰσθαι";
    infinitive_fut: "~πο~ησειν", _, _;
    infinitive_aor: "~πο~ησαι|~πο~ηναι", "~πο~ησᾰσθαι|~πο~ᾰσθαι", _;
    infinitive_perf: _, "~φε~ᾰσθαι", _;
    infinitive_futp: "~φε~ηξειν", _, "~φε~ηξεσθαι";
    participle_pres: "~φῐ~ᾱς", _, "~φῐ~ᾰμενος";
    participle_aor: "~πο~ησᾱς|~πο~ᾱς", "~πο~ησᾰμενος|~πο~ᾰμενος", _;
    participle_perf: "~φε~ᾰκως", _, "~φε~ᾰμενος";
    participle_futp: "~φε~ηξων", _, "~φε~ηξομενος";
    ),
    // Δρᾰσσομαι: 3 words (πλῐσσομαι, δρᾰσσομαι, μῠσσομαι). Like Εἰμῐ, except in 63 cells: ind_act_1 sg, ind_act_3 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ….
    conjugation!("δρᾰσσομαι", 3, "~σσομαι", like Εἰμῐ;
    stem aorist_ind: "~ξ- like ἑπομαι";
    ind_act_1: "~σσομαι", _, _;
    ind_act_3: derived, _, _;
    ind_mid_1: "~σσομαι", _, "~σσομεθᾰ";
    ind_mid_2: "~σσῃ|~σσει", _, _;
    ind_mid_3: _, "~σσεσθον", _;
    subj_mid_1: "~σσωμαι", _, "~σσωμεθᾰ";
    subj_mid_2: "~σσῃ", "~σσησθον", _;
    subj_mid_3: "~σσηται", _, "~σσωνται";
    opt_mid_1: "~σσοιμην", _, "~σσοιμεθᾰ";
    opt_mid_2: "~σσοιο", _, "~σσοισθε";
    opt_mid_3: "~σσοιτο", "~σσοισθην", "~σσοιντο";
    imp_mid_2: "~σσου", "~σσεσθον", "~σσεσθε";
    imp_mid_3: "~σσεσθω", _, "~σσεσθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    imperfect_mid_1: "ἐ~σσομην", _, "ἐ~σσομεθᾰ";
    imperfect_mid_2: "ἐ~σσου", "ἐ~σσεσθον", _;
    imperfect_mid_3: "ἐ~σσετο", _, "ἐ~σσοντο";
    future_ind_mid_1: "~ξομαι", _, _;
    future_ind_mid_2: "~ξῃ|~ξει", "~ξεσθον", _;
    future_ind_mid_3: "~ξεται", _, _;
    future_opt_mid_1: "~ξοιμην", _, "~ξοιμεθᾰ";
    future_opt_mid_3: _, "~ξοισθην", _;
    aorist_subj_mid_1: "~ξωμαι", _, _;
    aorist_subj_mid_2: "~ξῃ", "~ξησθον", _;
    aorist_opt_mid_1: "~ξαιμην", _, _;
    aorist_imp_mid_2: "~ξαι", "~ξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ξᾰσθων";
    perfect_ind_mid_1: _, _, "%ε~γμεθᾰ";
    perfect_ind_mid_2: "%ε~ξαι", "%ε~χθον", _;
    perfect_ind_mid_3: "%ε~κται", _, "%ε~χᾰται";
    perfect_ind_pass_1: "%ε~γμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~χθε";
    perfect_imp_mid_2: _, _, "%ε~χθε";
    perfect_imp_mid_3: _, _, "%ε~χθων";
    perfect_imp_pass_2: "%ε~ξο", _, "%ε~χθε";
    perfect_imp_pass_3: "%ε~χθω", "%ε~χθων", "%ε~χθων";
    infinitive_perf: _, "%ε~χθαι", _;
    participle_aor: _, "~ξᾰμενος", _;
    participle_perf: _, _, "%ε~γμενος";
    ),
    // Πᾰρᾰκᾰλεω: 3 words (πᾰρᾰκᾰλεω, ἀνᾰκᾰλεω, κᾰτᾰκᾰλεω). Like Πεμπω, except in 156 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("πᾰρᾰκᾰλεω", 3, "~ᾰκᾰ~εω", like Πεμπω;
    stem future_pass: "~ᾰκεκ~ησ- like ἐχω";
    stem aorist_ind: "~εκᾰ~εσ- like εὐλογεω";
    stem aorist_ind_pass: "~εκ~ηθην- like εὐλογεω";
    stem aorist_pass: "~ᾰκ~ηθ- like ποιεω";
    stem pluperfect_act: "~εκεκ~ηκ- like εὐλογεω";
    ind_act_1: "~ᾰκᾰ~ω", _, "~ᾰκᾰ~ουμεν";
    ind_act_2: "~ᾰκᾰ~εις", "~ᾰκᾰ~ειτον", _;
    ind_act_3: "~ᾰκᾰ~ει", _, "~ᾰκᾰ~ουσῐ|~ᾰκᾰ~ουσῐν";
    ind_mid_1: _, _, "~ᾰκᾰ~ουμεθᾰ";
    ind_mid_2: "~ᾰκᾰ~ει|~ᾰκᾰ~ῃ", _, _;
    ind_mid_3: _, "~ᾰκᾰ~εισθον", _;
    ind_pass_1: "~ᾰκᾰ~ουμαι", _, _;
    ind_pass_2: _, "~ᾰκᾰ~εισθον", "~ᾰκᾰ~εισθε";
    ind_pass_3: "~ᾰκᾰ~ειται", "~ᾰκᾰ~εισθον", "~ᾰκᾰ~ουνται";
    subj_act_1: "~ᾰκᾰ~ω", _, "~ᾰκᾰ~ωμεν";
    subj_act_2: "~ᾰκᾰ~ῃς", _, _;
    subj_act_3: "~ᾰκᾰ~ῃ", "~ᾰκᾰ~ητον", "~ᾰκᾰ~ωσῐ|~ᾰκᾰ~ωσῐν";
    subj_mid_1: _, _, "~ᾰκᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ᾰκᾰ~ησθον", _;
    subj_mid_3: "~ᾰκᾰ~ηται", _, "~ᾰκᾰ~ωνται";
    subj_pass_1: "~ᾰκᾰ~ωμαι", _, _;
    subj_pass_2: _, _, "~ᾰκᾰ~ησθε";
    opt_act_1: "~ᾰκᾰ~οιην|~ᾰκᾰ~οιμῐ", _, "~ᾰκᾰ~οιμεν|~ᾰκᾰ~οιημεν";
    opt_act_2: "~ᾰκᾰ~οιης|~ᾰκᾰ~οις", "~ᾰκᾰ~οιτον|~ᾰκᾰ~οιητον", "~ᾰκᾰ~οιτε|~ᾰκᾰ~οιητε";
    opt_act_3: "~ᾰκᾰ~οιη|~ᾰκᾰ~οι", "~ᾰκᾰ~οιτην|~ᾰκᾰ~οιητην", "~ᾰκᾰ~οιεν|~ᾰκᾰ~οιησᾰν";
    opt_mid_1: "~ᾰκᾰ~οιμην", _, "~ᾰκᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰκᾰ~οιο", _, "~ᾰκᾰ~οισθε";
    opt_mid_3: "~ᾰκᾰ~οιτο", "~ᾰκᾰ~οισθην", "~ᾰκᾰ~οιντο";
    opt_pass_2: _, "~ᾰκᾰ~οισθον", _;
    imp_act_2: "~ᾰκᾰ~ει", _, "~ᾰκᾰ~ειτε";
    imp_act_3: _, _, "~ᾰκᾰ~ουντων";
    imp_mid_2: "~ᾰκᾰ~ου", "~ᾰκᾰ~εισθον", "~ᾰκᾰ~εισθε";
    imp_mid_3: _, _, "~ᾰκᾰ~εισθων";
    imp_pass_2: _, "~ᾰκᾰ~εισθον", "~ᾰκᾰ~εισθε";
    imp_pass_3: "~ᾰκᾰ~εισθω", "~ᾰκᾰ~εισθων", "~ᾰκᾰ~εισθων";
    imperfect_act_1: "~εκᾰ~ουν", _, _;
    imperfect_act_2: "~εκᾰ~εις", "~εκᾰ~ειτον", "~εκᾰ~ειτε";
    imperfect_act_3: "~εκᾰ~ει", _, "~εκᾰ~ουν";
    imperfect_mid_1: "~εκᾰ~ουμην", _, "~εκᾰ~ουμεθᾰ";
    imperfect_mid_2: "~εκᾰ~ου", _, _;
    imperfect_mid_3: "~εκᾰ~ειτο", _, "~εκᾰ~ουντο";
    imperfect_pass_2: _, "~εκᾰ~εισθον", "~εκᾰ~εισθε";
    imperfect_pass_3: _, "~εκᾰ~εισθην", _;
    future_ind_act_1: "~ᾰκᾰ~ω", _, "~ᾰκᾰ~ουμεν";
    future_ind_act_2: "~ᾰκᾰ~εις", "~ᾰκᾰ~ειτον", _;
    future_ind_act_3: _, _, "~ᾰκᾰ~ουσῐ|~ᾰκᾰ~ουσῐν";
    future_ind_mid_1: "~ᾰκᾰ~ουμαι", _, _;
    future_ind_mid_2: "~ᾰκᾰ~ει|~ᾰκᾰ~ῃ", "~ᾰκᾰ~εισθον", _;
    future_ind_mid_3: "~ᾰκᾰ~ειται", _, _;
    future_opt_act_1: "~ᾰκᾰ~οιην|~ᾰκᾰ~οιμῐ", _, "~ᾰκᾰ~οιμεν|~ᾰκᾰ~οιημεν";
    future_opt_act_2: "~ᾰκᾰ~οιης|~ᾰκᾰ~οις", "~ᾰκᾰ~οιτον|~ᾰκᾰ~οιητον", "~ᾰκᾰ~οιτε|~ᾰκᾰ~οιητε";
    future_opt_act_3: "~ᾰκᾰ~οιη|~ᾰκᾰ~οι", "~ᾰκᾰ~οιτην|~ᾰκᾰ~οιητην", "~ᾰκᾰ~οιεν|~ᾰκᾰ~οιησᾰν";
    future_opt_mid_1: "~ᾰκᾰ~οιμην", _, "~ᾰκᾰ~οιμεθᾰ";
    future_opt_mid_3: _, "~ᾰκᾰ~οισθην", _;
    aorist_subj_act_1: "~ᾰκᾰ~εσω", _, _;
    aorist_subj_act_2: "~ᾰκᾰ~εσῃς", "~ᾰκᾰ~εσητον", _;
    aorist_subj_act_3: _, _, "~ᾰκᾰ~εσωσῐ|~ᾰκᾰ~εσωσῐν";
    aorist_subj_mid_1: "~ᾰκᾰ~εσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰκᾰ~εσῃ", "~ᾰκᾰ~εσησθον", _;
    aorist_opt_act_1: "~ᾰκᾰ~εσαιμῐ", _, "~ᾰκᾰ~εσαιμεν";
    aorist_opt_act_2: "~ᾰκᾰ~εσειᾰς|~ᾰκᾰ~εσαις", _, _;
    aorist_opt_act_3: "~ᾰκᾰ~εσειε|~ᾰκᾰ~εσειεν|~ᾰκᾰ~εσαι", _, "~ᾰκᾰ~εσειᾰν|~ᾰκᾰ~εσαιεν";
    aorist_opt_mid_1: "~ᾰκᾰ~εσαιμην", _, _;
    aorist_imp_act_2: "~ᾰκᾰ~εσον", _, "~ᾰκᾰ~εσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰκᾰ~εσᾰντων";
    aorist_imp_mid_2: "~ᾰκᾰ~εσαι", "~ᾰκᾰ~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰκᾰ~εσᾰσθων";
    perfect_ind_act_1: "~ᾰκεκ~ηκᾰ", _, "~ᾰκεκ~ηκᾰμεν";
    perfect_ind_act_2: _, "~ᾰκεκ~ηκᾰτον", _;
    perfect_ind_act_3: "~ᾰκεκ~ηκε|~ᾰκεκ~ηκεν", _, "~ᾰκεκ~ηκᾱσῐ|~ᾰκεκ~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ᾰκεκ~ημεθᾰ";
    perfect_ind_mid_2: "~ᾰκεκ~ησαι", "~ᾰκεκ~ησθον", _;
    perfect_ind_mid_3: "~ᾰκεκ~ηται", _, "~ᾰκεκ~ηνται";
    perfect_ind_pass_1: "~ᾰκεκ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ᾰκεκ~ησθε";
    perfect_subj_act_1: "~ᾰκεκ~ηκω", _, _;
    perfect_subj_act_2: "~ᾰκεκ~ηκῃς", "~ᾰκεκ~ηκητον", "~ᾰκεκ~ηκητε";
    perfect_subj_act_3: "~ᾰκεκ~ηκῃ", _, "~ᾰκεκ~ηκωσῐ|~ᾰκεκ~ηκωσῐν";
    perfect_opt_act_1: "~ᾰκεκ~ηκοιμῐ|~ᾰκεκ~ηκοιην", _, _;
    perfect_opt_act_2: "~ᾰκεκ~ηκοις|~ᾰκεκ~ηκοιης", _, _;
    perfect_opt_act_3: "~ᾰκεκ~ηκοι|~ᾰκεκ~ηκοιη", _, _;
    perfect_imp_act_2: "~ᾰκεκ~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~ᾰκεκ~ησθε";
    perfect_imp_mid_3: _, _, "~ᾰκεκ~ησθων";
    perfect_imp_pass_2: "~ᾰκεκ~ησο", _, "~ᾰκεκ~ησθε";
    perfect_imp_pass_3: "~ᾰκεκ~ησθω", "~ᾰκεκ~ησθων", "~ᾰκεκ~ησθων";
    pluperfect_mid_1: "~εκεκ~ημην", _, _;
    pluperfect_mid_2: "~εκεκ~ησο", "~εκεκ~ησθον", _;
    pluperfect_mid_3: "~εκεκ~ητο", _, "~εκεκ~ηντο";
    pluperfect_pass_1: _, _, "~εκεκ~ημεθᾰ";
    pluperfect_pass_2: _, _, "~εκεκ~ησθε";
    pluperfect_pass_3: _, "~εκεκ~ησθην", _;
    infinitive_pres: "~ᾰκᾰ~ειν", _, "~ᾰκᾰ~εισθαι";
    infinitive_fut: "~ᾰκᾰ~ειν", _, _;
    infinitive_aor: "~ᾰκᾰ~εσαι", _, _;
    infinitive_perf: _, "~ᾰκεκ~ησθαι", _;
    participle_pres: "~ᾰκᾰ~ων", _, "~ᾰκᾰ~ουμενος";
    participle_aor: "~ᾰκᾰ~εσᾱς", "~ᾰκᾰ~εσᾰμενος", _;
    participle_perf: "~ᾰκεκ~ηκως", _, "~ᾰκεκ~ημενος";
    ),
    // Ἐξαιρω: 3 words (κᾰταιρω, ἀνταιρω, ἐξαιρω). Like Δῐαιρω, except in 22 cells: aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, aorist_subj_mid_1 sg, aorist_subj_mid_2 sg, ….
    conjugation!("ἐξαιρω", 3, "~αι~ω", like Δῐαιρω;
    aorist_subj_act_1: "~α~ω", _, _;
    aorist_subj_act_2: "~α~ῃς", "~α~ητον", _;
    aorist_subj_act_3: _, _, "~α~ωσῐ|~α~ωσῐν";
    aorist_subj_mid_1: "~α~ωμαι", _, _;
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
    // Δῐᾰλῡω: 3 words (ἀντᾰνᾰλῡω, δῐᾰλῡω, κᾰτᾰλῡω). Like Πεμπω, except in 172 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("δῐᾰλῡω", 3, "~ᾰ~ῡω", like Πεμπω;
    stem future_pass: "~ᾰ~ῠθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ε~ῡσ- like εὐλογεω";
    stem aorist_ind_pass: "~ε~ῠθην- like εὐλογεω";
    stem aorist_pass: "~ᾰ~ῠθ- like ποιεω";
    stem pluperfect_act: "~ε~ελῠκ- like εὐλογεω";
    ind_act_1: "~ᾰ~ῡω", _, "~ᾰ~ῡομεν";
    ind_act_2: "~ᾰ~ῡεις", "~ᾰ~ῡετον", _;
    ind_act_3: "~ᾰ~ῡει", _, "~ᾰ~ῡουσῐ|~ᾰ~ῡουσῐν";
    ind_mid_1: _, _, "~ᾰ~ῡομεθᾰ";
    ind_mid_2: "~ᾰ~ῡῃ|~ᾰ~ῡει", _, _;
    ind_mid_3: _, "~ᾰ~ῡεσθον", _;
    ind_pass_1: "~ᾰ~ῡομαι", _, _;
    ind_pass_2: _, "~ᾰ~ῡεσθον", "~ᾰ~ῡεσθε";
    ind_pass_3: "~ᾰ~ῡεται", "~ᾰ~ῡεσθον", "~ᾰ~ῡονται";
    subj_act_1: "~ᾰ~ῡω", _, "~ᾰ~ῡωμεν";
    subj_act_2: "~ᾰ~ῡῃς", _, _;
    subj_act_3: "~ᾰ~ῡῃ", "~ᾰ~ῡητον", "~ᾰ~ῡωσῐ|~ᾰ~ῡωσῐν";
    subj_mid_1: _, _, "~ᾰ~ῡωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ῡησθον", _;
    subj_mid_3: "~ᾰ~ῡηται", _, "~ᾰ~ῡωνται";
    subj_pass_1: "~ᾰ~ῡωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~ῡησθε";
    opt_act_1: "~ᾰ~ῡοιμῐ", _, "~ᾰ~ῡοιμεν";
    opt_act_2: "~ᾰ~ῡοις", "~ᾰ~ῡοιτον", _;
    opt_act_3: "~ᾰ~ῡοι", "~ᾰ~ῡοιτην", "~ᾰ~ῡοιεν";
    opt_mid_1: "~ᾰ~ῡοιμην", _, "~ᾰ~ῡοιμεθᾰ";
    opt_mid_2: "~ᾰ~ῡοιο", _, "~ᾰ~ῡοισθε";
    opt_mid_3: "~ᾰ~ῡοιτο", "~ᾰ~ῡοισθην", "~ᾰ~ῡοιντο";
    opt_pass_2: _, "~ᾰ~ῡοισθον", _;
    imp_act_2: "~ᾰ~ῡε", _, "~ᾰ~ῡετε";
    imp_act_3: _, _, "~ᾰ~ῡοντων";
    imp_mid_2: "~ᾰ~ῡου", "~ᾰ~ῡεσθον", "~ᾰ~ῡεσθε";
    imp_mid_3: _, _, "~ᾰ~ῡεσθων";
    imp_pass_2: _, "~ᾰ~ῡεσθον", "~ᾰ~ῡεσθε";
    imp_pass_3: "~ᾰ~ῡεσθω", "~ᾰ~ῡεσθων", "~ᾰ~ῡεσθων";
    imperfect_act_1: "~ε~ῡον", _, _;
    imperfect_act_2: "~ε~ῡες", "~ε~ῡετον", "~ε~ῡετε";
    imperfect_act_3: "~ε~ῡε|~ε~ῡεν", _, "~ε~ῡον";
    imperfect_mid_1: "~ε~ῡομην", _, "~ε~ῡομεθᾰ";
    imperfect_mid_2: "~ε~ῡου", _, _;
    imperfect_mid_3: "~ε~ῡετο", _, "~ε~ῡοντο";
    imperfect_pass_2: _, "~ε~ῡεσθον", "~ε~ῡεσθε";
    imperfect_pass_3: _, "~ε~ῡεσθην", _;
    future_ind_act_1: "~ᾰ~ῡσω", _, "~ᾰ~ῡσομεν";
    future_ind_act_2: "~ᾰ~ῡσεις", "~ᾰ~ῡσετον", _;
    future_ind_act_3: _, _, "~ᾰ~ῡσουσῐ|~ᾰ~ῡσουσῐν";
    future_ind_mid_1: "~ᾰ~ῡσομαι", _, _;
    future_ind_mid_2: "~ᾰ~ῡσῃ|~ᾰ~ῡσει", "~ᾰ~ῡσεσθον", _;
    future_ind_mid_3: "~ᾰ~ῡσεται", _, _;
    future_opt_act_1: "~ᾰ~ῡσοιμῐ", _, "~ᾰ~ῡσοιμεν";
    future_opt_act_2: "~ᾰ~ῡσοις", "~ᾰ~ῡσοιτον", _;
    future_opt_act_3: "~ᾰ~ῡσοι", "~ᾰ~ῡσοιτην", _;
    future_opt_mid_1: "~ᾰ~ῡσοιμην", _, "~ᾰ~ῡσοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~ῡσοισθην", _;
    aorist_subj_act_1: "~ᾰ~ῡσω", _, _;
    aorist_subj_act_2: "~ᾰ~ῡσῃς", "~ᾰ~ῡσητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~ῡσωσῐ|~ᾰ~ῡσωσῐν";
    aorist_subj_mid_1: "~ᾰ~ῡσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ῡσῃ", "~ᾰ~ῡσησθον", _;
    aorist_opt_act_1: "~ᾰ~ῡσαιμῐ", _, "~ᾰ~ῡσαιμεν";
    aorist_opt_act_2: "~ᾰ~ῡσειᾰς|~ᾰ~ῡσαις", _, _;
    aorist_opt_act_3: "~ᾰ~ῡσειε|~ᾰ~ῡσειεν|~ᾰ~ῡσαι", _, "~ᾰ~ῡσειᾰν|~ᾰ~ῡσαιεν";
    aorist_opt_mid_1: "~ᾰ~ῡσαιμην", _, _;
    aorist_imp_act_2: "~ᾰ~ῡσον", _, "~ᾰ~ῡσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ῡσᾰντων";
    aorist_imp_mid_2: "~ᾰ~ῡσαι", "~ᾰ~ῡσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ῡσᾰσθων";
    perfect_ind_act_1: "~ᾰ~ελῠκᾰ", _, "~ᾰ~ελῠκᾰμεν";
    perfect_ind_act_2: _, "~ᾰ~ελῠκᾰτον", _;
    perfect_ind_act_3: "~ᾰ~ελῠκε|~ᾰ~ελῠκεν", _, "~ᾰ~ελῠκᾱσῐ|~ᾰ~ελῠκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ᾰ~ελῠμεθᾰ";
    perfect_ind_mid_2: "~ᾰ~ελῠσαι", "~ᾰ~ελῠσθον", _;
    perfect_ind_mid_3: "~ᾰ~ελῠται", _, "~ᾰ~ελῠνται";
    perfect_ind_pass_1: "~ᾰ~ελῠμαι", _, _;
    perfect_ind_pass_2: _, _, "~ᾰ~ελῠσθε";
    perfect_subj_act_1: "~ᾰ~ελῠκω", _, _;
    perfect_subj_act_2: "~ᾰ~ελῠκῃς", "~ᾰ~ελῠκητον", "~ᾰ~ελῠκητε";
    perfect_subj_act_3: "~ᾰ~ελῠκῃ", _, "~ᾰ~ελῠκωσῐ|~ᾰ~ελῠκωσῐν";
    perfect_opt_act_1: "~ᾰ~ελῠκοιμῐ|~ᾰ~ελῠκοιην", _, _;
    perfect_opt_act_2: "~ᾰ~ελῠκοις|~ᾰ~ελῠκοιης", _, _;
    perfect_opt_act_3: "~ᾰ~ελῠκοι|~ᾰ~ελῠκοιη", _, _;
    perfect_imp_act_2: "~ᾰ~ελῠκε", _, _;
    perfect_imp_mid_2: _, _, "~ᾰ~ελῠσθε";
    perfect_imp_mid_3: _, _, "~ᾰ~ελῠσθων";
    perfect_imp_pass_2: "~ᾰ~ελῠσο", _, "~ᾰ~ελῠσθε";
    perfect_imp_pass_3: "~ᾰ~ελῠσθω", "~ᾰ~ελῠσθων", "~ᾰ~ελῠσθων";
    pluperfect_mid_1: "~ε~ελῠμην", _, _;
    pluperfect_mid_2: "~ε~ελῠσο", "~ε~ελῠσθον", _;
    pluperfect_mid_3: "~ε~ελῠτο", _, "~ε~ελῠντο";
    pluperfect_pass_1: _, _, "~ε~ελῠμεθᾰ";
    pluperfect_pass_2: _, _, "~ε~ελῠσθε";
    pluperfect_pass_3: _, "~ε~ελῠσθην", _;
    future_perfect_ind_mid_1: "~ᾰ~ελῡσομαι", _, _;
    future_perfect_ind_mid_2: "~ᾰ~ελῡσῃ|~ᾰ~ελῡσει", _, _;
    future_perfect_ind_mid_3: _, "~ᾰ~ελῡσεσθον", _;
    future_perfect_ind_pass_1: _, _, "~ᾰ~ελῡσομεθᾰ";
    future_perfect_ind_pass_2: _, "~ᾰ~ελῡσεσθον", "~ᾰ~ελῡσεσθε";
    future_perfect_ind_pass_3: "~ᾰ~ελῡσεται", "~ᾰ~ελῡσεσθον", "~ᾰ~ελῡσονται";
    future_perfect_opt_pass_1: "~ᾰ~ελῡσοιμην", _, "~ᾰ~ελῡσοιμεθᾰ";
    future_perfect_opt_pass_2: "~ᾰ~ελῡσοιο", "~ᾰ~ελῡσοισθον", "~ᾰ~ελῡσοισθε";
    future_perfect_opt_pass_3: "~ᾰ~ελῡσοιτο", "~ᾰ~ελῡσοισθην", "~ᾰ~ελῡσοιντο";
    infinitive_pres: "~ᾰ~ῡειν", _, "~ᾰ~ῡεσθαι";
    infinitive_fut: "~ᾰ~ῡσειν", _, _;
    infinitive_aor: "~ᾰ~ῡσαι", _, _;
    infinitive_perf: _, "~ᾰ~ελῠσθαι", _;
    infinitive_futp: _, _, "~ᾰ~ελῡσεσθαι";
    participle_pres: "~ᾰ~ῡων", _, "~ᾰ~ῡομενος";
    participle_aor: "~ᾰ~ῡσᾱς", "~ᾰ~ῡσᾰμενος", _;
    participle_perf: "~ᾰ~ελῠκως", _, "~ᾰ~ελῠμενος";
    participle_futp: _, _, "~ᾰ~ελῡσομενος";
    ),
    // Ἁμᾰρτᾰνω: 3 words (ἁμᾰρτᾰνω, ἀφᾰμᾰρτᾰνω, ἀνᾰβλᾰστᾰνω). Like Ἰαινω, except in 122 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἁμᾰρτᾰνω", 3, "~ᾰ~ᾰνω", like Ἰαινω;
    stem future_pass: "~ᾰ~ηθησ- like ἐχω";
    stem aorist_ind: "~η~- like ἀνᾰγω";
    stem aorist_ind_pass: "~η~ηθην- like εὐλογεω";
    stem aorist_pass: "~ᾰ~ηθ- like ἐχω";
    ind_act_1: "~ᾰ~ᾰνω", _, "~ᾰ~ᾰνομεν";
    ind_act_2: "~ᾰ~ᾰνεις", "~ᾰ~ᾰνετον", _;
    ind_act_3: "~ᾰ~ᾰνει", _, "~ᾰ~ᾰνουσῐ|~ᾰ~ᾰνουσῐν";
    ind_mid_1: _, _, "~ᾰ~ᾰνομεθᾰ";
    ind_mid_2: "~ᾰ~ᾰνῃ", _, _;
    ind_mid_3: _, "~ᾰ~ᾰνεσθον", _;
    ind_pass_1: "~ᾰ~ᾰνομαι", _, _;
    ind_pass_2: _, "~ᾰ~ᾰνεσθον", "~ᾰ~ᾰνεσθε";
    ind_pass_3: "~ᾰ~ᾰνεται", "~ᾰ~ᾰνεσθον", "~ᾰ~ᾰνονται";
    subj_act_1: "~ᾰ~ᾰνω", _, "~ᾰ~ᾰνωμεν";
    subj_act_2: "~ᾰ~ᾰνῃς", _, _;
    subj_act_3: "~ᾰ~ᾰνῃ", "~ᾰ~ᾰνητον", "~ᾰ~ᾰνωσῐ|~ᾰ~ᾰνωσῐν";
    subj_mid_1: _, _, "~ᾰ~ᾰνωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ᾰνησθον", _;
    subj_mid_3: "~ᾰ~ᾰνηται", _, "~ᾰ~ᾰνωνται";
    subj_pass_1: "~ᾰ~ᾰνωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~ᾰνησθε";
    opt_act_1: "~ᾰ~ᾰνοιμῐ", _, "~ᾰ~ᾰνοιμεν";
    opt_act_2: "~ᾰ~ᾰνοις", "~ᾰ~ᾰνοιτον", _;
    opt_act_3: "~ᾰ~ᾰνοι", "~ᾰ~ᾰνοιτην", "~ᾰ~ᾰνοιεν";
    opt_mid_1: "~ᾰ~ᾰνοιμην", _, "~ᾰ~ᾰνοιμεθᾰ";
    opt_mid_2: "~ᾰ~ᾰνοιο", _, "~ᾰ~ᾰνοισθε";
    opt_mid_3: "~ᾰ~ᾰνοιτο", "~ᾰ~ᾰνοισθην", "~ᾰ~ᾰνοιντο";
    opt_pass_2: _, "~ᾰ~ᾰνοισθον", _;
    imp_act_2: "~ᾰ~ᾰνε", _, "~ᾰ~ᾰνετε";
    imp_act_3: _, _, "~ᾰ~ᾰνοντων";
    imp_mid_2: "~ᾰ~ᾰνου", "~ᾰ~ᾰνεσθον", "~ᾰ~ᾰνεσθε";
    imp_mid_3: _, _, "~ᾰ~ᾰνεσθων";
    imp_pass_2: _, "~ᾰ~ᾰνεσθον", "~ᾰ~ᾰνεσθε";
    imp_pass_3: "~ᾰ~ᾰνεσθω", "~ᾰ~ᾰνεσθων", "~ᾰ~ᾰνεσθων";
    imperfect_act_1: "~η~ᾰνον", _, _;
    imperfect_act_2: "~η~ᾰνες", "~η~ᾰνετον", "~η~ᾰνετε";
    imperfect_act_3: "~η~ᾰνε", _, "~η~ᾰνον";
    imperfect_mid_1: "~η~ᾰνομην", _, "~η~ᾰνομεθᾰ";
    imperfect_mid_2: "~η~ᾰνου", _, _;
    imperfect_mid_3: "~η~ᾰνετο", _, "~η~ᾰνοντο";
    imperfect_pass_2: _, "~η~ᾰνεσθον", "~η~ᾰνεσθε";
    imperfect_pass_3: _, "~η~ᾰνεσθην", _;
    future_ind_act_1: "~ᾰ~ησω", _, "~ᾰ~ησομεν";
    future_ind_act_2: "~ᾰ~ησεις", "~ᾰ~ησετον", _;
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ|~ᾰ~ησουσῐν";
    future_ind_mid_1: "~ᾰ~ησομαι", _, _;
    future_ind_mid_2: "~ᾰ~ησῃ", "~ᾰ~ησεσθον", _;
    future_ind_mid_3: "~ᾰ~ησεται", _, _;
    future_opt_act_1: "~ᾰ~ησοιμῐ", _, "~ᾰ~ησοιμεν";
    future_opt_act_2: "~ᾰ~ησοις", "~ᾰ~ησοιτον", _;
    future_opt_act_3: "~ᾰ~ησοι", "~ᾰ~ησοιτην", _;
    future_opt_mid_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~ησοισθην", _;
    aorist_subj_act_1: "~ᾰ~ω|~ᾰ~ησω", _, "~ᾰ~ωμεν|~ᾰ~ησωμεν";
    aorist_subj_act_2: "~ᾰ~ῃς|~ᾰ~ησῃς", "~ᾰ~ητον|~ᾰ~ησητον", "~ᾰ~ητε|~ᾰ~ησητε";
    aorist_subj_act_3: "~ᾰ~ῃ|~ᾰ~ησῃ", _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν|~ᾰ~ησωσῐ|~ᾰ~ησωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ῃ", "~ᾰ~ησθον", _;
    aorist_opt_act_1: "~ᾰ~οιμῐ|~ᾰ~ησαιμῐ", _, "~ᾰ~οιμεν|~ᾰ~ησαιμεν";
    aorist_opt_act_2: "~ᾰ~οις|~ᾰ~ησειᾰς|~ᾰ~ησαις", "~ᾰ~οιτον|~ᾰ~ησαιτον", "~ᾰ~οιτε|~ᾰ~ησαιτε";
    aorist_opt_act_3: "~ᾰ~οι|~ᾰ~ησειε|~ᾰ~ησειεν|~ᾰ~ησαι", "~ᾰ~οιτην|~ᾰ~ησαιτην", "~ᾰ~οιεν|~ᾰ~ησειᾰν|~ᾰ~ησαιεν";
    aorist_opt_mid_1: "~ᾰ~οιμην", _, _;
    aorist_imp_act_2: "~ᾰ~ε|~ᾰ~ησον", "~ᾰ~ετον|~ᾰ~ησᾰτον", "~ᾰ~ετε|~ᾰ~ησᾰτε";
    aorist_imp_act_3: "~ᾰ~ετω|~ᾰ~ησᾰτω", "~ᾰ~ετων|~ᾰ~ησᾰτων", "~ᾰ~οντων|~ᾰ~ησᾰντων";
    aorist_imp_mid_2: "~ᾰ~ου", "~ᾰ~εσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~εσθων";
    infinitive_pres: "~ᾰ~ᾰνειν", _, "~ᾰ~ᾰνεσθαι";
    infinitive_fut: "~ᾰ~ησειν", _, _;
    infinitive_aor: "~ᾰ~ειν|~ᾰ~ησαι", _, _;
    participle_pres: "~ᾰ~ᾰνων", _, "~ᾰ~ᾰνομενος";
    participle_aor: "~ᾰ~ων|~ᾰ~ησᾱς", "~ᾰ~ομενος", _;
    ),
    // Κᾰμπτω: 3 words (κλεπτω, κᾰμπτω, κολᾰπτω). Like Θρῠπτω, except in 13 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 pl, perfect_ind_mid_3 pl, perfect_subj_act_1 sg, ….
    conjugation!("κᾰμπτω", 3, "~πτω", like Θρῠπτω;
    stem future_pass: "~φθησ- like χρημᾰτῐζω";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_3: _, _, "%ε~πᾰται";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_imp_act_2: derived, _, _;
    participle_perf: derived, _, _;
    ),
    // Μεθῐστημῐ: 3 words (ἐγκᾰθῐστημῐ, μεθῐστημῐ, ἀντῐκᾰθῐστημῐ). Like Ἰαινω, except in 130 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("μεθῐστημῐ", 3, "~θῐ~ημῐ", like Ἰαινω;
    stem future_pass: "~τᾰ~ησ- like χρημᾰτῐζω";
    stem aorist_ind: "~τε~- like ἐφῐστημῐ";
    stem aorist_ind_pass: "~τε~ᾰθην- like εὐλογεω";
    stem aorist_pass: "~τᾰ~ᾰθ- like ἐχω";
    ind_act_1: "~θῐ~ημῐ", _, "~θῐ~ᾰμεν";
    ind_act_2: "~θῐ~ης", "~θῐ~ᾰτον", _;
    ind_act_3: "~θῐ~ησῐ|~θῐ~ησῐν", _, "~θῐ~ᾱσῐ|~θῐ~ᾱσῐν";
    ind_mid_1: _, _, "~θῐ~ᾰμεθᾰ";
    ind_mid_2: "~θῐ~ᾰσαι", _, _;
    ind_mid_3: _, "~θῐ~ᾰσθον", _;
    ind_pass_1: "~θῐ~ᾰμαι", _, _;
    ind_pass_2: _, "~θῐ~ᾰσθον", "~θῐ~ᾰσθε";
    ind_pass_3: "~θῐ~ᾰται", "~θῐ~ᾰσθον", "~θῐ~ᾰνται";
    subj_act_1: "~θῐ~ω", _, "~θῐ~ωμεν";
    subj_act_2: "~θῐ~ῃς", _, _;
    subj_act_3: "~θῐ~ῃ", "~θῐ~ητον", "~θῐ~ωσῐ|~θῐ~ωσῐν";
    subj_mid_1: _, _, "~θῐ~ωμεθᾰ";
    subj_mid_2: _, "~θῐ~ησθον", _;
    subj_mid_3: "~θῐ~ηται", _, "~θῐ~ωνται";
    subj_pass_1: "~θῐ~ωμαι", _, _;
    subj_pass_2: _, _, "~θῐ~ησθε";
    opt_act_1: "~θῐ~αιην", _, "~θῐ~αιμεν|~θῐ~αιημεν";
    opt_act_2: "~θῐ~αιης", "~θῐ~αιτον|~θῐ~αιητον", "~θῐ~αιτε|~θῐ~αιητε";
    opt_act_3: "~θῐ~αιη", "~θῐ~αιτην|~θῐ~αιητην", "~θῐ~αιεν|~θῐ~αιησᾰν";
    opt_mid_1: "~θῐ~αιμην", _, "~θῐ~αιμεθᾰ";
    opt_mid_2: "~θῐ~αιο", _, "~θῐ~αισθε";
    opt_mid_3: "~θῐ~αιτο", "~θῐ~αισθην", "~θῐ~αιντο";
    opt_pass_2: _, "~θῐ~αισθον", _;
    imp_act_2: "~θῐ~η", _, "~θῐ~ᾰτε";
    imp_act_3: _, _, "~θῐ~ᾰντων";
    imp_mid_2: "~θῐ~ᾰσο|~θῐ~ω", "~θῐ~ᾰσθον", "~θῐ~ᾰσθε";
    imp_mid_3: _, _, "~θῐ~ᾰσθων";
    imp_pass_2: _, "~θῐ~ᾰσθον", "~θῐ~ᾰσθε";
    imp_pass_3: "~θῐ~ᾰσθω", "~θῐ~ᾰσθων", "~θῐ~ᾰσθων";
    imperfect_act_1: "~θῑ~ην", _, _;
    imperfect_act_2: "~θῑ~ης", "~θῑ~ᾰτον", "~θῑ~ᾰτε";
    imperfect_act_3: "~θῑ~η", _, "~θῑ~ᾰσᾰν";
    imperfect_mid_1: "~θῑ~ᾰμην", _, "~θῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~θῑ~ᾰσο", _, _;
    imperfect_mid_3: "~θῑ~ᾰτο", _, "~θῑ~ᾰντο";
    imperfect_pass_2: _, "~θῑ~ᾰσθον", "~θῑ~ᾰσθε";
    imperfect_pass_3: _, "~θῑ~ᾰσθην", _;
    future_ind_act_1: "~τᾰ~ησω", _, "~τᾰ~ησομεν";
    future_ind_act_2: "~τᾰ~ησεις", "~τᾰ~ησετον", _;
    future_ind_act_3: _, _, "~τᾰ~ησουσῐ|~τᾰ~ησουσῐν";
    future_ind_mid_1: "~τᾰ~ησομαι", _, _;
    future_ind_mid_2: "~τᾰ~ησῃ|~τᾰ~ησει", "~τᾰ~ησεσθον", _;
    future_ind_mid_3: "~τᾰ~ησεται", _, _;
    future_opt_act_1: "~τᾰ~ησοιμῐ", _, "~τᾰ~ησοιμεν";
    future_opt_act_2: "~τᾰ~ησοις", "~τᾰ~ησοιτον", _;
    future_opt_act_3: "~τᾰ~ησοι", "~τᾰ~ησοιτην", _;
    future_opt_mid_1: "~τᾰ~ησοιμην", _, "~τᾰ~ησοιμεθᾰ";
    future_opt_mid_3: _, "~τᾰ~ησοισθην", _;
    aorist_subj_act_1: "~τᾰ~ησω", _, _;
    aorist_subj_act_2: "~τᾰ~ησῃς", "~τᾰ~ησητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~τᾰ~ησωμαι", _, _;
    aorist_subj_mid_2: "~τᾰ~ησῃ", "~τᾰ~ησησθον", _;
    aorist_opt_act_1: "~τᾰ~ησαιμῐ", _, "~τᾰ~ησαιμεν";
    aorist_opt_act_2: "~τᾰ~ησειᾰς", _, _;
    aorist_opt_act_3: "~τᾰ~ησειε", _, "~τᾰ~ησειᾰν";
    aorist_opt_mid_1: "~τᾰ~ησαιμην", _, _;
    aorist_imp_act_2: "~τᾰ~ησον", _, "~τᾰ~ησᾰτε";
    aorist_imp_act_3: _, _, "~τᾰ~ησᾰντων";
    aorist_imp_mid_2: "~τᾰ~ησαι", "~τᾰ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~τᾰ~ησᾰσθων";
    perfect_ind_act_1: "~θε~ηκᾰ", _, "~θε~ηκᾰμεν";
    perfect_ind_act_2: _, "~θε~ηκᾰτον", _;
    perfect_ind_act_3: "~θε~ηκε|~θε~ηκεν", _, "~θε~ηκᾱσῐ|~θε~ηκᾱσῐν";
    perfect_subj_act_1: "~θε~ηκω", _, _;
    perfect_subj_act_2: "~θε~ηκῃς", "~θε~ηκητον", "~θε~ηκητε";
    perfect_subj_act_3: "~θε~ηκῃ", _, "~θε~ηκωσῐ|~θε~ηκωσῐν";
    perfect_opt_act_1: "~θε~ηκοιμῐ|~θε~ηκοιην", _, _;
    perfect_opt_act_2: "~θε~ηκοις|~θε~ηκοιης", _, _;
    perfect_opt_act_3: "~θε~ηκοι|~θε~ηκοιη", _, _;
    perfect_imp_act_2: "~θε~ηκε", _, _;
    infinitive_pres: "~θῐ~ᾰναι", _, "~θῐ~ᾰσθαι";
    infinitive_fut: "~τᾰ~ησειν", _, _;
    infinitive_aor: "~τᾰ~ησαι", _, _;
    participle_pres: "~θῐ~ᾱς", _, "~θῐ~ᾰμενος";
    participle_aor: "~τᾰ~ησᾱς", "~τᾰ~ησᾰμενος", _;
    participle_perf: "~θε~ηκως", _, _;
    ),
    // Ἐκφερω: 3 words (ἐκφερω, ἐκγᾰμῐζω, ὑπεκφευγω). Like Ἰαινω, except in 109 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐκφερω", 3, "~κ~ω", like Ἰαινω;
    ind_act_1: "~κ~ω", _, "~κ~ομεν";
    ind_act_2: "~κ~εις", "~κ~ετον", _;
    ind_act_3: "~κ~ει", _, "~κ~ουσῐ";
    ind_mid_1: _, _, "~κ~ομεθᾰ";
    ind_mid_2: "~κ~ῃ", _, _;
    ind_mid_3: _, "~κ~εσθον", _;
    ind_pass_1: "~κ~ομαι", _, _;
    ind_pass_2: _, "~κ~εσθον", "~κ~εσθε";
    ind_pass_3: "~κ~εται", "~κ~εσθον", "~κ~ονται";
    subj_act_1: "~κ~ω", _, "~κ~ωμεν";
    subj_act_2: "~κ~ῃς", _, _;
    subj_act_3: "~κ~ῃ", "~κ~ητον", "~κ~ωσῐ";
    subj_mid_1: _, _, "~κ~ωμεθᾰ";
    subj_mid_2: _, "~κ~ησθον", _;
    subj_mid_3: "~κ~ηται", _, "~κ~ωνται";
    subj_pass_1: "~κ~ωμαι", _, _;
    subj_pass_2: _, _, "~κ~ησθε";
    opt_act_1: "~κ~οιμῐ", _, "~κ~οιμεν";
    opt_act_2: "~κ~οις", "~κ~οιτον", _;
    opt_act_3: "~κ~οι", "~κ~οιτην", "~κ~οιεν";
    opt_mid_1: "~κ~οιμην", _, "~κ~οιμεθᾰ";
    opt_mid_2: "~κ~οιο", _, "~κ~οισθε";
    opt_mid_3: "~κ~οιτο", "~κ~οισθην", "~κ~οιντο";
    opt_pass_2: _, "~κ~οισθον", _;
    imp_act_2: "~κ~ε", _, "~κ~ετε";
    imp_act_3: _, _, "~κ~οντων";
    imp_mid_2: "~κ~ου", "~κ~εσθον", "~κ~εσθε";
    imp_mid_3: _, _, "~κ~εσθων";
    imp_pass_2: _, "~κ~εσθον", "~κ~εσθε";
    imp_pass_3: "~κ~εσθω", "~κ~εσθων", "~κ~εσθων";
    imperfect_act_1: "~ξε~ον", _, _;
    imperfect_act_2: "~ξε~ες", "~ξε~ετον", "~ξε~ετε";
    imperfect_act_3: "~ξε~ε", _, "~ξε~ον";
    imperfect_mid_1: "~ξε~ομην", _, "~ξε~ομεθᾰ";
    imperfect_mid_2: "~ξε~ου", _, _;
    imperfect_mid_3: "~ξε~ετο", _, "~ξε~οντο";
    imperfect_pass_2: _, "~ξε~εσθον", "~ξε~εσθε";
    imperfect_pass_3: _, "~ξε~εσθην", _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_ind_mid_1: derived, _, _;
    aorist_ind_mid_2: derived, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: derived, _, _;
    aorist_subj_mid_2: derived, derived, _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_opt_mid_1: derived, _, _;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_mid_2: derived, derived, _;
    aorist_imp_mid_3: _, _, derived;
    aorist_imp_pass_2: derived, derived, derived;
    infinitive_pres: "~κ~ειν", _, "~κ~εσθαι";
    infinitive_aor: derived, _, _;
    participle_pres: "~κ~ων", _, "~κ~ομενος";
    participle_aor: derived, derived, _;
    ),
    // Ἐξετᾰζω: 3 words (προετοιμᾰζω, ἐξετᾰζω, ἀντελπῐζω). Like Δῐᾰρπᾰζω, except in 101 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐξετᾰζω", 3, "~ε~ζω", like Δῐᾰρπᾰζω;
    stem future_pass: "~ε~σθησ- like χρημᾰτῐζω";
    stem aorist_pass: "~ε~σθ- like ποιεω";
    ind_act_1: "~ε~ζω", _, "~ε~ζομεν";
    ind_act_2: "~ε~ζεις", "~ε~ζετον", _;
    ind_act_3: "~ε~ζει", _, "~ε~ζουσῐ|~ε~ζουσῐν";
    ind_mid_1: _, _, "~ε~ζομεθᾰ";
    ind_mid_2: "~ε~ζῃ|~ε~ζει", _, _;
    ind_mid_3: _, "~ε~ζεσθον", _;
    ind_pass_1: "~ε~ζομαι", _, _;
    ind_pass_2: _, "~ε~ζεσθον", "~ε~ζεσθε";
    ind_pass_3: "~ε~ζεται", "~ε~ζεσθον", "~ε~ζονται";
    subj_act_1: "~ε~ζω", _, "~ε~ζωμεν";
    subj_act_2: "~ε~ζῃς", _, _;
    subj_act_3: "~ε~ζῃ", "~ε~ζητον", "~ε~ζωσῐ|~ε~ζωσῐν";
    subj_mid_1: _, _, "~ε~ζωμεθᾰ";
    subj_mid_2: _, "~ε~ζησθον", _;
    subj_mid_3: "~ε~ζηται", _, "~ε~ζωνται";
    subj_pass_1: "~ε~ζωμαι", _, _;
    subj_pass_2: _, _, "~ε~ζησθε";
    opt_act_1: "~ε~ζοιμῐ", _, "~ε~ζοιμεν";
    opt_act_2: "~ε~ζοις", "~ε~ζοιτον", _;
    opt_act_3: "~ε~ζοι", "~ε~ζοιτην", "~ε~ζοιεν";
    opt_mid_1: "~ε~ζοιμην", _, "~ε~ζοιμεθᾰ";
    opt_mid_2: "~ε~ζοιο", _, "~ε~ζοισθε";
    opt_mid_3: "~ε~ζοιτο", "~ε~ζοισθην", "~ε~ζοιντο";
    opt_pass_2: _, "~ε~ζοισθον", _;
    imp_act_2: "~ε~ζε", _, "~ε~ζετε";
    imp_act_3: _, _, "~ε~ζοντων";
    imp_mid_2: "~ε~ζου", "~ε~ζεσθον", "~ε~ζεσθε";
    imp_mid_3: _, _, "~ε~ζεσθων";
    imp_pass_2: _, "~ε~ζεσθον", "~ε~ζεσθε";
    imp_pass_3: "~ε~ζεσθω", "~ε~ζεσθων", "~ε~ζεσθων";
    future_ind_act_1: "~ε~ω", _, "~ε~ουμεν";
    future_ind_act_2: "~ε~εις", "~ε~ειτον", _;
    future_ind_act_3: _, _, "~ε~ουσῐ";
    future_ind_mid_1: "~ε~ουμαι", _, _;
    future_ind_mid_2: "~ε~ει", "~ε~εισθον", _;
    future_ind_mid_3: "~ε~ειται", _, _;
    future_opt_act_1: "~ε~οιην", _, "~ε~οιμεν";
    future_opt_act_2: "~ε~οιης", "~ε~οιτον", derived;
    future_opt_act_3: "~ε~οιη", "~ε~οιτην", derived;
    future_opt_mid_1: "~ε~οιμην", _, "~ε~οιμεθᾰ";
    future_opt_mid_3: _, "~ε~οισθην", _;
    aorist_subj_act_1: "~ε~σω", _, _;
    aorist_subj_act_2: "~ε~σῃς", "~ε~σητον", _;
    aorist_subj_act_3: _, _, "~ε~σωσῐ|~ε~σωσῐν";
    aorist_subj_mid_1: "~ε~σωμαι", _, _;
    aorist_subj_mid_2: "~ε~σῃ", "~ε~σησθον", _;
    aorist_opt_act_1: "~ε~σαιμῐ", _, "~ε~σαιμεν";
    aorist_opt_act_2: "~ε~σειᾰς|~ε~σαις", _, _;
    aorist_opt_act_3: "~ε~σειε|~ε~σειεν|~ε~σαι", _, "~ε~σειᾰν|~ε~σαιεν";
    aorist_opt_mid_1: "~ε~σαιμην", _, _;
    aorist_imp_act_2: "~ε~σον", _, "~ε~σᾰτε";
    aorist_imp_act_3: _, _, "~ε~σᾰντων";
    aorist_imp_mid_2: "~ε~σαι", "~ε~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ε~σᾰσθων";
    infinitive_pres: "~ε~ζειν", _, "~ε~ζεσθαι";
    infinitive_fut: "~ε~ειν", _, _;
    infinitive_aor: "~ε~σαι", _, _;
    participle_pres: "~ε~ζων", _, "~ε~ζομενος";
    participle_aor: "~ε~σᾱς", "~ε~σᾰμενος", _;
    ),
    // Σῠναιρεω: 3 words (κᾰθαιρεω, σῠναιρεω, ξῠναιρεω). Like Ἀναιρεω, except in 61 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_3 pl, future_ind_mid_1 sg, ….
    conjugation!("σῠναιρεω", 3, "~αιρεω", like Ἀναιρεω;
    stem aorist_ind: "~ειλ- like πᾰρεχω";
    future_ind_act_1: "~ελω", _, "~ελουμεν";
    future_ind_act_2: "~ελεις", "~ελειτον", _;
    future_ind_act_3: _, _, "~ελουσῐ|~ελουσῐν";
    future_ind_mid_1: "~ελουμαι", _, _;
    future_ind_mid_2: "~ελει|~ελῃ", "~ελεισθον", _;
    future_ind_mid_3: "~ελειται", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_2: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: "~ελοιην|~ελοιμῐ", _, "~ελοιμεν|~ελοιημεν";
    future_opt_act_2: "~ελοιης|~ελοις", "~ελοιτον|~ελοιητον", "~ελοιτε|~ελοιητε";
    future_opt_act_3: "~ελοιη|~ελοι", "~ελοιτην|~ελοιητην", "~ελοιεν|~ελοιησᾰν";
    future_opt_mid_1: "~ελοιμην", _, "~ελοιμεθᾰ";
    future_opt_mid_3: _, "~ελοισθην", _;
    aorist_ind_act_2: _, _, derived;
    aorist_ind_act_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ελω", _, derived;
    aorist_subj_act_2: "~ελῃς", "~ελητον", derived;
    aorist_subj_act_3: derived, _, "~ελωσῐ|~ελωσῐν";
    aorist_subj_pass_3: derived, derived, derived;
    aorist_opt_act_1: "~ελοιμῐ", _, "~ελοιμεν";
    aorist_opt_act_2: "~ελοις", derived, derived;
    aorist_opt_act_3: "~ελοι", derived, "~ελοιεν";
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: "~ελε", derived, "~ελετε";
    aorist_imp_act_3: derived, derived, "~ελοντων";
    aorist_imp_pass_2: derived, derived, derived;
    infinitive_fut: "~ελειν", _, _;
    infinitive_aor: "~ελειν", _, _;
    participle_aor: "~ελων", _, _;
    ),
    // Φῡρω: 3 words (σῡρω, τῡφω, φῡρω). Like Ποιμαινω, except in 128 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("φῡρω", 3, "~ῡ~ω", like Ποιμαινω;
    stem aorist_ind: "~ῠ~σ- like ἐχω";
    stem aorist_ind_pass: "~ῠ~θην- like ἐχω";
    stem aorist_pass: "~ῠ~θ- like ἐχω";
    ind_act_1: "~ῡ~ω", _, "~ῡ~ομεν";
    ind_act_2: "~ῡ~εις", "~ῡ~ετον", _;
    ind_act_3: "~ῡ~ει", _, "~ῡ~ουσῐ|~ῡ~ουσῐν";
    ind_mid_1: _, _, "~ῡ~ομεθᾰ";
    ind_mid_2: "~ῡ~ῃ|~ῡ~ει", _, _;
    ind_mid_3: _, "~ῡ~εσθον", _;
    ind_pass_1: "~ῡ~ομαι", _, _;
    ind_pass_2: _, "~ῡ~εσθον", "~ῡ~εσθε";
    ind_pass_3: "~ῡ~εται", "~ῡ~εσθον", "~ῡ~ονται";
    subj_act_1: "~ῡ~ω", _, "~ῡ~ωμεν";
    subj_act_2: "~ῡ~ῃς", _, _;
    subj_act_3: "~ῡ~ῃ", "~ῡ~ητον", "~ῡ~ωσῐ|~ῡ~ωσῐν";
    subj_mid_1: _, _, "~ῡ~ωμεθᾰ";
    subj_mid_2: _, "~ῡ~ησθον", _;
    subj_mid_3: "~ῡ~ηται", _, "~ῡ~ωνται";
    subj_pass_1: "~ῡ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῡ~ησθε";
    opt_act_1: "~ῡ~οιμῐ", _, "~ῡ~οιμεν";
    opt_act_2: "~ῡ~οις", "~ῡ~οιτον", _;
    opt_act_3: "~ῡ~οι", "~ῡ~οιτην", "~ῡ~οιεν";
    opt_mid_1: "~ῡ~οιμην", _, "~ῡ~οιμεθᾰ";
    opt_mid_2: "~ῡ~οιο", _, "~ῡ~οισθε";
    opt_mid_3: "~ῡ~οιτο", "~ῡ~οισθην", "~ῡ~οιντο";
    opt_pass_2: _, "~ῡ~οισθον", _;
    imp_act_2: "~ῡ~ε", _, "~ῡ~ετε";
    imp_act_3: _, _, "~ῡ~οντων";
    imp_mid_2: "~ῡ~ου", "~ῡ~εσθον", "~ῡ~εσθε";
    imp_mid_3: _, _, "~ῡ~εσθων";
    imp_pass_2: _, "~ῡ~εσθον", "~ῡ~εσθε";
    imp_pass_3: "~ῡ~εσθω", "~ῡ~εσθων", "~ῡ~εσθων";
    imperfect_act_1: "ἐ~ῡ~ον", _, _;
    imperfect_act_2: "ἐ~ῡ~ες", "ἐ~ῡ~ετον", "ἐ~ῡ~ετε";
    imperfect_act_3: "ἐ~ῡ~ε|ἐ~ῡ~εν", _, "ἐ~ῡ~ον";
    imperfect_mid_1: "ἐ~ῡ~ομην", _, "ἐ~ῡ~ομεθᾰ";
    imperfect_mid_2: "ἐ~ῡ~ου", _, _;
    imperfect_mid_3: "ἐ~ῡ~ετο", _, "ἐ~ῡ~οντο";
    imperfect_pass_2: _, "ἐ~ῡ~εσθον", "ἐ~ῡ~εσθε";
    imperfect_pass_3: _, "ἐ~ῡ~εσθην", _;
    future_ind_act_1: "~ῠ~σω", _, "~ῠ~σομεν";
    future_ind_act_2: "~ῠ~σεις", "~ῠ~σετον", _;
    future_ind_act_3: _, _, "~ῠ~σουσῐ";
    future_ind_mid_1: "~ῠ~σομαι", _, _;
    future_ind_mid_2: "~ῠ~σῃ", "~ῠ~σεσθον", _;
    future_ind_mid_3: "~ῠ~σεται", _, _;
    future_opt_act_1: "~ῠ~σοιμῐ", _, "~ῠ~σοιμεν";
    future_opt_act_2: "~ῠ~σοις", "~ῠ~σοιτον", _;
    future_opt_act_3: "~ῠ~σοι", "~ῠ~σοιτην", _;
    future_opt_mid_1: "~ῠ~σοιμην", _, "~ῠ~σοιμεθᾰ";
    future_opt_mid_3: _, "~ῠ~σοισθην", _;
    aorist_subj_act_1: "~ῠ~σω", _, _;
    aorist_subj_act_2: "~ῠ~σῃς", "~ῠ~σητον", _;
    aorist_subj_mid_1: "~ῠ~σωμαι", _, _;
    aorist_subj_mid_2: "~ῠ~σῃ", "~ῠ~σησθον", _;
    aorist_opt_act_1: "~ῠ~σαιμῐ", _, "~ῠ~σαιμεν";
    aorist_opt_act_2: "~ῠ~σειᾰς", _, _;
    aorist_opt_act_3: "~ῠ~σειε", _, "~ῠ~σειᾰν";
    aorist_opt_mid_1: "~ῠ~σαιμην", _, _;
    aorist_imp_act_2: "~ῠ~σον", _, "~ῠ~σᾰτε";
    aorist_imp_act_3: _, _, "~ῠ~σᾰντων";
    aorist_imp_mid_2: "~ῠ~σαι", "~ῠ~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῠ~σᾰσθων";
    perfect_ind_mid_1: _, _, "%ε~ῠ~μεθᾰ";
    perfect_ind_mid_2: "%ε~ῠ~σαι", "%ε~ῠ~θον", _;
    perfect_ind_mid_3: "%ε~ῠ~ται", _, "%ε~ῠ~ᾰται";
    perfect_ind_pass_1: "%ε~ῠ~μαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ῠ~θε";
    perfect_imp_mid_2: _, _, "%ε~ῠ~θε";
    perfect_imp_mid_3: _, _, "%ε~ῠ~θων";
    perfect_imp_pass_2: "%ε~ῠ~σο", _, "%ε~ῠ~θε";
    perfect_imp_pass_3: "%ε~ῠ~θω", "%ε~ῠ~θων", "%ε~ῠ~θων";
    infinitive_pres: "~ῡ~ειν", _, "~ῡ~εσθαι";
    infinitive_fut: "~ῠ~σειν", _, _;
    infinitive_aor: "~ῠ~σαι", _, _;
    infinitive_perf: _, "%ε~ῠ~θαι", _;
    participle_pres: "~ῡ~ων", _, "~ῡ~ομενος";
    participle_aor: "~ῠ~σᾱς", "~ῠ~σᾰμενος", _;
    participle_perf: _, _, "%ε~ῠ~μενος";
    ),
    // Σταθμαω: 3 words (χαλαω, καθοραω, σταθμαω). Like Κατοικεω, except in 102 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ind_mid_1 pl, ….
    conjugation!("σταθμαω", 3, "~α~αω", like Κατοικεω;
    stem future_pass: "~ᾰ~ησθησ- like ἐχω";
    stem aorist_ind: "~ᾰ~ησ- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~ησθην- like ἐχω";
    stem aorist_pass: "~ᾰ~ησθ- like ἐχω";
    ind_act_1: _, _, "~ᾰ~ωμεν";
    ind_act_2: "~ᾰ~ᾳς", "~ᾰ~ᾱτον", _;
    ind_act_3: "~ᾰ~ᾳ", _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    ind_mid_1: _, _, "~ᾰ~ωμεθᾰ";
    ind_mid_2: "~ᾰ~ᾳ", _, _;
    ind_mid_3: _, "~ᾰ~ᾱσθον", _;
    ind_pass_1: "~ᾰ~ωμαι", _, _;
    ind_pass_2: _, "~ᾰ~ᾱσθον", "~ᾰ~ᾱσθε";
    ind_pass_3: "~ᾰ~ᾱται", "~ᾰ~ᾱσθον", "~ᾰ~ωνται";
    subj_act_2: "~ᾰ~ᾳς", _, _;
    subj_act_3: "~ᾰ~ᾳ", "~ᾰ~ᾱτον", _;
    subj_mid_2: _, "~ᾰ~ᾱσθον", _;
    subj_mid_3: "~ᾰ~ᾱται", _, _;
    subj_pass_2: _, _, "~ᾰ~ᾱσθε";
    opt_act_1: "~ᾰ~ῳην|~ᾰ~ῳμῐ", _, "~ᾰ~ῳμεν|~ᾰ~ῳημεν";
    opt_act_2: "~ᾰ~ῳης|~ᾰ~ῳς", "~ᾰ~ῳτον|~ᾰ~ῳητον", "~ᾰ~ῳτε|~ᾰ~ῳητε";
    opt_act_3: "~ᾰ~ῳη|~ᾰ~ῳ", "~ᾰ~ῳτην|~ᾰ~ῳητην", "~ᾰ~ῳεν|~ᾰ~ῳησᾰν";
    opt_mid_1: "~ᾰ~ῳμην", _, "~ᾰ~ῳμεθᾰ";
    opt_mid_2: "~ᾰ~ῳο", _, "~ᾰ~ῳσθε";
    opt_mid_3: "~ᾰ~ῳτο", "~ᾰ~ῳσθην", "~ᾰ~ῳντο";
    opt_pass_2: _, "~ᾰ~ῳσθον", _;
    imp_act_2: "~ᾰ~ᾱ", _, "~ᾰ~ᾱτε";
    imp_act_3: _, _, "~ᾰ~ωντων";
    imp_mid_2: "~ᾰ~ω", "~ᾰ~ᾱσθον", "~ᾰ~ᾱσθε";
    imp_mid_3: _, _, "~ᾰ~ᾱσθων";
    imp_pass_2: _, "~ᾰ~ᾱσθον", "~ᾰ~ᾱσθε";
    imp_pass_3: "~ᾰ~ᾱσθω", "~ᾰ~ᾱσθων", "~ᾰ~ᾱσθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, _, _;
    imperfect_mid_3: derived, _, derived;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, _;
    perfect_ind_act_1: "ἐ~ᾰ~ηκᾰ", _, "ἐ~ᾰ~ηκᾰμεν";
    perfect_ind_act_2: _, "ἐ~ᾰ~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~ᾰ~ηκᾱσῐ";
    perfect_ind_mid_1: _, _, "ἐ~ᾰ~ημεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰ~ησαι", "ἐ~ᾰ~ησθον", _;
    perfect_ind_mid_3: "ἐ~ᾰ~ηται", _, "ἐ~ᾰ~ηνται";
    perfect_ind_pass_1: "ἐ~ᾰ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~ᾰ~ησθε";
    perfect_subj_act_1: "ἐ~ᾰ~ηκω", _, _;
    perfect_subj_act_2: "ἐ~ᾰ~ηκῃς", "ἐ~ᾰ~ηκητον", "ἐ~ᾰ~ηκητε";
    perfect_subj_act_3: "ἐ~ᾰ~ηκῃ", _, "ἐ~ᾰ~ηκωσῐ";
    perfect_imp_act_2: "ἐ~ᾰ~ηκε", _, _;
    perfect_imp_mid_2: _, _, "ἐ~ᾰ~ησθε";
    perfect_imp_mid_3: _, _, "ἐ~ᾰ~ησθων";
    perfect_imp_pass_2: "ἐ~ᾰ~ησο", _, "ἐ~ᾰ~ησθε";
    perfect_imp_pass_3: "ἐ~ᾰ~ησθω", "ἐ~ᾰ~ησθων", "ἐ~ᾰ~ησθων";
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_pres: "~ᾰ~ᾱν", _, "~ᾰ~ᾱσθαι";
    infinitive_perf: _, "ἐ~ᾰ~ησθαι", _;
    participle_pres: _, _, "~ᾰ~ωμενος";
    participle_perf: "ἐ~ᾰ~ηκως", _, "ἐ~ᾰ~ημενος";
    ),
    // Κνᾰω: 3 words (βοᾰω, τρῠγᾰω, κνᾰω). Like Χραω, except in 84 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("κνᾰω", 3, "~ᾰω", like Χραω;
    stem future_pass: "~ησθησ- like χρημᾰτῐζω";
    stem aorist_pass: "~ησθ- like ποιεω";
    stem pluperfect_act: "~ηκ- like χρημᾰτῐζω";
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
    opt_pass_2: _, derived, _;
    imp_act_2: "~ᾰε|~ᾱ", _, derived;
    imp_act_3: "~ᾰετω|~ᾱτω", "~ᾰετων|~ᾱτων", "~ᾰοντων|~ωντων";
    imp_mid_2: "~ᾰου|~ω", derived, derived;
    imp_mid_3: "~ᾰεσθω|~ᾱσθω", "~ᾰεσθων|~ᾱσθων", derived;
    imp_pass_2: _, derived, derived;
    imp_pass_3: derived, derived, derived;
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    future_ind_mid_2: "~ησῃ|~ησει", _, _;
    perfect_ind_act_3: "%ε~ηκε|%ε~ηκεν", _, "%ε~ηκᾱσῐ|%ε~ηκᾱσῐν";
    perfect_subj_act_3: _, _, "%ε~ηκωσῐ|%ε~ηκωσῐν";
    perfect_opt_act_1: "%ε~ηκοιμῐ|%ε~ηκοιην", _, _;
    perfect_opt_act_2: "%ε~ηκοις|%ε~ηκοιης", _, _;
    perfect_opt_act_3: "%ε~ηκοι|%ε~ηκοιη", _, _;
    infinitive_pres: "~ᾰειν|~ᾱν", "~ᾰεσθαι|~ᾱσθαι", derived;
    participle_pres: "~ᾰων|~ων", "~ᾰομενος|~ωμενος", derived;
    ),
    // Δηλεομαι: 3 words (δηλεομαι, μῑμεομαι, δῐηγεομαι). Like Ἡγεομαι, except in 35 cells: ind_mid_2 sg, ind_pass_2 sg, imperfect_mid_1 sg, imperfect_mid_1 pl, imperfect_mid_2 sg, imperfect_mid_2 du, ….
    conjugation!("δηλεομαι", 3, "~εομαι", like Ἡγεομαι;
    stem aorist_ind: "~ησ- like ἑπομαι";
    stem aorist_ind_pass: "~ηθην- like ἐχω";
    ind_mid_2: "~ει|~ῃ", _, _;
    ind_pass_2: "~ει", _, _;
    imperfect_mid_1: "ἐ~ουμην", _, "ἐ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ου", "ἐ~εισθον", _;
    imperfect_mid_3: "ἐ~ειτο", _, "ἐ~ουντο";
    imperfect_pass_2: _, _, "ἐ~εισθε";
    imperfect_pass_3: _, "ἐ~εισθην", _;
    future_ind_mid_2: "~ησῃ|~ησει", _, _;
    perfect_ind_mid_1: _, _, "%ε~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ησαι", "%ε~ησθον", _;
    perfect_ind_mid_3: "%ε~ηται", _, "%ε~ηνται";
    perfect_ind_pass_1: "%ε~ημαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ησθε";
    perfect_imp_mid_2: _, _, "%ε~ησθε";
    perfect_imp_mid_3: _, _, "%ε~ησθων";
    perfect_imp_pass_2: "%ε~ησο", _, "%ε~ησθε";
    perfect_imp_pass_3: "%ε~ησθω", "%ε~ησθων", "%ε~ησθων";
    pluperfect_mid_1: "ἐ%ε~ημην", _, _;
    pluperfect_mid_2: "ἐ%ε~ησο", "ἐ%ε~ησθον", _;
    pluperfect_mid_3: "ἐ%ε~ητο", _, "ἐ%ε~ηντο";
    pluperfect_pass_1: _, _, "ἐ%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~ησθην", _;
    infinitive_perf: _, "%ε~ησθαι", _;
    participle_perf: _, _, "%ε~ημενος";
    ),
    // Λευκαινω: 3 words (λευκαινω, λῑμαινω, ξηραινω). Like Φαινω, except in 53 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, ….
    conjugation!("λευκαινω", 3, "~αινω", like Φαινω;
    stem future_pass: "~ανθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~αν- like ἐχω";
    stem aorist_ind_pass: "~ανθην- like ἐχω";
    stem aorist_pass: "~ανθ- like ποιεω";
    ind_act_3: _, _, "~αινουσῐ|~αινουσῐν";
    ind_mid_2: "~αινῃ|~αινει", _, _;
    subj_act_3: _, _, "~αινωσῐ|~αινωσῐν";
    future_ind_act_1: "~ανω", _, "~ανουμεν";
    future_ind_act_2: "~ανεις", "~ανειτον", _;
    future_ind_act_3: _, _, "~ανουσῐ|~ανουσῐν";
    future_ind_mid_1: "~ανουμαι", _, _;
    future_ind_mid_2: "~ανει|~ανῃ", "~ανεισθον", _;
    future_ind_mid_3: "~ανειται", _, _;
    future_opt_act_1: "~ανοιην|~ανοιμῐ", _, "~ανοιμεν|~ανοιημεν";
    future_opt_act_2: "~ανοιης|~ανοις", "~ανοιτον|~ανοιητον", "~ανοιτε|~ανοιητε";
    future_opt_act_3: "~ανοιη|~ανοι", "~ανοιτην|~ανοιητην", "~ανοιεν|~ανοιησᾰν";
    future_opt_mid_1: "~ανοιμην", _, "~ανοιμεθᾰ";
    future_opt_mid_3: _, "~ανοισθην", _;
    aorist_subj_act_1: "~ανω", _, _;
    aorist_subj_act_2: "~ανῃς", "~ανητον", _;
    aorist_subj_mid_1: "~ανωμαι", _, _;
    aorist_subj_mid_2: "~ανῃ", "~ανησθον", _;
    aorist_opt_act_1: "~αναιμῐ", _, "~αναιμεν";
    aorist_opt_act_2: "~ανειᾰς", _, _;
    aorist_opt_act_3: "~ανειε", _, "~ανειᾰν";
    aorist_opt_mid_1: "~αναιμην", _, _;
    aorist_imp_act_2: "~ανον", _, "~ανᾰτε";
    aorist_imp_act_3: _, _, "~ανᾰντων";
    aorist_imp_mid_2: "~αναι", "~ανᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ανᾰσθων";
    pluperfect_mid_1: "ἐ~ασμην", _, _;
    pluperfect_mid_2: "ἐ~ανσο", "ἐ~ανθον", _;
    pluperfect_mid_3: "ἐ~αντο", _, "ἐ~ανᾰτο";
    pluperfect_pass_1: _, _, "ἐ~ασμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ανθε";
    pluperfect_pass_3: _, "ἐ~ανθην", _;
    infinitive_fut: "~ανειν", _, _;
    infinitive_aor: "~ᾱναι", _, _;
    participle_aor: "~ανᾱς", "~ανᾰμενος", _;
    ),
    // Σῠνδεω: 3 words (σῠνδεω, σῠμμᾰχεω, τρωγλοδῠτεω). Like Πεμπω, except in 148 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("σῠνδεω", 3, "~ῠ~εω", like Πεμπω;
    stem future_pass: "~υ~ηθησ- like ἐχω";
    stem aorist_ind: "~υ~ησ- like ἐχω";
    stem aorist_ind_pass: "~υ~ηθην- like ἐχω";
    stem aorist_pass: "~υ~ηθ- like ἐχω";
    stem pluperfect_act: "~υ~ηκειν- like ἐχω";
    ind_act_1: "~υ~ω", _, "~υ~ουμεν";
    ind_act_2: "~υ~εις", "~υ~ειτον", _;
    ind_act_3: "~υ~ει", _, "~υ~ουσῐ";
    ind_mid_1: _, _, "~υ~ουμεθᾰ";
    ind_mid_2: "~υ~ει", _, _;
    ind_mid_3: _, "~υ~εισθον", _;
    ind_pass_1: "~υ~ουμαι", _, _;
    ind_pass_2: _, "~υ~εισθον", "~υ~εισθε";
    ind_pass_3: "~υ~ειται", "~υ~εισθον", "~υ~ουνται";
    subj_act_1: "~υ~ω", _, "~υ~ωμεν";
    subj_act_2: "~υ~ῃς", _, _;
    subj_act_3: "~υ~ῃ", "~υ~ητον", "~υ~ωσῐ";
    subj_mid_1: _, _, "~υ~ωμεθᾰ";
    subj_mid_2: _, "~υ~ησθον", _;
    subj_mid_3: "~υ~ηται", _, "~υ~ωνται";
    subj_pass_1: "~υ~ωμαι", _, _;
    subj_pass_2: _, _, "~υ~ησθε";
    opt_act_1: "~υ~οιην", _, "~υ~οιμεν";
    opt_act_2: "~υ~οιης", "~υ~οιτον", _;
    opt_act_3: "~υ~οιη", "~υ~οιτην", "~υ~οιεν";
    opt_mid_1: "~υ~οιμην", _, "~υ~οιμεθᾰ";
    opt_mid_2: "~υ~οιο", _, "~υ~οισθε";
    opt_mid_3: "~υ~οιτο", "~υ~οισθην", "~υ~οιντο";
    opt_pass_2: _, "~υ~οισθον", _;
    imp_act_2: "~υ~ει", _, "~υ~ειτε";
    imp_act_3: _, _, "~υ~ουντων";
    imp_mid_2: "~υ~ου", "~υ~εισθον", "~υ~εισθε";
    imp_mid_3: _, _, "~υ~εισθων";
    imp_pass_2: _, "~υ~εισθον", "~υ~εισθε";
    imp_pass_3: "~υ~εισθω", "~υ~εισθων", "~υ~εισθων";
    imperfect_act_1: "ἐ~υ~ουν", _, _;
    imperfect_act_2: "ἐ~υ~εις", "ἐ~υ~ειτον", "ἐ~υ~ειτε";
    imperfect_act_3: "ἐ~υ~ει", _, "ἐ~υ~ουν";
    imperfect_mid_1: "ἐ~υ~ουμην", _, "ἐ~υ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~υ~ου", _, _;
    imperfect_mid_3: "ἐ~υ~ειτο", _, "ἐ~υ~ουντο";
    imperfect_pass_2: _, "ἐ~υ~εισθον", "ἐ~υ~εισθε";
    imperfect_pass_3: _, "ἐ~υ~εισθην", _;
    future_ind_act_1: "~υ~ησω", _, "~υ~ησομεν";
    future_ind_act_2: "~υ~ησεις", "~υ~ησετον", _;
    future_ind_act_3: _, _, "~υ~ησουσῐ";
    future_ind_mid_1: "~υ~ησομαι", _, _;
    future_ind_mid_2: "~υ~ησῃ", "~υ~ησεσθον", _;
    future_ind_mid_3: "~υ~ησεται", _, _;
    future_opt_act_1: "~υ~ησοιμῐ", _, "~υ~ησοιμεν";
    future_opt_act_2: "~υ~ησοις", "~υ~ησοιτον", _;
    future_opt_act_3: "~υ~ησοι", "~υ~ησοιτην", _;
    future_opt_mid_1: "~υ~ησοιμην", _, "~υ~ησοιμεθᾰ";
    future_opt_mid_3: _, "~υ~ησοισθην", _;
    aorist_subj_act_1: "~υ~ησω", _, _;
    aorist_subj_act_2: "~υ~ησῃς", "~υ~ησητον", _;
    aorist_subj_mid_1: "~υ~ησωμαι", _, _;
    aorist_subj_mid_2: "~υ~ησῃ", "~υ~ησησθον", _;
    aorist_opt_act_1: "~υ~ησαιμῐ", _, "~υ~ησαιμεν";
    aorist_opt_act_2: "~υ~ησειᾰς", _, _;
    aorist_opt_act_3: "~υ~ησειε", _, "~υ~ησειᾰν";
    aorist_opt_mid_1: "~υ~ησαιμην", _, _;
    aorist_imp_act_2: "~υ~ησον", _, "~υ~ησᾰτε";
    aorist_imp_act_3: _, _, "~υ~ησᾰντων";
    aorist_imp_mid_2: "~υ~ησαι", "~υ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~υ~ησᾰσθων";
    perfect_ind_act_1: "%ε~υ~ηκᾰ", _, "%ε~υ~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~υ~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~υ~ηκᾱσῐ";
    perfect_ind_mid_1: _, _, "%ε~υ~ημεθᾰ";
    perfect_ind_mid_2: "%ε~υ~ησαι", "%ε~υ~ησθον", _;
    perfect_ind_mid_3: "%ε~υ~ηται", _, "%ε~υ~ηνται";
    perfect_ind_pass_1: "%ε~υ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~υ~ησθε";
    perfect_subj_act_1: "%ε~υ~ηκω", _, _;
    perfect_subj_act_2: "%ε~υ~ηκῃς", "%ε~υ~ηκητον", "%ε~υ~ηκητε";
    perfect_subj_act_3: "%ε~υ~ηκῃ", _, "%ε~υ~ηκωσῐ";
    perfect_imp_act_2: "%ε~υ~ηκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~υ~ησθε";
    perfect_imp_mid_3: _, _, "%ε~υ~ησθων";
    perfect_imp_pass_2: "%ε~υ~ησο", _, "%ε~υ~ησθε";
    perfect_imp_pass_3: "%ε~υ~ησθω", "%ε~υ~ησθων", "%ε~υ~ησθων";
    pluperfect_mid_1: "ἐ%ε~υ~ημην", _, _;
    pluperfect_mid_2: "ἐ%ε~υ~ησο", "ἐ%ε~υ~ησθον", _;
    pluperfect_mid_3: "ἐ%ε~υ~ητο", _, "ἐ%ε~υ~ηντο";
    pluperfect_pass_1: _, _, "ἐ%ε~υ~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~υ~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~υ~ησθην", _;
    infinitive_pres: "~υ~ειν", _, "~υ~εισθαι";
    infinitive_fut: "~υ~ησειν", _, _;
    infinitive_aor: "~υ~ησαι", _, _;
    infinitive_perf: _, "%ε~υ~ησθαι", _;
    participle_pres: "~υ~ων", _, "~υ~ουμενος";
    participle_aor: "~υ~ησᾱς", "~υ~ησᾰμενος", _;
    participle_perf: "%ε~υ~ηκως", _, "%ε~υ~ημενος";
    ),
    // Κηδευω: 3 words (σκῡλευω, κηδευω, τυμβευω). Like Μνημονευω, except in 32 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_ind_mid_1 pl, ….
    conjugation!("κηδευω", 3, "~ω", like Μνημονευω;
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: _, "~κᾰτον", _;
    perfect_ind_act_3: "~κε|~κεν", _, "~κᾱσῐ|~κᾱσῐν";
    perfect_ind_mid_1: _, _, "~μεθᾰ";
    perfect_ind_mid_2: "~σαι", "~σθον", _;
    perfect_ind_mid_3: "~ται", _, "~νται";
    perfect_ind_pass_1: "~μαι", _, _;
    perfect_ind_pass_2: _, _, "~σθε";
    perfect_subj_act_1: "~κω", _, _;
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ|~κωσῐν";
    perfect_opt_act_1: "~κοιμῐ|~κοιην", _, _;
    perfect_opt_act_2: "~κοις|~κοιης", _, _;
    perfect_opt_act_3: "~κοι|~κοιη", _, _;
    perfect_imp_act_2: "~κε", _, _;
    perfect_imp_mid_2: _, _, "~σθε";
    perfect_imp_mid_3: _, _, "~σθων";
    perfect_imp_pass_2: "~σο", _, "~σθε";
    perfect_imp_pass_3: "~σθω", "~σθων", "~σθων";
    infinitive_perf: _, "~σθαι", _;
    participle_perf: "~κως", _, "~μενος";
    ),
    // Λεγομαι: 3 words (φθεγγομαι, ἐρευγομαι, λεγομαι). Like Ἐρᾰμαι, except in 64 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("λεγομαι", 3, "~γομαι", like Ἐρᾰμαι;
    stem future_pass: "~χθησ- like ἐχω";
    stem aorist_ind: "~ξ- like ἑπομαι";
    stem aorist_ind_pass: "~γην- like ἐχω";
    stem aorist_pass: "~γ- like ἐγειρω";
    ind_act_1: "~γομαι", _, _;
    ind_mid_1: "~γομαι", _, "~γομεθᾰ";
    ind_mid_2: "~γῃ|~γει", _, _;
    ind_mid_3: _, "~γεσθον", _;
    ind_pass_2: _, "~γεσθον", "~γεσθε";
    ind_pass_3: "~γεται", "~γεσθον", "~γονται";
    subj_mid_1: "~γωμαι", _, "~γωμεθᾰ";
    subj_mid_2: "~γῃ", "~γησθον", _;
    subj_mid_3: "~γηται", _, "~γωνται";
    subj_pass_2: "~γῃ", _, "~γησθε";
    opt_mid_1: "~γοιμην", _, "~γοιμεθᾰ";
    opt_mid_2: "~γοιο", _, "~γοισθε";
    opt_mid_3: "~γοιτο", "~γοισθην", "~γοιντο";
    opt_pass_2: _, "~γοισθον", _;
    imp_mid_2: "~γου", "~γεσθον", "~γεσθε";
    imp_mid_3: "~γεσθω", _, "~γεσθων";
    imp_pass_2: _, "~γεσθον", "~γεσθε";
    imp_pass_3: _, "~γεσθων", "~γεσθων";
    imperfect_mid_1: "ἐ~γομην", _, "ἐ~γομεθᾰ";
    imperfect_mid_2: "ἐ~γου", "ἐ~γεσθον", _;
    imperfect_mid_3: "ἐ~γετο", _, "ἐ~γοντο";
    imperfect_pass_2: _, _, "ἐ~γεσθε";
    imperfect_pass_3: _, "ἐ~γεσθην", _;
    future_ind_mid_1: "~ξομαι", _, _;
    future_ind_mid_2: "~ξῃ", "~ξεσθον", _;
    future_ind_mid_3: "~ξεται", _, _;
    future_opt_mid_1: "~ξοιμην", _, "~ξοιμεθᾰ";
    future_opt_mid_3: _, "~ξοισθην", _;
    perfect_ind_mid_1: _, _, "ἐι~γμεθᾰ";
    perfect_ind_mid_2: "ἐι~ξαι", "ἐι~χθον", _;
    perfect_ind_mid_3: "ἐι~κται", _, "ἐι~γᾰται";
    perfect_imp_mid_2: _, _, "ἐι~χθε";
    perfect_imp_mid_3: _, _, "ἐι~χθων";
    pluperfect_mid_1: "ἐι~γμην", _, _;
    pluperfect_mid_2: "ἐι~ξο", "ἐι~χθον", _;
    pluperfect_mid_3: "ἐι~κτο", _, "ἐι~γᾰτο";
    infinitive_pres: _, _, "~γεσθαι";
    participle_pres: _, _, "~γομενος";
    ),
    // Ῥεω: 3 words (ῥεω, ἐκρεω, ρρεω). Like Αἱρεω, except in 100 cells: ind_mid_2 sg, imperfect_pass_2 du, imperfect_pass_2 pl, imperfect_pass_3 du, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("ῥεω", 3, "~εω", like Αἱρεω;
    stem future_pass: "~ῠησ- like ἐχω";
    stem aorist_ind: "~ευσ- like ἐχω";
    stem aorist_ind_pass: "~ῠην- like ἐχω";
    stem aorist_pass: "~ῠ- like ἐγειρω";
    ind_mid_2: "~εει", _, _;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, _;
    future_ind_act_1: "~ευσω", _, "~ευσομεν";
    future_ind_act_2: "~ευσεις", "~ευσετον", _;
    future_ind_act_3: _, _, "~ευσουσῐ";
    future_ind_mid_1: "~ευσομαι", _, _;
    future_ind_mid_2: "~ευσῃ", "~ευσεσθον", _;
    future_ind_mid_3: "~ευσεται", _, _;
    future_opt_act_1: "~ευσοιμῐ", _, "~ευσοιμεν";
    future_opt_act_2: "~ευσοις", "~ευσοιτον", _;
    future_opt_act_3: "~ευσοι", "~ευσοιτην", _;
    future_opt_mid_1: "~ευσοιμην", _, "~ευσοιμεθᾰ";
    future_opt_mid_3: _, "~ευσοισθην", _;
    aorist_subj_act_1: "~ευσω", _, _;
    aorist_subj_act_2: "~ευσῃς", "~ευσητον", _;
    aorist_subj_mid_1: "~ευσωμαι", _, _;
    aorist_subj_mid_2: "~ευσῃ", "~ευσησθον", _;
    aorist_opt_act_1: "~ευσαιμῐ", _, "~ευσαιμεν";
    aorist_opt_act_2: "~ευσειᾰς", _, _;
    aorist_opt_act_3: "~ευσειε", _, "~ευσειᾰν";
    aorist_opt_mid_1: "~ευσαιμην", _, _;
    aorist_imp_act_2: "~ευσον", _, "~ευσᾰτε";
    aorist_imp_act_3: _, _, "~ευσᾰντων";
    aorist_imp_mid_2: "~ευσαι", "~ευσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ευσᾰσθων";
    perfect_ind_act_1: "ἐ~υηκᾰ", _, "ἐ~υηκᾰμεν";
    perfect_ind_act_2: _, "ἐ~υηκᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~υηκᾱσῐ";
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "ἐ~υηκω", _, _;
    perfect_subj_act_2: "ἐ~υηκῃς", "ἐ~υηκητον", "ἐ~υηκητε";
    perfect_subj_act_3: "ἐ~υηκῃ", _, "ἐ~υηκωσῐ";
    perfect_imp_act_2: "ἐ~υηκε", _, _;
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
    infinitive_fut: "~ευσειν", _, _;
    infinitive_aor: "~ευσαι", _, _;
    infinitive_perf: _, derived, _;
    infinitive_futp: _, _, derived;
    participle_aor: "~ευσᾱς", "~ευσᾰμενος", _;
    participle_perf: "ἐ~υηκως", _, derived;
    participle_futp: _, _, derived;
    ),
    // Δῐᾰσῳζω: 3 words (ξῠσσῳζω, δῐᾰσῳζω, σσῳζω). Like Ληγω, except in 140 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("δῐᾰσῳζω", 3, "~σῳζω", like Ληγω;
    stem future_pass: "~σωθησ- like ἐχω";
    stem aorist_ind: "~ωσ- like ἐχω";
    stem aorist_ind_pass: "~ωθην- like ἐχω";
    stem aorist_pass: "~σωθ- like ἐχω";
    ind_act_1: "~σῳζω", _, "~σῳζομεν";
    ind_act_2: "~σῳζεις", "~σῳζετον", _;
    ind_act_3: "~σῳζει", _, "~σῳζουσῐ";
    ind_mid_1: _, _, "~σῳζομεθᾰ";
    ind_mid_2: "~σῳζῃ", _, _;
    ind_mid_3: _, "~σῳζεσθον", _;
    ind_pass_1: "~σῳζομαι", _, _;
    ind_pass_2: _, "~σῳζεσθον", "~σῳζεσθε";
    ind_pass_3: "~σῳζεται", "~σῳζεσθον", "~σῳζονται";
    subj_act_1: "~σῳζω", _, "~σῳζωμεν";
    subj_act_2: "~σῳζῃς", _, _;
    subj_act_3: "~σῳζῃ", "~σῳζητον", "~σῳζωσῐ";
    subj_mid_1: _, _, "~σῳζωμεθᾰ";
    subj_mid_2: _, "~σῳζησθον", _;
    subj_mid_3: "~σῳζηται", _, "~σῳζωνται";
    subj_pass_1: "~σῳζωμαι", _, _;
    subj_pass_2: _, _, "~σῳζησθε";
    opt_act_1: "~σῳζοιμῐ", _, "~σῳζοιμεν";
    opt_act_2: "~σῳζοις", "~σῳζοιτον", _;
    opt_act_3: "~σῳζοι", "~σῳζοιτην", "~σῳζοιεν";
    opt_mid_1: "~σῳζοιμην", _, "~σῳζοιμεθᾰ";
    opt_mid_2: "~σῳζοιο", _, "~σῳζοισθε";
    opt_mid_3: "~σῳζοιτο", "~σῳζοισθην", "~σῳζοιντο";
    opt_pass_2: _, "~σῳζοισθον", _;
    imp_act_2: "~σῳζε", _, "~σῳζετε";
    imp_act_3: _, _, "~σῳζοντων";
    imp_mid_2: "~σῳζου", "~σῳζεσθον", "~σῳζεσθε";
    imp_mid_3: _, _, "~σῳζεσθων";
    imp_pass_2: _, "~σῳζεσθον", "~σῳζεσθε";
    imp_pass_3: "~σῳζεσθω", "~σῳζεσθων", "~σῳζεσθων";
    imperfect_act_1: "ἐ~ῳζον", _, _;
    imperfect_act_2: "ἐ~ῳζες", "ἐ~ῳζετον", "ἐ~ῳζετε";
    imperfect_act_3: "ἐ~ῳζε", _, "ἐ~ῳζον";
    imperfect_mid_1: "ἐ~ῳζομην", _, "ἐ~ῳζομεθᾰ";
    imperfect_mid_2: "ἐ~ῳζου", _, _;
    imperfect_mid_3: "ἐ~ῳζετο", _, "ἐ~ῳζοντο";
    imperfect_pass_2: _, "ἐ~ῳζεσθον", "ἐ~ῳζεσθε";
    imperfect_pass_3: _, "ἐ~ῳζεσθην", _;
    future_ind_act_1: "~σωσω", _, "~σωσομεν";
    future_ind_act_2: "~σωσεις", "~σωσετον", _;
    future_ind_act_3: _, _, "~σωσουσῐ";
    future_ind_mid_1: "~σωσομαι", _, _;
    future_ind_mid_2: "~σωσῃ", "~σωσεσθον", _;
    future_ind_mid_3: "~σωσεται", _, _;
    future_opt_act_1: "~σωσοιμῐ", _, "~σωσοιμεν";
    future_opt_act_2: "~σωσοις", "~σωσοιτον", _;
    future_opt_act_3: "~σωσοι", "~σωσοιτην", _;
    future_opt_mid_1: "~σωσοιμην", _, "~σωσοιμεθᾰ";
    future_opt_mid_3: _, "~σωσοισθην", _;
    aorist_subj_act_1: "~σωσω", _, _;
    aorist_subj_act_2: "~σωσῃς", "~σωσητον", _;
    aorist_subj_mid_1: "~σωσωμαι", _, _;
    aorist_subj_mid_2: "~σωσῃ", "~σωσησθον", _;
    aorist_opt_act_1: "~σωσαιμῐ", _, "~σωσαιμεν";
    aorist_opt_act_2: "~σωσειᾰς", _, _;
    aorist_opt_act_3: "~σωσειε", _, "~σωσειᾰν";
    aorist_opt_mid_1: "~σωσαιμην", _, _;
    aorist_imp_act_2: "~σωσον", _, "~σωσᾰτε";
    aorist_imp_act_3: _, _, "~σωσᾰντων";
    aorist_imp_mid_2: "~σωσαι", "~σωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~σωσᾰσθων";
    perfect_ind_act_1: "~σεσωκᾰ", _, "~σεσωκᾰμεν";
    perfect_ind_act_2: _, "~σεσωκᾰτον", _;
    perfect_ind_act_3: _, _, "~σεσωκᾱσῐ";
    perfect_ind_mid_1: _, _, "~σεσωσμεθᾰ";
    perfect_ind_mid_2: "~σεσωσαι", "~σεσωσθον", _;
    perfect_ind_mid_3: "~σεσωσται", _, "~σεσωσᾰται";
    perfect_ind_pass_1: "~σεσωσμαι", _, _;
    perfect_ind_pass_2: _, _, "~σεσωσθε";
    perfect_subj_act_1: "~σεσωκω", _, _;
    perfect_subj_act_2: "~σεσωκῃς", "~σεσωκητον", "~σεσωκητε";
    perfect_subj_act_3: "~σεσωκῃ", _, "~σεσωκωσῐ";
    perfect_imp_act_2: "~σεσωκε", _, _;
    perfect_imp_mid_2: _, _, "~σεσωσθε";
    perfect_imp_mid_3: _, _, "~σεσωσθων";
    perfect_imp_pass_2: "~σεσωσο", _, "~σεσωσθε";
    perfect_imp_pass_3: "~σεσωσθω", "~σεσωσθων", "~σεσωσθων";
    infinitive_pres: "~σῳζειν", _, "~σῳζεσθαι";
    infinitive_fut: "~σωσειν", _, _;
    infinitive_aor: "~σωσαι", _, _;
    infinitive_perf: _, "~σεσωσθαι", _;
    participle_pres: "~σῳζων", _, "~σῳζομενος";
    participle_aor: "~σωσᾱς", "~σωσᾰμενος", _;
    participle_perf: "~σεσωκως", _, "~σεσωσμενος";
    ),
    // Κᾰτᾰβῐβᾰζω: 3 words (κᾰτᾰβῐβᾰζω, πᾰρᾰσκευᾰζω, κᾰτᾰσκευᾰζω). Like Δῐᾰρπᾰζω, except in 79 cells: imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, imperfect_act_3 pl, ….
    conjugation!("κᾰτᾰβῐβᾰζω", 3, "~ᾰ~ζω", like Δῐᾰρπᾰζω;
    stem aorist_ind: "~ε~σ- like εὐλογεω";
    stem aorist_ind_pass: "~ε~σθην- like εὐλογεω";
    imperfect_act_1: "~ε~ζον", _, _;
    imperfect_act_2: "~ε~ζες", "~ε~ζετον", "~ε~ζετε";
    imperfect_act_3: "~ε~ζε|~ε~ζεν", _, "~ε~ζον";
    imperfect_mid_1: "~ε~ζομην", _, "~ε~ζομεθᾰ";
    imperfect_mid_2: "~ε~ζου", _, _;
    imperfect_mid_3: "~ε~ζετο", _, "~ε~ζοντο";
    imperfect_pass_2: _, "~ε~ζεσθον", "~ε~ζεσθε";
    imperfect_pass_3: _, "~ε~ζεσθην", _;
    future_ind_act_1: "~ᾰ~σω", _, "~ᾰ~σομεν";
    future_ind_act_2: "~ᾰ~σεις", "~ᾰ~σετον", _;
    future_ind_act_3: _, _, "~ᾰ~σουσῐ|~ᾰ~σουσῐν";
    future_ind_mid_1: "~ᾰ~σομαι", _, _;
    future_ind_mid_2: "~ᾰ~σῃ|~ᾰ~σει", "~ᾰ~σεσθον", _;
    future_ind_mid_3: "~ᾰ~σεται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~ᾰ~σοιμῐ", _, "~ᾰ~σοιμεν";
    future_opt_act_2: "~ᾰ~σοις", "~ᾰ~σοιτον", derived;
    future_opt_act_3: "~ᾰ~σοι", "~ᾰ~σοιτην", derived;
    future_opt_mid_1: "~ᾰ~σοιμην", _, "~ᾰ~σοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~σοισθην", _;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_mid_1: _, _, "~ε~σμεθᾰ";
    perfect_ind_mid_2: "~ε~σαι", "~ε~σθον", _;
    perfect_ind_mid_3: "~ε~σται", _, "~ε~σᾰται";
    perfect_ind_pass_1: "~ε~σμαι", _, _;
    perfect_ind_pass_2: _, _, "~ε~σθε";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, _;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: derived, _, _;
    perfect_imp_act_2: derived, _, _;
    perfect_imp_mid_2: _, _, "~ε~σθε";
    perfect_imp_mid_3: _, _, "~ε~σθων";
    perfect_imp_pass_2: "~ε~σο", _, "~ε~σθε";
    perfect_imp_pass_3: "~ε~σθω", "~ε~σθων", "~ε~σθων";
    pluperfect_act_1: derived, _, _;
    pluperfect_act_2: derived, _, _;
    pluperfect_act_3: derived, _, _;
    pluperfect_mid_1: "~ε~σμην", _, _;
    pluperfect_mid_2: "~ε~σο", "~ε~σθον", _;
    pluperfect_mid_3: "~ε~στο", _, "~ε~σᾰτο";
    pluperfect_pass_1: _, _, "~ε~σμεθᾰ";
    pluperfect_pass_2: _, _, "~ε~σθε";
    pluperfect_pass_3: _, "~ε~σθην", _;
    infinitive_fut: "~ᾰ~σειν", _, _;
    infinitive_perf: _, "~ε~σθαι", _;
    participle_perf: derived, _, "~ε~σμενος";
    ),
    // Ῥηγνῡμῐ: 3 words (ῥηγνῡμῐ, ξῠρρηγνῡμῐ, ρρηγνῡμῐ). Like Ἀειρω, except in 149 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ῥηγνῡμῐ", 3, "~ηγνῡμῐ", like Ἀειρω;
    stem aorist_ind: "~ηξ- like ἐχω";
    stem aorist_ind_pass: "~ᾰγην- like ἐχω";
    stem aorist_pass: "~ᾰγ- like ἐγειρω";
    ind_act_1: "~ηγνῡμῐ", _, "~ηγνῠμεν";
    ind_act_2: "~ηγνῡς", "~ηγνῠτον", _;
    ind_act_3: "~ηγνῡσῐ", _, "~ηγνῠᾱσῐ";
    ind_mid_1: _, _, "~ηγνῠμεθᾰ";
    ind_mid_2: "~ηγνῠσαι", _, _;
    ind_mid_3: _, "~ηγνῠσθον", _;
    ind_pass_1: "~ηγνῠμαι", _, _;
    ind_pass_2: _, "~ηγνῠσθον", "~ηγνῠσθε";
    ind_pass_3: "~ηγνῠται", "~ηγνῠσθον", "~ηγνῠνται";
    subj_act_1: "~ηγνῠω", _, "~ηγνῠωμεν";
    subj_act_2: "~ηγνῠῃς", _, _;
    subj_act_3: "~ηγνῠῃ", "~ηγνῠητον", "~ηγνῠωσῐ";
    subj_mid_1: _, _, "~ηγνῠωμεθᾰ";
    subj_mid_2: _, "~ηγνῠησθον", _;
    subj_mid_3: "~ηγνῠηται", _, "~ηγνῠωνται";
    subj_pass_1: "~ηγνῠωμαι", _, _;
    subj_pass_2: _, _, "~ηγνῠησθε";
    opt_act_1: "~ηγνῠοιμῐ", _, "~ηγνῠοιμεν";
    opt_act_2: "~ηγνῠοις", "~ηγνῠοιτον", _;
    opt_act_3: "~ηγνῠοι", "~ηγνῠοιτην", "~ηγνῠοιεν";
    opt_mid_1: "~ηγνῠοιμην", _, "~ηγνῠοιμεθᾰ";
    opt_mid_2: "~ηγνῠοιο", _, "~ηγνῠοισθε";
    opt_mid_3: "~ηγνῠοιτο", "~ηγνῠοισθην", "~ηγνῠοιντο";
    opt_pass_2: _, "~ηγνῠοισθον", _;
    imp_act_2: "~ηγνῡ", _, "~ηγνῠτε";
    imp_act_3: _, _, "~ηγνῠντων";
    imp_mid_2: "~ηγνῠσο", "~ηγνῠσθον", "~ηγνῠσθε";
    imp_mid_3: _, _, "~ηγνῠσθων";
    imp_pass_2: _, "~ηγνῠσθον", "~ηγνῠσθε";
    imp_pass_3: "~ηγνῠσθω", "~ηγνῠσθων", "~ηγνῠσθων";
    imperfect_act_1: "ἐ~ηγνῡν", _, _;
    imperfect_act_2: "ἐ~ηγνῡς", "ἐ~ηγνῠτον", "ἐ~ηγνῠτε";
    imperfect_act_3: "ἐ~ηγνῡ", _, "ἐ~ηγνῠσᾰν";
    imperfect_mid_1: "ἐ~ηγνῠμην", _, "ἐ~ηγνῠμεθᾰ";
    imperfect_mid_2: "ἐ~ηγνῠσο", _, _;
    imperfect_mid_3: "ἐ~ηγνῠτο", _, "ἐ~ηγνῠντο";
    imperfect_pass_2: _, "ἐ~ηγνῠσθον", "ἐ~ηγνῠσθε";
    imperfect_pass_3: _, "ἐ~ηγνῠσθην", _;
    future_ind_act_1: "~ηξω", _, "~ηξομεν";
    future_ind_act_2: "~ηξεις", "~ηξετον", _;
    future_ind_act_3: _, _, "~ηξουσῐ";
    future_ind_mid_1: "~ηξομαι", _, _;
    future_ind_mid_2: "~ηξῃ", "~ηξεσθον", _;
    future_ind_mid_3: "~ηξεται", _, _;
    future_opt_act_1: "~ηξοιμῐ", _, "~ηξοιμεν";
    future_opt_act_2: "~ηξοις", "~ηξοιτον", _;
    future_opt_act_3: "~ηξοι", "~ηξοιτην", _;
    future_opt_mid_1: "~ηξοιμην", _, "~ηξοιμεθᾰ";
    future_opt_mid_3: _, "~ηξοισθην", _;
    aorist_subj_act_1: "~ηξω", _, _;
    aorist_subj_act_2: "~ηξῃς", "~ηξητον", _;
    aorist_subj_mid_1: "~ηξωμαι", _, _;
    aorist_subj_mid_2: "~ηξῃ", "~ηξησθον", _;
    aorist_opt_act_1: "~ηξαιμῐ", _, "~ηξαιμεν";
    aorist_opt_act_2: "~ηξειᾰς", _, _;
    aorist_opt_act_3: "~ηξειε", _, "~ηξειᾰν";
    aorist_opt_mid_1: "~ηξαιμην", _, _;
    aorist_imp_act_2: "~ηξον", _, "~ηξᾰτε";
    aorist_imp_act_3: _, _, "~ηξᾰντων";
    aorist_imp_mid_2: "~ηξαι", "~ηξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ηξᾰσθων";
    perfect_ind_act_1: "ἐ~ωγᾰ", _, "ἐ~ωγᾰμεν";
    perfect_ind_act_2: _, "ἐ~ωγᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~ωγᾱσῐ";
    perfect_ind_mid_1: _, _, "~ηρρηγμεθᾰ";
    perfect_ind_mid_2: "~ηρρηξαι", "~ηρρηχθον", _;
    perfect_ind_mid_3: "~ηρρηκται", _, "~ηρρηγᾰται";
    perfect_ind_pass_1: "~ηρρηγμαι", _, _;
    perfect_ind_pass_2: _, _, "~ηρρηχθε";
    perfect_subj_act_1: "ἐ~ωγω", _, _;
    perfect_subj_act_2: "ἐ~ωγῃς", "ἐ~ωγητον", "ἐ~ωγητε";
    perfect_subj_act_3: "ἐ~ωγῃ", _, "ἐ~ωγωσῐ";
    perfect_imp_act_2: "ἐ~ωγε", _, _;
    perfect_imp_mid_2: _, _, "~ηρρηχθε";
    perfect_imp_mid_3: _, _, "~ηρρηχθων";
    perfect_imp_pass_2: "~ηρρηξο", _, "~ηρρηχθε";
    perfect_imp_pass_3: "~ηρρηχθω", "~ηρρηχθων", "~ηρρηχθων";
    pluperfect_act_1: "ἐ~ωγειν", _, _;
    pluperfect_mid_1: "~ηρρηγμην", _, _;
    pluperfect_mid_2: "~ηρρηξο", "~ηρρηχθον", _;
    pluperfect_mid_3: "~ηρρηκτο", _, "~ηρρηγᾰτο";
    pluperfect_pass_1: _, _, "~ηρρηγμεθᾰ";
    pluperfect_pass_2: _, _, "~ηρρηχθε";
    pluperfect_pass_3: _, "~ηρρηχθην", _;
    infinitive_pres: "~ηγνῠναι", _, "~ηγνῠσθαι";
    infinitive_fut: "~ηξειν", _, _;
    infinitive_aor: "~ηξαι", _, _;
    infinitive_perf: _, "~ηρρηχθαι", _;
    participle_pres: "~ηγνῡς", _, "~ηγνῠμενος";
    participle_aor: "~ηξᾱς", "~ηξᾰμενος", _;
    participle_perf: "ἐ~ωγως", _, "~ηρρηγμενος";
    ),
    // Ὠνεομαι: 3 words (ὠνεομαι, θρεομαι, λογεομαι). Like Δηλεομαι, except in 73 cells: ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 sg, ind_pass_2 du, ….
    conjugation!("ὠνεομαι", 3, "~εομαι", like Δηλεομαι;
    stem aorist_pass: "~ηθ- like ἐρωτᾰω";
    ind_mid_1: "~εομαι", _, "~εομεθᾰ";
    ind_mid_2: "~εῃ", _, _;
    ind_mid_3: _, "~εεσθον", _;
    ind_pass_2: derived, "~εεσθον", "~εεσθε";
    ind_pass_3: "~εεται", "~εεσθον", "~εονται";
    subj_mid_1: "~εωμαι", _, "~εωμεθᾰ";
    subj_mid_2: "~εῃ", "~εησθον", _;
    subj_mid_3: "~εηται", _, "~εωνται";
    subj_pass_2: "~εῃ", _, "~εησθε";
    opt_mid_1: "~εοιμην", _, "~εοιμεθᾰ";
    opt_mid_2: "~εοιο", _, "~εοισθε";
    opt_mid_3: "~εοιτο", "~εοισθην", "~εοιντο";
    opt_pass_2: _, "~εοισθον", _;
    imp_mid_2: "~εου", "~εεσθον", "~εεσθε";
    imp_mid_3: "~εεσθω", _, "~εεσθων";
    imp_pass_2: _, "~εεσθον", "~εεσθε";
    imp_pass_3: _, "~εεσθων", "~εεσθων";
    imperfect_mid_1: "ἐ~εομην", _, "ἐ~εομεθᾰ";
    imperfect_mid_2: "ἐ~εου", "ἐ~εεσθον", _;
    imperfect_mid_3: "ἐ~εετο", _, "ἐ~εοντο";
    imperfect_pass_2: _, _, "ἐ~εεσθε";
    imperfect_pass_3: _, "ἐ~εεσθην", _;
    future_ind_mid_2: "~ησῃ", _, _;
    aorist_subj_mid_1: "ἐ~ησωμαι", _, _;
    aorist_subj_mid_2: "ἐ~ησῃ", "ἐ~ησησθον", _;
    aorist_opt_mid_1: "ἐ~ησαιμην", _, _;
    aorist_imp_mid_2: "ἐ~ησαι", "ἐ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἐ~ησᾰσθων";
    future_perfect_ind_mid_1: "%ε~ησομαι", _, _;
    future_perfect_ind_mid_3: _, "%ε~ησεσθον", _;
    future_perfect_ind_pass_1: _, _, "%ε~ησομεθᾰ";
    future_perfect_ind_pass_2: "%ε~ησῃ", "%ε~ησεσθον", "%ε~ησεσθε";
    future_perfect_ind_pass_3: "%ε~ησεται", "%ε~ησεσθον", "%ε~ησονται";
    future_perfect_opt_pass_1: "%ε~ησοιμην", _, "%ε~ησοιμεθᾰ";
    future_perfect_opt_pass_2: "%ε~ησοιο", "%ε~ησοισθον", "%ε~ησοισθε";
    future_perfect_opt_pass_3: "%ε~ησοιτο", "%ε~ησοισθην", "%ε~ησοιντο";
    infinitive_pres: _, _, "~εεσθαι";
    infinitive_futp: _, _, "%ε~ησεσθαι";
    participle_pres: _, _, "~εομενος";
    participle_aor: _, "ἐ~ησᾰμενος", _;
    participle_futp: _, _, "%ε~ησομενος";
    ),
    // Κᾰτᾰπονεω: 3 words (ὑπᾰνᾰχωρεω, ἐπᾰνᾰχωρεω, κᾰτᾰπονεω). Like Κατοικεω, except in 60 cells: imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, imperfect_act_3 pl, ….
    conjugation!("κᾰτᾰπονεω", 3, "~ᾰ~εω", like Κατοικεω;
    stem aorist_ind: "~ε~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ε~ηθην- like εὐλογεω";
    imperfect_act_1: "~ε~ουν", _, _;
    imperfect_act_2: "~ε~εις", "~ε~ειτον", "~ε~ειτε";
    imperfect_act_3: "~ε~ει", _, "~ε~ουν";
    imperfect_mid_1: "~ε~ουμην", _, "~ε~ουμεθᾰ";
    imperfect_mid_2: "~ε~ου", _, _;
    imperfect_mid_3: "~ε~ειτο", _, "~ε~ουντο";
    imperfect_pass_2: _, "~ε~εισθον", "~ε~εισθε";
    imperfect_pass_3: _, "~ε~εισθην", _;
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ|~ᾰ~ησουσῐν";
    aorist_subj_act_3: _, _, "~ᾰ~ησωσῐ|~ᾰ~ησωσῐν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς|~ᾰ~ησαις", _, _;
    aorist_opt_act_3: "~ᾰ~ησειε|~ᾰ~ησειεν|~ᾰ~ησαι", _, "~ᾰ~ησειᾰν|~ᾰ~ησαιεν";
    perfect_ind_act_1: "~ᾰ%ε~ηκᾰ", _, "~ᾰ%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "~ᾰ%ε~ηκᾰτον", _;
    perfect_ind_act_3: "~ᾰ%ε~ηκε|~ᾰ%ε~ηκεν", _, "~ᾰ%ε~ηκᾱσῐ|~ᾰ%ε~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ᾰ%ε~ημεθᾰ";
    perfect_ind_mid_2: "~ᾰ%ε~ησαι", "~ᾰ%ε~ησθον", _;
    perfect_ind_mid_3: "~ᾰ%ε~ηται", _, "~ᾰ%ε~ηνται";
    perfect_ind_pass_1: "~ᾰ%ε~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ᾰ%ε~ησθε";
    perfect_subj_act_1: "~ᾰ%ε~ηκω", _, _;
    perfect_subj_act_2: "~ᾰ%ε~ηκῃς", "~ᾰ%ε~ηκητον", "~ᾰ%ε~ηκητε";
    perfect_subj_act_3: "~ᾰ%ε~ηκῃ", _, "~ᾰ%ε~ηκωσῐ|~ᾰ%ε~ηκωσῐν";
    perfect_opt_act_1: "~ᾰ%ε~ηκοιμῐ|~ᾰ%ε~ηκοιην", _, _;
    perfect_opt_act_2: "~ᾰ%ε~ηκοις|~ᾰ%ε~ηκοιης", _, _;
    perfect_opt_act_3: "~ᾰ%ε~ηκοι|~ᾰ%ε~ηκοιη", _, _;
    perfect_imp_act_2: "~ᾰ%ε~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~ᾰ%ε~ησθε";
    perfect_imp_mid_3: _, _, "~ᾰ%ε~ησθων";
    perfect_imp_pass_2: "~ᾰ%ε~ησο", _, "~ᾰ%ε~ησθε";
    perfect_imp_pass_3: "~ᾰ%ε~ησθω", "~ᾰ%ε~ησθων", "~ᾰ%ε~ησθων";
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: _, "~ᾰ%ε~ησθαι", _;
    participle_perf: "~ᾰ%ε~ηκως", _, "~ᾰ%ε~ημενος";
    ),
    // Κηρῡσσω: 3 words (πῠρεσσω, κηρῡσσω, ἀλλᾰσσω). Like Τᾰσσω, except in 8 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_ind_mid_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, ….
    conjugation!("κηρῡσσω", 3, "~σσω", like Τᾰσσω;
    perfect_ind_act_3: "%ε~χε|%ε~χεν", _, "%ε~χᾱσῐ|%ε~χᾱσῐν";
    perfect_ind_mid_3: _, _, "%ε~χᾰται";
    perfect_subj_act_3: _, _, "%ε~χωσῐ|%ε~χωσῐν";
    perfect_opt_act_1: "%ε~χοιμῐ|%ε~χοιην", _, _;
    perfect_opt_act_2: "%ε~χοις|%ε~χοιης", _, _;
    perfect_opt_act_3: "%ε~χοι|%ε~χοιη", _, _;
    pluperfect_mid_3: _, _, "ἐ%ε~χᾰτο";
    ),
    // Ἐπορον: 2 words (ἐφᾰγον, ἐπορον). Like Εἰμῐ, except in 23 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, ….
    conjugation!("ἐπορον", 2, "ἐ~ον", like Εἰμῐ;
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
    // Εἰδον: 2 words (εἰπον, εἰδον). Like Ἐπορον, except in 28 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_3 pl, ….
    conjugation!("εἰδον", 2, "ε~ον", like Ἐπορον;
    stem aorist_ind: "~- like εἰδον";
    ind_act_1: "ε~ον", _, _;
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", _;
    future_ind_act_3: _, _, "~ησουσῐ";
    future_ind_mid_1: "~ησομαι", _, _;
    future_ind_mid_2: "~ησῃ", "~ησεσθον", _;
    future_ind_mid_3: "~ησεται", _, _;
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: "~ησοις", "~ησοιτον", _;
    future_opt_act_3: "~ησοι", "~ησοιτην", _;
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ησοισθην", _;
    aorist_subj_mid_1: "~ωμαι", _, _;
    aorist_subj_mid_2: "~ῃ", "~ησθον", _;
    aorist_opt_mid_1: "~οιμην", _, _;
    aorist_imp_mid_2: "~ου", "~εσθον", _;
    aorist_imp_mid_3: _, _, "~εσθων";
    infinitive_fut: "~ησειν", _, _;
    participle_aor: _, "~ομενος", _;
    ),
    // Καινῠμαι: 2 words (πτᾰρνῠμαι, καινῠμαι). Like Ἐπορον, except in 37 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("καινῠμαι", 2, "~νῠμαι", like Ἐπορον;
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
    opt_mid_2: "~νῠοιο", _, "~νῠοισθε";
    opt_mid_3: "~νῠοιτο", "~νῠοισθην", "~νῠοιντο";
    opt_pass_2: _, "~νῠοισθον", _;
    imp_mid_2: "~νῠσο", "~νῠσθον", "~νῠσθε";
    imp_mid_3: "~νῠσθω", _, "~νῠσθων";
    imp_pass_2: _, "~νῠσθον", "~νῠσθε";
    imp_pass_3: _, "~νῠσθων", "~νῠσθων";
    infinitive_pres: _, _, "~νῠσθαι";
    participle_pres: _, _, "~νῠμενος";
    ),
    // Ξυμπολεμεω: 2 words (συμπολεμεω, ξυμπολεμεω). Like Σταθμαω, except in 133 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ξυμπολεμεω", 2, "~υμ~εω", like Σταθμαω;
    stem imperfect: "~ῠνε~- like ξυνναυμᾰχεω";
    stem future_pass: "~ῠμ~ηθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ῠνε~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ῠνε~ηθην- like εὐλογεω";
    stem aorist_pass: "~ῠμ~ηθ- like ποιεω";
    ind_act_1: "~ῠμ~ω", _, "~ῠμ~ουμεν";
    ind_act_2: "~ῠμ~εις", "~ῠμ~ειτον", _;
    ind_act_3: "~ῠμ~ει", _, "~ῠμ~ουσῐ|~ῠμ~ουσῐν";
    ind_mid_1: _, _, derived;
    ind_mid_2: derived, _, _;
    ind_mid_3: _, derived, _;
    ind_pass_1: derived, _, _;
    ind_pass_2: _, derived, derived;
    ind_pass_3: derived, derived, derived;
    subj_act_1: "~ῠμ~ω", _, "~ῠμ~ωμεν";
    subj_act_2: "~ῠμ~ῃς", "~ῠμ~ητον", _;
    subj_act_3: "~ῠμ~ῃ", derived, "~ῠμ~ωσῐ|~ῠμ~ωσῐν";
    subj_mid_1: _, _, derived;
    subj_mid_2: _, derived, _;
    subj_mid_3: derived, _, derived;
    subj_pass_1: derived, _, _;
    subj_pass_2: _, _, derived;
    opt_act_1: "~ῠμ~οιην|~ῠμ~οιμῐ", _, "~ῠμ~οιμεν|~ῠμ~οιημεν";
    opt_act_2: "~ῠμ~οιης|~ῠμ~οις", "~ῠμ~οιτον|~ῠμ~οιητον", "~ῠμ~οιτε|~ῠμ~οιητε";
    opt_act_3: "~ῠμ~οιη|~ῠμ~οι", "~ῠμ~οιτην|~ῠμ~οιητην", "~ῠμ~οιεν|~ῠμ~οιησᾰν";
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, _, derived;
    opt_mid_3: derived, derived, derived;
    opt_pass_2: _, derived, _;
    imp_act_2: "~ῠμ~ει", _, "~ῠμ~ειτε";
    imp_act_3: _, _, "~ῠμ~ουντων";
    imp_mid_2: derived, derived, derived;
    imp_mid_3: _, _, derived;
    imp_pass_2: _, derived, derived;
    imp_pass_3: derived, derived, derived;
    future_ind_act_1: "~ῠμ~ησω", _, "~ῠμ~ησομεν";
    future_ind_act_2: "~ῠμ~ησεις", "~ῠμ~ησετον", _;
    future_ind_act_3: _, _, "~ῠμ~ησουσῐ|~ῠμ~ησουσῐν";
    future_ind_mid_1: "~ῠμ~ησομαι", _, _;
    future_ind_mid_2: "~ῠμ~ησῃ|~ῠμ~ησει", "~ῠμ~ησεσθον", _;
    future_ind_mid_3: "~ῠμ~ησεται", _, _;
    future_opt_act_1: "~ῠμ~ησοιμῐ", _, "~ῠμ~ησοιμεν";
    future_opt_act_2: "~ῠμ~ησοις", "~ῠμ~ησοιτον", _;
    future_opt_act_3: "~ῠμ~ησοι", "~ῠμ~ησοιτην", _;
    future_opt_mid_1: "~ῠμ~ησοιμην", _, "~ῠμ~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ῠμ~ησοισθην", _;
    aorist_subj_act_1: "~ῠμ~ησω", _, _;
    aorist_subj_act_2: "~ῠμ~ησῃς", "~ῠμ~ησητον", _;
    aorist_subj_act_3: _, _, "~ῠμ~ησωσῐ|~ῠμ~ησωσῐν";
    aorist_subj_mid_1: "~ῠμ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ῠμ~ησῃ", "~ῠμ~ησησθον", _;
    aorist_opt_act_1: "~ῠμ~ησαιμῐ", _, "~ῠμ~ησαιμεν";
    aorist_opt_act_2: "~ῠμ~ησειᾰς|~ῠμ~ησαις", _, _;
    aorist_opt_act_3: "~ῠμ~ησειε|~ῠμ~ησειεν|~ῠμ~ησαι", _, "~ῠμ~ησειᾰν|~ῠμ~ησαιεν";
    aorist_opt_mid_1: "~ῠμ~ησαιμην", _, _;
    aorist_imp_act_2: "~ῠμ~ησον", _, "~ῠμ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῠμ~ησᾰντων";
    aorist_imp_mid_2: "~ῠμ~ησαι", "~ῠμ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῠμ~ησᾰσθων";
    perfect_ind_act_1: "~ῠμ%ε~ηκᾰ", _, "~ῠμ%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "~ῠμ%ε~ηκᾰτον", _;
    perfect_ind_act_3: "~ῠμ%ε~ηκε|~ῠμ%ε~ηκεν", _, "~ῠμ%ε~ηκᾱσῐ|~ῠμ%ε~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ῠμ%ε~ημεθᾰ";
    perfect_ind_mid_2: "~ῠμ%ε~ησαι", "~ῠμ%ε~ησθον", _;
    perfect_ind_mid_3: "~ῠμ%ε~ηται", _, "~ῠμ%ε~ηνται";
    perfect_ind_pass_1: "~ῠμ%ε~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ῠμ%ε~ησθε";
    perfect_subj_act_1: "~ῠμ%ε~ηκω", _, _;
    perfect_subj_act_2: "~ῠμ%ε~ηκῃς", "~ῠμ%ε~ηκητον", "~ῠμ%ε~ηκητε";
    perfect_subj_act_3: "~ῠμ%ε~ηκῃ", _, "~ῠμ%ε~ηκωσῐ|~ῠμ%ε~ηκωσῐν";
    perfect_opt_act_1: "~ῠμ%ε~ηκοιμῐ|~ῠμ%ε~ηκοιην", _, _;
    perfect_opt_act_2: "~ῠμ%ε~ηκοις|~ῠμ%ε~ηκοιης", _, _;
    perfect_opt_act_3: "~ῠμ%ε~ηκοι|~ῠμ%ε~ηκοιη", _, _;
    perfect_imp_act_2: "~ῠμ%ε~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~ῠμ%ε~ησθε";
    perfect_imp_mid_3: _, _, "~ῠμ%ε~ησθων";
    perfect_imp_pass_2: "~ῠμ%ε~ησο", _, "~ῠμ%ε~ησθε";
    perfect_imp_pass_3: "~ῠμ%ε~ησθω", "~ῠμ%ε~ησθων", "~ῠμ%ε~ησθων";
    infinitive_pres: "~ῠμ~ειν", _, derived;
    infinitive_fut: "~ῠμ~ησειν", _, _;
    infinitive_aor: "~ῠμ~ησαι", _, _;
    infinitive_perf: _, "~ῠμ%ε~ησθαι", _;
    participle_pres: "~ῠμ~ων", _, derived;
    participle_aor: "~ῠμ~ησᾱς", "~ῠμ~ησᾰμενος", _;
    participle_perf: "~ῠμ%ε~ηκως", _, "~ῠμ%ε~ημενος";
    ),
    // Δῐακειμαι: 2 words (κᾰτακειμαι, δῐακειμαι). Like Πῠνθανομαι, except in 65 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 sg, ….
    conjugation!("δῐακειμαι", 2, "~α~ειμαι", like Πῠνθανομαι;
    stem imperfect: "~ε~ει- like δῐακειμαι";
    ind_act_1: "~α~ειμαι", _, _;
    ind_mid_1: "~ᾰ~ειμαι", _, "~ᾰ~ειμεθα";
    ind_mid_2: "~ᾰ~εισαι", _, _;
    ind_mid_3: _, "~ᾰ~εισθον", _;
    ind_pass_2: derived, derived, derived;
    ind_pass_3: derived, derived, derived;
    subj_mid_2: "~ᾰ~εῃ", "~ᾰ~εησθον", _;
    subj_mid_3: "~ᾰ~εηται", _, "~ᾰ~εωνται";
    subj_pass_2: derived, _, derived;
    opt_pass_2: _, derived, _;
    imp_mid_2: "~ᾰ~εισο", "~ᾰ~εισθον", "~ᾰ~εισθον";
    imp_mid_3: "~ᾰ~εισθω", _, "~ᾰ~εισθων";
    imp_pass_2: _, derived, derived;
    imp_pass_3: _, derived, derived;
    imperfect_pass_2: _, _, derived;
    imperfect_pass_3: _, derived, _;
    future_ind_mid_1: "~ᾰ~εισομαι", _, _;
    future_ind_mid_2: "~ᾰ~εισῃ", "~ᾰ~εισεσθον", _;
    future_ind_mid_3: "~ᾰ~εισεται", _, _;
    future_opt_mid_1: "~ᾰ~εισοιμην", _, "~ᾰ~εισοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~εισοισθην", _;
    aorist_ind_mid_1: derived, _, _;
    aorist_ind_mid_2: derived, derived, _;
    aorist_subj_mid_1: derived, _, _;
    aorist_subj_mid_2: derived, derived, _;
    aorist_opt_mid_1: derived, _, _;
    aorist_imp_mid_2: derived, derived, _;
    aorist_imp_mid_3: _, _, derived;
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    infinitive_pres: _, _, derived;
    infinitive_perf: _, derived, _;
    participle_pres: _, _, derived;
    participle_aor: _, derived, _;
    participle_perf: _, _, derived;
    ),
    // Ξῠνοιδᾰ: 2 words (ξῠνοιδᾰ, προοιδᾰ). Like Γεγονοι with `δᾰ` after each pattern, except in 50 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, subj_act_1 sg, ….
    conjugation!("ξῠνοιδᾰ", 2, "~οιδᾰ", like Γεγονοι with "δᾰ";
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
    future_opt_mid_1: "~εισοιμην", _, "~εισοιμεθᾰ";
    future_opt_mid_3: _, "~εισοισθην", _;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_imp_act_2: derived, _, _;
    infinitive_pres: "~ειδεναι", _, _;
    participle_pres: "~ειδως", _, _;
    participle_perf: derived, _, _;
    ),
    // Συγκειμαι: 2 words (συγκειμαι, ξυγκειμαι). Like Δῐακειμαι, except in 31 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_mid_3 pl, ….
    conjugation!("συγκειμαι", 2, "~γ~ειμαι", like Δῐακειμαι;
    stem imperfect: "~νε~ε- like συγκειμαι";
    ind_act_1: "~γ~ειμαι", _, _;
    ind_mid_1: "~γ~ειμαι", _, "~γ~ειμεθα";
    ind_mid_2: "~γ~εισαι", _, _;
    ind_mid_3: _, "~γ~εισθον", "~γ~εινται|~γ~εᾰται|~γ~εονται|~γ~ειᾰται";
    subj_mid_1: "~γ~ωμαι", _, "~γ~ωμεθᾰ";
    subj_mid_2: "~γ~εῃ", "~γ~εησθον", _;
    subj_mid_3: "~γ~εηται|~γ~ειται", _, "~γ~εωνται|~γ~ειωνται";
    opt_mid_1: "~γ~οιμην", _, "~γ~οιμεθᾰ";
    opt_mid_2: "~γ~οιο", _, "~γ~οισθε";
    opt_mid_3: "~γ~οιτο", "~γ~οισθην", "~γ~οιντο";
    imp_mid_2: "~γ~εισο", "~γ~εισθον", "~γ~εισθον";
    imp_mid_3: "~γ~εισθω", _, "~γ~εισθων";
    future_ind_mid_1: "~γ~εισομαι", _, _;
    future_ind_mid_2: "~γ~εισῃ|~γ~εισει", "~γ~εισεσθον", _;
    future_ind_mid_3: "~γ~εισεται", _, _;
    future_opt_mid_1: "~γ~εισοιμην", _, "~γ~εισοιμεθᾰ";
    future_opt_mid_3: _, "~γ~εισοισθην", _;
    ),
    // Νισσομαι: 2 words (κορινθιαζομαι, νισσομαι). Like Δῐακειμαι, except in 50 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("νισσομαι", 2, "~ι~ομαι", like Δῐακειμαι;
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
    opt_mid_2: "~ῐ~οιο", _, "~ῐ~οισθε";
    opt_mid_3: "~ῐ~οιτο", "~ῐ~οισθην", "~ῐ~οιντο";
    opt_pass_2: _, "~ῐ~οισθον", _;
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
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_3: _, derived, _;
    infinitive_pres: _, _, "~ῐ~εσθαι";
    participle_pres: _, _, "~ῐ~ομενος";
    ),
    // Κᾰτασκηνοω: 2 words (στεφανοω, κᾰτασκηνοω). Like Κατοικεω, except in 119 cells: ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("κᾰτασκηνοω", 2, "~α~οω", like Κατοικεω;
    stem future_pass: "~ᾰ~ωθησ- like ἐχω";
    stem aorist_ind: "~ᾰ~ωσ- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~ωθην- like ἐχω";
    stem aorist_pass: "~ᾰ~ωθ- like ἐχω";
    ind_act_2: "~ᾰ~οις", "~ᾰ~ουτον", _;
    ind_act_3: "~ᾰ~οι", _, _;
    ind_mid_2: "~ᾰ~οι", _, _;
    ind_mid_3: _, "~ᾰ~ουσθον", _;
    ind_pass_2: _, "~ᾰ~ουσθον", "~ᾰ~ουσθε";
    ind_pass_3: "~ᾰ~ουται", "~ᾰ~ουσθον", _;
    subj_act_2: "~ᾰ~οις", _, _;
    subj_act_3: "~ᾰ~οι", "~ᾰ~ωτον", _;
    subj_mid_2: _, "~ᾰ~ωσθον", _;
    subj_mid_3: "~ᾰ~ωται", _, _;
    subj_pass_2: _, _, "~ᾰ~ωσθε";
    imp_act_2: "~ᾰ~ου", _, "~ᾰ~ουτε";
    imp_mid_2: _, "~ᾰ~ουσθον", "~ᾰ~ουσθε";
    imp_mid_3: _, _, "~ᾰ~ουσθων";
    imp_pass_2: _, "~ᾰ~ουσθον", "~ᾰ~ουσθε";
    imp_pass_3: "~ᾰ~ουσθω", "~ᾰ~ουσθων", "~ᾰ~ουσθων";
    imperfect_act_1: "ἐ~ᾰ~ουν", _, _;
    imperfect_act_2: "ἐ~ᾰ~ους", "ἐ~ᾰ~ουτον", "ἐ~ᾰ~ουτε";
    imperfect_act_3: "ἐ~ᾰ~ου", _, "ἐ~ᾰ~ουν";
    imperfect_mid_1: "ἐ~ᾰ~ουμην", _, "ἐ~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~ου", _, _;
    imperfect_mid_3: "ἐ~ᾰ~ουτο", _, "ἐ~ᾰ~ουντο";
    imperfect_pass_2: _, "ἐ~ᾰ~ουσθον", "ἐ~ᾰ~ουσθε";
    imperfect_pass_3: _, "ἐ~ᾰ~ουσθην", _;
    future_ind_act_1: "~ᾰ~ωσω", _, "~ᾰ~ωσομεν";
    future_ind_act_2: "~ᾰ~ωσεις", "~ᾰ~ωσετον", _;
    future_ind_act_3: _, _, "~ᾰ~ωσουσῐ|~ᾰ~ωσουσῐν";
    future_ind_mid_1: "~ᾰ~ωσομαι", _, _;
    future_ind_mid_2: "~ᾰ~ωσῃ", "~ᾰ~ωσεσθον", _;
    future_ind_mid_3: "~ᾰ~ωσεται", _, _;
    future_opt_act_1: "~ᾰ~ωσοιμῐ", _, "~ᾰ~ωσοιμεν";
    future_opt_act_2: "~ᾰ~ωσοις", "~ᾰ~ωσοιτον", _;
    future_opt_act_3: "~ᾰ~ωσοι", "~ᾰ~ωσοιτην", _;
    future_opt_mid_1: "~ᾰ~ωσοιμην", _, "~ᾰ~ωσοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~ωσοισθην", _;
    aorist_subj_act_1: "~ᾰ~ωσω", _, _;
    aorist_subj_act_2: "~ᾰ~ωσῃς", "~ᾰ~ωσητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~ωσωσῐ|~ᾰ~ωσωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ωσῃ", "~ᾰ~ωσησθον", _;
    aorist_opt_act_1: "~ᾰ~ωσαιμῐ", _, "~ᾰ~ωσαιμεν";
    aorist_opt_act_2: "~ᾰ~ωσειᾰς|~ᾰ~ωσαις", _, _;
    aorist_opt_act_3: "~ᾰ~ωσειε|~ᾰ~ωσειεν|~ᾰ~ωσαι", _, "~ᾰ~ωσειᾰν|~ᾰ~ωσαιεν";
    aorist_opt_mid_1: "~ᾰ~ωσαιμην", _, _;
    aorist_imp_act_2: "~ᾰ~ωσον", _, "~ᾰ~ωσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ωσᾰντων";
    aorist_imp_mid_2: "~ᾰ~ωσαι", "~ᾰ~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ωσᾰσθων";
    perfect_ind_act_1: "ἐ~ᾰ~ωκᾰ", _, "ἐ~ᾰ~ωκᾰμεν";
    perfect_ind_act_2: _, "ἐ~ᾰ~ωκᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~ᾰ~ωκᾱσῐ";
    perfect_ind_mid_1: _, _, "ἐ~ᾰ~ωμεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰ~ωσαι", "ἐ~ᾰ~ωσθον", _;
    perfect_ind_mid_3: "ἐ~ᾰ~ωται", _, "ἐ~ᾰ~ωνται";
    perfect_ind_pass_1: "ἐ~ᾰ~ωμαι", _, _;
    perfect_ind_pass_2: _, _, "ἐ~ᾰ~ωσθε";
    perfect_subj_act_1: "ἐ~ᾰ~ωκω", _, _;
    perfect_subj_act_2: "ἐ~ᾰ~ωκῃς", "ἐ~ᾰ~ωκητον", "ἐ~ᾰ~ωκητε";
    perfect_subj_act_3: "ἐ~ᾰ~ωκῃ", _, "ἐ~ᾰ~ωκωσῐ";
    perfect_imp_act_2: "ἐ~ᾰ~ωκε", _, _;
    perfect_imp_mid_2: _, _, "ἐ~ᾰ~ωσθε";
    perfect_imp_mid_3: _, _, "ἐ~ᾰ~ωσθων";
    perfect_imp_pass_2: "ἐ~ᾰ~ωσο", _, "ἐ~ᾰ~ωσθε";
    perfect_imp_pass_3: "ἐ~ᾰ~ωσθω", "ἐ~ᾰ~ωσθων", "ἐ~ᾰ~ωσθων";
    pluperfect_act_1: "ἐ~ᾰ~ωκειν", _, _;
    pluperfect_mid_1: "ἐ~ᾰ~ωμην", _, _;
    pluperfect_mid_2: "ἐ~ᾰ~ωσο", "ἐ~ᾰ~ωσθον", _;
    pluperfect_mid_3: "ἐ~ᾰ~ωτο", _, "ἐ~ᾰ~ωντο";
    pluperfect_pass_1: _, _, "ἐ~ᾰ~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ᾰ~ωσθε";
    pluperfect_pass_3: _, "ἐ~ᾰ~ωσθην", _;
    infinitive_pres: "~ᾰ~ουν", _, "~ᾰ~ουσθαι";
    infinitive_fut: "~ᾰ~ωσειν", _, _;
    infinitive_aor: "~ᾰ~ωσαι", _, _;
    infinitive_perf: _, "ἐ~ᾰ~ωσθαι", _;
    participle_aor: "~ᾰ~ωσᾱς", "~ᾰ~ωσᾰμενος", _;
    participle_perf: "ἐ~ᾰ~ωκως", _, "ἐ~ᾰ~ωμενος";
    ),
    // Γεγονοι: 2 words (γεγονοι, τεθνηκοι). Like Εἰμῐ, except in 16 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, ….
    conjugation!("γεγονοι", 2, "~οι", like Εἰμῐ;
    ind_act_1: "~οι", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    perfect_ind_act_1: "~ᾰ", _, "~ᾰμεν";
    perfect_ind_act_2: _, "~ᾰτον", _;
    perfect_ind_act_3: _, _, "~ᾱσῐ";
    perfect_subj_act_1: "~ω", _, _;
    perfect_subj_act_2: "~ῃς", "~ητον", "~ητε";
    perfect_subj_act_3: "~ῃ", _, "~ωσῐ";
    perfect_imp_act_2: "~ε", _, _;
    participle_perf: "~ως", _, _;
    ),
    // Ἐπῐχειρεω: 2 words (ἀρῐθμεω, ἐπῐχειρεω). Like Κᾰτᾰπονεω, except in 141 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐπῐχειρεω", 2, "~ῐ~εω", like Κᾰτᾰπονεω;
    stem future_pass: "~ι~ηθησ- like ἐχω";
    stem aorist_ind: "~ε~ησ- like ἐλπω";
    stem aorist_pass: "~ι~ηθ- like ἐχω";
    ind_act_1: "~ι~ω", _, "~ι~ουμεν";
    ind_act_2: "~ι~εις", "~ι~ειτον", _;
    ind_act_3: "~ι~ει", _, "~ι~ουσῐ";
    ind_mid_1: _, _, "~ι~ουμεθᾰ";
    ind_mid_2: "~ι~ει", _, _;
    ind_mid_3: _, "~ι~εισθον", _;
    ind_pass_1: "~ι~ουμαι", _, _;
    ind_pass_2: _, "~ι~εισθον", "~ι~εισθε";
    ind_pass_3: "~ι~ειται", "~ι~εισθον", "~ι~ουνται";
    subj_act_1: "~ι~ω", _, "~ι~ωμεν";
    subj_act_2: "~ι~ῃς", _, _;
    subj_act_3: "~ι~ῃ", "~ι~ητον", "~ι~ωσῐ";
    subj_mid_1: _, _, "~ι~ωμεθᾰ";
    subj_mid_2: _, "~ι~ησθον", _;
    subj_mid_3: "~ι~ηται", _, "~ι~ωνται";
    subj_pass_1: "~ι~ωμαι", _, _;
    subj_pass_2: _, _, "~ι~ησθε";
    opt_act_1: "~ι~οιην", _, "~ι~οιμεν";
    opt_act_2: "~ι~οιης", "~ι~οιτον", derived;
    opt_act_3: "~ι~οιη", "~ι~οιτην", "~ι~οιεν";
    opt_mid_1: "~ι~οιμην", _, "~ι~οιμεθᾰ";
    opt_mid_2: "~ι~οιο", _, "~ι~οισθε";
    opt_mid_3: "~ι~οιτο", "~ι~οισθην", "~ι~οιντο";
    opt_pass_2: _, "~ι~οισθον", _;
    imp_act_2: "~ι~ει", _, "~ι~ειτε";
    imp_act_3: _, _, "~ι~ουντων";
    imp_mid_2: "~ι~ου", "~ι~εισθον", "~ι~εισθε";
    imp_mid_3: _, _, "~ι~εισθων";
    imp_pass_2: _, "~ι~εισθον", "~ι~εισθε";
    imp_pass_3: "~ι~εισθω", "~ι~εισθων", "~ι~εισθων";
    future_ind_act_1: "~ι~ησω", _, "~ι~ησομεν";
    future_ind_act_2: "~ι~ησεις", "~ι~ησετον", _;
    future_ind_act_3: _, _, "~ι~ησουσῐ";
    future_ind_mid_1: "~ι~ησομαι", _, _;
    future_ind_mid_2: "~ι~ησῃ", "~ι~ησεσθον", _;
    future_ind_mid_3: "~ι~ησεται", _, _;
    future_opt_act_1: "~ι~ησοιμῐ", _, "~ι~ησοιμεν";
    future_opt_act_2: "~ι~ησοις", "~ι~ησοιτον", _;
    future_opt_act_3: "~ι~ησοι", "~ι~ησοιτην", _;
    future_opt_mid_1: "~ι~ησοιμην", _, "~ι~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ι~ησοισθην", _;
    aorist_subj_act_1: "~ι~ησω", _, _;
    aorist_subj_act_2: "~ι~ησῃς", "~ι~ησητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~ι~ησωμαι", _, _;
    aorist_subj_mid_2: "~ι~ησῃ", "~ι~ησησθον", _;
    aorist_opt_act_1: "~ι~ησαιμῐ", _, "~ι~ησαιμεν";
    aorist_opt_act_2: "~ι~ησειᾰς", _, _;
    aorist_opt_act_3: "~ι~ησειε", _, "~ι~ησειᾰν";
    aorist_opt_mid_1: "~ι~ησαιμην", _, _;
    aorist_imp_act_2: "~ι~ησον", _, "~ι~ησᾰτε";
    aorist_imp_act_3: _, _, "~ι~ησᾰντων";
    aorist_imp_mid_2: "~ι~ησαι", "~ι~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ι~ησᾰσθων";
    perfect_ind_act_1: "~ε~ηκᾰ", _, "~ε~ηκᾰμεν";
    perfect_ind_act_2: _, "~ε~ηκᾰτον", _;
    perfect_ind_act_3: derived, _, "~ε~ηκᾱσῐ";
    perfect_ind_mid_1: _, _, "~ε~ημεθᾰ";
    perfect_ind_mid_2: "~ε~ησαι", "~ε~ησθον", _;
    perfect_ind_mid_3: "~ε~ηται", _, "~ε~ηνται";
    perfect_ind_pass_1: "~ε~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ε~ησθε";
    perfect_subj_act_1: "~ε~ηκω", _, _;
    perfect_subj_act_2: "~ε~ηκῃς", "~ε~ηκητον", "~ε~ηκητε";
    perfect_subj_act_3: "~ε~ηκῃ", _, "~ε~ηκωσῐ";
    perfect_opt_act_1: derived, _, _;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: derived, _, _;
    perfect_imp_act_2: "~ε~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~ε~ησθε";
    perfect_imp_mid_3: _, _, "~ε~ησθων";
    perfect_imp_pass_2: "~ε~ησο", _, "~ε~ησθε";
    perfect_imp_pass_3: "~ε~ησθω", "~ε~ησθων", "~ε~ησθων";
    pluperfect_act_1: "~ε~ηκειν", _, _;
    pluperfect_mid_1: "~ε~ημην", _, _;
    pluperfect_mid_2: "~ε~ησο", "~ε~ησθον", _;
    pluperfect_mid_3: "~ε~ητο", _, "~ε~ηντο";
    pluperfect_pass_1: _, _, "~ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ε~ησθε";
    pluperfect_pass_3: _, "~ε~ησθην", _;
    infinitive_pres: "~ι~ειν", _, "~ι~εισθαι";
    infinitive_fut: "~ι~ησειν", _, _;
    infinitive_aor: "~ι~ησαι", _, _;
    infinitive_perf: _, "~ε~ησθαι", _;
    participle_pres: "~ι~ων", _, "~ι~ουμενος";
    participle_aor: "~ι~ησᾱς", "~ι~ησᾰμενος", _;
    participle_perf: "~ε~ηκως", _, "~ε~ημενος";
    ),
    // Συνεπομαι: 2 words (συνεπομαι, ξυνεπομαι). Like Ἐρᾰμαι, except in 60 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("συνεπομαι", 2, "~υνεπομαι", like Ἐρᾰμαι;
    stem aorist_ind: "~ῠνεσπ- like πᾰρᾰγῐγνομαι";
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
    opt_mid_2: "~ῠνεποιο", _, "~ῠνεποισθε";
    opt_mid_3: "~ῠνεποιτο", "~ῠνεποισθην", "~ῠνεποιντο";
    opt_pass_2: _, "~ῠνεποισθον", _;
    imp_mid_2: "~ῠνεπου", "~ῠνεπεσθον", "~ῠνεπεσθε";
    imp_mid_3: "~ῠνεπεσθω", _, "~ῠνεπεσθων";
    imp_pass_2: _, "~ῠνεπεσθον", "~ῠνεπεσθε";
    imp_pass_3: _, "~ῠνεπεσθων", "~ῠνεπεσθων";
    imperfect_mid_1: "~ῠνειπομην", _, "~ῠνειπομεθᾰ";
    imperfect_mid_2: "~ῠνειπου", "~ῠνειπεσθον", _;
    imperfect_mid_3: "~ῠνειπετο", _, "~ῠνειποντο";
    imperfect_pass_2: _, _, "~ῠνειπεσθε";
    imperfect_pass_3: _, "~ῠνειπεσθην", _;
    future_ind_mid_1: "~ῠνεψομαι", _, _;
    future_ind_mid_2: "~ῠνεψῃ|~ῠνεψει", "~ῠνεψεσθον", _;
    future_ind_mid_3: "~ῠνεψεται", _, _;
    future_opt_mid_1: "~ῠνεψοιμην", _, "~ῠνεψοιμεθᾰ";
    future_opt_mid_3: _, "~ῠνεψοισθην", _;
    aorist_subj_mid_1: "~ῠσπωμαι", _, _;
    aorist_subj_mid_2: "~ῠσπῃ", "~ῠσπησθον", _;
    aorist_opt_mid_1: "~ῠσποιμην", _, _;
    aorist_imp_mid_2: "~ῠσπου", "~ῠσπεσθον", _;
    aorist_imp_mid_3: _, _, "~ῠσπεσθων";
    infinitive_pres: _, _, "~ῠνεπεσθαι";
    participle_pres: _, _, "~ῠνεπομενος";
    participle_aor: _, "~ῠσπομενος", _;
    ),
    // Λαλαγεω: 2 words (καναχεω, λαλαγεω). Like Ἐπᾰγγελλω, except in 149 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("λαλαγεω", 2, "~α~α~εω", like Ἐπᾰγγελλω;
    stem future_pass: "~ᾰ~ᾰ~ηθησ- like ἐχω";
    stem aorist_ind: "~ᾰ~ᾰ~ησ- like ποιεω";
    stem aorist_ind_pass: "~ᾰ~ᾰ~ηθην- like ἐχω";
    stem aorist_pass: "~ᾰ~ᾰ~ηθ- like ποιεω";
    ind_act_1: "~ᾰ~ᾰ~ω", _, "~ᾰ~ᾰ~ουμεν";
    ind_act_2: "~ᾰ~ᾰ~εις", "~ᾰ~ᾰ~ειτον", _;
    ind_act_3: "~ᾰ~ᾰ~ει", _, "~ᾰ~ᾰ~ουσῐ|~ᾰ~ᾰ~ουσῐν";
    ind_mid_1: _, _, "~ᾰ~ᾰ~ουμεθᾰ";
    ind_mid_2: "~ᾰ~ᾰ~ει|~ᾰ~ᾰ~ῃ", _, _;
    ind_mid_3: _, "~ᾰ~ᾰ~εισθον", _;
    ind_pass_1: "~ᾰ~ᾰ~ουμαι", _, _;
    ind_pass_2: _, "~ᾰ~ᾰ~εισθον", "~ᾰ~ᾰ~εισθε";
    ind_pass_3: "~ᾰ~ᾰ~ειται", "~ᾰ~ᾰ~εισθον", "~ᾰ~ᾰ~ουνται";
    subj_act_1: "~ᾰ~ᾰ~ω", _, "~ᾰ~ᾰ~ωμεν";
    subj_act_2: "~ᾰ~ᾰ~ῃς", _, _;
    subj_act_3: "~ᾰ~ᾰ~ῃ", "~ᾰ~ᾰ~ητον", "~ᾰ~ᾰ~ωσῐ|~ᾰ~ᾰ~ωσῐν";
    subj_mid_1: _, _, "~ᾰ~ᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ᾰ~ησθον", _;
    subj_mid_3: "~ᾰ~ᾰ~ηται", _, "~ᾰ~ᾰ~ωνται";
    subj_pass_1: "~ᾰ~ᾰ~ωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~ᾰ~ησθε";
    opt_act_1: "~ᾰ~ᾰ~οιην|~ᾰ~ᾰ~οιμῐ", _, "~ᾰ~ᾰ~οιμεν|~ᾰ~ᾰ~οιημεν";
    opt_act_2: "~ᾰ~ᾰ~οιης|~ᾰ~ᾰ~οις", "~ᾰ~ᾰ~οιτον|~ᾰ~ᾰ~οιητον", "~ᾰ~ᾰ~οιτε|~ᾰ~ᾰ~οιητε";
    opt_act_3: "~ᾰ~ᾰ~οιη|~ᾰ~ᾰ~οι", "~ᾰ~ᾰ~οιτην|~ᾰ~ᾰ~οιητην", "~ᾰ~ᾰ~οιεν|~ᾰ~ᾰ~οιησᾰν";
    opt_mid_1: "~ᾰ~ᾰ~οιμην", _, "~ᾰ~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~ᾰ~οιο", _, "~ᾰ~ᾰ~οισθε";
    opt_mid_3: "~ᾰ~ᾰ~οιτο", "~ᾰ~ᾰ~οισθην", "~ᾰ~ᾰ~οιντο";
    opt_pass_2: _, "~ᾰ~ᾰ~οισθον", _;
    imp_act_2: "~ᾰ~ᾰ~ει", _, "~ᾰ~ᾰ~ειτε";
    imp_act_3: _, _, "~ᾰ~ᾰ~ουντων";
    imp_mid_2: "~ᾰ~ᾰ~ου", "~ᾰ~ᾰ~εισθον", "~ᾰ~ᾰ~εισθε";
    imp_mid_3: _, _, "~ᾰ~ᾰ~εισθων";
    imp_pass_2: _, "~ᾰ~ᾰ~εισθον", "~ᾰ~ᾰ~εισθε";
    imp_pass_3: "~ᾰ~ᾰ~εισθω", "~ᾰ~ᾰ~εισθων", "~ᾰ~ᾰ~εισθων";
    imperfect_act_1: "ἐ~ᾰ~ᾰ~ουν", _, _;
    imperfect_act_2: "ἐ~ᾰ~ᾰ~εις", "ἐ~ᾰ~ᾰ~ειτον", "ἐ~ᾰ~ᾰ~ειτε";
    imperfect_act_3: "ἐ~ᾰ~ᾰ~ει", _, "ἐ~ᾰ~ᾰ~ουν";
    imperfect_mid_1: "ἐ~ᾰ~ᾰ~ουμην", _, "ἐ~ᾰ~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~ᾰ~ου", _, _;
    imperfect_mid_3: "ἐ~ᾰ~ᾰ~ειτο", _, "ἐ~ᾰ~ᾰ~ουντο";
    imperfect_pass_2: _, "ἐ~ᾰ~ᾰ~εισθον", "ἐ~ᾰ~ᾰ~εισθε";
    imperfect_pass_3: _, "ἐ~ᾰ~ᾰ~εισθην", _;
    future_ind_act_1: "~ᾰ~ᾰ~ησω", _, "~ᾰ~ᾰ~ησομεν";
    future_ind_act_2: "~ᾰ~ᾰ~ησεις", "~ᾰ~ᾰ~ησετον", _;
    future_ind_act_3: _, _, "~ᾰ~ᾰ~ησουσῐ|~ᾰ~ᾰ~ησουσῐν";
    future_ind_mid_1: "~ᾰ~ᾰ~ησομαι", _, _;
    future_ind_mid_2: "~ᾰ~ᾰ~ησῃ|~ᾰ~ᾰ~ησει", "~ᾰ~ᾰ~ησεσθον", _;
    future_ind_mid_3: "~ᾰ~ᾰ~ησεται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~ᾰ~ᾰ~ησοιμῐ", _, "~ᾰ~ᾰ~ησοιμεν";
    future_opt_act_2: "~ᾰ~ᾰ~ησοις", "~ᾰ~ᾰ~ησοιτον", derived;
    future_opt_act_3: "~ᾰ~ᾰ~ησοι", "~ᾰ~ᾰ~ησοιτην", derived;
    future_opt_mid_1: "~ᾰ~ᾰ~ησοιμην", _, "~ᾰ~ᾰ~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~ᾰ~ησοισθην", _;
    aorist_subj_act_1: "~ᾰ~ᾰ~ηθω", _, _;
    aorist_subj_act_2: "~ᾰ~ᾰ~ηθῃς", "~ᾰ~ᾰ~ηθητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~ᾰ~ᾰ~ηθωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ᾰ~ηθῃ", "~ᾰ~ᾰ~ηθησθον", _;
    aorist_opt_act_1: "~ᾰ~ᾰ~ηθαιμῐ", _, "~ᾰ~ᾰ~ηθαιμεν";
    aorist_opt_act_2: "~ᾰ~ᾰ~ηθειᾰς", _, _;
    aorist_opt_act_3: "~ᾰ~ᾰ~ηθειε", _, "~ᾰ~ᾰ~ηθειᾰν";
    aorist_opt_mid_1: "~ᾰ~ᾰ~ηθαιμην", _, _;
    aorist_imp_act_2: "~ᾰ~ᾰ~ηθον", _, "~ᾰ~ᾰ~ηθᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ᾰ~ηθᾰντων";
    aorist_imp_mid_2: "~ᾰ~ᾰ~ηθαι", "~ᾰ~ᾰ~ηθᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ᾰ~ηθᾰσθων";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, _;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: derived, _, _;
    perfect_imp_act_2: derived, _, _;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: derived, derived, derived;
    infinitive_pres: "~ᾰ~ᾰ~ειν", _, "~ᾰ~ᾰ~εισθαι";
    infinitive_fut: "~ᾰ~ᾰ~ησειν", _, _;
    infinitive_aor: "~ᾰ~ᾰ~ηθαι", _, _;
    infinitive_perf: _, derived, _;
    participle_pres: "~ᾰ~ᾰ~ων", _, "~ᾰ~ᾰ~ουμενος";
    participle_aor: "~ᾰ~ᾰ~ηθᾱς", "~ᾰ~ᾰ~ηθᾰμενος", _;
    participle_perf: derived, _, derived;
    ),
    // Ξυνναυμᾰχεω: 2 words (συνναυμᾰχεω, ξυνναυμᾰχεω). Like Λαλαγεω, except in 126 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ξυνναυμᾰχεω", 2, "~υ~~εω", like Λαλαγεω;
    stem imperfect: "~ῠ~ε~- like ξυνναυμᾰχεω";
    stem aorist_ind: "~ῠ~ε~ησ- like ξυνναυμᾰχεω";
    ind_act_1: "~ῠ~~ω", _, "~ῠ~~ουμεν";
    ind_act_2: "~ῠ~~εις", "~ῠ~~ειτον", _;
    ind_act_3: "~ῠ~~ει", _, "~ῠ~~ουσῐ|~ῠ~~ουσῐν";
    ind_mid_1: _, _, derived;
    ind_mid_2: derived, _, _;
    ind_mid_3: _, derived, _;
    ind_pass_1: derived, _, _;
    ind_pass_2: _, derived, derived;
    ind_pass_3: derived, derived, derived;
    subj_act_1: "~ῠ~~ω", _, "~ῠ~~ωμεν";
    subj_act_2: "~ῠ~~ῃς", "~ῠ~~ητον", _;
    subj_act_3: "~ῠ~~ῃ", derived, "~ῠ~~ωσῐ|~ῠ~~ωσῐν";
    subj_mid_1: _, _, derived;
    subj_mid_2: _, derived, _;
    subj_mid_3: derived, _, derived;
    subj_pass_1: derived, _, _;
    subj_pass_2: _, _, derived;
    opt_act_1: "~ῠ~~οιην|~ῠ~~οιμῐ", _, "~ῠ~~οιμεν|~ῠ~~οιημεν";
    opt_act_2: "~ῠ~~οιης|~ῠ~~οις", "~ῠ~~οιτον|~ῠ~~οιητον", "~ῠ~~οιτε|~ῠ~~οιητε";
    opt_act_3: "~ῠ~~οιη|~ῠ~~οι", "~ῠ~~οιτην|~ῠ~~οιητην", "~ῠ~~οιεν|~ῠ~~οιησᾰν";
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, _, derived;
    opt_mid_3: derived, derived, derived;
    opt_pass_2: _, derived, _;
    imp_act_2: "~ῠ~~ει", _, "~ῠ~~ειτε";
    imp_act_3: _, _, "~ῠ~~ουντων";
    imp_mid_2: derived, derived, derived;
    imp_mid_3: _, _, derived;
    imp_pass_2: _, derived, derived;
    imp_pass_3: derived, derived, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, _, _;
    imperfect_mid_3: derived, _, derived;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, _;
    future_ind_act_1: "~ῠ~~ησω", _, "~ῠ~~ησομεν";
    future_ind_act_2: "~ῠ~~ησεις", "~ῠ~~ησετον", _;
    future_ind_act_3: _, _, "~ῠ~~ησουσῐ|~ῠ~~ησουσῐν";
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: "~ῠ~~ησοιμῐ", _, "~ῠ~~ησοιμεν";
    future_opt_act_2: "~ῠ~~ησοις", "~ῠ~~ησοιτον", _;
    future_opt_act_3: "~ῠ~~ησοι", "~ῠ~~ησοιτην", _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_3: _, derived, _;
    aorist_ind_mid_1: derived, _, _;
    aorist_ind_mid_2: derived, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ῠ~~ησω", _, _;
    aorist_subj_act_2: "~ῠ~~ησῃς", "~ῠ~~ησητον", _;
    aorist_subj_act_3: _, _, "~ῠ~~ησωσῐ|~ῠ~~ησωσῐν";
    aorist_subj_mid_1: derived, _, _;
    aorist_subj_mid_2: derived, derived, _;
    aorist_subj_pass_3: derived, derived, derived;
    aorist_opt_act_1: "~ῠ~~ησαιμῐ", _, "~ῠ~~ησαιμεν";
    aorist_opt_act_2: "~ῠ~~ησειᾰς|~ῠ~~ησαις", _, _;
    aorist_opt_act_3: "~ῠ~~ησειε|~ῠ~~ησειεν|~ῠ~~ησαι", _, "~ῠ~~ησειᾰν|~ῠ~~ησαιεν";
    aorist_opt_mid_1: derived, _, _;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: "~ῠ~~ησον", _, "~ῠ~~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῠ~~ησᾰντων";
    aorist_imp_mid_2: derived, derived, _;
    aorist_imp_mid_3: _, _, derived;
    aorist_imp_pass_2: derived, derived, derived;
    infinitive_pres: "~ῠ~~ειν", _, derived;
    infinitive_fut: "~ῠ~~ησειν", _, _;
    infinitive_aor: "~ῠ~~ησαι", _, _;
    participle_pres: "~ῠ~~ων", _, derived;
    participle_aor: "~ῠ~~ησᾱς", derived, _;
    ),
    // Μετᾰκᾰλεω: 2 words (μηχᾰνορρᾰφεω, μετᾰκᾰλεω). Like Ξυνναυμᾰχεω, except in 153 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("μετᾰκᾰλεω", 2, "~ᾰ~ᾰ~εω", like Ξυνναυμᾰχεω;
    stem future_pass: "~α~α~εθησ- like ἐχω";
    stem aorist_ind: "~ε~α~εσ- like ἐλπω";
    stem aorist_ind_pass: "~ε~α~εθην- like εὐλογεω";
    stem aorist_pass: "~α~α~εθ- like ἐχω";
    ind_act_1: "~α~α~ω", _, "~α~α~ουμεν";
    ind_act_2: "~α~α~εις", "~α~α~ειτον", _;
    ind_act_3: "~α~α~ει", _, "~α~α~ουσῐ|~α~α~ουσῐν";
    ind_mid_1: _, _, "~α~α~ουμεθᾰ";
    ind_mid_2: "~α~α~ει|~α~α~ῃ", _, _;
    ind_mid_3: _, "~α~α~εισθον", _;
    ind_pass_1: "~α~α~ουμαι", _, _;
    ind_pass_2: _, "~α~α~εισθον", "~α~α~εισθε";
    ind_pass_3: "~α~α~ειται", "~α~α~εισθον", "~α~α~ουνται";
    subj_act_1: "~α~α~ω", _, "~α~α~ωμεν";
    subj_act_2: "~α~α~ῃς", derived, _;
    subj_act_3: "~α~α~ῃ", "~α~α~ητον", "~α~α~ωσῐ|~α~α~ωσῐν";
    subj_mid_1: _, _, "~α~α~ωμεθᾰ";
    subj_mid_2: _, "~α~α~ησθον", _;
    subj_mid_3: "~α~α~ηται", _, "~α~α~ωνται";
    subj_pass_1: "~α~α~ωμαι", _, _;
    subj_pass_2: _, _, "~α~α~ησθε";
    opt_act_1: "~α~α~οιην|~α~α~οιμῐ", _, "~α~α~οιμεν|~α~α~οιημεν";
    opt_act_2: "~α~α~οιης|~α~α~οις", "~α~α~οιτον|~α~α~οιητον", "~α~α~οιτε|~α~α~οιητε";
    opt_act_3: "~α~α~οιη|~α~α~οι", "~α~α~οιτην|~α~α~οιητην", "~α~α~οιεν|~α~α~οιησᾰν";
    opt_mid_1: "~α~α~οιμην", _, "~α~α~οιμεθᾰ";
    opt_mid_2: "~α~α~οιο", _, "~α~α~οισθε";
    opt_mid_3: "~α~α~οιτο", "~α~α~οισθην", "~α~α~οιντο";
    opt_pass_2: _, "~α~α~οισθον", _;
    imp_act_2: "~α~α~ει", _, "~α~α~ειτε";
    imp_act_3: _, _, "~α~α~ουντων";
    imp_mid_2: "~α~α~ου", "~α~α~εισθον", "~α~α~εισθε";
    imp_mid_3: _, _, "~α~α~εισθων";
    imp_pass_2: _, "~α~α~εισθον", "~α~α~εισθε";
    imp_pass_3: "~α~α~εισθω", "~α~α~εισθων", "~α~α~εισθων";
    imperfect_act_1: "~ε~α~ουν", _, derived;
    imperfect_act_2: "~ε~α~εις", "~ε~α~ειτον", "~ε~α~ειτε";
    imperfect_act_3: "~ε~α~ει", _, "~ε~α~ουν";
    imperfect_mid_1: "~ε~α~ουμην", _, "~ε~α~ουμεθᾰ";
    imperfect_mid_2: "~ε~α~ου", _, _;
    imperfect_mid_3: "~ε~α~ειτο", _, "~ε~α~ουντο";
    imperfect_pass_2: _, "~ε~α~εισθον", "~ε~α~εισθε";
    imperfect_pass_3: _, "~ε~α~εισθην", _;
    future_ind_act_1: "~α~α~εσω", _, "~α~α~εσομεν";
    future_ind_act_2: "~α~α~εσεις", "~α~α~εσετον", _;
    future_ind_act_3: _, _, "~α~α~εσουσῐ";
    future_ind_mid_1: "~α~α~εσομαι", _, _;
    future_ind_mid_2: "~α~α~εσῃ", "~α~α~εσεσθον", _;
    future_ind_mid_3: "~α~α~εσεται", _, _;
    future_opt_act_1: "~α~α~εσοιμῐ", _, "~α~α~εσοιμεν";
    future_opt_act_2: "~α~α~εσοις", "~α~α~εσοιτον", _;
    future_opt_act_3: "~α~α~εσοι", "~α~α~εσοιτην", _;
    future_opt_mid_1: "~α~α~εσοιμην", _, "~α~α~εσοιμεθᾰ";
    future_opt_mid_3: _, "~α~α~εσοισθην", _;
    aorist_subj_act_1: "~α~α~εσω", _, _;
    aorist_subj_act_2: "~α~α~εσῃς", "~α~α~εσητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~α~α~εσωμαι", _, _;
    aorist_subj_mid_2: "~α~α~εσῃ", "~α~α~εσησθον", _;
    aorist_opt_act_1: "~α~α~εσαιμῐ", _, "~α~α~εσαιμεν";
    aorist_opt_act_2: "~α~α~εσειᾰς", _, _;
    aorist_opt_act_3: "~α~α~εσειε", _, "~α~α~εσειᾰν";
    aorist_opt_mid_1: "~α~α~εσαιμην", _, _;
    aorist_imp_act_2: "~α~α~εσον", _, "~α~α~εσᾰτε";
    aorist_imp_act_3: _, _, "~α~α~εσᾰντων";
    aorist_imp_mid_2: "~α~α~εσαι", "~α~α~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~α~α~εσᾰσθων";
    perfect_ind_act_1: "~α~εκα~εκᾰ", _, "~α~εκα~εκᾰμεν";
    perfect_ind_act_2: _, "~α~εκα~εκᾰτον", _;
    perfect_ind_act_3: _, _, "~α~εκα~εκᾱσῐ";
    perfect_ind_mid_1: _, _, "~α~εκα~εμεθᾰ";
    perfect_ind_mid_2: "~α~εκα~εσαι", "~α~εκα~εσθον", _;
    perfect_ind_mid_3: "~α~εκα~εται", _, "~α~εκα~ενται";
    perfect_ind_pass_1: "~α~εκα~εμαι", _, _;
    perfect_ind_pass_2: _, _, "~α~εκα~εσθε";
    perfect_subj_act_1: "~α~εκα~εκω", _, _;
    perfect_subj_act_2: "~α~εκα~εκῃς", "~α~εκα~εκητον", "~α~εκα~εκητε";
    perfect_subj_act_3: "~α~εκα~εκῃ", _, "~α~εκα~εκωσῐ";
    perfect_imp_act_2: "~α~εκα~εκε", _, _;
    perfect_imp_mid_2: _, _, "~α~εκα~εσθε";
    perfect_imp_mid_3: _, _, "~α~εκα~εσθων";
    perfect_imp_pass_2: "~α~εκα~εσο", _, "~α~εκα~εσθε";
    perfect_imp_pass_3: "~α~εκα~εσθω", "~α~εκα~εσθων", "~α~εκα~εσθων";
    pluperfect_act_1: "~ε~εκα~εκειν", _, _;
    pluperfect_mid_1: "~ε~εκα~εμην", _, _;
    pluperfect_mid_2: "~ε~εκα~εσο", "~ε~εκα~εσθον", _;
    pluperfect_mid_3: "~ε~εκα~ετο", _, "~ε~εκα~εντο";
    pluperfect_pass_1: _, _, "~ε~εκα~εμεθᾰ";
    pluperfect_pass_2: _, _, "~ε~εκα~εσθε";
    pluperfect_pass_3: _, "~ε~εκα~εσθην", _;
    infinitive_pres: "~α~α~ειν", _, "~α~α~εισθαι";
    infinitive_fut: "~α~α~εσειν", _, _;
    infinitive_aor: "~α~α~εσαι", _, _;
    infinitive_perf: _, "~α~εκα~εσθαι", _;
    participle_pres: "~α~α~ων", _, "~α~α~ουμενος";
    participle_aor: "~α~α~εσᾱς", "~α~α~εσᾰμενος", _;
    participle_perf: "~α~εκα~εκως", _, "~α~εκα~εμενος";
    ),
    // Επῐλῡω: 2 words (επῐβουλευω, επῐλῡω). Like Πεμπω, except in 157 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("επῐλῡω", 2, "ε~ῐ~ω", like Πεμπω;
    stem aorist_ind: "~ε~σ- like επῐλῡω";
    ind_act_1: "ἑ~ῐ~ω", _, "ἑ~ῐ~ομεν";
    ind_act_2: "ἑ~ῐ~εις", "ἑ~ῐ~ετον", _;
    ind_act_3: "ἑ~ῐ~ει", _, "ἑ~ῐ~ουσῐ";
    ind_mid_1: _, _, "ἑ~ῐ~ομεθᾰ";
    ind_mid_2: "ἑ~ῐ~ῃ", _, _;
    ind_mid_3: _, "ἑ~ῐ~εσθον", _;
    ind_pass_1: "ἑ~ῐ~ομαι", _, _;
    ind_pass_2: _, "ἑ~ῐ~εσθον", "ἑ~ῐ~εσθε";
    ind_pass_3: "ἑ~ῐ~εται", "ἑ~ῐ~εσθον", "ἑ~ῐ~ονται";
    subj_act_1: "ἑ~ῐ~ω", _, "ἑ~ῐ~ωμεν";
    subj_act_2: "ἑ~ῐ~ῃς", _, _;
    subj_act_3: "ἑ~ῐ~ῃ", "ἑ~ῐ~ητον", "ἑ~ῐ~ωσῐ";
    subj_mid_1: _, _, "ἑ~ῐ~ωμεθᾰ";
    subj_mid_2: _, "ἑ~ῐ~ησθον", _;
    subj_mid_3: "ἑ~ῐ~ηται", _, "ἑ~ῐ~ωνται";
    subj_pass_1: "ἑ~ῐ~ωμαι", _, _;
    subj_pass_2: _, _, "ἑ~ῐ~ησθε";
    opt_act_1: "ἑ~ῐ~οιμῐ", _, "ἑ~ῐ~οιμεν";
    opt_act_2: "ἑ~ῐ~οις", "ἑ~ῐ~οιτον", _;
    opt_act_3: "ἑ~ῐ~οι", "ἑ~ῐ~οιτην", "ἑ~ῐ~οιεν";
    opt_mid_1: "ἑ~ῐ~οιμην", _, "ἑ~ῐ~οιμεθᾰ";
    opt_mid_2: "ἑ~ῐ~οιο", _, "ἑ~ῐ~οισθε";
    opt_mid_3: "ἑ~ῐ~οιτο", "ἑ~ῐ~οισθην", "ἑ~ῐ~οιντο";
    opt_pass_2: _, "ἑ~ῐ~οισθον", _;
    imp_act_2: "ἑ~ῐ~ε", _, "ἑ~ῐ~ετε";
    imp_act_3: _, _, "ἑ~ῐ~οντων";
    imp_mid_2: "ἑ~ῐ~ου", "ἑ~ῐ~εσθον", "ἑ~ῐ~εσθε";
    imp_mid_3: _, _, "ἑ~ῐ~εσθων";
    imp_pass_2: _, "ἑ~ῐ~εσθον", "ἑ~ῐ~εσθε";
    imp_pass_3: "ἑ~ῐ~εσθω", "ἑ~ῐ~εσθων", "ἑ~ῐ~εσθων";
    imperfect_act_1: "ἑ~ε~ον", _, _;
    imperfect_act_2: "ἑ~ε~ες", "ἑ~ε~ετον", "ἑ~ε~ετε";
    imperfect_act_3: "ἑ~ε~ε", _, "ἑ~ε~ον";
    imperfect_mid_1: "ἑ~ε~ομην", _, "ἑ~ε~ομεθᾰ";
    imperfect_mid_2: "ἑ~ε~ου", _, _;
    imperfect_mid_3: "ἑ~ε~ετο", _, "ἑ~ε~οντο";
    imperfect_pass_2: _, "ἑ~ε~εσθον", "ἑ~ε~εσθε";
    imperfect_pass_3: _, "ἑ~ε~εσθην", _;
    future_ind_act_1: "ἑ~ῐ~σω", _, "ἑ~ῐ~σομεν";
    future_ind_act_2: "ἑ~ῐ~σεις", "ἑ~ῐ~σετον", _;
    future_ind_act_3: _, _, "ἑ~ῐ~σουσῐ";
    future_ind_mid_1: "ἑ~ῐ~σομαι", _, _;
    future_ind_mid_2: "ἑ~ῐ~σῃ", "ἑ~ῐ~σεσθον", _;
    future_ind_mid_3: "ἑ~ῐ~σεται", _, _;
    future_ind_pass_1: "ἑ~ῐ~θησομαι", _, _;
    future_ind_pass_3: _, "ἑ~ῐ~θησεσθον", _;
    future_opt_act_1: "ἑ~ῐ~σοιμῐ", _, "ἑ~ῐ~σοιμεν";
    future_opt_act_2: "ἑ~ῐ~σοις", "ἑ~ῐ~σοιτον", _;
    future_opt_act_3: "ἑ~ῐ~σοι", "ἑ~ῐ~σοιτην", _;
    future_opt_mid_1: "ἑ~ῐ~σοιμην", _, "ἑ~ῐ~σοιμεθᾰ";
    future_opt_mid_3: _, "ἑ~ῐ~σοισθην", _;
    aorist_ind_pass_1: "ἑ~ε~θην", _, _;
    aorist_subj_act_1: "ἑ~ῐ~σω", _, _;
    aorist_subj_act_2: "ἑ~ῐ~σῃς", "ἑ~ῐ~σητον", _;
    aorist_subj_mid_1: "ἑ~ῐ~σωμαι", _, _;
    aorist_subj_mid_2: "ἑ~ῐ~σῃ", "ἑ~ῐ~σησθον", _;
    aorist_subj_pass_3: "ἑ~ῐ~θῃ", "ἑ~ῐ~θητον", _;
    aorist_opt_act_1: "ἑ~ῐ~σαιμῐ", _, "ἑ~ῐ~σαιμεν";
    aorist_opt_act_2: "ἑ~ῐ~σειᾰς", _, _;
    aorist_opt_act_3: "ἑ~ῐ~σειε", _, "ἑ~ῐ~σειᾰν";
    aorist_opt_mid_1: "ἑ~ῐ~σαιμην", _, _;
    aorist_imp_act_2: "ἑ~ῐ~σον", _, "ἑ~ῐ~σᾰτε";
    aorist_imp_act_3: _, _, "ἑ~ῐ~σᾰντων";
    aorist_imp_mid_2: "ἑ~ῐ~σαι", "ἑ~ῐ~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἑ~ῐ~σᾰσθων";
    aorist_imp_pass_2: "ἑ~ῐ~θητῐ", "ἑ~ῐ~θητον", "ἑ~ῐ~θητε";
    perfect_ind_act_1: "ἑ~ῐ%ε~κᾰ", _, "ἑ~ῐ%ε~κᾰμεν";
    perfect_ind_act_2: _, "ἑ~ῐ%ε~κᾰτον", _;
    perfect_ind_act_3: _, _, "ἑ~ῐ%ε~κᾱσῐ";
    perfect_ind_mid_1: _, _, "ἑ~ῐ%ε~μεθᾰ";
    perfect_ind_mid_2: "ἑ~ῐ%ε~σαι", "ἑ~ῐ%ε~σθον", _;
    perfect_ind_mid_3: "ἑ~ῐ%ε~ται", _, "ἑ~ῐ%ε~νται";
    perfect_ind_pass_1: "ἑ~ῐ%ε~μαι", _, _;
    perfect_ind_pass_2: _, _, "ἑ~ῐ%ε~σθε";
    perfect_subj_act_1: "ἑ~ῐ%ε~κω", _, _;
    perfect_subj_act_2: "ἑ~ῐ%ε~κῃς", "ἑ~ῐ%ε~κητον", "ἑ~ῐ%ε~κητε";
    perfect_subj_act_3: "ἑ~ῐ%ε~κῃ", _, "ἑ~ῐ%ε~κωσῐ";
    perfect_imp_act_2: "ἑ~ῐ%ε~κε", _, _;
    perfect_imp_mid_2: _, _, "ἑ~ῐ%ε~σθε";
    perfect_imp_mid_3: _, _, "ἑ~ῐ%ε~σθων";
    perfect_imp_pass_2: "ἑ~ῐ%ε~σο", _, "ἑ~ῐ%ε~σθε";
    perfect_imp_pass_3: "ἑ~ῐ%ε~σθω", "ἑ~ῐ%ε~σθων", "ἑ~ῐ%ε~σθων";
    pluperfect_act_1: "ἑ~ε%ε~κειν", _, _;
    pluperfect_mid_1: "ἑ~ε%ε~μην", _, _;
    pluperfect_mid_2: "ἑ~ε%ε~σο", "ἑ~ε%ε~σθον", _;
    pluperfect_mid_3: "ἑ~ε%ε~το", _, "ἑ~ε%ε~ντο";
    pluperfect_pass_1: _, _, "ἑ~ε%ε~μεθᾰ";
    pluperfect_pass_2: _, _, "ἑ~ε%ε~σθε";
    pluperfect_pass_3: _, "ἑ~ε%ε~σθην", _;
    infinitive_pres: "ἑ~ῐ~ειν", _, "ἑ~ῐ~εσθαι";
    infinitive_fut: "ἑ~ῐ~σειν", _, _;
    infinitive_aor: "ἑ~ῐ~σαι", _, _;
    infinitive_perf: _, "ἑ~ῐ%ε~σθαι", _;
    participle_pres: "ἑ~ῐ~ων", _, "ἑ~ῐ~ομενος";
    participle_aor: "ἑ~ῐ~σᾱς", "ἑ~ῐ~σᾰμενος", _;
    participle_perf: "ἑ~ῐ%ε~κως", _, "ἑ~ῐ%ε~μενος";
    ),
    // Ερεω: 2 words (ερεω, ετεω). Like Εἰμῐ, except in 80 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ερεω", 2, "ε~εω", like Εἰμῐ;
    ind_act_1: "ἑ~ω", _, "ἑ~ουμεν";
    ind_act_2: "ἑ~εις", "ἑ~ειτον", _;
    ind_act_3: "ἑ~ει", _, "ἑ~ουσῐ|ἑ~ουσῐν";
    ind_mid_1: _, _, "ἑ~ουμεθᾰ";
    ind_mid_2: "ἑ~ει|ἑ~ῃ", _, _;
    ind_mid_3: _, "ἑ~εισθον", _;
    ind_pass_1: "ἑ~ουμαι", _, _;
    ind_pass_2: _, "ἑ~εισθον", "ἑ~εισθε";
    ind_pass_3: "ἑ~ειται", "ἑ~εισθον", "ἑ~ουνται";
    subj_act_1: "ἑ~ω", _, "ἑ~ωμεν";
    subj_act_2: "ἑ~ῃς", _, _;
    subj_act_3: "ἑ~ῃ", "ἑ~ητον", "ἑ~ωσῐ|ἑ~ωσῐν";
    subj_mid_1: _, _, "ἑ~ωμεθᾰ";
    subj_mid_2: _, "ἑ~ησθον", _;
    subj_mid_3: "ἑ~ηται", _, "ἑ~ωνται";
    subj_pass_1: "ἑ~ωμαι", _, _;
    subj_pass_2: _, _, "ἑ~ησθε";
    opt_act_1: "ἑ~οιην|ἑ~οιμῐ", _, "ἑ~οιμεν|ἑ~οιημεν";
    opt_act_2: "ἑ~οιης|ἑ~οις", "ἑ~οιτον|ἑ~οιητον", "ἑ~οιτε|ἑ~οιητε";
    opt_act_3: "ἑ~οιη|ἑ~οι", "ἑ~οιτην|ἑ~οιητην", "ἑ~οιεν|ἑ~οιησᾰν";
    opt_mid_1: "ἑ~οιμην", _, "ἑ~οιμεθᾰ";
    opt_mid_2: "ἑ~οιο", _, "ἑ~οισθε";
    opt_mid_3: "ἑ~οιτο", "ἑ~οισθην", "ἑ~οιντο";
    opt_pass_2: _, "ἑ~οισθον", _;
    imp_act_2: "ἑ~ει", _, "ἑ~ειτε";
    imp_act_3: _, _, "ἑ~ουντων";
    imp_mid_2: "ἑ~ου", "ἑ~εισθον", "ἑ~εισθε";
    imp_mid_3: _, _, "ἑ~εισθων";
    imp_pass_2: _, "ἑ~εισθον", "ἑ~εισθε";
    imp_pass_3: "ἑ~εισθω", "ἑ~εισθων", "ἑ~εισθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_1: "ἑ~ησω", _, "ἑ~ησομεν";
    future_ind_act_2: "ἑ~ησεις", "ἑ~ησετον", _;
    future_ind_act_3: _, _, "ἑ~ησουσῐ";
    future_ind_mid_1: "ἑ~ησομαι", _, _;
    future_ind_mid_2: "ἑ~ησῃ", "ἑ~ησεσθον", _;
    future_ind_mid_3: "ἑ~ησεται", _, _;
    future_opt_act_1: "ἑ~ησοιμῐ", _, "ἑ~ησοιμεν";
    future_opt_act_2: "ἑ~ησοις", "ἑ~ησοιτον", _;
    future_opt_act_3: "ἑ~ησοι", "ἑ~ησοιτην", _;
    future_opt_mid_1: "ἑ~ησοιμην", _, "ἑ~ησοιμεθᾰ";
    future_opt_mid_3: _, "ἑ~ησοισθην", _;
    infinitive_pres: "ἑ~ειν", _, "ἑ~εισθαι";
    infinitive_fut: "ἑ~ησειν", _, _;
    participle_pres: "ἑ~ων", _, "ἑ~ουμενος";
    ),
    // Ἐδω: 1 words (ἐδω). Like Γεγονοι, except in 62 cells: ind_act_1 sg, future_ind_mid_1 sg, future_ind_mid_2 sg, future_ind_mid_2 du, future_ind_mid_3 sg, future_opt_mid_1 sg, ….
    conjugation!("ἐδω", 1, "~ω", like Γεγονοι;
    stem pluperfect_act: "~ηδοκ- like εὐλογεω";
    ind_act_1: "~ω", _, _;
    future_ind_mid_1: "~ομαι", _, _;
    future_ind_mid_2: "~ῃ|~ει", "~εσθον", _;
    future_ind_mid_3: "~εται", _, _;
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_3: _, "~οισθην", _;
    perfect_ind_act_1: "~ηδοκᾰ|~ηδᾰ", _, "~ηδοκᾰμεν|~ηδᾰμεν";
    perfect_ind_act_2: "~ηδοκᾰς|~ηδᾰς", "~ηδοκᾰτον|~ηδᾰτον", "~ηδοκᾰτε|~ηδᾰτε";
    perfect_ind_act_3: "~ηδοκε|~ηδοκεν|~ηδε|~ηδεν", _, "~ηδοκᾱσῐ|~ηδοκᾱσῐν|~ηδᾱσῐ|~ηδᾱσῐν";
    perfect_ind_mid_1: _, _, "~ηδομεθᾰ";
    perfect_ind_mid_2: "~ηδοσαι", "~ηδοσθον", _;
    perfect_ind_mid_3: "~ηδοται", _, "~ηδονται";
    perfect_ind_pass_1: "~ηδομαι", _, _;
    perfect_ind_pass_2: _, _, "~ηδοσθε";
    perfect_subj_act_1: "~ηδοκω|~ηδω", _, "~ηδοκωμεν|~ηδωμεν";
    perfect_subj_act_2: "~ηδοκῃς|~ηδῃς", "~ηδοκητον|~ηδητον", "~ηδοκητε|~ηδητε";
    perfect_subj_act_3: "~ηδοκῃ|~ηδῃ", _, "~ηδοκωσῐ|~ηδοκωσῐν|~ηδωσῐ|~ηδωσῐν";
    perfect_opt_act_1: "~ηδοκοιμῐ|~ηδοκοιην|~ηδοιμῐ|~ηδοιην", _, "~ηδοκοιμεν|~ηδοιμεν";
    perfect_opt_act_2: "~ηδοκοις|~ηδοκοιης|~ηδοις|~ηδοιης", "~ηδοκοιτον|~ηδοιτον", "~ηδοκοιτε|~ηδοιτε";
    perfect_opt_act_3: "~ηδοκοι|~ηδοκοιη|~ηδοι|~ηδοιη", "~ηδοκοιτην|~ηδοιτην", "~ηδοκοιεν|~ηδοιεν";
    perfect_imp_act_2: "~ηδοκε|~ηδε", "~ηδοκετον|~ηδετον", "~ηδοκετε|~ηδετε";
    perfect_imp_act_3: "~ηδοκετω|~ηδετω", "~ηδοκετων|~ηδετων", "~ηδοκοντων|~ηδοντων";
    perfect_imp_mid_2: _, _, "~ηδοσθε";
    perfect_imp_mid_3: _, _, "~ηδοσθων";
    perfect_imp_pass_2: "~ηδοσο", _, "~ηδοσθε";
    perfect_imp_pass_3: "~ηδοσθω", "~ηδοσθων", "~ηδοσθων";
    pluperfect_mid_1: "~ηδομην", _, _;
    pluperfect_mid_2: "~ηδοσο", "~ηδοσθον", _;
    pluperfect_mid_3: "~ηδοτο", _, "~ηδοντο";
    pluperfect_pass_1: _, _, "~ηδομεθᾰ";
    pluperfect_pass_2: _, _, "~ηδοσθε";
    pluperfect_pass_3: _, "~ηδοσθην", _;
    infinitive_perf: "~ηδοκεναι|~ηδεναι", "~ηδοσθαι", _;
    participle_perf: "~ηδοκως|~ηδως", _, "~ηδομενος";
    ),
    // Αἰσθᾰνομαι: 1 words (αἰσθᾰνομαι). Like Οἰομαι, except in 124 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("αἰσθᾰνομαι", 1, "αἰ~ᾰνομαι", like Οἰομαι;
    stem aorist_pass: "~ηθ- like αἰσθᾰνομαι";
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
    opt_mid_2: "αἰ~ανοιο", _, "αἰ~ανοισθε";
    opt_mid_3: "αἰ~ανοιτο", "αἰ~ανοισθην", "αἰ~ανοιντο";
    opt_pass_2: _, "αἰ~ανοισθον", _;
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
    future_opt_mid_1: "αἰ~ησοιμην", _, "αἰ~ησοιμεθᾰ";
    future_opt_mid_3: _, "αἰ~ησοισθην", _;
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
    perfect_ind_mid_1: _, _, "ᾐ~ημεθᾰ";
    perfect_ind_mid_2: "ᾐ~ησαι", "ᾐ~ησθον", _;
    perfect_ind_mid_3: "ᾐ~ηται", _, "ᾐ~ηνται";
    perfect_ind_pass_1: "ᾐ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "ᾐ~ησθε";
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
    // Τρυπαω: 1 words (τρυπαω). Like Ἐκφερω, except in 135 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("τρυπαω", 1, "~υ~αω", like Ἐκφερω;
    stem aorist_ind: "~ῡ~ησ- like ποιεω";
    stem pluperfect_act: "~ῡ~ηκ- like χρημᾰτῐζω";
    ind_act_1: "~ῡ~ω", _, "~ῡ~ωμεν";
    ind_act_2: "~ῡ~ᾳς", "~ῡ~ᾱτον", _;
    ind_act_3: "~ῡ~ᾳ", _, "~ῡ~ωσῐ|~ῡ~ωσῐν";
    ind_mid_1: _, _, "~ῡ~ωμεθᾰ";
    ind_mid_2: "~ῡ~ᾳ", _, _;
    ind_mid_3: _, "~ῡ~ᾱσθον", _;
    ind_pass_1: "~ῡ~ωμαι", _, _;
    ind_pass_2: _, "~ῡ~ᾱσθον", "~ῡ~ᾱσθε";
    ind_pass_3: "~ῡ~ᾱται", "~ῡ~ᾱσθον", "~ῡ~ωνται";
    subj_act_1: "~ῡ~ω", _, "~ῡ~ωμεν";
    subj_act_2: "~ῡ~ᾳς", _, _;
    subj_act_3: "~ῡ~ᾳ", "~ῡ~ᾱτον", "~ῡ~ωσῐ|~ῡ~ωσῐν";
    subj_mid_1: _, _, "~ῡ~ωμεθᾰ";
    subj_mid_2: _, "~ῡ~ᾱσθον", _;
    subj_mid_3: "~ῡ~ᾱται", _, "~ῡ~ωνται";
    subj_pass_1: "~ῡ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῡ~ᾱσθε";
    opt_act_1: "~ῡ~ῳην|~ῡ~ῳμῐ", _, "~ῡ~ῳμεν|~ῡ~ῳημεν";
    opt_act_2: "~ῡ~ῳης|~ῡ~ῳς", "~ῡ~ῳτον|~ῡ~ῳητον", "~ῡ~ῳτε|~ῡ~ῳητε";
    opt_act_3: "~ῡ~ῳη|~ῡ~ῳ", "~ῡ~ῳτην|~ῡ~ῳητην", "~ῡ~ῳεν|~ῡ~ῳησᾰν";
    opt_mid_1: "~ῡ~ῳμην", _, "~ῡ~ῳμεθᾰ";
    opt_mid_2: "~ῡ~ῳο", _, "~ῡ~ῳσθε";
    opt_mid_3: "~ῡ~ῳτο", "~ῡ~ῳσθην", "~ῡ~ῳντο";
    opt_pass_2: _, "~ῡ~ῳσθον", _;
    imp_act_2: "~ῡ~ᾱ", _, "~ῡ~ᾱτε";
    imp_act_3: _, _, "~ῡ~ωντων";
    imp_mid_2: "~ῡ~ω", "~ῡ~ᾱσθον", "~ῡ~ᾱσθε";
    imp_mid_3: _, _, "~ῡ~ᾱσθων";
    imp_pass_2: _, "~ῡ~ᾱσθον", "~ῡ~ᾱσθε";
    imp_pass_3: "~ῡ~ᾱσθω", "~ῡ~ᾱσθων", "~ῡ~ᾱσθων";
    imperfect_act_1: "ἐ~ῡ~ων", _, _;
    imperfect_act_2: "ἐ~ῡ~ᾱς", "ἐ~ῡ~ᾱτον", "ἐ~ῡ~ᾱτε";
    imperfect_act_3: "ἐ~ῡ~ᾱ", _, "ἐ~ῡ~ων";
    imperfect_mid_1: "ἐ~ῡ~ωμην", _, "ἐ~ῡ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ῡ~ω", _, _;
    imperfect_mid_3: "ἐ~ῡ~ᾱτο", _, "ἐ~ῡ~ωντο";
    imperfect_pass_2: _, "ἐ~ῡ~ᾱσθον", "ἐ~ῡ~ᾱσθε";
    imperfect_pass_3: _, "ἐ~ῡ~ᾱσθην", _;
    aorist_subj_act_1: "~ῡ~ησω", _, _;
    aorist_subj_act_2: "~ῡ~ησῃς", "~ῡ~ησητον", _;
    aorist_subj_act_3: _, _, "~ῡ~ησωσῐ|~ῡ~ησωσῐν";
    aorist_subj_mid_1: "~ῡ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ῡ~ησῃ", "~ῡ~ησησθον", _;
    aorist_opt_act_1: "~ῡ~ησαιμῐ", _, "~ῡ~ησαιμεν";
    aorist_opt_act_2: "~ῡ~ησειᾰς|~ῡ~ησαις", _, _;
    aorist_opt_act_3: "~ῡ~ησειε|~ῡ~ησειεν|~ῡ~ησαι", _, "~ῡ~ησειᾰν|~ῡ~ησαιεν";
    aorist_opt_mid_1: "~ῡ~ησαιμην", _, _;
    aorist_imp_act_2: "~ῡ~ησον", _, "~ῡ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῡ~ησᾰντων";
    aorist_imp_mid_2: "~ῡ~ησαι", "~ῡ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῡ~ησᾰσθων";
    perfect_ind_act_1: "%ε~ῡ~ηκᾰ", _, "%ε~ῡ~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~ῡ~ηκᾰτον", _;
    perfect_ind_act_3: "%ε~ῡ~ηκε|%ε~ῡ~ηκεν", _, "%ε~ῡ~ηκᾱσῐ|%ε~ῡ~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "%ε~ῡ~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ῡ~ησαι", "%ε~ῡ~ησθον", _;
    perfect_ind_mid_3: "%ε~ῡ~ηται", _, "%ε~ῡ~ηνται";
    perfect_ind_pass_1: "%ε~ῡ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~ῡ~ησθε";
    perfect_subj_act_1: "%ε~ῡ~ηκω", _, _;
    perfect_subj_act_2: "%ε~ῡ~ηκῃς", "%ε~ῡ~ηκητον", "%ε~ῡ~ηκητε";
    perfect_subj_act_3: "%ε~ῡ~ηκῃ", _, "%ε~ῡ~ηκωσῐ|%ε~ῡ~ηκωσῐν";
    perfect_opt_act_1: "%ε~ῡ~ηκοιμῐ|%ε~ῡ~ηκοιην", _, _;
    perfect_opt_act_2: "%ε~ῡ~ηκοις|%ε~ῡ~ηκοιης", _, _;
    perfect_opt_act_3: "%ε~ῡ~ηκοι|%ε~ῡ~ηκοιη", _, _;
    perfect_imp_act_2: "%ε~ῡ~ηκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~ῡ~ησθε";
    perfect_imp_mid_3: _, _, "%ε~ῡ~ησθων";
    perfect_imp_pass_2: "%ε~ῡ~ησο", _, "%ε~ῡ~ησθε";
    perfect_imp_pass_3: "%ε~ῡ~ησθω", "%ε~ῡ~ησθων", "%ε~ῡ~ησθων";
    pluperfect_mid_1: "ἐ%ε~ῡ~ημην", _, _;
    pluperfect_mid_2: "ἐ%ε~ῡ~ησο", "ἐ%ε~ῡ~ησθον", _;
    pluperfect_mid_3: "ἐ%ε~ῡ~ητο", _, "ἐ%ε~ῡ~ηντο";
    pluperfect_pass_1: _, _, "ἐ%ε~ῡ~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ῡ~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~ῡ~ησθην", _;
    infinitive_pres: "~ῡ~ᾱν", _, "~ῡ~ᾱσθαι";
    infinitive_aor: "~ῡ~ησαι", _, _;
    infinitive_perf: _, "%ε~ῡ~ησθαι", _;
    participle_pres: "~ῡ~ων", _, "~ῡ~ωμενος";
    participle_aor: "~ῡ~ησᾱς", "~ῡ~ησᾰμενος", _;
    participle_perf: "%ε~ῡ~ηκως", _, "%ε~ῡ~ημενος";
    ),
    // Μειδᾰω: 1 words (μειδᾰω). Like Εἰμῐ, except in 39 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("μειδᾰω", 1, "~ᾰω", like Εἰμῐ;
    stem imperfect: "~ι- like μειδᾰω";
    stem aorist_ind: "~ιᾱσ- like χᾰριζω";
    ind_act_1: "~ιω", _, "~ιωμεν";
    ind_act_2: "~ιᾳς", "~ιᾱτον", _;
    ind_act_3: "~ιᾳ", _, "~ιωσῐ|~ιωσῐν";
    subj_act_1: "~ιω", _, "~ιωμεν";
    subj_act_2: "~ιᾳς", "~ιᾱτον", _;
    subj_act_3: "~ιᾳ", _, "~ιωσῐ|~ιωσῐν";
    opt_act_1: "~ιῳην|~ιῳμῐ", _, "~ιῳμεν|~ιῳημεν";
    opt_act_2: "~ιῳης|~ιῳς", "~ιῳτον|~ιῳητον", "~ιῳτε|~ιῳητε";
    opt_act_3: "~ιῳη|~ιῳ", "~ιῳτην|~ιῳητην", "~ιῳεν|~ιῳησᾰν";
    imp_act_2: "~ιᾱ", _, "~ιᾱτε";
    imp_act_3: _, _, "~ιωντων";
    aorist_subj_act_1: "~ιᾱσω", _, _;
    aorist_subj_act_2: "~ιᾱσῃς", "~ιᾱσητον", _;
    aorist_subj_act_3: _, _, "~ιᾱσωσῐ|~ιᾱσωσῐν";
    aorist_opt_act_1: "~ιᾱσαιμῐ", _, "~ιᾱσαιμεν";
    aorist_opt_act_2: "~ιᾱσειᾰς|~ιᾱσαις", _, _;
    aorist_opt_act_3: "~ιᾱσειε|~ιᾱσειεν|~ιᾱσαι", _, "~ιᾱσειᾰν|~ιᾱσαιεν";
    aorist_imp_act_2: "~ιᾱσον", _, "~ιᾱσᾰτε";
    aorist_imp_act_3: _, _, "~ιᾱσᾰντων";
    infinitive_pres: "~ιᾱν", _, _;
    infinitive_aor: "~ιᾱσαι", _, _;
    participle_pres: "~ιων", _, _;
    participle_aor: "~ιᾱσᾱς", _, _;
    ),
    // Ἀνᾰχρονῐζομαι: 1 words (ἀνᾰχρονῐζομαι). Like Ἐπευχομαι, except in 84 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἀνᾰχρονῐζομαι", 1, "~ᾰ~ζομαι", like Ἐπευχομαι;
    stem future_pass: "~α~σθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ε~σ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ε~σθην- like εὐλογεω";
    stem aorist_pass: "~α~σθ- like ποιεω";
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
    opt_mid_2: "~α~ζοιο", _, "~α~ζοισθε";
    opt_mid_3: "~α~ζοιτο", "~α~ζοισθην", "~α~ζοιντο";
    opt_pass_2: _, "~α~ζοισθον", _;
    imp_mid_2: "~α~ζου", "~α~ζεσθον", "~α~ζεσθε";
    imp_mid_3: "~α~ζεσθω", _, "~α~ζεσθων";
    imp_pass_2: _, "~α~ζεσθον", "~α~ζεσθε";
    imp_pass_3: _, "~α~ζεσθων", "~α~ζεσθων";
    imperfect_mid_1: "~ε~ζομην", _, "~ε~ζομεθᾰ";
    imperfect_mid_2: "~ε~ζου", "~ε~ζεσθον", _;
    imperfect_mid_3: "~ε~ζετο", _, "~ε~ζοντο";
    imperfect_pass_2: _, _, "~ε~ζεσθε";
    imperfect_pass_3: _, "~ε~ζεσθην", _;
    future_ind_mid_1: "~α~ουμαι", _, _;
    future_ind_mid_2: "~α~ει|~α~ῃ", "~α~εισθον", _;
    future_ind_mid_3: "~α~ειται", _, _;
    future_opt_mid_1: "~α~οιμην", _, "~α~οιμεθᾰ";
    future_opt_mid_3: _, "~α~οισθην", _;
    aorist_subj_mid_1: "~α~σωμαι", _, _;
    aorist_subj_mid_2: "~α~σῃ", "~α~σησθον", _;
    aorist_opt_mid_1: "~α~σαιμην", _, _;
    aorist_imp_mid_2: "~α~σαι", "~α~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~α~σᾰσθων";
    perfect_ind_mid_1: _, _, "~α%ε~σμεθᾰ";
    perfect_ind_mid_2: "~α%ε~σαι", "~α%ε~σθον", _;
    perfect_ind_mid_3: "~α%ε~σται", _, "~α%ε~δᾰται";
    perfect_ind_pass_1: "~α%ε~σμαι", _, _;
    perfect_ind_pass_2: _, _, "~α%ε~σθε";
    perfect_imp_mid_2: _, _, "~α%ε~σθε";
    perfect_imp_mid_3: _, _, "~α%ε~σθων";
    perfect_imp_pass_2: "~α%ε~σο", _, "~α%ε~σθε";
    perfect_imp_pass_3: "~α%ε~σθω", "~α%ε~σθων", "~α%ε~σθων";
    pluperfect_mid_1: "~ε%ε~σμην", _, _;
    pluperfect_mid_2: "~ε%ε~σο", "~ε%ε~σθον", _;
    pluperfect_mid_3: "~ε%ε~στο", _, "~ε%ε~δᾰτο";
    pluperfect_pass_1: _, _, "~ε%ε~σμεθᾰ";
    pluperfect_pass_2: _, _, "~ε%ε~σθε";
    pluperfect_pass_3: _, "~ε%ε~σθην", _;
    infinitive_pres: _, _, "~α~ζεσθαι";
    infinitive_perf: _, "~α%ε~σθαι", _;
    participle_pres: _, _, "~α~ζομενος";
    participle_aor: _, "~α~σᾰμενος", _;
    participle_perf: _, _, "~α%ε~σμενος";
    ),
    // Σῠμμᾰχομαι: 1 words (σῠμμᾰχομαι). Like Νισσομαι, except in 52 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("σῠμμᾰχομαι", 1, "~ῠμ~ομαι", like Νισσομαι;
    stem aorist_ind: "~υνε~εσ- like μηκᾰομαι";
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
    opt_mid_2: "~υμ~οιο", _, "~υμ~οισθε";
    opt_mid_3: "~υμ~οιτο", "~υμ~οισθην", "~υμ~οιντο";
    opt_pass_2: _, "~υμ~οισθον", _;
    imp_mid_2: "~υμ~ου", "~υμ~εσθον", "~υμ~εσθε";
    imp_mid_3: "~υμ~εσθω", _, "~υμ~εσθων";
    imp_pass_2: _, "~υμ~εσθον", "~υμ~εσθε";
    imp_pass_3: _, "~υμ~εσθων", "~υμ~εσθων";
    future_ind_mid_1: "~υμ~ουμαι", _, _;
    future_ind_mid_2: "~υμ~ει|~υμ~ῃ", "~υμ~εισθον", _;
    future_ind_mid_3: "~υμ~ειται", _, _;
    future_opt_mid_1: "~υμ~οιμην", _, "~υμ~οιμεθᾰ";
    future_opt_mid_3: _, "~υμ~οισθην", _;
    aorist_subj_mid_1: "~υμ~εσωμαι", _, _;
    aorist_subj_mid_2: "~υμ~εσῃ", "~υμ~εσησθον", _;
    aorist_opt_mid_1: "~υμ~εσαιμην", _, _;
    aorist_imp_mid_2: "~υμ~εσαι", "~υμ~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~υμ~εσᾰσθων";
    infinitive_pres: _, _, "~υμ~εσθαι";
    participle_pres: _, _, "~υμ~ομενος";
    participle_aor: _, "~υμ~εσᾰμενος", _;
    ),
    // Κᾰθημαι: 1 words (κᾰθημαι). Like Δῐακειμαι, except in 44 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, subj_mid_1 sg, ….
    conjugation!("κᾰθημαι", 1, "~ᾰ~μαι", like Δῐακειμαι;
    ind_act_1: "~ᾰ~μαι", _, _;
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, _, _;
    ind_mid_3: _, derived, _;
    subj_mid_1: derived, _, derived;
    subj_mid_2: derived, derived, _;
    subj_mid_3: derived, _, derived;
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, _, derived;
    opt_mid_3: derived, derived, derived;
    imp_mid_2: derived, derived, derived;
    imp_mid_3: derived, _, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, _;
    imperfect_mid_3: derived, _, derived;
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_3: _, derived, _;
    pluperfect_mid_1: "ἐ~α~μην|~α~μην", _, "ἐ~α~μεθᾰ|~α~μεθᾰ";
    pluperfect_mid_2: "ἐ~α~σο|~α~σο", "ἐ~α~σθον|~α~σθον", "ἐ~α~σθε|~α~σθε";
    pluperfect_mid_3: "ἐ~α~το|~α~στο|~α~το", "ἐ~α~σθην|~α~σθην", "ἐ~α~ντο|~α~ντο";
    ),
    // Ἐπῐκᾰλεω: 1 words (ἐπῐκᾰλεω). Like Μετᾰκᾰλεω, except in 142 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐπῐκᾰλεω", 1, "~ῐ~ᾰ~εω", like Μετᾰκᾰλεω;
    stem future_pass: "~ι%ε~~ησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ε~α~εσ- like εὐλογεω";
    stem aorist_ind_pass: "~ε~~ηθην- like εὐλογεω";
    stem aorist_pass: "~ι~~ηθ- like ποιεω";
    stem pluperfect_act: "~ε%ε~~ηκ- like εὐλογεω";
    ind_act_1: "~ι~α~ω", _, "~ι~α~ουμεν";
    ind_act_2: "~ι~α~εις", "~ι~α~ειτον", _;
    ind_act_3: "~ι~α~ει", _, "~ι~α~ουσῐ|~ι~α~ουσῐν";
    ind_mid_1: _, _, "~ι~α~ουμεθᾰ";
    ind_mid_2: "~ι~α~ει|~ι~α~ῃ", _, _;
    ind_mid_3: _, "~ι~α~εισθον", _;
    ind_pass_1: "~ι~α~ουμαι", _, _;
    ind_pass_2: _, "~ι~α~εισθον", "~ι~α~εισθε";
    ind_pass_3: "~ι~α~ειται", "~ι~α~εισθον", "~ι~α~ουνται";
    subj_act_1: "~ι~α~ω", _, "~ι~α~ωμεν";
    subj_act_2: "~ι~α~ῃς", _, _;
    subj_act_3: "~ι~α~ῃ", "~ι~α~ητον", "~ι~α~ωσῐ|~ι~α~ωσῐν";
    subj_mid_1: _, _, "~ι~α~ωμεθᾰ";
    subj_mid_2: _, "~ι~α~ησθον", _;
    subj_mid_3: "~ι~α~ηται", _, "~ι~α~ωνται";
    subj_pass_1: "~ι~α~ωμαι", _, _;
    subj_pass_2: _, _, "~ι~α~ησθε";
    opt_act_1: "~ι~α~οιην|~ι~α~οιμῐ", _, "~ι~α~οιμεν|~ι~α~οιημεν";
    opt_act_2: "~ι~α~οιης|~ι~α~οις", "~ι~α~οιτον|~ι~α~οιητον", "~ι~α~οιτε|~ι~α~οιητε";
    opt_act_3: "~ι~α~οιη|~ι~α~οι", "~ι~α~οιτην|~ι~α~οιητην", "~ι~α~οιεν|~ι~α~οιησᾰν";
    opt_mid_1: "~ι~α~οιμην", _, "~ι~α~οιμεθᾰ";
    opt_mid_2: "~ι~α~οιο", _, "~ι~α~οισθε";
    opt_mid_3: "~ι~α~οιτο", "~ι~α~οισθην", "~ι~α~οιντο";
    opt_pass_2: _, "~ι~α~οισθον", _;
    imp_act_2: "~ι~α~ει", _, "~ι~α~ειτε";
    imp_act_3: _, _, "~ι~α~ουντων";
    imp_mid_2: "~ι~α~ου", "~ι~α~εισθον", "~ι~α~εισθε";
    imp_mid_3: _, _, "~ι~α~εισθων";
    imp_pass_2: _, "~ι~α~εισθον", "~ι~α~εισθε";
    imp_pass_3: "~ι~α~εισθω", "~ι~α~εισθων", "~ι~α~εισθων";
    future_ind_act_1: "~ι~α~ω", _, "~ι~α~ουμεν";
    future_ind_act_2: "~ι~α~εις", "~ι~α~ειτον", _;
    future_ind_act_3: _, _, "~ι~α~ουσῐ|~ι~α~ουσῐν";
    future_ind_mid_1: "~ι~α~ουμαι", _, _;
    future_ind_mid_2: "~ι~α~ει|~ι~α~ῃ", "~ι~α~εισθον", _;
    future_ind_mid_3: "~ι~α~ειται", _, _;
    future_opt_act_1: "~ι~α~οιην|~ι~α~οιμῐ", _, "~ι~α~οιμεν|~ι~α~οιημεν";
    future_opt_act_2: "~ι~α~οιης|~ι~α~οις", "~ι~α~οιτον|~ι~α~οιητον", "~ι~α~οιτε|~ι~α~οιητε";
    future_opt_act_3: "~ι~α~οιη|~ι~α~οι", "~ι~α~οιτην|~ι~α~οιητην", "~ι~α~οιεν|~ι~α~οιησᾰν";
    future_opt_mid_1: "~ι~α~οιμην", _, "~ι~α~οιμεθᾰ";
    future_opt_mid_3: _, "~ι~α~οισθην", _;
    aorist_subj_act_1: "~ι~α~εσω", _, _;
    aorist_subj_act_2: "~ι~α~εσῃς", "~ι~α~εσητον", _;
    aorist_subj_act_3: _, _, "~ι~α~εσωσῐ|~ι~α~εσωσῐν";
    aorist_subj_mid_1: "~ι~α~εσωμαι", _, _;
    aorist_subj_mid_2: "~ι~α~εσῃ", "~ι~α~εσησθον", _;
    aorist_opt_act_1: "~ι~α~εσαιμῐ", _, "~ι~α~εσαιμεν";
    aorist_opt_act_2: "~ι~α~εσειᾰς|~ι~α~εσαις", _, _;
    aorist_opt_act_3: "~ι~α~εσειε|~ι~α~εσειεν|~ι~α~εσαι", _, "~ι~α~εσειᾰν|~ι~α~εσαιεν";
    aorist_opt_mid_1: "~ι~α~εσαιμην", _, _;
    aorist_imp_act_2: "~ι~α~εσον", _, "~ι~α~εσᾰτε";
    aorist_imp_act_3: _, _, "~ι~α~εσᾰντων";
    aorist_imp_mid_2: "~ι~α~εσαι", "~ι~α~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ι~α~εσᾰσθων";
    perfect_ind_act_1: "~ι%ε~~ηκᾰ", _, "~ι%ε~~ηκᾰμεν";
    perfect_ind_act_2: _, "~ι%ε~~ηκᾰτον", _;
    perfect_ind_act_3: "~ι%ε~~ηκε|~ι%ε~~ηκεν", _, "~ι%ε~~ηκᾱσῐ|~ι%ε~~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ι%ε~~ημεθᾰ";
    perfect_ind_mid_2: "~ι%ε~~ησαι", "~ι%ε~~ησθον", _;
    perfect_ind_mid_3: "~ι%ε~~ηται", _, "~ι%ε~~ηνται";
    perfect_ind_pass_1: "~ι%ε~~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ι%ε~~ησθε";
    perfect_subj_act_1: "~ι%ε~~ηκω", _, _;
    perfect_subj_act_2: "~ι%ε~~ηκῃς", "~ι%ε~~ηκητον", "~ι%ε~~ηκητε";
    perfect_subj_act_3: "~ι%ε~~ηκῃ", _, "~ι%ε~~ηκωσῐ|~ι%ε~~ηκωσῐν";
    perfect_opt_act_1: "~ι%ε~~ηκοιμῐ|~ι%ε~~ηκοιην", _, _;
    perfect_opt_act_2: "~ι%ε~~ηκοις|~ι%ε~~ηκοιης", _, _;
    perfect_opt_act_3: "~ι%ε~~ηκοι|~ι%ε~~ηκοιη", _, _;
    perfect_imp_act_2: "~ι%ε~~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~ι%ε~~ησθε";
    perfect_imp_mid_3: _, _, "~ι%ε~~ησθων";
    perfect_imp_pass_2: "~ι%ε~~ησο", _, "~ι%ε~~ησθε";
    perfect_imp_pass_3: "~ι%ε~~ησθω", "~ι%ε~~ησθων", "~ι%ε~~ησθων";
    pluperfect_mid_1: "~ε%ε~~ημην", _, _;
    pluperfect_mid_2: "~ε%ε~~ησο", "~ε%ε~~ησθον", _;
    pluperfect_mid_3: "~ε%ε~~ητο", _, "~ε%ε~~ηντο";
    pluperfect_pass_1: _, _, "~ε%ε~~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ε%ε~~ησθε";
    pluperfect_pass_3: _, "~ε%ε~~ησθην", _;
    infinitive_pres: "~ι~α~ειν", _, "~ι~α~εισθαι";
    infinitive_fut: "~ι~α~ειν", _, _;
    infinitive_aor: "~ι~α~εσαι", _, _;
    infinitive_perf: _, "~ι%ε~~ησθαι", _;
    participle_pres: "~ι~α~ων", _, "~ι~α~ουμενος";
    participle_aor: "~ι~α~εσᾱς", "~ι~α~εσᾰμενος", _;
    participle_perf: "~ι%ε~~ηκως", _, "~ι%ε~~ημενος";
    ),
    // Χρη: 1 words (χρη). Like Εἰμῐ, except in 12 cells: ind_act_1 sg, ind_act_3 sg, subj_act_3 sg, opt_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, ….
    conjugation!("χρη", 1, "~η", like Εἰμῐ;
    ind_act_1: "~η", _, _;
    ind_act_3: "~η", _, _;
    subj_act_3: "~ῃ", _, _;
    opt_act_3: "~ειη", _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_3: "~ησει", _, _;
    future_ind_mid_3: "~ησται", _, _;
    future_opt_act_3: "~ησοι", _, _;
    future_opt_mid_3: "~ησοιτο", _, _;
    infinitive_pres: "~ηναι", _, _;
    infinitive_fut: "~ησειν", _, _;
    ),
    // Ἠλθον: 1 words (ἠλθον). Like Ἐπορον, except in 21 cells: ind_act_1 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_3 sg, ….
    conjugation!("ἠλθον", 1, "ἠ~ον", like Ἐπορον;
    ind_act_1: "ἠ~ον", _, _;
    aorist_ind_act_1: "ἠ~ον", _, "ἠ~ομεν";
    aorist_ind_act_2: "ἠ~ες", "ἠ~ετον", _;
    aorist_ind_act_3: "ἠ~ε|ἠ~εν", _, "ἠ~ον";
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
    // Ἀρνυμαι: 1 words (ἀρνυμαι). Like Καινῠμαι, except in 28 cells: ind_act_1 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_3 sg, ….
    conjugation!("ἀρνυμαι", 1, "~νυμαι", like Καινῠμαι;
    stem aorist_ind: "~- like πᾰρᾰγῐγνομαι";
    ind_act_1: "~νυμαι", _, _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_mid_1: "~ωμαι", _, _;
    aorist_subj_mid_2: "~ῃ", "~ησθον", _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, _, derived;
    aorist_opt_mid_1: "~οιμην", _, _;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_mid_2: "~ου", "~εσθον", _;
    aorist_imp_mid_3: _, _, "~εσθων";
    infinitive_aor: derived, _, _;
    participle_aor: derived, "~ομενος", _;
    ),
    // Ἐπεφνον: 1 words (ἐπεφνον). Like Ἐπορον, except in 31 cells: ind_act_1 sg, aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, aorist_opt_act_1 sg, ….
    conjugation!("ἐπεφνον", 1, "ἐ~νον", like Ἐπορον;
    stem future_pass: "~ησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ν- like ἐπεφνον";
    ind_act_1: "ἐ~νον", _, _;
    aorist_subj_act_1: "~νω", _, _;
    aorist_subj_act_2: "~νῃς", "~νητον", _;
    aorist_subj_act_3: _, _, "~νωσῐ|~νωσῐν";
    aorist_opt_act_1: "~νοιμῐ", _, "~νοιμεν";
    aorist_opt_act_2: "~νοις", _, _;
    aorist_opt_act_3: "~νοι", _, "~νοιεν";
    aorist_imp_act_2: "~νε", _, "~νετε";
    aorist_imp_act_3: _, _, "~νοντων";
    perfect_ind_mid_1: _, _, "~αμεθᾰ";
    perfect_ind_mid_2: "~ασαι", "~ασθον", _;
    perfect_ind_mid_3: "~αται", _, "~ανται";
    perfect_ind_pass_1: "~αμαι", _, _;
    perfect_ind_pass_2: _, _, "~ασθε";
    perfect_imp_mid_2: _, _, "~ασθε";
    perfect_imp_mid_3: _, _, "~ασθων";
    perfect_imp_pass_2: "~ασο", _, "~ασθε";
    perfect_imp_pass_3: "~ασθω", "~ασθων", "~ασθων";
    infinitive_aor: "~νειν", _, _;
    infinitive_perf: _, "~ᾱσθαι", _;
    participle_aor: "~νων", _, _;
    participle_perf: _, _, "~αμενος";
    ),
    // Μετεκῑᾰθον: 1 words (μετεκῑᾰθον). Like Κᾰθημαι, except in 23 cells: ind_act_1 sg, aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, aorist_opt_act_1 sg, ….
    conjugation!("μετεκῑᾰθον", 1, "~ε~ον", like Κᾰθημαι;
    stem aorist_ind: "~ε~- like εἰσερχομαι";
    ind_act_1: "~ε~ον", _, _;
    aorist_subj_act_1: "~α~ω", _, _;
    aorist_subj_act_2: "~α~ῃς", "~α~ητον", _;
    aorist_subj_act_3: _, _, "~α~ωσῐ|~α~ωσῐν";
    aorist_opt_act_1: "~α~οιμῐ", _, "~α~οιμεν";
    aorist_opt_act_2: "~α~οις", _, _;
    aorist_opt_act_3: "~α~οι", _, "~α~οιεν";
    aorist_imp_act_2: "~α~ε", _, "~α~ετε";
    aorist_imp_act_3: _, _, "~α~οντων";
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_aor: "~α~ειν", _, _;
    participle_aor: "~α~ων", _, _;
    ),
    // Πεπνῡμαι: 1 words (πεπνῡμαι). Like Εἰμῐ, except in 4 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg.
    conjugation!("πεπνῡμαι", 1, "πε~μαι", like Εἰμῐ;
    stem aorist_ind_pass: "~θην- like ἐχω";
    stem aorist_pass: "~θ- like ποιεω";
    ind_act_1: "πε~μαι", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    ),
    // Περνημῐ: 1 words (περνημῐ). Like Εἰμῐ, except in 23 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("περνημῐ", 1, "~νημῐ", like Εἰμῐ;
    stem future_pass: "~ᾰθησ- like χρημᾰτῐζω";
    ind_act_1: "~νημῐ", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_1: "~ᾰσω", _, "~ᾰσομεν";
    future_ind_act_2: "~ᾰσεις", "~ᾰσετον", _;
    future_ind_act_3: _, _, "~ᾰσουσῐ|~ᾰσουσῐν";
    future_ind_mid_1: "~ᾰσομαι", _, _;
    future_ind_mid_2: "~ᾰσῃ|~ᾰσει", "~ᾰσεσθον", _;
    future_ind_mid_3: "~ᾰσεται", _, _;
    future_opt_act_1: "~ᾰσοιμῐ", _, "~ᾰσοιμεν";
    future_opt_act_2: "~ᾰσοις", "~ᾰσοιτον", _;
    future_opt_act_3: "~ᾰσοι", "~ᾰσοιτην", _;
    future_opt_mid_1: "~ᾰσοιμην", _, "~ᾰσοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰσοισθην", _;
    infinitive_fut: "~ᾰᾱν|~ᾱμεν|~ᾱμεναι|~ᾰσμεν|~ᾰσμεναι", _, _;
    ),
    // Ἐπῐστᾰμαι: 1 words (ἐπῐστᾰμαι). Like Δῐακειμαι, except in 36 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, subj_mid_1 sg, ….
    conjugation!("ἐπῐστᾰμαι", 1, "ἐ~ῐ~ᾰμαι", like Δῐακειμαι;
    stem aorist_ind_pass: "~ι~ηθην- like ἀθῡρω";
    stem aorist_pass: "~ι~ηθ- like ἐλπῐζω";
    ind_act_1: "ἐ~ῐ~ᾰμαι", _, _;
    ind_mid_1: "ἐ~ι~ᾰμαι", _, "ἐ~ι~ᾰμεθᾰ";
    ind_mid_2: "ἐ~ι~ᾰσαι|ἐ~ι~ᾳ|ἐ~ι~ῃ", _, _;
    ind_mid_3: _, "ἐ~ι~ᾰσθον", _;
    subj_mid_1: "ἐ~ι~ωμαι|ἐ~ι~ωμαι", _, "ἐ~ι~ωμεθᾰ";
    subj_mid_2: "ἐ~ι~ῃ", "ἐ~ι~ησθον", _;
    subj_mid_3: "ἐ~ι~ηται", _, "ἐ~ι~ωνται";
    opt_mid_1: "ἐ~ι~αιμην", _, "ἐ~ι~αιμεθᾰ";
    opt_mid_2: "ἐ~ι~αιο", _, "ἐ~ι~αισθε";
    opt_mid_3: "ἐ~ι~αιτο", "ἐ~ι~αισθην", "ἐ~ι~αιντο";
    imp_mid_2: "ἐ~ι~ᾰσο|ἐ~ι~ω", "ἐ~ι~ᾰσθον", "ἐ~ι~ᾰσθε";
    imp_mid_3: "ἐ~ι~ᾰσθω", _, "ἐ~ι~ᾰσθων";
    imperfect_mid_1: "ἠ~ι~ᾰμην", _, "ἠ~ι~ᾰμεθᾰ";
    imperfect_mid_2: "ἠ~ῐ~ω|ἠ~ῐ~ᾰσο", "ἠ~ι~ᾰσθον", _;
    imperfect_mid_3: "ἠ~ι~ᾰτο", _, "ἠ~ι~ᾰντο";
    future_ind_mid_1: "ἐ~ι~ησομαι", _, _;
    future_ind_mid_2: "ἐ~ι~ησῃ|ἐ~ι~ησει", "ἐ~ι~ησεσθον", _;
    future_ind_mid_3: "ἐ~ι~ησεται", _, _;
    future_opt_mid_1: "ἐ~ι~ησοιμην", _, "ἐ~ι~ησοιμεθᾰ";
    future_opt_mid_3: _, "ἐ~ι~ησοισθην", _;
    ),
    // Ἀπαυραω: 1 words (ἀπαυραω). Like Μετεκῑᾰθον, except in 56 cells: ind_act_1 sg, imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("ἀπαυραω", 1, "~α~αω", like Μετεκῑᾰθον;
    stem aorist_ind: "~ο~- like εὐλογεω";
    ind_act_1: "~α~αω", _, _;
    imperfect_act_1: "~η~ων", _, _;
    imperfect_act_2: "~η~ᾱς", "~η~ᾱτον", "~η~ᾱτε";
    imperfect_act_3: "~η~ᾱ", _, "~η~ων";
    imperfect_mid_1: "~η~ωμην", _, "~η~ωμεθᾰ";
    imperfect_mid_2: "~η~ω", _, _;
    imperfect_mid_3: "~η~ᾱτο", _, "~η~ωντο";
    imperfect_pass_2: _, "~η~ᾱσθον", "~η~ᾱσθε";
    imperfect_pass_3: _, "~η~ᾱσθην", _;
    future_ind_act_1: "~ο~ησω", _, "~ο~ησομεν";
    future_ind_act_2: "~ο~ησεις", "~ο~ησετον", _;
    future_ind_act_3: _, _, "~ο~ησουσῐ|~ο~ησουσῐν";
    future_ind_mid_1: "~ο~ησομαι", _, _;
    future_ind_mid_2: "~ο~ησῃ|~ο~ησει", "~ο~ησεσθον", _;
    future_ind_mid_3: "~ο~ησεται", _, _;
    future_opt_act_1: "~ο~ησοιμῐ", _, "~ο~ησοιμεν";
    future_opt_act_2: "~ο~ησοις", "~ο~ησοιτον", _;
    future_opt_act_3: "~ο~ησοι", "~ο~ησοιτην", _;
    future_opt_mid_1: "~ο~ησοιμην", _, "~ο~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ο~ησοισθην", _;
    aorist_subj_act_1: "~ο~ω", _, _;
    aorist_subj_act_2: "~ο~ῃς", "~ο~ητον", _;
    aorist_subj_act_3: _, _, "~ο~ωσῐ|~ο~ωσῐν";
    aorist_subj_mid_1: "~ο~ωμαι", _, _;
    aorist_subj_mid_2: "~ο~ῃ", "~ο~ησθον", _;
    aorist_opt_act_1: "~ο~αιμῐ", _, "~ο~αιμεν";
    aorist_opt_act_2: "~ο~ειᾰς|~ο~αις", _, _;
    aorist_opt_act_3: "~ο~ειε|~ο~ειεν|~ο~αι", _, "~ο~ειᾰν|~ο~αιεν";
    aorist_opt_mid_1: "~ο~αιμην", _, _;
    aorist_imp_act_2: "~ο~ον", _, "~ο~ᾰτε";
    aorist_imp_act_3: _, _, "~ο~ᾰντων";
    aorist_imp_mid_2: "~ο~αι", "~ο~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ο~ᾰσθων";
    infinitive_fut: "~ο~ησειν", _, _;
    infinitive_aor: "~ο~αι", _, _;
    participle_aor: "~ο~ᾱς", "~ο~ᾰμενος", _;
    ),
    // Προκειμαι: 1 words (προκειμαι). Like Δῐακειμαι, except in 38 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, subj_mid_1 sg, ….
    conjugation!("προκειμαι", 1, "~~ειμαι", like Δῐακειμαι;
    ind_act_1: "~~ειμαι", _, _;
    ind_mid_1: "~~ειμαι", _, "~~ειμεθα";
    ind_mid_2: "~~εισαι", _, _;
    ind_mid_3: _, "~~εισθον", _;
    subj_mid_1: "~~ωμαι", _, "~~ωμεθᾰ";
    subj_mid_2: "~~εῃ", "~~εησθον", _;
    subj_mid_3: "~~εηται", _, "~~εωνται|~~ειωνται";
    opt_mid_1: "~~οιμην", _, "~~οιμεθᾰ";
    opt_mid_2: "~~οιο", _, "~~οισθε";
    opt_mid_3: "~~οιτο", "~~οισθην", "~~οιντο";
    imp_mid_2: "~~εισο", "~~εισθον", "~~εισθον";
    imp_mid_3: "~~εισθω", _, "~~εισθων";
    imperfect_mid_1: "~ε~ειμην|~υ~ειμην", _, "~ε~ειμεθα|~υ~ειμεθα";
    imperfect_mid_2: "~ε~εισο|~υ~εισο", "~ε~εισθον|~υ~εισθον", "~ε~εισθε|~υ~εισθε";
    imperfect_mid_3: "~ε~ειτο|~υ~ειτο", "~ε~εισθην|~υ~εισθην", "~ε~εατο|~υ~εατο";
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_3: _, derived, _;
    ),
    // Ἐπῐπειθομαι: 1 words (ἐπῐπειθομαι). Like Δῐακειμαι, except in 30 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, subj_mid_1 sg, ….
    conjugation!("ἐπῐπειθομαι", 1, "~ῐ~θομαι", like Δῐακειμαι;
    stem imperfect: "~ε~θ- like ὑποκοριζομαι";
    ind_act_1: "~ῐ~θομαι", _, _;
    ind_mid_1: "~ι~θομαι", _, "~ι~θομεθᾰ";
    ind_mid_2: "~ι~θῃ|~ι~θει", _, _;
    ind_mid_3: _, "~ι~θεσθον", _;
    subj_mid_1: "~ι~θωμαι", _, "~ι~θωμεθᾰ";
    subj_mid_2: "~ι~θῃ", "~ι~θησθον", _;
    subj_mid_3: "~ι~θηται", _, "~ι~θωνται";
    opt_mid_1: "~ι~θοιμην", _, "~ι~θοιμεθᾰ";
    opt_mid_2: "~ι~θοιο", _, "~ι~θοισθε";
    opt_mid_3: "~ι~θοιτο", "~ι~θοισθην", "~ι~θοιντο";
    imp_mid_2: "~ι~θου", "~ι~θεσθον", "~ι~θεσθε";
    imp_mid_3: "~ι~θεσθω", _, "~ι~θεσθων";
    future_ind_mid_1: "~ι~σομαι", _, _;
    future_ind_mid_2: "~ι~σῃ|~ι~σει", "~ι~σεσθον", _;
    future_ind_mid_3: "~ι~σεται", _, _;
    future_opt_mid_1: "~ι~σοιμην", _, "~ι~σοιμεθᾰ";
    future_opt_mid_3: _, "~ι~σοισθην", _;
    ),
    // Ἐπισευω: 1 words (ἐπισευω). Like Κᾰθημαι, except in 28 cells: ind_act_1 sg, perfect_ind_mid_1 pl, perfect_ind_mid_2 sg, perfect_ind_mid_2 du, perfect_ind_mid_3 sg, perfect_ind_mid_3 pl, ….
    conjugation!("ἐπισευω", 1, "~ι~ευω", like Κᾰθημαι;
    ind_act_1: "~ι~ευω", _, _;
    perfect_ind_mid_1: _, _, "~ι~ῠμεθᾰ";
    perfect_ind_mid_2: "~ι~ῠσαι", "~ι~ῠσθον", _;
    perfect_ind_mid_3: "~ι~ῠται", _, "~ι~ῠνται";
    perfect_ind_pass_1: "~ι~ῠμαι", _, _;
    perfect_ind_pass_2: _, _, "~ι~ῠσθε";
    perfect_imp_mid_2: _, _, "~ι~ῠσθε";
    perfect_imp_mid_3: _, _, "~ι~ῠσθων";
    perfect_imp_pass_2: "~ι~ῠσο", _, "~ι~ῠσθε";
    perfect_imp_pass_3: "~ι~ῠσθω", "~ι~ῠσθων", "~ι~ῠσθων";
    pluperfect_mid_1: "~ε~ῠμην", _, derived;
    pluperfect_mid_2: "~ε~ῠσο", "~ε~ῠσθον", derived;
    pluperfect_mid_3: "~ε~ῠτο", derived, "~ε~ῠντο";
    pluperfect_pass_1: _, _, "~ε~ῠμεθᾰ";
    pluperfect_pass_2: _, _, "~ε~ῠσθε";
    pluperfect_pass_3: _, "~ε~ῠσθην", _;
    infinitive_perf: _, "~ι~ῠσθαι", _;
    participle_perf: _, "~ι~υμενος", _;
    ),
    // Δειμᾰτοω: 1 words (δειμᾰτοω). Like Κᾰταφρονεω, except in 116 cells: ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("δειμᾰτοω", 1, "~ᾰ~οω", like Κᾰταφρονεω;
    stem future_pass: "~α~ωθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~α~ωσ- like ποιεω";
    stem aorist_ind_pass: "~α~ωθην- like ἐχω";
    stem aorist_pass: "~α~ωθ- like ποιεω";
    stem pluperfect_act: "~α~ωκ- like χρημᾰτῐζω";
    ind_act_2: "~α~οις", "~α~ουτον", _;
    ind_act_3: "~α~οι", _, _;
    ind_mid_2: "~α~οι", _, _;
    ind_mid_3: _, "~α~ουσθον", _;
    ind_pass_2: _, "~α~ουσθον", "~α~ουσθε";
    ind_pass_3: "~α~ουται", "~α~ουσθον", _;
    subj_act_2: "~α~οις", _, _;
    subj_act_3: "~α~οι", "~α~ωτον", _;
    subj_mid_2: _, "~α~ωσθον", _;
    subj_mid_3: "~α~ωται", _, _;
    subj_pass_2: _, _, "~α~ωσθε";
    imp_act_2: "~α~ου", _, "~α~ουτε";
    imp_mid_2: _, "~α~ουσθον", "~α~ουσθε";
    imp_mid_3: _, _, "~α~ουσθων";
    imp_pass_2: _, "~α~ουσθον", "~α~ουσθε";
    imp_pass_3: "~α~ουσθω", "~α~ουσθων", "~α~ουσθων";
    imperfect_act_2: "ἐ~α~ους", "ἐ~α~ουτον", "ἐ~α~ουτε";
    imperfect_act_3: "ἐ~α~ου", _, _;
    imperfect_mid_3: "ἐ~α~ουτο", _, _;
    imperfect_pass_2: _, "ἐ~α~ουσθον", "ἐ~α~ουσθε";
    imperfect_pass_3: _, "ἐ~α~ουσθην", _;
    future_ind_act_1: "~α~ωσω", _, "~α~ωσομεν";
    future_ind_act_2: "~α~ωσεις", "~α~ωσετον", _;
    future_ind_act_3: _, _, "~α~ωσουσῐ|~α~ωσουσῐν";
    future_ind_mid_1: "~α~ωσομαι", _, _;
    future_ind_mid_2: "~α~ωσῃ|~α~ωσει", "~α~ωσεσθον", _;
    future_ind_mid_3: "~α~ωσεται", _, _;
    future_opt_act_1: "~α~ωσοιμῐ", _, "~α~ωσοιμεν";
    future_opt_act_2: "~α~ωσοις", "~α~ωσοιτον", _;
    future_opt_act_3: "~α~ωσοι", "~α~ωσοιτην", _;
    future_opt_mid_1: "~α~ωσοιμην", _, "~α~ωσοιμεθᾰ";
    future_opt_mid_3: _, "~α~ωσοισθην", _;
    aorist_subj_act_1: "~α~ωσω", _, _;
    aorist_subj_act_2: "~α~ωσῃς", "~α~ωσητον", _;
    aorist_subj_act_3: _, _, "~α~ωσωσῐ|~α~ωσωσῐν";
    aorist_subj_mid_1: "~α~ωσωμαι", _, _;
    aorist_subj_mid_2: "~α~ωσῃ", "~α~ωσησθον", _;
    aorist_opt_act_1: "~α~ωσαιμῐ", _, "~α~ωσαιμεν";
    aorist_opt_act_2: "~α~ωσειᾰς|~α~ωσαις", _, _;
    aorist_opt_act_3: "~α~ωσειε|~α~ωσειεν|~α~ωσαι", _, "~α~ωσειᾰν|~α~ωσαιεν";
    aorist_opt_mid_1: "~α~ωσαιμην", _, _;
    aorist_imp_act_2: "~α~ωσον", _, "~α~ωσᾰτε";
    aorist_imp_act_3: _, _, "~α~ωσᾰντων";
    aorist_imp_mid_2: "~α~ωσαι", "~α~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~α~ωσᾰσθων";
    perfect_ind_act_1: "%ε~α~ωκᾰ", _, "%ε~α~ωκᾰμεν";
    perfect_ind_act_2: _, "%ε~α~ωκᾰτον", _;
    perfect_ind_act_3: "%ε~α~ωκε|%ε~α~ωκεν", _, "%ε~α~ωκᾱσῐ|%ε~α~ωκᾱσῐν";
    perfect_ind_mid_1: _, _, "%ε~α~ωμεθᾰ";
    perfect_ind_mid_2: "%ε~α~ωσαι", "%ε~α~ωσθον", _;
    perfect_ind_mid_3: "%ε~α~ωται", _, "%ε~α~ωνται";
    perfect_ind_pass_1: "%ε~α~ωμαι", _, _;
    perfect_ind_pass_2: _, _, "%ε~α~ωσθε";
    perfect_subj_act_1: "%ε~α~ωκω", _, _;
    perfect_subj_act_2: "%ε~α~ωκῃς", "%ε~α~ωκητον", "%ε~α~ωκητε";
    perfect_subj_act_3: "%ε~α~ωκῃ", _, "%ε~α~ωκωσῐ|%ε~α~ωκωσῐν";
    perfect_opt_act_1: "%ε~α~ωκοιμῐ|%ε~α~ωκοιην", _, _;
    perfect_opt_act_2: "%ε~α~ωκοις|%ε~α~ωκοιης", _, _;
    perfect_opt_act_3: "%ε~α~ωκοι|%ε~α~ωκοιη", _, _;
    perfect_imp_act_2: "%ε~α~ωκε", _, _;
    perfect_imp_mid_2: _, _, "%ε~α~ωσθε";
    perfect_imp_mid_3: _, _, "%ε~α~ωσθων";
    perfect_imp_pass_2: "%ε~α~ωσο", _, "%ε~α~ωσθε";
    perfect_imp_pass_3: "%ε~α~ωσθω", "%ε~α~ωσθων", "%ε~α~ωσθων";
    pluperfect_mid_1: "ἐ%ε~α~ωμην", _, _;
    pluperfect_mid_2: "ἐ%ε~α~ωσο", "ἐ%ε~α~ωσθον", _;
    pluperfect_mid_3: "ἐ%ε~α~ωτο", _, "ἐ%ε~α~ωντο";
    pluperfect_pass_1: _, _, "ἐ%ε~α~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~α~ωσθε";
    pluperfect_pass_3: _, "ἐ%ε~α~ωσθην", _;
    infinitive_pres: "~α~ουν", _, "~α~ουσθαι";
    infinitive_fut: "~α~ωσειν", _, _;
    infinitive_aor: "~α~ωσαι", _, _;
    infinitive_perf: _, "%ε~α~ωσθαι", _;
    participle_aor: "~α~ωσᾱς", "~α~ωσᾰμενος", _;
    participle_perf: "%ε~α~ωκως", _, "%ε~α~ωμενος";
    ),
    // Κορυσσω: 1 words (κορυσσω). Like Ἐπορον, except in 51 cells: ind_act_1 sg, imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("κορυσσω", 1, "~υσσω", like Ἐπορον;
    stem aorist_ind: "~ῠσσ- like ποιεω";
    ind_act_1: "~υσσω", _, _;
    imperfect_act_1: "~ῠσσον", _, _;
    imperfect_act_2: "~ῠσσες", "~ῠσσετον", "~ῠσσετε";
    imperfect_act_3: "~ῠσσε|~ῠσσεν", _, "~ῠσσον";
    imperfect_mid_1: "~ῠσσομην", _, "~ῠσσομεθᾰ";
    imperfect_mid_2: "~ῠσσου", _, _;
    imperfect_mid_3: "~ῠσσετο", _, "~ῠσσοντο";
    imperfect_pass_2: _, "~ῠσσεσθον", "~ῠσσεσθε";
    imperfect_pass_3: _, "~ῠσσεσθην", _;
    aorist_subj_act_1: "~ῠσσω", _, _;
    aorist_subj_act_2: "~ῠσσῃς", "~ῠσσητον", _;
    aorist_subj_act_3: _, _, "~ῠσσωσῐ|~ῠσσωσῐν";
    aorist_subj_mid_1: "~ῠσσωμαι", _, _;
    aorist_subj_mid_2: "~ῠσσῃ", "~ῠσσησθον", _;
    aorist_opt_act_1: "~ῠσσαιμῐ", _, "~ῠσσαιμεν";
    aorist_opt_act_2: "~ῠσσειᾰς|~ῠσσαις", _, _;
    aorist_opt_act_3: "~ῠσσειε|~ῠσσειεν|~ῠσσαι", _, "~ῠσσειᾰν|~ῠσσαιεν";
    aorist_opt_mid_1: "~ῠσσαιμην", _, _;
    aorist_imp_act_2: "~ῠσσον", _, "~ῠσσᾰτε";
    aorist_imp_act_3: _, _, "~ῠσσᾰντων";
    aorist_imp_mid_2: "~ῠσσαι", "~ῠσσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῠσσᾰσθων";
    perfect_ind_pass_1: "%ε~υθμαι", _, "%ε~υθμεθᾰ";
    perfect_ind_pass_2: "%ε~ῠσαι", "%ε~ῠσθον", "%ε~ῠσθε";
    perfect_ind_pass_3: "%ε~ῠσται", "%ε~ῠσθον", "%ε~ῠθᾰται";
    perfect_imp_pass_2: "%ε~ῠσο", "%ε~ῠσθον", "%ε~ῠσθε";
    perfect_imp_pass_3: "%ε~ῠσθω", "%ε~ῠσθων", "%ε~ῠσθων";
    infinitive_aor: "~ῠσσαι", _, _;
    participle_aor: "~ῠσσᾱς", "~ῠσσᾰμενος", _;
    ),
    // Κεκᾰδον: 1 words (κεκᾰδον). Like Μετεκῑᾰθον, except in 28 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_3 pl, ….
    conjugation!("κεκᾰδον", 1, "~ᾰ~ον", like Μετεκῑᾰθον;
    stem aorist_ind: "~ᾰ~- like πᾰρεχω";
    ind_act_1: "~ᾰ~ον", _, _;
    future_ind_act_1: "~α~ησω", _, "~α~ησομεν";
    future_ind_act_2: "~α~ησεις", "~α~ησετον", _;
    future_ind_act_3: _, _, "~α~ησουσῐ|~α~ησουσῐν";
    aorist_subj_act_1: "~ᾰ~ω", _, _;
    aorist_subj_act_2: "~ᾰ~ῃς", "~ᾰ~ητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ῃ", "~ᾰ~ησθον", _;
    aorist_opt_act_1: "~ᾰ~οιμῐ", _, "~ᾰ~οιμεν";
    aorist_opt_act_2: "~ᾰ~οις", _, _;
    aorist_opt_act_3: "~ᾰ~οι", _, "~ᾰ~οιεν";
    aorist_opt_mid_1: "~ᾰ~οιμην", _, _;
    aorist_imp_act_2: "~ᾰ~ε", _, "~ᾰ~ετε";
    aorist_imp_act_3: _, _, "~ᾰ~οντων";
    aorist_imp_mid_2: "~ᾰ~ου", "~ᾰ~εσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~εσθων";
    infinitive_aor: "~ᾰ~ειν", _, _;
    participle_aor: "~ᾰ~ων", "~ᾰ~ομενος", _;
    ),
    // Κᾰτᾰκλαω: 1 words (κᾰτᾰκλαω). Like Ξυνναυμᾰχεω, except in 125 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("κᾰτᾰκλαω", 1, "~ᾰ~ᾰ~αω", like Ξυνναυμᾰχεω;
    stem aorist_ind: "~ᾰ~ε~ᾰσ- like εὐλογεω";
    stem aorist_ind_pass: "~α~ε~ασθην- like εὐλογεω";
    stem aorist_pass: "~ᾰ~α~ασθ- like ποιεω";
    ind_act_1: "~ᾰ~α~ω", _, "~ᾰ~α~ωμεν";
    ind_act_2: "~ᾰ~α~ᾳς", "~ᾰ~α~ᾱτον", _;
    ind_act_3: "~ᾰ~α~ᾳ", _, "~ᾰ~α~ωσῐ|~ᾰ~α~ωσῐν";
    ind_mid_1: _, _, "~ᾰ~α~ωμεθᾰ";
    ind_mid_2: "~ᾰ~α~ᾳ", _, _;
    ind_mid_3: _, "~ᾰ~α~ᾱσθον", _;
    ind_pass_1: "~ᾰ~α~ωμαι", _, _;
    ind_pass_2: _, "~ᾰ~α~ᾱσθον", "~ᾰ~α~ᾱσθε";
    ind_pass_3: "~ᾰ~α~ᾱται", "~ᾰ~α~ᾱσθον", "~ᾰ~α~ωνται";
    subj_act_1: "~ᾰ~α~ω", _, "~ᾰ~α~ωμεν";
    subj_act_2: "~ᾰ~α~ᾳς", derived, _;
    subj_act_3: "~ᾰ~α~ᾳ", "~ᾰ~α~ᾱτον", "~ᾰ~α~ωσῐ|~ᾰ~α~ωσῐν";
    subj_mid_1: _, _, "~ᾰ~α~ωμεθᾰ";
    subj_mid_2: _, "~ᾰ~α~ᾱσθον", _;
    subj_mid_3: "~ᾰ~α~ᾱται", _, "~ᾰ~α~ωνται";
    subj_pass_1: "~ᾰ~α~ωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~α~ᾱσθε";
    opt_act_1: "~ᾰ~α~ῳην|~ᾰ~α~ῳμῐ", _, "~ᾰ~α~ῳμεν|~ᾰ~α~ῳημεν";
    opt_act_2: "~ᾰ~α~ῳης|~ᾰ~α~ῳς", "~ᾰ~α~ῳτον|~ᾰ~α~ῳητον", "~ᾰ~α~ῳτε|~ᾰ~α~ῳητε";
    opt_act_3: "~ᾰ~α~ῳη|~ᾰ~α~ῳ", "~ᾰ~α~ῳτην|~ᾰ~α~ῳητην", "~ᾰ~α~ῳεν|~ᾰ~α~ῳησᾰν";
    opt_mid_1: "~ᾰ~α~ῳμην", _, "~ᾰ~α~ῳμεθᾰ";
    opt_mid_2: "~ᾰ~α~ῳο", _, "~ᾰ~α~ῳσθε";
    opt_mid_3: "~ᾰ~α~ῳτο", "~ᾰ~α~ῳσθην", "~ᾰ~α~ῳντο";
    opt_pass_2: _, "~ᾰ~α~ῳσθον", _;
    imp_act_2: "~ᾰ~α~ᾱ", _, "~ᾰ~α~ᾱτε";
    imp_act_3: _, _, "~ᾰ~α~ωντων";
    imp_mid_2: "~ᾰ~α~ω", "~ᾰ~α~ᾱσθον", "~ᾰ~α~ᾱσθε";
    imp_mid_3: _, _, "~ᾰ~α~ᾱσθων";
    imp_pass_2: _, "~ᾰ~α~ᾱσθον", "~ᾰ~α~ᾱσθε";
    imp_pass_3: "~ᾰ~α~ᾱσθω", "~ᾰ~α~ᾱσθων", "~ᾰ~α~ᾱσθων";
    imperfect_act_1: "~α~ε~ων", _, derived;
    imperfect_act_2: "~α~ε~ᾱς", "~α~ε~ᾱτον", "~α~ε~ᾱτε";
    imperfect_act_3: "~α~ε~ᾱ", _, "~α~ε~ων";
    imperfect_mid_1: "~α~ε~ωμην", _, "~α~ε~ωμεθᾰ";
    imperfect_mid_2: "~α~ε~ω", _, _;
    imperfect_mid_3: "~α~ε~ᾱτο", _, "~α~ε~ωντο";
    imperfect_pass_2: _, "~α~ε~ᾱσθον", "~α~ε~ᾱσθε";
    imperfect_pass_3: _, "~α~ε~ᾱσθην", _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, _;
    future_ind_act_3: _, _, derived;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, _;
    future_opt_act_3: derived, derived, _;
    aorist_subj_act_1: "~ᾰ~α~ᾰσω", _, _;
    aorist_subj_act_2: "~ᾰ~α~ᾰσῃς", "~ᾰ~α~ᾰσητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~α~ᾰσωσῐ|~ᾰ~α~ᾰσωσῐν";
    aorist_subj_mid_1: "~ᾰ~α~ᾰσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~α~ᾰσῃ", "~ᾰ~α~ᾰσησθον", _;
    aorist_opt_act_1: "~ᾰ~α~ᾰσαιμῐ", _, "~ᾰ~α~ᾰσαιμεν";
    aorist_opt_act_2: "~ᾰ~α~ᾰσειᾰς|~ᾰ~α~ᾰσαις", _, _;
    aorist_opt_act_3: "~ᾰ~α~ᾰσειε|~ᾰ~α~ᾰσειεν|~ᾰ~α~ᾰσαι", _, "~ᾰ~α~ᾰσειᾰν|~ᾰ~α~ᾰσαιεν";
    aorist_opt_mid_1: "~ᾰ~α~ᾰσαιμην", _, _;
    aorist_imp_act_2: "~ᾰ~α~ᾰσον", _, "~ᾰ~α~ᾰσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~α~ᾰσᾰντων";
    aorist_imp_mid_2: "~ᾰ~α~ᾰσαι", "~ᾰ~α~ᾰσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~α~ᾰσᾰσθων";
    perfect_ind_mid_1: _, _, "~ᾰ~ᾰ%ε~ᾰσμεθᾰ";
    perfect_ind_mid_2: "~ᾰ~ᾰ%ε~ᾰσαι", "~ᾰ~ᾰ%ε~ᾰσθον", _;
    perfect_ind_mid_3: "~ᾰ~ᾰ%ε~ᾰσται", _, "~ᾰ~ᾰ%ε~ᾰσᾰται";
    perfect_ind_pass_1: "~ᾰ~ᾰ%ε~ᾰσμαι", _, _;
    perfect_ind_pass_2: _, _, "~ᾰ~ᾰ%ε~ᾰσθε";
    perfect_imp_mid_2: _, _, "~ᾰ~ᾰ%ε~ᾰσθε";
    perfect_imp_mid_3: _, _, "~ᾰ~ᾰ%ε~ᾰσθων";
    perfect_imp_pass_2: "~ᾰ~ᾰ%ε~ᾰσο", _, "~ᾰ~ᾰ%ε~ᾰσθε";
    perfect_imp_pass_3: "~ᾰ~ᾰ%ε~ᾰσθω", "~ᾰ~ᾰ%ε~ᾰσθων", "~ᾰ~ᾰ%ε~ᾰσθων";
    infinitive_pres: "~ᾰ~α~ᾱν", _, "~ᾰ~α~ᾱσθαι";
    infinitive_fut: derived, _, _;
    infinitive_aor: "~ᾰ~α~ᾰσαι", _, _;
    infinitive_perf: _, "~ᾰ~ᾰ%ε~ᾰσθαι", _;
    participle_pres: "~ᾰ~α~ων", _, "~ᾰ~α~ωμενος";
    participle_aor: "~ᾰ~α~ᾰσᾱς", "~ᾰ~α~ᾰσᾰμενος", _;
    participle_perf: _, _, "~ᾰ~ᾰ%ε~ᾰσμενος";
    ),
    // Ἀγωνιζομαι: 1 words (ἀγωνιζομαι). Like Εἰμῐ, except in 62 cells: ind_act_1 sg, ind_act_3 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ….
    conjugation!("ἀγωνιζομαι", 1, "ἀ~ιζομαι", like Εἰμῐ;
    ind_act_1: "ἀ~ιζομαι", _, _;
    ind_act_3: derived, _, _;
    ind_mid_1: "ἀ~ῐζομαι", _, "ἀ~ῐζομεθᾰ";
    ind_mid_2: "ἀ~ῐζῃ|ἀ~ῐζει", _, _;
    ind_mid_3: _, "ἀ~ῐζεσθον", _;
    subj_mid_1: "ἀ~ῐζωμαι", _, "ἀ~ῐζωμεθᾰ";
    subj_mid_2: "ἀ~ῐζῃ", "ἀ~ῐζησθον", _;
    subj_mid_3: "ἀ~ῐζηται", _, "ἀ~ῐζωνται";
    opt_mid_1: "ἀ~ῐζοιμην", _, "ἀ~ῐζοιμεθᾰ";
    opt_mid_2: "ἀ~ῐζοιο", _, "ἀ~ῐζοισθε";
    opt_mid_3: "ἀ~ῐζοιτο", "ἀ~ῐζοισθην", "ἀ~ῐζοιντο";
    imp_mid_2: "ἀ~ῐζου", "ἀ~ῐζεσθον", "ἀ~ῐζεσθε";
    imp_mid_3: "ἀ~ῐζεσθω", _, "ἀ~ῐζεσθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    imperfect_mid_1: "ἠ~ῐζομην", _, "ἠ~ῐζομεθᾰ";
    imperfect_mid_2: "ἠ~ῐζου", "ἠ~ῐζεσθον", _;
    imperfect_mid_3: "ἠ~ῐζετο", _, "ἠ~ῐζοντο";
    future_ind_mid_1: "ἀ~ῐουμαι", _, _;
    future_ind_mid_2: "ἀ~ῐει|ἀ~ῐῃ", "ἀ~ῐεισθον", _;
    future_ind_mid_3: "ἀ~ῐειται", _, _;
    future_opt_mid_1: "ἀ~ῐοιμην", _, "ἀ~ῐοιμεθᾰ";
    future_opt_mid_3: _, "ἀ~ῐοισθην", _;
    aorist_ind_mid_1: "ἠ~ῐσᾰμην", _, _;
    aorist_ind_mid_2: "ἠ~ῐσω", "ἠ~ῐσᾰσθον", _;
    aorist_subj_mid_1: "ἀ~ῐσωμαι", _, _;
    aorist_subj_mid_2: "ἀ~ῐσῃ", "ἀ~ῐσησθον", _;
    aorist_opt_mid_1: "ἀ~ῐσαιμην", _, _;
    aorist_imp_mid_2: "ἀ~ῐσαι", "ἀ~ῐσᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἀ~ῐσᾰσθων";
    perfect_ind_mid_1: _, _, "ἠ~ῐσμεθᾰ";
    perfect_ind_mid_2: "ἠ~ῐσαι", "ἠ~ῐσθον", _;
    perfect_ind_mid_3: "ἠ~ῐσται", _, "ἠ~ῐδᾰται";
    perfect_imp_mid_2: _, _, "ἠ~ῐσθε";
    perfect_imp_mid_3: _, _, "ἠ~ῐσθων";
    pluperfect_mid_1: "ἠ~ῐσμην", _, _;
    pluperfect_mid_2: "ἠ~ῐσο", "ἠ~ῐσθον", _;
    pluperfect_mid_3: "ἠ~ῐστο", _, "ἠ~ῐδᾰτο";
    participle_aor: _, "ἀ~ῐσᾰμενος", _;
    ),
    // Ἀπιστεω: 1 words (ἀπιστεω). Like Ἐκφερω, except in 114 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀπιστεω", 1, "ἀ~ι~εω", like Ἐκφερω;
    stem future_pass: "~ῐ~ηθησ- like ᾰντᾰποκρῑνομαι";
    stem aorist_ind: "~ῐ~ησ- like ἐλπῐζω";
    ind_act_1: "ἀ~ῐ~ω", _, "ἀ~ῐ~ουμεν";
    ind_act_2: "ἀ~ῐ~εις", "ἀ~ῐ~ειτον", _;
    ind_act_3: "ἀ~ῐ~ει", _, "ἀ~ῐ~ουσῐ|ἀ~ῐ~ουσῐν";
    ind_mid_1: _, _, "ἀ~ῐ~ουμεθᾰ";
    ind_mid_2: "ἀ~ῐ~ει|ἀ~ῐ~ῃ", _, _;
    ind_mid_3: _, "ἀ~ῐ~εισθον", _;
    ind_pass_1: "ἀ~ῐ~ουμαι", _, _;
    ind_pass_2: _, "ἀ~ῐ~εισθον", "ἀ~ῐ~εισθε";
    ind_pass_3: "ἀ~ῐ~ειται", "ἀ~ῐ~εισθον", "ἀ~ῐ~ουνται";
    subj_act_1: "ἀ~ῐ~ω", _, "ἀ~ῐ~ωμεν";
    subj_act_2: "ἀ~ῐ~ῃς", _, _;
    subj_act_3: "ἀ~ῐ~ῃ", "ἀ~ῐ~ητον", "ἀ~ῐ~ωσῐ|ἀ~ῐ~ωσῐν";
    subj_mid_1: _, _, "ἀ~ῐ~ωμεθᾰ";
    subj_mid_2: _, "ἀ~ῐ~ησθον", _;
    subj_mid_3: "ἀ~ῐ~ηται", _, "ἀ~ῐ~ωνται";
    subj_pass_1: "ἀ~ῐ~ωμαι", _, _;
    subj_pass_2: _, _, "ἀ~ῐ~ησθε";
    opt_act_1: "ἀ~ῐ~οιην|ἀ~ῐ~οιμῐ", _, "ἀ~ῐ~οιμεν|ἀ~ῐ~οιημεν";
    opt_act_2: "ἀ~ῐ~οιης|ἀ~ῐ~οις", "ἀ~ῐ~οιτον|ἀ~ῐ~οιητον", "ἀ~ῐ~οιτε|ἀ~ῐ~οιητε";
    opt_act_3: "ἀ~ῐ~οιη|ἀ~ῐ~οι", "ἀ~ῐ~οιτην|ἀ~ῐ~οιητην", "ἀ~ῐ~οιεν|ἀ~ῐ~οιησᾰν";
    opt_mid_1: "ἀ~ῐ~οιμην", _, "ἀ~ῐ~οιμεθᾰ";
    opt_mid_2: "ἀ~ῐ~οιο", _, "ἀ~ῐ~οισθε";
    opt_mid_3: "ἀ~ῐ~οιτο", "ἀ~ῐ~οισθην", "ἀ~ῐ~οιντο";
    opt_pass_2: _, "ἀ~ῐ~οισθον", _;
    imp_act_2: "ἀ~ῐ~ει", _, "ἀ~ῐ~ειτε";
    imp_act_3: _, _, "ἀ~ῐ~ουντων";
    imp_mid_2: "ἀ~ῐ~ου", "ἀ~ῐ~εισθον", "ἀ~ῐ~εισθε";
    imp_mid_3: _, _, "ἀ~ῐ~εισθων";
    imp_pass_2: _, "ἀ~ῐ~εισθον", "ἀ~ῐ~εισθε";
    imp_pass_3: "ἀ~ῐ~εισθω", "ἀ~ῐ~εισθων", "ἀ~ῐ~εισθων";
    imperfect_act_1: "ἠ~ῐ~ουν", _, _;
    imperfect_act_2: "ἠ~ῐ~εις", "ἠ~ῐ~ειτον", "ἠ~ῐ~ειτε";
    imperfect_act_3: "ἠ~ῐ~ει", _, "ἠ~ῐ~ουν";
    imperfect_mid_1: "ἠ~ῐ~ουμην", _, "ἠ~ῐ~ουμεθᾰ";
    imperfect_mid_2: "ἠ~ῐ~ου", _, _;
    imperfect_mid_3: "ἠ~ῐ~ειτο", _, "ἠ~ῐ~ουντο";
    imperfect_pass_2: _, "ἠ~ῐ~εισθον", "ἠ~ῐ~εισθε";
    imperfect_pass_3: _, "ἠ~ῐ~εισθην", _;
    future_ind_act_1: "ἀ~ῐ~ησω", _, "ἀ~ῐ~ησομεν";
    future_ind_act_2: "ἀ~ῐ~ησεις", "ἀ~ῐ~ησετον", _;
    future_ind_act_3: _, _, "ἀ~ῐ~ησουσῐ|ἀ~ῐ~ησουσῐν";
    future_ind_mid_1: "ἀ~ῐ~ησομαι", _, _;
    future_ind_mid_2: "ἀ~ῐ~ησῃ|ἀ~ῐ~ησει", "ἀ~ῐ~ησεσθον", _;
    future_ind_mid_3: "ἀ~ῐ~ησεται", _, _;
    future_opt_act_1: "ἀ~ῐ~ησοιμῐ", _, "ἀ~ῐ~ησοιμεν";
    future_opt_act_2: "ἀ~ῐ~ησοις", "ἀ~ῐ~ησοιτον", _;
    future_opt_act_3: "ἀ~ῐ~ησοι", "ἀ~ῐ~ησοιτην", _;
    future_opt_mid_1: "ἀ~ῐ~ησοιμην", _, "ἀ~ῐ~ησοιμεθᾰ";
    future_opt_mid_3: _, "ἀ~ῐ~ησοισθην", _;
    aorist_subj_act_1: "ἀ~ῐ~ησω", _, _;
    aorist_subj_act_2: "ἀ~ῐ~ησῃς", "ἀ~ῐ~ησητον", _;
    aorist_subj_act_3: _, _, "ἀ~ῐ~ησωσῐ|ἀ~ῐ~ησωσῐν";
    aorist_subj_mid_1: "ἀ~ῐ~ησωμαι", _, _;
    aorist_subj_mid_2: "ἀ~ῐ~ησῃ", "ἀ~ῐ~ησησθον", _;
    aorist_opt_act_1: "ἀ~ῐ~ησαιμῐ", _, "ἀ~ῐ~ησαιμεν";
    aorist_opt_act_2: "ἀ~ῐ~ησειᾰς|ἀ~ῐ~ησαις", _, _;
    aorist_opt_act_3: "ἀ~ῐ~ησειε|ἀ~ῐ~ησειεν|ἀ~ῐ~ησαι", _, "ἀ~ῐ~ησειᾰν|ἀ~ῐ~ησαιεν";
    aorist_opt_mid_1: "ἀ~ῐ~ησαιμην", _, _;
    aorist_imp_act_2: "ἀ~ῐ~ησον", _, "ἀ~ῐ~ησᾰτε";
    aorist_imp_act_3: _, _, "ἀ~ῐ~ησᾰντων";
    aorist_imp_mid_2: "ἀ~ῐ~ησαι", "ἀ~ῐ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἀ~ῐ~ησᾰσθων";
    infinitive_pres: "ἀ~ῐ~ειν", _, "ἀ~ῐ~εισθαι";
    infinitive_fut: "ἀ~ῐ~ησειν", _, _;
    infinitive_aor: "ἀ~ῐ~ησαι", _, _;
    participle_pres: "ἀ~ῐ~ων", _, "ἀ~ῐ~ουμενος";
    participle_aor: "ἀ~ῐ~ησᾱς", "ἀ~ῐ~ησᾰμενος", _;
    ),
    // Χᾰριζω: 1 words (χᾰριζω). Like Ἐπορον, except in 29 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_3 pl, ….
    conjugation!("χᾰριζω", 1, "~ιζω", like Ἐπορον;
    stem aorist_ind: "~ῐσ- like χᾰριζω";
    ind_act_1: "~ιζω", _, _;
    future_ind_act_1: "~ῐω", _, "~ῐουμεν";
    future_ind_act_2: "~ῐεις", "~ῐειτον", _;
    future_ind_act_3: _, _, "~ῐουσῐ|~ῐουσῐν";
    future_opt_act_1: "~ῐοιην|~ῐοιμῐ", _, "~ῐοιμεν|~ῐοιημεν";
    future_opt_act_2: "~ῐοιης|~ῐοις", "~ῐοιτον|~ῐοιητον", "~ῐοιτε|~ῐοιητε";
    future_opt_act_3: "~ῐοιη|~ῐοι", "~ῐοιτην|~ῐοιητην", "~ῐοιεν|~ῐοιησᾰν";
    aorist_subj_act_1: "~ῐσω", _, _;
    aorist_subj_act_2: "~ῐσῃς", "~ῐσητον", _;
    aorist_subj_act_3: _, _, "~ῐσωσῐ|~ῐσωσῐν";
    aorist_opt_act_1: "~ῐσαιμῐ", _, "~ῐσαιμεν";
    aorist_opt_act_2: "~ῐσειᾰς|~ῐσαις", _, _;
    aorist_opt_act_3: "~ῐσειε|~ῐσειεν|~ῐσαι", _, "~ῐσειᾰν|~ῐσαιεν";
    aorist_imp_act_2: "~ῐσον", _, "~ῐσᾰτε";
    aorist_imp_act_3: _, _, "~ῐσᾰντων";
    infinitive_fut: "~ῐειν", _, _;
    infinitive_aor: "~ῐσαι", _, _;
    participle_aor: "~ῐσᾱς", _, _;
    ),
    // Διαφορεω: 1 words (διαφορεω). Like Πεμπω, except in 154 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("διαφορεω", 1, "~ια~εω", like Πεμπω;
    stem future_pass: "~ῐᾰ~ηθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ῐε~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ῐε~ηθην- like εὐλογεω";
    stem aorist_pass: "~ῐᾰ~ηθ- like ποιεω";
    stem pluperfect_act: "~ῐε%ε~ηκ- like εὐλογεω";
    ind_act_1: "~ῐᾰ~ω", _, "~ῐᾰ~ουμεν";
    ind_act_2: "~ῐᾰ~εις", "~ῐᾰ~ειτον", _;
    ind_act_3: "~ῐᾰ~ει", _, "~ῐᾰ~ουσῐ|~ῐᾰ~ουσῐν";
    ind_mid_1: _, _, "~ῐᾰ~ουμεθᾰ";
    ind_mid_2: "~ῐᾰ~ει|~ῐᾰ~ῃ", _, _;
    ind_mid_3: _, "~ῐᾰ~εισθον", _;
    ind_pass_1: "~ῐᾰ~ουμαι", _, _;
    ind_pass_2: _, "~ῐᾰ~εισθον", "~ῐᾰ~εισθε";
    ind_pass_3: "~ῐᾰ~ειται", "~ῐᾰ~εισθον", "~ῐᾰ~ουνται";
    subj_act_1: "~ῐᾰ~ω", _, "~ῐᾰ~ωμεν";
    subj_act_2: "~ῐᾰ~ῃς", _, _;
    subj_act_3: "~ῐᾰ~ῃ", "~ῐᾰ~ητον", "~ῐᾰ~ωσῐ|~ῐᾰ~ωσῐν";
    subj_mid_1: _, _, "~ῐᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ῐᾰ~ησθον", _;
    subj_mid_3: "~ῐᾰ~ηται", _, "~ῐᾰ~ωνται";
    subj_pass_1: "~ῐᾰ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῐᾰ~ησθε";
    opt_act_1: "~ῐᾰ~οιην|~ῐᾰ~οιμῐ", _, "~ῐᾰ~οιμεν|~ῐᾰ~οιημεν";
    opt_act_2: "~ῐᾰ~οιης|~ῐᾰ~οις", "~ῐᾰ~οιτον|~ῐᾰ~οιητον", "~ῐᾰ~οιτε|~ῐᾰ~οιητε";
    opt_act_3: "~ῐᾰ~οιη|~ῐᾰ~οι", "~ῐᾰ~οιτην|~ῐᾰ~οιητην", "~ῐᾰ~οιεν|~ῐᾰ~οιησᾰν";
    opt_mid_1: "~ῐᾰ~οιμην", _, "~ῐᾰ~οιμεθᾰ";
    opt_mid_2: "~ῐᾰ~οιο", _, "~ῐᾰ~οισθε";
    opt_mid_3: "~ῐᾰ~οιτο", "~ῐᾰ~οισθην", "~ῐᾰ~οιντο";
    opt_pass_2: _, "~ῐᾰ~οισθον", _;
    imp_act_2: "~ῐᾰ~ει", _, "~ῐᾰ~ειτε";
    imp_act_3: _, _, "~ῐᾰ~ουντων";
    imp_mid_2: "~ῐᾰ~ου", "~ῐᾰ~εισθον", "~ῐᾰ~εισθε";
    imp_mid_3: _, _, "~ῐᾰ~εισθων";
    imp_pass_2: _, "~ῐᾰ~εισθον", "~ῐᾰ~εισθε";
    imp_pass_3: "~ῐᾰ~εισθω", "~ῐᾰ~εισθων", "~ῐᾰ~εισθων";
    imperfect_act_1: "~ῐε~ουν", _, _;
    imperfect_act_2: "~ῐε~εις", "~ῐε~ειτον", "~ῐε~ειτε";
    imperfect_act_3: "~ῐε~ει", _, "~ῐε~ουν";
    imperfect_mid_1: "~ῐε~ουμην", _, "~ῐε~ουμεθᾰ";
    imperfect_mid_2: "~ῐε~ου", _, _;
    imperfect_mid_3: "~ῐε~ειτο", _, "~ῐε~ουντο";
    imperfect_pass_2: _, "~ῐε~εισθον", "~ῐε~εισθε";
    imperfect_pass_3: _, "~ῐε~εισθην", _;
    future_ind_act_1: "~ῐᾰ~ησω", _, "~ῐᾰ~ησομεν";
    future_ind_act_2: "~ῐᾰ~ησεις", "~ῐᾰ~ησετον", _;
    future_ind_act_3: _, _, "~ῐᾰ~ησουσῐ|~ῐᾰ~ησουσῐν";
    future_ind_mid_1: "~ῐᾰ~ησομαι", _, _;
    future_ind_mid_2: "~ῐᾰ~ησῃ|~ῐᾰ~ησει", "~ῐᾰ~ησεσθον", _;
    future_ind_mid_3: "~ῐᾰ~ησεται", _, _;
    future_opt_act_1: "~ῐᾰ~ησοιμῐ", _, "~ῐᾰ~ησοιμεν";
    future_opt_act_2: "~ῐᾰ~ησοις", "~ῐᾰ~ησοιτον", _;
    future_opt_act_3: "~ῐᾰ~ησοι", "~ῐᾰ~ησοιτην", _;
    future_opt_mid_1: "~ῐᾰ~ησοιμην", _, "~ῐᾰ~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ῐᾰ~ησοισθην", _;
    aorist_subj_act_1: "~ῐᾰ~ησω", _, _;
    aorist_subj_act_2: "~ῐᾰ~ησῃς", "~ῐᾰ~ησητον", _;
    aorist_subj_act_3: _, _, "~ῐᾰ~ησωσῐ|~ῐᾰ~ησωσῐν";
    aorist_subj_mid_1: "~ῐᾰ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ῐᾰ~ησῃ", "~ῐᾰ~ησησθον", _;
    aorist_opt_act_1: "~ῐᾰ~ησαιμῐ", _, "~ῐᾰ~ησαιμεν";
    aorist_opt_act_2: "~ῐᾰ~ησειᾰς|~ῐᾰ~ησαις", _, _;
    aorist_opt_act_3: "~ῐᾰ~ησειε|~ῐᾰ~ησειεν|~ῐᾰ~ησαι", _, "~ῐᾰ~ησειᾰν|~ῐᾰ~ησαιεν";
    aorist_opt_mid_1: "~ῐᾰ~ησαιμην", _, _;
    aorist_imp_act_2: "~ῐᾰ~ησον", _, "~ῐᾰ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῐᾰ~ησᾰντων";
    aorist_imp_mid_2: "~ῐᾰ~ησαι", "~ῐᾰ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῐᾰ~ησᾰσθων";
    perfect_ind_act_1: "~ῐᾰ%ε~ηκᾰ", _, "~ῐᾰ%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "~ῐᾰ%ε~ηκᾰτον", _;
    perfect_ind_act_3: "~ῐᾰ%ε~ηκε|~ῐᾰ%ε~ηκεν", _, "~ῐᾰ%ε~ηκᾱσῐ|~ῐᾰ%ε~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ῐᾰ%ε~ημεθᾰ";
    perfect_ind_mid_2: "~ῐᾰ%ε~ησαι", "~ῐᾰ%ε~ησθον", _;
    perfect_ind_mid_3: "~ῐᾰ%ε~ηται", _, "~ῐᾰ%ε~ηνται";
    perfect_ind_pass_1: "~ῐᾰ%ε~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ῐᾰ%ε~ησθε";
    perfect_subj_act_1: "~ῐᾰ%ε~ηκω", _, _;
    perfect_subj_act_2: "~ῐᾰ%ε~ηκῃς", "~ῐᾰ%ε~ηκητον", "~ῐᾰ%ε~ηκητε";
    perfect_subj_act_3: "~ῐᾰ%ε~ηκῃ", _, "~ῐᾰ%ε~ηκωσῐ|~ῐᾰ%ε~ηκωσῐν";
    perfect_opt_act_1: "~ῐᾰ%ε~ηκοιμῐ|~ῐᾰ%ε~ηκοιην", _, _;
    perfect_opt_act_2: "~ῐᾰ%ε~ηκοις|~ῐᾰ%ε~ηκοιης", _, _;
    perfect_opt_act_3: "~ῐᾰ%ε~ηκοι|~ῐᾰ%ε~ηκοιη", _, _;
    perfect_imp_act_2: "~ῐᾰ%ε~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~ῐᾰ%ε~ησθε";
    perfect_imp_mid_3: _, _, "~ῐᾰ%ε~ησθων";
    perfect_imp_pass_2: "~ῐᾰ%ε~ησο", _, "~ῐᾰ%ε~ησθε";
    perfect_imp_pass_3: "~ῐᾰ%ε~ησθω", "~ῐᾰ%ε~ησθων", "~ῐᾰ%ε~ησθων";
    pluperfect_mid_1: "~ῐε%ε~ημην", _, _;
    pluperfect_mid_2: "~ῐε%ε~ησο", "~ῐε%ε~ησθον", _;
    pluperfect_mid_3: "~ῐε%ε~ητο", _, "~ῐε%ε~ηντο";
    pluperfect_pass_1: _, _, "~ῐε%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ῐε%ε~ησθε";
    pluperfect_pass_3: _, "~ῐε%ε~ησθην", _;
    infinitive_pres: "~ῐᾰ~ειν", _, "~ῐᾰ~εισθαι";
    infinitive_fut: "~ῐᾰ~ησειν", _, _;
    infinitive_aor: "~ῐᾰ~ησαι", _, _;
    infinitive_perf: _, "~ῐᾰ%ε~ησθαι", _;
    participle_pres: "~ῐᾰ~ων", _, "~ῐᾰ~ουμενος";
    participle_aor: "~ῐᾰ~ησᾱς", "~ῐᾰ~ησᾰμενος", _;
    participle_perf: "~ῐᾰ%ε~ηκως", _, "~ῐᾰ%ε~ημενος";
    ),
    // Τελεθω: 1 words (τελεθω). Like Εἰμῐ, except in 45 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("τελεθω", 1, "~ω", like Εἰμῐ;
    ind_act_1: "~εσκω", _, "~εσκομεν";
    ind_act_2: "~εσκεις", "~εσκετον", _;
    ind_act_3: "~εσκει", _, "~εσκουσῐ|~εσκουσῐν";
    ind_mid_1: _, _, "~εσκομεθᾰ";
    ind_mid_2: "~εσκεαι|~εσκῃ", _, _;
    ind_mid_3: _, "~εσκεσθον", _;
    subj_act_1: "~εσκω", _, "~εσκωμεν";
    subj_act_2: "~εσκῃς", _, _;
    subj_act_3: "~εσκῃ", "~εσκητον", "~εσκωσῐ|~εσκωσῐν";
    subj_mid_1: _, _, "~εσκωμεθᾰ";
    subj_mid_2: "~εσκηαι|~εσκῃ", "~εσκησθον", _;
    subj_mid_3: "~εσκηται", _, "~εσκωνται";
    opt_act_1: "~εσκοιμῐ", _, "~εσκοιμεν";
    opt_act_2: "~εσκοις", "~εσκοιτον", _;
    opt_act_3: "~εσκοι", "~εσκοιτην", "~εσκοιεν";
    opt_mid_1: "~εσκοιμην", _, "~εσκοιμεθᾰ";
    opt_mid_2: "~εσκοιο", _, "~εσκοισθε";
    opt_mid_3: "~εσκοιτο", "~εσκοισθην", "~εσκοιᾰτο";
    imp_act_2: "~εσκε", _, "~εσκετε";
    imp_act_3: _, _, "~εσκοντων";
    imp_mid_2: "~εσκεο|~εσκευ", "~εσκεσθον", "~εσκεσθε";
    imp_mid_3: _, _, "~εσκεσθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    infinitive_pres: "~εσκειν", _, _;
    participle_pres: "~εσκων", _, _;
    ),
    // Δῐᾰσευομαι: 1 words (δῐᾰσευομαι). Like Νισσομαι, except in 52 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("δῐᾰσευομαι", 1, "~ᾰσ~υομαι", like Νισσομαι;
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
    opt_mid_2: "~ασ~υοιο", _, "~ασ~υοισθε";
    opt_mid_3: "~ασ~υοιτο", "~ασ~υοισθην", "~ασ~υοιντο";
    opt_pass_2: _, "~ασ~υοισθον", _;
    imp_mid_2: "~ασ~υου", "~ασ~υεσθον", "~ασ~υεσθε";
    imp_mid_3: "~ασ~υεσθω", _, "~ασ~υεσθων";
    imp_pass_2: _, "~ασ~υεσθον", "~ασ~υεσθε";
    imp_pass_3: _, "~ασ~υεσθων", "~ασ~υεσθων";
    imperfect_mid_1: "~~σευομην", _, "~~σευομεθᾰ";
    imperfect_mid_2: "~~σευου", "~~σευεσθον", _;
    imperfect_mid_3: "~~σευετο", _, "~~σευοντο";
    imperfect_pass_2: _, _, "~~σευεσθε";
    imperfect_pass_3: _, "~~σευεσθην", _;
    perfect_ind_mid_1: _, _, "~~σσῠμεθᾰ";
    perfect_ind_mid_2: "~~σσῠσαι", "~~σσῠσθον", _;
    perfect_ind_mid_3: "~~σσῠται", _, "~~σσῠνται";
    perfect_imp_mid_2: _, _, "~~σσῠσθε";
    perfect_imp_mid_3: _, _, "~~σσῠσθων";
    infinitive_pres: _, _, "~ασ~υεσθαι";
    participle_pres: _, _, "~ασ~υομενος";
    ),
    // Φθιω: 1 words (φθιω). Like Κειμαι, except in 166 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("φθιω", 1, "~ιω", like Κειμαι;
    stem aorist_ind: "~- like φθιω";
    stem aorist_ind_pass: "~ῐθην- like ἐχω";
    stem aorist_pass: "~ῐθ- like ποιεω";
    ind_act_1: "~ῐνω", _, "~ῐνομεν";
    ind_act_2: "~ῐνεις", "~ῐνετον", _;
    ind_act_3: "~ῐνει", _, "~ῐνουσῐ|~ῐνουσῐν";
    ind_mid_1: _, _, "~ῐνομεθᾰ";
    ind_mid_2: "~ῐνῃ|~ῐνει", _, _;
    ind_mid_3: _, "~ῐνεσθον", _;
    ind_pass_1: "~ῐνομαι", _, _;
    ind_pass_2: _, "~ῐνεσθον", "~ῐνεσθε";
    ind_pass_3: "~ῐνεται", "~ῐνεσθον", "~ῐνονται";
    subj_act_1: "~ῐνω", _, "~ῐνωμεν";
    subj_act_2: "~ῐνῃς", _, _;
    subj_act_3: "~ῐνῃ", "~ῐνητον", "~ῐνωσῐ|~ῐνωσῐν";
    subj_mid_1: _, _, "~ῐνωμεθᾰ";
    subj_mid_2: _, "~ῐνησθον", _;
    subj_mid_3: "~ῐνηται", _, "~ῐνωνται";
    subj_pass_1: "~ῐνωμαι", _, _;
    subj_pass_2: _, _, "~ῐνησθε";
    opt_act_1: "~ῐνοιμῐ", _, "~ῐνοιμεν";
    opt_act_2: "~ῐνοις", "~ῐνοιτον", _;
    opt_act_3: "~ῐνοι", "~ῐνοιτην", "~ῐνοιεν";
    opt_mid_1: "~ῐνοιμην", _, "~ῐνοιμεθᾰ";
    opt_mid_2: "~ῐνοιο", _, "~ῐνοισθε";
    opt_mid_3: "~ῐνοιτο", "~ῐνοισθην", "~ῐνοιντο";
    opt_pass_2: _, "~ῐνοισθον", _;
    imp_act_2: "~ῐνε", _, "~ῐνετε";
    imp_act_3: _, _, "~ῐνοντων";
    imp_mid_2: "~ῐνου", "~ῐνεσθον", "~ῐνεσθε";
    imp_mid_3: _, _, "~ῐνεσθων";
    imp_pass_2: _, "~ῐνεσθον", "~ῐνεσθε";
    imp_pass_3: "~ῐνεσθω", "~ῐνεσθων", "~ῐνεσθων";
    imperfect_act_1: "ἐ~ῐνον", _, _;
    imperfect_act_2: "ἐ~ῐνες", "ἐ~ῐνετον", "ἐ~ῐνετε";
    imperfect_act_3: "ἐ~ῐνε|ἐ~ῐνεν", _, "ἐ~ῐνον";
    imperfect_mid_1: "ἐ~ῐνομην", _, "ἐ~ῐνομεθᾰ";
    imperfect_mid_2: "ἐ~ῐνου", _, _;
    imperfect_mid_3: "ἐ~ῐνετο", _, "ἐ~ῐνοντο";
    imperfect_pass_2: _, "ἐ~ῐνεσθον", "ἐ~ῐνεσθε";
    imperfect_pass_3: _, "ἐ~ῐνεσθην", _;
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
    perfect_ind_act_1: "ἐ~ῐνηκᾰ", _, "ἐ~ῐνηκᾰμεν";
    perfect_ind_act_2: _, "ἐ~ῐνηκᾰτον", _;
    perfect_ind_act_3: "ἐ~ῐνηκε|ἐ~ῐνηκεν", _, "ἐ~ῐνηκᾱσῐ|ἐ~ῐνηκᾱσῐν";
    perfect_subj_act_1: "ἐ~ῐνηκω", _, _;
    perfect_subj_act_2: "ἐ~ῐνηκῃς", "ἐ~ῐνηκητον", "ἐ~ῐνηκητε";
    perfect_subj_act_3: "ἐ~ῐνηκῃ", _, "ἐ~ῐνηκωσῐ|ἐ~ῐνηκωσῐν";
    perfect_opt_act_1: "ἐ~ῐνηκοιμῐ|ἐ~ῐνηκοιην", _, _;
    perfect_opt_act_2: "ἐ~ῐνηκοις|ἐ~ῐνηκοιης", _, _;
    perfect_opt_act_3: "ἐ~ῐνηκοι|ἐ~ῐνηκοιη", _, _;
    perfect_imp_act_2: "ἐ~ῐνηκε", _, _;
    infinitive_pres: "~ῐνειν", _, "~ῐνεσθαι";
    infinitive_fut: "~ῑσειν|~εισειν|~ῐνησειν", "~ῑσεσθαι|~εισεσθαι|~ῐνησεσθαι", _;
    infinitive_aor: "~ῑσαι|~εισαι|~ῐνησαι", "~ῑσᾰσθαι|~εισᾰσθαι|~ῐνησᾰσθαι", _;
    participle_pres: "~ῐνων", _, "~ῐνομενος";
    participle_fut: "~ῑσων|~εισων|~ῐνησων", "~ῑσομενος|~εισομενος|~ῐνησομενος", _;
    participle_aor: "~ῑσᾱς|~εισᾱς|~ῐνησᾱς", "~ῑσᾰμενος|~εισᾰμενος|~ῐνησᾰμενος", _;
    participle_perf: "ἐ~ῐνηκως", _, _;
    ),
    // Οικειοομαι: 1 words (οικειοομαι). Like Ἐρᾰμαι, except in 45 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("οικειοομαι", 1, "οι~οομαι", like Ἐρᾰμαι;
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
    opt_mid_2: "οἰ~οιο", _, "οἰ~οισθε";
    opt_mid_3: "οἰ~οιτο", "οἰ~οισθην", "οἰ~οιντο";
    opt_pass_2: _, "οἰ~οισθον", _;
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
    // Συγκᾰκουργεω: 1 words (συγκᾰκουργεω). Like Ξυμπολεμεω, except in 150 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("συγκᾰκουργεω", 1, "~υγ~εω", like Ξυμπολεμεω;
    stem future_pass: "~ῠγ~ηθησ- like χρημᾰτῐζω";
    stem aorist_pass: "~ῠγ~ηθ- like ποιεω";
    stem pluperfect_act: "~ῠγ%ε~ηκ- like δῐκεω";
    ind_act_1: "~ῠγ~ω", _, "~ῠγ~ουμεν";
    ind_act_2: "~ῠγ~εις", "~ῠγ~ειτον", _;
    ind_act_3: "~ῠγ~ει", _, "~ῠγ~ουσῐ|~ῠγ~ουσῐν";
    ind_mid_1: _, _, "~ῠγ~ουμεθᾰ";
    ind_mid_2: "~ῠγ~ει|~ῠγ~ῃ", _, _;
    ind_mid_3: _, "~ῠγ~εισθον", _;
    ind_pass_1: "~ῠγ~ουμαι", _, _;
    ind_pass_2: _, "~ῠγ~εισθον", "~ῠγ~εισθε";
    ind_pass_3: "~ῠγ~ειται", "~ῠγ~εισθον", "~ῠγ~ουνται";
    subj_act_1: "~ῠγ~ω", _, "~ῠγ~ωμεν";
    subj_act_2: "~ῠγ~ῃς", derived, _;
    subj_act_3: "~ῠγ~ῃ", "~ῠγ~ητον", "~ῠγ~ωσῐ|~ῠγ~ωσῐν";
    subj_mid_1: _, _, "~ῠγ~ωμεθᾰ";
    subj_mid_2: _, "~ῠγ~ησθον", _;
    subj_mid_3: "~ῠγ~ηται", _, "~ῠγ~ωνται";
    subj_pass_1: "~ῠγ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῠγ~ησθε";
    opt_act_1: "~ῠγ~οιην|~ῠγ~οιμῐ", _, "~ῠγ~οιμεν|~ῠγ~οιημεν";
    opt_act_2: "~ῠγ~οιης|~ῠγ~οις", "~ῠγ~οιτον|~ῠγ~οιητον", "~ῠγ~οιτε|~ῠγ~οιητε";
    opt_act_3: "~ῠγ~οιη|~ῠγ~οι", "~ῠγ~οιτην|~ῠγ~οιητην", "~ῠγ~οιεν|~ῠγ~οιησᾰν";
    opt_mid_1: "~ῠγ~οιμην", _, "~ῠγ~οιμεθᾰ";
    opt_mid_2: "~ῠγ~οιο", _, "~ῠγ~οισθε";
    opt_mid_3: "~ῠγ~οιτο", "~ῠγ~οισθην", "~ῠγ~οιντο";
    opt_pass_2: _, "~ῠγ~οισθον", _;
    imp_act_2: "~ῠγ~ει", _, "~ῠγ~ειτε";
    imp_act_3: _, _, "~ῠγ~ουντων";
    imp_mid_2: "~ῠγ~ου", "~ῠγ~εισθον", "~ῠγ~εισθε";
    imp_mid_3: _, _, "~ῠγ~εισθων";
    imp_pass_2: _, "~ῠγ~εισθον", "~ῠγ~εισθε";
    imp_pass_3: "~ῠγ~εισθω", "~ῠγ~εισθων", "~ῠγ~εισθων";
    imperfect_act_1: _, _, derived;
    imperfect_mid_1: "~ῠνε~ουμην", _, "~ῠνε~ουμεθᾰ";
    imperfect_mid_2: "~ῠνε~ου", _, _;
    imperfect_mid_3: "~ῠνε~ειτο", _, "~ῠνε~ουντο";
    imperfect_pass_2: _, "~ῠνε~εισθον", "~ῠνε~εισθε";
    imperfect_pass_3: _, "~ῠνε~εισθην", _;
    future_ind_act_1: "~ῠγ~ησω", _, "~ῠγ~ησομεν";
    future_ind_act_2: "~ῠγ~ησεις", "~ῠγ~ησετον", _;
    future_ind_act_3: _, _, "~ῠγ~ησουσῐ|~ῠγ~ησουσῐν";
    future_ind_mid_1: "~ῠγ~ησομαι", _, _;
    future_ind_mid_2: "~ῠγ~ησῃ|~ῠγ~ησει", "~ῠγ~ησεσθον", _;
    future_ind_mid_3: "~ῠγ~ησεται", _, _;
    future_opt_act_1: "~ῠγ~ησοιμῐ", _, "~ῠγ~ησοιμεν";
    future_opt_act_2: "~ῠγ~ησοις", "~ῠγ~ησοιτον", _;
    future_opt_act_3: "~ῠγ~ησοι", "~ῠγ~ησοιτην", _;
    future_opt_mid_1: "~ῠγ~ησοιμην", _, "~ῠγ~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ῠγ~ησοισθην", _;
    aorist_subj_act_1: "~ῠγ~ησω", _, _;
    aorist_subj_act_2: "~ῠγ~ησῃς", "~ῠγ~ησητον", _;
    aorist_subj_act_3: _, _, "~ῠγ~ησωσῐ|~ῠγ~ησωσῐν";
    aorist_subj_mid_1: "~ῠγ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ῠγ~ησῃ", "~ῠγ~ησησθον", _;
    aorist_opt_act_1: "~ῠγ~ησαιμῐ", _, "~ῠγ~ησαιμεν";
    aorist_opt_act_2: "~ῠγ~ησειᾰς|~ῠγ~ησαις", _, _;
    aorist_opt_act_3: "~ῠγ~ησειε|~ῠγ~ησειεν|~ῠγ~ησαι", _, "~ῠγ~ησειᾰν|~ῠγ~ησαιεν";
    aorist_opt_mid_1: "~ῠγ~ησαιμην", _, _;
    aorist_imp_act_2: "~ῠγ~ησον", _, "~ῠγ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῠγ~ησᾰντων";
    aorist_imp_mid_2: "~ῠγ~ησαι", "~ῠγ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῠγ~ησᾰσθων";
    perfect_ind_act_1: "~ῠγ%ε~ηκᾰ", _, "~ῠγ%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "~ῠγ%ε~ηκᾰτον", _;
    perfect_ind_act_3: "~ῠγ%ε~ηκε|~ῠγ%ε~ηκεν", _, "~ῠγ%ε~ηκᾱσῐ|~ῠγ%ε~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ῠγ%ε~ημεθᾰ";
    perfect_ind_mid_2: "~ῠγ%ε~ησαι", "~ῠγ%ε~ησθον", _;
    perfect_ind_mid_3: "~ῠγ%ε~ηται", _, "~ῠγ%ε~ηνται";
    perfect_ind_pass_1: "~ῠγ%ε~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ῠγ%ε~ησθε";
    perfect_subj_act_1: "~ῠγ%ε~ηκω", _, _;
    perfect_subj_act_2: "~ῠγ%ε~ηκῃς", "~ῠγ%ε~ηκητον", "~ῠγ%ε~ηκητε";
    perfect_subj_act_3: "~ῠγ%ε~ηκῃ", _, "~ῠγ%ε~ηκωσῐ|~ῠγ%ε~ηκωσῐν";
    perfect_opt_act_1: "~ῠγ%ε~ηκοιμῐ|~ῠγ%ε~ηκοιην", _, _;
    perfect_opt_act_2: "~ῠγ%ε~ηκοις|~ῠγ%ε~ηκοιης", _, _;
    perfect_opt_act_3: "~ῠγ%ε~ηκοι|~ῠγ%ε~ηκοιη", _, _;
    perfect_imp_act_2: "~ῠγ%ε~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~ῠγ%ε~ησθε";
    perfect_imp_mid_3: _, _, "~ῠγ%ε~ησθων";
    perfect_imp_pass_2: "~ῠγ%ε~ησο", _, "~ῠγ%ε~ησθε";
    perfect_imp_pass_3: "~ῠγ%ε~ησθω", "~ῠγ%ε~ησθων", "~ῠγ%ε~ησθων";
    pluperfect_mid_1: "ἐ~ῠγ%ε~ημην", _, _;
    pluperfect_mid_2: "ἐ~ῠγ%ε~ησο", "ἐ~ῠγ%ε~ησθον", _;
    pluperfect_mid_3: "ἐ~ῠγ%ε~ητο", _, "ἐ~ῠγ%ε~ηντο";
    pluperfect_pass_1: _, _, "ἐ~ῠγ%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ῠγ%ε~ησθε";
    pluperfect_pass_3: _, "ἐ~ῠγ%ε~ησθην", _;
    infinitive_pres: "~ῠγ~ειν", _, "~ῠγ~εισθαι";
    infinitive_fut: "~ῠγ~ησειν", _, _;
    infinitive_aor: "~ῠγ~ησαι", _, _;
    infinitive_perf: _, "~ῠγ%ε~ησθαι", _;
    participle_pres: "~ῠγ~ων", _, "~ῠγ~ουμενος";
    participle_aor: "~ῠγ~ησᾱς", "~ῠγ~ησᾰμενος", _;
    participle_perf: "~ῠγ%ε~ηκως", _, "~ῠγ%ε~ημενος";
    ),
    // Ἀποστατεω: 1 words (ἀποστατεω). Like Ξυνναυμᾰχεω, except in 57 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀποστατεω", 1, "ἀ~ο~α~εω", like Ξυνναυμᾰχεω;
    stem imperfect: "~ε~ᾰ~- like ἀποστατεω";
    ind_act_1: "ᾰ~ο~ᾰ~ω", _, "ᾰ~ο~ᾰ~ουμεν";
    ind_act_2: "ᾰ~ο~ᾰ~εις", "ᾰ~ο~ᾰ~ειτον", _;
    ind_act_3: "ᾰ~ο~ᾰ~ει", _, "ᾰ~ο~ᾰ~ουσῐ|ᾰ~ο~ᾰ~ουσῐν";
    subj_act_1: "ᾰ~ο~ᾰ~ω", _, "ᾰ~ο~ᾰ~ωμεν";
    subj_act_2: "ᾰ~ο~ᾰ~ῃς", "ᾰ~ο~ᾰ~ητον", _;
    subj_act_3: "ᾰ~ο~ᾰ~ῃ", _, "ᾰ~ο~ᾰ~ωσῐ|ᾰ~ο~ᾰ~ωσῐν";
    opt_act_1: "ᾰ~ο~ᾰ~οιην|ᾰ~ο~ᾰ~οιμῐ", _, "ᾰ~ο~ᾰ~οιμεν|ᾰ~ο~ᾰ~οιημεν";
    opt_act_2: "ᾰ~ο~ᾰ~οιης|ᾰ~ο~ᾰ~οις", "ᾰ~ο~ᾰ~οιτον|ᾰ~ο~ᾰ~οιητον", "ᾰ~ο~ᾰ~οιτε|ᾰ~ο~ᾰ~οιητε";
    opt_act_3: "ᾰ~ο~ᾰ~οιη|ᾰ~ο~ᾰ~οι", "ᾰ~ο~ᾰ~οιτην|ᾰ~ο~ᾰ~οιητην", "ᾰ~ο~ᾰ~οιεν|ᾰ~ο~ᾰ~οιησᾰν";
    imp_act_2: "ᾰ~ο~ᾰ~ει", _, "ᾰ~ο~ᾰ~ειτε";
    imp_act_3: _, _, "ᾰ~ο~ᾰ~ουντων";
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, _;
    future_ind_act_3: _, _, derived;
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
    infinitive_pres: "ᾰ~ο~ᾰ~ειν", _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    participle_pres: "ᾰ~ο~ᾰ~ων", _, _;
    participle_aor: derived, _, _;
    ),
    // Γεγενησαι: 1 words (γεγενησαι). Like Κᾰθημαι, except in 41 cells: ind_act_1 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, ….
    conjugation!("γεγενησαι", 1, "~ε~ησαι", like Κᾰθημαι;
    ind_act_1: "~ε~ησαι", _, _;
    perfect_ind_act_1: "~ο~ᾰ", _, "~ο~ᾰμεν";
    perfect_ind_act_2: _, "~ο~ᾰτον", _;
    perfect_ind_act_3: "~ο~ε|~ο~εν", _, "~ο~ᾱσῐ|~ο~ᾱσῐν";
    perfect_ind_mid_1: _, _, "~ε~ημεθᾰ";
    perfect_ind_mid_2: "~ε~ησαι", "~ε~ησθον", _;
    perfect_ind_mid_3: "~ε~ηται", _, "~ε~ηνται";
    perfect_ind_pass_1: "~ε~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ε~ησθε";
    perfect_subj_act_1: "~ο~ω", _, _;
    perfect_subj_act_2: "~ο~ῃς", "~ο~ητον", "~ο~ητε";
    perfect_subj_act_3: "~ο~ῃ", _, "~ο~ωσῐ|~ο~ωσῐν";
    perfect_opt_act_1: "~ο~οιμῐ|~ο~οιην", _, _;
    perfect_opt_act_2: "~ο~οις|~ο~οιης", _, _;
    perfect_opt_act_3: "~ο~οι|~ο~οιη", _, _;
    perfect_imp_act_2: "~ο~ε", _, _;
    perfect_imp_mid_2: _, _, "~ε~ησθε";
    perfect_imp_mid_3: _, _, "~ε~ησθων";
    perfect_imp_pass_2: "~ε~ησο", _, "~ε~ησθε";
    perfect_imp_pass_3: "~ε~ησθω", "~ε~ησθων", "~ε~ησθων";
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_perf: _, "~ε~ησθαι", _;
    participle_perf: "~ο~ως", _, "~ε~ημενος";
    ),
    // Γεγενηται: 1 words (γεγενηται). Like Γεγενησαι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενηται", 1, "~ε~ηται", like Γεγενησαι;
    ind_act_1: "~ε~ηται", _, _;
    ),
    // Γεγενησθω: 1 words (γεγενησθω). Like Γεγενησαι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησθω", 1, "~ε~ησθω", like Γεγενησαι;
    ind_act_1: "~ε~ησθω", _, _;
    ),
    // Γεγενησο: 1 words (γεγενησο). Like Γεγονοι, except in 33 cells: ind_act_1 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, ….
    conjugation!("γεγενησο", 1, "~ενησο", like Γεγονοι;
    ind_act_1: "~ενησο", _, _;
    perfect_ind_act_1: "~ονᾰ", _, "~ονᾰμεν";
    perfect_ind_act_2: _, "~ονᾰτον", _;
    perfect_ind_act_3: "~ονε|~ονεν", _, "~ονᾱσῐ|~ονᾱσῐν";
    perfect_ind_mid_1: _, _, "~ενημεθᾰ";
    perfect_ind_mid_2: "~ενησαι", "~ενησθον", _;
    perfect_ind_mid_3: "~ενηται", _, "~ενηνται";
    perfect_ind_pass_1: "~ενημαι", _, _;
    perfect_ind_pass_2: _, _, "~ενησθε";
    perfect_subj_act_1: "~ονω", _, _;
    perfect_subj_act_2: "~ονῃς", "~ονητον", "~ονητε";
    perfect_subj_act_3: "~ονῃ", _, "~ονωσῐ|~ονωσῐν";
    perfect_opt_act_1: "~ονοιμῐ|~ονοιην", _, _;
    perfect_opt_act_2: "~ονοις|~ονοιης", _, _;
    perfect_opt_act_3: "~ονοι|~ονοιη", _, _;
    perfect_imp_act_2: "~ονε", _, _;
    perfect_imp_mid_2: _, _, "~ενησθε";
    perfect_imp_mid_3: _, _, "~ενησθων";
    perfect_imp_pass_2: "~ενησο", _, "~ενησθε";
    perfect_imp_pass_3: "~ενησθω", "~ενησθων", "~ενησθων";
    infinitive_perf: _, "~ενησθαι", _;
    participle_perf: "~ονως", _, "~ενημενος";
    ),
    // Γεγενησθον: 1 words (γεγενησθον). Like Γεγενησαι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησθον", 1, "~ε~ησθον", like Γεγενησαι;
    ind_act_1: "~ε~ησθον", _, _;
    ),
    // Γεγενησθων: 1 words (γεγενησθων). Like Γεγενησθω with `` after each pattern (`ν` after ω), except in 2 cells: perfect_subj_act_1 sg, perfect_imp_pass_3 sg.
    conjugation!("γεγενησθων", 1, "~ε~ησθων", like Γεγενησθω with "", "ν" after "ω";
    perfect_subj_act_1: "~ο~ω", _, _;
    perfect_imp_pass_3: "~ε~ησθω", _, _;
    ),
    // Γεγενηνται: 1 words (γεγενηνται). Like Γεγενησαι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενηνται", 1, "~ε~ηνται", like Γεγενησαι;
    ind_act_1: "~ε~ηνται", _, _;
    ),
    // Γεγενησθε: 1 words (γεγενησθε). Like Γεγενησαι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησθε", 1, "~ε~ησθε", like Γεγενησαι;
    ind_act_1: "~ε~ησθε", _, _;
    ),
    // Γεγενημεθα: 1 words (γεγενημεθα). Like Γεγενησαι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενημεθα", 1, "~ε~ημεθα", like Γεγενησαι;
    ind_act_1: "~ε~ημεθα", _, _;
    ),
    // Τεθνηκας: 1 words (τεθνηκας). Like Γεγονοι, except in 7 cells: ind_act_1 sg, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, ….
    conjugation!("τεθνηκας", 1, "~ας", like Γεγονοι;
    ind_act_1: "~ας", _, _;
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκῃ: 1 words (τεθνηκῃ). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκῃ", 1, "~ῃ", like Τεθνηκας;
    ind_act_1: "~ῃ", _, _;
    ),
    // Τεθνηκοιη: 1 words (τεθνηκοιη). Like Γεγονοι with `` after each pattern (`η` after ι), except in 6 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιη", 1, "~οιη", like Γεγονοι with "", "η" after "ι";
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκοιμι: 1 words (τεθνηκοιμι). Like Γεγονοι with `` after each pattern (`μι` after ι), except in 6 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιμι", 1, "~οιμι", like Γεγονοι with "", "μι" after "ι";
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκοιην: 1 words (τεθνηκοιην). Like Γεγονοι with `` after each pattern (`ην` after ι), except in 6 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιην", 1, "~οιην", like Γεγονοι with "", "ην" after "ι";
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκοις: 1 words (τεθνηκοις). Like Γεγονοι with `` after each pattern (`ς` after ι), except in 6 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, perfect_opt_act_3 sg.
    conjugation!("τεθνηκοις", 1, "~οις", like Γεγονοι with "", "ς" after "ι";
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκοιης: 1 words (τεθνηκοιης). Like Γεγονοι with `` after each pattern (`ης` after ι), except in 6 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιης", 1, "~οιης", like Γεγονοι with "", "ης" after "ι";
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκῃς: 1 words (τεθνηκῃς). Like Τεθνηκῃ with `` after each pattern (`ς` after ῃ), except in 1 cell: perfect_subj_act_3 sg.
    conjugation!("τεθνηκῃς", 1, "~ῃς", like Τεθνηκῃ with "", "ς" after "ῃ";
    perfect_subj_act_3: "~ῃ", _, _;
    ),
    // Τεθνηκετω: 1 words (τεθνηκετω). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκετω", 1, "~ετω", like Τεθνηκας;
    ind_act_1: "~ετω", _, _;
    ),
    // Τεθνηκαμεν: 1 words (τεθνηκαμεν). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκαμεν", 1, "~αμεν", like Τεθνηκας;
    ind_act_1: "~αμεν", _, _;
    ),
    // Τεθνηκωμεν: 1 words (τεθνηκωμεν). Like Ἐδω with `μεν` after each pattern, except in 64 cells: future_ind_mid_1 sg, future_ind_mid_2 sg, future_ind_mid_2 du, future_ind_mid_3 sg, future_opt_mid_1 sg, future_opt_mid_1 pl, ….
    conjugation!("τεθνηκωμεν", 1, "~ωμεν", like Ἐδω with "μεν";
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_3: _, derived, _;
    perfect_ind_act_1: "~ᾰ", _, "~ᾰμεν";
    perfect_ind_act_2: derived, "~ᾰτον", derived;
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "~ω", _, derived;
    perfect_subj_act_2: "~ῃς", "~ητον", "~ητε";
    perfect_subj_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, derived;
    perfect_opt_act_2: "~οις|~οιης", derived, derived;
    perfect_opt_act_3: "~οι|~οιη", derived, derived;
    perfect_imp_act_2: "~ε", derived, derived;
    perfect_imp_act_3: derived, derived, derived;
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
    participle_perf: "~ως", _, derived;
    ),
    // Τεθνηκοιμεν: 1 words (τεθνηκοιμεν). Like Γεγονοι with `` after each pattern (`μεν` after ι), except in 6 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιμεν", 1, "~οιμεν", like Γεγονοι with "", "μεν" after "ι";
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκατε: 1 words (τεθνηκατε). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκατε", 1, "~ατε", like Τεθνηκας;
    ind_act_1: "~ατε", _, _;
    ),
    // Τεθνηκητε: 1 words (τεθνηκητε). Like Χρη with `τε` after each pattern, except in 25 cells: ind_act_3 sg, subj_act_3 sg, opt_act_3 sg, future_ind_act_3 sg, future_ind_mid_3 sg, future_opt_act_3 sg, ….
    conjugation!("τεθνηκητε", 1, "~ητε", like Χρη with "τε";
    ind_act_3: derived, _, _;
    subj_act_3: derived, _, _;
    opt_act_3: derived, _, _;
    future_ind_act_3: derived, _, _;
    future_ind_mid_3: derived, _, _;
    future_opt_act_3: derived, _, _;
    future_opt_mid_3: derived, _, _;
    perfect_ind_act_1: "~ᾰ", _, "~ᾰμεν";
    perfect_ind_act_2: _, "~ᾰτον", _;
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_1: "~ω", _, _;
    perfect_subj_act_2: "~ῃς", "~ητον", "~ητε";
    perfect_subj_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
    perfect_opt_act_3: "~οι|~οιη", _, _;
    perfect_imp_act_2: "~ε", _, _;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    participle_perf: "~ως", _, _;
    ),
    // Τεθνηκοιτε: 1 words (τεθνηκοιτε). Like Γεγονοι with `` after each pattern (`τε` after ι), except in 6 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιτε", 1, "~οιτε", like Γεγονοι with "", "τε" after "ι";
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
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
    // Τεθνηκωσι: 1 words (τεθνηκωσι). Like Ἐδω with `σι` after each pattern, except in 64 cells: future_ind_mid_1 sg, future_ind_mid_2 sg, future_ind_mid_2 du, future_ind_mid_3 sg, future_opt_mid_1 sg, future_opt_mid_1 pl, ….
    conjugation!("τεθνηκωσι", 1, "~ωσι", like Ἐδω with "σι";
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_3: _, derived, _;
    perfect_ind_act_1: "~ᾰ", _, "~ᾰμεν";
    perfect_ind_act_2: derived, "~ᾰτον", derived;
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "~ω", _, derived;
    perfect_subj_act_2: "~ῃς", "~ητον", "~ητε";
    perfect_subj_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, derived;
    perfect_opt_act_2: "~οις|~οιης", derived, derived;
    perfect_opt_act_3: "~οι|~οιη", derived, derived;
    perfect_imp_act_2: "~ε", derived, derived;
    perfect_imp_act_3: derived, derived, derived;
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
    participle_perf: "~ως", _, derived;
    ),
    // Τεθνηκωσιν: 1 words (τεθνηκωσιν). Like Ἐδω with `σιν` after each pattern, except in 64 cells: future_ind_mid_1 sg, future_ind_mid_2 sg, future_ind_mid_2 du, future_ind_mid_3 sg, future_opt_mid_1 sg, future_opt_mid_1 pl, ….
    conjugation!("τεθνηκωσιν", 1, "~ωσιν", like Ἐδω with "σιν";
    future_ind_mid_1: derived, _, _;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_3: _, derived, _;
    perfect_ind_act_1: "~ᾰ", _, "~ᾰμεν";
    perfect_ind_act_2: derived, "~ᾰτον", derived;
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_ind_mid_1: _, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: derived, _, _;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "~ω", _, derived;
    perfect_subj_act_2: "~ῃς", "~ητον", "~ητε";
    perfect_subj_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, derived;
    perfect_opt_act_2: "~οις|~οιης", derived, derived;
    perfect_opt_act_3: "~οι|~οιη", derived, derived;
    perfect_imp_act_2: "~ε", derived, derived;
    perfect_imp_act_3: derived, derived, derived;
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
    participle_perf: "~ως", _, derived;
    ),
    // Τεθνηκοιεν: 1 words (τεθνηκοιεν). Like Γεγονοι with `` after each pattern (`εν` after ι), except in 6 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιεν", 1, "~οιεν", like Γεγονοι with "", "εν" after "ι";
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
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
    // Τεθνηκητον: 1 words (τεθνηκητον). Like Χρη with `τον` after each pattern, except in 25 cells: ind_act_3 sg, subj_act_3 sg, opt_act_3 sg, future_ind_act_3 sg, future_ind_mid_3 sg, future_opt_act_3 sg, ….
    conjugation!("τεθνηκητον", 1, "~ητον", like Χρη with "τον";
    ind_act_3: derived, _, _;
    subj_act_3: derived, _, _;
    opt_act_3: derived, _, _;
    future_ind_act_3: derived, _, _;
    future_ind_mid_3: derived, _, _;
    future_opt_act_3: derived, _, _;
    future_opt_mid_3: derived, _, _;
    perfect_ind_act_1: "~ᾰ", _, "~ᾰμεν";
    perfect_ind_act_2: _, "~ᾰτον", _;
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_1: "~ω", _, _;
    perfect_subj_act_2: "~ῃς", "~ητον", "~ητε";
    perfect_subj_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
    perfect_opt_act_3: "~οι|~οιη", _, _;
    perfect_imp_act_2: "~ε", _, _;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    participle_perf: "~ως", _, _;
    ),
    // Τεθνηκοιτον: 1 words (τεθνηκοιτον). Like Γεγονοι with `` after each pattern (`τον` after ι), except in 6 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιτον", 1, "~οιτον", like Γεγονοι with "", "τον" after "ι";
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκοιτην: 1 words (τεθνηκοιτην). Like Γεγονοι with `` after each pattern (`την` after ι), except in 6 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, perfect_opt_act_3 sg.
    conjugation!("τεθνηκοιτην", 1, "~οιτην", like Γεγονοι with "", "την" after "ι";
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
    perfect_opt_act_3: "~οι|~οιη", _, _;
    ),
    // Τεθνηκετον: 1 words (τεθνηκετον). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκετον", 1, "~ετον", like Τεθνηκας;
    ind_act_1: "~ετον", _, _;
    ),
    // Τεθνηκετων: 1 words (τεθνηκετων). Like Τεθνηκετω with `` after each pattern (`ν` after ω), except in 1 cell: perfect_subj_act_1 sg.
    conjugation!("τεθνηκετων", 1, "~ετων", like Τεθνηκετω with "", "ν" after "ω";
    perfect_subj_act_1: "~ω", _, _;
    ),
    // Ἀντιπαρατασσομαι: 1 words (ἀντιπαρατασσομαι).
    conjugation!("ἀντιπαρατασσομαι", 1, "~ι~α~α~ασσομαι";
    stem future_pass: "~ῐ~ᾰ~ᾰ~ᾰχθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ῐ~ᾰ~ε~ᾰξ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ῐ~ᾰ~ε~ᾰχθην- like εὐλογεω";
    stem aorist_pass: "~ῐ~ᾰ~ᾰ~ᾰχθ- like ποιεω";
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
    opt_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσοιο", _, "~ῐ~ᾰ~ᾰ~ᾰσσοισθε";
    opt_mid_3: "~ῐ~ᾰ~ᾰ~ᾰσσοιτο", "~ῐ~ᾰ~ᾰ~ᾰσσοισθην", "~ῐ~ᾰ~ᾰ~ᾰσσοιντο";
    opt_pass_2: _, "~ῐ~ᾰ~ᾰ~ᾰσσοισθον", _;
    imp_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσου", "~ῐ~ᾰ~ᾰ~ᾰσσεσθον", "~ῐ~ᾰ~ᾰ~ᾰσσεσθε";
    imp_mid_3: "~ῐ~ᾰ~ᾰ~ᾰσσεσθω", _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθων";
    imp_pass_2: _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθον", "~ῐ~ᾰ~ᾰ~ᾰσσεσθε";
    imp_pass_3: _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθων", "~ῐ~ᾰ~ᾰ~ᾰσσεσθων";
    imperfect_mid_1: "~ῐ~ᾰ~ε~ᾰσσομην", _, "~ῐ~ᾰ~ε~ᾰσσομεθᾰ";
    imperfect_mid_2: "~ῐ~ᾰ~ε~ᾰσσου", "~ῐ~ᾰ~ε~ᾰσσεσθον", _;
    imperfect_mid_3: "~ῐ~ᾰ~ε~ᾰσσετο", _, "~ῐ~ᾰ~ε~ᾰσσοντο";
    imperfect_pass_2: _, _, "~ῐ~ᾰ~ε~ᾰσσεσθε";
    imperfect_pass_3: _, "~ῐ~ᾰ~ε~ᾰσσεσθην", _;
    future_ind_mid_1: "~ῐ~ᾰ~ᾰ~ᾰξομαι", _, _;
    future_ind_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξῃ|~ῐ~ᾰ~ᾰ~ᾰξει", "~ῐ~ᾰ~ᾰ~ᾰξεσθον", _;
    future_ind_mid_3: "~ῐ~ᾰ~ᾰ~ᾰξεται", _, _;
    future_opt_mid_1: "~ῐ~ᾰ~ᾰ~ᾰξοιμην", _, "~ῐ~ᾰ~ᾰ~ᾰξοιμεθᾰ";
    future_opt_mid_3: _, "~ῐ~ᾰ~ᾰ~ᾰξοισθην", _;
    aorist_subj_mid_1: "~ῐ~ᾰ~ᾰ~ᾰξωμαι", _, _;
    aorist_subj_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξῃ", "~ῐ~ᾰ~ᾰ~ᾰξησθον", _;
    aorist_opt_mid_1: "~ῐ~ᾰ~ᾰ~ᾰξαιμην", _, _;
    aorist_imp_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξαι", "~ῐ~ᾰ~ᾰ~ᾰξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῐ~ᾰ~ᾰ~ᾰξᾰσθων";
    perfect_ind_mid_1: _, _, "~ῐ~ᾰ~ᾰ~ετᾰγμεθᾰ";
    perfect_ind_mid_2: "~ῐ~ᾰ~ᾰ~ετᾰξαι", "~ῐ~ᾰ~ᾰ~ετᾰχθον", _;
    perfect_ind_mid_3: "~ῐ~ᾰ~ᾰ~ετᾰκται", _, "~ῐ~ᾰ~ᾰ~ετᾰγᾰται";
    perfect_ind_pass_1: "~ῐ~ᾰ~ᾰ~ετᾰγμαι", _, _;
    perfect_ind_pass_2: _, _, "~ῐ~ᾰ~ᾰ~ετᾰχθε";
    perfect_imp_mid_2: _, _, "~ῐ~ᾰ~ᾰ~ετᾰχθε";
    perfect_imp_mid_3: _, _, "~ῐ~ᾰ~ᾰ~ετᾰχθων";
    perfect_imp_pass_2: "~ῐ~ᾰ~ᾰ~ετᾰξο", _, "~ῐ~ᾰ~ᾰ~ετᾰχθε";
    perfect_imp_pass_3: "~ῐ~ᾰ~ᾰ~ετᾰχθω", "~ῐ~ᾰ~ᾰ~ετᾰχθων", "~ῐ~ᾰ~ᾰ~ετᾰχθων";
    pluperfect_mid_1: "~ῐ~ᾰ~ε~ετᾰγμην", _, _;
    pluperfect_mid_2: "~ῐ~ᾰ~ε~ετᾰξο", "~ῐ~ᾰ~ε~ετᾰχθον", _;
    pluperfect_mid_3: "~ῐ~ᾰ~ε~ετᾰκτο", _, "~ῐ~ᾰ~ε~ετᾰγᾰτο";
    pluperfect_pass_1: _, _, "~ῐ~ᾰ~ε~ετᾰγμεθᾰ";
    pluperfect_pass_2: _, _, "~ῐ~ᾰ~ε~ετᾰχθε";
    pluperfect_pass_3: _, "~ῐ~ᾰ~ε~ετᾰχθην", _;
    infinitive_pres: _, _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθαι";
    infinitive_perf: _, "~ῐ~ᾰ~ᾰ~ετᾰχθαι", _;
    participle_pres: _, _, "~ῐ~ᾰ~ᾰ~ᾰσσομενος";
    participle_aor: _, "~ῐ~ᾰ~ᾰ~ᾰξᾰμενος", _;
    participle_perf: _, _, "~ῐ~ᾰ~ᾰ~ετᾰγμενος";
    ),
    // Ἐπᾰνερχομαι: 1 words (ἐπᾰνερχομαι). Like Νισσομαι, except in 51 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἐπᾰνερχομαι", 1, "~ᾰ~ρχομαι", like Νισσομαι;
    stem aorist_ind: "~α~λθ- like εἰσερχομαι";
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
    opt_mid_2: "~α~ρχοιο", _, "~α~ρχοισθε";
    opt_mid_3: "~α~ρχοιτο", "~α~ρχοισθην", "~α~ρχοιντο";
    opt_pass_2: _, "~α~ρχοισθον", _;
    imp_mid_2: "~α~ρχου", "~α~ρχεσθον", "~α~ρχεσθε";
    imp_mid_3: "~α~ρχεσθω", _, "~α~ρχεσθων";
    imp_pass_2: _, "~α~ρχεσθον", "~α~ρχεσθε";
    imp_pass_3: _, "~α~ρχεσθων", "~α~ρχεσθων";
    aorist_subj_act_1: "~α~λθω", _, _;
    aorist_subj_act_2: "~α~λθῃς", "~α~λθητον", _;
    aorist_subj_act_3: _, _, "~α~λθωσῐ|~α~λθωσῐν";
    aorist_opt_act_1: "~α~λθοιμῐ", _, "~α~λθοιμεν";
    aorist_opt_act_2: "~α~λθοις", _, _;
    aorist_opt_act_3: "~α~λθοι", _, "~α~λθοιεν";
    aorist_imp_act_2: "~α~λθε", _, "~α~λθετε";
    aorist_imp_act_3: _, _, "~α~λθοντων";
    infinitive_pres: _, _, "~α~ρχεσθαι";
    infinitive_aor: "~α~λθειν", _, _;
    participle_pres: _, _, "~α~ρχομενος";
    participle_aor: "~α~λθων", _, _;
    ),
    // Κιδνᾰμαι: 1 words (κιδνᾰμαι). Like Νισσομαι, except in 37 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("κιδνᾰμαι", 1, "~ι~ᾰμαι", like Νισσομαι;
    ind_act_1: "~ι~ᾰμαι", _, _;
    ind_mid_1: "~ῐ~ᾰμαι", _, "~ῐ~ᾰμεθᾰ";
    ind_mid_2: "~ῐ~ᾰσαι", _, _;
    ind_mid_3: _, "~ῐ~ᾰσθον", _;
    ind_pass_2: _, "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    ind_pass_3: "~ῐ~ᾰται", "~ῐ~ᾰσθον", "~ῐ~ᾰνται";
    opt_mid_1: "~ῐ~αιμην", _, "~ῐ~αιμεθᾰ";
    opt_mid_2: "~ῐ~αιο", _, "~ῐ~αισθε";
    opt_mid_3: "~ῐ~αιτο", "~ῐ~αισθην", "~ῐ~αιντο";
    opt_pass_2: _, "~ῐ~αισθον", _;
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
    // Ἐπισκεπτομαι: 1 words (ἐπισκεπτομαι). Like Δῐακειμαι, except in 97 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἐπισκεπτομαι", 1, "~ι~πτομαι", like Δῐακειμαι;
    stem future_pass: "~ῐ~πησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ε~ψ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ε~- like ἐπισκεπτομαι";
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
    opt_mid_2: "~ῐ~πτοιο", _, "~ῐ~πτοισθε";
    opt_mid_3: "~ῐ~πτοιτο", "~ῐ~πτοισθην", "~ῐ~πτοιντο";
    opt_pass_2: _, "~ῐ~πτοισθον", _;
    imp_mid_2: "~ῐ~πτου", "~ῐ~πτεσθον", "~ῐ~πτεσθε";
    imp_mid_3: "~ῐ~πτεσθω", _, "~ῐ~πτεσθων";
    imp_pass_2: _, "~ῐ~πτεσθον", "~ῐ~πτεσθε";
    imp_pass_3: _, "~ῐ~πτεσθων", "~ῐ~πτεσθων";
    imperfect_mid_1: "~ε~πτομην", _, "~ε~πτομεθᾰ";
    imperfect_mid_2: "~ε~πτου", "~ε~πτεσθον", _;
    imperfect_mid_3: "~ε~πτετο", _, "~ε~πτοντο";
    imperfect_pass_2: _, _, "~ε~πτεσθε";
    imperfect_pass_3: _, "~ε~πτεσθην", _;
    future_ind_mid_1: "~ῐ~ψομαι", _, _;
    future_ind_mid_2: "~ῐ~ψῃ|~ῐ~ψει", "~ῐ~ψεσθον", _;
    future_ind_mid_3: "~ῐ~ψεται", _, _;
    future_opt_mid_1: "~ῐ~ψοιμην", _, "~ῐ~ψοιμεθᾰ";
    future_opt_mid_3: _, "~ῐ~ψοισθην", _;
    aorist_subj_mid_1: "~ῐ~ψωμαι", _, _;
    aorist_subj_mid_2: "~ῐ~ψῃ", "~ῐ~ψησθον", _;
    aorist_subj_pass_1: "~ῐ~φθω|~ῐ~πω", _, "~ῐ~φθωμεν|~ῐ~πωμεν";
    aorist_subj_pass_2: "~ῐ~φθῃς|~ῐ~πῃς", "~ῐ~φθητον|~ῐ~πητον", "~ῐ~φθητε|~ῐ~πητε";
    aorist_subj_pass_3: "~ῐ~φθῃ|~ῐ~πῃ", _, "~ῐ~φθωσῐ|~ῐ~φθωσῐν|~ῐ~πωσῐ|~ῐ~πωσῐν";
    aorist_opt_mid_1: "~ῐ~ψαιμην", _, _;
    aorist_opt_pass_1: "~ῐ~φθειην|~ῐ~πειην", _, "~ῐ~φθειμεν|~ῐ~φθειημεν|~ῐ~πειμεν|~ῐ~πειημεν";
    aorist_opt_pass_2: "~ῐ~φθειης|~ῐ~πειης", "~ῐ~φθειτον|~ῐ~φθειητον|~ῐ~πειτον|~ῐ~πειητον", "~ῐ~φθειτε|~ῐ~φθειητε|~ῐ~πειτε|~ῐ~πειητε";
    aorist_opt_pass_3: "~ῐ~φθειη|~ῐ~πειη", "~ῐ~φθειτην|~ῐ~φθειητην|~ῐ~πειτην|~ῐ~πειητην", "~ῐ~φθειεν|~ῐ~φθειησᾰν|~ῐ~πειεν|~ῐ~πειησᾰν";
    aorist_imp_mid_2: "~ῐ~ψαι", "~ῐ~ψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῐ~ψᾰσθων";
    aorist_imp_pass_2: "~ῐ~φθητῐ|~ῐ~πηθῐ", _, _;
    aorist_imp_pass_3: "~ῐ~φθητω|~ῐ~πητω", "~ῐ~φθητων|~ῐ~πητων", "~ῐ~φθεντων|~ῐ~πεντων";
    perfect_ind_mid_1: _, _, "~ε~μμεθᾰ";
    perfect_ind_mid_2: "~ε~ψαι", "~ε~φθον", _;
    perfect_ind_mid_3: "~ε~πται", _, "~ε~πᾰται";
    perfect_ind_pass_1: "~ε~μμαι", _, _;
    perfect_ind_pass_2: _, _, "~ε~φθε";
    perfect_imp_mid_2: _, _, "~ε~φθε";
    perfect_imp_mid_3: _, _, "~ε~φθων";
    perfect_imp_pass_2: "~ε~ψο", _, "~ε~φθε";
    perfect_imp_pass_3: "~ε~φθω", "~ε~φθων", "~ε~φθων";
    infinitive_pres: _, _, "~ῐ~πτεσθαι";
    infinitive_aor: _, _, "~ῐ~φθηναι|~ῐ~πηναι";
    infinitive_perf: _, "~ε~φθαι", _;
    participle_pres: _, _, "~ῐ~πτομενος";
    participle_aor: _, "~ῐ~ψᾰμενος", "~ῐ~φθεις|~ῐ~πεις";
    participle_perf: _, _, "~ε~μμενος";
    ),
    // Κολυμβαω: 1 words (κολυμβαω). Like Μετεκῑᾰθον, except in 51 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("κολυμβαω", 1, "~υ~αω", like Μετεκῑᾰθον;
    stem imperfect: "~ῠ~- like μειδᾰω";
    stem aorist_ind: "~ῠ~ησ- like χᾰριζω";
    ind_act_1: "~ῠ~ω", _, "~ῠ~ωμεν";
    ind_act_2: "~ῠ~ᾳς", "~ῠ~ᾱτον", _;
    ind_act_3: "~ῠ~ᾳ", _, "~ῠ~ωσῐ|~ῠ~ωσῐν";
    subj_act_1: "~ῠ~ω", _, "~ῠ~ωμεν";
    subj_act_2: "~ῠ~ᾳς", "~ῠ~ᾱτον", _;
    subj_act_3: "~ῠ~ᾳ", _, "~ῠ~ωσῐ|~ῠ~ωσῐν";
    opt_act_1: "~ῠ~ῳην|~ῠ~ῳμῐ", _, "~ῠ~ῳμεν|~ῠ~ῳημεν";
    opt_act_2: "~ῠ~ῳης|~ῠ~ῳς", "~ῠ~ῳτον|~ῠ~ῳητον", "~ῠ~ῳτε|~ῠ~ῳητε";
    opt_act_3: "~ῠ~ῳη|~ῠ~ῳ", "~ῠ~ῳτην|~ῠ~ῳητην", "~ῠ~ῳεν|~ῠ~ῳησᾰν";
    imp_act_2: "~ῠ~ᾱ", _, "~ῠ~ᾱτε";
    imp_act_3: _, _, "~ῠ~ωντων";
    future_ind_act_1: "~ῠ~ησω", _, "~ῠ~ησομεν";
    future_ind_act_2: "~ῠ~ησεις", "~ῠ~ησετον", _;
    future_ind_act_3: _, _, "~ῠ~ησουσῐ|~ῠ~ησουσῐν";
    future_opt_act_1: "~ῠ~ησοιμῐ", _, "~ῠ~ησοιμεν";
    future_opt_act_2: "~ῠ~ησοις", "~ῠ~ησοιτον", _;
    future_opt_act_3: "~ῠ~ησοι", "~ῠ~ησοιτην", _;
    aorist_subj_act_1: "~ῠ~ησω", _, _;
    aorist_subj_act_2: "~ῠ~ησῃς", "~ῠ~ησητον", _;
    aorist_subj_act_3: _, _, "~ῠ~ησωσῐ|~ῠ~ησωσῐν";
    aorist_opt_act_1: "~ῠ~ησαιμῐ", _, "~ῠ~ησαιμεν";
    aorist_opt_act_2: "~ῠ~ησειᾰς|~ῠ~ησαις", _, _;
    aorist_opt_act_3: "~ῠ~ησειε|~ῠ~ησειεν|~ῠ~ησαι", _, "~ῠ~ησειᾰν|~ῠ~ησαιεν";
    aorist_imp_act_2: "~ῠ~ησον", _, "~ῠ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῠ~ησᾰντων";
    infinitive_pres: "~ῠ~ᾱν", _, _;
    infinitive_fut: "~ῠ~ησειν", _, _;
    infinitive_aor: "~ῠ~ησαι", _, _;
    participle_pres: "~ῠ~ων", _, _;
    participle_aor: "~ῠ~ησᾱς", _, _;
    ),
    // Δῐκειν: 1 words (δῐκειν). Like Ἐπορον, except in 2 cells: ind_act_1 sg, aorist_subj_act_3 pl.
    conjugation!("δῐκειν", 1, "~ειν", like Ἐπορον;
    stem aorist_ind: "~- like ἐπεφνον";
    ind_act_1: "~ειν", _, _;
    aorist_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    ),
    // Ὑπονοεω: 1 words (ὑπονοεω). Like Πεμπω, except in 158 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ὑπονοεω", 1, "ὑ~ο~εω", like Πεμπω;
    stem aorist_ind: "~ε~ησ- like ὑπονοεω";
    stem aorist_pass: "~ο~ηθ- like ὑπονοεω";
    stem pluperfect_act: "~ε%ε~ηκ- like ὑπονοεω";
    ind_act_1: "ῠ~ο~ω", _, "ῠ~ο~ουμεν";
    ind_act_2: "ῠ~ο~εις", "ῠ~ο~ειτον", _;
    ind_act_3: "ῠ~ο~ει", _, "ῠ~ο~ουσῐ|ῠ~ο~ουσῐν";
    ind_mid_1: _, _, "ῠ~ο~ουμεθᾰ";
    ind_mid_2: "ῠ~ο~ει|ῠ~ο~ῃ", _, _;
    ind_mid_3: _, "ῠ~ο~εισθον", _;
    ind_pass_1: "ῠ~ο~ουμαι", _, _;
    ind_pass_2: _, "ῠ~ο~εισθον", "ῠ~ο~εισθε";
    ind_pass_3: "ῠ~ο~ειται", "ῠ~ο~εισθον", "ῠ~ο~ουνται";
    subj_act_1: "ῠ~ο~ω", _, "ῠ~ο~ωμεν";
    subj_act_2: "ῠ~ο~ῃς", _, _;
    subj_act_3: "ῠ~ο~ῃ", "ῠ~ο~ητον", "ῠ~ο~ωσῐ|ῠ~ο~ωσῐν";
    subj_mid_1: _, _, "ῠ~ο~ωμεθᾰ";
    subj_mid_2: _, "ῠ~ο~ησθον", _;
    subj_mid_3: "ῠ~ο~ηται", _, "ῠ~ο~ωνται";
    subj_pass_1: "ῠ~ο~ωμαι", _, _;
    subj_pass_2: _, _, "ῠ~ο~ησθε";
    opt_act_1: "ῠ~ο~οιην|ῠ~ο~οιμῐ", _, "ῠ~ο~οιμεν|ῠ~ο~οιημεν";
    opt_act_2: "ῠ~ο~οιης|ῠ~ο~οις", "ῠ~ο~οιτον|ῠ~ο~οιητον", "ῠ~ο~οιτε|ῠ~ο~οιητε";
    opt_act_3: "ῠ~ο~οιη|ῠ~ο~οι", "ῠ~ο~οιτην|ῠ~ο~οιητην", "ῠ~ο~οιεν|ῠ~ο~οιησᾰν";
    opt_mid_1: "ῠ~ο~οιμην", _, "ῠ~ο~οιμεθᾰ";
    opt_mid_2: "ῠ~ο~οιο", _, "ῠ~ο~οισθε";
    opt_mid_3: "ῠ~ο~οιτο", "ῠ~ο~οισθην", "ῠ~ο~οιντο";
    opt_pass_2: _, "ῠ~ο~οισθον", _;
    imp_act_2: "ῠ~ο~ει", _, "ῠ~ο~ειτε";
    imp_act_3: _, _, "ῠ~ο~ουντων";
    imp_mid_2: "ῠ~ο~ου", "ῠ~ο~εισθον", "ῠ~ο~εισθε";
    imp_mid_3: _, _, "ῠ~ο~εισθων";
    imp_pass_2: _, "ῠ~ο~εισθον", "ῠ~ο~εισθε";
    imp_pass_3: "ῠ~ο~εισθω", "ῠ~ο~εισθων", "ῠ~ο~εισθων";
    imperfect_act_1: "ῠ~ε~ουν", _, _;
    imperfect_act_2: "ῠ~ε~εις", "ῠ~ε~ειτον", "ῠ~ε~ειτε";
    imperfect_act_3: "ῠ~ε~ει", _, "ῠ~ε~ουν";
    imperfect_mid_1: "ῠ~ε~ουμην", _, "ῠ~ε~ουμεθᾰ";
    imperfect_mid_2: "ῠ~ε~ου", _, _;
    imperfect_mid_3: "ῠ~ε~ειτο", _, "ῠ~ε~ουντο";
    imperfect_pass_2: _, "ῠ~ε~εισθον", "ῠ~ε~εισθε";
    imperfect_pass_3: _, "ῠ~ε~εισθην", _;
    future_ind_act_1: "ῠ~ο~ησω", _, "ῠ~ο~ησομεν";
    future_ind_act_2: "ῠ~ο~ησεις", "ῠ~ο~ησετον", _;
    future_ind_act_3: _, _, "ῠ~ο~ησουσῐ|ῠ~ο~ησουσῐν";
    future_ind_mid_1: "ῠ~ο~ησομαι", _, _;
    future_ind_mid_2: "ῠ~ο~ησῃ|ῠ~ο~ησει", "ῠ~ο~ησεσθον", _;
    future_ind_mid_3: "ῠ~ο~ησεται", _, _;
    future_ind_pass_1: "ῠ~ο~ηθησομαι", _, _;
    future_ind_pass_2: "ῠ~ο~ηθησῃ|ῠ~ο~ηθησει", _, _;
    future_ind_pass_3: _, "ῠ~ο~ηθησεσθον", _;
    future_opt_act_1: "ῠ~ο~ησοιμῐ", _, "ῠ~ο~ησοιμεν";
    future_opt_act_2: "ῠ~ο~ησοις", "ῠ~ο~ησοιτον", _;
    future_opt_act_3: "ῠ~ο~ησοι", "ῠ~ο~ησοιτην", _;
    future_opt_mid_1: "ῠ~ο~ησοιμην", _, "ῠ~ο~ησοιμεθᾰ";
    future_opt_mid_3: _, "ῠ~ο~ησοισθην", _;
    aorist_ind_pass_1: "ῠ~ε~ηθην", _, _;
    aorist_subj_act_1: "ῠ~ο~ησω", _, _;
    aorist_subj_act_2: "ῠ~ο~ησῃς", "ῠ~ο~ησητον", _;
    aorist_subj_act_3: _, _, "ῠ~ο~ησωσῐ|ῠ~ο~ησωσῐν";
    aorist_subj_mid_1: "ῠ~ο~ησωμαι", _, _;
    aorist_subj_mid_2: "ῠ~ο~ησῃ", "ῠ~ο~ησησθον", _;
    aorist_opt_act_1: "ῠ~ο~ησαιμῐ", _, "ῠ~ο~ησαιμεν";
    aorist_opt_act_2: "ῠ~ο~ησειᾰς|ῠ~ο~ησαις", _, _;
    aorist_opt_act_3: "ῠ~ο~ησειε|ῠ~ο~ησειεν|ῠ~ο~ησαι", _, "ῠ~ο~ησειᾰν|ῠ~ο~ησαιεν";
    aorist_opt_mid_1: "ῠ~ο~ησαιμην", _, _;
    aorist_imp_act_2: "ῠ~ο~ησον", _, "ῠ~ο~ησᾰτε";
    aorist_imp_act_3: _, _, "ῠ~ο~ησᾰντων";
    aorist_imp_mid_2: "ῠ~ο~ησαι", "ῠ~ο~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "ῠ~ο~ησᾰσθων";
    perfect_ind_act_1: "ῠ~ο%ε~ηκᾰ", _, "ῠ~ο%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "ῠ~ο%ε~ηκᾰτον", _;
    perfect_ind_act_3: "ῠ~ο%ε~ηκε|ῠ~ο%ε~ηκεν", _, "ῠ~ο%ε~ηκᾱσῐ|ῠ~ο%ε~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "ῠ~ο%ε~ημεθᾰ";
    perfect_ind_mid_2: "ῠ~ο%ε~ησαι", "ῠ~ο%ε~ησθον", _;
    perfect_ind_mid_3: "ῠ~ο%ε~ηται", _, "ῠ~ο%ε~ηνται";
    perfect_ind_pass_1: "ῠ~ο%ε~ημαι", _, _;
    perfect_ind_pass_2: _, _, "ῠ~ο%ε~ησθε";
    perfect_subj_act_1: "ῠ~ο%ε~ηκω", _, _;
    perfect_subj_act_2: "ῠ~ο%ε~ηκῃς", "ῠ~ο%ε~ηκητον", "ῠ~ο%ε~ηκητε";
    perfect_subj_act_3: "ῠ~ο%ε~ηκῃ", _, "ῠ~ο%ε~ηκωσῐ|ῠ~ο%ε~ηκωσῐν";
    perfect_opt_act_1: "ῠ~ο%ε~ηκοιμῐ|ῠ~ο%ε~ηκοιην", _, _;
    perfect_opt_act_2: "ῠ~ο%ε~ηκοις|ῠ~ο%ε~ηκοιης", _, _;
    perfect_opt_act_3: "ῠ~ο%ε~ηκοι|ῠ~ο%ε~ηκοιη", _, _;
    perfect_imp_act_2: "ῠ~ο%ε~ηκε", _, _;
    perfect_imp_mid_2: _, _, "ῠ~ο%ε~ησθε";
    perfect_imp_mid_3: _, _, "ῠ~ο%ε~ησθων";
    perfect_imp_pass_2: "ῠ~ο%ε~ησο", _, "ῠ~ο%ε~ησθε";
    perfect_imp_pass_3: "ῠ~ο%ε~ησθω", "ῠ~ο%ε~ησθων", "ῠ~ο%ε~ησθων";
    pluperfect_mid_1: "ῠ~ε%ε~ημην", _, _;
    pluperfect_mid_2: "ῠ~ε%ε~ησο", "ῠ~ε%ε~ησθον", _;
    pluperfect_mid_3: "ῠ~ε%ε~ητο", _, "ῠ~ε%ε~ηντο";
    pluperfect_pass_1: _, _, "ῠ~ε%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "ῠ~ε%ε~ησθε";
    pluperfect_pass_3: _, "ῠ~ε%ε~ησθην", _;
    infinitive_pres: "ῠ~ο~ειν", _, "ῠ~ο~εισθαι";
    infinitive_fut: "ῠ~ο~ησειν", _, _;
    infinitive_aor: "ῠ~ο~ησαι", _, _;
    infinitive_perf: _, "ῠ~ο%ε~ησθαι", _;
    participle_pres: "ῠ~ο~ων", _, "ῠ~ο~ουμενος";
    participle_aor: "ῠ~ο~ησᾱς", "ῠ~ο~ησᾰμενος", _;
    participle_perf: "ῠ~ο%ε~ηκως", _, "ῠ~ο%ε~ημενος";
    ),
    // Ἀντᾰγωνιζομαι: 1 words (ἀντᾰγωνιζομαι). Like Ἐπευχομαι, except in 84 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἀντᾰγωνιζομαι", 1, "~ᾰ~ιζομαι", like Ἐπευχομαι;
    stem future_pass: "~ᾰ~ῐσθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~η~ῐσ- like μηκᾰομαι";
    stem aorist_ind_pass: "~η~ῐσθην- like εὐλογεω";
    stem aorist_pass: "~ᾰ~ῐσθ- like ποιεω";
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
    opt_mid_2: "~ᾰ~ῐζοιο", _, "~ᾰ~ῐζοισθε";
    opt_mid_3: "~ᾰ~ῐζοιτο", "~ᾰ~ῐζοισθην", "~ᾰ~ῐζοιντο";
    opt_pass_2: _, "~ᾰ~ῐζοισθον", _;
    imp_mid_2: "~ᾰ~ῐζου", "~ᾰ~ῐζεσθον", "~ᾰ~ῐζεσθε";
    imp_mid_3: "~ᾰ~ῐζεσθω", _, "~ᾰ~ῐζεσθων";
    imp_pass_2: _, "~ᾰ~ῐζεσθον", "~ᾰ~ῐζεσθε";
    imp_pass_3: _, "~ᾰ~ῐζεσθων", "~ᾰ~ῐζεσθων";
    imperfect_mid_1: "~η~ῐζομην", _, "~η~ῐζομεθᾰ";
    imperfect_mid_2: "~η~ῐζου", "~η~ῐζεσθον", _;
    imperfect_mid_3: "~η~ῐζετο", _, "~η~ῐζοντο";
    imperfect_pass_2: _, _, "~η~ῐζεσθε";
    imperfect_pass_3: _, "~η~ῐζεσθην", _;
    future_ind_mid_1: "~ᾰ~ῐουμαι", _, _;
    future_ind_mid_2: "~ᾰ~ῐει|~ᾰ~ῐῃ", "~ᾰ~ῐεισθον", _;
    future_ind_mid_3: "~ᾰ~ῐειται", _, _;
    future_opt_mid_1: "~ᾰ~ῐοιμην", _, "~ᾰ~ῐοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~ῐοισθην", _;
    aorist_subj_mid_1: "~ᾰ~ῐσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ῐσῃ", "~ᾰ~ῐσησθον", _;
    aorist_opt_mid_1: "~ᾰ~ῐσαιμην", _, _;
    aorist_imp_mid_2: "~ᾰ~ῐσαι", "~ᾰ~ῐσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ῐσᾰσθων";
    perfect_ind_mid_1: _, _, "~η~ῐσμεθᾰ";
    perfect_ind_mid_2: "~η~ῐσαι", "~η~ῐσθον", _;
    perfect_ind_mid_3: "~η~ῐσται", _, "~η~ῐδᾰται";
    perfect_ind_pass_1: "~η~ῐσμαι", _, _;
    perfect_ind_pass_2: _, _, "~η~ῐσθε";
    perfect_imp_mid_2: _, _, "~η~ῐσθε";
    perfect_imp_mid_3: _, _, "~η~ῐσθων";
    perfect_imp_pass_2: "~η~ῐσο", _, "~η~ῐσθε";
    perfect_imp_pass_3: "~η~ῐσθω", "~η~ῐσθων", "~η~ῐσθων";
    pluperfect_mid_1: "~η~ῐσμην", _, _;
    pluperfect_mid_2: "~η~ῐσο", "~η~ῐσθον", _;
    pluperfect_mid_3: "~η~ῐστο", _, "~η~ῐδᾰτο";
    pluperfect_pass_1: _, _, "~η~ῐσμεθᾰ";
    pluperfect_pass_2: _, _, "~η~ῐσθε";
    pluperfect_pass_3: _, "~η~ῐσθην", _;
    infinitive_pres: _, _, "~ᾰ~ῐζεσθαι";
    infinitive_perf: _, "~η~ῐσθαι", _;
    participle_pres: _, _, "~ᾰ~ῐζομενος";
    participle_aor: _, "~ᾰ~ῐσᾰμενος", _;
    participle_perf: _, _, "~η~ῐσμενος";
    ),
    // Διπλᾰσῐοομαι: 1 words (διπλᾰσῐοομαι). Like Νισσομαι, except in 25 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("διπλᾰσῐοομαι", 1, "~ι~οομαι", like Νισσομαι;
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
    // Ἐξικνεομαι: 1 words (ἐξικνεομαι). Like Ἐρᾰμαι, except in 67 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἐξικνεομαι", 1, "~ικνεομαι", like Ἐρᾰμαι;
    stem aorist_ind: "~ῑκ- like πᾰρᾰγῐγνομαι";
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
    opt_mid_2: "~ῐκνοιο", _, "~ῐκνοισθε";
    opt_mid_3: "~ῐκνοιτο", "~ῐκνοισθην", "~ῐκνοιντο";
    opt_pass_2: _, "~ῐκνοισθον", _;
    imp_mid_2: "~ῐκνου", "~ῐκνεισθον", "~ῐκνεισθε";
    imp_mid_3: "~ῐκνεισθω", _, "~ῐκνεισθων";
    imp_pass_2: _, "~ῐκνεισθον", "~ῐκνεισθε";
    imp_pass_3: _, "~ῐκνεισθων", "~ῐκνεισθων";
    imperfect_mid_1: "~ῑκνουμην", _, "~ῑκνουμεθᾰ";
    imperfect_mid_2: "~ῑκνου", "~ῑκνεισθον", _;
    imperfect_mid_3: "~ῑκνειτο", _, "~ῑκνουντο";
    imperfect_pass_2: _, _, "~ῑκνεισθε";
    imperfect_pass_3: _, "~ῑκνεισθην", _;
    future_ind_mid_1: "~ῐξομαι", _, _;
    future_ind_mid_2: "~ῐξῃ|~ῐξει", "~ῐξεσθον", _;
    future_ind_mid_3: "~ῐξεται", _, _;
    future_opt_mid_1: "~ῐξοιμην", _, "~ῐξοιμεθᾰ";
    future_opt_mid_3: _, "~ῐξοισθην", _;
    aorist_subj_mid_1: "~ῐκωμαι", _, _;
    aorist_subj_mid_2: "~ῐκῃ", "~ῐκησθον", _;
    aorist_opt_mid_1: "~ῐκοιμην", _, _;
    aorist_imp_mid_2: "~ῐκου", "~ῐκεσθον", _;
    aorist_imp_mid_3: _, _, "~ῐκεσθων";
    perfect_ind_mid_1: _, _, "~ῑγμεθᾰ";
    perfect_ind_mid_2: "~ῑξαι", "~ῑχθον", _;
    perfect_ind_mid_3: "~ῑκται", _, "~ῑγᾰται";
    perfect_imp_mid_2: _, _, "~ῑχθε";
    perfect_imp_mid_3: _, _, "~ῑχθων";
    infinitive_pres: _, _, "~ῐκνεισθαι";
    participle_pres: _, _, "~ῐκνουμενος";
    participle_aor: _, "~ῐκομενος", _;
    ),
    // Καταπετροω: 1 words (καταπετροω). Like Λαλαγεω, except in 122 cells: ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("καταπετροω", 1, "~α~α~οω", like Λαλαγεω;
    stem future_pass: "~ᾰ~ᾰ~ωθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ᾰ~ε~ωσ- like εὐλογεω";
    stem aorist_ind_pass: "~ᾰ~ε~ωθην- like εὐλογεω";
    stem aorist_pass: "~ᾰ~ᾰ~ωθ- like ποιεω";
    stem pluperfect_act: "~ᾰ~ε~ωκ- like εὐλογεω";
    ind_act_2: "~ᾰ~ᾰ~οις", "~ᾰ~ᾰ~ουτον", _;
    ind_act_3: "~ᾰ~ᾰ~οι", _, _;
    ind_mid_2: "~ᾰ~ᾰ~οι", _, _;
    ind_mid_3: _, "~ᾰ~ᾰ~ουσθον", _;
    ind_pass_2: _, "~ᾰ~ᾰ~ουσθον", "~ᾰ~ᾰ~ουσθε";
    ind_pass_3: "~ᾰ~ᾰ~ουται", "~ᾰ~ᾰ~ουσθον", _;
    subj_act_2: "~ᾰ~ᾰ~οις", _, _;
    subj_act_3: "~ᾰ~ᾰ~οι", "~ᾰ~ᾰ~ωτον", _;
    subj_mid_2: _, "~ᾰ~ᾰ~ωσθον", _;
    subj_mid_3: "~ᾰ~ᾰ~ωται", _, _;
    subj_pass_2: _, _, "~ᾰ~ᾰ~ωσθε";
    imp_act_2: "~ᾰ~ᾰ~ου", _, "~ᾰ~ᾰ~ουτε";
    imp_mid_2: _, "~ᾰ~ᾰ~ουσθον", "~ᾰ~ᾰ~ουσθε";
    imp_mid_3: _, _, "~ᾰ~ᾰ~ουσθων";
    imp_pass_2: _, "~ᾰ~ᾰ~ουσθον", "~ᾰ~ᾰ~ουσθε";
    imp_pass_3: "~ᾰ~ᾰ~ουσθω", "~ᾰ~ᾰ~ουσθων", "~ᾰ~ᾰ~ουσθων";
    imperfect_act_1: "~ᾰ~ε~ουν", _, _;
    imperfect_act_2: "~ᾰ~ε~ους", "~ᾰ~ε~ουτον", "~ᾰ~ε~ουτε";
    imperfect_act_3: "~ᾰ~ε~ου", _, "~ᾰ~ε~ουν";
    imperfect_mid_1: "~ᾰ~ε~ουμην", _, "~ᾰ~ε~ουμεθᾰ";
    imperfect_mid_2: "~ᾰ~ε~ου", _, _;
    imperfect_mid_3: "~ᾰ~ε~ουτο", _, "~ᾰ~ε~ουντο";
    imperfect_pass_2: _, "~ᾰ~ε~ουσθον", "~ᾰ~ε~ουσθε";
    imperfect_pass_3: _, "~ᾰ~ε~ουσθην", _;
    future_ind_act_1: "~ᾰ~ᾰ~ωσω", _, "~ᾰ~ᾰ~ωσομεν";
    future_ind_act_2: "~ᾰ~ᾰ~ωσεις", "~ᾰ~ᾰ~ωσετον", _;
    future_ind_act_3: _, _, "~ᾰ~ᾰ~ωσουσῐ|~ᾰ~ᾰ~ωσουσῐν";
    future_ind_mid_1: "~ᾰ~ᾰ~ωσομαι", _, _;
    future_ind_mid_2: "~ᾰ~ᾰ~ωσῃ|~ᾰ~ᾰ~ωσει", "~ᾰ~ᾰ~ωσεσθον", _;
    future_ind_mid_3: "~ᾰ~ᾰ~ωσεται", _, _;
    future_opt_act_1: "~ᾰ~ᾰ~ωσοιμῐ", _, "~ᾰ~ᾰ~ωσοιμεν";
    future_opt_act_2: "~ᾰ~ᾰ~ωσοις", "~ᾰ~ᾰ~ωσοιτον", _;
    future_opt_act_3: "~ᾰ~ᾰ~ωσοι", "~ᾰ~ᾰ~ωσοιτην", _;
    future_opt_mid_1: "~ᾰ~ᾰ~ωσοιμην", _, "~ᾰ~ᾰ~ωσοιμεθᾰ";
    future_opt_mid_3: _, "~ᾰ~ᾰ~ωσοισθην", _;
    aorist_subj_act_1: "~ᾰ~ᾰ~ωσω", _, _;
    aorist_subj_act_2: "~ᾰ~ᾰ~ωσῃς", "~ᾰ~ᾰ~ωσητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~ᾰ~ωσωσῐ|~ᾰ~ᾰ~ωσωσῐν";
    aorist_subj_mid_1: "~ᾰ~ᾰ~ωσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ᾰ~ωσῃ", "~ᾰ~ᾰ~ωσησθον", _;
    aorist_opt_act_1: "~ᾰ~ᾰ~ωσαιμῐ", _, "~ᾰ~ᾰ~ωσαιμεν";
    aorist_opt_act_2: "~ᾰ~ᾰ~ωσειᾰς|~ᾰ~ᾰ~ωσαις", _, _;
    aorist_opt_act_3: "~ᾰ~ᾰ~ωσειε|~ᾰ~ᾰ~ωσειεν|~ᾰ~ᾰ~ωσαι", _, "~ᾰ~ᾰ~ωσειᾰν|~ᾰ~ᾰ~ωσαιεν";
    aorist_opt_mid_1: "~ᾰ~ᾰ~ωσαιμην", _, _;
    aorist_imp_act_2: "~ᾰ~ᾰ~ωσον", _, "~ᾰ~ᾰ~ωσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ᾰ~ωσᾰντων";
    aorist_imp_mid_2: "~ᾰ~ᾰ~ωσαι", "~ᾰ~ᾰ~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ᾰ~ωσᾰσθων";
    perfect_ind_act_1: "~ᾰ~ε~ωκᾰ", _, "~ᾰ~ε~ωκᾰμεν";
    perfect_ind_act_2: _, "~ᾰ~ε~ωκᾰτον", _;
    perfect_ind_act_3: "~ᾰ~ε~ωκε|~ᾰ~ε~ωκεν", _, "~ᾰ~ε~ωκᾱσῐ|~ᾰ~ε~ωκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ᾰ~ε~ωμεθᾰ";
    perfect_ind_mid_2: "~ᾰ~ε~ωσαι", "~ᾰ~ε~ωσθον", _;
    perfect_ind_mid_3: "~ᾰ~ε~ωται", _, "~ᾰ~ε~ωνται";
    perfect_ind_pass_1: "~ᾰ~ε~ωμαι", _, _;
    perfect_ind_pass_2: _, _, "~ᾰ~ε~ωσθε";
    perfect_subj_act_1: "~ᾰ~ε~ωκω", _, _;
    perfect_subj_act_2: "~ᾰ~ε~ωκῃς", "~ᾰ~ε~ωκητον", "~ᾰ~ε~ωκητε";
    perfect_subj_act_3: "~ᾰ~ε~ωκῃ", _, "~ᾰ~ε~ωκωσῐ|~ᾰ~ε~ωκωσῐν";
    perfect_opt_act_1: "~ᾰ~ε~ωκοιμῐ|~ᾰ~ε~ωκοιην", _, _;
    perfect_opt_act_2: "~ᾰ~ε~ωκοις|~ᾰ~ε~ωκοιης", _, _;
    perfect_opt_act_3: "~ᾰ~ε~ωκοι|~ᾰ~ε~ωκοιη", _, _;
    perfect_imp_act_2: "~ᾰ~ε~ωκε", _, _;
    perfect_imp_mid_2: _, _, "~ᾰ~ε~ωσθε";
    perfect_imp_mid_3: _, _, "~ᾰ~ε~ωσθων";
    perfect_imp_pass_2: "~ᾰ~ε~ωσο", _, "~ᾰ~ε~ωσθε";
    perfect_imp_pass_3: "~ᾰ~ε~ωσθω", "~ᾰ~ε~ωσθων", "~ᾰ~ε~ωσθων";
    pluperfect_mid_1: "~ᾰ~ε~ωμην", _, _;
    pluperfect_mid_2: "~ᾰ~ε~ωσο", "~ᾰ~ε~ωσθον", _;
    pluperfect_mid_3: "~ᾰ~ε~ωτο", _, "~ᾰ~ε~ωντο";
    pluperfect_pass_1: _, _, "~ᾰ~ε~ωμεθᾰ";
    pluperfect_pass_2: _, _, "~ᾰ~ε~ωσθε";
    pluperfect_pass_3: _, "~ᾰ~ε~ωσθην", _;
    infinitive_pres: "~ᾰ~ᾰ~ουν", _, "~ᾰ~ᾰ~ουσθαι";
    infinitive_fut: "~ᾰ~ᾰ~ωσειν", _, _;
    infinitive_aor: "~ᾰ~ᾰ~ωσαι", _, _;
    infinitive_perf: _, "~ᾰ~ε~ωσθαι", _;
    participle_aor: "~ᾰ~ᾰ~ωσᾱς", "~ᾰ~ᾰ~ωσᾰμενος", _;
    participle_perf: "~ᾰ~ε~ωκως", _, "~ᾰ~ε~ωμενος";
    ),
    // Λαλαχευομαι: 1 words (λαλαχευομαι). Like Ἀποστατεω, except in 62 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("λαλαχευομαι", 1, "~α~α~ομαι", like Ἀποστατεω;
    stem imperfect: "~ᾰ~ᾰ~- like λαλαχευομαι";
    stem future_pass: "~ᾰ~ᾰ~θησ- like χρημᾰτῐζω";
    stem aorist_ind_pass: "~ᾰ~ᾰ~θην- like ἐχω";
    stem aorist_pass: "~ᾰ~ᾰ~θ- like ποιεω";
    stem perfect: "~ε~ᾰλᾰ~- like λαλαχευομαι";
    stem pluperfect: "~ε~ᾰλᾰ~- like λαλαχευομαι";
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
    // Μικρογρᾰφεω: 1 words (μικρογρᾰφεω). Like Νισσομαι, except in 50 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("μικρογρᾰφεω", 1, "~ι~εω", like Νισσομαι;
    ind_act_1: "~ῐ~ω", _, "~ῐ~ουμεν";
    ind_act_2: "~ῐ~εις", "~ῐ~ειτον", _;
    ind_act_3: "~ῐ~ει", _, "~ῐ~ουσῐ|~ῐ~ουσῐν";
    ind_mid_1: derived, _, "~ῐ~ουμεθᾰ";
    ind_mid_2: "~ῐ~ει|~ῐ~ῃ", _, _;
    ind_mid_3: _, "~ῐ~εισθον", _;
    ind_pass_1: "~ῐ~ουμαι", _, _;
    ind_pass_2: _, "~ῐ~εισθον", "~ῐ~εισθε";
    ind_pass_3: "~ῐ~ειται", "~ῐ~εισθον", "~ῐ~ουνται";
    subj_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    subj_act_2: "~ῐ~ῃς", _, _;
    subj_act_3: "~ῐ~ῃ", "~ῐ~ητον", "~ῐ~ωσῐ|~ῐ~ωσῐν";
    subj_mid_1: derived, _, _;
    subj_mid_2: derived, _, _;
    subj_pass_1: "~ῐ~ωμαι", _, _;
    subj_pass_2: derived, _, _;
    opt_act_1: "~ῐ~οιην|~ῐ~οιμῐ", _, "~ῐ~οιμεν|~ῐ~οιημεν";
    opt_act_2: "~ῐ~οιης|~ῐ~οις", "~ῐ~οιτον|~ῐ~οιητον", "~ῐ~οιτε|~ῐ~οιητε";
    opt_act_3: "~ῐ~οιη|~ῐ~οι", "~ῐ~οιτην|~ῐ~οιητην", "~ῐ~οιεν|~ῐ~οιησᾰν";
    imp_act_2: "~ῐ~ει", _, "~ῐ~ειτε";
    imp_act_3: _, _, "~ῐ~ουντων";
    imp_mid_2: _, "~ῐ~εισθον", "~ῐ~εισθε";
    imp_mid_3: derived, _, "~ῐ~εισθων";
    imp_pass_2: _, "~ῐ~εισθον", "~ῐ~εισθε";
    imp_pass_3: "~ῐ~εισθω", "~ῐ~εισθων", "~ῐ~εισθων";
    infinitive_pres: "~ῐ~ειν", _, "~ῐ~εισθαι";
    participle_pres: "~ῐ~ων", _, "~ῐ~ουμενος";
    ),
    // Λῐνοστᾰτεω: 1 words (λῐνοστᾰτεω). Like Ἐπῐκᾰλεω, except in 97 cells: imperfect_act_1 sg, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, imperfect_act_3 pl, ….
    conjugation!("λῐνοστᾰτεω", 1, "~ῐ~ᾰ~εω", like Ἐπῐκᾰλεω;
    stem future_pass: "~ι~α~ηθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ι~α~ησ- like ποιεω";
    stem aorist_ind_pass: "~ι~α~ηθην- like ἐχω";
    stem aorist_pass: "~ι~α~ηθ- like ποιεω";
    stem pluperfect_act: "~ελι~α~ηκ- like δῐκεω";
    imperfect_act_1: "ἐ~ι~α~ουν", _, _;
    imperfect_act_2: "ἐ~ι~α~εις", "ἐ~ι~α~ειτον", "ἐ~ι~α~ειτε";
    imperfect_act_3: "ἐ~ι~α~ει", _, "ἐ~ι~α~ουν";
    imperfect_mid_1: "ἐ~ι~α~ουμην", _, "ἐ~ι~α~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ι~α~ου", _, _;
    imperfect_mid_3: "ἐ~ι~α~ειτο", _, "ἐ~ι~α~ουντο";
    imperfect_pass_2: _, "ἐ~ι~α~εισθον", "ἐ~ι~α~εισθε";
    imperfect_pass_3: _, "ἐ~ι~α~εισθην", _;
    future_ind_act_1: "~ι~α~ησω", _, "~ι~α~ησομεν";
    future_ind_act_2: "~ι~α~ησεις", "~ι~α~ησετον", _;
    future_ind_act_3: _, _, "~ι~α~ησουσῐ|~ι~α~ησουσῐν";
    future_ind_mid_1: "~ι~α~ησομαι", _, _;
    future_ind_mid_2: "~ι~α~ησῃ|~ι~α~ησει", "~ι~α~ησεσθον", _;
    future_ind_mid_3: "~ι~α~ησεται", _, _;
    future_opt_act_1: "~ι~α~ησοιμῐ", _, "~ι~α~ησοιμεν";
    future_opt_act_2: "~ι~α~ησοις", "~ι~α~ησοιτον", derived;
    future_opt_act_3: "~ι~α~ησοι", "~ι~α~ησοιτην", derived;
    future_opt_mid_1: "~ι~α~ησοιμην", _, "~ι~α~ησοιμεθᾰ";
    future_opt_mid_3: _, "~ι~α~ησοισθην", _;
    aorist_subj_act_1: "~ι~α~ησω", _, _;
    aorist_subj_act_2: "~ι~α~ησῃς", "~ι~α~ησητον", _;
    aorist_subj_act_3: _, _, "~ι~α~ησωσῐ|~ι~α~ησωσῐν";
    aorist_subj_mid_1: "~ι~α~ησωμαι", _, _;
    aorist_subj_mid_2: "~ι~α~ησῃ", "~ι~α~ησησθον", _;
    aorist_opt_act_1: "~ι~α~ησαιμῐ", _, "~ι~α~ησαιμεν";
    aorist_opt_act_2: "~ι~α~ησειᾰς|~ι~α~ησαις", _, _;
    aorist_opt_act_3: "~ι~α~ησειε|~ι~α~ησειεν|~ι~α~ησαι", _, "~ι~α~ησειᾰν|~ι~α~ησαιεν";
    aorist_opt_mid_1: "~ι~α~ησαιμην", _, _;
    aorist_imp_act_2: "~ι~α~ησον", _, "~ι~α~ησᾰτε";
    aorist_imp_act_3: _, _, "~ι~α~ησᾰντων";
    aorist_imp_mid_2: "~ι~α~ησαι", "~ι~α~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ι~α~ησᾰσθων";
    perfect_ind_act_1: "~ελι~α~ηκᾰ", _, "~ελι~α~ηκᾰμεν";
    perfect_ind_act_2: _, "~ελι~α~ηκᾰτον", _;
    perfect_ind_act_3: "~ελι~α~ηκε|~ελι~α~ηκεν", _, "~ελι~α~ηκᾱσῐ|~ελι~α~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "~ελι~α~ημεθᾰ";
    perfect_ind_mid_2: "~ελι~α~ησαι", "~ελι~α~ησθον", _;
    perfect_ind_mid_3: "~ελι~α~ηται", _, "~ελι~α~ηνται";
    perfect_ind_pass_1: "~ελι~α~ημαι", _, _;
    perfect_ind_pass_2: _, _, "~ελι~α~ησθε";
    perfect_subj_act_1: "~ελι~α~ηκω", _, _;
    perfect_subj_act_2: "~ελι~α~ηκῃς", "~ελι~α~ηκητον", "~ελι~α~ηκητε";
    perfect_subj_act_3: "~ελι~α~ηκῃ", _, "~ελι~α~ηκωσῐ|~ελι~α~ηκωσῐν";
    perfect_opt_act_1: "~ελι~α~ηκοιμῐ|~ελι~α~ηκοιην", _, _;
    perfect_opt_act_2: "~ελι~α~ηκοις|~ελι~α~ηκοιης", _, _;
    perfect_opt_act_3: "~ελι~α~ηκοι|~ελι~α~ηκοιη", _, _;
    perfect_imp_act_2: "~ελι~α~ηκε", _, _;
    perfect_imp_mid_2: _, _, "~ελι~α~ησθε";
    perfect_imp_mid_3: _, _, "~ελι~α~ησθων";
    perfect_imp_pass_2: "~ελι~α~ησο", _, "~ελι~α~ησθε";
    perfect_imp_pass_3: "~ελι~α~ησθω", "~ελι~α~ησθων", "~ελι~α~ησθων";
    pluperfect_mid_1: "ἐ~ελι~α~ημην", _, _;
    pluperfect_mid_2: "ἐ~ελι~α~ησο", "ἐ~ελι~α~ησθον", _;
    pluperfect_mid_3: "ἐ~ελι~α~ητο", _, "ἐ~ελι~α~ηντο";
    pluperfect_pass_1: _, _, "ἐ~ελι~α~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ελι~α~ησθε";
    pluperfect_pass_3: _, "ἐ~ελι~α~ησθην", _;
    infinitive_fut: "~ι~α~ησειν", _, _;
    infinitive_aor: "~ι~α~ησαι", _, _;
    infinitive_perf: _, "~ελι~α~ησθαι", _;
    participle_aor: "~ι~α~ησᾱς", "~ι~α~ησᾰμενος", _;
    participle_perf: "~ελι~α~ηκως", _, "~ελι~α~ημενος";
    ),
    // Ἀδησᾰ: 1 words (ἀδησᾰ). Like Ἐπορον, except in 31 cells: ind_act_1 sg, aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, aorist_opt_act_1 sg, ….
    conjugation!("ἀδησᾰ", 1, "~σᾰ", like Ἐπορον;
    stem aorist_ind: "~σ- like ξυνναυμᾰχεω";
    ind_act_1: "~σᾰ", _, _;
    aorist_subj_act_1: "~σω", _, _;
    aorist_subj_act_2: "~σῃς", "~σητον", _;
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_opt_act_1: "~σαιμῐ", _, "~σαιμεν";
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", _, "~σειᾰν|~σαιεν";
    aorist_imp_act_2: "~σον", _, "~σᾰτε";
    aorist_imp_act_3: _, _, "~σᾰντων";
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: _, "~κᾰτον", _;
    perfect_ind_act_3: "~κε|~κεν", _, "~κᾱσῐ|~κᾱσῐν";
    perfect_subj_act_1: "~κω", _, _;
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ|~κωσῐν";
    perfect_opt_act_1: "~κοιμῐ|~κοιην", _, _;
    perfect_opt_act_2: "~κοις|~κοιης", _, _;
    perfect_opt_act_3: "~κοι|~κοιη", _, _;
    perfect_imp_act_2: "~κε", _, _;
    infinitive_aor: "~σαι", _, _;
    participle_aor: "~σᾱς", _, _;
    participle_perf: "~κως", _, _;
    ),
    // Ὑποκοριζομαι: 1 words (ὑποκοριζομαι). Like Δῐακειμαι, except in 45 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, subj_mid_1 sg, ….
    conjugation!("ὑποκοριζομαι", 1, "~ο~ιζομαι", like Δῐακειμαι;
    stem imperfect: "~ε~ῐζ- like ὑποκοριζομαι";
    stem aorist_ind: "~ε~ῐσ- like μηκᾰομαι";
    stem pluperfect: "~ε%ε~ῐ- like ὑποκοριζομαι";
    ind_act_1: "~ο~ιζομαι", _, _;
    ind_mid_1: "~ο~ῐζομαι", _, "~ο~ῐζομεθᾰ";
    ind_mid_2: "~ο~ῐζῃ|~ο~ῐζει", _, _;
    ind_mid_3: _, "~ο~ῐζεσθον", _;
    subj_mid_1: "~ο~ῐζωμαι", _, "~ο~ῐζωμεθᾰ";
    subj_mid_2: "~ο~ῐζῃ", "~ο~ῐζησθον", _;
    subj_mid_3: "~ο~ῐζηται", _, "~ο~ῐζωνται";
    opt_mid_1: "~ο~ῐζοιμην", _, "~ο~ῐζοιμεθᾰ";
    opt_mid_2: "~ο~ῐζοιο", _, "~ο~ῐζοισθε";
    opt_mid_3: "~ο~ῐζοιτο", "~ο~ῐζοισθην", "~ο~ῐζοιντο";
    imp_mid_2: "~ο~ῐζου", "~ο~ῐζεσθον", "~ο~ῐζεσθε";
    imp_mid_3: "~ο~ῐζεσθω", _, "~ο~ῐζεσθων";
    future_ind_mid_1: "~ο~ῐουμαι", _, _;
    future_ind_mid_2: "~ο~ῐει|~ο~ῐῃ", "~ο~ῐεισθον", _;
    future_ind_mid_3: "~ο~ῐειται", _, _;
    future_opt_mid_1: "~ο~ῐοιμην", _, "~ο~ῐοιμεθᾰ";
    future_opt_mid_3: _, "~ο~ῐοισθην", _;
    aorist_subj_mid_1: "~ο~ῐσωμαι", _, _;
    aorist_subj_mid_2: "~ο~ῐσῃ", "~ο~ῐσησθον", _;
    aorist_opt_mid_1: "~ο~ῐσαιμην", _, _;
    aorist_imp_mid_2: "~ο~ῐσαι", "~ο~ῐσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ο~ῐσᾰσθων";
    perfect_ind_mid_1: _, _, "~ο%ε~ῐσμεθᾰ";
    perfect_ind_mid_2: "~ο%ε~ῐσαι", "~ο%ε~ῐσθον", _;
    perfect_ind_mid_3: "~ο%ε~ῐσται", _, "~ο%ε~ῐδᾰται";
    perfect_imp_mid_2: _, _, "~ο%ε~ῐσθε";
    perfect_imp_mid_3: _, _, "~ο%ε~ῐσθων";
    participle_aor: _, "~ο~ῐσᾰμενος", _;
    ),
    // Ἰχᾰνᾰω: 1 words (ἰχᾰνᾰω). Like Μικρογρᾰφεω, except in 59 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἰχᾰνᾰω", 1, "~ᾰ~ᾰω", like Μικρογρᾰφεω;
    ind_act_1: "~α~ω", _, "~α~ωμεν";
    ind_act_2: "~α~ᾳς", "~α~ᾱτον", _;
    ind_act_3: "~α~ᾳ", _, "~α~ωσῐ|~α~ωσῐν";
    ind_mid_1: _, _, "~α~ωμεθᾰ";
    ind_mid_2: "~α~ᾳ", _, _;
    ind_mid_3: _, "~α~ᾱσθον", _;
    ind_pass_1: "~α~ωμαι", _, _;
    ind_pass_2: _, "~α~ᾱσθον", "~α~ᾱσθε";
    ind_pass_3: "~α~ᾱται", "~α~ᾱσθον", "~α~ωνται";
    subj_act_1: "~α~ω", _, "~α~ωμεν";
    subj_act_2: "~α~ᾳς", _, _;
    subj_act_3: "~α~ᾳ", "~α~ᾱτον", "~α~ωσῐ|~α~ωσῐν";
    subj_mid_1: _, _, "~α~ωμεθᾰ";
    subj_mid_2: _, "~α~ᾱσθον", _;
    subj_mid_3: "~α~ᾱται", _, "~α~ωνται";
    subj_pass_1: "~α~ωμαι", _, _;
    subj_pass_2: _, _, "~α~ᾱσθε";
    opt_act_1: "~α~ῳην|~α~ῳμῐ", _, "~α~ῳμεν|~α~ῳημεν";
    opt_act_2: "~α~ῳης|~α~ῳς", "~α~ῳτον|~α~ῳητον", "~α~ῳτε|~α~ῳητε";
    opt_act_3: "~α~ῳη|~α~ῳ", "~α~ῳτην|~α~ῳητην", "~α~ῳεν|~α~ῳησᾰν";
    opt_mid_1: "~α~ῳμην", _, "~α~ῳμεθᾰ";
    opt_mid_2: "~α~ῳο", _, "~α~ῳσθε";
    opt_mid_3: "~α~ῳτο", "~α~ῳσθην", "~α~ῳντο";
    opt_pass_2: _, "~α~ῳσθον", _;
    imp_act_2: "~α~ᾱ", _, "~α~ᾱτε";
    imp_act_3: _, _, "~α~ωντων";
    imp_mid_2: "~α~ω", "~α~ᾱσθον", "~α~ᾱσθε";
    imp_mid_3: _, _, "~α~ᾱσθων";
    imp_pass_2: _, "~α~ᾱσθον", "~α~ᾱσθε";
    imp_pass_3: "~α~ᾱσθω", "~α~ᾱσθων", "~α~ᾱσθων";
    infinitive_pres: "~α~ᾱν", _, "~α~ᾱσθαι";
    participle_pres: "~α~ων", _, "~α~ωμενος";
    ),
    // Ᾰντᾰποκρῑνομαι: 1 words (ᾰντᾰποκρῑνομαι). Like Δῐακειμαι, except in 80 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ᾰντᾰποκρῑνομαι", 1, "ᾰ~ο~ῑνομαι", like Δῐακειμαι;
    stem future_pass: "~ο~ῐθησ- like ᾰντᾰποκρῑνομαι";
    stem aorist_pass: "~ο~ῐθ- like ᾰντᾰποκρῑνομαι";
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
    opt_mid_2: "ἀ~ο~ῑνοιο", _, "ἀ~ο~ῑνοισθε";
    opt_mid_3: "ἀ~ο~ῑνοιτο", "ἀ~ο~ῑνοισθην", "ἀ~ο~ῑνοιντο";
    opt_pass_2: _, "ἀ~ο~ῑνοισθον", _;
    imp_mid_2: "ἀ~ο~ῑνου", "ἀ~ο~ῑνεσθον", "ἀ~ο~ῑνεσθε";
    imp_mid_3: "ἀ~ο~ῑνεσθω", _, "ἀ~ο~ῑνεσθων";
    imp_pass_2: _, "ἀ~ο~ῑνεσθον", "ἀ~ο~ῑνεσθε";
    imp_pass_3: _, "ἀ~ο~ῑνεσθων", "ἀ~ο~ῑνεσθων";
    imperfect_mid_1: "ἀ~ε~ῑνομην", _, "ἀ~ε~ῑνομεθᾰ";
    imperfect_mid_2: "ἀ~ε~ῑνου", "ἀ~ε~ῑνεσθον", _;
    imperfect_mid_3: "ἀ~ε~ῑνετο", _, "ἀ~ε~ῑνοντο";
    imperfect_pass_2: _, _, "ἀ~ε~ῑνεσθε";
    imperfect_pass_3: _, "ἀ~ε~ῑνεσθην", _;
    future_ind_mid_1: "ἀ~ο~ῐνουμαι", _, _;
    future_ind_mid_2: "ἀ~ο~ῐνει|ἀ~ο~ῐνῃ", "ἀ~ο~ῐνεισθον", _;
    future_ind_mid_3: "ἀ~ο~ῐνειται", _, _;
    future_opt_mid_1: "ἀ~ο~ῐνοιμην", _, "ἀ~ο~ῐνοιμεθᾰ";
    future_opt_mid_3: _, "ἀ~ο~ῐνοισθην", _;
    aorist_ind_mid_1: "ἀ~ε~ῑνᾰμην", _, _;
    aorist_ind_mid_2: "ἀ~ε~ῑνω", "ἀ~ε~ῑνᾰσθον", _;
    aorist_ind_pass_1: "ἀ~ε~ῐθην", _, _;
    aorist_subj_mid_1: "ἀ~ο~ῑνωμαι", _, _;
    aorist_subj_mid_2: "ἀ~ο~ῑνῃ", "ἀ~ο~ῑνησθον", _;
    aorist_opt_mid_1: "ἀ~ο~ῑναιμην", _, _;
    aorist_imp_mid_2: "ἀ~ο~ῑναι", "ἀ~ο~ῑνᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἀ~ο~ῑνᾰσθων";
    perfect_ind_mid_1: _, _, "ἀ~ο%ε~ῐμεθᾰ";
    perfect_ind_mid_2: "ἀ~ο%ε~ῐσαι", "ἀ~ο%ε~ῐσθον", _;
    perfect_ind_mid_3: "ἀ~ο%ε~ῐται", _, "ἀ~ο%ε~ῐνται";
    perfect_ind_pass_1: "ἀ~ο%ε~ῐμαι", _, _;
    perfect_ind_pass_2: _, _, "ἀ~ο%ε~ῐσθε";
    perfect_imp_mid_2: _, _, "ἀ~ο%ε~ῐσθε";
    perfect_imp_mid_3: _, _, "ἀ~ο%ε~ῐσθων";
    perfect_imp_pass_2: "ἀ~ο%ε~ῐσο", _, "ἀ~ο%ε~ῐσθε";
    perfect_imp_pass_3: "ἀ~ο%ε~ῐσθω", "ἀ~ο%ε~ῐσθων", "ἀ~ο%ε~ῐσθων";
    infinitive_pres: _, _, "ἀ~ο~ῑνεσθαι";
    infinitive_perf: _, "ἀ~ο%ε~ῐσθαι", _;
    participle_pres: _, _, "ἀ~ο~ῑνομενος";
    participle_aor: _, "ἀ~ο~ῑνᾰμενος", _;
    participle_perf: _, _, "ἀ~ο%ε~ῐμενος";
    ),
    // Ὑπεκτιθεμαι: 1 words (ὑπεκτιθεμαι). Like Δῐακειμαι, except in 80 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ὑπεκτιθεμαι", 1, "~κτι~εμαι", like Δῐακειμαι;
    stem future_pass: "~κ%ε~ησ- like χρημᾰτῐζω";
    stem aorist_ind_pass: "~ξετε~ην- like εὐλογεω";
    stem aorist_pass: "~κτε~- like ὑπεκτιθεμαι";
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
    imperfect_mid_1: "~ξε%ῐ~εμην", _, "~ξε%ῐ~εμεθᾰ";
    imperfect_mid_2: "~ξε%ῐ~εσο", "~ξε%ῐ~εσθον", _;
    imperfect_mid_3: "~ξε%ῐ~ετο", _, "~ξε%ῐ~εντο";
    imperfect_pass_2: _, _, "~ξε%ῐ~εσθε";
    imperfect_pass_3: _, "~ξε%ῐ~εσθην", _;
    future_ind_mid_1: "~κ~ησομαι", _, _;
    future_ind_mid_2: "~κ~ησῃ|~κ~ησει", "~κ~ησεσθον", _;
    future_ind_mid_3: "~κ~ησεται", _, _;
    future_opt_mid_1: "~κ~ησοιμην", _, "~κ~ησοιμεθᾰ";
    future_opt_mid_3: _, "~κ~ησοισθην", _;
    aorist_ind_mid_1: "~ξε~εμην", _, _;
    aorist_ind_mid_2: "~ξε~ου", "~ξε~εσθον", _;
    aorist_subj_mid_1: "~κ~ωμαι", _, _;
    aorist_subj_mid_2: "~κ~ῃ", "~κ~ησθον", _;
    aorist_opt_mid_1: "~κ~ειμην", _, _;
    aorist_opt_mid_3: "~κ~ειτο|~κ~οιτο", _, _;
    aorist_imp_mid_2: "~κ~ου", "~κ~εσθον", _;
    aorist_imp_mid_3: _, _, "~κ~εσθων";
    perfect_ind_mid_1: _, _, "~κ%ε~ειμεθᾰ";
    perfect_ind_mid_2: "~κ%ε~εισαι", "~κ%ε~εισθον", _;
    perfect_ind_mid_3: "~κ%ε~ειται", _, "~κ%ε~εινται";
    perfect_ind_pass_1: "~κ%ε~ειμαι", _, _;
    perfect_ind_pass_2: _, _, "~κ%ε~εισθε";
    perfect_imp_mid_2: _, _, "~κ%ε~εισθε";
    perfect_imp_mid_3: _, _, "~κ%ε~εισθων";
    perfect_imp_pass_2: "~κ%ε~εισο", _, "~κ%ε~εισθε";
    perfect_imp_pass_3: "~κ%ε~εισθω", "~κ%ε~εισθων", "~κ%ε~εισθων";
    infinitive_pres: _, _, "~κ%ῐ~εσθαι";
    infinitive_perf: _, "~κ%ε~εισθαι", _;
    participle_pres: _, _, "~κ%ῐ~εμενος";
    participle_aor: _, "~κ~εμενος", _;
    participle_perf: _, _, "~κ%ε~ειμενος";
    ),
    // Μᾰστῐγοω: 1 words (μᾰστῐγοω). Like Ἀποστατεω, except in 67 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("μᾰστῐγοω", 1, "~ᾰ~ῐ~οω", like Ἀποστατεω;
    ind_act_1: "~α~ι~ω", _, "~α~ι~ομεν";
    ind_act_2: "~α~ι~εις", "~α~ι~ετον", _;
    ind_act_3: "~α~ι~ει", _, "~α~ι~ουσῐ|~α~ι~ουσῐν";
    ind_mid_1: _, _, "~α~ι~ομεθᾰ";
    ind_mid_2: "~α~ι~ῃ|~α~ι~ει", _, _;
    ind_mid_3: _, "~α~ι~εσθον", _;
    ind_pass_1: "~α~ι~ομαι", _, _;
    ind_pass_2: _, "~α~ι~εσθον", "~α~ι~εσθε";
    ind_pass_3: "~α~ι~εται", "~α~ι~εσθον", "~α~ι~ονται";
    subj_act_1: "~α~ι~ω", _, "~α~ι~ωμεν";
    subj_act_2: "~α~ι~ῃς", derived, _;
    subj_act_3: "~α~ι~ῃ", "~α~ι~ητον", "~α~ι~ωσῐ|~α~ι~ωσῐν";
    subj_mid_1: _, _, "~α~ι~ωμεθᾰ";
    subj_mid_2: _, "~α~ι~ησθον", _;
    subj_mid_3: "~α~ι~ηται", _, "~α~ι~ωνται";
    subj_pass_1: "~α~ι~ωμαι", _, _;
    subj_pass_2: _, _, "~α~ι~ησθε";
    opt_act_1: "~α~ι~οιμῐ", _, "~α~ι~οιμεν";
    opt_act_2: "~α~ι~οις", "~α~ι~οιτον", derived;
    opt_act_3: "~α~ι~οι", "~α~ι~οιτην", "~α~ι~οιεν";
    opt_mid_1: "~α~ι~οιμην", _, "~α~ι~οιμεθᾰ";
    opt_mid_2: "~α~ι~οιο", _, "~α~ι~οισθε";
    opt_mid_3: "~α~ι~οιτο", "~α~ι~οισθην", "~α~ι~οιντο";
    opt_pass_2: _, "~α~ι~οισθον", _;
    imp_act_2: "~α~ι~ε", _, "~α~ι~ετε";
    imp_act_3: _, _, "~α~ι~οντων";
    imp_mid_2: "~α~ι~ου", "~α~ι~εσθον", "~α~ι~εσθε";
    imp_mid_3: _, _, "~α~ι~εσθων";
    imp_pass_2: _, "~α~ι~εσθον", "~α~ι~εσθε";
    imp_pass_3: "~α~ι~εσθω", "~α~ι~εσθων", "~α~ι~εσθων";
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, _, derived;
    infinitive_pres: "~α~ι~ειν", _, "~α~ι~εσθαι";
    participle_pres: "~α~ι~ων", _, "~α~ι~ομενος";
    ),
    // Βᾰρβᾰροομαι: 1 words (βᾰρβᾰροομαι). Like Ἀποστατεω, except in 39 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("βᾰρβᾰροομαι", 1, "~ᾰ~ᾰροομ~ι", like Ἀποστατεω;
    stem aorist_ind_pass: "~α~~ρωθην- like ἐχω";
    stem aorist_pass: "~α~~ρωθ- like ποιεω";
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
    perfect_ind_mid_1: _, _, "~εβα~~ρωμεθᾰ";
    perfect_ind_mid_2: "~εβα~~ρωσαι", "~εβα~~ρωσθον", _;
    perfect_ind_mid_3: "~εβα~~ρωται", _, "~εβα~~ρωνται";
    perfect_imp_mid_2: _, _, "~εβα~~ρωσθε";
    perfect_imp_mid_3: _, _, "~εβα~~ρωσθων";
    infinitive_pres: derived, _, _;
    participle_pres: derived, _, _;
    ),
    // Ἐπιμιμνησκομαι: 1 words (ἐπιμιμνησκομαι). Like Δῐακειμαι, except in 76 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_3 du, ind_pass_2 du, ….
    conjugation!("ἐπιμιμνησκομαι", 1, "~ιμι~σκομαι", like Δῐακειμαι;
    stem future_pass: "~ῐ~σθησ- like χρημᾰτῐζω";
    stem aorist_ind: "~ε~σ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ε~σθην- like εὐλογεω";
    stem aorist_pass: "~ῐ~σθ- like ποιεω";
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
    opt_mid_2: "~ῐ%ῐ~σκοιο", _, "~ῐ%ῐ~σκοισθε";
    opt_mid_3: "~ῐ%ῐ~σκοιτο", "~ῐ%ῐ~σκοισθην", "~ῐ%ῐ~σκοιντο";
    opt_pass_2: _, "~ῐ%ῐ~σκοισθον", _;
    imp_mid_2: "~ῐ%ῐ~σκου", "~ῐ%ῐ~σκεσθον", "~ῐ%ῐ~σκεσθε";
    imp_mid_3: "~ῐ%ῐ~σκεσθω", _, "~ῐ%ῐ~σκεσθων";
    imp_pass_2: _, "~ῐ%ῐ~σκεσθον", "~ῐ%ῐ~σκεσθε";
    imp_pass_3: _, "~ῐ%ῐ~σκεσθων", "~ῐ%ῐ~σκεσθων";
    imperfect_mid_1: "~ε%ῐ~σκομην", _, "~ε%ῐ~σκομεθᾰ";
    imperfect_mid_2: "~ε%ῐ~σκου", "~ε%ῐ~σκεσθον", _;
    imperfect_mid_3: "~ε%ῐ~σκετο", _, "~ε%ῐ~σκοντο";
    imperfect_pass_2: _, _, "~ε%ῐ~σκεσθε";
    imperfect_pass_3: _, "~ε%ῐ~σκεσθην", _;
    future_ind_mid_1: "~ῐ~σομαι", _, _;
    future_ind_mid_2: "~ῐ~σῃ|~ῐ~σει", "~ῐ~σεσθον", _;
    future_ind_mid_3: "~ῐ~σεται", _, _;
    future_opt_mid_1: "~ῐ~σοιμην", _, "~ῐ~σοιμεθᾰ";
    future_opt_mid_3: _, "~ῐ~σοισθην", _;
    aorist_subj_mid_1: "~ῐ~σωμαι", _, _;
    aorist_subj_mid_2: "~ῐ~σῃ", "~ῐ~σησθον", _;
    aorist_opt_mid_1: "~ῐ~σαιμην", _, _;
    aorist_imp_mid_2: "~ῐ~σαι", "~ῐ~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῐ~σᾰσθων";
    perfect_ind_mid_1: _, _, "~ῐ%ε~μεθᾰ";
    perfect_ind_mid_2: "~ῐ%ε~σαι", "~ῐ%ε~σθον", _;
    perfect_ind_mid_3: "~ῐ%ε~ται", _, "~ῐ%ε~νται";
    perfect_ind_pass_1: "~ῐ%ε~μαι", _, _;
    perfect_ind_pass_2: _, _, "~ῐ%ε~σθε";
    perfect_imp_mid_2: _, _, "~ῐ%ε~σθε";
    perfect_imp_mid_3: _, _, "~ῐ%ε~σθων";
    perfect_imp_pass_2: "~ῐ%ε~σο", _, "~ῐ%ε~σθε";
    perfect_imp_pass_3: "~ῐ%ε~σθω", "~ῐ%ε~σθων", "~ῐ%ε~σθων";
    infinitive_pres: _, _, "~ῐ%ῐ~σκεσθαι";
    infinitive_perf: _, "~ῐ%ε~σθαι", _;
    participle_pres: _, _, "~ῐ%ῐ~σκομενος";
    participle_aor: _, "~ῐ~σᾰμενος", _;
    participle_perf: _, _, "~ῐ%ε~μενος";
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
    opt_mid_2: "ἐ~οιο", _, "ἐ~οισθε";
    opt_mid_3: "ἐ~οιτο", "ἐ~οισθην", "ἐ~οιντο";
    opt_pass_2: _, "ἐ~οισθον", _;
    imp_act_2: "ἐ~ει", _, "ἐ~ειτε";
    imp_act_3: _, _, "ἐ~ουντων";
    imp_mid_2: "ἐ~ου", "ἐ~εισθον", "ἐ~εισθε";
    imp_mid_3: _, _, "ἐ~εισθων";
    imp_pass_2: _, "ἐ~εισθον", "ἐ~εισθε";
    imp_pass_3: "ἐ~εισθω", "ἐ~εισθων", "ἐ~εισθων";
    infinitive_pres: "ἐ~ειν", _, "ἐ~εισθαι";
    participle_pres: "ἐ~ων", _, "ἐ~ουμενος";
    ),
    // Ᾰνθρωπεω: 1 words (ᾰνθρωπεω). Like Ἐρωτᾰω, except in 129 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ᾰνθρωπεω", 1, "ᾰ~εω", like Ἐρωτᾰω;
    stem future_pass: "~ηθησ- like ᾰντᾰποκρῑνομαι";
    stem aorist_ind: "~ησ- like ἐλπῐζω";
    stem aorist_pass: "~ηθ- like ᾰντᾰποκρῑνομαι";
    ind_act_1: "ἀ~ω", _, "ἀ~ουμεν";
    ind_act_2: "ἀ~εις", "ἀ~ειτον", _;
    ind_act_3: "ἀ~ει", _, "ἀ~ουσῐ|ἀ~ουσῐν";
    ind_mid_1: _, _, "ἀ~ουμεθᾰ";
    ind_mid_2: "ἀ~ει|ἀ~ῃ", _, _;
    ind_mid_3: _, "ἀ~εισθον", _;
    ind_pass_1: "ἀ~ουμαι", _, _;
    ind_pass_2: _, "ἀ~εισθον", "ἀ~εισθε";
    ind_pass_3: "ἀ~ειται", "ἀ~εισθον", "ἀ~ουνται";
    subj_act_1: "ἀ~ω", _, "ἀ~ωμεν";
    subj_act_2: "ἀ~ῃς", _, _;
    subj_act_3: "ἀ~ῃ", "ἀ~ητον", "ἀ~ωσῐ|ἀ~ωσῐν";
    subj_mid_1: _, _, "ἀ~ωμεθᾰ";
    subj_mid_2: _, "ἀ~ησθον", _;
    subj_mid_3: "ἀ~ηται", _, "ἀ~ωνται";
    subj_pass_1: "ἀ~ωμαι", _, _;
    subj_pass_2: _, _, "ἀ~ησθε";
    opt_act_1: "ἀ~οιην|ἀ~οιμῐ", _, "ἀ~οιμεν|ἀ~οιημεν";
    opt_act_2: "ἀ~οιης|ἀ~οις", "ἀ~οιτον|ἀ~οιητον", "ἀ~οιτε|ἀ~οιητε";
    opt_act_3: "ἀ~οιη|ἀ~οι", "ἀ~οιτην|ἀ~οιητην", "ἀ~οιεν|ἀ~οιησᾰν";
    opt_mid_1: "ἀ~οιμην", _, "ἀ~οιμεθᾰ";
    opt_mid_2: "ἀ~οιο", _, "ἀ~οισθε";
    opt_mid_3: "ἀ~οιτο", "ἀ~οισθην", "ἀ~οιντο";
    opt_pass_2: _, "ἀ~οισθον", _;
    imp_act_2: "ἀ~ει", _, "ἀ~ειτε";
    imp_act_3: _, _, "ἀ~ουντων";
    imp_mid_2: "ἀ~ου", "ἀ~εισθον", "ἀ~εισθε";
    imp_mid_3: _, _, "ἀ~εισθων";
    imp_pass_2: _, "ἀ~εισθον", "ἀ~εισθε";
    imp_pass_3: "ἀ~εισθω", "ἀ~εισθων", "ἀ~εισθων";
    imperfect_act_1: "ἠ~ουν", _, _;
    imperfect_act_2: "ἠ~εις", "ἠ~ειτον", "ἠ~ειτε";
    imperfect_act_3: "ἠ~ει", _, "ἠ~ουν";
    imperfect_mid_1: "ἠ~ουμην", _, "ἠ~ουμεθᾰ";
    imperfect_mid_2: "ἠ~ου", _, _;
    imperfect_mid_3: "ἠ~ειτο", _, "ἠ~ουντο";
    imperfect_pass_2: _, "ἠ~εισθον", "ἠ~εισθε";
    imperfect_pass_3: _, "ἠ~εισθην", _;
    future_ind_act_1: "ἀ~ησω", _, "ἀ~ησομεν";
    future_ind_act_2: "ἀ~ησεις", "ἀ~ησετον", _;
    future_ind_act_3: _, _, "ἀ~ησουσῐ|ἀ~ησουσῐν";
    future_ind_mid_1: "ἀ~ησομαι", _, _;
    future_ind_mid_2: "ἀ~ησῃ|ἀ~ησει", "ἀ~ησεσθον", _;
    future_ind_mid_3: "ἀ~ησεται", _, _;
    future_opt_act_1: "ἀ~ησοιμῐ", _, "ἀ~ησοιμεν";
    future_opt_act_2: "ἀ~ησοις", "ἀ~ησοιτον", _;
    future_opt_act_3: "ἀ~ησοι", "ἀ~ησοιτην", _;
    future_opt_mid_1: "ἀ~ησοιμην", _, "ἀ~ησοιμεθᾰ";
    future_opt_mid_3: _, "ἀ~ησοισθην", _;
    aorist_subj_act_1: "ἀ~ησω", _, _;
    aorist_subj_act_2: "ἀ~ησῃς", "ἀ~ησητον", _;
    aorist_subj_act_3: _, _, "ἀ~ησωσῐ|ἀ~ησωσῐν";
    aorist_subj_mid_1: "ἀ~ησωμαι", _, _;
    aorist_subj_mid_2: "ἀ~ησῃ", "ἀ~ησησθον", _;
    aorist_opt_act_1: "ἀ~ησαιμῐ", _, "ἀ~ησαιμεν";
    aorist_opt_act_2: "ἀ~ησειᾰς|ἀ~ησαις", _, _;
    aorist_opt_act_3: "ἀ~ησειε|ἀ~ησειεν|ἀ~ησαι", _, "ἀ~ησειᾰν|ἀ~ησαιεν";
    aorist_opt_mid_1: "ἀ~ησαιμην", _, _;
    aorist_imp_act_2: "ἀ~ησον", _, "ἀ~ησᾰτε";
    aorist_imp_act_3: _, _, "ἀ~ησᾰντων";
    aorist_imp_mid_2: "ἀ~ησαι", "ἀ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἀ~ησᾰσθων";
    perfect_ind_act_3: "ἠ~ηκε|ἠ~ηκεν", _, "ἠ~ηκᾱσῐ|ἠ~ηκᾱσῐν";
    perfect_subj_act_3: _, _, "ἠ~ηκωσῐ|ἠ~ηκωσῐν";
    perfect_opt_act_1: "ἠ~ηκοιμῐ|ἠ~ηκοιην", _, _;
    perfect_opt_act_2: "ἠ~ηκοις|ἠ~ηκοιης", _, _;
    perfect_opt_act_3: "ἠ~ηκοι|ἠ~ηκοιη", _, _;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_pres: "ἀ~ειν", _, "ἀ~εισθαι";
    infinitive_fut: "ἀ~ησειν", _, _;
    infinitive_aor: "ἀ~ησαι", _, _;
    participle_pres: "ἀ~ων", _, "ἀ~ουμενος";
    participle_aor: "ἀ~ησᾱς", "ἀ~ησᾰμενος", _;
    ),
    // Εισκᾰτοικεω: 1 words (εισκᾰτοικεω). Like Πεμπω, except in 155 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("εισκᾰτοικεω", 1, "ε~οι~εω", like Πεμπω;
    stem future_pass: "~οι~ηθησ- like εισκᾰτοικεω";
    stem aorist_ind: "~ῳ~ησ- like εισκᾰτοικεω";
    stem aorist_pass: "~οι~ηθ- like εισκᾰτοικεω";
    stem pluperfect_act: "~ῳ~ηκ- like εισκᾰτοικεω";
    ind_act_1: "ἑ~οι~ω", _, "ἑ~οι~ουμεν";
    ind_act_2: "ἑ~οι~εις", "ἑ~οι~ειτον", _;
    ind_act_3: "ἑ~οι~ει", _, "ἑ~οι~ουσῐ|ἑ~οι~ουσῐν";
    ind_mid_1: _, _, "ἑ~οι~ουμεθᾰ";
    ind_mid_2: "ἑ~οι~ει|ἑ~οι~ῃ", _, _;
    ind_mid_3: _, "ἑ~οι~εισθον", _;
    ind_pass_1: "ἑ~οι~ουμαι", _, _;
    ind_pass_2: _, "ἑ~οι~εισθον", "ἑ~οι~εισθε";
    ind_pass_3: "ἑ~οι~ειται", "ἑ~οι~εισθον", "ἑ~οι~ουνται";
    subj_act_1: "ἑ~οι~ω", _, "ἑ~οι~ωμεν";
    subj_act_2: "ἑ~οι~ῃς", _, _;
    subj_act_3: "ἑ~οι~ῃ", "ἑ~οι~ητον", "ἑ~οι~ωσῐ|ἑ~οι~ωσῐν";
    subj_mid_1: _, _, "ἑ~οι~ωμεθᾰ";
    subj_mid_2: _, "ἑ~οι~ησθον", _;
    subj_mid_3: "ἑ~οι~ηται", _, "ἑ~οι~ωνται";
    subj_pass_1: "ἑ~οι~ωμαι", _, _;
    subj_pass_2: _, _, "ἑ~οι~ησθε";
    opt_act_1: "ἑ~οι~οιην|ἑ~οι~οιμῐ", _, "ἑ~οι~οιμεν|ἑ~οι~οιημεν";
    opt_act_2: "ἑ~οι~οιης|ἑ~οι~οις", "ἑ~οι~οιτον|ἑ~οι~οιητον", "ἑ~οι~οιτε|ἑ~οι~οιητε";
    opt_act_3: "ἑ~οι~οιη|ἑ~οι~οι", "ἑ~οι~οιτην|ἑ~οι~οιητην", "ἑ~οι~οιεν|ἑ~οι~οιησᾰν";
    opt_mid_1: "ἑ~οι~οιμην", _, "ἑ~οι~οιμεθᾰ";
    opt_mid_2: "ἑ~οι~οιο", _, "ἑ~οι~οισθε";
    opt_mid_3: "ἑ~οι~οιτο", "ἑ~οι~οισθην", "ἑ~οι~οιντο";
    opt_pass_2: _, "ἑ~οι~οισθον", _;
    imp_act_2: "ἑ~οι~ει", _, "ἑ~οι~ειτε";
    imp_act_3: _, _, "ἑ~οι~ουντων";
    imp_mid_2: "ἑ~οι~ου", "ἑ~οι~εισθον", "ἑ~οι~εισθε";
    imp_mid_3: _, _, "ἑ~οι~εισθων";
    imp_pass_2: _, "ἑ~οι~εισθον", "ἑ~οι~εισθε";
    imp_pass_3: "ἑ~οι~εισθω", "ἑ~οι~εισθων", "ἑ~οι~εισθων";
    imperfect_act_1: "ἑ~ῳ~ουν", _, _;
    imperfect_act_2: "ἑ~ῳ~εις", "ἑ~ῳ~ειτον", "ἑ~ῳ~ειτε";
    imperfect_act_3: "ἑ~ῳ~ει", _, "ἑ~ῳ~ουν";
    imperfect_mid_1: "ἑ~ῳ~ουμην", _, "ἑ~ῳ~ουμεθᾰ";
    imperfect_mid_2: "ἑ~ῳ~ου", _, _;
    imperfect_mid_3: "ἑ~ῳ~ειτο", _, "ἑ~ῳ~ουντο";
    imperfect_pass_2: _, "ἑ~ῳ~εισθον", "ἑ~ῳ~εισθε";
    imperfect_pass_3: _, "ἑ~ῳ~εισθην", _;
    future_ind_act_1: "ἑ~οι~ησω", _, "ἑ~οι~ησομεν";
    future_ind_act_2: "ἑ~οι~ησεις", "ἑ~οι~ησετον", _;
    future_ind_act_3: _, _, "ἑ~οι~ησουσῐ|ἑ~οι~ησουσῐν";
    future_ind_mid_1: "ἑ~οι~ησομαι", _, _;
    future_ind_mid_2: "ἑ~οι~ησῃ|ἑ~οι~ησει", "ἑ~οι~ησεσθον", _;
    future_ind_mid_3: "ἑ~οι~ησεται", _, _;
    future_opt_act_1: "ἑ~οι~ησοιμῐ", _, "ἑ~οι~ησοιμεν";
    future_opt_act_2: "ἑ~οι~ησοις", "ἑ~οι~ησοιτον", _;
    future_opt_act_3: "ἑ~οι~ησοι", "ἑ~οι~ησοιτην", _;
    future_opt_mid_1: "ἑ~οι~ησοιμην", _, "ἑ~οι~ησοιμεθᾰ";
    future_opt_mid_3: _, "ἑ~οι~ησοισθην", _;
    aorist_ind_pass_1: "ἑ~ῳ~ηθην", _, _;
    aorist_subj_act_1: "ἑ~οι~ησω", _, _;
    aorist_subj_act_2: "ἑ~οι~ησῃς", "ἑ~οι~ησητον", _;
    aorist_subj_act_3: _, _, "ἑ~οι~ησωσῐ|ἑ~οι~ησωσῐν";
    aorist_subj_mid_1: "ἑ~οι~ησωμαι", _, _;
    aorist_subj_mid_2: "ἑ~οι~ησῃ", "ἑ~οι~ησησθον", _;
    aorist_opt_act_1: "ἑ~οι~ησαιμῐ", _, "ἑ~οι~ησαιμεν";
    aorist_opt_act_2: "ἑ~οι~ησειᾰς|ἑ~οι~ησαις", _, _;
    aorist_opt_act_3: "ἑ~οι~ησειε|ἑ~οι~ησειεν|ἑ~οι~ησαι", _, "ἑ~οι~ησειᾰν|ἑ~οι~ησαιεν";
    aorist_opt_mid_1: "ἑ~οι~ησαιμην", _, _;
    aorist_imp_act_2: "ἑ~οι~ησον", _, "ἑ~οι~ησᾰτε";
    aorist_imp_act_3: _, _, "ἑ~οι~ησᾰντων";
    aorist_imp_mid_2: "ἑ~οι~ησαι", "ἑ~οι~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἑ~οι~ησᾰσθων";
    perfect_ind_act_1: "ἑ~ῳ~ηκᾰ", _, "ἑ~ῳ~ηκᾰμεν";
    perfect_ind_act_2: _, "ἑ~ῳ~ηκᾰτον", _;
    perfect_ind_act_3: "ἑ~ῳ~ηκε|ἑ~ῳ~ηκεν", _, "ἑ~ῳ~ηκᾱσῐ|ἑ~ῳ~ηκᾱσῐν";
    perfect_ind_mid_1: _, _, "ἑ~ῳ~ημεθᾰ";
    perfect_ind_mid_2: "ἑ~ῳ~ησαι", "ἑ~ῳ~ησθον", _;
    perfect_ind_mid_3: "ἑ~ῳ~ηται", _, "ἑ~ῳ~ηνται";
    perfect_ind_pass_1: "ἑ~ῳ~ημαι", _, _;
    perfect_ind_pass_2: _, _, "ἑ~ῳ~ησθε";
    perfect_subj_act_1: "ἑ~ῳ~ηκω", _, _;
    perfect_subj_act_2: "ἑ~ῳ~ηκῃς", "ἑ~ῳ~ηκητον", "ἑ~ῳ~ηκητε";
    perfect_subj_act_3: "ἑ~ῳ~ηκῃ", _, "ἑ~ῳ~ηκωσῐ|ἑ~ῳ~ηκωσῐν";
    perfect_opt_act_1: "ἑ~ῳ~ηκοιμῐ|ἑ~ῳ~ηκοιην", _, _;
    perfect_opt_act_2: "ἑ~ῳ~ηκοις|ἑ~ῳ~ηκοιης", _, _;
    perfect_opt_act_3: "ἑ~ῳ~ηκοι|ἑ~ῳ~ηκοιη", _, _;
    perfect_imp_act_2: "ἑ~ῳ~ηκε", _, _;
    perfect_imp_mid_2: _, _, "ἑ~ῳ~ησθε";
    perfect_imp_mid_3: _, _, "ἑ~ῳ~ησθων";
    perfect_imp_pass_2: "ἑ~ῳ~ησο", _, "ἑ~ῳ~ησθε";
    perfect_imp_pass_3: "ἑ~ῳ~ησθω", "ἑ~ῳ~ησθων", "ἑ~ῳ~ησθων";
    pluperfect_mid_1: "ἑ~ῳ~ημην", _, _;
    pluperfect_mid_2: "ἑ~ῳ~ησο", "ἑ~ῳ~ησθον", _;
    pluperfect_mid_3: "ἑ~ῳ~ητο", _, "ἑ~ῳ~ηντο";
    pluperfect_pass_1: _, _, "ἑ~ῳ~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἑ~ῳ~ησθε";
    pluperfect_pass_3: _, "ἑ~ῳ~ησθην", _;
    infinitive_pres: "ἑ~οι~ειν", _, "ἑ~οι~εισθαι";
    infinitive_fut: "ἑ~οι~ησειν", _, _;
    infinitive_aor: "ἑ~οι~ησαι", _, _;
    infinitive_perf: _, "ἑ~ῳ~ησθαι", _;
    participle_pres: "ἑ~οι~ων", _, "ἑ~οι~ουμενος";
    participle_aor: "ἑ~οι~ησᾱς", "ἑ~οι~ησᾰμενος", _;
    participle_perf: "ἑ~ῳ~ηκως", _, "ἑ~ῳ~ημενος";
    ),
    // Εκπλεω: 1 words (εκπλεω). Like Εἰμῐ, except in 124 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("εκπλεω", 1, "εκ~ω", like Εἰμῐ;
    stem aorist_ind: "~υσ- like εκπλεω";
    stem aorist_ind_pass: "~υσθην- like εκπλεω";
    stem aorist_pass: "~υσθ- like εκπλεω";
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
    opt_act_2: "ἐκ~οις", "ἐκ~οιτον", _;
    opt_act_3: "ἐκ~οι", "ἐκ~οιτην", "ἐκ~οιεν";
    opt_mid_1: "ἐκ~οιμην", _, "ἐκ~οιμεθᾰ";
    opt_mid_2: "ἐκ~οιο", _, "ἐκ~οισθε";
    opt_mid_3: "ἐκ~οιτο", "ἐκ~οισθην", "ἐκ~οιντο";
    opt_pass_2: _, "ἐκ~οισθον", _;
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
    future_opt_mid_1: "ἐκ~υσοιμην", _, "ἐκ~υσοιμεθᾰ";
    future_opt_mid_3: _, "ἐκ~υσοισθην", _;
    aorist_subj_act_1: "ἐκ~υσω", _, _;
    aorist_subj_act_2: "ἐκ~υσῃς", "ἐκ~υσητον", _;
    aorist_subj_act_3: _, _, "ἐκ~υσωσῐ|ἐκ~υσωσῐν";
    aorist_opt_act_1: "ἐκ~υσαιμῐ", _, "ἐκ~υσαιμεν";
    aorist_opt_act_2: "ἐκ~υσειᾰς|ἐκ~υσαις", _, _;
    aorist_opt_act_3: "ἐκ~υσειε|ἐκ~υσειεν|ἐκ~υσαι", _, "ἐκ~υσειᾰν|ἐκ~υσαιεν";
    aorist_imp_act_2: "ἐκ~υσον", _, "ἐκ~υσᾰτε";
    aorist_imp_act_3: _, _, "ἐκ~υσᾰντων";
    perfect_ind_act_1: "ἐκ%ε~υκᾰ", _, "ἐκ%ε~υκᾰμεν";
    perfect_ind_act_2: _, "ἐκ%ε~υκᾰτον", _;
    perfect_ind_act_3: "ἐκ%ε~υκε|ἐκ%ε~υκεν", _, "ἐκ%ε~υκᾱσῐ|ἐκ%ε~υκᾱσῐν";
    perfect_ind_mid_1: _, _, "ἐκ%ε~υσμεθᾰ";
    perfect_ind_mid_2: "ἐκ%ε~υσαι", "ἐκ%ε~υσθον", _;
    perfect_ind_mid_3: "ἐκ%ε~υσται", _, "ἐκ%ε~υσᾰται";
    perfect_ind_pass_1: "ἐκ%ε~υσμαι", _, _;
    perfect_ind_pass_2: _, _, "ἐκ%ε~υσθε";
    perfect_subj_act_1: "ἐκ%ε~υκω", _, _;
    perfect_subj_act_2: "ἐκ%ε~υκῃς", "ἐκ%ε~υκητον", "ἐκ%ε~υκητε";
    perfect_subj_act_3: "ἐκ%ε~υκῃ", _, "ἐκ%ε~υκωσῐ|ἐκ%ε~υκωσῐν";
    perfect_opt_act_1: "ἐκ%ε~υκοιμῐ|ἐκ%ε~υκοιην", _, _;
    perfect_opt_act_2: "ἐκ%ε~υκοις|ἐκ%ε~υκοιης", _, _;
    perfect_opt_act_3: "ἐκ%ε~υκοι|ἐκ%ε~υκοιη", _, _;
    perfect_imp_act_2: "ἐκ%ε~υκε", _, _;
    perfect_imp_mid_2: _, _, "ἐκ%ε~υσθε";
    perfect_imp_mid_3: _, _, "ἐκ%ε~υσθων";
    perfect_imp_pass_2: "ἐκ%ε~υσο", _, "ἐκ%ε~υσθε";
    perfect_imp_pass_3: "ἐκ%ε~υσθω", "ἐκ%ε~υσθων", "ἐκ%ε~υσθων";
    infinitive_pres: "ἐκ~ιν", _, "ἐκ~ισθαι";
    infinitive_aor: "ἐκ~υσαι", _, _;
    infinitive_perf: _, "ἐκ%ε~υσθαι", _;
    participle_pres: "ἐκ~ων", _, "ἐκ~ομενος";
    participle_aor: "ἐκ~υσᾱς", _, _;
    participle_perf: "ἐκ%ε~υκως", _, "ἐκ%ε~υσμενος";
    ),
    // Δειδῐᾰ: 1 words (δειδῐᾰ). Like Μετεκῑᾰθον, except in 39 cells: ind_act_1 sg, aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, aorist_subj_mid_1 sg, ….
    conjugation!("δειδῐᾰ", 1, "~~δῐᾰ", like Μετεκῑᾰθον;
    stem aorist_ind: "~~σ- like ποιεω";
    ind_act_1: "~~δῐᾰ", _, _;
    aorist_subj_act_1: "~~σω", _, _;
    aorist_subj_act_2: "~~σῃς", "~~σητον", _;
    aorist_subj_act_3: _, _, "~~σωσῐ|~~σωσῐν";
    aorist_subj_mid_1: "~~σωμαι", _, _;
    aorist_subj_mid_2: "~~σῃ", "~~σησθον", _;
    aorist_opt_act_1: "~~σαιμῐ", _, "~~σαιμεν";
    aorist_opt_act_2: "~~σειᾰς|~~σαις", _, _;
    aorist_opt_act_3: "~~σειε|~~σειεν|~~σαι", _, "~~σειᾰν|~~σαιεν";
    aorist_opt_mid_1: "~~σαιμην", _, _;
    aorist_imp_act_2: "~~σον", _, "~~σᾰτε";
    aorist_imp_act_3: _, _, "~~σᾰντων";
    aorist_imp_mid_2: "~~σαι", "~~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~σᾰσθων";
    perfect_ind_act_1: "~δο~κᾰ", _, "~δο~κᾰμεν";
    perfect_ind_act_2: _, "~δο~κᾰτον", _;
    perfect_ind_act_3: "~δο~κε|~δο~κεν", _, "~δο~κᾱσῐ|~δο~κᾱσῐν";
    perfect_subj_act_1: "~δο~κω", _, _;
    perfect_subj_act_2: "~δο~κῃς", "~δο~κητον", "~δο~κητε";
    perfect_subj_act_3: "~δο~κῃ", _, "~δο~κωσῐ|~δο~κωσῐν";
    perfect_opt_act_1: "~δο~κοιμῐ|~δο~κοιην", _, _;
    perfect_opt_act_2: "~δο~κοις|~δο~κοιης", _, _;
    perfect_opt_act_3: "~δο~κοι|~δο~κοιη", _, _;
    perfect_imp_act_2: "~δο~κε", _, _;
    infinitive_aor: "~~σαι", _, _;
    participle_aor: "~~σᾱς", "~~σᾰμενος", _;
    participle_perf: "~δο~κως", _, _;
    ),
    // Λανθανομαι: 1 words (λανθανομαι). Like Μᾰστῐγοω, except in 58 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("λανθανομαι", 1, "~α~α~ομαι", like Μᾰστῐγοω;
    ind_act_1: "~ᾰ~ᾰ~ω", _, "~ᾰ~ᾰ~ομεν";
    ind_act_2: "~ᾰ~ᾰ~εις", "~ᾰ~ᾰ~ετον", _;
    ind_act_3: "~ᾰ~ᾰ~ει", _, "~ᾰ~ᾰ~ουσῐ|~ᾰ~ᾰ~ουσῐν";
    ind_mid_1: _, _, "~ᾰ~ᾰ~ομεθᾰ";
    ind_mid_2: "~ᾰ~ᾰ~ῃ|~ᾰ~ᾰ~ει", _, _;
    ind_mid_3: _, "~ᾰ~ᾰ~εσθον", _;
    ind_pass_1: "~ᾰ~ᾰ~ομαι", _, _;
    ind_pass_2: _, "~ᾰ~ᾰ~εσθον", "~ᾰ~ᾰ~εσθε";
    ind_pass_3: "~ᾰ~ᾰ~εται", "~ᾰ~ᾰ~εσθον", "~ᾰ~ᾰ~ονται";
    subj_act_1: "~ᾰ~ᾰ~ω", _, "~ᾰ~ᾰ~ωμεν";
    subj_act_2: "~ᾰ~ᾰ~ῃς", _, _;
    subj_act_3: "~ᾰ~ᾰ~ῃ", "~ᾰ~ᾰ~ητον", "~ᾰ~ᾰ~ωσῐ|~ᾰ~ᾰ~ωσῐν";
    subj_mid_1: _, _, "~ᾰ~ᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ᾰ~ησθον", _;
    subj_mid_3: "~ᾰ~ᾰ~ηται", _, "~ᾰ~ᾰ~ωνται";
    subj_pass_1: "~ᾰ~ᾰ~ωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~ᾰ~ησθε";
    opt_act_1: "~ᾰ~ᾰ~οιμῐ", _, "~ᾰ~ᾰ~οιμεν";
    opt_act_2: "~ᾰ~ᾰ~οις", "~ᾰ~ᾰ~οιτον", _;
    opt_act_3: "~ᾰ~ᾰ~οι", "~ᾰ~ᾰ~οιτην", "~ᾰ~ᾰ~οιεν";
    opt_mid_1: "~ᾰ~ᾰ~οιμην", _, "~ᾰ~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~ᾰ~οιο", _, "~ᾰ~ᾰ~οισθε";
    opt_mid_3: "~ᾰ~ᾰ~οιτο", "~ᾰ~ᾰ~οισθην", "~ᾰ~ᾰ~οιντο";
    opt_pass_2: _, "~ᾰ~ᾰ~οισθον", _;
    imp_act_2: "~ᾰ~ᾰ~ε", _, "~ᾰ~ᾰ~ετε";
    imp_act_3: _, _, "~ᾰ~ᾰ~οντων";
    imp_mid_2: "~ᾰ~ᾰ~ου", "~ᾰ~ᾰ~εσθον", "~ᾰ~ᾰ~εσθε";
    imp_mid_3: _, _, "~ᾰ~ᾰ~εσθων";
    imp_pass_2: _, "~ᾰ~ᾰ~εσθον", "~ᾰ~ᾰ~εσθε";
    imp_pass_3: "~ᾰ~ᾰ~εσθω", "~ᾰ~ᾰ~εσθων", "~ᾰ~ᾰ~εσθων";
    infinitive_pres: "~ᾰ~ᾰ~ειν", _, "~ᾰ~ᾰ~εσθαι";
    participle_pres: "~ᾰ~ᾰ~ων", _, "~ᾰ~ᾰ~ομενος";
    ),
    // Τρᾰπω: 1 words (τρᾰπω). Like Εἰμῐ, except in 23 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("τρᾰπω", 1, "~ᾰπω", like Εἰμῐ;
    ind_act_1: "~ᾰπω", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_1: "~αψω", _, "~αψομεν";
    future_ind_act_2: "~αψεις", "~αψετον", _;
    future_ind_act_3: _, _, "~αψουσῐ|~αψουσῐν";
    future_ind_mid_1: "~αψομαι", _, _;
    future_ind_mid_2: "~αψεαι", "~αψεσθον", _;
    future_ind_mid_3: "~αψεται", _, _;
    future_opt_act_1: "~αψοιμῐ", _, "~αψοιμεν";
    future_opt_act_2: "~αψοις", "~αψοιτον", _;
    future_opt_act_3: "~αψοι", "~αψοιτην", _;
    future_opt_mid_1: "~αψοιμην", _, "~αψοιμεθᾰ";
    future_opt_mid_3: _, "~αψοισθην", _;
    infinitive_fut: "~αψειν", _, _;
    ),
];
