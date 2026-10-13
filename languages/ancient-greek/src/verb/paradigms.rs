//! The conjugations, commonest first, each named after a word that follows it.
//!
//! A pattern is the form with `~` where the stem goes: `~а` is the stem and
//! `а`; a stem in two pieces, `~о~`, has a vowel between them that other
//! forms drop. Alternatives are separated by `|`, the usual one first. A `%`
//! is the first letter of the stem piece after it (reduplication: `%ε~κα`),
//! as the copy laws in `laws.rs` change it.

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

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`ind_as_subj`: one value read as another).
const IND_AS_SUBJ: [u16; 624] = [
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
    72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115,
    116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134,
    162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180,
    181, 182, 183, 184, 185, 186, 187, 188, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172,
    173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210,
    211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229,
    230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 270, 271, 272, 273, 274, 275,
    276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294,
    295, 296, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286,
    287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305,
    306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324,
    325, 326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343,
    344, 345, 346, 347, 348, 349, 350, 378, 379, 380, 381, 382, 383, 384, 385, 386, 387, 388, 389,
    390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401, 402, 403, 404, 378, 379, 380, 381,
    382, 383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400,
    401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419,
    420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438,
    439, 440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457,
    458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476,
    477, 478, 479, 480, 481, 482, 483, 484, 485, 513, 514, 515, 516, 517, 518, 519, 520, 521, 522,
    523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534, 535, 536, 537, 538, 539, 513, 514,
    515, 516, 517, 518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533,
    534, 535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552,
    553, 554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571,
    572, 573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590,
    591, 592, 593, 594, 595, 596, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609,
    610, 611, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`act_as_pass`: one value read as another).
const ACT_AS_PASS: [u16; 624] = [
    18, 19, 20, 21, 22, 23, 24, 25, 26, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
    24, 25, 26, 45, 46, 47, 48, 49, 50, 51, 52, 53, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 72, 73, 74, 75, 76, 77, 78, 79, 80, 63, 64, 65, 66, 67, 68, 69, 70, 71,
    72, 73, 74, 75, 76, 77, 78, 79, 80, 99, 100, 101, 102, 103, 104, 105, 106, 107, 90, 91, 92, 93,
    94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 126, 127, 128, 129, 130, 131,
    132, 133, 134, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132,
    133, 134, 153, 154, 155, 156, 157, 158, 159, 160, 161, 144, 145, 146, 147, 148, 149, 150, 151,
    152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 180, 181, 182, 183, 184, 185, 186, 187, 188,
    171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 207,
    208, 209, 210, 211, 212, 213, 214, 215, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208,
    209, 210, 211, 212, 213, 214, 215, 234, 235, 236, 237, 238, 239, 240, 241, 242, 225, 226, 227,
    228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 261, 262, 263, 264,
    265, 266, 267, 268, 269, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265,
    266, 267, 268, 269, 288, 289, 290, 291, 292, 293, 294, 295, 296, 279, 280, 281, 282, 283, 284,
    285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 315, 316, 317, 318, 319, 320, 321,
    322, 323, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322,
    323, 342, 343, 344, 345, 346, 347, 348, 349, 350, 333, 334, 335, 336, 337, 338, 339, 340, 341,
    342, 343, 344, 345, 346, 347, 348, 349, 350, 369, 370, 371, 372, 373, 374, 375, 376, 377, 360,
    361, 362, 363, 364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 396, 397,
    398, 399, 400, 401, 402, 403, 404, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398,
    399, 400, 401, 402, 403, 404, 423, 424, 425, 426, 427, 428, 429, 430, 431, 414, 415, 416, 417,
    418, 419, 420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 450, 451, 452, 453, 454,
    455, 456, 457, 458, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455,
    456, 457, 458, 477, 478, 479, 480, 481, 482, 483, 484, 485, 468, 469, 470, 471, 472, 473, 474,
    475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 504, 505, 506, 507, 508, 509, 510, 511,
    512, 495, 496, 497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512,
    531, 532, 533, 534, 535, 536, 537, 538, 539, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531,
    532, 533, 534, 535, 536, 537, 538, 539, 558, 559, 560, 561, 562, 563, 564, 565, 566, 549, 550,
    551, 552, 553, 554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 585, 586, 587,
    588, 589, 590, 591, 592, 593, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588,
    589, 590, 591, 592, 593, 596, 595, 596, 599, 598, 599, 602, 601, 602, 605, 604, 605, 608, 607,
    608, 611, 610, 611, 614, 613, 614, 617, 616, 617, 620, 619, 620, 623, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`perfect_as_future`: one value read as another).
const PERFECT_AS_FUTURE: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268,
    269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287,
    288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306,
    307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325,
    326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344,
    345, 346, 347, 348, 349, 350, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 498,
    499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517,
    518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534, 535, 536,
    537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555,
    556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572, 573, 574,
    575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592, 593,
    459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477,
    478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496,
    497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515,
    516, 517, 518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534,
    535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553,
    554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572,
    573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591,
    592, 593, 594, 595, 596, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610,
    611, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`mid_as_act`: one value read as another).
const MID_AS_ACT: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32, 33, 34, 35, 27, 28, 29, 30, 31, 32, 33, 34, 35, 45, 46, 47, 48, 49, 50, 51,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 54, 55, 56, 57, 58, 59, 60, 61, 62, 72, 73, 74, 75,
    76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 81, 82, 83, 84, 85, 86, 87, 88, 89, 99,
    100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 108, 109,
    110, 111, 112, 113, 114, 115, 116, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137,
    138, 139, 140, 141, 142, 143, 135, 136, 137, 138, 139, 140, 141, 142, 143, 153, 154, 155, 156,
    157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 162, 163, 164, 165, 166,
    167, 168, 169, 170, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194,
    195, 196, 197, 189, 190, 191, 192, 193, 194, 195, 196, 197, 207, 208, 209, 210, 211, 212, 213,
    214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 216, 217, 218, 219, 220, 221, 222, 223,
    224, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251,
    243, 244, 245, 246, 247, 248, 249, 250, 251, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270,
    271, 272, 273, 274, 275, 276, 277, 278, 270, 271, 272, 273, 274, 275, 276, 277, 278, 288, 289,
    290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 297, 298, 299,
    300, 301, 302, 303, 304, 305, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327,
    328, 329, 330, 331, 332, 324, 325, 326, 327, 328, 329, 330, 331, 332, 342, 343, 344, 345, 346,
    347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 351, 352, 353, 354, 355, 356,
    357, 358, 359, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384,
    385, 386, 378, 379, 380, 381, 382, 383, 384, 385, 386, 396, 397, 398, 399, 400, 401, 402, 403,
    404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 405, 406, 407, 408, 409, 410, 411, 412, 413,
    423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439, 440, 432,
    433, 434, 435, 436, 437, 438, 439, 440, 450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460,
    461, 462, 463, 464, 465, 466, 467, 459, 460, 461, 462, 463, 464, 465, 466, 467, 477, 478, 479,
    480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 486, 487, 488, 489,
    490, 491, 492, 493, 494, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517,
    518, 519, 520, 521, 513, 514, 515, 516, 517, 518, 519, 520, 521, 531, 532, 533, 534, 535, 536,
    537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 540, 541, 542, 543, 544, 545, 546,
    547, 548, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572, 573, 574,
    575, 567, 568, 569, 570, 571, 572, 573, 574, 575, 585, 586, 587, 588, 589, 590, 591, 592, 593,
    594, 594, 596, 597, 597, 599, 600, 600, 602, 603, 603, 605, 606, 606, 608, 609, 609, 611, 612,
    612, 614, 615, 615, 617, 618, 618, 620, 621, 621, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`pass_as_act`: one value read as another).
const PASS_AS_ACT: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 0, 1, 2, 3, 4, 5, 6, 7, 8, 27,
    28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 27, 28, 29, 30, 31, 32, 33,
    34, 35, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 54, 55, 56, 57,
    58, 59, 60, 61, 62, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 81,
    82, 83, 84, 85, 86, 87, 88, 89, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
    120, 121, 122, 123, 124, 125, 108, 109, 110, 111, 112, 113, 114, 115, 116, 135, 136, 137, 138,
    139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 135, 136, 137, 138, 139,
    140, 141, 142, 143, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
    177, 178, 179, 162, 163, 164, 165, 166, 167, 168, 169, 170, 189, 190, 191, 192, 193, 194, 195,
    196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 189, 190, 191, 192, 193, 194, 195, 196,
    197, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233,
    216, 217, 218, 219, 220, 221, 222, 223, 224, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252,
    253, 254, 255, 256, 257, 258, 259, 260, 243, 244, 245, 246, 247, 248, 249, 250, 251, 270, 271,
    272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287, 270, 271, 272,
    273, 274, 275, 276, 277, 278, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309,
    310, 311, 312, 313, 314, 297, 298, 299, 300, 301, 302, 303, 304, 305, 324, 325, 326, 327, 328,
    329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 324, 325, 326, 327, 328, 329,
    330, 331, 332, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365, 366,
    367, 368, 351, 352, 353, 354, 355, 356, 357, 358, 359, 378, 379, 380, 381, 382, 383, 384, 385,
    386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 378, 379, 380, 381, 382, 383, 384, 385, 386,
    405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420, 421, 422, 405,
    406, 407, 408, 409, 410, 411, 412, 413, 432, 433, 434, 435, 436, 437, 438, 439, 440, 441, 442,
    443, 444, 445, 446, 447, 448, 449, 432, 433, 434, 435, 436, 437, 438, 439, 440, 459, 460, 461,
    462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 459, 460, 461, 462,
    463, 464, 465, 466, 467, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 498, 499,
    500, 501, 502, 503, 486, 487, 488, 489, 490, 491, 492, 493, 494, 513, 514, 515, 516, 517, 518,
    519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 513, 514, 515, 516, 517, 518, 519,
    520, 521, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556,
    557, 540, 541, 542, 543, 544, 545, 546, 547, 548, 567, 568, 569, 570, 571, 572, 573, 574, 575,
    576, 577, 578, 579, 580, 581, 582, 583, 584, 567, 568, 569, 570, 571, 572, 573, 574, 575, 594,
    595, 594, 597, 598, 597, 600, 601, 600, 603, 604, 603, 606, 607, 606, 609, 610, 609, 612, 613,
    612, 615, 616, 615, 618, 619, 618, 621, 622, 621,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`aorist_as_present`: one value read as another).
const AORIST_AS_PRESENT: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
    35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
    59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82,
    83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365, 366,
    367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384, 385,
    386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401, 402, 403, 404,
    405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420, 421, 422, 423,
    424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439, 440, 441, 442,
    443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461,
    462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477, 478, 479, 480,
    481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 498, 499,
    500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518,
    519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534, 535, 536, 537,
    538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556,
    557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572, 573, 574, 575,
    576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592, 593, 594,
    595, 596, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610, 611, 612, 613,
    614, 615, 616, 617, 618, 619, 620, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`sg_as_pl`: one value read as another).
const SG_AS_PL: [u16; 624] = [
    2, 1, 2, 5, 4, 5, 8, 7, 8, 11, 10, 11, 14, 13, 14, 17, 16, 17, 20, 19, 20, 23, 22, 23, 26, 25,
    26, 29, 28, 29, 32, 31, 32, 35, 34, 35, 38, 37, 38, 41, 40, 41, 44, 43, 44, 47, 46, 47, 50, 49,
    50, 53, 52, 53, 56, 55, 56, 59, 58, 59, 62, 61, 62, 65, 64, 65, 68, 67, 68, 71, 70, 71, 74, 73,
    74, 77, 76, 77, 80, 79, 80, 83, 82, 83, 86, 85, 86, 89, 88, 89, 92, 91, 92, 95, 94, 95, 98, 97,
    98, 101, 100, 101, 104, 103, 104, 107, 106, 107, 110, 109, 110, 113, 112, 113, 116, 115, 116,
    119, 118, 119, 122, 121, 122, 125, 124, 125, 128, 127, 128, 131, 130, 131, 134, 133, 134, 137,
    136, 137, 140, 139, 140, 143, 142, 143, 146, 145, 146, 149, 148, 149, 152, 151, 152, 155, 154,
    155, 158, 157, 158, 161, 160, 161, 164, 163, 164, 167, 166, 167, 170, 169, 170, 173, 172, 173,
    176, 175, 176, 179, 178, 179, 182, 181, 182, 185, 184, 185, 188, 187, 188, 191, 190, 191, 194,
    193, 194, 197, 196, 197, 200, 199, 200, 203, 202, 203, 206, 205, 206, 209, 208, 209, 212, 211,
    212, 215, 214, 215, 218, 217, 218, 221, 220, 221, 224, 223, 224, 227, 226, 227, 230, 229, 230,
    233, 232, 233, 236, 235, 236, 239, 238, 239, 242, 241, 242, 245, 244, 245, 248, 247, 248, 251,
    250, 251, 254, 253, 254, 257, 256, 257, 260, 259, 260, 263, 262, 263, 266, 265, 266, 269, 268,
    269, 272, 271, 272, 275, 274, 275, 278, 277, 278, 281, 280, 281, 284, 283, 284, 287, 286, 287,
    290, 289, 290, 293, 292, 293, 296, 295, 296, 299, 298, 299, 302, 301, 302, 305, 304, 305, 308,
    307, 308, 311, 310, 311, 314, 313, 314, 317, 316, 317, 320, 319, 320, 323, 322, 323, 326, 325,
    326, 329, 328, 329, 332, 331, 332, 335, 334, 335, 338, 337, 338, 341, 340, 341, 344, 343, 344,
    347, 346, 347, 350, 349, 350, 353, 352, 353, 356, 355, 356, 359, 358, 359, 362, 361, 362, 365,
    364, 365, 368, 367, 368, 371, 370, 371, 374, 373, 374, 377, 376, 377, 380, 379, 380, 383, 382,
    383, 386, 385, 386, 389, 388, 389, 392, 391, 392, 395, 394, 395, 398, 397, 398, 401, 400, 401,
    404, 403, 404, 407, 406, 407, 410, 409, 410, 413, 412, 413, 416, 415, 416, 419, 418, 419, 422,
    421, 422, 425, 424, 425, 428, 427, 428, 431, 430, 431, 434, 433, 434, 437, 436, 437, 440, 439,
    440, 443, 442, 443, 446, 445, 446, 449, 448, 449, 452, 451, 452, 455, 454, 455, 458, 457, 458,
    461, 460, 461, 464, 463, 464, 467, 466, 467, 470, 469, 470, 473, 472, 473, 476, 475, 476, 479,
    478, 479, 482, 481, 482, 485, 484, 485, 488, 487, 488, 491, 490, 491, 494, 493, 494, 497, 496,
    497, 500, 499, 500, 503, 502, 503, 506, 505, 506, 509, 508, 509, 512, 511, 512, 515, 514, 515,
    518, 517, 518, 521, 520, 521, 524, 523, 524, 527, 526, 527, 530, 529, 530, 533, 532, 533, 536,
    535, 536, 539, 538, 539, 542, 541, 542, 545, 544, 545, 548, 547, 548, 551, 550, 551, 554, 553,
    554, 557, 556, 557, 560, 559, 560, 563, 562, 563, 566, 565, 566, 569, 568, 569, 572, 571, 572,
    575, 574, 575, 578, 577, 578, 581, 580, 581, 584, 583, 584, 587, 586, 587, 590, 589, 590, 593,
    592, 593, 594, 595, 596, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610,
    611, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`aor_as_fut`: one value read as another).
const AOR_AS_FUT: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268,
    269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287,
    288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306,
    307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325,
    326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344,
    345, 346, 347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363,
    364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382,
    383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401,
    402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420,
    421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439,
    440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458,
    459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477,
    478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496,
    497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515,
    516, 517, 518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534,
    535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553,
    554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572,
    573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591,
    592, 593, 594, 595, 596, 597, 598, 599, 597, 598, 599, 603, 604, 605, 606, 607, 608, 609, 610,
    611, 612, 613, 614, 612, 613, 614, 618, 619, 620, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`3_as_1`: one value read as another).
const C3_AS_1: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 0, 1, 2, 9, 10, 11, 12, 13, 14, 9, 10, 11, 18, 19, 20, 21, 22, 23, 18, 19,
    20, 27, 28, 29, 30, 31, 32, 27, 28, 29, 36, 37, 38, 39, 40, 41, 36, 37, 38, 45, 46, 47, 48, 49,
    50, 45, 46, 47, 54, 55, 56, 57, 58, 59, 54, 55, 56, 63, 64, 65, 66, 67, 68, 63, 64, 65, 72, 73,
    74, 75, 76, 77, 72, 73, 74, 81, 82, 83, 84, 85, 86, 81, 82, 83, 90, 91, 92, 93, 94, 95, 90, 91,
    92, 99, 100, 101, 102, 103, 104, 99, 100, 101, 108, 109, 110, 111, 112, 113, 108, 109, 110,
    117, 118, 119, 120, 121, 122, 117, 118, 119, 126, 127, 128, 129, 130, 131, 126, 127, 128, 135,
    136, 137, 138, 139, 140, 135, 136, 137, 144, 145, 146, 147, 148, 149, 144, 145, 146, 153, 154,
    155, 156, 157, 158, 153, 154, 155, 162, 163, 164, 165, 166, 167, 162, 163, 164, 171, 172, 173,
    174, 175, 176, 171, 172, 173, 180, 181, 182, 183, 184, 185, 180, 181, 182, 189, 190, 191, 192,
    193, 194, 189, 190, 191, 198, 199, 200, 201, 202, 203, 198, 199, 200, 207, 208, 209, 210, 211,
    212, 207, 208, 209, 216, 217, 218, 219, 220, 221, 216, 217, 218, 225, 226, 227, 228, 229, 230,
    225, 226, 227, 234, 235, 236, 237, 238, 239, 234, 235, 236, 243, 244, 245, 246, 247, 248, 243,
    244, 245, 252, 253, 254, 255, 256, 257, 252, 253, 254, 261, 262, 263, 264, 265, 266, 261, 262,
    263, 270, 271, 272, 273, 274, 275, 270, 271, 272, 279, 280, 281, 282, 283, 284, 279, 280, 281,
    288, 289, 290, 291, 292, 293, 288, 289, 290, 297, 298, 299, 300, 301, 302, 297, 298, 299, 306,
    307, 308, 309, 310, 311, 306, 307, 308, 315, 316, 317, 318, 319, 320, 315, 316, 317, 324, 325,
    326, 327, 328, 329, 324, 325, 326, 333, 334, 335, 336, 337, 338, 333, 334, 335, 342, 343, 344,
    345, 346, 347, 342, 343, 344, 351, 352, 353, 354, 355, 356, 351, 352, 353, 360, 361, 362, 363,
    364, 365, 360, 361, 362, 369, 370, 371, 372, 373, 374, 369, 370, 371, 378, 379, 380, 381, 382,
    383, 378, 379, 380, 387, 388, 389, 390, 391, 392, 387, 388, 389, 396, 397, 398, 399, 400, 401,
    396, 397, 398, 405, 406, 407, 408, 409, 410, 405, 406, 407, 414, 415, 416, 417, 418, 419, 414,
    415, 416, 423, 424, 425, 426, 427, 428, 423, 424, 425, 432, 433, 434, 435, 436, 437, 432, 433,
    434, 441, 442, 443, 444, 445, 446, 441, 442, 443, 450, 451, 452, 453, 454, 455, 450, 451, 452,
    459, 460, 461, 462, 463, 464, 459, 460, 461, 468, 469, 470, 471, 472, 473, 468, 469, 470, 477,
    478, 479, 480, 481, 482, 477, 478, 479, 486, 487, 488, 489, 490, 491, 486, 487, 488, 495, 496,
    497, 498, 499, 500, 495, 496, 497, 504, 505, 506, 507, 508, 509, 504, 505, 506, 513, 514, 515,
    516, 517, 518, 513, 514, 515, 522, 523, 524, 525, 526, 527, 522, 523, 524, 531, 532, 533, 534,
    535, 536, 531, 532, 533, 540, 541, 542, 543, 544, 545, 540, 541, 542, 549, 550, 551, 552, 553,
    554, 549, 550, 551, 558, 559, 560, 561, 562, 563, 558, 559, 560, 567, 568, 569, 570, 571, 572,
    567, 568, 569, 576, 577, 578, 579, 580, 581, 576, 577, 578, 585, 586, 587, 588, 589, 590, 585,
    586, 587, 594, 595, 596, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610,
    611, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`future_as_present`: one value read as another).
const FUTURE_AS_PRESENT: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 0, 1,
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75,
    76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
    100, 101, 102, 103, 104, 105, 106, 107, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253,
    254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270, 271, 272,
    273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291,
    292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310,
    311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327, 328, 329,
    330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346, 347, 348,
    349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365, 366, 367,
    368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384, 385, 386,
    387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401, 402, 403, 404, 405,
    406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420, 421, 422, 423, 424,
    425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439, 440, 441, 442, 443,
    444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462,
    463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477, 478, 479, 480, 481,
    482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 498, 499, 500,
    501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518, 519,
    520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534, 535, 536, 537, 538,
    539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557,
    558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572, 573, 574, 575, 576,
    577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592, 593, 594, 595,
    596, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610, 611, 612, 613, 614,
    615, 616, 617, 618, 619, 620, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`perf_as_pres`: one value read as another).
const PERF_AS_PRES: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268,
    269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287,
    288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306,
    307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325,
    326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344,
    345, 346, 347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363,
    364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382,
    383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401,
    402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420,
    421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439,
    440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458,
    459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477,
    478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496,
    497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515,
    516, 517, 518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534,
    535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553,
    554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572,
    573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591,
    592, 593, 594, 595, 596, 597, 598, 599, 600, 601, 602, 594, 595, 596, 606, 607, 608, 609, 610,
    611, 612, 613, 614, 615, 616, 617, 609, 610, 611, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`mid_as_pass`: one value read as another).
const MID_AS_PASS: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 18, 19, 20, 21, 22, 23, 24, 25, 26, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 45, 46, 47, 48, 49, 50, 51, 52, 53, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 72, 73, 74, 75, 76, 77, 78, 79, 80, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114,
    115, 116, 126, 127, 128, 129, 130, 131, 132, 133, 134, 126, 127, 128, 129, 130, 131, 132, 133,
    134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 153, 154, 155, 156, 157, 158, 159, 160, 161,
    153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 180,
    181, 182, 183, 184, 185, 186, 187, 188, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190,
    191, 192, 193, 194, 195, 196, 197, 207, 208, 209, 210, 211, 212, 213, 214, 215, 207, 208, 209,
    210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 234, 235, 236, 237,
    238, 239, 240, 241, 242, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247,
    248, 249, 250, 251, 261, 262, 263, 264, 265, 266, 267, 268, 269, 261, 262, 263, 264, 265, 266,
    267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 288, 289, 290, 291, 292, 293, 294,
    295, 296, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304,
    305, 315, 316, 317, 318, 319, 320, 321, 322, 323, 315, 316, 317, 318, 319, 320, 321, 322, 323,
    324, 325, 326, 327, 328, 329, 330, 331, 332, 342, 343, 344, 345, 346, 347, 348, 349, 350, 342,
    343, 344, 345, 346, 347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 369, 370,
    371, 372, 373, 374, 375, 376, 377, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380,
    381, 382, 383, 384, 385, 386, 396, 397, 398, 399, 400, 401, 402, 403, 404, 396, 397, 398, 399,
    400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 423, 424, 425, 426, 427,
    428, 429, 430, 431, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437,
    438, 439, 440, 450, 451, 452, 453, 454, 455, 456, 457, 458, 450, 451, 452, 453, 454, 455, 456,
    457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 477, 478, 479, 480, 481, 482, 483, 484,
    485, 477, 478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494,
    504, 505, 506, 507, 508, 509, 510, 511, 512, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513,
    514, 515, 516, 517, 518, 519, 520, 521, 531, 532, 533, 534, 535, 536, 537, 538, 539, 531, 532,
    533, 534, 535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 558, 559, 560,
    561, 562, 563, 564, 565, 566, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570,
    571, 572, 573, 574, 575, 585, 586, 587, 588, 589, 590, 591, 592, 593, 585, 586, 587, 588, 589,
    590, 591, 592, 593, 594, 596, 596, 597, 599, 599, 600, 602, 602, 603, 605, 605, 606, 608, 608,
    609, 611, 611, 612, 614, 614, 615, 617, 617, 618, 620, 620, 621, 623, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`fut_as_pres`: one value read as another).
const FUT_AS_PRES: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268,
    269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287,
    288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306,
    307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325,
    326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344,
    345, 346, 347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363,
    364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382,
    383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401,
    402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420,
    421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439,
    440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458,
    459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477,
    478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496,
    497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515,
    516, 517, 518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534,
    535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553,
    554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572,
    573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591,
    592, 593, 594, 595, 596, 594, 595, 596, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610,
    611, 609, 610, 611, 615, 616, 617, 618, 619, 620, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`aorist_as_future`: one value read as another).
const AORIST_AS_FUTURE: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 486, 487, 488, 489, 490, 491, 492,
    493, 494, 495, 496, 497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511,
    512, 513, 514, 515, 516, 517, 518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530,
    531, 532, 533, 534, 535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549,
    550, 551, 552, 553, 554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568,
    569, 570, 571, 572, 573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587,
    588, 589, 590, 591, 592, 593, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363,
    364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382,
    383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401,
    402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420,
    421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439,
    440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458,
    459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477,
    478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496,
    497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515,
    516, 517, 518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534,
    535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553,
    554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572,
    573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591,
    592, 593, 594, 595, 596, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610,
    611, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`pres_as_futp`: one value read as another).
const PRES_AS_FUTP: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268,
    269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287,
    288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306,
    307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325,
    326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344,
    345, 346, 347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363,
    364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382,
    383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401,
    402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420,
    421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439,
    440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458,
    459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477,
    478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496,
    497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515,
    516, 517, 518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534,
    535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553,
    554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572,
    573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591,
    592, 593, 606, 607, 608, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 621, 622,
    623, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`perf_as_futp`: one value read as another).
const PERF_AS_FUTP: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268,
    269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287,
    288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306,
    307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325,
    326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344,
    345, 346, 347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363,
    364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382,
    383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401,
    402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420,
    421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439,
    440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458,
    459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477,
    478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496,
    497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515,
    516, 517, 518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534,
    535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553,
    554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572,
    573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591,
    592, 593, 594, 595, 596, 597, 598, 599, 600, 601, 602, 606, 607, 608, 606, 607, 608, 609, 610,
    611, 612, 613, 614, 615, 616, 617, 621, 622, 623, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`present_as_future`: one value read as another).
const PRESENT_AS_FUTURE: [u16; 624] = [
    486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 498, 499, 500, 501, 502, 503, 504,
    505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518, 519, 520, 521, 522, 523,
    524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534, 535, 536, 537, 538, 539, 540, 541, 542,
    543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557, 558, 559, 560, 561,
    562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572, 573, 574, 575, 576, 577, 578, 579, 580,
    581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592, 593, 108, 109, 110, 111, 112, 113,
    114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132,
    133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151,
    152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170,
    171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189,
    190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208,
    209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227,
    228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246,
    247, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265,
    266, 267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284,
    285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303,
    304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322,
    323, 324, 325, 326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341,
    342, 343, 344, 345, 346, 347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360,
    361, 362, 363, 364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379,
    380, 381, 382, 383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398,
    399, 400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417,
    418, 419, 420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436,
    437, 438, 439, 440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455,
    456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474,
    475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493,
    494, 495, 496, 497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512,
    513, 514, 515, 516, 517, 518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531,
    532, 533, 534, 535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550,
    551, 552, 553, 554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569,
    570, 571, 572, 573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588,
    589, 590, 591, 592, 593, 594, 595, 596, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607,
    608, 609, 610, 611, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623,
];

/// A view: per cell (in the cells' order), the cell of the conjugation it is
/// like that it reads (`imperfect_as_pluperfect`: one value read as another).
const IMPERFECT_AS_PLUPERFECT: [u16; 624] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 459, 460, 461, 462, 463, 464, 465, 466, 467,
    468, 469, 470, 471, 472, 473, 474, 475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268,
    269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287,
    288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306,
    307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325,
    326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344,
    345, 346, 347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363,
    364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382,
    383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399, 400, 401,
    402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420,
    421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439,
    440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458,
    459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477,
    478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496,
    497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515,
    516, 517, 518, 519, 520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534,
    535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553,
    554, 555, 556, 557, 558, 559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572,
    573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591,
    592, 593, 594, 595, 596, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610,
    611, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623,
];

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `ἐχω`: 234 words.
    Ἐχω,
    /// Like `ποιεω`: 201 words.
    Ποιεω,
    /// Like `γῡροω`: 80 words.
    Γῡροω,
    /// Like `βᾰπτῐζω`: 76 words.
    Βᾰπτῐζω,
    /// Like `χρημᾰτῐζω`: 63 words.
    Χρημᾰτῐζω,
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
    /// Like `χρῑω`: 15 words.
    Χρῑω,
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
    pub const ALL: [Conjugation; 292] = [
        Conjugation::Ἐχω,
        Conjugation::Ποιεω,
        Conjugation::Γῡροω,
        Conjugation::Βᾰπτῐζω,
        Conjugation::Χρημᾰτῐζω,
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
        Conjugation::Χρῑω,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 292] = [
    // Ἐχω: 234 words (θελω, τῠγχᾰνω, δῐδᾰσκω, πῑνω, εὑρῐσκω, ᾀδω).
    conjugation!("ἐχω", 234, "~ω";
    stem imperfect: "~- like ῥεω";
    stem aorist_ind: "~ησ- like ἐχω";
    stem aorist_ind_pass: "~ηθη- like ἐχω";
    stem pluperfect_act: "~ηκε- like ἐχω";
    stem pluperfect: "~η- like πειρᾰω";
    ind_act_1: "~ω", _, "~ομεν";
    ind_act_2: "~εις", "~ετον", "~ετε";
    ind_act_3: "~ει", _, "~ουσῐ|~ουσῐν";
    ind_mid_1: "~ομαι", _, "~ομεθᾰ";
    ind_mid_2: "~ῃ", "~εσθον", "~εσθε";
    ind_mid_3: "~εται", _, "~ονται";
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", "~ητον", "~ητε";
    subj_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    subj_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    subj_mid_2: _, "~ησθον", "~ησθε";
    subj_mid_3: "~ηται", _, "~ωνται";
    opt_act_1: "~οιμῐ", _, "~οιμεν";
    opt_act_2: "~οις", "~οιτον", "~οιτε";
    opt_act_3: "~οι", "~οιτην", "~οιεν";
    opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    opt_mid_2: "~οιο", "~οισθον", "~οισθε";
    opt_mid_3: "~οιτο", "~οισθην", "~οιντο";
    imp_act_2: "~ε", _, _;
    imp_act_3: "~ετω", "~ετων", "~οντων";
    imp_mid_2: "~ου", _, _;
    imp_mid_3: "~εσθω", "~εσθων", _;
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", "~ησετε";
    future_ind_act_3: "~ησει", _, "~ησουσῐ";
    future_ind_mid_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_mid_2: "~ησῃ", "~ησεσθον", "~ησεσθε";
    future_ind_mid_3: "~ησεται", _, "~ησονται";
    future_ind_pass_1: "~ηθησομαι", _, "~ηθησομεθᾰ";
    future_ind_pass_2: "~ηθησῃ", "~ηθησεσθον", "~ηθησεσθε";
    future_ind_pass_3: "~ηθησεται", _, "~ηθησονται";
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: "~ησοις", "~ησοιτον", "~ησοιτε";
    future_opt_act_3: "~ησοι", "~ησοιτην", "~ησοιεν";
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_2: "~ησοιο", "~ησοισθον", "~ησοισθε";
    future_opt_mid_3: "~ησοιτο", "~ησοισθην", "~ησοιντο";
    future_opt_pass_1: "~ηθησοιμην", _, "~ηθησοιμεθᾰ";
    future_opt_pass_2: "~ηθησοιο", "~ηθησοισθον", "~ηθησοισθε";
    future_opt_pass_3: "~ηθησοιτο", "~ηθησοισθην", "~ηθησοιντο";
    aorist_subj_act_1: "~ησω", _, "~ησωμεν";
    aorist_subj_act_2: "~ησῃς", "~ησητον", "~ησητε";
    aorist_subj_act_3: "~ησῃ", _, "~ησωσῐ";
    aorist_subj_mid_1: "~ησωμαι", _, "~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~ησησθον", "~ησησθε";
    aorist_subj_mid_3: "~ησηται", _, "~ησωνται";
    aorist_subj_pass_1: "~ηθω", _, "~ηθωμεν";
    aorist_subj_pass_2: "~ηθῃς", "~ηθητον", "~ηθητε";
    aorist_subj_pass_3: "~ηθῃ", _, "~ηθωσῐ";
    aorist_opt_act_1: "~ησαιμῐ", _, "~ησαιμεν";
    aorist_opt_act_2: "~ησειᾰς", "~ησαιτον", "~ησαιτε";
    aorist_opt_act_3: "~ησειε", "~ησαιτην", "~ησειᾰν";
    aorist_opt_mid_1: "~ησαιμην", _, "~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ησαιο", "~ησαισθον", "~ησαισθε";
    aorist_opt_mid_3: "~ησαιτο", "~ησαισθην", "~ησαιντο";
    aorist_opt_pass_1: "~ηθειην", _, "~ηθεῑμεν";
    aorist_opt_pass_2: "~ηθειης", "~ηθεῑτον", "~ηθεῑτε";
    aorist_opt_pass_3: "~ηθειη", "~ηθειτην", "~ηθεῑεν";
    aorist_imp_act_2: "~ησον", "~ησᾰτον", "~ησᾰτε";
    aorist_imp_act_3: "~ησᾰτω", "~ησᾰτων", "~ησᾰντων";
    aorist_imp_mid_2: "~ησαι", "~ησᾰσθον", "~ησᾰσθε";
    aorist_imp_mid_3: "~ησᾰσθω", "~ησᾰσθων", _;
    aorist_imp_pass_2: "~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ηθητω", "~ηθητων", "~ηθεντων";
    perfect_ind_act_1: "%ε~ηκᾰ", _, "%ε~ηκᾰμεν";
    perfect_ind_act_2: "%ε~ηκᾰς", "%ε~ηκᾰτον", "%ε~ηκᾰτε";
    perfect_ind_act_3: "%ε~ηκε", _, "%ε~ηκᾱσῐ";
    perfect_ind_mid_1: "%ε~ημαι", _, "%ε~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ησαι", "%ε~ησθον", "%ε~ησθε";
    perfect_ind_mid_3: "%ε~ηται", _, "%ε~ηνται";
    perfect_subj_act_1: "%ε~ηκω", _, "%ε~ηκωμεν";
    perfect_subj_act_2: "%ε~ηκῃς", "%ε~ηκητον", "%ε~ηκητε";
    perfect_subj_act_3: "%ε~ηκῃ", _, "%ε~ηκωσῐ";
    perfect_opt_act_1: "%ε~ηκοιμῐ", _, "%ε~ηκοιμεν";
    perfect_opt_act_2: "%ε~ηκοις", "%ε~ηκοιτον", "%ε~ηκοιτε";
    perfect_opt_act_3: "%ε~ηκοι", "%ε~ηκοιτην", "%ε~ηκοιεν";
    perfect_imp_act_2: _, "%ε~ηκετον", "%ε~ηκετε";
    perfect_imp_act_3: "%ε~ηκετω", "%ε~ηκετων", "%ε~ηκοντων";
    perfect_imp_mid_2: "%ε~ησο", _, _;
    perfect_imp_mid_3: "%ε~ησθω", "%ε~ησθων", _;
    future_perfect_ind_mid_1: "%ε~ησομαι", _, "%ε~ησομεθᾰ";
    future_perfect_ind_mid_2: "%ε~ησῃ", "%ε~ησεσθον", "%ε~ησεσθε";
    future_perfect_ind_mid_3: "%ε~ησεται", _, "%ε~ησονται";
    future_perfect_opt_mid_1: "%ε~ησοιμην", _, "%ε~ησοιμεθᾰ";
    future_perfect_opt_mid_2: "%ε~ησοιο", "%ε~ησοισθον", "%ε~ησοισθε";
    future_perfect_opt_mid_3: "%ε~ησοιτο", "%ε~ησοισθην", "%ε~ησοιντο";
    infinitive_pres: "~ειν", "~εσθαι", _;
    infinitive_fut: "~ησειν", "~ησεσθαι", "~ηθησεσθαι";
    infinitive_aor: "~ησαι", "~ησᾰσθαι", "~ηθηναι";
    infinitive_perf: "%ε~ηκεναι", "%ε~ησθαι", _;
    infinitive_futp: _, "%ε~ησεσθαι", _;
    participle_pres: "~ων", "~ομενος", _;
    participle_fut: "~ησων", "~ησομενος", "~ηθησομενος";
    participle_aor: "~ησᾱς", "~ησᾰμενος", "~ηθεις";
    participle_perf: "%ε~ηκως", "%ε~ημενος", _;
    participle_futp: _, "%ε~ησομενος", _;
    ),
    // Ποιεω: 201 words (φοβεω, βλασφημεω, γᾰμεω, μῑσεω, διακονεω, ποιεω). Like Ἐχω, except in 72 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ποιεω", 201, "~εω", like Ἐχω;
    stem aorist_ind: "~ησ- like ποιεω";
    ind_act_1: _, _, "~οῡμεν";
    ind_act_2: "~εῑς", "~εῑτον", "~εῑτε";
    ind_act_3: "~εῑ", _, "~οῡσῐ|~οῡσῐν";
    ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    ind_mid_2: "~εῑ|~ῃ", "~εῑσθον", "~εῑσθε";
    ind_mid_3: "~εῑται", _, "~οῡνται";
    opt_act_1: "~οιην|~οῑμῐ", _, "~οῑμεν|~οιημεν";
    opt_act_2: "~οιης|~οῑς", "~οῑτον|~οιητον", "~οῑτε|~οιητε";
    opt_act_3: "~οιη|~οῑ", "~οιτην|~οιητην", "~οῑεν|~οιησᾰν";
    opt_mid_2: "~οῑο", "~οῑσθον", "~οῑσθε";
    opt_mid_3: "~οῑτο", _, "~οῑντο";
    imp_act_2: "~ει", _, _;
    imp_act_3: "~ειτω", "~ειτων", "~ουντων";
    imp_mid_2: "~οῡ", _, _;
    imp_mid_3: "~εισθω", "~εισθων", _;
    imperfect_act_1: "ἐ~ουν", _, "ἐ~οῡμεν";
    imperfect_act_2: "ἐ~εις", "ἐ~εῑτον", "ἐ~εῑτε";
    imperfect_act_3: "ἐ~ει", "ἐ~ειτην", "ἐ~ουν";
    imperfect_mid_1: "ἐ~ουμην", _, "ἐ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~οῡ", "ἐ~εῑσθον", "ἐ~εῑσθε";
    imperfect_mid_3: "ἐ~εῑτο", "ἐ~εισθην", "ἐ~οῡντο";
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    future_ind_mid_2: "~ησῃ|~ησει", _, _;
    future_ind_pass_2: "~ηθησῃ|~ηθησει", _, _;
    aorist_subj_act_3: _, _, "~ησωσῐ|~ησωσῐν";
    aorist_subj_pass_3: _, _, "~ηθωσῐ|~ηθωσῐν";
    aorist_opt_act_2: "~ησειᾰς|~ησαις", _, _;
    aorist_opt_act_3: "~ησειε|~ησειεν|~ησαι", _, "~ησειᾰν|~ησαιεν";
    aorist_opt_pass_1: _, _, "~ηθεῑμεν|~ηθειημεν";
    aorist_opt_pass_2: _, "~ηθεῑτον|~ηθειητον", "~ηθεῑτε|~ηθειητε";
    aorist_opt_pass_3: _, "~ηθειτην|~ηθειητην", "~ηθεῑεν|~ηθειησᾰν";
    perfect_ind_act_3: "%ε~ηκε|%ε~ηκεν", _, "%ε~ηκᾱσῐ|%ε~ηκᾱσῐν";
    perfect_subj_act_3: _, _, "%ε~ηκωσῐ|%ε~ηκωσῐν";
    perfect_opt_act_1: "%ε~ηκοιμῐ|%ε~ηκοιην", _, _;
    perfect_opt_act_2: "%ε~ηκοις|%ε~ηκοιης", _, _;
    perfect_opt_act_3: "%ε~ηκοι|%ε~ηκοιη", _, _;
    perfect_imp_act_2: "%ε~ηκε", _, _;
    infinitive_pres: "~εῑν", "~εῑσθαι", _;
    participle_pres: _, "~ουμενος", _;
    ),
    // Γῡροω: 80 words (ὁμοιοω, μορφοω, ζηλοω, ναρκοω, κοινοω, ἀλλοιοω). Like Ποιεω, except in 204 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("γῡροω", 80, "~οω", like Ποιεω as IND_AS_SUBJ;
    stem aorist_ind: "~ωσ- like ποιεω";
    stem aorist_ind_pass: "~ωθη- like ἐχω";
    stem pluperfect_act: "~ωκε- like ἐχω";
    stem pluperfect: "~ω- like πειρᾰω";
    ind_act_1: _, _, "~οῡμεν";
    ind_act_2: "~οῑς", "~οῡτον", "~οῡτε";
    ind_act_3: "~οῑ", _, "~οῡσῐ|~οῡσῐν";
    ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    ind_mid_2: "~οῑ", "~οῡσθον", "~οῡσθε";
    ind_mid_3: "~οῡται", _, "~οῡνται";
    subj_act_2: "~οῑς", "~ωτον", "~ωτε";
    subj_act_3: "~οῑ", _, _;
    subj_mid_2: _, "~ωσθον", "~ωσθε";
    subj_mid_3: "~ωται", _, _;
    imp_act_2: "~ου", _, _;
    imp_act_3: "~ουτω", "~ουτων", _;
    imp_mid_3: "~ουσθω", "~ουσθων", _;
    imperfect_act_2: "ἐ~ους", "ἐ~οῡτον", "ἐ~οῡτε";
    imperfect_act_3: "ἐ~ου", "ἐ~ουτην", _;
    imperfect_mid_2: _, "ἐ~οῡσθον", "ἐ~οῡσθε";
    imperfect_mid_3: "ἐ~οῡτο", "ἐ~ουσθην", _;
    future_ind_act_1: "~ωσω", _, "~ωσομεν";
    future_ind_act_2: "~ωσεις", "~ωσετον", "~ωσετε";
    future_ind_act_3: "~ωσει", _, "~ωσουσῐ";
    future_ind_mid_1: "~ωσομαι", _, "~ωσομεθᾰ";
    future_ind_mid_2: "~ωσῃ|~ωσει", "~ωσεσθον", "~ωσεσθε";
    future_ind_mid_3: "~ωσεται", _, "~ωσονται";
    future_ind_pass_1: "~ωθησομαι", _, "~ωθησομεθᾰ";
    future_ind_pass_2: "~ωθησῃ", "~ωθησεσθον", "~ωθησεσθε";
    future_ind_pass_3: "~ωθησεται", _, "~ωθησονται";
    future_opt_act_1: "~ωσοιμῐ", _, "~ωσοιμεν";
    future_opt_act_2: "~ωσοις", "~ωσοιτον", "~ωσοιτε";
    future_opt_act_3: "~ωσοι", "~ωσοιτην", "~ωσοιεν";
    future_opt_mid_1: "~ωσοιμην", _, "~ωσοιμεθᾰ";
    future_opt_mid_2: "~ωσοιο", "~ωσοισθον", "~ωσοισθε";
    future_opt_mid_3: "~ωσοιτο", "~ωσοισθην", "~ωσοιντο";
    future_opt_pass_1: "~ωθησοιμην", _, "~ωθησοιμεθᾰ";
    future_opt_pass_2: "~ωθησοιο", "~ωθησοισθον", "~ωθησοισθε";
    future_opt_pass_3: "~ωθησοιτο", "~ωθησοισθην", "~ωθησοιντο";
    aorist_subj_act_1: "~ωσω", _, "~ωσωμεν";
    aorist_subj_act_2: "~ωσῃς", "~ωσητον", "~ωσητε";
    aorist_subj_act_3: "~ωσῃ", _, "~ωσωσῐ|~ωσωσῐν";
    aorist_subj_mid_1: "~ωσωμαι", _, "~ωσωμεθᾰ";
    aorist_subj_mid_2: _, "~ωσησθον", "~ωσησθε";
    aorist_subj_mid_3: "~ωσηται", _, "~ωσωνται";
    aorist_subj_pass_1: "~ωθω", _, "~ωθωμεν";
    aorist_subj_pass_2: "~ωθῃς", "~ωθητον", "~ωθητε";
    aorist_subj_pass_3: "~ωθῃ", _, "~ωθωσῐ";
    aorist_opt_act_1: "~ωσαιμῐ", _, "~ωσαιμεν";
    aorist_opt_act_2: "~ωσειᾰς|~ωσαις", "~ωσαιτον", "~ωσαιτε";
    aorist_opt_act_3: "~ωσειε|~ωσειεν|~ωσαι", "~ωσαιτην", "~ωσειᾰν|~ωσαιεν";
    aorist_opt_mid_1: "~ωσαιμην", _, "~ωσαιμεθᾰ";
    aorist_opt_mid_2: "~ωσαιο", "~ωσαισθον", "~ωσαισθε";
    aorist_opt_mid_3: "~ωσαιτο", "~ωσαισθην", "~ωσαιντο";
    aorist_opt_pass_1: "~ωθειην", _, "~ωθεῑμεν";
    aorist_opt_pass_2: "~ωθειης", "~ωθεῑτον", "~ωθεῑτε";
    aorist_opt_pass_3: "~ωθειη", "~ωθειτην", "~ωθεῑεν";
    aorist_imp_act_2: "~ωσον", "~ωσᾰτον", "~ωσᾰτε";
    aorist_imp_act_3: "~ωσᾰτω", "~ωσᾰτων", "~ωσᾰντων";
    aorist_imp_mid_2: "~ωσαι", "~ωσᾰσθον", "~ωσᾰσθε";
    aorist_imp_mid_3: "~ωσᾰσθω", "~ωσᾰσθων", _;
    aorist_imp_pass_2: "~ωθητῐ", _, _;
    aorist_imp_pass_3: "~ωθητω", "~ωθητων", "~ωθεντων";
    perfect_ind_act_1: "%ε~ωκᾰ", _, "%ε~ωκᾰμεν";
    perfect_ind_act_2: "%ε~ωκᾰς", "%ε~ωκᾰτον", "%ε~ωκᾰτε";
    perfect_ind_act_3: "%ε~ωκε", _, "%ε~ωκᾱσῐ";
    perfect_ind_mid_1: "%ε~ωμαι", _, "%ε~ωμεθᾰ";
    perfect_ind_mid_2: "%ε~ωσαι", "%ε~ωσθον", "%ε~ωσθε";
    perfect_ind_mid_3: "%ε~ωται", _, "%ε~ωνται";
    perfect_subj_act_1: "%ε~ωκω", _, "%ε~ωκωμεν";
    perfect_subj_act_2: "%ε~ωκῃς", "%ε~ωκητον", "%ε~ωκητε";
    perfect_subj_act_3: "%ε~ωκῃ", _, "%ε~ωκωσῐ";
    perfect_opt_act_1: "%ε~ωκοιμῐ", _, "%ε~ωκοιμεν";
    perfect_opt_act_2: "%ε~ωκοις", "%ε~ωκοιτον", "%ε~ωκοιτε";
    perfect_opt_act_3: "%ε~ωκοι", "%ε~ωκοιτην", "%ε~ωκοιεν";
    perfect_imp_act_2: derived, "%ε~ωκετον", "%ε~ωκετε";
    perfect_imp_act_3: "%ε~ωκετω", "%ε~ωκετων", "%ε~ωκοντων";
    perfect_imp_mid_2: "%ε~ωσο", _, _;
    perfect_imp_mid_3: "%ε~ωσθω", "%ε~ωσθων", _;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_pres: "~οῡν", "~οῡσθαι", _;
    infinitive_fut: "~ωσειν", "~ωσεσθαι", "~ωθησεσθαι";
    infinitive_aor: "~ωσαι", "~ωσᾰσθαι", "~ωθηναι";
    infinitive_perf: "%ε~ωκεναι", "%ε~ωσθαι", _;
    infinitive_futp: _, derived, _;
    participle_fut: "~ωσων", "~ωσομενος", "~ωθησομενος";
    participle_aor: "~ωσᾱς", "~ωσᾰμενος", "~ωθεις";
    participle_perf: "%ε~ωκως", "%ε~ωμενος", _;
    participle_futp: _, derived, _;
    ),
    // Βᾰπτῐζω: 76 words (πυρρᾰζω, σκᾰνδᾰλῐζω, βᾰπτῐζω, δειγμᾰτῐζω, δᾰμαζω, γογγῠζω). Like Ἐχω, except in 236 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("βᾰπτῐζω", 76, "~ζω", like Ἐχω;
    stem imperfect: "~ζ- like ῥεω";
    stem aorist_ind: "~σ- like ἐχω";
    stem aorist_ind_pass: "~σθη- like ἐχω";
    stem pluperfect_act: "~κε- like ἐχω";
    ind_act_1: "~ζω", _, "~ζομεν";
    ind_act_2: "~ζεις", "~ζετον", "~ζετε";
    ind_act_3: "~ζει", _, "~ζουσῐ";
    ind_mid_1: "~ζομαι", _, "~ζομεθᾰ";
    ind_mid_2: "~ζῃ", "~ζεσθον", "~ζεσθε";
    ind_mid_3: "~ζεται", _, "~ζονται";
    subj_act_1: "~ζω", _, "~ζωμεν";
    subj_act_2: "~ζῃς", "~ζητον", "~ζητε";
    subj_act_3: "~ζῃ", _, "~ζωσῐ";
    subj_mid_1: "~ζωμαι", _, "~ζωμεθᾰ";
    subj_mid_2: _, "~ζησθον", "~ζησθε";
    subj_mid_3: "~ζηται", _, "~ζωνται";
    opt_act_1: "~ζοιμῐ", _, "~ζοιμεν";
    opt_act_2: "~ζοις", "~ζοιτον", "~ζοιτε";
    opt_act_3: "~ζοι", "~ζοιτην", "~ζοιεν";
    opt_mid_1: "~ζοιμην", _, "~ζοιμεθᾰ";
    opt_mid_2: "~ζοιο", "~ζοισθον", "~ζοισθε";
    opt_mid_3: "~ζοιτο", "~ζοισθην", "~ζοιντο";
    imp_act_2: "~ζε", _, _;
    imp_act_3: "~ζετω", "~ζετων", "~ζοντων";
    imp_mid_2: "~ζου", _, _;
    imp_mid_3: "~ζεσθω", "~ζεσθων", _;
    future_ind_act_1: "~σω", _, "~σομεν";
    future_ind_act_2: "~σεις", "~σετον", "~σετε";
    future_ind_act_3: "~σει", _, "~σουσῐ";
    future_ind_mid_1: "~σομαι", _, "~σομεθᾰ";
    future_ind_mid_2: "~σῃ", "~σεσθον", "~σεσθε";
    future_ind_mid_3: "~σεται", _, "~σονται";
    future_ind_pass_1: "~σθησομαι", _, "~σθησομεθᾰ";
    future_ind_pass_2: "~σθησῃ", "~σθησεσθον", "~σθησεσθε";
    future_ind_pass_3: "~σθησεται", _, "~σθησονται";
    future_opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    future_opt_act_2: "~σοις", "~σοιτον", "~σοιτε";
    future_opt_act_3: "~σοι", "~σοιτην", "~σοιεν";
    future_opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    future_opt_mid_2: "~σοιο", "~σοισθον", "~σοισθε";
    future_opt_mid_3: "~σοιτο", "~σοισθην", "~σοιντο";
    future_opt_pass_1: "~σθησοιμην", _, "~σθησοιμεθᾰ";
    future_opt_pass_2: "~σθησοιο", "~σθησοισθον", "~σθησοισθε";
    future_opt_pass_3: "~σθησοιτο", "~σθησοισθην", "~σθησοιντο";
    aorist_subj_act_1: "~σω", _, "~σωμεν";
    aorist_subj_act_2: "~σῃς", "~σητον", "~σητε";
    aorist_subj_act_3: "~σῃ", _, "~σωσῐ";
    aorist_subj_mid_1: "~σωμαι", _, "~σωμεθᾰ";
    aorist_subj_mid_2: _, "~σησθον", "~σησθε";
    aorist_subj_mid_3: "~σηται", _, "~σωνται";
    aorist_subj_pass_1: "~σθω", _, "~σθωμεν";
    aorist_subj_pass_2: "~σθῃς", "~σθητον", "~σθητε";
    aorist_subj_pass_3: "~σθῃ", _, "~σθωσῐ";
    aorist_opt_act_1: "~σαιμῐ", _, "~σαιμεν";
    aorist_opt_act_2: "~σειᾰς", "~σαιτον", "~σαιτε";
    aorist_opt_act_3: "~σειε", "~σαιτην", "~σειᾰν";
    aorist_opt_mid_1: "~σαιμην", _, "~σαιμεθᾰ";
    aorist_opt_mid_2: "~σαιο", "~σαισθον", "~σαισθε";
    aorist_opt_mid_3: "~σαιτο", "~σαισθην", "~σαιντο";
    aorist_opt_pass_1: "~σθειην", _, "~σθεῑμεν";
    aorist_opt_pass_2: "~σθειης", "~σθεῑτον", "~σθεῑτε";
    aorist_opt_pass_3: "~σθειη", "~σθειτην", "~σθεῑεν";
    aorist_imp_act_2: "~σον", "~σᾰτον", "~σᾰτε";
    aorist_imp_act_3: "~σᾰτω", "~σᾰτων", "~σᾰντων";
    aorist_imp_mid_2: "~σαι", "~σᾰσθον", "~σᾰσθε";
    aorist_imp_mid_3: "~σᾰσθω", "~σᾰσθων", _;
    aorist_imp_pass_2: "~σθητῐ", _, _;
    aorist_imp_pass_3: "~σθητω", "~σθητων", "~σθεντων";
    perfect_ind_act_1: "%ε~κᾰ", _, "%ε~κᾰμεν";
    perfect_ind_act_2: "%ε~κᾰς", "%ε~κᾰτον", "%ε~κᾰτε";
    perfect_ind_act_3: "%ε~κε", _, "%ε~κᾱσῐ";
    perfect_ind_mid_1: "%ε~σμαι", _, "%ε~σμεθᾰ";
    perfect_ind_mid_2: "%ε~σαι", "%ε~σθον", "%ε~σθε";
    perfect_ind_mid_3: "%ε~σται", _, "%ε~δᾰται";
    perfect_subj_act_1: "%ε~κω", _, "%ε~κωμεν";
    perfect_subj_act_2: "%ε~κῃς", "%ε~κητον", "%ε~κητε";
    perfect_subj_act_3: "%ε~κῃ", _, "%ε~κωσῐ";
    perfect_opt_act_1: "%ε~κοιμῐ", _, "%ε~κοιμεν";
    perfect_opt_act_2: "%ε~κοις", "%ε~κοιτον", "%ε~κοιτε";
    perfect_opt_act_3: "%ε~κοι", "%ε~κοιτην", "%ε~κοιεν";
    perfect_imp_act_2: _, "%ε~κετον", "%ε~κετε";
    perfect_imp_act_3: "%ε~κετω", "%ε~κετων", "%ε~κοντων";
    perfect_imp_mid_2: "%ε~σο", _, _;
    perfect_imp_mid_3: "%ε~σθω", "%ε~σθων", _;
    pluperfect_mid_1: "ἐ%ε~σμην", _, "ἐ%ε~σμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~σο", "ἐ%ε~σθον", "ἐ%ε~σθε";
    pluperfect_mid_3: "ἐ%ε~στο", "ἐ%ε~σθην", "ἐ%ε~δᾰτο";
    future_perfect_ind_mid_1: "%ε~σομαι", _, "%ε~σομεθᾰ";
    future_perfect_ind_mid_2: "%ε~σῃ", "%ε~σεσθον", "%ε~σεσθε";
    future_perfect_ind_mid_3: "%ε~σεται", _, "%ε~σονται";
    future_perfect_opt_mid_1: "%ε~σοιμην", _, "%ε~σοιμεθᾰ";
    future_perfect_opt_mid_2: "%ε~σοιο", "%ε~σοισθον", "%ε~σοισθε";
    future_perfect_opt_mid_3: "%ε~σοιτο", "%ε~σοισθην", "%ε~σοιντο";
    infinitive_pres: "~ζειν", "~ζεσθαι", _;
    infinitive_fut: "~σειν", "~σεσθαι", "~σθησεσθαι";
    infinitive_aor: "~σαι", "~σᾰσθαι", "~σθηναι";
    infinitive_perf: "%ε~κεναι", "%ε~σθαι", _;
    infinitive_futp: _, "%ε~σεσθαι", _;
    participle_pres: "~ζων", "~ζομενος", _;
    participle_fut: "~σων", "~σομενος", "~σθησομενος";
    participle_aor: "~σᾱς", "~σᾰμενος", "~σθεις";
    participle_perf: "%ε~κως", "%ε~σμενος", _;
    participle_futp: _, "%ε~σομενος", _;
    ),
    // Χρημᾰτῐζω: 63 words (κᾰθᾰρῐζω, βᾰσᾰνῐζω, κᾰπνῐζω, φενᾱκῐζω, γεμῐζω, ἱζω). Like Βᾰπτῐζω, except in 73 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("χρημᾰτῐζω", 63, "~ζω", like Βᾰπτῐζω;
    stem aorist_ind: "~σ- like ποιεω";
    stem pluperfect_act: "~κ- like χρημᾰτῐζω";
    ind_act_3: _, _, "~ζουσῐ|~ζουσῐν";
    ind_mid_2: "~ζῃ|~ζει", _, _;
    subj_act_3: _, _, "~ζωσῐ|~ζωσῐν";
    imperfect_act_3: "ἐ~ζε|ἐ~ζεν", _, _;
    future_ind_act_1: "~ω", _, "~οῡμεν";
    future_ind_act_2: "~εῑς", "~εῑτον", "~εῑτε";
    future_ind_act_3: "~εῑ", _, "~οῡσῐ|~οῡσῐν";
    future_ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    future_ind_mid_2: "~εῑ|~ῃ", "~εῑσθον", "~εῑσθε";
    future_ind_mid_3: "~εῑται", _, "~οῡνται";
    future_ind_pass_2: "~σθησῃ|~σθησει", _, _;
    future_opt_act_1: "~οιην|~οῑμῐ", _, "~οῑμεν|~οιημεν";
    future_opt_act_2: "~οιης|~οῑς", "~οῑτον|~οιητον", "~οῑτε|~οιητε";
    future_opt_act_3: "~οιη|~οῑ", "~οιτην|~οιητην", "~οῑεν|~οιησᾰν";
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_2: "~οῑο", "~οῑσθον", "~οῑσθε";
    future_opt_mid_3: "~οῑτο", "~οισθην", "~οῑντο";
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_subj_pass_3: _, _, "~σθωσῐ|~σθωσῐν";
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", _, "~σειᾰν|~σαιεν";
    aorist_opt_pass_1: _, _, "~σθεῑμεν|~σθειημεν";
    aorist_opt_pass_2: _, "~σθεῑτον|~σθειητον", "~σθεῑτε|~σθειητε";
    aorist_opt_pass_3: _, "~σθειτην|~σθειητην", "~σθεῑεν|~σθειησᾰν";
    perfect_ind_act_3: "%ε~κε|%ε~κεν", _, "%ε~κᾱσῐ|%ε~κᾱσῐν";
    perfect_subj_act_3: _, _, "%ε~κωσῐ|%ε~κωσῐν";
    perfect_opt_act_1: "%ε~κοιμῐ|%ε~κοιην", _, _;
    perfect_opt_act_2: "%ε~κοις|%ε~κοιης", _, _;
    perfect_opt_act_3: "%ε~κοι|%ε~κοιη", _, _;
    perfect_imp_act_2: "%ε~κε", _, _;
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_fut: "~εῑν", "~εῑσθαι", _;
    infinitive_futp: _, derived, _;
    participle_fut: "~ων", "~ουμενος", _;
    participle_futp: _, derived, _;
    ),
    // Βουλευω: 61 words (θερᾰπευω, πῐστευω, νηστευω, χορευω, παιδευω, λουω). Like Γῡροω, except in 227 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("βουλευω", 61, "~υω", like Γῡροω;
    stem aorist_ind: "~υσ- like ποιεω";
    stem aorist_ind_pass: "~υθη- like ἐχω";
    stem pluperfect_act: "~υκε- like ἐχω";
    stem pluperfect: "~υ- like πειρᾰω";
    ind_act_1: "~υω", _, "~υομεν";
    ind_act_2: "~υεις", "~υετον", "~υετε";
    ind_act_3: "~υει", _, "~υουσῐ|~υουσῐν";
    ind_mid_1: "~υομαι", _, "~υομεθᾰ";
    ind_mid_2: "~υῃ|~υει", "~υεσθον", "~υεσθε";
    ind_mid_3: "~υεται", _, "~υονται";
    subj_act_1: "~υω", _, "~υωμεν";
    subj_act_2: "~υῃς", "~υητον", "~υητε";
    subj_act_3: "~υῃ", _, "~υωσῐ|~υωσῐν";
    subj_mid_1: "~υωμαι", _, "~υωμεθᾰ";
    subj_mid_2: _, "~υησθον", "~υησθε";
    subj_mid_3: "~υηται", _, "~υωνται";
    opt_act_1: "~υοιμῐ", _, "~υοιμεν";
    opt_act_2: "~υοις", "~υοιτον", "~υοιτε";
    opt_act_3: "~υοι", "~υοιτην", "~υοιεν";
    opt_mid_1: "~υοιμην", _, "~υοιμεθᾰ";
    opt_mid_2: "~υοιο", "~υοισθον", "~υοισθε";
    opt_mid_3: "~υοιτο", "~υοισθην", "~υοιντο";
    imp_act_2: "~υε", _, _;
    imp_act_3: "~υετω", "~υετων", "~υοντων";
    imp_mid_2: "~υου", _, _;
    imp_mid_3: "~υεσθω", "~υεσθων", _;
    imperfect_act_1: "ἐ~υον", _, "ἐ~υομεν";
    imperfect_act_2: "ἐ~υες", "ἐ~υετον", "ἐ~υετε";
    imperfect_act_3: "ἐ~υε|ἐ~υεν", "ἐ~υετην", "ἐ~υον";
    imperfect_mid_1: "ἐ~υομην", _, "ἐ~υομεθᾰ";
    imperfect_mid_2: "ἐ~υου", "ἐ~υεσθον", "ἐ~υεσθε";
    imperfect_mid_3: "ἐ~υετο", "ἐ~υεσθην", "ἐ~υοντο";
    future_ind_act_1: "~υσω", _, "~υσομεν";
    future_ind_act_2: "~υσεις", "~υσετον", "~υσετε";
    future_ind_act_3: "~υσει", _, "~υσουσῐ|~υσουσῐν";
    future_ind_mid_1: "~υσομαι", _, "~υσομεθᾰ";
    future_ind_mid_2: "~υσῃ|~υσει", "~υσεσθον", "~υσεσθε";
    future_ind_mid_3: "~υσεται", _, "~υσονται";
    future_ind_pass_1: "~υθησομαι", _, "~υθησομεθᾰ";
    future_ind_pass_2: "~υθησῃ|~υθησει", "~υθησεσθον", "~υθησεσθε";
    future_ind_pass_3: "~υθησεται", _, "~υθησονται";
    future_opt_act_1: "~υσοιμῐ", _, "~υσοιμεν";
    future_opt_act_2: "~υσοις", "~υσοιτον", "~υσοιτε";
    future_opt_act_3: "~υσοι", "~υσοιτην", "~υσοιεν";
    future_opt_mid_1: "~υσοιμην", _, "~υσοιμεθᾰ";
    future_opt_mid_2: "~υσοιο", "~υσοισθον", "~υσοισθε";
    future_opt_mid_3: "~υσοιτο", "~υσοισθην", "~υσοιντο";
    future_opt_pass_1: "~υθησοιμην", _, "~υθησοιμεθᾰ";
    future_opt_pass_2: "~υθησοιο", "~υθησοισθον", "~υθησοισθε";
    future_opt_pass_3: "~υθησοιτο", "~υθησοισθην", "~υθησοιντο";
    aorist_subj_act_1: "~υσω", _, "~υσωμεν";
    aorist_subj_act_2: "~υσῃς", "~υσητον", "~υσητε";
    aorist_subj_act_3: "~υσῃ", _, "~υσωσῐ|~υσωσῐν";
    aorist_subj_mid_1: "~υσωμαι", _, "~υσωμεθᾰ";
    aorist_subj_mid_2: _, "~υσησθον", "~υσησθε";
    aorist_subj_mid_3: "~υσηται", _, "~υσωνται";
    aorist_subj_pass_1: "~υθω", _, "~υθωμεν";
    aorist_subj_pass_2: "~υθῃς", "~υθητον", "~υθητε";
    aorist_subj_pass_3: "~υθῃ", _, "~υθωσῐ|~υθωσῐν";
    aorist_opt_act_1: "~υσαιμῐ", _, "~υσαιμεν";
    aorist_opt_act_2: "~υσειᾰς|~υσαις", "~υσαιτον", "~υσαιτε";
    aorist_opt_act_3: "~υσειε|~υσειεν|~υσαι", "~υσαιτην", "~υσειᾰν|~υσαιεν";
    aorist_opt_mid_1: "~υσαιμην", _, "~υσαιμεθᾰ";
    aorist_opt_mid_2: "~υσαιο", "~υσαισθον", "~υσαισθε";
    aorist_opt_mid_3: "~υσαιτο", "~υσαισθην", "~υσαιντο";
    aorist_opt_pass_1: "~υθειην", _, "~υθεῑμεν|~υθειημεν";
    aorist_opt_pass_2: "~υθειης", "~υθεῑτον|~υθειητον", "~υθεῑτε|~υθειητε";
    aorist_opt_pass_3: "~υθειη", "~υθειτην|~υθειητην", "~υθεῑεν|~υθειησᾰν";
    aorist_imp_act_2: "~υσον", "~υσᾰτον", "~υσᾰτε";
    aorist_imp_act_3: "~υσᾰτω", "~υσᾰτων", "~υσᾰντων";
    aorist_imp_mid_2: "~υσαι", "~υσᾰσθον", "~υσᾰσθε";
    aorist_imp_mid_3: "~υσᾰσθω", "~υσᾰσθων", _;
    aorist_imp_pass_2: "~υθητῐ", _, _;
    aorist_imp_pass_3: "~υθητω", "~υθητων", "~υθεντων";
    perfect_ind_act_1: "%ε~υκᾰ", _, "%ε~υκᾰμεν";
    perfect_ind_act_2: "%ε~υκᾰς", "%ε~υκᾰτον", "%ε~υκᾰτε";
    perfect_ind_act_3: "%ε~υκε", _, "%ε~υκᾱσῐ";
    perfect_ind_mid_1: "%ε~υμαι", _, "%ε~υμεθᾰ";
    perfect_ind_mid_2: "%ε~υσαι", "%ε~υσθον", "%ε~υσθε";
    perfect_ind_mid_3: "%ε~υται", _, "%ε~υνται";
    perfect_subj_act_1: "%ε~υκω", _, "%ε~υκωμεν";
    perfect_subj_act_2: "%ε~υκῃς", "%ε~υκητον", "%ε~υκητε";
    perfect_subj_act_3: "%ε~υκῃ", _, "%ε~υκωσῐ";
    perfect_opt_act_1: "%ε~υκοιμῐ", _, "%ε~υκοιμεν";
    perfect_opt_act_2: "%ε~υκοις", "%ε~υκοιτον", "%ε~υκοιτε";
    perfect_opt_act_3: "%ε~υκοι", "%ε~υκοιτην", "%ε~υκοιεν";
    perfect_imp_act_2: _, "%ε~υκετον", "%ε~υκετε";
    perfect_imp_act_3: "%ε~υκετω", "%ε~υκετων", "%ε~υκοντων";
    perfect_imp_mid_2: "%ε~υσο", _, _;
    perfect_imp_mid_3: "%ε~υσθω", "%ε~υσθων", _;
    infinitive_pres: "~υειν", "~υεσθαι", _;
    infinitive_fut: "~υσειν", "~υσεσθαι", "~υθησεσθαι";
    infinitive_aor: "~ῡσαι", "~υσᾰσθαι", "~υθηναι";
    infinitive_perf: "%ε~υκεναι", "%ε~ῡσθαι", _;
    participle_pres: "~υων", "~υομενος", _;
    participle_fut: "~υσων", "~υσομενος", "~υθησομενος";
    participle_aor: "~υσᾱς", "~υσᾰμενος", "~υθεις";
    participle_perf: "%ε~υκως", "%ε~υμενος", _;
    ),
    // Κᾰλεω: 54 words (ψῡχω, ἀρχω, ἀμᾰρῠσσω, αἰθω, στεφω, βρεχω). Like Βᾰπτῐζω, except in 158 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("κᾰλεω", 54, "~ω", like Βᾰπτῐζω;
    stem imperfect: "~- like ῥεω";
    stem aorist_ind_pass: "~θη- like ἐχω";
    stem pluperfect_act: "~κε- like κᾰλεω";
    ind_act_1: "~ω", _, "~ομεν";
    ind_act_2: "~εις", "~ετον", "~ετε";
    ind_act_3: "~ει", _, "~ουσῐ|~ουσῐν";
    ind_mid_1: "~ομαι", _, "~ομεθᾰ";
    ind_mid_2: "~ῃ", "~εσθον", "~εσθε";
    ind_mid_3: "~εται", _, "~ονται";
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", "~ητον", "~ητε";
    subj_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    subj_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    subj_mid_2: _, "~ησθον", "~ησθε";
    subj_mid_3: "~ηται", _, "~ωνται";
    opt_act_1: "~οιμῐ", _, "~οιμεν";
    opt_act_2: "~οις", "~οιτον", "~οιτε";
    opt_act_3: "~οι", "~οιτην", "~οιεν";
    opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    opt_mid_2: "~οιο", "~οισθον", "~οισθε";
    opt_mid_3: "~οιτο", "~οισθην", "~οιντο";
    imp_act_2: "~ε", _, _;
    imp_act_3: "~ετω", "~ετων", "~οντων";
    imp_mid_2: "~ου", _, _;
    imp_mid_3: "~εσθω", "~εσθων", _;
    future_ind_pass_1: "~θησομαι", _, "~θησομεθᾰ";
    future_ind_pass_2: "~θησῃ", "~θησεσθον", "~θησεσθε";
    future_ind_pass_3: "~θησεται", _, "~θησονται";
    future_opt_pass_1: "~θησοιμην", _, "~θησοιμεθᾰ";
    future_opt_pass_2: "~θησοιο", "~θησοισθον", "~θησοισθε";
    future_opt_pass_3: "~θησοιτο", "~θησοισθην", "~θησοιντο";
    aorist_subj_pass_1: "~θω", _, "~θωμεν";
    aorist_subj_pass_2: "~θῃς", "~θητον", "~θητε";
    aorist_subj_pass_3: "~θῃ", _, "~θωσῐ";
    aorist_opt_pass_1: "~θειην", _, "~θεῑμεν";
    aorist_opt_pass_2: "~θειης", "~θεῑτον", "~θεῑτε";
    aorist_opt_pass_3: "~θειη", "~θειτην", "~θεῑεν";
    aorist_imp_pass_2: "~θητῐ", _, _;
    aorist_imp_pass_3: "~θητω", "~θητων", "~θεντων";
    perfect_ind_act_1: "ἐ~κᾰ", _, "ἐ~κᾰμεν";
    perfect_ind_act_2: "ἐ~κᾰς", "ἐ~κᾰτον", "ἐ~κᾰτε";
    perfect_ind_act_3: "ἐ~κε", _, "ἐ~κᾱσῐ";
    perfect_ind_mid_1: "ἐ~μαι", _, "ἐ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~σαι", "ἐ~σθον", "ἐ~σθε";
    perfect_ind_mid_3: "ἐ~ται", _, "ἐ~νται";
    perfect_subj_act_1: "ἐ~κω", _, "ἐ~κωμεν";
    perfect_subj_act_2: "ἐ~κῃς", "ἐ~κητον", "ἐ~κητε";
    perfect_subj_act_3: "ἐ~κῃ", _, "ἐ~κωσῐ";
    perfect_opt_act_1: "ἐ~κοιμῐ", _, "ἐ~κοιμεν";
    perfect_opt_act_2: "ἐ~κοις", "ἐ~κοιτον", "ἐ~κοιτε";
    perfect_opt_act_3: "ἐ~κοι", "ἐ~κοιτην", "ἐ~κοιεν";
    perfect_imp_act_2: _, "ἐ~κετον", "ἐ~κετε";
    perfect_imp_act_3: "ἐ~κετω", "ἐ~κετων", "ἐ~κοντων";
    perfect_imp_mid_2: "ἐ~σο", _, _;
    perfect_imp_mid_3: "ἐ~σθω", "ἐ~σθων", _;
    pluperfect_mid_1: "ἐ~μην", _, "ἐ~μεθᾰ";
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", "ἐ~σθε";
    pluperfect_mid_3: "ἐ~το", "ἐ~σθην", "ἐ~ντο";
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_pres: "~ειν", "~εσθαι", _;
    infinitive_fut: _, _, "~θησεσθαι";
    infinitive_aor: _, _, "~θηναι";
    infinitive_perf: "ἐ~κεναι", "ἐ~σθαι", _;
    infinitive_futp: _, derived, _;
    participle_pres: "~ων", "~ομενος", _;
    participle_fut: _, _, "~θησομενος";
    participle_aor: _, _, "~θεις";
    participle_perf: "ἐ~κως", "ἐ~μενος", _;
    participle_futp: _, derived, _;
    ),
    // Δοκᾰω: 54 words (ἀγᾰπᾰω, γεννᾰω, κῠβερνᾰω, γλᾰγᾰω, σῑγᾰω, κᾰκκᾰω). Like Ἐχω, except in 67 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("δοκᾰω", 54, "~ᾰω", like Ἐχω;
    stem aorist_ind: "~ησ- like ποιεω";
    ind_act_1: _, _, "~ωμεν";
    ind_act_2: "~ᾳς", "~ᾱτον", "~ᾱτε";
    ind_act_3: "~ᾳ", _, "~ωσῐ|~ωσῐν";
    ind_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    ind_mid_2: "~ᾳ", "~ᾱσθον", "~ᾱσθε";
    ind_mid_3: "~ᾱται", _, "~ωνται";
    subj_act_2: "~ᾳς", "~ᾱτον", "~ᾱτε";
    subj_act_3: "~ᾳ", _, _;
    subj_mid_2: _, "~ᾱσθον", "~ᾱσθε";
    subj_mid_3: "~ᾱται", _, _;
    opt_act_1: "~ῳην|~ῳμῐ", _, "~ῳμεν|~ῳημεν";
    opt_act_2: "~ῳης|~ῳς", "~ῳτον|~ῳητον", "~ῳτε|~ῳητε";
    opt_act_3: "~ῳη|~ῳ", "~ῳτην|~ῳητην", "~ῳεν|~ῳησᾰν";
    opt_mid_1: "~ῳμην", _, "~ῳμεθᾰ";
    opt_mid_2: "~ῳο", "~ῳσθον", "~ῳσθε";
    opt_mid_3: "~ῳτο", "~ῳσθην", "~ῳντο";
    imp_act_2: "~ᾱ", _, _;
    imp_act_3: "~ᾱτω", "~ᾱτων", "~ωντων";
    imp_mid_2: "~ω", _, _;
    imp_mid_3: "~ᾱσθω", "~ᾱσθων", _;
    imperfect_act_1: "ἐ~ων", _, "ἐ~ωμεν";
    imperfect_act_2: "ἐ~ᾱς", "ἐ~ᾱτον", "ἐ~ᾱτε";
    imperfect_act_3: "ἐ~ᾱ", "ἐ~ᾱτην", "ἐ~ων";
    imperfect_mid_1: "ἐ~ωμην", _, "ἐ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ω", "ἐ~ᾱσθον", "ἐ~ᾱσθε";
    imperfect_mid_3: "ἐ~ᾱτο", "ἐ~ᾱσθην", "ἐ~ωντο";
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    aorist_subj_act_3: _, _, "~ησωσῐ|~ησωσῐν";
    aorist_opt_act_2: "~ησειᾰς|~ησαις", _, _;
    aorist_opt_act_3: "~ησειε|~ησειεν|~ησαι", _, "~ησειᾰν|~ησαιεν";
    infinitive_pres: "~ᾱν", "~ᾱσθαι", _;
    participle_pres: _, "~ωμενος", _;
    ),
    // Δῐκεω: 49 words (ἀγᾰθοποιεω, ἀγνοεω, ἀντλεω, ἀσκεω, οἰκεω, ἀλεω). Like Ποιεω, except in 67 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 sg, perfect_ind_act_2 du, perfect_ind_act_2 pl, perfect_ind_act_3 sg, ….
    conjugation!("δῐκεω", 49, "~εω", like Ποιεω;
    stem pluperfect_act: "~ηκ- like δῐκεω";
    perfect_ind_act_1: "ἐ~ηκᾰ", _, "ἐ~ηκᾰμεν";
    perfect_ind_act_2: "ἐ~ηκᾰς", "ἐ~ηκᾰτον", "ἐ~ηκᾰτε";
    perfect_ind_act_3: "ἐ~ηκε|ἐ~ηκεν", _, "ἐ~ηκᾱσῐ|ἐ~ηκᾱσῐν";
    perfect_ind_mid_1: "ἐ~ημαι", _, "ἐ~ημεθᾰ";
    perfect_ind_mid_2: "ἐ~ησαι", "ἐ~ησθον", "ἐ~ησθε";
    perfect_ind_mid_3: "ἐ~ηται", _, "ἐ~ηνται";
    perfect_subj_act_1: "ἐ~ηκω", _, "ἐ~ηκωμεν";
    perfect_subj_act_2: "ἐ~ηκῃς", "ἐ~ηκητον", "ἐ~ηκητε";
    perfect_subj_act_3: "ἐ~ηκῃ", _, "ἐ~ηκωσῐ|ἐ~ηκωσῐν";
    perfect_opt_act_1: "ἐ~ηκοιμῐ|ἐ~ηκοιην", _, "ἐ~ηκοιμεν";
    perfect_opt_act_2: "ἐ~ηκοις|ἐ~ηκοιης", "ἐ~ηκοιτον", "ἐ~ηκοιτε";
    perfect_opt_act_3: "ἐ~ηκοι|ἐ~ηκοιη", "ἐ~ηκοιτην", "ἐ~ηκοιεν";
    perfect_imp_act_2: "ἐ~ηκε", "ἐ~ηκετον", "ἐ~ηκετε";
    perfect_imp_act_3: "ἐ~ηκετω", "ἐ~ηκετων", "ἐ~ηκοντων";
    perfect_imp_mid_2: "ἐ~ησο", _, _;
    perfect_imp_mid_3: "ἐ~ησθω", "ἐ~ησθων", _;
    pluperfect_mid_1: "ἐ~ημην", _, "ἐ~ημεθᾰ";
    pluperfect_mid_2: "ἐ~ησο", "ἐ~ησθον", "ἐ~ησθε";
    pluperfect_mid_3: "ἐ~ητο", "ἐ~ησθην", "ἐ~ηντο";
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_perf: "ἐ~ηκεναι", "ἐ~ησθαι", _;
    infinitive_futp: _, derived, _;
    participle_perf: "ἐ~ηκως", "ἐ~ημενος", _;
    participle_futp: _, derived, _;
    ),
    // Οἰομαι: 43 words (γῐγνομαι, δεχομαι, μαινομαι, ἑπομαι, περδομαι, εὐχομαι). Like Ἐχω, except in 120 cells: ind_act_1 sg, subj_mid_2 sg, subj_pass_2 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, ….
    conjugation!("οἰομαι", 43, "~ομαι", like Ἐχω as ACT_AS_PASS;
    ind_act_1: "~ομαι", _, _;
    subj_mid_2: "~ῃ", _, _;
    subj_pass_2: "~ῃ", _, _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: derived, _, derived;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_ind_mid_1: derived, _, derived;
    aorist_ind_mid_2: derived, derived, derived;
    aorist_ind_mid_3: derived, derived, derived;
    aorist_subj_act_1: derived, _, derived;
    aorist_subj_act_2: derived, derived, derived;
    aorist_subj_act_3: derived, _, derived;
    aorist_subj_mid_1: derived, _, derived;
    aorist_subj_mid_2: _, derived, derived;
    aorist_subj_mid_3: derived, _, derived;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_mid_1: derived, _, derived;
    aorist_opt_mid_2: derived, derived, derived;
    aorist_opt_mid_3: derived, derived, derived;
    aorist_imp_act_2: derived, _, _;
    aorist_imp_act_3: derived, derived, derived;
    aorist_imp_mid_2: derived, derived, derived;
    aorist_imp_mid_3: derived, derived, _;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_fut: derived, _, derived;
    infinitive_aor: derived, derived, _;
    infinitive_futp: _, derived, _;
    participle_fut: derived, _, derived;
    participle_aor: derived, derived, _;
    participle_futp: _, derived, _;
    ),
    // Τᾰσσω: 32 words (ἀϊσσω, πλησσω, τᾰσσω, μᾰσσω, φῠλᾰσσω, πτῠσσω). Like Γῡροω, except in 235 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("τᾰσσω", 32, "~σσω", like Γῡροω;
    stem aorist_ind: "~ξ- like ποιεω";
    stem aorist_ind_pass: "~χθη- like ἐχω";
    stem pluperfect_act: "~χε- like ἐχω";
    ind_act_1: "~σσω", _, "~σσομεν";
    ind_act_2: "~σσεις", "~σσετον", "~σσετε";
    ind_act_3: "~σσει", _, "~σσουσῐ|~σσουσῐν";
    ind_mid_1: "~σσομαι", _, "~σσομεθᾰ";
    ind_mid_2: "~σσῃ|~σσει", "~σσεσθον", "~σσεσθε";
    ind_mid_3: "~σσεται", _, "~σσονται";
    subj_act_1: "~σσω", _, "~σσωμεν";
    subj_act_2: "~σσῃς", "~σσητον", "~σσητε";
    subj_act_3: "~σσῃ", _, "~σσωσῐ|~σσωσῐν";
    subj_mid_1: "~σσωμαι", _, "~σσωμεθᾰ";
    subj_mid_2: _, "~σσησθον", "~σσησθε";
    subj_mid_3: "~σσηται", _, "~σσωνται";
    opt_act_1: "~σσοιμῐ", _, "~σσοιμεν";
    opt_act_2: "~σσοις", "~σσοιτον", "~σσοιτε";
    opt_act_3: "~σσοι", "~σσοιτην", "~σσοιεν";
    opt_mid_1: "~σσοιμην", _, "~σσοιμεθᾰ";
    opt_mid_2: "~σσοιο", "~σσοισθον", "~σσοισθε";
    opt_mid_3: "~σσοιτο", "~σσοισθην", "~σσοιντο";
    imp_act_2: "~σσε", _, _;
    imp_act_3: "~σσετω", "~σσετων", "~σσοντων";
    imp_mid_2: "~σσου", _, _;
    imp_mid_3: "~σσεσθω", "~σσεσθων", _;
    imperfect_act_1: "ἐ~σσον", _, "ἐ~σσομεν";
    imperfect_act_2: "ἐ~σσες", "ἐ~σσετον", "ἐ~σσετε";
    imperfect_act_3: "ἐ~σσε|ἐ~σσεν", "ἐ~σσετην", "ἐ~σσον";
    imperfect_mid_1: "ἐ~σσομην", _, "ἐ~σσομεθᾰ";
    imperfect_mid_2: "ἐ~σσου", "ἐ~σσεσθον", "ἐ~σσεσθε";
    imperfect_mid_3: "ἐ~σσετο", "ἐ~σσεσθην", "ἐ~σσοντο";
    future_ind_act_1: "~ξω", _, "~ξομεν";
    future_ind_act_2: "~ξεις", "~ξετον", "~ξετε";
    future_ind_act_3: "~ξει", _, "~ξουσῐ|~ξουσῐν";
    future_ind_mid_1: "~ξομαι", _, "~ξομεθᾰ";
    future_ind_mid_2: "~ξῃ|~ξει", "~ξεσθον", "~ξεσθε";
    future_ind_mid_3: "~ξεται", _, "~ξονται";
    future_ind_pass_1: "~χθησομαι", _, "~χθησομεθᾰ";
    future_ind_pass_2: "~χθησῃ", "~χθησεσθον", "~χθησεσθε";
    future_ind_pass_3: "~χθησεται", _, "~χθησονται";
    future_opt_act_1: "~ξοιμῐ", _, "~ξοιμεν";
    future_opt_act_2: "~ξοις", "~ξοιτον", "~ξοιτε";
    future_opt_act_3: "~ξοι", "~ξοιτην", "~ξοιεν";
    future_opt_mid_1: "~ξοιμην", _, "~ξοιμεθᾰ";
    future_opt_mid_2: "~ξοιο", "~ξοισθον", "~ξοισθε";
    future_opt_mid_3: "~ξοιτο", "~ξοισθην", "~ξοιντο";
    future_opt_pass_1: "~χθησοιμην", _, "~χθησοιμεθᾰ";
    future_opt_pass_2: "~χθησοιο", "~χθησοισθον", "~χθησοισθε";
    future_opt_pass_3: "~χθησοιτο", "~χθησοισθην", "~χθησοιντο";
    aorist_subj_act_1: "~ξω", _, "~ξωμεν";
    aorist_subj_act_2: "~ξῃς", "~ξητον", "~ξητε";
    aorist_subj_act_3: "~ξῃ", _, "~ξωσῐ|~ξωσῐν";
    aorist_subj_mid_1: "~ξωμαι", _, "~ξωμεθᾰ";
    aorist_subj_mid_2: _, "~ξησθον", "~ξησθε";
    aorist_subj_mid_3: "~ξηται", _, "~ξωνται";
    aorist_subj_pass_1: "~χθω", _, "~χθωμεν";
    aorist_subj_pass_2: "~χθῃς", "~χθητον", "~χθητε";
    aorist_subj_pass_3: "~χθῃ", _, "~χθωσῐ";
    aorist_opt_act_1: "~ξαιμῐ", _, "~ξαιμεν";
    aorist_opt_act_2: "~ξειᾰς|~ξαις", "~ξαιτον", "~ξαιτε";
    aorist_opt_act_3: "~ξειε|~ξειεν|~ξαι", "~ξαιτην", "~ξειᾰν|~ξαιεν";
    aorist_opt_mid_1: "~ξαιμην", _, "~ξαιμεθᾰ";
    aorist_opt_mid_2: "~ξαιο", "~ξαισθον", "~ξαισθε";
    aorist_opt_mid_3: "~ξαιτο", "~ξαισθην", "~ξαιντο";
    aorist_opt_pass_1: "~χθειην", _, "~χθεῑμεν";
    aorist_opt_pass_2: "~χθειης", "~χθεῑτον", "~χθεῑτε";
    aorist_opt_pass_3: "~χθειη", "~χθειτην", "~χθεῑεν";
    aorist_imp_act_2: "~ξον", "~ξᾰτον", "~ξᾰτε";
    aorist_imp_act_3: "~ξᾰτω", "~ξᾰτων", "~ξᾰντων";
    aorist_imp_mid_2: "~ξαι", "~ξᾰσθον", "~ξᾰσθε";
    aorist_imp_mid_3: "~ξᾰσθω", "~ξᾰσθων", _;
    aorist_imp_pass_2: "~χθητῐ", _, _;
    aorist_imp_pass_3: "~χθητω", "~χθητων", "~χθεντων";
    perfect_ind_act_1: "%ε~χᾰ", _, "%ε~χᾰμεν";
    perfect_ind_act_2: "%ε~χᾰς", "%ε~χᾰτον", "%ε~χᾰτε";
    perfect_ind_act_3: "%ε~χε", _, "%ε~χᾱσῐ";
    perfect_ind_mid_1: "%ε~γμαι", _, "%ε~γμεθᾰ";
    perfect_ind_mid_2: "%ε~ξαι", "%ε~χθον", "%ε~χθε";
    perfect_ind_mid_3: "%ε~κται", _, "%ε~γᾰται";
    perfect_subj_act_1: "%ε~χω", _, "%ε~χωμεν";
    perfect_subj_act_2: "%ε~χῃς", "%ε~χητον", "%ε~χητε";
    perfect_subj_act_3: "%ε~χῃ", _, "%ε~χωσῐ";
    perfect_opt_act_1: "%ε~χοιμῐ", _, "%ε~χοιμεν";
    perfect_opt_act_2: "%ε~χοις", "%ε~χοιτον", "%ε~χοιτε";
    perfect_opt_act_3: "%ε~χοι", "%ε~χοιτην", "%ε~χοιεν";
    perfect_imp_act_2: _, "%ε~χετον", "%ε~χετε";
    perfect_imp_act_3: "%ε~χετω", "%ε~χετων", "%ε~χοντων";
    perfect_imp_mid_2: "%ε~ξο", _, _;
    perfect_imp_mid_3: "%ε~χθω", "%ε~χθων", _;
    pluperfect_mid_1: "ἐ%ε~γμην", _, "ἐ%ε~γμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ξο", "ἐ%ε~χθον", "ἐ%ε~χθε";
    pluperfect_mid_3: "ἐ%ε~κτο", "ἐ%ε~χθην", "ἐ%ε~γᾰτο";
    infinitive_pres: "~σσειν", "~σσεσθαι", _;
    infinitive_fut: "~ξειν", "~ξεσθαι", "~χθησεσθαι";
    infinitive_aor: "~ξαι", "~ξᾰσθαι", "~χθηναι";
    infinitive_perf: "%ε~χεναι", "%ε~χθαι", _;
    participle_pres: "~σσων", "~σσομενος", _;
    participle_fut: "~ξων", "~ξομενος", "~χθησομενος";
    participle_aor: "~ξᾱς", "~ξᾰμενος", "~χθεις";
    participle_perf: "%ε~χως", "%ε~γμενος", _;
    ),
    // Ληγω: 30 words (ἀγω, λεγω, φευγω, τρωγω, δῐᾰλεγω, φλεγω). Like Τᾰσσω, except in 88 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ληγω", 30, "~γω", like Τᾰσσω as PERFECT_AS_FUTURE;
    stem imperfect: "~γ- like ῥεω";
    stem aorist_ind: "~ξ- like ἐχω";
    ind_act_1: "~γω", _, "~γομεν";
    ind_act_2: "~γεις", "~γετον", "~γετε";
    ind_act_3: "~γει", _, "~γουσῐ";
    ind_mid_1: "~γομαι", _, "~γομεθᾰ";
    ind_mid_2: "~γῃ", "~γεσθον", "~γεσθε";
    ind_mid_3: "~γεται", _, "~γονται";
    subj_act_1: "~γω", _, "~γωμεν";
    subj_act_2: "~γῃς", "~γητον", "~γητε";
    subj_act_3: "~γῃ", _, "~γωσῐ";
    subj_mid_1: "~γωμαι", _, "~γωμεθᾰ";
    subj_mid_2: _, "~γησθον", "~γησθε";
    subj_mid_3: "~γηται", _, "~γωνται";
    opt_act_1: "~γοιμῐ", _, "~γοιμεν";
    opt_act_2: "~γοις", "~γοιτον", "~γοιτε";
    opt_act_3: "~γοι", "~γοιτην", "~γοιεν";
    opt_mid_1: "~γοιμην", _, "~γοιμεθᾰ";
    opt_mid_2: "~γοιο", "~γοισθον", "~γοισθε";
    opt_mid_3: "~γοιτο", "~γοισθην", "~γοιντο";
    imp_act_2: "~γε", _, _;
    imp_act_3: "~γετω", "~γετων", "~γοντων";
    imp_mid_2: "~γου", _, _;
    imp_mid_3: "~γεσθω", "~γεσθων", _;
    future_ind_act_3: _, _, "~ξουσῐ";
    future_ind_mid_2: "~ξῃ", _, _;
    aorist_subj_act_3: _, _, "~ξωσῐ";
    aorist_opt_act_2: "~ξειᾰς", _, _;
    aorist_opt_act_3: "~ξειε", _, "~ξειᾰν";
    perfect_ind_mid_1: "%ε~γμαι", _, "%ε~γμεθᾰ";
    perfect_ind_mid_2: "%ε~ξαι", "%ε~χθον", "%ε~χθε";
    perfect_ind_mid_3: "%ε~κται", _, "%ε~γᾰται";
    perfect_imp_mid_2: "%ε~ξο", _, _;
    perfect_imp_mid_3: "%ε~χθω", "%ε~χθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_pres: "~γειν", "~γεσθαι", _;
    infinitive_perf: derived, _, _;
    participle_pres: "~γων", "~γομενος", _;
    participle_perf: derived, _, _;
    ),
    // Ψηφῐζω: 28 words (γνωρῐζω, ψηφῐζω, ὁρῐζω, ἁγνῐζω, σχηματῐζω, ἀτενῐζω). Like Χρημᾰτῐζω, except in 50 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 sg, perfect_ind_act_2 du, perfect_ind_act_2 pl, perfect_ind_act_3 sg, ….
    conjugation!("ψηφῐζω", 28, "~ζω", like Χρημᾰτῐζω;
    stem pluperfect_act: "~κ- like δῐκεω";
    perfect_ind_act_1: "ἐ~κᾰ", _, "ἐ~κᾰμεν";
    perfect_ind_act_2: "ἐ~κᾰς", "ἐ~κᾰτον", "ἐ~κᾰτε";
    perfect_ind_act_3: "ἐ~κε|ἐ~κεν", _, "ἐ~κᾱσῐ|ἐ~κᾱσῐν";
    perfect_ind_mid_1: "ἐ~σμαι", _, "ἐ~σμεθᾰ";
    perfect_ind_mid_2: "ἐ~σαι", "ἐ~σθον", "ἐ~σθε";
    perfect_ind_mid_3: "ἐ~σται", _, "ἐ~δᾰται";
    perfect_subj_act_1: "ἐ~κω", _, "ἐ~κωμεν";
    perfect_subj_act_2: "ἐ~κῃς", "ἐ~κητον", "ἐ~κητε";
    perfect_subj_act_3: "ἐ~κῃ", _, "ἐ~κωσῐ|ἐ~κωσῐν";
    perfect_opt_act_1: "ἐ~κοιμῐ|ἐ~κοιην", _, "ἐ~κοιμεν";
    perfect_opt_act_2: "ἐ~κοις|ἐ~κοιης", "ἐ~κοιτον", "ἐ~κοιτε";
    perfect_opt_act_3: "ἐ~κοι|ἐ~κοιη", "ἐ~κοιτην", "ἐ~κοιεν";
    perfect_imp_act_2: "ἐ~κε", "ἐ~κετον", "ἐ~κετε";
    perfect_imp_act_3: "ἐ~κετω", "ἐ~κετων", "ἐ~κοντων";
    perfect_imp_mid_2: "ἐ~σο", _, _;
    perfect_imp_mid_3: "ἐ~σθω", "ἐ~σθων", _;
    pluperfect_mid_1: "ἐ~σμην", _, "ἐ~σμεθᾰ";
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", "ἐ~σθε";
    pluperfect_mid_3: "ἐ~στο", "ἐ~σθην", "ἐ~δᾰτο";
    infinitive_perf: "ἐ~κεναι", "ἐ~σθαι", _;
    participle_perf: "ἐ~κως", "ἐ~σμενος", _;
    ),
    // Σῠνᾰρπᾰζω: 24 words (βᾰζω, ἁρπᾰζω, ἐλελῐζω, κρᾱζω, στῐζω, σφυζω). Like Ληγω, except in 95 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("σῠνᾰρπᾰζω", 24, "~ζω", like Ληγω;
    stem imperfect: "~ζ- like ῥεω";
    ind_act_1: "~ζω", _, "~ζομεν";
    ind_act_2: "~ζεις", "~ζετον", "~ζετε";
    ind_act_3: "~ζει", _, "~ζουσῐ";
    ind_mid_1: "~ζομαι", _, "~ζομεθᾰ";
    ind_mid_2: "~ζῃ", "~ζεσθον", "~ζεσθε";
    ind_mid_3: "~ζεται", _, "~ζονται";
    subj_act_1: "~ζω", _, "~ζωμεν";
    subj_act_2: "~ζῃς", "~ζητον", "~ζητε";
    subj_act_3: "~ζῃ", _, "~ζωσῐ";
    subj_mid_1: "~ζωμαι", _, "~ζωμεθᾰ";
    subj_mid_2: _, "~ζησθον", "~ζησθε";
    subj_mid_3: "~ζηται", _, "~ζωνται";
    opt_act_1: "~ζοιμῐ", _, "~ζοιμεν";
    opt_act_2: "~ζοις", "~ζοιτον", "~ζοιτε";
    opt_act_3: "~ζοι", "~ζοιτην", "~ζοιεν";
    opt_mid_1: "~ζοιμην", _, "~ζοιμεθᾰ";
    opt_mid_2: "~ζοιο", "~ζοισθον", "~ζοισθε";
    opt_mid_3: "~ζοιτο", "~ζοισθην", "~ζοιντο";
    imp_act_2: "~ζε", _, _;
    imp_act_3: "~ζετω", "~ζετων", "~ζοντων";
    imp_mid_2: "~ζου", _, _;
    imp_mid_3: "~ζεσθω", "~ζεσθων", _;
    perfect_ind_act_1: "ἐ~χᾰ", _, "ἐ~χᾰμεν";
    perfect_ind_act_2: "ἐ~χᾰς", "ἐ~χᾰτον", "ἐ~χᾰτε";
    perfect_ind_act_3: "ἐ~χε", _, "ἐ~χᾱσῐ";
    perfect_ind_mid_1: "ἐ~γμαι", _, "ἐ~γμεθᾰ";
    perfect_ind_mid_2: "ἐ~ξαι", "ἐ~χθον", "ἐ~χθε";
    perfect_ind_mid_3: "ἐ~κται", _, "ἐ~γᾰται";
    perfect_subj_act_1: "ἐ~χω", _, "ἐ~χωμεν";
    perfect_subj_act_2: "ἐ~χῃς", "ἐ~χητον", "ἐ~χητε";
    perfect_subj_act_3: "ἐ~χῃ", _, "ἐ~χωσῐ";
    perfect_opt_act_1: "ἐ~χοιμῐ", _, "ἐ~χοιμεν";
    perfect_opt_act_2: "ἐ~χοις", "ἐ~χοιτον", "ἐ~χοιτε";
    perfect_opt_act_3: "ἐ~χοι", "ἐ~χοιτην", "ἐ~χοιεν";
    perfect_imp_act_2: _, "ἐ~χετον", "ἐ~χετε";
    perfect_imp_act_3: "ἐ~χετω", "ἐ~χετων", "ἐ~χοντων";
    perfect_imp_mid_2: "ἐ~ξο", _, _;
    perfect_imp_mid_3: "ἐ~χθω", "ἐ~χθων", _;
    infinitive_pres: "~ζειν", "~ζεσθαι", _;
    infinitive_perf: "ἐ~χεναι", "ἐ~χθαι", _;
    participle_pres: "~ζων", "~ζομενος", _;
    participle_perf: "ἐ~χως", "ἐ~γμενος", _;
    ),
    // Ἐλπω: 22 words (ὑω, εὑω, ἐλπω, ἰδῑω, ἐρδω, περικλειω). Like Κᾰλεω, except in 84 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("ἐλπω", 22, "~ω", like Κᾰλεω as PERFECT_AS_FUTURE;
    stem aorist_ind: "~σ- like ἐλπω";
    stem aorist_ind_pass: "~σθη- like ἐλπω";
    imperfect_act_1: "~ον", _, "~ομεν";
    imperfect_act_2: "~ες", "~ετον", "~ετε";
    imperfect_act_3: "~ε|~εν", "~ετην", "~ον";
    imperfect_mid_1: "~ομην", _, "~ομεθᾰ";
    imperfect_mid_2: "~ου", "~εσθον", "~εσθε";
    imperfect_mid_3: "~ετο", "~εσθην", "~οντο";
    future_ind_pass_1: "~σθησομαι", _, "~σθησομεθᾰ";
    future_ind_pass_2: "~σθησῃ", "~σθησεσθον", "~σθησεσθε";
    future_ind_pass_3: "~σθησεται", _, "~σθησονται";
    future_opt_pass_1: "~σθησοιμην", _, "~σθησοιμεθᾰ";
    future_opt_pass_2: "~σθησοιο", "~σθησοισθον", "~σθησοισθε";
    future_opt_pass_3: "~σθησοιτο", "~σθησοισθην", "~σθησοιντο";
    aorist_subj_pass_1: "~σθω", _, "~σθωμεν";
    aorist_subj_pass_2: "~σθῃς", "~σθητον", "~σθητε";
    aorist_subj_pass_3: "~σθῃ", _, "~σθωσῐ";
    aorist_opt_pass_1: "~σθειην", _, "~σθεῑμεν";
    aorist_opt_pass_2: "~σθειης", "~σθεῑτον", "~σθεῑτε";
    aorist_opt_pass_3: "~σθειη", "~σθειτην", "~σθεῑεν";
    aorist_imp_pass_2: "~σθητῐ", _, _;
    aorist_imp_pass_3: "~σθητω", "~σθητων", "~σθεντων";
    perfect_ind_mid_1: "~σμαι", _, "~σμεθᾰ";
    perfect_ind_mid_2: "~σαι", "~σθον", "~σθε";
    perfect_ind_mid_3: "~σται", _, "~σᾰται";
    perfect_imp_mid_2: "~σο", _, _;
    perfect_imp_mid_3: "~σθω", "~σθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_fut: _, _, "~σθησεσθαι";
    infinitive_aor: _, _, "~σθηναι";
    infinitive_perf: derived, "~σθαι", _;
    participle_fut: _, _, "~σθησομενος";
    participle_aor: _, _, "~σθεις";
    participle_perf: derived, "~σμενος", _;
    ),
    // Εὐλογεω: 22 words (εὐλογεω, ἠθεω, κᾰτηχεω, ἠχεω, ὀλιγαρχεω, μοναρχεω). Like Δῐκεω, except in 66 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("εὐλογεω", 22, "~εω", like Δῐκεω;
    stem aorist_ind: "~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ηθη- like ἐλπω";
    stem pluperfect_act: "~ηκ- like εὐλογεω";
    imperfect_act_1: "~ουν", _, "~οῡμεν";
    imperfect_act_2: "~εις", "~εῑτον", "~εῑτε";
    imperfect_act_3: "~ει", "~ειτην", "~ουν";
    imperfect_mid_1: "~ουμην", _, "~ουμεθᾰ";
    imperfect_mid_2: "~οῡ", "~εῑσθον", "~εῑσθε";
    imperfect_mid_3: "~εῑτο", "~εισθην", "~οῡντο";
    perfect_ind_act_1: "~ηκᾰ", _, "~ηκᾰμεν";
    perfect_ind_act_2: "~ηκᾰς", "~ηκᾰτον", "~ηκᾰτε";
    perfect_ind_act_3: "~ηκε|~ηκεν", _, "~ηκᾱσῐ|~ηκᾱσῐν";
    perfect_ind_mid_1: "~ημαι", _, "~ημεθᾰ";
    perfect_ind_mid_2: "~ησαι", "~ησθον", "~ησθε";
    perfect_ind_mid_3: "~ηται", _, "~ηνται";
    perfect_subj_act_1: "~ηκω", _, "~ηκωμεν";
    perfect_subj_act_2: "~ηκῃς", "~ηκητον", "~ηκητε";
    perfect_subj_act_3: "~ηκῃ", _, "~ηκωσῐ|~ηκωσῐν";
    perfect_opt_act_1: "~ηκοιμῐ|~ηκοιην", _, "~ηκοιμεν";
    perfect_opt_act_2: "~ηκοις|~ηκοιης", "~ηκοιτον", "~ηκοιτε";
    perfect_opt_act_3: "~ηκοι|~ηκοιη", "~ηκοιτην", "~ηκοιεν";
    perfect_imp_act_2: "~ηκε", "~ηκετον", "~ηκετε";
    perfect_imp_act_3: "~ηκετω", "~ηκετων", "~ηκοντων";
    perfect_imp_mid_2: "~ησο", _, _;
    perfect_imp_mid_3: "~ησθω", "~ησθων", _;
    pluperfect_mid_1: "~ημην", _, "~ημεθᾰ";
    pluperfect_mid_2: "~ησο", "~ησθον", "~ησθε";
    pluperfect_mid_3: "~ητο", "~ησθην", "~ηντο";
    infinitive_perf: "~ηκεναι", "~ησθαι", _;
    participle_perf: "~ηκως", "~ημενος", _;
    ),
    // Κτῐζω: 21 words (σχῐζω, ὀνομᾰζω, σχεδῐᾰζω, ἁρμοζω, κτῐζω, σπιζω). Like Ψηφῐζω, except in 34 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("κτῐζω", 21, "~ζω", like Ψηφῐζω;
    future_ind_act_1: "~σω", _, "~σομεν";
    future_ind_act_2: "~σεις", "~σετον", "~σετε";
    future_ind_act_3: "~σει", _, "~σουσῐ|~σουσῐν";
    future_ind_mid_1: "~σομαι", _, "~σομεθᾰ";
    future_ind_mid_2: "~σῃ|~σει", "~σεσθον", "~σεσθε";
    future_ind_mid_3: "~σεται", _, "~σονται";
    future_opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    future_opt_act_2: "~σοις", "~σοιτον", "~σοιτε";
    future_opt_act_3: "~σοι", "~σοιτην", "~σοιεν";
    future_opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    future_opt_mid_2: "~σοιο", "~σοισθον", "~σοισθε";
    future_opt_mid_3: "~σοιτο", "~σοισθην", "~σοιντο";
    infinitive_fut: "~σειν", "~σεσθαι", _;
    participle_fut: "~σων", "~σομενος", _;
    ),
    // Ἁπτω: 21 words (κρῠπτω, ἁπτω, κῠπτω, θᾰπτω, ἀποκαλυπτω, σῠναπτω). Like Βᾰπτῐζω, except in 229 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἁπτω", 21, "~πτω", like Βᾰπτῐζω;
    stem imperfect: "~πτ- like ῥεω";
    stem aorist_ind: "~ψ- like ἐχω";
    stem aorist_ind_pass: "~φθη- like ἐχω";
    stem pluperfect_act: "~φε- like ἐχω";
    ind_act_1: "~πτω", _, "~πτομεν";
    ind_act_2: "~πτεις", "~πτετον", "~πτετε";
    ind_act_3: "~πτει", _, "~πτουσῐ";
    ind_mid_1: "~πτομαι", _, "~πτομεθᾰ";
    ind_mid_2: "~πτῃ", "~πτεσθον", "~πτεσθε";
    ind_mid_3: "~πτεται", _, "~πτονται";
    subj_act_1: "~πτω", _, "~πτωμεν";
    subj_act_2: "~πτῃς", "~πτητον", "~πτητε";
    subj_act_3: "~πτῃ", _, "~πτωσῐ";
    subj_mid_1: "~πτωμαι", _, "~πτωμεθᾰ";
    subj_mid_2: _, "~πτησθον", "~πτησθε";
    subj_mid_3: "~πτηται", _, "~πτωνται";
    opt_act_1: "~πτοιμῐ", _, "~πτοιμεν";
    opt_act_2: "~πτοις", "~πτοιτον", "~πτοιτε";
    opt_act_3: "~πτοι", "~πτοιτην", "~πτοιεν";
    opt_mid_1: "~πτοιμην", _, "~πτοιμεθᾰ";
    opt_mid_2: "~πτοιο", "~πτοισθον", "~πτοισθε";
    opt_mid_3: "~πτοιτο", "~πτοισθην", "~πτοιντο";
    imp_act_2: "~πτε", _, _;
    imp_act_3: "~πτετω", "~πτετων", "~πτοντων";
    imp_mid_2: "~πτου", _, _;
    imp_mid_3: "~πτεσθω", "~πτεσθων", _;
    future_ind_act_1: "~ψω", _, "~ψομεν";
    future_ind_act_2: "~ψεις", "~ψετον", "~ψετε";
    future_ind_act_3: "~ψει", _, "~ψουσῐ";
    future_ind_mid_1: "~ψομαι", _, "~ψομεθᾰ";
    future_ind_mid_2: "~ψῃ", "~ψεσθον", "~ψεσθε";
    future_ind_mid_3: "~ψεται", _, "~ψονται";
    future_ind_pass_1: "~φθησομαι", _, "~φθησομεθᾰ";
    future_ind_pass_2: "~φθησῃ", "~φθησεσθον", "~φθησεσθε";
    future_ind_pass_3: "~φθησεται", _, "~φθησονται";
    future_opt_act_1: "~ψοιμῐ", _, "~ψοιμεν";
    future_opt_act_2: "~ψοις", "~ψοιτον", "~ψοιτε";
    future_opt_act_3: "~ψοι", "~ψοιτην", "~ψοιεν";
    future_opt_mid_1: "~ψοιμην", _, "~ψοιμεθᾰ";
    future_opt_mid_2: "~ψοιο", "~ψοισθον", "~ψοισθε";
    future_opt_mid_3: "~ψοιτο", "~ψοισθην", "~ψοιντο";
    future_opt_pass_1: "~φθησοιμην", _, "~φθησοιμεθᾰ";
    future_opt_pass_2: "~φθησοιο", "~φθησοισθον", "~φθησοισθε";
    future_opt_pass_3: "~φθησοιτο", "~φθησοισθην", "~φθησοιντο";
    aorist_subj_act_1: "~ψω", _, "~ψωμεν";
    aorist_subj_act_2: "~ψῃς", "~ψητον", "~ψητε";
    aorist_subj_act_3: "~ψῃ", _, "~ψωσῐ";
    aorist_subj_mid_1: "~ψωμαι", _, "~ψωμεθᾰ";
    aorist_subj_mid_2: _, "~ψησθον", "~ψησθε";
    aorist_subj_mid_3: "~ψηται", _, "~ψωνται";
    aorist_subj_pass_1: "~φθω", _, "~φθωμεν";
    aorist_subj_pass_2: "~φθῃς", "~φθητον", "~φθητε";
    aorist_subj_pass_3: "~φθῃ", _, "~φθωσῐ";
    aorist_opt_act_1: "~ψαιμῐ", _, "~ψαιμεν";
    aorist_opt_act_2: "~ψειᾰς", "~ψαιτον", "~ψαιτε";
    aorist_opt_act_3: "~ψειε", "~ψαιτην", "~ψειᾰν";
    aorist_opt_mid_1: "~ψαιμην", _, "~ψαιμεθᾰ";
    aorist_opt_mid_2: "~ψαιο", "~ψαισθον", "~ψαισθε";
    aorist_opt_mid_3: "~ψαιτο", "~ψαισθην", "~ψαιντο";
    aorist_opt_pass_1: "~φθειην", _, "~φθεῑμεν";
    aorist_opt_pass_2: "~φθειης", "~φθεῑτον", "~φθεῑτε";
    aorist_opt_pass_3: "~φθειη", "~φθειτην", "~φθεῑεν";
    aorist_imp_act_2: "~ψον", "~ψᾰτον", "~ψᾰτε";
    aorist_imp_act_3: "~ψᾰτω", "~ψᾰτων", "~ψᾰντων";
    aorist_imp_mid_2: "~ψαι", "~ψᾰσθον", "~ψᾰσθε";
    aorist_imp_mid_3: "~ψᾰσθω", "~ψᾰσθων", _;
    aorist_imp_pass_2: "~φθητῐ", _, _;
    aorist_imp_pass_3: "~φθητω", "~φθητων", "~φθεντων";
    perfect_ind_act_1: "%ε~φᾰ", _, "%ε~φᾰμεν";
    perfect_ind_act_2: "%ε~φᾰς", "%ε~φᾰτον", "%ε~φᾰτε";
    perfect_ind_act_3: "%ε~φε", _, "%ε~φᾱσῐ";
    perfect_ind_mid_1: "%ε~μαι", _, "%ε~μεθᾰ";
    perfect_ind_mid_3: "%ε~ται", _, "%ε~νται";
    perfect_subj_act_1: "%ε~φω", _, "%ε~φωμεν";
    perfect_subj_act_2: "%ε~φῃς", "%ε~φητον", "%ε~φητε";
    perfect_subj_act_3: "%ε~φῃ", _, "%ε~φωσῐ";
    perfect_opt_act_1: "%ε~φοιμῐ", _, "%ε~φοιμεν";
    perfect_opt_act_2: "%ε~φοις", "%ε~φοιτον", "%ε~φοιτε";
    perfect_opt_act_3: "%ε~φοι", "%ε~φοιτην", "%ε~φοιεν";
    perfect_imp_act_2: _, "%ε~φετον", "%ε~φετε";
    perfect_imp_act_3: "%ε~φετω", "%ε~φετων", "%ε~φοντων";
    pluperfect_mid_1: "%ε~μην", _, "%ε~μεθᾰ";
    pluperfect_mid_2: "%ε~σο", "%ε~σθον", "%ε~σθε";
    pluperfect_mid_3: "%ε~το", "%ε~σθην", "%ε~ντο";
    future_perfect_ind_mid_1: "%ε~φομαι", _, "%ε~φομεθᾰ";
    future_perfect_ind_mid_2: "%ε~φῃ", "%ε~φεσθον", "%ε~φεσθε";
    future_perfect_ind_mid_3: "%ε~φεται", _, "%ε~φονται";
    future_perfect_opt_mid_1: "%ε~φοιμην", _, "%ε~φοιμεθᾰ";
    future_perfect_opt_mid_2: "%ε~φοιο", "%ε~φοισθον", "%ε~φοισθε";
    future_perfect_opt_mid_3: "%ε~φοιτο", "%ε~φοισθην", "%ε~φοιντο";
    infinitive_pres: "~πτειν", "~πτεσθαι", _;
    infinitive_fut: "~ψειν", "~ψεσθαι", "~φθησεσθαι";
    infinitive_aor: "~ψαι", "~ψᾰσθαι", "~φθηναι";
    infinitive_perf: "%ε~φεναι", _, _;
    infinitive_futp: _, "%ε~φεσθαι", _;
    participle_pres: "~πτων", "~πτομενος", _;
    participle_fut: "~ψων", "~ψομενος", "~φθησομενος";
    participle_aor: "~ψᾱς", "~ψᾰμενος", "~φθεις";
    participle_perf: "%ε~φως", "%ε~μενος", _;
    participle_futp: _, "%ε~φομενος", _;
    ),
    // Μῑγνῡμῐ: 19 words (εἰμῐ, δῐδωμῐ, δεικνῡμῐ, ὀρνῡμῐ, ἀντᾰποδῐδωμῐ, μῑγνῡμῐ). Like Οἰομαι, except in 35 cells: ind_act_1 sg, ind_act_3 sg, ind_mid_1 sg, subj_pass_2 sg, imperfect_act_1 sg, imperfect_act_3 sg, ….
    conjugation!("μῑγνῡμῐ", 19, "~μῐ", like Οἰομαι as MID_AS_ACT;
    ind_act_1: "~μῐ", _, _;
    ind_act_3: "~σῐ", _, _;
    ind_mid_1: derived, _, _;
    subj_pass_2: derived, _, _;
    imperfect_act_1: "~ν", _, _;
    imperfect_act_3: "~", _, _;
    aorist_ind_pass_1: derived, _, derived;
    aorist_ind_pass_2: derived, derived, derived;
    aorist_ind_pass_3: derived, derived, derived;
    aorist_subj_pass_1: derived, _, derived;
    aorist_subj_pass_2: derived, derived, derived;
    aorist_subj_pass_3: derived, _, derived;
    aorist_opt_pass_1: derived, _, derived;
    aorist_opt_pass_2: derived, derived, derived;
    aorist_opt_pass_3: derived, derived, derived;
    aorist_imp_pass_2: derived, _, _;
    aorist_imp_pass_3: derived, derived, derived;
    infinitive_aor: _, _, derived;
    participle_aor: _, _, derived;
    ),
    // Νῑκαω: 18 words (νῑκαω, λικμαω, ὀργαω, μελεταω, κομαω, εἰσοραω). Like Δοκᾰω, except in 34 cells: future_ind_act_3 pl, pluperfect_act_1 sg, pluperfect_act_1 pl, pluperfect_act_2 sg, pluperfect_act_2 du, pluperfect_act_2 pl, ….
    conjugation!("νῑκαω", 18, "~αω", like Δοκᾰω;
    future_ind_act_3: _, _, "~ησουσῐ";
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_futp: _, derived, _;
    participle_futp: _, derived, _;
    ),
    // Πᾰραιτεομαι: 18 words (ὀρχεομαι, ἀρνεομαι, ἱκνεομαι, ἐπεξηγεομαι, ἀφῐκνεομαι, ὑπῐσχνεομαι). Like Ποιεω, except in 108 cells: ind_act_3 pl, ind_pass_2 sg, subj_act_3 pl, opt_act_1 sg, opt_act_1 pl, opt_act_2 sg, ….
    conjugation!("πᾰραιτεομαι", 18, "~εομαι", like Ποιεω;
    ind_act_3: _, _, "~οῡσῐ";
    ind_pass_2: "~εῑ", _, _;
    subj_act_3: _, _, "~ωσῐ";
    opt_act_1: "~οιην", _, "~οῑμεν";
    opt_act_2: "~οιης", "~οῑτον", "~οῑτε";
    opt_act_3: "~οιη", "~οιτην", "~οῑεν";
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_2: "~ησῃ", _, _;
    future_ind_pass_2: "~ηθησῃ", _, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_subj_act_1: derived, _, derived;
    aorist_subj_act_2: derived, derived, derived;
    aorist_subj_act_3: derived, _, derived;
    aorist_subj_mid_2: "~ησῃ", _, _;
    aorist_subj_pass_3: _, _, "~ηθωσῐ";
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_pass_1: _, _, "~ηθεῑμεν";
    aorist_opt_pass_2: _, "~ηθεῑτον", "~ηθεῑτε";
    aorist_opt_pass_3: _, "~ηθειτην", "~ηθεῑεν";
    aorist_imp_act_2: derived, derived, derived;
    aorist_imp_act_3: derived, derived, derived;
    perfect_ind_act_3: "%ε~ηκε", _, "%ε~ηκᾱσῐ";
    perfect_subj_act_3: _, _, "%ε~ηκωσῐ";
    perfect_opt_act_1: "%ε~ηκοιμῐ", _, _;
    perfect_opt_act_2: "%ε~ηκοις", _, _;
    perfect_opt_act_3: "%ε~ηκοι", _, _;
    perfect_imp_act_2: derived, _, _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    infinitive_futp: _, derived, _;
    participle_fut: derived, _, _;
    participle_aor: derived, _, _;
    participle_futp: _, derived, _;
    ),
    // Πορσῡνω: 18 words (πληθῡνω, ἀμῡνω, εὐρῡνω, δῐευθῡνω, εὐθῡνω, αἰσχῡνω). Like Γῡροω, except in 235 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("πορσῡνω", 18, "~ῡνω", like Γῡροω;
    stem aorist_ind: "~ῡν- like ποιεω";
    stem aorist_ind_pass: "~ῡνθη- like ἐχω";
    stem pluperfect_act: "~ῠγκε- like ἐχω";
    ind_act_1: "~ῡνω", _, "~ῡνομεν";
    ind_act_2: "~ῡνεις", "~ῡνετον", "~ῡνετε";
    ind_act_3: "~ῡνει", _, "~ῡνουσῐ|~ῡνουσῐν";
    ind_mid_1: "~ῡνομαι", _, "~ῡνομεθᾰ";
    ind_mid_2: "~ῡνῃ|~ῡνει", "~ῡνεσθον", "~ῡνεσθε";
    ind_mid_3: "~ῡνεται", _, "~ῡνονται";
    subj_act_1: "~ῡνω", _, "~ῡνωμεν";
    subj_act_2: "~ῡνῃς", "~ῡνητον", "~ῡνητε";
    subj_act_3: "~ῡνῃ", _, "~ῡνωσῐ|~ῡνωσῐν";
    subj_mid_1: "~ῡνωμαι", _, "~ῡνωμεθᾰ";
    subj_mid_2: _, "~ῡνησθον", "~ῡνησθε";
    subj_mid_3: "~ῡνηται", _, "~ῡνωνται";
    opt_act_1: "~ῡνοιμῐ", _, "~ῡνοιμεν";
    opt_act_2: "~ῡνοις", "~ῡνοιτον", "~ῡνοιτε";
    opt_act_3: "~ῡνοι", "~ῡνοιτην", "~ῡνοιεν";
    opt_mid_1: "~ῡνοιμην", _, "~ῡνοιμεθᾰ";
    opt_mid_2: "~ῡνοιο", "~ῡνοισθον", "~ῡνοισθε";
    opt_mid_3: "~ῡνοιτο", "~ῡνοισθην", "~ῡνοιντο";
    imp_act_2: "~ῡνε", _, _;
    imp_act_3: "~ῡνετω", "~ῡνετων", "~ῡνοντων";
    imp_mid_2: "~ῡνου", _, _;
    imp_mid_3: "~ῡνεσθω", "~ῡνεσθων", _;
    imperfect_act_1: "ἐ~ῡνον", _, "ἐ~ῡνομεν";
    imperfect_act_2: "ἐ~ῡνες", "ἐ~ῡνετον", "ἐ~ῡνετε";
    imperfect_act_3: "ἐ~ῡνε|ἐ~ῡνεν", "ἐ~ῡνετην", "ἐ~ῡνον";
    imperfect_mid_1: "ἐ~ῡνομην", _, "ἐ~ῡνομεθᾰ";
    imperfect_mid_2: "ἐ~ῡνου", "ἐ~ῡνεσθον", "ἐ~ῡνεσθε";
    imperfect_mid_3: "ἐ~ῡνετο", "ἐ~ῡνεσθην", "ἐ~ῡνοντο";
    future_ind_act_1: "~ῠνω", _, "~ῠνοῡμεν";
    future_ind_act_2: "~ῠνεῑς", "~ῠνεῑτον", "~ῠνεῑτε";
    future_ind_act_3: "~ῠνεῑ", _, "~ῠνοῡσῐ|~ῠνοῡσῐν";
    future_ind_mid_1: "~ῠνοῡμαι", _, "~ῠνουμεθᾰ";
    future_ind_mid_2: "~ῠνεῑ|~ῠνῃ", "~ῠνεῑσθον", "~ῠνεῑσθε";
    future_ind_mid_3: "~ῠνεῑται", _, "~ῠνοῡνται";
    future_ind_pass_1: "~ῠνθησομαι", _, "~ῠνθησομεθᾰ";
    future_ind_pass_2: "~ῠνθησῃ", "~ῠνθησεσθον", "~ῠνθησεσθε";
    future_ind_pass_3: "~ῠνθησεται", _, "~ῠνθησονται";
    future_opt_act_1: "~ῠνοιην|~ῠνοῑμῐ", _, "~ῠνοῑμεν|~ῠνοιημεν";
    future_opt_act_2: "~ῠνοιης|~ῠνοῑς", "~ῠνοῑτον|~ῠνοιητον", "~ῠνοῑτε|~ῠνοιητε";
    future_opt_act_3: "~ῠνοιη|~ῠνοῑ", "~ῠνοιτην|~ῠνοιητην", "~ῠνοῑεν|~ῠνοιησᾰν";
    future_opt_mid_1: "~ῠνοιμην", _, "~ῠνοιμεθᾰ";
    future_opt_mid_2: "~ῠνοῑο", "~ῠνοῑσθον", "~ῠνοῑσθε";
    future_opt_mid_3: "~ῠνοῑτο", "~ῠνοισθην", "~ῠνοῑντο";
    future_opt_pass_1: "~ῠνθησοιμην", _, "~ῠνθησοιμεθᾰ";
    future_opt_pass_2: "~ῠνθησοιο", "~ῠνθησοισθον", "~ῠνθησοισθε";
    future_opt_pass_3: "~ῠνθησοιτο", "~ῠνθησοισθην", "~ῠνθησοιντο";
    aorist_subj_act_1: "~ῡνω", _, "~ῡνωμεν";
    aorist_subj_act_2: "~ῡνῃς", "~ῡνητον", "~ῡνητε";
    aorist_subj_act_3: "~ῡνῃ", _, "~ῡνωσῐ";
    aorist_subj_mid_1: "~ῡνωμαι", _, "~ῡνωμεθᾰ";
    aorist_subj_mid_2: _, "~ῡνησθον", "~ῡνησθε";
    aorist_subj_mid_3: "~ῡνηται", _, "~ῡνωνται";
    aorist_subj_pass_1: "~ῡνθω", _, "~ῡνθωμεν";
    aorist_subj_pass_2: "~ῡνθῃς", "~ῡνθητον", "~ῡνθητε";
    aorist_subj_pass_3: "~ῡνθῃ", _, "~ῡνθωσῐ";
    aorist_opt_act_1: "~ῡναιμῐ", _, "~ῡναιμεν";
    aorist_opt_act_2: "~ῡνειᾰς", "~ῡναιτον", "~ῡναιτε";
    aorist_opt_act_3: "~ῡνειε", "~ῡναιτην", "~ῡνειᾰν";
    aorist_opt_mid_1: "~ῡναιμην", _, "~ῡναιμεθᾰ";
    aorist_opt_mid_2: "~ῡναιο", "~ῡναισθον", "~ῡναισθε";
    aorist_opt_mid_3: "~ῡναιτο", "~ῡναισθην", "~ῡναιντο";
    aorist_opt_pass_1: "~ῡνθειην", _, "~ῡνθεῑμεν";
    aorist_opt_pass_2: "~ῡνθειης", "~ῡνθεῑτον", "~ῡνθεῑτε";
    aorist_opt_pass_3: "~ῡνθειη", "~ῡνθειτην", "~ῡνθεῑεν";
    aorist_imp_act_2: "~ῡνον", "~ῡνᾰτον", "~ῡνᾰτε";
    aorist_imp_act_3: "~ῡνᾰτω", "~ῡνᾰτων", "~ῡνᾰντων";
    aorist_imp_mid_2: "~ῡναι", "~ῡνᾰσθον", "~ῡνᾰσθε";
    aorist_imp_mid_3: "~ῡνᾰσθω", "~ῡνᾰσθων", _;
    aorist_imp_pass_2: "~ῡνθητῐ", _, _;
    aorist_imp_pass_3: "~ῡνθητω", "~ῡνθητων", "~ῡνθεντων";
    perfect_ind_act_1: "%ε~ῠγκᾰ", _, "%ε~ῠγκᾰμεν";
    perfect_ind_act_2: "%ε~ῠγκᾰς", "%ε~ῠγκᾰτον", "%ε~ῠγκᾰτε";
    perfect_ind_act_3: "%ε~ῠγκε", _, "%ε~ῠγκᾱσῐ";
    perfect_ind_mid_1: "%ε~ῠσμαι", _, "%ε~ῠσμεθᾰ";
    perfect_ind_mid_2: "%ε~ῠνσαι", "%ε~ῠνθον", "%ε~ῠνθε";
    perfect_ind_mid_3: "%ε~ῠνται", _, "%ε~ῠνᾰται";
    perfect_subj_act_1: "%ε~ῠγκω", _, "%ε~ῠγκωμεν";
    perfect_subj_act_2: "%ε~ῠγκῃς", "%ε~ῠγκητον", "%ε~ῠγκητε";
    perfect_subj_act_3: "%ε~ῠγκῃ", _, "%ε~ῠγκωσῐ";
    perfect_opt_act_1: "%ε~ῠγκοιμῐ", _, "%ε~ῠγκοιμεν";
    perfect_opt_act_2: "%ε~ῠγκοις", "%ε~ῠγκοιτον", "%ε~ῠγκοιτε";
    perfect_opt_act_3: "%ε~ῠγκοι", "%ε~ῠγκοιτην", "%ε~ῠγκοιεν";
    perfect_imp_act_2: _, "%ε~ῠγκετον", "%ε~ῠγκετε";
    perfect_imp_act_3: "%ε~ῠγκετω", "%ε~ῠγκετων", "%ε~ῠγκοντων";
    perfect_imp_mid_2: "%ε~ῠνσο", _, _;
    perfect_imp_mid_3: "%ε~ῠνθω", "%ε~ῠνθων", _;
    pluperfect_mid_1: "ἐ%ε~ῠσμην", _, "ἐ%ε~ῠσμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ῠνσο", "ἐ%ε~ῠνθον", "ἐ%ε~ῠνθε";
    pluperfect_mid_3: "ἐ%ε~ῠντο", "ἐ%ε~ῠνθην", "ἐ%ε~ῠνᾰτο";
    infinitive_pres: "~ῡνειν", "~ῡνεσθαι", _;
    infinitive_fut: "~ῠνεῑν", "~ῠνεῑσθαι", "~ῠνθησεσθαι";
    infinitive_aor: "~ῡναι", "~ῡνᾰσθαι", "~ῡνθηναι";
    infinitive_perf: "%ε~ῠγκεναι", "%ε~ῠνθαι", _;
    participle_pres: "~ῡνων", "~ῡνομενος", _;
    participle_fut: "~ῠνων", "~ῠνουμενος", "~ῠνθησομενος";
    participle_aor: "~ῡνᾱς", "~ῡνᾰμενος", "~ῡνθεις";
    participle_perf: "%ε~ῠγκως", "%ε~ῠσμενος", _;
    ),
    // Πεμπω: 17 words (λᾰμπω, βλεπω, ἐρειπω, λειπω, ἑρπω, πεμπω).
    conjugation!("πεμπω", 17, "~~πω";
    stem imperfect: "~~π- like ῥεω";
    stem aorist_ind: "~~ψ- like ἐχω";
    stem aorist_ind_pass: "~~φθη- like ἐχω";
    stem pluperfect_act: "~πο~φε- like κᾰλεω";
    ind_act_1: "~~πω", _, "~~πομεν";
    ind_act_2: "~~πεις", "~~πετον", "~~πετε";
    ind_act_3: "~~πει", _, "~~πουσῐ";
    ind_mid_1: "~~πομαι", _, "~~πομεθᾰ";
    ind_mid_2: "~~πῃ", "~~πεσθον", "~~πεσθε";
    ind_mid_3: "~~πεται", _, "~~πονται";
    subj_act_1: "~~πω", _, "~~πωμεν";
    subj_act_2: "~~πῃς", "~~πητον", "~~πητε";
    subj_act_3: "~~πῃ", _, "~~πωσῐ";
    subj_mid_1: "~~πωμαι", _, "~~πωμεθᾰ";
    subj_mid_2: _, "~~πησθον", "~~πησθε";
    subj_mid_3: "~~πηται", _, "~~πωνται";
    opt_act_1: "~~ποιμῐ", _, "~~ποιμεν";
    opt_act_2: "~~ποις", "~~ποιτον", "~~ποιτε";
    opt_act_3: "~~ποι", "~~ποιτην", "~~ποιεν";
    opt_mid_1: "~~ποιμην", _, "~~ποιμεθᾰ";
    opt_mid_2: "~~ποιο", "~~ποισθον", "~~ποισθε";
    opt_mid_3: "~~ποιτο", "~~ποισθην", "~~ποιντο";
    imp_act_2: "~~πε", _, _;
    imp_act_3: "~~πετω", "~~πετων", "~~ποντων";
    imp_mid_2: "~~που", _, _;
    imp_mid_3: "~~πεσθω", "~~πεσθων", _;
    future_ind_act_1: "~~ψω", _, "~~ψομεν";
    future_ind_act_2: "~~ψεις", "~~ψετον", "~~ψετε";
    future_ind_act_3: "~~ψει", _, "~~ψουσῐ";
    future_ind_mid_1: "~~ψομαι", _, "~~ψομεθᾰ";
    future_ind_mid_2: "~~ψῃ", "~~ψεσθον", "~~ψεσθε";
    future_ind_mid_3: "~~ψεται", _, "~~ψονται";
    future_ind_pass_1: "~~φθησομαι", _, "~~φθησομεθᾰ";
    future_ind_pass_2: "~~φθησῃ", "~~φθησεσθον", "~~φθησεσθε";
    future_ind_pass_3: "~~φθησεται", _, "~~φθησονται";
    future_opt_act_1: "~~ψοιμῐ", _, "~~ψοιμεν";
    future_opt_act_2: "~~ψοις", "~~ψοιτον", "~~ψοιτε";
    future_opt_act_3: "~~ψοι", "~~ψοιτην", "~~ψοιεν";
    future_opt_mid_1: "~~ψοιμην", _, "~~ψοιμεθᾰ";
    future_opt_mid_2: "~~ψοιο", "~~ψοισθον", "~~ψοισθε";
    future_opt_mid_3: "~~ψοιτο", "~~ψοισθην", "~~ψοιντο";
    future_opt_pass_1: "~~φθησοιμην", _, "~~φθησοιμεθᾰ";
    future_opt_pass_2: "~~φθησοιο", "~~φθησοισθον", "~~φθησοισθε";
    future_opt_pass_3: "~~φθησοιτο", "~~φθησοισθην", "~~φθησοιντο";
    aorist_subj_act_1: "~~ψω", _, "~~ψωμεν";
    aorist_subj_act_2: "~~ψῃς", "~~ψητον", "~~ψητε";
    aorist_subj_act_3: "~~ψῃ", _, "~~ψωσῐ";
    aorist_subj_mid_1: "~~ψωμαι", _, "~~ψωμεθᾰ";
    aorist_subj_mid_2: _, "~~ψησθον", "~~ψησθε";
    aorist_subj_mid_3: "~~ψηται", _, "~~ψωνται";
    aorist_subj_pass_1: "~~φθω", _, "~~φθωμεν";
    aorist_subj_pass_2: "~~φθῃς", "~~φθητον", "~~φθητε";
    aorist_subj_pass_3: "~~φθῃ", _, "~~φθωσῐ";
    aorist_opt_act_1: "~~ψαιμῐ", _, "~~ψαιμεν";
    aorist_opt_act_2: "~~ψειᾰς", "~~ψαιτον", "~~ψαιτε";
    aorist_opt_act_3: "~~ψειε", "~~ψαιτην", "~~ψειᾰν";
    aorist_opt_mid_1: "~~ψαιμην", _, "~~ψαιμεθᾰ";
    aorist_opt_mid_2: "~~ψαιο", "~~ψαισθον", "~~ψαισθε";
    aorist_opt_mid_3: "~~ψαιτο", "~~ψαισθην", "~~ψαιντο";
    aorist_opt_pass_1: "~~φθειην", _, "~~φθεῑμεν";
    aorist_opt_pass_2: "~~φθειης", "~~φθεῑτον", "~~φθεῑτε";
    aorist_opt_pass_3: "~~φθειη", "~~φθειτην", "~~φθεῑεν";
    aorist_imp_act_2: "~~ψον", "~~ψᾰτον", "~~ψᾰτε";
    aorist_imp_act_3: "~~ψᾰτω", "~~ψᾰτων", "~~ψᾰντων";
    aorist_imp_mid_2: "~~ψαι", "~~ψᾰσθον", "~~ψᾰσθε";
    aorist_imp_mid_3: "~~ψᾰσθω", "~~ψᾰσθων", _;
    aorist_imp_pass_2: "~~φθητῐ", _, _;
    aorist_imp_pass_3: "~~φθητω", "~~φθητων", "~~φθεντων";
    perfect_ind_act_1: "~πο~φᾰ", _, "~πο~φᾰμεν";
    perfect_ind_act_2: "~πο~φᾰς", "~πο~φᾰτον", "~πο~φᾰτε";
    perfect_ind_act_3: "~πο~φε", _, "~πο~φᾱσῐ";
    perfect_ind_mid_1: "%ε~~μαι", _, "%ε~~μεθᾰ";
    perfect_ind_mid_2: "%ε~~ψαι", "%ε~~φθον", "%ε~~φθε";
    perfect_ind_mid_3: "%ε~~πται", _, "%ε~~πᾰται";
    perfect_subj_act_1: "~πο~φω", _, "~πο~φωμεν";
    perfect_subj_act_2: "~πο~φῃς", "~πο~φητον", "~πο~φητε";
    perfect_subj_act_3: "~πο~φῃ", _, "~πο~φωσῐ";
    perfect_opt_act_1: "~πο~φοιμῐ", _, "~πο~φοιμεν";
    perfect_opt_act_2: "~πο~φοις", "~πο~φοιτον", "~πο~φοιτε";
    perfect_opt_act_3: "~πο~φοι", "~πο~φοιτην", "~πο~φοιεν";
    perfect_imp_act_2: _, "~πο~φετον", "~πο~φετε";
    perfect_imp_act_3: "~πο~φετω", "~πο~φετων", "~πο~φοντων";
    perfect_imp_mid_2: "%ε~~ψο", _, _;
    perfect_imp_mid_3: "%ε~~φθω", "%ε~~φθων", _;
    pluperfect_mid_1: "ἐ%ε~~μην", _, "ἐ%ε~~μεθᾰ";
    pluperfect_mid_2: "ἐ%ε~~ψο", "ἐ%ε~~φθον", "ἐ%ε~~φθε";
    pluperfect_mid_3: "ἐ%ε~~πτο", "ἐ%ε~~φθην", "ἐ%ε~~πᾰτο";
    infinitive_pres: "~~πειν", "~~πεσθαι", _;
    infinitive_fut: "~~ψειν", "~~ψεσθαι", "~~φθησεσθαι";
    infinitive_aor: "~~ψαι", "~~ψᾰσθαι", "~~φθηναι";
    infinitive_perf: "~πο~φεναι", "%ε~~φθαι", _;
    participle_pres: "~~πων", "~~πομενος", _;
    participle_fut: "~~ψων", "~~ψομενος", "~~φθησομενος";
    participle_aor: "~~ψᾱς", "~~ψᾰμενος", "~~φθεις";
    participle_perf: "~πο~φως", "%ε~~μενος", _;
    ),
    // Θωᾰω: 16 words (πεπτω, τᾰττω, ἀλεγω, ϝερρω, φᾱρω, θωαδδω). Like Οἰομαι, except in 62 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("θωᾰω", 16, "~ω", like Οἰομαι as PASS_AS_ACT;
    stem imperfect: "~- like ῥεω";
    ind_act_1: "~ω", _, "~ομεν";
    ind_act_2: "~εις", "~ετον", "~ετε";
    ind_act_3: "~ει", _, "~ουσῐ|~ουσῐν";
    ind_mid_2: "~εαι", _, _;
    ind_pass_1: derived, _, _;
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", "~ητον", "~ητε";
    subj_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    subj_mid_2: "~ηαι", _, _;
    subj_pass_2: "~ηαι", _, _;
    opt_act_1: "~οιμῐ", _, "~οιμεν";
    opt_act_2: "~οις", "~οιτον", "~οιτε";
    opt_act_3: "~οι", "~οιτην", "~οιεν";
    imp_act_2: "~ε", _, _;
    imp_act_3: "~ετω", "~ετων", "~οντων";
    imp_mid_2: "~εο", _, _;
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, derived;
    future_ind_mid_3: derived, _, derived;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, derived, derived;
    future_opt_mid_3: derived, derived, derived;
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, derived;
    perfect_ind_mid_3: derived, _, derived;
    perfect_imp_mid_2: derived, _, _;
    perfect_imp_mid_3: derived, derived, _;
    infinitive_pres: "~ειν", _, _;
    infinitive_fut: _, derived, _;
    infinitive_perf: _, derived, _;
    participle_pres: "~ων", _, _;
    participle_fut: _, derived, _;
    participle_perf: _, derived, _;
    ),
    // Ξαινω: 16 words (κᾰθαιρω, σημαινω, φαινω, ξαινω, ὑφαινω, ὁρμαινω). Like Πεμπω, except in 190 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ξαινω", 16, "~αι~ω", like Πεμπω as ACT_AS_PASS;
    stem imperfect: "~αι~- like ῥεω";
    stem aorist_ind: "~η~- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~θη- like ἐχω";
    ind_act_1: "~αι~ω", _, "~αι~ομεν";
    ind_act_2: "~αι~εις", "~αι~ετον", "~αι~ετε";
    ind_act_3: "~αι~ει", _, "~αι~ουσῐ";
    ind_mid_1: "~αι~ομαι", _, "~αι~ομεθᾰ";
    ind_mid_2: "~αι~ῃ", "~αι~εσθον", "~αι~εσθε";
    ind_mid_3: "~αι~εται", _, "~αι~ονται";
    subj_act_1: "~αι~ω", _, "~αι~ωμεν";
    subj_act_2: "~αι~ῃς", "~αι~ητον", "~αι~ητε";
    subj_act_3: "~αι~ῃ", _, "~αι~ωσῐ";
    subj_mid_1: "~αι~ωμαι", _, "~αι~ωμεθᾰ";
    subj_mid_2: _, "~αι~ησθον", "~αι~ησθε";
    subj_mid_3: "~αι~ηται", _, "~αι~ωνται";
    opt_act_1: "~αι~οιμῐ", _, "~αι~οιμεν";
    opt_act_2: "~αι~οις", "~αι~οιτον", "~αι~οιτε";
    opt_act_3: "~αι~οι", "~αι~οιτην", "~αι~οιεν";
    opt_mid_1: "~αι~οιμην", _, "~αι~οιμεθᾰ";
    opt_mid_2: "~αι~οιο", "~αι~οισθον", "~αι~οισθε";
    opt_mid_3: "~αι~οιτο", "~αι~οισθην", "~αι~οιντο";
    imp_act_2: "~αι~ε", _, _;
    imp_act_3: "~αι~ετω", "~αι~ετων", "~αι~οντων";
    imp_mid_2: "~αι~ου", _, _;
    imp_mid_3: "~αι~εσθω", "~αι~εσθων", _;
    future_ind_act_1: "~ᾰ~ω", _, "~ᾰ~οῡμεν";
    future_ind_act_2: "~ᾰ~εῑς", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    future_ind_act_3: "~ᾰ~εῑ", _, "~ᾰ~οῡσῐ";
    future_ind_mid_1: "~ᾰ~οῡμαι", _, "~ᾰ~ουμεθᾰ";
    future_ind_mid_2: "~ᾰ~εῑ", "~ᾰ~εῑσθον", "~ᾰ~εῑσθε";
    future_ind_mid_3: "~ᾰ~εῑται", _, "~ᾰ~οῡνται";
    future_ind_pass_1: "~ᾰ~θησομαι", _, "~ᾰ~θησομεθᾰ";
    future_ind_pass_2: "~ᾰ~θησῃ", "~ᾰ~θησεσθον", "~ᾰ~θησεσθε";
    future_ind_pass_3: "~ᾰ~θησεται", _, "~ᾰ~θησονται";
    future_opt_act_1: "~ᾰ~οιην", _, "~ᾰ~οῑμεν";
    future_opt_act_2: "~ᾰ~οιης", "~ᾰ~οῑτον", "~ᾰ~οῑτε";
    future_opt_act_3: "~ᾰ~οιη", "~ᾰ~οιτην", "~ᾰ~οῑεν";
    future_opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    future_opt_mid_2: "~ᾰ~οῑο", "~ᾰ~οῑσθον", "~ᾰ~οῑσθε";
    future_opt_mid_3: "~ᾰ~οῑτο", "~ᾰ~οισθην", "~ᾰ~οῑντο";
    future_opt_pass_1: "~ᾰ~θησοιμην", _, "~ᾰ~θησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~θησοιο", "~ᾰ~θησοισθον", "~ᾰ~θησοισθε";
    future_opt_pass_3: "~ᾰ~θησοιτο", "~ᾰ~θησοισθην", "~ᾰ~θησοιντο";
    aorist_subj_act_1: "~η~ω", _, "~η~ωμεν";
    aorist_subj_act_2: "~η~ῃς", "~η~ητον", "~η~ητε";
    aorist_subj_act_3: "~η~ῃ", _, "~η~ωσῐ";
    aorist_subj_mid_1: "~η~ωμαι", _, "~η~ωμεθᾰ";
    aorist_subj_mid_2: _, "~η~ησθον", "~η~ησθε";
    aorist_subj_mid_3: "~η~ηται", _, "~η~ωνται";
    aorist_subj_pass_1: "~ᾰ~θω", _, "~ᾰ~θωμεν";
    aorist_subj_pass_2: "~ᾰ~θῃς", "~ᾰ~θητον", "~ᾰ~θητε";
    aorist_subj_pass_3: "~ᾰ~θῃ", _, "~ᾰ~θωσῐ";
    aorist_opt_act_1: "~η~αιμῐ", _, "~η~αιμεν";
    aorist_opt_act_2: "~η~ειᾰς", "~η~αιτον", "~η~αιτε";
    aorist_opt_act_3: "~η~ειε", "~η~αιτην", "~η~ειᾰν";
    aorist_opt_mid_1: "~η~αιμην", _, "~η~αιμεθᾰ";
    aorist_opt_mid_2: "~η~αιο", "~η~αισθον", "~η~αισθε";
    aorist_opt_mid_3: "~η~αιτο", "~η~αισθην", "~η~αιντο";
    aorist_opt_pass_1: "~ᾰ~θειην", _, "~ᾰ~θεῑμεν";
    aorist_opt_pass_2: "~ᾰ~θειης", "~ᾰ~θεῑτον", "~ᾰ~θεῑτε";
    aorist_opt_pass_3: "~ᾰ~θειη", "~ᾰ~θειτην", "~ᾰ~θεῑεν";
    aorist_imp_act_2: "~η~ον", "~η~ᾰτον", "~η~ᾰτε";
    aorist_imp_act_3: "~η~ᾰτω", "~η~ᾰτων", "~η~ᾰντων";
    aorist_imp_mid_2: "~η~αι", "~η~ᾰσθον", "~η~ᾰσθε";
    aorist_imp_mid_3: "~η~ᾰσθω", "~η~ᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~θητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~θητω", "~ᾰ~θητων", "~ᾰ~θεντων";
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, derived;
    perfect_ind_mid_3: derived, _, derived;
    perfect_imp_mid_2: derived, _, _;
    perfect_imp_mid_3: derived, derived, _;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_pres: "~αι~ειν", "~αι~εσθαι", _;
    infinitive_fut: "~ᾰ~εῑν", "~ᾰ~εῑσθαι", "~ᾰ~θησεσθαι";
    infinitive_aor: "~η~αι", "~η~ᾰσθαι", "~ᾰ~θηναι";
    infinitive_perf: _, derived, _;
    participle_pres: "~αι~ων", "~αι~ομενος", _;
    participle_fut: "~ᾰ~ων", "~ᾰ~ουμενος", "~ᾰ~θησομενος";
    participle_aor: "~η~ᾱς", "~η~ᾰμενος", "~ᾰ~θεις";
    participle_perf: _, derived, _;
    ),
    // Λογῐζομαι: 15 words (λογῐζομαι, χᾰρῐζομαι, λᾰζομαι, δῐᾰγωνῐζομαι, λῐᾰζομαι, πλειστηρῐζομαι). Like Χρημᾰτῐζω, except in 31 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, future_ind_act_3 pl, future_ind_mid_2 sg, future_ind_pass_2 sg, ….
    conjugation!("λογῐζομαι", 15, "~ζομαι", like Χρημᾰτῐζω;
    stem imperfect: "~ζ- like ῥεω";
    stem aorist_ind: "~σ- like ἐχω";
    stem pluperfect_act: "~κε- like ἐχω";
    ind_act_3: _, _, "~ζουσῐ";
    ind_mid_2: "~ζῃ", _, _;
    subj_act_3: _, _, "~ζωσῐ";
    future_ind_act_3: _, _, "~οῡσῐ";
    future_ind_mid_2: "~εῑ", _, _;
    future_ind_pass_2: "~σθησῃ", _, _;
    future_opt_act_1: "~οιην", _, "~οῑμεν";
    future_opt_act_2: "~οιης", "~οῑτον", "~οῑτε";
    future_opt_act_3: "~οιη", "~οιτην", "~οῑεν";
    aorist_subj_act_3: _, _, "~σωσῐ";
    aorist_subj_pass_3: _, _, "~σθωσῐ";
    aorist_opt_act_2: "~σειᾰς", _, _;
    aorist_opt_act_3: "~σειε", _, "~σειᾰν";
    aorist_opt_pass_1: _, _, "~σθεῑμεν";
    aorist_opt_pass_2: _, "~σθεῑτον", "~σθεῑτε";
    aorist_opt_pass_3: _, "~σθειτην", "~σθεῑεν";
    perfect_ind_act_3: "%ε~κε", _, "%ε~κᾱσῐ";
    perfect_subj_act_3: _, _, "%ε~κωσῐ";
    perfect_opt_act_1: "%ε~κοιμῐ", _, _;
    perfect_opt_act_2: "%ε~κοις", _, _;
    perfect_opt_act_3: "%ε~κοι", _, _;
    perfect_imp_act_2: derived, _, _;
    ),
    // Πειρᾰζω: 15 words (πᾰρουσῐᾰζω, πλεονᾰζω, θειᾰζω, γυμνᾰζω, πειρᾰζω, πλησῐᾰζω). Like Κτῐζω, except in 42 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 sg, perfect_ind_act_2 du, perfect_ind_act_2 pl, perfect_ind_act_3 sg, ….
    conjugation!("πειρᾰζω", 15, "~ζω", like Κτῐζω;
    perfect_ind_act_1: "%ε~κᾰ", _, "%ε~κᾰμεν";
    perfect_ind_act_2: "%ε~κᾰς", "%ε~κᾰτον", "%ε~κᾰτε";
    perfect_ind_act_3: "%ε~κε|%ε~κεν", _, "%ε~κᾱσῐ|%ε~κᾱσῐν";
    perfect_ind_mid_1: "%ε~σμαι", _, "%ε~σμεθᾰ";
    perfect_ind_mid_2: "%ε~σαι", "%ε~σθον", "%ε~σθε";
    perfect_ind_mid_3: "%ε~σται", _, "%ε~δᾰται";
    perfect_subj_act_1: "%ε~κω", _, "%ε~κωμεν";
    perfect_subj_act_2: "%ε~κῃς", "%ε~κητον", "%ε~κητε";
    perfect_subj_act_3: "%ε~κῃ", _, "%ε~κωσῐ|%ε~κωσῐν";
    perfect_opt_act_1: "%ε~κοιμῐ|%ε~κοιην", _, "%ε~κοιμεν";
    perfect_opt_act_2: "%ε~κοις|%ε~κοιης", "%ε~κοιτον", "%ε~κοιτε";
    perfect_opt_act_3: "%ε~κοι|%ε~κοιη", "%ε~κοιτην", "%ε~κοιεν";
    perfect_imp_act_2: "%ε~κε", "%ε~κετον", "%ε~κετε";
    perfect_imp_act_3: "%ε~κετω", "%ε~κετων", "%ε~κοντων";
    perfect_imp_mid_2: "%ε~σο", _, _;
    perfect_imp_mid_3: "%ε~σθω", "%ε~σθων", _;
    infinitive_perf: "%ε~κεναι", "%ε~σθαι", _;
    participle_perf: "%ε~κως", "%ε~σμενος", _;
    ),
    // Δερκομαι: 15 words (δερκομαι, μεμφομαι, σεβομαι, τερσομαι, λεχομαι, πετομαι). Like Ξαινω, except in 172 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("δερκομαι", 15, "~ε~ομαι", like Ξαινω as ACT_AS_PASS;
    stem aorist_ind: "~~- like δερκομαι";
    stem aorist_ind_pass: "~ε~ασθη- like ἐχω";
    ind_act_1: "~ε~ομαι", _, _;
    ind_mid_1: "~ε~ομαι", _, "~ε~ομεθᾰ";
    ind_mid_2: "~ε~ῃ", "~ε~εσθον", "~ε~εσθε";
    ind_mid_3: "~ε~εται", _, "~ε~ονται";
    subj_mid_1: "~ε~ωμαι", _, "~ε~ωμεθᾰ";
    subj_mid_2: "~ε~ῃ", "~ε~ησθον", "~ε~ησθε";
    subj_mid_3: "~ε~ηται", _, "~ε~ωνται";
    subj_pass_2: "~ε~ῃ", _, _;
    opt_mid_1: "~ε~οιμην", _, "~ε~οιμεθᾰ";
    opt_mid_2: "~ε~οιο", "~ε~οισθον", "~ε~οισθε";
    opt_mid_3: "~ε~οιτο", "~ε~οισθην", "~ε~οιντο";
    imp_mid_2: "~ε~ου", _, _;
    imp_mid_3: "~ε~εσθω", "~ε~εσθων", _;
    imperfect_mid_1: "ἐ~ε~ομην", _, "ἐ~ε~ομεθᾰ";
    imperfect_mid_2: "ἐ~ε~ου", "ἐ~ε~εσθον", "ἐ~ε~εσθε";
    imperfect_mid_3: "ἐ~ε~ετο", "ἐ~ε~εσθην", "ἐ~ε~οντο";
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_1: "~ε~ησομαι", _, "~ε~ησομεθᾰ";
    future_ind_mid_2: "~ε~ησῃ", "~ε~ησεσθον", "~ε~ησεσθε";
    future_ind_mid_3: "~ε~ησεται", _, "~ε~ησονται";
    future_ind_pass_1: "~ε~ασθησομαι", _, "~ε~ασθησομεθᾰ";
    future_ind_pass_2: "~ε~ασθησῃ", "~ε~ασθησεσθον", "~ε~ασθησεσθε";
    future_ind_pass_3: "~ε~ασθησεται", _, "~ε~ασθησονται";
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    future_opt_mid_1: "~ε~ησοιμην", _, "~ε~ησοιμεθᾰ";
    future_opt_mid_2: "~ε~ησοιο", "~ε~ησοισθον", "~ε~ησοισθε";
    future_opt_mid_3: "~ε~ησοιτο", "~ε~ησοισθην", "~ε~ησοιντο";
    future_opt_pass_1: "~ε~ασθησοιμην", _, "~ε~ασθησοιμεθᾰ";
    future_opt_pass_2: "~ε~ασθησοιο", "~ε~ασθησοισθον", "~ε~ασθησοισθε";
    future_opt_pass_3: "~ε~ασθησοιτο", "~ε~ασθησοισθην", "~ε~ασθησοιντο";
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_subj_act_1: derived, _, derived;
    aorist_subj_act_2: derived, derived, derived;
    aorist_subj_act_3: derived, _, derived;
    aorist_subj_mid_1: "~~ωμαι", _, "~~ωμεθᾰ";
    aorist_subj_mid_2: "~~ῃ", "~~ησθον", "~~ησθε";
    aorist_subj_mid_3: "~~ηται", _, "~~ωνται";
    aorist_subj_pass_1: "~ε~ασθω", _, "~ε~ασθωμεν";
    aorist_subj_pass_2: "~ε~ασθῃς", "~ε~ασθητον", "~ε~ασθητε";
    aorist_subj_pass_3: "~ε~ασθῃ", _, "~ε~ασθωσῐ";
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_mid_1: "~~οιμην", _, "~~οιμεθᾰ";
    aorist_opt_mid_2: "~~οῑο", "~~οῑσθον", "~~οῑσθε";
    aorist_opt_mid_3: "~~οῑτο", "~~οισθην", "~~οῑντο";
    aorist_opt_pass_1: "~ε~ασθειην", _, "~ε~ασθεῑμεν";
    aorist_opt_pass_2: "~ε~ασθειης", "~ε~ασθεῑτον", "~ε~ασθεῑτε";
    aorist_opt_pass_3: "~ε~ασθειη", "~ε~ασθειτην", "~ε~ασθεῑεν";
    aorist_imp_act_2: derived, _, _;
    aorist_imp_act_3: derived, derived, derived;
    aorist_imp_mid_2: "~~οῡ", "~~εσθον", "~~εσθε";
    aorist_imp_mid_3: "~~εσθω", "~~εσθων", _;
    aorist_imp_pass_2: "~ε~ασθητῐ", _, _;
    aorist_imp_pass_3: "~ε~ασθητω", "~ε~ασθητων", "~ε~ασθεντων";
    perfect_ind_mid_1: "~επο~ημαι", _, "~επο~ημεθᾰ";
    perfect_ind_mid_2: "~επο~ησαι", "~επο~ησθον", "~επο~ησθε";
    perfect_ind_mid_3: "~επο~ηται", _, "~επο~ηνται";
    perfect_imp_mid_2: "~επο~ησο", _, _;
    perfect_imp_mid_3: "~επο~ησθω", "~επο~ησθων", _;
    infinitive_pres: _, "~ε~εσθαι", _;
    infinitive_fut: derived, "~ε~ησεσθαι", "~ε~ασθησεσθαι";
    infinitive_aor: derived, "~~εσθαι", "~ε~ασθηναι";
    infinitive_perf: _, "~επο~ησθαι", _;
    participle_pres: _, "~ε~ομενος", _;
    participle_fut: derived, "~ε~ησομενος", "~ε~ασθησομενος";
    participle_aor: derived, "~~ομενος", "~ε~ασθεις";
    participle_perf: _, "~επο~ημενος", _;
    ),
    // Χρῑω: 15 words (γρᾰφω, σειω, παιω, δρᾱω, χρῑω, μηνῡω). Like Ἐλπω, except in 64 cells: ind_mid_2 sg, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, ….
    conjugation!("χρῑω", 15, "~ω", like Ἐλπω;
    stem aorist_ind: "~σ- like ποιεω";
    stem aorist_ind_pass: "~σθη- like ἐχω";
    stem pluperfect: "~- like πειρᾰω";
    ind_mid_2: "~ῃ|~ει", _, _;
    imperfect_act_1: "ἐ~ον", _, "ἐ~ομεν";
    imperfect_act_2: "ἐ~ες", "ἐ~ετον", "ἐ~ετε";
    imperfect_act_3: "ἐ~ε|ἐ~εν", "ἐ~ετην", "ἐ~ον";
    imperfect_mid_1: "ἐ~ομην", _, "ἐ~ομεθᾰ";
    imperfect_mid_2: "ἐ~ου", "ἐ~εσθον", "ἐ~εσθε";
    imperfect_mid_3: "ἐ~ετο", "ἐ~εσθην", "ἐ~οντο";
    future_ind_act_3: _, _, "~σουσῐ|~σουσῐν";
    future_ind_mid_2: "~σῃ|~σει", _, _;
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", _, "~σειᾰν|~σαιεν";
    perfect_ind_act_1: "%ε~κᾰ", _, "%ε~κᾰμεν";
    perfect_ind_act_2: "%ε~κᾰς", "%ε~κᾰτον", "%ε~κᾰτε";
    perfect_ind_act_3: "%ε~κε", _, "%ε~κᾱσῐ";
    perfect_ind_mid_1: "%ε~μαι", _, "%ε~μεθᾰ";
    perfect_ind_mid_2: "%ε~σαι", "%ε~σθον", "%ε~σθε";
    perfect_ind_mid_3: "%ε~ται", _, "%ε~νται";
    perfect_subj_act_1: "%ε~κω", _, "%ε~κωμεν";
    perfect_subj_act_2: "%ε~κῃς", "%ε~κητον", "%ε~κητε";
    perfect_subj_act_3: "%ε~κῃ", _, "%ε~κωσῐ";
    perfect_opt_act_1: "%ε~κοιμῐ", _, "%ε~κοιμεν";
    perfect_opt_act_2: "%ε~κοις", "%ε~κοιτον", "%ε~κοιτε";
    perfect_opt_act_3: "%ε~κοι", "%ε~κοιτην", "%ε~κοιεν";
    perfect_imp_act_2: _, "%ε~κετον", "%ε~κετε";
    perfect_imp_act_3: "%ε~κετω", "%ε~κετων", "%ε~κοντων";
    perfect_imp_mid_2: "%ε~σο", _, _;
    perfect_imp_mid_3: "%ε~σθω", "%ε~σθων", _;
    infinitive_perf: "%ε~κεναι", "%ε~σθαι", _;
    participle_perf: "%ε~κως", "%ε~μενος", _;
    ),
    // Ἀνῐστημῐ: 15 words (ἐξῐστημῐ, ἐπεντῐθημῐ, τῐθημῐ, πᾰρῐστημῐ, ἀνῐστημῐ, κᾰθῐστημῐ). Like Πεμπω, except in 269 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀνῐστημῐ", 15, "~ῐ~ημῐ", like Πεμπω;
    stem aorist_ind: "~ε~ησ- like ἐλπω";
    stem aorist_ind_pass: "~ε~ᾰθη- like ἐλπω";
    stem pluperfect_act: "~ε~ᾰκε- like ἀνῐστημῐ";
    ind_act_1: "~ῐ~ημῐ", _, "~ῐ~ᾰμεν";
    ind_act_2: "~ῐ~ης", "~ῐ~ᾰτον", "~ῐ~ᾰτε";
    ind_act_3: "~ῐ~ησῐ|~ῐ~ησῐν", _, "~ῐ~ᾱσῐ|~ῐ~ᾱσῐν";
    ind_mid_1: "~ῐ~ᾰμαι", _, "~ῐ~ᾰμεθᾰ";
    ind_mid_2: "~ῐ~ᾰσαι", "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    ind_mid_3: "~ῐ~ᾰται", _, "~ῐ~ᾰνται";
    subj_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    subj_act_2: "~ῐ~ῃς", "~ῐ~ητον", "~ῐ~ητε";
    subj_act_3: "~ῐ~ῃ", _, "~ῐ~ωσῐ|~ῐ~ωσῐν";
    subj_mid_1: "~ῐ~ωμαι", _, "~ῐ~ωμεθᾰ";
    subj_mid_2: _, "~ῐ~ησθον", "~ῐ~ησθε";
    subj_mid_3: "~ῐ~ηται", _, "~ῐ~ωνται";
    opt_act_1: "~ῐ~αιην", _, "~ῐ~αῑμεν|~ῐ~αιημεν";
    opt_act_2: "~ῐ~αιης", "~ῐ~αῑτον|~ῐ~αιητον", "~ῐ~αῑτε|~ῐ~αιητε";
    opt_act_3: "~ῐ~αιη", "~ῐ~αιτην|~ῐ~αιητην", "~ῐ~αῑεν|~ῐ~αιησᾰν";
    opt_mid_1: "~ῐ~αιμην", _, "~ῐ~αιμεθᾰ";
    opt_mid_2: "~ῐ~αιο", "~ῐ~αισθον", "~ῐ~αισθε";
    opt_mid_3: "~ῐ~αιτο", "~ῐ~αισθην", "~ῐ~αιντο";
    imp_act_2: "~ῐ~η", _, _;
    imp_act_3: "~ῐ~ᾰτω", "~ῐ~ᾰτων", "~ῐ~ᾰντων";
    imp_mid_2: "~ῐ~ᾰσο|~ῐ~ω", _, _;
    imp_mid_3: "~ῐ~ᾰσθω", "~ῐ~ᾰσθων", _;
    imperfect_act_1: "~ῑ~ην", _, "~ῑ~ᾰμεν";
    imperfect_act_2: "~ῑ~ης", "~ῑ~ᾰτον", "~ῑ~ᾰτε";
    imperfect_act_3: "~ῑ~η", "~ῑ~ᾰτην", "~ῑ~ᾰσᾰν";
    imperfect_mid_1: "~ῑ~ᾰμην", _, "~ῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~ῑ~ᾰσο", "~ῑ~ᾰσθον", "~ῑ~ᾰσθε";
    imperfect_mid_3: "~ῑ~ᾰτο", "~ῑ~ᾰσθην", "~ῑ~ᾰντο";
    future_ind_act_1: "~ᾰ~ησω", _, "~ᾰ~ησομεν";
    future_ind_act_2: "~ᾰ~ησεις", "~ᾰ~ησετον", "~ᾰ~ησετε";
    future_ind_act_3: "~ᾰ~ησει", _, "~ᾰ~ησουσῐ";
    future_ind_mid_1: "~ᾰ~ησομαι", _, "~ᾰ~ησομεθᾰ";
    future_ind_mid_2: "~ᾰ~ησῃ", "~ᾰ~ησεσθον", "~ᾰ~ησεσθε";
    future_ind_mid_3: "~ᾰ~ησεται", _, "~ᾰ~ησονται";
    future_ind_pass_1: "~ᾰ~ησομαι", _, "~ᾰ~ησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ησῃ", "~ᾰ~ησεσθον", "~ᾰ~ησεσθε";
    future_ind_pass_3: "~ᾰ~ησεται", _, "~ᾰ~ησονται";
    future_opt_act_1: "~ᾰ~ησοιμῐ", _, "~ᾰ~ησοιμεν";
    future_opt_act_2: "~ᾰ~ησοις", "~ᾰ~ησοιτον", "~ᾰ~ησοιτε";
    future_opt_act_3: "~ᾰ~ησοι", "~ᾰ~ησοιτην", "~ᾰ~ησοιεν";
    future_opt_mid_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ησοιο", "~ᾰ~ησοισθον", "~ᾰ~ησοισθε";
    future_opt_mid_3: "~ᾰ~ησοιτο", "~ᾰ~ησοισθην", "~ᾰ~ησοιντο";
    future_opt_pass_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ησοιο", "~ᾰ~ησοισθον", "~ᾰ~ησοισθε";
    future_opt_pass_3: "~ᾰ~ησοιτο", "~ᾰ~ησοισθην", "~ᾰ~ησοιντο";
    aorist_subj_act_1: "~ᾰ~ησω", _, "~ᾰ~ησωμεν";
    aorist_subj_act_2: "~ᾰ~ησῃς", "~ᾰ~ησητον", "~ᾰ~ησητε";
    aorist_subj_act_3: "~ᾰ~ησῃ", _, "~ᾰ~ησωσῐ";
    aorist_subj_mid_1: "~ᾰ~ησωμαι", _, "~ᾰ~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰ~ησησθον", "~ᾰ~ησησθε";
    aorist_subj_mid_3: "~ᾰ~ησηται", _, "~ᾰ~ησωνται";
    aorist_subj_pass_1: "~ᾰ~ᾰθω", _, "~ᾰ~ᾰθωμεν";
    aorist_subj_pass_2: "~ᾰ~ᾰθῃς", "~ᾰ~ᾰθητον", "~ᾰ~ᾰθητε";
    aorist_subj_pass_3: "~ᾰ~ᾰθῃ", _, "~ᾰ~ᾰθωσῐ";
    aorist_opt_act_1: "~ᾰ~ησαιμῐ", _, "~ᾰ~ησαιμεν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς", "~ᾰ~ησαιτον", "~ᾰ~ησαιτε";
    aorist_opt_act_3: "~ᾰ~ησειε", "~ᾰ~ησαιτην", "~ᾰ~ησειᾰν";
    aorist_opt_mid_1: "~ᾰ~ησαιμην", _, "~ᾰ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~ησαιο", "~ᾰ~ησαισθον", "~ᾰ~ησαισθε";
    aorist_opt_mid_3: "~ᾰ~ησαιτο", "~ᾰ~ησαισθην", "~ᾰ~ησαιντο";
    aorist_opt_pass_1: "~ᾰ~ᾰθειην", _, "~ᾰ~ᾰθεῑμεν";
    aorist_opt_pass_2: "~ᾰ~ᾰθειης", "~ᾰ~ᾰθεῑτον", "~ᾰ~ᾰθεῑτε";
    aorist_opt_pass_3: "~ᾰ~ᾰθειη", "~ᾰ~ᾰθειτην", "~ᾰ~ᾰθεῑεν";
    aorist_imp_act_2: "~ᾰ~ησον", "~ᾰ~ησᾰτον", "~ᾰ~ησᾰτε";
    aorist_imp_act_3: "~ᾰ~ησᾰτω", "~ᾰ~ησᾰτων", "~ᾰ~ησᾰντων";
    aorist_imp_mid_2: "~ᾰ~ησαι", "~ᾰ~ησᾰσθον", "~ᾰ~ησᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ησᾰσθω", "~ᾰ~ησᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~ᾰθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ᾰθητω", "~ᾰ~ᾰθητων", "~ᾰ~ᾰθεντων";
    perfect_ind_act_1: "~ε~ᾰκᾰ", _, "~ε~ᾰκᾰμεν";
    perfect_ind_act_2: "~ε~ᾰκᾰς", "~ε~ᾰκᾰτον", "~ε~ᾰκᾰτε";
    perfect_ind_act_3: "~ε~ᾰκε", _, "~ε~ᾰκᾱσῐ";
    perfect_ind_mid_1: "~ε~ᾰμαι", _, "~ε~ᾰμεθᾰ";
    perfect_ind_mid_2: "~ε~ᾰσαι", "~ε~ᾰσθον", "~ε~ᾰσθε";
    perfect_ind_mid_3: "~ε~ᾰται", _, "~ε~ᾰνται";
    perfect_subj_act_1: "~ε~ᾰκω", _, "~ε~ᾰκωμεν";
    perfect_subj_act_2: "~ε~ᾰκῃς", "~ε~ᾰκητον", "~ε~ᾰκητε";
    perfect_subj_act_3: "~ε~ᾰκῃ", _, "~ε~ᾰκωσῐ";
    perfect_opt_act_1: "~ε~ᾰκοιμῐ", _, "~ε~ᾰκοιμεν";
    perfect_opt_act_2: "~ε~ᾰκοις", "~ε~ᾰκοιτον", "~ε~ᾰκοιτε";
    perfect_opt_act_3: "~ε~ᾰκοι", "~ε~ᾰκοιτην", "~ε~ᾰκοιεν";
    perfect_imp_act_2: _, "~ε~ᾰκετον", "~ε~ᾰκετε";
    perfect_imp_act_3: "~ε~ᾰκετω", "~ε~ᾰκετων", "~ε~ᾰκοντων";
    perfect_imp_mid_2: "~ε~ᾰσο", _, _;
    perfect_imp_mid_3: "~ε~ᾰσθω", "~ε~ᾰσθων", _;
    pluperfect_mid_1: "~ε~ᾰμην", _, "~ε~ᾰμεθᾰ";
    pluperfect_mid_2: "~ε~ᾰσο", "~ε~ᾰσθον", "~ε~ᾰσθε";
    pluperfect_mid_3: "~ε~ᾰτο", "~ε~ᾰσθην", "~ε~ᾰντο";
    future_perfect_ind_act_1: "~ε~ηξω", _, "~ε~ηξομεν";
    future_perfect_ind_act_2: "~ε~ηξεις", "~ε~ηξετον", "~ε~ηξετε";
    future_perfect_ind_act_3: "~ε~ηξει", _, "~ε~ηξουσῐ";
    future_perfect_ind_mid_1: "~ε~ηξομαι", _, "~ε~ηξομεθᾰ";
    future_perfect_ind_mid_2: "~ε~ηξῃ", "~ε~ηξεσθον", "~ε~ηξεσθε";
    future_perfect_ind_mid_3: "~ε~ηξεται", _, "~ε~ηξονται";
    future_perfect_opt_act_1: "~ε~ηξοιμῐ", _, "~ε~ηξοιμεν";
    future_perfect_opt_act_2: "~ε~ηξοις", "~ε~ηξοιτον", "~ε~ηξοιτε";
    future_perfect_opt_act_3: "~ε~ηξοι", "~ε~ηξοιτην", "~ε~ηξοιεν";
    future_perfect_opt_mid_1: "~ε~ηξοιμην", _, "~ε~ηξοιμεθᾰ";
    future_perfect_opt_mid_2: "~ε~ηξοιο", "~ε~ηξοισθον", "~ε~ηξοισθε";
    future_perfect_opt_mid_3: "~ε~ηξοιτο", "~ε~ηξοισθην", "~ε~ηξοιντο";
    infinitive_pres: "~ῐ~ᾰναι", "~ῐ~ᾰσθαι", _;
    infinitive_fut: "~ᾰ~ησειν", "~ᾰ~ησεσθαι", "~ᾰ~ησεσθαι";
    infinitive_aor: "~ᾰ~ησαι", "~ᾰ~ησᾰσθαι", "~ᾰ~ᾰθηναι";
    infinitive_perf: "~ε~ᾰκεναι", "~ε~ᾰσθαι", _;
    infinitive_futp: "~ε~ηξειν", "~ε~ηξεσθαι", _;
    participle_pres: "~ῐ~ᾱς", "~ῐ~ᾰμενος", _;
    participle_fut: "~ᾰ~ησων", "~ᾰ~ησομενος", "~ᾰ~ησομενος";
    participle_aor: "~ᾰ~ησᾱς", "~ᾰ~ησᾰμενος", "~ᾰ~ᾰθεις";
    participle_perf: "~ε~ᾰκως", "~ε~ᾰμενος", _;
    participle_futp: "~ε~ηξων", "~ε~ηξομενος", _;
    ),
    // Ἀραομαι: 13 words (μναομαι, καυχᾰομαι, ἀραομαι, χρᾰομαι, κτᾰομαι, μᾰσαομαι). Like Οἰομαι, except in 74 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἀραομαι", 13, "~αομαι", like Οἰομαι as PASS_AS_ACT;
    stem aorist_ind: "~ησ- like πᾰραιτεομαι";
    ind_act_1: "~αομαι", _, _;
    ind_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    ind_mid_2: "~ᾳ", "~ᾱσθον", "~ᾱσθε";
    ind_mid_3: "~ᾱται", _, "~ωνται";
    ind_pass_1: derived, _, _;
    subj_mid_2: "~ᾳ", "~ᾱσθον", "~ᾱσθε";
    subj_mid_3: "~ᾱται", _, _;
    subj_pass_2: "~ᾳ", _, _;
    opt_mid_1: "~ῳμην", _, "~ῳμεθᾰ";
    opt_mid_2: "~ῳο", "~ῳσθον", "~ῳσθε";
    opt_mid_3: "~ῳτο", "~ῳσθην", "~ῳντο";
    imp_mid_2: "~ω", _, _;
    imp_mid_3: "~ᾱσθω", "~ᾱσθων", _;
    imperfect_mid_1: "ἐ~ωμην", _, "ἐ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ω", "ἐ~ᾱσθον", "ἐ~ᾱσθε";
    imperfect_mid_3: "ἐ~ᾱτο", "ἐ~ᾱσθην", "ἐ~ωντο";
    future_ind_pass_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_pass_2: "~ησῃ", "~ησεσθον", "~ησεσθε";
    future_ind_pass_3: "~ησεται", _, "~ησονται";
    future_opt_pass_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_pass_2: "~ησοιο", "~ησοισθον", "~ησοισθε";
    future_opt_pass_3: "~ησοιτο", "~ησοισθην", "~ησοιντο";
    aorist_subj_mid_1: "~ησωμαι", _, "~ησωμεθᾰ";
    aorist_subj_mid_2: "~ησῃ", "~ησησθον", "~ησησθε";
    aorist_subj_mid_3: "~ησηται", _, "~ησωνται";
    aorist_opt_mid_1: "~ησαιμην", _, "~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ησαιο", "~ησαισθον", "~ησαισθε";
    aorist_opt_mid_3: "~ησαιτο", "~ησαισθην", "~ησαιντο";
    aorist_imp_mid_2: "~ησαι", "~ησᾰσθον", "~ησᾰσθε";
    aorist_imp_mid_3: "~ησᾰσθω", "~ησᾰσθων", _;
    infinitive_pres: _, "~ᾱσθαι", _;
    infinitive_fut: _, _, "~ησεσθαι";
    infinitive_aor: _, "~ησᾰσθαι", _;
    participle_pres: _, "~ωμενος", _;
    participle_fut: _, _, "~ησομενος";
    participle_aor: _, "~ησᾰμενος", _;
    ),
    // Δῐοικεω: 12 words (δῐοικοδομεω, ἐνοικεω, προσοικεω, κᾰτοικοφθορεω, κατοικονομεω, κᾰτοικοδομεω). Like Πεμπω, except in 236 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("δῐοικεω", 12, "~οι~εω", like Πεμπω;
    stem aorist_ind: "~ῳ~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ῳ~ηθη- like ἐλπω";
    stem pluperfect_act: "~ῳ~ηκ- like εὐλογεω";
    ind_act_1: "~οι~ω", _, "~οι~οῡμεν";
    ind_act_2: "~οι~εῑς", "~οι~εῑτον", "~οι~εῑτε";
    ind_act_3: "~οι~εῑ", _, "~οι~οῡσῐ|~οι~οῡσῐν";
    ind_mid_1: "~οι~οῡμαι", _, "~οι~ουμεθᾰ";
    ind_mid_2: "~οι~εῑ|~οι~ῃ", "~οι~εῑσθον", "~οι~εῑσθε";
    ind_mid_3: "~οι~εῑται", _, "~οι~οῡνται";
    subj_act_1: "~οι~ω", _, "~οι~ωμεν";
    subj_act_2: "~οι~ῃς", "~οι~ητον", "~οι~ητε";
    subj_act_3: "~οι~ῃ", _, "~οι~ωσῐ|~οι~ωσῐν";
    subj_mid_1: "~οι~ωμαι", _, "~οι~ωμεθᾰ";
    subj_mid_2: _, "~οι~ησθον", "~οι~ησθε";
    subj_mid_3: "~οι~ηται", _, "~οι~ωνται";
    opt_act_1: "~οι~οιην|~οι~οῑμῐ", _, "~οι~οῑμεν|~οι~οιημεν";
    opt_act_2: "~οι~οιης|~οι~οῑς", "~οι~οῑτον|~οι~οιητον", "~οι~οῑτε|~οι~οιητε";
    opt_act_3: "~οι~οιη|~οι~οῑ", "~οι~οιτην|~οι~οιητην", "~οι~οῑεν|~οι~οιησᾰν";
    opt_mid_1: "~οι~οιμην", _, "~οι~οιμεθᾰ";
    opt_mid_2: "~οι~οῑο", "~οι~οῑσθον", "~οι~οῑσθε";
    opt_mid_3: "~οι~οῑτο", "~οι~οισθην", "~οι~οῑντο";
    imp_act_2: "~οι~ει", _, _;
    imp_act_3: "~οι~ειτω", "~οι~ειτων", "~οι~ουντων";
    imp_mid_2: "~οι~οῡ", _, _;
    imp_mid_3: "~οι~εισθω", "~οι~εισθων", _;
    imperfect_act_1: "~ῳ~ουν", _, "~ῳ~οῡμεν";
    imperfect_act_2: "~ῳ~εις", "~ῳ~εῑτον", "~ῳ~εῑτε";
    imperfect_act_3: "~ῳ~ει", "~ῳ~ειτην", "~ῳ~ουν";
    imperfect_mid_1: "~ῳ~ουμην", _, "~ῳ~ουμεθᾰ";
    imperfect_mid_2: "~ῳ~οῡ", "~ῳ~εῑσθον", "~ῳ~εῑσθε";
    imperfect_mid_3: "~ῳ~εῑτο", "~ῳ~εισθην", "~ῳ~οῡντο";
    future_ind_act_1: "~οι~ησω", _, "~οι~ησομεν";
    future_ind_act_2: "~οι~ησεις", "~οι~ησετον", "~οι~ησετε";
    future_ind_act_3: "~οι~ησει", _, "~οι~ησουσῐ|~οι~ησουσῐν";
    future_ind_mid_1: "~οι~ησομαι", _, "~οι~ησομεθᾰ";
    future_ind_mid_2: "~οι~ησῃ|~οι~ησει", "~οι~ησεσθον", "~οι~ησεσθε";
    future_ind_mid_3: "~οι~ησεται", _, "~οι~ησονται";
    future_ind_pass_1: "~οι~ηθησομαι", _, "~οι~ηθησομεθᾰ";
    future_ind_pass_2: "~οι~ηθησῃ|~οι~ηθησει", "~οι~ηθησεσθον", "~οι~ηθησεσθε";
    future_ind_pass_3: "~οι~ηθησεται", _, "~οι~ηθησονται";
    future_opt_act_1: "~οι~ησοιμῐ", _, "~οι~ησοιμεν";
    future_opt_act_2: "~οι~ησοις", "~οι~ησοιτον", "~οι~ησοιτε";
    future_opt_act_3: "~οι~ησοι", "~οι~ησοιτην", "~οι~ησοιεν";
    future_opt_mid_1: "~οι~ησοιμην", _, "~οι~ησοιμεθᾰ";
    future_opt_mid_2: "~οι~ησοιο", "~οι~ησοισθον", "~οι~ησοισθε";
    future_opt_mid_3: "~οι~ησοιτο", "~οι~ησοισθην", "~οι~ησοιντο";
    future_opt_pass_1: "~οι~ηθησοιμην", _, "~οι~ηθησοιμεθᾰ";
    future_opt_pass_2: "~οι~ηθησοιο", "~οι~ηθησοισθον", "~οι~ηθησοισθε";
    future_opt_pass_3: "~οι~ηθησοιτο", "~οι~ηθησοισθην", "~οι~ηθησοιντο";
    aorist_subj_act_1: "~οι~ησω", _, "~οι~ησωμεν";
    aorist_subj_act_2: "~οι~ησῃς", "~οι~ησητον", "~οι~ησητε";
    aorist_subj_act_3: "~οι~ησῃ", _, "~οι~ησωσῐ|~οι~ησωσῐν";
    aorist_subj_mid_1: "~οι~ησωμαι", _, "~οι~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~οι~ησησθον", "~οι~ησησθε";
    aorist_subj_mid_3: "~οι~ησηται", _, "~οι~ησωνται";
    aorist_subj_pass_1: "~οι~ηθω", _, "~οι~ηθωμεν";
    aorist_subj_pass_2: "~οι~ηθῃς", "~οι~ηθητον", "~οι~ηθητε";
    aorist_subj_pass_3: "~οι~ηθῃ", _, "~οι~ηθωσῐ|~οι~ηθωσῐν";
    aorist_opt_act_1: "~οι~ησαιμῐ", _, "~οι~ησαιμεν";
    aorist_opt_act_2: "~οι~ησειᾰς|~οι~ησαις", "~οι~ησαιτον", "~οι~ησαιτε";
    aorist_opt_act_3: "~οι~ησειε|~οι~ησειεν|~οι~ησαι", "~οι~ησαιτην", "~οι~ησειᾰν|~οι~ησαιεν";
    aorist_opt_mid_1: "~οι~ησαιμην", _, "~οι~ησαιμεθᾰ";
    aorist_opt_mid_2: "~οι~ησαιο", "~οι~ησαισθον", "~οι~ησαισθε";
    aorist_opt_mid_3: "~οι~ησαιτο", "~οι~ησαισθην", "~οι~ησαιντο";
    aorist_opt_pass_1: "~οι~ηθειην", _, "~οι~ηθεῑμεν|~οι~ηθειημεν";
    aorist_opt_pass_2: "~οι~ηθειης", "~οι~ηθεῑτον|~οι~ηθειητον", "~οι~ηθεῑτε|~οι~ηθειητε";
    aorist_opt_pass_3: "~οι~ηθειη", "~οι~ηθειτην|~οι~ηθειητην", "~οι~ηθεῑεν|~οι~ηθειησᾰν";
    aorist_imp_act_2: "~οι~ησον", "~οι~ησᾰτον", "~οι~ησᾰτε";
    aorist_imp_act_3: "~οι~ησᾰτω", "~οι~ησᾰτων", "~οι~ησᾰντων";
    aorist_imp_mid_2: "~οι~ησαι", "~οι~ησᾰσθον", "~οι~ησᾰσθε";
    aorist_imp_mid_3: "~οι~ησᾰσθω", "~οι~ησᾰσθων", _;
    aorist_imp_pass_2: "~οι~ηθητῐ", _, _;
    aorist_imp_pass_3: "~οι~ηθητω", "~οι~ηθητων", "~οι~ηθεντων";
    perfect_ind_act_1: "~ῳ~ηκᾰ", _, "~ῳ~ηκᾰμεν";
    perfect_ind_act_2: "~ῳ~ηκᾰς", "~ῳ~ηκᾰτον", "~ῳ~ηκᾰτε";
    perfect_ind_act_3: "~ῳ~ηκε|~ῳ~ηκεν", _, "~ῳ~ηκᾱσῐ|~ῳ~ηκᾱσῐν";
    perfect_ind_mid_1: "~ῳ~ημαι", _, "~ῳ~ημεθᾰ";
    perfect_ind_mid_2: "~ῳ~ησαι", "~ῳ~ησθον", "~ῳ~ησθε";
    perfect_ind_mid_3: "~ῳ~ηται", _, "~ῳ~ηνται";
    perfect_subj_act_1: "~ῳ~ηκω", _, "~ῳ~ηκωμεν";
    perfect_subj_act_2: "~ῳ~ηκῃς", "~ῳ~ηκητον", "~ῳ~ηκητε";
    perfect_subj_act_3: "~ῳ~ηκῃ", _, "~ῳ~ηκωσῐ|~ῳ~ηκωσῐν";
    perfect_opt_act_1: "~ῳ~ηκοιμῐ|~ῳ~ηκοιην", _, "~ῳ~ηκοιμεν";
    perfect_opt_act_2: "~ῳ~ηκοις|~ῳ~ηκοιης", "~ῳ~ηκοιτον", "~ῳ~ηκοιτε";
    perfect_opt_act_3: "~ῳ~ηκοι|~ῳ~ηκοιη", "~ῳ~ηκοιτην", "~ῳ~ηκοιεν";
    perfect_imp_act_2: "~ῳ~ηκε", "~ῳ~ηκετον", "~ῳ~ηκετε";
    perfect_imp_act_3: "~ῳ~ηκετω", "~ῳ~ηκετων", "~ῳ~ηκοντων";
    perfect_imp_mid_2: "~ῳ~ησο", _, _;
    perfect_imp_mid_3: "~ῳ~ησθω", "~ῳ~ησθων", _;
    pluperfect_mid_1: "~ῳ~ημην", _, "~ῳ~ημεθᾰ";
    pluperfect_mid_2: "~ῳ~ησο", "~ῳ~ησθον", "~ῳ~ησθε";
    pluperfect_mid_3: "~ῳ~ητο", "~ῳ~ησθην", "~ῳ~ηντο";
    infinitive_pres: "~οι~εῑν", "~οι~εῑσθαι", _;
    infinitive_fut: "~οι~ησειν", "~οι~ησεσθαι", "~οι~ηθησεσθαι";
    infinitive_aor: "~οι~ησαι", "~οι~ησᾰσθαι", "~οι~ηθηναι";
    infinitive_perf: "~ῳ~ηκεναι", "~ῳ~ησθαι", _;
    participle_pres: "~οι~ων", "~οι~ουμενος", _;
    participle_fut: "~οι~ησων", "~οι~ησομενος", "~οι~ηθησομενος";
    participle_aor: "~οι~ησᾱς", "~οι~ησᾰμενος", "~οι~ηθεις";
    participle_perf: "~ῳ~ηκως", "~ῳ~ημενος", _;
    ),
    // Σβεννῡμῐ: 12 words (ῥωννῡμῐ, ἑννῡμῐ, πετᾰννῡμῐ, σβεννῡμῐ, κερᾰννῡμῐ, ζωννῡμῐ). Like Χρῑω, except in 98 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("σβεννῡμῐ", 12, "~ννῡμῐ", like Χρῑω as PERFECT_AS_FUTURE;
    stem aorist_ind: "~σ- like ἐχω";
    ind_act_1: "~ννῡμῐ", _, "~ννῠμεν";
    ind_act_2: "~ννῡς", "~ννῠτον", "~ννῠτε";
    ind_act_3: "~ννῡσῐ", _, "~ννῠᾱσῐ";
    ind_mid_1: "~ννῠμαι", _, "~ννῠμεθᾰ";
    ind_mid_2: "~ννῠσαι", "~ννῠσθον", "~ννῠσθε";
    ind_mid_3: "~ννῠται", _, "~ννῠνται";
    subj_act_1: "~ννῠω", _, "~ννῠωμεν";
    subj_act_2: "~ννῠῃς", "~ννῠητον", "~ννῠητε";
    subj_act_3: "~ννῠῃ", _, "~ννῠωσῐ";
    subj_mid_1: "~ννῠωμαι", _, "~ννῠωμεθᾰ";
    subj_mid_2: _, "~ννῠησθον", "~ννῠησθε";
    subj_mid_3: "~ννῠηται", _, "~ννῠωνται";
    opt_act_1: "~ννῠοιμῐ", _, "~ννῠοιμεν";
    opt_act_2: "~ννῠοις", "~ννῠοιτον", "~ννῠοιτε";
    opt_act_3: "~ννῠοι", "~ννῠοιτην", "~ννῠοιεν";
    opt_mid_1: "~ννῠοιμην", _, "~ννῠοιμεθᾰ";
    opt_mid_2: "~ννῠοιο", "~ννῠοισθον", "~ννῠοισθε";
    opt_mid_3: "~ννῠοιτο", "~ννῠοισθην", "~ννῠοιντο";
    imp_act_2: "~ννῡ", _, _;
    imp_act_3: "~ννῠτω", "~ννῠτων", "~ννῠντων";
    imp_mid_2: "~ννῠσο", _, _;
    imp_mid_3: "~ννῠσθω", "~ννῠσθων", _;
    imperfect_act_1: "ἐ~ννῡν", _, "ἐ~ννῠμεν";
    imperfect_act_2: "ἐ~ννῡς", "ἐ~ννῠτον", "ἐ~ννῠτε";
    imperfect_act_3: "ἐ~ννῡ", "ἐ~ννῠτην", "ἐ~ννῠσᾰν";
    imperfect_mid_1: "ἐ~ννῠμην", _, "ἐ~ννῠμεθᾰ";
    imperfect_mid_2: "ἐ~ννῠσο", "ἐ~ννῠσθον", "ἐ~ννῠσθε";
    imperfect_mid_3: "ἐ~ννῠτο", "ἐ~ννῠσθην", "ἐ~ννῠντο";
    future_ind_act_3: _, _, "~σουσῐ";
    future_ind_mid_2: "~σῃ", _, _;
    aorist_subj_act_3: _, _, "~σωσῐ";
    aorist_opt_act_2: "~σειᾰς", _, _;
    aorist_opt_act_3: "~σειε", _, "~σειᾰν";
    perfect_ind_mid_1: "ἐ~μαι", _, "ἐ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~σαι", "ἐ~σθον", "ἐ~σθε";
    perfect_ind_mid_3: "ἐ~ται", _, "ἐ~νται";
    perfect_imp_mid_2: "ἐ~σο", _, _;
    perfect_imp_mid_3: "ἐ~σθω", "ἐ~σθων", _;
    pluperfect_mid_1: "ἐ~μην", _, "ἐ~μεθᾰ";
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", "ἐ~σθε";
    pluperfect_mid_3: "ἐ~το", "ἐ~σθην", "ἐ~ντο";
    infinitive_pres: "~ννῠναι", "~ννῠσθαι", _;
    infinitive_perf: derived, "ἐ~σθαι", _;
    participle_pres: "~ννῡς", "~ννῠμενος", _;
    participle_perf: derived, "ἐ~μενος", _;
    ),
    // Ἐκχεω: 12 words (κρῑνω, κλῑνω, εἰρω, ἐκχεω, ἰθῡνω, δῐᾰβαινω). Like Θωᾰω, except in 105 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, subj_mid_2 sg, subj_pass_2 sg, imp_mid_2 sg, ….
    conjugation!("ἐκχεω", 12, "~ω", like Θωᾰω as AORIST_AS_PRESENT;
    stem aorist_ind: "~- like ἐλπω";
    stem pluperfect_act: "~κε- like ἀνῐστημῐ";
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
    imperfect_mid_2: "~ου", "~εσθον", "~εσθε";
    imperfect_mid_3: "~ετο", "~εσθην", "~οντο";
    aorist_subj_act_3: _, _, "~ωσῐ";
    aorist_subj_mid_2: derived, _, _;
    aorist_subj_pass_2: derived, _, _;
    aorist_opt_act_1: "~αιμῐ", _, "~αιμεν";
    aorist_opt_act_2: "~ειᾰς", "~αιτον", "~αιτε";
    aorist_opt_act_3: "~ειε", "~αιτην", "~ειᾰν";
    aorist_opt_mid_1: "~αιμην", _, "~αιμεθᾰ";
    aorist_opt_mid_2: "~αιο", "~αισθον", "~αισθε";
    aorist_opt_mid_3: "~αιτο", "~αισθην", "~αιντο";
    aorist_imp_act_2: "~ον", "~ᾰτον", "~ᾰτε";
    aorist_imp_act_3: "~ᾰτω", "~ᾰτων", "~ᾰντων";
    aorist_imp_mid_2: "~αι", "~ᾰσθον", "~ᾰσθε";
    aorist_imp_mid_3: "~ᾰσθω", "~ᾰσθων", _;
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: "~κᾰς", "~κᾰτον", "~κᾰτε";
    perfect_ind_act_3: "~κε", _, "~κᾱσῐ";
    perfect_ind_mid_1: "~μαι", _, "~μεθᾰ";
    perfect_ind_mid_2: "~σαι", "~θον", "~θε";
    perfect_ind_mid_3: "~ται", _, "~ᾰται";
    perfect_subj_act_1: "~κω", _, "~κωμεν";
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ";
    perfect_opt_act_1: "~κοιμῐ", _, "~κοιμεν";
    perfect_opt_act_2: "~κοις", "~κοιτον", "~κοιτε";
    perfect_opt_act_3: "~κοι", "~κοιτην", "~κοιεν";
    perfect_imp_act_2: _, "~κετον", "~κετε";
    perfect_imp_act_3: "~κετω", "~κετων", "~κοντων";
    perfect_imp_mid_2: "~σο", _, _;
    perfect_imp_mid_3: "~θω", "~θων", _;
    pluperfect_mid_1: "~μην", _, "~μεθᾰ";
    pluperfect_mid_2: "~σο", "~θον", "~θε";
    pluperfect_mid_3: "~το", "~θην", "~ᾰτο";
    infinitive_aor: "~αι", "~ᾰσθαι", _;
    infinitive_perf: "~κεναι", "~θαι", _;
    participle_aor: "~ᾱς", "~ᾰμενος", _;
    participle_perf: "~κως", "~μενος", _;
    ),
    // Πειθω: 11 words (πειθω, περθω, ληθω, κλωθω, κευθω, δῐᾰπερθω). Like Σβεννῡμῐ, except in 74 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("πειθω", 11, "~θω", like Σβεννῡμῐ;
    stem imperfect: "~θ- like ῥεω";
    ind_act_1: "~θω", _, "~θομεν";
    ind_act_2: "~θεις", "~θετον", "~θετε";
    ind_act_3: "~θει", _, "~θουσῐ";
    ind_mid_1: "~θομαι", _, "~θομεθᾰ";
    ind_mid_2: "~θῃ", "~θεσθον", "~θεσθε";
    ind_mid_3: "~θεται", _, "~θονται";
    subj_act_1: "~θω", _, "~θωμεν";
    subj_act_2: "~θῃς", "~θητον", "~θητε";
    subj_act_3: "~θῃ", _, "~θωσῐ";
    subj_mid_1: "~θωμαι", _, "~θωμεθᾰ";
    subj_mid_2: _, "~θησθον", "~θησθε";
    subj_mid_3: "~θηται", _, "~θωνται";
    opt_act_1: "~θοιμῐ", _, "~θοιμεν";
    opt_act_2: "~θοις", "~θοιτον", "~θοιτε";
    opt_act_3: "~θοι", "~θοιτην", "~θοιεν";
    opt_mid_1: "~θοιμην", _, "~θοιμεθᾰ";
    opt_mid_2: "~θοιο", "~θοισθον", "~θοισθε";
    opt_mid_3: "~θοιτο", "~θοισθην", "~θοιντο";
    imp_act_2: "~θε", _, _;
    imp_act_3: "~θετω", "~θετων", "~θοντων";
    imp_mid_2: "~θου", _, _;
    imp_mid_3: "~θεσθω", "~θεσθων", _;
    perfect_ind_mid_1: "%ε~σμαι", _, "%ε~σμεθᾰ";
    perfect_ind_mid_2: "%ε~σαι", "%ε~σθον", "%ε~σθε";
    perfect_ind_mid_3: "%ε~σται", _, "%ε~θᾰται";
    perfect_imp_mid_2: "%ε~σο", _, _;
    perfect_imp_mid_3: "%ε~σθω", "%ε~σθων", _;
    pluperfect_mid_1: "ἐ%ε~σμην", _, "ἐ%ε~σμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~σο", "ἐ%ε~σθον", "ἐ%ε~σθε";
    pluperfect_mid_3: "ἐ%ε~στο", "ἐ%ε~σθην", "ἐ%ε~σᾰτο";
    infinitive_pres: "~θειν", "~θεσθαι", _;
    infinitive_perf: _, "%ε~σθαι", _;
    participle_pres: "~θων", "~θομενος", _;
    participle_perf: _, "%ε~σμενος", _;
    ),
    // Ἱημῐ: 11 words (ἱστημῐ, ἱημῐ, δαμνημῐ, πῐμπλημῐ, φημῐ, διατιθημῐ). Like Θωᾰω, except in 59 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἱημῐ", 11, "~ημῐ", like Θωᾰω;
    ind_act_1: "~ημῐ", _, "~ᾰμεν";
    ind_act_2: "~ης", "~ᾰτον", "~ᾰτε";
    ind_act_3: "~ησῐ", _, "~ᾱσῐ";
    ind_mid_1: "~ᾰμαι", _, "~ᾰμεθᾰ";
    ind_mid_2: "~ᾰσαι", "~ᾰσθον", "~ᾰσθε";
    ind_mid_3: "~ᾰται", _, "~ᾰνται";
    subj_mid_2: derived, _, _;
    subj_pass_2: derived, _, _;
    opt_act_1: "~αιην", _, "~αῑμεν|~αιημεν";
    opt_act_2: "~αιης", "~αῑτον|~αιητον", "~αῑτε|~αιητε";
    opt_act_3: "~αιη", "~αιτην|~αιητην", "~αῑεν|~αιησᾰν";
    opt_mid_1: "~αιμην", _, "~αιμεθᾰ";
    opt_mid_2: "~αιο", "~αισθον", "~αισθε";
    opt_mid_3: "~αιτο", "~αισθην", "~αιντο";
    imp_act_2: "~η", _, _;
    imp_act_3: "~ᾰτω", "~ᾰτων", "~ᾰντων";
    imp_mid_2: "~ᾰσο", _, _;
    imp_mid_3: "~ᾰσθω", "~ᾰσθων", _;
    imperfect_act_1: "ἐ~ην", _, "ἐ~ᾰμεν";
    imperfect_act_2: "ἐ~ης", "ἐ~ᾰτον", "ἐ~ᾰτε";
    imperfect_act_3: "ἐ~η", "ἐ~ᾰτην", "ἐ~ᾰσᾰν";
    imperfect_mid_1: "ἐ~ᾰμην", _, "ἐ~ᾰμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰσο", "ἐ~ᾰσθον", "ἐ~ᾰσθε";
    imperfect_mid_3: "ἐ~ᾰτο", "ἐ~ᾰσθην", "ἐ~ᾰντο";
    infinitive_pres: "~ᾰναι", "~ᾰσθαι", _;
    participle_pres: "~ᾱς", "~ᾰμενος", _;
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
    // Αἱρεω: 10 words (πωλεω, αἱρεω, ἀγραυλεω, αἰνεω, βῑνεω, ὑμνεω). Like Ἐχω, except in 54 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("αἱρεω", 10, "~εω", like Ἐχω;
    stem imperfect: "~ε- like ῥεω";
    ind_act_1: "~εω", _, "~εομεν";
    ind_act_2: "~εεις", "~εετον", "~εετε";
    ind_act_3: "~εει", _, "~εουσῐ";
    ind_mid_1: "~εομαι", _, "~εομεθᾰ";
    ind_mid_2: "~εῃ", "~εεσθον", "~εεσθε";
    ind_mid_3: "~εεται", _, "~εονται";
    subj_act_1: "~εω", _, "~εωμεν";
    subj_act_2: "~εῃς", "~εητον", "~εητε";
    subj_act_3: "~εῃ", _, "~εωσῐ";
    subj_mid_1: "~εωμαι", _, "~εωμεθᾰ";
    subj_mid_2: _, "~εησθον", "~εησθε";
    subj_mid_3: "~εηται", _, "~εωνται";
    opt_act_1: "~εοιμῐ", _, "~εοιμεν";
    opt_act_2: "~εοις", "~εοιτον", "~εοιτε";
    opt_act_3: "~εοι", "~εοιτην", "~εοιεν";
    opt_mid_1: "~εοιμην", _, "~εοιμεθᾰ";
    opt_mid_2: "~εοιο", "~εοισθον", "~εοισθε";
    opt_mid_3: "~εοιτο", "~εοισθην", "~εοιντο";
    imp_act_2: "~εε", _, _;
    imp_act_3: "~εετω", "~εετων", "~εοντων";
    imp_mid_2: "~εου", _, _;
    imp_mid_3: "~εεσθω", "~εεσθων", _;
    infinitive_pres: "~εειν", "~εεσθαι", _;
    participle_pres: "~εων", "~εομενος", _;
    ),
    // Ἀειρω: 10 words (κειρω, τεινω, θεινω, ἀειρω, ἐγειρω, ἀγειρω). Like Πεμπω, except in 219 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀειρω", 10, "~ει~ω", like Πεμπω;
    stem imperfect: "~ει~- like ῥεω";
    stem aorist_ind: "~ει~- like ἐχω";
    stem aorist_ind_pass: "~ε~θη- like ἐχω";
    stem pluperfect_act: "~εκα~κε- like κᾰλεω";
    ind_act_1: "~ει~ω", _, "~ει~ομεν";
    ind_act_2: "~ει~εις", "~ει~ετον", "~ει~ετε";
    ind_act_3: "~ει~ει", _, "~ει~ουσῐ";
    ind_mid_1: "~ει~ομαι", _, "~ει~ομεθᾰ";
    ind_mid_2: "~ει~ῃ", "~ει~εσθον", "~ει~εσθε";
    ind_mid_3: "~ει~εται", _, "~ει~ονται";
    subj_act_1: "~ει~ω", _, "~ει~ωμεν";
    subj_act_2: "~ει~ῃς", "~ει~ητον", "~ει~ητε";
    subj_act_3: "~ει~ῃ", _, "~ει~ωσῐ";
    subj_mid_1: "~ει~ωμαι", _, "~ει~ωμεθᾰ";
    subj_mid_2: _, "~ει~ησθον", "~ει~ησθε";
    subj_mid_3: "~ει~ηται", _, "~ει~ωνται";
    opt_act_1: "~ει~οιμῐ", _, "~ει~οιμεν";
    opt_act_2: "~ει~οις", "~ει~οιτον", "~ει~οιτε";
    opt_act_3: "~ει~οι", "~ει~οιτην", "~ει~οιεν";
    opt_mid_1: "~ει~οιμην", _, "~ει~οιμεθᾰ";
    opt_mid_2: "~ει~οιο", "~ει~οισθον", "~ει~οισθε";
    opt_mid_3: "~ει~οιτο", "~ει~οισθην", "~ει~οιντο";
    imp_act_2: "~εῑ~ε", _, _;
    imp_act_3: "~ει~ετω", "~ει~ετων", "~ει~οντων";
    imp_mid_2: "~ει~ου", _, _;
    imp_mid_3: "~ει~εσθω", "~ει~εσθων", _;
    future_ind_act_1: "~ε~ω", _, "~ε~οῡμεν";
    future_ind_act_2: "~ε~εῑς", "~ε~εῑτον", "~ε~εῑτε";
    future_ind_act_3: "~ε~εῑ", _, "~ε~οῡσῐ";
    future_ind_mid_1: "~ε~οῡμαι", _, "~ε~ουμεθᾰ";
    future_ind_mid_2: "~ε~εῑ", "~ε~εῑσθον", "~ε~εῑσθε";
    future_ind_mid_3: "~ε~εῑται", _, "~ε~οῡνται";
    future_ind_pass_1: "~ε~θησομαι", _, "~ε~θησομεθᾰ";
    future_ind_pass_2: "~ε~θησῃ", "~ε~θησεσθον", "~ε~θησεσθε";
    future_ind_pass_3: "~ε~θησεται", _, "~ε~θησονται";
    future_opt_act_1: "~ε~οιην", _, "~ε~οῑμεν";
    future_opt_act_2: "~ε~οιης", "~ε~οῑτον", "~ε~οῑτε";
    future_opt_act_3: "~ε~οιη", "~ε~οιτην", "~ε~οῑεν";
    future_opt_mid_1: "~ε~οιμην", _, "~ε~οιμεθᾰ";
    future_opt_mid_2: "~ε~οῑο", "~ε~οῑσθον", "~ε~οῑσθε";
    future_opt_mid_3: "~ε~οῑτο", "~ε~οισθην", "~ε~οῑντο";
    future_opt_pass_1: "~ε~θησοιμην", _, "~ε~θησοιμεθᾰ";
    future_opt_pass_2: "~ε~θησοιο", "~ε~θησοισθον", "~ε~θησοισθε";
    future_opt_pass_3: "~ε~θησοιτο", "~ε~θησοισθην", "~ε~θησοιντο";
    aorist_subj_act_1: "~ει~ω", _, "~ει~ωμεν";
    aorist_subj_act_2: "~ει~ῃς", "~ει~ητον", "~ει~ητε";
    aorist_subj_act_3: "~ει~ῃ", _, "~ει~ωσῐ";
    aorist_subj_mid_1: "~ει~ωμαι", _, "~ει~ωμεθᾰ";
    aorist_subj_mid_2: _, "~ει~ησθον", "~ει~ησθε";
    aorist_subj_mid_3: "~ει~ηται", _, "~ει~ωνται";
    aorist_subj_pass_1: "~ε~θω", _, "~ε~θωμεν";
    aorist_subj_pass_2: "~ε~θῃς", "~ε~θητον", "~ε~θητε";
    aorist_subj_pass_3: "~ε~θῃ", _, "~ε~θωσῐ";
    aorist_opt_act_1: "~ει~αιμῐ", _, "~ει~αιμεν";
    aorist_opt_act_2: "~ει~ειᾰς", "~ει~αιτον", "~ει~αιτε";
    aorist_opt_act_3: "~ει~ειε", "~ει~αιτην", "~ει~ειᾰν";
    aorist_opt_mid_1: "~ει~αιμην", _, "~ει~αιμεθᾰ";
    aorist_opt_mid_2: "~ει~αιο", "~ει~αισθον", "~ει~αισθε";
    aorist_opt_mid_3: "~ει~αιτο", "~ει~αισθην", "~ει~αιντο";
    aorist_opt_pass_1: "~ε~θειην", _, "~ε~θεῑμεν";
    aorist_opt_pass_2: "~ε~θειης", "~ε~θεῑτον", "~ε~θεῑτε";
    aorist_opt_pass_3: "~ε~θειη", "~ε~θειτην", "~ε~θεῑεν";
    aorist_imp_act_2: "~εῑ~ον", "~ει~ᾰτον", "~ει~ᾰτε";
    aorist_imp_act_3: "~ει~ᾰτω", "~ει~ᾰτων", "~ει~ᾰντων";
    aorist_imp_mid_2: "~εῑ~αι", "~ει~ᾰσθον", "~ει~ᾰσθε";
    aorist_imp_mid_3: "~ει~ᾰσθω", "~ει~ᾰσθων", _;
    aorist_imp_pass_2: "~ε~θητῐ", _, _;
    aorist_imp_pass_3: "~ε~θητω", "~ε~θητων", "~ε~θεντων";
    perfect_ind_act_1: "~εκα~κᾰ", _, "~εκα~κᾰμεν";
    perfect_ind_act_2: "~εκα~κᾰς", "~εκα~κᾰτον", "~εκα~κᾰτε";
    perfect_ind_act_3: "~εκα~κε", _, "~εκα~κᾱσῐ";
    perfect_ind_mid_1: "~εκα~μαι", _, "~εκα~μεθᾰ";
    perfect_ind_mid_2: "~εκα~σαι", "~εκα~θον", "~εκα~θε";
    perfect_ind_mid_3: "~εκα~ται", _, "~εκα~ᾰται";
    perfect_subj_act_1: "~εκα~κω", _, "~εκα~κωμεν";
    perfect_subj_act_2: "~εκα~κῃς", "~εκα~κητον", "~εκα~κητε";
    perfect_subj_act_3: "~εκα~κῃ", _, "~εκα~κωσῐ";
    perfect_opt_act_1: "~εκα~κοιμῐ", _, "~εκα~κοιμεν";
    perfect_opt_act_2: "~εκα~κοις", "~εκα~κοιτον", "~εκα~κοιτε";
    perfect_opt_act_3: "~εκα~κοι", "~εκα~κοιτην", "~εκα~κοιεν";
    perfect_imp_act_2: _, "~εκα~κετον", "~εκα~κετε";
    perfect_imp_act_3: "~εκα~κετω", "~εκα~κετων", "~εκα~κοντων";
    perfect_imp_mid_2: "~εκα~σο", _, _;
    perfect_imp_mid_3: "~εκα~θω", "~εκα~θων", _;
    pluperfect_mid_1: "ἐ~εκα~γμην", _, "ἐ~εκα~γμεθᾰ";
    pluperfect_mid_2: "ἐ~εκα~ξο", "ἐ~εκα~χθον", "ἐ~εκα~χθε";
    pluperfect_mid_3: "ἐ~εκα~κτο", "ἐ~εκα~χθην", "ἐ~εκα~κᾰτο";
    infinitive_pres: "~ει~ειν", "~ει~εσθαι", _;
    infinitive_fut: "~ε~εῑν", "~ε~εῑσθαι", "~ε~θησεσθαι";
    infinitive_aor: "~εῑ~αι", "~ει~ᾰσθαι", "~ε~θηναι";
    infinitive_perf: "~εκα~κεναι", "~εκᾱ~θαι", _;
    participle_pres: "~ει~ων", "~ει~ομενος", _;
    participle_fut: "~ε~ων", "~ε~ουμενος", "~ε~θησομενος";
    participle_aor: "~ει~ᾱς", "~ει~ᾰμενος", "~ε~θεις";
    participle_perf: "~εκα~κως", "~εκα~μενος", _;
    ),
    // Χεω: 10 words (πνεω, θεω, πλεω, χεω, νεω, ἀγρεω). Like Δεῑ, except in 144 cells: ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("χεω", 10, "~ω", like Δεῑ;
    stem aorist_ind: "~υσ- like χεω";
    stem aorist_ind_pass: "~υσθη- like ἐχω";
    ind_act_2: "~ῑς", "~ῑτον", "~ῑτε";
    ind_act_3: "~ῑ", _, _;
    ind_mid_2: _, "~ῑσθον", "~ῑσθε";
    ind_mid_3: "~ῑται", _, _;
    imp_act_2: "~ῑ", _, _;
    imp_act_3: "~ιτω", "~ιτων", _;
    imp_mid_3: "~ισθω", "~ισθων", "~εσθων";
    imp_pass_3: _, _, "~εσθων";
    imperfect_act_2: "ἐ~ις", "ἐ~ῑτον", "ἐ~ῑτε";
    imperfect_act_3: "ἐ~ι", "ἐ~ιτην", _;
    imperfect_mid_2: _, "ἐ~ῑσθον", "ἐ~ῑσθε";
    imperfect_mid_3: "ἐ~ῑτο", "ἐ~ισθην", _;
    future_ind_act_1: "~υσω", _, "~υσομεν";
    future_ind_act_2: "~υσεις", "~υσετον", "~υσετε";
    future_ind_act_3: "~υσει", _, "~υσουσῐ";
    future_ind_mid_1: "~υσομαι", _, "~υσομεθᾰ";
    future_ind_mid_2: "~υσῃ", "~υσεσθον", "~υσεσθε";
    future_ind_mid_3: "~υσεται", _, "~υσονται";
    future_opt_act_1: "~υσοιμῐ", _, "~υσοιμεν";
    future_opt_act_2: "~υσοις", "~υσοιτον", "~υσοιτε";
    future_opt_act_3: "~υσοι", "~υσοιτην", "~υσοιεν";
    future_opt_mid_1: "~υσοιμην", _, "~υσοιμεθᾰ";
    future_opt_mid_2: "~υσοιο", "~υσοισθον", "~υσοισθε";
    future_opt_mid_3: "~υσοιτο", "~υσοισθην", "~υσοιντο";
    aorist_subj_act_1: "~υσω", _, "~υσωμεν";
    aorist_subj_act_2: "~υσῃς", "~υσητον", "~υσητε";
    aorist_subj_act_3: "~υσῃ", _, "~υσωσῐ";
    aorist_subj_pass_1: "~υσθω", _, "~υσθωμεν";
    aorist_subj_pass_2: "~υσθῃς", "~υσθητον", "~υσθητε";
    aorist_subj_pass_3: "~υσθῃ", _, "~υσθωσῐ";
    aorist_opt_act_1: "~υσαιμῐ", _, "~υσαιμεν";
    aorist_opt_act_2: "~υσειᾰς", "~υσαιτον", "~υσαιτε";
    aorist_opt_act_3: "~υσειε", "~υσαιτην", "~υσειᾰν";
    aorist_opt_pass_1: "~υσθειην", _, "~υσθεῑμεν";
    aorist_opt_pass_2: "~υσθειης", "~υσθεῑτον", "~υσθεῑτε";
    aorist_opt_pass_3: "~υσθειη", "~υσθειτην", "~υσθεῑεν";
    aorist_imp_act_2: "~ῡσον", "~υσᾰτον", "~υσᾰτε";
    aorist_imp_act_3: "~υσᾰτω", "~υσᾰτων", "~υσᾰντων";
    aorist_imp_pass_2: "~υσθητῐ", _, _;
    aorist_imp_pass_3: "~υσθητω", "~υσθητων", "~υσθεντων";
    perfect_ind_act_1: "%ε~υκᾰ", _, "%ε~υκᾰμεν";
    perfect_ind_act_2: "%ε~υκᾰς", "%ε~υκᾰτον", "%ε~υκᾰτε";
    perfect_ind_act_3: "%ε~υκε", _, "%ε~υκᾱσῐ";
    perfect_ind_mid_1: "%ε~υσμαι", _, "%ε~υσμεθᾰ";
    perfect_ind_mid_2: "%ε~υσαι", "%ε~υσθον", "%ε~υσθε";
    perfect_ind_mid_3: "%ε~υσται", _, "%ε~υσᾰται";
    perfect_subj_act_1: "%ε~υκω", _, "%ε~υκωμεν";
    perfect_subj_act_2: "%ε~υκῃς", "%ε~υκητον", "%ε~υκητε";
    perfect_subj_act_3: "%ε~υκῃ", _, "%ε~υκωσῐ";
    perfect_opt_act_1: "%ε~υκοιμῐ", _, "%ε~υκοιμεν";
    perfect_opt_act_2: "%ε~υκοις", "%ε~υκοιτον", "%ε~υκοιτε";
    perfect_opt_act_3: "%ε~υκοι", "%ε~υκοιτην", "%ε~υκοιεν";
    perfect_imp_act_2: _, "%ε~υκετον", "%ε~υκετε";
    perfect_imp_act_3: "%ε~υκετω", "%ε~υκετων", "%ε~υκοντων";
    perfect_imp_mid_2: "%ε~υσο", _, _;
    perfect_imp_mid_3: "%ε~υσθω", "%ε~υσθων", _;
    infinitive_pres: "~ῑν", "~ῑσθαι", _;
    infinitive_fut: "~υσειν", "~υσεσθαι", _;
    infinitive_aor: "~ῡσαι", _, "~υσθηναι";
    infinitive_perf: "%ε~υκεναι", "%ε~ῡσθαι", _;
    participle_fut: "~υσων", "~υσομενος", _;
    participle_aor: "~υσᾱς", _, "~υσθεις";
    participle_perf: "%ε~υκως", "%ε~υσμενος", _;
    ),
    // Ἀφῑημῐ: 10 words (ἀφῑημῐ, σῠνῑημῐ, ἐπᾰνῐημῐ, προϊημῐ, ἀνιημῐ, ξῠνῑημῐ). Like Οἰομαι, except in 171 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀφῑημῐ", 10, "~ῑημῐ", like Οἰομαι as AORIST_AS_PRESENT;
    stem aorist_ind: "~ηκ- like εὐλογεω";
    ind_act_1: "~ῑημῐ", _, "~ῑεμεν";
    ind_act_2: "~ῑης", "~ῑετον", "~ῑετε";
    ind_act_3: "~ῑησῐ|~ῑησῐν", _, "~ῑᾱσι|~ῑᾱσιν";
    ind_mid_1: "~ῑεμαι", _, "~ῑεμεθᾰ";
    ind_mid_2: "~ῑεσαι", "~ῑεσθον", "~ῑεσθε";
    ind_mid_3: "~ῑεται", _, "~ῑενται";
    subj_act_1: "~ῑω", _, "~ῑωμεν";
    subj_act_2: "~ῑῃς", "~ῑητον", "~ῑητε";
    subj_act_3: "~ῑῃ", _, "~ῑωσῐ|~ῑωσῐν";
    subj_mid_1: "~ῑωμαι", _, "~ῑωμεθᾰ";
    subj_mid_2: derived, "~ῑησθον", "~ῑησθε";
    subj_mid_3: "~ῑηται", _, "~ῑωνται";
    subj_pass_2: derived, _, _;
    opt_act_1: "~ῑειην", _, "~ῑεῑμεν|~ῑειημεν";
    opt_act_2: "~ῑειης", "~ῑεῑτον|~ῑειητον", "~ῑεῑτε|~ῑειητε";
    opt_act_3: "~ῑειη", "~ῑειτην|~ῑειητην", "~ῑεῑεν|~ῑειησᾰν";
    opt_mid_1: "~ῑειμην", _, "~ῑειμεθᾰ|~ῑοιμεθᾰ";
    opt_mid_2: "~ῑεῑο", "~ῑεῑσθον|~ῑοῑσθον", "~ῑεῑσθε|~ῑοῑσθε";
    opt_mid_3: "~ῑεῑτο|~ῑοῑτο", "~ῑεισθην|~ῑοισθην", "~ῑεῑντο|~ῑοῑντο";
    imp_act_2: "~ῑει", _, _;
    imp_act_3: "~ῑετω", "~ῑετων", "~ῑεντων";
    imp_mid_2: "~ῑεσο|~ῑου", _, _;
    imp_mid_3: "~ῑεσθω", "~ῑεσθων", _;
    imperfect_act_1: "~ῑην", _, "~ῑεμεν";
    imperfect_act_2: "~ῑεις", "~ῑετον", "~ῑετε";
    imperfect_act_3: "~ῑει", "~ῑετην", "~ῑεσᾰν";
    imperfect_mid_1: "~ῑεμην", _, "~ῑεμεθᾰ";
    imperfect_mid_2: "~ῑεσο", "~ῑεσθον", "~ῑεσθε";
    imperfect_mid_3: "~ῑετο", "~ῑεσθην", "~ῑεντο";
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", "~ησετε";
    future_ind_act_3: "~ησει", _, "~ησουσῐ|~ησουσῐν";
    future_ind_mid_2: "~ησῃ|~ησει", _, _;
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: "~ησοις", "~ησοιτον", "~ησοιτε";
    future_opt_act_3: "~ησοι", "~ησοιτην", "~ησοιεν";
    aorist_subj_act_1: "~ω", _, "~ωμεν";
    aorist_subj_act_2: "~ῃς", "~ητον", "~ητε";
    aorist_subj_act_3: "~ῃ", _, "~ωσῐ";
    aorist_subj_mid_2: derived, _, _;
    aorist_subj_pass_2: derived, _, _;
    aorist_opt_act_1: "~ειην", _, "~εῑμεν";
    aorist_opt_act_2: "~ειης", "~εῑτον", "~εῑτε";
    aorist_opt_act_3: "~ειη", "~ειτην", "~εῑεν";
    aorist_opt_mid_1: "~ειμην", _, "~ειμεθᾰ";
    aorist_opt_mid_2: "~εῑο", "~εῑσθον", "~εῑσθε";
    aorist_opt_mid_3: "~εῑτο", "~εισθην", "~εῑντο";
    aorist_imp_act_2: "~ες", "~ετον", "~ετε";
    aorist_imp_act_3: "~ετω", "~ετων", "~εντων";
    aorist_imp_mid_2: "~οῡ", "~εσθον", "~εσθε";
    perfect_ind_act_1: "~εῑκᾰ", _, "~εικᾰμεν";
    perfect_ind_act_2: "~εῑκᾰς", "~εικᾰτον", "~εικᾰτε";
    perfect_ind_act_3: "~εῑκε", _, "~εικᾱσῐ";
    perfect_ind_mid_1: "~εῑμαι", _, "~ειμεθᾰ";
    perfect_ind_mid_2: "~εῑσαι", "~εῑσθον", "~εῑσθε";
    perfect_ind_mid_3: "~εῑται", _, "~εῑνται";
    perfect_subj_act_1: "~εικω", _, "~εικωμεν";
    perfect_subj_act_2: "~εικῃς", "~εικητον", "~εικητε";
    perfect_subj_act_3: "~εικῃ", _, "~εικωσῐ";
    perfect_opt_act_1: "~εικοιμῐ", _, "~εικοιμεν";
    perfect_opt_act_2: "~εικοις", "~εικοιτον", "~εικοιτε";
    perfect_opt_act_3: "~εικοι", "~εικοιτην", "~εικοιεν";
    perfect_imp_act_2: _, "~εικετον", "~εικετε";
    perfect_imp_act_3: "~εικετω", "~εικετων", "~εικοντων";
    perfect_imp_mid_2: "~εῑσο", _, _;
    perfect_imp_mid_3: "~εισθω", "~εισθων", _;
    infinitive_pres: "~ῑεναι", "~ῑεσθαι", _;
    infinitive_fut: "~ησειν", _, _;
    infinitive_aor: "~εῑναι", "~εσθαι", derived;
    infinitive_perf: "~εικεναι", "~εῑσθαι", _;
    participle_pres: "~ῑεις", "~ῑεμενος", _;
    participle_fut: "~ησων", _, _;
    participle_aor: "~εις", "~εμενος", derived;
    participle_perf: "~εικως", "~ειμενος", _;
    ),
    // Μᾰντευομαι: 9 words (μᾰντευομαι, κουρευομαι, ὀττευομαι, λοιμευομαι, ἀστειευομαι, εἰρωνευομαι). Like Βουλευω, except in 59 cells: ind_act_1 sg, ind_mid_2 sg, subj_mid_2 sg, subj_pass_2 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("μᾰντευομαι", 9, "~υομαι", like Βουλευω as ACT_AS_PASS;
    ind_act_1: "~υομαι", _, _;
    ind_mid_2: "~υῃ", _, _;
    subj_mid_2: "~υῃ", _, _;
    subj_pass_2: "~υῃ", _, _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_2: "~υσῃ", _, _;
    future_ind_pass_2: "~υθησῃ", _, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_subj_act_1: derived, _, derived;
    aorist_subj_act_2: derived, derived, derived;
    aorist_subj_act_3: derived, _, derived;
    aorist_subj_mid_2: "~υσῃ", _, _;
    aorist_subj_pass_3: _, _, "~υθωσῐ";
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_pass_1: _, _, "~υθεῑμεν";
    aorist_opt_pass_2: _, "~υθεῑτον", "~υθεῑτε";
    aorist_opt_pass_3: _, "~υθειτην", "~υθεῑεν";
    aorist_imp_act_2: derived, _, _;
    aorist_imp_act_3: derived, derived, derived;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    participle_fut: derived, _, _;
    participle_aor: derived, _, _;
    ),
    // Ἐνᾰντῐοομαι: 9 words (στραγγᾰλοομαι, χειροομαι, ἀπῐχθῠοομαι, κᾰτοικειοομαι, ἐνᾰντῐοομαι, δεξῐοομαι). Like Γῡροω, except in 51 cells: ind_act_1 sg, subj_mid_2 sg, subj_pass_2 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, ….
    conjugation!("ἐνᾰντῐοομαι", 9, "~οομαι", like Γῡροω as ACT_AS_PASS;
    ind_act_1: "~οομαι", _, _;
    subj_mid_2: "~οῑ", _, _;
    subj_pass_2: "~οῑ", _, _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_2: "~ωσῃ", _, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_subj_act_1: derived, _, derived;
    aorist_subj_act_2: derived, derived, derived;
    aorist_subj_act_3: derived, _, derived;
    aorist_subj_mid_2: "~ωσῃ", _, _;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_imp_act_2: derived, _, _;
    aorist_imp_act_3: derived, derived, derived;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    participle_fut: derived, _, _;
    participle_aor: derived, _, _;
    ),
    // Κᾰταφρονεω: 9 words (κᾰταφρονεω, κᾰταργεω, κᾰτηγορεω, λοιπογρᾰφεω, τερᾰτολογεω, τερᾰτουργεω). Like Πεμπω, except in 227 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("κᾰταφρονεω", 9, "~ᾰ~εω", like Πεμπω;
    stem aorist_ind: "~α~ησ- like ποιεω";
    stem aorist_ind_pass: "~α~ηθη- like ἐχω";
    stem pluperfect_act: "~α~ηκε- like ἐχω";
    stem pluperfect: "~α~η- like πειρᾰω";
    ind_act_1: "~α~ω", _, "~α~οῡμεν";
    ind_act_2: "~α~εῑς", "~α~εῑτον", "~α~εῑτε";
    ind_act_3: "~α~εῑ", _, "~α~οῡσῐ|~α~οῡσῐν";
    ind_mid_1: "~α~οῡμαι", _, "~α~ουμεθᾰ";
    ind_mid_2: "~α~εῑ|~α~ῃ", "~α~εῑσθον", "~α~εῑσθε";
    ind_mid_3: "~α~εῑται", _, "~α~οῡνται";
    subj_act_1: "~α~ω", _, "~α~ωμεν";
    subj_act_2: "~α~ῃς", "~α~ητον", "~α~ητε";
    subj_act_3: "~α~ῃ", _, "~α~ωσῐ|~α~ωσῐν";
    subj_mid_1: "~α~ωμαι", _, "~α~ωμεθᾰ";
    subj_mid_2: _, "~α~ησθον", "~α~ησθε";
    subj_mid_3: "~α~ηται", _, "~α~ωνται";
    opt_act_1: "~α~οιην|~α~οῑμῐ", _, "~α~οῑμεν|~α~οιημεν";
    opt_act_2: "~α~οιης|~α~οῑς", "~α~οῑτον|~α~οιητον", "~α~οῑτε|~α~οιητε";
    opt_act_3: "~α~οιη|~α~οῑ", "~α~οιτην|~α~οιητην", "~α~οῑεν|~α~οιησᾰν";
    opt_mid_1: "~α~οιμην", _, "~α~οιμεθᾰ";
    opt_mid_2: "~α~οῑο", "~α~οῑσθον", "~α~οῑσθε";
    opt_mid_3: "~α~οῑτο", "~α~οισθην", "~α~οῑντο";
    imp_act_2: "~α~ει", _, _;
    imp_act_3: "~α~ειτω", "~α~ειτων", "~α~ουντων";
    imp_mid_2: "~α~οῡ", _, _;
    imp_mid_3: "~α~εισθω", "~α~εισθων", _;
    imperfect_act_1: "ἐ~α~ουν", _, "ἐ~α~οῡμεν";
    imperfect_act_2: "ἐ~α~εις", "ἐ~α~εῑτον", "ἐ~α~εῑτε";
    imperfect_act_3: "ἐ~α~ει", "ἐ~α~ειτην", "ἐ~α~ουν";
    imperfect_mid_1: "ἐ~α~ουμην", _, "ἐ~α~ουμεθᾰ";
    imperfect_mid_2: "ἐ~α~οῡ", "ἐ~α~εῑσθον", "ἐ~α~εῑσθε";
    imperfect_mid_3: "ἐ~α~εῑτο", "ἐ~α~εισθην", "ἐ~α~οῡντο";
    future_ind_act_1: "~α~ησω", _, "~α~ησομεν";
    future_ind_act_2: "~α~ησεις", "~α~ησετον", "~α~ησετε";
    future_ind_act_3: "~α~ησει", _, "~α~ησουσῐ|~α~ησουσῐν";
    future_ind_mid_1: "~α~ησομαι", _, "~α~ησομεθᾰ";
    future_ind_mid_2: "~α~ησῃ|~α~ησει", "~α~ησεσθον", "~α~ησεσθε";
    future_ind_mid_3: "~α~ησεται", _, "~α~ησονται";
    future_ind_pass_1: "~α~ηθησομαι", _, "~α~ηθησομεθᾰ";
    future_ind_pass_2: "~α~ηθησῃ|~α~ηθησει", "~α~ηθησεσθον", "~α~ηθησεσθε";
    future_ind_pass_3: "~α~ηθησεται", _, "~α~ηθησονται";
    future_opt_act_1: "~α~ησοιμῐ", _, "~α~ησοιμεν";
    future_opt_act_2: "~α~ησοις", "~α~ησοιτον", "~α~ησοιτε";
    future_opt_act_3: "~α~ησοι", "~α~ησοιτην", "~α~ησοιεν";
    future_opt_mid_1: "~α~ησοιμην", _, "~α~ησοιμεθᾰ";
    future_opt_mid_2: "~α~ησοιο", "~α~ησοισθον", "~α~ησοισθε";
    future_opt_mid_3: "~α~ησοιτο", "~α~ησοισθην", "~α~ησοιντο";
    future_opt_pass_1: "~α~ηθησοιμην", _, "~α~ηθησοιμεθᾰ";
    future_opt_pass_2: "~α~ηθησοιο", "~α~ηθησοισθον", "~α~ηθησοισθε";
    future_opt_pass_3: "~α~ηθησοιτο", "~α~ηθησοισθην", "~α~ηθησοιντο";
    aorist_subj_act_1: "~α~ησω", _, "~α~ησωμεν";
    aorist_subj_act_2: "~α~ησῃς", "~α~ησητον", "~α~ησητε";
    aorist_subj_act_3: "~α~ησῃ", _, "~α~ησωσῐ|~α~ησωσῐν";
    aorist_subj_mid_1: "~α~ησωμαι", _, "~α~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~α~ησησθον", "~α~ησησθε";
    aorist_subj_mid_3: "~α~ησηται", _, "~α~ησωνται";
    aorist_subj_pass_1: "~α~ηθω", _, "~α~ηθωμεν";
    aorist_subj_pass_2: "~α~ηθῃς", "~α~ηθητον", "~α~ηθητε";
    aorist_subj_pass_3: "~α~ηθῃ", _, "~α~ηθωσῐ|~α~ηθωσῐν";
    aorist_opt_act_1: "~α~ησαιμῐ", _, "~α~ησαιμεν";
    aorist_opt_act_2: "~α~ησειᾰς|~α~ησαις", "~α~ησαιτον", "~α~ησαιτε";
    aorist_opt_act_3: "~α~ησειε|~α~ησειεν|~α~ησαι", "~α~ησαιτην", "~α~ησειᾰν|~α~ησαιεν";
    aorist_opt_mid_1: "~α~ησαιμην", _, "~α~ησαιμεθᾰ";
    aorist_opt_mid_2: "~α~ησαιο", "~α~ησαισθον", "~α~ησαισθε";
    aorist_opt_mid_3: "~α~ησαιτο", "~α~ησαισθην", "~α~ησαιντο";
    aorist_opt_pass_1: "~α~ηθειην", _, "~α~ηθεῑμεν|~α~ηθειημεν";
    aorist_opt_pass_2: "~α~ηθειης", "~α~ηθεῑτον|~α~ηθειητον", "~α~ηθεῑτε|~α~ηθειητε";
    aorist_opt_pass_3: "~α~ηθειη", "~α~ηθειτην|~α~ηθειητην", "~α~ηθεῑεν|~α~ηθειησᾰν";
    aorist_imp_act_2: "~α~ησον", "~α~ησᾰτον", "~α~ησᾰτε";
    aorist_imp_act_3: "~α~ησᾰτω", "~α~ησᾰτων", "~α~ησᾰντων";
    aorist_imp_mid_2: "~α~ησαι", "~α~ησᾰσθον", "~α~ησᾰσθε";
    aorist_imp_mid_3: "~α~ησᾰσθω", "~α~ησᾰσθων", _;
    aorist_imp_pass_2: "~α~ηθητῐ", _, _;
    aorist_imp_pass_3: "~α~ηθητω", "~α~ηθητων", "~α~ηθεντων";
    perfect_ind_act_1: "%ε~α~ηκᾰ", _, "%ε~α~ηκᾰμεν";
    perfect_ind_act_2: "%ε~α~ηκᾰς", "%ε~α~ηκᾰτον", "%ε~α~ηκᾰτε";
    perfect_ind_act_3: "%ε~α~ηκε", _, "%ε~α~ηκᾱσῐ";
    perfect_ind_mid_1: "%ε~α~ημαι", _, "%ε~α~ημεθᾰ";
    perfect_ind_mid_2: "%ε~α~ησαι", "%ε~α~ησθον", "%ε~α~ησθε";
    perfect_ind_mid_3: "%ε~α~ηται", _, "%ε~α~ηνται";
    perfect_subj_act_1: "%ε~α~ηκω", _, "%ε~α~ηκωμεν";
    perfect_subj_act_2: "%ε~α~ηκῃς", "%ε~α~ηκητον", "%ε~α~ηκητε";
    perfect_subj_act_3: "%ε~α~ηκῃ", _, "%ε~α~ηκωσῐ";
    perfect_opt_act_1: "%ε~α~ηκοιμῐ", _, "%ε~α~ηκοιμεν";
    perfect_opt_act_2: "%ε~α~ηκοις", "%ε~α~ηκοιτον", "%ε~α~ηκοιτε";
    perfect_opt_act_3: "%ε~α~ηκοι", "%ε~α~ηκοιτην", "%ε~α~ηκοιεν";
    perfect_imp_act_2: _, "%ε~α~ηκετον", "%ε~α~ηκετε";
    perfect_imp_act_3: "%ε~α~ηκετω", "%ε~α~ηκετων", "%ε~α~ηκοντων";
    perfect_imp_mid_2: "%ε~α~ησο", _, _;
    perfect_imp_mid_3: "%ε~α~ησθω", "%ε~α~ησθων", _;
    infinitive_pres: "~α~εῑν", "~α~εῑσθαι", _;
    infinitive_fut: "~α~ησειν", "~α~ησεσθαι", "~α~ηθησεσθαι";
    infinitive_aor: "~α~ησαι", "~α~ησᾰσθαι", "~α~ηθηναι";
    infinitive_perf: "%ε~α~ηκεναι", "%ε~α~ησθαι", _;
    participle_pres: "~α~ων", "~α~ουμενος", _;
    participle_fut: "~α~ησων", "~α~ησομενος", "~α~ηθησομενος";
    participle_aor: "~α~ησᾱς", "~α~ησᾰμενος", "~α~ηθεις";
    participle_perf: "%ε~α~ηκως", "%ε~α~ημενος", _;
    ),
    // Εἰσερχομαι: 9 words (ἐξερχομαι, εἰσερχομαι, ἀπερχομαι, προσερχομαι, δῐερχομαι, πᾰρερχομαι). Like Μῑγνῡμῐ, except in 100 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("εἰσερχομαι", 9, "~ερχομαι", like Μῑγνῡμῐ as SG_AS_PL;
    stem imperfect: "~ηρχ- like ὑποκοριζομαι";
    stem aorist_ind: "~ηλθ- like εἰσερχομαι";
    stem pluperfect_act: "~εληλῠθε- like ἀνῐστημῐ";
    ind_act_1: "~ερχομαι", _, _;
    ind_mid_1: "~ερχομαι", _, "~ερχομεθᾰ";
    ind_mid_2: "~ερχῃ|~ερχει", "~ερχεσθον", "~ερχεσθε";
    ind_mid_3: "~ερχεται", _, "~ερχονται";
    subj_mid_1: "~ερχωμαι", _, "~ερχωμεθᾰ";
    subj_mid_2: "~ερχῃ", "~ερχησθον", "~ερχησθε";
    subj_mid_3: "~ερχηται", _, "~ερχωνται";
    subj_pass_2: "~ερχῃ", _, _;
    opt_mid_1: "~ερχοιμην", _, "~ερχοιμεθᾰ";
    opt_mid_2: "~ερχοιο", "~ερχοισθον", "~ερχοισθε";
    opt_mid_3: "~ερχοιτο", "~ερχοισθην", "~ερχοιντο";
    imp_mid_2: "~ερχου", _, _;
    imp_mid_3: "~ερχεσθω", "~ερχεσθων", _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    future_ind_mid_1: "~ελευσομαι", _, "~ελευσομεθᾰ";
    future_ind_mid_2: "~ελευσῃ|~ελευσει", "~ελευσεσθον", "~ελευσεσθε";
    future_ind_mid_3: "~ελευσεται", _, "~ελευσονται";
    future_opt_mid_1: "~ελευσοιμην", _, "~ελευσοιμεθᾰ";
    future_opt_mid_2: "~ελευσοιο", "~ελευσοισθον", "~ελευσοισθε";
    future_opt_mid_3: "~ελευσοιτο", "~ελευσοισθην", "~ελευσοιντο";
    aorist_subj_act_1: "~ελθω", _, "~ελθωμεν";
    aorist_subj_act_2: "~ελθῃς", "~ελθητον", "~ελθητε";
    aorist_subj_act_3: "~ελθῃ", _, "~ελθωσῐ|~ελθωσῐν";
    aorist_opt_act_1: "~ελθοιμῐ", _, "~ελθοιμεν";
    aorist_opt_act_2: "~ελθοις", "~ελθοιτον", "~ελθοιτε";
    aorist_opt_act_3: "~ελθοι", "~ελθοιτην", "~ελθοιεν";
    aorist_imp_act_2: "~ελθε", "~ελθετον", "~ελθετε";
    aorist_imp_act_3: "~ελθετω", "~ελθετων", "~ελθοντων";
    perfect_ind_act_1: "~εληλῠθᾰ", _, "~εληλυμεν";
    perfect_ind_act_2: "~εληλῠθᾰς", "~εληλῠθᾰτον", "~εληλυτε";
    perfect_ind_act_3: "~εληλῠθε", _, "~εληλῠθᾱσῐ";
    perfect_subj_act_1: "~εληλῠθω", _, "~εληλῠθωμεν";
    perfect_subj_act_2: "~εληλῠθῃς", "~εληλῠθητον", "~εληλῠθητε";
    perfect_subj_act_3: "~εληλῠθῃ", _, "~εληλῠθωσῐ";
    perfect_opt_act_1: "~εληλῠθοιμῐ", _, "~εληλῠθοιμεν";
    perfect_opt_act_2: "~εληλῠθοις", "~εληλῠθοιτον", "~εληλῠθοιτε";
    perfect_opt_act_3: "~εληλῠθοι", "~εληλῠθοιτην", "~εληλῠθοιεν";
    perfect_imp_act_2: _, "~εληλῠθετον", "~εληλῠθετε";
    perfect_imp_act_3: "~εληλῠθετω", "~εληλῠθετων", "~εληλῠθοντων";
    infinitive_pres: _, "~ερχεσθαι", _;
    infinitive_fut: _, "~ελευσεσθαι", _;
    infinitive_aor: "~ελθεῑν", _, _;
    infinitive_perf: "~εληλῠθεναι", _, _;
    participle_pres: _, "~ερχομενος", _;
    participle_fut: _, "~ελευσομενος", _;
    participle_aor: "~ελθων", _, _;
    participle_perf: "~εληλῠθως", _, _;
    ),
    // Ἐπᾰϊω: 9 words (ὑπᾰρχω, δῐᾰνῠω, ἐξᾰρχω, ἐξᾰρτῡω, ἀπᾰρεσκω, ἐπᾰρχω). Like Πεμπω, except in 235 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐπᾰϊω", 9, "~ᾰ~ω", like Πεμπω;
    stem aorist_ind: "~η~σ- like εὐλογεω";
    stem aorist_ind_pass: "~η~σθη- like ἐλπω";
    stem pluperfect_act: "~η~κε- like ἀνῐστημῐ";
    ind_act_1: "~ᾰ~ω", _, "~ᾰ~ομεν";
    ind_act_2: "~ᾰ~εις", "~ᾰ~ετον", "~ᾰ~ετε";
    ind_act_3: "~ᾰ~ει", _, "~ᾰ~ουσῐ|~ᾰ~ουσῐν";
    ind_mid_1: "~ᾰ~ομαι", _, "~ᾰ~ομεθᾰ";
    ind_mid_2: "~ᾰ~ῃ|~ᾰ~ει", "~ᾰ~εσθον", "~ᾰ~εσθε";
    ind_mid_3: "~ᾰ~εται", _, "~ᾰ~ονται";
    subj_act_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    subj_act_2: "~ᾰ~ῃς", "~ᾰ~ητον", "~ᾰ~ητε";
    subj_act_3: "~ᾰ~ῃ", _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ησθον", "~ᾰ~ησθε";
    subj_mid_3: "~ᾰ~ηται", _, "~ᾰ~ωνται";
    opt_act_1: "~ᾰ~οιμῐ", _, "~ᾰ~οιμεν";
    opt_act_2: "~ᾰ~οις", "~ᾰ~οιτον", "~ᾰ~οιτε";
    opt_act_3: "~ᾰ~οι", "~ᾰ~οιτην", "~ᾰ~οιεν";
    opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~οιο", "~ᾰ~οισθον", "~ᾰ~οισθε";
    opt_mid_3: "~ᾰ~οιτο", "~ᾰ~οισθην", "~ᾰ~οιντο";
    imp_act_2: "~ᾰ~ε", _, _;
    imp_act_3: "~ᾰ~ετω", "~ᾰ~ετων", "~ᾰ~οντων";
    imp_mid_2: "~ᾰ~ου", _, _;
    imp_mid_3: "~ᾰ~εσθω", "~ᾰ~εσθων", _;
    imperfect_act_1: "~η~ον", _, "~η~ομεν";
    imperfect_act_2: "~η~ες", "~η~ετον", "~η~ετε";
    imperfect_act_3: "~η~ε|~η~εν", "~η~ετην", "~η~ον";
    imperfect_mid_1: "~η~ομην", _, "~η~ομεθᾰ";
    imperfect_mid_2: "~η~ου", "~η~εσθον", "~η~εσθε";
    imperfect_mid_3: "~η~ετο", "~η~εσθην", "~η~οντο";
    future_ind_act_1: "~ᾰ~σω", _, "~ᾰ~σομεν";
    future_ind_act_2: "~ᾰ~σεις", "~ᾰ~σετον", "~ᾰ~σετε";
    future_ind_act_3: "~ᾰ~σει", _, "~ᾰ~σουσῐ";
    future_ind_mid_1: "~ᾰ~σομαι", _, "~ᾰ~σομεθᾰ";
    future_ind_mid_2: "~ᾰ~σῃ", "~ᾰ~σεσθον", "~ᾰ~σεσθε";
    future_ind_mid_3: "~ᾰ~σεται", _, "~ᾰ~σονται";
    future_ind_pass_1: "~ᾰ~σθησομαι", _, "~ᾰ~σθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~σθησῃ", "~ᾰ~σθησεσθον", "~ᾰ~σθησεσθε";
    future_ind_pass_3: "~ᾰ~σθησεται", _, "~ᾰ~σθησονται";
    future_opt_act_1: "~ᾰ~σοιμῐ", _, "~ᾰ~σοιμεν";
    future_opt_act_2: "~ᾰ~σοις", "~ᾰ~σοιτον", "~ᾰ~σοιτε";
    future_opt_act_3: "~ᾰ~σοι", "~ᾰ~σοιτην", "~ᾰ~σοιεν";
    future_opt_mid_1: "~ᾰ~σοιμην", _, "~ᾰ~σοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~σοιο", "~ᾰ~σοισθον", "~ᾰ~σοισθε";
    future_opt_mid_3: "~ᾰ~σοιτο", "~ᾰ~σοισθην", "~ᾰ~σοιντο";
    future_opt_pass_1: "~ᾰ~σθησοιμην", _, "~ᾰ~σθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~σθησοιο", "~ᾰ~σθησοισθον", "~ᾰ~σθησοισθε";
    future_opt_pass_3: "~ᾰ~σθησοιτο", "~ᾰ~σθησοισθην", "~ᾰ~σθησοιντο";
    aorist_subj_act_1: "~ᾰ~σω", _, "~ᾰ~σωμεν";
    aorist_subj_act_2: "~ᾰ~σῃς", "~ᾰ~σητον", "~ᾰ~σητε";
    aorist_subj_act_3: "~ᾰ~σῃ", _, "~ᾰ~σωσῐ|~ᾰ~σωσῐν";
    aorist_subj_mid_1: "~ᾰ~σωμαι", _, "~ᾰ~σωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰ~σησθον", "~ᾰ~σησθε";
    aorist_subj_mid_3: "~ᾰ~σηται", _, "~ᾰ~σωνται";
    aorist_subj_pass_1: "~ᾰ~σθω", _, "~ᾰ~σθωμεν";
    aorist_subj_pass_2: "~ᾰ~σθῃς", "~ᾰ~σθητον", "~ᾰ~σθητε";
    aorist_subj_pass_3: "~ᾰ~σθῃ", _, "~ᾰ~σθωσῐ";
    aorist_opt_act_1: "~ᾰ~σαιμῐ", _, "~ᾰ~σαιμεν";
    aorist_opt_act_2: "~ᾰ~σειᾰς|~ᾰ~σαις", "~ᾰ~σαιτον", "~ᾰ~σαιτε";
    aorist_opt_act_3: "~ᾰ~σειε|~ᾰ~σειεν|~ᾰ~σαι", "~ᾰ~σαιτην", "~ᾰ~σειᾰν|~ᾰ~σαιεν";
    aorist_opt_mid_1: "~ᾰ~σαιμην", _, "~ᾰ~σαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~σαιο", "~ᾰ~σαισθον", "~ᾰ~σαισθε";
    aorist_opt_mid_3: "~ᾰ~σαιτο", "~ᾰ~σαισθην", "~ᾰ~σαιντο";
    aorist_opt_pass_1: "~ᾰ~σθειην", _, "~ᾰ~σθεῑμεν";
    aorist_opt_pass_2: "~ᾰ~σθειης", "~ᾰ~σθεῑτον", "~ᾰ~σθεῑτε";
    aorist_opt_pass_3: "~ᾰ~σθειη", "~ᾰ~σθειτην", "~ᾰ~σθεῑεν";
    aorist_imp_act_2: "~ᾰ~σον", "~ᾰ~σᾰτον", "~ᾰ~σᾰτε";
    aorist_imp_act_3: "~ᾰ~σᾰτω", "~ᾰ~σᾰτων", "~ᾰ~σᾰντων";
    aorist_imp_mid_2: "~ᾰ~σαι", "~ᾰ~σᾰσθον", "~ᾰ~σᾰσθε";
    aorist_imp_mid_3: "~ᾰ~σᾰσθω", "~ᾰ~σᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~σθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~σθητω", "~ᾰ~σθητων", "~ᾰ~σθεντων";
    perfect_ind_act_1: "~η~κᾰ", _, "~η~κᾰμεν";
    perfect_ind_act_2: "~η~κᾰς", "~η~κᾰτον", "~η~κᾰτε";
    perfect_ind_act_3: "~η~κε", _, "~η~κᾱσῐ";
    perfect_ind_mid_1: "~η~σμαι", _, "~η~σμεθᾰ";
    perfect_ind_mid_2: "~η~σαι", "~η~σθον", "~η~σθε";
    perfect_ind_mid_3: "~η~σται", _, "~η~τᾰται";
    perfect_subj_act_1: "~η~κω", _, "~η~κωμεν";
    perfect_subj_act_2: "~η~κῃς", "~η~κητον", "~η~κητε";
    perfect_subj_act_3: "~η~κῃ", _, "~η~κωσῐ";
    perfect_opt_act_1: "~η~κοιμῐ", _, "~η~κοιμεν";
    perfect_opt_act_2: "~η~κοις", "~η~κοιτον", "~η~κοιτε";
    perfect_opt_act_3: "~η~κοι", "~η~κοιτην", "~η~κοιεν";
    perfect_imp_act_2: _, "~η~κετον", "~η~κετε";
    perfect_imp_act_3: "~η~κετω", "~η~κετων", "~η~κοντων";
    perfect_imp_mid_2: "~η~σο", _, _;
    perfect_imp_mid_3: "~η~σθω", "~η~σθων", _;
    pluperfect_mid_1: "~η~σμην", _, "~η~σμεθᾰ";
    pluperfect_mid_2: "~η~σο", "~η~σθον", "~η~σθε";
    pluperfect_mid_3: "~η~στο", "~η~σθην", "~η~τᾰτο";
    infinitive_pres: "~ᾰ~ειν", "~ᾰ~εσθαι", _;
    infinitive_fut: "~ᾰ~σειν", "~ᾰ~σεσθαι", "~ᾰ~σθησεσθαι";
    infinitive_aor: "~ᾰ~σαι", "~ᾰ~σᾰσθαι", "~ᾰ~σθηναι";
    infinitive_perf: "~η~κεναι", "~η~σθαι", _;
    participle_pres: "~ᾰ~ων", "~ᾰ~ομενος", _;
    participle_fut: "~ᾰ~σων", "~ᾰ~σομενος", "~ᾰ~σθησομενος";
    participle_aor: "~ᾰ~σᾱς", "~ᾰ~σᾰμενος", "~ᾰ~σθεις";
    participle_perf: "~η~κως", "~η~σμενος", _;
    ),
    // Ψῑλοω: 8 words (ἱδροω, στενοω, πτεροω, ψῑλοω, κεραυνοω, φορτοω). Like Γῡροω, except in 58 cells: future_ind_act_3 pl, future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, ….
    conjugation!("ψῑλοω", 8, "~οω", like Γῡροω;
    stem pluperfect_act: "~ωκ- like δῐκεω";
    future_ind_act_3: _, _, "~ωσουσῐ|~ωσουσῐν";
    future_ind_pass_2: "~ωθησῃ|~ωθησει", _, _;
    aorist_subj_pass_3: _, _, "~ωθωσῐ|~ωθωσῐν";
    aorist_opt_pass_1: _, _, "~ωθεῑμεν|~ωθειημεν";
    aorist_opt_pass_2: _, "~ωθεῑτον|~ωθειητον", "~ωθεῑτε|~ωθειητε";
    aorist_opt_pass_3: _, "~ωθειτην|~ωθειητην", "~ωθεῑεν|~ωθειησᾰν";
    perfect_ind_act_1: "ἐ~ωκᾰ", _, "ἐ~ωκᾰμεν";
    perfect_ind_act_2: "ἐ~ωκᾰς", "ἐ~ωκᾰτον", "ἐ~ωκᾰτε";
    perfect_ind_act_3: "ἐ~ωκε|ἐ~ωκεν", _, "ἐ~ωκᾱσῐ|ἐ~ωκᾱσῐν";
    perfect_ind_mid_1: "ἐ~ωμαι", _, "ἐ~ωμεθᾰ";
    perfect_ind_mid_2: "ἐ~ωσαι", "ἐ~ωσθον", "ἐ~ωσθε";
    perfect_ind_mid_3: "ἐ~ωται", _, "ἐ~ωνται";
    perfect_subj_act_1: "ἐ~ωκω", _, "ἐ~ωκωμεν";
    perfect_subj_act_2: "ἐ~ωκῃς", "ἐ~ωκητον", "ἐ~ωκητε";
    perfect_subj_act_3: "ἐ~ωκῃ", _, "ἐ~ωκωσῐ|ἐ~ωκωσῐν";
    perfect_opt_act_1: "ἐ~ωκοιμῐ|ἐ~ωκοιην", _, "ἐ~ωκοιμεν";
    perfect_opt_act_2: "ἐ~ωκοις|ἐ~ωκοιης", "ἐ~ωκοιτον", "ἐ~ωκοιτε";
    perfect_opt_act_3: "ἐ~ωκοι|ἐ~ωκοιη", "ἐ~ωκοιτην", "ἐ~ωκοιεν";
    perfect_imp_act_2: "ἐ~ωκε", "ἐ~ωκετον", "ἐ~ωκετε";
    perfect_imp_act_3: "ἐ~ωκετω", "ἐ~ωκετων", "ἐ~ωκοντων";
    perfect_imp_mid_2: "ἐ~ωσο", _, _;
    perfect_imp_mid_3: "ἐ~ωσθω", "ἐ~ωσθων", _;
    pluperfect_mid_1: "ἐ~ωμην", _, "ἐ~ωμεθᾰ";
    pluperfect_mid_2: "ἐ~ωσο", "ἐ~ωσθον", "ἐ~ωσθε";
    pluperfect_mid_3: "ἐ~ωτο", "ἐ~ωσθην", "ἐ~ωντο";
    infinitive_perf: "ἐ~ωκεναι", "ἐ~ωσθαι", _;
    participle_perf: "ἐ~ωκως", "ἐ~ωμενος", _;
    ),
    // Ἀνᾰγω: 8 words (ἐξᾰγω, ὑπᾰγω, ἀνᾰγω, πᾰρᾰγω, ἐπᾰγω, κᾰτᾰγω). Like Ληγω, except in 227 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀνᾰγω", 8, "~ᾰγω", like Ληγω;
    stem aorist_ind: "~ηγᾰγ- like ἀνᾰγω";
    stem aorist_ind_pass: "~ηχθη- like ἐλπω";
    ind_act_1: "~ᾰγω", _, "~ᾰγομεν";
    ind_act_2: "~ᾰγεις", "~ᾰγετον", "~ᾰγετε";
    ind_act_3: "~ᾰγει", _, "~ᾰγουσῐ|~ᾰγουσῐν";
    ind_mid_1: "~ᾰγομαι", _, "~ᾰγομεθᾰ";
    ind_mid_2: "~ᾰγῃ|~ᾰγει", "~ᾰγεσθον", "~ᾰγεσθε";
    ind_mid_3: "~ᾰγεται", _, "~ᾰγονται";
    subj_act_1: "~ᾰγω", _, "~ᾰγωμεν";
    subj_act_2: "~ᾰγῃς", "~ᾰγητον", "~ᾰγητε";
    subj_act_3: "~ᾰγῃ", _, "~ᾰγωσῐ|~ᾰγωσῐν";
    subj_mid_1: "~ᾰγωμαι", _, "~ᾰγωμεθᾰ";
    subj_mid_2: _, "~ᾰγησθον", "~ᾰγησθε";
    subj_mid_3: "~ᾰγηται", _, "~ᾰγωνται";
    opt_act_1: "~ᾰγοιμῐ", _, "~ᾰγοιμεν";
    opt_act_2: "~ᾰγοις", "~ᾰγοιτον", "~ᾰγοιτε";
    opt_act_3: "~ᾰγοι", "~ᾰγοιτην", "~ᾰγοιεν";
    opt_mid_1: "~ᾰγοιμην", _, "~ᾰγοιμεθᾰ";
    opt_mid_2: "~ᾰγοιο", "~ᾰγοισθον", "~ᾰγοισθε";
    opt_mid_3: "~ᾰγοιτο", "~ᾰγοισθην", "~ᾰγοιντο";
    imp_act_2: "~ᾰγε", _, _;
    imp_act_3: "~ᾰγετω", "~ᾰγετων", "~ᾰγοντων";
    imp_mid_2: "~ᾰγου", _, _;
    imp_mid_3: "~ᾰγεσθω", "~ᾰγεσθων", _;
    imperfect_act_1: "~ηγον", _, "~ηγομεν";
    imperfect_act_2: "~ηγες", "~ηγετον", "~ηγετε";
    imperfect_act_3: "~ηγε|~ηγεν", "~ηγετην", "~ηγον";
    imperfect_mid_1: "~ηγομην", _, "~ηγομεθᾰ";
    imperfect_mid_2: "~ηγου", "~ηγεσθον", "~ηγεσθε";
    imperfect_mid_3: "~ηγετο", "~ηγεσθην", "~ηγοντο";
    future_ind_act_1: "~ᾰξω", _, "~ᾰξομεν";
    future_ind_act_2: "~ᾰξεις", "~ᾰξετον", "~ᾰξετε";
    future_ind_act_3: "~ᾰξει", _, "~ᾰξουσῐ|~ᾰξουσῐν";
    future_ind_mid_1: "~ᾰξομαι", _, "~ᾰξομεθᾰ";
    future_ind_mid_2: "~ᾰξῃ|~ᾰξει", "~ᾰξεσθον", "~ᾰξεσθε";
    future_ind_mid_3: "~ᾰξεται", _, "~ᾰξονται";
    future_ind_pass_1: "~ᾰχθησομαι", _, "~ᾰχθησομεθᾰ";
    future_ind_pass_2: "~ᾰχθησῃ|~ᾰχθησει", "~ᾰχθησεσθον", "~ᾰχθησεσθε";
    future_ind_pass_3: "~ᾰχθησεται", _, "~ᾰχθησονται";
    future_opt_act_1: "~ᾰξοιμῐ", _, "~ᾰξοιμεν";
    future_opt_act_2: "~ᾰξοις", "~ᾰξοιτον", "~ᾰξοιτε";
    future_opt_act_3: "~ᾰξοι", "~ᾰξοιτην", "~ᾰξοιεν";
    future_opt_mid_1: "~ᾰξοιμην", _, "~ᾰξοιμεθᾰ";
    future_opt_mid_2: "~ᾰξοιο", "~ᾰξοισθον", "~ᾰξοισθε";
    future_opt_mid_3: "~ᾰξοιτο", "~ᾰξοισθην", "~ᾰξοιντο";
    future_opt_pass_1: "~ᾰχθησοιμην", _, "~ᾰχθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰχθησοιο", "~ᾰχθησοισθον", "~ᾰχθησοισθε";
    future_opt_pass_3: "~ᾰχθησοιτο", "~ᾰχθησοισθην", "~ᾰχθησοιντο";
    aorist_subj_act_1: "~ᾰγᾰγω", _, "~ᾰγᾰγωμεν";
    aorist_subj_act_2: "~ᾰγᾰγῃς", "~ᾰγᾰγητον", "~ᾰγᾰγητε";
    aorist_subj_act_3: "~ᾰγᾰγῃ", _, "~ᾰγᾰγωσῐ|~ᾰγᾰγωσῐν";
    aorist_subj_mid_1: "~ᾰγᾰγωμαι", _, "~ᾰγᾰγωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰγᾰγησθον", "~ᾰγᾰγησθε";
    aorist_subj_mid_3: "~ᾰγᾰγηται", _, "~ᾰγᾰγωνται";
    aorist_subj_pass_1: "~ᾰχθω", _, "~ᾰχθωμεν";
    aorist_subj_pass_2: "~ᾰχθῃς", "~ᾰχθητον", "~ᾰχθητε";
    aorist_subj_pass_3: "~ᾰχθῃ", _, "~ᾰχθωσῐ";
    aorist_opt_act_1: "~ᾰγᾰγοιμῐ", _, "~ᾰγᾰγοιμεν";
    aorist_opt_act_2: "~ᾰγᾰγοις", "~ᾰγᾰγοιτον", "~ᾰγᾰγοιτε";
    aorist_opt_act_3: "~ᾰγᾰγοι", "~ᾰγᾰγοιτην", "~ᾰγᾰγοιεν";
    aorist_opt_mid_1: "~ᾰγᾰγοιμην", _, "~ᾰγᾰγοιμεθᾰ";
    aorist_opt_mid_2: "~ᾰγᾰγοιο", "~ᾰγᾰγοισθον", "~ᾰγᾰγοισθε";
    aorist_opt_mid_3: "~ᾰγᾰγοιτο", "~ᾰγᾰγοισθην", "~ᾰγᾰγοιντο";
    aorist_opt_pass_1: "~ᾰχθειην", _, "~ᾰχθεῑμεν";
    aorist_opt_pass_2: "~ᾰχθειης", "~ᾰχθεῑτον", "~ᾰχθεῑτε";
    aorist_opt_pass_3: "~ᾰχθειη", "~ᾰχθειτην", "~ᾰχθεῑεν";
    aorist_imp_act_2: "~ᾰγᾰγε", "~ᾰγᾰγετον", "~ᾰγᾰγετε";
    aorist_imp_act_3: "~ᾰγᾰγετω", "~ᾰγᾰγετων", "~ᾰγᾰγοντων";
    aorist_imp_mid_2: "~ᾰγᾰγοῡ", "~ᾰγᾰγεσθον", "~ᾰγᾰγεσθε";
    aorist_imp_mid_3: "~ᾰγᾰγεσθω", "~ᾰγᾰγεσθων", _;
    aorist_imp_pass_2: "~ᾰχθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰχθητω", "~ᾰχθητων", "~ᾰχθεντων";
    perfect_ind_act_1: "~ηχᾰ", _, "~ηχᾰμεν";
    perfect_ind_act_2: "~ηχᾰς", "~ηχᾰτον", "~ηχᾰτε";
    perfect_ind_act_3: "~ηχε", _, "~ηχᾱσῐ";
    perfect_ind_mid_1: "~ηγμαι", _, "~ηγμεθᾰ";
    perfect_ind_mid_2: "~ηξαι", "~ηχθον", "~ηχθε";
    perfect_ind_mid_3: "~ηκται", _, "~ηγᾰται";
    perfect_subj_act_1: "~ηχω", _, "~ηχωμεν";
    perfect_subj_act_2: "~ηχῃς", "~ηχητον", "~ηχητε";
    perfect_subj_act_3: "~ηχῃ", _, "~ηχωσῐ";
    perfect_opt_act_1: "~ηχοιμῐ", _, "~ηχοιμεν";
    perfect_opt_act_2: "~ηχοις", "~ηχοιτον", "~ηχοιτε";
    perfect_opt_act_3: "~ηχοι", "~ηχοιτην", "~ηχοιεν";
    perfect_imp_act_2: _, "~ηχετον", "~ηχετε";
    perfect_imp_act_3: "~ηχετω", "~ηχετων", "~ηχοντων";
    perfect_imp_mid_2: "~ηξο", _, _;
    perfect_imp_mid_3: "~ηχθω", "~ηχθων", _;
    infinitive_pres: "~ᾰγειν", "~ᾰγεσθαι", _;
    infinitive_fut: "~ᾰξειν", "~ᾰξεσθαι", "~ᾰχθησεσθαι";
    infinitive_aor: "~ᾰγᾰγεῑν", "~ᾰγᾰγεσθαι", "~ᾰχθηναι";
    infinitive_perf: "~ηχεναι", "~ηχθαι", _;
    participle_pres: "~ᾰγων", "~ᾰγομενος", _;
    participle_fut: "~ᾰξων", "~ᾰξομενος", "~ᾰχθησομενος";
    participle_aor: "~ᾰγᾰγων", "~ᾰγᾰγομενος", "~ᾰχθεις";
    participle_perf: "~ηχως", "~ηγμενος", _;
    ),
    // Φᾰρμᾰσσω: 8 words (τᾰρᾰσσω, θωρησσω, κνωσσω, θρῡλῐσσω, παιφᾰσσω, φᾰρμᾰσσω). Like Τᾰσσω, except in 55 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, future_ind_act_3 pl, future_ind_mid_2 sg, aorist_subj_act_3 pl, ….
    conjugation!("φᾰρμᾰσσω", 8, "~σσω", like Τᾰσσω as AOR_AS_FUT;
    stem imperfect: "~σσ- like ῥεω";
    stem aorist_ind: "~ξ- like φᾰρμᾰσσω";
    ind_act_3: _, _, "~σσουσῐ";
    ind_mid_2: "~σσῃ", _, _;
    subj_act_3: _, _, "~σσωσῐ";
    future_ind_act_3: _, _, "~ξουσῐ";
    future_ind_mid_2: "~ξῃ", _, _;
    aorist_subj_act_3: _, _, "~ξωσῐ";
    aorist_opt_act_1: "~ξοιμῐ", _, "~ξοιμεν";
    aorist_opt_act_2: "~ξοις", "~ξοιτον", "~ξοιτε";
    aorist_opt_act_3: "~ξοι", "~ξοιτην", "~ξοιεν";
    aorist_opt_mid_1: "~ξοιμην", _, "~ξοιμεθᾰ";
    aorist_opt_mid_2: "~ξοιο", "~ξοισθον", "~ξοισθε";
    aorist_opt_mid_3: "~ξοιτο", "~ξοισθην", "~ξοιντο";
    aorist_imp_act_2: "~ξε", "~ξετον", "~ξετε";
    aorist_imp_act_3: "~ξετω", "~ξετων", "~ξοντων";
    aorist_imp_mid_2: "~ξοῡ", "~ξεσθον", "~ξεσθε";
    aorist_imp_mid_3: "~ξεσθω", "~ξεσθων", _;
    perfect_ind_mid_3: _, _, "%ε~χᾰται";
    pluperfect_mid_3: _, _, "ἐ%ε~χᾰτο";
    future_perfect_ind_mid_1: "%ε~ξομαι", _, "%ε~ξομεθᾰ";
    future_perfect_ind_mid_2: "%ε~ξῃ", "%ε~ξεσθον", "%ε~ξεσθε";
    future_perfect_ind_mid_3: "%ε~ξεται", _, "%ε~ξονται";
    future_perfect_opt_mid_1: "%ε~ξοιμην", _, "%ε~ξοιμεθᾰ";
    future_perfect_opt_mid_2: "%ε~ξοιο", "%ε~ξοισθον", "%ε~ξοισθε";
    future_perfect_opt_mid_3: "%ε~ξοιτο", "%ε~ξοισθην", "%ε~ξοιντο";
    infinitive_aor: "~ξεῑν", _, "~χθηναι";
    infinitive_futp: _, "%ε~ξεσθαι", _;
    participle_aor: _, _, "~χθεις";
    participle_futp: _, "%ε~ξομενος", _;
    ),
    // Δῐωκω: 8 words (βρῡκω, πεκω, πλεκω, τηκω, κᾰτᾰδῐωκω, δῐωκω). Like Τᾰσσω, except in 91 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("δῐωκω", 8, "~κω", like Τᾰσσω;
    stem imperfect: "~κ- like ῥεω";
    stem aorist_ind: "~ξ- like ἐχω";
    ind_act_1: "~κω", _, "~κομεν";
    ind_act_2: "~κεις", "~κετον", "~κετε";
    ind_act_3: "~κει", _, "~κουσῐ";
    ind_mid_1: "~κομαι", _, "~κομεθᾰ";
    ind_mid_2: "~κῃ", "~κεσθον", "~κεσθε";
    ind_mid_3: "~κεται", _, "~κονται";
    subj_act_1: "~κω", _, "~κωμεν";
    subj_act_2: "~κῃς", "~κητον", "~κητε";
    subj_act_3: "~κῃ", _, "~κωσῐ";
    subj_mid_1: "~κωμαι", _, "~κωμεθᾰ";
    subj_mid_2: _, "~κησθον", "~κησθε";
    subj_mid_3: "~κηται", _, "~κωνται";
    opt_act_1: "~κοιμῐ", _, "~κοιμεν";
    opt_act_2: "~κοις", "~κοιτον", "~κοιτε";
    opt_act_3: "~κοι", "~κοιτην", "~κοιεν";
    opt_mid_1: "~κοιμην", _, "~κοιμεθᾰ";
    opt_mid_2: "~κοιο", "~κοισθον", "~κοισθε";
    opt_mid_3: "~κοιτο", "~κοισθην", "~κοιντο";
    imp_act_2: "~κε", _, _;
    imp_act_3: "~κετω", "~κετων", "~κοντων";
    imp_mid_2: "~κου", _, _;
    imp_mid_3: "~κεσθω", "~κεσθων", _;
    future_ind_act_3: _, _, "~ξουσῐ";
    future_ind_mid_2: "~ξῃ", _, _;
    aorist_ind_pass_1: derived, _, derived;
    aorist_ind_pass_2: derived, derived, derived;
    aorist_ind_pass_3: derived, derived, derived;
    aorist_subj_act_3: _, _, "~ξωσῐ";
    aorist_subj_pass_1: derived, _, derived;
    aorist_subj_pass_2: derived, derived, derived;
    aorist_subj_pass_3: derived, _, derived;
    aorist_opt_act_2: "~ξειᾰς", _, _;
    aorist_opt_act_3: "~ξειε", _, "~ξειᾰν";
    aorist_opt_pass_1: derived, _, derived;
    aorist_opt_pass_2: derived, derived, derived;
    aorist_opt_pass_3: derived, derived, derived;
    aorist_imp_pass_2: derived, _, _;
    aorist_imp_pass_3: derived, derived, derived;
    perfect_ind_mid_3: _, _, "%ε~χᾰται";
    pluperfect_mid_3: _, _, "ἐ%ε~χᾰτο";
    infinitive_pres: "~κειν", "~κεσθαι", _;
    infinitive_aor: _, _, derived;
    participle_pres: "~κων", "~κομενος", _;
    participle_aor: _, _, derived;
    ),
    // Καθιζω: 8 words (εὐαγγελιζω, βᾰδιζω, παιζω, καθιζω, συνᾱλιζω, βαλλιζω). Like Πεμπω, except in 235 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("καθιζω", 8, "~~ζω", like Πεμπω;
    stem aorist_ind: "~~σ- like ποιεω";
    stem aorist_ind_pass: "~~σθη- like ἐχω";
    stem pluperfect_act: "~~κε- like ἐχω";
    ind_act_1: "~~ζω", _, "~~ζομεν";
    ind_act_2: "~~ζεις", "~~ζετον", "~~ζετε";
    ind_act_3: "~~ζει", _, "~~ζουσῐ|~~ζουσῐν";
    ind_mid_1: "~~ζομαι", _, "~~ζομεθᾰ";
    ind_mid_2: "~~ζῃ", "~~ζεσθον", "~~ζεσθε";
    ind_mid_3: "~~ζεται", _, "~~ζονται";
    subj_act_1: "~~ζω", _, "~~ζωμεν";
    subj_act_2: "~~ζῃς", "~~ζητον", "~~ζητε";
    subj_act_3: "~~ζῃ", _, "~~ζωσῐ|~~ζωσῐν";
    subj_mid_1: "~~ζωμαι", _, "~~ζωμεθᾰ";
    subj_mid_2: _, "~~ζησθον", "~~ζησθε";
    subj_mid_3: "~~ζηται", _, "~~ζωνται";
    opt_act_1: "~~ζοιμῐ", _, "~~ζοιμεν";
    opt_act_2: "~~ζοις", "~~ζοιτον", "~~ζοιτε";
    opt_act_3: "~~ζοι", "~~ζοιτην", "~~ζοιεν";
    opt_mid_1: "~~ζοιμην", _, "~~ζοιμεθᾰ";
    opt_mid_2: "~~ζοιο", "~~ζοισθον", "~~ζοισθε";
    opt_mid_3: "~~ζοιτο", "~~ζοισθην", "~~ζοιντο";
    imp_act_2: "~~ζε", _, _;
    imp_act_3: "~~ζετω", "~~ζετων", "~~ζοντων";
    imp_mid_2: "~~ζου", _, _;
    imp_mid_3: "~~ζεσθω", "~~ζεσθων", _;
    imperfect_act_1: "ἐ~~ζον", _, "ἐ~~ζομεν";
    imperfect_act_2: "ἐ~~ζες", "ἐ~~ζετον", "ἐ~~ζετε";
    imperfect_act_3: "ἐ~~ζε|ἐ~~ζεν", "ἐ~~ζετην", "ἐ~~ζον";
    imperfect_mid_1: "ἐ~~ζομην", _, "ἐ~~ζομεθᾰ";
    imperfect_mid_2: "ἐ~~ζου", "ἐ~~ζεσθον", "ἐ~~ζεσθε";
    imperfect_mid_3: "ἐ~~ζετο", "ἐ~~ζεσθην", "ἐ~~ζοντο";
    future_ind_act_1: "~~ω", _, "~~οῡμεν";
    future_ind_act_2: "~~εῑς", "~~εῑτον", "~~εῑτε";
    future_ind_act_3: "~~εῑ", _, "~~οῡσῐ";
    future_ind_mid_1: "~~οῡμαι", _, "~~ουμεθᾰ";
    future_ind_mid_2: "~~εῑ|~~ῃ", "~~εῑσθον", "~~εῑσθε";
    future_ind_mid_3: "~~εῑται", _, "~~οῡνται";
    future_ind_pass_1: "~~σθησομαι", _, "~~σθησομεθᾰ";
    future_ind_pass_2: "~~σθησῃ", "~~σθησεσθον", "~~σθησεσθε";
    future_ind_pass_3: "~~σθησεται", _, "~~σθησονται";
    future_opt_act_1: "~~οιην", _, "~~οῑμεν";
    future_opt_act_2: "~~οιης", "~~οῑτον", "~~οῑτε";
    future_opt_act_3: "~~οιη", "~~οιτην", "~~οῑεν";
    future_opt_mid_1: "~~οιμην", _, "~~οιμεθᾰ";
    future_opt_mid_2: "~~οῑο", "~~οῑσθον", "~~οῑσθε";
    future_opt_mid_3: "~~οῑτο", "~~οισθην", "~~οῑντο";
    future_opt_pass_1: "~~σθησοιμην", _, "~~σθησοιμεθᾰ";
    future_opt_pass_2: "~~σθησοιο", "~~σθησοισθον", "~~σθησοισθε";
    future_opt_pass_3: "~~σθησοιτο", "~~σθησοισθην", "~~σθησοιντο";
    aorist_subj_act_1: "~~σω", _, "~~σωμεν";
    aorist_subj_act_2: "~~σῃς", "~~σητον", "~~σητε";
    aorist_subj_act_3: "~~σῃ", _, "~~σωσῐ|~~σωσῐν";
    aorist_subj_mid_1: "~~σωμαι", _, "~~σωμεθᾰ";
    aorist_subj_mid_2: _, "~~σησθον", "~~σησθε";
    aorist_subj_mid_3: "~~σηται", _, "~~σωνται";
    aorist_subj_pass_1: "~~σθω", _, "~~σθωμεν";
    aorist_subj_pass_2: "~~σθῃς", "~~σθητον", "~~σθητε";
    aorist_subj_pass_3: "~~σθῃ", _, "~~σθωσῐ";
    aorist_opt_act_1: "~~σαιμῐ", _, "~~σαιμεν";
    aorist_opt_act_2: "~~σειᾰς|~~σαις", "~~σαιτον", "~~σαιτε";
    aorist_opt_act_3: "~~σειε|~~σειεν|~~σαι", "~~σαιτην", "~~σειᾰν|~~σαιεν";
    aorist_opt_mid_1: "~~σαιμην", _, "~~σαιμεθᾰ";
    aorist_opt_mid_2: "~~σαιο", "~~σαισθον", "~~σαισθε";
    aorist_opt_mid_3: "~~σαιτο", "~~σαισθην", "~~σαιντο";
    aorist_opt_pass_1: "~~σθειην", _, "~~σθεῑμεν";
    aorist_opt_pass_2: "~~σθειης", "~~σθεῑτον", "~~σθεῑτε";
    aorist_opt_pass_3: "~~σθειη", "~~σθειτην", "~~σθεῑεν";
    aorist_imp_act_2: "~~σον", "~~σᾰτον", "~~σᾰτε";
    aorist_imp_act_3: "~~σᾰτω", "~~σᾰτων", "~~σᾰντων";
    aorist_imp_mid_2: "~~σαι", "~~σᾰσθον", "~~σᾰσθε";
    aorist_imp_mid_3: "~~σᾰσθω", "~~σᾰσθων", _;
    aorist_imp_pass_2: "~~σθητῐ", _, _;
    aorist_imp_pass_3: "~~σθητω", "~~σθητων", "~~σθεντων";
    perfect_ind_act_1: "%ε~~κᾰ", _, "%ε~~κᾰμεν";
    perfect_ind_act_2: "%ε~~κᾰς", "%ε~~κᾰτον", "%ε~~κᾰτε";
    perfect_ind_act_3: "%ε~~κε", _, "%ε~~κᾱσῐ";
    perfect_ind_mid_1: "%ε~~σμαι", _, "%ε~~σμεθᾰ";
    perfect_ind_mid_2: "%ε~~σαι", "%ε~~σθον", "%ε~~σθε";
    perfect_ind_mid_3: "%ε~~σται", _, "%ε~~σᾰται";
    perfect_subj_act_1: "%ε~~κω", _, "%ε~~κωμεν";
    perfect_subj_act_2: "%ε~~κῃς", "%ε~~κητον", "%ε~~κητε";
    perfect_subj_act_3: "%ε~~κῃ", _, "%ε~~κωσῐ";
    perfect_opt_act_1: "%ε~~κοιμῐ", _, "%ε~~κοιμεν";
    perfect_opt_act_2: "%ε~~κοις", "%ε~~κοιτον", "%ε~~κοιτε";
    perfect_opt_act_3: "%ε~~κοι", "%ε~~κοιτην", "%ε~~κοιεν";
    perfect_imp_act_2: _, "%ε~~κετον", "%ε~~κετε";
    perfect_imp_act_3: "%ε~~κετω", "%ε~~κετων", "%ε~~κοντων";
    perfect_imp_mid_2: "%ε~~σο", _, _;
    perfect_imp_mid_3: "%ε~~σθω", "%ε~~σθων", _;
    pluperfect_mid_1: "ἐ%ε~~σμην", _, "ἐ%ε~~σμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~~σο", "ἐ%ε~~σθον", "ἐ%ε~~σθε";
    pluperfect_mid_3: "ἐ%ε~~στο", "ἐ%ε~~σθην", "ἐ%ε~~σᾰτο";
    infinitive_pres: "~~ζειν", "~~ζεσθαι", _;
    infinitive_fut: "~~εῑν", "~~εῑσθαι", "~~σθησεσθαι";
    infinitive_aor: "~ῑσα~", "~~σᾰσθαι", "~~σθηναι";
    infinitive_perf: "%ε~~κεναι", "%ε~ῑσθα~", _;
    participle_pres: "~~ζων", "~~ζομενος", _;
    participle_fut: "~~ων", "~~ουμενος", "~~σθησομενος";
    participle_aor: "~~σᾱς", "~~σᾰμενος", "~~σθεις";
    participle_perf: "%ε~~κως", "%ε~~σμενος", _;
    ),
    // Φῠω: 8 words (φῠω, ἐνδῠω, κῠω, κωλῠω, προμηνῠω, δῐᾰμηνῠω). Like Χεω, except in 208 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("φῠω", 8, "~ῠω", like Χεω as C3_AS_1;
    stem imperfect: "~ῠ- like ῥεω";
    stem aorist_ind: "~ῡσ- like ἐχω";
    stem aorist_ind_pass: "~ῡθη- like ἐχω";
    ind_act_1: "~ῠω", _, "~ῠομεν";
    ind_act_2: "~ῠεις", "~ῠετον", "~ῠετε";
    ind_act_3: "~ῠει", _, "~ῠουσῐ|~ῠουσῐν";
    ind_mid_1: "~ῠομαι", _, "~ῠομεθᾰ";
    ind_mid_2: "~ῠῃ|~ῠει", "~ῠεσθον", "~ῠεσθε";
    ind_mid_3: "~ῠεται", _, "~ῠονται";
    subj_act_1: "~ῠω", _, "~ῠωμεν";
    subj_act_2: "~ῠῃς", "~ῠητον", "~ῠητε";
    subj_act_3: "~ῠῃ", _, "~ῠωσῐ|~ῠωσῐν";
    subj_mid_1: "~ῠωμαι", _, "~ῠωμεθᾰ";
    subj_mid_2: _, "~ῠησθον", "~ῠησθε";
    subj_mid_3: "~ῠηται", _, "~ῠωνται";
    opt_act_1: "~ῠοιμῐ", _, "~ῠοιμεν";
    opt_act_2: "~ῠοις", "~ῠοιτον", "~ῠοιτε";
    opt_act_3: "~ῠοι", "~ῠοιτην", "~ῠοιεν";
    opt_mid_1: "~ῠοιμην", _, "~ῠοιμεθᾰ";
    opt_mid_2: "~ῠοιο", "~ῠοισθον", "~ῠοισθε";
    opt_mid_3: "~ῠοιτο", "~ῠοισθην", "~ῠοιντο";
    imp_act_2: "~ῠε", _, _;
    imp_act_3: "~ῠετω", "~ῠετων", "~ῠοντων";
    imp_mid_2: "~ῠου", _, _;
    imp_mid_3: "~ῠεσθω", "~ῠεσθων", _;
    future_ind_act_1: "~ῡσω", _, "~ῡσομεν";
    future_ind_act_2: "~ῡσεις", "~ῡσετον", "~ῡσετε";
    future_ind_act_3: "~ῡσει", _, "~ῡσουσῐ";
    future_ind_mid_1: "~ῡσομαι", _, "~ῡσομεθᾰ";
    future_ind_mid_2: "~ῡσῃ", "~ῡσεσθον", "~ῡσεσθε";
    future_ind_mid_3: "~ῡσεται", _, "~ῡσονται";
    future_ind_pass_1: "~ῡθησομαι", _, "~ῡθησομεθᾰ";
    future_ind_pass_2: "~ῡθησῃ", "~ῡθησεσθον", "~ῡθησεσθε";
    future_ind_pass_3: "~ῡθησεται", _, "~ῡθησονται";
    future_opt_act_1: "~ῡσοιμῐ", _, "~ῡσοιμεν";
    future_opt_act_2: "~ῡσοις", "~ῡσοιτον", "~ῡσοιτε";
    future_opt_act_3: "~ῡσοι", "~ῡσοιτην", "~ῡσοιεν";
    future_opt_mid_1: "~ῡσοιμην", _, "~ῡσοιμεθᾰ";
    future_opt_mid_2: "~ῡσοιο", "~ῡσοισθον", "~ῡσοισθε";
    future_opt_mid_3: "~ῡσοιτο", "~ῡσοισθην", "~ῡσοιντο";
    future_opt_pass_1: "~ῡθησοιμην", _, "~ῡθησοιμεθᾰ";
    future_opt_pass_2: "~ῡθησοιο", "~ῡθησοισθον", "~ῡθησοισθε";
    future_opt_pass_3: "~ῡθησοιτο", "~ῡθησοισθην", "~ῡθησοιντο";
    aorist_subj_act_1: "~ῡσω", _, "~ῡσωμεν";
    aorist_subj_act_2: "~ῡσῃς", "~ῡσητον", "~ῡσητε";
    aorist_subj_act_3: "~ῡσῃ", _, "~ῡσωσῐ";
    aorist_subj_mid_1: "~ῡσωμαι", _, "~ῡσωμεθᾰ";
    aorist_subj_mid_2: _, "~ῡσησθον", "~ῡσησθε";
    aorist_subj_mid_3: "~ῡσηται", _, "~ῡσωνται";
    aorist_subj_pass_1: "~ῡθω", _, "~ῡθωμεν";
    aorist_subj_pass_2: "~ῡθῃς", "~ῡθητον", "~ῡθητε";
    aorist_subj_pass_3: "~ῡθῃ", _, "~ῡθωσῐ";
    aorist_opt_act_1: "~ῡσαιμῐ", _, "~ῡσαιμεν";
    aorist_opt_act_2: "~ῡσειᾰς|~ῡσαις", "~ῡσαιτον", "~ῡσαιτε";
    aorist_opt_act_3: "~ῡσειε|~ῡσειεν|~ῡσαι", "~ῡσαιτην", "~ῡσειᾰν|~ῡσαιεν";
    aorist_opt_mid_1: "~ῡσαιμην", _, "~ῡσαιμεθᾰ";
    aorist_opt_mid_2: "~ῡσαιο", "~ῡσαισθον", "~ῡσαισθε";
    aorist_opt_mid_3: "~ῡσαιτο", "~ῡσαισθην", "~ῡσαιντο";
    aorist_opt_pass_1: "~ῡθειην", _, "~ῡθεῑμεν";
    aorist_opt_pass_2: "~ῡθειης", "~ῡθεῑτον", "~ῡθεῑτε";
    aorist_opt_pass_3: "~ῡθειη", "~ῡθειτην", "~ῡθεῑεν";
    aorist_imp_act_2: _, "~ῡσᾰτον", "~ῡσᾰτε";
    aorist_imp_act_3: "~ῡσᾰτω", "~ῡσᾰτων", "~ῡσᾰντων";
    aorist_imp_mid_2: "~ῡσαι", "~ῡσᾰσθον", "~ῡσᾰσθε";
    aorist_imp_mid_3: "~ῡσᾰσθω", "~ῡσᾰσθων", _;
    aorist_imp_pass_2: "~ῡθητῐ", _, _;
    aorist_imp_pass_3: "~ῡθητω", "~ῡθητων", "~ῡθεντων";
    perfect_ind_act_1: "%ε~ῡκᾰ", _, "%ε~ῡκᾰμεν";
    perfect_ind_act_2: "%ε~ῡκᾰς", "%ε~ῡκᾰτον", "%ε~ῡκᾰτε";
    perfect_ind_act_3: "%ε~ῡκε", _, "%ε~ῡκᾱσῐ";
    perfect_ind_mid_1: "%ε~ῡμαι", _, "%ε~ῡμεθᾰ";
    perfect_ind_mid_2: "%ε~ῡσαι", "%ε~ῡσθον", "%ε~ῡσθε";
    perfect_ind_mid_3: "%ε~ῡται", _, "%ε~ῡνται";
    perfect_subj_act_1: "%ε~ῡκω", _, "%ε~ῡκωμεν";
    perfect_subj_act_2: "%ε~ῡκῃς", "%ε~ῡκητον", "%ε~ῡκητε";
    perfect_subj_act_3: "%ε~ῡκῃ", _, "%ε~ῡκωσῐ";
    perfect_opt_act_1: "%ε~ῡκοιμῐ", _, "%ε~ῡκοιμεν";
    perfect_opt_act_2: "%ε~ῡκοις", "%ε~ῡκοιτον", "%ε~ῡκοιτε";
    perfect_opt_act_3: "%ε~ῡκοι", "%ε~ῡκοιτην", "%ε~ῡκοιεν";
    perfect_imp_act_2: _, "%ε~ῡκετον", "%ε~ῡκετε";
    perfect_imp_act_3: "%ε~ῡκετω", "%ε~ῡκετων", "%ε~ῡκοντων";
    perfect_imp_mid_2: "%ε~ῡσο", _, _;
    perfect_imp_mid_3: "%ε~ῡσθω", "%ε~ῡσθων", _;
    infinitive_pres: "~ῠειν", "~ῠεσθαι", _;
    infinitive_fut: "~ῡσειν", "~ῡσεσθαι", "~ῡθησεσθαι";
    infinitive_aor: _, "~ῡσᾰσθαι", "~ῡθηναι";
    infinitive_perf: "%ε~ῡκεναι", _, _;
    participle_pres: "~ῠων", "~ῠομενος", _;
    participle_fut: "~ῡσων", "~ῡσομενος", "~ῡθησομενος";
    participle_aor: "~ῡσᾱς", "~ῡσᾰμενος", "~ῡθεις";
    participle_perf: "%ε~ῡκως", "%ε~ῡμενος", _;
    ),
    // Πλᾰσσω: 8 words (πλᾰσσω, πᾰσσω, βρᾰσσω, ἱμᾰσσω, ἐρεσσω, βλῐσσω). Like Κᾰλεω, except in 169 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("πλᾰσσω", 8, "~σω", like Κᾰλεω as ACT_AS_PASS;
    stem imperfect: "~σ- like ῥεω";
    stem aorist_ind: "~- like ἐχω";
    ind_act_1: "~σω", _, "~σομεν";
    ind_act_2: "~σεις", "~σετον", "~σετε";
    ind_act_3: "~σει", _, "~σουσῐ";
    ind_mid_1: "~σομαι", _, "~σομεθᾰ";
    ind_mid_2: "~σῃ", "~σεσθον", "~σεσθε";
    ind_mid_3: "~σεται", _, "~σονται";
    subj_act_1: "~σω", _, "~σωμεν";
    subj_act_2: "~σῃς", "~σητον", "~σητε";
    subj_act_3: "~σῃ", _, "~σωσῐ";
    subj_mid_1: "~σωμαι", _, "~σωμεθᾰ";
    subj_mid_2: _, "~σησθον", "~σησθε";
    subj_mid_3: "~σηται", _, "~σωνται";
    opt_act_1: "~σοιμῐ", _, "~σοιμεν";
    opt_act_2: "~σοις", "~σοιτον", "~σοιτε";
    opt_act_3: "~σοι", "~σοιτην", "~σοιεν";
    opt_mid_1: "~σοιμην", _, "~σοιμεθᾰ";
    opt_mid_2: "~σοιο", "~σοισθον", "~σοισθε";
    opt_mid_3: "~σοιτο", "~σοισθην", "~σοιντο";
    imp_act_2: "~σε", _, _;
    imp_act_3: "~σετω", "~σετων", "~σοντων";
    imp_mid_2: "~σου", _, _;
    imp_mid_3: "~σεσθω", "~σεσθων", _;
    future_ind_act_1: "~ω", _, "~ομεν";
    future_ind_act_2: "~εις", "~ετον", "~ετε";
    future_ind_act_3: "~ει", _, "~ουσῐ";
    future_ind_mid_1: "~ομαι", _, "~ομεθᾰ";
    future_ind_mid_2: "~ῃ", "~εσθον", "~εσθε";
    future_ind_mid_3: "~εται", _, "~ονται";
    future_opt_act_1: "~οιμῐ", _, "~οιμεν";
    future_opt_act_2: "~οις", "~οιτον", "~οιτε";
    future_opt_act_3: "~οι", "~οιτην", "~οιεν";
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_2: "~οιο", "~οισθον", "~οισθε";
    future_opt_mid_3: "~οιτο", "~οισθην", "~οιντο";
    aorist_subj_act_1: "~ω", _, "~ωμεν";
    aorist_subj_act_2: "~ῃς", "~ητον", "~ητε";
    aorist_subj_act_3: "~ῃ", _, "~ωσῐ";
    aorist_subj_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    aorist_subj_mid_2: _, "~ησθον", "~ησθε";
    aorist_subj_mid_3: "~ηται", _, "~ωνται";
    aorist_opt_act_1: "~αιμῐ", _, "~αιμεν";
    aorist_opt_act_2: "~ειᾰς", "~αιτον", "~αιτε";
    aorist_opt_act_3: "~ειε", "~αιτην", "~ειᾰν";
    aorist_opt_mid_1: "~αιμην", _, "~αιμεθᾰ";
    aorist_opt_mid_2: "~αιο", "~αισθον", "~αισθε";
    aorist_opt_mid_3: "~αιτο", "~αισθην", "~αιντο";
    aorist_imp_act_2: "~ον", "~ᾰτον", "~ᾰτε";
    aorist_imp_act_3: "~ᾰτω", "~ᾰτων", "~ᾰντων";
    aorist_imp_mid_2: "~αι", "~ᾰσθον", "~ᾰσθε";
    aorist_imp_mid_3: "~ᾰσθω", "~ᾰσθων", _;
    perfect_ind_mid_1: "%ε~μαι", _, "%ε~μεθᾰ";
    perfect_ind_mid_2: "%ε~αι", "%ε~θον", "%ε~θε";
    perfect_ind_mid_3: "%ε~ται", _, "%ε~ᾰται";
    perfect_imp_mid_2: "%ε~ο", _, _;
    perfect_imp_mid_3: "%ε~θω", "%ε~θων", _;
    pluperfect_mid_1: "ἐ%ε~μην", _, "ἐ%ε~μεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ο", "ἐ%ε~θον", "ἐ%ε~θε";
    pluperfect_mid_3: "ἐ%ε~το", "ἐ%ε~θην", "ἐ%ε~ᾰτο";
    future_perfect_ind_mid_1: "%ε~ομαι", _, "%ε~ομεθᾰ";
    future_perfect_ind_mid_2: "%ε~ῃ", "%ε~εσθον", "%ε~εσθε";
    future_perfect_ind_mid_3: "%ε~εται", _, "%ε~ονται";
    future_perfect_opt_mid_1: "%ε~οιμην", _, "%ε~οιμεθᾰ";
    future_perfect_opt_mid_2: "%ε~οιο", "%ε~οισθον", "%ε~οισθε";
    future_perfect_opt_mid_3: "%ε~οιτο", "%ε~οισθην", "%ε~οιντο";
    infinitive_pres: "~σειν", "~σεσθαι", _;
    infinitive_fut: "~ειν", "~εσθαι", _;
    infinitive_aor: "~αι", "~ᾰσθαι", _;
    infinitive_perf: _, "%ε~θαι", _;
    infinitive_futp: _, "%ε~εσθαι", _;
    participle_pres: "~σων", "~σομενος", _;
    participle_fut: "~ων", "~ομενος", _;
    participle_aor: "~ᾱς", "~ᾰμενος", _;
    participle_perf: _, "%ε~μενος", _;
    participle_futp: _, "%ε~ομενος", _;
    ),
    // Πᾰρεχω: 8 words (ἐπεχω, προσεχω, πᾰρεχω, ἀπεχω, περιεχω, ἀνεχω). Like Μῑγνῡμῐ, except in 211 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("πᾰρεχω", 8, "~εχω", like Μῑγνῡμῐ;
    stem aorist_ind: "~εσχ- like πᾰρεχω";
    stem aorist_ind_pass: "~εσχεθη- like ἐλπω";
    ind_act_1: "~εχω", _, "~εχομεν";
    ind_act_2: "~εχεις", "~εχετον", "~εχετε";
    ind_act_3: "~εχει", _, "~εχουσῐ|~εχουσῐν";
    ind_mid_1: "~εχομαι", _, "~εχομεθᾰ";
    ind_mid_2: "~εχῃ|~εχει", "~εχεσθον", "~εχεσθε";
    ind_mid_3: "~εχεται", _, "~εχονται";
    subj_act_1: "~εχω", _, "~εχωμεν";
    subj_act_2: "~εχῃς", "~εχητον", "~εχητε";
    subj_act_3: "~εχῃ", _, "~εχωσῐ|~εχωσῐν";
    subj_mid_1: "~εχωμαι", _, "~εχωμεθᾰ";
    subj_mid_2: _, "~εχησθον", "~εχησθε";
    subj_mid_3: "~εχηται", _, "~εχωνται";
    opt_act_1: "~εχοιμῐ", _, "~εχοιμεν";
    opt_act_2: "~εχοις", "~εχοιτον", "~εχοιτε";
    opt_act_3: "~εχοι", "~εχοιτην", "~εχοιεν";
    opt_mid_1: "~εχοιμην", _, "~εχοιμεθᾰ";
    opt_mid_2: "~εχοιο", "~εχοισθον", "~εχοισθε";
    opt_mid_3: "~εχοιτο", "~εχοισθην", "~εχοιντο";
    imp_act_2: "~εχε", _, _;
    imp_act_3: "~εχετω", "~εχετων", "~εχοντων";
    imp_mid_2: "~εχου", _, _;
    imp_mid_3: "~εχεσθω", "~εχεσθων", _;
    imperfect_act_1: "~ειχον", _, "~ειχομεν";
    imperfect_act_2: "~ειχες", "~ειχετον", "~ειχετε";
    imperfect_act_3: "~ειχε", "~ειχετην", "~ειχον";
    imperfect_mid_1: "~ειχομην", _, "~ειχομεθᾰ";
    imperfect_mid_2: "~ειχου", "~ειχεσθον", "~ειχεσθε";
    imperfect_mid_3: "~ειχετο", "~ειχεσθην", "~ειχοντο";
    future_ind_act_1: "~εξω", _, "~εξομεν";
    future_ind_act_2: "~εξεις", "~εξετον", "~εξετε";
    future_ind_act_3: "~εξει", _, "~εξουσῐ";
    future_ind_mid_1: "~εξομαι", _, "~εξομεθᾰ";
    future_ind_mid_2: "~εξῃ", "~εξεσθον", "~εξεσθε";
    future_ind_mid_3: "~εξεται", _, "~εξονται";
    future_opt_act_1: "~εξοιμῐ", _, "~εξοιμεν";
    future_opt_act_2: "~εξοις", "~εξοιτον", "~εξοιτε";
    future_opt_act_3: "~εξοι", "~εξοιτην", "~εξοιεν";
    future_opt_mid_1: "~εξοιμην", _, "~εξοιμεθᾰ";
    future_opt_mid_2: "~εξοιο", "~εξοισθον", "~εξοισθε";
    future_opt_mid_3: "~εξοιτο", "~εξοισθην", "~εξοιντο";
    aorist_subj_act_1: "~σχω", _, "~σχωμεν";
    aorist_subj_act_2: "~σχῃς", "~σχητον", "~σχητε";
    aorist_subj_act_3: "~σχῃ", _, "~σχωσῐ|~σχωσῐν";
    aorist_subj_mid_1: "~σχωμαι", _, "~σχωμεθᾰ";
    aorist_subj_mid_2: _, "~σχησθον", "~σχησθε";
    aorist_subj_mid_3: "~σχηται", _, "~σχωνται";
    aorist_subj_pass_1: "~σχεθω", _, "~σχεθωμεν";
    aorist_subj_pass_2: "~σχεθῃς", "~σχεθητον", "~σχεθητε";
    aorist_subj_pass_3: "~σχεθῃ", _, "~σχεθωσῐ";
    aorist_opt_act_1: "~σχοιμῐ", _, "~σχοιμεν";
    aorist_opt_act_2: "~σχοις", "~σχοιτον", "~σχοιτε";
    aorist_opt_act_3: "~σχοι", "~σχοιτην", "~σχοιεν";
    aorist_opt_mid_1: "~σχοιμην", _, "~σχοιμεθᾰ";
    aorist_opt_mid_2: "~σχοιο", "~σχοισθον", "~σχοισθε";
    aorist_opt_mid_3: "~σχοιτο", "~σχοισθην", "~σχοιντο";
    aorist_opt_pass_1: "~σχεθειην", _, "~σχεθεῑμεν";
    aorist_opt_pass_2: "~σχεθειης", "~σχεθεῑτον", "~σχεθεῑτε";
    aorist_opt_pass_3: "~σχεθειη", "~σχεθειτην", "~σχεθεῑεν";
    aorist_imp_act_2: "~σχε", "~σχετον", "~σχετε";
    aorist_imp_act_3: "~σχετω", "~σχετων", "~σχοντων";
    aorist_imp_mid_2: "~σχοῡ", "~σχεσθον", "~σχεσθε";
    aorist_imp_mid_3: "~σχεσθω", "~σχεσθων", _;
    aorist_imp_pass_2: "~σχεθητῐ", _, _;
    aorist_imp_pass_3: "~σχεθητω", "~σχεθητων", "~σχεθεντων";
    perfect_ind_act_1: "~εσχηκᾰ", _, "~εσχηκᾰμεν";
    perfect_ind_act_2: "~εσχηκᾰς", "~εσχηκᾰτον", "~εσχηκᾰτε";
    perfect_ind_act_3: "~εσχηκε|~εσχηκεν", _, "~εσχηκᾱσῐ|~εσχηκᾱσῐν";
    perfect_ind_mid_1: "~εσχημαι", _, "~εσχημεθᾰ";
    perfect_ind_mid_2: "~εσχησαι", "~εσχησθον", "~εσχησθε";
    perfect_ind_mid_3: "~εσχηται", _, "~εσχηνται";
    perfect_subj_act_1: "~εσχηκω", _, "~εσχηκωμεν";
    perfect_subj_act_2: "~εσχηκῃς", "~εσχηκητον", "~εσχηκητε";
    perfect_subj_act_3: "~εσχηκῃ", _, "~εσχηκωσῐ|~εσχηκωσῐν";
    perfect_opt_act_1: "~εσχηκοιμῐ|~εσχηκοιην", _, "~εσχηκοιμεν";
    perfect_opt_act_2: "~εσχηκοις|~εσχηκοιης", "~εσχηκοιτον", "~εσχηκοιτε";
    perfect_opt_act_3: "~εσχηκοι|~εσχηκοιη", "~εσχηκοιτην", "~εσχηκοιεν";
    perfect_imp_act_2: "~εσχηκε", "~εσχηκετον", "~εσχηκετε";
    perfect_imp_act_3: "~εσχηκετω", "~εσχηκετων", "~εσχηκοντων";
    perfect_imp_mid_2: "~εσχησο", _, _;
    perfect_imp_mid_3: "~εσχησθω", "~εσχησθων", _;
    infinitive_pres: "~εχειν", "~εχεσθαι", _;
    infinitive_fut: "~εξειν", "~εξεσθαι", _;
    infinitive_aor: "~σχεῑν", "~σχεσθαι", "~σχεθηναι";
    infinitive_perf: "~εσχηκεναι", "~εσχησθαι", _;
    participle_pres: "~εχων", "~εχομενος", _;
    participle_fut: "~εξων", "~εξομενος", _;
    participle_aor: "~σχων", "~σχομενος", "~σχεθεις";
    participle_perf: "~εσχηκως", "~εσχημενος", _;
    ),
    // Κρῑνομαι: 8 words (γεινομαι, ὀλοφῡρομαι, ἐρομαι, ὀδῡρομαι, σῑνομαι, ἐπερομαι). Like Οἰομαι, except in 54 cells: aorist_subj_act_1 sg, aorist_subj_act_1 pl, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_2 pl, aorist_subj_act_3 sg, ….
    conjugation!("κρῑνομαι", 8, "~ομαι", like Οἰομαι as AORIST_AS_PRESENT;
    stem aorist_ind: "~- like ἐχω";
    aorist_subj_act_1: "~ω", _, "~ωμεν";
    aorist_subj_act_2: "~ῃς", "~ητον", "~ητε";
    aorist_subj_act_3: "~ῃ", _, "~ωσῐ";
    aorist_subj_mid_2: derived, _, _;
    aorist_subj_pass_2: derived, _, _;
    aorist_opt_act_1: "~αιμῐ", _, "~αιμεν";
    aorist_opt_act_2: "~ειᾰς", "~αιτον", "~αιτε";
    aorist_opt_act_3: "~ειε", "~αιτην", "~ειᾰν";
    aorist_opt_mid_1: "~αιμην", _, "~αιμεθᾰ";
    aorist_opt_mid_2: "~αιο", "~αισθον", "~αισθε";
    aorist_opt_mid_3: "~αιτο", "~αισθην", "~αιντο";
    aorist_imp_act_2: "~ον", "~ᾰτον", "~ᾰτε";
    aorist_imp_act_3: "~ᾰτω", "~ᾰτων", "~ᾰντων";
    aorist_imp_mid_2: "~αι", "~ᾰσθον", "~ᾰσθε";
    aorist_imp_mid_3: "~ᾰσθω", "~ᾰσθων", _;
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, derived;
    perfect_ind_mid_3: derived, _, derived;
    perfect_imp_mid_2: derived, _, _;
    perfect_imp_mid_3: derived, derived, _;
    infinitive_aor: "~αι", "~ᾰσθαι", derived;
    infinitive_perf: _, derived, _;
    participle_aor: "~ᾱς", "~ᾰμενος", derived;
    participle_perf: _, derived, _;
    ),
    // Πῠνθανομαι: 8 words (μαχομαι, πῠνθανομαι, σῠνεργαζομαι, κᾰτεργαζομαι, ἀπεχθανομαι, ἐξεργαζομαι). Like Δερκομαι, except in 90 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("πῠνθανομαι", 8, "~α~ομαι", like Δερκομαι as PASS_AS_ACT;
    stem aorist_ind: "~ᾰ~εσ- like πᾰραιτεομαι";
    ind_act_1: "~α~ομαι", _, _;
    ind_mid_1: "~ᾰ~ομαι", _, "~ᾰ~ομεθᾰ";
    ind_mid_2: "~ᾰ~ῃ|~ᾰ~ει", "~ᾰ~εσθον", "~ᾰ~εσθε";
    ind_mid_3: "~ᾰ~εται", _, "~ᾰ~ονται";
    ind_pass_1: derived, _, _;
    ind_pass_2: "~ᾰ~ῃ", _, _;
    subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    subj_mid_2: "~ᾰ~ῃ", "~ᾰ~ησθον", "~ᾰ~ησθε";
    subj_mid_3: "~ᾰ~ηται", _, "~ᾰ~ωνται";
    subj_pass_2: "~ᾰ~ῃ", _, _;
    opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~οιο", "~ᾰ~οισθον", "~ᾰ~οισθε";
    opt_mid_3: "~ᾰ~οιτο", "~ᾰ~οισθην", "~ᾰ~οιντο";
    imp_mid_2: "~ᾰ~ου", _, _;
    imp_mid_3: "~ᾰ~εσθω", "~ᾰ~εσθων", _;
    imperfect_mid_1: "ἐ~ᾰ~ομην", _, "ἐ~ᾰ~ομεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~ου", "ἐ~ᾰ~εσθον", "ἐ~ᾰ~εσθε";
    imperfect_mid_3: "ἐ~ᾰ~ετο", "ἐ~ᾰ~εσθην", "ἐ~ᾰ~οντο";
    future_ind_mid_1: "~ᾰ~εσομαι", _, "~ᾰ~εσομεθᾰ";
    future_ind_mid_2: "~ᾰ~εσῃ", "~ᾰ~εσεσθον", "~ᾰ~εσεσθε";
    future_ind_mid_3: "~ᾰ~εσεται", _, "~ᾰ~εσονται";
    future_opt_mid_1: "~ᾰ~εσοιμην", _, "~ᾰ~εσοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~εσοιο", "~ᾰ~εσοισθον", "~ᾰ~εσοισθε";
    future_opt_mid_3: "~ᾰ~εσοιτο", "~ᾰ~εσοισθην", "~ᾰ~εσοιντο";
    aorist_subj_mid_1: "~ᾰ~εσωμαι", _, "~ᾰ~εσωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~εσῃ", "~ᾰ~εσησθον", "~ᾰ~εσησθε";
    aorist_subj_mid_3: "~ᾰ~εσηται", _, "~ᾰ~εσωνται";
    aorist_opt_mid_1: "~ᾰ~εσαιμην", _, "~ᾰ~εσαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~εσαιο", "~ᾰ~εσαισθον", "~ᾰ~εσαισθε";
    aorist_opt_mid_3: "~ᾰ~εσαιτο", "~ᾰ~εσαισθην", "~ᾰ~εσαιντο";
    aorist_imp_mid_2: "~ᾰ~εσαι", "~ᾰ~εσᾰσθον", "~ᾰ~εσᾰσθε";
    aorist_imp_mid_3: "~ᾰ~εσᾰσθω", "~ᾰ~εσᾰσθων", _;
    perfect_ind_mid_1: "~εμᾰ~ημαι", _, "~εμᾰ~ημεθᾰ";
    perfect_ind_mid_2: "~εμᾰ~ησαι", "~εμᾰ~ησθον", "~εμᾰ~ησθε";
    perfect_ind_mid_3: "~εμᾰ~ηται", _, "~εμᾰ~ηνται";
    perfect_imp_mid_2: "~εμᾰ~ησο", _, _;
    perfect_imp_mid_3: "~εμᾰ~ησθω", "~εμᾰ~ησθων", _;
    infinitive_pres: _, "~ᾰ~εσθαι", _;
    infinitive_fut: _, "~ᾰ~εσεσθαι", _;
    infinitive_aor: _, "~ᾰ~εσᾰσθαι", _;
    infinitive_perf: _, "~εμᾰ~ησθαι", _;
    participle_pres: _, "~ᾰ~ομενος", _;
    participle_fut: _, "~ᾰ~εσομενος", _;
    participle_aor: _, "~ᾰ~εσᾰμενος", _;
    participle_perf: _, "~εμᾰ~ημενος", _;
    ),
    // Σῳζω: 7 words (σοφῐζω, σῳζω, κεᾰζω, στρουθῐζω, τῠκῐζω, μερῐζω). Like Χρημᾰτῐζω, except in 2 cells: perfect_ind_mid_3 pl, pluperfect_mid_3 pl.
    conjugation!("σῳζω", 7, "~ζω", like Χρημᾰτῐζω;
    perfect_ind_mid_3: _, _, "%ε~σᾰται";
    pluperfect_mid_3: _, _, "ἐ%ε~σᾰτο";
    ),
    // Σφᾰλλω: 7 words (θᾰλλω, πᾰλλω, βδᾰλλω, σφᾰλλω, ψᾰλλω, ἰᾰλλω). Like Ξαινω, except in 161 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("σφᾰλλω", 7, "~ᾰ~λω", like Ξαινω;
    stem aorist_ind: "~η~- like ποιεω";
    stem aorist_ind_pass: "~ᾰ~η- like ἐχω";
    stem pluperfect_act: "~ᾰ~κε- like κᾰλεω";
    ind_act_1: "~ᾰ~λω", _, "~ᾰ~λομεν";
    ind_act_2: "~ᾰ~λεις", "~ᾰ~λετον", "~ᾰ~λετε";
    ind_act_3: "~ᾰ~λει", _, "~ᾰ~λουσῐ|~ᾰ~λουσῐν";
    ind_mid_1: "~ᾰ~λομαι", _, "~ᾰ~λομεθᾰ";
    ind_mid_2: "~ᾰ~λῃ|~ᾰ~λει", "~ᾰ~λεσθον", "~ᾰ~λεσθε";
    ind_mid_3: "~ᾰ~λεται", _, "~ᾰ~λονται";
    subj_act_1: "~ᾰ~λω", _, "~ᾰ~λωμεν";
    subj_act_2: "~ᾰ~λῃς", "~ᾰ~λητον", "~ᾰ~λητε";
    subj_act_3: "~ᾰ~λῃ", _, "~ᾰ~λωσῐ|~ᾰ~λωσῐν";
    subj_mid_1: "~ᾰ~λωμαι", _, "~ᾰ~λωμεθᾰ";
    subj_mid_2: _, "~ᾰ~λησθον", "~ᾰ~λησθε";
    subj_mid_3: "~ᾰ~ληται", _, "~ᾰ~λωνται";
    opt_act_1: "~ᾰ~λοιμῐ", _, "~ᾰ~λοιμεν";
    opt_act_2: "~ᾰ~λοις", "~ᾰ~λοιτον", "~ᾰ~λοιτε";
    opt_act_3: "~ᾰ~λοι", "~ᾰ~λοιτην", "~ᾰ~λοιεν";
    opt_mid_1: "~ᾰ~λοιμην", _, "~ᾰ~λοιμεθᾰ";
    opt_mid_2: "~ᾰ~λοιο", "~ᾰ~λοισθον", "~ᾰ~λοισθε";
    opt_mid_3: "~ᾰ~λοιτο", "~ᾰ~λοισθην", "~ᾰ~λοιντο";
    imp_act_2: "~ᾰ~λε", _, _;
    imp_act_3: "~ᾰ~λετω", "~ᾰ~λετων", "~ᾰ~λοντων";
    imp_mid_2: "~ᾰ~λου", _, _;
    imp_mid_3: "~ᾰ~λεσθω", "~ᾰ~λεσθων", _;
    imperfect_act_1: "ἐ~ᾰ~λον", _, "ἐ~ᾰ~λομεν";
    imperfect_act_2: "ἐ~ᾰ~λες", "ἐ~ᾰ~λετον", "ἐ~ᾰ~λετε";
    imperfect_act_3: "ἐ~ᾰ~λε|ἐ~ᾰ~λεν", "ἐ~ᾰ~λετην", "ἐ~ᾰ~λον";
    imperfect_mid_1: "ἐ~ᾰ~λομην", _, "ἐ~ᾰ~λομεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~λου", "ἐ~ᾰ~λεσθον", "ἐ~ᾰ~λεσθε";
    imperfect_mid_3: "ἐ~ᾰ~λετο", "ἐ~ᾰ~λεσθην", "ἐ~ᾰ~λοντο";
    future_ind_pass_1: "~ᾰ~ησησομαι", _, "~ᾰ~ησησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ησησῃ", "~ᾰ~ησησεσθον", "~ᾰ~ησησεσθε";
    future_ind_pass_3: "~ᾰ~ησησεται", _, "~ᾰ~ησησονται";
    future_opt_pass_1: "~ᾰ~ησησοιμην", _, "~ᾰ~ησησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ησησοιο", "~ᾰ~ησησοισθον", "~ᾰ~ησησοισθε";
    future_opt_pass_3: "~ᾰ~ησησοιτο", "~ᾰ~ησησοισθην", "~ᾰ~ησησοιντο";
    aorist_subj_act_3: _, _, "~η~ωσῐ|~η~ωσῐν";
    aorist_subj_pass_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    aorist_subj_pass_2: "~ᾰ~ῃς", "~ᾰ~ητον", "~ᾰ~ητε";
    aorist_subj_pass_3: "~ᾰ~ῃ", _, "~ᾰ~ωσῐ";
    aorist_opt_act_2: "~η~ειᾰς|~η~αις", _, _;
    aorist_opt_act_3: "~η~ειε|~η~ειεν|~η~αι", _, "~η~ειᾰν|~η~αιεν";
    aorist_opt_pass_1: "~ᾰ~ειην", _, "~ᾰ~εῑμεν";
    aorist_opt_pass_2: "~ᾰ~ειης", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    aorist_opt_pass_3: "~ᾰ~ειη", "~ᾰ~ειτην", "~ᾰ~εῑεν";
    aorist_imp_pass_2: "~ᾰ~ηθῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ητω", "~ᾰ~ητων", "~ᾰ~εντων";
    perfect_ind_act_1: "ἐ~ᾰ~κᾰ", _, "ἐ~ᾰ~κᾰμεν";
    perfect_ind_act_2: "ἐ~ᾰ~κᾰς", "ἐ~ᾰ~κᾰτον", "ἐ~ᾰ~κᾰτε";
    perfect_ind_act_3: "ἐ~ᾰ~κε", _, "ἐ~ᾰ~κᾱσῐ";
    perfect_ind_mid_1: "ἐ~ᾰ~μαι", _, "ἐ~ᾰ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰ~σαι", "ἐ~ᾰ~θον", "ἐ~ᾰ~θε";
    perfect_ind_mid_3: "ἐ~ᾰ~ται", _, "ἐ~ᾰ~ᾰται";
    perfect_subj_act_1: "ἐ~ᾰ~κω", _, "ἐ~ᾰ~κωμεν";
    perfect_subj_act_2: "ἐ~ᾰ~κῃς", "ἐ~ᾰ~κητον", "ἐ~ᾰ~κητε";
    perfect_subj_act_3: "ἐ~ᾰ~κῃ", _, "ἐ~ᾰ~κωσῐ";
    perfect_opt_act_1: "ἐ~ᾰ~κοιμῐ", _, "ἐ~ᾰ~κοιμεν";
    perfect_opt_act_2: "ἐ~ᾰ~κοις", "ἐ~ᾰ~κοιτον", "ἐ~ᾰ~κοιτε";
    perfect_opt_act_3: "ἐ~ᾰ~κοι", "ἐ~ᾰ~κοιτην", "ἐ~ᾰ~κοιεν";
    perfect_imp_act_2: _, "ἐ~ᾰ~κετον", "ἐ~ᾰ~κετε";
    perfect_imp_act_3: "ἐ~ᾰ~κετω", "ἐ~ᾰ~κετων", "ἐ~ᾰ~κοντων";
    perfect_imp_mid_2: "ἐ~ᾰ~σο", _, _;
    perfect_imp_mid_3: "ἐ~ᾰ~θω", "ἐ~ᾰ~θων", _;
    pluperfect_mid_1: "ἐ~ᾰ~μην", _, "ἐ~ᾰ~μεθᾰ";
    pluperfect_mid_2: "ἐ~ᾰ~σο", "ἐ~ᾰ~θον", "ἐ~ᾰ~θε";
    pluperfect_mid_3: "ἐ~ᾰ~το", "ἐ~ᾰ~θην", "ἐ~ᾰ~ᾰτο";
    infinitive_pres: "~ᾰ~λειν", "~ᾰ~λεσθαι", _;
    infinitive_fut: _, _, "~ᾰ~ησησεσθαι";
    infinitive_aor: _, _, "~ᾰ~ηναι";
    infinitive_perf: "ἐ~ᾰ~κεναι", "ἐ~ᾰ~θαι", _;
    participle_pres: "~ᾰ~λων", "~ᾰ~λομενος", _;
    participle_fut: _, _, "~ᾰ~ησησομενος";
    participle_aor: _, _, "~ᾰ~εις";
    participle_perf: "ἐ~ᾰ~κως", "ἐ~ᾰ~μενος", _;
    ),
    // Ἀπεῑπον: 7 words (προσεῑπον, ἐξεῑπον, προεῑπον, ἀπεῑπον, ὑπεῑπον, ἐπεῑπον). Like Μῑγνῡμῐ, except in 158 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("ἀπεῑπον", 7, "~εῑπον", like Μῑγνῡμῐ as SG_AS_PL;
    stem aorist_ind: "~ε- like ἀπεῑπον";
    stem aorist_ind_pass: "~ερρηθη- like ἐλπω";
    ind_act_1: "~εῑπον", _, _;
    future_ind_act_1: "~ερω", _, "~εροῡμεν";
    future_ind_act_2: "~ερεῑς", "~ερεῑτον", "~ερεῑτε";
    future_ind_act_3: "~ερεῑ", _, "~εροῡσῐ";
    future_ind_mid_1: "~εροῡμαι", _, "~ερουμεθᾰ";
    future_ind_mid_2: "~ερεῑ", "~ερεῑσθον", "~ερεῑσθε";
    future_ind_mid_3: "~ερεῑται", _, "~εροῡνται";
    future_ind_pass_1: "~ορρηθησομαι", _, "~ορρηθησομεθᾰ";
    future_ind_pass_2: "~ορρηθησῃ", "~ορρηθησεσθον", "~ορρηθησεσθε";
    future_ind_pass_3: "~ορρηθησεται", _, "~ορρηθησονται";
    future_opt_act_1: "~εροιην", _, "~εροῑμεν";
    future_opt_act_2: "~εροιης", "~εροῑτον", "~εροῑτε";
    future_opt_act_3: "~εροιη", "~εροιτην", "~εροῑεν";
    future_opt_mid_1: "~εροιμην", _, "~εροιμεθᾰ";
    future_opt_mid_2: "~εροῑο", "~εροῑσθον", "~εροῑσθε";
    future_opt_mid_3: "~εροῑτο", "~εροισθην", "~εροῑντο";
    future_opt_pass_1: "~ορρηθησοιμην", _, "~ορρηθησοιμεθᾰ";
    future_opt_pass_2: "~ορρηθησοιο", "~ορρηθησοισθον", "~ορρηθησοισθε";
    future_opt_pass_3: "~ορρηθησοιτο", "~ορρηθησοισθην", "~ορρηθησοιντο";
    aorist_subj_act_1: "~ειπω", _, "~ειπωμεν";
    aorist_subj_act_2: "~ειπῃς", "~ειπητον", "~ειπητε";
    aorist_subj_act_3: "~ειπῃ", _, "~ειπωσῐ";
    aorist_subj_mid_1: "~ειπωμαι", _, "~ειπωμεθᾰ";
    aorist_subj_mid_2: _, "~ειπησθον", "~ειπησθε";
    aorist_subj_mid_3: "~ειπηται", _, "~ειπωνται";
    aorist_subj_pass_1: "~ορρηθω", _, "~ορρηθωμεν";
    aorist_subj_pass_2: "~ορρηθῃς", "~ορρηθητον", "~ορρηθητε";
    aorist_subj_pass_3: "~ορρηθῃ", _, "~ορρηθωσῐ";
    aorist_opt_act_1: "~ειποιμῐ", _, "~ειποιμεν";
    aorist_opt_act_2: "~ειποις", "~ειποιτον", "~ειποιτε";
    aorist_opt_act_3: "~ειποι", "~ειποιτην", "~ειποιεν";
    aorist_opt_mid_1: "~ειποιμην", _, "~ειποιμεθᾰ";
    aorist_opt_mid_2: "~ειποιο", "~ειποισθον", "~ειποισθε";
    aorist_opt_mid_3: "~ειποιτο", "~ειποισθην", "~ειποιντο";
    aorist_opt_pass_1: "~ορρηθειην", _, "~ορρηθεῑμεν";
    aorist_opt_pass_2: "~ορρηθειης", "~ορρηθεῑτον", "~ορρηθεῑτε";
    aorist_opt_pass_3: "~ορρηθειη", "~ορρηθειτην", "~ορρηθεῑεν";
    aorist_imp_act_2: "~ειπε", "~ειπετον", "~ειπετε";
    aorist_imp_act_3: "~ειπετω", "~ειπετων", "~ειποντων";
    aorist_imp_mid_2: "~ειποῡ", "~ειπεσθον", "~ειπεσθε";
    aorist_imp_mid_3: "~ειπεσθω", "~ειπεσθων", _;
    aorist_imp_pass_2: "~ορρηθητῐ", _, _;
    aorist_imp_pass_3: "~ορρηθητω", "~ορρηθητων", "~ορρηθεντων";
    perfect_ind_act_1: "~ειρηκᾰ", _, "~ειρηκᾰμεν";
    perfect_ind_act_2: "~ειρηκᾰς", "~ειρηκᾰτον", "~ειρηκᾰτε";
    perfect_ind_act_3: "~ειρηκε", _, "~ειρηκᾱσῐ";
    perfect_ind_mid_1: "~ειρημαι", _, "~ειρημεθᾰ";
    perfect_ind_mid_2: "~ειρησαι", "~ειρησθον", "~ειρησθε";
    perfect_ind_mid_3: "~ειρηται", _, "~ειρηνται";
    perfect_subj_act_1: "~ειρηκω", _, "~ειρηκωμεν";
    perfect_subj_act_2: "~ειρηκῃς", "~ειρηκητον", "~ειρηκητε";
    perfect_subj_act_3: "~ειρηκῃ", _, "~ειρηκωσῐ";
    perfect_opt_act_1: "~ειρηκοιμῐ", _, "~ειρηκοιμεν";
    perfect_opt_act_2: "~ειρηκοις", "~ειρηκοιτον", "~ειρηκοιτε";
    perfect_opt_act_3: "~ειρηκοι", "~ειρηκοιτην", "~ειρηκοιεν";
    perfect_imp_act_2: _, "~ειρηκετον", "~ειρηκετε";
    perfect_imp_act_3: "~ειρηκετω", "~ειρηκετων", "~ειρηκοντων";
    perfect_imp_mid_2: "~ειρησο", _, _;
    perfect_imp_mid_3: "~ειρησθω", "~ειρησθων", _;
    infinitive_fut: "~ερεῑν", "~ερεῑσθαι", "~ορρηθησεσθαι";
    infinitive_aor: "~ειπεῑν", "~ειπεσθαι", "~ορρηθηναι";
    infinitive_perf: "~ειρηκεναι", "~ειρησθαι", _;
    participle_fut: "~ερων", "~ερουμενος", "~ορρηθησομενος";
    participle_aor: "~ειπων", "~ειπομενος", "~ορρηθεις";
    participle_perf: "~ειρηκως", "~ειρημενος", _;
    ),
    // Κᾰθιδρῡω: 7 words (ἑλκω, ἱκω, κᾰθιδρῡω, εἰκω, ἐρῡκω, ἐφηκω). Like Ἐλπω, except in 88 cells: ind_mid_2 sg, future_ind_act_3 pl, future_ind_mid_2 sg, future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, ….
    conjugation!("κᾰθιδρῡω", 7, "~ω", like Ἐλπω;
    stem aorist_ind: "~σ- like εὐλογεω";
    ind_mid_2: "~ῃ|~ει", _, _;
    future_ind_act_3: _, _, "~σουσῐ|~σουσῐν";
    future_ind_mid_2: "~σῃ|~σει", _, _;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_pass_1: derived, _, derived;
    aorist_ind_pass_2: derived, derived, derived;
    aorist_ind_pass_3: derived, derived, derived;
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_subj_pass_1: derived, _, derived;
    aorist_subj_pass_2: derived, derived, derived;
    aorist_subj_pass_3: derived, _, derived;
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", _, "~σειᾰν|~σαιεν";
    aorist_opt_pass_1: derived, _, derived;
    aorist_opt_pass_2: derived, derived, derived;
    aorist_opt_pass_3: derived, derived, derived;
    aorist_imp_pass_2: derived, _, _;
    aorist_imp_pass_3: derived, derived, derived;
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: "~κᾰς", "~κᾰτον", "~κᾰτε";
    perfect_ind_act_3: "~κε|~κεν", _, "~κᾱσῐ|~κᾱσῐν";
    perfect_ind_mid_1: "~μαι", _, "~μεθᾰ";
    perfect_ind_mid_3: "~ται", _, "~νται";
    perfect_subj_act_1: "~κω", _, "~κωμεν";
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ|~κωσῐν";
    perfect_opt_act_1: "~κοιμῐ|~κοιην", _, "~κοιμεν";
    perfect_opt_act_2: "~κοις|~κοιης", "~κοιτον", "~κοιτε";
    perfect_opt_act_3: "~κοι|~κοιη", "~κοιτην", "~κοιεν";
    perfect_imp_act_2: "~κε", "~κετον", "~κετε";
    perfect_imp_act_3: "~κετω", "~κετων", "~κοντων";
    infinitive_fut: _, _, derived;
    infinitive_aor: _, _, derived;
    infinitive_perf: "~κεναι", _, _;
    participle_fut: _, _, derived;
    participle_aor: _, _, derived;
    participle_perf: "~κως", "~μενος", _;
    ),
    // Χραω: 7 words (χραω, νωμαω, σκῐαω, ἀπαγχοναω, λοχαω, πασχητῐαω). Like Νῑκαω, except in 38 cells: future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, future_ind_pass_2 du, future_ind_pass_2 pl, future_ind_pass_3 sg, ….
    conjugation!("χραω", 7, "~αω", like Νῑκαω;
    stem aorist_ind_pass: "~ησθη- like ἐχω";
    stem pluperfect_act: "~ηκε- like ἐχω";
    stem pluperfect: "~η- like πειρᾰω";
    future_ind_pass_1: "~ησθησομαι", _, "~ησθησομεθᾰ";
    future_ind_pass_2: "~ησθησῃ", "~ησθησεσθον", "~ησθησεσθε";
    future_ind_pass_3: "~ησθησεται", _, "~ησθησονται";
    future_opt_pass_1: "~ησθησοιμην", _, "~ησθησοιμεθᾰ";
    future_opt_pass_2: "~ησθησοιο", "~ησθησοισθον", "~ησθησοισθε";
    future_opt_pass_3: "~ησθησοιτο", "~ησθησοισθην", "~ησθησοιντο";
    aorist_subj_pass_1: "~ησθω", _, "~ησθωμεν";
    aorist_subj_pass_2: "~ησθῃς", "~ησθητον", "~ησθητε";
    aorist_subj_pass_3: "~ησθῃ", _, "~ησθωσῐ";
    aorist_opt_pass_1: "~ησθειην", _, "~ησθεῑμεν";
    aorist_opt_pass_2: "~ησθειης", "~ησθεῑτον", "~ησθεῑτε";
    aorist_opt_pass_3: "~ησθειη", "~ησθειτην", "~ησθεῑεν";
    aorist_imp_pass_2: "~ησθητῐ", _, _;
    aorist_imp_pass_3: "~ησθητω", "~ησθητων", "~ησθεντων";
    infinitive_fut: _, _, "~ησθησεσθαι";
    infinitive_aor: _, _, "~ησθηναι";
    participle_fut: _, _, "~ησθησομενος";
    participle_aor: _, _, "~ησθεις";
    ),
    // Λλῡω: 7 words (λῡω, δῡω, καμμῡω, καταμῡω, κᾰσσῡω, περῐλῡω). Like Φῠω, except in 170 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("λλῡω", 7, "~ῡω", like Φῠω;
    stem aorist_ind: "~ῡσ- like ποιεω";
    stem aorist_ind_pass: "~ῠθη- like ἐχω";
    stem pluperfect_act: "~ῠκε- like ἐχω";
    stem pluperfect: "~ῠ- like πειρᾰω";
    ind_act_1: "~ῡω", _, "~ῡομεν";
    ind_act_2: "~ῡεις", "~ῡετον", "~ῡετε";
    ind_act_3: "~ῡει", _, "~ῡουσῐ|~ῡουσῐν";
    ind_mid_1: "~ῡομαι", _, "~ῡομεθᾰ";
    ind_mid_2: "~ῡῃ|~ῡει", "~ῡεσθον", "~ῡεσθε";
    ind_mid_3: "~ῡεται", _, "~ῡονται";
    subj_act_1: "~ῡω", _, "~ῡωμεν";
    subj_act_2: "~ῡῃς", "~ῡητον", "~ῡητε";
    subj_act_3: "~ῡῃ", _, "~ῡωσῐ|~ῡωσῐν";
    subj_mid_1: "~ῡωμαι", _, "~ῡωμεθᾰ";
    subj_mid_2: _, "~ῡησθον", "~ῡησθε";
    subj_mid_3: "~ῡηται", _, "~ῡωνται";
    opt_act_1: "~ῡοιμῐ", _, "~ῡοιμεν";
    opt_act_2: "~ῡοις", "~ῡοιτον", "~ῡοιτε";
    opt_act_3: "~ῡοι", "~ῡοιτην", "~ῡοιεν";
    opt_mid_1: "~ῡοιμην", _, "~ῡοιμεθᾰ";
    opt_mid_2: "~ῡοιο", "~ῡοισθον", "~ῡοισθε";
    opt_mid_3: "~ῡοιτο", "~ῡοισθην", "~ῡοιντο";
    imp_act_2: "~ῡε", _, _;
    imp_act_3: "~ῡετω", "~ῡετων", "~ῡοντων";
    imp_mid_2: "~ῡου", _, _;
    imp_mid_3: "~ῡεσθω", "~ῡεσθων", _;
    imperfect_act_1: "ἐ~ῡον", _, "ἐ~ῡομεν";
    imperfect_act_2: "ἐ~ῡες", "ἐ~ῡετον", "ἐ~ῡετε";
    imperfect_act_3: "ἐ~ῡε|ἐ~ῡεν", "ἐ~ῡετην", "ἐ~ῡον";
    imperfect_mid_1: "ἐ~ῡομην", _, "ἐ~ῡομεθᾰ";
    imperfect_mid_2: "ἐ~ῡου", "ἐ~ῡεσθον", "ἐ~ῡεσθε";
    imperfect_mid_3: "ἐ~ῡετο", "ἐ~ῡεσθην", "ἐ~ῡοντο";
    future_ind_act_3: _, _, "~ῡσουσῐ|~ῡσουσῐν";
    future_ind_mid_2: "~ῡσῃ|~ῡσει", _, _;
    future_ind_pass_1: "~ῠθησομαι", _, "~ῠθησομεθᾰ";
    future_ind_pass_2: "~ῠθησῃ|~ῠθησει", "~ῠθησεσθον", "~ῠθησεσθε";
    future_ind_pass_3: "~ῠθησεται", _, "~ῠθησονται";
    future_opt_pass_1: "~ῠθησοιμην", _, "~ῠθησοιμεθᾰ";
    future_opt_pass_2: "~ῠθησοιο", "~ῠθησοισθον", "~ῠθησοισθε";
    future_opt_pass_3: "~ῠθησοιτο", "~ῠθησοισθην", "~ῠθησοιντο";
    aorist_subj_act_3: _, _, "~ῡσωσῐ|~ῡσωσῐν";
    aorist_subj_pass_1: "~ῠθω", _, "~ῠθωμεν";
    aorist_subj_pass_2: "~ῠθῃς", "~ῠθητον", "~ῠθητε";
    aorist_subj_pass_3: "~ῠθῃ", _, "~ῠθωσῐ|~ῠθωσῐν";
    aorist_opt_pass_1: "~ῠθειην", _, "~ῠθεῑμεν|~ῠθειημεν";
    aorist_opt_pass_2: "~ῠθειης", "~ῠθεῑτον|~ῠθειητον", "~ῠθεῑτε|~ῠθειητε";
    aorist_opt_pass_3: "~ῠθειη", "~ῠθειτην|~ῠθειητην", "~ῠθεῑεν|~ῠθειησᾰν";
    aorist_imp_pass_2: "~ῠθητῐ", _, _;
    aorist_imp_pass_3: "~ῠθητω", "~ῠθητων", "~ῠθεντων";
    perfect_ind_act_1: "%ε~ῠκᾰ", _, "%ε~ῠκᾰμεν";
    perfect_ind_act_2: "%ε~ῠκᾰς", "%ε~ῠκᾰτον", "%ε~ῠκᾰτε";
    perfect_ind_act_3: "%ε~ῠκε|%ε~ῠκεν", _, "%ε~ῠκᾱσῐ|%ε~ῠκᾱσῐν";
    perfect_ind_mid_1: "%ε~ῠμαι", _, "%ε~ῠμεθᾰ";
    perfect_ind_mid_2: "%ε~ῠσαι", "%ε~ῠσθον", "%ε~ῠσθε";
    perfect_ind_mid_3: "%ε~ῠται", _, "%ε~ῠνται";
    perfect_subj_act_1: "%ε~ῠκω", _, "%ε~ῠκωμεν";
    perfect_subj_act_2: "%ε~ῠκῃς", "%ε~ῠκητον", "%ε~ῠκητε";
    perfect_subj_act_3: "%ε~ῠκῃ", _, "%ε~ῠκωσῐ|%ε~ῠκωσῐν";
    perfect_opt_act_1: "%ε~ῠκοιμῐ|%ε~ῠκοιην", _, "%ε~ῠκοιμεν";
    perfect_opt_act_2: "%ε~ῠκοις|%ε~ῠκοιης", "%ε~ῠκοιτον", "%ε~ῠκοιτε";
    perfect_opt_act_3: "%ε~ῠκοι|%ε~ῠκοιη", "%ε~ῠκοιτην", "%ε~ῠκοιεν";
    perfect_imp_act_2: "%ε~ῠκε", "%ε~ῠκετον", "%ε~ῠκετε";
    perfect_imp_act_3: "%ε~ῠκετω", "%ε~ῠκετων", "%ε~ῠκοντων";
    perfect_imp_mid_2: "%ε~ῠσο", _, _;
    perfect_imp_mid_3: "%ε~ῠσθω", "%ε~ῠσθων", _;
    future_perfect_ind_mid_1: "%ε~ῡσομαι", _, "%ε~ῡσομεθᾰ";
    future_perfect_ind_mid_2: "%ε~ῡσῃ", "%ε~ῡσεσθον", "%ε~ῡσεσθε";
    future_perfect_ind_mid_3: "%ε~ῡσεται", _, "%ε~ῡσονται";
    future_perfect_opt_mid_1: "%ε~ῡσοιμην", _, "%ε~ῡσοιμεθᾰ";
    future_perfect_opt_mid_2: "%ε~ῡσοιο", "%ε~ῡσοισθον", "%ε~ῡσοισθε";
    future_perfect_opt_mid_3: "%ε~ῡσοιτο", "%ε~ῡσοισθην", "%ε~ῡσοιντο";
    infinitive_pres: "~ῡειν", "~ῡεσθαι", _;
    infinitive_fut: _, _, "~ῠθησεσθαι";
    infinitive_aor: _, _, "~ῠθηναι";
    infinitive_perf: "%ε~ῠκεναι", "%ε~ῠσθαι", _;
    infinitive_futp: _, "%ε~ῡσεσθαι", _;
    participle_pres: "~ῡων", "~ῡομενος", _;
    participle_fut: _, _, "~ῠθησομενος";
    participle_aor: _, _, "~ῠθεις";
    participle_perf: "%ε~ῠκως", "%ε~ῠμενος", _;
    participle_futp: _, "%ε~ῡσομενος", _;
    ),
    // Θλαω: 7 words (ἀγαλλιαω, κρεμαω, θλαω, τηταω, ἐλᾰω, νεᾰω). Like Νῑκαω, except in 157 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("θλαω", 7, "~αω", like Νῑκαω;
    stem aorist_ind: "~ασ- like ἐχω";
    stem aorist_ind_pass: "~ασθη- like ἐχω";
    stem pluperfect_act: "~ακε- like ἐχω";
    stem pluperfect: "~α- like πειρᾰω";
    future_ind_act_1: "~ασω", _, "~ασομεν";
    future_ind_act_2: "~ασεις", "~ασετον", "~ασετε";
    future_ind_act_3: "~ασει", _, "~ασουσῐ";
    future_ind_mid_1: "~ασομαι", _, "~ασομεθᾰ";
    future_ind_mid_2: "~ασῃ", "~ασεσθον", "~ασεσθε";
    future_ind_mid_3: "~ασεται", _, "~ασονται";
    future_ind_pass_1: "~ασθησομαι", _, "~ασθησομεθᾰ";
    future_ind_pass_2: "~ασθησῃ", "~ασθησεσθον", "~ασθησεσθε";
    future_ind_pass_3: "~ασθησεται", _, "~ασθησονται";
    future_opt_act_1: "~ασοιμῐ", _, "~ασοιμεν";
    future_opt_act_2: "~ασοις", "~ασοιτον", "~ασοιτε";
    future_opt_act_3: "~ασοι", "~ασοιτην", "~ασοιεν";
    future_opt_mid_1: "~ασοιμην", _, "~ασοιμεθᾰ";
    future_opt_mid_2: "~ασοιο", "~ασοισθον", "~ασοισθε";
    future_opt_mid_3: "~ασοιτο", "~ασοισθην", "~ασοιντο";
    future_opt_pass_1: "~ασθησοιμην", _, "~ασθησοιμεθᾰ";
    future_opt_pass_2: "~ασθησοιο", "~ασθησοισθον", "~ασθησοισθε";
    future_opt_pass_3: "~ασθησοιτο", "~ασθησοισθην", "~ασθησοιντο";
    aorist_subj_act_1: "~ασω", _, "~ασωμεν";
    aorist_subj_act_2: "~ασῃς", "~ασητον", "~ασητε";
    aorist_subj_act_3: "~ασῃ", _, "~ασωσῐ";
    aorist_subj_mid_1: "~ασωμαι", _, "~ασωμεθᾰ";
    aorist_subj_mid_2: _, "~ασησθον", "~ασησθε";
    aorist_subj_mid_3: "~ασηται", _, "~ασωνται";
    aorist_subj_pass_1: "~ασθω", _, "~ασθωμεν";
    aorist_subj_pass_2: "~ασθῃς", "~ασθητον", "~ασθητε";
    aorist_subj_pass_3: "~ασθῃ", _, "~ασθωσῐ";
    aorist_opt_act_1: "~ασαιμῐ", _, "~ασαιμεν";
    aorist_opt_act_2: "~ασειᾰς", "~ασαιτον", "~ασαιτε";
    aorist_opt_act_3: "~ασειε", "~ασαιτην", "~ασειᾰν";
    aorist_opt_mid_1: "~ασαιμην", _, "~ασαιμεθᾰ";
    aorist_opt_mid_2: "~ασαιο", "~ασαισθον", "~ασαισθε";
    aorist_opt_mid_3: "~ασαιτο", "~ασαισθην", "~ασαιντο";
    aorist_opt_pass_1: "~ασθειην", _, "~ασθεῑμεν";
    aorist_opt_pass_2: "~ασθειης", "~ασθεῑτον", "~ασθεῑτε";
    aorist_opt_pass_3: "~ασθειη", "~ασθειτην", "~ασθεῑεν";
    aorist_imp_act_2: "~ᾱσον", "~ασᾰτον", "~ασᾰτε";
    aorist_imp_act_3: "~ασᾰτω", "~ασᾰτων", "~ασᾰντων";
    aorist_imp_mid_2: "~ᾱσαι", "~ασᾰσθον", "~ασᾰσθε";
    aorist_imp_mid_3: "~ασᾰσθω", "~ασᾰσθων", _;
    aorist_imp_pass_2: "~ασθητῐ", _, _;
    aorist_imp_pass_3: "~ασθητω", "~ασθητων", "~ασθεντων";
    perfect_ind_act_1: "%ε~ακᾰ", _, "%ε~ακᾰμεν";
    perfect_ind_act_2: "%ε~ακᾰς", "%ε~ακᾰτον", "%ε~ακᾰτε";
    perfect_ind_act_3: "%ε~ακε", _, "%ε~ακᾱσῐ";
    perfect_ind_mid_1: "%ε~αμαι", _, "%ε~αμεθᾰ";
    perfect_ind_mid_2: "%ε~ασαι", "%ε~ασθον", "%ε~ασθε";
    perfect_ind_mid_3: "%ε~αται", _, "%ε~ανται";
    perfect_subj_act_1: "%ε~ακω", _, "%ε~ακωμεν";
    perfect_subj_act_2: "%ε~ακῃς", "%ε~ακητον", "%ε~ακητε";
    perfect_subj_act_3: "%ε~ακῃ", _, "%ε~ακωσῐ";
    perfect_opt_act_1: "%ε~ακοιμῐ", _, "%ε~ακοιμεν";
    perfect_opt_act_2: "%ε~ακοις", "%ε~ακοιτον", "%ε~ακοιτε";
    perfect_opt_act_3: "%ε~ακοι", "%ε~ακοιτην", "%ε~ακοιεν";
    perfect_imp_act_2: _, "%ε~ακετον", "%ε~ακετε";
    perfect_imp_act_3: "%ε~ακετω", "%ε~ακετων", "%ε~ακοντων";
    perfect_imp_mid_2: "%ε~ασο", _, _;
    perfect_imp_mid_3: "%ε~ασθω", "%ε~ασθων", _;
    infinitive_fut: "~ασειν", "~ασεσθαι", "~ασθησεσθαι";
    infinitive_aor: "~ᾱσαι", "~ασᾰσθαι", "~ασθηναι";
    infinitive_perf: "%ε~ακεναι", "%ε~ᾱσθαι", _;
    participle_fut: "~ασων", "~ασομενος", "~ασθησομενος";
    participle_aor: "~ασᾱς", "~ασᾰμενος", "~ασθεις";
    participle_perf: "%ε~ακως", "%ε~αμενος", _;
    ),
    // Παυω: 6 words (γευω, παυω, κρουω, χναυω, χραυω, θραυω). Like Βουλευω, except in 41 cells: imp_act_2 sg, future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, future_ind_pass_2 du, future_ind_pass_2 pl, ….
    conjugation!("παυω", 6, "~υω", like Βουλευω;
    stem aorist_ind_pass: "~υσθη- like ἐχω";
    imp_act_2: "~ῡε", _, _;
    future_ind_pass_1: "~υσθησομαι", _, "~υσθησομεθᾰ";
    future_ind_pass_2: "~υσθησῃ", "~υσθησεσθον", "~υσθησεσθε";
    future_ind_pass_3: "~υσθησεται", _, "~υσθησονται";
    future_opt_pass_1: "~υσθησοιμην", _, "~υσθησοιμεθᾰ";
    future_opt_pass_2: "~υσθησοιο", "~υσθησοισθον", "~υσθησοισθε";
    future_opt_pass_3: "~υσθησοιτο", "~υσθησοισθην", "~υσθησοιντο";
    aorist_subj_pass_1: "~υσθω", _, "~υσθωμεν";
    aorist_subj_pass_2: "~υσθῃς", "~υσθητον", "~υσθητε";
    aorist_subj_pass_3: "~υσθῃ", _, "~υσθωσῐ";
    aorist_opt_pass_1: "~υσθειην", _, "~υσθεῑμεν";
    aorist_opt_pass_2: "~υσθειης", "~υσθεῑτον", "~υσθεῑτε";
    aorist_opt_pass_3: "~υσθειη", "~υσθειτην", "~υσθεῑεν";
    aorist_imp_act_2: "~ῡσον", _, _;
    aorist_imp_mid_2: "~ῡσαι", _, _;
    aorist_imp_pass_2: "~υσθητῐ", _, _;
    aorist_imp_pass_3: "~υσθητω", "~υσθητων", "~υσθεντων";
    infinitive_fut: _, _, "~υσθησεσθαι";
    infinitive_aor: _, _, "~υσθηναι";
    participle_fut: _, _, "~υσθησομενος";
    participle_aor: _, _, "~υσθεις";
    ),
    // Τελεω: 6 words (ἐμεω, ἀρκεω, τελεω, βδεω, νεικεω, δῐᾰρκεω). Like Πᾰραιτεομαι, except in 159 cells: ind_mid_2 sg, ind_pass_2 sg, future_ind_mid_2 sg, future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, ….
    conjugation!("τελεω", 6, "~εω", like Πᾰραιτεομαι as FUTURE_AS_PRESENT;
    stem aorist_ind: "~εσ- like ἐχω";
    stem aorist_ind_pass: "~εσθη- like ἐχω";
    stem pluperfect_act: "~εκε- like ἐχω";
    ind_mid_2: "~εῑ", _, _;
    ind_pass_2: derived, _, _;
    future_ind_mid_2: "~εῑ", _, _;
    future_ind_pass_1: "~εσθησομαι", _, "~εσθησομεθᾰ";
    future_ind_pass_2: "~εσθησῃ", "~εσθησεσθον", "~εσθησεσθε";
    future_ind_pass_3: "~εσθησεται", _, "~εσθησονται";
    future_subj_act_1: derived, _, derived;
    future_subj_act_2: derived, derived, derived;
    future_subj_act_3: derived, _, derived;
    future_subj_mid_1: derived, _, derived;
    future_subj_mid_2: _, derived, derived;
    future_subj_mid_3: derived, _, derived;
    future_opt_pass_1: "~εσθησοιμην", _, "~εσθησοιμεθᾰ";
    future_opt_pass_2: "~εσθησοιο", "~εσθησοισθον", "~εσθησοισθε";
    future_opt_pass_3: "~εσθησοιτο", "~εσθησοισθην", "~εσθησοιντο";
    future_imp_act_2: derived, _, _;
    future_imp_act_3: derived, derived, derived;
    future_imp_mid_2: derived, _, _;
    future_imp_mid_3: derived, derived, _;
    aorist_subj_act_1: "~εσω", _, "~εσωμεν";
    aorist_subj_act_2: "~εσῃς", "~εσητον", "~εσητε";
    aorist_subj_act_3: "~εσῃ", _, "~εσωσῐ";
    aorist_subj_mid_1: "~εσωμαι", _, "~εσωμεθᾰ";
    aorist_subj_mid_2: derived, "~εσησθον", "~εσησθε";
    aorist_subj_mid_3: "~εσηται", _, "~εσωνται";
    aorist_subj_pass_1: "~εσθω", _, "~εσθωμεν";
    aorist_subj_pass_2: "~εσθῃς", "~εσθητον", "~εσθητε";
    aorist_subj_pass_3: "~εσθῃ", _, "~εσθωσῐ";
    aorist_opt_act_1: "~εσαιμῐ", _, "~εσαιμεν";
    aorist_opt_act_2: "~εσειᾰς", "~εσαιτον", "~εσαιτε";
    aorist_opt_act_3: "~εσειε", "~εσαιτην", "~εσειᾰν";
    aorist_opt_mid_1: "~εσαιμην", _, "~εσαιμεθᾰ";
    aorist_opt_mid_2: "~εσαιο", "~εσαισθον", "~εσαισθε";
    aorist_opt_mid_3: "~εσαιτο", "~εσαισθην", "~εσαιντο";
    aorist_opt_pass_1: "~εσθειην", _, "~εσθεῑμεν";
    aorist_opt_pass_2: "~εσθειης", "~εσθεῑτον", "~εσθεῑτε";
    aorist_opt_pass_3: "~εσθειη", "~εσθειτην", "~εσθεῑεν";
    aorist_imp_act_2: "~εσον", "~εσᾰτον", "~εσᾰτε";
    aorist_imp_act_3: "~εσᾰτω", "~εσᾰτων", "~εσᾰντων";
    aorist_imp_mid_2: "~εσαι", "~εσᾰσθον", "~εσᾰσθε";
    aorist_imp_mid_3: "~εσᾰσθω", "~εσᾰσθων", _;
    aorist_imp_pass_2: "~εσθητῐ", _, _;
    aorist_imp_pass_3: "~εσθητω", "~εσθητων", "~εσθεντων";
    perfect_ind_act_1: "%ε~εκᾰ", _, "%ε~εκᾰμεν";
    perfect_ind_act_2: "%ε~εκᾰς", "%ε~εκᾰτον", "%ε~εκᾰτε";
    perfect_ind_act_3: "%ε~εκε", _, "%ε~εκᾱσῐ";
    perfect_ind_mid_1: "%ε~εσμαι", _, "%ε~εσμεθᾰ";
    perfect_ind_mid_2: "%ε~εσαι", "%ε~εσθον", "%ε~εσθε";
    perfect_ind_mid_3: "%ε~εσται", _, "%ε~εσᾰται";
    perfect_subj_act_1: "%ε~εκω", _, "%ε~εκωμεν";
    perfect_subj_act_2: "%ε~εκῃς", "%ε~εκητον", "%ε~εκητε";
    perfect_subj_act_3: "%ε~εκῃ", _, "%ε~εκωσῐ";
    perfect_opt_act_1: "%ε~εκοιμῐ", _, "%ε~εκοιμεν";
    perfect_opt_act_2: "%ε~εκοις", "%ε~εκοιτον", "%ε~εκοιτε";
    perfect_opt_act_3: "%ε~εκοι", "%ε~εκοιτην", "%ε~εκοιεν";
    perfect_imp_act_2: _, "%ε~εκετον", "%ε~εκετε";
    perfect_imp_act_3: "%ε~εκετω", "%ε~εκετων", "%ε~εκοντων";
    perfect_imp_mid_2: "%ε~εσο", _, _;
    perfect_imp_mid_3: "%ε~εσθω", "%ε~εσθων", _;
    pluperfect_mid_1: "ἐ%ε~εσμην", _, "ἐ%ε~εσμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~εσο", "ἐ%ε~εσθον", "ἐ%ε~εσθε";
    pluperfect_mid_3: "ἐ%ε~εστο", "ἐ%ε~εσθην", "ἐ%ε~εσᾰτο";
    infinitive_fut: "~εῑν", "~εῑσθαι", "~εσθησεσθαι";
    infinitive_aor: "~εσαι", "~εσᾰσθαι", "~εσθηναι";
    infinitive_perf: "%ε~εκεναι", "%ε~εσθαι", _;
    participle_fut: "~ων", "~ουμενος", "~εσθησομενος";
    participle_aor: "~εσᾱς", "~εσᾰμενος", "~εσθεις";
    participle_perf: "%ε~εκως", "%ε~εσμενος", _;
    ),
    // Ἀγορευω: 6 words (ἀγρευω, ἀγορευω, ἀμορβευω, ἀριστευω, ἀπῐστευω, ἀσωτευω). Like Ἐχω, except in 253 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀγορευω", 6, "ἀ~υω", like Ἐχω;
    stem aorist_ind: "~υσ- like ἀγορευω";
    stem aorist_ind_pass: "~υθη- like ἀγορευω";
    stem pluperfect_act: "~υκε- like ἀγορευω";
    ind_act_1: "ἀ~υω", _, "ἀ~υομεν";
    ind_act_2: "ἀ~υεις", "ἀ~υετον", "ἀ~υετε";
    ind_act_3: "ἀ~υει", _, "ἀ~υουσῐ|ἀ~υουσῐν";
    ind_mid_1: "ἀ~υομαι", _, "ἀ~υομεθᾰ";
    ind_mid_2: "ἀ~υῃ", "ἀ~υεσθον", "ἀ~υεσθε";
    ind_mid_3: "ἀ~υεται", _, "ἀ~υονται";
    subj_act_1: "ἀ~υω", _, "ἀ~υωμεν";
    subj_act_2: "ἀ~υῃς", "ἀ~υητον", "ἀ~υητε";
    subj_act_3: "ἀ~υῃ", _, "ἀ~υωσῐ|ἀ~υωσῐν";
    subj_mid_1: "ἀ~υωμαι", _, "ἀ~υωμεθᾰ";
    subj_mid_2: _, "ἀ~υησθον", "ἀ~υησθε";
    subj_mid_3: "ἀ~υηται", _, "ἀ~υωνται";
    opt_act_1: "ἀ~υοιμῐ", _, "ἀ~υοιμεν";
    opt_act_2: "ἀ~υοις", "ἀ~υοιτον", "ἀ~υοιτε";
    opt_act_3: "ἀ~υοι", "ἀ~υοιτην", "ἀ~υοιεν";
    opt_mid_1: "ἀ~υοιμην", _, "ἀ~υοιμεθᾰ";
    opt_mid_2: "ἀ~υοιο", "ἀ~υοισθον", "ἀ~υοισθε";
    opt_mid_3: "ἀ~υοιτο", "ἀ~υοισθην", "ἀ~υοιντο";
    imp_act_2: "ἀ~υε", _, _;
    imp_act_3: "ἀ~υετω", "ἀ~υετων", "ἀ~υοντων";
    imp_mid_2: "ἀ~υου", _, _;
    imp_mid_3: "ἀ~υεσθω", "ἀ~υεσθων", _;
    imperfect_act_1: "ἠ~υον", _, "ἠ~υομεν";
    imperfect_act_2: "ἠ~υες", "ἠ~υετον", "ἠ~υετε";
    imperfect_act_3: "ἠ~υε|ἠ~υεν", "ἠ~υετην", "ἠ~υον";
    imperfect_mid_1: "ἠ~υομην", _, "ἠ~υομεθᾰ";
    imperfect_mid_2: "ἠ~υου", "ἠ~υεσθον", "ἠ~υεσθε";
    imperfect_mid_3: "ἠ~υετο", "ἠ~υεσθην", "ἠ~υοντο";
    future_ind_act_1: "ἀ~υσω", _, "ἀ~υσομεν";
    future_ind_act_2: "ἀ~υσεις", "ἀ~υσετον", "ἀ~υσετε";
    future_ind_act_3: "ἀ~υσει", _, "ἀ~υσουσῐ|ἀ~υσουσῐν";
    future_ind_mid_1: "ἀ~υσομαι", _, "ἀ~υσομεθᾰ";
    future_ind_mid_2: "ἀ~υσῃ", "ἀ~υσεσθον", "ἀ~υσεσθε";
    future_ind_mid_3: "ἀ~υσεται", _, "ἀ~υσονται";
    future_ind_pass_1: "ἀ~υθησομαι", _, "ἀ~υθησομεθᾰ";
    future_ind_pass_2: "ἀ~υθησῃ", "ἀ~υθησεσθον", "ἀ~υθησεσθε";
    future_ind_pass_3: "ἀ~υθησεται", _, "ἀ~υθησονται";
    future_opt_act_1: "ἀ~υσοιμῐ", _, "ἀ~υσοιμεν";
    future_opt_act_2: "ἀ~υσοις", "ἀ~υσοιτον", "ἀ~υσοιτε";
    future_opt_act_3: "ἀ~υσοι", "ἀ~υσοιτην", "ἀ~υσοιεν";
    future_opt_mid_1: "ἀ~υσοιμην", _, "ἀ~υσοιμεθᾰ";
    future_opt_mid_2: "ἀ~υσοιο", "ἀ~υσοισθον", "ἀ~υσοισθε";
    future_opt_mid_3: "ἀ~υσοιτο", "ἀ~υσοισθην", "ἀ~υσοιντο";
    future_opt_pass_1: "ἀ~υθησοιμην", _, "ἀ~υθησοιμεθᾰ";
    future_opt_pass_2: "ἀ~υθησοιο", "ἀ~υθησοισθον", "ἀ~υθησοισθε";
    future_opt_pass_3: "ἀ~υθησοιτο", "ἀ~υθησοισθην", "ἀ~υθησοιντο";
    aorist_subj_act_1: "ἀ~υσω", _, "ἀ~υσωμεν";
    aorist_subj_act_2: "ἀ~υσῃς", "ἀ~υσητον", "ἀ~υσητε";
    aorist_subj_act_3: "ἀ~υσῃ", _, "ἀ~υσωσῐ";
    aorist_subj_mid_1: "ἀ~υσωμαι", _, "ἀ~υσωμεθᾰ";
    aorist_subj_mid_2: _, "ἀ~υσησθον", "ἀ~υσησθε";
    aorist_subj_mid_3: "ἀ~υσηται", _, "ἀ~υσωνται";
    aorist_subj_pass_1: "ἀ~υθω", _, "ἀ~υθωμεν";
    aorist_subj_pass_2: "ἀ~υθῃς", "ἀ~υθητον", "ἀ~υθητε";
    aorist_subj_pass_3: "ἀ~υθῃ", _, "ἀ~υθωσῐ";
    aorist_opt_act_1: "ἀ~υσαιμῐ", _, "ἀ~υσαιμεν";
    aorist_opt_act_2: "ἀ~υσειᾰς", "ἀ~υσαιτον", "ἀ~υσαιτε";
    aorist_opt_act_3: "ἀ~υσειε", "ἀ~υσαιτην", "ἀ~υσειᾰν";
    aorist_opt_mid_1: "ἀ~υσαιμην", _, "ἀ~υσαιμεθᾰ";
    aorist_opt_mid_2: "ἀ~υσαιο", "ἀ~υσαισθον", "ἀ~υσαισθε";
    aorist_opt_mid_3: "ἀ~υσαιτο", "ἀ~υσαισθην", "ἀ~υσαιντο";
    aorist_opt_pass_1: "ἀ~υθειην", _, "ἀ~υθεῑμεν";
    aorist_opt_pass_2: "ἀ~υθειης", "ἀ~υθεῑτον", "ἀ~υθεῑτε";
    aorist_opt_pass_3: "ἀ~υθειη", "ἀ~υθειτην", "ἀ~υθεῑεν";
    aorist_imp_act_2: "ἀ~υσον", "ἀ~υσᾰτον", "ἀ~υσᾰτε";
    aorist_imp_act_3: "ἀ~υσᾰτω", "ἀ~υσᾰτων", "ἀ~υσᾰντων";
    aorist_imp_mid_2: "ἀ~υσαι", "ἀ~υσᾰσθον", "ἀ~υσᾰσθε";
    aorist_imp_mid_3: "ἀ~υσᾰσθω", "ἀ~υσᾰσθων", _;
    aorist_imp_pass_2: "ἀ~υθητῐ", _, _;
    aorist_imp_pass_3: "ἀ~υθητω", "ἀ~υθητων", "ἀ~υθεντων";
    perfect_ind_act_1: "ἠ~υκᾰ", _, "ἠ~υκᾰμεν";
    perfect_ind_act_2: "ἠ~υκᾰς", "ἠ~υκᾰτον", "ἠ~υκᾰτε";
    perfect_ind_act_3: "ἠ~υκε|ἠ~υκεν", _, "ἠ~υκᾱσῐ|ἠ~υκᾱσῐν";
    perfect_ind_mid_1: "ἠ~υμαι", _, "ἠ~υμεθᾰ";
    perfect_ind_mid_2: "ἠ~υσαι", "ἠ~υσθον", "ἠ~υσθε";
    perfect_ind_mid_3: "ἠ~υται", _, "ἠ~υνται";
    perfect_subj_act_1: "ἠ~υκω", _, "ἠ~υκωμεν";
    perfect_subj_act_2: "ἠ~υκῃς", "ἠ~υκητον", "ἠ~υκητε";
    perfect_subj_act_3: "ἠ~υκῃ", _, "ἠ~υκωσῐ|ἠ~υκωσῐν";
    perfect_opt_act_1: "ἠ~υκοιμῐ|ἠ~υκοιην", _, "ἠ~υκοιμεν";
    perfect_opt_act_2: "ἠ~υκοις|ἠ~υκοιης", "ἠ~υκοιτον", "ἠ~υκοιτε";
    perfect_opt_act_3: "ἠ~υκοι|ἠ~υκοιη", "ἠ~υκοιτην", "ἠ~υκοιεν";
    perfect_imp_act_2: "ἠ~υκε", "ἠ~υκετον", "ἠ~υκετε";
    perfect_imp_act_3: "ἠ~υκετω", "ἠ~υκετων", "ἠ~υκοντων";
    perfect_imp_mid_2: "ἠ~υσο", _, _;
    perfect_imp_mid_3: "ἠ~υσθω", "ἠ~υσθων", _;
    pluperfect_mid_1: "ἠ~υμην", _, "ἠ~υμεθᾰ";
    pluperfect_mid_2: "ἠ~υσο", "ἠ~υσθον", "ἠ~υσθε";
    pluperfect_mid_3: "ἠ~υτο", "ἠ~υσθην", "ἠ~υντο";
    future_perfect_ind_mid_1: "ἠ~υσομαι", _, "ἠ~υσομεθᾰ";
    future_perfect_ind_mid_2: "ἠ~υσῃ", "ἠ~υσεσθον", "ἠ~υσεσθε";
    future_perfect_ind_mid_3: "ἠ~υσεται", _, "ἠ~υσονται";
    future_perfect_opt_mid_1: "ἠ~υσοιμην", _, "ἠ~υσοιμεθᾰ";
    future_perfect_opt_mid_2: "ἠ~υσοιο", "ἠ~υσοισθον", "ἠ~υσοισθε";
    future_perfect_opt_mid_3: "ἠ~υσοιτο", "ἠ~υσοισθην", "ἠ~υσοιντο";
    infinitive_pres: "ἀ~υειν", "ἀ~υεσθαι", _;
    infinitive_fut: "ἀ~υσειν", "ἀ~υσεσθαι", "ἀ~υθησεσθαι";
    infinitive_aor: "ἀ~ῡσαι", "ἀ~υσᾰσθαι", "ἀ~υθηναι";
    infinitive_perf: "ἠ~υκεναι", "ἠ~ῡσθαι", _;
    infinitive_futp: _, "ἠ~υσεσθαι", _;
    participle_pres: "ἀ~υων", "ἀ~υομενος", _;
    participle_fut: "ἀ~υσων", "ἀ~υσομενος", "ἀ~υθησομενος";
    participle_aor: "ἀ~υσᾱς", "ἀ~υσᾰμενος", "ἀ~υθεις";
    participle_perf: "ἠ~υκως", "ἠ~υμενος", _;
    participle_futp: _, "ἠ~υσομενος", _;
    ),
    // Κατοικεω: 6 words (κατοικεω, νεκραγωγεω, δῐασκοπεω, παιδεραστεω, πᾰνηγῠρῐαρχεω, ἐξασθενεω). Like Ἀνῐστημῐ, except in 191 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("κατοικεω", 6, "~α~εω", like Ἀνῐστημῐ;
    stem aorist_ind: "~ᾰ~ησ- like ἐλπω";
    stem aorist_ind_pass: "~ᾰ~ηθη- like ἐλπω";
    stem pluperfect_act: "~ᾰ~ηκε- like ἀνῐστημῐ";
    ind_act_1: "~ᾰ~ω", _, "~ᾰ~οῡμεν";
    ind_act_2: "~ᾰ~εῑς", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    ind_act_3: "~ᾰ~εῑ", _, "~ᾰ~οῡσῐ|~ᾰ~οῡσῐν";
    ind_mid_1: "~ᾰ~οῡμαι", _, "~ᾰ~ουμεθᾰ";
    ind_mid_2: "~ᾰ~εῑ", "~ᾰ~εῑσθον", "~ᾰ~εῑσθε";
    ind_mid_3: "~ᾰ~εῑται", _, "~ᾰ~οῡνται";
    subj_act_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    subj_act_2: "~ᾰ~ῃς", "~ᾰ~ητον", "~ᾰ~ητε";
    subj_act_3: "~ᾰ~ῃ", _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ησθον", "~ᾰ~ησθε";
    subj_mid_3: "~ᾰ~ηται", _, "~ᾰ~ωνται";
    opt_act_1: "~ᾰ~οιην|~ᾰ~οῑμῐ", _, "~ᾰ~οῑμεν|~ᾰ~οιημεν";
    opt_act_2: "~ᾰ~οιης|~ᾰ~οῑς", "~ᾰ~οῑτον|~ᾰ~οιητον", "~ᾰ~οῑτε|~ᾰ~οιητε";
    opt_act_3: "~ᾰ~οιη|~ᾰ~οῑ", "~ᾰ~οιτην|~ᾰ~οιητην", "~ᾰ~οῑεν|~ᾰ~οιησᾰν";
    opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~οῑο", "~ᾰ~οῑσθον", "~ᾰ~οῑσθε";
    opt_mid_3: "~ᾰ~οῑτο", "~ᾰ~οισθην", "~ᾰ~οῑντο";
    imp_act_2: "~ᾰ~ει", _, _;
    imp_act_3: "~ᾰ~ειτω", "~ᾰ~ειτων", "~ᾰ~ουντων";
    imp_mid_2: "~ᾰ~οῡ", _, _;
    imp_mid_3: "~ᾰ~εισθω", "~ᾰ~εισθων", _;
    imperfect_act_1: "~ᾰ~ουν", _, "~ᾰ~οῡμεν";
    imperfect_act_2: "~ᾰ~εις", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    imperfect_act_3: "~ᾰ~ει", "~ᾰ~ειτην", "~ᾰ~ουν";
    imperfect_mid_1: "~ᾰ~ουμην", _, "~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "~ᾰ~οῡ", "~ᾰ~εῑσθον", "~ᾰ~εῑσθε";
    imperfect_mid_3: "~ᾰ~εῑτο", "~ᾰ~εισθην", "~ᾰ~οῡντο";
    future_ind_pass_1: "~ᾰ~ηθησομαι", _, "~ᾰ~ηθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ηθησῃ", "~ᾰ~ηθησεσθον", "~ᾰ~ηθησεσθε";
    future_ind_pass_3: "~ᾰ~ηθησεται", _, "~ᾰ~ηθησονται";
    future_opt_pass_1: "~ᾰ~ηθησοιμην", _, "~ᾰ~ηθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ηθησοιο", "~ᾰ~ηθησοισθον", "~ᾰ~ηθησοισθε";
    future_opt_pass_3: "~ᾰ~ηθησοιτο", "~ᾰ~ηθησοισθην", "~ᾰ~ηθησοιντο";
    aorist_subj_pass_1: "~ᾰ~ηθω", _, "~ᾰ~ηθωμεν";
    aorist_subj_pass_2: "~ᾰ~ηθῃς", "~ᾰ~ηθητον", "~ᾰ~ηθητε";
    aorist_subj_pass_3: "~ᾰ~ηθῃ", _, "~ᾰ~ηθωσῐ";
    aorist_opt_pass_1: "~ᾰ~ηθειην", _, "~ᾰ~ηθεῑμεν";
    aorist_opt_pass_2: "~ᾰ~ηθειης", "~ᾰ~ηθεῑτον", "~ᾰ~ηθεῑτε";
    aorist_opt_pass_3: "~ᾰ~ηθειη", "~ᾰ~ηθειτην", "~ᾰ~ηθεῑεν";
    aorist_imp_pass_2: "~ᾰ~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ηθητω", "~ᾰ~ηθητων", "~ᾰ~ηθεντων";
    perfect_ind_act_1: "~ᾰ~ηκᾰ", _, "~ᾰ~ηκᾰμεν";
    perfect_ind_act_2: "~ᾰ~ηκᾰς", "~ᾰ~ηκᾰτον", "~ᾰ~ηκᾰτε";
    perfect_ind_act_3: "~ᾰ~ηκε", _, "~ᾰ~ηκᾱσῐ";
    perfect_ind_mid_1: "~ᾰ~ημαι", _, "~ᾰ~ημεθᾰ";
    perfect_ind_mid_2: "~ᾰ~ησαι", "~ᾰ~ησθον", "~ᾰ~ησθε";
    perfect_ind_mid_3: "~ᾰ~ηται", _, "~ᾰ~ηνται";
    perfect_subj_act_1: "~ᾰ~ηκω", _, "~ᾰ~ηκωμεν";
    perfect_subj_act_2: "~ᾰ~ηκῃς", "~ᾰ~ηκητον", "~ᾰ~ηκητε";
    perfect_subj_act_3: "~ᾰ~ηκῃ", _, "~ᾰ~ηκωσῐ";
    perfect_opt_act_1: "~ᾰ~ηκοιμῐ", _, "~ᾰ~ηκοιμεν";
    perfect_opt_act_2: "~ᾰ~ηκοις", "~ᾰ~ηκοιτον", "~ᾰ~ηκοιτε";
    perfect_opt_act_3: "~ᾰ~ηκοι", "~ᾰ~ηκοιτην", "~ᾰ~ηκοιεν";
    perfect_imp_act_2: _, "~ᾰ~ηκετον", "~ᾰ~ηκετε";
    perfect_imp_act_3: "~ᾰ~ηκετω", "~ᾰ~ηκετων", "~ᾰ~ηκοντων";
    perfect_imp_mid_2: "~ᾰ~ησο", _, _;
    perfect_imp_mid_3: "~ᾰ~ησθω", "~ᾰ~ησθων", _;
    pluperfect_mid_1: "~ᾰ~ημην", _, "~ᾰ~ημεθᾰ";
    pluperfect_mid_2: "~ᾰ~ησο", "~ᾰ~ησθον", "~ᾰ~ησθε";
    pluperfect_mid_3: "~ᾰ~ητο", "~ᾰ~ησθην", "~ᾰ~ηντο";
    future_perfect_ind_act_1: derived, _, derived;
    future_perfect_ind_act_2: derived, derived, derived;
    future_perfect_ind_act_3: derived, _, derived;
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_opt_act_1: derived, _, derived;
    future_perfect_opt_act_2: derived, derived, derived;
    future_perfect_opt_act_3: derived, derived, derived;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_pres: "~ᾰ~εῑν", "~ᾰ~εῑσθαι", _;
    infinitive_fut: _, _, "~ᾰ~ηθησεσθαι";
    infinitive_aor: _, _, "~ᾰ~ηθηναι";
    infinitive_perf: "~ᾰ~ηκεναι", "~ᾰ~ησθαι", _;
    infinitive_futp: derived, derived, _;
    participle_pres: "~ᾰ~ων", "~ᾰ~ουμενος", _;
    participle_fut: _, _, "~ᾰ~ηθησομενος";
    participle_aor: _, _, "~ᾰ~ηθεις";
    participle_perf: "~ᾰ~ηκως", "~ᾰ~ημενος", _;
    participle_futp: derived, derived, _;
    ),
    // Λᾰγοθηρεω: 6 words (κρατεω, φωνομᾰχεω, τερᾰτοτοκεω, λᾰγοθηρεω, λᾰβρηγορεω, λᾰρκοφορεω). Like Κατοικεω, except in 71 cells: ind_mid_2 sg, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, ….
    conjugation!("λᾰγοθηρεω", 6, "~ᾰ~εω", like Κατοικεω;
    stem aorist_ind: "~α~ησ- like ποιεω";
    stem aorist_ind_pass: "~α~ηθη- like ἐχω";
    stem pluperfect_act: "~α~ηκε- like ἐχω";
    stem pluperfect: "~α~η- like πειρᾰω";
    ind_mid_2: "~ᾰ~εῑ|~ᾰ~ῃ", _, _;
    imperfect_act_1: "ἐ~α~ουν", _, "ἐ~α~οῡμεν";
    imperfect_act_2: "ἐ~α~εις", "ἐ~α~εῑτον", "ἐ~α~εῑτε";
    imperfect_act_3: "ἐ~α~ει", "ἐ~α~ειτην", "ἐ~α~ουν";
    imperfect_mid_1: "ἐ~α~ουμην", _, "ἐ~α~ουμεθᾰ";
    imperfect_mid_2: "ἐ~α~οῡ", "ἐ~α~εῑσθον", "ἐ~α~εῑσθε";
    imperfect_mid_3: "ἐ~α~εῑτο", "ἐ~α~εισθην", "ἐ~α~οῡντο";
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ|~ᾰ~ησουσῐν";
    future_ind_mid_2: "~ᾰ~ησῃ|~ᾰ~ησει", _, _;
    future_ind_pass_2: "~ᾰ~ηθησῃ|~ᾰ~ηθησει", _, _;
    aorist_subj_act_3: _, _, "~ᾰ~ησωσῐ|~ᾰ~ησωσῐν";
    aorist_subj_pass_3: _, _, "~ᾰ~ηθωσῐ|~ᾰ~ηθωσῐν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς|~ᾰ~ησαις", _, _;
    aorist_opt_act_3: "~ᾰ~ησειε|~ᾰ~ησειεν|~ᾰ~ησαι", _, "~ᾰ~ησειᾰν|~ᾰ~ησαιεν";
    aorist_opt_pass_1: _, _, "~ᾰ~ηθεῑμεν|~ᾰ~ηθειημεν";
    aorist_opt_pass_2: _, "~ᾰ~ηθεῑτον|~ᾰ~ηθειητον", "~ᾰ~ηθεῑτε|~ᾰ~ηθειητε";
    aorist_opt_pass_3: _, "~ᾰ~ηθειτην|~ᾰ~ηθειητην", "~ᾰ~ηθεῑεν|~ᾰ~ηθειησᾰν";
    perfect_ind_act_1: "%ε~α~ηκᾰ", _, "%ε~α~ηκᾰμεν";
    perfect_ind_act_2: "%ε~α~ηκᾰς", "%ε~α~ηκᾰτον", "%ε~α~ηκᾰτε";
    perfect_ind_act_3: "%ε~α~ηκε", _, "%ε~α~ηκᾱσῐ";
    perfect_ind_mid_1: "%ε~α~ημαι", _, "%ε~α~ημεθᾰ";
    perfect_ind_mid_2: "%ε~α~ησαι", "%ε~α~ησθον", "%ε~α~ησθε";
    perfect_ind_mid_3: "%ε~α~ηται", _, "%ε~α~ηνται";
    perfect_subj_act_1: "%ε~α~ηκω", _, "%ε~α~ηκωμεν";
    perfect_subj_act_2: "%ε~α~ηκῃς", "%ε~α~ηκητον", "%ε~α~ηκητε";
    perfect_subj_act_3: "%ε~α~ηκῃ", _, "%ε~α~ηκωσῐ";
    perfect_opt_act_1: "%ε~α~ηκοιμῐ", _, "%ε~α~ηκοιμεν";
    perfect_opt_act_2: "%ε~α~ηκοις", "%ε~α~ηκοιτον", "%ε~α~ηκοιτε";
    perfect_opt_act_3: "%ε~α~ηκοι", "%ε~α~ηκοιτην", "%ε~α~ηκοιεν";
    perfect_imp_act_2: _, "%ε~α~ηκετον", "%ε~α~ηκετε";
    perfect_imp_act_3: "%ε~α~ηκετω", "%ε~α~ηκετων", "%ε~α~ηκοντων";
    perfect_imp_mid_2: "%ε~α~ησο", _, _;
    perfect_imp_mid_3: "%ε~α~ησθω", "%ε~α~ησθων", _;
    infinitive_perf: "%ε~α~ηκεναι", "%ε~α~ησθαι", _;
    participle_perf: "%ε~α~ηκως", "%ε~α~ημενος", _;
    ),
    // Μηκᾰομαι: 6 words (ἰᾰομαι, θεᾰομαι, ἀκροᾰομαι, αἰτῐᾰομαι, ἡσσᾰομαι, μηκᾰομαι). Like Ἀραομαι, except in 97 cells: ind_act_1 sg, imperfect_mid_1 sg, imperfect_mid_1 pl, imperfect_mid_2 sg, imperfect_mid_2 du, imperfect_mid_2 pl, ….
    conjugation!("μηκᾰομαι", 6, "~ᾰομαι", like Ἀραομαι as PERF_AS_PRES;
    stem aorist_ind: "~ᾱσ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ᾱθη- like ἐλπω";
    ind_act_1: "~ᾰομαι", _, _;
    imperfect_mid_1: "~ωμην", _, "~ωμεθᾰ";
    imperfect_mid_2: "~ω", "~ᾱσθον", "~ᾱσθε";
    imperfect_mid_3: "~ᾱτο", "~ᾱσθην", "~ωντο";
    future_ind_mid_1: "~ᾱσομαι", _, "~ᾱσομεθᾰ";
    future_ind_mid_2: "~ᾱσῃ", "~ᾱσεσθον", "~ᾱσεσθε";
    future_ind_mid_3: "~ᾱσεται", _, "~ᾱσονται";
    future_ind_pass_1: "~ᾱθησομαι", _, "~ᾱθησομεθᾰ";
    future_ind_pass_2: "~ᾱθησῃ", "~ᾱθησεσθον", "~ᾱθησεσθε";
    future_ind_pass_3: "~ᾱθησεται", _, "~ᾱθησονται";
    future_opt_mid_1: "~ᾱσοιμην", _, "~ᾱσοιμεθᾰ";
    future_opt_mid_2: "~ᾱσοιο", "~ᾱσοισθον", "~ᾱσοισθε";
    future_opt_mid_3: "~ᾱσοιτο", "~ᾱσοισθην", "~ᾱσοιντο";
    future_opt_pass_1: "~ᾱθησοιμην", _, "~ᾱθησοιμεθᾰ";
    future_opt_pass_2: "~ᾱθησοιο", "~ᾱθησοισθον", "~ᾱθησοισθε";
    future_opt_pass_3: "~ᾱθησοιτο", "~ᾱθησοισθην", "~ᾱθησοιντο";
    aorist_subj_mid_1: "~ᾱσωμαι", _, "~ᾱσωμεθᾰ";
    aorist_subj_mid_2: "~ᾱσῃ", "~ᾱσησθον", "~ᾱσησθε";
    aorist_subj_mid_3: "~ᾱσηται", _, "~ᾱσωνται";
    aorist_subj_pass_1: "~ᾱθω", _, "~ᾱθωμεν";
    aorist_subj_pass_2: "~ᾱθῃς", "~ᾱθητον", "~ᾱθητε";
    aorist_subj_pass_3: "~ᾱθῃ", _, "~ᾱθωσῐ";
    aorist_opt_mid_1: "~ᾱσαιμην", _, "~ᾱσαιμεθᾰ";
    aorist_opt_mid_2: "~ᾱσαιο", "~ᾱσαισθον", "~ᾱσαισθε";
    aorist_opt_mid_3: "~ᾱσαιτο", "~ᾱσαισθην", "~ᾱσαιντο";
    aorist_opt_pass_1: "~ᾱθειην", _, "~ᾱθεῑμεν";
    aorist_opt_pass_2: "~ᾱθειης", "~ᾱθεῑτον", "~ᾱθεῑτε";
    aorist_opt_pass_3: "~ᾱθειη", "~ᾱθειτην", "~ᾱθεῑεν";
    aorist_imp_mid_2: "~ᾱσαι", "~ᾱσᾰσθον", "~ᾱσᾰσθε";
    aorist_imp_mid_3: "~ᾱσᾰσθω", "~ᾱσᾰσθων", _;
    aorist_imp_pass_2: "~ᾱθητῐ", _, _;
    aorist_imp_pass_3: "~ᾱθητω", "~ᾱθητων", "~ᾱθεντων";
    perfect_ind_mid_1: "~ᾱμαι", _, "~ᾱμεθᾰ";
    perfect_ind_mid_2: "~ᾱσαι", "~ᾱσθον", "~ᾱσθε";
    perfect_ind_mid_3: "~ᾱται", _, "~ᾱνται";
    perfect_imp_mid_2: "~ᾱσο", _, _;
    perfect_imp_mid_3: "~ᾱσθω", "~ᾱσθων", _;
    infinitive_fut: _, "~ᾱσεσθαι", "~ᾱθησεσθαι";
    infinitive_aor: _, "~ᾱσᾰσθαι", "~ᾱθηναι";
    participle_fut: _, "~ᾱσομενος", "~ᾱθησομενος";
    participle_aor: _, "~ᾱσᾰμενος", "~ᾱθεις";
    participle_perf: _, "~ᾱμενος", _;
    ),
    // Ἐπεξειμι: 6 words (ἐξειμι, ἀπειμι, κᾰτειμῐ, προσειμι, προειμι, ἐπεξειμι). Like Μῑγνῡμῐ, except in 36 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐπεξειμι", 6, "~ειμι", like Μῑγνῡμῐ;
    ind_act_1: "~ειμι", _, "~ῐμεν";
    ind_act_2: "~ει", "~ῐτον", "~ῐτε";
    ind_act_3: "~εισῐ|~εισῐν", _, "~ῐᾱσῐ|~ῐᾱσῐν";
    subj_act_1: "~ῐω", _, "~ῐωμεν";
    subj_act_2: "~ῐῃς", "~ῐητον", "~ῐητε";
    subj_act_3: "~ῐῃ", _, "~ῐωσῐ|~ῐωσῐν";
    opt_act_1: "~ῐοιμῐ", _, "~ῐοιμεν";
    opt_act_2: "~ῐοις", "~ῐοιτον", "~ῐοιτε";
    opt_act_3: "~ῐοι", "~ῐοιτην", "~ῐοιεν";
    imp_act_2: "~ῐθῐ", _, _;
    imp_act_3: "~ῐτω", "~ῐτων", "~ῐοντων";
    imperfect_act_1: "~ῃᾰ|~ῃειν", _, "~ῃμεν";
    imperfect_act_2: "~ῃεισθᾰ|~ῃεις", "~ῃτον", "~ῃτε";
    imperfect_act_3: "~ῃειν|~ῃει", "~ῃτην", "~ῃσᾰν|~ῃεσᾰν";
    infinitive_pres: "~ῐεναι", _, _;
    participle_pres: "~ῐων", _, _;
    ),
    // Ἐαω: 6 words (ἐαω, θηραω, ἀπερᾱω, ἐξερᾱω, θῡμῐαω, φωραω). Like Μηκᾰομαι, except in 129 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐαω", 6, "~αω", like Μηκᾰομαι;
    stem aorist_ind: "~ᾱσ- like ποιεω";
    stem aorist_ind_pass: "~ᾱθη- like ἐχω";
    ind_act_1: "~ω", _, "~ωμεν";
    ind_act_2: "~ᾳς", "~ᾱτον", "~ᾱτε";
    ind_act_3: "~ᾳ", _, "~ωσῐ|~ωσῐν";
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ᾳς", "~ᾱτον", "~ᾱτε";
    subj_act_3: "~ᾳ", _, "~ωσῐ|~ωσῐν";
    subj_mid_2: derived, _, _;
    subj_pass_2: derived, _, _;
    opt_act_1: "~ῳην|~ῳμῐ", _, "~ῳμεν|~ῳημεν";
    opt_act_2: "~ῳης|~ῳς", "~ῳτον|~ῳητον", "~ῳτε|~ῳητε";
    opt_act_3: "~ῳη|~ῳ", "~ῳτην|~ῳητην", "~ῳεν|~ῳησᾰν";
    imp_act_2: "~ᾱ", _, _;
    imp_act_3: "~ᾱτω", "~ᾱτων", "~ωντων";
    imperfect_act_1: "ἐ~ων", _, "ἐ~ωμεν";
    imperfect_act_2: "ἐ~ᾱς", "ἐ~ᾱτον", "ἐ~ᾱτε";
    imperfect_act_3: "ἐ~ᾱ", "ἐ~ᾱτην", "ἐ~ων";
    imperfect_mid_1: "ἐ~ωμην", _, "ἐ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ω", "ἐ~ᾱσθον", "ἐ~ᾱσθε";
    imperfect_mid_3: "ἐ~ᾱτο", "ἐ~ᾱσθην", "ἐ~ωντο";
    future_ind_act_1: "~ᾱσω", _, "~ᾱσομεν";
    future_ind_act_2: "~ᾱσεις", "~ᾱσετον", "~ᾱσετε";
    future_ind_act_3: "~ᾱσει", _, "~ᾱσουσῐ";
    future_ind_mid_2: "~ᾱσῃ|~ᾱσει", _, _;
    future_opt_act_1: "~ᾱσοιμῐ", _, "~ᾱσοιμεν";
    future_opt_act_2: "~ᾱσοις", "~ᾱσοιτον", "~ᾱσοιτε";
    future_opt_act_3: "~ᾱσοι", "~ᾱσοιτην", "~ᾱσοιεν";
    aorist_subj_act_1: "~ᾱσω", _, "~ᾱσωμεν";
    aorist_subj_act_2: "~ᾱσῃς", "~ᾱσητον", "~ᾱσητε";
    aorist_subj_act_3: "~ᾱσῃ", _, "~ᾱσωσῐ|~ᾱσωσῐν";
    aorist_subj_mid_2: derived, _, _;
    aorist_opt_act_1: "~ᾱσαιμῐ", _, "~ᾱσαιμεν";
    aorist_opt_act_2: "~ᾱσειᾰς|~ᾱσαις", "~ᾱσαιτον", "~ᾱσαιτε";
    aorist_opt_act_3: "~ᾱσειε|~ᾱσειεν|~ᾱσαι", "~ᾱσαιτην", "~ᾱσειᾰν|~ᾱσαιεν";
    aorist_imp_act_2: "~ᾱσον", "~ᾱσᾰτον", "~ᾱσᾰτε";
    aorist_imp_act_3: "~ᾱσᾰτω", "~ᾱσᾰτων", "~ᾱσᾰντων";
    perfect_ind_act_1: "%ε~ᾱκᾰ", _, "%ε~ᾱκᾰμεν";
    perfect_ind_act_2: "%ε~ᾱκᾰς", "%ε~ᾱκᾰτον", "%ε~ᾱκᾰτε";
    perfect_ind_act_3: "%ε~ᾱκε", _, "%ε~ᾱκᾱσῐ";
    perfect_ind_mid_1: "%ε~ᾰμαι", _, "%ε~ᾰμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾰσαι", "%ε~ᾰσθον", "%ε~ᾰσθε";
    perfect_ind_mid_3: "%ε~ᾰται", _, "%ε~ᾰνται";
    perfect_subj_act_1: "%ε~ᾱκω", _, "%ε~ᾱκωμεν";
    perfect_subj_act_2: "%ε~ᾱκῃς", "%ε~ᾱκητον", "%ε~ᾱκητε";
    perfect_subj_act_3: "%ε~ᾱκῃ", _, "%ε~ᾱκωσῐ";
    perfect_opt_act_1: "%ε~ᾱκοιμῐ", _, "%ε~ᾱκοιμεν";
    perfect_opt_act_2: "%ε~ᾱκοις", "%ε~ᾱκοιτον", "%ε~ᾱκοιτε";
    perfect_opt_act_3: "%ε~ᾱκοι", "%ε~ᾱκοιτην", "%ε~ᾱκοιεν";
    perfect_imp_act_2: _, "%ε~ᾱκετον", "%ε~ᾱκετε";
    perfect_imp_act_3: "%ε~ᾱκετω", "%ε~ᾱκετων", "%ε~ᾱκοντων";
    perfect_imp_mid_2: "%ε~ᾰσο", _, _;
    perfect_imp_mid_3: "%ε~ᾰσθω", "%ε~ᾰσθων", _;
    infinitive_pres: "~ᾱν", _, _;
    infinitive_fut: "~ᾱσειν", _, _;
    infinitive_aor: "~ᾱσαι", _, _;
    infinitive_perf: "%ε~ᾱκεναι", "%ε~ᾰσθαι", _;
    participle_pres: "~ων", _, _;
    participle_fut: "~ᾱσων", _, _;
    participle_aor: "~ᾱσᾱς", _, _;
    participle_perf: "%ε~ᾱκως", "%ε~ᾰμενος", _;
    ),
    // Ἐρχομαι: 6 words (ἐρχομαι, ἑζομαι, ἡδομαι, κᾰθεζομαι, ἀπαλθαινομαι, ὠρῡομαι). Like Κρῑνομαι, except in 34 cells: ind_mid_2 sg, future_ind_mid_1 sg, future_ind_mid_1 pl, future_ind_mid_2 sg, future_ind_mid_2 du, future_ind_mid_2 pl, ….
    conjugation!("ἐρχομαι", 6, "~ομαι", like Κρῑνομαι as AORIST_AS_PRESENT;
    stem imperfect: "~- like ὑποκοριζομαι";
    stem aorist_ind: "~- like ἐρχομαι";
    ind_mid_2: "~ῃ|~ει", _, _;
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, derived;
    future_ind_mid_3: derived, _, derived;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, derived, derived;
    future_opt_mid_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_subj_pass_2: derived, _, _;
    aorist_imp_mid_2: "~οῡ", "~εσθον", "~εσθε";
    infinitive_fut: _, derived, _;
    infinitive_aor: derived, "~εσθαι", _;
    participle_fut: _, derived, _;
    participle_aor: derived, "~ομενος", _;
    ),
    // Ἐπῐλᾰμβᾰνω: 6 words (ἐπῐλᾰμβᾰνω, προλᾰμβᾰνω, ὑπολᾰμβᾰνω, ἀπολᾰμβᾰνω, ξῠλλᾰμβᾰνω, προσλαμβᾰνω). Like Δερκομαι, except in 211 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐπῐλᾰμβᾰνω", 6, "~~λᾰμβᾰνω", like Δερκομαι as MID_AS_ACT;
    stem aorist_ind: "~νε~ᾰβ- like ἀνᾰγω";
    stem aorist_ind_pass: "~νε~ηφθη- like ἐλπω";
    ind_act_1: "~~λᾰμβᾰνω", _, "~~λᾰμβᾰνομεν";
    ind_act_2: "~~λᾰμβᾰνεις", "~~λᾰμβᾰνετον", "~~λᾰμβᾰνετε";
    ind_act_3: "~~λᾰμβᾰνει", _, "~~λᾰμβᾰνουσῐ";
    ind_mid_1: "~~λᾰμβᾰνομαι", _, "~~λᾰμβᾰνομεθᾰ";
    ind_mid_2: "~~λᾰμβᾰνῃ", "~~λᾰμβᾰνεσθον", "~~λᾰμβᾰνεσθε";
    ind_mid_3: "~~λᾰμβᾰνεται", _, "~~λᾰμβᾰνονται";
    subj_act_1: "~~λᾰμβᾰνω", _, "~~λᾰμβᾰνωμεν";
    subj_act_2: "~~λᾰμβᾰνῃς", "~~λᾰμβᾰνητον", "~~λᾰμβᾰνητε";
    subj_act_3: "~~λᾰμβᾰνῃ", _, "~~λᾰμβᾰνωσῐ";
    subj_mid_1: "~~λᾰμβᾰνωμαι", _, "~~λᾰμβᾰνωμεθᾰ";
    subj_mid_2: _, "~~λᾰμβᾰνησθον", "~~λᾰμβᾰνησθε";
    subj_mid_3: "~~λᾰμβᾰνηται", _, "~~λᾰμβᾰνωνται";
    subj_pass_2: derived, _, _;
    opt_act_1: "~~λᾰμβᾰνοιμῐ", _, "~~λᾰμβᾰνοιμεν";
    opt_act_2: "~~λᾰμβᾰνοις", "~~λᾰμβᾰνοιτον", "~~λᾰμβᾰνοιτε";
    opt_act_3: "~~λᾰμβᾰνοι", "~~λᾰμβᾰνοιτην", "~~λᾰμβᾰνοιεν";
    opt_mid_1: "~~λᾰμβᾰνοιμην", _, "~~λᾰμβᾰνοιμεθᾰ";
    opt_mid_2: "~~λᾰμβᾰνοιο", "~~λᾰμβᾰνοισθον", "~~λᾰμβᾰνοισθε";
    opt_mid_3: "~~λᾰμβᾰνοιτο", "~~λᾰμβᾰνοισθην", "~~λᾰμβᾰνοιντο";
    imp_act_2: "~~λᾰμβᾰνε", _, _;
    imp_act_3: "~~λᾰμβᾰνετω", "~~λᾰμβᾰνετων", "~~λᾰμβᾰνοντων";
    imp_mid_2: "~~λᾰμβᾰνου", _, _;
    imp_mid_3: "~~λᾰμβᾰνεσθω", "~~λᾰμβᾰνεσθων", _;
    imperfect_act_1: "~νε~ᾰμβᾰνον", _, "~νε~ᾰμβᾰνομεν";
    imperfect_act_2: "~νε~ᾰμβᾰνες", "~νε~ᾰμβᾰνετον", "~νε~ᾰμβᾰνετε";
    imperfect_act_3: "~νε~ᾰμβᾰνε", "~νε~ᾰμβᾰνετην", "~νε~ᾰμβᾰνον";
    imperfect_mid_1: "~νε~ᾰμβᾰνομην", _, "~νε~ᾰμβᾰνομεθᾰ";
    imperfect_mid_2: "~νε~ᾰμβᾰνου", "~νε~ᾰμβᾰνεσθον", "~νε~ᾰμβᾰνεσθε";
    imperfect_mid_3: "~νε~ᾰμβᾰνετο", "~νε~ᾰμβᾰνεσθην", "~νε~ᾰμβᾰνοντο";
    future_ind_mid_1: "~~ληψομαι", _, "~~ληψομεθᾰ";
    future_ind_mid_2: "~~ληψῃ", "~~ληψεσθον", "~~ληψεσθε";
    future_ind_mid_3: "~~ληψεται", _, "~~ληψονται";
    future_ind_pass_1: "~~ληφθησομαι", _, "~~ληφθησομεθᾰ";
    future_ind_pass_2: "~~ληφθησῃ", "~~ληφθησεσθον", "~~ληφθησεσθε";
    future_ind_pass_3: "~~ληφθησεται", _, "~~ληφθησονται";
    future_opt_mid_1: "~~ληψοιμην", _, "~~ληψοιμεθᾰ";
    future_opt_mid_2: "~~ληψοιο", "~~ληψοισθον", "~~ληψοισθε";
    future_opt_mid_3: "~~ληψοιτο", "~~ληψοισθην", "~~ληψοιντο";
    future_opt_pass_1: "~~ληφθησοιμην", _, "~~ληφθησοιμεθᾰ";
    future_opt_pass_2: "~~ληφθησοιο", "~~ληφθησοισθον", "~~ληφθησοισθε";
    future_opt_pass_3: "~~ληφθησοιτο", "~~ληφθησοισθην", "~~ληφθησοιντο";
    aorist_subj_act_1: "~~λᾰβω", _, "~~λᾰβωμεν";
    aorist_subj_act_2: "~~λᾰβῃς", "~~λᾰβητον", "~~λᾰβητε";
    aorist_subj_act_3: "~~λᾰβῃ", _, "~~λᾰβωσῐ";
    aorist_subj_mid_1: "~~λᾰβωμαι", _, "~~λᾰβωμεθᾰ";
    aorist_subj_mid_2: _, "~~λᾰβησθον", "~~λᾰβησθε";
    aorist_subj_mid_3: "~~λᾰβηται", _, "~~λᾰβωνται";
    aorist_subj_pass_1: "~~ληφθω", _, "~~ληφθωμεν";
    aorist_subj_pass_2: "~~ληφθῃς", "~~ληφθητον", "~~ληφθητε";
    aorist_subj_pass_3: "~~ληφθῃ", _, "~~ληφθωσῐ";
    aorist_opt_act_1: "~~λᾰβοιμῐ", _, "~~λᾰβοιμεν";
    aorist_opt_act_2: "~~λᾰβοις", "~~λᾰβοιτον", "~~λᾰβοιτε";
    aorist_opt_act_3: "~~λᾰβοι", "~~λᾰβοιτην", "~~λᾰβοιεν";
    aorist_opt_mid_1: "~~λᾰβοιμην", _, "~~λᾰβοιμεθᾰ";
    aorist_opt_mid_2: "~~λᾰβοιο", "~~λᾰβοισθον", "~~λᾰβοισθε";
    aorist_opt_mid_3: "~~λᾰβοιτο", "~~λᾰβοισθην", "~~λᾰβοιντο";
    aorist_opt_pass_1: "~~ληφθειην", _, "~~ληφθεῑμεν";
    aorist_opt_pass_2: "~~ληφθειης", "~~ληφθεῑτον", "~~ληφθεῑτε";
    aorist_opt_pass_3: "~~ληφθειη", "~~ληφθειτην", "~~ληφθεῑεν";
    aorist_imp_act_2: "~~λᾰβε", "~~λᾰβετον", "~~λᾰβετε";
    aorist_imp_act_3: "~~λᾰβετω", "~~λᾰβετων", "~~λᾰβοντων";
    aorist_imp_mid_2: "~~λᾰβοῡ", "~~λᾰβεσθον", "~~λᾰβεσθε";
    aorist_imp_mid_3: "~~λᾰβεσθω", "~~λᾰβεσθων", _;
    aorist_imp_pass_2: "~~ληφθητῐ", _, _;
    aorist_imp_pass_3: "~~ληφθητω", "~~ληφθητων", "~~ληφθεντων";
    perfect_ind_act_1: "~νει~ηφᾰ", _, "~νει~ηφᾰμεν";
    perfect_ind_act_2: "~νει~ηφᾰς", "~νει~ηφᾰτον", "~νει~ηφᾰτε";
    perfect_ind_act_3: "~νει~ηφε", _, "~νει~ηφᾱσῐ";
    perfect_ind_mid_1: "~νει~ημμαι", _, "~νει~ημμεθᾰ";
    perfect_ind_mid_2: "~νει~ηψαι", "~νει~ηφθον", "~νει~ηφθε";
    perfect_ind_mid_3: "~νει~ηπται", _, "~νει~ηφᾰται";
    perfect_subj_act_1: "~νει~ηφω", _, "~νει~ηφωμεν";
    perfect_subj_act_2: "~νει~ηφῃς", "~νει~ηφητον", "~νει~ηφητε";
    perfect_subj_act_3: "~νει~ηφῃ", _, "~νει~ηφωσῐ";
    perfect_opt_act_1: "~νει~ηφοιμῐ", _, "~νει~ηφοιμεν";
    perfect_opt_act_2: "~νει~ηφοις", "~νει~ηφοιτον", "~νει~ηφοιτε";
    perfect_opt_act_3: "~νει~ηφοι", "~νει~ηφοιτην", "~νει~ηφοιεν";
    perfect_imp_act_2: _, "~νει~ηφετον", "~νει~ηφετε";
    perfect_imp_act_3: "~νει~ηφετω", "~νει~ηφετων", "~νει~ηφοντων";
    perfect_imp_mid_2: "~νει~ηψο", _, _;
    perfect_imp_mid_3: "~νει~ηφθω", "~νει~ηφθων", _;
    infinitive_pres: "~~λᾰμβᾰνειν", "~~λᾰμβᾰνεσθαι", _;
    infinitive_fut: _, "~~ληψεσθαι", "~~ληφθησεσθαι";
    infinitive_aor: "~~λᾰβεῑν", "~~λᾰβεσθαι", "~~ληφθηναι";
    infinitive_perf: "~νει~ηφεναι", "~νει~ηφθαι", _;
    participle_pres: "~~λᾰμβᾰνων", "~~λᾰμβᾰνομενος", _;
    participle_fut: _, "~~ληψομενος", "~~ληφθησομενος";
    participle_aor: "~~λᾰβων", "~~λᾰβομενος", "~~ληφθεις";
    participle_perf: "~νει~ηφως", "~νει~ημμενος", _;
    ),
    // Ἡδῠνω: 6 words (βᾰρῠνω, βᾰθῠνω, ἡδῠνω, πλᾰτῠνω, βρᾰδῠνω, βρᾰχῠνω). Like Πορσῡνω, except in 92 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἡδῠνω", 6, "~ῠνω", like Πορσῡνω as PERFECT_AS_FUTURE;
    stem imperfect: "~ῠν- like ῥεω";
    stem aorist_ind: "~ῡν- like ἐχω";
    ind_act_1: "~ῠνω", _, "~ῠνομεν";
    ind_act_2: "~ῠνεις", "~ῠνετον", "~ῠνετε";
    ind_act_3: "~ῠνει", _, "~ῠνουσῐ";
    ind_mid_1: "~ῠνομαι", _, "~ῠνομεθᾰ";
    ind_mid_2: "~ῠνῃ", "~ῠνεσθον", "~ῠνεσθε";
    ind_mid_3: "~ῠνεται", _, "~ῠνονται";
    subj_act_1: "~ῠνω", _, "~ῠνωμεν";
    subj_act_2: "~ῠνῃς", "~ῠνητον", "~ῠνητε";
    subj_act_3: "~ῠνῃ", _, "~ῠνωσῐ";
    subj_mid_1: "~ῠνωμαι", _, "~ῠνωμεθᾰ";
    subj_mid_2: _, "~ῠνησθον", "~ῠνησθε";
    subj_mid_3: "~ῠνηται", _, "~ῠνωνται";
    opt_act_1: "~ῠνοιμῐ", _, "~ῠνοιμεν";
    opt_act_2: "~ῠνοις", "~ῠνοιτον", "~ῠνοιτε";
    opt_act_3: "~ῠνοι", "~ῠνοιτην", "~ῠνοιεν";
    opt_mid_1: "~ῠνοιμην", _, "~ῠνοιμεθᾰ";
    opt_mid_2: "~ῠνοιο", "~ῠνοισθον", "~ῠνοισθε";
    opt_mid_3: "~ῠνοιτο", "~ῠνοισθην", "~ῠνοιντο";
    imp_act_2: "~ῠνε", _, _;
    imp_act_3: "~ῠνετω", "~ῠνετων", "~ῠνοντων";
    imp_mid_2: "~ῠνου", _, _;
    imp_mid_3: "~ῠνεσθω", "~ῠνεσθων", _;
    future_ind_act_3: _, _, "~ῠνοῡσῐ";
    future_ind_mid_2: "~ῠνεῑ", _, _;
    future_opt_act_1: "~ῠνοιην", _, "~ῠνοῑμεν";
    future_opt_act_2: "~ῠνοιης", "~ῠνοῑτον", "~ῠνοῑτε";
    future_opt_act_3: "~ῠνοιη", "~ῠνοιτην", "~ῠνοῑεν";
    perfect_ind_mid_1: "%ε~ῠσμαι", _, "%ε~ῠσμεθᾰ";
    perfect_ind_mid_2: "%ε~ῠνσαι", "%ε~ῠνθον", "%ε~ῠνθε";
    perfect_ind_mid_3: "%ε~ῠνται", _, "%ε~ῠνᾰται";
    perfect_imp_mid_2: "%ε~ῠνσο", _, _;
    perfect_imp_mid_3: "%ε~ῠνθω", "%ε~ῠνθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_pres: "~ῠνειν", "~ῠνεσθαι", _;
    infinitive_perf: derived, _, _;
    participle_pres: "~ῠνων", "~ῠνομενος", _;
    participle_perf: derived, _, _;
    ),
    // Περιειμι: 6 words (πᾰρειμι, ἐπειμι, μετειμι, περιειμι, ἐνειμι, σῠνειμῐ). Like Δεῑ, except in 46 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("περιειμι", 6, "~ειμι", like Δεῑ as MID_AS_PASS;
    ind_act_1: "~ειμι", _, "~εσμεν";
    ind_act_2: "~εῑ", "~εστον", "~εστε";
    ind_act_3: "~εστι", _, "~εισι";
    subj_act_3: _, _, "~ωσι";
    opt_act_1: "~ειην", _, "~ειημεν";
    opt_act_2: "~ειης", "~ειητον", "~ειητε";
    opt_act_3: "~ειη", "~ειητην", "~ειησαν";
    imp_act_2: "~ισθι", _, _;
    imp_act_3: "~εστω", "~εστων", "~εστων";
    imperfect_act_1: "~η", _, "~ημεν";
    imperfect_act_2: "~ησθα", "~ηστον", "~ητε";
    imperfect_act_3: "~ην", "~ηστην", "~ησαν";
    future_ind_mid_1: "~εσομαι", _, "~εσομεθᾰ";
    future_ind_mid_2: "~εσῃ", "~εσεσθον", "~εσεσθε";
    future_ind_mid_3: "~εσται", _, "~εσονται";
    future_opt_mid_1: "~εσοιμην", _, "~εσοιμεθᾰ";
    future_opt_mid_2: "~εσοιο", "~εσοισθον", "~εσοισθε";
    future_opt_mid_3: "~εσοιτο", "~εσοισθην", "~εσοιντο";
    infinitive_pres: "~εῑναι", _, _;
    infinitive_fut: _, "~εσεσθαι", _;
    participle_fut: _, "~εσομενος", _;
    ),
    // Πρεσβευω: 6 words (ἀκουω, κελευω, πρεσβευω, μᾰθητευω, λαμπᾰδευω, λαυω). Like Παυω, except in 17 cells: imp_act_2 sg, future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, ….
    conjugation!("πρεσβευω", 6, "~υω", like Παυω;
    imp_act_2: "~υε", _, _;
    future_ind_pass_2: "~υσθησῃ|~υσθησει", _, _;
    aorist_subj_pass_3: _, _, "~υσθωσῐ|~υσθωσῐν";
    aorist_opt_pass_1: _, _, "~υσθεῑμεν|~υσθειημεν";
    aorist_opt_pass_2: _, "~υσθεῑτον|~υσθειητον", "~υσθεῑτε|~υσθειητε";
    aorist_opt_pass_3: _, "~υσθειτην|~υσθειητην", "~υσθεῑεν|~υσθειησᾰν";
    aorist_imp_act_2: "~υσον", _, _;
    aorist_imp_mid_2: "~υσαι", _, _;
    perfect_ind_act_3: "%ε~υκε|%ε~υκεν", _, "%ε~υκᾱσῐ|%ε~υκᾱσῐν";
    perfect_subj_act_3: _, _, "%ε~υκωσῐ|%ε~υκωσῐν";
    perfect_opt_act_1: "%ε~υκοιμῐ|%ε~υκοιην", _, _;
    perfect_opt_act_2: "%ε~υκοις|%ε~υκοιης", _, _;
    perfect_opt_act_3: "%ε~υκοι|%ε~υκοιη", _, _;
    perfect_imp_act_2: "%ε~υκε", _, _;
    ),
    // Ζευγνῡμῐ: 6 words (ζευγνῡμῐ, ἀνοιγνῡμῐ, ὀμοργνῡμῐ, προσμειγνῡμῐ, μειγνῡμῐ, κᾰτᾰμῑγνῡμῐ).
    conjugation!("ζευγνῡμῐ", 6, "~~~γνῡμῐ";
    stem aorist_ind: "~ε~~ξ- like ἐλπω";
    ind_act_1: "~~~γνῡμῐ", _, "~~~γνῠμεν";
    ind_act_2: "~~~γνῡς", "~~~γνῠτον", "~~~γνῠτε";
    ind_act_3: "~~~γνῡσῐ", _, "~~~γνῠᾱσῐ";
    ind_mid_1: "~~~γνῠμαι", _, "~~~γνῠμεθᾰ";
    ind_mid_2: "~~~γνῠσαι", "~~~γνῠσθον", "~~~γνῠσθε";
    ind_mid_3: "~~~γνῠται", _, "~~~γνῠνται";
    subj_act_1: "~~~γνῠω", _, "~~~γνῠωμεν";
    subj_act_2: "~~~γνῠῃς", "~~~γνῠητον", "~~~γνῠητε";
    subj_act_3: "~~~γνῠῃ", _, "~~~γνῠωσῐ";
    subj_mid_1: "~~~γνῠωμαι", _, "~~~γνῠωμεθᾰ";
    subj_mid_2: _, "~~~γνῠησθον", "~~~γνῠησθε";
    subj_mid_3: "~~~γνῠηται", _, "~~~γνῠωνται";
    opt_act_1: "~~~γνῠοιμῐ", _, "~~~γνῠοιμεν";
    opt_act_2: "~~~γνῠοις", "~~~γνῠοιτον", "~~~γνῠοιτε";
    opt_act_3: "~~~γνῠοι", "~~~γνῠοιτην", "~~~γνῠοιεν";
    opt_mid_1: "~~~γνῠοιμην", _, "~~~γνῠοιμεθᾰ";
    opt_mid_2: "~~~γνῠοιο", "~~~γνῠοισθον", "~~~γνῠοισθε";
    opt_mid_3: "~~~γνῠοιτο", "~~~γνῠοισθην", "~~~γνῠοιντο";
    imp_act_2: "~~~γνῡ", _, _;
    imp_act_3: "~~~γνῠτω", "~~~γνῠτων", "~~~γνῠντων";
    imp_mid_2: "~~~γνῠσο", _, _;
    imp_mid_3: "~~~γνῠσθω", "~~~γνῠσθων", _;
    imperfect_act_1: "~ε~~γνῡν", _, "~ε~~γνῠμεν";
    imperfect_act_2: "~ε~~γνῡς", "~ε~~γνῠτον", "~ε~~γνῠτε";
    imperfect_act_3: "~ε~~γνῡ", "~ε~~γνῠτην", "~ε~~γνῠσᾰν";
    imperfect_mid_1: "~ε~~γνῠμην", _, "~ε~~γνῠμεθᾰ";
    imperfect_mid_2: "~ε~~γνῠσο", "~ε~~γνῠσθον", "~ε~~γνῠσθε";
    imperfect_mid_3: "~ε~~γνῠτο", "~ε~~γνῠσθην", "~ε~~γνῠντο";
    future_ind_act_1: "~~~ξω", _, "~~~ξομεν";
    future_ind_act_2: "~~~ξεις", "~~~ξετον", "~~~ξετε";
    future_ind_act_3: "~~~ξει", _, "~~~ξουσῐ";
    future_ind_mid_1: "~~~ξομαι", _, "~~~ξομεθᾰ";
    future_ind_mid_2: "~~~ξῃ", "~~~ξεσθον", "~~~ξεσθε";
    future_ind_mid_3: "~~~ξεται", _, "~~~ξονται";
    future_opt_act_1: "~~~ξοιμῐ", _, "~~~ξοιμεν";
    future_opt_act_2: "~~~ξοις", "~~~ξοιτον", "~~~ξοιτε";
    future_opt_act_3: "~~~ξοι", "~~~ξοιτην", "~~~ξοιεν";
    future_opt_mid_1: "~~~ξοιμην", _, "~~~ξοιμεθᾰ";
    future_opt_mid_2: "~~~ξοιο", "~~~ξοισθον", "~~~ξοισθε";
    future_opt_mid_3: "~~~ξοιτο", "~~~ξοισθην", "~~~ξοιντο";
    aorist_subj_act_1: "~ῐ~~ξω", _, "~ῐ~~ξωμεν";
    aorist_subj_act_2: "~ῐ~~ξῃς", "~ῐ~~ξητον", "~ῐ~~ξητε";
    aorist_subj_act_3: "~ῐ~~ξῃ", _, "~ῐ~~ξωσῐ";
    aorist_subj_mid_1: "~ῐ~~ξωμαι", _, "~ῐ~~ξωμεθᾰ";
    aorist_subj_mid_2: _, "~ῐ~~ξησθον", "~ῐ~~ξησθε";
    aorist_subj_mid_3: "~ῐ~~ξηται", _, "~ῐ~~ξωνται";
    aorist_opt_act_1: "~ῐ~~ξαιμῐ", _, "~ῐ~~ξαιμεν";
    aorist_opt_act_2: "~ῐ~~ξειᾰς", "~ῐ~~ξαιτον", "~ῐ~~ξαιτε";
    aorist_opt_act_3: "~ῐ~~ξειε", "~ῐ~~ξαιτην", "~ῐ~~ξειᾰν";
    aorist_opt_mid_1: "~ῐ~~ξαιμην", _, "~ῐ~~ξαιμεθᾰ";
    aorist_opt_mid_2: "~ῐ~~ξαιο", "~ῐ~~ξαισθον", "~ῐ~~ξαισθε";
    aorist_opt_mid_3: "~ῐ~~ξαιτο", "~ῐ~~ξαισθην", "~ῐ~~ξαιντο";
    aorist_imp_act_2: "~ῐ~~ξον", "~ῐ~~ξᾰτον", "~ῐ~~ξᾰτε";
    aorist_imp_act_3: "~ῐ~~ξᾰτω", "~ῐ~~ξᾰτων", "~ῐ~~ξᾰντων";
    aorist_imp_mid_2: "~ῐ~~ξαι", "~ῐ~~ξᾰσθον", "~ῐ~~ξᾰσθε";
    aorist_imp_mid_3: "~ῐ~~ξᾰσθω", "~ῐ~~ξᾰσθων", _;
    infinitive_pres: "~~~γνῠναι", "~~~γνῠσθαι", _;
    infinitive_fut: "~~~ξειν", "~~~ξεσθαι", _;
    infinitive_aor: "~ῐ~ῑξα~", "~ῐ~~ξᾰσθαι", _;
    participle_pres: "~~~γνῡς", "~~~γνῠμενος", _;
    participle_fut: "~~~ξων", "~~~ξομενος", _;
    participle_aor: "~ῐ~~ξᾱς", "~ῐ~~ξᾰμενος", _;
    ),
    // Ἡκω: 5 words (ἡκω, πᾰρηκω, προσηκω, τεθνηκω, κᾰθηκω). Like Δῐωκω, except in 81 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, ….
    conjugation!("ἡκω", 5, "~κω", like Δῐωκω as PERFECT_AS_FUTURE;
    stem aorist_ind: "~ξ- like εὐλογεω";
    stem pluperfect_act: "~κε- like ἀνῐστημῐ";
    ind_act_3: _, _, "~κουσῐ|~κουσῐν";
    ind_mid_2: "~κῃ|~κει", _, _;
    subj_act_3: _, _, "~κωσῐ|~κωσῐν";
    imperfect_act_1: "~κον", _, "~κομεν";
    imperfect_act_2: "~κες", "~κετον", "~κετε";
    imperfect_act_3: "~κε|~κεν", "~κετην", "~κον";
    imperfect_mid_1: "~κομην", _, "~κομεθᾰ";
    imperfect_mid_2: "~κου", "~κεσθον", "~κεσθε";
    imperfect_mid_3: "~κετο", "~κεσθην", "~κοντο";
    future_ind_act_3: _, _, "~ξουσῐ|~ξουσῐν";
    future_ind_mid_2: "~ξῃ|~ξει", _, _;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_subj_act_3: _, _, "~ξωσῐ|~ξωσῐν";
    aorist_opt_act_2: "~ξειᾰς|~ξαις", _, _;
    aorist_opt_act_3: "~ξειε|~ξειεν|~ξαι", _, "~ξειᾰν|~ξαιεν";
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: "~κᾰς", "~κᾰτον", "~κᾰτε";
    perfect_ind_act_3: "~κε", _, "~κᾱσῐ";
    perfect_subj_act_1: "~κω", _, "~κωμεν";
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ";
    perfect_opt_act_1: "~κοιμῐ", _, "~κοιμεν";
    perfect_opt_act_2: "~κοις", "~κοιτον", "~κοιτε";
    perfect_opt_act_3: "~κοι", "~κοιτην", "~κοιεν";
    perfect_imp_act_2: _, "~κετον", "~κετε";
    perfect_imp_act_3: "~κετω", "~κετων", "~κοντων";
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_fut: _, _, derived;
    infinitive_perf: "~κεναι", derived, _;
    participle_fut: _, _, derived;
    participle_perf: "~κως", derived, _;
    ),
    // Ἱζω: 5 words (ἑλληνῐζω, ἀφῠπνῐζω, ἡσῠχᾰζω, ῥωμᾱϊζω, ἐξῠβρῐζω). Like Λογῐζομαι, except in 64 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, ….
    conjugation!("ἱζω", 5, "~ζω", like Λογῐζομαι;
    stem aorist_ind: "~σ- like ἐλπω";
    stem aorist_ind_pass: "~σθη- like ἐλπω";
    stem pluperfect_act: "~κε- like ἀνῐστημῐ";
    stem pluperfect: "~- like ὑποκοριζομαι";
    ind_act_3: _, _, "~ζουσῐ|~ζουσῐν";
    ind_mid_2: "~ζῃ|~ζει", _, _;
    subj_act_3: _, _, "~ζωσῐ|~ζωσῐν";
    imperfect_act_1: "~ζον", _, "~ζομεν";
    imperfect_act_2: "~ζες", "~ζετον", "~ζετε";
    imperfect_act_3: "~ζε|~ζεν", "~ζετην", "~ζον";
    imperfect_mid_1: "~ζομην", _, "~ζομεθᾰ";
    imperfect_mid_2: "~ζου", "~ζεσθον", "~ζεσθε";
    imperfect_mid_3: "~ζετο", "~ζεσθην", "~ζοντο";
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", _, "~σειᾰν|~σαιεν";
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: "~κᾰς", "~κᾰτον", "~κᾰτε";
    perfect_ind_act_3: "~κε", _, "~κᾱσῐ";
    perfect_ind_mid_1: "~σμαι", _, "~σμεθᾰ";
    perfect_ind_mid_2: "~σαι", "~σθον", "~σθε";
    perfect_ind_mid_3: "~σται", _, "~δᾰται";
    perfect_subj_act_1: "~κω", _, "~κωμεν";
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ";
    perfect_opt_act_1: "~κοιμῐ", _, "~κοιμεν";
    perfect_opt_act_2: "~κοις", "~κοιτον", "~κοιτε";
    perfect_opt_act_3: "~κοι", "~κοιτην", "~κοιεν";
    perfect_imp_act_2: _, "~κετον", "~κετε";
    perfect_imp_act_3: "~κετω", "~κετων", "~κοντων";
    perfect_imp_mid_2: "~σο", _, _;
    perfect_imp_mid_3: "~σθω", "~σθων", _;
    infinitive_perf: "~κεναι", "~σθαι", _;
    participle_perf: "~κως", "~σμενος", _;
    ),
    // Φρᾰζω: 5 words (φρᾰζω, θαυμᾰζω, θοᾰζω, θωᾰζω, χειμᾰζω). Like Πειρᾰζω, except in 8 cells: pluperfect_mid_1 sg, pluperfect_mid_1 pl, pluperfect_mid_2 sg, pluperfect_mid_2 du, pluperfect_mid_2 pl, pluperfect_mid_3 sg, ….
    conjugation!("φρᾰζω", 5, "~ζω", like Πειρᾰζω;
    stem pluperfect_act: "~κ- like χρημᾰτῐζω";
    pluperfect_mid_1: "ἐ%ε~σμην", _, "ἐ%ε~σμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~σο", "ἐ%ε~σθον", "ἐ%ε~σθε";
    pluperfect_mid_3: "ἐ%ε~στο", "ἐ%ε~σθην", "ἐ%ε~δᾰτο";
    ),
    // Βαινω: 5 words (βαινω, χαινω, ἐπεκβαινω, εἰσᾰνᾰβαινω, ἐπαλθαινω). Like Νῑκαω, except in 155 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("βαινω", 5, "~αινω", like Νῑκαω;
    stem imperfect: "~αιν- like ῥεω";
    stem aorist_ind_pass: "~ᾰθη- like ἐχω";
    stem pluperfect_act: "~εβηκε- like κᾰλεω";
    ind_act_1: "~αινω", _, "~αινομεν";
    ind_act_2: "~αινεις", "~αινετον", "~αινετε";
    ind_act_3: "~αινει", _, "~αινουσῐ";
    ind_mid_1: "~αινομαι", _, "~αινομεθᾰ";
    ind_mid_2: "~αινῃ", "~αινεσθον", "~αινεσθε";
    ind_mid_3: "~αινεται", _, "~αινονται";
    subj_act_1: "~αινω", _, "~αινωμεν";
    subj_act_2: "~αινῃς", "~αινητον", "~αινητε";
    subj_act_3: "~αινῃ", _, "~αινωσῐ";
    subj_mid_1: "~αινωμαι", _, "~αινωμεθᾰ";
    subj_mid_2: _, "~αινησθον", "~αινησθε";
    subj_mid_3: "~αινηται", _, "~αινωνται";
    opt_act_1: "~αινοιμῐ", _, "~αινοιμεν";
    opt_act_2: "~αινοις", "~αινοιτον", "~αινοιτε";
    opt_act_3: "~αινοι", "~αινοιτην", "~αινοιεν";
    opt_mid_1: "~αινοιμην", _, "~αινοιμεθᾰ";
    opt_mid_2: "~αινοιο", "~αινοισθον", "~αινοισθε";
    opt_mid_3: "~αινοιτο", "~αινοισθην", "~αινοιντο";
    imp_act_2: "~αῑνε", _, _;
    imp_act_3: "~αινετω", "~αινετων", "~αινοντων";
    imp_mid_2: "~αινου", _, _;
    imp_mid_3: "~αινεσθω", "~αινεσθων", _;
    aorist_ind_act_1: "ἐ~ην", _, "ἐ~ημεν";
    aorist_ind_act_2: "ἐ~ης", "ἐ~ητον", "ἐ~ητε";
    aorist_ind_act_3: "ἐ~η", "ἐ~ητην", _;
    aorist_subj_act_1: "~ω", _, "~ωμεν";
    aorist_subj_act_2: "~ῃς", "~ητον", "~ητε";
    aorist_subj_act_3: "~ῃ", _, "~ωσῐ";
    aorist_subj_mid_2: "~ησῃ", _, _;
    aorist_subj_pass_1: "~ᾰθω", _, "~ᾰθωμεν";
    aorist_subj_pass_2: "~ᾰθῃς", "~ᾰθητον", "~ᾰθητε";
    aorist_subj_pass_3: "~ᾰθῃ", _, "~ᾰθωσῐ";
    aorist_opt_act_1: "~αιην", _, "~αῑμεν";
    aorist_opt_act_2: "~αιης", "~αῑτον", "~αῑτε";
    aorist_opt_act_3: "~αιη", "~αιτην", "~αῑεν";
    aorist_opt_pass_1: "~ᾰθειην", _, "~ᾰθεῑμεν";
    aorist_opt_pass_2: "~ᾰθειης", "~ᾰθεῑτον", "~ᾰθεῑτε";
    aorist_opt_pass_3: "~ᾰθειη", "~ᾰθειτην", "~ᾰθεῑεν";
    aorist_imp_act_2: "~ηθῐ", "~ητον", "~ητε";
    aorist_imp_act_3: "~ητω", "~ητων", "~ᾰντων";
    aorist_imp_pass_2: "~ᾰθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰθητω", "~ᾰθητων", "~ᾰθεντων";
    perfect_ind_act_1: "~εβηκᾰ", _, "~εβηκᾰμεν";
    perfect_ind_act_2: "~εβηκᾰς", "~εβηκᾰτον", "~εβηκᾰτε";
    perfect_ind_act_3: "~εβηκε", _, "~εβηκᾱσῐ";
    perfect_ind_mid_1: "~εβᾰμαι", _, "~εβᾰμεθᾰ";
    perfect_ind_mid_2: "~εβᾰσαι", "~εβᾰσθον", "~εβᾰσθε";
    perfect_ind_mid_3: "~εβᾰται", _, "~εβᾰνται";
    perfect_subj_act_1: "~εβηκω", _, "~εβηκωμεν";
    perfect_subj_act_2: "~εβηκῃς", "~εβηκητον", "~εβηκητε";
    perfect_subj_act_3: "~εβηκῃ", _, "~εβηκωσῐ";
    perfect_opt_act_1: "~εβηκοιμῐ", _, "~εβηκοιμεν";
    perfect_opt_act_2: "~εβηκοις", "~εβηκοιτον", "~εβηκοιτε";
    perfect_opt_act_3: "~εβηκοι", "~εβηκοιτην", "~εβηκοιεν";
    perfect_imp_act_2: _, "~εβηκετον", "~εβηκετε";
    perfect_imp_act_3: "~εβηκετω", "~εβηκετων", "~εβηκοντων";
    perfect_imp_mid_2: "~εβᾰσο", _, _;
    perfect_imp_mid_3: "~εβᾰσθω", "~εβᾰσθων", _;
    pluperfect_mid_1: "ἐ~εβᾰμην", _, "ἐ~εβᾰμεθᾰ";
    pluperfect_mid_2: "ἐ~εβᾰσο", "ἐ~εβᾰσθον", "ἐ~εβᾰσθε";
    pluperfect_mid_3: "ἐ~εβᾰτο", "ἐ~εβᾰσθην", "ἐ~εβᾰντο";
    infinitive_pres: "~αινειν", "~αινεσθαι", _;
    infinitive_aor: "~ηναι", _, "~ᾰθηναι";
    infinitive_perf: "~εβηκεναι", "~εβᾰσθαι", _;
    participle_pres: "~αινων", "~αινομενος", _;
    participle_aor: "~ᾱς", _, "~ᾰθεις";
    participle_perf: "~εβηκως", "~εβᾰμενος", _;
    ),
    // Πρεπω: 5 words (τρεπω, πρεπω, πᾰρεκτρεπω, λεπω, ρρεπω). Like Γῡροω, except in 219 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("πρεπω", 5, "~επω", like Γῡροω;
    stem imperfect: "~επ- like ῥεω";
    stem aorist_ind: "~εψ- like ἐχω";
    stem aorist_ind_pass: "~εφθη- like ἐχω";
    stem pluperfect_act: "~οφε- like ἐχω";
    ind_act_1: "~επω", _, "~επομεν";
    ind_act_2: "~επεις", "~επετον", "~επετε";
    ind_act_3: "~επει", _, "~επουσῐ";
    ind_mid_1: "~επομαι", _, "~επομεθᾰ";
    ind_mid_2: "~επῃ", "~επεσθον", "~επεσθε";
    ind_mid_3: "~επεται", _, "~επονται";
    subj_act_1: "~επω", _, "~επωμεν";
    subj_act_2: "~επῃς", "~επητον", "~επητε";
    subj_act_3: "~επῃ", _, "~επωσῐ";
    subj_mid_1: "~επωμαι", _, "~επωμεθᾰ";
    subj_mid_2: _, "~επησθον", "~επησθε";
    subj_mid_3: "~επηται", _, "~επωνται";
    opt_act_1: "~εποιμῐ", _, "~εποιμεν";
    opt_act_2: "~εποις", "~εποιτον", "~εποιτε";
    opt_act_3: "~εποι", "~εποιτην", "~εποιεν";
    opt_mid_1: "~εποιμην", _, "~εποιμεθᾰ";
    opt_mid_2: "~εποιο", "~εποισθον", "~εποισθε";
    opt_mid_3: "~εποιτο", "~εποισθην", "~εποιντο";
    imp_act_2: "~επε", _, _;
    imp_act_3: "~επετω", "~επετων", "~εποντων";
    imp_mid_2: "~επου", _, _;
    imp_mid_3: "~επεσθω", "~επεσθων", _;
    future_ind_act_1: "~εψω", _, "~εψομεν";
    future_ind_act_2: "~εψεις", "~εψετον", "~εψετε";
    future_ind_act_3: "~εψει", _, "~εψουσῐ";
    future_ind_mid_1: "~εψομαι", _, "~εψομεθᾰ";
    future_ind_mid_2: "~εψῃ", "~εψεσθον", "~εψεσθε";
    future_ind_mid_3: "~εψεται", _, "~εψονται";
    future_ind_pass_1: "~απησομαι", _, "~απησομεθᾰ";
    future_ind_pass_2: "~απησῃ", "~απησεσθον", "~απησεσθε";
    future_ind_pass_3: "~απησεται", _, "~απησονται";
    future_opt_act_1: "~εψοιμῐ", _, "~εψοιμεν";
    future_opt_act_2: "~εψοις", "~εψοιτον", "~εψοιτε";
    future_opt_act_3: "~εψοι", "~εψοιτην", "~εψοιεν";
    future_opt_mid_1: "~εψοιμην", _, "~εψοιμεθᾰ";
    future_opt_mid_2: "~εψοιο", "~εψοισθον", "~εψοισθε";
    future_opt_mid_3: "~εψοιτο", "~εψοισθην", "~εψοιντο";
    future_opt_pass_1: "~απησοιμην", _, "~απησοιμεθᾰ";
    future_opt_pass_2: "~απησοιο", "~απησοισθον", "~απησοισθε";
    future_opt_pass_3: "~απησοιτο", "~απησοισθην", "~απησοιντο";
    aorist_subj_act_1: "~εψω", _, "~εψωμεν";
    aorist_subj_act_2: "~εψῃς", "~εψητον", "~εψητε";
    aorist_subj_act_3: "~εψῃ", _, "~εψωσῐ";
    aorist_subj_mid_1: "~εψωμαι", _, "~εψωμεθᾰ";
    aorist_subj_mid_2: _, "~εψησθον", "~εψησθε";
    aorist_subj_mid_3: "~εψηται", _, "~εψωνται";
    aorist_subj_pass_1: "~εφθω", _, "~εφθωμεν";
    aorist_subj_pass_2: "~εφθῃς", "~εφθητον", "~εφθητε";
    aorist_subj_pass_3: "~εφθῃ", _, "~εφθωσῐ";
    aorist_opt_act_1: "~εψαιμῐ", _, "~εψαιμεν";
    aorist_opt_act_2: "~εψειᾰς", "~εψαιτον", "~εψαιτε";
    aorist_opt_act_3: "~εψειε", "~εψαιτην", "~εψειᾰν";
    aorist_opt_mid_1: "~εψαιμην", _, "~εψαιμεθᾰ";
    aorist_opt_mid_2: "~εψαιο", "~εψαισθον", "~εψαισθε";
    aorist_opt_mid_3: "~εψαιτο", "~εψαισθην", "~εψαιντο";
    aorist_opt_pass_1: "~εφθειην", _, "~εφθεῑμεν";
    aorist_opt_pass_2: "~εφθειης", "~εφθεῑτον", "~εφθεῑτε";
    aorist_opt_pass_3: "~εφθειη", "~εφθειτην", "~εφθεῑεν";
    aorist_imp_act_2: "~εψον", "~εψᾰτον", "~εψᾰτε";
    aorist_imp_act_3: "~εψᾰτω", "~εψᾰτων", "~εψᾰντων";
    aorist_imp_mid_2: "~εψαι", "~εψᾰσθον", "~εψᾰσθε";
    aorist_imp_mid_3: "~εψᾰσθω", "~εψᾰσθων", _;
    aorist_imp_pass_2: "~εφθητῐ", _, _;
    aorist_imp_pass_3: "~εφθητω", "~εφθητων", "~εφθεντων";
    perfect_ind_act_1: "%ε~οφᾰ", _, "%ε~οφᾰμεν";
    perfect_ind_act_2: "%ε~οφᾰς", "%ε~οφᾰτον", "%ε~οφᾰτε";
    perfect_ind_act_3: "%ε~οφε", _, "%ε~οφᾱσῐ";
    perfect_ind_mid_1: "%ε~αμμαι", _, "%ε~αμμεθᾰ";
    perfect_ind_mid_2: "%ε~αψαι", "%ε~αφθον", "%ε~αφθε";
    perfect_ind_mid_3: "%ε~απται", _, "%ε~αφᾰται";
    perfect_subj_act_1: "%ε~οφω", _, "%ε~οφωμεν";
    perfect_subj_act_2: "%ε~οφῃς", "%ε~οφητον", "%ε~οφητε";
    perfect_subj_act_3: "%ε~οφῃ", _, "%ε~οφωσῐ";
    perfect_opt_act_1: "%ε~οφοιμῐ", _, "%ε~οφοιμεν";
    perfect_opt_act_2: "%ε~οφοις", "%ε~οφοιτον", "%ε~οφοιτε";
    perfect_opt_act_3: "%ε~οφοι", "%ε~οφοιτην", "%ε~οφοιεν";
    perfect_imp_act_2: _, "%ε~οφετον", "%ε~οφετε";
    perfect_imp_act_3: "%ε~οφετω", "%ε~οφετων", "%ε~οφοντων";
    perfect_imp_mid_2: "%ε~αψο", _, _;
    perfect_imp_mid_3: "%ε~αφθω", "%ε~αφθων", _;
    pluperfect_mid_1: "ἐ%ε~αμμην", _, "ἐ%ε~αμμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~αψο", "ἐ%ε~αφθον", "ἐ%ε~αφθε";
    pluperfect_mid_3: "ἐ%ε~απτο", "ἐ%ε~αφθην", "ἐ%ε~αφᾰτο";
    infinitive_pres: "~επειν", "~επεσθαι", _;
    infinitive_fut: "~εψειν", "~εψεσθαι", "~απησεσθαι";
    infinitive_aor: "~εψαι", "~εψᾰσθαι", "~εφθηναι";
    infinitive_perf: "%ε~οφεναι", "%ε~ᾱφθαι", _;
    participle_pres: "~επων", "~επομενος", _;
    participle_fut: "~εψων", "~εψομενος", "~απησομενος";
    participle_aor: "~εψᾱς", "~εψᾰμενος", "~εφθεις";
    participle_perf: "%ε~οφως", "%ε~αμμενος", _;
    ),
    // Στερεω: 5 words (στερεω, σῐδηροφορεω, κορεω, σειρεω, γεωμετρεω). Like Δῐκεω, except in 81 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("στερεω", 5, "~εω", like Δῐκεω as FUT_AS_PRES;
    stem aorist_ind: "~ησ- like ἐχω";
    stem pluperfect_act: "~ηκε- like κᾰλεω";
    future_ind_act_1: "~ω", _, "~οῡμεν";
    future_ind_act_2: "~εῑς", "~εῑτον", "~εῑτε";
    future_ind_act_3: "~εῑ", _, "~οῡσῐ";
    future_ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    future_ind_mid_2: "~εῑ", "~εῑσθον", "~εῑσθε";
    future_ind_mid_3: "~εῑται", _, "~οῡνται";
    future_ind_pass_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_pass_2: "~ησῃ", "~ησεσθον", "~ησεσθε";
    future_ind_pass_3: "~ησεται", _, "~ησονται";
    future_opt_act_1: "~οιην", _, "~οῑμεν";
    future_opt_act_2: "~οιης", "~οῑτον", "~οῑτε";
    future_opt_act_3: "~οιη", "~οιτην", "~οῑεν";
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_2: "~οῑο", "~οῑσθον", "~οῑσθε";
    future_opt_mid_3: "~οῑτο", "~οισθην", "~οῑντο";
    future_opt_pass_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_pass_2: "~ησοιο", "~ησοισθον", "~ησοισθε";
    future_opt_pass_3: "~ησοιτο", "~ησοισθην", "~ησοιντο";
    aorist_subj_act_3: _, _, "~ησωσῐ";
    aorist_subj_pass_3: _, _, "~ηθωσῐ";
    aorist_opt_act_2: "~ησειᾰς", _, _;
    aorist_opt_act_3: "~ησειε", _, "~ησειᾰν";
    aorist_opt_pass_1: _, _, "~ηθεῑμεν";
    aorist_opt_pass_2: _, "~ηθεῑτον", "~ηθεῑτε";
    aorist_opt_pass_3: _, "~ηθειτην", "~ηθεῑεν";
    perfect_ind_act_3: "ἐ~ηκε", _, "ἐ~ηκᾱσῐ";
    perfect_subj_act_3: _, _, "ἐ~ηκωσῐ";
    perfect_opt_act_1: "ἐ~ηκοιμῐ", _, _;
    perfect_opt_act_2: "ἐ~ηκοις", _, _;
    perfect_opt_act_3: "ἐ~ηκοι", _, _;
    perfect_imp_act_2: derived, _, _;
    future_perfect_ind_mid_1: "ἐ~ησομαι", _, "ἐ~ησομεθᾰ";
    future_perfect_ind_mid_2: "ἐ~ησῃ", "ἐ~ησεσθον", "ἐ~ησεσθε";
    future_perfect_ind_mid_3: "ἐ~ησεται", _, "ἐ~ησονται";
    future_perfect_opt_mid_1: "ἐ~ησοιμην", _, "ἐ~ησοιμεθᾰ";
    future_perfect_opt_mid_2: "ἐ~ησοιο", "ἐ~ησοισθον", "ἐ~ησοισθε";
    future_perfect_opt_mid_3: "ἐ~ησοιτο", "ἐ~ησοισθην", "ἐ~ησοιντο";
    infinitive_fut: _, _, "~ησεσθαι";
    infinitive_futp: _, "ἐ~ησεσθαι", _;
    participle_fut: _, _, "~ησομενος";
    participle_futp: _, "ἐ~ησομενος", _;
    ),
    // Μᾰρτῠρεω: 5 words (μᾰρτῠρεω, λοιδορεω, μᾰραυγεω, δονεω, κερτομεω). Like Ποιεω, except in 38 cells: aorist_subj_pass_1 sg, aorist_subj_pass_1 pl, aorist_subj_pass_2 sg, aorist_subj_pass_2 du, aorist_subj_pass_2 pl, aorist_subj_pass_3 sg, ….
    conjugation!("μᾰρτῠρεω", 5, "~εω", like Ποιεω;
    stem aorist_ind_pass: "~ηθη- like ἐλπω";
    aorist_subj_pass_1: "ἐ~ηθω", _, "ἐ~ηθωμεν";
    aorist_subj_pass_2: "ἐ~ηθῃς", "ἐ~ηθητον", "ἐ~ηθητε";
    aorist_subj_pass_3: "ἐ~ηθῃ", _, "ἐ~ηθωσῐ|ἐ~ηθωσῐν";
    aorist_opt_pass_1: "ἐ~ηθειην", _, "ἐ~ηθεῑμεν|ἐ~ηθειημεν";
    aorist_opt_pass_2: "ἐ~ηθειης", "ἐ~ηθεῑτον|ἐ~ηθειητον", "ἐ~ηθεῑτε|ἐ~ηθειητε";
    aorist_opt_pass_3: "ἐ~ηθειη", "ἐ~ηθειτην|ἐ~ηθειητην", "ἐ~ηθεῑεν|ἐ~ηθειησᾰν";
    aorist_imp_pass_2: "ἐ~ηθητῐ", _, _;
    aorist_imp_pass_3: "ἐ~ηθητω", "ἐ~ηθητων", "ἐ~ηθεντων";
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_aor: _, _, "ἐ~ηθηναι";
    infinitive_futp: _, derived, _;
    participle_aor: _, _, "ἐ~ηθεις";
    participle_futp: _, derived, _;
    ),
    // Ἐλπῐζω: 5 words (ἐγγῐζω, ἐλπῐζω, ἐρεθῐζω, ἐνᾰρῐζω, ἐρῐζω). Like Γῡροω, except in 228 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐλπῐζω", 5, "ἐ~ζω", like Γῡροω;
    stem aorist_ind: "~σ- like ἐλπῐζω";
    stem aorist_ind_pass: "~σθη- like ἀγορευω";
    stem pluperfect_act: "~κ- like ἐλπῐζω";
    stem pluperfect: "~- like ἀγωνιζομαι";
    ind_act_1: "ἐ~ζω", _, "ἐ~ζομεν";
    ind_act_2: "ἐ~ζεις", "ἐ~ζετον", "ἐ~ζετε";
    ind_act_3: "ἐ~ζει", _, "ἐ~ζουσῐ|ἐ~ζουσῐν";
    ind_mid_1: "ἐ~ζομαι", _, "ἐ~ζομεθᾰ";
    ind_mid_2: "ἐ~ζῃ|ἐ~ζει", "ἐ~ζεσθον", "ἐ~ζεσθε";
    ind_mid_3: "ἐ~ζεται", _, "ἐ~ζονται";
    subj_act_1: "ἐ~ζω", _, "ἐ~ζωμεν";
    subj_act_2: "ἐ~ζῃς", "ἐ~ζητον", "ἐ~ζητε";
    subj_act_3: "ἐ~ζῃ", _, "ἐ~ζωσῐ|ἐ~ζωσῐν";
    subj_mid_1: "ἐ~ζωμαι", _, "ἐ~ζωμεθᾰ";
    subj_mid_2: _, "ἐ~ζησθον", "ἐ~ζησθε";
    subj_mid_3: "ἐ~ζηται", _, "ἐ~ζωνται";
    opt_act_1: "ἐ~ζοιμῐ", _, "ἐ~ζοιμεν";
    opt_act_2: "ἐ~ζοις", "ἐ~ζοιτον", "ἐ~ζοιτε";
    opt_act_3: "ἐ~ζοι", "ἐ~ζοιτην", "ἐ~ζοιεν";
    opt_mid_1: "ἐ~ζοιμην", _, "ἐ~ζοιμεθᾰ";
    opt_mid_2: "ἐ~ζοιο", "ἐ~ζοισθον", "ἐ~ζοισθε";
    opt_mid_3: "ἐ~ζοιτο", "ἐ~ζοισθην", "ἐ~ζοιντο";
    imp_act_2: "ἐ~ζε", _, _;
    imp_act_3: "ἐ~ζετω", "ἐ~ζετων", "ἐ~ζοντων";
    imp_mid_2: "ἐ~ζου", _, _;
    imp_mid_3: "ἐ~ζεσθω", "ἐ~ζεσθων", _;
    imperfect_act_1: "ἠ~ζον", _, "ἠ~ζομεν";
    imperfect_act_2: "ἠ~ζες", "ἠ~ζετον", "ἠ~ζετε";
    imperfect_act_3: "ἠ~ζε|ἠ~ζεν", "ἠ~ζετην", "ἠ~ζον";
    imperfect_mid_1: "ἠ~ζομην", _, "ἠ~ζομεθᾰ";
    imperfect_mid_2: "ἠ~ζου", "ἠ~ζεσθον", "ἠ~ζεσθε";
    imperfect_mid_3: "ἠ~ζετο", "ἠ~ζεσθην", "ἠ~ζοντο";
    future_ind_act_1: "ἐ~ω", _, "ἐ~οῡμεν";
    future_ind_act_2: "ἐ~εῑς", "ἐ~εῑτον", "ἐ~εῑτε";
    future_ind_act_3: "ἐ~εῑ", _, "ἐ~οῡσῐ|ἐ~οῡσῐν";
    future_ind_mid_1: "ἐ~οῡμαι", _, "ἐ~ουμεθᾰ";
    future_ind_mid_2: "ἐ~εῑ|ἐ~ῃ", "ἐ~εῑσθον", "ἐ~εῑσθε";
    future_ind_mid_3: "ἐ~εῑται", _, "ἐ~οῡνται";
    future_ind_pass_1: "ἐ~σθησομαι", _, "ἐ~σθησομεθᾰ";
    future_ind_pass_2: "ἐ~σθησῃ|ἐ~σθησει", "ἐ~σθησεσθον", "ἐ~σθησεσθε";
    future_ind_pass_3: "ἐ~σθησεται", _, "ἐ~σθησονται";
    future_opt_act_1: "ἐ~οιην|ἐ~οῑμῐ", _, "ἐ~οῑμεν|ἐ~οιημεν";
    future_opt_act_2: "ἐ~οιης|ἐ~οῑς", "ἐ~οῑτον|ἐ~οιητον", "ἐ~οῑτε|ἐ~οιητε";
    future_opt_act_3: "ἐ~οιη|ἐ~οῑ", "ἐ~οιτην|ἐ~οιητην", "ἐ~οῑεν|ἐ~οιησᾰν";
    future_opt_mid_1: "ἐ~οιμην", _, "ἐ~οιμεθᾰ";
    future_opt_mid_2: "ἐ~οῑο", "ἐ~οῑσθον", "ἐ~οῑσθε";
    future_opt_mid_3: "ἐ~οῑτο", "ἐ~οισθην", "ἐ~οῑντο";
    future_opt_pass_1: "ἐ~σθησοιμην", _, "ἐ~σθησοιμεθᾰ";
    future_opt_pass_2: "ἐ~σθησοιο", "ἐ~σθησοισθον", "ἐ~σθησοισθε";
    future_opt_pass_3: "ἐ~σθησοιτο", "ἐ~σθησοισθην", "ἐ~σθησοιντο";
    aorist_subj_act_1: "ἐ~σω", _, "ἐ~σωμεν";
    aorist_subj_act_2: "ἐ~σῃς", "ἐ~σητον", "ἐ~σητε";
    aorist_subj_act_3: "ἐ~σῃ", _, "ἐ~σωσῐ|ἐ~σωσῐν";
    aorist_subj_mid_1: "ἐ~σωμαι", _, "ἐ~σωμεθᾰ";
    aorist_subj_mid_2: _, "ἐ~σησθον", "ἐ~σησθε";
    aorist_subj_mid_3: "ἐ~σηται", _, "ἐ~σωνται";
    aorist_subj_pass_1: "ἐ~σθω", _, "ἐ~σθωμεν";
    aorist_subj_pass_2: "ἐ~σθῃς", "ἐ~σθητον", "ἐ~σθητε";
    aorist_subj_pass_3: "ἐ~σθῃ", _, "ἐ~σθωσῐ|ἐ~σθωσῐν";
    aorist_opt_act_1: "ἐ~σαιμῐ", _, "ἐ~σαιμεν";
    aorist_opt_act_2: "ἐ~σειᾰς|ἐ~σαις", "ἐ~σαιτον", "ἐ~σαιτε";
    aorist_opt_act_3: "ἐ~σειε|ἐ~σειεν|ἐ~σαι", "ἐ~σαιτην", "ἐ~σειᾰν|ἐ~σαιεν";
    aorist_opt_mid_1: "ἐ~σαιμην", _, "ἐ~σαιμεθᾰ";
    aorist_opt_mid_2: "ἐ~σαιο", "ἐ~σαισθον", "ἐ~σαισθε";
    aorist_opt_mid_3: "ἐ~σαιτο", "ἐ~σαισθην", "ἐ~σαιντο";
    aorist_opt_pass_1: "ἐ~σθειην", _, "ἐ~σθεῑμεν|ἐ~σθειημεν";
    aorist_opt_pass_2: "ἐ~σθειης", "ἐ~σθεῑτον|ἐ~σθειητον", "ἐ~σθεῑτε|ἐ~σθειητε";
    aorist_opt_pass_3: "ἐ~σθειη", "ἐ~σθειτην|ἐ~σθειητην", "ἐ~σθεῑεν|ἐ~σθειησᾰν";
    aorist_imp_act_2: "ἐ~σον", "ἐ~σᾰτον", "ἐ~σᾰτε";
    aorist_imp_act_3: "ἐ~σᾰτω", "ἐ~σᾰτων", "ἐ~σᾰντων";
    aorist_imp_mid_2: "ἐ~σαι", "ἐ~σᾰσθον", "ἐ~σᾰσθε";
    aorist_imp_mid_3: "ἐ~σᾰσθω", "ἐ~σᾰσθων", _;
    aorist_imp_pass_2: "ἐ~σθητῐ", _, _;
    aorist_imp_pass_3: "ἐ~σθητω", "ἐ~σθητων", "ἐ~σθεντων";
    perfect_ind_act_1: "ἠ~κᾰ", _, "ἠ~κᾰμεν";
    perfect_ind_act_2: "ἠ~κᾰς", "ἠ~κᾰτον", "ἠ~κᾰτε";
    perfect_ind_act_3: "ἠ~κε|ἠ~κεν", _, "ἠ~κᾱσῐ|ἠ~κᾱσῐν";
    perfect_ind_mid_1: "ἠ~σμαι", _, "ἠ~σμεθᾰ";
    perfect_ind_mid_2: "ἠ~σαι", "ἠ~σθον", "ἠ~σθε";
    perfect_ind_mid_3: "ἠ~σται", _, "ἠ~δᾰται";
    perfect_subj_act_1: "ἠ~κω", _, "ἠ~κωμεν";
    perfect_subj_act_2: "ἠ~κῃς", "ἠ~κητον", "ἠ~κητε";
    perfect_subj_act_3: "ἠ~κῃ", _, "ἠ~κωσῐ|ἠ~κωσῐν";
    perfect_opt_act_1: "ἠ~κοιμῐ|ἠ~κοιην", _, "ἠ~κοιμεν";
    perfect_opt_act_2: "ἠ~κοις|ἠ~κοιης", "ἠ~κοιτον", "ἠ~κοιτε";
    perfect_opt_act_3: "ἠ~κοι|ἠ~κοιη", "ἠ~κοιτην", "ἠ~κοιεν";
    perfect_imp_act_2: "ἠ~κε", "ἠ~κετον", "ἠ~κετε";
    perfect_imp_act_3: "ἠ~κετω", "ἠ~κετων", "ἠ~κοντων";
    perfect_imp_mid_2: "ἠ~σο", _, _;
    perfect_imp_mid_3: "ἠ~σθω", "ἠ~σθων", _;
    infinitive_pres: "ἐ~ζειν", "ἐ~ζεσθαι", _;
    infinitive_fut: "ἐ~εῑν", "ἐ~εῑσθαι", "ἐ~σθησεσθαι";
    infinitive_aor: "ἐ~σαι", "ἐ~σᾰσθαι", "ἐ~σθηναι";
    infinitive_perf: "ἠ~κεναι", "ἠ~σθαι", _;
    participle_pres: "ἐ~ζων", "ἐ~ζομενος", _;
    participle_fut: "ἐ~ων", "ἐ~ουμενος", "ἐ~σθησομενος";
    participle_aor: "ἐ~σᾱς", "ἐ~σᾰμενος", "ἐ~σθεις";
    participle_perf: "ἠ~κως", "ἠ~σμενος", _;
    ),
    // Σῠνοικῐζω: 5 words (ἀνοικῐζω, κᾰτοικῐζω, σῠνοικῐζω, ξῠνοικῐζω, ἀποικῐζω). Like Δῐοικεω, except in 218 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("σῠνοικῐζω", 5, "~οι~ζω", like Δῐοικεω as FUTURE_AS_PRESENT;
    stem aorist_ind: "~ῳ~σ- like εὐλογεω";
    stem aorist_ind_pass: "~ῳ~σθη- like ἐλπω";
    stem pluperfect_act: "~ῳ~κ- like εὐλογεω";
    stem pluperfect: "~ῳ~- like ὑποκοριζομαι";
    ind_act_1: "~οι~ζω", _, "~οι~ζομεν";
    ind_act_2: "~οι~ζεις", "~οι~ζετον", "~οι~ζετε";
    ind_act_3: "~οι~ζει", _, "~οι~ζουσῐ|~οι~ζουσῐν";
    ind_mid_1: "~οι~ζομαι", _, "~οι~ζομεθᾰ";
    ind_mid_2: "~οι~ζῃ|~οι~ζει", "~οι~ζεσθον", "~οι~ζεσθε";
    ind_mid_3: "~οι~ζεται", _, "~οι~ζονται";
    subj_act_1: "~οι~ζω", _, "~οι~ζωμεν";
    subj_act_2: "~οι~ζῃς", "~οι~ζητον", "~οι~ζητε";
    subj_act_3: "~οι~ζῃ", _, "~οι~ζωσῐ|~οι~ζωσῐν";
    subj_mid_1: "~οι~ζωμαι", _, "~οι~ζωμεθᾰ";
    subj_mid_2: _, "~οι~ζησθον", "~οι~ζησθε";
    subj_mid_3: "~οι~ζηται", _, "~οι~ζωνται";
    opt_act_1: "~οι~ζοιμῐ", _, "~οι~ζοιμεν";
    opt_act_2: "~οι~ζοις", "~οι~ζοιτον", "~οι~ζοιτε";
    opt_act_3: "~οι~ζοι", "~οι~ζοιτην", "~οι~ζοιεν";
    opt_mid_1: "~οι~ζοιμην", _, "~οι~ζοιμεθᾰ";
    opt_mid_2: "~οι~ζοιο", "~οι~ζοισθον", "~οι~ζοισθε";
    opt_mid_3: "~οι~ζοιτο", "~οι~ζοισθην", "~οι~ζοιντο";
    imp_act_2: "~οι~ζε", _, _;
    imp_act_3: "~οι~ζετω", "~οι~ζετων", "~οι~ζοντων";
    imp_mid_2: "~οι~ζου", _, _;
    imp_mid_3: "~οι~ζεσθω", "~οι~ζεσθων", _;
    imperfect_act_1: "~ῳ~ζον", _, "~ῳ~ζομεν";
    imperfect_act_2: "~ῳ~ζες", "~ῳ~ζετον", "~ῳ~ζετε";
    imperfect_act_3: "~ῳ~ζε|~ῳ~ζεν", "~ῳ~ζετην", "~ῳ~ζον";
    imperfect_mid_1: "~ῳ~ζομην", _, "~ῳ~ζομεθᾰ";
    imperfect_mid_2: "~ῳ~ζου", "~ῳ~ζεσθον", "~ῳ~ζεσθε";
    imperfect_mid_3: "~ῳ~ζετο", "~ῳ~ζεσθην", "~ῳ~ζοντο";
    future_ind_pass_1: "~οι~σθησομαι", _, "~οι~σθησομεθᾰ";
    future_ind_pass_2: "~οι~σθησῃ|~οι~σθησει", "~οι~σθησεσθον", "~οι~σθησεσθε";
    future_ind_pass_3: "~οι~σθησεται", _, "~οι~σθησονται";
    future_subj_act_1: derived, _, derived;
    future_subj_act_2: derived, derived, derived;
    future_subj_act_3: derived, _, derived;
    future_subj_mid_1: derived, _, derived;
    future_subj_mid_2: _, derived, derived;
    future_subj_mid_3: derived, _, derived;
    future_opt_pass_1: "~οι~σθησοιμην", _, "~οι~σθησοιμεθᾰ";
    future_opt_pass_2: "~οι~σθησοιο", "~οι~σθησοισθον", "~οι~σθησοισθε";
    future_opt_pass_3: "~οι~σθησοιτο", "~οι~σθησοισθην", "~οι~σθησοιντο";
    future_imp_act_2: derived, _, _;
    future_imp_act_3: derived, derived, derived;
    future_imp_mid_2: derived, _, _;
    future_imp_mid_3: derived, derived, _;
    aorist_subj_act_1: "~οι~σω", _, "~οι~σωμεν";
    aorist_subj_act_2: "~οι~σῃς", "~οι~σητον", "~οι~σητε";
    aorist_subj_act_3: "~οι~σῃ", _, "~οι~σωσῐ|~οι~σωσῐν";
    aorist_subj_mid_1: "~οι~σωμαι", _, "~οι~σωμεθᾰ";
    aorist_subj_mid_2: _, "~οι~σησθον", "~οι~σησθε";
    aorist_subj_mid_3: "~οι~σηται", _, "~οι~σωνται";
    aorist_subj_pass_1: "~οι~σθω", _, "~οι~σθωμεν";
    aorist_subj_pass_2: "~οι~σθῃς", "~οι~σθητον", "~οι~σθητε";
    aorist_subj_pass_3: "~οι~σθῃ", _, "~οι~σθωσῐ|~οι~σθωσῐν";
    aorist_opt_act_1: "~οι~σαιμῐ", _, "~οι~σαιμεν";
    aorist_opt_act_2: "~οι~σειᾰς|~οι~σαις", "~οι~σαιτον", "~οι~σαιτε";
    aorist_opt_act_3: "~οι~σειε|~οι~σειεν|~οι~σαι", "~οι~σαιτην", "~οι~σειᾰν|~οι~σαιεν";
    aorist_opt_mid_1: "~οι~σαιμην", _, "~οι~σαιμεθᾰ";
    aorist_opt_mid_2: "~οι~σαιο", "~οι~σαισθον", "~οι~σαισθε";
    aorist_opt_mid_3: "~οι~σαιτο", "~οι~σαισθην", "~οι~σαιντο";
    aorist_opt_pass_1: "~οι~σθειην", _, "~οι~σθεῑμεν|~οι~σθειημεν";
    aorist_opt_pass_2: "~οι~σθειης", "~οι~σθεῑτον|~οι~σθειητον", "~οι~σθεῑτε|~οι~σθειητε";
    aorist_opt_pass_3: "~οι~σθειη", "~οι~σθειτην|~οι~σθειητην", "~οι~σθεῑεν|~οι~σθειησᾰν";
    aorist_imp_act_2: "~οι~σον", "~οι~σᾰτον", "~οι~σᾰτε";
    aorist_imp_act_3: "~οι~σᾰτω", "~οι~σᾰτων", "~οι~σᾰντων";
    aorist_imp_mid_2: "~οι~σαι", "~οι~σᾰσθον", "~οι~σᾰσθε";
    aorist_imp_mid_3: "~οι~σᾰσθω", "~οι~σᾰσθων", _;
    aorist_imp_pass_2: "~οι~σθητῐ", _, _;
    aorist_imp_pass_3: "~οι~σθητω", "~οι~σθητων", "~οι~σθεντων";
    perfect_ind_act_1: "~ῳ~κᾰ", _, "~ῳ~κᾰμεν";
    perfect_ind_act_2: "~ῳ~κᾰς", "~ῳ~κᾰτον", "~ῳ~κᾰτε";
    perfect_ind_act_3: "~ῳ~κε|~ῳ~κεν", _, "~ῳ~κᾱσῐ|~ῳ~κᾱσῐν";
    perfect_ind_mid_1: "~ῳ~σμαι", _, "~ῳ~σμεθᾰ";
    perfect_ind_mid_2: "~ῳ~σαι", "~ῳ~σθον", "~ῳ~σθε";
    perfect_ind_mid_3: "~ῳ~σται", _, "~ῳ~δᾰται";
    perfect_subj_act_1: "~ῳ~κω", _, "~ῳ~κωμεν";
    perfect_subj_act_2: "~ῳ~κῃς", "~ῳ~κητον", "~ῳ~κητε";
    perfect_subj_act_3: "~ῳ~κῃ", _, "~ῳ~κωσῐ|~ῳ~κωσῐν";
    perfect_opt_act_1: "~ῳ~κοιμῐ|~ῳ~κοιην", _, "~ῳ~κοιμεν";
    perfect_opt_act_2: "~ῳ~κοις|~ῳ~κοιης", "~ῳ~κοιτον", "~ῳ~κοιτε";
    perfect_opt_act_3: "~ῳ~κοι|~ῳ~κοιη", "~ῳ~κοιτην", "~ῳ~κοιεν";
    perfect_imp_act_2: "~ῳ~κε", "~ῳ~κετον", "~ῳ~κετε";
    perfect_imp_act_3: "~ῳ~κετω", "~ῳ~κετων", "~ῳ~κοντων";
    perfect_imp_mid_2: "~ῳ~σο", _, _;
    perfect_imp_mid_3: "~ῳ~σθω", "~ῳ~σθων", _;
    infinitive_pres: "~οι~ζειν", "~οι~ζεσθαι", _;
    infinitive_fut: "~οι~εῑν", "~οι~εῑσθαι", "~οι~σθησεσθαι";
    infinitive_aor: "~οι~σαι", "~οι~σᾰσθαι", "~οι~σθηναι";
    infinitive_perf: "~ῳ~κεναι", "~ῳ~σθαι", _;
    participle_pres: "~οι~ζων", "~οι~ζομενος", _;
    participle_fut: "~οι~ων", "~οι~ουμενος", "~οι~σθησομενος";
    participle_aor: "~οι~σᾱς", "~οι~σᾰμενος", "~οι~σθεις";
    participle_perf: "~ῳ~κως", "~ῳ~σμενος", _;
    ),
    // Θρῠπτω: 5 words (βᾰπτω, θρῠπτω, γνᾰμπτω, μᾰρπτω, δρῠπτω). Like Ἁπτω, except in 107 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_3 pl, future_ind_mid_2 sg, ….
    conjugation!("θρῠπτω", 5, "~πτω", like Ἁπτω;
    stem aorist_ind: "~ψ- like ποιεω";
    ind_act_3: _, _, "~πτουσῐ|~πτουσῐν";
    ind_mid_2: "~πτῃ|~πτει", _, _;
    subj_act_3: _, _, "~πτωσῐ|~πτωσῐν";
    imperfect_act_3: "ἐ~πτε|ἐ~πτεν", _, _;
    future_ind_act_3: _, _, "~ψουσῐ|~ψουσῐν";
    future_ind_mid_2: "~ψῃ|~ψει", _, _;
    future_ind_pass_1: "~φησομαι", _, "~φησομεθᾰ";
    future_ind_pass_2: "~φησῃ|~φησει", "~φησεσθον", "~φησεσθε";
    future_ind_pass_3: "~φησεται", _, "~φησονται";
    future_opt_pass_1: "~φησοιμην", _, "~φησοιμεθᾰ";
    future_opt_pass_2: "~φησοιο", "~φησοισθον", "~φησοισθε";
    future_opt_pass_3: "~φησοιτο", "~φησοισθην", "~φησοιντο";
    aorist_subj_act_3: _, _, "~ψωσῐ|~ψωσῐν";
    aorist_subj_pass_3: _, _, "~φθωσῐ|~φθωσῐν";
    aorist_opt_act_2: "~ψειᾰς|~ψαις", _, _;
    aorist_opt_act_3: "~ψειε|~ψειεν|~ψαι", _, "~ψειᾰν|~ψαιεν";
    aorist_opt_pass_1: _, _, "~φθεῑμεν|~φθειημεν";
    aorist_opt_pass_2: _, "~φθεῑτον|~φθειητον", "~φθεῑτε|~φθειητε";
    aorist_opt_pass_3: _, "~φθειτην|~φθειητην", "~φθεῑεν|~φθειησᾰν";
    perfect_ind_act_1: "%ε~πτᾰ", _, "%ε~πτᾰμεν";
    perfect_ind_act_2: "%ε~πτᾰς", "%ε~πτᾰτον", "%ε~πτᾰτε";
    perfect_ind_act_3: "%ε~πτε", _, "%ε~πτᾱσῐ";
    perfect_ind_mid_1: "%ε~μμαι", _, "%ε~μμεθᾰ";
    perfect_ind_mid_2: "%ε~ψαι", "%ε~φθον", "%ε~φθε";
    perfect_ind_mid_3: "%ε~πται", _, "%ε~φᾰται";
    perfect_subj_act_1: "%ε~πτω", _, "%ε~πτωμεν";
    perfect_subj_act_2: "%ε~πτῃς", "%ε~πτητον", "%ε~πτητε";
    perfect_subj_act_3: "%ε~πτῃ", _, "%ε~πτωσῐ";
    perfect_opt_act_1: "%ε~πτοιμῐ", _, "%ε~πτοιμεν";
    perfect_opt_act_2: "%ε~πτοις", "%ε~πτοιτον", "%ε~πτοιτε";
    perfect_opt_act_3: "%ε~πτοι", "%ε~πτοιτην", "%ε~πτοιεν";
    perfect_imp_act_2: _, "%ε~πτετον", "%ε~πτετε";
    perfect_imp_act_3: "%ε~πτετω", "%ε~πτετων", "%ε~πτοντων";
    perfect_imp_mid_2: "%ε~ψο", _, _;
    perfect_imp_mid_3: "%ε~φθω", "%ε~φθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_fut: _, _, "~φησεσθαι";
    infinitive_perf: "%ε~πτεναι", "%ε~φθαι", _;
    infinitive_futp: _, derived, _;
    participle_fut: _, _, "~φησομενος";
    participle_perf: "%ε~πτως", "%ε~μμενος", _;
    participle_futp: _, derived, _;
    ),
    // Μωραινω: 5 words (μιαινω, κερδαινω, μελαινω, εὐφραινω, μωραινω). Like Βαινω, except in 178 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imp_act_2 sg, imperfect_act_3 sg, future_ind_act_1 sg, ….
    conjugation!("μωραινω", 5, "~αινω", like Βαινω as AORIST_AS_FUTURE;
    stem aorist_ind: "~ᾱν- like ἐχω";
    stem aorist_ind_pass: "~ανθη- like ἐχω";
    ind_act_3: _, _, "~αινουσῐ|~αινουσῐν";
    ind_mid_2: "~αινῃ|~αινει", _, _;
    subj_act_3: _, _, "~αινωσῐ|~αινωσῐν";
    imp_act_2: "~αινε", _, _;
    imperfect_act_3: "ἐ~αινε|ἐ~αινεν", _, _;
    future_ind_act_1: "~ανω", _, "~ανοῡμεν";
    future_ind_act_2: "~ανεῑς", "~ανεῑτον", "~ανεῑτε";
    future_ind_act_3: "~ανεῑ", _, "~ανοῡσῐ";
    future_ind_mid_1: "~ανοῡμαι", _, "~ανουμεθᾰ";
    future_ind_mid_2: "~ανεῑ", "~ανεῑσθον", "~ανεῑσθε";
    future_ind_mid_3: "~ανεῑται", _, "~ανοῡνται";
    future_ind_pass_1: "~ανθησομαι", _, "~ανθησομεθᾰ";
    future_ind_pass_2: "~ανθησῃ|~ανθησει", "~ανθησεσθον", "~ανθησεσθε";
    future_ind_pass_3: "~ανθησεται", _, "~ανθησονται";
    future_opt_act_1: "~ανοιην", _, "~ανοῑμεν";
    future_opt_act_2: "~ανοιης", "~ανοῑτον", "~ανοῑτε";
    future_opt_act_3: "~ανοιη", "~ανοιτην", "~ανοῑεν";
    future_opt_mid_1: "~ανοιμην", _, "~ανοιμεθᾰ";
    future_opt_mid_2: "~ανοῑο", "~ανοῑσθον", "~ανοῑσθε";
    future_opt_mid_3: "~ανοῑτο", "~ανοισθην", "~ανοῑντο";
    future_opt_pass_1: "~ανθησοιμην", _, "~ανθησοιμεθᾰ";
    future_opt_pass_2: "~ανθησοιο", "~ανθησοισθον", "~ανθησοισθε";
    future_opt_pass_3: "~ανθησοιτο", "~ανθησοισθην", "~ανθησοιντο";
    aorist_subj_act_1: "~ᾱνω", _, "~ᾱνωμεν";
    aorist_subj_act_2: "~ᾱνῃς", "~ᾱνητον", "~ᾱνητε";
    aorist_subj_act_3: "~ᾱνῃ", _, "~ᾱνωσῐ";
    aorist_subj_mid_1: "~ᾱνωμαι", _, "~ᾱνωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾱνησθον", "~ᾱνησθε";
    aorist_subj_mid_3: "~ᾱνηται", _, "~ᾱνωνται";
    aorist_subj_pass_1: "~ανθω", _, "~ανθωμεν";
    aorist_subj_pass_2: "~ανθῃς", "~ανθητον", "~ανθητε";
    aorist_subj_pass_3: "~ανθῃ", _, "~ανθωσῐ";
    aorist_opt_act_1: "~ᾱναιμῐ", _, "~ᾱναιμεν";
    aorist_opt_act_2: "~ᾱνειᾰς", "~ᾱναιτον", "~ᾱναιτε";
    aorist_opt_act_3: "~ᾱνειε", "~ᾱναιτην", "~ᾱνειᾰν";
    aorist_opt_mid_1: "~ᾱναιμην", _, "~ᾱναιμεθᾰ";
    aorist_opt_mid_2: "~ᾱναιο", "~ᾱναισθον", "~ᾱναισθε";
    aorist_opt_mid_3: "~ᾱναιτο", "~ᾱναισθην", "~ᾱναιντο";
    aorist_opt_pass_1: "~ανθειην", _, "~ανθεῑμεν";
    aorist_opt_pass_2: "~ανθειης", "~ανθεῑτον", "~ανθεῑτε";
    aorist_opt_pass_3: "~ανθειη", "~ανθειτην", "~ανθεῑεν";
    aorist_imp_act_2: "~ᾱνον", "~ᾱνᾰτον", "~ᾱνᾰτε";
    aorist_imp_act_3: "~ᾱνᾰτω", "~ᾱνᾰτων", "~ᾱνᾰντων";
    aorist_imp_mid_2: "~ᾱναι", "~ᾱνᾰσθον", "~ᾱνᾰσθε";
    aorist_imp_mid_3: "~ᾱνᾰσθω", "~ᾱνᾰσθων", _;
    aorist_imp_pass_2: "~ανθητῐ", _, _;
    aorist_imp_pass_3: "~ανθητω", "~ανθητων", "~ανθεντων";
    perfect_ind_act_1: "%ε~ᾰγκᾰ", _, "%ε~ᾰγκᾰμεν";
    perfect_ind_act_2: "%ε~ᾰγκᾰς", "%ε~ᾰγκᾰτον", "%ε~ᾰγκᾰτε";
    perfect_ind_act_3: "%ε~ᾰγκε", _, "%ε~ᾰγκᾱσῐ";
    perfect_ind_mid_1: "%ε~ᾰσμαι", _, "%ε~ᾰσμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾰνσαι", "%ε~ᾰνθον", "%ε~ᾰνθε";
    perfect_ind_mid_3: "%ε~ᾰνται", _, "%ε~ᾰνᾰται";
    perfect_subj_act_1: "%ε~ᾰγκω", _, "%ε~ᾰγκωμεν";
    perfect_subj_act_2: "%ε~ᾰγκῃς", "%ε~ᾰγκητον", "%ε~ᾰγκητε";
    perfect_subj_act_3: "%ε~ᾰγκῃ", _, "%ε~ᾰγκωσῐ";
    perfect_opt_act_1: "%ε~ᾰγκοιμῐ", _, "%ε~ᾰγκοιμεν";
    perfect_opt_act_2: "%ε~ᾰγκοις", "%ε~ᾰγκοιτον", "%ε~ᾰγκοιτε";
    perfect_opt_act_3: "%ε~ᾰγκοι", "%ε~ᾰγκοιτην", "%ε~ᾰγκοιεν";
    perfect_imp_act_2: _, "%ε~ᾰγκετον", "%ε~ᾰγκετε";
    perfect_imp_act_3: "%ε~ᾰγκετω", "%ε~ᾰγκετων", "%ε~ᾰγκοντων";
    perfect_imp_mid_2: "%ε~ᾰνσο", _, _;
    perfect_imp_mid_3: "%ε~ᾰνθω", "%ε~ᾰνθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_fut: "~ανεῑν", "~ανεῑσθαι", "~ανθησεσθαι";
    infinitive_aor: "~ᾱναι", "~ᾱνᾰσθαι", "~ανθηναι";
    infinitive_perf: "%ε~ᾰγκεναι", "%ε~ᾰνθαι", _;
    participle_fut: "~ανων", "~ανουμενος", "~ανθησομενος";
    participle_aor: "~ᾱνᾱς", "~ᾱνᾰμενος", "~ανθεις";
    participle_perf: "%ε~ᾰγκως", "%ε~ᾰσμενος", _;
    ),
    // Ἰᾱτρευω: 5 words (ἰᾱτρευω, ἰαυω, ἡγεμονευω, ἱκετευω, ἱππευω). Like Βουλευω, except in 119 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("ἰᾱτρευω", 5, "~υω", like Βουλευω;
    stem aorist_ind: "~υσ- like εὐλογεω";
    imperfect_act_1: "~υον", _, "~υομεν";
    imperfect_act_2: "~υες", "~υετον", "~υετε";
    imperfect_act_3: "~υε|~υεν", "~υετην", "~υον";
    imperfect_mid_1: "~υομην", _, "~υομεθᾰ";
    imperfect_mid_2: "~υου", "~υεσθον", "~υεσθε";
    imperfect_mid_3: "~υετο", "~υεσθην", "~υοντο";
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_pass_1: derived, _, derived;
    aorist_ind_pass_2: derived, derived, derived;
    aorist_ind_pass_3: derived, derived, derived;
    aorist_subj_pass_1: derived, _, derived;
    aorist_subj_pass_2: derived, derived, derived;
    aorist_subj_pass_3: derived, _, derived;
    aorist_opt_pass_1: derived, _, derived;
    aorist_opt_pass_2: derived, derived, derived;
    aorist_opt_pass_3: derived, derived, derived;
    aorist_imp_pass_2: derived, _, _;
    aorist_imp_pass_3: derived, derived, derived;
    perfect_ind_act_1: "~υκᾰ", _, "~υκᾰμεν";
    perfect_ind_act_2: "~υκᾰς", "~υκᾰτον", "~υκᾰτε";
    perfect_ind_act_3: "~υκε", _, "~υκᾱσῐ";
    perfect_ind_mid_1: "~υμαι", _, "~υμεθᾰ";
    perfect_ind_mid_2: "~υσαι", "~υσθον", "~υσθε";
    perfect_ind_mid_3: "~υται", _, "~υνται";
    perfect_subj_act_1: "~υκω", _, "~υκωμεν";
    perfect_subj_act_2: "~υκῃς", "~υκητον", "~υκητε";
    perfect_subj_act_3: "~υκῃ", _, "~υκωσῐ";
    perfect_opt_act_1: "~υκοιμῐ", _, "~υκοιμεν";
    perfect_opt_act_2: "~υκοις", "~υκοιτον", "~υκοιτε";
    perfect_opt_act_3: "~υκοι", "~υκοιτην", "~υκοιεν";
    perfect_imp_act_2: _, "~υκετον", "~υκετε";
    perfect_imp_act_3: "~υκετω", "~υκετων", "~υκοντων";
    perfect_imp_mid_2: "~υσο", _, _;
    perfect_imp_mid_3: "~υσθω", "~υσθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_fut: _, _, derived;
    infinitive_aor: _, _, derived;
    infinitive_perf: "~υκεναι", "~ῡσθαι", _;
    participle_fut: _, _, derived;
    participle_aor: _, _, derived;
    participle_perf: "~υκως", "~υμενος", _;
    ),
    // Ῥηγνῡμῐ: 5 words (ῥηγνῡμῐ, πηγνῡμῐ, ξῠρρηγνῡμῐ, ἐμπηγνῡμῐ, ρρηγνῡμῐ). Like Μῑγνῡμῐ, except in 218 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ῥηγνῡμῐ", 5, "~ηγνῡμῐ", like Μῑγνῡμῐ;
    stem aorist_ind: "~ηξ- like ἐχω";
    stem aorist_ind_pass: "~ᾰγη- like ἐχω";
    stem pluperfect_act: "~ωγε- like κᾰλεω";
    ind_act_1: "~ηγνῡμῐ", _, "~ηγνῠμεν";
    ind_act_2: "~ηγνῡς", "~ηγνῠτον", "~ηγνῠτε";
    ind_act_3: "~ηγνῡσῐ", _, "~ηγνῠᾱσῐ";
    ind_mid_1: "~ηγνῠμαι", _, "~ηγνῠμεθᾰ";
    ind_mid_2: "~ηγνῠσαι", "~ηγνῠσθον", "~ηγνῠσθε";
    ind_mid_3: "~ηγνῠται", _, "~ηγνῠνται";
    subj_act_1: "~ηγνῠω", _, "~ηγνῠωμεν";
    subj_act_2: "~ηγνῠῃς", "~ηγνῠητον", "~ηγνῠητε";
    subj_act_3: "~ηγνῠῃ", _, "~ηγνῠωσῐ";
    subj_mid_1: "~ηγνῠωμαι", _, "~ηγνῠωμεθᾰ";
    subj_mid_2: _, "~ηγνῠησθον", "~ηγνῠησθε";
    subj_mid_3: "~ηγνῠηται", _, "~ηγνῠωνται";
    opt_act_1: "~ηγνῠοιμῐ", _, "~ηγνῠοιμεν";
    opt_act_2: "~ηγνῠοις", "~ηγνῠοιτον", "~ηγνῠοιτε";
    opt_act_3: "~ηγνῠοι", "~ηγνῠοιτην", "~ηγνῠοιεν";
    opt_mid_1: "~ηγνῠοιμην", _, "~ηγνῠοιμεθᾰ";
    opt_mid_2: "~ηγνῠοιο", "~ηγνῠοισθον", "~ηγνῠοισθε";
    opt_mid_3: "~ηγνῠοιτο", "~ηγνῠοισθην", "~ηγνῠοιντο";
    imp_act_2: "~ηγνῡ", _, _;
    imp_act_3: "~ηγνῠτω", "~ηγνῠτων", "~ηγνῠντων";
    imp_mid_2: "~ηγνῠσο", _, _;
    imp_mid_3: "~ηγνῠσθω", "~ηγνῠσθων", _;
    imperfect_act_1: "ἐ~ηγνῡν", _, "ἐ~ηγνῠμεν";
    imperfect_act_2: "ἐ~ηγνῡς", "ἐ~ηγνῠτον", "ἐ~ηγνῠτε";
    imperfect_act_3: "ἐ~ηγνῡ", "ἐ~ηγνῠτην", "ἐ~ηγνῠσᾰν";
    imperfect_mid_1: "ἐ~ηγνῠμην", _, "ἐ~ηγνῠμεθᾰ";
    imperfect_mid_2: "ἐ~ηγνῠσο", "ἐ~ηγνῠσθον", "ἐ~ηγνῠσθε";
    imperfect_mid_3: "ἐ~ηγνῠτο", "ἐ~ηγνῠσθην", "ἐ~ηγνῠντο";
    future_ind_act_1: "~ηξω", _, "~ηξομεν";
    future_ind_act_2: "~ηξεις", "~ηξετον", "~ηξετε";
    future_ind_act_3: "~ηξει", _, "~ηξουσῐ";
    future_ind_mid_1: "~ηξομαι", _, "~ηξομεθᾰ";
    future_ind_mid_2: "~ηξῃ", "~ηξεσθον", "~ηξεσθε";
    future_ind_mid_3: "~ηξεται", _, "~ηξονται";
    future_opt_act_1: "~ηξοιμῐ", _, "~ηξοιμεν";
    future_opt_act_2: "~ηξοις", "~ηξοιτον", "~ηξοιτε";
    future_opt_act_3: "~ηξοι", "~ηξοιτην", "~ηξοιεν";
    future_opt_mid_1: "~ηξοιμην", _, "~ηξοιμεθᾰ";
    future_opt_mid_2: "~ηξοιο", "~ηξοισθον", "~ηξοισθε";
    future_opt_mid_3: "~ηξοιτο", "~ηξοισθην", "~ηξοιντο";
    aorist_subj_act_1: "~ηξω", _, "~ηξωμεν";
    aorist_subj_act_2: "~ηξῃς", "~ηξητον", "~ηξητε";
    aorist_subj_act_3: "~ηξῃ", _, "~ηξωσῐ";
    aorist_subj_mid_1: "~ηξωμαι", _, "~ηξωμεθᾰ";
    aorist_subj_mid_2: _, "~ηξησθον", "~ηξησθε";
    aorist_subj_mid_3: "~ηξηται", _, "~ηξωνται";
    aorist_subj_pass_1: "~ᾰγω", _, "~ᾰγωμεν";
    aorist_subj_pass_2: "~ᾰγῃς", "~ᾰγητον", "~ᾰγητε";
    aorist_subj_pass_3: "~ᾰγῃ", _, "~ᾰγωσῐ";
    aorist_opt_act_1: "~ηξαιμῐ", _, "~ηξαιμεν";
    aorist_opt_act_2: "~ηξειᾰς", "~ηξαιτον", "~ηξαιτε";
    aorist_opt_act_3: "~ηξειε", "~ηξαιτην", "~ηξειᾰν";
    aorist_opt_mid_1: "~ηξαιμην", _, "~ηξαιμεθᾰ";
    aorist_opt_mid_2: "~ηξαιο", "~ηξαισθον", "~ηξαισθε";
    aorist_opt_mid_3: "~ηξαιτο", "~ηξαισθην", "~ηξαιντο";
    aorist_opt_pass_1: "~ᾰγειην", _, "~ᾰγεῑμεν";
    aorist_opt_pass_2: "~ᾰγειης", "~ᾰγεῑτον", "~ᾰγεῑτε";
    aorist_opt_pass_3: "~ᾰγειη", "~ᾰγειτην", "~ᾰγεῑεν";
    aorist_imp_act_2: "~ηξον", "~ηξᾰτον", "~ηξᾰτε";
    aorist_imp_act_3: "~ηξᾰτω", "~ηξᾰτων", "~ηξᾰντων";
    aorist_imp_mid_2: "~ηξαι", "~ηξᾰσθον", "~ηξᾰσθε";
    aorist_imp_mid_3: "~ηξᾰσθω", "~ηξᾰσθων", _;
    aorist_imp_pass_2: "~ᾰγηθῐ", _, _;
    aorist_imp_pass_3: "~ᾰγητω", "~ᾰγητων", "~ᾰγεντων";
    perfect_ind_act_1: "ἐ~ωγᾰ", _, "ἐ~ωγᾰμεν";
    perfect_ind_act_2: "ἐ~ωγᾰς", "ἐ~ωγᾰτον", "ἐ~ωγᾰτε";
    perfect_ind_act_3: "ἐ~ωγε", _, "ἐ~ωγᾱσῐ";
    perfect_ind_mid_1: "~ηρρηγμαι", _, "~ηρρηγμεθᾰ";
    perfect_ind_mid_2: "~ηρρηξαι", "~ηρρηχθον", "~ηρρηχθε";
    perfect_ind_mid_3: "~ηρρηκται", _, "~ηρρηγᾰται";
    perfect_subj_act_1: "ἐ~ωγω", _, "ἐ~ωγωμεν";
    perfect_subj_act_2: "ἐ~ωγῃς", "ἐ~ωγητον", "ἐ~ωγητε";
    perfect_subj_act_3: "ἐ~ωγῃ", _, "ἐ~ωγωσῐ";
    perfect_opt_act_1: "ἐ~ωγοιμῐ", _, "ἐ~ωγοιμεν";
    perfect_opt_act_2: "ἐ~ωγοις", "ἐ~ωγοιτον", "ἐ~ωγοιτε";
    perfect_opt_act_3: "ἐ~ωγοι", "ἐ~ωγοιτην", "ἐ~ωγοιεν";
    perfect_imp_act_2: _, "ἐ~ωγετον", "ἐ~ωγετε";
    perfect_imp_act_3: "ἐ~ωγετω", "ἐ~ωγετων", "ἐ~ωγοντων";
    perfect_imp_mid_2: "~ηρρηξο", _, _;
    perfect_imp_mid_3: "~ηρρηχθω", "~ηρρηχθων", _;
    pluperfect_mid_1: "~ηρρηγμην", _, "~ηρρηγμεθᾰ";
    pluperfect_mid_2: "~ηρρηξο", "~ηρρηχθον", "~ηρρηχθε";
    pluperfect_mid_3: "~ηρρηκτο", "~ηρρηχθην", "~ηρρηγᾰτο";
    infinitive_pres: "~ηγνῠναι", "~ηγνῠσθαι", _;
    infinitive_fut: "~ηξειν", "~ηξεσθαι", _;
    infinitive_aor: "~ηξαι", "~ηξᾰσθαι", "~ᾰγηναι";
    infinitive_perf: "ἐ~ωγεναι", "~ηρρηχθαι", _;
    participle_pres: "~ηγνῡς", "~ηγνῠμενος", _;
    participle_fut: "~ηξων", "~ηξομενος", _;
    participle_aor: "~ηξᾱς", "~ηξᾰμενος", "~ᾰγεις";
    participle_perf: "ἐ~ωγως", "~ηρρηγμενος", _;
    ),
    // Τρεφω: 5 words (νεμω, τρεφω, δερω, δεμω, ξῠμμενω). Like Ἀειρω, except in 141 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("τρεφω", 5, "~ε~ω", like Ἀειρω;
    stem imperfect: "~ε~- like ῥεω";
    stem aorist_ind_pass: "~ᾰ~η- like ἐχω";
    stem pluperfect_act: "~εδᾰ~κε- like κᾰλεω";
    ind_act_1: "~ε~ω", _, "~ε~ομεν";
    ind_act_2: "~ε~εις", "~ε~ετον", "~ε~ετε";
    ind_act_3: "~ε~ει", _, "~ε~ουσῐ";
    ind_mid_1: "~ε~ομαι", _, "~ε~ομεθᾰ";
    ind_mid_2: "~ε~ῃ", "~ε~εσθον", "~ε~εσθε";
    ind_mid_3: "~ε~εται", _, "~ε~ονται";
    subj_act_1: "~ε~ω", _, "~ε~ωμεν";
    subj_act_2: "~ε~ῃς", "~ε~ητον", "~ε~ητε";
    subj_act_3: "~ε~ῃ", _, "~ε~ωσῐ";
    subj_mid_1: "~ε~ωμαι", _, "~ε~ωμεθᾰ";
    subj_mid_2: _, "~ε~ησθον", "~ε~ησθε";
    subj_mid_3: "~ε~ηται", _, "~ε~ωνται";
    opt_act_1: "~ε~οιμῐ", _, "~ε~οιμεν";
    opt_act_2: "~ε~οις", "~ε~οιτον", "~ε~οιτε";
    opt_act_3: "~ε~οι", "~ε~οιτην", "~ε~οιεν";
    opt_mid_1: "~ε~οιμην", _, "~ε~οιμεθᾰ";
    opt_mid_2: "~ε~οιο", "~ε~οισθον", "~ε~οισθε";
    opt_mid_3: "~ε~οιτο", "~ε~οισθην", "~ε~οιντο";
    imp_act_2: "~ε~ε", _, _;
    imp_act_3: "~ε~ετω", "~ε~ετων", "~ε~οντων";
    imp_mid_2: "~ε~ου", _, _;
    imp_mid_3: "~ε~εσθω", "~ε~εσθων", _;
    future_ind_pass_1: "~ᾰ~ησομαι", _, "~ᾰ~ησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ησῃ", "~ᾰ~ησεσθον", "~ᾰ~ησεσθε";
    future_ind_pass_3: "~ᾰ~ησεται", _, "~ᾰ~ησονται";
    future_opt_pass_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ησοιο", "~ᾰ~ησοισθον", "~ᾰ~ησοισθε";
    future_opt_pass_3: "~ᾰ~ησοιτο", "~ᾰ~ησοισθην", "~ᾰ~ησοιντο";
    aorist_subj_pass_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    aorist_subj_pass_2: "~ᾰ~ῃς", "~ᾰ~ητον", "~ᾰ~ητε";
    aorist_subj_pass_3: "~ᾰ~ῃ", _, "~ᾰ~ωσῐ";
    aorist_opt_pass_1: "~ᾰ~ειην", _, "~ᾰ~εῑμεν";
    aorist_opt_pass_2: "~ᾰ~ειης", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    aorist_opt_pass_3: "~ᾰ~ειη", "~ᾰ~ειτην", "~ᾰ~εῑεν";
    aorist_imp_pass_2: "~ᾰ~ηθῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ητω", "~ᾰ~ητων", "~ᾰ~εντων";
    perfect_ind_act_1: "~εδᾰ~κᾰ", _, "~εδᾰ~κᾰμεν";
    perfect_ind_act_2: "~εδᾰ~κᾰς", "~εδᾰ~κᾰτον", "~εδᾰ~κᾰτε";
    perfect_ind_act_3: "~εδᾰ~κε", _, "~εδᾰ~κᾱσῐ";
    perfect_ind_mid_1: "~εδᾰ~μαι", _, "~εδᾰ~μεθᾰ";
    perfect_ind_mid_2: "~εδᾰ~σαι", "~εδᾰ~θον", "~εδᾰ~θε";
    perfect_ind_mid_3: "~εδᾰ~ται", _, "~εδᾰ~ᾰται";
    perfect_subj_act_1: "~εδᾰ~κω", _, "~εδᾰ~κωμεν";
    perfect_subj_act_2: "~εδᾰ~κῃς", "~εδᾰ~κητον", "~εδᾰ~κητε";
    perfect_subj_act_3: "~εδᾰ~κῃ", _, "~εδᾰ~κωσῐ";
    perfect_opt_act_1: "~εδᾰ~κοιμῐ", _, "~εδᾰ~κοιμεν";
    perfect_opt_act_2: "~εδᾰ~κοις", "~εδᾰ~κοιτον", "~εδᾰ~κοιτε";
    perfect_opt_act_3: "~εδᾰ~κοι", "~εδᾰ~κοιτην", "~εδᾰ~κοιεν";
    perfect_imp_act_2: _, "~εδᾰ~κετον", "~εδᾰ~κετε";
    perfect_imp_act_3: "~εδᾰ~κετω", "~εδᾰ~κετων", "~εδᾰ~κοντων";
    perfect_imp_mid_2: "~εδᾰ~σο", _, _;
    perfect_imp_mid_3: "~εδᾰ~θω", "~εδᾰ~θων", _;
    pluperfect_mid_1: "ἐ~εδᾰ~μην", _, "ἐ~εδᾰ~μεθᾰ";
    pluperfect_mid_2: "ἐ~εδᾰ~σο", "ἐ~εδᾰ~θον", "ἐ~εδᾰ~θε";
    pluperfect_mid_3: "ἐ~εδᾰ~το", "ἐ~εδᾰ~θην", "ἐ~εδᾰ~ᾰτο";
    infinitive_pres: "~ε~ειν", "~ε~εσθαι", _;
    infinitive_fut: _, _, "~ᾰ~ησεσθαι";
    infinitive_aor: _, _, "~ᾰ~ηναι";
    infinitive_perf: "~εδᾰ~κεναι", "~εδᾰ~θαι", _;
    participle_pres: "~ε~ων", "~ε~ομενος", _;
    participle_fut: _, _, "~ᾰ~ησομενος";
    participle_aor: _, _, "~ᾰ~εις";
    participle_perf: "~εδᾰ~κως", "~εδᾰ~μενος", _;
    ),
    // Προβαλλω: 5 words (βᾰλλω, προβαλλω, διαβαλλω, τῐλλω, σκελλω). Like Κρῑνομαι, except in 164 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("προβαλλω", 5, "~λω", like Κρῑνομαι as PASS_AS_ACT;
    stem imperfect: "~λ- like ῥεω";
    stem aorist_ind_pass: "~η- like ἐχω";
    ind_act_1: "~λω", _, "~λομεν";
    ind_act_2: "~λεις", "~λετον", "~λετε";
    ind_act_3: "~λει", _, "~λουσῐ";
    ind_mid_1: "~λομαι", _, "~λομεθᾰ";
    ind_mid_2: "~λῃ", "~λεσθον", "~λεσθε";
    ind_mid_3: "~λεται", _, "~λονται";
    ind_pass_1: derived, _, _;
    subj_act_1: "~λω", _, "~λωμεν";
    subj_act_2: "~λῃς", "~λητον", "~λητε";
    subj_act_3: "~λῃ", _, "~λωσῐ";
    subj_mid_1: "~λωμαι", _, "~λωμεθᾰ";
    subj_mid_2: derived, "~λησθον", "~λησθε";
    subj_mid_3: "~ληται", _, "~λωνται";
    opt_act_1: "~λοιμῐ", _, "~λοιμεν";
    opt_act_2: "~λοις", "~λοιτον", "~λοιτε";
    opt_act_3: "~λοι", "~λοιτην", "~λοιεν";
    opt_mid_1: "~λοιμην", _, "~λοιμεθᾰ";
    opt_mid_2: "~λοιο", "~λοισθον", "~λοισθε";
    opt_mid_3: "~λοιτο", "~λοισθην", "~λοιντο";
    imp_act_2: "~λε", _, _;
    imp_act_3: "~λετω", "~λετων", "~λοντων";
    imp_mid_2: "~λου", _, _;
    imp_mid_3: "~λεσθω", "~λεσθων", _;
    future_ind_act_1: "~ω", _, "~οῡμεν";
    future_ind_act_2: "~εῑς", "~εῑτον", "~εῑτε";
    future_ind_act_3: "~εῑ", _, "~οῡσῐ";
    future_ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    future_ind_mid_2: "~εῑ", "~εῑσθον", "~εῑσθε";
    future_ind_mid_3: "~εῑται", _, "~οῡνται";
    future_ind_pass_1: "~ησησομαι", _, "~ησησομεθᾰ";
    future_ind_pass_2: "~ησησῃ", "~ησησεσθον", "~ησησεσθε";
    future_ind_pass_3: "~ησησεται", _, "~ησησονται";
    future_opt_act_1: "~οιην", _, "~οῑμεν";
    future_opt_act_2: "~οιης", "~οῑτον", "~οῑτε";
    future_opt_act_3: "~οιη", "~οιτην", "~οῑεν";
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_2: "~οῑο", "~οῑσθον", "~οῑσθε";
    future_opt_mid_3: "~οῑτο", "~οισθην", "~οῑντο";
    future_opt_pass_1: "~ησησοιμην", _, "~ησησοιμεθᾰ";
    future_opt_pass_2: "~ησησοιο", "~ησησοισθον", "~ησησοισθε";
    future_opt_pass_3: "~ησησοιτο", "~ησησοισθην", "~ησησοιντο";
    aorist_opt_pass_1: "~ειην", _, "~εῑμεν";
    aorist_opt_pass_2: "~ειης", "~εῑτον", "~εῑτε";
    aorist_opt_pass_3: "~ειη", "~ειτην", "~εῑεν";
    aorist_imp_pass_2: "~ηθῐ", derived, derived;
    aorist_imp_pass_3: "~ητω", "~ητων", "~εντων";
    perfect_ind_act_1: "%ε~κᾰ", _, "%ε~κᾰμεν";
    perfect_ind_act_2: "%ε~κᾰς", "%ε~κᾰτον", "%ε~κᾰτε";
    perfect_ind_act_3: "%ε~κε", _, "%ε~κᾱσῐ";
    perfect_ind_mid_1: "ἐ~μαι", _, "ἐ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~σαι", "ἐ~θον", "ἐ~θε";
    perfect_ind_mid_3: "ἐ~ται", _, "ἐ~ᾰται";
    perfect_subj_act_1: "%ε~κω", _, "%ε~κωμεν";
    perfect_subj_act_2: "%ε~κῃς", "%ε~κητον", "%ε~κητε";
    perfect_subj_act_3: "%ε~κῃ", _, "%ε~κωσῐ";
    perfect_opt_act_1: "%ε~κοιμῐ", _, "%ε~κοιμεν";
    perfect_opt_act_2: "%ε~κοις", "%ε~κοιτον", "%ε~κοιτε";
    perfect_opt_act_3: "%ε~κοι", "%ε~κοιτην", "%ε~κοιεν";
    perfect_imp_act_2: _, "%ε~κετον", "%ε~κετε";
    perfect_imp_act_3: "%ε~κετω", "%ε~κετων", "%ε~κοντων";
    perfect_imp_mid_2: "ἐ~σο", _, _;
    perfect_imp_mid_3: "ἐ~θω", "ἐ~θων", _;
    infinitive_pres: "~λειν", "~λεσθαι", _;
    infinitive_fut: "~εῑν", "~εῑσθαι", "~ησησεσθαι";
    infinitive_aor: _, _, "~ηναι";
    infinitive_perf: "%ε~κεναι", "ἐ~θαι", _;
    participle_pres: "~λων", "~λομενος", _;
    participle_fut: "~ων", "~ουμενος", "~ησησομενος";
    participle_aor: _, _, "~εις";
    participle_perf: "%ε~κως", "ἐ~μενος", _;
    ),
    // Μετερχομαι: 5 words (προερχομαι, σῠνερχομαι, ἐπεξερχομαι, μετερχομαι, ξῠνερχομαι). Like Εἰσερχομαι, except in 10 cells: subj_pass_2 sg, perfect_ind_act_1 pl, perfect_ind_act_2 pl, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, ….
    conjugation!("μετερχομαι", 5, "~ερχομαι", like Εἰσερχομαι as PRES_AS_FUTP;
    stem pluperfect_act: "~εληλυθε- like ἀνῐστημῐ";
    subj_pass_2: derived, _, _;
    perfect_ind_act_1: _, _, "~εληλῠθᾰμεν";
    perfect_ind_act_2: _, _, "~εληλῠθᾰτε";
    perfect_ind_act_3: "~εληλῠθε|~εληλῠθεν", _, "~εληλῠθᾱσῐ|~εληλῠθᾱσῐν";
    perfect_subj_act_3: _, _, "~εληλῠθωσῐ|~εληλῠθωσῐν";
    perfect_opt_act_1: "~εληλῠθοιμῐ|~εληλῠθοιην", _, _;
    perfect_opt_act_2: "~εληλῠθοις|~εληλῠθοιης", _, _;
    perfect_opt_act_3: "~εληλῠθοι|~εληλῠθοιη", _, _;
    perfect_imp_act_2: "~εληλῠθε", _, _;
    ),
    // Φειδομαι: 5 words (εἰδομαι, ἐλδομαι, μηδομαι, λῐνδομαι, φειδομαι). Like Πῠνθανομαι, except in 96 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("φειδομαι", 5, "~~δομαι", like Πῠνθανομαι as PERF_AS_FUTP;
    stem imperfect: "~~δ- like ὑποκοριζομαι";
    stem aorist_ind: "~~σ- like πᾰραιτεομαι";
    ind_act_1: "~~δομαι", _, _;
    ind_mid_1: "~~δομαι", _, "~~δομεθᾰ";
    ind_mid_2: "~~δῃ", "~~δεσθον", "~~δεσθε";
    ind_mid_3: "~~δεται", _, "~~δονται";
    ind_pass_2: derived, _, _;
    subj_mid_1: "~~δωμαι", _, "~~δωμεθᾰ";
    subj_mid_2: "~~δῃ", "~~δησθον", "~~δησθε";
    subj_mid_3: "~~δηται", _, "~~δωνται";
    subj_pass_2: "~~δῃ", _, _;
    opt_mid_1: "~~δοιμην", _, "~~δοιμεθᾰ";
    opt_mid_2: "~~δοιο", "~~δοισθον", "~~δοισθε";
    opt_mid_3: "~~δοιτο", "~~δοισθην", "~~δοιντο";
    imp_mid_2: "~~δου", _, _;
    imp_mid_3: "~~δεσθω", "~~δεσθων", _;
    future_ind_mid_1: "~~σομαι", _, "~~σομεθᾰ";
    future_ind_mid_2: "~~σῃ", "~~σεσθον", "~~σεσθε";
    future_ind_mid_3: "~~σεται", _, "~~σονται";
    future_ind_pass_1: "~~σθησομαι", _, "~~σθησομεθᾰ";
    future_ind_pass_2: "~~σθησῃ", "~~σθησεσθον", "~~σθησεσθε";
    future_ind_pass_3: "~~σθησεται", _, "~~σθησονται";
    future_opt_mid_1: "~~σοιμην", _, "~~σοιμεθᾰ";
    future_opt_mid_2: "~~σοιο", "~~σοισθον", "~~σοισθε";
    future_opt_mid_3: "~~σοιτο", "~~σοισθην", "~~σοιντο";
    future_opt_pass_1: "~~σθησοιμην", _, "~~σθησοιμεθᾰ";
    future_opt_pass_2: "~~σθησοιο", "~~σθησοισθον", "~~σθησοισθε";
    future_opt_pass_3: "~~σθησοιτο", "~~σθησοισθην", "~~σθησοιντο";
    aorist_subj_mid_1: "~~σωμαι", _, "~~σωμεθᾰ";
    aorist_subj_mid_2: "~~σῃ", "~~σησθον", "~~σησθε";
    aorist_subj_mid_3: "~~σηται", _, "~~σωνται";
    aorist_opt_mid_1: "~~σαιμην", _, "~~σαιμεθᾰ";
    aorist_opt_mid_2: "~~σαιο", "~~σαισθον", "~~σαισθε";
    aorist_opt_mid_3: "~~σαιτο", "~~σαισθην", "~~σαιντο";
    aorist_imp_mid_2: "~ῑσα~", "~~σᾰσθον", "~~σᾰσθε";
    aorist_imp_mid_3: "~~σᾰσθω", "~~σᾰσθων", _;
    perfect_ind_mid_1: "%ε~~σμαι", _, "%ε~~σμεθᾰ";
    perfect_ind_mid_2: "%ε~~σαι", "%ε~~σθον", "%ε~~σθε";
    perfect_ind_mid_3: "%ε~~σται", _, "%ε~~δᾰται";
    perfect_imp_mid_2: "%ε~~σο", _, _;
    perfect_imp_mid_3: "%ε~~σθω", "%ε~~σθων", _;
    infinitive_pres: _, "~~δεσθαι", _;
    infinitive_fut: _, "~~σεσθαι", "~~σθησεσθαι";
    infinitive_aor: _, "~~σᾰσθαι", _;
    participle_pres: _, "~~δομενος", _;
    participle_fut: _, "~~σομενος", "~~σθησομενος";
    participle_aor: _, "~~σᾰμενος", _;
    ),
    // Ἀποκωλῡω: 5 words (ἀποδῠω, ἀποκωλῡω, προκῐνδῡνευω, ὑποφερω, ἀποθραυω). Like Ξαινω, except in 227 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀποκωλῡω", 5, "~ο~ω", like Ξαινω;
    stem aorist_ind: "~ε~σ- like ἐλπω";
    stem aorist_ind_pass: "~ε~θη- like ἐλπω";
    ind_act_1: "~ο~ω", _, "~ο~ομεν";
    ind_act_2: "~ο~εις", "~ο~ετον", "~ο~ετε";
    ind_act_3: "~ο~ει", _, "~ο~ουσῐ";
    ind_mid_1: "~ο~ομαι", _, "~ο~ομεθᾰ";
    ind_mid_2: "~ο~ῃ", "~ο~εσθον", "~ο~εσθε";
    ind_mid_3: "~ο~εται", _, "~ο~ονται";
    subj_act_1: "~ο~ω", _, "~ο~ωμεν";
    subj_act_2: "~ο~ῃς", "~ο~ητον", "~ο~ητε";
    subj_act_3: "~ο~ῃ", _, "~ο~ωσῐ";
    subj_mid_1: "~ο~ωμαι", _, "~ο~ωμεθᾰ";
    subj_mid_2: _, "~ο~ησθον", "~ο~ησθε";
    subj_mid_3: "~ο~ηται", _, "~ο~ωνται";
    opt_act_1: "~ο~οιμῐ", _, "~ο~οιμεν";
    opt_act_2: "~ο~οις", "~ο~οιτον", "~ο~οιτε";
    opt_act_3: "~ο~οι", "~ο~οιτην", "~ο~οιεν";
    opt_mid_1: "~ο~οιμην", _, "~ο~οιμεθᾰ";
    opt_mid_2: "~ο~οιο", "~ο~οισθον", "~ο~οισθε";
    opt_mid_3: "~ο~οιτο", "~ο~οισθην", "~ο~οιντο";
    imp_act_2: "~ο~ε", _, _;
    imp_act_3: "~ο~ετω", "~ο~ετων", "~ο~οντων";
    imp_mid_2: "~ο~ου", _, _;
    imp_mid_3: "~ο~εσθω", "~ο~εσθων", _;
    imperfect_act_1: "~ε~ον", _, "~ε~ομεν";
    imperfect_act_2: "~ε~ες", "~ε~ετον", "~ε~ετε";
    imperfect_act_3: "~ε~ε", "~ε~ετην", "~ε~ον";
    imperfect_mid_1: "~ε~ομην", _, "~ε~ομεθᾰ";
    imperfect_mid_2: "~ε~ου", "~ε~εσθον", "~ε~εσθε";
    imperfect_mid_3: "~ε~ετο", "~ε~εσθην", "~ε~οντο";
    future_ind_act_1: "~ο~σω", _, "~ο~σομεν";
    future_ind_act_2: "~ο~σεις", "~ο~σετον", "~ο~σετε";
    future_ind_act_3: "~ο~σει", _, "~ο~σουσῐ";
    future_ind_mid_1: "~ο~σομαι", _, "~ο~σομεθᾰ";
    future_ind_mid_2: "~ο~σῃ", "~ο~σεσθον", "~ο~σεσθε";
    future_ind_mid_3: "~ο~σεται", _, "~ο~σονται";
    future_ind_pass_1: "~ο~θησομαι", _, "~ο~θησομεθᾰ";
    future_ind_pass_2: "~ο~θησῃ", "~ο~θησεσθον", "~ο~θησεσθε";
    future_ind_pass_3: "~ο~θησεται", _, "~ο~θησονται";
    future_opt_act_1: "~ο~σοιμῐ", _, "~ο~σοιμεν";
    future_opt_act_2: "~ο~σοις", "~ο~σοιτον", "~ο~σοιτε";
    future_opt_act_3: "~ο~σοι", "~ο~σοιτην", "~ο~σοιεν";
    future_opt_mid_1: "~ο~σοιμην", _, "~ο~σοιμεθᾰ";
    future_opt_mid_2: "~ο~σοιο", "~ο~σοισθον", "~ο~σοισθε";
    future_opt_mid_3: "~ο~σοιτο", "~ο~σοισθην", "~ο~σοιντο";
    future_opt_pass_1: "~ο~θησοιμην", _, "~ο~θησοιμεθᾰ";
    future_opt_pass_2: "~ο~θησοιο", "~ο~θησοισθον", "~ο~θησοισθε";
    future_opt_pass_3: "~ο~θησοιτο", "~ο~θησοισθην", "~ο~θησοιντο";
    aorist_subj_act_1: "~ο~σω", _, "~ο~σωμεν";
    aorist_subj_act_2: "~ο~σῃς", "~ο~σητον", "~ο~σητε";
    aorist_subj_act_3: "~ο~σῃ", _, "~ο~σωσῐ";
    aorist_subj_mid_1: "~ο~σωμαι", _, "~ο~σωμεθᾰ";
    aorist_subj_mid_2: _, "~ο~σησθον", "~ο~σησθε";
    aorist_subj_mid_3: "~ο~σηται", _, "~ο~σωνται";
    aorist_subj_pass_1: "~ο~θω", _, "~ο~θωμεν";
    aorist_subj_pass_2: "~ο~θῃς", "~ο~θητον", "~ο~θητε";
    aorist_subj_pass_3: "~ο~θῃ", _, "~ο~θωσῐ";
    aorist_opt_act_1: "~ο~σαιμῐ", _, "~ο~σαιμεν";
    aorist_opt_act_2: "~ο~σειᾰς", "~ο~σαιτον", "~ο~σαιτε";
    aorist_opt_act_3: "~ο~σειε", "~ο~σαιτην", "~ο~σειᾰν";
    aorist_opt_mid_1: "~ο~σαιμην", _, "~ο~σαιμεθᾰ";
    aorist_opt_mid_2: "~ο~σαιο", "~ο~σαισθον", "~ο~σαισθε";
    aorist_opt_mid_3: "~ο~σαιτο", "~ο~σαισθην", "~ο~σαιντο";
    aorist_opt_pass_1: "~ο~θειην", _, "~ο~θεῑμεν";
    aorist_opt_pass_2: "~ο~θειης", "~ο~θεῑτον", "~ο~θεῑτε";
    aorist_opt_pass_3: "~ο~θειη", "~ο~θειτην", "~ο~θεῑεν";
    aorist_imp_act_2: "~ο~σον", "~ο~σᾰτον", "~ο~σᾰτε";
    aorist_imp_act_3: "~ο~σᾰτω", "~ο~σᾰτων", "~ο~σᾰντων";
    aorist_imp_mid_2: "~ο~σαι", "~ο~σᾰσθον", "~ο~σᾰσθε";
    aorist_imp_mid_3: "~ο~σᾰσθω", "~ο~σᾰσθων", _;
    aorist_imp_pass_2: "~ο~θητῐ", _, _;
    aorist_imp_pass_3: "~ο~θητω", "~ο~θητων", "~ο~θεντων";
    perfect_ind_act_1: "~ο%ε~κᾰ", _, "~ο%ε~κᾰμεν";
    perfect_ind_act_2: "~ο%ε~κᾰς", "~ο%ε~κᾰτον", "~ο%ε~κᾰτε";
    perfect_ind_act_3: "~ο%ε~κε", _, "~ο%ε~κᾱσῐ";
    perfect_ind_mid_1: "~ο%ε~μαι", _, "~ο%ε~μεθᾰ";
    perfect_ind_mid_2: "~ο%ε~σαι", "~ο%ε~σθον", "~ο%ε~σθε";
    perfect_ind_mid_3: "~ο%ε~ται", _, "~ο%ε~νται";
    perfect_subj_act_1: "~ο%ε~κω", _, "~ο%ε~κωμεν";
    perfect_subj_act_2: "~ο%ε~κῃς", "~ο%ε~κητον", "~ο%ε~κητε";
    perfect_subj_act_3: "~ο%ε~κῃ", _, "~ο%ε~κωσῐ";
    perfect_opt_act_1: "~ο%ε~κοιμῐ", _, "~ο%ε~κοιμεν";
    perfect_opt_act_2: "~ο%ε~κοις", "~ο%ε~κοιτον", "~ο%ε~κοιτε";
    perfect_opt_act_3: "~ο%ε~κοι", "~ο%ε~κοιτην", "~ο%ε~κοιεν";
    perfect_imp_act_2: _, "~ο%ε~κετον", "~ο%ε~κετε";
    perfect_imp_act_3: "~ο%ε~κετω", "~ο%ε~κετων", "~ο%ε~κοντων";
    perfect_imp_mid_2: "~ο%ε~σο", _, _;
    perfect_imp_mid_3: "~ο%ε~σθω", "~ο%ε~σθων", _;
    infinitive_pres: "~ο~ειν", "~ο~εσθαι", _;
    infinitive_fut: "~ο~σειν", "~ο~σεσθαι", "~ο~θησεσθαι";
    infinitive_aor: "~ο~σαι", "~ο~σᾰσθαι", "~ο~θηναι";
    infinitive_perf: "~ο%ε~κεναι", "~ο%ε~σθαι", _;
    participle_pres: "~ο~ων", "~ο~ομενος", _;
    participle_fut: "~ο~σων", "~ο~σομενος", "~ο~θησομενος";
    participle_aor: "~ο~σᾱς", "~ο~σᾰμενος", "~ο~θεις";
    participle_perf: "~ο%ε~κως", "~ο%ε~μενος", _;
    ),
    // Σκηπτω: 5 words (ῥᾰπτω, ῥῑπτω, σκᾰπτω, ρρῑπτω, ρρᾰπτω). Like Θρῠπτω, except in 76 cells: future_ind_pass_2 sg, aorist_subj_act_3 pl, aorist_subj_pass_1 sg, aorist_subj_pass_1 pl, aorist_subj_pass_2 sg, aorist_subj_pass_2 du, ….
    conjugation!("σκηπτω", 5, "~πτω", like Θρῠπτω;
    stem aorist_ind: "~ψ- like ἐχω";
    stem aorist_ind_pass: "~φη- like ἐχω";
    stem pluperfect_act: "~φε- like κᾰλεω";
    future_ind_pass_2: "~φησῃ", _, _;
    aorist_subj_act_3: _, _, "~ψωσῐ";
    aorist_subj_pass_1: "~φω", _, "~φωμεν";
    aorist_subj_pass_2: "~φῃς", "~φητον", "~φητε";
    aorist_subj_pass_3: "~φῃ", _, "~φωσῐ";
    aorist_opt_act_2: "~ψειᾰς", _, _;
    aorist_opt_act_3: "~ψειε", _, "~ψειᾰν";
    aorist_opt_pass_1: "~φειην", _, "~φεῑμεν";
    aorist_opt_pass_2: "~φειης", "~φεῑτον", "~φεῑτε";
    aorist_opt_pass_3: "~φειη", "~φειτην", "~φεῑεν";
    aorist_imp_pass_2: "~φηθῐ", _, _;
    aorist_imp_pass_3: "~φητω", "~φητων", "~φεντων";
    perfect_ind_act_1: "ἐ~φᾰ", _, "ἐ~φᾰμεν";
    perfect_ind_act_2: "ἐ~φᾰς", "ἐ~φᾰτον", "ἐ~φᾰτε";
    perfect_ind_act_3: "ἐ~φε|ἐ~φεν", _, "ἐ~φᾱσῐ|ἐ~φᾱσῐν";
    perfect_ind_mid_1: "ἐ~μμαι", _, "ἐ~μμεθᾰ";
    perfect_ind_mid_2: "ἐ~ψαι", "ἐ~φθον", "ἐ~φθε";
    perfect_ind_mid_3: "ἐ~πται", _, "ἐ~φᾰται";
    perfect_subj_act_1: "ἐ~φω", _, "ἐ~φωμεν";
    perfect_subj_act_2: "ἐ~φῃς", "ἐ~φητον", "ἐ~φητε";
    perfect_subj_act_3: "ἐ~φῃ", _, "ἐ~φωσῐ|ἐ~φωσῐν";
    perfect_opt_act_1: "ἐ~φοιμῐ|ἐ~φοιην", _, "ἐ~φοιμεν";
    perfect_opt_act_2: "ἐ~φοις|ἐ~φοιης", "ἐ~φοιτον", "ἐ~φοιτε";
    perfect_opt_act_3: "ἐ~φοι|ἐ~φοιη", "ἐ~φοιτην", "ἐ~φοιεν";
    perfect_imp_act_2: "ἐ~φε", "ἐ~φετον", "ἐ~φετε";
    perfect_imp_act_3: "ἐ~φετω", "ἐ~φετων", "ἐ~φοντων";
    perfect_imp_mid_2: "ἐ~ψο", _, _;
    perfect_imp_mid_3: "ἐ~φθω", "ἐ~φθων", _;
    pluperfect_mid_1: "ἐ~μμην", _, "ἐ~μμεθᾰ";
    pluperfect_mid_2: "ἐ~ψο", "ἐ~φθον", "ἐ~φθε";
    pluperfect_mid_3: "ἐ~πτο", "ἐ~φθην", "ἐ~φᾰτο";
    infinitive_aor: _, _, "~φηναι";
    infinitive_perf: "ἐ~φεναι", "ἐ~φθαι", _;
    participle_aor: _, _, "~φεις";
    participle_perf: "ἐ~φως", "ἐ~μμενος", _;
    ),
    // Ἀναιρεω: 5 words (ἀναιρεω, ἀφαιρεω, ἐξαιρεω, δῐαιρεω, ὑφαιρεω). Like Ληγω, except in 228 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀναιρεω", 5, "~αιρεω", like Ληγω;
    stem aorist_ind: "~- like ἀναιρεω";
    stem aorist_ind_pass: "~ῃρεθη- like ἐλπω";
    ind_act_1: "~αιρω", _, "~αιροῡμεν";
    ind_act_2: "~αιρεῑς", "~αιρεῑτον", "~αιρεῑτε";
    ind_act_3: "~αιρεῑ", _, "~αιροῡσῐ|~αιροῡσῐν";
    ind_mid_1: "~αιροῡμαι", _, "~αιρουμεθᾰ";
    ind_mid_2: "~αιρεῑ|~αιρῃ", "~αιρεῑσθον", "~αιρεῑσθε";
    ind_mid_3: "~αιρεῑται", _, "~αιροῡνται";
    subj_act_1: "~αιρω", _, "~αιρωμεν";
    subj_act_2: "~αιρῃς", "~αιρητον", "~αιρητε";
    subj_act_3: "~αιρῃ", _, "~αιρωσῐ|~αιρωσῐν";
    subj_mid_1: "~αιρωμαι", _, "~αιρωμεθᾰ";
    subj_mid_2: _, "~αιρησθον", "~αιρησθε";
    subj_mid_3: "~αιρηται", _, "~αιρωνται";
    opt_act_1: "~αιροιην|~αιροῑμῐ", _, "~αιροῑμεν|~αιροιημεν";
    opt_act_2: "~αιροιης|~αιροῑς", "~αιροῑτον|~αιροιητον", "~αιροῑτε|~αιροιητε";
    opt_act_3: "~αιροιη|~αιροῑ", "~αιροιτην|~αιροιητην", "~αιροῑεν|~αιροιησᾰν";
    opt_mid_1: "~αιροιμην", _, "~αιροιμεθᾰ";
    opt_mid_2: "~αιροῑο", "~αιροῑσθον", "~αιροῑσθε";
    opt_mid_3: "~αιροῑτο", "~αιροισθην", "~αιροῑντο";
    imp_act_2: "~αιρει", _, _;
    imp_act_3: "~αιρειτω", "~αιρειτων", "~αιρουντων";
    imp_mid_2: "~αιροῡ", _, _;
    imp_mid_3: "~αιρεισθω", "~αιρεισθων", _;
    imperfect_act_1: "~ῃρουν", _, "~ῃροῡμεν";
    imperfect_act_2: "~ῃρεις", "~ῃρεῑτον", "~ῃρεῑτε";
    imperfect_act_3: "~ῃρει", "~ῃρειτην", "~ῃρουν";
    imperfect_mid_1: "~ῃρουμην", _, "~ῃρουμεθᾰ";
    imperfect_mid_2: "~ῃροῡ", "~ῃρεῑσθον", "~ῃρεῑσθε";
    imperfect_mid_3: "~ῃρεῑτο", "~ῃρεισθην", "~ῃροῡντο";
    future_ind_act_1: "~αιρησω", _, "~αιρησομεν";
    future_ind_act_2: "~αιρησεις", "~αιρησετον", "~αιρησετε";
    future_ind_act_3: "~αιρησει", _, "~αιρησουσῐ|~αιρησουσῐν";
    future_ind_mid_1: "~αιρησομαι", _, "~αιρησομεθᾰ";
    future_ind_mid_2: "~αιρησῃ|~αιρησει", "~αιρησεσθον", "~αιρησεσθε";
    future_ind_mid_3: "~αιρησεται", _, "~αιρησονται";
    future_ind_pass_1: "~αιρεθησομαι", _, "~αιρεθησομεθᾰ";
    future_ind_pass_2: "~αιρεθησῃ|~αιρεθησει", "~αιρεθησεσθον", "~αιρεθησεσθε";
    future_ind_pass_3: "~αιρεθησεται", _, "~αιρεθησονται";
    future_opt_act_1: "~αιρησοιμῐ", _, "~αιρησοιμεν";
    future_opt_act_2: "~αιρησοις", "~αιρησοιτον", "~αιρησοιτε";
    future_opt_act_3: "~αιρησοι", "~αιρησοιτην", "~αιρησοιεν";
    future_opt_mid_1: "~αιρησοιμην", _, "~αιρησοιμεθᾰ";
    future_opt_mid_2: "~αιρησοιο", "~αιρησοισθον", "~αιρησοισθε";
    future_opt_mid_3: "~αιρησοιτο", "~αιρησοισθην", "~αιρησοιντο";
    future_opt_pass_1: "~αιρεθησοιμην", _, "~αιρεθησοιμεθᾰ";
    future_opt_pass_2: "~αιρεθησοιο", "~αιρεθησοισθον", "~αιρεθησοισθε";
    future_opt_pass_3: "~αιρεθησοιτο", "~αιρεθησοισθην", "~αιρεθησοιντο";
    aorist_subj_act_1: "~ελω|~αιρησω", _, "~ελωμεν|~αιρησωμεν";
    aorist_subj_act_2: "~ελῃς|~αιρησῃς", "~ελητον|~αιρησητον", "~ελητε|~αιρησητε";
    aorist_subj_act_3: "~ελῃ|~αιρησῃ", _, "~ελωσῐ|~ελωσῐν|~αιρησωσῐ|~αιρησωσῐν";
    aorist_subj_mid_1: "~ελωμαι|~αιρησωμαι", _, "~ελωμεθᾰ|~αιρησωμεθᾰ";
    aorist_subj_mid_2: _, "~ελησθον|~αιρησησθον", "~ελησθε|~αιρησησθε";
    aorist_subj_mid_3: "~εληται|~αιρησηται", _, "~ελωνται|~αιρησωνται";
    aorist_subj_pass_1: "~αιρεθω", _, "~αιρεθωμεν";
    aorist_subj_pass_2: "~αιρεθῃς", "~αιρεθητον", "~αιρεθητε";
    aorist_subj_pass_3: "~αιρεθῃ", _, "~αιρεθωσῐ|~αιρεθωσῐν";
    aorist_opt_act_1: "~ελοιμῐ|~αιρησαιμῐ", _, "~ελοιμεν|~αιρησαιμεν";
    aorist_opt_act_2: "~ελοις|~αιρησειᾰς|~αιρησαις", "~ελοιτον|~αιρησαιτον", "~ελοιτε|~αιρησαιτε";
    aorist_opt_act_3: "~ελοι|~αιρησειε|~αιρησειεν|~αιρησαι", "~ελοιτην|~αιρησαιτην", "~ελοιεν|~αιρησειᾰν|~αιρησαιεν";
    aorist_opt_mid_1: "~ελοιμην|~αιρησαιμην", _, "~ελοιμεθᾰ|~αιρησαιμεθᾰ";
    aorist_opt_mid_2: "~ελοιο|~αιρησαιο", "~ελοισθον|~αιρησαισθον", "~ελοισθε|~αιρησαισθε";
    aorist_opt_mid_3: "~ελοιτο|~αιρησαιτο", "~ελοισθην|~αιρησαισθην", "~ελοιντο|~αιρησαιντο";
    aorist_opt_pass_1: "~αιρεθειην", _, "~αιρεθεῑμεν|~αιρεθειημεν";
    aorist_opt_pass_2: "~αιρεθειης", "~αιρεθεῑτον|~αιρεθειητον", "~αιρεθεῑτε|~αιρεθειητε";
    aorist_opt_pass_3: "~αιρεθειη", "~αιρεθειτην|~αιρεθειητην", "~αιρεθεῑεν|~αιρεθειησᾰν";
    aorist_imp_act_2: "~ελε|~αιρησον", "~ελετον|~αιρησᾰτον", "~ελετε|~αιρησᾰτε";
    aorist_imp_act_3: "~ελετω|~αιρησᾰτω", "~ελετων|~αιρησᾰτων", "~ελοντων|~αιρησᾰντων";
    aorist_imp_mid_2: "~ελοῡ|~αιρησαι", "~ελεσθον|~αιρησᾰσθον", "~ελεσθε|~αιρησᾰσθε";
    aorist_imp_mid_3: "~ελεσθω|~αιρησᾰσθω", "~ελεσθων|~αιρησᾰσθων", _;
    aorist_imp_pass_2: "~αιρεθητῐ", _, _;
    aorist_imp_pass_3: "~αιρεθητω", "~αιρεθητων", "~αιρεθεντων";
    perfect_ind_act_1: "~ῃρηκᾰ", _, "~ῃρηκᾰμεν";
    perfect_ind_act_2: "~ῃρηκᾰς", "~ῃρηκᾰτον", "~ῃρηκᾰτε";
    perfect_ind_act_3: "~ῃρηκε|~ῃρηκεν", _, "~ῃρηκᾱσῐ|~ῃρηκᾱσῐν";
    perfect_ind_mid_1: "~ῃρημαι", _, "~ῃρημεθᾰ";
    perfect_ind_mid_2: "~ῃρησαι", "~ῃρησθον", "~ῃρησθε";
    perfect_ind_mid_3: "~ῃρηται", _, "~ῃρηνται";
    perfect_subj_act_1: "~ῃρηκω", _, "~ῃρηκωμεν";
    perfect_subj_act_2: "~ῃρηκῃς", "~ῃρηκητον", "~ῃρηκητε";
    perfect_subj_act_3: "~ῃρηκῃ", _, "~ῃρηκωσῐ|~ῃρηκωσῐν";
    perfect_opt_act_1: "~ῃρηκοιμῐ|~ῃρηκοιην", _, "~ῃρηκοιμεν";
    perfect_opt_act_2: "~ῃρηκοις|~ῃρηκοιης", "~ῃρηκοιτον", "~ῃρηκοιτε";
    perfect_opt_act_3: "~ῃρηκοι|~ῃρηκοιη", "~ῃρηκοιτην", "~ῃρηκοιεν";
    perfect_imp_act_2: "~ῃρηκε", "~ῃρηκετον", "~ῃρηκετε";
    perfect_imp_act_3: "~ῃρηκετω", "~ῃρηκετων", "~ῃρηκοντων";
    perfect_imp_mid_2: "~ῃρησο", _, _;
    perfect_imp_mid_3: "~ῃρησθω", "~ῃρησθων", _;
    infinitive_pres: "~αιρεῑν", "~αιρεῑσθαι", _;
    infinitive_fut: "~αιρησειν", "~αιρησεσθαι", "~αιρεθησεσθαι";
    infinitive_aor: "~ελεῑν|~αιρησαι", "~ελεσθαι|~αιρησᾰσθαι", "~αιρεθηναι";
    infinitive_perf: "~ῃρηκεναι", "~ῃρησθαι", _;
    participle_pres: "~αιρων", "~αιρουμενος", _;
    participle_fut: "~αιρησων", "~αιρησομενος", "~αιρεθησομενος";
    participle_aor: "~ελων|~αιρησᾱς", "~ελομενος|~αιρησᾰμενος", "~αιρεθεις";
    participle_perf: "~ῃρηκως", "~ῃρημενος", _;
    ),
    // Ῥαινω: 5 words (ῥαινω, κραινω, ὑγῐαινω, ὑδραινω, τετραινω). Like Μωραινω, except in 90 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imp_act_2 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("ῥαινω", 5, "~αινω", like Μωραινω as PERFECT_AS_FUTURE;
    stem imperfect: "~αιν- like ῥεω";
    stem aorist_ind_pass: "~ᾱνθη- like ἐχω";
    ind_act_3: _, _, "~αινουσῐ";
    ind_mid_2: "~αινῃ", _, _;
    subj_act_3: _, _, "~αινωσῐ";
    imp_act_2: "~αῑνε", _, _;
    future_ind_act_1: "~ᾰνω", _, "~ᾰνοῡμεν";
    future_ind_act_2: "~ᾰνεῑς", "~ᾰνεῑτον", "~ᾰνεῑτε";
    future_ind_act_3: "~ᾰνεῑ", _, "~ᾰνοῡσῐ";
    future_ind_mid_1: "~ᾰνοῡμαι", _, "~ᾰνουμεθᾰ";
    future_ind_mid_2: "~ᾰνεῑ", "~ᾰνεῑσθον", "~ᾰνεῑσθε";
    future_ind_mid_3: "~ᾰνεῑται", _, "~ᾰνοῡνται";
    future_ind_pass_1: "~ανθησησομαι", _, "~ανθησησομεθᾰ";
    future_ind_pass_2: "~ανθησησῃ", "~ανθησησεσθον", "~ανθησησεσθε";
    future_ind_pass_3: "~ανθησησεται", _, "~ανθησησονται";
    future_opt_act_1: "~ᾰνοιην", _, "~ᾰνοῑμεν";
    future_opt_act_2: "~ᾰνοιης", "~ᾰνοῑτον", "~ᾰνοῑτε";
    future_opt_act_3: "~ᾰνοιη", "~ᾰνοιτην", "~ᾰνοῑεν";
    future_opt_mid_1: "~ᾰνοιμην", _, "~ᾰνοιμεθᾰ";
    future_opt_mid_2: "~ᾰνοῑο", "~ᾰνοῑσθον", "~ᾰνοῑσθε";
    future_opt_mid_3: "~ᾰνοῑτο", "~ᾰνοισθην", "~ᾰνοῑντο";
    future_opt_pass_1: "~ανθησησοιμην", _, "~ανθησησοιμεθᾰ";
    future_opt_pass_2: "~ανθησησοιο", "~ανθησησοισθον", "~ανθησησοισθε";
    future_opt_pass_3: "~ανθησησοιτο", "~ανθησησοισθην", "~ανθησησοιντο";
    aorist_subj_pass_1: "~ᾱνθω", _, "~ᾱνθωμεν";
    aorist_subj_pass_2: "~ᾱνθῃς", "~ᾱνθητον", "~ᾱνθητε";
    aorist_subj_pass_3: "~ᾱνθῃ", _, "~ᾱνθωσῐ";
    aorist_opt_pass_1: "~ᾱνθειην", _, "~ᾱνθεῑμεν";
    aorist_opt_pass_2: "~ᾱνθειης", "~ᾱνθεῑτον", "~ᾱνθεῑτε";
    aorist_opt_pass_3: "~ᾱνθειη", "~ᾱνθειτην", "~ᾱνθεῑεν";
    aorist_imp_pass_2: "~ᾱνθητῐ", _, _;
    aorist_imp_pass_3: "~ᾱνθητω", "~ᾱνθητων", "~ᾱνθεντων";
    perfect_ind_mid_1: "%ε~αμαι", _, "%ε~αμεθᾰ";
    perfect_ind_mid_2: "%ε~ασαι", "%ε~ασθον", "%ε~ασθε";
    perfect_ind_mid_3: "%ε~αται", _, "%ε~ανται";
    perfect_imp_mid_2: "%ε~ασο", _, _;
    perfect_imp_mid_3: "%ε~ασθω", "%ε~ασθων", _;
    infinitive_fut: "~ᾰνεῑν", "~ᾰνεῑσθαι", "~ανθησησεσθαι";
    infinitive_aor: _, _, "~ᾱνθηναι";
    infinitive_perf: derived, "%ε~ᾱσθαι", _;
    participle_fut: "~ᾰνων", "~ᾰνουμενος", "~ανθησησομενος";
    participle_aor: _, _, "~ᾱνθεις";
    participle_perf: derived, "%ε~αμενος", _;
    ),
    // Ἐλαυνω: 5 words (ἐσθῐω, ἐλαυνω, ἐρειδω, ἐχθω, εκπλεω). Like Μῑγνῡμῐ, except in 70 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐλαυνω", 5, "ἐ~ω", like Μῑγνῡμῐ;
    ind_act_1: "ἐ~ω", _, "ἐ~ομεν";
    ind_act_2: "ἐ~εις", "ἐ~ετον", "ἐ~ετε";
    ind_act_3: "ἐ~ει", _, "ἐ~ουσῐ";
    ind_mid_1: "ἐ~ομαι", _, "ἐ~ομεθᾰ";
    ind_mid_2: "ἐ~ῃ", "ἐ~εσθον", "ἐ~εσθε";
    ind_mid_3: "ἐ~εται", _, "ἐ~ονται";
    subj_act_1: "ἐ~ω", _, "ἐ~ωμεν";
    subj_act_2: "ἐ~ῃς", "ἐ~ητον", "ἐ~ητε";
    subj_act_3: "ἐ~ῃ", _, "ἐ~ωσῐ";
    subj_mid_1: "ἐ~ωμαι", _, "ἐ~ωμεθᾰ";
    subj_mid_2: _, "ἐ~ησθον", "ἐ~ησθε";
    subj_mid_3: "ἐ~ηται", _, "ἐ~ωνται";
    opt_act_1: "ἐ~οιμῐ", _, "ἐ~οιμεν";
    opt_act_2: "ἐ~οις", "ἐ~οιτον", "ἐ~οιτε";
    opt_act_3: "ἐ~οι", "ἐ~οιτην", "ἐ~οιεν";
    opt_mid_1: "ἐ~οιμην", _, "ἐ~οιμεθᾰ";
    opt_mid_2: "ἐ~οιο", "ἐ~οισθον", "ἐ~οισθε";
    opt_mid_3: "ἐ~οιτο", "ἐ~οισθην", "ἐ~οιντο";
    imp_act_2: "ἐ~ε", _, _;
    imp_act_3: "ἐ~ετω", "ἐ~ετων", "ἐ~οντων";
    imp_mid_2: "ἐ~ου", _, _;
    imp_mid_3: "ἐ~εσθω", "ἐ~εσθων", _;
    imperfect_act_1: "ἠ~ον", _, "ἠ~ομεν";
    imperfect_act_2: "ἠ~ες", "ἠ~ετον", "ἠ~ετε";
    imperfect_act_3: "ἠ~ε", "ἠ~ετην", "ἠ~ον";
    imperfect_mid_1: "ἠ~ομην", _, "ἠ~ομεθᾰ";
    imperfect_mid_2: "ἠ~ου", "ἠ~εσθον", "ἠ~εσθε";
    imperfect_mid_3: "ἠ~ετο", "ἠ~εσθην", "ἠ~οντο";
    infinitive_pres: "ἐ~ειν", "ἐ~εσθαι", _;
    participle_pres: "ἐ~ων", "ἐ~ομενος", _;
    ),
    // Ἐξᾰκολουθεω: 5 words (πᾰρᾰμελεω, σῠνᾰδῐκεω, ξῠνᾰδῐκεω, ἀπᾰσχολεω, ἐξᾰκολουθεω). Like Λᾰγοθηρεω, except in 66 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("ἐξᾰκολουθεω", 5, "~ᾰ~εω", like Λᾰγοθηρεω;
    stem aorist_ind: "~η~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~η~ηθη- like ἐλπω";
    stem pluperfect_act: "~η~ηκ- like εὐλογεω";
    imperfect_act_1: "~η~ουν", _, "~η~οῡμεν";
    imperfect_act_2: "~η~εις", "~η~εῑτον", "~η~εῑτε";
    imperfect_act_3: "~η~ει", "~η~ειτην", "~η~ουν";
    imperfect_mid_1: "~η~ουμην", _, "~η~ουμεθᾰ";
    imperfect_mid_2: "~η~οῡ", "~η~εῑσθον", "~η~εῑσθε";
    imperfect_mid_3: "~η~εῑτο", "~η~εισθην", "~η~οῡντο";
    perfect_ind_act_1: "~η~ηκᾰ", _, "~η~ηκᾰμεν";
    perfect_ind_act_2: "~η~ηκᾰς", "~η~ηκᾰτον", "~η~ηκᾰτε";
    perfect_ind_act_3: "~η~ηκε|~η~ηκεν", _, "~η~ηκᾱσῐ|~η~ηκᾱσῐν";
    perfect_ind_mid_1: "~η~ημαι", _, "~η~ημεθᾰ";
    perfect_ind_mid_2: "~η~ησαι", "~η~ησθον", "~η~ησθε";
    perfect_ind_mid_3: "~η~ηται", _, "~η~ηνται";
    perfect_subj_act_1: "~η~ηκω", _, "~η~ηκωμεν";
    perfect_subj_act_2: "~η~ηκῃς", "~η~ηκητον", "~η~ηκητε";
    perfect_subj_act_3: "~η~ηκῃ", _, "~η~ηκωσῐ|~η~ηκωσῐν";
    perfect_opt_act_1: "~η~ηκοιμῐ|~η~ηκοιην", _, "~η~ηκοιμεν";
    perfect_opt_act_2: "~η~ηκοις|~η~ηκοιης", "~η~ηκοιτον", "~η~ηκοιτε";
    perfect_opt_act_3: "~η~ηκοι|~η~ηκοιη", "~η~ηκοιτην", "~η~ηκοιεν";
    perfect_imp_act_2: "~η~ηκε", "~η~ηκετον", "~η~ηκετε";
    perfect_imp_act_3: "~η~ηκετω", "~η~ηκετων", "~η~ηκοντων";
    perfect_imp_mid_2: "~η~ησο", _, _;
    perfect_imp_mid_3: "~η~ησθω", "~η~ησθων", _;
    pluperfect_mid_1: "~η~ημην", _, "~η~ημεθᾰ";
    pluperfect_mid_2: "~η~ησο", "~η~ησθον", "~η~ησθε";
    pluperfect_mid_3: "~η~ητο", "~η~ησθην", "~η~ηντο";
    infinitive_perf: "~η~ηκεναι", "~η~ησθαι", _;
    participle_perf: "~η~ηκως", "~η~ημενος", _;
    ),
    // Προκᾰλεω: 5 words (προσκᾰλεω, σῠγκᾰλεω, προκᾰλεω, ἐγκᾰλεω, ἐκκᾰλεω). Like Ζευγνῡμῐ, except in 227 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("προκᾰλεω", 5, "~~κᾰ~εω", like Ζευγνῡμῐ;
    stem aorist_ind: "~ξε~ᾰ~εσ- like ἐλπω";
    stem aorist_ind_pass: "~ξε~~ηθη- like ἐλπω";
    ind_act_1: "~~κᾰ~ω", _, "~~κᾰ~οῡμεν";
    ind_act_2: "~~κᾰ~εῑς", "~~κᾰ~εῑτον", "~~κᾰ~εῑτε";
    ind_act_3: "~~κᾰ~εῑ", _, "~~κᾰ~οῡσῐ";
    ind_mid_1: "~~κᾰ~οῡμαι", _, "~~κᾰ~ουμεθᾰ";
    ind_mid_2: "~~κᾰ~εῑ", "~~κᾰ~εῑσθον", "~~κᾰ~εῑσθε";
    ind_mid_3: "~~κᾰ~εῑται", _, "~~κᾰ~οῡνται";
    subj_act_1: "~~κᾰ~ω", _, "~~κᾰ~ωμεν";
    subj_act_2: "~~κᾰ~ῃς", "~~κᾰ~ητον", "~~κᾰ~ητε";
    subj_act_3: "~~κᾰ~ῃ", _, "~~κᾰ~ωσῐ";
    subj_mid_1: "~~κᾰ~ωμαι", _, "~~κᾰ~ωμεθᾰ";
    subj_mid_2: _, "~~κᾰ~ησθον", "~~κᾰ~ησθε";
    subj_mid_3: "~~κᾰ~ηται", _, "~~κᾰ~ωνται";
    opt_act_1: "~~κᾰ~οιην", _, "~~κᾰ~οῑμεν";
    opt_act_2: "~~κᾰ~οιης", "~~κᾰ~οῑτον", "~~κᾰ~οῑτε";
    opt_act_3: "~~κᾰ~οιη", "~~κᾰ~οιτην", "~~κᾰ~οῑεν";
    opt_mid_1: "~~κᾰ~οιμην", _, "~~κᾰ~οιμεθᾰ";
    opt_mid_2: "~~κᾰ~οῑο", "~~κᾰ~οῑσθον", "~~κᾰ~οῑσθε";
    opt_mid_3: "~~κᾰ~οῑτο", "~~κᾰ~οισθην", "~~κᾰ~οῑντο";
    imp_act_2: "~~κᾰ~ει", _, _;
    imp_act_3: "~~κᾰ~ειτω", "~~κᾰ~ειτων", "~~κᾰ~ουντων";
    imp_mid_2: "~~κᾰ~οῡ", _, _;
    imp_mid_3: "~~κᾰ~εισθω", "~~κᾰ~εισθων", _;
    imperfect_act_1: "~ξε~ᾰ~ουν", _, "~ξε~ᾰ~οῡμεν";
    imperfect_act_2: "~ξε~ᾰ~εις", "~ξε~ᾰ~εῑτον", "~ξε~ᾰ~εῑτε";
    imperfect_act_3: "~ξε~ᾰ~ει", "~ξε~ᾰ~ειτην", "~ξε~ᾰ~ουν";
    imperfect_mid_1: "~ξε~ᾰ~ουμην", _, "~ξε~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "~ξε~ᾰ~οῡ", "~ξε~ᾰ~εῑσθον", "~ξε~ᾰ~εῑσθε";
    imperfect_mid_3: "~ξε~ᾰ~εῑτο", "~ξε~ᾰ~εισθην", "~ξε~ᾰ~οῡντο";
    future_ind_act_1: "~~κᾰ~εσω", _, "~~κᾰ~εσομεν";
    future_ind_act_2: "~~κᾰ~εσεις", "~~κᾰ~εσετον", "~~κᾰ~εσετε";
    future_ind_act_3: "~~κᾰ~εσει", _, "~~κᾰ~εσουσῐ";
    future_ind_mid_1: "~~κᾰ~εσομαι", _, "~~κᾰ~εσομεθᾰ";
    future_ind_mid_2: "~~κᾰ~εσῃ", "~~κᾰ~εσεσθον", "~~κᾰ~εσεσθε";
    future_ind_mid_3: "~~κᾰ~εσεται", _, "~~κᾰ~εσονται";
    future_ind_pass_1: "~~κ~ηθησομαι", _, "~~κ~ηθησομεθᾰ";
    future_ind_pass_2: "~~κ~ηθησῃ", "~~κ~ηθησεσθον", "~~κ~ηθησεσθε";
    future_ind_pass_3: "~~κ~ηθησεται", _, "~~κ~ηθησονται";
    future_opt_act_1: "~~κᾰ~εσοιμῐ", _, "~~κᾰ~εσοιμεν";
    future_opt_act_2: "~~κᾰ~εσοις", "~~κᾰ~εσοιτον", "~~κᾰ~εσοιτε";
    future_opt_act_3: "~~κᾰ~εσοι", "~~κᾰ~εσοιτην", "~~κᾰ~εσοιεν";
    future_opt_mid_1: "~~κᾰ~εσοιμην", _, "~~κᾰ~εσοιμεθᾰ";
    future_opt_mid_2: "~~κᾰ~εσοιο", "~~κᾰ~εσοισθον", "~~κᾰ~εσοισθε";
    future_opt_mid_3: "~~κᾰ~εσοιτο", "~~κᾰ~εσοισθην", "~~κᾰ~εσοιντο";
    future_opt_pass_1: "~~κ~ηθησοιμην", _, "~~κ~ηθησοιμεθᾰ";
    future_opt_pass_2: "~~κ~ηθησοιο", "~~κ~ηθησοισθον", "~~κ~ηθησοισθε";
    future_opt_pass_3: "~~κ~ηθησοιτο", "~~κ~ηθησοισθην", "~~κ~ηθησοιντο";
    aorist_subj_act_1: "~~κᾰ~εσω", _, "~~κᾰ~εσωμεν";
    aorist_subj_act_2: "~~κᾰ~εσῃς", "~~κᾰ~εσητον", "~~κᾰ~εσητε";
    aorist_subj_act_3: "~~κᾰ~εσῃ", _, "~~κᾰ~εσωσῐ";
    aorist_subj_mid_1: "~~κᾰ~εσωμαι", _, "~~κᾰ~εσωμεθᾰ";
    aorist_subj_mid_2: _, "~~κᾰ~εσησθον", "~~κᾰ~εσησθε";
    aorist_subj_mid_3: "~~κᾰ~εσηται", _, "~~κᾰ~εσωνται";
    aorist_subj_pass_1: "~~κ~ηθω", _, "~~κ~ηθωμεν";
    aorist_subj_pass_2: "~~κ~ηθῃς", "~~κ~ηθητον", "~~κ~ηθητε";
    aorist_subj_pass_3: "~~κ~ηθῃ", _, "~~κ~ηθωσῐ";
    aorist_opt_act_1: "~~κᾰ~εσαιμῐ", _, "~~κᾰ~εσαιμεν";
    aorist_opt_act_2: "~~κᾰ~εσειᾰς", "~~κᾰ~εσαιτον", "~~κᾰ~εσαιτε";
    aorist_opt_act_3: "~~κᾰ~εσειε", "~~κᾰ~εσαιτην", "~~κᾰ~εσειᾰν";
    aorist_opt_mid_1: "~~κᾰ~εσαιμην", _, "~~κᾰ~εσαιμεθᾰ";
    aorist_opt_mid_2: "~~κᾰ~εσαιο", "~~κᾰ~εσαισθον", "~~κᾰ~εσαισθε";
    aorist_opt_mid_3: "~~κᾰ~εσαιτο", "~~κᾰ~εσαισθην", "~~κᾰ~εσαιντο";
    aorist_opt_pass_1: "~~κ~ηθειην", _, "~~κ~ηθεῑμεν";
    aorist_opt_pass_2: "~~κ~ηθειης", "~~κ~ηθεῑτον", "~~κ~ηθεῑτε";
    aorist_opt_pass_3: "~~κ~ηθειη", "~~κ~ηθειτην", "~~κ~ηθεῑεν";
    aorist_imp_act_2: "~~κᾰ~εσον", "~~κᾰ~εσᾰτον", "~~κᾰ~εσᾰτε";
    aorist_imp_act_3: "~~κᾰ~εσᾰτω", "~~κᾰ~εσᾰτων", "~~κᾰ~εσᾰντων";
    aorist_imp_mid_2: "~~κᾰ~εσαι", "~~κᾰ~εσᾰσθον", "~~κᾰ~εσᾰσθε";
    aorist_imp_mid_3: "~~κᾰ~εσᾰσθω", "~~κᾰ~εσᾰσθων", _;
    aorist_imp_pass_2: "~~κ~ηθητῐ", _, _;
    aorist_imp_pass_3: "~~κ~ηθητω", "~~κ~ηθητων", "~~κ~ηθεντων";
    perfect_ind_act_1: "~~κεκ~ηκᾰ", _, "~~κεκ~ηκᾰμεν";
    perfect_ind_act_2: "~~κεκ~ηκᾰς", "~~κεκ~ηκᾰτον", "~~κεκ~ηκᾰτε";
    perfect_ind_act_3: "~~κεκ~ηκε", _, "~~κεκ~ηκᾱσῐ";
    perfect_ind_mid_1: "~~κεκ~ημαι", _, "~~κεκ~ημεθᾰ";
    perfect_ind_mid_2: "~~κεκ~ησαι", "~~κεκ~ησθον", "~~κεκ~ησθε";
    perfect_ind_mid_3: "~~κεκ~ηται", _, "~~κεκ~ηνται";
    perfect_subj_act_1: "~~κεκ~ηκω", _, "~~κεκ~ηκωμεν";
    perfect_subj_act_2: "~~κεκ~ηκῃς", "~~κεκ~ηκητον", "~~κεκ~ηκητε";
    perfect_subj_act_3: "~~κεκ~ηκῃ", _, "~~κεκ~ηκωσῐ";
    perfect_opt_act_1: "~~κεκ~ηκοιμῐ", _, "~~κεκ~ηκοιμεν";
    perfect_opt_act_2: "~~κεκ~ηκοις", "~~κεκ~ηκοιτον", "~~κεκ~ηκοιτε";
    perfect_opt_act_3: "~~κεκ~ηκοι", "~~κεκ~ηκοιτην", "~~κεκ~ηκοιεν";
    perfect_imp_act_2: _, "~~κεκ~ηκετον", "~~κεκ~ηκετε";
    perfect_imp_act_3: "~~κεκ~ηκετω", "~~κεκ~ηκετων", "~~κεκ~ηκοντων";
    perfect_imp_mid_2: "~~κεκ~ησο", _, _;
    perfect_imp_mid_3: "~~κεκ~ησθω", "~~κεκ~ησθων", _;
    infinitive_pres: "~~κᾰ~εῑν", "~~κᾰ~εῑσθαι", _;
    infinitive_fut: "~~κᾰ~εσειν", "~~κᾰ~εσεσθαι", "~~κ~ηθησεσθαι";
    infinitive_aor: "~~κᾰ~εσαι", "~~κᾰ~εσᾰσθαι", "~~κ~ηθηναι";
    infinitive_perf: "~~κεκ~ηκεναι", "~~κεκ~ησθαι", _;
    participle_pres: "~~κᾰ~ων", "~~κᾰ~ουμενος", _;
    participle_fut: "~~κᾰ~εσων", "~~κᾰ~εσομενος", "~~κ~ηθησομενος";
    participle_aor: "~~κᾰ~εσᾱς", "~~κᾰ~εσᾰμενος", "~~κ~ηθεις";
    participle_perf: "~~κεκ~ηκως", "~~κεκ~ημενος", _;
    ),
    // Ἡγεομαι: 4 words (ἡγεομαι, νεομαι, ἐξηγεομαι, εὐνομεομαι). Like Εὐλογεω, except in 67 cells: ind_act_1 sg, ind_mid_2 sg, subj_mid_2 sg, subj_pass_2 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("ἡγεομαι", 4, "~εομαι", like Εὐλογεω as ACT_AS_PASS;
    ind_act_1: "~εομαι", _, _;
    ind_mid_2: "~εῑ", _, _;
    subj_mid_2: "~ῃ", _, _;
    subj_pass_2: "~ῃ", _, _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_2: "~ησῃ", _, _;
    future_ind_pass_2: "~ηθησῃ", _, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_subj_act_1: derived, _, derived;
    aorist_subj_act_2: derived, derived, derived;
    aorist_subj_act_3: derived, _, derived;
    aorist_subj_mid_2: "~ησῃ", _, _;
    aorist_subj_pass_3: _, _, "~ηθωσῐ";
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_pass_1: _, _, "~ηθεῑμεν";
    aorist_opt_pass_2: _, "~ηθεῑτον", "~ηθεῑτε";
    aorist_opt_pass_3: _, "~ηθειτην", "~ηθεῑεν";
    aorist_imp_act_2: derived, _, _;
    aorist_imp_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    participle_fut: derived, _, _;
    participle_aor: derived, _, _;
    ),
    // Τῠπτω: 4 words (κοπτω, τῠπτω, βλᾰπτω, σκωπτω). Like Ἁπτω, except in 92 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_3 pl, future_ind_mid_2 sg, ….
    conjugation!("τῠπτω", 4, "~πτω", like Ἁπτω;
    stem aorist_ind: "~ψ- like ποιεω";
    stem aorist_ind_pass: "~πη- like ἐχω";
    stem pluperfect_act: "~φ- like χρημᾰτῐζω";
    ind_act_3: _, _, "~πτουσῐ|~πτουσῐν";
    ind_mid_2: "~πτῃ|~πτει", _, _;
    subj_act_3: _, _, "~πτωσῐ|~πτωσῐν";
    imperfect_act_3: "ἐ~πτε|ἐ~πτεν", _, _;
    future_ind_act_3: _, _, "~ψουσῐ|~ψουσῐν";
    future_ind_mid_2: "~ψῃ|~ψει", _, _;
    future_ind_pass_1: "~πησομαι", _, "~πησομεθᾰ";
    future_ind_pass_2: "~πησῃ", "~πησεσθον", "~πησεσθε";
    future_ind_pass_3: "~πησεται", _, "~πησονται";
    future_opt_pass_1: "~πησοιμην", _, "~πησοιμεθᾰ";
    future_opt_pass_2: "~πησοιο", "~πησοισθον", "~πησοισθε";
    future_opt_pass_3: "~πησοιτο", "~πησοισθην", "~πησοιντο";
    aorist_subj_act_3: _, _, "~ψωσῐ|~ψωσῐν";
    aorist_subj_pass_1: "~πω", _, "~πωμεν";
    aorist_subj_pass_2: "~πῃς", "~πητον", "~πητε";
    aorist_subj_pass_3: "~πῃ", _, "~πωσῐ";
    aorist_opt_act_2: "~ψειᾰς|~ψαις", _, _;
    aorist_opt_act_3: "~ψειε|~ψειεν|~ψαι", _, "~ψειᾰν|~ψαιεν";
    aorist_opt_pass_1: "~πειην", _, "~πεῑμεν";
    aorist_opt_pass_2: "~πειης", "~πεῑτον", "~πεῑτε";
    aorist_opt_pass_3: "~πειη", "~πειτην", "~πεῑεν";
    aorist_imp_pass_2: "~πηθῐ", _, _;
    aorist_imp_pass_3: "~πητω", "~πητων", "~πεντων";
    perfect_ind_act_3: "%ε~φε|%ε~φεν", _, "%ε~φᾱσῐ|%ε~φᾱσῐν";
    perfect_ind_mid_1: "%ε~μμαι", _, "%ε~μμεθᾰ";
    perfect_ind_mid_2: "%ε~ψαι", "%ε~φθον", "%ε~φθε";
    perfect_ind_mid_3: "%ε~πται", _, "%ε~πᾰται";
    perfect_subj_act_3: _, _, "%ε~φωσῐ|%ε~φωσῐν";
    perfect_opt_act_1: "%ε~φοιμῐ|%ε~φοιην", _, _;
    perfect_opt_act_2: "%ε~φοις|%ε~φοιης", _, _;
    perfect_opt_act_3: "%ε~φοι|%ε~φοιη", _, _;
    perfect_imp_act_2: "%ε~φε", _, _;
    perfect_imp_mid_2: "%ε~ψο", _, _;
    perfect_imp_mid_3: "%ε~φθω", "%ε~φθων", _;
    pluperfect_mid_1: "ἐ%ε~μμην", _, "ἐ%ε~μμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ψο", "ἐ%ε~φθον", "ἐ%ε~φθε";
    pluperfect_mid_3: "ἐ%ε~πτο", "ἐ%ε~φθην", "ἐ%ε~πᾰτο";
    future_perfect_ind_mid_1: "%ε~ψομαι", _, "%ε~ψομεθᾰ";
    future_perfect_ind_mid_2: "%ε~ψῃ", "%ε~ψεσθον", "%ε~ψεσθε";
    future_perfect_ind_mid_3: "%ε~ψεται", _, "%ε~ψονται";
    future_perfect_opt_mid_1: "%ε~ψοιμην", _, "%ε~ψοιμεθᾰ";
    future_perfect_opt_mid_2: "%ε~ψοιο", "%ε~ψοισθον", "%ε~ψοισθε";
    future_perfect_opt_mid_3: "%ε~ψοιτο", "%ε~ψοισθην", "%ε~ψοιντο";
    infinitive_fut: _, _, "~πησεσθαι";
    infinitive_aor: _, _, "~πηναι";
    infinitive_perf: _, "%ε~φθαι", _;
    infinitive_futp: _, "%ε~ψεσθαι", _;
    participle_fut: _, _, "~πησομενος";
    participle_aor: _, _, "~πεις";
    participle_perf: _, "%ε~μμενος", _;
    participle_futp: _, "%ε~ψομενος", _;
    ),
    // Ἐπᾰγγελλω: 4 words (πᾰρᾰγγελλω, ἀπᾰγγελλω, ἐπᾰγγελλω, ἀνᾰγγελλω). Like Ζευγνῡμῐ, except in 228 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐπᾰγγελλω", 4, "~ᾰ~~λω", like Ζευγνῡμῐ;
    stem aorist_ind: "~η~ι~- like εὐλογεω";
    stem aorist_ind_pass: "~η~~θη- like ἐλπω";
    ind_act_1: "~ᾰ~~λω", _, "~ᾰ~~λομεν";
    ind_act_2: "~ᾰ~~λεις", "~ᾰ~~λετον", "~ᾰ~~λετε";
    ind_act_3: "~ᾰ~~λει", _, "~ᾰ~~λουσῐ|~ᾰ~~λουσῐν";
    ind_mid_1: "~ᾰ~~λομαι", _, "~ᾰ~~λομεθᾰ";
    ind_mid_2: "~ᾰ~~λῃ|~ᾰ~~λει", "~ᾰ~~λεσθον", "~ᾰ~~λεσθε";
    ind_mid_3: "~ᾰ~~λεται", _, "~ᾰ~~λονται";
    subj_act_1: "~ᾰ~~λω", _, "~ᾰ~~λωμεν";
    subj_act_2: "~ᾰ~~λῃς", "~ᾰ~~λητον", "~ᾰ~~λητε";
    subj_act_3: "~ᾰ~~λῃ", _, "~ᾰ~~λωσῐ|~ᾰ~~λωσῐν";
    subj_mid_1: "~ᾰ~~λωμαι", _, "~ᾰ~~λωμεθᾰ";
    subj_mid_2: _, "~ᾰ~~λησθον", "~ᾰ~~λησθε";
    subj_mid_3: "~ᾰ~~ληται", _, "~ᾰ~~λωνται";
    opt_act_1: "~ᾰ~~λοιμῐ", _, "~ᾰ~~λοιμεν";
    opt_act_2: "~ᾰ~~λοις", "~ᾰ~~λοιτον", "~ᾰ~~λοιτε";
    opt_act_3: "~ᾰ~~λοι", "~ᾰ~~λοιτην", "~ᾰ~~λοιεν";
    opt_mid_1: "~ᾰ~~λοιμην", _, "~ᾰ~~λοιμεθᾰ";
    opt_mid_2: "~ᾰ~~λοιο", "~ᾰ~~λοισθον", "~ᾰ~~λοισθε";
    opt_mid_3: "~ᾰ~~λοιτο", "~ᾰ~~λοισθην", "~ᾰ~~λοιντο";
    imp_act_2: "~ᾰ~~λε", _, _;
    imp_act_3: "~ᾰ~~λετω", "~ᾰ~~λετων", "~ᾰ~~λοντων";
    imp_mid_2: "~ᾰ~~λου", _, _;
    imp_mid_3: "~ᾰ~~λεσθω", "~ᾰ~~λεσθων", _;
    imperfect_act_1: "~η~~λον", _, "~η~~λομεν";
    imperfect_act_2: "~η~~λες", "~η~~λετον", "~η~~λετε";
    imperfect_act_3: "~η~~λε|~η~~λεν", "~η~~λετην", "~η~~λον";
    imperfect_mid_1: "~η~~λομην", _, "~η~~λομεθᾰ";
    imperfect_mid_2: "~η~~λου", "~η~~λεσθον", "~η~~λεσθε";
    imperfect_mid_3: "~η~~λετο", "~η~~λεσθην", "~η~~λοντο";
    future_ind_act_1: "~ᾰ~~ω", _, "~ᾰ~~οῡμεν";
    future_ind_act_2: "~ᾰ~~εῑς", "~ᾰ~~εῑτον", "~ᾰ~~εῑτε";
    future_ind_act_3: "~ᾰ~~εῑ", _, "~ᾰ~~οῡσῐ|~ᾰ~~οῡσῐν";
    future_ind_mid_1: "~ᾰ~~οῡμαι", _, "~ᾰ~~ουμεθᾰ";
    future_ind_mid_2: "~ᾰ~~εῑ|~ᾰ~~ῃ", "~ᾰ~~εῑσθον", "~ᾰ~~εῑσθε";
    future_ind_mid_3: "~ᾰ~~εῑται", _, "~ᾰ~~οῡνται";
    future_ind_pass_1: "~ᾰ~~θησομαι", _, "~ᾰ~~θησομεθᾰ";
    future_ind_pass_2: "~ᾰ~~θησῃ|~ᾰ~~θησει", "~ᾰ~~θησεσθον", "~ᾰ~~θησεσθε";
    future_ind_pass_3: "~ᾰ~~θησεται", _, "~ᾰ~~θησονται";
    future_opt_act_1: "~ᾰ~~οιην|~ᾰ~~οῑμῐ", _, "~ᾰ~~οῑμεν|~ᾰ~~οιημεν";
    future_opt_act_2: "~ᾰ~~οιης|~ᾰ~~οῑς", "~ᾰ~~οῑτον|~ᾰ~~οιητον", "~ᾰ~~οῑτε|~ᾰ~~οιητε";
    future_opt_act_3: "~ᾰ~~οιη|~ᾰ~~οῑ", "~ᾰ~~οιτην|~ᾰ~~οιητην", "~ᾰ~~οῑεν|~ᾰ~~οιησᾰν";
    future_opt_mid_1: "~ᾰ~~οιμην", _, "~ᾰ~~οιμεθᾰ";
    future_opt_mid_2: "~ᾰ~~οῑο", "~ᾰ~~οῑσθον", "~ᾰ~~οῑσθε";
    future_opt_mid_3: "~ᾰ~~οῑτο", "~ᾰ~~οισθην", "~ᾰ~~οῑντο";
    future_opt_pass_1: "~ᾰ~~θησοιμην", _, "~ᾰ~~θησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~~θησοιο", "~ᾰ~~θησοισθον", "~ᾰ~~θησοισθε";
    future_opt_pass_3: "~ᾰ~~θησοιτο", "~ᾰ~~θησοισθην", "~ᾰ~~θησοιντο";
    aorist_subj_act_1: "~ᾰ~ι~ω", _, "~ᾰ~ι~ωμεν";
    aorist_subj_act_2: "~ᾰ~ι~ῃς", "~ᾰ~ι~ητον", "~ᾰ~ι~ητε";
    aorist_subj_act_3: "~ᾰ~ι~ῃ", _, "~ᾰ~ι~ωσῐ|~ᾰ~ι~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ι~ωμαι", _, "~ᾰ~ι~ωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰ~ι~ησθον", "~ᾰ~ι~ησθε";
    aorist_subj_mid_3: "~ᾰ~ι~ηται", _, "~ᾰ~ι~ωνται";
    aorist_subj_pass_1: "~ᾰ~~θω", _, "~ᾰ~~θωμεν";
    aorist_subj_pass_2: "~ᾰ~~θῃς", "~ᾰ~~θητον", "~ᾰ~~θητε";
    aorist_subj_pass_3: "~ᾰ~~θῃ", _, "~ᾰ~~θωσῐ|~ᾰ~~θωσῐν";
    aorist_opt_act_1: "~ᾰ~ι~αιμῐ", _, "~ᾰ~ι~αιμεν";
    aorist_opt_act_2: "~ᾰ~ι~ειᾰς|~ᾰ~ι~αις", "~ᾰ~ι~αιτον", "~ᾰ~ι~αιτε";
    aorist_opt_act_3: "~ᾰ~ι~ειε|~ᾰ~ι~ειεν|~ᾰ~ι~αι", "~ᾰ~ι~αιτην", "~ᾰ~ι~ειᾰν|~ᾰ~ι~αιεν";
    aorist_opt_mid_1: "~ᾰ~ι~αιμην", _, "~ᾰ~ι~αιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~ι~αιο", "~ᾰ~ι~αισθον", "~ᾰ~ι~αισθε";
    aorist_opt_mid_3: "~ᾰ~ι~αιτο", "~ᾰ~ι~αισθην", "~ᾰ~ι~αιντο";
    aorist_opt_pass_1: "~ᾰ~~θειην", _, "~ᾰ~~θεῑμεν|~ᾰ~~θειημεν";
    aorist_opt_pass_2: "~ᾰ~~θειης", "~ᾰ~~θεῑτον|~ᾰ~~θειητον", "~ᾰ~~θεῑτε|~ᾰ~~θειητε";
    aorist_opt_pass_3: "~ᾰ~~θειη", "~ᾰ~~θειτην|~ᾰ~~θειητην", "~ᾰ~~θεῑεν|~ᾰ~~θειησᾰν";
    aorist_imp_act_2: "~ᾰ~ι~ον", "~ᾰ~ι~ᾰτον", "~ᾰ~ι~ᾰτε";
    aorist_imp_act_3: "~ᾰ~ι~ᾰτω", "~ᾰ~ι~ᾰτων", "~ᾰ~ι~ᾰντων";
    aorist_imp_mid_2: "~ᾰ~ι~αι", "~ᾰ~ι~ᾰσθον", "~ᾰ~ι~ᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ι~ᾰσθω", "~ᾰ~ι~ᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~~θητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~~θητω", "~ᾰ~~θητων", "~ᾰ~~θεντων";
    perfect_ind_act_1: "~η~~κᾰ", _, "~η~~κᾰμεν";
    perfect_ind_act_2: "~η~~κᾰς", "~η~~κᾰτον", "~η~~κᾰτε";
    perfect_ind_act_3: "~η~~κε|~η~~κεν", _, "~η~~κᾱσῐ|~η~~κᾱσῐν";
    perfect_ind_mid_1: "~η~~μαι", _, "~η~~μεθᾰ";
    perfect_ind_mid_2: "~η~~σαι", "~η~~θον", "~η~~θε";
    perfect_ind_mid_3: "~η~~ται", _, "~η~~ᾰται";
    perfect_subj_act_1: "~η~~κω", _, "~η~~κωμεν";
    perfect_subj_act_2: "~η~~κῃς", "~η~~κητον", "~η~~κητε";
    perfect_subj_act_3: "~η~~κῃ", _, "~η~~κωσῐ|~η~~κωσῐν";
    perfect_opt_act_1: "~η~~κοιμῐ|~η~~κοιην", _, "~η~~κοιμεν";
    perfect_opt_act_2: "~η~~κοις|~η~~κοιης", "~η~~κοιτον", "~η~~κοιτε";
    perfect_opt_act_3: "~η~~κοι|~η~~κοιη", "~η~~κοιτην", "~η~~κοιεν";
    perfect_imp_act_2: "~η~~κε", "~η~~κετον", "~η~~κετε";
    perfect_imp_act_3: "~η~~κετω", "~η~~κετων", "~η~~κοντων";
    perfect_imp_mid_2: "~η~~σο", _, _;
    perfect_imp_mid_3: "~η~~θω", "~η~~θων", _;
    infinitive_pres: "~ᾰ~~λειν", "~ᾰ~~λεσθαι", _;
    infinitive_fut: "~ᾰ~~εῑν", "~ᾰ~~εῑσθαι", "~ᾰ~~θησεσθαι";
    infinitive_aor: "~ᾰ~ῑ~αι", "~ᾰ~ι~ᾰσθαι", "~ᾰ~~θηναι";
    infinitive_perf: "~η~~κεναι", "~η~~θαι", _;
    participle_pres: "~ᾰ~~λων", "~ᾰ~~λομενος", _;
    participle_fut: "~ᾰ~~ων", "~ᾰ~~ουμενος", "~ᾰ~~θησομενος";
    participle_aor: "~ᾰ~ι~ᾱς", "~ᾰ~ι~ᾰμενος", "~ᾰ~~θεις";
    participle_perf: "~η~~κως", "~η~~μενος", _;
    ),
    // Ναιω: 4 words (καιω, κλαιω, ναιω, δαιω). Like Γῡροω, except in 219 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ναιω", 4, "~αιω", like Γῡροω;
    stem imperfect: "~αι- like ῥεω";
    stem aorist_ind: "~αυσ- like ἐχω";
    stem aorist_ind_pass: "~αυθη- like ἐχω";
    stem pluperfect_act: "~εκαυκε- like κᾰλεω";
    ind_act_1: "~αιω", _, "~αιομεν";
    ind_act_2: "~αιεις", "~αιετον", "~αιετε";
    ind_act_3: "~αιει", _, "~αιουσῐ";
    ind_mid_1: "~αιομαι", _, "~αιομεθᾰ";
    ind_mid_2: "~αιῃ", "~αιεσθον", "~αιεσθε";
    ind_mid_3: "~αιεται", _, "~αιονται";
    subj_act_1: "~αιω", _, "~αιωμεν";
    subj_act_2: "~αιῃς", "~αιητον", "~αιητε";
    subj_act_3: "~αιῃ", _, "~αιωσῐ";
    subj_mid_1: "~αιωμαι", _, "~αιωμεθᾰ";
    subj_mid_2: _, "~αιησθον", "~αιησθε";
    subj_mid_3: "~αιηται", _, "~αιωνται";
    opt_act_1: "~αιοιμῐ", _, "~αιοιμεν";
    opt_act_2: "~αιοις", "~αιοιτον", "~αιοιτε";
    opt_act_3: "~αιοι", "~αιοιτην", "~αιοιεν";
    opt_mid_1: "~αιοιμην", _, "~αιοιμεθᾰ";
    opt_mid_2: "~αιοιο", "~αιοισθον", "~αιοισθε";
    opt_mid_3: "~αιοιτο", "~αιοισθην", "~αιοιντο";
    imp_act_2: "~αῑε", _, _;
    imp_act_3: "~αιετω", "~αιετων", "~αιοντων";
    imp_mid_2: "~αιου", _, _;
    imp_mid_3: "~αιεσθω", "~αιεσθων", _;
    future_ind_act_1: "~αυσω", _, "~αυσομεν";
    future_ind_act_2: "~αυσεις", "~αυσετον", "~αυσετε";
    future_ind_act_3: "~αυσει", _, "~αυσουσῐ";
    future_ind_mid_1: "~αυσομαι", _, "~αυσομεθᾰ";
    future_ind_mid_2: "~αυσῃ", "~αυσεσθον", "~αυσεσθε";
    future_ind_mid_3: "~αυσεται", _, "~αυσονται";
    future_ind_pass_1: "~αυθησομαι", _, "~αυθησομεθᾰ";
    future_ind_pass_2: "~αυθησῃ", "~αυθησεσθον", "~αυθησεσθε";
    future_ind_pass_3: "~αυθησεται", _, "~αυθησονται";
    future_opt_act_1: "~αυσοιμῐ", _, "~αυσοιμεν";
    future_opt_act_2: "~αυσοις", "~αυσοιτον", "~αυσοιτε";
    future_opt_act_3: "~αυσοι", "~αυσοιτην", "~αυσοιεν";
    future_opt_mid_1: "~αυσοιμην", _, "~αυσοιμεθᾰ";
    future_opt_mid_2: "~αυσοιο", "~αυσοισθον", "~αυσοισθε";
    future_opt_mid_3: "~αυσοιτο", "~αυσοισθην", "~αυσοιντο";
    future_opt_pass_1: "~αυθησοιμην", _, "~αυθησοιμεθᾰ";
    future_opt_pass_2: "~αυθησοιο", "~αυθησοισθον", "~αυθησοισθε";
    future_opt_pass_3: "~αυθησοιτο", "~αυθησοισθην", "~αυθησοιντο";
    aorist_subj_act_1: "~αυσω", _, "~αυσωμεν";
    aorist_subj_act_2: "~αυσῃς", "~αυσητον", "~αυσητε";
    aorist_subj_act_3: "~αυσῃ", _, "~αυσωσῐ";
    aorist_subj_mid_1: "~αυσωμαι", _, "~αυσωμεθᾰ";
    aorist_subj_mid_2: _, "~αυσησθον", "~αυσησθε";
    aorist_subj_mid_3: "~αυσηται", _, "~αυσωνται";
    aorist_subj_pass_1: "~αυθω", _, "~αυθωμεν";
    aorist_subj_pass_2: "~αυθῃς", "~αυθητον", "~αυθητε";
    aorist_subj_pass_3: "~αυθῃ", _, "~αυθωσῐ";
    aorist_opt_act_1: "~αυσαιμῐ", _, "~αυσαιμεν";
    aorist_opt_act_2: "~αυσειᾰς", "~αυσαιτον", "~αυσαιτε";
    aorist_opt_act_3: "~αυσειε", "~αυσαιτην", "~αυσειᾰν";
    aorist_opt_mid_1: "~αυσαιμην", _, "~αυσαιμεθᾰ";
    aorist_opt_mid_2: "~αυσαιο", "~αυσαισθον", "~αυσαισθε";
    aorist_opt_mid_3: "~αυσαιτο", "~αυσαισθην", "~αυσαιντο";
    aorist_opt_pass_1: "~αυθειην", _, "~αυθεῑμεν";
    aorist_opt_pass_2: "~αυθειης", "~αυθεῑτον", "~αυθεῑτε";
    aorist_opt_pass_3: "~αυθειη", "~αυθειτην", "~αυθεῑεν";
    aorist_imp_act_2: "~αῡσον", "~αυσᾰτον", "~αυσᾰτε";
    aorist_imp_act_3: "~αυσᾰτω", "~αυσᾰτων", "~αυσᾰντων";
    aorist_imp_mid_2: "~αῡσαι", "~αυσᾰσθον", "~αυσᾰσθε";
    aorist_imp_mid_3: "~αυσᾰσθω", "~αυσᾰσθων", _;
    aorist_imp_pass_2: "~αυθητῐ", _, _;
    aorist_imp_pass_3: "~αυθητω", "~αυθητων", "~αυθεντων";
    perfect_ind_act_1: "~εκαυκᾰ", _, "~εκαυκᾰμεν";
    perfect_ind_act_2: "~εκαυκᾰς", "~εκαυκᾰτον", "~εκαυκᾰτε";
    perfect_ind_act_3: "~εκαυκε", _, "~εκαυκᾱσῐ";
    perfect_ind_mid_1: "~εκαυμαι", _, "~εκαυμεθᾰ";
    perfect_ind_mid_2: "~εκαυσαι", "~εκαυσθον", "~εκαυσθε";
    perfect_ind_mid_3: "~εκαυται", _, "~εκαυνται";
    perfect_subj_act_1: "~εκαυκω", _, "~εκαυκωμεν";
    perfect_subj_act_2: "~εκαυκῃς", "~εκαυκητον", "~εκαυκητε";
    perfect_subj_act_3: "~εκαυκῃ", _, "~εκαυκωσῐ";
    perfect_opt_act_1: "~εκαυκοιμῐ", _, "~εκαυκοιμεν";
    perfect_opt_act_2: "~εκαυκοις", "~εκαυκοιτον", "~εκαυκοιτε";
    perfect_opt_act_3: "~εκαυκοι", "~εκαυκοιτην", "~εκαυκοιεν";
    perfect_imp_act_2: _, "~εκαυκετον", "~εκαυκετε";
    perfect_imp_act_3: "~εκαυκετω", "~εκαυκετων", "~εκαυκοντων";
    perfect_imp_mid_2: "~εκαυσο", _, _;
    perfect_imp_mid_3: "~εκαυσθω", "~εκαυσθων", _;
    pluperfect_mid_1: "ἐ~εκαυμην", _, "ἐ~εκαυμεθᾰ";
    pluperfect_mid_2: "ἐ~εκαυσο", "ἐ~εκαυσθον", "ἐ~εκαυσθε";
    pluperfect_mid_3: "ἐ~εκαυτο", "ἐ~εκαυσθην", "ἐ~εκαυντο";
    infinitive_pres: "~αιειν", "~αιεσθαι", _;
    infinitive_fut: "~αυσειν", "~αυσεσθαι", "~αυθησεσθαι";
    infinitive_aor: "~αῡσαι", "~αυσᾰσθαι", "~αυθηναι";
    infinitive_perf: "~εκαυκεναι", "~εκαῡσθαι", _;
    participle_pres: "~αιων", "~αιομενος", _;
    participle_fut: "~αυσων", "~αυσομενος", "~αυθησομενος";
    participle_aor: "~αυσᾱς", "~αυσᾰμενος", "~αυθεις";
    participle_perf: "~εκαυκως", "~εκαυμενος", _;
    ),
    // Προσφερω: 4 words (προφερω, προσφερω, περῐφερω, ὑπερφερω). Like Μῑγνῡμῐ, except in 215 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("προσφερω", 4, "~φερω", like Μῑγνῡμῐ;
    stem aorist_ind: "~ηνεγκ- like ἐλπω";
    stem aorist_ind_pass: "~ηνεχθη- like ἐλπω";
    ind_act_1: "~φερω", _, "~φερομεν";
    ind_act_2: "~φερεις", "~φερετον", "~φερετε";
    ind_act_3: "~φερει", _, "~φερουσῐ|~φερουσῐν";
    ind_mid_1: "~φερομαι", _, "~φερομεθᾰ";
    ind_mid_2: "~φερῃ|~φερει", "~φερεσθον", "~φερεσθε";
    ind_mid_3: "~φερεται", _, "~φερονται";
    subj_act_1: "~φερω", _, "~φερωμεν";
    subj_act_2: "~φερῃς", "~φερητον", "~φερητε";
    subj_act_3: "~φερῃ", _, "~φερωσῐ|~φερωσῐν";
    subj_mid_1: "~φερωμαι", _, "~φερωμεθᾰ";
    subj_mid_2: _, "~φερησθον", "~φερησθε";
    subj_mid_3: "~φερηται", _, "~φερωνται";
    opt_act_1: "~φεροιμῐ", _, "~φεροιμεν";
    opt_act_2: "~φεροις", "~φεροιτον", "~φεροιτε";
    opt_act_3: "~φεροι", "~φεροιτην", "~φεροιεν";
    opt_mid_1: "~φεροιμην", _, "~φεροιμεθᾰ";
    opt_mid_2: "~φεροιο", "~φεροισθον", "~φεροισθε";
    opt_mid_3: "~φεροιτο", "~φεροισθην", "~φεροιντο";
    imp_act_2: "~φερε", _, _;
    imp_act_3: "~φερετω", "~φερετων", "~φεροντων";
    imp_mid_2: "~φερου", _, _;
    imp_mid_3: "~φερεσθω", "~φερεσθων", _;
    imperfect_act_1: "~εφερον", _, "~εφερομεν";
    imperfect_act_2: "~εφερες", "~εφερετον", "~εφερετε";
    imperfect_act_3: "~εφερε|~εφερεν", "~εφερετην", "~εφερον";
    imperfect_mid_1: "~εφερομην", _, "~εφερομεθᾰ";
    imperfect_mid_2: "~εφερου", "~εφερεσθον", "~εφερεσθε";
    imperfect_mid_3: "~εφερετο", "~εφερεσθην", "~εφεροντο";
    future_ind_act_1: "~οισω", _, "~οισομεν";
    future_ind_act_2: "~οισεις", "~οισετον", "~οισετε";
    future_ind_act_3: "~οισει", _, "~οισουσῐ|~οισουσῐν";
    future_ind_mid_1: "~οισομαι", _, "~οισομεθᾰ";
    future_ind_mid_2: "~οισῃ|~οισει", "~οισεσθον", "~οισεσθε";
    future_ind_mid_3: "~οισεται", _, "~οισονται";
    future_ind_pass_1: "~οισθησομαι", _, "~οισθησομεθᾰ";
    future_ind_pass_2: "~οισθησῃ", "~οισθησεσθον", "~οισθησεσθε";
    future_ind_pass_3: "~οισθησεται", _, "~οισθησονται";
    future_opt_act_1: "~οισοιμῐ", _, "~οισοιμεν";
    future_opt_act_2: "~οισοις", "~οισοιτον", "~οισοιτε";
    future_opt_act_3: "~οισοι", "~οισοιτην", "~οισοιεν";
    future_opt_mid_1: "~οισοιμην", _, "~οισοιμεθᾰ";
    future_opt_mid_2: "~οισοιο", "~οισοισθον", "~οισοισθε";
    future_opt_mid_3: "~οισοιτο", "~οισοισθην", "~οισοιντο";
    future_opt_pass_1: "~οισθησοιμην", _, "~οισθησοιμεθᾰ";
    future_opt_pass_2: "~οισθησοιο", "~οισθησοισθον", "~οισθησοισθε";
    future_opt_pass_3: "~οισθησοιτο", "~οισθησοισθην", "~οισθησοιντο";
    aorist_subj_act_1: "~ενεγκω", _, "~ενεγκωμεν";
    aorist_subj_act_2: "~ενεγκῃς", "~ενεγκητον", "~ενεγκητε";
    aorist_subj_act_3: "~ενεγκῃ", _, "~ενεγκωσῐ";
    aorist_subj_mid_1: "~ενεγκωμαι", _, "~ενεγκωμεθᾰ";
    aorist_subj_mid_2: _, "~ενεγκησθον", "~ενεγκησθε";
    aorist_subj_mid_3: "~ενεγκηται", _, "~ενεγκωνται";
    aorist_subj_pass_1: "~ενεχθω", _, "~ενεχθωμεν";
    aorist_subj_pass_2: "~ενεχθῃς", "~ενεχθητον", "~ενεχθητε";
    aorist_subj_pass_3: "~ενεχθῃ", _, "~ενεχθωσῐ";
    aorist_opt_act_1: "~ενεγκαιμῐ", _, "~ενεγκαιμεν";
    aorist_opt_act_2: "~ενεγκειᾰς", "~ενεγκαιτον", "~ενεγκαιτε";
    aorist_opt_act_3: "~ενεγκειε", "~ενεγκαιτην", "~ενεγκειᾰν";
    aorist_opt_mid_1: "~ενεγκαιμην", _, "~ενεγκαιμεθᾰ";
    aorist_opt_mid_2: "~ενεγκαιο", "~ενεγκαισθον", "~ενεγκαισθε";
    aorist_opt_mid_3: "~ενεγκαιτο", "~ενεγκαισθην", "~ενεγκαιντο";
    aorist_opt_pass_1: "~ενεχθειην", _, "~ενεχθεῑμεν";
    aorist_opt_pass_2: "~ενεχθειης", "~ενεχθεῑτον", "~ενεχθεῑτε";
    aorist_opt_pass_3: "~ενεχθειη", "~ενεχθειτην", "~ενεχθεῑεν";
    aorist_imp_act_2: "~ενεγκον", "~ενεγκᾰτον", "~ενεγκᾰτε";
    aorist_imp_act_3: "~ενεγκᾰτω", "~ενεγκᾰτων", "~ενεγκᾰντων";
    aorist_imp_mid_2: "~ενεγκαι", "~ενεγκᾰσθον", "~ενεγκᾰσθε";
    aorist_imp_mid_3: "~ενεγκᾰσθω", "~ενεγκᾰσθων", _;
    aorist_imp_pass_2: "~ενεχθητῐ", _, _;
    aorist_imp_pass_3: "~ενεχθητω", "~ενεχθητων", "~ενεχθεντων";
    perfect_ind_act_1: "~ενηνεχᾰ", _, "~ενηνεχᾰμεν";
    perfect_ind_act_2: "~ενηνεχᾰς", "~ενηνεχᾰτον", "~ενηνεχᾰτε";
    perfect_ind_act_3: "~ενηνεχε", _, "~ενηνεχᾱσῐ";
    perfect_subj_act_1: "~ενηνεχω", _, "~ενηνεχωμεν";
    perfect_subj_act_2: "~ενηνεχῃς", "~ενηνεχητον", "~ενηνεχητε";
    perfect_subj_act_3: "~ενηνεχῃ", _, "~ενηνεχωσῐ";
    perfect_opt_act_1: "~ενηνεχοιμῐ", _, "~ενηνεχοιμεν";
    perfect_opt_act_2: "~ενηνεχοις", "~ενηνεχοιτον", "~ενηνεχοιτε";
    perfect_opt_act_3: "~ενηνεχοι", "~ενηνεχοιτην", "~ενηνεχοιεν";
    perfect_imp_act_2: _, "~ενηνεχετον", "~ενηνεχετε";
    perfect_imp_act_3: "~ενηνεχετω", "~ενηνεχετων", "~ενηνεχοντων";
    infinitive_pres: "~φερειν", "~φερεσθαι", _;
    infinitive_fut: "~οισειν", "~οισεσθαι", "~οισθησεσθαι";
    infinitive_aor: "~ενεγκαι", "~ενεγκᾰσθαι", "~ενεχθηναι";
    infinitive_perf: "~ενηνεχεναι", _, _;
    participle_pres: "~φερων", "~φερομενος", _;
    participle_fut: "~οισων", "~οισομενος", "~οισθησομενος";
    participle_aor: "~ενεγκᾱς", "~ενεγκᾰμενος", "~ενεχθεις";
    participle_perf: "~ενηνεχως", _, _;
    ),
    // Ἐπιτρεχω: 4 words (περῐτρεχω, κᾰτᾰτρεχω, ἐπιτρεχω, ἀνατρεχω). Like Πῠνθανομαι, except in 165 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐπιτρεχω", 4, "~τ~εχω", like Πῠνθανομαι as MID_AS_ACT;
    stem aorist_ind: "~εθ~εξ- like ἐλπω";
    ind_act_1: "~τ~εχω", _, "~τ~εχομεν";
    ind_act_2: "~τ~εχεις", "~τ~εχετον", "~τ~εχετε";
    ind_act_3: "~τ~εχει", _, "~τ~εχουσῐ";
    ind_mid_1: derived, _, _;
    ind_pass_1: "~τ~εχομαι", _, "~τ~εχομεθᾰ";
    ind_pass_2: "~τ~εχῃ", "~τ~εχεσθον", "~τ~εχεσθε";
    ind_pass_3: "~τ~εχεται", "~τ~εχεσθον", "~τ~εχονται";
    subj_act_1: "~τ~εχω", _, "~τ~εχωμεν";
    subj_act_2: "~τ~εχῃς", "~τ~εχητον", "~τ~εχητε";
    subj_act_3: "~τ~εχῃ", _, "~τ~εχωσῐ";
    subj_pass_1: "~τ~εχωμαι", _, "~τ~εχωμεθᾰ";
    subj_pass_2: derived, "~τ~εχησθον", "~τ~εχησθε";
    subj_pass_3: "~τ~εχηται", "~τ~εχησθον", "~τ~εχωνται";
    opt_act_1: "~τ~εχοιμῐ", _, "~τ~εχοιμεν";
    opt_act_2: "~τ~εχοις", "~τ~εχοιτον", "~τ~εχοιτε";
    opt_act_3: "~τ~εχοι", "~τ~εχοιτην", "~τ~εχοιεν";
    opt_pass_1: "~τ~εχοιμην", _, "~τ~εχοιμεθᾰ";
    opt_pass_2: "~τ~εχοιο", "~τ~εχοισθον", "~τ~εχοισθε";
    opt_pass_3: "~τ~εχοιτο", "~τ~εχοισθην", "~τ~εχοιντο";
    imp_act_2: "~τ~εχε", _, _;
    imp_act_3: "~τ~εχετω", "~τ~εχετων", "~τ~εχοντων";
    imp_pass_2: "~τ~εχου", "~τ~εχεσθον", "~τ~εχεσθε";
    imp_pass_3: "~τ~εχεσθω", "~τ~εχεσθων", "~τ~εχεσθων";
    imperfect_act_1: "~ετ~εχον", _, "~ετ~εχομεν";
    imperfect_act_2: "~ετ~εχες", "~ετ~εχετον", "~ετ~εχετε";
    imperfect_act_3: "~ετ~εχε|~ετ~εχεν", "~ετ~εχετην", "~ετ~εχον";
    imperfect_mid_1: "~ετ~εχομην", _, "~ετ~εχομεθᾰ";
    imperfect_mid_2: "~ετ~εχου", "~ετ~εχεσθον", "~ετ~εχεσθε";
    imperfect_mid_3: "~ετ~εχετο", "~ετ~εχεσθην", "~ετ~εχοντο";
    future_ind_mid_1: "~δ~ᾰμοῡμαι", _, "~δ~ᾰμουμεθᾰ";
    future_ind_mid_2: "~δ~ᾰμεῑ", "~δ~ᾰμεῑσθον", "~δ~ᾰμεῑσθε";
    future_ind_mid_3: "~δ~ᾰμεῑται", _, "~δ~ᾰμοῡνται";
    future_opt_mid_1: "~δ~ᾰμοιμην", _, "~δ~ᾰμοιμεθᾰ";
    future_opt_mid_2: "~δ~ᾰμοῑο", "~δ~ᾰμοῑσθον", "~δ~ᾰμοῑσθε";
    future_opt_mid_3: "~δ~ᾰμοῑτο", "~δ~ᾰμοισθην", "~δ~ᾰμοῑντο";
    aorist_subj_act_1: "~θ~εξω", _, "~θ~εξωμεν";
    aorist_subj_act_2: "~θ~εξῃς", "~θ~εξητον", "~θ~εξητε";
    aorist_subj_act_3: "~θ~εξῃ", _, "~θ~εξωσῐ";
    aorist_subj_mid_1: "~θ~εξωμαι", _, "~θ~εξωμεθᾰ";
    aorist_subj_mid_2: _, "~θ~εξησθον", "~θ~εξησθε";
    aorist_subj_mid_3: "~θ~εξηται", _, "~θ~εξωνται";
    aorist_opt_act_1: "~θ~εξαιμῐ", _, "~θ~εξαιμεν";
    aorist_opt_act_2: "~θ~εξειᾰς", "~θ~εξαιτον", "~θ~εξαιτε";
    aorist_opt_act_3: "~θ~εξειε", "~θ~εξαιτην", "~θ~εξειᾰν";
    aorist_opt_mid_1: "~θ~εξαιμην", _, "~θ~εξαιμεθᾰ";
    aorist_opt_mid_2: "~θ~εξαιο", "~θ~εξαισθον", "~θ~εξαισθε";
    aorist_opt_mid_3: "~θ~εξαιτο", "~θ~εξαισθην", "~θ~εξαιντο";
    aorist_imp_act_2: "~θ~εξον", "~θ~εξᾰτον", "~θ~εξᾰτε";
    aorist_imp_act_3: "~θ~εξᾰτω", "~θ~εξᾰτων", "~θ~εξᾰντων";
    aorist_imp_mid_2: "~θ~εξαι", "~θ~εξᾰσθον", "~θ~εξᾰσθε";
    aorist_imp_mid_3: "~θ~εξᾰσθω", "~θ~εξᾰσθων", _;
    perfect_ind_act_1: "~δεδ~ᾰμηκᾰ", _, "~δεδ~ᾰμηκᾰμεν";
    perfect_ind_act_2: "~δεδ~ᾰμηκᾰς", "~δεδ~ᾰμηκᾰτον", "~δεδ~ᾰμηκᾰτε";
    perfect_ind_act_3: "~δεδ~ᾰμηκε", _, "~δεδ~ᾰμηκᾱσῐ";
    perfect_subj_act_1: "~δεδ~ᾰμηκω", _, "~δεδ~ᾰμηκωμεν";
    perfect_subj_act_2: "~δεδ~ᾰμηκῃς", "~δεδ~ᾰμηκητον", "~δεδ~ᾰμηκητε";
    perfect_subj_act_3: "~δεδ~ᾰμηκῃ", _, "~δεδ~ᾰμηκωσῐ";
    perfect_opt_act_1: "~δεδ~ᾰμηκοιμῐ", _, "~δεδ~ᾰμηκοιμεν";
    perfect_opt_act_2: "~δεδ~ᾰμηκοις", "~δεδ~ᾰμηκοιτον", "~δεδ~ᾰμηκοιτε";
    perfect_opt_act_3: "~δεδ~ᾰμηκοι", "~δεδ~ᾰμηκοιτην", "~δεδ~ᾰμηκοιεν";
    perfect_imp_act_2: _, "~δεδ~ᾰμηκετον", "~δεδ~ᾰμηκετε";
    perfect_imp_act_3: "~δεδ~ᾰμηκετω", "~δεδ~ᾰμηκετων", "~δεδ~ᾰμηκοντων";
    infinitive_pres: "~τ~εχειν", _, _;
    infinitive_fut: _, "~δ~ᾰμεῑσθαι", _;
    infinitive_aor: "~θ~εξαι", "~θ~εξᾰσθαι", _;
    infinitive_perf: "~δεδ~ᾰμηκεναι", _, _;
    participle_pres: "~τ~εχων", _, _;
    participle_fut: _, "~δ~ᾰμουμενος", _;
    participle_aor: "~θ~εξᾱς", "~θ~εξᾰμενος", _;
    participle_perf: "~δεδ~ᾰμηκως", _, _;
    ),
    // Σπᾰω: 4 words (σπᾰω, κλᾰω, γελᾰω, ἀντῐᾰω). Like Ἐαω, except in 129 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("σπᾰω", 4, "~ᾰω", like Ἐαω as PERFECT_AS_FUTURE;
    stem aorist_ind: "~ᾰσ- like ποιεω";
    stem aorist_ind_pass: "~ᾰσθη- like ἐχω";
    future_ind_act_1: "~ᾰσω", _, "~ᾰσομεν";
    future_ind_act_2: "~ᾰσεις", "~ᾰσετον", "~ᾰσετε";
    future_ind_act_3: "~ᾰσει", _, "~ᾰσουσῐ|~ᾰσουσῐν";
    future_ind_mid_1: "~ᾰσομαι", _, "~ᾰσομεθᾰ";
    future_ind_mid_2: "~ᾰσῃ|~ᾰσει", "~ᾰσεσθον", "~ᾰσεσθε";
    future_ind_mid_3: "~ᾰσεται", _, "~ᾰσονται";
    future_ind_pass_1: "~ᾰσθησομαι", _, "~ᾰσθησομεθᾰ";
    future_ind_pass_2: "~ᾰσθησῃ", "~ᾰσθησεσθον", "~ᾰσθησεσθε";
    future_ind_pass_3: "~ᾰσθησεται", _, "~ᾰσθησονται";
    future_opt_act_1: "~ᾰσοιμῐ", _, "~ᾰσοιμεν";
    future_opt_act_2: "~ᾰσοις", "~ᾰσοιτον", "~ᾰσοιτε";
    future_opt_act_3: "~ᾰσοι", "~ᾰσοιτην", "~ᾰσοιεν";
    future_opt_mid_1: "~ᾰσοιμην", _, "~ᾰσοιμεθᾰ";
    future_opt_mid_2: "~ᾰσοιο", "~ᾰσοισθον", "~ᾰσοισθε";
    future_opt_mid_3: "~ᾰσοιτο", "~ᾰσοισθην", "~ᾰσοιντο";
    future_opt_pass_1: "~ᾰσθησοιμην", _, "~ᾰσθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰσθησοιο", "~ᾰσθησοισθον", "~ᾰσθησοισθε";
    future_opt_pass_3: "~ᾰσθησοιτο", "~ᾰσθησοισθην", "~ᾰσθησοιντο";
    aorist_subj_act_1: "~ᾰσω", _, "~ᾰσωμεν";
    aorist_subj_act_2: "~ᾰσῃς", "~ᾰσητον", "~ᾰσητε";
    aorist_subj_act_3: "~ᾰσῃ", _, "~ᾰσωσῐ|~ᾰσωσῐν";
    aorist_subj_mid_1: "~ᾰσωμαι", _, "~ᾰσωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰσησθον", "~ᾰσησθε";
    aorist_subj_mid_3: "~ᾰσηται", _, "~ᾰσωνται";
    aorist_subj_pass_1: "~ᾰσθω", _, "~ᾰσθωμεν";
    aorist_subj_pass_2: "~ᾰσθῃς", "~ᾰσθητον", "~ᾰσθητε";
    aorist_subj_pass_3: "~ᾰσθῃ", _, "~ᾰσθωσῐ";
    aorist_opt_act_1: "~ᾰσαιμῐ", _, "~ᾰσαιμεν";
    aorist_opt_act_2: "~ᾰσειᾰς|~ᾰσαις", "~ᾰσαιτον", "~ᾰσαιτε";
    aorist_opt_act_3: "~ᾰσειε|~ᾰσειεν|~ᾰσαι", "~ᾰσαιτην", "~ᾰσειᾰν|~ᾰσαιεν";
    aorist_opt_mid_1: "~ᾰσαιμην", _, "~ᾰσαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰσαιο", "~ᾰσαισθον", "~ᾰσαισθε";
    aorist_opt_mid_3: "~ᾰσαιτο", "~ᾰσαισθην", "~ᾰσαιντο";
    aorist_opt_pass_1: "~ᾰσθειην", _, "~ᾰσθεῑμεν";
    aorist_opt_pass_2: "~ᾰσθειης", "~ᾰσθεῑτον", "~ᾰσθεῑτε";
    aorist_opt_pass_3: "~ᾰσθειη", "~ᾰσθειτην", "~ᾰσθεῑεν";
    aorist_imp_act_2: "~ᾰσον", "~ᾰσᾰτον", "~ᾰσᾰτε";
    aorist_imp_act_3: "~ᾰσᾰτω", "~ᾰσᾰτων", "~ᾰσᾰντων";
    aorist_imp_mid_2: "~ᾰσαι", "~ᾰσᾰσθον", "~ᾰσᾰσθε";
    aorist_imp_mid_3: "~ᾰσᾰσθω", "~ᾰσᾰσθων", _;
    aorist_imp_pass_2: "~ᾰσθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰσθητω", "~ᾰσθητων", "~ᾰσθεντων";
    perfect_ind_mid_1: "%ε~ᾰσμαι", _, "%ε~ᾰσμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾰσαι", "%ε~ᾰσθον", "%ε~ᾰσθε";
    perfect_ind_mid_3: "%ε~ᾰσται", _, "%ε~ᾰσᾰται";
    perfect_imp_mid_2: "%ε~ᾰσο", _, _;
    perfect_imp_mid_3: "%ε~ᾰσθω", "%ε~ᾰσθων", _;
    infinitive_fut: "~ᾰσειν", "~ᾰσεσθαι", "~ᾰσθησεσθαι";
    infinitive_aor: "~ᾰσαι", "~ᾰσᾰσθαι", "~ᾰσθηναι";
    infinitive_perf: derived, _, _;
    participle_fut: "~ᾰσων", "~ᾰσομενος", "~ᾰσθησομενος";
    participle_aor: "~ᾰσᾱς", "~ᾰσᾰμενος", "~ᾰσθεις";
    participle_perf: derived, "%ε~ᾰσμενος", _;
    ),
    // Σῠνᾰγω: 4 words (σῠνᾰγω, προᾰγω, ξῠνᾰγω, προσᾰγω). Like Ἀνᾰγω, except in 55 cells: future_ind_act_3 pl, future_ind_mid_2 sg, future_ind_pass_2 sg, aorist_subj_act_1 sg, aorist_subj_act_1 pl, aorist_subj_act_2 sg, ….
    conjugation!("σῠνᾰγω", 4, "~ᾰγω", like Ἀνᾰγω;
    stem aorist_ind: "~η- like σῠνᾰγω";
    future_ind_act_3: _, _, "~ᾰξουσῐ";
    future_ind_mid_2: "~ᾰξῃ", _, _;
    future_ind_pass_2: "~ᾰχθησῃ", _, _;
    aorist_subj_act_1: "~ᾰξω|~ᾰγᾰγω", _, "~ᾰξωμεν|~ᾰγᾰγωμεν";
    aorist_subj_act_2: "~ᾰξῃς|~ᾰγᾰγῃς", "~ᾰξητον|~ᾰγᾰγητον", "~ᾰξητε|~ᾰγᾰγητε";
    aorist_subj_act_3: "~ᾰξῃ|~ᾰγᾰγῃ", _, "~ᾰξωσῐ|~ᾰξωσῐν|~ᾰγᾰγωσῐ|~ᾰγᾰγωσῐν";
    aorist_subj_mid_1: "~ᾰξωμαι|~ᾰγᾰγωμαι", _, "~ᾰξωμεθᾰ|~ᾰγᾰγωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰξησθον|~ᾰγᾰγησθον", "~ᾰξησθε|~ᾰγᾰγησθε";
    aorist_subj_mid_3: "~ᾰξηται|~ᾰγᾰγηται", _, "~ᾰξωνται|~ᾰγᾰγωνται";
    aorist_subj_pass_3: _, _, "~ᾰχθωσῐ|~ᾰχθωσῐν";
    aorist_opt_act_1: "~ᾰξαιμῐ|~ᾰγᾰγοιμῐ", _, "~ᾰξαιμεν|~ᾰγᾰγοιμεν";
    aorist_opt_act_2: "~ᾰξειᾰς|~ᾰξαις|~ᾰγᾰγοις", "~ᾰξαιτον|~ᾰγᾰγοιτον", "~ᾰξαιτε|~ᾰγᾰγοιτε";
    aorist_opt_act_3: "~ᾰξειε|~ᾰξειεν|~ᾰξαι|~ᾰγᾰγοι", "~ᾰξαιτην|~ᾰγᾰγοιτην", "~ᾰξειᾰν|~ᾰξαιεν|~ᾰγᾰγοιεν";
    aorist_opt_mid_1: "~ᾰξαιμην|~ᾰγᾰγοιμην", _, "~ᾰξαιμεθᾰ|~ᾰγᾰγοιμεθᾰ";
    aorist_opt_mid_2: "~ᾰξαιο|~ᾰγᾰγοιο", "~ᾰξαισθον|~ᾰγᾰγοισθον", "~ᾰξαισθε|~ᾰγᾰγοισθε";
    aorist_opt_mid_3: "~ᾰξαιτο|~ᾰγᾰγοιτο", "~ᾰξαισθην|~ᾰγᾰγοισθην", "~ᾰξαιντο|~ᾰγᾰγοιντο";
    aorist_opt_pass_1: _, _, "~ᾰχθεῑμεν|~ᾰχθειημεν";
    aorist_opt_pass_2: _, "~ᾰχθεῑτον|~ᾰχθειητον", "~ᾰχθεῑτε|~ᾰχθειητε";
    aorist_opt_pass_3: _, "~ᾰχθειτην|~ᾰχθειητην", "~ᾰχθεῑεν|~ᾰχθειησᾰν";
    aorist_imp_act_2: "~ᾰξον|~ᾰγᾰγε", "~ᾰξᾰτον|~ᾰγᾰγετον", "~ᾰξᾰτε|~ᾰγᾰγετε";
    aorist_imp_act_3: "~ᾰξᾰτω|~ᾰγᾰγετω", "~ᾰξᾰτων|~ᾰγᾰγετων", "~ᾰξᾰντων|~ᾰγᾰγοντων";
    aorist_imp_mid_2: "~ᾰξαι|~ᾰγᾰγοῡ", "~ᾰξᾰσθον|~ᾰγᾰγεσθον", "~ᾰξᾰσθε|~ᾰγᾰγεσθε";
    aorist_imp_mid_3: "~ᾰξᾰσθω|~ᾰγᾰγεσθω", "~ᾰξᾰσθων|~ᾰγᾰγεσθων", _;
    infinitive_aor: "~ᾰξαι|~ᾰγᾰγεῑν", "~ᾰξᾰσθαι|~ᾰγᾰγεσθαι", _;
    infinitive_perf: _, derived, _;
    participle_aor: "~ᾰξᾱς|~ᾰγᾰγων", "~ᾰξᾰμενος|~ᾰγᾰγομενος", _;
    participle_perf: _, derived, _;
    ),
    // Πειρω: 4 words (πειρω, κτεινω, σπειρω, φθειρω). Like Ἀειρω, except in 95 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_pass_1 sg, future_ind_pass_1 pl, ….
    conjugation!("πειρω", 4, "~ει~ω", like Ἀειρω;
    stem aorist_ind: "~ει~- like ποιεω";
    stem aorist_ind_pass: "~ᾰ~η- like ἐχω";
    stem pluperfect_act: "~ᾰ~κε- like κᾰλεω";
    ind_act_3: _, _, "~ει~ουσῐ|~ει~ουσῐν";
    ind_mid_2: "~ει~ῃ|~ει~ει", _, _;
    subj_act_3: _, _, "~ει~ωσῐ|~ει~ωσῐν";
    imperfect_act_3: "ἐ~ει~ε|ἐ~ει~εν", _, _;
    future_ind_pass_1: "~ᾰ~ησομαι", _, "~ᾰ~ησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ησῃ", "~ᾰ~ησεσθον", "~ᾰ~ησεσθε";
    future_ind_pass_3: "~ᾰ~ησεται", _, "~ᾰ~ησονται";
    future_opt_pass_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ησοιο", "~ᾰ~ησοισθον", "~ᾰ~ησοισθε";
    future_opt_pass_3: "~ᾰ~ησοιτο", "~ᾰ~ησοισθην", "~ᾰ~ησοιντο";
    aorist_subj_act_3: _, _, "~ει~ωσῐ|~ει~ωσῐν";
    aorist_subj_pass_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    aorist_subj_pass_2: "~ᾰ~ῃς", "~ᾰ~ητον", "~ᾰ~ητε";
    aorist_subj_pass_3: "~ᾰ~ῃ", _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    aorist_opt_act_2: "~ει~ειᾰς|~ει~αις", _, _;
    aorist_opt_act_3: "~ει~ειε|~ει~ειεν|~ει~αι", _, "~ει~ειᾰν|~ει~αιεν";
    aorist_opt_pass_1: "~ᾰ~ειην", _, "~ᾰ~εῑμεν|~ᾰ~ειημεν";
    aorist_opt_pass_2: "~ᾰ~ειης", "~ᾰ~εῑτον|~ᾰ~ειητον", "~ᾰ~εῑτε|~ᾰ~ειητε";
    aorist_opt_pass_3: "~ᾰ~ειη", "~ᾰ~ειτην|~ᾰ~ειητην", "~ᾰ~εῑεν|~ᾰ~ειησᾰν";
    aorist_imp_pass_2: "~ᾰ~ηθῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ητω", "~ᾰ~ητων", "~ᾰ~εντων";
    perfect_ind_act_1: "ἐ~ᾰ~κᾰ", _, "ἐ~ᾰ~κᾰμεν";
    perfect_ind_act_2: "ἐ~ᾰ~κᾰς", "ἐ~ᾰ~κᾰτον", "ἐ~ᾰ~κᾰτε";
    perfect_ind_act_3: "ἐ~ᾰ~κε", _, "ἐ~ᾰ~κᾱσῐ";
    perfect_ind_mid_1: "ἐ~ᾰ~μαι", _, "ἐ~ᾰ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰ~σαι", "ἐ~ᾰ~θον", "ἐ~ᾰ~θε";
    perfect_ind_mid_3: "ἐ~ᾰ~ται", _, "ἐ~ᾰ~ᾰται";
    perfect_subj_act_1: "ἐ~ᾰ~κω", _, "ἐ~ᾰ~κωμεν";
    perfect_subj_act_2: "ἐ~ᾰ~κῃς", "ἐ~ᾰ~κητον", "ἐ~ᾰ~κητε";
    perfect_subj_act_3: "ἐ~ᾰ~κῃ", _, "ἐ~ᾰ~κωσῐ";
    perfect_opt_act_1: "ἐ~ᾰ~κοιμῐ", _, "ἐ~ᾰ~κοιμεν";
    perfect_opt_act_2: "ἐ~ᾰ~κοις", "ἐ~ᾰ~κοιτον", "ἐ~ᾰ~κοιτε";
    perfect_opt_act_3: "ἐ~ᾰ~κοι", "ἐ~ᾰ~κοιτην", "ἐ~ᾰ~κοιεν";
    perfect_imp_act_2: _, "ἐ~ᾰ~κετον", "ἐ~ᾰ~κετε";
    perfect_imp_act_3: "ἐ~ᾰ~κετω", "ἐ~ᾰ~κετων", "ἐ~ᾰ~κοντων";
    perfect_imp_mid_2: "ἐ~ᾰ~σο", _, _;
    perfect_imp_mid_3: "ἐ~ᾰ~θω", "ἐ~ᾰ~θων", _;
    pluperfect_mid_1: "ἐ~ᾰ~μην", _, "ἐ~ᾰ~μεθᾰ";
    pluperfect_mid_2: "ἐ~ᾰ~σο", "ἐ~ᾰ~θον", "ἐ~ᾰ~θε";
    pluperfect_mid_3: "ἐ~ᾰ~το", "ἐ~ᾰ~θην", "ἐ~ᾰ~ᾰτο";
    infinitive_fut: _, _, "~ᾰ~ησεσθαι";
    infinitive_aor: _, _, "~ᾰ~ηναι";
    infinitive_perf: "ἐ~ᾰ~κεναι", "ἐ~ᾰ~θαι", _;
    participle_fut: _, _, "~ᾰ~ησομενος";
    participle_aor: _, _, "~ᾰ~εις";
    participle_perf: "ἐ~ᾰ~κως", "ἐ~ᾰ~μενος", _;
    ),
    // Σᾰττω: 4 words (φῠλᾰττω, πληττω, σᾰττω, ἀλλᾰττω). Like Ληγω, except in 139 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("σᾰττω", 4, "~ττω", like Ληγω;
    stem aorist_ind_pass: "~γη- like ἐχω";
    ind_act_1: "~ττω", _, "~ττομεν";
    ind_act_2: "~ττεις", "~ττετον", "~ττετε";
    ind_act_3: "~ττει", _, "~ττουσῐ|~ττουσῐν";
    ind_mid_1: "~ττομαι", _, "~ττομεθᾰ";
    ind_mid_2: "~ττῃ|~ττει", "~ττεσθον", "~ττεσθε";
    ind_mid_3: "~ττεται", _, "~ττονται";
    subj_act_1: "~ττω", _, "~ττωμεν";
    subj_act_2: "~ττῃς", "~ττητον", "~ττητε";
    subj_act_3: "~ττῃ", _, "~ττωσῐ|~ττωσῐν";
    subj_mid_1: "~ττωμαι", _, "~ττωμεθᾰ";
    subj_mid_2: _, "~ττησθον", "~ττησθε";
    subj_mid_3: "~ττηται", _, "~ττωνται";
    opt_act_1: "~ττοιμῐ", _, "~ττοιμεν";
    opt_act_2: "~ττοις", "~ττοιτον", "~ττοιτε";
    opt_act_3: "~ττοι", "~ττοιτην", "~ττοιεν";
    opt_mid_1: "~ττοιμην", _, "~ττοιμεθᾰ";
    opt_mid_2: "~ττοιο", "~ττοισθον", "~ττοισθε";
    opt_mid_3: "~ττοιτο", "~ττοισθην", "~ττοιντο";
    imp_act_2: "~ττε", _, _;
    imp_act_3: "~ττετω", "~ττετων", "~ττοντων";
    imp_mid_2: "~ττου", _, _;
    imp_mid_3: "~ττεσθω", "~ττεσθων", _;
    imperfect_act_1: "ἐ~ττον", _, "ἐ~ττομεν";
    imperfect_act_2: "ἐ~ττες", "ἐ~ττετον", "ἐ~ττετε";
    imperfect_act_3: "ἐ~ττε|ἐ~ττεν", "ἐ~ττετην", "ἐ~ττον";
    imperfect_mid_1: "ἐ~ττομην", _, "ἐ~ττομεθᾰ";
    imperfect_mid_2: "ἐ~ττου", "ἐ~ττεσθον", "ἐ~ττεσθε";
    imperfect_mid_3: "ἐ~ττετο", "ἐ~ττεσθην", "ἐ~ττοντο";
    future_ind_act_3: _, _, "~ξουσῐ|~ξουσῐν";
    future_ind_mid_2: "~ξῃ|~ξει", _, _;
    future_ind_pass_1: "~γησομαι", _, "~γησομεθᾰ";
    future_ind_pass_2: "~γησῃ", "~γησεσθον", "~γησεσθε";
    future_ind_pass_3: "~γησεται", _, "~γησονται";
    future_opt_pass_1: "~γησοιμην", _, "~γησοιμεθᾰ";
    future_opt_pass_2: "~γησοιο", "~γησοισθον", "~γησοισθε";
    future_opt_pass_3: "~γησοιτο", "~γησοισθην", "~γησοιντο";
    aorist_subj_pass_1: "~γω", _, "~γωμεν";
    aorist_subj_pass_2: "~γῃς", "~γητον", "~γητε";
    aorist_subj_pass_3: "~γῃ", _, "~γωσῐ";
    aorist_opt_pass_1: "~γειην", _, "~γεῑμεν";
    aorist_opt_pass_2: "~γειης", "~γεῑτον", "~γεῑτε";
    aorist_opt_pass_3: "~γειη", "~γειτην", "~γεῑεν";
    aorist_imp_pass_2: "~γηθῐ", _, _;
    aorist_imp_pass_3: "~γητω", "~γητων", "~γεντων";
    perfect_ind_act_1: "%ε~γᾰ", _, "%ε~γᾰμεν";
    perfect_ind_act_2: "%ε~γᾰς", "%ε~γᾰτον", "%ε~γᾰτε";
    perfect_ind_act_3: "%ε~γε", _, "%ε~γᾱσῐ";
    perfect_subj_act_1: "%ε~γω", _, "%ε~γωμεν";
    perfect_subj_act_2: "%ε~γῃς", "%ε~γητον", "%ε~γητε";
    perfect_subj_act_3: "%ε~γῃ", _, "%ε~γωσῐ";
    perfect_opt_act_1: "%ε~γοιμῐ", _, "%ε~γοιμεν";
    perfect_opt_act_2: "%ε~γοις", "%ε~γοιτον", "%ε~γοιτε";
    perfect_opt_act_3: "%ε~γοι", "%ε~γοιτην", "%ε~γοιεν";
    perfect_imp_act_2: _, "%ε~γετον", "%ε~γετε";
    perfect_imp_act_3: "%ε~γετω", "%ε~γετων", "%ε~γοντων";
    infinitive_pres: "~ττειν", "~ττεσθαι", _;
    infinitive_fut: _, _, "~γησεσθαι";
    infinitive_aor: _, _, "~γηναι";
    infinitive_perf: "%ε~γεναι", _, _;
    participle_pres: "~ττων", "~ττομενος", _;
    participle_fut: _, _, "~γησομενος";
    participle_aor: _, _, "~γεις";
    participle_perf: "%ε~γως", _, _;
    ),
    // Ἐργᾰζομαι: 4 words (στοχᾰζομαι, ἐργᾰζομαι, ἀσπᾰζομαι, βῐᾰζομαι). Like Φρᾰζω, except in 59 cells: ind_act_1 sg, ind_mid_2 sg, subj_mid_2 sg, subj_pass_2 sg, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("ἐργᾰζομαι", 4, "~ζομαι", like Φρᾰζω as ACT_AS_PASS;
    ind_act_1: "~ζομαι", _, _;
    ind_mid_2: "~ζῃ", _, _;
    subj_mid_2: "~ζῃ", _, _;
    subj_pass_2: "~ζῃ", _, _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_2: "~σῃ", _, _;
    future_ind_pass_2: "~σθησῃ", _, _;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_subj_act_1: derived, _, derived;
    aorist_subj_act_2: derived, derived, derived;
    aorist_subj_act_3: derived, _, derived;
    aorist_subj_mid_2: "~σῃ", _, _;
    aorist_subj_pass_3: _, _, "~σθωσῐ";
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_pass_1: _, _, "~σθεῑμεν";
    aorist_opt_pass_2: _, "~σθεῑτον", "~σθεῑτε";
    aorist_opt_pass_3: _, "~σθειτην", "~σθεῑεν";
    aorist_imp_act_2: derived, _, _;
    aorist_imp_act_3: derived, derived, derived;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    participle_fut: derived, _, _;
    participle_aor: derived, _, _;
    ),
    // Ἐρᾰμαι: 4 words (ἐρᾰμαι, μᾰρνᾰμαι, ἀγᾰμαι, λῐνᾰμαι). Like Ἱημῐ, except in 3 cells: ind_act_1 sg, subj_mid_2 sg, subj_pass_2 sg.
    conjugation!("ἐρᾰμαι", 4, "~ᾰμαι", like Ἱημῐ as ACT_AS_PASS;
    ind_act_1: "~ᾰμαι", _, _;
    subj_mid_2: "~ῃ", _, _;
    subj_pass_2: "~ῃ", _, _;
    ),
    // Ἀνᾰπτω: 4 words (ἀνᾰπτω, ἐξᾰπτω, καθᾰπτω, ἐπῐσῠνᾰπτω). Like Ληγω, except in 206 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀνᾰπτω", 4, "~ᾰπτω", like Ληγω;
    stem aorist_ind: "~ηψ- like εὐλογεω";
    stem aorist_ind_pass: "~ηφθη- like ἐλπω";
    ind_act_1: "~ᾰπτω", _, "~ᾰπτομεν";
    ind_act_2: "~ᾰπτεις", "~ᾰπτετον", "~ᾰπτετε";
    ind_act_3: "~ᾰπτει", _, "~ᾰπτουσῐ|~ᾰπτουσῐν";
    ind_mid_1: "~ᾰπτομαι", _, "~ᾰπτομεθᾰ";
    ind_mid_2: "~ᾰπτῃ|~ᾰπτει", "~ᾰπτεσθον", "~ᾰπτεσθε";
    ind_mid_3: "~ᾰπτεται", _, "~ᾰπτονται";
    subj_act_1: "~ᾰπτω", _, "~ᾰπτωμεν";
    subj_act_2: "~ᾰπτῃς", "~ᾰπτητον", "~ᾰπτητε";
    subj_act_3: "~ᾰπτῃ", _, "~ᾰπτωσῐ|~ᾰπτωσῐν";
    subj_mid_1: "~ᾰπτωμαι", _, "~ᾰπτωμεθᾰ";
    subj_mid_2: _, "~ᾰπτησθον", "~ᾰπτησθε";
    subj_mid_3: "~ᾰπτηται", _, "~ᾰπτωνται";
    opt_act_1: "~ᾰπτοιμῐ", _, "~ᾰπτοιμεν";
    opt_act_2: "~ᾰπτοις", "~ᾰπτοιτον", "~ᾰπτοιτε";
    opt_act_3: "~ᾰπτοι", "~ᾰπτοιτην", "~ᾰπτοιεν";
    opt_mid_1: "~ᾰπτοιμην", _, "~ᾰπτοιμεθᾰ";
    opt_mid_2: "~ᾰπτοιο", "~ᾰπτοισθον", "~ᾰπτοισθε";
    opt_mid_3: "~ᾰπτοιτο", "~ᾰπτοισθην", "~ᾰπτοιντο";
    imp_act_2: "~ᾰπτε", _, _;
    imp_act_3: "~ᾰπτετω", "~ᾰπτετων", "~ᾰπτοντων";
    imp_mid_2: "~ᾰπτου", _, _;
    imp_mid_3: "~ᾰπτεσθω", "~ᾰπτεσθων", _;
    imperfect_act_1: "~ηπτον", _, "~ηπτομεν";
    imperfect_act_2: "~ηπτες", "~ηπτετον", "~ηπτετε";
    imperfect_act_3: "~ηπτε|~ηπτεν", "~ηπτετην", "~ηπτον";
    imperfect_mid_1: "~ηπτομην", _, "~ηπτομεθᾰ";
    imperfect_mid_2: "~ηπτου", "~ηπτεσθον", "~ηπτεσθε";
    imperfect_mid_3: "~ηπτετο", "~ηπτεσθην", "~ηπτοντο";
    future_ind_act_1: "~ᾰψω", _, "~ᾰψομεν";
    future_ind_act_2: "~ᾰψεις", "~ᾰψετον", "~ᾰψετε";
    future_ind_act_3: "~ᾰψει", _, "~ᾰψουσῐ|~ᾰψουσῐν";
    future_ind_mid_1: "~ᾰψομαι", _, "~ᾰψομεθᾰ";
    future_ind_mid_2: "~ᾰψῃ|~ᾰψει", "~ᾰψεσθον", "~ᾰψεσθε";
    future_ind_mid_3: "~ᾰψεται", _, "~ᾰψονται";
    future_ind_pass_1: "~ᾰφθησομαι", _, "~ᾰφθησομεθᾰ";
    future_ind_pass_2: "~ᾰφθησῃ", "~ᾰφθησεσθον", "~ᾰφθησεσθε";
    future_ind_pass_3: "~ᾰφθησεται", _, "~ᾰφθησονται";
    future_opt_act_1: "~ᾰψοιμῐ", _, "~ᾰψοιμεν";
    future_opt_act_2: "~ᾰψοις", "~ᾰψοιτον", "~ᾰψοιτε";
    future_opt_act_3: "~ᾰψοι", "~ᾰψοιτην", "~ᾰψοιεν";
    future_opt_mid_1: "~ᾰψοιμην", _, "~ᾰψοιμεθᾰ";
    future_opt_mid_2: "~ᾰψοιο", "~ᾰψοισθον", "~ᾰψοισθε";
    future_opt_mid_3: "~ᾰψοιτο", "~ᾰψοισθην", "~ᾰψοιντο";
    future_opt_pass_1: "~ᾰφθησοιμην", _, "~ᾰφθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰφθησοιο", "~ᾰφθησοισθον", "~ᾰφθησοισθε";
    future_opt_pass_3: "~ᾰφθησοιτο", "~ᾰφθησοισθην", "~ᾰφθησοιντο";
    aorist_subj_act_1: "~ᾰψω", _, "~ᾰψωμεν";
    aorist_subj_act_2: "~ᾰψῃς", "~ᾰψητον", "~ᾰψητε";
    aorist_subj_act_3: "~ᾰψῃ", _, "~ᾰψωσῐ|~ᾰψωσῐν";
    aorist_subj_mid_1: "~ᾰψωμαι", _, "~ᾰψωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰψησθον", "~ᾰψησθε";
    aorist_subj_mid_3: "~ᾰψηται", _, "~ᾰψωνται";
    aorist_subj_pass_1: "~ᾰφθω", _, "~ᾰφθωμεν";
    aorist_subj_pass_2: "~ᾰφθῃς", "~ᾰφθητον", "~ᾰφθητε";
    aorist_subj_pass_3: "~ᾰφθῃ", _, "~ᾰφθωσῐ|~ᾰφθωσῐν";
    aorist_opt_act_1: "~ᾰψαιμῐ", _, "~ᾰψαιμεν";
    aorist_opt_act_2: "~ᾰψειᾰς|~ᾰψαις", "~ᾰψαιτον", "~ᾰψαιτε";
    aorist_opt_act_3: "~ᾰψειε|~ᾰψειεν|~ᾰψαι", "~ᾰψαιτην", "~ᾰψειᾰν|~ᾰψαιεν";
    aorist_opt_mid_1: "~ᾰψαιμην", _, "~ᾰψαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰψαιο", "~ᾰψαισθον", "~ᾰψαισθε";
    aorist_opt_mid_3: "~ᾰψαιτο", "~ᾰψαισθην", "~ᾰψαιντο";
    aorist_opt_pass_1: "~ᾰφθειην", _, "~ᾰφθεῑμεν|~ᾰφθειημεν";
    aorist_opt_pass_2: "~ᾰφθειης", "~ᾰφθεῑτον|~ᾰφθειητον", "~ᾰφθεῑτε|~ᾰφθειητε";
    aorist_opt_pass_3: "~ᾰφθειη", "~ᾰφθειτην|~ᾰφθειητην", "~ᾰφθεῑεν|~ᾰφθειησᾰν";
    aorist_imp_act_2: "~ᾰψον", "~ᾰψᾰτον", "~ᾰψᾰτε";
    aorist_imp_act_3: "~ᾰψᾰτω", "~ᾰψᾰτων", "~ᾰψᾰντων";
    aorist_imp_mid_2: "~ᾰψαι", "~ᾰψᾰσθον", "~ᾰψᾰσθε";
    aorist_imp_mid_3: "~ᾰψᾰσθω", "~ᾰψᾰσθων", _;
    aorist_imp_pass_2: "~ᾰφθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰφθητω", "~ᾰφθητων", "~ᾰφθεντων";
    perfect_ind_mid_1: "~ημμαι", _, "~ημμεθᾰ";
    perfect_ind_mid_2: "~ηψαι", "~ηφθον", "~ηφθε";
    perfect_ind_mid_3: "~ηπται", _, "~ηπᾰται";
    perfect_imp_mid_2: "~ηψο", _, _;
    perfect_imp_mid_3: "~ηφθω", "~ηφθων", _;
    pluperfect_mid_1: "~ημμην", _, "~ημμεθᾰ";
    pluperfect_mid_2: "~ηψο", "~ηφθον", "~ηφθε";
    pluperfect_mid_3: "~ηπτο", "~ηφθην", "~ηπᾰτο";
    infinitive_pres: "~ᾰπτειν", "~ᾰπτεσθαι", _;
    infinitive_fut: "~ᾰψειν", "~ᾰψεσθαι", "~ᾰφθησεσθαι";
    infinitive_aor: "~ᾰψαι", "~ᾰψᾰσθαι", "~ᾰφθηναι";
    infinitive_perf: _, "~ηφθαι", _;
    participle_pres: "~ᾰπτων", "~ᾰπτομενος", _;
    participle_fut: "~ᾰψων", "~ᾰψομενος", "~ᾰφθησομενος";
    participle_aor: "~ᾰψᾱς", "~ᾰψᾰμενος", "~ᾰφθεις";
    participle_perf: _, "~ημμενος", _;
    ),
    // Πειρᾰω: 4 words (κοπῐᾰω, ἀᾰω, πειρᾰω, μειδῐᾰω). Like Ἐαω, except in 30 cells: future_ind_mid_2 sg, future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, future_ind_pass_2 du, future_ind_pass_2 pl, ….
    conjugation!("πειρᾰω", 4, "~ᾰω", like Ἐαω;
    stem pluperfect: "~ᾱ- like πειρᾰω";
    future_ind_mid_2: "~ᾱσῃ", _, _;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    perfect_ind_mid_1: "%ε~ᾱμαι", _, "%ε~ᾱμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾱσαι", "%ε~ᾱσθον", "%ε~ᾱσθε";
    perfect_ind_mid_3: "%ε~ᾱται", _, "%ε~ᾱνται";
    perfect_imp_mid_2: "%ε~ᾱσο", _, _;
    perfect_imp_mid_3: "%ε~ᾱσθω", "%ε~ᾱσθων", _;
    infinitive_fut: _, _, derived;
    infinitive_perf: _, "%ε~ᾱσθαι", _;
    participle_fut: _, _, derived;
    participle_perf: _, "%ε~ᾱμενος", _;
    ),
    // Πᾰρᾰλλᾰσσω: 4 words (ἀπᾰλλᾰσσω, ὑπᾰλλᾰσσω, ἀντᾰλλᾰσσω, πᾰρᾰλλᾰσσω). Like Ξαινω, except in 228 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("πᾰρᾰλλᾰσσω", 4, "~ᾰ~σσω", like Ξαινω;
    stem aorist_ind: "~η~ξ- like ἐλπω";
    stem aorist_ind_pass: "~η~- like πᾰρᾰλλᾰσσω";
    ind_act_1: "~ᾰ~σσω", _, "~ᾰ~σσομεν";
    ind_act_2: "~ᾰ~σσεις", "~ᾰ~σσετον", "~ᾰ~σσετε";
    ind_act_3: "~ᾰ~σσει", _, "~ᾰ~σσουσῐ|~ᾰ~σσουσῐν";
    ind_mid_1: "~ᾰ~σσομαι", _, "~ᾰ~σσομεθᾰ";
    ind_mid_2: "~ᾰ~σσῃ|~ᾰ~σσει", "~ᾰ~σσεσθον", "~ᾰ~σσεσθε";
    ind_mid_3: "~ᾰ~σσεται", _, "~ᾰ~σσονται";
    subj_act_1: "~ᾰ~σσω", _, "~ᾰ~σσωμεν";
    subj_act_2: "~ᾰ~σσῃς", "~ᾰ~σσητον", "~ᾰ~σσητε";
    subj_act_3: "~ᾰ~σσῃ", _, "~ᾰ~σσωσῐ|~ᾰ~σσωσῐν";
    subj_mid_1: "~ᾰ~σσωμαι", _, "~ᾰ~σσωμεθᾰ";
    subj_mid_2: _, "~ᾰ~σσησθον", "~ᾰ~σσησθε";
    subj_mid_3: "~ᾰ~σσηται", _, "~ᾰ~σσωνται";
    opt_act_1: "~ᾰ~σσοιμῐ", _, "~ᾰ~σσοιμεν";
    opt_act_2: "~ᾰ~σσοις", "~ᾰ~σσοιτον", "~ᾰ~σσοιτε";
    opt_act_3: "~ᾰ~σσοι", "~ᾰ~σσοιτην", "~ᾰ~σσοιεν";
    opt_mid_1: "~ᾰ~σσοιμην", _, "~ᾰ~σσοιμεθᾰ";
    opt_mid_2: "~ᾰ~σσοιο", "~ᾰ~σσοισθον", "~ᾰ~σσοισθε";
    opt_mid_3: "~ᾰ~σσοιτο", "~ᾰ~σσοισθην", "~ᾰ~σσοιντο";
    imp_act_2: "~ᾰ~σσε", _, _;
    imp_act_3: "~ᾰ~σσετω", "~ᾰ~σσετων", "~ᾰ~σσοντων";
    imp_mid_2: "~ᾰ~σσου", _, _;
    imp_mid_3: "~ᾰ~σσεσθω", "~ᾰ~σσεσθων", _;
    imperfect_act_1: "~η~σσον", _, "~η~σσομεν";
    imperfect_act_2: "~η~σσες", "~η~σσετον", "~η~σσετε";
    imperfect_act_3: "~η~σσε|~η~σσεν", "~η~σσετην", "~η~σσον";
    imperfect_mid_1: "~η~σσομην", _, "~η~σσομεθᾰ";
    imperfect_mid_2: "~η~σσου", "~η~σσεσθον", "~η~σσεσθε";
    imperfect_mid_3: "~η~σσετο", "~η~σσεσθην", "~η~σσοντο";
    future_ind_act_1: "~ᾰ~ξω", _, "~ᾰ~ξομεν";
    future_ind_act_2: "~ᾰ~ξεις", "~ᾰ~ξετον", "~ᾰ~ξετε";
    future_ind_act_3: "~ᾰ~ξει", _, "~ᾰ~ξουσῐ|~ᾰ~ξουσῐν";
    future_ind_mid_1: "~ᾰ~ξομαι", _, "~ᾰ~ξομεθᾰ";
    future_ind_mid_2: "~ᾰ~ξῃ|~ᾰ~ξει", "~ᾰ~ξεσθον", "~ᾰ~ξεσθε";
    future_ind_mid_3: "~ᾰ~ξεται", _, "~ᾰ~ξονται";
    future_ind_pass_1: "~ᾰ~χθησομαι", _, "~ᾰ~χθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~χθησῃ|~ᾰ~χθησει", "~ᾰ~χθησεσθον", "~ᾰ~χθησεσθε";
    future_ind_pass_3: "~ᾰ~χθησεται", _, "~ᾰ~χθησονται";
    future_opt_act_1: "~ᾰ~ξοιμῐ", _, "~ᾰ~ξοιμεν";
    future_opt_act_2: "~ᾰ~ξοις", "~ᾰ~ξοιτον", "~ᾰ~ξοιτε";
    future_opt_act_3: "~ᾰ~ξοι", "~ᾰ~ξοιτην", "~ᾰ~ξοιεν";
    future_opt_mid_1: "~ᾰ~ξοιμην", _, "~ᾰ~ξοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ξοιο", "~ᾰ~ξοισθον", "~ᾰ~ξοισθε";
    future_opt_mid_3: "~ᾰ~ξοιτο", "~ᾰ~ξοισθην", "~ᾰ~ξοιντο";
    future_opt_pass_1: "~ᾰ~χθησοιμην", _, "~ᾰ~χθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~χθησοιο", "~ᾰ~χθησοισθον", "~ᾰ~χθησοισθε";
    future_opt_pass_3: "~ᾰ~χθησοιτο", "~ᾰ~χθησοισθην", "~ᾰ~χθησοιντο";
    aorist_subj_act_1: "~ᾰ~ξω", _, "~ᾰ~ξωμεν";
    aorist_subj_act_2: "~ᾰ~ξῃς", "~ᾰ~ξητον", "~ᾰ~ξητε";
    aorist_subj_act_3: "~ᾰ~ξῃ", _, "~ᾰ~ξωσῐ";
    aorist_subj_mid_1: "~ᾰ~ξωμαι", _, "~ᾰ~ξωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰ~ξησθον", "~ᾰ~ξησθε";
    aorist_subj_mid_3: "~ᾰ~ξηται", _, "~ᾰ~ξωνται";
    aorist_subj_pass_1: "~ᾰ~χθω|~ᾰ~γω", _, "~ᾰ~χθωμεν|~ᾰ~γωμεν";
    aorist_subj_pass_2: "~ᾰ~χθῃς|~ᾰ~γῃς", "~ᾰ~χθητον|~ᾰ~γητον", "~ᾰ~χθητε|~ᾰ~γητε";
    aorist_subj_pass_3: "~ᾰ~χθῃ|~ᾰ~γῃ", _, "~ᾰ~χθωσῐ|~ᾰ~χθωσῐν|~ᾰ~γωσῐ|~ᾰ~γωσῐν";
    aorist_opt_act_1: "~ᾰ~ξαιμῐ", _, "~ᾰ~ξαιμεν";
    aorist_opt_act_2: "~ᾰ~ξειᾰς", "~ᾰ~ξαιτον", "~ᾰ~ξαιτε";
    aorist_opt_act_3: "~ᾰ~ξειε", "~ᾰ~ξαιτην", "~ᾰ~ξειᾰν";
    aorist_opt_mid_1: "~ᾰ~ξαιμην", _, "~ᾰ~ξαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~ξαιο", "~ᾰ~ξαισθον", "~ᾰ~ξαισθε";
    aorist_opt_mid_3: "~ᾰ~ξαιτο", "~ᾰ~ξαισθην", "~ᾰ~ξαιντο";
    aorist_opt_pass_1: "~ᾰ~χθειην|~ᾰ~γειην", _, "~ᾰ~χθεῑμεν|~ᾰ~χθειημεν|~ᾰ~γεῑμεν|~ᾰ~γειημεν";
    aorist_opt_pass_2: "~ᾰ~χθειης|~ᾰ~γειης", "~ᾰ~χθεῑτον|~ᾰ~χθειητον|~ᾰ~γεῑτον|~ᾰ~γειητον", "~ᾰ~χθεῑτε|~ᾰ~χθειητε|~ᾰ~γεῑτε|~ᾰ~γειητε";
    aorist_opt_pass_3: "~ᾰ~χθειη|~ᾰ~γειη", "~ᾰ~χθειτην|~ᾰ~χθειητην|~ᾰ~γειτην|~ᾰ~γειητην", "~ᾰ~χθεῑεν|~ᾰ~χθειησᾰν|~ᾰ~γεῑεν|~ᾰ~γειησᾰν";
    aorist_imp_act_2: "~ᾰ~ξον", "~ᾰ~ξᾰτον", "~ᾰ~ξᾰτε";
    aorist_imp_act_3: "~ᾰ~ξᾰτω", "~ᾰ~ξᾰτων", "~ᾰ~ξᾰντων";
    aorist_imp_mid_2: "~ᾰ~ξαι", "~ᾰ~ξᾰσθον", "~ᾰ~ξᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ξᾰσθω", "~ᾰ~ξᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~χθητῐ|~ᾰ~γηθῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~χθητω|~ᾰ~γητω", "~ᾰ~χθητων|~ᾰ~γητων", "~ᾰ~χθεντων|~ᾰ~γεντων";
    perfect_ind_act_1: "~η~χᾰ", _, "~η~χᾰμεν";
    perfect_ind_act_2: "~η~χᾰς", "~η~χᾰτον", "~η~χᾰτε";
    perfect_ind_act_3: "~η~χε|~η~χεν", _, "~η~χᾱσῐ|~η~χᾱσῐν";
    perfect_ind_mid_1: "~η~γμαι", _, "~η~γμεθᾰ";
    perfect_ind_mid_2: "~η~ξαι", "~η~χθον", "~η~χθε";
    perfect_ind_mid_3: "~η~κται", _, "~η~χᾰται";
    perfect_subj_act_1: "~η~χω", _, "~η~χωμεν";
    perfect_subj_act_2: "~η~χῃς", "~η~χητον", "~η~χητε";
    perfect_subj_act_3: "~η~χῃ", _, "~η~χωσῐ|~η~χωσῐν";
    perfect_opt_act_1: "~η~χοιμῐ|~η~χοιην", _, "~η~χοιμεν";
    perfect_opt_act_2: "~η~χοις|~η~χοιης", "~η~χοιτον", "~η~χοιτε";
    perfect_opt_act_3: "~η~χοι|~η~χοιη", "~η~χοιτην", "~η~χοιεν";
    perfect_imp_act_2: "~η~χε", "~η~χετον", "~η~χετε";
    perfect_imp_act_3: "~η~χετω", "~η~χετων", "~η~χοντων";
    perfect_imp_mid_2: "~η~ξο", _, _;
    perfect_imp_mid_3: "~η~χθω", "~η~χθων", _;
    infinitive_pres: "~ᾰ~σσειν", "~ᾰ~σσεσθαι", _;
    infinitive_fut: "~ᾰ~ξειν", "~ᾰ~ξεσθαι", "~ᾰ~χθησεσθαι";
    infinitive_aor: "~ᾰ~ξαι", "~ᾰ~ξᾰσθαι", "~ᾰ~χθηναι|~ᾰ~γηναι";
    infinitive_perf: "~η~χεναι", "~η~χθαι", _;
    participle_pres: "~ᾰ~σσων", "~ᾰ~σσομενος", _;
    participle_fut: "~ᾰ~ξων", "~ᾰ~ξομενος", "~ᾰ~χθησομενος";
    participle_aor: "~ᾰ~ξᾱς", "~ᾰ~ξᾰμενος", "~ᾰ~χθεις|~ᾰ~γεις";
    participle_perf: "~η~χως", "~η~γμενος", _;
    ),
    // Κᾰτειργω: 4 words (ἐργω, εἰργω, ἀμεργω, κᾰτειργω). Like Ληγω, except in 41 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, ….
    conjugation!("κᾰτειργω", 4, "~γω", like Ληγω;
    stem aorist_ind: "~ξ- like ἐλπω";
    stem aorist_ind_pass: "~χθη- like ἐλπω";
    ind_act_3: _, _, "~γουσῐ|~γουσῐν";
    ind_mid_2: "~γῃ|~γει", _, _;
    subj_act_3: _, _, "~γωσῐ|~γωσῐν";
    imperfect_act_1: "~γον", _, "~γομεν";
    imperfect_act_2: "~γες", "~γετον", "~γετε";
    imperfect_act_3: "~γε", "~γετην", "~γον";
    imperfect_mid_1: "~γομην", _, "~γομεθᾰ";
    imperfect_mid_2: "~γου", "~γεσθον", "~γεσθε";
    imperfect_mid_3: "~γετο", "~γεσθην", "~γοντο";
    future_ind_act_3: _, _, "~ξουσῐ|~ξουσῐν";
    future_ind_mid_2: "~ξῃ|~ξει", _, _;
    perfect_ind_mid_1: "~γμαι", _, "~γμεθᾰ";
    perfect_ind_mid_2: "~ξαι", "~χθον", "~χθε";
    perfect_ind_mid_3: "~κται", _, "~χᾰται";
    perfect_imp_mid_2: "~ξο", _, _;
    perfect_imp_mid_3: "~χθω", "~χθων", _;
    pluperfect_mid_1: "~γμην", _, "~γμεθᾰ";
    pluperfect_mid_2: "~ξο", "~χθον", "~χθε";
    pluperfect_mid_3: "~κτο", "~χθην", "~χᾰτο";
    infinitive_perf: _, "~χθαι", _;
    participle_perf: _, "~γμενος", _;
    ),
    // Περαινω: 4 words (μᾰραινω, πημαινω, πῑαινω, περαινω). Like Ῥαινω, except in 80 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imp_act_2 sg, imperfect_act_1 sg, imperfect_act_1 pl, ….
    conjugation!("περαινω", 4, "~αινω", like Ῥαινω;
    stem aorist_ind_pass: "~ᾰνθη- like ἐχω";
    ind_act_3: _, _, "~αινουσῐ|~αινουσῐν";
    ind_mid_2: "~αινῃ|~αινει", _, _;
    subj_act_3: _, _, "~αινωσῐ|~αινωσῐν";
    imp_act_2: "~αινε", _, _;
    imperfect_act_1: "ε~αινον", _, "ε~αινομεν";
    imperfect_act_2: "ε~αινες", "ε~αινετον", "ε~αινετε";
    imperfect_act_3: "ε~αινε", "ε~αινετην", "ε~αινον";
    imperfect_mid_1: "ε~αινομην", _, "ε~αινομεθᾰ";
    imperfect_mid_2: "ε~αινου", "ε~αινεσθον", "ε~αινεσθε";
    imperfect_mid_3: "ε~αινετο", "ε~αινεσθην", "ε~αινοντο";
    future_ind_act_3: _, _, "~ᾰνοῡσῐ|~ᾰνοῡσῐν";
    future_ind_mid_2: "~ᾰνεῑ|~ᾰνῃ", _, _;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_act_1: "~ᾰνοιην|~ᾰνοῑμῐ", _, "~ᾰνοῑμεν|~ᾰνοιημεν";
    future_opt_act_2: "~ᾰνοιης|~ᾰνοῑς", "~ᾰνοῑτον|~ᾰνοιητον", "~ᾰνοῑτε|~ᾰνοιητε";
    future_opt_act_3: "~ᾰνοιη|~ᾰνοῑ", "~ᾰνοιτην|~ᾰνοιητην", "~ᾰνοῑεν|~ᾰνοιησᾰν";
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_subj_pass_1: "~ᾰνθω", _, "~ᾰνθωμεν";
    aorist_subj_pass_2: "~ᾰνθῃς", "~ᾰνθητον", "~ᾰνθητε";
    aorist_subj_pass_3: "~ᾰνθῃ", _, "~ᾰνθωσῐ|~ᾰνθωσῐν";
    aorist_opt_pass_1: "~ᾰνθειην", _, "~ᾰνθεῑμεν|~ᾰνθειημεν";
    aorist_opt_pass_2: "~ᾰνθειης", "~ᾰνθεῑτον|~ᾰνθειητον", "~ᾰνθεῑτε|~ᾰνθειητε";
    aorist_opt_pass_3: "~ᾰνθειη", "~ᾰνθειτην|~ᾰνθειητην", "~ᾰνθεῑεν|~ᾰνθειησᾰν";
    aorist_imp_pass_2: "~ᾰνθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰνθητω", "~ᾰνθητων", "~ᾰνθεντων";
    perfect_ind_mid_1: "%ε~ᾰσμαι", _, "%ε~ᾰσμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾰνσαι", "%ε~ᾰνθον", "%ε~ᾰνθε";
    perfect_ind_mid_3: "%ε~ᾰνται", _, "%ε~ᾰνᾰται";
    perfect_imp_mid_2: "%ε~ᾰνσο", _, _;
    perfect_imp_mid_3: "%ε~ᾰνθω", "%ε~ᾰνθων", _;
    infinitive_fut: _, _, derived;
    infinitive_aor: _, _, "~ᾰνθηναι";
    infinitive_perf: _, "%ε~ᾰνθαι", _;
    participle_fut: _, _, derived;
    participle_aor: _, _, "~ᾰνθεις";
    participle_perf: _, "%ε~ᾰσμενος", _;
    ),
    // Πᾰροξῡνω: 4 words (παροτρῡνω, ἐποτρῡνω, πᾰροξῡνω, ἐξοτρῡνω). Like Ξαινω, except in 227 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("πᾰροξῡνω", 4, "~ο~ῡνω", like Ξαινω;
    stem aorist_ind: "~ω~ῡν- like ἐλπω";
    stem aorist_ind_pass: "~ω~ῠνθη- like ἐλπω";
    ind_act_1: "~ο~ῡνω", _, "~ο~ῡνομεν";
    ind_act_2: "~ο~ῡνεις", "~ο~ῡνετον", "~ο~ῡνετε";
    ind_act_3: "~ο~ῡνει", _, "~ο~ῡνουσῐ";
    ind_mid_1: "~ο~ῡνομαι", _, "~ο~ῡνομεθᾰ";
    ind_mid_2: "~ο~ῡνῃ", "~ο~ῡνεσθον", "~ο~ῡνεσθε";
    ind_mid_3: "~ο~ῡνεται", _, "~ο~ῡνονται";
    subj_act_1: "~ο~ῡνω", _, "~ο~ῡνωμεν";
    subj_act_2: "~ο~ῡνῃς", "~ο~ῡνητον", "~ο~ῡνητε";
    subj_act_3: "~ο~ῡνῃ", _, "~ο~ῡνωσῐ";
    subj_mid_1: "~ο~ῡνωμαι", _, "~ο~ῡνωμεθᾰ";
    subj_mid_2: _, "~ο~ῡνησθον", "~ο~ῡνησθε";
    subj_mid_3: "~ο~ῡνηται", _, "~ο~ῡνωνται";
    opt_act_1: "~ο~ῡνοιμῐ", _, "~ο~ῡνοιμεν";
    opt_act_2: "~ο~ῡνοις", "~ο~ῡνοιτον", "~ο~ῡνοιτε";
    opt_act_3: "~ο~ῡνοι", "~ο~ῡνοιτην", "~ο~ῡνοιεν";
    opt_mid_1: "~ο~ῡνοιμην", _, "~ο~ῡνοιμεθᾰ";
    opt_mid_2: "~ο~ῡνοιο", "~ο~ῡνοισθον", "~ο~ῡνοισθε";
    opt_mid_3: "~ο~ῡνοιτο", "~ο~ῡνοισθην", "~ο~ῡνοιντο";
    imp_act_2: "~ο~ῡνε", _, _;
    imp_act_3: "~ο~ῡνετω", "~ο~ῡνετων", "~ο~ῡνοντων";
    imp_mid_2: "~ο~ῡνου", _, _;
    imp_mid_3: "~ο~ῡνεσθω", "~ο~ῡνεσθων", _;
    imperfect_act_1: "~ω~ῡνον", _, "~ω~ῡνομεν";
    imperfect_act_2: "~ω~ῡνες", "~ω~ῡνετον", "~ω~ῡνετε";
    imperfect_act_3: "~ω~ῡνε", "~ω~ῡνετην", "~ω~ῡνον";
    imperfect_mid_1: "~ω~ῡνομην", _, "~ω~ῡνομεθᾰ";
    imperfect_mid_2: "~ω~ῡνου", "~ω~ῡνεσθον", "~ω~ῡνεσθε";
    imperfect_mid_3: "~ω~ῡνετο", "~ω~ῡνεσθην", "~ω~ῡνοντο";
    future_ind_act_1: "~ο~ῠνω", _, "~ο~ῠνοῡμεν";
    future_ind_act_2: "~ο~ῠνεῑς", "~ο~ῠνεῑτον", "~ο~ῠνεῑτε";
    future_ind_act_3: "~ο~ῠνεῑ", _, "~ο~ῠνοῡσῐ";
    future_ind_mid_1: "~ο~ῠνοῡμαι", _, "~ο~ῠνουμεθᾰ";
    future_ind_mid_2: "~ο~ῠνεῑ", "~ο~ῠνεῑσθον", "~ο~ῠνεῑσθε";
    future_ind_mid_3: "~ο~ῠνεῑται", _, "~ο~ῠνοῡνται";
    future_ind_pass_1: "~ο~ῠνθησομαι", _, "~ο~ῠνθησομεθᾰ";
    future_ind_pass_2: "~ο~ῠνθησῃ", "~ο~ῠνθησεσθον", "~ο~ῠνθησεσθε";
    future_ind_pass_3: "~ο~ῠνθησεται", _, "~ο~ῠνθησονται";
    future_opt_act_1: "~ο~ῠνοιην", _, "~ο~ῠνοῑμεν";
    future_opt_act_2: "~ο~ῠνοιης", "~ο~ῠνοῑτον", "~ο~ῠνοῑτε";
    future_opt_act_3: "~ο~ῠνοιη", "~ο~ῠνοιτην", "~ο~ῠνοῑεν";
    future_opt_mid_1: "~ο~ῠνοιμην", _, "~ο~ῠνοιμεθᾰ";
    future_opt_mid_2: "~ο~ῠνοῑο", "~ο~ῠνοῑσθον", "~ο~ῠνοῑσθε";
    future_opt_mid_3: "~ο~ῠνοῑτο", "~ο~ῠνοισθην", "~ο~ῠνοῑντο";
    future_opt_pass_1: "~ο~ῠνθησοιμην", _, "~ο~ῠνθησοιμεθᾰ";
    future_opt_pass_2: "~ο~ῠνθησοιο", "~ο~ῠνθησοισθον", "~ο~ῠνθησοισθε";
    future_opt_pass_3: "~ο~ῠνθησοιτο", "~ο~ῠνθησοισθην", "~ο~ῠνθησοιντο";
    aorist_subj_act_1: "~ο~ῡνω", _, "~ο~ῡνωμεν";
    aorist_subj_act_2: "~ο~ῡνῃς", "~ο~ῡνητον", "~ο~ῡνητε";
    aorist_subj_act_3: "~ο~ῡνῃ", _, "~ο~ῡνωσῐ";
    aorist_subj_mid_1: "~ο~ῡνωμαι", _, "~ο~ῡνωμεθᾰ";
    aorist_subj_mid_2: _, "~ο~ῡνησθον", "~ο~ῡνησθε";
    aorist_subj_mid_3: "~ο~ῡνηται", _, "~ο~ῡνωνται";
    aorist_subj_pass_1: "~ο~ῠνθω", _, "~ο~ῠνθωμεν";
    aorist_subj_pass_2: "~ο~ῠνθῃς", "~ο~ῠνθητον", "~ο~ῠνθητε";
    aorist_subj_pass_3: "~ο~ῠνθῃ", _, "~ο~ῠνθωσῐ";
    aorist_opt_act_1: "~ο~ῡναιμῐ", _, "~ο~ῡναιμεν";
    aorist_opt_act_2: "~ο~ῡνειᾰς", "~ο~ῡναιτον", "~ο~ῡναιτε";
    aorist_opt_act_3: "~ο~ῡνειε", "~ο~ῡναιτην", "~ο~ῡνειᾰν";
    aorist_opt_mid_1: "~ο~ῡναιμην", _, "~ο~ῡναιμεθᾰ";
    aorist_opt_mid_2: "~ο~ῡναιο", "~ο~ῡναισθον", "~ο~ῡναισθε";
    aorist_opt_mid_3: "~ο~ῡναιτο", "~ο~ῡναισθην", "~ο~ῡναιντο";
    aorist_opt_pass_1: "~ο~ῠνθειην", _, "~ο~ῠνθεῑμεν";
    aorist_opt_pass_2: "~ο~ῠνθειης", "~ο~ῠνθεῑτον", "~ο~ῠνθεῑτε";
    aorist_opt_pass_3: "~ο~ῠνθειη", "~ο~ῠνθειτην", "~ο~ῠνθεῑεν";
    aorist_imp_act_2: "~ο~ῡνον", "~ο~ῡνᾰτον", "~ο~ῡνᾰτε";
    aorist_imp_act_3: "~ο~ῡνᾰτω", "~ο~ῡνᾰτων", "~ο~ῡνᾰντων";
    aorist_imp_mid_2: "~ο~ῡναι", "~ο~ῡνᾰσθον", "~ο~ῡνᾰσθε";
    aorist_imp_mid_3: "~ο~ῡνᾰσθω", "~ο~ῡνᾰσθων", _;
    aorist_imp_pass_2: "~ο~ῠνθητῐ", _, _;
    aorist_imp_pass_3: "~ο~ῠνθητω", "~ο~ῠνθητων", "~ο~ῠνθεντων";
    perfect_ind_act_1: "~ω~ῠγκᾰ", _, "~ω~ῠγκᾰμεν";
    perfect_ind_act_2: "~ω~ῠγκᾰς", "~ω~ῠγκᾰτον", "~ω~ῠγκᾰτε";
    perfect_ind_act_3: "~ω~ῠγκε", _, "~ω~ῠγκᾱσῐ";
    perfect_ind_mid_1: "~ω~ῠμμαι", _, "~ω~ῠμμεθᾰ";
    perfect_ind_mid_2: "~ω~ῠψαι", "~ω~ῠφθον", "~ω~ῠφθε";
    perfect_ind_mid_3: "~ω~ῠπται", _, "~ω~ῠπᾰται";
    perfect_subj_act_1: "~ω~ῠγκω", _, "~ω~ῠγκωμεν";
    perfect_subj_act_2: "~ω~ῠγκῃς", "~ω~ῠγκητον", "~ω~ῠγκητε";
    perfect_subj_act_3: "~ω~ῠγκῃ", _, "~ω~ῠγκωσῐ";
    perfect_opt_act_1: "~ω~ῠγκοιμῐ", _, "~ω~ῠγκοιμεν";
    perfect_opt_act_2: "~ω~ῠγκοις", "~ω~ῠγκοιτον", "~ω~ῠγκοιτε";
    perfect_opt_act_3: "~ω~ῠγκοι", "~ω~ῠγκοιτην", "~ω~ῠγκοιεν";
    perfect_imp_act_2: _, "~ω~ῠγκετον", "~ω~ῠγκετε";
    perfect_imp_act_3: "~ω~ῠγκετω", "~ω~ῠγκετων", "~ω~ῠγκοντων";
    perfect_imp_mid_2: "~ω~ῠψο", _, _;
    perfect_imp_mid_3: "~ω~ῠφθω", "~ω~ῠφθων", _;
    infinitive_pres: "~ο~ῡνειν", "~ο~ῡνεσθαι", _;
    infinitive_fut: "~ο~ῠνεῑν", "~ο~ῠνεῑσθαι", "~ο~ῠνθησεσθαι";
    infinitive_aor: "~ο~ῡναι", "~ο~ῡνᾰσθαι", "~ο~ῠνθηναι";
    infinitive_perf: "~ω~ῠγκεναι", "~ω~ῠφθαι", _;
    participle_pres: "~ο~ῡνων", "~ο~ῡνομενος", _;
    participle_fut: "~ο~ῠνων", "~ο~ῠνουμενος", "~ο~ῠνθησομενος";
    participle_aor: "~ο~ῡνᾱς", "~ο~ῡνᾰμενος", "~ο~ῠνθεις";
    participle_perf: "~ω~ῠγκως", "~ω~ῠμμενος", _;
    ),
    // Ἐλᾰσσοω: 4 words (ἐρημοω, ἐλᾰσσοω, ἐλευθεροω, ἐπουλοω). Like Ἐλαυνω, except in 225 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἐλᾰσσοω", 4, "ἐ~οω", like Ἐλαυνω;
    stem aorist_ind: "~ωσ- like ἐλπῐζω";
    stem aorist_ind_pass: "~ωθη- like ἀγορευω";
    stem pluperfect_act: "~ωκε- like ἀγορευω";
    ind_act_1: _, _, "ἐ~οῡμεν";
    ind_act_2: "ἐ~οῑς", "ἐ~οῡτον", "ἐ~οῡτε";
    ind_act_3: "ἐ~οῑ", _, "ἐ~οῡσῐ|ἐ~οῡσῐν";
    ind_mid_1: "ἐ~οῡμαι", _, "ἐ~ουμεθᾰ";
    ind_mid_2: "ἐ~οῑ", "ἐ~οῡσθον", "ἐ~οῡσθε";
    ind_mid_3: "ἐ~οῡται", _, "ἐ~οῡνται";
    subj_act_2: "ἐ~οῑς", "ἐ~ωτον", "ἐ~ωτε";
    subj_act_3: "ἐ~οῑ", _, "ἐ~ωσῐ|ἐ~ωσῐν";
    subj_mid_2: _, "ἐ~ωσθον", "ἐ~ωσθε";
    subj_mid_3: "ἐ~ωται", _, _;
    opt_act_1: "ἐ~οιην|ἐ~οῑμῐ", _, "ἐ~οῑμεν|ἐ~οιημεν";
    opt_act_2: "ἐ~οιης|ἐ~οῑς", "ἐ~οῑτον|ἐ~οιητον", "ἐ~οῑτε|ἐ~οιητε";
    opt_act_3: "ἐ~οιη|ἐ~οῑ", "ἐ~οιτην|ἐ~οιητην", "ἐ~οῑεν|ἐ~οιησᾰν";
    opt_mid_2: "ἐ~οῑο", "ἐ~οῑσθον", "ἐ~οῑσθε";
    opt_mid_3: "ἐ~οῑτο", _, "ἐ~οῑντο";
    imp_act_2: "ἐ~ου", _, _;
    imp_act_3: "ἐ~ουτω", "ἐ~ουτων", "ἐ~ουντων";
    imp_mid_2: "ἐ~οῡ", _, _;
    imp_mid_3: "ἐ~ουσθω", "ἐ~ουσθων", _;
    imperfect_act_1: "ἠ~ουν", _, "ἠ~οῡμεν";
    imperfect_act_2: "ἠ~ους", "ἠ~οῡτον", "ἠ~οῡτε";
    imperfect_act_3: "ἠ~ου", "ἠ~ουτην", "ἠ~ουν";
    imperfect_mid_1: "ἠ~ουμην", _, "ἠ~ουμεθᾰ";
    imperfect_mid_2: "ἠ~οῡ", "ἠ~οῡσθον", "ἠ~οῡσθε";
    imperfect_mid_3: "ἠ~οῡτο", "ἠ~ουσθην", "ἠ~οῡντο";
    future_ind_act_1: "ἐ~ωσω", _, "ἐ~ωσομεν";
    future_ind_act_2: "ἐ~ωσεις", "ἐ~ωσετον", "ἐ~ωσετε";
    future_ind_act_3: "ἐ~ωσει", _, "ἐ~ωσουσῐ|ἐ~ωσουσῐν";
    future_ind_mid_1: "ἐ~ωσομαι", _, "ἐ~ωσομεθᾰ";
    future_ind_mid_2: "ἐ~ωσῃ|ἐ~ωσει", "ἐ~ωσεσθον", "ἐ~ωσεσθε";
    future_ind_mid_3: "ἐ~ωσεται", _, "ἐ~ωσονται";
    future_ind_pass_1: "ἐ~ωθησομαι", _, "ἐ~ωθησομεθᾰ";
    future_ind_pass_2: "ἐ~ωθησῃ|ἐ~ωθησει", "ἐ~ωθησεσθον", "ἐ~ωθησεσθε";
    future_ind_pass_3: "ἐ~ωθησεται", _, "ἐ~ωθησονται";
    future_opt_act_1: "ἐ~ωσοιμῐ", _, "ἐ~ωσοιμεν";
    future_opt_act_2: "ἐ~ωσοις", "ἐ~ωσοιτον", "ἐ~ωσοιτε";
    future_opt_act_3: "ἐ~ωσοι", "ἐ~ωσοιτην", "ἐ~ωσοιεν";
    future_opt_mid_1: "ἐ~ωσοιμην", _, "ἐ~ωσοιμεθᾰ";
    future_opt_mid_2: "ἐ~ωσοιο", "ἐ~ωσοισθον", "ἐ~ωσοισθε";
    future_opt_mid_3: "ἐ~ωσοιτο", "ἐ~ωσοισθην", "ἐ~ωσοιντο";
    future_opt_pass_1: "ἐ~ωθησοιμην", _, "ἐ~ωθησοιμεθᾰ";
    future_opt_pass_2: "ἐ~ωθησοιο", "ἐ~ωθησοισθον", "ἐ~ωθησοισθε";
    future_opt_pass_3: "ἐ~ωθησοιτο", "ἐ~ωθησοισθην", "ἐ~ωθησοιντο";
    aorist_subj_act_1: "ἐ~ωσω", _, "ἐ~ωσωμεν";
    aorist_subj_act_2: "ἐ~ωσῃς", "ἐ~ωσητον", "ἐ~ωσητε";
    aorist_subj_act_3: "ἐ~ωσῃ", _, "ἐ~ωσωσῐ|ἐ~ωσωσῐν";
    aorist_subj_mid_1: "ἐ~ωσωμαι", _, "ἐ~ωσωμεθᾰ";
    aorist_subj_mid_2: _, "ἐ~ωσησθον", "ἐ~ωσησθε";
    aorist_subj_mid_3: "ἐ~ωσηται", _, "ἐ~ωσωνται";
    aorist_subj_pass_1: "ἐ~ωθω", _, "ἐ~ωθωμεν";
    aorist_subj_pass_2: "ἐ~ωθῃς", "ἐ~ωθητον", "ἐ~ωθητε";
    aorist_subj_pass_3: "ἐ~ωθῃ", _, "ἐ~ωθωσῐ|ἐ~ωθωσῐν";
    aorist_opt_act_1: "ἐ~ωσαιμῐ", _, "ἐ~ωσαιμεν";
    aorist_opt_act_2: "ἐ~ωσειᾰς|ἐ~ωσαις", "ἐ~ωσαιτον", "ἐ~ωσαιτε";
    aorist_opt_act_3: "ἐ~ωσειε|ἐ~ωσειεν|ἐ~ωσαι", "ἐ~ωσαιτην", "ἐ~ωσειᾰν|ἐ~ωσαιεν";
    aorist_opt_mid_1: "ἐ~ωσαιμην", _, "ἐ~ωσαιμεθᾰ";
    aorist_opt_mid_2: "ἐ~ωσαιο", "ἐ~ωσαισθον", "ἐ~ωσαισθε";
    aorist_opt_mid_3: "ἐ~ωσαιτο", "ἐ~ωσαισθην", "ἐ~ωσαιντο";
    aorist_opt_pass_1: "ἐ~ωθειην", _, "ἐ~ωθεῑμεν|ἐ~ωθειημεν";
    aorist_opt_pass_2: "ἐ~ωθειης", "ἐ~ωθεῑτον|ἐ~ωθειητον", "ἐ~ωθεῑτε|ἐ~ωθειητε";
    aorist_opt_pass_3: "ἐ~ωθειη", "ἐ~ωθειτην|ἐ~ωθειητην", "ἐ~ωθεῑεν|ἐ~ωθειησᾰν";
    aorist_imp_act_2: "ἐ~ωσον", "ἐ~ωσᾰτον", "ἐ~ωσᾰτε";
    aorist_imp_act_3: "ἐ~ωσᾰτω", "ἐ~ωσᾰτων", "ἐ~ωσᾰντων";
    aorist_imp_mid_2: "ἐ~ωσαι", "ἐ~ωσᾰσθον", "ἐ~ωσᾰσθε";
    aorist_imp_mid_3: "ἐ~ωσᾰσθω", "ἐ~ωσᾰσθων", _;
    aorist_imp_pass_2: "ἐ~ωθητῐ", _, _;
    aorist_imp_pass_3: "ἐ~ωθητω", "ἐ~ωθητων", "ἐ~ωθεντων";
    perfect_ind_act_1: "ἠ~ωκᾰ", _, "ἠ~ωκᾰμεν";
    perfect_ind_act_2: "ἠ~ωκᾰς", "ἠ~ωκᾰτον", "ἠ~ωκᾰτε";
    perfect_ind_act_3: "ἠ~ωκε", _, "ἠ~ωκᾱσῐ";
    perfect_ind_mid_1: "ἠ~ωμαι", _, "ἠ~ωμεθᾰ";
    perfect_ind_mid_2: "ἠ~ωσαι", "ἠ~ωσθον", "ἠ~ωσθε";
    perfect_ind_mid_3: "ἠ~ωται", _, "ἠ~ωνται";
    perfect_subj_act_1: "ἠ~ωκω", _, "ἠ~ωκωμεν";
    perfect_subj_act_2: "ἠ~ωκῃς", "ἠ~ωκητον", "ἠ~ωκητε";
    perfect_subj_act_3: "ἠ~ωκῃ", _, "ἠ~ωκωσῐ";
    perfect_opt_act_1: "ἠ~ωκοιμῐ", _, "ἠ~ωκοιμεν";
    perfect_opt_act_2: "ἠ~ωκοις", "ἠ~ωκοιτον", "ἠ~ωκοιτε";
    perfect_opt_act_3: "ἠ~ωκοι", "ἠ~ωκοιτην", "ἠ~ωκοιεν";
    perfect_imp_act_2: _, "ἠ~ωκετον", "ἠ~ωκετε";
    perfect_imp_act_3: "ἠ~ωκετω", "ἠ~ωκετων", "ἠ~ωκοντων";
    perfect_imp_mid_2: "ἠ~ωσο", _, _;
    perfect_imp_mid_3: "ἠ~ωσθω", "ἠ~ωσθων", _;
    pluperfect_mid_1: "ἠ~ωμην", _, "ἠ~ωμεθᾰ";
    pluperfect_mid_2: "ἠ~ωσο", "ἠ~ωσθον", "ἠ~ωσθε";
    pluperfect_mid_3: "ἠ~ωτο", "ἠ~ωσθην", "ἠ~ωντο";
    infinitive_pres: "ἐ~οῡν", "ἐ~οῡσθαι", _;
    infinitive_fut: "ἐ~ωσειν", "ἐ~ωσεσθαι", "ἐ~ωθησεσθαι";
    infinitive_aor: "ἐ~ωσαι", "ἐ~ωσᾰσθαι", "ἐ~ωθηναι";
    infinitive_perf: "ἠ~ωκεναι", "ἠ~ωσθαι", _;
    participle_pres: _, "ἐ~ουμενος", _;
    participle_fut: "ἐ~ωσων", "ἐ~ωσομενος", "ἐ~ωθησομενος";
    participle_aor: "ἐ~ωσᾱς", "ἐ~ωσᾰμενος", "ἐ~ωθεις";
    participle_perf: "ἠ~ωκως", "ἠ~ωμενος", _;
    ),
    // Ἐρωτᾰω: 4 words (ἐρωτᾰω, ἐρᾰω, ἐρευνᾰω, ἐμπολᾰω). Like Μᾰρτῠρεω, except in 220 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐρωτᾰω", 4, "ἐ~ᾰω", like Μᾰρτῠρεω as ACT_AS_PASS;
    stem aorist_ind: "~ησ- like ἀγορευω";
    stem aorist_ind_pass: "~ηθη- like ἀγορευω";
    stem pluperfect_act: "~ηκε- like ἀγορευω";
    ind_act_1: "ἐ~ω", _, "ἐ~ωμεν";
    ind_act_2: "ἐ~ᾳς", "ἐ~ᾱτον", "ἐ~ᾱτε";
    ind_act_3: "ἐ~ᾳ", _, "ἐ~ωσῐ|ἐ~ωσῐν";
    ind_mid_1: "ἐ~ωμαι", _, "ἐ~ωμεθᾰ";
    ind_mid_2: "ἐ~ᾳ", "ἐ~ᾱσθον", "ἐ~ᾱσθε";
    ind_mid_3: "ἐ~ᾱται", _, "ἐ~ωνται";
    subj_act_1: "ἐ~ω", _, "ἐ~ωμεν";
    subj_act_2: "ἐ~ᾳς", "ἐ~ᾱτον", "ἐ~ᾱτε";
    subj_act_3: "ἐ~ᾳ", _, "ἐ~ωσῐ|ἐ~ωσῐν";
    subj_mid_1: "ἐ~ωμαι", _, "ἐ~ωμεθᾰ";
    subj_mid_2: _, "ἐ~ᾱσθον", "ἐ~ᾱσθε";
    subj_mid_3: "ἐ~ᾱται", _, "ἐ~ωνται";
    opt_act_1: "ἐ~ῳην|ἐ~ῳμῐ", _, "ἐ~ῳμεν|ἐ~ῳημεν";
    opt_act_2: "ἐ~ῳης|ἐ~ῳς", "ἐ~ῳτον|ἐ~ῳητον", "ἐ~ῳτε|ἐ~ῳητε";
    opt_act_3: "ἐ~ῳη|ἐ~ῳ", "ἐ~ῳτην|ἐ~ῳητην", "ἐ~ῳεν|ἐ~ῳησᾰν";
    opt_mid_1: "ἐ~ῳμην", _, "ἐ~ῳμεθᾰ";
    opt_mid_2: "ἐ~ῳο", "ἐ~ῳσθον", "ἐ~ῳσθε";
    opt_mid_3: "ἐ~ῳτο", "ἐ~ῳσθην", "ἐ~ῳντο";
    imp_act_2: "ἐ~ᾱ", _, _;
    imp_act_3: "ἐ~ᾱτω", "ἐ~ᾱτων", "ἐ~ωντων";
    imp_mid_2: "ἐ~ω", _, _;
    imp_mid_3: "ἐ~ᾱσθω", "ἐ~ᾱσθων", _;
    imperfect_act_1: "ἠ~ων", _, "ἠ~ωμεν";
    imperfect_act_2: "ἠ~ᾱς", "ἠ~ᾱτον", "ἠ~ᾱτε";
    imperfect_act_3: "ἠ~ᾱ", "ἠ~ᾱτην", "ἠ~ων";
    imperfect_mid_1: "ἠ~ωμην", _, "ἠ~ωμεθᾰ";
    imperfect_mid_2: "ἠ~ω", "ἠ~ᾱσθον", "ἠ~ᾱσθε";
    imperfect_mid_3: "ἠ~ᾱτο", "ἠ~ᾱσθην", "ἠ~ωντο";
    future_ind_act_1: "ἐ~ησω", _, "ἐ~ησομεν";
    future_ind_act_2: "ἐ~ησεις", "ἐ~ησετον", "ἐ~ησετε";
    future_ind_act_3: "ἐ~ησει", _, "ἐ~ησουσῐ";
    future_ind_mid_1: "ἐ~ησομαι", _, "ἐ~ησομεθᾰ";
    future_ind_mid_2: "ἐ~ησῃ", "ἐ~ησεσθον", "ἐ~ησεσθε";
    future_ind_mid_3: "ἐ~ησεται", _, "ἐ~ησονται";
    future_ind_pass_1: "ἐ~ηθησομαι", _, "ἐ~ηθησομεθᾰ";
    future_ind_pass_2: "ἐ~ηθησῃ", "ἐ~ηθησεσθον", "ἐ~ηθησεσθε";
    future_ind_pass_3: "ἐ~ηθησεται", _, "ἐ~ηθησονται";
    future_opt_act_1: "ἐ~ησοιμῐ", _, "ἐ~ησοιμεν";
    future_opt_act_2: "ἐ~ησοις", "ἐ~ησοιτον", "ἐ~ησοιτε";
    future_opt_act_3: "ἐ~ησοι", "ἐ~ησοιτην", "ἐ~ησοιεν";
    future_opt_mid_1: "ἐ~ησοιμην", _, "ἐ~ησοιμεθᾰ";
    future_opt_mid_2: "ἐ~ησοιο", "ἐ~ησοισθον", "ἐ~ησοισθε";
    future_opt_mid_3: "ἐ~ησοιτο", "ἐ~ησοισθην", "ἐ~ησοιντο";
    future_opt_pass_1: "ἐ~ηθησοιμην", _, "ἐ~ηθησοιμεθᾰ";
    future_opt_pass_2: "ἐ~ηθησοιο", "ἐ~ηθησοισθον", "ἐ~ηθησοισθε";
    future_opt_pass_3: "ἐ~ηθησοιτο", "ἐ~ηθησοισθην", "ἐ~ηθησοιντο";
    aorist_subj_act_1: "ἐ~ησω", _, "ἐ~ησωμεν";
    aorist_subj_act_2: "ἐ~ησῃς", "ἐ~ησητον", "ἐ~ησητε";
    aorist_subj_act_3: "ἐ~ησῃ", _, "ἐ~ησωσῐ";
    aorist_subj_mid_1: "ἐ~ησωμαι", _, "ἐ~ησωμεθᾰ";
    aorist_subj_mid_2: _, "ἐ~ησησθον", "ἐ~ησησθε";
    aorist_subj_mid_3: "ἐ~ησηται", _, "ἐ~ησωνται";
    aorist_subj_pass_3: _, _, "ἐ~ηθωσῐ";
    aorist_opt_act_1: "ἐ~ησαιμῐ", _, "ἐ~ησαιμεν";
    aorist_opt_act_2: "ἐ~ησειᾰς", "ἐ~ησαιτον", "ἐ~ησαιτε";
    aorist_opt_act_3: "ἐ~ησειε", "ἐ~ησαιτην", "ἐ~ησειᾰν";
    aorist_opt_mid_1: "ἐ~ησαιμην", _, "ἐ~ησαιμεθᾰ";
    aorist_opt_mid_2: "ἐ~ησαιο", "ἐ~ησαισθον", "ἐ~ησαισθε";
    aorist_opt_mid_3: "ἐ~ησαιτο", "ἐ~ησαισθην", "ἐ~ησαιντο";
    aorist_opt_pass_1: _, _, "ἐ~ηθεῑμεν";
    aorist_opt_pass_2: _, "ἐ~ηθεῑτον", "ἐ~ηθεῑτε";
    aorist_opt_pass_3: _, "ἐ~ηθειτην", "ἐ~ηθεῑεν";
    aorist_imp_act_2: "ἐ~ησον", "ἐ~ησᾰτον", "ἐ~ησᾰτε";
    aorist_imp_act_3: "ἐ~ησᾰτω", "ἐ~ησᾰτων", "ἐ~ησᾰντων";
    aorist_imp_mid_2: "ἐ~ησαι", "ἐ~ησᾰσθον", "ἐ~ησᾰσθε";
    aorist_imp_mid_3: "ἐ~ησᾰσθω", "ἐ~ησᾰσθων", _;
    perfect_ind_act_1: "ἠ~ηκᾰ", _, "ἠ~ηκᾰμεν";
    perfect_ind_act_2: "ἠ~ηκᾰς", "ἠ~ηκᾰτον", "ἠ~ηκᾰτε";
    perfect_ind_act_3: "ἠ~ηκε", _, "ἠ~ηκᾱσῐ";
    perfect_ind_mid_1: "ἠ~ημαι", _, "ἠ~ημεθᾰ";
    perfect_ind_mid_2: "ἠ~ησαι", "ἠ~ησθον", "ἠ~ησθε";
    perfect_ind_mid_3: "ἠ~ηται", _, "ἠ~ηνται";
    perfect_subj_act_1: "ἠ~ηκω", _, "ἠ~ηκωμεν";
    perfect_subj_act_2: "ἠ~ηκῃς", "ἠ~ηκητον", "ἠ~ηκητε";
    perfect_subj_act_3: "ἠ~ηκῃ", _, "ἠ~ηκωσῐ";
    perfect_opt_act_1: "ἠ~ηκοιμῐ", _, "ἠ~ηκοιμεν";
    perfect_opt_act_2: "ἠ~ηκοις", "ἠ~ηκοιτον", "ἠ~ηκοιτε";
    perfect_opt_act_3: "ἠ~ηκοι", "ἠ~ηκοιτην", "ἠ~ηκοιεν";
    perfect_imp_act_2: _, "ἠ~ηκετον", "ἠ~ηκετε";
    perfect_imp_act_3: "ἠ~ηκετω", "ἠ~ηκετων", "ἠ~ηκοντων";
    perfect_imp_mid_2: "ἠ~ησο", _, _;
    perfect_imp_mid_3: "ἠ~ησθω", "ἠ~ησθων", _;
    pluperfect_mid_1: "ἠ~ημην", _, "ἠ~ημεθᾰ";
    pluperfect_mid_2: "ἠ~ησο", "ἠ~ησθον", "ἠ~ησθε";
    pluperfect_mid_3: "ἠ~ητο", "ἠ~ησθην", "ἠ~ηντο";
    infinitive_pres: "ἐ~ᾱν", "ἐ~ᾱσθαι", _;
    infinitive_fut: "ἐ~ησειν", "ἐ~ησεσθαι", "ἐ~ηθησεσθαι";
    infinitive_aor: "ἐ~ησαι", "ἐ~ησᾰσθαι", _;
    infinitive_perf: "ἠ~ηκεναι", "ἠ~ησθαι", _;
    participle_pres: "ἐ~ων", "ἐ~ωμενος", _;
    participle_fut: "ἐ~ησων", "ἐ~ησομενος", "ἐ~ηθησομενος";
    participle_aor: "ἐ~ησᾱς", "ἐ~ησᾰμενος", _;
    participle_perf: "ἠ~ηκως", "ἠ~ημενος", _;
    ),
    // Πᾰρᾰγῐγνομαι: 4 words (ὑπογῐγνομαι, πᾰρᾰγῐγνομαι, περῐγῐγνομαι, ῠγγῐγνομαι). Like Ζευγνῡμῐ, except in 171 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("πᾰρᾰγῐγνομαι", 4, "~~γῐγ~ομαι", like Ζευγνῡμῐ as ACT_AS_PASS;
    stem imperfect: "~νε%ῐ~~- like ὑποκοριζομαι";
    stem aorist_ind: "~νε~ε~- like ἐρχομαι";
    stem aorist_ind_pass: "~νε~ε~ηθη- like ἐλπω";
    stem pluperfect_act: "~νε~εγο~ε- like ἀνῐστημῐ";
    ind_act_1: "~~γῐγ~ομαι", _, _;
    ind_mid_1: "~~γῐγ~ομαι", _, "~~γῐγ~ομεθᾰ";
    ind_mid_2: "~~γῐγ~ῃ", "~~γῐγ~εσθον", "~~γῐγ~εσθε";
    ind_mid_3: "~~γῐγ~εται", _, "~~γῐγ~ονται";
    subj_mid_1: "~~γῐγ~ωμαι", _, "~~γῐγ~ωμεθᾰ";
    subj_mid_2: "~~γῐγ~ῃ", "~~γῐγ~ησθον", "~~γῐγ~ησθε";
    subj_mid_3: "~~γῐγ~ηται", _, "~~γῐγ~ωνται";
    subj_pass_2: "~~γῐγ~ῃ", _, _;
    opt_mid_1: "~~γῐγ~οιμην", _, "~~γῐγ~οιμεθᾰ";
    opt_mid_2: "~~γῐγ~οιο", "~~γῐγ~οισθον", "~~γῐγ~οισθε";
    opt_mid_3: "~~γῐγ~οιτο", "~~γῐγ~οισθην", "~~γῐγ~οιντο";
    imp_mid_2: "~~γῐγ~ου", _, _;
    imp_mid_3: "~~γῐγ~εσθω", "~~γῐγ~εσθων", _;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    future_ind_mid_1: "~~γε~ησομαι", _, "~~γε~ησομεθᾰ";
    future_ind_mid_2: "~~γε~ησῃ", "~~γε~ησεσθον", "~~γε~ησεσθε";
    future_ind_mid_3: "~~γε~ησεται", _, "~~γε~ησονται";
    future_ind_pass_1: "~~γε~ηθησομαι", _, "~~γε~ηθησομεθᾰ";
    future_ind_pass_2: "~~γε~ηθησῃ", "~~γε~ηθησεσθον", "~~γε~ηθησεσθε";
    future_ind_pass_3: "~~γε~ηθησεται", _, "~~γε~ηθησονται";
    future_opt_mid_1: "~~γε~ησοιμην", _, "~~γε~ησοιμεθᾰ";
    future_opt_mid_2: "~~γε~ησοιο", "~~γε~ησοισθον", "~~γε~ησοισθε";
    future_opt_mid_3: "~~γε~ησοιτο", "~~γε~ησοισθην", "~~γε~ησοιντο";
    future_opt_pass_1: "~~γε~ηθησοιμην", _, "~~γε~ηθησοιμεθᾰ";
    future_opt_pass_2: "~~γε~ηθησοιο", "~~γε~ηθησοισθον", "~~γε~ηθησοισθε";
    future_opt_pass_3: "~~γε~ηθησοιτο", "~~γε~ηθησοισθην", "~~γε~ηθησοιντο";
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_subj_mid_1: "~~γε~ωμαι", _, "~~γε~ωμεθᾰ";
    aorist_subj_mid_2: "~~γε~ῃ", "~~γε~ησθον", "~~γε~ησθε";
    aorist_subj_mid_3: "~~γε~ηται", _, "~~γε~ωνται";
    aorist_subj_pass_1: "~~γε~ηθω", _, "~~γε~ηθωμεν";
    aorist_subj_pass_2: "~~γε~ηθῃς", "~~γε~ηθητον", "~~γε~ηθητε";
    aorist_subj_pass_3: "~~γε~ηθῃ", _, "~~γε~ηθωσῐ";
    aorist_opt_mid_1: "~~γε~οιμην", _, "~~γε~οιμεθᾰ";
    aorist_opt_mid_2: "~~γε~οιο", "~~γε~οισθον", "~~γε~οισθε";
    aorist_opt_mid_3: "~~γε~οιτο", "~~γε~οισθην", "~~γε~οιντο";
    aorist_opt_pass_1: "~~γε~ηθειην", _, "~~γε~ηθεῑμεν";
    aorist_opt_pass_2: "~~γε~ηθειης", "~~γε~ηθεῑτον", "~~γε~ηθεῑτε";
    aorist_opt_pass_3: "~~γε~ηθειη", "~~γε~ηθειτην", "~~γε~ηθεῑεν";
    aorist_imp_mid_2: "~~γε~οῡ", "~~γε~εσθον", "~~γε~εσθε";
    aorist_imp_mid_3: "~~γε~εσθω", "~~γε~εσθων", _;
    aorist_imp_pass_2: "~~γε~ηθητῐ", _, _;
    aorist_imp_pass_3: "~~γε~ηθητω", "~~γε~ηθητων", "~~γε~ηθεντων";
    perfect_ind_act_1: "~~γεγο~ᾰ", _, "~~γεγο~ᾰμεν";
    perfect_ind_act_2: "~~γεγο~ᾰς", "~~γεγο~ᾰτον", "~~γεγο~ᾰτε";
    perfect_ind_act_3: "~~γεγο~ε", _, "~~γεγο~ᾱσῐ";
    perfect_ind_mid_1: "~~γεγε~ημαι", _, "~~γεγε~ημεθᾰ";
    perfect_ind_mid_2: "~~γεγε~ησαι", "~~γεγε~ησθον", "~~γεγε~ησθε";
    perfect_ind_mid_3: "~~γεγε~ηται", _, "~~γεγε~ηνται";
    perfect_subj_act_1: "~~γεγο~ω", _, "~~γεγο~ωμεν";
    perfect_subj_act_2: "~~γεγο~ῃς", "~~γεγο~ητον", "~~γεγο~ητε";
    perfect_subj_act_3: "~~γεγο~ῃ", _, "~~γεγο~ωσῐ";
    perfect_opt_act_1: "~~γεγο~οιμῐ", _, "~~γεγο~οιμεν";
    perfect_opt_act_2: "~~γεγο~οις", "~~γεγο~οιτον", "~~γεγο~οιτε";
    perfect_opt_act_3: "~~γεγο~οι", "~~γεγο~οιτην", "~~γεγο~οιεν";
    perfect_imp_act_2: _, "~~γεγο~ετον", "~~γεγο~ετε";
    perfect_imp_act_3: "~~γεγο~ετω", "~~γεγο~ετων", "~~γεγο~οντων";
    perfect_imp_mid_2: "~~γεγε~ησο", _, _;
    perfect_imp_mid_3: "~~γεγε~ησθω", "~~γεγε~ησθων", _;
    pluperfect_mid_1: "~νε~εγε~ημην", _, "~νε~εγε~ημεθᾰ";
    pluperfect_mid_2: "~νε~εγε~ησο", "~νε~εγε~ησθον", "~νε~εγε~ησθε";
    pluperfect_mid_3: "~νε~εγε~ητο", "~νε~εγε~ησθην", "~νε~εγε~ηντο";
    infinitive_pres: _, "~~γῐγ~εσθαι", _;
    infinitive_fut: _, "~~γε~ησεσθαι", "~~γε~ηθησεσθαι";
    infinitive_aor: _, "~~γε~εσθαι", "~~γε~ηθηναι";
    infinitive_perf: "~~γεγο~εναι", "~~γεγε~ησθαι", _;
    participle_pres: _, "~~γῐγ~ομενος", _;
    participle_fut: _, "~~γε~ησομενος", "~~γε~ηθησομενος";
    participle_aor: _, "~~γε~ομενος", "~~γε~ηθεις";
    participle_perf: "~~γεγο~ως", "~~γεγε~ημενος", _;
    ),
    // Δημοκρᾰτεομαι: 4 words (πλουτοκρᾰτεομαι, δημοκρᾰτεομαι, εὐλαβεομαι, ἀμηχᾰνοποιεομαι). Like Λᾰγοθηρεω, except in 46 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("δημοκρᾰτεομαι", 4, "~ᾰ~εομαι", like Λᾰγοθηρεω;
    stem aorist_ind: "~ᾰ~ησ- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~ηθη- like ἐχω";
    stem pluperfect_act: "~ᾰ~ηκ- like χρημᾰτῐζω";
    imperfect_act_1: "ἐ~ᾰ~ουν", _, "ἐ~ᾰ~οῡμεν";
    imperfect_act_2: "ἐ~ᾰ~εις", "ἐ~ᾰ~εῑτον", "ἐ~ᾰ~εῑτε";
    imperfect_act_3: "ἐ~ᾰ~ει", "ἐ~ᾰ~ειτην", "ἐ~ᾰ~ουν";
    imperfect_mid_1: "ἐ~ᾰ~ουμην", _, "ἐ~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~οῡ", "ἐ~ᾰ~εῑσθον", "ἐ~ᾰ~εῑσθε";
    imperfect_mid_3: "ἐ~ᾰ~εῑτο", "ἐ~ᾰ~εισθην", "ἐ~ᾰ~οῡντο";
    perfect_ind_act_1: "%ε~ᾰ~ηκᾰ", _, "%ε~ᾰ~ηκᾰμεν";
    perfect_ind_act_2: "%ε~ᾰ~ηκᾰς", "%ε~ᾰ~ηκᾰτον", "%ε~ᾰ~ηκᾰτε";
    perfect_ind_act_3: "%ε~ᾰ~ηκε|%ε~ᾰ~ηκεν", _, "%ε~ᾰ~ηκᾱσῐ|%ε~ᾰ~ηκᾱσῐν";
    perfect_subj_act_1: "%ε~ᾰ~ηκω", _, "%ε~ᾰ~ηκωμεν";
    perfect_subj_act_2: "%ε~ᾰ~ηκῃς", "%ε~ᾰ~ηκητον", "%ε~ᾰ~ηκητε";
    perfect_subj_act_3: "%ε~ᾰ~ηκῃ", _, "%ε~ᾰ~ηκωσῐ|%ε~ᾰ~ηκωσῐν";
    perfect_opt_act_1: "%ε~ᾰ~ηκοιμῐ|%ε~ᾰ~ηκοιην", _, "%ε~ᾰ~ηκοιμεν";
    perfect_opt_act_2: "%ε~ᾰ~ηκοις|%ε~ᾰ~ηκοιης", "%ε~ᾰ~ηκοιτον", "%ε~ᾰ~ηκοιτε";
    perfect_opt_act_3: "%ε~ᾰ~ηκοι|%ε~ᾰ~ηκοιη", "%ε~ᾰ~ηκοιτην", "%ε~ᾰ~ηκοιεν";
    perfect_imp_act_2: "%ε~ᾰ~ηκε", "%ε~ᾰ~ηκετον", "%ε~ᾰ~ηκετε";
    perfect_imp_act_3: "%ε~ᾰ~ηκετω", "%ε~ᾰ~ηκετων", "%ε~ᾰ~ηκοντων";
    infinitive_perf: "%ε~ᾰ~ηκεναι", _, _;
    participle_perf: "%ε~ᾰ~ηκως", _, _;
    ),
    // Ἀνᾰλᾰμβᾰνω: 4 words (κᾰτᾰλᾰμβᾰνω, ἀνᾰλᾰμβᾰνω, μετᾰλαμβᾰνω, ἐπᾰνᾰλᾰμβᾰνω). Like Μῑγνῡμῐ, except in 219 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀνᾰλᾰμβᾰνω", 4, "~ᾰλᾰμβᾰνω", like Μῑγνῡμῐ;
    stem aorist_ind: "~ελᾰβ- like πᾰρεχω";
    stem aorist_ind_pass: "~εληφθη- like ἐλπω";
    stem pluperfect_act: "~ειληφ- like εὐλογεω";
    ind_act_1: "~ᾰλᾰμβᾰνω", _, "~ᾰλᾰμβᾰνομεν";
    ind_act_2: "~ᾰλᾰμβᾰνεις", "~ᾰλᾰμβᾰνετον", "~ᾰλᾰμβᾰνετε";
    ind_act_3: "~ᾰλᾰμβᾰνει", _, "~ᾰλᾰμβᾰνουσῐ|~ᾰλᾰμβᾰνουσῐν";
    ind_mid_1: "~ᾰλᾰμβᾰνομαι", _, "~ᾰλᾰμβᾰνομεθᾰ";
    ind_mid_2: "~ᾰλᾰμβᾰνῃ|~ᾰλᾰμβᾰνει", "~ᾰλᾰμβᾰνεσθον", "~ᾰλᾰμβᾰνεσθε";
    ind_mid_3: "~ᾰλᾰμβᾰνεται", _, "~ᾰλᾰμβᾰνονται";
    subj_act_1: "~ᾰλᾰμβᾰνω", _, "~ᾰλᾰμβᾰνωμεν";
    subj_act_2: "~ᾰλᾰμβᾰνῃς", "~ᾰλᾰμβᾰνητον", "~ᾰλᾰμβᾰνητε";
    subj_act_3: "~ᾰλᾰμβᾰνῃ", _, "~ᾰλᾰμβᾰνωσῐ|~ᾰλᾰμβᾰνωσῐν";
    subj_mid_1: "~ᾰλᾰμβᾰνωμαι", _, "~ᾰλᾰμβᾰνωμεθᾰ";
    subj_mid_2: _, "~ᾰλᾰμβᾰνησθον", "~ᾰλᾰμβᾰνησθε";
    subj_mid_3: "~ᾰλᾰμβᾰνηται", _, "~ᾰλᾰμβᾰνωνται";
    opt_act_1: "~ᾰλᾰμβᾰνοιμῐ", _, "~ᾰλᾰμβᾰνοιμεν";
    opt_act_2: "~ᾰλᾰμβᾰνοις", "~ᾰλᾰμβᾰνοιτον", "~ᾰλᾰμβᾰνοιτε";
    opt_act_3: "~ᾰλᾰμβᾰνοι", "~ᾰλᾰμβᾰνοιτην", "~ᾰλᾰμβᾰνοιεν";
    opt_mid_1: "~ᾰλᾰμβᾰνοιμην", _, "~ᾰλᾰμβᾰνοιμεθᾰ";
    opt_mid_2: "~ᾰλᾰμβᾰνοιο", "~ᾰλᾰμβᾰνοισθον", "~ᾰλᾰμβᾰνοισθε";
    opt_mid_3: "~ᾰλᾰμβᾰνοιτο", "~ᾰλᾰμβᾰνοισθην", "~ᾰλᾰμβᾰνοιντο";
    imp_act_2: "~ᾰλᾰμβᾰνε", _, _;
    imp_act_3: "~ᾰλᾰμβᾰνετω", "~ᾰλᾰμβᾰνετων", "~ᾰλᾰμβᾰνοντων";
    imp_mid_2: "~ᾰλᾰμβᾰνου", _, _;
    imp_mid_3: "~ᾰλᾰμβᾰνεσθω", "~ᾰλᾰμβᾰνεσθων", _;
    imperfect_act_1: "~ελᾰμβᾰνον", _, "~ελᾰμβᾰνομεν";
    imperfect_act_2: "~ελᾰμβᾰνες", "~ελᾰμβᾰνετον", "~ελᾰμβᾰνετε";
    imperfect_act_3: "~ελᾰμβᾰνε|~ελᾰμβᾰνεν", "~ελᾰμβᾰνετην", "~ελᾰμβᾰνον";
    imperfect_mid_1: "~ελᾰμβᾰνομην", _, "~ελᾰμβᾰνομεθᾰ";
    imperfect_mid_2: "~ελᾰμβᾰνου", "~ελᾰμβᾰνεσθον", "~ελᾰμβᾰνεσθε";
    imperfect_mid_3: "~ελᾰμβᾰνετο", "~ελᾰμβᾰνεσθην", "~ελᾰμβᾰνοντο";
    future_ind_mid_1: "~ᾰληψομαι", _, "~ᾰληψομεθᾰ";
    future_ind_mid_2: "~ᾰληψῃ|~ᾰληψει", "~ᾰληψεσθον", "~ᾰληψεσθε";
    future_ind_mid_3: "~ᾰληψεται", _, "~ᾰληψονται";
    future_ind_pass_1: "~ᾰληφθησομαι", _, "~ᾰληφθησομεθᾰ";
    future_ind_pass_2: "~ᾰληφθησῃ|~ᾰληφθησει", "~ᾰληφθησεσθον", "~ᾰληφθησεσθε";
    future_ind_pass_3: "~ᾰληφθησεται", _, "~ᾰληφθησονται";
    future_opt_mid_1: "~ᾰληψοιμην", _, "~ᾰληψοιμεθᾰ";
    future_opt_mid_2: "~ᾰληψοιο", "~ᾰληψοισθον", "~ᾰληψοισθε";
    future_opt_mid_3: "~ᾰληψοιτο", "~ᾰληψοισθην", "~ᾰληψοιντο";
    future_opt_pass_1: "~ᾰληφθησοιμην", _, "~ᾰληφθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰληφθησοιο", "~ᾰληφθησοισθον", "~ᾰληφθησοισθε";
    future_opt_pass_3: "~ᾰληφθησοιτο", "~ᾰληφθησοισθην", "~ᾰληφθησοιντο";
    aorist_subj_act_1: "~ᾰλᾰβω", _, "~ᾰλᾰβωμεν";
    aorist_subj_act_2: "~ᾰλᾰβῃς", "~ᾰλᾰβητον", "~ᾰλᾰβητε";
    aorist_subj_act_3: "~ᾰλᾰβῃ", _, "~ᾰλᾰβωσῐ|~ᾰλᾰβωσῐν";
    aorist_subj_mid_1: "~ᾰλᾰβωμαι", _, "~ᾰλᾰβωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰλᾰβησθον", "~ᾰλᾰβησθε";
    aorist_subj_mid_3: "~ᾰλᾰβηται", _, "~ᾰλᾰβωνται";
    aorist_subj_pass_1: "~ᾰληφθω", _, "~ᾰληφθωμεν";
    aorist_subj_pass_2: "~ᾰληφθῃς", "~ᾰληφθητον", "~ᾰληφθητε";
    aorist_subj_pass_3: "~ᾰληφθῃ", _, "~ᾰληφθωσῐ|~ᾰληφθωσῐν";
    aorist_opt_act_1: "~ᾰλᾰβοιμῐ", _, "~ᾰλᾰβοιμεν";
    aorist_opt_act_2: "~ᾰλᾰβοις", "~ᾰλᾰβοιτον", "~ᾰλᾰβοιτε";
    aorist_opt_act_3: "~ᾰλᾰβοι", "~ᾰλᾰβοιτην", "~ᾰλᾰβοιεν";
    aorist_opt_mid_1: "~ᾰλᾰβοιμην", _, "~ᾰλᾰβοιμεθᾰ";
    aorist_opt_mid_2: "~ᾰλᾰβοιο", "~ᾰλᾰβοισθον", "~ᾰλᾰβοισθε";
    aorist_opt_mid_3: "~ᾰλᾰβοιτο", "~ᾰλᾰβοισθην", "~ᾰλᾰβοιντο";
    aorist_opt_pass_1: "~ᾰληφθειην", _, "~ᾰληφθεῑμεν|~ᾰληφθειημεν";
    aorist_opt_pass_2: "~ᾰληφθειης", "~ᾰληφθεῑτον|~ᾰληφθειητον", "~ᾰληφθεῑτε|~ᾰληφθειητε";
    aorist_opt_pass_3: "~ᾰληφθειη", "~ᾰληφθειτην|~ᾰληφθειητην", "~ᾰληφθεῑεν|~ᾰληφθειησᾰν";
    aorist_imp_act_2: "~ᾰλᾰβε", "~ᾰλᾰβετον", "~ᾰλᾰβετε";
    aorist_imp_act_3: "~ᾰλᾰβετω", "~ᾰλᾰβετων", "~ᾰλᾰβοντων";
    aorist_imp_mid_2: "~ᾰλᾰβοῡ", "~ᾰλᾰβεσθον", "~ᾰλᾰβεσθε";
    aorist_imp_mid_3: "~ᾰλᾰβεσθω", "~ᾰλᾰβεσθων", _;
    aorist_imp_pass_2: "~ᾰληφθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰληφθητω", "~ᾰληφθητων", "~ᾰληφθεντων";
    perfect_ind_act_1: "~ειληφᾰ", _, "~ειληφᾰμεν";
    perfect_ind_act_2: "~ειληφᾰς", "~ειληφᾰτον", "~ειληφᾰτε";
    perfect_ind_act_3: "~ειληφε|~ειληφεν", _, "~ειληφᾱσῐ|~ειληφᾱσῐν";
    perfect_ind_mid_1: "~ειλημμαι", _, "~ειλημμεθᾰ";
    perfect_ind_mid_2: "~ειληψαι", "~ειληφθον", "~ειληφθε";
    perfect_ind_mid_3: "~ειληπται", _, "~ειληφᾰται";
    perfect_subj_act_1: "~ειληφω", _, "~ειληφωμεν";
    perfect_subj_act_2: "~ειληφῃς", "~ειληφητον", "~ειληφητε";
    perfect_subj_act_3: "~ειληφῃ", _, "~ειληφωσῐ|~ειληφωσῐν";
    perfect_opt_act_1: "~ειληφοιμῐ|~ειληφοιην", _, "~ειληφοιμεν";
    perfect_opt_act_2: "~ειληφοις|~ειληφοιης", "~ειληφοιτον", "~ειληφοιτε";
    perfect_opt_act_3: "~ειληφοι|~ειληφοιη", "~ειληφοιτην", "~ειληφοιεν";
    perfect_imp_act_2: "~ειληφε", "~ειληφετον", "~ειληφετε";
    perfect_imp_act_3: "~ειληφετω", "~ειληφετων", "~ειληφοντων";
    perfect_imp_mid_2: "~ειληψο", _, _;
    perfect_imp_mid_3: "~ειληφθω", "~ειληφθων", _;
    pluperfect_mid_1: "~ειλημμην", _, "~ειλημμεθᾰ";
    pluperfect_mid_2: "~ειληψο", "~ειληφθον", "~ειληφθε";
    pluperfect_mid_3: "~ειληπτο", "~ειληφθην", "~ειληφᾰτο";
    infinitive_pres: "~ᾰλᾰμβᾰνειν", "~ᾰλᾰμβᾰνεσθαι", _;
    infinitive_fut: _, "~ᾰληψεσθαι", "~ᾰληφθησεσθαι";
    infinitive_aor: "~ᾰλᾰβεῑν", "~ᾰλᾰβεσθαι", "~ᾰληφθηναι";
    infinitive_perf: "~ειληφεναι", "~ειληφθαι", _;
    participle_pres: "~ᾰλᾰμβᾰνων", "~ᾰλᾰμβᾰνομενος", _;
    participle_fut: _, "~ᾰληψομενος", "~ᾰληφθησομενος";
    participle_aor: "~ᾰλᾰβων", "~ᾰλᾰβομενος", "~ᾰληφθεις";
    participle_perf: "~ειληφως", "~ειλημμενος", _;
    ),
    // Γυμνοω: 4 words (κυκλοω, γυμνοω, ἀπολυτροω, ἐκλυτροω). Like Ξαινω, except in 227 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("γυμνοω", 4, "~υ~οω", like Ξαινω;
    stem aorist_ind: "~ῠ~ωσ- like ἐχω";
    stem aorist_ind_pass: "~ῠ~ωθη- like ἐχω";
    ind_act_1: "~ῠ~ω", _, "~ῠ~οῡμεν";
    ind_act_2: "~ῠ~οῑς", "~ῠ~οῡτον", "~ῠ~οῡτε";
    ind_act_3: "~ῠ~οῑ", _, "~ῠ~οῡσῐ|~ῠ~οῡσῐν";
    ind_mid_1: "~ῠ~οῡμαι", _, "~ῠ~ουμεθᾰ";
    ind_mid_2: "~ῠ~οῑ", "~ῠ~οῡσθον", "~ῠ~οῡσθε";
    ind_mid_3: "~ῠ~οῡται", _, "~ῠ~οῡνται";
    subj_act_1: "~ῠ~ω", _, "~ῠ~ωμεν";
    subj_act_2: "~ῠ~οῑς", "~ῠ~ωτον", "~ῠ~ωτε";
    subj_act_3: "~ῠ~οῑ", _, "~ῠ~ωσῐ|~ῠ~ωσῐν";
    subj_mid_1: "~ῠ~ωμαι", _, "~ῠ~ωμεθᾰ";
    subj_mid_2: _, "~ῠ~ωσθον", "~ῠ~ωσθε";
    subj_mid_3: "~ῠ~ωται", _, "~ῠ~ωνται";
    opt_act_1: "~ῠ~οιην|~ῠ~οῑμῐ", _, "~ῠ~οῑμεν|~ῠ~οιημεν";
    opt_act_2: "~ῠ~οιης|~ῠ~οῑς", "~ῠ~οῑτον|~ῠ~οιητον", "~ῠ~οῑτε|~ῠ~οιητε";
    opt_act_3: "~ῠ~οιη|~ῠ~οῑ", "~ῠ~οιτην|~ῠ~οιητην", "~ῠ~οῑεν|~ῠ~οιησᾰν";
    opt_mid_1: "~ῠ~οιμην", _, "~ῠ~οιμεθᾰ";
    opt_mid_2: "~ῠ~οῑο", "~ῠ~οῑσθον", "~ῠ~οῑσθε";
    opt_mid_3: "~ῠ~οῑτο", "~ῠ~οισθην", "~ῠ~οῑντο";
    imp_act_2: "~ῠ~ου", _, _;
    imp_act_3: "~ῠ~ουτω", "~ῠ~ουτων", "~ῠ~ουντων";
    imp_mid_2: "~ῠ~οῡ", _, _;
    imp_mid_3: "~ῠ~ουσθω", "~ῠ~ουσθων", _;
    imperfect_act_1: "ἐ~ῠ~ουν", _, "ἐ~ῠ~οῡμεν";
    imperfect_act_2: "ἐ~ῠ~ους", "ἐ~ῠ~οῡτον", "ἐ~ῠ~οῡτε";
    imperfect_act_3: "ἐ~ῠ~ου", "ἐ~ῠ~ουτην", "ἐ~ῠ~ουν";
    imperfect_mid_1: "ἐ~ῠ~ουμην", _, "ἐ~ῠ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ῠ~οῡ", "ἐ~ῠ~οῡσθον", "ἐ~ῠ~οῡσθε";
    imperfect_mid_3: "ἐ~ῠ~οῡτο", "ἐ~ῠ~ουσθην", "ἐ~ῠ~οῡντο";
    future_ind_act_1: "~ῠ~ωσω", _, "~ῠ~ωσομεν";
    future_ind_act_2: "~ῠ~ωσεις", "~ῠ~ωσετον", "~ῠ~ωσετε";
    future_ind_act_3: "~ῠ~ωσει", _, "~ῠ~ωσουσῐ";
    future_ind_mid_1: "~ῠ~ωσομαι", _, "~ῠ~ωσομεθᾰ";
    future_ind_mid_2: "~ῠ~ωσῃ", "~ῠ~ωσεσθον", "~ῠ~ωσεσθε";
    future_ind_mid_3: "~ῠ~ωσεται", _, "~ῠ~ωσονται";
    future_ind_pass_1: "~ῠ~ωθησομαι", _, "~ῠ~ωθησομεθᾰ";
    future_ind_pass_2: "~ῠ~ωθησῃ", "~ῠ~ωθησεσθον", "~ῠ~ωθησεσθε";
    future_ind_pass_3: "~ῠ~ωθησεται", _, "~ῠ~ωθησονται";
    future_opt_act_1: "~ῠ~ωσοιμῐ", _, "~ῠ~ωσοιμεν";
    future_opt_act_2: "~ῠ~ωσοις", "~ῠ~ωσοιτον", "~ῠ~ωσοιτε";
    future_opt_act_3: "~ῠ~ωσοι", "~ῠ~ωσοιτην", "~ῠ~ωσοιεν";
    future_opt_mid_1: "~ῠ~ωσοιμην", _, "~ῠ~ωσοιμεθᾰ";
    future_opt_mid_2: "~ῠ~ωσοιο", "~ῠ~ωσοισθον", "~ῠ~ωσοισθε";
    future_opt_mid_3: "~ῠ~ωσοιτο", "~ῠ~ωσοισθην", "~ῠ~ωσοιντο";
    future_opt_pass_1: "~ῠ~ωθησοιμην", _, "~ῠ~ωθησοιμεθᾰ";
    future_opt_pass_2: "~ῠ~ωθησοιο", "~ῠ~ωθησοισθον", "~ῠ~ωθησοισθε";
    future_opt_pass_3: "~ῠ~ωθησοιτο", "~ῠ~ωθησοισθην", "~ῠ~ωθησοιντο";
    aorist_subj_act_1: "~ῠ~ωσω", _, "~ῠ~ωσωμεν";
    aorist_subj_act_2: "~ῠ~ωσῃς", "~ῠ~ωσητον", "~ῠ~ωσητε";
    aorist_subj_act_3: "~ῠ~ωσῃ", _, "~ῠ~ωσωσῐ";
    aorist_subj_mid_1: "~ῠ~ωσωμαι", _, "~ῠ~ωσωμεθᾰ";
    aorist_subj_mid_2: _, "~ῠ~ωσησθον", "~ῠ~ωσησθε";
    aorist_subj_mid_3: "~ῠ~ωσηται", _, "~ῠ~ωσωνται";
    aorist_subj_pass_1: "~ῠ~ηθω", _, "~ῠ~ηθωμεν";
    aorist_subj_pass_2: "~ῠ~ηθῃς", "~ῠ~ηθητον", "~ῠ~ηθητε";
    aorist_subj_pass_3: "~ῠ~ηθῃ", _, "~ῠ~ηθωσῐ";
    aorist_opt_act_1: "~ῠ~ωσαιμῐ", _, "~ῠ~ωσαιμεν";
    aorist_opt_act_2: "~ῠ~ωσειᾰς", "~ῠ~ωσαιτον", "~ῠ~ωσαιτε";
    aorist_opt_act_3: "~ῠ~ωσειε", "~ῠ~ωσαιτην", "~ῠ~ωσειᾰν";
    aorist_opt_mid_1: "~ῠ~ωσαιμην", _, "~ῠ~ωσαιμεθᾰ";
    aorist_opt_mid_2: "~ῠ~ωσαιο", "~ῠ~ωσαισθον", "~ῠ~ωσαισθε";
    aorist_opt_mid_3: "~ῠ~ωσαιτο", "~ῠ~ωσαισθην", "~ῠ~ωσαιντο";
    aorist_opt_pass_1: "~ῠ~ηθειην", _, "~ῠ~ηθεῑμεν";
    aorist_opt_pass_2: "~ῠ~ηθειης", "~ῠ~ηθεῑτον", "~ῠ~ηθεῑτε";
    aorist_opt_pass_3: "~ῠ~ηθειη", "~ῠ~ηθειτην", "~ῠ~ηθεῑεν";
    aorist_imp_act_2: "~ῠ~ωσον", "~ῠ~ωσᾰτον", "~ῠ~ωσᾰτε";
    aorist_imp_act_3: "~ῠ~ωσᾰτω", "~ῠ~ωσᾰτων", "~ῠ~ωσᾰντων";
    aorist_imp_mid_2: "~ῠ~ωσαι", "~ῠ~ωσᾰσθον", "~ῠ~ωσᾰσθε";
    aorist_imp_mid_3: "~ῠ~ωσᾰσθω", "~ῠ~ωσᾰσθων", _;
    aorist_imp_pass_2: "~ῠ~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ῠ~ηθητω", "~ῠ~ηθητων", "~ῠ~ηθεντων";
    perfect_ind_act_1: "~ε%ῠ~ωκᾰ", _, "~ε%ῠ~ωκᾰμεν";
    perfect_ind_act_2: "~ε%ῠ~ωκᾰς", "~ε%ῠ~ωκᾰτον", "~ε%ῠ~ωκᾰτε";
    perfect_ind_act_3: "~ε%ῠ~ωκε", _, "~ε%ῠ~ωκᾱσῐ";
    perfect_ind_mid_1: "~ε%ῠ~ωμαι", _, "~ε%ῠ~ωμεθᾰ";
    perfect_ind_mid_2: "~ε%ῠ~ωσαι", "~ε%ῠ~ωσθον", "~ε%ῠ~ωσθε";
    perfect_ind_mid_3: "~ε%ῠ~ωται", _, "~ε%ῠ~ωνται";
    perfect_subj_act_1: "~ε%ῠ~ωκω", _, "~ε%ῠ~ωκωμεν";
    perfect_subj_act_2: "~ε%ῠ~ωκῃς", "~ε%ῠ~ωκητον", "~ε%ῠ~ωκητε";
    perfect_subj_act_3: "~ε%ῠ~ωκῃ", _, "~ε%ῠ~ωκωσῐ";
    perfect_opt_act_1: "~ε%ῠ~ωκοιμῐ", _, "~ε%ῠ~ωκοιμεν";
    perfect_opt_act_2: "~ε%ῠ~ωκοις", "~ε%ῠ~ωκοιτον", "~ε%ῠ~ωκοιτε";
    perfect_opt_act_3: "~ε%ῠ~ωκοι", "~ε%ῠ~ωκοιτην", "~ε%ῠ~ωκοιεν";
    perfect_imp_act_2: _, "~ε%ῠ~ωκετον", "~ε%ῠ~ωκετε";
    perfect_imp_act_3: "~ε%ῠ~ωκετω", "~ε%ῠ~ωκετων", "~ε%ῠ~ωκοντων";
    perfect_imp_mid_2: "~ε%ῠ~ωσο", _, _;
    perfect_imp_mid_3: "~ε%ῠ~ωσθω", "~ε%ῠ~ωσθων", _;
    infinitive_pres: "~ῠ~οῡν", "~ῠ~οῡσθαι", _;
    infinitive_fut: "~ῠ~ωσειν", "~ῠ~ωσεσθαι", "~ῠ~ωθησεσθαι";
    infinitive_aor: "~ῠ~ωσαι", "~ῠ~ωσᾰσθαι", "~ῠ~ηθηναι";
    infinitive_perf: "~ε%ῠ~ωκεναι", "~ε%ῠ~ωσθαι", _;
    participle_pres: "~ῠ~ων", "~ῠ~ουμενος", _;
    participle_fut: "~ῠ~ωσων", "~ῠ~ωσομενος", "~ῠ~ωθησομενος";
    participle_aor: "~ῠ~ωσᾱς", "~ῠ~ωσᾰμενος", "~ῠ~ηθεις";
    participle_perf: "~ε%ῠ~ωκως", "~ε%ῠ~ωμενος", _;
    ),
    // Ἀφορῐζω: 4 words (ἀφορῐζω, προορῐζω, ἐξορκῐζω, ἑορτᾰζω). Like Ἀποκωλῡω, except in 187 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀφορῐζω", 4, "~ο~ζω", like Ἀποκωλῡω as FUT_AS_PRES;
    stem aorist_ind: "~ω~σ- like εὐλογεω";
    stem aorist_ind_pass: "~ω~σθη- like ἐλπω";
    stem pluperfect_act: "~ω~κ- like εὐλογεω";
    stem pluperfect: "~ω~- like ὑποκοριζομαι";
    ind_act_1: "~ο~ζω", _, "~ο~ζομεν";
    ind_act_2: "~ο~ζεις", "~ο~ζετον", "~ο~ζετε";
    ind_act_3: "~ο~ζει", _, "~ο~ζουσῐ|~ο~ζουσῐν";
    ind_mid_1: "~ο~ζομαι", _, "~ο~ζομεθᾰ";
    ind_mid_2: "~ο~ζῃ|~ο~ζει", "~ο~ζεσθον", "~ο~ζεσθε";
    ind_mid_3: "~ο~ζεται", _, "~ο~ζονται";
    subj_act_1: "~ο~ζω", _, "~ο~ζωμεν";
    subj_act_2: "~ο~ζῃς", "~ο~ζητον", "~ο~ζητε";
    subj_act_3: "~ο~ζῃ", _, "~ο~ζωσῐ|~ο~ζωσῐν";
    subj_mid_1: "~ο~ζωμαι", _, "~ο~ζωμεθᾰ";
    subj_mid_2: _, "~ο~ζησθον", "~ο~ζησθε";
    subj_mid_3: "~ο~ζηται", _, "~ο~ζωνται";
    opt_act_1: "~ο~ζοιμῐ", _, "~ο~ζοιμεν";
    opt_act_2: "~ο~ζοις", "~ο~ζοιτον", "~ο~ζοιτε";
    opt_act_3: "~ο~ζοι", "~ο~ζοιτην", "~ο~ζοιεν";
    opt_mid_1: "~ο~ζοιμην", _, "~ο~ζοιμεθᾰ";
    opt_mid_2: "~ο~ζοιο", "~ο~ζοισθον", "~ο~ζοισθε";
    opt_mid_3: "~ο~ζοιτο", "~ο~ζοισθην", "~ο~ζοιντο";
    imp_act_2: "~ο~ζε", _, _;
    imp_act_3: "~ο~ζετω", "~ο~ζετων", "~ο~ζοντων";
    imp_mid_2: "~ο~ζου", _, _;
    imp_mid_3: "~ο~ζεσθω", "~ο~ζεσθων", _;
    imperfect_act_1: "~ω~ζον", _, "~ω~ζομεν";
    imperfect_act_2: "~ω~ζες", "~ω~ζετον", "~ω~ζετε";
    imperfect_act_3: "~ω~ζε|~ω~ζεν", "~ω~ζετην", "~ω~ζον";
    imperfect_mid_1: "~ω~ζομην", _, "~ω~ζομεθᾰ";
    imperfect_mid_2: "~ω~ζου", "~ω~ζεσθον", "~ω~ζεσθε";
    imperfect_mid_3: "~ω~ζετο", "~ω~ζεσθην", "~ω~ζοντο";
    future_ind_act_1: "~ο~ω", _, "~ο~οῡμεν";
    future_ind_act_2: "~ο~εῑς", "~ο~εῑτον", "~ο~εῑτε";
    future_ind_act_3: "~ο~εῑ", _, "~ο~οῡσῐ|~ο~οῡσῐν";
    future_ind_mid_1: "~ο~οῡμαι", _, "~ο~ουμεθᾰ";
    future_ind_mid_2: "~ο~εῑ|~ο~ῃ", "~ο~εῑσθον", "~ο~εῑσθε";
    future_ind_mid_3: "~ο~εῑται", _, "~ο~οῡνται";
    future_ind_pass_1: "~ο~σθησομαι", _, "~ο~σθησομεθᾰ";
    future_ind_pass_2: "~ο~σθησῃ|~ο~σθησει", "~ο~σθησεσθον", "~ο~σθησεσθε";
    future_ind_pass_3: "~ο~σθησεται", _, "~ο~σθησονται";
    future_opt_act_1: "~ο~οιην|~ο~οῑμῐ", _, "~ο~οῑμεν|~ο~οιημεν";
    future_opt_act_2: "~ο~οιης|~ο~οῑς", "~ο~οῑτον|~ο~οιητον", "~ο~οῑτε|~ο~οιητε";
    future_opt_act_3: "~ο~οιη|~ο~οῑ", "~ο~οιτην|~ο~οιητην", "~ο~οῑεν|~ο~οιησᾰν";
    future_opt_mid_1: "~ο~οιμην", _, "~ο~οιμεθᾰ";
    future_opt_mid_2: "~ο~οῑο", "~ο~οῑσθον", "~ο~οῑσθε";
    future_opt_mid_3: "~ο~οῑτο", "~ο~οισθην", "~ο~οῑντο";
    future_opt_pass_1: "~ο~σθησοιμην", _, "~ο~σθησοιμεθᾰ";
    future_opt_pass_2: "~ο~σθησοιο", "~ο~σθησοισθον", "~ο~σθησοισθε";
    future_opt_pass_3: "~ο~σθησοιτο", "~ο~σθησοισθην", "~ο~σθησοιντο";
    aorist_subj_act_3: _, _, "~ο~σωσῐ|~ο~σωσῐν";
    aorist_subj_pass_1: "~ο~σθω", _, "~ο~σθωμεν";
    aorist_subj_pass_2: "~ο~σθῃς", "~ο~σθητον", "~ο~σθητε";
    aorist_subj_pass_3: "~ο~σθῃ", _, "~ο~σθωσῐ|~ο~σθωσῐν";
    aorist_opt_act_2: "~ο~σειᾰς|~ο~σαις", _, _;
    aorist_opt_act_3: "~ο~σειε|~ο~σειεν|~ο~σαι", _, "~ο~σειᾰν|~ο~σαιεν";
    aorist_opt_pass_1: "~ο~σθειην", _, "~ο~σθεῑμεν|~ο~σθειημεν";
    aorist_opt_pass_2: "~ο~σθειης", "~ο~σθεῑτον|~ο~σθειητον", "~ο~σθεῑτε|~ο~σθειητε";
    aorist_opt_pass_3: "~ο~σθειη", "~ο~σθειτην|~ο~σθειητην", "~ο~σθεῑεν|~ο~σθειησᾰν";
    aorist_imp_pass_2: "~ο~σθητῐ", _, _;
    aorist_imp_pass_3: "~ο~σθητω", "~ο~σθητων", "~ο~σθεντων";
    perfect_ind_act_1: "~ω~κᾰ", _, "~ω~κᾰμεν";
    perfect_ind_act_2: "~ω~κᾰς", "~ω~κᾰτον", "~ω~κᾰτε";
    perfect_ind_act_3: "~ω~κε|~ω~κεν", _, "~ω~κᾱσῐ|~ω~κᾱσῐν";
    perfect_ind_mid_1: "~ω~σμαι", _, "~ω~σμεθᾰ";
    perfect_ind_mid_2: "~ω~σαι", "~ω~σθον", "~ω~σθε";
    perfect_ind_mid_3: "~ω~σται", _, "~ω~δᾰται";
    perfect_subj_act_1: "~ω~κω", _, "~ω~κωμεν";
    perfect_subj_act_2: "~ω~κῃς", "~ω~κητον", "~ω~κητε";
    perfect_subj_act_3: "~ω~κῃ", _, "~ω~κωσῐ|~ω~κωσῐν";
    perfect_opt_act_1: "~ω~κοιμῐ|~ω~κοιην", _, "~ω~κοιμεν";
    perfect_opt_act_2: "~ω~κοις|~ω~κοιης", "~ω~κοιτον", "~ω~κοιτε";
    perfect_opt_act_3: "~ω~κοι|~ω~κοιη", "~ω~κοιτην", "~ω~κοιεν";
    perfect_imp_act_2: "~ω~κε", "~ω~κετον", "~ω~κετε";
    perfect_imp_act_3: "~ω~κετω", "~ω~κετων", "~ω~κοντων";
    perfect_imp_mid_2: "~ω~σο", _, _;
    perfect_imp_mid_3: "~ω~σθω", "~ω~σθων", _;
    infinitive_pres: "~ο~ζειν", "~ο~ζεσθαι", _;
    infinitive_fut: "~ο~εῑν", "~ο~εῑσθαι", "~ο~σθησεσθαι";
    infinitive_aor: _, _, "~ο~σθηναι";
    infinitive_perf: "~ω~κεναι", "~ω~σθαι", _;
    participle_pres: "~ο~ζων", "~ο~ζομενος", _;
    participle_fut: _, "~ο~ουμενος", "~ο~σθησομενος";
    participle_aor: _, _, "~ο~σθεις";
    participle_perf: "~ω~κως", "~ω~σμενος", _;
    ),
    // Κᾰκοω: 3 words (κᾰκοω, ἐξῐσοω, ἡλοω). Like Γῡροω, except in 67 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("κᾰκοω", 3, "~οω", like Γῡροω;
    stem aorist_ind: "~ωσ- like εὐλογεω";
    stem aorist_ind_pass: "~ωθη- like ἐλπω";
    stem pluperfect_act: "~ωκε- like ἀνῐστημῐ";
    imperfect_act_1: "~ουν", _, "~οῡμεν";
    imperfect_act_2: "~ους", "~οῡτον", "~οῡτε";
    imperfect_act_3: "~ου", "~ουτην", "~ουν";
    imperfect_mid_1: "~ουμην", _, "~ουμεθᾰ";
    imperfect_mid_2: "~οῡ", "~οῡσθον", "~οῡσθε";
    imperfect_mid_3: "~οῡτο", "~ουσθην", "~οῡντο";
    future_ind_act_3: _, _, "~ωσουσῐ|~ωσουσῐν";
    perfect_ind_act_1: "~ωκᾰ", _, "~ωκᾰμεν";
    perfect_ind_act_2: "~ωκᾰς", "~ωκᾰτον", "~ωκᾰτε";
    perfect_ind_act_3: "~ωκε|~ωκεν", _, "~ωκᾱσῐ|~ωκᾱσῐν";
    perfect_ind_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    perfect_ind_mid_2: "~ωσαι", "~ωσθον", "~ωσθε";
    perfect_ind_mid_3: "~ωται", _, "~ωνται";
    perfect_subj_act_1: "~ωκω", _, "~ωκωμεν";
    perfect_subj_act_2: "~ωκῃς", "~ωκητον", "~ωκητε";
    perfect_subj_act_3: "~ωκῃ", _, "~ωκωσῐ|~ωκωσῐν";
    perfect_opt_act_1: "~ωκοιμῐ|~ωκοιην", _, "~ωκοιμεν";
    perfect_opt_act_2: "~ωκοις|~ωκοιης", "~ωκοιτον", "~ωκοιτε";
    perfect_opt_act_3: "~ωκοι|~ωκοιη", "~ωκοιτην", "~ωκοιεν";
    perfect_imp_act_2: "~ωκε", "~ωκετον", "~ωκετε";
    perfect_imp_act_3: "~ωκετω", "~ωκετων", "~ωκοντων";
    perfect_imp_mid_2: "~ωσο", _, _;
    perfect_imp_mid_3: "~ωσθω", "~ωσθων", _;
    pluperfect_mid_1: "~ωμην", _, "~ωμεθᾰ";
    pluperfect_mid_2: "~ωσο", "~ωσθον", "~ωσθε";
    pluperfect_mid_3: "~ωτο", "~ωσθην", "~ωντο";
    infinitive_perf: "~ωκεναι", "~ωσθαι", _;
    participle_perf: "~ωκως", "~ωμενος", _;
    ),
    // Δεω: 3 words (δεω, φλεω, κλεω). Like Δῐκεω, except in 94 cells: imp_act_2 sg, future_ind_act_3 pl, future_ind_mid_2 sg, future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, ….
    conjugation!("δεω", 3, "~εω", like Δῐκεω as PERFECT_AS_FUTURE;
    stem aorist_ind: "~ησ- like ἐχω";
    stem aorist_ind_pass: "~εθη- like ἐχω";
    stem pluperfect_act: "~εδεκε- like κᾰλεω";
    imp_act_2: "~εῑ", _, _;
    future_ind_act_3: _, _, "~ησουσῐ";
    future_ind_mid_2: "~ησῃ", _, _;
    future_ind_pass_1: "~εθησομαι", _, "~εθησομεθᾰ";
    future_ind_pass_2: "~εθησῃ", "~εθησεσθον", "~εθησεσθε";
    future_ind_pass_3: "~εθησεται", _, "~εθησονται";
    future_opt_pass_1: "~εθησοιμην", _, "~εθησοιμεθᾰ";
    future_opt_pass_2: "~εθησοιο", "~εθησοισθον", "~εθησοισθε";
    future_opt_pass_3: "~εθησοιτο", "~εθησοισθην", "~εθησοιντο";
    aorist_subj_act_3: _, _, "~ησωσῐ";
    aorist_subj_pass_1: "~εθω", _, "~εθωμεν";
    aorist_subj_pass_2: "~εθῃς", "~εθητον", "~εθητε";
    aorist_subj_pass_3: "~εθῃ", _, "~εθωσῐ";
    aorist_opt_act_2: "~ησειᾰς", _, _;
    aorist_opt_act_3: "~ησειε", _, "~ησειᾰν";
    aorist_opt_pass_1: "~εθειην", _, "~εθεῑμεν";
    aorist_opt_pass_2: "~εθειης", "~εθεῑτον", "~εθεῑτε";
    aorist_opt_pass_3: "~εθειη", "~εθειτην", "~εθεῑεν";
    aorist_imp_pass_2: "~εθητῐ", _, _;
    aorist_imp_pass_3: "~εθητω", "~εθητων", "~εθεντων";
    perfect_ind_act_1: "~εδεκᾰ", _, "~εδεκᾰμεν";
    perfect_ind_act_2: "~εδεκᾰς", "~εδεκᾰτον", "~εδεκᾰτε";
    perfect_ind_act_3: "~εδεκε", _, "~εδεκᾱσῐ";
    perfect_ind_mid_1: "~εδεμαι", _, "~εδεμεθᾰ";
    perfect_ind_mid_2: "~εδεσαι", "~εδεσθον", "~εδεσθε";
    perfect_ind_mid_3: "~εδεται", _, "~εδενται";
    perfect_subj_act_1: "~εδεκω", _, "~εδεκωμεν";
    perfect_subj_act_2: "~εδεκῃς", "~εδεκητον", "~εδεκητε";
    perfect_subj_act_3: "~εδεκῃ", _, "~εδεκωσῐ";
    perfect_opt_act_1: "~εδεκοιμῐ", _, "~εδεκοιμεν";
    perfect_opt_act_2: "~εδεκοις", "~εδεκοιτον", "~εδεκοιτε";
    perfect_opt_act_3: "~εδεκοι", "~εδεκοιτην", "~εδεκοιεν";
    perfect_imp_act_2: _, "~εδεκετον", "~εδεκετε";
    perfect_imp_act_3: "~εδεκετω", "~εδεκετων", "~εδεκοντων";
    perfect_imp_mid_2: "~εδεσο", _, _;
    perfect_imp_mid_3: "~εδεσθω", "~εδεσθων", _;
    pluperfect_mid_1: "ἐ~εδεμην", _, "ἐ~εδεμεθᾰ";
    pluperfect_mid_2: "ἐ~εδεσο", "ἐ~εδεσθον", "ἐ~εδεσθε";
    pluperfect_mid_3: "ἐ~εδετο", "ἐ~εδεσθην", "ἐ~εδεντο";
    infinitive_fut: _, _, "~εθησεσθαι";
    infinitive_aor: _, _, "~εθηναι";
    infinitive_perf: "~εδεκεναι", "~εδεσθαι", _;
    participle_fut: _, _, "~εθησομενος";
    participle_aor: _, _, "~εθεις";
    participle_perf: "~εδεκως", "~εδεμενος", _;
    ),
    // Ἀποζω: 3 words (ζω, ἀποζω, ἀνᾰζω). Like Στερεω, except in 129 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ἀποζω", 3, "~ω", like Στερεω as MID_AS_PASS;
    ind_act_1: _, _, "~ωμεν";
    ind_act_2: "~ῃς", "~ητον", "~ητε";
    ind_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    opt_act_1: "~ῳην|~ῳμῐ", _, "~ῳμεν|~ῳημεν";
    opt_act_2: "~ῳης|~ῳς", "~ῳτον|~ῳητον", "~ῳτε|~ῳητε";
    opt_act_3: "~ῳη|~ῳ", "~ῳτην|~ῳητην", "~ῳεν|~ῳησᾰν";
    imp_act_2: "~η", _, _;
    imp_act_3: "~ητω", "~ητων", "~ωντων";
    imperfect_act_1: "ἐ~ων", _, "ἐ~ωμεν";
    imperfect_act_2: "ἐ~ης", "ἐ~ητον", "ἐ~ητε";
    imperfect_act_3: "ἐ~η", "ἐ~ητην", "ἐ~ων";
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", "~ησετε";
    future_ind_act_3: "~ησει", _, "~ησουσῐ";
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: "~ησοις", "~ησοιτον", "~ησοιτε";
    future_opt_act_3: "~ησοι", "~ησοιτην", "~ησοιεν";
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_mid_1: "ἐ~ησᾰμην", _, "ἐ~ησᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ησω", "ἐ~ησᾰσθον", "ἐ~ησᾰσθε";
    aorist_ind_mid_3: "ἐ~ησᾰτο", "ἐ~ησᾰσθην", "ἐ~ησᾰντο";
    aorist_ind_pass_1: derived, _, derived;
    aorist_ind_pass_2: derived, derived, derived;
    aorist_ind_pass_3: derived, derived, derived;
    aorist_subj_mid_1: "~ησωμαι", _, "~ησωμεθᾰ";
    aorist_subj_mid_2: derived, "~ησησθον", "~ησησθε";
    aorist_subj_mid_3: "~ησηται", _, "~ησωνται";
    aorist_subj_pass_1: derived, _, derived;
    aorist_subj_pass_2: derived, derived, derived;
    aorist_subj_pass_3: derived, _, derived;
    aorist_opt_mid_1: "~ησαιμην", _, "~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ησαιο", "~ησαισθον", "~ησαισθε";
    aorist_opt_mid_3: "~ησαιτο", "~ησαισθην", "~ησαιντο";
    aorist_opt_pass_1: derived, _, derived;
    aorist_opt_pass_2: derived, derived, derived;
    aorist_opt_pass_3: derived, derived, derived;
    aorist_imp_mid_2: "~ησαι", "~ησᾰσθον", "~ησᾰσθε";
    aorist_imp_mid_3: "~ησᾰσθω", "~ησᾰσθων", derived;
    aorist_imp_pass_2: derived, _, _;
    aorist_imp_pass_3: derived, derived, derived;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    infinitive_pres: "~ην", _, _;
    infinitive_fut: "~ησειν", _, derived;
    infinitive_aor: _, "~ησᾰσθαι", derived;
    participle_fut: "~ησων", _, derived;
    participle_aor: _, "~ησᾰμενος", derived;
    ),
    // Πλᾰναω: 3 words (δαμναω, πλᾰναω, γαναω). Like Μῑγνῡμῐ, except in 210 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("πλᾰναω", 3, "~ναω", like Μῑγνῡμῐ;
    stem aorist_ind: "~νησ- like ἐχω";
    stem aorist_ind_pass: "~νηθη- like ἐχω";
    stem pluperfect_act: "~νηκε- like ἐχω";
    stem pluperfect: "~νη- like πειρᾰω";
    ind_act_1: "~νω", _, "~νωμεν";
    ind_act_2: "~νᾳς", "~νᾱτον", "~νᾱτε";
    ind_act_3: "~νᾳ", _, "~νωσῐ";
    ind_mid_1: "~νωμαι", _, "~νωμεθᾰ";
    ind_mid_2: "~νᾳ", "~νᾱσθον", "~νᾱσθε";
    ind_mid_3: "~νᾱται", _, "~νωνται";
    subj_act_1: "~νω", _, "~νωμεν";
    subj_act_2: "~νᾳς", "~νᾱτον", "~νᾱτε";
    subj_act_3: "~νᾳ", _, "~νωσῐ";
    subj_mid_1: "~νωμαι", _, "~νωμεθᾰ";
    subj_mid_2: _, "~νᾱσθον", "~νᾱσθε";
    subj_mid_3: "~νᾱται", _, "~νωνται";
    opt_act_1: "~νῳην", _, "~νῳμεν";
    opt_act_2: "~νῳης", "~νῳτον", "~νῳτε";
    opt_act_3: "~νῳη", "~νῳτην", "~νῳεν";
    opt_mid_1: "~νῳμην", _, "~νῳμεθᾰ";
    opt_mid_2: "~νῳο", "~νῳσθον", "~νῳσθε";
    opt_mid_3: "~νῳτο", "~νῳσθην", "~νῳντο";
    imp_act_2: "~νᾱ", _, _;
    imp_act_3: "~νᾱτω", "~νᾱτων", "~νωντων";
    imp_mid_2: "~νω", _, _;
    imp_mid_3: "~νᾱσθω", "~νᾱσθων", _;
    imperfect_act_1: "ἐ~νων", _, "ἐ~νωμεν";
    imperfect_act_2: "ἐ~νᾱς", "ἐ~νᾱτον", "ἐ~νᾱτε";
    imperfect_act_3: "ἐ~νᾱ", "ἐ~νᾱτην", "ἐ~νων";
    imperfect_mid_1: "ἐ~νωμην", _, "ἐ~νωμεθᾰ";
    imperfect_mid_2: "ἐ~νω", "ἐ~νᾱσθον", "ἐ~νᾱσθε";
    imperfect_mid_3: "ἐ~νᾱτο", "ἐ~νᾱσθην", "ἐ~νωντο";
    future_ind_mid_1: "~γησομαι", _, "~γησομεθᾰ";
    future_ind_mid_2: "~γησῃ", "~γησεσθον", "~γησεσθε";
    future_ind_mid_3: "~γησεται", _, "~γησονται";
    future_opt_mid_1: "~γησοιμην", _, "~γησοιμεθᾰ";
    future_opt_mid_2: "~γησοιο", "~γησοισθον", "~γησοισθε";
    future_opt_mid_3: "~γησοιτο", "~γησοισθην", "~γησοιντο";
    aorist_subj_act_1: "~νησω", _, "~νησωμεν";
    aorist_subj_act_2: "~νησῃς", "~νησητον", "~νησητε";
    aorist_subj_act_3: "~νησῃ", _, "~νησωσῐ";
    aorist_subj_mid_1: "~νησωμαι", _, "~νησωμεθᾰ";
    aorist_subj_mid_2: _, "~νησησθον", "~νησησθε";
    aorist_subj_mid_3: "~νησηται", _, "~νησωνται";
    aorist_subj_pass_1: "~νηθω", _, "~νηθωμεν";
    aorist_subj_pass_2: "~νηθῃς", "~νηθητον", "~νηθητε";
    aorist_subj_pass_3: "~νηθῃ", _, "~νηθωσῐ";
    aorist_opt_act_1: "~νησαιμῐ", _, "~νησαιμεν";
    aorist_opt_act_2: "~νησειᾰς", "~νησαιτον", "~νησαιτε";
    aorist_opt_act_3: "~νησειε", "~νησαιτην", "~νησειᾰν";
    aorist_opt_mid_1: "~νησαιμην", _, "~νησαιμεθᾰ";
    aorist_opt_mid_2: "~νησαιο", "~νησαισθον", "~νησαισθε";
    aorist_opt_mid_3: "~νησαιτο", "~νησαισθην", "~νησαιντο";
    aorist_opt_pass_1: "~νηθειην", _, "~νηθεῑμεν";
    aorist_opt_pass_2: "~νηθειης", "~νηθεῑτον", "~νηθεῑτε";
    aorist_opt_pass_3: "~νηθειη", "~νηθειτην", "~νηθεῑεν";
    aorist_imp_act_2: "~νησον", "~νησᾰτον", "~νησᾰτε";
    aorist_imp_act_3: "~νησᾰτω", "~νησᾰτων", "~νησᾰντων";
    aorist_imp_mid_2: "~νησαι", "~νησᾰσθον", "~νησᾰσθε";
    aorist_imp_mid_3: "~νησᾰσθω", "~νησᾰσθων", _;
    aorist_imp_pass_2: "~νηθητῐ", _, _;
    aorist_imp_pass_3: "~νηθητω", "~νηθητων", "~νηθεντων";
    perfect_ind_act_1: "%ε~νηκᾰ", _, "%ε~νηκᾰμεν";
    perfect_ind_act_2: "%ε~νηκᾰς", "%ε~νηκᾰτον", "%ε~νηκᾰτε";
    perfect_ind_act_3: "%ε~νηκε", _, "%ε~νηκᾱσῐ";
    perfect_ind_mid_1: "%ε~νημαι", _, "%ε~νημεθᾰ";
    perfect_ind_mid_2: "%ε~νησαι", "%ε~νησθον", "%ε~νησθε";
    perfect_ind_mid_3: "%ε~νηται", _, "%ε~νηνται";
    perfect_subj_act_1: "%ε~νηκω", _, "%ε~νηκωμεν";
    perfect_subj_act_2: "%ε~νηκῃς", "%ε~νηκητον", "%ε~νηκητε";
    perfect_subj_act_3: "%ε~νηκῃ", _, "%ε~νηκωσῐ";
    perfect_opt_act_1: "%ε~νηκοιμῐ", _, "%ε~νηκοιμεν";
    perfect_opt_act_2: "%ε~νηκοις", "%ε~νηκοιτον", "%ε~νηκοιτε";
    perfect_opt_act_3: "%ε~νηκοι", "%ε~νηκοιτην", "%ε~νηκοιεν";
    perfect_imp_act_2: _, "%ε~νηκετον", "%ε~νηκετε";
    perfect_imp_act_3: "%ε~νηκετω", "%ε~νηκετων", "%ε~νηκοντων";
    perfect_imp_mid_2: "%ε~νησο", _, _;
    perfect_imp_mid_3: "%ε~νησθω", "%ε~νησθων", _;
    future_perfect_ind_mid_1: "%ε~νησομαι", _, "%ε~νησομεθᾰ";
    future_perfect_ind_mid_2: "%ε~νησῃ", "%ε~νησεσθον", "%ε~νησεσθε";
    future_perfect_ind_mid_3: "%ε~νησεται", _, "%ε~νησονται";
    future_perfect_opt_mid_1: "%ε~νησοιμην", _, "%ε~νησοιμεθᾰ";
    future_perfect_opt_mid_2: "%ε~νησοιο", "%ε~νησοισθον", "%ε~νησοισθε";
    future_perfect_opt_mid_3: "%ε~νησοιτο", "%ε~νησοισθην", "%ε~νησοιντο";
    infinitive_pres: "~νᾱν", "~νᾱσθαι", _;
    infinitive_fut: _, "~γησεσθαι", _;
    infinitive_aor: "~νησαι", "~νησᾰσθαι", "~νηθηναι";
    infinitive_perf: "%ε~νηκεναι", "%ε~νησθαι", _;
    infinitive_futp: _, "%ε~νησεσθαι", _;
    participle_pres: "~νων", "~νωμενος", _;
    participle_fut: _, "~γησομενος", _;
    participle_aor: "~νησᾱς", "~νησᾰμενος", "~νηθεις";
    participle_perf: "%ε~νηκως", "%ε~νημενος", _;
    participle_futp: _, "%ε~νησομενος", _;
    ),
    // Ἀμειβω: 3 words (στειβω, λειβω, ἀμειβω). Like Μῑγνῡμῐ, except in 114 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀμειβω", 3, "~ιβω", like Μῑγνῡμῐ;
    stem aorist_ind: "~ιψ- like ποιεω";
    ind_act_1: "~ιβω", _, "~ιβομεν";
    ind_act_2: "~ιβεις", "~ιβετον", "~ιβετε";
    ind_act_3: "~ιβει", _, "~ιβουσῐ|~ιβουσῐν";
    ind_mid_1: "~ιβομαι", _, "~ιβομεθᾰ";
    ind_mid_2: "~ιβῃ|~ιβει", "~ιβεσθον", "~ιβεσθε";
    ind_mid_3: "~ιβεται", _, "~ιβονται";
    subj_act_1: "~ιβω", _, "~ιβωμεν";
    subj_act_2: "~ιβῃς", "~ιβητον", "~ιβητε";
    subj_act_3: "~ιβῃ", _, "~ιβωσῐ|~ιβωσῐν";
    subj_mid_1: "~ιβωμαι", _, "~ιβωμεθᾰ";
    subj_mid_2: _, "~ιβησθον", "~ιβησθε";
    subj_mid_3: "~ιβηται", _, "~ιβωνται";
    opt_act_1: "~ιβοιμῐ", _, "~ιβοιμεν";
    opt_act_2: "~ιβοις", "~ιβοιτον", "~ιβοιτε";
    opt_act_3: "~ιβοι", "~ιβοιτην", "~ιβοιεν";
    opt_mid_1: "~ιβοιμην", _, "~ιβοιμεθᾰ";
    opt_mid_2: "~ιβοιο", "~ιβοισθον", "~ιβοισθε";
    opt_mid_3: "~ιβοιτο", "~ιβοισθην", "~ιβοιντο";
    imp_act_2: "~ῑβε", _, _;
    imp_act_3: "~ιβετω", "~ιβετων", "~ιβοντων";
    imp_mid_2: "~ιβου", _, _;
    imp_mid_3: "~ιβεσθω", "~ιβεσθων", _;
    imperfect_act_1: "ἐ~ιβον", _, "ἐ~ιβομεν";
    imperfect_act_2: "ἐ~ιβες", "ἐ~ιβετον", "ἐ~ιβετε";
    imperfect_act_3: "ἐ~ιβε|ἐ~ιβεν", "ἐ~ιβετην", "ἐ~ιβον";
    imperfect_mid_1: "ἐ~ιβομην", _, "ἐ~ιβομεθᾰ";
    imperfect_mid_2: "ἐ~ιβου", "ἐ~ιβεσθον", "ἐ~ιβεσθε";
    imperfect_mid_3: "ἐ~ιβετο", "ἐ~ιβεσθην", "ἐ~ιβοντο";
    aorist_subj_act_1: "~ιψω", _, "~ιψωμεν";
    aorist_subj_act_2: "~ιψῃς", "~ιψητον", "~ιψητε";
    aorist_subj_act_3: "~ιψῃ", _, "~ιψωσῐ|~ιψωσῐν";
    aorist_subj_mid_1: "~ιψωμαι", _, "~ιψωμεθᾰ";
    aorist_subj_mid_2: _, "~ιψησθον", "~ιψησθε";
    aorist_subj_mid_3: "~ιψηται", _, "~ιψωνται";
    aorist_opt_act_1: "~ιψαιμῐ", _, "~ιψαιμεν";
    aorist_opt_act_2: "~ιψειᾰς|~ιψαις", "~ιψαιτον", "~ιψαιτε";
    aorist_opt_act_3: "~ιψειε|~ιψειεν|~ιψαι", "~ιψαιτην", "~ιψειᾰν|~ιψαιεν";
    aorist_opt_mid_1: "~ιψαιμην", _, "~ιψαιμεθᾰ";
    aorist_opt_mid_2: "~ιψαιο", "~ιψαισθον", "~ιψαισθε";
    aorist_opt_mid_3: "~ιψαιτο", "~ιψαισθην", "~ιψαιντο";
    aorist_imp_act_2: "~ῑψον", "~ιψᾰτον", "~ιψᾰτε";
    aorist_imp_act_3: "~ιψᾰτω", "~ιψᾰτων", "~ιψᾰντων";
    aorist_imp_mid_2: "~ῑψαι", "~ιψᾰσθον", "~ιψᾰσθε";
    aorist_imp_mid_3: "~ιψᾰσθω", "~ιψᾰσθων", _;
    infinitive_pres: "~ιβειν", "~ιβεσθαι", _;
    infinitive_aor: "~ῑψαι", "~ιψᾰσθαι", _;
    participle_pres: "~ιβων", "~ιβομενος", _;
    participle_aor: "~ιψᾱς", "~ιψᾰμενος", _;
    ),
    // Δῐᾰρπᾰζω: 3 words (ἐξᾰφᾰνῐζω, δῐᾰρπᾰζω, ἐξᾰττῐκῐζω). Like Ἐπᾰϊω, except in 119 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("δῐᾰρπᾰζω", 3, "~ᾰ~ζω", like Ἐπᾰϊω;
    stem pluperfect_act: "~η~κ- like εὐλογεω";
    stem pluperfect: "~η~- like ὑποκοριζομαι";
    ind_act_1: "~ᾰ~ζω", _, "~ᾰ~ζομεν";
    ind_act_2: "~ᾰ~ζεις", "~ᾰ~ζετον", "~ᾰ~ζετε";
    ind_act_3: "~ᾰ~ζει", _, "~ᾰ~ζουσῐ|~ᾰ~ζουσῐν";
    ind_mid_1: "~ᾰ~ζομαι", _, "~ᾰ~ζομεθᾰ";
    ind_mid_2: "~ᾰ~ζῃ|~ᾰ~ζει", "~ᾰ~ζεσθον", "~ᾰ~ζεσθε";
    ind_mid_3: "~ᾰ~ζεται", _, "~ᾰ~ζονται";
    subj_act_1: "~ᾰ~ζω", _, "~ᾰ~ζωμεν";
    subj_act_2: "~ᾰ~ζῃς", "~ᾰ~ζητον", "~ᾰ~ζητε";
    subj_act_3: "~ᾰ~ζῃ", _, "~ᾰ~ζωσῐ|~ᾰ~ζωσῐν";
    subj_mid_1: "~ᾰ~ζωμαι", _, "~ᾰ~ζωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ζησθον", "~ᾰ~ζησθε";
    subj_mid_3: "~ᾰ~ζηται", _, "~ᾰ~ζωνται";
    opt_act_1: "~ᾰ~ζοιμῐ", _, "~ᾰ~ζοιμεν";
    opt_act_2: "~ᾰ~ζοις", "~ᾰ~ζοιτον", "~ᾰ~ζοιτε";
    opt_act_3: "~ᾰ~ζοι", "~ᾰ~ζοιτην", "~ᾰ~ζοιεν";
    opt_mid_1: "~ᾰ~ζοιμην", _, "~ᾰ~ζοιμεθᾰ";
    opt_mid_2: "~ᾰ~ζοιο", "~ᾰ~ζοισθον", "~ᾰ~ζοισθε";
    opt_mid_3: "~ᾰ~ζοιτο", "~ᾰ~ζοισθην", "~ᾰ~ζοιντο";
    imp_act_2: "~ᾰ~ζε", _, _;
    imp_act_3: "~ᾰ~ζετω", "~ᾰ~ζετων", "~ᾰ~ζοντων";
    imp_mid_2: "~ᾰ~ζου", _, _;
    imp_mid_3: "~ᾰ~ζεσθω", "~ᾰ~ζεσθων", _;
    imperfect_act_1: "~η~ζον", _, "~η~ζομεν";
    imperfect_act_2: "~η~ζες", "~η~ζετον", "~η~ζετε";
    imperfect_act_3: "~η~ζε|~η~ζεν", "~η~ζετην", "~η~ζον";
    imperfect_mid_1: "~η~ζομην", _, "~η~ζομεθᾰ";
    imperfect_mid_2: "~η~ζου", "~η~ζεσθον", "~η~ζεσθε";
    imperfect_mid_3: "~η~ζετο", "~η~ζεσθην", "~η~ζοντο";
    future_ind_act_1: "~ᾰ~ω", _, "~ᾰ~οῡμεν";
    future_ind_act_2: "~ᾰ~εῑς", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    future_ind_act_3: "~ᾰ~εῑ", _, "~ᾰ~οῡσῐ|~ᾰ~οῡσῐν";
    future_ind_mid_1: "~ᾰ~οῡμαι", _, "~ᾰ~ουμεθᾰ";
    future_ind_mid_2: "~ᾰ~εῑ|~ᾰ~ῃ", "~ᾰ~εῑσθον", "~ᾰ~εῑσθε";
    future_ind_mid_3: "~ᾰ~εῑται", _, "~ᾰ~οῡνται";
    future_ind_pass_2: "~ᾰ~σθησῃ|~ᾰ~σθησει", _, _;
    future_opt_act_1: "~ᾰ~οιην|~ᾰ~οῑμῐ", _, "~ᾰ~οῑμεν|~ᾰ~οιημεν";
    future_opt_act_2: "~ᾰ~οιης|~ᾰ~οῑς", "~ᾰ~οῑτον|~ᾰ~οιητον", "~ᾰ~οῑτε|~ᾰ~οιητε";
    future_opt_act_3: "~ᾰ~οιη|~ᾰ~οῑ", "~ᾰ~οιτην|~ᾰ~οιητην", "~ᾰ~οῑεν|~ᾰ~οιησᾰν";
    future_opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    future_opt_mid_2: "~ᾰ~οῑο", "~ᾰ~οῑσθον", "~ᾰ~οῑσθε";
    future_opt_mid_3: "~ᾰ~οῑτο", "~ᾰ~οισθην", "~ᾰ~οῑντο";
    aorist_subj_pass_3: _, _, "~ᾰ~σθωσῐ|~ᾰ~σθωσῐν";
    aorist_opt_pass_1: _, _, "~ᾰ~σθεῑμεν|~ᾰ~σθειημεν";
    aorist_opt_pass_2: _, "~ᾰ~σθεῑτον|~ᾰ~σθειητον", "~ᾰ~σθεῑτε|~ᾰ~σθειητε";
    aorist_opt_pass_3: _, "~ᾰ~σθειτην|~ᾰ~σθειητην", "~ᾰ~σθεῑεν|~ᾰ~σθειησᾰν";
    perfect_ind_act_3: "~η~κε|~η~κεν", _, "~η~κᾱσῐ|~η~κᾱσῐν";
    perfect_ind_mid_3: _, _, "~η~δᾰται";
    perfect_subj_act_3: _, _, "~η~κωσῐ|~η~κωσῐν";
    perfect_opt_act_1: "~η~κοιμῐ|~η~κοιην", _, _;
    perfect_opt_act_2: "~η~κοις|~η~κοιης", _, _;
    perfect_opt_act_3: "~η~κοι|~η~κοιη", _, _;
    perfect_imp_act_2: "~η~κε", _, _;
    infinitive_pres: "~ᾰ~ζειν", "~ᾰ~ζεσθαι", _;
    infinitive_fut: "~ᾰ~εῑν", "~ᾰ~εῑσθαι", _;
    participle_pres: "~ᾰ~ζων", "~ᾰ~ζομενος", _;
    participle_fut: "~ᾰ~ων", "~ᾰ~ουμενος", _;
    ),
    // Ὀμνῠω: 3 words (ὀφελλω, ὀμνῠω, ὀρῠττω). Like Μῑγνῡμῐ, except in 70 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ὀμνῠω", 3, "ὀ~ω", like Μῑγνῡμῐ;
    ind_act_1: "ὀ~ω", _, "ὀ~ομεν";
    ind_act_2: "ὀ~εις", "ὀ~ετον", "ὀ~ετε";
    ind_act_3: "ὀ~ει", _, "ὀ~ουσῐ|ὀ~ουσῐν";
    ind_mid_1: "ὀ~ομαι", _, "ὀ~ομεθᾰ";
    ind_mid_2: "ὀ~ῃ|ὀ~ει", "ὀ~εσθον", "ὀ~εσθε";
    ind_mid_3: "ὀ~εται", _, "ὀ~ονται";
    subj_act_1: "ὀ~ω", _, "ὀ~ωμεν";
    subj_act_2: "ὀ~ῃς", "ὀ~ητον", "ὀ~ητε";
    subj_act_3: "ὀ~ῃ", _, "ὀ~ωσῐ|ὀ~ωσῐν";
    subj_mid_1: "ὀ~ωμαι", _, "ὀ~ωμεθᾰ";
    subj_mid_2: _, "ὀ~ησθον", "ὀ~ησθε";
    subj_mid_3: "ὀ~ηται", _, "ὀ~ωνται";
    opt_act_1: "ὀ~οιμῐ", _, "ὀ~οιμεν";
    opt_act_2: "ὀ~οις", "ὀ~οιτον", "ὀ~οιτε";
    opt_act_3: "ὀ~οι", "ὀ~οιτην", "ὀ~οιεν";
    opt_mid_1: "ὀ~οιμην", _, "ὀ~οιμεθᾰ";
    opt_mid_2: "ὀ~οιο", "ὀ~οισθον", "ὀ~οισθε";
    opt_mid_3: "ὀ~οιτο", "ὀ~οισθην", "ὀ~οιντο";
    imp_act_2: "ὀ~ε", _, _;
    imp_act_3: "ὀ~ετω", "ὀ~ετων", "ὀ~οντων";
    imp_mid_2: "ὀ~ου", _, _;
    imp_mid_3: "ὀ~εσθω", "ὀ~εσθων", _;
    imperfect_act_1: "ὠ~ον", _, "ὠ~ομεν";
    imperfect_act_2: "ὠ~ες", "ὠ~ετον", "ὠ~ετε";
    imperfect_act_3: "ὠ~ε|ὠ~εν", "ὠ~ετην", "ὠ~ον";
    imperfect_mid_1: "ὠ~ομην", _, "ὠ~ομεθᾰ";
    imperfect_mid_2: "ὠ~ου", "ὠ~εσθον", "ὠ~εσθε";
    imperfect_mid_3: "ὠ~ετο", "ὠ~εσθην", "ὠ~οντο";
    infinitive_pres: "ὀ~ειν", "ὀ~εσθαι", _;
    participle_pres: "ὀ~ων", "ὀ~ομενος", _;
    ),
    // Ψᾱω: 3 words (δῐψᾱω, νᾱω, ψᾱω). Like Νῑκαω, except in 46 cells: ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_mid_2 sg, ind_mid_2 du, ….
    conjugation!("ψᾱω", 3, "~ᾱω", like Νῑκαω as PERFECT_AS_FUTURE;
    ind_act_2: "~ῃς", "~ητον", "~ητε";
    ind_act_3: "~ῃ", _, _;
    ind_mid_2: "~ῃ", "~ησθον", "~ησθε";
    ind_mid_3: "~ηται", _, _;
    subj_act_2: "~ῃς", "~ητον", "~ητε";
    subj_act_3: "~ῃ", _, _;
    subj_mid_2: _, "~ησθον", "~ησθε";
    subj_mid_3: "~ηται", _, _;
    imp_act_2: "~η", _, _;
    imp_act_3: "~ητω", "~ητων", _;
    imp_mid_3: "~ησθω", "~ησθων", _;
    imperfect_act_2: "ἐ~ης", "ἐ~ητον", "ἐ~ητε";
    imperfect_act_3: "ἐ~η", "ἐ~ητην", _;
    imperfect_mid_2: _, "ἐ~ησθον", "ἐ~ησθε";
    imperfect_mid_3: "ἐ~ητο", "ἐ~ησθην", _;
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    perfect_ind_mid_1: "ἐ~ημαι", _, "ἐ~ημεθᾰ";
    perfect_ind_mid_2: "ἐ~ησαι", "ἐ~ησθον", "ἐ~ησθε";
    perfect_ind_mid_3: "ἐ~ηται", _, "ἐ~ηνται";
    perfect_imp_mid_2: "ἐ~ησο", _, _;
    perfect_imp_mid_3: "ἐ~ησθω", "ἐ~ησθων", _;
    infinitive_pres: "~ην", "~ησθαι", _;
    infinitive_perf: derived, "ἐ~ησθαι", _;
    participle_perf: derived, "ἐ~ημενος", _;
    ),
    // Ἐπῐτῐθημῐ: 3 words (ἐπῐτῐθημῐ, συνεπῐτῐθημῐ, ξυνεπῐτῐθημῐ). Like Ἐπιτρεχω, except in 207 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐπῐτῐθημῐ", 3, "~ῐτῐ~ημῐ", like Ἐπιτρεχω as PRESENT_AS_FUTURE;
    stem aorist_ind: "~ε~- like ἐπῐτῐθημῐ";
    ind_act_1: "~ῐ%ῐ~ημῐ", _, "~ῐ%ῐ~εμεν";
    ind_act_2: "~ῐ%ῐ~ης", "~ῐ%ῐ~ετον", "~ῐ%ῐ~ετε";
    ind_act_3: "~ῐ%ῐ~ησῐ|~ῐ%ῐ~ησῐν", _, "~ῐ%ῐ~εᾱσῐ|~ῐ%ῐ~εᾱσῐν";
    ind_mid_1: "~ῐ%ῐ~εμαι", _, "~ῐ%ῐ~εμεθᾰ";
    ind_mid_2: "~ῐ%ῐ~εσαι", "~ῐ%ῐ~εσθον", "~ῐ%ῐ~εσθε";
    ind_mid_3: "~ῐ%ῐ~εται", _, "~ῐ%ῐ~ενται";
    subj_act_1: "~ῐ%ῐ~ω", _, "~ῐ%ῐ~ωμεν";
    subj_act_2: "~ῐ%ῐ~ῃς", "~ῐ%ῐ~ητον", "~ῐ%ῐ~ητε";
    subj_act_3: "~ῐ%ῐ~ῃ", _, "~ῐ%ῐ~ωσῐ|~ῐ%ῐ~ωσῐν";
    subj_mid_1: "~ῐ%ῐ~ωμαι", _, "~ῐ%ῐ~ωμεθᾰ";
    subj_mid_2: _, "~ῐ%ῐ~ησθον", "~ῐ%ῐ~ησθε";
    subj_mid_3: "~ῐ%ῐ~ηται", _, "~ῐ%ῐ~ωνται";
    opt_act_1: "~ῐ%ῐ~ειην", _, "~ῐ%ῐ~εῑμεν|~ῐ%ῐ~ειημεν";
    opt_act_2: "~ῐ%ῐ~ειης", "~ῐ%ῐ~εῑτον|~ῐ%ῐ~ειητον", "~ῐ%ῐ~εῑτε|~ῐ%ῐ~ειητε";
    opt_act_3: "~ῐ%ῐ~ειη", "~ῐ%ῐ~ειτην|~ῐ%ῐ~ειητην", "~ῐ%ῐ~εῑεν|~ῐ%ῐ~ειησᾰν";
    opt_mid_1: "~ῐ%ῐ~ειμην", _, "~ῐ%ῐ~ειμεθᾰ|~ῐ%ῐ~οιμεθᾰ";
    opt_mid_2: "~ῐ%ῐ~εῑο", "~ῐ%ῐ~εῑσθον|~ῐ%ῐ~οῑσθον", "~ῐ%ῐ~εῑσθε|~ῐ%ῐ~οῑσθε";
    opt_mid_3: "~ῐ%ῐ~εῑτο|~ῐ%ῐ~οῑτο", "~ῐ%ῐ~εισθην|~ῐ%ῐ~οισθην", "~ῐ%ῐ~εῑντο|~ῐ%ῐ~οῑντο";
    imp_act_2: "~ῐ%ῐ~ει", _, _;
    imp_act_3: "~ῐ%ῐ~ετω", "~ῐ%ῐ~ετων", "~ῐ%ῐ~εντων";
    imp_mid_2: "~ῐ%ῐ~εσο|~ῐ%ῐ~ου", _, _;
    imp_mid_3: "~ῐ%ῐ~εσθω", "~ῐ%ῐ~εσθων", _;
    imperfect_act_1: "~ε%ῐ~ην", _, "~ε%ῐ~εμεν";
    imperfect_act_2: "~ε%ῐ~εις", "~ε%ῐ~ετον", "~ε%ῐ~ετε";
    imperfect_act_3: "~ε%ῐ~ει", "~ε%ῐ~ετην", "~ε%ῐ~εσᾰν";
    imperfect_mid_1: "~ε%ῐ~εμην", _, "~ε%ῐ~εμεθᾰ";
    imperfect_mid_2: "~ε%ῐ~εσο", "~ε%ῐ~εσθον", "~ε%ῐ~εσθε";
    imperfect_mid_3: "~ε%ῐ~ετο", "~ε%ῐ~εσθην", "~ε%ῐ~εντο";
    future_ind_act_1: "~ῐ~ησω", _, "~ῐ~ησομεν";
    future_ind_act_2: "~ῐ~ησεις", "~ῐ~ησετον", "~ῐ~ησετε";
    future_ind_act_3: "~ῐ~ησει", _, "~ῐ~ησουσῐ|~ῐ~ησουσῐν";
    future_ind_mid_1: "~ῐ~ησομαι", _, "~ῐ~ησομεθᾰ";
    future_ind_mid_2: "~ῐ~ησῃ|~ῐ~ησει", "~ῐ~ησεσθον", "~ῐ~ησεσθε";
    future_ind_mid_3: "~ῐ~ησεται", _, "~ῐ~ησονται";
    future_ind_pass_1: "~ῐ%ε~ησομαι", _, "~ῐ%ε~ησομεθᾰ";
    future_ind_pass_2: "~ῐ%ε~ησῃ|~ῐ%ε~ησει", "~ῐ%ε~ησεσθον", "~ῐ%ε~ησεσθε";
    future_ind_pass_3: "~ῐ%ε~ησεται", _, "~ῐ%ε~ησονται";
    future_opt_act_1: "~ῐ~ησοιμῐ", _, "~ῐ~ησοιμεν";
    future_opt_act_2: "~ῐ~ησοις", "~ῐ~ησοιτον", "~ῐ~ησοιτε";
    future_opt_act_3: "~ῐ~ησοι", "~ῐ~ησοιτην", "~ῐ~ησοιεν";
    future_opt_mid_1: "~ῐ~ησοιμην", _, "~ῐ~ησοιμεθᾰ";
    future_opt_mid_2: "~ῐ~ησοιο", "~ῐ~ησοισθον", "~ῐ~ησοισθε";
    future_opt_mid_3: "~ῐ~ησοιτο", "~ῐ~ησοισθην", "~ῐ~ησοιντο";
    future_opt_pass_1: "~ῐ%ε~ησοιμην", _, "~ῐ%ε~ησοιμεθᾰ";
    future_opt_pass_2: "~ῐ%ε~ησοιο", "~ῐ%ε~ησοισθον", "~ῐ%ε~ησοισθε";
    future_opt_pass_3: "~ῐ%ε~ησοιτο", "~ῐ%ε~ησοισθην", "~ῐ%ε~ησοιντο";
    aorist_subj_act_1: "~ῐ~ηκω|~ῐ~ω", _, "~ῐ~ηκωμεν|~ῐ~ωμεν";
    aorist_subj_act_2: "~ῐ~ηκῃς|~ῐ~ῃς", "~ῐ~ηκητον|~ῐ~ητον", "~ῐ~ηκητε|~ῐ~ητε";
    aorist_subj_act_3: "~ῐ~ηκῃ|~ῐ~ῃ", _, "~ῐ~ηκωσῐ|~ῐ~ηκωσῐν|~ῐ~ωσῐ|~ῐ~ωσῐν";
    aorist_subj_mid_1: "~ῐ~ηκωμαι|~ῐ~ωμαι", _, "~ῐ~ηκωμεθᾰ|~ῐ~ωμεθᾰ";
    aorist_subj_mid_2: _, "~ῐ~ηκησθον|~ῐ~ησθον", "~ῐ~ηκησθε|~ῐ~ησθε";
    aorist_subj_mid_3: "~ῐ~ηκηται|~ῐ~ηται", _, "~ῐ~ηκωνται|~ῐ~ωνται";
    aorist_opt_act_1: "~ῐ~ηκαιμῐ|~ῐ~ειην", _, "~ῐ~ηκαιμεν|~ῐ~εῑμεν|~ῐ~ειημεν";
    aorist_opt_act_2: "~ῐ~ηκειᾰς|~ῐ~ηκαις|~ῐ~ειης", "~ῐ~ηκαιτον|~ῐ~εῑτον|~ῐ~ειητον", "~ῐ~ηκαιτε|~ῐ~εῑτε|~ῐ~ειητε";
    aorist_opt_act_3: "~ῐ~ηκειε|~ῐ~ηκειεν|~ῐ~ηκαι|~ῐ~ειη", "~ῐ~ηκαιτην|~ῐ~ειτην|~ῐ~ειητην", "~ῐ~ηκειᾰν|~ῐ~ηκαιεν|~ῐ~εῑεν|~ῐ~ειησᾰν";
    aorist_opt_mid_1: "~ῐ~ηκαιμην|~ῐ~ειμην", _, "~ῐ~ηκαιμεθᾰ|~ῐ~ειμεθᾰ";
    aorist_opt_mid_2: "~ῐ~ηκαιο|~ῐ~εῑο", "~ῐ~ηκαισθον|~ῐ~εῑσθον", "~ῐ~ηκαισθε|~ῐ~εῑσθε";
    aorist_opt_mid_3: "~ῐ~ηκαιτο|~ῐ~εῑτο|~ῐ~οῑτο", "~ῐ~ηκαισθην|~ῐ~εισθην", "~ῐ~ηκαιντο|~ῐ~εῑντο";
    aorist_imp_act_2: "~ῐ~ηκον|~ῐ~ες", "~ῐ~ηκᾰτον|~ῐ~ετον", "~ῐ~ηκᾰτε|~ῐ~ετε";
    aorist_imp_act_3: "~ῐ~ηκᾰτω|~ῐ~ετω", "~ῐ~ηκᾰτων|~ῐ~ετων", "~ῐ~ηκᾰντων|~ῐ~εντων";
    aorist_imp_mid_2: "~ῐ~ηκαι|~ῐ~ου", "~ῐ~ηκᾰσθον|~ῐ~εσθον", "~ῐ~ηκᾰσθε|~ῐ~εσθε";
    aorist_imp_mid_3: "~ῐ~ηκᾰσθω|~ῐ~εσθω", "~ῐ~ηκᾰσθων|~ῐ~εσθων", _;
    perfect_ind_act_1: "~ῐ%ε~εικᾰ", _, "~ῐ%ε~εικᾰμεν";
    perfect_ind_act_2: "~ῐ%ε~εικᾰς", "~ῐ%ε~εικᾰτον", "~ῐ%ε~εικᾰτε";
    perfect_ind_act_3: "~ῐ%ε~εικε|~ῐ%ε~εικεν", _, "~ῐ%ε~εικᾱσῐ|~ῐ%ε~εικᾱσῐν";
    perfect_ind_mid_1: "~ῐ%ε~ειμαι", _, "~ῐ%ε~ειμεθᾰ";
    perfect_ind_mid_2: "~ῐ%ε~εισαι", "~ῐ%ε~εισθον", "~ῐ%ε~εισθε";
    perfect_ind_mid_3: "~ῐ%ε~ειται", _, "~ῐ%ε~εινται";
    perfect_subj_act_1: "~ῐ%ε~εικω", _, "~ῐ%ε~εικωμεν";
    perfect_subj_act_2: "~ῐ%ε~εικῃς", "~ῐ%ε~εικητον", "~ῐ%ε~εικητε";
    perfect_subj_act_3: "~ῐ%ε~εικῃ", _, "~ῐ%ε~εικωσῐ|~ῐ%ε~εικωσῐν";
    perfect_opt_act_1: "~ῐ%ε~εικοιμῐ|~ῐ%ε~εικοιην", _, "~ῐ%ε~εικοιμεν";
    perfect_opt_act_2: "~ῐ%ε~εικοις|~ῐ%ε~εικοιης", "~ῐ%ε~εικοιτον", "~ῐ%ε~εικοιτε";
    perfect_opt_act_3: "~ῐ%ε~εικοι|~ῐ%ε~εικοιη", "~ῐ%ε~εικοιτην", "~ῐ%ε~εικοιεν";
    perfect_imp_act_2: "~ῐ%ε~εικε", "~ῐ%ε~εικετον", "~ῐ%ε~εικετε";
    perfect_imp_act_3: "~ῐ%ε~εικετω", "~ῐ%ε~εικετων", "~ῐ%ε~εικοντων";
    perfect_imp_mid_2: "~ῐ%ε~εισο", _, _;
    perfect_imp_mid_3: "~ῐ%ε~εισθω", "~ῐ%ε~εισθων", _;
    infinitive_pres: "~ῐ%ῐ~εναι", "~ῐ%ῐ~εσθαι", _;
    infinitive_fut: "~ῐ~ησειν", "~ῐ~ησεσθαι", "~ῐ%ε~ησεσθαι";
    infinitive_aor: "~ῐ~ηκαι|~ῐ~εῑναι", "~ῐ~ηκᾰσθαι|~ῐ~εσθαι", _;
    infinitive_perf: "~ῐ%ε~εικεναι", "~ῐ%ε~εῑσθαι", _;
    participle_pres: "~ῐ%ῐ~εις", "~ῐ%ῐ~εμενος", _;
    participle_fut: "~ῐ~ησων", "~ῐ~ησομενος", "~ῐ%ε~ησομενος";
    participle_aor: "~ῐ~ηκᾱς|~ῐ~εις", "~ῐ~ηκᾰμενος|~ῐ~εμενος", _;
    participle_perf: "~ῐ%ε~εικως", "~ῐ%ε~ειμενος", _;
    ),
    // Μνημονευω: 3 words (μνημονευω, στρᾰτευω, στρᾰτοπεδευω). Like Βουλευω, except in 65 cells: future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, aorist_opt_pass_3 du, ….
    conjugation!("μνημονευω", 3, "~υω", like Βουλευω;
    future_ind_pass_2: "~υθησῃ", _, _;
    aorist_subj_pass_3: _, _, "~υθωσῐ";
    aorist_opt_pass_1: _, _, "~υθεῑμεν";
    aorist_opt_pass_2: _, "~υθεῑτον", "~υθεῑτε";
    aorist_opt_pass_3: _, "~υθειτην", "~υθεῑεν";
    perfect_ind_act_1: "ἐ~υκᾰ", _, "ἐ~υκᾰμεν";
    perfect_ind_act_2: "ἐ~υκᾰς", "ἐ~υκᾰτον", "ἐ~υκᾰτε";
    perfect_ind_act_3: "ἐ~υκε|ἐ~υκεν", _, "ἐ~υκᾱσῐ|ἐ~υκᾱσῐν";
    perfect_ind_mid_1: "ἐ~υμαι", _, "ἐ~υμεθᾰ";
    perfect_ind_mid_2: "ἐ~υσαι", "ἐ~υσθον", "ἐ~υσθε";
    perfect_ind_mid_3: "ἐ~υται", _, "ἐ~υνται";
    perfect_subj_act_1: "ἐ~υκω", _, "ἐ~υκωμεν";
    perfect_subj_act_2: "ἐ~υκῃς", "ἐ~υκητον", "ἐ~υκητε";
    perfect_subj_act_3: "ἐ~υκῃ", _, "ἐ~υκωσῐ|ἐ~υκωσῐν";
    perfect_opt_act_1: "ἐ~υκοιμῐ|ἐ~υκοιην", _, "ἐ~υκοιμεν";
    perfect_opt_act_2: "ἐ~υκοις|ἐ~υκοιης", "ἐ~υκοιτον", "ἐ~υκοιτε";
    perfect_opt_act_3: "ἐ~υκοι|ἐ~υκοιη", "ἐ~υκοιτην", "ἐ~υκοιεν";
    perfect_imp_act_2: "ἐ~υκε", "ἐ~υκετον", "ἐ~υκετε";
    perfect_imp_act_3: "ἐ~υκετω", "ἐ~υκετων", "ἐ~υκοντων";
    perfect_imp_mid_2: "ἐ~υσο", _, _;
    perfect_imp_mid_3: "ἐ~υσθω", "ἐ~υσθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_perf: "ἐ~υκεναι", "ἐ~ῡσθαι", _;
    participle_perf: "ἐ~υκως", "ἐ~υμενος", _;
    ),
    // Ἐπαινεω: 3 words (ἐπαινεω, καταπαιδεραστεω, πᾰραινεω). Like Κᾰταφρονεω, except in 192 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, opt_act_1 sg, opt_act_1 pl, opt_act_2 sg, ….
    conjugation!("ἐπαινεω", 3, "~α~εω", like Κᾰταφρονεω;
    stem aorist_ind: "~ε~εσ- like ἐλπω";
    stem aorist_ind_pass: "~ε~εθη- like ἐλπω";
    stem pluperfect_act: "~ε%ε~ηκε- like ἀνῐστημῐ";
    ind_act_3: _, _, "~α~οῡσῐ";
    ind_mid_2: "~α~εῑ", _, _;
    subj_act_3: _, _, "~α~ωσῐ";
    opt_act_1: "~α~οιην", _, "~α~οῑμεν";
    opt_act_2: "~α~οιης", "~α~οῑτον", "~α~οῑτε";
    opt_act_3: "~α~οιη", "~α~οιτην", "~α~οῑεν";
    imperfect_act_1: "~ε~ουν", _, "~ε~οῡμεν";
    imperfect_act_2: "~ε~εις", "~ε~εῑτον", "~ε~εῑτε";
    imperfect_act_3: "~ε~ει", "~ε~ειτην", "~ε~ουν";
    imperfect_mid_1: "~ε~ουμην", _, "~ε~ουμεθᾰ";
    imperfect_mid_2: "~ε~οῡ", "~ε~εῑσθον", "~ε~εῑσθε";
    imperfect_mid_3: "~ε~εῑτο", "~ε~εισθην", "~ε~οῡντο";
    future_ind_act_1: "~α~εσω", _, "~α~εσομεν";
    future_ind_act_2: "~α~εσεις", "~α~εσετον", "~α~εσετε";
    future_ind_act_3: "~α~εσει", _, "~α~εσουσῐ";
    future_ind_mid_1: "~α~εσομαι", _, "~α~εσομεθᾰ";
    future_ind_mid_2: "~α~εσῃ", "~α~εσεσθον", "~α~εσεσθε";
    future_ind_mid_3: "~α~εσεται", _, "~α~εσονται";
    future_ind_pass_1: "~α~εθησησομαι", _, "~α~εθησησομεθᾰ";
    future_ind_pass_2: "~α~εθησησῃ", "~α~εθησησεσθον", "~α~εθησησεσθε";
    future_ind_pass_3: "~α~εθησησεται", _, "~α~εθησησονται";
    future_opt_act_1: "~α~εσοιμῐ", _, "~α~εσοιμεν";
    future_opt_act_2: "~α~εσοις", "~α~εσοιτον", "~α~εσοιτε";
    future_opt_act_3: "~α~εσοι", "~α~εσοιτην", "~α~εσοιεν";
    future_opt_mid_1: "~α~εσοιμην", _, "~α~εσοιμεθᾰ";
    future_opt_mid_2: "~α~εσοιο", "~α~εσοισθον", "~α~εσοισθε";
    future_opt_mid_3: "~α~εσοιτο", "~α~εσοισθην", "~α~εσοιντο";
    future_opt_pass_1: "~α~εθησησοιμην", _, "~α~εθησησοιμεθᾰ";
    future_opt_pass_2: "~α~εθησησοιο", "~α~εθησησοισθον", "~α~εθησησοισθε";
    future_opt_pass_3: "~α~εθησησοιτο", "~α~εθησησοισθην", "~α~εθησησοιντο";
    aorist_subj_act_1: "~α~εσω", _, "~α~εσωμεν";
    aorist_subj_act_2: "~α~εσῃς", "~α~εσητον", "~α~εσητε";
    aorist_subj_act_3: "~α~εσῃ", _, "~α~εσωσῐ";
    aorist_subj_mid_1: "~α~εσωμαι", _, "~α~εσωμεθᾰ";
    aorist_subj_mid_2: _, "~α~εσησθον", "~α~εσησθε";
    aorist_subj_mid_3: "~α~εσηται", _, "~α~εσωνται";
    aorist_subj_pass_1: "~α~εθω", _, "~α~εθωμεν";
    aorist_subj_pass_2: "~α~εθῃς", "~α~εθητον", "~α~εθητε";
    aorist_subj_pass_3: "~α~εθῃ", _, "~α~εθωσῐ";
    aorist_opt_act_1: "~α~εσαιμῐ", _, "~α~εσαιμεν";
    aorist_opt_act_2: "~α~εσειᾰς", "~α~εσαιτον", "~α~εσαιτε";
    aorist_opt_act_3: "~α~εσειε", "~α~εσαιτην", "~α~εσειᾰν";
    aorist_opt_mid_1: "~α~εσαιμην", _, "~α~εσαιμεθᾰ";
    aorist_opt_mid_2: "~α~εσαιο", "~α~εσαισθον", "~α~εσαισθε";
    aorist_opt_mid_3: "~α~εσαιτο", "~α~εσαισθην", "~α~εσαιντο";
    aorist_opt_pass_1: "~α~εθειην", _, "~α~εθεῑμεν";
    aorist_opt_pass_2: "~α~εθειης", "~α~εθεῑτον", "~α~εθεῑτε";
    aorist_opt_pass_3: "~α~εθειη", "~α~εθειτην", "~α~εθεῑεν";
    aorist_imp_act_2: "~α~εσον", "~α~εσᾰτον", "~α~εσᾰτε";
    aorist_imp_act_3: "~α~εσᾰτω", "~α~εσᾰτων", "~α~εσᾰντων";
    aorist_imp_mid_2: "~α~εσαι", "~α~εσᾰσθον", "~α~εσᾰσθε";
    aorist_imp_mid_3: "~α~εσᾰσθω", "~α~εσᾰσθων", _;
    aorist_imp_pass_2: "~α~εθητῐ", _, _;
    aorist_imp_pass_3: "~α~εθητω", "~α~εθητων", "~α~εθεντων";
    perfect_ind_act_1: "~α%ε~ηκᾰ", _, "~α%ε~ηκᾰμεν";
    perfect_ind_act_2: "~α%ε~ηκᾰς", "~α%ε~ηκᾰτον", "~α%ε~ηκᾰτε";
    perfect_ind_act_3: "~α%ε~ηκε", _, "~α%ε~ηκᾱσῐ";
    perfect_ind_mid_1: "~α%ε~ημαι", _, "~α%ε~ημεθᾰ";
    perfect_ind_mid_2: "~α%ε~ησαι", "~α%ε~ησθον", "~α%ε~ησθε";
    perfect_ind_mid_3: "~α%ε~ηται", _, "~α%ε~ηνται";
    perfect_subj_act_1: "~α%ε~ηκω", _, "~α%ε~ηκωμεν";
    perfect_subj_act_2: "~α%ε~ηκῃς", "~α%ε~ηκητον", "~α%ε~ηκητε";
    perfect_subj_act_3: "~α%ε~ηκῃ", _, "~α%ε~ηκωσῐ";
    perfect_opt_act_1: "~α%ε~ηκοιμῐ", _, "~α%ε~ηκοιμεν";
    perfect_opt_act_2: "~α%ε~ηκοις", "~α%ε~ηκοιτον", "~α%ε~ηκοιτε";
    perfect_opt_act_3: "~α%ε~ηκοι", "~α%ε~ηκοιτην", "~α%ε~ηκοιεν";
    perfect_imp_act_2: _, "~α%ε~ηκετον", "~α%ε~ηκετε";
    perfect_imp_act_3: "~α%ε~ηκετω", "~α%ε~ηκετων", "~α%ε~ηκοντων";
    perfect_imp_mid_2: "~α%ε~ησο", _, _;
    perfect_imp_mid_3: "~α%ε~ησθω", "~α%ε~ησθων", _;
    pluperfect_mid_1: "~ε%ε~ημην", _, "~ε%ε~ημεθᾰ";
    pluperfect_mid_2: "~ε%ε~ησο", "~ε%ε~ησθον", "~ε%ε~ησθε";
    pluperfect_mid_3: "~ε%ε~ητο", "~ε%ε~ησθην", "~ε%ε~ηντο";
    infinitive_fut: "~α~εσειν", "~α~εσεσθαι", "~α~εθησησεσθαι";
    infinitive_aor: "~α~εσαι", "~α~εσᾰσθαι", "~α~εθηναι";
    infinitive_perf: "~α%ε~ηκεναι", "~α%ε~ησθαι", _;
    participle_fut: "~α~εσων", "~α~εσομενος", "~α~εθησησομενος";
    participle_aor: "~α~εσᾱς", "~α~εσᾰμενος", "~α~εθεις";
    participle_perf: "~α%ε~ηκως", "~α%ε~ημενος", _;
    ),
    // Λιστραω: 3 words (μεριμναω, κῠβισταω, λιστραω). Like Ἐπῐτῐθημῐ, except in 174 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("λιστραω", 3, "~ι~αω", like Ἐπῐτῐθημῐ as PERFECT_AS_FUTURE;
    stem aorist_ind: "~ῐ~ησ- like ἐχω";
    ind_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    ind_act_2: "~ῐ~ᾳς", "~ῐ~ᾱτον", "~ῐ~ᾱτε";
    ind_act_3: "~ῐ~ᾳ", _, "~ῐ~ωσῐ|~ῐ~ωσῐν";
    ind_mid_1: "~ῐ~ωμαι", _, "~ῐ~ωμεθᾰ";
    ind_mid_2: "~ῐ~ᾳ", "~ῐ~ᾱσθον", "~ῐ~ᾱσθε";
    ind_mid_3: "~ῐ~ᾱται", _, "~ῐ~ωνται";
    subj_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    subj_act_2: "~ῐ~ᾳς", "~ῐ~ᾱτον", "~ῐ~ᾱτε";
    subj_act_3: "~ῐ~ᾳ", _, "~ῐ~ωσῐ|~ῐ~ωσῐν";
    subj_mid_1: "~ῐ~ωμαι", _, "~ῐ~ωμεθᾰ";
    subj_mid_2: _, "~ῐ~ᾱσθον", "~ῐ~ᾱσθε";
    subj_mid_3: "~ῐ~ᾱται", _, "~ῐ~ωνται";
    opt_act_1: "~ῐ~ῳην|~ῐ~ῳμῐ", _, "~ῐ~ῳμεν|~ῐ~ῳημεν";
    opt_act_2: "~ῐ~ῳης|~ῐ~ῳς", "~ῐ~ῳτον|~ῐ~ῳητον", "~ῐ~ῳτε|~ῐ~ῳητε";
    opt_act_3: "~ῐ~ῳη|~ῐ~ῳ", "~ῐ~ῳτην|~ῐ~ῳητην", "~ῐ~ῳεν|~ῐ~ῳησᾰν";
    opt_mid_1: "~ῐ~ῳμην", _, "~ῐ~ῳμεθᾰ";
    opt_mid_2: "~ῐ~ῳο", "~ῐ~ῳσθον", "~ῐ~ῳσθε";
    opt_mid_3: "~ῐ~ῳτο", "~ῐ~ῳσθην", "~ῐ~ῳντο";
    imp_act_2: "~ῐ~ᾱ", _, _;
    imp_act_3: "~ῐ~ᾱτω", "~ῐ~ᾱτων", "~ῐ~ωντων";
    imp_mid_2: "~ῐ~ω", _, _;
    imp_mid_3: "~ῐ~ᾱσθω", "~ῐ~ᾱσθων", _;
    imperfect_act_1: "ἐ~ῐ~ων", _, "ἐ~ῐ~ωμεν";
    imperfect_act_2: "ἐ~ῐ~ᾱς", "ἐ~ῐ~ᾱτον", "ἐ~ῐ~ᾱτε";
    imperfect_act_3: "ἐ~ῐ~ᾱ", "ἐ~ῐ~ᾱτην", "ἐ~ῐ~ων";
    imperfect_mid_1: "ἐ~ῐ~ωμην", _, "ἐ~ῐ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ῐ~ω", "ἐ~ῐ~ᾱσθον", "ἐ~ῐ~ᾱσθε";
    imperfect_mid_3: "ἐ~ῐ~ᾱτο", "ἐ~ῐ~ᾱσθην", "ἐ~ῐ~ωντο";
    future_ind_act_3: _, _, "~ῐ~ησουσῐ";
    future_ind_mid_2: "~ῐ~ησῃ", _, _;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_subj_act_1: "~ῐ~ησω", _, "~ῐ~ησωμεν";
    aorist_subj_act_2: "~ῐ~ησῃς", "~ῐ~ησητον", "~ῐ~ησητε";
    aorist_subj_act_3: "~ῐ~ησῃ", _, "~ῐ~ησωσῐ";
    aorist_subj_mid_1: "~ῐ~ησωμαι", _, "~ῐ~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~ῐ~ησησθον", "~ῐ~ησησθε";
    aorist_subj_mid_3: "~ῐ~ησηται", _, "~ῐ~ησωνται";
    aorist_opt_act_1: "~ῐ~ησαιμῐ", _, "~ῐ~ησαιμεν";
    aorist_opt_act_2: "~ῐ~ησειᾰς", "~ῐ~ησαιτον", "~ῐ~ησαιτε";
    aorist_opt_act_3: "~ῐ~ησειε", "~ῐ~ησαιτην", "~ῐ~ησειᾰν";
    aorist_opt_mid_1: "~ῐ~ησαιμην", _, "~ῐ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ῐ~ησαιο", "~ῐ~ησαισθον", "~ῐ~ησαισθε";
    aorist_opt_mid_3: "~ῐ~ησαιτο", "~ῐ~ησαισθην", "~ῐ~ησαιντο";
    aorist_imp_act_2: "~ῐ~ησον", "~ῐ~ησᾰτον", "~ῐ~ησᾰτε";
    aorist_imp_act_3: "~ῐ~ησᾰτω", "~ῐ~ησᾰτων", "~ῐ~ησᾰντων";
    aorist_imp_mid_2: "~ῐ~ησαι", "~ῐ~ησᾰσθον", "~ῐ~ησᾰσθε";
    aorist_imp_mid_3: "~ῐ~ησᾰσθω", "~ῐ~ησᾰσθων", _;
    perfect_ind_act_1: "%ε~ῐ~ηκᾰ", _, "%ε~ῐ~ηκᾰμεν";
    perfect_ind_act_2: "%ε~ῐ~ηκᾰς", "%ε~ῐ~ηκᾰτον", "%ε~ῐ~ηκᾰτε";
    perfect_ind_act_3: "%ε~ῐ~ηκε", _, "%ε~ῐ~ηκᾱσῐ";
    perfect_ind_mid_1: "%ε~ῐ~ημαι", _, "%ε~ῐ~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ῐ~ησαι", "%ε~ῐ~ησθον", "%ε~ῐ~ησθε";
    perfect_ind_mid_3: "%ε~ῐ~ηται", _, "%ε~ῐ~ηνται";
    perfect_subj_act_1: "%ε~ῐ~ηκω", _, "%ε~ῐ~ηκωμεν";
    perfect_subj_act_2: "%ε~ῐ~ηκῃς", "%ε~ῐ~ηκητον", "%ε~ῐ~ηκητε";
    perfect_subj_act_3: "%ε~ῐ~ηκῃ", _, "%ε~ῐ~ηκωσῐ";
    perfect_opt_act_1: "%ε~ῐ~ηκοιμῐ", _, "%ε~ῐ~ηκοιμεν";
    perfect_opt_act_2: "%ε~ῐ~ηκοις", "%ε~ῐ~ηκοιτον", "%ε~ῐ~ηκοιτε";
    perfect_opt_act_3: "%ε~ῐ~ηκοι", "%ε~ῐ~ηκοιτην", "%ε~ῐ~ηκοιεν";
    perfect_imp_act_2: _, "%ε~ῐ~ηκετον", "%ε~ῐ~ηκετε";
    perfect_imp_act_3: "%ε~ῐ~ηκετω", "%ε~ῐ~ηκετων", "%ε~ῐ~ηκοντων";
    perfect_imp_mid_2: "%ε~ῐ~ησο", _, _;
    perfect_imp_mid_3: "%ε~ῐ~ησθω", "%ε~ῐ~ησθων", _;
    infinitive_pres: "~ῐ~ᾱν", "~ῐ~ᾱσθαι", _;
    infinitive_fut: _, _, derived;
    infinitive_aor: "~ῐ~ησαι", "~ῐ~ησᾰσθαι", _;
    infinitive_perf: "%ε~ῐ~ηκεναι", "%ε~ῐ~ησθαι", _;
    participle_pres: "~ῐ~ων", "~ῐ~ωμενος", _;
    participle_fut: _, _, derived;
    participle_aor: "~ῐ~ησᾱς", "~ῐ~ησᾰμενος", _;
    participle_perf: "%ε~ῐ~ηκως", "%ε~ῐ~ημενος", _;
    ),
    // Φλῡω: 3 words (πτῡω, μῡω, φλῡω). Like Λλῡω, except in 152 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("φλῡω", 3, "~ῡω", like Λλῡω;
    stem aorist_ind: "~ῠσ- like ποιεω";
    stem pluperfect_act: "~εμῠκε- like κᾰλεω";
    future_ind_act_1: "~ῠσω", _, "~ῠσομεν";
    future_ind_act_2: "~ῠσεις", "~ῠσετον", "~ῠσετε";
    future_ind_act_3: "~ῠσει", _, "~ῠσουσῐ|~ῠσουσῐν";
    future_ind_mid_1: "~ῠσομαι", _, "~ῠσομεθᾰ";
    future_ind_mid_2: "~ῠσῃ|~ῠσει", "~ῠσεσθον", "~ῠσεσθε";
    future_ind_mid_3: "~ῠσεται", _, "~ῠσονται";
    future_ind_pass_2: "~ῠθησῃ", _, _;
    future_opt_act_1: "~ῠσοιμῐ", _, "~ῠσοιμεν";
    future_opt_act_2: "~ῠσοις", "~ῠσοιτον", "~ῠσοιτε";
    future_opt_act_3: "~ῠσοι", "~ῠσοιτην", "~ῠσοιεν";
    future_opt_mid_1: "~ῠσοιμην", _, "~ῠσοιμεθᾰ";
    future_opt_mid_2: "~ῠσοιο", "~ῠσοισθον", "~ῠσοισθε";
    future_opt_mid_3: "~ῠσοιτο", "~ῠσοισθην", "~ῠσοιντο";
    aorist_subj_act_1: "~ῠσω", _, "~ῠσωμεν";
    aorist_subj_act_2: "~ῠσῃς", "~ῠσητον", "~ῠσητε";
    aorist_subj_act_3: "~ῠσῃ", _, "~ῠσωσῐ|~ῠσωσῐν";
    aorist_subj_mid_1: "~ῠσωμαι", _, "~ῠσωμεθᾰ";
    aorist_subj_mid_2: _, "~ῠσησθον", "~ῠσησθε";
    aorist_subj_mid_3: "~ῠσηται", _, "~ῠσωνται";
    aorist_subj_pass_3: _, _, "~ῠθωσῐ";
    aorist_opt_act_1: "~ῠσαιμῐ", _, "~ῠσαιμεν";
    aorist_opt_act_2: "~ῠσειᾰς|~ῠσαις", "~ῠσαιτον", "~ῠσαιτε";
    aorist_opt_act_3: "~ῠσειε|~ῠσειεν|~ῠσαι", "~ῠσαιτην", "~ῠσειᾰν|~ῠσαιεν";
    aorist_opt_mid_1: "~ῠσαιμην", _, "~ῠσαιμεθᾰ";
    aorist_opt_mid_2: "~ῠσαιο", "~ῠσαισθον", "~ῠσαισθε";
    aorist_opt_mid_3: "~ῠσαιτο", "~ῠσαισθην", "~ῠσαιντο";
    aorist_opt_pass_1: _, _, "~ῠθεῑμεν";
    aorist_opt_pass_2: _, "~ῠθεῑτον", "~ῠθεῑτε";
    aorist_opt_pass_3: _, "~ῠθειτην", "~ῠθεῑεν";
    aorist_imp_act_2: "~ῠσον", "~ῠσᾰτον", "~ῠσᾰτε";
    aorist_imp_act_3: "~ῠσᾰτω", "~ῠσᾰτων", "~ῠσᾰντων";
    aorist_imp_mid_2: "~ῠσαι", "~ῠσᾰσθον", "~ῠσᾰσθε";
    aorist_imp_mid_3: "~ῠσᾰσθω", "~ῠσᾰσθων", _;
    perfect_ind_act_1: "~εμῡκᾰ", _, "~εμῡκᾰμεν";
    perfect_ind_act_2: "~εμῡκᾰς", "~εμῡκᾰτον", "~εμῡκᾰτε";
    perfect_ind_act_3: "~εμῡκε", _, "~εμῡκᾱσῐ";
    perfect_ind_mid_1: "~εμῡμαι", _, "~εμῡμεθᾰ";
    perfect_ind_mid_2: "~εμῡσαι", "~εμῡσθον", "~εμῡσθε";
    perfect_ind_mid_3: "~εμῡται", _, "~εμῡνται";
    perfect_subj_act_1: "~εμῡκω", _, "~εμῡκωμεν";
    perfect_subj_act_2: "~εμῡκῃς", "~εμῡκητον", "~εμῡκητε";
    perfect_subj_act_3: "~εμῡκῃ", _, "~εμῡκωσῐ";
    perfect_opt_act_1: "~εμῡκοιμῐ", _, "~εμῡκοιμεν";
    perfect_opt_act_2: "~εμῡκοις", "~εμῡκοιτον", "~εμῡκοιτε";
    perfect_opt_act_3: "~εμῡκοι", "~εμῡκοιτην", "~εμῡκοιεν";
    perfect_imp_act_2: derived, "~εμῡκετον", "~εμῡκετε";
    perfect_imp_act_3: "~εμῡκετω", "~εμῡκετων", "~εμῡκοντων";
    perfect_imp_mid_2: "~εμῡσο", _, _;
    perfect_imp_mid_3: "~εμῡσθω", "~εμῡσθων", _;
    pluperfect_mid_1: "ἐ~εμῠμην", _, "ἐ~εμῠμεθᾰ";
    pluperfect_mid_2: "ἐ~εμῠσο", "ἐ~εμῠσθον", "ἐ~εμῠσθε";
    pluperfect_mid_3: "ἐ~εμῠτο", "ἐ~εμῠσθην", "ἐ~εμῠντο";
    future_perfect_ind_mid_1: "~εμῡσομαι", _, "~εμῡσομεθᾰ";
    future_perfect_ind_mid_2: "~εμῡσῃ", "~εμῡσεσθον", "~εμῡσεσθε";
    future_perfect_ind_mid_3: "~εμῡσεται", _, "~εμῡσονται";
    future_perfect_opt_mid_1: "~εμῡσοιμην", _, "~εμῡσοιμεθᾰ";
    future_perfect_opt_mid_2: "~εμῡσοιο", "~εμῡσοισθον", "~εμῡσοισθε";
    future_perfect_opt_mid_3: "~εμῡσοιτο", "~εμῡσοισθην", "~εμῡσοιντο";
    infinitive_fut: "~ῠσειν", "~ῠσεσθαι", _;
    infinitive_aor: "~ῠσαι", "~ῠσᾰσθαι", _;
    infinitive_perf: "~εμῡκεναι", "~εμῡσθαι", _;
    infinitive_futp: _, "~εμῡσεσθαι", _;
    participle_fut: "~ῠσων", "~ῠσομενος", _;
    participle_aor: "~ῠσᾱς", "~ῠσᾰμενος", _;
    participle_perf: "~εμῡκως", "~εμῡμενος", _;
    participle_futp: _, "~εμῡσομενος", _;
    ),
    // Ἐπευχομαι: 3 words (προσευχομαι, ἀπευχομαι, ἐπευχομαι). Like Ἐρχομαι, except in 88 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἐπευχομαι", 3, "~ευχομαι", like Ἐρχομαι;
    stem imperfect: "~ηυχ- like ὑποκοριζομαι";
    stem aorist_ind: "~ηυξ- like μηκᾰομαι";
    ind_act_1: "~ευχομαι", _, _;
    ind_mid_1: "~ευχομαι", _, "~ευχομεθᾰ";
    ind_mid_2: "~ευχῃ", "~ευχεσθον", "~ευχεσθε";
    ind_mid_3: "~ευχεται", _, "~ευχονται";
    subj_mid_1: "~ευχωμαι", _, "~ευχωμεθᾰ";
    subj_mid_2: "~ευχῃ", "~ευχησθον", "~ευχησθε";
    subj_mid_3: "~ευχηται", _, "~ευχωνται";
    subj_pass_2: "~ευχῃ", _, _;
    opt_mid_1: "~ευχοιμην", _, "~ευχοιμεθᾰ";
    opt_mid_2: "~ευχοιο", "~ευχοισθον", "~ευχοισθε";
    opt_mid_3: "~ευχοιτο", "~ευχοισθην", "~ευχοιντο";
    imp_mid_2: "~ευχου", _, _;
    imp_mid_3: "~ευχεσθω", "~ευχεσθων", _;
    future_ind_mid_1: "~ευξομαι", _, "~ευξομεθᾰ";
    future_ind_mid_2: "~ευξῃ", "~ευξεσθον", "~ευξεσθε";
    future_ind_mid_3: "~ευξεται", _, "~ευξονται";
    future_opt_mid_1: "~ευξοιμην", _, "~ευξοιμεθᾰ";
    future_opt_mid_2: "~ευξοιο", "~ευξοισθον", "~ευξοισθε";
    future_opt_mid_3: "~ευξοιτο", "~ευξοισθην", "~ευξοιντο";
    aorist_subj_mid_1: "~ευξωμαι", _, "~ευξωμεθᾰ";
    aorist_subj_mid_2: "~ευξῃ", "~ευξησθον", "~ευξησθε";
    aorist_subj_mid_3: "~ευξηται", _, "~ευξωνται";
    aorist_opt_mid_1: "~ευξαιμην", _, "~ευξαιμεθᾰ";
    aorist_opt_mid_2: "~ευξαιο", "~ευξαισθον", "~ευξαισθε";
    aorist_opt_mid_3: "~ευξαιτο", "~ευξαισθην", "~ευξαιντο";
    aorist_imp_mid_2: "~ευξαι", "~ευξᾰσθον", "~ευξᾰσθε";
    aorist_imp_mid_3: "~ευξᾰσθω", "~ευξᾰσθων", _;
    perfect_ind_mid_1: "~ηῡγμαι", _, "~ηυγμεθᾰ";
    perfect_ind_mid_2: "~ηῡξαι", "~ηῡχθον", "~ηῡχθε";
    perfect_ind_mid_3: "~ηῡκται", _, "~ηυχᾰται";
    perfect_imp_mid_2: "~ηῡξο", _, _;
    perfect_imp_mid_3: "~ηυχθω", "~ηυχθων", _;
    pluperfect_mid_1: "~ηυγμην", _, "~ηυγμεθᾰ";
    pluperfect_mid_2: "~ηῡξο", "~ηῡχθον", "~ηῡχθε";
    pluperfect_mid_3: "~ηῡκτο", "~ηυχθην", "~ηυχᾰτο";
    infinitive_pres: _, "~ευχεσθαι", _;
    infinitive_fut: _, "~ευξεσθαι", _;
    infinitive_aor: _, "~ευξᾰσθαι", _;
    infinitive_perf: _, "~ηῡχθαι", _;
    participle_pres: _, "~ευχομενος", _;
    participle_fut: _, "~ευξομενος", _;
    participle_aor: _, "~ευξᾰμενος", _;
    participle_perf: _, "~ηυγμενος", _;
    ),
    // Ἐφελκω: 3 words (ἀφελκω, κᾰθελκω, ἐφελκω). Like Ξαινω, except in 228 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐφελκω", 3, "~~κω", like Ξαινω;
    stem aorist_ind: "~ι~κυσ- like εὐλογεω";
    stem aorist_ind_pass: "~ι~κυσθη- like ἐλπω";
    ind_act_1: "~~κω", _, "~~κομεν";
    ind_act_2: "~~κεις", "~~κετον", "~~κετε";
    ind_act_3: "~~κει", _, "~~κουσῐ|~~κουσῐν";
    ind_mid_1: "~~κομαι", _, "~~κομεθᾰ";
    ind_mid_2: "~~κῃ|~~κει", "~~κεσθον", "~~κεσθε";
    ind_mid_3: "~~κεται", _, "~~κονται";
    subj_act_1: "~~κω", _, "~~κωμεν";
    subj_act_2: "~~κῃς", "~~κητον", "~~κητε";
    subj_act_3: "~~κῃ", _, "~~κωσῐ|~~κωσῐν";
    subj_mid_1: "~~κωμαι", _, "~~κωμεθᾰ";
    subj_mid_2: _, "~~κησθον", "~~κησθε";
    subj_mid_3: "~~κηται", _, "~~κωνται";
    opt_act_1: "~~κοιμῐ", _, "~~κοιμεν";
    opt_act_2: "~~κοις", "~~κοιτον", "~~κοιτε";
    opt_act_3: "~~κοι", "~~κοιτην", "~~κοιεν";
    opt_mid_1: "~~κοιμην", _, "~~κοιμεθᾰ";
    opt_mid_2: "~~κοιο", "~~κοισθον", "~~κοισθε";
    opt_mid_3: "~~κοιτο", "~~κοισθην", "~~κοιντο";
    imp_act_2: "~~κε", _, _;
    imp_act_3: "~~κετω", "~~κετων", "~~κοντων";
    imp_mid_2: "~~κου", _, _;
    imp_mid_3: "~~κεσθω", "~~κεσθων", _;
    imperfect_act_1: "~ῑ~κον", _, "~ι~κομεν";
    imperfect_act_2: "~ῑ~κες", "~ι~κετον", "~ι~κετε";
    imperfect_act_3: "~ῑ~κε", "~ι~κετην", "~ῑ~κον";
    imperfect_mid_1: "~ι~κομην", _, "~ι~κομεθᾰ";
    imperfect_mid_2: "~ι~κου", "~ι~κεσθον", "~ι~κεσθε";
    imperfect_mid_3: "~ι~κετο", "~ι~κεσθην", "~ι~κοντο";
    future_ind_act_1: "~~ξω", _, "~~ξομεν";
    future_ind_act_2: "~~ξεις", "~~ξετον", "~~ξετε";
    future_ind_act_3: "~~ξει", _, "~~ξουσῐ|~~ξουσῐν";
    future_ind_mid_1: "~~ξομαι", _, "~~ξομεθᾰ";
    future_ind_mid_2: "~~ξῃ", "~~ξεσθον", "~~ξεσθε";
    future_ind_mid_3: "~~ξεται", _, "~~ξονται";
    future_ind_pass_1: "~~κυσθησομαι", _, "~~κυσθησομεθᾰ";
    future_ind_pass_2: "~~κυσθησῃ", "~~κυσθησεσθον", "~~κυσθησεσθε";
    future_ind_pass_3: "~~κυσθησεται", _, "~~κυσθησονται";
    future_opt_act_1: "~~ξοιμῐ", _, "~~ξοιμεν";
    future_opt_act_2: "~~ξοις", "~~ξοιτον", "~~ξοιτε";
    future_opt_act_3: "~~ξοι", "~~ξοιτην", "~~ξοιεν";
    future_opt_mid_1: "~~ξοιμην", _, "~~ξοιμεθᾰ";
    future_opt_mid_2: "~~ξοιο", "~~ξοισθον", "~~ξοισθε";
    future_opt_mid_3: "~~ξοιτο", "~~ξοισθην", "~~ξοιντο";
    future_opt_pass_1: "~~κυσθησοιμην", _, "~~κυσθησοιμεθᾰ";
    future_opt_pass_2: "~~κυσθησοιο", "~~κυσθησοισθον", "~~κυσθησοισθε";
    future_opt_pass_3: "~~κυσθησοιτο", "~~κυσθησοισθην", "~~κυσθησοιντο";
    aorist_subj_act_1: "~~κυσω", _, "~~κυσωμεν";
    aorist_subj_act_2: "~~κυσῃς", "~~κυσητον", "~~κυσητε";
    aorist_subj_act_3: "~~κυσῃ", _, "~~κυσωσῐ|~~κυσωσῐν";
    aorist_subj_mid_1: "~~κυσωμαι", _, "~~κυσωμεθᾰ";
    aorist_subj_mid_2: _, "~~κυσησθον", "~~κυσησθε";
    aorist_subj_mid_3: "~~κυσηται", _, "~~κυσωνται";
    aorist_subj_pass_1: "~ι~κυσθω", _, "~ι~κυσθωμεν";
    aorist_subj_pass_2: "~ι~κυσθῃς", "~ι~κυσθητον", "~ι~κυσθητε";
    aorist_subj_pass_3: "~ι~κυσθῃ", _, "~ι~κυσθωσῐ";
    aorist_opt_act_1: "~~κυσαιμῐ", _, "~~κυσαιμεν";
    aorist_opt_act_2: "~~κυσειᾰς|~~κυσαις", "~~κυσαιτον", "~~κυσαιτε";
    aorist_opt_act_3: "~~κυσειε|~~κυσειεν|~~κυσαι", "~~κυσαιτην", "~~κυσειᾰν|~~κυσαιεν";
    aorist_opt_mid_1: "~~κυσαιμην", _, "~~κυσαιμεθᾰ";
    aorist_opt_mid_2: "~~κυσαιο", "~~κυσαισθον", "~~κυσαισθε";
    aorist_opt_mid_3: "~~κυσαιτο", "~~κυσαισθην", "~~κυσαιντο";
    aorist_opt_pass_1: "~ι~κυσθειην", _, "~ι~κυσθεῑμεν";
    aorist_opt_pass_2: "~ι~κυσθειης", "~ι~κυσθεῑτον", "~ι~κυσθεῑτε";
    aorist_opt_pass_3: "~ι~κυσθειη", "~ι~κυσθειτην", "~ι~κυσθεῑεν";
    aorist_imp_act_2: "~~κυσον", "~~κυσᾰτον", "~~κυσᾰτε";
    aorist_imp_act_3: "~~κυσᾰτω", "~~κυσᾰτων", "~~κυσᾰντων";
    aorist_imp_mid_2: "~~κυσαι", "~~κυσᾰσθον", "~~κυσᾰσθε";
    aorist_imp_mid_3: "~~κυσᾰσθω", "~~κυσᾰσθων", _;
    aorist_imp_pass_2: "~ι~κυσθητῐ", _, _;
    aorist_imp_pass_3: "~ι~κυσθητω", "~ι~κυσθητων", "~ι~κυσθεντων";
    perfect_ind_act_1: "~ι~κῠκᾰ", _, "~ι~κῠκᾰμεν";
    perfect_ind_act_2: "~ι~κῠκᾰς", "~ι~κῠκᾰτον", "~ι~κῠκᾰτε";
    perfect_ind_act_3: "~ι~κῠκε|~ι~κῠκεν", _, "~ι~κῠκᾱσῐ|~ι~κῠκᾱσῐν";
    perfect_ind_mid_1: "~ι~κῠμαι", _, "~ι~κῠμεθᾰ";
    perfect_ind_mid_2: "~ι~κῠσαι", "~ι~κῠσθον", "~ι~κῠσθε";
    perfect_ind_mid_3: "~ι~κῠται", _, "~ι~κῠνται";
    perfect_subj_act_1: "~ι~κῠκω", _, "~ι~κῠκωμεν";
    perfect_subj_act_2: "~ι~κῠκῃς", "~ι~κῠκητον", "~ι~κῠκητε";
    perfect_subj_act_3: "~ι~κῠκῃ", _, "~ι~κῠκωσῐ|~ι~κῠκωσῐν";
    perfect_opt_act_1: "~ι~κῠκοιμῐ|~ι~κῠκοιην", _, "~ι~κῠκοιμεν";
    perfect_opt_act_2: "~ι~κῠκοις|~ι~κῠκοιης", "~ι~κῠκοιτον", "~ι~κῠκοιτε";
    perfect_opt_act_3: "~ι~κῠκοι|~ι~κῠκοιη", "~ι~κῠκοιτην", "~ι~κῠκοιεν";
    perfect_imp_act_2: "~ι~κῠκε", "~ι~κῠκετον", "~ι~κῠκετε";
    perfect_imp_act_3: "~ι~κῠκετω", "~ι~κῠκετων", "~ι~κῠκοντων";
    perfect_imp_mid_2: "~ι~κῠσο", _, _;
    perfect_imp_mid_3: "~ι~κῠσθω", "~ι~κῠσθων", _;
    infinitive_pres: "~~κειν", "~~κεσθαι", _;
    infinitive_fut: "~~ξειν", "~~ξεσθαι", "~~κυσθησεσθαι";
    infinitive_aor: "~~κῡσαι", "~~κυσᾰσθαι", "~ι~κυσθηναι";
    infinitive_perf: "~ι~κῠκεναι", "~ι~κῠσθαι", _;
    participle_pres: "~~κων", "~~κομενος", _;
    participle_fut: "~~ξων", "~~ξομενος", "~~κυσθησομενος";
    participle_aor: "~~κυσᾱς", "~~κυσᾰμενος", "~ι~κυσθεις";
    participle_perf: "~ι~κῠκως", "~ι~κῠμενος", _;
    ),
    // Τρῑβω: 3 words (τρῑβω, θλῑβω, φλῑβω). Like Ἀμειβω, except in 191 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("τρῑβω", 3, "~ῑβω", like Ἀμειβω;
    stem aorist_ind_pass: "~ιφθη- like ἐχω";
    stem pluperfect_act: "~ῐφ- like χρημᾰτῐζω";
    ind_act_1: "~ῑβω", _, "~ῑβομεν";
    ind_act_2: "~ῑβεις", "~ῑβετον", "~ῑβετε";
    ind_act_3: "~ῑβει", _, "~ῑβουσῐ|~ῑβουσῐν";
    ind_mid_1: "~ῑβομαι", _, "~ῑβομεθᾰ";
    ind_mid_2: "~ῑβῃ|~ῑβει", "~ῑβεσθον", "~ῑβεσθε";
    ind_mid_3: "~ῑβεται", _, "~ῑβονται";
    subj_act_1: "~ῑβω", _, "~ῑβωμεν";
    subj_act_2: "~ῑβῃς", "~ῑβητον", "~ῑβητε";
    subj_act_3: "~ῑβῃ", _, "~ῑβωσῐ|~ῑβωσῐν";
    subj_mid_1: "~ῑβωμαι", _, "~ῑβωμεθᾰ";
    subj_mid_2: _, "~ῑβησθον", "~ῑβησθε";
    subj_mid_3: "~ῑβηται", _, "~ῑβωνται";
    opt_act_1: "~ῑβοιμῐ", _, "~ῑβοιμεν";
    opt_act_2: "~ῑβοις", "~ῑβοιτον", "~ῑβοιτε";
    opt_act_3: "~ῑβοι", "~ῑβοιτην", "~ῑβοιεν";
    opt_mid_1: "~ῑβοιμην", _, "~ῑβοιμεθᾰ";
    opt_mid_2: "~ῑβοιο", "~ῑβοισθον", "~ῑβοισθε";
    opt_mid_3: "~ῑβοιτο", "~ῑβοισθην", "~ῑβοιντο";
    imp_act_3: "~ῑβετω", "~ῑβετων", "~ῑβοντων";
    imp_mid_2: "~ῑβου", _, _;
    imp_mid_3: "~ῑβεσθω", "~ῑβεσθων", _;
    imperfect_act_1: "ἐ~ῑβον", _, "ἐ~ῑβομεν";
    imperfect_act_2: "ἐ~ῑβες", "ἐ~ῑβετον", "ἐ~ῑβετε";
    imperfect_act_3: "ἐ~ῑβε|ἐ~ῑβεν", "ἐ~ῑβετην", "ἐ~ῑβον";
    imperfect_mid_1: "ἐ~ῑβομην", _, "ἐ~ῑβομεθᾰ";
    imperfect_mid_2: "ἐ~ῑβου", "ἐ~ῑβεσθον", "ἐ~ῑβεσθε";
    imperfect_mid_3: "ἐ~ῑβετο", "ἐ~ῑβεσθην", "ἐ~ῑβοντο";
    future_ind_act_1: "~ιψω", _, "~ιψομεν";
    future_ind_act_2: "~ιψεις", "~ιψετον", "~ιψετε";
    future_ind_act_3: "~ιψει", _, "~ιψουσῐ|~ιψουσῐν";
    future_ind_mid_1: "~ιψομαι", _, "~ιψομεθᾰ";
    future_ind_mid_2: "~ιψῃ|~ιψει", "~ιψεσθον", "~ιψεσθε";
    future_ind_mid_3: "~ιψεται", _, "~ιψονται";
    future_ind_pass_1: "~ῐβησομαι", _, "~ῐβησομεθᾰ";
    future_ind_pass_2: "~ῐβησῃ|~ῐβησει", "~ῐβησεσθον", "~ῐβησεσθε";
    future_ind_pass_3: "~ῐβησεται", _, "~ῐβησονται";
    future_opt_act_1: "~ιψοιμῐ", _, "~ιψοιμεν";
    future_opt_act_2: "~ιψοις", "~ιψοιτον", "~ιψοιτε";
    future_opt_act_3: "~ιψοι", "~ιψοιτην", "~ιψοιεν";
    future_opt_mid_1: "~ιψοιμην", _, "~ιψοιμεθᾰ";
    future_opt_mid_2: "~ιψοιο", "~ιψοισθον", "~ιψοισθε";
    future_opt_mid_3: "~ιψοιτο", "~ιψοισθην", "~ιψοιντο";
    future_opt_pass_1: "~ῐβησοιμην", _, "~ῐβησοιμεθᾰ";
    future_opt_pass_2: "~ῐβησοιο", "~ῐβησοισθον", "~ῐβησοισθε";
    future_opt_pass_3: "~ῐβησοιτο", "~ῐβησοισθην", "~ῐβησοιντο";
    aorist_subj_pass_1: "~ιφθω", _, "~ιφθωμεν";
    aorist_subj_pass_2: "~ιφθῃς", "~ιφθητον", "~ιφθητε";
    aorist_subj_pass_3: "~ιφθῃ", _, "~ιφθωσῐ";
    aorist_opt_pass_1: "~ιφθειην", _, "~ιφθεῑμεν";
    aorist_opt_pass_2: "~ιφθειης", "~ιφθεῑτον", "~ιφθεῑτε";
    aorist_opt_pass_3: "~ιφθειη", "~ιφθειτην", "~ιφθεῑεν";
    aorist_imp_pass_2: "~ιφθητῐ", _, _;
    aorist_imp_pass_3: "~ιφθητω", "~ιφθητων", "~ιφθεντων";
    perfect_ind_act_1: "%ε~ῐφᾰ", _, "%ε~ῐφᾰμεν";
    perfect_ind_act_2: "%ε~ῐφᾰς", "%ε~ῐφᾰτον", "%ε~ῐφᾰτε";
    perfect_ind_act_3: "%ε~ῐφε|%ε~ῐφεν", _, "%ε~ῐφᾱσῐ|%ε~ῐφᾱσῐν";
    perfect_ind_mid_1: "%ε~ῑμμαι", _, "%ε~ῑμμεθᾰ";
    perfect_ind_mid_2: "%ε~ῑψαι", "%ε~ῑφθον", "%ε~ῑφθε";
    perfect_ind_mid_3: "%ε~ῑπται", _, "%ε~ῑφᾰται";
    perfect_subj_act_1: "%ε~ῐφω", _, "%ε~ῐφωμεν";
    perfect_subj_act_2: "%ε~ῐφῃς", "%ε~ῐφητον", "%ε~ῐφητε";
    perfect_subj_act_3: "%ε~ῐφῃ", _, "%ε~ῐφωσῐ|%ε~ῐφωσῐν";
    perfect_opt_act_1: "%ε~ῐφοιμῐ|%ε~ῐφοιην", _, "%ε~ῐφοιμεν";
    perfect_opt_act_2: "%ε~ῐφοις|%ε~ῐφοιης", "%ε~ῐφοιτον", "%ε~ῐφοιτε";
    perfect_opt_act_3: "%ε~ῐφοι|%ε~ῐφοιη", "%ε~ῐφοιτην", "%ε~ῐφοιεν";
    perfect_imp_act_2: "%ε~ῐφε", "%ε~ῐφετον", "%ε~ῐφετε";
    perfect_imp_act_3: "%ε~ῐφετω", "%ε~ῐφετων", "%ε~ῐφοντων";
    perfect_imp_mid_2: "%ε~ῑψο", _, _;
    perfect_imp_mid_3: "%ε~ῑφθω", "%ε~ῑφθων", _;
    pluperfect_mid_1: "ἐ%ε~ῐμμην", _, "ἐ%ε~ῐμμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ῐψο", "ἐ%ε~ῐφθον", "ἐ%ε~ῐφθε";
    pluperfect_mid_3: "ἐ%ε~ῐπτο", "ἐ%ε~ῐφθην", "ἐ%ε~ῐφᾰτο";
    infinitive_pres: "~ῑβειν", "~ῑβεσθαι", _;
    infinitive_fut: "~ιψειν", "~ιψεσθαι", "~ῐβησεσθαι";
    infinitive_aor: _, _, "~ιφθηναι";
    infinitive_perf: "%ε~ῐφεναι", "%ε~ῑφθαι", _;
    participle_pres: "~ῑβων", "~ῑβομενος", _;
    participle_fut: "~ιψων", "~ιψομενος", "~ῐβησομενος";
    participle_aor: _, _, "~ιφθεις";
    participle_perf: "%ε~ῐφως", "%ε~ῑμμενος", _;
    ),
    // Ἀντεχω: 3 words (μετεχω, κᾰτεχω, ἀντεχω). Like Μῑγνῡμῐ, except in 190 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀντεχω", 3, "~τεχω", like Μῑγνῡμῐ;
    stem aorist_ind: "~τεσχ- like πᾰρεχω";
    ind_act_1: "~τεχω", _, "~τεχομεν";
    ind_act_2: "~τεχεις", "~τεχετον", "~τεχετε";
    ind_act_3: "~τεχει", _, "~τεχουσῐ|~τεχουσῐν";
    ind_mid_1: "~τεχομαι", _, "~τεχομεθᾰ";
    ind_mid_2: "~τεχῃ|~τεχει", "~τεχεσθον", "~τεχεσθε";
    ind_mid_3: "~τεχεται", _, "~τεχονται";
    subj_act_1: "~τεχω", _, "~τεχωμεν";
    subj_act_2: "~τεχῃς", "~τεχητον", "~τεχητε";
    subj_act_3: "~τεχῃ", _, "~τεχωσῐ|~τεχωσῐν";
    subj_mid_1: "~τεχωμαι", _, "~τεχωμεθᾰ";
    subj_mid_2: _, "~τεχησθον", "~τεχησθε";
    subj_mid_3: "~τεχηται", _, "~τεχωνται";
    opt_act_1: "~τεχοιμῐ", _, "~τεχοιμεν";
    opt_act_2: "~τεχοις", "~τεχοιτον", "~τεχοιτε";
    opt_act_3: "~τεχοι", "~τεχοιτην", "~τεχοιεν";
    opt_mid_1: "~τεχοιμην", _, "~τεχοιμεθᾰ";
    opt_mid_2: "~τεχοιο", "~τεχοισθον", "~τεχοισθε";
    opt_mid_3: "~τεχοιτο", "~τεχοισθην", "~τεχοιντο";
    imp_act_2: "~τεχε", _, _;
    imp_act_3: "~τεχετω", "~τεχετων", "~τεχοντων";
    imp_mid_2: "~τεχου", _, _;
    imp_mid_3: "~τεχεσθω", "~τεχεσθων", _;
    imperfect_act_1: "~τεῑχον", _, "~τειχομεν";
    imperfect_act_2: "~τεῑχες", "~τειχετον", "~τειχετε";
    imperfect_act_3: "~τεῑχε|~τεῑχεν", "~τειχετην", "~τεῑχον";
    imperfect_mid_1: "~τειχομην", _, "~τειχομεθᾰ";
    imperfect_mid_2: "~τειχου", "~τειχεσθον", "~τειχεσθε";
    imperfect_mid_3: "~τειχετο", "~τειχεσθην", "~τειχοντο";
    future_ind_act_1: "~θεξω", _, "~θεξομεν";
    future_ind_act_2: "~θεξεις", "~θεξετον", "~θεξετε";
    future_ind_act_3: "~θεξει", _, "~θεξουσῐ|~θεξουσῐν";
    future_ind_mid_1: "~θεξομαι", _, "~θεξομεθᾰ";
    future_ind_mid_2: "~θεξῃ|~θεξει", "~θεξεσθον", "~θεξεσθε";
    future_ind_mid_3: "~θεξεται", _, "~θεξονται";
    future_opt_act_1: "~θεξοιμῐ", _, "~θεξοιμεν";
    future_opt_act_2: "~θεξοις", "~θεξοιτον", "~θεξοιτε";
    future_opt_act_3: "~θεξοι", "~θεξοιτην", "~θεξοιεν";
    future_opt_mid_1: "~θεξοιμην", _, "~θεξοιμεθᾰ";
    future_opt_mid_2: "~θεξοιο", "~θεξοισθον", "~θεξοισθε";
    future_opt_mid_3: "~θεξοιτο", "~θεξοισθην", "~θεξοιντο";
    aorist_subj_act_1: "~τᾰσχω", _, "~τᾰσχωμεν";
    aorist_subj_act_2: "~τᾰσχῃς", "~τᾰσχητον", "~τᾰσχητε";
    aorist_subj_act_3: "~τᾰσχῃ", _, "~τᾰσχωσῐ";
    aorist_subj_mid_1: "~τᾰσχωμαι", _, "~τᾰσχωμεθᾰ";
    aorist_subj_mid_2: _, "~τᾰσχησθον", "~τᾰσχησθε";
    aorist_subj_mid_3: "~τᾰσχηται", _, "~τᾰσχωνται";
    aorist_opt_act_1: "~τᾰσχοιμῐ", _, "~τᾰσχοιμεν";
    aorist_opt_act_2: "~τᾰσχοις", "~τᾰσχοιτον", "~τᾰσχοιτε";
    aorist_opt_act_3: "~τᾰσχοι", "~τᾰσχοιτην", "~τᾰσχοιεν";
    aorist_opt_mid_1: "~τᾰσχοιμην", _, "~τᾰσχοιμεθᾰ";
    aorist_opt_mid_2: "~τᾰσχοιο", "~τᾰσχοισθον", "~τᾰσχοισθε";
    aorist_opt_mid_3: "~τᾰσχοιτο", "~τᾰσχοισθην", "~τᾰσχοιντο";
    aorist_imp_act_2: "~τᾰσχε", "~τᾰσχετον", "~τᾰσχετε";
    aorist_imp_act_3: "~τᾰσχετω", "~τᾰσχετων", "~τᾰσχοντων";
    aorist_imp_mid_2: "~τᾰσχοῡ", "~τᾰσχεσθον", "~τᾰσχεσθε";
    aorist_imp_mid_3: "~τᾰσχεσθω", "~τᾰσχεσθων", _;
    perfect_ind_act_1: "~τεσχηκᾰ", _, "~τεσχηκᾰμεν";
    perfect_ind_act_2: "~τεσχηκᾰς", "~τεσχηκᾰτον", "~τεσχηκᾰτε";
    perfect_ind_act_3: "~τεσχηκε|~τεσχηκεν", _, "~τεσχηκᾱσῐ|~τεσχηκᾱσῐν";
    perfect_ind_mid_1: "~τεσχημαι", _, "~τεσχημεθᾰ";
    perfect_ind_mid_2: "~τεσχησαι", "~τεσχησθον", "~τεσχησθε";
    perfect_ind_mid_3: "~τεσχηται", _, "~τεσχηνται";
    perfect_subj_act_1: "~τεσχηκω", _, "~τεσχηκωμεν";
    perfect_subj_act_2: "~τεσχηκῃς", "~τεσχηκητον", "~τεσχηκητε";
    perfect_subj_act_3: "~τεσχηκῃ", _, "~τεσχηκωσῐ|~τεσχηκωσῐν";
    perfect_opt_act_1: "~τεσχηκοιμῐ|~τεσχηκοιην", _, "~τεσχηκοιμεν";
    perfect_opt_act_2: "~τεσχηκοις|~τεσχηκοιης", "~τεσχηκοιτον", "~τεσχηκοιτε";
    perfect_opt_act_3: "~τεσχηκοι|~τεσχηκοιη", "~τεσχηκοιτην", "~τεσχηκοιεν";
    perfect_imp_act_2: "~τεσχηκε", "~τεσχηκετον", "~τεσχηκετε";
    perfect_imp_act_3: "~τεσχηκετω", "~τεσχηκετων", "~τεσχηκοντων";
    perfect_imp_mid_2: "~τεσχησο", _, _;
    perfect_imp_mid_3: "~τεσχησθω", "~τεσχησθων", _;
    infinitive_pres: "~τεχειν", "~τεχεσθαι", _;
    infinitive_fut: "~θεξειν", "~θεξεσθαι", _;
    infinitive_aor: "~τᾰσχεῑν", "~τᾰσχεσθαι", _;
    infinitive_perf: "~τεσχηκεναι", "~τεσχησθαι", _;
    participle_pres: "~τεχων", "~τεχομενος", _;
    participle_fut: "~θεξων", "~θεξομενος", _;
    participle_aor: "~τᾰσχων", "~τᾰσχομενος", _;
    participle_perf: "~τεσχηκως", "~τεσχημενος", _;
    ),
    // Ποιμαινω: 3 words (ἐπῐφαινω, ποιμαινω, χλῑαινω). Like Λιστραω, except in 181 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ποιμαινω", 3, "~~ινω", like Λιστραω as ACT_AS_PASS;
    stem aorist_ind: "~~ν- like ἐχω";
    stem aorist_ind_pass: "~~νθη- like ἐχω";
    ind_act_1: "~~ινω", _, "~~ινομεν";
    ind_act_2: "~~ινεις", "~~ινετον", "~~ινετε";
    ind_act_3: "~~ινει", _, "~~ινουσῐ";
    ind_mid_1: "~~ινομαι", _, "~~ινομεθᾰ";
    ind_mid_2: "~~ινῃ", "~~ινεσθον", "~~ινεσθε";
    ind_mid_3: "~~ινεται", _, "~~ινονται";
    subj_act_1: "~~ινω", _, "~~ινωμεν";
    subj_act_2: "~~ινῃς", "~~ινητον", "~~ινητε";
    subj_act_3: "~~ινῃ", _, "~~ινωσῐ";
    subj_mid_1: "~~ινωμαι", _, "~~ινωμεθᾰ";
    subj_mid_2: _, "~~ινησθον", "~~ινησθε";
    subj_mid_3: "~~ινηται", _, "~~ινωνται";
    opt_act_1: "~~ινοιμῐ", _, "~~ινοιμεν";
    opt_act_2: "~~ινοις", "~~ινοιτον", "~~ινοιτε";
    opt_act_3: "~~ινοι", "~~ινοιτην", "~~ινοιεν";
    opt_mid_1: "~~ινοιμην", _, "~~ινοιμεθᾰ";
    opt_mid_2: "~~ινοιο", "~~ινοισθον", "~~ινοισθε";
    opt_mid_3: "~~ινοιτο", "~~ινοισθην", "~~ινοιντο";
    imp_act_2: "~~ινε", _, _;
    imp_act_3: "~~ινετω", "~~ινετων", "~~ινοντων";
    imp_mid_2: "~~ινου", _, _;
    imp_mid_3: "~~ινεσθω", "~~ινεσθων", _;
    imperfect_act_1: "ε~~ινον", _, "ε~~ινομεν";
    imperfect_act_2: "ε~~ινες", "ε~~ινετον", "ε~~ινετε";
    imperfect_act_3: "ε~~ινε", "ε~~ινετην", "ε~~ινον";
    imperfect_mid_1: "ε~~ινομην", _, "ε~~ινομεθᾰ";
    imperfect_mid_2: "ε~~ινου", "ε~~ινεσθον", "ε~~ινεσθε";
    imperfect_mid_3: "ε~~ινετο", "ε~~ινεσθην", "ε~~ινοντο";
    future_ind_act_1: "~~νω", _, "~~νοῡμεν";
    future_ind_act_2: "~~νεῑς", "~~νεῑτον", "~~νεῑτε";
    future_ind_act_3: "~~νεῑ", _, "~~νοῡσῐ";
    future_ind_mid_1: "~~νοῡμαι", _, "~~νουμεθᾰ";
    future_ind_mid_2: "~~νεῑ", "~~νεῑσθον", "~~νεῑσθε";
    future_ind_mid_3: "~~νεῑται", _, "~~νοῡνται";
    future_opt_act_1: "~~νοιην", _, "~~νοῑμεν";
    future_opt_act_2: "~~νοιης", "~~νοῑτον", "~~νοῑτε";
    future_opt_act_3: "~~νοιη", "~~νοιτην", "~~νοῑεν";
    future_opt_mid_1: "~~νοιμην", _, "~~νοιμεθᾰ";
    future_opt_mid_2: "~~νοῑο", "~~νοῑσθον", "~~νοῑσθε";
    future_opt_mid_3: "~~νοῑτο", "~~νοισθην", "~~νοῑντο";
    aorist_subj_act_1: "~~νω", _, "~~νωμεν";
    aorist_subj_act_2: "~~νῃς", "~~νητον", "~~νητε";
    aorist_subj_act_3: "~~νῃ", _, "~~νωσῐ";
    aorist_subj_mid_1: "~~νωμαι", _, "~~νωμεθᾰ";
    aorist_subj_mid_2: _, "~~νησθον", "~~νησθε";
    aorist_subj_mid_3: "~~νηται", _, "~~νωνται";
    aorist_subj_pass_1: "~~νθω", _, "~~νθωμεν";
    aorist_subj_pass_2: "~~νθῃς", "~~νθητον", "~~νθητε";
    aorist_subj_pass_3: "~~νθῃ", _, "~~νθωσῐ";
    aorist_opt_act_1: "~~ναιμῐ", _, "~~ναιμεν";
    aorist_opt_act_2: "~~νειᾰς", "~~ναιτον", "~~ναιτε";
    aorist_opt_act_3: "~~νειε", "~~ναιτην", "~~νειᾰν";
    aorist_opt_mid_1: "~~ναιμην", _, "~~ναιμεθᾰ";
    aorist_opt_mid_2: "~~ναιο", "~~ναισθον", "~~ναισθε";
    aorist_opt_mid_3: "~~ναιτο", "~~ναισθην", "~~ναιντο";
    aorist_opt_pass_1: "~~νθειην", _, "~~νθεῑμεν";
    aorist_opt_pass_2: "~~νθειης", "~~νθεῑτον", "~~νθεῑτε";
    aorist_opt_pass_3: "~~νθειη", "~~νθειτην", "~~νθεῑεν";
    aorist_imp_act_2: "~~νον", "~~νᾰτον", "~~νᾰτε";
    aorist_imp_act_3: "~~νᾰτω", "~~νᾰτων", "~~νᾰντων";
    aorist_imp_mid_2: "~~ναι", "~~νᾰσθον", "~~νᾰσθε";
    aorist_imp_mid_3: "~~νᾰσθω", "~~νᾰσθων", _;
    aorist_imp_pass_2: "~~νθητῐ", _, _;
    aorist_imp_pass_3: "~~νθητω", "~~νθητων", "~~νθεντων";
    perfect_ind_mid_1: "%ε~~σμαι", _, "%ε~~σμεθᾰ";
    perfect_ind_mid_2: "%ε~~νσαι", "%ε~~νθον", "%ε~~νθε";
    perfect_ind_mid_3: "%ε~~νται", _, "%ε~~νᾰται";
    perfect_imp_mid_2: "%ε~~νσο", _, _;
    perfect_imp_mid_3: "%ε~~νθω", "%ε~~νθων", _;
    infinitive_pres: "~~ινειν", "~~ινεσθαι", _;
    infinitive_fut: "~~νεῑν", "~~νεῑσθαι", _;
    infinitive_aor: "~ᾱν~ι", "~~νᾰσθαι", "~~νθηναι";
    infinitive_perf: _, "%ε~ᾱνθ~ι", _;
    participle_pres: "~~ινων", "~~ινομενος", _;
    participle_fut: "~~νων", "~~νουμενος", _;
    participle_aor: "~~νᾱς", "~~νᾰμενος", "~~νθεις";
    participle_perf: _, "%ε~~σμενος", _;
    ),
    // Σῠνιστημῐ: 3 words (σῠνιστημῐ, ξῠνιστημῐ, ὀνινημῐ). Like Ἐπιτρεχω, except in 198 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("σῠνιστημῐ", 3, "~νι~ημῐ", like Ἐπιτρεχω as PRESENT_AS_FUTURE;
    stem aorist_ind: "~νε~- like σῠνιστημῐ";
    stem aorist_ind_pass: "~νε~ᾰθη- like ἐλπω";
    stem pluperfect_act: "~νει~ηκ- like εὐλογεω";
    ind_act_1: "~νι~ημῐ", _, "~νι~ᾰμεν";
    ind_act_2: "~νι~ης", "~νι~ᾰτον", "~νι~ᾰτε";
    ind_act_3: "~νι~ησῐ|~νι~ησῐν", _, "~νι~ᾱσῐ|~νι~ᾱσῐν";
    ind_mid_1: "~νι~ᾰμαι", _, "~νι~ᾰμεθᾰ";
    ind_mid_2: "~νι~ᾰσαι", "~νι~ᾰσθον", "~νι~ᾰσθε";
    ind_mid_3: "~νι~ᾰται", _, "~νι~ᾰνται";
    subj_act_1: "~νι~ω", _, "~νι~ωμεν";
    subj_act_2: "~νι~ῃς", "~νι~ητον", "~νι~ητε";
    subj_act_3: "~νι~ῃ", _, "~νι~ωσῐ|~νι~ωσῐν";
    subj_mid_1: "~νι~ωμαι", _, "~νι~ωμεθᾰ";
    subj_mid_2: _, "~νι~ησθον", "~νι~ησθε";
    subj_mid_3: "~νι~ηται", _, "~νι~ωνται";
    opt_act_1: "~νι~αιην", _, "~νι~αῑμεν|~νι~αιημεν";
    opt_act_2: "~νι~αιης", "~νι~αῑτον|~νι~αιητον", "~νι~αῑτε|~νι~αιητε";
    opt_act_3: "~νι~αιη", "~νι~αιτην|~νι~αιητην", "~νι~αῑεν|~νι~αιησᾰν";
    opt_mid_1: "~νι~αιμην", _, "~νι~αιμεθᾰ";
    opt_mid_2: "~νι~αιο", "~νι~αισθον", "~νι~αισθε";
    opt_mid_3: "~νι~αιτο", "~νι~αισθην", "~νι~αιντο";
    imp_act_2: "~νι~η", _, _;
    imp_act_3: "~νι~ᾰτω", "~νι~ᾰτων", "~νι~ᾰντων";
    imp_mid_2: "~νι~ᾰσο|~νι~ω", _, _;
    imp_mid_3: "~νι~ᾰσθω", "~νι~ᾰσθων", _;
    imperfect_act_1: "~νῑ~ην", _, "~νῑ~ᾰμεν";
    imperfect_act_2: "~νῑ~ης", "~νῑ~ᾰτον", "~νῑ~ᾰτε";
    imperfect_act_3: "~νῑ~η", "~νῑ~ᾰτην", "~νῑ~ᾰσᾰν";
    imperfect_mid_1: "~νῑ~ᾰμην", _, "~νῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~νῑ~ᾰσο", "~νῑ~ᾰσθον", "~νῑ~ᾰσθε";
    imperfect_mid_3: "~νῑ~ᾰτο", "~νῑ~ᾰσθην", "~νῑ~ᾰντο";
    future_ind_act_1: "~~ησω", _, "~~ησομεν";
    future_ind_act_2: "~~ησεις", "~~ησετον", "~~ησετε";
    future_ind_act_3: "~~ησει", _, "~~ησουσῐ|~~ησουσῐν";
    future_ind_mid_1: "~~ησομαι", _, "~~ησομεθᾰ";
    future_ind_mid_2: "~~ησῃ|~~ησει", "~~ησεσθον", "~~ησεσθε";
    future_ind_mid_3: "~~ησεται", _, "~~ησονται";
    future_opt_act_1: "~~ησοιμῐ", _, "~~ησοιμεν";
    future_opt_act_2: "~~ησοις", "~~ησοιτον", "~~ησοιτε";
    future_opt_act_3: "~~ησοι", "~~ησοιτην", "~~ησοιεν";
    future_opt_mid_1: "~~ησοιμην", _, "~~ησοιμεθᾰ";
    future_opt_mid_2: "~~ησοιο", "~~ησοισθον", "~~ησοισθε";
    future_opt_mid_3: "~~ησοιτο", "~~ησοισθην", "~~ησοιντο";
    aorist_subj_act_1: "~~ω", _, "~~ωμεν";
    aorist_subj_act_2: "~~ῃς", "~~ητον", "~~ητε";
    aorist_subj_act_3: "~~ῃ", _, "~~ωσῐ";
    aorist_subj_mid_1: "~~ωμαι", _, "~~ωμεθᾰ";
    aorist_subj_mid_2: _, "~~ησθον", "~~ησθε";
    aorist_subj_mid_3: "~~ηται", _, "~~ωνται";
    aorist_subj_pass_1: "~~ᾰθω", _, "~~ᾰθωμεν";
    aorist_subj_pass_2: "~~ᾰθῃς", "~~ᾰθητον", "~~ᾰθητε";
    aorist_subj_pass_3: "~~ᾰθῃ", _, "~~ᾰθωσῐ";
    aorist_opt_act_1: "~~αιην", _, "~~αῑμεν";
    aorist_opt_act_2: "~~αιης", "~~αῑτον", "~~αῑτε";
    aorist_opt_act_3: "~~αιη", "~~αιτην", "~~αῑεν";
    aorist_opt_mid_1: "~~αιμην", _, "~~αιμεθᾰ";
    aorist_opt_mid_2: "~~αῑο", "~~αῑσθον", "~~αῑσθε";
    aorist_opt_mid_3: "~~αῑτο", "~~αισθην", "~~αῑντο";
    aorist_opt_pass_1: "~~ᾰθειην", _, "~~ᾰθεῑμεν";
    aorist_opt_pass_2: "~~ᾰθειης", "~~ᾰθεῑτον", "~~ᾰθεῑτε";
    aorist_opt_pass_3: "~~ᾰθειη", "~~ᾰθειτην", "~~ᾰθεῑεν";
    aorist_imp_act_2: "~~ηθῐ", "~~ητον", "~~ητε";
    aorist_imp_act_3: "~~ητω", "~~ητων", "~~ᾰντων";
    aorist_imp_mid_2: "~~ω", "~~ᾰσθον", "~~ᾰσθε";
    aorist_imp_mid_3: "~~ᾰσθω", "~~ᾰσθων", _;
    aorist_imp_pass_2: "~~ᾰθηθῐ", _, _;
    aorist_imp_pass_3: "~~ᾰθητω", "~~ᾰθητων", "~~ᾰθεντων";
    perfect_ind_act_1: "~νε~ηκᾰ", _, "~νε~ηκᾰμεν";
    perfect_ind_act_2: "~νε~ηκᾰς", "~νε~ηκᾰτον", "~νε~ηκᾰτε";
    perfect_ind_act_3: "~νε~ηκε", _, "~νε~ηκᾱσῐ";
    perfect_subj_act_1: "~νε~ηκω", _, "~νε~ηκωμεν";
    perfect_subj_act_2: "~νε~ηκῃς", "~νε~ηκητον", "~νε~ηκητε";
    perfect_subj_act_3: "~νε~ηκῃ", _, "~νε~ηκωσῐ";
    perfect_opt_act_1: "~νε~ηκοιμῐ", _, "~νε~ηκοιμεν";
    perfect_opt_act_2: "~νε~ηκοις", "~νε~ηκοιτον", "~νε~ηκοιτε";
    perfect_opt_act_3: "~νε~ηκοι", "~νε~ηκοιτην", "~νε~ηκοιεν";
    perfect_imp_act_2: _, "~νε~ηκετον", "~νε~ηκετε";
    perfect_imp_act_3: "~νε~ηκετω", "~νε~ηκετων", "~νε~ηκοντων";
    infinitive_pres: "~νι~ᾰναι", "~νι~ᾰσθαι", _;
    infinitive_fut: "~~ησειν", "~~ησεσθαι", _;
    infinitive_aor: "~~ηναι", "~~ᾰσθαι", "~~ᾰθηναι";
    infinitive_perf: "~νε~ηκεναι", _, _;
    participle_pres: "~νι~ᾱς", "~νι~ᾰμενος", _;
    participle_fut: "~~ησων", "~~ησομενος", _;
    participle_aor: "~~ᾱς", "~~ᾰμενος", "~~ᾰθεις";
    participle_perf: "~νε~ηκως", _, _;
    ),
    // Ἐφῐστημῐ: 3 words (ἐφῐστημῐ, ἀφῐστημῐ, ὑφῐστημῐ). Like Ξαινω, except in 262 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐφῐστημῐ", 3, "~φῐ~ημῐ", like Ξαινω;
    stem aorist_ind: "~πε~- like ἐφῐστημῐ";
    stem aorist_ind_pass: "~πε~ᾰθη- like ἐλπω";
    stem pluperfect_act: "~φε~ηκε- like ἀνῐστημῐ";
    ind_act_1: "~φῐ~ημῐ", _, "~φῐ~ᾰμεν";
    ind_act_2: "~φῐ~ης", "~φῐ~ᾰτον", "~φῐ~ᾰτε";
    ind_act_3: "~φῐ~ησῐ|~φῐ~ησῐν", _, "~φῐ~ᾱσῐ|~φῐ~ᾱσῐν";
    ind_mid_1: "~φῐ~ᾰμαι", _, "~φῐ~ᾰμεθᾰ";
    ind_mid_2: "~φῐ~ᾰσαι", "~φῐ~ᾰσθον", "~φῐ~ᾰσθε";
    ind_mid_3: "~φῐ~ᾰται", _, "~φῐ~ᾰνται";
    subj_act_1: "~φῐ~ω", _, "~φῐ~ωμεν";
    subj_act_2: "~φῐ~ῃς", "~φῐ~ητον", "~φῐ~ητε";
    subj_act_3: "~φῐ~ῃ", _, "~φῐ~ωσῐ|~φῐ~ωσῐν";
    subj_mid_1: "~φῐ~ωμαι", _, "~φῐ~ωμεθᾰ";
    subj_mid_2: _, "~φῐ~ησθον", "~φῐ~ησθε";
    subj_mid_3: "~φῐ~ηται", _, "~φῐ~ωνται";
    opt_act_1: "~φῐ~αιην", _, "~φῐ~αῑμεν|~φῐ~αιημεν";
    opt_act_2: "~φῐ~αιης", "~φῐ~αῑτον|~φῐ~αιητον", "~φῐ~αῑτε|~φῐ~αιητε";
    opt_act_3: "~φῐ~αιη", "~φῐ~αιτην|~φῐ~αιητην", "~φῐ~αῑεν|~φῐ~αιησᾰν";
    opt_mid_1: "~φῐ~αιμην", _, "~φῐ~αιμεθᾰ";
    opt_mid_2: "~φῐ~αιο", "~φῐ~αισθον", "~φῐ~αισθε";
    opt_mid_3: "~φῐ~αιτο", "~φῐ~αισθην", "~φῐ~αιντο";
    imp_act_2: "~φῐ~η", _, _;
    imp_act_3: "~φῐ~ᾰτω", "~φῐ~ᾰτων", "~φῐ~ᾰντων";
    imp_mid_2: "~φῐ~ᾰσο|~φῐ~ω", _, _;
    imp_mid_3: "~φῐ~ᾰσθω", "~φῐ~ᾰσθων", _;
    imperfect_act_1: "~φῑ~ην", _, "~φῑ~ᾰμεν";
    imperfect_act_2: "~φῑ~ης", "~φῑ~ᾰτον", "~φῑ~ᾰτε";
    imperfect_act_3: "~φῑ~η", "~φῑ~ᾰτην", "~φῑ~ᾰσᾰν";
    imperfect_mid_1: "~φῑ~ᾰμην", _, "~φῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~φῑ~ᾰσο", "~φῑ~ᾰσθον", "~φῑ~ᾰσθε";
    imperfect_mid_3: "~φῑ~ᾰτο", "~φῑ~ᾰσθην", "~φῑ~ᾰντο";
    future_ind_act_1: "~πο~ησω", _, "~πο~ησομεν";
    future_ind_act_2: "~πο~ησεις", "~πο~ησετον", "~πο~ησετε";
    future_ind_act_3: "~πο~ησει", _, "~πο~ησουσῐ|~πο~ησουσῐν";
    future_ind_mid_1: "~πο~ησομαι", _, "~πο~ησομεθᾰ";
    future_ind_mid_2: "~πο~ησῃ|~πο~ησει", "~πο~ησεσθον", "~πο~ησεσθε";
    future_ind_mid_3: "~πο~ησεται", _, "~πο~ησονται";
    future_ind_pass_1: "~πο~ησομαι", _, "~πο~ησομεθᾰ";
    future_ind_pass_2: "~πο~ησῃ", "~πο~ησεσθον", "~πο~ησεσθε";
    future_ind_pass_3: "~πο~ησεται", _, "~πο~ησονται";
    future_opt_act_1: "~πο~ησοιμῐ", _, "~πο~ησοιμεν";
    future_opt_act_2: "~πο~ησοις", "~πο~ησοιτον", "~πο~ησοιτε";
    future_opt_act_3: "~πο~ησοι", "~πο~ησοιτην", "~πο~ησοιεν";
    future_opt_mid_1: "~πο~ησοιμην", _, "~πο~ησοιμεθᾰ";
    future_opt_mid_2: "~πο~ησοιο", "~πο~ησοισθον", "~πο~ησοισθε";
    future_opt_mid_3: "~πο~ησοιτο", "~πο~ησοισθην", "~πο~ησοιντο";
    future_opt_pass_1: "~πο~ησοιμην", _, "~πο~ησοιμεθᾰ";
    future_opt_pass_2: "~πο~ησοιο", "~πο~ησοισθον", "~πο~ησοισθε";
    future_opt_pass_3: "~πο~ησοιτο", "~πο~ησοισθην", "~πο~ησοιντο";
    aorist_subj_act_1: "~πο~ησω|~πο~ω", _, "~πο~ησωμεν|~πο~ωμεν";
    aorist_subj_act_2: "~πο~ησῃς|~πο~ῃς", "~πο~ησητον|~πο~ητον", "~πο~ησητε|~πο~ητε";
    aorist_subj_act_3: "~πο~ησῃ|~πο~ῃ", _, "~πο~ησωσῐ|~πο~ησωσῐν|~πο~ωσῐ|~πο~ωσῐν";
    aorist_subj_mid_1: "~πο~ησωμαι|~πο~ωμαι", _, "~πο~ησωμεθᾰ|~πο~ωμεθᾰ";
    aorist_subj_mid_2: _, "~πο~ησησθον|~πο~ησθον", "~πο~ησησθε|~πο~ησθε";
    aorist_subj_mid_3: "~πο~ησηται|~πο~ηται", _, "~πο~ησωνται|~πο~ωνται";
    aorist_subj_pass_1: "~πο~ᾰθω", _, "~πο~ᾰθωμεν";
    aorist_subj_pass_2: "~πο~ᾰθῃς", "~πο~ᾰθητον", "~πο~ᾰθητε";
    aorist_subj_pass_3: "~πο~ᾰθῃ", _, "~πο~ᾰθωσῐ|~πο~ᾰθωσῐν";
    aorist_opt_act_1: "~πο~ησαιμῐ|~πο~αιην", _, "~πο~ησαιμεν|~πο~αῑμεν|~πο~αιημεν";
    aorist_opt_act_2: "~πο~ησειᾰς|~πο~ησαις|~πο~αιης", "~πο~ησαιτον|~πο~αῑτον|~πο~αιητον", "~πο~ησαιτε|~πο~αῑτε|~πο~αιητε";
    aorist_opt_act_3: "~πο~ησειε|~πο~ησειεν|~πο~ησαι|~πο~αιη", "~πο~ησαιτην|~πο~αιτην|~πο~αιητην", "~πο~ησειᾰν|~πο~ησαιεν|~πο~αῑεν|~πο~αιησᾰν";
    aorist_opt_mid_1: "~πο~ησαιμην|~πο~αιμην", _, "~πο~ησαιμεθᾰ|~πο~αιμεθᾰ";
    aorist_opt_mid_2: "~πο~ησαιο|~πο~αῑο", "~πο~ησαισθον|~πο~αῑσθον", "~πο~ησαισθε|~πο~αῑσθε";
    aorist_opt_mid_3: "~πο~ησαιτο|~πο~αῑτο", "~πο~ησαισθην|~πο~αισθην", "~πο~ησαιντο|~πο~αῑντο";
    aorist_opt_pass_1: "~πο~ᾰθειην", _, "~πο~ᾰθεῑμεν|~πο~ᾰθειημεν";
    aorist_opt_pass_2: "~πο~ᾰθειης", "~πο~ᾰθεῑτον|~πο~ᾰθειητον", "~πο~ᾰθεῑτε|~πο~ᾰθειητε";
    aorist_opt_pass_3: "~πο~ᾰθειη", "~πο~ᾰθειτην|~πο~ᾰθειητην", "~πο~ᾰθεῑεν|~πο~ᾰθειησᾰν";
    aorist_imp_act_2: "~πο~ησον|~πο~ηθῐ", "~πο~ησᾰτον|~πο~ητον", "~πο~ησᾰτε|~πο~ητε";
    aorist_imp_act_3: "~πο~ησᾰτω|~πο~ητω", "~πο~ησᾰτων|~πο~ητων", "~πο~ησᾰντων|~πο~ᾰντων";
    aorist_imp_mid_2: "~πο~ησαι|~πο~ω", "~πο~ησᾰσθον|~πο~ᾰσθον", "~πο~ησᾰσθε|~πο~ᾰσθε";
    aorist_imp_mid_3: "~πο~ησᾰσθω|~πο~ᾰσθω", "~πο~ησᾰσθων|~πο~ᾰσθων", _;
    aorist_imp_pass_2: "~πο~ᾰθητῐ", _, _;
    aorist_imp_pass_3: "~πο~ᾰθητω", "~πο~ᾰθητων", "~πο~ᾰθεντων";
    perfect_ind_act_1: "~φε~ᾰκᾰ|~φε~ηκᾰ", _, "~φε~ᾰκᾰμεν";
    perfect_ind_act_2: "~φε~ᾰκᾰς|~φε~ηκᾰς", "~φε~ᾰκᾰτον", "~φε~ᾰκᾰτε";
    perfect_ind_act_3: "~φε~ᾰκε|~φε~ᾰκεν|~φε~ηκε|~φε~ηκεν", _, "~φε~ᾰκᾱσῐ";
    perfect_ind_mid_1: "~φε~ᾰμαι", _, "~φε~ᾰμεθᾰ";
    perfect_ind_mid_2: "~φε~ᾰσαι", "~φε~ᾰσθον", "~φε~ᾰσθε";
    perfect_ind_mid_3: "~φε~ᾰται", _, "~φε~ᾰνται";
    perfect_subj_act_1: "~φε~ᾰκω", _, "~φε~ᾰκωμεν";
    perfect_subj_act_2: "~φε~ᾰκῃς", "~φε~ᾰκητον", "~φε~ᾰκητε";
    perfect_subj_act_3: "~φε~ᾰκῃ", _, "~φε~ᾰκωσῐ";
    perfect_opt_act_1: "~φε~ᾰκοιμῐ", _, "~φε~ᾰκοιμεν";
    perfect_opt_act_2: "~φε~ᾰκοις", "~φε~ᾰκοιτον", "~φε~ᾰκοιτε";
    perfect_opt_act_3: "~φε~ᾰκοι", "~φε~ᾰκοιτην", "~φε~ᾰκοιεν";
    perfect_imp_act_2: "~φε~ᾰκε", "~φε~ᾰκετον", "~φε~ᾰκετε";
    perfect_imp_act_3: "~φε~ᾰκετω", "~φε~ᾰκετων", "~φε~ᾰκοντων";
    perfect_imp_mid_2: "~φε~ᾰσο", _, _;
    perfect_imp_mid_3: "~φε~ᾰσθω", "~φε~ᾰσθων", _;
    future_perfect_ind_act_1: "~φε~ηξω", _, "~φε~ηξομεν";
    future_perfect_ind_act_2: "~φε~ηξεις", "~φε~ηξετον", "~φε~ηξετε";
    future_perfect_ind_act_3: "~φε~ηξει", _, "~φε~ηξουσῐ|~φε~ηξουσῐν";
    future_perfect_ind_mid_1: "~φε~ηξομαι", _, "~φε~ηξομεθᾰ";
    future_perfect_ind_mid_2: "~φε~ηξῃ|~φε~ηξει", "~φε~ηξεσθον", "~φε~ηξεσθε";
    future_perfect_ind_mid_3: "~φε~ηξεται", _, "~φε~ηξονται";
    future_perfect_opt_act_1: "~φε~ηξοιμῐ", _, "~φε~ηξοιμεν";
    future_perfect_opt_act_2: "~φε~ηξοις", "~φε~ηξοιτον", "~φε~ηξοιτε";
    future_perfect_opt_act_3: "~φε~ηξοι", "~φε~ηξοιτην", "~φε~ηξοιεν";
    future_perfect_opt_mid_1: "~φε~ηξοιμην", _, "~φε~ηξοιμεθᾰ";
    future_perfect_opt_mid_2: "~φε~ηξοιο", "~φε~ηξοισθον", "~φε~ηξοισθε";
    future_perfect_opt_mid_3: "~φε~ηξοιτο", "~φε~ηξοισθην", "~φε~ηξοιντο";
    infinitive_pres: "~φῐ~ᾰναι", "~φῐ~ᾰσθαι", _;
    infinitive_fut: "~πο~ησειν", "~πο~ησεσθαι", "~πο~ησεσθαι";
    infinitive_aor: "~πο~ησαι|~πο~ηναι", "~πο~ησᾰσθαι|~πο~ᾰσθαι", "~πο~ᾰθηναι";
    infinitive_perf: "~φε~ᾰκεναι", "~φε~ᾰσθαι", _;
    infinitive_futp: "~φε~ηξειν", "~φε~ηξεσθαι", _;
    participle_pres: "~φῐ~ᾱς", "~φῐ~ᾰμενος", _;
    participle_fut: "~πο~ησων", "~πο~ησομενος", "~πο~ησομενος";
    participle_aor: "~πο~ησᾱς|~πο~ᾱς", "~πο~ησᾰμενος|~πο~ᾰμενος", "~πο~ᾰθεις";
    participle_perf: "~φε~ᾰκως", "~φε~ᾰμενος", _;
    participle_futp: "~φε~ηξων", "~φε~ηξομενος", _;
    ),
    // Δῐαιρω: 3 words (ἐπαιρω, δῐαιρω, ῥῠπαινω). Like Ξαινω, except in 122 cells: ind_act_3 pl, ind_mid_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, ….
    conjugation!("δῐαιρω", 3, "~αι~ω", like Ξαινω;
    stem aorist_ind: "~η~- like εὐλογεω";
    stem aorist_ind_pass: "~η~θη- like ἐλπω";
    ind_act_3: _, _, "~αι~ουσῐ|~αι~ουσῐν";
    ind_mid_2: "~αι~ῃ|~αι~ει", _, _;
    subj_act_3: _, _, "~αι~ωσῐ|~αι~ωσῐν";
    imperfect_act_1: "~ῃ~ον", _, "~ῃ~ομεν";
    imperfect_act_2: "~ῃ~ες", "~ῃ~ετον", "~ῃ~ετε";
    imperfect_act_3: "~ῃ~ε|~ῃ~εν", "~ῃ~ετην", "~ῃ~ον";
    imperfect_mid_1: "~ῃ~ομην", _, "~ῃ~ομεθᾰ";
    imperfect_mid_2: "~ῃ~ου", "~ῃ~εσθον", "~ῃ~εσθε";
    imperfect_mid_3: "~ῃ~ετο", "~ῃ~εσθην", "~ῃ~οντο";
    future_ind_act_3: _, _, "~ᾰ~οῡσῐ|~ᾰ~οῡσῐν";
    future_ind_mid_2: "~ᾰ~εῑ|~ᾰ~ῃ", _, _;
    future_ind_pass_2: "~ᾰ~θησῃ|~ᾰ~θησει", _, _;
    future_opt_act_1: "~ᾰ~οιην|~ᾰ~οῑμῐ", _, "~ᾰ~οῑμεν|~ᾰ~οιημεν";
    future_opt_act_2: "~ᾰ~οιης|~ᾰ~οῑς", "~ᾰ~οῑτον|~ᾰ~οιητον", "~ᾰ~οῑτε|~ᾰ~οιητε";
    future_opt_act_3: "~ᾰ~οιη|~ᾰ~οῑ", "~ᾰ~οιτην|~ᾰ~οιητην", "~ᾰ~οῑεν|~ᾰ~οιησᾰν";
    aorist_subj_act_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    aorist_subj_act_2: "~ᾰ~ῃς", "~ᾰ~ητον", "~ᾰ~ητε";
    aorist_subj_act_3: "~ᾰ~ῃ", _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰ~ησθον", "~ᾰ~ησθε";
    aorist_subj_mid_3: "~ᾰ~ηται", _, "~ᾰ~ωνται";
    aorist_subj_pass_3: _, _, "~ᾰ~θωσῐ|~ᾰ~θωσῐν";
    aorist_opt_act_1: "~ᾰ~αιμῐ", _, "~ᾰ~αιμεν";
    aorist_opt_act_2: "~ᾰ~ειᾰς|~ᾰ~αις", "~ᾰ~αιτον", "~ᾰ~αιτε";
    aorist_opt_act_3: "~ᾰ~ειε|~ᾰ~ειεν|~ᾰ~αι", "~ᾰ~αιτην", "~ᾰ~ειᾰν|~ᾰ~αιεν";
    aorist_opt_mid_1: "~ᾰ~αιμην", _, "~ᾰ~αιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~αιο", "~ᾰ~αισθον", "~ᾰ~αισθε";
    aorist_opt_mid_3: "~ᾰ~αιτο", "~ᾰ~αισθην", "~ᾰ~αιντο";
    aorist_opt_pass_1: _, _, "~ᾰ~θεῑμεν|~ᾰ~θειημεν";
    aorist_opt_pass_2: _, "~ᾰ~θεῑτον|~ᾰ~θειητον", "~ᾰ~θεῑτε|~ᾰ~θειητε";
    aorist_opt_pass_3: _, "~ᾰ~θειτην|~ᾰ~θειητην", "~ᾰ~θεῑεν|~ᾰ~θειησᾰν";
    aorist_imp_act_2: "~ᾰ~ον", "~ᾰ~ᾰτον", "~ᾰ~ᾰτε";
    aorist_imp_act_3: "~ᾰ~ᾰτω", "~ᾰ~ᾰτων", "~ᾰ~ᾰντων";
    aorist_imp_mid_2: "~ᾰ~αι", "~ᾰ~ᾰσθον", "~ᾰ~ᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ᾰσθω", "~ᾰ~ᾰσθων", _;
    perfect_ind_act_1: "~η~κᾰ", _, "~η~κᾰμεν";
    perfect_ind_act_2: "~η~κᾰς", "~η~κᾰτον", "~η~κᾰτε";
    perfect_ind_act_3: "~η~κε|~η~κεν", _, "~η~κᾱσῐ|~η~κᾱσῐν";
    perfect_ind_mid_1: "~η~μαι", _, "~η~μεθᾰ";
    perfect_ind_mid_2: "~η~σαι", "~η~θον", "~η~θε";
    perfect_ind_mid_3: "~η~ται", _, "~η~ᾰται";
    perfect_subj_act_1: "~η~κω", _, "~η~κωμεν";
    perfect_subj_act_2: "~η~κῃς", "~η~κητον", "~η~κητε";
    perfect_subj_act_3: "~η~κῃ", _, "~η~κωσῐ|~η~κωσῐν";
    perfect_opt_act_1: "~η~κοιμῐ|~η~κοιην", _, "~η~κοιμεν";
    perfect_opt_act_2: "~η~κοις|~η~κοιης", "~η~κοιτον", "~η~κοιτε";
    perfect_opt_act_3: "~η~κοι|~η~κοιη", "~η~κοιτην", "~η~κοιεν";
    perfect_imp_act_2: "~η~κε", "~η~κετον", "~η~κετε";
    perfect_imp_act_3: "~η~κετω", "~η~κετων", "~η~κοντων";
    perfect_imp_mid_2: "~η~σο", _, _;
    perfect_imp_mid_3: "~η~θω", "~η~θων", _;
    infinitive_aor: "~ᾰ~αι", "~ᾰ~ᾰσθαι", _;
    infinitive_perf: "~η~κεναι", "~η~θαι", _;
    participle_aor: "~ᾰ~ᾱς", "~ᾰ~ᾰμενος", _;
    participle_perf: "~η~κως", "~η~μενος", _;
    ),
    // Ἐγκειμαι: 3 words (συγκειμαι, ξυγκειμαι, ἐγκειμαι). Like Πῠνθανομαι, except in 46 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἐγκειμαι", 3, "~γ~ειμαι", like Πῠνθανομαι as MID_AS_PASS;
    stem imperfect: "~νε~ε- like ἐγκειμαι";
    ind_act_1: "~γ~ειμαι", _, _;
    ind_mid_1: "~γ~ειμαι", _, "~γ~ειμεθα";
    ind_mid_2: "~γ~εισαι", "~γ~εισθον", "~γ~εισθε";
    ind_mid_3: "~γ~ειται", _, "~γ~εινται|~γ~εᾰται|~γ~εονται|~γ~ειᾰται";
    ind_pass_2: derived, _, _;
    subj_mid_1: "~γ~ωμαι", _, "~γ~ωμεθᾰ";
    subj_mid_2: "~γ~εῃ", "~γ~εησθον", "~γ~εησθε";
    subj_mid_3: "~γ~εηται|~γ~εῑται", _, "~γ~εωνται|~γ~ειωνται";
    subj_pass_2: derived, _, _;
    opt_mid_1: "~γ~οιμην", _, "~γ~οιμεθᾰ";
    opt_mid_2: "~γ~οῑο", "~γ~οῑσθον", "~γ~οῑσθε";
    opt_mid_3: "~γ~οῑτο", "~γ~οισθην", "~γ~οῑντο";
    imp_mid_2: "~γ~εισο", _, "~γ~εισθον";
    imp_mid_3: "~γ~εισθω", "~γ~εισθων", _;
    future_ind_mid_1: "~γ~εισομαι", _, "~γ~εισομεθᾰ";
    future_ind_mid_2: "~γ~εισῃ|~γ~εισει", "~γ~εισεσθον", "~γ~εισεσθε";
    future_ind_mid_3: "~γ~εισεται", _, "~γ~εισονται";
    future_opt_mid_1: "~γ~εισοιμην", _, "~γ~εισοιμεθᾰ";
    future_opt_mid_2: "~γ~εισοιο", "~γ~εισοισθον", "~γ~εισοισθε";
    future_opt_mid_3: "~γ~εισοιτο", "~γ~εισοισθην", "~γ~εισοιντο";
    infinitive_fut: _, "~γ~εισεσθαι", _;
    participle_fut: _, "~γ~εισομενος", _;
    ),
    // Πᾰρᾰκᾰλεω: 3 words (πᾰρᾰκᾰλεω, ἀνᾰκᾰλεω, κᾰτᾰκᾰλεω). Like Πεμπω, except in 236 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("πᾰρᾰκᾰλεω", 3, "~ᾰκᾰ~εω", like Πεμπω;
    stem aorist_ind: "~εκᾰ~εσ- like εὐλογεω";
    stem aorist_ind_pass: "~εκ~ηθη- like ἐλπω";
    stem pluperfect_act: "~εκεκ~ηκ- like εὐλογεω";
    ind_act_1: "~ᾰκᾰ~ω", _, "~ᾰκᾰ~οῡμεν";
    ind_act_2: "~ᾰκᾰ~εῑς", "~ᾰκᾰ~εῑτον", "~ᾰκᾰ~εῑτε";
    ind_act_3: "~ᾰκᾰ~εῑ", _, "~ᾰκᾰ~οῡσῐ|~ᾰκᾰ~οῡσῐν";
    ind_mid_1: "~ᾰκᾰ~οῡμαι", _, "~ᾰκᾰ~ουμεθᾰ";
    ind_mid_2: "~ᾰκᾰ~εῑ|~ᾰκᾰ~ῃ", "~ᾰκᾰ~εῑσθον", "~ᾰκᾰ~εῑσθε";
    ind_mid_3: "~ᾰκᾰ~εῑται", _, "~ᾰκᾰ~οῡνται";
    subj_act_1: "~ᾰκᾰ~ω", _, "~ᾰκᾰ~ωμεν";
    subj_act_2: "~ᾰκᾰ~ῃς", "~ᾰκᾰ~ητον", "~ᾰκᾰ~ητε";
    subj_act_3: "~ᾰκᾰ~ῃ", _, "~ᾰκᾰ~ωσῐ|~ᾰκᾰ~ωσῐν";
    subj_mid_1: "~ᾰκᾰ~ωμαι", _, "~ᾰκᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ᾰκᾰ~ησθον", "~ᾰκᾰ~ησθε";
    subj_mid_3: "~ᾰκᾰ~ηται", _, "~ᾰκᾰ~ωνται";
    opt_act_1: "~ᾰκᾰ~οιην|~ᾰκᾰ~οῑμῐ", _, "~ᾰκᾰ~οῑμεν|~ᾰκᾰ~οιημεν";
    opt_act_2: "~ᾰκᾰ~οιης|~ᾰκᾰ~οῑς", "~ᾰκᾰ~οῑτον|~ᾰκᾰ~οιητον", "~ᾰκᾰ~οῑτε|~ᾰκᾰ~οιητε";
    opt_act_3: "~ᾰκᾰ~οιη|~ᾰκᾰ~οῑ", "~ᾰκᾰ~οιτην|~ᾰκᾰ~οιητην", "~ᾰκᾰ~οῑεν|~ᾰκᾰ~οιησᾰν";
    opt_mid_1: "~ᾰκᾰ~οιμην", _, "~ᾰκᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰκᾰ~οῑο", "~ᾰκᾰ~οῑσθον", "~ᾰκᾰ~οῑσθε";
    opt_mid_3: "~ᾰκᾰ~οῑτο", "~ᾰκᾰ~οισθην", "~ᾰκᾰ~οῑντο";
    imp_act_2: "~ᾰκᾰ~ει", _, _;
    imp_act_3: "~ᾰκᾰ~ειτω", "~ᾰκᾰ~ειτων", "~ᾰκᾰ~ουντων";
    imp_mid_2: "~ᾰκᾰ~οῡ", _, _;
    imp_mid_3: "~ᾰκᾰ~εισθω", "~ᾰκᾰ~εισθων", _;
    imperfect_act_1: "~εκᾰ~ουν", _, "~εκᾰ~οῡμεν";
    imperfect_act_2: "~εκᾰ~εις", "~εκᾰ~εῑτον", "~εκᾰ~εῑτε";
    imperfect_act_3: "~εκᾰ~ει", "~εκᾰ~ειτην", "~εκᾰ~ουν";
    imperfect_mid_1: "~εκᾰ~ουμην", _, "~εκᾰ~ουμεθᾰ";
    imperfect_mid_2: "~εκᾰ~οῡ", "~εκᾰ~εῑσθον", "~εκᾰ~εῑσθε";
    imperfect_mid_3: "~εκᾰ~εῑτο", "~εκᾰ~εισθην", "~εκᾰ~οῡντο";
    future_ind_act_1: "~ᾰκᾰ~ω", _, "~ᾰκᾰ~οῡμεν";
    future_ind_act_2: "~ᾰκᾰ~εῑς", "~ᾰκᾰ~εῑτον", "~ᾰκᾰ~εῑτε";
    future_ind_act_3: "~ᾰκᾰ~εῑ", _, "~ᾰκᾰ~οῡσῐ|~ᾰκᾰ~οῡσῐν";
    future_ind_mid_1: "~ᾰκᾰ~οῡμαι", _, "~ᾰκᾰ~ουμεθᾰ";
    future_ind_mid_2: "~ᾰκᾰ~εῑ|~ᾰκᾰ~ῃ", "~ᾰκᾰ~εῑσθον", "~ᾰκᾰ~εῑσθε";
    future_ind_mid_3: "~ᾰκᾰ~εῑται", _, "~ᾰκᾰ~οῡνται";
    future_ind_pass_1: "~ᾰκεκ~ησομαι", _, "~ᾰκεκ~ησομεθᾰ";
    future_ind_pass_2: "~ᾰκεκ~ησῃ", "~ᾰκεκ~ησεσθον", "~ᾰκεκ~ησεσθε";
    future_ind_pass_3: "~ᾰκεκ~ησεται", _, "~ᾰκεκ~ησονται";
    future_opt_act_1: "~ᾰκᾰ~οιην|~ᾰκᾰ~οῑμῐ", _, "~ᾰκᾰ~οῑμεν|~ᾰκᾰ~οιημεν";
    future_opt_act_2: "~ᾰκᾰ~οιης|~ᾰκᾰ~οῑς", "~ᾰκᾰ~οῑτον|~ᾰκᾰ~οιητον", "~ᾰκᾰ~οῑτε|~ᾰκᾰ~οιητε";
    future_opt_act_3: "~ᾰκᾰ~οιη|~ᾰκᾰ~οῑ", "~ᾰκᾰ~οιτην|~ᾰκᾰ~οιητην", "~ᾰκᾰ~οῑεν|~ᾰκᾰ~οιησᾰν";
    future_opt_mid_1: "~ᾰκᾰ~οιμην", _, "~ᾰκᾰ~οιμεθᾰ";
    future_opt_mid_2: "~ᾰκᾰ~οῑο", "~ᾰκᾰ~οῑσθον", "~ᾰκᾰ~οῑσθε";
    future_opt_mid_3: "~ᾰκᾰ~οῑτο", "~ᾰκᾰ~οισθην", "~ᾰκᾰ~οῑντο";
    future_opt_pass_1: "~ᾰκεκ~ησοιμην", _, "~ᾰκεκ~ησοιμεθᾰ";
    future_opt_pass_2: "~ᾰκεκ~ησοιο", "~ᾰκεκ~ησοισθον", "~ᾰκεκ~ησοισθε";
    future_opt_pass_3: "~ᾰκεκ~ησοιτο", "~ᾰκεκ~ησοισθην", "~ᾰκεκ~ησοιντο";
    aorist_subj_act_1: "~ᾰκᾰ~εσω", _, "~ᾰκᾰ~εσωμεν";
    aorist_subj_act_2: "~ᾰκᾰ~εσῃς", "~ᾰκᾰ~εσητον", "~ᾰκᾰ~εσητε";
    aorist_subj_act_3: "~ᾰκᾰ~εσῃ", _, "~ᾰκᾰ~εσωσῐ|~ᾰκᾰ~εσωσῐν";
    aorist_subj_mid_1: "~ᾰκᾰ~εσωμαι", _, "~ᾰκᾰ~εσωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰκᾰ~εσησθον", "~ᾰκᾰ~εσησθε";
    aorist_subj_mid_3: "~ᾰκᾰ~εσηται", _, "~ᾰκᾰ~εσωνται";
    aorist_subj_pass_1: "~ᾰκ~ηθω", _, "~ᾰκ~ηθωμεν";
    aorist_subj_pass_2: "~ᾰκ~ηθῃς", "~ᾰκ~ηθητον", "~ᾰκ~ηθητε";
    aorist_subj_pass_3: "~ᾰκ~ηθῃ", _, "~ᾰκ~ηθωσῐ|~ᾰκ~ηθωσῐν";
    aorist_opt_act_1: "~ᾰκᾰ~εσαιμῐ", _, "~ᾰκᾰ~εσαιμεν";
    aorist_opt_act_2: "~ᾰκᾰ~εσειᾰς|~ᾰκᾰ~εσαις", "~ᾰκᾰ~εσαιτον", "~ᾰκᾰ~εσαιτε";
    aorist_opt_act_3: "~ᾰκᾰ~εσειε|~ᾰκᾰ~εσειεν|~ᾰκᾰ~εσαι", "~ᾰκᾰ~εσαιτην", "~ᾰκᾰ~εσειᾰν|~ᾰκᾰ~εσαιεν";
    aorist_opt_mid_1: "~ᾰκᾰ~εσαιμην", _, "~ᾰκᾰ~εσαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰκᾰ~εσαιο", "~ᾰκᾰ~εσαισθον", "~ᾰκᾰ~εσαισθε";
    aorist_opt_mid_3: "~ᾰκᾰ~εσαιτο", "~ᾰκᾰ~εσαισθην", "~ᾰκᾰ~εσαιντο";
    aorist_opt_pass_1: "~ᾰκ~ηθειην", _, "~ᾰκ~ηθεῑμεν|~ᾰκ~ηθειημεν";
    aorist_opt_pass_2: "~ᾰκ~ηθειης", "~ᾰκ~ηθεῑτον|~ᾰκ~ηθειητον", "~ᾰκ~ηθεῑτε|~ᾰκ~ηθειητε";
    aorist_opt_pass_3: "~ᾰκ~ηθειη", "~ᾰκ~ηθειτην|~ᾰκ~ηθειητην", "~ᾰκ~ηθεῑεν|~ᾰκ~ηθειησᾰν";
    aorist_imp_act_2: "~ᾰκᾰ~εσον", "~ᾰκᾰ~εσᾰτον", "~ᾰκᾰ~εσᾰτε";
    aorist_imp_act_3: "~ᾰκᾰ~εσᾰτω", "~ᾰκᾰ~εσᾰτων", "~ᾰκᾰ~εσᾰντων";
    aorist_imp_mid_2: "~ᾰκᾰ~εσαι", "~ᾰκᾰ~εσᾰσθον", "~ᾰκᾰ~εσᾰσθε";
    aorist_imp_mid_3: "~ᾰκᾰ~εσᾰσθω", "~ᾰκᾰ~εσᾰσθων", _;
    aorist_imp_pass_2: "~ᾰκ~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰκ~ηθητω", "~ᾰκ~ηθητων", "~ᾰκ~ηθεντων";
    perfect_ind_act_1: "~ᾰκεκ~ηκᾰ", _, "~ᾰκεκ~ηκᾰμεν";
    perfect_ind_act_2: "~ᾰκεκ~ηκᾰς", "~ᾰκεκ~ηκᾰτον", "~ᾰκεκ~ηκᾰτε";
    perfect_ind_act_3: "~ᾰκεκ~ηκε|~ᾰκεκ~ηκεν", _, "~ᾰκεκ~ηκᾱσῐ|~ᾰκεκ~ηκᾱσῐν";
    perfect_ind_mid_1: "~ᾰκεκ~ημαι", _, "~ᾰκεκ~ημεθᾰ";
    perfect_ind_mid_2: "~ᾰκεκ~ησαι", "~ᾰκεκ~ησθον", "~ᾰκεκ~ησθε";
    perfect_ind_mid_3: "~ᾰκεκ~ηται", _, "~ᾰκεκ~ηνται";
    perfect_subj_act_1: "~ᾰκεκ~ηκω", _, "~ᾰκεκ~ηκωμεν";
    perfect_subj_act_2: "~ᾰκεκ~ηκῃς", "~ᾰκεκ~ηκητον", "~ᾰκεκ~ηκητε";
    perfect_subj_act_3: "~ᾰκεκ~ηκῃ", _, "~ᾰκεκ~ηκωσῐ|~ᾰκεκ~ηκωσῐν";
    perfect_opt_act_1: "~ᾰκεκ~ηκοιμῐ|~ᾰκεκ~ηκοιην", _, "~ᾰκεκ~ηκοιμεν";
    perfect_opt_act_2: "~ᾰκεκ~ηκοις|~ᾰκεκ~ηκοιης", "~ᾰκεκ~ηκοιτον", "~ᾰκεκ~ηκοιτε";
    perfect_opt_act_3: "~ᾰκεκ~ηκοι|~ᾰκεκ~ηκοιη", "~ᾰκεκ~ηκοιτην", "~ᾰκεκ~ηκοιεν";
    perfect_imp_act_2: "~ᾰκεκ~ηκε", "~ᾰκεκ~ηκετον", "~ᾰκεκ~ηκετε";
    perfect_imp_act_3: "~ᾰκεκ~ηκετω", "~ᾰκεκ~ηκετων", "~ᾰκεκ~ηκοντων";
    perfect_imp_mid_2: "~ᾰκεκ~ησο", _, _;
    perfect_imp_mid_3: "~ᾰκεκ~ησθω", "~ᾰκεκ~ησθων", _;
    pluperfect_mid_1: "~εκεκ~ημην", _, "~εκεκ~ημεθᾰ";
    pluperfect_mid_2: "~εκεκ~ησο", "~εκεκ~ησθον", "~εκεκ~ησθε";
    pluperfect_mid_3: "~εκεκ~ητο", "~εκεκ~ησθην", "~εκεκ~ηντο";
    infinitive_pres: "~ᾰκᾰ~εῑν", "~ᾰκᾰ~εῑσθαι", _;
    infinitive_fut: "~ᾰκᾰ~εῑν", "~ᾰκᾰ~εῑσθαι", "~ᾰκεκ~ησεσθαι";
    infinitive_aor: "~ᾰκᾰ~εσαι", "~ᾰκᾰ~εσᾰσθαι", "~ᾰκ~ηθηναι";
    infinitive_perf: "~ᾰκεκ~ηκεναι", "~ᾰκεκ~ησθαι", _;
    participle_pres: "~ᾰκᾰ~ων", "~ᾰκᾰ~ουμενος", _;
    participle_fut: "~ᾰκᾰ~ων", "~ᾰκᾰ~ουμενος", "~ᾰκεκ~ησομενος";
    participle_aor: "~ᾰκᾰ~εσᾱς", "~ᾰκᾰ~εσᾰμενος", "~ᾰκ~ηθεις";
    participle_perf: "~ᾰκεκ~ηκως", "~ᾰκεκ~ημενος", _;
    ),
    // Στελλω: 3 words (στελλω, ἀγγελλω, τελλω). Like Καθιζω, except in 167 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("στελλω", 3, "~~λω", like Καθιζω as AORIST_AS_FUTURE;
    stem imperfect: "~~λ- like ῥεω";
    stem aorist_ind: "~ι~- like ἐχω";
    stem pluperfect_act: "~τα~κε- like κᾰλεω";
    ind_act_1: "~~λω", _, "~~λομεν";
    ind_act_2: "~~λεις", "~~λετον", "~~λετε";
    ind_act_3: "~~λει", _, "~~λουσῐ";
    ind_mid_1: "~~λομαι", _, "~~λομεθᾰ";
    ind_mid_2: "~~λῃ", "~~λεσθον", "~~λεσθε";
    ind_mid_3: "~~λεται", _, "~~λονται";
    subj_act_1: "~~λω", _, "~~λωμεν";
    subj_act_2: "~~λῃς", "~~λητον", "~~λητε";
    subj_act_3: "~~λῃ", _, "~~λωσῐ";
    subj_mid_1: "~~λωμαι", _, "~~λωμεθᾰ";
    subj_mid_2: _, "~~λησθον", "~~λησθε";
    subj_mid_3: "~~ληται", _, "~~λωνται";
    opt_act_1: "~~λοιμῐ", _, "~~λοιμεν";
    opt_act_2: "~~λοις", "~~λοιτον", "~~λοιτε";
    opt_act_3: "~~λοι", "~~λοιτην", "~~λοιεν";
    opt_mid_1: "~~λοιμην", _, "~~λοιμεθᾰ";
    opt_mid_2: "~~λοιο", "~~λοισθον", "~~λοισθε";
    opt_mid_3: "~~λοιτο", "~~λοισθην", "~~λοιντο";
    imp_act_2: "~~λε", _, _;
    imp_act_3: "~~λετω", "~~λετων", "~~λοντων";
    imp_mid_2: "~~λου", _, _;
    imp_mid_3: "~~λεσθω", "~~λεσθων", _;
    future_ind_mid_2: "~~εῑ", _, _;
    future_ind_pass_1: "~~θησομαι", _, "~~θησομεθᾰ";
    future_ind_pass_2: "~~θησῃ", "~~θησεσθον", "~~θησεσθε";
    future_ind_pass_3: "~~θησεται", _, "~~θησονται";
    future_opt_pass_1: "~~θησοιμην", _, "~~θησοιμεθᾰ";
    future_opt_pass_2: "~~θησοιο", "~~θησοισθον", "~~θησοισθε";
    future_opt_pass_3: "~~θησοιτο", "~~θησοισθην", "~~θησοιντο";
    aorist_subj_act_1: "~ι~ω", _, "~ι~ωμεν";
    aorist_subj_act_2: "~ι~ῃς", "~ι~ητον", "~ι~ητε";
    aorist_subj_act_3: "~ι~ῃ", _, "~ι~ωσῐ";
    aorist_subj_mid_1: "~ι~ωμαι", _, "~ι~ωμεθᾰ";
    aorist_subj_mid_2: _, "~ι~ησθον", "~ι~ησθε";
    aorist_subj_mid_3: "~ι~ηται", _, "~ι~ωνται";
    aorist_opt_act_1: "~ι~αιμῐ", _, "~ι~αιμεν";
    aorist_opt_act_2: "~ι~ειᾰς", "~ι~αιτον", "~ι~αιτε";
    aorist_opt_act_3: "~ι~ειε", "~ι~αιτην", "~ι~ειᾰν";
    aorist_opt_mid_1: "~ι~αιμην", _, "~ι~αιμεθᾰ";
    aorist_opt_mid_2: "~ι~αιο", "~ι~αισθον", "~ι~αισθε";
    aorist_opt_mid_3: "~ι~αιτο", "~ι~αισθην", "~ι~αιντο";
    aorist_imp_act_2: "~ῑ~ον", "~ι~ᾰτον", "~ι~ᾰτε";
    aorist_imp_act_3: "~ι~ᾰτω", "~ι~ᾰτων", "~ι~ᾰντων";
    aorist_imp_mid_2: "~ῑ~αι", "~ι~ᾰσθον", "~ι~ᾰσθε";
    aorist_imp_mid_3: "~ι~ᾰσθω", "~ι~ᾰσθων", _;
    perfect_ind_act_1: "~τα~κᾰ", _, "~τα~κᾰμεν";
    perfect_ind_act_2: "~τα~κᾰς", "~τα~κᾰτον", "~τα~κᾰτε";
    perfect_ind_act_3: "~τα~κε", _, "~τα~κᾱσῐ";
    perfect_ind_mid_1: "~τα~μαι", _, "~τα~μεθᾰ";
    perfect_ind_mid_2: "~τα~σαι", "~τα~θον", "~τα~θε";
    perfect_ind_mid_3: "~τα~ται", _, "~τα~ᾰται";
    perfect_subj_act_1: "~τα~κω", _, "~τα~κωμεν";
    perfect_subj_act_2: "~τα~κῃς", "~τα~κητον", "~τα~κητε";
    perfect_subj_act_3: "~τα~κῃ", _, "~τα~κωσῐ";
    perfect_opt_act_1: "~τα~κοιμῐ", _, "~τα~κοιμεν";
    perfect_opt_act_2: "~τα~κοις", "~τα~κοιτον", "~τα~κοιτε";
    perfect_opt_act_3: "~τα~κοι", "~τα~κοιτην", "~τα~κοιεν";
    perfect_imp_act_2: _, "~τα~κετον", "~τα~κετε";
    perfect_imp_act_3: "~τα~κετω", "~τα~κετων", "~τα~κοντων";
    perfect_imp_mid_2: "~τα~σο", _, _;
    perfect_imp_mid_3: "~τα~θω", "~τα~θων", _;
    pluperfect_mid_1: "ἐ~τα~μην", _, "ἐ~τα~μεθᾰ";
    pluperfect_mid_2: "ἐ~τα~σο", "ἐ~τα~θον", "ἐ~τα~θε";
    pluperfect_mid_3: "ἐ~τα~το", "ἐ~τα~θην", "ἐ~τα~ᾰτο";
    infinitive_pres: "~~λειν", "~~λεσθαι", _;
    infinitive_fut: _, _, "~~θησεσθαι";
    infinitive_aor: "~ῑ~αι", "~ι~ᾰσθαι", derived;
    infinitive_perf: "~τα~κεναι", "~τᾱ~θαι", _;
    participle_pres: "~~λων", "~~λομενος", _;
    participle_fut: _, _, "~~θησομενος";
    participle_aor: "~ι~ᾱς", "~ι~ᾰμενος", derived;
    participle_perf: "~τα~κως", "~τα~μενος", _;
    ),
    // Ἐξαιρω: 3 words (κᾰταιρω, ἀνταιρω, ἐξαιρω). Like Δῐαιρω, except in 44 cells: aorist_subj_act_1 sg, aorist_subj_act_1 pl, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_2 pl, aorist_subj_act_3 sg, ….
    conjugation!("ἐξαιρω", 3, "~αι~ω", like Δῐαιρω;
    aorist_subj_act_1: "~α~ω", _, "~α~ωμεν";
    aorist_subj_act_2: "~α~ῃς", "~α~ητον", "~α~ητε";
    aorist_subj_act_3: "~α~ῃ", _, "~α~ωσῐ|~α~ωσῐν";
    aorist_subj_mid_1: "~α~ωμαι", _, "~α~ωμεθᾰ";
    aorist_subj_mid_2: _, "~α~ησθον", "~α~ησθε";
    aorist_subj_mid_3: "~α~ηται", _, "~α~ωνται";
    aorist_opt_act_1: "~α~αιμῐ", _, "~α~αιμεν";
    aorist_opt_act_2: "~α~ειᾰς|~α~αις", "~α~αιτον", "~α~αιτε";
    aorist_opt_act_3: "~α~ειε|~α~ειεν|~α~αι", "~α~αιτην", "~α~ειᾰν|~α~αιεν";
    aorist_opt_mid_1: "~α~αιμην", _, "~α~αιμεθᾰ";
    aorist_opt_mid_2: "~α~αιο", "~α~αισθον", "~α~αισθε";
    aorist_opt_mid_3: "~α~αιτο", "~α~αισθην", "~α~αιντο";
    aorist_imp_act_2: "~α~ον", "~α~ᾰτον", "~α~ᾰτε";
    aorist_imp_act_3: "~α~ᾰτω", "~α~ᾰτων", "~α~ᾰντων";
    aorist_imp_mid_2: "~α~αι", "~α~ᾰσθον", "~α~ᾰσθε";
    aorist_imp_mid_3: "~α~ᾰσθω", "~α~ᾰσθων", _;
    infinitive_aor: "~ᾱ~αι", "~α~ᾰσθαι", _;
    participle_aor: "~α~ᾱς", "~α~ᾰμενος", _;
    ),
    // Δῐᾰλῡω: 3 words (ἀντᾰνᾰλῡω, δῐᾰλῡω, κᾰτᾰλῡω). Like Πεμπω, except in 253 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("δῐᾰλῡω", 3, "~ᾰ~ῡω", like Πεμπω;
    stem aorist_ind: "~ε~ῡσ- like εὐλογεω";
    stem aorist_ind_pass: "~ε~ῠθη- like ἐλπω";
    stem pluperfect_act: "~ε~ελῠκ- like εὐλογεω";
    ind_act_1: "~ᾰ~ῡω", _, "~ᾰ~ῡομεν";
    ind_act_2: "~ᾰ~ῡεις", "~ᾰ~ῡετον", "~ᾰ~ῡετε";
    ind_act_3: "~ᾰ~ῡει", _, "~ᾰ~ῡουσῐ|~ᾰ~ῡουσῐν";
    ind_mid_1: "~ᾰ~ῡομαι", _, "~ᾰ~ῡομεθᾰ";
    ind_mid_2: "~ᾰ~ῡῃ|~ᾰ~ῡει", "~ᾰ~ῡεσθον", "~ᾰ~ῡεσθε";
    ind_mid_3: "~ᾰ~ῡεται", _, "~ᾰ~ῡονται";
    subj_act_1: "~ᾰ~ῡω", _, "~ᾰ~ῡωμεν";
    subj_act_2: "~ᾰ~ῡῃς", "~ᾰ~ῡητον", "~ᾰ~ῡητε";
    subj_act_3: "~ᾰ~ῡῃ", _, "~ᾰ~ῡωσῐ|~ᾰ~ῡωσῐν";
    subj_mid_1: "~ᾰ~ῡωμαι", _, "~ᾰ~ῡωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ῡησθον", "~ᾰ~ῡησθε";
    subj_mid_3: "~ᾰ~ῡηται", _, "~ᾰ~ῡωνται";
    opt_act_1: "~ᾰ~ῡοιμῐ", _, "~ᾰ~ῡοιμεν";
    opt_act_2: "~ᾰ~ῡοις", "~ᾰ~ῡοιτον", "~ᾰ~ῡοιτε";
    opt_act_3: "~ᾰ~ῡοι", "~ᾰ~ῡοιτην", "~ᾰ~ῡοιεν";
    opt_mid_1: "~ᾰ~ῡοιμην", _, "~ᾰ~ῡοιμεθᾰ";
    opt_mid_2: "~ᾰ~ῡοιο", "~ᾰ~ῡοισθον", "~ᾰ~ῡοισθε";
    opt_mid_3: "~ᾰ~ῡοιτο", "~ᾰ~ῡοισθην", "~ᾰ~ῡοιντο";
    imp_act_2: "~ᾰ~ῡε", _, _;
    imp_act_3: "~ᾰ~ῡετω", "~ᾰ~ῡετων", "~ᾰ~ῡοντων";
    imp_mid_2: "~ᾰ~ῡου", _, _;
    imp_mid_3: "~ᾰ~ῡεσθω", "~ᾰ~ῡεσθων", _;
    imperfect_act_1: "~ε~ῡον", _, "~ε~ῡομεν";
    imperfect_act_2: "~ε~ῡες", "~ε~ῡετον", "~ε~ῡετε";
    imperfect_act_3: "~ε~ῡε|~ε~ῡεν", "~ε~ῡετην", "~ε~ῡον";
    imperfect_mid_1: "~ε~ῡομην", _, "~ε~ῡομεθᾰ";
    imperfect_mid_2: "~ε~ῡου", "~ε~ῡεσθον", "~ε~ῡεσθε";
    imperfect_mid_3: "~ε~ῡετο", "~ε~ῡεσθην", "~ε~ῡοντο";
    future_ind_act_1: "~ᾰ~ῡσω", _, "~ᾰ~ῡσομεν";
    future_ind_act_2: "~ᾰ~ῡσεις", "~ᾰ~ῡσετον", "~ᾰ~ῡσετε";
    future_ind_act_3: "~ᾰ~ῡσει", _, "~ᾰ~ῡσουσῐ|~ᾰ~ῡσουσῐν";
    future_ind_mid_1: "~ᾰ~ῡσομαι", _, "~ᾰ~ῡσομεθᾰ";
    future_ind_mid_2: "~ᾰ~ῡσῃ|~ᾰ~ῡσει", "~ᾰ~ῡσεσθον", "~ᾰ~ῡσεσθε";
    future_ind_mid_3: "~ᾰ~ῡσεται", _, "~ᾰ~ῡσονται";
    future_ind_pass_1: "~ᾰ~ῠθησομαι", _, "~ᾰ~ῠθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ῠθησῃ|~ᾰ~ῠθησει", "~ᾰ~ῠθησεσθον", "~ᾰ~ῠθησεσθε";
    future_ind_pass_3: "~ᾰ~ῠθησεται", _, "~ᾰ~ῠθησονται";
    future_opt_act_1: "~ᾰ~ῡσοιμῐ", _, "~ᾰ~ῡσοιμεν";
    future_opt_act_2: "~ᾰ~ῡσοις", "~ᾰ~ῡσοιτον", "~ᾰ~ῡσοιτε";
    future_opt_act_3: "~ᾰ~ῡσοι", "~ᾰ~ῡσοιτην", "~ᾰ~ῡσοιεν";
    future_opt_mid_1: "~ᾰ~ῡσοιμην", _, "~ᾰ~ῡσοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ῡσοιο", "~ᾰ~ῡσοισθον", "~ᾰ~ῡσοισθε";
    future_opt_mid_3: "~ᾰ~ῡσοιτο", "~ᾰ~ῡσοισθην", "~ᾰ~ῡσοιντο";
    future_opt_pass_1: "~ᾰ~ῠθησοιμην", _, "~ᾰ~ῠθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ῠθησοιο", "~ᾰ~ῠθησοισθον", "~ᾰ~ῠθησοισθε";
    future_opt_pass_3: "~ᾰ~ῠθησοιτο", "~ᾰ~ῠθησοισθην", "~ᾰ~ῠθησοιντο";
    aorist_subj_act_1: "~ᾰ~ῡσω", _, "~ᾰ~ῡσωμεν";
    aorist_subj_act_2: "~ᾰ~ῡσῃς", "~ᾰ~ῡσητον", "~ᾰ~ῡσητε";
    aorist_subj_act_3: "~ᾰ~ῡσῃ", _, "~ᾰ~ῡσωσῐ|~ᾰ~ῡσωσῐν";
    aorist_subj_mid_1: "~ᾰ~ῡσωμαι", _, "~ᾰ~ῡσωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰ~ῡσησθον", "~ᾰ~ῡσησθε";
    aorist_subj_mid_3: "~ᾰ~ῡσηται", _, "~ᾰ~ῡσωνται";
    aorist_subj_pass_1: "~ᾰ~ῠθω", _, "~ᾰ~ῠθωμεν";
    aorist_subj_pass_2: "~ᾰ~ῠθῃς", "~ᾰ~ῠθητον", "~ᾰ~ῠθητε";
    aorist_subj_pass_3: "~ᾰ~ῠθῃ", _, "~ᾰ~ῠθωσῐ|~ᾰ~ῠθωσῐν";
    aorist_opt_act_1: "~ᾰ~ῡσαιμῐ", _, "~ᾰ~ῡσαιμεν";
    aorist_opt_act_2: "~ᾰ~ῡσειᾰς|~ᾰ~ῡσαις", "~ᾰ~ῡσαιτον", "~ᾰ~ῡσαιτε";
    aorist_opt_act_3: "~ᾰ~ῡσειε|~ᾰ~ῡσειεν|~ᾰ~ῡσαι", "~ᾰ~ῡσαιτην", "~ᾰ~ῡσειᾰν|~ᾰ~ῡσαιεν";
    aorist_opt_mid_1: "~ᾰ~ῡσαιμην", _, "~ᾰ~ῡσαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~ῡσαιο", "~ᾰ~ῡσαισθον", "~ᾰ~ῡσαισθε";
    aorist_opt_mid_3: "~ᾰ~ῡσαιτο", "~ᾰ~ῡσαισθην", "~ᾰ~ῡσαιντο";
    aorist_opt_pass_1: "~ᾰ~ῠθειην", _, "~ᾰ~ῠθεῑμεν|~ᾰ~ῠθειημεν";
    aorist_opt_pass_2: "~ᾰ~ῠθειης", "~ᾰ~ῠθεῑτον|~ᾰ~ῠθειητον", "~ᾰ~ῠθεῑτε|~ᾰ~ῠθειητε";
    aorist_opt_pass_3: "~ᾰ~ῠθειη", "~ᾰ~ῠθειτην|~ᾰ~ῠθειητην", "~ᾰ~ῠθεῑεν|~ᾰ~ῠθειησᾰν";
    aorist_imp_act_2: "~ᾰ~ῡσον", "~ᾰ~ῡσᾰτον", "~ᾰ~ῡσᾰτε";
    aorist_imp_act_3: "~ᾰ~ῡσᾰτω", "~ᾰ~ῡσᾰτων", "~ᾰ~ῡσᾰντων";
    aorist_imp_mid_2: "~ᾰ~ῡσαι", "~ᾰ~ῡσᾰσθον", "~ᾰ~ῡσᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ῡσᾰσθω", "~ᾰ~ῡσᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~ῠθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ῠθητω", "~ᾰ~ῠθητων", "~ᾰ~ῠθεντων";
    perfect_ind_act_1: "~ᾰ~ελῠκᾰ", _, "~ᾰ~ελῠκᾰμεν";
    perfect_ind_act_2: "~ᾰ~ελῠκᾰς", "~ᾰ~ελῠκᾰτον", "~ᾰ~ελῠκᾰτε";
    perfect_ind_act_3: "~ᾰ~ελῠκε|~ᾰ~ελῠκεν", _, "~ᾰ~ελῠκᾱσῐ|~ᾰ~ελῠκᾱσῐν";
    perfect_ind_mid_1: "~ᾰ~ελῠμαι", _, "~ᾰ~ελῠμεθᾰ";
    perfect_ind_mid_2: "~ᾰ~ελῠσαι", "~ᾰ~ελῠσθον", "~ᾰ~ελῠσθε";
    perfect_ind_mid_3: "~ᾰ~ελῠται", _, "~ᾰ~ελῠνται";
    perfect_subj_act_1: "~ᾰ~ελῠκω", _, "~ᾰ~ελῠκωμεν";
    perfect_subj_act_2: "~ᾰ~ελῠκῃς", "~ᾰ~ελῠκητον", "~ᾰ~ελῠκητε";
    perfect_subj_act_3: "~ᾰ~ελῠκῃ", _, "~ᾰ~ελῠκωσῐ|~ᾰ~ελῠκωσῐν";
    perfect_opt_act_1: "~ᾰ~ελῠκοιμῐ|~ᾰ~ελῠκοιην", _, "~ᾰ~ελῠκοιμεν";
    perfect_opt_act_2: "~ᾰ~ελῠκοις|~ᾰ~ελῠκοιης", "~ᾰ~ελῠκοιτον", "~ᾰ~ελῠκοιτε";
    perfect_opt_act_3: "~ᾰ~ελῠκοι|~ᾰ~ελῠκοιη", "~ᾰ~ελῠκοιτην", "~ᾰ~ελῠκοιεν";
    perfect_imp_act_2: "~ᾰ~ελῠκε", "~ᾰ~ελῠκετον", "~ᾰ~ελῠκετε";
    perfect_imp_act_3: "~ᾰ~ελῠκετω", "~ᾰ~ελῠκετων", "~ᾰ~ελῠκοντων";
    perfect_imp_mid_2: "~ᾰ~ελῠσο", _, _;
    perfect_imp_mid_3: "~ᾰ~ελῠσθω", "~ᾰ~ελῠσθων", _;
    pluperfect_mid_1: "~ε~ελῠμην", _, "~ε~ελῠμεθᾰ";
    pluperfect_mid_2: "~ε~ελῠσο", "~ε~ελῠσθον", "~ε~ελῠσθε";
    pluperfect_mid_3: "~ε~ελῠτο", "~ε~ελῠσθην", "~ε~ελῠντο";
    future_perfect_ind_mid_1: "~ᾰ~ελῡσομαι", _, "~ᾰ~ελῡσομεθᾰ";
    future_perfect_ind_mid_2: "~ᾰ~ελῡσῃ|~ᾰ~ελῡσει", "~ᾰ~ελῡσεσθον", "~ᾰ~ελῡσεσθε";
    future_perfect_ind_mid_3: "~ᾰ~ελῡσεται", _, "~ᾰ~ελῡσονται";
    future_perfect_opt_mid_1: "~ᾰ~ελῡσοιμην", _, "~ᾰ~ελῡσοιμεθᾰ";
    future_perfect_opt_mid_2: "~ᾰ~ελῡσοιο", "~ᾰ~ελῡσοισθον", "~ᾰ~ελῡσοισθε";
    future_perfect_opt_mid_3: "~ᾰ~ελῡσοιτο", "~ᾰ~ελῡσοισθην", "~ᾰ~ελῡσοιντο";
    infinitive_pres: "~ᾰ~ῡειν", "~ᾰ~ῡεσθαι", _;
    infinitive_fut: "~ᾰ~ῡσειν", "~ᾰ~ῡσεσθαι", "~ᾰ~ῠθησεσθαι";
    infinitive_aor: "~ᾰ~ῡσαι", "~ᾰ~ῡσᾰσθαι", "~ᾰ~ῠθηναι";
    infinitive_perf: "~ᾰ~ελῠκεναι", "~ᾰ~ελῠσθαι", _;
    infinitive_futp: _, "~ᾰ~ελῡσεσθαι", _;
    participle_pres: "~ᾰ~ῡων", "~ᾰ~ῡομενος", _;
    participle_fut: "~ᾰ~ῡσων", "~ᾰ~ῡσομενος", "~ᾰ~ῠθησομενος";
    participle_aor: "~ᾰ~ῡσᾱς", "~ᾰ~ῡσᾰμενος", "~ᾰ~ῠθεις";
    participle_perf: "~ᾰ~ελῠκως", "~ᾰ~ελῠμενος", _;
    participle_futp: _, "~ᾰ~ελῡσομενος", _;
    ),
    // Ἐξαναλῡω: 3 words (ἐξαναλῡω, ἐπαναλῡω, ἐπαπολῡω). Like Δῐᾰλῡω, except in 229 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐξαναλῡω", 3, "~α~ῡω", like Δῐᾰλῡω;
    ind_act_1: "~α~ῡω", _, "~α~ῡομεν";
    ind_act_2: "~α~ῡεις", "~α~ῡετον", "~α~ῡετε";
    ind_act_3: "~α~ῡει", _, "~α~ῡουσῐ|~α~ῡουσῐν";
    ind_mid_1: "~α~ῡομαι", _, "~α~ῡομεθᾰ";
    ind_mid_2: "~α~ῡῃ|~α~ῡει", "~α~ῡεσθον", "~α~ῡεσθε";
    ind_mid_3: "~α~ῡεται", _, "~α~ῡονται";
    subj_act_1: "~α~ῡω", _, "~α~ῡωμεν";
    subj_act_2: "~α~ῡῃς", "~α~ῡητον", "~α~ῡητε";
    subj_act_3: "~α~ῡῃ", _, "~α~ῡωσῐ|~α~ῡωσῐν";
    subj_mid_1: "~α~ῡωμαι", _, "~α~ῡωμεθᾰ";
    subj_mid_2: _, "~α~ῡησθον", "~α~ῡησθε";
    subj_mid_3: "~α~ῡηται", _, "~α~ῡωνται";
    opt_act_1: "~α~ῡοιμῐ", _, "~α~ῡοιμεν";
    opt_act_2: "~α~ῡοις", "~α~ῡοιτον", "~α~ῡοιτε";
    opt_act_3: "~α~ῡοι", "~α~ῡοιτην", "~α~ῡοιεν";
    opt_mid_1: "~α~ῡοιμην", _, "~α~ῡοιμεθᾰ";
    opt_mid_2: "~α~ῡοιο", "~α~ῡοισθον", "~α~ῡοισθε";
    opt_mid_3: "~α~ῡοιτο", "~α~ῡοισθην", "~α~ῡοιντο";
    imp_act_2: "~α~ῡε", _, _;
    imp_act_3: "~α~ῡετω", "~α~ῡετων", "~α~ῡοντων";
    imp_mid_2: "~α~ῡου", _, _;
    imp_mid_3: "~α~ῡεσθω", "~α~ῡεσθων", _;
    future_ind_act_1: "~α~ῡσω", _, "~α~ῡσομεν";
    future_ind_act_2: "~α~ῡσεις", "~α~ῡσετον", "~α~ῡσετε";
    future_ind_act_3: "~α~ῡσει", _, "~α~ῡσουσῐ|~α~ῡσουσῐν";
    future_ind_mid_1: "~α~ῡσομαι", _, "~α~ῡσομεθᾰ";
    future_ind_mid_2: "~α~ῡσῃ|~α~ῡσει", "~α~ῡσεσθον", "~α~ῡσεσθε";
    future_ind_mid_3: "~α~ῡσεται", _, "~α~ῡσονται";
    future_ind_pass_1: "~α~ῠθησομαι", _, "~α~ῠθησομεθᾰ";
    future_ind_pass_2: "~α~ῠθησῃ|~α~ῠθησει", "~α~ῠθησεσθον", "~α~ῠθησεσθε";
    future_ind_pass_3: "~α~ῠθησεται", _, "~α~ῠθησονται";
    future_opt_act_1: "~α~ῡσοιμῐ", _, "~α~ῡσοιμεν";
    future_opt_act_2: "~α~ῡσοις", "~α~ῡσοιτον", "~α~ῡσοιτε";
    future_opt_act_3: "~α~ῡσοι", "~α~ῡσοιτην", "~α~ῡσοιεν";
    future_opt_mid_1: "~α~ῡσοιμην", _, "~α~ῡσοιμεθᾰ";
    future_opt_mid_2: "~α~ῡσοιο", "~α~ῡσοισθον", "~α~ῡσοισθε";
    future_opt_mid_3: "~α~ῡσοιτο", "~α~ῡσοισθην", "~α~ῡσοιντο";
    future_opt_pass_1: "~α~ῠθησοιμην", _, "~α~ῠθησοιμεθᾰ";
    future_opt_pass_2: "~α~ῠθησοιο", "~α~ῠθησοισθον", "~α~ῠθησοισθε";
    future_opt_pass_3: "~α~ῠθησοιτο", "~α~ῠθησοισθην", "~α~ῠθησοιντο";
    aorist_subj_act_1: "~α~ῡσω", _, "~α~ῡσωμεν";
    aorist_subj_act_2: "~α~ῡσῃς", "~α~ῡσητον", "~α~ῡσητε";
    aorist_subj_act_3: "~α~ῡσῃ", _, "~α~ῡσωσῐ|~α~ῡσωσῐν";
    aorist_subj_mid_1: "~α~ῡσωμαι", _, "~α~ῡσωμεθᾰ";
    aorist_subj_mid_2: _, "~α~ῡσησθον", "~α~ῡσησθε";
    aorist_subj_mid_3: "~α~ῡσηται", _, "~α~ῡσωνται";
    aorist_subj_pass_1: "~α~ῠθω", _, "~α~ῠθωμεν";
    aorist_subj_pass_2: "~α~ῠθῃς", "~α~ῠθητον", "~α~ῠθητε";
    aorist_subj_pass_3: "~α~ῠθῃ", _, "~α~ῠθωσῐ|~α~ῠθωσῐν";
    aorist_opt_act_1: "~α~ῡσαιμῐ", _, "~α~ῡσαιμεν";
    aorist_opt_act_2: "~α~ῡσειᾰς|~α~ῡσαις", "~α~ῡσαιτον", "~α~ῡσαιτε";
    aorist_opt_act_3: "~α~ῡσειε|~α~ῡσειεν|~α~ῡσαι", "~α~ῡσαιτην", "~α~ῡσειᾰν|~α~ῡσαιεν";
    aorist_opt_mid_1: "~α~ῡσαιμην", _, "~α~ῡσαιμεθᾰ";
    aorist_opt_mid_2: "~α~ῡσαιο", "~α~ῡσαισθον", "~α~ῡσαισθε";
    aorist_opt_mid_3: "~α~ῡσαιτο", "~α~ῡσαισθην", "~α~ῡσαιντο";
    aorist_opt_pass_1: "~α~ῠθειην", _, "~α~ῠθεῑμεν|~α~ῠθειημεν";
    aorist_opt_pass_2: "~α~ῠθειης", "~α~ῠθεῑτον|~α~ῠθειητον", "~α~ῠθεῑτε|~α~ῠθειητε";
    aorist_opt_pass_3: "~α~ῠθειη", "~α~ῠθειτην|~α~ῠθειητην", "~α~ῠθεῑεν|~α~ῠθειησᾰν";
    aorist_imp_act_2: "~α~ῡσον", "~α~ῡσᾰτον", "~α~ῡσᾰτε";
    aorist_imp_act_3: "~α~ῡσᾰτω", "~α~ῡσᾰτων", "~α~ῡσᾰντων";
    aorist_imp_mid_2: "~α~ῡσαι", "~α~ῡσᾰσθον", "~α~ῡσᾰσθε";
    aorist_imp_mid_3: "~α~ῡσᾰσθω", "~α~ῡσᾰσθων", _;
    aorist_imp_pass_2: "~α~ῠθητῐ", _, _;
    aorist_imp_pass_3: "~α~ῠθητω", "~α~ῠθητων", "~α~ῠθεντων";
    perfect_ind_act_1: "~α~ελῠκᾰ", _, "~α~ελῠκᾰμεν";
    perfect_ind_act_2: "~α~ελῠκᾰς", "~α~ελῠκᾰτον", "~α~ελῠκᾰτε";
    perfect_ind_act_3: "~α~ελῠκε|~α~ελῠκεν", _, "~α~ελῠκᾱσῐ|~α~ελῠκᾱσῐν";
    perfect_ind_mid_1: "~α~ελῠμαι", _, "~α~ελῠμεθᾰ";
    perfect_ind_mid_2: "~α~ελῠσαι", "~α~ελῠσθον", "~α~ελῠσθε";
    perfect_ind_mid_3: "~α~ελῠται", _, "~α~ελῠνται";
    perfect_subj_act_1: "~α~ελῠκω", _, "~α~ελῠκωμεν";
    perfect_subj_act_2: "~α~ελῠκῃς", "~α~ελῠκητον", "~α~ελῠκητε";
    perfect_subj_act_3: "~α~ελῠκῃ", _, "~α~ελῠκωσῐ|~α~ελῠκωσῐν";
    perfect_opt_act_1: "~α~ελῠκοιμῐ|~α~ελῠκοιην", _, "~α~ελῠκοιμεν";
    perfect_opt_act_2: "~α~ελῠκοις|~α~ελῠκοιης", "~α~ελῠκοιτον", "~α~ελῠκοιτε";
    perfect_opt_act_3: "~α~ελῠκοι|~α~ελῠκοιη", "~α~ελῠκοιτην", "~α~ελῠκοιεν";
    perfect_imp_act_2: "~α~ελῠκε", "~α~ελῠκετον", "~α~ελῠκετε";
    perfect_imp_act_3: "~α~ελῠκετω", "~α~ελῠκετων", "~α~ελῠκοντων";
    perfect_imp_mid_2: "~α~ελῠσο", _, _;
    perfect_imp_mid_3: "~α~ελῠσθω", "~α~ελῠσθων", _;
    future_perfect_ind_mid_1: "~α~ελῡσομαι", _, "~α~ελῡσομεθᾰ";
    future_perfect_ind_mid_2: "~α~ελῡσῃ|~α~ελῡσει", "~α~ελῡσεσθον", "~α~ελῡσεσθε";
    future_perfect_ind_mid_3: "~α~ελῡσεται", _, "~α~ελῡσονται";
    future_perfect_opt_mid_1: "~α~ελῡσοιμην", _, "~α~ελῡσοιμεθᾰ";
    future_perfect_opt_mid_2: "~α~ελῡσοιο", "~α~ελῡσοισθον", "~α~ελῡσοισθε";
    future_perfect_opt_mid_3: "~α~ελῡσοιτο", "~α~ελῡσοισθην", "~α~ελῡσοιντο";
    infinitive_pres: "~α~ῡειν", "~α~ῡεσθαι", _;
    infinitive_fut: "~α~ῡσειν", "~α~ῡσεσθαι", "~α~ῠθησεσθαι";
    infinitive_aor: "~α~ῡσαι", "~α~ῡσᾰσθαι", "~α~ῠθηναι";
    infinitive_perf: "~α~ελῠκεναι", "~α~ελῠσθαι", _;
    infinitive_futp: _, "~α~ελῡσεσθαι", _;
    participle_pres: "~α~ῡων", "~α~ῡομενος", _;
    participle_fut: "~α~ῡσων", "~α~ῡσομενος", "~α~ῠθησομενος";
    participle_aor: "~α~ῡσᾱς", "~α~ῡσᾰμενος", "~α~ῠθεις";
    participle_perf: "~α~ελῠκως", "~α~ελῠμενος", _;
    participle_futp: _, "~α~ελῡσομενος", _;
    ),
    // Ἁμᾰρτᾰνω: 3 words (ἁμᾰρτᾰνω, ἀφᾰμᾰρτᾰνω, ἀνᾰβλᾰστᾰνω). Like Ἐξᾰκολουθεω, except in 143 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἁμᾰρτᾰνω", 3, "~ᾰ~ᾰνω", like Ἐξᾰκολουθεω as PERFECT_AS_FUTURE;
    stem aorist_ind: "~η~- like ἀνᾰγω";
    ind_act_1: "~ᾰ~ᾰνω", _, "~ᾰ~ᾰνομεν";
    ind_act_2: "~ᾰ~ᾰνεις", "~ᾰ~ᾰνετον", "~ᾰ~ᾰνετε";
    ind_act_3: "~ᾰ~ᾰνει", _, "~ᾰ~ᾰνουσῐ|~ᾰ~ᾰνουσῐν";
    ind_mid_1: "~ᾰ~ᾰνομαι", _, "~ᾰ~ᾰνομεθᾰ";
    ind_mid_2: "~ᾰ~ᾰνῃ", "~ᾰ~ᾰνεσθον", "~ᾰ~ᾰνεσθε";
    ind_mid_3: "~ᾰ~ᾰνεται", _, "~ᾰ~ᾰνονται";
    subj_act_1: "~ᾰ~ᾰνω", _, "~ᾰ~ᾰνωμεν";
    subj_act_2: "~ᾰ~ᾰνῃς", "~ᾰ~ᾰνητον", "~ᾰ~ᾰνητε";
    subj_act_3: "~ᾰ~ᾰνῃ", _, "~ᾰ~ᾰνωσῐ|~ᾰ~ᾰνωσῐν";
    subj_mid_1: "~ᾰ~ᾰνωμαι", _, "~ᾰ~ᾰνωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ᾰνησθον", "~ᾰ~ᾰνησθε";
    subj_mid_3: "~ᾰ~ᾰνηται", _, "~ᾰ~ᾰνωνται";
    opt_act_1: "~ᾰ~ᾰνοιμῐ", _, "~ᾰ~ᾰνοιμεν";
    opt_act_2: "~ᾰ~ᾰνοις", "~ᾰ~ᾰνοιτον", "~ᾰ~ᾰνοιτε";
    opt_act_3: "~ᾰ~ᾰνοι", "~ᾰ~ᾰνοιτην", "~ᾰ~ᾰνοιεν";
    opt_mid_1: "~ᾰ~ᾰνοιμην", _, "~ᾰ~ᾰνοιμεθᾰ";
    opt_mid_2: "~ᾰ~ᾰνοιο", "~ᾰ~ᾰνοισθον", "~ᾰ~ᾰνοισθε";
    opt_mid_3: "~ᾰ~ᾰνοιτο", "~ᾰ~ᾰνοισθην", "~ᾰ~ᾰνοιντο";
    imp_act_2: "~ᾰ~ᾰνε", _, _;
    imp_act_3: "~ᾰ~ᾰνετω", "~ᾰ~ᾰνετων", "~ᾰ~ᾰνοντων";
    imp_mid_2: "~ᾰ~ᾰνου", _, _;
    imp_mid_3: "~ᾰ~ᾰνεσθω", "~ᾰ~ᾰνεσθων", _;
    imperfect_act_1: "~η~ᾰνον", _, "~η~ᾰνομεν";
    imperfect_act_2: "~η~ᾰνες", "~η~ᾰνετον", "~η~ᾰνετε";
    imperfect_act_3: "~η~ᾰνε", "~η~ᾰνετην", "~η~ᾰνον";
    imperfect_mid_1: "~η~ᾰνομην", _, "~η~ᾰνομεθᾰ";
    imperfect_mid_2: "~η~ᾰνου", "~η~ᾰνεσθον", "~η~ᾰνεσθε";
    imperfect_mid_3: "~η~ᾰνετο", "~η~ᾰνεσθην", "~η~ᾰνοντο";
    future_ind_mid_2: "~ᾰ~ησῃ", _, _;
    future_ind_pass_2: "~ᾰ~ηθησῃ", _, _;
    aorist_subj_act_1: "~ᾰ~ω|~ᾰ~ησω", _, "~ᾰ~ωμεν|~ᾰ~ησωμεν";
    aorist_subj_act_2: "~ᾰ~ῃς|~ᾰ~ησῃς", "~ᾰ~ητον|~ᾰ~ησητον", "~ᾰ~ητε|~ᾰ~ησητε";
    aorist_subj_act_3: "~ᾰ~ῃ|~ᾰ~ησῃ", _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν|~ᾰ~ησωσῐ|~ᾰ~ησωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~ῃ", "~ᾰ~ησθον", "~ᾰ~ησθε";
    aorist_subj_mid_3: "~ᾰ~ηται", _, "~ᾰ~ωνται";
    aorist_subj_pass_3: _, _, "~ᾰ~ηθωσῐ";
    aorist_opt_act_1: "~ᾰ~οιμῐ|~ᾰ~ησαιμῐ", _, "~ᾰ~οιμεν|~ᾰ~ησαιμεν";
    aorist_opt_act_2: "~ᾰ~οις|~ᾰ~ησειᾰς|~ᾰ~ησαις", "~ᾰ~οιτον|~ᾰ~ησαιτον", "~ᾰ~οιτε|~ᾰ~ησαιτε";
    aorist_opt_act_3: "~ᾰ~οι|~ᾰ~ησειε|~ᾰ~ησειεν|~ᾰ~ησαι", "~ᾰ~οιτην|~ᾰ~ησαιτην", "~ᾰ~οιεν|~ᾰ~ησειᾰν|~ᾰ~ησαιεν";
    aorist_opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~οιο", "~ᾰ~οισθον", "~ᾰ~οισθε";
    aorist_opt_mid_3: "~ᾰ~οιτο", "~ᾰ~οισθην", "~ᾰ~οιντο";
    aorist_opt_pass_1: _, _, "~ᾰ~ηθεῑμεν";
    aorist_opt_pass_2: _, "~ᾰ~ηθεῑτον", "~ᾰ~ηθεῑτε";
    aorist_opt_pass_3: _, "~ᾰ~ηθειτην", "~ᾰ~ηθεῑεν";
    aorist_imp_act_2: "~ᾰ~ε|~ᾰ~ησον", "~ᾰ~ετον|~ᾰ~ησᾰτον", "~ᾰ~ετε|~ᾰ~ησᾰτε";
    aorist_imp_act_3: "~ᾰ~ετω|~ᾰ~ησᾰτω", "~ᾰ~ετων|~ᾰ~ησᾰτων", "~ᾰ~οντων|~ᾰ~ησᾰντων";
    aorist_imp_mid_2: "~ᾰ~οῡ", "~ᾰ~εσθον", "~ᾰ~εσθε";
    aorist_imp_mid_3: "~ᾰ~εσθω", "~ᾰ~εσθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_pres: "~ᾰ~ᾰνειν", "~ᾰ~ᾰνεσθαι", _;
    infinitive_aor: "~ᾰ~εῑν|~ᾰ~ησαι", "~ᾰ~εσθαι", _;
    infinitive_perf: derived, derived, _;
    participle_pres: "~ᾰ~ᾰνων", "~ᾰ~ᾰνομενος", _;
    participle_aor: "~ᾰ~ων|~ᾰ~ησᾱς", "~ᾰ~ομενος", _;
    participle_perf: derived, derived, _;
    ),
    // Κᾰμπτω: 3 words (κλεπτω, κᾰμπτω, κολᾰπτω). Like Θρῠπτω, except in 29 cells: future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, future_ind_pass_2 du, future_ind_pass_2 pl, future_ind_pass_3 sg, ….
    conjugation!("κᾰμπτω", 3, "~πτω", like Θρῠπτω as PERFECT_AS_FUTURE;
    future_ind_pass_1: "~φθησομαι", _, "~φθησομεθᾰ";
    future_ind_pass_2: "~φθησῃ|~φθησει", "~φθησεσθον", "~φθησεσθε";
    future_ind_pass_3: "~φθησεται", _, "~φθησονται";
    future_opt_pass_1: "~φθησοιμην", _, "~φθησοιμεθᾰ";
    future_opt_pass_2: "~φθησοιο", "~φθησοισθον", "~φθησοισθε";
    future_opt_pass_3: "~φθησοιτο", "~φθησοισθην", "~φθησοιντο";
    perfect_ind_mid_1: "%ε~μμαι", _, "%ε~μμεθᾰ";
    perfect_ind_mid_2: "%ε~ψαι", "%ε~φθον", "%ε~φθε";
    perfect_ind_mid_3: "%ε~πται", _, "%ε~πᾰται";
    perfect_imp_mid_2: "%ε~ψο", _, _;
    perfect_imp_mid_3: "%ε~φθω", "%ε~φθων", _;
    infinitive_fut: _, _, "~φθησεσθαι";
    infinitive_perf: derived, _, _;
    participle_fut: _, _, "~φθησομενος";
    participle_perf: derived, _, _;
    ),
    // Μεθῐστημῐ: 3 words (ἐγκᾰθῐστημῐ, μεθῐστημῐ, ἀντῐκᾰθῐστημῐ). Like Ξαινω, except in 216 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("μεθῐστημῐ", 3, "~θῐ~ημῐ", like Ξαινω;
    stem aorist_ind: "~τε~- like ἐφῐστημῐ";
    stem aorist_ind_pass: "~τε~ᾰθη- like ἐλπω";
    ind_act_1: "~θῐ~ημῐ", _, "~θῐ~ᾰμεν";
    ind_act_2: "~θῐ~ης", "~θῐ~ᾰτον", "~θῐ~ᾰτε";
    ind_act_3: "~θῐ~ησῐ|~θῐ~ησῐν", _, "~θῐ~ᾱσῐ|~θῐ~ᾱσῐν";
    ind_mid_1: "~θῐ~ᾰμαι", _, "~θῐ~ᾰμεθᾰ";
    ind_mid_2: "~θῐ~ᾰσαι", "~θῐ~ᾰσθον", "~θῐ~ᾰσθε";
    ind_mid_3: "~θῐ~ᾰται", _, "~θῐ~ᾰνται";
    subj_act_1: "~θῐ~ω", _, "~θῐ~ωμεν";
    subj_act_2: "~θῐ~ῃς", "~θῐ~ητον", "~θῐ~ητε";
    subj_act_3: "~θῐ~ῃ", _, "~θῐ~ωσῐ|~θῐ~ωσῐν";
    subj_mid_1: "~θῐ~ωμαι", _, "~θῐ~ωμεθᾰ";
    subj_mid_2: _, "~θῐ~ησθον", "~θῐ~ησθε";
    subj_mid_3: "~θῐ~ηται", _, "~θῐ~ωνται";
    opt_act_1: "~θῐ~αιην", _, "~θῐ~αῑμεν|~θῐ~αιημεν";
    opt_act_2: "~θῐ~αιης", "~θῐ~αῑτον|~θῐ~αιητον", "~θῐ~αῑτε|~θῐ~αιητε";
    opt_act_3: "~θῐ~αιη", "~θῐ~αιτην|~θῐ~αιητην", "~θῐ~αῑεν|~θῐ~αιησᾰν";
    opt_mid_1: "~θῐ~αιμην", _, "~θῐ~αιμεθᾰ";
    opt_mid_2: "~θῐ~αιο", "~θῐ~αισθον", "~θῐ~αισθε";
    opt_mid_3: "~θῐ~αιτο", "~θῐ~αισθην", "~θῐ~αιντο";
    imp_act_2: "~θῐ~η", _, _;
    imp_act_3: "~θῐ~ᾰτω", "~θῐ~ᾰτων", "~θῐ~ᾰντων";
    imp_mid_2: "~θῐ~ᾰσο|~θῐ~ω", _, _;
    imp_mid_3: "~θῐ~ᾰσθω", "~θῐ~ᾰσθων", _;
    imperfect_act_1: "~θῑ~ην", _, "~θῑ~ᾰμεν";
    imperfect_act_2: "~θῑ~ης", "~θῑ~ᾰτον", "~θῑ~ᾰτε";
    imperfect_act_3: "~θῑ~η", "~θῑ~ᾰτην", "~θῑ~ᾰσᾰν";
    imperfect_mid_1: "~θῑ~ᾰμην", _, "~θῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~θῑ~ᾰσο", "~θῑ~ᾰσθον", "~θῑ~ᾰσθε";
    imperfect_mid_3: "~θῑ~ᾰτο", "~θῑ~ᾰσθην", "~θῑ~ᾰντο";
    future_ind_act_1: "~τᾰ~ησω", _, "~τᾰ~ησομεν";
    future_ind_act_2: "~τᾰ~ησεις", "~τᾰ~ησετον", "~τᾰ~ησετε";
    future_ind_act_3: "~τᾰ~ησει", _, "~τᾰ~ησουσῐ|~τᾰ~ησουσῐν";
    future_ind_mid_1: "~τᾰ~ησομαι", _, "~τᾰ~ησομεθᾰ";
    future_ind_mid_2: "~τᾰ~ησῃ|~τᾰ~ησει", "~τᾰ~ησεσθον", "~τᾰ~ησεσθε";
    future_ind_mid_3: "~τᾰ~ησεται", _, "~τᾰ~ησονται";
    future_ind_pass_1: "~τᾰ~ησομαι", _, "~τᾰ~ησομεθᾰ";
    future_ind_pass_2: "~τᾰ~ησῃ|~τᾰ~ησει", "~τᾰ~ησεσθον", "~τᾰ~ησεσθε";
    future_ind_pass_3: "~τᾰ~ησεται", _, "~τᾰ~ησονται";
    future_opt_act_1: "~τᾰ~ησοιμῐ", _, "~τᾰ~ησοιμεν";
    future_opt_act_2: "~τᾰ~ησοις", "~τᾰ~ησοιτον", "~τᾰ~ησοιτε";
    future_opt_act_3: "~τᾰ~ησοι", "~τᾰ~ησοιτην", "~τᾰ~ησοιεν";
    future_opt_mid_1: "~τᾰ~ησοιμην", _, "~τᾰ~ησοιμεθᾰ";
    future_opt_mid_2: "~τᾰ~ησοιο", "~τᾰ~ησοισθον", "~τᾰ~ησοισθε";
    future_opt_mid_3: "~τᾰ~ησοιτο", "~τᾰ~ησοισθην", "~τᾰ~ησοιντο";
    future_opt_pass_1: "~τᾰ~ησοιμην", _, "~τᾰ~ησοιμεθᾰ";
    future_opt_pass_2: "~τᾰ~ησοιο", "~τᾰ~ησοισθον", "~τᾰ~ησοισθε";
    future_opt_pass_3: "~τᾰ~ησοιτο", "~τᾰ~ησοισθην", "~τᾰ~ησοιντο";
    aorist_subj_act_1: "~τᾰ~ησω", _, "~τᾰ~ησωμεν";
    aorist_subj_act_2: "~τᾰ~ησῃς", "~τᾰ~ησητον", "~τᾰ~ησητε";
    aorist_subj_act_3: "~τᾰ~ησῃ", _, "~τᾰ~ησωσῐ";
    aorist_subj_mid_1: "~τᾰ~ησωμαι", _, "~τᾰ~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~τᾰ~ησησθον", "~τᾰ~ησησθε";
    aorist_subj_mid_3: "~τᾰ~ησηται", _, "~τᾰ~ησωνται";
    aorist_subj_pass_1: "~τᾰ~ᾰθω", _, "~τᾰ~ᾰθωμεν";
    aorist_subj_pass_2: "~τᾰ~ᾰθῃς", "~τᾰ~ᾰθητον", "~τᾰ~ᾰθητε";
    aorist_subj_pass_3: "~τᾰ~ᾰθῃ", _, "~τᾰ~ᾰθωσῐ";
    aorist_opt_act_1: "~τᾰ~ησαιμῐ", _, "~τᾰ~ησαιμεν";
    aorist_opt_act_2: "~τᾰ~ησειᾰς", "~τᾰ~ησαιτον", "~τᾰ~ησαιτε";
    aorist_opt_act_3: "~τᾰ~ησειε", "~τᾰ~ησαιτην", "~τᾰ~ησειᾰν";
    aorist_opt_mid_1: "~τᾰ~ησαιμην", _, "~τᾰ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~τᾰ~ησαιο", "~τᾰ~ησαισθον", "~τᾰ~ησαισθε";
    aorist_opt_mid_3: "~τᾰ~ησαιτο", "~τᾰ~ησαισθην", "~τᾰ~ησαιντο";
    aorist_opt_pass_1: "~τᾰ~ᾰθειην", _, "~τᾰ~ᾰθεῑμεν";
    aorist_opt_pass_2: "~τᾰ~ᾰθειης", "~τᾰ~ᾰθεῑτον", "~τᾰ~ᾰθεῑτε";
    aorist_opt_pass_3: "~τᾰ~ᾰθειη", "~τᾰ~ᾰθειτην", "~τᾰ~ᾰθεῑεν";
    aorist_imp_act_2: "~τᾰ~ησον", "~τᾰ~ησᾰτον", "~τᾰ~ησᾰτε";
    aorist_imp_act_3: "~τᾰ~ησᾰτω", "~τᾰ~ησᾰτων", "~τᾰ~ησᾰντων";
    aorist_imp_mid_2: "~τᾰ~ησαι", "~τᾰ~ησᾰσθον", "~τᾰ~ησᾰσθε";
    aorist_imp_mid_3: "~τᾰ~ησᾰσθω", "~τᾰ~ησᾰσθων", _;
    aorist_imp_pass_2: "~τᾰ~ᾰθητῐ", _, _;
    aorist_imp_pass_3: "~τᾰ~ᾰθητω", "~τᾰ~ᾰθητων", "~τᾰ~ᾰθεντων";
    perfect_ind_act_1: "~θε~ηκᾰ", _, "~θε~ηκᾰμεν";
    perfect_ind_act_2: "~θε~ηκᾰς", "~θε~ηκᾰτον", "~θε~ηκᾰτε";
    perfect_ind_act_3: "~θε~ηκε|~θε~ηκεν", _, "~θε~ηκᾱσῐ|~θε~ηκᾱσῐν";
    perfect_subj_act_1: "~θε~ηκω", _, "~θε~ηκωμεν";
    perfect_subj_act_2: "~θε~ηκῃς", "~θε~ηκητον", "~θε~ηκητε";
    perfect_subj_act_3: "~θε~ηκῃ", _, "~θε~ηκωσῐ|~θε~ηκωσῐν";
    perfect_opt_act_1: "~θε~ηκοιμῐ|~θε~ηκοιην", _, "~θε~ηκοιμεν";
    perfect_opt_act_2: "~θε~ηκοις|~θε~ηκοιης", "~θε~ηκοιτον", "~θε~ηκοιτε";
    perfect_opt_act_3: "~θε~ηκοι|~θε~ηκοιη", "~θε~ηκοιτην", "~θε~ηκοιεν";
    perfect_imp_act_2: "~θε~ηκε", "~θε~ηκετον", "~θε~ηκετε";
    perfect_imp_act_3: "~θε~ηκετω", "~θε~ηκετων", "~θε~ηκοντων";
    infinitive_pres: "~θῐ~ᾰναι", "~θῐ~ᾰσθαι", _;
    infinitive_fut: "~τᾰ~ησειν", "~τᾰ~ησεσθαι", "~τᾰ~ησεσθαι";
    infinitive_aor: "~τᾰ~ησαι", "~τᾰ~ησᾰσθαι", "~τᾰ~ᾰθηναι";
    infinitive_perf: "~θε~ηκεναι", _, _;
    participle_pres: "~θῐ~ᾱς", "~θῐ~ᾰμενος", _;
    participle_fut: "~τᾰ~ησων", "~τᾰ~ησομενος", "~τᾰ~ησομενος";
    participle_aor: "~τᾰ~ησᾱς", "~τᾰ~ησᾰμενος", "~τᾰ~ᾰθεις";
    participle_perf: "~θε~ηκως", _, _;
    ),
    // Ἐκφερω: 3 words (ἐκφερω, ἐκγᾰμῐζω, ὑπεκφευγω). Like Ἐγκειμαι, except in 70 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐκφερω", 3, "~κ~ω", like Ἐγκειμαι as MID_AS_PASS;
    ind_act_1: "~κ~ω", _, "~κ~ομεν";
    ind_act_2: "~κ~εις", "~κ~ετον", "~κ~ετε";
    ind_act_3: "~κ~ει", _, "~κ~ουσῐ";
    ind_mid_1: "~κ~ομαι", _, "~κ~ομεθᾰ";
    ind_mid_2: "~κ~ῃ", "~κ~εσθον", "~κ~εσθε";
    ind_mid_3: "~κ~εται", _, "~κ~ονται";
    subj_act_1: "~κ~ω", _, "~κ~ωμεν";
    subj_act_2: "~κ~ῃς", "~κ~ητον", "~κ~ητε";
    subj_act_3: "~κ~ῃ", _, "~κ~ωσῐ";
    subj_mid_1: "~κ~ωμαι", _, "~κ~ωμεθᾰ";
    subj_mid_2: _, "~κ~ησθον", "~κ~ησθε";
    subj_mid_3: "~κ~ηται", _, "~κ~ωνται";
    opt_act_1: "~κ~οιμῐ", _, "~κ~οιμεν";
    opt_act_2: "~κ~οις", "~κ~οιτον", "~κ~οιτε";
    opt_act_3: "~κ~οι", "~κ~οιτην", "~κ~οιεν";
    opt_mid_1: "~κ~οιμην", _, "~κ~οιμεθᾰ";
    opt_mid_2: "~κ~οιο", "~κ~οισθον", "~κ~οισθε";
    opt_mid_3: "~κ~οιτο", "~κ~οισθην", "~κ~οιντο";
    imp_act_2: "~κ~ε", _, _;
    imp_act_3: "~κ~ετω", "~κ~ετων", "~κ~οντων";
    imp_mid_2: "~κ~ου", _, _;
    imp_mid_3: "~κ~εσθω", "~κ~εσθων", _;
    imperfect_act_1: "~ξε~ον", _, "~ξε~ομεν";
    imperfect_act_2: "~ξε~ες", "~ξε~ετον", "~ξε~ετε";
    imperfect_act_3: "~ξε~ε", "~ξε~ετην", "~ξε~ον";
    imperfect_mid_1: "~ξε~ομην", _, "~ξε~ομεθᾰ";
    imperfect_mid_2: "~ξε~ου", "~ξε~εσθον", "~ξε~εσθε";
    imperfect_mid_3: "~ξε~ετο", "~ξε~εσθην", "~ξε~οντο";
    infinitive_pres: "~κ~ειν", "~κ~εσθαι", _;
    participle_pres: "~κ~ων", "~κ~ομενος", _;
    ),
    // Ἐξετᾰζω: 3 words (προετοιμᾰζω, ἐξετᾰζω, ἀντελπῐζω). Like Δῐᾰρπᾰζω, except in 170 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐξετᾰζω", 3, "~ε~ζω", like Δῐᾰρπᾰζω;
    ind_act_1: "~ε~ζω", _, "~ε~ζομεν";
    ind_act_2: "~ε~ζεις", "~ε~ζετον", "~ε~ζετε";
    ind_act_3: "~ε~ζει", _, "~ε~ζουσῐ|~ε~ζουσῐν";
    ind_mid_1: "~ε~ζομαι", _, "~ε~ζομεθᾰ";
    ind_mid_2: "~ε~ζῃ|~ε~ζει", "~ε~ζεσθον", "~ε~ζεσθε";
    ind_mid_3: "~ε~ζεται", _, "~ε~ζονται";
    subj_act_1: "~ε~ζω", _, "~ε~ζωμεν";
    subj_act_2: "~ε~ζῃς", "~ε~ζητον", "~ε~ζητε";
    subj_act_3: "~ε~ζῃ", _, "~ε~ζωσῐ|~ε~ζωσῐν";
    subj_mid_1: "~ε~ζωμαι", _, "~ε~ζωμεθᾰ";
    subj_mid_2: _, "~ε~ζησθον", "~ε~ζησθε";
    subj_mid_3: "~ε~ζηται", _, "~ε~ζωνται";
    opt_act_1: "~ε~ζοιμῐ", _, "~ε~ζοιμεν";
    opt_act_2: "~ε~ζοις", "~ε~ζοιτον", "~ε~ζοιτε";
    opt_act_3: "~ε~ζοι", "~ε~ζοιτην", "~ε~ζοιεν";
    opt_mid_1: "~ε~ζοιμην", _, "~ε~ζοιμεθᾰ";
    opt_mid_2: "~ε~ζοιο", "~ε~ζοισθον", "~ε~ζοισθε";
    opt_mid_3: "~ε~ζοιτο", "~ε~ζοισθην", "~ε~ζοιντο";
    imp_act_2: "~ε~ζε", _, _;
    imp_act_3: "~ε~ζετω", "~ε~ζετων", "~ε~ζοντων";
    imp_mid_2: "~ε~ζου", _, _;
    imp_mid_3: "~ε~ζεσθω", "~ε~ζεσθων", _;
    future_ind_act_1: "~ε~ω", _, "~ε~οῡμεν";
    future_ind_act_2: "~ε~εῑς", "~ε~εῑτον", "~ε~εῑτε";
    future_ind_act_3: "~ε~εῑ", _, "~ε~οῡσῐ";
    future_ind_mid_1: "~ε~οῡμαι", _, "~ε~ουμεθᾰ";
    future_ind_mid_2: "~ε~εῑ", "~ε~εῑσθον", "~ε~εῑσθε";
    future_ind_mid_3: "~ε~εῑται", _, "~ε~οῡνται";
    future_ind_pass_1: "~ε~σθησομαι", _, "~ε~σθησομεθᾰ";
    future_ind_pass_2: "~ε~σθησῃ|~ε~σθησει", "~ε~σθησεσθον", "~ε~σθησεσθε";
    future_ind_pass_3: "~ε~σθησεται", _, "~ε~σθησονται";
    future_opt_act_1: "~ε~οιην", _, "~ε~οῑμεν";
    future_opt_act_2: "~ε~οιης", "~ε~οῑτον", "~ε~οῑτε";
    future_opt_act_3: "~ε~οιη", "~ε~οιτην", "~ε~οῑεν";
    future_opt_mid_1: "~ε~οιμην", _, "~ε~οιμεθᾰ";
    future_opt_mid_2: "~ε~οῑο", "~ε~οῑσθον", "~ε~οῑσθε";
    future_opt_mid_3: "~ε~οῑτο", "~ε~οισθην", "~ε~οῑντο";
    future_opt_pass_1: "~ε~σθησοιμην", _, "~ε~σθησοιμεθᾰ";
    future_opt_pass_2: "~ε~σθησοιο", "~ε~σθησοισθον", "~ε~σθησοισθε";
    future_opt_pass_3: "~ε~σθησοιτο", "~ε~σθησοισθην", "~ε~σθησοιντο";
    aorist_subj_act_1: "~ε~σω", _, "~ε~σωμεν";
    aorist_subj_act_2: "~ε~σῃς", "~ε~σητον", "~ε~σητε";
    aorist_subj_act_3: "~ε~σῃ", _, "~ε~σωσῐ|~ε~σωσῐν";
    aorist_subj_mid_1: "~ε~σωμαι", _, "~ε~σωμεθᾰ";
    aorist_subj_mid_2: _, "~ε~σησθον", "~ε~σησθε";
    aorist_subj_mid_3: "~ε~σηται", _, "~ε~σωνται";
    aorist_subj_pass_1: "~ε~σθω", _, "~ε~σθωμεν";
    aorist_subj_pass_2: "~ε~σθῃς", "~ε~σθητον", "~ε~σθητε";
    aorist_subj_pass_3: "~ε~σθῃ", _, "~ε~σθωσῐ|~ε~σθωσῐν";
    aorist_opt_act_1: "~ε~σαιμῐ", _, "~ε~σαιμεν";
    aorist_opt_act_2: "~ε~σειᾰς|~ε~σαις", "~ε~σαιτον", "~ε~σαιτε";
    aorist_opt_act_3: "~ε~σειε|~ε~σειεν|~ε~σαι", "~ε~σαιτην", "~ε~σειᾰν|~ε~σαιεν";
    aorist_opt_mid_1: "~ε~σαιμην", _, "~ε~σαιμεθᾰ";
    aorist_opt_mid_2: "~ε~σαιο", "~ε~σαισθον", "~ε~σαισθε";
    aorist_opt_mid_3: "~ε~σαιτο", "~ε~σαισθην", "~ε~σαιντο";
    aorist_opt_pass_1: "~ε~σθειην", _, "~ε~σθεῑμεν|~ε~σθειημεν";
    aorist_opt_pass_2: "~ε~σθειης", "~ε~σθεῑτον|~ε~σθειητον", "~ε~σθεῑτε|~ε~σθειητε";
    aorist_opt_pass_3: "~ε~σθειη", "~ε~σθειτην|~ε~σθειητην", "~ε~σθεῑεν|~ε~σθειησᾰν";
    aorist_imp_act_2: "~ε~σον", "~ε~σᾰτον", "~ε~σᾰτε";
    aorist_imp_act_3: "~ε~σᾰτω", "~ε~σᾰτων", "~ε~σᾰντων";
    aorist_imp_mid_2: "~ε~σαι", "~ε~σᾰσθον", "~ε~σᾰσθε";
    aorist_imp_mid_3: "~ε~σᾰσθω", "~ε~σᾰσθων", _;
    aorist_imp_pass_2: "~ε~σθητῐ", _, _;
    aorist_imp_pass_3: "~ε~σθητω", "~ε~σθητων", "~ε~σθεντων";
    infinitive_pres: "~ε~ζειν", "~ε~ζεσθαι", _;
    infinitive_fut: "~ε~εῑν", "~ε~εῑσθαι", "~ε~σθησεσθαι";
    infinitive_aor: "~ε~σαι", "~ε~σᾰσθαι", "~ε~σθηναι";
    participle_pres: "~ε~ζων", "~ε~ζομενος", _;
    participle_fut: "~ε~ων", "~ε~ουμενος", "~ε~σθησομενος";
    participle_aor: "~ε~σᾱς", "~ε~σᾰμενος", "~ε~σθεις";
    ),
    // Σῠναιρεω: 3 words (κᾰθαιρεω, σῠναιρεω, ξῠναιρεω). Like Ἀναιρεω, except in 97 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("σῠναιρεω", 3, "~αιρεω", like Ἀναιρεω as AORIST_AS_FUTURE;
    stem aorist_ind: "~ειλ- like πᾰρεχω";
    future_ind_act_1: "~ελω", _, "~ελοῡμεν";
    future_ind_act_2: "~ελεῑς", "~ελεῑτον", "~ελεῑτε";
    future_ind_act_3: "~ελεῑ", _, "~ελοῡσῐ|~ελοῡσῐν";
    future_ind_mid_1: "~ελοῡμαι", _, "~ελουμεθᾰ";
    future_ind_mid_2: "~ελεῑ|~ελῃ", "~ελεῑσθον", "~ελεῑσθε";
    future_ind_mid_3: "~ελεῑται", _, "~ελοῡνται";
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_act_1: "~ελοιην|~ελοῑμῐ", _, "~ελοῑμεν|~ελοιημεν";
    future_opt_act_2: "~ελοιης|~ελοῑς", "~ελοῑτον|~ελοιητον", "~ελοῑτε|~ελοιητε";
    future_opt_act_3: "~ελοιη|~ελοῑ", "~ελοιτην|~ελοιητην", "~ελοῑεν|~ελοιησᾰν";
    future_opt_mid_1: "~ελοιμην", _, "~ελοιμεθᾰ";
    future_opt_mid_2: "~ελοῑο", "~ελοῑσθον", "~ελοῑσθε";
    future_opt_mid_3: "~ελοῑτο", "~ελοισθην", "~ελοῑντο";
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_subj_act_1: "~ελω", _, "~ελωμεν";
    aorist_subj_act_2: "~ελῃς", "~ελητον", "~ελητε";
    aorist_subj_act_3: "~ελῃ", _, "~ελωσῐ|~ελωσῐν";
    aorist_subj_mid_1: "~ελωμαι", _, "~ελωμεθᾰ";
    aorist_subj_mid_2: _, "~ελησθον", "~ελησθε";
    aorist_subj_mid_3: "~εληται", _, "~ελωνται";
    aorist_opt_act_1: "~ελοιμῐ", _, "~ελοιμεν";
    aorist_opt_act_2: "~ελοις", "~ελοιτον", "~ελοιτε";
    aorist_opt_act_3: "~ελοι", "~ελοιτην", "~ελοιεν";
    aorist_opt_mid_1: "~ελοιμην", _, "~ελοιμεθᾰ";
    aorist_opt_mid_2: "~ελοιο", "~ελοισθον", "~ελοισθε";
    aorist_opt_mid_3: "~ελοιτο", "~ελοισθην", "~ελοιντο";
    aorist_imp_act_2: "~ελε", "~ελετον", "~ελετε";
    aorist_imp_act_3: "~ελετω", "~ελετων", "~ελοντων";
    aorist_imp_mid_2: "~ελοῡ", "~ελεσθον", "~ελεσθε";
    aorist_imp_mid_3: "~ελεσθω", "~ελεσθων", _;
    infinitive_fut: "~ελεῑν", "~ελεῑσθαι", derived;
    infinitive_aor: "~ελεῑν", "~ελεσθαι", derived;
    participle_fut: "~ελων", "~ελουμενος", derived;
    participle_aor: "~ελων", "~ελομενος", derived;
    ),
    // Φῡρω: 3 words (σῡρω, τῡφω, φῡρω). Like Ποιμαινω, except in 181 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("φῡρω", 3, "~ῡ~ω", like Ποιμαινω;
    stem aorist_ind: "~ῠ~σ- like ἐχω";
    stem aorist_ind_pass: "~ῠ~θη- like ἐχω";
    ind_act_1: "~ῡ~ω", _, "~ῡ~ομεν";
    ind_act_2: "~ῡ~εις", "~ῡ~ετον", "~ῡ~ετε";
    ind_act_3: "~ῡ~ει", _, "~ῡ~ουσῐ|~ῡ~ουσῐν";
    ind_mid_1: "~ῡ~ομαι", _, "~ῡ~ομεθᾰ";
    ind_mid_2: "~ῡ~ῃ|~ῡ~ει", "~ῡ~εσθον", "~ῡ~εσθε";
    ind_mid_3: "~ῡ~εται", _, "~ῡ~ονται";
    subj_act_1: "~ῡ~ω", _, "~ῡ~ωμεν";
    subj_act_2: "~ῡ~ῃς", "~ῡ~ητον", "~ῡ~ητε";
    subj_act_3: "~ῡ~ῃ", _, "~ῡ~ωσῐ|~ῡ~ωσῐν";
    subj_mid_1: "~ῡ~ωμαι", _, "~ῡ~ωμεθᾰ";
    subj_mid_2: _, "~ῡ~ησθον", "~ῡ~ησθε";
    subj_mid_3: "~ῡ~ηται", _, "~ῡ~ωνται";
    opt_act_1: "~ῡ~οιμῐ", _, "~ῡ~οιμεν";
    opt_act_2: "~ῡ~οις", "~ῡ~οιτον", "~ῡ~οιτε";
    opt_act_3: "~ῡ~οι", "~ῡ~οιτην", "~ῡ~οιεν";
    opt_mid_1: "~ῡ~οιμην", _, "~ῡ~οιμεθᾰ";
    opt_mid_2: "~ῡ~οιο", "~ῡ~οισθον", "~ῡ~οισθε";
    opt_mid_3: "~ῡ~οιτο", "~ῡ~οισθην", "~ῡ~οιντο";
    imp_act_2: "~ῡ~ε", _, _;
    imp_act_3: "~ῡ~ετω", "~ῡ~ετων", "~ῡ~οντων";
    imp_mid_2: "~ῡ~ου", _, _;
    imp_mid_3: "~ῡ~εσθω", "~ῡ~εσθων", _;
    imperfect_act_1: "ἐ~ῡ~ον", _, "ἐ~ῡ~ομεν";
    imperfect_act_2: "ἐ~ῡ~ες", "ἐ~ῡ~ετον", "ἐ~ῡ~ετε";
    imperfect_act_3: "ἐ~ῡ~ε|ἐ~ῡ~εν", "ἐ~ῡ~ετην", "ἐ~ῡ~ον";
    imperfect_mid_1: "ἐ~ῡ~ομην", _, "ἐ~ῡ~ομεθᾰ";
    imperfect_mid_2: "ἐ~ῡ~ου", "ἐ~ῡ~εσθον", "ἐ~ῡ~εσθε";
    imperfect_mid_3: "ἐ~ῡ~ετο", "ἐ~ῡ~εσθην", "ἐ~ῡ~οντο";
    future_ind_act_1: "~ῠ~σω", _, "~ῠ~σομεν";
    future_ind_act_2: "~ῠ~σεις", "~ῠ~σετον", "~ῠ~σετε";
    future_ind_act_3: "~ῠ~σει", _, "~ῠ~σουσῐ";
    future_ind_mid_1: "~ῠ~σομαι", _, "~ῠ~σομεθᾰ";
    future_ind_mid_2: "~ῠ~σῃ", "~ῠ~σεσθον", "~ῠ~σεσθε";
    future_ind_mid_3: "~ῠ~σεται", _, "~ῠ~σονται";
    future_opt_act_1: "~ῠ~σοιμῐ", _, "~ῠ~σοιμεν";
    future_opt_act_2: "~ῠ~σοις", "~ῠ~σοιτον", "~ῠ~σοιτε";
    future_opt_act_3: "~ῠ~σοι", "~ῠ~σοιτην", "~ῠ~σοιεν";
    future_opt_mid_1: "~ῠ~σοιμην", _, "~ῠ~σοιμεθᾰ";
    future_opt_mid_2: "~ῠ~σοιο", "~ῠ~σοισθον", "~ῠ~σοισθε";
    future_opt_mid_3: "~ῠ~σοιτο", "~ῠ~σοισθην", "~ῠ~σοιντο";
    aorist_subj_act_1: "~ῠ~σω", _, "~ῠ~σωμεν";
    aorist_subj_act_2: "~ῠ~σῃς", "~ῠ~σητον", "~ῠ~σητε";
    aorist_subj_act_3: "~ῠ~σῃ", _, "~ῠ~σωσῐ";
    aorist_subj_mid_1: "~ῠ~σωμαι", _, "~ῠ~σωμεθᾰ";
    aorist_subj_mid_2: _, "~ῠ~σησθον", "~ῠ~σησθε";
    aorist_subj_mid_3: "~ῠ~σηται", _, "~ῠ~σωνται";
    aorist_subj_pass_1: "~ῠ~θω", _, "~ῠ~θωμεν";
    aorist_subj_pass_2: "~ῠ~θῃς", "~ῠ~θητον", "~ῠ~θητε";
    aorist_subj_pass_3: "~ῠ~θῃ", _, "~ῠ~θωσῐ";
    aorist_opt_act_1: "~ῠ~σαιμῐ", _, "~ῠ~σαιμεν";
    aorist_opt_act_2: "~ῠ~σειᾰς", "~ῠ~σαιτον", "~ῠ~σαιτε";
    aorist_opt_act_3: "~ῠ~σειε", "~ῠ~σαιτην", "~ῠ~σειᾰν";
    aorist_opt_mid_1: "~ῠ~σαιμην", _, "~ῠ~σαιμεθᾰ";
    aorist_opt_mid_2: "~ῠ~σαιο", "~ῠ~σαισθον", "~ῠ~σαισθε";
    aorist_opt_mid_3: "~ῠ~σαιτο", "~ῠ~σαισθην", "~ῠ~σαιντο";
    aorist_opt_pass_1: "~ῠ~θειην", _, "~ῠ~θεῑμεν";
    aorist_opt_pass_2: "~ῠ~θειης", "~ῠ~θεῑτον", "~ῠ~θεῑτε";
    aorist_opt_pass_3: "~ῠ~θειη", "~ῠ~θειτην", "~ῠ~θεῑεν";
    aorist_imp_act_2: "~ῠ~σον", "~ῠ~σᾰτον", "~ῠ~σᾰτε";
    aorist_imp_act_3: "~ῠ~σᾰτω", "~ῠ~σᾰτων", "~ῠ~σᾰντων";
    aorist_imp_mid_2: "~ῠ~σαι", "~ῠ~σᾰσθον", "~ῠ~σᾰσθε";
    aorist_imp_mid_3: "~ῠ~σᾰσθω", "~ῠ~σᾰσθων", _;
    aorist_imp_pass_2: "~ῠ~θητῐ", _, _;
    aorist_imp_pass_3: "~ῠ~θητω", "~ῠ~θητων", "~ῠ~θεντων";
    perfect_ind_mid_1: "%ε~ῠ~μαι", _, "%ε~ῠ~μεθᾰ";
    perfect_ind_mid_2: "%ε~ῠ~σαι", "%ε~ῠ~θον", "%ε~ῠ~θε";
    perfect_ind_mid_3: "%ε~ῠ~ται", _, "%ε~ῠ~ᾰται";
    perfect_imp_mid_2: "%ε~ῠ~σο", _, _;
    perfect_imp_mid_3: "%ε~ῠ~θω", "%ε~ῠ~θων", _;
    infinitive_pres: "~ῡ~ειν", "~ῡ~εσθαι", _;
    infinitive_fut: "~ῠ~σειν", "~ῠ~σεσθαι", _;
    infinitive_aor: "~ῠ~σαι", "~ῠ~σᾰσθαι", "~ῠ~θηναι";
    infinitive_perf: _, "%ε~ῠ~θαι", _;
    participle_pres: "~ῡ~ων", "~ῡ~ομενος", _;
    participle_fut: "~ῠ~σων", "~ῠ~σομενος", _;
    participle_aor: "~ῠ~σᾱς", "~ῠ~σᾰμενος", "~ῠ~θεις";
    participle_perf: _, "%ε~ῠ~μενος", _;
    ),
    // Σταθμαω: 3 words (χαλαω, καθοραω, σταθμαω). Like Δημοκρᾰτεομαι, except in 163 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("σταθμαω", 3, "~α~αω", like Δημοκρᾰτεομαι as PERFECT_AS_FUTURE;
    stem aorist_ind_pass: "~ᾰ~ησθη- like ἐχω";
    ind_act_1: _, _, "~ᾰ~ωμεν";
    ind_act_2: "~ᾰ~ᾳς", "~ᾰ~ᾱτον", "~ᾰ~ᾱτε";
    ind_act_3: "~ᾰ~ᾳ", _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    ind_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    ind_mid_2: "~ᾰ~ᾳ", "~ᾰ~ᾱσθον", "~ᾰ~ᾱσθε";
    ind_mid_3: "~ᾰ~ᾱται", _, "~ᾰ~ωνται";
    subj_act_2: "~ᾰ~ᾳς", "~ᾰ~ᾱτον", "~ᾰ~ᾱτε";
    subj_act_3: "~ᾰ~ᾳ", _, _;
    subj_mid_2: _, "~ᾰ~ᾱσθον", "~ᾰ~ᾱσθε";
    subj_mid_3: "~ᾰ~ᾱται", _, _;
    opt_act_1: "~ᾰ~ῳην|~ᾰ~ῳμῐ", _, "~ᾰ~ῳμεν|~ᾰ~ῳημεν";
    opt_act_2: "~ᾰ~ῳης|~ᾰ~ῳς", "~ᾰ~ῳτον|~ᾰ~ῳητον", "~ᾰ~ῳτε|~ᾰ~ῳητε";
    opt_act_3: "~ᾰ~ῳη|~ᾰ~ῳ", "~ᾰ~ῳτην|~ᾰ~ῳητην", "~ᾰ~ῳεν|~ᾰ~ῳησᾰν";
    opt_mid_1: "~ᾰ~ῳμην", _, "~ᾰ~ῳμεθᾰ";
    opt_mid_2: "~ᾰ~ῳο", "~ᾰ~ῳσθον", "~ᾰ~ῳσθε";
    opt_mid_3: "~ᾰ~ῳτο", "~ᾰ~ῳσθην", "~ᾰ~ῳντο";
    imp_act_2: "~ᾰ~ᾱ", _, _;
    imp_act_3: "~ᾰ~ᾱτω", "~ᾰ~ᾱτων", "~ᾰ~ωντων";
    imp_mid_2: "~ᾰ~ω", _, _;
    imp_mid_3: "~ᾰ~ᾱσθω", "~ᾰ~ᾱσθων", _;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, derived;
    imperfect_mid_3: derived, derived, derived;
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ";
    future_ind_mid_2: "~ᾰ~ησῃ", _, _;
    future_ind_pass_1: "~ᾰ~ησθησομαι", _, "~ᾰ~ησθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ησθησῃ", "~ᾰ~ησθησεσθον", "~ᾰ~ησθησεσθε";
    future_ind_pass_3: "~ᾰ~ησθησεται", _, "~ᾰ~ησθησονται";
    future_opt_pass_1: "~ᾰ~ησθησοιμην", _, "~ᾰ~ησθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ησθησοιο", "~ᾰ~ησθησοισθον", "~ᾰ~ησθησοισθε";
    future_opt_pass_3: "~ᾰ~ησθησοιτο", "~ᾰ~ησθησοισθην", "~ᾰ~ησθησοιντο";
    aorist_subj_act_3: _, _, "~ᾰ~ησωσῐ";
    aorist_subj_pass_1: "~ᾰ~ησθω", _, "~ᾰ~ησθωμεν";
    aorist_subj_pass_2: "~ᾰ~ησθῃς", "~ᾰ~ησθητον", "~ᾰ~ησθητε";
    aorist_subj_pass_3: "~ᾰ~ησθῃ", _, "~ᾰ~ησθωσῐ";
    aorist_opt_act_2: "~ᾰ~ησειᾰς", _, _;
    aorist_opt_act_3: "~ᾰ~ησειε", _, "~ᾰ~ησειᾰν";
    aorist_opt_pass_1: "~ᾰ~ησθειην", _, "~ᾰ~ησθεῑμεν";
    aorist_opt_pass_2: "~ᾰ~ησθειης", "~ᾰ~ησθεῑτον", "~ᾰ~ησθεῑτε";
    aorist_opt_pass_3: "~ᾰ~ησθειη", "~ᾰ~ησθειτην", "~ᾰ~ησθεῑεν";
    aorist_imp_pass_2: "~ᾰ~ησθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ησθητω", "~ᾰ~ησθητων", "~ᾰ~ησθεντων";
    perfect_ind_act_1: "ἐ~ᾰ~ηκᾰ", _, "ἐ~ᾰ~ηκᾰμεν";
    perfect_ind_act_2: "ἐ~ᾰ~ηκᾰς", "ἐ~ᾰ~ηκᾰτον", "ἐ~ᾰ~ηκᾰτε";
    perfect_ind_act_3: "ἐ~ᾰ~ηκε", _, "ἐ~ᾰ~ηκᾱσῐ";
    perfect_ind_mid_1: "ἐ~ᾰ~ημαι", _, "ἐ~ᾰ~ημεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰ~ησαι", "ἐ~ᾰ~ησθον", "ἐ~ᾰ~ησθε";
    perfect_ind_mid_3: "ἐ~ᾰ~ηται", _, "ἐ~ᾰ~ηνται";
    perfect_subj_act_1: "ἐ~ᾰ~ηκω", _, "ἐ~ᾰ~ηκωμεν";
    perfect_subj_act_2: "ἐ~ᾰ~ηκῃς", "ἐ~ᾰ~ηκητον", "ἐ~ᾰ~ηκητε";
    perfect_subj_act_3: "ἐ~ᾰ~ηκῃ", _, "ἐ~ᾰ~ηκωσῐ";
    perfect_opt_act_1: "ἐ~ᾰ~ηκοιμῐ", _, "ἐ~ᾰ~ηκοιμεν";
    perfect_opt_act_2: "ἐ~ᾰ~ηκοις", "ἐ~ᾰ~ηκοιτον", "ἐ~ᾰ~ηκοιτε";
    perfect_opt_act_3: "ἐ~ᾰ~ηκοι", "ἐ~ᾰ~ηκοιτην", "ἐ~ᾰ~ηκοιεν";
    perfect_imp_act_2: _, "ἐ~ᾰ~ηκετον", "ἐ~ᾰ~ηκετε";
    perfect_imp_act_3: "ἐ~ᾰ~ηκετω", "ἐ~ᾰ~ηκετων", "ἐ~ᾰ~ηκοντων";
    perfect_imp_mid_2: "ἐ~ᾰ~ησο", _, _;
    perfect_imp_mid_3: "ἐ~ᾰ~ησθω", "ἐ~ᾰ~ησθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_pres: "~ᾰ~ᾱν", "~ᾰ~ᾱσθαι", _;
    infinitive_fut: _, _, "~ᾰ~ησθησεσθαι";
    infinitive_aor: _, _, "~ᾰ~ησθηναι";
    infinitive_perf: "ἐ~ᾰ~ηκεναι", "ἐ~ᾰ~ησθαι", _;
    participle_pres: _, "~ᾰ~ωμενος", _;
    participle_fut: _, _, "~ᾰ~ησθησομενος";
    participle_aor: _, _, "~ᾰ~ησθεις";
    participle_perf: "ἐ~ᾰ~ηκως", "ἐ~ᾰ~ημενος", _;
    ),
    // Κνᾰω: 3 words (βοᾰω, τρῠγᾰω, κνᾰω). Like Χραω, except in 70 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("κνᾰω", 3, "~ᾰω", like Χραω;
    stem pluperfect_act: "~ηκ- like χρημᾰτῐζω";
    ind_act_1: "~ᾰω|~ω", _, "~ᾰομεν|~ωμεν";
    ind_act_2: "~ᾰεις|~ᾳς", "~ᾰετον|~ᾱτον", "~ᾰετε|~ᾱτε";
    ind_act_3: "~ᾰει|~ᾳ", _, "~ᾰουσῐ|~ᾰουσῐν|~ωσῐ|~ωσῐν";
    ind_mid_1: "~ᾰομαι|~ωμαι", _, "~ᾰομεθᾰ|~ωμεθᾰ";
    ind_mid_2: "~ᾰῃ|~ᾰει|~ᾳ", "~ᾰεσθον|~ᾱσθον", "~ᾰεσθε|~ᾱσθε";
    ind_mid_3: "~ᾰεται|~ᾱται", _, "~ᾰονται|~ωνται";
    subj_act_1: "~ᾰω|~ω", _, "~ᾰωμεν|~ωμεν";
    subj_act_2: "~ᾰῃς|~ᾳς", "~ᾰητον|~ᾱτον", "~ᾰητε|~ᾱτε";
    subj_act_3: "~ᾰῃ|~ᾳ", _, "~ᾰωσῐ|~ᾰωσῐν|~ωσῐ|~ωσῐν";
    subj_mid_1: "~ᾰωμαι|~ωμαι", _, "~ᾰωμεθᾰ|~ωμεθᾰ";
    subj_mid_2: _, "~ᾰησθον|~ᾱσθον", "~ᾰησθε|~ᾱσθε";
    subj_mid_3: "~ᾰηται|~ᾱται", _, "~ᾰωνται|~ωνται";
    opt_act_1: "~ᾰοιμῐ|~ῳην|~ῳμῐ", _, "~ᾰοιμεν|~ῳμεν|~ῳημεν";
    opt_act_2: "~ᾰοις|~ῳης|~ῳς", "~ᾰοιτον|~ῳτον|~ῳητον", "~ᾰοιτε|~ῳτε|~ῳητε";
    opt_act_3: "~ᾰοι|~ῳη|~ῳ", "~ᾰοιτην|~ῳτην|~ῳητην", "~ᾰοιεν|~ῳεν|~ῳησᾰν";
    opt_mid_1: "~ᾰοιμην|~ῳμην", _, "~ᾰοιμεθᾰ|~ῳμεθᾰ";
    opt_mid_2: "~ᾰοιο|~ῳο", "~ᾰοισθον|~ῳσθον", "~ᾰοισθε|~ῳσθε";
    opt_mid_3: "~ᾰοιτο|~ῳτο", "~ᾰοισθην|~ῳσθην", "~ᾰοιντο|~ῳντο";
    imp_act_2: "~ᾰε|~ᾱ", _, _;
    imp_act_3: "~ᾰετω|~ᾱτω", "~ᾰετων|~ᾱτων", "~ᾰοντων|~ωντων";
    imp_mid_2: "~ᾰου|~ω", _, _;
    imp_mid_3: "~ᾰεσθω|~ᾱσθω", "~ᾰεσθων|~ᾱσθων", _;
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    future_ind_mid_2: "~ησῃ|~ησει", _, _;
    future_ind_pass_2: "~ησθησῃ|~ησθησει", _, _;
    aorist_subj_pass_3: _, _, "~ησθωσῐ|~ησθωσῐν";
    aorist_opt_pass_1: _, _, "~ησθεῑμεν|~ησθειημεν";
    aorist_opt_pass_2: _, "~ησθεῑτον|~ησθειητον", "~ησθεῑτε|~ησθειητε";
    aorist_opt_pass_3: _, "~ησθειτην|~ησθειητην", "~ησθεῑεν|~ησθειησᾰν";
    perfect_ind_act_3: "%ε~ηκε|%ε~ηκεν", _, "%ε~ηκᾱσῐ|%ε~ηκᾱσῐν";
    perfect_subj_act_3: _, _, "%ε~ηκωσῐ|%ε~ηκωσῐν";
    perfect_opt_act_1: "%ε~ηκοιμῐ|%ε~ηκοιην", _, _;
    perfect_opt_act_2: "%ε~ηκοις|%ε~ηκοιης", _, _;
    perfect_opt_act_3: "%ε~ηκοι|%ε~ηκοιη", _, _;
    perfect_imp_act_2: "%ε~ηκε", _, _;
    infinitive_pres: "~ᾰειν|~ᾱν", "~ᾰεσθαι|~ᾱσθαι", _;
    participle_pres: "~ᾰων|~ων", "~ᾰομενος|~ωμενος", _;
    ),
    // Δηλεομαι: 3 words (δηλεομαι, μῑμεομαι, δῐηγεομαι). Like Ἡγεομαι, except in 23 cells: ind_mid_2 sg, ind_pass_2 sg, imperfect_mid_1 sg, imperfect_mid_1 pl, imperfect_mid_2 sg, imperfect_mid_2 du, ….
    conjugation!("δηλεομαι", 3, "~εομαι", like Ἡγεομαι;
    stem aorist_ind: "~ησ- like πᾰραιτεομαι";
    stem aorist_ind_pass: "~ηθη- like ἐχω";
    stem pluperfect: "~η- like πειρᾰω";
    ind_mid_2: "~εῑ|~ῃ", _, _;
    ind_pass_2: "~εῑ", _, _;
    imperfect_mid_1: "ἐ~ουμην", _, "ἐ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~οῡ", "ἐ~εῑσθον", "ἐ~εῑσθε";
    imperfect_mid_3: "ἐ~εῑτο", "ἐ~εισθην", "ἐ~οῡντο";
    future_ind_mid_2: "~ησῃ|~ησει", _, _;
    perfect_ind_mid_1: "%ε~ημαι", _, "%ε~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ησαι", "%ε~ησθον", "%ε~ησθε";
    perfect_ind_mid_3: "%ε~ηται", _, "%ε~ηνται";
    perfect_imp_mid_2: "%ε~ησο", _, _;
    perfect_imp_mid_3: "%ε~ησθω", "%ε~ησθων", _;
    infinitive_perf: _, "%ε~ησθαι", _;
    participle_perf: _, "%ε~ημενος", _;
    ),
    // Ζεω: 3 words (ζεω, πταιω, ξεω). Like Κᾰλεω, except in 70 cells: ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_mid_2 sg, imp_act_2 sg, ….
    conjugation!("ζεω", 3, "~ω", like Κᾰλεω;
    stem aorist_ind_pass: "~σθη- like ἐχω";
    ind_act_2: "~ῑς", "~ῑτον", "~ῑτε";
    ind_act_3: "~ῑ", _, _;
    ind_mid_2: "~ῃ|~ει", _, _;
    imp_act_2: "~ῑ", _, _;
    imp_act_3: "~ιτω", "~ιτων", _;
    imperfect_act_2: "ἐ~ις", "ἐ~ῑτον", _;
    imperfect_act_3: "ἐ~ι", "ἐ~ιτην", "ἐ~ῑτε";
    future_ind_act_3: _, _, "~σουσῐ|~σουσῐν";
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_subj_pass_1: "~σθω", _, "~σθωμεν";
    aorist_subj_pass_2: "~σθῃς", "~σθητον", "~σθητε";
    aorist_subj_pass_3: "~σθῃ", _, "~σθωσῐ|~σθωσῐν";
    aorist_opt_pass_1: "~σθειην", _, "~σθεῑμεν|~σθειημεν";
    aorist_opt_pass_2: "~σθειης", "~σθεῑτον|~σθειητον", "~σθεῑτε|~σθειητε";
    aorist_opt_pass_3: "~σθειη", "~σθειτην|~σθειητην", "~σθεῑεν|~σθειησᾰν";
    aorist_imp_pass_2: "~σθητῐ", _, _;
    aorist_imp_pass_3: "~σθητω", "~σθητων", "~σθεντων";
    perfect_ind_mid_1: "ἐ~σμαι", _, "ἐ~σμεθᾰ";
    perfect_ind_mid_3: "ἐ~σται", _, "ἐ~σᾰται";
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: "ἐ~σμην", _, "ἐ~σμεθᾰ";
    pluperfect_mid_3: "ἐ~στο", _, "ἐ~σᾰτο";
    infinitive_pres: "~ῑν", _, _;
    infinitive_fut: _, _, derived;
    infinitive_aor: _, _, "~σθηναι";
    participle_fut: _, _, derived;
    participle_aor: _, _, "~σθεις";
    participle_perf: _, "ἐ~σμενος", _;
    ),
    // Ὑπᾰκουω: 3 words (ὑπᾰκουω, ἐπᾰκουω, εἰσᾰκουω). Like Ξαινω, except in 216 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ὑπᾰκουω", 3, "~ᾰ~υω", like Ξαινω;
    stem aorist_ind: "~η~υσ- like εὐλογεω";
    stem aorist_ind_pass: "~η~υσθη- like ἐλπω";
    ind_act_1: "~ᾰ~υω", _, "~ᾰ~υομεν";
    ind_act_2: "~ᾰ~υεις", "~ᾰ~υετον", "~ᾰ~υετε";
    ind_act_3: "~ᾰ~υει", _, "~ᾰ~υουσῐ|~ᾰ~υουσῐν";
    ind_mid_1: "~ᾰ~υομαι", _, "~ᾰ~υομεθᾰ";
    ind_mid_2: "~ᾰ~υῃ|~ᾰ~υει", "~ᾰ~υεσθον", "~ᾰ~υεσθε";
    ind_mid_3: "~ᾰ~υεται", _, "~ᾰ~υονται";
    subj_act_1: "~ᾰ~υω", _, "~ᾰ~υωμεν";
    subj_act_2: "~ᾰ~υῃς", "~ᾰ~υητον", "~ᾰ~υητε";
    subj_act_3: "~ᾰ~υῃ", _, "~ᾰ~υωσῐ|~ᾰ~υωσῐν";
    subj_mid_1: "~ᾰ~υωμαι", _, "~ᾰ~υωμεθᾰ";
    subj_mid_2: _, "~ᾰ~υησθον", "~ᾰ~υησθε";
    subj_mid_3: "~ᾰ~υηται", _, "~ᾰ~υωνται";
    opt_act_1: "~ᾰ~υοιμῐ", _, "~ᾰ~υοιμεν";
    opt_act_2: "~ᾰ~υοις", "~ᾰ~υοιτον", "~ᾰ~υοιτε";
    opt_act_3: "~ᾰ~υοι", "~ᾰ~υοιτην", "~ᾰ~υοιεν";
    opt_mid_1: "~ᾰ~υοιμην", _, "~ᾰ~υοιμεθᾰ";
    opt_mid_2: "~ᾰ~υοιο", "~ᾰ~υοισθον", "~ᾰ~υοισθε";
    opt_mid_3: "~ᾰ~υοιτο", "~ᾰ~υοισθην", "~ᾰ~υοιντο";
    imp_act_2: "~ᾰ~υε", _, _;
    imp_act_3: "~ᾰ~υετω", "~ᾰ~υετων", "~ᾰ~υοντων";
    imp_mid_2: "~ᾰ~υου", _, _;
    imp_mid_3: "~ᾰ~υεσθω", "~ᾰ~υεσθων", _;
    imperfect_act_1: "~η~υον", _, "~η~υομεν";
    imperfect_act_2: "~η~υες", "~η~υετον", "~η~υετε";
    imperfect_act_3: "~η~υε|~η~υεν", "~η~υετην", "~η~υον";
    imperfect_mid_1: "~η~υομην", _, "~η~υομεθᾰ";
    imperfect_mid_2: "~η~υου", "~η~υεσθον", "~η~υεσθε";
    imperfect_mid_3: "~η~υετο", "~η~υεσθην", "~η~υοντο";
    future_ind_act_1: "~ᾰ~υσω", _, "~ᾰ~υσομεν";
    future_ind_act_2: "~ᾰ~υσεις", "~ᾰ~υσετον", "~ᾰ~υσετε";
    future_ind_act_3: "~ᾰ~υσει", _, "~ᾰ~υσουσῐ|~ᾰ~υσουσῐν";
    future_ind_mid_1: "~ᾰ~υσομαι", _, "~ᾰ~υσομεθᾰ";
    future_ind_mid_2: "~ᾰ~υσῃ|~ᾰ~υσει", "~ᾰ~υσεσθον", "~ᾰ~υσεσθε";
    future_ind_mid_3: "~ᾰ~υσεται", _, "~ᾰ~υσονται";
    future_ind_pass_1: "~ᾰ~υσθησομαι", _, "~ᾰ~υσθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~υσθησῃ|~ᾰ~υσθησει", "~ᾰ~υσθησεσθον", "~ᾰ~υσθησεσθε";
    future_ind_pass_3: "~ᾰ~υσθησεται", _, "~ᾰ~υσθησονται";
    future_opt_act_1: "~ᾰ~υσοιμῐ", _, "~ᾰ~υσοιμεν";
    future_opt_act_2: "~ᾰ~υσοις", "~ᾰ~υσοιτον", "~ᾰ~υσοιτε";
    future_opt_act_3: "~ᾰ~υσοι", "~ᾰ~υσοιτην", "~ᾰ~υσοιεν";
    future_opt_mid_1: "~ᾰ~υσοιμην", _, "~ᾰ~υσοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~υσοιο", "~ᾰ~υσοισθον", "~ᾰ~υσοισθε";
    future_opt_mid_3: "~ᾰ~υσοιτο", "~ᾰ~υσοισθην", "~ᾰ~υσοιντο";
    future_opt_pass_1: "~ᾰ~υσθησοιμην", _, "~ᾰ~υσθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~υσθησοιο", "~ᾰ~υσθησοισθον", "~ᾰ~υσθησοισθε";
    future_opt_pass_3: "~ᾰ~υσθησοιτο", "~ᾰ~υσθησοισθην", "~ᾰ~υσθησοιντο";
    aorist_subj_act_1: "~ᾰ~υσω", _, "~ᾰ~υσωμεν";
    aorist_subj_act_2: "~ᾰ~υσῃς", "~ᾰ~υσητον", "~ᾰ~υσητε";
    aorist_subj_act_3: "~ᾰ~υσῃ", _, "~ᾰ~υσωσῐ|~ᾰ~υσωσῐν";
    aorist_subj_mid_1: "~ᾰ~υσωμαι", _, "~ᾰ~υσωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰ~υσησθον", "~ᾰ~υσησθε";
    aorist_subj_mid_3: "~ᾰ~υσηται", _, "~ᾰ~υσωνται";
    aorist_subj_pass_1: "~ᾰ~υσθω", _, "~ᾰ~υσθωμεν";
    aorist_subj_pass_2: "~ᾰ~υσθῃς", "~ᾰ~υσθητον", "~ᾰ~υσθητε";
    aorist_subj_pass_3: "~ᾰ~υσθῃ", _, "~ᾰ~υσθωσῐ|~ᾰ~υσθωσῐν";
    aorist_opt_act_1: "~ᾰ~υσαιμῐ", _, "~ᾰ~υσαιμεν";
    aorist_opt_act_2: "~ᾰ~υσειᾰς|~ᾰ~υσαις", "~ᾰ~υσαιτον", "~ᾰ~υσαιτε";
    aorist_opt_act_3: "~ᾰ~υσειε|~ᾰ~υσειεν|~ᾰ~υσαι", "~ᾰ~υσαιτην", "~ᾰ~υσειᾰν|~ᾰ~υσαιεν";
    aorist_opt_mid_1: "~ᾰ~υσαιμην", _, "~ᾰ~υσαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~υσαιο", "~ᾰ~υσαισθον", "~ᾰ~υσαισθε";
    aorist_opt_mid_3: "~ᾰ~υσαιτο", "~ᾰ~υσαισθην", "~ᾰ~υσαιντο";
    aorist_opt_pass_1: "~ᾰ~υσθειην", _, "~ᾰ~υσθεῑμεν|~ᾰ~υσθειημεν";
    aorist_opt_pass_2: "~ᾰ~υσθειης", "~ᾰ~υσθεῑτον|~ᾰ~υσθειητον", "~ᾰ~υσθεῑτε|~ᾰ~υσθειητε";
    aorist_opt_pass_3: "~ᾰ~υσθειη", "~ᾰ~υσθειτην|~ᾰ~υσθειητην", "~ᾰ~υσθεῑεν|~ᾰ~υσθειησᾰν";
    aorist_imp_act_2: "~ᾰ~υσον", "~ᾰ~υσᾰτον", "~ᾰ~υσᾰτε";
    aorist_imp_act_3: "~ᾰ~υσᾰτω", "~ᾰ~υσᾰτων", "~ᾰ~υσᾰντων";
    aorist_imp_mid_2: "~ᾰ~υσαι", "~ᾰ~υσᾰσθον", "~ᾰ~υσᾰσθε";
    aorist_imp_mid_3: "~ᾰ~υσᾰσθω", "~ᾰ~υσᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~υσθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~υσθητω", "~ᾰ~υσθητων", "~ᾰ~υσθεντων";
    perfect_ind_act_1: "~ᾰ%η~ᾰ", _, "~ᾰ%η~ᾰμεν";
    perfect_ind_act_2: "~ᾰ%η~ᾰς", "~ᾰ%η~ᾰτον", "~ᾰ%η~ᾰτε";
    perfect_ind_act_3: "~ᾰ%η~ε|~ᾰ%η~εν", _, "~ᾰ%η~ᾱσῐ|~ᾰ%η~ᾱσῐν";
    perfect_subj_act_1: "~ᾰ%η~ω", _, "~ᾰ%η~ωμεν";
    perfect_subj_act_2: "~ᾰ%η~ῃς", "~ᾰ%η~ητον", "~ᾰ%η~ητε";
    perfect_subj_act_3: "~ᾰ%η~ῃ", _, "~ᾰ%η~ωσῐ|~ᾰ%η~ωσῐν";
    perfect_opt_act_1: "~ᾰ%η~οιμῐ|~ᾰ%η~οιην", _, "~ᾰ%η~οιμεν";
    perfect_opt_act_2: "~ᾰ%η~οις|~ᾰ%η~οιης", "~ᾰ%η~οιτον", "~ᾰ%η~οιτε";
    perfect_opt_act_3: "~ᾰ%η~οι|~ᾰ%η~οιη", "~ᾰ%η~οιτην", "~ᾰ%η~οιεν";
    perfect_imp_act_2: "~ᾰ%η~ε", "~ᾰ%η~ετον", "~ᾰ%η~ετε";
    perfect_imp_act_3: "~ᾰ%η~ετω", "~ᾰ%η~ετων", "~ᾰ%η~οντων";
    infinitive_pres: "~ᾰ~υειν", "~ᾰ~υεσθαι", _;
    infinitive_fut: "~ᾰ~υσειν", "~ᾰ~υσεσθαι", "~ᾰ~υσθησεσθαι";
    infinitive_aor: "~ᾰ~ῡσαι", "~ᾰ~υσᾰσθαι", "~ᾰ~υσθηναι";
    infinitive_perf: "~ᾰ%η~εναι", _, _;
    participle_pres: "~ᾰ~υων", "~ᾰ~υομενος", _;
    participle_fut: "~ᾰ~υσων", "~ᾰ~υσομενος", "~ᾰ~υσθησομενος";
    participle_aor: "~ᾰ~υσᾱς", "~ᾰ~υσᾰμενος", "~ᾰ~υσθεις";
    participle_perf: "~ᾰ%η~ως", _, _;
    ),
    // Λευκαινω: 3 words (λευκαινω, λῑμαινω, ξηραινω). Like Μωραινω, except in 81 cells: future_ind_act_3 pl, future_ind_mid_2 sg, future_opt_act_1 sg, future_opt_act_1 pl, future_opt_act_2 sg, future_opt_act_2 du, ….
    conjugation!("λευκαινω", 3, "~αινω", like Μωραινω as PERFECT_AS_FUTURE;
    stem imperfect: "~αιν- like ῥεω";
    stem aorist_ind: "~αν- like ἐχω";
    future_ind_act_3: _, _, "~ανοῡσῐ|~ανοῡσῐν";
    future_ind_mid_2: "~ανεῑ|~ανῃ", _, _;
    future_opt_act_1: "~ανοιην|~ανοῑμῐ", _, "~ανοῑμεν|~ανοιημεν";
    future_opt_act_2: "~ανοιης|~ανοῑς", "~ανοῑτον|~ανοιητον", "~ανοῑτε|~ανοιητε";
    future_opt_act_3: "~ανοιη|~ανοῑ", "~ανοιτην|~ανοιητην", "~ανοῑεν|~ανοιησᾰν";
    aorist_subj_act_1: "~ανω", _, "~ανωμεν";
    aorist_subj_act_2: "~ανῃς", "~ανητον", "~ανητε";
    aorist_subj_act_3: "~ανῃ", _, "~ανωσῐ";
    aorist_subj_mid_1: "~ανωμαι", _, "~ανωμεθᾰ";
    aorist_subj_mid_2: _, "~ανησθον", "~ανησθε";
    aorist_subj_mid_3: "~ανηται", _, "~ανωνται";
    aorist_subj_pass_3: _, _, "~ανθωσῐ|~ανθωσῐν";
    aorist_opt_act_1: "~αναιμῐ", _, "~αναιμεν";
    aorist_opt_act_2: "~ανειᾰς", "~αναιτον", "~αναιτε";
    aorist_opt_act_3: "~ανειε", "~αναιτην", "~ανειᾰν";
    aorist_opt_mid_1: "~αναιμην", _, "~αναιμεθᾰ";
    aorist_opt_mid_2: "~αναιο", "~αναισθον", "~αναισθε";
    aorist_opt_mid_3: "~αναιτο", "~αναισθην", "~αναιντο";
    aorist_opt_pass_1: _, _, "~ανθεῑμεν|~ανθειημεν";
    aorist_opt_pass_2: _, "~ανθεῑτον|~ανθειητον", "~ανθεῑτε|~ανθειητε";
    aorist_opt_pass_3: _, "~ανθειτην|~ανθειητην", "~ανθεῑεν|~ανθειησᾰν";
    aorist_imp_act_2: "~ανον", "~ανᾰτον", "~ανᾰτε";
    aorist_imp_act_3: "~ανᾰτω", "~ανᾰτων", "~ανᾰντων";
    aorist_imp_mid_2: "~αναι", "~ανᾰσθον", "~ανᾰσθε";
    aorist_imp_mid_3: "~ανᾰσθω", "~ανᾰσθων", _;
    perfect_ind_mid_1: "ἐ~ασμαι", _, "ἐ~ασμεθᾰ";
    perfect_ind_mid_2: "ἐ~ανσαι", "ἐ~ανθον", "ἐ~ανθε";
    perfect_ind_mid_3: "ἐ~ανται", _, "ἐ~ανᾰται";
    perfect_imp_mid_2: "ἐ~ανσο", _, _;
    perfect_imp_mid_3: "ἐ~ανθω", "ἐ~ανθων", _;
    pluperfect_mid_1: "ἐ~ασμην", _, "ἐ~ασμεθᾰ";
    pluperfect_mid_2: "ἐ~ανσο", "ἐ~ανθον", "ἐ~ανθε";
    pluperfect_mid_3: "ἐ~αντο", "ἐ~ανθην", "ἐ~ανᾰτο";
    infinitive_aor: _, "~ανᾰσθαι", _;
    infinitive_perf: derived, "ἐ~ᾱνθαι", _;
    participle_aor: "~ανᾱς", "~ανᾰμενος", _;
    participle_perf: derived, "ἐ~ασμενος", _;
    ),
    // Σῠνδεω: 3 words (σῠνδεω, σῠμμᾰχεω, τρωγλοδῠτεω). Like Πεμπω, except in 227 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("σῠνδεω", 3, "~ῠ~εω", like Πεμπω;
    stem aorist_ind: "~υ~ησ- like ἐχω";
    stem aorist_ind_pass: "~υ~ηθη- like ἐχω";
    stem pluperfect_act: "~υ~ηκε- like ἐχω";
    stem pluperfect: "~υ~η- like πειρᾰω";
    ind_act_1: "~υ~ω", _, "~υ~οῡμεν";
    ind_act_2: "~υ~εῑς", "~υ~εῑτον", "~υ~εῑτε";
    ind_act_3: "~υ~εῑ", _, "~υ~οῡσῐ";
    ind_mid_1: "~υ~οῡμαι", _, "~υ~ουμεθᾰ";
    ind_mid_2: "~υ~εῑ", "~υ~εῑσθον", "~υ~εῑσθε";
    ind_mid_3: "~υ~εῑται", _, "~υ~οῡνται";
    subj_act_1: "~υ~ω", _, "~υ~ωμεν";
    subj_act_2: "~υ~ῃς", "~υ~ητον", "~υ~ητε";
    subj_act_3: "~υ~ῃ", _, "~υ~ωσῐ";
    subj_mid_1: "~υ~ωμαι", _, "~υ~ωμεθᾰ";
    subj_mid_2: _, "~υ~ησθον", "~υ~ησθε";
    subj_mid_3: "~υ~ηται", _, "~υ~ωνται";
    opt_act_1: "~υ~οιην", _, "~υ~οῑμεν";
    opt_act_2: "~υ~οιης", "~υ~οῑτον", "~υ~οῑτε";
    opt_act_3: "~υ~οιη", "~υ~οιτην", "~υ~οῑεν";
    opt_mid_1: "~υ~οιμην", _, "~υ~οιμεθᾰ";
    opt_mid_2: "~υ~οῑο", "~υ~οῑσθον", "~υ~οῑσθε";
    opt_mid_3: "~υ~οῑτο", "~υ~οισθην", "~υ~οῑντο";
    imp_act_2: "~υ~ει", _, _;
    imp_act_3: "~υ~ειτω", "~υ~ειτων", "~υ~ουντων";
    imp_mid_2: "~υ~οῡ", _, _;
    imp_mid_3: "~υ~εισθω", "~υ~εισθων", _;
    imperfect_act_1: "ἐ~υ~ουν", _, "ἐ~υ~οῡμεν";
    imperfect_act_2: "ἐ~υ~εις", "ἐ~υ~εῑτον", "ἐ~υ~εῑτε";
    imperfect_act_3: "ἐ~υ~ει", "ἐ~υ~ειτην", "ἐ~υ~ουν";
    imperfect_mid_1: "ἐ~υ~ουμην", _, "ἐ~υ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~υ~οῡ", "ἐ~υ~εῑσθον", "ἐ~υ~εῑσθε";
    imperfect_mid_3: "ἐ~υ~εῑτο", "ἐ~υ~εισθην", "ἐ~υ~οῡντο";
    future_ind_act_1: "~υ~ησω", _, "~υ~ησομεν";
    future_ind_act_2: "~υ~ησεις", "~υ~ησετον", "~υ~ησετε";
    future_ind_act_3: "~υ~ησει", _, "~υ~ησουσῐ";
    future_ind_mid_1: "~υ~ησομαι", _, "~υ~ησομεθᾰ";
    future_ind_mid_2: "~υ~ησῃ", "~υ~ησεσθον", "~υ~ησεσθε";
    future_ind_mid_3: "~υ~ησεται", _, "~υ~ησονται";
    future_ind_pass_1: "~υ~ηθησομαι", _, "~υ~ηθησομεθᾰ";
    future_ind_pass_2: "~υ~ηθησῃ", "~υ~ηθησεσθον", "~υ~ηθησεσθε";
    future_ind_pass_3: "~υ~ηθησεται", _, "~υ~ηθησονται";
    future_opt_act_1: "~υ~ησοιμῐ", _, "~υ~ησοιμεν";
    future_opt_act_2: "~υ~ησοις", "~υ~ησοιτον", "~υ~ησοιτε";
    future_opt_act_3: "~υ~ησοι", "~υ~ησοιτην", "~υ~ησοιεν";
    future_opt_mid_1: "~υ~ησοιμην", _, "~υ~ησοιμεθᾰ";
    future_opt_mid_2: "~υ~ησοιο", "~υ~ησοισθον", "~υ~ησοισθε";
    future_opt_mid_3: "~υ~ησοιτο", "~υ~ησοισθην", "~υ~ησοιντο";
    future_opt_pass_1: "~υ~ηθησοιμην", _, "~υ~ηθησοιμεθᾰ";
    future_opt_pass_2: "~υ~ηθησοιο", "~υ~ηθησοισθον", "~υ~ηθησοισθε";
    future_opt_pass_3: "~υ~ηθησοιτο", "~υ~ηθησοισθην", "~υ~ηθησοιντο";
    aorist_subj_act_1: "~υ~ησω", _, "~υ~ησωμεν";
    aorist_subj_act_2: "~υ~ησῃς", "~υ~ησητον", "~υ~ησητε";
    aorist_subj_act_3: "~υ~ησῃ", _, "~υ~ησωσῐ";
    aorist_subj_mid_1: "~υ~ησωμαι", _, "~υ~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~υ~ησησθον", "~υ~ησησθε";
    aorist_subj_mid_3: "~υ~ησηται", _, "~υ~ησωνται";
    aorist_subj_pass_1: "~υ~ηθω", _, "~υ~ηθωμεν";
    aorist_subj_pass_2: "~υ~ηθῃς", "~υ~ηθητον", "~υ~ηθητε";
    aorist_subj_pass_3: "~υ~ηθῃ", _, "~υ~ηθωσῐ";
    aorist_opt_act_1: "~υ~ησαιμῐ", _, "~υ~ησαιμεν";
    aorist_opt_act_2: "~υ~ησειᾰς", "~υ~ησαιτον", "~υ~ησαιτε";
    aorist_opt_act_3: "~υ~ησειε", "~υ~ησαιτην", "~υ~ησειᾰν";
    aorist_opt_mid_1: "~υ~ησαιμην", _, "~υ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~υ~ησαιο", "~υ~ησαισθον", "~υ~ησαισθε";
    aorist_opt_mid_3: "~υ~ησαιτο", "~υ~ησαισθην", "~υ~ησαιντο";
    aorist_opt_pass_1: "~υ~ηθειην", _, "~υ~ηθεῑμεν";
    aorist_opt_pass_2: "~υ~ηθειης", "~υ~ηθεῑτον", "~υ~ηθεῑτε";
    aorist_opt_pass_3: "~υ~ηθειη", "~υ~ηθειτην", "~υ~ηθεῑεν";
    aorist_imp_act_2: "~υ~ησον", "~υ~ησᾰτον", "~υ~ησᾰτε";
    aorist_imp_act_3: "~υ~ησᾰτω", "~υ~ησᾰτων", "~υ~ησᾰντων";
    aorist_imp_mid_2: "~υ~ησαι", "~υ~ησᾰσθον", "~υ~ησᾰσθε";
    aorist_imp_mid_3: "~υ~ησᾰσθω", "~υ~ησᾰσθων", _;
    aorist_imp_pass_2: "~υ~ηθητῐ", _, _;
    aorist_imp_pass_3: "~υ~ηθητω", "~υ~ηθητων", "~υ~ηθεντων";
    perfect_ind_act_1: "%ε~υ~ηκᾰ", _, "%ε~υ~ηκᾰμεν";
    perfect_ind_act_2: "%ε~υ~ηκᾰς", "%ε~υ~ηκᾰτον", "%ε~υ~ηκᾰτε";
    perfect_ind_act_3: "%ε~υ~ηκε", _, "%ε~υ~ηκᾱσῐ";
    perfect_ind_mid_1: "%ε~υ~ημαι", _, "%ε~υ~ημεθᾰ";
    perfect_ind_mid_2: "%ε~υ~ησαι", "%ε~υ~ησθον", "%ε~υ~ησθε";
    perfect_ind_mid_3: "%ε~υ~ηται", _, "%ε~υ~ηνται";
    perfect_subj_act_1: "%ε~υ~ηκω", _, "%ε~υ~ηκωμεν";
    perfect_subj_act_2: "%ε~υ~ηκῃς", "%ε~υ~ηκητον", "%ε~υ~ηκητε";
    perfect_subj_act_3: "%ε~υ~ηκῃ", _, "%ε~υ~ηκωσῐ";
    perfect_opt_act_1: "%ε~υ~ηκοιμῐ", _, "%ε~υ~ηκοιμεν";
    perfect_opt_act_2: "%ε~υ~ηκοις", "%ε~υ~ηκοιτον", "%ε~υ~ηκοιτε";
    perfect_opt_act_3: "%ε~υ~ηκοι", "%ε~υ~ηκοιτην", "%ε~υ~ηκοιεν";
    perfect_imp_act_2: _, "%ε~υ~ηκετον", "%ε~υ~ηκετε";
    perfect_imp_act_3: "%ε~υ~ηκετω", "%ε~υ~ηκετων", "%ε~υ~ηκοντων";
    perfect_imp_mid_2: "%ε~υ~ησο", _, _;
    perfect_imp_mid_3: "%ε~υ~ησθω", "%ε~υ~ησθων", _;
    infinitive_pres: "~υ~εῑν", "~υ~εῑσθαι", _;
    infinitive_fut: "~υ~ησειν", "~υ~ησεσθαι", "~υ~ηθησεσθαι";
    infinitive_aor: "~υ~ησαι", "~υ~ησᾰσθαι", "~υ~ηθηναι";
    infinitive_perf: "%ε~υ~ηκεναι", "%ε~υ~ησθαι", _;
    participle_pres: "~υ~ων", "~υ~ουμενος", _;
    participle_fut: "~υ~ησων", "~υ~ησομενος", "~υ~ηθησομενος";
    participle_aor: "~υ~ησᾱς", "~υ~ησᾰμενος", "~υ~ηθεις";
    participle_perf: "%ε~υ~ηκως", "%ε~υ~ημενος", _;
    ),
    // Κηδευω: 3 words (σκῡλευω, κηδευω, τυμβευω). Like Βουλευω, except in 50 cells: perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 sg, perfect_ind_act_2 du, perfect_ind_act_2 pl, perfect_ind_act_3 sg, ….
    conjugation!("κηδευω", 3, "~υω", like Βουλευω;
    stem pluperfect_act: "~υκε- like κᾰλεω";
    perfect_ind_act_1: "~υκᾰ", _, "~υκᾰμεν";
    perfect_ind_act_2: "~υκᾰς", "~υκᾰτον", "~υκᾰτε";
    perfect_ind_act_3: "~υκε|~υκεν", _, "~υκᾱσῐ|~υκᾱσῐν";
    perfect_ind_mid_1: "~υμαι", _, "~υμεθᾰ";
    perfect_ind_mid_2: "~υσαι", "~υσθον", "~υσθε";
    perfect_ind_mid_3: "~υται", _, "~υνται";
    perfect_subj_act_1: "~υκω", _, "~υκωμεν";
    perfect_subj_act_2: "~υκῃς", "~υκητον", "~υκητε";
    perfect_subj_act_3: "~υκῃ", _, "~υκωσῐ|~υκωσῐν";
    perfect_opt_act_1: "~υκοιμῐ|~υκοιην", _, "~υκοιμεν";
    perfect_opt_act_2: "~υκοις|~υκοιης", "~υκοιτον", "~υκοιτε";
    perfect_opt_act_3: "~υκοι|~υκοιη", "~υκοιτην", "~υκοιεν";
    perfect_imp_act_2: "~υκε", "~υκετον", "~υκετε";
    perfect_imp_act_3: "~υκετω", "~υκετων", "~υκοντων";
    perfect_imp_mid_2: "~υσο", _, _;
    perfect_imp_mid_3: "~υσθω", "~υσθων", _;
    pluperfect_mid_1: "ἐ~υμην", _, "ἐ~υμεθᾰ";
    pluperfect_mid_2: "ἐ~υσο", "ἐ~υσθον", "ἐ~υσθε";
    pluperfect_mid_3: "ἐ~υτο", "ἐ~υσθην", "ἐ~υντο";
    infinitive_perf: "~υκεναι", "~ῡσθαι", _;
    participle_perf: "~υκως", "~υμενος", _;
    ),
    // Λεγομαι: 3 words (φθεγγομαι, ἐρευγομαι, λεγομαι). Like Ληγω, except in 109 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("λεγομαι", 3, "~γομαι", like Ληγω as AORIST_AS_FUTURE;
    stem aorist_ind: "~ξ- like πᾰραιτεομαι";
    stem aorist_ind_pass: "~γη- like ἐχω";
    ind_act_1: "~γομαι", _, derived;
    ind_act_2: derived, derived, derived;
    ind_act_3: derived, _, derived;
    ind_mid_2: "~γῃ|~γει", _, _;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, _, derived;
    subj_mid_2: "~γῃ", _, _;
    subj_pass_2: "~γῃ", _, _;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, _, _;
    imp_act_3: derived, derived, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: derived, _, derived;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_subj_pass_1: "~γω", _, "~γωμεν";
    aorist_subj_pass_2: "~γῃς", "~γητον", "~γητε";
    aorist_subj_pass_3: "~γῃ", _, "~γωσῐ";
    aorist_opt_pass_1: "~γειην", _, "~γεῑμεν";
    aorist_opt_pass_2: "~γειης", "~γεῑτον", "~γεῑτε";
    aorist_opt_pass_3: "~γειη", "~γειτην", "~γεῑεν";
    aorist_imp_pass_2: "~γηθῐ", _, _;
    aorist_imp_pass_3: "~γητω", "~γητων", "~γεντων";
    perfect_ind_mid_1: "ἐι~γμαι", _, "ἐι~γμεθᾰ";
    perfect_ind_mid_2: "ἐι~ξαι", "ἐι~χθον", "ἐι~χθε";
    perfect_ind_mid_3: "ἐι~κται", _, "ἐι~γᾰται";
    perfect_imp_mid_2: "ἐι~ξο", _, _;
    perfect_imp_mid_3: "ἐι~χθω", "ἐι~χθων", _;
    pluperfect_mid_1: "ἐι~γμην", _, "ἐι~γμεθᾰ";
    pluperfect_mid_2: "ἐι~ξο", "ἐι~χθον", "ἐι~χθε";
    pluperfect_mid_3: "ἐι~κτο", "ἐι~χθην", "ἐι~γᾰτο";
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, derived, "~γηναι";
    infinitive_perf: _, derived, _;
    participle_pres: derived, _, _;
    participle_fut: derived, _, _;
    participle_aor: derived, derived, "~γεις";
    participle_perf: _, derived, _;
    ),
    // Ῥεω: 3 words (ῥεω, ἐκρεω, ρρεω). Like Αἱρεω, except in 189 cells: ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ind_mid_3 sg, ….
    conjugation!("ῥεω", 3, "~εω", like Αἱρεω as MID_AS_PASS;
    stem aorist_ind: "~ευσ- like ἐχω";
    stem aorist_ind_pass: "~ῠη- like ἐχω";
    ind_mid_1: "~εομαι", _, "~εομεθᾰ";
    ind_mid_2: "~εει", "~εεσθον", "~εεσθε";
    ind_mid_3: "~εεται", _, "~εονται";
    subj_mid_1: "~εωμαι", _, "~εωμεθᾰ";
    subj_mid_2: _, "~εησθον", "~εησθε";
    subj_mid_3: "~εηται", _, "~εωνται";
    opt_mid_1: "~εοιμην", _, "~εοιμεθᾰ";
    opt_mid_2: "~εοιο", "~εοισθον", "~εοισθε";
    opt_mid_3: "~εοιτο", "~εοισθην", "~εοιντο";
    imp_mid_2: "~εου", _, _;
    imp_mid_3: "~εεσθω", "~εεσθων", _;
    imperfect_mid_1: "ἐ~εομην", _, "ἐ~εομεθᾰ";
    imperfect_mid_2: "ἐ~εου", "ἐ~εεσθον", "ἐ~εεσθε";
    imperfect_mid_3: "ἐ~εετο", "ἐ~εεσθην", "ἐ~εοντο";
    future_ind_act_1: "~ευσω", _, "~ευσομεν";
    future_ind_act_2: "~ευσεις", "~ευσετον", "~ευσετε";
    future_ind_act_3: "~ευσει", _, "~ευσουσῐ";
    future_ind_mid_1: "~ευσομαι", _, "~ευσομεθᾰ";
    future_ind_mid_2: "~ευσῃ", "~ευσεσθον", "~ευσεσθε";
    future_ind_mid_3: "~ευσεται", _, "~ευσονται";
    future_ind_pass_1: "~ῠησομαι", _, "~ῠησομεθᾰ";
    future_ind_pass_2: "~ῠησῃ", "~ῠησεσθον", "~ῠησεσθε";
    future_ind_pass_3: "~ῠησεται", _, "~ῠησονται";
    future_opt_act_1: "~ευσοιμῐ", _, "~ευσοιμεν";
    future_opt_act_2: "~ευσοις", "~ευσοιτον", "~ευσοιτε";
    future_opt_act_3: "~ευσοι", "~ευσοιτην", "~ευσοιεν";
    future_opt_mid_1: "~ευσοιμην", _, "~ευσοιμεθᾰ";
    future_opt_mid_2: "~ευσοιο", "~ευσοισθον", "~ευσοισθε";
    future_opt_mid_3: "~ευσοιτο", "~ευσοισθην", "~ευσοιντο";
    future_opt_pass_1: "~ῠησοιμην", _, "~ῠησοιμεθᾰ";
    future_opt_pass_2: "~ῠησοιο", "~ῠησοισθον", "~ῠησοισθε";
    future_opt_pass_3: "~ῠησοιτο", "~ῠησοισθην", "~ῠησοιντο";
    aorist_subj_act_1: "~ευσω", _, "~ευσωμεν";
    aorist_subj_act_2: "~ευσῃς", "~ευσητον", "~ευσητε";
    aorist_subj_act_3: "~ευσῃ", _, "~ευσωσῐ";
    aorist_subj_mid_1: "~ευσωμαι", _, "~ευσωμεθᾰ";
    aorist_subj_mid_2: derived, "~ευσησθον", "~ευσησθε";
    aorist_subj_mid_3: "~ευσηται", _, "~ευσωνται";
    aorist_subj_pass_1: "~ῠω", _, "~ῠωμεν";
    aorist_subj_pass_2: "~ῠῃς", "~ῠητον", "~ῠητε";
    aorist_subj_pass_3: "~ῠῃ", _, "~ῠωσῐ";
    aorist_opt_act_1: "~ευσαιμῐ", _, "~ευσαιμεν";
    aorist_opt_act_2: "~ευσειᾰς", "~ευσαιτον", "~ευσαιτε";
    aorist_opt_act_3: "~ευσειε", "~ευσαιτην", "~ευσειᾰν";
    aorist_opt_mid_1: "~ευσαιμην", _, "~ευσαιμεθᾰ";
    aorist_opt_mid_2: "~ευσαιο", "~ευσαισθον", "~ευσαισθε";
    aorist_opt_mid_3: "~ευσαιτο", "~ευσαισθην", "~ευσαιντο";
    aorist_opt_pass_1: "~ῠειην", _, "~ῠεῑμεν";
    aorist_opt_pass_2: "~ῠειης", "~ῠεῑτον", "~ῠεῑτε";
    aorist_opt_pass_3: "~ῠειη", "~ῠειτην", "~ῠεῑεν";
    aorist_imp_act_2: "~ευσον", "~ευσᾰτον", "~ευσᾰτε";
    aorist_imp_act_3: "~ευσᾰτω", "~ευσᾰτων", "~ευσᾰντων";
    aorist_imp_mid_2: "~ευσαι", "~ευσᾰσθον", "~ευσᾰσθε";
    aorist_imp_mid_3: "~ευσᾰσθω", "~ευσᾰσθων", derived;
    aorist_imp_pass_2: "~ῠηθῐ", _, _;
    aorist_imp_pass_3: "~ῠητω", "~ῠητων", "~ῠεντων";
    perfect_ind_act_1: "ἐ~υηκᾰ", _, "ἐ~υηκᾰμεν";
    perfect_ind_act_2: "ἐ~υηκᾰς", "ἐ~υηκᾰτον", "ἐ~υηκᾰτε";
    perfect_ind_act_3: "ἐ~υηκε", _, "ἐ~υηκᾱσῐ";
    perfect_subj_act_1: "ἐ~υηκω", _, "ἐ~υηκωμεν";
    perfect_subj_act_2: "ἐ~υηκῃς", "ἐ~υηκητον", "ἐ~υηκητε";
    perfect_subj_act_3: "ἐ~υηκῃ", _, "ἐ~υηκωσῐ";
    perfect_opt_act_1: "ἐ~υηκοιμῐ", _, "ἐ~υηκοιμεν";
    perfect_opt_act_2: "ἐ~υηκοις", "ἐ~υηκοιτον", "ἐ~υηκοιτε";
    perfect_opt_act_3: "ἐ~υηκοι", "ἐ~υηκοιτην", "ἐ~υηκοιεν";
    perfect_imp_act_2: _, "ἐ~υηκετον", "ἐ~υηκετε";
    perfect_imp_act_3: "ἐ~υηκετω", "ἐ~υηκετων", "ἐ~υηκοντων";
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    infinitive_pres: _, "~εεσθαι", _;
    infinitive_fut: "~ευσειν", "~ευσεσθαι", "~ῠησεσθαι";
    infinitive_aor: "~εῡσαι", "~ευσᾰσθαι", "~ῠηναι";
    infinitive_perf: "ἐ~υηκεναι", _, _;
    participle_pres: _, "~εομενος", _;
    participle_fut: "~ευσων", "~ευσομενος", "~ῠησομενος";
    participle_aor: "~ευσᾱς", "~ευσᾰμενος", "~ῠεις";
    participle_perf: "ἐ~υηκως", _, _;
    ),
    // Δῐᾰσῳζω: 3 words (ξῠσσῳζω, δῐᾰσῳζω, σσῳζω). Like Γῡροω, except in 219 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("δῐᾰσῳζω", 3, "~σῳζω", like Γῡροω as ACT_AS_PASS;
    stem imperfect: "~ῳζ- like ῥεω";
    stem aorist_ind: "~ωσ- like ἐχω";
    ind_act_1: "~σῳζω", _, "~σῳζομεν";
    ind_act_2: "~σῳζεις", "~σῳζετον", "~σῳζετε";
    ind_act_3: "~σῳζει", _, "~σῳζουσῐ";
    ind_mid_1: "~σῳζομαι", _, "~σῳζομεθᾰ";
    ind_mid_2: "~σῳζῃ", "~σῳζεσθον", "~σῳζεσθε";
    ind_mid_3: "~σῳζεται", _, "~σῳζονται";
    subj_act_1: "~σῳζω", _, "~σῳζωμεν";
    subj_act_2: "~σῳζῃς", "~σῳζητον", "~σῳζητε";
    subj_act_3: "~σῳζῃ", _, "~σῳζωσῐ";
    subj_mid_1: "~σῳζωμαι", _, "~σῳζωμεθᾰ";
    subj_mid_2: _, "~σῳζησθον", "~σῳζησθε";
    subj_mid_3: "~σῳζηται", _, "~σῳζωνται";
    opt_act_1: "~σῳζοιμῐ", _, "~σῳζοιμεν";
    opt_act_2: "~σῳζοις", "~σῳζοιτον", "~σῳζοιτε";
    opt_act_3: "~σῳζοι", "~σῳζοιτην", "~σῳζοιεν";
    opt_mid_1: "~σῳζοιμην", _, "~σῳζοιμεθᾰ";
    opt_mid_2: "~σῳζοιο", "~σῳζοισθον", "~σῳζοισθε";
    opt_mid_3: "~σῳζοιτο", "~σῳζοισθην", "~σῳζοιντο";
    imp_act_2: "~σῳζε", _, _;
    imp_act_3: "~σῳζετω", "~σῳζετων", "~σῳζοντων";
    imp_mid_2: "~σῳζου", _, _;
    imp_mid_3: "~σῳζεσθω", "~σῳζεσθων", _;
    future_ind_act_1: "~σωσω", _, "~σωσομεν";
    future_ind_act_2: "~σωσεις", "~σωσετον", "~σωσετε";
    future_ind_act_3: "~σωσει", _, "~σωσουσῐ";
    future_ind_mid_1: "~σωσομαι", _, "~σωσομεθᾰ";
    future_ind_mid_2: "~σωσῃ", "~σωσεσθον", "~σωσεσθε";
    future_ind_mid_3: "~σωσεται", _, "~σωσονται";
    future_ind_pass_1: "~σωθησομαι", _, "~σωθησομεθᾰ";
    future_ind_pass_2: "~σωθησῃ", "~σωθησεσθον", "~σωθησεσθε";
    future_ind_pass_3: "~σωθησεται", _, "~σωθησονται";
    future_opt_act_1: "~σωσοιμῐ", _, "~σωσοιμεν";
    future_opt_act_2: "~σωσοις", "~σωσοιτον", "~σωσοιτε";
    future_opt_act_3: "~σωσοι", "~σωσοιτην", "~σωσοιεν";
    future_opt_mid_1: "~σωσοιμην", _, "~σωσοιμεθᾰ";
    future_opt_mid_2: "~σωσοιο", "~σωσοισθον", "~σωσοισθε";
    future_opt_mid_3: "~σωσοιτο", "~σωσοισθην", "~σωσοιντο";
    future_opt_pass_1: "~σωθησοιμην", _, "~σωθησοιμεθᾰ";
    future_opt_pass_2: "~σωθησοιο", "~σωθησοισθον", "~σωθησοισθε";
    future_opt_pass_3: "~σωθησοιτο", "~σωθησοισθην", "~σωθησοιντο";
    aorist_subj_act_1: "~σωσω", _, "~σωσωμεν";
    aorist_subj_act_2: "~σωσῃς", "~σωσητον", "~σωσητε";
    aorist_subj_act_3: "~σωσῃ", _, "~σωσωσῐ";
    aorist_subj_mid_1: "~σωσωμαι", _, "~σωσωμεθᾰ";
    aorist_subj_mid_2: _, "~σωσησθον", "~σωσησθε";
    aorist_subj_mid_3: "~σωσηται", _, "~σωσωνται";
    aorist_subj_pass_1: "~σωθω", _, "~σωθωμεν";
    aorist_subj_pass_2: "~σωθῃς", "~σωθητον", "~σωθητε";
    aorist_subj_pass_3: "~σωθῃ", _, "~σωθωσῐ";
    aorist_opt_act_1: "~σωσαιμῐ", _, "~σωσαιμεν";
    aorist_opt_act_2: "~σωσειᾰς", "~σωσαιτον", "~σωσαιτε";
    aorist_opt_act_3: "~σωσειε", "~σωσαιτην", "~σωσειᾰν";
    aorist_opt_mid_1: "~σωσαιμην", _, "~σωσαιμεθᾰ";
    aorist_opt_mid_2: "~σωσαιο", "~σωσαισθον", "~σωσαισθε";
    aorist_opt_mid_3: "~σωσαιτο", "~σωσαισθην", "~σωσαιντο";
    aorist_opt_pass_1: "~σωθειην", _, "~σωθεῑμεν";
    aorist_opt_pass_2: "~σωθειης", "~σωθεῑτον", "~σωθεῑτε";
    aorist_opt_pass_3: "~σωθειη", "~σωθειτην", "~σωθεῑεν";
    aorist_imp_act_2: "~σωσον", "~σωσᾰτον", "~σωσᾰτε";
    aorist_imp_act_3: "~σωσᾰτω", "~σωσᾰτων", "~σωσᾰντων";
    aorist_imp_mid_2: "~σωσαι", "~σωσᾰσθον", "~σωσᾰσθε";
    aorist_imp_mid_3: "~σωσᾰσθω", "~σωσᾰσθων", _;
    aorist_imp_pass_2: "~σωθητῐ", _, _;
    aorist_imp_pass_3: "~σωθητω", "~σωθητων", "~σωθεντων";
    perfect_ind_act_1: "~σεσωκᾰ", _, "~σεσωκᾰμεν";
    perfect_ind_act_2: "~σεσωκᾰς", "~σεσωκᾰτον", "~σεσωκᾰτε";
    perfect_ind_act_3: "~σεσωκε", _, "~σεσωκᾱσῐ";
    perfect_ind_mid_1: "~σεσωσμαι", _, "~σεσωσμεθᾰ";
    perfect_ind_mid_2: "~σεσωσαι", "~σεσωσθον", "~σεσωσθε";
    perfect_ind_mid_3: "~σεσωσται", _, "~σεσωσᾰται";
    perfect_subj_act_1: "~σεσωκω", _, "~σεσωκωμεν";
    perfect_subj_act_2: "~σεσωκῃς", "~σεσωκητον", "~σεσωκητε";
    perfect_subj_act_3: "~σεσωκῃ", _, "~σεσωκωσῐ";
    perfect_opt_act_1: "~σεσωκοιμῐ", _, "~σεσωκοιμεν";
    perfect_opt_act_2: "~σεσωκοις", "~σεσωκοιτον", "~σεσωκοιτε";
    perfect_opt_act_3: "~σεσωκοι", "~σεσωκοιτην", "~σεσωκοιεν";
    perfect_imp_act_2: _, "~σεσωκετον", "~σεσωκετε";
    perfect_imp_act_3: "~σεσωκετω", "~σεσωκετων", "~σεσωκοντων";
    perfect_imp_mid_2: "~σεσωσο", _, _;
    perfect_imp_mid_3: "~σεσωσθω", "~σεσωσθων", _;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_pres: "~σῳζειν", "~σῳζεσθαι", _;
    infinitive_fut: "~σωσειν", "~σωσεσθαι", "~σωθησεσθαι";
    infinitive_aor: "~σωσαι", "~σωσᾰσθαι", "~σωθηναι";
    infinitive_perf: "~σεσωκεναι", "~σεσωσθαι", _;
    participle_pres: "~σῳζων", "~σῳζομενος", _;
    participle_fut: "~σωσων", "~σωσομενος", "~σωθησομενος";
    participle_aor: "~σωσᾱς", "~σωσᾰμενος", "~σωθεις";
    participle_perf: "~σεσωκως", "~σεσωσμενος", _;
    ),
    // Κᾰτᾰβῐβᾰζω: 3 words (κᾰτᾰβῐβᾰζω, πᾰρᾰσκευᾰζω, κᾰτᾰσκευᾰζω). Like Δῐᾰρπᾰζω, except in 81 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("κᾰτᾰβῐβᾰζω", 3, "~ᾰ~ζω", like Δῐᾰρπᾰζω as PERFECT_AS_FUTURE;
    stem aorist_ind: "~ε~σ- like εὐλογεω";
    stem aorist_ind_pass: "~ε~σθη- like ἐλπω";
    imperfect_act_1: "~ε~ζον", _, "~ε~ζομεν";
    imperfect_act_2: "~ε~ζες", "~ε~ζετον", "~ε~ζετε";
    imperfect_act_3: "~ε~ζε|~ε~ζεν", "~ε~ζετην", "~ε~ζον";
    imperfect_mid_1: "~ε~ζομην", _, "~ε~ζομεθᾰ";
    imperfect_mid_2: "~ε~ζου", "~ε~ζεσθον", "~ε~ζεσθε";
    imperfect_mid_3: "~ε~ζετο", "~ε~ζεσθην", "~ε~ζοντο";
    future_ind_act_1: "~ᾰ~σω", _, "~ᾰ~σομεν";
    future_ind_act_2: "~ᾰ~σεις", "~ᾰ~σετον", "~ᾰ~σετε";
    future_ind_act_3: "~ᾰ~σει", _, "~ᾰ~σουσῐ|~ᾰ~σουσῐν";
    future_ind_mid_1: "~ᾰ~σομαι", _, "~ᾰ~σομεθᾰ";
    future_ind_mid_2: "~ᾰ~σῃ|~ᾰ~σει", "~ᾰ~σεσθον", "~ᾰ~σεσθε";
    future_ind_mid_3: "~ᾰ~σεται", _, "~ᾰ~σονται";
    future_ind_pass_2: "~ᾰ~σθησῃ", _, _;
    future_opt_act_1: "~ᾰ~σοιμῐ", _, "~ᾰ~σοιμεν";
    future_opt_act_2: "~ᾰ~σοις", "~ᾰ~σοιτον", "~ᾰ~σοιτε";
    future_opt_act_3: "~ᾰ~σοι", "~ᾰ~σοιτην", "~ᾰ~σοιεν";
    future_opt_mid_1: "~ᾰ~σοιμην", _, "~ᾰ~σοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~σοιο", "~ᾰ~σοισθον", "~ᾰ~σοισθε";
    future_opt_mid_3: "~ᾰ~σοιτο", "~ᾰ~σοισθην", "~ᾰ~σοιντο";
    perfect_ind_mid_1: "~ε~σμαι", _, "~ε~σμεθᾰ";
    perfect_ind_mid_2: "~ε~σαι", "~ε~σθον", "~ε~σθε";
    perfect_ind_mid_3: "~ε~σται", _, "~ε~σᾰται";
    perfect_imp_mid_2: "~ε~σο", _, _;
    perfect_imp_mid_3: "~ε~σθω", "~ε~σθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: "~ε~σμην", _, "~ε~σμεθᾰ";
    pluperfect_mid_2: "~ε~σο", "~ε~σθον", "~ε~σθε";
    pluperfect_mid_3: "~ε~στο", "~ε~σθην", "~ε~σᾰτο";
    infinitive_fut: "~ᾰ~σειν", "~ᾰ~σεσθαι", _;
    infinitive_perf: derived, "~ε~σθαι", _;
    participle_fut: "~ᾰ~σων", "~ᾰ~σομενος", _;
    participle_perf: derived, "~ε~σμενος", _;
    ),
    // Ὠνεομαι: 3 words (ὠνεομαι, θρεομαι, λογεομαι). Like Αἱρεω, except in 92 cells: ind_act_1 sg, subj_mid_2 sg, subj_pass_2 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, ….
    conjugation!("ὠνεομαι", 3, "~εομαι", like Αἱρεω as ACT_AS_PASS;
    ind_act_1: "~εομαι", _, _;
    subj_mid_2: "~εῃ", _, _;
    subj_pass_2: "~εῃ", _, _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: derived, _, derived;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_subj_act_1: derived, _, derived;
    aorist_subj_act_2: derived, derived, derived;
    aorist_subj_act_3: derived, _, derived;
    aorist_subj_mid_1: "ἐ~ησωμαι", _, "ἐ~ησωμεθᾰ";
    aorist_subj_mid_2: "ἐ~ησῃ", "ἐ~ησησθον", "ἐ~ησησθε";
    aorist_subj_mid_3: "ἐ~ησηται", _, "ἐ~ησωνται";
    aorist_subj_pass_1: "ἐ~ηθω", _, "ἐ~ηθωμεν";
    aorist_subj_pass_2: "ἐ~ηθῃς", "ἐ~ηθητον", "ἐ~ηθητε";
    aorist_subj_pass_3: "ἐ~ηθῃ", _, "ἐ~ηθωσῐ";
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_mid_1: "ἐ~ησαιμην", _, "ἐ~ησαιμεθᾰ";
    aorist_opt_mid_2: "ἐ~ησαιο", "ἐ~ησαισθον", "ἐ~ησαισθε";
    aorist_opt_mid_3: "ἐ~ησαιτο", "ἐ~ησαισθην", "ἐ~ησαιντο";
    aorist_opt_pass_1: "ἐ~ηθειην", _, "ἐ~ηθεῑμεν";
    aorist_opt_pass_2: "ἐ~ηθειης", "ἐ~ηθεῑτον", "ἐ~ηθεῑτε";
    aorist_opt_pass_3: "ἐ~ηθειη", "ἐ~ηθειτην", "ἐ~ηθεῑεν";
    aorist_imp_act_2: derived, _, _;
    aorist_imp_act_3: derived, derived, derived;
    aorist_imp_mid_2: "ἐ~ησαι", "ἐ~ησᾰσθον", "ἐ~ησᾰσθε";
    aorist_imp_mid_3: "ἐ~ησᾰσθω", "ἐ~ησᾰσθων", _;
    aorist_imp_pass_2: "ἐ~ηθητῐ", _, _;
    aorist_imp_pass_3: "ἐ~ηθητω", "ἐ~ηθητων", "ἐ~ηθεντων";
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, "ἐ~ησᾰσθαι", "ἐ~ηθηναι";
    participle_fut: derived, _, _;
    participle_aor: derived, "ἐ~ησᾰμενος", "ἐ~ηθεις";
    ),
    // Κᾰτᾰπονεω: 3 words (ὑπᾰνᾰχωρεω, ἐπᾰνᾰχωρεω, κᾰτᾰπονεω). Like Κατοικεω, except in 79 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("κᾰτᾰπονεω", 3, "~ᾰ~εω", like Κατοικεω;
    stem aorist_ind: "~ε~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ε~ηθη- like ἐλπω";
    imperfect_act_1: "~ε~ουν", _, "~ε~οῡμεν";
    imperfect_act_2: "~ε~εις", "~ε~εῑτον", "~ε~εῑτε";
    imperfect_act_3: "~ε~ει", "~ε~ειτην", "~ε~ουν";
    imperfect_mid_1: "~ε~ουμην", _, "~ε~ουμεθᾰ";
    imperfect_mid_2: "~ε~οῡ", "~ε~εῑσθον", "~ε~εῑσθε";
    imperfect_mid_3: "~ε~εῑτο", "~ε~εισθην", "~ε~οῡντο";
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ|~ᾰ~ησουσῐν";
    aorist_subj_act_3: _, _, "~ᾰ~ησωσῐ|~ᾰ~ησωσῐν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς|~ᾰ~ησαις", _, _;
    aorist_opt_act_3: "~ᾰ~ησειε|~ᾰ~ησειεν|~ᾰ~ησαι", _, "~ᾰ~ησειᾰν|~ᾰ~ησαιεν";
    perfect_ind_act_1: "~ᾰ%ε~ηκᾰ", _, "~ᾰ%ε~ηκᾰμεν";
    perfect_ind_act_2: "~ᾰ%ε~ηκᾰς", "~ᾰ%ε~ηκᾰτον", "~ᾰ%ε~ηκᾰτε";
    perfect_ind_act_3: "~ᾰ%ε~ηκε|~ᾰ%ε~ηκεν", _, "~ᾰ%ε~ηκᾱσῐ|~ᾰ%ε~ηκᾱσῐν";
    perfect_ind_mid_1: "~ᾰ%ε~ημαι", _, "~ᾰ%ε~ημεθᾰ";
    perfect_ind_mid_2: "~ᾰ%ε~ησαι", "~ᾰ%ε~ησθον", "~ᾰ%ε~ησθε";
    perfect_ind_mid_3: "~ᾰ%ε~ηται", _, "~ᾰ%ε~ηνται";
    perfect_subj_act_1: "~ᾰ%ε~ηκω", _, "~ᾰ%ε~ηκωμεν";
    perfect_subj_act_2: "~ᾰ%ε~ηκῃς", "~ᾰ%ε~ηκητον", "~ᾰ%ε~ηκητε";
    perfect_subj_act_3: "~ᾰ%ε~ηκῃ", _, "~ᾰ%ε~ηκωσῐ|~ᾰ%ε~ηκωσῐν";
    perfect_opt_act_1: "~ᾰ%ε~ηκοιμῐ|~ᾰ%ε~ηκοιην", _, "~ᾰ%ε~ηκοιμεν";
    perfect_opt_act_2: "~ᾰ%ε~ηκοις|~ᾰ%ε~ηκοιης", "~ᾰ%ε~ηκοιτον", "~ᾰ%ε~ηκοιτε";
    perfect_opt_act_3: "~ᾰ%ε~ηκοι|~ᾰ%ε~ηκοιη", "~ᾰ%ε~ηκοιτην", "~ᾰ%ε~ηκοιεν";
    perfect_imp_act_2: "~ᾰ%ε~ηκε", "~ᾰ%ε~ηκετον", "~ᾰ%ε~ηκετε";
    perfect_imp_act_3: "~ᾰ%ε~ηκετω", "~ᾰ%ε~ηκετων", "~ᾰ%ε~ηκοντων";
    perfect_imp_mid_2: "~ᾰ%ε~ησο", _, _;
    perfect_imp_mid_3: "~ᾰ%ε~ησθω", "~ᾰ%ε~ησθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_perf: "~ᾰ%ε~ηκεναι", "~ᾰ%ε~ησθαι", _;
    participle_perf: "~ᾰ%ε~ηκως", "~ᾰ%ε~ημενος", _;
    ),
    // Κηρῡσσω: 3 words (πῠρεσσω, κηρῡσσω, ἀλλᾰσσω). Like Τᾰσσω, except in 9 cells: perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_ind_mid_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, ….
    conjugation!("κηρῡσσω", 3, "~σσω", like Τᾰσσω;
    perfect_ind_act_3: "%ε~χε|%ε~χεν", _, "%ε~χᾱσῐ|%ε~χᾱσῐν";
    perfect_ind_mid_3: _, _, "%ε~χᾰται";
    perfect_subj_act_3: _, _, "%ε~χωσῐ|%ε~χωσῐν";
    perfect_opt_act_1: "%ε~χοιμῐ|%ε~χοιην", _, _;
    perfect_opt_act_2: "%ε~χοις|%ε~χοιης", _, _;
    perfect_opt_act_3: "%ε~χοι|%ε~χοιη", _, _;
    perfect_imp_act_2: "%ε~χε", _, _;
    pluperfect_mid_3: _, _, "ἐ%ε~χᾰτο";
    ),
    // Ἐπορον: 2 words (ἐφᾰγον, ἐπορον). Like Κρῑνομαι, except in 26 cells: ind_act_1 sg, subj_mid_2 sg, subj_pass_2 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, ….
    conjugation!("ἐπορον", 2, "ἐ~ον", like Κρῑνομαι as MID_AS_PASS;
    ind_act_1: "ἐ~ον", _, _;
    subj_mid_2: derived, _, _;
    subj_pass_2: derived, _, _;
    aorist_ind_act_1: "ἐ~ον", _, "ἐ~ομεν";
    aorist_ind_act_2: "ἐ~ες", "ἐ~ετον", "ἐ~ετε";
    aorist_ind_act_3: _, "ἐ~ετην", "ἐ~ον";
    aorist_opt_act_1: "~οιμῐ", _, "~οιμεν";
    aorist_opt_act_2: "~οις", "~οιτον", "~οιτε";
    aorist_opt_act_3: "~οι", "~οιτην", "~οιεν";
    aorist_imp_act_2: "~ε", "~ετον", "~ετε";
    aorist_imp_act_3: "~ετω", "~ετων", "~οντων";
    infinitive_aor: "~εῑν", _, _;
    participle_aor: "~ων", _, _;
    ),
    // Εἰδον: 2 words (εἰπον, εἰδον). Like Ἐπορον, except in 56 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("εἰδον", 2, "ε~ον", like Ἐπορον;
    stem aorist_ind: "~- like εἰδον";
    ind_act_1: "ε~ον", _, _;
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", "~ησετε";
    future_ind_act_3: "~ησει", _, "~ησουσῐ";
    future_ind_mid_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_mid_2: "~ησῃ", "~ησεσθον", "~ησεσθε";
    future_ind_mid_3: "~ησεται", _, "~ησονται";
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: "~ησοις", "~ησοιτον", "~ησοιτε";
    future_opt_act_3: "~ησοι", "~ησοιτην", "~ησοιεν";
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_2: "~ησοιο", "~ησοισθον", "~ησοισθε";
    future_opt_mid_3: "~ησοιτο", "~ησοισθην", "~ησοιντο";
    aorist_subj_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    aorist_subj_mid_2: _, "~ησθον", "~ησθε";
    aorist_subj_mid_3: "~ηται", _, "~ωνται";
    aorist_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    aorist_opt_mid_2: "~οιο", "~οισθον", "~οισθε";
    aorist_opt_mid_3: "~οιτο", "~οισθην", "~οιντο";
    aorist_imp_mid_2: "~οῡ", "~εσθον", "~εσθε";
    aorist_imp_mid_3: "~εσθω", "~εσθων", _;
    infinitive_fut: "~ησειν", "~ησεσθαι", _;
    infinitive_aor: _, "~εσθαι", _;
    participle_fut: "~ησων", "~ησομενος", _;
    participle_aor: _, "~ομενος", _;
    ),
    // Καινῠμαι: 2 words (πτᾰρνῠμαι, καινῠμαι). Like Ἐπορον, except in 29 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("καινῠμαι", 2, "~νῠμαι", like Ἐπορον;
    ind_act_1: "~νῠμαι", _, _;
    ind_mid_1: "~νῠμαι", _, "~νῠμεθᾰ";
    ind_mid_2: "~νῠσαι", "~νῠσθον", "~νῠσθε";
    ind_mid_3: "~νῠται", _, "~νῠνται";
    subj_mid_1: "~νῠωμαι", _, "~νῠωμεθᾰ";
    subj_mid_2: "~νῠῃ", "~νῠησθον", "~νῠησθε";
    subj_mid_3: "~νῠηται", _, "~νῠωνται";
    subj_pass_2: "~νῠῃ", _, _;
    opt_mid_1: "~νῠοιμην", _, "~νῠοιμεθᾰ";
    opt_mid_2: "~νῠοιο", "~νῠοισθον", "~νῠοισθε";
    opt_mid_3: "~νῠοιτο", "~νῠοισθην", "~νῠοιντο";
    imp_mid_2: "~νῠσο", _, _;
    imp_mid_3: "~νῠσθω", "~νῠσθων", _;
    infinitive_pres: _, "~νῠσθαι", _;
    participle_pres: _, "~νῠμενος", _;
    ),
    // Ξυμπολεμεω: 2 words (συμπολεμεω, ξυμπολεμεω). Like Ἐπῐτῐθημῐ, except in 194 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ξυμπολεμεω", 2, "~υμ~εω", like Ἐπῐτῐθημῐ as MID_AS_PASS;
    stem imperfect: "~ῠνε~- like ξυνναυμᾰχεω";
    stem aorist_ind: "~ῠνε~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ῠνε~ηθη- like ἐλπω";
    ind_act_1: "~ῠμ~ω", _, "~ῠμ~οῡμεν";
    ind_act_2: "~ῠμ~εῑς", "~ῠμ~εῑτον", "~ῠμ~εῑτε";
    ind_act_3: "~ῠμ~εῑ", _, "~ῠμ~οῡσῐ|~ῠμ~οῡσῐν";
    subj_act_1: "~ῠμ~ω", _, "~ῠμ~ωμεν";
    subj_act_2: "~ῠμ~ῃς", "~ῠμ~ητον", "~ῠμ~ητε";
    subj_act_3: "~ῠμ~ῃ", _, "~ῠμ~ωσῐ|~ῠμ~ωσῐν";
    opt_act_1: "~ῠμ~οιην|~ῠμ~οῑμῐ", _, "~ῠμ~οῑμεν|~ῠμ~οιημεν";
    opt_act_2: "~ῠμ~οιης|~ῠμ~οῑς", "~ῠμ~οῑτον|~ῠμ~οιητον", "~ῠμ~οῑτε|~ῠμ~οιητε";
    opt_act_3: "~ῠμ~οιη|~ῠμ~οῑ", "~ῠμ~οιτην|~ῠμ~οιητην", "~ῠμ~οῑεν|~ῠμ~οιησᾰν";
    imp_act_2: "~ῠμ~ει", _, _;
    imp_act_3: "~ῠμ~ειτω", "~ῠμ~ειτων", "~ῠμ~ουντων";
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, derived;
    imperfect_mid_3: derived, derived, derived;
    future_ind_act_1: "~ῠμ~ησω", _, "~ῠμ~ησομεν";
    future_ind_act_2: "~ῠμ~ησεις", "~ῠμ~ησετον", "~ῠμ~ησετε";
    future_ind_act_3: "~ῠμ~ησει", _, "~ῠμ~ησουσῐ|~ῠμ~ησουσῐν";
    future_ind_mid_1: "~ῠμ~ησομαι", _, "~ῠμ~ησομεθᾰ";
    future_ind_mid_2: "~ῠμ~ησῃ|~ῠμ~ησει", "~ῠμ~ησεσθον", "~ῠμ~ησεσθε";
    future_ind_mid_3: "~ῠμ~ησεται", _, "~ῠμ~ησονται";
    future_ind_pass_1: "~ῠμ~ηθησομαι", _, "~ῠμ~ηθησομεθᾰ";
    future_ind_pass_2: "~ῠμ~ηθησῃ|~ῠμ~ηθησει", "~ῠμ~ηθησεσθον", "~ῠμ~ηθησεσθε";
    future_ind_pass_3: "~ῠμ~ηθησεται", _, "~ῠμ~ηθησονται";
    future_opt_act_1: "~ῠμ~ησοιμῐ", _, "~ῠμ~ησοιμεν";
    future_opt_act_2: "~ῠμ~ησοις", "~ῠμ~ησοιτον", "~ῠμ~ησοιτε";
    future_opt_act_3: "~ῠμ~ησοι", "~ῠμ~ησοιτην", "~ῠμ~ησοιεν";
    future_opt_mid_1: "~ῠμ~ησοιμην", _, "~ῠμ~ησοιμεθᾰ";
    future_opt_mid_2: "~ῠμ~ησοιο", "~ῠμ~ησοισθον", "~ῠμ~ησοισθε";
    future_opt_mid_3: "~ῠμ~ησοιτο", "~ῠμ~ησοισθην", "~ῠμ~ησοιντο";
    future_opt_pass_1: "~ῠμ~ηθησοιμην", _, "~ῠμ~ηθησοιμεθᾰ";
    future_opt_pass_2: "~ῠμ~ηθησοιο", "~ῠμ~ηθησοισθον", "~ῠμ~ηθησοισθε";
    future_opt_pass_3: "~ῠμ~ηθησοιτο", "~ῠμ~ηθησοισθην", "~ῠμ~ηθησοιντο";
    aorist_subj_act_1: "~ῠμ~ησω", _, "~ῠμ~ησωμεν";
    aorist_subj_act_2: "~ῠμ~ησῃς", "~ῠμ~ησητον", "~ῠμ~ησητε";
    aorist_subj_act_3: "~ῠμ~ησῃ", _, "~ῠμ~ησωσῐ|~ῠμ~ησωσῐν";
    aorist_subj_mid_1: "~ῠμ~ησωμαι", _, "~ῠμ~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~ῠμ~ησησθον", "~ῠμ~ησησθε";
    aorist_subj_mid_3: "~ῠμ~ησηται", _, "~ῠμ~ησωνται";
    aorist_subj_pass_1: "~ῠμ~ηθω", _, "~ῠμ~ηθωμεν";
    aorist_subj_pass_2: "~ῠμ~ηθῃς", "~ῠμ~ηθητον", "~ῠμ~ηθητε";
    aorist_subj_pass_3: "~ῠμ~ηθῃ", _, "~ῠμ~ηθωσῐ|~ῠμ~ηθωσῐν";
    aorist_opt_act_1: "~ῠμ~ησαιμῐ", _, "~ῠμ~ησαιμεν";
    aorist_opt_act_2: "~ῠμ~ησειᾰς|~ῠμ~ησαις", "~ῠμ~ησαιτον", "~ῠμ~ησαιτε";
    aorist_opt_act_3: "~ῠμ~ησειε|~ῠμ~ησειεν|~ῠμ~ησαι", "~ῠμ~ησαιτην", "~ῠμ~ησειᾰν|~ῠμ~ησαιεν";
    aorist_opt_mid_1: "~ῠμ~ησαιμην", _, "~ῠμ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ῠμ~ησαιο", "~ῠμ~ησαισθον", "~ῠμ~ησαισθε";
    aorist_opt_mid_3: "~ῠμ~ησαιτο", "~ῠμ~ησαισθην", "~ῠμ~ησαιντο";
    aorist_opt_pass_1: "~ῠμ~ηθειην", _, "~ῠμ~ηθεῑμεν|~ῠμ~ηθειημεν";
    aorist_opt_pass_2: "~ῠμ~ηθειης", "~ῠμ~ηθεῑτον|~ῠμ~ηθειητον", "~ῠμ~ηθεῑτε|~ῠμ~ηθειητε";
    aorist_opt_pass_3: "~ῠμ~ηθειη", "~ῠμ~ηθειτην|~ῠμ~ηθειητην", "~ῠμ~ηθεῑεν|~ῠμ~ηθειησᾰν";
    aorist_imp_act_2: "~ῠμ~ησον", "~ῠμ~ησᾰτον", "~ῠμ~ησᾰτε";
    aorist_imp_act_3: "~ῠμ~ησᾰτω", "~ῠμ~ησᾰτων", "~ῠμ~ησᾰντων";
    aorist_imp_mid_2: "~ῠμ~ησαι", "~ῠμ~ησᾰσθον", "~ῠμ~ησᾰσθε";
    aorist_imp_mid_3: "~ῠμ~ησᾰσθω", "~ῠμ~ησᾰσθων", _;
    aorist_imp_pass_2: "~ῠμ~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ῠμ~ηθητω", "~ῠμ~ηθητων", "~ῠμ~ηθεντων";
    perfect_ind_act_1: "~ῠμ%ε~ηκᾰ", _, "~ῠμ%ε~ηκᾰμεν";
    perfect_ind_act_2: "~ῠμ%ε~ηκᾰς", "~ῠμ%ε~ηκᾰτον", "~ῠμ%ε~ηκᾰτε";
    perfect_ind_act_3: "~ῠμ%ε~ηκε|~ῠμ%ε~ηκεν", _, "~ῠμ%ε~ηκᾱσῐ|~ῠμ%ε~ηκᾱσῐν";
    perfect_ind_mid_1: "~ῠμ%ε~ημαι", _, "~ῠμ%ε~ημεθᾰ";
    perfect_ind_mid_2: "~ῠμ%ε~ησαι", "~ῠμ%ε~ησθον", "~ῠμ%ε~ησθε";
    perfect_ind_mid_3: "~ῠμ%ε~ηται", _, "~ῠμ%ε~ηνται";
    perfect_subj_act_1: "~ῠμ%ε~ηκω", _, "~ῠμ%ε~ηκωμεν";
    perfect_subj_act_2: "~ῠμ%ε~ηκῃς", "~ῠμ%ε~ηκητον", "~ῠμ%ε~ηκητε";
    perfect_subj_act_3: "~ῠμ%ε~ηκῃ", _, "~ῠμ%ε~ηκωσῐ|~ῠμ%ε~ηκωσῐν";
    perfect_opt_act_1: "~ῠμ%ε~ηκοιμῐ|~ῠμ%ε~ηκοιην", _, "~ῠμ%ε~ηκοιμεν";
    perfect_opt_act_2: "~ῠμ%ε~ηκοις|~ῠμ%ε~ηκοιης", "~ῠμ%ε~ηκοιτον", "~ῠμ%ε~ηκοιτε";
    perfect_opt_act_3: "~ῠμ%ε~ηκοι|~ῠμ%ε~ηκοιη", "~ῠμ%ε~ηκοιτην", "~ῠμ%ε~ηκοιεν";
    perfect_imp_act_2: "~ῠμ%ε~ηκε", "~ῠμ%ε~ηκετον", "~ῠμ%ε~ηκετε";
    perfect_imp_act_3: "~ῠμ%ε~ηκετω", "~ῠμ%ε~ηκετων", "~ῠμ%ε~ηκοντων";
    perfect_imp_mid_2: "~ῠμ%ε~ησο", _, _;
    perfect_imp_mid_3: "~ῠμ%ε~ησθω", "~ῠμ%ε~ησθων", _;
    infinitive_pres: "~ῠμ~εῑν", _, _;
    infinitive_fut: "~ῠμ~ησειν", "~ῠμ~ησεσθαι", "~ῠμ~ηθησεσθαι";
    infinitive_aor: "~ῠμ~ησαι", "~ῠμ~ησᾰσθαι", "~ῠμ~ηθηναι";
    infinitive_perf: "~ῠμ%ε~ηκεναι", "~ῠμ%ε~ησθαι", _;
    participle_pres: "~ῠμ~ων", _, _;
    participle_fut: "~ῠμ~ησων", "~ῠμ~ησομενος", "~ῠμ~ηθησομενος";
    participle_aor: "~ῠμ~ησᾱς", "~ῠμ~ησᾰμενος", "~ῠμ~ηθεις";
    participle_perf: "~ῠμ%ε~ηκως", "~ῠμ%ε~ημενος", _;
    ),
    // Δῐακειμαι: 2 words (κᾰτακειμαι, δῐακειμαι). Like Ἐγκειμαι, except in 44 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("δῐακειμαι", 2, "~α~ειμαι", like Ἐγκειμαι;
    stem imperfect: "~ε~ει- like δῐακειμαι";
    ind_act_1: "~α~ειμαι", _, _;
    ind_mid_1: "~ᾰ~ειμαι", _, "~ᾰ~ειμεθα";
    ind_mid_2: "~ᾰ~εισαι", "~ᾰ~εισθον", "~ᾰ~εισθε";
    ind_mid_3: "~ᾰ~ειται", _, "~ᾰ~εινται";
    subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    subj_mid_2: "~ᾰ~εῃ", "~ᾰ~εησθον", "~ᾰ~εησθε";
    subj_mid_3: "~ᾰ~εηται", _, "~ᾰ~εωνται";
    opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~οῑο", "~ᾰ~οῑσθον", "~ᾰ~οῑσθε";
    opt_mid_3: "~ᾰ~οῑτο", "~ᾰ~οισθην", "~ᾰ~οῑντο";
    imp_mid_2: "~ᾰ~εισο", _, "~ᾰ~εισθον";
    imp_mid_3: "~ᾰ~εισθω", "~ᾰ~εισθων", _;
    future_ind_mid_1: "~ᾰ~εισομαι", _, "~ᾰ~εισομεθᾰ";
    future_ind_mid_2: "~ᾰ~εισῃ", "~ᾰ~εισεσθον", "~ᾰ~εισεσθε";
    future_ind_mid_3: "~ᾰ~εισεται", _, "~ᾰ~εισονται";
    future_opt_mid_1: "~ᾰ~εισοιμην", _, "~ᾰ~εισοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~εισοιο", "~ᾰ~εισοισθον", "~ᾰ~εισοισθε";
    future_opt_mid_3: "~ᾰ~εισοιτο", "~ᾰ~εισοισθην", "~ᾰ~εισοιντο";
    infinitive_fut: _, "~ᾰ~εισεσθαι", _;
    participle_fut: _, "~ᾰ~εισομενος", _;
    ),
    // Ξῠνοιδᾰ: 2 words (ξῠνοιδᾰ, προοιδᾰ). Like Μῑγνῡμῐ, except in 45 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ξῠνοιδᾰ", 2, "~οιδᾰ", like Μῑγνῡμῐ;
    stem imperfect: "~ῃ- like ξῠνοιδᾰ";
    ind_act_1: "~οιδᾰ", _, "~ϊσμεν";
    ind_act_2: "~οισθᾰ|~οιδᾰς", "~ϊστον", "~ϊστε";
    ind_act_3: "~οιδε", _, "~ϊσᾱσῐ";
    subj_act_1: "~ειδω", _, "~ειδωμεν";
    subj_act_2: "~ειδῃς", "~ειδητον", "~ειδητε";
    subj_act_3: "~ειδῃ", _, "~ειδωσῐ|~ειδωσῐν";
    opt_act_1: "~ειδειην", _, "~ειδεῑμεν|~ειδειημεν";
    opt_act_2: "~ειδειης", "~ειδεῑτον", "~ειδεῑτε|~ειδειητε";
    opt_act_3: "~ειδειη", "~ειδειτην", "~ειδεῑεν|~ειδειησᾰν";
    imp_act_2: "~ϊσθῐ", _, _;
    imp_act_3: "~ϊστω", "~ϊστων", "~ϊστων";
    future_ind_mid_1: "~εισομαι", _, "~εισομεθᾰ";
    future_ind_mid_2: "~εισῃ|~εισει", "~εισεσθον", "~εισεσθε";
    future_ind_mid_3: "~εισεται", _, "~εισονται";
    future_opt_mid_1: "~εισοιμην", _, "~εισοιμεθᾰ";
    future_opt_mid_2: "~εισοιο", "~εισοισθον", "~εισοισθε";
    future_opt_mid_3: "~εισοιτο", "~εισοισθην", "~εισοιντο";
    infinitive_pres: "~ειδεναι", _, _;
    infinitive_fut: _, "~εισεσθαι", _;
    participle_pres: "~ειδως", _, _;
    participle_fut: _, "~εισομενος", _;
    ),
    // Νισσομαι: 2 words (κορινθιαζομαι, νισσομαι). Like Ἐγκειμαι, except in 29 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("νισσομαι", 2, "~ι~ομαι", like Ἐγκειμαι as MID_AS_PASS;
    ind_act_1: "~ι~ομαι", _, _;
    ind_mid_1: "~ῐ~ομαι", _, "~ῐ~ομεθᾰ";
    ind_mid_2: "~ῐ~ῃ", "~ῐ~εσθον", "~ῐ~εσθε";
    ind_mid_3: "~ῐ~εται", _, "~ῐ~ονται";
    subj_mid_1: "~ῐ~ωμαι", _, "~ῐ~ωμεθᾰ";
    subj_mid_2: "~ῐ~ῃ", "~ῐ~ησθον", "~ῐ~ησθε";
    subj_mid_3: "~ῐ~ηται", _, "~ῐ~ωνται";
    subj_pass_2: "~ῐ~ῃ", _, _;
    opt_mid_1: "~ῐ~οιμην", _, "~ῐ~οιμεθᾰ";
    opt_mid_2: "~ῐ~οιο", "~ῐ~οισθον", "~ῐ~οισθε";
    opt_mid_3: "~ῐ~οιτο", "~ῐ~οισθην", "~ῐ~οιντο";
    imp_mid_2: "~ῐ~ου", _, _;
    imp_mid_3: "~ῐ~εσθω", "~ῐ~εσθων", _;
    infinitive_pres: _, "~ῐ~εσθαι", _;
    participle_pres: _, "~ῐ~ομενος", _;
    ),
    // Κᾰτασκηνοω: 2 words (στεφανοω, κᾰτασκηνοω). Like Δημοκρᾰτεομαι, except in 196 cells: ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_mid_2 sg, ind_mid_2 du, ….
    conjugation!("κᾰτασκηνοω", 2, "~α~οω", like Δημοκρᾰτεομαι as PERFECT_AS_FUTURE;
    stem aorist_ind: "~ᾰ~ωσ- like ἐχω";
    stem aorist_ind_pass: "~ᾰ~ωθη- like ἐχω";
    stem pluperfect_act: "~ᾰ~ωκε- like κᾰλεω";
    ind_act_2: "~ᾰ~οῑς", "~ᾰ~οῡτον", "~ᾰ~οῡτε";
    ind_act_3: "~ᾰ~οῑ", _, _;
    ind_mid_2: "~ᾰ~οῑ", "~ᾰ~οῡσθον", "~ᾰ~οῡσθε";
    ind_mid_3: "~ᾰ~οῡται", _, _;
    subj_act_2: "~ᾰ~οῑς", "~ᾰ~ωτον", "~ᾰ~ωτε";
    subj_act_3: "~ᾰ~οῑ", _, _;
    subj_mid_2: _, "~ᾰ~ωσθον", "~ᾰ~ωσθε";
    subj_mid_3: "~ᾰ~ωται", _, _;
    imp_act_2: "~ᾰ~ου", _, _;
    imp_act_3: "~ᾰ~ουτω", "~ᾰ~ουτων", _;
    imp_mid_3: "~ᾰ~ουσθω", "~ᾰ~ουσθων", _;
    imperfect_act_2: "ἐ~ᾰ~ους", "ἐ~ᾰ~οῡτον", "ἐ~ᾰ~οῡτε";
    imperfect_act_3: "ἐ~ᾰ~ου", "ἐ~ᾰ~ουτην", _;
    imperfect_mid_2: _, "ἐ~ᾰ~οῡσθον", "ἐ~ᾰ~οῡσθε";
    imperfect_mid_3: "ἐ~ᾰ~οῡτο", "ἐ~ᾰ~ουσθην", _;
    future_ind_act_1: "~ᾰ~ωσω", _, "~ᾰ~ωσομεν";
    future_ind_act_2: "~ᾰ~ωσεις", "~ᾰ~ωσετον", "~ᾰ~ωσετε";
    future_ind_act_3: "~ᾰ~ωσει", _, "~ᾰ~ωσουσῐ|~ᾰ~ωσουσῐν";
    future_ind_mid_1: "~ᾰ~ωσομαι", _, "~ᾰ~ωσομεθᾰ";
    future_ind_mid_2: "~ᾰ~ωσῃ", "~ᾰ~ωσεσθον", "~ᾰ~ωσεσθε";
    future_ind_mid_3: "~ᾰ~ωσεται", _, "~ᾰ~ωσονται";
    future_ind_pass_1: "~ᾰ~ωθησομαι", _, "~ᾰ~ωθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ωθησῃ", "~ᾰ~ωθησεσθον", "~ᾰ~ωθησεσθε";
    future_ind_pass_3: "~ᾰ~ωθησεται", _, "~ᾰ~ωθησονται";
    future_opt_act_1: "~ᾰ~ωσοιμῐ", _, "~ᾰ~ωσοιμεν";
    future_opt_act_2: "~ᾰ~ωσοις", "~ᾰ~ωσοιτον", "~ᾰ~ωσοιτε";
    future_opt_act_3: "~ᾰ~ωσοι", "~ᾰ~ωσοιτην", "~ᾰ~ωσοιεν";
    future_opt_mid_1: "~ᾰ~ωσοιμην", _, "~ᾰ~ωσοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ωσοιο", "~ᾰ~ωσοισθον", "~ᾰ~ωσοισθε";
    future_opt_mid_3: "~ᾰ~ωσοιτο", "~ᾰ~ωσοισθην", "~ᾰ~ωσοιντο";
    future_opt_pass_1: "~ᾰ~ωθησοιμην", _, "~ᾰ~ωθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ωθησοιο", "~ᾰ~ωθησοισθον", "~ᾰ~ωθησοισθε";
    future_opt_pass_3: "~ᾰ~ωθησοιτο", "~ᾰ~ωθησοισθην", "~ᾰ~ωθησοιντο";
    aorist_subj_act_1: "~ᾰ~ωσω", _, "~ᾰ~ωσωμεν";
    aorist_subj_act_2: "~ᾰ~ωσῃς", "~ᾰ~ωσητον", "~ᾰ~ωσητε";
    aorist_subj_act_3: "~ᾰ~ωσῃ", _, "~ᾰ~ωσωσῐ|~ᾰ~ωσωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωσωμαι", _, "~ᾰ~ωσωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰ~ωσησθον", "~ᾰ~ωσησθε";
    aorist_subj_mid_3: "~ᾰ~ωσηται", _, "~ᾰ~ωσωνται";
    aorist_subj_pass_1: "~ᾰ~ωθω", _, "~ᾰ~ωθωμεν";
    aorist_subj_pass_2: "~ᾰ~ωθῃς", "~ᾰ~ωθητον", "~ᾰ~ωθητε";
    aorist_subj_pass_3: "~ᾰ~ωθῃ", _, "~ᾰ~ωθωσῐ";
    aorist_opt_act_1: "~ᾰ~ωσαιμῐ", _, "~ᾰ~ωσαιμεν";
    aorist_opt_act_2: "~ᾰ~ωσειᾰς|~ᾰ~ωσαις", "~ᾰ~ωσαιτον", "~ᾰ~ωσαιτε";
    aorist_opt_act_3: "~ᾰ~ωσειε|~ᾰ~ωσειεν|~ᾰ~ωσαι", "~ᾰ~ωσαιτην", "~ᾰ~ωσειᾰν|~ᾰ~ωσαιεν";
    aorist_opt_mid_1: "~ᾰ~ωσαιμην", _, "~ᾰ~ωσαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~ωσαιο", "~ᾰ~ωσαισθον", "~ᾰ~ωσαισθε";
    aorist_opt_mid_3: "~ᾰ~ωσαιτο", "~ᾰ~ωσαισθην", "~ᾰ~ωσαιντο";
    aorist_opt_pass_1: "~ᾰ~ωθειην", _, "~ᾰ~ωθεῑμεν";
    aorist_opt_pass_2: "~ᾰ~ωθειης", "~ᾰ~ωθεῑτον", "~ᾰ~ωθεῑτε";
    aorist_opt_pass_3: "~ᾰ~ωθειη", "~ᾰ~ωθειτην", "~ᾰ~ωθεῑεν";
    aorist_imp_act_2: "~ᾰ~ωσον", "~ᾰ~ωσᾰτον", "~ᾰ~ωσᾰτε";
    aorist_imp_act_3: "~ᾰ~ωσᾰτω", "~ᾰ~ωσᾰτων", "~ᾰ~ωσᾰντων";
    aorist_imp_mid_2: "~ᾰ~ωσαι", "~ᾰ~ωσᾰσθον", "~ᾰ~ωσᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ωσᾰσθω", "~ᾰ~ωσᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~ωθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ωθητω", "~ᾰ~ωθητων", "~ᾰ~ωθεντων";
    perfect_ind_act_1: "ἐ~ᾰ~ωκᾰ", _, "ἐ~ᾰ~ωκᾰμεν";
    perfect_ind_act_2: "ἐ~ᾰ~ωκᾰς", "ἐ~ᾰ~ωκᾰτον", "ἐ~ᾰ~ωκᾰτε";
    perfect_ind_act_3: "ἐ~ᾰ~ωκε", _, "ἐ~ᾰ~ωκᾱσῐ";
    perfect_ind_mid_1: "ἐ~ᾰ~ωμαι", _, "ἐ~ᾰ~ωμεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰ~ωσαι", "ἐ~ᾰ~ωσθον", "ἐ~ᾰ~ωσθε";
    perfect_ind_mid_3: "ἐ~ᾰ~ωται", _, "ἐ~ᾰ~ωνται";
    perfect_subj_act_1: "ἐ~ᾰ~ωκω", _, "ἐ~ᾰ~ωκωμεν";
    perfect_subj_act_2: "ἐ~ᾰ~ωκῃς", "ἐ~ᾰ~ωκητον", "ἐ~ᾰ~ωκητε";
    perfect_subj_act_3: "ἐ~ᾰ~ωκῃ", _, "ἐ~ᾰ~ωκωσῐ";
    perfect_opt_act_1: "ἐ~ᾰ~ωκοιμῐ", _, "ἐ~ᾰ~ωκοιμεν";
    perfect_opt_act_2: "ἐ~ᾰ~ωκοις", "ἐ~ᾰ~ωκοιτον", "ἐ~ᾰ~ωκοιτε";
    perfect_opt_act_3: "ἐ~ᾰ~ωκοι", "ἐ~ᾰ~ωκοιτην", "ἐ~ᾰ~ωκοιεν";
    perfect_imp_act_2: _, "ἐ~ᾰ~ωκετον", "ἐ~ᾰ~ωκετε";
    perfect_imp_act_3: "ἐ~ᾰ~ωκετω", "ἐ~ᾰ~ωκετων", "ἐ~ᾰ~ωκοντων";
    perfect_imp_mid_2: "ἐ~ᾰ~ωσο", _, _;
    perfect_imp_mid_3: "ἐ~ᾰ~ωσθω", "ἐ~ᾰ~ωσθων", _;
    pluperfect_mid_1: "ἐ~ᾰ~ωμην", _, "ἐ~ᾰ~ωμεθᾰ";
    pluperfect_mid_2: "ἐ~ᾰ~ωσο", "ἐ~ᾰ~ωσθον", "ἐ~ᾰ~ωσθε";
    pluperfect_mid_3: "ἐ~ᾰ~ωτο", "ἐ~ᾰ~ωσθην", "ἐ~ᾰ~ωντο";
    infinitive_pres: "~ᾰ~οῡν", "~ᾰ~οῡσθαι", _;
    infinitive_fut: "~ᾰ~ωσειν", "~ᾰ~ωσεσθαι", "~ᾰ~ωθησεσθαι";
    infinitive_aor: "~ᾰ~ωσαι", "~ᾰ~ωσᾰσθαι", "~ᾰ~ωθηναι";
    infinitive_perf: "ἐ~ᾰ~ωκεναι", "ἐ~ᾰ~ωσθαι", _;
    participle_fut: "~ᾰ~ωσων", "~ᾰ~ωσομενος", "~ᾰ~ωθησομενος";
    participle_aor: "~ᾰ~ωσᾱς", "~ᾰ~ωσᾰμενος", "~ᾰ~ωθεις";
    participle_perf: "ἐ~ᾰ~ωκως", "ἐ~ᾰ~ωμενος", _;
    ),
    // Γεγονοι: 2 words (γεγονοι, τεθνηκοι). Like Μῑγνῡμῐ, except in 30 cells: ind_act_1 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 sg, perfect_ind_act_2 du, perfect_ind_act_2 pl, ….
    conjugation!("γεγονοι", 2, "~οι", like Μῑγνῡμῐ as SG_AS_PL;
    ind_act_1: "~οι", _, _;
    perfect_ind_act_1: "~ᾰ", _, "~ᾰμεν";
    perfect_ind_act_2: "~ᾰς", "~ᾰτον", "~ᾰτε";
    perfect_ind_act_3: "~ε", _, "~ᾱσῐ";
    perfect_subj_act_1: "~ω", _, "~ωμεν";
    perfect_subj_act_2: "~ῃς", "~ητον", "~ητε";
    perfect_subj_act_3: "~ῃ", _, "~ωσῐ";
    perfect_opt_act_1: "~οιμῐ", _, "~οιμεν";
    perfect_opt_act_2: "~οις", "~οιτον", "~οιτε";
    perfect_opt_act_3: "~οι", "~οιτην", "~οιεν";
    perfect_imp_act_2: _, "~ετον", "~ετε";
    perfect_imp_act_3: "~ετω", "~ετων", "~οντων";
    infinitive_perf: "~εναι", _, _;
    participle_perf: "~ως", _, _;
    ),
    // Ἐπῐχειρεω: 2 words (ἀρῐθμεω, ἐπῐχειρεω). Like Κᾰτᾰπονεω, except in 219 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐπῐχειρεω", 2, "~ῐ~εω", like Κᾰτᾰπονεω as PERFECT_AS_FUTURE;
    stem aorist_ind: "~ε~ησ- like ἐλπω";
    stem pluperfect_act: "~ε~ηκε- like ἀνῐστημῐ";
    ind_act_1: "~ι~ω", _, "~ι~οῡμεν";
    ind_act_2: "~ι~εῑς", "~ι~εῑτον", "~ι~εῑτε";
    ind_act_3: "~ι~εῑ", _, "~ι~οῡσῐ";
    ind_mid_1: "~ι~οῡμαι", _, "~ι~ουμεθᾰ";
    ind_mid_2: "~ι~εῑ", "~ι~εῑσθον", "~ι~εῑσθε";
    ind_mid_3: "~ι~εῑται", _, "~ι~οῡνται";
    subj_act_1: "~ι~ω", _, "~ι~ωμεν";
    subj_act_2: "~ι~ῃς", "~ι~ητον", "~ι~ητε";
    subj_act_3: "~ι~ῃ", _, "~ι~ωσῐ";
    subj_mid_1: "~ι~ωμαι", _, "~ι~ωμεθᾰ";
    subj_mid_2: _, "~ι~ησθον", "~ι~ησθε";
    subj_mid_3: "~ι~ηται", _, "~ι~ωνται";
    opt_act_1: "~ι~οιην", _, "~ι~οῑμεν";
    opt_act_2: "~ι~οιης", "~ι~οῑτον", "~ι~οῑτε";
    opt_act_3: "~ι~οιη", "~ι~οιτην", "~ι~οῑεν";
    opt_mid_1: "~ι~οιμην", _, "~ι~οιμεθᾰ";
    opt_mid_2: "~ι~οῑο", "~ι~οῑσθον", "~ι~οῑσθε";
    opt_mid_3: "~ι~οῑτο", "~ι~οισθην", "~ι~οῑντο";
    imp_act_2: "~ι~ει", _, _;
    imp_act_3: "~ι~ειτω", "~ι~ειτων", "~ι~ουντων";
    imp_mid_2: "~ι~οῡ", _, _;
    imp_mid_3: "~ι~εισθω", "~ι~εισθων", _;
    future_ind_act_1: "~ι~ησω", _, "~ι~ησομεν";
    future_ind_act_2: "~ι~ησεις", "~ι~ησετον", "~ι~ησετε";
    future_ind_act_3: "~ι~ησει", _, "~ι~ησουσῐ";
    future_ind_mid_1: "~ι~ησομαι", _, "~ι~ησομεθᾰ";
    future_ind_mid_2: "~ι~ησῃ", "~ι~ησεσθον", "~ι~ησεσθε";
    future_ind_mid_3: "~ι~ησεται", _, "~ι~ησονται";
    future_ind_pass_1: "~ι~ηθησομαι", _, "~ι~ηθησομεθᾰ";
    future_ind_pass_2: "~ι~ηθησῃ", "~ι~ηθησεσθον", "~ι~ηθησεσθε";
    future_ind_pass_3: "~ι~ηθησεται", _, "~ι~ηθησονται";
    future_opt_act_1: "~ι~ησοιμῐ", _, "~ι~ησοιμεν";
    future_opt_act_2: "~ι~ησοις", "~ι~ησοιτον", "~ι~ησοιτε";
    future_opt_act_3: "~ι~ησοι", "~ι~ησοιτην", "~ι~ησοιεν";
    future_opt_mid_1: "~ι~ησοιμην", _, "~ι~ησοιμεθᾰ";
    future_opt_mid_2: "~ι~ησοιο", "~ι~ησοισθον", "~ι~ησοισθε";
    future_opt_mid_3: "~ι~ησοιτο", "~ι~ησοισθην", "~ι~ησοιντο";
    future_opt_pass_1: "~ι~ηθησοιμην", _, "~ι~ηθησοιμεθᾰ";
    future_opt_pass_2: "~ι~ηθησοιο", "~ι~ηθησοισθον", "~ι~ηθησοισθε";
    future_opt_pass_3: "~ι~ηθησοιτο", "~ι~ηθησοισθην", "~ι~ηθησοιντο";
    aorist_subj_act_1: "~ι~ησω", _, "~ι~ησωμεν";
    aorist_subj_act_2: "~ι~ησῃς", "~ι~ησητον", "~ι~ησητε";
    aorist_subj_act_3: "~ι~ησῃ", _, "~ι~ησωσῐ";
    aorist_subj_mid_1: "~ι~ησωμαι", _, "~ι~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~ι~ησησθον", "~ι~ησησθε";
    aorist_subj_mid_3: "~ι~ησηται", _, "~ι~ησωνται";
    aorist_subj_pass_1: "~ι~ηθω", _, "~ι~ηθωμεν";
    aorist_subj_pass_2: "~ι~ηθῃς", "~ι~ηθητον", "~ι~ηθητε";
    aorist_subj_pass_3: "~ι~ηθῃ", _, "~ι~ηθωσῐ";
    aorist_opt_act_1: "~ι~ησαιμῐ", _, "~ι~ησαιμεν";
    aorist_opt_act_2: "~ι~ησειᾰς", "~ι~ησαιτον", "~ι~ησαιτε";
    aorist_opt_act_3: "~ι~ησειε", "~ι~ησαιτην", "~ι~ησειᾰν";
    aorist_opt_mid_1: "~ι~ησαιμην", _, "~ι~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ι~ησαιο", "~ι~ησαισθον", "~ι~ησαισθε";
    aorist_opt_mid_3: "~ι~ησαιτο", "~ι~ησαισθην", "~ι~ησαιντο";
    aorist_opt_pass_1: "~ι~ηθειην", _, "~ι~ηθεῑμεν";
    aorist_opt_pass_2: "~ι~ηθειης", "~ι~ηθεῑτον", "~ι~ηθεῑτε";
    aorist_opt_pass_3: "~ι~ηθειη", "~ι~ηθειτην", "~ι~ηθεῑεν";
    aorist_imp_act_2: "~ι~ησον", "~ι~ησᾰτον", "~ι~ησᾰτε";
    aorist_imp_act_3: "~ι~ησᾰτω", "~ι~ησᾰτων", "~ι~ησᾰντων";
    aorist_imp_mid_2: "~ι~ησαι", "~ι~ησᾰσθον", "~ι~ησᾰσθε";
    aorist_imp_mid_3: "~ι~ησᾰσθω", "~ι~ησᾰσθων", _;
    aorist_imp_pass_2: "~ι~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ι~ηθητω", "~ι~ηθητων", "~ι~ηθεντων";
    perfect_ind_act_1: "~ε~ηκᾰ", _, "~ε~ηκᾰμεν";
    perfect_ind_act_2: "~ε~ηκᾰς", "~ε~ηκᾰτον", "~ε~ηκᾰτε";
    perfect_ind_act_3: "~ε~ηκε", _, "~ε~ηκᾱσῐ";
    perfect_ind_mid_1: "~ε~ημαι", _, "~ε~ημεθᾰ";
    perfect_ind_mid_2: "~ε~ησαι", "~ε~ησθον", "~ε~ησθε";
    perfect_ind_mid_3: "~ε~ηται", _, "~ε~ηνται";
    perfect_subj_act_1: "~ε~ηκω", _, "~ε~ηκωμεν";
    perfect_subj_act_2: "~ε~ηκῃς", "~ε~ηκητον", "~ε~ηκητε";
    perfect_subj_act_3: "~ε~ηκῃ", _, "~ε~ηκωσῐ";
    perfect_opt_act_1: "~ε~ηκοιμῐ", _, "~ε~ηκοιμεν";
    perfect_opt_act_2: "~ε~ηκοις", "~ε~ηκοιτον", "~ε~ηκοιτε";
    perfect_opt_act_3: "~ε~ηκοι", "~ε~ηκοιτην", "~ε~ηκοιεν";
    perfect_imp_act_2: _, "~ε~ηκετον", "~ε~ηκετε";
    perfect_imp_act_3: "~ε~ηκετω", "~ε~ηκετων", "~ε~ηκοντων";
    perfect_imp_mid_2: "~ε~ησο", _, _;
    perfect_imp_mid_3: "~ε~ησθω", "~ε~ησθων", _;
    pluperfect_mid_1: "~ε~ημην", _, "~ε~ημεθᾰ";
    pluperfect_mid_2: "~ε~ησο", "~ε~ησθον", "~ε~ησθε";
    pluperfect_mid_3: "~ε~ητο", "~ε~ησθην", "~ε~ηντο";
    infinitive_pres: "~ι~εῑν", "~ι~εῑσθαι", _;
    infinitive_fut: "~ι~ησειν", "~ι~ησεσθαι", "~ι~ηθησεσθαι";
    infinitive_aor: "~ι~ησαι", "~ι~ησᾰσθαι", "~ι~ηθηναι";
    infinitive_perf: "~ε~ηκεναι", "~ε~ησθαι", _;
    participle_pres: "~ι~ων", "~ι~ουμενος", _;
    participle_fut: "~ι~ησων", "~ι~ησομενος", "~ι~ηθησομενος";
    participle_aor: "~ι~ησᾱς", "~ι~ησᾰμενος", "~ι~ηθεις";
    participle_perf: "~ε~ηκως", "~ε~ημενος", _;
    ),
    // Συνεπομαι: 2 words (συνεπομαι, ξυνεπομαι). Like Ἐρχομαι, except in 68 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("συνεπομαι", 2, "~υνεπομαι", like Ἐρχομαι;
    stem imperfect: "~ῠνειπ- like ὑποκοριζομαι";
    stem aorist_ind: "~ῠνεσπ- like ἐρχομαι";
    ind_act_1: "~υνεπομαι", _, _;
    ind_mid_1: "~ῠνεπομαι", _, "~ῠνεπομεθᾰ";
    ind_mid_2: "~ῠνεπῃ|~ῠνεπει", "~ῠνεπεσθον", "~ῠνεπεσθε";
    ind_mid_3: "~ῠνεπεται", _, "~ῠνεπονται";
    subj_mid_1: "~ῠνεπωμαι", _, "~ῠνεπωμεθᾰ";
    subj_mid_2: "~ῠνεπῃ", "~ῠνεπησθον", "~ῠνεπησθε";
    subj_mid_3: "~ῠνεπηται", _, "~ῠνεπωνται";
    subj_pass_2: "~ῠνεπῃ", _, _;
    opt_mid_1: "~ῠνεποιμην", _, "~ῠνεποιμεθᾰ";
    opt_mid_2: "~ῠνεποιο", "~ῠνεποισθον", "~ῠνεποισθε";
    opt_mid_3: "~ῠνεποιτο", "~ῠνεποισθην", "~ῠνεποιντο";
    imp_mid_2: "~ῠνεπου", _, _;
    imp_mid_3: "~ῠνεπεσθω", "~ῠνεπεσθων", _;
    future_ind_mid_1: "~ῠνεψομαι", _, "~ῠνεψομεθᾰ";
    future_ind_mid_2: "~ῠνεψῃ|~ῠνεψει", "~ῠνεψεσθον", "~ῠνεψεσθε";
    future_ind_mid_3: "~ῠνεψεται", _, "~ῠνεψονται";
    future_opt_mid_1: "~ῠνεψοιμην", _, "~ῠνεψοιμεθᾰ";
    future_opt_mid_2: "~ῠνεψοιο", "~ῠνεψοισθον", "~ῠνεψοισθε";
    future_opt_mid_3: "~ῠνεψοιτο", "~ῠνεψοισθην", "~ῠνεψοιντο";
    aorist_subj_mid_1: "~ῠσπωμαι", _, "~ῠσπωμεθᾰ";
    aorist_subj_mid_2: "~ῠσπῃ", "~ῠσπησθον", "~ῠσπησθε";
    aorist_subj_mid_3: "~ῠσπηται", _, "~ῠσπωνται";
    aorist_opt_mid_1: "~ῠσποιμην", _, "~ῠσποιμεθᾰ";
    aorist_opt_mid_2: "~ῠσποιο", "~ῠσποισθον", "~ῠσποισθε";
    aorist_opt_mid_3: "~ῠσποιτο", "~ῠσποισθην", "~ῠσποιντο";
    aorist_imp_mid_2: "~ῠσποῡ", "~ῠσπεσθον", "~ῠσπεσθε";
    aorist_imp_mid_3: "~ῠσπεσθω", "~ῠσπεσθων", _;
    infinitive_pres: _, "~ῠνεπεσθαι", _;
    infinitive_fut: _, "~ῠνεψεσθαι", _;
    infinitive_aor: _, "~ῠσπεσθαι", _;
    participle_pres: _, "~ῠνεπομενος", _;
    participle_fut: _, "~ῠνεψομενος", _;
    participle_aor: _, "~ῠσπομενος", _;
    ),
    // Λαλαγεω: 2 words (καναχεω, λαλαγεω). Like Ζευγνῡμῐ, except in 186 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("λαλαγεω", 2, "~α~α~εω", like Ζευγνῡμῐ;
    stem aorist_ind: "~ᾰ~ᾰ~ησ- like ποιεω";
    stem aorist_ind_pass: "~ᾰ~ᾰ~ηθη- like ἐχω";
    ind_act_1: "~ᾰ~ᾰ~ω", _, "~ᾰ~ᾰ~οῡμεν";
    ind_act_2: "~ᾰ~ᾰ~εῑς", "~ᾰ~ᾰ~εῑτον", "~ᾰ~ᾰ~εῑτε";
    ind_act_3: "~ᾰ~ᾰ~εῑ", _, "~ᾰ~ᾰ~οῡσῐ|~ᾰ~ᾰ~οῡσῐν";
    ind_mid_1: "~ᾰ~ᾰ~οῡμαι", _, "~ᾰ~ᾰ~ουμεθᾰ";
    ind_mid_2: "~ᾰ~ᾰ~εῑ|~ᾰ~ᾰ~ῃ", "~ᾰ~ᾰ~εῑσθον", "~ᾰ~ᾰ~εῑσθε";
    ind_mid_3: "~ᾰ~ᾰ~εῑται", _, "~ᾰ~ᾰ~οῡνται";
    subj_act_1: "~ᾰ~ᾰ~ω", _, "~ᾰ~ᾰ~ωμεν";
    subj_act_2: "~ᾰ~ᾰ~ῃς", "~ᾰ~ᾰ~ητον", "~ᾰ~ᾰ~ητε";
    subj_act_3: "~ᾰ~ᾰ~ῃ", _, "~ᾰ~ᾰ~ωσῐ|~ᾰ~ᾰ~ωσῐν";
    subj_mid_1: "~ᾰ~ᾰ~ωμαι", _, "~ᾰ~ᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ᾰ~ησθον", "~ᾰ~ᾰ~ησθε";
    subj_mid_3: "~ᾰ~ᾰ~ηται", _, "~ᾰ~ᾰ~ωνται";
    opt_act_1: "~ᾰ~ᾰ~οιην|~ᾰ~ᾰ~οῑμῐ", _, "~ᾰ~ᾰ~οῑμεν|~ᾰ~ᾰ~οιημεν";
    opt_act_2: "~ᾰ~ᾰ~οιης|~ᾰ~ᾰ~οῑς", "~ᾰ~ᾰ~οῑτον|~ᾰ~ᾰ~οιητον", "~ᾰ~ᾰ~οῑτε|~ᾰ~ᾰ~οιητε";
    opt_act_3: "~ᾰ~ᾰ~οιη|~ᾰ~ᾰ~οῑ", "~ᾰ~ᾰ~οιτην|~ᾰ~ᾰ~οιητην", "~ᾰ~ᾰ~οῑεν|~ᾰ~ᾰ~οιησᾰν";
    opt_mid_1: "~ᾰ~ᾰ~οιμην", _, "~ᾰ~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~ᾰ~οῑο", "~ᾰ~ᾰ~οῑσθον", "~ᾰ~ᾰ~οῑσθε";
    opt_mid_3: "~ᾰ~ᾰ~οῑτο", "~ᾰ~ᾰ~οισθην", "~ᾰ~ᾰ~οῑντο";
    imp_act_2: "~ᾰ~ᾰ~ει", _, _;
    imp_act_3: "~ᾰ~ᾰ~ειτω", "~ᾰ~ᾰ~ειτων", "~ᾰ~ᾰ~ουντων";
    imp_mid_2: "~ᾰ~ᾰ~οῡ", _, _;
    imp_mid_3: "~ᾰ~ᾰ~εισθω", "~ᾰ~ᾰ~εισθων", _;
    imperfect_act_1: "ἐ~ᾰ~ᾰ~ουν", _, "ἐ~ᾰ~ᾰ~οῡμεν";
    imperfect_act_2: "ἐ~ᾰ~ᾰ~εις", "ἐ~ᾰ~ᾰ~εῑτον", "ἐ~ᾰ~ᾰ~εῑτε";
    imperfect_act_3: "ἐ~ᾰ~ᾰ~ει", "ἐ~ᾰ~ᾰ~ειτην", "ἐ~ᾰ~ᾰ~ουν";
    imperfect_mid_1: "ἐ~ᾰ~ᾰ~ουμην", _, "ἐ~ᾰ~ᾰ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~ᾰ~οῡ", "ἐ~ᾰ~ᾰ~εῑσθον", "ἐ~ᾰ~ᾰ~εῑσθε";
    imperfect_mid_3: "ἐ~ᾰ~ᾰ~εῑτο", "ἐ~ᾰ~ᾰ~εισθην", "ἐ~ᾰ~ᾰ~οῡντο";
    future_ind_act_1: "~ᾰ~ᾰ~ησω", _, "~ᾰ~ᾰ~ησομεν";
    future_ind_act_2: "~ᾰ~ᾰ~ησεις", "~ᾰ~ᾰ~ησετον", "~ᾰ~ᾰ~ησετε";
    future_ind_act_3: "~ᾰ~ᾰ~ησει", _, "~ᾰ~ᾰ~ησουσῐ|~ᾰ~ᾰ~ησουσῐν";
    future_ind_mid_1: "~ᾰ~ᾰ~ησομαι", _, "~ᾰ~ᾰ~ησομεθᾰ";
    future_ind_mid_2: "~ᾰ~ᾰ~ησῃ|~ᾰ~ᾰ~ησει", "~ᾰ~ᾰ~ησεσθον", "~ᾰ~ᾰ~ησεσθε";
    future_ind_mid_3: "~ᾰ~ᾰ~ησεται", _, "~ᾰ~ᾰ~ησονται";
    future_ind_pass_1: "~ᾰ~ᾰ~ηθησομαι", _, "~ᾰ~ᾰ~ηθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ᾰ~ηθησῃ", "~ᾰ~ᾰ~ηθησεσθον", "~ᾰ~ᾰ~ηθησεσθε";
    future_ind_pass_3: "~ᾰ~ᾰ~ηθησεται", _, "~ᾰ~ᾰ~ηθησονται";
    future_opt_act_1: "~ᾰ~ᾰ~ησοιμῐ", _, "~ᾰ~ᾰ~ησοιμεν";
    future_opt_act_2: "~ᾰ~ᾰ~ησοις", "~ᾰ~ᾰ~ησοιτον", "~ᾰ~ᾰ~ησοιτε";
    future_opt_act_3: "~ᾰ~ᾰ~ησοι", "~ᾰ~ᾰ~ησοιτην", "~ᾰ~ᾰ~ησοιεν";
    future_opt_mid_1: "~ᾰ~ᾰ~ησοιμην", _, "~ᾰ~ᾰ~ησοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ᾰ~ησοιο", "~ᾰ~ᾰ~ησοισθον", "~ᾰ~ᾰ~ησοισθε";
    future_opt_mid_3: "~ᾰ~ᾰ~ησοιτο", "~ᾰ~ᾰ~ησοισθην", "~ᾰ~ᾰ~ησοιντο";
    future_opt_pass_1: "~ᾰ~ᾰ~ηθησοιμην", _, "~ᾰ~ᾰ~ηθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ᾰ~ηθησοιο", "~ᾰ~ᾰ~ηθησοισθον", "~ᾰ~ᾰ~ηθησοισθε";
    future_opt_pass_3: "~ᾰ~ᾰ~ηθησοιτο", "~ᾰ~ᾰ~ηθησοισθην", "~ᾰ~ᾰ~ηθησοιντο";
    aorist_subj_act_1: "~ᾰ~ᾰ~ηθω", _, "~ᾰ~ᾰ~ηθωμεν";
    aorist_subj_act_2: "~ᾰ~ᾰ~ηθῃς", "~ᾰ~ᾰ~ηθητον", "~ᾰ~ᾰ~ηθητε";
    aorist_subj_act_3: "~ᾰ~ᾰ~ηθῃ", _, "~ᾰ~ᾰ~ηθωσῐ";
    aorist_subj_mid_1: "~ᾰ~ᾰ~ηθωμαι", _, "~ᾰ~ᾰ~ηθωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰ~ᾰ~ηθησθον", "~ᾰ~ᾰ~ηθησθε";
    aorist_subj_mid_3: "~ᾰ~ᾰ~ηθηται", _, "~ᾰ~ᾰ~ηθωνται";
    aorist_subj_pass_1: "~ᾰ~ᾰ~ηθω", _, "~ᾰ~ᾰ~ηθωμεν";
    aorist_subj_pass_2: "~ᾰ~ᾰ~ηθῃς", "~ᾰ~ᾰ~ηθητον", "~ᾰ~ᾰ~ηθητε";
    aorist_subj_pass_3: "~ᾰ~ᾰ~ηθῃ", _, "~ᾰ~ᾰ~ηθωσῐ|~ᾰ~ᾰ~ηθωσῐν";
    aorist_opt_act_1: "~ᾰ~ᾰ~ηθαιμῐ", _, "~ᾰ~ᾰ~ηθαιμεν";
    aorist_opt_act_2: "~ᾰ~ᾰ~ηθειᾰς", "~ᾰ~ᾰ~ηθαιτον", "~ᾰ~ᾰ~ηθαιτε";
    aorist_opt_act_3: "~ᾰ~ᾰ~ηθειε", "~ᾰ~ᾰ~ηθαιτην", "~ᾰ~ᾰ~ηθειᾰν";
    aorist_opt_mid_1: "~ᾰ~ᾰ~ηθαιμην", _, "~ᾰ~ᾰ~ηθαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~ᾰ~ηθαιο", "~ᾰ~ᾰ~ηθαισθον", "~ᾰ~ᾰ~ηθαισθε";
    aorist_opt_mid_3: "~ᾰ~ᾰ~ηθαιτο", "~ᾰ~ᾰ~ηθαισθην", "~ᾰ~ᾰ~ηθαιντο";
    aorist_opt_pass_1: "~ᾰ~ᾰ~ηθειην", _, "~ᾰ~ᾰ~ηθεῑμεν|~ᾰ~ᾰ~ηθειημεν";
    aorist_opt_pass_2: "~ᾰ~ᾰ~ηθειης", "~ᾰ~ᾰ~ηθεῑτον|~ᾰ~ᾰ~ηθειητον", "~ᾰ~ᾰ~ηθεῑτε|~ᾰ~ᾰ~ηθειητε";
    aorist_opt_pass_3: "~ᾰ~ᾰ~ηθειη", "~ᾰ~ᾰ~ηθειτην|~ᾰ~ᾰ~ηθειητην", "~ᾰ~ᾰ~ηθεῑεν|~ᾰ~ᾰ~ηθειησᾰν";
    aorist_imp_act_2: "~ᾰ~ᾰ~ηθον", "~ᾰ~ᾰ~ηθᾰτον", "~ᾰ~ᾰ~ηθᾰτε";
    aorist_imp_act_3: "~ᾰ~ᾰ~ηθᾰτω", "~ᾰ~ᾰ~ηθᾰτων", "~ᾰ~ᾰ~ηθᾰντων";
    aorist_imp_mid_2: "~ᾰ~ᾰ~ηθαι", "~ᾰ~ᾰ~ηθᾰσθον", "~ᾰ~ᾰ~ηθᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ᾰ~ηθᾰσθω", "~ᾰ~ᾰ~ηθᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~ᾰ~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ᾰ~ηθητω", "~ᾰ~ᾰ~ηθητων", "~ᾰ~ᾰ~ηθεντων";
    infinitive_pres: "~ᾰ~ᾰ~εῑν", "~ᾰ~ᾰ~εῑσθαι", _;
    infinitive_fut: "~ᾰ~ᾰ~ησειν", "~ᾰ~ᾰ~ησεσθαι", "~ᾰ~ᾰ~ηθησεσθαι";
    infinitive_aor: "~ᾰ~ᾰ~ηθαι", "~ᾰ~ᾰ~ηθᾰσθαι", "~ᾰ~ᾰ~ηθηναι";
    participle_pres: "~ᾰ~ᾰ~ων", "~ᾰ~ᾰ~ουμενος", _;
    participle_fut: "~ᾰ~ᾰ~ησων", "~ᾰ~ᾰ~ησομενος", "~ᾰ~ᾰ~ηθησομενος";
    participle_aor: "~ᾰ~ᾰ~ηθᾱς", "~ᾰ~ᾰ~ηθᾰμενος", "~ᾰ~ᾰ~ηθεις";
    ),
    // Ξυνναυμᾰχεω: 2 words (συνναυμᾰχεω, ξυνναυμᾰχεω). Like Ζευγνῡμῐ, except in 84 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ξυνναυμᾰχεω", 2, "~υ~~εω", like Ζευγνῡμῐ as MID_AS_PASS;
    stem imperfect: "~ῠ~ε~- like ξυνναυμᾰχεω";
    stem aorist_ind: "~ῠ~ε~ησ- like ξυνναυμᾰχεω";
    ind_act_1: "~ῠ~~ω", _, "~ῠ~~οῡμεν";
    ind_act_2: "~ῠ~~εῑς", "~ῠ~~εῑτον", "~ῠ~~εῑτε";
    ind_act_3: "~ῠ~~εῑ", _, "~ῠ~~οῡσῐ|~ῠ~~οῡσῐν";
    subj_act_1: "~ῠ~~ω", _, "~ῠ~~ωμεν";
    subj_act_2: "~ῠ~~ῃς", "~ῠ~~ητον", "~ῠ~~ητε";
    subj_act_3: "~ῠ~~ῃ", _, "~ῠ~~ωσῐ|~ῠ~~ωσῐν";
    opt_act_1: "~ῠ~~οιην|~ῠ~~οῑμῐ", _, "~ῠ~~οῑμεν|~ῠ~~οιημεν";
    opt_act_2: "~ῠ~~οιης|~ῠ~~οῑς", "~ῠ~~οῑτον|~ῠ~~οιητον", "~ῠ~~οῑτε|~ῠ~~οιητε";
    opt_act_3: "~ῠ~~οιη|~ῠ~~οῑ", "~ῠ~~οιτην|~ῠ~~οιητην", "~ῠ~~οῑεν|~ῠ~~οιησᾰν";
    imp_act_2: "~ῠ~~ει", _, _;
    imp_act_3: "~ῠ~~ειτω", "~ῠ~~ειτων", "~ῠ~~ουντων";
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, derived;
    imperfect_mid_3: derived, derived, derived;
    future_ind_act_1: "~ῠ~~ησω", _, "~ῠ~~ησομεν";
    future_ind_act_2: "~ῠ~~ησεις", "~ῠ~~ησετον", "~ῠ~~ησετε";
    future_ind_act_3: "~ῠ~~ησει", _, "~ῠ~~ησουσῐ|~ῠ~~ησουσῐν";
    future_opt_act_1: "~ῠ~~ησοιμῐ", _, "~ῠ~~ησοιμεν";
    future_opt_act_2: "~ῠ~~ησοις", "~ῠ~~ησοιτον", "~ῠ~~ησοιτε";
    future_opt_act_3: "~ῠ~~ησοι", "~ῠ~~ησοιτην", "~ῠ~~ησοιεν";
    aorist_ind_mid_1: derived, _, derived;
    aorist_ind_mid_2: derived, derived, derived;
    aorist_ind_mid_3: derived, derived, derived;
    aorist_subj_act_1: "~ῠ~~ησω", _, "~ῠ~~ησωμεν";
    aorist_subj_act_2: "~ῠ~~ησῃς", "~ῠ~~ησητον", "~ῠ~~ησητε";
    aorist_subj_act_3: "~ῠ~~ησῃ", _, "~ῠ~~ησωσῐ|~ῠ~~ησωσῐν";
    aorist_opt_act_1: "~ῠ~~ησαιμῐ", _, "~ῠ~~ησαιμεν";
    aorist_opt_act_2: "~ῠ~~ησειᾰς|~ῠ~~ησαις", "~ῠ~~ησαιτον", "~ῠ~~ησαιτε";
    aorist_opt_act_3: "~ῠ~~ησειε|~ῠ~~ησειεν|~ῠ~~ησαι", "~ῠ~~ησαιτην", "~ῠ~~ησειᾰν|~ῠ~~ησαιεν";
    aorist_imp_act_2: "~ῠ~~ησον", "~ῠ~~ησᾰτον", "~ῠ~~ησᾰτε";
    aorist_imp_act_3: "~ῠ~~ησᾰτω", "~ῠ~~ησᾰτων", "~ῠ~~ησᾰντων";
    infinitive_pres: "~ῠ~~εῑν", _, _;
    infinitive_fut: "~ῠ~~ησειν", _, _;
    infinitive_aor: "~ῠ~~ησαι", _, _;
    participle_pres: "~ῠ~~ων", _, _;
    participle_fut: "~ῠ~~ησων", _, _;
    participle_aor: "~ῠ~~ησᾱς", _, _;
    ),
    // Προσκειμαι: 2 words (προκειμαι, προσκειμαι). Like Δῐακειμαι, except in 45 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("προσκειμαι", 2, "~~ειμαι", like Δῐακειμαι;
    ind_act_1: "~~ειμαι", _, _;
    ind_mid_1: "~~ειμαι", _, "~~ειμεθα";
    ind_mid_2: "~~εισαι", "~~εισθον", "~~εισθε";
    ind_mid_3: "~~ειται", _, "~~εινται";
    subj_mid_1: "~~ωμαι", _, "~~ωμεθᾰ";
    subj_mid_2: "~~εῃ", "~~εησθον", "~~εησθε";
    subj_mid_3: "~~εηται", _, "~~εωνται|~~ειωνται";
    opt_mid_1: "~~οιμην", _, "~~οιμεθᾰ";
    opt_mid_2: "~~οῑο", "~~οῑσθον", "~~οῑσθε";
    opt_mid_3: "~~οῑτο", "~~οισθην", "~~οῑντο";
    imp_mid_2: "~~εισο", _, "~~εισθον";
    imp_mid_3: "~~εισθω", "~~εισθων", _;
    imperfect_mid_3: _, _, "~ε~εατο";
    future_ind_mid_1: "~~εισομαι", _, "~~εισομεθᾰ";
    future_ind_mid_2: "~~εισῃ", "~~εισεσθον", "~~εισεσθε";
    future_ind_mid_3: "~~εισεται", _, "~~εισονται";
    future_opt_mid_1: "~~εισοιμην", _, "~~εισοιμεθᾰ";
    future_opt_mid_2: "~~εισοιο", "~~εισοισθον", "~~εισοισθε";
    future_opt_mid_3: "~~εισοιτο", "~~εισοισθην", "~~εισοιντο";
    infinitive_fut: _, "~~εισεσθαι", _;
    participle_fut: _, "~~εισομενος", _;
    ),
    // Μετᾰκᾰλεω: 2 words (μηχᾰνορρᾰφεω, μετᾰκᾰλεω). Like Ζευγνῡμῐ, except in 235 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("μετᾰκᾰλεω", 2, "~ᾰ~ᾰ~εω", like Ζευγνῡμῐ;
    stem aorist_ind: "~ε~α~εσ- like ἐλπω";
    stem aorist_ind_pass: "~ε~α~εθη- like ἐλπω";
    stem pluperfect_act: "~ε~εκα~εκε- like ἀνῐστημῐ";
    ind_act_1: "~α~α~ω", _, "~α~α~οῡμεν";
    ind_act_2: "~α~α~εῑς", "~α~α~εῑτον", "~α~α~εῑτε";
    ind_act_3: "~α~α~εῑ", _, "~α~α~οῡσῐ|~α~α~οῡσῐν";
    ind_mid_1: "~α~α~οῡμαι", _, "~α~α~ουμεθᾰ";
    ind_mid_2: "~α~α~εῑ|~α~α~ῃ", "~α~α~εῑσθον", "~α~α~εῑσθε";
    ind_mid_3: "~α~α~εῑται", _, "~α~α~οῡνται";
    subj_act_1: "~α~α~ω", _, "~α~α~ωμεν";
    subj_act_2: "~α~α~ῃς", "~α~α~ητον", "~α~α~ητε";
    subj_act_3: "~α~α~ῃ", _, "~α~α~ωσῐ|~α~α~ωσῐν";
    subj_mid_1: "~α~α~ωμαι", _, "~α~α~ωμεθᾰ";
    subj_mid_2: _, "~α~α~ησθον", "~α~α~ησθε";
    subj_mid_3: "~α~α~ηται", _, "~α~α~ωνται";
    opt_act_1: "~α~α~οιην|~α~α~οῑμῐ", _, "~α~α~οῑμεν|~α~α~οιημεν";
    opt_act_2: "~α~α~οιης|~α~α~οῑς", "~α~α~οῑτον|~α~α~οιητον", "~α~α~οῑτε|~α~α~οιητε";
    opt_act_3: "~α~α~οιη|~α~α~οῑ", "~α~α~οιτην|~α~α~οιητην", "~α~α~οῑεν|~α~α~οιησᾰν";
    opt_mid_1: "~α~α~οιμην", _, "~α~α~οιμεθᾰ";
    opt_mid_2: "~α~α~οῑο", "~α~α~οῑσθον", "~α~α~οῑσθε";
    opt_mid_3: "~α~α~οῑτο", "~α~α~οισθην", "~α~α~οῑντο";
    imp_act_2: "~α~α~ει", _, _;
    imp_act_3: "~α~α~ειτω", "~α~α~ειτων", "~α~α~ουντων";
    imp_mid_2: "~α~α~οῡ", _, _;
    imp_mid_3: "~α~α~εισθω", "~α~α~εισθων", _;
    imperfect_act_1: "~ε~α~ουν", _, "~ε~α~οῡμεν";
    imperfect_act_2: "~ε~α~εις", "~ε~α~εῑτον", "~ε~α~εῑτε";
    imperfect_act_3: "~ε~α~ει", "~ε~α~ειτην", "~ε~α~ουν";
    imperfect_mid_1: "~ε~α~ουμην", _, "~ε~α~ουμεθᾰ";
    imperfect_mid_2: "~ε~α~οῡ", "~ε~α~εῑσθον", "~ε~α~εῑσθε";
    imperfect_mid_3: "~ε~α~εῑτο", "~ε~α~εισθην", "~ε~α~οῡντο";
    future_ind_act_1: "~α~α~εσω", _, "~α~α~εσομεν";
    future_ind_act_2: "~α~α~εσεις", "~α~α~εσετον", "~α~α~εσετε";
    future_ind_act_3: "~α~α~εσει", _, "~α~α~εσουσῐ";
    future_ind_mid_1: "~α~α~εσομαι", _, "~α~α~εσομεθᾰ";
    future_ind_mid_2: "~α~α~εσῃ", "~α~α~εσεσθον", "~α~α~εσεσθε";
    future_ind_mid_3: "~α~α~εσεται", _, "~α~α~εσονται";
    future_ind_pass_1: "~α~α~εθησομαι", _, "~α~α~εθησομεθᾰ";
    future_ind_pass_2: "~α~α~εθησῃ", "~α~α~εθησεσθον", "~α~α~εθησεσθε";
    future_ind_pass_3: "~α~α~εθησεται", _, "~α~α~εθησονται";
    future_opt_act_1: "~α~α~εσοιμῐ", _, "~α~α~εσοιμεν";
    future_opt_act_2: "~α~α~εσοις", "~α~α~εσοιτον", "~α~α~εσοιτε";
    future_opt_act_3: "~α~α~εσοι", "~α~α~εσοιτην", "~α~α~εσοιεν";
    future_opt_mid_1: "~α~α~εσοιμην", _, "~α~α~εσοιμεθᾰ";
    future_opt_mid_2: "~α~α~εσοιο", "~α~α~εσοισθον", "~α~α~εσοισθε";
    future_opt_mid_3: "~α~α~εσοιτο", "~α~α~εσοισθην", "~α~α~εσοιντο";
    future_opt_pass_1: "~α~α~εθησοιμην", _, "~α~α~εθησοιμεθᾰ";
    future_opt_pass_2: "~α~α~εθησοιο", "~α~α~εθησοισθον", "~α~α~εθησοισθε";
    future_opt_pass_3: "~α~α~εθησοιτο", "~α~α~εθησοισθην", "~α~α~εθησοιντο";
    aorist_subj_act_1: "~α~α~εσω", _, "~α~α~εσωμεν";
    aorist_subj_act_2: "~α~α~εσῃς", "~α~α~εσητον", "~α~α~εσητε";
    aorist_subj_act_3: "~α~α~εσῃ", _, "~α~α~εσωσῐ";
    aorist_subj_mid_1: "~α~α~εσωμαι", _, "~α~α~εσωμεθᾰ";
    aorist_subj_mid_2: _, "~α~α~εσησθον", "~α~α~εσησθε";
    aorist_subj_mid_3: "~α~α~εσηται", _, "~α~α~εσωνται";
    aorist_subj_pass_1: "~α~α~εθω", _, "~α~α~εθωμεν";
    aorist_subj_pass_2: "~α~α~εθῃς", "~α~α~εθητον", "~α~α~εθητε";
    aorist_subj_pass_3: "~α~α~εθῃ", _, "~α~α~εθωσῐ";
    aorist_opt_act_1: "~α~α~εσαιμῐ", _, "~α~α~εσαιμεν";
    aorist_opt_act_2: "~α~α~εσειᾰς", "~α~α~εσαιτον", "~α~α~εσαιτε";
    aorist_opt_act_3: "~α~α~εσειε", "~α~α~εσαιτην", "~α~α~εσειᾰν";
    aorist_opt_mid_1: "~α~α~εσαιμην", _, "~α~α~εσαιμεθᾰ";
    aorist_opt_mid_2: "~α~α~εσαιο", "~α~α~εσαισθον", "~α~α~εσαισθε";
    aorist_opt_mid_3: "~α~α~εσαιτο", "~α~α~εσαισθην", "~α~α~εσαιντο";
    aorist_opt_pass_1: "~α~α~εθειην", _, "~α~α~εθεῑμεν";
    aorist_opt_pass_2: "~α~α~εθειης", "~α~α~εθεῑτον", "~α~α~εθεῑτε";
    aorist_opt_pass_3: "~α~α~εθειη", "~α~α~εθειτην", "~α~α~εθεῑεν";
    aorist_imp_act_2: "~α~α~εσον", "~α~α~εσᾰτον", "~α~α~εσᾰτε";
    aorist_imp_act_3: "~α~α~εσᾰτω", "~α~α~εσᾰτων", "~α~α~εσᾰντων";
    aorist_imp_mid_2: "~α~α~εσαι", "~α~α~εσᾰσθον", "~α~α~εσᾰσθε";
    aorist_imp_mid_3: "~α~α~εσᾰσθω", "~α~α~εσᾰσθων", _;
    aorist_imp_pass_2: "~α~α~εθητῐ", _, _;
    aorist_imp_pass_3: "~α~α~εθητω", "~α~α~εθητων", "~α~α~εθεντων";
    perfect_ind_act_1: "~α~εκα~εκᾰ", _, "~α~εκα~εκᾰμεν";
    perfect_ind_act_2: "~α~εκα~εκᾰς", "~α~εκα~εκᾰτον", "~α~εκα~εκᾰτε";
    perfect_ind_act_3: "~α~εκα~εκε", _, "~α~εκα~εκᾱσῐ";
    perfect_ind_mid_1: "~α~εκα~εμαι", _, "~α~εκα~εμεθᾰ";
    perfect_ind_mid_2: "~α~εκα~εσαι", "~α~εκα~εσθον", "~α~εκα~εσθε";
    perfect_ind_mid_3: "~α~εκα~εται", _, "~α~εκα~ενται";
    perfect_subj_act_1: "~α~εκα~εκω", _, "~α~εκα~εκωμεν";
    perfect_subj_act_2: "~α~εκα~εκῃς", "~α~εκα~εκητον", "~α~εκα~εκητε";
    perfect_subj_act_3: "~α~εκα~εκῃ", _, "~α~εκα~εκωσῐ";
    perfect_opt_act_1: "~α~εκα~εκοιμῐ", _, "~α~εκα~εκοιμεν";
    perfect_opt_act_2: "~α~εκα~εκοις", "~α~εκα~εκοιτον", "~α~εκα~εκοιτε";
    perfect_opt_act_3: "~α~εκα~εκοι", "~α~εκα~εκοιτην", "~α~εκα~εκοιεν";
    perfect_imp_act_2: _, "~α~εκα~εκετον", "~α~εκα~εκετε";
    perfect_imp_act_3: "~α~εκα~εκετω", "~α~εκα~εκετων", "~α~εκα~εκοντων";
    perfect_imp_mid_2: "~α~εκα~εσο", _, _;
    perfect_imp_mid_3: "~α~εκα~εσθω", "~α~εκα~εσθων", _;
    pluperfect_mid_1: "~ε~εκα~εμην", _, "~ε~εκα~εμεθᾰ";
    pluperfect_mid_2: "~ε~εκα~εσο", "~ε~εκα~εσθον", "~ε~εκα~εσθε";
    pluperfect_mid_3: "~ε~εκα~ετο", "~ε~εκα~εσθην", "~ε~εκα~εντο";
    infinitive_pres: "~α~α~εῑν", "~α~α~εῑσθαι", _;
    infinitive_fut: "~α~α~εσειν", "~α~α~εσεσθαι", "~α~α~εθησεσθαι";
    infinitive_aor: "~α~α~εσαι", "~α~α~εσᾰσθαι", "~α~α~εθηναι";
    infinitive_perf: "~α~εκα~εκεναι", "~α~εκα~εσθαι", _;
    participle_pres: "~α~α~ων", "~α~α~ουμενος", _;
    participle_fut: "~α~α~εσων", "~α~α~εσομενος", "~α~α~εθησομενος";
    participle_aor: "~α~α~εσᾱς", "~α~α~εσᾰμενος", "~α~α~εθεις";
    participle_perf: "~α~εκα~εκως", "~α~εκα~εμενος", _;
    ),
    // Ερεω: 2 words (ερεω, ετεω). Like Μῑγνῡμῐ, except in 88 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ερεω", 2, "ε~εω", like Μῑγνῡμῐ as SG_AS_PL;
    ind_act_1: "ἑ~ω", _, "ἑ~οῡμεν";
    ind_act_2: "ἑ~εῑς", "ἑ~εῑτον", "ἑ~εῑτε";
    ind_act_3: "ἑ~εῑ", _, "ἑ~οῡσῐ|ἑ~οῡσῐν";
    ind_mid_1: "ἑ~οῡμαι", _, "ἑ~ουμεθᾰ";
    ind_mid_2: "ἑ~εῑ|ἑ~ῃ", "ἑ~εῑσθον", "ἑ~εῑσθε";
    ind_mid_3: "ἑ~εῑται", _, "ἑ~οῡνται";
    subj_act_1: "ἑ~ω", _, "ἑ~ωμεν";
    subj_act_2: "ἑ~ῃς", "ἑ~ητον", "ἑ~ητε";
    subj_act_3: "ἑ~ῃ", _, "ἑ~ωσῐ|ἑ~ωσῐν";
    subj_mid_1: "ἑ~ωμαι", _, "ἑ~ωμεθᾰ";
    subj_mid_2: _, "ἑ~ησθον", "ἑ~ησθε";
    subj_mid_3: "ἑ~ηται", _, "ἑ~ωνται";
    opt_act_1: "ἑ~οιην|ἑ~οῑμῐ", _, "ἑ~οῑμεν|ἑ~οιημεν";
    opt_act_2: "ἑ~οιης|ἑ~οῑς", "ἑ~οῑτον|ἑ~οιητον", "ἑ~οῑτε|ἑ~οιητε";
    opt_act_3: "ἑ~οιη|ἑ~οῑ", "ἑ~οιτην|ἑ~οιητην", "ἑ~οῑεν|ἑ~οιησᾰν";
    opt_mid_1: "ἑ~οιμην", _, "ἑ~οιμεθᾰ";
    opt_mid_2: "ἑ~οῑο", "ἑ~οῑσθον", "ἑ~οῑσθε";
    opt_mid_3: "ἑ~οῑτο", "ἑ~οισθην", "ἑ~οῑντο";
    imp_act_2: "ἑ~ει", _, _;
    imp_act_3: "ἑ~ειτω", "ἑ~ειτων", "ἑ~ουντων";
    imp_mid_2: "ἑ~οῡ", _, _;
    imp_mid_3: "ἑ~εισθω", "ἑ~εισθων", _;
    future_ind_act_1: "ἑ~ησω", _, "ἑ~ησομεν";
    future_ind_act_2: "ἑ~ησεις", "ἑ~ησετον", "ἑ~ησετε";
    future_ind_act_3: "ἑ~ησει", _, "ἑ~ησουσῐ";
    future_ind_mid_1: "ἑ~ησομαι", _, "ἑ~ησομεθᾰ";
    future_ind_mid_2: "ἑ~ησῃ", "ἑ~ησεσθον", "ἑ~ησεσθε";
    future_ind_mid_3: "ἑ~ησεται", _, "ἑ~ησονται";
    future_opt_act_1: "ἑ~ησοιμῐ", _, "ἑ~ησοιμεν";
    future_opt_act_2: "ἑ~ησοις", "ἑ~ησοιτον", "ἑ~ησοιτε";
    future_opt_act_3: "ἑ~ησοι", "ἑ~ησοιτην", "ἑ~ησοιεν";
    future_opt_mid_1: "ἑ~ησοιμην", _, "ἑ~ησοιμεθᾰ";
    future_opt_mid_2: "ἑ~ησοιο", "ἑ~ησοισθον", "ἑ~ησοισθε";
    future_opt_mid_3: "ἑ~ησοιτο", "ἑ~ησοισθην", "ἑ~ησοιντο";
    infinitive_pres: "ἑ~εῑν", "ἑ~εῑσθαι", _;
    infinitive_fut: "ἑ~ησειν", "ἑ~ησεσθαι", _;
    participle_pres: "ἑ~ων", "ἑ~ουμενος", _;
    participle_fut: "ἑ~ησων", "ἑ~ησομενος", _;
    ),
    // Ἐδω: 1 words (ἐδω). Like Γεγονοι, except in 68 cells: ind_act_1 sg, future_ind_mid_1 sg, future_ind_mid_1 pl, future_ind_mid_2 sg, future_ind_mid_2 du, future_ind_mid_2 pl, ….
    conjugation!("ἐδω", 1, "~ω", like Γεγονοι;
    stem pluperfect_act: "~ηδοκ- like εὐλογεω";
    ind_act_1: "~ω", _, _;
    future_ind_mid_1: "~ομαι", _, "~ομεθᾰ";
    future_ind_mid_2: "~ῃ|~ει", "~εσθον", "~εσθε";
    future_ind_mid_3: "~εται", _, "~ονται";
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_2: "~οιο", "~οισθον", "~οισθε";
    future_opt_mid_3: "~οιτο", "~οισθην", "~οιντο";
    perfect_ind_act_1: "~ηδοκᾰ|~ηδᾰ", _, "~ηδοκᾰμεν|~ηδᾰμεν";
    perfect_ind_act_2: "~ηδοκᾰς|~ηδᾰς", "~ηδοκᾰτον|~ηδᾰτον", "~ηδοκᾰτε|~ηδᾰτε";
    perfect_ind_act_3: "~ηδοκε|~ηδοκεν|~ηδε|~ηδεν", _, "~ηδοκᾱσῐ|~ηδοκᾱσῐν|~ηδᾱσῐ|~ηδᾱσῐν";
    perfect_ind_mid_1: "~ηδομαι", _, "~ηδομεθᾰ";
    perfect_ind_mid_2: "~ηδοσαι", "~ηδοσθον", "~ηδοσθε";
    perfect_ind_mid_3: "~ηδοται", _, "~ηδονται";
    perfect_subj_act_1: "~ηδοκω|~ηδω", _, "~ηδοκωμεν|~ηδωμεν";
    perfect_subj_act_2: "~ηδοκῃς|~ηδῃς", "~ηδοκητον|~ηδητον", "~ηδοκητε|~ηδητε";
    perfect_subj_act_3: "~ηδοκῃ|~ηδῃ", _, "~ηδοκωσῐ|~ηδοκωσῐν|~ηδωσῐ|~ηδωσῐν";
    perfect_opt_act_1: "~ηδοκοιμῐ|~ηδοκοιην|~ηδοιμῐ|~ηδοιην", _, "~ηδοκοιμεν|~ηδοιμεν";
    perfect_opt_act_2: "~ηδοκοις|~ηδοκοιης|~ηδοις|~ηδοιης", "~ηδοκοιτον|~ηδοιτον", "~ηδοκοιτε|~ηδοιτε";
    perfect_opt_act_3: "~ηδοκοι|~ηδοκοιη|~ηδοι|~ηδοιη", "~ηδοκοιτην|~ηδοιτην", "~ηδοκοιεν|~ηδοιεν";
    perfect_imp_act_2: "~ηδοκε|~ηδε", "~ηδοκετον|~ηδετον", "~ηδοκετε|~ηδετε";
    perfect_imp_act_3: "~ηδοκετω|~ηδετω", "~ηδοκετων|~ηδετων", "~ηδοκοντων|~ηδοντων";
    perfect_imp_mid_2: "~ηδοσο", _, _;
    perfect_imp_mid_3: "~ηδοσθω", "~ηδοσθων", _;
    pluperfect_mid_1: "~ηδομην", _, "~ηδομεθᾰ";
    pluperfect_mid_2: "~ηδοσο", "~ηδοσθον", "~ηδοσθε";
    pluperfect_mid_3: "~ηδοτο", "~ηδοσθην", "~ηδοντο";
    infinitive_fut: _, "~εσθαι", _;
    infinitive_perf: "~ηδοκεναι|~ηδεναι", "~ηδοσθαι", _;
    participle_fut: _, "~ομενος", _;
    participle_perf: "~ηδοκως|~ηδως", "~ηδομενος", _;
    ),
    // Αἰσθᾰνομαι: 1 words (αἰσθᾰνομαι). Like Οἰομαι, except in 142 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("αἰσθᾰνομαι", 1, "αἰ~ᾰνομαι", like Οἰομαι;
    stem aorist_ind_pass: "~ηθη- like αἰσθᾰνομαι";
    ind_act_1: "αἰ~ᾰνομαι", _, _;
    ind_mid_1: "αἰ~ανομαι", _, "αἰ~ανομεθᾰ";
    ind_mid_2: "αἰ~ανῃ|αἰ~ανει", "αἰ~ανεσθον", "αἰ~ανεσθε";
    ind_mid_3: "αἰ~ανεται", _, "αἰ~ανονται";
    subj_mid_1: "αἰ~ανωμαι", _, "αἰ~ανωμεθᾰ";
    subj_mid_2: "αἰ~ανῃ", "αἰ~ανησθον", "αἰ~ανησθε";
    subj_mid_3: "αἰ~ανηται", _, "αἰ~ανωνται";
    subj_pass_2: "αἰ~ανῃ", _, _;
    opt_mid_1: "αἰ~ανοιμην", _, "αἰ~ανοιμεθᾰ";
    opt_mid_2: "αἰ~ανοιο", "αἰ~ανοισθον", "αἰ~ανοισθε";
    opt_mid_3: "αἰ~ανοιτο", "αἰ~ανοισθην", "αἰ~ανοιντο";
    imp_mid_2: "αἰ~ανου", _, _;
    imp_mid_3: "αἰ~ανεσθω", "αἰ~ανεσθων", _;
    imperfect_mid_1: "ᾐ~ανομην", _, "ᾐ~ανομεθᾰ";
    imperfect_mid_2: "ᾐ~ανου", "ᾐ~ανεσθον", "ᾐ~ανεσθε";
    imperfect_mid_3: "ᾐ~ανετο", "ᾐ~ανεσθην", "ᾐ~ανοντο";
    future_ind_mid_1: "αἰ~ησομαι", _, "αἰ~ησομεθᾰ";
    future_ind_mid_2: "αἰ~ησῃ|αἰ~ησει", "αἰ~ησεσθον", "αἰ~ησεσθε";
    future_ind_mid_3: "αἰ~ησεται", _, "αἰ~ησονται";
    future_ind_pass_1: "αἰ~ηθησομαι|αἰ~ανθησομαι", _, "αἰ~ηθησομεθᾰ|αἰ~ανθησομεθᾰ";
    future_ind_pass_2: "αἰ~ηθησῃ|αἰ~ηθησει|αἰ~ανθησῃ|αἰ~ανθησει", "αἰ~ηθησεσθον|αἰ~ανθησεσθον", "αἰ~ηθησεσθε|αἰ~ανθησεσθε";
    future_ind_pass_3: "αἰ~ηθησεται|αἰ~ανθησεται", _, "αἰ~ηθησονται|αἰ~ανθησονται";
    future_opt_mid_1: "αἰ~ησοιμην", _, "αἰ~ησοιμεθᾰ";
    future_opt_mid_2: "αἰ~ησοιο", "αἰ~ησοισθον", "αἰ~ησοισθε";
    future_opt_mid_3: "αἰ~ησοιτο", "αἰ~ησοισθην", "αἰ~ησοιντο";
    future_opt_pass_1: "αἰ~ηθησοιμην|αἰ~ανθησοιμην", _, "αἰ~ηθησοιμεθᾰ|αἰ~ανθησοιμεθᾰ";
    future_opt_pass_2: "αἰ~ηθησοιο|αἰ~ανθησοιο", "αἰ~ηθησοισθον|αἰ~ανθησοισθον", "αἰ~ηθησοισθε|αἰ~ανθησοισθε";
    future_opt_pass_3: "αἰ~ηθησοιτο|αἰ~ανθησοιτο", "αἰ~ηθησοισθην|αἰ~ανθησοισθην", "αἰ~ηθησοιντο|αἰ~ανθησοιντο";
    aorist_ind_mid_1: "ᾐ~ομην|ᾐ~ησᾰμην", _, "ᾐ~ομεθᾰ|ᾐ~ησᾰμεθᾰ";
    aorist_ind_mid_2: "ᾐ~ου|ᾐ~ησω", "ᾐ~εσθον|ᾐ~ησᾰσθον", "ᾐ~εσθε|ᾐ~ησᾰσθε";
    aorist_ind_mid_3: "ᾐ~ετο|ᾐ~ησᾰτο", "ᾐ~εσθην|ᾐ~ησᾰσθην", "ᾐ~οντο|ᾐ~ησᾰντο";
    aorist_subj_mid_1: "αἰ~ωμαι|αἰ~ησωμαι", _, "αἰ~ωμεθᾰ|αἰ~ησωμεθᾰ";
    aorist_subj_mid_2: "αἰ~ῃ|αἰ~ησῃ", "αἰ~ησθον|αἰ~ησησθον", "αἰ~ησθε|αἰ~ησησθε";
    aorist_subj_mid_3: "αἰ~ηται|αἰ~ησηται", _, "αἰ~ωνται|αἰ~ησωνται";
    aorist_subj_pass_1: "αἰ~ηθω", _, "αἰ~ηθωμεν";
    aorist_subj_pass_2: "αἰ~ηθῃς", "αἰ~ηθητον", "αἰ~ηθητε";
    aorist_subj_pass_3: "αἰ~ηθῃ", _, "αἰ~ηθωσῐ|αἰ~ηθωσῐν";
    aorist_opt_mid_1: "αἰ~οιμην|αἰ~ησαιμην", _, "αἰ~οιμεθᾰ|αἰ~ησαιμεθᾰ";
    aorist_opt_mid_2: "αἰ~οιο|αἰ~ησαιο", "αἰ~οισθον|αἰ~ησαισθον", "αἰ~οισθε|αἰ~ησαισθε";
    aorist_opt_mid_3: "αἰ~οιτο|αἰ~ησαιτο", "αἰ~οισθην|αἰ~ησαισθην", "αἰ~οιντο|αἰ~ησαιντο";
    aorist_opt_pass_1: "αἰ~ηθειην", _, "αἰ~ηθεῑμεν|αἰ~ηθειημεν";
    aorist_opt_pass_2: "αἰ~ηθειης", "αἰ~ηθεῑτον|αἰ~ηθειητον", "αἰ~ηθεῑτε|αἰ~ηθειητε";
    aorist_opt_pass_3: "αἰ~ηθειη", "αἰ~ηθειτην|αἰ~ηθειητην", "αἰ~ηθεῑεν|αἰ~ηθειησᾰν";
    aorist_imp_mid_2: "αἰ~οῡ|αἰ~ησαι", "αἰ~εσθον|αἰ~ησᾰσθον", "αἰ~εσθε|αἰ~ησᾰσθε";
    aorist_imp_mid_3: "αἰ~εσθω|αἰ~ησᾰσθω", "αἰ~εσθων|αἰ~ησᾰσθων", _;
    aorist_imp_pass_2: "αἰ~ηθητῐ", _, _;
    aorist_imp_pass_3: "αἰ~ηθητω", "αἰ~ηθητων", "αἰ~ηθεντων";
    perfect_ind_mid_1: "ᾐ~ημαι", _, "ᾐ~ημεθᾰ";
    perfect_ind_mid_2: "ᾐ~ησαι", "ᾐ~ησθον", "ᾐ~ησθε";
    perfect_ind_mid_3: "ᾐ~ηται", _, "ᾐ~ηνται";
    perfect_imp_mid_2: "ᾐ~ησο", _, _;
    perfect_imp_mid_3: "ᾐ~ησθω", "ᾐ~ησθων", _;
    pluperfect_mid_1: "ᾐ~ημην", _, "ᾐ~ημεθᾰ";
    pluperfect_mid_2: "ᾐ~ησο", "ᾐ~ησθον", "ᾐ~ησθε";
    pluperfect_mid_3: "ᾐ~ητο", "ᾐ~ησθην", "ᾐ~ηντο";
    infinitive_pres: _, "αἰ~ανεσθαι", _;
    infinitive_fut: _, "αἰ~ησεσθαι", "αἰ~ηθησεσθαι|αἰ~ανθησεσθαι";
    infinitive_aor: _, "αἰ~εσθαι|αἰ~ησᾰσθαι", "αἰ~ηθηναι";
    infinitive_perf: _, "ᾐ~ησθαι", _;
    participle_pres: _, "αἰ~ανομενος", _;
    participle_fut: _, "αἰ~ησομενος", "αἰ~ηθησομενος|αἰ~ανθησομενος";
    participle_aor: _, "αἰ~ομενος|αἰ~ησᾰμενος", "αἰ~ηθεις";
    participle_perf: _, "ᾐ~ημενος", _;
    ),
    // Τρυπαω: 1 words (τρυπαω). Like Ἐκφερω, except in 156 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("τρυπαω", 1, "~υ~αω", like Ἐκφερω;
    stem aorist_ind: "~ῡ~ησ- like ποιεω";
    stem pluperfect_act: "~ῡ~ηκ- like χρημᾰτῐζω";
    stem pluperfect: "~ῡ~η- like πειρᾰω";
    ind_act_1: "~ῡ~ω", _, "~ῡ~ωμεν";
    ind_act_2: "~ῡ~ᾳς", "~ῡ~ᾱτον", "~ῡ~ᾱτε";
    ind_act_3: "~ῡ~ᾳ", _, "~ῡ~ωσῐ|~ῡ~ωσῐν";
    ind_mid_1: "~ῡ~ωμαι", _, "~ῡ~ωμεθᾰ";
    ind_mid_2: "~ῡ~ᾳ", "~ῡ~ᾱσθον", "~ῡ~ᾱσθε";
    ind_mid_3: "~ῡ~ᾱται", _, "~ῡ~ωνται";
    subj_act_1: "~ῡ~ω", _, "~ῡ~ωμεν";
    subj_act_2: "~ῡ~ᾳς", "~ῡ~ᾱτον", "~ῡ~ᾱτε";
    subj_act_3: "~ῡ~ᾳ", _, "~ῡ~ωσῐ|~ῡ~ωσῐν";
    subj_mid_1: "~ῡ~ωμαι", _, "~ῡ~ωμεθᾰ";
    subj_mid_2: _, "~ῡ~ᾱσθον", "~ῡ~ᾱσθε";
    subj_mid_3: "~ῡ~ᾱται", _, "~ῡ~ωνται";
    opt_act_1: "~ῡ~ῳην|~ῡ~ῳμῐ", _, "~ῡ~ῳμεν|~ῡ~ῳημεν";
    opt_act_2: "~ῡ~ῳης|~ῡ~ῳς", "~ῡ~ῳτον|~ῡ~ῳητον", "~ῡ~ῳτε|~ῡ~ῳητε";
    opt_act_3: "~ῡ~ῳη|~ῡ~ῳ", "~ῡ~ῳτην|~ῡ~ῳητην", "~ῡ~ῳεν|~ῡ~ῳησᾰν";
    opt_mid_1: "~ῡ~ῳμην", _, "~ῡ~ῳμεθᾰ";
    opt_mid_2: "~ῡ~ῳο", "~ῡ~ῳσθον", "~ῡ~ῳσθε";
    opt_mid_3: "~ῡ~ῳτο", "~ῡ~ῳσθην", "~ῡ~ῳντο";
    imp_act_2: "~ῡ~ᾱ", _, _;
    imp_act_3: "~ῡ~ᾱτω", "~ῡ~ᾱτων", "~ῡ~ωντων";
    imp_mid_2: "~ῡ~ω", _, _;
    imp_mid_3: "~ῡ~ᾱσθω", "~ῡ~ᾱσθων", _;
    imperfect_act_1: "ἐ~ῡ~ων", _, "ἐ~ῡ~ωμεν";
    imperfect_act_2: "ἐ~ῡ~ᾱς", "ἐ~ῡ~ᾱτον", "ἐ~ῡ~ᾱτε";
    imperfect_act_3: "ἐ~ῡ~ᾱ", "ἐ~ῡ~ᾱτην", "ἐ~ῡ~ων";
    imperfect_mid_1: "ἐ~ῡ~ωμην", _, "ἐ~ῡ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ῡ~ω", "ἐ~ῡ~ᾱσθον", "ἐ~ῡ~ᾱσθε";
    imperfect_mid_3: "ἐ~ῡ~ᾱτο", "ἐ~ῡ~ᾱσθην", "ἐ~ῡ~ωντο";
    aorist_subj_act_1: "~ῡ~ησω", _, "~ῡ~ησωμεν";
    aorist_subj_act_2: "~ῡ~ησῃς", "~ῡ~ησητον", "~ῡ~ησητε";
    aorist_subj_act_3: "~ῡ~ησῃ", _, "~ῡ~ησωσῐ|~ῡ~ησωσῐν";
    aorist_subj_mid_1: "~ῡ~ησωμαι", _, "~ῡ~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~ῡ~ησησθον", "~ῡ~ησησθε";
    aorist_subj_mid_3: "~ῡ~ησηται", _, "~ῡ~ησωνται";
    aorist_opt_act_1: "~ῡ~ησαιμῐ", _, "~ῡ~ησαιμεν";
    aorist_opt_act_2: "~ῡ~ησειᾰς|~ῡ~ησαις", "~ῡ~ησαιτον", "~ῡ~ησαιτε";
    aorist_opt_act_3: "~ῡ~ησειε|~ῡ~ησειεν|~ῡ~ησαι", "~ῡ~ησαιτην", "~ῡ~ησειᾰν|~ῡ~ησαιεν";
    aorist_opt_mid_1: "~ῡ~ησαιμην", _, "~ῡ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ῡ~ησαιο", "~ῡ~ησαισθον", "~ῡ~ησαισθε";
    aorist_opt_mid_3: "~ῡ~ησαιτο", "~ῡ~ησαισθην", "~ῡ~ησαιντο";
    aorist_imp_act_2: "~ῡ~ησον", "~ῡ~ησᾰτον", "~ῡ~ησᾰτε";
    aorist_imp_act_3: "~ῡ~ησᾰτω", "~ῡ~ησᾰτων", "~ῡ~ησᾰντων";
    aorist_imp_mid_2: "~ῡ~ησαι", "~ῡ~ησᾰσθον", "~ῡ~ησᾰσθε";
    aorist_imp_mid_3: "~ῡ~ησᾰσθω", "~ῡ~ησᾰσθων", _;
    perfect_ind_act_1: "%ε~ῡ~ηκᾰ", _, "%ε~ῡ~ηκᾰμεν";
    perfect_ind_act_2: "%ε~ῡ~ηκᾰς", "%ε~ῡ~ηκᾰτον", "%ε~ῡ~ηκᾰτε";
    perfect_ind_act_3: "%ε~ῡ~ηκε|%ε~ῡ~ηκεν", _, "%ε~ῡ~ηκᾱσῐ|%ε~ῡ~ηκᾱσῐν";
    perfect_ind_mid_1: "%ε~ῡ~ημαι", _, "%ε~ῡ~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ῡ~ησαι", "%ε~ῡ~ησθον", "%ε~ῡ~ησθε";
    perfect_ind_mid_3: "%ε~ῡ~ηται", _, "%ε~ῡ~ηνται";
    perfect_subj_act_1: "%ε~ῡ~ηκω", _, "%ε~ῡ~ηκωμεν";
    perfect_subj_act_2: "%ε~ῡ~ηκῃς", "%ε~ῡ~ηκητον", "%ε~ῡ~ηκητε";
    perfect_subj_act_3: "%ε~ῡ~ηκῃ", _, "%ε~ῡ~ηκωσῐ|%ε~ῡ~ηκωσῐν";
    perfect_opt_act_1: "%ε~ῡ~ηκοιμῐ|%ε~ῡ~ηκοιην", _, "%ε~ῡ~ηκοιμεν";
    perfect_opt_act_2: "%ε~ῡ~ηκοις|%ε~ῡ~ηκοιης", "%ε~ῡ~ηκοιτον", "%ε~ῡ~ηκοιτε";
    perfect_opt_act_3: "%ε~ῡ~ηκοι|%ε~ῡ~ηκοιη", "%ε~ῡ~ηκοιτην", "%ε~ῡ~ηκοιεν";
    perfect_imp_act_2: "%ε~ῡ~ηκε", "%ε~ῡ~ηκετον", "%ε~ῡ~ηκετε";
    perfect_imp_act_3: "%ε~ῡ~ηκετω", "%ε~ῡ~ηκετων", "%ε~ῡ~ηκοντων";
    perfect_imp_mid_2: "%ε~ῡ~ησο", _, _;
    perfect_imp_mid_3: "%ε~ῡ~ησθω", "%ε~ῡ~ησθων", _;
    infinitive_pres: "~ῡ~ᾱν", "~ῡ~ᾱσθαι", _;
    infinitive_aor: "~ῡ~ησαι", "~ῡ~ησᾰσθαι", _;
    infinitive_perf: "%ε~ῡ~ηκεναι", "%ε~ῡ~ησθαι", _;
    participle_pres: "~ῡ~ων", "~ῡ~ωμενος", _;
    participle_aor: "~ῡ~ησᾱς", "~ῡ~ησᾰμενος", _;
    participle_perf: "%ε~ῡ~ηκως", "%ε~ῡ~ημενος", _;
    ),
    // Μειδᾰω: 1 words (μειδᾰω). Like Μῑγνῡμῐ, except in 51 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("μειδᾰω", 1, "~ᾰω", like Μῑγνῡμῐ;
    stem imperfect: "~ι- like μειδᾰω";
    stem aorist_ind: "~ιᾱσ- like χᾰριζω";
    ind_act_1: "~ιω", _, "~ιωμεν";
    ind_act_2: "~ιᾳς", "~ιᾱτον", "~ιᾱτε";
    ind_act_3: "~ιᾳ", _, "~ιωσῐ|~ιωσῐν";
    subj_act_1: "~ιω", _, "~ιωμεν";
    subj_act_2: "~ιᾳς", "~ιᾱτον", "~ιᾱτε";
    subj_act_3: "~ιᾳ", _, "~ιωσῐ|~ιωσῐν";
    opt_act_1: "~ιῳην|~ιῳμῐ", _, "~ιῳμεν|~ιῳημεν";
    opt_act_2: "~ιῳης|~ιῳς", "~ιῳτον|~ιῳητον", "~ιῳτε|~ιῳητε";
    opt_act_3: "~ιῳη|~ιῳ", "~ιῳτην|~ιῳητην", "~ιῳεν|~ιῳησᾰν";
    imp_act_2: "~ιᾱ", _, _;
    imp_act_3: "~ιᾱτω", "~ιᾱτων", "~ιωντων";
    aorist_subj_act_1: "~ιᾱσω", _, "~ιᾱσωμεν";
    aorist_subj_act_2: "~ιᾱσῃς", "~ιᾱσητον", "~ιᾱσητε";
    aorist_subj_act_3: "~ιᾱσῃ", _, "~ιᾱσωσῐ|~ιᾱσωσῐν";
    aorist_opt_act_1: "~ιᾱσαιμῐ", _, "~ιᾱσαιμεν";
    aorist_opt_act_2: "~ιᾱσειᾰς|~ιᾱσαις", "~ιᾱσαιτον", "~ιᾱσαιτε";
    aorist_opt_act_3: "~ιᾱσειε|~ιᾱσειεν|~ιᾱσαι", "~ιᾱσαιτην", "~ιᾱσειᾰν|~ιᾱσαιεν";
    aorist_imp_act_2: "~ιᾱσον", "~ιᾱσᾰτον", "~ιᾱσᾰτε";
    aorist_imp_act_3: "~ιᾱσᾰτω", "~ιᾱσᾰτων", "~ιᾱσᾰντων";
    infinitive_pres: "~ιᾱν", _, _;
    infinitive_aor: "~ιᾱσαι", _, _;
    participle_pres: "~ιων", _, _;
    participle_aor: "~ιᾱσᾱς", _, _;
    ),
    // Κεῑμαι: 1 words (κεῑμαι). Like Δεῑ, except in 45 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("κεῑμαι", 1, "~ῑμαι", like Δεῑ as ACT_AS_PASS;
    ind_act_1: "~ῑμαι", _, _;
    ind_mid_1: "~ῑμαι", _, "~ιμεθα";
    ind_mid_2: "~ῑσαι", "~ῑσθον", "~ῑσθε";
    ind_mid_3: "~ῑται", _, "~ῑνται|~ᾰται|~ονται|~ιᾰται";
    subj_mid_1: _, _, "~ωμεθα";
    subj_mid_2: "~ῃ", _, _;
    subj_mid_3: "~ηται|~ῑται", _, "~ωνται|~ιωνται";
    opt_mid_1: _, _, "~οιμεθα";
    opt_mid_2: _, derived, _;
    opt_mid_3: _, derived, _;
    imp_mid_2: "~ῑσο", _, _;
    imp_mid_3: "~ισθω", "~ισθων", _;
    imperfect_mid_1: "ἐ~ιμην", _, "ἐ~ιμεθα";
    imperfect_mid_2: "ἐ~ισο", "ἐ~ισθον", "ἐ~ισθε";
    imperfect_mid_3: "ἐ~ιτο", "ἐ~ισθην", "ἐ~ιντο|ἐ~ατο";
    future_ind_mid_1: "~ισομαι", _, "~ισομεθᾰ";
    future_ind_mid_2: "~ισῃ|~ισει", "~ισεσθον", "~ισεσθε";
    future_ind_mid_3: "~ισεται", _, "~ισονται";
    future_opt_mid_1: "~ισοιμην", _, "~ισοιμεθᾰ";
    future_opt_mid_2: "~ισοιο", "~ισοισθον", "~ισοισθε";
    future_opt_mid_3: "~ισοιτο", "~ισοισθην", "~ισοιντο";
    infinitive_pres: _, derived, _;
    infinitive_fut: _, "~ισεσθαι", _;
    participle_pres: _, derived, _;
    participle_fut: _, "~ισομενος", _;
    ),
    // Ἀνᾰχρονῐζομαι: 1 words (ἀνᾰχρονῐζομαι). Like Δερκομαι, except in 118 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἀνᾰχρονῐζομαι", 1, "~ᾰ~ζομαι", like Δερκομαι;
    stem imperfect: "~ε~ζ- like ὑποκοριζομαι";
    stem aorist_ind: "~ε~σ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ε~σθη- like ἐλπω";
    stem pluperfect: "~ε%ε~- like ὑποκοριζομαι";
    ind_act_1: "~ᾰ~ζομαι", _, _;
    ind_mid_1: "~α~ζομαι", _, "~α~ζομεθᾰ";
    ind_mid_2: "~α~ζῃ|~α~ζει", "~α~ζεσθον", "~α~ζεσθε";
    ind_mid_3: "~α~ζεται", _, "~α~ζονται";
    subj_mid_1: "~α~ζωμαι", _, "~α~ζωμεθᾰ";
    subj_mid_2: "~α~ζῃ", "~α~ζησθον", "~α~ζησθε";
    subj_mid_3: "~α~ζηται", _, "~α~ζωνται";
    subj_pass_2: "~α~ζῃ", _, _;
    opt_mid_1: "~α~ζοιμην", _, "~α~ζοιμεθᾰ";
    opt_mid_2: "~α~ζοιο", "~α~ζοισθον", "~α~ζοισθε";
    opt_mid_3: "~α~ζοιτο", "~α~ζοισθην", "~α~ζοιντο";
    imp_mid_2: "~α~ζου", _, _;
    imp_mid_3: "~α~ζεσθω", "~α~ζεσθων", _;
    future_ind_mid_1: "~α~οῡμαι", _, "~α~ουμεθᾰ";
    future_ind_mid_2: "~α~εῑ|~α~ῃ", "~α~εῑσθον", "~α~εῑσθε";
    future_ind_mid_3: "~α~εῑται", _, "~α~οῡνται";
    future_ind_pass_1: "~α~σθησομαι", _, "~α~σθησομεθᾰ";
    future_ind_pass_2: "~α~σθησῃ|~α~σθησει", "~α~σθησεσθον", "~α~σθησεσθε";
    future_ind_pass_3: "~α~σθησεται", _, "~α~σθησονται";
    future_opt_mid_1: "~α~οιμην", _, "~α~οιμεθᾰ";
    future_opt_mid_2: "~α~οῑο", "~α~οῑσθον", "~α~οῑσθε";
    future_opt_mid_3: "~α~οῑτο", "~α~οισθην", "~α~οῑντο";
    future_opt_pass_1: "~α~σθησοιμην", _, "~α~σθησοιμεθᾰ";
    future_opt_pass_2: "~α~σθησοιο", "~α~σθησοισθον", "~α~σθησοισθε";
    future_opt_pass_3: "~α~σθησοιτο", "~α~σθησοισθην", "~α~σθησοιντο";
    aorist_subj_mid_1: "~α~σωμαι", _, "~α~σωμεθᾰ";
    aorist_subj_mid_2: "~α~σῃ", "~α~σησθον", "~α~σησθε";
    aorist_subj_mid_3: "~α~σηται", _, "~α~σωνται";
    aorist_subj_pass_1: "~α~σθω", _, "~α~σθωμεν";
    aorist_subj_pass_2: "~α~σθῃς", "~α~σθητον", "~α~σθητε";
    aorist_subj_pass_3: "~α~σθῃ", _, "~α~σθωσῐ|~α~σθωσῐν";
    aorist_opt_mid_1: "~α~σαιμην", _, "~α~σαιμεθᾰ";
    aorist_opt_mid_2: "~α~σαιο", "~α~σαισθον", "~α~σαισθε";
    aorist_opt_mid_3: "~α~σαιτο", "~α~σαισθην", "~α~σαιντο";
    aorist_opt_pass_1: "~α~σθειην", _, "~α~σθεῑμεν|~α~σθειημεν";
    aorist_opt_pass_2: "~α~σθειης", "~α~σθεῑτον|~α~σθειητον", "~α~σθεῑτε|~α~σθειητε";
    aorist_opt_pass_3: "~α~σθειη", "~α~σθειτην|~α~σθειητην", "~α~σθεῑεν|~α~σθειησᾰν";
    aorist_imp_mid_2: "~α~σαι", "~α~σᾰσθον", "~α~σᾰσθε";
    aorist_imp_mid_3: "~α~σᾰσθω", "~α~σᾰσθων", _;
    aorist_imp_pass_2: "~α~σθητῐ", _, _;
    aorist_imp_pass_3: "~α~σθητω", "~α~σθητων", "~α~σθεντων";
    perfect_ind_mid_1: "~α%ε~σμαι", _, "~α%ε~σμεθᾰ";
    perfect_ind_mid_2: "~α%ε~σαι", "~α%ε~σθον", "~α%ε~σθε";
    perfect_ind_mid_3: "~α%ε~σται", _, "~α%ε~δᾰται";
    perfect_imp_mid_2: "~α%ε~σο", _, _;
    perfect_imp_mid_3: "~α%ε~σθω", "~α%ε~σθων", _;
    infinitive_pres: _, "~α~ζεσθαι", _;
    infinitive_fut: _, "~α~εῑσθαι", "~α~σθησεσθαι";
    infinitive_aor: _, "~α~σᾰσθαι", "~α~σθηναι";
    infinitive_perf: _, "~α%ε~σθαι", _;
    participle_pres: _, "~α~ζομενος", _;
    participle_fut: _, "~α~ουμενος", "~α~σθησομενος";
    participle_aor: _, "~α~σᾰμενος", "~α~σθεις";
    participle_perf: _, "~α%ε~σμενος", _;
    ),
    // Σῠμμᾰχομαι: 1 words (σῠμμᾰχομαι). Like Νισσομαι, except in 68 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("σῠμμᾰχομαι", 1, "~ῠμ~ομαι", like Νισσομαι;
    stem aorist_ind: "~υνε~εσ- like μηκᾰομαι";
    ind_act_1: "~ῠμ~ομαι", _, _;
    ind_mid_1: "~υμ~ομαι", _, "~υμ~ομεθᾰ";
    ind_mid_2: "~υμ~ῃ|~υμ~ει", "~υμ~εσθον", "~υμ~εσθε";
    ind_mid_3: "~υμ~εται", _, "~υμ~ονται";
    subj_mid_1: "~υμ~ωμαι", _, "~υμ~ωμεθᾰ";
    subj_mid_2: "~υμ~ῃ", "~υμ~ησθον", "~υμ~ησθε";
    subj_mid_3: "~υμ~ηται", _, "~υμ~ωνται";
    subj_pass_2: "~υμ~ῃ", _, _;
    opt_mid_1: "~υμ~οιμην", _, "~υμ~οιμεθᾰ";
    opt_mid_2: "~υμ~οιο", "~υμ~οισθον", "~υμ~οισθε";
    opt_mid_3: "~υμ~οιτο", "~υμ~οισθην", "~υμ~οιντο";
    imp_mid_2: "~υμ~ου", _, _;
    imp_mid_3: "~υμ~εσθω", "~υμ~εσθων", _;
    future_ind_mid_1: "~υμ~οῡμαι", _, "~υμ~ουμεθᾰ";
    future_ind_mid_2: "~υμ~εῑ|~υμ~ῃ", "~υμ~εῑσθον", "~υμ~εῑσθε";
    future_ind_mid_3: "~υμ~εῑται", _, "~υμ~οῡνται";
    future_opt_mid_1: "~υμ~οιμην", _, "~υμ~οιμεθᾰ";
    future_opt_mid_2: "~υμ~οῑο", "~υμ~οῑσθον", "~υμ~οῑσθε";
    future_opt_mid_3: "~υμ~οῑτο", "~υμ~οισθην", "~υμ~οῑντο";
    aorist_subj_mid_1: "~υμ~εσωμαι", _, "~υμ~εσωμεθᾰ";
    aorist_subj_mid_2: "~υμ~εσῃ", "~υμ~εσησθον", "~υμ~εσησθε";
    aorist_subj_mid_3: "~υμ~εσηται", _, "~υμ~εσωνται";
    aorist_opt_mid_1: "~υμ~εσαιμην", _, "~υμ~εσαιμεθᾰ";
    aorist_opt_mid_2: "~υμ~εσαιο", "~υμ~εσαισθον", "~υμ~εσαισθε";
    aorist_opt_mid_3: "~υμ~εσαιτο", "~υμ~εσαισθην", "~υμ~εσαιντο";
    aorist_imp_mid_2: "~υμ~εσαι", "~υμ~εσᾰσθον", "~υμ~εσᾰσθε";
    aorist_imp_mid_3: "~υμ~εσᾰσθω", "~υμ~εσᾰσθων", _;
    infinitive_pres: _, "~υμ~εσθαι", _;
    infinitive_fut: _, "~υμ~εῑσθαι", _;
    infinitive_aor: _, "~υμ~εσᾰσθαι", _;
    participle_pres: _, "~υμ~ομενος", _;
    participle_fut: _, "~υμ~ουμενος", _;
    participle_aor: _, "~υμ~εσᾰμενος", _;
    ),
    // Κᾰθημαι: 1 words (κᾰθημαι). Like Ἐγκειμαι, except in 9 cells: ind_act_1 sg, pluperfect_mid_1 sg, pluperfect_mid_1 pl, pluperfect_mid_2 sg, pluperfect_mid_2 du, pluperfect_mid_2 pl, ….
    conjugation!("κᾰθημαι", 1, "~ᾰ~μαι", like Ἐγκειμαι as MID_AS_PASS;
    ind_act_1: "~ᾰ~μαι", _, _;
    pluperfect_mid_1: "ἐ~α~μην|~α~μην", _, "ἐ~α~μεθᾰ|~α~μεθᾰ";
    pluperfect_mid_2: "ἐ~α~σο|~α~σο", "ἐ~α~σθον|~α~σθον", "ἐ~α~σθε|~α~σθε";
    pluperfect_mid_3: "ἐ~α~το|~α~στο|~α~το", "ἐ~α~σθην|~α~σθην", "ἐ~α~ντο|~α~ντο";
    ),
    // Ἐπῐκᾰλεω: 1 words (ἐπῐκᾰλεω). Like Μετᾰκᾰλεω, except in 220 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐπῐκᾰλεω", 1, "~ῐ~ᾰ~εω", like Μετᾰκᾰλεω;
    stem aorist_ind: "~ε~α~εσ- like εὐλογεω";
    stem aorist_ind_pass: "~ε~~ηθη- like ἐλπω";
    stem pluperfect_act: "~ε%ε~~ηκ- like εὐλογεω";
    ind_act_1: "~ι~α~ω", _, "~ι~α~οῡμεν";
    ind_act_2: "~ι~α~εῑς", "~ι~α~εῑτον", "~ι~α~εῑτε";
    ind_act_3: "~ι~α~εῑ", _, "~ι~α~οῡσῐ|~ι~α~οῡσῐν";
    ind_mid_1: "~ι~α~οῡμαι", _, "~ι~α~ουμεθᾰ";
    ind_mid_2: "~ι~α~εῑ|~ι~α~ῃ", "~ι~α~εῑσθον", "~ι~α~εῑσθε";
    ind_mid_3: "~ι~α~εῑται", _, "~ι~α~οῡνται";
    subj_act_1: "~ι~α~ω", _, "~ι~α~ωμεν";
    subj_act_2: "~ι~α~ῃς", "~ι~α~ητον", "~ι~α~ητε";
    subj_act_3: "~ι~α~ῃ", _, "~ι~α~ωσῐ|~ι~α~ωσῐν";
    subj_mid_1: "~ι~α~ωμαι", _, "~ι~α~ωμεθᾰ";
    subj_mid_2: _, "~ι~α~ησθον", "~ι~α~ησθε";
    subj_mid_3: "~ι~α~ηται", _, "~ι~α~ωνται";
    opt_act_1: "~ι~α~οιην|~ι~α~οῑμῐ", _, "~ι~α~οῑμεν|~ι~α~οιημεν";
    opt_act_2: "~ι~α~οιης|~ι~α~οῑς", "~ι~α~οῑτον|~ι~α~οιητον", "~ι~α~οῑτε|~ι~α~οιητε";
    opt_act_3: "~ι~α~οιη|~ι~α~οῑ", "~ι~α~οιτην|~ι~α~οιητην", "~ι~α~οῑεν|~ι~α~οιησᾰν";
    opt_mid_1: "~ι~α~οιμην", _, "~ι~α~οιμεθᾰ";
    opt_mid_2: "~ι~α~οῑο", "~ι~α~οῑσθον", "~ι~α~οῑσθε";
    opt_mid_3: "~ι~α~οῑτο", "~ι~α~οισθην", "~ι~α~οῑντο";
    imp_act_2: "~ι~α~ει", _, _;
    imp_act_3: "~ι~α~ειτω", "~ι~α~ειτων", "~ι~α~ουντων";
    imp_mid_2: "~ι~α~οῡ", _, _;
    imp_mid_3: "~ι~α~εισθω", "~ι~α~εισθων", _;
    future_ind_act_1: "~ι~α~ω", _, "~ι~α~οῡμεν";
    future_ind_act_2: "~ι~α~εῑς", "~ι~α~εῑτον", "~ι~α~εῑτε";
    future_ind_act_3: "~ι~α~εῑ", _, "~ι~α~οῡσῐ|~ι~α~οῡσῐν";
    future_ind_mid_1: "~ι~α~οῡμαι", _, "~ι~α~ουμεθᾰ";
    future_ind_mid_2: "~ι~α~εῑ|~ι~α~ῃ", "~ι~α~εῑσθον", "~ι~α~εῑσθε";
    future_ind_mid_3: "~ι~α~εῑται", _, "~ι~α~οῡνται";
    future_ind_pass_1: "~ι%ε~~ησομαι", _, "~ι%ε~~ησομεθᾰ";
    future_ind_pass_2: "~ι%ε~~ησῃ|~ι%ε~~ησει", "~ι%ε~~ησεσθον", "~ι%ε~~ησεσθε";
    future_ind_pass_3: "~ι%ε~~ησεται", _, "~ι%ε~~ησονται";
    future_opt_act_1: "~ι~α~οιην|~ι~α~οῑμῐ", _, "~ι~α~οῑμεν|~ι~α~οιημεν";
    future_opt_act_2: "~ι~α~οιης|~ι~α~οῑς", "~ι~α~οῑτον|~ι~α~οιητον", "~ι~α~οῑτε|~ι~α~οιητε";
    future_opt_act_3: "~ι~α~οιη|~ι~α~οῑ", "~ι~α~οιτην|~ι~α~οιητην", "~ι~α~οῑεν|~ι~α~οιησᾰν";
    future_opt_mid_1: "~ι~α~οιμην", _, "~ι~α~οιμεθᾰ";
    future_opt_mid_2: "~ι~α~οῑο", "~ι~α~οῑσθον", "~ι~α~οῑσθε";
    future_opt_mid_3: "~ι~α~οῑτο", "~ι~α~οισθην", "~ι~α~οῑντο";
    future_opt_pass_1: "~ι%ε~~ησοιμην", _, "~ι%ε~~ησοιμεθᾰ";
    future_opt_pass_2: "~ι%ε~~ησοιο", "~ι%ε~~ησοισθον", "~ι%ε~~ησοισθε";
    future_opt_pass_3: "~ι%ε~~ησοιτο", "~ι%ε~~ησοισθην", "~ι%ε~~ησοιντο";
    aorist_subj_act_1: "~ι~α~εσω", _, "~ι~α~εσωμεν";
    aorist_subj_act_2: "~ι~α~εσῃς", "~ι~α~εσητον", "~ι~α~εσητε";
    aorist_subj_act_3: "~ι~α~εσῃ", _, "~ι~α~εσωσῐ|~ι~α~εσωσῐν";
    aorist_subj_mid_1: "~ι~α~εσωμαι", _, "~ι~α~εσωμεθᾰ";
    aorist_subj_mid_2: _, "~ι~α~εσησθον", "~ι~α~εσησθε";
    aorist_subj_mid_3: "~ι~α~εσηται", _, "~ι~α~εσωνται";
    aorist_subj_pass_1: "~ι~~ηθω", _, "~ι~~ηθωμεν";
    aorist_subj_pass_2: "~ι~~ηθῃς", "~ι~~ηθητον", "~ι~~ηθητε";
    aorist_subj_pass_3: "~ι~~ηθῃ", _, "~ι~~ηθωσῐ|~ι~~ηθωσῐν";
    aorist_opt_act_1: "~ι~α~εσαιμῐ", _, "~ι~α~εσαιμεν";
    aorist_opt_act_2: "~ι~α~εσειᾰς|~ι~α~εσαις", "~ι~α~εσαιτον", "~ι~α~εσαιτε";
    aorist_opt_act_3: "~ι~α~εσειε|~ι~α~εσειεν|~ι~α~εσαι", "~ι~α~εσαιτην", "~ι~α~εσειᾰν|~ι~α~εσαιεν";
    aorist_opt_mid_1: "~ι~α~εσαιμην", _, "~ι~α~εσαιμεθᾰ";
    aorist_opt_mid_2: "~ι~α~εσαιο", "~ι~α~εσαισθον", "~ι~α~εσαισθε";
    aorist_opt_mid_3: "~ι~α~εσαιτο", "~ι~α~εσαισθην", "~ι~α~εσαιντο";
    aorist_opt_pass_1: "~ι~~ηθειην", _, "~ι~~ηθεῑμεν|~ι~~ηθειημεν";
    aorist_opt_pass_2: "~ι~~ηθειης", "~ι~~ηθεῑτον|~ι~~ηθειητον", "~ι~~ηθεῑτε|~ι~~ηθειητε";
    aorist_opt_pass_3: "~ι~~ηθειη", "~ι~~ηθειτην|~ι~~ηθειητην", "~ι~~ηθεῑεν|~ι~~ηθειησᾰν";
    aorist_imp_act_2: "~ι~α~εσον", "~ι~α~εσᾰτον", "~ι~α~εσᾰτε";
    aorist_imp_act_3: "~ι~α~εσᾰτω", "~ι~α~εσᾰτων", "~ι~α~εσᾰντων";
    aorist_imp_mid_2: "~ι~α~εσαι", "~ι~α~εσᾰσθον", "~ι~α~εσᾰσθε";
    aorist_imp_mid_3: "~ι~α~εσᾰσθω", "~ι~α~εσᾰσθων", _;
    aorist_imp_pass_2: "~ι~~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ι~~ηθητω", "~ι~~ηθητων", "~ι~~ηθεντων";
    perfect_ind_act_1: "~ι%ε~~ηκᾰ", _, "~ι%ε~~ηκᾰμεν";
    perfect_ind_act_2: "~ι%ε~~ηκᾰς", "~ι%ε~~ηκᾰτον", "~ι%ε~~ηκᾰτε";
    perfect_ind_act_3: "~ι%ε~~ηκε|~ι%ε~~ηκεν", _, "~ι%ε~~ηκᾱσῐ|~ι%ε~~ηκᾱσῐν";
    perfect_ind_mid_1: "~ι%ε~~ημαι", _, "~ι%ε~~ημεθᾰ";
    perfect_ind_mid_2: "~ι%ε~~ησαι", "~ι%ε~~ησθον", "~ι%ε~~ησθε";
    perfect_ind_mid_3: "~ι%ε~~ηται", _, "~ι%ε~~ηνται";
    perfect_subj_act_1: "~ι%ε~~ηκω", _, "~ι%ε~~ηκωμεν";
    perfect_subj_act_2: "~ι%ε~~ηκῃς", "~ι%ε~~ηκητον", "~ι%ε~~ηκητε";
    perfect_subj_act_3: "~ι%ε~~ηκῃ", _, "~ι%ε~~ηκωσῐ|~ι%ε~~ηκωσῐν";
    perfect_opt_act_1: "~ι%ε~~ηκοιμῐ|~ι%ε~~ηκοιην", _, "~ι%ε~~ηκοιμεν";
    perfect_opt_act_2: "~ι%ε~~ηκοις|~ι%ε~~ηκοιης", "~ι%ε~~ηκοιτον", "~ι%ε~~ηκοιτε";
    perfect_opt_act_3: "~ι%ε~~ηκοι|~ι%ε~~ηκοιη", "~ι%ε~~ηκοιτην", "~ι%ε~~ηκοιεν";
    perfect_imp_act_2: "~ι%ε~~ηκε", "~ι%ε~~ηκετον", "~ι%ε~~ηκετε";
    perfect_imp_act_3: "~ι%ε~~ηκετω", "~ι%ε~~ηκετων", "~ι%ε~~ηκοντων";
    perfect_imp_mid_2: "~ι%ε~~ησο", _, _;
    perfect_imp_mid_3: "~ι%ε~~ησθω", "~ι%ε~~ησθων", _;
    pluperfect_mid_1: "~ε%ε~~ημην", _, "~ε%ε~~ημεθᾰ";
    pluperfect_mid_2: "~ε%ε~~ησο", "~ε%ε~~ησθον", "~ε%ε~~ησθε";
    pluperfect_mid_3: "~ε%ε~~ητο", "~ε%ε~~ησθην", "~ε%ε~~ηντο";
    infinitive_pres: "~ι~α~εῑν", "~ι~α~εῑσθαι", _;
    infinitive_fut: "~ι~α~εῑν", "~ι~α~εῑσθαι", "~ι%ε~~ησεσθαι";
    infinitive_aor: "~ι~α~εσαι", "~ι~α~εσᾰσθαι", "~ι~~ηθηναι";
    infinitive_perf: "~ι%ε~~ηκεναι", "~ι%ε~~ησθαι", _;
    participle_pres: "~ι~α~ων", "~ι~α~ουμενος", _;
    participle_fut: "~ι~α~ων", "~ι~α~ουμενος", "~ι%ε~~ησομενος";
    participle_aor: "~ι~α~εσᾱς", "~ι~α~εσᾰμενος", "~ι~~ηθεις";
    participle_perf: "~ι%ε~~ηκως", "~ι%ε~~ημενος", _;
    ),
    // Χρη: 1 words (χρη). Like Μῑγνῡμῐ, except in 10 cells: ind_act_1 sg, ind_act_3 sg, subj_act_3 sg, opt_act_3 sg, future_ind_act_3 sg, future_ind_mid_3 sg, ….
    conjugation!("χρη", 1, "~η", like Μῑγνῡμῐ as SG_AS_PL;
    ind_act_1: "~η", _, _;
    ind_act_3: "~η", _, _;
    subj_act_3: "~ῃ", _, _;
    opt_act_3: "~ειη", _, _;
    future_ind_act_3: "~ησει", _, _;
    future_ind_mid_3: "~ησται", _, _;
    future_opt_act_3: "~ησοι", _, _;
    future_opt_mid_3: "~ησοιτο", _, _;
    infinitive_pres: "~ηναι", _, _;
    infinitive_fut: "~ησειν", _, _;
    ),
    // Ἠλθον: 1 words (ἠλθον). Like Ἐπορον, except in 32 cells: ind_act_1 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_2 pl, ….
    conjugation!("ἠλθον", 1, "ἠ~ον", like Ἐπορον;
    ind_act_1: "ἠ~ον", _, _;
    aorist_ind_act_1: "ἠ~ον", _, "ἠ~ομεν";
    aorist_ind_act_2: "ἠ~ες", "ἠ~ετον", "ἠ~ετε";
    aorist_ind_act_3: "ἠ~ε|ἠ~εν", "ἠ~ετην", "ἠ~ον";
    aorist_subj_act_1: "ἐ~ω", _, "ἐ~ωμεν";
    aorist_subj_act_2: "ἐ~ῃς", "ἐ~ητον", "ἐ~ητε";
    aorist_subj_act_3: "ἐ~ῃ", _, "ἐ~ωσῐ|ἐ~ωσῐν";
    aorist_opt_act_1: "ἐ~οιμῐ", _, "ἐ~οιμεν";
    aorist_opt_act_2: "ἐ~οις", "ἐ~οιτον", "ἐ~οιτε";
    aorist_opt_act_3: "ἐ~οι", "ἐ~οιτην", "ἐ~οιεν";
    aorist_imp_act_2: "ἐ~ε", "ἐ~ετον", "ἐ~ετε";
    aorist_imp_act_3: "ἐ~ετω", "ἐ~ετων", "ἐ~οντων";
    infinitive_aor: "ἐ~εῑν", _, _;
    participle_aor: "ἐ~ων", _, _;
    ),
    // Ἀρνυμαι: 1 words (ἀρνυμαι). Like Ἐρχομαι, except in 29 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἀρνυμαι", 1, "~νυμαι", like Ἐρχομαι as IMPERFECT_AS_PLUPERFECT;
    ind_act_1: "~νυμαι", _, _;
    ind_mid_1: "~νῠμαι", _, "~νῠμεθᾰ";
    ind_mid_2: "~νῠσαι", "~νῠσθον", "~νῠσθε";
    ind_mid_3: "~νῠται", _, "~νῠνται";
    subj_mid_1: "~νῠωμαι", _, "~νῠωμεθᾰ";
    subj_mid_2: "~νῠῃ", "~νῠησθον", "~νῠησθε";
    subj_mid_3: "~νῠηται", _, "~νῠωνται";
    subj_pass_2: "~νῠῃ", _, _;
    opt_mid_1: "~νῠοιμην", _, "~νῠοιμεθᾰ";
    opt_mid_2: "~νῠοιο", "~νῠοισθον", "~νῠοισθε";
    opt_mid_3: "~νῠοιτο", "~νῠοισθην", "~νῠοιντο";
    imp_mid_2: "~νῠσο", _, _;
    imp_mid_3: "~νῠσθω", "~νῠσθων", _;
    infinitive_pres: _, "~νῠσθαι", _;
    participle_pres: _, "~νῠμενος", _;
    ),
    // Ἐπεφνον: 1 words (ἐπεφνον). Like Ἀραομαι, except in 47 cells: ind_act_1 sg, ind_mid_1 sg, subj_pass_2 sg, future_ind_pass_2 sg, aorist_ind_mid_1 sg, aorist_ind_mid_1 pl, ….
    conjugation!("ἐπεφνον", 1, "ἐ~νον", like Ἀραομαι as MID_AS_ACT;
    stem aorist_ind: "~ν- like ἐπεφνον";
    ind_act_1: "ἐ~νον", _, _;
    ind_mid_1: derived, _, _;
    subj_pass_2: derived, _, _;
    future_ind_pass_2: "~ησῃ|~ησει", _, _;
    aorist_ind_mid_1: derived, _, derived;
    aorist_ind_mid_2: derived, derived, derived;
    aorist_ind_mid_3: derived, derived, derived;
    aorist_subj_act_1: "~νω", _, "~νωμεν";
    aorist_subj_act_2: "~νῃς", "~νητον", "~νητε";
    aorist_subj_act_3: "~νῃ", _, "~νωσῐ|~νωσῐν";
    aorist_opt_act_1: "~νοιμῐ", _, "~νοιμεν";
    aorist_opt_act_2: "~νοις", "~νοιτον", "~νοιτε";
    aorist_opt_act_3: "~νοι", "~νοιτην", "~νοιεν";
    aorist_imp_act_2: "~νε", "~νετον", "~νετε";
    aorist_imp_act_3: "~νετω", "~νετων", "~νοντων";
    perfect_ind_mid_1: "~αμαι", _, "~αμεθᾰ";
    perfect_ind_mid_2: "~ασαι", "~ασθον", "~ασθε";
    perfect_ind_mid_3: "~αται", _, "~ανται";
    perfect_imp_mid_2: "~ασο", _, _;
    perfect_imp_mid_3: "~ασθω", "~ασθων", _;
    infinitive_aor: "~νεῑν", _, _;
    infinitive_perf: _, "~ᾱσθαι", _;
    participle_aor: "~νων", _, _;
    participle_perf: _, "~αμενος", _;
    ),
    // Μετεκῑᾰθον: 1 words (μετεκῑᾰθον). Like Ἐγκειμαι, except in 24 cells: ind_act_1 sg, aorist_subj_act_1 sg, aorist_subj_act_1 pl, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_2 pl, ….
    conjugation!("μετεκῑᾰθον", 1, "~ε~ον", like Ἐγκειμαι as MID_AS_PASS;
    stem aorist_ind: "~ε~- like εἰσερχομαι";
    ind_act_1: "~ε~ον", _, _;
    aorist_subj_act_1: "~α~ω", _, "~α~ωμεν";
    aorist_subj_act_2: "~α~ῃς", "~α~ητον", "~α~ητε";
    aorist_subj_act_3: "~α~ῃ", _, "~α~ωσῐ|~α~ωσῐν";
    aorist_opt_act_1: "~α~οιμῐ", _, "~α~οιμεν";
    aorist_opt_act_2: "~α~οις", "~α~οιτον", "~α~οιτε";
    aorist_opt_act_3: "~α~οι", "~α~οιτην", "~α~οιεν";
    aorist_imp_act_2: "~α~ε", "~α~ετον", "~α~ετε";
    aorist_imp_act_3: "~α~ετω", "~α~ετων", "~α~οντων";
    infinitive_aor: "~α~εῑν", _, _;
    participle_aor: "~α~ων", _, _;
    ),
    // Πεπνῡμαι: 1 words (πεπνῡμαι). Like Μῑγνῡμῐ, except in 22 cells: ind_act_1 sg, aorist_subj_pass_1 sg, aorist_subj_pass_1 pl, aorist_subj_pass_2 sg, aorist_subj_pass_2 du, aorist_subj_pass_2 pl, ….
    conjugation!("πεπνῡμαι", 1, "πε~μαι", like Μῑγνῡμῐ as SG_AS_PL;
    stem aorist_ind_pass: "~θη- like ἐχω";
    ind_act_1: "πε~μαι", _, _;
    aorist_subj_pass_1: "~θω", _, "~θωμεν";
    aorist_subj_pass_2: "~θῃς", "~θητον", "~θητε";
    aorist_subj_pass_3: "~θῃ", _, "~θωσῐ|~θωσῐν";
    aorist_opt_pass_1: "~θειην", _, "~θεῑμεν|~θειημεν";
    aorist_opt_pass_2: "~θειης", "~θεῑτον|~θειητον", "~θεῑτε|~θειητε";
    aorist_opt_pass_3: "~θειη", "~θειτην|~θειητην", "~θεῑεν|~θειησᾰν";
    aorist_imp_pass_2: "~θητῐ", _, _;
    aorist_imp_pass_3: "~θητω", "~θητων", "~θεντων";
    infinitive_aor: _, _, "~θηναι";
    participle_aor: _, _, "~θεις";
    ),
    // Περνημῐ: 1 words (περνημῐ). Like Μῑγνῡμῐ, except in 52 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("περνημῐ", 1, "~νημῐ", like Μῑγνῡμῐ as SG_AS_PL;
    ind_act_1: "~νημῐ", _, _;
    future_ind_act_1: "~ᾰσω", _, "~ᾰσομεν";
    future_ind_act_2: "~ᾰσεις", "~ᾰσετον", "~ᾰσετε";
    future_ind_act_3: "~ᾰσει", _, "~ᾰσουσῐ|~ᾰσουσῐν";
    future_ind_mid_1: "~ᾰσομαι", _, "~ᾰσομεθᾰ";
    future_ind_mid_2: "~ᾰσῃ|~ᾰσει", "~ᾰσεσθον", "~ᾰσεσθε";
    future_ind_mid_3: "~ᾰσεται", _, "~ᾰσονται";
    future_ind_pass_1: "~ᾰθησομαι", _, "~ᾰθησομεθᾰ";
    future_ind_pass_2: "~ᾰθησῃ|~ᾰθησει", "~ᾰθησεσθον", "~ᾰθησεσθε";
    future_ind_pass_3: "~ᾰθησεται", _, "~ᾰθησονται";
    future_opt_act_1: "~ᾰσοιμῐ", _, "~ᾰσοιμεν";
    future_opt_act_2: "~ᾰσοις", "~ᾰσοιτον", "~ᾰσοιτε";
    future_opt_act_3: "~ᾰσοι", "~ᾰσοιτην", "~ᾰσοιεν";
    future_opt_mid_1: "~ᾰσοιμην", _, "~ᾰσοιμεθᾰ";
    future_opt_mid_2: "~ᾰσοιο", "~ᾰσοισθον", "~ᾰσοισθε";
    future_opt_mid_3: "~ᾰσοιτο", "~ᾰσοισθην", "~ᾰσοιντο";
    future_opt_pass_1: "~ᾰθησοιμην", _, "~ᾰθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰθησοιο", "~ᾰθησοισθον", "~ᾰθησοισθε";
    future_opt_pass_3: "~ᾰθησοιτο", "~ᾰθησοισθην", "~ᾰθησοιντο";
    infinitive_fut: "~ᾰᾱν|~ᾱμεν|~ᾱμεναι|~ᾰσμεν|~ᾰσμεναι", "~ᾰσεσθαι", "~ᾰθησεσθαι";
    participle_fut: "~ᾰσων", "~ᾰσομενος", "~ᾰθησομενος";
    ),
    // Ἐπῐστᾰμαι: 1 words (ἐπῐστᾰμαι). Like Ἐγκειμαι, except in 72 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἐπῐστᾰμαι", 1, "ἐ~ῐ~ᾰμαι", like Ἐγκειμαι as MID_AS_PASS;
    stem aorist_ind_pass: "~ι~ηθη- like ἀγορευω";
    ind_act_1: "ἐ~ῐ~ᾰμαι", _, _;
    ind_mid_1: "ἐ~ι~ᾰμαι", _, "ἐ~ι~ᾰμεθᾰ";
    ind_mid_2: "ἐ~ι~ᾰσαι|ἐ~ι~ᾳ|ἐ~ι~ῃ", "ἐ~ι~ᾰσθον", "ἐ~ι~ᾰσθε";
    ind_mid_3: "ἐ~ι~ᾰται", _, "ἐ~ι~ᾰνται";
    subj_mid_1: "ἐ~ι~ωμαι|ἐ~ι~ωμαι", _, "ἐ~ι~ωμεθᾰ";
    subj_mid_2: "ἐ~ι~ῃ", "ἐ~ι~ησθον", "ἐ~ι~ησθε";
    subj_mid_3: "ἐ~ι~ηται", _, "ἐ~ι~ωνται";
    opt_mid_1: "ἐ~ι~αιμην", _, "ἐ~ι~αιμεθᾰ";
    opt_mid_2: "ἐ~ι~αιο", "ἐ~ι~αισθον", "ἐ~ι~αισθε";
    opt_mid_3: "ἐ~ι~αιτο", "ἐ~ι~αισθην", "ἐ~ι~αιντο";
    imp_mid_2: "ἐ~ι~ᾰσο|ἐ~ι~ω", _, _;
    imp_mid_3: "ἐ~ι~ᾰσθω", "ἐ~ι~ᾰσθων", _;
    imperfect_mid_1: "ἠ~ι~ᾰμην", _, "ἠ~ι~ᾰμεθᾰ";
    imperfect_mid_2: "ἠ~ῐ~ω|ἠ~ῐ~ᾰσο", "ἠ~ι~ᾰσθον", "ἠ~ι~ᾰσθε";
    imperfect_mid_3: "ἠ~ι~ᾰτο", "ἠ~ι~ᾰσθην", "ἠ~ι~ᾰντο";
    future_ind_mid_1: "ἐ~ι~ησομαι", _, "ἐ~ι~ησομεθᾰ";
    future_ind_mid_2: "ἐ~ι~ησῃ|ἐ~ι~ησει", "ἐ~ι~ησεσθον", "ἐ~ι~ησεσθε";
    future_ind_mid_3: "ἐ~ι~ησεται", _, "ἐ~ι~ησονται";
    future_opt_mid_1: "ἐ~ι~ησοιμην", _, "ἐ~ι~ησοιμεθᾰ";
    future_opt_mid_2: "ἐ~ι~ησοιο", "ἐ~ι~ησοισθον", "ἐ~ι~ησοισθε";
    future_opt_mid_3: "ἐ~ι~ησοιτο", "ἐ~ι~ησοισθην", "ἐ~ι~ησοιντο";
    aorist_subj_pass_1: "ἐ~ι~ηθω", _, "ἐ~ι~ηθωμεν";
    aorist_subj_pass_2: "ἐ~ι~ηθῃς", "ἐ~ι~ηθητον", "ἐ~ι~ηθητε";
    aorist_subj_pass_3: "ἐ~ι~ηθῃ", _, "ἐ~ι~ηθωσῐ|ἐ~ι~ηθωσῐν";
    aorist_opt_pass_1: "ἐ~ι~ηθειην", _, "ἐ~ι~ηθεῑμεν|ἐ~ι~ηθειημεν";
    aorist_opt_pass_2: "ἐ~ι~ηθειης", "ἐ~ι~ηθεῑτον|ἐ~ι~ηθειητον", "ἐ~ι~ηθεῑτε|ἐ~ι~ηθειητε";
    aorist_opt_pass_3: "ἐ~ι~ηθειη", "ἐ~ι~ηθειτην|ἐ~ι~ηθειητην", "ἐ~ι~ηθεῑεν|ἐ~ι~ηθειησᾰν";
    aorist_imp_pass_2: "ἐ~ι~ηθητῐ", _, _;
    aorist_imp_pass_3: "ἐ~ι~ηθητω", "ἐ~ι~ηθητων", "ἐ~ι~ηθεντων";
    infinitive_fut: _, "ἐ~ι~ησεσθαι", _;
    infinitive_aor: _, _, "ἐ~ι~ηθηναι";
    participle_fut: _, "ἐ~ι~ησομενος", _;
    participle_aor: _, _, "ἐ~ι~ηθεις";
    ),
    // Δειδω: 1 words (δειδω). Like Κεῑμαι, except in 79 cells: ind_act_1 sg, imperfect_mid_1 sg, imperfect_mid_1 pl, imperfect_mid_2 sg, imperfect_mid_2 du, imperfect_mid_2 pl, ….
    conjugation!("δειδω", 1, "~ιδω", like Κεῑμαι as PRESENT_AS_FUTURE;
    stem aorist_ind: "~ισ- like χᾰριζω";
    stem pluperfect_act: "~δοικ- like δειδω";
    ind_act_1: "~ιδω", _, _;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, derived;
    imperfect_mid_3: derived, derived, derived;
    future_ind_act_1: "~ισω", _, "~ισομεν";
    future_ind_act_2: "~ισεις", "~ισετον", "~ισετε";
    future_ind_act_3: "~ισει", _, "~ισουσῐ|~ισουσῐν";
    future_opt_act_1: "~ισοιμῐ", _, "~ισοιμεν";
    future_opt_act_2: "~ισοις", "~ισοιτον", "~ισοιτε";
    future_opt_act_3: "~ισοι", "~ισοιτην", "~ισοιεν";
    aorist_subj_act_1: "~ισω", _, "~ισωμεν";
    aorist_subj_act_2: "~ισῃς", "~ισητον", "~ισητε";
    aorist_subj_act_3: "~ισῃ", _, "~ισωσῐ|~ισωσῐν";
    aorist_opt_act_1: "~ισαιμῐ", _, "~ισαιμεν";
    aorist_opt_act_2: "~ισειᾰς|~ισαις", "~ισαιτον", "~ισαιτε";
    aorist_opt_act_3: "~ισειε|~ισειεν|~ισαι", "~ισαιτην", "~ισειᾰν|~ισαιεν";
    aorist_imp_act_2: "~ῑσον", "~ισᾰτον", "~ισᾰτε";
    aorist_imp_act_3: "~ισᾰτω", "~ισᾰτων", "~ισᾰντων";
    perfect_ind_act_1: "~δοικᾰ", _, "~δοικᾰμεν";
    perfect_ind_act_2: "~δοικᾰς", "~δοικᾰτον", "~δοικᾰτε";
    perfect_ind_act_3: "~δοικε|~δοικεν", _, "~δοικᾱσῐ|~δοικᾱσῐν";
    perfect_subj_act_1: "~δοικω", _, "~δοικωμεν";
    perfect_subj_act_2: "~δοικῃς", "~δοικητον", "~δοικητε";
    perfect_subj_act_3: "~δοικῃ", _, "~δοικωσῐ|~δοικωσῐν";
    perfect_opt_act_1: "~δοικοιμῐ|~δοικοιην", _, "~δοικοιμεν";
    perfect_opt_act_2: "~δοικοις|~δοικοιης", "~δοικοιτον", "~δοικοιτε";
    perfect_opt_act_3: "~δοικοι|~δοικοιη", "~δοικοιτην", "~δοικοιεν";
    perfect_imp_act_2: "~δοικε", "~δοικετον", "~δοικετε";
    perfect_imp_act_3: "~δοικετω", "~δοικετων", "~δοικοντων";
    infinitive_fut: "~ισειν", _, _;
    infinitive_aor: "~ῑσαι", _, _;
    infinitive_perf: "~δοικεναι", _, _;
    participle_fut: "~ισων", _, _;
    participle_aor: "~ισᾱς", _, _;
    participle_perf: "~δοικως", _, _;
    ),
    // Ἀπαυραω: 1 words (ἀπαυραω). Like Μετεκῑᾰθον, except in 95 cells: ind_act_1 sg, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, ….
    conjugation!("ἀπαυραω", 1, "~αυ~αω", like Μετεκῑᾰθον;
    stem aorist_ind: "~ου~- like εὐλογεω";
    ind_act_1: "~αυ~αω", _, _;
    imperfect_act_1: "~ηυ~ων", _, "~ηυ~ωμεν";
    imperfect_act_2: "~ηυ~ᾱς", "~ηυ~ᾱτον", "~ηυ~ᾱτε";
    imperfect_act_3: "~ηυ~ᾱ", "~ηυ~ᾱτην", "~ηυ~ων";
    imperfect_mid_1: "~ηυ~ωμην", _, "~ηυ~ωμεθᾰ";
    imperfect_mid_2: "~ηυ~ω", "~ηυ~ᾱσθον", "~ηυ~ᾱσθε";
    imperfect_mid_3: "~ηυ~ᾱτο", "~ηυ~ᾱσθην", "~ηυ~ωντο";
    future_ind_act_1: "~ου~ησω", _, "~ου~ησομεν";
    future_ind_act_2: "~ου~ησεις", "~ου~ησετον", "~ου~ησετε";
    future_ind_act_3: "~ου~ησει", _, "~ου~ησουσῐ|~ου~ησουσῐν";
    future_ind_mid_1: "~ου~ησομαι", _, "~ου~ησομεθᾰ";
    future_ind_mid_2: "~ου~ησῃ|~ου~ησει", "~ου~ησεσθον", "~ου~ησεσθε";
    future_ind_mid_3: "~ου~ησεται", _, "~ου~ησονται";
    future_opt_act_1: "~ου~ησοιμῐ", _, "~ου~ησοιμεν";
    future_opt_act_2: "~ου~ησοις", "~ου~ησοιτον", "~ου~ησοιτε";
    future_opt_act_3: "~ου~ησοι", "~ου~ησοιτην", "~ου~ησοιεν";
    future_opt_mid_1: "~ου~ησοιμην", _, "~ου~ησοιμεθᾰ";
    future_opt_mid_2: "~ου~ησοιο", "~ου~ησοισθον", "~ου~ησοισθε";
    future_opt_mid_3: "~ου~ησοιτο", "~ου~ησοισθην", "~ου~ησοιντο";
    aorist_subj_act_1: "~ου~ω", _, "~ου~ωμεν";
    aorist_subj_act_2: "~ου~ῃς", "~ου~ητον", "~ου~ητε";
    aorist_subj_act_3: "~ου~ῃ", _, "~ου~ωσῐ|~ου~ωσῐν";
    aorist_subj_mid_1: "~ου~ωμαι", _, "~ου~ωμεθᾰ";
    aorist_subj_mid_2: _, "~ου~ησθον", "~ου~ησθε";
    aorist_subj_mid_3: "~ου~ηται", _, "~ου~ωνται";
    aorist_opt_act_1: "~ου~αιμῐ", _, "~ου~αιμεν";
    aorist_opt_act_2: "~ου~ειᾰς|~ου~αις", "~ου~αιτον", "~ου~αιτε";
    aorist_opt_act_3: "~ου~ειε|~ου~ειεν|~ου~αι", "~ου~αιτην", "~ου~ειᾰν|~ου~αιεν";
    aorist_opt_mid_1: "~ου~αιμην", _, "~ου~αιμεθᾰ";
    aorist_opt_mid_2: "~ου~αιο", "~ου~αισθον", "~ου~αισθε";
    aorist_opt_mid_3: "~ου~αιτο", "~ου~αισθην", "~ου~αιντο";
    aorist_imp_act_2: "~ου~ον", "~ου~ᾰτον", "~ου~ᾰτε";
    aorist_imp_act_3: "~ου~ᾰτω", "~ου~ᾰτων", "~ου~ᾰντων";
    aorist_imp_mid_2: "~ου~αι", "~ου~ᾰσθον", "~ου~ᾰσθε";
    aorist_imp_mid_3: "~ου~ᾰσθω", "~ου~ᾰσθων", _;
    infinitive_fut: "~ου~ησειν", "~ου~ησεσθαι", _;
    infinitive_aor: "~οῡ~αι", "~ου~ᾰσθαι", _;
    participle_fut: "~ου~ησων", "~ου~ησομενος", _;
    participle_aor: "~ου~ᾱς", "~ου~ᾰμενος", _;
    ),
    // Ἐπῐπειθομαι: 1 words (ἐπῐπειθομαι). Like Ἐγκειμαι, except in 43 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἐπῐπειθομαι", 1, "~ῐ~θομαι", like Ἐγκειμαι as MID_AS_PASS;
    stem imperfect: "~ε~θ- like ὑποκοριζομαι";
    ind_act_1: "~ῐ~θομαι", _, _;
    ind_mid_1: "~ι~θομαι", _, "~ι~θομεθᾰ";
    ind_mid_2: "~ι~θῃ|~ι~θει", "~ι~θεσθον", "~ι~θεσθε";
    ind_mid_3: "~ι~θεται", _, "~ι~θονται";
    subj_mid_1: "~ι~θωμαι", _, "~ι~θωμεθᾰ";
    subj_mid_2: "~ι~θῃ", "~ι~θησθον", "~ι~θησθε";
    subj_mid_3: "~ι~θηται", _, "~ι~θωνται";
    opt_mid_1: "~ι~θοιμην", _, "~ι~θοιμεθᾰ";
    opt_mid_2: "~ι~θοιο", "~ι~θοισθον", "~ι~θοισθε";
    opt_mid_3: "~ι~θοιτο", "~ι~θοισθην", "~ι~θοιντο";
    imp_mid_2: "~ι~θου", _, _;
    imp_mid_3: "~ι~θεσθω", "~ι~θεσθων", _;
    future_ind_mid_1: "~ι~σομαι", _, "~ι~σομεθᾰ";
    future_ind_mid_2: "~ι~σῃ|~ι~σει", "~ι~σεσθον", "~ι~σεσθε";
    future_ind_mid_3: "~ι~σεται", _, "~ι~σονται";
    future_opt_mid_1: "~ι~σοιμην", _, "~ι~σοιμεθᾰ";
    future_opt_mid_2: "~ι~σοιο", "~ι~σοισθον", "~ι~σοισθε";
    future_opt_mid_3: "~ι~σοιτο", "~ι~σοισθην", "~ι~σοιντο";
    infinitive_fut: _, "~ι~σεσθαι", _;
    participle_fut: _, "~ι~σομενος", _;
    ),
    // Ἐπισευω: 1 words (ἐπισευω). Like Κᾰθημαι, except in 21 cells: ind_act_1 sg, perfect_ind_mid_1 sg, perfect_ind_mid_1 pl, perfect_ind_mid_2 sg, perfect_ind_mid_2 du, perfect_ind_mid_2 pl, ….
    conjugation!("ἐπισευω", 1, "~ι~ευω", like Κᾰθημαι;
    ind_act_1: "~ι~ευω", _, _;
    perfect_ind_mid_1: "~ι~ῠμαι", _, "~ι~ῠμεθᾰ";
    perfect_ind_mid_2: "~ι~ῠσαι", "~ι~ῠσθον", "~ι~ῠσθε";
    perfect_ind_mid_3: "~ι~ῠται", _, "~ι~ῠνται";
    perfect_imp_mid_2: "~ι~ῠσο", _, _;
    perfect_imp_mid_3: "~ι~ῠσθω", "~ι~ῠσθων", _;
    pluperfect_mid_1: "~ε~ῠμην", _, "~ε~ῠμεθᾰ";
    pluperfect_mid_2: "~ε~ῠσο", "~ε~ῠσθον", "~ε~ῠσθε";
    pluperfect_mid_3: "~ε~ῠτο", "~ε~ῠσθην", "~ε~ῠντο";
    infinitive_perf: _, "~ι~ῠσθαι", _;
    participle_perf: _, "~ι~υμενος", _;
    ),
    // Δειμᾰτοω: 1 words (δειμᾰτοω). Like Κᾰταφρονεω, except in 189 cells: ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_mid_2 sg, ind_mid_2 du, ….
    conjugation!("δειμᾰτοω", 1, "~ᾰ~οω", like Κᾰταφρονεω;
    stem aorist_ind: "~α~ωσ- like ποιεω";
    stem aorist_ind_pass: "~α~ωθη- like ἐχω";
    stem pluperfect_act: "~α~ωκ- like χρημᾰτῐζω";
    stem pluperfect: "~α~ω- like πειρᾰω";
    ind_act_2: "~α~οῑς", "~α~οῡτον", "~α~οῡτε";
    ind_act_3: "~α~οῑ", _, _;
    ind_mid_2: "~α~οῑ", "~α~οῡσθον", "~α~οῡσθε";
    ind_mid_3: "~α~οῡται", _, _;
    subj_act_2: "~α~οῑς", "~α~ωτον", "~α~ωτε";
    subj_act_3: "~α~οῑ", _, _;
    subj_mid_2: _, "~α~ωσθον", "~α~ωσθε";
    subj_mid_3: "~α~ωται", _, _;
    imp_act_2: "~α~ου", _, _;
    imp_act_3: "~α~ουτω", "~α~ουτων", _;
    imp_mid_3: "~α~ουσθω", "~α~ουσθων", _;
    imperfect_act_2: "ἐ~α~ους", "ἐ~α~οῡτον", "ἐ~α~οῡτε";
    imperfect_act_3: "ἐ~α~ου", "ἐ~α~ουτην", _;
    imperfect_mid_2: _, "ἐ~α~οῡσθον", "ἐ~α~οῡσθε";
    imperfect_mid_3: "ἐ~α~οῡτο", "ἐ~α~ουσθην", _;
    future_ind_act_1: "~α~ωσω", _, "~α~ωσομεν";
    future_ind_act_2: "~α~ωσεις", "~α~ωσετον", "~α~ωσετε";
    future_ind_act_3: "~α~ωσει", _, "~α~ωσουσῐ|~α~ωσουσῐν";
    future_ind_mid_1: "~α~ωσομαι", _, "~α~ωσομεθᾰ";
    future_ind_mid_2: "~α~ωσῃ|~α~ωσει", "~α~ωσεσθον", "~α~ωσεσθε";
    future_ind_mid_3: "~α~ωσεται", _, "~α~ωσονται";
    future_ind_pass_1: "~α~ωθησομαι", _, "~α~ωθησομεθᾰ";
    future_ind_pass_2: "~α~ωθησῃ|~α~ωθησει", "~α~ωθησεσθον", "~α~ωθησεσθε";
    future_ind_pass_3: "~α~ωθησεται", _, "~α~ωθησονται";
    future_opt_act_1: "~α~ωσοιμῐ", _, "~α~ωσοιμεν";
    future_opt_act_2: "~α~ωσοις", "~α~ωσοιτον", "~α~ωσοιτε";
    future_opt_act_3: "~α~ωσοι", "~α~ωσοιτην", "~α~ωσοιεν";
    future_opt_mid_1: "~α~ωσοιμην", _, "~α~ωσοιμεθᾰ";
    future_opt_mid_2: "~α~ωσοιο", "~α~ωσοισθον", "~α~ωσοισθε";
    future_opt_mid_3: "~α~ωσοιτο", "~α~ωσοισθην", "~α~ωσοιντο";
    future_opt_pass_1: "~α~ωθησοιμην", _, "~α~ωθησοιμεθᾰ";
    future_opt_pass_2: "~α~ωθησοιο", "~α~ωθησοισθον", "~α~ωθησοισθε";
    future_opt_pass_3: "~α~ωθησοιτο", "~α~ωθησοισθην", "~α~ωθησοιντο";
    aorist_subj_act_1: "~α~ωσω", _, "~α~ωσωμεν";
    aorist_subj_act_2: "~α~ωσῃς", "~α~ωσητον", "~α~ωσητε";
    aorist_subj_act_3: "~α~ωσῃ", _, "~α~ωσωσῐ|~α~ωσωσῐν";
    aorist_subj_mid_1: "~α~ωσωμαι", _, "~α~ωσωμεθᾰ";
    aorist_subj_mid_2: _, "~α~ωσησθον", "~α~ωσησθε";
    aorist_subj_mid_3: "~α~ωσηται", _, "~α~ωσωνται";
    aorist_subj_pass_1: "~α~ωθω", _, "~α~ωθωμεν";
    aorist_subj_pass_2: "~α~ωθῃς", "~α~ωθητον", "~α~ωθητε";
    aorist_subj_pass_3: "~α~ωθῃ", _, "~α~ωθωσῐ|~α~ωθωσῐν";
    aorist_opt_act_1: "~α~ωσαιμῐ", _, "~α~ωσαιμεν";
    aorist_opt_act_2: "~α~ωσειᾰς|~α~ωσαις", "~α~ωσαιτον", "~α~ωσαιτε";
    aorist_opt_act_3: "~α~ωσειε|~α~ωσειεν|~α~ωσαι", "~α~ωσαιτην", "~α~ωσειᾰν|~α~ωσαιεν";
    aorist_opt_mid_1: "~α~ωσαιμην", _, "~α~ωσαιμεθᾰ";
    aorist_opt_mid_2: "~α~ωσαιο", "~α~ωσαισθον", "~α~ωσαισθε";
    aorist_opt_mid_3: "~α~ωσαιτο", "~α~ωσαισθην", "~α~ωσαιντο";
    aorist_opt_pass_1: "~α~ωθειην", _, "~α~ωθεῑμεν|~α~ωθειημεν";
    aorist_opt_pass_2: "~α~ωθειης", "~α~ωθεῑτον|~α~ωθειητον", "~α~ωθεῑτε|~α~ωθειητε";
    aorist_opt_pass_3: "~α~ωθειη", "~α~ωθειτην|~α~ωθειητην", "~α~ωθεῑεν|~α~ωθειησᾰν";
    aorist_imp_act_2: "~α~ωσον", "~α~ωσᾰτον", "~α~ωσᾰτε";
    aorist_imp_act_3: "~α~ωσᾰτω", "~α~ωσᾰτων", "~α~ωσᾰντων";
    aorist_imp_mid_2: "~α~ωσαι", "~α~ωσᾰσθον", "~α~ωσᾰσθε";
    aorist_imp_mid_3: "~α~ωσᾰσθω", "~α~ωσᾰσθων", _;
    aorist_imp_pass_2: "~α~ωθητῐ", _, _;
    aorist_imp_pass_3: "~α~ωθητω", "~α~ωθητων", "~α~ωθεντων";
    perfect_ind_act_1: "%ε~α~ωκᾰ", _, "%ε~α~ωκᾰμεν";
    perfect_ind_act_2: "%ε~α~ωκᾰς", "%ε~α~ωκᾰτον", "%ε~α~ωκᾰτε";
    perfect_ind_act_3: "%ε~α~ωκε|%ε~α~ωκεν", _, "%ε~α~ωκᾱσῐ|%ε~α~ωκᾱσῐν";
    perfect_ind_mid_1: "%ε~α~ωμαι", _, "%ε~α~ωμεθᾰ";
    perfect_ind_mid_2: "%ε~α~ωσαι", "%ε~α~ωσθον", "%ε~α~ωσθε";
    perfect_ind_mid_3: "%ε~α~ωται", _, "%ε~α~ωνται";
    perfect_subj_act_1: "%ε~α~ωκω", _, "%ε~α~ωκωμεν";
    perfect_subj_act_2: "%ε~α~ωκῃς", "%ε~α~ωκητον", "%ε~α~ωκητε";
    perfect_subj_act_3: "%ε~α~ωκῃ", _, "%ε~α~ωκωσῐ|%ε~α~ωκωσῐν";
    perfect_opt_act_1: "%ε~α~ωκοιμῐ|%ε~α~ωκοιην", _, "%ε~α~ωκοιμεν";
    perfect_opt_act_2: "%ε~α~ωκοις|%ε~α~ωκοιης", "%ε~α~ωκοιτον", "%ε~α~ωκοιτε";
    perfect_opt_act_3: "%ε~α~ωκοι|%ε~α~ωκοιη", "%ε~α~ωκοιτην", "%ε~α~ωκοιεν";
    perfect_imp_act_2: "%ε~α~ωκε", "%ε~α~ωκετον", "%ε~α~ωκετε";
    perfect_imp_act_3: "%ε~α~ωκετω", "%ε~α~ωκετων", "%ε~α~ωκοντων";
    perfect_imp_mid_2: "%ε~α~ωσο", _, _;
    perfect_imp_mid_3: "%ε~α~ωσθω", "%ε~α~ωσθων", _;
    infinitive_pres: "~α~οῡν", "~α~οῡσθαι", _;
    infinitive_fut: "~α~ωσειν", "~α~ωσεσθαι", "~α~ωθησεσθαι";
    infinitive_aor: "~α~ωσαι", "~α~ωσᾰσθαι", "~α~ωθηναι";
    infinitive_perf: "%ε~α~ωκεναι", "%ε~α~ωσθαι", _;
    participle_fut: "~α~ωσων", "~α~ωσομενος", "~α~ωθησομενος";
    participle_aor: "~α~ωσᾱς", "~α~ωσᾰμενος", "~α~ωθεις";
    participle_perf: "%ε~α~ωκως", "%ε~α~ωμενος", _;
    ),
    // Κορυσσω: 1 words (κορυσσω). Like Ἐπορον, except in 75 cells: ind_act_1 sg, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, ….
    conjugation!("κορυσσω", 1, "~υσσω", like Ἐπορον;
    stem aorist_ind: "~ῠσσ- like ποιεω";
    ind_act_1: "~υσσω", _, _;
    imperfect_act_1: "~ῠσσον", _, "~ῠσσομεν";
    imperfect_act_2: "~ῠσσες", "~ῠσσετον", "~ῠσσετε";
    imperfect_act_3: "~ῠσσε|~ῠσσεν", "~ῠσσετην", "~ῠσσον";
    imperfect_mid_1: "~ῠσσομην", _, "~ῠσσομεθᾰ";
    imperfect_mid_2: "~ῠσσου", "~ῠσσεσθον", "~ῠσσεσθε";
    imperfect_mid_3: "~ῠσσετο", "~ῠσσεσθην", "~ῠσσοντο";
    aorist_subj_act_1: "~ῠσσω", _, "~ῠσσωμεν";
    aorist_subj_act_2: "~ῠσσῃς", "~ῠσσητον", "~ῠσσητε";
    aorist_subj_act_3: "~ῠσσῃ", _, "~ῠσσωσῐ|~ῠσσωσῐν";
    aorist_subj_mid_1: "~ῠσσωμαι", _, "~ῠσσωμεθᾰ";
    aorist_subj_mid_2: _, "~ῠσσησθον", "~ῠσσησθε";
    aorist_subj_mid_3: "~ῠσσηται", _, "~ῠσσωνται";
    aorist_opt_act_1: "~ῠσσαιμῐ", _, "~ῠσσαιμεν";
    aorist_opt_act_2: "~ῠσσειᾰς|~ῠσσαις", "~ῠσσαιτον", "~ῠσσαιτε";
    aorist_opt_act_3: "~ῠσσειε|~ῠσσειεν|~ῠσσαι", "~ῠσσαιτην", "~ῠσσειᾰν|~ῠσσαιεν";
    aorist_opt_mid_1: "~ῠσσαιμην", _, "~ῠσσαιμεθᾰ";
    aorist_opt_mid_2: "~ῠσσαιο", "~ῠσσαισθον", "~ῠσσαισθε";
    aorist_opt_mid_3: "~ῠσσαιτο", "~ῠσσαισθην", "~ῠσσαιντο";
    aorist_imp_act_2: "~ῠσσον", "~ῠσσᾰτον", "~ῠσσᾰτε";
    aorist_imp_act_3: "~ῠσσᾰτω", "~ῠσσᾰτων", "~ῠσσᾰντων";
    aorist_imp_mid_2: "~ῠσσαι", "~ῠσσᾰσθον", "~ῠσσᾰσθε";
    aorist_imp_mid_3: "~ῠσσᾰσθω", "~ῠσσᾰσθων", _;
    perfect_ind_pass_1: "%ε~υθμαι", _, "%ε~υθμεθᾰ";
    perfect_ind_pass_2: "%ε~ῠσαι", "%ε~ῠσθον", "%ε~ῠσθε";
    perfect_ind_pass_3: "%ε~ῠσται", "%ε~ῠσθον", "%ε~ῠθᾰται";
    perfect_imp_pass_2: "%ε~ῠσο", "%ε~ῠσθον", "%ε~ῠσθε";
    perfect_imp_pass_3: "%ε~ῠσθω", "%ε~ῠσθων", "%ε~ῠσθων";
    infinitive_aor: "~ῠσσαι", "~ῠσσᾰσθαι", _;
    participle_aor: "~ῠσσᾱς", "~ῠσσᾰμενος", _;
    ),
    // Κεκᾰδον: 1 words (κεκᾰδον). Like Μετεκῑᾰθον, except in 52 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("κεκᾰδον", 1, "~ᾰ~ον", like Μετεκῑᾰθον;
    stem aorist_ind: "~ᾰ~- like πᾰρεχω";
    ind_act_1: "~ᾰ~ον", _, _;
    future_ind_act_1: "~α~ησω", _, "~α~ησομεν";
    future_ind_act_2: "~α~ησεις", "~α~ησετον", "~α~ησετε";
    future_ind_act_3: "~α~ησει", _, "~α~ησουσῐ|~α~ησουσῐν";
    aorist_subj_act_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    aorist_subj_act_2: "~ᾰ~ῃς", "~ᾰ~ητον", "~ᾰ~ητε";
    aorist_subj_act_3: "~ᾰ~ῃ", _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰ~ησθον", "~ᾰ~ησθε";
    aorist_subj_mid_3: "~ᾰ~ηται", _, "~ᾰ~ωνται";
    aorist_opt_act_1: "~ᾰ~οιμῐ", _, "~ᾰ~οιμεν";
    aorist_opt_act_2: "~ᾰ~οις", "~ᾰ~οιτον", "~ᾰ~οιτε";
    aorist_opt_act_3: "~ᾰ~οι", "~ᾰ~οιτην", "~ᾰ~οιεν";
    aorist_opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~οιο", "~ᾰ~οισθον", "~ᾰ~οισθε";
    aorist_opt_mid_3: "~ᾰ~οιτο", "~ᾰ~οισθην", "~ᾰ~οιντο";
    aorist_imp_act_2: "~ᾰ~ε", "~ᾰ~ετον", "~ᾰ~ετε";
    aorist_imp_act_3: "~ᾰ~ετω", "~ᾰ~ετων", "~ᾰ~οντων";
    aorist_imp_mid_2: "~ᾰ~οῡ", "~ᾰ~εσθον", "~ᾰ~εσθε";
    aorist_imp_mid_3: "~ᾰ~εσθω", "~ᾰ~εσθων", _;
    infinitive_aor: "~ᾰ~εῑν", "~ᾰ~εσθαι", _;
    participle_aor: "~ᾰ~ων", "~ᾰ~ομενος", _;
    ),
    // Κᾰτᾰκλαω: 1 words (κᾰτᾰκλαω). Like Ξυνναυμᾰχεω, except in 147 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("κᾰτᾰκλαω", 1, "~ᾰ~ᾰ~αω", like Ξυνναυμᾰχεω as ACT_AS_PASS;
    stem aorist_ind: "~ᾰ~ε~ᾰσ- like εὐλογεω";
    stem aorist_ind_pass: "~α~ε~ασθη- like ἐλπω";
    ind_act_1: "~ᾰ~α~ω", _, "~ᾰ~α~ωμεν";
    ind_act_2: "~ᾰ~α~ᾳς", "~ᾰ~α~ᾱτον", "~ᾰ~α~ᾱτε";
    ind_act_3: "~ᾰ~α~ᾳ", _, "~ᾰ~α~ωσῐ|~ᾰ~α~ωσῐν";
    ind_mid_1: "~ᾰ~α~ωμαι", _, "~ᾰ~α~ωμεθᾰ";
    ind_mid_2: "~ᾰ~α~ᾳ", "~ᾰ~α~ᾱσθον", "~ᾰ~α~ᾱσθε";
    ind_mid_3: "~ᾰ~α~ᾱται", _, "~ᾰ~α~ωνται";
    subj_act_1: "~ᾰ~α~ω", _, "~ᾰ~α~ωμεν";
    subj_act_2: "~ᾰ~α~ᾳς", "~ᾰ~α~ᾱτον", "~ᾰ~α~ᾱτε";
    subj_act_3: "~ᾰ~α~ᾳ", _, "~ᾰ~α~ωσῐ|~ᾰ~α~ωσῐν";
    subj_mid_1: "~ᾰ~α~ωμαι", _, "~ᾰ~α~ωμεθᾰ";
    subj_mid_2: _, "~ᾰ~α~ᾱσθον", "~ᾰ~α~ᾱσθε";
    subj_mid_3: "~ᾰ~α~ᾱται", _, "~ᾰ~α~ωνται";
    opt_act_1: "~ᾰ~α~ῳην|~ᾰ~α~ῳμῐ", _, "~ᾰ~α~ῳμεν|~ᾰ~α~ῳημεν";
    opt_act_2: "~ᾰ~α~ῳης|~ᾰ~α~ῳς", "~ᾰ~α~ῳτον|~ᾰ~α~ῳητον", "~ᾰ~α~ῳτε|~ᾰ~α~ῳητε";
    opt_act_3: "~ᾰ~α~ῳη|~ᾰ~α~ῳ", "~ᾰ~α~ῳτην|~ᾰ~α~ῳητην", "~ᾰ~α~ῳεν|~ᾰ~α~ῳησᾰν";
    opt_mid_1: "~ᾰ~α~ῳμην", _, "~ᾰ~α~ῳμεθᾰ";
    opt_mid_2: "~ᾰ~α~ῳο", "~ᾰ~α~ῳσθον", "~ᾰ~α~ῳσθε";
    opt_mid_3: "~ᾰ~α~ῳτο", "~ᾰ~α~ῳσθην", "~ᾰ~α~ῳντο";
    imp_act_2: "~ᾰ~α~ᾱ", _, _;
    imp_act_3: "~ᾰ~α~ᾱτω", "~ᾰ~α~ᾱτων", "~ᾰ~α~ωντων";
    imp_mid_2: "~ᾰ~α~ω", _, _;
    imp_mid_3: "~ᾰ~α~ᾱσθω", "~ᾰ~α~ᾱσθων", _;
    imperfect_act_1: "~α~ε~ων", _, "~α~ε~ωμεν";
    imperfect_act_2: "~α~ε~ᾱς", "~α~ε~ᾱτον", "~α~ε~ᾱτε";
    imperfect_act_3: "~α~ε~ᾱ", "~α~ε~ᾱτην", "~α~ε~ων";
    imperfect_mid_1: "~α~ε~ωμην", _, "~α~ε~ωμεθᾰ";
    imperfect_mid_2: "~α~ε~ω", "~α~ε~ᾱσθον", "~α~ε~ᾱσθε";
    imperfect_mid_3: "~α~ε~ᾱτο", "~α~ε~ᾱσθην", "~α~ε~ωντο";
    aorist_subj_act_1: "~ᾰ~α~ᾰσω", _, "~ᾰ~α~ᾰσωμεν";
    aorist_subj_act_2: "~ᾰ~α~ᾰσῃς", "~ᾰ~α~ᾰσητον", "~ᾰ~α~ᾰσητε";
    aorist_subj_act_3: "~ᾰ~α~ᾰσῃ", _, "~ᾰ~α~ᾰσωσῐ|~ᾰ~α~ᾰσωσῐν";
    aorist_subj_mid_1: "~ᾰ~α~ᾰσωμαι", _, "~ᾰ~α~ᾰσωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰ~α~ᾰσησθον", "~ᾰ~α~ᾰσησθε";
    aorist_subj_mid_3: "~ᾰ~α~ᾰσηται", _, "~ᾰ~α~ᾰσωνται";
    aorist_subj_pass_1: "~ᾰ~α~ασθω", _, "~ᾰ~α~ασθωμεν";
    aorist_subj_pass_2: "~ᾰ~α~ασθῃς", "~ᾰ~α~ασθητον", "~ᾰ~α~ασθητε";
    aorist_subj_pass_3: "~ᾰ~α~ασθῃ", _, "~ᾰ~α~ασθωσῐ|~ᾰ~α~ασθωσῐν";
    aorist_opt_act_1: "~ᾰ~α~ᾰσαιμῐ", _, "~ᾰ~α~ᾰσαιμεν";
    aorist_opt_act_2: "~ᾰ~α~ᾰσειᾰς|~ᾰ~α~ᾰσαις", "~ᾰ~α~ᾰσαιτον", "~ᾰ~α~ᾰσαιτε";
    aorist_opt_act_3: "~ᾰ~α~ᾰσειε|~ᾰ~α~ᾰσειεν|~ᾰ~α~ᾰσαι", "~ᾰ~α~ᾰσαιτην", "~ᾰ~α~ᾰσειᾰν|~ᾰ~α~ᾰσαιεν";
    aorist_opt_mid_1: "~ᾰ~α~ᾰσαιμην", _, "~ᾰ~α~ᾰσαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~α~ᾰσαιο", "~ᾰ~α~ᾰσαισθον", "~ᾰ~α~ᾰσαισθε";
    aorist_opt_mid_3: "~ᾰ~α~ᾰσαιτο", "~ᾰ~α~ᾰσαισθην", "~ᾰ~α~ᾰσαιντο";
    aorist_opt_pass_1: "~ᾰ~α~ασθειην", _, "~ᾰ~α~ασθεῑμεν|~ᾰ~α~ασθειημεν";
    aorist_opt_pass_2: "~ᾰ~α~ασθειης", "~ᾰ~α~ασθεῑτον|~ᾰ~α~ασθειητον", "~ᾰ~α~ασθεῑτε|~ᾰ~α~ασθειητε";
    aorist_opt_pass_3: "~ᾰ~α~ασθειη", "~ᾰ~α~ασθειτην|~ᾰ~α~ασθειητην", "~ᾰ~α~ασθεῑεν|~ᾰ~α~ασθειησᾰν";
    aorist_imp_act_2: "~ᾰ~α~ᾰσον", "~ᾰ~α~ᾰσᾰτον", "~ᾰ~α~ᾰσᾰτε";
    aorist_imp_act_3: "~ᾰ~α~ᾰσᾰτω", "~ᾰ~α~ᾰσᾰτων", "~ᾰ~α~ᾰσᾰντων";
    aorist_imp_mid_2: "~ᾰ~α~ᾰσαι", "~ᾰ~α~ᾰσᾰσθον", "~ᾰ~α~ᾰσᾰσθε";
    aorist_imp_mid_3: "~ᾰ~α~ᾰσᾰσθω", "~ᾰ~α~ᾰσᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~α~ασθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~α~ασθητω", "~ᾰ~α~ασθητων", "~ᾰ~α~ασθεντων";
    perfect_ind_mid_1: "~ᾰ~ᾰ%ε~ᾰσμαι", _, "~ᾰ~ᾰ%ε~ᾰσμεθᾰ";
    perfect_ind_mid_2: "~ᾰ~ᾰ%ε~ᾰσαι", "~ᾰ~ᾰ%ε~ᾰσθον", "~ᾰ~ᾰ%ε~ᾰσθε";
    perfect_ind_mid_3: "~ᾰ~ᾰ%ε~ᾰσται", _, "~ᾰ~ᾰ%ε~ᾰσᾰται";
    perfect_imp_mid_2: "~ᾰ~ᾰ%ε~ᾰσο", _, _;
    perfect_imp_mid_3: "~ᾰ~ᾰ%ε~ᾰσθω", "~ᾰ~ᾰ%ε~ᾰσθων", _;
    infinitive_pres: "~ᾰ~α~ᾱν", "~ᾰ~α~ᾱσθαι", _;
    infinitive_aor: "~ᾰ~α~ᾰσαι", "~ᾰ~α~ᾰσᾰσθαι", "~ᾰ~α~ασθηναι";
    infinitive_perf: _, "~ᾰ~ᾰ%ε~ᾰσθαι", _;
    participle_pres: "~ᾰ~α~ων", "~ᾰ~α~ωμενος", _;
    participle_aor: "~ᾰ~α~ᾰσᾱς", "~ᾰ~α~ᾰσᾰμενος", "~ᾰ~α~ασθεις";
    participle_perf: _, "~ᾰ~ᾰ%ε~ᾰσμενος", _;
    ),
    // Ἀγωνιζομαι: 1 words (ἀγωνιζομαι). Like Κεῑμαι, except in 75 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἀγωνιζομαι", 1, "ἀ~ιζομαι", like Κεῑμαι;
    stem imperfect: "~ῐζ- like ἀγωνιζομαι";
    stem aorist_ind: "~ῐσ- like ἀγωνιζομαι";
    stem pluperfect: "~ῐ- like ἀγωνιζομαι";
    ind_act_1: "ἀ~ιζομαι", _, _;
    ind_mid_1: "ἀ~ῐζομαι", _, "ἀ~ῐζομεθᾰ";
    ind_mid_2: "ἀ~ῐζῃ|ἀ~ῐζει", "ἀ~ῐζεσθον", "ἀ~ῐζεσθε";
    ind_mid_3: "ἀ~ῐζεται", _, "ἀ~ῐζονται";
    subj_mid_1: "ἀ~ῐζωμαι", _, "ἀ~ῐζωμεθᾰ";
    subj_mid_2: "ἀ~ῐζῃ", "ἀ~ῐζησθον", "ἀ~ῐζησθε";
    subj_mid_3: "ἀ~ῐζηται", _, "ἀ~ῐζωνται";
    opt_mid_1: "ἀ~ῐζοιμην", _, "ἀ~ῐζοιμεθᾰ";
    opt_mid_2: "ἀ~ῐζοιο", "ἀ~ῐζοισθον", "ἀ~ῐζοισθε";
    opt_mid_3: "ἀ~ῐζοιτο", "ἀ~ῐζοισθην", "ἀ~ῐζοιντο";
    imp_mid_2: "ἀ~ῐζου", _, _;
    imp_mid_3: "ἀ~ῐζεσθω", "ἀ~ῐζεσθων", _;
    future_ind_mid_1: "ἀ~ῐοῡμαι", _, "ἀ~ῐουμεθᾰ";
    future_ind_mid_2: "ἀ~ῐεῑ|ἀ~ῐῃ", "ἀ~ῐεῑσθον", "ἀ~ῐεῑσθε";
    future_ind_mid_3: "ἀ~ῐεῑται", _, "ἀ~ῐοῡνται";
    future_opt_mid_1: "ἀ~ῐοιμην", _, "ἀ~ῐοιμεθᾰ";
    future_opt_mid_2: "ἀ~ῐοῑο", "ἀ~ῐοῑσθον", "ἀ~ῐοῑσθε";
    future_opt_mid_3: "ἀ~ῐοῑτο", "ἀ~ῐοισθην", "ἀ~ῐοῑντο";
    aorist_subj_mid_1: "ἀ~ῐσωμαι", _, "ἀ~ῐσωμεθᾰ";
    aorist_subj_mid_2: "ἀ~ῐσῃ", "ἀ~ῐσησθον", "ἀ~ῐσησθε";
    aorist_subj_mid_3: "ἀ~ῐσηται", _, "ἀ~ῐσωνται";
    aorist_opt_mid_1: "ἀ~ῐσαιμην", _, "ἀ~ῐσαιμεθᾰ";
    aorist_opt_mid_2: "ἀ~ῐσαιο", "ἀ~ῐσαισθον", "ἀ~ῐσαισθε";
    aorist_opt_mid_3: "ἀ~ῐσαιτο", "ἀ~ῐσαισθην", "ἀ~ῐσαιντο";
    aorist_imp_mid_2: "ἀ~ῐσαι", "ἀ~ῐσᾰσθον", "ἀ~ῐσᾰσθε";
    aorist_imp_mid_3: "ἀ~ῐσᾰσθω", "ἀ~ῐσᾰσθων", _;
    perfect_ind_mid_1: "ἠ~ῐσμαι", _, "ἠ~ῐσμεθᾰ";
    perfect_ind_mid_2: "ἠ~ῐσαι", "ἠ~ῐσθον", "ἠ~ῐσθε";
    perfect_ind_mid_3: "ἠ~ῐσται", _, "ἠ~ῐδᾰται";
    perfect_imp_mid_2: "ἠ~ῐσο", _, _;
    perfect_imp_mid_3: "ἠ~ῐσθω", "ἠ~ῐσθων", _;
    infinitive_fut: _, "ἀ~ῐεῑσθαι", _;
    infinitive_aor: _, "ἀ~ῐσᾰσθαι", _;
    participle_fut: _, "ἀ~ῐουμενος", _;
    participle_aor: _, "ἀ~ῐσᾰμενος", _;
    ),
    // Ἀπιστεω: 1 words (ἀπιστεω). Like Ἐκφερω, except in 165 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀπιστεω", 1, "ἀ~ι~εω", like Ἐκφερω;
    stem aorist_ind: "~ῐ~ησ- like ἐλπῐζω";
    ind_act_1: "ἀ~ῐ~ω", _, "ἀ~ῐ~οῡμεν";
    ind_act_2: "ἀ~ῐ~εῑς", "ἀ~ῐ~εῑτον", "ἀ~ῐ~εῑτε";
    ind_act_3: "ἀ~ῐ~εῑ", _, "ἀ~ῐ~οῡσῐ|ἀ~ῐ~οῡσῐν";
    ind_mid_1: "ἀ~ῐ~οῡμαι", _, "ἀ~ῐ~ουμεθᾰ";
    ind_mid_2: "ἀ~ῐ~εῑ|ἀ~ῐ~ῃ", "ἀ~ῐ~εῑσθον", "ἀ~ῐ~εῑσθε";
    ind_mid_3: "ἀ~ῐ~εῑται", _, "ἀ~ῐ~οῡνται";
    subj_act_1: "ἀ~ῐ~ω", _, "ἀ~ῐ~ωμεν";
    subj_act_2: "ἀ~ῐ~ῃς", "ἀ~ῐ~ητον", "ἀ~ῐ~ητε";
    subj_act_3: "ἀ~ῐ~ῃ", _, "ἀ~ῐ~ωσῐ|ἀ~ῐ~ωσῐν";
    subj_mid_1: "ἀ~ῐ~ωμαι", _, "ἀ~ῐ~ωμεθᾰ";
    subj_mid_2: _, "ἀ~ῐ~ησθον", "ἀ~ῐ~ησθε";
    subj_mid_3: "ἀ~ῐ~ηται", _, "ἀ~ῐ~ωνται";
    opt_act_1: "ἀ~ῐ~οιην|ἀ~ῐ~οῑμῐ", _, "ἀ~ῐ~οῑμεν|ἀ~ῐ~οιημεν";
    opt_act_2: "ἀ~ῐ~οιης|ἀ~ῐ~οῑς", "ἀ~ῐ~οῑτον|ἀ~ῐ~οιητον", "ἀ~ῐ~οῑτε|ἀ~ῐ~οιητε";
    opt_act_3: "ἀ~ῐ~οιη|ἀ~ῐ~οῑ", "ἀ~ῐ~οιτην|ἀ~ῐ~οιητην", "ἀ~ῐ~οῑεν|ἀ~ῐ~οιησᾰν";
    opt_mid_1: "ἀ~ῐ~οιμην", _, "ἀ~ῐ~οιμεθᾰ";
    opt_mid_2: "ἀ~ῐ~οῑο", "ἀ~ῐ~οῑσθον", "ἀ~ῐ~οῑσθε";
    opt_mid_3: "ἀ~ῐ~οῑτο", "ἀ~ῐ~οισθην", "ἀ~ῐ~οῑντο";
    imp_act_2: "ἀ~ῐ~ει", _, _;
    imp_act_3: "ἀ~ῐ~ειτω", "ἀ~ῐ~ειτων", "ἀ~ῐ~ουντων";
    imp_mid_2: "ἀ~ῐ~οῡ", _, _;
    imp_mid_3: "ἀ~ῐ~εισθω", "ἀ~ῐ~εισθων", _;
    imperfect_act_1: "ἠ~ῐ~ουν", _, "ἠ~ῐ~οῡμεν";
    imperfect_act_2: "ἠ~ῐ~εις", "ἠ~ῐ~εῑτον", "ἠ~ῐ~εῑτε";
    imperfect_act_3: "ἠ~ῐ~ει", "ἠ~ῐ~ειτην", "ἠ~ῐ~ουν";
    imperfect_mid_1: "ἠ~ῐ~ουμην", _, "ἠ~ῐ~ουμεθᾰ";
    imperfect_mid_2: "ἠ~ῐ~οῡ", "ἠ~ῐ~εῑσθον", "ἠ~ῐ~εῑσθε";
    imperfect_mid_3: "ἠ~ῐ~εῑτο", "ἠ~ῐ~εισθην", "ἠ~ῐ~οῡντο";
    future_ind_act_1: "ἀ~ῐ~ησω", _, "ἀ~ῐ~ησομεν";
    future_ind_act_2: "ἀ~ῐ~ησεις", "ἀ~ῐ~ησετον", "ἀ~ῐ~ησετε";
    future_ind_act_3: "ἀ~ῐ~ησει", _, "ἀ~ῐ~ησουσῐ|ἀ~ῐ~ησουσῐν";
    future_ind_mid_1: "ἀ~ῐ~ησομαι", _, "ἀ~ῐ~ησομεθᾰ";
    future_ind_mid_2: "ἀ~ῐ~ησῃ|ἀ~ῐ~ησει", "ἀ~ῐ~ησεσθον", "ἀ~ῐ~ησεσθε";
    future_ind_mid_3: "ἀ~ῐ~ησεται", _, "ἀ~ῐ~ησονται";
    future_ind_pass_1: "ἀ~ῐ~ηθησομαι", _, "ἀ~ῐ~ηθησομεθᾰ";
    future_ind_pass_2: "ἀ~ῐ~ηθησῃ|ἀ~ῐ~ηθησει", "ἀ~ῐ~ηθησεσθον", "ἀ~ῐ~ηθησεσθε";
    future_ind_pass_3: "ἀ~ῐ~ηθησεται", _, "ἀ~ῐ~ηθησονται";
    future_opt_act_1: "ἀ~ῐ~ησοιμῐ", _, "ἀ~ῐ~ησοιμεν";
    future_opt_act_2: "ἀ~ῐ~ησοις", "ἀ~ῐ~ησοιτον", "ἀ~ῐ~ησοιτε";
    future_opt_act_3: "ἀ~ῐ~ησοι", "ἀ~ῐ~ησοιτην", "ἀ~ῐ~ησοιεν";
    future_opt_mid_1: "ἀ~ῐ~ησοιμην", _, "ἀ~ῐ~ησοιμεθᾰ";
    future_opt_mid_2: "ἀ~ῐ~ησοιο", "ἀ~ῐ~ησοισθον", "ἀ~ῐ~ησοισθε";
    future_opt_mid_3: "ἀ~ῐ~ησοιτο", "ἀ~ῐ~ησοισθην", "ἀ~ῐ~ησοιντο";
    future_opt_pass_1: "ἀ~ῐ~ηθησοιμην", _, "ἀ~ῐ~ηθησοιμεθᾰ";
    future_opt_pass_2: "ἀ~ῐ~ηθησοιο", "ἀ~ῐ~ηθησοισθον", "ἀ~ῐ~ηθησοισθε";
    future_opt_pass_3: "ἀ~ῐ~ηθησοιτο", "ἀ~ῐ~ηθησοισθην", "ἀ~ῐ~ηθησοιντο";
    aorist_subj_act_1: "ἀ~ῐ~ησω", _, "ἀ~ῐ~ησωμεν";
    aorist_subj_act_2: "ἀ~ῐ~ησῃς", "ἀ~ῐ~ησητον", "ἀ~ῐ~ησητε";
    aorist_subj_act_3: "ἀ~ῐ~ησῃ", _, "ἀ~ῐ~ησωσῐ|ἀ~ῐ~ησωσῐν";
    aorist_subj_mid_1: "ἀ~ῐ~ησωμαι", _, "ἀ~ῐ~ησωμεθᾰ";
    aorist_subj_mid_2: _, "ἀ~ῐ~ησησθον", "ἀ~ῐ~ησησθε";
    aorist_subj_mid_3: "ἀ~ῐ~ησηται", _, "ἀ~ῐ~ησωνται";
    aorist_opt_act_1: "ἀ~ῐ~ησαιμῐ", _, "ἀ~ῐ~ησαιμεν";
    aorist_opt_act_2: "ἀ~ῐ~ησειᾰς|ἀ~ῐ~ησαις", "ἀ~ῐ~ησαιτον", "ἀ~ῐ~ησαιτε";
    aorist_opt_act_3: "ἀ~ῐ~ησειε|ἀ~ῐ~ησειεν|ἀ~ῐ~ησαι", "ἀ~ῐ~ησαιτην", "ἀ~ῐ~ησειᾰν|ἀ~ῐ~ησαιεν";
    aorist_opt_mid_1: "ἀ~ῐ~ησαιμην", _, "ἀ~ῐ~ησαιμεθᾰ";
    aorist_opt_mid_2: "ἀ~ῐ~ησαιο", "ἀ~ῐ~ησαισθον", "ἀ~ῐ~ησαισθε";
    aorist_opt_mid_3: "ἀ~ῐ~ησαιτο", "ἀ~ῐ~ησαισθην", "ἀ~ῐ~ησαιντο";
    aorist_imp_act_2: "ἀ~ῐ~ησον", "ἀ~ῐ~ησᾰτον", "ἀ~ῐ~ησᾰτε";
    aorist_imp_act_3: "ἀ~ῐ~ησᾰτω", "ἀ~ῐ~ησᾰτων", "ἀ~ῐ~ησᾰντων";
    aorist_imp_mid_2: "ἀ~ῐ~ησαι", "ἀ~ῐ~ησᾰσθον", "ἀ~ῐ~ησᾰσθε";
    aorist_imp_mid_3: "ἀ~ῐ~ησᾰσθω", "ἀ~ῐ~ησᾰσθων", _;
    infinitive_pres: "ἀ~ῐ~εῑν", "ἀ~ῐ~εῑσθαι", _;
    infinitive_fut: "ἀ~ῐ~ησειν", "ἀ~ῐ~ησεσθαι", "ἀ~ῐ~ηθησεσθαι";
    infinitive_aor: "ἀ~ῐ~ησαι", "ἀ~ῐ~ησᾰσθαι", _;
    participle_pres: "ἀ~ῐ~ων", "ἀ~ῐ~ουμενος", _;
    participle_fut: "ἀ~ῐ~ησων", "ἀ~ῐ~ησομενος", "ἀ~ῐ~ηθησομενος";
    participle_aor: "ἀ~ῐ~ησᾱς", "ἀ~ῐ~ησᾰμενος", _;
    ),
    // Χᾰριζω: 1 words (χᾰριζω). Like Ἐπορον, except in 41 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("χᾰριζω", 1, "~ιζω", like Ἐπορον;
    stem aorist_ind: "~ῐσ- like χᾰριζω";
    ind_act_1: "~ιζω", _, _;
    future_ind_act_1: "~ῐω", _, "~ῐοῡμεν";
    future_ind_act_2: "~ῐεῑς", "~ῐεῑτον", "~ῐεῑτε";
    future_ind_act_3: "~ῐεῑ", _, "~ῐοῡσῐ|~ῐοῡσῐν";
    future_opt_act_1: "~ῐοιην|~ῐοῑμῐ", _, "~ῐοῑμεν|~ῐοιημεν";
    future_opt_act_2: "~ῐοιης|~ῐοῑς", "~ῐοῑτον|~ῐοιητον", "~ῐοῑτε|~ῐοιητε";
    future_opt_act_3: "~ῐοιη|~ῐοῑ", "~ῐοιτην|~ῐοιητην", "~ῐοῑεν|~ῐοιησᾰν";
    aorist_subj_act_1: "~ῐσω", _, "~ῐσωμεν";
    aorist_subj_act_2: "~ῐσῃς", "~ῐσητον", "~ῐσητε";
    aorist_subj_act_3: "~ῐσῃ", _, "~ῐσωσῐ|~ῐσωσῐν";
    aorist_opt_act_1: "~ῐσαιμῐ", _, "~ῐσαιμεν";
    aorist_opt_act_2: "~ῐσειᾰς|~ῐσαις", "~ῐσαιτον", "~ῐσαιτε";
    aorist_opt_act_3: "~ῐσειε|~ῐσειεν|~ῐσαι", "~ῐσαιτην", "~ῐσειᾰν|~ῐσαιεν";
    aorist_imp_act_2: "~ῐσον", "~ῐσᾰτον", "~ῐσᾰτε";
    aorist_imp_act_3: "~ῐσᾰτω", "~ῐσᾰτων", "~ῐσᾰντων";
    infinitive_fut: "~ῐεῑν", _, _;
    infinitive_aor: "~ῐσαι", _, _;
    participle_fut: "~ῐων", _, _;
    participle_aor: "~ῐσᾱς", _, _;
    ),
    // Διαφορεω: 1 words (διαφορεω). Like Πεμπω, except in 236 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("διαφορεω", 1, "~ια~εω", like Πεμπω;
    stem aorist_ind: "~ῐε~ησ- like εὐλογεω";
    stem aorist_ind_pass: "~ῐε~ηθη- like ἐλπω";
    stem pluperfect_act: "~ῐε%ε~ηκ- like εὐλογεω";
    ind_act_1: "~ῐᾰ~ω", _, "~ῐᾰ~οῡμεν";
    ind_act_2: "~ῐᾰ~εῑς", "~ῐᾰ~εῑτον", "~ῐᾰ~εῑτε";
    ind_act_3: "~ῐᾰ~εῑ", _, "~ῐᾰ~οῡσῐ|~ῐᾰ~οῡσῐν";
    ind_mid_1: "~ῐᾰ~οῡμαι", _, "~ῐᾰ~ουμεθᾰ";
    ind_mid_2: "~ῐᾰ~εῑ|~ῐᾰ~ῃ", "~ῐᾰ~εῑσθον", "~ῐᾰ~εῑσθε";
    ind_mid_3: "~ῐᾰ~εῑται", _, "~ῐᾰ~οῡνται";
    subj_act_1: "~ῐᾰ~ω", _, "~ῐᾰ~ωμεν";
    subj_act_2: "~ῐᾰ~ῃς", "~ῐᾰ~ητον", "~ῐᾰ~ητε";
    subj_act_3: "~ῐᾰ~ῃ", _, "~ῐᾰ~ωσῐ|~ῐᾰ~ωσῐν";
    subj_mid_1: "~ῐᾰ~ωμαι", _, "~ῐᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ῐᾰ~ησθον", "~ῐᾰ~ησθε";
    subj_mid_3: "~ῐᾰ~ηται", _, "~ῐᾰ~ωνται";
    opt_act_1: "~ῐᾰ~οιην|~ῐᾰ~οῑμῐ", _, "~ῐᾰ~οῑμεν|~ῐᾰ~οιημεν";
    opt_act_2: "~ῐᾰ~οιης|~ῐᾰ~οῑς", "~ῐᾰ~οῑτον|~ῐᾰ~οιητον", "~ῐᾰ~οῑτε|~ῐᾰ~οιητε";
    opt_act_3: "~ῐᾰ~οιη|~ῐᾰ~οῑ", "~ῐᾰ~οιτην|~ῐᾰ~οιητην", "~ῐᾰ~οῑεν|~ῐᾰ~οιησᾰν";
    opt_mid_1: "~ῐᾰ~οιμην", _, "~ῐᾰ~οιμεθᾰ";
    opt_mid_2: "~ῐᾰ~οῑο", "~ῐᾰ~οῑσθον", "~ῐᾰ~οῑσθε";
    opt_mid_3: "~ῐᾰ~οῑτο", "~ῐᾰ~οισθην", "~ῐᾰ~οῑντο";
    imp_act_2: "~ῐᾰ~ει", _, _;
    imp_act_3: "~ῐᾰ~ειτω", "~ῐᾰ~ειτων", "~ῐᾰ~ουντων";
    imp_mid_2: "~ῐᾰ~οῡ", _, _;
    imp_mid_3: "~ῐᾰ~εισθω", "~ῐᾰ~εισθων", _;
    imperfect_act_1: "~ῐε~ουν", _, "~ῐε~οῡμεν";
    imperfect_act_2: "~ῐε~εις", "~ῐε~εῑτον", "~ῐε~εῑτε";
    imperfect_act_3: "~ῐε~ει", "~ῐε~ειτην", "~ῐε~ουν";
    imperfect_mid_1: "~ῐε~ουμην", _, "~ῐε~ουμεθᾰ";
    imperfect_mid_2: "~ῐε~οῡ", "~ῐε~εῑσθον", "~ῐε~εῑσθε";
    imperfect_mid_3: "~ῐε~εῑτο", "~ῐε~εισθην", "~ῐε~οῡντο";
    future_ind_act_1: "~ῐᾰ~ησω", _, "~ῐᾰ~ησομεν";
    future_ind_act_2: "~ῐᾰ~ησεις", "~ῐᾰ~ησετον", "~ῐᾰ~ησετε";
    future_ind_act_3: "~ῐᾰ~ησει", _, "~ῐᾰ~ησουσῐ|~ῐᾰ~ησουσῐν";
    future_ind_mid_1: "~ῐᾰ~ησομαι", _, "~ῐᾰ~ησομεθᾰ";
    future_ind_mid_2: "~ῐᾰ~ησῃ|~ῐᾰ~ησει", "~ῐᾰ~ησεσθον", "~ῐᾰ~ησεσθε";
    future_ind_mid_3: "~ῐᾰ~ησεται", _, "~ῐᾰ~ησονται";
    future_ind_pass_1: "~ῐᾰ~ηθησομαι", _, "~ῐᾰ~ηθησομεθᾰ";
    future_ind_pass_2: "~ῐᾰ~ηθησῃ|~ῐᾰ~ηθησει", "~ῐᾰ~ηθησεσθον", "~ῐᾰ~ηθησεσθε";
    future_ind_pass_3: "~ῐᾰ~ηθησεται", _, "~ῐᾰ~ηθησονται";
    future_opt_act_1: "~ῐᾰ~ησοιμῐ", _, "~ῐᾰ~ησοιμεν";
    future_opt_act_2: "~ῐᾰ~ησοις", "~ῐᾰ~ησοιτον", "~ῐᾰ~ησοιτε";
    future_opt_act_3: "~ῐᾰ~ησοι", "~ῐᾰ~ησοιτην", "~ῐᾰ~ησοιεν";
    future_opt_mid_1: "~ῐᾰ~ησοιμην", _, "~ῐᾰ~ησοιμεθᾰ";
    future_opt_mid_2: "~ῐᾰ~ησοιο", "~ῐᾰ~ησοισθον", "~ῐᾰ~ησοισθε";
    future_opt_mid_3: "~ῐᾰ~ησοιτο", "~ῐᾰ~ησοισθην", "~ῐᾰ~ησοιντο";
    future_opt_pass_1: "~ῐᾰ~ηθησοιμην", _, "~ῐᾰ~ηθησοιμεθᾰ";
    future_opt_pass_2: "~ῐᾰ~ηθησοιο", "~ῐᾰ~ηθησοισθον", "~ῐᾰ~ηθησοισθε";
    future_opt_pass_3: "~ῐᾰ~ηθησοιτο", "~ῐᾰ~ηθησοισθην", "~ῐᾰ~ηθησοιντο";
    aorist_subj_act_1: "~ῐᾰ~ησω", _, "~ῐᾰ~ησωμεν";
    aorist_subj_act_2: "~ῐᾰ~ησῃς", "~ῐᾰ~ησητον", "~ῐᾰ~ησητε";
    aorist_subj_act_3: "~ῐᾰ~ησῃ", _, "~ῐᾰ~ησωσῐ|~ῐᾰ~ησωσῐν";
    aorist_subj_mid_1: "~ῐᾰ~ησωμαι", _, "~ῐᾰ~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~ῐᾰ~ησησθον", "~ῐᾰ~ησησθε";
    aorist_subj_mid_3: "~ῐᾰ~ησηται", _, "~ῐᾰ~ησωνται";
    aorist_subj_pass_1: "~ῐᾰ~ηθω", _, "~ῐᾰ~ηθωμεν";
    aorist_subj_pass_2: "~ῐᾰ~ηθῃς", "~ῐᾰ~ηθητον", "~ῐᾰ~ηθητε";
    aorist_subj_pass_3: "~ῐᾰ~ηθῃ", _, "~ῐᾰ~ηθωσῐ|~ῐᾰ~ηθωσῐν";
    aorist_opt_act_1: "~ῐᾰ~ησαιμῐ", _, "~ῐᾰ~ησαιμεν";
    aorist_opt_act_2: "~ῐᾰ~ησειᾰς|~ῐᾰ~ησαις", "~ῐᾰ~ησαιτον", "~ῐᾰ~ησαιτε";
    aorist_opt_act_3: "~ῐᾰ~ησειε|~ῐᾰ~ησειεν|~ῐᾰ~ησαι", "~ῐᾰ~ησαιτην", "~ῐᾰ~ησειᾰν|~ῐᾰ~ησαιεν";
    aorist_opt_mid_1: "~ῐᾰ~ησαιμην", _, "~ῐᾰ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ῐᾰ~ησαιο", "~ῐᾰ~ησαισθον", "~ῐᾰ~ησαισθε";
    aorist_opt_mid_3: "~ῐᾰ~ησαιτο", "~ῐᾰ~ησαισθην", "~ῐᾰ~ησαιντο";
    aorist_opt_pass_1: "~ῐᾰ~ηθειην", _, "~ῐᾰ~ηθεῑμεν|~ῐᾰ~ηθειημεν";
    aorist_opt_pass_2: "~ῐᾰ~ηθειης", "~ῐᾰ~ηθεῑτον|~ῐᾰ~ηθειητον", "~ῐᾰ~ηθεῑτε|~ῐᾰ~ηθειητε";
    aorist_opt_pass_3: "~ῐᾰ~ηθειη", "~ῐᾰ~ηθειτην|~ῐᾰ~ηθειητην", "~ῐᾰ~ηθεῑεν|~ῐᾰ~ηθειησᾰν";
    aorist_imp_act_2: "~ῐᾰ~ησον", "~ῐᾰ~ησᾰτον", "~ῐᾰ~ησᾰτε";
    aorist_imp_act_3: "~ῐᾰ~ησᾰτω", "~ῐᾰ~ησᾰτων", "~ῐᾰ~ησᾰντων";
    aorist_imp_mid_2: "~ῐᾰ~ησαι", "~ῐᾰ~ησᾰσθον", "~ῐᾰ~ησᾰσθε";
    aorist_imp_mid_3: "~ῐᾰ~ησᾰσθω", "~ῐᾰ~ησᾰσθων", _;
    aorist_imp_pass_2: "~ῐᾰ~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ῐᾰ~ηθητω", "~ῐᾰ~ηθητων", "~ῐᾰ~ηθεντων";
    perfect_ind_act_1: "~ῐᾰ%ε~ηκᾰ", _, "~ῐᾰ%ε~ηκᾰμεν";
    perfect_ind_act_2: "~ῐᾰ%ε~ηκᾰς", "~ῐᾰ%ε~ηκᾰτον", "~ῐᾰ%ε~ηκᾰτε";
    perfect_ind_act_3: "~ῐᾰ%ε~ηκε|~ῐᾰ%ε~ηκεν", _, "~ῐᾰ%ε~ηκᾱσῐ|~ῐᾰ%ε~ηκᾱσῐν";
    perfect_ind_mid_1: "~ῐᾰ%ε~ημαι", _, "~ῐᾰ%ε~ημεθᾰ";
    perfect_ind_mid_2: "~ῐᾰ%ε~ησαι", "~ῐᾰ%ε~ησθον", "~ῐᾰ%ε~ησθε";
    perfect_ind_mid_3: "~ῐᾰ%ε~ηται", _, "~ῐᾰ%ε~ηνται";
    perfect_subj_act_1: "~ῐᾰ%ε~ηκω", _, "~ῐᾰ%ε~ηκωμεν";
    perfect_subj_act_2: "~ῐᾰ%ε~ηκῃς", "~ῐᾰ%ε~ηκητον", "~ῐᾰ%ε~ηκητε";
    perfect_subj_act_3: "~ῐᾰ%ε~ηκῃ", _, "~ῐᾰ%ε~ηκωσῐ|~ῐᾰ%ε~ηκωσῐν";
    perfect_opt_act_1: "~ῐᾰ%ε~ηκοιμῐ|~ῐᾰ%ε~ηκοιην", _, "~ῐᾰ%ε~ηκοιμεν";
    perfect_opt_act_2: "~ῐᾰ%ε~ηκοις|~ῐᾰ%ε~ηκοιης", "~ῐᾰ%ε~ηκοιτον", "~ῐᾰ%ε~ηκοιτε";
    perfect_opt_act_3: "~ῐᾰ%ε~ηκοι|~ῐᾰ%ε~ηκοιη", "~ῐᾰ%ε~ηκοιτην", "~ῐᾰ%ε~ηκοιεν";
    perfect_imp_act_2: "~ῐᾰ%ε~ηκε", "~ῐᾰ%ε~ηκετον", "~ῐᾰ%ε~ηκετε";
    perfect_imp_act_3: "~ῐᾰ%ε~ηκετω", "~ῐᾰ%ε~ηκετων", "~ῐᾰ%ε~ηκοντων";
    perfect_imp_mid_2: "~ῐᾰ%ε~ησο", _, _;
    perfect_imp_mid_3: "~ῐᾰ%ε~ησθω", "~ῐᾰ%ε~ησθων", _;
    pluperfect_mid_1: "~ῐε%ε~ημην", _, "~ῐε%ε~ημεθᾰ";
    pluperfect_mid_2: "~ῐε%ε~ησο", "~ῐε%ε~ησθον", "~ῐε%ε~ησθε";
    pluperfect_mid_3: "~ῐε%ε~ητο", "~ῐε%ε~ησθην", "~ῐε%ε~ηντο";
    infinitive_pres: "~ῐᾰ~εῑν", "~ῐᾰ~εῑσθαι", _;
    infinitive_fut: "~ῐᾰ~ησειν", "~ῐᾰ~ησεσθαι", "~ῐᾰ~ηθησεσθαι";
    infinitive_aor: "~ῐᾰ~ησαι", "~ῐᾰ~ησᾰσθαι", "~ῐᾰ~ηθηναι";
    infinitive_perf: "~ῐᾰ%ε~ηκεναι", "~ῐᾰ%ε~ησθαι", _;
    participle_pres: "~ῐᾰ~ων", "~ῐᾰ~ουμενος", _;
    participle_fut: "~ῐᾰ~ησων", "~ῐᾰ~ησομενος", "~ῐᾰ~ηθησομενος";
    participle_aor: "~ῐᾰ~ησᾱς", "~ῐᾰ~ησᾰμενος", "~ῐᾰ~ηθεις";
    participle_perf: "~ῐᾰ%ε~ηκως", "~ῐᾰ%ε~ημενος", _;
    ),
    // Τελεθω: 1 words (τελεθω). Like Μῑγνῡμῐ, except in 53 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("τελεθω", 1, "~ω", like Μῑγνῡμῐ as SG_AS_PL;
    ind_act_1: "~εσκω", _, "~εσκομεν";
    ind_act_2: "~εσκεις", "~εσκετον", "~εσκετε";
    ind_act_3: "~εσκει", _, "~εσκουσῐ|~εσκουσῐν";
    ind_mid_1: "~εσκομαι", _, "~εσκομεθᾰ";
    ind_mid_2: "~εσκεαι|~εσκῃ", "~εσκεσθον", "~εσκεσθε";
    ind_mid_3: "~εσκεται", _, "~εσκονται";
    subj_act_1: "~εσκω", _, "~εσκωμεν";
    subj_act_2: "~εσκῃς", "~εσκητον", "~εσκητε";
    subj_act_3: "~εσκῃ", _, "~εσκωσῐ|~εσκωσῐν";
    subj_mid_1: "~εσκωμαι", _, "~εσκωμεθᾰ";
    subj_mid_2: "~εσκηαι|~εσκῃ", "~εσκησθον", "~εσκησθε";
    subj_mid_3: "~εσκηται", _, "~εσκωνται";
    opt_act_1: "~εσκοιμῐ", _, "~εσκοιμεν";
    opt_act_2: "~εσκοις", "~εσκοιτον", "~εσκοιτε";
    opt_act_3: "~εσκοι", "~εσκοιτην", "~εσκοιεν";
    opt_mid_1: "~εσκοιμην", _, "~εσκοιμεθᾰ";
    opt_mid_2: "~εσκοιο", "~εσκοισθον", "~εσκοισθε";
    opt_mid_3: "~εσκοιτο", "~εσκοισθην", "~εσκοιᾰτο";
    imp_act_2: "~εσκε", _, _;
    imp_act_3: "~εσκετω", "~εσκετων", "~εσκοντων";
    imp_mid_2: "~εσκεο|~εσκευ", _, _;
    imp_mid_3: "~εσκεσθω", "~εσκεσθων", _;
    infinitive_pres: "~εσκειν", _, _;
    participle_pres: "~εσκων", _, _;
    ),
    // Δῐᾰσευομαι: 1 words (δῐᾰσευομαι). Like Νισσομαι, except in 39 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("δῐᾰσευομαι", 1, "~ᾰσ~υομαι", like Νισσομαι;
    stem imperfect: "~~σευ- like ὑποκοριζομαι";
    ind_act_1: "~ᾰσ~υομαι", _, _;
    ind_mid_1: "~ασ~υομαι", _, "~ασ~υομεθᾰ";
    ind_mid_2: "~ασ~υῃ|~ασ~υει", "~ασ~υεσθον", "~ασ~υεσθε";
    ind_mid_3: "~ασ~υεται", _, "~ασ~υονται";
    subj_mid_1: "~ασ~υωμαι", _, "~ασ~υωμεθᾰ";
    subj_mid_2: "~ασ~υῃ", "~ασ~υησθον", "~ασ~υησθε";
    subj_mid_3: "~ασ~υηται", _, "~ασ~υωνται";
    subj_pass_2: "~ασ~υῃ", _, _;
    opt_mid_1: "~ασ~υοιμην", _, "~ασ~υοιμεθᾰ";
    opt_mid_2: "~ασ~υοιο", "~ασ~υοισθον", "~ασ~υοισθε";
    opt_mid_3: "~ασ~υοιτο", "~ασ~υοισθην", "~ασ~υοιντο";
    imp_mid_2: "~ασ~υου", _, _;
    imp_mid_3: "~ασ~υεσθω", "~ασ~υεσθων", _;
    perfect_ind_mid_1: "~~σσῠμαι", _, "~~σσῠμεθᾰ";
    perfect_ind_mid_2: "~~σσῠσαι", "~~σσῠσθον", "~~σσῠσθε";
    perfect_ind_mid_3: "~~σσῠται", _, "~~σσῠνται";
    perfect_imp_mid_2: "~~σσῠσο", _, _;
    perfect_imp_mid_3: "~~σσῠσθω", "~~σσῠσθων", _;
    infinitive_pres: _, "~ασ~υεσθαι", _;
    participle_pres: _, "~ασ~υομενος", _;
    ),
    // Φθιω: 1 words (φθιω). Like Μῑγνῡμῐ, except in 199 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("φθιω", 1, "~ιω", like Μῑγνῡμῐ;
    stem aorist_ind: "~- like φθιω";
    stem aorist_ind_pass: "~ῐθη- like ἐχω";
    ind_act_1: "~ῐνω", _, "~ῐνομεν";
    ind_act_2: "~ῐνεις", "~ῐνετον", "~ῐνετε";
    ind_act_3: "~ῐνει", _, "~ῐνουσῐ|~ῐνουσῐν";
    ind_mid_1: "~ῐνομαι", _, "~ῐνομεθᾰ";
    ind_mid_2: "~ῐνῃ|~ῐνει", "~ῐνεσθον", "~ῐνεσθε";
    ind_mid_3: "~ῐνεται", _, "~ῐνονται";
    subj_act_1: "~ῐνω", _, "~ῐνωμεν";
    subj_act_2: "~ῐνῃς", "~ῐνητον", "~ῐνητε";
    subj_act_3: "~ῐνῃ", _, "~ῐνωσῐ|~ῐνωσῐν";
    subj_mid_1: "~ῐνωμαι", _, "~ῐνωμεθᾰ";
    subj_mid_2: _, "~ῐνησθον", "~ῐνησθε";
    subj_mid_3: "~ῐνηται", _, "~ῐνωνται";
    opt_act_1: "~ῐνοιμῐ", _, "~ῐνοιμεν";
    opt_act_2: "~ῐνοις", "~ῐνοιτον", "~ῐνοιτε";
    opt_act_3: "~ῐνοι", "~ῐνοιτην", "~ῐνοιεν";
    opt_mid_1: "~ῐνοιμην", _, "~ῐνοιμεθᾰ";
    opt_mid_2: "~ῐνοιο", "~ῐνοισθον", "~ῐνοισθε";
    opt_mid_3: "~ῐνοιτο", "~ῐνοισθην", "~ῐνοιντο";
    imp_act_2: "~ῐνε", _, _;
    imp_act_3: "~ῐνετω", "~ῐνετων", "~ῐνοντων";
    imp_mid_2: "~ῐνου", _, _;
    imp_mid_3: "~ῐνεσθω", "~ῐνεσθων", _;
    imperfect_act_1: "ἐ~ῐνον", _, "ἐ~ῐνομεν";
    imperfect_act_2: "ἐ~ῐνες", "ἐ~ῐνετον", "ἐ~ῐνετε";
    imperfect_act_3: "ἐ~ῐνε|ἐ~ῐνεν", "ἐ~ῐνετην", "ἐ~ῐνον";
    imperfect_mid_1: "ἐ~ῐνομην", _, "ἐ~ῐνομεθᾰ";
    imperfect_mid_2: "ἐ~ῐνου", "ἐ~ῐνεσθον", "ἐ~ῐνεσθε";
    imperfect_mid_3: "ἐ~ῐνετο", "ἐ~ῐνεσθην", "ἐ~ῐνοντο";
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
    aorist_subj_pass_1: "~ῐθω", _, "~ῐθωμεν";
    aorist_subj_pass_2: "~ῐθῃς", "~ῐθητον", "~ῐθητε";
    aorist_subj_pass_3: "~ῐθῃ", _, "~ῐθωσῐ|~ῐθωσῐν";
    aorist_opt_act_1: "~ῑσαιμῐ|~εισαιμῐ|~ῐνησαιμῐ", _, "~ῑσαιμεν|~εισαιμεν|~ῐνησαιμεν";
    aorist_opt_act_2: "~ῑσειᾰς|~ῑσαις|~εισειᾰς|~εισαις|~ῐνησειᾰς|~ῐνησαις", "~ῑσαιτον|~εισαιτον|~ῐνησαιτον", "~ῑσαιτε|~εισαιτε|~ῐνησαιτε";
    aorist_opt_act_3: "~ῑσειε|~ῑσειεν|~ῑσαι|~εισειε|~εισειεν|~εισαι|~ῐνησειε|~ῐνησειεν|~ῐνησαι", "~ῑσαιτην|~εισαιτην|~ῐνησαιτην", "~ῑσειᾰν|~ῑσαιεν|~εισειᾰν|~εισαιεν|~ῐνησειᾰν|~ῐνησαιεν";
    aorist_opt_mid_1: "~ῑσαιμην|~εισαιμην|~ῐνησαιμην", _, "~ῑσαιμεθᾰ|~εισαιμεθᾰ|~ῐνησαιμεθᾰ";
    aorist_opt_mid_2: "~ῑσαιο|~εισαιο|~ῐνησαιο", "~ῑσαισθον|~εισαισθον|~ῐνησαισθον", "~ῑσαισθε|~εισαισθε|~ῐνησαισθε";
    aorist_opt_mid_3: "~ῑσαιτο|~εισαιτο|~ῐνησαιτο", "~ῑσαισθην|~εισαισθην|~ῐνησαισθην", "~ῑσαιντο|~εισαιντο|~ῐνησαιντο";
    aorist_opt_pass_1: "~ῐθειην", _, "~ῐθεῑμεν|~ῐθειημεν";
    aorist_opt_pass_2: "~ῐθειης", "~ῐθεῑτον|~ῐθειητον", "~ῐθεῑτε|~ῐθειητε";
    aorist_opt_pass_3: "~ῐθειη", "~ῐθειτην|~ῐθειητην", "~ῐθεῑεν|~ῐθειησᾰν";
    aorist_imp_act_2: "~ῑσον|~εῑσον|~ῐνησον", "~ῑσᾰτον|~εισᾰτον|~ῐνησᾰτον", "~ῑσᾰτε|~εισᾰτε|~ῐνησᾰτε";
    aorist_imp_act_3: "~ῑσᾰτω|~εισᾰτω|~ῐνησᾰτω", "~ῑσᾰτων|~εισᾰτων|~ῐνησᾰτων", "~ῑσᾰντων|~εισᾰντων|~ῐνησᾰντων";
    aorist_imp_mid_2: "~ῑσαι|~εῑσαι|~ῐνησαι", "~ῑσᾰσθον|~εισᾰσθον|~ῐνησᾰσθον", "~ῑσᾰσθε|~εισᾰσθε|~ῐνησᾰσθε";
    aorist_imp_mid_3: "~ῑσᾰσθω|~εισᾰσθω|~ῐνησᾰσθω", "~ῑσᾰσθων|~εισᾰσθων|~ῐνησᾰσθων", _;
    aorist_imp_pass_2: "~ῐθητῐ", _, _;
    aorist_imp_pass_3: "~ῐθητω", "~ῐθητων", "~ῐθεντων";
    perfect_ind_act_1: "ἐ~ῐνηκᾰ", _, "ἐ~ῐνηκᾰμεν";
    perfect_ind_act_2: "ἐ~ῐνηκᾰς", "ἐ~ῐνηκᾰτον", "ἐ~ῐνηκᾰτε";
    perfect_ind_act_3: "ἐ~ῐνηκε|ἐ~ῐνηκεν", _, "ἐ~ῐνηκᾱσῐ|ἐ~ῐνηκᾱσῐν";
    perfect_subj_act_1: "ἐ~ῐνηκω", _, "ἐ~ῐνηκωμεν";
    perfect_subj_act_2: "ἐ~ῐνηκῃς", "ἐ~ῐνηκητον", "ἐ~ῐνηκητε";
    perfect_subj_act_3: "ἐ~ῐνηκῃ", _, "ἐ~ῐνηκωσῐ|ἐ~ῐνηκωσῐν";
    perfect_opt_act_1: "ἐ~ῐνηκοιμῐ|ἐ~ῐνηκοιην", _, "ἐ~ῐνηκοιμεν";
    perfect_opt_act_2: "ἐ~ῐνηκοις|ἐ~ῐνηκοιης", "ἐ~ῐνηκοιτον", "ἐ~ῐνηκοιτε";
    perfect_opt_act_3: "ἐ~ῐνηκοι|ἐ~ῐνηκοιη", "ἐ~ῐνηκοιτην", "ἐ~ῐνηκοιεν";
    perfect_imp_act_2: "ἐ~ῐνηκε", "ἐ~ῐνηκετον", "ἐ~ῐνηκετε";
    perfect_imp_act_3: "ἐ~ῐνηκετω", "ἐ~ῐνηκετων", "ἐ~ῐνηκοντων";
    infinitive_pres: "~ῐνειν", "~ῐνεσθαι", _;
    infinitive_fut: "~ῑσειν|~εισειν|~ῐνησειν", "~ῑσεσθαι|~εισεσθαι|~ῐνησεσθαι", _;
    infinitive_aor: "~ῑσαι|~εῑσαι|~ῐνησαι", "~ῑσᾰσθαι|~εισᾰσθαι|~ῐνησᾰσθαι", "~ῐθηναι";
    infinitive_perf: "ἐ~ῐνηκεναι", _, _;
    participle_pres: "~ῐνων", "~ῐνομενος", _;
    participle_fut: "~ῑσων|~εισων|~ῐνησων", "~ῑσομενος|~εισομενος|~ῐνησομενος", _;
    participle_aor: "~ῑσᾱς|~εισᾱς|~ῐνησᾱς", "~ῑσᾰμενος|~εισᾰμενος|~ῐνησᾰμενος", "~ῐθεις";
    participle_perf: "ἐ~ῐνηκως", _, _;
    ),
    // Οικειοομαι: 1 words (οικειοομαι). Like Ἐρᾰμαι, except in 37 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("οικειοομαι", 1, "οι~οομαι", like Ἐρᾰμαι;
    ind_act_1: "οι~οομαι", _, _;
    ind_mid_1: "οἰ~οῡμαι", _, "οἰ~ουμεθᾰ";
    ind_mid_2: "οἰ~οῑ", "οἰ~οῡσθον", "οἰ~οῡσθε";
    ind_mid_3: "οἰ~οῡται", _, "οἰ~οῡνται";
    subj_mid_1: "οἰ~ωμαι", _, "οἰ~ωμεθᾰ";
    subj_mid_2: "οἰ~οῑ", "οἰ~ωσθον", "οἰ~ωσθε";
    subj_mid_3: "οἰ~ωται", _, "οἰ~ωνται";
    subj_pass_2: "οἰ~οῑ", _, _;
    opt_mid_1: "οἰ~οιμην", _, "οἰ~οιμεθᾰ";
    opt_mid_2: "οἰ~οῑο", "οἰ~οῑσθον", "οἰ~οῑσθε";
    opt_mid_3: "οἰ~οῑτο", "οἰ~οισθην", "οἰ~οῑντο";
    imp_mid_2: "οἰ~οῡ", _, _;
    imp_mid_3: "οἰ~ουσθω", "οἰ~ουσθων", _;
    imperfect_mid_1: "ᾠ~ουμην", _, "ᾠ~ουμεθᾰ";
    imperfect_mid_2: "ᾠ~οῡ", "ᾠ~οῡσθον", "ᾠ~οῡσθε";
    imperfect_mid_3: "ᾠ~οῡτο", "ᾠ~ουσθην", "ᾠ~οῡντο";
    infinitive_pres: _, "οἰ~οῡσθαι", _;
    participle_pres: _, "οἰ~ουμενος", _;
    ),
    // Συγκᾰκουργεω: 1 words (συγκᾰκουργεω). Like Ξυμπολεμεω, except in 228 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("συγκᾰκουργεω", 1, "~υγ~εω", like Ξυμπολεμεω;
    stem pluperfect_act: "~ῠγ%ε~ηκ- like δῐκεω";
    ind_act_1: "~ῠγ~ω", _, "~ῠγ~οῡμεν";
    ind_act_2: "~ῠγ~εῑς", "~ῠγ~εῑτον", "~ῠγ~εῑτε";
    ind_act_3: "~ῠγ~εῑ", _, "~ῠγ~οῡσῐ|~ῠγ~οῡσῐν";
    ind_mid_1: "~ῠγ~οῡμαι", _, "~ῠγ~ουμεθᾰ";
    ind_mid_2: "~ῠγ~εῑ|~ῠγ~ῃ", "~ῠγ~εῑσθον", "~ῠγ~εῑσθε";
    ind_mid_3: "~ῠγ~εῑται", _, "~ῠγ~οῡνται";
    subj_act_1: "~ῠγ~ω", _, "~ῠγ~ωμεν";
    subj_act_2: "~ῠγ~ῃς", "~ῠγ~ητον", "~ῠγ~ητε";
    subj_act_3: "~ῠγ~ῃ", _, "~ῠγ~ωσῐ|~ῠγ~ωσῐν";
    subj_mid_1: "~ῠγ~ωμαι", _, "~ῠγ~ωμεθᾰ";
    subj_mid_2: _, "~ῠγ~ησθον", "~ῠγ~ησθε";
    subj_mid_3: "~ῠγ~ηται", _, "~ῠγ~ωνται";
    opt_act_1: "~ῠγ~οιην|~ῠγ~οῑμῐ", _, "~ῠγ~οῑμεν|~ῠγ~οιημεν";
    opt_act_2: "~ῠγ~οιης|~ῠγ~οῑς", "~ῠγ~οῑτον|~ῠγ~οιητον", "~ῠγ~οῑτε|~ῠγ~οιητε";
    opt_act_3: "~ῠγ~οιη|~ῠγ~οῑ", "~ῠγ~οιτην|~ῠγ~οιητην", "~ῠγ~οῑεν|~ῠγ~οιησᾰν";
    opt_mid_1: "~ῠγ~οιμην", _, "~ῠγ~οιμεθᾰ";
    opt_mid_2: "~ῠγ~οῑο", "~ῠγ~οῑσθον", "~ῠγ~οῑσθε";
    opt_mid_3: "~ῠγ~οῑτο", "~ῠγ~οισθην", "~ῠγ~οῑντο";
    imp_act_2: "~ῠγ~ει", _, _;
    imp_act_3: "~ῠγ~ειτω", "~ῠγ~ειτων", "~ῠγ~ουντων";
    imp_mid_2: "~ῠγ~οῡ", _, _;
    imp_mid_3: "~ῠγ~εισθω", "~ῠγ~εισθων", _;
    imperfect_mid_1: "~ῠνε~ουμην", _, "~ῠνε~ουμεθᾰ";
    imperfect_mid_2: "~ῠνε~οῡ", "~ῠνε~εῑσθον", "~ῠνε~εῑσθε";
    imperfect_mid_3: "~ῠνε~εῑτο", "~ῠνε~εισθην", "~ῠνε~οῡντο";
    future_ind_act_1: "~ῠγ~ησω", _, "~ῠγ~ησομεν";
    future_ind_act_2: "~ῠγ~ησεις", "~ῠγ~ησετον", "~ῠγ~ησετε";
    future_ind_act_3: "~ῠγ~ησει", _, "~ῠγ~ησουσῐ|~ῠγ~ησουσῐν";
    future_ind_mid_1: "~ῠγ~ησομαι", _, "~ῠγ~ησομεθᾰ";
    future_ind_mid_2: "~ῠγ~ησῃ|~ῠγ~ησει", "~ῠγ~ησεσθον", "~ῠγ~ησεσθε";
    future_ind_mid_3: "~ῠγ~ησεται", _, "~ῠγ~ησονται";
    future_ind_pass_1: "~ῠγ~ηθησομαι", _, "~ῠγ~ηθησομεθᾰ";
    future_ind_pass_2: "~ῠγ~ηθησῃ|~ῠγ~ηθησει", "~ῠγ~ηθησεσθον", "~ῠγ~ηθησεσθε";
    future_ind_pass_3: "~ῠγ~ηθησεται", _, "~ῠγ~ηθησονται";
    future_opt_act_1: "~ῠγ~ησοιμῐ", _, "~ῠγ~ησοιμεν";
    future_opt_act_2: "~ῠγ~ησοις", "~ῠγ~ησοιτον", "~ῠγ~ησοιτε";
    future_opt_act_3: "~ῠγ~ησοι", "~ῠγ~ησοιτην", "~ῠγ~ησοιεν";
    future_opt_mid_1: "~ῠγ~ησοιμην", _, "~ῠγ~ησοιμεθᾰ";
    future_opt_mid_2: "~ῠγ~ησοιο", "~ῠγ~ησοισθον", "~ῠγ~ησοισθε";
    future_opt_mid_3: "~ῠγ~ησοιτο", "~ῠγ~ησοισθην", "~ῠγ~ησοιντο";
    future_opt_pass_1: "~ῠγ~ηθησοιμην", _, "~ῠγ~ηθησοιμεθᾰ";
    future_opt_pass_2: "~ῠγ~ηθησοιο", "~ῠγ~ηθησοισθον", "~ῠγ~ηθησοισθε";
    future_opt_pass_3: "~ῠγ~ηθησοιτο", "~ῠγ~ηθησοισθην", "~ῠγ~ηθησοιντο";
    aorist_subj_act_1: "~ῠγ~ησω", _, "~ῠγ~ησωμεν";
    aorist_subj_act_2: "~ῠγ~ησῃς", "~ῠγ~ησητον", "~ῠγ~ησητε";
    aorist_subj_act_3: "~ῠγ~ησῃ", _, "~ῠγ~ησωσῐ|~ῠγ~ησωσῐν";
    aorist_subj_mid_1: "~ῠγ~ησωμαι", _, "~ῠγ~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~ῠγ~ησησθον", "~ῠγ~ησησθε";
    aorist_subj_mid_3: "~ῠγ~ησηται", _, "~ῠγ~ησωνται";
    aorist_subj_pass_1: "~ῠγ~ηθω", _, "~ῠγ~ηθωμεν";
    aorist_subj_pass_2: "~ῠγ~ηθῃς", "~ῠγ~ηθητον", "~ῠγ~ηθητε";
    aorist_subj_pass_3: "~ῠγ~ηθῃ", _, "~ῠγ~ηθωσῐ|~ῠγ~ηθωσῐν";
    aorist_opt_act_1: "~ῠγ~ησαιμῐ", _, "~ῠγ~ησαιμεν";
    aorist_opt_act_2: "~ῠγ~ησειᾰς|~ῠγ~ησαις", "~ῠγ~ησαιτον", "~ῠγ~ησαιτε";
    aorist_opt_act_3: "~ῠγ~ησειε|~ῠγ~ησειεν|~ῠγ~ησαι", "~ῠγ~ησαιτην", "~ῠγ~ησειᾰν|~ῠγ~ησαιεν";
    aorist_opt_mid_1: "~ῠγ~ησαιμην", _, "~ῠγ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ῠγ~ησαιο", "~ῠγ~ησαισθον", "~ῠγ~ησαισθε";
    aorist_opt_mid_3: "~ῠγ~ησαιτο", "~ῠγ~ησαισθην", "~ῠγ~ησαιντο";
    aorist_opt_pass_1: "~ῠγ~ηθειην", _, "~ῠγ~ηθεῑμεν|~ῠγ~ηθειημεν";
    aorist_opt_pass_2: "~ῠγ~ηθειης", "~ῠγ~ηθεῑτον|~ῠγ~ηθειητον", "~ῠγ~ηθεῑτε|~ῠγ~ηθειητε";
    aorist_opt_pass_3: "~ῠγ~ηθειη", "~ῠγ~ηθειτην|~ῠγ~ηθειητην", "~ῠγ~ηθεῑεν|~ῠγ~ηθειησᾰν";
    aorist_imp_act_2: "~ῠγ~ησον", "~ῠγ~ησᾰτον", "~ῠγ~ησᾰτε";
    aorist_imp_act_3: "~ῠγ~ησᾰτω", "~ῠγ~ησᾰτων", "~ῠγ~ησᾰντων";
    aorist_imp_mid_2: "~ῠγ~ησαι", "~ῠγ~ησᾰσθον", "~ῠγ~ησᾰσθε";
    aorist_imp_mid_3: "~ῠγ~ησᾰσθω", "~ῠγ~ησᾰσθων", _;
    aorist_imp_pass_2: "~ῠγ~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ῠγ~ηθητω", "~ῠγ~ηθητων", "~ῠγ~ηθεντων";
    perfect_ind_act_1: "~ῠγ%ε~ηκᾰ", _, "~ῠγ%ε~ηκᾰμεν";
    perfect_ind_act_2: "~ῠγ%ε~ηκᾰς", "~ῠγ%ε~ηκᾰτον", "~ῠγ%ε~ηκᾰτε";
    perfect_ind_act_3: "~ῠγ%ε~ηκε|~ῠγ%ε~ηκεν", _, "~ῠγ%ε~ηκᾱσῐ|~ῠγ%ε~ηκᾱσῐν";
    perfect_ind_mid_1: "~ῠγ%ε~ημαι", _, "~ῠγ%ε~ημεθᾰ";
    perfect_ind_mid_2: "~ῠγ%ε~ησαι", "~ῠγ%ε~ησθον", "~ῠγ%ε~ησθε";
    perfect_ind_mid_3: "~ῠγ%ε~ηται", _, "~ῠγ%ε~ηνται";
    perfect_subj_act_1: "~ῠγ%ε~ηκω", _, "~ῠγ%ε~ηκωμεν";
    perfect_subj_act_2: "~ῠγ%ε~ηκῃς", "~ῠγ%ε~ηκητον", "~ῠγ%ε~ηκητε";
    perfect_subj_act_3: "~ῠγ%ε~ηκῃ", _, "~ῠγ%ε~ηκωσῐ|~ῠγ%ε~ηκωσῐν";
    perfect_opt_act_1: "~ῠγ%ε~ηκοιμῐ|~ῠγ%ε~ηκοιην", _, "~ῠγ%ε~ηκοιμεν";
    perfect_opt_act_2: "~ῠγ%ε~ηκοις|~ῠγ%ε~ηκοιης", "~ῠγ%ε~ηκοιτον", "~ῠγ%ε~ηκοιτε";
    perfect_opt_act_3: "~ῠγ%ε~ηκοι|~ῠγ%ε~ηκοιη", "~ῠγ%ε~ηκοιτην", "~ῠγ%ε~ηκοιεν";
    perfect_imp_act_2: "~ῠγ%ε~ηκε", "~ῠγ%ε~ηκετον", "~ῠγ%ε~ηκετε";
    perfect_imp_act_3: "~ῠγ%ε~ηκετω", "~ῠγ%ε~ηκετων", "~ῠγ%ε~ηκοντων";
    perfect_imp_mid_2: "~ῠγ%ε~ησο", _, _;
    perfect_imp_mid_3: "~ῠγ%ε~ησθω", "~ῠγ%ε~ησθων", _;
    pluperfect_mid_1: "ἐ~ῠγ%ε~ημην", _, "ἐ~ῠγ%ε~ημεθᾰ";
    pluperfect_mid_2: "ἐ~ῠγ%ε~ησο", "ἐ~ῠγ%ε~ησθον", "ἐ~ῠγ%ε~ησθε";
    pluperfect_mid_3: "ἐ~ῠγ%ε~ητο", "ἐ~ῠγ%ε~ησθην", "ἐ~ῠγ%ε~ηντο";
    infinitive_pres: "~ῠγ~εῑν", "~ῠγ~εῑσθαι", _;
    infinitive_fut: "~ῠγ~ησειν", "~ῠγ~ησεσθαι", "~ῠγ~ηθησεσθαι";
    infinitive_aor: "~ῠγ~ησαι", "~ῠγ~ησᾰσθαι", "~ῠγ~ηθηναι";
    infinitive_perf: "~ῠγ%ε~ηκεναι", "~ῠγ%ε~ησθαι", _;
    participle_pres: "~ῠγ~ων", "~ῠγ~ουμενος", _;
    participle_fut: "~ῠγ~ησων", "~ῠγ~ησομενος", "~ῠγ~ηθησομενος";
    participle_aor: "~ῠγ~ησᾱς", "~ῠγ~ησᾰμενος", "~ῠγ~ηθεις";
    participle_perf: "~ῠγ%ε~ηκως", "~ῠγ%ε~ημενος", _;
    ),
    // Ἀποστατεω: 1 words (ἀποστατεω). Like Ξυνναυμᾰχεω, except in 28 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀποστατεω", 1, "ἀ~ο~α~εω", like Ξυνναυμᾰχεω as ACT_AS_PASS;
    stem imperfect: "~ε~ᾰ~- like ἀποστατεω";
    ind_act_1: "ᾰ~ο~ᾰ~ω", _, "ᾰ~ο~ᾰ~οῡμεν";
    ind_act_2: "ᾰ~ο~ᾰ~εῑς", "ᾰ~ο~ᾰ~εῑτον", "ᾰ~ο~ᾰ~εῑτε";
    ind_act_3: "ᾰ~ο~ᾰ~εῑ", _, "ᾰ~ο~ᾰ~οῡσῐ|ᾰ~ο~ᾰ~οῡσῐν";
    subj_act_1: "ᾰ~ο~ᾰ~ω", _, "ᾰ~ο~ᾰ~ωμεν";
    subj_act_2: "ᾰ~ο~ᾰ~ῃς", "ᾰ~ο~ᾰ~ητον", "ᾰ~ο~ᾰ~ητε";
    subj_act_3: "ᾰ~ο~ᾰ~ῃ", _, "ᾰ~ο~ᾰ~ωσῐ|ᾰ~ο~ᾰ~ωσῐν";
    opt_act_1: "ᾰ~ο~ᾰ~οιην|ᾰ~ο~ᾰ~οῑμῐ", _, "ᾰ~ο~ᾰ~οῑμεν|ᾰ~ο~ᾰ~οιημεν";
    opt_act_2: "ᾰ~ο~ᾰ~οιης|ᾰ~ο~ᾰ~οῑς", "ᾰ~ο~ᾰ~οῑτον|ᾰ~ο~ᾰ~οιητον", "ᾰ~ο~ᾰ~οῑτε|ᾰ~ο~ᾰ~οιητε";
    opt_act_3: "ᾰ~ο~ᾰ~οιη|ᾰ~ο~ᾰ~οῑ", "ᾰ~ο~ᾰ~οιτην|ᾰ~ο~ᾰ~οιητην", "ᾰ~ο~ᾰ~οῑεν|ᾰ~ο~ᾰ~οιησᾰν";
    imp_act_2: "ᾰ~ο~ᾰ~ει", _, _;
    imp_act_3: "ᾰ~ο~ᾰ~ειτω", "ᾰ~ο~ᾰ~ειτων", "ᾰ~ο~ᾰ~ουντων";
    infinitive_pres: "ᾰ~ο~ᾰ~εῑν", _, _;
    participle_pres: "ᾰ~ο~ᾰ~ων", _, _;
    ),
    // Γεγενησαι: 1 words (γεγενησαι). Like Ἐγκειμαι, except in 43 cells: ind_act_1 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 sg, perfect_ind_act_2 du, perfect_ind_act_2 pl, ….
    conjugation!("γεγενησαι", 1, "~ε~ησαι", like Ἐγκειμαι as MID_AS_PASS;
    ind_act_1: "~ε~ησαι", _, _;
    perfect_ind_act_1: "~ο~ᾰ", _, "~ο~ᾰμεν";
    perfect_ind_act_2: "~ο~ᾰς", "~ο~ᾰτον", "~ο~ᾰτε";
    perfect_ind_act_3: "~ο~ε|~ο~εν", _, "~ο~ᾱσῐ|~ο~ᾱσῐν";
    perfect_ind_mid_1: "~ε~ημαι", _, "~ε~ημεθᾰ";
    perfect_ind_mid_2: "~ε~ησαι", "~ε~ησθον", "~ε~ησθε";
    perfect_ind_mid_3: "~ε~ηται", _, "~ε~ηνται";
    perfect_subj_act_1: "~ο~ω", _, "~ο~ωμεν";
    perfect_subj_act_2: "~ο~ῃς", "~ο~ητον", "~ο~ητε";
    perfect_subj_act_3: "~ο~ῃ", _, "~ο~ωσῐ|~ο~ωσῐν";
    perfect_opt_act_1: "~ο~οιμῐ|~ο~οιην", _, "~ο~οιμεν";
    perfect_opt_act_2: "~ο~οις|~ο~οιης", "~ο~οιτον", "~ο~οιτε";
    perfect_opt_act_3: "~ο~οι|~ο~οιη", "~ο~οιτην", "~ο~οιεν";
    perfect_imp_act_2: "~ο~ε", "~ο~ετον", "~ο~ετε";
    perfect_imp_act_3: "~ο~ετω", "~ο~ετων", "~ο~οντων";
    perfect_imp_mid_2: "~ε~ησο", _, _;
    perfect_imp_mid_3: "~ε~ησθω", "~ε~ησθων", _;
    infinitive_perf: "~ο~εναι", "~ε~ησθαι", _;
    participle_perf: "~ο~ως", "~ε~ημενος", _;
    ),
    // Γεγενηται: 1 words (γεγενηται). Like Γεγενησαι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενηται", 1, "~ε~ηται", like Γεγενησαι;
    ind_act_1: "~ε~ηται", _, _;
    ),
    // Γεγενησθω: 1 words (γεγενησθω). Like Γεγενησαι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησθω", 1, "~ε~ησθω", like Γεγενησαι;
    ind_act_1: "~ε~ησθω", _, _;
    ),
    // Γεγενησο: 1 words (γεγενησο). Like Γεγονοι, except in 43 cells: ind_act_1 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 sg, perfect_ind_act_2 du, perfect_ind_act_2 pl, ….
    conjugation!("γεγενησο", 1, "~ενησο", like Γεγονοι;
    ind_act_1: "~ενησο", _, _;
    perfect_ind_act_1: "~ονᾰ", _, "~ονᾰμεν";
    perfect_ind_act_2: "~ονᾰς", "~ονᾰτον", "~ονᾰτε";
    perfect_ind_act_3: "~ονε|~ονεν", _, "~ονᾱσῐ|~ονᾱσῐν";
    perfect_ind_mid_1: "~ενημαι", _, "~ενημεθᾰ";
    perfect_ind_mid_2: "~ενησαι", "~ενησθον", "~ενησθε";
    perfect_ind_mid_3: "~ενηται", _, "~ενηνται";
    perfect_subj_act_1: "~ονω", _, "~ονωμεν";
    perfect_subj_act_2: "~ονῃς", "~ονητον", "~ονητε";
    perfect_subj_act_3: "~ονῃ", _, "~ονωσῐ|~ονωσῐν";
    perfect_opt_act_1: "~ονοιμῐ|~ονοιην", _, "~ονοιμεν";
    perfect_opt_act_2: "~ονοις|~ονοιης", "~ονοιτον", "~ονοιτε";
    perfect_opt_act_3: "~ονοι|~ονοιη", "~ονοιτην", "~ονοιεν";
    perfect_imp_act_2: "~ονε", "~ονετον", "~ονετε";
    perfect_imp_act_3: "~ονετω", "~ονετων", "~ονοντων";
    perfect_imp_mid_2: "~ενησο", _, _;
    perfect_imp_mid_3: "~ενησθω", "~ενησθων", _;
    infinitive_perf: "~ονεναι", "~ενησθαι", _;
    participle_perf: "~ονως", "~ενημενος", _;
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
    // Τεθνηκας: 1 words (τεθνηκας). Like Γεγονοι, except in 8 cells: ind_act_1 sg, perfect_ind_act_3 sg, perfect_ind_act_3 pl, perfect_subj_act_3 pl, perfect_opt_act_1 sg, perfect_opt_act_2 sg, ….
    conjugation!("τεθνηκας", 1, "~ας", like Γεγονοι;
    ind_act_1: "~ας", _, _;
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, _;
    perfect_opt_act_2: "~οις|~οιης", _, _;
    perfect_opt_act_3: "~οι|~οιη", _, _;
    perfect_imp_act_2: "~ε", _, _;
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
    stem imperfect: "~ῐ~ᾰ~ε~ᾰσσ- like ὑποκοριζομαι";
    stem aorist_ind: "~ῐ~ᾰ~ε~ᾰξ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ῐ~ᾰ~ε~ᾰχθη- like ἐλπω";
    ind_act_1: "~ι~α~α~ασσομαι", _, _;
    ind_mid_1: "~ῐ~ᾰ~ᾰ~ᾰσσομαι", _, "~ῐ~ᾰ~ᾰ~ᾰσσομεθᾰ";
    ind_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσῃ|~ῐ~ᾰ~ᾰ~ᾰσσει", "~ῐ~ᾰ~ᾰ~ᾰσσεσθον", "~ῐ~ᾰ~ᾰ~ᾰσσεσθε";
    ind_mid_3: "~ῐ~ᾰ~ᾰ~ᾰσσεται", _, "~ῐ~ᾰ~ᾰ~ᾰσσονται";
    subj_mid_1: "~ῐ~ᾰ~ᾰ~ᾰσσωμαι", _, "~ῐ~ᾰ~ᾰ~ᾰσσωμεθᾰ";
    subj_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσῃ", "~ῐ~ᾰ~ᾰ~ᾰσσησθον", "~ῐ~ᾰ~ᾰ~ᾰσσησθε";
    subj_mid_3: "~ῐ~ᾰ~ᾰ~ᾰσσηται", _, "~ῐ~ᾰ~ᾰ~ᾰσσωνται";
    subj_pass_2: "~ῐ~ᾰ~ᾰ~ᾰσσῃ", _, _;
    opt_mid_1: "~ῐ~ᾰ~ᾰ~ᾰσσοιμην", _, "~ῐ~ᾰ~ᾰ~ᾰσσοιμεθᾰ";
    opt_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσοιο", "~ῐ~ᾰ~ᾰ~ᾰσσοισθον", "~ῐ~ᾰ~ᾰ~ᾰσσοισθε";
    opt_mid_3: "~ῐ~ᾰ~ᾰ~ᾰσσοιτο", "~ῐ~ᾰ~ᾰ~ᾰσσοισθην", "~ῐ~ᾰ~ᾰ~ᾰσσοιντο";
    imp_mid_2: "~ῐ~ᾰ~ᾰ~ᾰσσου", _, _;
    imp_mid_3: "~ῐ~ᾰ~ᾰ~ᾰσσεσθω", "~ῐ~ᾰ~ᾰ~ᾰσσεσθων", _;
    future_ind_mid_1: "~ῐ~ᾰ~ᾰ~ᾰξομαι", _, "~ῐ~ᾰ~ᾰ~ᾰξομεθᾰ";
    future_ind_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξῃ|~ῐ~ᾰ~ᾰ~ᾰξει", "~ῐ~ᾰ~ᾰ~ᾰξεσθον", "~ῐ~ᾰ~ᾰ~ᾰξεσθε";
    future_ind_mid_3: "~ῐ~ᾰ~ᾰ~ᾰξεται", _, "~ῐ~ᾰ~ᾰ~ᾰξονται";
    future_ind_pass_1: "~ῐ~ᾰ~ᾰ~ᾰχθησομαι", _, "~ῐ~ᾰ~ᾰ~ᾰχθησομεθᾰ";
    future_ind_pass_2: "~ῐ~ᾰ~ᾰ~ᾰχθησῃ|~ῐ~ᾰ~ᾰ~ᾰχθησει", "~ῐ~ᾰ~ᾰ~ᾰχθησεσθον", "~ῐ~ᾰ~ᾰ~ᾰχθησεσθε";
    future_ind_pass_3: "~ῐ~ᾰ~ᾰ~ᾰχθησεται", _, "~ῐ~ᾰ~ᾰ~ᾰχθησονται";
    future_opt_mid_1: "~ῐ~ᾰ~ᾰ~ᾰξοιμην", _, "~ῐ~ᾰ~ᾰ~ᾰξοιμεθᾰ";
    future_opt_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξοιο", "~ῐ~ᾰ~ᾰ~ᾰξοισθον", "~ῐ~ᾰ~ᾰ~ᾰξοισθε";
    future_opt_mid_3: "~ῐ~ᾰ~ᾰ~ᾰξοιτο", "~ῐ~ᾰ~ᾰ~ᾰξοισθην", "~ῐ~ᾰ~ᾰ~ᾰξοιντο";
    future_opt_pass_1: "~ῐ~ᾰ~ᾰ~ᾰχθησοιμην", _, "~ῐ~ᾰ~ᾰ~ᾰχθησοιμεθᾰ";
    future_opt_pass_2: "~ῐ~ᾰ~ᾰ~ᾰχθησοιο", "~ῐ~ᾰ~ᾰ~ᾰχθησοισθον", "~ῐ~ᾰ~ᾰ~ᾰχθησοισθε";
    future_opt_pass_3: "~ῐ~ᾰ~ᾰ~ᾰχθησοιτο", "~ῐ~ᾰ~ᾰ~ᾰχθησοισθην", "~ῐ~ᾰ~ᾰ~ᾰχθησοιντο";
    aorist_subj_mid_1: "~ῐ~ᾰ~ᾰ~ᾰξωμαι", _, "~ῐ~ᾰ~ᾰ~ᾰξωμεθᾰ";
    aorist_subj_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξῃ", "~ῐ~ᾰ~ᾰ~ᾰξησθον", "~ῐ~ᾰ~ᾰ~ᾰξησθε";
    aorist_subj_mid_3: "~ῐ~ᾰ~ᾰ~ᾰξηται", _, "~ῐ~ᾰ~ᾰ~ᾰξωνται";
    aorist_subj_pass_1: "~ῐ~ᾰ~ᾰ~ᾰχθω", _, "~ῐ~ᾰ~ᾰ~ᾰχθωμεν";
    aorist_subj_pass_2: "~ῐ~ᾰ~ᾰ~ᾰχθῃς", "~ῐ~ᾰ~ᾰ~ᾰχθητον", "~ῐ~ᾰ~ᾰ~ᾰχθητε";
    aorist_subj_pass_3: "~ῐ~ᾰ~ᾰ~ᾰχθῃ", _, "~ῐ~ᾰ~ᾰ~ᾰχθωσῐ|~ῐ~ᾰ~ᾰ~ᾰχθωσῐν";
    aorist_opt_mid_1: "~ῐ~ᾰ~ᾰ~ᾰξαιμην", _, "~ῐ~ᾰ~ᾰ~ᾰξαιμεθᾰ";
    aorist_opt_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξαιο", "~ῐ~ᾰ~ᾰ~ᾰξαισθον", "~ῐ~ᾰ~ᾰ~ᾰξαισθε";
    aorist_opt_mid_3: "~ῐ~ᾰ~ᾰ~ᾰξαιτο", "~ῐ~ᾰ~ᾰ~ᾰξαισθην", "~ῐ~ᾰ~ᾰ~ᾰξαιντο";
    aorist_opt_pass_1: "~ῐ~ᾰ~ᾰ~ᾰχθειην", _, "~ῐ~ᾰ~ᾰ~ᾰχθεῑμεν|~ῐ~ᾰ~ᾰ~ᾰχθειημεν";
    aorist_opt_pass_2: "~ῐ~ᾰ~ᾰ~ᾰχθειης", "~ῐ~ᾰ~ᾰ~ᾰχθεῑτον|~ῐ~ᾰ~ᾰ~ᾰχθειητον", "~ῐ~ᾰ~ᾰ~ᾰχθεῑτε|~ῐ~ᾰ~ᾰ~ᾰχθειητε";
    aorist_opt_pass_3: "~ῐ~ᾰ~ᾰ~ᾰχθειη", "~ῐ~ᾰ~ᾰ~ᾰχθειτην|~ῐ~ᾰ~ᾰ~ᾰχθειητην", "~ῐ~ᾰ~ᾰ~ᾰχθεῑεν|~ῐ~ᾰ~ᾰ~ᾰχθειησᾰν";
    aorist_imp_mid_2: "~ῐ~ᾰ~ᾰ~ᾰξαι", "~ῐ~ᾰ~ᾰ~ᾰξᾰσθον", "~ῐ~ᾰ~ᾰ~ᾰξᾰσθε";
    aorist_imp_mid_3: "~ῐ~ᾰ~ᾰ~ᾰξᾰσθω", "~ῐ~ᾰ~ᾰ~ᾰξᾰσθων", _;
    aorist_imp_pass_2: "~ῐ~ᾰ~ᾰ~ᾰχθητῐ", _, _;
    aorist_imp_pass_3: "~ῐ~ᾰ~ᾰ~ᾰχθητω", "~ῐ~ᾰ~ᾰ~ᾰχθητων", "~ῐ~ᾰ~ᾰ~ᾰχθεντων";
    perfect_ind_mid_1: "~ῐ~ᾰ~ᾰ~ετᾰγμαι", _, "~ῐ~ᾰ~ᾰ~ετᾰγμεθᾰ";
    perfect_ind_mid_2: "~ῐ~ᾰ~ᾰ~ετᾰξαι", "~ῐ~ᾰ~ᾰ~ετᾰχθον", "~ῐ~ᾰ~ᾰ~ετᾰχθε";
    perfect_ind_mid_3: "~ῐ~ᾰ~ᾰ~ετᾰκται", _, "~ῐ~ᾰ~ᾰ~ετᾰγᾰται";
    perfect_imp_mid_2: "~ῐ~ᾰ~ᾰ~ετᾰξο", _, _;
    perfect_imp_mid_3: "~ῐ~ᾰ~ᾰ~ετᾰχθω", "~ῐ~ᾰ~ᾰ~ετᾰχθων", _;
    pluperfect_mid_1: "~ῐ~ᾰ~ε~ετᾰγμην", _, "~ῐ~ᾰ~ε~ετᾰγμεθᾰ";
    pluperfect_mid_2: "~ῐ~ᾰ~ε~ετᾰξο", "~ῐ~ᾰ~ε~ετᾰχθον", "~ῐ~ᾰ~ε~ετᾰχθε";
    pluperfect_mid_3: "~ῐ~ᾰ~ε~ετᾰκτο", "~ῐ~ᾰ~ε~ετᾰχθην", "~ῐ~ᾰ~ε~ετᾰγᾰτο";
    infinitive_pres: _, "~ῐ~ᾰ~ᾰ~ᾰσσεσθαι", _;
    infinitive_fut: _, "~ῐ~ᾰ~ᾰ~ᾰξεσθαι", "~ῐ~ᾰ~ᾰ~ᾰχθησεσθαι";
    infinitive_aor: _, "~ῐ~ᾰ~ᾰ~ᾰξᾰσθαι", "~ῐ~ᾰ~ᾰ~ᾰχθηναι";
    infinitive_perf: _, "~ῐ~ᾰ~ᾰ~ετᾰχθαι", _;
    participle_pres: _, "~ῐ~ᾰ~ᾰ~ᾰσσομενος", _;
    participle_fut: _, "~ῐ~ᾰ~ᾰ~ᾰξομενος", "~ῐ~ᾰ~ᾰ~ᾰχθησομενος";
    participle_aor: _, "~ῐ~ᾰ~ᾰ~ᾰξᾰμενος", "~ῐ~ᾰ~ᾰ~ᾰχθεις";
    participle_perf: _, "~ῐ~ᾰ~ᾰ~ετᾰγμενος", _;
    ),
    // Ἐπᾰνερχομαι: 1 words (ἐπᾰνερχομαι). Like Νισσομαι, except in 52 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἐπᾰνερχομαι", 1, "~ᾰ~ρχομαι", like Νισσομαι;
    stem aorist_ind: "~α~λθ- like εἰσερχομαι";
    ind_act_1: "~ᾰ~ρχομαι", _, _;
    ind_mid_1: "~α~ρχομαι", _, "~α~ρχομεθᾰ";
    ind_mid_2: "~α~ρχῃ|~α~ρχει", "~α~ρχεσθον", "~α~ρχεσθε";
    ind_mid_3: "~α~ρχεται", _, "~α~ρχονται";
    subj_mid_1: "~α~ρχωμαι", _, "~α~ρχωμεθᾰ";
    subj_mid_2: "~α~ρχῃ", "~α~ρχησθον", "~α~ρχησθε";
    subj_mid_3: "~α~ρχηται", _, "~α~ρχωνται";
    subj_pass_2: "~α~ρχῃ", _, _;
    opt_mid_1: "~α~ρχοιμην", _, "~α~ρχοιμεθᾰ";
    opt_mid_2: "~α~ρχοιο", "~α~ρχοισθον", "~α~ρχοισθε";
    opt_mid_3: "~α~ρχοιτο", "~α~ρχοισθην", "~α~ρχοιντο";
    imp_mid_2: "~α~ρχου", _, _;
    imp_mid_3: "~α~ρχεσθω", "~α~ρχεσθων", _;
    aorist_subj_act_1: "~α~λθω", _, "~α~λθωμεν";
    aorist_subj_act_2: "~α~λθῃς", "~α~λθητον", "~α~λθητε";
    aorist_subj_act_3: "~α~λθῃ", _, "~α~λθωσῐ|~α~λθωσῐν";
    aorist_opt_act_1: "~α~λθοιμῐ", _, "~α~λθοιμεν";
    aorist_opt_act_2: "~α~λθοις", "~α~λθοιτον", "~α~λθοιτε";
    aorist_opt_act_3: "~α~λθοι", "~α~λθοιτην", "~α~λθοιεν";
    aorist_imp_act_2: "~α~λθε", "~α~λθετον", "~α~λθετε";
    aorist_imp_act_3: "~α~λθετω", "~α~λθετων", "~α~λθοντων";
    infinitive_pres: _, "~α~ρχεσθαι", _;
    infinitive_aor: "~α~λθεῑν", _, _;
    participle_pres: _, "~α~ρχομενος", _;
    participle_aor: "~α~λθων", _, _;
    ),
    // Κιδνᾰμαι: 1 words (κιδνᾰμαι). Like Νισσομαι, except in 29 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("κιδνᾰμαι", 1, "~ι~ᾰμαι", like Νισσομαι;
    ind_act_1: "~ι~ᾰμαι", _, _;
    ind_mid_1: "~ῐ~ᾰμαι", _, "~ῐ~ᾰμεθᾰ";
    ind_mid_2: "~ῐ~ᾰσαι", "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    ind_mid_3: "~ῐ~ᾰται", _, "~ῐ~ᾰνται";
    opt_mid_1: "~ῐ~αιμην", _, "~ῐ~αιμεθᾰ";
    opt_mid_2: "~ῐ~αιο", "~ῐ~αισθον", "~ῐ~αισθε";
    opt_mid_3: "~ῐ~αιτο", "~ῐ~αισθην", "~ῐ~αιντο";
    imp_mid_2: "~ῐ~ᾰσο|~ῐ~ω", _, _;
    imp_mid_3: "~ῐ~ᾰσθω", "~ῐ~ᾰσθων", _;
    imperfect_mid_1: "ἐ~ῐ~ᾰμην", _, "ἐ~ῐ~ᾰμεθᾰ";
    imperfect_mid_2: "ἐ~ῐ~ᾰσο", "ἐ~ῐ~ᾰσθον", "ἐ~ῐ~ᾰσθε";
    imperfect_mid_3: "ἐ~ῐ~ᾰτο", "ἐ~ῐ~ᾰσθην", "ἐ~ῐ~ᾰντο";
    infinitive_pres: _, "~ῐ~ᾰσθαι", _;
    participle_pres: _, "~ῐ~ᾰμενος", _;
    ),
    // Ἐπισκεπτομαι: 1 words (ἐπισκεπτομαι). Like Δερκομαι, except in 118 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἐπισκεπτομαι", 1, "~ι~πτομαι", like Δερκομαι;
    stem imperfect: "~ε~πτ- like ὑποκοριζομαι";
    stem aorist_ind: "~ε~ψ- like μηκᾰομαι";
    stem aorist_ind_pass: "~ε~- like ἐπισκεπτομαι";
    ind_act_1: "~ι~πτομαι", _, _;
    ind_mid_1: "~ῐ~πτομαι", _, "~ῐ~πτομεθᾰ";
    ind_mid_2: "~ῐ~πτῃ|~ῐ~πτει", "~ῐ~πτεσθον", "~ῐ~πτεσθε";
    ind_mid_3: "~ῐ~πτεται", _, "~ῐ~πτονται";
    subj_mid_1: "~ῐ~πτωμαι", _, "~ῐ~πτωμεθᾰ";
    subj_mid_2: "~ῐ~πτῃ", "~ῐ~πτησθον", "~ῐ~πτησθε";
    subj_mid_3: "~ῐ~πτηται", _, "~ῐ~πτωνται";
    subj_pass_2: "~ῐ~πτῃ", _, _;
    opt_mid_1: "~ῐ~πτοιμην", _, "~ῐ~πτοιμεθᾰ";
    opt_mid_2: "~ῐ~πτοιο", "~ῐ~πτοισθον", "~ῐ~πτοισθε";
    opt_mid_3: "~ῐ~πτοιτο", "~ῐ~πτοισθην", "~ῐ~πτοιντο";
    imp_mid_2: "~ῐ~πτου", _, _;
    imp_mid_3: "~ῐ~πτεσθω", "~ῐ~πτεσθων", _;
    future_ind_mid_1: "~ῐ~ψομαι", _, "~ῐ~ψομεθᾰ";
    future_ind_mid_2: "~ῐ~ψῃ|~ῐ~ψει", "~ῐ~ψεσθον", "~ῐ~ψεσθε";
    future_ind_mid_3: "~ῐ~ψεται", _, "~ῐ~ψονται";
    future_ind_pass_1: "~ῐ~πησομαι", _, "~ῐ~πησομεθᾰ";
    future_ind_pass_2: "~ῐ~πησῃ|~ῐ~πησει", "~ῐ~πησεσθον", "~ῐ~πησεσθε";
    future_ind_pass_3: "~ῐ~πησεται", _, "~ῐ~πησονται";
    future_opt_mid_1: "~ῐ~ψοιμην", _, "~ῐ~ψοιμεθᾰ";
    future_opt_mid_2: "~ῐ~ψοιο", "~ῐ~ψοισθον", "~ῐ~ψοισθε";
    future_opt_mid_3: "~ῐ~ψοιτο", "~ῐ~ψοισθην", "~ῐ~ψοιντο";
    future_opt_pass_1: "~ῐ~πησοιμην", _, "~ῐ~πησοιμεθᾰ";
    future_opt_pass_2: "~ῐ~πησοιο", "~ῐ~πησοισθον", "~ῐ~πησοισθε";
    future_opt_pass_3: "~ῐ~πησοιτο", "~ῐ~πησοισθην", "~ῐ~πησοιντο";
    aorist_subj_mid_1: "~ῐ~ψωμαι", _, "~ῐ~ψωμεθᾰ";
    aorist_subj_mid_2: "~ῐ~ψῃ", "~ῐ~ψησθον", "~ῐ~ψησθε";
    aorist_subj_mid_3: "~ῐ~ψηται", _, "~ῐ~ψωνται";
    aorist_subj_pass_1: "~ῐ~φθω|~ῐ~πω", _, "~ῐ~φθωμεν|~ῐ~πωμεν";
    aorist_subj_pass_2: "~ῐ~φθῃς|~ῐ~πῃς", "~ῐ~φθητον|~ῐ~πητον", "~ῐ~φθητε|~ῐ~πητε";
    aorist_subj_pass_3: "~ῐ~φθῃ|~ῐ~πῃ", _, "~ῐ~φθωσῐ|~ῐ~φθωσῐν|~ῐ~πωσῐ|~ῐ~πωσῐν";
    aorist_opt_mid_1: "~ῐ~ψαιμην", _, "~ῐ~ψαιμεθᾰ";
    aorist_opt_mid_2: "~ῐ~ψαιο", "~ῐ~ψαισθον", "~ῐ~ψαισθε";
    aorist_opt_mid_3: "~ῐ~ψαιτο", "~ῐ~ψαισθην", "~ῐ~ψαιντο";
    aorist_opt_pass_1: "~ῐ~φθειην|~ῐ~πειην", _, "~ῐ~φθεῑμεν|~ῐ~φθειημεν|~ῐ~πεῑμεν|~ῐ~πειημεν";
    aorist_opt_pass_2: "~ῐ~φθειης|~ῐ~πειης", "~ῐ~φθεῑτον|~ῐ~φθειητον|~ῐ~πεῑτον|~ῐ~πειητον", "~ῐ~φθεῑτε|~ῐ~φθειητε|~ῐ~πεῑτε|~ῐ~πειητε";
    aorist_opt_pass_3: "~ῐ~φθειη|~ῐ~πειη", "~ῐ~φθειτην|~ῐ~φθειητην|~ῐ~πειτην|~ῐ~πειητην", "~ῐ~φθεῑεν|~ῐ~φθειησᾰν|~ῐ~πεῑεν|~ῐ~πειησᾰν";
    aorist_imp_mid_2: "~ῐ~ψαι", "~ῐ~ψᾰσθον", "~ῐ~ψᾰσθε";
    aorist_imp_mid_3: "~ῐ~ψᾰσθω", "~ῐ~ψᾰσθων", _;
    aorist_imp_pass_2: "~ῐ~φθητῐ|~ῐ~πηθῐ", _, _;
    aorist_imp_pass_3: "~ῐ~φθητω|~ῐ~πητω", "~ῐ~φθητων|~ῐ~πητων", "~ῐ~φθεντων|~ῐ~πεντων";
    perfect_ind_mid_1: "~ε~μμαι", _, "~ε~μμεθᾰ";
    perfect_ind_mid_2: "~ε~ψαι", "~ε~φθον", "~ε~φθε";
    perfect_ind_mid_3: "~ε~πται", _, "~ε~πᾰται";
    perfect_imp_mid_2: "~ε~ψο", _, _;
    perfect_imp_mid_3: "~ε~φθω", "~ε~φθων", _;
    infinitive_pres: _, "~ῐ~πτεσθαι", _;
    infinitive_fut: _, "~ῐ~ψεσθαι", "~ῐ~πησεσθαι";
    infinitive_aor: _, "~ῐ~ψᾰσθαι", "~ῐ~φθηναι|~ῐ~πηναι";
    infinitive_perf: _, "~ε~φθαι", _;
    participle_pres: _, "~ῐ~πτομενος", _;
    participle_fut: _, "~ῐ~ψομενος", "~ῐ~πησομενος";
    participle_aor: _, "~ῐ~ψᾰμενος", "~ῐ~φθεις|~ῐ~πεις";
    participle_perf: _, "~ε~μμενος", _;
    ),
    // Κολυμβαω: 1 words (κολυμβαω). Like Μετεκῑᾰθον, except in 68 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("κολυμβαω", 1, "~υ~αω", like Μετεκῑᾰθον;
    stem imperfect: "~ῠ~- like μειδᾰω";
    stem aorist_ind: "~ῠ~ησ- like χᾰριζω";
    ind_act_1: "~ῠ~ω", _, "~ῠ~ωμεν";
    ind_act_2: "~ῠ~ᾳς", "~ῠ~ᾱτον", "~ῠ~ᾱτε";
    ind_act_3: "~ῠ~ᾳ", _, "~ῠ~ωσῐ|~ῠ~ωσῐν";
    subj_act_1: "~ῠ~ω", _, "~ῠ~ωμεν";
    subj_act_2: "~ῠ~ᾳς", "~ῠ~ᾱτον", "~ῠ~ᾱτε";
    subj_act_3: "~ῠ~ᾳ", _, "~ῠ~ωσῐ|~ῠ~ωσῐν";
    opt_act_1: "~ῠ~ῳην|~ῠ~ῳμῐ", _, "~ῠ~ῳμεν|~ῠ~ῳημεν";
    opt_act_2: "~ῠ~ῳης|~ῠ~ῳς", "~ῠ~ῳτον|~ῠ~ῳητον", "~ῠ~ῳτε|~ῠ~ῳητε";
    opt_act_3: "~ῠ~ῳη|~ῠ~ῳ", "~ῠ~ῳτην|~ῠ~ῳητην", "~ῠ~ῳεν|~ῠ~ῳησᾰν";
    imp_act_2: "~ῠ~ᾱ", _, _;
    imp_act_3: "~ῠ~ᾱτω", "~ῠ~ᾱτων", "~ῠ~ωντων";
    future_ind_act_1: "~ῠ~ησω", _, "~ῠ~ησομεν";
    future_ind_act_2: "~ῠ~ησεις", "~ῠ~ησετον", "~ῠ~ησετε";
    future_ind_act_3: "~ῠ~ησει", _, "~ῠ~ησουσῐ|~ῠ~ησουσῐν";
    future_opt_act_1: "~ῠ~ησοιμῐ", _, "~ῠ~ησοιμεν";
    future_opt_act_2: "~ῠ~ησοις", "~ῠ~ησοιτον", "~ῠ~ησοιτε";
    future_opt_act_3: "~ῠ~ησοι", "~ῠ~ησοιτην", "~ῠ~ησοιεν";
    aorist_subj_act_1: "~ῠ~ησω", _, "~ῠ~ησωμεν";
    aorist_subj_act_2: "~ῠ~ησῃς", "~ῠ~ησητον", "~ῠ~ησητε";
    aorist_subj_act_3: "~ῠ~ησῃ", _, "~ῠ~ησωσῐ|~ῠ~ησωσῐν";
    aorist_opt_act_1: "~ῠ~ησαιμῐ", _, "~ῠ~ησαιμεν";
    aorist_opt_act_2: "~ῠ~ησειᾰς|~ῠ~ησαις", "~ῠ~ησαιτον", "~ῠ~ησαιτε";
    aorist_opt_act_3: "~ῠ~ησειε|~ῠ~ησειεν|~ῠ~ησαι", "~ῠ~ησαιτην", "~ῠ~ησειᾰν|~ῠ~ησαιεν";
    aorist_imp_act_2: "~ῠ~ησον", "~ῠ~ησᾰτον", "~ῠ~ησᾰτε";
    aorist_imp_act_3: "~ῠ~ησᾰτω", "~ῠ~ησᾰτων", "~ῠ~ησᾰντων";
    infinitive_pres: "~ῠ~ᾱν", _, _;
    infinitive_fut: "~ῠ~ησειν", _, _;
    infinitive_aor: "~ῠ~ησαι", _, _;
    participle_pres: "~ῠ~ων", _, _;
    participle_fut: "~ῠ~ησων", _, _;
    participle_aor: "~ῠ~ησᾱς", _, _;
    ),
    // Δῐκεῑν: 1 words (δῐκεῑν). Like Ἐπορον, except in 2 cells: ind_act_1 sg, aorist_subj_act_3 pl.
    conjugation!("δῐκεῑν", 1, "~εῑν", like Ἐπορον;
    stem aorist_ind: "~- like ἐπεφνον";
    ind_act_1: "~εῑν", _, _;
    aorist_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    ),
    // Ὑπονοεω: 1 words (ὑπονοεω). Like Πεμπω, except in 236 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ὑπονοεω", 1, "ὑ~ο~εω", like Πεμπω;
    stem aorist_ind: "~ε~ησ- like ὑπονοεω";
    stem aorist_ind_pass: "~ε~ηθη- like ὑπονοεω";
    stem pluperfect_act: "~ε%ε~ηκ- like ὑπονοεω";
    ind_act_1: "ῠ~ο~ω", _, "ῠ~ο~οῡμεν";
    ind_act_2: "ῠ~ο~εῑς", "ῠ~ο~εῑτον", "ῠ~ο~εῑτε";
    ind_act_3: "ῠ~ο~εῑ", _, "ῠ~ο~οῡσῐ|ῠ~ο~οῡσῐν";
    ind_mid_1: "ῠ~ο~οῡμαι", _, "ῠ~ο~ουμεθᾰ";
    ind_mid_2: "ῠ~ο~εῑ|ῠ~ο~ῃ", "ῠ~ο~εῑσθον", "ῠ~ο~εῑσθε";
    ind_mid_3: "ῠ~ο~εῑται", _, "ῠ~ο~οῡνται";
    subj_act_1: "ῠ~ο~ω", _, "ῠ~ο~ωμεν";
    subj_act_2: "ῠ~ο~ῃς", "ῠ~ο~ητον", "ῠ~ο~ητε";
    subj_act_3: "ῠ~ο~ῃ", _, "ῠ~ο~ωσῐ|ῠ~ο~ωσῐν";
    subj_mid_1: "ῠ~ο~ωμαι", _, "ῠ~ο~ωμεθᾰ";
    subj_mid_2: _, "ῠ~ο~ησθον", "ῠ~ο~ησθε";
    subj_mid_3: "ῠ~ο~ηται", _, "ῠ~ο~ωνται";
    opt_act_1: "ῠ~ο~οιην|ῠ~ο~οῑμῐ", _, "ῠ~ο~οῑμεν|ῠ~ο~οιημεν";
    opt_act_2: "ῠ~ο~οιης|ῠ~ο~οῑς", "ῠ~ο~οῑτον|ῠ~ο~οιητον", "ῠ~ο~οῑτε|ῠ~ο~οιητε";
    opt_act_3: "ῠ~ο~οιη|ῠ~ο~οῑ", "ῠ~ο~οιτην|ῠ~ο~οιητην", "ῠ~ο~οῑεν|ῠ~ο~οιησᾰν";
    opt_mid_1: "ῠ~ο~οιμην", _, "ῠ~ο~οιμεθᾰ";
    opt_mid_2: "ῠ~ο~οῑο", "ῠ~ο~οῑσθον", "ῠ~ο~οῑσθε";
    opt_mid_3: "ῠ~ο~οῑτο", "ῠ~ο~οισθην", "ῠ~ο~οῑντο";
    imp_act_2: "ῠ~ο~ει", _, _;
    imp_act_3: "ῠ~ο~ειτω", "ῠ~ο~ειτων", "ῠ~ο~ουντων";
    imp_mid_2: "ῠ~ο~οῡ", _, _;
    imp_mid_3: "ῠ~ο~εισθω", "ῠ~ο~εισθων", _;
    imperfect_act_1: "ῠ~ε~ουν", _, "ῠ~ε~οῡμεν";
    imperfect_act_2: "ῠ~ε~εις", "ῠ~ε~εῑτον", "ῠ~ε~εῑτε";
    imperfect_act_3: "ῠ~ε~ει", "ῠ~ε~ειτην", "ῠ~ε~ουν";
    imperfect_mid_1: "ῠ~ε~ουμην", _, "ῠ~ε~ουμεθᾰ";
    imperfect_mid_2: "ῠ~ε~οῡ", "ῠ~ε~εῑσθον", "ῠ~ε~εῑσθε";
    imperfect_mid_3: "ῠ~ε~εῑτο", "ῠ~ε~εισθην", "ῠ~ε~οῡντο";
    future_ind_act_1: "ῠ~ο~ησω", _, "ῠ~ο~ησομεν";
    future_ind_act_2: "ῠ~ο~ησεις", "ῠ~ο~ησετον", "ῠ~ο~ησετε";
    future_ind_act_3: "ῠ~ο~ησει", _, "ῠ~ο~ησουσῐ|ῠ~ο~ησουσῐν";
    future_ind_mid_1: "ῠ~ο~ησομαι", _, "ῠ~ο~ησομεθᾰ";
    future_ind_mid_2: "ῠ~ο~ησῃ|ῠ~ο~ησει", "ῠ~ο~ησεσθον", "ῠ~ο~ησεσθε";
    future_ind_mid_3: "ῠ~ο~ησεται", _, "ῠ~ο~ησονται";
    future_ind_pass_1: "ῠ~ο~ηθησομαι", _, "ῠ~ο~ηθησομεθᾰ";
    future_ind_pass_2: "ῠ~ο~ηθησῃ|ῠ~ο~ηθησει", "ῠ~ο~ηθησεσθον", "ῠ~ο~ηθησεσθε";
    future_ind_pass_3: "ῠ~ο~ηθησεται", _, "ῠ~ο~ηθησονται";
    future_opt_act_1: "ῠ~ο~ησοιμῐ", _, "ῠ~ο~ησοιμεν";
    future_opt_act_2: "ῠ~ο~ησοις", "ῠ~ο~ησοιτον", "ῠ~ο~ησοιτε";
    future_opt_act_3: "ῠ~ο~ησοι", "ῠ~ο~ησοιτην", "ῠ~ο~ησοιεν";
    future_opt_mid_1: "ῠ~ο~ησοιμην", _, "ῠ~ο~ησοιμεθᾰ";
    future_opt_mid_2: "ῠ~ο~ησοιο", "ῠ~ο~ησοισθον", "ῠ~ο~ησοισθε";
    future_opt_mid_3: "ῠ~ο~ησοιτο", "ῠ~ο~ησοισθην", "ῠ~ο~ησοιντο";
    future_opt_pass_1: "ῠ~ο~ηθησοιμην", _, "ῠ~ο~ηθησοιμεθᾰ";
    future_opt_pass_2: "ῠ~ο~ηθησοιο", "ῠ~ο~ηθησοισθον", "ῠ~ο~ηθησοισθε";
    future_opt_pass_3: "ῠ~ο~ηθησοιτο", "ῠ~ο~ηθησοισθην", "ῠ~ο~ηθησοιντο";
    aorist_subj_act_1: "ῠ~ο~ησω", _, "ῠ~ο~ησωμεν";
    aorist_subj_act_2: "ῠ~ο~ησῃς", "ῠ~ο~ησητον", "ῠ~ο~ησητε";
    aorist_subj_act_3: "ῠ~ο~ησῃ", _, "ῠ~ο~ησωσῐ|ῠ~ο~ησωσῐν";
    aorist_subj_mid_1: "ῠ~ο~ησωμαι", _, "ῠ~ο~ησωμεθᾰ";
    aorist_subj_mid_2: _, "ῠ~ο~ησησθον", "ῠ~ο~ησησθε";
    aorist_subj_mid_3: "ῠ~ο~ησηται", _, "ῠ~ο~ησωνται";
    aorist_subj_pass_1: "ῠ~ο~ηθω", _, "ῠ~ο~ηθωμεν";
    aorist_subj_pass_2: "ῠ~ο~ηθῃς", "ῠ~ο~ηθητον", "ῠ~ο~ηθητε";
    aorist_subj_pass_3: "ῠ~ο~ηθῃ", _, "ῠ~ο~ηθωσῐ|ῠ~ο~ηθωσῐν";
    aorist_opt_act_1: "ῠ~ο~ησαιμῐ", _, "ῠ~ο~ησαιμεν";
    aorist_opt_act_2: "ῠ~ο~ησειᾰς|ῠ~ο~ησαις", "ῠ~ο~ησαιτον", "ῠ~ο~ησαιτε";
    aorist_opt_act_3: "ῠ~ο~ησειε|ῠ~ο~ησειεν|ῠ~ο~ησαι", "ῠ~ο~ησαιτην", "ῠ~ο~ησειᾰν|ῠ~ο~ησαιεν";
    aorist_opt_mid_1: "ῠ~ο~ησαιμην", _, "ῠ~ο~ησαιμεθᾰ";
    aorist_opt_mid_2: "ῠ~ο~ησαιο", "ῠ~ο~ησαισθον", "ῠ~ο~ησαισθε";
    aorist_opt_mid_3: "ῠ~ο~ησαιτο", "ῠ~ο~ησαισθην", "ῠ~ο~ησαιντο";
    aorist_opt_pass_1: "ῠ~ο~ηθειην", _, "ῠ~ο~ηθεῑμεν|ῠ~ο~ηθειημεν";
    aorist_opt_pass_2: "ῠ~ο~ηθειης", "ῠ~ο~ηθεῑτον|ῠ~ο~ηθειητον", "ῠ~ο~ηθεῑτε|ῠ~ο~ηθειητε";
    aorist_opt_pass_3: "ῠ~ο~ηθειη", "ῠ~ο~ηθειτην|ῠ~ο~ηθειητην", "ῠ~ο~ηθεῑεν|ῠ~ο~ηθειησᾰν";
    aorist_imp_act_2: "ῠ~ο~ησον", "ῠ~ο~ησᾰτον", "ῠ~ο~ησᾰτε";
    aorist_imp_act_3: "ῠ~ο~ησᾰτω", "ῠ~ο~ησᾰτων", "ῠ~ο~ησᾰντων";
    aorist_imp_mid_2: "ῠ~ο~ησαι", "ῠ~ο~ησᾰσθον", "ῠ~ο~ησᾰσθε";
    aorist_imp_mid_3: "ῠ~ο~ησᾰσθω", "ῠ~ο~ησᾰσθων", _;
    aorist_imp_pass_2: "ῠ~ο~ηθητῐ", _, _;
    aorist_imp_pass_3: "ῠ~ο~ηθητω", "ῠ~ο~ηθητων", "ῠ~ο~ηθεντων";
    perfect_ind_act_1: "ῠ~ο%ε~ηκᾰ", _, "ῠ~ο%ε~ηκᾰμεν";
    perfect_ind_act_2: "ῠ~ο%ε~ηκᾰς", "ῠ~ο%ε~ηκᾰτον", "ῠ~ο%ε~ηκᾰτε";
    perfect_ind_act_3: "ῠ~ο%ε~ηκε|ῠ~ο%ε~ηκεν", _, "ῠ~ο%ε~ηκᾱσῐ|ῠ~ο%ε~ηκᾱσῐν";
    perfect_ind_mid_1: "ῠ~ο%ε~ημαι", _, "ῠ~ο%ε~ημεθᾰ";
    perfect_ind_mid_2: "ῠ~ο%ε~ησαι", "ῠ~ο%ε~ησθον", "ῠ~ο%ε~ησθε";
    perfect_ind_mid_3: "ῠ~ο%ε~ηται", _, "ῠ~ο%ε~ηνται";
    perfect_subj_act_1: "ῠ~ο%ε~ηκω", _, "ῠ~ο%ε~ηκωμεν";
    perfect_subj_act_2: "ῠ~ο%ε~ηκῃς", "ῠ~ο%ε~ηκητον", "ῠ~ο%ε~ηκητε";
    perfect_subj_act_3: "ῠ~ο%ε~ηκῃ", _, "ῠ~ο%ε~ηκωσῐ|ῠ~ο%ε~ηκωσῐν";
    perfect_opt_act_1: "ῠ~ο%ε~ηκοιμῐ|ῠ~ο%ε~ηκοιην", _, "ῠ~ο%ε~ηκοιμεν";
    perfect_opt_act_2: "ῠ~ο%ε~ηκοις|ῠ~ο%ε~ηκοιης", "ῠ~ο%ε~ηκοιτον", "ῠ~ο%ε~ηκοιτε";
    perfect_opt_act_3: "ῠ~ο%ε~ηκοι|ῠ~ο%ε~ηκοιη", "ῠ~ο%ε~ηκοιτην", "ῠ~ο%ε~ηκοιεν";
    perfect_imp_act_2: "ῠ~ο%ε~ηκε", "ῠ~ο%ε~ηκετον", "ῠ~ο%ε~ηκετε";
    perfect_imp_act_3: "ῠ~ο%ε~ηκετω", "ῠ~ο%ε~ηκετων", "ῠ~ο%ε~ηκοντων";
    perfect_imp_mid_2: "ῠ~ο%ε~ησο", _, _;
    perfect_imp_mid_3: "ῠ~ο%ε~ησθω", "ῠ~ο%ε~ησθων", _;
    pluperfect_mid_1: "ῠ~ε%ε~ημην", _, "ῠ~ε%ε~ημεθᾰ";
    pluperfect_mid_2: "ῠ~ε%ε~ησο", "ῠ~ε%ε~ησθον", "ῠ~ε%ε~ησθε";
    pluperfect_mid_3: "ῠ~ε%ε~ητο", "ῠ~ε%ε~ησθην", "ῠ~ε%ε~ηντο";
    infinitive_pres: "ῠ~ο~εῑν", "ῠ~ο~εῑσθαι", _;
    infinitive_fut: "ῠ~ο~ησειν", "ῠ~ο~ησεσθαι", "ῠ~ο~ηθησεσθαι";
    infinitive_aor: "ῠ~ο~ησαι", "ῠ~ο~ησᾰσθαι", "ῠ~ο~ηθηναι";
    infinitive_perf: "ῠ~ο%ε~ηκεναι", "ῠ~ο%ε~ησθαι", _;
    participle_pres: "ῠ~ο~ων", "ῠ~ο~ουμενος", _;
    participle_fut: "ῠ~ο~ησων", "ῠ~ο~ησομενος", "ῠ~ο~ηθησομενος";
    participle_aor: "ῠ~ο~ησᾱς", "ῠ~ο~ησᾰμενος", "ῠ~ο~ηθεις";
    participle_perf: "ῠ~ο%ε~ηκως", "ῠ~ο%ε~ημενος", _;
    ),
    // Ἀντᾰγωνιζομαι: 1 words (ἀντᾰγωνιζομαι). Like Δερκομαι, except in 118 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἀντᾰγωνιζομαι", 1, "~ᾰ~ιζομαι", like Δερκομαι;
    stem imperfect: "~η~ῐζ- like ὑποκοριζομαι";
    stem aorist_ind: "~η~ῐσ- like μηκᾰομαι";
    stem aorist_ind_pass: "~η~ῐσθη- like ἐλπω";
    stem pluperfect: "~η~ῐ- like ὑποκοριζομαι";
    ind_act_1: "~ᾰ~ιζομαι", _, _;
    ind_mid_1: "~ᾰ~ῐζομαι", _, "~ᾰ~ῐζομεθᾰ";
    ind_mid_2: "~ᾰ~ῐζῃ|~ᾰ~ῐζει", "~ᾰ~ῐζεσθον", "~ᾰ~ῐζεσθε";
    ind_mid_3: "~ᾰ~ῐζεται", _, "~ᾰ~ῐζονται";
    subj_mid_1: "~ᾰ~ῐζωμαι", _, "~ᾰ~ῐζωμεθᾰ";
    subj_mid_2: "~ᾰ~ῐζῃ", "~ᾰ~ῐζησθον", "~ᾰ~ῐζησθε";
    subj_mid_3: "~ᾰ~ῐζηται", _, "~ᾰ~ῐζωνται";
    subj_pass_2: "~ᾰ~ῐζῃ", _, _;
    opt_mid_1: "~ᾰ~ῐζοιμην", _, "~ᾰ~ῐζοιμεθᾰ";
    opt_mid_2: "~ᾰ~ῐζοιο", "~ᾰ~ῐζοισθον", "~ᾰ~ῐζοισθε";
    opt_mid_3: "~ᾰ~ῐζοιτο", "~ᾰ~ῐζοισθην", "~ᾰ~ῐζοιντο";
    imp_mid_2: "~ᾰ~ῐζου", _, _;
    imp_mid_3: "~ᾰ~ῐζεσθω", "~ᾰ~ῐζεσθων", _;
    future_ind_mid_1: "~ᾰ~ῐοῡμαι", _, "~ᾰ~ῐουμεθᾰ";
    future_ind_mid_2: "~ᾰ~ῐεῑ|~ᾰ~ῐῃ", "~ᾰ~ῐεῑσθον", "~ᾰ~ῐεῑσθε";
    future_ind_mid_3: "~ᾰ~ῐεῑται", _, "~ᾰ~ῐοῡνται";
    future_ind_pass_1: "~ᾰ~ῐσθησομαι", _, "~ᾰ~ῐσθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ῐσθησῃ|~ᾰ~ῐσθησει", "~ᾰ~ῐσθησεσθον", "~ᾰ~ῐσθησεσθε";
    future_ind_pass_3: "~ᾰ~ῐσθησεται", _, "~ᾰ~ῐσθησονται";
    future_opt_mid_1: "~ᾰ~ῐοιμην", _, "~ᾰ~ῐοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ῐοῑο", "~ᾰ~ῐοῑσθον", "~ᾰ~ῐοῑσθε";
    future_opt_mid_3: "~ᾰ~ῐοῑτο", "~ᾰ~ῐοισθην", "~ᾰ~ῐοῑντο";
    future_opt_pass_1: "~ᾰ~ῐσθησοιμην", _, "~ᾰ~ῐσθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ῐσθησοιο", "~ᾰ~ῐσθησοισθον", "~ᾰ~ῐσθησοισθε";
    future_opt_pass_3: "~ᾰ~ῐσθησοιτο", "~ᾰ~ῐσθησοισθην", "~ᾰ~ῐσθησοιντο";
    aorist_subj_mid_1: "~ᾰ~ῐσωμαι", _, "~ᾰ~ῐσωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~ῐσῃ", "~ᾰ~ῐσησθον", "~ᾰ~ῐσησθε";
    aorist_subj_mid_3: "~ᾰ~ῐσηται", _, "~ᾰ~ῐσωνται";
    aorist_subj_pass_1: "~ᾰ~ῐσθω", _, "~ᾰ~ῐσθωμεν";
    aorist_subj_pass_2: "~ᾰ~ῐσθῃς", "~ᾰ~ῐσθητον", "~ᾰ~ῐσθητε";
    aorist_subj_pass_3: "~ᾰ~ῐσθῃ", _, "~ᾰ~ῐσθωσῐ|~ᾰ~ῐσθωσῐν";
    aorist_opt_mid_1: "~ᾰ~ῐσαιμην", _, "~ᾰ~ῐσαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~ῐσαιο", "~ᾰ~ῐσαισθον", "~ᾰ~ῐσαισθε";
    aorist_opt_mid_3: "~ᾰ~ῐσαιτο", "~ᾰ~ῐσαισθην", "~ᾰ~ῐσαιντο";
    aorist_opt_pass_1: "~ᾰ~ῐσθειην", _, "~ᾰ~ῐσθεῑμεν|~ᾰ~ῐσθειημεν";
    aorist_opt_pass_2: "~ᾰ~ῐσθειης", "~ᾰ~ῐσθεῑτον|~ᾰ~ῐσθειητον", "~ᾰ~ῐσθεῑτε|~ᾰ~ῐσθειητε";
    aorist_opt_pass_3: "~ᾰ~ῐσθειη", "~ᾰ~ῐσθειτην|~ᾰ~ῐσθειητην", "~ᾰ~ῐσθεῑεν|~ᾰ~ῐσθειησᾰν";
    aorist_imp_mid_2: "~ᾰ~ῐσαι", "~ᾰ~ῐσᾰσθον", "~ᾰ~ῐσᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ῐσᾰσθω", "~ᾰ~ῐσᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~ῐσθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ῐσθητω", "~ᾰ~ῐσθητων", "~ᾰ~ῐσθεντων";
    perfect_ind_mid_1: "~η~ῐσμαι", _, "~η~ῐσμεθᾰ";
    perfect_ind_mid_2: "~η~ῐσαι", "~η~ῐσθον", "~η~ῐσθε";
    perfect_ind_mid_3: "~η~ῐσται", _, "~η~ῐδᾰται";
    perfect_imp_mid_2: "~η~ῐσο", _, _;
    perfect_imp_mid_3: "~η~ῐσθω", "~η~ῐσθων", _;
    infinitive_pres: _, "~ᾰ~ῐζεσθαι", _;
    infinitive_fut: _, "~ᾰ~ῐεῑσθαι", "~ᾰ~ῐσθησεσθαι";
    infinitive_aor: _, "~ᾰ~ῐσᾰσθαι", "~ᾰ~ῐσθηναι";
    infinitive_perf: _, "~η~ῐσθαι", _;
    participle_pres: _, "~ᾰ~ῐζομενος", _;
    participle_fut: _, "~ᾰ~ῐουμενος", "~ᾰ~ῐσθησομενος";
    participle_aor: _, "~ᾰ~ῐσᾰμενος", "~ᾰ~ῐσθεις";
    participle_perf: _, "~η~ῐσμενος", _;
    ),
    // Διπλᾰσῐοομαι: 1 words (διπλᾰσῐοομαι). Like Νισσομαι, except in 23 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("διπλᾰσῐοομαι", 1, "~ι~οομαι", like Νισσομαι;
    ind_act_1: "~ι~οομαι", _, _;
    ind_mid_1: "~ῐ~οῡμαι", _, "~ῐ~ουμεθᾰ";
    ind_mid_2: "~ῐ~οῑ", "~ῐ~οῡσθον", "~ῐ~οῡσθε";
    ind_mid_3: "~ῐ~οῡται", _, "~ῐ~οῡνται";
    subj_mid_2: "~ῐ~οῑ", "~ῐ~ωσθον", "~ῐ~ωσθε";
    subj_mid_3: "~ῐ~ωται", _, _;
    subj_pass_2: "~ῐ~οῑ", _, _;
    opt_mid_2: "~ῐ~οῑο", "~ῐ~οῑσθον", "~ῐ~οῑσθε";
    opt_mid_3: "~ῐ~οῑτο", _, "~ῐ~οῑντο";
    imp_mid_2: "~ῐ~οῡ", _, _;
    imp_mid_3: "~ῐ~ουσθω", "~ῐ~ουσθων", _;
    infinitive_pres: _, "~ῐ~οῡσθαι", _;
    participle_pres: _, "~ῐ~ουμενος", _;
    ),
    // Ἐξικνεομαι: 1 words (ἐξικνεομαι). Like Ἐρχομαι, except in 86 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἐξικνεομαι", 1, "~ικνεομαι", like Ἐρχομαι;
    stem aorist_ind: "~ῑκ- like ἐρχομαι";
    ind_act_1: "~ικνεομαι", _, _;
    ind_mid_1: "~ῐκνοῡμαι", _, "~ῐκνουμεθᾰ";
    ind_mid_2: "~ῐκνεῑ|~ῐκνῃ", "~ῐκνεῑσθον", "~ῐκνεῑσθε";
    ind_mid_3: "~ῐκνεῑται", _, "~ῐκνοῡνται";
    subj_mid_1: "~ῐκνωμαι", _, "~ῐκνωμεθᾰ";
    subj_mid_2: "~ῐκνῃ", "~ῐκνησθον", "~ῐκνησθε";
    subj_mid_3: "~ῐκνηται", _, "~ῐκνωνται";
    subj_pass_2: "~ῐκνῃ", _, _;
    opt_mid_1: "~ῐκνοιμην", _, "~ῐκνοιμεθᾰ";
    opt_mid_2: "~ῐκνοῑο", "~ῐκνοῑσθον", "~ῐκνοῑσθε";
    opt_mid_3: "~ῐκνοῑτο", "~ῐκνοισθην", "~ῐκνοῑντο";
    imp_mid_2: "~ῐκνοῡ", _, _;
    imp_mid_3: "~ῐκνεισθω", "~ῐκνεισθων", _;
    imperfect_mid_1: "~ῑκνουμην", _, "~ῑκνουμεθᾰ";
    imperfect_mid_2: "~ῑκνοῡ", "~ῑκνεῑσθον", "~ῑκνεῑσθε";
    imperfect_mid_3: "~ῑκνεῑτο", "~ῑκνεισθην", "~ῑκνοῡντο";
    future_ind_mid_1: "~ῐξομαι", _, "~ῐξομεθᾰ";
    future_ind_mid_2: "~ῐξῃ|~ῐξει", "~ῐξεσθον", "~ῐξεσθε";
    future_ind_mid_3: "~ῐξεται", _, "~ῐξονται";
    future_opt_mid_1: "~ῐξοιμην", _, "~ῐξοιμεθᾰ";
    future_opt_mid_2: "~ῐξοιο", "~ῐξοισθον", "~ῐξοισθε";
    future_opt_mid_3: "~ῐξοιτο", "~ῐξοισθην", "~ῐξοιντο";
    aorist_subj_mid_1: "~ῐκωμαι", _, "~ῐκωμεθᾰ";
    aorist_subj_mid_2: "~ῐκῃ", "~ῐκησθον", "~ῐκησθε";
    aorist_subj_mid_3: "~ῐκηται", _, "~ῐκωνται";
    aorist_opt_mid_1: "~ῐκοιμην", _, "~ῐκοιμεθᾰ";
    aorist_opt_mid_2: "~ῐκοιο", "~ῐκοισθον", "~ῐκοισθε";
    aorist_opt_mid_3: "~ῐκοιτο", "~ῐκοισθην", "~ῐκοιντο";
    aorist_imp_mid_2: "~ῐκοῡ", "~ῐκεσθον", "~ῐκεσθε";
    aorist_imp_mid_3: "~ῐκεσθω", "~ῐκεσθων", _;
    perfect_ind_mid_1: "~ῑγμαι", _, "~ῑγμεθᾰ";
    perfect_ind_mid_2: "~ῑξαι", "~ῑχθον", "~ῑχθε";
    perfect_ind_mid_3: "~ῑκται", _, "~ῑγᾰται";
    perfect_imp_mid_2: "~ῑξο", _, _;
    perfect_imp_mid_3: "~ῑχθω", "~ῑχθων", _;
    infinitive_pres: _, "~ῐκνεῑσθαι", _;
    infinitive_fut: _, "~ῐξεσθαι", _;
    infinitive_aor: _, "~ῐκεσθαι", _;
    participle_pres: _, "~ῐκνουμενος", _;
    participle_fut: _, "~ῐξομενος", _;
    participle_aor: _, "~ῐκομενος", _;
    ),
    // Καταπετροω: 1 words (καταπετροω). Like Λαλαγεω, except in 204 cells: ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_mid_2 sg, ind_mid_2 du, ….
    conjugation!("καταπετροω", 1, "~α~α~οω", like Λαλαγεω;
    stem aorist_ind: "~ᾰ~ε~ωσ- like εὐλογεω";
    stem aorist_ind_pass: "~ᾰ~ε~ωθη- like ἐλπω";
    stem pluperfect_act: "~ᾰ~ε~ωκ- like εὐλογεω";
    ind_act_2: "~ᾰ~ᾰ~οῑς", "~ᾰ~ᾰ~οῡτον", "~ᾰ~ᾰ~οῡτε";
    ind_act_3: "~ᾰ~ᾰ~οῑ", _, _;
    ind_mid_2: "~ᾰ~ᾰ~οῑ", "~ᾰ~ᾰ~οῡσθον", "~ᾰ~ᾰ~οῡσθε";
    ind_mid_3: "~ᾰ~ᾰ~οῡται", _, _;
    subj_act_2: "~ᾰ~ᾰ~οῑς", "~ᾰ~ᾰ~ωτον", "~ᾰ~ᾰ~ωτε";
    subj_act_3: "~ᾰ~ᾰ~οῑ", _, _;
    subj_mid_2: _, "~ᾰ~ᾰ~ωσθον", "~ᾰ~ᾰ~ωσθε";
    subj_mid_3: "~ᾰ~ᾰ~ωται", _, _;
    imp_act_2: "~ᾰ~ᾰ~ου", _, _;
    imp_act_3: "~ᾰ~ᾰ~ουτω", "~ᾰ~ᾰ~ουτων", _;
    imp_mid_3: "~ᾰ~ᾰ~ουσθω", "~ᾰ~ᾰ~ουσθων", _;
    imperfect_act_1: "~ᾰ~ε~ουν", _, "~ᾰ~ε~οῡμεν";
    imperfect_act_2: "~ᾰ~ε~ους", "~ᾰ~ε~οῡτον", "~ᾰ~ε~οῡτε";
    imperfect_act_3: "~ᾰ~ε~ου", "~ᾰ~ε~ουτην", "~ᾰ~ε~ουν";
    imperfect_mid_1: "~ᾰ~ε~ουμην", _, "~ᾰ~ε~ουμεθᾰ";
    imperfect_mid_2: "~ᾰ~ε~οῡ", "~ᾰ~ε~οῡσθον", "~ᾰ~ε~οῡσθε";
    imperfect_mid_3: "~ᾰ~ε~οῡτο", "~ᾰ~ε~ουσθην", "~ᾰ~ε~οῡντο";
    future_ind_act_1: "~ᾰ~ᾰ~ωσω", _, "~ᾰ~ᾰ~ωσομεν";
    future_ind_act_2: "~ᾰ~ᾰ~ωσεις", "~ᾰ~ᾰ~ωσετον", "~ᾰ~ᾰ~ωσετε";
    future_ind_act_3: "~ᾰ~ᾰ~ωσει", _, "~ᾰ~ᾰ~ωσουσῐ|~ᾰ~ᾰ~ωσουσῐν";
    future_ind_mid_1: "~ᾰ~ᾰ~ωσομαι", _, "~ᾰ~ᾰ~ωσομεθᾰ";
    future_ind_mid_2: "~ᾰ~ᾰ~ωσῃ|~ᾰ~ᾰ~ωσει", "~ᾰ~ᾰ~ωσεσθον", "~ᾰ~ᾰ~ωσεσθε";
    future_ind_mid_3: "~ᾰ~ᾰ~ωσεται", _, "~ᾰ~ᾰ~ωσονται";
    future_ind_pass_1: "~ᾰ~ᾰ~ωθησομαι", _, "~ᾰ~ᾰ~ωθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ᾰ~ωθησῃ|~ᾰ~ᾰ~ωθησει", "~ᾰ~ᾰ~ωθησεσθον", "~ᾰ~ᾰ~ωθησεσθε";
    future_ind_pass_3: "~ᾰ~ᾰ~ωθησεται", _, "~ᾰ~ᾰ~ωθησονται";
    future_opt_act_1: "~ᾰ~ᾰ~ωσοιμῐ", _, "~ᾰ~ᾰ~ωσοιμεν";
    future_opt_act_2: "~ᾰ~ᾰ~ωσοις", "~ᾰ~ᾰ~ωσοιτον", "~ᾰ~ᾰ~ωσοιτε";
    future_opt_act_3: "~ᾰ~ᾰ~ωσοι", "~ᾰ~ᾰ~ωσοιτην", "~ᾰ~ᾰ~ωσοιεν";
    future_opt_mid_1: "~ᾰ~ᾰ~ωσοιμην", _, "~ᾰ~ᾰ~ωσοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ᾰ~ωσοιο", "~ᾰ~ᾰ~ωσοισθον", "~ᾰ~ᾰ~ωσοισθε";
    future_opt_mid_3: "~ᾰ~ᾰ~ωσοιτο", "~ᾰ~ᾰ~ωσοισθην", "~ᾰ~ᾰ~ωσοιντο";
    future_opt_pass_1: "~ᾰ~ᾰ~ωθησοιμην", _, "~ᾰ~ᾰ~ωθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ᾰ~ωθησοιο", "~ᾰ~ᾰ~ωθησοισθον", "~ᾰ~ᾰ~ωθησοισθε";
    future_opt_pass_3: "~ᾰ~ᾰ~ωθησοιτο", "~ᾰ~ᾰ~ωθησοισθην", "~ᾰ~ᾰ~ωθησοιντο";
    aorist_subj_act_1: "~ᾰ~ᾰ~ωσω", _, "~ᾰ~ᾰ~ωσωμεν";
    aorist_subj_act_2: "~ᾰ~ᾰ~ωσῃς", "~ᾰ~ᾰ~ωσητον", "~ᾰ~ᾰ~ωσητε";
    aorist_subj_act_3: "~ᾰ~ᾰ~ωσῃ", _, "~ᾰ~ᾰ~ωσωσῐ|~ᾰ~ᾰ~ωσωσῐν";
    aorist_subj_mid_1: "~ᾰ~ᾰ~ωσωμαι", _, "~ᾰ~ᾰ~ωσωμεθᾰ";
    aorist_subj_mid_2: _, "~ᾰ~ᾰ~ωσησθον", "~ᾰ~ᾰ~ωσησθε";
    aorist_subj_mid_3: "~ᾰ~ᾰ~ωσηται", _, "~ᾰ~ᾰ~ωσωνται";
    aorist_subj_pass_1: "~ᾰ~ᾰ~ωθω", _, "~ᾰ~ᾰ~ωθωμεν";
    aorist_subj_pass_2: "~ᾰ~ᾰ~ωθῃς", "~ᾰ~ᾰ~ωθητον", "~ᾰ~ᾰ~ωθητε";
    aorist_subj_pass_3: "~ᾰ~ᾰ~ωθῃ", _, "~ᾰ~ᾰ~ωθωσῐ|~ᾰ~ᾰ~ωθωσῐν";
    aorist_opt_act_1: "~ᾰ~ᾰ~ωσαιμῐ", _, "~ᾰ~ᾰ~ωσαιμεν";
    aorist_opt_act_2: "~ᾰ~ᾰ~ωσειᾰς|~ᾰ~ᾰ~ωσαις", "~ᾰ~ᾰ~ωσαιτον", "~ᾰ~ᾰ~ωσαιτε";
    aorist_opt_act_3: "~ᾰ~ᾰ~ωσειε|~ᾰ~ᾰ~ωσειεν|~ᾰ~ᾰ~ωσαι", "~ᾰ~ᾰ~ωσαιτην", "~ᾰ~ᾰ~ωσειᾰν|~ᾰ~ᾰ~ωσαιεν";
    aorist_opt_mid_1: "~ᾰ~ᾰ~ωσαιμην", _, "~ᾰ~ᾰ~ωσαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~ᾰ~ωσαιο", "~ᾰ~ᾰ~ωσαισθον", "~ᾰ~ᾰ~ωσαισθε";
    aorist_opt_mid_3: "~ᾰ~ᾰ~ωσαιτο", "~ᾰ~ᾰ~ωσαισθην", "~ᾰ~ᾰ~ωσαιντο";
    aorist_opt_pass_1: "~ᾰ~ᾰ~ωθειην", _, "~ᾰ~ᾰ~ωθεῑμεν|~ᾰ~ᾰ~ωθειημεν";
    aorist_opt_pass_2: "~ᾰ~ᾰ~ωθειης", "~ᾰ~ᾰ~ωθεῑτον|~ᾰ~ᾰ~ωθειητον", "~ᾰ~ᾰ~ωθεῑτε|~ᾰ~ᾰ~ωθειητε";
    aorist_opt_pass_3: "~ᾰ~ᾰ~ωθειη", "~ᾰ~ᾰ~ωθειτην|~ᾰ~ᾰ~ωθειητην", "~ᾰ~ᾰ~ωθεῑεν|~ᾰ~ᾰ~ωθειησᾰν";
    aorist_imp_act_2: "~ᾰ~ᾰ~ωσον", "~ᾰ~ᾰ~ωσᾰτον", "~ᾰ~ᾰ~ωσᾰτε";
    aorist_imp_act_3: "~ᾰ~ᾰ~ωσᾰτω", "~ᾰ~ᾰ~ωσᾰτων", "~ᾰ~ᾰ~ωσᾰντων";
    aorist_imp_mid_2: "~ᾰ~ᾰ~ωσαι", "~ᾰ~ᾰ~ωσᾰσθον", "~ᾰ~ᾰ~ωσᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ᾰ~ωσᾰσθω", "~ᾰ~ᾰ~ωσᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~ᾰ~ωθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ᾰ~ωθητω", "~ᾰ~ᾰ~ωθητων", "~ᾰ~ᾰ~ωθεντων";
    perfect_ind_act_1: "~ᾰ~ε~ωκᾰ", _, "~ᾰ~ε~ωκᾰμεν";
    perfect_ind_act_2: "~ᾰ~ε~ωκᾰς", "~ᾰ~ε~ωκᾰτον", "~ᾰ~ε~ωκᾰτε";
    perfect_ind_act_3: "~ᾰ~ε~ωκε|~ᾰ~ε~ωκεν", _, "~ᾰ~ε~ωκᾱσῐ|~ᾰ~ε~ωκᾱσῐν";
    perfect_ind_mid_1: "~ᾰ~ε~ωμαι", _, "~ᾰ~ε~ωμεθᾰ";
    perfect_ind_mid_2: "~ᾰ~ε~ωσαι", "~ᾰ~ε~ωσθον", "~ᾰ~ε~ωσθε";
    perfect_ind_mid_3: "~ᾰ~ε~ωται", _, "~ᾰ~ε~ωνται";
    perfect_subj_act_1: "~ᾰ~ε~ωκω", _, "~ᾰ~ε~ωκωμεν";
    perfect_subj_act_2: "~ᾰ~ε~ωκῃς", "~ᾰ~ε~ωκητον", "~ᾰ~ε~ωκητε";
    perfect_subj_act_3: "~ᾰ~ε~ωκῃ", _, "~ᾰ~ε~ωκωσῐ|~ᾰ~ε~ωκωσῐν";
    perfect_opt_act_1: "~ᾰ~ε~ωκοιμῐ|~ᾰ~ε~ωκοιην", _, "~ᾰ~ε~ωκοιμεν";
    perfect_opt_act_2: "~ᾰ~ε~ωκοις|~ᾰ~ε~ωκοιης", "~ᾰ~ε~ωκοιτον", "~ᾰ~ε~ωκοιτε";
    perfect_opt_act_3: "~ᾰ~ε~ωκοι|~ᾰ~ε~ωκοιη", "~ᾰ~ε~ωκοιτην", "~ᾰ~ε~ωκοιεν";
    perfect_imp_act_2: "~ᾰ~ε~ωκε", "~ᾰ~ε~ωκετον", "~ᾰ~ε~ωκετε";
    perfect_imp_act_3: "~ᾰ~ε~ωκετω", "~ᾰ~ε~ωκετων", "~ᾰ~ε~ωκοντων";
    perfect_imp_mid_2: "~ᾰ~ε~ωσο", _, _;
    perfect_imp_mid_3: "~ᾰ~ε~ωσθω", "~ᾰ~ε~ωσθων", _;
    pluperfect_mid_1: "~ᾰ~ε~ωμην", _, "~ᾰ~ε~ωμεθᾰ";
    pluperfect_mid_2: "~ᾰ~ε~ωσο", "~ᾰ~ε~ωσθον", "~ᾰ~ε~ωσθε";
    pluperfect_mid_3: "~ᾰ~ε~ωτο", "~ᾰ~ε~ωσθην", "~ᾰ~ε~ωντο";
    infinitive_pres: "~ᾰ~ᾰ~οῡν", "~ᾰ~ᾰ~οῡσθαι", _;
    infinitive_fut: "~ᾰ~ᾰ~ωσειν", "~ᾰ~ᾰ~ωσεσθαι", "~ᾰ~ᾰ~ωθησεσθαι";
    infinitive_aor: "~ᾰ~ᾰ~ωσαι", "~ᾰ~ᾰ~ωσᾰσθαι", "~ᾰ~ᾰ~ωθηναι";
    infinitive_perf: "~ᾰ~ε~ωκεναι", "~ᾰ~ε~ωσθαι", _;
    participle_fut: "~ᾰ~ᾰ~ωσων", "~ᾰ~ᾰ~ωσομενος", "~ᾰ~ᾰ~ωθησομενος";
    participle_aor: "~ᾰ~ᾰ~ωσᾱς", "~ᾰ~ᾰ~ωσᾰμενος", "~ᾰ~ᾰ~ωθεις";
    participle_perf: "~ᾰ~ε~ωκως", "~ᾰ~ε~ωμενος", _;
    ),
    // Λαλαχευομαι: 1 words (λαλαχευομαι). Like Ξυνναυμᾰχεω, except in 77 cells: ind_act_1 sg, ind_pass_1 sg, ind_pass_1 pl, ind_pass_2 sg, ind_pass_2 du, ind_pass_2 pl, ….
    conjugation!("λαλαχευομαι", 1, "~α~α~ομαι", like Ξυνναυμᾰχεω as ACT_AS_PASS;
    stem imperfect: "~ᾰ~ᾰ~- like λαλαχευομαι";
    stem aorist_ind_pass: "~ᾰ~ᾰ~θη- like ἐχω";
    stem perfect: "~ε~ᾰλᾰ~- like λαλαχευομαι";
    stem pluperfect: "~ε~ᾰλᾰ~- like λαλαχευομαι";
    ind_act_1: "~α~α~ομαι", _, _;
    ind_pass_1: "~ᾰ~ᾰ~ομαι", _, "~ᾰ~ᾰ~ομεθᾰ";
    ind_pass_2: "~ᾰ~ᾰ~ῃ|~ᾰ~ᾰ~ει", "~ᾰ~ᾰ~εσθον", "~ᾰ~ᾰ~εσθε";
    ind_pass_3: "~ᾰ~ᾰ~εται", "~ᾰ~ᾰ~εσθον", "~ᾰ~ᾰ~ονται";
    subj_pass_1: "~ᾰ~ᾰ~ωμαι", _, "~ᾰ~ᾰ~ωμεθᾰ";
    subj_pass_2: "~ᾰ~ᾰ~ῃ", "~ᾰ~ᾰ~ησθον", "~ᾰ~ᾰ~ησθε";
    subj_pass_3: "~ᾰ~ᾰ~ηται", "~ᾰ~ᾰ~ησθον", "~ᾰ~ᾰ~ωνται";
    opt_pass_1: "~ᾰ~ᾰ~οιμην", _, "~ᾰ~ᾰ~οιμεθᾰ";
    opt_pass_2: "~ᾰ~ᾰ~οιο", "~ᾰ~ᾰ~οισθον", "~ᾰ~ᾰ~οισθε";
    opt_pass_3: "~ᾰ~ᾰ~οιτο", "~ᾰ~ᾰ~οισθην", "~ᾰ~ᾰ~οιντο";
    imp_pass_2: "~ᾰ~ᾰ~ου", "~ᾰ~ᾰ~εσθον", "~ᾰ~ᾰ~εσθε";
    imp_pass_3: "~ᾰ~ᾰ~εσθω", "~ᾰ~ᾰ~εσθων", "~ᾰ~ᾰ~εσθων";
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    future_ind_pass_1: "~ᾰ~ᾰ~θησομαι", _, "~ᾰ~ᾰ~θησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ᾰ~θησῃ|~ᾰ~ᾰ~θησει", "~ᾰ~ᾰ~θησεσθον", "~ᾰ~ᾰ~θησεσθε";
    future_ind_pass_3: "~ᾰ~ᾰ~θησεται", _, "~ᾰ~ᾰ~θησονται";
    future_opt_pass_1: "~ᾰ~ᾰ~θησοιμην", _, "~ᾰ~ᾰ~θησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ᾰ~θησοιο", "~ᾰ~ᾰ~θησοισθον", "~ᾰ~ᾰ~θησοισθε";
    future_opt_pass_3: "~ᾰ~ᾰ~θησοιτο", "~ᾰ~ᾰ~θησοισθην", "~ᾰ~ᾰ~θησοιντο";
    aorist_subj_pass_1: "~ᾰ~ᾰ~θω", _, "~ᾰ~ᾰ~θωμεν";
    aorist_subj_pass_2: "~ᾰ~ᾰ~θῃς", "~ᾰ~ᾰ~θητον", "~ᾰ~ᾰ~θητε";
    aorist_subj_pass_3: "~ᾰ~ᾰ~θῃ", _, "~ᾰ~ᾰ~θωσῐ|~ᾰ~ᾰ~θωσῐν";
    aorist_opt_pass_1: "~ᾰ~ᾰ~θειην", _, "~ᾰ~ᾰ~θεῑμεν|~ᾰ~ᾰ~θειημεν";
    aorist_opt_pass_2: "~ᾰ~ᾰ~θειης", "~ᾰ~ᾰ~θεῑτον|~ᾰ~ᾰ~θειητον", "~ᾰ~ᾰ~θεῑτε|~ᾰ~ᾰ~θειητε";
    aorist_opt_pass_3: "~ᾰ~ᾰ~θειη", "~ᾰ~ᾰ~θειτην|~ᾰ~ᾰ~θειητην", "~ᾰ~ᾰ~θεῑεν|~ᾰ~ᾰ~θειησᾰν";
    aorist_imp_pass_2: "~ᾰ~ᾰ~θητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ᾰ~θητω", "~ᾰ~ᾰ~θητων", "~ᾰ~ᾰ~θεντων";
    infinitive_fut: _, _, "~ᾰ~ᾰ~θησεσθαι";
    infinitive_aor: _, _, "~ᾰ~ᾰ~θηναι";
    participle_fut: _, _, "~ᾰ~ᾰ~θησομενος";
    participle_aor: _, _, "~ᾰ~ᾰ~θεις";
    ),
    // Μικρογρᾰφεω: 1 words (μικρογρᾰφεω). Like Διπλᾰσῐοομαι, except in 40 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("μικρογρᾰφεω", 1, "~ι~εω", like Διπλᾰσῐοομαι;
    ind_act_1: "~ῐ~ω", _, "~ῐ~οῡμεν";
    ind_act_2: "~ῐ~εῑς", "~ῐ~εῑτον", "~ῐ~εῑτε";
    ind_act_3: "~ῐ~εῑ", _, "~ῐ~οῡσῐ|~ῐ~οῡσῐν";
    ind_mid_2: "~ῐ~εῑ|~ῐ~ῃ", "~ῐ~εῑσθον", "~ῐ~εῑσθε";
    ind_mid_3: "~ῐ~εῑται", _, _;
    subj_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    subj_act_2: "~ῐ~ῃς", "~ῐ~ητον", "~ῐ~ητε";
    subj_act_3: "~ῐ~ῃ", _, "~ῐ~ωσῐ|~ῐ~ωσῐν";
    subj_mid_2: derived, "~ῐ~ησθον", "~ῐ~ησθε";
    subj_mid_3: "~ῐ~ηται", _, _;
    subj_pass_2: derived, _, _;
    opt_act_1: "~ῐ~οιην|~ῐ~οῑμῐ", _, "~ῐ~οῑμεν|~ῐ~οιημεν";
    opt_act_2: "~ῐ~οιης|~ῐ~οῑς", "~ῐ~οῑτον|~ῐ~οιητον", "~ῐ~οῑτε|~ῐ~οιητε";
    opt_act_3: "~ῐ~οιη|~ῐ~οῑ", "~ῐ~οιτην|~ῐ~οιητην", "~ῐ~οῑεν|~ῐ~οιησᾰν";
    imp_act_2: "~ῐ~ει", _, _;
    imp_act_3: "~ῐ~ειτω", "~ῐ~ειτων", "~ῐ~ουντων";
    imp_mid_3: "~ῐ~εισθω", "~ῐ~εισθων", _;
    infinitive_pres: "~ῐ~εῑν", "~ῐ~εῑσθαι", _;
    participle_pres: "~ῐ~ων", _, _;
    ),
    // Λῐνοστᾰτεω: 1 words (λῐνοστᾰτεω). Like Ἐπῐκᾰλεω, except in 182 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("λῐνοστᾰτεω", 1, "~ῐ~ᾰ~εω", like Ἐπῐκᾰλεω;
    stem aorist_ind: "~ι~α~ησ- like ποιεω";
    stem aorist_ind_pass: "~ι~α~ηθη- like ἐχω";
    stem pluperfect_act: "~ελι~α~ηκ- like δῐκεω";
    imperfect_act_1: "ἐ~ι~α~ουν", _, "ἐ~ι~α~οῡμεν";
    imperfect_act_2: "ἐ~ι~α~εις", "ἐ~ι~α~εῑτον", "ἐ~ι~α~εῑτε";
    imperfect_act_3: "ἐ~ι~α~ει", "ἐ~ι~α~ειτην", "ἐ~ι~α~ουν";
    imperfect_mid_1: "ἐ~ι~α~ουμην", _, "ἐ~ι~α~ουμεθᾰ";
    imperfect_mid_2: "ἐ~ι~α~οῡ", "ἐ~ι~α~εῑσθον", "ἐ~ι~α~εῑσθε";
    imperfect_mid_3: "ἐ~ι~α~εῑτο", "ἐ~ι~α~εισθην", "ἐ~ι~α~οῡντο";
    future_ind_act_1: "~ι~α~ησω", _, "~ι~α~ησομεν";
    future_ind_act_2: "~ι~α~ησεις", "~ι~α~ησετον", "~ι~α~ησετε";
    future_ind_act_3: "~ι~α~ησει", _, "~ι~α~ησουσῐ|~ι~α~ησουσῐν";
    future_ind_mid_1: "~ι~α~ησομαι", _, "~ι~α~ησομεθᾰ";
    future_ind_mid_2: "~ι~α~ησῃ|~ι~α~ησει", "~ι~α~ησεσθον", "~ι~α~ησεσθε";
    future_ind_mid_3: "~ι~α~ησεται", _, "~ι~α~ησονται";
    future_ind_pass_1: "~ι~α~ηθησομαι", _, "~ι~α~ηθησομεθᾰ";
    future_ind_pass_2: "~ι~α~ηθησῃ|~ι~α~ηθησει", "~ι~α~ηθησεσθον", "~ι~α~ηθησεσθε";
    future_ind_pass_3: "~ι~α~ηθησεται", _, "~ι~α~ηθησονται";
    future_opt_act_1: "~ι~α~ησοιμῐ", _, "~ι~α~ησοιμεν";
    future_opt_act_2: "~ι~α~ησοις", "~ι~α~ησοιτον", "~ι~α~ησοιτε";
    future_opt_act_3: "~ι~α~ησοι", "~ι~α~ησοιτην", "~ι~α~ησοιεν";
    future_opt_mid_1: "~ι~α~ησοιμην", _, "~ι~α~ησοιμεθᾰ";
    future_opt_mid_2: "~ι~α~ησοιο", "~ι~α~ησοισθον", "~ι~α~ησοισθε";
    future_opt_mid_3: "~ι~α~ησοιτο", "~ι~α~ησοισθην", "~ι~α~ησοιντο";
    future_opt_pass_1: "~ι~α~ηθησοιμην", _, "~ι~α~ηθησοιμεθᾰ";
    future_opt_pass_2: "~ι~α~ηθησοιο", "~ι~α~ηθησοισθον", "~ι~α~ηθησοισθε";
    future_opt_pass_3: "~ι~α~ηθησοιτο", "~ι~α~ηθησοισθην", "~ι~α~ηθησοιντο";
    aorist_subj_act_1: "~ι~α~ησω", _, "~ι~α~ησωμεν";
    aorist_subj_act_2: "~ι~α~ησῃς", "~ι~α~ησητον", "~ι~α~ησητε";
    aorist_subj_act_3: "~ι~α~ησῃ", _, "~ι~α~ησωσῐ|~ι~α~ησωσῐν";
    aorist_subj_mid_1: "~ι~α~ησωμαι", _, "~ι~α~ησωμεθᾰ";
    aorist_subj_mid_2: _, "~ι~α~ησησθον", "~ι~α~ησησθε";
    aorist_subj_mid_3: "~ι~α~ησηται", _, "~ι~α~ησωνται";
    aorist_subj_pass_1: "~ι~α~ηθω", _, "~ι~α~ηθωμεν";
    aorist_subj_pass_2: "~ι~α~ηθῃς", "~ι~α~ηθητον", "~ι~α~ηθητε";
    aorist_subj_pass_3: "~ι~α~ηθῃ", _, "~ι~α~ηθωσῐ|~ι~α~ηθωσῐν";
    aorist_opt_act_1: "~ι~α~ησαιμῐ", _, "~ι~α~ησαιμεν";
    aorist_opt_act_2: "~ι~α~ησειᾰς|~ι~α~ησαις", "~ι~α~ησαιτον", "~ι~α~ησαιτε";
    aorist_opt_act_3: "~ι~α~ησειε|~ι~α~ησειεν|~ι~α~ησαι", "~ι~α~ησαιτην", "~ι~α~ησειᾰν|~ι~α~ησαιεν";
    aorist_opt_mid_1: "~ι~α~ησαιμην", _, "~ι~α~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ι~α~ησαιο", "~ι~α~ησαισθον", "~ι~α~ησαισθε";
    aorist_opt_mid_3: "~ι~α~ησαιτο", "~ι~α~ησαισθην", "~ι~α~ησαιντο";
    aorist_opt_pass_1: "~ι~α~ηθειην", _, "~ι~α~ηθεῑμεν|~ι~α~ηθειημεν";
    aorist_opt_pass_2: "~ι~α~ηθειης", "~ι~α~ηθεῑτον|~ι~α~ηθειητον", "~ι~α~ηθεῑτε|~ι~α~ηθειητε";
    aorist_opt_pass_3: "~ι~α~ηθειη", "~ι~α~ηθειτην|~ι~α~ηθειητην", "~ι~α~ηθεῑεν|~ι~α~ηθειησᾰν";
    aorist_imp_act_2: "~ι~α~ησον", "~ι~α~ησᾰτον", "~ι~α~ησᾰτε";
    aorist_imp_act_3: "~ι~α~ησᾰτω", "~ι~α~ησᾰτων", "~ι~α~ησᾰντων";
    aorist_imp_mid_2: "~ι~α~ησαι", "~ι~α~ησᾰσθον", "~ι~α~ησᾰσθε";
    aorist_imp_mid_3: "~ι~α~ησᾰσθω", "~ι~α~ησᾰσθων", _;
    aorist_imp_pass_2: "~ι~α~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ι~α~ηθητω", "~ι~α~ηθητων", "~ι~α~ηθεντων";
    perfect_ind_act_1: "~ελι~α~ηκᾰ", _, "~ελι~α~ηκᾰμεν";
    perfect_ind_act_2: "~ελι~α~ηκᾰς", "~ελι~α~ηκᾰτον", "~ελι~α~ηκᾰτε";
    perfect_ind_act_3: "~ελι~α~ηκε|~ελι~α~ηκεν", _, "~ελι~α~ηκᾱσῐ|~ελι~α~ηκᾱσῐν";
    perfect_ind_mid_1: "~ελι~α~ημαι", _, "~ελι~α~ημεθᾰ";
    perfect_ind_mid_2: "~ελι~α~ησαι", "~ελι~α~ησθον", "~ελι~α~ησθε";
    perfect_ind_mid_3: "~ελι~α~ηται", _, "~ελι~α~ηνται";
    perfect_subj_act_1: "~ελι~α~ηκω", _, "~ελι~α~ηκωμεν";
    perfect_subj_act_2: "~ελι~α~ηκῃς", "~ελι~α~ηκητον", "~ελι~α~ηκητε";
    perfect_subj_act_3: "~ελι~α~ηκῃ", _, "~ελι~α~ηκωσῐ|~ελι~α~ηκωσῐν";
    perfect_opt_act_1: "~ελι~α~ηκοιμῐ|~ελι~α~ηκοιην", _, "~ελι~α~ηκοιμεν";
    perfect_opt_act_2: "~ελι~α~ηκοις|~ελι~α~ηκοιης", "~ελι~α~ηκοιτον", "~ελι~α~ηκοιτε";
    perfect_opt_act_3: "~ελι~α~ηκοι|~ελι~α~ηκοιη", "~ελι~α~ηκοιτην", "~ελι~α~ηκοιεν";
    perfect_imp_act_2: "~ελι~α~ηκε", "~ελι~α~ηκετον", "~ελι~α~ηκετε";
    perfect_imp_act_3: "~ελι~α~ηκετω", "~ελι~α~ηκετων", "~ελι~α~ηκοντων";
    perfect_imp_mid_2: "~ελι~α~ησο", _, _;
    perfect_imp_mid_3: "~ελι~α~ησθω", "~ελι~α~ησθων", _;
    pluperfect_mid_1: "ἐ~ελι~α~ημην", _, "ἐ~ελι~α~ημεθᾰ";
    pluperfect_mid_2: "ἐ~ελι~α~ησο", "ἐ~ελι~α~ησθον", "ἐ~ελι~α~ησθε";
    pluperfect_mid_3: "ἐ~ελι~α~ητο", "ἐ~ελι~α~ησθην", "ἐ~ελι~α~ηντο";
    infinitive_fut: "~ι~α~ησειν", "~ι~α~ησεσθαι", "~ι~α~ηθησεσθαι";
    infinitive_aor: "~ι~α~ησαι", "~ι~α~ησᾰσθαι", "~ι~α~ηθηναι";
    infinitive_perf: "~ελι~α~ηκεναι", "~ελι~α~ησθαι", _;
    participle_fut: "~ι~α~ησων", "~ι~α~ησομενος", "~ι~α~ηθησομενος";
    participle_aor: "~ι~α~ησᾱς", "~ι~α~ησᾰμενος", "~ι~α~ηθεις";
    participle_perf: "~ελι~α~ηκως", "~ελι~α~ημενος", _;
    ),
    // Ἀδησᾰ: 1 words (ἀδησᾰ). Like Κᾰθιδρῡω, except in 53 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀδησᾰ", 1, "~σᾰ", like Κᾰθιδρῡω as MID_AS_PASS;
    ind_act_1: "~σᾰ", _, derived;
    ind_act_2: derived, derived, derived;
    ind_act_3: derived, _, derived;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, _, _;
    imp_act_3: derived, derived, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: derived, _, derived;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    participle_pres: derived, _, _;
    participle_fut: derived, _, _;
    ),
    // Ὑποκοριζομαι: 1 words (ὑποκοριζομαι). Like Κᾰθημαι, except in 75 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ὑποκοριζομαι", 1, "~ο~ιζομαι", like Κᾰθημαι;
    stem imperfect: "~ε~ῐζ- like ὑποκοριζομαι";
    stem aorist_ind: "~ε~ῐσ- like μηκᾰομαι";
    stem pluperfect: "~ε%ε~ῐ- like ὑποκοριζομαι";
    ind_act_1: "~ο~ιζομαι", _, _;
    ind_mid_1: "~ο~ῐζομαι", _, "~ο~ῐζομεθᾰ";
    ind_mid_2: "~ο~ῐζῃ|~ο~ῐζει", "~ο~ῐζεσθον", "~ο~ῐζεσθε";
    ind_mid_3: "~ο~ῐζεται", _, "~ο~ῐζονται";
    subj_mid_1: "~ο~ῐζωμαι", _, "~ο~ῐζωμεθᾰ";
    subj_mid_2: "~ο~ῐζῃ", "~ο~ῐζησθον", "~ο~ῐζησθε";
    subj_mid_3: "~ο~ῐζηται", _, "~ο~ῐζωνται";
    opt_mid_1: "~ο~ῐζοιμην", _, "~ο~ῐζοιμεθᾰ";
    opt_mid_2: "~ο~ῐζοιο", "~ο~ῐζοισθον", "~ο~ῐζοισθε";
    opt_mid_3: "~ο~ῐζοιτο", "~ο~ῐζοισθην", "~ο~ῐζοιντο";
    imp_mid_2: "~ο~ῐζου", _, _;
    imp_mid_3: "~ο~ῐζεσθω", "~ο~ῐζεσθων", _;
    future_ind_mid_1: "~ο~ῐοῡμαι", _, "~ο~ῐουμεθᾰ";
    future_ind_mid_2: "~ο~ῐεῑ|~ο~ῐῃ", "~ο~ῐεῑσθον", "~ο~ῐεῑσθε";
    future_ind_mid_3: "~ο~ῐεῑται", _, "~ο~ῐοῡνται";
    future_opt_mid_1: "~ο~ῐοιμην", _, "~ο~ῐοιμεθᾰ";
    future_opt_mid_2: "~ο~ῐοῑο", "~ο~ῐοῑσθον", "~ο~ῐοῑσθε";
    future_opt_mid_3: "~ο~ῐοῑτο", "~ο~ῐοισθην", "~ο~ῐοῑντο";
    aorist_subj_mid_1: "~ο~ῐσωμαι", _, "~ο~ῐσωμεθᾰ";
    aorist_subj_mid_2: "~ο~ῐσῃ", "~ο~ῐσησθον", "~ο~ῐσησθε";
    aorist_subj_mid_3: "~ο~ῐσηται", _, "~ο~ῐσωνται";
    aorist_opt_mid_1: "~ο~ῐσαιμην", _, "~ο~ῐσαιμεθᾰ";
    aorist_opt_mid_2: "~ο~ῐσαιο", "~ο~ῐσαισθον", "~ο~ῐσαισθε";
    aorist_opt_mid_3: "~ο~ῐσαιτο", "~ο~ῐσαισθην", "~ο~ῐσαιντο";
    aorist_imp_mid_2: "~ο~ῐσαι", "~ο~ῐσᾰσθον", "~ο~ῐσᾰσθε";
    aorist_imp_mid_3: "~ο~ῐσᾰσθω", "~ο~ῐσᾰσθων", _;
    perfect_ind_mid_1: "~ο%ε~ῐσμαι", _, "~ο%ε~ῐσμεθᾰ";
    perfect_ind_mid_2: "~ο%ε~ῐσαι", "~ο%ε~ῐσθον", "~ο%ε~ῐσθε";
    perfect_ind_mid_3: "~ο%ε~ῐσται", _, "~ο%ε~ῐδᾰται";
    perfect_imp_mid_2: "~ο%ε~ῐσο", _, _;
    perfect_imp_mid_3: "~ο%ε~ῐσθω", "~ο%ε~ῐσθων", _;
    infinitive_fut: _, "~ο~ῐεῑσθαι", _;
    infinitive_aor: _, "~ο~ῐσᾰσθαι", _;
    participle_fut: _, "~ο~ῐουμενος", _;
    participle_aor: _, "~ο~ῐσᾰμενος", _;
    ),
    // Ἰχᾰνᾰω: 1 words (ἰχᾰνᾰω). Like Μικρογρᾰφεω, except in 54 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἰχᾰνᾰω", 1, "~ᾰ~ᾰω", like Μικρογρᾰφεω;
    ind_act_1: "~α~ω", _, "~α~ωμεν";
    ind_act_2: "~α~ᾳς", "~α~ᾱτον", "~α~ᾱτε";
    ind_act_3: "~α~ᾳ", _, "~α~ωσῐ|~α~ωσῐν";
    ind_mid_1: "~α~ωμαι", _, "~α~ωμεθᾰ";
    ind_mid_2: "~α~ᾳ", "~α~ᾱσθον", "~α~ᾱσθε";
    ind_mid_3: "~α~ᾱται", _, "~α~ωνται";
    subj_act_1: "~α~ω", _, "~α~ωμεν";
    subj_act_2: "~α~ᾳς", "~α~ᾱτον", "~α~ᾱτε";
    subj_act_3: "~α~ᾳ", _, "~α~ωσῐ|~α~ωσῐν";
    subj_mid_1: "~α~ωμαι", _, "~α~ωμεθᾰ";
    subj_mid_2: _, "~α~ᾱσθον", "~α~ᾱσθε";
    subj_mid_3: "~α~ᾱται", _, "~α~ωνται";
    opt_act_1: "~α~ῳην|~α~ῳμῐ", _, "~α~ῳμεν|~α~ῳημεν";
    opt_act_2: "~α~ῳης|~α~ῳς", "~α~ῳτον|~α~ῳητον", "~α~ῳτε|~α~ῳητε";
    opt_act_3: "~α~ῳη|~α~ῳ", "~α~ῳτην|~α~ῳητην", "~α~ῳεν|~α~ῳησᾰν";
    opt_mid_1: "~α~ῳμην", _, "~α~ῳμεθᾰ";
    opt_mid_2: "~α~ῳο", "~α~ῳσθον", "~α~ῳσθε";
    opt_mid_3: "~α~ῳτο", "~α~ῳσθην", "~α~ῳντο";
    imp_act_2: "~α~ᾱ", _, _;
    imp_act_3: "~α~ᾱτω", "~α~ᾱτων", "~α~ωντων";
    imp_mid_2: "~α~ω", _, _;
    imp_mid_3: "~α~ᾱσθω", "~α~ᾱσθων", _;
    infinitive_pres: "~α~ᾱν", "~α~ᾱσθαι", _;
    participle_pres: "~α~ων", "~α~ωμενος", _;
    ),
    // Ᾰντᾰποκρῑνομαι: 1 words (ᾰντᾰποκρῑνομαι). Like Δερκομαι, except in 126 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ᾰντᾰποκρῑνομαι", 1, "ᾰ~ο~ῑνομαι", like Δερκομαι;
    stem aorist_ind: "~ε~ῑν- like ᾰντᾰποκρῑνομαι";
    stem aorist_ind_pass: "~ε~ῐθη- like ᾰντᾰποκρῑνομαι";
    ind_act_1: "ᾰ~ο~ῑνομαι", _, _;
    ind_mid_1: "ἀ~ο~ῑνομαι", _, "ἀ~ο~ῑνομεθᾰ";
    ind_mid_2: "ἀ~ο~ῑνῃ|ἀ~ο~ῑνει", "ἀ~ο~ῑνεσθον", "ἀ~ο~ῑνεσθε";
    ind_mid_3: "ἀ~ο~ῑνεται", _, "ἀ~ο~ῑνονται";
    subj_mid_1: "ἀ~ο~ῑνωμαι", _, "ἀ~ο~ῑνωμεθᾰ";
    subj_mid_2: "ἀ~ο~ῑνῃ", "ἀ~ο~ῑνησθον", "ἀ~ο~ῑνησθε";
    subj_mid_3: "ἀ~ο~ῑνηται", _, "ἀ~ο~ῑνωνται";
    subj_pass_2: "ἀ~ο~ῑνῃ", _, _;
    opt_mid_1: "ἀ~ο~ῑνοιμην", _, "ἀ~ο~ῑνοιμεθᾰ";
    opt_mid_2: "ἀ~ο~ῑνοιο", "ἀ~ο~ῑνοισθον", "ἀ~ο~ῑνοισθε";
    opt_mid_3: "ἀ~ο~ῑνοιτο", "ἀ~ο~ῑνοισθην", "ἀ~ο~ῑνοιντο";
    imp_mid_2: "ἀ~ο~ῑνου", _, _;
    imp_mid_3: "ἀ~ο~ῑνεσθω", "ἀ~ο~ῑνεσθων", _;
    imperfect_mid_1: "ἀ~ε~ῑνομην", _, "ἀ~ε~ῑνομεθᾰ";
    imperfect_mid_2: "ἀ~ε~ῑνου", "ἀ~ε~ῑνεσθον", "ἀ~ε~ῑνεσθε";
    imperfect_mid_3: "ἀ~ε~ῑνετο", "ἀ~ε~ῑνεσθην", "ἀ~ε~ῑνοντο";
    future_ind_mid_1: "ἀ~ο~ῐνοῡμαι", _, "ἀ~ο~ῐνουμεθᾰ";
    future_ind_mid_2: "ἀ~ο~ῐνεῑ|ἀ~ο~ῐνῃ", "ἀ~ο~ῐνεῑσθον", "ἀ~ο~ῐνεῑσθε";
    future_ind_mid_3: "ἀ~ο~ῐνεῑται", _, "ἀ~ο~ῐνοῡνται";
    future_ind_pass_1: "ἀ~ο~ῐθησομαι", _, "ἀ~ο~ῐθησομεθᾰ";
    future_ind_pass_2: "ἀ~ο~ῐθησῃ|ἀ~ο~ῐθησει", "ἀ~ο~ῐθησεσθον", "ἀ~ο~ῐθησεσθε";
    future_ind_pass_3: "ἀ~ο~ῐθησεται", _, "ἀ~ο~ῐθησονται";
    future_opt_mid_1: "ἀ~ο~ῐνοιμην", _, "ἀ~ο~ῐνοιμεθᾰ";
    future_opt_mid_2: "ἀ~ο~ῐνοῑο", "ἀ~ο~ῐνοῑσθον", "ἀ~ο~ῐνοῑσθε";
    future_opt_mid_3: "ἀ~ο~ῐνοῑτο", "ἀ~ο~ῐνοισθην", "ἀ~ο~ῐνοῑντο";
    future_opt_pass_1: "ἀ~ο~ῐθησοιμην", _, "ἀ~ο~ῐθησοιμεθᾰ";
    future_opt_pass_2: "ἀ~ο~ῐθησοιο", "ἀ~ο~ῐθησοισθον", "ἀ~ο~ῐθησοισθε";
    future_opt_pass_3: "ἀ~ο~ῐθησοιτο", "ἀ~ο~ῐθησοισθην", "ἀ~ο~ῐθησοιντο";
    aorist_subj_mid_1: "ἀ~ο~ῑνωμαι", _, "ἀ~ο~ῑνωμεθᾰ";
    aorist_subj_mid_2: "ἀ~ο~ῑνῃ", "ἀ~ο~ῑνησθον", "ἀ~ο~ῑνησθε";
    aorist_subj_mid_3: "ἀ~ο~ῑνηται", _, "ἀ~ο~ῑνωνται";
    aorist_subj_pass_1: "ἀ~ο~ῐθω", _, "ἀ~ο~ῐθωμεν";
    aorist_subj_pass_2: "ἀ~ο~ῐθῃς", "ἀ~ο~ῐθητον", "ἀ~ο~ῐθητε";
    aorist_subj_pass_3: "ἀ~ο~ῐθῃ", _, "ἀ~ο~ῐθωσῐ|ἀ~ο~ῐθωσῐν";
    aorist_opt_mid_1: "ἀ~ο~ῑναιμην", _, "ἀ~ο~ῑναιμεθᾰ";
    aorist_opt_mid_2: "ἀ~ο~ῑναιο", "ἀ~ο~ῑναισθον", "ἀ~ο~ῑναισθε";
    aorist_opt_mid_3: "ἀ~ο~ῑναιτο", "ἀ~ο~ῑναισθην", "ἀ~ο~ῑναιντο";
    aorist_opt_pass_1: "ἀ~ο~ῐθειην", _, "ἀ~ο~ῐθεῑμεν|ἀ~ο~ῐθειημεν";
    aorist_opt_pass_2: "ἀ~ο~ῐθειης", "ἀ~ο~ῐθεῑτον|ἀ~ο~ῐθειητον", "ἀ~ο~ῐθεῑτε|ἀ~ο~ῐθειητε";
    aorist_opt_pass_3: "ἀ~ο~ῐθειη", "ἀ~ο~ῐθειτην|ἀ~ο~ῐθειητην", "ἀ~ο~ῐθεῑεν|ἀ~ο~ῐθειησᾰν";
    aorist_imp_mid_2: "ἀ~ο~ῑναι", "ἀ~ο~ῑνᾰσθον", "ἀ~ο~ῑνᾰσθε";
    aorist_imp_mid_3: "ἀ~ο~ῑνᾰσθω", "ἀ~ο~ῑνᾰσθων", _;
    aorist_imp_pass_2: "ἀ~ο~ῐθητῐ", _, _;
    aorist_imp_pass_3: "ἀ~ο~ῐθητω", "ἀ~ο~ῐθητων", "ἀ~ο~ῐθεντων";
    perfect_ind_mid_1: "ἀ~ο%ε~ῐμαι", _, "ἀ~ο%ε~ῐμεθᾰ";
    perfect_ind_mid_2: "ἀ~ο%ε~ῐσαι", "ἀ~ο%ε~ῐσθον", "ἀ~ο%ε~ῐσθε";
    perfect_ind_mid_3: "ἀ~ο%ε~ῐται", _, "ἀ~ο%ε~ῐνται";
    perfect_imp_mid_2: "ἀ~ο%ε~ῐσο", _, _;
    perfect_imp_mid_3: "ἀ~ο%ε~ῐσθω", "ἀ~ο%ε~ῐσθων", _;
    infinitive_pres: _, "ἀ~ο~ῑνεσθαι", _;
    infinitive_fut: _, "ἀ~ο~ῐνεῑσθαι", "ἀ~ο~ῐθησεσθαι";
    infinitive_aor: _, "ἀ~ο~ῑνᾰσθαι", "ἀ~ο~ῐθηναι";
    infinitive_perf: _, "ἀ~ο%ε~ῐσθαι", _;
    participle_pres: _, "ἀ~ο~ῑνομενος", _;
    participle_fut: _, "ἀ~ο~ῐνουμενος", "ἀ~ο~ῐθησομενος";
    participle_aor: _, "ἀ~ο~ῑνᾰμενος", "ἀ~ο~ῐθεις";
    participle_perf: _, "ἀ~ο%ε~ῐμενος", _;
    ),
    // Ὑπεκτιθεμαι: 1 words (ὑπεκτιθεμαι). Like Δερκομαι, except in 126 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ὑπεκτιθεμαι", 1, "~κτι~εμαι", like Δερκομαι;
    stem aorist_ind: "~ξε~- like ὑπεκτιθεμαι";
    stem aorist_ind_pass: "~ξετε~η- like ἐλπω";
    ind_act_1: "~κτι~εμαι", _, _;
    ind_mid_1: "~κ%ῐ~εμαι", _, "~κ%ῐ~εμεθᾰ";
    ind_mid_2: "~κ%ῐ~εσαι", "~κ%ῐ~εσθον", "~κ%ῐ~εσθε";
    ind_mid_3: "~κ%ῐ~εται", _, "~κ%ῐ~ενται";
    subj_mid_1: "~κ%ῐ~ωμαι", _, "~κ%ῐ~ωμεθᾰ";
    subj_mid_2: "~κ%ῐ~ῃ", "~κ%ῐ~ησθον", "~κ%ῐ~ησθε";
    subj_mid_3: "~κ%ῐ~ηται", _, "~κ%ῐ~ωνται";
    subj_pass_2: "~κ%ῐ~ῃ", _, _;
    opt_mid_1: "~κ%ῐ~ειμην", _, "~κ%ῐ~ειμεθᾰ|~κ%ῐ~οιμεθᾰ";
    opt_mid_2: "~κ%ῐ~εῑο", "~κ%ῐ~εῑσθον|~κ%ῐ~οῑσθον", "~κ%ῐ~εῑσθε|~κ%ῐ~οῑσθε";
    opt_mid_3: "~κ%ῐ~εῑτο|~κ%ῐ~οῑτο", "~κ%ῐ~εισθην|~κ%ῐ~οισθην", "~κ%ῐ~εῑντο|~κ%ῐ~οῑντο";
    imp_mid_2: "~κ%ῐ~εσο|~κ%ῐ~ου", _, _;
    imp_mid_3: "~κ%ῐ~εσθω", "~κ%ῐ~εσθων", _;
    imperfect_mid_1: "~ξε%ῐ~εμην", _, "~ξε%ῐ~εμεθᾰ";
    imperfect_mid_2: "~ξε%ῐ~εσο", "~ξε%ῐ~εσθον", "~ξε%ῐ~εσθε";
    imperfect_mid_3: "~ξε%ῐ~ετο", "~ξε%ῐ~εσθην", "~ξε%ῐ~εντο";
    future_ind_mid_1: "~κ~ησομαι", _, "~κ~ησομεθᾰ";
    future_ind_mid_2: "~κ~ησῃ|~κ~ησει", "~κ~ησεσθον", "~κ~ησεσθε";
    future_ind_mid_3: "~κ~ησεται", _, "~κ~ησονται";
    future_ind_pass_1: "~κ%ε~ησομαι", _, "~κ%ε~ησομεθᾰ";
    future_ind_pass_2: "~κ%ε~ησῃ|~κ%ε~ησει", "~κ%ε~ησεσθον", "~κ%ε~ησεσθε";
    future_ind_pass_3: "~κ%ε~ησεται", _, "~κ%ε~ησονται";
    future_opt_mid_1: "~κ~ησοιμην", _, "~κ~ησοιμεθᾰ";
    future_opt_mid_2: "~κ~ησοιο", "~κ~ησοισθον", "~κ~ησοισθε";
    future_opt_mid_3: "~κ~ησοιτο", "~κ~ησοισθην", "~κ~ησοιντο";
    future_opt_pass_1: "~κ%ε~ησοιμην", _, "~κ%ε~ησοιμεθᾰ";
    future_opt_pass_2: "~κ%ε~ησοιο", "~κ%ε~ησοισθον", "~κ%ε~ησοισθε";
    future_opt_pass_3: "~κ%ε~ησοιτο", "~κ%ε~ησοισθην", "~κ%ε~ησοιντο";
    aorist_subj_mid_1: "~κ~ωμαι", _, "~κ~ωμεθᾰ";
    aorist_subj_mid_2: "~κ~ῃ", "~κ~ησθον", "~κ~ησθε";
    aorist_subj_mid_3: "~κ~ηται", _, "~κ~ωνται";
    aorist_subj_pass_1: "~κτε~ω", _, "~κτε~ωμεν";
    aorist_subj_pass_2: "~κτε~ῃς", "~κτε~ητον", "~κτε~ητε";
    aorist_subj_pass_3: "~κτε~ῃ", _, "~κτε~ωσῐ|~κτε~ωσῐν";
    aorist_opt_mid_1: "~κ~ειμην", _, "~κ~ειμεθᾰ";
    aorist_opt_mid_2: "~κ~εῑο", "~κ~εῑσθον", "~κ~εῑσθε";
    aorist_opt_mid_3: "~κ~εῑτο|~κ~οῑτο", "~κ~εισθην", "~κ~εῑντο";
    aorist_opt_pass_1: "~κτε~ειην", _, "~κτε~εῑμεν|~κτε~ειημεν";
    aorist_opt_pass_2: "~κτε~ειης", "~κτε~εῑτον|~κτε~ειητον", "~κτε~εῑτε|~κτε~ειητε";
    aorist_opt_pass_3: "~κτε~ειη", "~κτε~ειτην|~κτε~ειητην", "~κτε~εῑεν|~κτε~ειησᾰν";
    aorist_imp_mid_2: "~κ~ου", "~κ~εσθον", "~κ~εσθε";
    aorist_imp_mid_3: "~κ~εσθω", "~κ~εσθων", _;
    aorist_imp_pass_2: "~κτε~ηθῐ", _, _;
    aorist_imp_pass_3: "~κτε~ητω", "~κτε~ητων", "~κτε~εντων";
    perfect_ind_mid_1: "~κ%ε~ειμαι", _, "~κ%ε~ειμεθᾰ";
    perfect_ind_mid_2: "~κ%ε~εισαι", "~κ%ε~εισθον", "~κ%ε~εισθε";
    perfect_ind_mid_3: "~κ%ε~ειται", _, "~κ%ε~εινται";
    perfect_imp_mid_2: "~κ%ε~εισο", _, _;
    perfect_imp_mid_3: "~κ%ε~εισθω", "~κ%ε~εισθων", _;
    infinitive_pres: _, "~κ%ῐ~εσθαι", _;
    infinitive_fut: _, "~κ~ησεσθαι", "~κ%ε~ησεσθαι";
    infinitive_aor: _, "~κ~εσθαι", "~κτε~ηναι";
    infinitive_perf: _, "~κ%ε~εῑσθαι", _;
    participle_pres: _, "~κ%ῐ~εμενος", _;
    participle_fut: _, "~κ~ησομενος", "~κ%ε~ησομενος";
    participle_aor: _, "~κ~εμενος", "~κτε~εις";
    participle_perf: _, "~κ%ε~ειμενος", _;
    ),
    // Μᾰστῐγοω: 1 words (μᾰστῐγοω). Like Ξυνναυμᾰχεω, except in 54 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("μᾰστῐγοω", 1, "~ᾰ~ῐ~οω", like Ξυνναυμᾰχεω as ACT_AS_PASS;
    ind_act_1: "~α~ι~ω", _, "~α~ι~ομεν";
    ind_act_2: "~α~ι~εις", "~α~ι~ετον", "~α~ι~ετε";
    ind_act_3: "~α~ι~ει", _, "~α~ι~ουσῐ|~α~ι~ουσῐν";
    ind_mid_1: "~α~ι~ομαι", _, "~α~ι~ομεθᾰ";
    ind_mid_2: "~α~ι~ῃ|~α~ι~ει", "~α~ι~εσθον", "~α~ι~εσθε";
    ind_mid_3: "~α~ι~εται", _, "~α~ι~ονται";
    subj_act_1: "~α~ι~ω", _, "~α~ι~ωμεν";
    subj_act_2: "~α~ι~ῃς", "~α~ι~ητον", "~α~ι~ητε";
    subj_act_3: "~α~ι~ῃ", _, "~α~ι~ωσῐ|~α~ι~ωσῐν";
    subj_mid_1: "~α~ι~ωμαι", _, "~α~ι~ωμεθᾰ";
    subj_mid_2: _, "~α~ι~ησθον", "~α~ι~ησθε";
    subj_mid_3: "~α~ι~ηται", _, "~α~ι~ωνται";
    opt_act_1: "~α~ι~οιμῐ", _, "~α~ι~οιμεν";
    opt_act_2: "~α~ι~οις", "~α~ι~οιτον", "~α~ι~οιτε";
    opt_act_3: "~α~ι~οι", "~α~ι~οιτην", "~α~ι~οιεν";
    opt_mid_1: "~α~ι~οιμην", _, "~α~ι~οιμεθᾰ";
    opt_mid_2: "~α~ι~οιο", "~α~ι~οισθον", "~α~ι~οισθε";
    opt_mid_3: "~α~ι~οιτο", "~α~ι~οισθην", "~α~ι~οιντο";
    imp_act_2: "~α~ι~ε", _, _;
    imp_act_3: "~α~ι~ετω", "~α~ι~ετων", "~α~ι~οντων";
    imp_mid_2: "~α~ι~ου", _, _;
    imp_mid_3: "~α~ι~εσθω", "~α~ι~εσθων", _;
    infinitive_pres: "~α~ι~ειν", "~α~ι~εσθαι", _;
    participle_pres: "~α~ι~ων", "~α~ι~ομενος", _;
    ),
    // Βᾰρβᾰροομαι: 1 words (βᾰρβᾰροομαι). Like Ξυνναυμᾰχεω, except in 32 cells: ind_act_1 sg, aorist_subj_pass_1 sg, aorist_subj_pass_1 pl, aorist_subj_pass_2 sg, aorist_subj_pass_2 du, aorist_subj_pass_2 pl, ….
    conjugation!("βᾰρβᾰροομαι", 1, "~ᾰ~ᾰροομ~ι", like Ξυνναυμᾰχεω as ACT_AS_PASS;
    stem aorist_ind_pass: "~α~~ρωθη- like ἐχω";
    ind_act_1: "~ᾰ~ᾰροομ~ι", _, _;
    aorist_subj_pass_1: "~α~~ρωθω", _, "~α~~ρωθωμεν";
    aorist_subj_pass_2: "~α~~ρωθῃς", "~α~~ρωθητον", "~α~~ρωθητε";
    aorist_subj_pass_3: "~α~~ρωθῃ", _, "~α~~ρωθωσῐ|~α~~ρωθωσῐν";
    aorist_opt_pass_1: "~α~~ρωθειην", _, "~α~~ρωθεῑμεν|~α~~ρωθειημεν";
    aorist_opt_pass_2: "~α~~ρωθειης", "~α~~ρωθεῑτον|~α~~ρωθειητον", "~α~~ρωθεῑτε|~α~~ρωθειητε";
    aorist_opt_pass_3: "~α~~ρωθειη", "~α~~ρωθειτην|~α~~ρωθειητην", "~α~~ρωθεῑεν|~α~~ρωθειησᾰν";
    aorist_imp_pass_2: "~α~~ρωθητῐ", _, _;
    aorist_imp_pass_3: "~α~~ρωθητω", "~α~~ρωθητων", "~α~~ρωθεντων";
    perfect_ind_mid_1: "~εβα~~ρωμαι", _, "~εβα~~ρωμεθᾰ";
    perfect_ind_mid_2: "~εβα~~ρωσαι", "~εβα~~ρωσθον", "~εβα~~ρωσθε";
    perfect_ind_mid_3: "~εβα~~ρωται", _, "~εβα~~ρωνται";
    perfect_imp_mid_2: "~εβα~~ρωσο", _, _;
    perfect_imp_mid_3: "~εβα~~ρωσθω", "~εβα~~ρωσθων", _;
    infinitive_aor: _, _, "~α~~ρωθηναι";
    participle_aor: _, _, "~α~~ρωθεις";
    ),
    // Ἐπιμιμνησκομαι: 1 words (ἐπιμιμνησκομαι). Like Ἀνᾰχρονῐζομαι, except in 126 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἐπιμιμνησκομαι", 1, "~ιμι~σκομαι", like Ἀνᾰχρονῐζομαι;
    stem imperfect: "~ε%ῐ~σκ- like ὑποκοριζομαι";
    ind_act_1: "~ιμι~σκομαι", _, _;
    ind_mid_1: "~ῐ%ῐ~σκομαι", _, "~ῐ%ῐ~σκομεθᾰ";
    ind_mid_2: "~ῐ%ῐ~σκῃ|~ῐ%ῐ~σκει", "~ῐ%ῐ~σκεσθον", "~ῐ%ῐ~σκεσθε";
    ind_mid_3: "~ῐ%ῐ~σκεται", _, "~ῐ%ῐ~σκονται";
    subj_mid_1: "~ῐ%ῐ~σκωμαι", _, "~ῐ%ῐ~σκωμεθᾰ";
    subj_mid_2: "~ῐ%ῐ~σκῃ", "~ῐ%ῐ~σκησθον", "~ῐ%ῐ~σκησθε";
    subj_mid_3: "~ῐ%ῐ~σκηται", _, "~ῐ%ῐ~σκωνται";
    subj_pass_2: "~ῐ%ῐ~σκῃ", _, _;
    opt_mid_1: "~ῐ%ῐ~σκοιμην", _, "~ῐ%ῐ~σκοιμεθᾰ";
    opt_mid_2: "~ῐ%ῐ~σκοιο", "~ῐ%ῐ~σκοισθον", "~ῐ%ῐ~σκοισθε";
    opt_mid_3: "~ῐ%ῐ~σκοιτο", "~ῐ%ῐ~σκοισθην", "~ῐ%ῐ~σκοιντο";
    imp_mid_2: "~ῐ%ῐ~σκου", _, _;
    imp_mid_3: "~ῐ%ῐ~σκεσθω", "~ῐ%ῐ~σκεσθων", _;
    future_ind_mid_1: "~ῐ~σομαι", _, "~ῐ~σομεθᾰ";
    future_ind_mid_2: "~ῐ~σῃ|~ῐ~σει", "~ῐ~σεσθον", "~ῐ~σεσθε";
    future_ind_mid_3: "~ῐ~σεται", _, "~ῐ~σονται";
    future_ind_pass_1: "~ῐ~σθησομαι", _, "~ῐ~σθησομεθᾰ";
    future_ind_pass_2: "~ῐ~σθησῃ|~ῐ~σθησει", "~ῐ~σθησεσθον", "~ῐ~σθησεσθε";
    future_ind_pass_3: "~ῐ~σθησεται", _, "~ῐ~σθησονται";
    future_opt_mid_1: "~ῐ~σοιμην", _, "~ῐ~σοιμεθᾰ";
    future_opt_mid_2: "~ῐ~σοιο", "~ῐ~σοισθον", "~ῐ~σοισθε";
    future_opt_mid_3: "~ῐ~σοιτο", "~ῐ~σοισθην", "~ῐ~σοιντο";
    future_opt_pass_1: "~ῐ~σθησοιμην", _, "~ῐ~σθησοιμεθᾰ";
    future_opt_pass_2: "~ῐ~σθησοιο", "~ῐ~σθησοισθον", "~ῐ~σθησοισθε";
    future_opt_pass_3: "~ῐ~σθησοιτο", "~ῐ~σθησοισθην", "~ῐ~σθησοιντο";
    aorist_subj_mid_1: "~ῐ~σωμαι", _, "~ῐ~σωμεθᾰ";
    aorist_subj_mid_2: "~ῐ~σῃ", "~ῐ~σησθον", "~ῐ~σησθε";
    aorist_subj_mid_3: "~ῐ~σηται", _, "~ῐ~σωνται";
    aorist_subj_pass_1: "~ῐ~σθω", _, "~ῐ~σθωμεν";
    aorist_subj_pass_2: "~ῐ~σθῃς", "~ῐ~σθητον", "~ῐ~σθητε";
    aorist_subj_pass_3: "~ῐ~σθῃ", _, "~ῐ~σθωσῐ|~ῐ~σθωσῐν";
    aorist_opt_mid_1: "~ῐ~σαιμην", _, "~ῐ~σαιμεθᾰ";
    aorist_opt_mid_2: "~ῐ~σαιο", "~ῐ~σαισθον", "~ῐ~σαισθε";
    aorist_opt_mid_3: "~ῐ~σαιτο", "~ῐ~σαισθην", "~ῐ~σαιντο";
    aorist_opt_pass_1: "~ῐ~σθειην", _, "~ῐ~σθεῑμεν|~ῐ~σθειημεν";
    aorist_opt_pass_2: "~ῐ~σθειης", "~ῐ~σθεῑτον|~ῐ~σθειητον", "~ῐ~σθεῑτε|~ῐ~σθειητε";
    aorist_opt_pass_3: "~ῐ~σθειη", "~ῐ~σθειτην|~ῐ~σθειητην", "~ῐ~σθεῑεν|~ῐ~σθειησᾰν";
    aorist_imp_mid_2: "~ῐ~σαι", "~ῐ~σᾰσθον", "~ῐ~σᾰσθε";
    aorist_imp_mid_3: "~ῐ~σᾰσθω", "~ῐ~σᾰσθων", _;
    aorist_imp_pass_2: "~ῐ~σθητῐ", _, _;
    aorist_imp_pass_3: "~ῐ~σθητω", "~ῐ~σθητων", "~ῐ~σθεντων";
    perfect_ind_mid_1: "~ῐ%ε~μαι", _, "~ῐ%ε~μεθᾰ";
    perfect_ind_mid_2: "~ῐ%ε~σαι", "~ῐ%ε~σθον", "~ῐ%ε~σθε";
    perfect_ind_mid_3: "~ῐ%ε~ται", _, "~ῐ%ε~νται";
    perfect_imp_mid_2: "~ῐ%ε~σο", _, _;
    perfect_imp_mid_3: "~ῐ%ε~σθω", "~ῐ%ε~σθων", _;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_pres: _, "~ῐ%ῐ~σκεσθαι", _;
    infinitive_fut: _, "~ῐ~σεσθαι", "~ῐ~σθησεσθαι";
    infinitive_aor: _, "~ῐ~σᾰσθαι", "~ῐ~σθηναι";
    infinitive_perf: _, "~ῐ%ε~σθαι", _;
    participle_pres: _, "~ῐ%ῐ~σκομενος", _;
    participle_fut: _, "~ῐ~σομενος", "~ῐ~σθησομενος";
    participle_aor: _, "~ῐ~σᾰμενος", "~ῐ~σθεις";
    participle_perf: _, "~ῐ%ε~μενος", _;
    ),
    // Εργεω: 1 words (εργεω). Like Ἐλαυνω, except in 37 cells: ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("εργεω", 1, "ε~εω", like Ἐλαυνω as IMPERFECT_AS_PLUPERFECT;
    ind_act_1: _, _, "ἐ~οῡμεν";
    ind_act_2: "ἐ~εῑς", "ἐ~εῑτον", "ἐ~εῑτε";
    ind_act_3: "ἐ~εῑ", _, "ἐ~οῡσῐ|ἐ~οῡσῐν";
    ind_mid_1: "ἐ~οῡμαι", _, "ἐ~ουμεθᾰ";
    ind_mid_2: "ἐ~εῑ|ἐ~ῃ", "ἐ~εῑσθον", "ἐ~εῑσθε";
    ind_mid_3: "ἐ~εῑται", _, "ἐ~οῡνται";
    subj_act_3: _, _, "ἐ~ωσῐ|ἐ~ωσῐν";
    opt_act_1: "ἐ~οιην|ἐ~οῑμῐ", _, "ἐ~οῑμεν|ἐ~οιημεν";
    opt_act_2: "ἐ~οιης|ἐ~οῑς", "ἐ~οῑτον|ἐ~οιητον", "ἐ~οῑτε|ἐ~οιητε";
    opt_act_3: "ἐ~οιη|ἐ~οῑ", "ἐ~οιτην|ἐ~οιητην", "ἐ~οῑεν|ἐ~οιησᾰν";
    opt_mid_2: "ἐ~οῑο", "ἐ~οῑσθον", "ἐ~οῑσθε";
    opt_mid_3: "ἐ~οῑτο", _, "ἐ~οῑντο";
    imp_act_2: "ἐ~ει", _, _;
    imp_act_3: "ἐ~ειτω", "ἐ~ειτων", "ἐ~ουντων";
    imp_mid_2: "ἐ~οῡ", _, _;
    imp_mid_3: "ἐ~εισθω", "ἐ~εισθων", _;
    infinitive_pres: "ἐ~εῑν", "ἐ~εῑσθαι", _;
    participle_pres: _, "ἐ~ουμενος", _;
    ),
    // Ᾰνθρωπεω: 1 words (ᾰνθρωπεω). Like Ἐρωτᾰω, except in 209 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ᾰνθρωπεω", 1, "ᾰ~εω", like Ἐρωτᾰω;
    stem aorist_ind: "~ησ- like ἐλπῐζω";
    ind_act_1: "ἀ~ω", _, "ἀ~οῡμεν";
    ind_act_2: "ἀ~εῑς", "ἀ~εῑτον", "ἀ~εῑτε";
    ind_act_3: "ἀ~εῑ", _, "ἀ~οῡσῐ|ἀ~οῡσῐν";
    ind_mid_1: "ἀ~οῡμαι", _, "ἀ~ουμεθᾰ";
    ind_mid_2: "ἀ~εῑ|ἀ~ῃ", "ἀ~εῑσθον", "ἀ~εῑσθε";
    ind_mid_3: "ἀ~εῑται", _, "ἀ~οῡνται";
    subj_act_1: "ἀ~ω", _, "ἀ~ωμεν";
    subj_act_2: "ἀ~ῃς", "ἀ~ητον", "ἀ~ητε";
    subj_act_3: "ἀ~ῃ", _, "ἀ~ωσῐ|ἀ~ωσῐν";
    subj_mid_1: "ἀ~ωμαι", _, "ἀ~ωμεθᾰ";
    subj_mid_2: _, "ἀ~ησθον", "ἀ~ησθε";
    subj_mid_3: "ἀ~ηται", _, "ἀ~ωνται";
    opt_act_1: "ἀ~οιην|ἀ~οῑμῐ", _, "ἀ~οῑμεν|ἀ~οιημεν";
    opt_act_2: "ἀ~οιης|ἀ~οῑς", "ἀ~οῑτον|ἀ~οιητον", "ἀ~οῑτε|ἀ~οιητε";
    opt_act_3: "ἀ~οιη|ἀ~οῑ", "ἀ~οιτην|ἀ~οιητην", "ἀ~οῑεν|ἀ~οιησᾰν";
    opt_mid_1: "ἀ~οιμην", _, "ἀ~οιμεθᾰ";
    opt_mid_2: "ἀ~οῑο", "ἀ~οῑσθον", "ἀ~οῑσθε";
    opt_mid_3: "ἀ~οῑτο", "ἀ~οισθην", "ἀ~οῑντο";
    imp_act_2: "ἀ~ει", _, _;
    imp_act_3: "ἀ~ειτω", "ἀ~ειτων", "ἀ~ουντων";
    imp_mid_2: "ἀ~οῡ", _, _;
    imp_mid_3: "ἀ~εισθω", "ἀ~εισθων", _;
    imperfect_act_1: "ἠ~ουν", _, "ἠ~οῡμεν";
    imperfect_act_2: "ἠ~εις", "ἠ~εῑτον", "ἠ~εῑτε";
    imperfect_act_3: "ἠ~ει", "ἠ~ειτην", "ἠ~ουν";
    imperfect_mid_1: "ἠ~ουμην", _, "ἠ~ουμεθᾰ";
    imperfect_mid_2: "ἠ~οῡ", "ἠ~εῑσθον", "ἠ~εῑσθε";
    imperfect_mid_3: "ἠ~εῑτο", "ἠ~εισθην", "ἠ~οῡντο";
    future_ind_act_1: "ἀ~ησω", _, "ἀ~ησομεν";
    future_ind_act_2: "ἀ~ησεις", "ἀ~ησετον", "ἀ~ησετε";
    future_ind_act_3: "ἀ~ησει", _, "ἀ~ησουσῐ|ἀ~ησουσῐν";
    future_ind_mid_1: "ἀ~ησομαι", _, "ἀ~ησομεθᾰ";
    future_ind_mid_2: "ἀ~ησῃ|ἀ~ησει", "ἀ~ησεσθον", "ἀ~ησεσθε";
    future_ind_mid_3: "ἀ~ησεται", _, "ἀ~ησονται";
    future_ind_pass_1: "ἀ~ηθησομαι", _, "ἀ~ηθησομεθᾰ";
    future_ind_pass_2: "ἀ~ηθησῃ|ἀ~ηθησει", "ἀ~ηθησεσθον", "ἀ~ηθησεσθε";
    future_ind_pass_3: "ἀ~ηθησεται", _, "ἀ~ηθησονται";
    future_opt_act_1: "ἀ~ησοιμῐ", _, "ἀ~ησοιμεν";
    future_opt_act_2: "ἀ~ησοις", "ἀ~ησοιτον", "ἀ~ησοιτε";
    future_opt_act_3: "ἀ~ησοι", "ἀ~ησοιτην", "ἀ~ησοιεν";
    future_opt_mid_1: "ἀ~ησοιμην", _, "ἀ~ησοιμεθᾰ";
    future_opt_mid_2: "ἀ~ησοιο", "ἀ~ησοισθον", "ἀ~ησοισθε";
    future_opt_mid_3: "ἀ~ησοιτο", "ἀ~ησοισθην", "ἀ~ησοιντο";
    future_opt_pass_1: "ἀ~ηθησοιμην", _, "ἀ~ηθησοιμεθᾰ";
    future_opt_pass_2: "ἀ~ηθησοιο", "ἀ~ηθησοισθον", "ἀ~ηθησοισθε";
    future_opt_pass_3: "ἀ~ηθησοιτο", "ἀ~ηθησοισθην", "ἀ~ηθησοιντο";
    aorist_subj_act_1: "ἀ~ησω", _, "ἀ~ησωμεν";
    aorist_subj_act_2: "ἀ~ησῃς", "ἀ~ησητον", "ἀ~ησητε";
    aorist_subj_act_3: "ἀ~ησῃ", _, "ἀ~ησωσῐ|ἀ~ησωσῐν";
    aorist_subj_mid_1: "ἀ~ησωμαι", _, "ἀ~ησωμεθᾰ";
    aorist_subj_mid_2: _, "ἀ~ησησθον", "ἀ~ησησθε";
    aorist_subj_mid_3: "ἀ~ησηται", _, "ἀ~ησωνται";
    aorist_subj_pass_1: "ἀ~ηθω", _, "ἀ~ηθωμεν";
    aorist_subj_pass_2: "ἀ~ηθῃς", "ἀ~ηθητον", "ἀ~ηθητε";
    aorist_subj_pass_3: "ἀ~ηθῃ", _, "ἀ~ηθωσῐ|ἀ~ηθωσῐν";
    aorist_opt_act_1: "ἀ~ησαιμῐ", _, "ἀ~ησαιμεν";
    aorist_opt_act_2: "ἀ~ησειᾰς|ἀ~ησαις", "ἀ~ησαιτον", "ἀ~ησαιτε";
    aorist_opt_act_3: "ἀ~ησειε|ἀ~ησειεν|ἀ~ησαι", "ἀ~ησαιτην", "ἀ~ησειᾰν|ἀ~ησαιεν";
    aorist_opt_mid_1: "ἀ~ησαιμην", _, "ἀ~ησαιμεθᾰ";
    aorist_opt_mid_2: "ἀ~ησαιο", "ἀ~ησαισθον", "ἀ~ησαισθε";
    aorist_opt_mid_3: "ἀ~ησαιτο", "ἀ~ησαισθην", "ἀ~ησαιντο";
    aorist_opt_pass_1: "ἀ~ηθειην", _, "ἀ~ηθεῑμεν|ἀ~ηθειημεν";
    aorist_opt_pass_2: "ἀ~ηθειης", "ἀ~ηθεῑτον|ἀ~ηθειητον", "ἀ~ηθεῑτε|ἀ~ηθειητε";
    aorist_opt_pass_3: "ἀ~ηθειη", "ἀ~ηθειτην|ἀ~ηθειητην", "ἀ~ηθεῑεν|ἀ~ηθειησᾰν";
    aorist_imp_act_2: "ἀ~ησον", "ἀ~ησᾰτον", "ἀ~ησᾰτε";
    aorist_imp_act_3: "ἀ~ησᾰτω", "ἀ~ησᾰτων", "ἀ~ησᾰντων";
    aorist_imp_mid_2: "ἀ~ησαι", "ἀ~ησᾰσθον", "ἀ~ησᾰσθε";
    aorist_imp_mid_3: "ἀ~ησᾰσθω", "ἀ~ησᾰσθων", _;
    aorist_imp_pass_2: "ἀ~ηθητῐ", _, _;
    aorist_imp_pass_3: "ἀ~ηθητω", "ἀ~ηθητων", "ἀ~ηθεντων";
    perfect_ind_act_3: "ἠ~ηκε|ἠ~ηκεν", _, "ἠ~ηκᾱσῐ|ἠ~ηκᾱσῐν";
    perfect_subj_act_3: _, _, "ἠ~ηκωσῐ|ἠ~ηκωσῐν";
    perfect_opt_act_1: "ἠ~ηκοιμῐ|ἠ~ηκοιην", _, _;
    perfect_opt_act_2: "ἠ~ηκοις|ἠ~ηκοιης", _, _;
    perfect_opt_act_3: "ἠ~ηκοι|ἠ~ηκοιη", _, _;
    perfect_imp_act_2: "ἠ~ηκε", _, _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    infinitive_pres: "ἀ~εῑν", "ἀ~εῑσθαι", _;
    infinitive_fut: "ἀ~ησειν", "ἀ~ησεσθαι", "ἀ~ηθησεσθαι";
    infinitive_aor: "ἀ~ησαι", "ἀ~ησᾰσθαι", "ἀ~ηθηναι";
    participle_pres: "ἀ~ων", "ἀ~ουμενος", _;
    participle_fut: "ἀ~ησων", "ἀ~ησομενος", "ἀ~ηθησομενος";
    participle_aor: "ἀ~ησᾱς", "ἀ~ησᾰμενος", "ἀ~ηθεις";
    ),
    // Επῐβουλευω: 1 words (επῐβουλευω). Like Πεμπω, except in 236 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("επῐβουλευω", 1, "ε~ῐ~υω", like Πεμπω;
    stem aorist_ind: "~ε~υσ- like επῐβουλευω";
    stem aorist_ind_pass: "~ε~υθη- like επῐβουλευω";
    stem pluperfect_act: "~ε%ε~υκ- like επῐβουλευω";
    ind_act_1: "ἑ~ῐ~υω", _, "ἑ~ῐ~υομεν";
    ind_act_2: "ἑ~ῐ~υεις", "ἑ~ῐ~υετον", "ἑ~ῐ~υετε";
    ind_act_3: "ἑ~ῐ~υει", _, "ἑ~ῐ~υουσῐ|ἑ~ῐ~υουσῐν";
    ind_mid_1: "ἑ~ῐ~υομαι", _, "ἑ~ῐ~υομεθᾰ";
    ind_mid_2: "ἑ~ῐ~υῃ|ἑ~ῐ~υει", "ἑ~ῐ~υεσθον", "ἑ~ῐ~υεσθε";
    ind_mid_3: "ἑ~ῐ~υεται", _, "ἑ~ῐ~υονται";
    subj_act_1: "ἑ~ῐ~υω", _, "ἑ~ῐ~υωμεν";
    subj_act_2: "ἑ~ῐ~υῃς", "ἑ~ῐ~υητον", "ἑ~ῐ~υητε";
    subj_act_3: "ἑ~ῐ~υῃ", _, "ἑ~ῐ~υωσῐ|ἑ~ῐ~υωσῐν";
    subj_mid_1: "ἑ~ῐ~υωμαι", _, "ἑ~ῐ~υωμεθᾰ";
    subj_mid_2: _, "ἑ~ῐ~υησθον", "ἑ~ῐ~υησθε";
    subj_mid_3: "ἑ~ῐ~υηται", _, "ἑ~ῐ~υωνται";
    opt_act_1: "ἑ~ῐ~υοιμῐ", _, "ἑ~ῐ~υοιμεν";
    opt_act_2: "ἑ~ῐ~υοις", "ἑ~ῐ~υοιτον", "ἑ~ῐ~υοιτε";
    opt_act_3: "ἑ~ῐ~υοι", "ἑ~ῐ~υοιτην", "ἑ~ῐ~υοιεν";
    opt_mid_1: "ἑ~ῐ~υοιμην", _, "ἑ~ῐ~υοιμεθᾰ";
    opt_mid_2: "ἑ~ῐ~υοιο", "ἑ~ῐ~υοισθον", "ἑ~ῐ~υοισθε";
    opt_mid_3: "ἑ~ῐ~υοιτο", "ἑ~ῐ~υοισθην", "ἑ~ῐ~υοιντο";
    imp_act_2: "ἑ~ῐ~υε", _, _;
    imp_act_3: "ἑ~ῐ~υετω", "ἑ~ῐ~υετων", "ἑ~ῐ~υοντων";
    imp_mid_2: "ἑ~ῐ~υου", _, _;
    imp_mid_3: "ἑ~ῐ~υεσθω", "ἑ~ῐ~υεσθων", _;
    imperfect_act_1: "ἑ~ε~υον", _, "ἑ~ε~υομεν";
    imperfect_act_2: "ἑ~ε~υες", "ἑ~ε~υετον", "ἑ~ε~υετε";
    imperfect_act_3: "ἑ~ε~υε|ἑ~ε~υεν", "ἑ~ε~υετην", "ἑ~ε~υον";
    imperfect_mid_1: "ἑ~ε~υομην", _, "ἑ~ε~υομεθᾰ";
    imperfect_mid_2: "ἑ~ε~υου", "ἑ~ε~υεσθον", "ἑ~ε~υεσθε";
    imperfect_mid_3: "ἑ~ε~υετο", "ἑ~ε~υεσθην", "ἑ~ε~υοντο";
    future_ind_act_1: "ἑ~ῐ~υσω", _, "ἑ~ῐ~υσομεν";
    future_ind_act_2: "ἑ~ῐ~υσεις", "ἑ~ῐ~υσετον", "ἑ~ῐ~υσετε";
    future_ind_act_3: "ἑ~ῐ~υσει", _, "ἑ~ῐ~υσουσῐ|ἑ~ῐ~υσουσῐν";
    future_ind_mid_1: "ἑ~ῐ~υσομαι", _, "ἑ~ῐ~υσομεθᾰ";
    future_ind_mid_2: "ἑ~ῐ~υσῃ|ἑ~ῐ~υσει", "ἑ~ῐ~υσεσθον", "ἑ~ῐ~υσεσθε";
    future_ind_mid_3: "ἑ~ῐ~υσεται", _, "ἑ~ῐ~υσονται";
    future_ind_pass_1: "ἑ~ῐ~υθησομαι", _, "ἑ~ῐ~υθησομεθᾰ";
    future_ind_pass_2: "ἑ~ῐ~υθησῃ|ἑ~ῐ~υθησει", "ἑ~ῐ~υθησεσθον", "ἑ~ῐ~υθησεσθε";
    future_ind_pass_3: "ἑ~ῐ~υθησεται", _, "ἑ~ῐ~υθησονται";
    future_opt_act_1: "ἑ~ῐ~υσοιμῐ", _, "ἑ~ῐ~υσοιμεν";
    future_opt_act_2: "ἑ~ῐ~υσοις", "ἑ~ῐ~υσοιτον", "ἑ~ῐ~υσοιτε";
    future_opt_act_3: "ἑ~ῐ~υσοι", "ἑ~ῐ~υσοιτην", "ἑ~ῐ~υσοιεν";
    future_opt_mid_1: "ἑ~ῐ~υσοιμην", _, "ἑ~ῐ~υσοιμεθᾰ";
    future_opt_mid_2: "ἑ~ῐ~υσοιο", "ἑ~ῐ~υσοισθον", "ἑ~ῐ~υσοισθε";
    future_opt_mid_3: "ἑ~ῐ~υσοιτο", "ἑ~ῐ~υσοισθην", "ἑ~ῐ~υσοιντο";
    future_opt_pass_1: "ἑ~ῐ~υθησοιμην", _, "ἑ~ῐ~υθησοιμεθᾰ";
    future_opt_pass_2: "ἑ~ῐ~υθησοιο", "ἑ~ῐ~υθησοισθον", "ἑ~ῐ~υθησοισθε";
    future_opt_pass_3: "ἑ~ῐ~υθησοιτο", "ἑ~ῐ~υθησοισθην", "ἑ~ῐ~υθησοιντο";
    aorist_subj_act_1: "ἑ~ῐ~υσω", _, "ἑ~ῐ~υσωμεν";
    aorist_subj_act_2: "ἑ~ῐ~υσῃς", "ἑ~ῐ~υσητον", "ἑ~ῐ~υσητε";
    aorist_subj_act_3: "ἑ~ῐ~υσῃ", _, "ἑ~ῐ~υσωσῐ|ἑ~ῐ~υσωσῐν";
    aorist_subj_mid_1: "ἑ~ῐ~υσωμαι", _, "ἑ~ῐ~υσωμεθᾰ";
    aorist_subj_mid_2: _, "ἑ~ῐ~υσησθον", "ἑ~ῐ~υσησθε";
    aorist_subj_mid_3: "ἑ~ῐ~υσηται", _, "ἑ~ῐ~υσωνται";
    aorist_subj_pass_1: "ἑ~ῐ~υθω", _, "ἑ~ῐ~υθωμεν";
    aorist_subj_pass_2: "ἑ~ῐ~υθῃς", "ἑ~ῐ~υθητον", "ἑ~ῐ~υθητε";
    aorist_subj_pass_3: "ἑ~ῐ~υθῃ", _, "ἑ~ῐ~υθωσῐ|ἑ~ῐ~υθωσῐν";
    aorist_opt_act_1: "ἑ~ῐ~υσαιμῐ", _, "ἑ~ῐ~υσαιμεν";
    aorist_opt_act_2: "ἑ~ῐ~υσειᾰς|ἑ~ῐ~υσαις", "ἑ~ῐ~υσαιτον", "ἑ~ῐ~υσαιτε";
    aorist_opt_act_3: "ἑ~ῐ~υσειε|ἑ~ῐ~υσειεν|ἑ~ῐ~υσαι", "ἑ~ῐ~υσαιτην", "ἑ~ῐ~υσειᾰν|ἑ~ῐ~υσαιεν";
    aorist_opt_mid_1: "ἑ~ῐ~υσαιμην", _, "ἑ~ῐ~υσαιμεθᾰ";
    aorist_opt_mid_2: "ἑ~ῐ~υσαιο", "ἑ~ῐ~υσαισθον", "ἑ~ῐ~υσαισθε";
    aorist_opt_mid_3: "ἑ~ῐ~υσαιτο", "ἑ~ῐ~υσαισθην", "ἑ~ῐ~υσαιντο";
    aorist_opt_pass_1: "ἑ~ῐ~υθειην", _, "ἑ~ῐ~υθεῑμεν|ἑ~ῐ~υθειημεν";
    aorist_opt_pass_2: "ἑ~ῐ~υθειης", "ἑ~ῐ~υθεῑτον|ἑ~ῐ~υθειητον", "ἑ~ῐ~υθεῑτε|ἑ~ῐ~υθειητε";
    aorist_opt_pass_3: "ἑ~ῐ~υθειη", "ἑ~ῐ~υθειτην|ἑ~ῐ~υθειητην", "ἑ~ῐ~υθεῑεν|ἑ~ῐ~υθειησᾰν";
    aorist_imp_act_2: "ἑ~ῐ~υσον", "ἑ~ῐ~υσᾰτον", "ἑ~ῐ~υσᾰτε";
    aorist_imp_act_3: "ἑ~ῐ~υσᾰτω", "ἑ~ῐ~υσᾰτων", "ἑ~ῐ~υσᾰντων";
    aorist_imp_mid_2: "ἑ~ῐ~υσαι", "ἑ~ῐ~υσᾰσθον", "ἑ~ῐ~υσᾰσθε";
    aorist_imp_mid_3: "ἑ~ῐ~υσᾰσθω", "ἑ~ῐ~υσᾰσθων", _;
    aorist_imp_pass_2: "ἑ~ῐ~υθητῐ", _, _;
    aorist_imp_pass_3: "ἑ~ῐ~υθητω", "ἑ~ῐ~υθητων", "ἑ~ῐ~υθεντων";
    perfect_ind_act_1: "ἑ~ῐ%ε~υκᾰ", _, "ἑ~ῐ%ε~υκᾰμεν";
    perfect_ind_act_2: "ἑ~ῐ%ε~υκᾰς", "ἑ~ῐ%ε~υκᾰτον", "ἑ~ῐ%ε~υκᾰτε";
    perfect_ind_act_3: "ἑ~ῐ%ε~υκε|ἑ~ῐ%ε~υκεν", _, "ἑ~ῐ%ε~υκᾱσῐ|ἑ~ῐ%ε~υκᾱσῐν";
    perfect_ind_mid_1: "ἑ~ῐ%ε~υμαι", _, "ἑ~ῐ%ε~υμεθᾰ";
    perfect_ind_mid_2: "ἑ~ῐ%ε~υσαι", "ἑ~ῐ%ε~υσθον", "ἑ~ῐ%ε~υσθε";
    perfect_ind_mid_3: "ἑ~ῐ%ε~υται", _, "ἑ~ῐ%ε~υνται";
    perfect_subj_act_1: "ἑ~ῐ%ε~υκω", _, "ἑ~ῐ%ε~υκωμεν";
    perfect_subj_act_2: "ἑ~ῐ%ε~υκῃς", "ἑ~ῐ%ε~υκητον", "ἑ~ῐ%ε~υκητε";
    perfect_subj_act_3: "ἑ~ῐ%ε~υκῃ", _, "ἑ~ῐ%ε~υκωσῐ|ἑ~ῐ%ε~υκωσῐν";
    perfect_opt_act_1: "ἑ~ῐ%ε~υκοιμῐ|ἑ~ῐ%ε~υκοιην", _, "ἑ~ῐ%ε~υκοιμεν";
    perfect_opt_act_2: "ἑ~ῐ%ε~υκοις|ἑ~ῐ%ε~υκοιης", "ἑ~ῐ%ε~υκοιτον", "ἑ~ῐ%ε~υκοιτε";
    perfect_opt_act_3: "ἑ~ῐ%ε~υκοι|ἑ~ῐ%ε~υκοιη", "ἑ~ῐ%ε~υκοιτην", "ἑ~ῐ%ε~υκοιεν";
    perfect_imp_act_2: "ἑ~ῐ%ε~υκε", "ἑ~ῐ%ε~υκετον", "ἑ~ῐ%ε~υκετε";
    perfect_imp_act_3: "ἑ~ῐ%ε~υκετω", "ἑ~ῐ%ε~υκετων", "ἑ~ῐ%ε~υκοντων";
    perfect_imp_mid_2: "ἑ~ῐ%ε~υσο", _, _;
    perfect_imp_mid_3: "ἑ~ῐ%ε~υσθω", "ἑ~ῐ%ε~υσθων", _;
    pluperfect_mid_1: "ἑ~ε%ε~υμην", _, "ἑ~ε%ε~υμεθᾰ";
    pluperfect_mid_2: "ἑ~ε%ε~υσο", "ἑ~ε%ε~υσθον", "ἑ~ε%ε~υσθε";
    pluperfect_mid_3: "ἑ~ε%ε~υτο", "ἑ~ε%ε~υσθην", "ἑ~ε%ε~υντο";
    infinitive_pres: "ἑ~ῐ~υειν", "ἑ~ῐ~υεσθαι", _;
    infinitive_fut: "ἑ~ῐ~υσειν", "ἑ~ῐ~υσεσθαι", "ἑ~ῐ~υθησεσθαι";
    infinitive_aor: "ἑ~ῐ~ῡσαι", "ἑ~ῐ~υσᾰσθαι", "ἑ~ῐ~υθηναι";
    infinitive_perf: "ἑ~ῐ%ε~υκεναι", "ἑ~ῐ%ε~ῡσθαι", _;
    participle_pres: "ἑ~ῐ~υων", "ἑ~ῐ~υομενος", _;
    participle_fut: "ἑ~ῐ~υσων", "ἑ~ῐ~υσομενος", "ἑ~ῐ~υθησομενος";
    participle_aor: "ἑ~ῐ~υσᾱς", "ἑ~ῐ~υσᾰμενος", "ἑ~ῐ~υθεις";
    participle_perf: "ἑ~ῐ%ε~υκως", "ἑ~ῐ%ε~υμενος", _;
    ),
    // Εισκᾰτοικεω: 1 words (εισκᾰτοικεω). Like Πεμπω, except in 236 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("εισκᾰτοικεω", 1, "ε~οι~εω", like Πεμπω;
    stem aorist_ind: "~ῳ~ησ- like επῐβουλευω";
    stem aorist_ind_pass: "~ῳ~ηθη- like επῐβουλευω";
    stem pluperfect_act: "~ῳ~ηκ- like επῐβουλευω";
    ind_act_1: "ἑ~οι~ω", _, "ἑ~οι~οῡμεν";
    ind_act_2: "ἑ~οι~εῑς", "ἑ~οι~εῑτον", "ἑ~οι~εῑτε";
    ind_act_3: "ἑ~οι~εῑ", _, "ἑ~οι~οῡσῐ|ἑ~οι~οῡσῐν";
    ind_mid_1: "ἑ~οι~οῡμαι", _, "ἑ~οι~ουμεθᾰ";
    ind_mid_2: "ἑ~οι~εῑ|ἑ~οι~ῃ", "ἑ~οι~εῑσθον", "ἑ~οι~εῑσθε";
    ind_mid_3: "ἑ~οι~εῑται", _, "ἑ~οι~οῡνται";
    subj_act_1: "ἑ~οι~ω", _, "ἑ~οι~ωμεν";
    subj_act_2: "ἑ~οι~ῃς", "ἑ~οι~ητον", "ἑ~οι~ητε";
    subj_act_3: "ἑ~οι~ῃ", _, "ἑ~οι~ωσῐ|ἑ~οι~ωσῐν";
    subj_mid_1: "ἑ~οι~ωμαι", _, "ἑ~οι~ωμεθᾰ";
    subj_mid_2: _, "ἑ~οι~ησθον", "ἑ~οι~ησθε";
    subj_mid_3: "ἑ~οι~ηται", _, "ἑ~οι~ωνται";
    opt_act_1: "ἑ~οι~οιην|ἑ~οι~οῑμῐ", _, "ἑ~οι~οῑμεν|ἑ~οι~οιημεν";
    opt_act_2: "ἑ~οι~οιης|ἑ~οι~οῑς", "ἑ~οι~οῑτον|ἑ~οι~οιητον", "ἑ~οι~οῑτε|ἑ~οι~οιητε";
    opt_act_3: "ἑ~οι~οιη|ἑ~οι~οῑ", "ἑ~οι~οιτην|ἑ~οι~οιητην", "ἑ~οι~οῑεν|ἑ~οι~οιησᾰν";
    opt_mid_1: "ἑ~οι~οιμην", _, "ἑ~οι~οιμεθᾰ";
    opt_mid_2: "ἑ~οι~οῑο", "ἑ~οι~οῑσθον", "ἑ~οι~οῑσθε";
    opt_mid_3: "ἑ~οι~οῑτο", "ἑ~οι~οισθην", "ἑ~οι~οῑντο";
    imp_act_2: "ἑ~οι~ει", _, _;
    imp_act_3: "ἑ~οι~ειτω", "ἑ~οι~ειτων", "ἑ~οι~ουντων";
    imp_mid_2: "ἑ~οι~οῡ", _, _;
    imp_mid_3: "ἑ~οι~εισθω", "ἑ~οι~εισθων", _;
    imperfect_act_1: "ἑ~ῳ~ουν", _, "ἑ~ῳ~οῡμεν";
    imperfect_act_2: "ἑ~ῳ~εις", "ἑ~ῳ~εῑτον", "ἑ~ῳ~εῑτε";
    imperfect_act_3: "ἑ~ῳ~ει", "ἑ~ῳ~ειτην", "ἑ~ῳ~ουν";
    imperfect_mid_1: "ἑ~ῳ~ουμην", _, "ἑ~ῳ~ουμεθᾰ";
    imperfect_mid_2: "ἑ~ῳ~οῡ", "ἑ~ῳ~εῑσθον", "ἑ~ῳ~εῑσθε";
    imperfect_mid_3: "ἑ~ῳ~εῑτο", "ἑ~ῳ~εισθην", "ἑ~ῳ~οῡντο";
    future_ind_act_1: "ἑ~οι~ησω", _, "ἑ~οι~ησομεν";
    future_ind_act_2: "ἑ~οι~ησεις", "ἑ~οι~ησετον", "ἑ~οι~ησετε";
    future_ind_act_3: "ἑ~οι~ησει", _, "ἑ~οι~ησουσῐ|ἑ~οι~ησουσῐν";
    future_ind_mid_1: "ἑ~οι~ησομαι", _, "ἑ~οι~ησομεθᾰ";
    future_ind_mid_2: "ἑ~οι~ησῃ|ἑ~οι~ησει", "ἑ~οι~ησεσθον", "ἑ~οι~ησεσθε";
    future_ind_mid_3: "ἑ~οι~ησεται", _, "ἑ~οι~ησονται";
    future_ind_pass_1: "ἑ~οι~ηθησομαι", _, "ἑ~οι~ηθησομεθᾰ";
    future_ind_pass_2: "ἑ~οι~ηθησῃ|ἑ~οι~ηθησει", "ἑ~οι~ηθησεσθον", "ἑ~οι~ηθησεσθε";
    future_ind_pass_3: "ἑ~οι~ηθησεται", _, "ἑ~οι~ηθησονται";
    future_opt_act_1: "ἑ~οι~ησοιμῐ", _, "ἑ~οι~ησοιμεν";
    future_opt_act_2: "ἑ~οι~ησοις", "ἑ~οι~ησοιτον", "ἑ~οι~ησοιτε";
    future_opt_act_3: "ἑ~οι~ησοι", "ἑ~οι~ησοιτην", "ἑ~οι~ησοιεν";
    future_opt_mid_1: "ἑ~οι~ησοιμην", _, "ἑ~οι~ησοιμεθᾰ";
    future_opt_mid_2: "ἑ~οι~ησοιο", "ἑ~οι~ησοισθον", "ἑ~οι~ησοισθε";
    future_opt_mid_3: "ἑ~οι~ησοιτο", "ἑ~οι~ησοισθην", "ἑ~οι~ησοιντο";
    future_opt_pass_1: "ἑ~οι~ηθησοιμην", _, "ἑ~οι~ηθησοιμεθᾰ";
    future_opt_pass_2: "ἑ~οι~ηθησοιο", "ἑ~οι~ηθησοισθον", "ἑ~οι~ηθησοισθε";
    future_opt_pass_3: "ἑ~οι~ηθησοιτο", "ἑ~οι~ηθησοισθην", "ἑ~οι~ηθησοιντο";
    aorist_subj_act_1: "ἑ~οι~ησω", _, "ἑ~οι~ησωμεν";
    aorist_subj_act_2: "ἑ~οι~ησῃς", "ἑ~οι~ησητον", "ἑ~οι~ησητε";
    aorist_subj_act_3: "ἑ~οι~ησῃ", _, "ἑ~οι~ησωσῐ|ἑ~οι~ησωσῐν";
    aorist_subj_mid_1: "ἑ~οι~ησωμαι", _, "ἑ~οι~ησωμεθᾰ";
    aorist_subj_mid_2: _, "ἑ~οι~ησησθον", "ἑ~οι~ησησθε";
    aorist_subj_mid_3: "ἑ~οι~ησηται", _, "ἑ~οι~ησωνται";
    aorist_subj_pass_1: "ἑ~οι~ηθω", _, "ἑ~οι~ηθωμεν";
    aorist_subj_pass_2: "ἑ~οι~ηθῃς", "ἑ~οι~ηθητον", "ἑ~οι~ηθητε";
    aorist_subj_pass_3: "ἑ~οι~ηθῃ", _, "ἑ~οι~ηθωσῐ|ἑ~οι~ηθωσῐν";
    aorist_opt_act_1: "ἑ~οι~ησαιμῐ", _, "ἑ~οι~ησαιμεν";
    aorist_opt_act_2: "ἑ~οι~ησειᾰς|ἑ~οι~ησαις", "ἑ~οι~ησαιτον", "ἑ~οι~ησαιτε";
    aorist_opt_act_3: "ἑ~οι~ησειε|ἑ~οι~ησειεν|ἑ~οι~ησαι", "ἑ~οι~ησαιτην", "ἑ~οι~ησειᾰν|ἑ~οι~ησαιεν";
    aorist_opt_mid_1: "ἑ~οι~ησαιμην", _, "ἑ~οι~ησαιμεθᾰ";
    aorist_opt_mid_2: "ἑ~οι~ησαιο", "ἑ~οι~ησαισθον", "ἑ~οι~ησαισθε";
    aorist_opt_mid_3: "ἑ~οι~ησαιτο", "ἑ~οι~ησαισθην", "ἑ~οι~ησαιντο";
    aorist_opt_pass_1: "ἑ~οι~ηθειην", _, "ἑ~οι~ηθεῑμεν|ἑ~οι~ηθειημεν";
    aorist_opt_pass_2: "ἑ~οι~ηθειης", "ἑ~οι~ηθεῑτον|ἑ~οι~ηθειητον", "ἑ~οι~ηθεῑτε|ἑ~οι~ηθειητε";
    aorist_opt_pass_3: "ἑ~οι~ηθειη", "ἑ~οι~ηθειτην|ἑ~οι~ηθειητην", "ἑ~οι~ηθεῑεν|ἑ~οι~ηθειησᾰν";
    aorist_imp_act_2: "ἑ~οι~ησον", "ἑ~οι~ησᾰτον", "ἑ~οι~ησᾰτε";
    aorist_imp_act_3: "ἑ~οι~ησᾰτω", "ἑ~οι~ησᾰτων", "ἑ~οι~ησᾰντων";
    aorist_imp_mid_2: "ἑ~οι~ησαι", "ἑ~οι~ησᾰσθον", "ἑ~οι~ησᾰσθε";
    aorist_imp_mid_3: "ἑ~οι~ησᾰσθω", "ἑ~οι~ησᾰσθων", _;
    aorist_imp_pass_2: "ἑ~οι~ηθητῐ", _, _;
    aorist_imp_pass_3: "ἑ~οι~ηθητω", "ἑ~οι~ηθητων", "ἑ~οι~ηθεντων";
    perfect_ind_act_1: "ἑ~ῳ~ηκᾰ", _, "ἑ~ῳ~ηκᾰμεν";
    perfect_ind_act_2: "ἑ~ῳ~ηκᾰς", "ἑ~ῳ~ηκᾰτον", "ἑ~ῳ~ηκᾰτε";
    perfect_ind_act_3: "ἑ~ῳ~ηκε|ἑ~ῳ~ηκεν", _, "ἑ~ῳ~ηκᾱσῐ|ἑ~ῳ~ηκᾱσῐν";
    perfect_ind_mid_1: "ἑ~ῳ~ημαι", _, "ἑ~ῳ~ημεθᾰ";
    perfect_ind_mid_2: "ἑ~ῳ~ησαι", "ἑ~ῳ~ησθον", "ἑ~ῳ~ησθε";
    perfect_ind_mid_3: "ἑ~ῳ~ηται", _, "ἑ~ῳ~ηνται";
    perfect_subj_act_1: "ἑ~ῳ~ηκω", _, "ἑ~ῳ~ηκωμεν";
    perfect_subj_act_2: "ἑ~ῳ~ηκῃς", "ἑ~ῳ~ηκητον", "ἑ~ῳ~ηκητε";
    perfect_subj_act_3: "ἑ~ῳ~ηκῃ", _, "ἑ~ῳ~ηκωσῐ|ἑ~ῳ~ηκωσῐν";
    perfect_opt_act_1: "ἑ~ῳ~ηκοιμῐ|ἑ~ῳ~ηκοιην", _, "ἑ~ῳ~ηκοιμεν";
    perfect_opt_act_2: "ἑ~ῳ~ηκοις|ἑ~ῳ~ηκοιης", "ἑ~ῳ~ηκοιτον", "ἑ~ῳ~ηκοιτε";
    perfect_opt_act_3: "ἑ~ῳ~ηκοι|ἑ~ῳ~ηκοιη", "ἑ~ῳ~ηκοιτην", "ἑ~ῳ~ηκοιεν";
    perfect_imp_act_2: "ἑ~ῳ~ηκε", "ἑ~ῳ~ηκετον", "ἑ~ῳ~ηκετε";
    perfect_imp_act_3: "ἑ~ῳ~ηκετω", "ἑ~ῳ~ηκετων", "ἑ~ῳ~ηκοντων";
    perfect_imp_mid_2: "ἑ~ῳ~ησο", _, _;
    perfect_imp_mid_3: "ἑ~ῳ~ησθω", "ἑ~ῳ~ησθων", _;
    pluperfect_mid_1: "ἑ~ῳ~ημην", _, "ἑ~ῳ~ημεθᾰ";
    pluperfect_mid_2: "ἑ~ῳ~ησο", "ἑ~ῳ~ησθον", "ἑ~ῳ~ησθε";
    pluperfect_mid_3: "ἑ~ῳ~ητο", "ἑ~ῳ~ησθην", "ἑ~ῳ~ηντο";
    infinitive_pres: "ἑ~οι~εῑν", "ἑ~οι~εῑσθαι", _;
    infinitive_fut: "ἑ~οι~ησειν", "ἑ~οι~ησεσθαι", "ἑ~οι~ηθησεσθαι";
    infinitive_aor: "ἑ~οι~ησαι", "ἑ~οι~ησᾰσθαι", "ἑ~οι~ηθηναι";
    infinitive_perf: "ἑ~ῳ~ηκεναι", "ἑ~ῳ~ησθαι", _;
    participle_pres: "ἑ~οι~ων", "ἑ~οι~ουμενος", _;
    participle_fut: "ἑ~οι~ησων", "ἑ~οι~ησομενος", "ἑ~οι~ηθησομενος";
    participle_aor: "ἑ~οι~ησᾱς", "ἑ~οι~ησᾰμενος", "ἑ~οι~ηθεις";
    participle_perf: "ἑ~ῳ~ηκως", "ἑ~ῳ~ημενος", _;
    ),
    // Δειδῐᾰ: 1 words (δειδῐᾰ). Like Μετεκῑᾰθον, except in 75 cells: ind_act_1 sg, aorist_subj_act_1 sg, aorist_subj_act_1 pl, aorist_subj_act_2 sg, aorist_subj_act_2 du, aorist_subj_act_2 pl, ….
    conjugation!("δειδῐᾰ", 1, "~~δῐᾰ", like Μετεκῑᾰθον;
    stem aorist_ind: "~~σ- like ποιεω";
    ind_act_1: "~~δῐᾰ", _, _;
    aorist_subj_act_1: "~~σω", _, "~~σωμεν";
    aorist_subj_act_2: "~~σῃς", "~~σητον", "~~σητε";
    aorist_subj_act_3: "~~σῃ", _, "~~σωσῐ|~~σωσῐν";
    aorist_subj_mid_1: "~~σωμαι", _, "~~σωμεθᾰ";
    aorist_subj_mid_2: _, "~~σησθον", "~~σησθε";
    aorist_subj_mid_3: "~~σηται", _, "~~σωνται";
    aorist_opt_act_1: "~~σαιμῐ", _, "~~σαιμεν";
    aorist_opt_act_2: "~~σειᾰς|~~σαις", "~~σαιτον", "~~σαιτε";
    aorist_opt_act_3: "~~σειε|~~σειεν|~~σαι", "~~σαιτην", "~~σειᾰν|~~σαιεν";
    aorist_opt_mid_1: "~~σαιμην", _, "~~σαιμεθᾰ";
    aorist_opt_mid_2: "~~σαιο", "~~σαισθον", "~~σαισθε";
    aorist_opt_mid_3: "~~σαιτο", "~~σαισθην", "~~σαιντο";
    aorist_imp_act_2: "~~σον", "~~σᾰτον", "~~σᾰτε";
    aorist_imp_act_3: "~~σᾰτω", "~~σᾰτων", "~~σᾰντων";
    aorist_imp_mid_2: "~~σαι", "~~σᾰσθον", "~~σᾰσθε";
    aorist_imp_mid_3: "~~σᾰσθω", "~~σᾰσθων", _;
    perfect_ind_act_1: "~δο~κᾰ", _, "~δο~κᾰμεν";
    perfect_ind_act_2: "~δο~κᾰς", "~δο~κᾰτον", "~δο~κᾰτε";
    perfect_ind_act_3: "~δο~κε|~δο~κεν", _, "~δο~κᾱσῐ|~δο~κᾱσῐν";
    perfect_subj_act_1: "~δο~κω", _, "~δο~κωμεν";
    perfect_subj_act_2: "~δο~κῃς", "~δο~κητον", "~δο~κητε";
    perfect_subj_act_3: "~δο~κῃ", _, "~δο~κωσῐ|~δο~κωσῐν";
    perfect_opt_act_1: "~δο~κοιμῐ|~δο~κοιην", _, "~δο~κοιμεν";
    perfect_opt_act_2: "~δο~κοις|~δο~κοιης", "~δο~κοιτον", "~δο~κοιτε";
    perfect_opt_act_3: "~δο~κοι|~δο~κοιη", "~δο~κοιτην", "~δο~κοιεν";
    perfect_imp_act_2: "~δο~κε", "~δο~κετον", "~δο~κετε";
    perfect_imp_act_3: "~δο~κετω", "~δο~κετων", "~δο~κοντων";
    infinitive_aor: "~ῑσα~", "~~σᾰσθαι", _;
    infinitive_perf: "~δο~κεναι", _, _;
    participle_aor: "~~σᾱς", "~~σᾰμενος", _;
    participle_perf: "~δο~κως", _, _;
    ),
    // Λανθανομαι: 1 words (λανθανομαι). Like Μᾰστῐγοω, except in 54 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("λανθανομαι", 1, "~α~α~ομαι", like Μᾰστῐγοω;
    ind_act_1: "~ᾰ~ᾰ~ω", _, "~ᾰ~ᾰ~ομεν";
    ind_act_2: "~ᾰ~ᾰ~εις", "~ᾰ~ᾰ~ετον", "~ᾰ~ᾰ~ετε";
    ind_act_3: "~ᾰ~ᾰ~ει", _, "~ᾰ~ᾰ~ουσῐ|~ᾰ~ᾰ~ουσῐν";
    ind_mid_1: "~ᾰ~ᾰ~ομαι", _, "~ᾰ~ᾰ~ομεθᾰ";
    ind_mid_2: "~ᾰ~ᾰ~ῃ|~ᾰ~ᾰ~ει", "~ᾰ~ᾰ~εσθον", "~ᾰ~ᾰ~εσθε";
    ind_mid_3: "~ᾰ~ᾰ~εται", _, "~ᾰ~ᾰ~ονται";
    subj_act_1: "~ᾰ~ᾰ~ω", _, "~ᾰ~ᾰ~ωμεν";
    subj_act_2: "~ᾰ~ᾰ~ῃς", "~ᾰ~ᾰ~ητον", "~ᾰ~ᾰ~ητε";
    subj_act_3: "~ᾰ~ᾰ~ῃ", _, "~ᾰ~ᾰ~ωσῐ|~ᾰ~ᾰ~ωσῐν";
    subj_mid_1: "~ᾰ~ᾰ~ωμαι", _, "~ᾰ~ᾰ~ωμεθᾰ";
    subj_mid_2: _, "~ᾰ~ᾰ~ησθον", "~ᾰ~ᾰ~ησθε";
    subj_mid_3: "~ᾰ~ᾰ~ηται", _, "~ᾰ~ᾰ~ωνται";
    opt_act_1: "~ᾰ~ᾰ~οιμῐ", _, "~ᾰ~ᾰ~οιμεν";
    opt_act_2: "~ᾰ~ᾰ~οις", "~ᾰ~ᾰ~οιτον", "~ᾰ~ᾰ~οιτε";
    opt_act_3: "~ᾰ~ᾰ~οι", "~ᾰ~ᾰ~οιτην", "~ᾰ~ᾰ~οιεν";
    opt_mid_1: "~ᾰ~ᾰ~οιμην", _, "~ᾰ~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~ᾰ~οιο", "~ᾰ~ᾰ~οισθον", "~ᾰ~ᾰ~οισθε";
    opt_mid_3: "~ᾰ~ᾰ~οιτο", "~ᾰ~ᾰ~οισθην", "~ᾰ~ᾰ~οιντο";
    imp_act_2: "~ᾰ~ᾰ~ε", _, _;
    imp_act_3: "~ᾰ~ᾰ~ετω", "~ᾰ~ᾰ~ετων", "~ᾰ~ᾰ~οντων";
    imp_mid_2: "~ᾰ~ᾰ~ου", _, _;
    imp_mid_3: "~ᾰ~ᾰ~εσθω", "~ᾰ~ᾰ~εσθων", _;
    infinitive_pres: "~ᾰ~ᾰ~ειν", "~ᾰ~ᾰ~εσθαι", _;
    participle_pres: "~ᾰ~ᾰ~ων", "~ᾰ~ᾰ~ομενος", _;
    ),
    // Τρᾰπω: 1 words (τρᾰπω). Like Μῑγνῡμῐ, except in 35 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("τρᾰπω", 1, "~ᾰπω", like Μῑγνῡμῐ as SG_AS_PL;
    ind_act_1: "~ᾰπω", _, _;
    future_ind_act_1: "~αψω", _, "~αψομεν";
    future_ind_act_2: "~αψεις", "~αψετον", "~αψετε";
    future_ind_act_3: "~αψει", _, "~αψουσῐ|~αψουσῐν";
    future_ind_mid_1: "~αψομαι", _, "~αψομεθᾰ";
    future_ind_mid_2: "~αψεαι", "~αψεσθον", "~αψεσθε";
    future_ind_mid_3: "~αψεται", _, "~αψονται";
    future_opt_act_1: "~αψοιμῐ", _, "~αψοιμεν";
    future_opt_act_2: "~αψοις", "~αψοιτον", "~αψοιτε";
    future_opt_act_3: "~αψοι", "~αψοιτην", "~αψοιεν";
    future_opt_mid_1: "~αψοιμην", _, "~αψοιμεθᾰ";
    future_opt_mid_2: "~αψοιο", "~αψοισθον", "~αψοισθε";
    future_opt_mid_3: "~αψοιτο", "~αψοισθην", "~αψοιντο";
    infinitive_fut: "~αψειν", "~αψεσθαι", _;
    participle_fut: "~αψων", "~αψομενος", _;
    ),
];
