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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident $(as $view:ident)?)?;
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
    /// Like `ἐχω`: 234 words.
    Ἐχω,
    /// Like `ποιεω`: 201 words.
    Ποιεω,
    /// Like `γῡροω`: 80 words.
    Γῡροω,
    /// Like `χρημᾰτῐζω`: 64 words.
    Χρημᾰτῐζω,
    /// Like `βᾰπτῐζω`: 62 words.
    Βᾰπτῐζω,
    /// Like `βουλευω`: 61 words.
    Βουλευω,
    /// Like `κᾰλεω`: 54 words.
    Κᾰλεω,
    /// Like `δοκᾰω`: 54 words.
    Δοκᾰω,
    /// Like `δῐκεω`: 49 words.
    Δῐκεω,
    /// Like `οἰομαι`: 43 words.
    Οἰομαι,
    /// Like `τᾰσσω`: 32 words.
    Τᾰσσω,
    /// Like `ληγω`: 30 words.
    Ληγω,
    /// Like `ψηφῐζω`: 28 words.
    Ψηφῐζω,
    /// Like `σῠνᾰρπᾰζω`: 24 words.
    Σῠνᾰρπᾰζω,
    /// Like `ἐλπω`: 22 words.
    Ἐλπω,
    /// Like `εὐλογεω`: 22 words.
    Εὐλογεω,
    /// Like `κτῐζω`: 21 words.
    Κτῐζω,
    /// Like `ἁπτω`: 21 words.
    Ἁπτω,
    /// Like `σκᾰνδᾰλῐζω`: 19 words.
    Σκᾰνδᾰλῐζω,
    /// Like `μῑγνῡμῐ`: 19 words.
    Μῑγνῡμῐ,
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
    /// Like `ξαινω`: 16 words.
    Ξαινω,
    /// Like `λογῐζομαι`: 15 words.
    Λογῐζομαι,
    /// Like `πειρᾰζω`: 15 words.
    Πειρᾰζω,
    /// Like `δερκομαι`: 15 words.
    Δερκομαι,
    /// Like `ἀνῐστημῐ`: 15 words.
    Ἀνῐστημῐ,
    /// Like `ἀραομαι`: 13 words.
    Ἀραομαι,
    /// Like `δῐοικεω`: 12 words.
    Δῐοικεω,
    /// Like `σβεννῡμῐ`: 12 words.
    Σβεννῡμῐ,
    /// Like `ἐκχεω`: 12 words.
    Ἐκχεω,
    /// Like `πειθω`: 11 words.
    Πειθω,
    /// Like `ἱημῐ`: 11 words.
    Ἱημῐ,
    /// Like `δεῑ`: 10 words.
    Δεῑ,
    /// Like `αἱρεω`: 10 words.
    Αἱρεω,
    /// Like `ἀειρω`: 10 words.
    Ἀειρω,
    /// Like `χεω`: 10 words.
    Χεω,
    /// Like `ἀφῑημῐ`: 10 words.
    Ἀφῑημῐ,
    /// Like `χρῑω`: 9 words.
    Χρῑω,
    /// Like `μᾰντευομαι`: 9 words.
    Μᾰντευομαι,
    /// Like `ἐνᾰντῐοομαι`: 9 words.
    Ἐνᾰντῐοομαι,
    /// Like `κᾰταφρονεω`: 9 words.
    Κᾰταφρονεω,
    /// Like `εἰσερχομαι`: 9 words.
    Εἰσερχομαι,
    /// Like `ἐπᾰϊω`: 9 words.
    Ἐπᾰϊω,
    /// Like `ψῑλοω`: 8 words.
    Ψῑλοω,
    /// Like `ἀνᾰγω`: 8 words.
    Ἀνᾰγω,
    /// Like `φᾰρμᾰσσω`: 8 words.
    Φᾰρμᾰσσω,
    /// Like `δῐωκω`: 8 words.
    Δῐωκω,
    /// Like `καθιζω`: 8 words.
    Καθιζω,
    /// Like `φῠω`: 8 words.
    Φῠω,
    /// Like `πλᾰσσω`: 8 words.
    Πλᾰσσω,
    /// Like `πᾰρεχω`: 8 words.
    Πᾰρεχω,
    /// Like `κρῑνομαι`: 8 words.
    Κρῑνομαι,
    /// Like `πῠνθανομαι`: 8 words.
    Πῠνθανομαι,
    /// Like `σῳζω`: 7 words.
    Σῳζω,
    /// Like `σφᾰλλω`: 7 words.
    Σφᾰλλω,
    /// Like `ἀπεῑπον`: 7 words.
    Ἀπεῑπον,
    /// Like `κᾰθιδρῡω`: 7 words.
    Κᾰθιδρῡω,
    /// Like `χραω`: 7 words.
    Χραω,
    /// Like `λλῡω`: 7 words.
    Λλῡω,
    /// Like `θλαω`: 7 words.
    Θλαω,
    /// Like `παυω`: 6 words.
    Παυω,
    /// Like `τελεω`: 6 words.
    Τελεω,
    /// Like `ἀγορευω`: 6 words.
    Ἀγορευω,
    /// Like `κατοικεω`: 6 words.
    Κατοικεω,
    /// Like `λᾰγοθηρεω`: 6 words.
    Λᾰγοθηρεω,
    /// Like `μηκᾰομαι`: 6 words.
    Μηκᾰομαι,
    /// Like `ἐπεξειμι`: 6 words.
    Ἐπεξειμι,
    /// Like `ἐαω`: 6 words.
    Ἐαω,
    /// Like `ἐρχομαι`: 6 words.
    Ἐρχομαι,
    /// Like `ἐπῐλᾰμβᾰνω`: 6 words.
    Ἐπῐλᾰμβᾰνω,
    /// Like `ἡδῠνω`: 6 words.
    Ἡδῠνω,
    /// Like `περιειμι`: 6 words.
    Περιειμι,
    /// Like `πρεσβευω`: 6 words.
    Πρεσβευω,
    /// Like `ζευγνῡμῐ`: 6 words.
    Ζευγνῡμῐ,
    /// Like `ἡκω`: 5 words.
    Ἡκω,
    /// Like `ἱζω`: 5 words.
    Ἱζω,
    /// Like `φρᾰζω`: 5 words.
    Φρᾰζω,
    /// Like `βαινω`: 5 words.
    Βαινω,
    /// Like `πρεπω`: 5 words.
    Πρεπω,
    /// Like `στερεω`: 5 words.
    Στερεω,
    /// Like `μᾰρτῠρεω`: 5 words.
    Μᾰρτῠρεω,
    /// Like `ἐλπῐζω`: 5 words.
    Ἐλπῐζω,
    /// Like `σῠνοικῐζω`: 5 words.
    Σῠνοικῐζω,
    /// Like `θρῠπτω`: 5 words.
    Θρῠπτω,
    /// Like `μωραινω`: 5 words.
    Μωραινω,
    /// Like `ἰᾱτρευω`: 5 words.
    Ἰᾱτρευω,
    /// Like `ῥηγνῡμῐ`: 5 words.
    Ῥηγνῡμῐ,
    /// Like `τρεφω`: 5 words.
    Τρεφω,
    /// Like `προβαλλω`: 5 words.
    Προβαλλω,
    /// Like `μετερχομαι`: 5 words.
    Μετερχομαι,
    /// Like `φειδομαι`: 5 words.
    Φειδομαι,
    /// Like `ἀποκωλῡω`: 5 words.
    Ἀποκωλῡω,
    /// Like `σκηπτω`: 5 words.
    Σκηπτω,
    /// Like `ἀναιρεω`: 5 words.
    Ἀναιρεω,
    /// Like `ῥαινω`: 5 words.
    Ῥαινω,
    /// Like `ἐλαυνω`: 5 words.
    Ἐλαυνω,
    /// Like `ἐξᾰκολουθεω`: 5 words.
    Ἐξᾰκολουθεω,
    /// Like `προκᾰλεω`: 5 words.
    Προκᾰλεω,
    /// Like `ἡγεομαι`: 4 words.
    Ἡγεομαι,
    /// Like `τῠπτω`: 4 words.
    Τῠπτω,
    /// Like `ἐπᾰγγελλω`: 4 words.
    Ἐπᾰγγελλω,
    /// Like `ναιω`: 4 words.
    Ναιω,
    /// Like `προσφερω`: 4 words.
    Προσφερω,
    /// Like `ἐπιτρεχω`: 4 words.
    Ἐπιτρεχω,
    /// Like `σπᾰω`: 4 words.
    Σπᾰω,
    /// Like `σῠνᾰγω`: 4 words.
    Σῠνᾰγω,
    /// Like `πειρω`: 4 words.
    Πειρω,
    /// Like `σᾰττω`: 4 words.
    Σᾰττω,
    /// Like `ἐργᾰζομαι`: 4 words.
    Ἐργᾰζομαι,
    /// Like `ἐρᾰμαι`: 4 words.
    Ἐρᾰμαι,
    /// Like `ἀνᾰπτω`: 4 words.
    Ἀνᾰπτω,
    /// Like `πειρᾰω`: 4 words.
    Πειρᾰω,
    /// Like `πᾰρᾰλλᾰσσω`: 4 words.
    Πᾰρᾰλλᾰσσω,
    /// Like `κᾰτειργω`: 4 words.
    Κᾰτειργω,
    /// Like `περαινω`: 4 words.
    Περαινω,
    /// Like `πᾰροξῡνω`: 4 words.
    Πᾰροξῡνω,
    /// Like `ἐλᾰσσοω`: 4 words.
    Ἐλᾰσσοω,
    /// Like `ἐρωτᾰω`: 4 words.
    Ἐρωτᾰω,
    /// Like `πᾰρᾰγῐγνομαι`: 4 words.
    Πᾰρᾰγῐγνομαι,
    /// Like `δημοκρᾰτεομαι`: 4 words.
    Δημοκρᾰτεομαι,
    /// Like `ἀνᾰλᾰμβᾰνω`: 4 words.
    Ἀνᾰλᾰμβᾰνω,
    /// Like `γυμνοω`: 4 words.
    Γυμνοω,
    /// Like `ἀφορῐζω`: 4 words.
    Ἀφορῐζω,
    /// Like `κᾰκοω`: 3 words.
    Κᾰκοω,
    /// Like `δεω`: 3 words.
    Δεω,
    /// Like `ἀποζω`: 3 words.
    Ἀποζω,
    /// Like `πλᾰναω`: 3 words.
    Πλᾰναω,
    /// Like `ἀμειβω`: 3 words.
    Ἀμειβω,
    /// Like `δῐᾰρπᾰζω`: 3 words.
    Δῐᾰρπᾰζω,
    /// Like `ὀμνῠω`: 3 words.
    Ὀμνῠω,
    /// Like `ψᾱω`: 3 words.
    Ψᾱω,
    /// Like `ἐπῐτῐθημῐ`: 3 words.
    Ἐπῐτῐθημῐ,
    /// Like `μνημονευω`: 3 words.
    Μνημονευω,
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
    /// Like `ἐφῐστημῐ`: 3 words.
    Ἐφῐστημῐ,
    /// Like `δῐαιρω`: 3 words.
    Δῐαιρω,
    /// Like `ἐγκειμαι`: 3 words.
    Ἐγκειμαι,
    /// Like `πᾰρᾰκᾰλεω`: 3 words.
    Πᾰρᾰκᾰλεω,
    /// Like `στελλω`: 3 words.
    Στελλω,
    /// Like `ἐξαιρω`: 3 words.
    Ἐξαιρω,
    /// Like `δῐᾰλῡω`: 3 words.
    Δῐᾰλῡω,
    /// Like `ἐξαναλῡω`: 3 words.
    Ἐξαναλῡω,
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
    /// Like `ζεω`: 3 words.
    Ζεω,
    /// Like `ὑπᾰκουω`: 3 words.
    Ὑπᾰκουω,
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
    /// Like `προσκειμαι`: 2 words.
    Προσκειμαι,
    /// Like `μετᾰκᾰλεω`: 2 words.
    Μετᾰκᾰλεω,
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
    /// Like `κεῑμαι`: 1 words.
    Κεῑμαι,
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
    /// Like `δειδω`: 1 words.
    Δειδω,
    /// Like `ἀπαυραω`: 1 words.
    Ἀπαυραω,
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
    /// Like `δῐκεῑν`: 1 words.
    Δῐκεῑν,
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
    /// Like `επῐβουλευω`: 1 words.
    Επῐβουλευω,
    /// Like `εισκᾰτοικεω`: 1 words.
    Εισκᾰτοικεω,
    /// Like `δειδῐᾰ`: 1 words.
    Δειδῐᾰ,
    /// Like `λανθανομαι`: 1 words.
    Λανθανομαι,
    /// Like `τρᾰπω`: 1 words.
    Τρᾰπω,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 293] = [
        Conjugation::Ἐχω,
        Conjugation::Ποιεω,
        Conjugation::Γῡροω,
        Conjugation::Χρημᾰτῐζω,
        Conjugation::Βᾰπτῐζω,
        Conjugation::Βουλευω,
        Conjugation::Κᾰλεω,
        Conjugation::Δοκᾰω,
        Conjugation::Δῐκεω,
        Conjugation::Οἰομαι,
        Conjugation::Τᾰσσω,
        Conjugation::Ληγω,
        Conjugation::Ψηφῐζω,
        Conjugation::Σῠνᾰρπᾰζω,
        Conjugation::Ἐλπω,
        Conjugation::Εὐλογεω,
        Conjugation::Κτῐζω,
        Conjugation::Ἁπτω,
        Conjugation::Σκᾰνδᾰλῐζω,
        Conjugation::Μῑγνῡμῐ,
        Conjugation::Νῑκαω,
        Conjugation::Πᾰραιτεομαι,
        Conjugation::Πορσῡνω,
        Conjugation::Πεμπω,
        Conjugation::Θωᾰω,
        Conjugation::Ξαινω,
        Conjugation::Λογῐζομαι,
        Conjugation::Πειρᾰζω,
        Conjugation::Δερκομαι,
        Conjugation::Ἀνῐστημῐ,
        Conjugation::Ἀραομαι,
        Conjugation::Δῐοικεω,
        Conjugation::Σβεννῡμῐ,
        Conjugation::Ἐκχεω,
        Conjugation::Πειθω,
        Conjugation::Ἱημῐ,
        Conjugation::Δεῑ,
        Conjugation::Αἱρεω,
        Conjugation::Ἀειρω,
        Conjugation::Χεω,
        Conjugation::Ἀφῑημῐ,
        Conjugation::Χρῑω,
        Conjugation::Μᾰντευομαι,
        Conjugation::Ἐνᾰντῐοομαι,
        Conjugation::Κᾰταφρονεω,
        Conjugation::Εἰσερχομαι,
        Conjugation::Ἐπᾰϊω,
        Conjugation::Ψῑλοω,
        Conjugation::Ἀνᾰγω,
        Conjugation::Φᾰρμᾰσσω,
        Conjugation::Δῐωκω,
        Conjugation::Καθιζω,
        Conjugation::Φῠω,
        Conjugation::Πλᾰσσω,
        Conjugation::Πᾰρεχω,
        Conjugation::Κρῑνομαι,
        Conjugation::Πῠνθανομαι,
        Conjugation::Σῳζω,
        Conjugation::Σφᾰλλω,
        Conjugation::Ἀπεῑπον,
        Conjugation::Κᾰθιδρῡω,
        Conjugation::Χραω,
        Conjugation::Λλῡω,
        Conjugation::Θλαω,
        Conjugation::Παυω,
        Conjugation::Τελεω,
        Conjugation::Ἀγορευω,
        Conjugation::Κατοικεω,
        Conjugation::Λᾰγοθηρεω,
        Conjugation::Μηκᾰομαι,
        Conjugation::Ἐπεξειμι,
        Conjugation::Ἐαω,
        Conjugation::Ἐρχομαι,
        Conjugation::Ἐπῐλᾰμβᾰνω,
        Conjugation::Ἡδῠνω,
        Conjugation::Περιειμι,
        Conjugation::Πρεσβευω,
        Conjugation::Ζευγνῡμῐ,
        Conjugation::Ἡκω,
        Conjugation::Ἱζω,
        Conjugation::Φρᾰζω,
        Conjugation::Βαινω,
        Conjugation::Πρεπω,
        Conjugation::Στερεω,
        Conjugation::Μᾰρτῠρεω,
        Conjugation::Ἐλπῐζω,
        Conjugation::Σῠνοικῐζω,
        Conjugation::Θρῠπτω,
        Conjugation::Μωραινω,
        Conjugation::Ἰᾱτρευω,
        Conjugation::Ῥηγνῡμῐ,
        Conjugation::Τρεφω,
        Conjugation::Προβαλλω,
        Conjugation::Μετερχομαι,
        Conjugation::Φειδομαι,
        Conjugation::Ἀποκωλῡω,
        Conjugation::Σκηπτω,
        Conjugation::Ἀναιρεω,
        Conjugation::Ῥαινω,
        Conjugation::Ἐλαυνω,
        Conjugation::Ἐξᾰκολουθεω,
        Conjugation::Προκᾰλεω,
        Conjugation::Ἡγεομαι,
        Conjugation::Τῠπτω,
        Conjugation::Ἐπᾰγγελλω,
        Conjugation::Ναιω,
        Conjugation::Προσφερω,
        Conjugation::Ἐπιτρεχω,
        Conjugation::Σπᾰω,
        Conjugation::Σῠνᾰγω,
        Conjugation::Πειρω,
        Conjugation::Σᾰττω,
        Conjugation::Ἐργᾰζομαι,
        Conjugation::Ἐρᾰμαι,
        Conjugation::Ἀνᾰπτω,
        Conjugation::Πειρᾰω,
        Conjugation::Πᾰρᾰλλᾰσσω,
        Conjugation::Κᾰτειργω,
        Conjugation::Περαινω,
        Conjugation::Πᾰροξῡνω,
        Conjugation::Ἐλᾰσσοω,
        Conjugation::Ἐρωτᾰω,
        Conjugation::Πᾰρᾰγῐγνομαι,
        Conjugation::Δημοκρᾰτεομαι,
        Conjugation::Ἀνᾰλᾰμβᾰνω,
        Conjugation::Γυμνοω,
        Conjugation::Ἀφορῐζω,
        Conjugation::Κᾰκοω,
        Conjugation::Δεω,
        Conjugation::Ἀποζω,
        Conjugation::Πλᾰναω,
        Conjugation::Ἀμειβω,
        Conjugation::Δῐᾰρπᾰζω,
        Conjugation::Ὀμνῠω,
        Conjugation::Ψᾱω,
        Conjugation::Ἐπῐτῐθημῐ,
        Conjugation::Μνημονευω,
        Conjugation::Ἐπαινεω,
        Conjugation::Λιστραω,
        Conjugation::Φλῡω,
        Conjugation::Ἐπευχομαι,
        Conjugation::Ἐφελκω,
        Conjugation::Τρῑβω,
        Conjugation::Ἀντεχω,
        Conjugation::Ποιμαινω,
        Conjugation::Σῠνιστημῐ,
        Conjugation::Ἐφῐστημῐ,
        Conjugation::Δῐαιρω,
        Conjugation::Ἐγκειμαι,
        Conjugation::Πᾰρᾰκᾰλεω,
        Conjugation::Στελλω,
        Conjugation::Ἐξαιρω,
        Conjugation::Δῐᾰλῡω,
        Conjugation::Ἐξαναλῡω,
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
        Conjugation::Ζεω,
        Conjugation::Ὑπᾰκουω,
        Conjugation::Λευκαινω,
        Conjugation::Σῠνδεω,
        Conjugation::Κηδευω,
        Conjugation::Λεγομαι,
        Conjugation::Ῥεω,
        Conjugation::Δῐᾰσῳζω,
        Conjugation::Κᾰτᾰβῐβᾰζω,
        Conjugation::Ὠνεομαι,
        Conjugation::Κᾰτᾰπονεω,
        Conjugation::Κηρῡσσω,
        Conjugation::Ἐπορον,
        Conjugation::Εἰδον,
        Conjugation::Καινῠμαι,
        Conjugation::Ξυμπολεμεω,
        Conjugation::Δῐακειμαι,
        Conjugation::Ξῠνοιδᾰ,
        Conjugation::Νισσομαι,
        Conjugation::Κᾰτασκηνοω,
        Conjugation::Γεγονοι,
        Conjugation::Ἐπῐχειρεω,
        Conjugation::Συνεπομαι,
        Conjugation::Λαλαγεω,
        Conjugation::Ξυνναυμᾰχεω,
        Conjugation::Προσκειμαι,
        Conjugation::Μετᾰκᾰλεω,
        Conjugation::Ερεω,
        Conjugation::Ἐδω,
        Conjugation::Αἰσθᾰνομαι,
        Conjugation::Τρυπαω,
        Conjugation::Μειδᾰω,
        Conjugation::Κεῑμαι,
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
        Conjugation::Δειδω,
        Conjugation::Ἀπαυραω,
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
        Conjugation::Δῐκεῑν,
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
        Conjugation::Επῐβουλευω,
        Conjugation::Εισκᾰτοικεω,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 293] = [
    // Ἐχω: 234 words (θελω, τῠγχᾰνω, δῐδᾰσκω, πῑνω, εὑρῐσκω, ᾀδω).
    conjugation!("ἐχω", 234, "~ω";
    stem future_pass: "~ηθησ- like ἐχω";
    stem aorist_ind: "~ησ- like ἐχω";
    stem aorist_ind_pass: "~ηθην- like ἐχω";
    stem aorist_pass: "~ηθ- like ἐχω";
    stem pluperfect_act: "~ηκειν- like ἐχω";
    ind_act_1: "~ω", _, "~ομεν";
    ind_act_2: "~εις", _, _;
    ind_act_3: "~ει", "~ετον", "~ουσῐ|~ουσῐν";
    ind_mid_1: "~ομαι", _, "~ομεθᾰ";
    ind_mid_2: "~ῃ", "~εσθον", _;
    ind_pass_2: _, _, "~εσθε";
    ind_pass_3: "~εται", _, "~ονται";
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
    opt_mid_2: "~οιο", "~οισθον", _;
    opt_mid_3: "~οιτο", "~οισθην", "~οιντο";
    opt_pass_2: _, _, "~οισθε";
    imp_act_2: "~ε", "~ετον", "~ετε";
    imp_mid_2: "~ου", _, "~εσθε";
    imp_mid_3: "~εσθω", _, "~εσθων";
    imp_pass_2: _, _, "~εσθε";
    imp_pass_3: _, "~εσθων", "~εσθων";
    imperfect_act_1: "ἐ~ον", _, "ἐ~ομεν";
    imperfect_act_2: "ἐ~ες", "ἐ~ετον", "ἐ~ετε";
    imperfect_act_3: "ἐ~ε", "ἐ~ετην", "ἐ~ον";
    imperfect_mid_1: "ἐ~ομην", _, "ἐ~ομεθᾰ";
    imperfect_mid_2: "ἐ~ου", _, _;
    imperfect_mid_3: "ἐ~ετο", _, _;
    imperfect_pass_2: _, "ἐ~εσθον", "ἐ~εσθε";
    imperfect_pass_3: _, "ἐ~εσθην", "ἐ~οντο";
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", "~ησετε";
    future_ind_act_3: _, _, "~ησουσῐ";
    future_ind_mid_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_mid_2: "~ησῃ", "~ησεσθον", _;
    future_ind_mid_3: "~ησεται", _, _;
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: _, "~ησοιτον", "~ησοιτε";
    future_opt_act_3: "~ησοι", "~ησοιτην", "~ησοιεν";
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_2: "~ησοιο", _, _;
    future_opt_mid_3: _, "~ησοισθην", _;
    aorist_subj_act_1: "~ησω", _, _;
    aorist_subj_act_2: "~ησῃς", "~ησητον", _;
    aorist_subj_mid_1: "~ησωμαι", _, _;
    aorist_subj_mid_2: "~ησῃ", "~ησησθον", _;
    aorist_opt_act_1: "~ησαιμῐ", _, "~ησαιμεν";
    aorist_opt_act_2: "~ησειᾰς", _, _;
    aorist_opt_act_3: "~ησειε", "~ησαιτην", "~ησειᾰν";
    aorist_opt_mid_1: "~ησαιμην", _, _;
    aorist_opt_mid_2: "~ησαιο", _, _;
    aorist_imp_act_2: "~ησον", _, "~ησᾰτε";
    aorist_imp_act_3: _, _, "~ησᾰντων";
    aorist_imp_mid_2: "~ησαι", "~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ησᾰσθων";
    perfect_ind_act_1: "%ε~ηκᾰ", _, "%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ηκᾱσῐ";
    perfect_ind_mid_1: "%ε~ημαι", _, "%ε~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ησαι", "%ε~ησθον", _;
    perfect_ind_mid_3: "%ε~ηται", _, "%ε~ηνται";
    perfect_ind_pass_2: _, _, "%ε~ησθε";
    perfect_subj_act_1: "%ε~ηκω", _, _;
    perfect_subj_act_2: "%ε~ηκῃς", "%ε~ηκητον", "%ε~ηκητε";
    perfect_subj_act_3: "%ε~ηκῃ", _, "%ε~ηκωσῐ";
    perfect_opt_act_1: "%ε~ηκοιμῐ", _, "%ε~ηκοιμεν";
    perfect_opt_act_2: "%ε~ηκοις", _, _;
    perfect_opt_act_3: _, "%ε~ηκοιτην", _;
    perfect_imp_act_2: "%ε~ηκε", _, "%ε~ηκετε";
    perfect_imp_act_3: _, _, "%ε~ηκοντων";
    perfect_imp_mid_2: _, _, "%ε~ησθε";
    perfect_imp_mid_3: "%ε~ησθω", _, "%ε~ησθων";
    perfect_imp_pass_2: "%ε~ησο", _, "%ε~ησθε";
    perfect_imp_pass_3: _, "%ε~ησθων", "%ε~ησθων";
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
    // Ποιεω: 201 words (φοβεω, βλασφημεω, γᾰμεω, μῑσεω, διακονεω, ποιεω). Like Ἐχω, except in 66 cells: ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ποιεω", 201, "~εω", like Ἐχω;
    stem future_pass: "~ηθησ- like ποιεω";
    stem aorist_ind: "~ησ- like ποιεω";
    stem aorist_pass: "~ηθ- like ποιεω";
    ind_act_1: _, _, "~οῡμεν";
    ind_act_2: "~εῑς", _, _;
    ind_act_3: "~εῑ", "~εῑτον", "~οῡσῐ|~οῡσῐν";
    ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    ind_mid_2: "~εῑ|~ῃ", "~εῑσθον", _;
    ind_pass_2: _, _, "~εῑσθε";
    ind_pass_3: "~εῑται", _, "~οῡνται";
    opt_act_1: "~οιην|~οῑμῐ", _, "~οῑμεν|~οιημεν";
    opt_act_2: "~οιης|~οῑς", "~οῑτον|~οιητον", "~οῑτε|~οιητε";
    opt_act_3: "~οιη|~οῑ", "~οιτην|~οιητην", "~οῑεν|~οιησᾰν";
    opt_mid_2: "~οῑο", "~οῑσθον", _;
    opt_mid_3: "~οῑτο", _, "~οῑντο";
    opt_pass_2: _, _, "~οῑσθε";
    imp_act_2: "~ει", "~εῑτον", "~εῑτε";
    imp_mid_2: "~οῡ", _, "~εῑσθε";
    imp_mid_3: "~εισθω", _, "~εισθων";
    imp_pass_2: _, _, "~εῑσθε";
    imp_pass_3: _, "~εισθων", "~εισθων";
    imperfect_act_1: "ἐ~ουν", _, "ἐ~οῡμεν";
    imperfect_act_2: "ἐ~εις", "ἐ~εῑτον", "ἐ~εῑτε";
    imperfect_act_3: "ἐ~ει", "ἐ~ειτην", "ἐ~ουν";
    imperfect_mid_1: "ἐ~ουμην", _, "ἐ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~οῡ", _, _;
    imperfect_mid_3: "ἐ~εῑτο", _, _;
    imperfect_pass_2: _, "ἐ~εῑσθον", "ἐ~εῑσθε";
    imperfect_pass_3: _, "ἐ~εισθην", "ἐ~οῡντο";
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
    infinitive_pres: "~εῑν", _, "~εῑσθαι";
    participle_pres: _, _, "~ουμενος";
    ),
    // Γῡροω: 80 words (ὁμοιοω, μορφοω, ζηλοω, ναρκοω, κοινοω, ἀλλοιοω). Like Ποιεω, except in 150 cells: ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("γῡροω", 80, "~οω", like Ποιεω;
    stem future_pass: "~ωθησ- like ἐχω";
    stem aorist_ind: "~ωσ- like ποιεω";
    stem aorist_ind_pass: "~ωθην- like ἐχω";
    stem aorist_pass: "~ωθ- like ἐχω";
    stem pluperfect_act: "~ωκειν- like ἐχω";
    ind_act_2: "~οῑς", _, _;
    ind_act_3: "~οῑ", "~οῡτον", _;
    ind_mid_2: "~οῑ", "~οῡσθον", _;
    ind_pass_2: _, _, "~οῡσθε";
    ind_pass_3: "~οῡται", _, _;
    subj_act_2: "~οῑς", _, _;
    subj_act_3: "~οῑ", "~ωτον", _;
    subj_mid_2: _, "~ωσθον", _;
    subj_mid_3: "~ωται", _, _;
    subj_pass_2: _, _, "~ωσθε";
    imp_act_2: "~ου", "~οῡτον", "~οῡτε";
    imp_mid_2: _, _, "~οῡσθε";
    imp_mid_3: "~ουσθω", _, "~ουσθων";
    imp_pass_2: _, _, "~οῡσθε";
    imp_pass_3: _, "~ουσθων", "~ουσθων";
    imperfect_act_2: "ἐ~ους", "ἐ~οῡτον", "ἐ~οῡτε";
    imperfect_act_3: "ἐ~ου", "ἐ~ουτην", _;
    imperfect_mid_3: "ἐ~οῡτο", _, _;
    imperfect_pass_2: _, "ἐ~οῡσθον", "ἐ~οῡσθε";
    imperfect_pass_3: _, "ἐ~ουσθην", _;
    future_ind_act_1: "~ωσω", _, "~ωσομεν";
    future_ind_act_2: "~ωσεις", "~ωσετον", "~ωσετε";
    future_ind_act_3: _, _, "~ωσουσῐ";
    future_ind_mid_1: "~ωσομαι", _, "~ωσομεθᾰ";
    future_ind_mid_2: "~ωσῃ|~ωσει", "~ωσεσθον", _;
    future_ind_mid_3: "~ωσεται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~ωσοιμῐ", _, "~ωσοιμεν";
    future_opt_act_2: _, "~ωσοιτον", "~ωσοιτε";
    future_opt_act_3: "~ωσοι", "~ωσοιτην", "~ωσοιεν";
    future_opt_mid_1: "~ωσοιμην", _, "~ωσοιμεθᾰ";
    future_opt_mid_2: "~ωσοιο", _, _;
    future_opt_mid_3: _, "~ωσοισθην", _;
    aorist_subj_act_1: "~ωσω", _, _;
    aorist_subj_act_2: "~ωσῃς", "~ωσητον", _;
    aorist_subj_act_3: _, _, "~ωσωσῐ|~ωσωσῐν";
    aorist_subj_mid_1: "~ωσωμαι", _, _;
    aorist_subj_mid_2: "~ωσῃ", "~ωσησθον", _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_1: "~ωσαιμῐ", _, "~ωσαιμεν";
    aorist_opt_act_2: "~ωσειᾰς|~ωσαις", _, _;
    aorist_opt_act_3: "~ωσειε|~ωσειεν|~ωσαι", "~ωσαιτην", "~ωσειᾰν|~ωσαιεν";
    aorist_opt_mid_1: "~ωσαιμην", _, _;
    aorist_opt_mid_2: "~ωσαιο", _, _;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: "~ωσον", _, "~ωσᾰτε";
    aorist_imp_act_3: _, _, "~ωσᾰντων";
    aorist_imp_mid_2: "~ωσαι", "~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ωσᾰσθων";
    perfect_ind_act_1: "%ε~ωκᾰ", _, "%ε~ωκᾰμεν";
    perfect_ind_act_2: _, "%ε~ωκᾰτον", _;
    perfect_ind_act_3: derived, _, "%ε~ωκᾱσῐ";
    perfect_ind_mid_1: "%ε~ωμαι", _, "%ε~ωμεθᾰ";
    perfect_ind_mid_2: "%ε~ωσαι", "%ε~ωσθον", _;
    perfect_ind_mid_3: "%ε~ωται", _, "%ε~ωνται";
    perfect_ind_pass_2: _, _, "%ε~ωσθε";
    perfect_subj_act_1: "%ε~ωκω", _, _;
    perfect_subj_act_2: "%ε~ωκῃς", "%ε~ωκητον", "%ε~ωκητε";
    perfect_subj_act_3: "%ε~ωκῃ", _, "%ε~ωκωσῐ";
    perfect_opt_act_1: "%ε~ωκοιμῐ", _, "%ε~ωκοιμεν";
    perfect_opt_act_2: "%ε~ωκοις", _, _;
    perfect_opt_act_3: derived, "%ε~ωκοιτην", _;
    perfect_imp_act_2: "%ε~ωκε", _, "%ε~ωκετε";
    perfect_imp_act_3: _, _, "%ε~ωκοντων";
    perfect_imp_mid_2: _, _, "%ε~ωσθε";
    perfect_imp_mid_3: "%ε~ωσθω", _, "%ε~ωσθων";
    perfect_imp_pass_2: "%ε~ωσο", _, "%ε~ωσθε";
    perfect_imp_pass_3: _, "%ε~ωσθων", "%ε~ωσθων";
    pluperfect_mid_1: "ἐ%ε~ωμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ωσο", "ἐ%ε~ωσθον", _;
    pluperfect_mid_3: "ἐ%ε~ωτο", _, "ἐ%ε~ωντο";
    pluperfect_pass_1: _, _, "ἐ%ε~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ωσθε";
    pluperfect_pass_3: _, "ἐ%ε~ωσθην", _;
    future_perfect_ind_mid_1: derived, _, _;
    future_perfect_ind_mid_3: _, derived, _;
    future_perfect_ind_pass_1: _, _, derived;
    future_perfect_ind_pass_2: derived, derived, derived;
    future_perfect_ind_pass_3: derived, derived, derived;
    future_perfect_opt_pass_1: derived, _, derived;
    future_perfect_opt_pass_2: derived, derived, derived;
    future_perfect_opt_pass_3: derived, derived, derived;
    infinitive_pres: "~οῡν", _, "~οῡσθαι";
    infinitive_fut: "~ωσειν", _, _;
    infinitive_aor: "~ωσαι", _, _;
    infinitive_perf: _, "%ε~ωσθαι", _;
    infinitive_futp: _, _, derived;
    participle_aor: "~ωσᾱς", "~ωσᾰμενος", _;
    participle_perf: "%ε~ωκως", _, "%ε~ωμενος";
    participle_futp: _, _, derived;
    ),
    // Χρημᾰτῐζω: 64 words (κᾰθᾰρῐζω, βᾰσᾰνῐζω, κᾰπνῐζω, φενᾱκῐζω, γεμῐζω, ἱζω). Like Γῡροω, except in 163 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("χρημᾰτῐζω", 64, "~ζω", like Γῡροω;
    stem future_pass: "~σθησ- like ποιεω";
    stem aorist_ind: "~σ- like ποιεω";
    stem aorist_ind_pass: "~σθην- like ἐχω";
    stem aorist_pass: "~σθ- like ποιεω";
    stem pluperfect_act: "~κ- like χρημᾰτῐζω";
    ind_act_1: "~ζω", _, "~ζομεν";
    ind_act_2: "~ζεις", _, _;
    ind_act_3: "~ζει", "~ζετον", "~ζουσῐ|~ζουσῐν";
    ind_mid_1: "~ζομαι", _, "~ζομεθᾰ";
    ind_mid_2: "~ζῃ|~ζει", "~ζεσθον", _;
    ind_pass_2: _, _, "~ζεσθε";
    ind_pass_3: "~ζεται", _, "~ζονται";
    subj_act_1: "~ζω", _, "~ζωμεν";
    subj_act_2: "~ζῃς", _, _;
    subj_act_3: "~ζῃ", "~ζητον", "~ζωσῐ|~ζωσῐν";
    subj_mid_1: _, _, "~ζωμεθᾰ";
    subj_mid_2: _, "~ζησθον", _;
    subj_mid_3: "~ζηται", _, "~ζωνται";
    subj_pass_1: "~ζωμαι", _, _;
    subj_pass_2: _, _, "~ζησθε";
    opt_act_1: "~ζοιμῐ", _, "~ζοιμεν";
    opt_act_2: "~ζοις", "~ζοιτον", derived;
    opt_act_3: "~ζοι", "~ζοιτην", "~ζοιεν";
    opt_mid_1: "~ζοιμην", _, "~ζοιμεθᾰ";
    opt_mid_2: "~ζοιο", "~ζοισθον", _;
    opt_mid_3: "~ζοιτο", "~ζοισθην", "~ζοιντο";
    opt_pass_2: _, _, "~ζοισθε";
    imp_act_2: "~ζε", "~ζετον", "~ζετε";
    imp_mid_2: "~ζου", _, "~ζεσθε";
    imp_mid_3: "~ζεσθω", _, "~ζεσθων";
    imp_pass_2: _, _, "~ζεσθε";
    imp_pass_3: _, "~ζεσθων", "~ζεσθων";
    imperfect_act_1: "ἐ~ζον", _, "ἐ~ζομεν";
    imperfect_act_2: "ἐ~ζες", "ἐ~ζετον", "ἐ~ζετε";
    imperfect_act_3: "ἐ~ζε|ἐ~ζεν", "ἐ~ζετην", "ἐ~ζον";
    imperfect_mid_1: "ἐ~ζομην", _, "ἐ~ζομεθᾰ";
    imperfect_mid_2: "ἐ~ζου", _, _;
    imperfect_mid_3: "ἐ~ζετο", _, _;
    imperfect_pass_2: _, "ἐ~ζεσθον", "ἐ~ζεσθε";
    imperfect_pass_3: _, "ἐ~ζεσθην", "ἐ~ζοντο";
    future_ind_act_1: "~ω", _, "~οῡμεν";
    future_ind_act_2: "~εῑς", "~εῑτον", "~εῑτε";
    future_ind_act_3: _, _, "~οῡσῐ|~οῡσῐν";
    future_ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    future_ind_mid_2: "~εῑ|~ῃ", "~εῑσθον", _;
    future_ind_mid_3: "~εῑται", _, _;
    future_opt_act_1: "~οιην|~οῑμῐ", _, "~οῑμεν|~οιημεν";
    future_opt_act_2: "~οιης|~οῑς", "~οῑτον|~οιητον", "~οῑτε|~οιητε";
    future_opt_act_3: "~οιη|~οῑ", "~οιτην|~οιητην", "~οῑεν|~οιησᾰν";
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_2: "~οῑο", _, _;
    future_opt_mid_3: _, "~οισθην", _;
    aorist_subj_act_1: "~σω", _, _;
    aorist_subj_act_2: "~σῃς", "~σητον", _;
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_subj_mid_1: "~σωμαι", _, _;
    aorist_subj_mid_2: "~σῃ", "~σησθον", _;
    aorist_opt_act_1: "~σαιμῐ", _, "~σαιμεν";
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", "~σαιτην", "~σειᾰν|~σαιεν";
    aorist_opt_mid_1: "~σαιμην", _, _;
    aorist_opt_mid_2: "~σαιο", _, _;
    aorist_imp_act_2: "~σον", _, "~σᾰτε";
    aorist_imp_act_3: _, _, "~σᾰντων";
    aorist_imp_mid_2: "~σαι", "~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~σᾰσθων";
    perfect_ind_act_1: "%ε~κᾰ", _, "%ε~κᾰμεν";
    perfect_ind_act_2: _, "%ε~κᾰτον", _;
    perfect_ind_act_3: "%ε~κε|%ε~κεν", _, "%ε~κᾱσῐ|%ε~κᾱσῐν";
    perfect_ind_mid_1: "%ε~σμαι", _, "%ε~σμεθᾰ";
    perfect_ind_mid_2: "%ε~σαι", "%ε~σθον", _;
    perfect_ind_mid_3: "%ε~σται", _, "%ε~δᾰται";
    perfect_ind_pass_2: _, _, "%ε~σθε";
    perfect_subj_act_1: "%ε~κω", _, _;
    perfect_subj_act_2: "%ε~κῃς", "%ε~κητον", "%ε~κητε";
    perfect_subj_act_3: "%ε~κῃ", _, "%ε~κωσῐ|%ε~κωσῐν";
    perfect_opt_act_1: "%ε~κοιμῐ|%ε~κοιην", _, "%ε~κοιμεν";
    perfect_opt_act_2: "%ε~κοις|%ε~κοιης", _, _;
    perfect_opt_act_3: "%ε~κοι|%ε~κοιη", "%ε~κοιτην", _;
    perfect_imp_act_2: "%ε~κε", _, "%ε~κετε";
    perfect_imp_act_3: _, _, "%ε~κοντων";
    perfect_imp_mid_2: _, _, "%ε~σθε";
    perfect_imp_mid_3: "%ε~σθω", _, "%ε~σθων";
    perfect_imp_pass_2: "%ε~σο", _, "%ε~σθε";
    perfect_imp_pass_3: _, "%ε~σθων", "%ε~σθων";
    pluperfect_mid_1: "ἐ%ε~σμην", _, _;
    pluperfect_mid_2: "ἐ%ε~σο", "ἐ%ε~σθον", _;
    pluperfect_mid_3: "ἐ%ε~στο", _, "ἐ%ε~δᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~σθε";
    pluperfect_pass_3: _, "ἐ%ε~σθην", _;
    infinitive_pres: "~ζειν", _, "~ζεσθαι";
    infinitive_fut: "~εῑν", _, _;
    infinitive_aor: "~σαι", _, _;
    infinitive_perf: _, "%ε~σθαι", _;
    participle_pres: "~ζων", _, "~ζομενος";
    participle_aor: "~σᾱς", "~σᾰμενος", _;
    participle_perf: "%ε~κως", _, "%ε~σμενος";
    ),
    // Βᾰπτῐζω: 62 words (πυρρᾰζω, βᾰπτῐζω, δᾰμαζω, σῐζω, εὐᾱζω, ἐξῠπνῐζω). Like Χρημᾰτῐζω, except in 66 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("βᾰπτῐζω", 62, "~ζω", like Χρημᾰτῐζω;
    stem aorist_ind: "~σ- like ἐχω";
    stem pluperfect_act: "~κειν- like ἐχω";
    ind_act_3: _, _, "~ζουσῐ";
    ind_mid_2: "~ζῃ", _, _;
    subj_act_3: _, _, "~ζωσῐ";
    imperfect_act_3: "ἐ~ζε", _, _;
    future_ind_act_1: "~σω", _, "~σομεν";
    future_ind_act_2: "~σεις", "~σετον", "~σετε";
    future_ind_act_3: _, _, "~σουσῐ";
    future_ind_mid_1: "~σομαι", _, "~σομεθᾰ";
    future_ind_mid_2: "~σῃ", "~σεσθον", _;
    future_ind_mid_3: "~σεται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    future_opt_act_2: derived, "~σοιτον", "~σοιτε";
    future_opt_act_3: "~σοι", "~σοιτην", "~σοιεν";
    future_opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    future_opt_mid_2: "~σοιο", _, _;
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
    perfect_opt_act_1: "%ε~κοιμῐ", _, _;
    perfect_opt_act_2: "%ε~κοις", _, _;
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
    // Βουλευω: 61 words (θερᾰπευω, πῐστευω, νηστευω, χορευω, παιδευω, λουω). Like Γῡροω, except in 160 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("βουλευω", 61, "~υω", like Γῡροω;
    stem future_pass: "~υθησ- like ποιεω";
    stem aorist_ind: "~υσ- like ποιεω";
    stem aorist_ind_pass: "~υθην- like ἐχω";
    stem aorist_pass: "~υθ- like ποιεω";
    stem pluperfect_act: "~υκειν- like ἐχω";
    ind_act_1: "~υω", _, "~υομεν";
    ind_act_2: "~υεις", _, _;
    ind_act_3: "~υει", "~υετον", "~υουσῐ|~υουσῐν";
    ind_mid_1: "~υομαι", _, "~υομεθᾰ";
    ind_mid_2: "~υῃ|~υει", "~υεσθον", _;
    ind_pass_2: _, _, "~υεσθε";
    ind_pass_3: "~υεται", _, "~υονται";
    subj_act_1: "~υω", _, "~υωμεν";
    subj_act_2: "~υῃς", _, _;
    subj_act_3: "~υῃ", "~υητον", "~υωσῐ|~υωσῐν";
    subj_mid_1: _, _, "~υωμεθᾰ";
    subj_mid_2: _, "~υησθον", _;
    subj_mid_3: "~υηται", _, "~υωνται";
    subj_pass_1: "~υωμαι", _, _;
    subj_pass_2: _, _, "~υησθε";
    opt_act_1: "~υοιμῐ", _, "~υοιμεν";
    opt_act_2: "~υοις", "~υοιτον", derived;
    opt_act_3: "~υοι", "~υοιτην", "~υοιεν";
    opt_mid_1: "~υοιμην", _, "~υοιμεθᾰ";
    opt_mid_2: "~υοιο", "~υοισθον", _;
    opt_mid_3: "~υοιτο", "~υοισθην", "~υοιντο";
    opt_pass_2: _, _, "~υοισθε";
    imp_act_2: "~υε", "~υετον", "~υετε";
    imp_mid_2: "~υου", _, "~υεσθε";
    imp_mid_3: "~υεσθω", _, "~υεσθων";
    imp_pass_2: _, _, "~υεσθε";
    imp_pass_3: _, "~υεσθων", "~υεσθων";
    imperfect_act_1: "ἐ~υον", _, "ἐ~υομεν";
    imperfect_act_2: "ἐ~υες", "ἐ~υετον", "ἐ~υετε";
    imperfect_act_3: "ἐ~υε|ἐ~υεν", "ἐ~υετην", "ἐ~υον";
    imperfect_mid_1: "ἐ~υομην", _, "ἐ~υομεθᾰ";
    imperfect_mid_2: "ἐ~υου", _, _;
    imperfect_mid_3: "ἐ~υετο", _, _;
    imperfect_pass_2: _, "ἐ~υεσθον", "ἐ~υεσθε";
    imperfect_pass_3: _, "ἐ~υεσθην", "ἐ~υοντο";
    future_ind_act_1: "~υσω", _, "~υσομεν";
    future_ind_act_2: "~υσεις", "~υσετον", "~υσετε";
    future_ind_act_3: _, _, "~υσουσῐ|~υσουσῐν";
    future_ind_mid_1: "~υσομαι", _, "~υσομεθᾰ";
    future_ind_mid_2: "~υσῃ|~υσει", "~υσεσθον", _;
    future_ind_mid_3: "~υσεται", _, _;
    future_opt_act_1: "~υσοιμῐ", _, "~υσοιμεν";
    future_opt_act_2: _, "~υσοιτον", "~υσοιτε";
    future_opt_act_3: "~υσοι", "~υσοιτην", "~υσοιεν";
    future_opt_mid_1: "~υσοιμην", _, "~υσοιμεθᾰ";
    future_opt_mid_2: "~υσοιο", _, _;
    future_opt_mid_3: _, "~υσοισθην", _;
    aorist_subj_act_1: "~υσω", _, _;
    aorist_subj_act_2: "~υσῃς", "~υσητον", _;
    aorist_subj_act_3: _, _, "~υσωσῐ|~υσωσῐν";
    aorist_subj_mid_1: "~υσωμαι", _, _;
    aorist_subj_mid_2: "~υσῃ", "~υσησθον", _;
    aorist_opt_act_1: "~υσαιμῐ", _, "~υσαιμεν";
    aorist_opt_act_2: "~υσειᾰς|~υσαις", _, _;
    aorist_opt_act_3: "~υσειε|~υσειεν|~υσαι", "~υσαιτην", "~υσειᾰν|~υσαιεν";
    aorist_opt_mid_1: "~υσαιμην", _, _;
    aorist_opt_mid_2: "~υσαιο", _, _;
    aorist_imp_act_2: "~υσον", _, "~υσᾰτε";
    aorist_imp_act_3: _, _, "~υσᾰντων";
    aorist_imp_mid_2: "~υσαι", "~υσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~υσᾰσθων";
    perfect_ind_act_1: "%ε~υκᾰ", _, "%ε~υκᾰμεν";
    perfect_ind_act_2: _, "%ε~υκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~υκᾱσῐ";
    perfect_ind_mid_1: "%ε~υμαι", _, "%ε~υμεθᾰ";
    perfect_ind_mid_2: "%ε~υσαι", "%ε~υσθον", _;
    perfect_ind_mid_3: "%ε~υται", _, "%ε~υνται";
    perfect_ind_pass_2: _, _, "%ε~υσθε";
    perfect_subj_act_1: "%ε~υκω", _, _;
    perfect_subj_act_2: "%ε~υκῃς", "%ε~υκητον", "%ε~υκητε";
    perfect_subj_act_3: "%ε~υκῃ", _, "%ε~υκωσῐ";
    perfect_opt_act_1: "%ε~υκοιμῐ", _, "%ε~υκοιμεν";
    perfect_opt_act_2: "%ε~υκοις", _, _;
    perfect_opt_act_3: _, "%ε~υκοιτην", _;
    perfect_imp_act_2: "%ε~υκε", _, "%ε~υκετε";
    perfect_imp_act_3: _, _, "%ε~υκοντων";
    perfect_imp_mid_2: _, _, "%ε~υσθε";
    perfect_imp_mid_3: "%ε~υσθω", _, "%ε~υσθων";
    perfect_imp_pass_2: "%ε~υσο", _, "%ε~υσθε";
    perfect_imp_pass_3: _, "%ε~υσθων", "%ε~υσθων";
    pluperfect_mid_1: "ἐ%ε~υμην", _, _;
    pluperfect_mid_2: "ἐ%ε~υσο", "ἐ%ε~υσθον", _;
    pluperfect_mid_3: "ἐ%ε~υτο", _, "ἐ%ε~υντο";
    pluperfect_pass_1: _, _, "ἐ%ε~υμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~υσθε";
    pluperfect_pass_3: _, "ἐ%ε~υσθην", _;
    infinitive_pres: "~υειν", _, "~υεσθαι";
    infinitive_fut: "~υσειν", _, _;
    infinitive_aor: "~ῡσαι", _, _;
    infinitive_perf: _, "%ε~ῡσθαι", _;
    participle_pres: "~υων", _, "~υομενος";
    participle_aor: "~υσᾱς", "~υσᾰμενος", _;
    participle_perf: "%ε~υκως", _, "%ε~υμενος";
    ),
    // Κᾰλεω: 54 words (ψῡχω, ἀρχω, ἀμᾰρῠσσω, αἰθω, στεφω, βρεχω). Like Ἐχω, except in 107 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 pl, ….
    conjugation!("κᾰλεω", 54, "~ω", like Ἐχω;
    stem future_pass: "~θησ- like ἐχω";
    stem aorist_ind: "~σ- like ἐχω";
    stem aorist_ind_pass: "~θην- like ἐχω";
    stem aorist_pass: "~θ- like ἐχω";
    stem pluperfect_act: "~κειν- like κᾰλεω";
    future_ind_act_1: "~σω", _, "~σομεν";
    future_ind_act_2: "~σεις", "~σετον", "~σετε";
    future_ind_act_3: _, _, "~σουσῐ";
    future_ind_mid_1: "~σομαι", _, "~σομεθᾰ";
    future_ind_mid_2: "~σῃ", "~σεσθον", _;
    future_ind_mid_3: "~σεται", _, _;
    future_opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    future_opt_act_2: _, "~σοιτον", "~σοιτε";
    future_opt_act_3: "~σοι", "~σοιτην", "~σοιεν";
    future_opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    future_opt_mid_2: "~σοιο", _, _;
    future_opt_mid_3: _, "~σοισθην", _;
    aorist_subj_act_1: "~σω", _, _;
    aorist_subj_act_2: "~σῃς", "~σητον", _;
    aorist_subj_mid_1: "~σωμαι", _, _;
    aorist_subj_mid_2: "~σῃ", "~σησθον", _;
    aorist_opt_act_1: "~σαιμῐ", _, "~σαιμεν";
    aorist_opt_act_2: "~σειᾰς", _, _;
    aorist_opt_act_3: "~σειε", "~σαιτην", "~σειᾰν";
    aorist_opt_mid_1: "~σαιμην", _, _;
    aorist_opt_mid_2: "~σαιο", _, _;
    aorist_imp_act_2: "~σον", _, "~σᾰτε";
    aorist_imp_act_3: _, _, "~σᾰντων";
    aorist_imp_mid_2: "~σαι", "~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~σᾰσθων";
    perfect_ind_act_1: "ἐ~κᾰ", _, "ἐ~κᾰμεν";
    perfect_ind_act_2: _, "ἐ~κᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~κᾱσῐ";
    perfect_ind_mid_1: "ἐ~μαι", _, "ἐ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~σαι", "ἐ~σθον", _;
    perfect_ind_mid_3: "ἐ~ται", _, "ἐ~νται";
    perfect_ind_pass_2: _, _, "ἐ~σθε";
    perfect_subj_act_1: "ἐ~κω", _, _;
    perfect_subj_act_2: "ἐ~κῃς", "ἐ~κητον", "ἐ~κητε";
    perfect_subj_act_3: "ἐ~κῃ", _, "ἐ~κωσῐ";
    perfect_opt_act_1: "ἐ~κοιμῐ", _, "ἐ~κοιμεν";
    perfect_opt_act_2: "ἐ~κοις", _, _;
    perfect_opt_act_3: _, "ἐ~κοιτην", _;
    perfect_imp_act_2: "ἐ~κε", _, "ἐ~κετε";
    perfect_imp_act_3: _, _, "ἐ~κοντων";
    perfect_imp_mid_2: _, _, "ἐ~σθε";
    perfect_imp_mid_3: "ἐ~σθω", _, "ἐ~σθων";
    perfect_imp_pass_2: "ἐ~σο", _, "ἐ~σθε";
    perfect_imp_pass_3: _, "ἐ~σθων", "ἐ~σθων";
    pluperfect_mid_1: "ἐ~μην", _, _;
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", _;
    pluperfect_mid_3: "ἐ~το", _, "ἐ~ντο";
    pluperfect_pass_1: _, _, "ἐ~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~σθε";
    pluperfect_pass_3: _, "ἐ~σθην", _;
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
    infinitive_perf: _, "ἐ~σθαι", _;
    infinitive_futp: _, _, derived;
    participle_aor: "~σᾱς", "~σᾰμενος", _;
    participle_perf: "ἐ~κως", _, "ἐ~μενος";
    participle_futp: _, _, derived;
    ),
    // Δοκᾰω: 54 words (ἀγᾰπᾰω, γεννᾰω, κῠβερνᾰω, γλᾰγᾰω, σῑγᾰω, κᾰκκᾰω). Like Ἐχω, except in 68 cells: ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("δοκᾰω", 54, "~ᾰω", like Ἐχω;
    stem aorist_ind: "~ησ- like ποιεω";
    ind_act_1: _, _, "~ωμεν";
    ind_act_2: "~ᾳς", _, _;
    ind_act_3: "~ᾳ", "~ᾱτον", "~ωσῐ|~ωσῐν";
    ind_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    ind_mid_2: "~ᾳ", "~ᾱσθον", _;
    ind_pass_2: _, _, "~ᾱσθε";
    ind_pass_3: "~ᾱται", _, "~ωνται";
    subj_act_2: "~ᾳς", _, _;
    subj_act_3: "~ᾳ", "~ᾱτον", _;
    subj_mid_2: _, "~ᾱσθον", _;
    subj_mid_3: "~ᾱται", _, _;
    subj_pass_2: _, _, "~ᾱσθε";
    opt_act_1: "~ῳην|~ῳμῐ", _, "~ῳμεν|~ῳημεν";
    opt_act_2: "~ῳης|~ῳς", "~ῳτον|~ῳητον", "~ῳτε|~ῳητε";
    opt_act_3: "~ῳη|~ῳ", "~ῳτην|~ῳητην", "~ῳεν|~ῳησᾰν";
    opt_mid_1: "~ῳμην", _, "~ῳμεθᾰ";
    opt_mid_2: "~ῳο", "~ῳσθον", _;
    opt_mid_3: "~ῳτο", "~ῳσθην", "~ῳντο";
    opt_pass_2: _, _, "~ῳσθε";
    imp_act_2: "~ᾱ", "~ᾱτον", "~ᾱτε";
    imp_mid_2: "~ω", _, "~ᾱσθε";
    imp_mid_3: "~ᾱσθω", _, "~ᾱσθων";
    imp_pass_2: _, _, "~ᾱσθε";
    imp_pass_3: _, "~ᾱσθων", "~ᾱσθων";
    imperfect_act_1: "ἐ~ων", _, "ἐ~ωμεν";
    imperfect_act_2: "ἐ~ᾱς", "ἐ~ᾱτον", "ἐ~ᾱτε";
    imperfect_act_3: "ἐ~ᾱ", "ἐ~ᾱτην", "ἐ~ων";
    imperfect_mid_1: "ἐ~ωμην", _, "ἐ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ω", _, _;
    imperfect_mid_3: "ἐ~ᾱτο", _, _;
    imperfect_pass_2: _, "ἐ~ᾱσθον", "ἐ~ᾱσθε";
    imperfect_pass_3: _, "ἐ~ᾱσθην", "ἐ~ωντο";
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    aorist_subj_act_3: _, _, "~ησωσῐ|~ησωσῐν";
    aorist_opt_act_2: "~ησειᾰς|~ησαις", _, _;
    aorist_opt_act_3: "~ησειε|~ησειεν|~ησαι", _, "~ησειᾰν|~ησαιεν";
    infinitive_pres: "~ᾱν", _, "~ᾱσθαι";
    participle_pres: _, _, "~ωμενος";
    ),
    // Δῐκεω: 49 words (ἀγᾰθοποιεω, ἀγνοεω, ἀντλεω, ἀσκεω, οἰκεω, ἀλεω). Like Ποιεω, except in 63 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_ind_mid_1 sg, ….
    conjugation!("δῐκεω", 49, "~εω", like Ποιεω;
    stem pluperfect_act: "~ηκ- like δῐκεω";
    perfect_ind_act_1: "ἐ~ηκᾰ", _, "ἐ~ηκᾰμεν";
    perfect_ind_act_2: _, "ἐ~ηκᾰτον", _;
    perfect_ind_act_3: "ἐ~ηκε|ἐ~ηκεν", _, "ἐ~ηκᾱσῐ|ἐ~ηκᾱσῐν";
    perfect_ind_mid_1: "ἐ~ημαι", _, "ἐ~ημεθᾰ";
    perfect_ind_mid_2: "ἐ~ησαι", "ἐ~ησθον", _;
    perfect_ind_mid_3: "ἐ~ηται", _, "ἐ~ηνται";
    perfect_ind_pass_2: _, _, "ἐ~ησθε";
    perfect_subj_act_1: "ἐ~ηκω", _, _;
    perfect_subj_act_2: "ἐ~ηκῃς", "ἐ~ηκητον", "ἐ~ηκητε";
    perfect_subj_act_3: "ἐ~ηκῃ", _, "ἐ~ηκωσῐ|ἐ~ηκωσῐν";
    perfect_opt_act_1: "ἐ~ηκοιμῐ|ἐ~ηκοιην", _, "ἐ~ηκοιμεν";
    perfect_opt_act_2: "ἐ~ηκοις|ἐ~ηκοιης", _, _;
    perfect_opt_act_3: "ἐ~ηκοι|ἐ~ηκοιη", "ἐ~ηκοιτην", _;
    perfect_imp_act_2: "ἐ~ηκε", _, "ἐ~ηκετε";
    perfect_imp_act_3: _, _, "ἐ~ηκοντων";
    perfect_imp_mid_2: _, _, "ἐ~ησθε";
    perfect_imp_mid_3: "ἐ~ησθω", _, "ἐ~ησθων";
    perfect_imp_pass_2: "ἐ~ησο", _, "ἐ~ησθε";
    perfect_imp_pass_3: _, "ἐ~ησθων", "ἐ~ησθων";
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
    // Οἰομαι: 43 words (γῐγνομαι, δεχομαι, μαινομαι, ἑπομαι, περδομαι, εὐχομαι). Like Ἐχω, except in 128 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("οἰομαι", 43, "~ομαι", like Ἐχω;
    ind_act_1: "~ομαι", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, derived, derived;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_2: "~ῃ", _, _;
    subj_pass_2: "~ῃ", _, _;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, _;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: _, _, derived;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: _, derived, derived;
    future_opt_act_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_ind_mid_1: derived, _, _;
    aorist_ind_mid_2: derived, derived, _;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_mid_1: derived, _, _;
    aorist_subj_mid_2: derived, derived, _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_mid_1: derived, _, _;
    aorist_opt_mid_2: derived, _, _;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_mid_2: derived, derived, _;
    aorist_imp_mid_3: _, _, derived;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: _, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
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
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    infinitive_futp: _, _, derived;
    participle_pres: derived, _, _;
    participle_aor: derived, derived, _;
    participle_perf: derived, _, _;
    participle_futp: _, _, derived;
    ),
    // Τᾰσσω: 32 words (ἀϊσσω, πλησσω, τᾰσσω, μᾰσσω, φῠλᾰσσω, πτῠσσω). Like Κᾰλεω, except in 159 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("τᾰσσω", 32, "~σσω", like Κᾰλεω;
    stem future_pass: "~χθησ- like ἐχω";
    stem aorist_ind: "~ξ- like ποιεω";
    stem aorist_ind_pass: "~χθην- like ἐχω";
    stem aorist_pass: "~χθ- like ἐχω";
    stem pluperfect_act: "~χειν- like ἐχω";
    ind_act_1: "~σσω", _, "~σσομεν";
    ind_act_2: "~σσεις", _, _;
    ind_act_3: "~σσει", "~σσετον", "~σσουσῐ|~σσουσῐν";
    ind_mid_1: "~σσομαι", _, "~σσομεθᾰ";
    ind_mid_2: "~σσῃ|~σσει", "~σσεσθον", _;
    ind_pass_2: _, _, "~σσεσθε";
    ind_pass_3: "~σσεται", _, "~σσονται";
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
    opt_mid_2: "~σσοιο", "~σσοισθον", _;
    opt_mid_3: "~σσοιτο", "~σσοισθην", "~σσοιντο";
    opt_pass_2: _, _, "~σσοισθε";
    imp_act_2: "~σσε", "~σσετον", "~σσετε";
    imp_mid_2: "~σσου", _, "~σσεσθε";
    imp_mid_3: "~σσεσθω", _, "~σσεσθων";
    imp_pass_2: _, _, "~σσεσθε";
    imp_pass_3: _, "~σσεσθων", "~σσεσθων";
    imperfect_act_1: "ἐ~σσον", _, "ἐ~σσομεν";
    imperfect_act_2: "ἐ~σσες", "ἐ~σσετον", "ἐ~σσετε";
    imperfect_act_3: "ἐ~σσε|ἐ~σσεν", "ἐ~σσετην", "ἐ~σσον";
    imperfect_mid_1: "ἐ~σσομην", _, "ἐ~σσομεθᾰ";
    imperfect_mid_2: "ἐ~σσου", _, _;
    imperfect_mid_3: "ἐ~σσετο", _, _;
    imperfect_pass_2: _, "ἐ~σσεσθον", "ἐ~σσεσθε";
    imperfect_pass_3: _, "ἐ~σσεσθην", "ἐ~σσοντο";
    future_ind_act_1: "~ξω", _, "~ξομεν";
    future_ind_act_2: "~ξεις", "~ξετον", "~ξετε";
    future_ind_act_3: _, _, "~ξουσῐ|~ξουσῐν";
    future_ind_mid_1: "~ξομαι", _, "~ξομεθᾰ";
    future_ind_mid_2: "~ξῃ|~ξει", "~ξεσθον", _;
    future_ind_mid_3: "~ξεται", _, _;
    future_opt_act_1: "~ξοιμῐ", _, "~ξοιμεν";
    future_opt_act_2: _, "~ξοιτον", "~ξοιτε";
    future_opt_act_3: "~ξοι", "~ξοιτην", "~ξοιεν";
    future_opt_mid_1: "~ξοιμην", _, "~ξοιμεθᾰ";
    future_opt_mid_2: "~ξοιο", _, _;
    future_opt_mid_3: _, "~ξοισθην", _;
    aorist_subj_act_1: "~ξω", _, _;
    aorist_subj_act_2: "~ξῃς", "~ξητον", _;
    aorist_subj_act_3: _, _, "~ξωσῐ|~ξωσῐν";
    aorist_subj_mid_1: "~ξωμαι", _, _;
    aorist_subj_mid_2: "~ξῃ", "~ξησθον", _;
    aorist_opt_act_1: "~ξαιμῐ", _, "~ξαιμεν";
    aorist_opt_act_2: "~ξειᾰς|~ξαις", _, _;
    aorist_opt_act_3: "~ξειε|~ξειεν|~ξαι", "~ξαιτην", "~ξειᾰν|~ξαιεν";
    aorist_opt_mid_1: "~ξαιμην", _, _;
    aorist_opt_mid_2: "~ξαιο", _, _;
    aorist_imp_act_2: "~ξον", _, "~ξᾰτε";
    aorist_imp_act_3: _, _, "~ξᾰντων";
    aorist_imp_mid_2: "~ξαι", "~ξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ξᾰσθων";
    perfect_ind_act_1: "%ε~χᾰ", _, "%ε~χᾰμεν";
    perfect_ind_act_2: _, "%ε~χᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~χᾱσῐ";
    perfect_ind_mid_1: "%ε~γμαι", _, "%ε~γμεθᾰ";
    perfect_ind_mid_2: "%ε~ξαι", "%ε~χθον", _;
    perfect_ind_mid_3: "%ε~κται", _, "%ε~γᾰται";
    perfect_ind_pass_2: _, _, "%ε~χθε";
    perfect_subj_act_1: "%ε~χω", _, _;
    perfect_subj_act_2: "%ε~χῃς", "%ε~χητον", "%ε~χητε";
    perfect_subj_act_3: "%ε~χῃ", _, "%ε~χωσῐ";
    perfect_opt_act_1: "%ε~χοιμῐ", _, "%ε~χοιμεν";
    perfect_opt_act_2: "%ε~χοις", _, _;
    perfect_opt_act_3: _, "%ε~χοιτην", _;
    perfect_imp_act_2: "%ε~χε", _, "%ε~χετε";
    perfect_imp_act_3: _, _, "%ε~χοντων";
    perfect_imp_mid_2: _, _, "%ε~χθε";
    perfect_imp_mid_3: "%ε~χθω", _, "%ε~χθων";
    perfect_imp_pass_2: "%ε~ξο", _, "%ε~χθε";
    perfect_imp_pass_3: _, "%ε~χθων", "%ε~χθων";
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
    // Ληγω: 30 words (ἀγω, λεγω, φευγω, τρωγω, δῐᾰλεγω, φλεγω). Like Τᾰσσω, except in 103 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ληγω", 30, "~γω", like Τᾰσσω;
    stem aorist_ind: "~ξ- like ἐχω";
    ind_act_1: "~γω", _, "~γομεν";
    ind_act_2: "~γεις", _, _;
    ind_act_3: "~γει", "~γετον", "~γουσῐ";
    ind_mid_1: "~γομαι", _, "~γομεθᾰ";
    ind_mid_2: "~γῃ", "~γεσθον", _;
    ind_pass_2: _, _, "~γεσθε";
    ind_pass_3: "~γεται", _, "~γονται";
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
    opt_mid_2: "~γοιο", "~γοισθον", _;
    opt_mid_3: "~γοιτο", "~γοισθην", "~γοιντο";
    opt_pass_2: _, _, "~γοισθε";
    imp_act_2: "~γε", "~γετον", "~γετε";
    imp_mid_2: "~γου", _, "~γεσθε";
    imp_mid_3: "~γεσθω", _, "~γεσθων";
    imp_pass_2: _, _, "~γεσθε";
    imp_pass_3: _, "~γεσθων", "~γεσθων";
    imperfect_act_1: "ἐ~γον", _, "ἐ~γομεν";
    imperfect_act_2: "ἐ~γες", "ἐ~γετον", "ἐ~γετε";
    imperfect_act_3: "ἐ~γε", "ἐ~γετην", "ἐ~γον";
    imperfect_mid_1: "ἐ~γομην", _, "ἐ~γομεθᾰ";
    imperfect_mid_2: "ἐ~γου", _, _;
    imperfect_mid_3: "ἐ~γετο", _, _;
    imperfect_pass_2: _, "ἐ~γεσθον", "ἐ~γεσθε";
    imperfect_pass_3: _, "ἐ~γεσθην", "ἐ~γοντο";
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
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: _, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
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
    // Ψηφῐζω: 28 words (γνωρῐζω, ψηφῐζω, ὁρῐζω, ἁγνῐζω, σχηματῐζω, ἀτενῐζω). Like Χρημᾰτῐζω, except in 44 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_ind_mid_1 sg, ….
    conjugation!("ψηφῐζω", 28, "~ζω", like Χρημᾰτῐζω;
    stem pluperfect_act: "~κ- like δῐκεω";
    perfect_ind_act_1: "ἐ~κᾰ", _, "ἐ~κᾰμεν";
    perfect_ind_act_2: _, "ἐ~κᾰτον", _;
    perfect_ind_act_3: "ἐ~κε|ἐ~κεν", _, "ἐ~κᾱσῐ|ἐ~κᾱσῐν";
    perfect_ind_mid_1: "ἐ~σμαι", _, "ἐ~σμεθᾰ";
    perfect_ind_mid_2: "ἐ~σαι", "ἐ~σθον", _;
    perfect_ind_mid_3: "ἐ~σται", _, "ἐ~δᾰται";
    perfect_ind_pass_2: _, _, "ἐ~σθε";
    perfect_subj_act_1: "ἐ~κω", _, _;
    perfect_subj_act_2: "ἐ~κῃς", "ἐ~κητον", "ἐ~κητε";
    perfect_subj_act_3: "ἐ~κῃ", _, "ἐ~κωσῐ|ἐ~κωσῐν";
    perfect_opt_act_1: "ἐ~κοιμῐ|ἐ~κοιην", _, "ἐ~κοιμεν";
    perfect_opt_act_2: "ἐ~κοις|ἐ~κοιης", _, _;
    perfect_opt_act_3: "ἐ~κοι|ἐ~κοιη", "ἐ~κοιτην", _;
    perfect_imp_act_2: "ἐ~κε", _, "ἐ~κετε";
    perfect_imp_act_3: _, _, "ἐ~κοντων";
    perfect_imp_mid_2: _, _, "ἐ~σθε";
    perfect_imp_mid_3: "ἐ~σθω", _, "ἐ~σθων";
    perfect_imp_pass_2: "ἐ~σο", _, "ἐ~σθε";
    perfect_imp_pass_3: _, "ἐ~σθων", "ἐ~σθων";
    pluperfect_mid_1: "ἐ~σμην", _, _;
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", _;
    pluperfect_mid_3: "ἐ~στο", _, "ἐ~δᾰτο";
    pluperfect_pass_1: _, _, "ἐ~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~σθε";
    pluperfect_pass_3: _, "ἐ~σθην", _;
    infinitive_perf: _, "ἐ~σθαι", _;
    participle_perf: "ἐ~κως", _, "ἐ~σμενος";
    ),
    // Σῠνᾰρπᾰζω: 24 words (βᾰζω, ἁρπᾰζω, ἐλελῐζω, κρᾱζω, στῐζω, σφυζω). Like Ληγω, except in 104 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("σῠνᾰρπᾰζω", 24, "~ζω", like Ληγω;
    ind_act_1: "~ζω", _, "~ζομεν";
    ind_act_2: "~ζεις", _, _;
    ind_act_3: "~ζει", "~ζετον", "~ζουσῐ";
    ind_mid_1: "~ζομαι", _, "~ζομεθᾰ";
    ind_mid_2: "~ζῃ", "~ζεσθον", _;
    ind_pass_2: _, _, "~ζεσθε";
    ind_pass_3: "~ζεται", _, "~ζονται";
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
    opt_mid_2: "~ζοιο", "~ζοισθον", _;
    opt_mid_3: "~ζοιτο", "~ζοισθην", "~ζοιντο";
    opt_pass_2: _, _, "~ζοισθε";
    imp_act_2: "~ζε", "~ζετον", "~ζετε";
    imp_mid_2: "~ζου", _, "~ζεσθε";
    imp_mid_3: "~ζεσθω", _, "~ζεσθων";
    imp_pass_2: _, _, "~ζεσθε";
    imp_pass_3: _, "~ζεσθων", "~ζεσθων";
    imperfect_act_1: "ἐ~ζον", _, "ἐ~ζομεν";
    imperfect_act_2: "ἐ~ζες", "ἐ~ζετον", "ἐ~ζετε";
    imperfect_act_3: "ἐ~ζε", "ἐ~ζετην", "ἐ~ζον";
    imperfect_mid_1: "ἐ~ζομην", _, "ἐ~ζομεθᾰ";
    imperfect_mid_2: "ἐ~ζου", _, _;
    imperfect_mid_3: "ἐ~ζετο", _, _;
    imperfect_pass_2: _, "ἐ~ζεσθον", "ἐ~ζεσθε";
    imperfect_pass_3: _, "ἐ~ζεσθην", "ἐ~ζοντο";
    perfect_ind_act_1: "ἐ~χᾰ", _, "ἐ~χᾰμεν";
    perfect_ind_act_2: _, "ἐ~χᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~χᾱσῐ";
    perfect_ind_mid_1: "ἐ~γμαι", _, "ἐ~γμεθᾰ";
    perfect_ind_mid_2: "ἐ~ξαι", "ἐ~χθον", _;
    perfect_ind_mid_3: "ἐ~κται", _, "ἐ~γᾰται";
    perfect_ind_pass_2: _, _, "ἐ~χθε";
    perfect_subj_act_1: "ἐ~χω", _, _;
    perfect_subj_act_2: "ἐ~χῃς", "ἐ~χητον", "ἐ~χητε";
    perfect_subj_act_3: "ἐ~χῃ", _, "ἐ~χωσῐ";
    perfect_opt_act_1: "ἐ~χοιμῐ", _, "ἐ~χοιμεν";
    perfect_opt_act_2: "ἐ~χοις", _, _;
    perfect_opt_act_3: _, "ἐ~χοιτην", _;
    perfect_imp_act_2: "ἐ~χε", _, "ἐ~χετε";
    perfect_imp_act_3: _, _, "ἐ~χοντων";
    perfect_imp_mid_2: _, _, "ἐ~χθε";
    perfect_imp_mid_3: "ἐ~χθω", _, "ἐ~χθων";
    perfect_imp_pass_2: "ἐ~ξο", _, "ἐ~χθε";
    perfect_imp_pass_3: _, "ἐ~χθων", "ἐ~χθων";
    infinitive_pres: "~ζειν", _, "~ζεσθαι";
    infinitive_perf: _, "ἐ~χθαι", _;
    participle_pres: "~ζων", _, "~ζομενος";
    participle_perf: "ἐ~χως", _, "ἐ~γμενος";
    ),
    // Ἐλπω: 22 words (ὑω, εὑω, ἐλπω, ἰδῑω, ἐρδω, περικλειω). Like Κᾰλεω, except in 59 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("ἐλπω", 22, "~ω", like Κᾰλεω;
    stem future_pass: "~σθησ- like ἐχω";
    stem aorist_ind: "~σ- like ἐλπω";
    stem aorist_ind_pass: "~σθην- like ἐλπω";
    stem aorist_pass: "~σθ- like ἐχω";
    imperfect_act_1: "~ον", _, "~ομεν";
    imperfect_act_2: "~ες", "~ετον", "~ετε";
    imperfect_act_3: "~ε|~εν", "~ετην", "~ον";
    imperfect_mid_1: "~ομην", _, "~ομεθᾰ";
    imperfect_mid_2: "~ου", _, _;
    imperfect_mid_3: "~ετο", _, _;
    imperfect_pass_2: _, "~εσθον", "~εσθε";
    imperfect_pass_3: _, "~εσθην", "~οντο";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_1: "~σμαι", _, "~σμεθᾰ";
    perfect_ind_mid_2: "~σαι", "~σθον", _;
    perfect_ind_mid_3: "~σται", _, "~σᾰται";
    perfect_ind_pass_2: _, _, "~σθε";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: _, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, _, "~σθε";
    perfect_imp_mid_3: "~σθω", _, "~σθων";
    perfect_imp_pass_2: "~σο", _, "~σθε";
    perfect_imp_pass_3: _, "~σθων", "~σθων";
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
    // Εὐλογεω: 22 words (εὐλογεω, ἠθεω, κᾰτηχεω, ἠχεω, ὀλιγαρχεω, μοναρχεω). Like Δῐκεω, except in 60 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("εὐλογεω", 22, "~εω", like Δῐκεω;
    stem aorist_ind: "~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ηθην- like ἐλπω";
    stem pluperfect_act: "~ηκ- like εὐλογεω";
    imperfect_act_1: "~ουν", _, "~οῡμεν";
    imperfect_act_2: "~εις", "~εῑτον", "~εῑτε";
    imperfect_act_3: "~ει", "~ειτην", "~ουν";
    imperfect_mid_1: "~ουμην", _, "~ουμεθᾰ";
    imperfect_mid_2: "~οῡ", _, _;
    imperfect_mid_3: "~εῑτο", _, _;
    imperfect_pass_2: _, "~εῑσθον", "~εῑσθε";
    imperfect_pass_3: _, "~εισθην", "~οῡντο";
    perfect_ind_act_1: "~ηκᾰ", _, "~ηκᾰμεν";
    perfect_ind_act_2: _, "~ηκᾰτον", _;
    perfect_ind_act_3: "~ηκε|~ηκεν", _, "~ηκᾱσῐ|~ηκᾱσῐν";
    perfect_ind_mid_1: "~ημαι", _, "~ημεθᾰ";
    perfect_ind_mid_2: "~ησαι", "~ησθον", _;
    perfect_ind_mid_3: "~ηται", _, "~ηνται";
    perfect_ind_pass_2: _, _, "~ησθε";
    perfect_subj_act_1: "~ηκω", _, _;
    perfect_subj_act_2: "~ηκῃς", "~ηκητον", "~ηκητε";
    perfect_subj_act_3: "~ηκῃ", _, "~ηκωσῐ|~ηκωσῐν";
    perfect_opt_act_1: "~ηκοιμῐ|~ηκοιην", _, "~ηκοιμεν";
    perfect_opt_act_2: "~ηκοις|~ηκοιης", _, _;
    perfect_opt_act_3: "~ηκοι|~ηκοιη", "~ηκοιτην", _;
    perfect_imp_act_2: "~ηκε", _, "~ηκετε";
    perfect_imp_act_3: _, _, "~ηκοντων";
    perfect_imp_mid_2: _, _, "~ησθε";
    perfect_imp_mid_3: "~ησθω", _, "~ησθων";
    perfect_imp_pass_2: "~ησο", _, "~ησθε";
    perfect_imp_pass_3: _, "~ησθων", "~ησθων";
    pluperfect_mid_1: "~ημην", _, _;
    pluperfect_mid_2: "~ησο", "~ησθον", _;
    pluperfect_mid_3: "~ητο", _, "~ηντο";
    pluperfect_pass_1: _, _, "~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ησθε";
    pluperfect_pass_3: _, "~ησθην", _;
    infinitive_perf: _, "~ησθαι", _;
    participle_perf: "~ηκως", _, "~ημενος";
    ),
    // Κτῐζω: 21 words (σχῐζω, ὀνομᾰζω, σχεδῐᾰζω, ἁρμοζω, κτῐζω, σπιζω). Like Ψηφῐζω, except in 24 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 pl, ….
    conjugation!("κτῐζω", 21, "~ζω", like Ψηφῐζω;
    future_ind_act_1: "~σω", _, "~σομεν";
    future_ind_act_2: "~σεις", "~σετον", "~σετε";
    future_ind_act_3: _, _, "~σουσῐ|~σουσῐν";
    future_ind_mid_1: "~σομαι", _, "~σομεθᾰ";
    future_ind_mid_2: "~σῃ|~σει", "~σεσθον", _;
    future_ind_mid_3: "~σεται", _, _;
    future_opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    future_opt_act_2: derived, "~σοιτον", "~σοιτε";
    future_opt_act_3: "~σοι", "~σοιτην", "~σοιεν";
    future_opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    future_opt_mid_2: "~σοιο", _, _;
    future_opt_mid_3: _, "~σοισθην", _;
    infinitive_fut: "~σειν", _, _;
    ),
    // Ἁπτω: 21 words (κρῠπτω, ἁπτω, κῠπτω, θᾰπτω, ἀποκαλυπτω, σῠναπτω). Like Βᾰπτῐζω, except in 166 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἁπτω", 21, "~πτω", like Βᾰπτῐζω;
    stem future_pass: "~φθησ- like ἐχω";
    stem aorist_ind: "~ψ- like ἐχω";
    stem aorist_ind_pass: "~φθην- like ἐχω";
    stem aorist_pass: "~φθ- like ἐχω";
    stem pluperfect_act: "~φειν- like ἐχω";
    ind_act_1: "~πτω", _, "~πτομεν";
    ind_act_2: "~πτεις", _, _;
    ind_act_3: "~πτει", "~πτετον", "~πτουσῐ";
    ind_mid_1: "~πτομαι", _, "~πτομεθᾰ";
    ind_mid_2: "~πτῃ", "~πτεσθον", _;
    ind_pass_2: _, _, "~πτεσθε";
    ind_pass_3: "~πτεται", _, "~πτονται";
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
    opt_mid_2: "~πτοιο", "~πτοισθον", _;
    opt_mid_3: "~πτοιτο", "~πτοισθην", "~πτοιντο";
    opt_pass_2: _, _, "~πτοισθε";
    imp_act_2: "~πτε", "~πτετον", "~πτετε";
    imp_mid_2: "~πτου", _, "~πτεσθε";
    imp_mid_3: "~πτεσθω", _, "~πτεσθων";
    imp_pass_2: _, _, "~πτεσθε";
    imp_pass_3: _, "~πτεσθων", "~πτεσθων";
    imperfect_act_1: "ἐ~πτον", _, "ἐ~πτομεν";
    imperfect_act_2: "ἐ~πτες", "ἐ~πτετον", "ἐ~πτετε";
    imperfect_act_3: "ἐ~πτε", "ἐ~πτετην", "ἐ~πτον";
    imperfect_mid_1: "ἐ~πτομην", _, "ἐ~πτομεθᾰ";
    imperfect_mid_2: "ἐ~πτου", _, _;
    imperfect_mid_3: "ἐ~πτετο", _, _;
    imperfect_pass_2: _, "ἐ~πτεσθον", "ἐ~πτεσθε";
    imperfect_pass_3: _, "ἐ~πτεσθην", "ἐ~πτοντο";
    future_ind_act_1: "~ψω", _, "~ψομεν";
    future_ind_act_2: "~ψεις", "~ψετον", "~ψετε";
    future_ind_act_3: _, _, "~ψουσῐ";
    future_ind_mid_1: "~ψομαι", _, "~ψομεθᾰ";
    future_ind_mid_2: "~ψῃ", "~ψεσθον", _;
    future_ind_mid_3: "~ψεται", _, _;
    future_opt_act_1: "~ψοιμῐ", _, "~ψοιμεν";
    future_opt_act_2: _, "~ψοιτον", "~ψοιτε";
    future_opt_act_3: "~ψοι", "~ψοιτην", "~ψοιεν";
    future_opt_mid_1: "~ψοιμην", _, "~ψοιμεθᾰ";
    future_opt_mid_2: "~ψοιο", _, _;
    future_opt_mid_3: _, "~ψοισθην", _;
    aorist_subj_act_1: "~ψω", _, _;
    aorist_subj_act_2: "~ψῃς", "~ψητον", _;
    aorist_subj_mid_1: "~ψωμαι", _, _;
    aorist_subj_mid_2: "~ψῃ", "~ψησθον", _;
    aorist_opt_act_1: "~ψαιμῐ", _, "~ψαιμεν";
    aorist_opt_act_2: "~ψειᾰς", _, _;
    aorist_opt_act_3: "~ψειε", "~ψαιτην", "~ψειᾰν";
    aorist_opt_mid_1: "~ψαιμην", _, _;
    aorist_opt_mid_2: "~ψαιο", _, _;
    aorist_imp_act_2: "~ψον", _, "~ψᾰτε";
    aorist_imp_act_3: _, _, "~ψᾰντων";
    aorist_imp_mid_2: "~ψαι", "~ψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ψᾰσθων";
    perfect_ind_act_1: "%ε~φᾰ", _, "%ε~φᾰμεν";
    perfect_ind_act_2: _, "%ε~φᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~φᾱσῐ";
    perfect_ind_mid_1: "%ε~μαι", _, "%ε~μεθᾰ";
    perfect_ind_mid_3: "%ε~ται", _, "%ε~νται";
    perfect_subj_act_1: "%ε~φω", _, _;
    perfect_subj_act_2: "%ε~φῃς", "%ε~φητον", "%ε~φητε";
    perfect_subj_act_3: "%ε~φῃ", _, "%ε~φωσῐ";
    perfect_opt_act_1: "%ε~φοιμῐ", _, "%ε~φοιμεν";
    perfect_opt_act_2: "%ε~φοις", _, _;
    perfect_opt_act_3: _, "%ε~φοιτην", _;
    perfect_imp_act_2: "%ε~φε", _, "%ε~φετε";
    perfect_imp_act_3: _, _, "%ε~φοντων";
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
    participle_perf: "%ε~φως", _, "%ε~μενος";
    participle_futp: _, _, "%ε~φομενος";
    ),
    // Σκᾰνδᾰλῐζω: 19 words (γρᾰφω, σκᾰνδᾰλῐζω, δειγμᾰτῐζω, σειω, γογγῠζω, παιω). Like Ληγω, except in 166 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("σκᾰνδᾰλῐζω", 19, "~ωˇ", like Ληγω;
    stem aorist_ind: "~σ- like σκᾰνδᾰλῐζω";
    stem aorist_ind_pass: "~σθηνˇ- like ἐχω";
    ind_act_1: "~ωˇ", _, "~ομενˇ";
    ind_act_2: "~ειςˇ", _, _;
    ind_act_3: "~ειˇ", "~ετονˇ", "~ουσῐˇ|~ουσῐνˇ";
    ind_mid_1: "~ομαιˇ", _, "~ομεθᾰˇ";
    ind_mid_2: "~ῃˇ|~ειˇ", "~εσθονˇ", _;
    ind_pass_2: _, _, "~εσθεˇ";
    ind_pass_3: "~εταιˇ", _, "~ονταιˇ";
    subj_act_1: "~ωˇ", _, "~ωμενˇ";
    subj_act_2: "~ῃςˇ", _, _;
    subj_act_3: "~ῃˇ", "~ητονˇ", "~ωσῐˇ|~ωσῐνˇ";
    subj_mid_1: _, _, "~ωμεθᾰˇ";
    subj_mid_2: _, "~ησθονˇ", _;
    subj_mid_3: "~ηταιˇ", _, "~ωνταιˇ";
    subj_pass_1: "~ωμαιˇ", _, _;
    subj_pass_2: _, _, "~ησθεˇ";
    opt_act_1: "~οιμῐˇ", _, "~οιμενˇ";
    opt_act_2: "~οιςˇ", "~οιτονˇ", _;
    opt_act_3: "~οιˇ", "~οιτηνˇ", "~οιενˇ";
    opt_mid_1: "~οιμηνˇ", _, "~οιμεθᾰˇ";
    opt_mid_2: "~οιοˇ", "~οισθονˇ", _;
    opt_mid_3: "~οιτοˇ", "~οισθηνˇ", "~οιντοˇ";
    opt_pass_2: _, _, "~οισθεˇ";
    imp_act_2: "~εˇ", "~ετονˇ", "~ετεˇ";
    imp_mid_2: "~ουˇ", _, "~εσθεˇ";
    imp_mid_3: "~εσθωˇ", _, "~εσθωνˇ";
    imp_pass_2: _, _, "~εσθεˇ";
    imp_pass_3: _, "~εσθωνˇ", "~εσθωνˇ";
    imperfect_act_1: "ἐ~ονˇ", _, "ἐ~ομενˇ";
    imperfect_act_2: "ἐ~εςˇ", "ἐ~ετονˇ", "ἐ~ετεˇ";
    imperfect_act_3: "ἐ~εˇ|ἐ~ενˇ", "ἐ~ετηνˇ", "ἐ~ονˇ";
    imperfect_mid_1: "ἐ~ομηνˇ", _, "ἐ~ομεθᾰˇ";
    imperfect_mid_2: "ἐ~ουˇ", _, _;
    imperfect_mid_3: "ἐ~ετοˇ", _, _;
    imperfect_pass_2: _, "ἐ~εσθονˇ", "ἐ~εσθεˇ";
    imperfect_pass_3: _, "ἐ~εσθηνˇ", "ἐ~οντοˇ";
    future_ind_act_1: "~σωˇ", _, "~σομενˇ";
    future_ind_act_2: "~σειςˇ", "~σετονˇ", "~σετεˇ";
    future_ind_act_3: _, _, "~σουσῐˇ|~σουσῐνˇ";
    future_ind_mid_1: "~σομαιˇ", _, "~σομεθᾰˇ";
    future_ind_mid_2: "~σῃˇ|~σειˇ", "~σεσθονˇ", _;
    future_ind_mid_3: "~σεταιˇ", _, _;
    future_ind_pass_1: "~σθησομαιˇ", _, _;
    future_ind_pass_3: _, "~σθησεσθονˇ", _;
    future_opt_act_1: "~σοιμῐˇ", _, "~σοιμενˇ";
    future_opt_act_2: _, "~σοιτονˇ", "~σοιτεˇ";
    future_opt_act_3: "~σοιˇ", "~σοιτηνˇ", "~σοιενˇ";
    future_opt_mid_1: "~σοιμηνˇ", _, "~σοιμεθᾰˇ";
    future_opt_mid_2: "~σοιοˇ", _, _;
    future_opt_mid_3: _, "~σοισθηνˇ", _;
    aorist_subj_act_1: "~σωˇ", _, _;
    aorist_subj_act_2: "~σῃςˇ", "~σητονˇ", _;
    aorist_subj_act_3: _, _, "~σωσῐˇ|~σωσῐνˇ";
    aorist_subj_mid_1: "~σωμαιˇ", _, _;
    aorist_subj_mid_2: "~σῃˇ", "~σησθονˇ", _;
    aorist_subj_pass_3: "~σθῃˇ", "~σθητονˇ", _;
    aorist_opt_act_1: "~σαιμῐˇ", _, "~σαιμενˇ";
    aorist_opt_act_2: "~σειᾰςˇ|~σαιςˇ", _, _;
    aorist_opt_act_3: "~σειεˇ|~σειενˇ|~σαιˇ", "~σαιτηνˇ", "~σειᾰνˇ|~σαιενˇ";
    aorist_opt_mid_1: "~σαιμηνˇ", _, _;
    aorist_opt_mid_2: "~σαιοˇ", _, _;
    aorist_imp_act_2: "~σονˇ", _, "~σᾰτεˇ";
    aorist_imp_act_3: _, _, "~σᾰντωνˇ";
    aorist_imp_mid_2: "~σαιˇ", "~σᾰσθονˇ", _;
    aorist_imp_mid_3: _, _, "~σᾰσθωνˇ";
    aorist_imp_pass_2: "~σθητῐˇ", "~σθητονˇ", "~σθητεˇ";
    perfect_ind_act_1: "%ε~κᾰˇ", _, "%ε~κᾰμενˇ";
    perfect_ind_act_2: _, "%ε~κᾰτονˇ", _;
    perfect_ind_act_3: _, _, "%ε~κᾱσῐˇ";
    perfect_ind_mid_1: "%ε~μαιˇ", _, "%ε~μεθᾰˇ";
    perfect_ind_mid_2: "%ε~σαιˇ", "%ε~σθονˇ", _;
    perfect_ind_mid_3: "%ε~ταιˇ", _, "%ε~νταιˇ";
    perfect_ind_pass_2: _, _, "%ε~σθεˇ";
    perfect_subj_act_1: "%ε~κωˇ", _, _;
    perfect_subj_act_2: "%ε~κῃςˇ", "%ε~κητονˇ", "%ε~κητεˇ";
    perfect_subj_act_3: "%ε~κῃˇ", _, "%ε~κωσῐˇ";
    perfect_opt_act_1: "%ε~κοιμῐˇ", _, "%ε~κοιμενˇ";
    perfect_opt_act_2: "%ε~κοιςˇ", _, _;
    perfect_opt_act_3: _, "%ε~κοιτηνˇ", _;
    perfect_imp_act_2: "%ε~κεˇ", _, "%ε~κετεˇ";
    perfect_imp_act_3: _, _, "%ε~κοντωνˇ";
    perfect_imp_mid_2: _, _, "%ε~σθεˇ";
    perfect_imp_mid_3: "%ε~σθωˇ", _, "%ε~σθωνˇ";
    perfect_imp_pass_2: "%ε~σοˇ", _, "%ε~σθεˇ";
    perfect_imp_pass_3: _, "%ε~σθωνˇ", "%ε~σθωνˇ";
    pluperfect_mid_1: "ἐ%ε~μηνˇ", _, _;
    pluperfect_mid_2: "ἐ%ε~σοˇ", "ἐ%ε~σθονˇ", _;
    pluperfect_mid_3: "ἐ%ε~τοˇ", _, "ἐ%ε~ντοˇ";
    pluperfect_pass_1: _, _, "ἐ%ε~μεθᾰˇ";
    pluperfect_pass_2: _, _, "ἐ%ε~σθεˇ";
    pluperfect_pass_3: _, "ἐ%ε~σθηνˇ", _;
    infinitive_pres: "~εινˇ", _, "~εσθαιˇ";
    infinitive_fut: "~σεινˇ", _, _;
    infinitive_aor: "~σαιˇ", _, _;
    infinitive_perf: _, "%ε~σθαιˇ", _;
    participle_pres: "~ωνˇ", _, "~ομενοςˇ";
    participle_aor: "~σᾱςˇ", "~σᾰμενοςˇ", _;
    participle_perf: "%ε~κωςˇ", _, "%ε~μενοςˇ";
    ),
    // Μῑγνῡμῐ: 19 words (εἰμῐ, δῐδωμῐ, δεικνῡμῐ, ὀρνῡμῐ, ἀντᾰποδῐδωμῐ, μῑγνῡμῐ). Like Οἰομαι, except in 75 cells: ind_act_1 sg, ind_act_3 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ….
    conjugation!("μῑγνῡμῐ", 19, "~μῐ", like Οἰομαι;
    ind_act_1: "~μῐ", _, _;
    ind_act_3: "~σῐ", _, _;
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, derived, _;
    ind_pass_2: _, _, derived;
    ind_pass_3: derived, _, derived;
    subj_mid_1: _, _, derived;
    subj_mid_2: derived, derived, _;
    subj_mid_3: derived, _, derived;
    subj_pass_1: derived, _, _;
    subj_pass_2: derived, _, derived;
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, _;
    opt_mid_3: derived, derived, derived;
    opt_pass_2: _, _, derived;
    imp_mid_2: derived, _, derived;
    imp_mid_3: derived, _, derived;
    imp_pass_2: _, _, derived;
    imp_pass_3: _, derived, derived;
    imperfect_act_1: "~ν", _, _;
    imperfect_act_3: "~", _, _;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, _, _;
    imperfect_mid_3: derived, _, _;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, derived;
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, _, _;
    future_opt_mid_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: derived, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: _, derived, derived;
    infinitive_pres: _, _, derived;
    infinitive_perf: _, derived, _;
    participle_pres: _, _, derived;
    participle_perf: _, _, derived;
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
    // Πᾰραιτεομαι: 18 words (ὀρχεομαι, ἀρνεομαι, ἱκνεομαι, ἐπεξηγεομαι, ἀφῐκνεομαι, ὑπῐσχνεομαι). Like Ποιεω, except in 88 cells: ind_act_3 pl, ind_pass_2 sg, subj_act_3 pl, opt_act_1 sg, opt_act_1 pl, opt_act_2 sg, ….
    conjugation!("πᾰραιτεομαι", 18, "~εομαι", like Ποιεω;
    ind_act_3: _, _, "~οῡσῐ";
    ind_pass_2: "~εῑ", _, _;
    subj_act_3: _, _, "~ωσῐ";
    opt_act_1: "~οιην", _, "~οῑμεν";
    opt_act_2: "~οιης", "~οῑτον", derived;
    opt_act_3: "~οιη", "~οιτην", "~οῑεν";
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: _, _, derived;
    future_ind_mid_2: "~ησῃ", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: _, derived, derived;
    future_opt_act_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    perfect_ind_act_3: derived, _, "%ε~ηκᾱσῐ";
    perfect_subj_act_3: _, _, "%ε~ηκωσῐ";
    perfect_opt_act_1: "%ε~ηκοιμῐ", _, _;
    perfect_opt_act_2: "%ε~ηκοις", _, _;
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
    // Πορσῡνω: 18 words (πληθῡνω, ἀμῡνω, εὐρῡνω, δῐευθῡνω, εὐθῡνω, αἰσχῡνω). Like Κᾰλεω, except in 159 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("πορσῡνω", 18, "~ῡνω", like Κᾰλεω;
    stem future_pass: "~ῠνθησ- like ἐχω";
    stem aorist_ind: "~ῡν- like ποιεω";
    stem aorist_ind_pass: "~ῡνθην- like ἐχω";
    stem aorist_pass: "~ῡνθ- like ἐχω";
    stem pluperfect_act: "~ῠγκειν- like ἐχω";
    ind_act_1: "~ῡνω", _, "~ῡνομεν";
    ind_act_2: "~ῡνεις", _, _;
    ind_act_3: "~ῡνει", "~ῡνετον", "~ῡνουσῐ|~ῡνουσῐν";
    ind_mid_1: "~ῡνομαι", _, "~ῡνομεθᾰ";
    ind_mid_2: "~ῡνῃ|~ῡνει", "~ῡνεσθον", _;
    ind_pass_2: _, _, "~ῡνεσθε";
    ind_pass_3: "~ῡνεται", _, "~ῡνονται";
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
    opt_mid_2: "~ῡνοιο", "~ῡνοισθον", _;
    opt_mid_3: "~ῡνοιτο", "~ῡνοισθην", "~ῡνοιντο";
    opt_pass_2: _, _, "~ῡνοισθε";
    imp_act_2: "~ῡνε", "~ῡνετον", "~ῡνετε";
    imp_mid_2: "~ῡνου", _, "~ῡνεσθε";
    imp_mid_3: "~ῡνεσθω", _, "~ῡνεσθων";
    imp_pass_2: _, _, "~ῡνεσθε";
    imp_pass_3: _, "~ῡνεσθων", "~ῡνεσθων";
    imperfect_act_1: "ἐ~ῡνον", _, "ἐ~ῡνομεν";
    imperfect_act_2: "ἐ~ῡνες", "ἐ~ῡνετον", "ἐ~ῡνετε";
    imperfect_act_3: "ἐ~ῡνε|ἐ~ῡνεν", "ἐ~ῡνετην", "ἐ~ῡνον";
    imperfect_mid_1: "ἐ~ῡνομην", _, "ἐ~ῡνομεθᾰ";
    imperfect_mid_2: "ἐ~ῡνου", _, _;
    imperfect_mid_3: "ἐ~ῡνετο", _, _;
    imperfect_pass_2: _, "ἐ~ῡνεσθον", "ἐ~ῡνεσθε";
    imperfect_pass_3: _, "ἐ~ῡνεσθην", "ἐ~ῡνοντο";
    future_ind_act_1: "~ῠνω", _, "~ῠνοῡμεν";
    future_ind_act_2: "~ῠνεῑς", "~ῠνεῑτον", "~ῠνεῑτε";
    future_ind_act_3: _, _, "~ῠνοῡσῐ|~ῠνοῡσῐν";
    future_ind_mid_1: "~ῠνοῡμαι", _, "~ῠνουμεθᾰ";
    future_ind_mid_2: "~ῠνεῑ|~ῠνῃ", "~ῠνεῑσθον", _;
    future_ind_mid_3: "~ῠνεῑται", _, _;
    future_opt_act_1: "~ῠνοιην|~ῠνοῑμῐ", _, "~ῠνοῑμεν|~ῠνοιημεν";
    future_opt_act_2: "~ῠνοιης|~ῠνοῑς", "~ῠνοῑτον|~ῠνοιητον", "~ῠνοῑτε|~ῠνοιητε";
    future_opt_act_3: "~ῠνοιη|~ῠνοῑ", "~ῠνοιτην|~ῠνοιητην", "~ῠνοῑεν|~ῠνοιησᾰν";
    future_opt_mid_1: "~ῠνοιμην", _, "~ῠνοιμεθᾰ";
    future_opt_mid_2: "~ῠνοῑο", _, _;
    future_opt_mid_3: _, "~ῠνοισθην", _;
    aorist_subj_act_1: "~ῡνω", _, _;
    aorist_subj_act_2: "~ῡνῃς", "~ῡνητον", _;
    aorist_subj_mid_1: "~ῡνωμαι", _, _;
    aorist_subj_mid_2: "~ῡνῃ", "~ῡνησθον", _;
    aorist_opt_act_1: "~ῡναιμῐ", _, "~ῡναιμεν";
    aorist_opt_act_2: "~ῡνειᾰς", _, _;
    aorist_opt_act_3: "~ῡνειε", "~ῡναιτην", "~ῡνειᾰν";
    aorist_opt_mid_1: "~ῡναιμην", _, _;
    aorist_opt_mid_2: "~ῡναιο", _, _;
    aorist_imp_act_2: "~ῡνον", _, "~ῡνᾰτε";
    aorist_imp_act_3: _, _, "~ῡνᾰντων";
    aorist_imp_mid_2: "~ῡναι", "~ῡνᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῡνᾰσθων";
    perfect_ind_act_1: "%ε~ῠγκᾰ", _, "%ε~ῠγκᾰμεν";
    perfect_ind_act_2: _, "%ε~ῠγκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ῠγκᾱσῐ";
    perfect_ind_mid_1: "%ε~ῠσμαι", _, "%ε~ῠσμεθᾰ";
    perfect_ind_mid_2: "%ε~ῠνσαι", "%ε~ῠνθον", _;
    perfect_ind_mid_3: "%ε~ῠνται", _, "%ε~ῠνᾰται";
    perfect_ind_pass_2: _, _, "%ε~ῠνθε";
    perfect_subj_act_1: "%ε~ῠγκω", _, _;
    perfect_subj_act_2: "%ε~ῠγκῃς", "%ε~ῠγκητον", "%ε~ῠγκητε";
    perfect_subj_act_3: "%ε~ῠγκῃ", _, "%ε~ῠγκωσῐ";
    perfect_opt_act_1: "%ε~ῠγκοιμῐ", _, "%ε~ῠγκοιμεν";
    perfect_opt_act_2: "%ε~ῠγκοις", _, _;
    perfect_opt_act_3: _, "%ε~ῠγκοιτην", _;
    perfect_imp_act_2: "%ε~ῠγκε", _, "%ε~ῠγκετε";
    perfect_imp_act_3: _, _, "%ε~ῠγκοντων";
    perfect_imp_mid_2: _, _, "%ε~ῠνθε";
    perfect_imp_mid_3: "%ε~ῠνθω", _, "%ε~ῠνθων";
    perfect_imp_pass_2: "%ε~ῠνσο", _, "%ε~ῠνθε";
    perfect_imp_pass_3: _, "%ε~ῠνθων", "%ε~ῠνθων";
    pluperfect_mid_1: "ἐ%ε~ῠσμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ῠνσο", "ἐ%ε~ῠνθον", _;
    pluperfect_mid_3: "ἐ%ε~ῠντο", _, "ἐ%ε~ῠνᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~ῠσμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ῠνθε";
    pluperfect_pass_3: _, "ἐ%ε~ῠνθην", _;
    infinitive_pres: "~ῡνειν", _, "~ῡνεσθαι";
    infinitive_fut: "~ῠνεῑν", _, _;
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
    stem pluperfect_act: "~πο~φειν- like κᾰλεω";
    ind_act_1: "~~πω", _, "~~πομεν";
    ind_act_2: "~~πεις", _, _;
    ind_act_3: "~~πει", "~~πετον", "~~πουσῐ";
    ind_mid_1: "~~πομαι", _, "~~πομεθᾰ";
    ind_mid_2: "~~πῃ", "~~πεσθον", _;
    ind_pass_2: _, _, "~~πεσθε";
    ind_pass_3: "~~πεται", _, "~~πονται";
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
    opt_mid_2: "~~ποιο", "~~ποισθον", _;
    opt_mid_3: "~~ποιτο", "~~ποισθην", "~~ποιντο";
    opt_pass_2: _, _, "~~ποισθε";
    imp_act_2: "~~πε", "~~πετον", "~~πετε";
    imp_mid_2: "~~που", _, "~~πεσθε";
    imp_mid_3: "~~πεσθω", _, "~~πεσθων";
    imp_pass_2: _, _, "~~πεσθε";
    imp_pass_3: _, "~~πεσθων", "~~πεσθων";
    imperfect_act_1: "ἐ~~πον", _, "ἐ~~πομεν";
    imperfect_act_2: "ἐ~~πες", "ἐ~~πετον", "ἐ~~πετε";
    imperfect_act_3: "ἐ~~πε", "ἐ~~πετην", "ἐ~~πον";
    imperfect_mid_1: "ἐ~~πομην", _, "ἐ~~πομεθᾰ";
    imperfect_mid_2: "ἐ~~που", _, _;
    imperfect_mid_3: "ἐ~~πετο", _, _;
    imperfect_pass_2: _, "ἐ~~πεσθον", "ἐ~~πεσθε";
    imperfect_pass_3: _, "ἐ~~πεσθην", "ἐ~~ποντο";
    future_ind_act_1: "~~ψω", _, "~~ψομεν";
    future_ind_act_2: "~~ψεις", "~~ψετον", "~~ψετε";
    future_ind_act_3: _, _, "~~ψουσῐ";
    future_ind_mid_1: "~~ψομαι", _, "~~ψομεθᾰ";
    future_ind_mid_2: "~~ψῃ", "~~ψεσθον", _;
    future_ind_mid_3: "~~ψεται", _, _;
    future_opt_act_1: "~~ψοιμῐ", _, "~~ψοιμεν";
    future_opt_act_2: _, "~~ψοιτον", "~~ψοιτε";
    future_opt_act_3: "~~ψοι", "~~ψοιτην", "~~ψοιεν";
    future_opt_mid_1: "~~ψοιμην", _, "~~ψοιμεθᾰ";
    future_opt_mid_2: "~~ψοιο", _, _;
    future_opt_mid_3: _, "~~ψοισθην", _;
    aorist_subj_act_1: "~~ψω", _, _;
    aorist_subj_act_2: "~~ψῃς", "~~ψητον", _;
    aorist_subj_mid_1: "~~ψωμαι", _, _;
    aorist_subj_mid_2: "~~ψῃ", "~~ψησθον", _;
    aorist_opt_act_1: "~~ψαιμῐ", _, "~~ψαιμεν";
    aorist_opt_act_2: "~~ψειᾰς", _, _;
    aorist_opt_act_3: "~~ψειε", "~~ψαιτην", "~~ψειᾰν";
    aorist_opt_mid_1: "~~ψαιμην", _, _;
    aorist_opt_mid_2: "~~ψαιο", _, _;
    aorist_imp_act_2: "~~ψον", _, "~~ψᾰτε";
    aorist_imp_act_3: _, _, "~~ψᾰντων";
    aorist_imp_mid_2: "~~ψαι", "~~ψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~ψᾰσθων";
    perfect_ind_act_1: "~πο~φᾰ", _, "~πο~φᾰμεν";
    perfect_ind_act_2: _, "~πο~φᾰτον", _;
    perfect_ind_act_3: _, _, "~πο~φᾱσῐ";
    perfect_ind_mid_1: "%ε~~μαι", _, "%ε~~μεθᾰ";
    perfect_ind_mid_2: "%ε~~ψαι", "%ε~~φθον", _;
    perfect_ind_mid_3: "%ε~~πται", _, "%ε~~πᾰται";
    perfect_ind_pass_2: _, _, "%ε~~φθε";
    perfect_subj_act_1: "~πο~φω", _, _;
    perfect_subj_act_2: "~πο~φῃς", "~πο~φητον", "~πο~φητε";
    perfect_subj_act_3: "~πο~φῃ", _, "~πο~φωσῐ";
    perfect_opt_act_1: "~πο~φοιμῐ", _, "~πο~φοιμεν";
    perfect_opt_act_2: "~πο~φοις", _, _;
    perfect_opt_act_3: _, "~πο~φοιτην", _;
    perfect_imp_act_2: "~πο~φε", _, "~πο~φετε";
    perfect_imp_act_3: _, _, "~πο~φοντων";
    perfect_imp_mid_2: _, _, "%ε~~φθε";
    perfect_imp_mid_3: "%ε~~φθω", _, "%ε~~φθων";
    perfect_imp_pass_2: "%ε~~ψο", _, "%ε~~φθε";
    perfect_imp_pass_3: _, "%ε~~φθων", "%ε~~φθων";
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
    // Θωᾰω: 16 words (πεπτω, τᾰττω, ἀλεγω, ϝερρω, φᾱρω, θωαδδω). Like Οἰομαι, except in 67 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("θωᾰω", 16, "~ω", like Οἰομαι;
    ind_act_1: "~ω", _, "~ομεν";
    ind_act_2: "~εις", _, _;
    ind_act_3: "~ει", "~ετον", "~ουσῐ|~ουσῐν";
    ind_mid_2: "~εαι", _, _;
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", _, _;
    subj_act_3: "~ῃ", "~ητον", "~ωσῐ|~ωσῐν";
    subj_mid_2: "~ηαι", _, _;
    subj_pass_2: "~ηαι", _, _;
    opt_act_1: "~οιμῐ", _, "~οιμεν";
    opt_act_2: "~οις", "~οιτον", _;
    opt_act_3: "~οι", "~οιτην", "~οιεν";
    imp_act_2: "~ε", "~ετον", "~ετε";
    imp_mid_2: "~εο", _, _;
    imperfect_act_1: "ἐ~ον", _, "ἐ~ομεν";
    imperfect_act_2: "ἐ~ες", "ἐ~ετον", "ἐ~ετε";
    imperfect_act_3: "ἐ~ε", "ἐ~ετην", "ἐ~ον";
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, _, _;
    future_opt_mid_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: derived, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: _, derived, derived;
    infinitive_pres: "~ειν", _, _;
    infinitive_perf: _, derived, _;
    participle_pres: "~ων", _, _;
    participle_perf: _, _, derived;
    ),
    // Ξαινω: 16 words (κᾰθαιρω, σημαινω, φαινω, ξαινω, ὑφαινω, ὁρμαινω). Like Πεμπω, except in 159 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ξαινω", 16, "~αι~ω", like Πεμπω;
    stem future_pass: "~ᾰ~θησ- like ἐχω";
    stem aorist_ind: "~η~- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~θην- like ἐχω";
    stem aorist_pass: "~ᾰ~θ- like ἐχω";
    ind_act_1: "~αι~ω", _, "~αι~ομεν";
    ind_act_2: "~αι~εις", _, _;
    ind_act_3: "~αι~ει", "~αι~ετον", "~αι~ουσῐ";
    ind_mid_1: "~αι~ομαι", _, "~αι~ομεθᾰ";
    ind_mid_2: "~αι~ῃ", "~αι~εσθον", _;
    ind_pass_2: _, _, "~αι~εσθε";
    ind_pass_3: "~αι~εται", _, "~αι~ονται";
    subj_act_1: "~αι~ω", _, "~αι~ωμεν";
    subj_act_2: "~αι~ῃς", _, _;
    subj_act_3: "~αι~ῃ", "~αι~ητον", "~αι~ωσῐ";
    subj_mid_1: _, _, "~αι~ωμεθᾰ";
    subj_mid_2: _, "~αι~ησθον", _;
    subj_mid_3: "~αι~ηται", _, "~αι~ωνται";
    subj_pass_1: "~αι~ωμαι", _, _;
    subj_pass_2: _, _, "~αι~ησθε";
    opt_act_1: "~αι~οιμῐ", _, "~αι~οιμεν";
    opt_act_2: "~αι~οις", "~αι~οιτον", _;
    opt_act_3: "~αι~οι", "~αι~οιτην", "~αι~οιεν";
    opt_mid_1: "~αι~οιμην", _, "~αι~οιμεθᾰ";
    opt_mid_2: "~αι~οιο", "~αι~οισθον", _;
    opt_mid_3: "~αι~οιτο", "~αι~οισθην", "~αι~οιντο";
    opt_pass_2: _, _, "~αι~οισθε";
    imp_act_2: "~αι~ε", "~αι~ετον", "~αι~ετε";
    imp_mid_2: "~αι~ου", _, "~αι~εσθε";
    imp_mid_3: "~αι~εσθω", _, "~αι~εσθων";
    imp_pass_2: _, _, "~αι~εσθε";
    imp_pass_3: _, "~αι~εσθων", "~αι~εσθων";
    imperfect_act_1: "ἐ~αι~ον", _, "ἐ~αι~ομεν";
    imperfect_act_2: "ἐ~αι~ες", "ἐ~αι~ετον", "ἐ~αι~ετε";
    imperfect_act_3: "ἐ~αι~ε", "ἐ~αι~ετην", "ἐ~αι~ον";
    imperfect_mid_1: "ἐ~αι~ομην", _, "ἐ~αι~ομεθᾰ";
    imperfect_mid_2: "ἐ~αι~ου", _, _;
    imperfect_mid_3: "ἐ~αι~ετο", _, _;
    imperfect_pass_2: _, "ἐ~αι~εσθον", "ἐ~αι~εσθε";
    imperfect_pass_3: _, "ἐ~αι~εσθην", "ἐ~αι~οντο";
    future_ind_act_1: "~ᾰ~ω", _, "~ᾰ~οῡμεν";
    future_ind_act_2: "~ᾰ~εῑς", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    future_ind_act_3: _, _, "~ᾰ~οῡσῐ";
    future_ind_mid_1: "~ᾰ~οῡμαι", _, "~ᾰ~ουμεθᾰ";
    future_ind_mid_2: "~ᾰ~εῑ", "~ᾰ~εῑσθον", _;
    future_ind_mid_3: "~ᾰ~εῑται", _, _;
    future_opt_act_1: "~ᾰ~οιην", _, "~ᾰ~οῑμεν";
    future_opt_act_2: _, "~ᾰ~οῑτον", "~ᾰ~οῑτε";
    future_opt_act_3: "~ᾰ~οιη", "~ᾰ~οιτην", "~ᾰ~οῑεν";
    future_opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    future_opt_mid_2: "~ᾰ~οῑο", _, _;
    future_opt_mid_3: _, "~ᾰ~οισθην", _;
    aorist_subj_act_1: "~η~ω", _, _;
    aorist_subj_act_2: "~η~ῃς", "~η~ητον", _;
    aorist_subj_mid_1: "~η~ωμαι", _, _;
    aorist_subj_mid_2: "~η~ῃ", "~η~ησθον", _;
    aorist_opt_act_1: "~η~αιμῐ", _, "~η~αιμεν";
    aorist_opt_act_2: "~η~ειᾰς", _, _;
    aorist_opt_act_3: "~η~ειε", "~η~αιτην", "~η~ειᾰν";
    aorist_opt_mid_1: "~η~αιμην", _, _;
    aorist_opt_mid_2: "~η~αιο", _, _;
    aorist_imp_act_2: "~η~ον", _, "~η~ᾰτε";
    aorist_imp_act_3: _, _, "~η~ᾰντων";
    aorist_imp_mid_2: "~η~αι", "~η~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~η~ᾰσθων";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: _, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: derived, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: _, derived, derived;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_pres: "~αι~ειν", _, "~αι~εσθαι";
    infinitive_fut: "~ᾰ~εῑν", _, _;
    infinitive_aor: "~η~αι", _, _;
    infinitive_perf: _, derived, _;
    participle_pres: "~αι~ων", _, "~αι~ομενος";
    participle_aor: "~η~ᾱς", "~η~ᾰμενος", _;
    participle_perf: derived, _, derived;
    ),
    // Λογῐζομαι: 15 words (λογῐζομαι, χᾰρῐζομαι, λᾰζομαι, δῐᾰγωνῐζομαι, λῐᾰζομαι, πλειστηρῐζομαι). Like Χρημᾰτῐζω, except in 33 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_3 pl, future_ind_mid_2 sg, ….
    conjugation!("λογῐζομαι", 15, "~ζομαι", like Χρημᾰτῐζω;
    stem aorist_ind: "~σ- like ἐχω";
    stem pluperfect_act: "~κειν- like ἐχω";
    ind_act_3: _, _, "~ζουσῐ";
    ind_mid_2: "~ζῃ", _, _;
    subj_act_3: _, _, "~ζωσῐ";
    imperfect_act_3: "ἐ~ζε", _, _;
    future_ind_act_3: _, _, "~οῡσῐ";
    future_ind_mid_2: "~εῑ", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~οιην", _, "~οῑμεν";
    future_opt_act_2: derived, "~οῑτον", "~οῑτε";
    future_opt_act_3: "~οιη", "~οιτην", "~οῑεν";
    aorist_subj_act_3: _, _, derived;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_2: "~σειᾰς", _, _;
    aorist_opt_act_3: "~σειε", _, "~σειᾰν";
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_3: derived, _, "%ε~κᾱσῐ";
    perfect_subj_act_3: _, _, "%ε~κωσῐ";
    perfect_opt_act_1: "%ε~κοιμῐ", _, _;
    perfect_opt_act_2: "%ε~κοις", _, _;
    perfect_opt_act_3: derived, _, _;
    pluperfect_act_2: derived, _, _;
    pluperfect_act_3: derived, _, _;
    ),
    // Πειρᾰζω: 15 words (πᾰρουσῐᾰζω, πλεονᾰζω, θειᾰζω, γυμνᾰζω, πειρᾰζω, πλησῐᾰζω). Like Χρημᾰτῐζω, except in 32 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 pl, ….
    conjugation!("πειρᾰζω", 15, "~ζω", like Χρημᾰτῐζω;
    stem pluperfect_act: "~κ- like δῐκεω";
    future_ind_act_1: "~σω", _, "~σομεν";
    future_ind_act_2: "~σεις", "~σετον", "~σετε";
    future_ind_act_3: _, _, "~σουσῐ|~σουσῐν";
    future_ind_mid_1: "~σομαι", _, "~σομεθᾰ";
    future_ind_mid_2: "~σῃ|~σει", "~σεσθον", _;
    future_ind_mid_3: "~σεται", _, _;
    future_opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    future_opt_act_2: derived, "~σοιτον", "~σοιτε";
    future_opt_act_3: "~σοι", "~σοιτην", "~σοιεν";
    future_opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    future_opt_mid_2: "~σοιο", _, _;
    future_opt_mid_3: _, "~σοισθην", _;
    pluperfect_mid_1: "ἐ~σμην", _, _;
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", _;
    pluperfect_mid_3: "ἐ~στο", _, "ἐ~δᾰτο";
    pluperfect_pass_1: _, _, "ἐ~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~σθε";
    pluperfect_pass_3: _, "ἐ~σθην", _;
    infinitive_fut: "~σειν", _, _;
    ),
    // Δερκομαι: 15 words (δερκομαι, μεμφομαι, σεβομαι, τερσομαι, λεχομαι, πετομαι). Like Ξαινω, except in 145 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("δερκομαι", 15, "~ε~ομαι", like Ξαινω;
    stem future_pass: "~ε~ασθησ- like ἐχω";
    stem aorist_ind: "~~- like δερκομαι";
    stem aorist_ind_pass: "~ε~ασθην- like ἐχω";
    stem aorist_pass: "~ε~ασθ- like ἐχω";
    ind_act_1: "~ε~ομαι", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, derived, derived;
    ind_mid_1: "~ε~ομαι", _, "~ε~ομεθᾰ";
    ind_mid_2: "~ε~ῃ", "~ε~εσθον", _;
    ind_pass_2: _, _, "~ε~εσθε";
    ind_pass_3: "~ε~εται", _, "~ε~ονται";
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_1: "~ε~ωμαι", _, "~ε~ωμεθᾰ";
    subj_mid_2: "~ε~ῃ", "~ε~ησθον", _;
    subj_mid_3: "~ε~ηται", _, "~ε~ωνται";
    subj_pass_1: derived, _, _;
    subj_pass_2: "~ε~ῃ", _, "~ε~ησθε";
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, _;
    opt_act_3: derived, derived, derived;
    opt_mid_1: "~ε~οιμην", _, "~ε~οιμεθᾰ";
    opt_mid_2: "~ε~οιο", "~ε~οισθον", _;
    opt_mid_3: "~ε~οιτο", "~ε~οισθην", "~ε~οιντο";
    opt_pass_2: _, _, "~ε~οισθε";
    imp_act_2: derived, derived, derived;
    imp_mid_2: "~ε~ου", _, "~ε~εσθε";
    imp_mid_3: "~ε~εσθω", _, "~ε~εσθων";
    imp_pass_2: _, _, "~ε~εσθε";
    imp_pass_3: _, "~ε~εσθων", "~ε~εσθων";
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    imperfect_mid_1: "ἐ~ε~ομην", _, "ἐ~ε~ομεθᾰ";
    imperfect_mid_2: "ἐ~ε~ου", "ἐ~ε~εσθον", _;
    imperfect_mid_3: "ἐ~ε~ετο", "ἐ~ε~εσθην", "ἐ~ε~οντο";
    imperfect_pass_2: _, derived, "ἐ~ε~εσθε";
    imperfect_pass_3: _, derived, derived;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: _, _, derived;
    future_ind_mid_1: "~ε~ησομαι", _, "~ε~ησομεθᾰ";
    future_ind_mid_2: "~ε~ησῃ", "~ε~ησεσθον", _;
    future_ind_mid_3: "~ε~ησεται", _, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: _, derived, derived;
    future_opt_act_3: derived, derived, derived;
    future_opt_mid_1: "~ε~ησοιμην", _, "~ε~ησοιμεθᾰ";
    future_opt_mid_2: "~ε~ησοιο", _, _;
    future_opt_mid_3: _, "~ε~ησοισθην", _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_mid_1: "~~ωμαι", _, _;
    aorist_subj_mid_2: "~~ῃ", "~~ησθον", _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_mid_1: "~~οιμην", _, _;
    aorist_opt_mid_2: "~~οῑο", _, _;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_mid_2: "~~οῡ", "~~εσθον", _;
    aorist_imp_mid_3: _, _, "~~εσθων";
    perfect_ind_mid_1: "~επο~ημαι", _, "~επο~ημεθᾰ";
    perfect_ind_mid_2: "~επο~ησαι", "~επο~ησθον", _;
    perfect_ind_mid_3: "~επο~ηται", _, "~επο~ηνται";
    perfect_ind_pass_2: _, _, "~επο~ησθε";
    perfect_imp_mid_2: _, _, "~επο~ησθε";
    perfect_imp_mid_3: "~επο~ησθω", _, "~επο~ησθων";
    perfect_imp_pass_2: "~επο~ησο", _, "~επο~ησθε";
    perfect_imp_pass_3: _, "~επο~ησθων", "~επο~ησθων";
    infinitive_pres: derived, _, "~ε~εσθαι";
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    infinitive_perf: _, "~επο~ησθαι", _;
    participle_pres: derived, "~ε~ομενος", derived;
    participle_aor: derived, "~~ομενος", _;
    participle_perf: _, _, "~επο~ημενος";
    ),
    // Ἀνῐστημῐ: 15 words (ἐξῐστημῐ, ἐπεντῐθημῐ, τῐθημῐ, πᾰρῐστημῐ, ἀνῐστημῐ, κᾰθῐστημῐ). Like Πεμπω, except in 196 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀνῐστημῐ", 15, "~ῐ~ημῐ", like Πεμπω;
    stem future_pass: "~ᾰ~ησ- like ἐχω";
    stem aorist_ind: "~ε~ησ- like ἐλπω";
    stem aorist_ind_pass: "~ε~ᾰθην- like ἐλπω";
    stem aorist_pass: "~ᾰ~ᾰθ- like ἐχω";
    ind_act_1: "~ῐ~ημῐ", _, "~ῐ~ᾰμεν";
    ind_act_2: "~ῐ~ης", _, _;
    ind_act_3: "~ῐ~ησῐ|~ῐ~ησῐν", "~ῐ~ᾰτον", "~ῐ~ᾱσῐ|~ῐ~ᾱσῐν";
    ind_mid_1: "~ῐ~ᾰμαι", _, "~ῐ~ᾰμεθᾰ";
    ind_mid_2: "~ῐ~ᾰσαι", "~ῐ~ᾰσθον", _;
    ind_pass_2: _, _, "~ῐ~ᾰσθε";
    ind_pass_3: "~ῐ~ᾰται", _, "~ῐ~ᾰνται";
    subj_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    subj_act_2: "~ῐ~ῃς", _, _;
    subj_act_3: "~ῐ~ῃ", "~ῐ~ητον", "~ῐ~ωσῐ|~ῐ~ωσῐν";
    subj_mid_1: _, _, "~ῐ~ωμεθᾰ";
    subj_mid_2: _, "~ῐ~ησθον", _;
    subj_mid_3: "~ῐ~ηται", _, "~ῐ~ωνται";
    subj_pass_1: "~ῐ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῐ~ησθε";
    opt_act_1: "~ῐ~αιην", _, "~ῐ~αῑμεν|~ῐ~αιημεν";
    opt_act_2: "~ῐ~αιης", "~ῐ~αῑτον|~ῐ~αιητον", "~ῐ~αῑτε|~ῐ~αιητε";
    opt_act_3: "~ῐ~αιη", "~ῐ~αιτην|~ῐ~αιητην", "~ῐ~αῑεν|~ῐ~αιησᾰν";
    opt_mid_1: "~ῐ~αιμην", _, "~ῐ~αιμεθᾰ";
    opt_mid_2: "~ῐ~αιο", "~ῐ~αισθον", _;
    opt_mid_3: "~ῐ~αιτο", "~ῐ~αισθην", "~ῐ~αιντο";
    opt_pass_2: _, _, "~ῐ~αισθε";
    imp_act_2: "~ῐ~η", "~ῐ~ᾰτον", "~ῐ~ᾰτε";
    imp_mid_2: "~ῐ~ᾰσο|~ῐ~ω", _, "~ῐ~ᾰσθε";
    imp_mid_3: "~ῐ~ᾰσθω", _, "~ῐ~ᾰσθων";
    imp_pass_2: _, _, "~ῐ~ᾰσθε";
    imp_pass_3: _, "~ῐ~ᾰσθων", "~ῐ~ᾰσθων";
    imperfect_act_1: "~ῑ~ην", _, "~ῑ~ᾰμεν";
    imperfect_act_2: "~ῑ~ης", "~ῑ~ᾰτον", "~ῑ~ᾰτε";
    imperfect_act_3: "~ῑ~η", "~ῑ~ᾰτην", "~ῑ~ᾰσᾰν";
    imperfect_mid_1: "~ῑ~ᾰμην", _, "~ῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~ῑ~ᾰσο", _, _;
    imperfect_mid_3: "~ῑ~ᾰτο", _, _;
    imperfect_pass_2: _, "~ῑ~ᾰσθον", "~ῑ~ᾰσθε";
    imperfect_pass_3: _, "~ῑ~ᾰσθην", "~ῑ~ᾰντο";
    future_ind_act_1: "~ᾰ~ησω", _, "~ᾰ~ησομεν";
    future_ind_act_2: "~ᾰ~ησεις", "~ᾰ~ησετον", "~ᾰ~ησετε";
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ";
    future_ind_mid_1: "~ᾰ~ησομαι", _, "~ᾰ~ησομεθᾰ";
    future_ind_mid_2: "~ᾰ~ησῃ", "~ᾰ~ησεσθον", _;
    future_ind_mid_3: "~ᾰ~ησεται", _, _;
    future_opt_act_1: "~ᾰ~ησοιμῐ", _, "~ᾰ~ησοιμεν";
    future_opt_act_2: _, "~ᾰ~ησοιτον", "~ᾰ~ησοιτε";
    future_opt_act_3: "~ᾰ~ησοι", "~ᾰ~ησοιτην", "~ᾰ~ησοιεν";
    future_opt_mid_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ησοιο", _, _;
    future_opt_mid_3: _, "~ᾰ~ησοισθην", _;
    aorist_subj_act_1: "~ᾰ~ησω", _, _;
    aorist_subj_act_2: "~ᾰ~ησῃς", "~ᾰ~ησητον", _;
    aorist_subj_mid_1: "~ᾰ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ησῃ", "~ᾰ~ησησθον", _;
    aorist_opt_act_1: "~ᾰ~ησαιμῐ", _, "~ᾰ~ησαιμεν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς", _, _;
    aorist_opt_act_3: "~ᾰ~ησειε", "~ᾰ~ησαιτην", "~ᾰ~ησειᾰν";
    aorist_opt_mid_1: "~ᾰ~ησαιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~ησαιο", _, _;
    aorist_imp_act_2: "~ᾰ~ησον", _, "~ᾰ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ησᾰντων";
    aorist_imp_mid_2: "~ᾰ~ησαι", "~ᾰ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ησᾰσθων";
    perfect_ind_act_1: "~ε~ᾰκᾰ", _, "~ε~ᾰκᾰμεν";
    perfect_ind_act_2: _, "~ε~ᾰκᾰτον", _;
    perfect_ind_act_3: _, _, "~ε~ᾰκᾱσῐ";
    perfect_ind_mid_1: "~ε~ᾰμαι", _, "~ε~ᾰμεθᾰ";
    perfect_ind_mid_2: "~ε~ᾰσαι", "~ε~ᾰσθον", _;
    perfect_ind_mid_3: "~ε~ᾰται", _, "~ε~ᾰνται";
    perfect_ind_pass_2: _, _, "~ε~ᾰσθε";
    perfect_subj_act_1: "~ε~ᾰκω", _, _;
    perfect_subj_act_2: "~ε~ᾰκῃς", "~ε~ᾰκητον", "~ε~ᾰκητε";
    perfect_subj_act_3: "~ε~ᾰκῃ", _, "~ε~ᾰκωσῐ";
    perfect_opt_act_1: "~ε~ᾰκοιμῐ", _, "~ε~ᾰκοιμεν";
    perfect_opt_act_2: "~ε~ᾰκοις", _, _;
    perfect_opt_act_3: _, "~ε~ᾰκοιτην", _;
    perfect_imp_act_2: "~ε~ᾰκε", _, "~ε~ᾰκετε";
    perfect_imp_act_3: _, _, "~ε~ᾰκοντων";
    perfect_imp_mid_2: _, _, "~ε~ᾰσθε";
    perfect_imp_mid_3: "~ε~ᾰσθω", _, "~ε~ᾰσθων";
    perfect_imp_pass_2: "~ε~ᾰσο", _, "~ε~ᾰσθε";
    perfect_imp_pass_3: _, "~ε~ᾰσθων", "~ε~ᾰσθων";
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
    // Ἀραομαι: 13 words (μναομαι, καυχᾰομαι, ἀραομαι, χρᾰομαι, κτᾰομαι, μᾰσαομαι). Like Οἰομαι, except in 58 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ἀραομαι", 13, "~αομαι", like Οἰομαι;
    stem future_pass: "~ησ- like ἐχω";
    stem aorist_ind: "~ησ- like πᾰραιτεομαι";
    ind_act_1: "~αομαι", _, _;
    ind_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    ind_mid_2: "~ᾳ", "~ᾱσθον", _;
    ind_pass_2: _, _, "~ᾱσθε";
    ind_pass_3: "~ᾱται", _, "~ωνται";
    subj_mid_1: "~ωμαι", _, _;
    subj_mid_2: "~ᾳ", "~ᾱσθον", _;
    subj_mid_3: "~ᾱται", _, _;
    subj_pass_1: derived, _, _;
    subj_pass_2: "~ᾳ", _, "~ᾱσθε";
    opt_mid_1: "~ῳμην", _, "~ῳμεθᾰ";
    opt_mid_2: "~ῳο", "~ῳσθον", _;
    opt_mid_3: "~ῳτο", "~ῳσθην", "~ῳντο";
    opt_pass_2: _, _, "~ῳσθε";
    imp_mid_2: "~ω", _, "~ᾱσθε";
    imp_mid_3: "~ᾱσθω", _, "~ᾱσθων";
    imp_pass_2: _, _, "~ᾱσθε";
    imp_pass_3: _, "~ᾱσθων", "~ᾱσθων";
    imperfect_mid_1: "ἐ~ωμην", _, "ἐ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ω", "ἐ~ᾱσθον", _;
    imperfect_mid_3: "ἐ~ᾱτο", "ἐ~ᾱσθην", "ἐ~ωντο";
    imperfect_pass_2: _, derived, "ἐ~ᾱσθε";
    imperfect_pass_3: _, derived, derived;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_mid_1: "~ησωμαι", _, _;
    aorist_subj_mid_2: "~ησῃ", "~ησησθον", _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_mid_1: "~ησαιμην", _, _;
    aorist_opt_mid_2: "~ησαιο", _, _;
    aorist_imp_mid_2: "~ησαι", "~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ησᾰσθων";
    aorist_imp_pass_2: derived, derived, derived;
    infinitive_pres: _, _, "~ᾱσθαι";
    participle_pres: _, _, "~ωμενος";
    participle_aor: _, "~ησᾰμενος", _;
    ),
    // Δῐοικεω: 12 words (δῐοικοδομεω, ἐνοικεω, προσοικεω, κᾰτοικοφθορεω, κατοικονομεω, κᾰτοικοδομεω). Like Πεμπω, except in 162 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("δῐοικεω", 12, "~οι~εω", like Πεμπω;
    stem future_pass: "~οι~ηθησ- like ποιεω";
    stem aorist_ind: "~ῳ~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ῳ~ηθην- like ἐλπω";
    stem aorist_pass: "~οι~ηθ- like ποιεω";
    stem pluperfect_act: "~ῳ~ηκ- like εὐλογεω";
    ind_act_1: "~οι~ω", _, "~οι~οῡμεν";
    ind_act_2: "~οι~εῑς", _, _;
    ind_act_3: "~οι~εῑ", "~οι~εῑτον", "~οι~οῡσῐ|~οι~οῡσῐν";
    ind_mid_1: "~οι~οῡμαι", _, "~οι~ουμεθᾰ";
    ind_mid_2: "~οι~εῑ|~οι~ῃ", "~οι~εῑσθον", _;
    ind_pass_2: _, _, "~οι~εῑσθε";
    ind_pass_3: "~οι~εῑται", _, "~οι~οῡνται";
    subj_act_1: "~οι~ω", _, "~οι~ωμεν";
    subj_act_2: "~οι~ῃς", _, _;
    subj_act_3: "~οι~ῃ", "~οι~ητον", "~οι~ωσῐ|~οι~ωσῐν";
    subj_mid_1: _, _, "~οι~ωμεθᾰ";
    subj_mid_2: _, "~οι~ησθον", _;
    subj_mid_3: "~οι~ηται", _, "~οι~ωνται";
    subj_pass_1: "~οι~ωμαι", _, _;
    subj_pass_2: _, _, "~οι~ησθε";
    opt_act_1: "~οι~οιην|~οι~οῑμῐ", _, "~οι~οῑμεν|~οι~οιημεν";
    opt_act_2: "~οι~οιης|~οι~οῑς", "~οι~οῑτον|~οι~οιητον", "~οι~οῑτε|~οι~οιητε";
    opt_act_3: "~οι~οιη|~οι~οῑ", "~οι~οιτην|~οι~οιητην", "~οι~οῑεν|~οι~οιησᾰν";
    opt_mid_1: "~οι~οιμην", _, "~οι~οιμεθᾰ";
    opt_mid_2: "~οι~οῑο", "~οι~οῑσθον", _;
    opt_mid_3: "~οι~οῑτο", "~οι~οισθην", "~οι~οῑντο";
    opt_pass_2: _, _, "~οι~οῑσθε";
    imp_act_2: "~οι~ει", "~οι~εῑτον", "~οι~εῑτε";
    imp_mid_2: "~οι~οῡ", _, "~οι~εῑσθε";
    imp_mid_3: "~οι~εισθω", _, "~οι~εισθων";
    imp_pass_2: _, _, "~οι~εῑσθε";
    imp_pass_3: _, "~οι~εισθων", "~οι~εισθων";
    imperfect_act_1: "~ῳ~ουν", _, "~ῳ~οῡμεν";
    imperfect_act_2: "~ῳ~εις", "~ῳ~εῑτον", "~ῳ~εῑτε";
    imperfect_act_3: "~ῳ~ει", "~ῳ~ειτην", "~ῳ~ουν";
    imperfect_mid_1: "~ῳ~ουμην", _, "~ῳ~ουμεθᾰ";
    imperfect_mid_2: "~ῳ~οῡ", _, _;
    imperfect_mid_3: "~ῳ~εῑτο", _, _;
    imperfect_pass_2: _, "~ῳ~εῑσθον", "~ῳ~εῑσθε";
    imperfect_pass_3: _, "~ῳ~εισθην", "~ῳ~οῡντο";
    future_ind_act_1: "~οι~ησω", _, "~οι~ησομεν";
    future_ind_act_2: "~οι~ησεις", "~οι~ησετον", "~οι~ησετε";
    future_ind_act_3: _, _, "~οι~ησουσῐ|~οι~ησουσῐν";
    future_ind_mid_1: "~οι~ησομαι", _, "~οι~ησομεθᾰ";
    future_ind_mid_2: "~οι~ησῃ|~οι~ησει", "~οι~ησεσθον", _;
    future_ind_mid_3: "~οι~ησεται", _, _;
    future_opt_act_1: "~οι~ησοιμῐ", _, "~οι~ησοιμεν";
    future_opt_act_2: _, "~οι~ησοιτον", "~οι~ησοιτε";
    future_opt_act_3: "~οι~ησοι", "~οι~ησοιτην", "~οι~ησοιεν";
    future_opt_mid_1: "~οι~ησοιμην", _, "~οι~ησοιμεθᾰ";
    future_opt_mid_2: "~οι~ησοιο", _, _;
    future_opt_mid_3: _, "~οι~ησοισθην", _;
    aorist_subj_act_1: "~οι~ησω", _, _;
    aorist_subj_act_2: "~οι~ησῃς", "~οι~ησητον", _;
    aorist_subj_act_3: _, _, "~οι~ησωσῐ|~οι~ησωσῐν";
    aorist_subj_mid_1: "~οι~ησωμαι", _, _;
    aorist_subj_mid_2: "~οι~ησῃ", "~οι~ησησθον", _;
    aorist_opt_act_1: "~οι~ησαιμῐ", _, "~οι~ησαιμεν";
    aorist_opt_act_2: "~οι~ησειᾰς|~οι~ησαις", _, _;
    aorist_opt_act_3: "~οι~ησειε|~οι~ησειεν|~οι~ησαι", "~οι~ησαιτην", "~οι~ησειᾰν|~οι~ησαιεν";
    aorist_opt_mid_1: "~οι~ησαιμην", _, _;
    aorist_opt_mid_2: "~οι~ησαιο", _, _;
    aorist_imp_act_2: "~οι~ησον", _, "~οι~ησᾰτε";
    aorist_imp_act_3: _, _, "~οι~ησᾰντων";
    aorist_imp_mid_2: "~οι~ησαι", "~οι~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~οι~ησᾰσθων";
    perfect_ind_act_1: "~ῳ~ηκᾰ", _, "~ῳ~ηκᾰμεν";
    perfect_ind_act_2: _, "~ῳ~ηκᾰτον", _;
    perfect_ind_act_3: "~ῳ~ηκε|~ῳ~ηκεν", _, "~ῳ~ηκᾱσῐ|~ῳ~ηκᾱσῐν";
    perfect_ind_mid_1: "~ῳ~ημαι", _, "~ῳ~ημεθᾰ";
    perfect_ind_mid_2: "~ῳ~ησαι", "~ῳ~ησθον", _;
    perfect_ind_mid_3: "~ῳ~ηται", _, "~ῳ~ηνται";
    perfect_ind_pass_2: _, _, "~ῳ~ησθε";
    perfect_subj_act_1: "~ῳ~ηκω", _, _;
    perfect_subj_act_2: "~ῳ~ηκῃς", "~ῳ~ηκητον", "~ῳ~ηκητε";
    perfect_subj_act_3: "~ῳ~ηκῃ", _, "~ῳ~ηκωσῐ|~ῳ~ηκωσῐν";
    perfect_opt_act_1: "~ῳ~ηκοιμῐ|~ῳ~ηκοιην", _, "~ῳ~ηκοιμεν";
    perfect_opt_act_2: "~ῳ~ηκοις|~ῳ~ηκοιης", _, _;
    perfect_opt_act_3: "~ῳ~ηκοι|~ῳ~ηκοιη", "~ῳ~ηκοιτην", _;
    perfect_imp_act_2: "~ῳ~ηκε", _, "~ῳ~ηκετε";
    perfect_imp_act_3: _, _, "~ῳ~ηκοντων";
    perfect_imp_mid_2: _, _, "~ῳ~ησθε";
    perfect_imp_mid_3: "~ῳ~ησθω", _, "~ῳ~ησθων";
    perfect_imp_pass_2: "~ῳ~ησο", _, "~ῳ~ησθε";
    perfect_imp_pass_3: _, "~ῳ~ησθων", "~ῳ~ησθων";
    pluperfect_mid_1: "~ῳ~ημην", _, _;
    pluperfect_mid_2: "~ῳ~ησο", "~ῳ~ησθον", _;
    pluperfect_mid_3: "~ῳ~ητο", _, "~ῳ~ηντο";
    pluperfect_pass_1: _, _, "~ῳ~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ῳ~ησθε";
    pluperfect_pass_3: _, "~ῳ~ησθην", _;
    infinitive_pres: "~οι~εῑν", _, "~οι~εῑσθαι";
    infinitive_fut: "~οι~ησειν", _, _;
    infinitive_aor: "~οι~ησαι", _, _;
    infinitive_perf: _, "~ῳ~ησθαι", _;
    participle_pres: "~οι~ων", _, "~οι~ουμενος";
    participle_aor: "~οι~ησᾱς", "~οι~ησᾰμενος", _;
    participle_perf: "~ῳ~ηκως", _, "~ῳ~ημενος";
    ),
    // Σβεννῡμῐ: 12 words (ῥωννῡμῐ, ἑννῡμῐ, πετᾰννῡμῐ, σβεννῡμῐ, κερᾰννῡμῐ, ζωννῡμῐ). Like Κᾰλεω, except in 89 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("σβεννῡμῐ", 12, "~ννῡμῐ", like Κᾰλεω;
    stem future_pass: "~σθησ- like ἐχω";
    stem aorist_ind_pass: "~σθην- like ἐχω";
    stem aorist_pass: "~σθ- like ἐχω";
    ind_act_1: "~ννῡμῐ", _, "~ννῠμεν";
    ind_act_2: "~ννῡς", _, _;
    ind_act_3: "~ννῡσῐ", "~ννῠτον", "~ννῠᾱσῐ";
    ind_mid_1: "~ννῠμαι", _, "~ννῠμεθᾰ";
    ind_mid_2: "~ννῠσαι", "~ννῠσθον", _;
    ind_pass_2: _, _, "~ννῠσθε";
    ind_pass_3: "~ννῠται", _, "~ννῠνται";
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
    opt_mid_2: "~ννῠοιο", "~ννῠοισθον", _;
    opt_mid_3: "~ννῠοιτο", "~ννῠοισθην", "~ννῠοιντο";
    opt_pass_2: _, _, "~ννῠοισθε";
    imp_act_2: "~ννῡ", "~ννῠτον", "~ννῠτε";
    imp_mid_2: "~ννῠσο", _, "~ννῠσθε";
    imp_mid_3: "~ννῠσθω", _, "~ννῠσθων";
    imp_pass_2: _, _, "~ννῠσθε";
    imp_pass_3: _, "~ννῠσθων", "~ννῠσθων";
    imperfect_act_1: "ἐ~ννῡν", _, "ἐ~ννῠμεν";
    imperfect_act_2: "ἐ~ννῡς", "ἐ~ννῠτον", "ἐ~ννῠτε";
    imperfect_act_3: "ἐ~ννῡ", "ἐ~ννῠτην", "ἐ~ννῠσᾰν";
    imperfect_mid_1: "ἐ~ννῠμην", _, "ἐ~ννῠμεθᾰ";
    imperfect_mid_2: "ἐ~ννῠσο", _, _;
    imperfect_mid_3: "ἐ~ννῠτο", _, _;
    imperfect_pass_2: _, "ἐ~ννῠσθον", "ἐ~ννῠσθε";
    imperfect_pass_3: _, "ἐ~ννῠσθην", "ἐ~ννῠντο";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: _, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
    pluperfect_act_1: derived, _, _;
    infinitive_pres: "~ννῠναι", _, "~ννῠσθαι";
    participle_pres: "~ννῡς", _, "~ννῠμενος";
    participle_perf: derived, _, _;
    ),
    // Ἐκχεω: 12 words (κρῑνω, κλῑνω, εἰρω, ἐκχεω, ἰθῡνω, δῐᾰβαινω). Like Θωᾰω, except in 88 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, subj_mid_2 sg, subj_pass_2 sg, imp_mid_2 sg, ….
    conjugation!("ἐκχεω", 12, "~ω", like Θωᾰω;
    stem aorist_ind: "~- like ἐλπω";
    ind_act_3: _, _, "~ουσῐ";
    ind_mid_2: "~ῃ", _, _;
    subj_act_3: _, _, "~ωσῐ";
    subj_mid_2: derived, _, _;
    subj_pass_2: derived, _, _;
    imp_mid_2: "~ου", _, _;
    imperfect_act_1: "~ον", _, "~ομεν";
    imperfect_act_2: "~ες", "~ετον", "~ετε";
    imperfect_act_3: "~ε", "~ετην", "~ον";
    imperfect_mid_1: "~ομην", _, "~ομεθᾰ";
    imperfect_mid_2: "~ου", _, _;
    imperfect_mid_3: "~ετο", _, _;
    imperfect_pass_2: _, "~εσθον", "~εσθε";
    imperfect_pass_3: _, "~εσθην", "~οντο";
    aorist_subj_act_1: "~ω", _, _;
    aorist_subj_act_2: "~ῃς", "~ητον", _;
    aorist_subj_mid_1: "~ωμαι", _, _;
    aorist_subj_mid_2: "~ῃ", "~ησθον", _;
    aorist_opt_act_1: "~αιμῐ", _, "~αιμεν";
    aorist_opt_act_2: "~ειᾰς", _, _;
    aorist_opt_act_3: "~ειε", "~αιτην", "~ειᾰν";
    aorist_opt_mid_1: "~αιμην", _, _;
    aorist_opt_mid_2: "~αιο", _, _;
    aorist_imp_act_2: "~ον", _, "~ᾰτε";
    aorist_imp_act_3: _, _, "~ᾰντων";
    aorist_imp_mid_2: "~αι", "~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰσθων";
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: _, "~κᾰτον", _;
    perfect_ind_act_3: _, _, "~κᾱσῐ";
    perfect_ind_mid_1: "~μαι", _, "~μεθᾰ";
    perfect_ind_mid_2: "~σαι", "~θον", _;
    perfect_ind_mid_3: "~ται", _, "~ᾰται";
    perfect_ind_pass_2: _, _, "~θε";
    perfect_subj_act_1: "~κω", _, _;
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ";
    perfect_opt_act_1: "~κοιμῐ", _, "~κοιμεν";
    perfect_opt_act_2: "~κοις", _, _;
    perfect_opt_act_3: _, "~κοιτην", _;
    perfect_imp_act_2: "~κε", _, "~κετε";
    perfect_imp_act_3: _, _, "~κοντων";
    perfect_imp_mid_2: _, _, "~θε";
    perfect_imp_mid_3: "~θω", _, "~θων";
    perfect_imp_pass_2: "~σο", _, "~θε";
    perfect_imp_pass_3: _, "~θων", "~θων";
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
    // Πειθω: 11 words (πειθω, περθω, ληθω, κλωθω, κευθω, δῐᾰπερθω). Like Σβεννῡμῐ, except in 94 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("πειθω", 11, "~θω", like Σβεννῡμῐ;
    ind_act_1: "~θω", _, "~θομεν";
    ind_act_2: "~θεις", _, _;
    ind_act_3: "~θει", "~θετον", "~θουσῐ";
    ind_mid_1: "~θομαι", _, "~θομεθᾰ";
    ind_mid_2: "~θῃ", "~θεσθον", _;
    ind_pass_2: _, _, "~θεσθε";
    ind_pass_3: "~θεται", _, "~θονται";
    subj_act_1: "~θω", _, "~θωμεν";
    subj_act_2: "~θῃς", _, _;
    subj_act_3: "~θῃ", "~θητον", "~θωσῐ";
    subj_mid_1: _, _, "~θωμεθᾰ";
    subj_mid_2: _, "~θησθον", _;
    subj_mid_3: "~θηται", _, "~θωνται";
    subj_pass_1: "~θωμαι", _, _;
    subj_pass_2: _, _, "~θησθε";
    opt_act_1: "~θοιμῐ", _, "~θοιμεν";
    opt_act_2: "~θοις", "~θοιτον", _;
    opt_act_3: "~θοι", "~θοιτην", "~θοιεν";
    opt_mid_1: "~θοιμην", _, "~θοιμεθᾰ";
    opt_mid_2: "~θοιο", "~θοισθον", _;
    opt_mid_3: "~θοιτο", "~θοισθην", "~θοιντο";
    opt_pass_2: _, _, "~θοισθε";
    imp_act_2: "~θε", "~θετον", "~θετε";
    imp_mid_2: "~θου", _, "~θεσθε";
    imp_mid_3: "~θεσθω", _, "~θεσθων";
    imp_pass_2: _, _, "~θεσθε";
    imp_pass_3: _, "~θεσθων", "~θεσθων";
    imperfect_act_1: "ἐ~θον", _, "ἐ~θομεν";
    imperfect_act_2: "ἐ~θες", "ἐ~θετον", "ἐ~θετε";
    imperfect_act_3: "ἐ~θε", "ἐ~θετην", "ἐ~θον";
    imperfect_mid_1: "ἐ~θομην", _, "ἐ~θομεθᾰ";
    imperfect_mid_2: "ἐ~θου", _, _;
    imperfect_mid_3: "ἐ~θετο", _, _;
    imperfect_pass_2: _, "ἐ~θεσθον", "ἐ~θεσθε";
    imperfect_pass_3: _, "ἐ~θεσθην", "ἐ~θοντο";
    perfect_ind_mid_1: "%ε~σμαι", _, "%ε~σμεθᾰ";
    perfect_ind_mid_2: "%ε~σαι", "%ε~σθον", _;
    perfect_ind_mid_3: "%ε~σται", _, "%ε~θᾰται";
    perfect_ind_pass_2: _, _, "%ε~σθε";
    perfect_imp_mid_2: _, _, "%ε~σθε";
    perfect_imp_mid_3: "%ε~σθω", _, "%ε~σθων";
    perfect_imp_pass_2: "%ε~σο", _, "%ε~σθε";
    perfect_imp_pass_3: _, "%ε~σθων", "%ε~σθων";
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
    // Ἱημῐ: 11 words (ἱστημῐ, ἱημῐ, δαμνημῐ, πῐμπλημῐ, φημῐ, διατιθημῐ). Like Θωᾰω, except in 61 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἱημῐ", 11, "~ημῐ", like Θωᾰω;
    ind_act_1: "~ημῐ", _, "~ᾰμεν";
    ind_act_2: "~ης", _, _;
    ind_act_3: "~ησῐ", "~ᾰτον", "~ᾱσῐ";
    ind_mid_1: "~ᾰμαι", _, "~ᾰμεθᾰ";
    ind_mid_2: "~ᾰσαι", "~ᾰσθον", _;
    ind_pass_2: _, _, "~ᾰσθε";
    ind_pass_3: "~ᾰται", _, "~ᾰνται";
    subj_mid_2: derived, _, _;
    subj_pass_2: derived, _, _;
    opt_act_1: "~αιην", _, "~αῑμεν|~αιημεν";
    opt_act_2: "~αιης", "~αῑτον|~αιητον", "~αῑτε|~αιητε";
    opt_act_3: "~αιη", "~αιτην|~αιητην", "~αῑεν|~αιησᾰν";
    opt_mid_1: "~αιμην", _, "~αιμεθᾰ";
    opt_mid_2: "~αιο", "~αισθον", _;
    opt_mid_3: "~αιτο", "~αισθην", "~αιντο";
    opt_pass_2: _, _, "~αισθε";
    imp_act_2: "~η", "~ᾰτον", "~ᾰτε";
    imp_mid_2: "~ᾰσο", _, "~ᾰσθε";
    imp_mid_3: "~ᾰσθω", _, "~ᾰσθων";
    imp_pass_2: _, _, "~ᾰσθε";
    imp_pass_3: _, "~ᾰσθων", "~ᾰσθων";
    imperfect_act_1: "ἐ~ην", _, "ἐ~ᾰμεν";
    imperfect_act_2: "ἐ~ης", "ἐ~ᾰτον", "ἐ~ᾰτε";
    imperfect_act_3: "ἐ~η", "ἐ~ᾰτην", "ἐ~ᾰσᾰν";
    imperfect_mid_1: "ἐ~ᾰμην", _, "ἐ~ᾰμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰσο", _, _;
    imperfect_mid_3: "ἐ~ᾰτο", _, _;
    imperfect_pass_2: _, "ἐ~ᾰσθον", "ἐ~ᾰσθε";
    imperfect_pass_3: _, "ἐ~ᾰσθην", "ἐ~ᾰντο";
    infinitive_pres: "~ᾰναι", _, "~ᾰσθαι";
    participle_pres: "~ᾱς", _, "~ᾰμενος";
    ),
    // Δεῑ: 10 words (ἐοικᾰ, δυναμαι, οἰδᾰ, δεῑ, ἐτλην, συνοιδᾰ). Like Θωᾰω, except in 6 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, subj_mid_2 sg, subj_pass_2 sg, imp_mid_2 sg.
    conjugation!("δεῑ", 10, "~", like Θωᾰω;
    ind_act_3: _, _, "~ουσῐ";
    ind_mid_2: "~ῃ", _, _;
    subj_act_3: _, _, "~ωσῐ";
    subj_mid_2: derived, _, _;
    subj_pass_2: derived, _, _;
    imp_mid_2: "~ου", _, _;
    ),
    // Αἱρεω: 10 words (πωλεω, αἱρεω, ἀγραυλεω, αἰνεω, βῑνεω, ὑμνεω). Like Ἐχω, except in 70 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("αἱρεω", 10, "~εω", like Ἐχω;
    ind_act_1: "~εω", _, "~εομεν";
    ind_act_2: "~εεις", _, _;
    ind_act_3: "~εει", "~εετον", "~εουσῐ";
    ind_mid_1: "~εομαι", _, "~εομεθᾰ";
    ind_mid_2: "~εῃ", "~εεσθον", _;
    ind_pass_2: _, _, "~εεσθε";
    ind_pass_3: "~εεται", _, "~εονται";
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
    opt_mid_2: "~εοιο", "~εοισθον", _;
    opt_mid_3: "~εοιτο", "~εοισθην", "~εοιντο";
    opt_pass_2: _, _, "~εοισθε";
    imp_act_2: "~εε", "~εετον", "~εετε";
    imp_mid_2: "~εου", _, "~εεσθε";
    imp_mid_3: "~εεσθω", _, "~εεσθων";
    imp_pass_2: _, _, "~εεσθε";
    imp_pass_3: _, "~εεσθων", "~εεσθων";
    imperfect_act_1: "ἐ~εον", _, "ἐ~εομεν";
    imperfect_act_2: "ἐ~εες", "ἐ~εετον", "ἐ~εετε";
    imperfect_act_3: "ἐ~εε", "ἐ~εετην", "ἐ~εον";
    imperfect_mid_1: "ἐ~εομην", _, "ἐ~εομεθᾰ";
    imperfect_mid_2: "ἐ~εου", _, _;
    imperfect_mid_3: "ἐ~εετο", _, _;
    imperfect_pass_2: _, "ἐ~εεσθον", "ἐ~εεσθε";
    imperfect_pass_3: _, "ἐ~εεσθην", "ἐ~εοντο";
    infinitive_pres: "~εειν", _, "~εεσθαι";
    participle_pres: "~εων", _, "~εομενος";
    ),
    // Ἀειρω: 10 words (κειρω, τεινω, θεινω, ἀειρω, ἐγειρω, ἀγειρω). Like Πεμπω, except in 158 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀειρω", 10, "~ει~ω", like Πεμπω;
    stem future_pass: "~ε~θησ- like ἐχω";
    stem aorist_ind: "~ει~- like ἐχω";
    stem aorist_ind_pass: "~ε~θην- like ἐχω";
    stem aorist_pass: "~ε~θ- like ἐχω";
    stem pluperfect_act: "~εκα~κειν- like κᾰλεω";
    ind_act_1: "~ει~ω", _, "~ει~ομεν";
    ind_act_2: "~ει~εις", _, _;
    ind_act_3: "~ει~ει", "~ει~ετον", "~ει~ουσῐ";
    ind_mid_1: "~ει~ομαι", _, "~ει~ομεθᾰ";
    ind_mid_2: "~ει~ῃ", "~ει~εσθον", _;
    ind_pass_2: _, _, "~ει~εσθε";
    ind_pass_3: "~ει~εται", _, "~ει~ονται";
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
    opt_mid_2: "~ει~οιο", "~ει~οισθον", _;
    opt_mid_3: "~ει~οιτο", "~ει~οισθην", "~ει~οιντο";
    opt_pass_2: _, _, "~ει~οισθε";
    imp_act_2: "~εῑ~ε", "~ει~ετον", "~ει~ετε";
    imp_mid_2: "~ει~ου", _, "~ει~εσθε";
    imp_mid_3: "~ει~εσθω", _, "~ει~εσθων";
    imp_pass_2: _, _, "~ει~εσθε";
    imp_pass_3: _, "~ει~εσθων", "~ει~εσθων";
    imperfect_act_1: "ἐ~ει~ον", _, "ἐ~ει~ομεν";
    imperfect_act_2: "ἐ~ει~ες", "ἐ~ει~ετον", "ἐ~ει~ετε";
    imperfect_act_3: "ἐ~ει~ε", "ἐ~ει~ετην", "ἐ~ει~ον";
    imperfect_mid_1: "ἐ~ει~ομην", _, "ἐ~ει~ομεθᾰ";
    imperfect_mid_2: "ἐ~ει~ου", _, _;
    imperfect_mid_3: "ἐ~ει~ετο", _, _;
    imperfect_pass_2: _, "ἐ~ει~εσθον", "ἐ~ει~εσθε";
    imperfect_pass_3: _, "ἐ~ει~εσθην", "ἐ~ει~οντο";
    future_ind_act_1: "~ε~ω", _, "~ε~οῡμεν";
    future_ind_act_2: "~ε~εῑς", "~ε~εῑτον", "~ε~εῑτε";
    future_ind_act_3: _, _, "~ε~οῡσῐ";
    future_ind_mid_1: "~ε~οῡμαι", _, "~ε~ουμεθᾰ";
    future_ind_mid_2: "~ε~εῑ", "~ε~εῑσθον", _;
    future_ind_mid_3: "~ε~εῑται", _, _;
    future_opt_act_1: "~ε~οιην", _, "~ε~οῑμεν";
    future_opt_act_2: _, "~ε~οῑτον", "~ε~οῑτε";
    future_opt_act_3: "~ε~οιη", "~ε~οιτην", "~ε~οῑεν";
    future_opt_mid_1: "~ε~οιμην", _, "~ε~οιμεθᾰ";
    future_opt_mid_2: "~ε~οῑο", _, _;
    future_opt_mid_3: _, "~ε~οισθην", _;
    aorist_subj_act_1: "~ει~ω", _, _;
    aorist_subj_act_2: "~ει~ῃς", "~ει~ητον", _;
    aorist_subj_mid_1: "~ει~ωμαι", _, _;
    aorist_subj_mid_2: "~ει~ῃ", "~ει~ησθον", _;
    aorist_opt_act_1: "~ει~αιμῐ", _, "~ει~αιμεν";
    aorist_opt_act_2: "~ει~ειᾰς", _, _;
    aorist_opt_act_3: "~ει~ειε", "~ει~αιτην", "~ει~ειᾰν";
    aorist_opt_mid_1: "~ει~αιμην", _, _;
    aorist_opt_mid_2: "~ει~αιο", _, _;
    aorist_imp_act_2: "~εῑ~ον", _, "~ει~ᾰτε";
    aorist_imp_act_3: _, _, "~ει~ᾰντων";
    aorist_imp_mid_2: "~εῑ~αι", "~ει~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ει~ᾰσθων";
    perfect_ind_act_1: "~εκα~κᾰ", _, "~εκα~κᾰμεν";
    perfect_ind_act_2: _, "~εκα~κᾰτον", _;
    perfect_ind_act_3: _, _, "~εκα~κᾱσῐ";
    perfect_ind_mid_1: "~εκα~μαι", _, "~εκα~μεθᾰ";
    perfect_ind_mid_2: "~εκα~σαι", "~εκα~θον", _;
    perfect_ind_mid_3: "~εκα~ται", _, "~εκα~ᾰται";
    perfect_ind_pass_2: _, _, "~εκα~θε";
    perfect_subj_act_1: "~εκα~κω", _, _;
    perfect_subj_act_2: "~εκα~κῃς", "~εκα~κητον", "~εκα~κητε";
    perfect_subj_act_3: "~εκα~κῃ", _, "~εκα~κωσῐ";
    perfect_opt_act_1: "~εκα~κοιμῐ", _, "~εκα~κοιμεν";
    perfect_opt_act_2: "~εκα~κοις", _, _;
    perfect_opt_act_3: _, "~εκα~κοιτην", _;
    perfect_imp_act_2: "~εκα~κε", _, "~εκα~κετε";
    perfect_imp_act_3: _, _, "~εκα~κοντων";
    perfect_imp_mid_2: _, _, "~εκα~θε";
    perfect_imp_mid_3: "~εκα~θω", _, "~εκα~θων";
    perfect_imp_pass_2: "~εκα~σο", _, "~εκα~θε";
    perfect_imp_pass_3: _, "~εκα~θων", "~εκα~θων";
    pluperfect_mid_1: "ἐ~εκα~γμην", _, _;
    pluperfect_mid_2: "ἐ~εκα~ξο", "ἐ~εκα~χθον", _;
    pluperfect_mid_3: "ἐ~εκα~κτο", _, "ἐ~εκα~κᾰτο";
    pluperfect_pass_1: _, _, "ἐ~εκα~γμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~εκα~χθε";
    pluperfect_pass_3: _, "ἐ~εκα~χθην", _;
    infinitive_pres: "~ει~ειν", _, "~ει~εσθαι";
    infinitive_fut: "~ε~εῑν", _, _;
    infinitive_aor: "~εῑ~αι", _, _;
    infinitive_perf: _, "~εκᾱ~θαι", _;
    participle_pres: "~ει~ων", _, "~ει~ομενος";
    participle_aor: "~ει~ᾱς", "~ει~ᾰμενος", _;
    participle_perf: "~εκα~κως", _, "~εκα~μενος";
    ),
    // Χεω: 10 words (πνεω, θεω, πλεω, χεω, νεω, ἀγρεω). Like Δεῑ, except in 101 cells: ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_mid_2 du, ind_pass_2 pl, ind_pass_3 sg, ….
    conjugation!("χεω", 10, "~ω", like Δεῑ;
    stem aorist_ind_pass: "~υσθην- like ἐχω";
    stem aorist_pass: "~υσθ- like ἐχω";
    ind_act_2: "~ῑς", _, _;
    ind_act_3: "~ῑ", "~ῑτον", _;
    ind_mid_2: _, "~ῑσθον", _;
    ind_pass_2: _, _, "~ῑσθε";
    ind_pass_3: "~ῑται", _, _;
    imp_act_2: "~ῑ", "~ῑτον", "~ῑτε";
    imp_mid_2: _, _, "~ῑσθε";
    imp_mid_3: "~ισθω", _, _;
    imp_pass_2: _, _, "~ῑσθε";
    imp_pass_3: _, "~ισθων", _;
    imperfect_act_2: "ἐ~ις", "ἐ~ῑτον", "ἐ~ῑτε";
    imperfect_act_3: "ἐ~ι", "ἐ~ιτην", _;
    imperfect_mid_3: "ἐ~ῑτο", _, _;
    imperfect_pass_2: _, "ἐ~ῑσθον", "ἐ~ῑσθε";
    imperfect_pass_3: _, "ἐ~ισθην", _;
    future_ind_act_1: "~υσω", _, "~υσομεν";
    future_ind_act_2: "~υσεις", "~υσετον", "~υσετε";
    future_ind_act_3: _, _, "~υσουσῐ";
    future_ind_mid_1: "~υσομαι", _, "~υσομεθᾰ";
    future_ind_mid_2: "~υσῃ", "~υσεσθον", _;
    future_ind_mid_3: "~υσεται", _, _;
    future_opt_act_1: "~υσοιμῐ", _, "~υσοιμεν";
    future_opt_act_2: _, "~υσοιτον", "~υσοιτε";
    future_opt_act_3: "~υσοι", "~υσοιτην", "~υσοιεν";
    future_opt_mid_1: "~υσοιμην", _, "~υσοιμεθᾰ";
    future_opt_mid_2: "~υσοιο", _, _;
    future_opt_mid_3: _, "~υσοισθην", _;
    aorist_ind_act_1: "ἐ~υσᾰ", _, "ἐ~υσᾰμεν";
    aorist_ind_act_2: "ἐ~υσᾰς", "ἐ~υσᾰτον", _;
    aorist_ind_act_3: "ἐ~υσε", _, "ἐ~υσᾰν";
    aorist_subj_act_1: "~υσω", _, _;
    aorist_subj_act_2: "~υσῃς", "~υσητον", _;
    aorist_opt_act_1: "~υσαιμῐ", _, "~υσαιμεν";
    aorist_opt_act_2: "~υσειᾰς", _, _;
    aorist_opt_act_3: "~υσειε", "~υσαιτην", "~υσειᾰν";
    aorist_imp_act_2: "~ῡσον", _, "~υσᾰτε";
    aorist_imp_act_3: _, _, "~υσᾰντων";
    perfect_ind_act_1: "%ε~υκᾰ", _, "%ε~υκᾰμεν";
    perfect_ind_act_2: _, "%ε~υκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~υκᾱσῐ";
    perfect_ind_mid_1: "%ε~υσμαι", _, "%ε~υσμεθᾰ";
    perfect_ind_mid_2: "%ε~υσαι", "%ε~υσθον", _;
    perfect_ind_mid_3: "%ε~υσται", _, "%ε~υσᾰται";
    perfect_ind_pass_2: _, _, "%ε~υσθε";
    perfect_subj_act_1: "%ε~υκω", _, _;
    perfect_subj_act_2: "%ε~υκῃς", "%ε~υκητον", "%ε~υκητε";
    perfect_subj_act_3: "%ε~υκῃ", _, "%ε~υκωσῐ";
    perfect_opt_act_1: "%ε~υκοιμῐ", _, "%ε~υκοιμεν";
    perfect_opt_act_2: "%ε~υκοις", _, _;
    perfect_opt_act_3: _, "%ε~υκοιτην", _;
    perfect_imp_act_2: "%ε~υκε", _, "%ε~υκετε";
    perfect_imp_act_3: _, _, "%ε~υκοντων";
    perfect_imp_mid_2: _, _, "%ε~υσθε";
    perfect_imp_mid_3: "%ε~υσθω", _, "%ε~υσθων";
    perfect_imp_pass_2: "%ε~υσο", _, "%ε~υσθε";
    perfect_imp_pass_3: _, "%ε~υσθων", "%ε~υσθων";
    infinitive_pres: "~ῑν", _, "~ῑσθαι";
    infinitive_fut: "~υσειν", _, _;
    infinitive_aor: "~ῡσαι", _, _;
    infinitive_perf: _, "%ε~ῡσθαι", _;
    participle_aor: "~υσᾱς", _, _;
    participle_perf: "%ε~υκως", _, "%ε~υσμενος";
    ),
    // Ἀφῑημῐ: 10 words (ἀφῑημῐ, σῠνῑημῐ, ἐπᾰνῐημῐ, προϊημῐ, ἀνιημῐ, ξῠνῑημῐ). Like Νῑκαω, except in 140 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀφῑημῐ", 10, "~ῑημῐ", like Νῑκαω;
    stem aorist_ind: "~ηκ- like εὐλογεω";
    ind_act_1: "~ῑημῐ", _, "~ῑεμεν";
    ind_act_2: "~ῑης", _, _;
    ind_act_3: "~ῑησῐ|~ῑησῐν", "~ῑετον", "~ῑᾱσι|~ῑᾱσιν";
    ind_mid_1: "~ῑεμαι", _, "~ῑεμεθᾰ";
    ind_mid_2: "~ῑεσαι", "~ῑεσθον", _;
    ind_pass_2: _, _, "~ῑεσθε";
    ind_pass_3: "~ῑεται", _, "~ῑενται";
    subj_act_1: "~ῑω", _, "~ῑωμεν";
    subj_act_2: "~ῑῃς", _, _;
    subj_act_3: "~ῑῃ", "~ῑητον", "~ῑωσῐ|~ῑωσῐν";
    subj_mid_1: _, _, "~ῑωμεθᾰ";
    subj_mid_2: _, "~ῑησθον", _;
    subj_mid_3: "~ῑηται", _, "~ῑωνται";
    subj_pass_1: "~ῑωμαι", _, _;
    subj_pass_2: _, _, "~ῑησθε";
    opt_act_1: "~ῑειην", _, "~ῑεῑμεν|~ῑειημεν";
    opt_act_2: "~ῑειης", "~ῑεῑτον|~ῑειητον", "~ῑεῑτε|~ῑειητε";
    opt_act_3: "~ῑειη", "~ῑειτην|~ῑειητην", "~ῑεῑεν|~ῑειησᾰν";
    opt_mid_1: "~ῑειμην", _, "~ῑειμεθᾰ|~ῑοιμεθᾰ";
    opt_mid_2: "~ῑεῑο", "~ῑεῑσθον|~ῑοῑσθον", "~ῑεῑσθε|~ῑοῑσθε";
    opt_mid_3: "~ῑεῑτο|~ῑοῑτο", "~ῑεισθην|~ῑοισθην", "~ῑεῑντο|~ῑοῑντο";
    opt_pass_2: _, _, derived;
    imp_act_2: "~ῑει", "~ῑετον", "~ῑετε";
    imp_mid_2: "~ῑεσο|~ῑου", _, "~ῑεσθε";
    imp_mid_3: "~ῑεσθω", _, "~ῑεσθων";
    imp_pass_2: _, _, "~ῑεσθε";
    imp_pass_3: _, "~ῑεσθων", "~ῑεσθων";
    imperfect_act_1: "~ῑην", _, "~ῑεμεν";
    imperfect_act_2: "~ῑεις", "~ῑετον", "~ῑετε";
    imperfect_act_3: "~ῑει", "~ῑετην", "~ῑεσᾰν";
    imperfect_mid_1: "~ῑεμην", _, "~ῑεμεθᾰ";
    imperfect_mid_2: "~ῑεσο", _, _;
    imperfect_mid_3: "~ῑετο", _, _;
    imperfect_pass_2: _, "~ῑεσθον", "~ῑεσθε";
    imperfect_pass_3: _, "~ῑεσθην", "~ῑεντο";
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
    aorist_opt_act_1: "~ειην", _, "~εῑμεν";
    aorist_opt_act_2: "~ειης", _, _;
    aorist_opt_act_3: "~ειη", "~ειτην", "~εῑεν";
    aorist_opt_mid_1: "~ειμην", _, _;
    aorist_opt_mid_2: "~εῑο", _, _;
    aorist_imp_act_2: "~ες", _, "~ετε";
    aorist_imp_act_3: _, _, "~εντων";
    aorist_imp_mid_2: "~οῡ", "~εσθον", _;
    aorist_imp_mid_3: _, _, "~εσθων";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: "~εῑκᾰ", _, "~εικᾰμεν";
    perfect_ind_act_2: _, "~εικᾰτον", _;
    perfect_ind_act_3: _, _, "~εικᾱσῐ";
    perfect_ind_mid_1: "~εῑμαι", _, "~ειμεθᾰ";
    perfect_ind_mid_2: "~εῑσαι", "~εῑσθον", _;
    perfect_ind_mid_3: "~εῑται", _, "~εῑνται";
    perfect_ind_pass_2: _, _, "~εῑσθε";
    perfect_subj_act_1: "~εικω", _, _;
    perfect_subj_act_2: "~εικῃς", "~εικητον", "~εικητε";
    perfect_subj_act_3: "~εικῃ", _, "~εικωσῐ";
    perfect_opt_act_1: "~εικοιμῐ", _, "~εικοιμεν";
    perfect_opt_act_2: "~εικοις", _, _;
    perfect_opt_act_3: _, "~εικοιτην", _;
    perfect_imp_act_2: "~εῑκε", _, "~εικετε";
    perfect_imp_act_3: _, _, "~εικοντων";
    perfect_imp_mid_2: _, _, "~εῑσθε";
    perfect_imp_mid_3: "~εισθω", _, "~εισθων";
    perfect_imp_pass_2: "~εῑσο", _, "~εῑσθε";
    perfect_imp_pass_3: _, "~εισθων", "~εισθων";
    infinitive_pres: "~ῑεναι", _, "~ῑεσθαι";
    infinitive_aor: "~εῑναι", _, _;
    infinitive_perf: _, "~εῑσθαι", _;
    participle_pres: "~ῑεις", _, "~ῑεμενος";
    participle_aor: "~εις", "~εμενος", _;
    participle_perf: "~εικως", _, "~ειμενος";
    ),
    // Χρῑω: 9 words (δρᾱω, χρῑω, μηνῡω, ξῡω, δᾰκρῡω, τῑω). Like Κᾰλεω, except in 51 cells: ind_mid_2 sg, imperfect_act_3 sg, future_ind_act_3 pl, future_ind_mid_2 sg, aorist_subj_act_3 pl, aorist_opt_act_2 sg, ….
    conjugation!("χρῑω", 9, "~ω", like Κᾰλεω;
    stem future_pass: "~σθησ- like ἐχω";
    stem aorist_ind: "~σ- like ποιεω";
    stem aorist_ind_pass: "~σθην- like ἐχω";
    stem aorist_pass: "~σθ- like ἐχω";
    ind_mid_2: "~ῃ|~ει", _, _;
    imperfect_act_3: "ἐ~ε|ἐ~εν", _, _;
    future_ind_act_3: _, _, "~σουσῐ|~σουσῐν";
    future_ind_mid_2: "~σῃ|~σει", _, _;
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", _, "~σειᾰν|~σαιεν";
    perfect_ind_act_1: "%ε~κᾰ", _, "%ε~κᾰμεν";
    perfect_ind_act_2: _, "%ε~κᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~κᾱσῐ";
    perfect_ind_mid_1: "%ε~μαι", _, "%ε~μεθᾰ";
    perfect_ind_mid_2: "%ε~σαι", "%ε~σθον", _;
    perfect_ind_mid_3: "%ε~ται", _, "%ε~νται";
    perfect_ind_pass_2: _, _, "%ε~σθε";
    perfect_subj_act_1: "%ε~κω", _, _;
    perfect_subj_act_2: "%ε~κῃς", "%ε~κητον", "%ε~κητε";
    perfect_subj_act_3: "%ε~κῃ", _, "%ε~κωσῐ";
    perfect_opt_act_1: "%ε~κοιμῐ", _, "%ε~κοιμεν";
    perfect_opt_act_2: "%ε~κοις", _, _;
    perfect_opt_act_3: _, "%ε~κοιτην", _;
    perfect_imp_act_2: "%ε~κε", _, "%ε~κετε";
    perfect_imp_act_3: _, _, "%ε~κοντων";
    perfect_imp_mid_2: _, _, "%ε~σθε";
    perfect_imp_mid_3: "%ε~σθω", _, "%ε~σθων";
    perfect_imp_pass_2: "%ε~σο", _, "%ε~σθε";
    perfect_imp_pass_3: _, "%ε~σθων", "%ε~σθων";
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: "ἐ%ε~μην", _, _;
    pluperfect_mid_2: "ἐ%ε~σο", "ἐ%ε~σθον", _;
    pluperfect_mid_3: "ἐ%ε~το", _, "ἐ%ε~ντο";
    pluperfect_pass_1: _, _, "ἐ%ε~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~σθε";
    pluperfect_pass_3: _, "ἐ%ε~σθην", _;
    infinitive_perf: _, "%ε~σθαι", _;
    participle_perf: "%ε~κως", _, "%ε~μενος";
    ),
    // Μᾰντευομαι: 9 words (μᾰντευομαι, κουρευομαι, ὀττευομαι, λοιμευομαι, ἀστειευομαι, εἰρωνευομαι). Like Οἰομαι, except in 83 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("μᾰντευομαι", 9, "~υομαι", like Οἰομαι;
    stem future_pass: "~υθησ- like ἐχω";
    stem aorist_ind: "~υσ- like πᾰραιτεομαι";
    stem aorist_ind_pass: "~υθην- like ἐχω";
    stem aorist_pass: "~υθ- like ἐχω";
    ind_act_1: "~υομαι", _, _;
    ind_mid_1: "~υομαι", _, "~υομεθᾰ";
    ind_mid_2: "~υῃ", "~υεσθον", _;
    ind_pass_2: _, _, "~υεσθε";
    ind_pass_3: "~υεται", _, "~υονται";
    subj_mid_1: _, _, "~υωμεθᾰ";
    subj_mid_2: "~υῃ", "~υησθον", _;
    subj_mid_3: "~υηται", _, "~υωνται";
    subj_pass_1: "~υωμαι", _, _;
    subj_pass_2: "~υῃ", _, "~υησθε";
    opt_mid_1: "~υοιμην", _, "~υοιμεθᾰ";
    opt_mid_2: "~υοιο", "~υοισθον", _;
    opt_mid_3: "~υοιτο", "~υοισθην", "~υοιντο";
    opt_pass_2: _, _, "~υοισθε";
    imp_mid_2: "~υου", _, "~υεσθε";
    imp_mid_3: "~υεσθω", _, "~υεσθων";
    imp_pass_2: _, _, "~υεσθε";
    imp_pass_3: _, "~υεσθων", "~υεσθων";
    imperfect_mid_1: "ἐ~υομην", _, "ἐ~υομεθᾰ";
    imperfect_mid_2: "ἐ~υου", _, _;
    imperfect_mid_3: "ἐ~υετο", _, _;
    imperfect_pass_2: _, "ἐ~υεσθον", "ἐ~υεσθε";
    imperfect_pass_3: _, "ἐ~υεσθην", "ἐ~υοντο";
    future_ind_mid_1: "~υσομαι", _, "~υσομεθᾰ";
    future_ind_mid_2: "~υσῃ", "~υσεσθον", _;
    future_ind_mid_3: "~υσεται", _, _;
    future_opt_mid_1: "~υσοιμην", _, "~υσοιμεθᾰ";
    future_opt_mid_2: "~υσοιο", _, _;
    future_opt_mid_3: _, "~υσοισθην", _;
    aorist_subj_mid_1: "~υσωμαι", _, _;
    aorist_subj_mid_2: "~υσῃ", "~υσησθον", _;
    aorist_opt_mid_1: "~υσαιμην", _, _;
    aorist_opt_mid_2: "~υσαιο", _, _;
    aorist_imp_mid_2: "~υσαι", "~υσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~υσᾰσθων";
    perfect_ind_mid_1: "%ε~υμαι", _, "%ε~υμεθᾰ";
    perfect_ind_mid_2: "%ε~υσαι", "%ε~υσθον", _;
    perfect_ind_mid_3: "%ε~υται", _, "%ε~υνται";
    perfect_ind_pass_2: _, _, "%ε~υσθε";
    perfect_imp_mid_2: _, _, "%ε~υσθε";
    perfect_imp_mid_3: "%ε~υσθω", _, "%ε~υσθων";
    perfect_imp_pass_2: "%ε~υσο", _, "%ε~υσθε";
    perfect_imp_pass_3: _, "%ε~υσθων", "%ε~υσθων";
    pluperfect_mid_1: "ἐ%ε~υμην", _, _;
    pluperfect_mid_2: "ἐ%ε~υσο", "ἐ%ε~υσθον", _;
    pluperfect_mid_3: "ἐ%ε~υτο", _, "ἐ%ε~υντο";
    pluperfect_pass_1: _, _, "ἐ%ε~υμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~υσθε";
    pluperfect_pass_3: _, "ἐ%ε~υσθην", _;
    infinitive_pres: _, _, "~υεσθαι";
    infinitive_perf: _, "%ε~ῡσθαι", _;
    participle_pres: _, _, "~υομενος";
    participle_aor: _, "~υσᾰμενος", _;
    participle_perf: _, _, "%ε~υμενος";
    ),
    // Ἐνᾰντῐοομαι: 9 words (στραγγᾰλοομαι, χειροομαι, ἀπῐχθῠοομαι, κᾰτοικειοομαι, ἐνᾰντῐοομαι, δεξῐοομαι). Like Ἀραομαι, except in 80 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ἐνᾰντῐοομαι", 9, "~οομαι", like Ἀραομαι;
    stem future_pass: "~ωθησ- like ἐχω";
    stem aorist_ind: "~ωσ- like πᾰραιτεομαι";
    stem aorist_ind_pass: "~ωθην- like ἐχω";
    stem aorist_pass: "~ωθ- like ἐχω";
    ind_act_1: "~οομαι", _, _;
    ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    ind_mid_2: "~οῑ", "~οῡσθον", _;
    ind_pass_2: _, _, "~οῡσθε";
    ind_pass_3: "~οῡται", _, "~οῡνται";
    subj_mid_2: "~οῑ", "~ωσθον", _;
    subj_mid_3: "~ωται", _, _;
    subj_pass_2: "~οῑ", _, "~ωσθε";
    opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    opt_mid_2: "~οῑο", "~οῑσθον", _;
    opt_mid_3: "~οῑτο", "~οισθην", "~οῑντο";
    opt_pass_2: _, _, "~οῑσθε";
    imp_mid_2: "~οῡ", _, "~οῡσθε";
    imp_mid_3: "~ουσθω", _, "~ουσθων";
    imp_pass_2: _, _, "~οῡσθε";
    imp_pass_3: _, "~ουσθων", "~ουσθων";
    imperfect_mid_1: "ἐ~ουμην", _, "ἐ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~οῡ", "ἐ~οῡσθον", _;
    imperfect_mid_3: "ἐ~οῡτο", "ἐ~ουσθην", "ἐ~οῡντο";
    imperfect_pass_2: _, _, "ἐ~οῡσθε";
    future_ind_mid_1: "~ωσομαι", _, "~ωσομεθᾰ";
    future_ind_mid_2: "~ωσῃ", "~ωσεσθον", _;
    future_ind_mid_3: "~ωσεται", _, _;
    future_opt_mid_1: "~ωσοιμην", _, "~ωσοιμεθᾰ";
    future_opt_mid_2: "~ωσοιο", _, _;
    future_opt_mid_3: _, "~ωσοισθην", _;
    aorist_subj_mid_1: "~ωσωμαι", _, _;
    aorist_subj_mid_2: "~ωσῃ", "~ωσησθον", _;
    aorist_opt_mid_1: "~ωσαιμην", _, _;
    aorist_opt_mid_2: "~ωσαιο", _, _;
    aorist_imp_mid_2: "~ωσαι", "~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ωσᾰσθων";
    perfect_ind_mid_1: "%ε~ωμαι", _, "%ε~ωμεθᾰ";
    perfect_ind_mid_2: "%ε~ωσαι", "%ε~ωσθον", _;
    perfect_ind_mid_3: "%ε~ωται", _, "%ε~ωνται";
    perfect_ind_pass_2: _, _, "%ε~ωσθε";
    perfect_imp_mid_2: _, _, "%ε~ωσθε";
    perfect_imp_mid_3: "%ε~ωσθω", _, "%ε~ωσθων";
    perfect_imp_pass_2: "%ε~ωσο", _, "%ε~ωσθε";
    perfect_imp_pass_3: _, "%ε~ωσθων", "%ε~ωσθων";
    pluperfect_mid_1: "ἐ%ε~ωμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ωσο", "ἐ%ε~ωσθον", _;
    pluperfect_mid_3: "ἐ%ε~ωτο", _, "ἐ%ε~ωντο";
    pluperfect_pass_1: _, _, "ἐ%ε~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ωσθε";
    pluperfect_pass_3: _, "ἐ%ε~ωσθην", _;
    infinitive_pres: _, _, "~οῡσθαι";
    infinitive_perf: _, "%ε~ωσθαι", _;
    participle_pres: _, _, "~ουμενος";
    participle_aor: _, "~ωσᾰμενος", _;
    participle_perf: _, _, "%ε~ωμενος";
    ),
    // Κᾰταφρονεω: 9 words (κᾰταφρονεω, κᾰταργεω, κᾰτηγορεω, λοιπογρᾰφεω, τερᾰτολογεω, τερᾰτουργεω). Like Πεμπω, except in 160 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("κᾰταφρονεω", 9, "~ᾰ~εω", like Πεμπω;
    stem future_pass: "~α~ηθησ- like ποιεω";
    stem aorist_ind: "~α~ησ- like ποιεω";
    stem aorist_ind_pass: "~α~ηθην- like ἐχω";
    stem aorist_pass: "~α~ηθ- like ποιεω";
    stem pluperfect_act: "~α~ηκειν- like ἐχω";
    ind_act_1: "~α~ω", _, "~α~οῡμεν";
    ind_act_2: "~α~εῑς", _, _;
    ind_act_3: "~α~εῑ", "~α~εῑτον", "~α~οῡσῐ|~α~οῡσῐν";
    ind_mid_1: "~α~οῡμαι", _, "~α~ουμεθᾰ";
    ind_mid_2: "~α~εῑ|~α~ῃ", "~α~εῑσθον", _;
    ind_pass_2: _, _, "~α~εῑσθε";
    ind_pass_3: "~α~εῑται", _, "~α~οῡνται";
    subj_act_1: "~α~ω", _, "~α~ωμεν";
    subj_act_2: "~α~ῃς", _, _;
    subj_act_3: "~α~ῃ", "~α~ητον", "~α~ωσῐ|~α~ωσῐν";
    subj_mid_1: _, _, "~α~ωμεθᾰ";
    subj_mid_2: _, "~α~ησθον", _;
    subj_mid_3: "~α~ηται", _, "~α~ωνται";
    subj_pass_1: "~α~ωμαι", _, _;
    subj_pass_2: _, _, "~α~ησθε";
    opt_act_1: "~α~οιην|~α~οῑμῐ", _, "~α~οῑμεν|~α~οιημεν";
    opt_act_2: "~α~οιης|~α~οῑς", "~α~οῑτον|~α~οιητον", "~α~οῑτε|~α~οιητε";
    opt_act_3: "~α~οιη|~α~οῑ", "~α~οιτην|~α~οιητην", "~α~οῑεν|~α~οιησᾰν";
    opt_mid_1: "~α~οιμην", _, "~α~οιμεθᾰ";
    opt_mid_2: "~α~οῑο", "~α~οῑσθον", _;
    opt_mid_3: "~α~οῑτο", "~α~οισθην", "~α~οῑντο";
    opt_pass_2: _, _, "~α~οῑσθε";
    imp_act_2: "~α~ει", "~α~εῑτον", "~α~εῑτε";
    imp_mid_2: "~α~οῡ", _, "~α~εῑσθε";
    imp_mid_3: "~α~εισθω", _, "~α~εισθων";
    imp_pass_2: _, _, "~α~εῑσθε";
    imp_pass_3: _, "~α~εισθων", "~α~εισθων";
    imperfect_act_1: "ἐ~α~ουν", _, "ἐ~α~οῡμεν";
    imperfect_act_2: "ἐ~α~εις", "ἐ~α~εῑτον", "ἐ~α~εῑτε";
    imperfect_act_3: "ἐ~α~ει", "ἐ~α~ειτην", "ἐ~α~ουν";
    imperfect_mid_1: "ἐ~α~ουμην", _, "ἐ~α~ουμεθᾰ";
    imperfect_mid_2: "ἐ~α~οῡ", _, _;
    imperfect_mid_3: "ἐ~α~εῑτο", _, _;
    imperfect_pass_2: _, "ἐ~α~εῑσθον", "ἐ~α~εῑσθε";
    imperfect_pass_3: _, "ἐ~α~εισθην", "ἐ~α~οῡντο";
    future_ind_act_1: "~α~ησω", _, "~α~ησομεν";
    future_ind_act_2: "~α~ησεις", "~α~ησετον", "~α~ησετε";
    future_ind_act_3: _, _, "~α~ησουσῐ|~α~ησουσῐν";
    future_ind_mid_1: "~α~ησομαι", _, "~α~ησομεθᾰ";
    future_ind_mid_2: "~α~ησῃ|~α~ησει", "~α~ησεσθον", _;
    future_ind_mid_3: "~α~ησεται", _, _;
    future_opt_act_1: "~α~ησοιμῐ", _, "~α~ησοιμεν";
    future_opt_act_2: _, "~α~ησοιτον", "~α~ησοιτε";
    future_opt_act_3: "~α~ησοι", "~α~ησοιτην", "~α~ησοιεν";
    future_opt_mid_1: "~α~ησοιμην", _, "~α~ησοιμεθᾰ";
    future_opt_mid_2: "~α~ησοιο", _, _;
    future_opt_mid_3: _, "~α~ησοισθην", _;
    aorist_subj_act_1: "~α~ησω", _, _;
    aorist_subj_act_2: "~α~ησῃς", "~α~ησητον", _;
    aorist_subj_act_3: _, _, "~α~ησωσῐ|~α~ησωσῐν";
    aorist_subj_mid_1: "~α~ησωμαι", _, _;
    aorist_subj_mid_2: "~α~ησῃ", "~α~ησησθον", _;
    aorist_opt_act_1: "~α~ησαιμῐ", _, "~α~ησαιμεν";
    aorist_opt_act_2: "~α~ησειᾰς|~α~ησαις", _, _;
    aorist_opt_act_3: "~α~ησειε|~α~ησειεν|~α~ησαι", "~α~ησαιτην", "~α~ησειᾰν|~α~ησαιεν";
    aorist_opt_mid_1: "~α~ησαιμην", _, _;
    aorist_opt_mid_2: "~α~ησαιο", _, _;
    aorist_imp_act_2: "~α~ησον", _, "~α~ησᾰτε";
    aorist_imp_act_3: _, _, "~α~ησᾰντων";
    aorist_imp_mid_2: "~α~ησαι", "~α~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~α~ησᾰσθων";
    perfect_ind_act_1: "%ε~α~ηκᾰ", _, "%ε~α~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~α~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~α~ηκᾱσῐ";
    perfect_ind_mid_1: "%ε~α~ημαι", _, "%ε~α~ημεθᾰ";
    perfect_ind_mid_2: "%ε~α~ησαι", "%ε~α~ησθον", _;
    perfect_ind_mid_3: "%ε~α~ηται", _, "%ε~α~ηνται";
    perfect_ind_pass_2: _, _, "%ε~α~ησθε";
    perfect_subj_act_1: "%ε~α~ηκω", _, _;
    perfect_subj_act_2: "%ε~α~ηκῃς", "%ε~α~ηκητον", "%ε~α~ηκητε";
    perfect_subj_act_3: "%ε~α~ηκῃ", _, "%ε~α~ηκωσῐ";
    perfect_opt_act_1: "%ε~α~ηκοιμῐ", _, "%ε~α~ηκοιμεν";
    perfect_opt_act_2: "%ε~α~ηκοις", _, _;
    perfect_opt_act_3: _, "%ε~α~ηκοιτην", _;
    perfect_imp_act_2: "%ε~α~ηκε", _, "%ε~α~ηκετε";
    perfect_imp_act_3: _, _, "%ε~α~ηκοντων";
    perfect_imp_mid_2: _, _, "%ε~α~ησθε";
    perfect_imp_mid_3: "%ε~α~ησθω", _, "%ε~α~ησθων";
    perfect_imp_pass_2: "%ε~α~ησο", _, "%ε~α~ησθε";
    perfect_imp_pass_3: _, "%ε~α~ησθων", "%ε~α~ησθων";
    pluperfect_mid_1: "ἐ%ε~α~ημην", _, _;
    pluperfect_mid_2: "ἐ%ε~α~ησο", "ἐ%ε~α~ησθον", _;
    pluperfect_mid_3: "ἐ%ε~α~ητο", _, "ἐ%ε~α~ηντο";
    pluperfect_pass_1: _, _, "ἐ%ε~α~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~α~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~α~ησθην", _;
    infinitive_pres: "~α~εῑν", _, "~α~εῑσθαι";
    infinitive_fut: "~α~ησειν", _, _;
    infinitive_aor: "~α~ησαι", _, _;
    infinitive_perf: _, "%ε~α~ησθαι", _;
    participle_pres: "~α~ων", _, "~α~ουμενος";
    participle_aor: "~α~ησᾱς", "~α~ησᾰμενος", _;
    participle_perf: "%ε~α~ηκως", _, "%ε~α~ημενος";
    ),
    // Εἰσερχομαι: 9 words (ἐξερχομαι, εἰσερχομαι, ἀπερχομαι, προσερχομαι, δῐερχομαι, πᾰρερχομαι). Like Μῑγνῡμῐ, except in 87 cells: ind_act_1 sg, ind_act_3 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ….
    conjugation!("εἰσερχομαι", 9, "~ερχομαι", like Μῑγνῡμῐ;
    stem aorist_ind: "~ηλθ- like εἰσερχομαι";
    ind_act_1: "~ερχομαι", _, _;
    ind_act_3: derived, _, _;
    ind_mid_1: "~ερχομαι", _, "~ερχομεθᾰ";
    ind_mid_2: "~ερχῃ|~ερχει", "~ερχεσθον", _;
    ind_pass_2: _, _, "~ερχεσθε";
    ind_pass_3: "~ερχεται", _, "~ερχονται";
    subj_mid_1: "~ερχωμαι", _, "~ερχωμεθᾰ";
    subj_mid_2: "~ερχῃ", "~ερχησθον", _;
    subj_mid_3: "~ερχηται", _, "~ερχωνται";
    subj_pass_2: "~ερχῃ", _, "~ερχησθε";
    opt_mid_1: "~ερχοιμην", _, "~ερχοιμεθᾰ";
    opt_mid_2: "~ερχοιο", "~ερχοισθον", _;
    opt_mid_3: "~ερχοιτο", "~ερχοισθην", "~ερχοιντο";
    opt_pass_2: _, _, "~ερχοισθε";
    imp_mid_2: "~ερχου", _, "~ερχεσθε";
    imp_mid_3: "~ερχεσθω", _, "~ερχεσθων";
    imp_pass_2: _, _, "~ερχεσθε";
    imp_pass_3: _, "~ερχεσθων", "~ερχεσθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    imperfect_mid_1: "~ηρχομην", _, "~ηρχομεθᾰ";
    imperfect_mid_2: "~ηρχου", "~ηρχεσθον", _;
    imperfect_mid_3: "~ηρχετο", "~ηρχεσθην", "~ηρχοντο";
    imperfect_pass_2: _, _, "~ηρχεσθε";
    future_ind_mid_1: "~ελευσομαι", _, "~ελευσομεθᾰ";
    future_ind_mid_2: "~ελευσῃ|~ελευσει", "~ελευσεσθον", _;
    future_ind_mid_3: "~ελευσεται", _, _;
    future_opt_mid_1: "~ελευσοιμην", _, "~ελευσοιμεθᾰ";
    future_opt_mid_2: "~ελευσοιο", _, _;
    future_opt_mid_3: _, "~ελευσοισθην", _;
    aorist_subj_act_1: "~ελθω", _, _;
    aorist_subj_act_2: "~ελθῃς", "~ελθητον", _;
    aorist_subj_act_3: _, _, "~ελθωσῐ|~ελθωσῐν";
    aorist_opt_act_1: "~ελθοιμῐ", _, "~ελθοιμεν";
    aorist_opt_act_2: "~ελθοις", _, _;
    aorist_opt_act_3: "~ελθοι", "~ελθοιτην", "~ελθοιεν";
    aorist_imp_act_2: "~ελθε", _, "~ελθετε";
    aorist_imp_act_3: _, _, "~ελθοντων";
    perfect_ind_act_1: "~εληλῠθᾰ", _, "~εληλυμεν";
    perfect_ind_act_2: _, "~εληλῠθᾰτον", _;
    perfect_ind_act_3: _, _, "~εληλῠθᾱσῐ";
    perfect_subj_act_1: "~εληλῠθω", _, _;
    perfect_subj_act_2: "~εληλῠθῃς", "~εληλῠθητον", "~εληλῠθητε";
    perfect_subj_act_3: "~εληλῠθῃ", _, "~εληλῠθωσῐ";
    perfect_opt_act_1: "~εληλῠθοιμῐ", _, "~εληλῠθοιμεν";
    perfect_opt_act_2: "~εληλῠθοις", _, _;
    perfect_opt_act_3: _, "~εληλῠθοιτην", _;
    perfect_imp_act_2: "~εληλῠθε", _, "~εληλῠθετε";
    perfect_imp_act_3: _, _, "~εληλῠθοντων";
    pluperfect_act_1: "~εληλῠθειν", _, _;
    infinitive_pres: _, _, "~ερχεσθαι";
    infinitive_aor: "~ελθεῑν", _, _;
    participle_pres: _, _, "~ερχομενος";
    participle_aor: "~ελθων", _, _;
    participle_perf: "~εληλῠθως", _, _;
    ),
    // Ἐπᾰϊω: 9 words (ὑπᾰρχω, δῐᾰνῠω, ἐξᾰρχω, ἐξᾰρτῡω, ἀπᾰρεσκω, ἐπᾰρχω). Like Πεμπω, except in 160 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐπᾰϊω", 9, "~ᾰ~ω", like Πεμπω;
    stem future_pass: "~ᾰ~σθησ- like ἐχω";
    stem aorist_ind: "~η~σ- like εὐλογεω";
    stem aorist_ind_pass: "~η~σθην- like ἐλπω";
    stem aorist_pass: "~ᾰ~σθ- like ἐχω";
    ind_act_1: "~ᾰ~ω", _, "~ᾰ~ομεν";
    ind_act_2: "~ᾰ~εις", _, _;
    ind_act_3: "~ᾰ~ει", "~ᾰ~ετον", "~ᾰ~ουσῐ|~ᾰ~ουσῐν";
    ind_mid_1: "~ᾰ~ομαι", _, "~ᾰ~ομεθᾰ";
    ind_mid_2: "~ᾰ~ῃ|~ᾰ~ει", "~ᾰ~εσθον", _;
    ind_pass_2: _, _, "~ᾰ~εσθε";
    ind_pass_3: "~ᾰ~εται", _, "~ᾰ~ονται";
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
    opt_mid_2: "~ᾰ~οιο", "~ᾰ~οισθον", _;
    opt_mid_3: "~ᾰ~οιτο", "~ᾰ~οισθην", "~ᾰ~οιντο";
    opt_pass_2: _, _, "~ᾰ~οισθε";
    imp_act_2: "~ᾰ~ε", "~ᾰ~ετον", "~ᾰ~ετε";
    imp_mid_2: "~ᾰ~ου", _, "~ᾰ~εσθε";
    imp_mid_3: "~ᾰ~εσθω", _, "~ᾰ~εσθων";
    imp_pass_2: _, _, "~ᾰ~εσθε";
    imp_pass_3: _, "~ᾰ~εσθων", "~ᾰ~εσθων";
    imperfect_act_1: "~η~ον", _, "~η~ομεν";
    imperfect_act_2: "~η~ες", "~η~ετον", "~η~ετε";
    imperfect_act_3: "~η~ε|~η~εν", "~η~ετην", "~η~ον";
    imperfect_mid_1: "~η~ομην", _, "~η~ομεθᾰ";
    imperfect_mid_2: "~η~ου", _, _;
    imperfect_mid_3: "~η~ετο", _, _;
    imperfect_pass_2: _, "~η~εσθον", "~η~εσθε";
    imperfect_pass_3: _, "~η~εσθην", "~η~οντο";
    future_ind_act_1: "~ᾰ~σω", _, "~ᾰ~σομεν";
    future_ind_act_2: "~ᾰ~σεις", "~ᾰ~σετον", "~ᾰ~σετε";
    future_ind_act_3: _, _, "~ᾰ~σουσῐ";
    future_ind_mid_1: "~ᾰ~σομαι", _, "~ᾰ~σομεθᾰ";
    future_ind_mid_2: "~ᾰ~σῃ", "~ᾰ~σεσθον", _;
    future_ind_mid_3: "~ᾰ~σεται", _, _;
    future_opt_act_1: "~ᾰ~σοιμῐ", _, "~ᾰ~σοιμεν";
    future_opt_act_2: _, "~ᾰ~σοιτον", "~ᾰ~σοιτε";
    future_opt_act_3: "~ᾰ~σοι", "~ᾰ~σοιτην", "~ᾰ~σοιεν";
    future_opt_mid_1: "~ᾰ~σοιμην", _, "~ᾰ~σοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~σοιο", _, _;
    future_opt_mid_3: _, "~ᾰ~σοισθην", _;
    aorist_subj_act_1: "~ᾰ~σω", _, _;
    aorist_subj_act_2: "~ᾰ~σῃς", "~ᾰ~σητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~σωσῐ|~ᾰ~σωσῐν";
    aorist_subj_mid_1: "~ᾰ~σωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~σῃ", "~ᾰ~σησθον", _;
    aorist_opt_act_1: "~ᾰ~σαιμῐ", _, "~ᾰ~σαιμεν";
    aorist_opt_act_2: "~ᾰ~σειᾰς|~ᾰ~σαις", _, _;
    aorist_opt_act_3: "~ᾰ~σειε|~ᾰ~σειεν|~ᾰ~σαι", "~ᾰ~σαιτην", "~ᾰ~σειᾰν|~ᾰ~σαιεν";
    aorist_opt_mid_1: "~ᾰ~σαιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~σαιο", _, _;
    aorist_imp_act_2: "~ᾰ~σον", _, "~ᾰ~σᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~σᾰντων";
    aorist_imp_mid_2: "~ᾰ~σαι", "~ᾰ~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~σᾰσθων";
    perfect_ind_act_1: "~η~κᾰ", _, "~η~κᾰμεν";
    perfect_ind_act_2: _, "~η~κᾰτον", _;
    perfect_ind_act_3: _, _, "~η~κᾱσῐ";
    perfect_ind_mid_1: "~η~σμαι", _, "~η~σμεθᾰ";
    perfect_ind_mid_2: "~η~σαι", "~η~σθον", _;
    perfect_ind_mid_3: "~η~σται", _, "~η~τᾰται";
    perfect_ind_pass_2: _, _, "~η~σθε";
    perfect_subj_act_1: "~η~κω", _, _;
    perfect_subj_act_2: "~η~κῃς", "~η~κητον", "~η~κητε";
    perfect_subj_act_3: "~η~κῃ", _, "~η~κωσῐ";
    perfect_opt_act_1: "~η~κοιμῐ", _, "~η~κοιμεν";
    perfect_opt_act_2: "~η~κοις", _, _;
    perfect_opt_act_3: _, "~η~κοιτην", _;
    perfect_imp_act_2: "~η~κε", _, "~η~κετε";
    perfect_imp_act_3: _, _, "~η~κοντων";
    perfect_imp_mid_2: _, _, "~η~σθε";
    perfect_imp_mid_3: "~η~σθω", _, "~η~σθων";
    perfect_imp_pass_2: "~η~σο", _, "~η~σθε";
    perfect_imp_pass_3: _, "~η~σθων", "~η~σθων";
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
    // Ψῑλοω: 8 words (ἱδροω, στενοω, πτεροω, ψῑλοω, κεραυνοω, φορτοω). Like Γῡροω, except in 45 cells: future_ind_act_3 pl, perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, ….
    conjugation!("ψῑλοω", 8, "~οω", like Γῡροω;
    stem future_pass: "~ωθησ- like ποιεω";
    stem aorist_pass: "~ωθ- like ποιεω";
    stem pluperfect_act: "~ωκ- like δῐκεω";
    future_ind_act_3: _, _, "~ωσουσῐ|~ωσουσῐν";
    perfect_ind_act_1: "ἐ~ωκᾰ", _, "ἐ~ωκᾰμεν";
    perfect_ind_act_2: _, "ἐ~ωκᾰτον", _;
    perfect_ind_act_3: "ἐ~ωκε|ἐ~ωκεν", _, "ἐ~ωκᾱσῐ|ἐ~ωκᾱσῐν";
    perfect_ind_mid_1: "ἐ~ωμαι", _, "ἐ~ωμεθᾰ";
    perfect_ind_mid_2: "ἐ~ωσαι", "ἐ~ωσθον", _;
    perfect_ind_mid_3: "ἐ~ωται", _, "ἐ~ωνται";
    perfect_ind_pass_2: _, _, "ἐ~ωσθε";
    perfect_subj_act_1: "ἐ~ωκω", _, _;
    perfect_subj_act_2: "ἐ~ωκῃς", "ἐ~ωκητον", "ἐ~ωκητε";
    perfect_subj_act_3: "ἐ~ωκῃ", _, "ἐ~ωκωσῐ|ἐ~ωκωσῐν";
    perfect_opt_act_1: "ἐ~ωκοιμῐ|ἐ~ωκοιην", _, "ἐ~ωκοιμεν";
    perfect_opt_act_2: "ἐ~ωκοις|ἐ~ωκοιης", _, _;
    perfect_opt_act_3: "ἐ~ωκοι|ἐ~ωκοιη", "ἐ~ωκοιτην", _;
    perfect_imp_act_2: "ἐ~ωκε", _, "ἐ~ωκετε";
    perfect_imp_act_3: _, _, "ἐ~ωκοντων";
    perfect_imp_mid_2: _, _, "ἐ~ωσθε";
    perfect_imp_mid_3: "ἐ~ωσθω", _, "ἐ~ωσθων";
    perfect_imp_pass_2: "ἐ~ωσο", _, "ἐ~ωσθε";
    perfect_imp_pass_3: _, "ἐ~ωσθων", "ἐ~ωσθων";
    pluperfect_mid_1: "ἐ~ωμην", _, _;
    pluperfect_mid_2: "ἐ~ωσο", "ἐ~ωσθον", _;
    pluperfect_mid_3: "ἐ~ωτο", _, "ἐ~ωντο";
    pluperfect_pass_1: _, _, "ἐ~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ωσθε";
    pluperfect_pass_3: _, "ἐ~ωσθην", _;
    infinitive_perf: _, "ἐ~ωσθαι", _;
    participle_perf: "ἐ~ωκως", _, "ἐ~ωμενος";
    ),
    // Ἀνᾰγω: 8 words (ἐξᾰγω, ὑπᾰγω, ἀνᾰγω, πᾰρᾰγω, ἐπᾰγω, κᾰτᾰγω). Like Ληγω, except in 151 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀνᾰγω", 8, "~ᾰγω", like Ληγω;
    stem future_pass: "~ᾰχθησ- like ποιεω";
    stem aorist_ind: "~ηγᾰγ- like ἀνᾰγω";
    stem aorist_ind_pass: "~ηχθην- like ἐλπω";
    stem aorist_pass: "~ᾰχθ- like ἐχω";
    ind_act_1: "~ᾰγω", _, "~ᾰγομεν";
    ind_act_2: "~ᾰγεις", _, _;
    ind_act_3: "~ᾰγει", "~ᾰγετον", "~ᾰγουσῐ|~ᾰγουσῐν";
    ind_mid_1: "~ᾰγομαι", _, "~ᾰγομεθᾰ";
    ind_mid_2: "~ᾰγῃ|~ᾰγει", "~ᾰγεσθον", _;
    ind_pass_2: _, _, "~ᾰγεσθε";
    ind_pass_3: "~ᾰγεται", _, "~ᾰγονται";
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
    opt_mid_2: "~ᾰγοιο", "~ᾰγοισθον", _;
    opt_mid_3: "~ᾰγοιτο", "~ᾰγοισθην", "~ᾰγοιντο";
    opt_pass_2: _, _, "~ᾰγοισθε";
    imp_act_2: "~ᾰγε", "~ᾰγετον", "~ᾰγετε";
    imp_mid_2: "~ᾰγου", _, "~ᾰγεσθε";
    imp_mid_3: "~ᾰγεσθω", _, "~ᾰγεσθων";
    imp_pass_2: _, _, "~ᾰγεσθε";
    imp_pass_3: _, "~ᾰγεσθων", "~ᾰγεσθων";
    imperfect_act_1: "~ηγον", _, "~ηγομεν";
    imperfect_act_2: "~ηγες", "~ηγετον", "~ηγετε";
    imperfect_act_3: "~ηγε|~ηγεν", "~ηγετην", "~ηγον";
    imperfect_mid_1: "~ηγομην", _, "~ηγομεθᾰ";
    imperfect_mid_2: "~ηγου", _, _;
    imperfect_mid_3: "~ηγετο", _, _;
    imperfect_pass_2: _, "~ηγεσθον", "~ηγεσθε";
    imperfect_pass_3: _, "~ηγεσθην", "~ηγοντο";
    future_ind_act_1: "~ᾰξω", _, "~ᾰξομεν";
    future_ind_act_2: "~ᾰξεις", "~ᾰξετον", "~ᾰξετε";
    future_ind_act_3: _, _, "~ᾰξουσῐ|~ᾰξουσῐν";
    future_ind_mid_1: "~ᾰξομαι", _, "~ᾰξομεθᾰ";
    future_ind_mid_2: "~ᾰξῃ|~ᾰξει", "~ᾰξεσθον", _;
    future_ind_mid_3: "~ᾰξεται", _, _;
    future_opt_act_1: "~ᾰξοιμῐ", _, "~ᾰξοιμεν";
    future_opt_act_2: _, "~ᾰξοιτον", "~ᾰξοιτε";
    future_opt_act_3: "~ᾰξοι", "~ᾰξοιτην", "~ᾰξοιεν";
    future_opt_mid_1: "~ᾰξοιμην", _, "~ᾰξοιμεθᾰ";
    future_opt_mid_2: "~ᾰξοιο", _, _;
    future_opt_mid_3: _, "~ᾰξοισθην", _;
    aorist_subj_act_1: "~ᾰγᾰγω", _, _;
    aorist_subj_act_2: "~ᾰγᾰγῃς", "~ᾰγᾰγητον", _;
    aorist_subj_act_3: _, _, "~ᾰγᾰγωσῐ|~ᾰγᾰγωσῐν";
    aorist_subj_mid_1: "~ᾰγᾰγωμαι", _, _;
    aorist_subj_mid_2: "~ᾰγᾰγῃ", "~ᾰγᾰγησθον", _;
    aorist_opt_act_1: "~ᾰγᾰγοιμῐ", _, "~ᾰγᾰγοιμεν";
    aorist_opt_act_2: "~ᾰγᾰγοις", _, _;
    aorist_opt_act_3: "~ᾰγᾰγοι", "~ᾰγᾰγοιτην", "~ᾰγᾰγοιεν";
    aorist_opt_mid_1: "~ᾰγᾰγοιμην", _, _;
    aorist_opt_mid_2: "~ᾰγᾰγοιο", _, _;
    aorist_imp_act_2: "~ᾰγᾰγε", _, "~ᾰγᾰγετε";
    aorist_imp_act_3: _, _, "~ᾰγᾰγοντων";
    aorist_imp_mid_2: "~ᾰγᾰγοῡ", "~ᾰγᾰγεσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰγᾰγεσθων";
    perfect_ind_act_1: "~ηχᾰ", _, "~ηχᾰμεν";
    perfect_ind_act_2: _, "~ηχᾰτον", _;
    perfect_ind_act_3: _, _, "~ηχᾱσῐ";
    perfect_ind_mid_1: "~ηγμαι", _, "~ηγμεθᾰ";
    perfect_ind_mid_2: "~ηξαι", "~ηχθον", _;
    perfect_ind_mid_3: "~ηκται", _, "~ηγᾰται";
    perfect_ind_pass_2: _, _, "~ηχθε";
    perfect_subj_act_1: "~ηχω", _, _;
    perfect_subj_act_2: "~ηχῃς", "~ηχητον", "~ηχητε";
    perfect_subj_act_3: "~ηχῃ", _, "~ηχωσῐ";
    perfect_opt_act_1: "~ηχοιμῐ", _, "~ηχοιμεν";
    perfect_opt_act_2: "~ηχοις", _, _;
    perfect_opt_act_3: _, "~ηχοιτην", _;
    perfect_imp_act_2: "~ηχε", _, "~ηχετε";
    perfect_imp_act_3: _, _, "~ηχοντων";
    perfect_imp_mid_2: _, _, "~ηχθε";
    perfect_imp_mid_3: "~ηχθω", _, "~ηχθων";
    perfect_imp_pass_2: "~ηξο", _, "~ηχθε";
    perfect_imp_pass_3: _, "~ηχθων", "~ηχθων";
    infinitive_pres: "~ᾰγειν", _, "~ᾰγεσθαι";
    infinitive_fut: "~ᾰξειν", _, _;
    infinitive_aor: "~ᾰγᾰγεῑν", _, _;
    infinitive_perf: _, "~ηχθαι", _;
    participle_pres: "~ᾰγων", _, "~ᾰγομενος";
    participle_aor: "~ᾰγᾰγων", "~ᾰγᾰγομενος", _;
    participle_perf: "~ηχως", _, "~ηγμενος";
    ),
    // Φᾰρμᾰσσω: 8 words (τᾰρᾰσσω, θωρησσω, κνωσσω, θρῡλῐσσω, παιφᾰσσω, φᾰρμᾰσσω). Like Τᾰσσω, except in 46 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_3 pl, future_ind_mid_2 sg, ….
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
    aorist_opt_act_3: "~ξοι", "~ξοιτην", "~ξοιεν";
    aorist_opt_mid_1: "~ξοιμην", _, _;
    aorist_opt_mid_2: "~ξοιο", _, _;
    aorist_imp_act_2: "~ξε", _, "~ξετε";
    aorist_imp_act_3: _, _, "~ξοντων";
    aorist_imp_mid_2: "~ξοῡ", "~ξεσθον", _;
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
    infinitive_aor: "~ξεῑν", _, _;
    infinitive_futp: _, _, "%ε~ξεσθαι";
    participle_aor: "~ξων", "~ξομενος", _;
    participle_futp: _, _, "%ε~ξομενος";
    ),
    // Δῐωκω: 8 words (βρῡκω, πεκω, πλεκω, τηκω, κᾰτᾰδῐωκω, δῐωκω). Like Τᾰσσω, except in 84 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("δῐωκω", 8, "~κω", like Τᾰσσω;
    stem aorist_ind: "~ξ- like ἐχω";
    ind_act_1: "~κω", _, "~κομεν";
    ind_act_2: "~κεις", _, _;
    ind_act_3: "~κει", "~κετον", "~κουσῐ";
    ind_mid_1: "~κομαι", _, "~κομεθᾰ";
    ind_mid_2: "~κῃ", "~κεσθον", _;
    ind_pass_2: _, _, "~κεσθε";
    ind_pass_3: "~κεται", _, "~κονται";
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
    opt_mid_2: "~κοιο", "~κοισθον", _;
    opt_mid_3: "~κοιτο", "~κοισθην", "~κοιντο";
    opt_pass_2: _, _, "~κοισθε";
    imp_act_2: "~κε", "~κετον", "~κετε";
    imp_mid_2: "~κου", _, "~κεσθε";
    imp_mid_3: "~κεσθω", _, "~κεσθων";
    imp_pass_2: _, _, "~κεσθε";
    imp_pass_3: _, "~κεσθων", "~κεσθων";
    imperfect_act_1: "ἐ~κον", _, "ἐ~κομεν";
    imperfect_act_2: "ἐ~κες", "ἐ~κετον", "ἐ~κετε";
    imperfect_act_3: "ἐ~κε", "ἐ~κετην", "ἐ~κον";
    imperfect_mid_1: "ἐ~κομην", _, "ἐ~κομεθᾰ";
    imperfect_mid_2: "ἐ~κου", _, _;
    imperfect_mid_3: "ἐ~κετο", _, _;
    imperfect_pass_2: _, "ἐ~κεσθον", "ἐ~κεσθε";
    imperfect_pass_3: _, "ἐ~κεσθην", "ἐ~κοντο";
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
    // Καθιζω: 8 words (εὐαγγελιζω, βᾰδιζω, παιζω, καθιζω, συνᾱλιζω, βαλλιζω). Like Πεμπω, except in 159 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("καθιζω", 8, "~~ζω", like Πεμπω;
    stem future_pass: "~~σθησ- like ἐχω";
    stem aorist_ind: "~~σ- like ποιεω";
    stem aorist_ind_pass: "~~σθην- like ἐχω";
    stem aorist_pass: "~~σθ- like ἐχω";
    stem pluperfect_act: "~~κειν- like ἐχω";
    ind_act_1: "~~ζω", _, "~~ζομεν";
    ind_act_2: "~~ζεις", _, _;
    ind_act_3: "~~ζει", "~~ζετον", "~~ζουσῐ|~~ζουσῐν";
    ind_mid_1: "~~ζομαι", _, "~~ζομεθᾰ";
    ind_mid_2: "~~ζῃ", "~~ζεσθον", _;
    ind_pass_2: _, _, "~~ζεσθε";
    ind_pass_3: "~~ζεται", _, "~~ζονται";
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
    opt_mid_2: "~~ζοιο", "~~ζοισθον", _;
    opt_mid_3: "~~ζοιτο", "~~ζοισθην", "~~ζοιντο";
    opt_pass_2: _, _, "~~ζοισθε";
    imp_act_2: "~~ζε", "~~ζετον", "~~ζετε";
    imp_mid_2: "~~ζου", _, "~~ζεσθε";
    imp_mid_3: "~~ζεσθω", _, "~~ζεσθων";
    imp_pass_2: _, _, "~~ζεσθε";
    imp_pass_3: _, "~~ζεσθων", "~~ζεσθων";
    imperfect_act_1: "ἐ~~ζον", _, "ἐ~~ζομεν";
    imperfect_act_2: "ἐ~~ζες", "ἐ~~ζετον", "ἐ~~ζετε";
    imperfect_act_3: "ἐ~~ζε|ἐ~~ζεν", "ἐ~~ζετην", "ἐ~~ζον";
    imperfect_mid_1: "ἐ~~ζομην", _, "ἐ~~ζομεθᾰ";
    imperfect_mid_2: "ἐ~~ζου", _, _;
    imperfect_mid_3: "ἐ~~ζετο", _, _;
    imperfect_pass_2: _, "ἐ~~ζεσθον", "ἐ~~ζεσθε";
    imperfect_pass_3: _, "ἐ~~ζεσθην", "ἐ~~ζοντο";
    future_ind_act_1: "~~ω", _, "~~οῡμεν";
    future_ind_act_2: "~~εῑς", "~~εῑτον", "~~εῑτε";
    future_ind_act_3: _, _, "~~οῡσῐ";
    future_ind_mid_1: "~~οῡμαι", _, "~~ουμεθᾰ";
    future_ind_mid_2: "~~εῑ|~~ῃ", "~~εῑσθον", _;
    future_ind_mid_3: "~~εῑται", _, _;
    future_opt_act_1: "~~οιην", _, "~~οῑμεν";
    future_opt_act_2: _, "~~οῑτον", "~~οῑτε";
    future_opt_act_3: "~~οιη", "~~οιτην", "~~οῑεν";
    future_opt_mid_1: "~~οιμην", _, "~~οιμεθᾰ";
    future_opt_mid_2: "~~οῑο", _, _;
    future_opt_mid_3: _, "~~οισθην", _;
    aorist_subj_act_1: "~~σω", _, _;
    aorist_subj_act_2: "~~σῃς", "~~σητον", _;
    aorist_subj_act_3: _, _, "~~σωσῐ|~~σωσῐν";
    aorist_subj_mid_1: "~~σωμαι", _, _;
    aorist_subj_mid_2: "~~σῃ", "~~σησθον", _;
    aorist_opt_act_1: "~~σαιμῐ", _, "~~σαιμεν";
    aorist_opt_act_2: "~~σειᾰς|~~σαις", _, _;
    aorist_opt_act_3: "~~σειε|~~σειεν|~~σαι", "~~σαιτην", "~~σειᾰν|~~σαιεν";
    aorist_opt_mid_1: "~~σαιμην", _, _;
    aorist_opt_mid_2: "~~σαιο", _, _;
    aorist_imp_act_2: "~~σον", _, "~~σᾰτε";
    aorist_imp_act_3: _, _, "~~σᾰντων";
    aorist_imp_mid_2: "~~σαι", "~~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~σᾰσθων";
    perfect_ind_act_1: "%ε~~κᾰ", _, "%ε~~κᾰμεν";
    perfect_ind_act_2: _, "%ε~~κᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~~κᾱσῐ";
    perfect_ind_mid_1: "%ε~~σμαι", _, "%ε~~σμεθᾰ";
    perfect_ind_mid_2: "%ε~~σαι", "%ε~~σθον", _;
    perfect_ind_mid_3: "%ε~~σται", _, "%ε~~σᾰται";
    perfect_ind_pass_2: _, _, "%ε~~σθε";
    perfect_subj_act_1: "%ε~~κω", _, _;
    perfect_subj_act_2: "%ε~~κῃς", "%ε~~κητον", "%ε~~κητε";
    perfect_subj_act_3: "%ε~~κῃ", _, "%ε~~κωσῐ";
    perfect_opt_act_1: "%ε~~κοιμῐ", _, "%ε~~κοιμεν";
    perfect_opt_act_2: "%ε~~κοις", _, _;
    perfect_opt_act_3: _, "%ε~~κοιτην", _;
    perfect_imp_act_2: "%ε~~κε", _, "%ε~~κετε";
    perfect_imp_act_3: _, _, "%ε~~κοντων";
    perfect_imp_mid_2: _, _, "%ε~~σθε";
    perfect_imp_mid_3: "%ε~~σθω", _, "%ε~~σθων";
    perfect_imp_pass_2: "%ε~~σο", _, "%ε~~σθε";
    perfect_imp_pass_3: _, "%ε~~σθων", "%ε~~σθων";
    pluperfect_mid_1: "ἐ%ε~~σμην", _, _;
    pluperfect_mid_2: "ἐ%ε~~σο", "ἐ%ε~~σθον", _;
    pluperfect_mid_3: "ἐ%ε~~στο", _, "ἐ%ε~~σᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~~σθε";
    pluperfect_pass_3: _, "ἐ%ε~~σθην", _;
    infinitive_pres: "~~ζειν", _, "~~ζεσθαι";
    infinitive_fut: "~~εῑν", _, _;
    infinitive_aor: "~ῑσα~", _, _;
    infinitive_perf: _, "%ε~ῑσθα~", _;
    participle_pres: "~~ζων", _, "~~ζομενος";
    participle_aor: "~~σᾱς", "~~σᾰμενος", _;
    participle_perf: "%ε~~κως", _, "%ε~~σμενος";
    ),
    // Φῠω: 8 words (φῠω, ἐνδῠω, κῠω, κωλῠω, προμηνῠω, δῐᾰμηνῠω). Like Χεω, except in 148 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("φῠω", 8, "~ῠω", like Χεω;
    stem future_pass: "~ῡθησ- like ἐχω";
    stem aorist_ind: "~ῡσ- like ἐχω";
    stem aorist_ind_pass: "~ῡθην- like ἐχω";
    stem aorist_pass: "~ῡθ- like ἐχω";
    ind_act_1: "~ῠω", _, "~ῠομεν";
    ind_act_2: "~ῠεις", _, _;
    ind_act_3: "~ῠει", "~ῠετον", "~ῠουσῐ|~ῠουσῐν";
    ind_mid_1: "~ῠομαι", _, "~ῠομεθᾰ";
    ind_mid_2: "~ῠῃ|~ῠει", "~ῠεσθον", _;
    ind_pass_2: _, _, "~ῠεσθε";
    ind_pass_3: "~ῠεται", _, "~ῠονται";
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
    opt_mid_2: "~ῠοιο", "~ῠοισθον", _;
    opt_mid_3: "~ῠοιτο", "~ῠοισθην", "~ῠοιντο";
    opt_pass_2: _, _, "~ῠοισθε";
    imp_act_2: "~ῠε", "~ῠετον", "~ῠετε";
    imp_mid_2: "~ῠου", _, "~ῠεσθε";
    imp_mid_3: "~ῠεσθω", _, "~ῠεσθων";
    imp_pass_2: _, _, "~ῠεσθε";
    imp_pass_3: _, "~ῠεσθων", "~ῠεσθων";
    imperfect_act_1: "ἐ~ῠον", _, "ἐ~ῠομεν";
    imperfect_act_2: "ἐ~ῠες", "ἐ~ῠετον", "ἐ~ῠετε";
    imperfect_act_3: "ἐ~ῠε", "ἐ~ῠετην", "ἐ~ῠον";
    imperfect_mid_1: "ἐ~ῠομην", _, "ἐ~ῠομεθᾰ";
    imperfect_mid_2: "ἐ~ῠου", _, _;
    imperfect_mid_3: "ἐ~ῠετο", _, _;
    imperfect_pass_2: _, "ἐ~ῠεσθον", "ἐ~ῠεσθε";
    imperfect_pass_3: _, "ἐ~ῠεσθην", "ἐ~ῠοντο";
    future_ind_act_1: "~ῡσω", _, "~ῡσομεν";
    future_ind_act_2: "~ῡσεις", "~ῡσετον", "~ῡσετε";
    future_ind_act_3: _, _, "~ῡσουσῐ";
    future_ind_mid_1: "~ῡσομαι", _, "~ῡσομεθᾰ";
    future_ind_mid_2: "~ῡσῃ", "~ῡσεσθον", _;
    future_ind_mid_3: "~ῡσεται", _, _;
    future_opt_act_1: "~ῡσοιμῐ", _, "~ῡσοιμεν";
    future_opt_act_2: _, "~ῡσοιτον", "~ῡσοιτε";
    future_opt_act_3: "~ῡσοι", "~ῡσοιτην", "~ῡσοιεν";
    future_opt_mid_1: "~ῡσοιμην", _, "~ῡσοιμεθᾰ";
    future_opt_mid_2: "~ῡσοιο", _, _;
    future_opt_mid_3: _, "~ῡσοισθην", _;
    aorist_subj_act_1: "~ῡσω", _, _;
    aorist_subj_act_2: "~ῡσῃς", "~ῡσητον", _;
    aorist_subj_mid_1: "~ῡσωμαι", _, _;
    aorist_subj_mid_2: "~ῡσῃ", "~ῡσησθον", _;
    aorist_opt_act_1: "~ῡσαιμῐ", _, "~ῡσαιμεν";
    aorist_opt_act_2: "~ῡσειᾰς|~ῡσαις", _, _;
    aorist_opt_act_3: "~ῡσειε|~ῡσειεν|~ῡσαι", "~ῡσαιτην", "~ῡσειᾰν|~ῡσαιεν";
    aorist_opt_mid_1: "~ῡσαιμην", _, _;
    aorist_opt_mid_2: "~ῡσαιο", _, _;
    aorist_imp_act_2: _, _, "~ῡσᾰτε";
    aorist_imp_act_3: _, _, "~ῡσᾰντων";
    aorist_imp_mid_2: "~ῡσαι", "~ῡσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῡσᾰσθων";
    perfect_ind_act_1: "%ε~ῡκᾰ", _, "%ε~ῡκᾰμεν";
    perfect_ind_act_2: _, "%ε~ῡκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ῡκᾱσῐ";
    perfect_ind_mid_1: "%ε~ῡμαι", _, "%ε~ῡμεθᾰ";
    perfect_ind_mid_2: "%ε~ῡσαι", "%ε~ῡσθον", _;
    perfect_ind_mid_3: "%ε~ῡται", _, "%ε~ῡνται";
    perfect_ind_pass_2: _, _, "%ε~ῡσθε";
    perfect_subj_act_1: "%ε~ῡκω", _, _;
    perfect_subj_act_2: "%ε~ῡκῃς", "%ε~ῡκητον", "%ε~ῡκητε";
    perfect_subj_act_3: "%ε~ῡκῃ", _, "%ε~ῡκωσῐ";
    perfect_opt_act_1: "%ε~ῡκοιμῐ", _, "%ε~ῡκοιμεν";
    perfect_opt_act_2: "%ε~ῡκοις", _, _;
    perfect_opt_act_3: _, "%ε~ῡκοιτην", _;
    perfect_imp_act_2: "%ε~ῡκε", _, "%ε~ῡκετε";
    perfect_imp_act_3: _, _, "%ε~ῡκοντων";
    perfect_imp_mid_2: _, _, "%ε~ῡσθε";
    perfect_imp_mid_3: "%ε~ῡσθω", _, "%ε~ῡσθων";
    perfect_imp_pass_2: "%ε~ῡσο", _, "%ε~ῡσθε";
    perfect_imp_pass_3: _, "%ε~ῡσθων", "%ε~ῡσθων";
    infinitive_pres: "~ῠειν", _, "~ῠεσθαι";
    infinitive_fut: "~ῡσειν", _, _;
    participle_pres: "~ῠων", _, "~ῠομενος";
    participle_aor: "~ῡσᾱς", "~ῡσᾰμενος", _;
    participle_perf: "%ε~ῡκως", _, "%ε~ῡμενος";
    ),
    // Πλᾰσσω: 8 words (πλᾰσσω, πᾰσσω, βρᾰσσω, ἱμᾰσσω, ἐρεσσω, βλῐσσω). Like Ἐκχεω, except in 155 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("πλᾰσσω", 8, "~σω", like Ἐκχεω;
    stem future_pass: "~θησ- like ἐχω";
    stem aorist_ind: "~- like ἐχω";
    stem aorist_ind_pass: "~θην- like ἐχω";
    stem aorist_pass: "~θ- like ἐχω";
    ind_act_1: "~σω", _, "~σομεν";
    ind_act_2: "~σεις", _, _;
    ind_act_3: "~σει", "~σετον", "~σουσῐ";
    ind_mid_1: "~σομαι", _, "~σομεθᾰ";
    ind_mid_2: "~σῃ", "~σεσθον", _;
    ind_pass_2: _, _, "~σεσθε";
    ind_pass_3: "~σεται", _, "~σονται";
    subj_act_1: "~σω", _, "~σωμεν";
    subj_act_2: "~σῃς", _, _;
    subj_act_3: "~σῃ", "~σητον", "~σωσῐ";
    subj_mid_1: _, _, "~σωμεθᾰ";
    subj_mid_2: _, "~σησθον", _;
    subj_mid_3: "~σηται", _, "~σωνται";
    subj_pass_1: "~σωμαι", _, _;
    subj_pass_2: _, _, "~σησθε";
    opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    opt_act_2: "~σοις", "~σοιτον", _;
    opt_act_3: "~σοι", "~σοιτην", "~σοιεν";
    opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    opt_mid_2: "~σοιο", "~σοισθον", _;
    opt_mid_3: "~σοιτο", "~σοισθην", "~σοιντο";
    opt_pass_2: _, _, "~σοισθε";
    imp_act_2: "~σε", "~σετον", "~σετε";
    imp_mid_2: "~σου", _, "~σεσθε";
    imp_mid_3: "~σεσθω", _, "~σεσθων";
    imp_pass_2: _, _, "~σεσθε";
    imp_pass_3: _, "~σεσθων", "~σεσθων";
    imperfect_act_1: "ἐ~σον", _, "ἐ~σομεν";
    imperfect_act_2: "ἐ~σες", "ἐ~σετον", "ἐ~σετε";
    imperfect_act_3: "ἐ~σε", "ἐ~σετην", "ἐ~σον";
    imperfect_mid_1: "ἐ~σομην", _, "ἐ~σομεθᾰ";
    imperfect_mid_2: "ἐ~σου", _, _;
    imperfect_mid_3: "ἐ~σετο", _, _;
    imperfect_pass_2: _, "ἐ~σεσθον", "ἐ~σεσθε";
    imperfect_pass_3: _, "ἐ~σεσθην", "ἐ~σοντο";
    future_ind_act_1: "~ω", _, "~ομεν";
    future_ind_act_2: "~εις", "~ετον", "~ετε";
    future_ind_act_3: _, _, "~ουσῐ";
    future_ind_mid_1: "~ομαι", _, "~ομεθᾰ";
    future_ind_mid_2: "~ῃ", "~εσθον", _;
    future_ind_mid_3: "~εται", _, _;
    future_opt_act_1: "~οιμῐ", _, "~οιμεν";
    future_opt_act_2: _, "~οιτον", "~οιτε";
    future_opt_act_3: "~οι", "~οιτην", "~οιεν";
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_2: "~οιο", _, _;
    future_opt_mid_3: _, "~οισθην", _;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_1: "%ε~μαι", _, "%ε~μεθᾰ";
    perfect_ind_mid_2: "%ε~αι", "%ε~θον", _;
    perfect_ind_mid_3: "%ε~ται", _, "%ε~ᾰται";
    perfect_ind_pass_2: _, _, "%ε~θε";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: _, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, _, "%ε~θε";
    perfect_imp_mid_3: "%ε~θω", _, "%ε~θων";
    perfect_imp_pass_2: "%ε~ο", _, "%ε~θε";
    perfect_imp_pass_3: _, "%ε~θων", "%ε~θων";
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: "ἐ%ε~μην", _, _;
    pluperfect_mid_2: "ἐ%ε~ο", "ἐ%ε~θον", _;
    pluperfect_mid_3: "ἐ%ε~το", _, "ἐ%ε~ᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~μεθᾰ";
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
    infinitive_perf: _, "%ε~θαι", _;
    infinitive_futp: _, _, "%ε~εσθαι";
    participle_pres: "~σων", _, "~σομενος";
    participle_perf: derived, _, "%ε~μενος";
    participle_futp: _, _, "%ε~ομενος";
    ),
    // Πᾰρεχω: 8 words (ἐπεχω, προσεχω, πᾰρεχω, ἀπεχω, περιεχω, ἀνεχω). Like Μῑγνῡμῐ, except in 153 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("πᾰρεχω", 8, "~εχω", like Μῑγνῡμῐ;
    stem aorist_ind: "~εσχ- like πᾰρεχω";
    stem aorist_ind_pass: "~εσχεθην- like ἐλπω";
    stem aorist_pass: "~σχεθ- like ἐχω";
    ind_act_1: "~εχω", _, "~εχομεν";
    ind_act_2: "~εχεις", _, _;
    ind_act_3: "~εχει", "~εχετον", "~εχουσῐ|~εχουσῐν";
    ind_mid_1: "~εχομαι", _, "~εχομεθᾰ";
    ind_mid_2: "~εχῃ|~εχει", "~εχεσθον", _;
    ind_pass_2: _, _, "~εχεσθε";
    ind_pass_3: "~εχεται", _, "~εχονται";
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
    opt_mid_2: "~εχοιο", "~εχοισθον", _;
    opt_mid_3: "~εχοιτο", "~εχοισθην", "~εχοιντο";
    opt_pass_2: _, _, "~εχοισθε";
    imp_act_2: "~εχε", "~εχετον", "~εχετε";
    imp_mid_2: "~εχου", _, "~εχεσθε";
    imp_mid_3: "~εχεσθω", _, "~εχεσθων";
    imp_pass_2: _, _, "~εχεσθε";
    imp_pass_3: _, "~εχεσθων", "~εχεσθων";
    imperfect_act_1: "~ειχον", _, "~ειχομεν";
    imperfect_act_2: "~ειχες", "~ειχετον", "~ειχετε";
    imperfect_act_3: "~ειχε", "~ειχετην", "~ειχον";
    imperfect_mid_1: "~ειχομην", _, "~ειχομεθᾰ";
    imperfect_mid_2: "~ειχου", _, _;
    imperfect_mid_3: "~ειχετο", _, _;
    imperfect_pass_2: _, "~ειχεσθον", "~ειχεσθε";
    imperfect_pass_3: _, "~ειχεσθην", "~ειχοντο";
    future_ind_act_1: "~εξω", _, "~εξομεν";
    future_ind_act_2: "~εξεις", "~εξετον", "~εξετε";
    future_ind_act_3: _, _, "~εξουσῐ";
    future_ind_mid_1: "~εξομαι", _, "~εξομεθᾰ";
    future_ind_mid_2: "~εξῃ", "~εξεσθον", _;
    future_ind_mid_3: "~εξεται", _, _;
    future_opt_act_1: "~εξοιμῐ", _, "~εξοιμεν";
    future_opt_act_2: _, "~εξοιτον", "~εξοιτε";
    future_opt_act_3: "~εξοι", "~εξοιτην", "~εξοιεν";
    future_opt_mid_1: "~εξοιμην", _, "~εξοιμεθᾰ";
    future_opt_mid_2: "~εξοιο", _, _;
    future_opt_mid_3: _, "~εξοισθην", _;
    aorist_subj_act_1: "~σχω", _, _;
    aorist_subj_act_2: "~σχῃς", "~σχητον", _;
    aorist_subj_act_3: _, _, "~σχωσῐ|~σχωσῐν";
    aorist_subj_mid_1: "~σχωμαι", _, _;
    aorist_subj_mid_2: "~σχῃ", "~σχησθον", _;
    aorist_opt_act_1: "~σχοιμῐ", _, "~σχοιμεν";
    aorist_opt_act_2: "~σχοις", _, _;
    aorist_opt_act_3: "~σχοι", "~σχοιτην", "~σχοιεν";
    aorist_opt_mid_1: "~σχοιμην", _, _;
    aorist_opt_mid_2: "~σχοιο", _, _;
    aorist_imp_act_2: "~σχε", _, "~σχετε";
    aorist_imp_act_3: _, _, "~σχοντων";
    aorist_imp_mid_2: "~σχοῡ", "~σχεσθον", _;
    aorist_imp_mid_3: _, _, "~σχεσθων";
    perfect_ind_act_1: "~εσχηκᾰ", _, "~εσχηκᾰμεν";
    perfect_ind_act_2: _, "~εσχηκᾰτον", _;
    perfect_ind_act_3: "~εσχηκε|~εσχηκεν", _, "~εσχηκᾱσῐ|~εσχηκᾱσῐν";
    perfect_ind_mid_1: "~εσχημαι", _, "~εσχημεθᾰ";
    perfect_ind_mid_2: "~εσχησαι", "~εσχησθον", _;
    perfect_ind_mid_3: "~εσχηται", _, "~εσχηνται";
    perfect_ind_pass_2: _, _, "~εσχησθε";
    perfect_subj_act_1: "~εσχηκω", _, _;
    perfect_subj_act_2: "~εσχηκῃς", "~εσχηκητον", "~εσχηκητε";
    perfect_subj_act_3: "~εσχηκῃ", _, "~εσχηκωσῐ|~εσχηκωσῐν";
    perfect_opt_act_1: "~εσχηκοιμῐ|~εσχηκοιην", _, "~εσχηκοιμεν";
    perfect_opt_act_2: "~εσχηκοις|~εσχηκοιης", _, _;
    perfect_opt_act_3: "~εσχηκοι|~εσχηκοιη", "~εσχηκοιτην", _;
    perfect_imp_act_2: "~εσχηκε", _, "~εσχηκετε";
    perfect_imp_act_3: _, _, "~εσχηκοντων";
    perfect_imp_mid_2: _, _, "~εσχησθε";
    perfect_imp_mid_3: "~εσχησθω", _, "~εσχησθων";
    perfect_imp_pass_2: "~εσχησο", _, "~εσχησθε";
    perfect_imp_pass_3: _, "~εσχησθων", "~εσχησθων";
    infinitive_pres: "~εχειν", _, "~εχεσθαι";
    infinitive_fut: "~εξειν", _, _;
    infinitive_aor: "~σχεῑν", _, _;
    infinitive_perf: _, "~εσχησθαι", _;
    participle_pres: "~εχων", _, "~εχομενος";
    participle_aor: "~σχων", "~σχομενος", _;
    participle_perf: "~εσχηκως", _, "~εσχημενος";
    ),
    // Κρῑνομαι: 8 words (γεινομαι, ὀλοφῡρομαι, ἐρομαι, ὀδῡρομαι, σῑνομαι, ἐπερομαι). Like Οἰομαι, except in 53 cells: subj_mid_1 sg, subj_pass_1 sg, imperfect_mid_2 du, imperfect_mid_3 du, imperfect_mid_3 pl, imperfect_pass_2 du, ….
    conjugation!("κρῑνομαι", 8, "~ομαι", like Οἰομαι;
    stem aorist_ind: "~- like ἐχω";
    subj_mid_1: "~ωμαι", _, _;
    subj_pass_1: derived, _, _;
    imperfect_mid_2: _, "ἐ~εσθον", _;
    imperfect_mid_3: _, "ἐ~εσθην", "ἐ~οντο";
    imperfect_pass_2: _, derived, _;
    imperfect_pass_3: _, derived, derived;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ω", _, _;
    aorist_subj_act_2: "~ῃς", "~ητον", _;
    aorist_subj_mid_1: "~ωμαι", _, _;
    aorist_subj_mid_2: "~ῃ", "~ησθον", _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_1: "~αιμῐ", _, "~αιμεν";
    aorist_opt_act_2: "~ειᾰς", _, _;
    aorist_opt_act_3: "~ειε", "~αιτην", "~ειᾰν";
    aorist_opt_mid_1: "~αιμην", _, _;
    aorist_opt_mid_2: "~αιο", _, _;
    aorist_imp_act_2: "~ον", _, "~ᾰτε";
    aorist_imp_act_3: _, _, "~ᾰντων";
    aorist_imp_mid_2: "~αι", "~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰσθων";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: derived, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: _, derived, derived;
    infinitive_aor: "~αι", _, _;
    infinitive_perf: _, derived, _;
    participle_aor: "~ᾱς", "~ᾰμενος", _;
    participle_perf: _, _, derived;
    ),
    // Πῠνθανομαι: 8 words (μαχομαι, πῠνθανομαι, σῠνεργαζομαι, κᾰτεργαζομαι, ἀπεχθανομαι, ἐξεργαζομαι). Like Δερκομαι, except in 85 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 sg, ….
    conjugation!("πῠνθανομαι", 8, "~α~ομαι", like Δερκομαι;
    stem aorist_ind: "~ᾰ~εσ- like πᾰραιτεομαι";
    ind_act_1: "~α~ομαι", _, _;
    ind_mid_1: "~ᾰ~ομαι", _, "~ᾰ~ομεθᾰ";
    ind_mid_2: "~ᾰ~ῃ|~ᾰ~ει", "~ᾰ~εσθον", _;
    ind_pass_2: "~ᾰ~ῃ", _, "~ᾰ~εσθε";
    ind_pass_3: "~ᾰ~εται", _, "~ᾰ~ονται";
    subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    subj_mid_2: "~ᾰ~ῃ", "~ᾰ~ησθον", _;
    subj_mid_3: "~ᾰ~ηται", _, "~ᾰ~ωνται";
    subj_pass_2: "~ᾰ~ῃ", _, "~ᾰ~ησθε";
    opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~οιο", "~ᾰ~οισθον", _;
    opt_mid_3: "~ᾰ~οιτο", "~ᾰ~οισθην", "~ᾰ~οιντο";
    opt_pass_2: _, _, "~ᾰ~οισθε";
    imp_mid_2: "~ᾰ~ου", _, "~ᾰ~εσθε";
    imp_mid_3: "~ᾰ~εσθω", _, "~ᾰ~εσθων";
    imp_pass_2: _, _, "~ᾰ~εσθε";
    imp_pass_3: _, "~ᾰ~εσθων", "~ᾰ~εσθων";
    imperfect_mid_1: "ἐ~ᾰ~ομην", _, "ἐ~ᾰ~ομεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~ου", "ἐ~ᾰ~εσθον", _;
    imperfect_mid_3: "ἐ~ᾰ~ετο", "ἐ~ᾰ~εσθην", "ἐ~ᾰ~οντο";
    imperfect_pass_2: _, _, "ἐ~ᾰ~εσθε";
    future_ind_mid_1: "~ᾰ~εσομαι", _, "~ᾰ~εσομεθᾰ";
    future_ind_mid_2: "~ᾰ~εσῃ", "~ᾰ~εσεσθον", _;
    future_ind_mid_3: "~ᾰ~εσεται", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_mid_1: "~ᾰ~εσοιμην", _, "~ᾰ~εσοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~εσοιο", _, _;
    future_opt_mid_3: _, "~ᾰ~εσοισθην", _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_mid_1: "~ᾰ~εσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~εσῃ", "~ᾰ~εσησθον", _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_mid_1: "~ᾰ~εσαιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~εσαιο", _, _;
    aorist_imp_mid_2: "~ᾰ~εσαι", "~ᾰ~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~εσᾰσθων";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_mid_1: "~εμᾰ~ημαι", _, "~εμᾰ~ημεθᾰ";
    perfect_ind_mid_2: "~εμᾰ~ησαι", "~εμᾰ~ησθον", _;
    perfect_ind_mid_3: "~εμᾰ~ηται", _, "~εμᾰ~ηνται";
    perfect_ind_pass_2: _, _, "~εμᾰ~ησθε";
    perfect_imp_mid_2: _, _, "~εμᾰ~ησθε";
    perfect_imp_mid_3: "~εμᾰ~ησθω", _, "~εμᾰ~ησθων";
    perfect_imp_pass_2: "~εμᾰ~ησο", _, "~εμᾰ~ησθε";
    perfect_imp_pass_3: _, "~εμᾰ~ησθων", "~εμᾰ~ησθων";
    infinitive_pres: _, _, "~ᾰ~εσθαι";
    infinitive_perf: _, "~εμᾰ~ησθαι", _;
    participle_pres: _, derived, "~ᾰ~ομενος";
    participle_aor: _, "~ᾰ~εσᾰμενος", _;
    participle_perf: _, _, "~εμᾰ~ημενος";
    ),
    // Σῳζω: 7 words (σοφῐζω, σῳζω, κεᾰζω, στρουθῐζω, τῠκῐζω, μερῐζω). Like Χρημᾰτῐζω, except in 2 cells: perfect_ind_mid_3 pl, pluperfect_mid_3 pl.
    conjugation!("σῳζω", 7, "~ζω", like Χρημᾰτῐζω;
    perfect_ind_mid_3: _, _, "%ε~σᾰται";
    pluperfect_mid_3: _, _, "ἐ%ε~σᾰτο";
    ),
    // Σφᾰλλω: 7 words (θᾰλλω, πᾰλλω, βδᾰλλω, σφᾰλλω, ψᾰλλω, ἰᾰλλω). Like Ξαινω, except in 116 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("σφᾰλλω", 7, "~ᾰ~λω", like Ξαινω;
    stem future_pass: "~ᾰ~ησησ- like ἐχω";
    stem aorist_ind: "~η~- like ποιεω";
    stem aorist_ind_pass: "~ᾰ~ην- like ἐχω";
    stem aorist_pass: "~ᾰ~- like σφᾰλλω";
    stem pluperfect_act: "~ᾰ~κειν- like κᾰλεω";
    ind_act_1: "~ᾰ~λω", _, "~ᾰ~λομεν";
    ind_act_2: "~ᾰ~λεις", _, _;
    ind_act_3: "~ᾰ~λει", "~ᾰ~λετον", "~ᾰ~λουσῐ|~ᾰ~λουσῐν";
    ind_mid_1: "~ᾰ~λομαι", _, "~ᾰ~λομεθᾰ";
    ind_mid_2: "~ᾰ~λῃ|~ᾰ~λει", "~ᾰ~λεσθον", _;
    ind_pass_2: _, _, "~ᾰ~λεσθε";
    ind_pass_3: "~ᾰ~λεται", _, "~ᾰ~λονται";
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
    opt_mid_2: "~ᾰ~λοιο", "~ᾰ~λοισθον", _;
    opt_mid_3: "~ᾰ~λοιτο", "~ᾰ~λοισθην", "~ᾰ~λοιντο";
    opt_pass_2: _, _, "~ᾰ~λοισθε";
    imp_act_2: "~ᾰ~λε", "~ᾰ~λετον", "~ᾰ~λετε";
    imp_mid_2: "~ᾰ~λου", _, "~ᾰ~λεσθε";
    imp_mid_3: "~ᾰ~λεσθω", _, "~ᾰ~λεσθων";
    imp_pass_2: _, _, "~ᾰ~λεσθε";
    imp_pass_3: _, "~ᾰ~λεσθων", "~ᾰ~λεσθων";
    imperfect_act_1: "ἐ~ᾰ~λον", _, "ἐ~ᾰ~λομεν";
    imperfect_act_2: "ἐ~ᾰ~λες", "ἐ~ᾰ~λετον", "ἐ~ᾰ~λετε";
    imperfect_act_3: "ἐ~ᾰ~λε|ἐ~ᾰ~λεν", "ἐ~ᾰ~λετην", "ἐ~ᾰ~λον";
    imperfect_mid_1: "ἐ~ᾰ~λομην", _, "ἐ~ᾰ~λομεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~λου", _, _;
    imperfect_mid_3: "ἐ~ᾰ~λετο", _, _;
    imperfect_pass_2: _, "ἐ~ᾰ~λεσθον", "ἐ~ᾰ~λεσθε";
    imperfect_pass_3: _, "ἐ~ᾰ~λεσθην", "ἐ~ᾰ~λοντο";
    aorist_subj_act_3: _, _, "~η~ωσῐ|~η~ωσῐν";
    aorist_opt_act_2: "~η~ειᾰς|~η~αις", _, _;
    aorist_opt_act_3: "~η~ειε|~η~ειεν|~η~αι", _, "~η~ειᾰν|~η~αιεν";
    perfect_ind_act_1: "ἐ~ᾰ~κᾰ", _, "ἐ~ᾰ~κᾰμεν";
    perfect_ind_act_2: _, "ἐ~ᾰ~κᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~ᾰ~κᾱσῐ";
    perfect_ind_mid_1: "ἐ~ᾰ~μαι", _, "ἐ~ᾰ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰ~σαι", "ἐ~ᾰ~θον", _;
    perfect_ind_mid_3: "ἐ~ᾰ~ται", _, "ἐ~ᾰ~ᾰται";
    perfect_ind_pass_2: _, _, "ἐ~ᾰ~θε";
    perfect_subj_act_1: "ἐ~ᾰ~κω", _, _;
    perfect_subj_act_2: "ἐ~ᾰ~κῃς", "ἐ~ᾰ~κητον", "ἐ~ᾰ~κητε";
    perfect_subj_act_3: "ἐ~ᾰ~κῃ", _, "ἐ~ᾰ~κωσῐ";
    perfect_opt_act_1: "ἐ~ᾰ~κοιμῐ", _, "ἐ~ᾰ~κοιμεν";
    perfect_opt_act_2: "ἐ~ᾰ~κοις", _, _;
    perfect_opt_act_3: _, "ἐ~ᾰ~κοιτην", _;
    perfect_imp_act_2: "ἐ~ᾰ~κε", _, "ἐ~ᾰ~κετε";
    perfect_imp_act_3: _, _, "ἐ~ᾰ~κοντων";
    perfect_imp_mid_2: _, _, "ἐ~ᾰ~θε";
    perfect_imp_mid_3: "ἐ~ᾰ~θω", _, "ἐ~ᾰ~θων";
    perfect_imp_pass_2: "ἐ~ᾰ~σο", _, "ἐ~ᾰ~θε";
    perfect_imp_pass_3: _, "ἐ~ᾰ~θων", "ἐ~ᾰ~θων";
    pluperfect_mid_1: "ἐ~ᾰ~μην", _, _;
    pluperfect_mid_2: "ἐ~ᾰ~σο", "ἐ~ᾰ~θον", _;
    pluperfect_mid_3: "ἐ~ᾰ~το", _, "ἐ~ᾰ~ᾰτο";
    pluperfect_pass_1: _, _, "ἐ~ᾰ~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ᾰ~θε";
    pluperfect_pass_3: _, "ἐ~ᾰ~θην", _;
    infinitive_pres: "~ᾰ~λειν", _, "~ᾰ~λεσθαι";
    infinitive_perf: _, "ἐ~ᾰ~θαι", _;
    participle_pres: "~ᾰ~λων", _, "~ᾰ~λομενος";
    participle_perf: "ἐ~ᾰ~κως", _, "ἐ~ᾰ~μενος";
    ),
    // Ἀπεῑπον: 7 words (προσεῑπον, ἐξεῑπον, προεῑπον, ἀπεῑπον, ὑπεῑπον, ἐπεῑπον). Like Μῑγνῡμῐ, except in 84 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("ἀπεῑπον", 7, "~εῑπον", like Μῑγνῡμῐ;
    stem future_pass: "~ορρηθησ- like ἐχω";
    stem aorist_ind: "~ε- like ἀπεῑπον";
    stem aorist_ind_pass: "~ερρηθην- like ἐλπω";
    stem aorist_pass: "~ορρηθ- like ἐχω";
    ind_act_1: "~εῑπον", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_1: "~ερω", _, "~εροῡμεν";
    future_ind_act_2: "~ερεῑς", "~ερεῑτον", "~ερεῑτε";
    future_ind_act_3: _, _, "~εροῡσῐ";
    future_ind_mid_1: "~εροῡμαι", _, "~ερουμεθᾰ";
    future_ind_mid_2: "~ερεῑ", "~ερεῑσθον", _;
    future_ind_mid_3: "~ερεῑται", _, _;
    future_opt_act_1: "~εροιην", _, "~εροῑμεν";
    future_opt_act_2: _, "~εροῑτον", "~εροῑτε";
    future_opt_act_3: "~εροιη", "~εροιτην", "~εροῑεν";
    future_opt_mid_1: "~εροιμην", _, "~εροιμεθᾰ";
    future_opt_mid_2: "~εροῑο", _, _;
    future_opt_mid_3: _, "~εροισθην", _;
    aorist_subj_act_1: "~ειπω", _, _;
    aorist_subj_act_2: "~ειπῃς", "~ειπητον", _;
    aorist_subj_mid_1: "~ειπωμαι", _, _;
    aorist_subj_mid_2: "~ειπῃ", "~ειπησθον", _;
    aorist_opt_act_1: "~ειποιμῐ", _, "~ειποιμεν";
    aorist_opt_act_2: "~ειποις", _, _;
    aorist_opt_act_3: "~ειποι", "~ειποιτην", "~ειποιεν";
    aorist_opt_mid_1: "~ειποιμην", _, _;
    aorist_opt_mid_2: "~ειποιο", _, _;
    aorist_imp_act_2: "~ειπε", _, "~ειπετε";
    aorist_imp_act_3: _, _, "~ειποντων";
    aorist_imp_mid_2: "~ειποῡ", "~ειπεσθον", _;
    aorist_imp_mid_3: _, _, "~ειπεσθων";
    perfect_ind_act_1: "~ειρηκᾰ", _, "~ειρηκᾰμεν";
    perfect_ind_act_2: _, "~ειρηκᾰτον", _;
    perfect_ind_act_3: _, _, "~ειρηκᾱσῐ";
    perfect_ind_mid_1: "~ειρημαι", _, "~ειρημεθᾰ";
    perfect_ind_mid_2: "~ειρησαι", "~ειρησθον", _;
    perfect_ind_mid_3: "~ειρηται", _, "~ειρηνται";
    perfect_ind_pass_2: _, _, "~ειρησθε";
    perfect_subj_act_1: "~ειρηκω", _, _;
    perfect_subj_act_2: "~ειρηκῃς", "~ειρηκητον", "~ειρηκητε";
    perfect_subj_act_3: "~ειρηκῃ", _, "~ειρηκωσῐ";
    perfect_opt_act_1: "~ειρηκοιμῐ", _, "~ειρηκοιμεν";
    perfect_opt_act_2: "~ειρηκοις", _, _;
    perfect_opt_act_3: _, "~ειρηκοιτην", _;
    perfect_imp_act_2: "~ειρηκε", _, "~ειρηκετε";
    perfect_imp_act_3: _, _, "~ειρηκοντων";
    perfect_imp_mid_2: _, _, "~ειρησθε";
    perfect_imp_mid_3: "~ειρησθω", _, "~ειρησθων";
    perfect_imp_pass_2: "~ειρησο", _, "~ειρησθε";
    perfect_imp_pass_3: _, "~ειρησθων", "~ειρησθων";
    infinitive_fut: "~ερεῑν", _, _;
    infinitive_aor: "~ειπεῑν", _, _;
    infinitive_perf: _, "~ειρησθαι", _;
    participle_aor: "~ειπων", "~ειπομενος", _;
    participle_perf: "~ειρηκως", _, "~ειρημενος";
    ),
    // Κᾰθιδρῡω: 7 words (ἑλκω, ἱκω, κᾰθιδρῡω, εἰκω, ἐρῡκω, ἐφηκω). Like Ἐλπω, except in 40 cells: ind_mid_2 sg, future_ind_act_3 pl, future_ind_mid_2 sg, future_ind_pass_1 sg, future_ind_pass_3 du, aorist_ind_pass_1 sg, ….
    conjugation!("κᾰθιδρῡω", 7, "~ω", like Ἐλπω;
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
    perfect_ind_mid_1: "~μαι", _, "~μεθᾰ";
    perfect_ind_mid_3: "~ται", _, "~νται";
    perfect_subj_act_1: "~κω", _, _;
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ|~κωσῐν";
    perfect_opt_act_1: "~κοιμῐ|~κοιην", _, "~κοιμεν";
    perfect_opt_act_2: "~κοις|~κοιης", _, _;
    perfect_opt_act_3: "~κοι|~κοιη", "~κοιτην", _;
    perfect_imp_act_2: "~κε", _, "~κετε";
    perfect_imp_act_3: _, _, "~κοντων";
    participle_perf: "~κως", _, "~μενος";
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
    // Λλῡω: 7 words (λῡω, δῡω, καμμῡω, καταμῡω, κᾰσσῡω, περῐλῡω). Like Φῠω, except in 137 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("λλῡω", 7, "~ῡω", like Φῠω;
    stem future_pass: "~ῠθησ- like ποιεω";
    stem aorist_ind: "~ῡσ- like ποιεω";
    stem aorist_ind_pass: "~ῠθην- like ἐχω";
    stem aorist_pass: "~ῠθ- like ποιεω";
    stem pluperfect_act: "~ῠκειν- like ἐχω";
    ind_act_1: "~ῡω", _, "~ῡομεν";
    ind_act_2: "~ῡεις", _, _;
    ind_act_3: "~ῡει", "~ῡετον", "~ῡουσῐ|~ῡουσῐν";
    ind_mid_1: "~ῡομαι", _, "~ῡομεθᾰ";
    ind_mid_2: "~ῡῃ|~ῡει", "~ῡεσθον", _;
    ind_pass_2: _, _, "~ῡεσθε";
    ind_pass_3: "~ῡεται", _, "~ῡονται";
    subj_act_1: "~ῡω", _, "~ῡωμεν";
    subj_act_2: "~ῡῃς", _, _;
    subj_act_3: "~ῡῃ", "~ῡητον", "~ῡωσῐ|~ῡωσῐν";
    subj_mid_1: derived, _, "~ῡωμεθᾰ";
    subj_mid_2: _, "~ῡησθον", _;
    subj_mid_3: "~ῡηται", _, "~ῡωνται";
    subj_pass_1: "~ῡωμαι", _, _;
    subj_pass_2: _, _, "~ῡησθε";
    opt_act_1: "~ῡοιμῐ", _, "~ῡοιμεν";
    opt_act_2: "~ῡοις", "~ῡοιτον", _;
    opt_act_3: "~ῡοι", "~ῡοιτην", "~ῡοιεν";
    opt_mid_1: "~ῡοιμην", _, "~ῡοιμεθᾰ";
    opt_mid_2: "~ῡοιο", "~ῡοισθον", _;
    opt_mid_3: "~ῡοιτο", "~ῡοισθην", "~ῡοιντο";
    opt_pass_2: _, _, "~ῡοισθε";
    imp_act_2: "~ῡε", "~ῡετον", "~ῡετε";
    imp_mid_2: "~ῡου", _, "~ῡεσθε";
    imp_mid_3: "~ῡεσθω", _, "~ῡεσθων";
    imp_pass_2: _, _, "~ῡεσθε";
    imp_pass_3: _, "~ῡεσθων", "~ῡεσθων";
    imperfect_act_1: "ἐ~ῡον", _, "ἐ~ῡομεν";
    imperfect_act_2: "ἐ~ῡες", "ἐ~ῡετον", "ἐ~ῡετε";
    imperfect_act_3: "ἐ~ῡε|ἐ~ῡεν", "ἐ~ῡετην", "ἐ~ῡον";
    imperfect_mid_1: "ἐ~ῡομην", _, "ἐ~ῡομεθᾰ";
    imperfect_mid_2: "ἐ~ῡου", _, _;
    imperfect_mid_3: "ἐ~ῡετο", _, _;
    imperfect_pass_2: _, "ἐ~ῡεσθον", "ἐ~ῡεσθε";
    imperfect_pass_3: _, "ἐ~ῡεσθην", "ἐ~ῡοντο";
    future_ind_act_3: _, _, "~ῡσουσῐ|~ῡσουσῐν";
    future_ind_mid_2: "~ῡσῃ|~ῡσει", _, _;
    aorist_subj_act_3: _, _, "~ῡσωσῐ|~ῡσωσῐν";
    perfect_ind_act_1: "%ε~ῠκᾰ", _, "%ε~ῠκᾰμεν";
    perfect_ind_act_2: _, "%ε~ῠκᾰτον", _;
    perfect_ind_act_3: "%ε~ῠκε|%ε~ῠκεν", _, "%ε~ῠκᾱσῐ|%ε~ῠκᾱσῐν";
    perfect_ind_mid_1: "%ε~ῠμαι", _, "%ε~ῠμεθᾰ";
    perfect_ind_mid_2: "%ε~ῠσαι", "%ε~ῠσθον", _;
    perfect_ind_mid_3: "%ε~ῠται", _, "%ε~ῠνται";
    perfect_ind_pass_2: _, _, "%ε~ῠσθε";
    perfect_subj_act_1: "%ε~ῠκω", _, _;
    perfect_subj_act_2: "%ε~ῠκῃς", "%ε~ῠκητον", "%ε~ῠκητε";
    perfect_subj_act_3: "%ε~ῠκῃ", _, "%ε~ῠκωσῐ|%ε~ῠκωσῐν";
    perfect_opt_act_1: "%ε~ῠκοιμῐ|%ε~ῠκοιην", _, "%ε~ῠκοιμεν";
    perfect_opt_act_2: "%ε~ῠκοις|%ε~ῠκοιης", _, _;
    perfect_opt_act_3: "%ε~ῠκοι|%ε~ῠκοιη", "%ε~ῠκοιτην", _;
    perfect_imp_act_2: "%ε~ῠκε", _, "%ε~ῠκετε";
    perfect_imp_act_3: _, _, "%ε~ῠκοντων";
    perfect_imp_mid_2: _, _, "%ε~ῠσθε";
    perfect_imp_mid_3: "%ε~ῠσθω", _, "%ε~ῠσθων";
    perfect_imp_pass_2: "%ε~ῠσο", _, "%ε~ῠσθε";
    perfect_imp_pass_3: _, "%ε~ῠσθων", "%ε~ῠσθων";
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
    infinitive_pres: "~ῡειν", _, "~ῡεσθαι";
    infinitive_perf: _, "%ε~ῠσθαι", _;
    infinitive_futp: _, _, "%ε~ῡσεσθαι";
    participle_pres: "~ῡων", _, "~ῡομενος";
    participle_perf: "%ε~ῠκως", _, "%ε~ῠμενος";
    participle_futp: _, _, "%ε~ῡσομενος";
    ),
    // Θλαω: 7 words (ἀγαλλιαω, κρεμαω, θλαω, τηταω, ἐλᾰω, νεᾰω). Like Νῑκαω, except in 89 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 pl, ….
    conjugation!("θλαω", 7, "~αω", like Νῑκαω;
    stem future_pass: "~ασθησ- like ἐχω";
    stem aorist_ind: "~ασ- like ἐχω";
    stem aorist_ind_pass: "~ασθην- like ἐχω";
    stem aorist_pass: "~ασθ- like ἐχω";
    stem pluperfect_act: "~ακειν- like ἐχω";
    future_ind_act_1: "~ασω", _, "~ασομεν";
    future_ind_act_2: "~ασεις", "~ασετον", "~ασετε";
    future_ind_act_3: _, _, "~ασουσῐ";
    future_ind_mid_1: "~ασομαι", _, "~ασομεθᾰ";
    future_ind_mid_2: "~ασῃ", "~ασεσθον", _;
    future_ind_mid_3: "~ασεται", _, _;
    future_opt_act_1: "~ασοιμῐ", _, "~ασοιμεν";
    future_opt_act_2: _, "~ασοιτον", "~ασοιτε";
    future_opt_act_3: "~ασοι", "~ασοιτην", "~ασοιεν";
    future_opt_mid_1: "~ασοιμην", _, "~ασοιμεθᾰ";
    future_opt_mid_2: "~ασοιο", _, _;
    future_opt_mid_3: _, "~ασοισθην", _;
    aorist_subj_act_1: "~ασω", _, _;
    aorist_subj_act_2: "~ασῃς", "~ασητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~ασωμαι", _, _;
    aorist_subj_mid_2: "~ασῃ", "~ασησθον", _;
    aorist_opt_act_1: "~ασαιμῐ", _, "~ασαιμεν";
    aorist_opt_act_2: "~ασειᾰς", _, _;
    aorist_opt_act_3: "~ασειε", "~ασαιτην", "~ασειᾰν";
    aorist_opt_mid_1: "~ασαιμην", _, _;
    aorist_opt_mid_2: "~ασαιο", _, _;
    aorist_imp_act_2: "~ᾱσον", _, "~ασᾰτε";
    aorist_imp_act_3: _, _, "~ασᾰντων";
    aorist_imp_mid_2: "~ᾱσαι", "~ασᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ασᾰσθων";
    perfect_ind_act_1: "%ε~ακᾰ", _, "%ε~ακᾰμεν";
    perfect_ind_act_2: _, "%ε~ακᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ακᾱσῐ";
    perfect_ind_mid_1: "%ε~αμαι", _, "%ε~αμεθᾰ";
    perfect_ind_mid_2: "%ε~ασαι", "%ε~ασθον", _;
    perfect_ind_mid_3: "%ε~αται", _, "%ε~ανται";
    perfect_ind_pass_2: _, _, "%ε~ασθε";
    perfect_subj_act_1: "%ε~ακω", _, _;
    perfect_subj_act_2: "%ε~ακῃς", "%ε~ακητον", "%ε~ακητε";
    perfect_subj_act_3: "%ε~ακῃ", _, "%ε~ακωσῐ";
    perfect_opt_act_1: "%ε~ακοιμῐ", _, "%ε~ακοιμεν";
    perfect_opt_act_2: "%ε~ακοις", _, _;
    perfect_opt_act_3: _, "%ε~ακοιτην", _;
    perfect_imp_act_2: "%ε~ακε", _, "%ε~ακετε";
    perfect_imp_act_3: _, _, "%ε~ακοντων";
    perfect_imp_mid_2: _, _, "%ε~ασθε";
    perfect_imp_mid_3: "%ε~ασθω", _, "%ε~ασθων";
    perfect_imp_pass_2: "%ε~ασο", _, "%ε~ασθε";
    perfect_imp_pass_3: _, "%ε~ασθων", "%ε~ασθων";
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
    // Παυω: 6 words (γευω, παυω, κρουω, χναυω, χραυω, θραυω). Like Βουλευω, except in 10 cells: imp_act_2 sg, future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, ….
    conjugation!("παυω", 6, "~υω", like Βουλευω;
    stem future_pass: "~υσθησ- like ἐχω";
    stem aorist_ind_pass: "~υσθην- like ἐχω";
    stem aorist_pass: "~υσθ- like ἐχω";
    imp_act_2: "~ῡε", _, _;
    future_ind_pass_2: derived, _, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: "~ῡσον", _, _;
    aorist_imp_mid_2: "~ῡσαι", _, _;
    ),
    // Τελεω: 6 words (ἐμεω, ἀρκεω, τελεω, βδεω, νεικεω, δῐᾰρκεω). Like Πᾰραιτεομαι, except in 90 cells: ind_mid_2 sg, ind_pass_2 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, ….
    conjugation!("τελεω", 6, "~εω", like Πᾰραιτεομαι;
    stem future_pass: "~εσθησ- like ἐχω";
    stem aorist_ind: "~εσ- like ἐχω";
    stem aorist_ind_pass: "~εσθην- like ἐχω";
    stem aorist_pass: "~εσθ- like ἐχω";
    stem pluperfect_act: "~εκειν- like ἐχω";
    ind_mid_2: "~εῑ", _, _;
    ind_pass_2: derived, _, _;
    future_ind_act_1: "~ω", _, "~οῡμεν";
    future_ind_act_2: "~εῑς", "~εῑτον", "~εῑτε";
    future_ind_act_3: _, _, "~οῡσῐ";
    future_ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    future_ind_mid_2: "~εῑ", "~εῑσθον", _;
    future_ind_mid_3: "~εῑται", _, _;
    future_opt_act_1: "~οιην", _, "~οῑμεν";
    future_opt_act_2: _, "~οῑτον", "~οῑτε";
    future_opt_act_3: "~οιη", "~οιτην", "~οῑεν";
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_2: "~οῑο", _, _;
    future_opt_mid_3: _, "~οισθην", _;
    aorist_subj_act_1: "~εσω", _, _;
    aorist_subj_act_2: "~εσῃς", "~εσητον", _;
    aorist_subj_mid_1: "~εσωμαι", _, _;
    aorist_subj_mid_2: "~εσῃ", "~εσησθον", _;
    aorist_opt_act_1: "~εσαιμῐ", _, "~εσαιμεν";
    aorist_opt_act_2: "~εσειᾰς", _, _;
    aorist_opt_act_3: "~εσειε", "~εσαιτην", "~εσειᾰν";
    aorist_opt_mid_1: "~εσαιμην", _, _;
    aorist_opt_mid_2: "~εσαιο", _, _;
    aorist_imp_act_2: "~εσον", _, "~εσᾰτε";
    aorist_imp_act_3: _, _, "~εσᾰντων";
    aorist_imp_mid_2: "~εσαι", "~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~εσᾰσθων";
    perfect_ind_act_1: "%ε~εκᾰ", _, "%ε~εκᾰμεν";
    perfect_ind_act_2: _, "%ε~εκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~εκᾱσῐ";
    perfect_ind_mid_1: "%ε~εσμαι", _, "%ε~εσμεθᾰ";
    perfect_ind_mid_2: "%ε~εσαι", "%ε~εσθον", _;
    perfect_ind_mid_3: "%ε~εσται", _, "%ε~εσᾰται";
    perfect_ind_pass_2: _, _, "%ε~εσθε";
    perfect_subj_act_1: "%ε~εκω", _, _;
    perfect_subj_act_2: "%ε~εκῃς", "%ε~εκητον", "%ε~εκητε";
    perfect_subj_act_3: "%ε~εκῃ", _, "%ε~εκωσῐ";
    perfect_opt_act_1: "%ε~εκοιμῐ", _, "%ε~εκοιμεν";
    perfect_opt_act_2: "%ε~εκοις", _, _;
    perfect_opt_act_3: _, "%ε~εκοιτην", _;
    perfect_imp_act_2: "%ε~εκε", _, "%ε~εκετε";
    perfect_imp_act_3: _, _, "%ε~εκοντων";
    perfect_imp_mid_2: _, _, "%ε~εσθε";
    perfect_imp_mid_3: "%ε~εσθω", _, "%ε~εσθων";
    perfect_imp_pass_2: "%ε~εσο", _, "%ε~εσθε";
    perfect_imp_pass_3: _, "%ε~εσθων", "%ε~εσθων";
    pluperfect_mid_1: "ἐ%ε~εσμην", _, _;
    pluperfect_mid_2: "ἐ%ε~εσο", "ἐ%ε~εσθον", _;
    pluperfect_mid_3: "ἐ%ε~εστο", _, "ἐ%ε~εσᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~εσμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~εσθε";
    pluperfect_pass_3: _, "ἐ%ε~εσθην", _;
    infinitive_fut: "~εῑν", _, _;
    infinitive_aor: "~εσαι", _, _;
    infinitive_perf: _, "%ε~εσθαι", _;
    participle_aor: "~εσᾱς", "~εσᾰμενος", _;
    participle_perf: "%ε~εκως", _, "%ε~εσμενος";
    ),
    // Ἀγορευω: 6 words (ἀγρευω, ἀγορευω, ἀμορβευω, ἀριστευω, ἀπῐστευω, ἀσωτευω). Like Ἐχω, except in 187 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀγορευω", 6, "ἀ~υω", like Ἐχω;
    stem aorist_ind: "~υσ- like ἀγορευω";
    stem aorist_ind_pass: "~υθην- like ἀγορευω";
    ind_act_1: "ἀ~υω", _, "ἀ~υομεν";
    ind_act_2: "ἀ~υεις", _, _;
    ind_act_3: "ἀ~υει", "ἀ~υετον", "ἀ~υουσῐ|ἀ~υουσῐν";
    ind_mid_1: "ἀ~υομαι", _, "ἀ~υομεθᾰ";
    ind_mid_2: "ἀ~υῃ", "ἀ~υεσθον", _;
    ind_pass_2: _, _, "ἀ~υεσθε";
    ind_pass_3: "ἀ~υεται", _, "ἀ~υονται";
    subj_act_1: "ἀ~υω", _, "ἀ~υωμεν";
    subj_act_2: "ἀ~υῃς", _, _;
    subj_act_3: "ἀ~υῃ", "ἀ~υητον", "ἀ~υωσῐ|ἀ~υωσῐν";
    subj_mid_1: _, _, "ἀ~υωμεθᾰ";
    subj_mid_2: _, "ἀ~υησθον", _;
    subj_mid_3: "ἀ~υηται", _, "ἀ~υωνται";
    subj_pass_1: "ἀ~υωμαι", _, _;
    subj_pass_2: _, _, "ἀ~υησθε";
    opt_act_1: "ἀ~υοιμῐ", _, "ἀ~υοιμεν";
    opt_act_2: "ἀ~υοις", "ἀ~υοιτον", _;
    opt_act_3: "ἀ~υοι", "ἀ~υοιτην", "ἀ~υοιεν";
    opt_mid_1: "ἀ~υοιμην", _, "ἀ~υοιμεθᾰ";
    opt_mid_2: "ἀ~υοιο", "ἀ~υοισθον", _;
    opt_mid_3: "ἀ~υοιτο", "ἀ~υοισθην", "ἀ~υοιντο";
    opt_pass_2: _, _, "ἀ~υοισθε";
    imp_act_2: "ἀ~υε", "ἀ~υετον", "ἀ~υετε";
    imp_mid_2: "ἀ~υου", _, "ἀ~υεσθε";
    imp_mid_3: "ἀ~υεσθω", _, "ἀ~υεσθων";
    imp_pass_2: _, _, "ἀ~υεσθε";
    imp_pass_3: _, "ἀ~υεσθων", "ἀ~υεσθων";
    imperfect_act_1: "ἠ~υον", _, "ἠ~υομεν";
    imperfect_act_2: "ἠ~υες", "ἠ~υετον", "ἠ~υετε";
    imperfect_act_3: "ἠ~υε|ἠ~υεν", "ἠ~υετην", "ἠ~υον";
    imperfect_mid_1: "ἠ~υομην", _, "ἠ~υομεθᾰ";
    imperfect_mid_2: "ἠ~υου", _, _;
    imperfect_mid_3: "ἠ~υετο", _, _;
    imperfect_pass_2: _, "ἠ~υεσθον", "ἠ~υεσθε";
    imperfect_pass_3: _, "ἠ~υεσθην", "ἠ~υοντο";
    future_ind_act_1: "ἀ~υσω", _, "ἀ~υσομεν";
    future_ind_act_2: "ἀ~υσεις", "ἀ~υσετον", "ἀ~υσετε";
    future_ind_act_3: _, _, "ἀ~υσουσῐ|ἀ~υσουσῐν";
    future_ind_mid_1: "ἀ~υσομαι", _, "ἀ~υσομεθᾰ";
    future_ind_mid_2: "ἀ~υσῃ", "ἀ~υσεσθον", _;
    future_ind_mid_3: "ἀ~υσεται", _, _;
    future_ind_pass_1: "ἀ~υθησομαι", _, _;
    future_ind_pass_3: _, "ἀ~υθησεσθον", _;
    future_opt_act_1: "ἀ~υσοιμῐ", _, "ἀ~υσοιμεν";
    future_opt_act_2: _, "ἀ~υσοιτον", "ἀ~υσοιτε";
    future_opt_act_3: "ἀ~υσοι", "ἀ~υσοιτην", "ἀ~υσοιεν";
    future_opt_mid_1: "ἀ~υσοιμην", _, "ἀ~υσοιμεθᾰ";
    future_opt_mid_2: "ἀ~υσοιο", _, _;
    future_opt_mid_3: _, "ἀ~υσοισθην", _;
    aorist_subj_act_1: "ἀ~υσω", _, _;
    aorist_subj_act_2: "ἀ~υσῃς", "ἀ~υσητον", _;
    aorist_subj_mid_1: "ἀ~υσωμαι", _, _;
    aorist_subj_mid_2: "ἀ~υσῃ", "ἀ~υσησθον", _;
    aorist_subj_pass_3: "ἀ~υθῃ", "ἀ~υθητον", _;
    aorist_opt_act_1: "ἀ~υσαιμῐ", _, "ἀ~υσαιμεν";
    aorist_opt_act_2: "ἀ~υσειᾰς", _, _;
    aorist_opt_act_3: "ἀ~υσειε", "ἀ~υσαιτην", "ἀ~υσειᾰν";
    aorist_opt_mid_1: "ἀ~υσαιμην", _, _;
    aorist_opt_mid_2: "ἀ~υσαιο", _, _;
    aorist_imp_act_2: "ἀ~υσον", _, "ἀ~υσᾰτε";
    aorist_imp_act_3: _, _, "ἀ~υσᾰντων";
    aorist_imp_mid_2: "ἀ~υσαι", "ἀ~υσᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἀ~υσᾰσθων";
    aorist_imp_pass_2: "ἀ~υθητῐ", "ἀ~υθητον", "ἀ~υθητε";
    perfect_ind_act_1: "ἠ~υκᾰ", _, "ἠ~υκᾰμεν";
    perfect_ind_act_2: _, "ἠ~υκᾰτον", _;
    perfect_ind_act_3: "ἠ~υκε|ἠ~υκεν", _, "ἠ~υκᾱσῐ|ἠ~υκᾱσῐν";
    perfect_ind_mid_1: "ἠ~υμαι", _, "ἠ~υμεθᾰ";
    perfect_ind_mid_2: "ἠ~υσαι", "ἠ~υσθον", _;
    perfect_ind_mid_3: "ἠ~υται", _, "ἠ~υνται";
    perfect_ind_pass_2: _, _, "ἠ~υσθε";
    perfect_subj_act_1: "ἠ~υκω", _, _;
    perfect_subj_act_2: "ἠ~υκῃς", "ἠ~υκητον", "ἠ~υκητε";
    perfect_subj_act_3: "ἠ~υκῃ", _, "ἠ~υκωσῐ|ἠ~υκωσῐν";
    perfect_opt_act_1: "ἠ~υκοιμῐ|ἠ~υκοιην", _, "ἠ~υκοιμεν";
    perfect_opt_act_2: "ἠ~υκοις|ἠ~υκοιης", _, _;
    perfect_opt_act_3: "ἠ~υκοι|ἠ~υκοιη", "ἠ~υκοιτην", _;
    perfect_imp_act_2: "ἠ~υκε", _, "ἠ~υκετε";
    perfect_imp_act_3: _, _, "ἠ~υκοντων";
    perfect_imp_mid_2: _, _, "ἠ~υσθε";
    perfect_imp_mid_3: "ἠ~υσθω", _, "ἠ~υσθων";
    perfect_imp_pass_2: "ἠ~υσο", _, "ἠ~υσθε";
    perfect_imp_pass_3: _, "ἠ~υσθων", "ἠ~υσθων";
    pluperfect_act_1: "ἠ~υκειν", _, _;
    pluperfect_mid_1: "ἠ~υμην", _, _;
    pluperfect_mid_2: "ἠ~υσο", "ἠ~υσθον", _;
    pluperfect_mid_3: "ἠ~υτο", _, "ἠ~υντο";
    pluperfect_pass_1: _, _, "ἠ~υμεθᾰ";
    pluperfect_pass_2: _, _, "ἠ~υσθε";
    pluperfect_pass_3: _, "ἠ~υσθην", _;
    future_perfect_ind_mid_1: "ἠ~υσομαι", _, _;
    future_perfect_ind_mid_3: _, "ἠ~υσεσθον", _;
    future_perfect_ind_pass_1: _, _, "ἠ~υσομεθᾰ";
    future_perfect_ind_pass_2: "ἠ~υσῃ", "ἠ~υσεσθον", "ἠ~υσεσθε";
    future_perfect_ind_pass_3: "ἠ~υσεται", "ἠ~υσεσθον", "ἠ~υσονται";
    future_perfect_opt_pass_1: "ἠ~υσοιμην", _, "ἠ~υσοιμεθᾰ";
    future_perfect_opt_pass_2: "ἠ~υσοιο", "ἠ~υσοισθον", "ἠ~υσοισθε";
    future_perfect_opt_pass_3: "ἠ~υσοιτο", "ἠ~υσοισθην", "ἠ~υσοιντο";
    infinitive_pres: "ἀ~υειν", _, "ἀ~υεσθαι";
    infinitive_fut: "ἀ~υσειν", _, _;
    infinitive_aor: "ἀ~ῡσαι", _, _;
    infinitive_perf: _, "ἠ~ῡσθαι", _;
    infinitive_futp: _, _, "ἠ~υσεσθαι";
    participle_pres: "ἀ~υων", _, "ἀ~υομενος";
    participle_aor: "ἀ~υσᾱς", "ἀ~υσᾰμενος", _;
    participle_perf: "ἠ~υκως", _, "ἠ~υμενος";
    participle_futp: _, _, "ἠ~υσομενος";
    ),
    // Κατοικεω: 6 words (κατοικεω, νεκραγωγεω, δῐασκοπεω, παιδεραστεω, πᾰνηγῠρῐαρχεω, ἐξασθενεω). Like Ἐπᾰϊω, except in 144 cells: ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("κατοικεω", 6, "~α~εω", like Ἐπᾰϊω;
    stem future_pass: "~ᾰ~ηθησ- like ἐχω";
    stem aorist_ind: "~ᾰ~ησ- like ἐλπω";
    stem aorist_ind_pass: "~ᾰ~ηθην- like ἐλπω";
    stem aorist_pass: "~ᾰ~ηθ- like ἐχω";
    ind_act_1: _, _, "~ᾰ~οῡμεν";
    ind_act_2: "~ᾰ~εῑς", _, _;
    ind_act_3: "~ᾰ~εῑ", "~ᾰ~εῑτον", "~ᾰ~οῡσῐ|~ᾰ~οῡσῐν";
    ind_mid_1: "~ᾰ~οῡμαι", _, "~ᾰ~ουμεθᾰ";
    ind_mid_2: "~ᾰ~εῑ", "~ᾰ~εῑσθον", _;
    ind_pass_2: _, _, "~ᾰ~εῑσθε";
    ind_pass_3: "~ᾰ~εῑται", _, "~ᾰ~οῡνται";
    opt_act_1: "~ᾰ~οιην|~ᾰ~οῑμῐ", _, "~ᾰ~οῑμεν|~ᾰ~οιημεν";
    opt_act_2: "~ᾰ~οιης|~ᾰ~οῑς", "~ᾰ~οῑτον|~ᾰ~οιητον", "~ᾰ~οῑτε|~ᾰ~οιητε";
    opt_act_3: "~ᾰ~οιη|~ᾰ~οῑ", "~ᾰ~οιτην|~ᾰ~οιητην", "~ᾰ~οῑεν|~ᾰ~οιησᾰν";
    opt_mid_2: "~ᾰ~οῑο", "~ᾰ~οῑσθον", _;
    opt_mid_3: "~ᾰ~οῑτο", _, "~ᾰ~οῑντο";
    opt_pass_2: _, _, "~ᾰ~οῑσθε";
    imp_act_2: "~ᾰ~ει", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    imp_mid_2: "~ᾰ~οῡ", _, "~ᾰ~εῑσθε";
    imp_mid_3: "~ᾰ~εισθω", _, "~ᾰ~εισθων";
    imp_pass_2: _, _, "~ᾰ~εῑσθε";
    imp_pass_3: _, "~ᾰ~εισθων", "~ᾰ~εισθων";
    imperfect_act_1: "~ᾰ~ουν", _, "~ᾰ~οῡμεν";
    imperfect_act_2: "~ᾰ~εις", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    imperfect_act_3: "~ᾰ~ει", "~ᾰ~ειτην", "~ᾰ~ουν";
    imperfect_mid_1: "~ᾰ~ουμην", _, "~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "~ᾰ~οῡ", _, _;
    imperfect_mid_3: "~ᾰ~εῑτο", _, _;
    imperfect_pass_2: _, "~ᾰ~εῑσθον", "~ᾰ~εῑσθε";
    imperfect_pass_3: _, "~ᾰ~εισθην", "~ᾰ~οῡντο";
    future_ind_act_1: "~ᾰ~ησω", _, "~ᾰ~ησομεν";
    future_ind_act_2: "~ᾰ~ησεις", "~ᾰ~ησετον", "~ᾰ~ησετε";
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ";
    future_ind_mid_1: "~ᾰ~ησομαι", _, "~ᾰ~ησομεθᾰ";
    future_ind_mid_2: "~ᾰ~ησῃ", "~ᾰ~ησεσθον", _;
    future_ind_mid_3: "~ᾰ~ησεται", _, _;
    future_opt_act_1: "~ᾰ~ησοιμῐ", _, "~ᾰ~ησοιμεν";
    future_opt_act_2: _, "~ᾰ~ησοιτον", "~ᾰ~ησοιτε";
    future_opt_act_3: "~ᾰ~ησοι", "~ᾰ~ησοιτην", "~ᾰ~ησοιεν";
    future_opt_mid_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ησοιο", _, _;
    future_opt_mid_3: _, "~ᾰ~ησοισθην", _;
    aorist_subj_act_1: "~ᾰ~ησω", _, _;
    aorist_subj_act_2: "~ᾰ~ησῃς", "~ᾰ~ησητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~ᾰ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ησῃ", "~ᾰ~ησησθον", _;
    aorist_opt_act_1: "~ᾰ~ησαιμῐ", _, "~ᾰ~ησαιμεν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς", _, _;
    aorist_opt_act_3: "~ᾰ~ησειε", "~ᾰ~ησαιτην", "~ᾰ~ησειᾰν";
    aorist_opt_mid_1: "~ᾰ~ησαιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~ησαιο", _, _;
    aorist_imp_act_2: "~ᾰ~ησον", _, "~ᾰ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ησᾰντων";
    aorist_imp_mid_2: "~ᾰ~ησαι", "~ᾰ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ησᾰσθων";
    perfect_ind_act_1: "~ᾰ~ηκᾰ", _, "~ᾰ~ηκᾰμεν";
    perfect_ind_act_2: _, "~ᾰ~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "~ᾰ~ηκᾱσῐ";
    perfect_ind_mid_1: "~ᾰ~ημαι", _, "~ᾰ~ημεθᾰ";
    perfect_ind_mid_2: "~ᾰ~ησαι", "~ᾰ~ησθον", _;
    perfect_ind_mid_3: "~ᾰ~ηται", _, "~ᾰ~ηνται";
    perfect_ind_pass_2: _, _, "~ᾰ~ησθε";
    perfect_subj_act_1: "~ᾰ~ηκω", _, _;
    perfect_subj_act_2: "~ᾰ~ηκῃς", "~ᾰ~ηκητον", "~ᾰ~ηκητε";
    perfect_subj_act_3: "~ᾰ~ηκῃ", _, "~ᾰ~ηκωσῐ";
    perfect_opt_act_1: "~ᾰ~ηκοιμῐ", _, "~ᾰ~ηκοιμεν";
    perfect_opt_act_2: "~ᾰ~ηκοις", _, _;
    perfect_opt_act_3: _, "~ᾰ~ηκοιτην", _;
    perfect_imp_act_2: "~ᾰ~ηκε", _, "~ᾰ~ηκετε";
    perfect_imp_act_3: _, _, "~ᾰ~ηκοντων";
    perfect_imp_mid_2: _, _, "~ᾰ~ησθε";
    perfect_imp_mid_3: "~ᾰ~ησθω", _, "~ᾰ~ησθων";
    perfect_imp_pass_2: "~ᾰ~ησο", _, "~ᾰ~ησθε";
    perfect_imp_pass_3: _, "~ᾰ~ησθων", "~ᾰ~ησθων";
    pluperfect_act_1: "~ᾰ~ηκειν", _, _;
    pluperfect_mid_1: "~ᾰ~ημην", _, _;
    pluperfect_mid_2: "~ᾰ~ησο", "~ᾰ~ησθον", _;
    pluperfect_mid_3: "~ᾰ~ητο", _, "~ᾰ~ηντο";
    pluperfect_pass_1: _, _, "~ᾰ~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ᾰ~ησθε";
    pluperfect_pass_3: _, "~ᾰ~ησθην", _;
    infinitive_pres: "~ᾰ~εῑν", _, "~ᾰ~εῑσθαι";
    infinitive_fut: "~ᾰ~ησειν", _, _;
    infinitive_aor: "~ᾰ~ησαι", _, _;
    infinitive_perf: _, "~ᾰ~ησθαι", _;
    participle_pres: _, _, "~ᾰ~ουμενος";
    participle_aor: "~ᾰ~ησᾱς", "~ᾰ~ησᾰμενος", _;
    participle_perf: "~ᾰ~ηκως", _, "~ᾰ~ημενος";
    ),
    // Λᾰγοθηρεω: 6 words (κρατεω, φωνομᾰχεω, τερᾰτοτοκεω, λᾰγοθηρεω, λᾰβρηγορεω, λᾰρκοφορεω). Like Κατοικεω, except in 65 cells: ind_mid_2 sg, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, ….
    conjugation!("λᾰγοθηρεω", 6, "~ᾰ~εω", like Κατοικεω;
    stem future_pass: "~ᾰ~ηθησ- like ποιεω";
    stem aorist_ind: "~α~ησ- like ποιεω";
    stem aorist_ind_pass: "~α~ηθην- like ἐχω";
    stem aorist_pass: "~ᾰ~ηθ- like ποιεω";
    stem pluperfect_act: "~α~ηκειν- like ἐχω";
    ind_mid_2: "~ᾰ~εῑ|~ᾰ~ῃ", _, _;
    imperfect_act_1: "ἐ~α~ουν", _, "ἐ~α~οῡμεν";
    imperfect_act_2: "ἐ~α~εις", "ἐ~α~εῑτον", "ἐ~α~εῑτε";
    imperfect_act_3: "ἐ~α~ει", "ἐ~α~ειτην", "ἐ~α~ουν";
    imperfect_mid_1: "ἐ~α~ουμην", _, "ἐ~α~ουμεθᾰ";
    imperfect_mid_2: "ἐ~α~οῡ", _, _;
    imperfect_mid_3: "ἐ~α~εῑτο", _, _;
    imperfect_pass_2: _, "ἐ~α~εῑσθον", "ἐ~α~εῑσθε";
    imperfect_pass_3: _, "ἐ~α~εισθην", "ἐ~α~οῡντο";
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ|~ᾰ~ησουσῐν";
    future_ind_mid_2: "~ᾰ~ησῃ|~ᾰ~ησει", _, _;
    aorist_subj_act_3: _, _, "~ᾰ~ησωσῐ|~ᾰ~ησωσῐν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς|~ᾰ~ησαις", _, _;
    aorist_opt_act_3: "~ᾰ~ησειε|~ᾰ~ησειεν|~ᾰ~ησαι", _, "~ᾰ~ησειᾰν|~ᾰ~ησαιεν";
    perfect_ind_act_1: "%ε~α~ηκᾰ", _, "%ε~α~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~α~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~α~ηκᾱσῐ";
    perfect_ind_mid_1: "%ε~α~ημαι", _, "%ε~α~ημεθᾰ";
    perfect_ind_mid_2: "%ε~α~ησαι", "%ε~α~ησθον", _;
    perfect_ind_mid_3: "%ε~α~ηται", _, "%ε~α~ηνται";
    perfect_ind_pass_2: _, _, "%ε~α~ησθε";
    perfect_subj_act_1: "%ε~α~ηκω", _, _;
    perfect_subj_act_2: "%ε~α~ηκῃς", "%ε~α~ηκητον", "%ε~α~ηκητε";
    perfect_subj_act_3: "%ε~α~ηκῃ", _, "%ε~α~ηκωσῐ";
    perfect_opt_act_1: "%ε~α~ηκοιμῐ", _, "%ε~α~ηκοιμεν";
    perfect_opt_act_2: "%ε~α~ηκοις", _, _;
    perfect_opt_act_3: _, "%ε~α~ηκοιτην", _;
    perfect_imp_act_2: "%ε~α~ηκε", _, "%ε~α~ηκετε";
    perfect_imp_act_3: _, _, "%ε~α~ηκοντων";
    perfect_imp_mid_2: _, _, "%ε~α~ησθε";
    perfect_imp_mid_3: "%ε~α~ησθω", _, "%ε~α~ησθων";
    perfect_imp_pass_2: "%ε~α~ησο", _, "%ε~α~ησθε";
    perfect_imp_pass_3: _, "%ε~α~ησθων", "%ε~α~ησθων";
    pluperfect_mid_1: "ἐ%ε~α~ημην", _, _;
    pluperfect_mid_2: "ἐ%ε~α~ησο", "ἐ%ε~α~ησθον", _;
    pluperfect_mid_3: "ἐ%ε~α~ητο", _, "ἐ%ε~α~ηντο";
    pluperfect_pass_1: _, _, "ἐ%ε~α~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~α~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~α~ησθην", _;
    infinitive_perf: _, "%ε~α~ησθαι", _;
    participle_perf: "%ε~α~ηκως", _, "%ε~α~ημενος";
    ),
    // Μηκᾰομαι: 6 words (ἰᾰομαι, θεᾰομαι, ἀκροᾰομαι, αἰτῐᾰομαι, ἡσσᾰομαι, μηκᾰομαι). Like Ἀραομαι, except in 43 cells: ind_act_1 sg, imperfect_mid_1 sg, imperfect_mid_1 pl, imperfect_mid_2 sg, imperfect_mid_2 du, imperfect_mid_3 sg, ….
    conjugation!("μηκᾰομαι", 6, "~ᾰομαι", like Ἀραομαι;
    stem future_pass: "~ᾱθησ- like ἐχω";
    stem aorist_ind: "~ᾱσ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ᾱθην- like ἐλπω";
    stem aorist_pass: "~ᾱθ- like ἐχω";
    ind_act_1: "~ᾰομαι", _, _;
    imperfect_mid_1: "~ωμην", _, "~ωμεθᾰ";
    imperfect_mid_2: "~ω", "~ᾱσθον", _;
    imperfect_mid_3: "~ᾱτο", "~ᾱσθην", "~ωντο";
    imperfect_pass_2: _, _, "~ᾱσθε";
    future_ind_mid_1: "~ᾱσομαι", _, "~ᾱσομεθᾰ";
    future_ind_mid_2: "~ᾱσῃ", "~ᾱσεσθον", _;
    future_ind_mid_3: "~ᾱσεται", _, _;
    future_opt_mid_1: "~ᾱσοιμην", _, "~ᾱσοιμεθᾰ";
    future_opt_mid_2: "~ᾱσοιο", _, _;
    future_opt_mid_3: _, "~ᾱσοισθην", _;
    aorist_subj_mid_1: "~ᾱσωμαι", _, _;
    aorist_subj_mid_2: "~ᾱσῃ", "~ᾱσησθον", _;
    aorist_opt_mid_1: "~ᾱσαιμην", _, _;
    aorist_opt_mid_2: "~ᾱσαιο", _, _;
    aorist_imp_mid_2: "~ᾱσαι", "~ᾱσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾱσᾰσθων";
    perfect_ind_mid_1: "~ᾱμαι", _, "~ᾱμεθᾰ";
    perfect_ind_mid_2: "~ᾱσαι", "~ᾱσθον", _;
    perfect_ind_mid_3: "~ᾱται", _, "~ᾱνται";
    perfect_ind_pass_2: _, _, "~ᾱσθε";
    perfect_imp_mid_2: _, _, "~ᾱσθε";
    perfect_imp_mid_3: "~ᾱσθω", _, "~ᾱσθων";
    perfect_imp_pass_2: "~ᾱσο", _, "~ᾱσθε";
    perfect_imp_pass_3: _, "~ᾱσθων", "~ᾱσθων";
    infinitive_perf: _, "~ᾱσθαι", _;
    participle_aor: _, "~ᾱσᾰμενος", _;
    participle_perf: _, _, "~ᾱμενος";
    ),
    // Ἐπεξειμι: 6 words (ἐξειμι, ἀπειμι, κᾰτειμῐ, προσειμι, προειμι, ἐπεξειμι). Like Μῑγνῡμῐ, except in 33 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐπεξειμι", 6, "~ειμι", like Μῑγνῡμῐ;
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
    imp_act_3: "~ῐτω", _, "~ῐοντων";
    imperfect_act_1: "~ῃᾰ|~ῃειν", _, "~ῃμεν";
    imperfect_act_2: "~ῃεισθᾰ|~ῃεις", "~ῃτον", "~ῃτε";
    imperfect_act_3: "~ῃειν|~ῃει", "~ῃτην", "~ῃσᾰν|~ῃεσᾰν";
    infinitive_pres: "~ῐεναι", _, _;
    participle_pres: "~ῐων", _, _;
    ),
    // Ἐαω: 6 words (ἐαω, θηραω, ἀπερᾱω, ἐξερᾱω, θῡμῐαω, φωραω). Like Νῑκαω, except in 81 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 pl, ….
    conjugation!("ἐαω", 6, "~αω", like Νῑκαω;
    stem future_pass: "~ᾱθησ- like ἐχω";
    stem aorist_ind: "~ᾱσ- like ποιεω";
    stem aorist_ind_pass: "~ᾱθην- like ἐχω";
    stem aorist_pass: "~ᾱθ- like ἐχω";
    future_ind_act_1: "~ᾱσω", _, "~ᾱσομεν";
    future_ind_act_2: "~ᾱσεις", "~ᾱσετον", "~ᾱσετε";
    future_ind_act_3: _, _, "~ᾱσουσῐ";
    future_ind_mid_1: "~ᾱσομαι", _, "~ᾱσομεθᾰ";
    future_ind_mid_2: "~ᾱσῃ|~ᾱσει", "~ᾱσεσθον", _;
    future_ind_mid_3: "~ᾱσεται", _, _;
    future_opt_act_1: "~ᾱσοιμῐ", _, "~ᾱσοιμεν";
    future_opt_act_2: _, "~ᾱσοιτον", "~ᾱσοιτε";
    future_opt_act_3: "~ᾱσοι", "~ᾱσοιτην", "~ᾱσοιεν";
    future_opt_mid_1: "~ᾱσοιμην", _, "~ᾱσοιμεθᾰ";
    future_opt_mid_2: "~ᾱσοιο", _, _;
    future_opt_mid_3: _, "~ᾱσοισθην", _;
    aorist_subj_act_1: "~ᾱσω", _, _;
    aorist_subj_act_2: "~ᾱσῃς", "~ᾱσητον", _;
    aorist_subj_act_3: _, _, "~ᾱσωσῐ|~ᾱσωσῐν";
    aorist_subj_mid_1: "~ᾱσωμαι", _, _;
    aorist_subj_mid_2: "~ᾱσῃ", "~ᾱσησθον", _;
    aorist_opt_act_1: "~ᾱσαιμῐ", _, "~ᾱσαιμεν";
    aorist_opt_act_2: "~ᾱσειᾰς|~ᾱσαις", _, _;
    aorist_opt_act_3: "~ᾱσειε|~ᾱσειεν|~ᾱσαι", "~ᾱσαιτην", "~ᾱσειᾰν|~ᾱσαιεν";
    aorist_opt_mid_1: "~ᾱσαιμην", _, _;
    aorist_opt_mid_2: "~ᾱσαιο", _, _;
    aorist_imp_act_2: "~ᾱσον", _, "~ᾱσᾰτε";
    aorist_imp_act_3: _, _, "~ᾱσᾰντων";
    aorist_imp_mid_2: "~ᾱσαι", "~ᾱσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾱσᾰσθων";
    perfect_ind_act_1: "%ε~ᾱκᾰ", _, "%ε~ᾱκᾰμεν";
    perfect_ind_act_2: _, "%ε~ᾱκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ᾱκᾱσῐ";
    perfect_ind_mid_1: "%ε~ᾰμαι", _, "%ε~ᾰμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾰσαι", "%ε~ᾰσθον", _;
    perfect_ind_mid_3: "%ε~ᾰται", _, "%ε~ᾰνται";
    perfect_ind_pass_2: _, _, "%ε~ᾰσθε";
    perfect_subj_act_1: "%ε~ᾱκω", _, _;
    perfect_subj_act_2: "%ε~ᾱκῃς", "%ε~ᾱκητον", "%ε~ᾱκητε";
    perfect_subj_act_3: "%ε~ᾱκῃ", _, "%ε~ᾱκωσῐ";
    perfect_opt_act_1: "%ε~ᾱκοιμῐ", _, "%ε~ᾱκοιμεν";
    perfect_opt_act_2: "%ε~ᾱκοις", _, _;
    perfect_opt_act_3: _, "%ε~ᾱκοιτην", _;
    perfect_imp_act_2: "%ε~ᾱκε", _, "%ε~ᾱκετε";
    perfect_imp_act_3: _, _, "%ε~ᾱκοντων";
    perfect_imp_mid_2: _, _, "%ε~ᾰσθε";
    perfect_imp_mid_3: "%ε~ᾰσθω", _, "%ε~ᾰσθων";
    perfect_imp_pass_2: "%ε~ᾰσο", _, "%ε~ᾰσθε";
    perfect_imp_pass_3: _, "%ε~ᾰσθων", "%ε~ᾰσθων";
    infinitive_fut: "~ᾱσειν", _, _;
    infinitive_aor: "~ᾱσαι", _, _;
    infinitive_perf: _, "%ε~ᾰσθαι", _;
    participle_aor: "~ᾱσᾱς", "~ᾱσᾰμενος", _;
    participle_perf: "%ε~ᾱκως", _, "%ε~ᾰμενος";
    ),
    // Ἐρχομαι: 6 words (ἐρχομαι, ἑζομαι, ἡδομαι, κᾰθεζομαι, ἀπαλθαινομαι, ὠρῡομαι). Like Κρῑνομαι, except in 44 cells: ind_mid_2 sg, imperfect_mid_1 sg, imperfect_mid_1 pl, imperfect_mid_2 sg, imperfect_mid_2 du, imperfect_mid_3 sg, ….
    conjugation!("ἐρχομαι", 6, "~ομαι", like Κρῑνομαι;
    stem aorist_ind: "~- like ἐρχομαι";
    ind_mid_2: "~ῃ|~ει", _, _;
    imperfect_mid_1: "~ομην", _, "~ομεθᾰ";
    imperfect_mid_2: "~ου", "~εσθον", _;
    imperfect_mid_3: "~ετο", "~εσθην", "~οντο";
    imperfect_pass_2: _, _, "~εσθε";
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, _, _;
    future_opt_mid_3: _, derived, _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_mid_1: "~οιμην", _, _;
    aorist_opt_mid_2: "~οιο", _, _;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_mid_2: "~οῡ", "~εσθον", _;
    aorist_imp_mid_3: _, _, "~εσθων";
    infinitive_aor: derived, _, _;
    participle_aor: derived, "~ομενος", _;
    ),
    // Ἐπῐλᾰμβᾰνω: 6 words (ἐπῐλᾰμβᾰνω, προλᾰμβᾰνω, ὑπολᾰμβᾰνω, ἀπολᾰμβᾰνω, ξῠλλᾰμβᾰνω, προσλαμβᾰνω). Like Δερκομαι, except in 143 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐπῐλᾰμβᾰνω", 6, "~~λᾰμβᾰνω", like Δερκομαι;
    stem future_pass: "~~ληφθησ- like ἐχω";
    stem aorist_ind: "~νε~ᾰβ- like ἀνᾰγω";
    stem aorist_ind_pass: "~νε~ηφθην- like ἐλπω";
    stem aorist_pass: "~~ληφθ- like ἐχω";
    ind_act_1: "~~λᾰμβᾰνω", _, "~~λᾰμβᾰνομεν";
    ind_act_2: "~~λᾰμβᾰνεις", _, _;
    ind_act_3: "~~λᾰμβᾰνει", "~~λᾰμβᾰνετον", "~~λᾰμβᾰνουσῐ";
    ind_mid_1: "~~λᾰμβᾰνομαι", _, "~~λᾰμβᾰνομεθᾰ";
    ind_mid_2: "~~λᾰμβᾰνῃ", "~~λᾰμβᾰνεσθον", _;
    ind_pass_2: _, _, "~~λᾰμβᾰνεσθε";
    ind_pass_3: "~~λᾰμβᾰνεται", _, "~~λᾰμβᾰνονται";
    subj_act_1: "~~λᾰμβᾰνω", _, "~~λᾰμβᾰνωμεν";
    subj_act_2: "~~λᾰμβᾰνῃς", _, _;
    subj_act_3: "~~λᾰμβᾰνῃ", "~~λᾰμβᾰνητον", "~~λᾰμβᾰνωσῐ";
    subj_mid_1: derived, _, "~~λᾰμβᾰνωμεθᾰ";
    subj_mid_2: derived, "~~λᾰμβᾰνησθον", _;
    subj_mid_3: "~~λᾰμβᾰνηται", _, "~~λᾰμβᾰνωνται";
    subj_pass_1: "~~λᾰμβᾰνωμαι", _, _;
    subj_pass_2: derived, _, "~~λᾰμβᾰνησθε";
    opt_act_1: "~~λᾰμβᾰνοιμῐ", _, "~~λᾰμβᾰνοιμεν";
    opt_act_2: "~~λᾰμβᾰνοις", "~~λᾰμβᾰνοιτον", _;
    opt_act_3: "~~λᾰμβᾰνοι", "~~λᾰμβᾰνοιτην", "~~λᾰμβᾰνοιεν";
    opt_mid_1: "~~λᾰμβᾰνοιμην", _, "~~λᾰμβᾰνοιμεθᾰ";
    opt_mid_2: "~~λᾰμβᾰνοιο", "~~λᾰμβᾰνοισθον", _;
    opt_mid_3: "~~λᾰμβᾰνοιτο", "~~λᾰμβᾰνοισθην", "~~λᾰμβᾰνοιντο";
    opt_pass_2: _, _, "~~λᾰμβᾰνοισθε";
    imp_act_2: "~~λᾰμβᾰνε", "~~λᾰμβᾰνετον", "~~λᾰμβᾰνετε";
    imp_mid_2: "~~λᾰμβᾰνου", _, "~~λᾰμβᾰνεσθε";
    imp_mid_3: "~~λᾰμβᾰνεσθω", _, "~~λᾰμβᾰνεσθων";
    imp_pass_2: _, _, "~~λᾰμβᾰνεσθε";
    imp_pass_3: _, "~~λᾰμβᾰνεσθων", "~~λᾰμβᾰνεσθων";
    imperfect_act_1: "~νε~ᾰμβᾰνον", _, "~νε~ᾰμβᾰνομεν";
    imperfect_act_2: "~νε~ᾰμβᾰνες", "~νε~ᾰμβᾰνετον", "~νε~ᾰμβᾰνετε";
    imperfect_act_3: "~νε~ᾰμβᾰνε", "~νε~ᾰμβᾰνετην", "~νε~ᾰμβᾰνον";
    imperfect_mid_1: "~νε~ᾰμβᾰνομην", _, "~νε~ᾰμβᾰνομεθᾰ";
    imperfect_mid_2: "~νε~ᾰμβᾰνου", derived, _;
    imperfect_mid_3: "~νε~ᾰμβᾰνετο", derived, derived;
    imperfect_pass_2: _, "~νε~ᾰμβᾰνεσθον", "~νε~ᾰμβᾰνεσθε";
    imperfect_pass_3: _, "~νε~ᾰμβᾰνεσθην", "~νε~ᾰμβᾰνοντο";
    future_ind_mid_1: "~~ληψομαι", _, "~~ληψομεθᾰ";
    future_ind_mid_2: "~~ληψῃ", "~~ληψεσθον", _;
    future_ind_mid_3: "~~ληψεται", _, _;
    future_opt_mid_1: "~~ληψοιμην", _, "~~ληψοιμεθᾰ";
    future_opt_mid_2: "~~ληψοιο", _, _;
    future_opt_mid_3: _, "~~ληψοισθην", _;
    aorist_subj_act_1: "~~λᾰβω", _, _;
    aorist_subj_act_2: "~~λᾰβῃς", "~~λᾰβητον", _;
    aorist_subj_mid_1: "~~λᾰβωμαι", _, _;
    aorist_subj_mid_2: "~~λᾰβῃ", "~~λᾰβησθον", _;
    aorist_opt_act_1: "~~λᾰβοιμῐ", _, "~~λᾰβοιμεν";
    aorist_opt_act_2: "~~λᾰβοις", _, _;
    aorist_opt_act_3: "~~λᾰβοι", "~~λᾰβοιτην", "~~λᾰβοιεν";
    aorist_opt_mid_1: "~~λᾰβοιμην", _, _;
    aorist_opt_mid_2: "~~λᾰβοιο", _, _;
    aorist_imp_act_2: "~~λᾰβε", _, "~~λᾰβετε";
    aorist_imp_act_3: _, _, "~~λᾰβοντων";
    aorist_imp_mid_2: "~~λᾰβοῡ", "~~λᾰβεσθον", _;
    aorist_imp_mid_3: _, _, "~~λᾰβεσθων";
    perfect_ind_act_1: "~νει~ηφᾰ", _, "~νει~ηφᾰμεν";
    perfect_ind_act_2: _, "~νει~ηφᾰτον", _;
    perfect_ind_act_3: _, _, "~νει~ηφᾱσῐ";
    perfect_ind_mid_1: "~νει~ημμαι", _, "~νει~ημμεθᾰ";
    perfect_ind_mid_2: "~νει~ηψαι", "~νει~ηφθον", _;
    perfect_ind_mid_3: "~νει~ηπται", _, "~νει~ηφᾰται";
    perfect_ind_pass_2: _, _, "~νει~ηφθε";
    perfect_subj_act_1: "~νει~ηφω", _, _;
    perfect_subj_act_2: "~νει~ηφῃς", "~νει~ηφητον", "~νει~ηφητε";
    perfect_subj_act_3: "~νει~ηφῃ", _, "~νει~ηφωσῐ";
    perfect_opt_act_1: "~νει~ηφοιμῐ", _, "~νει~ηφοιμεν";
    perfect_opt_act_2: "~νει~ηφοις", _, _;
    perfect_opt_act_3: _, "~νει~ηφοιτην", _;
    perfect_imp_act_2: "~νει~ηφε", _, "~νει~ηφετε";
    perfect_imp_act_3: _, _, "~νει~ηφοντων";
    perfect_imp_mid_2: _, _, "~νει~ηφθε";
    perfect_imp_mid_3: "~νει~ηφθω", _, "~νει~ηφθων";
    perfect_imp_pass_2: "~νει~ηψο", _, "~νει~ηφθε";
    perfect_imp_pass_3: _, "~νει~ηφθων", "~νει~ηφθων";
    infinitive_pres: "~~λᾰμβᾰνειν", _, "~~λᾰμβᾰνεσθαι";
    infinitive_aor: "~~λᾰβεῑν", _, _;
    infinitive_perf: _, "~νει~ηφθαι", _;
    participle_pres: "~~λᾰμβᾰνων", derived, "~~λᾰμβᾰνομενος";
    participle_aor: "~~λᾰβων", "~~λᾰβομενος", _;
    participle_perf: "~νει~ηφως", _, "~νει~ημμενος";
    ),
    // Ἡδῠνω: 6 words (βᾰρῠνω, βᾰθῠνω, ἡδῠνω, πλᾰτῠνω, βρᾰδῠνω, βρᾰχῠνω). Like Πορσῡνω, except in 107 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἡδῠνω", 6, "~ῠνω", like Πορσῡνω;
    stem aorist_ind: "~ῡν- like ἐχω";
    ind_act_1: "~ῠνω", _, "~ῠνομεν";
    ind_act_2: "~ῠνεις", _, _;
    ind_act_3: "~ῠνει", "~ῠνετον", "~ῠνουσῐ";
    ind_mid_1: "~ῠνομαι", _, "~ῠνομεθᾰ";
    ind_mid_2: "~ῠνῃ", "~ῠνεσθον", _;
    ind_pass_2: _, _, "~ῠνεσθε";
    ind_pass_3: "~ῠνεται", _, "~ῠνονται";
    subj_act_1: "~ῠνω", _, "~ῠνωμεν";
    subj_act_2: "~ῠνῃς", _, _;
    subj_act_3: "~ῠνῃ", "~ῠνητον", "~ῠνωσῐ";
    subj_mid_1: _, _, "~ῠνωμεθᾰ";
    subj_mid_2: _, "~ῠνησθον", _;
    subj_mid_3: "~ῠνηται", _, "~ῠνωνται";
    subj_pass_1: "~ῠνωμαι", _, _;
    subj_pass_2: _, _, "~ῠνησθε";
    opt_act_1: "~ῠνοιμῐ", _, "~ῠνοιμεν";
    opt_act_2: "~ῠνοις", "~ῠνοιτον", _;
    opt_act_3: "~ῠνοι", "~ῠνοιτην", "~ῠνοιεν";
    opt_mid_1: "~ῠνοιμην", _, "~ῠνοιμεθᾰ";
    opt_mid_2: "~ῠνοιο", "~ῠνοισθον", _;
    opt_mid_3: "~ῠνοιτο", "~ῠνοισθην", "~ῠνοιντο";
    opt_pass_2: _, _, "~ῠνοισθε";
    imp_act_2: "~ῠνε", "~ῠνετον", "~ῠνετε";
    imp_mid_2: "~ῠνου", _, "~ῠνεσθε";
    imp_mid_3: "~ῠνεσθω", _, "~ῠνεσθων";
    imp_pass_2: _, _, "~ῠνεσθε";
    imp_pass_3: _, "~ῠνεσθων", "~ῠνεσθων";
    imperfect_act_1: "ἐ~ῠνον", _, "ἐ~ῠνομεν";
    imperfect_act_2: "ἐ~ῠνες", "ἐ~ῠνετον", "ἐ~ῠνετε";
    imperfect_act_3: "ἐ~ῠνε", "ἐ~ῠνετην", "ἐ~ῠνον";
    imperfect_mid_1: "ἐ~ῠνομην", _, "ἐ~ῠνομεθᾰ";
    imperfect_mid_2: "ἐ~ῠνου", _, _;
    imperfect_mid_3: "ἐ~ῠνετο", _, _;
    imperfect_pass_2: _, "ἐ~ῠνεσθον", "ἐ~ῠνεσθε";
    imperfect_pass_3: _, "ἐ~ῠνεσθην", "ἐ~ῠνοντο";
    future_ind_act_3: _, _, "~ῠνοῡσῐ";
    future_ind_mid_2: "~ῠνεῑ", _, _;
    future_opt_act_1: "~ῠνοιην", _, "~ῠνοῑμεν";
    future_opt_act_2: derived, "~ῠνοῑτον", "~ῠνοῑτε";
    future_opt_act_3: "~ῠνοιη", "~ῠνοιτην", "~ῠνοῑεν";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: _, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_pres: "~ῠνειν", _, "~ῠνεσθαι";
    participle_pres: "~ῠνων", _, "~ῠνομενος";
    participle_perf: derived, _, _;
    ),
    // Περιειμι: 6 words (πᾰρειμι, ἐπειμι, μετειμι, περιειμι, ἐνειμι, σῠνειμῐ). Like Ἐπεξειμι, except in 41 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, subj_act_1 sg, ….
    conjugation!("περιειμι", 6, "~ειμι", like Ἐπεξειμι;
    ind_act_1: _, _, "~εσμεν";
    ind_act_2: "~εῑ", "~εστον", _;
    ind_act_3: "~εστι", _, "~εισι";
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", "~ητον", _;
    subj_act_3: "~ῃ", _, "~ωσι";
    opt_act_1: "~ειην", _, "~ειημεν";
    opt_act_2: "~ειης", "~ειητον", _;
    opt_act_3: "~ειη", "~ειητην", "~ειησαν";
    imp_act_2: "~ισθι", _, "~εστε";
    imp_act_3: "~εστω", _, "~εστων";
    imperfect_act_1: "~η", _, "~ημεν";
    imperfect_act_2: "~ησθα", "~ηστον", "~ητε";
    imperfect_act_3: "~ην", "~ηστην", "~ησαν";
    future_ind_mid_1: "~εσομαι", _, "~εσομεθᾰ";
    future_ind_mid_2: "~εσῃ", "~εσεσθον", _;
    future_ind_mid_3: "~εσται", _, _;
    future_opt_mid_1: "~εσοιμην", _, "~εσοιμεθᾰ";
    future_opt_mid_2: "~εσοιο", _, _;
    future_opt_mid_3: _, "~εσοισθην", _;
    infinitive_pres: "~εῑναι", _, _;
    participle_pres: "~ων", _, _;
    ),
    // Πρεσβευω: 6 words (ἀκουω, κελευω, πρεσβευω, μᾰθητευω, λαμπᾰδευω, λαυω). Like Παυω, except in 9 cells: imp_act_2 sg, aorist_imp_act_2 sg, aorist_imp_mid_2 sg, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, ….
    conjugation!("πρεσβευω", 6, "~υω", like Παυω;
    stem future_pass: "~υσθησ- like ποιεω";
    stem aorist_pass: "~υσθ- like ποιεω";
    imp_act_2: "~υε", _, _;
    aorist_imp_act_2: "~υσον", _, _;
    aorist_imp_mid_2: "~υσαι", _, _;
    perfect_ind_act_3: "%ε~υκε|%ε~υκεν", _, "%ε~υκᾱσῐ|%ε~υκᾱσῐν";
    perfect_subj_act_3: _, _, "%ε~υκωσῐ|%ε~υκωσῐν";
    perfect_opt_act_1: "%ε~υκοιμῐ|%ε~υκοιην", _, _;
    perfect_opt_act_2: "%ε~υκοις|%ε~υκοιης", _, _;
    perfect_opt_act_3: "%ε~υκοι|%ε~υκοιη", _, _;
    ),
    // Ζευγνῡμῐ: 6 words (ζευγνῡμῐ, ἀνοιγνῡμῐ, ὀμοργνῡμῐ, προσμειγνῡμῐ, μειγνῡμῐ, κᾰτᾰμῑγνῡμῐ).
    conjugation!("ζευγνῡμῐ", 6, "~~~γνῡμῐ";
    stem aorist_ind: "~ε~~ξ- like ἐλπω";
    ind_act_1: "~~~γνῡμῐ", _, "~~~γνῠμεν";
    ind_act_2: "~~~γνῡς", _, _;
    ind_act_3: "~~~γνῡσῐ", "~~~γνῠτον", "~~~γνῠᾱσῐ";
    ind_mid_1: "~~~γνῠμαι", _, "~~~γνῠμεθᾰ";
    ind_mid_2: "~~~γνῠσαι", "~~~γνῠσθον", _;
    ind_pass_2: _, _, "~~~γνῠσθε";
    ind_pass_3: "~~~γνῠται", _, "~~~γνῠνται";
    subj_act_1: "~~~γνῠω", _, "~~~γνῠωμεν";
    subj_act_2: "~~~γνῠῃς", _, _;
    subj_act_3: "~~~γνῠῃ", "~~~γνῠητον", "~~~γνῠωσῐ";
    subj_mid_1: _, _, "~~~γνῠωμεθᾰ";
    subj_mid_2: _, "~~~γνῠησθον", _;
    subj_mid_3: "~~~γνῠηται", _, "~~~γνῠωνται";
    subj_pass_1: "~~~γνῠωμαι", _, _;
    subj_pass_2: _, _, "~~~γνῠησθε";
    opt_act_1: "~~~γνῠοιμῐ", _, "~~~γνῠοιμεν";
    opt_act_2: "~~~γνῠοις", "~~~γνῠοιτον", _;
    opt_act_3: "~~~γνῠοι", "~~~γνῠοιτην", "~~~γνῠοιεν";
    opt_mid_1: "~~~γνῠοιμην", _, "~~~γνῠοιμεθᾰ";
    opt_mid_2: "~~~γνῠοιο", "~~~γνῠοισθον", _;
    opt_mid_3: "~~~γνῠοιτο", "~~~γνῠοισθην", "~~~γνῠοιντο";
    opt_pass_2: _, _, "~~~γνῠοισθε";
    imp_act_2: "~~~γνῡ", "~~~γνῠτον", "~~~γνῠτε";
    imp_mid_2: "~~~γνῠσο", _, "~~~γνῠσθε";
    imp_mid_3: "~~~γνῠσθω", _, "~~~γνῠσθων";
    imp_pass_2: _, _, "~~~γνῠσθε";
    imp_pass_3: _, "~~~γνῠσθων", "~~~γνῠσθων";
    imperfect_act_1: "~ε~~γνῡν", _, "~ε~~γνῠμεν";
    imperfect_act_2: "~ε~~γνῡς", "~ε~~γνῠτον", "~ε~~γνῠτε";
    imperfect_act_3: "~ε~~γνῡ", "~ε~~γνῠτην", "~ε~~γνῠσᾰν";
    imperfect_mid_1: "~ε~~γνῠμην", _, "~ε~~γνῠμεθᾰ";
    imperfect_mid_2: "~ε~~γνῠσο", _, _;
    imperfect_mid_3: "~ε~~γνῠτο", _, _;
    imperfect_pass_2: _, "~ε~~γνῠσθον", "~ε~~γνῠσθε";
    imperfect_pass_3: _, "~ε~~γνῠσθην", "~ε~~γνῠντο";
    future_ind_act_1: "~~~ξω", _, "~~~ξομεν";
    future_ind_act_2: "~~~ξεις", "~~~ξετον", "~~~ξετε";
    future_ind_act_3: _, _, "~~~ξουσῐ";
    future_ind_mid_1: "~~~ξομαι", _, "~~~ξομεθᾰ";
    future_ind_mid_2: "~~~ξῃ", "~~~ξεσθον", _;
    future_ind_mid_3: "~~~ξεται", _, _;
    future_opt_act_1: "~~~ξοιμῐ", _, "~~~ξοιμεν";
    future_opt_act_2: _, "~~~ξοιτον", "~~~ξοιτε";
    future_opt_act_3: "~~~ξοι", "~~~ξοιτην", "~~~ξοιεν";
    future_opt_mid_1: "~~~ξοιμην", _, "~~~ξοιμεθᾰ";
    future_opt_mid_2: "~~~ξοιο", _, _;
    future_opt_mid_3: _, "~~~ξοισθην", _;
    aorist_subj_act_1: "~ῐ~~ξω", _, _;
    aorist_subj_act_2: "~ῐ~~ξῃς", "~ῐ~~ξητον", _;
    aorist_subj_mid_1: "~ῐ~~ξωμαι", _, _;
    aorist_subj_mid_2: "~ῐ~~ξῃ", "~ῐ~~ξησθον", _;
    aorist_opt_act_1: "~ῐ~~ξαιμῐ", _, "~ῐ~~ξαιμεν";
    aorist_opt_act_2: "~ῐ~~ξειᾰς", _, _;
    aorist_opt_act_3: "~ῐ~~ξειε", "~ῐ~~ξαιτην", "~ῐ~~ξειᾰν";
    aorist_opt_mid_1: "~ῐ~~ξαιμην", _, _;
    aorist_opt_mid_2: "~ῐ~~ξαιο", _, _;
    aorist_imp_act_2: "~ῐ~~ξον", _, "~ῐ~~ξᾰτε";
    aorist_imp_act_3: _, _, "~ῐ~~ξᾰντων";
    aorist_imp_mid_2: "~ῐ~~ξαι", "~ῐ~~ξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῐ~~ξᾰσθων";
    infinitive_pres: "~~~γνῠναι", _, "~~~γνῠσθαι";
    infinitive_fut: "~~~ξειν", _, _;
    infinitive_aor: "~ῐ~ῑξα~", _, _;
    participle_pres: "~~~γνῡς", _, "~~~γνῠμενος";
    participle_aor: "~ῐ~~ξᾱς", "~ῐ~~ξᾰμενος", _;
    ),
    // Ἡκω: 5 words (ἡκω, πᾰρηκω, προσηκω, τεθνηκω, κᾰθηκω). Like Δῐωκω, except in 70 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, ….
    conjugation!("ἡκω", 5, "~κω", like Δῐωκω;
    stem aorist_ind: "~ξ- like εὐλογεω";
    ind_act_3: _, _, "~κουσῐ|~κουσῐν";
    ind_mid_2: "~κῃ|~κει", _, _;
    subj_act_3: _, _, "~κωσῐ|~κωσῐν";
    imperfect_act_1: "~κον", _, "~κομεν";
    imperfect_act_2: "~κες", "~κετον", "~κετε";
    imperfect_act_3: "~κε|~κεν", "~κετην", "~κον";
    imperfect_mid_1: "~κομην", _, "~κομεθᾰ";
    imperfect_mid_2: "~κου", _, _;
    imperfect_mid_3: "~κετο", _, _;
    imperfect_pass_2: _, "~κεσθον", "~κεσθε";
    imperfect_pass_3: _, "~κεσθην", "~κοντο";
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
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "~κω", _, _;
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ";
    perfect_opt_act_1: "~κοιμῐ", _, "~κοιμεν";
    perfect_opt_act_2: "~κοις", _, _;
    perfect_opt_act_3: _, "~κοιτην", _;
    perfect_imp_act_2: "~κε", _, "~κετε";
    perfect_imp_act_3: _, _, "~κοντων";
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: derived, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: _, derived, derived;
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
    // Ἱζω: 5 words (ἑλληνῐζω, ἀφῠπνῐζω, ἡσῠχᾰζω, ῥωμᾱϊζω, ἐξῠβρῐζω). Like Λογῐζομαι, except in 66 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, ….
    conjugation!("ἱζω", 5, "~ζω", like Λογῐζομαι;
    stem aorist_ind: "~σ- like ἐλπω";
    stem aorist_ind_pass: "~σθην- like ἐλπω";
    ind_act_3: _, _, "~ζουσῐ|~ζουσῐν";
    ind_mid_2: "~ζῃ|~ζει", _, _;
    subj_act_3: _, _, "~ζωσῐ|~ζωσῐν";
    imperfect_act_1: "~ζον", _, "~ζομεν";
    imperfect_act_2: "~ζες", "~ζετον", "~ζετε";
    imperfect_act_3: "~ζε|~ζεν", "~ζετην", "~ζον";
    imperfect_mid_1: "~ζομην", _, "~ζομεθᾰ";
    imperfect_mid_2: "~ζου", _, _;
    imperfect_mid_3: "~ζετο", _, _;
    imperfect_pass_2: _, "~ζεσθον", "~ζεσθε";
    imperfect_pass_3: _, "~ζεσθην", "~ζοντο";
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", _, "~σειᾰν|~σαιεν";
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: _, "~κᾰτον", _;
    perfect_ind_act_3: _, _, "~κᾱσῐ";
    perfect_ind_mid_1: "~σμαι", _, "~σμεθᾰ";
    perfect_ind_mid_2: "~σαι", "~σθον", _;
    perfect_ind_mid_3: "~σται", _, "~δᾰται";
    perfect_ind_pass_2: _, _, "~σθε";
    perfect_subj_act_1: "~κω", _, _;
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ";
    perfect_opt_act_1: "~κοιμῐ", _, "~κοιμεν";
    perfect_opt_act_2: "~κοις", _, _;
    perfect_opt_act_3: _, "~κοιτην", _;
    perfect_imp_act_2: "~κε", _, "~κετε";
    perfect_imp_act_3: _, _, "~κοντων";
    perfect_imp_mid_2: _, _, "~σθε";
    perfect_imp_mid_3: "~σθω", _, "~σθων";
    perfect_imp_pass_2: "~σο", _, "~σθε";
    perfect_imp_pass_3: _, "~σθων", "~σθων";
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
    // Βαινω: 5 words (βαινω, χαινω, ἐπεκβαινω, εἰσᾰνᾰβαινω, ἐπαλθαινω). Like Νῑκαω, except in 133 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("βαινω", 5, "~αινω", like Νῑκαω;
    stem aorist_ind_pass: "~ᾰθην- like ἐχω";
    stem aorist_pass: "~ᾰθ- like ἐχω";
    stem pluperfect_act: "~εβηκειν- like κᾰλεω";
    ind_act_1: "~αινω", _, "~αινομεν";
    ind_act_2: "~αινεις", _, _;
    ind_act_3: "~αινει", "~αινετον", "~αινουσῐ";
    ind_mid_1: "~αινομαι", _, "~αινομεθᾰ";
    ind_mid_2: "~αινῃ", "~αινεσθον", _;
    ind_pass_2: _, _, "~αινεσθε";
    ind_pass_3: "~αινεται", _, "~αινονται";
    subj_act_1: "~αινω", _, "~αινωμεν";
    subj_act_2: "~αινῃς", _, _;
    subj_act_3: "~αινῃ", "~αινητον", "~αινωσῐ";
    subj_mid_1: _, _, "~αινωμεθᾰ";
    subj_mid_2: _, "~αινησθον", _;
    subj_mid_3: "~αινηται", _, "~αινωνται";
    subj_pass_1: "~αινωμαι", _, _;
    subj_pass_2: _, _, "~αινησθε";
    opt_act_1: "~αινοιμῐ", _, "~αινοιμεν";
    opt_act_2: "~αινοις", "~αινοιτον", derived;
    opt_act_3: "~αινοι", "~αινοιτην", "~αινοιεν";
    opt_mid_1: "~αινοιμην", _, "~αινοιμεθᾰ";
    opt_mid_2: "~αινοιο", "~αινοισθον", _;
    opt_mid_3: "~αινοιτο", "~αινοισθην", "~αινοιντο";
    opt_pass_2: _, _, "~αινοισθε";
    imp_act_2: "~αῑνε", "~αινετον", "~αινετε";
    imp_mid_2: "~αινου", _, "~αινεσθε";
    imp_mid_3: "~αινεσθω", _, "~αινεσθων";
    imp_pass_2: _, _, "~αινεσθε";
    imp_pass_3: _, "~αινεσθων", "~αινεσθων";
    imperfect_act_1: "ἐ~αινον", _, "ἐ~αινομεν";
    imperfect_act_2: "ἐ~αινες", "ἐ~αινετον", "ἐ~αινετε";
    imperfect_act_3: "ἐ~αινε", "ἐ~αινετην", "ἐ~αινον";
    imperfect_mid_1: "ἐ~αινομην", _, "ἐ~αινομεθᾰ";
    imperfect_mid_2: "ἐ~αινου", _, _;
    imperfect_mid_3: "ἐ~αινετο", _, _;
    imperfect_pass_2: _, "ἐ~αινεσθον", "ἐ~αινεσθε";
    imperfect_pass_3: _, "ἐ~αινεσθην", "ἐ~αινοντο";
    aorist_ind_act_1: "ἐ~ην", _, "ἐ~ημεν";
    aorist_ind_act_2: "ἐ~ης", "ἐ~ητον", _;
    aorist_ind_act_3: "ἐ~η", _, _;
    aorist_subj_act_1: "~ω", _, _;
    aorist_subj_act_2: "~ῃς", "~ητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_opt_act_1: "~αιην", _, "~αῑμεν";
    aorist_opt_act_2: "~αιης", _, _;
    aorist_opt_act_3: "~αιη", "~αιτην", "~αῑεν";
    aorist_imp_act_2: "~ηθῐ", _, "~ητε";
    aorist_imp_act_3: _, _, "~ᾰντων";
    perfect_ind_act_1: "~εβηκᾰ", _, "~εβηκᾰμεν";
    perfect_ind_act_2: _, "~εβηκᾰτον", _;
    perfect_ind_act_3: _, _, "~εβηκᾱσῐ";
    perfect_ind_mid_1: "~εβᾰμαι", _, "~εβᾰμεθᾰ";
    perfect_ind_mid_2: "~εβᾰσαι", "~εβᾰσθον", _;
    perfect_ind_mid_3: "~εβᾰται", _, "~εβᾰνται";
    perfect_ind_pass_2: _, _, "~εβᾰσθε";
    perfect_subj_act_1: "~εβηκω", _, _;
    perfect_subj_act_2: "~εβηκῃς", "~εβηκητον", "~εβηκητε";
    perfect_subj_act_3: "~εβηκῃ", _, "~εβηκωσῐ";
    perfect_opt_act_1: "~εβηκοιμῐ", _, "~εβηκοιμεν";
    perfect_opt_act_2: "~εβηκοις", _, _;
    perfect_opt_act_3: _, "~εβηκοιτην", _;
    perfect_imp_act_2: "~εβηκε", _, "~εβηκετε";
    perfect_imp_act_3: _, _, "~εβηκοντων";
    perfect_imp_mid_2: _, _, "~εβᾰσθε";
    perfect_imp_mid_3: "~εβᾰσθω", _, "~εβᾰσθων";
    perfect_imp_pass_2: "~εβᾰσο", _, "~εβᾰσθε";
    perfect_imp_pass_3: _, "~εβᾰσθων", "~εβᾰσθων";
    pluperfect_mid_1: "ἐ~εβᾰμην", _, _;
    pluperfect_mid_2: "ἐ~εβᾰσο", "ἐ~εβᾰσθον", _;
    pluperfect_mid_3: "ἐ~εβᾰτο", _, "ἐ~εβᾰντο";
    pluperfect_pass_1: _, _, "ἐ~εβᾰμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~εβᾰσθε";
    pluperfect_pass_3: _, "ἐ~εβᾰσθην", _;
    infinitive_pres: "~αινειν", _, "~αινεσθαι";
    infinitive_aor: "~ηναι", _, _;
    infinitive_perf: _, "~εβᾰσθαι", _;
    participle_pres: "~αινων", _, "~αινομενος";
    participle_aor: "~ᾱς", _, _;
    participle_perf: "~εβηκως", _, "~εβᾰμενος";
    ),
    // Πρεπω: 5 words (τρεπω, πρεπω, πᾰρεκτρεπω, λεπω, ρρεπω). Like Κᾰλεω, except in 158 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("πρεπω", 5, "~επω", like Κᾰλεω;
    stem future_pass: "~απησ- like ἐχω";
    stem aorist_ind: "~εψ- like ἐχω";
    stem aorist_ind_pass: "~εφθην- like ἐχω";
    stem aorist_pass: "~εφθ- like ἐχω";
    stem pluperfect_act: "~οφειν- like ἐχω";
    ind_act_1: "~επω", _, "~επομεν";
    ind_act_2: "~επεις", _, _;
    ind_act_3: "~επει", "~επετον", "~επουσῐ";
    ind_mid_1: "~επομαι", _, "~επομεθᾰ";
    ind_mid_2: "~επῃ", "~επεσθον", _;
    ind_pass_2: _, _, "~επεσθε";
    ind_pass_3: "~επεται", _, "~επονται";
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
    opt_mid_2: "~εποιο", "~εποισθον", _;
    opt_mid_3: "~εποιτο", "~εποισθην", "~εποιντο";
    opt_pass_2: _, _, "~εποισθε";
    imp_act_2: "~επε", "~επετον", "~επετε";
    imp_mid_2: "~επου", _, "~επεσθε";
    imp_mid_3: "~επεσθω", _, "~επεσθων";
    imp_pass_2: _, _, "~επεσθε";
    imp_pass_3: _, "~επεσθων", "~επεσθων";
    imperfect_act_1: "ἐ~επον", _, "ἐ~επομεν";
    imperfect_act_2: "ἐ~επες", "ἐ~επετον", "ἐ~επετε";
    imperfect_act_3: "ἐ~επε", "ἐ~επετην", "ἐ~επον";
    imperfect_mid_1: "ἐ~επομην", _, "ἐ~επομεθᾰ";
    imperfect_mid_2: "ἐ~επου", _, _;
    imperfect_mid_3: "ἐ~επετο", _, _;
    imperfect_pass_2: _, "ἐ~επεσθον", "ἐ~επεσθε";
    imperfect_pass_3: _, "ἐ~επεσθην", "ἐ~εποντο";
    future_ind_act_1: "~εψω", _, "~εψομεν";
    future_ind_act_2: "~εψεις", "~εψετον", "~εψετε";
    future_ind_act_3: _, _, "~εψουσῐ";
    future_ind_mid_1: "~εψομαι", _, "~εψομεθᾰ";
    future_ind_mid_2: "~εψῃ", "~εψεσθον", _;
    future_ind_mid_3: "~εψεται", _, _;
    future_opt_act_1: "~εψοιμῐ", _, "~εψοιμεν";
    future_opt_act_2: _, "~εψοιτον", "~εψοιτε";
    future_opt_act_3: "~εψοι", "~εψοιτην", "~εψοιεν";
    future_opt_mid_1: "~εψοιμην", _, "~εψοιμεθᾰ";
    future_opt_mid_2: "~εψοιο", _, _;
    future_opt_mid_3: _, "~εψοισθην", _;
    aorist_subj_act_1: "~εψω", _, _;
    aorist_subj_act_2: "~εψῃς", "~εψητον", _;
    aorist_subj_mid_1: "~εψωμαι", _, _;
    aorist_subj_mid_2: "~εψῃ", "~εψησθον", _;
    aorist_opt_act_1: "~εψαιμῐ", _, "~εψαιμεν";
    aorist_opt_act_2: "~εψειᾰς", _, _;
    aorist_opt_act_3: "~εψειε", "~εψαιτην", "~εψειᾰν";
    aorist_opt_mid_1: "~εψαιμην", _, _;
    aorist_opt_mid_2: "~εψαιο", _, _;
    aorist_imp_act_2: "~εψον", _, "~εψᾰτε";
    aorist_imp_act_3: _, _, "~εψᾰντων";
    aorist_imp_mid_2: "~εψαι", "~εψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~εψᾰσθων";
    perfect_ind_act_1: "%ε~οφᾰ", _, "%ε~οφᾰμεν";
    perfect_ind_act_2: _, "%ε~οφᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~οφᾱσῐ";
    perfect_ind_mid_1: "%ε~αμμαι", _, "%ε~αμμεθᾰ";
    perfect_ind_mid_2: "%ε~αψαι", "%ε~αφθον", _;
    perfect_ind_mid_3: "%ε~απται", _, "%ε~αφᾰται";
    perfect_ind_pass_2: _, _, "%ε~αφθε";
    perfect_subj_act_1: "%ε~οφω", _, _;
    perfect_subj_act_2: "%ε~οφῃς", "%ε~οφητον", "%ε~οφητε";
    perfect_subj_act_3: "%ε~οφῃ", _, "%ε~οφωσῐ";
    perfect_opt_act_1: "%ε~οφοιμῐ", _, "%ε~οφοιμεν";
    perfect_opt_act_2: "%ε~οφοις", _, _;
    perfect_opt_act_3: _, "%ε~οφοιτην", _;
    perfect_imp_act_2: "%ε~οφε", _, "%ε~οφετε";
    perfect_imp_act_3: _, _, "%ε~οφοντων";
    perfect_imp_mid_2: _, _, "%ε~αφθε";
    perfect_imp_mid_3: "%ε~αφθω", _, "%ε~αφθων";
    perfect_imp_pass_2: "%ε~αψο", _, "%ε~αφθε";
    perfect_imp_pass_3: _, "%ε~αφθων", "%ε~αφθων";
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
    // Στερεω: 5 words (στερεω, σῐδηροφορεω, κορεω, σειρεω, γεωμετρεω). Like Δῐκεω, except in 61 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 pl, ….
    conjugation!("στερεω", 5, "~εω", like Δῐκεω;
    stem future_pass: "~ησ- like ἐχω";
    stem aorist_ind: "~ησ- like ἐχω";
    stem pluperfect_act: "~ηκειν- like κᾰλεω";
    future_ind_act_1: "~ω", _, "~οῡμεν";
    future_ind_act_2: "~εῑς", "~εῑτον", "~εῑτε";
    future_ind_act_3: _, _, "~οῡσῐ";
    future_ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    future_ind_mid_2: "~εῑ", "~εῑσθον", _;
    future_ind_mid_3: "~εῑται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~οιην", _, "~οῑμεν";
    future_opt_act_2: _, "~οῑτον", "~οῑτε";
    future_opt_act_3: "~οιη", "~οιτην", "~οῑεν";
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_2: "~οῑο", _, _;
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
    perfect_opt_act_1: "ἐ~ηκοιμῐ", _, _;
    perfect_opt_act_2: "ἐ~ηκοις", _, _;
    perfect_opt_act_3: derived, _, _;
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
    infinitive_fut: "~εῑν", _, _;
    infinitive_futp: _, _, "ἐ~ησεσθαι";
    participle_futp: _, _, "ἐ~ησομενος";
    ),
    // Μᾰρτῠρεω: 5 words (μᾰρτῠρεω, λοιδορεω, μᾰραυγεω, δονεω, κερτομεω). Like Ποιεω, except in 19 cells: future_perfect_ind_mid_1 sg, future_perfect_ind_mid_3 du, future_perfect_ind_pass_1 pl, future_perfect_ind_pass_2 sg, future_perfect_ind_pass_2 du, future_perfect_ind_pass_2 pl, ….
    conjugation!("μᾰρτῠρεω", 5, "~εω", like Ποιεω;
    stem aorist_ind_pass: "~ηθην- like ἐλπω";
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
    // Ἐλπῐζω: 5 words (ἐγγῐζω, ἐλπῐζω, ἐρεθῐζω, ἐνᾰρῐζω, ἐρῐζω). Like Χρημᾰτῐζω, except in 165 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐλπῐζω", 5, "ἐ~ζω", like Χρημᾰτῐζω;
    stem future_pass: "~σθησ- like ἐλπῐζω";
    stem aorist_ind: "~σ- like ἐλπῐζω";
    stem aorist_ind_pass: "~σθην- like ἀγορευω";
    stem aorist_pass: "~σθ- like ἐλπῐζω";
    ind_act_1: "ἐ~ζω", _, "ἐ~ζομεν";
    ind_act_2: "ἐ~ζεις", _, _;
    ind_act_3: "ἐ~ζει", "ἐ~ζετον", "ἐ~ζουσῐ|ἐ~ζουσῐν";
    ind_mid_1: "ἐ~ζομαι", _, "ἐ~ζομεθᾰ";
    ind_mid_2: "ἐ~ζῃ|ἐ~ζει", "ἐ~ζεσθον", _;
    ind_pass_2: _, _, "ἐ~ζεσθε";
    ind_pass_3: "ἐ~ζεται", _, "ἐ~ζονται";
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
    opt_mid_2: "ἐ~ζοιο", "ἐ~ζοισθον", _;
    opt_mid_3: "ἐ~ζοιτο", "ἐ~ζοισθην", "ἐ~ζοιντο";
    opt_pass_2: _, _, "ἐ~ζοισθε";
    imp_act_2: "ἐ~ζε", "ἐ~ζετον", "ἐ~ζετε";
    imp_mid_2: "ἐ~ζου", _, "ἐ~ζεσθε";
    imp_mid_3: "ἐ~ζεσθω", _, "ἐ~ζεσθων";
    imp_pass_2: _, _, "ἐ~ζεσθε";
    imp_pass_3: _, "ἐ~ζεσθων", "ἐ~ζεσθων";
    imperfect_act_1: "ἠ~ζον", _, "ἠ~ζομεν";
    imperfect_act_2: "ἠ~ζες", "ἠ~ζετον", "ἠ~ζετε";
    imperfect_act_3: "ἠ~ζε|ἠ~ζεν", "ἠ~ζετην", "ἠ~ζον";
    imperfect_mid_1: "ἠ~ζομην", _, "ἠ~ζομεθᾰ";
    imperfect_mid_2: "ἠ~ζου", _, _;
    imperfect_mid_3: "ἠ~ζετο", _, _;
    imperfect_pass_2: _, "ἠ~ζεσθον", "ἠ~ζεσθε";
    imperfect_pass_3: _, "ἠ~ζεσθην", "ἠ~ζοντο";
    future_ind_act_1: "ἐ~ω", _, "ἐ~οῡμεν";
    future_ind_act_2: "ἐ~εῑς", "ἐ~εῑτον", "ἐ~εῑτε";
    future_ind_act_3: _, _, "ἐ~οῡσῐ|ἐ~οῡσῐν";
    future_ind_mid_1: "ἐ~οῡμαι", _, "ἐ~ουμεθᾰ";
    future_ind_mid_2: "ἐ~εῑ|ἐ~ῃ", "ἐ~εῑσθον", _;
    future_ind_mid_3: "ἐ~εῑται", _, _;
    future_opt_act_1: "ἐ~οιην|ἐ~οῑμῐ", _, "ἐ~οῑμεν|ἐ~οιημεν";
    future_opt_act_2: "ἐ~οιης|ἐ~οῑς", "ἐ~οῑτον|ἐ~οιητον", "ἐ~οῑτε|ἐ~οιητε";
    future_opt_act_3: "ἐ~οιη|ἐ~οῑ", "ἐ~οιτην|ἐ~οιητην", "ἐ~οῑεν|ἐ~οιησᾰν";
    future_opt_mid_1: "ἐ~οιμην", _, "ἐ~οιμεθᾰ";
    future_opt_mid_2: "ἐ~οῑο", _, _;
    future_opt_mid_3: _, "ἐ~οισθην", _;
    aorist_subj_act_1: "ἐ~σω", _, _;
    aorist_subj_act_2: "ἐ~σῃς", "ἐ~σητον", _;
    aorist_subj_act_3: _, _, "ἐ~σωσῐ|ἐ~σωσῐν";
    aorist_subj_mid_1: "ἐ~σωμαι", _, _;
    aorist_subj_mid_2: "ἐ~σῃ", "ἐ~σησθον", _;
    aorist_opt_act_1: "ἐ~σαιμῐ", _, "ἐ~σαιμεν";
    aorist_opt_act_2: "ἐ~σειᾰς|ἐ~σαις", _, _;
    aorist_opt_act_3: "ἐ~σειε|ἐ~σειεν|ἐ~σαι", "ἐ~σαιτην", "ἐ~σειᾰν|ἐ~σαιεν";
    aorist_opt_mid_1: "ἐ~σαιμην", _, _;
    aorist_opt_mid_2: "ἐ~σαιο", _, _;
    aorist_imp_act_2: "ἐ~σον", _, "ἐ~σᾰτε";
    aorist_imp_act_3: _, _, "ἐ~σᾰντων";
    aorist_imp_mid_2: "ἐ~σαι", "ἐ~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἐ~σᾰσθων";
    perfect_ind_act_1: "ἠ~κᾰ", _, "ἠ~κᾰμεν";
    perfect_ind_act_2: _, "ἠ~κᾰτον", _;
    perfect_ind_act_3: "ἠ~κε|ἠ~κεν", _, "ἠ~κᾱσῐ|ἠ~κᾱσῐν";
    perfect_ind_mid_1: "ἠ~σμαι", _, "ἠ~σμεθᾰ";
    perfect_ind_mid_2: "ἠ~σαι", "ἠ~σθον", _;
    perfect_ind_mid_3: "ἠ~σται", _, "ἠ~δᾰται";
    perfect_ind_pass_2: _, _, "ἠ~σθε";
    perfect_subj_act_1: "ἠ~κω", _, _;
    perfect_subj_act_2: "ἠ~κῃς", "ἠ~κητον", "ἠ~κητε";
    perfect_subj_act_3: "ἠ~κῃ", _, "ἠ~κωσῐ|ἠ~κωσῐν";
    perfect_opt_act_1: "ἠ~κοιμῐ|ἠ~κοιην", _, "ἠ~κοιμεν";
    perfect_opt_act_2: "ἠ~κοις|ἠ~κοιης", _, _;
    perfect_opt_act_3: "ἠ~κοι|ἠ~κοιη", "ἠ~κοιτην", _;
    perfect_imp_act_2: "ἠ~κε", _, "ἠ~κετε";
    perfect_imp_act_3: _, _, "ἠ~κοντων";
    perfect_imp_mid_2: _, _, "ἠ~σθε";
    perfect_imp_mid_3: "ἠ~σθω", _, "ἠ~σθων";
    perfect_imp_pass_2: "ἠ~σο", _, "ἠ~σθε";
    perfect_imp_pass_3: _, "ἠ~σθων", "ἠ~σθων";
    pluperfect_act_1: "ἠ~κειν|ἠ~κη", _, _;
    pluperfect_act_2: "ἠ~κεις|ἠ~κης", _, _;
    pluperfect_act_3: "ἠ~κει|ἠ~κειν", _, _;
    pluperfect_mid_1: "ἠ~σμην", _, _;
    pluperfect_mid_2: "ἠ~σο", "ἠ~σθον", _;
    pluperfect_mid_3: "ἠ~στο", _, "ἠ~δᾰτο";
    pluperfect_pass_1: _, _, "ἠ~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἠ~σθε";
    pluperfect_pass_3: _, "ἠ~σθην", _;
    infinitive_pres: "ἐ~ζειν", _, "ἐ~ζεσθαι";
    infinitive_fut: "ἐ~εῑν", _, _;
    infinitive_aor: "ἐ~σαι", _, _;
    infinitive_perf: _, "ἠ~σθαι", _;
    participle_pres: "ἐ~ζων", _, "ἐ~ζομενος";
    participle_aor: "ἐ~σᾱς", "ἐ~σᾰμενος", _;
    participle_perf: "ἠ~κως", _, "ἠ~σμενος";
    ),
    // Σῠνοικῐζω: 5 words (ἀνοικῐζω, κᾰτοικῐζω, σῠνοικῐζω, ξῠνοικῐζω, ἀποικῐζω). Like Πεμπω, except in 162 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("σῠνοικῐζω", 5, "~οι~ζω", like Πεμπω;
    stem future_pass: "~οι~σθησ- like ποιεω";
    stem aorist_ind: "~ῳ~σ- like εὐλογεω";
    stem aorist_ind_pass: "~ῳ~σθην- like ἐλπω";
    stem aorist_pass: "~οι~σθ- like ποιεω";
    stem pluperfect_act: "~ῳ~κ- like εὐλογεω";
    ind_act_1: "~οι~ζω", _, "~οι~ζομεν";
    ind_act_2: "~οι~ζεις", _, _;
    ind_act_3: "~οι~ζει", "~οι~ζετον", "~οι~ζουσῐ|~οι~ζουσῐν";
    ind_mid_1: "~οι~ζομαι", _, "~οι~ζομεθᾰ";
    ind_mid_2: "~οι~ζῃ|~οι~ζει", "~οι~ζεσθον", _;
    ind_pass_2: _, _, "~οι~ζεσθε";
    ind_pass_3: "~οι~ζεται", _, "~οι~ζονται";
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
    opt_mid_2: "~οι~ζοιο", "~οι~ζοισθον", _;
    opt_mid_3: "~οι~ζοιτο", "~οι~ζοισθην", "~οι~ζοιντο";
    opt_pass_2: _, _, "~οι~ζοισθε";
    imp_act_2: "~οι~ζε", "~οι~ζετον", "~οι~ζετε";
    imp_mid_2: "~οι~ζου", _, "~οι~ζεσθε";
    imp_mid_3: "~οι~ζεσθω", _, "~οι~ζεσθων";
    imp_pass_2: _, _, "~οι~ζεσθε";
    imp_pass_3: _, "~οι~ζεσθων", "~οι~ζεσθων";
    imperfect_act_1: "~ῳ~ζον", _, "~ῳ~ζομεν";
    imperfect_act_2: "~ῳ~ζες", "~ῳ~ζετον", "~ῳ~ζετε";
    imperfect_act_3: "~ῳ~ζε|~ῳ~ζεν", "~ῳ~ζετην", "~ῳ~ζον";
    imperfect_mid_1: "~ῳ~ζομην", _, "~ῳ~ζομεθᾰ";
    imperfect_mid_2: "~ῳ~ζου", _, _;
    imperfect_mid_3: "~ῳ~ζετο", _, _;
    imperfect_pass_2: _, "~ῳ~ζεσθον", "~ῳ~ζεσθε";
    imperfect_pass_3: _, "~ῳ~ζεσθην", "~ῳ~ζοντο";
    future_ind_act_1: "~οι~ω", _, "~οι~οῡμεν";
    future_ind_act_2: "~οι~εῑς", "~οι~εῑτον", "~οι~εῑτε";
    future_ind_act_3: _, _, "~οι~οῡσῐ|~οι~οῡσῐν";
    future_ind_mid_1: "~οι~οῡμαι", _, "~οι~ουμεθᾰ";
    future_ind_mid_2: "~οι~εῑ|~οι~ῃ", "~οι~εῑσθον", _;
    future_ind_mid_3: "~οι~εῑται", _, _;
    future_opt_act_1: "~οι~οιην|~οι~οῑμῐ", _, "~οι~οῑμεν|~οι~οιημεν";
    future_opt_act_2: "~οι~οιης|~οι~οῑς", "~οι~οῑτον|~οι~οιητον", "~οι~οῑτε|~οι~οιητε";
    future_opt_act_3: "~οι~οιη|~οι~οῑ", "~οι~οιτην|~οι~οιητην", "~οι~οῑεν|~οι~οιησᾰν";
    future_opt_mid_1: "~οι~οιμην", _, "~οι~οιμεθᾰ";
    future_opt_mid_2: "~οι~οῑο", _, _;
    future_opt_mid_3: _, "~οι~οισθην", _;
    aorist_subj_act_1: "~οι~σω", _, _;
    aorist_subj_act_2: "~οι~σῃς", "~οι~σητον", _;
    aorist_subj_act_3: _, _, "~οι~σωσῐ|~οι~σωσῐν";
    aorist_subj_mid_1: "~οι~σωμαι", _, _;
    aorist_subj_mid_2: "~οι~σῃ", "~οι~σησθον", _;
    aorist_opt_act_1: "~οι~σαιμῐ", _, "~οι~σαιμεν";
    aorist_opt_act_2: "~οι~σειᾰς|~οι~σαις", _, _;
    aorist_opt_act_3: "~οι~σειε|~οι~σειεν|~οι~σαι", "~οι~σαιτην", "~οι~σειᾰν|~οι~σαιεν";
    aorist_opt_mid_1: "~οι~σαιμην", _, _;
    aorist_opt_mid_2: "~οι~σαιο", _, _;
    aorist_imp_act_2: "~οι~σον", _, "~οι~σᾰτε";
    aorist_imp_act_3: _, _, "~οι~σᾰντων";
    aorist_imp_mid_2: "~οι~σαι", "~οι~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~οι~σᾰσθων";
    perfect_ind_act_1: "~ῳ~κᾰ", _, "~ῳ~κᾰμεν";
    perfect_ind_act_2: _, "~ῳ~κᾰτον", _;
    perfect_ind_act_3: "~ῳ~κε|~ῳ~κεν", _, "~ῳ~κᾱσῐ|~ῳ~κᾱσῐν";
    perfect_ind_mid_1: "~ῳ~σμαι", _, "~ῳ~σμεθᾰ";
    perfect_ind_mid_2: "~ῳ~σαι", "~ῳ~σθον", _;
    perfect_ind_mid_3: "~ῳ~σται", _, "~ῳ~δᾰται";
    perfect_ind_pass_2: _, _, "~ῳ~σθε";
    perfect_subj_act_1: "~ῳ~κω", _, _;
    perfect_subj_act_2: "~ῳ~κῃς", "~ῳ~κητον", "~ῳ~κητε";
    perfect_subj_act_3: "~ῳ~κῃ", _, "~ῳ~κωσῐ|~ῳ~κωσῐν";
    perfect_opt_act_1: "~ῳ~κοιμῐ|~ῳ~κοιην", _, "~ῳ~κοιμεν";
    perfect_opt_act_2: "~ῳ~κοις|~ῳ~κοιης", _, _;
    perfect_opt_act_3: "~ῳ~κοι|~ῳ~κοιη", "~ῳ~κοιτην", _;
    perfect_imp_act_2: "~ῳ~κε", _, "~ῳ~κετε";
    perfect_imp_act_3: _, _, "~ῳ~κοντων";
    perfect_imp_mid_2: _, _, "~ῳ~σθε";
    perfect_imp_mid_3: "~ῳ~σθω", _, "~ῳ~σθων";
    perfect_imp_pass_2: "~ῳ~σο", _, "~ῳ~σθε";
    perfect_imp_pass_3: _, "~ῳ~σθων", "~ῳ~σθων";
    pluperfect_mid_1: "~ῳ~σμην", _, _;
    pluperfect_mid_2: "~ῳ~σο", "~ῳ~σθον", _;
    pluperfect_mid_3: "~ῳ~στο", _, "~ῳ~δᾰτο";
    pluperfect_pass_1: _, _, "~ῳ~σμεθᾰ";
    pluperfect_pass_2: _, _, "~ῳ~σθε";
    pluperfect_pass_3: _, "~ῳ~σθην", _;
    infinitive_pres: "~οι~ζειν", _, "~οι~ζεσθαι";
    infinitive_fut: "~οι~εῑν", _, _;
    infinitive_aor: "~οι~σαι", _, _;
    infinitive_perf: _, "~ῳ~σθαι", _;
    participle_pres: "~οι~ζων", _, "~οι~ζομενος";
    participle_aor: "~οι~σᾱς", "~οι~σᾰμενος", _;
    participle_perf: "~ῳ~κως", _, "~ῳ~σμενος";
    ),
    // Θρῠπτω: 5 words (βᾰπτω, θρῠπτω, γνᾰμπτω, μᾰρπτω, δρῠπτω). Like Ἁπτω, except in 72 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_3 pl, future_ind_mid_2 sg, ….
    conjugation!("θρῠπτω", 5, "~πτω", like Ἁπτω;
    stem future_pass: "~φησ- like ποιεω";
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
    perfect_ind_mid_1: "%ε~μμαι", _, "%ε~μμεθᾰ";
    perfect_ind_mid_2: "%ε~ψαι", "%ε~φθον", _;
    perfect_ind_mid_3: "%ε~πται", _, "%ε~φᾰται";
    perfect_ind_pass_2: _, _, "%ε~φθε";
    perfect_subj_act_1: "%ε~πτω", _, _;
    perfect_subj_act_2: "%ε~πτῃς", "%ε~πτητον", "%ε~πτητε";
    perfect_subj_act_3: "%ε~πτῃ", _, "%ε~πτωσῐ";
    perfect_opt_act_1: "%ε~πτοιμῐ", _, "%ε~πτοιμεν";
    perfect_opt_act_2: "%ε~πτοις", _, _;
    perfect_opt_act_3: _, "%ε~πτοιτην", _;
    perfect_imp_act_2: "%ε~πτε", _, "%ε~πτετε";
    perfect_imp_act_3: _, _, "%ε~πτοντων";
    perfect_imp_mid_2: _, _, "%ε~φθε";
    perfect_imp_mid_3: "%ε~φθω", _, "%ε~φθων";
    perfect_imp_pass_2: "%ε~ψο", _, "%ε~φθε";
    perfect_imp_pass_3: _, "%ε~φθων", "%ε~φθων";
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
    // Μωραινω: 5 words (μιαινω, κερδαινω, μελαινω, εὐφραινω, μωραινω). Like Βαινω, except in 94 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imp_act_2 sg, imperfect_act_3 sg, future_ind_act_1 sg, ….
    conjugation!("μωραινω", 5, "~αινω", like Βαινω;
    stem future_pass: "~ανθησ- like ποιεω";
    stem aorist_ind: "~ᾱν- like ἐχω";
    stem aorist_ind_pass: "~ανθην- like ἐχω";
    stem aorist_pass: "~ανθ- like ἐχω";
    ind_act_3: _, _, "~αινουσῐ|~αινουσῐν";
    ind_mid_2: "~αινῃ|~αινει", _, _;
    subj_act_3: _, _, "~αινωσῐ|~αινωσῐν";
    imp_act_2: "~αινε", _, _;
    imperfect_act_3: "ἐ~αινε|ἐ~αινεν", _, _;
    future_ind_act_1: "~ανω", _, "~ανοῡμεν";
    future_ind_act_2: "~ανεῑς", "~ανεῑτον", "~ανεῑτε";
    future_ind_act_3: _, _, "~ανοῡσῐ";
    future_ind_mid_1: "~ανοῡμαι", _, "~ανουμεθᾰ";
    future_ind_mid_2: "~ανεῑ", "~ανεῑσθον", _;
    future_ind_mid_3: "~ανεῑται", _, _;
    future_opt_act_1: "~ανοιην", _, "~ανοῑμεν";
    future_opt_act_2: _, "~ανοῑτον", "~ανοῑτε";
    future_opt_act_3: "~ανοιη", "~ανοιτην", "~ανοῑεν";
    future_opt_mid_1: "~ανοιμην", _, "~ανοιμεθᾰ";
    future_opt_mid_2: "~ανοῑο", _, _;
    future_opt_mid_3: _, "~ανοισθην", _;
    aorist_subj_act_1: "~ᾱνω", _, _;
    aorist_subj_act_2: "~ᾱνῃς", "~ᾱνητον", _;
    aorist_subj_mid_1: "~ᾱνωμαι", _, _;
    aorist_subj_mid_2: "~ᾱνῃ", "~ᾱνησθον", _;
    aorist_opt_act_1: "~ᾱναιμῐ", _, "~ᾱναιμεν";
    aorist_opt_act_2: "~ᾱνειᾰς", _, _;
    aorist_opt_act_3: "~ᾱνειε", "~ᾱναιτην", "~ᾱνειᾰν";
    aorist_opt_mid_1: "~ᾱναιμην", _, _;
    aorist_opt_mid_2: "~ᾱναιο", _, _;
    aorist_imp_act_2: "~ᾱνον", _, "~ᾱνᾰτε";
    aorist_imp_act_3: _, _, "~ᾱνᾰντων";
    aorist_imp_mid_2: "~ᾱναι", "~ᾱνᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾱνᾰσθων";
    perfect_ind_act_1: "%ε~ᾰγκᾰ", _, "%ε~ᾰγκᾰμεν";
    perfect_ind_act_2: _, "%ε~ᾰγκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ᾰγκᾱσῐ";
    perfect_ind_mid_1: "%ε~ᾰσμαι", _, "%ε~ᾰσμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾰνσαι", "%ε~ᾰνθον", _;
    perfect_ind_mid_3: "%ε~ᾰνται", _, "%ε~ᾰνᾰται";
    perfect_ind_pass_2: _, _, "%ε~ᾰνθε";
    perfect_subj_act_1: "%ε~ᾰγκω", _, _;
    perfect_subj_act_2: "%ε~ᾰγκῃς", "%ε~ᾰγκητον", "%ε~ᾰγκητε";
    perfect_subj_act_3: "%ε~ᾰγκῃ", _, "%ε~ᾰγκωσῐ";
    perfect_opt_act_1: "%ε~ᾰγκοιμῐ", _, "%ε~ᾰγκοιμεν";
    perfect_opt_act_2: "%ε~ᾰγκοις", _, _;
    perfect_opt_act_3: _, "%ε~ᾰγκοιτην", _;
    perfect_imp_act_2: "%ε~ᾰγκε", _, "%ε~ᾰγκετε";
    perfect_imp_act_3: _, _, "%ε~ᾰγκοντων";
    perfect_imp_mid_2: _, _, "%ε~ᾰνθε";
    perfect_imp_mid_3: "%ε~ᾰνθω", _, "%ε~ᾰνθων";
    perfect_imp_pass_2: "%ε~ᾰνσο", _, "%ε~ᾰνθε";
    perfect_imp_pass_3: _, "%ε~ᾰνθων", "%ε~ᾰνθων";
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_fut: "~ανεῑν", _, _;
    infinitive_aor: "~ᾱναι", _, _;
    infinitive_perf: _, "%ε~ᾰνθαι", _;
    participle_aor: "~ᾱνᾱς", "~ᾱνᾰμενος", _;
    participle_perf: "%ε~ᾰγκως", _, "%ε~ᾰσμενος";
    ),
    // Ἰᾱτρευω: 5 words (ἰᾱτρευω, ἰαυω, ἡγεμονευω, ἱκετευω, ἱππευω). Like Παυω, except in 70 cells: imp_act_2 sg, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, ….
    conjugation!("ἰᾱτρευω", 5, "~υω", like Παυω;
    stem aorist_ind: "~υσ- like εὐλογεω";
    imp_act_2: "~υε", _, _;
    imperfect_act_1: "~υον", _, "~υομεν";
    imperfect_act_2: "~υες", "~υετον", "~υετε";
    imperfect_act_3: "~υε|~υεν", "~υετην", "~υον";
    imperfect_mid_1: "~υομην", _, "~υομεθᾰ";
    imperfect_mid_2: "~υου", _, _;
    imperfect_mid_3: "~υετο", _, _;
    imperfect_pass_2: _, "~υεσθον", "~υεσθε";
    imperfect_pass_3: _, "~υεσθην", "~υοντο";
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_imp_act_2: "~υσον", _, _;
    aorist_imp_mid_2: "~υσαι", _, _;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: "~υκᾰ", _, "~υκᾰμεν";
    perfect_ind_act_2: _, "~υκᾰτον", _;
    perfect_ind_act_3: _, _, "~υκᾱσῐ";
    perfect_ind_mid_1: "~υμαι", _, "~υμεθᾰ";
    perfect_ind_mid_2: "~υσαι", "~υσθον", _;
    perfect_ind_mid_3: "~υται", _, "~υνται";
    perfect_ind_pass_2: _, _, "~υσθε";
    perfect_subj_act_1: "~υκω", _, _;
    perfect_subj_act_2: "~υκῃς", "~υκητον", "~υκητε";
    perfect_subj_act_3: "~υκῃ", _, "~υκωσῐ";
    perfect_opt_act_1: "~υκοιμῐ", _, "~υκοιμεν";
    perfect_opt_act_2: "~υκοις", _, _;
    perfect_opt_act_3: _, "~υκοιτην", _;
    perfect_imp_act_2: "~υκε", _, "~υκετε";
    perfect_imp_act_3: _, _, "~υκοντων";
    perfect_imp_mid_2: _, _, "~υσθε";
    perfect_imp_mid_3: "~υσθω", _, "~υσθων";
    perfect_imp_pass_2: "~υσο", _, "~υσθε";
    perfect_imp_pass_3: _, "~υσθων", "~υσθων";
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: _, "~ῡσθαι", _;
    participle_perf: "~υκως", _, "~υμενος";
    ),
    // Ῥηγνῡμῐ: 5 words (ῥηγνῡμῐ, πηγνῡμῐ, ξῠρρηγνῡμῐ, ἐμπηγνῡμῐ, ρρηγνῡμῐ). Like Μῑγνῡμῐ, except in 158 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ῥηγνῡμῐ", 5, "~ηγνῡμῐ", like Μῑγνῡμῐ;
    stem aorist_ind: "~ηξ- like ἐχω";
    stem aorist_ind_pass: "~ᾰγην- like ἐχω";
    stem aorist_pass: "~ᾰγ- like σφᾰλλω";
    stem pluperfect_act: "~ωγειν- like κᾰλεω";
    ind_act_1: "~ηγνῡμῐ", _, "~ηγνῠμεν";
    ind_act_2: "~ηγνῡς", _, _;
    ind_act_3: "~ηγνῡσῐ", "~ηγνῠτον", "~ηγνῠᾱσῐ";
    ind_mid_1: "~ηγνῠμαι", _, "~ηγνῠμεθᾰ";
    ind_mid_2: "~ηγνῠσαι", "~ηγνῠσθον", _;
    ind_pass_2: _, _, "~ηγνῠσθε";
    ind_pass_3: "~ηγνῠται", _, "~ηγνῠνται";
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
    opt_mid_2: "~ηγνῠοιο", "~ηγνῠοισθον", _;
    opt_mid_3: "~ηγνῠοιτο", "~ηγνῠοισθην", "~ηγνῠοιντο";
    opt_pass_2: _, _, "~ηγνῠοισθε";
    imp_act_2: "~ηγνῡ", "~ηγνῠτον", "~ηγνῠτε";
    imp_mid_2: "~ηγνῠσο", _, "~ηγνῠσθε";
    imp_mid_3: "~ηγνῠσθω", _, "~ηγνῠσθων";
    imp_pass_2: _, _, "~ηγνῠσθε";
    imp_pass_3: _, "~ηγνῠσθων", "~ηγνῠσθων";
    imperfect_act_1: "ἐ~ηγνῡν", _, "ἐ~ηγνῠμεν";
    imperfect_act_2: "ἐ~ηγνῡς", "ἐ~ηγνῠτον", "ἐ~ηγνῠτε";
    imperfect_act_3: "ἐ~ηγνῡ", "ἐ~ηγνῠτην", "ἐ~ηγνῠσᾰν";
    imperfect_mid_1: "ἐ~ηγνῠμην", _, "ἐ~ηγνῠμεθᾰ";
    imperfect_mid_2: "ἐ~ηγνῠσο", _, _;
    imperfect_mid_3: "ἐ~ηγνῠτο", _, _;
    imperfect_pass_2: _, "ἐ~ηγνῠσθον", "ἐ~ηγνῠσθε";
    imperfect_pass_3: _, "ἐ~ηγνῠσθην", "ἐ~ηγνῠντο";
    future_ind_act_1: "~ηξω", _, "~ηξομεν";
    future_ind_act_2: "~ηξεις", "~ηξετον", "~ηξετε";
    future_ind_act_3: _, _, "~ηξουσῐ";
    future_ind_mid_1: "~ηξομαι", _, "~ηξομεθᾰ";
    future_ind_mid_2: "~ηξῃ", "~ηξεσθον", _;
    future_ind_mid_3: "~ηξεται", _, _;
    future_opt_act_1: "~ηξοιμῐ", _, "~ηξοιμεν";
    future_opt_act_2: _, "~ηξοιτον", "~ηξοιτε";
    future_opt_act_3: "~ηξοι", "~ηξοιτην", "~ηξοιεν";
    future_opt_mid_1: "~ηξοιμην", _, "~ηξοιμεθᾰ";
    future_opt_mid_2: "~ηξοιο", _, _;
    future_opt_mid_3: _, "~ηξοισθην", _;
    aorist_subj_act_1: "~ηξω", _, _;
    aorist_subj_act_2: "~ηξῃς", "~ηξητον", _;
    aorist_subj_mid_1: "~ηξωμαι", _, _;
    aorist_subj_mid_2: "~ηξῃ", "~ηξησθον", _;
    aorist_opt_act_1: "~ηξαιμῐ", _, "~ηξαιμεν";
    aorist_opt_act_2: "~ηξειᾰς", _, _;
    aorist_opt_act_3: "~ηξειε", "~ηξαιτην", "~ηξειᾰν";
    aorist_opt_mid_1: "~ηξαιμην", _, _;
    aorist_opt_mid_2: "~ηξαιο", _, _;
    aorist_imp_act_2: "~ηξον", _, "~ηξᾰτε";
    aorist_imp_act_3: _, _, "~ηξᾰντων";
    aorist_imp_mid_2: "~ηξαι", "~ηξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ηξᾰσθων";
    perfect_ind_act_1: "ἐ~ωγᾰ", _, "ἐ~ωγᾰμεν";
    perfect_ind_act_2: _, "ἐ~ωγᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~ωγᾱσῐ";
    perfect_ind_mid_1: "~ηρρηγμαι", _, "~ηρρηγμεθᾰ";
    perfect_ind_mid_2: "~ηρρηξαι", "~ηρρηχθον", _;
    perfect_ind_mid_3: "~ηρρηκται", _, "~ηρρηγᾰται";
    perfect_ind_pass_2: _, _, "~ηρρηχθε";
    perfect_subj_act_1: "ἐ~ωγω", _, _;
    perfect_subj_act_2: "ἐ~ωγῃς", "ἐ~ωγητον", "ἐ~ωγητε";
    perfect_subj_act_3: "ἐ~ωγῃ", _, "ἐ~ωγωσῐ";
    perfect_opt_act_1: "ἐ~ωγοιμῐ", _, "ἐ~ωγοιμεν";
    perfect_opt_act_2: "ἐ~ωγοις", _, _;
    perfect_opt_act_3: _, "ἐ~ωγοιτην", _;
    perfect_imp_act_2: "ἐ~ωγε", _, "ἐ~ωγετε";
    perfect_imp_act_3: _, _, "ἐ~ωγοντων";
    perfect_imp_mid_2: _, _, "~ηρρηχθε";
    perfect_imp_mid_3: "~ηρρηχθω", _, "~ηρρηχθων";
    perfect_imp_pass_2: "~ηρρηξο", _, "~ηρρηχθε";
    perfect_imp_pass_3: _, "~ηρρηχθων", "~ηρρηχθων";
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
    // Τρεφω: 5 words (νεμω, τρεφω, δερω, δεμω, ξῠμμενω). Like Ἀειρω, except in 112 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("τρεφω", 5, "~ε~ω", like Ἀειρω;
    stem future_pass: "~ᾰ~ησ- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~ην- like ἐχω";
    stem aorist_pass: "~ᾰ~- like σφᾰλλω";
    stem pluperfect_act: "~εδᾰ~κειν- like κᾰλεω";
    ind_act_1: "~ε~ω", _, "~ε~ομεν";
    ind_act_2: "~ε~εις", _, _;
    ind_act_3: "~ε~ει", "~ε~ετον", "~ε~ουσῐ";
    ind_mid_1: "~ε~ομαι", _, "~ε~ομεθᾰ";
    ind_mid_2: "~ε~ῃ", "~ε~εσθον", _;
    ind_pass_2: _, _, "~ε~εσθε";
    ind_pass_3: "~ε~εται", _, "~ε~ονται";
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
    opt_mid_2: "~ε~οιο", "~ε~οισθον", _;
    opt_mid_3: "~ε~οιτο", "~ε~οισθην", "~ε~οιντο";
    opt_pass_2: _, _, "~ε~οισθε";
    imp_act_2: "~ε~ε", "~ε~ετον", "~ε~ετε";
    imp_mid_2: "~ε~ου", _, "~ε~εσθε";
    imp_mid_3: "~ε~εσθω", _, "~ε~εσθων";
    imp_pass_2: _, _, "~ε~εσθε";
    imp_pass_3: _, "~ε~εσθων", "~ε~εσθων";
    imperfect_act_1: "ἐ~ε~ον", _, "ἐ~ε~ομεν";
    imperfect_act_2: "ἐ~ε~ες", "ἐ~ε~ετον", "ἐ~ε~ετε";
    imperfect_act_3: "ἐ~ε~ε", "ἐ~ε~ετην", "ἐ~ε~ον";
    imperfect_mid_1: "ἐ~ε~ομην", _, "ἐ~ε~ομεθᾰ";
    imperfect_mid_2: "ἐ~ε~ου", _, _;
    imperfect_mid_3: "ἐ~ε~ετο", _, _;
    imperfect_pass_2: _, "ἐ~ε~εσθον", "ἐ~ε~εσθε";
    imperfect_pass_3: _, "ἐ~ε~εσθην", "ἐ~ε~οντο";
    perfect_ind_act_1: "~εδᾰ~κᾰ", _, "~εδᾰ~κᾰμεν";
    perfect_ind_act_2: _, "~εδᾰ~κᾰτον", _;
    perfect_ind_act_3: _, _, "~εδᾰ~κᾱσῐ";
    perfect_ind_mid_1: "~εδᾰ~μαι", _, "~εδᾰ~μεθᾰ";
    perfect_ind_mid_2: "~εδᾰ~σαι", "~εδᾰ~θον", _;
    perfect_ind_mid_3: "~εδᾰ~ται", _, "~εδᾰ~ᾰται";
    perfect_ind_pass_2: _, _, "~εδᾰ~θε";
    perfect_subj_act_1: "~εδᾰ~κω", _, _;
    perfect_subj_act_2: "~εδᾰ~κῃς", "~εδᾰ~κητον", "~εδᾰ~κητε";
    perfect_subj_act_3: "~εδᾰ~κῃ", _, "~εδᾰ~κωσῐ";
    perfect_opt_act_1: "~εδᾰ~κοιμῐ", _, "~εδᾰ~κοιμεν";
    perfect_opt_act_2: "~εδᾰ~κοις", _, _;
    perfect_opt_act_3: _, "~εδᾰ~κοιτην", _;
    perfect_imp_act_2: "~εδᾰ~κε", _, "~εδᾰ~κετε";
    perfect_imp_act_3: _, _, "~εδᾰ~κοντων";
    perfect_imp_mid_2: _, _, "~εδᾰ~θε";
    perfect_imp_mid_3: "~εδᾰ~θω", _, "~εδᾰ~θων";
    perfect_imp_pass_2: "~εδᾰ~σο", _, "~εδᾰ~θε";
    perfect_imp_pass_3: _, "~εδᾰ~θων", "~εδᾰ~θων";
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
    // Προβαλλω: 5 words (βᾰλλω, προβαλλω, διαβαλλω, τῐλλω, σκελλω). Like Λογῐζομαι, except in 118 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("προβαλλω", 5, "~λω", like Λογῐζομαι;
    stem future_pass: "~ησησ- like ἐχω";
    stem aorist_ind: "~- like ἐχω";
    stem aorist_ind_pass: "~ην- like ἐχω";
    stem aorist_pass: "~- like σφᾰλλω";
    ind_act_1: "~λω", _, "~λομεν";
    ind_act_2: "~λεις", _, _;
    ind_act_3: "~λει", "~λετον", "~λουσῐ";
    ind_mid_1: "~λομαι", _, "~λομεθᾰ";
    ind_mid_2: "~λῃ", "~λεσθον", _;
    ind_pass_2: _, _, "~λεσθε";
    ind_pass_3: "~λεται", _, "~λονται";
    subj_act_1: "~λω", _, "~λωμεν";
    subj_act_2: "~λῃς", _, _;
    subj_act_3: "~λῃ", "~λητον", "~λωσῐ";
    subj_mid_1: _, _, "~λωμεθᾰ";
    subj_mid_2: _, "~λησθον", _;
    subj_mid_3: "~ληται", _, "~λωνται";
    subj_pass_1: "~λωμαι", _, _;
    subj_pass_2: _, _, "~λησθε";
    opt_act_1: "~λοιμῐ", _, "~λοιμεν";
    opt_act_2: "~λοις", "~λοιτον", _;
    opt_act_3: "~λοι", "~λοιτην", "~λοιεν";
    opt_mid_1: "~λοιμην", _, "~λοιμεθᾰ";
    opt_mid_2: "~λοιο", "~λοισθον", _;
    opt_mid_3: "~λοιτο", "~λοισθην", "~λοιντο";
    opt_pass_2: _, _, "~λοισθε";
    imp_act_2: "~λε", "~λετον", "~λετε";
    imp_mid_2: "~λου", _, "~λεσθε";
    imp_mid_3: "~λεσθω", _, "~λεσθων";
    imp_pass_2: _, _, "~λεσθε";
    imp_pass_3: _, "~λεσθων", "~λεσθων";
    imperfect_act_1: "ἐ~λον", _, "ἐ~λομεν";
    imperfect_act_2: "ἐ~λες", "ἐ~λετον", "ἐ~λετε";
    imperfect_act_3: "ἐ~λε", "ἐ~λετην", "ἐ~λον";
    imperfect_mid_1: "ἐ~λομην", _, "ἐ~λομεθᾰ";
    imperfect_mid_2: "ἐ~λου", _, _;
    imperfect_mid_3: "ἐ~λετο", _, _;
    imperfect_pass_2: _, "ἐ~λεσθον", "ἐ~λεσθε";
    imperfect_pass_3: _, "ἐ~λεσθην", "ἐ~λοντο";
    aorist_subj_act_1: "~ω", _, _;
    aorist_subj_act_2: "~ῃς", "~ητον", _;
    aorist_subj_mid_1: "~ωμαι", _, _;
    aorist_subj_mid_2: "~ῃ", "~ησθον", _;
    aorist_opt_act_1: "~αιμῐ", _, "~αιμεν";
    aorist_opt_act_2: "~ειᾰς", _, _;
    aorist_opt_act_3: "~ειε", "~αιτην", "~ειᾰν";
    aorist_opt_mid_1: "~αιμην", _, _;
    aorist_opt_mid_2: "~αιο", _, _;
    aorist_imp_act_2: "~ον", _, "~ᾰτε";
    aorist_imp_act_3: _, _, "~ᾰντων";
    aorist_imp_mid_2: "~αι", "~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰσθων";
    perfect_ind_mid_1: "ἐ~μαι", _, "ἐ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~σαι", "ἐ~θον", _;
    perfect_ind_mid_3: "ἐ~ται", _, "ἐ~ᾰται";
    perfect_ind_pass_2: _, _, "ἐ~θε";
    perfect_imp_mid_2: _, _, "ἐ~θε";
    perfect_imp_mid_3: "ἐ~θω", _, "ἐ~θων";
    perfect_imp_pass_2: "ἐ~σο", _, "ἐ~θε";
    perfect_imp_pass_3: _, "ἐ~θων", "ἐ~θων";
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_pres: "~λειν", _, "~λεσθαι";
    infinitive_aor: "~αι", _, _;
    infinitive_perf: _, "ἐ~θαι", _;
    participle_pres: "~λων", _, "~λομενος";
    participle_aor: "~ᾱς", "~ᾰμενος", _;
    participle_perf: _, _, "ἐ~μενος";
    ),
    // Μετερχομαι: 5 words (προερχομαι, σῠνερχομαι, ἐπεξερχομαι, μετερχομαι, ξῠνερχομαι). Like Εἰσερχομαι, except in 19 cells: ind_pass_2 pl, ind_pass_3 sg, ind_pass_3 pl, subj_pass_2 sg, subj_pass_2 pl, opt_pass_2 pl, ….
    conjugation!("μετερχομαι", 5, "~ερχομαι", like Εἰσερχομαι;
    ind_pass_2: _, _, derived;
    ind_pass_3: derived, _, derived;
    subj_pass_2: derived, _, derived;
    opt_pass_2: _, _, derived;
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
    // Φειδομαι: 5 words (εἰδομαι, ἐλδομαι, μηδομαι, λῐνδομαι, φειδομαι). Like Πῠνθανομαι, except in 76 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 sg, ….
    conjugation!("φειδομαι", 5, "~~δομαι", like Πῠνθανομαι;
    stem future_pass: "~~σθησ- like ἐχω";
    stem aorist_ind: "~~σ- like πᾰραιτεομαι";
    ind_act_1: "~~δομαι", _, _;
    ind_mid_1: "~~δομαι", _, "~~δομεθᾰ";
    ind_mid_2: "~~δῃ", "~~δεσθον", _;
    ind_pass_2: derived, _, "~~δεσθε";
    ind_pass_3: "~~δεται", _, "~~δονται";
    subj_mid_1: "~~δωμαι", _, "~~δωμεθᾰ";
    subj_mid_2: "~~δῃ", "~~δησθον", _;
    subj_mid_3: "~~δηται", _, "~~δωνται";
    subj_pass_2: "~~δῃ", _, "~~δησθε";
    opt_mid_1: "~~δοιμην", _, "~~δοιμεθᾰ";
    opt_mid_2: "~~δοιο", "~~δοισθον", _;
    opt_mid_3: "~~δοιτο", "~~δοισθην", "~~δοιντο";
    opt_pass_2: _, _, "~~δοισθε";
    imp_mid_2: "~~δου", _, "~~δεσθε";
    imp_mid_3: "~~δεσθω", _, "~~δεσθων";
    imp_pass_2: _, _, "~~δεσθε";
    imp_pass_3: _, "~~δεσθων", "~~δεσθων";
    imperfect_mid_1: "~~δομην", _, "~~δομεθᾰ";
    imperfect_mid_2: "~~δου", "~~δεσθον", _;
    imperfect_mid_3: "~~δετο", "~~δεσθην", "~~δοντο";
    imperfect_pass_2: _, _, "~~δεσθε";
    future_ind_mid_1: "~~σομαι", _, "~~σομεθᾰ";
    future_ind_mid_2: "~~σῃ", "~~σεσθον", _;
    future_ind_mid_3: "~~σεται", _, _;
    future_opt_mid_1: "~~σοιμην", _, "~~σοιμεθᾰ";
    future_opt_mid_2: "~~σοιο", _, _;
    future_opt_mid_3: _, "~~σοισθην", _;
    aorist_subj_mid_1: "~~σωμαι", _, _;
    aorist_subj_mid_2: "~~σῃ", "~~σησθον", _;
    aorist_opt_mid_1: "~~σαιμην", _, _;
    aorist_opt_mid_2: "~~σαιο", _, _;
    aorist_imp_mid_2: "~ῑσα~", "~~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~σᾰσθων";
    perfect_ind_mid_1: "%ε~~σμαι", _, "%ε~~σμεθᾰ";
    perfect_ind_mid_2: "%ε~~σαι", "%ε~~σθον", _;
    perfect_ind_mid_3: "%ε~~σται", _, "%ε~~δᾰται";
    perfect_ind_pass_2: _, _, derived;
    perfect_imp_mid_2: _, _, "%ε~~σθε";
    perfect_imp_mid_3: "%ε~~σθω", _, "%ε~~σθων";
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: _, derived, derived;
    infinitive_pres: _, _, "~~δεσθαι";
    infinitive_perf: _, derived, _;
    participle_pres: _, _, "~~δομενος";
    participle_aor: _, "~~σᾰμενος", _;
    participle_perf: _, _, derived;
    ),
    // Ἀποκωλῡω: 5 words (ἀποδῠω, ἀποκωλῡω, προκῐνδῡνευω, ὑποφερω, ἀποθραυω). Like Ξαινω, except in 150 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀποκωλῡω", 5, "~ο~ω", like Ξαινω;
    stem future_pass: "~ο~θησ- like ἐχω";
    stem aorist_ind: "~ε~σ- like ἐλπω";
    stem aorist_ind_pass: "~ε~θην- like ἐλπω";
    stem aorist_pass: "~ο~θ- like ἐχω";
    ind_act_1: "~ο~ω", _, "~ο~ομεν";
    ind_act_2: "~ο~εις", _, _;
    ind_act_3: "~ο~ει", "~ο~ετον", "~ο~ουσῐ";
    ind_mid_1: "~ο~ομαι", _, "~ο~ομεθᾰ";
    ind_mid_2: "~ο~ῃ", "~ο~εσθον", _;
    ind_pass_2: _, _, "~ο~εσθε";
    ind_pass_3: "~ο~εται", _, "~ο~ονται";
    subj_act_1: "~ο~ω", _, "~ο~ωμεν";
    subj_act_2: "~ο~ῃς", _, _;
    subj_act_3: "~ο~ῃ", "~ο~ητον", "~ο~ωσῐ";
    subj_mid_1: _, _, "~ο~ωμεθᾰ";
    subj_mid_2: _, "~ο~ησθον", _;
    subj_mid_3: "~ο~ηται", _, "~ο~ωνται";
    subj_pass_1: "~ο~ωμαι", _, _;
    subj_pass_2: _, _, "~ο~ησθε";
    opt_act_1: "~ο~οιμῐ", _, "~ο~οιμεν";
    opt_act_2: "~ο~οις", "~ο~οιτον", _;
    opt_act_3: "~ο~οι", "~ο~οιτην", "~ο~οιεν";
    opt_mid_1: "~ο~οιμην", _, "~ο~οιμεθᾰ";
    opt_mid_2: "~ο~οιο", "~ο~οισθον", _;
    opt_mid_3: "~ο~οιτο", "~ο~οισθην", "~ο~οιντο";
    opt_pass_2: _, _, "~ο~οισθε";
    imp_act_2: "~ο~ε", "~ο~ετον", "~ο~ετε";
    imp_mid_2: "~ο~ου", _, "~ο~εσθε";
    imp_mid_3: "~ο~εσθω", _, "~ο~εσθων";
    imp_pass_2: _, _, "~ο~εσθε";
    imp_pass_3: _, "~ο~εσθων", "~ο~εσθων";
    imperfect_act_1: "~ε~ον", _, "~ε~ομεν";
    imperfect_act_2: "~ε~ες", "~ε~ετον", "~ε~ετε";
    imperfect_act_3: "~ε~ε", "~ε~ετην", "~ε~ον";
    imperfect_mid_1: "~ε~ομην", _, "~ε~ομεθᾰ";
    imperfect_mid_2: "~ε~ου", _, _;
    imperfect_mid_3: "~ε~ετο", _, _;
    imperfect_pass_2: _, "~ε~εσθον", "~ε~εσθε";
    imperfect_pass_3: _, "~ε~εσθην", "~ε~οντο";
    future_ind_act_1: "~ο~σω", _, "~ο~σομεν";
    future_ind_act_2: "~ο~σεις", "~ο~σετον", "~ο~σετε";
    future_ind_act_3: _, _, "~ο~σουσῐ";
    future_ind_mid_1: "~ο~σομαι", _, "~ο~σομεθᾰ";
    future_ind_mid_2: "~ο~σῃ", "~ο~σεσθον", _;
    future_ind_mid_3: "~ο~σεται", _, _;
    future_opt_act_1: "~ο~σοιμῐ", _, "~ο~σοιμεν";
    future_opt_act_2: _, "~ο~σοιτον", "~ο~σοιτε";
    future_opt_act_3: "~ο~σοι", "~ο~σοιτην", "~ο~σοιεν";
    future_opt_mid_1: "~ο~σοιμην", _, "~ο~σοιμεθᾰ";
    future_opt_mid_2: "~ο~σοιο", _, _;
    future_opt_mid_3: _, "~ο~σοισθην", _;
    aorist_subj_act_1: "~ο~σω", _, _;
    aorist_subj_act_2: "~ο~σῃς", "~ο~σητον", _;
    aorist_subj_mid_1: "~ο~σωμαι", _, _;
    aorist_subj_mid_2: "~ο~σῃ", "~ο~σησθον", _;
    aorist_opt_act_1: "~ο~σαιμῐ", _, "~ο~σαιμεν";
    aorist_opt_act_2: "~ο~σειᾰς", _, _;
    aorist_opt_act_3: "~ο~σειε", "~ο~σαιτην", "~ο~σειᾰν";
    aorist_opt_mid_1: "~ο~σαιμην", _, _;
    aorist_opt_mid_2: "~ο~σαιο", _, _;
    aorist_imp_act_2: "~ο~σον", _, "~ο~σᾰτε";
    aorist_imp_act_3: _, _, "~ο~σᾰντων";
    aorist_imp_mid_2: "~ο~σαι", "~ο~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ο~σᾰσθων";
    perfect_ind_act_1: "~ο%ε~κᾰ", _, "~ο%ε~κᾰμεν";
    perfect_ind_act_2: _, "~ο%ε~κᾰτον", _;
    perfect_ind_act_3: _, _, "~ο%ε~κᾱσῐ";
    perfect_ind_mid_1: "~ο%ε~μαι", _, "~ο%ε~μεθᾰ";
    perfect_ind_mid_2: "~ο%ε~σαι", "~ο%ε~σθον", _;
    perfect_ind_mid_3: "~ο%ε~ται", _, "~ο%ε~νται";
    perfect_ind_pass_2: _, _, "~ο%ε~σθε";
    perfect_subj_act_1: "~ο%ε~κω", _, _;
    perfect_subj_act_2: "~ο%ε~κῃς", "~ο%ε~κητον", "~ο%ε~κητε";
    perfect_subj_act_3: "~ο%ε~κῃ", _, "~ο%ε~κωσῐ";
    perfect_opt_act_1: "~ο%ε~κοιμῐ", _, "~ο%ε~κοιμεν";
    perfect_opt_act_2: "~ο%ε~κοις", _, _;
    perfect_opt_act_3: _, "~ο%ε~κοιτην", _;
    perfect_imp_act_2: "~ο%ε~κε", _, "~ο%ε~κετε";
    perfect_imp_act_3: _, _, "~ο%ε~κοντων";
    perfect_imp_mid_2: _, _, "~ο%ε~σθε";
    perfect_imp_mid_3: "~ο%ε~σθω", _, "~ο%ε~σθων";
    perfect_imp_pass_2: "~ο%ε~σο", _, "~ο%ε~σθε";
    perfect_imp_pass_3: _, "~ο%ε~σθων", "~ο%ε~σθων";
    infinitive_pres: "~ο~ειν", _, "~ο~εσθαι";
    infinitive_fut: "~ο~σειν", _, _;
    infinitive_aor: "~ο~σαι", _, _;
    infinitive_perf: _, "~ο%ε~σθαι", _;
    participle_pres: "~ο~ων", _, "~ο~ομενος";
    participle_aor: "~ο~σᾱς", "~ο~σᾰμενος", _;
    participle_perf: "~ο%ε~κως", _, "~ο%ε~μενος";
    ),
    // Σκηπτω: 5 words (ῥᾰπτω, ῥῑπτω, σκᾰπτω, ρρῑπτω, ρρᾰπτω). Like Θρῠπτω, except in 61 cells: imperfect_mid_2 du, imperfect_mid_3 du, imperfect_mid_3 pl, imperfect_pass_2 du, imperfect_pass_3 du, imperfect_pass_3 pl, ….
    conjugation!("σκηπτω", 5, "~πτω", like Θρῠπτω;
    stem aorist_ind: "~ψ- like ἐχω";
    stem aorist_ind_pass: "~φην- like ἐχω";
    stem aorist_pass: "~φ- like σφᾰλλω";
    stem pluperfect_act: "~φειν- like κᾰλεω";
    imperfect_mid_2: _, "ἐ~πτεσθον", _;
    imperfect_mid_3: _, "ἐ~πτεσθην", "ἐ~πτοντο";
    imperfect_pass_2: _, derived, _;
    imperfect_pass_3: _, derived, derived;
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
    perfect_ind_mid_1: "ἐ~μμαι", _, "ἐ~μμεθᾰ";
    perfect_ind_mid_2: "ἐ~ψαι", "ἐ~φθον", _;
    perfect_ind_mid_3: "ἐ~πται", _, "ἐ~φᾰται";
    perfect_ind_pass_2: _, _, "ἐ~φθε";
    perfect_subj_act_1: "ἐ~φω", _, _;
    perfect_subj_act_2: "ἐ~φῃς", "ἐ~φητον", "ἐ~φητε";
    perfect_subj_act_3: "ἐ~φῃ", _, "ἐ~φωσῐ|ἐ~φωσῐν";
    perfect_opt_act_1: "ἐ~φοιμῐ|ἐ~φοιην", _, "ἐ~φοιμεν";
    perfect_opt_act_2: "ἐ~φοις|ἐ~φοιης", _, _;
    perfect_opt_act_3: "ἐ~φοι|ἐ~φοιη", "ἐ~φοιτην", _;
    perfect_imp_act_2: "ἐ~φε", _, "ἐ~φετε";
    perfect_imp_act_3: _, _, "ἐ~φοντων";
    perfect_imp_mid_2: _, _, "ἐ~φθε";
    perfect_imp_mid_3: "ἐ~φθω", _, "ἐ~φθων";
    perfect_imp_pass_2: "ἐ~ψο", _, "ἐ~φθε";
    perfect_imp_pass_3: _, "ἐ~φθων", "ἐ~φθων";
    pluperfect_mid_1: "ἐ~μμην", _, _;
    pluperfect_mid_2: "ἐ~ψο", "ἐ~φθον", _;
    pluperfect_mid_3: "ἐ~πτο", _, "ἐ~φᾰτο";
    pluperfect_pass_1: _, _, "ἐ~μμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~φθε";
    pluperfect_pass_3: _, "ἐ~φθην", _;
    infinitive_perf: _, "ἐ~φθαι", _;
    participle_perf: "ἐ~φως", _, "ἐ~μμενος";
    ),
    // Ἀναιρεω: 5 words (ἀναιρεω, ἀφαιρεω, ἐξαιρεω, δῐαιρεω, ὑφαιρεω). Like Μῑγνῡμῐ, except in 174 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀναιρεω", 5, "~αιρεω", like Μῑγνῡμῐ;
    stem future_pass: "~αιρεθησ- like ποιεω";
    stem aorist_ind: "~- like ἀναιρεω";
    stem aorist_ind_pass: "~ῃρεθην- like ἐλπω";
    stem aorist_pass: "~αιρεθ- like ποιεω";
    ind_act_1: "~αιρω", _, "~αιροῡμεν";
    ind_act_2: "~αιρεῑς", _, _;
    ind_act_3: "~αιρεῑ", "~αιρεῑτον", "~αιροῡσῐ|~αιροῡσῐν";
    ind_mid_1: "~αιροῡμαι", _, "~αιρουμεθᾰ";
    ind_mid_2: "~αιρεῑ|~αιρῃ", "~αιρεῑσθον", _;
    ind_pass_2: _, _, "~αιρεῑσθε";
    ind_pass_3: "~αιρεῑται", _, "~αιροῡνται";
    subj_act_1: "~αιρω", _, "~αιρωμεν";
    subj_act_2: "~αιρῃς", _, _;
    subj_act_3: "~αιρῃ", "~αιρητον", "~αιρωσῐ|~αιρωσῐν";
    subj_mid_1: _, _, "~αιρωμεθᾰ";
    subj_mid_2: _, "~αιρησθον", _;
    subj_mid_3: "~αιρηται", _, "~αιρωνται";
    subj_pass_1: "~αιρωμαι", _, _;
    subj_pass_2: _, _, "~αιρησθε";
    opt_act_1: "~αιροιην|~αιροῑμῐ", _, "~αιροῑμεν|~αιροιημεν";
    opt_act_2: "~αιροιης|~αιροῑς", "~αιροῑτον|~αιροιητον", "~αιροῑτε|~αιροιητε";
    opt_act_3: "~αιροιη|~αιροῑ", "~αιροιτην|~αιροιητην", "~αιροῑεν|~αιροιησᾰν";
    opt_mid_1: "~αιροιμην", _, "~αιροιμεθᾰ";
    opt_mid_2: "~αιροῑο", "~αιροῑσθον", _;
    opt_mid_3: "~αιροῑτο", "~αιροισθην", "~αιροῑντο";
    opt_pass_2: _, _, "~αιροῑσθε";
    imp_act_2: "~αιρει", "~αιρεῑτον", "~αιρεῑτε";
    imp_mid_2: "~αιροῡ", _, "~αιρεῑσθε";
    imp_mid_3: "~αιρεισθω", _, "~αιρεισθων";
    imp_pass_2: _, _, "~αιρεῑσθε";
    imp_pass_3: _, "~αιρεισθων", "~αιρεισθων";
    imperfect_act_1: "~ῃρουν", _, "~ῃροῡμεν";
    imperfect_act_2: "~ῃρεις", "~ῃρεῑτον", "~ῃρεῑτε";
    imperfect_act_3: "~ῃρει", "~ῃρειτην", "~ῃρουν";
    imperfect_mid_1: "~ῃρουμην", _, "~ῃρουμεθᾰ";
    imperfect_mid_2: "~ῃροῡ", _, _;
    imperfect_mid_3: "~ῃρεῑτο", _, _;
    imperfect_pass_2: _, "~ῃρεῑσθον", "~ῃρεῑσθε";
    imperfect_pass_3: _, "~ῃρεισθην", "~ῃροῡντο";
    future_ind_act_1: "~αιρησω", _, "~αιρησομεν";
    future_ind_act_2: "~αιρησεις", "~αιρησετον", "~αιρησετε";
    future_ind_act_3: _, _, "~αιρησουσῐ|~αιρησουσῐν";
    future_ind_mid_1: "~αιρησομαι", _, "~αιρησομεθᾰ";
    future_ind_mid_2: "~αιρησῃ|~αιρησει", "~αιρησεσθον", _;
    future_ind_mid_3: "~αιρησεται", _, _;
    future_opt_act_1: "~αιρησοιμῐ", _, "~αιρησοιμεν";
    future_opt_act_2: _, "~αιρησοιτον", "~αιρησοιτε";
    future_opt_act_3: "~αιρησοι", "~αιρησοιτην", "~αιρησοιεν";
    future_opt_mid_1: "~αιρησοιμην", _, "~αιρησοιμεθᾰ";
    future_opt_mid_2: "~αιρησοιο", _, _;
    future_opt_mid_3: _, "~αιρησοισθην", _;
    aorist_subj_act_1: "~ελω|~αιρησω", _, "~ελωμεν|~αιρησωμεν";
    aorist_subj_act_2: "~ελῃς|~αιρησῃς", "~ελητον|~αιρησητον", "~ελητε|~αιρησητε";
    aorist_subj_act_3: "~ελῃ|~αιρησῃ", _, "~ελωσῐ|~ελωσῐν|~αιρησωσῐ|~αιρησωσῐν";
    aorist_subj_mid_1: "~ελωμαι|~αιρησωμαι", _, "~ελωμεθᾰ|~αιρησωμεθᾰ";
    aorist_subj_mid_2: _, "~ελησθον|~αιρησησθον", "~ελησθε|~αιρησησθε";
    aorist_subj_mid_3: "~εληται|~αιρησηται", _, "~ελωνται|~αιρησωνται";
    aorist_opt_act_1: "~ελοιμῐ|~αιρησαιμῐ", _, "~ελοιμεν|~αιρησαιμεν";
    aorist_opt_act_2: "~ελοις|~αιρησειᾰς|~αιρησαις", "~ελοιτον|~αιρησαιτον", "~ελοιτε|~αιρησαιτε";
    aorist_opt_act_3: "~ελοι|~αιρησειε|~αιρησειεν|~αιρησαι", "~ελοιτην|~αιρησαιτην", "~ελοιεν|~αιρησειᾰν|~αιρησαιεν";
    aorist_opt_mid_1: "~ελοιμην|~αιρησαιμην", _, "~ελοιμεθᾰ|~αιρησαιμεθᾰ";
    aorist_opt_mid_2: "~ελοιο|~αιρησαιο", "~ελοισθον|~αιρησαισθον", "~ελοισθε|~αιρησαισθε";
    aorist_opt_mid_3: "~ελοιτο|~αιρησαιτο", "~ελοισθην|~αιρησαισθην", "~ελοιντο|~αιρησαιντο";
    aorist_imp_act_2: "~ελε|~αιρησον", "~ελετον|~αιρησᾰτον", "~ελετε|~αιρησᾰτε";
    aorist_imp_act_3: "~ελετω|~αιρησᾰτω", "~ελετων|~αιρησᾰτων", "~ελοντων|~αιρησᾰντων";
    aorist_imp_mid_2: "~ελοῡ|~αιρησαι", "~ελεσθον|~αιρησᾰσθον", "~ελεσθε|~αιρησᾰσθε";
    aorist_imp_mid_3: "~ελεσθω|~αιρησᾰσθω", "~ελεσθων|~αιρησᾰσθων", _;
    perfect_ind_act_1: "~ῃρηκᾰ", _, "~ῃρηκᾰμεν";
    perfect_ind_act_2: _, "~ῃρηκᾰτον", _;
    perfect_ind_act_3: "~ῃρηκε|~ῃρηκεν", _, "~ῃρηκᾱσῐ|~ῃρηκᾱσῐν";
    perfect_ind_mid_1: "~ῃρημαι", _, "~ῃρημεθᾰ";
    perfect_ind_mid_2: "~ῃρησαι", "~ῃρησθον", _;
    perfect_ind_mid_3: "~ῃρηται", _, "~ῃρηνται";
    perfect_ind_pass_2: _, _, "~ῃρησθε";
    perfect_subj_act_1: "~ῃρηκω", _, _;
    perfect_subj_act_2: "~ῃρηκῃς", "~ῃρηκητον", "~ῃρηκητε";
    perfect_subj_act_3: "~ῃρηκῃ", _, "~ῃρηκωσῐ|~ῃρηκωσῐν";
    perfect_opt_act_1: "~ῃρηκοιμῐ|~ῃρηκοιην", _, "~ῃρηκοιμεν";
    perfect_opt_act_2: "~ῃρηκοις|~ῃρηκοιης", _, _;
    perfect_opt_act_3: "~ῃρηκοι|~ῃρηκοιη", "~ῃρηκοιτην", _;
    perfect_imp_act_2: "~ῃρηκε", _, "~ῃρηκετε";
    perfect_imp_act_3: _, _, "~ῃρηκοντων";
    perfect_imp_mid_2: _, _, "~ῃρησθε";
    perfect_imp_mid_3: "~ῃρησθω", _, "~ῃρησθων";
    perfect_imp_pass_2: "~ῃρησο", _, "~ῃρησθε";
    perfect_imp_pass_3: _, "~ῃρησθων", "~ῃρησθων";
    infinitive_pres: "~αιρεῑν", _, "~αιρεῑσθαι";
    infinitive_fut: "~αιρησειν", _, _;
    infinitive_aor: "~ελεῑν|~αιρησαι", "~ελεσθαι|~αιρησᾰσθαι", _;
    infinitive_perf: _, "~ῃρησθαι", _;
    participle_pres: "~αιρων", _, "~αιρουμενος";
    participle_aor: "~ελων|~αιρησᾱς", "~ελομενος|~αιρησᾰμενος", _;
    participle_perf: "~ῃρηκως", _, "~ῃρημενος";
    ),
    // Ῥαινω: 5 words (ῥαινω, κραινω, ὑγῐαινω, ὑδραινω, τετραινω). Like Μωραινω, except in 63 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imp_act_2 sg, imperfect_act_3 sg, future_ind_act_1 sg, ….
    conjugation!("ῥαινω", 5, "~αινω", like Μωραινω;
    stem future_pass: "~ανθησησ- like ἐχω";
    stem aorist_ind_pass: "~ᾱνθην- like ἐχω";
    stem aorist_pass: "~ᾱνθ- like ἐχω";
    ind_act_3: _, _, "~αινουσῐ";
    ind_mid_2: "~αινῃ", _, _;
    subj_act_3: _, _, "~αινωσῐ";
    imp_act_2: "~αῑνε", _, _;
    imperfect_act_3: "ἐ~αινε", _, _;
    future_ind_act_1: "~ᾰνω", _, "~ᾰνοῡμεν";
    future_ind_act_2: "~ᾰνεῑς", "~ᾰνεῑτον", "~ᾰνεῑτε";
    future_ind_act_3: _, _, "~ᾰνοῡσῐ";
    future_ind_mid_1: "~ᾰνοῡμαι", _, "~ᾰνουμεθᾰ";
    future_ind_mid_2: "~ᾰνεῑ", "~ᾰνεῑσθον", _;
    future_ind_mid_3: "~ᾰνεῑται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~ᾰνοιην", _, "~ᾰνοῑμεν";
    future_opt_act_2: _, "~ᾰνοῑτον", "~ᾰνοῑτε";
    future_opt_act_3: "~ᾰνοιη", "~ᾰνοιτην", "~ᾰνοῑεν";
    future_opt_mid_1: "~ᾰνοιμην", _, "~ᾰνοιμεθᾰ";
    future_opt_mid_2: "~ᾰνοῑο", _, _;
    future_opt_mid_3: _, "~ᾰνοισθην", _;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_1: "%ε~αμαι", _, "%ε~αμεθᾰ";
    perfect_ind_mid_2: "%ε~ασαι", "%ε~ασθον", _;
    perfect_ind_mid_3: "%ε~αται", _, "%ε~ανται";
    perfect_ind_pass_2: _, _, "%ε~ασθε";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: _, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, _, "%ε~ασθε";
    perfect_imp_mid_3: "%ε~ασθω", _, "%ε~ασθων";
    perfect_imp_pass_2: "%ε~ασο", _, "%ε~ασθε";
    perfect_imp_pass_3: _, "%ε~ασθων", "%ε~ασθων";
    infinitive_fut: "~ᾰνεῑν", _, _;
    infinitive_perf: _, "%ε~ᾱσθαι", _;
    participle_perf: derived, _, "%ε~αμενος";
    ),
    // Ἐλαυνω: 5 words (ἐσθῐω, ἐλαυνω, ἐρειδω, ἐχθω, εκπλεω). Like Μῑγνῡμῐ, except in 70 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐλαυνω", 5, "ἐ~ω", like Μῑγνῡμῐ;
    ind_act_1: "ἐ~ω", _, "ἐ~ομεν";
    ind_act_2: "ἐ~εις", _, _;
    ind_act_3: "ἐ~ει", "ἐ~ετον", "ἐ~ουσῐ";
    ind_mid_1: "ἐ~ομαι", _, "ἐ~ομεθᾰ";
    ind_mid_2: "ἐ~ῃ", "ἐ~εσθον", _;
    ind_pass_2: _, _, "ἐ~εσθε";
    ind_pass_3: "ἐ~εται", _, "ἐ~ονται";
    subj_act_1: "ἐ~ω", _, "ἐ~ωμεν";
    subj_act_2: "ἐ~ῃς", _, _;
    subj_act_3: "ἐ~ῃ", "ἐ~ητον", "ἐ~ωσῐ";
    subj_mid_1: _, _, "ἐ~ωμεθᾰ";
    subj_mid_2: _, "ἐ~ησθον", _;
    subj_mid_3: "ἐ~ηται", _, "ἐ~ωνται";
    subj_pass_1: "ἐ~ωμαι", _, _;
    subj_pass_2: _, _, "ἐ~ησθε";
    opt_act_1: "ἐ~οιμῐ", _, "ἐ~οιμεν";
    opt_act_2: "ἐ~οις", "ἐ~οιτον", _;
    opt_act_3: "ἐ~οι", "ἐ~οιτην", "ἐ~οιεν";
    opt_mid_1: "ἐ~οιμην", _, "ἐ~οιμεθᾰ";
    opt_mid_2: "ἐ~οιο", "ἐ~οισθον", _;
    opt_mid_3: "ἐ~οιτο", "ἐ~οισθην", "ἐ~οιντο";
    opt_pass_2: _, _, "ἐ~οισθε";
    imp_act_2: "ἐ~ε", "ἐ~ετον", "ἐ~ετε";
    imp_mid_2: "ἐ~ου", _, "ἐ~εσθε";
    imp_mid_3: "ἐ~εσθω", _, "ἐ~εσθων";
    imp_pass_2: _, _, "ἐ~εσθε";
    imp_pass_3: _, "ἐ~εσθων", "ἐ~εσθων";
    imperfect_act_1: "ἠ~ον", _, "ἠ~ομεν";
    imperfect_act_2: "ἠ~ες", "ἠ~ετον", "ἠ~ετε";
    imperfect_act_3: "ἠ~ε", "ἠ~ετην", "ἠ~ον";
    imperfect_mid_1: "ἠ~ομην", _, "ἠ~ομεθᾰ";
    imperfect_mid_2: "ἠ~ου", _, _;
    imperfect_mid_3: "ἠ~ετο", _, _;
    imperfect_pass_2: _, "ἠ~εσθον", "ἠ~εσθε";
    imperfect_pass_3: _, "ἠ~εσθην", "ἠ~οντο";
    infinitive_pres: "ἐ~ειν", _, "ἐ~εσθαι";
    participle_pres: "ἐ~ων", _, "ἐ~ομενος";
    ),
    // Ἐξᾰκολουθεω: 5 words (πᾰρᾰμελεω, σῠνᾰδῐκεω, ξῠνᾰδῐκεω, ἀπᾰσχολεω, ἐξᾰκολουθεω). Like Λᾰγοθηρεω, except in 60 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("ἐξᾰκολουθεω", 5, "~ᾰ~εω", like Λᾰγοθηρεω;
    stem aorist_ind: "~η~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~η~ηθην- like ἐλπω";
    stem pluperfect_act: "~η~ηκ- like εὐλογεω";
    imperfect_act_1: "~η~ουν", _, "~η~οῡμεν";
    imperfect_act_2: "~η~εις", "~η~εῑτον", "~η~εῑτε";
    imperfect_act_3: "~η~ει", "~η~ειτην", "~η~ουν";
    imperfect_mid_1: "~η~ουμην", _, "~η~ουμεθᾰ";
    imperfect_mid_2: "~η~οῡ", _, _;
    imperfect_mid_3: "~η~εῑτο", _, _;
    imperfect_pass_2: _, "~η~εῑσθον", "~η~εῑσθε";
    imperfect_pass_3: _, "~η~εισθην", "~η~οῡντο";
    perfect_ind_act_1: "~η~ηκᾰ", _, "~η~ηκᾰμεν";
    perfect_ind_act_2: _, "~η~ηκᾰτον", _;
    perfect_ind_act_3: "~η~ηκε|~η~ηκεν", _, "~η~ηκᾱσῐ|~η~ηκᾱσῐν";
    perfect_ind_mid_1: "~η~ημαι", _, "~η~ημεθᾰ";
    perfect_ind_mid_2: "~η~ησαι", "~η~ησθον", _;
    perfect_ind_mid_3: "~η~ηται", _, "~η~ηνται";
    perfect_ind_pass_2: _, _, "~η~ησθε";
    perfect_subj_act_1: "~η~ηκω", _, _;
    perfect_subj_act_2: "~η~ηκῃς", "~η~ηκητον", "~η~ηκητε";
    perfect_subj_act_3: "~η~ηκῃ", _, "~η~ηκωσῐ|~η~ηκωσῐν";
    perfect_opt_act_1: "~η~ηκοιμῐ|~η~ηκοιην", _, "~η~ηκοιμεν";
    perfect_opt_act_2: "~η~ηκοις|~η~ηκοιης", _, _;
    perfect_opt_act_3: "~η~ηκοι|~η~ηκοιη", "~η~ηκοιτην", _;
    perfect_imp_act_2: "~η~ηκε", _, "~η~ηκετε";
    perfect_imp_act_3: _, _, "~η~ηκοντων";
    perfect_imp_mid_2: _, _, "~η~ησθε";
    perfect_imp_mid_3: "~η~ησθω", _, "~η~ησθων";
    perfect_imp_pass_2: "~η~ησο", _, "~η~ησθε";
    perfect_imp_pass_3: _, "~η~ησθων", "~η~ησθων";
    pluperfect_mid_1: "~η~ημην", _, _;
    pluperfect_mid_2: "~η~ησο", "~η~ησθον", _;
    pluperfect_mid_3: "~η~ητο", _, "~η~ηντο";
    pluperfect_pass_1: _, _, "~η~ημεθᾰ";
    pluperfect_pass_2: _, _, "~η~ησθε";
    pluperfect_pass_3: _, "~η~ησθην", _;
    infinitive_perf: _, "~η~ησθαι", _;
    participle_perf: "~η~ηκως", _, "~η~ημενος";
    ),
    // Προκᾰλεω: 5 words (προσκᾰλεω, σῠγκᾰλεω, προκᾰλεω, ἐγκᾰλεω, ἐκκᾰλεω). Like Ζευγνῡμῐ, except in 150 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("προκᾰλεω", 5, "~~κᾰ~εω", like Ζευγνῡμῐ;
    stem future_pass: "~~κ~ηθησ- like ἐχω";
    stem aorist_ind: "~ξε~ᾰ~εσ- like ἐλπω";
    stem aorist_ind_pass: "~ξε~~ηθην- like ἐλπω";
    stem aorist_pass: "~~κ~ηθ- like ἐχω";
    ind_act_1: "~~κᾰ~ω", _, "~~κᾰ~οῡμεν";
    ind_act_2: "~~κᾰ~εῑς", _, _;
    ind_act_3: "~~κᾰ~εῑ", "~~κᾰ~εῑτον", "~~κᾰ~οῡσῐ";
    ind_mid_1: "~~κᾰ~οῡμαι", _, "~~κᾰ~ουμεθᾰ";
    ind_mid_2: "~~κᾰ~εῑ", "~~κᾰ~εῑσθον", _;
    ind_pass_2: _, _, "~~κᾰ~εῑσθε";
    ind_pass_3: "~~κᾰ~εῑται", _, "~~κᾰ~οῡνται";
    subj_act_1: "~~κᾰ~ω", _, "~~κᾰ~ωμεν";
    subj_act_2: "~~κᾰ~ῃς", _, _;
    subj_act_3: "~~κᾰ~ῃ", "~~κᾰ~ητον", "~~κᾰ~ωσῐ";
    subj_mid_1: _, _, "~~κᾰ~ωμεθᾰ";
    subj_mid_2: _, "~~κᾰ~ησθον", _;
    subj_mid_3: "~~κᾰ~ηται", _, "~~κᾰ~ωνται";
    subj_pass_1: "~~κᾰ~ωμαι", _, _;
    subj_pass_2: _, _, "~~κᾰ~ησθε";
    opt_act_1: "~~κᾰ~οιην", _, "~~κᾰ~οῑμεν";
    opt_act_2: "~~κᾰ~οιης", "~~κᾰ~οῑτον", _;
    opt_act_3: "~~κᾰ~οιη", "~~κᾰ~οιτην", "~~κᾰ~οῑεν";
    opt_mid_1: "~~κᾰ~οιμην", _, "~~κᾰ~οιμεθᾰ";
    opt_mid_2: "~~κᾰ~οῑο", "~~κᾰ~οῑσθον", _;
    opt_mid_3: "~~κᾰ~οῑτο", "~~κᾰ~οισθην", "~~κᾰ~οῑντο";
    opt_pass_2: _, _, "~~κᾰ~οῑσθε";
    imp_act_2: "~~κᾰ~ει", "~~κᾰ~εῑτον", "~~κᾰ~εῑτε";
    imp_mid_2: "~~κᾰ~οῡ", _, "~~κᾰ~εῑσθε";
    imp_mid_3: "~~κᾰ~εισθω", _, "~~κᾰ~εισθων";
    imp_pass_2: _, _, "~~κᾰ~εῑσθε";
    imp_pass_3: _, "~~κᾰ~εισθων", "~~κᾰ~εισθων";
    imperfect_act_1: "~ξε~ᾰ~ουν", _, "~ξε~ᾰ~οῡμεν";
    imperfect_act_2: "~ξε~ᾰ~εις", "~ξε~ᾰ~εῑτον", "~ξε~ᾰ~εῑτε";
    imperfect_act_3: "~ξε~ᾰ~ει", "~ξε~ᾰ~ειτην", "~ξε~ᾰ~ουν";
    imperfect_mid_1: "~ξε~ᾰ~ουμην", _, "~ξε~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "~ξε~ᾰ~οῡ", _, _;
    imperfect_mid_3: "~ξε~ᾰ~εῑτο", _, _;
    imperfect_pass_2: _, "~ξε~ᾰ~εῑσθον", "~ξε~ᾰ~εῑσθε";
    imperfect_pass_3: _, "~ξε~ᾰ~εισθην", "~ξε~ᾰ~οῡντο";
    future_ind_act_1: "~~κᾰ~εσω", _, "~~κᾰ~εσομεν";
    future_ind_act_2: "~~κᾰ~εσεις", "~~κᾰ~εσετον", "~~κᾰ~εσετε";
    future_ind_act_3: _, _, "~~κᾰ~εσουσῐ";
    future_ind_mid_1: "~~κᾰ~εσομαι", _, "~~κᾰ~εσομεθᾰ";
    future_ind_mid_2: "~~κᾰ~εσῃ", "~~κᾰ~εσεσθον", _;
    future_ind_mid_3: "~~κᾰ~εσεται", _, _;
    future_opt_act_1: "~~κᾰ~εσοιμῐ", _, "~~κᾰ~εσοιμεν";
    future_opt_act_2: _, "~~κᾰ~εσοιτον", "~~κᾰ~εσοιτε";
    future_opt_act_3: "~~κᾰ~εσοι", "~~κᾰ~εσοιτην", "~~κᾰ~εσοιεν";
    future_opt_mid_1: "~~κᾰ~εσοιμην", _, "~~κᾰ~εσοιμεθᾰ";
    future_opt_mid_2: "~~κᾰ~εσοιο", _, _;
    future_opt_mid_3: _, "~~κᾰ~εσοισθην", _;
    aorist_subj_act_1: "~~κᾰ~εσω", _, _;
    aorist_subj_act_2: "~~κᾰ~εσῃς", "~~κᾰ~εσητον", _;
    aorist_subj_mid_1: "~~κᾰ~εσωμαι", _, _;
    aorist_subj_mid_2: "~~κᾰ~εσῃ", "~~κᾰ~εσησθον", _;
    aorist_opt_act_1: "~~κᾰ~εσαιμῐ", _, "~~κᾰ~εσαιμεν";
    aorist_opt_act_2: "~~κᾰ~εσειᾰς", _, _;
    aorist_opt_act_3: "~~κᾰ~εσειε", "~~κᾰ~εσαιτην", "~~κᾰ~εσειᾰν";
    aorist_opt_mid_1: "~~κᾰ~εσαιμην", _, _;
    aorist_opt_mid_2: "~~κᾰ~εσαιο", _, _;
    aorist_imp_act_2: "~~κᾰ~εσον", _, "~~κᾰ~εσᾰτε";
    aorist_imp_act_3: _, _, "~~κᾰ~εσᾰντων";
    aorist_imp_mid_2: "~~κᾰ~εσαι", "~~κᾰ~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~κᾰ~εσᾰσθων";
    perfect_ind_act_1: "~~κεκ~ηκᾰ", _, "~~κεκ~ηκᾰμεν";
    perfect_ind_act_2: _, "~~κεκ~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "~~κεκ~ηκᾱσῐ";
    perfect_ind_mid_1: "~~κεκ~ημαι", _, "~~κεκ~ημεθᾰ";
    perfect_ind_mid_2: "~~κεκ~ησαι", "~~κεκ~ησθον", _;
    perfect_ind_mid_3: "~~κεκ~ηται", _, "~~κεκ~ηνται";
    perfect_ind_pass_2: _, _, "~~κεκ~ησθε";
    perfect_subj_act_1: "~~κεκ~ηκω", _, _;
    perfect_subj_act_2: "~~κεκ~ηκῃς", "~~κεκ~ηκητον", "~~κεκ~ηκητε";
    perfect_subj_act_3: "~~κεκ~ηκῃ", _, "~~κεκ~ηκωσῐ";
    perfect_opt_act_1: "~~κεκ~ηκοιμῐ", _, "~~κεκ~ηκοιμεν";
    perfect_opt_act_2: "~~κεκ~ηκοις", _, _;
    perfect_opt_act_3: _, "~~κεκ~ηκοιτην", _;
    perfect_imp_act_2: "~~κεκ~ηκε", _, "~~κεκ~ηκετε";
    perfect_imp_act_3: _, _, "~~κεκ~ηκοντων";
    perfect_imp_mid_2: _, _, "~~κεκ~ησθε";
    perfect_imp_mid_3: "~~κεκ~ησθω", _, "~~κεκ~ησθων";
    perfect_imp_pass_2: "~~κεκ~ησο", _, "~~κεκ~ησθε";
    perfect_imp_pass_3: _, "~~κεκ~ησθων", "~~κεκ~ησθων";
    infinitive_pres: "~~κᾰ~εῑν", _, "~~κᾰ~εῑσθαι";
    infinitive_fut: "~~κᾰ~εσειν", _, _;
    infinitive_aor: "~~κᾰ~εσαι", _, _;
    infinitive_perf: _, "~~κεκ~ησθαι", _;
    participle_pres: "~~κᾰ~ων", _, "~~κᾰ~ουμενος";
    participle_aor: "~~κᾰ~εσᾱς", "~~κᾰ~εσᾰμενος", _;
    participle_perf: "~~κεκ~ηκως", _, "~~κεκ~ημενος";
    ),
    // Ἡγεομαι: 4 words (ἡγεομαι, νεομαι, ἐξηγεομαι, εὐνομεομαι). Like Ἀραομαι, except in 54 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ἡγεομαι", 4, "~εομαι", like Ἀραομαι;
    stem future_pass: "~ηθησ- like ἐχω";
    stem aorist_ind: "~ησ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ηθην- like ἐλπω";
    stem aorist_pass: "~ηθ- like ἐχω";
    ind_act_1: "~εομαι", _, _;
    ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    ind_mid_2: "~εῑ", "~εῑσθον", _;
    ind_pass_2: _, _, "~εῑσθε";
    ind_pass_3: "~εῑται", _, "~οῡνται";
    subj_mid_2: "~ῃ", "~ησθον", _;
    subj_mid_3: "~ηται", _, _;
    subj_pass_2: "~ῃ", _, "~ησθε";
    opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    opt_mid_2: "~οῑο", "~οῑσθον", _;
    opt_mid_3: "~οῑτο", "~οισθην", "~οῑντο";
    opt_pass_2: _, _, "~οῑσθε";
    imp_mid_2: "~οῡ", _, "~εῑσθε";
    imp_mid_3: "~εισθω", _, "~εισθων";
    imp_pass_2: _, _, "~εῑσθε";
    imp_pass_3: _, "~εισθων", "~εισθων";
    imperfect_mid_1: "~ουμην", _, "~ουμεθᾰ";
    imperfect_mid_2: "~οῡ", "~εῑσθον", _;
    imperfect_mid_3: "~εῑτο", "~εισθην", "~οῡντο";
    imperfect_pass_2: _, _, "~εῑσθε";
    perfect_ind_mid_1: "~ημαι", _, "~ημεθᾰ";
    perfect_ind_mid_2: "~ησαι", "~ησθον", _;
    perfect_ind_mid_3: "~ηται", _, "~ηνται";
    perfect_ind_pass_2: _, _, "~ησθε";
    perfect_imp_mid_2: _, _, "~ησθε";
    perfect_imp_mid_3: "~ησθω", _, "~ησθων";
    perfect_imp_pass_2: "~ησο", _, "~ησθε";
    perfect_imp_pass_3: _, "~ησθων", "~ησθων";
    infinitive_pres: _, _, "~εῑσθαι";
    infinitive_perf: _, "~ησθαι", _;
    participle_pres: _, _, "~ουμενος";
    participle_perf: _, _, "~ημενος";
    ),
    // Τῠπτω: 4 words (κοπτω, τῠπτω, βλᾰπτω, σκωπτω). Like Θρῠπτω, except in 55 cells: future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, aorist_opt_pass_3 du, ….
    conjugation!("τῠπτω", 4, "~πτω", like Θρῠπτω;
    stem future_pass: "~πησ- like ἐχω";
    stem aorist_ind_pass: "~πην- like ἐχω";
    stem aorist_pass: "~π- like σφᾰλλω";
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
    perfect_opt_act_1: "%ε~φοιμῐ|%ε~φοιην", _, "%ε~φοιμεν";
    perfect_opt_act_2: "%ε~φοις|%ε~φοιης", _, _;
    perfect_opt_act_3: "%ε~φοι|%ε~φοιη", "%ε~φοιτην", _;
    perfect_imp_act_2: "%ε~φε", _, "%ε~φετε";
    perfect_imp_act_3: _, _, "%ε~φοντων";
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
    // Ἐπᾰγγελλω: 4 words (πᾰρᾰγγελλω, ἀπᾰγγελλω, ἐπᾰγγελλω, ἀνᾰγγελλω). Like Ζευγνῡμῐ, except in 154 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐπᾰγγελλω", 4, "~ᾰ~~λω", like Ζευγνῡμῐ;
    stem future_pass: "~ᾰ~~θησ- like ποιεω";
    stem aorist_ind: "~η~ι~- like εὐλογεω";
    stem aorist_ind_pass: "~η~~θην- like ἐλπω";
    stem aorist_pass: "~ᾰ~~θ- like ποιεω";
    ind_act_1: "~ᾰ~~λω", _, "~ᾰ~~λομεν";
    ind_act_2: "~ᾰ~~λεις", _, _;
    ind_act_3: "~ᾰ~~λει", "~ᾰ~~λετον", "~ᾰ~~λουσῐ|~ᾰ~~λουσῐν";
    ind_mid_1: "~ᾰ~~λομαι", _, "~ᾰ~~λομεθᾰ";
    ind_mid_2: "~ᾰ~~λῃ|~ᾰ~~λει", "~ᾰ~~λεσθον", _;
    ind_pass_2: _, _, "~ᾰ~~λεσθε";
    ind_pass_3: "~ᾰ~~λεται", _, "~ᾰ~~λονται";
    subj_act_1: "~ᾰ~~λω", _, "~ᾰ~~λωμεν";
    subj_act_2: "~ᾰ~~λῃς", _, _;
    subj_act_3: "~ᾰ~~λῃ", "~ᾰ~~λητον", "~ᾰ~~λωσῐ|~ᾰ~~λωσῐν";
    subj_mid_1: _, _, "~ᾰ~~λωμεθᾰ";
    subj_mid_2: _, "~ᾰ~~λησθον", _;
    subj_mid_3: "~ᾰ~~ληται", _, "~ᾰ~~λωνται";
    subj_pass_1: "~ᾰ~~λωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~~λησθε";
    opt_act_1: "~ᾰ~~λοιμῐ", _, "~ᾰ~~λοιμεν";
    opt_act_2: "~ᾰ~~λοις", "~ᾰ~~λοιτον", _;
    opt_act_3: "~ᾰ~~λοι", "~ᾰ~~λοιτην", "~ᾰ~~λοιεν";
    opt_mid_1: "~ᾰ~~λοιμην", _, "~ᾰ~~λοιμεθᾰ";
    opt_mid_2: "~ᾰ~~λοιο", "~ᾰ~~λοισθον", _;
    opt_mid_3: "~ᾰ~~λοιτο", "~ᾰ~~λοισθην", "~ᾰ~~λοιντο";
    opt_pass_2: _, _, "~ᾰ~~λοισθε";
    imp_act_2: "~ᾰ~~λε", "~ᾰ~~λετον", "~ᾰ~~λετε";
    imp_mid_2: "~ᾰ~~λου", _, "~ᾰ~~λεσθε";
    imp_mid_3: "~ᾰ~~λεσθω", _, "~ᾰ~~λεσθων";
    imp_pass_2: _, _, "~ᾰ~~λεσθε";
    imp_pass_3: _, "~ᾰ~~λεσθων", "~ᾰ~~λεσθων";
    imperfect_act_1: "~η~~λον", _, "~η~~λομεν";
    imperfect_act_2: "~η~~λες", "~η~~λετον", "~η~~λετε";
    imperfect_act_3: "~η~~λε|~η~~λεν", "~η~~λετην", "~η~~λον";
    imperfect_mid_1: "~η~~λομην", _, "~η~~λομεθᾰ";
    imperfect_mid_2: "~η~~λου", _, _;
    imperfect_mid_3: "~η~~λετο", _, _;
    imperfect_pass_2: _, "~η~~λεσθον", "~η~~λεσθε";
    imperfect_pass_3: _, "~η~~λεσθην", "~η~~λοντο";
    future_ind_act_1: "~ᾰ~~ω", _, "~ᾰ~~οῡμεν";
    future_ind_act_2: "~ᾰ~~εῑς", "~ᾰ~~εῑτον", "~ᾰ~~εῑτε";
    future_ind_act_3: _, _, "~ᾰ~~οῡσῐ|~ᾰ~~οῡσῐν";
    future_ind_mid_1: "~ᾰ~~οῡμαι", _, "~ᾰ~~ουμεθᾰ";
    future_ind_mid_2: "~ᾰ~~εῑ|~ᾰ~~ῃ", "~ᾰ~~εῑσθον", _;
    future_ind_mid_3: "~ᾰ~~εῑται", _, _;
    future_opt_act_1: "~ᾰ~~οιην|~ᾰ~~οῑμῐ", _, "~ᾰ~~οῑμεν|~ᾰ~~οιημεν";
    future_opt_act_2: "~ᾰ~~οιης|~ᾰ~~οῑς", "~ᾰ~~οῑτον|~ᾰ~~οιητον", "~ᾰ~~οῑτε|~ᾰ~~οιητε";
    future_opt_act_3: "~ᾰ~~οιη|~ᾰ~~οῑ", "~ᾰ~~οιτην|~ᾰ~~οιητην", "~ᾰ~~οῑεν|~ᾰ~~οιησᾰν";
    future_opt_mid_1: "~ᾰ~~οιμην", _, "~ᾰ~~οιμεθᾰ";
    future_opt_mid_2: "~ᾰ~~οῑο", _, _;
    future_opt_mid_3: _, "~ᾰ~~οισθην", _;
    aorist_subj_act_1: "~ᾰ~ι~ω", _, _;
    aorist_subj_act_2: "~ᾰ~ι~ῃς", "~ᾰ~ι~ητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~ι~ωσῐ|~ᾰ~ι~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ι~ωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ι~ῃ", "~ᾰ~ι~ησθον", _;
    aorist_opt_act_1: "~ᾰ~ι~αιμῐ", _, "~ᾰ~ι~αιμεν";
    aorist_opt_act_2: "~ᾰ~ι~ειᾰς|~ᾰ~ι~αις", _, _;
    aorist_opt_act_3: "~ᾰ~ι~ειε|~ᾰ~ι~ειεν|~ᾰ~ι~αι", "~ᾰ~ι~αιτην", "~ᾰ~ι~ειᾰν|~ᾰ~ι~αιεν";
    aorist_opt_mid_1: "~ᾰ~ι~αιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~ι~αιο", _, _;
    aorist_imp_act_2: "~ᾰ~ι~ον", _, "~ᾰ~ι~ᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ι~ᾰντων";
    aorist_imp_mid_2: "~ᾰ~ι~αι", "~ᾰ~ι~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ι~ᾰσθων";
    perfect_ind_act_1: "~η~~κᾰ", _, "~η~~κᾰμεν";
    perfect_ind_act_2: _, "~η~~κᾰτον", _;
    perfect_ind_act_3: "~η~~κε|~η~~κεν", _, "~η~~κᾱσῐ|~η~~κᾱσῐν";
    perfect_ind_mid_1: "~η~~μαι", _, "~η~~μεθᾰ";
    perfect_ind_mid_2: "~η~~σαι", "~η~~θον", _;
    perfect_ind_mid_3: "~η~~ται", _, "~η~~ᾰται";
    perfect_ind_pass_2: _, _, "~η~~θε";
    perfect_subj_act_1: "~η~~κω", _, _;
    perfect_subj_act_2: "~η~~κῃς", "~η~~κητον", "~η~~κητε";
    perfect_subj_act_3: "~η~~κῃ", _, "~η~~κωσῐ|~η~~κωσῐν";
    perfect_opt_act_1: "~η~~κοιμῐ|~η~~κοιην", _, "~η~~κοιμεν";
    perfect_opt_act_2: "~η~~κοις|~η~~κοιης", _, _;
    perfect_opt_act_3: "~η~~κοι|~η~~κοιη", "~η~~κοιτην", _;
    perfect_imp_act_2: "~η~~κε", _, "~η~~κετε";
    perfect_imp_act_3: _, _, "~η~~κοντων";
    perfect_imp_mid_2: _, _, "~η~~θε";
    perfect_imp_mid_3: "~η~~θω", _, "~η~~θων";
    perfect_imp_pass_2: "~η~~σο", _, "~η~~θε";
    perfect_imp_pass_3: _, "~η~~θων", "~η~~θων";
    infinitive_pres: "~ᾰ~~λειν", _, "~ᾰ~~λεσθαι";
    infinitive_fut: "~ᾰ~~εῑν", _, _;
    infinitive_aor: "~ᾰ~ῑ~αι", _, _;
    infinitive_perf: _, "~η~~θαι", _;
    participle_pres: "~ᾰ~~λων", _, "~ᾰ~~λομενος";
    participle_aor: "~ᾰ~ι~ᾱς", "~ᾰ~ι~ᾰμενος", _;
    participle_perf: "~η~~κως", _, "~η~~μενος";
    ),
    // Ναιω: 4 words (καιω, κλαιω, ναιω, δαιω). Like Κᾰλεω, except in 158 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ναιω", 4, "~αιω", like Κᾰλεω;
    stem future_pass: "~αυθησ- like ἐχω";
    stem aorist_ind: "~αυσ- like ἐχω";
    stem aorist_ind_pass: "~αυθην- like ἐχω";
    stem aorist_pass: "~αυθ- like ἐχω";
    stem pluperfect_act: "~εκαυκειν- like κᾰλεω";
    ind_act_1: "~αιω", _, "~αιομεν";
    ind_act_2: "~αιεις", _, _;
    ind_act_3: "~αιει", "~αιετον", "~αιουσῐ";
    ind_mid_1: "~αιομαι", _, "~αιομεθᾰ";
    ind_mid_2: "~αιῃ", "~αιεσθον", _;
    ind_pass_2: _, _, "~αιεσθε";
    ind_pass_3: "~αιεται", _, "~αιονται";
    subj_act_1: "~αιω", _, "~αιωμεν";
    subj_act_2: "~αιῃς", _, _;
    subj_act_3: "~αιῃ", "~αιητον", "~αιωσῐ";
    subj_mid_1: _, _, "~αιωμεθᾰ";
    subj_mid_2: _, "~αιησθον", _;
    subj_mid_3: "~αιηται", _, "~αιωνται";
    subj_pass_1: "~αιωμαι", _, _;
    subj_pass_2: _, _, "~αιησθε";
    opt_act_1: "~αιοιμῐ", _, "~αιοιμεν";
    opt_act_2: "~αιοις", "~αιοιτον", _;
    opt_act_3: "~αιοι", "~αιοιτην", "~αιοιεν";
    opt_mid_1: "~αιοιμην", _, "~αιοιμεθᾰ";
    opt_mid_2: "~αιοιο", "~αιοισθον", _;
    opt_mid_3: "~αιοιτο", "~αιοισθην", "~αιοιντο";
    opt_pass_2: _, _, "~αιοισθε";
    imp_act_2: "~αῑε", "~αιετον", "~αιετε";
    imp_mid_2: "~αιου", _, "~αιεσθε";
    imp_mid_3: "~αιεσθω", _, "~αιεσθων";
    imp_pass_2: _, _, "~αιεσθε";
    imp_pass_3: _, "~αιεσθων", "~αιεσθων";
    imperfect_act_1: "ἐ~αιον", _, "ἐ~αιομεν";
    imperfect_act_2: "ἐ~αιες", "ἐ~αιετον", "ἐ~αιετε";
    imperfect_act_3: "ἐ~αιε", "ἐ~αιετην", "ἐ~αιον";
    imperfect_mid_1: "ἐ~αιομην", _, "ἐ~αιομεθᾰ";
    imperfect_mid_2: "ἐ~αιου", _, _;
    imperfect_mid_3: "ἐ~αιετο", _, _;
    imperfect_pass_2: _, "ἐ~αιεσθον", "ἐ~αιεσθε";
    imperfect_pass_3: _, "ἐ~αιεσθην", "ἐ~αιοντο";
    future_ind_act_1: "~αυσω", _, "~αυσομεν";
    future_ind_act_2: "~αυσεις", "~αυσετον", "~αυσετε";
    future_ind_act_3: _, _, "~αυσουσῐ";
    future_ind_mid_1: "~αυσομαι", _, "~αυσομεθᾰ";
    future_ind_mid_2: "~αυσῃ", "~αυσεσθον", _;
    future_ind_mid_3: "~αυσεται", _, _;
    future_opt_act_1: "~αυσοιμῐ", _, "~αυσοιμεν";
    future_opt_act_2: _, "~αυσοιτον", "~αυσοιτε";
    future_opt_act_3: "~αυσοι", "~αυσοιτην", "~αυσοιεν";
    future_opt_mid_1: "~αυσοιμην", _, "~αυσοιμεθᾰ";
    future_opt_mid_2: "~αυσοιο", _, _;
    future_opt_mid_3: _, "~αυσοισθην", _;
    aorist_subj_act_1: "~αυσω", _, _;
    aorist_subj_act_2: "~αυσῃς", "~αυσητον", _;
    aorist_subj_mid_1: "~αυσωμαι", _, _;
    aorist_subj_mid_2: "~αυσῃ", "~αυσησθον", _;
    aorist_opt_act_1: "~αυσαιμῐ", _, "~αυσαιμεν";
    aorist_opt_act_2: "~αυσειᾰς", _, _;
    aorist_opt_act_3: "~αυσειε", "~αυσαιτην", "~αυσειᾰν";
    aorist_opt_mid_1: "~αυσαιμην", _, _;
    aorist_opt_mid_2: "~αυσαιο", _, _;
    aorist_imp_act_2: "~αῡσον", _, "~αυσᾰτε";
    aorist_imp_act_3: _, _, "~αυσᾰντων";
    aorist_imp_mid_2: "~αῡσαι", "~αυσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~αυσᾰσθων";
    perfect_ind_act_1: "~εκαυκᾰ", _, "~εκαυκᾰμεν";
    perfect_ind_act_2: _, "~εκαυκᾰτον", _;
    perfect_ind_act_3: _, _, "~εκαυκᾱσῐ";
    perfect_ind_mid_1: "~εκαυμαι", _, "~εκαυμεθᾰ";
    perfect_ind_mid_2: "~εκαυσαι", "~εκαυσθον", _;
    perfect_ind_mid_3: "~εκαυται", _, "~εκαυνται";
    perfect_ind_pass_2: _, _, "~εκαυσθε";
    perfect_subj_act_1: "~εκαυκω", _, _;
    perfect_subj_act_2: "~εκαυκῃς", "~εκαυκητον", "~εκαυκητε";
    perfect_subj_act_3: "~εκαυκῃ", _, "~εκαυκωσῐ";
    perfect_opt_act_1: "~εκαυκοιμῐ", _, "~εκαυκοιμεν";
    perfect_opt_act_2: "~εκαυκοις", _, _;
    perfect_opt_act_3: _, "~εκαυκοιτην", _;
    perfect_imp_act_2: "~εκαυκε", _, "~εκαυκετε";
    perfect_imp_act_3: _, _, "~εκαυκοντων";
    perfect_imp_mid_2: _, _, "~εκαυσθε";
    perfect_imp_mid_3: "~εκαυσθω", _, "~εκαυσθων";
    perfect_imp_pass_2: "~εκαυσο", _, "~εκαυσθε";
    perfect_imp_pass_3: _, "~εκαυσθων", "~εκαυσθων";
    pluperfect_mid_1: "ἐ~εκαυμην", _, _;
    pluperfect_mid_2: "ἐ~εκαυσο", "ἐ~εκαυσθον", _;
    pluperfect_mid_3: "ἐ~εκαυτο", _, "ἐ~εκαυντο";
    pluperfect_pass_1: _, _, "ἐ~εκαυμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~εκαυσθε";
    pluperfect_pass_3: _, "ἐ~εκαυσθην", _;
    infinitive_pres: "~αιειν", _, "~αιεσθαι";
    infinitive_fut: "~αυσειν", _, _;
    infinitive_aor: "~αῡσαι", _, _;
    infinitive_perf: _, "~εκαῡσθαι", _;
    participle_pres: "~αιων", _, "~αιομενος";
    participle_aor: "~αυσᾱς", "~αυσᾰμενος", _;
    participle_perf: "~εκαυκως", _, "~εκαυμενος";
    ),
    // Προσφερω: 4 words (προφερω, προσφερω, περῐφερω, ὑπερφερω). Like Μῑγνῡμῐ, except in 134 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("προσφερω", 4, "~φερω", like Μῑγνῡμῐ;
    stem future_pass: "~οισθησ- like ἐχω";
    stem aorist_ind: "~ηνεγκ- like ἐλπω";
    stem aorist_ind_pass: "~ηνεχθην- like ἐλπω";
    stem aorist_pass: "~ενεχθ- like ἐχω";
    ind_act_1: "~φερω", _, "~φερομεν";
    ind_act_2: "~φερεις", _, _;
    ind_act_3: "~φερει", "~φερετον", "~φερουσῐ|~φερουσῐν";
    ind_mid_1: "~φερομαι", _, "~φερομεθᾰ";
    ind_mid_2: "~φερῃ|~φερει", "~φερεσθον", _;
    ind_pass_2: _, _, "~φερεσθε";
    ind_pass_3: "~φερεται", _, "~φερονται";
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
    opt_mid_2: "~φεροιο", "~φεροισθον", _;
    opt_mid_3: "~φεροιτο", "~φεροισθην", "~φεροιντο";
    opt_pass_2: _, _, "~φεροισθε";
    imp_act_2: "~φερε", "~φερετον", "~φερετε";
    imp_mid_2: "~φερου", _, "~φερεσθε";
    imp_mid_3: "~φερεσθω", _, "~φερεσθων";
    imp_pass_2: _, _, "~φερεσθε";
    imp_pass_3: _, "~φερεσθων", "~φερεσθων";
    imperfect_act_1: "~εφερον", _, "~εφερομεν";
    imperfect_act_2: "~εφερες", "~εφερετον", "~εφερετε";
    imperfect_act_3: "~εφερε|~εφερεν", "~εφερετην", "~εφερον";
    imperfect_mid_1: "~εφερομην", _, "~εφερομεθᾰ";
    imperfect_mid_2: "~εφερου", _, _;
    imperfect_mid_3: "~εφερετο", _, _;
    imperfect_pass_2: _, "~εφερεσθον", "~εφερεσθε";
    imperfect_pass_3: _, "~εφερεσθην", "~εφεροντο";
    future_ind_act_1: "~οισω", _, "~οισομεν";
    future_ind_act_2: "~οισεις", "~οισετον", "~οισετε";
    future_ind_act_3: _, _, "~οισουσῐ|~οισουσῐν";
    future_ind_mid_1: "~οισομαι", _, "~οισομεθᾰ";
    future_ind_mid_2: "~οισῃ|~οισει", "~οισεσθον", _;
    future_ind_mid_3: "~οισεται", _, _;
    future_opt_act_1: "~οισοιμῐ", _, "~οισοιμεν";
    future_opt_act_2: _, "~οισοιτον", "~οισοιτε";
    future_opt_act_3: "~οισοι", "~οισοιτην", "~οισοιεν";
    future_opt_mid_1: "~οισοιμην", _, "~οισοιμεθᾰ";
    future_opt_mid_2: "~οισοιο", _, _;
    future_opt_mid_3: _, "~οισοισθην", _;
    aorist_subj_act_1: "~ενεγκω", _, _;
    aorist_subj_act_2: "~ενεγκῃς", "~ενεγκητον", _;
    aorist_subj_mid_1: "~ενεγκωμαι", _, _;
    aorist_subj_mid_2: "~ενεγκῃ", "~ενεγκησθον", _;
    aorist_opt_act_1: "~ενεγκαιμῐ", _, "~ενεγκαιμεν";
    aorist_opt_act_2: "~ενεγκειᾰς", _, _;
    aorist_opt_act_3: "~ενεγκειε", "~ενεγκαιτην", "~ενεγκειᾰν";
    aorist_opt_mid_1: "~ενεγκαιμην", _, _;
    aorist_opt_mid_2: "~ενεγκαιο", _, _;
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
    perfect_opt_act_1: "~ενηνεχοιμῐ", _, "~ενηνεχοιμεν";
    perfect_opt_act_2: "~ενηνεχοις", _, _;
    perfect_opt_act_3: _, "~ενηνεχοιτην", _;
    perfect_imp_act_2: "~ενηνεχε", _, "~ενηνεχετε";
    perfect_imp_act_3: _, _, "~ενηνεχοντων";
    infinitive_pres: "~φερειν", _, "~φερεσθαι";
    infinitive_fut: "~οισειν", _, _;
    infinitive_aor: "~ενεγκαι", _, _;
    participle_pres: "~φερων", _, "~φερομενος";
    participle_aor: "~ενεγκᾱς", "~ενεγκᾰμενος", _;
    participle_perf: "~ενηνεχως", _, _;
    ),
    // Ἐπιτρεχω: 4 words (περῐτρεχω, κᾰτᾰτρεχω, ἐπιτρεχω, ἀνατρεχω). Like Φειδομαι, except in 158 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐπιτρεχω", 4, "~τ~εχω", like Φειδομαι;
    stem aorist_ind: "~εθ~εξ- like ἐλπω";
    ind_act_1: "~τ~εχω", _, "~τ~εχομεν";
    ind_act_2: "~τ~εχεις", "~τ~εχετον", _;
    ind_act_3: "~τ~εχει", _, "~τ~εχουσῐ";
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, derived, _;
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
    opt_mid_2: derived, derived, _;
    opt_mid_3: derived, derived, derived;
    opt_pass_1: "~τ~εχοιμην", _, "~τ~εχοιμεθᾰ";
    opt_pass_2: "~τ~εχοιο", "~τ~εχοισθον", "~τ~εχοισθε";
    opt_pass_3: "~τ~εχοιτο", "~τ~εχοισθην", "~τ~εχοιντο";
    imp_act_2: "~τ~εχε", _, "~τ~εχετε";
    imp_act_3: "~τ~εχετω", _, "~τ~εχοντων";
    imp_mid_2: derived, _, derived;
    imp_mid_3: derived, _, derived;
    imp_pass_2: "~τ~εχου", "~τ~εχεσθον", "~τ~εχεσθε";
    imp_pass_3: "~τ~εχεσθω", "~τ~εχεσθων", "~τ~εχεσθων";
    imperfect_act_1: "~ετ~εχον", _, "~ετ~εχομεν";
    imperfect_act_2: "~ετ~εχες", "~ετ~εχετον", "~ετ~εχετε";
    imperfect_act_3: "~ετ~εχε|~ετ~εχεν", "~ετ~εχετην", "~ετ~εχον";
    imperfect_mid_1: "~ετ~εχομην", _, "~ετ~εχομεθᾰ";
    imperfect_mid_2: "~ετ~εχου", derived, _;
    imperfect_mid_3: "~ετ~εχετο", derived, derived;
    imperfect_pass_2: _, "~ετ~εχεσθον", "~ετ~εχεσθε";
    imperfect_pass_3: _, "~ετ~εχεσθην", "~ετ~εχοντο";
    future_ind_mid_1: "~δ~ᾰμοῡμαι", _, "~δ~ᾰμουμεθᾰ";
    future_ind_mid_2: "~δ~ᾰμεῑ", "~δ~ᾰμεῑσθον", _;
    future_ind_mid_3: "~δ~ᾰμεῑται", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_mid_1: "~δ~ᾰμοιμην", _, "~δ~ᾰμοιμεθᾰ";
    future_opt_mid_2: "~δ~ᾰμοῑο", _, _;
    future_opt_mid_3: _, "~δ~ᾰμοισθην", _;
    aorist_subj_act_1: "~θ~εξω", _, _;
    aorist_subj_act_2: "~θ~εξῃς", "~θ~εξητον", _;
    aorist_subj_mid_1: "~θ~εξωμαι", _, _;
    aorist_subj_mid_2: "~θ~εξῃ", "~θ~εξησθον", _;
    aorist_opt_act_1: "~θ~εξαιμῐ", _, "~θ~εξαιμεν";
    aorist_opt_act_2: "~θ~εξειᾰς", _, _;
    aorist_opt_act_3: "~θ~εξειε", "~θ~εξαιτην", "~θ~εξειᾰν";
    aorist_opt_mid_1: "~θ~εξαιμην", _, _;
    aorist_opt_mid_2: "~θ~εξαιο", _, _;
    aorist_imp_act_2: "~θ~εξον", _, "~θ~εξᾰτε";
    aorist_imp_act_3: _, _, "~θ~εξᾰντων";
    aorist_imp_mid_2: "~θ~εξαι", "~θ~εξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~θ~εξᾰσθων";
    perfect_ind_act_1: "~δεδ~ᾰμηκᾰ", _, "~δεδ~ᾰμηκᾰμεν";
    perfect_ind_act_2: _, "~δεδ~ᾰμηκᾰτον", _;
    perfect_ind_act_3: _, _, "~δεδ~ᾰμηκᾱσῐ";
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_subj_act_1: "~δεδ~ᾰμηκω", _, _;
    perfect_subj_act_2: "~δεδ~ᾰμηκῃς", "~δεδ~ᾰμηκητον", "~δεδ~ᾰμηκητε";
    perfect_subj_act_3: "~δεδ~ᾰμηκῃ", _, "~δεδ~ᾰμηκωσῐ";
    perfect_opt_act_1: "~δεδ~ᾰμηκοιμῐ", _, "~δεδ~ᾰμηκοιμεν";
    perfect_opt_act_2: "~δεδ~ᾰμηκοις", _, _;
    perfect_opt_act_3: _, "~δεδ~ᾰμηκοιτην", _;
    perfect_imp_act_2: "~δεδ~ᾰμηκε", _, "~δεδ~ᾰμηκετε";
    perfect_imp_act_3: _, _, "~δεδ~ᾰμηκοντων";
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: derived, _, derived;
    infinitive_pres: "~τ~εχειν", _, derived;
    infinitive_aor: "~θ~εξαι", _, _;
    participle_pres: "~τ~εχων", _, derived;
    participle_aor: "~θ~εξᾱς", "~θ~εξᾰμενος", _;
    participle_perf: "~δεδ~ᾰμηκως", _, _;
    ),
    // Σπᾰω: 4 words (σπᾰω, κλᾰω, γελᾰω, ἀντῐᾰω). Like Ἐαω, except in 70 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 pl, ….
    conjugation!("σπᾰω", 4, "~ᾰω", like Ἐαω;
    stem future_pass: "~ᾰσθησ- like ἐχω";
    stem aorist_ind: "~ᾰσ- like ποιεω";
    stem aorist_ind_pass: "~ᾰσθην- like ἐχω";
    stem aorist_pass: "~ᾰσθ- like ἐχω";
    future_ind_act_1: "~ᾰσω", _, "~ᾰσομεν";
    future_ind_act_2: "~ᾰσεις", "~ᾰσετον", "~ᾰσετε";
    future_ind_act_3: _, _, "~ᾰσουσῐ|~ᾰσουσῐν";
    future_ind_mid_1: "~ᾰσομαι", _, "~ᾰσομεθᾰ";
    future_ind_mid_2: "~ᾰσῃ|~ᾰσει", "~ᾰσεσθον", _;
    future_ind_mid_3: "~ᾰσεται", _, _;
    future_opt_act_1: "~ᾰσοιμῐ", _, "~ᾰσοιμεν";
    future_opt_act_2: _, "~ᾰσοιτον", "~ᾰσοιτε";
    future_opt_act_3: "~ᾰσοι", "~ᾰσοιτην", "~ᾰσοιεν";
    future_opt_mid_1: "~ᾰσοιμην", _, "~ᾰσοιμεθᾰ";
    future_opt_mid_2: "~ᾰσοιο", _, _;
    future_opt_mid_3: _, "~ᾰσοισθην", _;
    aorist_subj_act_1: "~ᾰσω", _, _;
    aorist_subj_act_2: "~ᾰσῃς", "~ᾰσητον", _;
    aorist_subj_act_3: _, _, "~ᾰσωσῐ|~ᾰσωσῐν";
    aorist_subj_mid_1: "~ᾰσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰσῃ", "~ᾰσησθον", _;
    aorist_opt_act_1: "~ᾰσαιμῐ", _, "~ᾰσαιμεν";
    aorist_opt_act_2: "~ᾰσειᾰς|~ᾰσαις", _, _;
    aorist_opt_act_3: "~ᾰσειε|~ᾰσειεν|~ᾰσαι", "~ᾰσαιτην", "~ᾰσειᾰν|~ᾰσαιεν";
    aorist_opt_mid_1: "~ᾰσαιμην", _, _;
    aorist_opt_mid_2: "~ᾰσαιο", _, _;
    aorist_imp_act_2: "~ᾰσον", _, "~ᾰσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰσᾰντων";
    aorist_imp_mid_2: "~ᾰσαι", "~ᾰσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰσᾰσθων";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_1: "%ε~ᾰσμαι", _, "%ε~ᾰσμεθᾰ";
    perfect_ind_mid_3: "%ε~ᾰσται", _, "%ε~ᾰσᾰται";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: _, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
    infinitive_fut: "~ᾰσειν", _, _;
    infinitive_aor: "~ᾰσαι", _, _;
    participle_aor: "~ᾰσᾱς", "~ᾰσᾰμενος", _;
    participle_perf: derived, _, "%ε~ᾰσμενος";
    ),
    // Σῠνᾰγω: 4 words (σῠνᾰγω, προᾰγω, ξῠνᾰγω, προσᾰγω). Like Ἀνᾰγω, except in 56 cells: future_ind_act_3 pl, future_ind_mid_2 sg, future_ind_pass_2 sg, aorist_subj_act_1 sg, aorist_subj_act_1 pl, aorist_subj_act_2 sg, ….
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
    aorist_imp_mid_2: "~ᾰξαι|~ᾰγᾰγοῡ", "~ᾰξᾰσθον|~ᾰγᾰγεσθον", "~ᾰξᾰσθε|~ᾰγᾰγεσθε";
    aorist_imp_mid_3: "~ᾰξᾰσθω|~ᾰγᾰγεσθω", "~ᾰξᾰσθων|~ᾰγᾰγεσθων", derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: _, derived, derived;
    infinitive_aor: "~ᾰξαι|~ᾰγᾰγεῑν", "~ᾰξᾰσθαι|~ᾰγᾰγεσθαι", _;
    infinitive_perf: _, derived, _;
    participle_aor: "~ᾰξᾱς|~ᾰγᾰγων", "~ᾰξᾰμενος|~ᾰγᾰγομενος", _;
    participle_perf: _, _, derived;
    ),
    // Πειρω: 4 words (πειρω, κτεινω, σπειρω, φθειρω). Like Ἀειρω, except in 50 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, aorist_subj_act_3 pl, aorist_opt_act_2 sg, ….
    conjugation!("πειρω", 4, "~ει~ω", like Ἀειρω;
    stem future_pass: "~ᾰ~ησ- like ἐχω";
    stem aorist_ind: "~ει~- like ποιεω";
    stem aorist_ind_pass: "~ᾰ~ην- like ἐχω";
    stem aorist_pass: "~ᾰ~- like πειρω";
    stem pluperfect_act: "~ᾰ~κειν- like κᾰλεω";
    ind_act_3: _, _, "~ει~ουσῐ|~ει~ουσῐν";
    ind_mid_2: "~ει~ῃ|~ει~ει", _, _;
    subj_act_3: _, _, "~ει~ωσῐ|~ει~ωσῐν";
    imperfect_act_3: "ἐ~ει~ε|ἐ~ει~εν", _, _;
    aorist_subj_act_3: _, _, "~ει~ωσῐ|~ει~ωσῐν";
    aorist_opt_act_2: "~ει~ειᾰς|~ει~αις", _, _;
    aorist_opt_act_3: "~ει~ειε|~ει~ειεν|~ει~αι", _, "~ει~ειᾰν|~ει~αιεν";
    perfect_ind_act_1: "ἐ~ᾰ~κᾰ", _, "ἐ~ᾰ~κᾰμεν";
    perfect_ind_act_2: _, "ἐ~ᾰ~κᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~ᾰ~κᾱσῐ";
    perfect_ind_mid_1: "ἐ~ᾰ~μαι", _, "ἐ~ᾰ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰ~σαι", "ἐ~ᾰ~θον", _;
    perfect_ind_mid_3: "ἐ~ᾰ~ται", _, "ἐ~ᾰ~ᾰται";
    perfect_ind_pass_2: _, _, "ἐ~ᾰ~θε";
    perfect_subj_act_1: "ἐ~ᾰ~κω", _, _;
    perfect_subj_act_2: "ἐ~ᾰ~κῃς", "ἐ~ᾰ~κητον", "ἐ~ᾰ~κητε";
    perfect_subj_act_3: "ἐ~ᾰ~κῃ", _, "ἐ~ᾰ~κωσῐ";
    perfect_opt_act_1: "ἐ~ᾰ~κοιμῐ", _, "ἐ~ᾰ~κοιμεν";
    perfect_opt_act_2: "ἐ~ᾰ~κοις", _, _;
    perfect_opt_act_3: _, "ἐ~ᾰ~κοιτην", _;
    perfect_imp_act_2: "ἐ~ᾰ~κε", _, "ἐ~ᾰ~κετε";
    perfect_imp_act_3: _, _, "ἐ~ᾰ~κοντων";
    perfect_imp_mid_2: _, _, "ἐ~ᾰ~θε";
    perfect_imp_mid_3: "ἐ~ᾰ~θω", _, "ἐ~ᾰ~θων";
    perfect_imp_pass_2: "ἐ~ᾰ~σο", _, "ἐ~ᾰ~θε";
    perfect_imp_pass_3: _, "ἐ~ᾰ~θων", "ἐ~ᾰ~θων";
    pluperfect_mid_1: "ἐ~ᾰ~μην", _, _;
    pluperfect_mid_2: "ἐ~ᾰ~σο", "ἐ~ᾰ~θον", _;
    pluperfect_mid_3: "ἐ~ᾰ~το", _, "ἐ~ᾰ~ᾰτο";
    pluperfect_pass_1: _, _, "ἐ~ᾰ~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ᾰ~θε";
    pluperfect_pass_3: _, "ἐ~ᾰ~θην", _;
    infinitive_perf: _, "ἐ~ᾰ~θαι", _;
    participle_perf: "ἐ~ᾰ~κως", _, "ἐ~ᾰ~μενος";
    ),
    // Σᾰττω: 4 words (φῠλᾰττω, πληττω, σᾰττω, ἀλλᾰττω). Like Ληγω, except in 90 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("σᾰττω", 4, "~ττω", like Ληγω;
    stem future_pass: "~γησ- like ἐχω";
    stem aorist_ind_pass: "~γην- like ἐχω";
    stem aorist_pass: "~γ- like σφᾰλλω";
    ind_act_1: "~ττω", _, "~ττομεν";
    ind_act_2: "~ττεις", _, _;
    ind_act_3: "~ττει", "~ττετον", "~ττουσῐ|~ττουσῐν";
    ind_mid_1: "~ττομαι", _, "~ττομεθᾰ";
    ind_mid_2: "~ττῃ|~ττει", "~ττεσθον", _;
    ind_pass_2: _, _, "~ττεσθε";
    ind_pass_3: "~ττεται", _, "~ττονται";
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
    opt_mid_2: "~ττοιο", "~ττοισθον", _;
    opt_mid_3: "~ττοιτο", "~ττοισθην", "~ττοιντο";
    opt_pass_2: _, _, "~ττοισθε";
    imp_act_2: "~ττε", "~ττετον", "~ττετε";
    imp_mid_2: "~ττου", _, "~ττεσθε";
    imp_mid_3: "~ττεσθω", _, "~ττεσθων";
    imp_pass_2: _, _, "~ττεσθε";
    imp_pass_3: _, "~ττεσθων", "~ττεσθων";
    imperfect_act_1: "ἐ~ττον", _, "ἐ~ττομεν";
    imperfect_act_2: "ἐ~ττες", "ἐ~ττετον", "ἐ~ττετε";
    imperfect_act_3: "ἐ~ττε|ἐ~ττεν", "ἐ~ττετην", "ἐ~ττον";
    imperfect_mid_1: "ἐ~ττομην", _, "ἐ~ττομεθᾰ";
    imperfect_mid_2: "ἐ~ττου", _, _;
    imperfect_mid_3: "ἐ~ττετο", _, _;
    imperfect_pass_2: _, "ἐ~ττεσθον", "ἐ~ττεσθε";
    imperfect_pass_3: _, "ἐ~ττεσθην", "ἐ~ττοντο";
    future_ind_act_3: _, _, "~ξουσῐ|~ξουσῐν";
    future_ind_mid_2: "~ξῃ|~ξει", _, _;
    perfect_ind_act_1: "%ε~γᾰ", _, "%ε~γᾰμεν";
    perfect_ind_act_2: _, "%ε~γᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~γᾱσῐ";
    perfect_subj_act_1: "%ε~γω", _, _;
    perfect_subj_act_2: "%ε~γῃς", "%ε~γητον", "%ε~γητε";
    perfect_subj_act_3: "%ε~γῃ", _, "%ε~γωσῐ";
    perfect_opt_act_1: "%ε~γοιμῐ", _, "%ε~γοιμεν";
    perfect_opt_act_2: "%ε~γοις", _, _;
    perfect_opt_act_3: _, "%ε~γοιτην", _;
    perfect_imp_act_2: "%ε~γε", _, "%ε~γετε";
    perfect_imp_act_3: _, _, "%ε~γοντων";
    infinitive_pres: "~ττειν", _, "~ττεσθαι";
    participle_pres: "~ττων", _, "~ττομενος";
    participle_perf: "%ε~γως", _, _;
    ),
    // Ἐργᾰζομαι: 4 words (στοχᾰζομαι, ἐργᾰζομαι, ἀσπᾰζομαι, βῐᾰζομαι). Like Ἀραομαι, except in 83 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ἐργᾰζομαι", 4, "~ζομαι", like Ἀραομαι;
    stem future_pass: "~σθησ- like ἐχω";
    stem aorist_ind: "~σ- like πᾰραιτεομαι";
    stem aorist_ind_pass: "~σθην- like ἐχω";
    stem aorist_pass: "~σθ- like ἐχω";
    ind_act_1: "~ζομαι", _, _;
    ind_mid_1: "~ζομαι", _, "~ζομεθᾰ";
    ind_mid_2: "~ζῃ", "~ζεσθον", _;
    ind_pass_2: _, _, "~ζεσθε";
    ind_pass_3: "~ζεται", _, "~ζονται";
    subj_mid_1: "~ζωμαι", _, "~ζωμεθᾰ";
    subj_mid_2: "~ζῃ", "~ζησθον", _;
    subj_mid_3: "~ζηται", _, "~ζωνται";
    subj_pass_2: "~ζῃ", _, "~ζησθε";
    opt_mid_1: "~ζοιμην", _, "~ζοιμεθᾰ";
    opt_mid_2: "~ζοιο", "~ζοισθον", _;
    opt_mid_3: "~ζοιτο", "~ζοισθην", "~ζοιντο";
    opt_pass_2: _, _, "~ζοισθε";
    imp_mid_2: "~ζου", _, "~ζεσθε";
    imp_mid_3: "~ζεσθω", _, "~ζεσθων";
    imp_pass_2: _, _, "~ζεσθε";
    imp_pass_3: _, "~ζεσθων", "~ζεσθων";
    imperfect_mid_1: "ἐ~ζομην", _, "ἐ~ζομεθᾰ";
    imperfect_mid_2: "ἐ~ζου", "ἐ~ζεσθον", _;
    imperfect_mid_3: "ἐ~ζετο", "ἐ~ζεσθην", "ἐ~ζοντο";
    imperfect_pass_2: _, _, "ἐ~ζεσθε";
    future_ind_mid_1: "~σομαι", _, "~σομεθᾰ";
    future_ind_mid_2: "~σῃ", "~σεσθον", _;
    future_ind_mid_3: "~σεται", _, _;
    future_opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    future_opt_mid_2: "~σοιο", _, _;
    future_opt_mid_3: _, "~σοισθην", _;
    aorist_subj_mid_1: "~σωμαι", _, _;
    aorist_subj_mid_2: "~σῃ", "~σησθον", _;
    aorist_opt_mid_1: "~σαιμην", _, _;
    aorist_opt_mid_2: "~σαιο", _, _;
    aorist_imp_mid_2: "~σαι", "~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~σᾰσθων";
    perfect_ind_mid_1: "%ε~σμαι", _, "%ε~σμεθᾰ";
    perfect_ind_mid_2: "%ε~σαι", "%ε~σθον", _;
    perfect_ind_mid_3: "%ε~σται", _, "%ε~δᾰται";
    perfect_ind_pass_2: _, _, "%ε~σθε";
    perfect_imp_mid_2: _, _, "%ε~σθε";
    perfect_imp_mid_3: "%ε~σθω", _, "%ε~σθων";
    perfect_imp_pass_2: "%ε~σο", _, "%ε~σθε";
    perfect_imp_pass_3: _, "%ε~σθων", "%ε~σθων";
    pluperfect_mid_1: "ἐ%ε~σμην", _, _;
    pluperfect_mid_2: "ἐ%ε~σο", "ἐ%ε~σθον", _;
    pluperfect_mid_3: "ἐ%ε~στο", _, "ἐ%ε~δᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~σθε";
    pluperfect_pass_3: _, "ἐ%ε~σθην", _;
    infinitive_pres: _, _, "~ζεσθαι";
    infinitive_perf: _, "%ε~σθαι", _;
    participle_pres: _, _, "~ζομενος";
    participle_aor: _, "~σᾰμενος", _;
    participle_perf: _, _, "%ε~σμενος";
    ),
    // Ἐρᾰμαι: 4 words (ἐρᾰμαι, μᾰρνᾰμαι, ἀγᾰμαι, λῐνᾰμαι). Like Ἱημῐ, except in 43 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐρᾰμαι", 4, "~ᾰμαι", like Ἱημῐ;
    ind_act_1: "~ᾰμαι", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, derived, derived;
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
    imp_act_2: derived, derived, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    imperfect_mid_2: _, "ἐ~ᾰσθον", _;
    imperfect_mid_3: _, "ἐ~ᾰσθην", "ἐ~ᾰντο";
    imperfect_pass_2: _, derived, _;
    imperfect_pass_3: _, derived, derived;
    infinitive_pres: derived, _, _;
    participle_pres: derived, _, _;
    ),
    // Ἀνᾰπτω: 4 words (ἀνᾰπτω, ἐξᾰπτω, καθᾰπτω, ἐπῐσῠνᾰπτω). Like Ληγω, except in 141 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀνᾰπτω", 4, "~ᾰπτω", like Ληγω;
    stem future_pass: "~ᾰφθησ- like ἐχω";
    stem aorist_ind: "~ηψ- like εὐλογεω";
    stem aorist_ind_pass: "~ηφθην- like ἐλπω";
    stem aorist_pass: "~ᾰφθ- like ποιεω";
    ind_act_1: "~ᾰπτω", _, "~ᾰπτομεν";
    ind_act_2: "~ᾰπτεις", _, _;
    ind_act_3: "~ᾰπτει", "~ᾰπτετον", "~ᾰπτουσῐ|~ᾰπτουσῐν";
    ind_mid_1: "~ᾰπτομαι", _, "~ᾰπτομεθᾰ";
    ind_mid_2: "~ᾰπτῃ|~ᾰπτει", "~ᾰπτεσθον", _;
    ind_pass_2: _, _, "~ᾰπτεσθε";
    ind_pass_3: "~ᾰπτεται", _, "~ᾰπτονται";
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
    opt_mid_2: "~ᾰπτοιο", "~ᾰπτοισθον", _;
    opt_mid_3: "~ᾰπτοιτο", "~ᾰπτοισθην", "~ᾰπτοιντο";
    opt_pass_2: _, _, "~ᾰπτοισθε";
    imp_act_2: "~ᾰπτε", "~ᾰπτετον", "~ᾰπτετε";
    imp_mid_2: "~ᾰπτου", _, "~ᾰπτεσθε";
    imp_mid_3: "~ᾰπτεσθω", _, "~ᾰπτεσθων";
    imp_pass_2: _, _, "~ᾰπτεσθε";
    imp_pass_3: _, "~ᾰπτεσθων", "~ᾰπτεσθων";
    imperfect_act_1: "~ηπτον", _, "~ηπτομεν";
    imperfect_act_2: "~ηπτες", "~ηπτετον", "~ηπτετε";
    imperfect_act_3: "~ηπτε|~ηπτεν", "~ηπτετην", "~ηπτον";
    imperfect_mid_1: "~ηπτομην", _, "~ηπτομεθᾰ";
    imperfect_mid_2: "~ηπτου", _, _;
    imperfect_mid_3: "~ηπτετο", _, _;
    imperfect_pass_2: _, "~ηπτεσθον", "~ηπτεσθε";
    imperfect_pass_3: _, "~ηπτεσθην", "~ηπτοντο";
    future_ind_act_1: "~ᾰψω", _, "~ᾰψομεν";
    future_ind_act_2: "~ᾰψεις", "~ᾰψετον", "~ᾰψετε";
    future_ind_act_3: _, _, "~ᾰψουσῐ|~ᾰψουσῐν";
    future_ind_mid_1: "~ᾰψομαι", _, "~ᾰψομεθᾰ";
    future_ind_mid_2: "~ᾰψῃ|~ᾰψει", "~ᾰψεσθον", _;
    future_ind_mid_3: "~ᾰψεται", _, _;
    future_opt_act_1: "~ᾰψοιμῐ", _, "~ᾰψοιμεν";
    future_opt_act_2: _, "~ᾰψοιτον", "~ᾰψοιτε";
    future_opt_act_3: "~ᾰψοι", "~ᾰψοιτην", "~ᾰψοιεν";
    future_opt_mid_1: "~ᾰψοιμην", _, "~ᾰψοιμεθᾰ";
    future_opt_mid_2: "~ᾰψοιο", _, _;
    future_opt_mid_3: _, "~ᾰψοισθην", _;
    aorist_subj_act_1: "~ᾰψω", _, _;
    aorist_subj_act_2: "~ᾰψῃς", "~ᾰψητον", _;
    aorist_subj_act_3: _, _, "~ᾰψωσῐ|~ᾰψωσῐν";
    aorist_subj_mid_1: "~ᾰψωμαι", _, _;
    aorist_subj_mid_2: "~ᾰψῃ", "~ᾰψησθον", _;
    aorist_opt_act_1: "~ᾰψαιμῐ", _, "~ᾰψαιμεν";
    aorist_opt_act_2: "~ᾰψειᾰς|~ᾰψαις", _, _;
    aorist_opt_act_3: "~ᾰψειε|~ᾰψειεν|~ᾰψαι", "~ᾰψαιτην", "~ᾰψειᾰν|~ᾰψαιεν";
    aorist_opt_mid_1: "~ᾰψαιμην", _, _;
    aorist_opt_mid_2: "~ᾰψαιο", _, _;
    aorist_imp_act_2: "~ᾰψον", _, "~ᾰψᾰτε";
    aorist_imp_act_3: _, _, "~ᾰψᾰντων";
    aorist_imp_mid_2: "~ᾰψαι", "~ᾰψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰψᾰσθων";
    perfect_ind_mid_1: "~ημμαι", _, "~ημμεθᾰ";
    perfect_ind_mid_2: "~ηψαι", "~ηφθον", _;
    perfect_ind_mid_3: "~ηπται", _, "~ηπᾰται";
    perfect_ind_pass_2: _, _, "~ηφθε";
    perfect_imp_mid_2: _, _, "~ηφθε";
    perfect_imp_mid_3: "~ηφθω", _, "~ηφθων";
    perfect_imp_pass_2: "~ηψο", _, "~ηφθε";
    perfect_imp_pass_3: _, "~ηφθων", "~ηφθων";
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
    // Πειρᾰω: 4 words (κοπῐᾰω, ἀᾰω, πειρᾰω, μειδῐᾰω). Like Ἐαω, except in 24 cells: future_ind_mid_2 sg, future_ind_pass_1 sg, future_ind_pass_3 du, perfect_ind_mid_1 sg, perfect_ind_mid_1 pl, perfect_ind_mid_2 sg, ….
    conjugation!("πειρᾰω", 4, "~ᾰω", like Ἐαω;
    future_ind_mid_2: "~ᾱσῃ", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    perfect_ind_mid_1: "%ε~ᾱμαι", _, "%ε~ᾱμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾱσαι", "%ε~ᾱσθον", _;
    perfect_ind_mid_3: "%ε~ᾱται", _, "%ε~ᾱνται";
    perfect_ind_pass_2: _, _, "%ε~ᾱσθε";
    perfect_imp_mid_2: _, _, "%ε~ᾱσθε";
    perfect_imp_mid_3: "%ε~ᾱσθω", _, "%ε~ᾱσθων";
    perfect_imp_pass_2: "%ε~ᾱσο", _, "%ε~ᾱσθε";
    perfect_imp_pass_3: _, "%ε~ᾱσθων", "%ε~ᾱσθων";
    pluperfect_mid_1: "ἐ%ε~ᾱμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ᾱσο", "ἐ%ε~ᾱσθον", _;
    pluperfect_mid_3: "ἐ%ε~ᾱτο", _, "ἐ%ε~ᾱντο";
    infinitive_perf: _, "%ε~ᾱσθαι", _;
    participle_perf: _, _, "%ε~ᾱμενος";
    ),
    // Πᾰρᾰλλᾰσσω: 4 words (ἀπᾰλλᾰσσω, ὑπᾰλλᾰσσω, ἀντᾰλλᾰσσω, πᾰρᾰλλᾰσσω). Like Ξαινω, except in 176 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("πᾰρᾰλλᾰσσω", 4, "~ᾰ~σσω", like Ξαινω;
    stem future_pass: "~ᾰ~χθησ- like ποιεω";
    stem aorist_ind: "~η~ξ- like ἐλπω";
    stem aorist_ind_pass: "~η~- like πᾰρᾰλλᾰσσω";
    ind_act_1: "~ᾰ~σσω", _, "~ᾰ~σσομεν";
    ind_act_2: "~ᾰ~σσεις", _, _;
    ind_act_3: "~ᾰ~σσει", "~ᾰ~σσετον", "~ᾰ~σσουσῐ|~ᾰ~σσουσῐν";
    ind_mid_1: "~ᾰ~σσομαι", _, "~ᾰ~σσομεθᾰ";
    ind_mid_2: "~ᾰ~σσῃ|~ᾰ~σσει", "~ᾰ~σσεσθον", _;
    ind_pass_2: _, _, "~ᾰ~σσεσθε";
    ind_pass_3: "~ᾰ~σσεται", _, "~ᾰ~σσονται";
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
    opt_mid_2: "~ᾰ~σσοιο", "~ᾰ~σσοισθον", _;
    opt_mid_3: "~ᾰ~σσοιτο", "~ᾰ~σσοισθην", "~ᾰ~σσοιντο";
    opt_pass_2: _, _, "~ᾰ~σσοισθε";
    imp_act_2: "~ᾰ~σσε", "~ᾰ~σσετον", "~ᾰ~σσετε";
    imp_mid_2: "~ᾰ~σσου", _, "~ᾰ~σσεσθε";
    imp_mid_3: "~ᾰ~σσεσθω", _, "~ᾰ~σσεσθων";
    imp_pass_2: _, _, "~ᾰ~σσεσθε";
    imp_pass_3: _, "~ᾰ~σσεσθων", "~ᾰ~σσεσθων";
    imperfect_act_1: "~η~σσον", _, "~η~σσομεν";
    imperfect_act_2: "~η~σσες", "~η~σσετον", "~η~σσετε";
    imperfect_act_3: "~η~σσε|~η~σσεν", "~η~σσετην", "~η~σσον";
    imperfect_mid_1: "~η~σσομην", _, "~η~σσομεθᾰ";
    imperfect_mid_2: "~η~σσου", _, _;
    imperfect_mid_3: "~η~σσετο", _, _;
    imperfect_pass_2: _, "~η~σσεσθον", "~η~σσεσθε";
    imperfect_pass_3: _, "~η~σσεσθην", "~η~σσοντο";
    future_ind_act_1: "~ᾰ~ξω", _, "~ᾰ~ξομεν";
    future_ind_act_2: "~ᾰ~ξεις", "~ᾰ~ξετον", "~ᾰ~ξετε";
    future_ind_act_3: _, _, "~ᾰ~ξουσῐ|~ᾰ~ξουσῐν";
    future_ind_mid_1: "~ᾰ~ξομαι", _, "~ᾰ~ξομεθᾰ";
    future_ind_mid_2: "~ᾰ~ξῃ|~ᾰ~ξει", "~ᾰ~ξεσθον", _;
    future_ind_mid_3: "~ᾰ~ξεται", _, _;
    future_opt_act_1: "~ᾰ~ξοιμῐ", _, "~ᾰ~ξοιμεν";
    future_opt_act_2: _, "~ᾰ~ξοιτον", "~ᾰ~ξοιτε";
    future_opt_act_3: "~ᾰ~ξοι", "~ᾰ~ξοιτην", "~ᾰ~ξοιεν";
    future_opt_mid_1: "~ᾰ~ξοιμην", _, "~ᾰ~ξοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ξοιο", _, _;
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
    aorist_opt_act_3: "~ᾰ~ξειε", "~ᾰ~ξαιτην", "~ᾰ~ξειᾰν";
    aorist_opt_mid_1: "~ᾰ~ξαιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~ξαιο", _, _;
    aorist_opt_pass_1: "~ᾰ~χθειην|~ᾰ~γειην", _, "~ᾰ~χθεῑμεν|~ᾰ~χθειημεν|~ᾰ~γεῑμεν|~ᾰ~γειημεν";
    aorist_opt_pass_2: "~ᾰ~χθειης|~ᾰ~γειης", "~ᾰ~χθεῑτον|~ᾰ~χθειητον|~ᾰ~γεῑτον|~ᾰ~γειητον", "~ᾰ~χθεῑτε|~ᾰ~χθειητε|~ᾰ~γεῑτε|~ᾰ~γειητε";
    aorist_opt_pass_3: "~ᾰ~χθειη|~ᾰ~γειη", "~ᾰ~χθειτην|~ᾰ~χθειητην|~ᾰ~γειτην|~ᾰ~γειητην", "~ᾰ~χθεῑεν|~ᾰ~χθειησᾰν|~ᾰ~γεῑεν|~ᾰ~γειησᾰν";
    aorist_imp_act_2: "~ᾰ~ξον", _, "~ᾰ~ξᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ξᾰντων";
    aorist_imp_mid_2: "~ᾰ~ξαι", "~ᾰ~ξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ξᾰσθων";
    aorist_imp_pass_2: "~ᾰ~χθητῐ|~ᾰ~γηθῐ", derived, derived;
    aorist_imp_pass_3: "~ᾰ~χθητω|~ᾰ~γητω", "~ᾰ~χθητων|~ᾰ~γητων", "~ᾰ~χθεντων|~ᾰ~γεντων";
    perfect_ind_act_1: "~η~χᾰ", _, "~η~χᾰμεν";
    perfect_ind_act_2: _, "~η~χᾰτον", _;
    perfect_ind_act_3: "~η~χε|~η~χεν", _, "~η~χᾱσῐ|~η~χᾱσῐν";
    perfect_ind_mid_1: "~η~γμαι", _, "~η~γμεθᾰ";
    perfect_ind_mid_2: "~η~ξαι", "~η~χθον", _;
    perfect_ind_mid_3: "~η~κται", _, "~η~χᾰται";
    perfect_ind_pass_2: _, _, "~η~χθε";
    perfect_subj_act_1: "~η~χω", _, _;
    perfect_subj_act_2: "~η~χῃς", "~η~χητον", "~η~χητε";
    perfect_subj_act_3: "~η~χῃ", _, "~η~χωσῐ|~η~χωσῐν";
    perfect_opt_act_1: "~η~χοιμῐ|~η~χοιην", _, "~η~χοιμεν";
    perfect_opt_act_2: "~η~χοις|~η~χοιης", _, _;
    perfect_opt_act_3: "~η~χοι|~η~χοιη", "~η~χοιτην", _;
    perfect_imp_act_2: "~η~χε", _, "~η~χετε";
    perfect_imp_act_3: _, _, "~η~χοντων";
    perfect_imp_mid_2: _, _, "~η~χθε";
    perfect_imp_mid_3: "~η~χθω", _, "~η~χθων";
    perfect_imp_pass_2: "~η~ξο", _, "~η~χθε";
    perfect_imp_pass_3: _, "~η~χθων", "~η~χθων";
    infinitive_pres: "~ᾰ~σσειν", _, "~ᾰ~σσεσθαι";
    infinitive_fut: "~ᾰ~ξειν", _, _;
    infinitive_aor: "~ᾰ~ξαι", _, "~ᾰ~χθηναι|~ᾰ~γηναι";
    infinitive_perf: _, "~η~χθαι", _;
    participle_pres: "~ᾰ~σσων", _, "~ᾰ~σσομενος";
    participle_aor: "~ᾰ~ξᾱς", "~ᾰ~ξᾰμενος", "~ᾰ~χθεις|~ᾰ~γεις";
    participle_perf: "~η~χως", _, "~η~γμενος";
    ),
    // Κᾰτειργω: 4 words (ἐργω, εἰργω, ἀμεργω, κᾰτειργω). Like Ληγω, except in 45 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, ….
    conjugation!("κᾰτειργω", 4, "~γω", like Ληγω;
    stem aorist_ind: "~ξ- like ἐλπω";
    stem aorist_ind_pass: "~χθην- like ἐλπω";
    ind_act_3: _, _, "~γουσῐ|~γουσῐν";
    ind_mid_2: "~γῃ|~γει", _, _;
    subj_act_3: _, _, "~γωσῐ|~γωσῐν";
    imperfect_act_1: "~γον", _, "~γομεν";
    imperfect_act_2: "~γες", "~γετον", "~γετε";
    imperfect_act_3: "~γε", "~γετην", "~γον";
    imperfect_mid_1: "~γομην", _, "~γομεθᾰ";
    imperfect_mid_2: "~γου", _, _;
    imperfect_mid_3: "~γετο", _, _;
    imperfect_pass_2: _, "~γεσθον", "~γεσθε";
    imperfect_pass_3: _, "~γεσθην", "~γοντο";
    future_ind_act_3: _, _, "~ξουσῐ|~ξουσῐν";
    future_ind_mid_2: "~ξῃ|~ξει", _, _;
    perfect_ind_mid_1: "~γμαι", _, "~γμεθᾰ";
    perfect_ind_mid_2: "~ξαι", "~χθον", _;
    perfect_ind_mid_3: "~κται", _, "~χᾰται";
    perfect_ind_pass_2: _, _, "~χθε";
    perfect_imp_mid_2: _, _, "~χθε";
    perfect_imp_mid_3: "~χθω", _, "~χθων";
    perfect_imp_pass_2: "~ξο", _, "~χθε";
    perfect_imp_pass_3: _, "~χθων", "~χθων";
    pluperfect_mid_1: "~γμην", _, _;
    pluperfect_mid_2: "~ξο", "~χθον", _;
    pluperfect_mid_3: "~κτο", _, "~χᾰτο";
    pluperfect_pass_1: _, _, "~γμεθᾰ";
    pluperfect_pass_2: _, _, "~χθε";
    pluperfect_pass_3: _, "~χθην", _;
    infinitive_perf: _, "~χθαι", _;
    participle_perf: _, _, "~γμενος";
    ),
    // Περαινω: 4 words (μᾰραινω, πημαινω, πῑαινω, περαινω). Like Ῥαινω, except in 48 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imp_act_2 sg, imperfect_act_1 sg, imperfect_act_1 pl, ….
    conjugation!("περαινω", 4, "~αινω", like Ῥαινω;
    stem aorist_ind_pass: "~ᾰνθην- like ἐχω";
    stem aorist_pass: "~ᾰνθ- like ποιεω";
    ind_act_3: _, _, "~αινουσῐ|~αινουσῐν";
    ind_mid_2: "~αινῃ|~αινει", _, _;
    subj_act_3: _, _, "~αινωσῐ|~αινωσῐν";
    imp_act_2: "~αινε", _, _;
    imperfect_act_1: "ε~αινον", _, "ε~αινομεν";
    imperfect_act_2: "ε~αινες", "ε~αινετον", "ε~αινετε";
    imperfect_act_3: "ε~αινε", "ε~αινετην", "ε~αινον";
    imperfect_mid_1: "ε~αινομην", _, "ε~αινομεθᾰ";
    imperfect_mid_2: "ε~αινου", _, _;
    imperfect_mid_3: "ε~αινετο", _, _;
    imperfect_pass_2: _, "ε~αινεσθον", "ε~αινεσθε";
    imperfect_pass_3: _, "ε~αινεσθην", "ε~αινοντο";
    future_ind_act_3: _, _, "~ᾰνοῡσῐ|~ᾰνοῡσῐν";
    future_ind_mid_2: "~ᾰνεῑ|~ᾰνῃ", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: "~ᾰνοιην|~ᾰνοῑμῐ", _, "~ᾰνοῑμεν|~ᾰνοιημεν";
    future_opt_act_2: "~ᾰνοιης|~ᾰνοῑς", "~ᾰνοῑτον|~ᾰνοιητον", "~ᾰνοῑτε|~ᾰνοιητε";
    future_opt_act_3: "~ᾰνοιη|~ᾰνοῑ", "~ᾰνοιτην|~ᾰνοιητην", "~ᾰνοῑεν|~ᾰνοιησᾰν";
    perfect_ind_mid_1: "%ε~ᾰσμαι", _, "%ε~ᾰσμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾰνσαι", "%ε~ᾰνθον", _;
    perfect_ind_mid_3: "%ε~ᾰνται", _, "%ε~ᾰνᾰται";
    perfect_ind_pass_2: _, _, "%ε~ᾰνθε";
    perfect_imp_mid_2: _, _, "%ε~ᾰνθε";
    perfect_imp_mid_3: "%ε~ᾰνθω", _, "%ε~ᾰνθων";
    perfect_imp_pass_2: "%ε~ᾰνσο", _, "%ε~ᾰνθε";
    perfect_imp_pass_3: _, "%ε~ᾰνθων", "%ε~ᾰνθων";
    infinitive_perf: _, "%ε~ᾰνθαι", _;
    participle_perf: _, _, "%ε~ᾰσμενος";
    ),
    // Πᾰροξῡνω: 4 words (παροτρῡνω, ἐποτρῡνω, πᾰροξῡνω, ἐξοτρῡνω). Like Ξαινω, except in 150 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("πᾰροξῡνω", 4, "~ο~ῡνω", like Ξαινω;
    stem future_pass: "~ο~ῠνθησ- like ἐχω";
    stem aorist_ind: "~ω~ῡν- like ἐλπω";
    stem aorist_ind_pass: "~ω~ῠνθην- like ἐλπω";
    stem aorist_pass: "~ο~ῠνθ- like ἐχω";
    ind_act_1: "~ο~ῡνω", _, "~ο~ῡνομεν";
    ind_act_2: "~ο~ῡνεις", _, _;
    ind_act_3: "~ο~ῡνει", "~ο~ῡνετον", "~ο~ῡνουσῐ";
    ind_mid_1: "~ο~ῡνομαι", _, "~ο~ῡνομεθᾰ";
    ind_mid_2: "~ο~ῡνῃ", "~ο~ῡνεσθον", _;
    ind_pass_2: _, _, "~ο~ῡνεσθε";
    ind_pass_3: "~ο~ῡνεται", _, "~ο~ῡνονται";
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
    opt_mid_2: "~ο~ῡνοιο", "~ο~ῡνοισθον", _;
    opt_mid_3: "~ο~ῡνοιτο", "~ο~ῡνοισθην", "~ο~ῡνοιντο";
    opt_pass_2: _, _, "~ο~ῡνοισθε";
    imp_act_2: "~ο~ῡνε", "~ο~ῡνετον", "~ο~ῡνετε";
    imp_mid_2: "~ο~ῡνου", _, "~ο~ῡνεσθε";
    imp_mid_3: "~ο~ῡνεσθω", _, "~ο~ῡνεσθων";
    imp_pass_2: _, _, "~ο~ῡνεσθε";
    imp_pass_3: _, "~ο~ῡνεσθων", "~ο~ῡνεσθων";
    imperfect_act_1: "~ω~ῡνον", _, "~ω~ῡνομεν";
    imperfect_act_2: "~ω~ῡνες", "~ω~ῡνετον", "~ω~ῡνετε";
    imperfect_act_3: "~ω~ῡνε", "~ω~ῡνετην", "~ω~ῡνον";
    imperfect_mid_1: "~ω~ῡνομην", _, "~ω~ῡνομεθᾰ";
    imperfect_mid_2: "~ω~ῡνου", _, _;
    imperfect_mid_3: "~ω~ῡνετο", _, _;
    imperfect_pass_2: _, "~ω~ῡνεσθον", "~ω~ῡνεσθε";
    imperfect_pass_3: _, "~ω~ῡνεσθην", "~ω~ῡνοντο";
    future_ind_act_1: "~ο~ῠνω", _, "~ο~ῠνοῡμεν";
    future_ind_act_2: "~ο~ῠνεῑς", "~ο~ῠνεῑτον", "~ο~ῠνεῑτε";
    future_ind_act_3: _, _, "~ο~ῠνοῡσῐ";
    future_ind_mid_1: "~ο~ῠνοῡμαι", _, "~ο~ῠνουμεθᾰ";
    future_ind_mid_2: "~ο~ῠνεῑ", "~ο~ῠνεῑσθον", _;
    future_ind_mid_3: "~ο~ῠνεῑται", _, _;
    future_opt_act_1: "~ο~ῠνοιην", _, "~ο~ῠνοῑμεν";
    future_opt_act_2: _, "~ο~ῠνοῑτον", "~ο~ῠνοῑτε";
    future_opt_act_3: "~ο~ῠνοιη", "~ο~ῠνοιτην", "~ο~ῠνοῑεν";
    future_opt_mid_1: "~ο~ῠνοιμην", _, "~ο~ῠνοιμεθᾰ";
    future_opt_mid_2: "~ο~ῠνοῑο", _, _;
    future_opt_mid_3: _, "~ο~ῠνοισθην", _;
    aorist_subj_act_1: "~ο~ῡνω", _, _;
    aorist_subj_act_2: "~ο~ῡνῃς", "~ο~ῡνητον", _;
    aorist_subj_mid_1: "~ο~ῡνωμαι", _, _;
    aorist_subj_mid_2: "~ο~ῡνῃ", "~ο~ῡνησθον", _;
    aorist_opt_act_1: "~ο~ῡναιμῐ", _, "~ο~ῡναιμεν";
    aorist_opt_act_2: "~ο~ῡνειᾰς", _, _;
    aorist_opt_act_3: "~ο~ῡνειε", "~ο~ῡναιτην", "~ο~ῡνειᾰν";
    aorist_opt_mid_1: "~ο~ῡναιμην", _, _;
    aorist_opt_mid_2: "~ο~ῡναιο", _, _;
    aorist_imp_act_2: "~ο~ῡνον", _, "~ο~ῡνᾰτε";
    aorist_imp_act_3: _, _, "~ο~ῡνᾰντων";
    aorist_imp_mid_2: "~ο~ῡναι", "~ο~ῡνᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ο~ῡνᾰσθων";
    perfect_ind_act_1: "~ω~ῠγκᾰ", _, "~ω~ῠγκᾰμεν";
    perfect_ind_act_2: _, "~ω~ῠγκᾰτον", _;
    perfect_ind_act_3: _, _, "~ω~ῠγκᾱσῐ";
    perfect_ind_mid_1: "~ω~ῠμμαι", _, "~ω~ῠμμεθᾰ";
    perfect_ind_mid_2: "~ω~ῠψαι", "~ω~ῠφθον", _;
    perfect_ind_mid_3: "~ω~ῠπται", _, "~ω~ῠπᾰται";
    perfect_ind_pass_2: _, _, "~ω~ῠφθε";
    perfect_subj_act_1: "~ω~ῠγκω", _, _;
    perfect_subj_act_2: "~ω~ῠγκῃς", "~ω~ῠγκητον", "~ω~ῠγκητε";
    perfect_subj_act_3: "~ω~ῠγκῃ", _, "~ω~ῠγκωσῐ";
    perfect_opt_act_1: "~ω~ῠγκοιμῐ", _, "~ω~ῠγκοιμεν";
    perfect_opt_act_2: "~ω~ῠγκοις", _, _;
    perfect_opt_act_3: _, "~ω~ῠγκοιτην", _;
    perfect_imp_act_2: "~ω~ῠγκε", _, "~ω~ῠγκετε";
    perfect_imp_act_3: _, _, "~ω~ῠγκοντων";
    perfect_imp_mid_2: _, _, "~ω~ῠφθε";
    perfect_imp_mid_3: "~ω~ῠφθω", _, "~ω~ῠφθων";
    perfect_imp_pass_2: "~ω~ῠψο", _, "~ω~ῠφθε";
    perfect_imp_pass_3: _, "~ω~ῠφθων", "~ω~ῠφθων";
    infinitive_pres: "~ο~ῡνειν", _, "~ο~ῡνεσθαι";
    infinitive_fut: "~ο~ῠνεῑν", _, _;
    infinitive_aor: "~ο~ῡναι", _, _;
    infinitive_perf: _, "~ω~ῠφθαι", _;
    participle_pres: "~ο~ῡνων", _, "~ο~ῡνομενος";
    participle_aor: "~ο~ῡνᾱς", "~ο~ῡνᾰμενος", _;
    participle_perf: "~ω~ῠγκως", _, "~ω~ῠμμενος";
    ),
    // Ἐλᾰσσοω: 4 words (ἐρημοω, ἐλᾰσσοω, ἐλευθεροω, ἐπουλοω). Like Ἐλαυνω, except in 151 cells: ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἐλᾰσσοω", 4, "ἐ~οω", like Ἐλαυνω;
    stem future_pass: "~ωθησ- like ἐλπῐζω";
    stem aorist_ind: "~ωσ- like ἐλπῐζω";
    stem aorist_ind_pass: "~ωθην- like ἀγορευω";
    stem aorist_pass: "~ωθ- like ἐλπῐζω";
    ind_act_1: _, _, "ἐ~οῡμεν";
    ind_act_2: "ἐ~οῑς", _, _;
    ind_act_3: "ἐ~οῑ", "ἐ~οῡτον", "ἐ~οῡσῐ|ἐ~οῡσῐν";
    ind_mid_1: "ἐ~οῡμαι", _, "ἐ~ουμεθᾰ";
    ind_mid_2: "ἐ~οῑ", "ἐ~οῡσθον", _;
    ind_pass_2: _, _, "ἐ~οῡσθε";
    ind_pass_3: "ἐ~οῡται", _, "ἐ~οῡνται";
    subj_act_2: "ἐ~οῑς", _, _;
    subj_act_3: "ἐ~οῑ", "ἐ~ωτον", "ἐ~ωσῐ|ἐ~ωσῐν";
    subj_mid_2: _, "ἐ~ωσθον", _;
    subj_mid_3: "ἐ~ωται", _, _;
    subj_pass_2: _, _, "ἐ~ωσθε";
    opt_act_1: "ἐ~οιην|ἐ~οῑμῐ", _, "ἐ~οῑμεν|ἐ~οιημεν";
    opt_act_2: "ἐ~οιης|ἐ~οῑς", "ἐ~οῑτον|ἐ~οιητον", "ἐ~οῑτε|ἐ~οιητε";
    opt_act_3: "ἐ~οιη|ἐ~οῑ", "ἐ~οιτην|ἐ~οιητην", "ἐ~οῑεν|ἐ~οιησᾰν";
    opt_mid_2: "ἐ~οῑο", "ἐ~οῑσθον", _;
    opt_mid_3: "ἐ~οῑτο", _, "ἐ~οῑντο";
    opt_pass_2: _, _, "ἐ~οῑσθε";
    imp_act_2: "ἐ~ου", "ἐ~οῡτον", "ἐ~οῡτε";
    imp_mid_2: "ἐ~οῡ", _, "ἐ~οῡσθε";
    imp_mid_3: "ἐ~ουσθω", _, "ἐ~ουσθων";
    imp_pass_2: _, _, "ἐ~οῡσθε";
    imp_pass_3: _, "ἐ~ουσθων", "ἐ~ουσθων";
    imperfect_act_1: "ἠ~ουν", _, "ἠ~οῡμεν";
    imperfect_act_2: "ἠ~ους", "ἠ~οῡτον", "ἠ~οῡτε";
    imperfect_act_3: "ἠ~ου", "ἠ~ουτην", "ἠ~ουν";
    imperfect_mid_1: "ἠ~ουμην", _, "ἠ~ουμεθᾰ";
    imperfect_mid_2: "ἠ~οῡ", _, _;
    imperfect_mid_3: "ἠ~οῡτο", _, _;
    imperfect_pass_2: _, "ἠ~οῡσθον", "ἠ~οῡσθε";
    imperfect_pass_3: _, "ἠ~ουσθην", "ἠ~οῡντο";
    future_ind_act_1: "ἐ~ωσω", _, "ἐ~ωσομεν";
    future_ind_act_2: "ἐ~ωσεις", "ἐ~ωσετον", "ἐ~ωσετε";
    future_ind_act_3: _, _, "ἐ~ωσουσῐ|ἐ~ωσουσῐν";
    future_ind_mid_1: "ἐ~ωσομαι", _, "ἐ~ωσομεθᾰ";
    future_ind_mid_2: "ἐ~ωσῃ|ἐ~ωσει", "ἐ~ωσεσθον", _;
    future_ind_mid_3: "ἐ~ωσεται", _, _;
    future_opt_act_1: "ἐ~ωσοιμῐ", _, "ἐ~ωσοιμεν";
    future_opt_act_2: _, "ἐ~ωσοιτον", "ἐ~ωσοιτε";
    future_opt_act_3: "ἐ~ωσοι", "ἐ~ωσοιτην", "ἐ~ωσοιεν";
    future_opt_mid_1: "ἐ~ωσοιμην", _, "ἐ~ωσοιμεθᾰ";
    future_opt_mid_2: "ἐ~ωσοιο", _, _;
    future_opt_mid_3: _, "ἐ~ωσοισθην", _;
    aorist_subj_act_1: "ἐ~ωσω", _, _;
    aorist_subj_act_2: "ἐ~ωσῃς", "ἐ~ωσητον", _;
    aorist_subj_act_3: _, _, "ἐ~ωσωσῐ|ἐ~ωσωσῐν";
    aorist_subj_mid_1: "ἐ~ωσωμαι", _, _;
    aorist_subj_mid_2: "ἐ~ωσῃ", "ἐ~ωσησθον", _;
    aorist_opt_act_1: "ἐ~ωσαιμῐ", _, "ἐ~ωσαιμεν";
    aorist_opt_act_2: "ἐ~ωσειᾰς|ἐ~ωσαις", _, _;
    aorist_opt_act_3: "ἐ~ωσειε|ἐ~ωσειεν|ἐ~ωσαι", "ἐ~ωσαιτην", "ἐ~ωσειᾰν|ἐ~ωσαιεν";
    aorist_opt_mid_1: "ἐ~ωσαιμην", _, _;
    aorist_opt_mid_2: "ἐ~ωσαιο", _, _;
    aorist_imp_act_2: "ἐ~ωσον", _, "ἐ~ωσᾰτε";
    aorist_imp_act_3: _, _, "ἐ~ωσᾰντων";
    aorist_imp_mid_2: "ἐ~ωσαι", "ἐ~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἐ~ωσᾰσθων";
    perfect_ind_act_1: "ἠ~ωκᾰ", _, "ἠ~ωκᾰμεν";
    perfect_ind_act_2: _, "ἠ~ωκᾰτον", _;
    perfect_ind_act_3: _, _, "ἠ~ωκᾱσῐ";
    perfect_ind_mid_1: "ἠ~ωμαι", _, "ἠ~ωμεθᾰ";
    perfect_ind_mid_2: "ἠ~ωσαι", "ἠ~ωσθον", _;
    perfect_ind_mid_3: "ἠ~ωται", _, "ἠ~ωνται";
    perfect_ind_pass_2: _, _, "ἠ~ωσθε";
    perfect_subj_act_1: "ἠ~ωκω", _, _;
    perfect_subj_act_2: "ἠ~ωκῃς", "ἠ~ωκητον", "ἠ~ωκητε";
    perfect_subj_act_3: "ἠ~ωκῃ", _, "ἠ~ωκωσῐ";
    perfect_opt_act_1: "ἠ~ωκοιμῐ", _, "ἠ~ωκοιμεν";
    perfect_opt_act_2: "ἠ~ωκοις", _, _;
    perfect_opt_act_3: _, "ἠ~ωκοιτην", _;
    perfect_imp_act_2: "ἠ~ωκε", _, "ἠ~ωκετε";
    perfect_imp_act_3: _, _, "ἠ~ωκοντων";
    perfect_imp_mid_2: _, _, "ἠ~ωσθε";
    perfect_imp_mid_3: "ἠ~ωσθω", _, "ἠ~ωσθων";
    perfect_imp_pass_2: "ἠ~ωσο", _, "ἠ~ωσθε";
    perfect_imp_pass_3: _, "ἠ~ωσθων", "ἠ~ωσθων";
    pluperfect_act_1: "ἠ~ωκειν", _, _;
    pluperfect_mid_1: "ἠ~ωμην", _, _;
    pluperfect_mid_2: "ἠ~ωσο", "ἠ~ωσθον", _;
    pluperfect_mid_3: "ἠ~ωτο", _, "ἠ~ωντο";
    pluperfect_pass_1: _, _, "ἠ~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἠ~ωσθε";
    pluperfect_pass_3: _, "ἠ~ωσθην", _;
    infinitive_pres: "ἐ~οῡν", _, "ἐ~οῡσθαι";
    infinitive_fut: "ἐ~ωσειν", _, _;
    infinitive_aor: "ἐ~ωσαι", _, _;
    infinitive_perf: _, "ἠ~ωσθαι", _;
    participle_pres: _, _, "ἐ~ουμενος";
    participle_aor: "ἐ~ωσᾱς", "ἐ~ωσᾰμενος", _;
    participle_perf: "ἠ~ωκως", _, "ἠ~ωμενος";
    ),
    // Ἐρωτᾰω: 4 words (ἐρωτᾰω, ἐρᾰω, ἐρευνᾰω, ἐμπολᾰω). Like Ἐλαυνω, except in 155 cells: ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἐρωτᾰω", 4, "ἐ~ᾰω", like Ἐλαυνω;
    stem aorist_ind: "~ησ- like ἀγορευω";
    stem aorist_ind_pass: "~ηθην- like ἀγορευω";
    stem aorist_pass: "~ηθ- like ἐρωτᾰω";
    ind_act_1: _, _, "ἐ~ωμεν";
    ind_act_2: "ἐ~ᾳς", _, _;
    ind_act_3: "ἐ~ᾳ", "ἐ~ᾱτον", "ἐ~ωσῐ|ἐ~ωσῐν";
    ind_mid_1: "ἐ~ωμαι", _, "ἐ~ωμεθᾰ";
    ind_mid_2: "ἐ~ᾳ", "ἐ~ᾱσθον", _;
    ind_pass_2: _, _, "ἐ~ᾱσθε";
    ind_pass_3: "ἐ~ᾱται", _, "ἐ~ωνται";
    subj_act_2: "ἐ~ᾳς", _, _;
    subj_act_3: "ἐ~ᾳ", "ἐ~ᾱτον", "ἐ~ωσῐ|ἐ~ωσῐν";
    subj_mid_2: _, "ἐ~ᾱσθον", _;
    subj_mid_3: "ἐ~ᾱται", _, _;
    subj_pass_2: _, _, "ἐ~ᾱσθε";
    opt_act_1: "ἐ~ῳην|ἐ~ῳμῐ", _, "ἐ~ῳμεν|ἐ~ῳημεν";
    opt_act_2: "ἐ~ῳης|ἐ~ῳς", "ἐ~ῳτον|ἐ~ῳητον", "ἐ~ῳτε|ἐ~ῳητε";
    opt_act_3: "ἐ~ῳη|ἐ~ῳ", "ἐ~ῳτην|ἐ~ῳητην", "ἐ~ῳεν|ἐ~ῳησᾰν";
    opt_mid_1: "ἐ~ῳμην", _, "ἐ~ῳμεθᾰ";
    opt_mid_2: "ἐ~ῳο", "ἐ~ῳσθον", _;
    opt_mid_3: "ἐ~ῳτο", "ἐ~ῳσθην", "ἐ~ῳντο";
    opt_pass_2: _, _, "ἐ~ῳσθε";
    imp_act_2: "ἐ~ᾱ", "ἐ~ᾱτον", "ἐ~ᾱτε";
    imp_mid_2: "ἐ~ω", _, "ἐ~ᾱσθε";
    imp_mid_3: "ἐ~ᾱσθω", _, "ἐ~ᾱσθων";
    imp_pass_2: _, _, "ἐ~ᾱσθε";
    imp_pass_3: _, "ἐ~ᾱσθων", "ἐ~ᾱσθων";
    imperfect_act_1: "ἠ~ων", _, "ἠ~ωμεν";
    imperfect_act_2: "ἠ~ᾱς", "ἠ~ᾱτον", "ἠ~ᾱτε";
    imperfect_act_3: "ἠ~ᾱ", "ἠ~ᾱτην", "ἠ~ων";
    imperfect_mid_1: "ἠ~ωμην", _, "ἠ~ωμεθᾰ";
    imperfect_mid_2: "ἠ~ω", _, _;
    imperfect_mid_3: "ἠ~ᾱτο", _, _;
    imperfect_pass_2: _, "ἠ~ᾱσθον", "ἠ~ᾱσθε";
    imperfect_pass_3: _, "ἠ~ᾱσθην", "ἠ~ωντο";
    future_ind_act_1: "ἐ~ησω", _, "ἐ~ησομεν";
    future_ind_act_2: "ἐ~ησεις", "ἐ~ησετον", "ἐ~ησετε";
    future_ind_act_3: _, _, "ἐ~ησουσῐ";
    future_ind_mid_1: "ἐ~ησομαι", _, "ἐ~ησομεθᾰ";
    future_ind_mid_2: "ἐ~ησῃ", "ἐ~ησεσθον", _;
    future_ind_mid_3: "ἐ~ησεται", _, _;
    future_ind_pass_1: "ἐ~ηθησομαι", _, _;
    future_ind_pass_3: _, "ἐ~ηθησεσθον", _;
    future_opt_act_1: "ἐ~ησοιμῐ", _, "ἐ~ησοιμεν";
    future_opt_act_2: _, "ἐ~ησοιτον", "ἐ~ησοιτε";
    future_opt_act_3: "ἐ~ησοι", "ἐ~ησοιτην", "ἐ~ησοιεν";
    future_opt_mid_1: "ἐ~ησοιμην", _, "ἐ~ησοιμεθᾰ";
    future_opt_mid_2: "ἐ~ησοιο", _, _;
    future_opt_mid_3: _, "ἐ~ησοισθην", _;
    aorist_subj_act_1: "ἐ~ησω", _, _;
    aorist_subj_act_2: "ἐ~ησῃς", "ἐ~ησητον", _;
    aorist_subj_mid_1: "ἐ~ησωμαι", _, _;
    aorist_subj_mid_2: "ἐ~ησῃ", "ἐ~ησησθον", _;
    aorist_opt_act_1: "ἐ~ησαιμῐ", _, "ἐ~ησαιμεν";
    aorist_opt_act_2: "ἐ~ησειᾰς", _, _;
    aorist_opt_act_3: "ἐ~ησειε", "ἐ~ησαιτην", "ἐ~ησειᾰν";
    aorist_opt_mid_1: "ἐ~ησαιμην", _, _;
    aorist_opt_mid_2: "ἐ~ησαιο", _, _;
    aorist_imp_act_2: "ἐ~ησον", _, "ἐ~ησᾰτε";
    aorist_imp_act_3: _, _, "ἐ~ησᾰντων";
    aorist_imp_mid_2: "ἐ~ησαι", "ἐ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἐ~ησᾰσθων";
    perfect_ind_act_1: "ἠ~ηκᾰ", _, "ἠ~ηκᾰμεν";
    perfect_ind_act_2: _, "ἠ~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "ἠ~ηκᾱσῐ";
    perfect_ind_mid_1: "ἠ~ημαι", _, "ἠ~ημεθᾰ";
    perfect_ind_mid_2: "ἠ~ησαι", "ἠ~ησθον", _;
    perfect_ind_mid_3: "ἠ~ηται", _, "ἠ~ηνται";
    perfect_ind_pass_2: _, _, "ἠ~ησθε";
    perfect_subj_act_1: "ἠ~ηκω", _, _;
    perfect_subj_act_2: "ἠ~ηκῃς", "ἠ~ηκητον", "ἠ~ηκητε";
    perfect_subj_act_3: "ἠ~ηκῃ", _, "ἠ~ηκωσῐ";
    perfect_opt_act_1: "ἠ~ηκοιμῐ", _, "ἠ~ηκοιμεν";
    perfect_opt_act_2: "ἠ~ηκοις", _, _;
    perfect_opt_act_3: _, "ἠ~ηκοιτην", _;
    perfect_imp_act_2: "ἠ~ηκε", _, "ἠ~ηκετε";
    perfect_imp_act_3: _, _, "ἠ~ηκοντων";
    perfect_imp_mid_2: _, _, "ἠ~ησθε";
    perfect_imp_mid_3: "ἠ~ησθω", _, "ἠ~ησθων";
    perfect_imp_pass_2: "ἠ~ησο", _, "ἠ~ησθε";
    perfect_imp_pass_3: _, "ἠ~ησθων", "ἠ~ησθων";
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
    participle_pres: _, _, "ἐ~ωμενος";
    participle_aor: "ἐ~ησᾱς", "ἐ~ησᾰμενος", _;
    participle_perf: "ἠ~ηκως", _, "ἠ~ημενος";
    ),
    // Πᾰρᾰγῐγνομαι: 4 words (ὑπογῐγνομαι, πᾰρᾰγῐγνομαι, περῐγῐγνομαι, ῠγγῐγνομαι). Like Ζευγνῡμῐ, except in 171 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("πᾰρᾰγῐγνομαι", 4, "~~γῐγ~ομαι", like Ζευγνῡμῐ;
    stem future_pass: "~~γε~ηθησ- like ἐχω";
    stem aorist_ind: "~νε~ε~- like ἐρχομαι";
    stem aorist_ind_pass: "~νε~ε~ηθην- like ἐλπω";
    stem aorist_pass: "~~γε~ηθ- like ἐχω";
    ind_act_1: "~~γῐγ~ομαι", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, derived, derived;
    ind_mid_1: "~~γῐγ~ομαι", _, "~~γῐγ~ομεθᾰ";
    ind_mid_2: "~~γῐγ~ῃ", "~~γῐγ~εσθον", _;
    ind_pass_2: _, _, "~~γῐγ~εσθε";
    ind_pass_3: "~~γῐγ~εται", _, "~~γῐγ~ονται";
    subj_act_1: derived, _, derived;
    subj_act_2: derived, _, _;
    subj_act_3: derived, derived, derived;
    subj_mid_1: "~~γῐγ~ωμαι", _, "~~γῐγ~ωμεθᾰ";
    subj_mid_2: "~~γῐγ~ῃ", "~~γῐγ~ησθον", _;
    subj_mid_3: "~~γῐγ~ηται", _, "~~γῐγ~ωνται";
    subj_pass_1: derived, _, _;
    subj_pass_2: "~~γῐγ~ῃ", _, "~~γῐγ~ησθε";
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, _;
    opt_act_3: derived, derived, derived;
    opt_mid_1: "~~γῐγ~οιμην", _, "~~γῐγ~οιμεθᾰ";
    opt_mid_2: "~~γῐγ~οιο", "~~γῐγ~οισθον", _;
    opt_mid_3: "~~γῐγ~οιτο", "~~γῐγ~οισθην", "~~γῐγ~οιντο";
    opt_pass_2: _, _, "~~γῐγ~οισθε";
    imp_act_2: derived, derived, derived;
    imp_mid_2: "~~γῐγ~ου", _, "~~γῐγ~εσθε";
    imp_mid_3: "~~γῐγ~εσθω", _, "~~γῐγ~εσθων";
    imp_pass_2: _, _, "~~γῐγ~εσθε";
    imp_pass_3: _, "~~γῐγ~εσθων", "~~γῐγ~εσθων";
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    imperfect_mid_1: "~νε%ῐ~~ομην", _, "~νε%ῐ~~ομεθᾰ";
    imperfect_mid_2: "~νε%ῐ~~ου", "~νε%ῐ~~εσθον", _;
    imperfect_mid_3: "~νε%ῐ~~ετο", "~νε%ῐ~~εσθην", "~νε%ῐ~~οντο";
    imperfect_pass_2: _, derived, "~νε%ῐ~~εσθε";
    imperfect_pass_3: _, derived, derived;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: _, _, derived;
    future_ind_mid_1: "~~γε~ησομαι", _, "~~γε~ησομεθᾰ";
    future_ind_mid_2: "~~γε~ησῃ", "~~γε~ησεσθον", _;
    future_ind_mid_3: "~~γε~ησεται", _, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: _, derived, derived;
    future_opt_act_3: derived, derived, derived;
    future_opt_mid_1: "~~γε~ησοιμην", _, "~~γε~ησοιμεθᾰ";
    future_opt_mid_2: "~~γε~ησοιο", _, _;
    future_opt_mid_3: _, "~~γε~ησοισθην", _;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_mid_1: "~~γε~ωμαι", _, _;
    aorist_subj_mid_2: "~~γε~ῃ", "~~γε~ησθον", _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_mid_1: "~~γε~οιμην", _, _;
    aorist_opt_mid_2: "~~γε~οιο", _, _;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_mid_2: "~~γε~οῡ", "~~γε~εσθον", _;
    aorist_imp_mid_3: _, _, "~~γε~εσθων";
    perfect_ind_act_1: "~~γεγο~ᾰ", _, "~~γεγο~ᾰμεν";
    perfect_ind_act_2: _, "~~γεγο~ᾰτον", _;
    perfect_ind_act_3: _, _, "~~γεγο~ᾱσῐ";
    perfect_ind_mid_1: "~~γεγε~ημαι", _, "~~γεγε~ημεθᾰ";
    perfect_ind_mid_2: "~~γεγε~ησαι", "~~γεγε~ησθον", _;
    perfect_ind_mid_3: "~~γεγε~ηται", _, "~~γεγε~ηνται";
    perfect_ind_pass_2: _, _, "~~γεγε~ησθε";
    perfect_subj_act_1: "~~γεγο~ω", _, _;
    perfect_subj_act_2: "~~γεγο~ῃς", "~~γεγο~ητον", "~~γεγο~ητε";
    perfect_subj_act_3: "~~γεγο~ῃ", _, "~~γεγο~ωσῐ";
    perfect_opt_act_1: "~~γεγο~οιμῐ", _, "~~γεγο~οιμεν";
    perfect_opt_act_2: "~~γεγο~οις", _, _;
    perfect_opt_act_3: _, "~~γεγο~οιτην", _;
    perfect_imp_act_2: "~~γεγο~ε", _, "~~γεγο~ετε";
    perfect_imp_act_3: _, _, "~~γεγο~οντων";
    perfect_imp_mid_2: _, _, "~~γεγε~ησθε";
    perfect_imp_mid_3: "~~γεγε~ησθω", _, "~~γεγε~ησθων";
    perfect_imp_pass_2: "~~γεγε~ησο", _, "~~γεγε~ησθε";
    perfect_imp_pass_3: _, "~~γεγε~ησθων", "~~γεγε~ησθων";
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
    // Δημοκρᾰτεομαι: 4 words (πλουτοκρᾰτεομαι, δημοκρᾰτεομαι, εὐλαβεομαι, ἀμηχᾰνοποιεομαι). Like Λᾰγοθηρεω, except in 36 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("δημοκρᾰτεομαι", 4, "~ᾰ~εομαι", like Λᾰγοθηρεω;
    stem aorist_ind: "~ᾰ~ησ- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~ηθην- like ἐχω";
    stem pluperfect_act: "~ᾰ~ηκ- like χρημᾰτῐζω";
    imperfect_act_1: "ἐ~ᾰ~ουν", _, "ἐ~ᾰ~οῡμεν";
    imperfect_act_2: "ἐ~ᾰ~εις", "ἐ~ᾰ~εῑτον", "ἐ~ᾰ~εῑτε";
    imperfect_act_3: "ἐ~ᾰ~ει", "ἐ~ᾰ~ειτην", "ἐ~ᾰ~ουν";
    imperfect_mid_1: "ἐ~ᾰ~ουμην", _, "ἐ~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~οῡ", _, _;
    imperfect_mid_3: "ἐ~ᾰ~εῑτο", _, _;
    imperfect_pass_2: _, "ἐ~ᾰ~εῑσθον", "ἐ~ᾰ~εῑσθε";
    imperfect_pass_3: _, "ἐ~ᾰ~εισθην", "ἐ~ᾰ~οῡντο";
    perfect_ind_act_1: "%ε~ᾰ~ηκᾰ", _, "%ε~ᾰ~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~ᾰ~ηκᾰτον", _;
    perfect_ind_act_3: "%ε~ᾰ~ηκε|%ε~ᾰ~ηκεν", _, "%ε~ᾰ~ηκᾱσῐ|%ε~ᾰ~ηκᾱσῐν";
    perfect_subj_act_1: "%ε~ᾰ~ηκω", _, _;
    perfect_subj_act_2: "%ε~ᾰ~ηκῃς", "%ε~ᾰ~ηκητον", "%ε~ᾰ~ηκητε";
    perfect_subj_act_3: "%ε~ᾰ~ηκῃ", _, "%ε~ᾰ~ηκωσῐ|%ε~ᾰ~ηκωσῐν";
    perfect_opt_act_1: "%ε~ᾰ~ηκοιμῐ|%ε~ᾰ~ηκοιην", _, "%ε~ᾰ~ηκοιμεν";
    perfect_opt_act_2: "%ε~ᾰ~ηκοις|%ε~ᾰ~ηκοιης", _, _;
    perfect_opt_act_3: "%ε~ᾰ~ηκοι|%ε~ᾰ~ηκοιη", "%ε~ᾰ~ηκοιτην", _;
    perfect_imp_act_2: "%ε~ᾰ~ηκε", _, "%ε~ᾰ~ηκετε";
    perfect_imp_act_3: _, _, "%ε~ᾰ~ηκοντων";
    participle_perf: "%ε~ᾰ~ηκως", _, _;
    ),
    // Ἀνᾰλᾰμβᾰνω: 4 words (κᾰτᾰλᾰμβᾰνω, ἀνᾰλᾰμβᾰνω, μετᾰλαμβᾰνω, ἐπᾰνᾰλᾰμβᾰνω). Like Μῑγνῡμῐ, except in 147 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀνᾰλᾰμβᾰνω", 4, "~ᾰλᾰμβᾰνω", like Μῑγνῡμῐ;
    stem future_pass: "~ᾰληφθησ- like ποιεω";
    stem aorist_ind: "~ελᾰβ- like πᾰρεχω";
    stem aorist_ind_pass: "~εληφθην- like ἐλπω";
    stem aorist_pass: "~ᾰληφθ- like ποιεω";
    stem pluperfect_act: "~ειληφ- like εὐλογεω";
    ind_act_1: "~ᾰλᾰμβᾰνω", _, "~ᾰλᾰμβᾰνομεν";
    ind_act_2: "~ᾰλᾰμβᾰνεις", _, _;
    ind_act_3: "~ᾰλᾰμβᾰνει", "~ᾰλᾰμβᾰνετον", "~ᾰλᾰμβᾰνουσῐ|~ᾰλᾰμβᾰνουσῐν";
    ind_mid_1: "~ᾰλᾰμβᾰνομαι", _, "~ᾰλᾰμβᾰνομεθᾰ";
    ind_mid_2: "~ᾰλᾰμβᾰνῃ|~ᾰλᾰμβᾰνει", "~ᾰλᾰμβᾰνεσθον", _;
    ind_pass_2: _, _, "~ᾰλᾰμβᾰνεσθε";
    ind_pass_3: "~ᾰλᾰμβᾰνεται", _, "~ᾰλᾰμβᾰνονται";
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
    opt_mid_2: "~ᾰλᾰμβᾰνοιο", "~ᾰλᾰμβᾰνοισθον", _;
    opt_mid_3: "~ᾰλᾰμβᾰνοιτο", "~ᾰλᾰμβᾰνοισθην", "~ᾰλᾰμβᾰνοιντο";
    opt_pass_2: _, _, "~ᾰλᾰμβᾰνοισθε";
    imp_act_2: "~ᾰλᾰμβᾰνε", "~ᾰλᾰμβᾰνετον", "~ᾰλᾰμβᾰνετε";
    imp_mid_2: "~ᾰλᾰμβᾰνου", _, "~ᾰλᾰμβᾰνεσθε";
    imp_mid_3: "~ᾰλᾰμβᾰνεσθω", _, "~ᾰλᾰμβᾰνεσθων";
    imp_pass_2: _, _, "~ᾰλᾰμβᾰνεσθε";
    imp_pass_3: _, "~ᾰλᾰμβᾰνεσθων", "~ᾰλᾰμβᾰνεσθων";
    imperfect_act_1: "~ελᾰμβᾰνον", _, "~ελᾰμβᾰνομεν";
    imperfect_act_2: "~ελᾰμβᾰνες", "~ελᾰμβᾰνετον", "~ελᾰμβᾰνετε";
    imperfect_act_3: "~ελᾰμβᾰνε|~ελᾰμβᾰνεν", "~ελᾰμβᾰνετην", "~ελᾰμβᾰνον";
    imperfect_mid_1: "~ελᾰμβᾰνομην", _, "~ελᾰμβᾰνομεθᾰ";
    imperfect_mid_2: "~ελᾰμβᾰνου", _, _;
    imperfect_mid_3: "~ελᾰμβᾰνετο", _, _;
    imperfect_pass_2: _, "~ελᾰμβᾰνεσθον", "~ελᾰμβᾰνεσθε";
    imperfect_pass_3: _, "~ελᾰμβᾰνεσθην", "~ελᾰμβᾰνοντο";
    future_ind_mid_1: "~ᾰληψομαι", _, "~ᾰληψομεθᾰ";
    future_ind_mid_2: "~ᾰληψῃ|~ᾰληψει", "~ᾰληψεσθον", _;
    future_ind_mid_3: "~ᾰληψεται", _, _;
    future_opt_mid_1: "~ᾰληψοιμην", _, "~ᾰληψοιμεθᾰ";
    future_opt_mid_2: "~ᾰληψοιο", _, _;
    future_opt_mid_3: _, "~ᾰληψοισθην", _;
    aorist_subj_act_1: "~ᾰλᾰβω", _, _;
    aorist_subj_act_2: "~ᾰλᾰβῃς", "~ᾰλᾰβητον", _;
    aorist_subj_act_3: _, _, "~ᾰλᾰβωσῐ|~ᾰλᾰβωσῐν";
    aorist_subj_mid_1: "~ᾰλᾰβωμαι", _, _;
    aorist_subj_mid_2: "~ᾰλᾰβῃ", "~ᾰλᾰβησθον", _;
    aorist_opt_act_1: "~ᾰλᾰβοιμῐ", _, "~ᾰλᾰβοιμεν";
    aorist_opt_act_2: "~ᾰλᾰβοις", _, _;
    aorist_opt_act_3: "~ᾰλᾰβοι", "~ᾰλᾰβοιτην", "~ᾰλᾰβοιεν";
    aorist_opt_mid_1: "~ᾰλᾰβοιμην", _, _;
    aorist_opt_mid_2: "~ᾰλᾰβοιο", _, _;
    aorist_imp_act_2: "~ᾰλᾰβε", _, "~ᾰλᾰβετε";
    aorist_imp_act_3: _, _, "~ᾰλᾰβοντων";
    aorist_imp_mid_2: "~ᾰλᾰβοῡ", "~ᾰλᾰβεσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰλᾰβεσθων";
    perfect_ind_act_1: "~ειληφᾰ", _, "~ειληφᾰμεν";
    perfect_ind_act_2: _, "~ειληφᾰτον", _;
    perfect_ind_act_3: "~ειληφε|~ειληφεν", _, "~ειληφᾱσῐ|~ειληφᾱσῐν";
    perfect_ind_mid_1: "~ειλημμαι", _, "~ειλημμεθᾰ";
    perfect_ind_mid_2: "~ειληψαι", "~ειληφθον", _;
    perfect_ind_mid_3: "~ειληπται", _, "~ειληφᾰται";
    perfect_ind_pass_2: _, _, "~ειληφθε";
    perfect_subj_act_1: "~ειληφω", _, _;
    perfect_subj_act_2: "~ειληφῃς", "~ειληφητον", "~ειληφητε";
    perfect_subj_act_3: "~ειληφῃ", _, "~ειληφωσῐ|~ειληφωσῐν";
    perfect_opt_act_1: "~ειληφοιμῐ|~ειληφοιην", _, "~ειληφοιμεν";
    perfect_opt_act_2: "~ειληφοις|~ειληφοιης", _, _;
    perfect_opt_act_3: "~ειληφοι|~ειληφοιη", "~ειληφοιτην", _;
    perfect_imp_act_2: "~ειληφε", _, "~ειληφετε";
    perfect_imp_act_3: _, _, "~ειληφοντων";
    perfect_imp_mid_2: _, _, "~ειληφθε";
    perfect_imp_mid_3: "~ειληφθω", _, "~ειληφθων";
    perfect_imp_pass_2: "~ειληψο", _, "~ειληφθε";
    perfect_imp_pass_3: _, "~ειληφθων", "~ειληφθων";
    pluperfect_mid_1: "~ειλημμην", _, _;
    pluperfect_mid_2: "~ειληψο", "~ειληφθον", _;
    pluperfect_mid_3: "~ειληπτο", _, "~ειληφᾰτο";
    pluperfect_pass_1: _, _, "~ειλημμεθᾰ";
    pluperfect_pass_2: _, _, "~ειληφθε";
    pluperfect_pass_3: _, "~ειληφθην", _;
    infinitive_pres: "~ᾰλᾰμβᾰνειν", _, "~ᾰλᾰμβᾰνεσθαι";
    infinitive_aor: "~ᾰλᾰβεῑν", _, _;
    infinitive_perf: _, "~ειληφθαι", _;
    participle_pres: "~ᾰλᾰμβᾰνων", _, "~ᾰλᾰμβᾰνομενος";
    participle_aor: "~ᾰλᾰβων", "~ᾰλᾰβομενος", _;
    participle_perf: "~ειληφως", _, "~ειλημμενος";
    ),
    // Γυμνοω: 4 words (κυκλοω, γυμνοω, ἀπολυτροω, ἐκλυτροω). Like Ξαινω, except in 151 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("γυμνοω", 4, "~υ~οω", like Ξαινω;
    stem future_pass: "~ῠ~ωθησ- like ἐχω";
    stem aorist_ind: "~ῠ~ωσ- like ἐχω";
    stem aorist_ind_pass: "~ῠ~ωθην- like ἐχω";
    stem aorist_pass: "~ῠ~ηθ- like ἐχω";
    ind_act_1: "~ῠ~ω", _, "~ῠ~οῡμεν";
    ind_act_2: "~ῠ~οῑς", _, _;
    ind_act_3: "~ῠ~οῑ", "~ῠ~οῡτον", "~ῠ~οῡσῐ|~ῠ~οῡσῐν";
    ind_mid_1: "~ῠ~οῡμαι", _, "~ῠ~ουμεθᾰ";
    ind_mid_2: "~ῠ~οῑ", "~ῠ~οῡσθον", _;
    ind_pass_2: _, _, "~ῠ~οῡσθε";
    ind_pass_3: "~ῠ~οῡται", _, "~ῠ~οῡνται";
    subj_act_1: "~ῠ~ω", _, "~ῠ~ωμεν";
    subj_act_2: "~ῠ~οῑς", _, _;
    subj_act_3: "~ῠ~οῑ", "~ῠ~ωτον", "~ῠ~ωσῐ|~ῠ~ωσῐν";
    subj_mid_1: _, _, "~ῠ~ωμεθᾰ";
    subj_mid_2: _, "~ῠ~ωσθον", _;
    subj_mid_3: "~ῠ~ωται", _, "~ῠ~ωνται";
    subj_pass_1: "~ῠ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῠ~ωσθε";
    opt_act_1: "~ῠ~οιην|~ῠ~οῑμῐ", _, "~ῠ~οῑμεν|~ῠ~οιημεν";
    opt_act_2: "~ῠ~οιης|~ῠ~οῑς", "~ῠ~οῑτον|~ῠ~οιητον", "~ῠ~οῑτε|~ῠ~οιητε";
    opt_act_3: "~ῠ~οιη|~ῠ~οῑ", "~ῠ~οιτην|~ῠ~οιητην", "~ῠ~οῑεν|~ῠ~οιησᾰν";
    opt_mid_1: "~ῠ~οιμην", _, "~ῠ~οιμεθᾰ";
    opt_mid_2: "~ῠ~οῑο", "~ῠ~οῑσθον", _;
    opt_mid_3: "~ῠ~οῑτο", "~ῠ~οισθην", "~ῠ~οῑντο";
    opt_pass_2: _, _, "~ῠ~οῑσθε";
    imp_act_2: "~ῠ~ου", "~ῠ~οῡτον", "~ῠ~οῡτε";
    imp_mid_2: "~ῠ~οῡ", _, "~ῠ~οῡσθε";
    imp_mid_3: "~ῠ~ουσθω", _, "~ῠ~ουσθων";
    imp_pass_2: _, _, "~ῠ~οῡσθε";
    imp_pass_3: _, "~ῠ~ουσθων", "~ῠ~ουσθων";
    imperfect_act_1: "ἐ~ῠ~ουν", _, "ἐ~ῠ~οῡμεν";
    imperfect_act_2: "ἐ~ῠ~ους", "ἐ~ῠ~οῡτον", "ἐ~ῠ~οῡτε";
    imperfect_act_3: "ἐ~ῠ~ου", "ἐ~ῠ~ουτην", "ἐ~ῠ~ουν";
    imperfect_mid_1: "ἐ~ῠ~ουμην", _, "ἐ~ῠ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ῠ~οῡ", _, _;
    imperfect_mid_3: "ἐ~ῠ~οῡτο", _, _;
    imperfect_pass_2: _, "ἐ~ῠ~οῡσθον", "ἐ~ῠ~οῡσθε";
    imperfect_pass_3: _, "ἐ~ῠ~ουσθην", "ἐ~ῠ~οῡντο";
    future_ind_act_1: "~ῠ~ωσω", _, "~ῠ~ωσομεν";
    future_ind_act_2: "~ῠ~ωσεις", "~ῠ~ωσετον", "~ῠ~ωσετε";
    future_ind_act_3: _, _, "~ῠ~ωσουσῐ";
    future_ind_mid_1: "~ῠ~ωσομαι", _, "~ῠ~ωσομεθᾰ";
    future_ind_mid_2: "~ῠ~ωσῃ", "~ῠ~ωσεσθον", _;
    future_ind_mid_3: "~ῠ~ωσεται", _, _;
    future_opt_act_1: "~ῠ~ωσοιμῐ", _, "~ῠ~ωσοιμεν";
    future_opt_act_2: _, "~ῠ~ωσοιτον", "~ῠ~ωσοιτε";
    future_opt_act_3: "~ῠ~ωσοι", "~ῠ~ωσοιτην", "~ῠ~ωσοιεν";
    future_opt_mid_1: "~ῠ~ωσοιμην", _, "~ῠ~ωσοιμεθᾰ";
    future_opt_mid_2: "~ῠ~ωσοιο", _, _;
    future_opt_mid_3: _, "~ῠ~ωσοισθην", _;
    aorist_subj_act_1: "~ῠ~ωσω", _, _;
    aorist_subj_act_2: "~ῠ~ωσῃς", "~ῠ~ωσητον", _;
    aorist_subj_mid_1: "~ῠ~ωσωμαι", _, _;
    aorist_subj_mid_2: "~ῠ~ωσῃ", "~ῠ~ωσησθον", _;
    aorist_opt_act_1: "~ῠ~ωσαιμῐ", _, "~ῠ~ωσαιμεν";
    aorist_opt_act_2: "~ῠ~ωσειᾰς", _, _;
    aorist_opt_act_3: "~ῠ~ωσειε", "~ῠ~ωσαιτην", "~ῠ~ωσειᾰν";
    aorist_opt_mid_1: "~ῠ~ωσαιμην", _, _;
    aorist_opt_mid_2: "~ῠ~ωσαιο", _, _;
    aorist_imp_act_2: "~ῠ~ωσον", _, "~ῠ~ωσᾰτε";
    aorist_imp_act_3: _, _, "~ῠ~ωσᾰντων";
    aorist_imp_mid_2: "~ῠ~ωσαι", "~ῠ~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῠ~ωσᾰσθων";
    perfect_ind_act_1: "~ε%ῠ~ωκᾰ", _, "~ε%ῠ~ωκᾰμεν";
    perfect_ind_act_2: _, "~ε%ῠ~ωκᾰτον", _;
    perfect_ind_act_3: _, _, "~ε%ῠ~ωκᾱσῐ";
    perfect_ind_mid_1: "~ε%ῠ~ωμαι", _, "~ε%ῠ~ωμεθᾰ";
    perfect_ind_mid_2: "~ε%ῠ~ωσαι", "~ε%ῠ~ωσθον", _;
    perfect_ind_mid_3: "~ε%ῠ~ωται", _, "~ε%ῠ~ωνται";
    perfect_ind_pass_2: _, _, "~ε%ῠ~ωσθε";
    perfect_subj_act_1: "~ε%ῠ~ωκω", _, _;
    perfect_subj_act_2: "~ε%ῠ~ωκῃς", "~ε%ῠ~ωκητον", "~ε%ῠ~ωκητε";
    perfect_subj_act_3: "~ε%ῠ~ωκῃ", _, "~ε%ῠ~ωκωσῐ";
    perfect_opt_act_1: "~ε%ῠ~ωκοιμῐ", _, "~ε%ῠ~ωκοιμεν";
    perfect_opt_act_2: "~ε%ῠ~ωκοις", _, _;
    perfect_opt_act_3: _, "~ε%ῠ~ωκοιτην", _;
    perfect_imp_act_2: "~ε%ῠ~ωκε", _, "~ε%ῠ~ωκετε";
    perfect_imp_act_3: _, _, "~ε%ῠ~ωκοντων";
    perfect_imp_mid_2: _, _, "~ε%ῠ~ωσθε";
    perfect_imp_mid_3: "~ε%ῠ~ωσθω", _, "~ε%ῠ~ωσθων";
    perfect_imp_pass_2: "~ε%ῠ~ωσο", _, "~ε%ῠ~ωσθε";
    perfect_imp_pass_3: _, "~ε%ῠ~ωσθων", "~ε%ῠ~ωσθων";
    infinitive_pres: "~ῠ~οῡν", _, "~ῠ~οῡσθαι";
    infinitive_fut: "~ῠ~ωσειν", _, _;
    infinitive_aor: "~ῠ~ωσαι", _, _;
    infinitive_perf: _, "~ε%ῠ~ωσθαι", _;
    participle_pres: "~ῠ~ων", _, "~ῠ~ουμενος";
    participle_aor: "~ῠ~ωσᾱς", "~ῠ~ωσᾰμενος", _;
    participle_perf: "~ε%ῠ~ωκως", _, "~ε%ῠ~ωμενος";
    ),
    // Ἀφορῐζω: 4 words (ἀφορῐζω, προορῐζω, ἐξορκῐζω, ἑορτᾰζω). Like Ἀποκωλῡω, except in 142 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀφορῐζω", 4, "~ο~ζω", like Ἀποκωλῡω;
    stem future_pass: "~ο~σθησ- like ποιεω";
    stem aorist_ind: "~ω~σ- like εὐλογεω";
    stem aorist_ind_pass: "~ω~σθην- like ἐλπω";
    stem aorist_pass: "~ο~σθ- like ποιεω";
    stem pluperfect_act: "~ω~κ- like εὐλογεω";
    ind_act_1: "~ο~ζω", _, "~ο~ζομεν";
    ind_act_2: "~ο~ζεις", _, _;
    ind_act_3: "~ο~ζει", "~ο~ζετον", "~ο~ζουσῐ|~ο~ζουσῐν";
    ind_mid_1: "~ο~ζομαι", _, "~ο~ζομεθᾰ";
    ind_mid_2: "~ο~ζῃ|~ο~ζει", "~ο~ζεσθον", _;
    ind_pass_2: _, _, "~ο~ζεσθε";
    ind_pass_3: "~ο~ζεται", _, "~ο~ζονται";
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
    opt_mid_2: "~ο~ζοιο", "~ο~ζοισθον", _;
    opt_mid_3: "~ο~ζοιτο", "~ο~ζοισθην", "~ο~ζοιντο";
    opt_pass_2: _, _, "~ο~ζοισθε";
    imp_act_2: "~ο~ζε", "~ο~ζετον", "~ο~ζετε";
    imp_mid_2: "~ο~ζου", _, "~ο~ζεσθε";
    imp_mid_3: "~ο~ζεσθω", _, "~ο~ζεσθων";
    imp_pass_2: _, _, "~ο~ζεσθε";
    imp_pass_3: _, "~ο~ζεσθων", "~ο~ζεσθων";
    imperfect_act_1: "~ω~ζον", _, "~ω~ζομεν";
    imperfect_act_2: "~ω~ζες", "~ω~ζετον", "~ω~ζετε";
    imperfect_act_3: "~ω~ζε|~ω~ζεν", "~ω~ζετην", "~ω~ζον";
    imperfect_mid_1: "~ω~ζομην", _, "~ω~ζομεθᾰ";
    imperfect_mid_2: "~ω~ζου", _, _;
    imperfect_mid_3: "~ω~ζετο", _, _;
    imperfect_pass_2: _, "~ω~ζεσθον", "~ω~ζεσθε";
    imperfect_pass_3: _, "~ω~ζεσθην", "~ω~ζοντο";
    future_ind_act_1: "~ο~ω", _, "~ο~οῡμεν";
    future_ind_act_2: "~ο~εῑς", "~ο~εῑτον", "~ο~εῑτε";
    future_ind_act_3: _, _, "~ο~οῡσῐ|~ο~οῡσῐν";
    future_ind_mid_1: "~ο~οῡμαι", _, "~ο~ουμεθᾰ";
    future_ind_mid_2: "~ο~εῑ|~ο~ῃ", "~ο~εῑσθον", _;
    future_ind_mid_3: "~ο~εῑται", _, _;
    future_opt_act_1: "~ο~οιην|~ο~οῑμῐ", _, "~ο~οῑμεν|~ο~οιημεν";
    future_opt_act_2: "~ο~οιης|~ο~οῑς", "~ο~οῑτον|~ο~οιητον", "~ο~οῑτε|~ο~οιητε";
    future_opt_act_3: "~ο~οιη|~ο~οῑ", "~ο~οιτην|~ο~οιητην", "~ο~οῑεν|~ο~οιησᾰν";
    future_opt_mid_1: "~ο~οιμην", _, "~ο~οιμεθᾰ";
    future_opt_mid_2: "~ο~οῑο", _, _;
    future_opt_mid_3: _, "~ο~οισθην", _;
    aorist_subj_act_3: _, _, "~ο~σωσῐ|~ο~σωσῐν";
    aorist_opt_act_2: "~ο~σειᾰς|~ο~σαις", _, _;
    aorist_opt_act_3: "~ο~σειε|~ο~σειεν|~ο~σαι", _, "~ο~σειᾰν|~ο~σαιεν";
    perfect_ind_act_1: "~ω~κᾰ", _, "~ω~κᾰμεν";
    perfect_ind_act_2: _, "~ω~κᾰτον", _;
    perfect_ind_act_3: "~ω~κε|~ω~κεν", _, "~ω~κᾱσῐ|~ω~κᾱσῐν";
    perfect_ind_mid_1: "~ω~σμαι", _, "~ω~σμεθᾰ";
    perfect_ind_mid_2: "~ω~σαι", "~ω~σθον", _;
    perfect_ind_mid_3: "~ω~σται", _, "~ω~δᾰται";
    perfect_ind_pass_2: _, _, "~ω~σθε";
    perfect_subj_act_1: "~ω~κω", _, _;
    perfect_subj_act_2: "~ω~κῃς", "~ω~κητον", "~ω~κητε";
    perfect_subj_act_3: "~ω~κῃ", _, "~ω~κωσῐ|~ω~κωσῐν";
    perfect_opt_act_1: "~ω~κοιμῐ|~ω~κοιην", _, "~ω~κοιμεν";
    perfect_opt_act_2: "~ω~κοις|~ω~κοιης", _, _;
    perfect_opt_act_3: "~ω~κοι|~ω~κοιη", "~ω~κοιτην", _;
    perfect_imp_act_2: "~ω~κε", _, "~ω~κετε";
    perfect_imp_act_3: _, _, "~ω~κοντων";
    perfect_imp_mid_2: _, _, "~ω~σθε";
    perfect_imp_mid_3: "~ω~σθω", _, "~ω~σθων";
    perfect_imp_pass_2: "~ω~σο", _, "~ω~σθε";
    perfect_imp_pass_3: _, "~ω~σθων", "~ω~σθων";
    pluperfect_mid_1: "~ω~σμην", _, _;
    pluperfect_mid_2: "~ω~σο", "~ω~σθον", _;
    pluperfect_mid_3: "~ω~στο", _, "~ω~δᾰτο";
    pluperfect_pass_1: _, _, "~ω~σμεθᾰ";
    pluperfect_pass_2: _, _, "~ω~σθε";
    pluperfect_pass_3: _, "~ω~σθην", _;
    infinitive_pres: "~ο~ζειν", _, "~ο~ζεσθαι";
    infinitive_fut: "~ο~εῑν", _, _;
    infinitive_perf: _, "~ω~σθαι", _;
    participle_pres: "~ο~ζων", _, "~ο~ζομενος";
    participle_perf: "~ω~κως", _, "~ω~σμενος";
    ),
    // Κᾰκοω: 3 words (κᾰκοω, ἐξῐσοω, ἡλοω). Like Γῡροω, except in 62 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("κᾰκοω", 3, "~οω", like Γῡροω;
    stem aorist_ind: "~ωσ- like εὐλογεω";
    stem aorist_ind_pass: "~ωθην- like ἐλπω";
    imperfect_act_1: "~ουν", _, "~οῡμεν";
    imperfect_act_2: "~ους", "~οῡτον", "~οῡτε";
    imperfect_act_3: "~ου", "~ουτην", "~ουν";
    imperfect_mid_1: "~ουμην", _, "~ουμεθᾰ";
    imperfect_mid_2: "~οῡ", _, _;
    imperfect_mid_3: "~οῡτο", _, _;
    imperfect_pass_2: _, "~οῡσθον", "~οῡσθε";
    imperfect_pass_3: _, "~ουσθην", "~οῡντο";
    future_ind_act_3: _, _, "~ωσουσῐ|~ωσουσῐν";
    perfect_ind_act_1: "~ωκᾰ", _, "~ωκᾰμεν";
    perfect_ind_act_2: _, "~ωκᾰτον", _;
    perfect_ind_act_3: "~ωκε|~ωκεν", _, "~ωκᾱσῐ|~ωκᾱσῐν";
    perfect_ind_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    perfect_ind_mid_2: "~ωσαι", "~ωσθον", _;
    perfect_ind_mid_3: "~ωται", _, "~ωνται";
    perfect_ind_pass_2: _, _, "~ωσθε";
    perfect_subj_act_1: "~ωκω", _, _;
    perfect_subj_act_2: "~ωκῃς", "~ωκητον", "~ωκητε";
    perfect_subj_act_3: "~ωκῃ", _, "~ωκωσῐ|~ωκωσῐν";
    perfect_opt_act_1: "~ωκοιμῐ|~ωκοιην", _, "~ωκοιμεν";
    perfect_opt_act_2: "~ωκοις|~ωκοιης", _, _;
    perfect_opt_act_3: "~ωκοι|~ωκοιη", "~ωκοιτην", _;
    perfect_imp_act_2: "~ωκε", _, "~ωκετε";
    perfect_imp_act_3: _, _, "~ωκοντων";
    perfect_imp_mid_2: _, _, "~ωσθε";
    perfect_imp_mid_3: "~ωσθω", _, "~ωσθων";
    perfect_imp_pass_2: "~ωσο", _, "~ωσθε";
    perfect_imp_pass_3: _, "~ωσθων", "~ωσθων";
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
    // Δεω: 3 words (δεω, φλεω, κλεω). Like Μᾰρτῠρεω, except in 58 cells: imp_act_2 sg, future_ind_act_3 pl, future_ind_mid_2 sg, future_ind_pass_2 sg, aorist_subj_act_3 pl, aorist_subj_pass_3 pl, ….
    conjugation!("δεω", 3, "~εω", like Μᾰρτῠρεω;
    stem future_pass: "~εθησ- like ἐχω";
    stem aorist_ind: "~ησ- like ἐχω";
    stem aorist_ind_pass: "~εθην- like ἐχω";
    stem aorist_pass: "~εθ- like ἐχω";
    stem pluperfect_act: "~εδεκειν- like κᾰλεω";
    imp_act_2: "~εῑ", _, _;
    future_ind_act_3: _, _, "~ησουσῐ";
    future_ind_mid_2: "~ησῃ", _, _;
    future_ind_pass_2: derived, _, _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_2: "~ησειᾰς", _, _;
    aorist_opt_act_3: "~ησειε", _, "~ησειᾰν";
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_1: "~εδεκᾰ", _, "~εδεκᾰμεν";
    perfect_ind_act_2: _, "~εδεκᾰτον", _;
    perfect_ind_act_3: derived, _, "~εδεκᾱσῐ";
    perfect_ind_mid_1: "~εδεμαι", _, "~εδεμεθᾰ";
    perfect_ind_mid_2: "~εδεσαι", "~εδεσθον", _;
    perfect_ind_mid_3: "~εδεται", _, "~εδενται";
    perfect_ind_pass_2: _, _, "~εδεσθε";
    perfect_subj_act_1: "~εδεκω", _, _;
    perfect_subj_act_2: "~εδεκῃς", "~εδεκητον", "~εδεκητε";
    perfect_subj_act_3: "~εδεκῃ", _, "~εδεκωσῐ";
    perfect_opt_act_1: "~εδεκοιμῐ", _, "~εδεκοιμεν";
    perfect_opt_act_2: "~εδεκοις", _, _;
    perfect_opt_act_3: derived, "~εδεκοιτην", _;
    perfect_imp_act_2: "~εδεκε", _, "~εδεκετε";
    perfect_imp_act_3: _, _, "~εδεκοντων";
    perfect_imp_mid_2: _, _, "~εδεσθε";
    perfect_imp_mid_3: "~εδεσθω", _, "~εδεσθων";
    perfect_imp_pass_2: "~εδεσο", _, "~εδεσθε";
    perfect_imp_pass_3: _, "~εδεσθων", "~εδεσθων";
    pluperfect_mid_1: "ἐ~εδεμην", _, _;
    pluperfect_mid_2: "ἐ~εδεσο", "ἐ~εδεσθον", _;
    pluperfect_mid_3: "ἐ~εδετο", _, "ἐ~εδεντο";
    pluperfect_pass_1: _, _, "ἐ~εδεμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~εδεσθε";
    pluperfect_pass_3: _, "ἐ~εδεσθην", _;
    infinitive_perf: _, "~εδεσθαι", _;
    participle_perf: "~εδεκως", _, "~εδεμενος";
    ),
    // Ἀποζω: 3 words (ζω, ἀποζω, ἀνᾰζω). Like Περιειμι, except in 92 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
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
    imp_act_3: "~ητω", _, "~ωντων";
    imperfect_act_1: "ἐ~ων", _, "ἐ~ωμεν";
    imperfect_act_2: "ἐ~ης", "ἐ~ητον", "ἐ~ητε";
    imperfect_act_3: "ἐ~η", "ἐ~ητην", "ἐ~ων";
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", "~ησετε";
    future_ind_act_3: _, _, "~ησουσῐ";
    future_ind_mid_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_mid_2: "~ησῃ", "~ησεσθον", _;
    future_ind_mid_3: "~ησεται", _, _;
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: _, "~ησοιτον", "~ησοιτε";
    future_opt_act_3: "~ησοι", "~ησοιτην", "~ησοιεν";
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_2: "~ησοιο", _, _;
    future_opt_mid_3: _, "~ησοισθην", _;
    aorist_subj_act_1: "~ησω", _, _;
    aorist_subj_act_2: "~ησῃς", "~ησητον", _;
    aorist_subj_mid_1: "~ησωμαι", _, _;
    aorist_subj_mid_2: "~ησῃ", "~ησησθον", _;
    aorist_opt_act_1: "~ησαιμῐ", _, "~ησαιμεν";
    aorist_opt_act_2: "~ησειᾰς", _, _;
    aorist_opt_act_3: "~ησειε", "~ησαιτην", "~ησειᾰν";
    aorist_opt_mid_1: "~ησαιμην", _, _;
    aorist_opt_mid_2: "~ησαιο", _, _;
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
    perfect_opt_act_1: "ἐ~ηκοιμῐ", _, "ἐ~ηκοιμεν";
    perfect_opt_act_2: "ἐ~ηκοις", _, _;
    perfect_opt_act_3: _, "ἐ~ηκοιτην", _;
    perfect_imp_act_2: "ἐ~ηκε", _, "ἐ~ηκετε";
    perfect_imp_act_3: _, _, "ἐ~ηκοντων";
    infinitive_pres: "~ην", _, _;
    infinitive_fut: "~ησειν", _, _;
    infinitive_aor: "~ησαι", _, _;
    participle_aor: "~ησᾱς", "~ησᾰμενος", _;
    participle_perf: "ἐ~ηκως", _, _;
    ),
    // Πλᾰναω: 3 words (δαμναω, πλᾰναω, γαναω). Like Μῑγνῡμῐ, except in 163 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("πλᾰναω", 3, "~ναω", like Μῑγνῡμῐ;
    stem aorist_ind: "~νησ- like ἐχω";
    stem aorist_ind_pass: "~νηθην- like ἐχω";
    stem aorist_pass: "~νηθ- like ἐχω";
    stem pluperfect_act: "~νηκειν- like ἐχω";
    ind_act_1: "~νω", _, "~νωμεν";
    ind_act_2: "~νᾳς", _, _;
    ind_act_3: "~νᾳ", "~νᾱτον", "~νωσῐ";
    ind_mid_1: "~νωμαι", _, "~νωμεθᾰ";
    ind_mid_2: "~νᾳ", "~νᾱσθον", _;
    ind_pass_2: _, _, "~νᾱσθε";
    ind_pass_3: "~νᾱται", _, "~νωνται";
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
    opt_mid_2: "~νῳο", "~νῳσθον", _;
    opt_mid_3: "~νῳτο", "~νῳσθην", "~νῳντο";
    opt_pass_2: _, _, "~νῳσθε";
    imp_act_2: "~νᾱ", "~νᾱτον", "~νᾱτε";
    imp_mid_2: "~νω", _, "~νᾱσθε";
    imp_mid_3: "~νᾱσθω", _, "~νᾱσθων";
    imp_pass_2: _, _, "~νᾱσθε";
    imp_pass_3: _, "~νᾱσθων", "~νᾱσθων";
    imperfect_act_1: "ἐ~νων", _, "ἐ~νωμεν";
    imperfect_act_2: "ἐ~νᾱς", "ἐ~νᾱτον", "ἐ~νᾱτε";
    imperfect_act_3: "ἐ~νᾱ", "ἐ~νᾱτην", "ἐ~νων";
    imperfect_mid_1: "ἐ~νωμην", _, "ἐ~νωμεθᾰ";
    imperfect_mid_2: "ἐ~νω", _, _;
    imperfect_mid_3: "ἐ~νᾱτο", _, _;
    imperfect_pass_2: _, "ἐ~νᾱσθον", "ἐ~νᾱσθε";
    imperfect_pass_3: _, "ἐ~νᾱσθην", "ἐ~νωντο";
    future_ind_mid_1: "~γησομαι", _, "~γησομεθᾰ";
    future_ind_mid_2: "~γησῃ", "~γησεσθον", _;
    future_ind_mid_3: "~γησεται", _, _;
    future_opt_mid_1: "~γησοιμην", _, "~γησοιμεθᾰ";
    future_opt_mid_2: "~γησοιο", _, _;
    future_opt_mid_3: _, "~γησοισθην", _;
    aorist_subj_act_1: "~νησω", _, _;
    aorist_subj_act_2: "~νησῃς", "~νησητον", _;
    aorist_subj_mid_1: "~νησωμαι", _, _;
    aorist_subj_mid_2: "~νησῃ", "~νησησθον", _;
    aorist_opt_act_1: "~νησαιμῐ", _, "~νησαιμεν";
    aorist_opt_act_2: "~νησειᾰς", _, _;
    aorist_opt_act_3: "~νησειε", "~νησαιτην", "~νησειᾰν";
    aorist_opt_mid_1: "~νησαιμην", _, _;
    aorist_opt_mid_2: "~νησαιο", _, _;
    aorist_imp_act_2: "~νησον", _, "~νησᾰτε";
    aorist_imp_act_3: _, _, "~νησᾰντων";
    aorist_imp_mid_2: "~νησαι", "~νησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~νησᾰσθων";
    perfect_ind_act_1: "%ε~νηκᾰ", _, "%ε~νηκᾰμεν";
    perfect_ind_act_2: _, "%ε~νηκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~νηκᾱσῐ";
    perfect_ind_mid_1: "%ε~νημαι", _, "%ε~νημεθᾰ";
    perfect_ind_mid_2: "%ε~νησαι", "%ε~νησθον", _;
    perfect_ind_mid_3: "%ε~νηται", _, "%ε~νηνται";
    perfect_ind_pass_2: _, _, "%ε~νησθε";
    perfect_subj_act_1: "%ε~νηκω", _, _;
    perfect_subj_act_2: "%ε~νηκῃς", "%ε~νηκητον", "%ε~νηκητε";
    perfect_subj_act_3: "%ε~νηκῃ", _, "%ε~νηκωσῐ";
    perfect_opt_act_1: "%ε~νηκοιμῐ", _, "%ε~νηκοιμεν";
    perfect_opt_act_2: "%ε~νηκοις", _, _;
    perfect_opt_act_3: _, "%ε~νηκοιτην", _;
    perfect_imp_act_2: "%ε~νηκε", _, "%ε~νηκετε";
    perfect_imp_act_3: _, _, "%ε~νηκοντων";
    perfect_imp_mid_2: _, _, "%ε~νησθε";
    perfect_imp_mid_3: "%ε~νησθω", _, "%ε~νησθων";
    perfect_imp_pass_2: "%ε~νησο", _, "%ε~νησθε";
    perfect_imp_pass_3: _, "%ε~νησθων", "%ε~νησθων";
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
    // Ἀμειβω: 3 words (στειβω, λειβω, ἀμειβω). Like Μῑγνῡμῐ, except in 94 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀμειβω", 3, "~ιβω", like Μῑγνῡμῐ;
    stem aorist_ind: "~ιψ- like ποιεω";
    ind_act_1: "~ιβω", _, "~ιβομεν";
    ind_act_2: "~ιβεις", _, _;
    ind_act_3: "~ιβει", "~ιβετον", "~ιβουσῐ|~ιβουσῐν";
    ind_mid_1: "~ιβομαι", _, "~ιβομεθᾰ";
    ind_mid_2: "~ιβῃ|~ιβει", "~ιβεσθον", _;
    ind_pass_2: _, _, "~ιβεσθε";
    ind_pass_3: "~ιβεται", _, "~ιβονται";
    subj_act_1: "~ιβω", _, "~ιβωμεν";
    subj_act_2: "~ιβῃς", _, _;
    subj_act_3: "~ιβῃ", "~ιβητον", "~ιβωσῐ|~ιβωσῐν";
    subj_mid_1: _, _, "~ιβωμεθᾰ";
    subj_mid_2: _, "~ιβησθον", _;
    subj_mid_3: "~ιβηται", _, "~ιβωνται";
    subj_pass_1: "~ιβωμαι", _, _;
    subj_pass_2: _, _, "~ιβησθε";
    opt_act_1: "~ιβοιμῐ", _, "~ιβοιμεν";
    opt_act_2: "~ιβοις", "~ιβοιτον", _;
    opt_act_3: "~ιβοι", "~ιβοιτην", "~ιβοιεν";
    opt_mid_1: "~ιβοιμην", _, "~ιβοιμεθᾰ";
    opt_mid_2: "~ιβοιο", "~ιβοισθον", _;
    opt_mid_3: "~ιβοιτο", "~ιβοισθην", "~ιβοιντο";
    opt_pass_2: _, _, "~ιβοισθε";
    imp_act_2: "~ῑβε", "~ιβετον", "~ιβετε";
    imp_mid_2: "~ιβου", _, "~ιβεσθε";
    imp_mid_3: "~ιβεσθω", _, "~ιβεσθων";
    imp_pass_2: _, _, "~ιβεσθε";
    imp_pass_3: _, "~ιβεσθων", "~ιβεσθων";
    imperfect_act_1: "ἐ~ιβον", _, "ἐ~ιβομεν";
    imperfect_act_2: "ἐ~ιβες", "ἐ~ιβετον", "ἐ~ιβετε";
    imperfect_act_3: "ἐ~ιβε|ἐ~ιβεν", "ἐ~ιβετην", "ἐ~ιβον";
    imperfect_mid_1: "ἐ~ιβομην", _, "ἐ~ιβομεθᾰ";
    imperfect_mid_2: "ἐ~ιβου", _, _;
    imperfect_mid_3: "ἐ~ιβετο", _, _;
    imperfect_pass_2: _, "ἐ~ιβεσθον", "ἐ~ιβεσθε";
    imperfect_pass_3: _, "ἐ~ιβεσθην", "ἐ~ιβοντο";
    aorist_subj_act_1: "~ιψω", _, _;
    aorist_subj_act_2: "~ιψῃς", "~ιψητον", _;
    aorist_subj_act_3: _, _, "~ιψωσῐ|~ιψωσῐν";
    aorist_subj_mid_1: "~ιψωμαι", _, _;
    aorist_subj_mid_2: "~ιψῃ", "~ιψησθον", _;
    aorist_opt_act_1: "~ιψαιμῐ", _, "~ιψαιμεν";
    aorist_opt_act_2: "~ιψειᾰς|~ιψαις", _, _;
    aorist_opt_act_3: "~ιψειε|~ιψειεν|~ιψαι", "~ιψαιτην", "~ιψειᾰν|~ιψαιεν";
    aorist_opt_mid_1: "~ιψαιμην", _, _;
    aorist_opt_mid_2: "~ιψαιο", _, _;
    aorist_imp_act_2: "~ῑψον", _, "~ιψᾰτε";
    aorist_imp_act_3: _, _, "~ιψᾰντων";
    aorist_imp_mid_2: "~ῑψαι", "~ιψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ιψᾰσθων";
    infinitive_pres: "~ιβειν", _, "~ιβεσθαι";
    infinitive_aor: "~ῑψαι", _, _;
    participle_pres: "~ιβων", _, "~ιβομενος";
    participle_aor: "~ιψᾱς", "~ιψᾰμενος", _;
    ),
    // Δῐᾰρπᾰζω: 3 words (ἐξᾰφᾰνῐζω, δῐᾰρπᾰζω, ἐξᾰττῐκῐζω). Like Ἐπᾰϊω, except in 102 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("δῐᾰρπᾰζω", 3, "~ᾰ~ζω", like Ἐπᾰϊω;
    stem future_pass: "~ᾰ~σθησ- like ποιεω";
    stem aorist_pass: "~ᾰ~σθ- like ποιεω";
    stem pluperfect_act: "~η~κ- like εὐλογεω";
    ind_act_1: "~ᾰ~ζω", _, "~ᾰ~ζομεν";
    ind_act_2: "~ᾰ~ζεις", _, _;
    ind_act_3: "~ᾰ~ζει", "~ᾰ~ζετον", "~ᾰ~ζουσῐ|~ᾰ~ζουσῐν";
    ind_mid_1: "~ᾰ~ζομαι", _, "~ᾰ~ζομεθᾰ";
    ind_mid_2: "~ᾰ~ζῃ|~ᾰ~ζει", "~ᾰ~ζεσθον", _;
    ind_pass_2: _, _, "~ᾰ~ζεσθε";
    ind_pass_3: "~ᾰ~ζεται", _, "~ᾰ~ζονται";
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
    opt_mid_2: "~ᾰ~ζοιο", "~ᾰ~ζοισθον", _;
    opt_mid_3: "~ᾰ~ζοιτο", "~ᾰ~ζοισθην", "~ᾰ~ζοιντο";
    opt_pass_2: _, _, "~ᾰ~ζοισθε";
    imp_act_2: "~ᾰ~ζε", "~ᾰ~ζετον", "~ᾰ~ζετε";
    imp_mid_2: "~ᾰ~ζου", _, "~ᾰ~ζεσθε";
    imp_mid_3: "~ᾰ~ζεσθω", _, "~ᾰ~ζεσθων";
    imp_pass_2: _, _, "~ᾰ~ζεσθε";
    imp_pass_3: _, "~ᾰ~ζεσθων", "~ᾰ~ζεσθων";
    imperfect_act_1: "~η~ζον", _, "~η~ζομεν";
    imperfect_act_2: "~η~ζες", "~η~ζετον", "~η~ζετε";
    imperfect_act_3: "~η~ζε|~η~ζεν", "~η~ζετην", "~η~ζον";
    imperfect_mid_1: "~η~ζομην", _, "~η~ζομεθᾰ";
    imperfect_mid_2: "~η~ζου", _, _;
    imperfect_mid_3: "~η~ζετο", _, _;
    imperfect_pass_2: _, "~η~ζεσθον", "~η~ζεσθε";
    imperfect_pass_3: _, "~η~ζεσθην", "~η~ζοντο";
    future_ind_act_1: "~ᾰ~ω", _, "~ᾰ~οῡμεν";
    future_ind_act_2: "~ᾰ~εῑς", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    future_ind_act_3: _, _, "~ᾰ~οῡσῐ|~ᾰ~οῡσῐν";
    future_ind_mid_1: "~ᾰ~οῡμαι", _, "~ᾰ~ουμεθᾰ";
    future_ind_mid_2: "~ᾰ~εῑ|~ᾰ~ῃ", "~ᾰ~εῑσθον", _;
    future_ind_mid_3: "~ᾰ~εῑται", _, _;
    future_opt_act_1: "~ᾰ~οιην|~ᾰ~οῑμῐ", _, "~ᾰ~οῑμεν|~ᾰ~οιημεν";
    future_opt_act_2: "~ᾰ~οιης|~ᾰ~οῑς", "~ᾰ~οῑτον|~ᾰ~οιητον", "~ᾰ~οῑτε|~ᾰ~οιητε";
    future_opt_act_3: "~ᾰ~οιη|~ᾰ~οῑ", "~ᾰ~οιτην|~ᾰ~οιητην", "~ᾰ~οῑεν|~ᾰ~οιησᾰν";
    future_opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    future_opt_mid_2: "~ᾰ~οῑο", _, _;
    future_opt_mid_3: _, "~ᾰ~οισθην", _;
    perfect_ind_act_3: "~η~κε|~η~κεν", _, "~η~κᾱσῐ|~η~κᾱσῐν";
    perfect_ind_mid_3: _, _, "~η~δᾰται";
    perfect_subj_act_3: _, _, "~η~κωσῐ|~η~κωσῐν";
    perfect_opt_act_1: "~η~κοιμῐ|~η~κοιην", _, _;
    perfect_opt_act_2: "~η~κοις|~η~κοιης", _, _;
    perfect_opt_act_3: "~η~κοι|~η~κοιη", _, _;
    pluperfect_mid_3: _, _, "~η~δᾰτο";
    infinitive_pres: "~ᾰ~ζειν", _, "~ᾰ~ζεσθαι";
    infinitive_fut: "~ᾰ~εῑν", _, _;
    participle_pres: "~ᾰ~ζων", _, "~ᾰ~ζομενος";
    ),
    // Ὀμνῠω: 3 words (ὀφελλω, ὀμνῠω, ὀρῠττω). Like Μῑγνῡμῐ, except in 70 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ὀμνῠω", 3, "ὀ~ω", like Μῑγνῡμῐ;
    ind_act_1: "ὀ~ω", _, "ὀ~ομεν";
    ind_act_2: "ὀ~εις", _, _;
    ind_act_3: "ὀ~ει", "ὀ~ετον", "ὀ~ουσῐ|ὀ~ουσῐν";
    ind_mid_1: "ὀ~ομαι", _, "ὀ~ομεθᾰ";
    ind_mid_2: "ὀ~ῃ|ὀ~ει", "ὀ~εσθον", _;
    ind_pass_2: _, _, "ὀ~εσθε";
    ind_pass_3: "ὀ~εται", _, "ὀ~ονται";
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
    opt_mid_2: "ὀ~οιο", "ὀ~οισθον", _;
    opt_mid_3: "ὀ~οιτο", "ὀ~οισθην", "ὀ~οιντο";
    opt_pass_2: _, _, "ὀ~οισθε";
    imp_act_2: "ὀ~ε", "ὀ~ετον", "ὀ~ετε";
    imp_mid_2: "ὀ~ου", _, "ὀ~εσθε";
    imp_mid_3: "ὀ~εσθω", _, "ὀ~εσθων";
    imp_pass_2: _, _, "ὀ~εσθε";
    imp_pass_3: _, "ὀ~εσθων", "ὀ~εσθων";
    imperfect_act_1: "ὠ~ον", _, "ὠ~ομεν";
    imperfect_act_2: "ὠ~ες", "ὠ~ετον", "ὠ~ετε";
    imperfect_act_3: "ὠ~ε|ὠ~εν", "ὠ~ετην", "ὠ~ον";
    imperfect_mid_1: "ὠ~ομην", _, "ὠ~ομεθᾰ";
    imperfect_mid_2: "ὠ~ου", _, _;
    imperfect_mid_3: "ὠ~ετο", _, _;
    imperfect_pass_2: _, "ὠ~εσθον", "ὠ~εσθε";
    imperfect_pass_3: _, "ὠ~εσθην", "ὠ~οντο";
    infinitive_pres: "ὀ~ειν", _, "ὀ~εσθαι";
    participle_pres: "ὀ~ων", _, "ὀ~ομενος";
    ),
    // Ψᾱω: 3 words (δῐψᾱω, νᾱω, ψᾱω). Like Νῑκαω, except in 68 cells: ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ψᾱω", 3, "~ᾱω", like Νῑκαω;
    ind_act_2: "~ῃς", _, _;
    ind_act_3: "~ῃ", "~ητον", _;
    ind_mid_2: "~ῃ", "~ησθον", _;
    ind_pass_2: _, _, "~ησθε";
    ind_pass_3: "~ηται", _, _;
    subj_act_2: "~ῃς", _, _;
    subj_act_3: "~ῃ", "~ητον", _;
    subj_mid_2: _, "~ησθον", _;
    subj_mid_3: "~ηται", _, _;
    subj_pass_2: _, _, "~ησθε";
    imp_act_2: "~η", "~ητον", "~ητε";
    imp_mid_2: _, _, "~ησθε";
    imp_mid_3: "~ησθω", _, "~ησθων";
    imp_pass_2: _, _, "~ησθε";
    imp_pass_3: _, "~ησθων", "~ησθων";
    imperfect_act_2: "ἐ~ης", "ἐ~ητον", "ἐ~ητε";
    imperfect_act_3: "ἐ~η", "ἐ~ητην", _;
    imperfect_mid_3: "ἐ~ητο", _, _;
    imperfect_pass_2: _, "ἐ~ησθον", "ἐ~ησθε";
    imperfect_pass_3: _, "ἐ~ησθην", _;
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_1: "ἐ~ημαι", _, "ἐ~ημεθᾰ";
    perfect_ind_mid_2: "ἐ~ησαι", "ἐ~ησθον", _;
    perfect_ind_mid_3: "ἐ~ηται", _, "ἐ~ηνται";
    perfect_ind_pass_2: _, _, "ἐ~ησθε";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: _, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, _, "ἐ~ησθε";
    perfect_imp_mid_3: "ἐ~ησθω", _, "ἐ~ησθων";
    perfect_imp_pass_2: "ἐ~ησο", _, "ἐ~ησθε";
    perfect_imp_pass_3: _, "ἐ~ησθων", "ἐ~ησθων";
    infinitive_pres: "~ην", _, "~ησθαι";
    infinitive_perf: _, "ἐ~ησθαι", _;
    participle_perf: derived, _, "ἐ~ημενος";
    ),
    // Ἐπῐτῐθημῐ: 3 words (ἐπῐτῐθημῐ, συνεπῐτῐθημῐ, ξυνεπῐτῐθημῐ). Like Ξαινω, except in 183 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐπῐτῐθημῐ", 3, "~ῐτῐ~ημῐ", like Ξαινω;
    stem future_pass: "~ῐ%ε~ησ- like ποιεω";
    stem aorist_ind: "~ε~- like ἐπῐτῐθημῐ";
    ind_act_1: "~ῐ%ῐ~ημῐ", _, "~ῐ%ῐ~εμεν";
    ind_act_2: "~ῐ%ῐ~ης", _, _;
    ind_act_3: "~ῐ%ῐ~ησῐ|~ῐ%ῐ~ησῐν", "~ῐ%ῐ~ετον", "~ῐ%ῐ~εᾱσῐ|~ῐ%ῐ~εᾱσῐν";
    ind_mid_1: "~ῐ%ῐ~εμαι", _, "~ῐ%ῐ~εμεθᾰ";
    ind_mid_2: "~ῐ%ῐ~εσαι", "~ῐ%ῐ~εσθον", _;
    ind_pass_2: _, _, "~ῐ%ῐ~εσθε";
    ind_pass_3: "~ῐ%ῐ~εται", _, "~ῐ%ῐ~ενται";
    subj_act_1: "~ῐ%ῐ~ω", _, "~ῐ%ῐ~ωμεν";
    subj_act_2: "~ῐ%ῐ~ῃς", _, _;
    subj_act_3: "~ῐ%ῐ~ῃ", "~ῐ%ῐ~ητον", "~ῐ%ῐ~ωσῐ|~ῐ%ῐ~ωσῐν";
    subj_mid_1: _, _, "~ῐ%ῐ~ωμεθᾰ";
    subj_mid_2: _, "~ῐ%ῐ~ησθον", _;
    subj_mid_3: "~ῐ%ῐ~ηται", _, "~ῐ%ῐ~ωνται";
    subj_pass_1: "~ῐ%ῐ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῐ%ῐ~ησθε";
    opt_act_1: "~ῐ%ῐ~ειην", _, "~ῐ%ῐ~εῑμεν|~ῐ%ῐ~ειημεν";
    opt_act_2: "~ῐ%ῐ~ειης", "~ῐ%ῐ~εῑτον|~ῐ%ῐ~ειητον", "~ῐ%ῐ~εῑτε|~ῐ%ῐ~ειητε";
    opt_act_3: "~ῐ%ῐ~ειη", "~ῐ%ῐ~ειτην|~ῐ%ῐ~ειητην", "~ῐ%ῐ~εῑεν|~ῐ%ῐ~ειησᾰν";
    opt_mid_1: "~ῐ%ῐ~ειμην", _, "~ῐ%ῐ~ειμεθᾰ|~ῐ%ῐ~οιμεθᾰ";
    opt_mid_2: "~ῐ%ῐ~εῑο", "~ῐ%ῐ~εῑσθον|~ῐ%ῐ~οῑσθον", "~ῐ%ῐ~εῑσθε|~ῐ%ῐ~οῑσθε";
    opt_mid_3: "~ῐ%ῐ~εῑτο|~ῐ%ῐ~οῑτο", "~ῐ%ῐ~εισθην|~ῐ%ῐ~οισθην", "~ῐ%ῐ~εῑντο|~ῐ%ῐ~οῑντο";
    opt_pass_2: _, _, derived;
    imp_act_2: "~ῐ%ῐ~ει", "~ῐ%ῐ~ετον", "~ῐ%ῐ~ετε";
    imp_mid_2: "~ῐ%ῐ~εσο|~ῐ%ῐ~ου", _, "~ῐ%ῐ~εσθε";
    imp_mid_3: "~ῐ%ῐ~εσθω", _, "~ῐ%ῐ~εσθων";
    imp_pass_2: _, _, "~ῐ%ῐ~εσθε";
    imp_pass_3: _, "~ῐ%ῐ~εσθων", "~ῐ%ῐ~εσθων";
    imperfect_act_1: "~ε%ῐ~ην", _, "~ε%ῐ~εμεν";
    imperfect_act_2: "~ε%ῐ~εις", "~ε%ῐ~ετον", "~ε%ῐ~ετε";
    imperfect_act_3: "~ε%ῐ~ει", "~ε%ῐ~ετην", "~ε%ῐ~εσᾰν";
    imperfect_mid_1: "~ε%ῐ~εμην", _, "~ε%ῐ~εμεθᾰ";
    imperfect_mid_2: "~ε%ῐ~εσο", _, _;
    imperfect_mid_3: "~ε%ῐ~ετο", _, _;
    imperfect_pass_2: _, "~ε%ῐ~εσθον", "~ε%ῐ~εσθε";
    imperfect_pass_3: _, "~ε%ῐ~εσθην", "~ε%ῐ~εντο";
    future_ind_act_1: "~ῐ~ησω", _, "~ῐ~ησομεν";
    future_ind_act_2: "~ῐ~ησεις", "~ῐ~ησετον", "~ῐ~ησετε";
    future_ind_act_3: _, _, "~ῐ~ησουσῐ|~ῐ~ησουσῐν";
    future_ind_mid_1: "~ῐ~ησομαι", _, "~ῐ~ησομεθᾰ";
    future_ind_mid_2: "~ῐ~ησῃ|~ῐ~ησει", "~ῐ~ησεσθον", _;
    future_ind_mid_3: "~ῐ~ησεται", _, _;
    future_opt_act_1: "~ῐ~ησοιμῐ", _, "~ῐ~ησοιμεν";
    future_opt_act_2: _, "~ῐ~ησοιτον", "~ῐ~ησοιτε";
    future_opt_act_3: "~ῐ~ησοι", "~ῐ~ησοιτην", "~ῐ~ησοιεν";
    future_opt_mid_1: "~ῐ~ησοιμην", _, "~ῐ~ησοιμεθᾰ";
    future_opt_mid_2: "~ῐ~ησοιο", _, _;
    future_opt_mid_3: _, "~ῐ~ησοισθην", _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ῐ~ηκω|~ῐ~ω", _, "~ῐ~ηκωμεν|~ῐ~ωμεν";
    aorist_subj_act_2: "~ῐ~ηκῃς|~ῐ~ῃς", "~ῐ~ηκητον|~ῐ~ητον", "~ῐ~ηκητε|~ῐ~ητε";
    aorist_subj_act_3: "~ῐ~ηκῃ|~ῐ~ῃ", _, "~ῐ~ηκωσῐ|~ῐ~ηκωσῐν|~ῐ~ωσῐ|~ῐ~ωσῐν";
    aorist_subj_mid_1: "~ῐ~ηκωμαι|~ῐ~ωμαι", _, "~ῐ~ηκωμεθᾰ|~ῐ~ωμεθᾰ";
    aorist_subj_mid_2: derived, "~ῐ~ηκησθον|~ῐ~ησθον", "~ῐ~ηκησθε|~ῐ~ησθε";
    aorist_subj_mid_3: "~ῐ~ηκηται|~ῐ~ηται", _, "~ῐ~ηκωνται|~ῐ~ωνται";
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_1: "~ῐ~ηκαιμῐ|~ῐ~ειην", _, "~ῐ~ηκαιμεν|~ῐ~εῑμεν|~ῐ~ειημεν";
    aorist_opt_act_2: "~ῐ~ηκειᾰς|~ῐ~ηκαις|~ῐ~ειης", "~ῐ~ηκαιτον|~ῐ~εῑτον|~ῐ~ειητον", "~ῐ~ηκαιτε|~ῐ~εῑτε|~ῐ~ειητε";
    aorist_opt_act_3: "~ῐ~ηκειε|~ῐ~ηκειεν|~ῐ~ηκαι|~ῐ~ειη", "~ῐ~ηκαιτην|~ῐ~ειτην|~ῐ~ειητην", "~ῐ~ηκειᾰν|~ῐ~ηκαιεν|~ῐ~εῑεν|~ῐ~ειησᾰν";
    aorist_opt_mid_1: "~ῐ~ηκαιμην|~ῐ~ειμην", _, "~ῐ~ηκαιμεθᾰ|~ῐ~ειμεθᾰ";
    aorist_opt_mid_2: "~ῐ~ηκαιο|~ῐ~εῑο", "~ῐ~ηκαισθον|~ῐ~εῑσθον", "~ῐ~ηκαισθε|~ῐ~εῑσθε";
    aorist_opt_mid_3: "~ῐ~ηκαιτο|~ῐ~εῑτο|~ῐ~οῑτο", "~ῐ~ηκαισθην|~ῐ~εισθην", "~ῐ~ηκαιντο|~ῐ~εῑντο";
    aorist_imp_act_2: "~ῐ~ηκον|~ῐ~ες", "~ῐ~ηκᾰτον|~ῐ~ετον", "~ῐ~ηκᾰτε|~ῐ~ετε";
    aorist_imp_act_3: "~ῐ~ηκᾰτω|~ῐ~ετω", "~ῐ~ηκᾰτων|~ῐ~ετων", "~ῐ~ηκᾰντων|~ῐ~εντων";
    aorist_imp_mid_2: "~ῐ~ηκαι|~ῐ~ου", "~ῐ~ηκᾰσθον|~ῐ~εσθον", "~ῐ~ηκᾰσθε|~ῐ~εσθε";
    aorist_imp_mid_3: "~ῐ~ηκᾰσθω|~ῐ~εσθω", "~ῐ~ηκᾰσθων|~ῐ~εσθων", derived;
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: "~ῐ%ε~εικᾰ", _, "~ῐ%ε~εικᾰμεν";
    perfect_ind_act_2: _, "~ῐ%ε~εικᾰτον", _;
    perfect_ind_act_3: "~ῐ%ε~εικε|~ῐ%ε~εικεν", _, "~ῐ%ε~εικᾱσῐ|~ῐ%ε~εικᾱσῐν";
    perfect_ind_mid_1: "~ῐ%ε~ειμαι", _, "~ῐ%ε~ειμεθᾰ";
    perfect_ind_mid_2: "~ῐ%ε~εισαι", "~ῐ%ε~εισθον", _;
    perfect_ind_mid_3: "~ῐ%ε~ειται", _, "~ῐ%ε~εινται";
    perfect_ind_pass_2: _, _, "~ῐ%ε~εισθε";
    perfect_subj_act_1: "~ῐ%ε~εικω", _, _;
    perfect_subj_act_2: "~ῐ%ε~εικῃς", "~ῐ%ε~εικητον", "~ῐ%ε~εικητε";
    perfect_subj_act_3: "~ῐ%ε~εικῃ", _, "~ῐ%ε~εικωσῐ|~ῐ%ε~εικωσῐν";
    perfect_opt_act_1: "~ῐ%ε~εικοιμῐ|~ῐ%ε~εικοιην", _, "~ῐ%ε~εικοιμεν";
    perfect_opt_act_2: "~ῐ%ε~εικοις|~ῐ%ε~εικοιης", _, _;
    perfect_opt_act_3: "~ῐ%ε~εικοι|~ῐ%ε~εικοιη", "~ῐ%ε~εικοιτην", _;
    perfect_imp_act_2: "~ῐ%ε~εικε", _, "~ῐ%ε~εικετε";
    perfect_imp_act_3: _, _, "~ῐ%ε~εικοντων";
    perfect_imp_mid_2: _, _, "~ῐ%ε~εισθε";
    perfect_imp_mid_3: "~ῐ%ε~εισθω", _, "~ῐ%ε~εισθων";
    perfect_imp_pass_2: "~ῐ%ε~εισο", _, "~ῐ%ε~εισθε";
    perfect_imp_pass_3: _, "~ῐ%ε~εισθων", "~ῐ%ε~εισθων";
    infinitive_pres: "~ῐ%ῐ~εναι", _, "~ῐ%ῐ~εσθαι";
    infinitive_fut: "~ῐ~ησειν", _, _;
    infinitive_aor: "~ῐ~ηκαι|~ῐ~εῑναι", "~ῐ~ηκᾰσθαι|~ῐ~εσθαι", _;
    infinitive_perf: _, "~ῐ%ε~εῑσθαι", _;
    participle_pres: "~ῐ%ῐ~εις", _, "~ῐ%ῐ~εμενος";
    participle_aor: "~ῐ~ηκᾱς|~ῐ~εις", "~ῐ~ηκᾰμενος|~ῐ~εμενος", _;
    participle_perf: "~ῐ%ε~εικως", _, "~ῐ%ε~ειμενος";
    ),
    // Μνημονευω: 3 words (μνημονευω, στρᾰτευω, στρᾰτοπεδευω). Like Βουλευω, except in 52 cells: future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, aorist_opt_pass_3 du, ….
    conjugation!("μνημονευω", 3, "~υω", like Βουλευω;
    future_ind_pass_2: derived, _, _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    perfect_ind_act_1: "ἐ~υκᾰ", _, "ἐ~υκᾰμεν";
    perfect_ind_act_2: _, "ἐ~υκᾰτον", _;
    perfect_ind_act_3: "ἐ~υκε|ἐ~υκεν", _, "ἐ~υκᾱσῐ|ἐ~υκᾱσῐν";
    perfect_ind_mid_1: "ἐ~υμαι", _, "ἐ~υμεθᾰ";
    perfect_ind_mid_2: "ἐ~υσαι", "ἐ~υσθον", _;
    perfect_ind_mid_3: "ἐ~υται", _, "ἐ~υνται";
    perfect_ind_pass_2: _, _, "ἐ~υσθε";
    perfect_subj_act_1: "ἐ~υκω", _, _;
    perfect_subj_act_2: "ἐ~υκῃς", "ἐ~υκητον", "ἐ~υκητε";
    perfect_subj_act_3: "ἐ~υκῃ", _, "ἐ~υκωσῐ|ἐ~υκωσῐν";
    perfect_opt_act_1: "ἐ~υκοιμῐ|ἐ~υκοιην", _, "ἐ~υκοιμεν";
    perfect_opt_act_2: "ἐ~υκοις|ἐ~υκοιης", _, _;
    perfect_opt_act_3: "ἐ~υκοι|ἐ~υκοιη", "ἐ~υκοιτην", _;
    perfect_imp_act_2: "ἐ~υκε", _, "ἐ~υκετε";
    perfect_imp_act_3: _, _, "ἐ~υκοντων";
    perfect_imp_mid_2: _, _, "ἐ~υσθε";
    perfect_imp_mid_3: "ἐ~υσθω", _, "ἐ~υσθων";
    perfect_imp_pass_2: "ἐ~υσο", _, "ἐ~υσθε";
    perfect_imp_pass_3: _, "ἐ~υσθων", "ἐ~υσθων";
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: derived, _, _;
    pluperfect_mid_2: derived, derived, _;
    pluperfect_mid_3: derived, _, derived;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, _;
    infinitive_perf: _, "ἐ~ῡσθαι", _;
    participle_perf: "ἐ~υκως", _, "ἐ~υμενος";
    ),
    // Ἐπαινεω: 3 words (ἐπαινεω, καταπαιδεραστεω, πᾰραινεω). Like Κᾰταφρονεω, except in 124 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, opt_act_1 sg, opt_act_1 pl, opt_act_2 sg, ….
    conjugation!("ἐπαινεω", 3, "~α~εω", like Κᾰταφρονεω;
    stem future_pass: "~α~εθησησ- like ἐχω";
    stem aorist_ind: "~ε~εσ- like ἐλπω";
    stem aorist_ind_pass: "~ε~εθην- like ἐλπω";
    stem aorist_pass: "~α~εθ- like ἐχω";
    ind_act_3: _, _, "~α~οῡσῐ";
    ind_mid_2: "~α~εῑ", _, _;
    subj_act_3: _, _, "~α~ωσῐ";
    opt_act_1: "~α~οιην", _, "~α~οῑμεν";
    opt_act_2: "~α~οιης", "~α~οῑτον", derived;
    opt_act_3: "~α~οιη", "~α~οιτην", "~α~οῑεν";
    imperfect_act_1: "~ε~ουν", _, "~ε~οῡμεν";
    imperfect_act_2: "~ε~εις", "~ε~εῑτον", "~ε~εῑτε";
    imperfect_act_3: "~ε~ει", "~ε~ειτην", "~ε~ουν";
    imperfect_mid_1: "~ε~ουμην", _, "~ε~ουμεθᾰ";
    imperfect_mid_2: "~ε~οῡ", _, _;
    imperfect_mid_3: "~ε~εῑτο", _, _;
    imperfect_pass_2: _, "~ε~εῑσθον", "~ε~εῑσθε";
    imperfect_pass_3: _, "~ε~εισθην", "~ε~οῡντο";
    future_ind_act_1: "~α~εσω", _, "~α~εσομεν";
    future_ind_act_2: "~α~εσεις", "~α~εσετον", "~α~εσετε";
    future_ind_act_3: _, _, "~α~εσουσῐ";
    future_ind_mid_1: "~α~εσομαι", _, "~α~εσομεθᾰ";
    future_ind_mid_2: "~α~εσῃ", "~α~εσεσθον", _;
    future_ind_mid_3: "~α~εσεται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~α~εσοιμῐ", _, "~α~εσοιμεν";
    future_opt_act_2: _, "~α~εσοιτον", "~α~εσοιτε";
    future_opt_act_3: "~α~εσοι", "~α~εσοιτην", "~α~εσοιεν";
    future_opt_mid_1: "~α~εσοιμην", _, "~α~εσοιμεθᾰ";
    future_opt_mid_2: "~α~εσοιο", _, _;
    future_opt_mid_3: _, "~α~εσοισθην", _;
    aorist_subj_act_1: "~α~εσω", _, _;
    aorist_subj_act_2: "~α~εσῃς", "~α~εσητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~α~εσωμαι", _, _;
    aorist_subj_mid_2: "~α~εσῃ", "~α~εσησθον", _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_1: "~α~εσαιμῐ", _, "~α~εσαιμεν";
    aorist_opt_act_2: "~α~εσειᾰς", _, _;
    aorist_opt_act_3: "~α~εσειε", "~α~εσαιτην", "~α~εσειᾰν";
    aorist_opt_mid_1: "~α~εσαιμην", _, _;
    aorist_opt_mid_2: "~α~εσαιο", _, _;
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
    perfect_ind_mid_1: "~α%ε~ημαι", _, "~α%ε~ημεθᾰ";
    perfect_ind_mid_2: "~α%ε~ησαι", "~α%ε~ησθον", _;
    perfect_ind_mid_3: "~α%ε~ηται", _, "~α%ε~ηνται";
    perfect_ind_pass_2: _, _, "~α%ε~ησθε";
    perfect_subj_act_1: "~α%ε~ηκω", _, _;
    perfect_subj_act_2: "~α%ε~ηκῃς", "~α%ε~ηκητον", "~α%ε~ηκητε";
    perfect_subj_act_3: "~α%ε~ηκῃ", _, "~α%ε~ηκωσῐ";
    perfect_opt_act_1: "~α%ε~ηκοιμῐ", _, "~α%ε~ηκοιμεν";
    perfect_opt_act_2: "~α%ε~ηκοις", _, _;
    perfect_opt_act_3: _, "~α%ε~ηκοιτην", _;
    perfect_imp_act_2: "~α%ε~ηκε", _, "~α%ε~ηκετε";
    perfect_imp_act_3: _, _, "~α%ε~ηκοντων";
    perfect_imp_mid_2: _, _, "~α%ε~ησθε";
    perfect_imp_mid_3: "~α%ε~ησθω", _, "~α%ε~ησθων";
    perfect_imp_pass_2: "~α%ε~ησο", _, "~α%ε~ησθε";
    perfect_imp_pass_3: _, "~α%ε~ησθων", "~α%ε~ησθων";
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
    // Λιστραω: 3 words (μεριμναω, κῠβισταω, λιστραω). Like Πῠνθανομαι, except in 158 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("λιστραω", 3, "~ι~αω", like Πῠνθανομαι;
    stem aorist_ind: "~ῐ~ησ- like ἐχω";
    ind_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    ind_act_2: "~ῐ~ᾳς", _, _;
    ind_act_3: "~ῐ~ᾳ", "~ῐ~ᾱτον", "~ῐ~ωσῐ|~ῐ~ωσῐν";
    ind_mid_1: "~ῐ~ωμαι", _, "~ῐ~ωμεθᾰ";
    ind_mid_2: "~ῐ~ᾳ", "~ῐ~ᾱσθον", _;
    ind_pass_2: derived, _, "~ῐ~ᾱσθε";
    ind_pass_3: "~ῐ~ᾱται", _, "~ῐ~ωνται";
    subj_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    subj_act_2: "~ῐ~ᾳς", _, _;
    subj_act_3: "~ῐ~ᾳ", "~ῐ~ᾱτον", "~ῐ~ωσῐ|~ῐ~ωσῐν";
    subj_mid_1: derived, _, "~ῐ~ωμεθᾰ";
    subj_mid_2: derived, "~ῐ~ᾱσθον", _;
    subj_mid_3: "~ῐ~ᾱται", _, "~ῐ~ωνται";
    subj_pass_1: "~ῐ~ωμαι", _, _;
    subj_pass_2: derived, _, "~ῐ~ᾱσθε";
    opt_act_1: "~ῐ~ῳην|~ῐ~ῳμῐ", _, "~ῐ~ῳμεν|~ῐ~ῳημεν";
    opt_act_2: "~ῐ~ῳης|~ῐ~ῳς", "~ῐ~ῳτον|~ῐ~ῳητον", "~ῐ~ῳτε|~ῐ~ῳητε";
    opt_act_3: "~ῐ~ῳη|~ῐ~ῳ", "~ῐ~ῳτην|~ῐ~ῳητην", "~ῐ~ῳεν|~ῐ~ῳησᾰν";
    opt_mid_1: "~ῐ~ῳμην", _, "~ῐ~ῳμεθᾰ";
    opt_mid_2: "~ῐ~ῳο", "~ῐ~ῳσθον", _;
    opt_mid_3: "~ῐ~ῳτο", "~ῐ~ῳσθην", "~ῐ~ῳντο";
    opt_pass_2: _, _, "~ῐ~ῳσθε";
    imp_act_2: "~ῐ~ᾱ", "~ῐ~ᾱτον", "~ῐ~ᾱτε";
    imp_mid_2: "~ῐ~ω", _, "~ῐ~ᾱσθε";
    imp_mid_3: "~ῐ~ᾱσθω", _, "~ῐ~ᾱσθων";
    imp_pass_2: _, _, "~ῐ~ᾱσθε";
    imp_pass_3: _, "~ῐ~ᾱσθων", "~ῐ~ᾱσθων";
    imperfect_act_1: "ἐ~ῐ~ων", _, "ἐ~ῐ~ωμεν";
    imperfect_act_2: "ἐ~ῐ~ᾱς", "ἐ~ῐ~ᾱτον", "ἐ~ῐ~ᾱτε";
    imperfect_act_3: "ἐ~ῐ~ᾱ", "ἐ~ῐ~ᾱτην", "ἐ~ῐ~ων";
    imperfect_mid_1: "ἐ~ῐ~ωμην", _, "ἐ~ῐ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ῐ~ω", derived, _;
    imperfect_mid_3: "ἐ~ῐ~ᾱτο", derived, derived;
    imperfect_pass_2: _, "ἐ~ῐ~ᾱσθον", "ἐ~ῐ~ᾱσθε";
    imperfect_pass_3: _, "ἐ~ῐ~ᾱσθην", "ἐ~ῐ~ωντο";
    future_ind_act_1: "~ῐ~ησω", _, "~ῐ~ησομεν";
    future_ind_act_2: "~ῐ~ησεις", "~ῐ~ησετον", "~ῐ~ησετε";
    future_ind_act_3: _, _, "~ῐ~ησουσῐ";
    future_ind_mid_1: "~ῐ~ησομαι", _, "~ῐ~ησομεθᾰ";
    future_ind_mid_2: "~ῐ~ησῃ", "~ῐ~ησεσθον", _;
    future_ind_mid_3: "~ῐ~ησεται", _, _;
    future_opt_act_1: "~ῐ~ησοιμῐ", _, "~ῐ~ησοιμεν";
    future_opt_act_2: _, "~ῐ~ησοιτον", "~ῐ~ησοιτε";
    future_opt_act_3: "~ῐ~ησοι", "~ῐ~ησοιτην", "~ῐ~ησοιεν";
    future_opt_mid_1: "~ῐ~ησοιμην", _, "~ῐ~ησοιμεθᾰ";
    future_opt_mid_2: "~ῐ~ησοιο", _, _;
    future_opt_mid_3: _, "~ῐ~ησοισθην", _;
    aorist_subj_act_1: "~ῐ~ησω", _, _;
    aorist_subj_act_2: "~ῐ~ησῃς", "~ῐ~ησητον", _;
    aorist_subj_mid_1: "~ῐ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ῐ~ησῃ", "~ῐ~ησησθον", _;
    aorist_opt_act_1: "~ῐ~ησαιμῐ", _, "~ῐ~ησαιμεν";
    aorist_opt_act_2: "~ῐ~ησειᾰς", _, _;
    aorist_opt_act_3: "~ῐ~ησειε", "~ῐ~ησαιτην", "~ῐ~ησειᾰν";
    aorist_opt_mid_1: "~ῐ~ησαιμην", _, _;
    aorist_opt_mid_2: "~ῐ~ησαιο", _, _;
    aorist_imp_act_2: "~ῐ~ησον", _, "~ῐ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῐ~ησᾰντων";
    aorist_imp_mid_2: "~ῐ~ησαι", "~ῐ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῐ~ησᾰσθων";
    perfect_ind_act_1: "%ε~ῐ~ηκᾰ", _, "%ε~ῐ~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~ῐ~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~ῐ~ηκᾱσῐ";
    perfect_ind_mid_1: "%ε~ῐ~ημαι", _, "%ε~ῐ~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ῐ~ησαι", "%ε~ῐ~ησθον", _;
    perfect_ind_mid_3: "%ε~ῐ~ηται", _, "%ε~ῐ~ηνται";
    perfect_ind_pass_2: _, _, "%ε~ῐ~ησθε";
    perfect_subj_act_1: "%ε~ῐ~ηκω", _, _;
    perfect_subj_act_2: "%ε~ῐ~ηκῃς", "%ε~ῐ~ηκητον", "%ε~ῐ~ηκητε";
    perfect_subj_act_3: "%ε~ῐ~ηκῃ", _, "%ε~ῐ~ηκωσῐ";
    perfect_opt_act_1: "%ε~ῐ~ηκοιμῐ", _, "%ε~ῐ~ηκοιμεν";
    perfect_opt_act_2: "%ε~ῐ~ηκοις", _, _;
    perfect_opt_act_3: _, "%ε~ῐ~ηκοιτην", _;
    perfect_imp_act_2: "%ε~ῐ~ηκε", _, "%ε~ῐ~ηκετε";
    perfect_imp_act_3: _, _, "%ε~ῐ~ηκοντων";
    perfect_imp_mid_2: _, _, "%ε~ῐ~ησθε";
    perfect_imp_mid_3: "%ε~ῐ~ησθω", _, "%ε~ῐ~ησθων";
    perfect_imp_pass_2: "%ε~ῐ~ησο", _, "%ε~ῐ~ησθε";
    perfect_imp_pass_3: _, "%ε~ῐ~ησθων", "%ε~ῐ~ησθων";
    infinitive_pres: "~ῐ~ᾱν", _, "~ῐ~ᾱσθαι";
    infinitive_fut: "~ῐ~ησειν", _, _;
    infinitive_aor: "~ῐ~ησαι", _, _;
    infinitive_perf: _, "%ε~ῐ~ησθαι", _;
    participle_pres: "~ῐ~ων", _, "~ῐ~ωμενος";
    participle_aor: "~ῐ~ησᾱς", "~ῐ~ησᾰμενος", _;
    participle_perf: "%ε~ῐ~ηκως", _, "%ε~ῐ~ημενος";
    ),
    // Φλῡω: 3 words (πτῡω, μῡω, φλῡω). Like Λλῡω, except in 117 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 pl, ….
    conjugation!("φλῡω", 3, "~ῡω", like Λλῡω;
    stem aorist_ind: "~ῠσ- like ποιεω";
    stem pluperfect_act: "~εμῠκειν- like κᾰλεω";
    future_ind_act_1: "~ῠσω", _, "~ῠσομεν";
    future_ind_act_2: "~ῠσεις", "~ῠσετον", "~ῠσετε";
    future_ind_act_3: _, _, "~ῠσουσῐ|~ῠσουσῐν";
    future_ind_mid_1: "~ῠσομαι", _, "~ῠσομεθᾰ";
    future_ind_mid_2: "~ῠσῃ|~ῠσει", "~ῠσεσθον", _;
    future_ind_mid_3: "~ῠσεται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~ῠσοιμῐ", _, "~ῠσοιμεν";
    future_opt_act_2: _, "~ῠσοιτον", "~ῠσοιτε";
    future_opt_act_3: "~ῠσοι", "~ῠσοιτην", "~ῠσοιεν";
    future_opt_mid_1: "~ῠσοιμην", _, "~ῠσοιμεθᾰ";
    future_opt_mid_2: "~ῠσοιο", _, _;
    future_opt_mid_3: _, "~ῠσοισθην", _;
    aorist_subj_act_1: "~ῠσω", _, _;
    aorist_subj_act_2: "~ῠσῃς", "~ῠσητον", _;
    aorist_subj_act_3: _, _, "~ῠσωσῐ|~ῠσωσῐν";
    aorist_subj_mid_1: "~ῠσωμαι", _, _;
    aorist_subj_mid_2: "~ῠσῃ", "~ῠσησθον", _;
    aorist_subj_pass_3: _, _, derived;
    aorist_opt_act_1: "~ῠσαιμῐ", _, "~ῠσαιμεν";
    aorist_opt_act_2: "~ῠσειᾰς|~ῠσαις", _, _;
    aorist_opt_act_3: "~ῠσειε|~ῠσειεν|~ῠσαι", "~ῠσαιτην", "~ῠσειᾰν|~ῠσαιεν";
    aorist_opt_mid_1: "~ῠσαιμην", _, _;
    aorist_opt_mid_2: "~ῠσαιο", _, _;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: "~ῠσον", _, "~ῠσᾰτε";
    aorist_imp_act_3: _, _, "~ῠσᾰντων";
    aorist_imp_mid_2: "~ῠσαι", "~ῠσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῠσᾰσθων";
    perfect_ind_act_1: "~εμῡκᾰ", _, "~εμῡκᾰμεν";
    perfect_ind_act_2: _, "~εμῡκᾰτον", _;
    perfect_ind_act_3: derived, _, "~εμῡκᾱσῐ";
    perfect_ind_mid_1: "~εμῡμαι", _, "~εμῡμεθᾰ";
    perfect_ind_mid_2: "~εμῡσαι", "~εμῡσθον", _;
    perfect_ind_mid_3: "~εμῡται", _, "~εμῡνται";
    perfect_ind_pass_2: _, _, "~εμῡσθε";
    perfect_subj_act_1: "~εμῡκω", _, _;
    perfect_subj_act_2: "~εμῡκῃς", "~εμῡκητον", "~εμῡκητε";
    perfect_subj_act_3: "~εμῡκῃ", _, "~εμῡκωσῐ";
    perfect_opt_act_1: "~εμῡκοιμῐ", _, "~εμῡκοιμεν";
    perfect_opt_act_2: "~εμῡκοις", _, _;
    perfect_opt_act_3: derived, "~εμῡκοιτην", _;
    perfect_imp_act_2: "~εμῡκε", _, "~εμῡκετε";
    perfect_imp_act_3: _, _, "~εμῡκοντων";
    perfect_imp_mid_2: _, _, "~εμῡσθε";
    perfect_imp_mid_3: "~εμῡσθω", _, "~εμῡσθων";
    perfect_imp_pass_2: "~εμῡσο", _, "~εμῡσθε";
    perfect_imp_pass_3: _, "~εμῡσθων", "~εμῡσθων";
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
    // Ἐπευχομαι: 3 words (προσευχομαι, ἀπευχομαι, ἐπευχομαι). Like Ἐρχομαι, except in 83 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ἐπευχομαι", 3, "~ευχομαι", like Ἐρχομαι;
    stem aorist_ind: "~ηυξ- like μηκᾰομαι";
    ind_act_1: "~ευχομαι", _, _;
    ind_mid_1: "~ευχομαι", _, "~ευχομεθᾰ";
    ind_mid_2: "~ευχῃ", "~ευχεσθον", _;
    ind_pass_2: _, _, "~ευχεσθε";
    ind_pass_3: "~ευχεται", _, "~ευχονται";
    subj_mid_1: "~ευχωμαι", _, "~ευχωμεθᾰ";
    subj_mid_2: "~ευχῃ", "~ευχησθον", _;
    subj_mid_3: "~ευχηται", _, "~ευχωνται";
    subj_pass_2: "~ευχῃ", _, "~ευχησθε";
    opt_mid_1: "~ευχοιμην", _, "~ευχοιμεθᾰ";
    opt_mid_2: "~ευχοιο", "~ευχοισθον", _;
    opt_mid_3: "~ευχοιτο", "~ευχοισθην", "~ευχοιντο";
    opt_pass_2: _, _, "~ευχοισθε";
    imp_mid_2: "~ευχου", _, "~ευχεσθε";
    imp_mid_3: "~ευχεσθω", _, "~ευχεσθων";
    imp_pass_2: _, _, "~ευχεσθε";
    imp_pass_3: _, "~ευχεσθων", "~ευχεσθων";
    imperfect_mid_1: "~ηυχομην", _, "~ηυχομεθᾰ";
    imperfect_mid_2: "~ηυχου", "~ηυχεσθον", _;
    imperfect_mid_3: "~ηυχετο", "~ηυχεσθην", "~ηυχοντο";
    imperfect_pass_2: _, _, "~ηυχεσθε";
    future_ind_mid_1: "~ευξομαι", _, "~ευξομεθᾰ";
    future_ind_mid_2: "~ευξῃ", "~ευξεσθον", _;
    future_ind_mid_3: "~ευξεται", _, _;
    future_opt_mid_1: "~ευξοιμην", _, "~ευξοιμεθᾰ";
    future_opt_mid_2: "~ευξοιο", _, _;
    future_opt_mid_3: _, "~ευξοισθην", _;
    aorist_subj_mid_1: "~ευξωμαι", _, _;
    aorist_subj_mid_2: "~ευξῃ", "~ευξησθον", _;
    aorist_opt_mid_1: "~ευξαιμην", _, _;
    aorist_opt_mid_2: "~ευξαιο", _, _;
    aorist_imp_mid_2: "~ευξαι", "~ευξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ευξᾰσθων";
    perfect_ind_mid_1: "~ηῡγμαι", _, "~ηυγμεθᾰ";
    perfect_ind_mid_2: "~ηῡξαι", "~ηῡχθον", _;
    perfect_ind_mid_3: "~ηῡκται", _, "~ηυχᾰται";
    perfect_ind_pass_2: _, _, "~ηῡχθε";
    perfect_imp_mid_2: _, _, "~ηῡχθε";
    perfect_imp_mid_3: "~ηυχθω", _, "~ηυχθων";
    perfect_imp_pass_2: "~ηῡξο", _, "~ηῡχθε";
    perfect_imp_pass_3: _, "~ηυχθων", "~ηυχθων";
    pluperfect_mid_1: "~ηυγμην", _, _;
    pluperfect_mid_2: "~ηῡξο", "~ηῡχθον", _;
    pluperfect_mid_3: "~ηῡκτο", "~ηυχθην", "~ηυχᾰτο";
    pluperfect_pass_1: _, _, "~ηυγμεθᾰ";
    pluperfect_pass_2: _, _, "~ηῡχθε";
    infinitive_pres: _, _, "~ευχεσθαι";
    infinitive_perf: _, "~ηῡχθαι", _;
    participle_pres: _, _, "~ευχομενος";
    participle_aor: _, "~ευξᾰμενος", _;
    participle_perf: _, _, "~ηυγμενος";
    ),
    // Ἐφελκω: 3 words (ἀφελκω, κᾰθελκω, ἐφελκω). Like Ξαινω, except in 153 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐφελκω", 3, "~~κω", like Ξαινω;
    stem future_pass: "~~κυσθησ- like ἐχω";
    stem aorist_ind: "~ι~κυσ- like εὐλογεω";
    stem aorist_ind_pass: "~ι~κυσθην- like ἐλπω";
    stem aorist_pass: "~ι~κυσθ- like ἐχω";
    ind_act_1: "~~κω", _, "~~κομεν";
    ind_act_2: "~~κεις", _, _;
    ind_act_3: "~~κει", "~~κετον", "~~κουσῐ|~~κουσῐν";
    ind_mid_1: "~~κομαι", _, "~~κομεθᾰ";
    ind_mid_2: "~~κῃ|~~κει", "~~κεσθον", _;
    ind_pass_2: _, _, "~~κεσθε";
    ind_pass_3: "~~κεται", _, "~~κονται";
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
    opt_mid_2: "~~κοιο", "~~κοισθον", _;
    opt_mid_3: "~~κοιτο", "~~κοισθην", "~~κοιντο";
    opt_pass_2: _, _, "~~κοισθε";
    imp_act_2: "~~κε", "~~κετον", "~~κετε";
    imp_mid_2: "~~κου", _, "~~κεσθε";
    imp_mid_3: "~~κεσθω", _, "~~κεσθων";
    imp_pass_2: _, _, "~~κεσθε";
    imp_pass_3: _, "~~κεσθων", "~~κεσθων";
    imperfect_act_1: "~ῑ~κον", _, "~ι~κομεν";
    imperfect_act_2: "~ῑ~κες", "~ι~κετον", "~ι~κετε";
    imperfect_act_3: "~ῑ~κε", "~ι~κετην", "~ῑ~κον";
    imperfect_mid_1: "~ι~κομην", _, "~ι~κομεθᾰ";
    imperfect_mid_2: "~ι~κου", _, _;
    imperfect_mid_3: "~ι~κετο", _, _;
    imperfect_pass_2: _, "~ι~κεσθον", "~ι~κεσθε";
    imperfect_pass_3: _, "~ι~κεσθην", "~ι~κοντο";
    future_ind_act_1: "~~ξω", _, "~~ξομεν";
    future_ind_act_2: "~~ξεις", "~~ξετον", "~~ξετε";
    future_ind_act_3: _, _, "~~ξουσῐ|~~ξουσῐν";
    future_ind_mid_1: "~~ξομαι", _, "~~ξομεθᾰ";
    future_ind_mid_2: "~~ξῃ", "~~ξεσθον", _;
    future_ind_mid_3: "~~ξεται", _, _;
    future_opt_act_1: "~~ξοιμῐ", _, "~~ξοιμεν";
    future_opt_act_2: _, "~~ξοιτον", "~~ξοιτε";
    future_opt_act_3: "~~ξοι", "~~ξοιτην", "~~ξοιεν";
    future_opt_mid_1: "~~ξοιμην", _, "~~ξοιμεθᾰ";
    future_opt_mid_2: "~~ξοιο", _, _;
    future_opt_mid_3: _, "~~ξοισθην", _;
    aorist_subj_act_1: "~~κυσω", _, _;
    aorist_subj_act_2: "~~κυσῃς", "~~κυσητον", _;
    aorist_subj_act_3: _, _, "~~κυσωσῐ|~~κυσωσῐν";
    aorist_subj_mid_1: "~~κυσωμαι", _, _;
    aorist_subj_mid_2: "~~κυσῃ", "~~κυσησθον", _;
    aorist_opt_act_1: "~~κυσαιμῐ", _, "~~κυσαιμεν";
    aorist_opt_act_2: "~~κυσειᾰς|~~κυσαις", _, _;
    aorist_opt_act_3: "~~κυσειε|~~κυσειεν|~~κυσαι", "~~κυσαιτην", "~~κυσειᾰν|~~κυσαιεν";
    aorist_opt_mid_1: "~~κυσαιμην", _, _;
    aorist_opt_mid_2: "~~κυσαιο", _, _;
    aorist_imp_act_2: "~~κυσον", _, "~~κυσᾰτε";
    aorist_imp_act_3: _, _, "~~κυσᾰντων";
    aorist_imp_mid_2: "~~κυσαι", "~~κυσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~κυσᾰσθων";
    perfect_ind_act_1: "~ι~κῠκᾰ", _, "~ι~κῠκᾰμεν";
    perfect_ind_act_2: _, "~ι~κῠκᾰτον", _;
    perfect_ind_act_3: "~ι~κῠκε|~ι~κῠκεν", _, "~ι~κῠκᾱσῐ|~ι~κῠκᾱσῐν";
    perfect_ind_mid_1: "~ι~κῠμαι", _, "~ι~κῠμεθᾰ";
    perfect_ind_mid_2: "~ι~κῠσαι", "~ι~κῠσθον", _;
    perfect_ind_mid_3: "~ι~κῠται", _, "~ι~κῠνται";
    perfect_ind_pass_2: _, _, "~ι~κῠσθε";
    perfect_subj_act_1: "~ι~κῠκω", _, _;
    perfect_subj_act_2: "~ι~κῠκῃς", "~ι~κῠκητον", "~ι~κῠκητε";
    perfect_subj_act_3: "~ι~κῠκῃ", _, "~ι~κῠκωσῐ|~ι~κῠκωσῐν";
    perfect_opt_act_1: "~ι~κῠκοιμῐ|~ι~κῠκοιην", _, "~ι~κῠκοιμεν";
    perfect_opt_act_2: "~ι~κῠκοις|~ι~κῠκοιης", _, _;
    perfect_opt_act_3: "~ι~κῠκοι|~ι~κῠκοιη", "~ι~κῠκοιτην", _;
    perfect_imp_act_2: "~ι~κῠκε", _, "~ι~κῠκετε";
    perfect_imp_act_3: _, _, "~ι~κῠκοντων";
    perfect_imp_mid_2: _, _, "~ι~κῠσθε";
    perfect_imp_mid_3: "~ι~κῠσθω", _, "~ι~κῠσθων";
    perfect_imp_pass_2: "~ι~κῠσο", _, "~ι~κῠσθε";
    perfect_imp_pass_3: _, "~ι~κῠσθων", "~ι~κῠσθων";
    infinitive_pres: "~~κειν", _, "~~κεσθαι";
    infinitive_fut: "~~ξειν", _, _;
    infinitive_aor: "~~κῡσαι", _, _;
    infinitive_perf: _, "~ι~κῠσθαι", _;
    participle_pres: "~~κων", _, "~~κομενος";
    participle_aor: "~~κυσᾱς", "~~κυσᾰμενος", _;
    participle_perf: "~ι~κῠκως", _, "~ι~κῠμενος";
    ),
    // Τρῑβω: 3 words (τρῑβω, θλῑβω, φλῑβω). Like Ἀμειβω, except in 136 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("τρῑβω", 3, "~ῑβω", like Ἀμειβω;
    stem future_pass: "~ῐβησ- like ποιεω";
    stem aorist_ind_pass: "~ιφθην- like ἐχω";
    stem aorist_pass: "~ιφθ- like ἐχω";
    stem pluperfect_act: "~ῐφ- like χρημᾰτῐζω";
    ind_act_1: "~ῑβω", _, "~ῑβομεν";
    ind_act_2: "~ῑβεις", _, _;
    ind_act_3: "~ῑβει", "~ῑβετον", "~ῑβουσῐ|~ῑβουσῐν";
    ind_mid_1: "~ῑβομαι", _, "~ῑβομεθᾰ";
    ind_mid_2: "~ῑβῃ|~ῑβει", "~ῑβεσθον", _;
    ind_pass_2: _, _, "~ῑβεσθε";
    ind_pass_3: "~ῑβεται", _, "~ῑβονται";
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
    opt_mid_2: "~ῑβοιο", "~ῑβοισθον", _;
    opt_mid_3: "~ῑβοιτο", "~ῑβοισθην", "~ῑβοιντο";
    opt_pass_2: _, _, "~ῑβοισθε";
    imp_act_2: _, "~ῑβετον", "~ῑβετε";
    imp_mid_2: "~ῑβου", _, "~ῑβεσθε";
    imp_mid_3: "~ῑβεσθω", _, "~ῑβεσθων";
    imp_pass_2: _, _, "~ῑβεσθε";
    imp_pass_3: _, "~ῑβεσθων", "~ῑβεσθων";
    imperfect_act_1: "ἐ~ῑβον", _, "ἐ~ῑβομεν";
    imperfect_act_2: "ἐ~ῑβες", "ἐ~ῑβετον", "ἐ~ῑβετε";
    imperfect_act_3: "ἐ~ῑβε|ἐ~ῑβεν", "ἐ~ῑβετην", "ἐ~ῑβον";
    imperfect_mid_1: "ἐ~ῑβομην", _, "ἐ~ῑβομεθᾰ";
    imperfect_mid_2: "ἐ~ῑβου", _, _;
    imperfect_mid_3: "ἐ~ῑβετο", _, _;
    imperfect_pass_2: _, "ἐ~ῑβεσθον", "ἐ~ῑβεσθε";
    imperfect_pass_3: _, "ἐ~ῑβεσθην", "ἐ~ῑβοντο";
    future_ind_act_1: "~ιψω", _, "~ιψομεν";
    future_ind_act_2: "~ιψεις", "~ιψετον", "~ιψετε";
    future_ind_act_3: _, _, "~ιψουσῐ|~ιψουσῐν";
    future_ind_mid_1: "~ιψομαι", _, "~ιψομεθᾰ";
    future_ind_mid_2: "~ιψῃ|~ιψει", "~ιψεσθον", _;
    future_ind_mid_3: "~ιψεται", _, _;
    future_opt_act_1: "~ιψοιμῐ", _, "~ιψοιμεν";
    future_opt_act_2: _, "~ιψοιτον", "~ιψοιτε";
    future_opt_act_3: "~ιψοι", "~ιψοιτην", "~ιψοιεν";
    future_opt_mid_1: "~ιψοιμην", _, "~ιψοιμεθᾰ";
    future_opt_mid_2: "~ιψοιο", _, _;
    future_opt_mid_3: _, "~ιψοισθην", _;
    perfect_ind_act_1: "%ε~ῐφᾰ", _, "%ε~ῐφᾰμεν";
    perfect_ind_act_2: _, "%ε~ῐφᾰτον", _;
    perfect_ind_act_3: "%ε~ῐφε|%ε~ῐφεν", _, "%ε~ῐφᾱσῐ|%ε~ῐφᾱσῐν";
    perfect_ind_mid_1: "%ε~ῑμμαι", _, "%ε~ῑμμεθᾰ";
    perfect_ind_mid_2: "%ε~ῑψαι", "%ε~ῑφθον", _;
    perfect_ind_mid_3: "%ε~ῑπται", _, "%ε~ῑφᾰται";
    perfect_ind_pass_2: _, _, "%ε~ῑφθε";
    perfect_subj_act_1: "%ε~ῐφω", _, _;
    perfect_subj_act_2: "%ε~ῐφῃς", "%ε~ῐφητον", "%ε~ῐφητε";
    perfect_subj_act_3: "%ε~ῐφῃ", _, "%ε~ῐφωσῐ|%ε~ῐφωσῐν";
    perfect_opt_act_1: "%ε~ῐφοιμῐ|%ε~ῐφοιην", _, "%ε~ῐφοιμεν";
    perfect_opt_act_2: "%ε~ῐφοις|%ε~ῐφοιης", _, _;
    perfect_opt_act_3: "%ε~ῐφοι|%ε~ῐφοιη", "%ε~ῐφοιτην", _;
    perfect_imp_act_2: "%ε~ῐφε", _, "%ε~ῐφετε";
    perfect_imp_act_3: _, _, "%ε~ῐφοντων";
    perfect_imp_mid_2: _, _, "%ε~ῑφθε";
    perfect_imp_mid_3: "%ε~ῑφθω", _, "%ε~ῑφθων";
    perfect_imp_pass_2: "%ε~ῑψο", _, "%ε~ῑφθε";
    perfect_imp_pass_3: _, "%ε~ῑφθων", "%ε~ῑφθων";
    pluperfect_mid_1: "ἐ%ε~ῐμμην", _, _;
    pluperfect_mid_2: "ἐ%ε~ῐψο", "ἐ%ε~ῐφθον", _;
    pluperfect_mid_3: "ἐ%ε~ῐπτο", _, "ἐ%ε~ῐφᾰτο";
    pluperfect_pass_1: _, _, "ἐ%ε~ῐμμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ῐφθε";
    pluperfect_pass_3: _, "ἐ%ε~ῐφθην", _;
    infinitive_pres: "~ῑβειν", _, "~ῑβεσθαι";
    infinitive_fut: "~ιψειν", _, _;
    infinitive_perf: _, "%ε~ῑφθαι", _;
    participle_pres: "~ῑβων", _, "~ῑβομενος";
    participle_perf: "%ε~ῐφως", _, "%ε~ῑμμενος";
    ),
    // Ἀντεχω: 3 words (μετεχω, κᾰτεχω, ἀντεχω). Like Μῑγνῡμῐ, except in 152 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀντεχω", 3, "~τεχω", like Μῑγνῡμῐ;
    stem aorist_ind: "~τεσχ- like πᾰρεχω";
    ind_act_1: "~τεχω", _, "~τεχομεν";
    ind_act_2: "~τεχεις", _, _;
    ind_act_3: "~τεχει", "~τεχετον", "~τεχουσῐ|~τεχουσῐν";
    ind_mid_1: "~τεχομαι", _, "~τεχομεθᾰ";
    ind_mid_2: "~τεχῃ|~τεχει", "~τεχεσθον", _;
    ind_pass_2: _, _, "~τεχεσθε";
    ind_pass_3: "~τεχεται", _, "~τεχονται";
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
    opt_mid_2: "~τεχοιο", "~τεχοισθον", _;
    opt_mid_3: "~τεχοιτο", "~τεχοισθην", "~τεχοιντο";
    opt_pass_2: _, _, "~τεχοισθε";
    imp_act_2: "~τεχε", "~τεχετον", "~τεχετε";
    imp_mid_2: "~τεχου", _, "~τεχεσθε";
    imp_mid_3: "~τεχεσθω", _, "~τεχεσθων";
    imp_pass_2: _, _, "~τεχεσθε";
    imp_pass_3: _, "~τεχεσθων", "~τεχεσθων";
    imperfect_act_1: "~τεῑχον", _, "~τειχομεν";
    imperfect_act_2: "~τεῑχες", "~τειχετον", "~τειχετε";
    imperfect_act_3: "~τεῑχε|~τεῑχεν", "~τειχετην", "~τεῑχον";
    imperfect_mid_1: "~τειχομην", _, "~τειχομεθᾰ";
    imperfect_mid_2: "~τειχου", _, _;
    imperfect_mid_3: "~τειχετο", _, _;
    imperfect_pass_2: _, "~τειχεσθον", "~τειχεσθε";
    imperfect_pass_3: _, "~τειχεσθην", "~τειχοντο";
    future_ind_act_1: "~θεξω", _, "~θεξομεν";
    future_ind_act_2: "~θεξεις", "~θεξετον", "~θεξετε";
    future_ind_act_3: _, _, "~θεξουσῐ|~θεξουσῐν";
    future_ind_mid_1: "~θεξομαι", _, "~θεξομεθᾰ";
    future_ind_mid_2: "~θεξῃ|~θεξει", "~θεξεσθον", _;
    future_ind_mid_3: "~θεξεται", _, _;
    future_opt_act_1: "~θεξοιμῐ", _, "~θεξοιμεν";
    future_opt_act_2: _, "~θεξοιτον", "~θεξοιτε";
    future_opt_act_3: "~θεξοι", "~θεξοιτην", "~θεξοιεν";
    future_opt_mid_1: "~θεξοιμην", _, "~θεξοιμεθᾰ";
    future_opt_mid_2: "~θεξοιο", _, _;
    future_opt_mid_3: _, "~θεξοισθην", _;
    aorist_subj_act_1: "~τᾰσχω", _, _;
    aorist_subj_act_2: "~τᾰσχῃς", "~τᾰσχητον", _;
    aorist_subj_mid_1: "~τᾰσχωμαι", _, _;
    aorist_subj_mid_2: "~τᾰσχῃ", "~τᾰσχησθον", _;
    aorist_opt_act_1: "~τᾰσχοιμῐ", _, "~τᾰσχοιμεν";
    aorist_opt_act_2: "~τᾰσχοις", _, _;
    aorist_opt_act_3: "~τᾰσχοι", "~τᾰσχοιτην", "~τᾰσχοιεν";
    aorist_opt_mid_1: "~τᾰσχοιμην", _, _;
    aorist_opt_mid_2: "~τᾰσχοιο", _, _;
    aorist_imp_act_2: "~τᾰσχε", _, "~τᾰσχετε";
    aorist_imp_act_3: _, _, "~τᾰσχοντων";
    aorist_imp_mid_2: "~τᾰσχοῡ", "~τᾰσχεσθον", _;
    aorist_imp_mid_3: _, _, "~τᾰσχεσθων";
    perfect_ind_act_1: "~τεσχηκᾰ", _, "~τεσχηκᾰμεν";
    perfect_ind_act_2: _, "~τεσχηκᾰτον", _;
    perfect_ind_act_3: "~τεσχηκε|~τεσχηκεν", _, "~τεσχηκᾱσῐ|~τεσχηκᾱσῐν";
    perfect_ind_mid_1: "~τεσχημαι", _, "~τεσχημεθᾰ";
    perfect_ind_mid_2: "~τεσχησαι", "~τεσχησθον", _;
    perfect_ind_mid_3: "~τεσχηται", _, "~τεσχηνται";
    perfect_ind_pass_2: _, _, "~τεσχησθε";
    perfect_subj_act_1: "~τεσχηκω", _, _;
    perfect_subj_act_2: "~τεσχηκῃς", "~τεσχηκητον", "~τεσχηκητε";
    perfect_subj_act_3: "~τεσχηκῃ", _, "~τεσχηκωσῐ|~τεσχηκωσῐν";
    perfect_opt_act_1: "~τεσχηκοιμῐ|~τεσχηκοιην", _, "~τεσχηκοιμεν";
    perfect_opt_act_2: "~τεσχηκοις|~τεσχηκοιης", _, _;
    perfect_opt_act_3: "~τεσχηκοι|~τεσχηκοιη", "~τεσχηκοιτην", _;
    perfect_imp_act_2: "~τεσχηκε", _, "~τεσχηκετε";
    perfect_imp_act_3: _, _, "~τεσχηκοντων";
    perfect_imp_mid_2: _, _, "~τεσχησθε";
    perfect_imp_mid_3: "~τεσχησθω", _, "~τεσχησθων";
    perfect_imp_pass_2: "~τεσχησο", _, "~τεσχησθε";
    perfect_imp_pass_3: _, "~τεσχησθων", "~τεσχησθων";
    infinitive_pres: "~τεχειν", _, "~τεχεσθαι";
    infinitive_fut: "~θεξειν", _, _;
    infinitive_aor: "~τᾰσχεῑν", _, _;
    infinitive_perf: _, "~τεσχησθαι", _;
    participle_pres: "~τεχων", _, "~τεχομενος";
    participle_aor: "~τᾰσχων", "~τᾰσχομενος", _;
    participle_perf: "~τεσχηκως", _, "~τεσχημενος";
    ),
    // Ποιμαινω: 3 words (ἐπῐφαινω, ποιμαινω, χλῑαινω). Like Ξαινω, except in 134 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ποιμαινω", 3, "~~ινω", like Ξαινω;
    stem aorist_ind: "~~ν- like ἐχω";
    stem aorist_ind_pass: "~~νθην- like ἐχω";
    stem aorist_pass: "~~νθ- like ἐχω";
    ind_act_1: "~~ινω", _, "~~ινομεν";
    ind_act_2: "~~ινεις", _, _;
    ind_act_3: "~~ινει", "~~ινετον", "~~ινουσῐ";
    ind_mid_1: "~~ινομαι", _, "~~ινομεθᾰ";
    ind_mid_2: "~~ινῃ", "~~ινεσθον", _;
    ind_pass_2: _, _, "~~ινεσθε";
    ind_pass_3: "~~ινεται", _, "~~ινονται";
    subj_act_1: "~~ινω", _, "~~ινωμεν";
    subj_act_2: "~~ινῃς", _, _;
    subj_act_3: "~~ινῃ", "~~ινητον", "~~ινωσῐ";
    subj_mid_1: _, _, "~~ινωμεθᾰ";
    subj_mid_2: _, "~~ινησθον", _;
    subj_mid_3: "~~ινηται", _, "~~ινωνται";
    subj_pass_1: "~~ινωμαι", _, _;
    subj_pass_2: _, _, "~~ινησθε";
    opt_act_1: "~~ινοιμῐ", _, "~~ινοιμεν";
    opt_act_2: "~~ινοις", "~~ινοιτον", _;
    opt_act_3: "~~ινοι", "~~ινοιτην", "~~ινοιεν";
    opt_mid_1: "~~ινοιμην", _, "~~ινοιμεθᾰ";
    opt_mid_2: "~~ινοιο", "~~ινοισθον", _;
    opt_mid_3: "~~ινοιτο", "~~ινοισθην", "~~ινοιντο";
    opt_pass_2: _, _, "~~ινοισθε";
    imp_act_2: "~~ινε", "~~ινετον", "~~ινετε";
    imp_mid_2: "~~ινου", _, "~~ινεσθε";
    imp_mid_3: "~~ινεσθω", _, "~~ινεσθων";
    imp_pass_2: _, _, "~~ινεσθε";
    imp_pass_3: _, "~~ινεσθων", "~~ινεσθων";
    imperfect_act_1: "ε~~ινον", _, "ε~~ινομεν";
    imperfect_act_2: "ε~~ινες", "ε~~ινετον", "ε~~ινετε";
    imperfect_act_3: "ε~~ινε", "ε~~ινετην", "ε~~ινον";
    imperfect_mid_1: "ε~~ινομην", _, "ε~~ινομεθᾰ";
    imperfect_mid_2: "ε~~ινου", _, _;
    imperfect_mid_3: "ε~~ινετο", _, _;
    imperfect_pass_2: _, "ε~~ινεσθον", "ε~~ινεσθε";
    imperfect_pass_3: _, "ε~~ινεσθην", "ε~~ινοντο";
    future_ind_act_1: "~~νω", _, "~~νοῡμεν";
    future_ind_act_2: "~~νεῑς", "~~νεῑτον", "~~νεῑτε";
    future_ind_act_3: _, _, "~~νοῡσῐ";
    future_ind_mid_1: "~~νοῡμαι", _, "~~νουμεθᾰ";
    future_ind_mid_2: "~~νεῑ", "~~νεῑσθον", _;
    future_ind_mid_3: "~~νεῑται", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: "~~νοιην", _, "~~νοῑμεν";
    future_opt_act_2: _, "~~νοῑτον", "~~νοῑτε";
    future_opt_act_3: "~~νοιη", "~~νοιτην", "~~νοῑεν";
    future_opt_mid_1: "~~νοιμην", _, "~~νοιμεθᾰ";
    future_opt_mid_2: "~~νοῑο", _, _;
    future_opt_mid_3: _, "~~νοισθην", _;
    aorist_subj_act_1: "~~νω", _, _;
    aorist_subj_act_2: "~~νῃς", "~~νητον", _;
    aorist_subj_mid_1: "~~νωμαι", _, _;
    aorist_subj_mid_2: "~~νῃ", "~~νησθον", _;
    aorist_opt_act_1: "~~ναιμῐ", _, "~~ναιμεν";
    aorist_opt_act_2: "~~νειᾰς", _, _;
    aorist_opt_act_3: "~~νειε", "~~ναιτην", "~~νειᾰν";
    aorist_opt_mid_1: "~~ναιμην", _, _;
    aorist_opt_mid_2: "~~ναιο", _, _;
    aorist_imp_act_2: "~~νον", _, "~~νᾰτε";
    aorist_imp_act_3: _, _, "~~νᾰντων";
    aorist_imp_mid_2: "~~ναι", "~~νᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~νᾰσθων";
    perfect_ind_mid_1: "%ε~~σμαι", _, "%ε~~σμεθᾰ";
    perfect_ind_mid_2: "%ε~~νσαι", "%ε~~νθον", _;
    perfect_ind_mid_3: "%ε~~νται", _, "%ε~~νᾰται";
    perfect_ind_pass_2: _, _, "%ε~~νθε";
    perfect_imp_mid_2: _, _, "%ε~~νθε";
    perfect_imp_mid_3: "%ε~~νθω", _, "%ε~~νθων";
    perfect_imp_pass_2: "%ε~~νσο", _, "%ε~~νθε";
    perfect_imp_pass_3: _, "%ε~~νθων", "%ε~~νθων";
    infinitive_pres: "~~ινειν", _, "~~ινεσθαι";
    infinitive_fut: "~~νεῑν", _, _;
    infinitive_aor: "~ᾱν~ι", _, _;
    infinitive_perf: _, "%ε~ᾱνθ~ι", _;
    participle_pres: "~~ινων", _, "~~ινομενος";
    participle_aor: "~~νᾱς", "~~νᾰμενος", _;
    participle_perf: _, _, "%ε~~σμενος";
    ),
    // Σῠνιστημῐ: 3 words (σῠνιστημῐ, ξῠνιστημῐ, ὀνινημῐ). Like Ξαινω, except in 137 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("σῠνιστημῐ", 3, "~νι~ημῐ", like Ξαινω;
    stem aorist_ind: "~νε~- like σῠνιστημῐ";
    stem aorist_ind_pass: "~νε~ᾰθην- like ἐλπω";
    stem aorist_pass: "~~ᾰθ- like σφᾰλλω";
    stem pluperfect_act: "~νει~ηκ- like εὐλογεω";
    ind_act_1: "~νι~ημῐ", _, "~νι~ᾰμεν";
    ind_act_2: "~νι~ης", _, _;
    ind_act_3: "~νι~ησῐ|~νι~ησῐν", "~νι~ᾰτον", "~νι~ᾱσῐ|~νι~ᾱσῐν";
    ind_mid_1: "~νι~ᾰμαι", _, "~νι~ᾰμεθᾰ";
    ind_mid_2: "~νι~ᾰσαι", "~νι~ᾰσθον", _;
    ind_pass_2: _, _, "~νι~ᾰσθε";
    ind_pass_3: "~νι~ᾰται", _, "~νι~ᾰνται";
    subj_act_1: "~νι~ω", _, "~νι~ωμεν";
    subj_act_2: "~νι~ῃς", _, _;
    subj_act_3: "~νι~ῃ", "~νι~ητον", "~νι~ωσῐ|~νι~ωσῐν";
    subj_mid_1: _, _, "~νι~ωμεθᾰ";
    subj_mid_2: _, "~νι~ησθον", _;
    subj_mid_3: "~νι~ηται", _, "~νι~ωνται";
    subj_pass_1: "~νι~ωμαι", _, _;
    subj_pass_2: _, _, "~νι~ησθε";
    opt_act_1: "~νι~αιην", _, "~νι~αῑμεν|~νι~αιημεν";
    opt_act_2: "~νι~αιης", "~νι~αῑτον|~νι~αιητον", "~νι~αῑτε|~νι~αιητε";
    opt_act_3: "~νι~αιη", "~νι~αιτην|~νι~αιητην", "~νι~αῑεν|~νι~αιησᾰν";
    opt_mid_1: "~νι~αιμην", _, "~νι~αιμεθᾰ";
    opt_mid_2: "~νι~αιο", "~νι~αισθον", _;
    opt_mid_3: "~νι~αιτο", "~νι~αισθην", "~νι~αιντο";
    opt_pass_2: _, _, "~νι~αισθε";
    imp_act_2: "~νι~η", "~νι~ᾰτον", "~νι~ᾰτε";
    imp_mid_2: "~νι~ᾰσο|~νι~ω", _, "~νι~ᾰσθε";
    imp_mid_3: "~νι~ᾰσθω", _, "~νι~ᾰσθων";
    imp_pass_2: _, _, "~νι~ᾰσθε";
    imp_pass_3: _, "~νι~ᾰσθων", "~νι~ᾰσθων";
    imperfect_act_1: "~νῑ~ην", _, "~νῑ~ᾰμεν";
    imperfect_act_2: "~νῑ~ης", "~νῑ~ᾰτον", "~νῑ~ᾰτε";
    imperfect_act_3: "~νῑ~η", "~νῑ~ᾰτην", "~νῑ~ᾰσᾰν";
    imperfect_mid_1: "~νῑ~ᾰμην", _, "~νῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~νῑ~ᾰσο", _, _;
    imperfect_mid_3: "~νῑ~ᾰτο", _, _;
    imperfect_pass_2: _, "~νῑ~ᾰσθον", "~νῑ~ᾰσθε";
    imperfect_pass_3: _, "~νῑ~ᾰσθην", "~νῑ~ᾰντο";
    future_ind_act_1: "~~ησω", _, "~~ησομεν";
    future_ind_act_2: "~~ησεις", "~~ησετον", "~~ησετε";
    future_ind_act_3: _, _, "~~ησουσῐ|~~ησουσῐν";
    future_ind_mid_1: "~~ησομαι", _, "~~ησομεθᾰ";
    future_ind_mid_2: "~~ησῃ|~~ησει", "~~ησεσθον", _;
    future_ind_mid_3: "~~ησεται", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: "~~ησοιμῐ", _, "~~ησοιμεν";
    future_opt_act_2: _, "~~ησοιτον", "~~ησοιτε";
    future_opt_act_3: "~~ησοι", "~~ησοιτην", "~~ησοιεν";
    future_opt_mid_1: "~~ησοιμην", _, "~~ησοιμεθᾰ";
    future_opt_mid_2: "~~ησοιο", _, _;
    future_opt_mid_3: _, "~~ησοισθην", _;
    aorist_subj_act_1: "~~ω", _, _;
    aorist_subj_act_2: "~~ῃς", "~~ητον", _;
    aorist_subj_mid_1: "~~ωμαι", _, _;
    aorist_subj_mid_2: "~~ῃ", "~~ησθον", _;
    aorist_opt_act_1: "~~αιην", _, "~~αῑμεν";
    aorist_opt_act_2: "~~αιης", _, _;
    aorist_opt_act_3: "~~αιη", "~~αιτην", "~~αῑεν";
    aorist_opt_mid_1: "~~αιμην", _, _;
    aorist_opt_mid_2: "~~αῑο", _, _;
    aorist_imp_act_2: "~~ηθῐ", _, "~~ητε";
    aorist_imp_act_3: _, _, "~~ᾰντων";
    aorist_imp_mid_2: "~~ω", "~~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~~ᾰσθων";
    perfect_ind_act_1: "~νε~ηκᾰ", _, "~νε~ηκᾰμεν";
    perfect_ind_act_2: _, "~νε~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "~νε~ηκᾱσῐ";
    perfect_subj_act_1: "~νε~ηκω", _, _;
    perfect_subj_act_2: "~νε~ηκῃς", "~νε~ηκητον", "~νε~ηκητε";
    perfect_subj_act_3: "~νε~ηκῃ", _, "~νε~ηκωσῐ";
    perfect_opt_act_1: "~νε~ηκοιμῐ", _, "~νε~ηκοιμεν";
    perfect_opt_act_2: "~νε~ηκοις", _, _;
    perfect_opt_act_3: _, "~νε~ηκοιτην", _;
    perfect_imp_act_2: "~νε~ηκε", _, "~νε~ηκετε";
    perfect_imp_act_3: _, _, "~νε~ηκοντων";
    infinitive_pres: "~νι~ᾰναι", _, "~νι~ᾰσθαι";
    infinitive_fut: "~~ησειν", _, _;
    infinitive_aor: "~~ηναι", _, _;
    participle_pres: "~νι~ᾱς", _, "~νι~ᾰμενος";
    participle_aor: "~~ᾱς", "~~ᾰμενος", _;
    participle_perf: "~νε~ηκως", _, _;
    ),
    // Ἐφῐστημῐ: 3 words (ἐφῐστημῐ, ἀφῐστημῐ, ὑφῐστημῐ). Like Ξαινω, except in 213 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐφῐστημῐ", 3, "~φῐ~ημῐ", like Ξαινω;
    stem future_pass: "~πο~ησ- like ἐχω";
    stem aorist_ind: "~πε~- like ἐφῐστημῐ";
    stem aorist_ind_pass: "~πε~ᾰθην- like ἐλπω";
    stem aorist_pass: "~πο~ᾰθ- like ποιεω";
    ind_act_1: "~φῐ~ημῐ", _, "~φῐ~ᾰμεν";
    ind_act_2: "~φῐ~ης", _, _;
    ind_act_3: "~φῐ~ησῐ|~φῐ~ησῐν", "~φῐ~ᾰτον", "~φῐ~ᾱσῐ|~φῐ~ᾱσῐν";
    ind_mid_1: "~φῐ~ᾰμαι", _, "~φῐ~ᾰμεθᾰ";
    ind_mid_2: "~φῐ~ᾰσαι", "~φῐ~ᾰσθον", _;
    ind_pass_2: _, _, "~φῐ~ᾰσθε";
    ind_pass_3: "~φῐ~ᾰται", _, "~φῐ~ᾰνται";
    subj_act_1: "~φῐ~ω", _, "~φῐ~ωμεν";
    subj_act_2: "~φῐ~ῃς", _, _;
    subj_act_3: "~φῐ~ῃ", "~φῐ~ητον", "~φῐ~ωσῐ|~φῐ~ωσῐν";
    subj_mid_1: _, _, "~φῐ~ωμεθᾰ";
    subj_mid_2: _, "~φῐ~ησθον", _;
    subj_mid_3: "~φῐ~ηται", _, "~φῐ~ωνται";
    subj_pass_1: "~φῐ~ωμαι", _, _;
    subj_pass_2: _, _, "~φῐ~ησθε";
    opt_act_1: "~φῐ~αιην", _, "~φῐ~αῑμεν|~φῐ~αιημεν";
    opt_act_2: "~φῐ~αιης", "~φῐ~αῑτον|~φῐ~αιητον", "~φῐ~αῑτε|~φῐ~αιητε";
    opt_act_3: "~φῐ~αιη", "~φῐ~αιτην|~φῐ~αιητην", "~φῐ~αῑεν|~φῐ~αιησᾰν";
    opt_mid_1: "~φῐ~αιμην", _, "~φῐ~αιμεθᾰ";
    opt_mid_2: "~φῐ~αιο", "~φῐ~αισθον", _;
    opt_mid_3: "~φῐ~αιτο", "~φῐ~αισθην", "~φῐ~αιντο";
    opt_pass_2: _, _, "~φῐ~αισθε";
    imp_act_2: "~φῐ~η", "~φῐ~ᾰτον", "~φῐ~ᾰτε";
    imp_mid_2: "~φῐ~ᾰσο|~φῐ~ω", _, "~φῐ~ᾰσθε";
    imp_mid_3: "~φῐ~ᾰσθω", _, "~φῐ~ᾰσθων";
    imp_pass_2: _, _, "~φῐ~ᾰσθε";
    imp_pass_3: _, "~φῐ~ᾰσθων", "~φῐ~ᾰσθων";
    imperfect_act_1: "~φῑ~ην", _, "~φῑ~ᾰμεν";
    imperfect_act_2: "~φῑ~ης", "~φῑ~ᾰτον", "~φῑ~ᾰτε";
    imperfect_act_3: "~φῑ~η", "~φῑ~ᾰτην", "~φῑ~ᾰσᾰν";
    imperfect_mid_1: "~φῑ~ᾰμην", _, "~φῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~φῑ~ᾰσο", _, _;
    imperfect_mid_3: "~φῑ~ᾰτο", _, _;
    imperfect_pass_2: _, "~φῑ~ᾰσθον", "~φῑ~ᾰσθε";
    imperfect_pass_3: _, "~φῑ~ᾰσθην", "~φῑ~ᾰντο";
    future_ind_act_1: "~πο~ησω", _, "~πο~ησομεν";
    future_ind_act_2: "~πο~ησεις", "~πο~ησετον", "~πο~ησετε";
    future_ind_act_3: _, _, "~πο~ησουσῐ|~πο~ησουσῐν";
    future_ind_mid_1: "~πο~ησομαι", _, "~πο~ησομεθᾰ";
    future_ind_mid_2: "~πο~ησῃ|~πο~ησει", "~πο~ησεσθον", _;
    future_ind_mid_3: "~πο~ησεται", _, _;
    future_opt_act_1: "~πο~ησοιμῐ", _, "~πο~ησοιμεν";
    future_opt_act_2: _, "~πο~ησοιτον", "~πο~ησοιτε";
    future_opt_act_3: "~πο~ησοι", "~πο~ησοιτην", "~πο~ησοιεν";
    future_opt_mid_1: "~πο~ησοιμην", _, "~πο~ησοιμεθᾰ";
    future_opt_mid_2: "~πο~ησοιο", _, _;
    future_opt_mid_3: _, "~πο~ησοισθην", _;
    aorist_subj_act_1: "~πο~ησω|~πο~ω", _, "~πο~ησωμεν|~πο~ωμεν";
    aorist_subj_act_2: "~πο~ησῃς|~πο~ῃς", "~πο~ησητον|~πο~ητον", "~πο~ησητε|~πο~ητε";
    aorist_subj_act_3: "~πο~ησῃ|~πο~ῃ", _, "~πο~ησωσῐ|~πο~ησωσῐν|~πο~ωσῐ|~πο~ωσῐν";
    aorist_subj_mid_1: "~πο~ησωμαι|~πο~ωμαι", _, "~πο~ησωμεθᾰ|~πο~ωμεθᾰ";
    aorist_subj_mid_2: derived, "~πο~ησησθον|~πο~ησθον", "~πο~ησησθε|~πο~ησθε";
    aorist_subj_mid_3: "~πο~ησηται|~πο~ηται", _, "~πο~ησωνται|~πο~ωνται";
    aorist_opt_act_1: "~πο~ησαιμῐ|~πο~αιην", _, "~πο~ησαιμεν|~πο~αῑμεν|~πο~αιημεν";
    aorist_opt_act_2: "~πο~ησειᾰς|~πο~ησαις|~πο~αιης", "~πο~ησαιτον|~πο~αῑτον|~πο~αιητον", "~πο~ησαιτε|~πο~αῑτε|~πο~αιητε";
    aorist_opt_act_3: "~πο~ησειε|~πο~ησειεν|~πο~ησαι|~πο~αιη", "~πο~ησαιτην|~πο~αιτην|~πο~αιητην", "~πο~ησειᾰν|~πο~ησαιεν|~πο~αῑεν|~πο~αιησᾰν";
    aorist_opt_mid_1: "~πο~ησαιμην|~πο~αιμην", _, "~πο~ησαιμεθᾰ|~πο~αιμεθᾰ";
    aorist_opt_mid_2: "~πο~ησαιο|~πο~αῑο", "~πο~ησαισθον|~πο~αῑσθον", "~πο~ησαισθε|~πο~αῑσθε";
    aorist_opt_mid_3: "~πο~ησαιτο|~πο~αῑτο", "~πο~ησαισθην|~πο~αισθην", "~πο~ησαιντο|~πο~αῑντο";
    aorist_imp_act_2: "~πο~ησον|~πο~ηθῐ", "~πο~ησᾰτον|~πο~ητον", "~πο~ησᾰτε|~πο~ητε";
    aorist_imp_act_3: "~πο~ησᾰτω|~πο~ητω", "~πο~ησᾰτων|~πο~ητων", "~πο~ησᾰντων|~πο~ᾰντων";
    aorist_imp_mid_2: "~πο~ησαι|~πο~ω", "~πο~ησᾰσθον|~πο~ᾰσθον", "~πο~ησᾰσθε|~πο~ᾰσθε";
    aorist_imp_mid_3: "~πο~ησᾰσθω|~πο~ᾰσθω", "~πο~ησᾰσθων|~πο~ᾰσθων", derived;
    perfect_ind_act_1: "~φε~ᾰκᾰ|~φε~ηκᾰ", _, "~φε~ᾰκᾰμεν";
    perfect_ind_act_2: "~φε~ᾰκᾰς|~φε~ηκᾰς", "~φε~ᾰκᾰτον", _;
    perfect_ind_act_3: "~φε~ᾰκε|~φε~ᾰκεν|~φε~ηκε|~φε~ηκεν", _, "~φε~ᾰκᾱσῐ";
    perfect_ind_mid_1: "~φε~ᾰμαι", _, "~φε~ᾰμεθᾰ";
    perfect_ind_mid_2: "~φε~ᾰσαι", "~φε~ᾰσθον", _;
    perfect_ind_mid_3: "~φε~ᾰται", _, "~φε~ᾰνται";
    perfect_ind_pass_2: _, _, "~φε~ᾰσθε";
    perfect_subj_act_1: "~φε~ᾰκω", _, _;
    perfect_subj_act_2: "~φε~ᾰκῃς", "~φε~ᾰκητον", "~φε~ᾰκητε";
    perfect_subj_act_3: "~φε~ᾰκῃ", _, "~φε~ᾰκωσῐ";
    perfect_opt_act_1: "~φε~ᾰκοιμῐ", _, "~φε~ᾰκοιμεν";
    perfect_opt_act_2: "~φε~ᾰκοις", _, _;
    perfect_opt_act_3: _, "~φε~ᾰκοιτην", _;
    perfect_imp_act_2: "~φε~ᾰκε", _, "~φε~ᾰκετε";
    perfect_imp_act_3: _, _, "~φε~ᾰκοντων";
    perfect_imp_mid_2: _, _, "~φε~ᾰσθε";
    perfect_imp_mid_3: "~φε~ᾰσθω", _, "~φε~ᾰσθων";
    perfect_imp_pass_2: "~φε~ᾰσο", _, "~φε~ᾰσθε";
    perfect_imp_pass_3: _, "~φε~ᾰσθων", "~φε~ᾰσθων";
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
    // Δῐαιρω: 3 words (ἐπαιρω, δῐαιρω, ῥῠπαινω). Like Ξαινω, except in 89 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, ….
    conjugation!("δῐαιρω", 3, "~αι~ω", like Ξαινω;
    stem future_pass: "~ᾰ~θησ- like ποιεω";
    stem aorist_ind: "~η~- like εὐλογεω";
    stem aorist_ind_pass: "~η~θην- like ἐλπω";
    stem aorist_pass: "~ᾰ~θ- like ποιεω";
    ind_act_3: _, _, "~αι~ουσῐ|~αι~ουσῐν";
    ind_mid_2: "~αι~ῃ|~αι~ει", _, _;
    subj_act_3: _, _, "~αι~ωσῐ|~αι~ωσῐν";
    imperfect_act_1: "~ῃ~ον", _, "~ῃ~ομεν";
    imperfect_act_2: "~ῃ~ες", "~ῃ~ετον", "~ῃ~ετε";
    imperfect_act_3: "~ῃ~ε|~ῃ~εν", "~ῃ~ετην", "~ῃ~ον";
    imperfect_mid_1: "~ῃ~ομην", _, "~ῃ~ομεθᾰ";
    imperfect_mid_2: "~ῃ~ου", _, _;
    imperfect_mid_3: "~ῃ~ετο", _, _;
    imperfect_pass_2: _, "~ῃ~εσθον", "~ῃ~εσθε";
    imperfect_pass_3: _, "~ῃ~εσθην", "~ῃ~οντο";
    future_ind_act_3: _, _, "~ᾰ~οῡσῐ|~ᾰ~οῡσῐν";
    future_ind_mid_2: "~ᾰ~εῑ|~ᾰ~ῃ", _, _;
    future_opt_act_1: "~ᾰ~οιην|~ᾰ~οῑμῐ", _, "~ᾰ~οῑμεν|~ᾰ~οιημεν";
    future_opt_act_2: "~ᾰ~οιης|~ᾰ~οῑς", "~ᾰ~οῑτον|~ᾰ~οιητον", "~ᾰ~οῑτε|~ᾰ~οιητε";
    future_opt_act_3: "~ᾰ~οιη|~ᾰ~οῑ", "~ᾰ~οιτην|~ᾰ~οιητην", "~ᾰ~οῑεν|~ᾰ~οιησᾰν";
    aorist_subj_act_1: "~ᾰ~ω", _, _;
    aorist_subj_act_2: "~ᾰ~ῃς", "~ᾰ~ητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ῃ", "~ᾰ~ησθον", _;
    aorist_opt_act_1: "~ᾰ~αιμῐ", _, "~ᾰ~αιμεν";
    aorist_opt_act_2: "~ᾰ~ειᾰς|~ᾰ~αις", _, _;
    aorist_opt_act_3: "~ᾰ~ειε|~ᾰ~ειεν|~ᾰ~αι", "~ᾰ~αιτην", "~ᾰ~ειᾰν|~ᾰ~αιεν";
    aorist_opt_mid_1: "~ᾰ~αιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~αιο", _, _;
    aorist_imp_act_2: "~ᾰ~ον", _, "~ᾰ~ᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ᾰντων";
    aorist_imp_mid_2: "~ᾰ~αι", "~ᾰ~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ᾰσθων";
    perfect_ind_act_1: "~η~κᾰ", _, "~η~κᾰμεν";
    perfect_ind_act_2: _, "~η~κᾰτον", _;
    perfect_ind_act_3: "~η~κε|~η~κεν", _, "~η~κᾱσῐ|~η~κᾱσῐν";
    perfect_ind_mid_1: "~η~μαι", _, "~η~μεθᾰ";
    perfect_ind_mid_2: "~η~σαι", "~η~θον", _;
    perfect_ind_mid_3: "~η~ται", _, "~η~ᾰται";
    perfect_ind_pass_2: _, _, "~η~θε";
    perfect_subj_act_1: "~η~κω", _, _;
    perfect_subj_act_2: "~η~κῃς", "~η~κητον", "~η~κητε";
    perfect_subj_act_3: "~η~κῃ", _, "~η~κωσῐ|~η~κωσῐν";
    perfect_opt_act_1: "~η~κοιμῐ|~η~κοιην", _, "~η~κοιμεν";
    perfect_opt_act_2: "~η~κοις|~η~κοιης", _, _;
    perfect_opt_act_3: "~η~κοι|~η~κοιη", "~η~κοιτην", _;
    perfect_imp_act_2: "~η~κε", _, "~η~κετε";
    perfect_imp_act_3: _, _, "~η~κοντων";
    perfect_imp_mid_2: _, _, "~η~θε";
    perfect_imp_mid_3: "~η~θω", _, "~η~θων";
    perfect_imp_pass_2: "~η~σο", _, "~η~θε";
    perfect_imp_pass_3: _, "~η~θων", "~η~θων";
    infinitive_aor: "~ᾰ~αι", _, _;
    infinitive_perf: _, "~η~θαι", _;
    participle_aor: "~ᾰ~ᾱς", "~ᾰ~ᾰμενος", _;
    participle_perf: "~η~κως", _, "~η~μενος";
    ),
    // Ἐγκειμαι: 3 words (συγκειμαι, ξυγκειμαι, ἐγκειμαι). Like Φειδομαι, except in 67 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_3 pl, ….
    conjugation!("ἐγκειμαι", 3, "~γ~ειμαι", like Φειδομαι;
    stem imperfect: "~νε~ε- like ἐγκειμαι";
    ind_act_1: "~γ~ειμαι", _, _;
    ind_mid_1: "~γ~ειμαι", _, "~γ~ειμεθα";
    ind_mid_2: "~γ~εισαι", "~γ~εισθον", _;
    ind_mid_3: _, _, "~γ~εινται|~γ~εᾰται|~γ~εονται|~γ~ειᾰται";
    ind_pass_2: _, _, derived;
    ind_pass_3: derived, _, derived;
    subj_mid_1: "~γ~ωμαι", _, "~γ~ωμεθᾰ";
    subj_mid_2: "~γ~εῃ", "~γ~εησθον", _;
    subj_mid_3: "~γ~εηται|~γ~εῑται", _, "~γ~εωνται|~γ~ειωνται";
    subj_pass_2: derived, _, derived;
    opt_mid_1: "~γ~οιμην", _, "~γ~οιμεθᾰ";
    opt_mid_2: "~γ~οῑο", "~γ~οῑσθον", _;
    opt_mid_3: "~γ~οῑτο", "~γ~οισθην", "~γ~οῑντο";
    opt_pass_2: _, _, derived;
    imp_mid_2: "~γ~εισο", _, "~γ~εισθον";
    imp_mid_3: "~γ~εισθω", _, "~γ~εισθων";
    imp_pass_2: _, _, derived;
    imp_pass_3: _, derived, derived;
    imperfect_pass_2: _, _, derived;
    future_ind_mid_1: "~γ~εισομαι", _, "~γ~εισομεθᾰ";
    future_ind_mid_2: "~γ~εισῃ|~γ~εισει", "~γ~εισεσθον", _;
    future_ind_mid_3: "~γ~εισεται", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_mid_1: "~γ~εισοιμην", _, "~γ~εισοιμεθᾰ";
    future_opt_mid_2: "~γ~εισοιο", _, _;
    future_opt_mid_3: _, "~γ~εισοισθην", _;
    aorist_ind_mid_1: derived, _, _;
    aorist_ind_mid_2: derived, derived, _;
    aorist_subj_mid_1: derived, _, _;
    aorist_subj_mid_2: derived, derived, _;
    aorist_opt_mid_1: derived, _, _;
    aorist_opt_mid_2: derived, _, _;
    aorist_imp_mid_2: derived, derived, _;
    aorist_imp_mid_3: _, _, derived;
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: derived, _, derived;
    infinitive_pres: _, _, derived;
    participle_pres: _, _, derived;
    participle_aor: _, derived, _;
    ),
    // Πᾰρᾰκᾰλεω: 3 words (πᾰρᾰκᾰλεω, ἀνᾰκᾰλεω, κᾰτᾰκᾰλεω). Like Πεμπω, except in 163 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("πᾰρᾰκᾰλεω", 3, "~ᾰκᾰ~εω", like Πεμπω;
    stem future_pass: "~ᾰκεκ~ησ- like ἐχω";
    stem aorist_ind: "~εκᾰ~εσ- like εὐλογεω";
    stem aorist_ind_pass: "~εκ~ηθην- like ἐλπω";
    stem aorist_pass: "~ᾰκ~ηθ- like ποιεω";
    stem pluperfect_act: "~εκεκ~ηκ- like εὐλογεω";
    ind_act_1: "~ᾰκᾰ~ω", _, "~ᾰκᾰ~οῡμεν";
    ind_act_2: "~ᾰκᾰ~εῑς", _, _;
    ind_act_3: "~ᾰκᾰ~εῑ", "~ᾰκᾰ~εῑτον", "~ᾰκᾰ~οῡσῐ|~ᾰκᾰ~οῡσῐν";
    ind_mid_1: "~ᾰκᾰ~οῡμαι", _, "~ᾰκᾰ~ουμεθᾰ";
    ind_mid_2: "~ᾰκᾰ~εῑ|~ᾰκᾰ~ῃ", "~ᾰκᾰ~εῑσθον", _;
    ind_pass_2: _, _, "~ᾰκᾰ~εῑσθε";
    ind_pass_3: "~ᾰκᾰ~εῑται", _, "~ᾰκᾰ~οῡνται";
    subj_act_1: "~ᾰκᾰ~ω", _, "~ᾰκᾰ~ωμεν";
    subj_act_2: "~ᾰκᾰ~ῃς", _, _;
    subj_act_3: "~ᾰκᾰ~ῃ", "~ᾰκᾰ~ητον", "~ᾰκᾰ~ωσῐ|~ᾰκᾰ~ωσῐν";
    subj_mid_1: _, _, "~ᾰκᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ᾰκᾰ~ησθον", _;
    subj_mid_3: "~ᾰκᾰ~ηται", _, "~ᾰκᾰ~ωνται";
    subj_pass_1: "~ᾰκᾰ~ωμαι", _, _;
    subj_pass_2: _, _, "~ᾰκᾰ~ησθε";
    opt_act_1: "~ᾰκᾰ~οιην|~ᾰκᾰ~οῑμῐ", _, "~ᾰκᾰ~οῑμεν|~ᾰκᾰ~οιημεν";
    opt_act_2: "~ᾰκᾰ~οιης|~ᾰκᾰ~οῑς", "~ᾰκᾰ~οῑτον|~ᾰκᾰ~οιητον", "~ᾰκᾰ~οῑτε|~ᾰκᾰ~οιητε";
    opt_act_3: "~ᾰκᾰ~οιη|~ᾰκᾰ~οῑ", "~ᾰκᾰ~οιτην|~ᾰκᾰ~οιητην", "~ᾰκᾰ~οῑεν|~ᾰκᾰ~οιησᾰν";
    opt_mid_1: "~ᾰκᾰ~οιμην", _, "~ᾰκᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰκᾰ~οῑο", "~ᾰκᾰ~οῑσθον", _;
    opt_mid_3: "~ᾰκᾰ~οῑτο", "~ᾰκᾰ~οισθην", "~ᾰκᾰ~οῑντο";
    opt_pass_2: _, _, "~ᾰκᾰ~οῑσθε";
    imp_act_2: "~ᾰκᾰ~ει", "~ᾰκᾰ~εῑτον", "~ᾰκᾰ~εῑτε";
    imp_mid_2: "~ᾰκᾰ~οῡ", _, "~ᾰκᾰ~εῑσθε";
    imp_mid_3: "~ᾰκᾰ~εισθω", _, "~ᾰκᾰ~εισθων";
    imp_pass_2: _, _, "~ᾰκᾰ~εῑσθε";
    imp_pass_3: _, "~ᾰκᾰ~εισθων", "~ᾰκᾰ~εισθων";
    imperfect_act_1: "~εκᾰ~ουν", _, "~εκᾰ~οῡμεν";
    imperfect_act_2: "~εκᾰ~εις", "~εκᾰ~εῑτον", "~εκᾰ~εῑτε";
    imperfect_act_3: "~εκᾰ~ει", "~εκᾰ~ειτην", "~εκᾰ~ουν";
    imperfect_mid_1: "~εκᾰ~ουμην", _, "~εκᾰ~ουμεθᾰ";
    imperfect_mid_2: "~εκᾰ~οῡ", _, _;
    imperfect_mid_3: "~εκᾰ~εῑτο", _, _;
    imperfect_pass_2: _, "~εκᾰ~εῑσθον", "~εκᾰ~εῑσθε";
    imperfect_pass_3: _, "~εκᾰ~εισθην", "~εκᾰ~οῡντο";
    future_ind_act_1: "~ᾰκᾰ~ω", _, "~ᾰκᾰ~οῡμεν";
    future_ind_act_2: "~ᾰκᾰ~εῑς", "~ᾰκᾰ~εῑτον", "~ᾰκᾰ~εῑτε";
    future_ind_act_3: _, _, "~ᾰκᾰ~οῡσῐ|~ᾰκᾰ~οῡσῐν";
    future_ind_mid_1: "~ᾰκᾰ~οῡμαι", _, "~ᾰκᾰ~ουμεθᾰ";
    future_ind_mid_2: "~ᾰκᾰ~εῑ|~ᾰκᾰ~ῃ", "~ᾰκᾰ~εῑσθον", _;
    future_ind_mid_3: "~ᾰκᾰ~εῑται", _, _;
    future_opt_act_1: "~ᾰκᾰ~οιην|~ᾰκᾰ~οῑμῐ", _, "~ᾰκᾰ~οῑμεν|~ᾰκᾰ~οιημεν";
    future_opt_act_2: "~ᾰκᾰ~οιης|~ᾰκᾰ~οῑς", "~ᾰκᾰ~οῑτον|~ᾰκᾰ~οιητον", "~ᾰκᾰ~οῑτε|~ᾰκᾰ~οιητε";
    future_opt_act_3: "~ᾰκᾰ~οιη|~ᾰκᾰ~οῑ", "~ᾰκᾰ~οιτην|~ᾰκᾰ~οιητην", "~ᾰκᾰ~οῑεν|~ᾰκᾰ~οιησᾰν";
    future_opt_mid_1: "~ᾰκᾰ~οιμην", _, "~ᾰκᾰ~οιμεθᾰ";
    future_opt_mid_2: "~ᾰκᾰ~οῑο", _, _;
    future_opt_mid_3: _, "~ᾰκᾰ~οισθην", _;
    aorist_subj_act_1: "~ᾰκᾰ~εσω", _, _;
    aorist_subj_act_2: "~ᾰκᾰ~εσῃς", "~ᾰκᾰ~εσητον", _;
    aorist_subj_act_3: _, _, "~ᾰκᾰ~εσωσῐ|~ᾰκᾰ~εσωσῐν";
    aorist_subj_mid_1: "~ᾰκᾰ~εσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰκᾰ~εσῃ", "~ᾰκᾰ~εσησθον", _;
    aorist_opt_act_1: "~ᾰκᾰ~εσαιμῐ", _, "~ᾰκᾰ~εσαιμεν";
    aorist_opt_act_2: "~ᾰκᾰ~εσειᾰς|~ᾰκᾰ~εσαις", _, _;
    aorist_opt_act_3: "~ᾰκᾰ~εσειε|~ᾰκᾰ~εσειεν|~ᾰκᾰ~εσαι", "~ᾰκᾰ~εσαιτην", "~ᾰκᾰ~εσειᾰν|~ᾰκᾰ~εσαιεν";
    aorist_opt_mid_1: "~ᾰκᾰ~εσαιμην", _, _;
    aorist_opt_mid_2: "~ᾰκᾰ~εσαιο", _, _;
    aorist_imp_act_2: "~ᾰκᾰ~εσον", _, "~ᾰκᾰ~εσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰκᾰ~εσᾰντων";
    aorist_imp_mid_2: "~ᾰκᾰ~εσαι", "~ᾰκᾰ~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰκᾰ~εσᾰσθων";
    perfect_ind_act_1: "~ᾰκεκ~ηκᾰ", _, "~ᾰκεκ~ηκᾰμεν";
    perfect_ind_act_2: _, "~ᾰκεκ~ηκᾰτον", _;
    perfect_ind_act_3: "~ᾰκεκ~ηκε|~ᾰκεκ~ηκεν", _, "~ᾰκεκ~ηκᾱσῐ|~ᾰκεκ~ηκᾱσῐν";
    perfect_ind_mid_1: "~ᾰκεκ~ημαι", _, "~ᾰκεκ~ημεθᾰ";
    perfect_ind_mid_2: "~ᾰκεκ~ησαι", "~ᾰκεκ~ησθον", _;
    perfect_ind_mid_3: "~ᾰκεκ~ηται", _, "~ᾰκεκ~ηνται";
    perfect_ind_pass_2: _, _, "~ᾰκεκ~ησθε";
    perfect_subj_act_1: "~ᾰκεκ~ηκω", _, _;
    perfect_subj_act_2: "~ᾰκεκ~ηκῃς", "~ᾰκεκ~ηκητον", "~ᾰκεκ~ηκητε";
    perfect_subj_act_3: "~ᾰκεκ~ηκῃ", _, "~ᾰκεκ~ηκωσῐ|~ᾰκεκ~ηκωσῐν";
    perfect_opt_act_1: "~ᾰκεκ~ηκοιμῐ|~ᾰκεκ~ηκοιην", _, "~ᾰκεκ~ηκοιμεν";
    perfect_opt_act_2: "~ᾰκεκ~ηκοις|~ᾰκεκ~ηκοιης", _, _;
    perfect_opt_act_3: "~ᾰκεκ~ηκοι|~ᾰκεκ~ηκοιη", "~ᾰκεκ~ηκοιτην", _;
    perfect_imp_act_2: "~ᾰκεκ~ηκε", _, "~ᾰκεκ~ηκετε";
    perfect_imp_act_3: _, _, "~ᾰκεκ~ηκοντων";
    perfect_imp_mid_2: _, _, "~ᾰκεκ~ησθε";
    perfect_imp_mid_3: "~ᾰκεκ~ησθω", _, "~ᾰκεκ~ησθων";
    perfect_imp_pass_2: "~ᾰκεκ~ησο", _, "~ᾰκεκ~ησθε";
    perfect_imp_pass_3: _, "~ᾰκεκ~ησθων", "~ᾰκεκ~ησθων";
    pluperfect_mid_1: "~εκεκ~ημην", _, _;
    pluperfect_mid_2: "~εκεκ~ησο", "~εκεκ~ησθον", _;
    pluperfect_mid_3: "~εκεκ~ητο", _, "~εκεκ~ηντο";
    pluperfect_pass_1: _, _, "~εκεκ~ημεθᾰ";
    pluperfect_pass_2: _, _, "~εκεκ~ησθε";
    pluperfect_pass_3: _, "~εκεκ~ησθην", _;
    infinitive_pres: "~ᾰκᾰ~εῑν", _, "~ᾰκᾰ~εῑσθαι";
    infinitive_fut: "~ᾰκᾰ~εῑν", _, _;
    infinitive_aor: "~ᾰκᾰ~εσαι", _, _;
    infinitive_perf: _, "~ᾰκεκ~ησθαι", _;
    participle_pres: "~ᾰκᾰ~ων", _, "~ᾰκᾰ~ουμενος";
    participle_aor: "~ᾰκᾰ~εσᾱς", "~ᾰκᾰ~εσᾰμενος", _;
    participle_perf: "~ᾰκεκ~ηκως", _, "~ᾰκεκ~ημενος";
    ),
    // Στελλω: 3 words (στελλω, ἀγγελλω, τελλω). Like Καθιζω, except in 143 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("στελλω", 3, "~~λω", like Καθιζω;
    stem future_pass: "~~θησ- like ἐχω";
    stem aorist_ind: "~ι~- like ἐχω";
    stem pluperfect_act: "~τα~κειν- like κᾰλεω";
    ind_act_1: "~~λω", _, "~~λομεν";
    ind_act_2: "~~λεις", _, _;
    ind_act_3: "~~λει", "~~λετον", "~~λουσῐ";
    ind_mid_1: "~~λομαι", _, "~~λομεθᾰ";
    ind_mid_2: "~~λῃ", "~~λεσθον", _;
    ind_pass_2: _, _, "~~λεσθε";
    ind_pass_3: "~~λεται", _, "~~λονται";
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
    opt_mid_2: "~~λοιο", "~~λοισθον", _;
    opt_mid_3: "~~λοιτο", "~~λοισθην", "~~λοιντο";
    opt_pass_2: _, _, "~~λοισθε";
    imp_act_2: "~~λε", "~~λετον", "~~λετε";
    imp_mid_2: "~~λου", _, "~~λεσθε";
    imp_mid_3: "~~λεσθω", _, "~~λεσθων";
    imp_pass_2: _, _, "~~λεσθε";
    imp_pass_3: _, "~~λεσθων", "~~λεσθων";
    imperfect_act_1: "ἐ~~λον", _, "ἐ~~λομεν";
    imperfect_act_2: "ἐ~~λες", "ἐ~~λετον", "ἐ~~λετε";
    imperfect_act_3: "ἐ~~λε", "ἐ~~λετην", "ἐ~~λον";
    imperfect_mid_1: "ἐ~~λομην", _, "ἐ~~λομεθᾰ";
    imperfect_mid_2: "ἐ~~λου", _, _;
    imperfect_mid_3: "ἐ~~λετο", _, _;
    imperfect_pass_2: _, "ἐ~~λεσθον", "ἐ~~λεσθε";
    imperfect_pass_3: _, "ἐ~~λεσθην", "ἐ~~λοντο";
    future_ind_mid_2: "~~εῑ", _, _;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ι~ω", _, _;
    aorist_subj_act_2: "~ι~ῃς", "~ι~ητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~ι~ωμαι", _, _;
    aorist_subj_mid_2: "~ι~ῃ", "~ι~ησθον", _;
    aorist_subj_pass_3: derived, derived, _;
    aorist_opt_act_1: "~ι~αιμῐ", _, "~ι~αιμεν";
    aorist_opt_act_2: "~ι~ειᾰς", _, _;
    aorist_opt_act_3: "~ι~ειε", "~ι~αιτην", "~ι~ειᾰν";
    aorist_opt_mid_1: "~ι~αιμην", _, _;
    aorist_opt_mid_2: "~ι~αιο", _, _;
    aorist_imp_act_2: "~ῑ~ον", _, "~ι~ᾰτε";
    aorist_imp_act_3: _, _, "~ι~ᾰντων";
    aorist_imp_mid_2: "~ῑ~αι", "~ι~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ι~ᾰσθων";
    aorist_imp_pass_2: derived, derived, derived;
    perfect_ind_act_1: "~τα~κᾰ", _, "~τα~κᾰμεν";
    perfect_ind_act_2: _, "~τα~κᾰτον", _;
    perfect_ind_act_3: _, _, "~τα~κᾱσῐ";
    perfect_ind_mid_1: "~τα~μαι", _, "~τα~μεθᾰ";
    perfect_ind_mid_2: "~τα~σαι", "~τα~θον", _;
    perfect_ind_mid_3: "~τα~ται", _, "~τα~ᾰται";
    perfect_ind_pass_2: _, _, "~τα~θε";
    perfect_subj_act_1: "~τα~κω", _, _;
    perfect_subj_act_2: "~τα~κῃς", "~τα~κητον", "~τα~κητε";
    perfect_subj_act_3: "~τα~κῃ", _, "~τα~κωσῐ";
    perfect_opt_act_1: "~τα~κοιμῐ", _, "~τα~κοιμεν";
    perfect_opt_act_2: "~τα~κοις", _, _;
    perfect_opt_act_3: _, "~τα~κοιτην", _;
    perfect_imp_act_2: "~τα~κε", _, "~τα~κετε";
    perfect_imp_act_3: _, _, "~τα~κοντων";
    perfect_imp_mid_2: _, _, "~τα~θε";
    perfect_imp_mid_3: "~τα~θω", _, "~τα~θων";
    perfect_imp_pass_2: "~τα~σο", _, "~τα~θε";
    perfect_imp_pass_3: _, "~τα~θων", "~τα~θων";
    pluperfect_mid_1: "ἐ~τα~μην", _, _;
    pluperfect_mid_2: "ἐ~τα~σο", "ἐ~τα~θον", _;
    pluperfect_mid_3: "ἐ~τα~το", _, "ἐ~τα~ᾰτο";
    pluperfect_pass_1: _, _, "ἐ~τα~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~τα~θε";
    pluperfect_pass_3: _, "ἐ~τα~θην", _;
    infinitive_pres: "~~λειν", _, "~~λεσθαι";
    infinitive_aor: "~ῑ~αι", _, _;
    infinitive_perf: _, "~τᾱ~θαι", _;
    participle_pres: "~~λων", _, "~~λομενος";
    participle_aor: "~ι~ᾱς", "~ι~ᾰμενος", _;
    participle_perf: "~τα~κως", _, "~τα~μενος";
    ),
    // Ἐξαιρω: 3 words (κᾰταιρω, ἀνταιρω, ἐξαιρω). Like Δῐαιρω, except in 24 cells: aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, aorist_subj_mid_1 sg, aorist_subj_mid_2 sg, ….
    conjugation!("ἐξαιρω", 3, "~αι~ω", like Δῐαιρω;
    aorist_subj_act_1: "~α~ω", _, _;
    aorist_subj_act_2: "~α~ῃς", "~α~ητον", _;
    aorist_subj_act_3: _, _, "~α~ωσῐ|~α~ωσῐν";
    aorist_subj_mid_1: "~α~ωμαι", _, _;
    aorist_subj_mid_2: "~α~ῃ", "~α~ησθον", _;
    aorist_opt_act_1: "~α~αιμῐ", _, "~α~αιμεν";
    aorist_opt_act_2: "~α~ειᾰς|~α~αις", _, _;
    aorist_opt_act_3: "~α~ειε|~α~ειεν|~α~αι", "~α~αιτην", "~α~ειᾰν|~α~αιεν";
    aorist_opt_mid_1: "~α~αιμην", _, _;
    aorist_opt_mid_2: "~α~αιο", _, _;
    aorist_imp_act_2: "~α~ον", _, "~α~ᾰτε";
    aorist_imp_act_3: _, _, "~α~ᾰντων";
    aorist_imp_mid_2: "~α~αι", "~α~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~α~ᾰσθων";
    infinitive_aor: "~ᾱ~αι", _, _;
    participle_aor: "~α~ᾱς", "~α~ᾰμενος", _;
    ),
    // Δῐᾰλῡω: 3 words (ἀντᾰνᾰλῡω, δῐᾰλῡω, κᾰτᾰλῡω). Like Πεμπω, except in 180 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("δῐᾰλῡω", 3, "~ᾰ~ῡω", like Πεμπω;
    stem future_pass: "~ᾰ~ῠθησ- like ποιεω";
    stem aorist_ind: "~ε~ῡσ- like εὐλογεω";
    stem aorist_ind_pass: "~ε~ῠθην- like ἐλπω";
    stem aorist_pass: "~ᾰ~ῠθ- like ποιεω";
    stem pluperfect_act: "~ε~ελῠκ- like εὐλογεω";
    ind_act_1: "~ᾰ~ῡω", _, "~ᾰ~ῡομεν";
    ind_act_2: "~ᾰ~ῡεις", _, _;
    ind_act_3: "~ᾰ~ῡει", "~ᾰ~ῡετον", "~ᾰ~ῡουσῐ|~ᾰ~ῡουσῐν";
    ind_mid_1: "~ᾰ~ῡομαι", _, "~ᾰ~ῡομεθᾰ";
    ind_mid_2: "~ᾰ~ῡῃ|~ᾰ~ῡει", "~ᾰ~ῡεσθον", _;
    ind_pass_2: _, _, "~ᾰ~ῡεσθε";
    ind_pass_3: "~ᾰ~ῡεται", _, "~ᾰ~ῡονται";
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
    opt_mid_2: "~ᾰ~ῡοιο", "~ᾰ~ῡοισθον", _;
    opt_mid_3: "~ᾰ~ῡοιτο", "~ᾰ~ῡοισθην", "~ᾰ~ῡοιντο";
    opt_pass_2: _, _, "~ᾰ~ῡοισθε";
    imp_act_2: "~ᾰ~ῡε", "~ᾰ~ῡετον", "~ᾰ~ῡετε";
    imp_mid_2: "~ᾰ~ῡου", _, "~ᾰ~ῡεσθε";
    imp_mid_3: "~ᾰ~ῡεσθω", _, "~ᾰ~ῡεσθων";
    imp_pass_2: _, _, "~ᾰ~ῡεσθε";
    imp_pass_3: _, "~ᾰ~ῡεσθων", "~ᾰ~ῡεσθων";
    imperfect_act_1: "~ε~ῡον", _, "~ε~ῡομεν";
    imperfect_act_2: "~ε~ῡες", "~ε~ῡετον", "~ε~ῡετε";
    imperfect_act_3: "~ε~ῡε|~ε~ῡεν", "~ε~ῡετην", "~ε~ῡον";
    imperfect_mid_1: "~ε~ῡομην", _, "~ε~ῡομεθᾰ";
    imperfect_mid_2: "~ε~ῡου", _, _;
    imperfect_mid_3: "~ε~ῡετο", _, _;
    imperfect_pass_2: _, "~ε~ῡεσθον", "~ε~ῡεσθε";
    imperfect_pass_3: _, "~ε~ῡεσθην", "~ε~ῡοντο";
    future_ind_act_1: "~ᾰ~ῡσω", _, "~ᾰ~ῡσομεν";
    future_ind_act_2: "~ᾰ~ῡσεις", "~ᾰ~ῡσετον", "~ᾰ~ῡσετε";
    future_ind_act_3: _, _, "~ᾰ~ῡσουσῐ|~ᾰ~ῡσουσῐν";
    future_ind_mid_1: "~ᾰ~ῡσομαι", _, "~ᾰ~ῡσομεθᾰ";
    future_ind_mid_2: "~ᾰ~ῡσῃ|~ᾰ~ῡσει", "~ᾰ~ῡσεσθον", _;
    future_ind_mid_3: "~ᾰ~ῡσεται", _, _;
    future_opt_act_1: "~ᾰ~ῡσοιμῐ", _, "~ᾰ~ῡσοιμεν";
    future_opt_act_2: _, "~ᾰ~ῡσοιτον", "~ᾰ~ῡσοιτε";
    future_opt_act_3: "~ᾰ~ῡσοι", "~ᾰ~ῡσοιτην", "~ᾰ~ῡσοιεν";
    future_opt_mid_1: "~ᾰ~ῡσοιμην", _, "~ᾰ~ῡσοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ῡσοιο", _, _;
    future_opt_mid_3: _, "~ᾰ~ῡσοισθην", _;
    aorist_subj_act_1: "~ᾰ~ῡσω", _, _;
    aorist_subj_act_2: "~ᾰ~ῡσῃς", "~ᾰ~ῡσητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~ῡσωσῐ|~ᾰ~ῡσωσῐν";
    aorist_subj_mid_1: "~ᾰ~ῡσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ῡσῃ", "~ᾰ~ῡσησθον", _;
    aorist_opt_act_1: "~ᾰ~ῡσαιμῐ", _, "~ᾰ~ῡσαιμεν";
    aorist_opt_act_2: "~ᾰ~ῡσειᾰς|~ᾰ~ῡσαις", _, _;
    aorist_opt_act_3: "~ᾰ~ῡσειε|~ᾰ~ῡσειεν|~ᾰ~ῡσαι", "~ᾰ~ῡσαιτην", "~ᾰ~ῡσειᾰν|~ᾰ~ῡσαιεν";
    aorist_opt_mid_1: "~ᾰ~ῡσαιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~ῡσαιο", _, _;
    aorist_imp_act_2: "~ᾰ~ῡσον", _, "~ᾰ~ῡσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ῡσᾰντων";
    aorist_imp_mid_2: "~ᾰ~ῡσαι", "~ᾰ~ῡσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ῡσᾰσθων";
    perfect_ind_act_1: "~ᾰ~ελῠκᾰ", _, "~ᾰ~ελῠκᾰμεν";
    perfect_ind_act_2: _, "~ᾰ~ελῠκᾰτον", _;
    perfect_ind_act_3: "~ᾰ~ελῠκε|~ᾰ~ελῠκεν", _, "~ᾰ~ελῠκᾱσῐ|~ᾰ~ελῠκᾱσῐν";
    perfect_ind_mid_1: "~ᾰ~ελῠμαι", _, "~ᾰ~ελῠμεθᾰ";
    perfect_ind_mid_2: "~ᾰ~ελῠσαι", "~ᾰ~ελῠσθον", _;
    perfect_ind_mid_3: "~ᾰ~ελῠται", _, "~ᾰ~ελῠνται";
    perfect_ind_pass_2: _, _, "~ᾰ~ελῠσθε";
    perfect_subj_act_1: "~ᾰ~ελῠκω", _, _;
    perfect_subj_act_2: "~ᾰ~ελῠκῃς", "~ᾰ~ελῠκητον", "~ᾰ~ελῠκητε";
    perfect_subj_act_3: "~ᾰ~ελῠκῃ", _, "~ᾰ~ελῠκωσῐ|~ᾰ~ελῠκωσῐν";
    perfect_opt_act_1: "~ᾰ~ελῠκοιμῐ|~ᾰ~ελῠκοιην", _, "~ᾰ~ελῠκοιμεν";
    perfect_opt_act_2: "~ᾰ~ελῠκοις|~ᾰ~ελῠκοιης", _, _;
    perfect_opt_act_3: "~ᾰ~ελῠκοι|~ᾰ~ελῠκοιη", "~ᾰ~ελῠκοιτην", _;
    perfect_imp_act_2: "~ᾰ~ελῠκε", _, "~ᾰ~ελῠκετε";
    perfect_imp_act_3: _, _, "~ᾰ~ελῠκοντων";
    perfect_imp_mid_2: _, _, "~ᾰ~ελῠσθε";
    perfect_imp_mid_3: "~ᾰ~ελῠσθω", _, "~ᾰ~ελῠσθων";
    perfect_imp_pass_2: "~ᾰ~ελῠσο", _, "~ᾰ~ελῠσθε";
    perfect_imp_pass_3: _, "~ᾰ~ελῠσθων", "~ᾰ~ελῠσθων";
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
    // Ἐξαναλῡω: 3 words (ἐξαναλῡω, ἐπαναλῡω, ἐπαπολῡω). Like Δῐᾰλῡω, except in 156 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐξαναλῡω", 3, "~α~ῡω", like Δῐᾰλῡω;
    stem future_pass: "~α~ῠθησ- like ποιεω";
    stem aorist_pass: "~α~ῠθ- like ποιεω";
    ind_act_1: "~α~ῡω", _, "~α~ῡομεν";
    ind_act_2: "~α~ῡεις", _, _;
    ind_act_3: "~α~ῡει", "~α~ῡετον", "~α~ῡουσῐ|~α~ῡουσῐν";
    ind_mid_1: "~α~ῡομαι", _, "~α~ῡομεθᾰ";
    ind_mid_2: "~α~ῡῃ|~α~ῡει", "~α~ῡεσθον", _;
    ind_pass_2: _, _, "~α~ῡεσθε";
    ind_pass_3: "~α~ῡεται", _, "~α~ῡονται";
    subj_act_1: "~α~ῡω", _, "~α~ῡωμεν";
    subj_act_2: "~α~ῡῃς", _, _;
    subj_act_3: "~α~ῡῃ", "~α~ῡητον", "~α~ῡωσῐ|~α~ῡωσῐν";
    subj_mid_1: _, _, "~α~ῡωμεθᾰ";
    subj_mid_2: _, "~α~ῡησθον", _;
    subj_mid_3: "~α~ῡηται", _, "~α~ῡωνται";
    subj_pass_1: "~α~ῡωμαι", _, _;
    subj_pass_2: _, _, "~α~ῡησθε";
    opt_act_1: "~α~ῡοιμῐ", _, "~α~ῡοιμεν";
    opt_act_2: "~α~ῡοις", "~α~ῡοιτον", _;
    opt_act_3: "~α~ῡοι", "~α~ῡοιτην", "~α~ῡοιεν";
    opt_mid_1: "~α~ῡοιμην", _, "~α~ῡοιμεθᾰ";
    opt_mid_2: "~α~ῡοιο", "~α~ῡοισθον", _;
    opt_mid_3: "~α~ῡοιτο", "~α~ῡοισθην", "~α~ῡοιντο";
    opt_pass_2: _, _, "~α~ῡοισθε";
    imp_act_2: "~α~ῡε", "~α~ῡετον", "~α~ῡετε";
    imp_mid_2: "~α~ῡου", _, "~α~ῡεσθε";
    imp_mid_3: "~α~ῡεσθω", _, "~α~ῡεσθων";
    imp_pass_2: _, _, "~α~ῡεσθε";
    imp_pass_3: _, "~α~ῡεσθων", "~α~ῡεσθων";
    future_ind_act_1: "~α~ῡσω", _, "~α~ῡσομεν";
    future_ind_act_2: "~α~ῡσεις", "~α~ῡσετον", "~α~ῡσετε";
    future_ind_act_3: _, _, "~α~ῡσουσῐ|~α~ῡσουσῐν";
    future_ind_mid_1: "~α~ῡσομαι", _, "~α~ῡσομεθᾰ";
    future_ind_mid_2: "~α~ῡσῃ|~α~ῡσει", "~α~ῡσεσθον", _;
    future_ind_mid_3: "~α~ῡσεται", _, _;
    future_opt_act_1: "~α~ῡσοιμῐ", _, "~α~ῡσοιμεν";
    future_opt_act_2: _, "~α~ῡσοιτον", "~α~ῡσοιτε";
    future_opt_act_3: "~α~ῡσοι", "~α~ῡσοιτην", "~α~ῡσοιεν";
    future_opt_mid_1: "~α~ῡσοιμην", _, "~α~ῡσοιμεθᾰ";
    future_opt_mid_2: "~α~ῡσοιο", _, _;
    future_opt_mid_3: _, "~α~ῡσοισθην", _;
    aorist_subj_act_1: "~α~ῡσω", _, _;
    aorist_subj_act_2: "~α~ῡσῃς", "~α~ῡσητον", _;
    aorist_subj_act_3: _, _, "~α~ῡσωσῐ|~α~ῡσωσῐν";
    aorist_subj_mid_1: "~α~ῡσωμαι", _, _;
    aorist_subj_mid_2: "~α~ῡσῃ", "~α~ῡσησθον", _;
    aorist_opt_act_1: "~α~ῡσαιμῐ", _, "~α~ῡσαιμεν";
    aorist_opt_act_2: "~α~ῡσειᾰς|~α~ῡσαις", _, _;
    aorist_opt_act_3: "~α~ῡσειε|~α~ῡσειεν|~α~ῡσαι", "~α~ῡσαιτην", "~α~ῡσειᾰν|~α~ῡσαιεν";
    aorist_opt_mid_1: "~α~ῡσαιμην", _, _;
    aorist_opt_mid_2: "~α~ῡσαιο", _, _;
    aorist_imp_act_2: "~α~ῡσον", _, "~α~ῡσᾰτε";
    aorist_imp_act_3: _, _, "~α~ῡσᾰντων";
    aorist_imp_mid_2: "~α~ῡσαι", "~α~ῡσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~α~ῡσᾰσθων";
    perfect_ind_act_1: "~α~ελῠκᾰ", _, "~α~ελῠκᾰμεν";
    perfect_ind_act_2: _, "~α~ελῠκᾰτον", _;
    perfect_ind_act_3: "~α~ελῠκε|~α~ελῠκεν", _, "~α~ελῠκᾱσῐ|~α~ελῠκᾱσῐν";
    perfect_ind_mid_1: "~α~ελῠμαι", _, "~α~ελῠμεθᾰ";
    perfect_ind_mid_2: "~α~ελῠσαι", "~α~ελῠσθον", _;
    perfect_ind_mid_3: "~α~ελῠται", _, "~α~ελῠνται";
    perfect_ind_pass_2: _, _, "~α~ελῠσθε";
    perfect_subj_act_1: "~α~ελῠκω", _, _;
    perfect_subj_act_2: "~α~ελῠκῃς", "~α~ελῠκητον", "~α~ελῠκητε";
    perfect_subj_act_3: "~α~ελῠκῃ", _, "~α~ελῠκωσῐ|~α~ελῠκωσῐν";
    perfect_opt_act_1: "~α~ελῠκοιμῐ|~α~ελῠκοιην", _, "~α~ελῠκοιμεν";
    perfect_opt_act_2: "~α~ελῠκοις|~α~ελῠκοιης", _, _;
    perfect_opt_act_3: "~α~ελῠκοι|~α~ελῠκοιη", "~α~ελῠκοιτην", _;
    perfect_imp_act_2: "~α~ελῠκε", _, "~α~ελῠκετε";
    perfect_imp_act_3: _, _, "~α~ελῠκοντων";
    perfect_imp_mid_2: _, _, "~α~ελῠσθε";
    perfect_imp_mid_3: "~α~ελῠσθω", _, "~α~ελῠσθων";
    perfect_imp_pass_2: "~α~ελῠσο", _, "~α~ελῠσθε";
    perfect_imp_pass_3: _, "~α~ελῠσθων", "~α~ελῠσθων";
    future_perfect_ind_mid_1: "~α~ελῡσομαι", _, _;
    future_perfect_ind_mid_2: "~α~ελῡσῃ|~α~ελῡσει", _, _;
    future_perfect_ind_mid_3: _, "~α~ελῡσεσθον", _;
    future_perfect_ind_pass_1: _, _, "~α~ελῡσομεθᾰ";
    future_perfect_ind_pass_2: _, "~α~ελῡσεσθον", "~α~ελῡσεσθε";
    future_perfect_ind_pass_3: "~α~ελῡσεται", "~α~ελῡσεσθον", "~α~ελῡσονται";
    future_perfect_opt_pass_1: "~α~ελῡσοιμην", _, "~α~ελῡσοιμεθᾰ";
    future_perfect_opt_pass_2: "~α~ελῡσοιο", "~α~ελῡσοισθον", "~α~ελῡσοισθε";
    future_perfect_opt_pass_3: "~α~ελῡσοιτο", "~α~ελῡσοισθην", "~α~ελῡσοιντο";
    infinitive_pres: "~α~ῡειν", _, "~α~ῡεσθαι";
    infinitive_fut: "~α~ῡσειν", _, _;
    infinitive_aor: "~α~ῡσαι", _, _;
    infinitive_perf: _, "~α~ελῠσθαι", _;
    infinitive_futp: _, _, "~α~ελῡσεσθαι";
    participle_pres: "~α~ῡων", _, "~α~ῡομενος";
    participle_aor: "~α~ῡσᾱς", "~α~ῡσᾰμενος", _;
    participle_perf: "~α~ελῠκως", _, "~α~ελῠμενος";
    participle_futp: _, _, "~α~ελῡσομενος";
    ),
    // Ἁμᾰρτᾰνω: 3 words (ἁμᾰρτᾰνω, ἀφᾰμᾰρτᾰνω, ἀνᾰβλᾰστᾰνω). Like Ξαινω, except in 125 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἁμᾰρτᾰνω", 3, "~ᾰ~ᾰνω", like Ξαινω;
    stem future_pass: "~ᾰ~ηθησ- like ἐχω";
    stem aorist_ind: "~η~- like ἀνᾰγω";
    stem aorist_ind_pass: "~η~ηθην- like ἐλπω";
    stem aorist_pass: "~ᾰ~ηθ- like ἐχω";
    ind_act_1: "~ᾰ~ᾰνω", _, "~ᾰ~ᾰνομεν";
    ind_act_2: "~ᾰ~ᾰνεις", _, _;
    ind_act_3: "~ᾰ~ᾰνει", "~ᾰ~ᾰνετον", "~ᾰ~ᾰνουσῐ|~ᾰ~ᾰνουσῐν";
    ind_mid_1: "~ᾰ~ᾰνομαι", _, "~ᾰ~ᾰνομεθᾰ";
    ind_mid_2: "~ᾰ~ᾰνῃ", "~ᾰ~ᾰνεσθον", _;
    ind_pass_2: _, _, "~ᾰ~ᾰνεσθε";
    ind_pass_3: "~ᾰ~ᾰνεται", _, "~ᾰ~ᾰνονται";
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
    opt_mid_2: "~ᾰ~ᾰνοιο", "~ᾰ~ᾰνοισθον", _;
    opt_mid_3: "~ᾰ~ᾰνοιτο", "~ᾰ~ᾰνοισθην", "~ᾰ~ᾰνοιντο";
    opt_pass_2: _, _, "~ᾰ~ᾰνοισθε";
    imp_act_2: "~ᾰ~ᾰνε", "~ᾰ~ᾰνετον", "~ᾰ~ᾰνετε";
    imp_mid_2: "~ᾰ~ᾰνου", _, "~ᾰ~ᾰνεσθε";
    imp_mid_3: "~ᾰ~ᾰνεσθω", _, "~ᾰ~ᾰνεσθων";
    imp_pass_2: _, _, "~ᾰ~ᾰνεσθε";
    imp_pass_3: _, "~ᾰ~ᾰνεσθων", "~ᾰ~ᾰνεσθων";
    imperfect_act_1: "~η~ᾰνον", _, "~η~ᾰνομεν";
    imperfect_act_2: "~η~ᾰνες", "~η~ᾰνετον", "~η~ᾰνετε";
    imperfect_act_3: "~η~ᾰνε", "~η~ᾰνετην", "~η~ᾰνον";
    imperfect_mid_1: "~η~ᾰνομην", _, "~η~ᾰνομεθᾰ";
    imperfect_mid_2: "~η~ᾰνου", _, _;
    imperfect_mid_3: "~η~ᾰνετο", _, _;
    imperfect_pass_2: _, "~η~ᾰνεσθον", "~η~ᾰνεσθε";
    imperfect_pass_3: _, "~η~ᾰνεσθην", "~η~ᾰνοντο";
    future_ind_act_1: "~ᾰ~ησω", _, "~ᾰ~ησομεν";
    future_ind_act_2: "~ᾰ~ησεις", "~ᾰ~ησετον", "~ᾰ~ησετε";
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ|~ᾰ~ησουσῐν";
    future_ind_mid_1: "~ᾰ~ησομαι", _, "~ᾰ~ησομεθᾰ";
    future_ind_mid_2: "~ᾰ~ησῃ", "~ᾰ~ησεσθον", _;
    future_ind_mid_3: "~ᾰ~ησεται", _, _;
    future_opt_act_1: "~ᾰ~ησοιμῐ", _, "~ᾰ~ησοιμεν";
    future_opt_act_2: _, "~ᾰ~ησοιτον", "~ᾰ~ησοιτε";
    future_opt_act_3: "~ᾰ~ησοι", "~ᾰ~ησοιτην", "~ᾰ~ησοιεν";
    future_opt_mid_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ησοιο", _, _;
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
    aorist_opt_mid_2: "~ᾰ~οιο", _, _;
    aorist_imp_act_2: "~ᾰ~ε|~ᾰ~ησον", "~ᾰ~ετον|~ᾰ~ησᾰτον", "~ᾰ~ετε|~ᾰ~ησᾰτε";
    aorist_imp_act_3: "~ᾰ~ετω|~ᾰ~ησᾰτω", "~ᾰ~ετων|~ᾰ~ησᾰτων", "~ᾰ~οντων|~ᾰ~ησᾰντων";
    aorist_imp_mid_2: "~ᾰ~οῡ", "~ᾰ~εσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~εσθων";
    infinitive_pres: "~ᾰ~ᾰνειν", _, "~ᾰ~ᾰνεσθαι";
    infinitive_fut: "~ᾰ~ησειν", _, _;
    infinitive_aor: "~ᾰ~εῑν|~ᾰ~ησαι", _, _;
    participle_pres: "~ᾰ~ᾰνων", _, "~ᾰ~ᾰνομενος";
    participle_aor: "~ᾰ~ων|~ᾰ~ησᾱς", "~ᾰ~ομενος", _;
    ),
    // Κᾰμπτω: 3 words (κλεπτω, κᾰμπτω, κολᾰπτω). Like Θρῠπτω, except in 19 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 pl, perfect_ind_mid_3 pl, perfect_subj_act_1 sg, ….
    conjugation!("κᾰμπτω", 3, "~πτω", like Θρῠπτω;
    stem future_pass: "~φθησ- like ποιεω";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_3: _, _, "%ε~πᾰται";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: _, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
    participle_perf: derived, _, _;
    ),
    // Μεθῐστημῐ: 3 words (ἐγκᾰθῐστημῐ, μεθῐστημῐ, ἀντῐκᾰθῐστημῐ). Like Ξαινω, except in 137 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("μεθῐστημῐ", 3, "~θῐ~ημῐ", like Ξαινω;
    stem future_pass: "~τᾰ~ησ- like ποιεω";
    stem aorist_ind: "~τε~- like ἐφῐστημῐ";
    stem aorist_ind_pass: "~τε~ᾰθην- like ἐλπω";
    stem aorist_pass: "~τᾰ~ᾰθ- like ἐχω";
    ind_act_1: "~θῐ~ημῐ", _, "~θῐ~ᾰμεν";
    ind_act_2: "~θῐ~ης", _, _;
    ind_act_3: "~θῐ~ησῐ|~θῐ~ησῐν", "~θῐ~ᾰτον", "~θῐ~ᾱσῐ|~θῐ~ᾱσῐν";
    ind_mid_1: "~θῐ~ᾰμαι", _, "~θῐ~ᾰμεθᾰ";
    ind_mid_2: "~θῐ~ᾰσαι", "~θῐ~ᾰσθον", _;
    ind_pass_2: _, _, "~θῐ~ᾰσθε";
    ind_pass_3: "~θῐ~ᾰται", _, "~θῐ~ᾰνται";
    subj_act_1: "~θῐ~ω", _, "~θῐ~ωμεν";
    subj_act_2: "~θῐ~ῃς", _, _;
    subj_act_3: "~θῐ~ῃ", "~θῐ~ητον", "~θῐ~ωσῐ|~θῐ~ωσῐν";
    subj_mid_1: _, _, "~θῐ~ωμεθᾰ";
    subj_mid_2: _, "~θῐ~ησθον", _;
    subj_mid_3: "~θῐ~ηται", _, "~θῐ~ωνται";
    subj_pass_1: "~θῐ~ωμαι", _, _;
    subj_pass_2: _, _, "~θῐ~ησθε";
    opt_act_1: "~θῐ~αιην", _, "~θῐ~αῑμεν|~θῐ~αιημεν";
    opt_act_2: "~θῐ~αιης", "~θῐ~αῑτον|~θῐ~αιητον", "~θῐ~αῑτε|~θῐ~αιητε";
    opt_act_3: "~θῐ~αιη", "~θῐ~αιτην|~θῐ~αιητην", "~θῐ~αῑεν|~θῐ~αιησᾰν";
    opt_mid_1: "~θῐ~αιμην", _, "~θῐ~αιμεθᾰ";
    opt_mid_2: "~θῐ~αιο", "~θῐ~αισθον", _;
    opt_mid_3: "~θῐ~αιτο", "~θῐ~αισθην", "~θῐ~αιντο";
    opt_pass_2: _, _, "~θῐ~αισθε";
    imp_act_2: "~θῐ~η", "~θῐ~ᾰτον", "~θῐ~ᾰτε";
    imp_mid_2: "~θῐ~ᾰσο|~θῐ~ω", _, "~θῐ~ᾰσθε";
    imp_mid_3: "~θῐ~ᾰσθω", _, "~θῐ~ᾰσθων";
    imp_pass_2: _, _, "~θῐ~ᾰσθε";
    imp_pass_3: _, "~θῐ~ᾰσθων", "~θῐ~ᾰσθων";
    imperfect_act_1: "~θῑ~ην", _, "~θῑ~ᾰμεν";
    imperfect_act_2: "~θῑ~ης", "~θῑ~ᾰτον", "~θῑ~ᾰτε";
    imperfect_act_3: "~θῑ~η", "~θῑ~ᾰτην", "~θῑ~ᾰσᾰν";
    imperfect_mid_1: "~θῑ~ᾰμην", _, "~θῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~θῑ~ᾰσο", _, _;
    imperfect_mid_3: "~θῑ~ᾰτο", _, _;
    imperfect_pass_2: _, "~θῑ~ᾰσθον", "~θῑ~ᾰσθε";
    imperfect_pass_3: _, "~θῑ~ᾰσθην", "~θῑ~ᾰντο";
    future_ind_act_1: "~τᾰ~ησω", _, "~τᾰ~ησομεν";
    future_ind_act_2: "~τᾰ~ησεις", "~τᾰ~ησετον", "~τᾰ~ησετε";
    future_ind_act_3: _, _, "~τᾰ~ησουσῐ|~τᾰ~ησουσῐν";
    future_ind_mid_1: "~τᾰ~ησομαι", _, "~τᾰ~ησομεθᾰ";
    future_ind_mid_2: "~τᾰ~ησῃ|~τᾰ~ησει", "~τᾰ~ησεσθον", _;
    future_ind_mid_3: "~τᾰ~ησεται", _, _;
    future_opt_act_1: "~τᾰ~ησοιμῐ", _, "~τᾰ~ησοιμεν";
    future_opt_act_2: _, "~τᾰ~ησοιτον", "~τᾰ~ησοιτε";
    future_opt_act_3: "~τᾰ~ησοι", "~τᾰ~ησοιτην", "~τᾰ~ησοιεν";
    future_opt_mid_1: "~τᾰ~ησοιμην", _, "~τᾰ~ησοιμεθᾰ";
    future_opt_mid_2: "~τᾰ~ησοιο", _, _;
    future_opt_mid_3: _, "~τᾰ~ησοισθην", _;
    aorist_subj_act_1: "~τᾰ~ησω", _, _;
    aorist_subj_act_2: "~τᾰ~ησῃς", "~τᾰ~ησητον", _;
    aorist_subj_mid_1: "~τᾰ~ησωμαι", _, _;
    aorist_subj_mid_2: "~τᾰ~ησῃ", "~τᾰ~ησησθον", _;
    aorist_opt_act_1: "~τᾰ~ησαιμῐ", _, "~τᾰ~ησαιμεν";
    aorist_opt_act_2: "~τᾰ~ησειᾰς", _, _;
    aorist_opt_act_3: "~τᾰ~ησειε", "~τᾰ~ησαιτην", "~τᾰ~ησειᾰν";
    aorist_opt_mid_1: "~τᾰ~ησαιμην", _, _;
    aorist_opt_mid_2: "~τᾰ~ησαιο", _, _;
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
    perfect_opt_act_1: "~θε~ηκοιμῐ|~θε~ηκοιην", _, "~θε~ηκοιμεν";
    perfect_opt_act_2: "~θε~ηκοις|~θε~ηκοιης", _, _;
    perfect_opt_act_3: "~θε~ηκοι|~θε~ηκοιη", "~θε~ηκοιτην", _;
    perfect_imp_act_2: "~θε~ηκε", _, "~θε~ηκετε";
    perfect_imp_act_3: _, _, "~θε~ηκοντων";
    infinitive_pres: "~θῐ~ᾰναι", _, "~θῐ~ᾰσθαι";
    infinitive_fut: "~τᾰ~ησειν", _, _;
    infinitive_aor: "~τᾰ~ησαι", _, _;
    participle_pres: "~θῐ~ᾱς", _, "~θῐ~ᾰμενος";
    participle_aor: "~τᾰ~ησᾱς", "~τᾰ~ησᾰμενος", _;
    participle_perf: "~θε~ηκως", _, _;
    ),
    // Ἐκφερω: 3 words (ἐκφερω, ἐκγᾰμῐζω, ὑπεκφευγω). Like Ἐγκειμαι, except in 85 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐκφερω", 3, "~κ~ω", like Ἐγκειμαι;
    ind_act_1: "~κ~ω", _, "~κ~ομεν";
    ind_act_2: "~κ~εις", _, _;
    ind_act_3: "~κ~ει", "~κ~ετον", "~κ~ουσῐ";
    ind_mid_1: "~κ~ομαι", _, "~κ~ομεθᾰ";
    ind_mid_2: "~κ~ῃ", "~κ~εσθον", _;
    ind_mid_3: _, _, derived;
    ind_pass_2: _, _, "~κ~εσθε";
    ind_pass_3: "~κ~εται", _, "~κ~ονται";
    subj_act_1: "~κ~ω", _, "~κ~ωμεν";
    subj_act_2: "~κ~ῃς", _, _;
    subj_act_3: "~κ~ῃ", "~κ~ητον", "~κ~ωσῐ";
    subj_mid_1: derived, _, "~κ~ωμεθᾰ";
    subj_mid_2: derived, "~κ~ησθον", _;
    subj_mid_3: "~κ~ηται", _, "~κ~ωνται";
    subj_pass_1: "~κ~ωμαι", _, _;
    subj_pass_2: _, _, "~κ~ησθε";
    opt_act_1: "~κ~οιμῐ", _, "~κ~οιμεν";
    opt_act_2: "~κ~οις", "~κ~οιτον", _;
    opt_act_3: "~κ~οι", "~κ~οιτην", "~κ~οιεν";
    opt_mid_1: "~κ~οιμην", _, "~κ~οιμεθᾰ";
    opt_mid_2: "~κ~οιο", "~κ~οισθον", _;
    opt_mid_3: "~κ~οιτο", "~κ~οισθην", "~κ~οιντο";
    opt_pass_2: _, _, "~κ~οισθε";
    imp_act_2: "~κ~ε", "~κ~ετον", "~κ~ετε";
    imp_mid_2: "~κ~ου", _, "~κ~εσθε";
    imp_mid_3: "~κ~εσθω", _, "~κ~εσθων";
    imp_pass_2: _, _, "~κ~εσθε";
    imp_pass_3: _, "~κ~εσθων", "~κ~εσθων";
    imperfect_act_1: "~ξε~ον", _, "~ξε~ομεν";
    imperfect_act_2: "~ξε~ες", "~ξε~ετον", "~ξε~ετε";
    imperfect_act_3: "~ξε~ε", "~ξε~ετην", "~ξε~ον";
    imperfect_mid_1: "~ξε~ομην", _, "~ξε~ομεθᾰ";
    imperfect_mid_2: "~ξε~ου", derived, _;
    imperfect_mid_3: "~ξε~ετο", derived, derived;
    imperfect_pass_2: _, "~ξε~εσθον", "~ξε~εσθε";
    imperfect_pass_3: _, "~ξε~εσθην", "~ξε~οντο";
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, _, _;
    future_opt_mid_3: _, derived, _;
    infinitive_pres: "~κ~ειν", _, "~κ~εσθαι";
    participle_pres: "~κ~ων", _, "~κ~ομενος";
    ),
    // Ἐξετᾰζω: 3 words (προετοιμᾰζω, ἐξετᾰζω, ἀντελπῐζω). Like Δῐᾰρπᾰζω, except in 102 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐξετᾰζω", 3, "~ε~ζω", like Δῐᾰρπᾰζω;
    stem future_pass: "~ε~σθησ- like ποιεω";
    stem aorist_pass: "~ε~σθ- like ποιεω";
    ind_act_1: "~ε~ζω", _, "~ε~ζομεν";
    ind_act_2: "~ε~ζεις", _, _;
    ind_act_3: "~ε~ζει", "~ε~ζετον", "~ε~ζουσῐ|~ε~ζουσῐν";
    ind_mid_1: "~ε~ζομαι", _, "~ε~ζομεθᾰ";
    ind_mid_2: "~ε~ζῃ|~ε~ζει", "~ε~ζεσθον", _;
    ind_pass_2: _, _, "~ε~ζεσθε";
    ind_pass_3: "~ε~ζεται", _, "~ε~ζονται";
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
    opt_mid_2: "~ε~ζοιο", "~ε~ζοισθον", _;
    opt_mid_3: "~ε~ζοιτο", "~ε~ζοισθην", "~ε~ζοιντο";
    opt_pass_2: _, _, "~ε~ζοισθε";
    imp_act_2: "~ε~ζε", "~ε~ζετον", "~ε~ζετε";
    imp_mid_2: "~ε~ζου", _, "~ε~ζεσθε";
    imp_mid_3: "~ε~ζεσθω", _, "~ε~ζεσθων";
    imp_pass_2: _, _, "~ε~ζεσθε";
    imp_pass_3: _, "~ε~ζεσθων", "~ε~ζεσθων";
    future_ind_act_1: "~ε~ω", _, "~ε~οῡμεν";
    future_ind_act_2: "~ε~εῑς", "~ε~εῑτον", "~ε~εῑτε";
    future_ind_act_3: _, _, "~ε~οῡσῐ";
    future_ind_mid_1: "~ε~οῡμαι", _, "~ε~ουμεθᾰ";
    future_ind_mid_2: "~ε~εῑ", "~ε~εῑσθον", _;
    future_ind_mid_3: "~ε~εῑται", _, _;
    future_opt_act_1: "~ε~οιην", _, "~ε~οῑμεν";
    future_opt_act_2: derived, "~ε~οῑτον", "~ε~οῑτε";
    future_opt_act_3: "~ε~οιη", "~ε~οιτην", "~ε~οῑεν";
    future_opt_mid_1: "~ε~οιμην", _, "~ε~οιμεθᾰ";
    future_opt_mid_2: "~ε~οῑο", _, _;
    future_opt_mid_3: _, "~ε~οισθην", _;
    aorist_subj_act_1: "~ε~σω", _, _;
    aorist_subj_act_2: "~ε~σῃς", "~ε~σητον", _;
    aorist_subj_act_3: _, _, "~ε~σωσῐ|~ε~σωσῐν";
    aorist_subj_mid_1: "~ε~σωμαι", _, _;
    aorist_subj_mid_2: "~ε~σῃ", "~ε~σησθον", _;
    aorist_opt_act_1: "~ε~σαιμῐ", _, "~ε~σαιμεν";
    aorist_opt_act_2: "~ε~σειᾰς|~ε~σαις", _, _;
    aorist_opt_act_3: "~ε~σειε|~ε~σειεν|~ε~σαι", "~ε~σαιτην", "~ε~σειᾰν|~ε~σαιεν";
    aorist_opt_mid_1: "~ε~σαιμην", _, _;
    aorist_opt_mid_2: "~ε~σαιο", _, _;
    aorist_imp_act_2: "~ε~σον", _, "~ε~σᾰτε";
    aorist_imp_act_3: _, _, "~ε~σᾰντων";
    aorist_imp_mid_2: "~ε~σαι", "~ε~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ε~σᾰσθων";
    infinitive_pres: "~ε~ζειν", _, "~ε~ζεσθαι";
    infinitive_fut: "~ε~εῑν", _, _;
    infinitive_aor: "~ε~σαι", _, _;
    participle_pres: "~ε~ζων", _, "~ε~ζομενος";
    participle_aor: "~ε~σᾱς", "~ε~σᾰμενος", _;
    ),
    // Σῠναιρεω: 3 words (κᾰθαιρεω, σῠναιρεω, ξῠναιρεω). Like Ἀναιρεω, except in 92 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 pl, ….
    conjugation!("σῠναιρεω", 3, "~αιρεω", like Ἀναιρεω;
    stem aorist_ind: "~ειλ- like πᾰρεχω";
    future_ind_act_1: "~ελω", _, "~ελοῡμεν";
    future_ind_act_2: "~ελεῑς", "~ελεῑτον", "~ελεῑτε";
    future_ind_act_3: _, _, "~ελοῡσῐ|~ελοῡσῐν";
    future_ind_mid_1: "~ελοῡμαι", _, "~ελουμεθᾰ";
    future_ind_mid_2: "~ελεῑ|~ελῃ", "~ελεῑσθον", _;
    future_ind_mid_3: "~ελεῑται", _, _;
    future_ind_pass_1: derived, _, _;
    future_ind_pass_2: derived, _, _;
    future_ind_pass_3: _, derived, _;
    future_opt_act_1: "~ελοιην|~ελοῑμῐ", _, "~ελοῑμεν|~ελοιημεν";
    future_opt_act_2: "~ελοιης|~ελοῑς", "~ελοῑτον|~ελοιητον", "~ελοῑτε|~ελοιητε";
    future_opt_act_3: "~ελοιη|~ελοῑ", "~ελοιτην|~ελοιητην", "~ελοῑεν|~ελοιησᾰν";
    future_opt_mid_1: "~ελοιμην", _, "~ελοιμεθᾰ";
    future_opt_mid_2: "~ελοῑο", _, _;
    future_opt_mid_3: _, "~ελοισθην", _;
    aorist_ind_act_2: _, _, derived;
    aorist_ind_act_3: _, derived, _;
    aorist_ind_mid_1: _, _, derived;
    aorist_ind_mid_2: _, _, derived;
    aorist_ind_mid_3: derived, derived, derived;
    aorist_ind_pass_1: derived, _, _;
    aorist_subj_act_1: "~ελω", _, derived;
    aorist_subj_act_2: "~ελῃς", "~ελητον", derived;
    aorist_subj_act_3: derived, _, "~ελωσῐ|~ελωσῐν";
    aorist_subj_mid_1: "~ελωμαι", _, derived;
    aorist_subj_mid_2: "~ελῃ", "~ελησθον", derived;
    aorist_subj_mid_3: derived, _, derived;
    aorist_subj_pass_3: derived, derived, derived;
    aorist_opt_act_1: "~ελοιμῐ", _, "~ελοιμεν";
    aorist_opt_act_2: "~ελοις", derived, derived;
    aorist_opt_act_3: "~ελοι", "~ελοιτην", "~ελοιεν";
    aorist_opt_mid_1: "~ελοιμην", _, derived;
    aorist_opt_mid_2: "~ελοιο", derived, derived;
    aorist_opt_mid_3: derived, derived, derived;
    aorist_opt_pass_1: _, _, derived;
    aorist_opt_pass_2: _, derived, derived;
    aorist_opt_pass_3: _, derived, derived;
    aorist_imp_act_2: "~ελε", derived, "~ελετε";
    aorist_imp_act_3: derived, derived, "~ελοντων";
    aorist_imp_mid_2: "~ελοῡ", "~ελεσθον", derived;
    aorist_imp_mid_3: derived, derived, "~ελεσθων";
    aorist_imp_pass_2: derived, derived, derived;
    infinitive_fut: "~ελεῑν", _, _;
    infinitive_aor: "~ελεῑν", derived, _;
    participle_aor: "~ελων", "~ελομενος", _;
    ),
    // Φῡρω: 3 words (σῡρω, τῡφω, φῡρω). Like Ποιμαινω, except in 132 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("φῡρω", 3, "~ῡ~ω", like Ποιμαινω;
    stem aorist_ind: "~ῠ~σ- like ἐχω";
    stem aorist_ind_pass: "~ῠ~θην- like ἐχω";
    stem aorist_pass: "~ῠ~θ- like ἐχω";
    ind_act_1: "~ῡ~ω", _, "~ῡ~ομεν";
    ind_act_2: "~ῡ~εις", _, _;
    ind_act_3: "~ῡ~ει", "~ῡ~ετον", "~ῡ~ουσῐ|~ῡ~ουσῐν";
    ind_mid_1: "~ῡ~ομαι", _, "~ῡ~ομεθᾰ";
    ind_mid_2: "~ῡ~ῃ|~ῡ~ει", "~ῡ~εσθον", _;
    ind_pass_2: _, _, "~ῡ~εσθε";
    ind_pass_3: "~ῡ~εται", _, "~ῡ~ονται";
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
    opt_mid_2: "~ῡ~οιο", "~ῡ~οισθον", _;
    opt_mid_3: "~ῡ~οιτο", "~ῡ~οισθην", "~ῡ~οιντο";
    opt_pass_2: _, _, "~ῡ~οισθε";
    imp_act_2: "~ῡ~ε", "~ῡ~ετον", "~ῡ~ετε";
    imp_mid_2: "~ῡ~ου", _, "~ῡ~εσθε";
    imp_mid_3: "~ῡ~εσθω", _, "~ῡ~εσθων";
    imp_pass_2: _, _, "~ῡ~εσθε";
    imp_pass_3: _, "~ῡ~εσθων", "~ῡ~εσθων";
    imperfect_act_1: "ἐ~ῡ~ον", _, "ἐ~ῡ~ομεν";
    imperfect_act_2: "ἐ~ῡ~ες", "ἐ~ῡ~ετον", "ἐ~ῡ~ετε";
    imperfect_act_3: "ἐ~ῡ~ε|ἐ~ῡ~εν", "ἐ~ῡ~ετην", "ἐ~ῡ~ον";
    imperfect_mid_1: "ἐ~ῡ~ομην", _, "ἐ~ῡ~ομεθᾰ";
    imperfect_mid_2: "ἐ~ῡ~ου", _, _;
    imperfect_mid_3: "ἐ~ῡ~ετο", _, _;
    imperfect_pass_2: _, "ἐ~ῡ~εσθον", "ἐ~ῡ~εσθε";
    imperfect_pass_3: _, "ἐ~ῡ~εσθην", "ἐ~ῡ~οντο";
    future_ind_act_1: "~ῠ~σω", _, "~ῠ~σομεν";
    future_ind_act_2: "~ῠ~σεις", "~ῠ~σετον", "~ῠ~σετε";
    future_ind_act_3: _, _, "~ῠ~σουσῐ";
    future_ind_mid_1: "~ῠ~σομαι", _, "~ῠ~σομεθᾰ";
    future_ind_mid_2: "~ῠ~σῃ", "~ῠ~σεσθον", _;
    future_ind_mid_3: "~ῠ~σεται", _, _;
    future_opt_act_1: "~ῠ~σοιμῐ", _, "~ῠ~σοιμεν";
    future_opt_act_2: _, "~ῠ~σοιτον", "~ῠ~σοιτε";
    future_opt_act_3: "~ῠ~σοι", "~ῠ~σοιτην", "~ῠ~σοιεν";
    future_opt_mid_1: "~ῠ~σοιμην", _, "~ῠ~σοιμεθᾰ";
    future_opt_mid_2: "~ῠ~σοιο", _, _;
    future_opt_mid_3: _, "~ῠ~σοισθην", _;
    aorist_subj_act_1: "~ῠ~σω", _, _;
    aorist_subj_act_2: "~ῠ~σῃς", "~ῠ~σητον", _;
    aorist_subj_mid_1: "~ῠ~σωμαι", _, _;
    aorist_subj_mid_2: "~ῠ~σῃ", "~ῠ~σησθον", _;
    aorist_opt_act_1: "~ῠ~σαιμῐ", _, "~ῠ~σαιμεν";
    aorist_opt_act_2: "~ῠ~σειᾰς", _, _;
    aorist_opt_act_3: "~ῠ~σειε", "~ῠ~σαιτην", "~ῠ~σειᾰν";
    aorist_opt_mid_1: "~ῠ~σαιμην", _, _;
    aorist_opt_mid_2: "~ῠ~σαιο", _, _;
    aorist_imp_act_2: "~ῠ~σον", _, "~ῠ~σᾰτε";
    aorist_imp_act_3: _, _, "~ῠ~σᾰντων";
    aorist_imp_mid_2: "~ῠ~σαι", "~ῠ~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῠ~σᾰσθων";
    perfect_ind_mid_1: "%ε~ῠ~μαι", _, "%ε~ῠ~μεθᾰ";
    perfect_ind_mid_2: "%ε~ῠ~σαι", "%ε~ῠ~θον", _;
    perfect_ind_mid_3: "%ε~ῠ~ται", _, "%ε~ῠ~ᾰται";
    perfect_ind_pass_2: _, _, "%ε~ῠ~θε";
    perfect_imp_mid_2: _, _, "%ε~ῠ~θε";
    perfect_imp_mid_3: "%ε~ῠ~θω", _, "%ε~ῠ~θων";
    perfect_imp_pass_2: "%ε~ῠ~σο", _, "%ε~ῠ~θε";
    perfect_imp_pass_3: _, "%ε~ῠ~θων", "%ε~ῠ~θων";
    infinitive_pres: "~ῡ~ειν", _, "~ῡ~εσθαι";
    infinitive_fut: "~ῠ~σειν", _, _;
    infinitive_aor: "~ῠ~σαι", _, _;
    infinitive_perf: _, "%ε~ῠ~θαι", _;
    participle_pres: "~ῡ~ων", _, "~ῡ~ομενος";
    participle_aor: "~ῠ~σᾱς", "~ῠ~σᾰμενος", _;
    participle_perf: _, _, "%ε~ῠ~μενος";
    ),
    // Σταθμαω: 3 words (χαλαω, καθοραω, σταθμαω). Like Κατοικεω, except in 106 cells: ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("σταθμαω", 3, "~α~αω", like Κατοικεω;
    stem future_pass: "~ᾰ~ησθησ- like ἐχω";
    stem aorist_ind: "~ᾰ~ησ- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~ησθην- like ἐχω";
    stem aorist_pass: "~ᾰ~ησθ- like ἐχω";
    ind_act_1: _, _, "~ᾰ~ωμεν";
    ind_act_2: "~ᾰ~ᾳς", _, _;
    ind_act_3: "~ᾰ~ᾳ", "~ᾰ~ᾱτον", "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    ind_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    ind_mid_2: "~ᾰ~ᾳ", "~ᾰ~ᾱσθον", _;
    ind_pass_2: _, _, "~ᾰ~ᾱσθε";
    ind_pass_3: "~ᾰ~ᾱται", _, "~ᾰ~ωνται";
    subj_act_2: "~ᾰ~ᾳς", _, _;
    subj_act_3: "~ᾰ~ᾳ", "~ᾰ~ᾱτον", _;
    subj_mid_2: _, "~ᾰ~ᾱσθον", _;
    subj_mid_3: "~ᾰ~ᾱται", _, _;
    subj_pass_2: _, _, "~ᾰ~ᾱσθε";
    opt_act_1: "~ᾰ~ῳην|~ᾰ~ῳμῐ", _, "~ᾰ~ῳμεν|~ᾰ~ῳημεν";
    opt_act_2: "~ᾰ~ῳης|~ᾰ~ῳς", "~ᾰ~ῳτον|~ᾰ~ῳητον", "~ᾰ~ῳτε|~ᾰ~ῳητε";
    opt_act_3: "~ᾰ~ῳη|~ᾰ~ῳ", "~ᾰ~ῳτην|~ᾰ~ῳητην", "~ᾰ~ῳεν|~ᾰ~ῳησᾰν";
    opt_mid_1: "~ᾰ~ῳμην", _, "~ᾰ~ῳμεθᾰ";
    opt_mid_2: "~ᾰ~ῳο", "~ᾰ~ῳσθον", _;
    opt_mid_3: "~ᾰ~ῳτο", "~ᾰ~ῳσθην", "~ᾰ~ῳντο";
    opt_pass_2: _, _, "~ᾰ~ῳσθε";
    imp_act_2: "~ᾰ~ᾱ", "~ᾰ~ᾱτον", "~ᾰ~ᾱτε";
    imp_mid_2: "~ᾰ~ω", _, "~ᾰ~ᾱσθε";
    imp_mid_3: "~ᾰ~ᾱσθω", _, "~ᾰ~ᾱσθων";
    imp_pass_2: _, _, "~ᾰ~ᾱσθε";
    imp_pass_3: _, "~ᾰ~ᾱσθων", "~ᾰ~ᾱσθων";
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, _, _;
    imperfect_mid_3: derived, _, _;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, derived;
    perfect_ind_act_1: "ἐ~ᾰ~ηκᾰ", _, "ἐ~ᾰ~ηκᾰμεν";
    perfect_ind_act_2: _, "ἐ~ᾰ~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~ᾰ~ηκᾱσῐ";
    perfect_ind_mid_1: "ἐ~ᾰ~ημαι", _, "ἐ~ᾰ~ημεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰ~ησαι", "ἐ~ᾰ~ησθον", _;
    perfect_ind_mid_3: "ἐ~ᾰ~ηται", _, "ἐ~ᾰ~ηνται";
    perfect_ind_pass_2: _, _, "ἐ~ᾰ~ησθε";
    perfect_subj_act_1: "ἐ~ᾰ~ηκω", _, _;
    perfect_subj_act_2: "ἐ~ᾰ~ηκῃς", "ἐ~ᾰ~ηκητον", "ἐ~ᾰ~ηκητε";
    perfect_subj_act_3: "ἐ~ᾰ~ηκῃ", _, "ἐ~ᾰ~ηκωσῐ";
    perfect_opt_act_1: "ἐ~ᾰ~ηκοιμῐ", _, "ἐ~ᾰ~ηκοιμεν";
    perfect_opt_act_2: "ἐ~ᾰ~ηκοις", _, _;
    perfect_opt_act_3: _, "ἐ~ᾰ~ηκοιτην", _;
    perfect_imp_act_2: "ἐ~ᾰ~ηκε", _, "ἐ~ᾰ~ηκετε";
    perfect_imp_act_3: _, _, "ἐ~ᾰ~ηκοντων";
    perfect_imp_mid_2: _, _, "ἐ~ᾰ~ησθε";
    perfect_imp_mid_3: "ἐ~ᾰ~ησθω", _, "ἐ~ᾰ~ησθων";
    perfect_imp_pass_2: "ἐ~ᾰ~ησο", _, "ἐ~ᾰ~ησθε";
    perfect_imp_pass_3: _, "ἐ~ᾰ~ησθων", "ἐ~ᾰ~ησθων";
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
    // Κνᾰω: 3 words (βοᾰω, τρῠγᾰω, κνᾰω). Like Χραω, except in 79 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("κνᾰω", 3, "~ᾰω", like Χραω;
    stem future_pass: "~ησθησ- like ποιεω";
    stem aorist_pass: "~ησθ- like ποιεω";
    stem pluperfect_act: "~ηκ- like χρημᾰτῐζω";
    ind_act_1: "~ᾰω|~ω", _, "~ᾰομεν|~ωμεν";
    ind_act_2: "~ᾰεις|~ᾳς", "~ᾰετον|~ᾱτον", "~ᾰετε|~ᾱτε";
    ind_act_3: "~ᾰει|~ᾳ", derived, "~ᾰουσῐ|~ᾰουσῐν|~ωσῐ|~ωσῐν";
    ind_mid_1: "~ᾰομαι|~ωμαι", _, "~ᾰομεθᾰ|~ωμεθᾰ";
    ind_mid_2: "~ᾰῃ|~ᾰει|~ᾳ", "~ᾰεσθον|~ᾱσθον", "~ᾰεσθε|~ᾱσθε";
    ind_mid_3: "~ᾰεται|~ᾱται", _, "~ᾰονται|~ωνται";
    ind_pass_2: _, _, derived;
    ind_pass_3: derived, _, derived;
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
    opt_pass_2: _, _, derived;
    imp_act_2: "~ᾰε|~ᾱ", derived, derived;
    imp_act_3: "~ᾰετω|~ᾱτω", "~ᾰετων|~ᾱτων", "~ᾰοντων|~ωντων";
    imp_mid_2: "~ᾰου|~ω", _, derived;
    imp_mid_3: "~ᾰεσθω|~ᾱσθω", "~ᾰεσθων|~ᾱσθων", derived;
    imp_pass_2: _, _, derived;
    imp_pass_3: _, derived, derived;
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
    stem aorist_ind: "~ησ- like πᾰραιτεομαι";
    stem aorist_ind_pass: "~ηθην- like ἐχω";
    ind_mid_2: "~εῑ|~ῃ", _, _;
    ind_pass_2: "~εῑ", _, _;
    imperfect_mid_1: "ἐ~ουμην", _, "ἐ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~οῡ", "ἐ~εῑσθον", _;
    imperfect_mid_3: "ἐ~εῑτο", "ἐ~εισθην", "ἐ~οῡντο";
    imperfect_pass_2: _, _, "ἐ~εῑσθε";
    future_ind_mid_2: "~ησῃ|~ησει", _, _;
    perfect_ind_mid_1: "%ε~ημαι", _, "%ε~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ησαι", "%ε~ησθον", _;
    perfect_ind_mid_3: "%ε~ηται", _, "%ε~ηνται";
    perfect_ind_pass_2: _, _, "%ε~ησθε";
    perfect_imp_mid_2: _, _, "%ε~ησθε";
    perfect_imp_mid_3: "%ε~ησθω", _, "%ε~ησθων";
    perfect_imp_pass_2: "%ε~ησο", _, "%ε~ησθε";
    perfect_imp_pass_3: _, "%ε~ησθων", "%ε~ησθων";
    pluperfect_mid_1: "ἐ%ε~ημην", _, _;
    pluperfect_mid_2: "ἐ%ε~ησο", "ἐ%ε~ησθον", _;
    pluperfect_mid_3: "ἐ%ε~ητο", _, "ἐ%ε~ηντο";
    pluperfect_pass_1: _, _, "ἐ%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~ησθην", _;
    infinitive_perf: _, "%ε~ησθαι", _;
    participle_perf: _, _, "%ε~ημενος";
    ),
    // Ζεω: 3 words (ζεω, πταιω, ξεω). Like Κᾰλεω, except in 26 cells: ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, imp_act_2 sg, imp_act_2 du, ….
    conjugation!("ζεω", 3, "~ω", like Κᾰλεω;
    stem aorist_ind_pass: "~σθην- like ἐχω";
    stem aorist_pass: "~σθ- like ποιεω";
    ind_act_2: "~ῑς", _, _;
    ind_act_3: "~ῑ", "~ῑτον", _;
    ind_mid_2: "~ῃ|~ει", _, _;
    imp_act_2: "~ῑ", "~ῑτον", "~ῑτε";
    imperfect_act_2: "ἐ~ις", "ἐ~ῑτον", _;
    imperfect_act_3: "ἐ~ι", "ἐ~ιτην", "ἐ~ῑτε";
    future_ind_act_3: _, _, "~σουσῐ|~σουσῐν";
    future_ind_pass_1: derived, _, _;
    future_ind_pass_3: _, derived, _;
    perfect_ind_mid_1: "ἐ~σμαι", _, "ἐ~σμεθᾰ";
    perfect_ind_mid_3: "ἐ~σται", _, "ἐ~σᾰται";
    pluperfect_act_1: derived, _, _;
    pluperfect_mid_1: "ἐ~σμην", _, _;
    pluperfect_mid_3: "ἐ~στο", _, "ἐ~σᾰτο";
    pluperfect_pass_1: _, _, "ἐ~σμεθᾰ";
    infinitive_pres: "~ῑν", _, _;
    participle_perf: _, _, "ἐ~σμενος";
    ),
    // Ὑπᾰκουω: 3 words (ὑπᾰκουω, ἐπᾰκουω, εἰσᾰκουω). Like Ξαινω, except in 137 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ὑπᾰκουω", 3, "~ᾰ~υω", like Ξαινω;
    stem future_pass: "~ᾰ~υσθησ- like ποιεω";
    stem aorist_ind: "~η~υσ- like εὐλογεω";
    stem aorist_ind_pass: "~η~υσθην- like ἐλπω";
    stem aorist_pass: "~ᾰ~υσθ- like ποιεω";
    ind_act_1: "~ᾰ~υω", _, "~ᾰ~υομεν";
    ind_act_2: "~ᾰ~υεις", _, _;
    ind_act_3: "~ᾰ~υει", "~ᾰ~υετον", "~ᾰ~υουσῐ|~ᾰ~υουσῐν";
    ind_mid_1: "~ᾰ~υομαι", _, "~ᾰ~υομεθᾰ";
    ind_mid_2: "~ᾰ~υῃ|~ᾰ~υει", "~ᾰ~υεσθον", _;
    ind_pass_2: _, _, "~ᾰ~υεσθε";
    ind_pass_3: "~ᾰ~υεται", _, "~ᾰ~υονται";
    subj_act_1: "~ᾰ~υω", _, "~ᾰ~υωμεν";
    subj_act_2: "~ᾰ~υῃς", _, _;
    subj_act_3: "~ᾰ~υῃ", "~ᾰ~υητον", "~ᾰ~υωσῐ|~ᾰ~υωσῐν";
    subj_mid_1: _, _, "~ᾰ~υωμεθᾰ";
    subj_mid_2: _, "~ᾰ~υησθον", _;
    subj_mid_3: "~ᾰ~υηται", _, "~ᾰ~υωνται";
    subj_pass_1: "~ᾰ~υωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~υησθε";
    opt_act_1: "~ᾰ~υοιμῐ", _, "~ᾰ~υοιμεν";
    opt_act_2: "~ᾰ~υοις", "~ᾰ~υοιτον", _;
    opt_act_3: "~ᾰ~υοι", "~ᾰ~υοιτην", "~ᾰ~υοιεν";
    opt_mid_1: "~ᾰ~υοιμην", _, "~ᾰ~υοιμεθᾰ";
    opt_mid_2: "~ᾰ~υοιο", "~ᾰ~υοισθον", _;
    opt_mid_3: "~ᾰ~υοιτο", "~ᾰ~υοισθην", "~ᾰ~υοιντο";
    opt_pass_2: _, _, "~ᾰ~υοισθε";
    imp_act_2: "~ᾰ~υε", "~ᾰ~υετον", "~ᾰ~υετε";
    imp_mid_2: "~ᾰ~υου", _, "~ᾰ~υεσθε";
    imp_mid_3: "~ᾰ~υεσθω", _, "~ᾰ~υεσθων";
    imp_pass_2: _, _, "~ᾰ~υεσθε";
    imp_pass_3: _, "~ᾰ~υεσθων", "~ᾰ~υεσθων";
    imperfect_act_1: "~η~υον", _, "~η~υομεν";
    imperfect_act_2: "~η~υες", "~η~υετον", "~η~υετε";
    imperfect_act_3: "~η~υε|~η~υεν", "~η~υετην", "~η~υον";
    imperfect_mid_1: "~η~υομην", _, "~η~υομεθᾰ";
    imperfect_mid_2: "~η~υου", _, _;
    imperfect_mid_3: "~η~υετο", _, _;
    imperfect_pass_2: _, "~η~υεσθον", "~η~υεσθε";
    imperfect_pass_3: _, "~η~υεσθην", "~η~υοντο";
    future_ind_act_1: "~ᾰ~υσω", _, "~ᾰ~υσομεν";
    future_ind_act_2: "~ᾰ~υσεις", "~ᾰ~υσετον", "~ᾰ~υσετε";
    future_ind_act_3: _, _, "~ᾰ~υσουσῐ|~ᾰ~υσουσῐν";
    future_ind_mid_1: "~ᾰ~υσομαι", _, "~ᾰ~υσομεθᾰ";
    future_ind_mid_2: "~ᾰ~υσῃ|~ᾰ~υσει", "~ᾰ~υσεσθον", _;
    future_ind_mid_3: "~ᾰ~υσεται", _, _;
    future_opt_act_1: "~ᾰ~υσοιμῐ", _, "~ᾰ~υσοιμεν";
    future_opt_act_2: _, "~ᾰ~υσοιτον", "~ᾰ~υσοιτε";
    future_opt_act_3: "~ᾰ~υσοι", "~ᾰ~υσοιτην", "~ᾰ~υσοιεν";
    future_opt_mid_1: "~ᾰ~υσοιμην", _, "~ᾰ~υσοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~υσοιο", _, _;
    future_opt_mid_3: _, "~ᾰ~υσοισθην", _;
    aorist_subj_act_1: "~ᾰ~υσω", _, _;
    aorist_subj_act_2: "~ᾰ~υσῃς", "~ᾰ~υσητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~υσωσῐ|~ᾰ~υσωσῐν";
    aorist_subj_mid_1: "~ᾰ~υσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~υσῃ", "~ᾰ~υσησθον", _;
    aorist_opt_act_1: "~ᾰ~υσαιμῐ", _, "~ᾰ~υσαιμεν";
    aorist_opt_act_2: "~ᾰ~υσειᾰς|~ᾰ~υσαις", _, _;
    aorist_opt_act_3: "~ᾰ~υσειε|~ᾰ~υσειεν|~ᾰ~υσαι", "~ᾰ~υσαιτην", "~ᾰ~υσειᾰν|~ᾰ~υσαιεν";
    aorist_opt_mid_1: "~ᾰ~υσαιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~υσαιο", _, _;
    aorist_imp_act_2: "~ᾰ~υσον", _, "~ᾰ~υσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~υσᾰντων";
    aorist_imp_mid_2: "~ᾰ~υσαι", "~ᾰ~υσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~υσᾰσθων";
    perfect_ind_act_1: "~ᾰ%η~ᾰ", _, "~ᾰ%η~ᾰμεν";
    perfect_ind_act_2: _, "~ᾰ%η~ᾰτον", _;
    perfect_ind_act_3: "~ᾰ%η~ε|~ᾰ%η~εν", _, "~ᾰ%η~ᾱσῐ|~ᾰ%η~ᾱσῐν";
    perfect_subj_act_1: "~ᾰ%η~ω", _, _;
    perfect_subj_act_2: "~ᾰ%η~ῃς", "~ᾰ%η~ητον", "~ᾰ%η~ητε";
    perfect_subj_act_3: "~ᾰ%η~ῃ", _, "~ᾰ%η~ωσῐ|~ᾰ%η~ωσῐν";
    perfect_opt_act_1: "~ᾰ%η~οιμῐ|~ᾰ%η~οιην", _, "~ᾰ%η~οιμεν";
    perfect_opt_act_2: "~ᾰ%η~οις|~ᾰ%η~οιης", _, _;
    perfect_opt_act_3: "~ᾰ%η~οι|~ᾰ%η~οιη", "~ᾰ%η~οιτην", _;
    perfect_imp_act_2: "~ᾰ%η~ε", _, "~ᾰ%η~ετε";
    perfect_imp_act_3: _, _, "~ᾰ%η~οντων";
    infinitive_pres: "~ᾰ~υειν", _, "~ᾰ~υεσθαι";
    infinitive_fut: "~ᾰ~υσειν", _, _;
    infinitive_aor: "~ᾰ~ῡσαι", _, _;
    participle_pres: "~ᾰ~υων", _, "~ᾰ~υομενος";
    participle_aor: "~ᾰ~υσᾱς", "~ᾰ~υσᾰμενος", _;
    participle_perf: "~ᾰ%η~ως", _, _;
    ),
    // Λευκαινω: 3 words (λευκαινω, λῑμαινω, ξηραινω). Like Μωραινω, except in 75 cells: imperfect_act_3 sg, future_ind_act_3 pl, future_ind_mid_2 sg, future_opt_act_1 sg, future_opt_act_1 pl, future_opt_act_2 sg, ….
    conjugation!("λευκαινω", 3, "~αινω", like Μωραινω;
    stem aorist_ind: "~αν- like ἐχω";
    stem aorist_pass: "~ανθ- like ποιεω";
    imperfect_act_3: "ἐ~αινε", _, _;
    future_ind_act_3: _, _, "~ανοῡσῐ|~ανοῡσῐν";
    future_ind_mid_2: "~ανεῑ|~ανῃ", _, _;
    future_opt_act_1: "~ανοιην|~ανοῑμῐ", _, "~ανοῑμεν|~ανοιημεν";
    future_opt_act_2: "~ανοιης|~ανοῑς", "~ανοῑτον|~ανοιητον", "~ανοῑτε|~ανοιητε";
    future_opt_act_3: "~ανοιη|~ανοῑ", "~ανοιτην|~ανοιητην", "~ανοῑεν|~ανοιησᾰν";
    aorist_subj_act_1: "~ανω", _, _;
    aorist_subj_act_2: "~ανῃς", "~ανητον", _;
    aorist_subj_mid_1: "~ανωμαι", _, _;
    aorist_subj_mid_2: "~ανῃ", "~ανησθον", _;
    aorist_opt_act_1: "~αναιμῐ", _, "~αναιμεν";
    aorist_opt_act_2: "~ανειᾰς", _, _;
    aorist_opt_act_3: "~ανειε", "~αναιτην", "~ανειᾰν";
    aorist_opt_mid_1: "~αναιμην", _, _;
    aorist_opt_mid_2: "~αναιο", _, _;
    aorist_imp_act_2: "~ανον", _, "~ανᾰτε";
    aorist_imp_act_3: _, _, "~ανᾰντων";
    aorist_imp_mid_2: "~αναι", "~ανᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ανᾰσθων";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: _, _, derived;
    perfect_ind_mid_1: "ἐ~ασμαι", _, "ἐ~ασμεθᾰ";
    perfect_ind_mid_2: "ἐ~ανσαι", "ἐ~ανθον", _;
    perfect_ind_mid_3: "ἐ~ανται", _, "ἐ~ανᾰται";
    perfect_ind_pass_2: _, _, "ἐ~ανθε";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: _, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, _, "ἐ~ανθε";
    perfect_imp_mid_3: "ἐ~ανθω", _, "ἐ~ανθων";
    perfect_imp_pass_2: "ἐ~ανσο", _, "ἐ~ανθε";
    perfect_imp_pass_3: _, "ἐ~ανθων", "ἐ~ανθων";
    pluperfect_mid_1: "ἐ~ασμην", _, _;
    pluperfect_mid_2: "ἐ~ανσο", "ἐ~ανθον", _;
    pluperfect_mid_3: "ἐ~αντο", _, "ἐ~ανᾰτο";
    pluperfect_pass_1: _, _, "ἐ~ασμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ανθε";
    pluperfect_pass_3: _, "ἐ~ανθην", _;
    infinitive_perf: _, "ἐ~ᾱνθαι", _;
    participle_aor: "~ανᾱς", "~ανᾰμενος", _;
    participle_perf: derived, _, "ἐ~ασμενος";
    ),
    // Σῠνδεω: 3 words (σῠνδεω, σῠμμᾰχεω, τρωγλοδῠτεω). Like Πεμπω, except in 158 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("σῠνδεω", 3, "~ῠ~εω", like Πεμπω;
    stem future_pass: "~υ~ηθησ- like ἐχω";
    stem aorist_ind: "~υ~ησ- like ἐχω";
    stem aorist_ind_pass: "~υ~ηθην- like ἐχω";
    stem aorist_pass: "~υ~ηθ- like ἐχω";
    stem pluperfect_act: "~υ~ηκειν- like ἐχω";
    ind_act_1: "~υ~ω", _, "~υ~οῡμεν";
    ind_act_2: "~υ~εῑς", _, _;
    ind_act_3: "~υ~εῑ", "~υ~εῑτον", "~υ~οῡσῐ";
    ind_mid_1: "~υ~οῡμαι", _, "~υ~ουμεθᾰ";
    ind_mid_2: "~υ~εῑ", "~υ~εῑσθον", _;
    ind_pass_2: _, _, "~υ~εῑσθε";
    ind_pass_3: "~υ~εῑται", _, "~υ~οῡνται";
    subj_act_1: "~υ~ω", _, "~υ~ωμεν";
    subj_act_2: "~υ~ῃς", _, _;
    subj_act_3: "~υ~ῃ", "~υ~ητον", "~υ~ωσῐ";
    subj_mid_1: _, _, "~υ~ωμεθᾰ";
    subj_mid_2: _, "~υ~ησθον", _;
    subj_mid_3: "~υ~ηται", _, "~υ~ωνται";
    subj_pass_1: "~υ~ωμαι", _, _;
    subj_pass_2: _, _, "~υ~ησθε";
    opt_act_1: "~υ~οιην", _, "~υ~οῑμεν";
    opt_act_2: "~υ~οιης", "~υ~οῑτον", _;
    opt_act_3: "~υ~οιη", "~υ~οιτην", "~υ~οῑεν";
    opt_mid_1: "~υ~οιμην", _, "~υ~οιμεθᾰ";
    opt_mid_2: "~υ~οῑο", "~υ~οῑσθον", _;
    opt_mid_3: "~υ~οῑτο", "~υ~οισθην", "~υ~οῑντο";
    opt_pass_2: _, _, "~υ~οῑσθε";
    imp_act_2: "~υ~ει", "~υ~εῑτον", "~υ~εῑτε";
    imp_mid_2: "~υ~οῡ", _, "~υ~εῑσθε";
    imp_mid_3: "~υ~εισθω", _, "~υ~εισθων";
    imp_pass_2: _, _, "~υ~εῑσθε";
    imp_pass_3: _, "~υ~εισθων", "~υ~εισθων";
    imperfect_act_1: "ἐ~υ~ουν", _, "ἐ~υ~οῡμεν";
    imperfect_act_2: "ἐ~υ~εις", "ἐ~υ~εῑτον", "ἐ~υ~εῑτε";
    imperfect_act_3: "ἐ~υ~ει", "ἐ~υ~ειτην", "ἐ~υ~ουν";
    imperfect_mid_1: "ἐ~υ~ουμην", _, "ἐ~υ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~υ~οῡ", _, _;
    imperfect_mid_3: "ἐ~υ~εῑτο", _, _;
    imperfect_pass_2: _, "ἐ~υ~εῑσθον", "ἐ~υ~εῑσθε";
    imperfect_pass_3: _, "ἐ~υ~εισθην", "ἐ~υ~οῡντο";
    future_ind_act_1: "~υ~ησω", _, "~υ~ησομεν";
    future_ind_act_2: "~υ~ησεις", "~υ~ησετον", "~υ~ησετε";
    future_ind_act_3: _, _, "~υ~ησουσῐ";
    future_ind_mid_1: "~υ~ησομαι", _, "~υ~ησομεθᾰ";
    future_ind_mid_2: "~υ~ησῃ", "~υ~ησεσθον", _;
    future_ind_mid_3: "~υ~ησεται", _, _;
    future_opt_act_1: "~υ~ησοιμῐ", _, "~υ~ησοιμεν";
    future_opt_act_2: _, "~υ~ησοιτον", "~υ~ησοιτε";
    future_opt_act_3: "~υ~ησοι", "~υ~ησοιτην", "~υ~ησοιεν";
    future_opt_mid_1: "~υ~ησοιμην", _, "~υ~ησοιμεθᾰ";
    future_opt_mid_2: "~υ~ησοιο", _, _;
    future_opt_mid_3: _, "~υ~ησοισθην", _;
    aorist_subj_act_1: "~υ~ησω", _, _;
    aorist_subj_act_2: "~υ~ησῃς", "~υ~ησητον", _;
    aorist_subj_mid_1: "~υ~ησωμαι", _, _;
    aorist_subj_mid_2: "~υ~ησῃ", "~υ~ησησθον", _;
    aorist_opt_act_1: "~υ~ησαιμῐ", _, "~υ~ησαιμεν";
    aorist_opt_act_2: "~υ~ησειᾰς", _, _;
    aorist_opt_act_3: "~υ~ησειε", "~υ~ησαιτην", "~υ~ησειᾰν";
    aorist_opt_mid_1: "~υ~ησαιμην", _, _;
    aorist_opt_mid_2: "~υ~ησαιο", _, _;
    aorist_imp_act_2: "~υ~ησον", _, "~υ~ησᾰτε";
    aorist_imp_act_3: _, _, "~υ~ησᾰντων";
    aorist_imp_mid_2: "~υ~ησαι", "~υ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~υ~ησᾰσθων";
    perfect_ind_act_1: "%ε~υ~ηκᾰ", _, "%ε~υ~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~υ~ηκᾰτον", _;
    perfect_ind_act_3: _, _, "%ε~υ~ηκᾱσῐ";
    perfect_ind_mid_1: "%ε~υ~ημαι", _, "%ε~υ~ημεθᾰ";
    perfect_ind_mid_2: "%ε~υ~ησαι", "%ε~υ~ησθον", _;
    perfect_ind_mid_3: "%ε~υ~ηται", _, "%ε~υ~ηνται";
    perfect_ind_pass_2: _, _, "%ε~υ~ησθε";
    perfect_subj_act_1: "%ε~υ~ηκω", _, _;
    perfect_subj_act_2: "%ε~υ~ηκῃς", "%ε~υ~ηκητον", "%ε~υ~ηκητε";
    perfect_subj_act_3: "%ε~υ~ηκῃ", _, "%ε~υ~ηκωσῐ";
    perfect_opt_act_1: "%ε~υ~ηκοιμῐ", _, "%ε~υ~ηκοιμεν";
    perfect_opt_act_2: "%ε~υ~ηκοις", _, _;
    perfect_opt_act_3: _, "%ε~υ~ηκοιτην", _;
    perfect_imp_act_2: "%ε~υ~ηκε", _, "%ε~υ~ηκετε";
    perfect_imp_act_3: _, _, "%ε~υ~ηκοντων";
    perfect_imp_mid_2: _, _, "%ε~υ~ησθε";
    perfect_imp_mid_3: "%ε~υ~ησθω", _, "%ε~υ~ησθων";
    perfect_imp_pass_2: "%ε~υ~ησο", _, "%ε~υ~ησθε";
    perfect_imp_pass_3: _, "%ε~υ~ησθων", "%ε~υ~ησθων";
    pluperfect_mid_1: "ἐ%ε~υ~ημην", _, _;
    pluperfect_mid_2: "ἐ%ε~υ~ησο", "ἐ%ε~υ~ησθον", _;
    pluperfect_mid_3: "ἐ%ε~υ~ητο", _, "ἐ%ε~υ~ηντο";
    pluperfect_pass_1: _, _, "ἐ%ε~υ~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~υ~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~υ~ησθην", _;
    infinitive_pres: "~υ~εῑν", _, "~υ~εῑσθαι";
    infinitive_fut: "~υ~ησειν", _, _;
    infinitive_aor: "~υ~ησαι", _, _;
    infinitive_perf: _, "%ε~υ~ησθαι", _;
    participle_pres: "~υ~ων", _, "~υ~ουμενος";
    participle_aor: "~υ~ησᾱς", "~υ~ησᾰμενος", _;
    participle_perf: "%ε~υ~ηκως", _, "%ε~υ~ημενος";
    ),
    // Κηδευω: 3 words (σκῡλευω, κηδευω, τυμβευω). Like Βουλευω, except in 44 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_ind_mid_1 sg, ….
    conjugation!("κηδευω", 3, "~υω", like Βουλευω;
    stem pluperfect_act: "~υκειν- like κᾰλεω";
    perfect_ind_act_1: "~υκᾰ", _, "~υκᾰμεν";
    perfect_ind_act_2: _, "~υκᾰτον", _;
    perfect_ind_act_3: "~υκε|~υκεν", _, "~υκᾱσῐ|~υκᾱσῐν";
    perfect_ind_mid_1: "~υμαι", _, "~υμεθᾰ";
    perfect_ind_mid_2: "~υσαι", "~υσθον", _;
    perfect_ind_mid_3: "~υται", _, "~υνται";
    perfect_ind_pass_2: _, _, "~υσθε";
    perfect_subj_act_1: "~υκω", _, _;
    perfect_subj_act_2: "~υκῃς", "~υκητον", "~υκητε";
    perfect_subj_act_3: "~υκῃ", _, "~υκωσῐ|~υκωσῐν";
    perfect_opt_act_1: "~υκοιμῐ|~υκοιην", _, "~υκοιμεν";
    perfect_opt_act_2: "~υκοις|~υκοιης", _, _;
    perfect_opt_act_3: "~υκοι|~υκοιη", "~υκοιτην", _;
    perfect_imp_act_2: "~υκε", _, "~υκετε";
    perfect_imp_act_3: _, _, "~υκοντων";
    perfect_imp_mid_2: _, _, "~υσθε";
    perfect_imp_mid_3: "~υσθω", _, "~υσθων";
    perfect_imp_pass_2: "~υσο", _, "~υσθε";
    perfect_imp_pass_3: _, "~υσθων", "~υσθων";
    pluperfect_mid_1: "ἐ~υμην", _, _;
    pluperfect_mid_2: "ἐ~υσο", "ἐ~υσθον", _;
    pluperfect_mid_3: "ἐ~υτο", _, "ἐ~υντο";
    pluperfect_pass_1: _, _, "ἐ~υμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~υσθε";
    pluperfect_pass_3: _, "ἐ~υσθην", _;
    infinitive_perf: _, "~ῡσθαι", _;
    participle_perf: "~υκως", _, "~υμενος";
    ),
    // Λεγομαι: 3 words (φθεγγομαι, ἐρευγομαι, λεγομαι). Like Ἐρᾰμαι, except in 64 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("λεγομαι", 3, "~γομαι", like Ἐρᾰμαι;
    stem future_pass: "~χθησ- like ἐχω";
    stem aorist_ind: "~ξ- like πᾰραιτεομαι";
    stem aorist_ind_pass: "~γην- like ἐχω";
    stem aorist_pass: "~γ- like σφᾰλλω";
    ind_act_1: "~γομαι", _, _;
    ind_mid_1: "~γομαι", _, "~γομεθᾰ";
    ind_mid_2: "~γῃ|~γει", "~γεσθον", _;
    ind_pass_2: _, _, "~γεσθε";
    ind_pass_3: "~γεται", _, "~γονται";
    subj_mid_1: "~γωμαι", _, "~γωμεθᾰ";
    subj_mid_2: "~γῃ", "~γησθον", _;
    subj_mid_3: "~γηται", _, "~γωνται";
    subj_pass_2: "~γῃ", _, "~γησθε";
    opt_mid_1: "~γοιμην", _, "~γοιμεθᾰ";
    opt_mid_2: "~γοιο", "~γοισθον", _;
    opt_mid_3: "~γοιτο", "~γοισθην", "~γοιντο";
    opt_pass_2: _, _, "~γοισθε";
    imp_mid_2: "~γου", _, "~γεσθε";
    imp_mid_3: "~γεσθω", _, "~γεσθων";
    imp_pass_2: _, _, "~γεσθε";
    imp_pass_3: _, "~γεσθων", "~γεσθων";
    imperfect_mid_1: "ἐ~γομην", _, "ἐ~γομεθᾰ";
    imperfect_mid_2: "ἐ~γου", "ἐ~γεσθον", _;
    imperfect_mid_3: "ἐ~γετο", "ἐ~γεσθην", "ἐ~γοντο";
    imperfect_pass_2: _, _, "ἐ~γεσθε";
    future_ind_mid_1: "~ξομαι", _, "~ξομεθᾰ";
    future_ind_mid_2: "~ξῃ", "~ξεσθον", _;
    future_ind_mid_3: "~ξεται", _, _;
    future_opt_mid_1: "~ξοιμην", _, "~ξοιμεθᾰ";
    future_opt_mid_2: "~ξοιο", _, _;
    future_opt_mid_3: _, "~ξοισθην", _;
    perfect_ind_mid_1: "ἐι~γμαι", _, "ἐι~γμεθᾰ";
    perfect_ind_mid_2: "ἐι~ξαι", "ἐι~χθον", _;
    perfect_ind_mid_3: "ἐι~κται", _, "ἐι~γᾰται";
    perfect_imp_mid_2: _, _, "ἐι~χθε";
    perfect_imp_mid_3: "ἐι~χθω", _, "ἐι~χθων";
    pluperfect_mid_1: "ἐι~γμην", _, _;
    pluperfect_mid_2: "ἐι~ξο", "ἐι~χθον", _;
    pluperfect_mid_3: "ἐι~κτο", _, "ἐι~γᾰτο";
    infinitive_pres: _, _, "~γεσθαι";
    participle_pres: _, _, "~γομενος";
    ),
    // Ῥεω: 3 words (ῥεω, ἐκρεω, ρρεω). Like Αἱρεω, except in 113 cells: ind_mid_2 sg, imperfect_pass_2 du, imperfect_pass_2 pl, imperfect_pass_3 du, imperfect_pass_3 pl, future_ind_act_1 sg, ….
    conjugation!("ῥεω", 3, "~εω", like Αἱρεω;
    stem future_pass: "~ῠησ- like ἐχω";
    stem aorist_ind: "~ευσ- like ἐχω";
    stem aorist_ind_pass: "~ῠην- like ἐχω";
    stem aorist_pass: "~ῠ- like σφᾰλλω";
    ind_mid_2: "~εει", _, _;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, derived;
    future_ind_act_1: "~ευσω", _, "~ευσομεν";
    future_ind_act_2: "~ευσεις", "~ευσετον", "~ευσετε";
    future_ind_act_3: _, _, "~ευσουσῐ";
    future_ind_mid_1: "~ευσομαι", _, "~ευσομεθᾰ";
    future_ind_mid_2: "~ευσῃ", "~ευσεσθον", _;
    future_ind_mid_3: "~ευσεται", _, _;
    future_opt_act_1: "~ευσοιμῐ", _, "~ευσοιμεν";
    future_opt_act_2: _, "~ευσοιτον", "~ευσοιτε";
    future_opt_act_3: "~ευσοι", "~ευσοιτην", "~ευσοιεν";
    future_opt_mid_1: "~ευσοιμην", _, "~ευσοιμεθᾰ";
    future_opt_mid_2: "~ευσοιο", _, _;
    future_opt_mid_3: _, "~ευσοισθην", _;
    aorist_subj_act_1: "~ευσω", _, _;
    aorist_subj_act_2: "~ευσῃς", "~ευσητον", _;
    aorist_subj_mid_1: "~ευσωμαι", _, _;
    aorist_subj_mid_2: "~ευσῃ", "~ευσησθον", _;
    aorist_opt_act_1: "~ευσαιμῐ", _, "~ευσαιμεν";
    aorist_opt_act_2: "~ευσειᾰς", _, _;
    aorist_opt_act_3: "~ευσειε", "~ευσαιτην", "~ευσειᾰν";
    aorist_opt_mid_1: "~ευσαιμην", _, _;
    aorist_opt_mid_2: "~ευσαιο", _, _;
    aorist_imp_act_2: "~ευσον", _, "~ευσᾰτε";
    aorist_imp_act_3: _, _, "~ευσᾰντων";
    aorist_imp_mid_2: "~ευσαι", "~ευσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ευσᾰσθων";
    perfect_ind_act_1: "ἐ~υηκᾰ", _, "ἐ~υηκᾰμεν";
    perfect_ind_act_2: _, "ἐ~υηκᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~υηκᾱσῐ";
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, _;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "ἐ~υηκω", _, _;
    perfect_subj_act_2: "ἐ~υηκῃς", "ἐ~υηκητον", "ἐ~υηκητε";
    perfect_subj_act_3: "ἐ~υηκῃ", _, "ἐ~υηκωσῐ";
    perfect_opt_act_1: "ἐ~υηκοιμῐ", _, "ἐ~υηκοιμεν";
    perfect_opt_act_2: "ἐ~υηκοις", _, _;
    perfect_opt_act_3: _, "ἐ~υηκοιτην", _;
    perfect_imp_act_2: "ἐ~υηκε", _, "ἐ~υηκετε";
    perfect_imp_act_3: _, _, "ἐ~υηκοντων";
    perfect_imp_mid_2: _, _, derived;
    perfect_imp_mid_3: derived, _, derived;
    perfect_imp_pass_2: derived, _, derived;
    perfect_imp_pass_3: _, derived, derived;
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
    infinitive_aor: "~εῡσαι", _, _;
    infinitive_perf: _, derived, _;
    infinitive_futp: _, _, derived;
    participle_aor: "~ευσᾱς", "~ευσᾰμενος", _;
    participle_perf: "ἐ~υηκως", _, derived;
    participle_futp: _, _, derived;
    ),
    // Δῐᾰσῳζω: 3 words (ξῠσσῳζω, δῐᾰσῳζω, σσῳζω). Like Ληγω, except in 150 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("δῐᾰσῳζω", 3, "~σῳζω", like Ληγω;
    stem future_pass: "~σωθησ- like ἐχω";
    stem aorist_ind: "~ωσ- like ἐχω";
    stem aorist_ind_pass: "~ωθην- like ἐχω";
    stem aorist_pass: "~σωθ- like ἐχω";
    ind_act_1: "~σῳζω", _, "~σῳζομεν";
    ind_act_2: "~σῳζεις", _, _;
    ind_act_3: "~σῳζει", "~σῳζετον", "~σῳζουσῐ";
    ind_mid_1: "~σῳζομαι", _, "~σῳζομεθᾰ";
    ind_mid_2: "~σῳζῃ", "~σῳζεσθον", _;
    ind_pass_2: _, _, "~σῳζεσθε";
    ind_pass_3: "~σῳζεται", _, "~σῳζονται";
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
    opt_mid_2: "~σῳζοιο", "~σῳζοισθον", _;
    opt_mid_3: "~σῳζοιτο", "~σῳζοισθην", "~σῳζοιντο";
    opt_pass_2: _, _, "~σῳζοισθε";
    imp_act_2: "~σῳζε", "~σῳζετον", "~σῳζετε";
    imp_mid_2: "~σῳζου", _, "~σῳζεσθε";
    imp_mid_3: "~σῳζεσθω", _, "~σῳζεσθων";
    imp_pass_2: _, _, "~σῳζεσθε";
    imp_pass_3: _, "~σῳζεσθων", "~σῳζεσθων";
    imperfect_act_1: "ἐ~ῳζον", _, "ἐ~ῳζομεν";
    imperfect_act_2: "ἐ~ῳζες", "ἐ~ῳζετον", "ἐ~ῳζετε";
    imperfect_act_3: "ἐ~ῳζε", "ἐ~ῳζετην", "ἐ~ῳζον";
    imperfect_mid_1: "ἐ~ῳζομην", _, "ἐ~ῳζομεθᾰ";
    imperfect_mid_2: "ἐ~ῳζου", _, _;
    imperfect_mid_3: "ἐ~ῳζετο", _, _;
    imperfect_pass_2: _, "ἐ~ῳζεσθον", "ἐ~ῳζεσθε";
    imperfect_pass_3: _, "ἐ~ῳζεσθην", "ἐ~ῳζοντο";
    future_ind_act_1: "~σωσω", _, "~σωσομεν";
    future_ind_act_2: "~σωσεις", "~σωσετον", "~σωσετε";
    future_ind_act_3: _, _, "~σωσουσῐ";
    future_ind_mid_1: "~σωσομαι", _, "~σωσομεθᾰ";
    future_ind_mid_2: "~σωσῃ", "~σωσεσθον", _;
    future_ind_mid_3: "~σωσεται", _, _;
    future_opt_act_1: "~σωσοιμῐ", _, "~σωσοιμεν";
    future_opt_act_2: _, "~σωσοιτον", "~σωσοιτε";
    future_opt_act_3: "~σωσοι", "~σωσοιτην", "~σωσοιεν";
    future_opt_mid_1: "~σωσοιμην", _, "~σωσοιμεθᾰ";
    future_opt_mid_2: "~σωσοιο", _, _;
    future_opt_mid_3: _, "~σωσοισθην", _;
    aorist_subj_act_1: "~σωσω", _, _;
    aorist_subj_act_2: "~σωσῃς", "~σωσητον", _;
    aorist_subj_mid_1: "~σωσωμαι", _, _;
    aorist_subj_mid_2: "~σωσῃ", "~σωσησθον", _;
    aorist_opt_act_1: "~σωσαιμῐ", _, "~σωσαιμεν";
    aorist_opt_act_2: "~σωσειᾰς", _, _;
    aorist_opt_act_3: "~σωσειε", "~σωσαιτην", "~σωσειᾰν";
    aorist_opt_mid_1: "~σωσαιμην", _, _;
    aorist_opt_mid_2: "~σωσαιο", _, _;
    aorist_imp_act_2: "~σωσον", _, "~σωσᾰτε";
    aorist_imp_act_3: _, _, "~σωσᾰντων";
    aorist_imp_mid_2: "~σωσαι", "~σωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~σωσᾰσθων";
    perfect_ind_act_1: "~σεσωκᾰ", _, "~σεσωκᾰμεν";
    perfect_ind_act_2: _, "~σεσωκᾰτον", _;
    perfect_ind_act_3: _, _, "~σεσωκᾱσῐ";
    perfect_ind_mid_1: "~σεσωσμαι", _, "~σεσωσμεθᾰ";
    perfect_ind_mid_2: "~σεσωσαι", "~σεσωσθον", _;
    perfect_ind_mid_3: "~σεσωσται", _, "~σεσωσᾰται";
    perfect_ind_pass_2: _, _, "~σεσωσθε";
    perfect_subj_act_1: "~σεσωκω", _, _;
    perfect_subj_act_2: "~σεσωκῃς", "~σεσωκητον", "~σεσωκητε";
    perfect_subj_act_3: "~σεσωκῃ", _, "~σεσωκωσῐ";
    perfect_opt_act_1: "~σεσωκοιμῐ", _, "~σεσωκοιμεν";
    perfect_opt_act_2: "~σεσωκοις", _, _;
    perfect_opt_act_3: _, "~σεσωκοιτην", _;
    perfect_imp_act_2: "~σεσωκε", _, "~σεσωκετε";
    perfect_imp_act_3: _, _, "~σεσωκοντων";
    perfect_imp_mid_2: _, _, "~σεσωσθε";
    perfect_imp_mid_3: "~σεσωσθω", _, "~σεσωσθων";
    perfect_imp_pass_2: "~σεσωσο", _, "~σεσωσθε";
    perfect_imp_pass_3: _, "~σεσωσθων", "~σεσωσθων";
    infinitive_pres: "~σῳζειν", _, "~σῳζεσθαι";
    infinitive_fut: "~σωσειν", _, _;
    infinitive_aor: "~σωσαι", _, _;
    infinitive_perf: _, "~σεσωσθαι", _;
    participle_pres: "~σῳζων", _, "~σῳζομενος";
    participle_aor: "~σωσᾱς", "~σωσᾰμενος", _;
    participle_perf: "~σεσωκως", _, "~σεσωσμενος";
    ),
    // Κᾰτᾰβῐβᾰζω: 3 words (κᾰτᾰβῐβᾰζω, πᾰρᾰσκευᾰζω, κᾰτᾰσκευᾰζω). Like Δῐᾰρπᾰζω, except in 88 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("κᾰτᾰβῐβᾰζω", 3, "~ᾰ~ζω", like Δῐᾰρπᾰζω;
    stem aorist_ind: "~ε~σ- like εὐλογεω";
    stem aorist_ind_pass: "~ε~σθην- like ἐλπω";
    imperfect_act_1: "~ε~ζον", _, "~ε~ζομεν";
    imperfect_act_2: "~ε~ζες", "~ε~ζετον", "~ε~ζετε";
    imperfect_act_3: "~ε~ζε|~ε~ζεν", "~ε~ζετην", "~ε~ζον";
    imperfect_mid_1: "~ε~ζομην", _, "~ε~ζομεθᾰ";
    imperfect_mid_2: "~ε~ζου", _, _;
    imperfect_mid_3: "~ε~ζετο", _, _;
    imperfect_pass_2: _, "~ε~ζεσθον", "~ε~ζεσθε";
    imperfect_pass_3: _, "~ε~ζεσθην", "~ε~ζοντο";
    future_ind_act_1: "~ᾰ~σω", _, "~ᾰ~σομεν";
    future_ind_act_2: "~ᾰ~σεις", "~ᾰ~σετον", "~ᾰ~σετε";
    future_ind_act_3: _, _, "~ᾰ~σουσῐ|~ᾰ~σουσῐν";
    future_ind_mid_1: "~ᾰ~σομαι", _, "~ᾰ~σομεθᾰ";
    future_ind_mid_2: "~ᾰ~σῃ|~ᾰ~σει", "~ᾰ~σεσθον", _;
    future_ind_mid_3: "~ᾰ~σεται", _, _;
    future_ind_pass_2: derived, _, _;
    future_opt_act_1: "~ᾰ~σοιμῐ", _, "~ᾰ~σοιμεν";
    future_opt_act_2: derived, "~ᾰ~σοιτον", "~ᾰ~σοιτε";
    future_opt_act_3: "~ᾰ~σοι", "~ᾰ~σοιτην", "~ᾰ~σοιεν";
    future_opt_mid_1: "~ᾰ~σοιμην", _, "~ᾰ~σοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~σοιο", _, _;
    future_opt_mid_3: _, "~ᾰ~σοισθην", _;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: _, derived, _;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_mid_1: "~ε~σμαι", _, "~ε~σμεθᾰ";
    perfect_ind_mid_2: "~ε~σαι", "~ε~σθον", _;
    perfect_ind_mid_3: "~ε~σται", _, "~ε~σᾰται";
    perfect_ind_pass_2: _, _, "~ε~σθε";
    perfect_subj_act_1: derived, _, _;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, _, _;
    perfect_opt_act_3: derived, derived, _;
    perfect_imp_act_2: derived, _, derived;
    perfect_imp_act_3: _, _, derived;
    perfect_imp_mid_2: _, _, "~ε~σθε";
    perfect_imp_mid_3: "~ε~σθω", _, "~ε~σθων";
    perfect_imp_pass_2: "~ε~σο", _, "~ε~σθε";
    perfect_imp_pass_3: _, "~ε~σθων", "~ε~σθων";
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
    // Ὠνεομαι: 3 words (ὠνεομαι, θρεομαι, λογεομαι). Like Δηλεομαι, except in 70 cells: ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 sg, ind_pass_2 pl, ….
    conjugation!("ὠνεομαι", 3, "~εομαι", like Δηλεομαι;
    stem aorist_pass: "~ηθ- like ἐρωτᾰω";
    ind_mid_1: "~εομαι", _, "~εομεθᾰ";
    ind_mid_2: "~εῃ", "~εεσθον", _;
    ind_pass_2: derived, _, "~εεσθε";
    ind_pass_3: "~εεται", _, "~εονται";
    subj_mid_1: "~εωμαι", _, "~εωμεθᾰ";
    subj_mid_2: "~εῃ", "~εησθον", _;
    subj_mid_3: "~εηται", _, "~εωνται";
    subj_pass_2: "~εῃ", _, "~εησθε";
    opt_mid_1: "~εοιμην", _, "~εοιμεθᾰ";
    opt_mid_2: "~εοιο", "~εοισθον", _;
    opt_mid_3: "~εοιτο", "~εοισθην", "~εοιντο";
    opt_pass_2: _, _, "~εοισθε";
    imp_mid_2: "~εου", _, "~εεσθε";
    imp_mid_3: "~εεσθω", _, "~εεσθων";
    imp_pass_2: _, _, "~εεσθε";
    imp_pass_3: _, "~εεσθων", "~εεσθων";
    imperfect_mid_1: "ἐ~εομην", _, "ἐ~εομεθᾰ";
    imperfect_mid_2: "ἐ~εου", "ἐ~εεσθον", _;
    imperfect_mid_3: "ἐ~εετο", "ἐ~εεσθην", "ἐ~εοντο";
    imperfect_pass_2: _, _, "ἐ~εεσθε";
    future_ind_mid_2: "~ησῃ", _, _;
    aorist_subj_mid_1: "ἐ~ησωμαι", _, _;
    aorist_subj_mid_2: "ἐ~ησῃ", "ἐ~ησησθον", _;
    aorist_opt_mid_1: "ἐ~ησαιμην", _, _;
    aorist_opt_mid_2: "ἐ~ησαιο", _, _;
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
    // Κᾰτᾰπονεω: 3 words (ὑπᾰνᾰχωρεω, ἐπᾰνᾰχωρεω, κᾰτᾰπονεω). Like Κατοικεω, except in 66 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("κᾰτᾰπονεω", 3, "~ᾰ~εω", like Κατοικεω;
    stem aorist_ind: "~ε~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ε~ηθην- like ἐλπω";
    imperfect_act_1: "~ε~ουν", _, "~ε~οῡμεν";
    imperfect_act_2: "~ε~εις", "~ε~εῑτον", "~ε~εῑτε";
    imperfect_act_3: "~ε~ει", "~ε~ειτην", "~ε~ουν";
    imperfect_mid_1: "~ε~ουμην", _, "~ε~ουμεθᾰ";
    imperfect_mid_2: "~ε~οῡ", _, _;
    imperfect_mid_3: "~ε~εῑτο", _, _;
    imperfect_pass_2: _, "~ε~εῑσθον", "~ε~εῑσθε";
    imperfect_pass_3: _, "~ε~εισθην", "~ε~οῡντο";
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ|~ᾰ~ησουσῐν";
    aorist_subj_act_3: _, _, "~ᾰ~ησωσῐ|~ᾰ~ησωσῐν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς|~ᾰ~ησαις", _, _;
    aorist_opt_act_3: "~ᾰ~ησειε|~ᾰ~ησειεν|~ᾰ~ησαι", _, "~ᾰ~ησειᾰν|~ᾰ~ησαιεν";
    perfect_ind_act_1: "~ᾰ%ε~ηκᾰ", _, "~ᾰ%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "~ᾰ%ε~ηκᾰτον", _;
    perfect_ind_act_3: "~ᾰ%ε~ηκε|~ᾰ%ε~ηκεν", _, "~ᾰ%ε~ηκᾱσῐ|~ᾰ%ε~ηκᾱσῐν";
    perfect_ind_mid_1: "~ᾰ%ε~ημαι", _, "~ᾰ%ε~ημεθᾰ";
    perfect_ind_mid_2: "~ᾰ%ε~ησαι", "~ᾰ%ε~ησθον", _;
    perfect_ind_mid_3: "~ᾰ%ε~ηται", _, "~ᾰ%ε~ηνται";
    perfect_ind_pass_2: _, _, "~ᾰ%ε~ησθε";
    perfect_subj_act_1: "~ᾰ%ε~ηκω", _, _;
    perfect_subj_act_2: "~ᾰ%ε~ηκῃς", "~ᾰ%ε~ηκητον", "~ᾰ%ε~ηκητε";
    perfect_subj_act_3: "~ᾰ%ε~ηκῃ", _, "~ᾰ%ε~ηκωσῐ|~ᾰ%ε~ηκωσῐν";
    perfect_opt_act_1: "~ᾰ%ε~ηκοιμῐ|~ᾰ%ε~ηκοιην", _, "~ᾰ%ε~ηκοιμεν";
    perfect_opt_act_2: "~ᾰ%ε~ηκοις|~ᾰ%ε~ηκοιης", _, _;
    perfect_opt_act_3: "~ᾰ%ε~ηκοι|~ᾰ%ε~ηκοιη", "~ᾰ%ε~ηκοιτην", _;
    perfect_imp_act_2: "~ᾰ%ε~ηκε", _, "~ᾰ%ε~ηκετε";
    perfect_imp_act_3: _, _, "~ᾰ%ε~ηκοντων";
    perfect_imp_mid_2: _, _, "~ᾰ%ε~ησθε";
    perfect_imp_mid_3: "~ᾰ%ε~ησθω", _, "~ᾰ%ε~ησθων";
    perfect_imp_pass_2: "~ᾰ%ε~ησο", _, "~ᾰ%ε~ησθε";
    perfect_imp_pass_3: _, "~ᾰ%ε~ησθων", "~ᾰ%ε~ησθων";
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
    // Ἐπορον: 2 words (ἐφᾰγον, ἐπορον). Like Μῑγνῡμῐ, except in 24 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, ….
    conjugation!("ἐπορον", 2, "ἐ~ον", like Μῑγνῡμῐ;
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
    aorist_opt_act_3: "~οι", "~οιτην", "~οιεν";
    aorist_imp_act_2: "~ε", _, "~ετε";
    aorist_imp_act_3: _, _, "~οντων";
    infinitive_aor: "~εῑν", _, _;
    participle_aor: "~ων", _, _;
    ),
    // Εἰδον: 2 words (εἰπον, εἰδον). Like Ἐπορον, except in 33 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("εἰδον", 2, "ε~ον", like Ἐπορον;
    stem aorist_ind: "~- like εἰδον";
    ind_act_1: "ε~ον", _, _;
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", "~ησετε";
    future_ind_act_3: _, _, "~ησουσῐ";
    future_ind_mid_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_mid_2: "~ησῃ", "~ησεσθον", _;
    future_ind_mid_3: "~ησεται", _, _;
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: _, "~ησοιτον", "~ησοιτε";
    future_opt_act_3: "~ησοι", "~ησοιτην", "~ησοιεν";
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_2: "~ησοιο", _, _;
    future_opt_mid_3: _, "~ησοισθην", _;
    aorist_subj_mid_1: "~ωμαι", _, _;
    aorist_subj_mid_2: "~ῃ", "~ησθον", _;
    aorist_opt_mid_1: "~οιμην", _, _;
    aorist_opt_mid_2: "~οιο", _, _;
    aorist_imp_mid_2: "~οῡ", "~εσθον", _;
    aorist_imp_mid_3: _, _, "~εσθων";
    infinitive_fut: "~ησειν", _, _;
    participle_aor: _, "~ομενος", _;
    ),
    // Καινῠμαι: 2 words (πτᾰρνῠμαι, καινῠμαι). Like Ἐπορον, except in 33 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("καινῠμαι", 2, "~νῠμαι", like Ἐπορον;
    ind_act_1: "~νῠμαι", _, _;
    ind_mid_1: "~νῠμαι", _, "~νῠμεθᾰ";
    ind_mid_2: "~νῠσαι", "~νῠσθον", _;
    ind_pass_2: _, _, "~νῠσθε";
    ind_pass_3: "~νῠται", _, "~νῠνται";
    subj_mid_1: "~νῠωμαι", _, "~νῠωμεθᾰ";
    subj_mid_2: "~νῠῃ", "~νῠησθον", _;
    subj_mid_3: "~νῠηται", _, "~νῠωνται";
    subj_pass_2: "~νῠῃ", _, "~νῠησθε";
    opt_mid_1: "~νῠοιμην", _, "~νῠοιμεθᾰ";
    opt_mid_2: "~νῠοιο", "~νῠοισθον", _;
    opt_mid_3: "~νῠοιτο", "~νῠοισθην", "~νῠοιντο";
    opt_pass_2: _, _, "~νῠοισθε";
    imp_mid_2: "~νῠσο", _, "~νῠσθε";
    imp_mid_3: "~νῠσθω", _, "~νῠσθων";
    imp_pass_2: _, _, "~νῠσθε";
    imp_pass_3: _, "~νῠσθων", "~νῠσθων";
    infinitive_pres: _, _, "~νῠσθαι";
    participle_pres: _, _, "~νῠμενος";
    ),
    // Ξυμπολεμεω: 2 words (συμπολεμεω, ξυμπολεμεω). Like Ἐγκειμαι, except in 138 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ξυμπολεμεω", 2, "~υμ~εω", like Ἐγκειμαι;
    stem imperfect: "~ῠνε~- like ξυνναυμᾰχεω";
    stem future_pass: "~ῠμ~ηθησ- like ποιεω";
    stem aorist_ind: "~ῠνε~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ῠνε~ηθην- like ἐλπω";
    stem aorist_pass: "~ῠμ~ηθ- like ποιεω";
    ind_act_1: "~ῠμ~ω", _, "~ῠμ~οῡμεν";
    ind_act_2: "~ῠμ~εῑς", "~ῠμ~εῑτον", _;
    ind_act_3: "~ῠμ~εῑ", _, "~ῠμ~οῡσῐ|~ῠμ~οῡσῐν";
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, derived, _;
    ind_mid_3: _, _, derived;
    subj_act_1: "~ῠμ~ω", _, "~ῠμ~ωμεν";
    subj_act_2: "~ῠμ~ῃς", "~ῠμ~ητον", _;
    subj_act_3: "~ῠμ~ῃ", _, "~ῠμ~ωσῐ|~ῠμ~ωσῐν";
    subj_mid_1: derived, _, derived;
    subj_mid_2: derived, derived, _;
    subj_mid_3: derived, _, derived;
    opt_act_1: "~ῠμ~οιην|~ῠμ~οῑμῐ", _, "~ῠμ~οῑμεν|~ῠμ~οιημεν";
    opt_act_2: "~ῠμ~οιης|~ῠμ~οῑς", "~ῠμ~οῑτον|~ῠμ~οιητον", "~ῠμ~οῑτε|~ῠμ~οιητε";
    opt_act_3: "~ῠμ~οιη|~ῠμ~οῑ", "~ῠμ~οιτην|~ῠμ~οιητην", "~ῠμ~οῑεν|~ῠμ~οιησᾰν";
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, _;
    opt_mid_3: derived, derived, derived;
    imp_act_2: "~ῠμ~ει", _, "~ῠμ~εῑτε";
    imp_act_3: "~ῠμ~ειτω", _, "~ῠμ~ουντων";
    imp_mid_2: derived, _, derived;
    imp_mid_3: derived, _, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, _;
    imperfect_mid_3: derived, derived, derived;
    future_ind_act_1: "~ῠμ~ησω", _, "~ῠμ~ησομεν";
    future_ind_act_2: "~ῠμ~ησεις", "~ῠμ~ησετον", "~ῠμ~ησετε";
    future_ind_act_3: _, _, "~ῠμ~ησουσῐ|~ῠμ~ησουσῐν";
    future_ind_mid_1: "~ῠμ~ησομαι", _, "~ῠμ~ησομεθᾰ";
    future_ind_mid_2: "~ῠμ~ησῃ|~ῠμ~ησει", "~ῠμ~ησεσθον", _;
    future_ind_mid_3: "~ῠμ~ησεται", _, _;
    future_opt_act_1: "~ῠμ~ησοιμῐ", _, "~ῠμ~ησοιμεν";
    future_opt_act_2: _, "~ῠμ~ησοιτον", "~ῠμ~ησοιτε";
    future_opt_act_3: "~ῠμ~ησοι", "~ῠμ~ησοιτην", "~ῠμ~ησοιεν";
    future_opt_mid_1: "~ῠμ~ησοιμην", _, "~ῠμ~ησοιμεθᾰ";
    future_opt_mid_2: "~ῠμ~ησοιο", _, _;
    future_opt_mid_3: _, "~ῠμ~ησοισθην", _;
    aorist_subj_act_1: "~ῠμ~ησω", _, _;
    aorist_subj_act_2: "~ῠμ~ησῃς", "~ῠμ~ησητον", _;
    aorist_subj_act_3: _, _, "~ῠμ~ησωσῐ|~ῠμ~ησωσῐν";
    aorist_subj_mid_1: "~ῠμ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ῠμ~ησῃ", "~ῠμ~ησησθον", _;
    aorist_opt_act_1: "~ῠμ~ησαιμῐ", _, "~ῠμ~ησαιμεν";
    aorist_opt_act_2: "~ῠμ~ησειᾰς|~ῠμ~ησαις", _, _;
    aorist_opt_act_3: "~ῠμ~ησειε|~ῠμ~ησειεν|~ῠμ~ησαι", "~ῠμ~ησαιτην", "~ῠμ~ησειᾰν|~ῠμ~ησαιεν";
    aorist_opt_mid_1: "~ῠμ~ησαιμην", _, _;
    aorist_opt_mid_2: "~ῠμ~ησαιο", _, _;
    aorist_imp_act_2: "~ῠμ~ησον", _, "~ῠμ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῠμ~ησᾰντων";
    aorist_imp_mid_2: "~ῠμ~ησαι", "~ῠμ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῠμ~ησᾰσθων";
    perfect_ind_act_1: "~ῠμ%ε~ηκᾰ", _, "~ῠμ%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "~ῠμ%ε~ηκᾰτον", _;
    perfect_ind_act_3: "~ῠμ%ε~ηκε|~ῠμ%ε~ηκεν", _, "~ῠμ%ε~ηκᾱσῐ|~ῠμ%ε~ηκᾱσῐν";
    perfect_ind_mid_1: "~ῠμ%ε~ημαι", _, "~ῠμ%ε~ημεθᾰ";
    perfect_ind_mid_2: "~ῠμ%ε~ησαι", "~ῠμ%ε~ησθον", _;
    perfect_ind_mid_3: "~ῠμ%ε~ηται", _, "~ῠμ%ε~ηνται";
    perfect_ind_pass_2: _, _, "~ῠμ%ε~ησθε";
    perfect_subj_act_1: "~ῠμ%ε~ηκω", _, _;
    perfect_subj_act_2: "~ῠμ%ε~ηκῃς", "~ῠμ%ε~ηκητον", "~ῠμ%ε~ηκητε";
    perfect_subj_act_3: "~ῠμ%ε~ηκῃ", _, "~ῠμ%ε~ηκωσῐ|~ῠμ%ε~ηκωσῐν";
    perfect_opt_act_1: "~ῠμ%ε~ηκοιμῐ|~ῠμ%ε~ηκοιην", _, "~ῠμ%ε~ηκοιμεν";
    perfect_opt_act_2: "~ῠμ%ε~ηκοις|~ῠμ%ε~ηκοιης", _, _;
    perfect_opt_act_3: "~ῠμ%ε~ηκοι|~ῠμ%ε~ηκοιη", "~ῠμ%ε~ηκοιτην", _;
    perfect_imp_act_2: "~ῠμ%ε~ηκε", _, "~ῠμ%ε~ηκετε";
    perfect_imp_act_3: _, _, "~ῠμ%ε~ηκοντων";
    perfect_imp_mid_2: _, _, "~ῠμ%ε~ησθε";
    perfect_imp_mid_3: "~ῠμ%ε~ησθω", _, "~ῠμ%ε~ησθων";
    perfect_imp_pass_2: "~ῠμ%ε~ησο", _, "~ῠμ%ε~ησθε";
    perfect_imp_pass_3: _, "~ῠμ%ε~ησθων", "~ῠμ%ε~ησθων";
    infinitive_pres: "~ῠμ~εῑν", _, _;
    infinitive_fut: "~ῠμ~ησειν", _, _;
    infinitive_aor: "~ῠμ~ησαι", _, _;
    infinitive_perf: _, "~ῠμ%ε~ησθαι", _;
    participle_pres: "~ῠμ~ων", _, _;
    participle_aor: "~ῠμ~ησᾱς", "~ῠμ~ησᾰμενος", _;
    participle_perf: "~ῠμ%ε~ηκως", _, "~ῠμ%ε~ημενος";
    ),
    // Δῐακειμαι: 2 words (κᾰτακειμαι, δῐακειμαι). Like Ἐγκειμαι, except in 32 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_3 pl, ….
    conjugation!("δῐακειμαι", 2, "~α~ειμαι", like Ἐγκειμαι;
    stem imperfect: "~ε~ει- like δῐακειμαι";
    ind_act_1: "~α~ειμαι", _, _;
    ind_mid_1: "~ᾰ~ειμαι", _, "~ᾰ~ειμεθα";
    ind_mid_2: "~ᾰ~εισαι", "~ᾰ~εισθον", _;
    ind_mid_3: _, _, derived;
    subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    subj_mid_2: "~ᾰ~εῃ", "~ᾰ~εησθον", _;
    subj_mid_3: "~ᾰ~εηται", _, "~ᾰ~εωνται";
    opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~οῑο", "~ᾰ~οῑσθον", _;
    opt_mid_3: "~ᾰ~οῑτο", "~ᾰ~οισθην", "~ᾰ~οῑντο";
    imp_mid_2: "~ᾰ~εισο", _, "~ᾰ~εισθον";
    imp_mid_3: "~ᾰ~εισθω", _, "~ᾰ~εισθων";
    future_ind_mid_1: "~ᾰ~εισομαι", _, "~ᾰ~εισομεθᾰ";
    future_ind_mid_2: "~ᾰ~εισῃ", "~ᾰ~εισεσθον", _;
    future_ind_mid_3: "~ᾰ~εισεται", _, _;
    future_opt_mid_1: "~ᾰ~εισοιμην", _, "~ᾰ~εισοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~εισοιο", _, _;
    future_opt_mid_3: _, "~ᾰ~εισοισθην", _;
    ),
    // Ξῠνοιδᾰ: 2 words (ξῠνοιδᾰ, προοιδᾰ). Like Μῑγνῡμῐ, except in 35 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ξῠνοιδᾰ", 2, "~οιδᾰ", like Μῑγνῡμῐ;
    stem imperfect: "~ῃ- like ξῠνοιδᾰ";
    ind_act_1: "~οιδᾰ", _, "~ϊσμεν";
    ind_act_2: "~οισθᾰ|~οιδᾰς", "~ϊστον", _;
    ind_act_3: "~οιδε", _, "~ϊσᾱσῐ";
    subj_act_1: "~ειδω", _, "~ειδωμεν";
    subj_act_2: "~ειδῃς", "~ειδητον", _;
    subj_act_3: "~ειδῃ", _, "~ειδωσῐ|~ειδωσῐν";
    opt_act_1: "~ειδειην", _, "~ειδεῑμεν|~ειδειημεν";
    opt_act_2: "~ειδειης", "~ειδεῑτον", "~ειδεῑτε|~ειδειητε";
    opt_act_3: "~ειδειη", "~ειδειτην", "~ειδεῑεν|~ειδειησᾰν";
    imp_act_2: "~ϊσθῐ", _, "~ϊστε";
    imp_act_3: "~ϊστω", _, "~ϊστων";
    future_ind_mid_1: "~εισομαι", _, "~εισομεθᾰ";
    future_ind_mid_2: "~εισῃ|~εισει", "~εισεσθον", _;
    future_ind_mid_3: "~εισεται", _, _;
    future_opt_mid_1: "~εισοιμην", _, "~εισοιμεθᾰ";
    future_opt_mid_2: "~εισοιο", _, _;
    future_opt_mid_3: _, "~εισοισθην", _;
    infinitive_pres: "~ειδεναι", _, _;
    participle_pres: "~ειδως", _, _;
    ),
    // Νισσομαι: 2 words (κορινθιαζομαι, νισσομαι). Like Δῐακειμαι, except in 49 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("νισσομαι", 2, "~ι~ομαι", like Δῐακειμαι;
    ind_act_1: "~ι~ομαι", _, _;
    ind_mid_1: "~ῐ~ομαι", _, "~ῐ~ομεθᾰ";
    ind_mid_2: "~ῐ~ῃ", "~ῐ~εσθον", _;
    ind_pass_2: _, _, "~ῐ~εσθε";
    ind_pass_3: "~ῐ~εται", _, "~ῐ~ονται";
    subj_mid_1: "~ῐ~ωμαι", _, "~ῐ~ωμεθᾰ";
    subj_mid_2: "~ῐ~ῃ", "~ῐ~ησθον", _;
    subj_mid_3: "~ῐ~ηται", _, "~ῐ~ωνται";
    subj_pass_2: "~ῐ~ῃ", _, "~ῐ~ησθε";
    opt_mid_1: "~ῐ~οιμην", _, "~ῐ~οιμεθᾰ";
    opt_mid_2: "~ῐ~οιο", "~ῐ~οισθον", _;
    opt_mid_3: "~ῐ~οιτο", "~ῐ~οισθην", "~ῐ~οιντο";
    opt_pass_2: _, _, "~ῐ~οισθε";
    imp_mid_2: "~ῐ~ου", _, "~ῐ~εσθε";
    imp_mid_3: "~ῐ~εσθω", _, "~ῐ~εσθων";
    imp_pass_2: _, _, "~ῐ~εσθε";
    imp_pass_3: _, "~ῐ~εσθων", "~ῐ~εσθων";
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, _;
    imperfect_mid_3: derived, derived, derived;
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, _, _;
    future_opt_mid_3: _, derived, _;
    infinitive_pres: _, _, "~ῐ~εσθαι";
    participle_pres: _, _, "~ῐ~ομενος";
    ),
    // Κᾰτασκηνοω: 2 words (στεφανοω, κᾰτασκηνοω). Like Κατοικεω, except in 129 cells: ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("κᾰτασκηνοω", 2, "~α~οω", like Κατοικεω;
    stem future_pass: "~ᾰ~ωθησ- like ἐχω";
    stem aorist_ind: "~ᾰ~ωσ- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~ωθην- like ἐχω";
    stem aorist_pass: "~ᾰ~ωθ- like ἐχω";
    stem pluperfect_act: "~ᾰ~ωκειν- like κᾰλεω";
    ind_act_2: "~ᾰ~οῑς", _, _;
    ind_act_3: "~ᾰ~οῑ", "~ᾰ~οῡτον", _;
    ind_mid_2: "~ᾰ~οῑ", "~ᾰ~οῡσθον", _;
    ind_pass_2: _, _, "~ᾰ~οῡσθε";
    ind_pass_3: "~ᾰ~οῡται", _, _;
    subj_act_2: "~ᾰ~οῑς", _, _;
    subj_act_3: "~ᾰ~οῑ", "~ᾰ~ωτον", _;
    subj_mid_2: _, "~ᾰ~ωσθον", _;
    subj_mid_3: "~ᾰ~ωται", _, _;
    subj_pass_2: _, _, "~ᾰ~ωσθε";
    imp_act_2: "~ᾰ~ου", "~ᾰ~οῡτον", "~ᾰ~οῡτε";
    imp_mid_2: _, _, "~ᾰ~οῡσθε";
    imp_mid_3: "~ᾰ~ουσθω", _, "~ᾰ~ουσθων";
    imp_pass_2: _, _, "~ᾰ~οῡσθε";
    imp_pass_3: _, "~ᾰ~ουσθων", "~ᾰ~ουσθων";
    imperfect_act_1: "ἐ~ᾰ~ουν", _, "ἐ~ᾰ~οῡμεν";
    imperfect_act_2: "ἐ~ᾰ~ους", "ἐ~ᾰ~οῡτον", "ἐ~ᾰ~οῡτε";
    imperfect_act_3: "ἐ~ᾰ~ου", "ἐ~ᾰ~ουτην", "ἐ~ᾰ~ουν";
    imperfect_mid_1: "ἐ~ᾰ~ουμην", _, "ἐ~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~οῡ", _, _;
    imperfect_mid_3: "ἐ~ᾰ~οῡτο", _, _;
    imperfect_pass_2: _, "ἐ~ᾰ~οῡσθον", "ἐ~ᾰ~οῡσθε";
    imperfect_pass_3: _, "ἐ~ᾰ~ουσθην", "ἐ~ᾰ~οῡντο";
    future_ind_act_1: "~ᾰ~ωσω", _, "~ᾰ~ωσομεν";
    future_ind_act_2: "~ᾰ~ωσεις", "~ᾰ~ωσετον", "~ᾰ~ωσετε";
    future_ind_act_3: _, _, "~ᾰ~ωσουσῐ|~ᾰ~ωσουσῐν";
    future_ind_mid_1: "~ᾰ~ωσομαι", _, "~ᾰ~ωσομεθᾰ";
    future_ind_mid_2: "~ᾰ~ωσῃ", "~ᾰ~ωσεσθον", _;
    future_ind_mid_3: "~ᾰ~ωσεται", _, _;
    future_opt_act_1: "~ᾰ~ωσοιμῐ", _, "~ᾰ~ωσοιμεν";
    future_opt_act_2: _, "~ᾰ~ωσοιτον", "~ᾰ~ωσοιτε";
    future_opt_act_3: "~ᾰ~ωσοι", "~ᾰ~ωσοιτην", "~ᾰ~ωσοιεν";
    future_opt_mid_1: "~ᾰ~ωσοιμην", _, "~ᾰ~ωσοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ωσοιο", _, _;
    future_opt_mid_3: _, "~ᾰ~ωσοισθην", _;
    aorist_subj_act_1: "~ᾰ~ωσω", _, _;
    aorist_subj_act_2: "~ᾰ~ωσῃς", "~ᾰ~ωσητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~ωσωσῐ|~ᾰ~ωσωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ωσῃ", "~ᾰ~ωσησθον", _;
    aorist_opt_act_1: "~ᾰ~ωσαιμῐ", _, "~ᾰ~ωσαιμεν";
    aorist_opt_act_2: "~ᾰ~ωσειᾰς|~ᾰ~ωσαις", _, _;
    aorist_opt_act_3: "~ᾰ~ωσειε|~ᾰ~ωσειεν|~ᾰ~ωσαι", "~ᾰ~ωσαιτην", "~ᾰ~ωσειᾰν|~ᾰ~ωσαιεν";
    aorist_opt_mid_1: "~ᾰ~ωσαιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~ωσαιο", _, _;
    aorist_imp_act_2: "~ᾰ~ωσον", _, "~ᾰ~ωσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ωσᾰντων";
    aorist_imp_mid_2: "~ᾰ~ωσαι", "~ᾰ~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ωσᾰσθων";
    perfect_ind_act_1: "ἐ~ᾰ~ωκᾰ", _, "ἐ~ᾰ~ωκᾰμεν";
    perfect_ind_act_2: _, "ἐ~ᾰ~ωκᾰτον", _;
    perfect_ind_act_3: _, _, "ἐ~ᾰ~ωκᾱσῐ";
    perfect_ind_mid_1: "ἐ~ᾰ~ωμαι", _, "ἐ~ᾰ~ωμεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰ~ωσαι", "ἐ~ᾰ~ωσθον", _;
    perfect_ind_mid_3: "ἐ~ᾰ~ωται", _, "ἐ~ᾰ~ωνται";
    perfect_ind_pass_2: _, _, "ἐ~ᾰ~ωσθε";
    perfect_subj_act_1: "ἐ~ᾰ~ωκω", _, _;
    perfect_subj_act_2: "ἐ~ᾰ~ωκῃς", "ἐ~ᾰ~ωκητον", "ἐ~ᾰ~ωκητε";
    perfect_subj_act_3: "ἐ~ᾰ~ωκῃ", _, "ἐ~ᾰ~ωκωσῐ";
    perfect_opt_act_1: "ἐ~ᾰ~ωκοιμῐ", _, "ἐ~ᾰ~ωκοιμεν";
    perfect_opt_act_2: "ἐ~ᾰ~ωκοις", _, _;
    perfect_opt_act_3: _, "ἐ~ᾰ~ωκοιτην", _;
    perfect_imp_act_2: "ἐ~ᾰ~ωκε", _, "ἐ~ᾰ~ωκετε";
    perfect_imp_act_3: _, _, "ἐ~ᾰ~ωκοντων";
    perfect_imp_mid_2: _, _, "ἐ~ᾰ~ωσθε";
    perfect_imp_mid_3: "ἐ~ᾰ~ωσθω", _, "ἐ~ᾰ~ωσθων";
    perfect_imp_pass_2: "ἐ~ᾰ~ωσο", _, "ἐ~ᾰ~ωσθε";
    perfect_imp_pass_3: _, "ἐ~ᾰ~ωσθων", "ἐ~ᾰ~ωσθων";
    pluperfect_mid_1: "ἐ~ᾰ~ωμην", _, _;
    pluperfect_mid_2: "ἐ~ᾰ~ωσο", "ἐ~ᾰ~ωσθον", _;
    pluperfect_mid_3: "ἐ~ᾰ~ωτο", _, "ἐ~ᾰ~ωντο";
    pluperfect_pass_1: _, _, "ἐ~ᾰ~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ᾰ~ωσθε";
    pluperfect_pass_3: _, "ἐ~ᾰ~ωσθην", _;
    infinitive_pres: "~ᾰ~οῡν", _, "~ᾰ~οῡσθαι";
    infinitive_fut: "~ᾰ~ωσειν", _, _;
    infinitive_aor: "~ᾰ~ωσαι", _, _;
    infinitive_perf: _, "ἐ~ᾰ~ωσθαι", _;
    participle_aor: "~ᾰ~ωσᾱς", "~ᾰ~ωσᾰμενος", _;
    participle_perf: "ἐ~ᾰ~ωκως", _, "ἐ~ᾰ~ωμενος";
    ),
    // Γεγονοι: 2 words (γεγονοι, τεθνηκοι). Like Μῑγνῡμῐ, except in 22 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, ….
    conjugation!("γεγονοι", 2, "~οι", like Μῑγνῡμῐ;
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
    perfect_opt_act_1: "~οιμῐ", _, "~οιμεν";
    perfect_opt_act_2: "~οις", _, _;
    perfect_opt_act_3: _, "~οιτην", _;
    perfect_imp_act_2: "~ε", _, "~ετε";
    perfect_imp_act_3: _, _, "~οντων";
    participle_perf: "~ως", _, _;
    ),
    // Ἐπῐχειρεω: 2 words (ἀρῐθμεω, ἐπῐχειρεω). Like Κᾰτᾰπονεω, except in 147 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐπῐχειρεω", 2, "~ῐ~εω", like Κᾰτᾰπονεω;
    stem future_pass: "~ι~ηθησ- like ἐχω";
    stem aorist_ind: "~ε~ησ- like ἐλπω";
    stem aorist_pass: "~ι~ηθ- like ἐχω";
    ind_act_1: "~ι~ω", _, "~ι~οῡμεν";
    ind_act_2: "~ι~εῑς", _, _;
    ind_act_3: "~ι~εῑ", "~ι~εῑτον", "~ι~οῡσῐ";
    ind_mid_1: "~ι~οῡμαι", _, "~ι~ουμεθᾰ";
    ind_mid_2: "~ι~εῑ", "~ι~εῑσθον", _;
    ind_pass_2: _, _, "~ι~εῑσθε";
    ind_pass_3: "~ι~εῑται", _, "~ι~οῡνται";
    subj_act_1: "~ι~ω", _, "~ι~ωμεν";
    subj_act_2: "~ι~ῃς", _, _;
    subj_act_3: "~ι~ῃ", "~ι~ητον", "~ι~ωσῐ";
    subj_mid_1: _, _, "~ι~ωμεθᾰ";
    subj_mid_2: _, "~ι~ησθον", _;
    subj_mid_3: "~ι~ηται", _, "~ι~ωνται";
    subj_pass_1: "~ι~ωμαι", _, _;
    subj_pass_2: _, _, "~ι~ησθε";
    opt_act_1: "~ι~οιην", _, "~ι~οῑμεν";
    opt_act_2: "~ι~οιης", "~ι~οῑτον", derived;
    opt_act_3: "~ι~οιη", "~ι~οιτην", "~ι~οῑεν";
    opt_mid_1: "~ι~οιμην", _, "~ι~οιμεθᾰ";
    opt_mid_2: "~ι~οῑο", "~ι~οῑσθον", _;
    opt_mid_3: "~ι~οῑτο", "~ι~οισθην", "~ι~οῑντο";
    opt_pass_2: _, _, "~ι~οῑσθε";
    imp_act_2: "~ι~ει", "~ι~εῑτον", "~ι~εῑτε";
    imp_mid_2: "~ι~οῡ", _, "~ι~εῑσθε";
    imp_mid_3: "~ι~εισθω", _, "~ι~εισθων";
    imp_pass_2: _, _, "~ι~εῑσθε";
    imp_pass_3: _, "~ι~εισθων", "~ι~εισθων";
    future_ind_act_1: "~ι~ησω", _, "~ι~ησομεν";
    future_ind_act_2: "~ι~ησεις", "~ι~ησετον", "~ι~ησετε";
    future_ind_act_3: _, _, "~ι~ησουσῐ";
    future_ind_mid_1: "~ι~ησομαι", _, "~ι~ησομεθᾰ";
    future_ind_mid_2: "~ι~ησῃ", "~ι~ησεσθον", _;
    future_ind_mid_3: "~ι~ησεται", _, _;
    future_opt_act_1: "~ι~ησοιμῐ", _, "~ι~ησοιμεν";
    future_opt_act_2: _, "~ι~ησοιτον", "~ι~ησοιτε";
    future_opt_act_3: "~ι~ησοι", "~ι~ησοιτην", "~ι~ησοιεν";
    future_opt_mid_1: "~ι~ησοιμην", _, "~ι~ησοιμεθᾰ";
    future_opt_mid_2: "~ι~ησοιο", _, _;
    future_opt_mid_3: _, "~ι~ησοισθην", _;
    aorist_subj_act_1: "~ι~ησω", _, _;
    aorist_subj_act_2: "~ι~ησῃς", "~ι~ησητον", _;
    aorist_subj_act_3: _, _, derived;
    aorist_subj_mid_1: "~ι~ησωμαι", _, _;
    aorist_subj_mid_2: "~ι~ησῃ", "~ι~ησησθον", _;
    aorist_opt_act_1: "~ι~ησαιμῐ", _, "~ι~ησαιμεν";
    aorist_opt_act_2: "~ι~ησειᾰς", _, _;
    aorist_opt_act_3: "~ι~ησειε", "~ι~ησαιτην", "~ι~ησειᾰν";
    aorist_opt_mid_1: "~ι~ησαιμην", _, _;
    aorist_opt_mid_2: "~ι~ησαιο", _, _;
    aorist_imp_act_2: "~ι~ησον", _, "~ι~ησᾰτε";
    aorist_imp_act_3: _, _, "~ι~ησᾰντων";
    aorist_imp_mid_2: "~ι~ησαι", "~ι~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ι~ησᾰσθων";
    perfect_ind_act_1: "~ε~ηκᾰ", _, "~ε~ηκᾰμεν";
    perfect_ind_act_2: _, "~ε~ηκᾰτον", _;
    perfect_ind_act_3: derived, _, "~ε~ηκᾱσῐ";
    perfect_ind_mid_1: "~ε~ημαι", _, "~ε~ημεθᾰ";
    perfect_ind_mid_2: "~ε~ησαι", "~ε~ησθον", _;
    perfect_ind_mid_3: "~ε~ηται", _, "~ε~ηνται";
    perfect_ind_pass_2: _, _, "~ε~ησθε";
    perfect_subj_act_1: "~ε~ηκω", _, _;
    perfect_subj_act_2: "~ε~ηκῃς", "~ε~ηκητον", "~ε~ηκητε";
    perfect_subj_act_3: "~ε~ηκῃ", _, "~ε~ηκωσῐ";
    perfect_opt_act_1: "~ε~ηκοιμῐ", _, "~ε~ηκοιμεν";
    perfect_opt_act_2: "~ε~ηκοις", _, _;
    perfect_opt_act_3: derived, "~ε~ηκοιτην", _;
    perfect_imp_act_2: "~ε~ηκε", _, "~ε~ηκετε";
    perfect_imp_act_3: _, _, "~ε~ηκοντων";
    perfect_imp_mid_2: _, _, "~ε~ησθε";
    perfect_imp_mid_3: "~ε~ησθω", _, "~ε~ησθων";
    perfect_imp_pass_2: "~ε~ησο", _, "~ε~ησθε";
    perfect_imp_pass_3: _, "~ε~ησθων", "~ε~ησθων";
    pluperfect_act_1: "~ε~ηκειν", _, _;
    pluperfect_mid_1: "~ε~ημην", _, _;
    pluperfect_mid_2: "~ε~ησο", "~ε~ησθον", _;
    pluperfect_mid_3: "~ε~ητο", _, "~ε~ηντο";
    pluperfect_pass_1: _, _, "~ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ε~ησθε";
    pluperfect_pass_3: _, "~ε~ησθην", _;
    infinitive_pres: "~ι~εῑν", _, "~ι~εῑσθαι";
    infinitive_fut: "~ι~ησειν", _, _;
    infinitive_aor: "~ι~ησαι", _, _;
    infinitive_perf: _, "~ε~ησθαι", _;
    participle_pres: "~ι~ων", _, "~ι~ουμενος";
    participle_aor: "~ι~ησᾱς", "~ι~ησᾰμενος", _;
    participle_perf: "~ε~ηκως", _, "~ε~ημενος";
    ),
    // Συνεπομαι: 2 words (συνεπομαι, ξυνεπομαι). Like Ἐρχομαι, except in 59 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("συνεπομαι", 2, "~υνεπομαι", like Ἐρχομαι;
    stem aorist_ind: "~ῠνεσπ- like ἐρχομαι";
    ind_act_1: "~υνεπομαι", _, _;
    ind_mid_1: "~ῠνεπομαι", _, "~ῠνεπομεθᾰ";
    ind_mid_2: "~ῠνεπῃ|~ῠνεπει", "~ῠνεπεσθον", _;
    ind_pass_2: _, _, "~ῠνεπεσθε";
    ind_pass_3: "~ῠνεπεται", _, "~ῠνεπονται";
    subj_mid_1: "~ῠνεπωμαι", _, "~ῠνεπωμεθᾰ";
    subj_mid_2: "~ῠνεπῃ", "~ῠνεπησθον", _;
    subj_mid_3: "~ῠνεπηται", _, "~ῠνεπωνται";
    subj_pass_2: "~ῠνεπῃ", _, "~ῠνεπησθε";
    opt_mid_1: "~ῠνεποιμην", _, "~ῠνεποιμεθᾰ";
    opt_mid_2: "~ῠνεποιο", "~ῠνεποισθον", _;
    opt_mid_3: "~ῠνεποιτο", "~ῠνεποισθην", "~ῠνεποιντο";
    opt_pass_2: _, _, "~ῠνεποισθε";
    imp_mid_2: "~ῠνεπου", _, "~ῠνεπεσθε";
    imp_mid_3: "~ῠνεπεσθω", _, "~ῠνεπεσθων";
    imp_pass_2: _, _, "~ῠνεπεσθε";
    imp_pass_3: _, "~ῠνεπεσθων", "~ῠνεπεσθων";
    imperfect_mid_1: "~ῠνειπομην", _, "~ῠνειπομεθᾰ";
    imperfect_mid_2: "~ῠνειπου", "~ῠνειπεσθον", _;
    imperfect_mid_3: "~ῠνειπετο", "~ῠνειπεσθην", "~ῠνειποντο";
    imperfect_pass_2: _, _, "~ῠνειπεσθε";
    future_ind_mid_1: "~ῠνεψομαι", _, "~ῠνεψομεθᾰ";
    future_ind_mid_2: "~ῠνεψῃ|~ῠνεψει", "~ῠνεψεσθον", _;
    future_ind_mid_3: "~ῠνεψεται", _, _;
    future_opt_mid_1: "~ῠνεψοιμην", _, "~ῠνεψοιμεθᾰ";
    future_opt_mid_2: "~ῠνεψοιο", _, _;
    future_opt_mid_3: _, "~ῠνεψοισθην", _;
    aorist_subj_mid_1: "~ῠσπωμαι", _, _;
    aorist_subj_mid_2: "~ῠσπῃ", "~ῠσπησθον", _;
    aorist_opt_mid_1: "~ῠσποιμην", _, _;
    aorist_opt_mid_2: "~ῠσποιο", _, _;
    aorist_imp_mid_2: "~ῠσποῡ", "~ῠσπεσθον", _;
    aorist_imp_mid_3: _, _, "~ῠσπεσθων";
    infinitive_pres: _, _, "~ῠνεπεσθαι";
    participle_pres: _, _, "~ῠνεπομενος";
    participle_aor: _, "~ῠσπομενος", _;
    ),
    // Λαλαγεω: 2 words (καναχεω, λαλαγεω). Like Ζευγνῡμῐ, except in 117 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("λαλαγεω", 2, "~α~α~εω", like Ζευγνῡμῐ;
    stem future_pass: "~ᾰ~ᾰ~ηθησ- like ἐχω";
    stem aorist_ind: "~ᾰ~ᾰ~ησ- like ποιεω";
    stem aorist_ind_pass: "~ᾰ~ᾰ~ηθην- like ἐχω";
    stem aorist_pass: "~ᾰ~ᾰ~ηθ- like ποιεω";
    ind_act_1: "~ᾰ~ᾰ~ω", _, "~ᾰ~ᾰ~οῡμεν";
    ind_act_2: "~ᾰ~ᾰ~εῑς", _, _;
    ind_act_3: "~ᾰ~ᾰ~εῑ", "~ᾰ~ᾰ~εῑτον", "~ᾰ~ᾰ~οῡσῐ|~ᾰ~ᾰ~οῡσῐν";
    ind_mid_1: "~ᾰ~ᾰ~οῡμαι", _, "~ᾰ~ᾰ~ουμεθᾰ";
    ind_mid_2: "~ᾰ~ᾰ~εῑ|~ᾰ~ᾰ~ῃ", "~ᾰ~ᾰ~εῑσθον", _;
    ind_pass_2: _, _, "~ᾰ~ᾰ~εῑσθε";
    ind_pass_3: "~ᾰ~ᾰ~εῑται", _, "~ᾰ~ᾰ~οῡνται";
    subj_act_1: "~ᾰ~ᾰ~ω", _, "~ᾰ~ᾰ~ωμεν";
    subj_act_2: "~ᾰ~ᾰ~ῃς", _, _;
    subj_act_3: "~ᾰ~ᾰ~ῃ", "~ᾰ~ᾰ~ητον", "~ᾰ~ᾰ~ωσῐ|~ᾰ~ᾰ~ωσῐν";
    subj_mid_1: _, _, "~ᾰ~ᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ᾰ~ησθον", _;
    subj_mid_3: "~ᾰ~ᾰ~ηται", _, "~ᾰ~ᾰ~ωνται";
    subj_pass_1: "~ᾰ~ᾰ~ωμαι", _, _;
    subj_pass_2: _, _, "~ᾰ~ᾰ~ησθε";
    opt_act_1: "~ᾰ~ᾰ~οιην|~ᾰ~ᾰ~οῑμῐ", _, "~ᾰ~ᾰ~οῑμεν|~ᾰ~ᾰ~οιημεν";
    opt_act_2: "~ᾰ~ᾰ~οιης|~ᾰ~ᾰ~οῑς", "~ᾰ~ᾰ~οῑτον|~ᾰ~ᾰ~οιητον", "~ᾰ~ᾰ~οῑτε|~ᾰ~ᾰ~οιητε";
    opt_act_3: "~ᾰ~ᾰ~οιη|~ᾰ~ᾰ~οῑ", "~ᾰ~ᾰ~οιτην|~ᾰ~ᾰ~οιητην", "~ᾰ~ᾰ~οῑεν|~ᾰ~ᾰ~οιησᾰν";
    opt_mid_1: "~ᾰ~ᾰ~οιμην", _, "~ᾰ~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~ᾰ~οῑο", "~ᾰ~ᾰ~οῑσθον", _;
    opt_mid_3: "~ᾰ~ᾰ~οῑτο", "~ᾰ~ᾰ~οισθην", "~ᾰ~ᾰ~οῑντο";
    opt_pass_2: _, _, "~ᾰ~ᾰ~οῑσθε";
    imp_act_2: "~ᾰ~ᾰ~ει", "~ᾰ~ᾰ~εῑτον", "~ᾰ~ᾰ~εῑτε";
    imp_mid_2: "~ᾰ~ᾰ~οῡ", _, "~ᾰ~ᾰ~εῑσθε";
    imp_mid_3: "~ᾰ~ᾰ~εισθω", _, "~ᾰ~ᾰ~εισθων";
    imp_pass_2: _, _, "~ᾰ~ᾰ~εῑσθε";
    imp_pass_3: _, "~ᾰ~ᾰ~εισθων", "~ᾰ~ᾰ~εισθων";
    imperfect_act_1: "ἐ~ᾰ~ᾰ~ουν", _, "ἐ~ᾰ~ᾰ~οῡμεν";
    imperfect_act_2: "ἐ~ᾰ~ᾰ~εις", "ἐ~ᾰ~ᾰ~εῑτον", "ἐ~ᾰ~ᾰ~εῑτε";
    imperfect_act_3: "ἐ~ᾰ~ᾰ~ει", "ἐ~ᾰ~ᾰ~ειτην", "ἐ~ᾰ~ᾰ~ουν";
    imperfect_mid_1: "ἐ~ᾰ~ᾰ~ουμην", _, "ἐ~ᾰ~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~ᾰ~οῡ", _, _;
    imperfect_mid_3: "ἐ~ᾰ~ᾰ~εῑτο", _, _;
    imperfect_pass_2: _, "ἐ~ᾰ~ᾰ~εῑσθον", "ἐ~ᾰ~ᾰ~εῑσθε";
    imperfect_pass_3: _, "ἐ~ᾰ~ᾰ~εισθην", "ἐ~ᾰ~ᾰ~οῡντο";
    future_ind_act_1: "~ᾰ~ᾰ~ησω", _, "~ᾰ~ᾰ~ησομεν";
    future_ind_act_2: "~ᾰ~ᾰ~ησεις", "~ᾰ~ᾰ~ησετον", "~ᾰ~ᾰ~ησετε";
    future_ind_act_3: _, _, "~ᾰ~ᾰ~ησουσῐ|~ᾰ~ᾰ~ησουσῐν";
    future_ind_mid_1: "~ᾰ~ᾰ~ησομαι", _, "~ᾰ~ᾰ~ησομεθᾰ";
    future_ind_mid_2: "~ᾰ~ᾰ~ησῃ|~ᾰ~ᾰ~ησει", "~ᾰ~ᾰ~ησεσθον", _;
    future_ind_mid_3: "~ᾰ~ᾰ~ησεται", _, _;
    future_opt_act_1: "~ᾰ~ᾰ~ησοιμῐ", _, "~ᾰ~ᾰ~ησοιμεν";
    future_opt_act_2: _, "~ᾰ~ᾰ~ησοιτον", "~ᾰ~ᾰ~ησοιτε";
    future_opt_act_3: "~ᾰ~ᾰ~ησοι", "~ᾰ~ᾰ~ησοιτην", "~ᾰ~ᾰ~ησοιεν";
    future_opt_mid_1: "~ᾰ~ᾰ~ησοιμην", _, "~ᾰ~ᾰ~ησοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ᾰ~ησοιο", _, _;
    future_opt_mid_3: _, "~ᾰ~ᾰ~ησοισθην", _;
    aorist_subj_act_1: "~ᾰ~ᾰ~ηθω", _, _;
    aorist_subj_act_2: "~ᾰ~ᾰ~ηθῃς", "~ᾰ~ᾰ~ηθητον", _;
    aorist_subj_mid_1: "~ᾰ~ᾰ~ηθωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ᾰ~ηθῃ", "~ᾰ~ᾰ~ηθησθον", _;
    aorist_opt_act_1: "~ᾰ~ᾰ~ηθαιμῐ", _, "~ᾰ~ᾰ~ηθαιμεν";
    aorist_opt_act_2: "~ᾰ~ᾰ~ηθειᾰς", _, _;
    aorist_opt_act_3: "~ᾰ~ᾰ~ηθειε", "~ᾰ~ᾰ~ηθαιτην", "~ᾰ~ᾰ~ηθειᾰν";
    aorist_opt_mid_1: "~ᾰ~ᾰ~ηθαιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~ᾰ~ηθαιο", _, _;
    aorist_imp_act_2: "~ᾰ~ᾰ~ηθον", _, "~ᾰ~ᾰ~ηθᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ᾰ~ηθᾰντων";
    aorist_imp_mid_2: "~ᾰ~ᾰ~ηθαι", "~ᾰ~ᾰ~ηθᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ᾰ~ηθᾰσθων";
    infinitive_pres: "~ᾰ~ᾰ~εῑν", _, "~ᾰ~ᾰ~εῑσθαι";
    infinitive_fut: "~ᾰ~ᾰ~ησειν", _, _;
    infinitive_aor: "~ᾰ~ᾰ~ηθαι", _, _;
    participle_pres: "~ᾰ~ᾰ~ων", _, "~ᾰ~ᾰ~ουμενος";
    participle_aor: "~ᾰ~ᾰ~ηθᾱς", "~ᾰ~ᾰ~ηθᾰμενος", _;
    ),
    // Ξυνναυμᾰχεω: 2 words (συνναυμᾰχεω, ξυνναυμᾰχεω). Like Ζευγνῡμῐ, except in 117 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 du, ….
    conjugation!("ξυνναυμᾰχεω", 2, "~υ~~εω", like Ζευγνῡμῐ;
    stem imperfect: "~ῠ~ε~- like ξυνναυμᾰχεω";
    stem aorist_ind: "~ῠ~ε~ησ- like ξυνναυμᾰχεω";
    ind_act_1: "~ῠ~~ω", _, "~ῠ~~οῡμεν";
    ind_act_2: "~ῠ~~εῑς", "~ῠ~~εῑτον", _;
    ind_act_3: "~ῠ~~εῑ", derived, "~ῠ~~οῡσῐ|~ῠ~~οῡσῐν";
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, derived, _;
    ind_pass_2: _, _, derived;
    ind_pass_3: derived, _, derived;
    subj_act_1: "~ῠ~~ω", _, "~ῠ~~ωμεν";
    subj_act_2: "~ῠ~~ῃς", "~ῠ~~ητον", _;
    subj_act_3: "~ῠ~~ῃ", derived, "~ῠ~~ωσῐ|~ῠ~~ωσῐν";
    subj_mid_1: _, _, derived;
    subj_mid_2: _, derived, _;
    subj_mid_3: derived, _, derived;
    subj_pass_1: derived, _, _;
    subj_pass_2: _, _, derived;
    opt_act_1: "~ῠ~~οιην|~ῠ~~οῑμῐ", _, "~ῠ~~οῑμεν|~ῠ~~οιημεν";
    opt_act_2: "~ῠ~~οιης|~ῠ~~οῑς", "~ῠ~~οῑτον|~ῠ~~οιητον", "~ῠ~~οῑτε|~ῠ~~οιητε";
    opt_act_3: "~ῠ~~οιη|~ῠ~~οῑ", "~ῠ~~οιτην|~ῠ~~οιητην", "~ῠ~~οῑεν|~ῠ~~οιησᾰν";
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, _;
    opt_mid_3: derived, derived, derived;
    opt_pass_2: _, _, derived;
    imp_act_2: "~ῠ~~ει", derived, "~ῠ~~εῑτε";
    imp_act_3: "~ῠ~~ειτω", _, "~ῠ~~ουντων";
    imp_mid_2: derived, _, derived;
    imp_mid_3: derived, _, derived;
    imp_pass_2: _, _, derived;
    imp_pass_3: _, derived, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, _, _;
    imperfect_mid_3: derived, _, _;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, derived;
    future_ind_act_1: "~ῠ~~ησω", _, "~ῠ~~ησομεν";
    future_ind_act_2: "~ῠ~~ησεις", "~ῠ~~ησετον", "~ῠ~~ησετε";
    future_ind_act_3: _, _, "~ῠ~~ησουσῐ|~ῠ~~ησουσῐν";
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, _;
    future_ind_mid_3: derived, _, _;
    future_opt_act_1: "~ῠ~~ησοιμῐ", _, "~ῠ~~ησοιμεν";
    future_opt_act_2: _, "~ῠ~~ησοιτον", "~ῠ~~ησοιτε";
    future_opt_act_3: "~ῠ~~ησοι", "~ῠ~~ησοιτην", "~ῠ~~ησοιεν";
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, _, _;
    future_opt_mid_3: _, derived, _;
    aorist_ind_mid_1: derived, _, _;
    aorist_ind_mid_2: derived, derived, _;
    aorist_subj_act_1: "~ῠ~~ησω", _, _;
    aorist_subj_act_2: "~ῠ~~ησῃς", "~ῠ~~ησητον", _;
    aorist_subj_act_3: _, _, "~ῠ~~ησωσῐ|~ῠ~~ησωσῐν";
    aorist_subj_mid_1: derived, _, _;
    aorist_subj_mid_2: derived, derived, _;
    aorist_opt_act_1: "~ῠ~~ησαιμῐ", _, "~ῠ~~ησαιμεν";
    aorist_opt_act_2: "~ῠ~~ησειᾰς|~ῠ~~ησαις", _, _;
    aorist_opt_act_3: "~ῠ~~ησειε|~ῠ~~ησειεν|~ῠ~~ησαι", "~ῠ~~ησαιτην", "~ῠ~~ησειᾰν|~ῠ~~ησαιεν";
    aorist_opt_mid_1: derived, _, _;
    aorist_opt_mid_2: derived, _, _;
    aorist_imp_act_2: "~ῠ~~ησον", _, "~ῠ~~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῠ~~ησᾰντων";
    aorist_imp_mid_2: derived, derived, _;
    aorist_imp_mid_3: _, _, derived;
    infinitive_pres: "~ῠ~~εῑν", _, derived;
    infinitive_fut: "~ῠ~~ησειν", _, _;
    infinitive_aor: "~ῠ~~ησαι", _, _;
    participle_pres: "~ῠ~~ων", _, derived;
    participle_aor: "~ῠ~~ησᾱς", derived, _;
    ),
    // Προσκειμαι: 2 words (προκειμαι, προσκειμαι). Like Δῐακειμαι, except in 32 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, subj_mid_1 sg, ….
    conjugation!("προσκειμαι", 2, "~~ειμαι", like Δῐακειμαι;
    ind_act_1: "~~ειμαι", _, _;
    ind_mid_1: "~~ειμαι", _, "~~ειμεθα";
    ind_mid_2: "~~εισαι", "~~εισθον", _;
    subj_mid_1: "~~ωμαι", _, "~~ωμεθᾰ";
    subj_mid_2: "~~εῃ", "~~εησθον", _;
    subj_mid_3: "~~εηται", _, "~~εωνται|~~ειωνται";
    opt_mid_1: "~~οιμην", _, "~~οιμεθᾰ";
    opt_mid_2: "~~οῑο", "~~οῑσθον", _;
    opt_mid_3: "~~οῑτο", "~~οισθην", "~~οῑντο";
    imp_mid_2: "~~εισο", _, "~~εισθον";
    imp_mid_3: "~~εισθω", _, "~~εισθων";
    imperfect_mid_3: _, _, "~ε~εατο";
    future_ind_mid_1: "~~εισομαι", _, "~~εισομεθᾰ";
    future_ind_mid_2: "~~εισῃ", "~~εισεσθον", _;
    future_ind_mid_3: "~~εισεται", _, _;
    future_opt_mid_1: "~~εισοιμην", _, "~~εισοιμεθᾰ";
    future_opt_mid_2: "~~εισοιο", _, _;
    future_opt_mid_3: _, "~~εισοισθην", _;
    ),
    // Μετᾰκᾰλεω: 2 words (μηχᾰνορρᾰφεω, μετᾰκᾰλεω). Like Ζευγνῡμῐ, except in 160 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("μετᾰκᾰλεω", 2, "~ᾰ~ᾰ~εω", like Ζευγνῡμῐ;
    stem future_pass: "~α~α~εθησ- like ἐχω";
    stem aorist_ind: "~ε~α~εσ- like ἐλπω";
    stem aorist_ind_pass: "~ε~α~εθην- like ἐλπω";
    stem aorist_pass: "~α~α~εθ- like ἐχω";
    ind_act_1: "~α~α~ω", _, "~α~α~οῡμεν";
    ind_act_2: "~α~α~εῑς", _, _;
    ind_act_3: "~α~α~εῑ", "~α~α~εῑτον", "~α~α~οῡσῐ|~α~α~οῡσῐν";
    ind_mid_1: "~α~α~οῡμαι", _, "~α~α~ουμεθᾰ";
    ind_mid_2: "~α~α~εῑ|~α~α~ῃ", "~α~α~εῑσθον", _;
    ind_pass_2: _, _, "~α~α~εῑσθε";
    ind_pass_3: "~α~α~εῑται", _, "~α~α~οῡνται";
    subj_act_1: "~α~α~ω", _, "~α~α~ωμεν";
    subj_act_2: "~α~α~ῃς", _, _;
    subj_act_3: "~α~α~ῃ", "~α~α~ητον", "~α~α~ωσῐ|~α~α~ωσῐν";
    subj_mid_1: _, _, "~α~α~ωμεθᾰ";
    subj_mid_2: _, "~α~α~ησθον", _;
    subj_mid_3: "~α~α~ηται", _, "~α~α~ωνται";
    subj_pass_1: "~α~α~ωμαι", _, _;
    subj_pass_2: _, _, "~α~α~ησθε";
    opt_act_1: "~α~α~οιην|~α~α~οῑμῐ", _, "~α~α~οῑμεν|~α~α~οιημεν";
    opt_act_2: "~α~α~οιης|~α~α~οῑς", "~α~α~οῑτον|~α~α~οιητον", "~α~α~οῑτε|~α~α~οιητε";
    opt_act_3: "~α~α~οιη|~α~α~οῑ", "~α~α~οιτην|~α~α~οιητην", "~α~α~οῑεν|~α~α~οιησᾰν";
    opt_mid_1: "~α~α~οιμην", _, "~α~α~οιμεθᾰ";
    opt_mid_2: "~α~α~οῑο", "~α~α~οῑσθον", _;
    opt_mid_3: "~α~α~οῑτο", "~α~α~οισθην", "~α~α~οῑντο";
    opt_pass_2: _, _, "~α~α~οῑσθε";
    imp_act_2: "~α~α~ει", "~α~α~εῑτον", "~α~α~εῑτε";
    imp_mid_2: "~α~α~οῡ", _, "~α~α~εῑσθε";
    imp_mid_3: "~α~α~εισθω", _, "~α~α~εισθων";
    imp_pass_2: _, _, "~α~α~εῑσθε";
    imp_pass_3: _, "~α~α~εισθων", "~α~α~εισθων";
    imperfect_act_1: "~ε~α~ουν", _, "~ε~α~οῡμεν";
    imperfect_act_2: "~ε~α~εις", "~ε~α~εῑτον", "~ε~α~εῑτε";
    imperfect_act_3: "~ε~α~ει", "~ε~α~ειτην", "~ε~α~ουν";
    imperfect_mid_1: "~ε~α~ουμην", _, "~ε~α~ουμεθᾰ";
    imperfect_mid_2: "~ε~α~οῡ", _, _;
    imperfect_mid_3: "~ε~α~εῑτο", _, _;
    imperfect_pass_2: _, "~ε~α~εῑσθον", "~ε~α~εῑσθε";
    imperfect_pass_3: _, "~ε~α~εισθην", "~ε~α~οῡντο";
    future_ind_act_1: "~α~α~εσω", _, "~α~α~εσομεν";
    future_ind_act_2: "~α~α~εσεις", "~α~α~εσετον", "~α~α~εσετε";
    future_ind_act_3: _, _, "~α~α~εσουσῐ";
    future_ind_mid_1: "~α~α~εσομαι", _, "~α~α~εσομεθᾰ";
    future_ind_mid_2: "~α~α~εσῃ", "~α~α~εσεσθον", _;
    future_ind_mid_3: "~α~α~εσεται", _, _;
    future_opt_act_1: "~α~α~εσοιμῐ", _, "~α~α~εσοιμεν";
    future_opt_act_2: _, "~α~α~εσοιτον", "~α~α~εσοιτε";
    future_opt_act_3: "~α~α~εσοι", "~α~α~εσοιτην", "~α~α~εσοιεν";
    future_opt_mid_1: "~α~α~εσοιμην", _, "~α~α~εσοιμεθᾰ";
    future_opt_mid_2: "~α~α~εσοιο", _, _;
    future_opt_mid_3: _, "~α~α~εσοισθην", _;
    aorist_subj_act_1: "~α~α~εσω", _, _;
    aorist_subj_act_2: "~α~α~εσῃς", "~α~α~εσητον", _;
    aorist_subj_mid_1: "~α~α~εσωμαι", _, _;
    aorist_subj_mid_2: "~α~α~εσῃ", "~α~α~εσησθον", _;
    aorist_opt_act_1: "~α~α~εσαιμῐ", _, "~α~α~εσαιμεν";
    aorist_opt_act_2: "~α~α~εσειᾰς", _, _;
    aorist_opt_act_3: "~α~α~εσειε", "~α~α~εσαιτην", "~α~α~εσειᾰν";
    aorist_opt_mid_1: "~α~α~εσαιμην", _, _;
    aorist_opt_mid_2: "~α~α~εσαιο", _, _;
    aorist_imp_act_2: "~α~α~εσον", _, "~α~α~εσᾰτε";
    aorist_imp_act_3: _, _, "~α~α~εσᾰντων";
    aorist_imp_mid_2: "~α~α~εσαι", "~α~α~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~α~α~εσᾰσθων";
    perfect_ind_act_1: "~α~εκα~εκᾰ", _, "~α~εκα~εκᾰμεν";
    perfect_ind_act_2: _, "~α~εκα~εκᾰτον", _;
    perfect_ind_act_3: _, _, "~α~εκα~εκᾱσῐ";
    perfect_ind_mid_1: "~α~εκα~εμαι", _, "~α~εκα~εμεθᾰ";
    perfect_ind_mid_2: "~α~εκα~εσαι", "~α~εκα~εσθον", _;
    perfect_ind_mid_3: "~α~εκα~εται", _, "~α~εκα~ενται";
    perfect_ind_pass_2: _, _, "~α~εκα~εσθε";
    perfect_subj_act_1: "~α~εκα~εκω", _, _;
    perfect_subj_act_2: "~α~εκα~εκῃς", "~α~εκα~εκητον", "~α~εκα~εκητε";
    perfect_subj_act_3: "~α~εκα~εκῃ", _, "~α~εκα~εκωσῐ";
    perfect_opt_act_1: "~α~εκα~εκοιμῐ", _, "~α~εκα~εκοιμεν";
    perfect_opt_act_2: "~α~εκα~εκοις", _, _;
    perfect_opt_act_3: _, "~α~εκα~εκοιτην", _;
    perfect_imp_act_2: "~α~εκα~εκε", _, "~α~εκα~εκετε";
    perfect_imp_act_3: _, _, "~α~εκα~εκοντων";
    perfect_imp_mid_2: _, _, "~α~εκα~εσθε";
    perfect_imp_mid_3: "~α~εκα~εσθω", _, "~α~εκα~εσθων";
    perfect_imp_pass_2: "~α~εκα~εσο", _, "~α~εκα~εσθε";
    perfect_imp_pass_3: _, "~α~εκα~εσθων", "~α~εκα~εσθων";
    pluperfect_act_1: "~ε~εκα~εκειν", _, _;
    pluperfect_mid_1: "~ε~εκα~εμην", _, _;
    pluperfect_mid_2: "~ε~εκα~εσο", "~ε~εκα~εσθον", _;
    pluperfect_mid_3: "~ε~εκα~ετο", _, "~ε~εκα~εντο";
    pluperfect_pass_1: _, _, "~ε~εκα~εμεθᾰ";
    pluperfect_pass_2: _, _, "~ε~εκα~εσθε";
    pluperfect_pass_3: _, "~ε~εκα~εσθην", _;
    infinitive_pres: "~α~α~εῑν", _, "~α~α~εῑσθαι";
    infinitive_fut: "~α~α~εσειν", _, _;
    infinitive_aor: "~α~α~εσαι", _, _;
    infinitive_perf: _, "~α~εκα~εσθαι", _;
    participle_pres: "~α~α~ων", _, "~α~α~ουμενος";
    participle_aor: "~α~α~εσᾱς", "~α~α~εσᾰμενος", _;
    participle_perf: "~α~εκα~εκως", _, "~α~εκα~εμενος";
    ),
    // Ερεω: 2 words (ερεω, ετεω). Like Μῑγνῡμῐ, except in 80 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ερεω", 2, "ε~εω", like Μῑγνῡμῐ;
    ind_act_1: "ἑ~ω", _, "ἑ~οῡμεν";
    ind_act_2: "ἑ~εῑς", _, _;
    ind_act_3: "ἑ~εῑ", "ἑ~εῑτον", "ἑ~οῡσῐ|ἑ~οῡσῐν";
    ind_mid_1: "ἑ~οῡμαι", _, "ἑ~ουμεθᾰ";
    ind_mid_2: "ἑ~εῑ|ἑ~ῃ", "ἑ~εῑσθον", _;
    ind_pass_2: _, _, "ἑ~εῑσθε";
    ind_pass_3: "ἑ~εῑται", _, "ἑ~οῡνται";
    subj_act_1: "ἑ~ω", _, "ἑ~ωμεν";
    subj_act_2: "ἑ~ῃς", _, _;
    subj_act_3: "ἑ~ῃ", "ἑ~ητον", "ἑ~ωσῐ|ἑ~ωσῐν";
    subj_mid_1: _, _, "ἑ~ωμεθᾰ";
    subj_mid_2: _, "ἑ~ησθον", _;
    subj_mid_3: "ἑ~ηται", _, "ἑ~ωνται";
    subj_pass_1: "ἑ~ωμαι", _, _;
    subj_pass_2: _, _, "ἑ~ησθε";
    opt_act_1: "ἑ~οιην|ἑ~οῑμῐ", _, "ἑ~οῑμεν|ἑ~οιημεν";
    opt_act_2: "ἑ~οιης|ἑ~οῑς", "ἑ~οῑτον|ἑ~οιητον", "ἑ~οῑτε|ἑ~οιητε";
    opt_act_3: "ἑ~οιη|ἑ~οῑ", "ἑ~οιτην|ἑ~οιητην", "ἑ~οῑεν|ἑ~οιησᾰν";
    opt_mid_1: "ἑ~οιμην", _, "ἑ~οιμεθᾰ";
    opt_mid_2: "ἑ~οῑο", "ἑ~οῑσθον", _;
    opt_mid_3: "ἑ~οῑτο", "ἑ~οισθην", "ἑ~οῑντο";
    opt_pass_2: _, _, "ἑ~οῑσθε";
    imp_act_2: "ἑ~ει", "ἑ~εῑτον", "ἑ~εῑτε";
    imp_mid_2: "ἑ~οῡ", _, "ἑ~εῑσθε";
    imp_mid_3: "ἑ~εισθω", _, "ἑ~εισθων";
    imp_pass_2: _, _, "ἑ~εῑσθε";
    imp_pass_3: _, "ἑ~εισθων", "ἑ~εισθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_1: "ἑ~ησω", _, "ἑ~ησομεν";
    future_ind_act_2: "ἑ~ησεις", "ἑ~ησετον", "ἑ~ησετε";
    future_ind_act_3: _, _, "ἑ~ησουσῐ";
    future_ind_mid_1: "ἑ~ησομαι", _, "ἑ~ησομεθᾰ";
    future_ind_mid_2: "ἑ~ησῃ", "ἑ~ησεσθον", _;
    future_ind_mid_3: "ἑ~ησεται", _, _;
    future_opt_act_1: "ἑ~ησοιμῐ", _, "ἑ~ησοιμεν";
    future_opt_act_2: _, "ἑ~ησοιτον", "ἑ~ησοιτε";
    future_opt_act_3: "ἑ~ησοι", "ἑ~ησοιτην", "ἑ~ησοιεν";
    future_opt_mid_1: "ἑ~ησοιμην", _, "ἑ~ησοιμεθᾰ";
    future_opt_mid_2: "ἑ~ησοιο", _, _;
    future_opt_mid_3: _, "ἑ~ησοισθην", _;
    infinitive_pres: "ἑ~εῑν", _, "ἑ~εῑσθαι";
    infinitive_fut: "ἑ~ησειν", _, _;
    participle_pres: "ἑ~ων", _, "ἑ~ουμενος";
    ),
    // Ἐδω: 1 words (ἐδω). Like Γεγονοι, except in 64 cells: ind_act_1 sg, future_ind_mid_1 sg, future_ind_mid_1 pl, future_ind_mid_2 sg, future_ind_mid_2 du, future_ind_mid_3 sg, ….
    conjugation!("ἐδω", 1, "~ω", like Γεγονοι;
    stem pluperfect_act: "~ηδοκ- like εὐλογεω";
    ind_act_1: "~ω", _, _;
    future_ind_mid_1: "~ομαι", _, "~ομεθᾰ";
    future_ind_mid_2: "~ῃ|~ει", "~εσθον", _;
    future_ind_mid_3: "~εται", _, _;
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_2: "~οιο", _, _;
    future_opt_mid_3: _, "~οισθην", _;
    perfect_ind_act_1: "~ηδοκᾰ|~ηδᾰ", _, "~ηδοκᾰμεν|~ηδᾰμεν";
    perfect_ind_act_2: "~ηδοκᾰς|~ηδᾰς", "~ηδοκᾰτον|~ηδᾰτον", "~ηδοκᾰτε|~ηδᾰτε";
    perfect_ind_act_3: "~ηδοκε|~ηδοκεν|~ηδε|~ηδεν", _, "~ηδοκᾱσῐ|~ηδοκᾱσῐν|~ηδᾱσῐ|~ηδᾱσῐν";
    perfect_ind_mid_1: "~ηδομαι", _, "~ηδομεθᾰ";
    perfect_ind_mid_2: "~ηδοσαι", "~ηδοσθον", _;
    perfect_ind_mid_3: "~ηδοται", _, "~ηδονται";
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
    perfect_imp_mid_3: "~ηδοσθω", _, "~ηδοσθων";
    perfect_imp_pass_2: "~ηδοσο", _, "~ηδοσθε";
    perfect_imp_pass_3: _, "~ηδοσθων", "~ηδοσθων";
    pluperfect_mid_1: "~ηδομην", _, _;
    pluperfect_mid_2: "~ηδοσο", "~ηδοσθον", _;
    pluperfect_mid_3: "~ηδοτο", _, "~ηδοντο";
    pluperfect_pass_1: _, _, "~ηδομεθᾰ";
    pluperfect_pass_2: _, _, "~ηδοσθε";
    pluperfect_pass_3: _, "~ηδοσθην", _;
    infinitive_perf: "~ηδοκεναι|~ηδεναι", "~ηδοσθαι", _;
    participle_perf: "~ηδοκως|~ηδως", _, "~ηδομενος";
    ),
    // Αἰσθᾰνομαι: 1 words (αἰσθᾰνομαι). Like Ἐρᾰμαι, except in 122 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("αἰσθᾰνομαι", 1, "αἰ~ᾰνομαι", like Ἐρᾰμαι;
    stem aorist_pass: "~ηθ- like αἰσθᾰνομαι";
    ind_act_1: "αἰ~ᾰνομαι", _, _;
    ind_mid_1: "αἰ~ανομαι", _, "αἰ~ανομεθᾰ";
    ind_mid_2: "αἰ~ανῃ|αἰ~ανει", "αἰ~ανεσθον", _;
    ind_pass_2: _, _, "αἰ~ανεσθε";
    ind_pass_3: "αἰ~ανεται", _, "αἰ~ανονται";
    subj_mid_1: "αἰ~ανωμαι", _, "αἰ~ανωμεθᾰ";
    subj_mid_2: "αἰ~ανῃ", "αἰ~ανησθον", _;
    subj_mid_3: "αἰ~ανηται", _, "αἰ~ανωνται";
    subj_pass_2: "αἰ~ανῃ", _, "αἰ~ανησθε";
    opt_mid_1: "αἰ~ανοιμην", _, "αἰ~ανοιμεθᾰ";
    opt_mid_2: "αἰ~ανοιο", "αἰ~ανοισθον", _;
    opt_mid_3: "αἰ~ανοιτο", "αἰ~ανοισθην", "αἰ~ανοιντο";
    opt_pass_2: _, _, "αἰ~ανοισθε";
    imp_mid_2: "αἰ~ανου", _, "αἰ~ανεσθε";
    imp_mid_3: "αἰ~ανεσθω", _, "αἰ~ανεσθων";
    imp_pass_2: _, _, "αἰ~ανεσθε";
    imp_pass_3: _, "αἰ~ανεσθων", "αἰ~ανεσθων";
    imperfect_mid_1: "ᾐ~ανομην", _, "ᾐ~ανομεθᾰ";
    imperfect_mid_2: "ᾐ~ανου", "ᾐ~ανεσθον", _;
    imperfect_mid_3: "ᾐ~ανετο", "ᾐ~ανεσθην", "ᾐ~ανοντο";
    imperfect_pass_2: _, _, "ᾐ~ανεσθε";
    future_ind_mid_1: "αἰ~ησομαι", _, "αἰ~ησομεθᾰ";
    future_ind_mid_2: "αἰ~ησῃ|αἰ~ησει", "αἰ~ησεσθον", _;
    future_ind_mid_3: "αἰ~ησεται", _, _;
    future_ind_pass_1: "αἰ~ηθησομαι|αἰ~ανθησομαι", _, "αἰ~ηθησομεθᾰ|αἰ~ανθησομεθᾰ";
    future_ind_pass_2: "αἰ~ηθησῃ|αἰ~ηθησει|αἰ~ανθησῃ|αἰ~ανθησει", "αἰ~ηθησεσθον|αἰ~ανθησεσθον", "αἰ~ηθησεσθε|αἰ~ανθησεσθε";
    future_ind_pass_3: "αἰ~ηθησεται|αἰ~ανθησεται", _, "αἰ~ηθησονται|αἰ~ανθησονται";
    future_opt_mid_1: "αἰ~ησοιμην", _, "αἰ~ησοιμεθᾰ";
    future_opt_mid_2: "αἰ~ησοιο", _, _;
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
    aorist_imp_mid_2: "αἰ~οῡ|αἰ~ησαι", "αἰ~εσθον|αἰ~ησᾰσθον", "αἰ~εσθε|αἰ~ησᾰσθε";
    aorist_imp_mid_3: "αἰ~εσθω|αἰ~ησᾰσθω", "αἰ~εσθων|αἰ~ησᾰσθων", _;
    perfect_ind_mid_1: "ᾐ~ημαι", _, "ᾐ~ημεθᾰ";
    perfect_ind_mid_2: "ᾐ~ησαι", "ᾐ~ησθον", _;
    perfect_ind_mid_3: "ᾐ~ηται", _, "ᾐ~ηνται";
    perfect_ind_pass_2: _, _, "ᾐ~ησθε";
    perfect_imp_mid_2: _, _, "ᾐ~ησθε";
    perfect_imp_mid_3: "ᾐ~ησθω", _, "ᾐ~ησθων";
    perfect_imp_pass_2: "ᾐ~ησο", _, "ᾐ~ησθε";
    perfect_imp_pass_3: _, "ᾐ~ησθων", "ᾐ~ησθων";
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
    // Τρυπαω: 1 words (τρυπαω). Like Ἐκφερω, except in 139 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("τρυπαω", 1, "~υ~αω", like Ἐκφερω;
    stem aorist_ind: "~ῡ~ησ- like ποιεω";
    stem pluperfect_act: "~ῡ~ηκ- like χρημᾰτῐζω";
    ind_act_1: "~ῡ~ω", _, "~ῡ~ωμεν";
    ind_act_2: "~ῡ~ᾳς", _, _;
    ind_act_3: "~ῡ~ᾳ", "~ῡ~ᾱτον", "~ῡ~ωσῐ|~ῡ~ωσῐν";
    ind_mid_1: "~ῡ~ωμαι", _, "~ῡ~ωμεθᾰ";
    ind_mid_2: "~ῡ~ᾳ", "~ῡ~ᾱσθον", _;
    ind_pass_2: _, _, "~ῡ~ᾱσθε";
    ind_pass_3: "~ῡ~ᾱται", _, "~ῡ~ωνται";
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
    opt_mid_2: "~ῡ~ῳο", "~ῡ~ῳσθον", _;
    opt_mid_3: "~ῡ~ῳτο", "~ῡ~ῳσθην", "~ῡ~ῳντο";
    opt_pass_2: _, _, "~ῡ~ῳσθε";
    imp_act_2: "~ῡ~ᾱ", "~ῡ~ᾱτον", "~ῡ~ᾱτε";
    imp_mid_2: "~ῡ~ω", _, "~ῡ~ᾱσθε";
    imp_mid_3: "~ῡ~ᾱσθω", _, "~ῡ~ᾱσθων";
    imp_pass_2: _, _, "~ῡ~ᾱσθε";
    imp_pass_3: _, "~ῡ~ᾱσθων", "~ῡ~ᾱσθων";
    imperfect_act_1: "ἐ~ῡ~ων", _, "ἐ~ῡ~ωμεν";
    imperfect_act_2: "ἐ~ῡ~ᾱς", "ἐ~ῡ~ᾱτον", "ἐ~ῡ~ᾱτε";
    imperfect_act_3: "ἐ~ῡ~ᾱ", "ἐ~ῡ~ᾱτην", "ἐ~ῡ~ων";
    imperfect_mid_1: "ἐ~ῡ~ωμην", _, "ἐ~ῡ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ῡ~ω", _, _;
    imperfect_mid_3: "ἐ~ῡ~ᾱτο", _, _;
    imperfect_pass_2: _, "ἐ~ῡ~ᾱσθον", "ἐ~ῡ~ᾱσθε";
    imperfect_pass_3: _, "ἐ~ῡ~ᾱσθην", "ἐ~ῡ~ωντο";
    aorist_subj_act_1: "~ῡ~ησω", _, _;
    aorist_subj_act_2: "~ῡ~ησῃς", "~ῡ~ησητον", _;
    aorist_subj_act_3: _, _, "~ῡ~ησωσῐ|~ῡ~ησωσῐν";
    aorist_subj_mid_1: "~ῡ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ῡ~ησῃ", "~ῡ~ησησθον", _;
    aorist_opt_act_1: "~ῡ~ησαιμῐ", _, "~ῡ~ησαιμεν";
    aorist_opt_act_2: "~ῡ~ησειᾰς|~ῡ~ησαις", _, _;
    aorist_opt_act_3: "~ῡ~ησειε|~ῡ~ησειεν|~ῡ~ησαι", "~ῡ~ησαιτην", "~ῡ~ησειᾰν|~ῡ~ησαιεν";
    aorist_opt_mid_1: "~ῡ~ησαιμην", _, _;
    aorist_opt_mid_2: "~ῡ~ησαιο", _, _;
    aorist_imp_act_2: "~ῡ~ησον", _, "~ῡ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῡ~ησᾰντων";
    aorist_imp_mid_2: "~ῡ~ησαι", "~ῡ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῡ~ησᾰσθων";
    perfect_ind_act_1: "%ε~ῡ~ηκᾰ", _, "%ε~ῡ~ηκᾰμεν";
    perfect_ind_act_2: _, "%ε~ῡ~ηκᾰτον", _;
    perfect_ind_act_3: "%ε~ῡ~ηκε|%ε~ῡ~ηκεν", _, "%ε~ῡ~ηκᾱσῐ|%ε~ῡ~ηκᾱσῐν";
    perfect_ind_mid_1: "%ε~ῡ~ημαι", _, "%ε~ῡ~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ῡ~ησαι", "%ε~ῡ~ησθον", _;
    perfect_ind_mid_3: "%ε~ῡ~ηται", _, "%ε~ῡ~ηνται";
    perfect_ind_pass_2: _, _, "%ε~ῡ~ησθε";
    perfect_subj_act_1: "%ε~ῡ~ηκω", _, _;
    perfect_subj_act_2: "%ε~ῡ~ηκῃς", "%ε~ῡ~ηκητον", "%ε~ῡ~ηκητε";
    perfect_subj_act_3: "%ε~ῡ~ηκῃ", _, "%ε~ῡ~ηκωσῐ|%ε~ῡ~ηκωσῐν";
    perfect_opt_act_1: "%ε~ῡ~ηκοιμῐ|%ε~ῡ~ηκοιην", _, "%ε~ῡ~ηκοιμεν";
    perfect_opt_act_2: "%ε~ῡ~ηκοις|%ε~ῡ~ηκοιης", _, _;
    perfect_opt_act_3: "%ε~ῡ~ηκοι|%ε~ῡ~ηκοιη", "%ε~ῡ~ηκοιτην", _;
    perfect_imp_act_2: "%ε~ῡ~ηκε", _, "%ε~ῡ~ηκετε";
    perfect_imp_act_3: _, _, "%ε~ῡ~ηκοντων";
    perfect_imp_mid_2: _, _, "%ε~ῡ~ησθε";
    perfect_imp_mid_3: "%ε~ῡ~ησθω", _, "%ε~ῡ~ησθων";
    perfect_imp_pass_2: "%ε~ῡ~ησο", _, "%ε~ῡ~ησθε";
    perfect_imp_pass_3: _, "%ε~ῡ~ησθων", "%ε~ῡ~ησθων";
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
    // Μειδᾰω: 1 words (μειδᾰω). Like Μῑγνῡμῐ, except in 41 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("μειδᾰω", 1, "~ᾰω", like Μῑγνῡμῐ;
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
    imp_act_3: "~ιᾱτω", _, "~ιωντων";
    aorist_subj_act_1: "~ιᾱσω", _, _;
    aorist_subj_act_2: "~ιᾱσῃς", "~ιᾱσητον", _;
    aorist_subj_act_3: _, _, "~ιᾱσωσῐ|~ιᾱσωσῐν";
    aorist_opt_act_1: "~ιᾱσαιμῐ", _, "~ιᾱσαιμεν";
    aorist_opt_act_2: "~ιᾱσειᾰς|~ιᾱσαις", _, _;
    aorist_opt_act_3: "~ιᾱσειε|~ιᾱσειεν|~ιᾱσαι", "~ιᾱσαιτην", "~ιᾱσειᾰν|~ιᾱσαιεν";
    aorist_imp_act_2: "~ιᾱσον", _, "~ιᾱσᾰτε";
    aorist_imp_act_3: _, _, "~ιᾱσᾰντων";
    infinitive_pres: "~ιᾱν", _, _;
    infinitive_aor: "~ιᾱσαι", _, _;
    participle_pres: "~ιων", _, _;
    participle_aor: "~ιᾱσᾱς", _, _;
    ),
    // Κεῑμαι: 1 words (κεῑμαι). Like Μῑγνῡμῐ, except in 41 cells: ind_act_1 sg, ind_act_3 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ….
    conjugation!("κεῑμαι", 1, "~ῑμαι", like Μῑγνῡμῐ;
    ind_act_1: "~ῑμαι", _, _;
    ind_act_3: derived, _, _;
    ind_mid_1: "~ῑμαι", _, "~ιμεθα";
    ind_mid_2: "~ῑσαι", "~ῑσθον", _;
    ind_mid_3: _, _, "~ῑνται|~ᾰται|~ονται|~ιᾰται";
    subj_mid_1: "~ωμαι", _, "~ωμεθα";
    subj_mid_2: "~ῃ", "~ησθον", _;
    subj_mid_3: "~ηται|~ῑται", _, "~ωνται|~ιωνται";
    opt_mid_1: "~οιμην", _, "~οιμεθα";
    opt_mid_2: "~οιο", _, "~οισθε";
    opt_mid_3: "~οιτο", _, "~οιντο";
    imp_mid_2: "~ῑσο", _, "~ῑσθε";
    imp_mid_3: "~ισθω", _, "~ισθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    imperfect_mid_1: "ἐ~ιμην", _, "ἐ~ιμεθα";
    imperfect_mid_2: "ἐ~ισο", "ἐ~ισθον", _;
    imperfect_mid_3: "ἐ~ιτο", "ἐ~ισθην", "ἐ~ιντο|ἐ~ατο";
    future_ind_mid_1: "~ισομαι", _, "~ισομεθᾰ";
    future_ind_mid_2: "~ισῃ|~ισει", "~ισεσθον", _;
    future_ind_mid_3: "~ισεται", _, _;
    future_opt_mid_1: "~ισοιμην", _, "~ισοιμεθᾰ";
    future_opt_mid_2: "~ισοιο", _, _;
    future_opt_mid_3: _, "~ισοισθην", _;
    ),
    // Ἀνᾰχρονῐζομαι: 1 words (ἀνᾰχρονῐζομαι). Like Φειδομαι, except in 83 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ἀνᾰχρονῐζομαι", 1, "~ᾰ~ζομαι", like Φειδομαι;
    stem future_pass: "~α~σθησ- like ποιεω";
    stem aorist_ind: "~ε~σ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ε~σθην- like ἐλπω";
    stem aorist_pass: "~α~σθ- like ποιεω";
    ind_act_1: "~ᾰ~ζομαι", _, _;
    ind_mid_1: "~α~ζομαι", _, "~α~ζομεθᾰ";
    ind_mid_2: "~α~ζῃ|~α~ζει", "~α~ζεσθον", _;
    ind_pass_2: _, _, "~α~ζεσθε";
    ind_pass_3: "~α~ζεται", _, "~α~ζονται";
    subj_mid_1: "~α~ζωμαι", _, "~α~ζωμεθᾰ";
    subj_mid_2: "~α~ζῃ", "~α~ζησθον", _;
    subj_mid_3: "~α~ζηται", _, "~α~ζωνται";
    subj_pass_2: "~α~ζῃ", _, "~α~ζησθε";
    opt_mid_1: "~α~ζοιμην", _, "~α~ζοιμεθᾰ";
    opt_mid_2: "~α~ζοιο", "~α~ζοισθον", _;
    opt_mid_3: "~α~ζοιτο", "~α~ζοισθην", "~α~ζοιντο";
    opt_pass_2: _, _, "~α~ζοισθε";
    imp_mid_2: "~α~ζου", _, "~α~ζεσθε";
    imp_mid_3: "~α~ζεσθω", _, "~α~ζεσθων";
    imp_pass_2: _, _, "~α~ζεσθε";
    imp_pass_3: _, "~α~ζεσθων", "~α~ζεσθων";
    imperfect_mid_1: "~ε~ζομην", _, "~ε~ζομεθᾰ";
    imperfect_mid_2: "~ε~ζου", "~ε~ζεσθον", _;
    imperfect_mid_3: "~ε~ζετο", "~ε~ζεσθην", "~ε~ζοντο";
    imperfect_pass_2: _, _, "~ε~ζεσθε";
    future_ind_mid_1: "~α~οῡμαι", _, "~α~ουμεθᾰ";
    future_ind_mid_2: "~α~εῑ|~α~ῃ", "~α~εῑσθον", _;
    future_ind_mid_3: "~α~εῑται", _, _;
    future_opt_mid_1: "~α~οιμην", _, "~α~οιμεθᾰ";
    future_opt_mid_2: "~α~οῑο", _, _;
    future_opt_mid_3: _, "~α~οισθην", _;
    aorist_subj_mid_1: "~α~σωμαι", _, _;
    aorist_subj_mid_2: "~α~σῃ", "~α~σησθον", _;
    aorist_opt_mid_1: "~α~σαιμην", _, _;
    aorist_opt_mid_2: "~α~σαιο", _, _;
    aorist_imp_mid_2: "~α~σαι", "~α~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~α~σᾰσθων";
    perfect_ind_mid_1: "~α%ε~σμαι", _, "~α%ε~σμεθᾰ";
    perfect_ind_mid_2: "~α%ε~σαι", "~α%ε~σθον", _;
    perfect_ind_mid_3: "~α%ε~σται", _, "~α%ε~δᾰται";
    perfect_ind_pass_2: _, _, "~α%ε~σθε";
    perfect_imp_mid_2: _, _, "~α%ε~σθε";
    perfect_imp_mid_3: "~α%ε~σθω", _, "~α%ε~σθων";
    perfect_imp_pass_2: "~α%ε~σο", _, "~α%ε~σθε";
    perfect_imp_pass_3: _, "~α%ε~σθων", "~α%ε~σθων";
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
    // Σῠμμᾰχομαι: 1 words (σῠμμᾰχομαι). Like Νισσομαι, except in 51 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("σῠμμᾰχομαι", 1, "~ῠμ~ομαι", like Νισσομαι;
    stem aorist_ind: "~υνε~εσ- like μηκᾰομαι";
    ind_act_1: "~ῠμ~ομαι", _, _;
    ind_mid_1: "~υμ~ομαι", _, "~υμ~ομεθᾰ";
    ind_mid_2: "~υμ~ῃ|~υμ~ει", "~υμ~εσθον", _;
    ind_pass_2: _, _, "~υμ~εσθε";
    ind_pass_3: "~υμ~εται", _, "~υμ~ονται";
    subj_mid_1: "~υμ~ωμαι", _, "~υμ~ωμεθᾰ";
    subj_mid_2: "~υμ~ῃ", "~υμ~ησθον", _;
    subj_mid_3: "~υμ~ηται", _, "~υμ~ωνται";
    subj_pass_2: "~υμ~ῃ", _, "~υμ~ησθε";
    opt_mid_1: "~υμ~οιμην", _, "~υμ~οιμεθᾰ";
    opt_mid_2: "~υμ~οιο", "~υμ~οισθον", _;
    opt_mid_3: "~υμ~οιτο", "~υμ~οισθην", "~υμ~οιντο";
    opt_pass_2: _, _, "~υμ~οισθε";
    imp_mid_2: "~υμ~ου", _, "~υμ~εσθε";
    imp_mid_3: "~υμ~εσθω", _, "~υμ~εσθων";
    imp_pass_2: _, _, "~υμ~εσθε";
    imp_pass_3: _, "~υμ~εσθων", "~υμ~εσθων";
    future_ind_mid_1: "~υμ~οῡμαι", _, "~υμ~ουμεθᾰ";
    future_ind_mid_2: "~υμ~εῑ|~υμ~ῃ", "~υμ~εῑσθον", _;
    future_ind_mid_3: "~υμ~εῑται", _, _;
    future_opt_mid_1: "~υμ~οιμην", _, "~υμ~οιμεθᾰ";
    future_opt_mid_2: "~υμ~οῑο", _, _;
    future_opt_mid_3: _, "~υμ~οισθην", _;
    aorist_subj_mid_1: "~υμ~εσωμαι", _, _;
    aorist_subj_mid_2: "~υμ~εσῃ", "~υμ~εσησθον", _;
    aorist_opt_mid_1: "~υμ~εσαιμην", _, _;
    aorist_opt_mid_2: "~υμ~εσαιο", _, _;
    aorist_imp_mid_2: "~υμ~εσαι", "~υμ~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~υμ~εσᾰσθων";
    infinitive_pres: _, _, "~υμ~εσθαι";
    participle_pres: _, _, "~υμ~ομενος";
    participle_aor: _, "~υμ~εσᾰμενος", _;
    ),
    // Κᾰθημαι: 1 words (κᾰθημαι). Like Νισσομαι, except in 41 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("κᾰθημαι", 1, "~ᾰ~μαι", like Νισσομαι;
    ind_act_1: "~ᾰ~μαι", _, _;
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, derived, _;
    ind_pass_2: _, _, derived;
    ind_pass_3: derived, _, derived;
    subj_mid_1: derived, _, derived;
    subj_mid_2: derived, derived, _;
    subj_mid_3: derived, _, derived;
    subj_pass_2: derived, _, derived;
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, _;
    opt_mid_3: derived, derived, derived;
    opt_pass_2: _, _, derived;
    imp_mid_2: derived, _, derived;
    imp_mid_3: derived, _, derived;
    imp_pass_2: _, _, derived;
    imp_pass_3: _, derived, derived;
    pluperfect_mid_1: "ἐ~α~μην|~α~μην", _, "ἐ~α~μεθᾰ|~α~μεθᾰ";
    pluperfect_mid_2: "ἐ~α~σο|~α~σο", "ἐ~α~σθον|~α~σθον", "ἐ~α~σθε|~α~σθε";
    pluperfect_mid_3: "ἐ~α~το|~α~στο|~α~το", "ἐ~α~σθην|~α~σθην", "ἐ~α~ντο|~α~ντο";
    infinitive_pres: _, _, derived;
    participle_pres: _, _, derived;
    ),
    // Ἐπῐκᾰλεω: 1 words (ἐπῐκᾰλεω). Like Μετᾰκᾰλεω, except in 147 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἐπῐκᾰλεω", 1, "~ῐ~ᾰ~εω", like Μετᾰκᾰλεω;
    stem future_pass: "~ι%ε~~ησ- like ποιεω";
    stem aorist_ind: "~ε~α~εσ- like εὐλογεω";
    stem aorist_ind_pass: "~ε~~ηθην- like ἐλπω";
    stem aorist_pass: "~ι~~ηθ- like ποιεω";
    stem pluperfect_act: "~ε%ε~~ηκ- like εὐλογεω";
    ind_act_1: "~ι~α~ω", _, "~ι~α~οῡμεν";
    ind_act_2: "~ι~α~εῑς", _, _;
    ind_act_3: "~ι~α~εῑ", "~ι~α~εῑτον", "~ι~α~οῡσῐ|~ι~α~οῡσῐν";
    ind_mid_1: "~ι~α~οῡμαι", _, "~ι~α~ουμεθᾰ";
    ind_mid_2: "~ι~α~εῑ|~ι~α~ῃ", "~ι~α~εῑσθον", _;
    ind_pass_2: _, _, "~ι~α~εῑσθε";
    ind_pass_3: "~ι~α~εῑται", _, "~ι~α~οῡνται";
    subj_act_1: "~ι~α~ω", _, "~ι~α~ωμεν";
    subj_act_2: "~ι~α~ῃς", _, _;
    subj_act_3: "~ι~α~ῃ", "~ι~α~ητον", "~ι~α~ωσῐ|~ι~α~ωσῐν";
    subj_mid_1: _, _, "~ι~α~ωμεθᾰ";
    subj_mid_2: _, "~ι~α~ησθον", _;
    subj_mid_3: "~ι~α~ηται", _, "~ι~α~ωνται";
    subj_pass_1: "~ι~α~ωμαι", _, _;
    subj_pass_2: _, _, "~ι~α~ησθε";
    opt_act_1: "~ι~α~οιην|~ι~α~οῑμῐ", _, "~ι~α~οῑμεν|~ι~α~οιημεν";
    opt_act_2: "~ι~α~οιης|~ι~α~οῑς", "~ι~α~οῑτον|~ι~α~οιητον", "~ι~α~οῑτε|~ι~α~οιητε";
    opt_act_3: "~ι~α~οιη|~ι~α~οῑ", "~ι~α~οιτην|~ι~α~οιητην", "~ι~α~οῑεν|~ι~α~οιησᾰν";
    opt_mid_1: "~ι~α~οιμην", _, "~ι~α~οιμεθᾰ";
    opt_mid_2: "~ι~α~οῑο", "~ι~α~οῑσθον", _;
    opt_mid_3: "~ι~α~οῑτο", "~ι~α~οισθην", "~ι~α~οῑντο";
    opt_pass_2: _, _, "~ι~α~οῑσθε";
    imp_act_2: "~ι~α~ει", "~ι~α~εῑτον", "~ι~α~εῑτε";
    imp_mid_2: "~ι~α~οῡ", _, "~ι~α~εῑσθε";
    imp_mid_3: "~ι~α~εισθω", _, "~ι~α~εισθων";
    imp_pass_2: _, _, "~ι~α~εῑσθε";
    imp_pass_3: _, "~ι~α~εισθων", "~ι~α~εισθων";
    future_ind_act_1: "~ι~α~ω", _, "~ι~α~οῡμεν";
    future_ind_act_2: "~ι~α~εῑς", "~ι~α~εῑτον", "~ι~α~εῑτε";
    future_ind_act_3: _, _, "~ι~α~οῡσῐ|~ι~α~οῡσῐν";
    future_ind_mid_1: "~ι~α~οῡμαι", _, "~ι~α~ουμεθᾰ";
    future_ind_mid_2: "~ι~α~εῑ|~ι~α~ῃ", "~ι~α~εῑσθον", _;
    future_ind_mid_3: "~ι~α~εῑται", _, _;
    future_opt_act_1: "~ι~α~οιην|~ι~α~οῑμῐ", _, "~ι~α~οῑμεν|~ι~α~οιημεν";
    future_opt_act_2: "~ι~α~οιης|~ι~α~οῑς", "~ι~α~οῑτον|~ι~α~οιητον", "~ι~α~οῑτε|~ι~α~οιητε";
    future_opt_act_3: "~ι~α~οιη|~ι~α~οῑ", "~ι~α~οιτην|~ι~α~οιητην", "~ι~α~οῑεν|~ι~α~οιησᾰν";
    future_opt_mid_1: "~ι~α~οιμην", _, "~ι~α~οιμεθᾰ";
    future_opt_mid_2: "~ι~α~οῑο", _, _;
    future_opt_mid_3: _, "~ι~α~οισθην", _;
    aorist_subj_act_1: "~ι~α~εσω", _, _;
    aorist_subj_act_2: "~ι~α~εσῃς", "~ι~α~εσητον", _;
    aorist_subj_act_3: _, _, "~ι~α~εσωσῐ|~ι~α~εσωσῐν";
    aorist_subj_mid_1: "~ι~α~εσωμαι", _, _;
    aorist_subj_mid_2: "~ι~α~εσῃ", "~ι~α~εσησθον", _;
    aorist_opt_act_1: "~ι~α~εσαιμῐ", _, "~ι~α~εσαιμεν";
    aorist_opt_act_2: "~ι~α~εσειᾰς|~ι~α~εσαις", _, _;
    aorist_opt_act_3: "~ι~α~εσειε|~ι~α~εσειεν|~ι~α~εσαι", "~ι~α~εσαιτην", "~ι~α~εσειᾰν|~ι~α~εσαιεν";
    aorist_opt_mid_1: "~ι~α~εσαιμην", _, _;
    aorist_opt_mid_2: "~ι~α~εσαιο", _, _;
    aorist_imp_act_2: "~ι~α~εσον", _, "~ι~α~εσᾰτε";
    aorist_imp_act_3: _, _, "~ι~α~εσᾰντων";
    aorist_imp_mid_2: "~ι~α~εσαι", "~ι~α~εσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ι~α~εσᾰσθων";
    perfect_ind_act_1: "~ι%ε~~ηκᾰ", _, "~ι%ε~~ηκᾰμεν";
    perfect_ind_act_2: _, "~ι%ε~~ηκᾰτον", _;
    perfect_ind_act_3: "~ι%ε~~ηκε|~ι%ε~~ηκεν", _, "~ι%ε~~ηκᾱσῐ|~ι%ε~~ηκᾱσῐν";
    perfect_ind_mid_1: "~ι%ε~~ημαι", _, "~ι%ε~~ημεθᾰ";
    perfect_ind_mid_2: "~ι%ε~~ησαι", "~ι%ε~~ησθον", _;
    perfect_ind_mid_3: "~ι%ε~~ηται", _, "~ι%ε~~ηνται";
    perfect_ind_pass_2: _, _, "~ι%ε~~ησθε";
    perfect_subj_act_1: "~ι%ε~~ηκω", _, _;
    perfect_subj_act_2: "~ι%ε~~ηκῃς", "~ι%ε~~ηκητον", "~ι%ε~~ηκητε";
    perfect_subj_act_3: "~ι%ε~~ηκῃ", _, "~ι%ε~~ηκωσῐ|~ι%ε~~ηκωσῐν";
    perfect_opt_act_1: "~ι%ε~~ηκοιμῐ|~ι%ε~~ηκοιην", _, "~ι%ε~~ηκοιμεν";
    perfect_opt_act_2: "~ι%ε~~ηκοις|~ι%ε~~ηκοιης", _, _;
    perfect_opt_act_3: "~ι%ε~~ηκοι|~ι%ε~~ηκοιη", "~ι%ε~~ηκοιτην", _;
    perfect_imp_act_2: "~ι%ε~~ηκε", _, "~ι%ε~~ηκετε";
    perfect_imp_act_3: _, _, "~ι%ε~~ηκοντων";
    perfect_imp_mid_2: _, _, "~ι%ε~~ησθε";
    perfect_imp_mid_3: "~ι%ε~~ησθω", _, "~ι%ε~~ησθων";
    perfect_imp_pass_2: "~ι%ε~~ησο", _, "~ι%ε~~ησθε";
    perfect_imp_pass_3: _, "~ι%ε~~ησθων", "~ι%ε~~ησθων";
    pluperfect_mid_1: "~ε%ε~~ημην", _, _;
    pluperfect_mid_2: "~ε%ε~~ησο", "~ε%ε~~ησθον", _;
    pluperfect_mid_3: "~ε%ε~~ητο", _, "~ε%ε~~ηντο";
    pluperfect_pass_1: _, _, "~ε%ε~~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ε%ε~~ησθε";
    pluperfect_pass_3: _, "~ε%ε~~ησθην", _;
    infinitive_pres: "~ι~α~εῑν", _, "~ι~α~εῑσθαι";
    infinitive_fut: "~ι~α~εῑν", _, _;
    infinitive_aor: "~ι~α~εσαι", _, _;
    infinitive_perf: _, "~ι%ε~~ησθαι", _;
    participle_pres: "~ι~α~ων", _, "~ι~α~ουμενος";
    participle_aor: "~ι~α~εσᾱς", "~ι~α~εσᾰμενος", _;
    participle_perf: "~ι%ε~~ηκως", _, "~ι%ε~~ημενος";
    ),
    // Χρη: 1 words (χρη). Like Μῑγνῡμῐ, except in 12 cells: ind_act_1 sg, ind_act_3 sg, subj_act_3 sg, opt_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, ….
    conjugation!("χρη", 1, "~η", like Μῑγνῡμῐ;
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
    // Ἠλθον: 1 words (ἠλθον). Like Ἐπορον, except in 22 cells: ind_act_1 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_3 sg, ….
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
    aorist_opt_act_3: "ἐ~οι", "ἐ~οιτην", "ἐ~οιεν";
    aorist_imp_act_2: "ἐ~ε", _, "ἐ~ετε";
    aorist_imp_act_3: _, _, "ἐ~οντων";
    infinitive_aor: "ἐ~εῑν", _, _;
    participle_aor: "ἐ~ων", _, _;
    ),
    // Ἀρνυμαι: 1 words (ἀρνυμαι). Like Καινῠμαι, except in 30 cells: ind_act_1 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_3 sg, ….
    conjugation!("ἀρνυμαι", 1, "~νυμαι", like Καινῠμαι;
    stem aorist_ind: "~- like ἐρχομαι";
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
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_mid_1: "~οιμην", _, _;
    aorist_opt_mid_2: "~οιο", _, _;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    aorist_imp_mid_2: "~οῡ", "~εσθον", _;
    aorist_imp_mid_3: _, _, "~εσθων";
    infinitive_aor: derived, _, _;
    participle_aor: derived, "~ομενος", _;
    ),
    // Ἐπεφνον: 1 words (ἐπεφνον). Like Ἐπορον, except in 32 cells: ind_act_1 sg, aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, aorist_opt_act_1 sg, ….
    conjugation!("ἐπεφνον", 1, "ἐ~νον", like Ἐπορον;
    stem future_pass: "~ησ- like ποιεω";
    stem aorist_ind: "~ν- like ἐπεφνον";
    ind_act_1: "ἐ~νον", _, _;
    aorist_subj_act_1: "~νω", _, _;
    aorist_subj_act_2: "~νῃς", "~νητον", _;
    aorist_subj_act_3: _, _, "~νωσῐ|~νωσῐν";
    aorist_opt_act_1: "~νοιμῐ", _, "~νοιμεν";
    aorist_opt_act_2: "~νοις", _, _;
    aorist_opt_act_3: "~νοι", "~νοιτην", "~νοιεν";
    aorist_imp_act_2: "~νε", _, "~νετε";
    aorist_imp_act_3: _, _, "~νοντων";
    perfect_ind_mid_1: "~αμαι", _, "~αμεθᾰ";
    perfect_ind_mid_2: "~ασαι", "~ασθον", _;
    perfect_ind_mid_3: "~αται", _, "~ανται";
    perfect_ind_pass_2: _, _, "~ασθε";
    perfect_imp_mid_2: _, _, "~ασθε";
    perfect_imp_mid_3: "~ασθω", _, "~ασθων";
    perfect_imp_pass_2: "~ασο", _, "~ασθε";
    perfect_imp_pass_3: _, "~ασθων", "~ασθων";
    infinitive_aor: "~νεῑν", _, _;
    infinitive_perf: _, "~ᾱσθαι", _;
    participle_aor: "~νων", _, _;
    participle_perf: _, _, "~αμενος";
    ),
    // Μετεκῑᾰθον: 1 words (μετεκῑᾰθον). Like Κᾰθημαι, except in 24 cells: ind_act_1 sg, aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, aorist_opt_act_1 sg, ….
    conjugation!("μετεκῑᾰθον", 1, "~ε~ον", like Κᾰθημαι;
    stem aorist_ind: "~ε~- like εἰσερχομαι";
    ind_act_1: "~ε~ον", _, _;
    aorist_subj_act_1: "~α~ω", _, _;
    aorist_subj_act_2: "~α~ῃς", "~α~ητον", _;
    aorist_subj_act_3: _, _, "~α~ωσῐ|~α~ωσῐν";
    aorist_opt_act_1: "~α~οιμῐ", _, "~α~οιμεν";
    aorist_opt_act_2: "~α~οις", _, _;
    aorist_opt_act_3: "~α~οι", "~α~οιτην", "~α~οιεν";
    aorist_imp_act_2: "~α~ε", _, "~α~ετε";
    aorist_imp_act_3: _, _, "~α~οντων";
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_aor: "~α~εῑν", _, _;
    participle_aor: "~α~ων", _, _;
    ),
    // Πεπνῡμαι: 1 words (πεπνῡμαι). Like Μῑγνῡμῐ, except in 4 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg.
    conjugation!("πεπνῡμαι", 1, "πε~μαι", like Μῑγνῡμῐ;
    stem aorist_ind_pass: "~θην- like ἐχω";
    stem aorist_pass: "~θ- like ποιεω";
    ind_act_1: "πε~μαι", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    ),
    // Περνημῐ: 1 words (περνημῐ). Like Μῑγνῡμῐ, except in 27 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("περνημῐ", 1, "~νημῐ", like Μῑγνῡμῐ;
    stem future_pass: "~ᾰθησ- like ποιεω";
    ind_act_1: "~νημῐ", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_1: "~ᾰσω", _, "~ᾰσομεν";
    future_ind_act_2: "~ᾰσεις", "~ᾰσετον", "~ᾰσετε";
    future_ind_act_3: _, _, "~ᾰσουσῐ|~ᾰσουσῐν";
    future_ind_mid_1: "~ᾰσομαι", _, "~ᾰσομεθᾰ";
    future_ind_mid_2: "~ᾰσῃ|~ᾰσει", "~ᾰσεσθον", _;
    future_ind_mid_3: "~ᾰσεται", _, _;
    future_opt_act_1: "~ᾰσοιμῐ", _, "~ᾰσοιμεν";
    future_opt_act_2: _, "~ᾰσοιτον", "~ᾰσοιτε";
    future_opt_act_3: "~ᾰσοι", "~ᾰσοιτην", "~ᾰσοιεν";
    future_opt_mid_1: "~ᾰσοιμην", _, "~ᾰσοιμεθᾰ";
    future_opt_mid_2: "~ᾰσοιο", _, _;
    future_opt_mid_3: _, "~ᾰσοισθην", _;
    infinitive_fut: "~ᾰᾱν|~ᾱμεν|~ᾱμεναι|~ᾰσμεν|~ᾰσμεναι", _, _;
    ),
    // Ἐπῐστᾰμαι: 1 words (ἐπῐστᾰμαι). Like Δῐακειμαι, except in 38 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, subj_mid_1 sg, ….
    conjugation!("ἐπῐστᾰμαι", 1, "ἐ~ῐ~ᾰμαι", like Δῐακειμαι;
    stem aorist_ind_pass: "~ι~ηθην- like ἀγορευω";
    stem aorist_pass: "~ι~ηθ- like ἐλπῐζω";
    ind_act_1: "ἐ~ῐ~ᾰμαι", _, _;
    ind_mid_1: "ἐ~ι~ᾰμαι", _, "ἐ~ι~ᾰμεθᾰ";
    ind_mid_2: "ἐ~ι~ᾰσαι|ἐ~ι~ᾳ|ἐ~ι~ῃ", "ἐ~ι~ᾰσθον", _;
    subj_mid_1: "ἐ~ι~ωμαι|ἐ~ι~ωμαι", _, "ἐ~ι~ωμεθᾰ";
    subj_mid_2: "ἐ~ι~ῃ", "ἐ~ι~ησθον", _;
    subj_mid_3: "ἐ~ι~ηται", _, "ἐ~ι~ωνται";
    opt_mid_1: "ἐ~ι~αιμην", _, "ἐ~ι~αιμεθᾰ";
    opt_mid_2: "ἐ~ι~αιο", "ἐ~ι~αισθον", _;
    opt_mid_3: "ἐ~ι~αιτο", "ἐ~ι~αισθην", "ἐ~ι~αιντο";
    imp_mid_2: "ἐ~ι~ᾰσο|ἐ~ι~ω", _, "ἐ~ι~ᾰσθε";
    imp_mid_3: "ἐ~ι~ᾰσθω", _, "ἐ~ι~ᾰσθων";
    imperfect_mid_1: "ἠ~ι~ᾰμην", _, "ἠ~ι~ᾰμεθᾰ";
    imperfect_mid_2: "ἠ~ῐ~ω|ἠ~ῐ~ᾰσο", "ἠ~ι~ᾰσθον", _;
    imperfect_mid_3: "ἠ~ι~ᾰτο", "ἠ~ι~ᾰσθην", "ἠ~ι~ᾰντο";
    future_ind_mid_1: "ἐ~ι~ησομαι", _, "ἐ~ι~ησομεθᾰ";
    future_ind_mid_2: "ἐ~ι~ησῃ|ἐ~ι~ησει", "ἐ~ι~ησεσθον", _;
    future_ind_mid_3: "ἐ~ι~ησεται", _, _;
    future_opt_mid_1: "ἐ~ι~ησοιμην", _, "ἐ~ι~ησοιμεθᾰ";
    future_opt_mid_2: "ἐ~ι~ησοιο", _, _;
    future_opt_mid_3: _, "ἐ~ι~ησοισθην", _;
    ),
    // Δειδω: 1 words (δειδω). Like Ἐπορον, except in 59 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("δειδω", 1, "~ιδω", like Ἐπορον;
    stem aorist_ind: "~ισ- like χᾰριζω";
    stem pluperfect_act: "~δοικ- like δειδω";
    ind_act_1: "~ιδω", _, _;
    future_ind_act_1: "~ισω", _, "~ισομεν";
    future_ind_act_2: "~ισεις", "~ισετον", "~ισετε";
    future_ind_act_3: _, _, "~ισουσῐ|~ισουσῐν";
    future_ind_mid_1: "~ισομαι", _, "~ισομεθᾰ";
    future_ind_mid_2: "~ισῃ|~ισει", "~ισεσθον", _;
    future_ind_mid_3: "~ισεται", _, _;
    future_opt_act_1: "~ισοιμῐ", _, "~ισοιμεν";
    future_opt_act_2: _, "~ισοιτον", "~ισοιτε";
    future_opt_act_3: "~ισοι", "~ισοιτην", "~ισοιεν";
    future_opt_mid_1: "~ισοιμην", _, "~ισοιμεθᾰ";
    future_opt_mid_2: "~ισοιο", _, _;
    future_opt_mid_3: _, "~ισοισθην", _;
    aorist_subj_act_1: "~ισω", _, _;
    aorist_subj_act_2: "~ισῃς", "~ισητον", _;
    aorist_subj_act_3: _, _, "~ισωσῐ|~ισωσῐν";
    aorist_opt_act_1: "~ισαιμῐ", _, "~ισαιμεν";
    aorist_opt_act_2: "~ισειᾰς|~ισαις", _, _;
    aorist_opt_act_3: "~ισειε|~ισειεν|~ισαι", "~ισαιτην", "~ισειᾰν|~ισαιεν";
    aorist_imp_act_2: "~ῑσον", _, "~ισᾰτε";
    aorist_imp_act_3: _, _, "~ισᾰντων";
    perfect_ind_act_1: "~δοικᾰ", _, "~δοικᾰμεν";
    perfect_ind_act_2: _, "~δοικᾰτον", _;
    perfect_ind_act_3: "~δοικε|~δοικεν", _, "~δοικᾱσῐ|~δοικᾱσῐν";
    perfect_subj_act_1: "~δοικω", _, _;
    perfect_subj_act_2: "~δοικῃς", "~δοικητον", "~δοικητε";
    perfect_subj_act_3: "~δοικῃ", _, "~δοικωσῐ|~δοικωσῐν";
    perfect_opt_act_1: "~δοικοιμῐ|~δοικοιην", _, "~δοικοιμεν";
    perfect_opt_act_2: "~δοικοις|~δοικοιης", _, _;
    perfect_opt_act_3: "~δοικοι|~δοικοιη", "~δοικοιτην", _;
    perfect_imp_act_2: "~δοικε", _, "~δοικετε";
    perfect_imp_act_3: _, _, "~δοικοντων";
    infinitive_fut: "~ισειν", _, _;
    infinitive_aor: "~ῑσαι", _, _;
    participle_aor: "~ισᾱς", _, _;
    participle_perf: "~δοικως", _, _;
    ),
    // Ἀπαυραω: 1 words (ἀπαυραω). Like Μετεκῑᾰθον, except in 64 cells: ind_act_1 sg, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, ….
    conjugation!("ἀπαυραω", 1, "~αυ~αω", like Μετεκῑᾰθον;
    stem aorist_ind: "~ου~- like εὐλογεω";
    ind_act_1: "~αυ~αω", _, _;
    imperfect_act_1: "~ηυ~ων", _, "~ηυ~ωμεν";
    imperfect_act_2: "~ηυ~ᾱς", "~ηυ~ᾱτον", "~ηυ~ᾱτε";
    imperfect_act_3: "~ηυ~ᾱ", "~ηυ~ᾱτην", "~ηυ~ων";
    imperfect_mid_1: "~ηυ~ωμην", _, "~ηυ~ωμεθᾰ";
    imperfect_mid_2: "~ηυ~ω", _, _;
    imperfect_mid_3: "~ηυ~ᾱτο", _, _;
    imperfect_pass_2: _, "~ηυ~ᾱσθον", "~ηυ~ᾱσθε";
    imperfect_pass_3: _, "~ηυ~ᾱσθην", "~ηυ~ωντο";
    future_ind_act_1: "~ου~ησω", _, "~ου~ησομεν";
    future_ind_act_2: "~ου~ησεις", "~ου~ησετον", "~ου~ησετε";
    future_ind_act_3: _, _, "~ου~ησουσῐ|~ου~ησουσῐν";
    future_ind_mid_1: "~ου~ησομαι", _, "~ου~ησομεθᾰ";
    future_ind_mid_2: "~ου~ησῃ|~ου~ησει", "~ου~ησεσθον", _;
    future_ind_mid_3: "~ου~ησεται", _, _;
    future_opt_act_1: "~ου~ησοιμῐ", _, "~ου~ησοιμεν";
    future_opt_act_2: _, "~ου~ησοιτον", "~ου~ησοιτε";
    future_opt_act_3: "~ου~ησοι", "~ου~ησοιτην", "~ου~ησοιεν";
    future_opt_mid_1: "~ου~ησοιμην", _, "~ου~ησοιμεθᾰ";
    future_opt_mid_2: "~ου~ησοιο", _, _;
    future_opt_mid_3: _, "~ου~ησοισθην", _;
    aorist_subj_act_1: "~ου~ω", _, _;
    aorist_subj_act_2: "~ου~ῃς", "~ου~ητον", _;
    aorist_subj_act_3: _, _, "~ου~ωσῐ|~ου~ωσῐν";
    aorist_subj_mid_1: "~ου~ωμαι", _, _;
    aorist_subj_mid_2: "~ου~ῃ", "~ου~ησθον", _;
    aorist_opt_act_1: "~ου~αιμῐ", _, "~ου~αιμεν";
    aorist_opt_act_2: "~ου~ειᾰς|~ου~αις", _, _;
    aorist_opt_act_3: "~ου~ειε|~ου~ειεν|~ου~αι", "~ου~αιτην", "~ου~ειᾰν|~ου~αιεν";
    aorist_opt_mid_1: "~ου~αιμην", _, _;
    aorist_opt_mid_2: "~ου~αιο", _, _;
    aorist_imp_act_2: "~ου~ον", _, "~ου~ᾰτε";
    aorist_imp_act_3: _, _, "~ου~ᾰντων";
    aorist_imp_mid_2: "~ου~αι", "~ου~ᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ου~ᾰσθων";
    infinitive_fut: "~ου~ησειν", _, _;
    infinitive_aor: "~οῡ~αι", _, _;
    participle_aor: "~ου~ᾱς", "~ου~ᾰμενος", _;
    ),
    // Ἐπῐπειθομαι: 1 words (ἐπῐπειθομαι). Like Δῐακειμαι, except in 31 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, subj_mid_1 sg, ….
    conjugation!("ἐπῐπειθομαι", 1, "~ῐ~θομαι", like Δῐακειμαι;
    stem imperfect: "~ε~θ- like ὑποκοριζομαι";
    ind_act_1: "~ῐ~θομαι", _, _;
    ind_mid_1: "~ι~θομαι", _, "~ι~θομεθᾰ";
    ind_mid_2: "~ι~θῃ|~ι~θει", "~ι~θεσθον", _;
    subj_mid_1: "~ι~θωμαι", _, "~ι~θωμεθᾰ";
    subj_mid_2: "~ι~θῃ", "~ι~θησθον", _;
    subj_mid_3: "~ι~θηται", _, "~ι~θωνται";
    opt_mid_1: "~ι~θοιμην", _, "~ι~θοιμεθᾰ";
    opt_mid_2: "~ι~θοιο", "~ι~θοισθον", _;
    opt_mid_3: "~ι~θοιτο", "~ι~θοισθην", "~ι~θοιντο";
    imp_mid_2: "~ι~θου", _, "~ι~θεσθε";
    imp_mid_3: "~ι~θεσθω", _, "~ι~θεσθων";
    future_ind_mid_1: "~ι~σομαι", _, "~ι~σομεθᾰ";
    future_ind_mid_2: "~ι~σῃ|~ι~σει", "~ι~σεσθον", _;
    future_ind_mid_3: "~ι~σεται", _, _;
    future_opt_mid_1: "~ι~σοιμην", _, "~ι~σοιμεθᾰ";
    future_opt_mid_2: "~ι~σοιο", _, _;
    future_opt_mid_3: _, "~ι~σοισθην", _;
    ),
    // Ἐπισευω: 1 words (ἐπισευω). Like Κᾰθημαι, except in 28 cells: ind_act_1 sg, perfect_ind_mid_1 sg, perfect_ind_mid_1 pl, perfect_ind_mid_2 sg, perfect_ind_mid_2 du, perfect_ind_mid_3 sg, ….
    conjugation!("ἐπισευω", 1, "~ι~ευω", like Κᾰθημαι;
    ind_act_1: "~ι~ευω", _, _;
    perfect_ind_mid_1: "~ι~ῠμαι", _, "~ι~ῠμεθᾰ";
    perfect_ind_mid_2: "~ι~ῠσαι", "~ι~ῠσθον", _;
    perfect_ind_mid_3: "~ι~ῠται", _, "~ι~ῠνται";
    perfect_ind_pass_2: _, _, "~ι~ῠσθε";
    perfect_imp_mid_2: _, _, "~ι~ῠσθε";
    perfect_imp_mid_3: "~ι~ῠσθω", _, "~ι~ῠσθων";
    perfect_imp_pass_2: "~ι~ῠσο", _, "~ι~ῠσθε";
    perfect_imp_pass_3: _, "~ι~ῠσθων", "~ι~ῠσθων";
    pluperfect_mid_1: "~ε~ῠμην", _, derived;
    pluperfect_mid_2: "~ε~ῠσο", "~ε~ῠσθον", derived;
    pluperfect_mid_3: "~ε~ῠτο", derived, "~ε~ῠντο";
    pluperfect_pass_1: _, _, "~ε~ῠμεθᾰ";
    pluperfect_pass_2: _, _, "~ε~ῠσθε";
    pluperfect_pass_3: _, "~ε~ῠσθην", _;
    infinitive_perf: _, "~ι~ῠσθαι", _;
    participle_perf: _, "~ι~υμενος", _;
    ),
    // Δειμᾰτοω: 1 words (δειμᾰτοω). Like Κᾰταφρονεω, except in 124 cells: ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("δειμᾰτοω", 1, "~ᾰ~οω", like Κᾰταφρονεω;
    stem future_pass: "~α~ωθησ- like ποιεω";
    stem aorist_ind: "~α~ωσ- like ποιεω";
    stem aorist_ind_pass: "~α~ωθην- like ἐχω";
    stem aorist_pass: "~α~ωθ- like ποιεω";
    stem pluperfect_act: "~α~ωκ- like χρημᾰτῐζω";
    ind_act_2: "~α~οῑς", _, _;
    ind_act_3: "~α~οῑ", "~α~οῡτον", _;
    ind_mid_2: "~α~οῑ", "~α~οῡσθον", _;
    ind_pass_2: _, _, "~α~οῡσθε";
    ind_pass_3: "~α~οῡται", _, _;
    subj_act_2: "~α~οῑς", _, _;
    subj_act_3: "~α~οῑ", "~α~ωτον", _;
    subj_mid_2: _, "~α~ωσθον", _;
    subj_mid_3: "~α~ωται", _, _;
    subj_pass_2: _, _, "~α~ωσθε";
    imp_act_2: "~α~ου", "~α~οῡτον", "~α~οῡτε";
    imp_mid_2: _, _, "~α~οῡσθε";
    imp_mid_3: "~α~ουσθω", _, "~α~ουσθων";
    imp_pass_2: _, _, "~α~οῡσθε";
    imp_pass_3: _, "~α~ουσθων", "~α~ουσθων";
    imperfect_act_2: "ἐ~α~ους", "ἐ~α~οῡτον", "ἐ~α~οῡτε";
    imperfect_act_3: "ἐ~α~ου", "ἐ~α~ουτην", _;
    imperfect_mid_3: "ἐ~α~οῡτο", _, _;
    imperfect_pass_2: _, "ἐ~α~οῡσθον", "ἐ~α~οῡσθε";
    imperfect_pass_3: _, "ἐ~α~ουσθην", _;
    future_ind_act_1: "~α~ωσω", _, "~α~ωσομεν";
    future_ind_act_2: "~α~ωσεις", "~α~ωσετον", "~α~ωσετε";
    future_ind_act_3: _, _, "~α~ωσουσῐ|~α~ωσουσῐν";
    future_ind_mid_1: "~α~ωσομαι", _, "~α~ωσομεθᾰ";
    future_ind_mid_2: "~α~ωσῃ|~α~ωσει", "~α~ωσεσθον", _;
    future_ind_mid_3: "~α~ωσεται", _, _;
    future_opt_act_1: "~α~ωσοιμῐ", _, "~α~ωσοιμεν";
    future_opt_act_2: _, "~α~ωσοιτον", "~α~ωσοιτε";
    future_opt_act_3: "~α~ωσοι", "~α~ωσοιτην", "~α~ωσοιεν";
    future_opt_mid_1: "~α~ωσοιμην", _, "~α~ωσοιμεθᾰ";
    future_opt_mid_2: "~α~ωσοιο", _, _;
    future_opt_mid_3: _, "~α~ωσοισθην", _;
    aorist_subj_act_1: "~α~ωσω", _, _;
    aorist_subj_act_2: "~α~ωσῃς", "~α~ωσητον", _;
    aorist_subj_act_3: _, _, "~α~ωσωσῐ|~α~ωσωσῐν";
    aorist_subj_mid_1: "~α~ωσωμαι", _, _;
    aorist_subj_mid_2: "~α~ωσῃ", "~α~ωσησθον", _;
    aorist_opt_act_1: "~α~ωσαιμῐ", _, "~α~ωσαιμεν";
    aorist_opt_act_2: "~α~ωσειᾰς|~α~ωσαις", _, _;
    aorist_opt_act_3: "~α~ωσειε|~α~ωσειεν|~α~ωσαι", "~α~ωσαιτην", "~α~ωσειᾰν|~α~ωσαιεν";
    aorist_opt_mid_1: "~α~ωσαιμην", _, _;
    aorist_opt_mid_2: "~α~ωσαιο", _, _;
    aorist_imp_act_2: "~α~ωσον", _, "~α~ωσᾰτε";
    aorist_imp_act_3: _, _, "~α~ωσᾰντων";
    aorist_imp_mid_2: "~α~ωσαι", "~α~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~α~ωσᾰσθων";
    perfect_ind_act_1: "%ε~α~ωκᾰ", _, "%ε~α~ωκᾰμεν";
    perfect_ind_act_2: _, "%ε~α~ωκᾰτον", _;
    perfect_ind_act_3: "%ε~α~ωκε|%ε~α~ωκεν", _, "%ε~α~ωκᾱσῐ|%ε~α~ωκᾱσῐν";
    perfect_ind_mid_1: "%ε~α~ωμαι", _, "%ε~α~ωμεθᾰ";
    perfect_ind_mid_2: "%ε~α~ωσαι", "%ε~α~ωσθον", _;
    perfect_ind_mid_3: "%ε~α~ωται", _, "%ε~α~ωνται";
    perfect_ind_pass_2: _, _, "%ε~α~ωσθε";
    perfect_subj_act_1: "%ε~α~ωκω", _, _;
    perfect_subj_act_2: "%ε~α~ωκῃς", "%ε~α~ωκητον", "%ε~α~ωκητε";
    perfect_subj_act_3: "%ε~α~ωκῃ", _, "%ε~α~ωκωσῐ|%ε~α~ωκωσῐν";
    perfect_opt_act_1: "%ε~α~ωκοιμῐ|%ε~α~ωκοιην", _, "%ε~α~ωκοιμεν";
    perfect_opt_act_2: "%ε~α~ωκοις|%ε~α~ωκοιης", _, _;
    perfect_opt_act_3: "%ε~α~ωκοι|%ε~α~ωκοιη", "%ε~α~ωκοιτην", _;
    perfect_imp_act_2: "%ε~α~ωκε", _, "%ε~α~ωκετε";
    perfect_imp_act_3: _, _, "%ε~α~ωκοντων";
    perfect_imp_mid_2: _, _, "%ε~α~ωσθε";
    perfect_imp_mid_3: "%ε~α~ωσθω", _, "%ε~α~ωσθων";
    perfect_imp_pass_2: "%ε~α~ωσο", _, "%ε~α~ωσθε";
    perfect_imp_pass_3: _, "%ε~α~ωσθων", "%ε~α~ωσθων";
    pluperfect_mid_1: "ἐ%ε~α~ωμην", _, _;
    pluperfect_mid_2: "ἐ%ε~α~ωσο", "ἐ%ε~α~ωσθον", _;
    pluperfect_mid_3: "ἐ%ε~α~ωτο", _, "ἐ%ε~α~ωντο";
    pluperfect_pass_1: _, _, "ἐ%ε~α~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~α~ωσθε";
    pluperfect_pass_3: _, "ἐ%ε~α~ωσθην", _;
    infinitive_pres: "~α~οῡν", _, "~α~οῡσθαι";
    infinitive_fut: "~α~ωσειν", _, _;
    infinitive_aor: "~α~ωσαι", _, _;
    infinitive_perf: _, "%ε~α~ωσθαι", _;
    participle_aor: "~α~ωσᾱς", "~α~ωσᾰμενος", _;
    participle_perf: "%ε~α~ωκως", _, "%ε~α~ωμενος";
    ),
    // Κορυσσω: 1 words (κορυσσω). Like Ἐπορον, except in 55 cells: ind_act_1 sg, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, ….
    conjugation!("κορυσσω", 1, "~υσσω", like Ἐπορον;
    stem aorist_ind: "~ῠσσ- like ποιεω";
    ind_act_1: "~υσσω", _, _;
    imperfect_act_1: "~ῠσσον", _, "~ῠσσομεν";
    imperfect_act_2: "~ῠσσες", "~ῠσσετον", "~ῠσσετε";
    imperfect_act_3: "~ῠσσε|~ῠσσεν", "~ῠσσετην", "~ῠσσον";
    imperfect_mid_1: "~ῠσσομην", _, "~ῠσσομεθᾰ";
    imperfect_mid_2: "~ῠσσου", _, _;
    imperfect_mid_3: "~ῠσσετο", _, _;
    imperfect_pass_2: _, "~ῠσσεσθον", "~ῠσσεσθε";
    imperfect_pass_3: _, "~ῠσσεσθην", "~ῠσσοντο";
    aorist_subj_act_1: "~ῠσσω", _, _;
    aorist_subj_act_2: "~ῠσσῃς", "~ῠσσητον", _;
    aorist_subj_act_3: _, _, "~ῠσσωσῐ|~ῠσσωσῐν";
    aorist_subj_mid_1: "~ῠσσωμαι", _, _;
    aorist_subj_mid_2: "~ῠσσῃ", "~ῠσσησθον", _;
    aorist_opt_act_1: "~ῠσσαιμῐ", _, "~ῠσσαιμεν";
    aorist_opt_act_2: "~ῠσσειᾰς|~ῠσσαις", _, _;
    aorist_opt_act_3: "~ῠσσειε|~ῠσσειεν|~ῠσσαι", "~ῠσσαιτην", "~ῠσσειᾰν|~ῠσσαιεν";
    aorist_opt_mid_1: "~ῠσσαιμην", _, _;
    aorist_opt_mid_2: "~ῠσσαιο", _, _;
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
    // Κεκᾰδον: 1 words (κεκᾰδον). Like Μετεκῑᾰθον, except in 31 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("κεκᾰδον", 1, "~ᾰ~ον", like Μετεκῑᾰθον;
    stem aorist_ind: "~ᾰ~- like πᾰρεχω";
    ind_act_1: "~ᾰ~ον", _, _;
    future_ind_act_1: "~α~ησω", _, "~α~ησομεν";
    future_ind_act_2: "~α~ησεις", "~α~ησετον", "~α~ησετε";
    future_ind_act_3: _, _, "~α~ησουσῐ|~α~ησουσῐν";
    aorist_subj_act_1: "~ᾰ~ω", _, _;
    aorist_subj_act_2: "~ᾰ~ῃς", "~ᾰ~ητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ῃ", "~ᾰ~ησθον", _;
    aorist_opt_act_1: "~ᾰ~οιμῐ", _, "~ᾰ~οιμεν";
    aorist_opt_act_2: "~ᾰ~οις", _, _;
    aorist_opt_act_3: "~ᾰ~οι", "~ᾰ~οιτην", "~ᾰ~οιεν";
    aorist_opt_mid_1: "~ᾰ~οιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~οιο", _, _;
    aorist_imp_act_2: "~ᾰ~ε", _, "~ᾰ~ετε";
    aorist_imp_act_3: _, _, "~ᾰ~οντων";
    aorist_imp_mid_2: "~ᾰ~οῡ", "~ᾰ~εσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~εσθων";
    infinitive_aor: "~ᾰ~εῑν", _, _;
    participle_aor: "~ᾰ~ων", "~ᾰ~ομενος", _;
    ),
    // Κᾰτᾰκλαω: 1 words (κᾰτᾰκλαω). Like Ξυνναυμᾰχεω, except in 129 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 du, ….
    conjugation!("κᾰτᾰκλαω", 1, "~ᾰ~ᾰ~αω", like Ξυνναυμᾰχεω;
    stem aorist_ind: "~ᾰ~ε~ᾰσ- like εὐλογεω";
    stem aorist_ind_pass: "~α~ε~ασθην- like ἐλπω";
    stem aorist_pass: "~ᾰ~α~ασθ- like ποιεω";
    ind_act_1: "~ᾰ~α~ω", _, "~ᾰ~α~ωμεν";
    ind_act_2: "~ᾰ~α~ᾳς", derived, _;
    ind_act_3: "~ᾰ~α~ᾳ", "~ᾰ~α~ᾱτον", "~ᾰ~α~ωσῐ|~ᾰ~α~ωσῐν";
    ind_mid_1: "~ᾰ~α~ωμαι", _, "~ᾰ~α~ωμεθᾰ";
    ind_mid_2: "~ᾰ~α~ᾳ", "~ᾰ~α~ᾱσθον", _;
    ind_pass_2: _, _, "~ᾰ~α~ᾱσθε";
    ind_pass_3: "~ᾰ~α~ᾱται", _, "~ᾰ~α~ωνται";
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
    opt_mid_2: "~ᾰ~α~ῳο", "~ᾰ~α~ῳσθον", _;
    opt_mid_3: "~ᾰ~α~ῳτο", "~ᾰ~α~ῳσθην", "~ᾰ~α~ῳντο";
    opt_pass_2: _, _, "~ᾰ~α~ῳσθε";
    imp_act_2: "~ᾰ~α~ᾱ", "~ᾰ~α~ᾱτον", "~ᾰ~α~ᾱτε";
    imp_act_3: derived, _, derived;
    imp_mid_2: "~ᾰ~α~ω", _, "~ᾰ~α~ᾱσθε";
    imp_mid_3: "~ᾰ~α~ᾱσθω", _, "~ᾰ~α~ᾱσθων";
    imp_pass_2: _, _, "~ᾰ~α~ᾱσθε";
    imp_pass_3: _, "~ᾰ~α~ᾱσθων", "~ᾰ~α~ᾱσθων";
    imperfect_act_1: "~α~ε~ων", _, "~α~ε~ωμεν";
    imperfect_act_2: "~α~ε~ᾱς", "~α~ε~ᾱτον", "~α~ε~ᾱτε";
    imperfect_act_3: "~α~ε~ᾱ", "~α~ε~ᾱτην", "~α~ε~ων";
    imperfect_mid_1: "~α~ε~ωμην", _, "~α~ε~ωμεθᾰ";
    imperfect_mid_2: "~α~ε~ω", _, _;
    imperfect_mid_3: "~α~ε~ᾱτο", _, _;
    imperfect_pass_2: _, "~α~ε~ᾱσθον", "~α~ε~ᾱσθε";
    imperfect_pass_3: _, "~α~ε~ᾱσθην", "~α~ε~ωντο";
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: _, _, derived;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: _, derived, derived;
    future_opt_act_3: derived, derived, derived;
    aorist_subj_act_1: "~ᾰ~α~ᾰσω", _, _;
    aorist_subj_act_2: "~ᾰ~α~ᾰσῃς", "~ᾰ~α~ᾰσητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~α~ᾰσωσῐ|~ᾰ~α~ᾰσωσῐν";
    aorist_subj_mid_1: "~ᾰ~α~ᾰσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~α~ᾰσῃ", "~ᾰ~α~ᾰσησθον", _;
    aorist_opt_act_1: "~ᾰ~α~ᾰσαιμῐ", _, "~ᾰ~α~ᾰσαιμεν";
    aorist_opt_act_2: "~ᾰ~α~ᾰσειᾰς|~ᾰ~α~ᾰσαις", _, _;
    aorist_opt_act_3: "~ᾰ~α~ᾰσειε|~ᾰ~α~ᾰσειεν|~ᾰ~α~ᾰσαι", "~ᾰ~α~ᾰσαιτην", "~ᾰ~α~ᾰσειᾰν|~ᾰ~α~ᾰσαιεν";
    aorist_opt_mid_1: "~ᾰ~α~ᾰσαιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~α~ᾰσαιο", _, _;
    aorist_imp_act_2: "~ᾰ~α~ᾰσον", _, "~ᾰ~α~ᾰσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~α~ᾰσᾰντων";
    aorist_imp_mid_2: "~ᾰ~α~ᾰσαι", "~ᾰ~α~ᾰσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~α~ᾰσᾰσθων";
    perfect_ind_mid_1: "~ᾰ~ᾰ%ε~ᾰσμαι", _, "~ᾰ~ᾰ%ε~ᾰσμεθᾰ";
    perfect_ind_mid_2: "~ᾰ~ᾰ%ε~ᾰσαι", "~ᾰ~ᾰ%ε~ᾰσθον", _;
    perfect_ind_mid_3: "~ᾰ~ᾰ%ε~ᾰσται", _, "~ᾰ~ᾰ%ε~ᾰσᾰται";
    perfect_ind_pass_2: _, _, "~ᾰ~ᾰ%ε~ᾰσθε";
    perfect_imp_mid_2: _, _, "~ᾰ~ᾰ%ε~ᾰσθε";
    perfect_imp_mid_3: "~ᾰ~ᾰ%ε~ᾰσθω", _, "~ᾰ~ᾰ%ε~ᾰσθων";
    perfect_imp_pass_2: "~ᾰ~ᾰ%ε~ᾰσο", _, "~ᾰ~ᾰ%ε~ᾰσθε";
    perfect_imp_pass_3: _, "~ᾰ~ᾰ%ε~ᾰσθων", "~ᾰ~ᾰ%ε~ᾰσθων";
    infinitive_pres: "~ᾰ~α~ᾱν", _, "~ᾰ~α~ᾱσθαι";
    infinitive_fut: derived, _, _;
    infinitive_aor: "~ᾰ~α~ᾰσαι", _, _;
    infinitive_perf: _, "~ᾰ~ᾰ%ε~ᾰσθαι", _;
    participle_pres: "~ᾰ~α~ων", _, "~ᾰ~α~ωμενος";
    participle_aor: "~ᾰ~α~ᾰσᾱς", "~ᾰ~α~ᾰσᾰμενος", _;
    participle_perf: _, _, "~ᾰ~ᾰ%ε~ᾰσμενος";
    ),
    // Ἀγωνιζομαι: 1 words (ἀγωνιζομαι). Like Κεῑμαι, except in 66 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_3 pl, ….
    conjugation!("ἀγωνιζομαι", 1, "ἀ~ιζομαι", like Κεῑμαι;
    ind_act_1: "ἀ~ιζομαι", _, _;
    ind_mid_1: "ἀ~ῐζομαι", _, "ἀ~ῐζομεθᾰ";
    ind_mid_2: "ἀ~ῐζῃ|ἀ~ῐζει", "ἀ~ῐζεσθον", _;
    ind_mid_3: _, _, derived;
    subj_mid_1: "ἀ~ῐζωμαι", _, "ἀ~ῐζωμεθᾰ";
    subj_mid_2: "ἀ~ῐζῃ", "ἀ~ῐζησθον", _;
    subj_mid_3: "ἀ~ῐζηται", _, "ἀ~ῐζωνται";
    opt_mid_1: "ἀ~ῐζοιμην", _, "ἀ~ῐζοιμεθᾰ";
    opt_mid_2: "ἀ~ῐζοιο", "ἀ~ῐζοισθον", derived;
    opt_mid_3: "ἀ~ῐζοιτο", "ἀ~ῐζοισθην", "ἀ~ῐζοιντο";
    imp_mid_2: "ἀ~ῐζου", _, "ἀ~ῐζεσθε";
    imp_mid_3: "ἀ~ῐζεσθω", _, "ἀ~ῐζεσθων";
    imperfect_mid_1: "ἠ~ῐζομην", _, "ἠ~ῐζομεθᾰ";
    imperfect_mid_2: "ἠ~ῐζου", "ἠ~ῐζεσθον", _;
    imperfect_mid_3: "ἠ~ῐζετο", "ἠ~ῐζεσθην", "ἠ~ῐζοντο";
    future_ind_mid_1: "ἀ~ῐοῡμαι", _, "ἀ~ῐουμεθᾰ";
    future_ind_mid_2: "ἀ~ῐεῑ|ἀ~ῐῃ", "ἀ~ῐεῑσθον", _;
    future_ind_mid_3: "ἀ~ῐεῑται", _, _;
    future_opt_mid_1: "ἀ~ῐοιμην", _, "ἀ~ῐοιμεθᾰ";
    future_opt_mid_2: "ἀ~ῐοῑο", _, _;
    future_opt_mid_3: _, "ἀ~ῐοισθην", _;
    aorist_ind_mid_1: "ἠ~ῐσᾰμην", _, _;
    aorist_ind_mid_2: "ἠ~ῐσω", "ἠ~ῐσᾰσθον", _;
    aorist_subj_mid_1: "ἀ~ῐσωμαι", _, _;
    aorist_subj_mid_2: "ἀ~ῐσῃ", "ἀ~ῐσησθον", _;
    aorist_opt_mid_1: "ἀ~ῐσαιμην", _, _;
    aorist_opt_mid_2: "ἀ~ῐσαιο", _, _;
    aorist_imp_mid_2: "ἀ~ῐσαι", "ἀ~ῐσᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἀ~ῐσᾰσθων";
    perfect_ind_mid_1: "ἠ~ῐσμαι", _, "ἠ~ῐσμεθᾰ";
    perfect_ind_mid_2: "ἠ~ῐσαι", "ἠ~ῐσθον", _;
    perfect_ind_mid_3: "ἠ~ῐσται", _, "ἠ~ῐδᾰται";
    perfect_imp_mid_2: _, _, "ἠ~ῐσθε";
    perfect_imp_mid_3: "ἠ~ῐσθω", _, "ἠ~ῐσθων";
    pluperfect_mid_1: "ἠ~ῐσμην", _, _;
    pluperfect_mid_2: "ἠ~ῐσο", "ἠ~ῐσθον", _;
    pluperfect_mid_3: "ἠ~ῐστο", _, "ἠ~ῐδᾰτο";
    participle_aor: _, "ἀ~ῐσᾰμενος", _;
    ),
    // Ἀπιστεω: 1 words (ἀπιστεω). Like Ἐκφερω, except in 118 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἀπιστεω", 1, "ἀ~ι~εω", like Ἐκφερω;
    stem future_pass: "~ῐ~ηθησ- like ᾰντᾰποκρῑνομαι";
    stem aorist_ind: "~ῐ~ησ- like ἐλπῐζω";
    ind_act_1: "ἀ~ῐ~ω", _, "ἀ~ῐ~οῡμεν";
    ind_act_2: "ἀ~ῐ~εῑς", _, _;
    ind_act_3: "ἀ~ῐ~εῑ", "ἀ~ῐ~εῑτον", "ἀ~ῐ~οῡσῐ|ἀ~ῐ~οῡσῐν";
    ind_mid_1: "ἀ~ῐ~οῡμαι", _, "ἀ~ῐ~ουμεθᾰ";
    ind_mid_2: "ἀ~ῐ~εῑ|ἀ~ῐ~ῃ", "ἀ~ῐ~εῑσθον", _;
    ind_pass_2: _, _, "ἀ~ῐ~εῑσθε";
    ind_pass_3: "ἀ~ῐ~εῑται", _, "ἀ~ῐ~οῡνται";
    subj_act_1: "ἀ~ῐ~ω", _, "ἀ~ῐ~ωμεν";
    subj_act_2: "ἀ~ῐ~ῃς", _, _;
    subj_act_3: "ἀ~ῐ~ῃ", "ἀ~ῐ~ητον", "ἀ~ῐ~ωσῐ|ἀ~ῐ~ωσῐν";
    subj_mid_1: _, _, "ἀ~ῐ~ωμεθᾰ";
    subj_mid_2: _, "ἀ~ῐ~ησθον", _;
    subj_mid_3: "ἀ~ῐ~ηται", _, "ἀ~ῐ~ωνται";
    subj_pass_1: "ἀ~ῐ~ωμαι", _, _;
    subj_pass_2: _, _, "ἀ~ῐ~ησθε";
    opt_act_1: "ἀ~ῐ~οιην|ἀ~ῐ~οῑμῐ", _, "ἀ~ῐ~οῑμεν|ἀ~ῐ~οιημεν";
    opt_act_2: "ἀ~ῐ~οιης|ἀ~ῐ~οῑς", "ἀ~ῐ~οῑτον|ἀ~ῐ~οιητον", "ἀ~ῐ~οῑτε|ἀ~ῐ~οιητε";
    opt_act_3: "ἀ~ῐ~οιη|ἀ~ῐ~οῑ", "ἀ~ῐ~οιτην|ἀ~ῐ~οιητην", "ἀ~ῐ~οῑεν|ἀ~ῐ~οιησᾰν";
    opt_mid_1: "ἀ~ῐ~οιμην", _, "ἀ~ῐ~οιμεθᾰ";
    opt_mid_2: "ἀ~ῐ~οῑο", "ἀ~ῐ~οῑσθον", _;
    opt_mid_3: "ἀ~ῐ~οῑτο", "ἀ~ῐ~οισθην", "ἀ~ῐ~οῑντο";
    opt_pass_2: _, _, "ἀ~ῐ~οῑσθε";
    imp_act_2: "ἀ~ῐ~ει", "ἀ~ῐ~εῑτον", "ἀ~ῐ~εῑτε";
    imp_mid_2: "ἀ~ῐ~οῡ", _, "ἀ~ῐ~εῑσθε";
    imp_mid_3: "ἀ~ῐ~εισθω", _, "ἀ~ῐ~εισθων";
    imp_pass_2: _, _, "ἀ~ῐ~εῑσθε";
    imp_pass_3: _, "ἀ~ῐ~εισθων", "ἀ~ῐ~εισθων";
    imperfect_act_1: "ἠ~ῐ~ουν", _, "ἠ~ῐ~οῡμεν";
    imperfect_act_2: "ἠ~ῐ~εις", "ἠ~ῐ~εῑτον", "ἠ~ῐ~εῑτε";
    imperfect_act_3: "ἠ~ῐ~ει", "ἠ~ῐ~ειτην", "ἠ~ῐ~ουν";
    imperfect_mid_1: "ἠ~ῐ~ουμην", _, "ἠ~ῐ~ουμεθᾰ";
    imperfect_mid_2: "ἠ~ῐ~οῡ", _, _;
    imperfect_mid_3: "ἠ~ῐ~εῑτο", _, _;
    imperfect_pass_2: _, "ἠ~ῐ~εῑσθον", "ἠ~ῐ~εῑσθε";
    imperfect_pass_3: _, "ἠ~ῐ~εισθην", "ἠ~ῐ~οῡντο";
    future_ind_act_1: "ἀ~ῐ~ησω", _, "ἀ~ῐ~ησομεν";
    future_ind_act_2: "ἀ~ῐ~ησεις", "ἀ~ῐ~ησετον", "ἀ~ῐ~ησετε";
    future_ind_act_3: _, _, "ἀ~ῐ~ησουσῐ|ἀ~ῐ~ησουσῐν";
    future_ind_mid_1: "ἀ~ῐ~ησομαι", _, "ἀ~ῐ~ησομεθᾰ";
    future_ind_mid_2: "ἀ~ῐ~ησῃ|ἀ~ῐ~ησει", "ἀ~ῐ~ησεσθον", _;
    future_ind_mid_3: "ἀ~ῐ~ησεται", _, _;
    future_opt_act_1: "ἀ~ῐ~ησοιμῐ", _, "ἀ~ῐ~ησοιμεν";
    future_opt_act_2: _, "ἀ~ῐ~ησοιτον", "ἀ~ῐ~ησοιτε";
    future_opt_act_3: "ἀ~ῐ~ησοι", "ἀ~ῐ~ησοιτην", "ἀ~ῐ~ησοιεν";
    future_opt_mid_1: "ἀ~ῐ~ησοιμην", _, "ἀ~ῐ~ησοιμεθᾰ";
    future_opt_mid_2: "ἀ~ῐ~ησοιο", _, _;
    future_opt_mid_3: _, "ἀ~ῐ~ησοισθην", _;
    aorist_subj_act_1: "ἀ~ῐ~ησω", _, _;
    aorist_subj_act_2: "ἀ~ῐ~ησῃς", "ἀ~ῐ~ησητον", _;
    aorist_subj_act_3: _, _, "ἀ~ῐ~ησωσῐ|ἀ~ῐ~ησωσῐν";
    aorist_subj_mid_1: "ἀ~ῐ~ησωμαι", _, _;
    aorist_subj_mid_2: "ἀ~ῐ~ησῃ", "ἀ~ῐ~ησησθον", _;
    aorist_opt_act_1: "ἀ~ῐ~ησαιμῐ", _, "ἀ~ῐ~ησαιμεν";
    aorist_opt_act_2: "ἀ~ῐ~ησειᾰς|ἀ~ῐ~ησαις", _, _;
    aorist_opt_act_3: "ἀ~ῐ~ησειε|ἀ~ῐ~ησειεν|ἀ~ῐ~ησαι", "ἀ~ῐ~ησαιτην", "ἀ~ῐ~ησειᾰν|ἀ~ῐ~ησαιεν";
    aorist_opt_mid_1: "ἀ~ῐ~ησαιμην", _, _;
    aorist_opt_mid_2: "ἀ~ῐ~ησαιο", _, _;
    aorist_imp_act_2: "ἀ~ῐ~ησον", _, "ἀ~ῐ~ησᾰτε";
    aorist_imp_act_3: _, _, "ἀ~ῐ~ησᾰντων";
    aorist_imp_mid_2: "ἀ~ῐ~ησαι", "ἀ~ῐ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἀ~ῐ~ησᾰσθων";
    infinitive_pres: "ἀ~ῐ~εῑν", _, "ἀ~ῐ~εῑσθαι";
    infinitive_fut: "ἀ~ῐ~ησειν", _, _;
    infinitive_aor: "ἀ~ῐ~ησαι", _, _;
    participle_pres: "ἀ~ῐ~ων", _, "ἀ~ῐ~ουμενος";
    participle_aor: "ἀ~ῐ~ησᾱς", "ἀ~ῐ~ησᾰμενος", _;
    ),
    // Χᾰριζω: 1 words (χᾰριζω). Like Ἐπορον, except in 31 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("χᾰριζω", 1, "~ιζω", like Ἐπορον;
    stem aorist_ind: "~ῐσ- like χᾰριζω";
    ind_act_1: "~ιζω", _, _;
    future_ind_act_1: "~ῐω", _, "~ῐοῡμεν";
    future_ind_act_2: "~ῐεῑς", "~ῐεῑτον", "~ῐεῑτε";
    future_ind_act_3: _, _, "~ῐοῡσῐ|~ῐοῡσῐν";
    future_opt_act_1: "~ῐοιην|~ῐοῑμῐ", _, "~ῐοῑμεν|~ῐοιημεν";
    future_opt_act_2: "~ῐοιης|~ῐοῑς", "~ῐοῑτον|~ῐοιητον", "~ῐοῑτε|~ῐοιητε";
    future_opt_act_3: "~ῐοιη|~ῐοῑ", "~ῐοιτην|~ῐοιητην", "~ῐοῑεν|~ῐοιησᾰν";
    aorist_subj_act_1: "~ῐσω", _, _;
    aorist_subj_act_2: "~ῐσῃς", "~ῐσητον", _;
    aorist_subj_act_3: _, _, "~ῐσωσῐ|~ῐσωσῐν";
    aorist_opt_act_1: "~ῐσαιμῐ", _, "~ῐσαιμεν";
    aorist_opt_act_2: "~ῐσειᾰς|~ῐσαις", _, _;
    aorist_opt_act_3: "~ῐσειε|~ῐσειεν|~ῐσαι", "~ῐσαιτην", "~ῐσειᾰν|~ῐσαιεν";
    aorist_imp_act_2: "~ῐσον", _, "~ῐσᾰτε";
    aorist_imp_act_3: _, _, "~ῐσᾰντων";
    infinitive_fut: "~ῐεῑν", _, _;
    infinitive_aor: "~ῐσαι", _, _;
    participle_aor: "~ῐσᾱς", _, _;
    ),
    // Διαφορεω: 1 words (διαφορεω). Like Πεμπω, except in 162 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("διαφορεω", 1, "~ια~εω", like Πεμπω;
    stem future_pass: "~ῐᾰ~ηθησ- like ποιεω";
    stem aorist_ind: "~ῐε~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ῐε~ηθην- like ἐλπω";
    stem aorist_pass: "~ῐᾰ~ηθ- like ποιεω";
    stem pluperfect_act: "~ῐε%ε~ηκ- like εὐλογεω";
    ind_act_1: "~ῐᾰ~ω", _, "~ῐᾰ~οῡμεν";
    ind_act_2: "~ῐᾰ~εῑς", _, _;
    ind_act_3: "~ῐᾰ~εῑ", "~ῐᾰ~εῑτον", "~ῐᾰ~οῡσῐ|~ῐᾰ~οῡσῐν";
    ind_mid_1: "~ῐᾰ~οῡμαι", _, "~ῐᾰ~ουμεθᾰ";
    ind_mid_2: "~ῐᾰ~εῑ|~ῐᾰ~ῃ", "~ῐᾰ~εῑσθον", _;
    ind_pass_2: _, _, "~ῐᾰ~εῑσθε";
    ind_pass_3: "~ῐᾰ~εῑται", _, "~ῐᾰ~οῡνται";
    subj_act_1: "~ῐᾰ~ω", _, "~ῐᾰ~ωμεν";
    subj_act_2: "~ῐᾰ~ῃς", _, _;
    subj_act_3: "~ῐᾰ~ῃ", "~ῐᾰ~ητον", "~ῐᾰ~ωσῐ|~ῐᾰ~ωσῐν";
    subj_mid_1: _, _, "~ῐᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ῐᾰ~ησθον", _;
    subj_mid_3: "~ῐᾰ~ηται", _, "~ῐᾰ~ωνται";
    subj_pass_1: "~ῐᾰ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῐᾰ~ησθε";
    opt_act_1: "~ῐᾰ~οιην|~ῐᾰ~οῑμῐ", _, "~ῐᾰ~οῑμεν|~ῐᾰ~οιημεν";
    opt_act_2: "~ῐᾰ~οιης|~ῐᾰ~οῑς", "~ῐᾰ~οῑτον|~ῐᾰ~οιητον", "~ῐᾰ~οῑτε|~ῐᾰ~οιητε";
    opt_act_3: "~ῐᾰ~οιη|~ῐᾰ~οῑ", "~ῐᾰ~οιτην|~ῐᾰ~οιητην", "~ῐᾰ~οῑεν|~ῐᾰ~οιησᾰν";
    opt_mid_1: "~ῐᾰ~οιμην", _, "~ῐᾰ~οιμεθᾰ";
    opt_mid_2: "~ῐᾰ~οῑο", "~ῐᾰ~οῑσθον", _;
    opt_mid_3: "~ῐᾰ~οῑτο", "~ῐᾰ~οισθην", "~ῐᾰ~οῑντο";
    opt_pass_2: _, _, "~ῐᾰ~οῑσθε";
    imp_act_2: "~ῐᾰ~ει", "~ῐᾰ~εῑτον", "~ῐᾰ~εῑτε";
    imp_mid_2: "~ῐᾰ~οῡ", _, "~ῐᾰ~εῑσθε";
    imp_mid_3: "~ῐᾰ~εισθω", _, "~ῐᾰ~εισθων";
    imp_pass_2: _, _, "~ῐᾰ~εῑσθε";
    imp_pass_3: _, "~ῐᾰ~εισθων", "~ῐᾰ~εισθων";
    imperfect_act_1: "~ῐε~ουν", _, "~ῐε~οῡμεν";
    imperfect_act_2: "~ῐε~εις", "~ῐε~εῑτον", "~ῐε~εῑτε";
    imperfect_act_3: "~ῐε~ει", "~ῐε~ειτην", "~ῐε~ουν";
    imperfect_mid_1: "~ῐε~ουμην", _, "~ῐε~ουμεθᾰ";
    imperfect_mid_2: "~ῐε~οῡ", _, _;
    imperfect_mid_3: "~ῐε~εῑτο", _, _;
    imperfect_pass_2: _, "~ῐε~εῑσθον", "~ῐε~εῑσθε";
    imperfect_pass_3: _, "~ῐε~εισθην", "~ῐε~οῡντο";
    future_ind_act_1: "~ῐᾰ~ησω", _, "~ῐᾰ~ησομεν";
    future_ind_act_2: "~ῐᾰ~ησεις", "~ῐᾰ~ησετον", "~ῐᾰ~ησετε";
    future_ind_act_3: _, _, "~ῐᾰ~ησουσῐ|~ῐᾰ~ησουσῐν";
    future_ind_mid_1: "~ῐᾰ~ησομαι", _, "~ῐᾰ~ησομεθᾰ";
    future_ind_mid_2: "~ῐᾰ~ησῃ|~ῐᾰ~ησει", "~ῐᾰ~ησεσθον", _;
    future_ind_mid_3: "~ῐᾰ~ησεται", _, _;
    future_opt_act_1: "~ῐᾰ~ησοιμῐ", _, "~ῐᾰ~ησοιμεν";
    future_opt_act_2: _, "~ῐᾰ~ησοιτον", "~ῐᾰ~ησοιτε";
    future_opt_act_3: "~ῐᾰ~ησοι", "~ῐᾰ~ησοιτην", "~ῐᾰ~ησοιεν";
    future_opt_mid_1: "~ῐᾰ~ησοιμην", _, "~ῐᾰ~ησοιμεθᾰ";
    future_opt_mid_2: "~ῐᾰ~ησοιο", _, _;
    future_opt_mid_3: _, "~ῐᾰ~ησοισθην", _;
    aorist_subj_act_1: "~ῐᾰ~ησω", _, _;
    aorist_subj_act_2: "~ῐᾰ~ησῃς", "~ῐᾰ~ησητον", _;
    aorist_subj_act_3: _, _, "~ῐᾰ~ησωσῐ|~ῐᾰ~ησωσῐν";
    aorist_subj_mid_1: "~ῐᾰ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ῐᾰ~ησῃ", "~ῐᾰ~ησησθον", _;
    aorist_opt_act_1: "~ῐᾰ~ησαιμῐ", _, "~ῐᾰ~ησαιμεν";
    aorist_opt_act_2: "~ῐᾰ~ησειᾰς|~ῐᾰ~ησαις", _, _;
    aorist_opt_act_3: "~ῐᾰ~ησειε|~ῐᾰ~ησειεν|~ῐᾰ~ησαι", "~ῐᾰ~ησαιτην", "~ῐᾰ~ησειᾰν|~ῐᾰ~ησαιεν";
    aorist_opt_mid_1: "~ῐᾰ~ησαιμην", _, _;
    aorist_opt_mid_2: "~ῐᾰ~ησαιο", _, _;
    aorist_imp_act_2: "~ῐᾰ~ησον", _, "~ῐᾰ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῐᾰ~ησᾰντων";
    aorist_imp_mid_2: "~ῐᾰ~ησαι", "~ῐᾰ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῐᾰ~ησᾰσθων";
    perfect_ind_act_1: "~ῐᾰ%ε~ηκᾰ", _, "~ῐᾰ%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "~ῐᾰ%ε~ηκᾰτον", _;
    perfect_ind_act_3: "~ῐᾰ%ε~ηκε|~ῐᾰ%ε~ηκεν", _, "~ῐᾰ%ε~ηκᾱσῐ|~ῐᾰ%ε~ηκᾱσῐν";
    perfect_ind_mid_1: "~ῐᾰ%ε~ημαι", _, "~ῐᾰ%ε~ημεθᾰ";
    perfect_ind_mid_2: "~ῐᾰ%ε~ησαι", "~ῐᾰ%ε~ησθον", _;
    perfect_ind_mid_3: "~ῐᾰ%ε~ηται", _, "~ῐᾰ%ε~ηνται";
    perfect_ind_pass_2: _, _, "~ῐᾰ%ε~ησθε";
    perfect_subj_act_1: "~ῐᾰ%ε~ηκω", _, _;
    perfect_subj_act_2: "~ῐᾰ%ε~ηκῃς", "~ῐᾰ%ε~ηκητον", "~ῐᾰ%ε~ηκητε";
    perfect_subj_act_3: "~ῐᾰ%ε~ηκῃ", _, "~ῐᾰ%ε~ηκωσῐ|~ῐᾰ%ε~ηκωσῐν";
    perfect_opt_act_1: "~ῐᾰ%ε~ηκοιμῐ|~ῐᾰ%ε~ηκοιην", _, "~ῐᾰ%ε~ηκοιμεν";
    perfect_opt_act_2: "~ῐᾰ%ε~ηκοις|~ῐᾰ%ε~ηκοιης", _, _;
    perfect_opt_act_3: "~ῐᾰ%ε~ηκοι|~ῐᾰ%ε~ηκοιη", "~ῐᾰ%ε~ηκοιτην", _;
    perfect_imp_act_2: "~ῐᾰ%ε~ηκε", _, "~ῐᾰ%ε~ηκετε";
    perfect_imp_act_3: _, _, "~ῐᾰ%ε~ηκοντων";
    perfect_imp_mid_2: _, _, "~ῐᾰ%ε~ησθε";
    perfect_imp_mid_3: "~ῐᾰ%ε~ησθω", _, "~ῐᾰ%ε~ησθων";
    perfect_imp_pass_2: "~ῐᾰ%ε~ησο", _, "~ῐᾰ%ε~ησθε";
    perfect_imp_pass_3: _, "~ῐᾰ%ε~ησθων", "~ῐᾰ%ε~ησθων";
    pluperfect_mid_1: "~ῐε%ε~ημην", _, _;
    pluperfect_mid_2: "~ῐε%ε~ησο", "~ῐε%ε~ησθον", _;
    pluperfect_mid_3: "~ῐε%ε~ητο", _, "~ῐε%ε~ηντο";
    pluperfect_pass_1: _, _, "~ῐε%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ῐε%ε~ησθε";
    pluperfect_pass_3: _, "~ῐε%ε~ησθην", _;
    infinitive_pres: "~ῐᾰ~εῑν", _, "~ῐᾰ~εῑσθαι";
    infinitive_fut: "~ῐᾰ~ησειν", _, _;
    infinitive_aor: "~ῐᾰ~ησαι", _, _;
    infinitive_perf: _, "~ῐᾰ%ε~ησθαι", _;
    participle_pres: "~ῐᾰ~ων", _, "~ῐᾰ~ουμενος";
    participle_aor: "~ῐᾰ~ησᾱς", "~ῐᾰ~ησᾰμενος", _;
    participle_perf: "~ῐᾰ%ε~ηκως", _, "~ῐᾰ%ε~ημενος";
    ),
    // Τελεθω: 1 words (τελεθω). Like Μῑγνῡμῐ, except in 46 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("τελεθω", 1, "~ω", like Μῑγνῡμῐ;
    ind_act_1: "~εσκω", _, "~εσκομεν";
    ind_act_2: "~εσκεις", _, _;
    ind_act_3: "~εσκει", "~εσκετον", "~εσκουσῐ|~εσκουσῐν";
    ind_mid_1: "~εσκομαι", _, "~εσκομεθᾰ";
    ind_mid_2: "~εσκεαι|~εσκῃ", "~εσκεσθον", _;
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
    opt_mid_2: "~εσκοιο", "~εσκοισθον", _;
    opt_mid_3: "~εσκοιτο", "~εσκοισθην", "~εσκοιᾰτο";
    imp_act_2: "~εσκε", "~εσκετον", "~εσκετε";
    imp_mid_2: "~εσκεο|~εσκευ", _, "~εσκεσθε";
    imp_mid_3: "~εσκεσθω", _, "~εσκεσθων";
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    infinitive_pres: "~εσκειν", _, _;
    participle_pres: "~εσκων", _, _;
    ),
    // Δῐᾰσευομαι: 1 words (δῐᾰσευομαι). Like Νισσομαι, except in 50 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("δῐᾰσευομαι", 1, "~ᾰσ~υομαι", like Νισσομαι;
    ind_act_1: "~ᾰσ~υομαι", _, _;
    ind_mid_1: "~ασ~υομαι", _, "~ασ~υομεθᾰ";
    ind_mid_2: "~ασ~υῃ|~ασ~υει", "~ασ~υεσθον", _;
    ind_pass_2: _, _, "~ασ~υεσθε";
    ind_pass_3: "~ασ~υεται", _, "~ασ~υονται";
    subj_mid_1: "~ασ~υωμαι", _, "~ασ~υωμεθᾰ";
    subj_mid_2: "~ασ~υῃ", "~ασ~υησθον", _;
    subj_mid_3: "~ασ~υηται", _, "~ασ~υωνται";
    subj_pass_2: "~ασ~υῃ", _, "~ασ~υησθε";
    opt_mid_1: "~ασ~υοιμην", _, "~ασ~υοιμεθᾰ";
    opt_mid_2: "~ασ~υοιο", "~ασ~υοισθον", _;
    opt_mid_3: "~ασ~υοιτο", "~ασ~υοισθην", "~ασ~υοιντο";
    opt_pass_2: _, _, "~ασ~υοισθε";
    imp_mid_2: "~ασ~υου", _, "~ασ~υεσθε";
    imp_mid_3: "~ασ~υεσθω", _, "~ασ~υεσθων";
    imp_pass_2: _, _, "~ασ~υεσθε";
    imp_pass_3: _, "~ασ~υεσθων", "~ασ~υεσθων";
    imperfect_mid_1: "~~σευομην", _, "~~σευομεθᾰ";
    imperfect_mid_2: "~~σευου", "~~σευεσθον", _;
    imperfect_mid_3: "~~σευετο", "~~σευεσθην", "~~σευοντο";
    imperfect_pass_2: _, _, "~~σευεσθε";
    perfect_ind_mid_1: "~~σσῠμαι", _, "~~σσῠμεθᾰ";
    perfect_ind_mid_2: "~~σσῠσαι", "~~σσῠσθον", _;
    perfect_ind_mid_3: "~~σσῠται", _, "~~σσῠνται";
    perfect_imp_mid_2: _, _, "~~σσῠσθε";
    perfect_imp_mid_3: "~~σσῠσθω", _, "~~σσῠσθων";
    infinitive_pres: _, _, "~ασ~υεσθαι";
    participle_pres: _, _, "~ασ~υομενος";
    ),
    // Φθιω: 1 words (φθιω). Like Μῑγνῡμῐ, except in 168 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("φθιω", 1, "~ιω", like Μῑγνῡμῐ;
    stem aorist_ind: "~- like φθιω";
    stem aorist_ind_pass: "~ῐθην- like ἐχω";
    stem aorist_pass: "~ῐθ- like ποιεω";
    ind_act_1: "~ῐνω", _, "~ῐνομεν";
    ind_act_2: "~ῐνεις", _, _;
    ind_act_3: "~ῐνει", "~ῐνετον", "~ῐνουσῐ|~ῐνουσῐν";
    ind_mid_1: "~ῐνομαι", _, "~ῐνομεθᾰ";
    ind_mid_2: "~ῐνῃ|~ῐνει", "~ῐνεσθον", _;
    ind_pass_2: _, _, "~ῐνεσθε";
    ind_pass_3: "~ῐνεται", _, "~ῐνονται";
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
    opt_mid_2: "~ῐνοιο", "~ῐνοισθον", _;
    opt_mid_3: "~ῐνοιτο", "~ῐνοισθην", "~ῐνοιντο";
    opt_pass_2: _, _, "~ῐνοισθε";
    imp_act_2: "~ῐνε", "~ῐνετον", "~ῐνετε";
    imp_mid_2: "~ῐνου", _, "~ῐνεσθε";
    imp_mid_3: "~ῐνεσθω", _, "~ῐνεσθων";
    imp_pass_2: _, _, "~ῐνεσθε";
    imp_pass_3: _, "~ῐνεσθων", "~ῐνεσθων";
    imperfect_act_1: "ἐ~ῐνον", _, "ἐ~ῐνομεν";
    imperfect_act_2: "ἐ~ῐνες", "ἐ~ῐνετον", "ἐ~ῐνετε";
    imperfect_act_3: "ἐ~ῐνε|ἐ~ῐνεν", "ἐ~ῐνετην", "ἐ~ῐνον";
    imperfect_mid_1: "ἐ~ῐνομην", _, "ἐ~ῐνομεθᾰ";
    imperfect_mid_2: "ἐ~ῐνου", _, _;
    imperfect_mid_3: "ἐ~ῐνετο", _, _;
    imperfect_pass_2: _, "ἐ~ῐνεσθον", "ἐ~ῐνεσθε";
    imperfect_pass_3: _, "ἐ~ῐνεσθην", "ἐ~ῐνοντο";
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
    aorist_imp_act_2: "~ῑσον|~εῑσον|~ῐνησον", "~ῑσᾰτον|~εισᾰτον|~ῐνησᾰτον", "~ῑσᾰτε|~εισᾰτε|~ῐνησᾰτε";
    aorist_imp_act_3: "~ῑσᾰτω|~εισᾰτω|~ῐνησᾰτω", "~ῑσᾰτων|~εισᾰτων|~ῐνησᾰτων", "~ῑσᾰντων|~εισᾰντων|~ῐνησᾰντων";
    aorist_imp_mid_2: "~ῑσαι|~εῑσαι|~ῐνησαι", "~ῑσᾰσθον|~εισᾰσθον|~ῐνησᾰσθον", "~ῑσᾰσθε|~εισᾰσθε|~ῐνησᾰσθε";
    aorist_imp_mid_3: "~ῑσᾰσθω|~εισᾰσθω|~ῐνησᾰσθω", "~ῑσᾰσθων|~εισᾰσθων|~ῐνησᾰσθων", _;
    perfect_ind_act_1: "ἐ~ῐνηκᾰ", _, "ἐ~ῐνηκᾰμεν";
    perfect_ind_act_2: _, "ἐ~ῐνηκᾰτον", _;
    perfect_ind_act_3: "ἐ~ῐνηκε|ἐ~ῐνηκεν", _, "ἐ~ῐνηκᾱσῐ|ἐ~ῐνηκᾱσῐν";
    perfect_subj_act_1: "ἐ~ῐνηκω", _, _;
    perfect_subj_act_2: "ἐ~ῐνηκῃς", "ἐ~ῐνηκητον", "ἐ~ῐνηκητε";
    perfect_subj_act_3: "ἐ~ῐνηκῃ", _, "ἐ~ῐνηκωσῐ|ἐ~ῐνηκωσῐν";
    perfect_opt_act_1: "ἐ~ῐνηκοιμῐ|ἐ~ῐνηκοιην", _, "ἐ~ῐνηκοιμεν";
    perfect_opt_act_2: "ἐ~ῐνηκοις|ἐ~ῐνηκοιης", _, _;
    perfect_opt_act_3: "ἐ~ῐνηκοι|ἐ~ῐνηκοιη", "ἐ~ῐνηκοιτην", _;
    perfect_imp_act_2: "ἐ~ῐνηκε", _, "ἐ~ῐνηκετε";
    perfect_imp_act_3: _, _, "ἐ~ῐνηκοντων";
    infinitive_pres: "~ῐνειν", _, "~ῐνεσθαι";
    infinitive_fut: "~ῑσειν|~εισειν|~ῐνησειν", "~ῑσεσθαι|~εισεσθαι|~ῐνησεσθαι", _;
    infinitive_aor: "~ῑσαι|~εῑσαι|~ῐνησαι", "~ῑσᾰσθαι|~εισᾰσθαι|~ῐνησᾰσθαι", _;
    participle_pres: "~ῐνων", _, "~ῐνομενος";
    participle_fut: "~ῑσων|~εισων|~ῐνησων", "~ῑσομενος|~εισομενος|~ῐνησομενος", _;
    participle_aor: "~ῑσᾱς|~εισᾱς|~ῐνησᾱς", "~ῑσᾰμενος|~εισᾰμενος|~ῐνησᾰμενος", _;
    participle_perf: "ἐ~ῐνηκως", _, _;
    ),
    // Οικειοομαι: 1 words (οικειοομαι). Like Ἐρᾰμαι, except in 41 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("οικειοομαι", 1, "οι~οομαι", like Ἐρᾰμαι;
    ind_act_1: "οι~οομαι", _, _;
    ind_mid_1: "οἰ~οῡμαι", _, "οἰ~ουμεθᾰ";
    ind_mid_2: "οἰ~οῑ", "οἰ~οῡσθον", _;
    ind_pass_2: _, _, "οἰ~οῡσθε";
    ind_pass_3: "οἰ~οῡται", _, "οἰ~οῡνται";
    subj_mid_1: "οἰ~ωμαι", _, "οἰ~ωμεθᾰ";
    subj_mid_2: "οἰ~οῑ", "οἰ~ωσθον", _;
    subj_mid_3: "οἰ~ωται", _, "οἰ~ωνται";
    subj_pass_2: "οἰ~οῑ", _, "οἰ~ωσθε";
    opt_mid_1: "οἰ~οιμην", _, "οἰ~οιμεθᾰ";
    opt_mid_2: "οἰ~οῑο", "οἰ~οῑσθον", _;
    opt_mid_3: "οἰ~οῑτο", "οἰ~οισθην", "οἰ~οῑντο";
    opt_pass_2: _, _, "οἰ~οῑσθε";
    imp_mid_2: "οἰ~οῡ", _, "οἰ~οῡσθε";
    imp_mid_3: "οἰ~ουσθω", _, "οἰ~ουσθων";
    imp_pass_2: _, _, "οἰ~οῡσθε";
    imp_pass_3: _, "οἰ~ουσθων", "οἰ~ουσθων";
    imperfect_mid_1: "ᾠ~ουμην", _, "ᾠ~ουμεθᾰ";
    imperfect_mid_2: "ᾠ~οῡ", "ᾠ~οῡσθον", _;
    imperfect_mid_3: "ᾠ~οῡτο", "ᾠ~ουσθην", "ᾠ~οῡντο";
    imperfect_pass_2: _, _, "ᾠ~οῡσθε";
    infinitive_pres: _, _, "οἰ~οῡσθαι";
    participle_pres: _, _, "οἰ~ουμενος";
    ),
    // Συγκᾰκουργεω: 1 words (συγκᾰκουργεω). Like Ξυμπολεμεω, except in 158 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 du, ….
    conjugation!("συγκᾰκουργεω", 1, "~υγ~εω", like Ξυμπολεμεω;
    stem future_pass: "~ῠγ~ηθησ- like ποιεω";
    stem aorist_pass: "~ῠγ~ηθ- like ποιεω";
    stem pluperfect_act: "~ῠγ%ε~ηκ- like δῐκεω";
    ind_act_1: "~ῠγ~ω", _, "~ῠγ~οῡμεν";
    ind_act_2: "~ῠγ~εῑς", derived, _;
    ind_act_3: "~ῠγ~εῑ", "~ῠγ~εῑτον", "~ῠγ~οῡσῐ|~ῠγ~οῡσῐν";
    ind_mid_1: "~ῠγ~οῡμαι", _, "~ῠγ~ουμεθᾰ";
    ind_mid_2: "~ῠγ~εῑ|~ῠγ~ῃ", "~ῠγ~εῑσθον", _;
    ind_pass_2: _, _, "~ῠγ~εῑσθε";
    ind_pass_3: "~ῠγ~εῑται", _, "~ῠγ~οῡνται";
    subj_act_1: "~ῠγ~ω", _, "~ῠγ~ωμεν";
    subj_act_2: "~ῠγ~ῃς", derived, _;
    subj_act_3: "~ῠγ~ῃ", "~ῠγ~ητον", "~ῠγ~ωσῐ|~ῠγ~ωσῐν";
    subj_mid_1: _, _, "~ῠγ~ωμεθᾰ";
    subj_mid_2: _, "~ῠγ~ησθον", _;
    subj_mid_3: "~ῠγ~ηται", _, "~ῠγ~ωνται";
    subj_pass_1: "~ῠγ~ωμαι", _, _;
    subj_pass_2: _, _, "~ῠγ~ησθε";
    opt_act_1: "~ῠγ~οιην|~ῠγ~οῑμῐ", _, "~ῠγ~οῑμεν|~ῠγ~οιημεν";
    opt_act_2: "~ῠγ~οιης|~ῠγ~οῑς", "~ῠγ~οῑτον|~ῠγ~οιητον", "~ῠγ~οῑτε|~ῠγ~οιητε";
    opt_act_3: "~ῠγ~οιη|~ῠγ~οῑ", "~ῠγ~οιτην|~ῠγ~οιητην", "~ῠγ~οῑεν|~ῠγ~οιησᾰν";
    opt_mid_1: "~ῠγ~οιμην", _, "~ῠγ~οιμεθᾰ";
    opt_mid_2: "~ῠγ~οῑο", "~ῠγ~οῑσθον", _;
    opt_mid_3: "~ῠγ~οῑτο", "~ῠγ~οισθην", "~ῠγ~οῑντο";
    opt_pass_2: _, _, "~ῠγ~οῑσθε";
    imp_act_2: "~ῠγ~ει", "~ῠγ~εῑτον", "~ῠγ~εῑτε";
    imp_act_3: derived, _, derived;
    imp_mid_2: "~ῠγ~οῡ", _, "~ῠγ~εῑσθε";
    imp_mid_3: "~ῠγ~εισθω", _, "~ῠγ~εισθων";
    imp_pass_2: _, _, "~ῠγ~εῑσθε";
    imp_pass_3: _, "~ῠγ~εισθων", "~ῠγ~εισθων";
    imperfect_mid_1: "~ῠνε~ουμην", _, "~ῠνε~ουμεθᾰ";
    imperfect_mid_2: "~ῠνε~οῡ", _, _;
    imperfect_mid_3: "~ῠνε~εῑτο", _, _;
    imperfect_pass_2: _, "~ῠνε~εῑσθον", "~ῠνε~εῑσθε";
    imperfect_pass_3: _, "~ῠνε~εισθην", "~ῠνε~οῡντο";
    future_ind_act_1: "~ῠγ~ησω", _, "~ῠγ~ησομεν";
    future_ind_act_2: "~ῠγ~ησεις", "~ῠγ~ησετον", "~ῠγ~ησετε";
    future_ind_act_3: _, _, "~ῠγ~ησουσῐ|~ῠγ~ησουσῐν";
    future_ind_mid_1: "~ῠγ~ησομαι", _, "~ῠγ~ησομεθᾰ";
    future_ind_mid_2: "~ῠγ~ησῃ|~ῠγ~ησει", "~ῠγ~ησεσθον", _;
    future_ind_mid_3: "~ῠγ~ησεται", _, _;
    future_opt_act_1: "~ῠγ~ησοιμῐ", _, "~ῠγ~ησοιμεν";
    future_opt_act_2: _, "~ῠγ~ησοιτον", "~ῠγ~ησοιτε";
    future_opt_act_3: "~ῠγ~ησοι", "~ῠγ~ησοιτην", "~ῠγ~ησοιεν";
    future_opt_mid_1: "~ῠγ~ησοιμην", _, "~ῠγ~ησοιμεθᾰ";
    future_opt_mid_2: "~ῠγ~ησοιο", _, _;
    future_opt_mid_3: _, "~ῠγ~ησοισθην", _;
    aorist_subj_act_1: "~ῠγ~ησω", _, _;
    aorist_subj_act_2: "~ῠγ~ησῃς", "~ῠγ~ησητον", _;
    aorist_subj_act_3: _, _, "~ῠγ~ησωσῐ|~ῠγ~ησωσῐν";
    aorist_subj_mid_1: "~ῠγ~ησωμαι", _, _;
    aorist_subj_mid_2: "~ῠγ~ησῃ", "~ῠγ~ησησθον", _;
    aorist_opt_act_1: "~ῠγ~ησαιμῐ", _, "~ῠγ~ησαιμεν";
    aorist_opt_act_2: "~ῠγ~ησειᾰς|~ῠγ~ησαις", _, _;
    aorist_opt_act_3: "~ῠγ~ησειε|~ῠγ~ησειεν|~ῠγ~ησαι", "~ῠγ~ησαιτην", "~ῠγ~ησειᾰν|~ῠγ~ησαιεν";
    aorist_opt_mid_1: "~ῠγ~ησαιμην", _, _;
    aorist_opt_mid_2: "~ῠγ~ησαιο", _, _;
    aorist_imp_act_2: "~ῠγ~ησον", _, "~ῠγ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῠγ~ησᾰντων";
    aorist_imp_mid_2: "~ῠγ~ησαι", "~ῠγ~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῠγ~ησᾰσθων";
    perfect_ind_act_1: "~ῠγ%ε~ηκᾰ", _, "~ῠγ%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "~ῠγ%ε~ηκᾰτον", _;
    perfect_ind_act_3: "~ῠγ%ε~ηκε|~ῠγ%ε~ηκεν", _, "~ῠγ%ε~ηκᾱσῐ|~ῠγ%ε~ηκᾱσῐν";
    perfect_ind_mid_1: "~ῠγ%ε~ημαι", _, "~ῠγ%ε~ημεθᾰ";
    perfect_ind_mid_2: "~ῠγ%ε~ησαι", "~ῠγ%ε~ησθον", _;
    perfect_ind_mid_3: "~ῠγ%ε~ηται", _, "~ῠγ%ε~ηνται";
    perfect_ind_pass_2: _, _, "~ῠγ%ε~ησθε";
    perfect_subj_act_1: "~ῠγ%ε~ηκω", _, _;
    perfect_subj_act_2: "~ῠγ%ε~ηκῃς", "~ῠγ%ε~ηκητον", "~ῠγ%ε~ηκητε";
    perfect_subj_act_3: "~ῠγ%ε~ηκῃ", _, "~ῠγ%ε~ηκωσῐ|~ῠγ%ε~ηκωσῐν";
    perfect_opt_act_1: "~ῠγ%ε~ηκοιμῐ|~ῠγ%ε~ηκοιην", _, "~ῠγ%ε~ηκοιμεν";
    perfect_opt_act_2: "~ῠγ%ε~ηκοις|~ῠγ%ε~ηκοιης", _, _;
    perfect_opt_act_3: "~ῠγ%ε~ηκοι|~ῠγ%ε~ηκοιη", "~ῠγ%ε~ηκοιτην", _;
    perfect_imp_act_2: "~ῠγ%ε~ηκε", _, "~ῠγ%ε~ηκετε";
    perfect_imp_act_3: _, _, "~ῠγ%ε~ηκοντων";
    perfect_imp_mid_2: _, _, "~ῠγ%ε~ησθε";
    perfect_imp_mid_3: "~ῠγ%ε~ησθω", _, "~ῠγ%ε~ησθων";
    perfect_imp_pass_2: "~ῠγ%ε~ησο", _, "~ῠγ%ε~ησθε";
    perfect_imp_pass_3: _, "~ῠγ%ε~ησθων", "~ῠγ%ε~ησθων";
    pluperfect_mid_1: "ἐ~ῠγ%ε~ημην", _, _;
    pluperfect_mid_2: "ἐ~ῠγ%ε~ησο", "ἐ~ῠγ%ε~ησθον", _;
    pluperfect_mid_3: "ἐ~ῠγ%ε~ητο", _, "ἐ~ῠγ%ε~ηντο";
    pluperfect_pass_1: _, _, "ἐ~ῠγ%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ῠγ%ε~ησθε";
    pluperfect_pass_3: _, "ἐ~ῠγ%ε~ησθην", _;
    infinitive_pres: "~ῠγ~εῑν", _, "~ῠγ~εῑσθαι";
    infinitive_fut: "~ῠγ~ησειν", _, _;
    infinitive_aor: "~ῠγ~ησαι", _, _;
    infinitive_perf: _, "~ῠγ%ε~ησθαι", _;
    participle_pres: "~ῠγ~ων", _, "~ῠγ~ουμενος";
    participle_aor: "~ῠγ~ησᾱς", "~ῠγ~ησᾰμενος", _;
    participle_perf: "~ῠγ%ε~ηκως", _, "~ῠγ%ε~ημενος";
    ),
    // Ἀποστατεω: 1 words (ἀποστατεω). Like Ξυνναυμᾰχεω, except in 61 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀποστατεω", 1, "ἀ~ο~α~εω", like Ξυνναυμᾰχεω;
    stem imperfect: "~ε~ᾰ~- like ἀποστατεω";
    ind_act_1: "ᾰ~ο~ᾰ~ω", _, "ᾰ~ο~ᾰ~οῡμεν";
    ind_act_2: "ᾰ~ο~ᾰ~εῑς", "ᾰ~ο~ᾰ~εῑτον", _;
    ind_act_3: "ᾰ~ο~ᾰ~εῑ", _, "ᾰ~ο~ᾰ~οῡσῐ|ᾰ~ο~ᾰ~οῡσῐν";
    subj_act_1: "ᾰ~ο~ᾰ~ω", _, "ᾰ~ο~ᾰ~ωμεν";
    subj_act_2: "ᾰ~ο~ᾰ~ῃς", "ᾰ~ο~ᾰ~ητον", _;
    subj_act_3: "ᾰ~ο~ᾰ~ῃ", _, "ᾰ~ο~ᾰ~ωσῐ|ᾰ~ο~ᾰ~ωσῐν";
    opt_act_1: "ᾰ~ο~ᾰ~οιην|ᾰ~ο~ᾰ~οῑμῐ", _, "ᾰ~ο~ᾰ~οῑμεν|ᾰ~ο~ᾰ~οιημεν";
    opt_act_2: "ᾰ~ο~ᾰ~οιης|ᾰ~ο~ᾰ~οῑς", "ᾰ~ο~ᾰ~οῑτον|ᾰ~ο~ᾰ~οιητον", "ᾰ~ο~ᾰ~οῑτε|ᾰ~ο~ᾰ~οιητε";
    opt_act_3: "ᾰ~ο~ᾰ~οιη|ᾰ~ο~ᾰ~οῑ", "ᾰ~ο~ᾰ~οιτην|ᾰ~ο~ᾰ~οιητην", "ᾰ~ο~ᾰ~οῑεν|ᾰ~ο~ᾰ~οιησᾰν";
    imp_act_2: "ᾰ~ο~ᾰ~ει", _, "ᾰ~ο~ᾰ~εῑτε";
    imp_act_3: "ᾰ~ο~ᾰ~ειτω", _, "ᾰ~ο~ᾰ~ουντων";
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: _, _, derived;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: _, derived, derived;
    future_opt_act_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, _;
    aorist_ind_act_3: derived, _, derived;
    aorist_subj_act_1: derived, _, _;
    aorist_subj_act_2: derived, derived, _;
    aorist_subj_act_3: _, _, derived;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, _, _;
    aorist_opt_act_3: derived, derived, derived;
    aorist_imp_act_2: derived, _, derived;
    aorist_imp_act_3: _, _, derived;
    infinitive_pres: "ᾰ~ο~ᾰ~εῑν", _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    participle_pres: "ᾰ~ο~ᾰ~ων", _, _;
    participle_aor: derived, _, _;
    ),
    // Γεγενησαι: 1 words (γεγενησαι). Like Κᾰθημαι, except in 45 cells: ind_act_1 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, ….
    conjugation!("γεγενησαι", 1, "~ε~ησαι", like Κᾰθημαι;
    ind_act_1: "~ε~ησαι", _, _;
    perfect_ind_act_1: "~ο~ᾰ", _, "~ο~ᾰμεν";
    perfect_ind_act_2: _, "~ο~ᾰτον", _;
    perfect_ind_act_3: "~ο~ε|~ο~εν", _, "~ο~ᾱσῐ|~ο~ᾱσῐν";
    perfect_ind_mid_1: "~ε~ημαι", _, "~ε~ημεθᾰ";
    perfect_ind_mid_2: "~ε~ησαι", "~ε~ησθον", _;
    perfect_ind_mid_3: "~ε~ηται", _, "~ε~ηνται";
    perfect_ind_pass_2: _, _, "~ε~ησθε";
    perfect_subj_act_1: "~ο~ω", _, _;
    perfect_subj_act_2: "~ο~ῃς", "~ο~ητον", "~ο~ητε";
    perfect_subj_act_3: "~ο~ῃ", _, "~ο~ωσῐ|~ο~ωσῐν";
    perfect_opt_act_1: "~ο~οιμῐ|~ο~οιην", _, "~ο~οιμεν";
    perfect_opt_act_2: "~ο~οις|~ο~οιης", _, _;
    perfect_opt_act_3: "~ο~οι|~ο~οιη", "~ο~οιτην", _;
    perfect_imp_act_2: "~ο~ε", _, "~ο~ετε";
    perfect_imp_act_3: _, _, "~ο~οντων";
    perfect_imp_mid_2: _, _, "~ε~ησθε";
    perfect_imp_mid_3: "~ε~ησθω", _, "~ε~ησθων";
    perfect_imp_pass_2: "~ε~ησο", _, "~ε~ησθε";
    perfect_imp_pass_3: _, "~ε~ησθων", "~ε~ησθων";
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
    // Γεγενησο: 1 words (γεγενησο). Like Γεγονοι, except in 37 cells: ind_act_1 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 du, perfect_ind_act_3 sg, perfect_ind_act_3 pl, ….
    conjugation!("γεγενησο", 1, "~ενησο", like Γεγονοι;
    ind_act_1: "~ενησο", _, _;
    perfect_ind_act_1: "~ονᾰ", _, "~ονᾰμεν";
    perfect_ind_act_2: _, "~ονᾰτον", _;
    perfect_ind_act_3: "~ονε|~ονεν", _, "~ονᾱσῐ|~ονᾱσῐν";
    perfect_ind_mid_1: "~ενημαι", _, "~ενημεθᾰ";
    perfect_ind_mid_2: "~ενησαι", "~ενησθον", _;
    perfect_ind_mid_3: "~ενηται", _, "~ενηνται";
    perfect_ind_pass_2: _, _, "~ενησθε";
    perfect_subj_act_1: "~ονω", _, _;
    perfect_subj_act_2: "~ονῃς", "~ονητον", "~ονητε";
    perfect_subj_act_3: "~ονῃ", _, "~ονωσῐ|~ονωσῐν";
    perfect_opt_act_1: "~ονοιμῐ|~ονοιην", _, "~ονοιμεν";
    perfect_opt_act_2: "~ονοις|~ονοιης", _, _;
    perfect_opt_act_3: "~ονοι|~ονοιη", "~ονοιτην", _;
    perfect_imp_act_2: "~ονε", _, "~ονετε";
    perfect_imp_act_3: _, _, "~ονοντων";
    perfect_imp_mid_2: _, _, "~ενησθε";
    perfect_imp_mid_3: "~ενησθω", _, "~ενησθων";
    perfect_imp_pass_2: "~ενησο", _, "~ενησθε";
    perfect_imp_pass_3: _, "~ενησθων", "~ενησθων";
    infinitive_perf: _, "~ενησθαι", _;
    participle_perf: "~ονως", _, "~ενημενος";
    ),
    // Γεγενησθον: 1 words (γεγενησθον). Like Γεγενησαι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησθον", 1, "~ε~ησθον", like Γεγενησαι;
    ind_act_1: "~ε~ησθον", _, _;
    ),
    // Γεγενησθων: 1 words (γεγενησθων). Like Γεγενησαι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησθων", 1, "~ε~ησθων", like Γεγενησαι;
    ind_act_1: "~ε~ησθων", _, _;
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
    // Τεθνηκοιη: 1 words (τεθνηκοιη). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοιη", 1, "~οιη", like Τεθνηκας;
    ind_act_1: "~οιη", _, _;
    ),
    // Τεθνηκοιμι: 1 words (τεθνηκοιμι). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοιμι", 1, "~οιμι", like Τεθνηκας;
    ind_act_1: "~οιμι", _, _;
    ),
    // Τεθνηκοιην: 1 words (τεθνηκοιην). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοιην", 1, "~οιην", like Τεθνηκας;
    ind_act_1: "~οιην", _, _;
    ),
    // Τεθνηκοις: 1 words (τεθνηκοις). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοις", 1, "~οις", like Τεθνηκας;
    ind_act_1: "~οις", _, _;
    ),
    // Τεθνηκοιης: 1 words (τεθνηκοιης). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοιης", 1, "~οιης", like Τεθνηκας;
    ind_act_1: "~οιης", _, _;
    ),
    // Τεθνηκῃς: 1 words (τεθνηκῃς). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκῃς", 1, "~ῃς", like Τεθνηκας;
    ind_act_1: "~ῃς", _, _;
    ),
    // Τεθνηκετω: 1 words (τεθνηκετω). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκετω", 1, "~ετω", like Τεθνηκας;
    ind_act_1: "~ετω", _, _;
    ),
    // Τεθνηκαμεν: 1 words (τεθνηκαμεν). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκαμεν", 1, "~αμεν", like Τεθνηκας;
    ind_act_1: "~αμεν", _, _;
    ),
    // Τεθνηκωμεν: 1 words (τεθνηκωμεν). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκωμεν", 1, "~ωμεν", like Τεθνηκας;
    ind_act_1: "~ωμεν", _, _;
    ),
    // Τεθνηκοιμεν: 1 words (τεθνηκοιμεν). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοιμεν", 1, "~οιμεν", like Τεθνηκας;
    ind_act_1: "~οιμεν", _, _;
    ),
    // Τεθνηκατε: 1 words (τεθνηκατε). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκατε", 1, "~ατε", like Τεθνηκας;
    ind_act_1: "~ατε", _, _;
    ),
    // Τεθνηκητε: 1 words (τεθνηκητε). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκητε", 1, "~ητε", like Τεθνηκας;
    ind_act_1: "~ητε", _, _;
    ),
    // Τεθνηκοιτε: 1 words (τεθνηκοιτε). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοιτε", 1, "~οιτε", like Τεθνηκας;
    ind_act_1: "~οιτε", _, _;
    ),
    // Τεθνηκετε: 1 words (τεθνηκετε). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκετε", 1, "~ετε", like Τεθνηκας;
    ind_act_1: "~ετε", _, _;
    ),
    // Τεθνηκασι: 1 words (τεθνηκασι). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκασι", 1, "~ασι", like Τεθνηκας;
    ind_act_1: "~ασι", _, _;
    ),
    // Τεθνηκασιν: 1 words (τεθνηκασιν). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκασιν", 1, "~ασιν", like Τεθνηκας;
    ind_act_1: "~ασιν", _, _;
    ),
    // Τεθνηκωσι: 1 words (τεθνηκωσι). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκωσι", 1, "~ωσι", like Τεθνηκας;
    ind_act_1: "~ωσι", _, _;
    ),
    // Τεθνηκωσιν: 1 words (τεθνηκωσιν). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκωσιν", 1, "~ωσιν", like Τεθνηκας;
    ind_act_1: "~ωσιν", _, _;
    ),
    // Τεθνηκοιεν: 1 words (τεθνηκοιεν). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοιεν", 1, "~οιεν", like Τεθνηκας;
    ind_act_1: "~οιεν", _, _;
    ),
    // Τεθνηκοντων: 1 words (τεθνηκοντων). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοντων", 1, "~οντων", like Τεθνηκας;
    ind_act_1: "~οντων", _, _;
    ),
    // Τεθνηκατον: 1 words (τεθνηκατον). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκατον", 1, "~ατον", like Τεθνηκας;
    ind_act_1: "~ατον", _, _;
    ),
    // Τεθνηκητον: 1 words (τεθνηκητον). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκητον", 1, "~ητον", like Τεθνηκας;
    ind_act_1: "~ητον", _, _;
    ),
    // Τεθνηκοιτον: 1 words (τεθνηκοιτον). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοιτον", 1, "~οιτον", like Τεθνηκας;
    ind_act_1: "~οιτον", _, _;
    ),
    // Τεθνηκοιτην: 1 words (τεθνηκοιτην). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοιτην", 1, "~οιτην", like Τεθνηκας;
    ind_act_1: "~οιτην", _, _;
    ),
    // Τεθνηκετον: 1 words (τεθνηκετον). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκετον", 1, "~ετον", like Τεθνηκας;
    ind_act_1: "~ετον", _, _;
    ),
    // Τεθνηκετων: 1 words (τεθνηκετων). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκετων", 1, "~ετων", like Τεθνηκας;
    ind_act_1: "~ετων", _, _;
    ),
    // Ἀντιπαρατασσομαι: 1 words (ἀντιπαρατασσομαι).
    conjugation!("ἀντιπαρατασσομαι", 1, "~ι~α~α~ασσομαι";
    stem future_pass: "~ῐ~ᾰ~ᾰ~ᾰχθησ- like ποιεω";
    stem aorist_ind: "~ῐ~ᾰ~ε~ᾰξ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ῐ~ᾰ~ε~ᾰχθην- like ἐλπω";
    stem aorist_pass: "~ῐ~ᾰ~ᾰ~ᾰχθ- like ποιεω";
    ind_act_1: "~ι~α~α~ασσομαι", _, _;
    ind_mid_1: "~ῐ~ᾰ~ᾰ~ᾰσσομαι", _, "~ῐ~ᾰ~ᾰ~ᾰσσομεθᾰ";
    ind_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσῃ|~ῐ~ᾰ~ᾰ~ᾰσσει", "~ῐ~ᾰ~ᾰ~ᾰσσεσθον", _;
    ind_pass_2: _, _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθε";
    ind_pass_3: "~ῐ~ᾰ~ᾰ~ᾰσσεται", _, "~ῐ~ᾰ~ᾰ~ᾰσσονται";
    subj_mid_1: "~ῐ~ᾰ~ᾰ~ᾰσσωμαι", _, "~ῐ~ᾰ~ᾰ~ᾰσσωμεθᾰ";
    subj_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσῃ", "~ῐ~ᾰ~ᾰ~ᾰσσησθον", _;
    subj_mid_3: "~ῐ~ᾰ~ᾰ~ᾰσσηται", _, "~ῐ~ᾰ~ᾰ~ᾰσσωνται";
    subj_pass_2: "~ῐ~ᾰ~ᾰ~ᾰσσῃ", _, "~ῐ~ᾰ~ᾰ~ᾰσσησθε";
    opt_mid_1: "~ῐ~ᾰ~ᾰ~ᾰσσοιμην", _, "~ῐ~ᾰ~ᾰ~ᾰσσοιμεθᾰ";
    opt_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσοιο", "~ῐ~ᾰ~ᾰ~ᾰσσοισθον", _;
    opt_mid_3: "~ῐ~ᾰ~ᾰ~ᾰσσοιτο", "~ῐ~ᾰ~ᾰ~ᾰσσοισθην", "~ῐ~ᾰ~ᾰ~ᾰσσοιντο";
    opt_pass_2: _, _, "~ῐ~ᾰ~ᾰ~ᾰσσοισθε";
    imp_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσου", _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθε";
    imp_mid_3: "~ῐ~ᾰ~ᾰ~ᾰσσεσθω", _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθων";
    imp_pass_2: _, _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθε";
    imp_pass_3: _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθων", "~ῐ~ᾰ~ᾰ~ᾰσσεσθων";
    imperfect_mid_1: "~ῐ~ᾰ~ε~ᾰσσομην", _, "~ῐ~ᾰ~ε~ᾰσσομεθᾰ";
    imperfect_mid_2: "~ῐ~ᾰ~ε~ᾰσσου", "~ῐ~ᾰ~ε~ᾰσσεσθον", _;
    imperfect_mid_3: "~ῐ~ᾰ~ε~ᾰσσετο", "~ῐ~ᾰ~ε~ᾰσσεσθην", "~ῐ~ᾰ~ε~ᾰσσοντο";
    imperfect_pass_2: _, _, "~ῐ~ᾰ~ε~ᾰσσεσθε";
    future_ind_mid_1: "~ῐ~ᾰ~ᾰ~ᾰξομαι", _, "~ῐ~ᾰ~ᾰ~ᾰξομεθᾰ";
    future_ind_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξῃ|~ῐ~ᾰ~ᾰ~ᾰξει", "~ῐ~ᾰ~ᾰ~ᾰξεσθον", _;
    future_ind_mid_3: "~ῐ~ᾰ~ᾰ~ᾰξεται", _, _;
    future_opt_mid_1: "~ῐ~ᾰ~ᾰ~ᾰξοιμην", _, "~ῐ~ᾰ~ᾰ~ᾰξοιμεθᾰ";
    future_opt_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξοιο", _, _;
    future_opt_mid_3: _, "~ῐ~ᾰ~ᾰ~ᾰξοισθην", _;
    aorist_subj_mid_1: "~ῐ~ᾰ~ᾰ~ᾰξωμαι", _, _;
    aorist_subj_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξῃ", "~ῐ~ᾰ~ᾰ~ᾰξησθον", _;
    aorist_opt_mid_1: "~ῐ~ᾰ~ᾰ~ᾰξαιμην", _, _;
    aorist_opt_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξαιο", _, _;
    aorist_imp_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξαι", "~ῐ~ᾰ~ᾰ~ᾰξᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῐ~ᾰ~ᾰ~ᾰξᾰσθων";
    perfect_ind_mid_1: "~ῐ~ᾰ~ᾰ~ετᾰγμαι", _, "~ῐ~ᾰ~ᾰ~ετᾰγμεθᾰ";
    perfect_ind_mid_2: "~ῐ~ᾰ~ᾰ~ετᾰξαι", "~ῐ~ᾰ~ᾰ~ετᾰχθον", _;
    perfect_ind_mid_3: "~ῐ~ᾰ~ᾰ~ετᾰκται", _, "~ῐ~ᾰ~ᾰ~ετᾰγᾰται";
    perfect_ind_pass_2: _, _, "~ῐ~ᾰ~ᾰ~ετᾰχθε";
    perfect_imp_mid_2: _, _, "~ῐ~ᾰ~ᾰ~ετᾰχθε";
    perfect_imp_mid_3: "~ῐ~ᾰ~ᾰ~ετᾰχθω", _, "~ῐ~ᾰ~ᾰ~ετᾰχθων";
    perfect_imp_pass_2: "~ῐ~ᾰ~ᾰ~ετᾰξο", _, "~ῐ~ᾰ~ᾰ~ετᾰχθε";
    perfect_imp_pass_3: _, "~ῐ~ᾰ~ᾰ~ετᾰχθων", "~ῐ~ᾰ~ᾰ~ετᾰχθων";
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
    // Ἐπᾰνερχομαι: 1 words (ἐπᾰνερχομαι). Like Νισσομαι, except in 48 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ἐπᾰνερχομαι", 1, "~ᾰ~ρχομαι", like Νισσομαι;
    stem aorist_ind: "~α~λθ- like εἰσερχομαι";
    ind_act_1: "~ᾰ~ρχομαι", _, _;
    ind_mid_1: "~α~ρχομαι", _, "~α~ρχομεθᾰ";
    ind_mid_2: "~α~ρχῃ|~α~ρχει", "~α~ρχεσθον", _;
    ind_pass_2: _, _, "~α~ρχεσθε";
    ind_pass_3: "~α~ρχεται", _, "~α~ρχονται";
    subj_mid_1: "~α~ρχωμαι", _, "~α~ρχωμεθᾰ";
    subj_mid_2: "~α~ρχῃ", "~α~ρχησθον", _;
    subj_mid_3: "~α~ρχηται", _, "~α~ρχωνται";
    subj_pass_2: "~α~ρχῃ", _, "~α~ρχησθε";
    opt_mid_1: "~α~ρχοιμην", _, "~α~ρχοιμεθᾰ";
    opt_mid_2: "~α~ρχοιο", "~α~ρχοισθον", _;
    opt_mid_3: "~α~ρχοιτο", "~α~ρχοισθην", "~α~ρχοιντο";
    opt_pass_2: _, _, "~α~ρχοισθε";
    imp_mid_2: "~α~ρχου", _, "~α~ρχεσθε";
    imp_mid_3: "~α~ρχεσθω", _, "~α~ρχεσθων";
    imp_pass_2: _, _, "~α~ρχεσθε";
    imp_pass_3: _, "~α~ρχεσθων", "~α~ρχεσθων";
    aorist_subj_act_1: "~α~λθω", _, _;
    aorist_subj_act_2: "~α~λθῃς", "~α~λθητον", _;
    aorist_subj_act_3: _, _, "~α~λθωσῐ|~α~λθωσῐν";
    aorist_opt_act_1: "~α~λθοιμῐ", _, "~α~λθοιμεν";
    aorist_opt_act_2: "~α~λθοις", _, _;
    aorist_opt_act_3: "~α~λθοι", "~α~λθοιτην", "~α~λθοιεν";
    aorist_imp_act_2: "~α~λθε", _, "~α~λθετε";
    aorist_imp_act_3: _, _, "~α~λθοντων";
    infinitive_pres: _, _, "~α~ρχεσθαι";
    infinitive_aor: "~α~λθεῑν", _, _;
    participle_pres: _, _, "~α~ρχομενος";
    participle_aor: "~α~λθων", _, _;
    ),
    // Κιδνᾰμαι: 1 words (κιδνᾰμαι). Like Νισσομαι, except in 33 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("κιδνᾰμαι", 1, "~ι~ᾰμαι", like Νισσομαι;
    ind_act_1: "~ι~ᾰμαι", _, _;
    ind_mid_1: "~ῐ~ᾰμαι", _, "~ῐ~ᾰμεθᾰ";
    ind_mid_2: "~ῐ~ᾰσαι", "~ῐ~ᾰσθον", _;
    ind_pass_2: _, _, "~ῐ~ᾰσθε";
    ind_pass_3: "~ῐ~ᾰται", _, "~ῐ~ᾰνται";
    opt_mid_1: "~ῐ~αιμην", _, "~ῐ~αιμεθᾰ";
    opt_mid_2: "~ῐ~αιο", "~ῐ~αισθον", _;
    opt_mid_3: "~ῐ~αιτο", "~ῐ~αισθην", "~ῐ~αιντο";
    opt_pass_2: _, _, "~ῐ~αισθε";
    imp_mid_2: "~ῐ~ᾰσο|~ῐ~ω", _, "~ῐ~ᾰσθε";
    imp_mid_3: "~ῐ~ᾰσθω", _, "~ῐ~ᾰσθων";
    imp_pass_2: _, _, "~ῐ~ᾰσθε";
    imp_pass_3: _, "~ῐ~ᾰσθων", "~ῐ~ᾰσθων";
    imperfect_mid_1: "ἐ~ῐ~ᾰμην", _, "ἐ~ῐ~ᾰμεθᾰ";
    imperfect_mid_2: "ἐ~ῐ~ᾰσο", "ἐ~ῐ~ᾰσθον", _;
    imperfect_mid_3: "ἐ~ῐ~ᾰτο", "ἐ~ῐ~ᾰσθην", "ἐ~ῐ~ᾰντο";
    imperfect_pass_2: _, _, "ἐ~ῐ~ᾰσθε";
    infinitive_pres: _, _, "~ῐ~ᾰσθαι";
    participle_pres: _, _, "~ῐ~ᾰμενος";
    ),
    // Ἐπισκεπτομαι: 1 words (ἐπισκεπτομαι). Like Φειδομαι, except in 96 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ἐπισκεπτομαι", 1, "~ι~πτομαι", like Φειδομαι;
    stem future_pass: "~ῐ~πησ- like ποιεω";
    stem aorist_ind: "~ε~ψ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ε~- like ἐπισκεπτομαι";
    ind_act_1: "~ι~πτομαι", _, _;
    ind_mid_1: "~ῐ~πτομαι", _, "~ῐ~πτομεθᾰ";
    ind_mid_2: "~ῐ~πτῃ|~ῐ~πτει", "~ῐ~πτεσθον", _;
    ind_pass_2: _, _, "~ῐ~πτεσθε";
    ind_pass_3: "~ῐ~πτεται", _, "~ῐ~πτονται";
    subj_mid_1: "~ῐ~πτωμαι", _, "~ῐ~πτωμεθᾰ";
    subj_mid_2: "~ῐ~πτῃ", "~ῐ~πτησθον", _;
    subj_mid_3: "~ῐ~πτηται", _, "~ῐ~πτωνται";
    subj_pass_2: "~ῐ~πτῃ", _, "~ῐ~πτησθε";
    opt_mid_1: "~ῐ~πτοιμην", _, "~ῐ~πτοιμεθᾰ";
    opt_mid_2: "~ῐ~πτοιο", "~ῐ~πτοισθον", _;
    opt_mid_3: "~ῐ~πτοιτο", "~ῐ~πτοισθην", "~ῐ~πτοιντο";
    opt_pass_2: _, _, "~ῐ~πτοισθε";
    imp_mid_2: "~ῐ~πτου", _, "~ῐ~πτεσθε";
    imp_mid_3: "~ῐ~πτεσθω", _, "~ῐ~πτεσθων";
    imp_pass_2: _, _, "~ῐ~πτεσθε";
    imp_pass_3: _, "~ῐ~πτεσθων", "~ῐ~πτεσθων";
    imperfect_mid_1: "~ε~πτομην", _, "~ε~πτομεθᾰ";
    imperfect_mid_2: "~ε~πτου", "~ε~πτεσθον", _;
    imperfect_mid_3: "~ε~πτετο", "~ε~πτεσθην", "~ε~πτοντο";
    imperfect_pass_2: _, _, "~ε~πτεσθε";
    future_ind_mid_1: "~ῐ~ψομαι", _, "~ῐ~ψομεθᾰ";
    future_ind_mid_2: "~ῐ~ψῃ|~ῐ~ψει", "~ῐ~ψεσθον", _;
    future_ind_mid_3: "~ῐ~ψεται", _, _;
    future_opt_mid_1: "~ῐ~ψοιμην", _, "~ῐ~ψοιμεθᾰ";
    future_opt_mid_2: "~ῐ~ψοιο", _, _;
    future_opt_mid_3: _, "~ῐ~ψοισθην", _;
    aorist_subj_mid_1: "~ῐ~ψωμαι", _, _;
    aorist_subj_mid_2: "~ῐ~ψῃ", "~ῐ~ψησθον", _;
    aorist_subj_pass_1: "~ῐ~φθω|~ῐ~πω", _, "~ῐ~φθωμεν|~ῐ~πωμεν";
    aorist_subj_pass_2: "~ῐ~φθῃς|~ῐ~πῃς", "~ῐ~φθητον|~ῐ~πητον", "~ῐ~φθητε|~ῐ~πητε";
    aorist_subj_pass_3: "~ῐ~φθῃ|~ῐ~πῃ", _, "~ῐ~φθωσῐ|~ῐ~φθωσῐν|~ῐ~πωσῐ|~ῐ~πωσῐν";
    aorist_opt_mid_1: "~ῐ~ψαιμην", _, _;
    aorist_opt_mid_2: "~ῐ~ψαιο", _, _;
    aorist_opt_pass_1: "~ῐ~φθειην|~ῐ~πειην", _, "~ῐ~φθεῑμεν|~ῐ~φθειημεν|~ῐ~πεῑμεν|~ῐ~πειημεν";
    aorist_opt_pass_2: "~ῐ~φθειης|~ῐ~πειης", "~ῐ~φθεῑτον|~ῐ~φθειητον|~ῐ~πεῑτον|~ῐ~πειητον", "~ῐ~φθεῑτε|~ῐ~φθειητε|~ῐ~πεῑτε|~ῐ~πειητε";
    aorist_opt_pass_3: "~ῐ~φθειη|~ῐ~πειη", "~ῐ~φθειτην|~ῐ~φθειητην|~ῐ~πειτην|~ῐ~πειητην", "~ῐ~φθεῑεν|~ῐ~φθειησᾰν|~ῐ~πεῑεν|~ῐ~πειησᾰν";
    aorist_imp_mid_2: "~ῐ~ψαι", "~ῐ~ψᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῐ~ψᾰσθων";
    aorist_imp_pass_2: "~ῐ~φθητῐ|~ῐ~πηθῐ", _, _;
    aorist_imp_pass_3: "~ῐ~φθητω|~ῐ~πητω", "~ῐ~φθητων|~ῐ~πητων", "~ῐ~φθεντων|~ῐ~πεντων";
    perfect_ind_mid_1: "~ε~μμαι", _, "~ε~μμεθᾰ";
    perfect_ind_mid_2: "~ε~ψαι", "~ε~φθον", _;
    perfect_ind_mid_3: "~ε~πται", _, "~ε~πᾰται";
    perfect_ind_pass_2: _, _, "~ε~φθε";
    perfect_imp_mid_2: _, _, "~ε~φθε";
    perfect_imp_mid_3: "~ε~φθω", _, "~ε~φθων";
    perfect_imp_pass_2: "~ε~ψο", _, "~ε~φθε";
    perfect_imp_pass_3: _, "~ε~φθων", "~ε~φθων";
    infinitive_pres: _, _, "~ῐ~πτεσθαι";
    infinitive_aor: _, _, "~ῐ~φθηναι|~ῐ~πηναι";
    infinitive_perf: _, "~ε~φθαι", _;
    participle_pres: _, _, "~ῐ~πτομενος";
    participle_aor: _, "~ῐ~ψᾰμενος", "~ῐ~φθεις|~ῐ~πεις";
    participle_perf: _, _, "~ε~μμενος";
    ),
    // Κολυμβαω: 1 words (κολυμβαω). Like Μετεκῑᾰθον, except in 55 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
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
    imp_act_3: "~ῠ~ᾱτω", _, "~ῠ~ωντων";
    future_ind_act_1: "~ῠ~ησω", _, "~ῠ~ησομεν";
    future_ind_act_2: "~ῠ~ησεις", "~ῠ~ησετον", "~ῠ~ησετε";
    future_ind_act_3: _, _, "~ῠ~ησουσῐ|~ῠ~ησουσῐν";
    future_opt_act_1: "~ῠ~ησοιμῐ", _, "~ῠ~ησοιμεν";
    future_opt_act_2: _, "~ῠ~ησοιτον", "~ῠ~ησοιτε";
    future_opt_act_3: "~ῠ~ησοι", "~ῠ~ησοιτην", "~ῠ~ησοιεν";
    aorist_subj_act_1: "~ῠ~ησω", _, _;
    aorist_subj_act_2: "~ῠ~ησῃς", "~ῠ~ησητον", _;
    aorist_subj_act_3: _, _, "~ῠ~ησωσῐ|~ῠ~ησωσῐν";
    aorist_opt_act_1: "~ῠ~ησαιμῐ", _, "~ῠ~ησαιμεν";
    aorist_opt_act_2: "~ῠ~ησειᾰς|~ῠ~ησαις", _, _;
    aorist_opt_act_3: "~ῠ~ησειε|~ῠ~ησειεν|~ῠ~ησαι", "~ῠ~ησαιτην", "~ῠ~ησειᾰν|~ῠ~ησαιεν";
    aorist_imp_act_2: "~ῠ~ησον", _, "~ῠ~ησᾰτε";
    aorist_imp_act_3: _, _, "~ῠ~ησᾰντων";
    infinitive_pres: "~ῠ~ᾱν", _, _;
    infinitive_fut: "~ῠ~ησειν", _, _;
    infinitive_aor: "~ῠ~ησαι", _, _;
    participle_pres: "~ῠ~ων", _, _;
    participle_aor: "~ῠ~ησᾱς", _, _;
    ),
    // Δῐκεῑν: 1 words (δῐκεῑν). Like Ἐπορον, except in 2 cells: ind_act_1 sg, aorist_subj_act_3 pl.
    conjugation!("δῐκεῑν", 1, "~εῑν", like Ἐπορον;
    stem aorist_ind: "~- like ἐπεφνον";
    ind_act_1: "~εῑν", _, _;
    aorist_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    ),
    // Ὑπονοεω: 1 words (ὑπονοεω). Like Πεμπω, except in 166 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ὑπονοεω", 1, "ὑ~ο~εω", like Πεμπω;
    stem aorist_ind: "~ε~ησ- like ὑπονοεω";
    stem aorist_pass: "~ο~ηθ- like ὑπονοεω";
    stem pluperfect_act: "~ε%ε~ηκ- like ὑπονοεω";
    ind_act_1: "ῠ~ο~ω", _, "ῠ~ο~οῡμεν";
    ind_act_2: "ῠ~ο~εῑς", _, _;
    ind_act_3: "ῠ~ο~εῑ", "ῠ~ο~εῑτον", "ῠ~ο~οῡσῐ|ῠ~ο~οῡσῐν";
    ind_mid_1: "ῠ~ο~οῡμαι", _, "ῠ~ο~ουμεθᾰ";
    ind_mid_2: "ῠ~ο~εῑ|ῠ~ο~ῃ", "ῠ~ο~εῑσθον", _;
    ind_pass_2: _, _, "ῠ~ο~εῑσθε";
    ind_pass_3: "ῠ~ο~εῑται", _, "ῠ~ο~οῡνται";
    subj_act_1: "ῠ~ο~ω", _, "ῠ~ο~ωμεν";
    subj_act_2: "ῠ~ο~ῃς", _, _;
    subj_act_3: "ῠ~ο~ῃ", "ῠ~ο~ητον", "ῠ~ο~ωσῐ|ῠ~ο~ωσῐν";
    subj_mid_1: _, _, "ῠ~ο~ωμεθᾰ";
    subj_mid_2: _, "ῠ~ο~ησθον", _;
    subj_mid_3: "ῠ~ο~ηται", _, "ῠ~ο~ωνται";
    subj_pass_1: "ῠ~ο~ωμαι", _, _;
    subj_pass_2: _, _, "ῠ~ο~ησθε";
    opt_act_1: "ῠ~ο~οιην|ῠ~ο~οῑμῐ", _, "ῠ~ο~οῑμεν|ῠ~ο~οιημεν";
    opt_act_2: "ῠ~ο~οιης|ῠ~ο~οῑς", "ῠ~ο~οῑτον|ῠ~ο~οιητον", "ῠ~ο~οῑτε|ῠ~ο~οιητε";
    opt_act_3: "ῠ~ο~οιη|ῠ~ο~οῑ", "ῠ~ο~οιτην|ῠ~ο~οιητην", "ῠ~ο~οῑεν|ῠ~ο~οιησᾰν";
    opt_mid_1: "ῠ~ο~οιμην", _, "ῠ~ο~οιμεθᾰ";
    opt_mid_2: "ῠ~ο~οῑο", "ῠ~ο~οῑσθον", _;
    opt_mid_3: "ῠ~ο~οῑτο", "ῠ~ο~οισθην", "ῠ~ο~οῑντο";
    opt_pass_2: _, _, "ῠ~ο~οῑσθε";
    imp_act_2: "ῠ~ο~ει", "ῠ~ο~εῑτον", "ῠ~ο~εῑτε";
    imp_mid_2: "ῠ~ο~οῡ", _, "ῠ~ο~εῑσθε";
    imp_mid_3: "ῠ~ο~εισθω", _, "ῠ~ο~εισθων";
    imp_pass_2: _, _, "ῠ~ο~εῑσθε";
    imp_pass_3: _, "ῠ~ο~εισθων", "ῠ~ο~εισθων";
    imperfect_act_1: "ῠ~ε~ουν", _, "ῠ~ε~οῡμεν";
    imperfect_act_2: "ῠ~ε~εις", "ῠ~ε~εῑτον", "ῠ~ε~εῑτε";
    imperfect_act_3: "ῠ~ε~ει", "ῠ~ε~ειτην", "ῠ~ε~ουν";
    imperfect_mid_1: "ῠ~ε~ουμην", _, "ῠ~ε~ουμεθᾰ";
    imperfect_mid_2: "ῠ~ε~οῡ", _, _;
    imperfect_mid_3: "ῠ~ε~εῑτο", _, _;
    imperfect_pass_2: _, "ῠ~ε~εῑσθον", "ῠ~ε~εῑσθε";
    imperfect_pass_3: _, "ῠ~ε~εισθην", "ῠ~ε~οῡντο";
    future_ind_act_1: "ῠ~ο~ησω", _, "ῠ~ο~ησομεν";
    future_ind_act_2: "ῠ~ο~ησεις", "ῠ~ο~ησετον", "ῠ~ο~ησετε";
    future_ind_act_3: _, _, "ῠ~ο~ησουσῐ|ῠ~ο~ησουσῐν";
    future_ind_mid_1: "ῠ~ο~ησομαι", _, "ῠ~ο~ησομεθᾰ";
    future_ind_mid_2: "ῠ~ο~ησῃ|ῠ~ο~ησει", "ῠ~ο~ησεσθον", _;
    future_ind_mid_3: "ῠ~ο~ησεται", _, _;
    future_ind_pass_1: "ῠ~ο~ηθησομαι", _, _;
    future_ind_pass_2: "ῠ~ο~ηθησῃ|ῠ~ο~ηθησει", _, _;
    future_ind_pass_3: _, "ῠ~ο~ηθησεσθον", _;
    future_opt_act_1: "ῠ~ο~ησοιμῐ", _, "ῠ~ο~ησοιμεν";
    future_opt_act_2: _, "ῠ~ο~ησοιτον", "ῠ~ο~ησοιτε";
    future_opt_act_3: "ῠ~ο~ησοι", "ῠ~ο~ησοιτην", "ῠ~ο~ησοιεν";
    future_opt_mid_1: "ῠ~ο~ησοιμην", _, "ῠ~ο~ησοιμεθᾰ";
    future_opt_mid_2: "ῠ~ο~ησοιο", _, _;
    future_opt_mid_3: _, "ῠ~ο~ησοισθην", _;
    aorist_ind_pass_1: "ῠ~ε~ηθην", _, _;
    aorist_subj_act_1: "ῠ~ο~ησω", _, _;
    aorist_subj_act_2: "ῠ~ο~ησῃς", "ῠ~ο~ησητον", _;
    aorist_subj_act_3: _, _, "ῠ~ο~ησωσῐ|ῠ~ο~ησωσῐν";
    aorist_subj_mid_1: "ῠ~ο~ησωμαι", _, _;
    aorist_subj_mid_2: "ῠ~ο~ησῃ", "ῠ~ο~ησησθον", _;
    aorist_opt_act_1: "ῠ~ο~ησαιμῐ", _, "ῠ~ο~ησαιμεν";
    aorist_opt_act_2: "ῠ~ο~ησειᾰς|ῠ~ο~ησαις", _, _;
    aorist_opt_act_3: "ῠ~ο~ησειε|ῠ~ο~ησειεν|ῠ~ο~ησαι", "ῠ~ο~ησαιτην", "ῠ~ο~ησειᾰν|ῠ~ο~ησαιεν";
    aorist_opt_mid_1: "ῠ~ο~ησαιμην", _, _;
    aorist_opt_mid_2: "ῠ~ο~ησαιο", _, _;
    aorist_imp_act_2: "ῠ~ο~ησον", _, "ῠ~ο~ησᾰτε";
    aorist_imp_act_3: _, _, "ῠ~ο~ησᾰντων";
    aorist_imp_mid_2: "ῠ~ο~ησαι", "ῠ~ο~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "ῠ~ο~ησᾰσθων";
    perfect_ind_act_1: "ῠ~ο%ε~ηκᾰ", _, "ῠ~ο%ε~ηκᾰμεν";
    perfect_ind_act_2: _, "ῠ~ο%ε~ηκᾰτον", _;
    perfect_ind_act_3: "ῠ~ο%ε~ηκε|ῠ~ο%ε~ηκεν", _, "ῠ~ο%ε~ηκᾱσῐ|ῠ~ο%ε~ηκᾱσῐν";
    perfect_ind_mid_1: "ῠ~ο%ε~ημαι", _, "ῠ~ο%ε~ημεθᾰ";
    perfect_ind_mid_2: "ῠ~ο%ε~ησαι", "ῠ~ο%ε~ησθον", _;
    perfect_ind_mid_3: "ῠ~ο%ε~ηται", _, "ῠ~ο%ε~ηνται";
    perfect_ind_pass_2: _, _, "ῠ~ο%ε~ησθε";
    perfect_subj_act_1: "ῠ~ο%ε~ηκω", _, _;
    perfect_subj_act_2: "ῠ~ο%ε~ηκῃς", "ῠ~ο%ε~ηκητον", "ῠ~ο%ε~ηκητε";
    perfect_subj_act_3: "ῠ~ο%ε~ηκῃ", _, "ῠ~ο%ε~ηκωσῐ|ῠ~ο%ε~ηκωσῐν";
    perfect_opt_act_1: "ῠ~ο%ε~ηκοιμῐ|ῠ~ο%ε~ηκοιην", _, "ῠ~ο%ε~ηκοιμεν";
    perfect_opt_act_2: "ῠ~ο%ε~ηκοις|ῠ~ο%ε~ηκοιης", _, _;
    perfect_opt_act_3: "ῠ~ο%ε~ηκοι|ῠ~ο%ε~ηκοιη", "ῠ~ο%ε~ηκοιτην", _;
    perfect_imp_act_2: "ῠ~ο%ε~ηκε", _, "ῠ~ο%ε~ηκετε";
    perfect_imp_act_3: _, _, "ῠ~ο%ε~ηκοντων";
    perfect_imp_mid_2: _, _, "ῠ~ο%ε~ησθε";
    perfect_imp_mid_3: "ῠ~ο%ε~ησθω", _, "ῠ~ο%ε~ησθων";
    perfect_imp_pass_2: "ῠ~ο%ε~ησο", _, "ῠ~ο%ε~ησθε";
    perfect_imp_pass_3: _, "ῠ~ο%ε~ησθων", "ῠ~ο%ε~ησθων";
    pluperfect_mid_1: "ῠ~ε%ε~ημην", _, _;
    pluperfect_mid_2: "ῠ~ε%ε~ησο", "ῠ~ε%ε~ησθον", _;
    pluperfect_mid_3: "ῠ~ε%ε~ητο", _, "ῠ~ε%ε~ηντο";
    pluperfect_pass_1: _, _, "ῠ~ε%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "ῠ~ε%ε~ησθε";
    pluperfect_pass_3: _, "ῠ~ε%ε~ησθην", _;
    infinitive_pres: "ῠ~ο~εῑν", _, "ῠ~ο~εῑσθαι";
    infinitive_fut: "ῠ~ο~ησειν", _, _;
    infinitive_aor: "ῠ~ο~ησαι", _, _;
    infinitive_perf: _, "ῠ~ο%ε~ησθαι", _;
    participle_pres: "ῠ~ο~ων", _, "ῠ~ο~ουμενος";
    participle_aor: "ῠ~ο~ησᾱς", "ῠ~ο~ησᾰμενος", _;
    participle_perf: "ῠ~ο%ε~ηκως", _, "ῠ~ο%ε~ημενος";
    ),
    // Ἀντᾰγωνιζομαι: 1 words (ἀντᾰγωνιζομαι). Like Φειδομαι, except in 83 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ἀντᾰγωνιζομαι", 1, "~ᾰ~ιζομαι", like Φειδομαι;
    stem future_pass: "~ᾰ~ῐσθησ- like ποιεω";
    stem aorist_ind: "~η~ῐσ- like μηκᾰομαι";
    stem aorist_ind_pass: "~η~ῐσθην- like ἐλπω";
    stem aorist_pass: "~ᾰ~ῐσθ- like ποιεω";
    ind_act_1: "~ᾰ~ιζομαι", _, _;
    ind_mid_1: "~ᾰ~ῐζομαι", _, "~ᾰ~ῐζομεθᾰ";
    ind_mid_2: "~ᾰ~ῐζῃ|~ᾰ~ῐζει", "~ᾰ~ῐζεσθον", _;
    ind_pass_2: _, _, "~ᾰ~ῐζεσθε";
    ind_pass_3: "~ᾰ~ῐζεται", _, "~ᾰ~ῐζονται";
    subj_mid_1: "~ᾰ~ῐζωμαι", _, "~ᾰ~ῐζωμεθᾰ";
    subj_mid_2: "~ᾰ~ῐζῃ", "~ᾰ~ῐζησθον", _;
    subj_mid_3: "~ᾰ~ῐζηται", _, "~ᾰ~ῐζωνται";
    subj_pass_2: "~ᾰ~ῐζῃ", _, "~ᾰ~ῐζησθε";
    opt_mid_1: "~ᾰ~ῐζοιμην", _, "~ᾰ~ῐζοιμεθᾰ";
    opt_mid_2: "~ᾰ~ῐζοιο", "~ᾰ~ῐζοισθον", _;
    opt_mid_3: "~ᾰ~ῐζοιτο", "~ᾰ~ῐζοισθην", "~ᾰ~ῐζοιντο";
    opt_pass_2: _, _, "~ᾰ~ῐζοισθε";
    imp_mid_2: "~ᾰ~ῐζου", _, "~ᾰ~ῐζεσθε";
    imp_mid_3: "~ᾰ~ῐζεσθω", _, "~ᾰ~ῐζεσθων";
    imp_pass_2: _, _, "~ᾰ~ῐζεσθε";
    imp_pass_3: _, "~ᾰ~ῐζεσθων", "~ᾰ~ῐζεσθων";
    imperfect_mid_1: "~η~ῐζομην", _, "~η~ῐζομεθᾰ";
    imperfect_mid_2: "~η~ῐζου", "~η~ῐζεσθον", _;
    imperfect_mid_3: "~η~ῐζετο", "~η~ῐζεσθην", "~η~ῐζοντο";
    imperfect_pass_2: _, _, "~η~ῐζεσθε";
    future_ind_mid_1: "~ᾰ~ῐοῡμαι", _, "~ᾰ~ῐουμεθᾰ";
    future_ind_mid_2: "~ᾰ~ῐεῑ|~ᾰ~ῐῃ", "~ᾰ~ῐεῑσθον", _;
    future_ind_mid_3: "~ᾰ~ῐεῑται", _, _;
    future_opt_mid_1: "~ᾰ~ῐοιμην", _, "~ᾰ~ῐοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ῐοῑο", _, _;
    future_opt_mid_3: _, "~ᾰ~ῐοισθην", _;
    aorist_subj_mid_1: "~ᾰ~ῐσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ῐσῃ", "~ᾰ~ῐσησθον", _;
    aorist_opt_mid_1: "~ᾰ~ῐσαιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~ῐσαιο", _, _;
    aorist_imp_mid_2: "~ᾰ~ῐσαι", "~ᾰ~ῐσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ῐσᾰσθων";
    perfect_ind_mid_1: "~η~ῐσμαι", _, "~η~ῐσμεθᾰ";
    perfect_ind_mid_2: "~η~ῐσαι", "~η~ῐσθον", _;
    perfect_ind_mid_3: "~η~ῐσται", _, "~η~ῐδᾰται";
    perfect_ind_pass_2: _, _, "~η~ῐσθε";
    perfect_imp_mid_2: _, _, "~η~ῐσθε";
    perfect_imp_mid_3: "~η~ῐσθω", _, "~η~ῐσθων";
    perfect_imp_pass_2: "~η~ῐσο", _, "~η~ῐσθε";
    perfect_imp_pass_3: _, "~η~ῐσθων", "~η~ῐσθων";
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
    // Διπλᾰσῐοομαι: 1 words (διπλᾰσῐοομαι). Like Νισσομαι, except in 27 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("διπλᾰσῐοομαι", 1, "~ι~οομαι", like Νισσομαι;
    ind_act_1: "~ι~οομαι", _, _;
    ind_mid_1: "~ῐ~οῡμαι", _, "~ῐ~ουμεθᾰ";
    ind_mid_2: "~ῐ~οῑ", "~ῐ~οῡσθον", _;
    ind_pass_2: _, _, "~ῐ~οῡσθε";
    ind_pass_3: "~ῐ~οῡται", _, "~ῐ~οῡνται";
    subj_mid_2: "~ῐ~οῑ", "~ῐ~ωσθον", _;
    subj_mid_3: "~ῐ~ωται", _, _;
    subj_pass_2: "~ῐ~οῑ", _, "~ῐ~ωσθε";
    opt_mid_2: "~ῐ~οῑο", "~ῐ~οῑσθον", _;
    opt_mid_3: "~ῐ~οῑτο", _, "~ῐ~οῑντο";
    opt_pass_2: _, _, "~ῐ~οῑσθε";
    imp_mid_2: "~ῐ~οῡ", _, "~ῐ~οῡσθε";
    imp_mid_3: "~ῐ~ουσθω", _, "~ῐ~ουσθων";
    imp_pass_2: _, _, "~ῐ~οῡσθε";
    imp_pass_3: _, "~ῐ~ουσθων", "~ῐ~ουσθων";
    infinitive_pres: _, _, "~ῐ~οῡσθαι";
    participle_pres: _, _, "~ῐ~ουμενος";
    ),
    // Ἐξικνεομαι: 1 words (ἐξικνεομαι). Like Ἐρχομαι, except in 68 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ἐξικνεομαι", 1, "~ικνεομαι", like Ἐρχομαι;
    stem aorist_ind: "~ῑκ- like ἐρχομαι";
    ind_act_1: "~ικνεομαι", _, _;
    ind_mid_1: "~ῐκνοῡμαι", _, "~ῐκνουμεθᾰ";
    ind_mid_2: "~ῐκνεῑ|~ῐκνῃ", "~ῐκνεῑσθον", _;
    ind_pass_2: _, _, "~ῐκνεῑσθε";
    ind_pass_3: "~ῐκνεῑται", _, "~ῐκνοῡνται";
    subj_mid_1: "~ῐκνωμαι", _, "~ῐκνωμεθᾰ";
    subj_mid_2: "~ῐκνῃ", "~ῐκνησθον", _;
    subj_mid_3: "~ῐκνηται", _, "~ῐκνωνται";
    subj_pass_2: "~ῐκνῃ", _, "~ῐκνησθε";
    opt_mid_1: "~ῐκνοιμην", _, "~ῐκνοιμεθᾰ";
    opt_mid_2: "~ῐκνοῑο", "~ῐκνοῑσθον", _;
    opt_mid_3: "~ῐκνοῑτο", "~ῐκνοισθην", "~ῐκνοῑντο";
    opt_pass_2: _, _, "~ῐκνοῑσθε";
    imp_mid_2: "~ῐκνοῡ", _, "~ῐκνεῑσθε";
    imp_mid_3: "~ῐκνεισθω", _, "~ῐκνεισθων";
    imp_pass_2: _, _, "~ῐκνεῑσθε";
    imp_pass_3: _, "~ῐκνεισθων", "~ῐκνεισθων";
    imperfect_mid_1: "~ῑκνουμην", _, "~ῑκνουμεθᾰ";
    imperfect_mid_2: "~ῑκνοῡ", "~ῑκνεῑσθον", _;
    imperfect_mid_3: "~ῑκνεῑτο", "~ῑκνεισθην", "~ῑκνοῡντο";
    imperfect_pass_2: _, _, "~ῑκνεῑσθε";
    future_ind_mid_1: "~ῐξομαι", _, "~ῐξομεθᾰ";
    future_ind_mid_2: "~ῐξῃ|~ῐξει", "~ῐξεσθον", _;
    future_ind_mid_3: "~ῐξεται", _, _;
    future_opt_mid_1: "~ῐξοιμην", _, "~ῐξοιμεθᾰ";
    future_opt_mid_2: "~ῐξοιο", _, _;
    future_opt_mid_3: _, "~ῐξοισθην", _;
    aorist_subj_mid_1: "~ῐκωμαι", _, _;
    aorist_subj_mid_2: "~ῐκῃ", "~ῐκησθον", _;
    aorist_opt_mid_1: "~ῐκοιμην", _, _;
    aorist_opt_mid_2: "~ῐκοιο", _, _;
    aorist_imp_mid_2: "~ῐκοῡ", "~ῐκεσθον", _;
    aorist_imp_mid_3: _, _, "~ῐκεσθων";
    perfect_ind_mid_1: "~ῑγμαι", _, "~ῑγμεθᾰ";
    perfect_ind_mid_2: "~ῑξαι", "~ῑχθον", _;
    perfect_ind_mid_3: "~ῑκται", _, "~ῑγᾰται";
    perfect_imp_mid_2: _, _, "~ῑχθε";
    perfect_imp_mid_3: "~ῑχθω", _, "~ῑχθων";
    infinitive_pres: _, _, "~ῐκνεῑσθαι";
    participle_pres: _, _, "~ῐκνουμενος";
    participle_aor: _, "~ῐκομενος", _;
    ),
    // Καταπετροω: 1 words (καταπετροω). Like Λαλαγεω, except in 131 cells: ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("καταπετροω", 1, "~α~α~οω", like Λαλαγεω;
    stem future_pass: "~ᾰ~ᾰ~ωθησ- like ποιεω";
    stem aorist_ind: "~ᾰ~ε~ωσ- like εὐλογεω";
    stem aorist_ind_pass: "~ᾰ~ε~ωθην- like ἐλπω";
    stem aorist_pass: "~ᾰ~ᾰ~ωθ- like ποιεω";
    stem pluperfect_act: "~ᾰ~ε~ωκ- like εὐλογεω";
    ind_act_2: "~ᾰ~ᾰ~οῑς", _, _;
    ind_act_3: "~ᾰ~ᾰ~οῑ", "~ᾰ~ᾰ~οῡτον", _;
    ind_mid_2: "~ᾰ~ᾰ~οῑ", "~ᾰ~ᾰ~οῡσθον", _;
    ind_pass_2: _, _, "~ᾰ~ᾰ~οῡσθε";
    ind_pass_3: "~ᾰ~ᾰ~οῡται", _, _;
    subj_act_2: "~ᾰ~ᾰ~οῑς", _, _;
    subj_act_3: "~ᾰ~ᾰ~οῑ", "~ᾰ~ᾰ~ωτον", _;
    subj_mid_2: _, "~ᾰ~ᾰ~ωσθον", _;
    subj_mid_3: "~ᾰ~ᾰ~ωται", _, _;
    subj_pass_2: _, _, "~ᾰ~ᾰ~ωσθε";
    imp_act_2: "~ᾰ~ᾰ~ου", "~ᾰ~ᾰ~οῡτον", "~ᾰ~ᾰ~οῡτε";
    imp_mid_2: _, _, "~ᾰ~ᾰ~οῡσθε";
    imp_mid_3: "~ᾰ~ᾰ~ουσθω", _, "~ᾰ~ᾰ~ουσθων";
    imp_pass_2: _, _, "~ᾰ~ᾰ~οῡσθε";
    imp_pass_3: _, "~ᾰ~ᾰ~ουσθων", "~ᾰ~ᾰ~ουσθων";
    imperfect_act_1: "~ᾰ~ε~ουν", _, "~ᾰ~ε~οῡμεν";
    imperfect_act_2: "~ᾰ~ε~ους", "~ᾰ~ε~οῡτον", "~ᾰ~ε~οῡτε";
    imperfect_act_3: "~ᾰ~ε~ου", "~ᾰ~ε~ουτην", "~ᾰ~ε~ουν";
    imperfect_mid_1: "~ᾰ~ε~ουμην", _, "~ᾰ~ε~ουμεθᾰ";
    imperfect_mid_2: "~ᾰ~ε~οῡ", _, _;
    imperfect_mid_3: "~ᾰ~ε~οῡτο", _, _;
    imperfect_pass_2: _, "~ᾰ~ε~οῡσθον", "~ᾰ~ε~οῡσθε";
    imperfect_pass_3: _, "~ᾰ~ε~ουσθην", "~ᾰ~ε~οῡντο";
    future_ind_act_1: "~ᾰ~ᾰ~ωσω", _, "~ᾰ~ᾰ~ωσομεν";
    future_ind_act_2: "~ᾰ~ᾰ~ωσεις", "~ᾰ~ᾰ~ωσετον", "~ᾰ~ᾰ~ωσετε";
    future_ind_act_3: _, _, "~ᾰ~ᾰ~ωσουσῐ|~ᾰ~ᾰ~ωσουσῐν";
    future_ind_mid_1: "~ᾰ~ᾰ~ωσομαι", _, "~ᾰ~ᾰ~ωσομεθᾰ";
    future_ind_mid_2: "~ᾰ~ᾰ~ωσῃ|~ᾰ~ᾰ~ωσει", "~ᾰ~ᾰ~ωσεσθον", _;
    future_ind_mid_3: "~ᾰ~ᾰ~ωσεται", _, _;
    future_opt_act_1: "~ᾰ~ᾰ~ωσοιμῐ", _, "~ᾰ~ᾰ~ωσοιμεν";
    future_opt_act_2: _, "~ᾰ~ᾰ~ωσοιτον", "~ᾰ~ᾰ~ωσοιτε";
    future_opt_act_3: "~ᾰ~ᾰ~ωσοι", "~ᾰ~ᾰ~ωσοιτην", "~ᾰ~ᾰ~ωσοιεν";
    future_opt_mid_1: "~ᾰ~ᾰ~ωσοιμην", _, "~ᾰ~ᾰ~ωσοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ᾰ~ωσοιο", _, _;
    future_opt_mid_3: _, "~ᾰ~ᾰ~ωσοισθην", _;
    aorist_subj_act_1: "~ᾰ~ᾰ~ωσω", _, _;
    aorist_subj_act_2: "~ᾰ~ᾰ~ωσῃς", "~ᾰ~ᾰ~ωσητον", _;
    aorist_subj_act_3: _, _, "~ᾰ~ᾰ~ωσωσῐ|~ᾰ~ᾰ~ωσωσῐν";
    aorist_subj_mid_1: "~ᾰ~ᾰ~ωσωμαι", _, _;
    aorist_subj_mid_2: "~ᾰ~ᾰ~ωσῃ", "~ᾰ~ᾰ~ωσησθον", _;
    aorist_opt_act_1: "~ᾰ~ᾰ~ωσαιμῐ", _, "~ᾰ~ᾰ~ωσαιμεν";
    aorist_opt_act_2: "~ᾰ~ᾰ~ωσειᾰς|~ᾰ~ᾰ~ωσαις", _, _;
    aorist_opt_act_3: "~ᾰ~ᾰ~ωσειε|~ᾰ~ᾰ~ωσειεν|~ᾰ~ᾰ~ωσαι", "~ᾰ~ᾰ~ωσαιτην", "~ᾰ~ᾰ~ωσειᾰν|~ᾰ~ᾰ~ωσαιεν";
    aorist_opt_mid_1: "~ᾰ~ᾰ~ωσαιμην", _, _;
    aorist_opt_mid_2: "~ᾰ~ᾰ~ωσαιο", _, _;
    aorist_imp_act_2: "~ᾰ~ᾰ~ωσον", _, "~ᾰ~ᾰ~ωσᾰτε";
    aorist_imp_act_3: _, _, "~ᾰ~ᾰ~ωσᾰντων";
    aorist_imp_mid_2: "~ᾰ~ᾰ~ωσαι", "~ᾰ~ᾰ~ωσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ᾰ~ᾰ~ωσᾰσθων";
    perfect_ind_act_1: "~ᾰ~ε~ωκᾰ", _, "~ᾰ~ε~ωκᾰμεν";
    perfect_ind_act_2: _, "~ᾰ~ε~ωκᾰτον", _;
    perfect_ind_act_3: "~ᾰ~ε~ωκε|~ᾰ~ε~ωκεν", _, "~ᾰ~ε~ωκᾱσῐ|~ᾰ~ε~ωκᾱσῐν";
    perfect_ind_mid_1: "~ᾰ~ε~ωμαι", _, "~ᾰ~ε~ωμεθᾰ";
    perfect_ind_mid_2: "~ᾰ~ε~ωσαι", "~ᾰ~ε~ωσθον", _;
    perfect_ind_mid_3: "~ᾰ~ε~ωται", _, "~ᾰ~ε~ωνται";
    perfect_ind_pass_2: _, _, "~ᾰ~ε~ωσθε";
    perfect_subj_act_1: "~ᾰ~ε~ωκω", _, _;
    perfect_subj_act_2: "~ᾰ~ε~ωκῃς", "~ᾰ~ε~ωκητον", "~ᾰ~ε~ωκητε";
    perfect_subj_act_3: "~ᾰ~ε~ωκῃ", _, "~ᾰ~ε~ωκωσῐ|~ᾰ~ε~ωκωσῐν";
    perfect_opt_act_1: "~ᾰ~ε~ωκοιμῐ|~ᾰ~ε~ωκοιην", _, "~ᾰ~ε~ωκοιμεν";
    perfect_opt_act_2: "~ᾰ~ε~ωκοις|~ᾰ~ε~ωκοιης", _, _;
    perfect_opt_act_3: "~ᾰ~ε~ωκοι|~ᾰ~ε~ωκοιη", "~ᾰ~ε~ωκοιτην", _;
    perfect_imp_act_2: "~ᾰ~ε~ωκε", _, "~ᾰ~ε~ωκετε";
    perfect_imp_act_3: _, _, "~ᾰ~ε~ωκοντων";
    perfect_imp_mid_2: _, _, "~ᾰ~ε~ωσθε";
    perfect_imp_mid_3: "~ᾰ~ε~ωσθω", _, "~ᾰ~ε~ωσθων";
    perfect_imp_pass_2: "~ᾰ~ε~ωσο", _, "~ᾰ~ε~ωσθε";
    perfect_imp_pass_3: _, "~ᾰ~ε~ωσθων", "~ᾰ~ε~ωσθων";
    pluperfect_mid_1: "~ᾰ~ε~ωμην", _, _;
    pluperfect_mid_2: "~ᾰ~ε~ωσο", "~ᾰ~ε~ωσθον", _;
    pluperfect_mid_3: "~ᾰ~ε~ωτο", _, "~ᾰ~ε~ωντο";
    pluperfect_pass_1: _, _, "~ᾰ~ε~ωμεθᾰ";
    pluperfect_pass_2: _, _, "~ᾰ~ε~ωσθε";
    pluperfect_pass_3: _, "~ᾰ~ε~ωσθην", _;
    infinitive_pres: "~ᾰ~ᾰ~οῡν", _, "~ᾰ~ᾰ~οῡσθαι";
    infinitive_fut: "~ᾰ~ᾰ~ωσειν", _, _;
    infinitive_aor: "~ᾰ~ᾰ~ωσαι", _, _;
    infinitive_perf: _, "~ᾰ~ε~ωσθαι", _;
    participle_aor: "~ᾰ~ᾰ~ωσᾱς", "~ᾰ~ᾰ~ωσᾰμενος", _;
    participle_perf: "~ᾰ~ε~ωκως", _, "~ᾰ~ε~ωμενος";
    ),
    // Λαλαχευομαι: 1 words (λαλαχευομαι). Like Ἀποστατεω, except in 64 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("λαλαχευομαι", 1, "~α~α~ομαι", like Ἀποστατεω;
    stem imperfect: "~ᾰ~ᾰ~- like λαλαχευομαι";
    stem future_pass: "~ᾰ~ᾰ~θησ- like ποιεω";
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
    imp_act_3: derived, _, derived;
    imp_pass_2: "~ᾰ~ᾰ~ου", "~ᾰ~ᾰ~εσθον", "~ᾰ~ᾰ~εσθε";
    imp_pass_3: "~ᾰ~ᾰ~εσθω", "~ᾰ~ᾰ~εσθων", "~ᾰ~ᾰ~εσθων";
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    infinitive_pres: derived, _, _;
    participle_pres: derived, _, _;
    ),
    // Μικρογρᾰφεω: 1 words (μικρογρᾰφεω). Like Διπλᾰσῐοομαι, except in 43 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("μικρογρᾰφεω", 1, "~ι~εω", like Διπλᾰσῐοομαι;
    ind_act_1: "~ῐ~ω", _, "~ῐ~οῡμεν";
    ind_act_2: "~ῐ~εῑς", _, _;
    ind_act_3: "~ῐ~εῑ", "~ῐ~εῑτον", "~ῐ~οῡσῐ|~ῐ~οῡσῐν";
    ind_mid_2: "~ῐ~εῑ|~ῐ~ῃ", "~ῐ~εῑσθον", _;
    ind_pass_2: _, _, "~ῐ~εῑσθε";
    ind_pass_3: "~ῐ~εῑται", _, _;
    subj_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    subj_act_2: "~ῐ~ῃς", _, _;
    subj_act_3: "~ῐ~ῃ", "~ῐ~ητον", "~ῐ~ωσῐ|~ῐ~ωσῐν";
    subj_mid_1: derived, _, _;
    subj_mid_2: derived, "~ῐ~ησθον", _;
    subj_mid_3: "~ῐ~ηται", _, _;
    subj_pass_1: "~ῐ~ωμαι", _, _;
    subj_pass_2: derived, _, "~ῐ~ησθε";
    opt_act_1: "~ῐ~οιην|~ῐ~οῑμῐ", _, "~ῐ~οῑμεν|~ῐ~οιημεν";
    opt_act_2: "~ῐ~οιης|~ῐ~οῑς", "~ῐ~οῑτον|~ῐ~οιητον", "~ῐ~οῑτε|~ῐ~οιητε";
    opt_act_3: "~ῐ~οιη|~ῐ~οῑ", "~ῐ~οιτην|~ῐ~οιητην", "~ῐ~οῑεν|~ῐ~οιησᾰν";
    imp_act_2: "~ῐ~ει", "~ῐ~εῑτον", "~ῐ~εῑτε";
    imp_mid_2: _, _, "~ῐ~εῑσθε";
    imp_mid_3: "~ῐ~εισθω", _, "~ῐ~εισθων";
    imp_pass_2: _, _, "~ῐ~εῑσθε";
    imp_pass_3: _, "~ῐ~εισθων", "~ῐ~εισθων";
    infinitive_pres: "~ῐ~εῑν", _, "~ῐ~εῑσθαι";
    participle_pres: "~ῐ~ων", _, _;
    ),
    // Λῐνοστᾰτεω: 1 words (λῐνοστᾰτεω). Like Ἐπῐκᾰλεω, except in 108 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("λῐνοστᾰτεω", 1, "~ῐ~ᾰ~εω", like Ἐπῐκᾰλεω;
    stem future_pass: "~ι~α~ηθησ- like ποιεω";
    stem aorist_ind: "~ι~α~ησ- like ποιεω";
    stem aorist_ind_pass: "~ι~α~ηθην- like ἐχω";
    stem aorist_pass: "~ι~α~ηθ- like ποιεω";
    stem pluperfect_act: "~ελι~α~ηκ- like δῐκεω";
    imperfect_act_1: "ἐ~ι~α~ουν", _, "ἐ~ι~α~οῡμεν";
    imperfect_act_2: "ἐ~ι~α~εις", "ἐ~ι~α~εῑτον", "ἐ~ι~α~εῑτε";
    imperfect_act_3: "ἐ~ι~α~ει", "ἐ~ι~α~ειτην", "ἐ~ι~α~ουν";
    imperfect_mid_1: "ἐ~ι~α~ουμην", _, "ἐ~ι~α~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ι~α~οῡ", _, _;
    imperfect_mid_3: "ἐ~ι~α~εῑτο", _, _;
    imperfect_pass_2: _, "ἐ~ι~α~εῑσθον", "ἐ~ι~α~εῑσθε";
    imperfect_pass_3: _, "ἐ~ι~α~εισθην", "ἐ~ι~α~οῡντο";
    future_ind_act_1: "~ι~α~ησω", _, "~ι~α~ησομεν";
    future_ind_act_2: "~ι~α~ησεις", "~ι~α~ησετον", "~ι~α~ησετε";
    future_ind_act_3: _, _, "~ι~α~ησουσῐ|~ι~α~ησουσῐν";
    future_ind_mid_1: "~ι~α~ησομαι", _, "~ι~α~ησομεθᾰ";
    future_ind_mid_2: "~ι~α~ησῃ|~ι~α~ησει", "~ι~α~ησεσθον", _;
    future_ind_mid_3: "~ι~α~ησεται", _, _;
    future_opt_act_1: "~ι~α~ησοιμῐ", _, "~ι~α~ησοιμεν";
    future_opt_act_2: derived, "~ι~α~ησοιτον", "~ι~α~ησοιτε";
    future_opt_act_3: "~ι~α~ησοι", "~ι~α~ησοιτην", "~ι~α~ησοιεν";
    future_opt_mid_1: "~ι~α~ησοιμην", _, "~ι~α~ησοιμεθᾰ";
    future_opt_mid_2: "~ι~α~ησοιο", _, _;
    future_opt_mid_3: _, "~ι~α~ησοισθην", _;
    aorist_subj_act_1: "~ι~α~ησω", _, _;
    aorist_subj_act_2: "~ι~α~ησῃς", "~ι~α~ησητον", _;
    aorist_subj_act_3: _, _, "~ι~α~ησωσῐ|~ι~α~ησωσῐν";
    aorist_subj_mid_1: "~ι~α~ησωμαι", _, _;
    aorist_subj_mid_2: "~ι~α~ησῃ", "~ι~α~ησησθον", _;
    aorist_opt_act_1: "~ι~α~ησαιμῐ", _, "~ι~α~ησαιμεν";
    aorist_opt_act_2: "~ι~α~ησειᾰς|~ι~α~ησαις", _, _;
    aorist_opt_act_3: "~ι~α~ησειε|~ι~α~ησειεν|~ι~α~ησαι", "~ι~α~ησαιτην", "~ι~α~ησειᾰν|~ι~α~ησαιεν";
    aorist_opt_mid_1: "~ι~α~ησαιμην", _, _;
    aorist_opt_mid_2: "~ι~α~ησαιο", _, _;
    aorist_imp_act_2: "~ι~α~ησον", _, "~ι~α~ησᾰτε";
    aorist_imp_act_3: _, _, "~ι~α~ησᾰντων";
    aorist_imp_mid_2: "~ι~α~ησαι", "~ι~α~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ι~α~ησᾰσθων";
    perfect_ind_act_1: "~ελι~α~ηκᾰ", _, "~ελι~α~ηκᾰμεν";
    perfect_ind_act_2: _, "~ελι~α~ηκᾰτον", _;
    perfect_ind_act_3: "~ελι~α~ηκε|~ελι~α~ηκεν", _, "~ελι~α~ηκᾱσῐ|~ελι~α~ηκᾱσῐν";
    perfect_ind_mid_1: "~ελι~α~ημαι", _, "~ελι~α~ημεθᾰ";
    perfect_ind_mid_2: "~ελι~α~ησαι", "~ελι~α~ησθον", _;
    perfect_ind_mid_3: "~ελι~α~ηται", _, "~ελι~α~ηνται";
    perfect_ind_pass_2: _, _, "~ελι~α~ησθε";
    perfect_subj_act_1: "~ελι~α~ηκω", _, _;
    perfect_subj_act_2: "~ελι~α~ηκῃς", "~ελι~α~ηκητον", "~ελι~α~ηκητε";
    perfect_subj_act_3: "~ελι~α~ηκῃ", _, "~ελι~α~ηκωσῐ|~ελι~α~ηκωσῐν";
    perfect_opt_act_1: "~ελι~α~ηκοιμῐ|~ελι~α~ηκοιην", _, "~ελι~α~ηκοιμεν";
    perfect_opt_act_2: "~ελι~α~ηκοις|~ελι~α~ηκοιης", _, _;
    perfect_opt_act_3: "~ελι~α~ηκοι|~ελι~α~ηκοιη", "~ελι~α~ηκοιτην", _;
    perfect_imp_act_2: "~ελι~α~ηκε", _, "~ελι~α~ηκετε";
    perfect_imp_act_3: _, _, "~ελι~α~ηκοντων";
    perfect_imp_mid_2: _, _, "~ελι~α~ησθε";
    perfect_imp_mid_3: "~ελι~α~ησθω", _, "~ελι~α~ησθων";
    perfect_imp_pass_2: "~ελι~α~ησο", _, "~ελι~α~ησθε";
    perfect_imp_pass_3: _, "~ελι~α~ησθων", "~ελι~α~ησθων";
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
    // Ἀδησᾰ: 1 words (ἀδησᾰ). Like Ἐπορον, except in 36 cells: ind_act_1 sg, aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, aorist_opt_act_1 sg, ….
    conjugation!("ἀδησᾰ", 1, "~σᾰ", like Ἐπορον;
    stem aorist_ind: "~σ- like ξυνναυμᾰχεω";
    ind_act_1: "~σᾰ", _, _;
    aorist_subj_act_1: "~σω", _, _;
    aorist_subj_act_2: "~σῃς", "~σητον", _;
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_opt_act_1: "~σαιμῐ", _, "~σαιμεν";
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", "~σαιτην", "~σειᾰν|~σαιεν";
    aorist_imp_act_2: "~σον", _, "~σᾰτε";
    aorist_imp_act_3: _, _, "~σᾰντων";
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: _, "~κᾰτον", _;
    perfect_ind_act_3: "~κε|~κεν", _, "~κᾱσῐ|~κᾱσῐν";
    perfect_subj_act_1: "~κω", _, _;
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ|~κωσῐν";
    perfect_opt_act_1: "~κοιμῐ|~κοιην", _, "~κοιμεν";
    perfect_opt_act_2: "~κοις|~κοιης", _, _;
    perfect_opt_act_3: "~κοι|~κοιη", "~κοιτην", _;
    perfect_imp_act_2: "~κε", _, "~κετε";
    perfect_imp_act_3: _, _, "~κοντων";
    infinitive_aor: "~σαι", _, _;
    participle_aor: "~σᾱς", _, _;
    participle_perf: "~κως", _, _;
    ),
    // Ὑποκοριζομαι: 1 words (ὑποκοριζομαι). Like Δῐακειμαι, except in 49 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, subj_mid_1 sg, ….
    conjugation!("ὑποκοριζομαι", 1, "~ο~ιζομαι", like Δῐακειμαι;
    stem imperfect: "~ε~ῐζ- like ὑποκοριζομαι";
    stem aorist_ind: "~ε~ῐσ- like μηκᾰομαι";
    stem pluperfect: "~ε%ε~ῐ- like ὑποκοριζομαι";
    ind_act_1: "~ο~ιζομαι", _, _;
    ind_mid_1: "~ο~ῐζομαι", _, "~ο~ῐζομεθᾰ";
    ind_mid_2: "~ο~ῐζῃ|~ο~ῐζει", "~ο~ῐζεσθον", _;
    subj_mid_1: "~ο~ῐζωμαι", _, "~ο~ῐζωμεθᾰ";
    subj_mid_2: "~ο~ῐζῃ", "~ο~ῐζησθον", _;
    subj_mid_3: "~ο~ῐζηται", _, "~ο~ῐζωνται";
    opt_mid_1: "~ο~ῐζοιμην", _, "~ο~ῐζοιμεθᾰ";
    opt_mid_2: "~ο~ῐζοιο", "~ο~ῐζοισθον", _;
    opt_mid_3: "~ο~ῐζοιτο", "~ο~ῐζοισθην", "~ο~ῐζοιντο";
    imp_mid_2: "~ο~ῐζου", _, "~ο~ῐζεσθε";
    imp_mid_3: "~ο~ῐζεσθω", _, "~ο~ῐζεσθων";
    future_ind_mid_1: "~ο~ῐοῡμαι", _, "~ο~ῐουμεθᾰ";
    future_ind_mid_2: "~ο~ῐεῑ|~ο~ῐῃ", "~ο~ῐεῑσθον", _;
    future_ind_mid_3: "~ο~ῐεῑται", _, _;
    future_opt_mid_1: "~ο~ῐοιμην", _, "~ο~ῐοιμεθᾰ";
    future_opt_mid_2: "~ο~ῐοῑο", _, _;
    future_opt_mid_3: _, "~ο~ῐοισθην", _;
    aorist_subj_mid_1: "~ο~ῐσωμαι", _, _;
    aorist_subj_mid_2: "~ο~ῐσῃ", "~ο~ῐσησθον", _;
    aorist_opt_mid_1: "~ο~ῐσαιμην", _, _;
    aorist_opt_mid_2: "~ο~ῐσαιο", _, _;
    aorist_imp_mid_2: "~ο~ῐσαι", "~ο~ῐσᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ο~ῐσᾰσθων";
    perfect_ind_mid_1: "~ο%ε~ῐσμαι", _, "~ο%ε~ῐσμεθᾰ";
    perfect_ind_mid_2: "~ο%ε~ῐσαι", "~ο%ε~ῐσθον", _;
    perfect_ind_mid_3: "~ο%ε~ῐσται", _, "~ο%ε~ῐδᾰται";
    perfect_imp_mid_2: _, _, "~ο%ε~ῐσθε";
    perfect_imp_mid_3: "~ο%ε~ῐσθω", _, "~ο%ε~ῐσθων";
    participle_aor: _, "~ο~ῐσᾰμενος", _;
    ),
    // Ἰχᾰνᾰω: 1 words (ἰχᾰνᾰω). Like Μικρογρᾰφεω, except in 55 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ἰχᾰνᾰω", 1, "~ᾰ~ᾰω", like Μικρογρᾰφεω;
    ind_act_1: "~α~ω", _, "~α~ωμεν";
    ind_act_2: "~α~ᾳς", _, _;
    ind_act_3: "~α~ᾳ", "~α~ᾱτον", "~α~ωσῐ|~α~ωσῐν";
    ind_mid_1: "~α~ωμαι", _, "~α~ωμεθᾰ";
    ind_mid_2: "~α~ᾳ", "~α~ᾱσθον", _;
    ind_pass_2: _, _, "~α~ᾱσθε";
    ind_pass_3: "~α~ᾱται", _, "~α~ωνται";
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
    opt_mid_2: "~α~ῳο", "~α~ῳσθον", _;
    opt_mid_3: "~α~ῳτο", "~α~ῳσθην", "~α~ῳντο";
    opt_pass_2: _, _, "~α~ῳσθε";
    imp_act_2: "~α~ᾱ", "~α~ᾱτον", "~α~ᾱτε";
    imp_mid_2: "~α~ω", _, "~α~ᾱσθε";
    imp_mid_3: "~α~ᾱσθω", _, "~α~ᾱσθων";
    imp_pass_2: _, _, "~α~ᾱσθε";
    imp_pass_3: _, "~α~ᾱσθων", "~α~ᾱσθων";
    infinitive_pres: "~α~ᾱν", _, "~α~ᾱσθαι";
    participle_pres: "~α~ων", _, "~α~ωμενος";
    ),
    // Ᾰντᾰποκρῑνομαι: 1 words (ᾰντᾰποκρῑνομαι). Like Φειδομαι, except in 79 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ᾰντᾰποκρῑνομαι", 1, "ᾰ~ο~ῑνομαι", like Φειδομαι;
    stem future_pass: "~ο~ῐθησ- like ᾰντᾰποκρῑνομαι";
    stem aorist_pass: "~ο~ῐθ- like ᾰντᾰποκρῑνομαι";
    ind_act_1: "ᾰ~ο~ῑνομαι", _, _;
    ind_mid_1: "ἀ~ο~ῑνομαι", _, "ἀ~ο~ῑνομεθᾰ";
    ind_mid_2: "ἀ~ο~ῑνῃ|ἀ~ο~ῑνει", "ἀ~ο~ῑνεσθον", _;
    ind_pass_2: _, _, "ἀ~ο~ῑνεσθε";
    ind_pass_3: "ἀ~ο~ῑνεται", _, "ἀ~ο~ῑνονται";
    subj_mid_1: "ἀ~ο~ῑνωμαι", _, "ἀ~ο~ῑνωμεθᾰ";
    subj_mid_2: "ἀ~ο~ῑνῃ", "ἀ~ο~ῑνησθον", _;
    subj_mid_3: "ἀ~ο~ῑνηται", _, "ἀ~ο~ῑνωνται";
    subj_pass_2: "ἀ~ο~ῑνῃ", _, "ἀ~ο~ῑνησθε";
    opt_mid_1: "ἀ~ο~ῑνοιμην", _, "ἀ~ο~ῑνοιμεθᾰ";
    opt_mid_2: "ἀ~ο~ῑνοιο", "ἀ~ο~ῑνοισθον", _;
    opt_mid_3: "ἀ~ο~ῑνοιτο", "ἀ~ο~ῑνοισθην", "ἀ~ο~ῑνοιντο";
    opt_pass_2: _, _, "ἀ~ο~ῑνοισθε";
    imp_mid_2: "ἀ~ο~ῑνου", _, "ἀ~ο~ῑνεσθε";
    imp_mid_3: "ἀ~ο~ῑνεσθω", _, "ἀ~ο~ῑνεσθων";
    imp_pass_2: _, _, "ἀ~ο~ῑνεσθε";
    imp_pass_3: _, "ἀ~ο~ῑνεσθων", "ἀ~ο~ῑνεσθων";
    imperfect_mid_1: "ἀ~ε~ῑνομην", _, "ἀ~ε~ῑνομεθᾰ";
    imperfect_mid_2: "ἀ~ε~ῑνου", "ἀ~ε~ῑνεσθον", _;
    imperfect_mid_3: "ἀ~ε~ῑνετο", "ἀ~ε~ῑνεσθην", "ἀ~ε~ῑνοντο";
    imperfect_pass_2: _, _, "ἀ~ε~ῑνεσθε";
    future_ind_mid_1: "ἀ~ο~ῐνοῡμαι", _, "ἀ~ο~ῐνουμεθᾰ";
    future_ind_mid_2: "ἀ~ο~ῐνεῑ|ἀ~ο~ῐνῃ", "ἀ~ο~ῐνεῑσθον", _;
    future_ind_mid_3: "ἀ~ο~ῐνεῑται", _, _;
    future_opt_mid_1: "ἀ~ο~ῐνοιμην", _, "ἀ~ο~ῐνοιμεθᾰ";
    future_opt_mid_2: "ἀ~ο~ῐνοῑο", _, _;
    future_opt_mid_3: _, "ἀ~ο~ῐνοισθην", _;
    aorist_ind_mid_1: "ἀ~ε~ῑνᾰμην", _, _;
    aorist_ind_mid_2: "ἀ~ε~ῑνω", "ἀ~ε~ῑνᾰσθον", _;
    aorist_ind_pass_1: "ἀ~ε~ῐθην", _, _;
    aorist_subj_mid_1: "ἀ~ο~ῑνωμαι", _, _;
    aorist_subj_mid_2: "ἀ~ο~ῑνῃ", "ἀ~ο~ῑνησθον", _;
    aorist_opt_mid_1: "ἀ~ο~ῑναιμην", _, _;
    aorist_opt_mid_2: "ἀ~ο~ῑναιο", _, _;
    aorist_imp_mid_2: "ἀ~ο~ῑναι", "ἀ~ο~ῑνᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἀ~ο~ῑνᾰσθων";
    perfect_ind_mid_1: "ἀ~ο%ε~ῐμαι", _, "ἀ~ο%ε~ῐμεθᾰ";
    perfect_ind_mid_2: "ἀ~ο%ε~ῐσαι", "ἀ~ο%ε~ῐσθον", _;
    perfect_ind_mid_3: "ἀ~ο%ε~ῐται", _, "ἀ~ο%ε~ῐνται";
    perfect_ind_pass_2: _, _, "ἀ~ο%ε~ῐσθε";
    perfect_imp_mid_2: _, _, "ἀ~ο%ε~ῐσθε";
    perfect_imp_mid_3: "ἀ~ο%ε~ῐσθω", _, "ἀ~ο%ε~ῐσθων";
    perfect_imp_pass_2: "ἀ~ο%ε~ῐσο", _, "ἀ~ο%ε~ῐσθε";
    perfect_imp_pass_3: _, "ἀ~ο%ε~ῐσθων", "ἀ~ο%ε~ῐσθων";
    infinitive_pres: _, _, "ἀ~ο~ῑνεσθαι";
    infinitive_perf: _, "ἀ~ο%ε~ῐσθαι", _;
    participle_pres: _, _, "ἀ~ο~ῑνομενος";
    participle_aor: _, "ἀ~ο~ῑνᾰμενος", _;
    participle_perf: _, _, "ἀ~ο%ε~ῐμενος";
    ),
    // Ὑπεκτιθεμαι: 1 words (ὑπεκτιθεμαι). Like Δῐακειμαι, except in 79 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ὑπεκτιθεμαι", 1, "~κτι~εμαι", like Δῐακειμαι;
    stem future_pass: "~κ%ε~ησ- like ποιεω";
    stem aorist_ind_pass: "~ξετε~ην- like ἐλπω";
    stem aorist_pass: "~κτε~- like πειρω";
    ind_act_1: "~κτι~εμαι", _, _;
    ind_mid_1: "~κ%ῐ~εμαι", _, "~κ%ῐ~εμεθᾰ";
    ind_mid_2: "~κ%ῐ~εσαι", "~κ%ῐ~εσθον", _;
    ind_pass_2: _, _, "~κ%ῐ~εσθε";
    ind_pass_3: "~κ%ῐ~εται", _, "~κ%ῐ~ενται";
    subj_mid_1: "~κ%ῐ~ωμαι", _, "~κ%ῐ~ωμεθᾰ";
    subj_mid_2: "~κ%ῐ~ῃ", "~κ%ῐ~ησθον", _;
    subj_mid_3: "~κ%ῐ~ηται", _, "~κ%ῐ~ωνται";
    subj_pass_2: "~κ%ῐ~ῃ", _, "~κ%ῐ~ησθε";
    opt_mid_1: "~κ%ῐ~ειμην", _, "~κ%ῐ~ειμεθᾰ|~κ%ῐ~οιμεθᾰ";
    opt_mid_2: "~κ%ῐ~εῑο", "~κ%ῐ~εῑσθον|~κ%ῐ~οῑσθον", "~κ%ῐ~εῑσθε|~κ%ῐ~οῑσθε";
    opt_mid_3: "~κ%ῐ~εῑτο|~κ%ῐ~οῑτο", "~κ%ῐ~εισθην|~κ%ῐ~οισθην", "~κ%ῐ~εῑντο|~κ%ῐ~οῑντο";
    imp_mid_2: "~κ%ῐ~εσο|~κ%ῐ~ου", _, "~κ%ῐ~εσθε";
    imp_mid_3: "~κ%ῐ~εσθω", _, "~κ%ῐ~εσθων";
    imp_pass_2: _, _, "~κ%ῐ~εσθε";
    imp_pass_3: _, "~κ%ῐ~εσθων", "~κ%ῐ~εσθων";
    imperfect_mid_1: "~ξε%ῐ~εμην", _, "~ξε%ῐ~εμεθᾰ";
    imperfect_mid_2: "~ξε%ῐ~εσο", "~ξε%ῐ~εσθον", _;
    imperfect_mid_3: "~ξε%ῐ~ετο", "~ξε%ῐ~εσθην", "~ξε%ῐ~εντο";
    imperfect_pass_2: _, _, "~ξε%ῐ~εσθε";
    future_ind_mid_1: "~κ~ησομαι", _, "~κ~ησομεθᾰ";
    future_ind_mid_2: "~κ~ησῃ|~κ~ησει", "~κ~ησεσθον", _;
    future_ind_mid_3: "~κ~ησεται", _, _;
    future_opt_mid_1: "~κ~ησοιμην", _, "~κ~ησοιμεθᾰ";
    future_opt_mid_2: "~κ~ησοιο", _, _;
    future_opt_mid_3: _, "~κ~ησοισθην", _;
    aorist_ind_mid_1: "~ξε~εμην", _, _;
    aorist_ind_mid_2: "~ξε~ου", "~ξε~εσθον", _;
    aorist_subj_mid_1: "~κ~ωμαι", _, _;
    aorist_subj_mid_2: "~κ~ῃ", "~κ~ησθον", _;
    aorist_opt_mid_1: "~κ~ειμην", _, _;
    aorist_opt_mid_2: "~κ~εῑο", _, _;
    aorist_opt_mid_3: "~κ~εῑτο|~κ~οῑτο", _, _;
    aorist_imp_mid_2: "~κ~ου", "~κ~εσθον", _;
    aorist_imp_mid_3: _, _, "~κ~εσθων";
    perfect_ind_mid_1: "~κ%ε~ειμαι", _, "~κ%ε~ειμεθᾰ";
    perfect_ind_mid_2: "~κ%ε~εισαι", "~κ%ε~εισθον", _;
    perfect_ind_mid_3: "~κ%ε~ειται", _, "~κ%ε~εινται";
    perfect_ind_pass_2: _, _, "~κ%ε~εισθε";
    perfect_imp_mid_2: _, _, "~κ%ε~εισθε";
    perfect_imp_mid_3: "~κ%ε~εισθω", _, "~κ%ε~εισθων";
    perfect_imp_pass_2: "~κ%ε~εισο", _, "~κ%ε~εισθε";
    perfect_imp_pass_3: _, "~κ%ε~εισθων", "~κ%ε~εισθων";
    infinitive_pres: _, _, "~κ%ῐ~εσθαι";
    infinitive_perf: _, "~κ%ε~εῑσθαι", _;
    participle_pres: _, _, "~κ%ῐ~εμενος";
    participle_aor: _, "~κ~εμενος", _;
    participle_perf: _, _, "~κ%ε~ειμενος";
    ),
    // Μᾰστῐγοω: 1 words (μᾰστῐγοω). Like Ἀποστατεω, except in 67 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 du, ….
    conjugation!("μᾰστῐγοω", 1, "~ᾰ~ῐ~οω", like Ἀποστατεω;
    ind_act_1: "~α~ι~ω", _, "~α~ι~ομεν";
    ind_act_2: "~α~ι~εις", derived, _;
    ind_act_3: "~α~ι~ει", "~α~ι~ετον", "~α~ι~ουσῐ|~α~ι~ουσῐν";
    ind_mid_1: "~α~ι~ομαι", _, "~α~ι~ομεθᾰ";
    ind_mid_2: "~α~ι~ῃ|~α~ι~ει", "~α~ι~εσθον", _;
    ind_pass_2: _, _, "~α~ι~εσθε";
    ind_pass_3: "~α~ι~εται", _, "~α~ι~ονται";
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
    opt_mid_2: "~α~ι~οιο", "~α~ι~οισθον", _;
    opt_mid_3: "~α~ι~οιτο", "~α~ι~οισθην", "~α~ι~οιντο";
    opt_pass_2: _, _, "~α~ι~οισθε";
    imp_act_2: "~α~ι~ε", "~α~ι~ετον", "~α~ι~ετε";
    imp_act_3: derived, _, derived;
    imp_mid_2: "~α~ι~ου", _, "~α~ι~εσθε";
    imp_mid_3: "~α~ι~εσθω", _, "~α~ι~εσθων";
    imp_pass_2: _, _, "~α~ι~εσθε";
    imp_pass_3: _, "~α~ι~εσθων", "~α~ι~εσθων";
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    infinitive_pres: "~α~ι~ειν", _, "~α~ι~εσθαι";
    participle_pres: "~α~ι~ων", _, "~α~ι~ομενος";
    ),
    // Βᾰρβᾰροομαι: 1 words (βᾰρβᾰροομαι). Like Ἀποστατεω, except in 43 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_3 sg, ind_act_3 pl, ….
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
    imp_act_3: derived, _, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    perfect_ind_mid_1: "~εβα~~ρωμαι", _, "~εβα~~ρωμεθᾰ";
    perfect_ind_mid_2: "~εβα~~ρωσαι", "~εβα~~ρωσθον", _;
    perfect_ind_mid_3: "~εβα~~ρωται", _, "~εβα~~ρωνται";
    perfect_imp_mid_2: _, _, "~εβα~~ρωσθε";
    perfect_imp_mid_3: "~εβα~~ρωσθω", _, "~εβα~~ρωσθων";
    infinitive_pres: derived, _, _;
    participle_pres: derived, _, _;
    ),
    // Ἐπιμιμνησκομαι: 1 words (ἐπιμιμνησκομαι). Like Φειδομαι, except in 75 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_pass_2 pl, ….
    conjugation!("ἐπιμιμνησκομαι", 1, "~ιμι~σκομαι", like Φειδομαι;
    stem future_pass: "~ῐ~σθησ- like ποιεω";
    stem aorist_ind: "~ε~σ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ε~σθην- like ἐλπω";
    stem aorist_pass: "~ῐ~σθ- like ποιεω";
    ind_act_1: "~ιμι~σκομαι", _, _;
    ind_mid_1: "~ῐ%ῐ~σκομαι", _, "~ῐ%ῐ~σκομεθᾰ";
    ind_mid_2: "~ῐ%ῐ~σκῃ|~ῐ%ῐ~σκει", "~ῐ%ῐ~σκεσθον", _;
    ind_pass_2: _, _, "~ῐ%ῐ~σκεσθε";
    ind_pass_3: "~ῐ%ῐ~σκεται", _, "~ῐ%ῐ~σκονται";
    subj_mid_1: "~ῐ%ῐ~σκωμαι", _, "~ῐ%ῐ~σκωμεθᾰ";
    subj_mid_2: "~ῐ%ῐ~σκῃ", "~ῐ%ῐ~σκησθον", _;
    subj_mid_3: "~ῐ%ῐ~σκηται", _, "~ῐ%ῐ~σκωνται";
    subj_pass_2: "~ῐ%ῐ~σκῃ", _, "~ῐ%ῐ~σκησθε";
    opt_mid_1: "~ῐ%ῐ~σκοιμην", _, "~ῐ%ῐ~σκοιμεθᾰ";
    opt_mid_2: "~ῐ%ῐ~σκοιο", "~ῐ%ῐ~σκοισθον", _;
    opt_mid_3: "~ῐ%ῐ~σκοιτο", "~ῐ%ῐ~σκοισθην", "~ῐ%ῐ~σκοιντο";
    opt_pass_2: _, _, "~ῐ%ῐ~σκοισθε";
    imp_mid_2: "~ῐ%ῐ~σκου", _, "~ῐ%ῐ~σκεσθε";
    imp_mid_3: "~ῐ%ῐ~σκεσθω", _, "~ῐ%ῐ~σκεσθων";
    imp_pass_2: _, _, "~ῐ%ῐ~σκεσθε";
    imp_pass_3: _, "~ῐ%ῐ~σκεσθων", "~ῐ%ῐ~σκεσθων";
    imperfect_mid_1: "~ε%ῐ~σκομην", _, "~ε%ῐ~σκομεθᾰ";
    imperfect_mid_2: "~ε%ῐ~σκου", "~ε%ῐ~σκεσθον", _;
    imperfect_mid_3: "~ε%ῐ~σκετο", "~ε%ῐ~σκεσθην", "~ε%ῐ~σκοντο";
    imperfect_pass_2: _, _, "~ε%ῐ~σκεσθε";
    future_ind_mid_1: "~ῐ~σομαι", _, "~ῐ~σομεθᾰ";
    future_ind_mid_2: "~ῐ~σῃ|~ῐ~σει", "~ῐ~σεσθον", _;
    future_ind_mid_3: "~ῐ~σεται", _, _;
    future_opt_mid_1: "~ῐ~σοιμην", _, "~ῐ~σοιμεθᾰ";
    future_opt_mid_2: "~ῐ~σοιο", _, _;
    future_opt_mid_3: _, "~ῐ~σοισθην", _;
    aorist_subj_mid_1: "~ῐ~σωμαι", _, _;
    aorist_subj_mid_2: "~ῐ~σῃ", "~ῐ~σησθον", _;
    aorist_opt_mid_1: "~ῐ~σαιμην", _, _;
    aorist_opt_mid_2: "~ῐ~σαιο", _, _;
    aorist_imp_mid_2: "~ῐ~σαι", "~ῐ~σᾰσθον", _;
    aorist_imp_mid_3: _, _, "~ῐ~σᾰσθων";
    perfect_ind_mid_1: "~ῐ%ε~μαι", _, "~ῐ%ε~μεθᾰ";
    perfect_ind_mid_2: "~ῐ%ε~σαι", "~ῐ%ε~σθον", _;
    perfect_ind_mid_3: "~ῐ%ε~ται", _, "~ῐ%ε~νται";
    perfect_ind_pass_2: _, _, "~ῐ%ε~σθε";
    perfect_imp_mid_2: _, _, "~ῐ%ε~σθε";
    perfect_imp_mid_3: "~ῐ%ε~σθω", _, "~ῐ%ε~σθων";
    perfect_imp_pass_2: "~ῐ%ε~σο", _, "~ῐ%ε~σθε";
    perfect_imp_pass_3: _, "~ῐ%ε~σθων", "~ῐ%ε~σθων";
    infinitive_pres: _, _, "~ῐ%ῐ~σκεσθαι";
    infinitive_perf: _, "~ῐ%ε~σθαι", _;
    participle_pres: _, _, "~ῐ%ῐ~σκομενος";
    participle_aor: _, "~ῐ~σᾰμενος", _;
    participle_perf: _, _, "~ῐ%ε~μενος";
    ),
    // Εργεω: 1 words (εργεω). Like Ἐλαυνω, except in 55 cells: ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("εργεω", 1, "ε~εω", like Ἐλαυνω;
    ind_act_1: _, _, "ἐ~οῡμεν";
    ind_act_2: "ἐ~εῑς", _, _;
    ind_act_3: "ἐ~εῑ", "ἐ~εῑτον", "ἐ~οῡσῐ|ἐ~οῡσῐν";
    ind_mid_1: "ἐ~οῡμαι", _, "ἐ~ουμεθᾰ";
    ind_mid_2: "ἐ~εῑ|ἐ~ῃ", "ἐ~εῑσθον", _;
    ind_pass_2: _, _, "ἐ~εῑσθε";
    ind_pass_3: "ἐ~εῑται", _, "ἐ~οῡνται";
    subj_act_3: _, _, "ἐ~ωσῐ|ἐ~ωσῐν";
    opt_act_1: "ἐ~οιην|ἐ~οῑμῐ", _, "ἐ~οῑμεν|ἐ~οιημεν";
    opt_act_2: "ἐ~οιης|ἐ~οῑς", "ἐ~οῑτον|ἐ~οιητον", "ἐ~οῑτε|ἐ~οιητε";
    opt_act_3: "ἐ~οιη|ἐ~οῑ", "ἐ~οιτην|ἐ~οιητην", "ἐ~οῑεν|ἐ~οιησᾰν";
    opt_mid_2: "ἐ~οῑο", "ἐ~οῑσθον", _;
    opt_mid_3: "ἐ~οῑτο", _, "ἐ~οῑντο";
    opt_pass_2: _, _, "ἐ~οῑσθε";
    imp_act_2: "ἐ~ει", "ἐ~εῑτον", "ἐ~εῑτε";
    imp_mid_2: "ἐ~οῡ", _, "ἐ~εῑσθε";
    imp_mid_3: "ἐ~εισθω", _, "ἐ~εισθων";
    imp_pass_2: _, _, "ἐ~εῑσθε";
    imp_pass_3: _, "ἐ~εισθων", "ἐ~εισθων";
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, _, _;
    imperfect_mid_3: derived, _, _;
    imperfect_pass_2: _, derived, derived;
    imperfect_pass_3: _, derived, derived;
    infinitive_pres: "ἐ~εῑν", _, "ἐ~εῑσθαι";
    participle_pres: _, _, "ἐ~ουμενος";
    ),
    // Ᾰνθρωπεω: 1 words (ᾰνθρωπεω). Like Ἐρωτᾰω, except in 133 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("ᾰνθρωπεω", 1, "ᾰ~εω", like Ἐρωτᾰω;
    stem future_pass: "~ηθησ- like ᾰντᾰποκρῑνομαι";
    stem aorist_ind: "~ησ- like ἐλπῐζω";
    stem aorist_pass: "~ηθ- like ᾰντᾰποκρῑνομαι";
    ind_act_1: "ἀ~ω", _, "ἀ~οῡμεν";
    ind_act_2: "ἀ~εῑς", _, _;
    ind_act_3: "ἀ~εῑ", "ἀ~εῑτον", "ἀ~οῡσῐ|ἀ~οῡσῐν";
    ind_mid_1: "ἀ~οῡμαι", _, "ἀ~ουμεθᾰ";
    ind_mid_2: "ἀ~εῑ|ἀ~ῃ", "ἀ~εῑσθον", _;
    ind_pass_2: _, _, "ἀ~εῑσθε";
    ind_pass_3: "ἀ~εῑται", _, "ἀ~οῡνται";
    subj_act_1: "ἀ~ω", _, "ἀ~ωμεν";
    subj_act_2: "ἀ~ῃς", _, _;
    subj_act_3: "ἀ~ῃ", "ἀ~ητον", "ἀ~ωσῐ|ἀ~ωσῐν";
    subj_mid_1: _, _, "ἀ~ωμεθᾰ";
    subj_mid_2: _, "ἀ~ησθον", _;
    subj_mid_3: "ἀ~ηται", _, "ἀ~ωνται";
    subj_pass_1: "ἀ~ωμαι", _, _;
    subj_pass_2: _, _, "ἀ~ησθε";
    opt_act_1: "ἀ~οιην|ἀ~οῑμῐ", _, "ἀ~οῑμεν|ἀ~οιημεν";
    opt_act_2: "ἀ~οιης|ἀ~οῑς", "ἀ~οῑτον|ἀ~οιητον", "ἀ~οῑτε|ἀ~οιητε";
    opt_act_3: "ἀ~οιη|ἀ~οῑ", "ἀ~οιτην|ἀ~οιητην", "ἀ~οῑεν|ἀ~οιησᾰν";
    opt_mid_1: "ἀ~οιμην", _, "ἀ~οιμεθᾰ";
    opt_mid_2: "ἀ~οῑο", "ἀ~οῑσθον", _;
    opt_mid_3: "ἀ~οῑτο", "ἀ~οισθην", "ἀ~οῑντο";
    opt_pass_2: _, _, "ἀ~οῑσθε";
    imp_act_2: "ἀ~ει", "ἀ~εῑτον", "ἀ~εῑτε";
    imp_mid_2: "ἀ~οῡ", _, "ἀ~εῑσθε";
    imp_mid_3: "ἀ~εισθω", _, "ἀ~εισθων";
    imp_pass_2: _, _, "ἀ~εῑσθε";
    imp_pass_3: _, "ἀ~εισθων", "ἀ~εισθων";
    imperfect_act_1: "ἠ~ουν", _, "ἠ~οῡμεν";
    imperfect_act_2: "ἠ~εις", "ἠ~εῑτον", "ἠ~εῑτε";
    imperfect_act_3: "ἠ~ει", "ἠ~ειτην", "ἠ~ουν";
    imperfect_mid_1: "ἠ~ουμην", _, "ἠ~ουμεθᾰ";
    imperfect_mid_2: "ἠ~οῡ", _, _;
    imperfect_mid_3: "ἠ~εῑτο", _, _;
    imperfect_pass_2: _, "ἠ~εῑσθον", "ἠ~εῑσθε";
    imperfect_pass_3: _, "ἠ~εισθην", "ἠ~οῡντο";
    future_ind_act_1: "ἀ~ησω", _, "ἀ~ησομεν";
    future_ind_act_2: "ἀ~ησεις", "ἀ~ησετον", "ἀ~ησετε";
    future_ind_act_3: _, _, "ἀ~ησουσῐ|ἀ~ησουσῐν";
    future_ind_mid_1: "ἀ~ησομαι", _, "ἀ~ησομεθᾰ";
    future_ind_mid_2: "ἀ~ησῃ|ἀ~ησει", "ἀ~ησεσθον", _;
    future_ind_mid_3: "ἀ~ησεται", _, _;
    future_opt_act_1: "ἀ~ησοιμῐ", _, "ἀ~ησοιμεν";
    future_opt_act_2: _, "ἀ~ησοιτον", "ἀ~ησοιτε";
    future_opt_act_3: "ἀ~ησοι", "ἀ~ησοιτην", "ἀ~ησοιεν";
    future_opt_mid_1: "ἀ~ησοιμην", _, "ἀ~ησοιμεθᾰ";
    future_opt_mid_2: "ἀ~ησοιο", _, _;
    future_opt_mid_3: _, "ἀ~ησοισθην", _;
    aorist_subj_act_1: "ἀ~ησω", _, _;
    aorist_subj_act_2: "ἀ~ησῃς", "ἀ~ησητον", _;
    aorist_subj_act_3: _, _, "ἀ~ησωσῐ|ἀ~ησωσῐν";
    aorist_subj_mid_1: "ἀ~ησωμαι", _, _;
    aorist_subj_mid_2: "ἀ~ησῃ", "ἀ~ησησθον", _;
    aorist_opt_act_1: "ἀ~ησαιμῐ", _, "ἀ~ησαιμεν";
    aorist_opt_act_2: "ἀ~ησειᾰς|ἀ~ησαις", _, _;
    aorist_opt_act_3: "ἀ~ησειε|ἀ~ησειεν|ἀ~ησαι", "ἀ~ησαιτην", "ἀ~ησειᾰν|ἀ~ησαιεν";
    aorist_opt_mid_1: "ἀ~ησαιμην", _, _;
    aorist_opt_mid_2: "ἀ~ησαιο", _, _;
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
    infinitive_pres: "ἀ~εῑν", _, "ἀ~εῑσθαι";
    infinitive_fut: "ἀ~ησειν", _, _;
    infinitive_aor: "ἀ~ησαι", _, _;
    participle_pres: "ἀ~ων", _, "ἀ~ουμενος";
    participle_aor: "ἀ~ησᾱς", "ἀ~ησᾰμενος", _;
    ),
    // Επῐβουλευω: 1 words (επῐβουλευω). Like Πεμπω, except in 162 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("επῐβουλευω", 1, "ε~ῐ~υω", like Πεμπω;
    stem future_pass: "~ῐ~υθησ- like επῐβουλευω";
    stem aorist_ind: "~ε~υσ- like επῐβουλευω";
    stem aorist_pass: "~ῐ~υθ- like επῐβουλευω";
    stem pluperfect_act: "~ε%ε~υκ- like επῐβουλευω";
    ind_act_1: "ἑ~ῐ~υω", _, "ἑ~ῐ~υομεν";
    ind_act_2: "ἑ~ῐ~υεις", _, _;
    ind_act_3: "ἑ~ῐ~υει", "ἑ~ῐ~υετον", "ἑ~ῐ~υουσῐ|ἑ~ῐ~υουσῐν";
    ind_mid_1: "ἑ~ῐ~υομαι", _, "ἑ~ῐ~υομεθᾰ";
    ind_mid_2: "ἑ~ῐ~υῃ|ἑ~ῐ~υει", "ἑ~ῐ~υεσθον", _;
    ind_pass_2: _, _, "ἑ~ῐ~υεσθε";
    ind_pass_3: "ἑ~ῐ~υεται", _, "ἑ~ῐ~υονται";
    subj_act_1: "ἑ~ῐ~υω", _, "ἑ~ῐ~υωμεν";
    subj_act_2: "ἑ~ῐ~υῃς", _, _;
    subj_act_3: "ἑ~ῐ~υῃ", "ἑ~ῐ~υητον", "ἑ~ῐ~υωσῐ|ἑ~ῐ~υωσῐν";
    subj_mid_1: _, _, "ἑ~ῐ~υωμεθᾰ";
    subj_mid_2: _, "ἑ~ῐ~υησθον", _;
    subj_mid_3: "ἑ~ῐ~υηται", _, "ἑ~ῐ~υωνται";
    subj_pass_1: "ἑ~ῐ~υωμαι", _, _;
    subj_pass_2: _, _, "ἑ~ῐ~υησθε";
    opt_act_1: "ἑ~ῐ~υοιμῐ", _, "ἑ~ῐ~υοιμεν";
    opt_act_2: "ἑ~ῐ~υοις", "ἑ~ῐ~υοιτον", _;
    opt_act_3: "ἑ~ῐ~υοι", "ἑ~ῐ~υοιτην", "ἑ~ῐ~υοιεν";
    opt_mid_1: "ἑ~ῐ~υοιμην", _, "ἑ~ῐ~υοιμεθᾰ";
    opt_mid_2: "ἑ~ῐ~υοιο", "ἑ~ῐ~υοισθον", _;
    opt_mid_3: "ἑ~ῐ~υοιτο", "ἑ~ῐ~υοισθην", "ἑ~ῐ~υοιντο";
    opt_pass_2: _, _, "ἑ~ῐ~υοισθε";
    imp_act_2: "ἑ~ῐ~υε", "ἑ~ῐ~υετον", "ἑ~ῐ~υετε";
    imp_mid_2: "ἑ~ῐ~υου", _, "ἑ~ῐ~υεσθε";
    imp_mid_3: "ἑ~ῐ~υεσθω", _, "ἑ~ῐ~υεσθων";
    imp_pass_2: _, _, "ἑ~ῐ~υεσθε";
    imp_pass_3: _, "ἑ~ῐ~υεσθων", "ἑ~ῐ~υεσθων";
    imperfect_act_1: "ἑ~ε~υον", _, "ἑ~ε~υομεν";
    imperfect_act_2: "ἑ~ε~υες", "ἑ~ε~υετον", "ἑ~ε~υετε";
    imperfect_act_3: "ἑ~ε~υε|ἑ~ε~υεν", "ἑ~ε~υετην", "ἑ~ε~υον";
    imperfect_mid_1: "ἑ~ε~υομην", _, "ἑ~ε~υομεθᾰ";
    imperfect_mid_2: "ἑ~ε~υου", _, _;
    imperfect_mid_3: "ἑ~ε~υετο", _, _;
    imperfect_pass_2: _, "ἑ~ε~υεσθον", "ἑ~ε~υεσθε";
    imperfect_pass_3: _, "ἑ~ε~υεσθην", "ἑ~ε~υοντο";
    future_ind_act_1: "ἑ~ῐ~υσω", _, "ἑ~ῐ~υσομεν";
    future_ind_act_2: "ἑ~ῐ~υσεις", "ἑ~ῐ~υσετον", "ἑ~ῐ~υσετε";
    future_ind_act_3: _, _, "ἑ~ῐ~υσουσῐ|ἑ~ῐ~υσουσῐν";
    future_ind_mid_1: "ἑ~ῐ~υσομαι", _, "ἑ~ῐ~υσομεθᾰ";
    future_ind_mid_2: "ἑ~ῐ~υσῃ|ἑ~ῐ~υσει", "ἑ~ῐ~υσεσθον", _;
    future_ind_mid_3: "ἑ~ῐ~υσεται", _, _;
    future_opt_act_1: "ἑ~ῐ~υσοιμῐ", _, "ἑ~ῐ~υσοιμεν";
    future_opt_act_2: _, "ἑ~ῐ~υσοιτον", "ἑ~ῐ~υσοιτε";
    future_opt_act_3: "ἑ~ῐ~υσοι", "ἑ~ῐ~υσοιτην", "ἑ~ῐ~υσοιεν";
    future_opt_mid_1: "ἑ~ῐ~υσοιμην", _, "ἑ~ῐ~υσοιμεθᾰ";
    future_opt_mid_2: "ἑ~ῐ~υσοιο", _, _;
    future_opt_mid_3: _, "ἑ~ῐ~υσοισθην", _;
    aorist_ind_pass_1: "ἑ~ε~υθην", _, _;
    aorist_subj_act_1: "ἑ~ῐ~υσω", _, _;
    aorist_subj_act_2: "ἑ~ῐ~υσῃς", "ἑ~ῐ~υσητον", _;
    aorist_subj_act_3: _, _, "ἑ~ῐ~υσωσῐ|ἑ~ῐ~υσωσῐν";
    aorist_subj_mid_1: "ἑ~ῐ~υσωμαι", _, _;
    aorist_subj_mid_2: "ἑ~ῐ~υσῃ", "ἑ~ῐ~υσησθον", _;
    aorist_opt_act_1: "ἑ~ῐ~υσαιμῐ", _, "ἑ~ῐ~υσαιμεν";
    aorist_opt_act_2: "ἑ~ῐ~υσειᾰς|ἑ~ῐ~υσαις", _, _;
    aorist_opt_act_3: "ἑ~ῐ~υσειε|ἑ~ῐ~υσειεν|ἑ~ῐ~υσαι", "ἑ~ῐ~υσαιτην", "ἑ~ῐ~υσειᾰν|ἑ~ῐ~υσαιεν";
    aorist_opt_mid_1: "ἑ~ῐ~υσαιμην", _, _;
    aorist_opt_mid_2: "ἑ~ῐ~υσαιο", _, _;
    aorist_imp_act_2: "ἑ~ῐ~υσον", _, "ἑ~ῐ~υσᾰτε";
    aorist_imp_act_3: _, _, "ἑ~ῐ~υσᾰντων";
    aorist_imp_mid_2: "ἑ~ῐ~υσαι", "ἑ~ῐ~υσᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἑ~ῐ~υσᾰσθων";
    perfect_ind_act_1: "ἑ~ῐ%ε~υκᾰ", _, "ἑ~ῐ%ε~υκᾰμεν";
    perfect_ind_act_2: _, "ἑ~ῐ%ε~υκᾰτον", _;
    perfect_ind_act_3: "ἑ~ῐ%ε~υκε|ἑ~ῐ%ε~υκεν", _, "ἑ~ῐ%ε~υκᾱσῐ|ἑ~ῐ%ε~υκᾱσῐν";
    perfect_ind_mid_1: "ἑ~ῐ%ε~υμαι", _, "ἑ~ῐ%ε~υμεθᾰ";
    perfect_ind_mid_2: "ἑ~ῐ%ε~υσαι", "ἑ~ῐ%ε~υσθον", _;
    perfect_ind_mid_3: "ἑ~ῐ%ε~υται", _, "ἑ~ῐ%ε~υνται";
    perfect_ind_pass_2: _, _, "ἑ~ῐ%ε~υσθε";
    perfect_subj_act_1: "ἑ~ῐ%ε~υκω", _, _;
    perfect_subj_act_2: "ἑ~ῐ%ε~υκῃς", "ἑ~ῐ%ε~υκητον", "ἑ~ῐ%ε~υκητε";
    perfect_subj_act_3: "ἑ~ῐ%ε~υκῃ", _, "ἑ~ῐ%ε~υκωσῐ|ἑ~ῐ%ε~υκωσῐν";
    perfect_opt_act_1: "ἑ~ῐ%ε~υκοιμῐ|ἑ~ῐ%ε~υκοιην", _, "ἑ~ῐ%ε~υκοιμεν";
    perfect_opt_act_2: "ἑ~ῐ%ε~υκοις|ἑ~ῐ%ε~υκοιης", _, _;
    perfect_opt_act_3: "ἑ~ῐ%ε~υκοι|ἑ~ῐ%ε~υκοιη", "ἑ~ῐ%ε~υκοιτην", _;
    perfect_imp_act_2: "ἑ~ῐ%ε~υκε", _, "ἑ~ῐ%ε~υκετε";
    perfect_imp_act_3: _, _, "ἑ~ῐ%ε~υκοντων";
    perfect_imp_mid_2: _, _, "ἑ~ῐ%ε~υσθε";
    perfect_imp_mid_3: "ἑ~ῐ%ε~υσθω", _, "ἑ~ῐ%ε~υσθων";
    perfect_imp_pass_2: "ἑ~ῐ%ε~υσο", _, "ἑ~ῐ%ε~υσθε";
    perfect_imp_pass_3: _, "ἑ~ῐ%ε~υσθων", "ἑ~ῐ%ε~υσθων";
    pluperfect_mid_1: "ἑ~ε%ε~υμην", _, _;
    pluperfect_mid_2: "ἑ~ε%ε~υσο", "ἑ~ε%ε~υσθον", _;
    pluperfect_mid_3: "ἑ~ε%ε~υτο", _, "ἑ~ε%ε~υντο";
    pluperfect_pass_1: _, _, "ἑ~ε%ε~υμεθᾰ";
    pluperfect_pass_2: _, _, "ἑ~ε%ε~υσθε";
    pluperfect_pass_3: _, "ἑ~ε%ε~υσθην", _;
    infinitive_pres: "ἑ~ῐ~υειν", _, "ἑ~ῐ~υεσθαι";
    infinitive_fut: "ἑ~ῐ~υσειν", _, _;
    infinitive_aor: "ἑ~ῐ~ῡσαι", _, _;
    infinitive_perf: _, "ἑ~ῐ%ε~ῡσθαι", _;
    participle_pres: "ἑ~ῐ~υων", _, "ἑ~ῐ~υομενος";
    participle_aor: "ἑ~ῐ~υσᾱς", "ἑ~ῐ~υσᾰμενος", _;
    participle_perf: "ἑ~ῐ%ε~υκως", _, "ἑ~ῐ%ε~υμενος";
    ),
    // Εισκᾰτοικεω: 1 words (εισκᾰτοικεω). Like Πεμπω, except in 163 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("εισκᾰτοικεω", 1, "ε~οι~εω", like Πεμπω;
    stem future_pass: "~οι~ηθησ- like επῐβουλευω";
    stem aorist_ind: "~ῳ~ησ- like επῐβουλευω";
    stem aorist_pass: "~οι~ηθ- like επῐβουλευω";
    stem pluperfect_act: "~ῳ~ηκ- like επῐβουλευω";
    ind_act_1: "ἑ~οι~ω", _, "ἑ~οι~οῡμεν";
    ind_act_2: "ἑ~οι~εῑς", _, _;
    ind_act_3: "ἑ~οι~εῑ", "ἑ~οι~εῑτον", "ἑ~οι~οῡσῐ|ἑ~οι~οῡσῐν";
    ind_mid_1: "ἑ~οι~οῡμαι", _, "ἑ~οι~ουμεθᾰ";
    ind_mid_2: "ἑ~οι~εῑ|ἑ~οι~ῃ", "ἑ~οι~εῑσθον", _;
    ind_pass_2: _, _, "ἑ~οι~εῑσθε";
    ind_pass_3: "ἑ~οι~εῑται", _, "ἑ~οι~οῡνται";
    subj_act_1: "ἑ~οι~ω", _, "ἑ~οι~ωμεν";
    subj_act_2: "ἑ~οι~ῃς", _, _;
    subj_act_3: "ἑ~οι~ῃ", "ἑ~οι~ητον", "ἑ~οι~ωσῐ|ἑ~οι~ωσῐν";
    subj_mid_1: _, _, "ἑ~οι~ωμεθᾰ";
    subj_mid_2: _, "ἑ~οι~ησθον", _;
    subj_mid_3: "ἑ~οι~ηται", _, "ἑ~οι~ωνται";
    subj_pass_1: "ἑ~οι~ωμαι", _, _;
    subj_pass_2: _, _, "ἑ~οι~ησθε";
    opt_act_1: "ἑ~οι~οιην|ἑ~οι~οῑμῐ", _, "ἑ~οι~οῑμεν|ἑ~οι~οιημεν";
    opt_act_2: "ἑ~οι~οιης|ἑ~οι~οῑς", "ἑ~οι~οῑτον|ἑ~οι~οιητον", "ἑ~οι~οῑτε|ἑ~οι~οιητε";
    opt_act_3: "ἑ~οι~οιη|ἑ~οι~οῑ", "ἑ~οι~οιτην|ἑ~οι~οιητην", "ἑ~οι~οῑεν|ἑ~οι~οιησᾰν";
    opt_mid_1: "ἑ~οι~οιμην", _, "ἑ~οι~οιμεθᾰ";
    opt_mid_2: "ἑ~οι~οῑο", "ἑ~οι~οῑσθον", _;
    opt_mid_3: "ἑ~οι~οῑτο", "ἑ~οι~οισθην", "ἑ~οι~οῑντο";
    opt_pass_2: _, _, "ἑ~οι~οῑσθε";
    imp_act_2: "ἑ~οι~ει", "ἑ~οι~εῑτον", "ἑ~οι~εῑτε";
    imp_mid_2: "ἑ~οι~οῡ", _, "ἑ~οι~εῑσθε";
    imp_mid_3: "ἑ~οι~εισθω", _, "ἑ~οι~εισθων";
    imp_pass_2: _, _, "ἑ~οι~εῑσθε";
    imp_pass_3: _, "ἑ~οι~εισθων", "ἑ~οι~εισθων";
    imperfect_act_1: "ἑ~ῳ~ουν", _, "ἑ~ῳ~οῡμεν";
    imperfect_act_2: "ἑ~ῳ~εις", "ἑ~ῳ~εῑτον", "ἑ~ῳ~εῑτε";
    imperfect_act_3: "ἑ~ῳ~ει", "ἑ~ῳ~ειτην", "ἑ~ῳ~ουν";
    imperfect_mid_1: "ἑ~ῳ~ουμην", _, "ἑ~ῳ~ουμεθᾰ";
    imperfect_mid_2: "ἑ~ῳ~οῡ", _, _;
    imperfect_mid_3: "ἑ~ῳ~εῑτο", _, _;
    imperfect_pass_2: _, "ἑ~ῳ~εῑσθον", "ἑ~ῳ~εῑσθε";
    imperfect_pass_3: _, "ἑ~ῳ~εισθην", "ἑ~ῳ~οῡντο";
    future_ind_act_1: "ἑ~οι~ησω", _, "ἑ~οι~ησομεν";
    future_ind_act_2: "ἑ~οι~ησεις", "ἑ~οι~ησετον", "ἑ~οι~ησετε";
    future_ind_act_3: _, _, "ἑ~οι~ησουσῐ|ἑ~οι~ησουσῐν";
    future_ind_mid_1: "ἑ~οι~ησομαι", _, "ἑ~οι~ησομεθᾰ";
    future_ind_mid_2: "ἑ~οι~ησῃ|ἑ~οι~ησει", "ἑ~οι~ησεσθον", _;
    future_ind_mid_3: "ἑ~οι~ησεται", _, _;
    future_opt_act_1: "ἑ~οι~ησοιμῐ", _, "ἑ~οι~ησοιμεν";
    future_opt_act_2: _, "ἑ~οι~ησοιτον", "ἑ~οι~ησοιτε";
    future_opt_act_3: "ἑ~οι~ησοι", "ἑ~οι~ησοιτην", "ἑ~οι~ησοιεν";
    future_opt_mid_1: "ἑ~οι~ησοιμην", _, "ἑ~οι~ησοιμεθᾰ";
    future_opt_mid_2: "ἑ~οι~ησοιο", _, _;
    future_opt_mid_3: _, "ἑ~οι~ησοισθην", _;
    aorist_ind_pass_1: "ἑ~ῳ~ηθην", _, _;
    aorist_subj_act_1: "ἑ~οι~ησω", _, _;
    aorist_subj_act_2: "ἑ~οι~ησῃς", "ἑ~οι~ησητον", _;
    aorist_subj_act_3: _, _, "ἑ~οι~ησωσῐ|ἑ~οι~ησωσῐν";
    aorist_subj_mid_1: "ἑ~οι~ησωμαι", _, _;
    aorist_subj_mid_2: "ἑ~οι~ησῃ", "ἑ~οι~ησησθον", _;
    aorist_opt_act_1: "ἑ~οι~ησαιμῐ", _, "ἑ~οι~ησαιμεν";
    aorist_opt_act_2: "ἑ~οι~ησειᾰς|ἑ~οι~ησαις", _, _;
    aorist_opt_act_3: "ἑ~οι~ησειε|ἑ~οι~ησειεν|ἑ~οι~ησαι", "ἑ~οι~ησαιτην", "ἑ~οι~ησειᾰν|ἑ~οι~ησαιεν";
    aorist_opt_mid_1: "ἑ~οι~ησαιμην", _, _;
    aorist_opt_mid_2: "ἑ~οι~ησαιο", _, _;
    aorist_imp_act_2: "ἑ~οι~ησον", _, "ἑ~οι~ησᾰτε";
    aorist_imp_act_3: _, _, "ἑ~οι~ησᾰντων";
    aorist_imp_mid_2: "ἑ~οι~ησαι", "ἑ~οι~ησᾰσθον", _;
    aorist_imp_mid_3: _, _, "ἑ~οι~ησᾰσθων";
    perfect_ind_act_1: "ἑ~ῳ~ηκᾰ", _, "ἑ~ῳ~ηκᾰμεν";
    perfect_ind_act_2: _, "ἑ~ῳ~ηκᾰτον", _;
    perfect_ind_act_3: "ἑ~ῳ~ηκε|ἑ~ῳ~ηκεν", _, "ἑ~ῳ~ηκᾱσῐ|ἑ~ῳ~ηκᾱσῐν";
    perfect_ind_mid_1: "ἑ~ῳ~ημαι", _, "ἑ~ῳ~ημεθᾰ";
    perfect_ind_mid_2: "ἑ~ῳ~ησαι", "ἑ~ῳ~ησθον", _;
    perfect_ind_mid_3: "ἑ~ῳ~ηται", _, "ἑ~ῳ~ηνται";
    perfect_ind_pass_2: _, _, "ἑ~ῳ~ησθε";
    perfect_subj_act_1: "ἑ~ῳ~ηκω", _, _;
    perfect_subj_act_2: "ἑ~ῳ~ηκῃς", "ἑ~ῳ~ηκητον", "ἑ~ῳ~ηκητε";
    perfect_subj_act_3: "ἑ~ῳ~ηκῃ", _, "ἑ~ῳ~ηκωσῐ|ἑ~ῳ~ηκωσῐν";
    perfect_opt_act_1: "ἑ~ῳ~ηκοιμῐ|ἑ~ῳ~ηκοιην", _, "ἑ~ῳ~ηκοιμεν";
    perfect_opt_act_2: "ἑ~ῳ~ηκοις|ἑ~ῳ~ηκοιης", _, _;
    perfect_opt_act_3: "ἑ~ῳ~ηκοι|ἑ~ῳ~ηκοιη", "ἑ~ῳ~ηκοιτην", _;
    perfect_imp_act_2: "ἑ~ῳ~ηκε", _, "ἑ~ῳ~ηκετε";
    perfect_imp_act_3: _, _, "ἑ~ῳ~ηκοντων";
    perfect_imp_mid_2: _, _, "ἑ~ῳ~ησθε";
    perfect_imp_mid_3: "ἑ~ῳ~ησθω", _, "ἑ~ῳ~ησθων";
    perfect_imp_pass_2: "ἑ~ῳ~ησο", _, "ἑ~ῳ~ησθε";
    perfect_imp_pass_3: _, "ἑ~ῳ~ησθων", "ἑ~ῳ~ησθων";
    pluperfect_mid_1: "ἑ~ῳ~ημην", _, _;
    pluperfect_mid_2: "ἑ~ῳ~ησο", "ἑ~ῳ~ησθον", _;
    pluperfect_mid_3: "ἑ~ῳ~ητο", _, "ἑ~ῳ~ηντο";
    pluperfect_pass_1: _, _, "ἑ~ῳ~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἑ~ῳ~ησθε";
    pluperfect_pass_3: _, "ἑ~ῳ~ησθην", _;
    infinitive_pres: "ἑ~οι~εῑν", _, "ἑ~οι~εῑσθαι";
    infinitive_fut: "ἑ~οι~ησειν", _, _;
    infinitive_aor: "ἑ~οι~ησαι", _, _;
    infinitive_perf: _, "ἑ~ῳ~ησθαι", _;
    participle_pres: "ἑ~οι~ων", _, "ἑ~οι~ουμενος";
    participle_aor: "ἑ~οι~ησᾱς", "ἑ~οι~ησᾰμενος", _;
    participle_perf: "ἑ~ῳ~ηκως", _, "ἑ~ῳ~ημενος";
    ),
    // Δειδῐᾰ: 1 words (δειδῐᾰ). Like Μετεκῑᾰθον, except in 45 cells: ind_act_1 sg, aorist_subj_act_1 sg, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_3 pl, aorist_subj_mid_1 sg, ….
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
    aorist_opt_act_3: "~~σειε|~~σειεν|~~σαι", "~~σαιτην", "~~σειᾰν|~~σαιεν";
    aorist_opt_mid_1: "~~σαιμην", _, _;
    aorist_opt_mid_2: "~~σαιο", _, _;
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
    perfect_opt_act_1: "~δο~κοιμῐ|~δο~κοιην", _, "~δο~κοιμεν";
    perfect_opt_act_2: "~δο~κοις|~δο~κοιης", _, _;
    perfect_opt_act_3: "~δο~κοι|~δο~κοιη", "~δο~κοιτην", _;
    perfect_imp_act_2: "~δο~κε", _, "~δο~κετε";
    perfect_imp_act_3: _, _, "~δο~κοντων";
    infinitive_aor: "~ῑσα~", _, _;
    participle_aor: "~~σᾱς", "~~σᾰμενος", _;
    participle_perf: "~δο~κως", _, _;
    ),
    // Λανθανομαι: 1 words (λανθανομαι). Like Μᾰστῐγοω, except in 54 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 du, ind_act_3 pl, ….
    conjugation!("λανθανομαι", 1, "~α~α~ομαι", like Μᾰστῐγοω;
    ind_act_1: "~ᾰ~ᾰ~ω", _, "~ᾰ~ᾰ~ομεν";
    ind_act_2: "~ᾰ~ᾰ~εις", _, _;
    ind_act_3: "~ᾰ~ᾰ~ει", "~ᾰ~ᾰ~ετον", "~ᾰ~ᾰ~ουσῐ|~ᾰ~ᾰ~ουσῐν";
    ind_mid_1: "~ᾰ~ᾰ~ομαι", _, "~ᾰ~ᾰ~ομεθᾰ";
    ind_mid_2: "~ᾰ~ᾰ~ῃ|~ᾰ~ᾰ~ει", "~ᾰ~ᾰ~εσθον", _;
    ind_pass_2: _, _, "~ᾰ~ᾰ~εσθε";
    ind_pass_3: "~ᾰ~ᾰ~εται", _, "~ᾰ~ᾰ~ονται";
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
    opt_mid_2: "~ᾰ~ᾰ~οιο", "~ᾰ~ᾰ~οισθον", _;
    opt_mid_3: "~ᾰ~ᾰ~οιτο", "~ᾰ~ᾰ~οισθην", "~ᾰ~ᾰ~οιντο";
    opt_pass_2: _, _, "~ᾰ~ᾰ~οισθε";
    imp_act_2: "~ᾰ~ᾰ~ε", "~ᾰ~ᾰ~ετον", "~ᾰ~ᾰ~ετε";
    imp_mid_2: "~ᾰ~ᾰ~ου", _, "~ᾰ~ᾰ~εσθε";
    imp_mid_3: "~ᾰ~ᾰ~εσθω", _, "~ᾰ~ᾰ~εσθων";
    imp_pass_2: _, _, "~ᾰ~ᾰ~εσθε";
    imp_pass_3: _, "~ᾰ~ᾰ~εσθων", "~ᾰ~ᾰ~εσθων";
    infinitive_pres: "~ᾰ~ᾰ~ειν", _, "~ᾰ~ᾰ~εσθαι";
    participle_pres: "~ᾰ~ᾰ~ων", _, "~ᾰ~ᾰ~ομενος";
    ),
    // Τρᾰπω: 1 words (τρᾰπω). Like Μῑγνῡμῐ, except in 27 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("τρᾰπω", 1, "~ᾰπω", like Μῑγνῡμῐ;
    ind_act_1: "~ᾰπω", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_act_1: "~αψω", _, "~αψομεν";
    future_ind_act_2: "~αψεις", "~αψετον", "~αψετε";
    future_ind_act_3: _, _, "~αψουσῐ|~αψουσῐν";
    future_ind_mid_1: "~αψομαι", _, "~αψομεθᾰ";
    future_ind_mid_2: "~αψεαι", "~αψεσθον", _;
    future_ind_mid_3: "~αψεται", _, _;
    future_opt_act_1: "~αψοιμῐ", _, "~αψοιμεν";
    future_opt_act_2: _, "~αψοιτον", "~αψοιτε";
    future_opt_act_3: "~αψοι", "~αψοιτην", "~αψοιεν";
    future_opt_mid_1: "~αψοιμην", _, "~αψοιμεθᾰ";
    future_opt_mid_2: "~αψοιο", _, _;
    future_opt_mid_3: _, "~αψοισθην", _;
    infinitive_fut: "~αψειν", _, _;
    ),
];
