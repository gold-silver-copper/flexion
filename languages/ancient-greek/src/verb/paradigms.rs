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
/// they share them), the conjugation it is like (if any), then a row of patterns
/// per line (`derived`: the form comes from another cell's, see
/// `relations.rs`; `_`, or a row left out: the form is the conjugation's it is like,
/// or, in a conjugation like no other, derived):
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
    ($name:literal, $words:literal, $lemma:literal $(, like $parent:ident)?;
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

/// Every conjugation learned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conjugation {
    /// Like `ἐχω`: 457 words.
    Ἐχω,
    /// Like `ποιεω`: 251 words.
    Ποιεω,
    /// Like `ὁρῐζω`: 93 words.
    Ὁρῐζω,
    /// Like `γῡροω`: 87 words.
    Γῡροω,
    /// Like `ἀφορῐζω`: 78 words.
    Ἀφορῐζω,
    /// Like `σπᾰω`: 78 words.
    Σπᾰω,
    /// Like `βουλευω`: 70 words.
    Βουλευω,
    /// Like `δοκᾰω`: 58 words.
    Δοκᾰω,
    /// Like `ἑπομαι`: 52 words.
    Ἑπομαι,
    /// Like `δῐκεω`: 49 words.
    Δῐκεω,
    /// Like `προσερχομαι`: 43 words.
    Προσερχομαι,
    /// Like `τᾰσσω`: 36 words.
    Τᾰσσω,
    /// Like `κᾰτεργαζομαι`: 31 words.
    Κᾰτεργαζομαι,
    /// Like `ληγω`: 31 words.
    Ληγω,
    /// Like `ἁπτω`: 28 words.
    Ἁπτω,
    /// Like `νῑκαω`: 27 words.
    Νῑκαω,
    /// Like `πρεσβευω`: 27 words.
    Πρεσβευω,
    /// Like `ἐλπω`: 26 words.
    Ἐλπω,
    /// Like `σῠνᾰρπᾰζω`: 25 words.
    Σῠνᾰρπᾰζω,
    /// Like `ὠνεομαι`: 24 words.
    Ὠνεομαι,
    /// Like `εὐλογεω`: 22 words.
    Εὐλογεω,
    /// Like `ψηφῐζω`: 20 words.
    Ψηφῐζω,
    /// Like `θωᾰω`: 16 words.
    Θωᾰω,
    /// Like `πειρᾰζω`: 15 words.
    Πειρᾰζω,
    /// Like `κτῐζω`: 14 words.
    Κτῐζω,
    /// Like `πεμπω`: 14 words.
    Πεμπω,
    /// Like `οἰομαι`: 14 words.
    Οἰομαι,
    /// Like `πᾰρερχομαι`: 13 words.
    Πᾰρερχομαι,
    /// Like `εὐθῡνω`: 12 words.
    Εὐθῡνω,
    /// Like `ἀναιρεω`: 12 words.
    Ἀναιρεω,
    /// Like `σῠνᾰγω`: 12 words.
    Σῠνᾰγω,
    /// Like `ὑπᾰρχω`: 12 words.
    Ὑπᾰρχω,
    /// Like `δῐοικεω`: 12 words.
    Δῐοικεω,
    /// Like `πᾰρειμι`: 12 words.
    Πᾰρειμι,
    /// Like `ἐνᾰντῐοομαι`: 11 words.
    Ἐνᾰντῐοομαι,
    /// Like `λῡω`: 11 words.
    Λῡω,
    /// Like `παυω`: 10 words.
    Παυω,
    /// Like `θρῠπτω`: 10 words.
    Θρῠπτω,
    /// Like `αἱρεω`: 10 words.
    Αἱρεω,
    /// Like `ἀφῑημῐ`: 10 words.
    Ἀφῑημῐ,
    /// Like `ψῑλοω`: 9 words.
    Ψῑλοω,
    /// Like `μᾰντευομαι`: 9 words.
    Μᾰντευομαι,
    /// Like `καθιζω`: 9 words.
    Καθιζω,
    /// Like `ἐαω`: 9 words.
    Ἐαω,
    /// Like `κᾰτηγορεω`: 9 words.
    Κᾰτηγορεω,
    /// Like `πρεπω`: 8 words.
    Πρεπω,
    /// Like `σφᾰλλω`: 8 words.
    Σφᾰλλω,
    /// Like `πᾰρᾰκᾰλεω`: 8 words.
    Πᾰρᾰκᾰλεω,
    /// Like `φᾰρμᾰσσω`: 8 words.
    Φᾰρμᾰσσω,
    /// Like `σῳζω`: 7 words.
    Σῳζω,
    /// Like `ἀπεῑπον`: 7 words.
    Ἀπεῑπον,
    /// Like `φειδομαι`: 6 words.
    Φειδομαι,
    /// Like `ἀραομαι`: 6 words.
    Ἀραομαι,
    /// Like `δῐωκω`: 5 words.
    Δῐωκω,
    /// Like `χραω`: 5 words.
    Χραω,
    /// Like `κᾰθιδρῡω`: 5 words.
    Κᾰθιδρῡω,
    /// Like `φρᾰζω`: 5 words.
    Φρᾰζω,
    /// Like `μᾰρτῠρεω`: 5 words.
    Μᾰρτῠρεω,
    /// Like `πειρᾰω`: 5 words.
    Πειρᾰω,
    /// Like `σῠνοικῐζω`: 5 words.
    Σῠνοικῐζω,
    /// Like `ἐξᾰκολουθεω`: 5 words.
    Ἐξᾰκολουθεω,
    /// Like `ἱζω`: 4 words.
    Ἱζω,
    /// Like `ἡγεομαι`: 4 words.
    Ἡγεομαι,
    /// Like `ἐπᾰγγελλω`: 4 words.
    Ἐπᾰγγελλω,
    /// Like `στεινω`: 4 words.
    Στεινω,
    /// Like `ἀνᾰπτω`: 4 words.
    Ἀνᾰπτω,
    /// Like `πᾰρᾰλλᾰσσω`: 4 words.
    Πᾰρᾰλλᾰσσω,
    /// Like `κᾰτειργω`: 4 words.
    Κᾰτειργω,
    /// Like `πᾰροξῡνω`: 4 words.
    Πᾰροξῡνω,
    /// Like `λογεομαι`: 4 words.
    Λογεομαι,
    /// Like `ἀκουω`: 3 words.
    Ἀκουω,
    /// Like `ἰᾱτρευω`: 3 words.
    Ἰᾱτρευω,
    /// Like `ἐπῐτῐθημῐ`: 3 words.
    Ἐπῐτῐθημῐ,
    /// Like `μνημονευω`: 3 words.
    Μνημονευω,
    /// Like `λιστραω`: 3 words.
    Λιστραω,
    /// Like `ἐπευχομαι`: 3 words.
    Ἐπευχομαι,
    /// Like `ἐγκειμαι`: 3 words.
    Ἐγκειμαι,
    /// Like `δῐᾰλῡω`: 3 words.
    Δῐᾰλῡω,
    /// Like `δαμνημῐ`: 2 words.
    Δαμνημῐ,
    /// Like `πῐμπλημῐ`: 2 words.
    Πῐμπλημῐ,
    /// Like `πᾰρῐστημῐ`: 2 words.
    Πᾰρῐστημῐ,
    /// Like `μᾰρνᾰμαι`: 2 words.
    Μᾰρνᾰμαι,
    /// Like `σῠνομνῡμῐ`: 2 words.
    Σῠνομνῡμῐ,
    /// Like `προσκειμαι`: 2 words.
    Προσκειμαι,
    /// Like `ρρωννῡμῐ`: 2 words.
    Ρρωννῡμῐ,
    /// Like `ῥηγνῡμῐ`: 1 words.
    Ῥηγνῡμῐ,
    /// Like `ἐξῐστημῐ`: 1 words.
    Ἐξῐστημῐ,
    /// Like `ἱστημῐ`: 1 words.
    Ἱστημῐ,
    /// Like `εἰμῐ`: 1 words.
    Εἰμῐ,
    /// Like `ἱημῐ`: 1 words.
    Ἱημῐ,
    /// Like `ἐπεντῐθημῐ`: 1 words.
    Ἐπεντῐθημῐ,
    /// Like `τῐθημῐ`: 1 words.
    Τῐθημῐ,
    /// Like `δῐδωμῐ`: 1 words.
    Δῐδωμῐ,
    /// Like `εἰπον`: 1 words.
    Εἰπον,
    /// Like `δεικνῡμῐ`: 1 words.
    Δεικνῡμῐ,
    /// Like `ἐφᾰγον`: 1 words.
    Ἐφᾰγον,
    /// Like `ἑννῡμῐ`: 1 words.
    Ἑννῡμῐ,
    /// Like `ὀρνῡμῐ`: 1 words.
    Ὀρνῡμῐ,
    /// Like `πηγνῡμῐ`: 1 words.
    Πηγνῡμῐ,
    /// Like `ἀντᾰποδῐδωμῐ`: 1 words.
    Ἀντᾰποδῐδωμῐ,
    /// Like `πετᾰννῡμῐ`: 1 words.
    Πετᾰννῡμῐ,
    /// Like `ἐρᾰμαι`: 1 words.
    Ἐρᾰμαι,
    /// Like `σβεννῡμῐ`: 1 words.
    Σβεννῡμῐ,
    /// Like `φημῐ`: 1 words.
    Φημῐ,
    /// Like `ἐοικᾰ`: 1 words.
    Ἐοικᾰ,
    /// Like `κεῑμαι`: 1 words.
    Κεῑμαι,
    /// Like `μῑγνῡμῐ`: 1 words.
    Μῑγνῡμῐ,
    /// Like `διατιθημῐ`: 1 words.
    Διατιθημῐ,
    /// Like `κερᾰννῡμῐ`: 1 words.
    Κερᾰννῡμῐ,
    /// Like `δυναμαι`: 1 words.
    Δυναμαι,
    /// Like `οἰδᾰ`: 1 words.
    Οἰδᾰ,
    /// Like `πᾰρᾰδιδωμῐ`: 1 words.
    Πᾰρᾰδιδωμῐ,
    /// Like `ἀνῐστημῐ`: 1 words.
    Ἀνῐστημῐ,
    /// Like `δεῑ`: 1 words.
    Δεῑ,
    /// Like `ζωννῡμῐ`: 1 words.
    Ζωννῡμῐ,
    /// Like `κᾰθημαι`: 1 words.
    Κᾰθημαι,
    /// Like `ὀμνῡμῐ`: 1 words.
    Ὀμνῡμῐ,
    /// Like `ἐφῐστημῐ`: 1 words.
    Ἐφῐστημῐ,
    /// Like `κᾰθῐστημῐ`: 1 words.
    Κᾰθῐστημῐ,
    /// Like `ἠμῐ`: 1 words.
    Ἠμῐ,
    /// Like `ἐτλην`: 1 words.
    Ἐτλην,
    /// Like `χρη`: 1 words.
    Χρη,
    /// Like `ἠλθον`: 1 words.
    Ἠλθον,
    /// Like `ζευγνῡμῐ`: 1 words.
    Ζευγνῡμῐ,
    /// Like `ἀρνυμαι`: 1 words.
    Ἀρνυμαι,
    /// Like `εἰδον`: 1 words.
    Εἰδον,
    /// Like `ἐπεφνον`: 1 words.
    Ἐπεφνον,
    /// Like `μετεκῑᾰθον`: 1 words.
    Μετεκῑᾰθον,
    /// Like `ὀλλῡμῐ`: 1 words.
    Ὀλλῡμῐ,
    /// Like `ἀνοιγνῡμῐ`: 1 words.
    Ἀνοιγνῡμῐ,
    /// Like `οἰγνῡμῐ`: 1 words.
    Οἰγνῡμῐ,
    /// Like `πεπνῡμαι`: 1 words.
    Πεπνῡμαι,
    /// Like `περνημῐ`: 1 words.
    Περνημῐ,
    /// Like `κρεμᾰννῡμῐ`: 1 words.
    Κρεμᾰννῡμῐ,
    /// Like `ἐπῐστᾰμαι`: 1 words.
    Ἐπῐστᾰμαι,
    /// Like `πτᾰρνῠμαι`: 1 words.
    Πτᾰρνῠμαι,
    /// Like `ἐπορον`: 1 words.
    Ἐπορον,
    /// Like `δαινῡμῐ`: 1 words.
    Δαινῡμῐ,
    /// Like `ἐνῐστημῐ`: 1 words.
    Ἐνῐστημῐ,
    /// Like `πᾰρᾰτῐθημῐ`: 1 words.
    Πᾰρᾰτῐθημῐ,
    /// Like `κατατῐθημῐ`: 1 words.
    Κατατῐθημῐ,
    /// Like `κεκᾰδον`: 1 words.
    Κεκᾰδον,
    /// Like `ἀγᾰμαι`: 1 words.
    Ἀγᾰμαι,
    /// Like `σῠνιστημῐ`: 1 words.
    Σῠνιστημῐ,
    /// Like `ξῠνιστημῐ`: 1 words.
    Ξῠνιστημῐ,
    /// Like `ἀπᾰνῐστημῐ`: 1 words.
    Ἀπᾰνῐστημῐ,
    /// Like `καινῠμαι`: 1 words.
    Καινῠμαι,
    /// Like `ἐγκᾰθῐστημῐ`: 1 words.
    Ἐγκᾰθῐστημῐ,
    /// Like `μεθῐστημῐ`: 1 words.
    Μεθῐστημῐ,
    /// Like `διαζωννῡμῐ`: 1 words.
    Διαζωννῡμῐ,
    /// Like `σκεδᾰννῡμῐ`: 1 words.
    Σκεδᾰννῡμῐ,
    /// Like `κορεννῡμῐ`: 1 words.
    Κορεννῡμῐ,
    /// Like `ὀμοργνῡμῐ`: 1 words.
    Ὀμοργνῡμῐ,
    /// Like `δῐϊστημῐ`: 1 words.
    Δῐϊστημῐ,
    /// Like `στορνῡμῐ`: 1 words.
    Στορνῡμῐ,
    /// Like `ἀφῐστημῐ`: 1 words.
    Ἀφῐστημῐ,
    /// Like `κᾰτακειμαι`: 1 words.
    Κᾰτακειμαι,
    /// Like `συνοιδᾰ`: 1 words.
    Συνοιδᾰ,
    /// Like `ξῠνοιδᾰ`: 1 words.
    Ξῠνοιδᾰ,
    /// Like `προοιδᾰ`: 1 words.
    Προοιδᾰ,
    /// Like `ὀνινημῐ`: 1 words.
    Ὀνινημῐ,
    /// Like `ἐμπιμπρημι`: 1 words.
    Ἐμπιμπρημι,
    /// Like `ἀνθῐστημῐ`: 1 words.
    Ἀνθῐστημῐ,
    /// Like `περῐῐστημῐ`: 1 words.
    Περῐῐστημῐ,
    /// Like `ἐπορνῡμῐ`: 1 words.
    Ἐπορνῡμῐ,
    /// Like `ἐξεστῐ`: 1 words.
    Ἐξεστῐ,
    /// Like `ὑφῐστημῐ`: 1 words.
    Ὑφῐστημῐ,
    /// Like `γεγονοι`: 1 words.
    Γεγονοι,
    /// Like `γεγενησαι`: 1 words.
    Γεγενησαι,
    /// Like `γεγενηται`: 1 words.
    Γεγενηται,
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
    /// Like `προσμειγνῡμῐ`: 1 words.
    Προσμειγνῡμῐ,
    /// Like `κιδνᾰμαι`: 1 words.
    Κιδνᾰμαι,
    /// Like `τεθηπᾰ`: 1 words.
    Τεθηπᾰ,
    /// Like `ἀντῐκᾰθῐστημῐ`: 1 words.
    Ἀντῐκᾰθῐστημῐ,
    /// Like `στρωννῡμῐ`: 1 words.
    Στρωννῡμῐ,
    /// Like `δῐκεῑν`: 1 words.
    Δῐκεῑν,
    /// Like `ξῠρρηγνῡμῐ`: 1 words.
    Ξῠρρηγνῡμῐ,
    /// Like `ἀμφῑστημῐ`: 1 words.
    Ἀμφῑστημῐ,
    /// Like `γανῠμαι`: 1 words.
    Γανῠμαι,
    /// Like `δῐακειμαι`: 1 words.
    Δῐακειμαι,
    /// Like `ἀδησᾰ`: 1 words.
    Ἀδησᾰ,
    /// Like `μειγνῡμῐ`: 1 words.
    Μειγνῡμῐ,
    /// Like `ἐμπηγνῡμῐ`: 1 words.
    Ἐμπηγνῡμῐ,
    /// Like `ὑπεκτιθεμαι`: 1 words.
    Ὑπεκτιθεμαι,
    /// Like `ἐκτιθημῐ`: 1 words.
    Ἐκτιθημῐ,
    /// Like `προϊστημῐ`: 1 words.
    Προϊστημῐ,
    /// Like `λλῡμῐ`: 1 words.
    Λλῡμῐ,
    /// Like `γνῡμῐ`: 1 words.
    Γνῡμῐ,
    /// Like `ρρηγνῡμῐ`: 1 words.
    Ρρηγνῡμῐ,
    /// Like `δειδῐᾰ`: 1 words.
    Δειδῐᾰ,
    /// Like `κᾰτᾰμῑγνῡμῐ`: 1 words.
    Κᾰτᾰμῑγνῡμῐ,
    /// Like `προδῐδωμῐ`: 1 words.
    Προδῐδωμῐ,
}

impl Conjugation {
    /// Every conjugation, in order.
    pub const ALL: [Conjugation; 226] = [
        Conjugation::Ἐχω,
        Conjugation::Ποιεω,
        Conjugation::Ὁρῐζω,
        Conjugation::Γῡροω,
        Conjugation::Ἀφορῐζω,
        Conjugation::Σπᾰω,
        Conjugation::Βουλευω,
        Conjugation::Δοκᾰω,
        Conjugation::Ἑπομαι,
        Conjugation::Δῐκεω,
        Conjugation::Προσερχομαι,
        Conjugation::Τᾰσσω,
        Conjugation::Κᾰτεργαζομαι,
        Conjugation::Ληγω,
        Conjugation::Ἁπτω,
        Conjugation::Νῑκαω,
        Conjugation::Πρεσβευω,
        Conjugation::Ἐλπω,
        Conjugation::Σῠνᾰρπᾰζω,
        Conjugation::Ὠνεομαι,
        Conjugation::Εὐλογεω,
        Conjugation::Ψηφῐζω,
        Conjugation::Θωᾰω,
        Conjugation::Πειρᾰζω,
        Conjugation::Κτῐζω,
        Conjugation::Πεμπω,
        Conjugation::Οἰομαι,
        Conjugation::Πᾰρερχομαι,
        Conjugation::Εὐθῡνω,
        Conjugation::Ἀναιρεω,
        Conjugation::Σῠνᾰγω,
        Conjugation::Ὑπᾰρχω,
        Conjugation::Δῐοικεω,
        Conjugation::Πᾰρειμι,
        Conjugation::Ἐνᾰντῐοομαι,
        Conjugation::Λῡω,
        Conjugation::Παυω,
        Conjugation::Θρῠπτω,
        Conjugation::Αἱρεω,
        Conjugation::Ἀφῑημῐ,
        Conjugation::Ψῑλοω,
        Conjugation::Μᾰντευομαι,
        Conjugation::Καθιζω,
        Conjugation::Ἐαω,
        Conjugation::Κᾰτηγορεω,
        Conjugation::Πρεπω,
        Conjugation::Σφᾰλλω,
        Conjugation::Πᾰρᾰκᾰλεω,
        Conjugation::Φᾰρμᾰσσω,
        Conjugation::Σῳζω,
        Conjugation::Ἀπεῑπον,
        Conjugation::Φειδομαι,
        Conjugation::Ἀραομαι,
        Conjugation::Δῐωκω,
        Conjugation::Χραω,
        Conjugation::Κᾰθιδρῡω,
        Conjugation::Φρᾰζω,
        Conjugation::Μᾰρτῠρεω,
        Conjugation::Πειρᾰω,
        Conjugation::Σῠνοικῐζω,
        Conjugation::Ἐξᾰκολουθεω,
        Conjugation::Ἱζω,
        Conjugation::Ἡγεομαι,
        Conjugation::Ἐπᾰγγελλω,
        Conjugation::Στεινω,
        Conjugation::Ἀνᾰπτω,
        Conjugation::Πᾰρᾰλλᾰσσω,
        Conjugation::Κᾰτειργω,
        Conjugation::Πᾰροξῡνω,
        Conjugation::Λογεομαι,
        Conjugation::Ἀκουω,
        Conjugation::Ἰᾱτρευω,
        Conjugation::Ἐπῐτῐθημῐ,
        Conjugation::Μνημονευω,
        Conjugation::Λιστραω,
        Conjugation::Ἐπευχομαι,
        Conjugation::Ἐγκειμαι,
        Conjugation::Δῐᾰλῡω,
        Conjugation::Δαμνημῐ,
        Conjugation::Πῐμπλημῐ,
        Conjugation::Πᾰρῐστημῐ,
        Conjugation::Μᾰρνᾰμαι,
        Conjugation::Σῠνομνῡμῐ,
        Conjugation::Προσκειμαι,
        Conjugation::Ρρωννῡμῐ,
        Conjugation::Ῥηγνῡμῐ,
        Conjugation::Ἐξῐστημῐ,
        Conjugation::Ἱστημῐ,
        Conjugation::Εἰμῐ,
        Conjugation::Ἱημῐ,
        Conjugation::Ἐπεντῐθημῐ,
        Conjugation::Τῐθημῐ,
        Conjugation::Δῐδωμῐ,
        Conjugation::Εἰπον,
        Conjugation::Δεικνῡμῐ,
        Conjugation::Ἐφᾰγον,
        Conjugation::Ἑννῡμῐ,
        Conjugation::Ὀρνῡμῐ,
        Conjugation::Πηγνῡμῐ,
        Conjugation::Ἀντᾰποδῐδωμῐ,
        Conjugation::Πετᾰννῡμῐ,
        Conjugation::Ἐρᾰμαι,
        Conjugation::Σβεννῡμῐ,
        Conjugation::Φημῐ,
        Conjugation::Ἐοικᾰ,
        Conjugation::Κεῑμαι,
        Conjugation::Μῑγνῡμῐ,
        Conjugation::Διατιθημῐ,
        Conjugation::Κερᾰννῡμῐ,
        Conjugation::Δυναμαι,
        Conjugation::Οἰδᾰ,
        Conjugation::Πᾰρᾰδιδωμῐ,
        Conjugation::Ἀνῐστημῐ,
        Conjugation::Δεῑ,
        Conjugation::Ζωννῡμῐ,
        Conjugation::Κᾰθημαι,
        Conjugation::Ὀμνῡμῐ,
        Conjugation::Ἐφῐστημῐ,
        Conjugation::Κᾰθῐστημῐ,
        Conjugation::Ἠμῐ,
        Conjugation::Ἐτλην,
        Conjugation::Χρη,
        Conjugation::Ἠλθον,
        Conjugation::Ζευγνῡμῐ,
        Conjugation::Ἀρνυμαι,
        Conjugation::Εἰδον,
        Conjugation::Ἐπεφνον,
        Conjugation::Μετεκῑᾰθον,
        Conjugation::Ὀλλῡμῐ,
        Conjugation::Ἀνοιγνῡμῐ,
        Conjugation::Οἰγνῡμῐ,
        Conjugation::Πεπνῡμαι,
        Conjugation::Περνημῐ,
        Conjugation::Κρεμᾰννῡμῐ,
        Conjugation::Ἐπῐστᾰμαι,
        Conjugation::Πτᾰρνῠμαι,
        Conjugation::Ἐπορον,
        Conjugation::Δαινῡμῐ,
        Conjugation::Ἐνῐστημῐ,
        Conjugation::Πᾰρᾰτῐθημῐ,
        Conjugation::Κατατῐθημῐ,
        Conjugation::Κεκᾰδον,
        Conjugation::Ἀγᾰμαι,
        Conjugation::Σῠνιστημῐ,
        Conjugation::Ξῠνιστημῐ,
        Conjugation::Ἀπᾰνῐστημῐ,
        Conjugation::Καινῠμαι,
        Conjugation::Ἐγκᾰθῐστημῐ,
        Conjugation::Μεθῐστημῐ,
        Conjugation::Διαζωννῡμῐ,
        Conjugation::Σκεδᾰννῡμῐ,
        Conjugation::Κορεννῡμῐ,
        Conjugation::Ὀμοργνῡμῐ,
        Conjugation::Δῐϊστημῐ,
        Conjugation::Στορνῡμῐ,
        Conjugation::Ἀφῐστημῐ,
        Conjugation::Κᾰτακειμαι,
        Conjugation::Συνοιδᾰ,
        Conjugation::Ξῠνοιδᾰ,
        Conjugation::Προοιδᾰ,
        Conjugation::Ὀνινημῐ,
        Conjugation::Ἐμπιμπρημι,
        Conjugation::Ἀνθῐστημῐ,
        Conjugation::Περῐῐστημῐ,
        Conjugation::Ἐπορνῡμῐ,
        Conjugation::Ἐξεστῐ,
        Conjugation::Ὑφῐστημῐ,
        Conjugation::Γεγονοι,
        Conjugation::Γεγενησαι,
        Conjugation::Γεγενηται,
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
        Conjugation::Προσμειγνῡμῐ,
        Conjugation::Κιδνᾰμαι,
        Conjugation::Τεθηπᾰ,
        Conjugation::Ἀντῐκᾰθῐστημῐ,
        Conjugation::Στρωννῡμῐ,
        Conjugation::Δῐκεῑν,
        Conjugation::Ξῠρρηγνῡμῐ,
        Conjugation::Ἀμφῑστημῐ,
        Conjugation::Γανῠμαι,
        Conjugation::Δῐακειμαι,
        Conjugation::Ἀδησᾰ,
        Conjugation::Μειγνῡμῐ,
        Conjugation::Ἐμπηγνῡμῐ,
        Conjugation::Ὑπεκτιθεμαι,
        Conjugation::Ἐκτιθημῐ,
        Conjugation::Προϊστημῐ,
        Conjugation::Λλῡμῐ,
        Conjugation::Γνῡμῐ,
        Conjugation::Ρρηγνῡμῐ,
        Conjugation::Δειδῐᾰ,
        Conjugation::Κᾰτᾰμῑγνῡμῐ,
        Conjugation::Προδῐδωμῐ,
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

pub(super) static CONJUGATIONS: [Paradigm<'static>; 226] = [
    // Ἐχω: 457 words (θελω, τῠγχᾰνω, δῐψᾱω, δῐδᾰσκω, πῑνω, εὑρῐσκω).
    conjugation!("ἐχω", 457, "~ω";
    ind_act_1: "~ω", _, "~ομεν";
    ind_act_2: "~εις", _, _;
    ind_act_3: "~ει", _, "~ουσῐ";
    ind_mid_1: "~ομαι", _, "~ομεθᾰ";
    ind_mid_2: "~ῃ", "~εσθον", "~εσθε";
    ind_mid_3: "~εται", _, "~ονται";
    ind_pass_1: _, _, "~ομεθᾰ";
    ind_pass_2: _, _, "~εσθε";
    ind_pass_3: "~εται", _, "~ονται";
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", "~ητον", "~ητε";
    subj_act_3: "~ῃ", _, "~ωσῐ";
    subj_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    subj_mid_2: "~ῃ", "~ησθον", "~ησθε";
    subj_mid_3: "~ηται", _, "~ωνται";
    subj_pass_1: _, _, "~ωμεθᾰ";
    subj_pass_2: _, _, "~ησθε";
    subj_pass_3: "~ηται", _, "~ωνται";
    opt_act_1: "~οιμῐ", _, "~οιμεν";
    opt_act_2: "~οις", "~οιτον", "~οιτε";
    opt_act_3: "~οι", "~οιτην", "~οιεν";
    opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    opt_mid_2: "~οιο", "~οισθον", "~οισθε";
    opt_mid_3: "~οιτο", "~οισθην", "~οιντο";
    opt_pass_1: _, _, "~οιμεθᾰ";
    opt_pass_2: _, "~οισθον", "~οισθε";
    opt_pass_3: "~οιτο", "~οισθην", "~οιντο";
    imp_act_2: "~ε", "~ετον", "~ετε";
    imp_act_3: "~ετω", "~ετων", "~οντων";
    imp_mid_2: "~ου", _, "~εσθε";
    imp_mid_3: "~εσθω", "~εσθων", "~εσθων";
    imp_pass_2: _, _, "~εσθε";
    imp_pass_3: _, "~εσθων", _;
    imperfect_act_1: "ἐ~ον", _, "ἐ~ομεν";
    imperfect_act_2: "ἐ~ες", "ἐ~ετον", "ἐ~ετε";
    imperfect_act_3: "ἐ~ε", "ἐ~ετην", "ἐ~ον";
    imperfect_mid_1: "ἐ~ομην", _, "ἐ~ομεθᾰ";
    imperfect_mid_2: "ἐ~ου", "ἐ~εσθον", "ἐ~εσθε";
    imperfect_mid_3: "ἐ~ετο", "ἐ~εσθην", "ἐ~οντο";
    imperfect_pass_1: _, _, "ἐ~ομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~εσθε";
    imperfect_pass_3: "ἐ~ετο", _, _;
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
    aorist_ind_act_1: "ἐ~ησᾰ", _, "ἐ~ησᾰμεν";
    aorist_ind_act_2: "ἐ~ησᾰς", "ἐ~ησᾰτον", "ἐ~ησᾰτε";
    aorist_ind_act_3: "ἐ~ησε", "ἐ~ησᾰτην", "ἐ~ησᾰν";
    aorist_ind_mid_1: "ἐ~ησᾰμην", _, "ἐ~ησᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ησω", "ἐ~ησᾰσθον", "ἐ~ησᾰσθε";
    aorist_ind_mid_3: "ἐ~ησᾰτο", "ἐ~ησᾰσθην", "ἐ~ησᾰντο";
    aorist_ind_pass_1: "ἐ~ηθην", _, "ἐ~ηθημεν";
    aorist_ind_pass_2: "ἐ~ηθης", "ἐ~ηθητον", "ἐ~ηθητε";
    aorist_ind_pass_3: "ἐ~ηθη", "ἐ~ηθητην", "ἐ~ηθησᾰν";
    aorist_subj_act_1: "~ησω", _, "~ησωμεν";
    aorist_subj_act_2: "~ησῃς", "~ησητον", "~ησητε";
    aorist_subj_act_3: "~ησῃ", _, "~ησωσῐ";
    aorist_subj_mid_1: "~ησωμαι", _, "~ησωμεθᾰ";
    aorist_subj_mid_2: "~ησῃ", "~ησησθον", "~ησησθε";
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
    perfect_ind_pass_1: _, _, "%ε~ημεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~ησθε";
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
    perfect_imp_pass_2: "%ε~ησο", _, _;
    perfect_imp_pass_3: "%ε~ησθω", "%ε~ησθων", _;
    pluperfect_act_1: "ἐ%ε~ηκειν", _, "ἐ%ε~ηκεμεν";
    pluperfect_act_2: "ἐ%ε~ηκεις", "ἐ%ε~ηκετον", "ἐ%ε~ηκετε";
    pluperfect_act_3: "ἐ%ε~ηκει", "ἐ%ε~ηκετην", "ἐ%ε~ηκεσᾰν";
    pluperfect_mid_1: "ἐ%ε~ημην", _, "ἐ%ε~ημεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ησο", "ἐ%ε~ησθον", "ἐ%ε~ησθε";
    pluperfect_mid_3: "ἐ%ε~ητο", "ἐ%ε~ησθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~ησθην", "ἐ%ε~ηντο";
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
    // Ποιεω: 251 words (φοβεω, βλασφημεω, γᾰμεω, μῑσεω, διακονεω, ποιεω). Like Ἐχω, except in 69 cells: ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ind_mid_1 pl, ….
    conjugation!("ποιεω", 251, "~εω", like Ἐχω;
    ind_act_1: _, _, "~οῡμεν";
    ind_act_2: "~εῑς", _, _;
    ind_act_3: "~εῑ", _, "~οῡσῐ|~οῡσῐν";
    ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    ind_mid_2: "~εῑ|~ῃ", "~εῑσθον", "~εῑσθε";
    ind_mid_3: "~εῑται", _, "~οῡνται";
    ind_pass_1: _, _, "~ουμεθᾰ";
    ind_pass_2: "~εῑ|~ῃ", _, "~εῑσθε";
    ind_pass_3: "~εῑται", _, "~οῡνται";
    subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    opt_act_1: "~οιην|~οῑμῐ", _, "~οῑμεν|~οιημεν";
    opt_act_2: "~οιης|~οῑς", "~οῑτον|~οιητον", "~οῑτε|~οιητε";
    opt_act_3: "~οιη|~οῑ", "~οιτην|~οιητην", "~οῑεν|~οιησᾰν";
    opt_mid_2: "~οῑο", "~οῑσθον", "~οῑσθε";
    opt_mid_3: "~οῑτο", _, "~οῑντο";
    opt_pass_2: _, "~οῑσθον", "~οῑσθε";
    opt_pass_3: "~οῑτο", _, "~οῑντο";
    imp_act_2: "~ει", "~εῑτον", "~εῑτε";
    imp_act_3: "~ειτω", "~ειτων", "~ουντων";
    imp_mid_2: "~οῡ", _, "~εῑσθε";
    imp_mid_3: "~εισθω", "~εισθων", "~εισθων";
    imp_pass_2: _, _, "~εῑσθε";
    imp_pass_3: _, "~εισθων", _;
    imperfect_act_1: "ἐ~ουν", _, "ἐ~οῡμεν";
    imperfect_act_2: "ἐ~εις", "ἐ~εῑτον", "ἐ~εῑτε";
    imperfect_act_3: "ἐ~ει", "ἐ~ειτην", "ἐ~ουν";
    imperfect_mid_1: "ἐ~ουμην", _, "ἐ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~οῡ", "ἐ~εῑσθον", "ἐ~εῑσθε";
    imperfect_mid_3: "ἐ~εῑτο", "ἐ~εισθην", "ἐ~οῡντο";
    imperfect_pass_1: _, _, "ἐ~ουμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~εῑσθε";
    imperfect_pass_3: "ἐ~εῑτο", _, _;
    infinitive_pres: "~εῑν", "~εῑσθαι", _;
    participle_pres: _, "~ουμενος", _;
    ),
    // Ὁρῐζω: 93 words (πυρρᾰζω, σκᾰνδᾰλῐζω, βᾰπτῐζω, δειγμᾰτῐζω, δᾰμαζω, ἁρμοζω).
    conjugation!("ὁρῐζω", 93, "~ζω";
    ind_act_1: "~ζω", _, "~ζομεν";
    ind_act_2: "~ζεις", _, _;
    ind_act_3: "~ζει", _, "~ζουσῐ";
    ind_mid_1: "~ζομαι", _, "~ζομεθᾰ";
    ind_mid_2: "~ζῃ", "~ζεσθον", "~ζεσθε";
    ind_mid_3: "~ζεται", _, "~ζονται";
    ind_pass_1: _, _, "~ζομεθᾰ";
    ind_pass_2: _, _, "~ζεσθε";
    ind_pass_3: "~ζεται", _, "~ζονται";
    subj_act_1: "~ζω", _, "~ζωμεν";
    subj_act_2: "~ζῃς", "~ζητον", "~ζητε";
    subj_act_3: "~ζῃ", _, "~ζωσῐ";
    subj_mid_1: "~ζωμαι", _, "~ζωμεθᾰ";
    subj_mid_2: "~ζῃ", "~ζησθον", "~ζησθε";
    subj_mid_3: "~ζηται", _, "~ζωνται";
    subj_pass_1: _, _, "~ζωμεθᾰ";
    subj_pass_2: _, _, "~ζησθε";
    subj_pass_3: "~ζηται", _, "~ζωνται";
    opt_act_1: "~ζοιμῐ", _, "~ζοιμεν";
    opt_act_2: "~ζοις", "~ζοιτον", "~ζοιτε";
    opt_act_3: "~ζοι", "~ζοιτην", "~ζοιεν";
    opt_mid_1: "~ζοιμην", _, "~ζοιμεθᾰ";
    opt_mid_2: "~ζοιο", "~ζοισθον", "~ζοισθε";
    opt_mid_3: "~ζοιτο", "~ζοισθην", "~ζοιντο";
    opt_pass_1: _, _, "~ζοιμεθᾰ";
    opt_pass_2: _, "~ζοισθον", "~ζοισθε";
    opt_pass_3: "~ζοιτο", "~ζοισθην", "~ζοιντο";
    imp_act_2: "~ζε", "~ζετον", "~ζετε";
    imp_act_3: "~ζετω", "~ζετων", "~ζοντων";
    imp_mid_2: "~ζου", _, "~ζεσθε";
    imp_mid_3: "~ζεσθω", "~ζεσθων", "~ζεσθων";
    imp_pass_2: _, _, "~ζεσθε";
    imp_pass_3: _, "~ζεσθων", _;
    imperfect_act_1: "ἐ~ζον", _, "ἐ~ζομεν";
    imperfect_act_2: "ἐ~ζες", "ἐ~ζετον", "ἐ~ζετε";
    imperfect_act_3: "ἐ~ζε", "ἐ~ζετην", "ἐ~ζον";
    imperfect_mid_1: "ἐ~ζομην", _, "ἐ~ζομεθᾰ";
    imperfect_mid_2: "ἐ~ζου", "ἐ~ζεσθον", "ἐ~ζεσθε";
    imperfect_mid_3: "ἐ~ζετο", "ἐ~ζεσθην", "ἐ~ζοντο";
    imperfect_pass_1: _, _, "ἐ~ζομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ζεσθε";
    imperfect_pass_3: "ἐ~ζετο", _, _;
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
    aorist_ind_act_1: "ἐ~σᾰ", _, "ἐ~σᾰμεν";
    aorist_ind_act_2: "ἐ~σᾰς", "ἐ~σᾰτον", "ἐ~σᾰτε";
    aorist_ind_act_3: "ἐ~σε", "ἐ~σᾰτην", "ἐ~σᾰν";
    aorist_ind_mid_1: "ἐ~σᾰμην", _, "ἐ~σᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~σω", "ἐ~σᾰσθον", "ἐ~σᾰσθε";
    aorist_ind_mid_3: "ἐ~σᾰτο", "ἐ~σᾰσθην", "ἐ~σᾰντο";
    aorist_ind_pass_1: "ἐ~σθην", _, "ἐ~σθημεν";
    aorist_ind_pass_2: "ἐ~σθης", "ἐ~σθητον", "ἐ~σθητε";
    aorist_ind_pass_3: "ἐ~σθη", "ἐ~σθητην", "ἐ~σθησᾰν";
    aorist_subj_act_1: "~σω", _, "~σωμεν";
    aorist_subj_act_2: "~σῃς", "~σητον", "~σητε";
    aorist_subj_act_3: "~σῃ", _, "~σωσῐ";
    aorist_subj_mid_1: "~σωμαι", _, "~σωμεθᾰ";
    aorist_subj_mid_2: "~σῃ", "~σησθον", "~σησθε";
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
    perfect_ind_pass_1: _, _, "%ε~σμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~σθε";
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
    perfect_imp_pass_2: "%ε~σο", _, _;
    perfect_imp_pass_3: "%ε~σθω", "%ε~σθων", _;
    pluperfect_act_1: "ἐ%ε~κειν", _, "ἐ%ε~κεμεν";
    pluperfect_act_2: "ἐ%ε~κεις", "ἐ%ε~κετον", "ἐ%ε~κετε";
    pluperfect_act_3: "ἐ%ε~κει", "ἐ%ε~κετην", "ἐ%ε~κεσᾰν";
    pluperfect_mid_1: "ἐ%ε~σμην", _, "ἐ%ε~σμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~σο", "ἐ%ε~σθον", "ἐ%ε~σθε";
    pluperfect_mid_3: "ἐ%ε~στο", "ἐ%ε~σθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~σθε";
    pluperfect_pass_3: _, "ἐ%ε~σθην", "ἐ%ε~δᾰτο";
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
    // Γῡροω: 87 words (ὁμοιοω, μορφοω, ζηλοω, ναρκοω, κοινοω, ἀλλοιοω). Like Ποιεω, except in 266 cells: ind_act_2 sg, ind_act_3 sg, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ind_mid_3 sg, ….
    conjugation!("γῡροω", 87, "~οω", like Ποιεω;
    ind_act_2: "~οῑς", _, _;
    ind_act_3: "~οῑ", _, _;
    ind_mid_2: "~οῑ", "~οῡσθον", "~οῡσθε";
    ind_mid_3: "~οῡται", _, _;
    ind_pass_2: derived, _, "~οῡσθε";
    ind_pass_3: "~οῡται", _, _;
    subj_act_2: "~οῑς", "~ωτον", "~ωτε";
    subj_act_3: "~οῑ", _, _;
    subj_mid_2: "~οῑ", "~ωσθον", "~ωσθε";
    subj_mid_3: "~ωται", _, _;
    subj_pass_2: _, _, "~ωσθε";
    subj_pass_3: "~ωται", _, _;
    imp_act_2: "~ου", "~οῡτον", "~οῡτε";
    imp_act_3: "~ουτω", "~ουτων", _;
    imp_mid_2: _, _, "~οῡσθε";
    imp_mid_3: "~ουσθω", "~ουσθων", "~ουσθων";
    imp_pass_2: _, _, "~οῡσθε";
    imp_pass_3: _, "~ουσθων", _;
    imperfect_act_2: "ἐ~ους", "ἐ~οῡτον", "ἐ~οῡτε";
    imperfect_act_3: "ἐ~ου", "ἐ~ουτην", _;
    imperfect_mid_2: _, "ἐ~οῡσθον", "ἐ~οῡσθε";
    imperfect_mid_3: "ἐ~οῡτο", "ἐ~ουσθην", _;
    imperfect_pass_2: _, _, "ἐ~οῡσθε";
    imperfect_pass_3: "ἐ~οῡτο", _, _;
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
    aorist_ind_act_1: "ἐ~ωσᾰ", _, "ἐ~ωσᾰμεν";
    aorist_ind_act_2: "ἐ~ωσᾰς", "ἐ~ωσᾰτον", "ἐ~ωσᾰτε";
    aorist_ind_act_3: "ἐ~ωσε|ἐ~ωσεν", "ἐ~ωσᾰτην", "ἐ~ωσᾰν";
    aorist_ind_mid_1: "ἐ~ωσᾰμην", _, "ἐ~ωσᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ωσω", "ἐ~ωσᾰσθον", "ἐ~ωσᾰσθε";
    aorist_ind_mid_3: "ἐ~ωσᾰτο", "ἐ~ωσᾰσθην", "ἐ~ωσᾰντο";
    aorist_ind_pass_1: "ἐ~ωθην", _, "ἐ~ωθημεν";
    aorist_ind_pass_2: "ἐ~ωθης", "ἐ~ωθητον", "ἐ~ωθητε";
    aorist_ind_pass_3: "ἐ~ωθη", "ἐ~ωθητην", "ἐ~ωθησᾰν";
    aorist_subj_act_1: "~ωσω", _, "~ωσωμεν";
    aorist_subj_act_2: "~ωσῃς", "~ωσητον", "~ωσητε";
    aorist_subj_act_3: "~ωσῃ", _, "~ωσωσῐ|~ωσωσῐν";
    aorist_subj_mid_1: "~ωσωμαι", _, "~ωσωμεθᾰ";
    aorist_subj_mid_2: "~ωσῃ", "~ωσησθον", "~ωσησθε";
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
    perfect_ind_pass_1: _, _, "%ε~ωμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~ωσθε";
    perfect_subj_act_1: "%ε~ωκω", _, "%ε~ωκωμεν";
    perfect_subj_act_2: "%ε~ωκῃς", "%ε~ωκητον", "%ε~ωκητε";
    perfect_subj_act_3: "%ε~ωκῃ", _, "%ε~ωκωσῐ";
    perfect_opt_act_1: "%ε~ωκοιμῐ", _, "%ε~ωκοιμεν";
    perfect_opt_act_2: "%ε~ωκοις", "%ε~ωκοιτον", "%ε~ωκοιτε";
    perfect_opt_act_3: "%ε~ωκοι", "%ε~ωκοιτην", "%ε~ωκοιεν";
    perfect_imp_act_2: _, "%ε~ωκετον", "%ε~ωκετε";
    perfect_imp_act_3: "%ε~ωκετω", "%ε~ωκετων", "%ε~ωκοντων";
    perfect_imp_mid_2: "%ε~ωσο", _, _;
    perfect_imp_mid_3: "%ε~ωσθω", "%ε~ωσθων", _;
    perfect_imp_pass_2: "%ε~ωσο", _, _;
    perfect_imp_pass_3: "%ε~ωσθω", "%ε~ωσθων", _;
    pluperfect_act_1: "ἐ%ε~ωκειν", _, "ἐ%ε~ωκεμεν";
    pluperfect_act_2: "ἐ%ε~ωκεις", "ἐ%ε~ωκετον", "ἐ%ε~ωκετε";
    pluperfect_act_3: "ἐ%ε~ωκει", "ἐ%ε~ωκετην", "ἐ%ε~ωκεσᾰν";
    pluperfect_mid_1: "ἐ%ε~ωμην", _, "ἐ%ε~ωμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ωσο", "ἐ%ε~ωσθον", "ἐ%ε~ωσθε";
    pluperfect_mid_3: "ἐ%ε~ωτο", "ἐ%ε~ωσθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ωσθε";
    pluperfect_pass_3: _, "ἐ%ε~ωσθην", "ἐ%ε~ωντο";
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
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
    // Ἀφορῐζω: 78 words (κᾰθᾰρῐζω, βᾰσᾰνῐζω, κᾰπνῐζω, φενᾱκῐζω, γεμῐζω, ἱζω). Like Ὁρῐζω, except in 73 cells: ind_act_3 pl, ind_mid_2 sg, ind_pass_2 sg, subj_act_3 pl, future_ind_act_1 sg, future_ind_act_1 pl, ….
    conjugation!("ἀφορῐζω", 78, "~ζω", like Ὁρῐζω;
    ind_act_3: _, _, "~ζουσῐ|~ζουσῐν";
    ind_mid_2: "~ζῃ|~ζει", _, _;
    ind_pass_2: "~ζῃ|~ζει", _, _;
    subj_act_3: _, _, "~ζωσῐ|~ζωσῐν";
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
    // Σπᾰω: 78 words (ἀρχω, στεφω, βρεχω, ἐκδῠω, οἰφω, πεκω). Like Ὁρῐζω, except in 220 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("σπᾰω", 78, "~ω", like Ὁρῐζω;
    ind_act_1: "~ω", _, "~ομεν";
    ind_act_2: "~εις", _, _;
    ind_act_3: "~ει", _, "~ουσῐ";
    ind_mid_1: "~ομαι", _, "~ομεθᾰ";
    ind_mid_2: "~ῃ", "~εσθον", "~εσθε";
    ind_mid_3: "~εται", _, "~ονται";
    ind_pass_1: _, _, "~ομεθᾰ";
    ind_pass_2: _, _, "~εσθε";
    ind_pass_3: "~εται", _, "~ονται";
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", "~ητον", "~ητε";
    subj_act_3: "~ῃ", _, "~ωσῐ";
    subj_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    subj_mid_2: "~ῃ", "~ησθον", "~ησθε";
    subj_mid_3: "~ηται", _, "~ωνται";
    subj_pass_1: _, _, "~ωμεθᾰ";
    subj_pass_2: _, _, "~ησθε";
    subj_pass_3: "~ηται", _, "~ωνται";
    opt_act_1: "~οιμῐ", _, "~οιμεν";
    opt_act_2: "~οις", "~οιτον", "~οιτε";
    opt_act_3: "~οι", "~οιτην", "~οιεν";
    opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    opt_mid_2: "~οιο", "~οισθον", "~οισθε";
    opt_mid_3: "~οιτο", "~οισθην", "~οιντο";
    opt_pass_1: _, _, "~οιμεθᾰ";
    opt_pass_2: _, "~οισθον", "~οισθε";
    opt_pass_3: "~οιτο", "~οισθην", "~οιντο";
    imp_act_2: "~ε", "~ετον", "~ετε";
    imp_act_3: "~ετω", "~ετων", "~οντων";
    imp_mid_2: "~ου", _, "~εσθε";
    imp_mid_3: "~εσθω", "~εσθων", "~εσθων";
    imp_pass_2: _, _, "~εσθε";
    imp_pass_3: _, "~εσθων", _;
    imperfect_act_1: "ἐ~ον", _, "ἐ~ομεν";
    imperfect_act_2: "ἐ~ες", "ἐ~ετον", "ἐ~ετε";
    imperfect_act_3: "ἐ~ε", "ἐ~ετην", "ἐ~ον";
    imperfect_mid_1: "ἐ~ομην", _, "ἐ~ομεθᾰ";
    imperfect_mid_2: "ἐ~ου", "ἐ~εσθον", "ἐ~εσθε";
    imperfect_mid_3: "ἐ~ετο", "ἐ~εσθην", "ἐ~οντο";
    imperfect_pass_1: _, _, "ἐ~ομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~εσθε";
    imperfect_pass_3: "ἐ~ετο", _, _;
    future_ind_pass_1: "~θησομαι", _, "~θησομεθᾰ";
    future_ind_pass_2: "~θησῃ", "~θησεσθον", "~θησεσθε";
    future_ind_pass_3: "~θησεται", _, "~θησονται";
    future_opt_pass_1: "~θησοιμην", _, "~θησοιμεθᾰ";
    future_opt_pass_2: "~θησοιο", "~θησοισθον", "~θησοισθε";
    future_opt_pass_3: "~θησοιτο", "~θησοισθην", "~θησοιντο";
    aorist_ind_pass_1: "ἐ~θην", _, "ἐ~θημεν";
    aorist_ind_pass_2: "ἐ~θης", "ἐ~θητον", "ἐ~θητε";
    aorist_ind_pass_3: "ἐ~θη", "ἐ~θητην", "ἐ~θησᾰν";
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
    perfect_ind_pass_1: _, _, "ἐ~μεθᾰ";
    perfect_ind_pass_2: _, _, "ἐ~σθε";
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
    perfect_imp_pass_2: "ἐ~σο", _, _;
    perfect_imp_pass_3: "ἐ~σθω", "ἐ~σθων", _;
    pluperfect_act_1: "ἐ~κειν", _, "ἐ~κεμεν";
    pluperfect_act_2: "ἐ~κεις", "ἐ~κετον", "ἐ~κετε";
    pluperfect_act_3: "ἐ~κει", "ἐ~κετην", "ἐ~κεσᾰν";
    pluperfect_mid_1: "ἐ~μην", _, "ἐ~μεθᾰ";
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", "ἐ~σθε";
    pluperfect_mid_3: "ἐ~το", "ἐ~σθην", _;
    pluperfect_pass_1: _, _, "ἐ~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~σθε";
    pluperfect_pass_3: _, "ἐ~σθην", "ἐ~ντο";
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
    // Βουλευω: 70 words (ἀκουω, θερᾰπευω, πῐστευω, νηστευω, χορευω, παιδευω). Like Γῡροω, except in 299 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("βουλευω", 70, "~υω", like Γῡροω;
    ind_act_1: "~υω", _, "~υομεν";
    ind_act_2: "~υεις", _, _;
    ind_act_3: "~υει", _, "~υουσῐ|~υουσῐν";
    ind_mid_1: "~υομαι", _, "~υομεθᾰ";
    ind_mid_2: "~υῃ|~υει", "~υεσθον", "~υεσθε";
    ind_mid_3: "~υεται", _, "~υονται";
    ind_pass_1: _, _, "~υομεθᾰ";
    ind_pass_2: "~υῃ|~υει", _, "~υεσθε";
    ind_pass_3: "~υεται", _, "~υονται";
    subj_act_1: "~υω", _, "~υωμεν";
    subj_act_2: "~υῃς", "~υητον", "~υητε";
    subj_act_3: "~υῃ", _, "~υωσῐ|~υωσῐν";
    subj_mid_1: "~υωμαι", _, "~υωμεθᾰ";
    subj_mid_2: "~υῃ", "~υησθον", "~υησθε";
    subj_mid_3: "~υηται", _, "~υωνται";
    subj_pass_1: _, _, "~υωμεθᾰ";
    subj_pass_2: _, _, "~υησθε";
    subj_pass_3: "~υηται", _, "~υωνται";
    opt_act_1: "~υοιμῐ", _, "~υοιμεν";
    opt_act_2: "~υοις", "~υοιτον", "~υοιτε";
    opt_act_3: "~υοι", "~υοιτην", "~υοιεν";
    opt_mid_1: "~υοιμην", _, "~υοιμεθᾰ";
    opt_mid_2: "~υοιο", "~υοισθον", "~υοισθε";
    opt_mid_3: "~υοιτο", "~υοισθην", "~υοιντο";
    opt_pass_1: _, _, "~υοιμεθᾰ";
    opt_pass_2: _, "~υοισθον", "~υοισθε";
    opt_pass_3: "~υοιτο", "~υοισθην", "~υοιντο";
    imp_act_2: "~υε", "~υετον", "~υετε";
    imp_act_3: "~υετω", "~υετων", "~υοντων";
    imp_mid_2: "~υου", _, "~υεσθε";
    imp_mid_3: "~υεσθω", "~υεσθων", "~υεσθων";
    imp_pass_2: _, _, "~υεσθε";
    imp_pass_3: _, "~υεσθων", _;
    imperfect_act_1: "ἐ~υον", _, "ἐ~υομεν";
    imperfect_act_2: "ἐ~υες", "ἐ~υετον", "ἐ~υετε";
    imperfect_act_3: "ἐ~υε|ἐ~υεν", "ἐ~υετην", "ἐ~υον";
    imperfect_mid_1: "ἐ~υομην", _, "ἐ~υομεθᾰ";
    imperfect_mid_2: "ἐ~υου", "ἐ~υεσθον", "ἐ~υεσθε";
    imperfect_mid_3: "ἐ~υετο", "ἐ~υεσθην", "ἐ~υοντο";
    imperfect_pass_1: _, _, "ἐ~υομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~υεσθε";
    imperfect_pass_3: "ἐ~υετο", _, _;
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
    aorist_ind_act_1: "ἐ~υσᾰ", _, "ἐ~υσᾰμεν";
    aorist_ind_act_2: "ἐ~υσᾰς", "ἐ~υσᾰτον", "ἐ~υσᾰτε";
    aorist_ind_act_3: "ἐ~υσε|ἐ~υσεν", "ἐ~υσᾰτην", "ἐ~υσᾰν";
    aorist_ind_mid_1: "ἐ~υσᾰμην", _, "ἐ~υσᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~υσω", "ἐ~υσᾰσθον", "ἐ~υσᾰσθε";
    aorist_ind_mid_3: "ἐ~υσᾰτο", "ἐ~υσᾰσθην", "ἐ~υσᾰντο";
    aorist_ind_pass_1: "ἐ~υθην", _, "ἐ~υθημεν";
    aorist_ind_pass_2: "ἐ~υθης", "ἐ~υθητον", "ἐ~υθητε";
    aorist_ind_pass_3: "ἐ~υθη", "ἐ~υθητην", "ἐ~υθησᾰν";
    aorist_subj_act_1: "~υσω", _, "~υσωμεν";
    aorist_subj_act_2: "~υσῃς", "~υσητον", "~υσητε";
    aorist_subj_act_3: "~υσῃ", _, "~υσωσῐ|~υσωσῐν";
    aorist_subj_mid_1: "~υσωμαι", _, "~υσωμεθᾰ";
    aorist_subj_mid_2: "~υσῃ", "~υσησθον", "~υσησθε";
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
    perfect_ind_pass_1: _, _, "%ε~υμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~υσθε";
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
    perfect_imp_pass_2: "%ε~υσο", _, _;
    perfect_imp_pass_3: "%ε~υσθω", "%ε~υσθων", _;
    pluperfect_act_1: "ἐ%ε~υκειν", _, "ἐ%ε~υκεμεν";
    pluperfect_act_2: "ἐ%ε~υκεις", "ἐ%ε~υκετον", "ἐ%ε~υκετε";
    pluperfect_act_3: "ἐ%ε~υκει", "ἐ%ε~υκετην", "ἐ%ε~υκεσᾰν";
    pluperfect_mid_1: "ἐ%ε~υμην", _, "ἐ%ε~υμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~υσο", "ἐ%ε~υσθον", "ἐ%ε~υσθε";
    pluperfect_mid_3: "ἐ%ε~υτο", "ἐ%ε~υσθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~υμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~υσθε";
    pluperfect_pass_3: _, "ἐ%ε~υσθην", "ἐ%ε~υντο";
    infinitive_pres: "~υειν", "~υεσθαι", _;
    infinitive_fut: "~υσειν", "~υσεσθαι", "~υθησεσθαι";
    infinitive_aor: "~ῡσαι", "~υσᾰσθαι", "~υθηναι";
    infinitive_perf: "%ε~υκεναι", "%ε~ῡσθαι", _;
    participle_pres: "~υων", "~υομενος", _;
    participle_fut: "~υσων", "~υσομενος", "~υθησομενος";
    participle_aor: "~υσᾱς", "~υσᾰμενος", "~υθεις";
    participle_perf: "%ε~υκως", "%ε~υμενος", _;
    ),
    // Δοκᾰω: 58 words (ἀγᾰπᾰω, γεννᾰω, κῠβερνᾰω, γλᾰγᾰω, σῑγᾰω, δαμναω). Like Ἐχω, except in 88 cells: ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ind_mid_1 pl, ….
    conjugation!("δοκᾰω", 58, "~ᾰω", like Ἐχω;
    ind_act_1: _, _, "~ωμεν";
    ind_act_2: "~ᾳς", _, _;
    ind_act_3: "~ᾳ", _, "~ωσῐ|~ωσῐν";
    ind_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    ind_mid_2: "~ᾳ", "~ᾱσθον", "~ᾱσθε";
    ind_mid_3: "~ᾱται", _, "~ωνται";
    ind_pass_1: _, _, "~ωμεθᾰ";
    ind_pass_2: _, _, "~ᾱσθε";
    ind_pass_3: "~ᾱται", _, "~ωνται";
    subj_act_2: "~ᾳς", "~ᾱτον", "~ᾱτε";
    subj_act_3: "~ᾳ", _, "~ωσῐ|~ωσῐν";
    subj_mid_2: "~ᾳ", "~ᾱσθον", "~ᾱσθε";
    subj_mid_3: "~ᾱται", _, _;
    subj_pass_2: _, _, "~ᾱσθε";
    subj_pass_3: "~ᾱται", _, _;
    opt_act_1: "~ῳην|~ῳμῐ", _, "~ῳμεν|~ῳημεν";
    opt_act_2: "~ῳης|~ῳς", "~ῳτον|~ῳητον", "~ῳτε|~ῳητε";
    opt_act_3: "~ῳη|~ῳ", "~ῳτην|~ῳητην", "~ῳεν|~ῳησᾰν";
    opt_mid_1: "~ῳμην", _, "~ῳμεθᾰ";
    opt_mid_2: "~ῳο", "~ῳσθον", "~ῳσθε";
    opt_mid_3: "~ῳτο", "~ῳσθην", "~ῳντο";
    opt_pass_1: _, _, "~ῳμεθᾰ";
    opt_pass_2: _, "~ῳσθον", "~ῳσθε";
    opt_pass_3: "~ῳτο", "~ῳσθην", "~ῳντο";
    imp_act_2: "~ᾱ", "~ᾱτον", "~ᾱτε";
    imp_act_3: "~ᾱτω", "~ᾱτων", "~ωντων";
    imp_mid_2: "~ω", _, "~ᾱσθε";
    imp_mid_3: "~ᾱσθω", "~ᾱσθων", "~ᾱσθων";
    imp_pass_2: _, _, "~ᾱσθε";
    imp_pass_3: _, "~ᾱσθων", _;
    imperfect_act_1: "ἐ~ων", _, "ἐ~ωμεν";
    imperfect_act_2: "ἐ~ᾱς", "ἐ~ᾱτον", "ἐ~ᾱτε";
    imperfect_act_3: "ἐ~ᾱ", "ἐ~ᾱτην", "ἐ~ων";
    imperfect_mid_1: "ἐ~ωμην", _, "ἐ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ω", "ἐ~ᾱσθον", "ἐ~ᾱσθε";
    imperfect_mid_3: "ἐ~ᾱτο", "ἐ~ᾱσθην", "ἐ~ωντο";
    imperfect_pass_1: _, _, "ἐ~ωμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ᾱσθε";
    imperfect_pass_3: "ἐ~ᾱτο", _, _;
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    aorist_subj_act_3: _, _, "~ησωσῐ|~ησωσῐν";
    aorist_opt_act_2: "~ησειᾰς|~ησαις", _, _;
    aorist_opt_act_3: "~ησειε|~ησειεν|~ησαι", _, "~ησειᾰν|~ησαιεν";
    infinitive_pres: "~ᾱν", "~ᾱσθαι", _;
    participle_pres: _, "~ωμενος", _;
    ),
    // Ἑπομαι: 52 words (μαχομαι, ἐρχομαι, ἰᾰομαι, πῠνθανομαι, ἑπομαι, ἑζομαι). Like Σπᾰω, except in 253 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, subj_act_1 sg, ….
    conjugation!("ἑπομαι", 52, "~ομαι", like Σπᾰω;
    ind_act_1: "~ομαι", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, _, derived;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imp_act_3: derived, derived, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    imperfect_mid_1: "~ομην", _, "~ομεθᾰ";
    imperfect_mid_2: "~ου", "~εσθον", "~εσθε";
    imperfect_mid_3: "~ετο", "~εσθην", "~οντο";
    imperfect_pass_1: _, _, "~ομεθᾰ";
    imperfect_pass_2: _, _, "~εσθε";
    imperfect_pass_3: "~ετο", _, _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, derived;
    future_ind_mid_3: derived, _, derived;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, derived, derived;
    future_opt_mid_3: derived, derived, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_ind_mid_1: "~ομην", _, "~ομεθᾰ";
    aorist_ind_mid_2: "~ου", "~εσθον", "~εσθε";
    aorist_ind_mid_3: "~ετο", "~εσθην", "~οντο";
    aorist_ind_pass_1: derived, _, derived;
    aorist_ind_pass_2: derived, derived, derived;
    aorist_ind_pass_3: derived, derived, derived;
    aorist_subj_act_1: derived, _, derived;
    aorist_subj_act_2: derived, derived, derived;
    aorist_subj_act_3: derived, _, derived;
    aorist_subj_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    aorist_subj_mid_2: "~ῃ", "~ησθον", "~ησθε";
    aorist_subj_mid_3: "~ηται", _, "~ωνται";
    aorist_subj_pass_1: derived, _, derived;
    aorist_subj_pass_2: derived, derived, derived;
    aorist_subj_pass_3: derived, _, derived;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    aorist_opt_mid_2: "~οιο", "~οισθον", "~οισθε";
    aorist_opt_mid_3: "~οιτο", "~οισθην", "~οιντο";
    aorist_opt_pass_1: derived, _, derived;
    aorist_opt_pass_2: derived, derived, derived;
    aorist_opt_pass_3: derived, derived, derived;
    aorist_imp_act_2: derived, derived, derived;
    aorist_imp_act_3: derived, derived, derived;
    aorist_imp_mid_2: "~οῡ", "~εσθον", "~εσθε";
    aorist_imp_mid_3: "~εσθω", "~εσθων", _;
    aorist_imp_pass_2: derived, _, _;
    aorist_imp_pass_3: derived, derived, derived;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: derived, derived, derived;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, derived;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: _, derived, derived;
    perfect_imp_act_3: derived, derived, derived;
    perfect_imp_mid_2: derived, _, _;
    perfect_imp_mid_3: derived, derived, _;
    perfect_imp_pass_2: derived, _, _;
    perfect_imp_pass_3: derived, derived, _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, derived, derived;
    infinitive_aor: derived, "~εσθαι", derived;
    infinitive_perf: derived, derived, _;
    participle_pres: derived, _, _;
    participle_fut: derived, derived, derived;
    participle_aor: derived, "~ομενος", derived;
    participle_perf: derived, derived, _;
    ),
    // Δῐκεω: 49 words (ἀγᾰθοποιεω, ἀγνοεω, ἀντλεω, ἀσκεω, οἰκεω, ἀλεω). Like Ποιεω, except in 97 cells: future_ind_act_3 pl, future_ind_mid_2 sg, future_ind_pass_2 sg, aorist_ind_act_3 sg, aorist_subj_act_3 pl, aorist_subj_pass_3 pl, ….
    conjugation!("δῐκεω", 49, "~εω", like Ποιεω;
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    future_ind_mid_2: "~ησῃ|~ησει", _, _;
    future_ind_pass_2: "~ηθησῃ|~ηθησει", _, _;
    aorist_ind_act_3: "ἐ~ησε|ἐ~ησεν", _, _;
    aorist_subj_act_3: _, _, "~ησωσῐ|~ησωσῐν";
    aorist_subj_pass_3: _, _, "~ηθωσῐ|~ηθωσῐν";
    aorist_opt_act_2: "~ησειᾰς|~ησαις", _, _;
    aorist_opt_act_3: "~ησειε|~ησειεν|~ησαι", _, "~ησειᾰν|~ησαιεν";
    aorist_opt_pass_1: _, _, "~ηθεῑμεν|~ηθειημεν";
    aorist_opt_pass_2: _, "~ηθεῑτον|~ηθειητον", "~ηθεῑτε|~ηθειητε";
    aorist_opt_pass_3: _, "~ηθειτην|~ηθειητην", "~ηθεῑεν|~ηθειησᾰν";
    perfect_ind_act_1: "ἐ~ηκᾰ", _, "ἐ~ηκᾰμεν";
    perfect_ind_act_2: "ἐ~ηκᾰς", "ἐ~ηκᾰτον", "ἐ~ηκᾰτε";
    perfect_ind_act_3: "ἐ~ηκε|ἐ~ηκεν", _, "ἐ~ηκᾱσῐ|ἐ~ηκᾱσῐν";
    perfect_ind_mid_1: "ἐ~ημαι", _, "ἐ~ημεθᾰ";
    perfect_ind_mid_2: "ἐ~ησαι", "ἐ~ησθον", "ἐ~ησθε";
    perfect_ind_mid_3: "ἐ~ηται", _, "ἐ~ηνται";
    perfect_ind_pass_1: _, _, "ἐ~ημεθᾰ";
    perfect_ind_pass_2: _, _, "ἐ~ησθε";
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
    perfect_imp_pass_2: "ἐ~ησο", _, _;
    perfect_imp_pass_3: "ἐ~ησθω", "ἐ~ησθων", _;
    pluperfect_act_1: "ἐ~ηκειν|ἐ~ηκη", _, "ἐ~ηκεμεν";
    pluperfect_act_2: "ἐ~ηκεις|ἐ~ηκης", "ἐ~ηκετον", "ἐ~ηκετε";
    pluperfect_act_3: "ἐ~ηκει|ἐ~ηκειν", "ἐ~ηκετην", "ἐ~ηκεσᾰν";
    pluperfect_mid_1: "ἐ~ημην", _, "ἐ~ημεθᾰ";
    pluperfect_mid_2: "ἐ~ησο", "ἐ~ησθον", "ἐ~ησθε";
    pluperfect_mid_3: "ἐ~ητο", "ἐ~ησθην", _;
    pluperfect_pass_1: _, _, "ἐ~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ησθε";
    pluperfect_pass_3: _, "ἐ~ησθην", "ἐ~ηντο";
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
    // Προσερχομαι: 43 words (γεινομαι, δεχομαι, δερκομαι, μεμφομαι, σεβομαι, φθεγγομαι).
    conjugation!("προσερχομαι", 43, "~ε~ομαι";
    ind_act_1: "~ε~ομαι", _, _;
    ind_mid_1: "~ε~ομαι", _, "~ε~ομεθᾰ";
    ind_mid_2: "~ε~ῃ", "~ε~εσθον", "~ε~εσθε";
    ind_mid_3: "~ε~εται", _, "~ε~ονται";
    ind_pass_1: _, _, "~ε~ομεθᾰ";
    ind_pass_2: _, _, "~ε~εσθε";
    ind_pass_3: "~ε~εται", _, "~ε~ονται";
    subj_mid_1: "~ε~ωμαι", _, "~ε~ωμεθᾰ";
    subj_mid_2: "~ε~ῃ", "~ε~ησθον", "~ε~ησθε";
    subj_mid_3: "~ε~ηται", _, "~ε~ωνται";
    subj_pass_1: _, _, "~ε~ωμεθᾰ";
    subj_pass_2: _, _, "~ε~ησθε";
    subj_pass_3: "~ε~ηται", _, "~ε~ωνται";
    opt_mid_1: "~ε~οιμην", _, "~ε~οιμεθᾰ";
    opt_mid_2: "~ε~οιο", "~ε~οισθον", "~ε~οισθε";
    opt_mid_3: "~ε~οιτο", "~ε~οισθην", "~ε~οιντο";
    opt_pass_1: _, _, "~ε~οιμεθᾰ";
    opt_pass_2: _, "~ε~οισθον", "~ε~οισθε";
    opt_pass_3: "~ε~οιτο", "~ε~οισθην", "~ε~οιντο";
    imp_mid_2: "~ε~ου", _, "~ε~εσθε";
    imp_mid_3: "~ε~εσθω", "~ε~εσθων", "~ε~εσθων";
    imp_pass_2: _, _, "~ε~εσθε";
    imp_pass_3: _, "~ε~εσθων", _;
    imperfect_mid_1: "ἐ~ε~ομην", _, "ἐ~ε~ομεθᾰ";
    imperfect_mid_2: "ἐ~ε~ου", "ἐ~ε~εσθον", "ἐ~ε~εσθε";
    imperfect_mid_3: "ἐ~ε~ετο", "ἐ~ε~εσθην", "ἐ~ε~οντο";
    imperfect_pass_1: _, _, "ἐ~ε~ομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ε~εσθε";
    imperfect_pass_3: "ἐ~ε~ετο", _, _;
    future_ind_mid_1: "~ε~ησομαι", _, "~ε~ησομεθᾰ";
    future_ind_mid_2: "~ε~ησῃ", "~ε~ησεσθον", "~ε~ησεσθε";
    future_ind_mid_3: "~ε~ησεται", _, "~ε~ησονται";
    future_ind_pass_1: "~ε~ασθησομαι", _, "~ε~ασθησομεθᾰ";
    future_ind_pass_2: "~ε~ασθησῃ", "~ε~ασθησεσθον", "~ε~ασθησεσθε";
    future_ind_pass_3: "~ε~ασθησεται", _, "~ε~ασθησονται";
    future_opt_mid_1: "~ε~ησοιμην", _, "~ε~ησοιμεθᾰ";
    future_opt_mid_2: "~ε~ησοιο", "~ε~ησοισθον", "~ε~ησοισθε";
    future_opt_mid_3: "~ε~ησοιτο", "~ε~ησοισθην", "~ε~ησοιντο";
    future_opt_pass_1: "~ε~ασθησοιμην", _, "~ε~ασθησοιμεθᾰ";
    future_opt_pass_2: "~ε~ασθησοιο", "~ε~ασθησοισθον", "~ε~ασθησοισθε";
    future_opt_pass_3: "~ε~ασθησοιτο", "~ε~ασθησοισθην", "~ε~ασθησοιντο";
    aorist_ind_mid_1: "ἐ~~ομην", _, "ἐ~~ομεθᾰ";
    aorist_ind_mid_2: "ἐ~~ου", "ἐ~~εσθον", "ἐ~~εσθε";
    aorist_ind_mid_3: "ἐ~~ετο", "ἐ~~εσθην", "ἐ~~οντο";
    aorist_ind_pass_1: "ἐ~ε~ασθην", _, "ἐ~ε~ασθημεν";
    aorist_ind_pass_2: "ἐ~ε~ασθης", "ἐ~ε~ασθητον", "ἐ~ε~ασθητε";
    aorist_ind_pass_3: "ἐ~ε~ασθη", "ἐ~ε~ασθητην", "ἐ~ε~ασθησᾰν";
    aorist_subj_mid_1: "~~ωμαι", _, "~~ωμεθᾰ";
    aorist_subj_mid_2: "~~ῃ", "~~ησθον", "~~ησθε";
    aorist_subj_mid_3: "~~ηται", _, "~~ωνται";
    aorist_subj_pass_1: "~ε~ασθω", _, "~ε~ασθωμεν";
    aorist_subj_pass_2: "~ε~ασθῃς", "~ε~ασθητον", "~ε~ασθητε";
    aorist_subj_pass_3: "~ε~ασθῃ", _, "~ε~ασθωσῐ";
    aorist_opt_mid_1: "~~οιμην", _, "~~οιμεθᾰ";
    aorist_opt_mid_2: "~~οῑο", "~~οῑσθον", "~~οῑσθε";
    aorist_opt_mid_3: "~~οῑτο", "~~οισθην", "~~οῑντο";
    aorist_opt_pass_1: "~ε~ασθειην", _, "~ε~ασθεῑμεν";
    aorist_opt_pass_2: "~ε~ασθειης", "~ε~ασθεῑτον", "~ε~ασθεῑτε";
    aorist_opt_pass_3: "~ε~ασθειη", "~ε~ασθειτην", "~ε~ασθεῑεν";
    aorist_imp_mid_2: "~~οῡ", "~~εσθον", "~~εσθε";
    aorist_imp_mid_3: "~~εσθω", "~~εσθων", _;
    aorist_imp_pass_2: "~ε~ασθητῐ", _, _;
    aorist_imp_pass_3: "~ε~ασθητω", "~ε~ασθητων", "~ε~ασθεντων";
    perfect_ind_mid_1: "~επο~ημαι", _, "~επο~ημεθᾰ";
    perfect_ind_mid_2: "~επο~ησαι", "~επο~ησθον", "~επο~ησθε";
    perfect_ind_mid_3: "~επο~ηται", _, "~επο~ηνται";
    perfect_ind_pass_1: _, _, "~επο~ημεθᾰ";
    perfect_ind_pass_2: _, _, "~επο~ησθε";
    perfect_imp_mid_2: "~επο~ησο", _, _;
    perfect_imp_mid_3: "~επο~ησθω", "~επο~ησθων", _;
    perfect_imp_pass_2: "~επο~ησο", _, _;
    perfect_imp_pass_3: "~επο~ησθω", "~επο~ησθων", _;
    infinitive_pres: _, "~ε~εσθαι", _;
    infinitive_fut: _, "~ε~ησεσθαι", "~ε~ασθησεσθαι";
    infinitive_aor: _, "~~εσθαι", "~ε~ασθηναι";
    infinitive_perf: _, "~επο~ησθαι", _;
    participle_pres: _, "~ε~ομενος", _;
    participle_fut: _, "~ε~ησομενος", "~ε~ασθησομενος";
    participle_aor: _, "~~ομενος", "~ε~ασθεις";
    participle_perf: _, "~επο~ημενος", _;
    ),
    // Τᾰσσω: 36 words (πῠρεσσω, ἀϊσσω, πλησσω, τᾰσσω, μᾰσσω, φῠλᾰσσω). Like Γῡροω, except in 299 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("τᾰσσω", 36, "~σσω", like Γῡροω;
    ind_act_1: "~σσω", _, "~σσομεν";
    ind_act_2: "~σσεις", _, _;
    ind_act_3: "~σσει", _, "~σσουσῐ|~σσουσῐν";
    ind_mid_1: "~σσομαι", _, "~σσομεθᾰ";
    ind_mid_2: "~σσῃ|~σσει", "~σσεσθον", "~σσεσθε";
    ind_mid_3: "~σσεται", _, "~σσονται";
    ind_pass_1: _, _, "~σσομεθᾰ";
    ind_pass_2: "~σσῃ|~σσει", _, "~σσεσθε";
    ind_pass_3: "~σσεται", _, "~σσονται";
    subj_act_1: "~σσω", _, "~σσωμεν";
    subj_act_2: "~σσῃς", "~σσητον", "~σσητε";
    subj_act_3: "~σσῃ", _, "~σσωσῐ|~σσωσῐν";
    subj_mid_1: "~σσωμαι", _, "~σσωμεθᾰ";
    subj_mid_2: "~σσῃ", "~σσησθον", "~σσησθε";
    subj_mid_3: "~σσηται", _, "~σσωνται";
    subj_pass_1: _, _, "~σσωμεθᾰ";
    subj_pass_2: _, _, "~σσησθε";
    subj_pass_3: "~σσηται", _, "~σσωνται";
    opt_act_1: "~σσοιμῐ", _, "~σσοιμεν";
    opt_act_2: "~σσοις", "~σσοιτον", "~σσοιτε";
    opt_act_3: "~σσοι", "~σσοιτην", "~σσοιεν";
    opt_mid_1: "~σσοιμην", _, "~σσοιμεθᾰ";
    opt_mid_2: "~σσοιο", "~σσοισθον", "~σσοισθε";
    opt_mid_3: "~σσοιτο", "~σσοισθην", "~σσοιντο";
    opt_pass_1: _, _, "~σσοιμεθᾰ";
    opt_pass_2: _, "~σσοισθον", "~σσοισθε";
    opt_pass_3: "~σσοιτο", "~σσοισθην", "~σσοιντο";
    imp_act_2: "~σσε", "~σσετον", "~σσετε";
    imp_act_3: "~σσετω", "~σσετων", "~σσοντων";
    imp_mid_2: "~σσου", _, "~σσεσθε";
    imp_mid_3: "~σσεσθω", "~σσεσθων", "~σσεσθων";
    imp_pass_2: _, _, "~σσεσθε";
    imp_pass_3: _, "~σσεσθων", _;
    imperfect_act_1: "ἐ~σσον", _, "ἐ~σσομεν";
    imperfect_act_2: "ἐ~σσες", "ἐ~σσετον", "ἐ~σσετε";
    imperfect_act_3: "ἐ~σσε|ἐ~σσεν", "ἐ~σσετην", "ἐ~σσον";
    imperfect_mid_1: "ἐ~σσομην", _, "ἐ~σσομεθᾰ";
    imperfect_mid_2: "ἐ~σσου", "ἐ~σσεσθον", "ἐ~σσεσθε";
    imperfect_mid_3: "ἐ~σσετο", "ἐ~σσεσθην", "ἐ~σσοντο";
    imperfect_pass_1: _, _, "ἐ~σσομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~σσεσθε";
    imperfect_pass_3: "ἐ~σσετο", _, _;
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
    aorist_ind_act_1: "ἐ~ξᾰ", _, "ἐ~ξᾰμεν";
    aorist_ind_act_2: "ἐ~ξᾰς", "ἐ~ξᾰτον", "ἐ~ξᾰτε";
    aorist_ind_act_3: "ἐ~ξε|ἐ~ξεν", "ἐ~ξᾰτην", "ἐ~ξᾰν";
    aorist_ind_mid_1: "ἐ~ξᾰμην", _, "ἐ~ξᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ξω", "ἐ~ξᾰσθον", "ἐ~ξᾰσθε";
    aorist_ind_mid_3: "ἐ~ξᾰτο", "ἐ~ξᾰσθην", "ἐ~ξᾰντο";
    aorist_ind_pass_1: "ἐ~χθην", _, "ἐ~χθημεν";
    aorist_ind_pass_2: "ἐ~χθης", "ἐ~χθητον", "ἐ~χθητε";
    aorist_ind_pass_3: "ἐ~χθη", "ἐ~χθητην", "ἐ~χθησᾰν";
    aorist_subj_act_1: "~ξω", _, "~ξωμεν";
    aorist_subj_act_2: "~ξῃς", "~ξητον", "~ξητε";
    aorist_subj_act_3: "~ξῃ", _, "~ξωσῐ|~ξωσῐν";
    aorist_subj_mid_1: "~ξωμαι", _, "~ξωμεθᾰ";
    aorist_subj_mid_2: "~ξῃ", "~ξησθον", "~ξησθε";
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
    perfect_ind_pass_1: _, _, "%ε~γμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~χθε";
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
    perfect_imp_pass_2: "%ε~ξο", _, _;
    perfect_imp_pass_3: "%ε~χθω", "%ε~χθων", _;
    pluperfect_act_1: "ἐ%ε~χειν", _, "ἐ%ε~χεμεν";
    pluperfect_act_2: "ἐ%ε~χεις", "ἐ%ε~χετον", "ἐ%ε~χετε";
    pluperfect_act_3: "ἐ%ε~χει", "ἐ%ε~χετην", "ἐ%ε~χεσᾰν";
    pluperfect_mid_1: "ἐ%ε~γμην", _, "ἐ%ε~γμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ξο", "ἐ%ε~χθον", "ἐ%ε~χθε";
    pluperfect_mid_3: "ἐ%ε~κτο", "ἐ%ε~χθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~γμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~χθε";
    pluperfect_pass_3: _, "ἐ%ε~χθην", "ἐ%ε~γᾰτο";
    infinitive_pres: "~σσειν", "~σσεσθαι", _;
    infinitive_fut: "~ξειν", "~ξεσθαι", "~χθησεσθαι";
    infinitive_aor: "~ξαι", "~ξᾰσθαι", "~χθηναι";
    infinitive_perf: "%ε~χεναι", "%ε~χθαι", _;
    participle_pres: "~σσων", "~σσομενος", _;
    participle_fut: "~ξων", "~ξομενος", "~χθησομενος";
    participle_aor: "~ξᾱς", "~ξᾰμενος", "~χθεις";
    participle_perf: "%ε~χως", "%ε~γμενος", _;
    ),
    // Κᾰτεργαζομαι: 31 words (λογῐζομαι, χᾰρῐζομαι, στοχᾰζομαι, σῠνεργαζομαι, ἐργᾰζομαι, ἀνᾰχρονῐζομαι). Like Ἀφορῐζω, except in 32 cells: ind_act_3 pl, ind_mid_2 sg, ind_pass_2 sg, subj_act_3 pl, future_ind_act_3 pl, future_ind_mid_2 sg, ….
    conjugation!("κᾰτεργαζομαι", 31, "~ζομαι", like Ἀφορῐζω;
    ind_act_3: _, _, "~ζουσῐ";
    ind_mid_2: "~ζῃ", _, _;
    ind_pass_2: derived, _, _;
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
    // Ληγω: 31 words (ἀγω, λεγω, φευγω, τρωγω, δῐᾰλεγω, φλεγω). Like Τᾰσσω, except in 148 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ληγω", 31, "~γω", like Τᾰσσω;
    ind_act_1: "~γω", _, "~γομεν";
    ind_act_2: "~γεις", _, _;
    ind_act_3: "~γει", _, "~γουσῐ";
    ind_mid_1: "~γομαι", _, "~γομεθᾰ";
    ind_mid_2: "~γῃ", "~γεσθον", "~γεσθε";
    ind_mid_3: "~γεται", _, "~γονται";
    ind_pass_1: _, _, "~γομεθᾰ";
    ind_pass_2: derived, _, "~γεσθε";
    ind_pass_3: "~γεται", _, "~γονται";
    subj_act_1: "~γω", _, "~γωμεν";
    subj_act_2: "~γῃς", "~γητον", "~γητε";
    subj_act_3: "~γῃ", _, "~γωσῐ";
    subj_mid_1: "~γωμαι", _, "~γωμεθᾰ";
    subj_mid_2: "~γῃ", "~γησθον", "~γησθε";
    subj_mid_3: "~γηται", _, "~γωνται";
    subj_pass_1: _, _, "~γωμεθᾰ";
    subj_pass_2: _, _, "~γησθε";
    subj_pass_3: "~γηται", _, "~γωνται";
    opt_act_1: "~γοιμῐ", _, "~γοιμεν";
    opt_act_2: "~γοις", "~γοιτον", "~γοιτε";
    opt_act_3: "~γοι", "~γοιτην", "~γοιεν";
    opt_mid_1: "~γοιμην", _, "~γοιμεθᾰ";
    opt_mid_2: "~γοιο", "~γοισθον", "~γοισθε";
    opt_mid_3: "~γοιτο", "~γοισθην", "~γοιντο";
    opt_pass_1: _, _, "~γοιμεθᾰ";
    opt_pass_2: _, "~γοισθον", "~γοισθε";
    opt_pass_3: "~γοιτο", "~γοισθην", "~γοιντο";
    imp_act_2: "~γε", "~γετον", "~γετε";
    imp_act_3: "~γετω", "~γετων", "~γοντων";
    imp_mid_2: "~γου", _, "~γεσθε";
    imp_mid_3: "~γεσθω", "~γεσθων", "~γεσθων";
    imp_pass_2: _, _, "~γεσθε";
    imp_pass_3: _, "~γεσθων", _;
    imperfect_act_1: "ἐ~γον", _, "ἐ~γομεν";
    imperfect_act_2: "ἐ~γες", "ἐ~γετον", "ἐ~γετε";
    imperfect_act_3: "ἐ~γε", "ἐ~γετην", "ἐ~γον";
    imperfect_mid_1: "ἐ~γομην", _, "ἐ~γομεθᾰ";
    imperfect_mid_2: "ἐ~γου", "ἐ~γεσθον", "ἐ~γεσθε";
    imperfect_mid_3: "ἐ~γετο", "ἐ~γεσθην", "ἐ~γοντο";
    imperfect_pass_1: _, _, "ἐ~γομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~γεσθε";
    imperfect_pass_3: "ἐ~γετο", _, _;
    future_ind_act_3: _, _, "~ξουσῐ";
    future_ind_mid_2: "~ξῃ", _, _;
    aorist_ind_act_3: "ἐ~ξε", _, _;
    aorist_subj_act_3: _, _, "~ξωσῐ";
    aorist_opt_act_2: "~ξειᾰς", _, _;
    aorist_opt_act_3: "~ξειε", _, "~ξειᾰν";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: derived, derived, derived;
    perfect_ind_act_3: derived, _, derived;
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: _, derived, derived;
    perfect_imp_act_3: derived, derived, derived;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
    infinitive_pres: "~γειν", "~γεσθαι", _;
    infinitive_perf: derived, _, _;
    participle_pres: "~γων", "~γομενος", _;
    participle_perf: derived, _, _;
    ),
    // Ἁπτω: 28 words (κρῠπτω, κᾰμπτω, κοπτω, τῠπτω, ἁπτω, κῠπτω). Like Ὁρῐζω, except in 304 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἁπτω", 28, "~πτω", like Ὁρῐζω;
    ind_act_1: "~πτω", _, "~πτομεν";
    ind_act_2: "~πτεις", _, _;
    ind_act_3: "~πτει", _, "~πτουσῐ";
    ind_mid_1: "~πτομαι", _, "~πτομεθᾰ";
    ind_mid_2: "~πτῃ", "~πτεσθον", "~πτεσθε";
    ind_mid_3: "~πτεται", _, "~πτονται";
    ind_pass_1: _, _, "~πτομεθᾰ";
    ind_pass_2: _, _, "~πτεσθε";
    ind_pass_3: "~πτεται", _, "~πτονται";
    subj_act_1: "~πτω", _, "~πτωμεν";
    subj_act_2: "~πτῃς", "~πτητον", "~πτητε";
    subj_act_3: "~πτῃ", _, "~πτωσῐ";
    subj_mid_1: "~πτωμαι", _, "~πτωμεθᾰ";
    subj_mid_2: "~πτῃ", "~πτησθον", "~πτησθε";
    subj_mid_3: "~πτηται", _, "~πτωνται";
    subj_pass_1: _, _, "~πτωμεθᾰ";
    subj_pass_2: _, _, "~πτησθε";
    subj_pass_3: "~πτηται", _, "~πτωνται";
    opt_act_1: "~πτοιμῐ", _, "~πτοιμεν";
    opt_act_2: "~πτοις", "~πτοιτον", "~πτοιτε";
    opt_act_3: "~πτοι", "~πτοιτην", "~πτοιεν";
    opt_mid_1: "~πτοιμην", _, "~πτοιμεθᾰ";
    opt_mid_2: "~πτοιο", "~πτοισθον", "~πτοισθε";
    opt_mid_3: "~πτοιτο", "~πτοισθην", "~πτοιντο";
    opt_pass_1: _, _, "~πτοιμεθᾰ";
    opt_pass_2: _, "~πτοισθον", "~πτοισθε";
    opt_pass_3: "~πτοιτο", "~πτοισθην", "~πτοιντο";
    imp_act_2: "~πτε", "~πτετον", "~πτετε";
    imp_act_3: "~πτετω", "~πτετων", "~πτοντων";
    imp_mid_2: "~πτου", _, "~πτεσθε";
    imp_mid_3: "~πτεσθω", "~πτεσθων", "~πτεσθων";
    imp_pass_2: _, _, "~πτεσθε";
    imp_pass_3: _, "~πτεσθων", _;
    imperfect_act_1: "ἐ~πτον", _, "ἐ~πτομεν";
    imperfect_act_2: "ἐ~πτες", "ἐ~πτετον", "ἐ~πτετε";
    imperfect_act_3: "ἐ~πτε", "ἐ~πτετην", "ἐ~πτον";
    imperfect_mid_1: "ἐ~πτομην", _, "ἐ~πτομεθᾰ";
    imperfect_mid_2: "ἐ~πτου", "ἐ~πτεσθον", "ἐ~πτεσθε";
    imperfect_mid_3: "ἐ~πτετο", "ἐ~πτεσθην", "ἐ~πτοντο";
    imperfect_pass_1: _, _, "ἐ~πτομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~πτεσθε";
    imperfect_pass_3: "ἐ~πτετο", _, _;
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
    aorist_ind_act_1: "ἐ~ψᾰ", _, "ἐ~ψᾰμεν";
    aorist_ind_act_2: "ἐ~ψᾰς", "ἐ~ψᾰτον", "ἐ~ψᾰτε";
    aorist_ind_act_3: "ἐ~ψε", "ἐ~ψᾰτην", "ἐ~ψᾰν";
    aorist_ind_mid_1: "ἐ~ψᾰμην", _, "ἐ~ψᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ψω", "ἐ~ψᾰσθον", "ἐ~ψᾰσθε";
    aorist_ind_mid_3: "ἐ~ψᾰτο", "ἐ~ψᾰσθην", "ἐ~ψᾰντο";
    aorist_ind_pass_1: "ἐ~φθην", _, "ἐ~φθημεν";
    aorist_ind_pass_2: "ἐ~φθης", "ἐ~φθητον", "ἐ~φθητε";
    aorist_ind_pass_3: "ἐ~φθη", "ἐ~φθητην", "ἐ~φθησᾰν";
    aorist_subj_act_1: "~ψω", _, "~ψωμεν";
    aorist_subj_act_2: "~ψῃς", "~ψητον", "~ψητε";
    aorist_subj_act_3: "~ψῃ", _, "~ψωσῐ";
    aorist_subj_mid_1: "~ψωμαι", _, "~ψωμεθᾰ";
    aorist_subj_mid_2: "~ψῃ", "~ψησθον", "~ψησθε";
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
    perfect_ind_pass_1: _, _, "%ε~μεθᾰ";
    perfect_subj_act_1: "%ε~φω", _, "%ε~φωμεν";
    perfect_subj_act_2: "%ε~φῃς", "%ε~φητον", "%ε~φητε";
    perfect_subj_act_3: "%ε~φῃ", _, "%ε~φωσῐ";
    perfect_opt_act_1: "%ε~φοιμῐ", _, "%ε~φοιμεν";
    perfect_opt_act_2: "%ε~φοις", "%ε~φοιτον", "%ε~φοιτε";
    perfect_opt_act_3: "%ε~φοι", "%ε~φοιτην", "%ε~φοιεν";
    perfect_imp_act_2: _, "%ε~φετον", "%ε~φετε";
    perfect_imp_act_3: "%ε~φετω", "%ε~φετων", "%ε~φοντων";
    pluperfect_act_1: "ἐ%ε~φειν", _, "ἐ%ε~φεμεν";
    pluperfect_act_2: "ἐ%ε~φεις", "ἐ%ε~φετον", "ἐ%ε~φετε";
    pluperfect_act_3: "ἐ%ε~φει", "ἐ%ε~φετην", "ἐ%ε~φεσᾰν";
    pluperfect_mid_1: "%ε~μην", _, "%ε~μεθᾰ";
    pluperfect_mid_2: "%ε~σο", "%ε~σθον", "%ε~σθε";
    pluperfect_mid_3: "%ε~το", "%ε~σθην", _;
    pluperfect_pass_1: _, _, "%ε~μεθᾰ";
    pluperfect_pass_2: _, _, "%ε~σθε";
    pluperfect_pass_3: _, "%ε~σθην", "%ε~ντο";
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
    // Νῑκαω: 27 words (νῑκαω, λικμαω, ὀργαω, μελεταω, προσαυδαω, κομαω). Like Δοκᾰω, except in 51 cells: ind_act_3 pl, subj_act_3 pl, opt_act_1 sg, opt_act_1 pl, opt_act_2 sg, opt_act_2 du, ….
    conjugation!("νῑκαω", 27, "~αω", like Δοκᾰω;
    ind_act_3: _, _, "~ωσῐ";
    subj_act_3: _, _, "~ωσῐ";
    opt_act_1: "~ῳην", _, "~ῳμεν";
    opt_act_2: "~ῳης", "~ῳτον", "~ῳτε";
    opt_act_3: "~ῳη", "~ῳτην", "~ῳεν";
    future_ind_act_3: _, _, "~ησουσῐ";
    aorist_subj_act_3: _, _, "~ησωσῐ";
    aorist_opt_act_2: "~ησειᾰς", _, _;
    aorist_opt_act_3: "~ησειε", _, "~ησειᾰν";
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_futp: _, derived, _;
    participle_futp: _, derived, _;
    ),
    // Πρεσβευω: 27 words (γρᾰφω, ζεω, σειω, ὀμειχω, τελεω, παιω). Like Σπᾰω, except in 112 cells: subj_act_3 pl, future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, future_ind_pass_2 du, future_ind_pass_2 pl, ….
    conjugation!("πρεσβευω", 27, "~ω", like Σπᾰω;
    subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    future_ind_pass_1: "~σθησομαι", _, "~σθησομεθᾰ";
    future_ind_pass_2: "~σθησῃ", "~σθησεσθον", "~σθησεσθε";
    future_ind_pass_3: "~σθησεται", _, "~σθησονται";
    future_opt_pass_1: "~σθησοιμην", _, "~σθησοιμεθᾰ";
    future_opt_pass_2: "~σθησοιο", "~σθησοισθον", "~σθησοισθε";
    future_opt_pass_3: "~σθησοιτο", "~σθησοισθην", "~σθησοιντο";
    aorist_ind_pass_1: "ἐ~σθην", _, "ἐ~σθημεν";
    aorist_ind_pass_2: "ἐ~σθης", "ἐ~σθητον", "ἐ~σθητε";
    aorist_ind_pass_3: "ἐ~σθη", "ἐ~σθητην", "ἐ~σθησᾰν";
    aorist_subj_pass_1: "~σθω", _, "~σθωμεν";
    aorist_subj_pass_2: "~σθῃς", "~σθητον", "~σθητε";
    aorist_subj_pass_3: "~σθῃ", _, "~σθωσῐ";
    aorist_opt_pass_1: "~σθειην", _, "~σθεῑμεν";
    aorist_opt_pass_2: "~σθειης", "~σθεῑτον", "~σθεῑτε";
    aorist_opt_pass_3: "~σθειη", "~σθειτην", "~σθεῑεν";
    aorist_imp_pass_2: "~σθητῐ", _, _;
    aorist_imp_pass_3: "~σθητω", "~σθητων", "~σθεντων";
    perfect_ind_act_1: "%ε~κᾰ", _, "%ε~κᾰμεν";
    perfect_ind_act_2: "%ε~κᾰς", "%ε~κᾰτον", "%ε~κᾰτε";
    perfect_ind_act_3: "%ε~κε", _, "%ε~κᾱσῐ";
    perfect_ind_mid_1: "%ε~μαι", _, "%ε~μεθᾰ";
    perfect_ind_mid_2: "%ε~σαι", "%ε~σθον", "%ε~σθε";
    perfect_ind_mid_3: "%ε~ται", _, "%ε~νται";
    perfect_ind_pass_1: _, _, "%ε~μεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~σθε";
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
    perfect_imp_pass_2: "%ε~σο", _, _;
    perfect_imp_pass_3: "%ε~σθω", "%ε~σθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: "ἐ%ε~μην", _, "ἐ%ε~μεθᾰ";
    pluperfect_mid_2: "ἐ%ε~σο", "ἐ%ε~σθον", "ἐ%ε~σθε";
    pluperfect_mid_3: "ἐ%ε~το", "ἐ%ε~σθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~σθε";
    pluperfect_pass_3: _, "ἐ%ε~σθην", "ἐ%ε~ντο";
    infinitive_fut: _, _, "~σθησεσθαι";
    infinitive_aor: _, _, "~σθηναι";
    infinitive_perf: "%ε~κεναι", "%ε~σθαι", _;
    participle_fut: _, _, "~σθησομενος";
    participle_aor: _, _, "~σθεις";
    participle_perf: "%ε~κως", "%ε~μενος", _;
    ),
    // Ἐλπω: 26 words (ὑω, εὑω, ἐλπω, ἰδῑω, ἐρδω, περικλειω). Like Πρεσβευω, except in 101 cells: subj_act_3 pl, imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, ….
    conjugation!("ἐλπω", 26, "~ω", like Πρεσβευω;
    subj_act_3: _, _, "~ωσῐ";
    imperfect_act_1: "~ον", _, "~ομεν";
    imperfect_act_2: "~ες", "~ετον", "~ετε";
    imperfect_act_3: "~ε", "~ετην", "~ον";
    imperfect_mid_1: "~ομην", _, "~ομεθᾰ";
    imperfect_mid_2: "~ου", "~εσθον", "~εσθε";
    imperfect_mid_3: "~ετο", "~εσθην", "~οντο";
    imperfect_pass_1: _, _, "~ομεθᾰ";
    imperfect_pass_2: _, _, "~εσθε";
    imperfect_pass_3: "~ετο", _, _;
    aorist_ind_act_1: "~σᾰ", _, "~σᾰμεν";
    aorist_ind_act_2: "~σᾰς", "~σᾰτον", "~σᾰτε";
    aorist_ind_act_3: "~σε", "~σᾰτην", "~σᾰν";
    aorist_ind_mid_1: "~σᾰμην", _, "~σᾰμεθᾰ";
    aorist_ind_mid_2: "~σω", "~σᾰσθον", "~σᾰσθε";
    aorist_ind_mid_3: "~σᾰτο", "~σᾰσθην", "~σᾰντο";
    aorist_ind_pass_1: "~σθην", _, "~σθημεν";
    aorist_ind_pass_2: "~σθης", "~σθητον", "~σθητε";
    aorist_ind_pass_3: "~σθη", "~σθητην", "~σθησᾰν";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: derived, derived, derived;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_mid_1: "~σμαι", _, "~σμεθᾰ";
    perfect_ind_mid_2: "~σαι", "~σθον", "~σθε";
    perfect_ind_mid_3: "~σται", _, "~σᾰται";
    perfect_ind_pass_1: _, _, "~σμεθᾰ";
    perfect_ind_pass_2: _, _, "~σθε";
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: _, derived, derived;
    perfect_imp_act_3: derived, derived, derived;
    perfect_imp_mid_2: "~σο", _, _;
    perfect_imp_mid_3: "~σθω", "~σθων", _;
    perfect_imp_pass_2: "~σο", _, _;
    perfect_imp_pass_3: "~σθω", "~σθων", _;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
    infinitive_perf: derived, "~σθαι", _;
    participle_perf: derived, "~σμενος", _;
    ),
    // Σῠνᾰρπᾰζω: 25 words (βᾰζω, ἁρπᾰζω, ἐλελῐζω, κρᾱζω, στῐζω, σφυζω). Like Ληγω, except in 138 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("σῠνᾰρπᾰζω", 25, "~ζω", like Ληγω;
    ind_act_1: "~ζω", _, "~ζομεν";
    ind_act_2: "~ζεις", _, _;
    ind_act_3: "~ζει", _, "~ζουσῐ";
    ind_mid_1: "~ζομαι", _, "~ζομεθᾰ";
    ind_mid_2: "~ζῃ", "~ζεσθον", "~ζεσθε";
    ind_mid_3: "~ζεται", _, "~ζονται";
    ind_pass_1: _, _, "~ζομεθᾰ";
    ind_pass_2: _, _, "~ζεσθε";
    ind_pass_3: "~ζεται", _, "~ζονται";
    subj_act_1: "~ζω", _, "~ζωμεν";
    subj_act_2: "~ζῃς", "~ζητον", "~ζητε";
    subj_act_3: "~ζῃ", _, "~ζωσῐ";
    subj_mid_1: "~ζωμαι", _, "~ζωμεθᾰ";
    subj_mid_2: "~ζῃ", "~ζησθον", "~ζησθε";
    subj_mid_3: "~ζηται", _, "~ζωνται";
    subj_pass_1: _, _, "~ζωμεθᾰ";
    subj_pass_2: _, _, "~ζησθε";
    subj_pass_3: "~ζηται", _, "~ζωνται";
    opt_act_1: "~ζοιμῐ", _, "~ζοιμεν";
    opt_act_2: "~ζοις", "~ζοιτον", "~ζοιτε";
    opt_act_3: "~ζοι", "~ζοιτην", "~ζοιεν";
    opt_mid_1: "~ζοιμην", _, "~ζοιμεθᾰ";
    opt_mid_2: "~ζοιο", "~ζοισθον", "~ζοισθε";
    opt_mid_3: "~ζοιτο", "~ζοισθην", "~ζοιντο";
    opt_pass_1: _, _, "~ζοιμεθᾰ";
    opt_pass_2: _, "~ζοισθον", "~ζοισθε";
    opt_pass_3: "~ζοιτο", "~ζοισθην", "~ζοιντο";
    imp_act_2: "~ζε", "~ζετον", "~ζετε";
    imp_act_3: "~ζετω", "~ζετων", "~ζοντων";
    imp_mid_2: "~ζου", _, "~ζεσθε";
    imp_mid_3: "~ζεσθω", "~ζεσθων", "~ζεσθων";
    imp_pass_2: _, _, "~ζεσθε";
    imp_pass_3: _, "~ζεσθων", _;
    imperfect_act_1: "ἐ~ζον", _, "ἐ~ζομεν";
    imperfect_act_2: "ἐ~ζες", "ἐ~ζετον", "ἐ~ζετε";
    imperfect_act_3: "ἐ~ζε", "ἐ~ζετην", "ἐ~ζον";
    imperfect_mid_1: "ἐ~ζομην", _, "ἐ~ζομεθᾰ";
    imperfect_mid_2: "ἐ~ζου", "ἐ~ζεσθον", "ἐ~ζεσθε";
    imperfect_mid_3: "ἐ~ζετο", "ἐ~ζεσθην", "ἐ~ζοντο";
    imperfect_pass_1: _, _, "ἐ~ζομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ζεσθε";
    imperfect_pass_3: "ἐ~ζετο", _, _;
    perfect_ind_act_1: "ἐ~χᾰ", _, "ἐ~χᾰμεν";
    perfect_ind_act_2: "ἐ~χᾰς", "ἐ~χᾰτον", "ἐ~χᾰτε";
    perfect_ind_act_3: "ἐ~χε", _, "ἐ~χᾱσῐ";
    perfect_ind_mid_1: "ἐ~γμαι", _, "ἐ~γμεθᾰ";
    perfect_ind_mid_2: "ἐ~ξαι", "ἐ~χθον", "ἐ~χθε";
    perfect_ind_mid_3: "ἐ~κται", _, "ἐ~γᾰται";
    perfect_ind_pass_1: _, _, "ἐ~γμεθᾰ";
    perfect_ind_pass_2: _, _, "ἐ~χθε";
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
    perfect_imp_pass_2: "ἐ~ξο", _, _;
    perfect_imp_pass_3: "ἐ~χθω", "ἐ~χθων", _;
    infinitive_pres: "~ζειν", "~ζεσθαι", _;
    infinitive_perf: "ἐ~χεναι", "ἐ~χθαι", _;
    participle_pres: "~ζων", "~ζομενος", _;
    participle_perf: "ἐ~χως", "ἐ~γμενος", _;
    ),
    // Ὠνεομαι: 24 words (ὀρχεομαι, πλουτοκρᾰτεομαι, ὠνεομαι, ἀρνεομαι, δημοκρᾰτεομαι, ἱκνεομαι). Like Ποιεω, except in 96 cells: ind_act_3 pl, ind_mid_2 sg, ind_pass_2 sg, subj_act_3 pl, opt_act_1 sg, opt_act_1 pl, ….
    conjugation!("ὠνεομαι", 24, "~εομαι", like Ποιεω;
    ind_act_3: _, _, "~οῡσῐ";
    ind_mid_2: "~εῑ", _, _;
    ind_pass_2: derived, _, _;
    subj_act_3: _, _, "~ωσῐ";
    opt_act_1: "~οιην", _, "~οῑμεν";
    opt_act_2: "~οιης", "~οῑτον", "~οῑτε";
    opt_act_3: "~οιη", "~οιτην", "~οῑεν";
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
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_imp_act_2: derived, derived, derived;
    aorist_imp_act_3: derived, derived, derived;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
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
    // Εὐλογεω: 22 words (εὐλογεω, ἠθεω, κᾰτηχεω, ἠχεω, ὀλιγαρχεω, μοναρχεω). Like Δῐκεω, except in 109 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("εὐλογεω", 22, "~εω", like Δῐκεω;
    imperfect_act_1: "~ουν", _, "~οῡμεν";
    imperfect_act_2: "~εις", "~εῑτον", "~εῑτε";
    imperfect_act_3: "~ει", "~ειτην", "~ουν";
    imperfect_mid_1: "~ουμην", _, "~ουμεθᾰ";
    imperfect_mid_2: "~οῡ", "~εῑσθον", "~εῑσθε";
    imperfect_mid_3: "~εῑτο", "~εισθην", "~οῡντο";
    imperfect_pass_1: _, _, "~ουμεθᾰ";
    imperfect_pass_2: _, _, "~εῑσθε";
    imperfect_pass_3: "~εῑτο", _, _;
    aorist_ind_act_1: "~ησᾰ", _, "~ησᾰμεν";
    aorist_ind_act_2: "~ησᾰς", "~ησᾰτον", "~ησᾰτε";
    aorist_ind_act_3: "~ησε|~ησεν", "~ησᾰτην", "~ησᾰν";
    aorist_ind_mid_1: "~ησᾰμην", _, "~ησᾰμεθᾰ";
    aorist_ind_mid_2: "~ησω", "~ησᾰσθον", "~ησᾰσθε";
    aorist_ind_mid_3: "~ησᾰτο", "~ησᾰσθην", "~ησᾰντο";
    aorist_ind_pass_1: "~ηθην", _, "~ηθημεν";
    aorist_ind_pass_2: "~ηθης", "~ηθητον", "~ηθητε";
    aorist_ind_pass_3: "~ηθη", "~ηθητην", "~ηθησᾰν";
    perfect_ind_act_1: "~ηκᾰ", _, "~ηκᾰμεν";
    perfect_ind_act_2: "~ηκᾰς", "~ηκᾰτον", "~ηκᾰτε";
    perfect_ind_act_3: "~ηκε|~ηκεν", _, "~ηκᾱσῐ|~ηκᾱσῐν";
    perfect_ind_mid_1: "~ημαι", _, "~ημεθᾰ";
    perfect_ind_mid_2: "~ησαι", "~ησθον", "~ησθε";
    perfect_ind_mid_3: "~ηται", _, "~ηνται";
    perfect_ind_pass_1: _, _, "~ημεθᾰ";
    perfect_ind_pass_2: _, _, "~ησθε";
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
    perfect_imp_pass_2: "~ησο", _, _;
    perfect_imp_pass_3: "~ησθω", "~ησθων", _;
    pluperfect_act_1: "~ηκειν|~ηκη", _, "~ηκεμεν";
    pluperfect_act_2: "~ηκεις|~ηκης", "~ηκετον", "~ηκετε";
    pluperfect_act_3: "~ηκει|~ηκειν", "~ηκετην", "~ηκεσᾰν";
    pluperfect_mid_1: "~ημην", _, "~ημεθᾰ";
    pluperfect_mid_2: "~ησο", "~ησθον", "~ησθε";
    pluperfect_mid_3: "~ητο", "~ησθην", _;
    pluperfect_pass_1: _, _, "~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ησθε";
    pluperfect_pass_3: _, "~ησθην", "~ηντο";
    infinitive_perf: "~ηκεναι", "~ησθαι", _;
    participle_perf: "~ηκως", "~ημενος", _;
    ),
    // Ψηφῐζω: 20 words (γνωρῐζω, ψηφῐζω, σχηματῐζω, ἀτενῐζω, κτενῐζω, ὀνειδῐζω). Like Ἀφορῐζω, except in 68 cells: imperfect_act_3 sg, aorist_ind_act_3 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 sg, perfect_ind_act_2 du, ….
    conjugation!("ψηφῐζω", 20, "~ζω", like Ἀφορῐζω;
    imperfect_act_3: "ἐ~ζε|ἐ~ζεν", _, _;
    aorist_ind_act_3: "ἐ~σε|ἐ~σεν", _, _;
    perfect_ind_act_1: "ἐ~κᾰ", _, "ἐ~κᾰμεν";
    perfect_ind_act_2: "ἐ~κᾰς", "ἐ~κᾰτον", "ἐ~κᾰτε";
    perfect_ind_act_3: "ἐ~κε|ἐ~κεν", _, "ἐ~κᾱσῐ|ἐ~κᾱσῐν";
    perfect_ind_mid_1: "ἐ~σμαι", _, "ἐ~σμεθᾰ";
    perfect_ind_mid_2: "ἐ~σαι", "ἐ~σθον", "ἐ~σθε";
    perfect_ind_mid_3: "ἐ~σται", _, "ἐ~δᾰται";
    perfect_ind_pass_1: _, _, "ἐ~σμεθᾰ";
    perfect_ind_pass_2: _, _, "ἐ~σθε";
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
    perfect_imp_pass_2: "ἐ~σο", _, _;
    perfect_imp_pass_3: "ἐ~σθω", "ἐ~σθων", _;
    pluperfect_act_1: "ἐ~κειν|ἐ~κη", _, "ἐ~κεμεν";
    pluperfect_act_2: "ἐ~κεις|ἐ~κης", "ἐ~κετον", "ἐ~κετε";
    pluperfect_act_3: "ἐ~κει|ἐ~κειν", "ἐ~κετην", "ἐ~κεσᾰν";
    pluperfect_mid_1: "ἐ~σμην", _, "ἐ~σμεθᾰ";
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", "ἐ~σθε";
    pluperfect_mid_3: "ἐ~στο", "ἐ~σθην", _;
    pluperfect_pass_1: _, _, "ἐ~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~σθε";
    pluperfect_pass_3: _, "ἐ~σθην", "ἐ~δᾰτο";
    infinitive_perf: "ἐ~κεναι", "ἐ~σθαι", _;
    participle_perf: "ἐ~κως", "ἐ~σμενος", _;
    ),
    // Θωᾰω: 16 words (πεπτω, τᾰττω, ἀλεγω, ϝερρω, φᾱρω, θωαδδω). Like Ἑπομαι, except in 80 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_2 sg, ….
    conjugation!("θωᾰω", 16, "~ω", like Ἑπομαι;
    ind_act_1: "~ω", _, "~ομεν";
    ind_act_2: "~εις", _, _;
    ind_act_3: "~ει", _, "~ουσῐ|~ουσῐν";
    ind_mid_2: "~εαι", _, _;
    subj_act_1: "~ω", _, "~ωμεν";
    subj_act_2: "~ῃς", "~ητον", "~ητε";
    subj_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    subj_mid_2: "~ηαι", _, _;
    opt_act_1: "~οιμῐ", _, "~οιμεν";
    opt_act_2: "~οις", "~οιτον", "~οιτε";
    opt_act_3: "~οι", "~οιτην", "~οιεν";
    imp_act_2: "~ε", "~ετον", "~ετε";
    imp_act_3: "~ετω", "~ετων", "~οντων";
    imp_mid_2: "~εο", _, _;
    imperfect_act_1: "ἐ~ον", _, "ἐ~ομεν";
    imperfect_act_2: "ἐ~ες", "ἐ~ετον", "ἐ~ετε";
    imperfect_act_3: "ἐ~ε", "ἐ~ετην", "ἐ~ον";
    imperfect_mid_1: "ἐ~ομην", _, "ἐ~ομεθᾰ";
    imperfect_mid_2: "ἐ~ου", "ἐ~εσθον", "ἐ~εσθε";
    imperfect_mid_3: "ἐ~ετο", "ἐ~εσθην", "ἐ~οντο";
    imperfect_pass_1: _, _, "ἐ~ομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~εσθε";
    imperfect_pass_3: "ἐ~ετο", _, _;
    aorist_ind_mid_1: derived, _, derived;
    aorist_ind_mid_2: derived, derived, derived;
    aorist_ind_mid_3: derived, derived, derived;
    aorist_subj_mid_1: derived, _, derived;
    aorist_subj_mid_2: derived, derived, derived;
    aorist_subj_mid_3: derived, _, derived;
    aorist_opt_mid_1: derived, _, derived;
    aorist_opt_mid_2: derived, derived, derived;
    aorist_opt_mid_3: derived, derived, derived;
    aorist_imp_mid_2: derived, derived, derived;
    aorist_imp_mid_3: derived, derived, _;
    infinitive_pres: "~ειν", _, _;
    infinitive_aor: _, derived, _;
    participle_pres: "~ων", _, _;
    participle_aor: _, derived, _;
    ),
    // Πειρᾰζω: 15 words (πᾰρουσῐᾰζω, πλεονᾰζω, θειᾰζω, γυμνᾰζω, πειρᾰζω, πλησῐᾰζω). Like Ἀφορῐζω, except in 55 cells: imperfect_act_3 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("πειρᾰζω", 15, "~ζω", like Ἀφορῐζω;
    imperfect_act_3: "ἐ~ζε|ἐ~ζεν", _, _;
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
    aorist_ind_act_3: "ἐ~σε|ἐ~σεν", _, _;
    pluperfect_act_1: "ἐ~κειν|ἐ~κη", _, "ἐ~κεμεν";
    pluperfect_act_2: "ἐ~κεις|ἐ~κης", "ἐ~κετον", "ἐ~κετε";
    pluperfect_act_3: "ἐ~κει|ἐ~κειν", "ἐ~κετην", "ἐ~κεσᾰν";
    pluperfect_mid_1: "ἐ~σμην", _, "ἐ~σμεθᾰ";
    pluperfect_mid_2: "ἐ~σο", "ἐ~σθον", "ἐ~σθε";
    pluperfect_mid_3: "ἐ~στο", "ἐ~σθην", _;
    pluperfect_pass_1: _, _, "ἐ~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~σθε";
    pluperfect_pass_3: _, "ἐ~σθην", "ἐ~δᾰτο";
    infinitive_fut: "~σειν", "~σεσθαι", _;
    participle_fut: "~σων", "~σομενος", _;
    ),
    // Κτῐζω: 14 words (σχῐζω, ὀνομᾰζω, σχεδῐᾰζω, κτῐζω, σπιζω, φᾰντᾰζω). Like Ψηφῐζω, except in 34 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("κτῐζω", 14, "~ζω", like Ψηφῐζω;
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
    // Πεμπω: 14 words (λᾰμπω, βλεπω, ἐρειπω, λειπω, ἑρπω, πεμπω). Like Προσερχομαι, except in 298 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("πεμπω", 14, "~~πω", like Προσερχομαι;
    ind_act_1: "~~πω", _, "~~πομεν";
    ind_act_2: "~~πεις", _, _;
    ind_act_3: "~~πει", _, "~~πουσῐ";
    ind_mid_1: "~~πομαι", _, "~~πομεθᾰ";
    ind_mid_2: "~~πῃ", "~~πεσθον", "~~πεσθε";
    ind_mid_3: "~~πεται", _, "~~πονται";
    ind_pass_1: _, _, "~~πομεθᾰ";
    ind_pass_2: _, _, "~~πεσθε";
    ind_pass_3: "~~πεται", _, "~~πονται";
    subj_act_1: "~~πω", _, "~~πωμεν";
    subj_act_2: "~~πῃς", "~~πητον", "~~πητε";
    subj_act_3: "~~πῃ", _, "~~πωσῐ";
    subj_mid_1: "~~πωμαι", _, "~~πωμεθᾰ";
    subj_mid_2: "~~πῃ", "~~πησθον", "~~πησθε";
    subj_mid_3: "~~πηται", _, "~~πωνται";
    subj_pass_1: _, _, "~~πωμεθᾰ";
    subj_pass_2: _, _, "~~πησθε";
    subj_pass_3: "~~πηται", _, "~~πωνται";
    opt_act_1: "~~ποιμῐ", _, "~~ποιμεν";
    opt_act_2: "~~ποις", "~~ποιτον", "~~ποιτε";
    opt_act_3: "~~ποι", "~~ποιτην", "~~ποιεν";
    opt_mid_1: "~~ποιμην", _, "~~ποιμεθᾰ";
    opt_mid_2: "~~ποιο", "~~ποισθον", "~~ποισθε";
    opt_mid_3: "~~ποιτο", "~~ποισθην", "~~ποιντο";
    opt_pass_1: _, _, "~~ποιμεθᾰ";
    opt_pass_2: _, "~~ποισθον", "~~ποισθε";
    opt_pass_3: "~~ποιτο", "~~ποισθην", "~~ποιντο";
    imp_act_2: "~~πε", "~~πετον", "~~πετε";
    imp_act_3: "~~πετω", "~~πετων", "~~ποντων";
    imp_mid_2: "~~που", _, "~~πεσθε";
    imp_mid_3: "~~πεσθω", "~~πεσθων", "~~πεσθων";
    imp_pass_2: _, _, "~~πεσθε";
    imp_pass_3: _, "~~πεσθων", _;
    imperfect_act_1: "ἐ~~πον", _, "ἐ~~πομεν";
    imperfect_act_2: "ἐ~~πες", "ἐ~~πετον", "ἐ~~πετε";
    imperfect_act_3: "ἐ~~πε", "ἐ~~πετην", "ἐ~~πον";
    imperfect_mid_1: "ἐ~~πομην", _, "ἐ~~πομεθᾰ";
    imperfect_mid_2: "ἐ~~που", "ἐ~~πεσθον", "ἐ~~πεσθε";
    imperfect_mid_3: "ἐ~~πετο", "ἐ~~πεσθην", "ἐ~~ποντο";
    imperfect_pass_1: _, _, "ἐ~~πομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~~πεσθε";
    imperfect_pass_3: "ἐ~~πετο", _, _;
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
    aorist_ind_act_1: "ἐ~~ψᾰ", _, "ἐ~~ψᾰμεν";
    aorist_ind_act_2: "ἐ~~ψᾰς", "ἐ~~ψᾰτον", "ἐ~~ψᾰτε";
    aorist_ind_act_3: "ἐ~~ψε", "ἐ~~ψᾰτην", "ἐ~~ψᾰν";
    aorist_ind_mid_1: "ἐ~~ψᾰμην", _, "ἐ~~ψᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~~ψω", "ἐ~~ψᾰσθον", "ἐ~~ψᾰσθε";
    aorist_ind_mid_3: "ἐ~~ψᾰτο", "ἐ~~ψᾰσθην", "ἐ~~ψᾰντο";
    aorist_ind_pass_1: "ἐ~~φθην", _, "ἐ~~φθημεν";
    aorist_ind_pass_2: "ἐ~~φθης", "ἐ~~φθητον", "ἐ~~φθητε";
    aorist_ind_pass_3: "ἐ~~φθη", "ἐ~~φθητην", "ἐ~~φθησᾰν";
    aorist_subj_act_1: "~~ψω", _, "~~ψωμεν";
    aorist_subj_act_2: "~~ψῃς", "~~ψητον", "~~ψητε";
    aorist_subj_act_3: "~~ψῃ", _, "~~ψωσῐ";
    aorist_subj_mid_1: "~~ψωμαι", _, "~~ψωμεθᾰ";
    aorist_subj_mid_2: "~~ψῃ", "~~ψησθον", "~~ψησθε";
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
    perfect_ind_pass_1: _, _, "%ε~~μεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~~φθε";
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
    perfect_imp_pass_2: "%ε~~ψο", _, _;
    perfect_imp_pass_3: "%ε~~φθω", "%ε~~φθων", _;
    pluperfect_act_1: "ἐ~πο~φειν", _, "ἐ~πο~φεμεν";
    pluperfect_act_2: "ἐ~πο~φεις", "ἐ~πο~φετον", "ἐ~πο~φετε";
    pluperfect_act_3: "ἐ~πο~φει", "ἐ~πο~φετην", "ἐ~πο~φεσᾰν";
    pluperfect_mid_1: "ἐ%ε~~μην", _, "ἐ%ε~~μεθᾰ";
    pluperfect_mid_2: "ἐ%ε~~ψο", "ἐ%ε~~φθον", "ἐ%ε~~φθε";
    pluperfect_mid_3: "ἐ%ε~~πτο", "ἐ%ε~~φθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~~φθε";
    pluperfect_pass_3: _, "ἐ%ε~~φθην", "ἐ%ε~~πᾰτο";
    infinitive_pres: "~~πειν", "~~πεσθαι", _;
    infinitive_fut: "~~ψειν", "~~ψεσθαι", "~~φθησεσθαι";
    infinitive_aor: "~~ψαι", "~~ψᾰσθαι", "~~φθηναι";
    infinitive_perf: "~πο~φεναι", "%ε~~φθαι", _;
    participle_pres: "~~πων", "~~πομενος", _;
    participle_fut: "~~ψων", "~~ψομενος", "~~φθησομενος";
    participle_aor: "~~ψᾱς", "~~ψᾰμενος", "~~φθεις";
    participle_perf: "~πο~φως", "%ε~~μενος", _;
    ),
    // Οἰομαι: 14 words (γῐγνομαι, μαινομαι, βουλομαι, οἰομαι, οἰχομαι, κτᾰομαι). Like Θωᾰω, except in 102 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_2 sg, ….
    conjugation!("οἰομαι", 14, "~ομαι", like Θωᾰω;
    ind_act_1: "~ομαι", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, _, derived;
    ind_mid_2: "~ῃ", _, _;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, _, derived;
    subj_mid_2: "~ῃ", _, _;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imp_act_3: derived, derived, derived;
    imp_mid_2: "~ου", _, _;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    future_ind_mid_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_mid_2: "~ησῃ", "~ησεσθον", "~ησεσθε";
    future_ind_mid_3: "~ησεται", _, "~ησονται";
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_2: "~ησοιο", "~ησοισθον", "~ησοισθε";
    future_opt_mid_3: "~ησοιτο", "~ησοισθην", "~ησοιντο";
    aorist_ind_pass_1: "ἐ~ηθην", _, "ἐ~ηθημεν";
    aorist_ind_pass_2: "ἐ~ηθης", "ἐ~ηθητον", "ἐ~ηθητε";
    aorist_ind_pass_3: "ἐ~ηθη", "ἐ~ηθητην", "ἐ~ηθησᾰν";
    aorist_subj_pass_1: "~ηθω", _, "~ηθωμεν";
    aorist_subj_pass_2: "~ηθῃς", "~ηθητον", "~ηθητε";
    aorist_subj_pass_3: "~ηθῃ", _, "~ηθωσῐ";
    aorist_opt_pass_1: "~ηθειην", _, "~ηθεῑμεν";
    aorist_opt_pass_2: "~ηθειης", "~ηθεῑτον", "~ηθεῑτε";
    aorist_opt_pass_3: "~ηθειη", "~ηθειτην", "~ηθεῑεν";
    aorist_imp_pass_2: "~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ηθητω", "~ηθητων", "~ηθεντων";
    perfect_ind_mid_1: "%ε~ημαι", _, "%ε~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ησαι", "%ε~ησθον", "%ε~ησθε";
    perfect_ind_mid_3: "%ε~ηται", _, "%ε~ηνται";
    perfect_ind_pass_1: _, _, "%ε~ημεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~ησθε";
    perfect_imp_mid_2: "%ε~ησο", _, _;
    perfect_imp_mid_3: "%ε~ησθω", "%ε~ησθων", _;
    perfect_imp_pass_2: "%ε~ησο", _, _;
    perfect_imp_pass_3: "%ε~ησθω", "%ε~ησθων", _;
    infinitive_pres: derived, _, _;
    infinitive_fut: _, "~ησεσθαι", _;
    infinitive_aor: _, _, "~ηθηναι";
    infinitive_perf: _, "%ε~ησθαι", _;
    participle_pres: derived, _, _;
    participle_fut: _, "~ησομενος", _;
    participle_aor: _, _, "~ηθεις";
    participle_perf: _, "%ε~ημενος", _;
    ),
    // Πᾰρερχομαι: 13 words (προερχομαι, σῠνερχομαι, ἐξερχομαι, εἰσερχομαι, ἀπερχομαι, δῐερχομαι). Like Ἑπομαι, except in 173 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("πᾰρερχομαι", 13, "~ερχομαι", like Ἑπομαι;
    ind_act_1: "~ερχομαι", _, _;
    ind_mid_1: "~ερχομαι", _, "~ερχομεθᾰ";
    ind_mid_2: "~ερχῃ|~ερχει", "~ερχεσθον", "~ερχεσθε";
    ind_mid_3: "~ερχεται", _, "~ερχονται";
    ind_pass_1: _, _, derived;
    ind_pass_2: _, _, derived;
    ind_pass_3: derived, _, derived;
    subj_mid_1: "~ερχωμαι", _, "~ερχωμεθᾰ";
    subj_mid_2: "~ερχῃ", "~ερχησθον", "~ερχησθε";
    subj_mid_3: "~ερχηται", _, "~ερχωνται";
    subj_pass_1: _, _, derived;
    subj_pass_2: _, _, derived;
    subj_pass_3: derived, _, derived;
    opt_mid_1: "~ερχοιμην", _, "~ερχοιμεθᾰ";
    opt_mid_2: "~ερχοιο", "~ερχοισθον", "~ερχοισθε";
    opt_mid_3: "~ερχοιτο", "~ερχοισθην", "~ερχοιντο";
    opt_pass_1: _, _, derived;
    opt_pass_2: _, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_mid_2: "~ερχου", _, "~ερχεσθε";
    imp_mid_3: "~ερχεσθω", "~ερχεσθων", "~ερχεσθων";
    imp_pass_2: _, _, derived;
    imp_pass_3: _, derived, _;
    imperfect_mid_1: "~ηρχομην", _, "~ηρχομεθᾰ";
    imperfect_mid_2: "~ηρχου", "~ηρχεσθον", "~ηρχεσθε";
    imperfect_mid_3: "~ηρχετο", "~ηρχεσθην", "~ηρχοντο";
    imperfect_pass_1: _, _, "~ηρχομεθᾰ";
    imperfect_pass_2: _, _, "~ηρχεσθε";
    imperfect_pass_3: "~ηρχετο", _, _;
    future_ind_mid_1: "~ελευσομαι", _, "~ελευσομεθᾰ";
    future_ind_mid_2: "~ελευσῃ|~ελευσει", "~ελευσεσθον", "~ελευσεσθε";
    future_ind_mid_3: "~ελευσεται", _, "~ελευσονται";
    future_opt_mid_1: "~ελευσοιμην", _, "~ελευσοιμεθᾰ";
    future_opt_mid_2: "~ελευσοιο", "~ελευσοισθον", "~ελευσοισθε";
    future_opt_mid_3: "~ελευσοιτο", "~ελευσοισθην", "~ελευσοιντο";
    aorist_ind_act_1: "~ηλθον", _, "~ηλθομεν";
    aorist_ind_act_2: "~ηλθες", "~ηλθετον", "~ηλθετε";
    aorist_ind_act_3: "~ηλθε|~ηλθεν", "~ηλθετην", "~ηλθον";
    aorist_ind_mid_1: derived, _, derived;
    aorist_ind_mid_2: derived, derived, derived;
    aorist_ind_mid_3: derived, derived, derived;
    aorist_subj_act_1: "~ελθω", _, "~ελθωμεν";
    aorist_subj_act_2: "~ελθῃς", "~ελθητον", "~ελθητε";
    aorist_subj_act_3: "~ελθῃ", _, "~ελθωσῐ|~ελθωσῐν";
    aorist_subj_mid_1: derived, _, derived;
    aorist_subj_mid_2: derived, derived, derived;
    aorist_subj_mid_3: derived, _, derived;
    aorist_opt_act_1: "~ελθοιμῐ", _, "~ελθοιμεν";
    aorist_opt_act_2: "~ελθοις", "~ελθοιτον", "~ελθοιτε";
    aorist_opt_act_3: "~ελθοι", "~ελθοιτην", "~ελθοιεν";
    aorist_opt_mid_1: derived, _, derived;
    aorist_opt_mid_2: derived, derived, derived;
    aorist_opt_mid_3: derived, derived, derived;
    aorist_imp_act_2: "~ελθε", "~ελθετον", "~ελθετε";
    aorist_imp_act_3: "~ελθετω", "~ελθετων", "~ελθοντων";
    aorist_imp_mid_2: derived, derived, derived;
    aorist_imp_mid_3: derived, derived, _;
    perfect_ind_act_1: "~εληλῠθᾰ", _, "~εληλῠθᾰμεν";
    perfect_ind_act_2: "~εληλῠθᾰς", "~εληλῠθᾰτον", "~εληλῠθᾰτε";
    perfect_ind_act_3: "~εληλῠθε|~εληλῠθεν", _, "~εληλῠθᾱσῐ|~εληλῠθᾱσῐν";
    perfect_subj_act_1: "~εληλῠθω", _, "~εληλῠθωμεν";
    perfect_subj_act_2: "~εληλῠθῃς", "~εληλῠθητον", "~εληλῠθητε";
    perfect_subj_act_3: "~εληλῠθῃ", _, "~εληλῠθωσῐ|~εληλῠθωσῐν";
    perfect_opt_act_1: "~εληλῠθοιμῐ|~εληλῠθοιην", _, "~εληλῠθοιμεν";
    perfect_opt_act_2: "~εληλῠθοις|~εληλῠθοιης", "~εληλῠθοιτον", "~εληλῠθοιτε";
    perfect_opt_act_3: "~εληλῠθοι|~εληλῠθοιη", "~εληλῠθοιτην", "~εληλῠθοιεν";
    perfect_imp_act_2: "~εληλῠθε", "~εληλῠθετον", "~εληλῠθετε";
    perfect_imp_act_3: "~εληλῠθετω", "~εληλῠθετων", "~εληλῠθοντων";
    pluperfect_act_1: "~εληλυθειν", _, "~εληλυθεμεν";
    pluperfect_act_2: "~εληλυθεις", "~εληλυθετον", "~εληλυθετε";
    pluperfect_act_3: "~εληλυθει", "~εληλυθετην", "~εληλυθεσᾰν";
    infinitive_pres: _, derived, _;
    infinitive_fut: _, "~ελευσεσθαι", _;
    infinitive_aor: "~ελθεῑν", derived, _;
    infinitive_perf: "~εληλῠθεναι", _, _;
    participle_pres: _, derived, _;
    participle_fut: _, "~ελευσομενος", _;
    participle_aor: "~ελθων", derived, _;
    participle_perf: "~εληλῠθως", _, _;
    ),
    // Εὐθῡνω: 12 words (πληθῡνω, ἀμῡνω, αἰσχῡνω, πλῡνω, μολῡνω, ὀξῡνω). Like Γῡροω, except in 298 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("εὐθῡνω", 12, "~ῡνω", like Γῡροω;
    ind_act_1: "~ῡνω", _, "~ῡνομεν";
    ind_act_2: "~ῡνεις", _, _;
    ind_act_3: "~ῡνει", _, "~ῡνουσῐ";
    ind_mid_1: "~ῡνομαι", _, "~ῡνομεθᾰ";
    ind_mid_2: "~ῡνῃ", "~ῡνεσθον", "~ῡνεσθε";
    ind_mid_3: "~ῡνεται", _, "~ῡνονται";
    ind_pass_1: _, _, "~ῡνομεθᾰ";
    ind_pass_2: _, _, "~ῡνεσθε";
    ind_pass_3: "~ῡνεται", _, "~ῡνονται";
    subj_act_1: "~ῡνω", _, "~ῡνωμεν";
    subj_act_2: "~ῡνῃς", "~ῡνητον", "~ῡνητε";
    subj_act_3: "~ῡνῃ", _, "~ῡνωσῐ";
    subj_mid_1: "~ῡνωμαι", _, "~ῡνωμεθᾰ";
    subj_mid_2: "~ῡνῃ", "~ῡνησθον", "~ῡνησθε";
    subj_mid_3: "~ῡνηται", _, "~ῡνωνται";
    subj_pass_1: _, _, "~ῡνωμεθᾰ";
    subj_pass_2: _, _, "~ῡνησθε";
    subj_pass_3: "~ῡνηται", _, "~ῡνωνται";
    opt_act_1: "~ῡνοιμῐ", _, "~ῡνοιμεν";
    opt_act_2: "~ῡνοις", "~ῡνοιτον", "~ῡνοιτε";
    opt_act_3: "~ῡνοι", "~ῡνοιτην", "~ῡνοιεν";
    opt_mid_1: "~ῡνοιμην", _, "~ῡνοιμεθᾰ";
    opt_mid_2: "~ῡνοιο", "~ῡνοισθον", "~ῡνοισθε";
    opt_mid_3: "~ῡνοιτο", "~ῡνοισθην", "~ῡνοιντο";
    opt_pass_1: _, _, "~ῡνοιμεθᾰ";
    opt_pass_2: _, "~ῡνοισθον", "~ῡνοισθε";
    opt_pass_3: "~ῡνοιτο", "~ῡνοισθην", "~ῡνοιντο";
    imp_act_2: "~ῡνε", "~ῡνετον", "~ῡνετε";
    imp_act_3: "~ῡνετω", "~ῡνετων", "~ῡνοντων";
    imp_mid_2: "~ῡνου", _, "~ῡνεσθε";
    imp_mid_3: "~ῡνεσθω", "~ῡνεσθων", "~ῡνεσθων";
    imp_pass_2: _, _, "~ῡνεσθε";
    imp_pass_3: _, "~ῡνεσθων", _;
    imperfect_act_1: "ἐ~ῡνον", _, "ἐ~ῡνομεν";
    imperfect_act_2: "ἐ~ῡνες", "ἐ~ῡνετον", "ἐ~ῡνετε";
    imperfect_act_3: "ἐ~ῡνε", "ἐ~ῡνετην", "ἐ~ῡνον";
    imperfect_mid_1: "ἐ~ῡνομην", _, "ἐ~ῡνομεθᾰ";
    imperfect_mid_2: "ἐ~ῡνου", "ἐ~ῡνεσθον", "ἐ~ῡνεσθε";
    imperfect_mid_3: "ἐ~ῡνετο", "ἐ~ῡνεσθην", "ἐ~ῡνοντο";
    imperfect_pass_1: _, _, "ἐ~ῡνομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ῡνεσθε";
    imperfect_pass_3: "ἐ~ῡνετο", _, _;
    future_ind_act_1: "~ῠνω", _, "~ῠνοῡμεν";
    future_ind_act_2: "~ῠνεῑς", "~ῠνεῑτον", "~ῠνεῑτε";
    future_ind_act_3: "~ῠνεῑ", _, "~ῠνοῡσῐ";
    future_ind_mid_1: "~ῠνοῡμαι", _, "~ῠνουμεθᾰ";
    future_ind_mid_2: "~ῠνεῑ", "~ῠνεῑσθον", "~ῠνεῑσθε";
    future_ind_mid_3: "~ῠνεῑται", _, "~ῠνοῡνται";
    future_ind_pass_1: "~ῠνθησομαι", _, "~ῠνθησομεθᾰ";
    future_ind_pass_2: "~ῠνθησῃ", "~ῠνθησεσθον", "~ῠνθησεσθε";
    future_ind_pass_3: "~ῠνθησεται", _, "~ῠνθησονται";
    future_opt_act_1: "~ῠνοιην", _, "~ῠνοῑμεν";
    future_opt_act_2: "~ῠνοιης", "~ῠνοῑτον", "~ῠνοῑτε";
    future_opt_act_3: "~ῠνοιη", "~ῠνοιτην", "~ῠνοῑεν";
    future_opt_mid_1: "~ῠνοιμην", _, "~ῠνοιμεθᾰ";
    future_opt_mid_2: "~ῠνοῑο", "~ῠνοῑσθον", "~ῠνοῑσθε";
    future_opt_mid_3: "~ῠνοῑτο", "~ῠνοισθην", "~ῠνοῑντο";
    future_opt_pass_1: "~ῠνθησοιμην", _, "~ῠνθησοιμεθᾰ";
    future_opt_pass_2: "~ῠνθησοιο", "~ῠνθησοισθον", "~ῠνθησοισθε";
    future_opt_pass_3: "~ῠνθησοιτο", "~ῠνθησοισθην", "~ῠνθησοιντο";
    aorist_ind_act_1: "ἐ~ῡνᾰ", _, "ἐ~ῡνᾰμεν";
    aorist_ind_act_2: "ἐ~ῡνᾰς", "ἐ~ῡνᾰτον", "ἐ~ῡνᾰτε";
    aorist_ind_act_3: "ἐ~ῡνε", "ἐ~ῡνᾰτην", "ἐ~ῡνᾰν";
    aorist_ind_mid_1: "ἐ~ῡνᾰμην", _, "ἐ~ῡνᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ῡνω", "ἐ~ῡνᾰσθον", "ἐ~ῡνᾰσθε";
    aorist_ind_mid_3: "ἐ~ῡνᾰτο", "ἐ~ῡνᾰσθην", "ἐ~ῡνᾰντο";
    aorist_ind_pass_1: "ἐ~ῡνθην", _, "ἐ~ῡνθημεν";
    aorist_ind_pass_2: "ἐ~ῡνθης", "ἐ~ῡνθητον", "ἐ~ῡνθητε";
    aorist_ind_pass_3: "ἐ~ῡνθη", "ἐ~ῡνθητην", "ἐ~ῡνθησᾰν";
    aorist_subj_act_1: "~ῡνω", _, "~ῡνωμεν";
    aorist_subj_act_2: "~ῡνῃς", "~ῡνητον", "~ῡνητε";
    aorist_subj_act_3: "~ῡνῃ", _, "~ῡνωσῐ";
    aorist_subj_mid_1: "~ῡνωμαι", _, "~ῡνωμεθᾰ";
    aorist_subj_mid_2: "~ῡνῃ", "~ῡνησθον", "~ῡνησθε";
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
    perfect_ind_pass_1: _, _, "%ε~ῠσμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~ῠνθε";
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
    perfect_imp_pass_2: "%ε~ῠνσο", _, _;
    perfect_imp_pass_3: "%ε~ῠνθω", "%ε~ῠνθων", _;
    pluperfect_act_1: "ἐ%ε~ῠγκειν", _, "ἐ%ε~ῠγκεμεν";
    pluperfect_act_2: "ἐ%ε~ῠγκεις", "ἐ%ε~ῠγκετον", "ἐ%ε~ῠγκετε";
    pluperfect_act_3: "ἐ%ε~ῠγκει", "ἐ%ε~ῠγκετην", "ἐ%ε~ῠγκεσᾰν";
    pluperfect_mid_1: "ἐ%ε~ῠσμην", _, "ἐ%ε~ῠσμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ῠνσο", "ἐ%ε~ῠνθον", "ἐ%ε~ῠνθε";
    pluperfect_mid_3: "ἐ%ε~ῠντο", "ἐ%ε~ῠνθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~ῠσμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ῠνθε";
    pluperfect_pass_3: _, "ἐ%ε~ῠνθην", "ἐ%ε~ῠνᾰτο";
    infinitive_pres: "~ῡνειν", "~ῡνεσθαι", _;
    infinitive_fut: "~ῠνεῑν", "~ῠνεῑσθαι", "~ῠνθησεσθαι";
    infinitive_aor: "~ῡναι", "~ῡνᾰσθαι", "~ῡνθηναι";
    infinitive_perf: "%ε~ῠγκεναι", "%ε~ῠνθαι", _;
    participle_pres: "~ῡνων", "~ῡνομενος", _;
    participle_fut: "~ῠνων", "~ῠνουμενος", "~ῠνθησομενος";
    participle_aor: "~ῡνᾱς", "~ῡνᾰμενος", "~ῡνθεις";
    participle_perf: "%ε~ῠγκως", "%ε~ῠσμενος", _;
    ),
    // Ἀναιρεω: 12 words (δεω, φλεω, ἐμεω, ἐπαινεω, κλεω, ἀναιρεω). Like Δῐκεω, except in 132 cells: ind_act_3 pl, ind_mid_2 sg, ind_pass_2 sg, subj_act_3 pl, opt_act_1 sg, opt_act_1 pl, ….
    conjugation!("ἀναιρεω", 12, "~εω", like Δῐκεω;
    ind_act_3: _, _, "~οῡσῐ";
    ind_mid_2: "~εῑ", _, _;
    ind_pass_2: derived, _, _;
    subj_act_3: _, _, "~ωσῐ";
    opt_act_1: "~οιην", _, "~οῑμεν";
    opt_act_2: "~οιης", "~οῑτον", "~οῑτε";
    opt_act_3: "~οιη", "~οιτην", "~οῑεν";
    imp_act_2: "~εῑ", _, _;
    future_ind_act_3: _, _, "~ησουσῐ";
    future_ind_mid_2: "~ησῃ", _, _;
    future_ind_pass_1: "~εθησομαι", _, "~εθησομεθᾰ";
    future_ind_pass_2: "~εθησῃ", "~εθησεσθον", "~εθησεσθε";
    future_ind_pass_3: "~εθησεται", _, "~εθησονται";
    future_opt_pass_1: "~εθησοιμην", _, "~εθησοιμεθᾰ";
    future_opt_pass_2: "~εθησοιο", "~εθησοισθον", "~εθησοισθε";
    future_opt_pass_3: "~εθησοιτο", "~εθησοισθην", "~εθησοιντο";
    aorist_ind_act_3: "ἐ~ησε", _, _;
    aorist_ind_pass_1: "ἐ~εθην", _, "ἐ~εθημεν";
    aorist_ind_pass_2: "ἐ~εθης", "ἐ~εθητον", "ἐ~εθητε";
    aorist_ind_pass_3: "ἐ~εθη", "ἐ~εθητην", "ἐ~εθησᾰν";
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
    perfect_ind_pass_1: _, _, "~εδεμεθᾰ";
    perfect_ind_pass_2: _, _, "~εδεσθε";
    perfect_subj_act_1: "~εδεκω", _, "~εδεκωμεν";
    perfect_subj_act_2: "~εδεκῃς", "~εδεκητον", "~εδεκητε";
    perfect_subj_act_3: "~εδεκῃ", _, "~εδεκωσῐ";
    perfect_opt_act_1: "~εδεκοιμῐ", _, "~εδεκοιμεν";
    perfect_opt_act_2: "~εδεκοις", "~εδεκοιτον", "~εδεκοιτε";
    perfect_opt_act_3: "~εδεκοι", "~εδεκοιτην", "~εδεκοιεν";
    perfect_imp_act_2: derived, "~εδεκετον", "~εδεκετε";
    perfect_imp_act_3: "~εδεκετω", "~εδεκετων", "~εδεκοντων";
    perfect_imp_mid_2: "~εδεσο", _, _;
    perfect_imp_mid_3: "~εδεσθω", "~εδεσθων", _;
    perfect_imp_pass_2: "~εδεσο", _, _;
    perfect_imp_pass_3: "~εδεσθω", "~εδεσθων", _;
    pluperfect_act_1: "ἐ~εδεκειν", _, "ἐ~εδεκεμεν";
    pluperfect_act_2: "ἐ~εδεκεις", "ἐ~εδεκετον", "ἐ~εδεκετε";
    pluperfect_act_3: "ἐ~εδεκει", "ἐ~εδεκετην", "ἐ~εδεκεσᾰν";
    pluperfect_mid_1: "ἐ~εδεμην", _, "ἐ~εδεμεθᾰ";
    pluperfect_mid_2: "ἐ~εδεσο", "ἐ~εδεσθον", "ἐ~εδεσθε";
    pluperfect_mid_3: "ἐ~εδετο", "ἐ~εδεσθην", _;
    pluperfect_pass_1: _, _, "ἐ~εδεμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~εδεσθε";
    pluperfect_pass_3: _, "ἐ~εδεσθην", "ἐ~εδεντο";
    infinitive_fut: _, _, "~εθησεσθαι";
    infinitive_aor: _, _, "~εθηναι";
    infinitive_perf: "~εδεκεναι", "~εδεσθαι", _;
    participle_fut: _, _, "~εθησομενος";
    participle_aor: _, _, "~εθεις";
    participle_perf: "~εδεκως", "~εδεμενος", _;
    ),
    // Σῠνᾰγω: 12 words (σῠνᾰγω, ἐξᾰγω, ὑπᾰγω, ἀνᾰγω, προᾰγω, πᾰρᾰγω). Like Ἑπομαι, except in 280 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("σῠνᾰγω", 12, "~ᾰγω", like Ἑπομαι;
    ind_act_1: "~ᾰγω", _, "~ᾰγομεν";
    ind_act_2: "~ᾰγεις", _, _;
    ind_act_3: "~ᾰγει", _, "~ᾰγουσῐ|~ᾰγουσῐν";
    ind_mid_1: "~ᾰγομαι", _, "~ᾰγομεθᾰ";
    ind_mid_2: "~ᾰγῃ|~ᾰγει", "~ᾰγεσθον", "~ᾰγεσθε";
    ind_mid_3: "~ᾰγεται", _, "~ᾰγονται";
    ind_pass_1: _, _, "~ᾰγομεθᾰ";
    ind_pass_2: "~ᾰγῃ|~ᾰγει", _, "~ᾰγεσθε";
    ind_pass_3: "~ᾰγεται", _, "~ᾰγονται";
    subj_act_1: "~ᾰγω", _, "~ᾰγωμεν";
    subj_act_2: "~ᾰγῃς", "~ᾰγητον", "~ᾰγητε";
    subj_act_3: "~ᾰγῃ", _, "~ᾰγωσῐ|~ᾰγωσῐν";
    subj_mid_1: "~ᾰγωμαι", _, "~ᾰγωμεθᾰ";
    subj_mid_2: "~ᾰγῃ", "~ᾰγησθον", "~ᾰγησθε";
    subj_mid_3: "~ᾰγηται", _, "~ᾰγωνται";
    subj_pass_1: _, _, "~ᾰγωμεθᾰ";
    subj_pass_2: _, _, "~ᾰγησθε";
    subj_pass_3: "~ᾰγηται", _, "~ᾰγωνται";
    opt_act_1: "~ᾰγοιμῐ", _, "~ᾰγοιμεν";
    opt_act_2: "~ᾰγοις", "~ᾰγοιτον", "~ᾰγοιτε";
    opt_act_3: "~ᾰγοι", "~ᾰγοιτην", "~ᾰγοιεν";
    opt_mid_1: "~ᾰγοιμην", _, "~ᾰγοιμεθᾰ";
    opt_mid_2: "~ᾰγοιο", "~ᾰγοισθον", "~ᾰγοισθε";
    opt_mid_3: "~ᾰγοιτο", "~ᾰγοισθην", "~ᾰγοιντο";
    opt_pass_1: _, _, "~ᾰγοιμεθᾰ";
    opt_pass_2: _, "~ᾰγοισθον", "~ᾰγοισθε";
    opt_pass_3: "~ᾰγοιτο", "~ᾰγοισθην", "~ᾰγοιντο";
    imp_act_2: "~ᾰγε", "~ᾰγετον", "~ᾰγετε";
    imp_act_3: "~ᾰγετω", "~ᾰγετων", "~ᾰγοντων";
    imp_mid_2: "~ᾰγου", _, "~ᾰγεσθε";
    imp_mid_3: "~ᾰγεσθω", "~ᾰγεσθων", "~ᾰγεσθων";
    imp_pass_2: _, _, "~ᾰγεσθε";
    imp_pass_3: _, "~ᾰγεσθων", _;
    imperfect_act_1: "~ηγον", _, "~ηγομεν";
    imperfect_act_2: "~ηγες", "~ηγετον", "~ηγετε";
    imperfect_act_3: "~ηγε|~ηγεν", "~ηγετην", "~ηγον";
    imperfect_mid_1: "~ηγομην", _, "~ηγομεθᾰ";
    imperfect_mid_2: "~ηγου", "~ηγεσθον", "~ηγεσθε";
    imperfect_mid_3: "~ηγετο", "~ηγεσθην", "~ηγοντο";
    imperfect_pass_1: _, _, "~ηγομεθᾰ";
    imperfect_pass_2: _, _, "~ηγεσθε";
    imperfect_pass_3: "~ηγετο", _, _;
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
    aorist_ind_act_1: "~ηγᾰγον", _, "~ηγᾰγομεν";
    aorist_ind_act_2: "~ηγᾰγες", "~ηγᾰγετον", "~ηγᾰγετε";
    aorist_ind_act_3: "~ηγᾰγε", "~ηγᾰγετην", "~ηγᾰγον";
    aorist_ind_mid_1: "~ηγᾰγομην", _, "~ηγᾰγομεθᾰ";
    aorist_ind_mid_2: "~ηγᾰγου", "~ηγᾰγεσθον", "~ηγᾰγεσθε";
    aorist_ind_mid_3: "~ηγᾰγετο", "~ηγᾰγεσθην", "~ηγᾰγοντο";
    aorist_ind_pass_1: "~ηχθην", _, "~ηχθημεν";
    aorist_ind_pass_2: "~ηχθης", "~ηχθητον", "~ηχθητε";
    aorist_ind_pass_3: "~ηχθη", "~ηχθητην", "~ηχθησᾰν";
    aorist_subj_act_1: "~ᾰγᾰγω", _, "~ᾰγᾰγωμεν";
    aorist_subj_act_2: "~ᾰγᾰγῃς", "~ᾰγᾰγητον", "~ᾰγᾰγητε";
    aorist_subj_act_3: "~ᾰγᾰγῃ", _, "~ᾰγᾰγωσῐ";
    aorist_subj_mid_1: "~ᾰγᾰγωμαι", _, "~ᾰγᾰγωμεθᾰ";
    aorist_subj_mid_2: "~ᾰγᾰγῃ", "~ᾰγᾰγησθον", "~ᾰγᾰγησθε";
    aorist_subj_mid_3: "~ᾰγᾰγηται", _, "~ᾰγᾰγωνται";
    aorist_subj_pass_1: "~ᾰχθω", _, "~ᾰχθωμεν";
    aorist_subj_pass_2: "~ᾰχθῃς", "~ᾰχθητον", "~ᾰχθητε";
    aorist_subj_pass_3: "~ᾰχθῃ", _, "~ᾰχθωσῐ|~ᾰχθωσῐν";
    aorist_opt_act_1: "~ᾰγᾰγοιμῐ", _, "~ᾰγᾰγοιμεν";
    aorist_opt_act_2: "~ᾰγᾰγοις", "~ᾰγᾰγοιτον", "~ᾰγᾰγοιτε";
    aorist_opt_act_3: "~ᾰγᾰγοι", "~ᾰγᾰγοιτην", "~ᾰγᾰγοιεν";
    aorist_opt_mid_1: "~ᾰγᾰγοιμην", _, "~ᾰγᾰγοιμεθᾰ";
    aorist_opt_mid_2: "~ᾰγᾰγοιο", "~ᾰγᾰγοισθον", "~ᾰγᾰγοισθε";
    aorist_opt_mid_3: "~ᾰγᾰγοιτο", "~ᾰγᾰγοισθην", "~ᾰγᾰγοιντο";
    aorist_opt_pass_1: "~ᾰχθειην", _, "~ᾰχθεῑμεν|~ᾰχθειημεν";
    aorist_opt_pass_2: "~ᾰχθειης", "~ᾰχθεῑτον|~ᾰχθειητον", "~ᾰχθεῑτε|~ᾰχθειητε";
    aorist_opt_pass_3: "~ᾰχθειη", "~ᾰχθειτην|~ᾰχθειητην", "~ᾰχθεῑεν|~ᾰχθειησᾰν";
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
    perfect_ind_pass_1: _, _, "~ηγμεθᾰ";
    perfect_ind_pass_2: _, _, "~ηχθε";
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
    perfect_imp_pass_2: "~ηξο", _, _;
    perfect_imp_pass_3: "~ηχθω", "~ηχθων", _;
    infinitive_pres: "~ᾰγειν", "~ᾰγεσθαι", _;
    infinitive_fut: "~ᾰξειν", "~ᾰξεσθαι", "~ᾰχθησεσθαι";
    infinitive_aor: "~ᾰγᾰγεῑν", "~ᾰγᾰγεσθαι", "~ᾰχθηναι";
    infinitive_perf: "~ηχεναι", "~ηχθαι", _;
    participle_pres: "~ᾰγων", "~ᾰγομενος", _;
    participle_fut: "~ᾰξων", "~ᾰξομενος", "~ᾰχθησομενος";
    participle_aor: "~ᾰγᾰγων", "~ᾰγᾰγομενος", "~ᾰχθεις";
    participle_perf: "~ηχως", "~ηγμενος", _;
    ),
    // Ὑπᾰρχω: 12 words (ὑπᾰρχω, ὑπᾰκουω, δῐᾰνῠω, ἐξᾰρχω, ἐξᾰρτῡω, ἀπᾰρεσκω). Like Προσερχομαι, except in 298 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ὑπᾰρχω", 12, "~ᾰ~ω", like Προσερχομαι;
    ind_act_1: "~ᾰ~ω", _, "~ᾰ~ομεν";
    ind_act_2: "~ᾰ~εις", _, _;
    ind_act_3: "~ᾰ~ει", _, "~ᾰ~ουσῐ";
    ind_mid_1: "~ᾰ~ομαι", _, "~ᾰ~ομεθᾰ";
    ind_mid_2: "~ᾰ~ῃ", "~ᾰ~εσθον", "~ᾰ~εσθε";
    ind_mid_3: "~ᾰ~εται", _, "~ᾰ~ονται";
    ind_pass_1: _, _, "~ᾰ~ομεθᾰ";
    ind_pass_2: _, _, "~ᾰ~εσθε";
    ind_pass_3: "~ᾰ~εται", _, "~ᾰ~ονται";
    subj_act_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    subj_act_2: "~ᾰ~ῃς", "~ᾰ~ητον", "~ᾰ~ητε";
    subj_act_3: "~ᾰ~ῃ", _, "~ᾰ~ωσῐ";
    subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    subj_mid_2: "~ᾰ~ῃ", "~ᾰ~ησθον", "~ᾰ~ησθε";
    subj_mid_3: "~ᾰ~ηται", _, "~ᾰ~ωνται";
    subj_pass_1: _, _, "~ᾰ~ωμεθᾰ";
    subj_pass_2: _, _, "~ᾰ~ησθε";
    subj_pass_3: "~ᾰ~ηται", _, "~ᾰ~ωνται";
    opt_act_1: "~ᾰ~οιμῐ", _, "~ᾰ~οιμεν";
    opt_act_2: "~ᾰ~οις", "~ᾰ~οιτον", "~ᾰ~οιτε";
    opt_act_3: "~ᾰ~οι", "~ᾰ~οιτην", "~ᾰ~οιεν";
    opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~οιο", "~ᾰ~οισθον", "~ᾰ~οισθε";
    opt_mid_3: "~ᾰ~οιτο", "~ᾰ~οισθην", "~ᾰ~οιντο";
    opt_pass_1: _, _, "~ᾰ~οιμεθᾰ";
    opt_pass_2: _, "~ᾰ~οισθον", "~ᾰ~οισθε";
    opt_pass_3: "~ᾰ~οιτο", "~ᾰ~οισθην", "~ᾰ~οιντο";
    imp_act_2: "~ᾰ~ε", "~ᾰ~ετον", "~ᾰ~ετε";
    imp_act_3: "~ᾰ~ετω", "~ᾰ~ετων", "~ᾰ~οντων";
    imp_mid_2: "~ᾰ~ου", _, "~ᾰ~εσθε";
    imp_mid_3: "~ᾰ~εσθω", "~ᾰ~εσθων", "~ᾰ~εσθων";
    imp_pass_2: _, _, "~ᾰ~εσθε";
    imp_pass_3: _, "~ᾰ~εσθων", _;
    imperfect_act_1: "~η~ον", _, "~η~ομεν";
    imperfect_act_2: "~η~ες", "~η~ετον", "~η~ετε";
    imperfect_act_3: "~η~ε", "~η~ετην", "~η~ον";
    imperfect_mid_1: "~η~ομην", _, "~η~ομεθᾰ";
    imperfect_mid_2: "~η~ου", "~η~εσθον", "~η~εσθε";
    imperfect_mid_3: "~η~ετο", "~η~εσθην", "~η~οντο";
    imperfect_pass_1: _, _, "~η~ομεθᾰ";
    imperfect_pass_2: _, _, "~η~εσθε";
    imperfect_pass_3: "~η~ετο", _, _;
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
    aorist_ind_act_1: "~η~σᾰ", _, "~η~σᾰμεν";
    aorist_ind_act_2: "~η~σᾰς", "~η~σᾰτον", "~η~σᾰτε";
    aorist_ind_act_3: "~η~σε", "~η~σᾰτην", "~η~σᾰν";
    aorist_ind_mid_1: "~η~σᾰμην", _, "~η~σᾰμεθᾰ";
    aorist_ind_mid_2: "~η~σω", "~η~σᾰσθον", "~η~σᾰσθε";
    aorist_ind_mid_3: "~η~σᾰτο", "~η~σᾰσθην", "~η~σᾰντο";
    aorist_ind_pass_1: "~η~σθην", _, "~η~σθημεν";
    aorist_ind_pass_2: "~η~σθης", "~η~σθητον", "~η~σθητε";
    aorist_ind_pass_3: "~η~σθη", "~η~σθητην", "~η~σθησᾰν";
    aorist_subj_act_1: "~ᾰ~σω", _, "~ᾰ~σωμεν";
    aorist_subj_act_2: "~ᾰ~σῃς", "~ᾰ~σητον", "~ᾰ~σητε";
    aorist_subj_act_3: "~ᾰ~σῃ", _, "~ᾰ~σωσῐ";
    aorist_subj_mid_1: "~ᾰ~σωμαι", _, "~ᾰ~σωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~σῃ", "~ᾰ~σησθον", "~ᾰ~σησθε";
    aorist_subj_mid_3: "~ᾰ~σηται", _, "~ᾰ~σωνται";
    aorist_subj_pass_1: "~ᾰ~σθω", _, "~ᾰ~σθωμεν";
    aorist_subj_pass_2: "~ᾰ~σθῃς", "~ᾰ~σθητον", "~ᾰ~σθητε";
    aorist_subj_pass_3: "~ᾰ~σθῃ", _, "~ᾰ~σθωσῐ";
    aorist_opt_act_1: "~ᾰ~σαιμῐ", _, "~ᾰ~σαιμεν";
    aorist_opt_act_2: "~ᾰ~σειᾰς", "~ᾰ~σαιτον", "~ᾰ~σαιτε";
    aorist_opt_act_3: "~ᾰ~σειε", "~ᾰ~σαιτην", "~ᾰ~σειᾰν";
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
    perfect_ind_pass_1: _, _, "~η~σμεθᾰ";
    perfect_ind_pass_2: _, _, "~η~σθε";
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
    perfect_imp_pass_2: "~η~σο", _, _;
    perfect_imp_pass_3: "~η~σθω", "~η~σθων", _;
    pluperfect_act_1: "~η~κειν", _, "~η~κεμεν";
    pluperfect_act_2: "~η~κεις", "~η~κετον", "~η~κετε";
    pluperfect_act_3: "~η~κει", "~η~κετην", "~η~κεσᾰν";
    pluperfect_mid_1: "~η~σμην", _, "~η~σμεθᾰ";
    pluperfect_mid_2: "~η~σο", "~η~σθον", "~η~σθε";
    pluperfect_mid_3: "~η~στο", "~η~σθην", _;
    pluperfect_pass_1: _, _, "~η~σμεθᾰ";
    pluperfect_pass_2: _, _, "~η~σθε";
    pluperfect_pass_3: _, "~η~σθην", "~η~τᾰτο";
    infinitive_pres: "~ᾰ~ειν", "~ᾰ~εσθαι", _;
    infinitive_fut: "~ᾰ~σειν", "~ᾰ~σεσθαι", "~ᾰ~σθησεσθαι";
    infinitive_aor: "~ᾰ~σαι", "~ᾰ~σᾰσθαι", "~ᾰ~σθηναι";
    infinitive_perf: "~η~κεναι", "~η~σθαι", _;
    participle_pres: "~ᾰ~ων", "~ᾰ~ομενος", _;
    participle_fut: "~ᾰ~σων", "~ᾰ~σομενος", "~ᾰ~σθησομενος";
    participle_aor: "~ᾰ~σᾱς", "~ᾰ~σᾰμενος", "~ᾰ~σθεις";
    participle_perf: "~η~κως", "~η~σμενος", _;
    ),
    // Δῐοικεω: 12 words (δῐοικοδομεω, ἐνοικεω, προσοικεω, κᾰτοικοφθορεω, κατοικονομεω, κᾰτοικοδομεω). Like Προσερχομαι, except in 300 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("δῐοικεω", 12, "~οι~εω", like Προσερχομαι;
    ind_act_1: "~οι~ω", _, "~οι~οῡμεν";
    ind_act_2: "~οι~εῑς", _, _;
    ind_act_3: "~οι~εῑ", _, "~οι~οῡσῐ|~οι~οῡσῐν";
    ind_mid_1: "~οι~οῡμαι", _, "~οι~ουμεθᾰ";
    ind_mid_2: "~οι~εῑ|~οι~ῃ", "~οι~εῑσθον", "~οι~εῑσθε";
    ind_mid_3: "~οι~εῑται", _, "~οι~οῡνται";
    ind_pass_1: _, _, "~οι~ουμεθᾰ";
    ind_pass_2: "~οι~εῑ|~οι~ῃ", _, "~οι~εῑσθε";
    ind_pass_3: "~οι~εῑται", _, "~οι~οῡνται";
    subj_act_1: "~οι~ω", _, "~οι~ωμεν";
    subj_act_2: "~οι~ῃς", "~οι~ητον", "~οι~ητε";
    subj_act_3: "~οι~ῃ", _, "~οι~ωσῐ|~οι~ωσῐν";
    subj_mid_1: "~οι~ωμαι", _, "~οι~ωμεθᾰ";
    subj_mid_2: "~οι~ῃ", "~οι~ησθον", "~οι~ησθε";
    subj_mid_3: "~οι~ηται", _, "~οι~ωνται";
    subj_pass_1: _, _, "~οι~ωμεθᾰ";
    subj_pass_2: _, _, "~οι~ησθε";
    subj_pass_3: "~οι~ηται", _, "~οι~ωνται";
    opt_act_1: "~οι~οιην|~οι~οῑμῐ", _, "~οι~οῑμεν|~οι~οιημεν";
    opt_act_2: "~οι~οιης|~οι~οῑς", "~οι~οῑτον|~οι~οιητον", "~οι~οῑτε|~οι~οιητε";
    opt_act_3: "~οι~οιη|~οι~οῑ", "~οι~οιτην|~οι~οιητην", "~οι~οῑεν|~οι~οιησᾰν";
    opt_mid_1: "~οι~οιμην", _, "~οι~οιμεθᾰ";
    opt_mid_2: "~οι~οῑο", "~οι~οῑσθον", "~οι~οῑσθε";
    opt_mid_3: "~οι~οῑτο", "~οι~οισθην", "~οι~οῑντο";
    opt_pass_1: _, _, "~οι~οιμεθᾰ";
    opt_pass_2: _, "~οι~οῑσθον", "~οι~οῑσθε";
    opt_pass_3: "~οι~οῑτο", "~οι~οισθην", "~οι~οῑντο";
    imp_act_2: "~οι~ει", "~οι~εῑτον", "~οι~εῑτε";
    imp_act_3: "~οι~ειτω", "~οι~ειτων", "~οι~ουντων";
    imp_mid_2: "~οι~οῡ", _, "~οι~εῑσθε";
    imp_mid_3: "~οι~εισθω", "~οι~εισθων", "~οι~εισθων";
    imp_pass_2: _, _, "~οι~εῑσθε";
    imp_pass_3: _, "~οι~εισθων", _;
    imperfect_act_1: "~ῳ~ουν", _, "~ῳ~οῡμεν";
    imperfect_act_2: "~ῳ~εις", "~ῳ~εῑτον", "~ῳ~εῑτε";
    imperfect_act_3: "~ῳ~ει", "~ῳ~ειτην", "~ῳ~ουν";
    imperfect_mid_1: "~ῳ~ουμην", _, "~ῳ~ουμεθᾰ";
    imperfect_mid_2: "~ῳ~οῡ", "~ῳ~εῑσθον", "~ῳ~εῑσθε";
    imperfect_mid_3: "~ῳ~εῑτο", "~ῳ~εισθην", "~ῳ~οῡντο";
    imperfect_pass_1: _, _, "~ῳ~ουμεθᾰ";
    imperfect_pass_2: _, _, "~ῳ~εῑσθε";
    imperfect_pass_3: "~ῳ~εῑτο", _, _;
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
    aorist_ind_act_1: "~ῳ~ησᾰ", _, "~ῳ~ησᾰμεν";
    aorist_ind_act_2: "~ῳ~ησᾰς", "~ῳ~ησᾰτον", "~ῳ~ησᾰτε";
    aorist_ind_act_3: "~ῳ~ησε|~ῳ~ησεν", "~ῳ~ησᾰτην", "~ῳ~ησᾰν";
    aorist_ind_mid_1: "~ῳ~ησᾰμην", _, "~ῳ~ησᾰμεθᾰ";
    aorist_ind_mid_2: "~ῳ~ησω", "~ῳ~ησᾰσθον", "~ῳ~ησᾰσθε";
    aorist_ind_mid_3: "~ῳ~ησᾰτο", "~ῳ~ησᾰσθην", "~ῳ~ησᾰντο";
    aorist_ind_pass_1: "~ῳ~ηθην", _, "~ῳ~ηθημεν";
    aorist_ind_pass_2: "~ῳ~ηθης", "~ῳ~ηθητον", "~ῳ~ηθητε";
    aorist_ind_pass_3: "~ῳ~ηθη", "~ῳ~ηθητην", "~ῳ~ηθησᾰν";
    aorist_subj_act_1: "~οι~ησω", _, "~οι~ησωμεν";
    aorist_subj_act_2: "~οι~ησῃς", "~οι~ησητον", "~οι~ησητε";
    aorist_subj_act_3: "~οι~ησῃ", _, "~οι~ησωσῐ|~οι~ησωσῐν";
    aorist_subj_mid_1: "~οι~ησωμαι", _, "~οι~ησωμεθᾰ";
    aorist_subj_mid_2: "~οι~ησῃ", "~οι~ησησθον", "~οι~ησησθε";
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
    perfect_ind_pass_1: _, _, "~ῳ~ημεθᾰ";
    perfect_ind_pass_2: _, _, "~ῳ~ησθε";
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
    perfect_imp_pass_2: "~ῳ~ησο", _, _;
    perfect_imp_pass_3: "~ῳ~ησθω", "~ῳ~ησθων", _;
    pluperfect_act_1: "~ῳ~ηκειν|~ῳ~ηκη", _, "~ῳ~ηκεμεν";
    pluperfect_act_2: "~ῳ~ηκεις|~ῳ~ηκης", "~ῳ~ηκετον", "~ῳ~ηκετε";
    pluperfect_act_3: "~ῳ~ηκει|~ῳ~ηκειν", "~ῳ~ηκετην", "~ῳ~ηκεσᾰν";
    pluperfect_mid_1: "~ῳ~ημην", _, "~ῳ~ημεθᾰ";
    pluperfect_mid_2: "~ῳ~ησο", "~ῳ~ησθον", "~ῳ~ησθε";
    pluperfect_mid_3: "~ῳ~ητο", "~ῳ~ησθην", _;
    pluperfect_pass_1: _, _, "~ῳ~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ῳ~ησθε";
    pluperfect_pass_3: _, "~ῳ~ησθην", "~ῳ~ηντο";
    infinitive_pres: "~οι~εῑν", "~οι~εῑσθαι", _;
    infinitive_fut: "~οι~ησειν", "~οι~ησεσθαι", "~οι~ηθησεσθαι";
    infinitive_aor: "~οι~ησαι", "~οι~ησᾰσθαι", "~οι~ηθηναι";
    infinitive_perf: "~ῳ~ηκεναι", "~ῳ~ησθαι", _;
    participle_pres: "~οι~ων", "~οι~ουμενος", _;
    participle_fut: "~οι~ησων", "~οι~ησομενος", "~οι~ηθησομενος";
    participle_aor: "~οι~ησᾱς", "~οι~ησᾰμενος", "~οι~ηθεις";
    participle_perf: "~ῳ~ηκως", "~ῳ~ημενος", _;
    ),
    // Πᾰρειμι: 12 words (ἐξειμι, πᾰρειμι, ἐπειμι, ἀπειμι, κᾰτειμῐ, προσειμι). Like Θωᾰω, except in 92 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("πᾰρειμι", 12, "~ειμι", like Θωᾰω;
    ind_act_1: "~ειμι", _, "~ῐμεν";
    ind_act_2: "~ει", _, _;
    ind_act_3: "~εισῐ", _, "~ῐᾱσῐ";
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, derived, derived;
    ind_mid_3: derived, _, derived;
    ind_pass_1: _, _, derived;
    ind_pass_2: _, _, derived;
    ind_pass_3: derived, _, derived;
    subj_act_1: "~ῐω", _, "~ῐωμεν";
    subj_act_2: "~ῐῃς", "~ῐητον", "~ῐητε";
    subj_act_3: "~ῐῃ", _, "~ῐωσῐ";
    subj_mid_1: derived, _, derived;
    subj_mid_2: derived, derived, derived;
    subj_mid_3: derived, _, derived;
    subj_pass_1: _, _, derived;
    subj_pass_2: _, _, derived;
    subj_pass_3: derived, _, derived;
    opt_act_1: "~ῐοιμῐ", _, "~ῐοιμεν";
    opt_act_2: "~ῐοις", "~ῐοιτον", "~ῐοιτε";
    opt_act_3: "~ῐοι", "~ῐοιτην", "~ῐοιεν";
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, derived;
    opt_mid_3: derived, derived, derived;
    opt_pass_1: _, _, derived;
    opt_pass_2: _, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: "~ῐθῐ", "~ῐτον", "~ῐτε";
    imp_act_3: "~ῐτω", "~ῐτων", "~ῐοντων";
    imp_mid_2: derived, _, derived;
    imp_mid_3: derived, derived, derived;
    imp_pass_2: _, _, derived;
    imp_pass_3: _, derived, _;
    imperfect_act_1: "~ῃᾰ", _, "~ῃμεν";
    imperfect_act_2: "~ῃεισθᾰ", "~ῃτον", "~ῃτε";
    imperfect_act_3: "~ῃειν", "~ῃτην", "~ῃσᾰν";
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, derived;
    imperfect_mid_3: derived, derived, derived;
    imperfect_pass_1: _, _, derived;
    imperfect_pass_2: _, _, derived;
    imperfect_pass_3: derived, _, _;
    infinitive_pres: "~ῐεναι", derived, _;
    participle_pres: "~ῐων", derived, _;
    ),
    // Ἐνᾰντῐοομαι: 11 words (στραγγᾰλοομαι, χειροομαι, ἀπῐχθῠοομαι, κᾰτοικειοομαι, ἐνᾰντῐοομαι, διπλᾰσῐοομαι). Like Γῡροω, except in 122 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, subj_act_1 sg, ….
    conjugation!("ἐνᾰντῐοομαι", 11, "~οομαι", like Γῡροω;
    ind_act_1: "~οομαι", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, _, derived;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imp_act_3: derived, derived, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
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
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_imp_act_2: derived, derived, derived;
    aorist_imp_act_3: derived, derived, derived;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: derived, derived, derived;
    perfect_ind_act_3: derived, _, derived;
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: _, derived, derived;
    perfect_imp_act_3: derived, derived, derived;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    infinitive_perf: derived, _, _;
    participle_pres: derived, _, _;
    participle_fut: derived, _, _;
    participle_aor: derived, _, _;
    participle_perf: derived, _, _;
    ),
    // Λῡω: 11 words (λῡω, δῡω, καμμῡω, καταμῡω, μῡω, κᾰσσῡω).
    conjugation!("λῡω", 11, "~ῡω";
    ind_act_1: "~ῡω", _, "~ῡομεν";
    ind_act_2: "~ῡεις", _, _;
    ind_act_3: "~ῡει", _, "~ῡουσῐ|~ῡουσῐν";
    ind_mid_1: "~ῡομαι", _, "~ῡομεθᾰ";
    ind_mid_2: "~ῡῃ|~ῡει", "~ῡεσθον", "~ῡεσθε";
    ind_mid_3: "~ῡεται", _, "~ῡονται";
    ind_pass_1: _, _, "~ῡομεθᾰ";
    ind_pass_2: "~ῡῃ|~ῡει", _, "~ῡεσθε";
    ind_pass_3: "~ῡεται", _, "~ῡονται";
    subj_act_1: "~ῡω", _, "~ῡωμεν";
    subj_act_2: "~ῡῃς", "~ῡητον", "~ῡητε";
    subj_act_3: "~ῡῃ", _, "~ῡωσῐ|~ῡωσῐν";
    subj_mid_1: "~ῡωμαι", _, "~ῡωμεθᾰ";
    subj_mid_2: "~ῡῃ", "~ῡησθον", "~ῡησθε";
    subj_mid_3: "~ῡηται", _, "~ῡωνται";
    subj_pass_1: _, _, "~ῡωμεθᾰ";
    subj_pass_2: _, _, "~ῡησθε";
    subj_pass_3: "~ῡηται", _, "~ῡωνται";
    opt_act_1: "~ῡοιμῐ", _, "~ῡοιμεν";
    opt_act_2: "~ῡοις", "~ῡοιτον", "~ῡοιτε";
    opt_act_3: "~ῡοι", "~ῡοιτην", "~ῡοιεν";
    opt_mid_1: "~ῡοιμην", _, "~ῡοιμεθᾰ";
    opt_mid_2: "~ῡοιο", "~ῡοισθον", "~ῡοισθε";
    opt_mid_3: "~ῡοιτο", "~ῡοισθην", "~ῡοιντο";
    opt_pass_1: _, _, "~ῡοιμεθᾰ";
    opt_pass_2: _, "~ῡοισθον", "~ῡοισθε";
    opt_pass_3: "~ῡοιτο", "~ῡοισθην", "~ῡοιντο";
    imp_act_2: "~ῡε", "~ῡετον", "~ῡετε";
    imp_act_3: "~ῡετω", "~ῡετων", "~ῡοντων";
    imp_mid_2: "~ῡου", _, "~ῡεσθε";
    imp_mid_3: "~ῡεσθω", "~ῡεσθων", "~ῡεσθων";
    imp_pass_2: _, _, "~ῡεσθε";
    imp_pass_3: _, "~ῡεσθων", _;
    imperfect_act_1: "ἐ~ῡον", _, "ἐ~ῡομεν";
    imperfect_act_2: "ἐ~ῡες", "ἐ~ῡετον", "ἐ~ῡετε";
    imperfect_act_3: "ἐ~ῡε", "ἐ~ῡετην", "ἐ~ῡον";
    imperfect_mid_1: "ἐ~ῡομην", _, "ἐ~ῡομεθᾰ";
    imperfect_mid_2: "ἐ~ῡου", "ἐ~ῡεσθον", "ἐ~ῡεσθε";
    imperfect_mid_3: "ἐ~ῡετο", "ἐ~ῡεσθην", "ἐ~ῡοντο";
    imperfect_pass_1: _, _, "ἐ~ῡομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ῡεσθε";
    imperfect_pass_3: "ἐ~ῡετο", _, _;
    future_ind_act_1: "~ῡσω", _, "~ῡσομεν";
    future_ind_act_2: "~ῡσεις", "~ῡσετον", "~ῡσετε";
    future_ind_act_3: "~ῡσει", _, "~ῡσουσῐ";
    future_ind_mid_1: "~ῡσομαι", _, "~ῡσομεθᾰ";
    future_ind_mid_2: "~ῡσῃ", "~ῡσεσθον", "~ῡσεσθε";
    future_ind_mid_3: "~ῡσεται", _, "~ῡσονται";
    future_ind_pass_1: "~ῠθησομαι", _, "~ῠθησομεθᾰ";
    future_ind_pass_2: "~ῠθησῃ|~ῠθησει", "~ῠθησεσθον", "~ῠθησεσθε";
    future_ind_pass_3: "~ῠθησεται", _, "~ῠθησονται";
    future_opt_act_1: "~ῡσοιμῐ", _, "~ῡσοιμεν";
    future_opt_act_2: "~ῡσοις", "~ῡσοιτον", "~ῡσοιτε";
    future_opt_act_3: "~ῡσοι", "~ῡσοιτην", "~ῡσοιεν";
    future_opt_mid_1: "~ῡσοιμην", _, "~ῡσοιμεθᾰ";
    future_opt_mid_2: "~ῡσοιο", "~ῡσοισθον", "~ῡσοισθε";
    future_opt_mid_3: "~ῡσοιτο", "~ῡσοισθην", "~ῡσοιντο";
    future_opt_pass_1: "~ῠθησοιμην", _, "~ῠθησοιμεθᾰ";
    future_opt_pass_2: "~ῠθησοιο", "~ῠθησοισθον", "~ῠθησοισθε";
    future_opt_pass_3: "~ῠθησοιτο", "~ῠθησοισθην", "~ῠθησοιντο";
    aorist_ind_act_1: "ἐ~ῡσᾰ", _, "ἐ~ῡσᾰμεν";
    aorist_ind_act_2: "ἐ~ῡσᾰς", "ἐ~ῡσᾰτον", "ἐ~ῡσᾰτε";
    aorist_ind_act_3: "ἐ~ῡσε", "ἐ~ῡσᾰτην", "ἐ~ῡσᾰν";
    aorist_ind_mid_1: "ἐ~ῡσᾰμην", _, "ἐ~ῡσᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ῡσω", "ἐ~ῡσᾰσθον", "ἐ~ῡσᾰσθε";
    aorist_ind_mid_3: "ἐ~ῡσᾰτο", "ἐ~ῡσᾰσθην", "ἐ~ῡσᾰντο";
    aorist_ind_pass_1: "ἐ~ῠθην", _, "ἐ~ῠθημεν";
    aorist_ind_pass_2: "ἐ~ῠθης", "ἐ~ῠθητον", "ἐ~ῠθητε";
    aorist_ind_pass_3: "ἐ~ῠθη", "ἐ~ῠθητην", "ἐ~ῠθησᾰν";
    aorist_subj_act_1: "~ῡσω", _, "~ῡσωμεν";
    aorist_subj_act_2: "~ῡσῃς", "~ῡσητον", "~ῡσητε";
    aorist_subj_act_3: "~ῡσῃ", _, "~ῡσωσῐ";
    aorist_subj_mid_1: "~ῡσωμαι", _, "~ῡσωμεθᾰ";
    aorist_subj_mid_2: "~ῡσῃ", "~ῡσησθον", "~ῡσησθε";
    aorist_subj_mid_3: "~ῡσηται", _, "~ῡσωνται";
    aorist_subj_pass_1: "~ῠθω", _, "~ῠθωμεν";
    aorist_subj_pass_2: "~ῠθῃς", "~ῠθητον", "~ῠθητε";
    aorist_subj_pass_3: "~ῠθῃ", _, "~ῠθωσῐ|~ῠθωσῐν";
    aorist_opt_act_1: "~ῡσαιμῐ", _, "~ῡσαιμεν";
    aorist_opt_act_2: "~ῡσειᾰς", "~ῡσαιτον", "~ῡσαιτε";
    aorist_opt_act_3: "~ῡσειε", "~ῡσαιτην", "~ῡσειᾰν";
    aorist_opt_mid_1: "~ῡσαιμην", _, "~ῡσαιμεθᾰ";
    aorist_opt_mid_2: "~ῡσαιο", "~ῡσαισθον", "~ῡσαισθε";
    aorist_opt_mid_3: "~ῡσαιτο", "~ῡσαισθην", "~ῡσαιντο";
    aorist_opt_pass_1: "~ῠθειην", _, "~ῠθεῑμεν|~ῠθειημεν";
    aorist_opt_pass_2: "~ῠθειης", "~ῠθεῑτον|~ῠθειητον", "~ῠθεῑτε|~ῠθειητε";
    aorist_opt_pass_3: "~ῠθειη", "~ῠθειτην|~ῠθειητην", "~ῠθεῑεν|~ῠθειησᾰν";
    aorist_imp_act_2: "~ῡσον", "~ῡσᾰτον", "~ῡσᾰτε";
    aorist_imp_act_3: "~ῡσᾰτω", "~ῡσᾰτων", "~ῡσᾰντων";
    aorist_imp_mid_2: "~ῡσαι", "~ῡσᾰσθον", "~ῡσᾰσθε";
    aorist_imp_mid_3: "~ῡσᾰσθω", "~ῡσᾰσθων", _;
    aorist_imp_pass_2: "~ῠθητῐ", _, _;
    aorist_imp_pass_3: "~ῠθητω", "~ῠθητων", "~ῠθεντων";
    perfect_ind_act_1: "%ε~ῠκᾰ", _, "%ε~ῠκᾰμεν";
    perfect_ind_act_2: "%ε~ῠκᾰς", "%ε~ῠκᾰτον", "%ε~ῠκᾰτε";
    perfect_ind_act_3: "%ε~ῠκε", _, "%ε~ῠκᾱσῐ";
    perfect_ind_mid_1: "%ε~ῠμαι", _, "%ε~ῠμεθᾰ";
    perfect_ind_mid_2: "%ε~ῠσαι", "%ε~ῠσθον", "%ε~ῠσθε";
    perfect_ind_mid_3: "%ε~ῠται", _, "%ε~ῠνται";
    perfect_ind_pass_1: _, _, "%ε~ῠμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~ῠσθε";
    perfect_subj_act_1: "%ε~ῠκω", _, "%ε~ῠκωμεν";
    perfect_subj_act_2: "%ε~ῠκῃς", "%ε~ῠκητον", "%ε~ῠκητε";
    perfect_subj_act_3: "%ε~ῠκῃ", _, "%ε~ῠκωσῐ";
    perfect_opt_act_1: "%ε~ῠκοιμῐ", _, "%ε~ῠκοιμεν";
    perfect_opt_act_2: "%ε~ῠκοις", "%ε~ῠκοιτον", "%ε~ῠκοιτε";
    perfect_opt_act_3: "%ε~ῠκοι", "%ε~ῠκοιτην", "%ε~ῠκοιεν";
    perfect_imp_act_2: _, "%ε~ῠκετον", "%ε~ῠκετε";
    perfect_imp_act_3: "%ε~ῠκετω", "%ε~ῠκετων", "%ε~ῠκοντων";
    perfect_imp_mid_2: "%ε~ῠσο", _, _;
    perfect_imp_mid_3: "%ε~ῠσθω", "%ε~ῠσθων", _;
    perfect_imp_pass_2: "%ε~ῠσο", _, _;
    perfect_imp_pass_3: "%ε~ῠσθω", "%ε~ῠσθων", _;
    pluperfect_act_1: "ἐ%ε~ῠκειν", _, "ἐ%ε~ῠκεμεν";
    pluperfect_act_2: "ἐ%ε~ῠκεις", "ἐ%ε~ῠκετον", "ἐ%ε~ῠκετε";
    pluperfect_act_3: "ἐ%ε~ῠκει", "ἐ%ε~ῠκετην", "ἐ%ε~ῠκεσᾰν";
    pluperfect_mid_1: "ἐ%ε~ῠμην", _, "ἐ%ε~ῠμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ῠσο", "ἐ%ε~ῠσθον", "ἐ%ε~ῠσθε";
    pluperfect_mid_3: "ἐ%ε~ῠτο", "ἐ%ε~ῠσθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~ῠμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ῠσθε";
    pluperfect_pass_3: _, "ἐ%ε~ῠσθην", "ἐ%ε~ῠντο";
    future_perfect_ind_mid_1: "%ε~ῡσομαι", _, "%ε~ῡσομεθᾰ";
    future_perfect_ind_mid_2: "%ε~ῡσῃ", "%ε~ῡσεσθον", "%ε~ῡσεσθε";
    future_perfect_ind_mid_3: "%ε~ῡσεται", _, "%ε~ῡσονται";
    future_perfect_opt_mid_1: "%ε~ῡσοιμην", _, "%ε~ῡσοιμεθᾰ";
    future_perfect_opt_mid_2: "%ε~ῡσοιο", "%ε~ῡσοισθον", "%ε~ῡσοισθε";
    future_perfect_opt_mid_3: "%ε~ῡσοιτο", "%ε~ῡσοισθην", "%ε~ῡσοιντο";
    infinitive_pres: "~ῡειν", "~ῡεσθαι", _;
    infinitive_fut: "~ῡσειν", "~ῡσεσθαι", "~ῠθησεσθαι";
    infinitive_aor: "~ῡσαι", "~ῡσᾰσθαι", "~ῠθηναι";
    infinitive_perf: "%ε~ῠκεναι", "%ε~ῠσθαι", _;
    infinitive_futp: _, "%ε~ῡσεσθαι", _;
    participle_pres: "~ῡων", "~ῡομενος", _;
    participle_fut: "~ῡσων", "~ῡσομενος", "~ῠθησομενος";
    participle_aor: "~ῡσᾱς", "~ῡσᾰμενος", "~ῠθεις";
    participle_perf: "%ε~ῠκως", "%ε~ῠμενος", _;
    participle_futp: _, "%ε~ῡσομενος", _;
    ),
    // Παυω: 10 words (γευω, παυω, κελευω, κρουω, κᾰτᾰδυω, χναυω). Like Βουλευω, except in 49 cells: imp_act_2 sg, future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, future_ind_pass_2 du, future_ind_pass_2 pl, ….
    conjugation!("παυω", 10, "~υω", like Βουλευω;
    imp_act_2: "~ῡε", _, _;
    future_ind_pass_1: "~υσθησομαι", _, "~υσθησομεθᾰ";
    future_ind_pass_2: "~υσθησῃ", "~υσθησεσθον", "~υσθησεσθε";
    future_ind_pass_3: "~υσθησεται", _, "~υσθησονται";
    future_opt_pass_1: "~υσθησοιμην", _, "~υσθησοιμεθᾰ";
    future_opt_pass_2: "~υσθησοιο", "~υσθησοισθον", "~υσθησοισθε";
    future_opt_pass_3: "~υσθησοιτο", "~υσθησοισθην", "~υσθησοιντο";
    aorist_ind_pass_1: "ἐ~υσθην", _, "ἐ~υσθημεν";
    aorist_ind_pass_2: "ἐ~υσθης", "ἐ~υσθητον", "ἐ~υσθητε";
    aorist_ind_pass_3: "ἐ~υσθη", "ἐ~υσθητην", "ἐ~υσθησᾰν";
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
    // Θρῠπτω: 10 words (κλεπτω, βᾰπτω, ῥᾰπτω, θρῠπτω, γνᾰμπτω, μᾰρπτω). Like Ἁπτω, except in 111 cells: ind_act_3 pl, ind_mid_2 sg, ind_pass_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_3 pl, ….
    conjugation!("θρῠπτω", 10, "~πτω", like Ἁπτω;
    ind_act_3: _, _, "~πτουσῐ|~πτουσῐν";
    ind_mid_2: "~πτῃ|~πτει", _, _;
    ind_pass_2: "~πτῃ|~πτει", _, _;
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
    aorist_ind_act_3: "ἐ~ψε|ἐ~ψεν", _, _;
    aorist_subj_act_3: _, _, "~ψωσῐ|~ψωσῐν";
    aorist_opt_act_2: "~ψειᾰς|~ψαις", _, _;
    aorist_opt_act_3: "~ψειε|~ψειεν|~ψαι", _, "~ψειᾰν|~ψαιεν";
    perfect_ind_act_1: "%ε~πτᾰ", _, "%ε~πτᾰμεν";
    perfect_ind_act_2: "%ε~πτᾰς", "%ε~πτᾰτον", "%ε~πτᾰτε";
    perfect_ind_act_3: "%ε~πτε", _, "%ε~πτᾱσῐ";
    perfect_ind_mid_1: "%ε~μμαι", _, "%ε~μμεθᾰ";
    perfect_ind_mid_2: "%ε~ψαι", "%ε~φθον", "%ε~φθε";
    perfect_ind_mid_3: "%ε~πται", _, "%ε~φᾰται";
    perfect_ind_pass_1: _, _, "%ε~μμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~φθε";
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
    perfect_imp_pass_2: "%ε~ψο", _, _;
    perfect_imp_pass_3: "%ε~φθω", "%ε~φθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
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
    // Αἱρεω: 10 words (πωλεω, αἱρεω, ἀγραυλεω, αἰνεω, βῑνεω, ὑμνεω). Like Ἐχω, except in 92 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("αἱρεω", 10, "~εω", like Ἐχω;
    ind_act_1: "~εω", _, "~εομεν";
    ind_act_2: "~εεις", _, _;
    ind_act_3: "~εει", _, "~εουσῐ";
    ind_mid_1: "~εομαι", _, "~εομεθᾰ";
    ind_mid_2: "~εῃ", "~εεσθον", "~εεσθε";
    ind_mid_3: "~εεται", _, "~εονται";
    ind_pass_1: _, _, "~εομεθᾰ";
    ind_pass_2: _, _, "~εεσθε";
    ind_pass_3: "~εεται", _, "~εονται";
    subj_act_1: "~εω", _, "~εωμεν";
    subj_act_2: "~εῃς", "~εητον", "~εητε";
    subj_act_3: "~εῃ", _, "~εωσῐ";
    subj_mid_1: "~εωμαι", _, "~εωμεθᾰ";
    subj_mid_2: "~εῃ", "~εησθον", "~εησθε";
    subj_mid_3: "~εηται", _, "~εωνται";
    subj_pass_1: _, _, "~εωμεθᾰ";
    subj_pass_2: _, _, "~εησθε";
    subj_pass_3: "~εηται", _, "~εωνται";
    opt_act_1: "~εοιμῐ", _, "~εοιμεν";
    opt_act_2: "~εοις", "~εοιτον", "~εοιτε";
    opt_act_3: "~εοι", "~εοιτην", "~εοιεν";
    opt_mid_1: "~εοιμην", _, "~εοιμεθᾰ";
    opt_mid_2: "~εοιο", "~εοισθον", "~εοισθε";
    opt_mid_3: "~εοιτο", "~εοισθην", "~εοιντο";
    opt_pass_1: _, _, "~εοιμεθᾰ";
    opt_pass_2: _, "~εοισθον", "~εοισθε";
    opt_pass_3: "~εοιτο", "~εοισθην", "~εοιντο";
    imp_act_2: "~εε", "~εετον", "~εετε";
    imp_act_3: "~εετω", "~εετων", "~εοντων";
    imp_mid_2: "~εου", _, "~εεσθε";
    imp_mid_3: "~εεσθω", "~εεσθων", "~εεσθων";
    imp_pass_2: _, _, "~εεσθε";
    imp_pass_3: _, "~εεσθων", _;
    imperfect_act_1: "ἐ~εον", _, "ἐ~εομεν";
    imperfect_act_2: "ἐ~εες", "ἐ~εετον", "ἐ~εετε";
    imperfect_act_3: "ἐ~εε", "ἐ~εετην", "ἐ~εον";
    imperfect_mid_1: "ἐ~εομην", _, "ἐ~εομεθᾰ";
    imperfect_mid_2: "ἐ~εου", "ἐ~εεσθον", "ἐ~εεσθε";
    imperfect_mid_3: "ἐ~εετο", "ἐ~εεσθην", "ἐ~εοντο";
    imperfect_pass_1: _, _, "ἐ~εομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~εεσθε";
    imperfect_pass_3: "ἐ~εετο", _, _;
    infinitive_pres: "~εειν", "~εεσθαι", _;
    participle_pres: "~εων", "~εομενος", _;
    ),
    // Ἀφῑημῐ: 10 words (ἀφῑημῐ, σῠνῑημῐ, ἐπᾰνῐημῐ, προϊημῐ, ἀνιημῐ, ξῠνῑημῐ). Like Ἑπομαι, except in 221 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἀφῑημῐ", 10, "~ῑημῐ", like Ἑπομαι;
    ind_act_1: "~ῑημῐ", _, "~ῑεμεν";
    ind_act_2: "~ῑης", _, _;
    ind_act_3: "~ῑησῐ|~ῑησῐν", _, "~ῑᾱσι|~ῑᾱσιν";
    ind_mid_1: "~ῑεμαι", _, "~ῑεμεθᾰ";
    ind_mid_2: "~ῑεσαι", "~ῑεσθον", "~ῑεσθε";
    ind_mid_3: "~ῑεται", _, "~ῑενται";
    ind_pass_1: _, _, "~ῑεμεθᾰ";
    ind_pass_2: _, _, "~ῑεσθε";
    ind_pass_3: "~ῑεται", _, "~ῑενται";
    subj_act_1: "~ῑω", _, "~ῑωμεν";
    subj_act_2: "~ῑῃς", "~ῑητον", "~ῑητε";
    subj_act_3: "~ῑῃ", _, "~ῑωσῐ|~ῑωσῐν";
    subj_mid_1: "~ῑωμαι", _, "~ῑωμεθᾰ";
    subj_mid_2: "~ῑῃ", "~ῑησθον", "~ῑησθε";
    subj_mid_3: "~ῑηται", _, "~ῑωνται";
    subj_pass_1: _, _, "~ῑωμεθᾰ";
    subj_pass_2: _, _, "~ῑησθε";
    subj_pass_3: "~ῑηται", _, "~ῑωνται";
    opt_act_1: "~ῑειην", _, "~ῑεῑμεν|~ῑειημεν";
    opt_act_2: "~ῑειης", "~ῑεῑτον|~ῑειητον", "~ῑεῑτε|~ῑειητε";
    opt_act_3: "~ῑειη", "~ῑειτην|~ῑειητην", "~ῑεῑεν|~ῑειησᾰν";
    opt_mid_1: "~ῑειμην", _, "~ῑειμεθᾰ|~ῑοιμεθᾰ";
    opt_mid_2: "~ῑεῑο", "~ῑεῑσθον|~ῑοῑσθον", "~ῑεῑσθε|~ῑοῑσθε";
    opt_mid_3: "~ῑεῑτο|~ῑοῑτο", "~ῑεισθην|~ῑοισθην", "~ῑεῑντο|~ῑοῑντο";
    opt_pass_1: _, _, "~ῑειμεθᾰ|~ῑοιμεθᾰ";
    opt_pass_2: _, "~ῑεῑσθον|~ῑοῑσθον", "~ῑεῑσθε|~ῑοῑσθε";
    opt_pass_3: "~ῑεῑτο|~ῑοῑτο", "~ῑεισθην|~ῑοισθην", "~ῑεῑντο|~ῑοῑντο";
    imp_act_2: "~ῑει", "~ῑετον", "~ῑετε";
    imp_act_3: "~ῑετω", "~ῑετων", "~ῑεντων";
    imp_mid_2: "~ῑεσο|~ῑου", _, "~ῑεσθε";
    imp_mid_3: "~ῑεσθω", "~ῑεσθων", "~ῑεσθων";
    imp_pass_2: "~ῑεσο|~ῑου", _, "~ῑεσθε";
    imp_pass_3: _, "~ῑεσθων", _;
    imperfect_act_1: "~ῑην", _, "~ῑεμεν";
    imperfect_act_2: "~ῑεις", "~ῑετον", "~ῑετε";
    imperfect_act_3: "~ῑει", "~ῑετην", "~ῑεσᾰν";
    imperfect_mid_1: "~ῑεμην", _, "~ῑεμεθᾰ";
    imperfect_mid_2: "~ῑεσο", "~ῑεσθον", "~ῑεσθε";
    imperfect_mid_3: "~ῑετο", "~ῑεσθην", "~ῑεντο";
    imperfect_pass_1: _, _, "~ῑεμεθᾰ";
    imperfect_pass_2: _, _, "~ῑεσθε";
    imperfect_pass_3: "~ῑετο", _, _;
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", "~ησετε";
    future_ind_act_3: "~ησει", _, "~ησουσῐ|~ησουσῐν";
    future_ind_mid_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_mid_2: "~ησῃ|~ησει", "~ησεσθον", "~ησεσθε";
    future_ind_mid_3: "~ησεται", _, "~ησονται";
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: "~ησοις", "~ησοιτον", "~ησοιτε";
    future_opt_act_3: "~ησοι", "~ησοιτην", "~ησοιεν";
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_2: "~ησοιο", "~ησοισθον", "~ησοισθε";
    future_opt_mid_3: "~ησοιτο", "~ησοισθην", "~ησοιντο";
    aorist_ind_act_1: "~ηκᾰ", _, "~ηκᾰμεν";
    aorist_ind_act_2: "~ηκᾰς", "~ηκᾰτον", "~ηκᾰτε";
    aorist_ind_act_3: "~ηκε|~ηκεν", "~ηκᾰτην", "~ηκᾰν";
    aorist_ind_mid_1: "~ηκᾰμην", _, "~ηκᾰμεθᾰ";
    aorist_ind_mid_2: "~ηκω", "~ηκᾰσθον", "~ηκᾰσθε";
    aorist_ind_mid_3: "~ηκᾰτο", "~ηκᾰσθην", "~ηκᾰντο";
    aorist_subj_act_1: "~ω", _, "~ωμεν";
    aorist_subj_act_2: "~ῃς", "~ητον", "~ητε";
    aorist_subj_act_3: "~ῃ", _, "~ωσῐ";
    aorist_opt_act_1: "~ειην", _, "~εῑμεν";
    aorist_opt_act_2: "~ειης", "~εῑτον", "~εῑτε";
    aorist_opt_act_3: "~ειη", "~ειτην", "~εῑεν";
    aorist_opt_mid_1: "~ειμην", _, "~ειμεθᾰ";
    aorist_opt_mid_2: "~εῑο", "~εῑσθον", "~εῑσθε";
    aorist_opt_mid_3: "~εῑτο", "~εισθην", "~εῑντο";
    aorist_imp_act_2: "~ες", "~ετον", "~ετε";
    aorist_imp_act_3: "~ετω", "~ετων", "~εντων";
    perfect_ind_act_1: "~εῑκᾰ", _, "~εικᾰμεν";
    perfect_ind_act_2: "~εῑκᾰς", "~εικᾰτον", "~εικᾰτε";
    perfect_ind_act_3: "~εῑκε", _, "~εικᾱσῐ";
    perfect_ind_mid_1: "~εῑμαι", _, "~ειμεθᾰ";
    perfect_ind_mid_2: "~εῑσαι", "~εῑσθον", "~εῑσθε";
    perfect_ind_mid_3: "~εῑται", _, "~εῑνται";
    perfect_ind_pass_1: _, _, "~ειμεθᾰ";
    perfect_ind_pass_2: _, _, "~εῑσθε";
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
    perfect_imp_pass_2: "~εῑσο", _, _;
    perfect_imp_pass_3: "~εισθω", "~εισθων", _;
    infinitive_pres: "~ῑεναι", "~ῑεσθαι", _;
    infinitive_fut: "~ησειν", "~ησεσθαι", _;
    infinitive_aor: "~εῑναι", _, _;
    infinitive_perf: "~εικεναι", "~εῑσθαι", _;
    participle_pres: "~ῑεις", "~ῑεμενος", _;
    participle_fut: "~ησων", "~ησομενος", _;
    participle_aor: "~εις", "~εμενος", _;
    participle_perf: "~εικως", "~ειμενος", _;
    ),
    // Ψῑλοω: 9 words (ἱδροω, στενοω, πτεροω, ψῑλοω, κεραυνοω, φορτοω). Like Γῡροω, except in 74 cells: future_ind_act_3 pl, future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, ….
    conjugation!("ψῑλοω", 9, "~οω", like Γῡροω;
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
    perfect_ind_pass_1: _, _, "ἐ~ωμεθᾰ";
    perfect_ind_pass_2: _, _, "ἐ~ωσθε";
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
    perfect_imp_pass_2: "ἐ~ωσο", _, _;
    perfect_imp_pass_3: "ἐ~ωσθω", "ἐ~ωσθων", _;
    pluperfect_act_1: "ἐ~ωκειν|ἐ~ωκη", _, "ἐ~ωκεμεν";
    pluperfect_act_2: "ἐ~ωκεις|ἐ~ωκης", "ἐ~ωκετον", "ἐ~ωκετε";
    pluperfect_act_3: "ἐ~ωκει|ἐ~ωκειν", "ἐ~ωκετην", "ἐ~ωκεσᾰν";
    pluperfect_mid_1: "ἐ~ωμην", _, "ἐ~ωμεθᾰ";
    pluperfect_mid_2: "ἐ~ωσο", "ἐ~ωσθον", "ἐ~ωσθε";
    pluperfect_mid_3: "ἐ~ωτο", "ἐ~ωσθην", _;
    pluperfect_pass_1: _, _, "ἐ~ωμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ωσθε";
    pluperfect_pass_3: _, "ἐ~ωσθην", "ἐ~ωντο";
    infinitive_perf: "ἐ~ωκεναι", "ἐ~ωσθαι", _;
    participle_perf: "ἐ~ωκως", "ἐ~ωμενος", _;
    ),
    // Μᾰντευομαι: 9 words (μᾰντευομαι, κουρευομαι, ὀττευομαι, λοιμευομαι, ἀστειευομαι, εἰρωνευομαι). Like Βουλευω, except in 131 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_2 sg, ….
    conjugation!("μᾰντευομαι", 9, "~υομαι", like Βουλευω;
    ind_act_1: "~υομαι", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, _, derived;
    ind_mid_2: "~υῃ", _, _;
    ind_pass_2: derived, _, _;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imp_act_3: derived, derived, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
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
    aorist_subj_pass_3: _, _, "~υθωσῐ";
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_pass_1: _, _, "~υθεῑμεν";
    aorist_opt_pass_2: _, "~υθεῑτον", "~υθεῑτε";
    aorist_opt_pass_3: _, "~υθειτην", "~υθεῑεν";
    aorist_imp_act_2: derived, derived, derived;
    aorist_imp_act_3: derived, derived, derived;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: derived, derived, derived;
    perfect_ind_act_3: derived, _, derived;
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: _, derived, derived;
    perfect_imp_act_3: derived, derived, derived;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    infinitive_perf: derived, _, _;
    participle_pres: derived, _, _;
    participle_fut: derived, _, _;
    participle_aor: derived, _, _;
    participle_perf: derived, _, _;
    ),
    // Καθιζω: 9 words (εὐαγγελιζω, βᾰδιζω, παιζω, καθιζω, συνᾱλιζω, βαλλιζω). Like Προσερχομαι, except in 298 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("καθιζω", 9, "~~ζω", like Προσερχομαι;
    ind_act_1: "~~ζω", _, "~~ζομεν";
    ind_act_2: "~~ζεις", _, _;
    ind_act_3: "~~ζει", _, "~~ζουσῐ|~~ζουσῐν";
    ind_mid_1: "~~ζομαι", _, "~~ζομεθᾰ";
    ind_mid_2: "~~ζῃ", "~~ζεσθον", "~~ζεσθε";
    ind_mid_3: "~~ζεται", _, "~~ζονται";
    ind_pass_1: _, _, "~~ζομεθᾰ";
    ind_pass_2: _, _, "~~ζεσθε";
    ind_pass_3: "~~ζεται", _, "~~ζονται";
    subj_act_1: "~~ζω", _, "~~ζωμεν";
    subj_act_2: "~~ζῃς", "~~ζητον", "~~ζητε";
    subj_act_3: "~~ζῃ", _, "~~ζωσῐ|~~ζωσῐν";
    subj_mid_1: "~~ζωμαι", _, "~~ζωμεθᾰ";
    subj_mid_2: "~~ζῃ", "~~ζησθον", "~~ζησθε";
    subj_mid_3: "~~ζηται", _, "~~ζωνται";
    subj_pass_1: _, _, "~~ζωμεθᾰ";
    subj_pass_2: _, _, "~~ζησθε";
    subj_pass_3: "~~ζηται", _, "~~ζωνται";
    opt_act_1: "~~ζοιμῐ", _, "~~ζοιμεν";
    opt_act_2: "~~ζοις", "~~ζοιτον", "~~ζοιτε";
    opt_act_3: "~~ζοι", "~~ζοιτην", "~~ζοιεν";
    opt_mid_1: "~~ζοιμην", _, "~~ζοιμεθᾰ";
    opt_mid_2: "~~ζοιο", "~~ζοισθον", "~~ζοισθε";
    opt_mid_3: "~~ζοιτο", "~~ζοισθην", "~~ζοιντο";
    opt_pass_1: _, _, "~~ζοιμεθᾰ";
    opt_pass_2: _, "~~ζοισθον", "~~ζοισθε";
    opt_pass_3: "~~ζοιτο", "~~ζοισθην", "~~ζοιντο";
    imp_act_2: "~~ζε", "~~ζετον", "~~ζετε";
    imp_act_3: "~~ζετω", "~~ζετων", "~~ζοντων";
    imp_mid_2: "~~ζου", _, "~~ζεσθε";
    imp_mid_3: "~~ζεσθω", "~~ζεσθων", "~~ζεσθων";
    imp_pass_2: _, _, "~~ζεσθε";
    imp_pass_3: _, "~~ζεσθων", _;
    imperfect_act_1: "ἐ~~ζον", _, "ἐ~~ζομεν";
    imperfect_act_2: "ἐ~~ζες", "ἐ~~ζετον", "ἐ~~ζετε";
    imperfect_act_3: "ἐ~~ζε|ἐ~~ζεν", "ἐ~~ζετην", "ἐ~~ζον";
    imperfect_mid_1: "ἐ~~ζομην", _, "ἐ~~ζομεθᾰ";
    imperfect_mid_2: "ἐ~~ζου", "ἐ~~ζεσθον", "ἐ~~ζεσθε";
    imperfect_mid_3: "ἐ~~ζετο", "ἐ~~ζεσθην", "ἐ~~ζοντο";
    imperfect_pass_1: _, _, "ἐ~~ζομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~~ζεσθε";
    imperfect_pass_3: "ἐ~~ζετο", _, _;
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
    aorist_ind_act_1: "ἐ~~σᾰ", _, "ἐ~~σᾰμεν";
    aorist_ind_act_2: "ἐ~~σᾰς", "ἐ~~σᾰτον", "ἐ~~σᾰτε";
    aorist_ind_act_3: "ἐ~~σε|ἐ~~σεν", "ἐ~~σᾰτην", "ἐ~~σᾰν";
    aorist_ind_mid_1: "ἐ~~σᾰμην", _, "ἐ~~σᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~~σω", "ἐ~~σᾰσθον", "ἐ~~σᾰσθε";
    aorist_ind_mid_3: "ἐ~~σᾰτο", "ἐ~~σᾰσθην", "ἐ~~σᾰντο";
    aorist_ind_pass_1: "ἐ~~σθην", _, "ἐ~~σθημεν";
    aorist_ind_pass_2: "ἐ~~σθης", "ἐ~~σθητον", "ἐ~~σθητε";
    aorist_ind_pass_3: "ἐ~~σθη", "ἐ~~σθητην", "ἐ~~σθησᾰν";
    aorist_subj_act_1: "~~σω", _, "~~σωμεν";
    aorist_subj_act_2: "~~σῃς", "~~σητον", "~~σητε";
    aorist_subj_act_3: "~~σῃ", _, "~~σωσῐ|~~σωσῐν";
    aorist_subj_mid_1: "~~σωμαι", _, "~~σωμεθᾰ";
    aorist_subj_mid_2: "~~σῃ", "~~σησθον", "~~σησθε";
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
    perfect_ind_pass_1: _, _, "%ε~~σμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~~σθε";
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
    perfect_imp_pass_2: "%ε~~σο", _, _;
    perfect_imp_pass_3: "%ε~~σθω", "%ε~~σθων", _;
    pluperfect_act_1: "ἐ%ε~~κειν", _, "ἐ%ε~~κεμεν";
    pluperfect_act_2: "ἐ%ε~~κεις", "ἐ%ε~~κετον", "ἐ%ε~~κετε";
    pluperfect_act_3: "ἐ%ε~~κει", "ἐ%ε~~κετην", "ἐ%ε~~κεσᾰν";
    pluperfect_mid_1: "ἐ%ε~~σμην", _, "ἐ%ε~~σμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~~σο", "ἐ%ε~~σθον", "ἐ%ε~~σθε";
    pluperfect_mid_3: "ἐ%ε~~στο", "ἐ%ε~~σθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~~σθε";
    pluperfect_pass_3: _, "ἐ%ε~~σθην", "ἐ%ε~~σᾰτο";
    infinitive_pres: "~~ζειν", "~~ζεσθαι", _;
    infinitive_fut: "~~εῑν", "~~εῑσθαι", "~~σθησεσθαι";
    infinitive_aor: "~ῑσα~", "~~σᾰσθαι", "~~σθηναι";
    infinitive_perf: "%ε~~κεναι", "%ε~ῑσθα~", _;
    participle_pres: "~~ζων", "~~ζομενος", _;
    participle_fut: "~~ων", "~~ουμενος", "~~σθησομενος";
    participle_aor: "~~σᾱς", "~~σᾰμενος", "~~σθεις";
    participle_perf: "%ε~~κως", "%ε~~σμενος", _;
    ),
    // Ἐαω: 9 words (ἀγαλλιαω, ἐαω, κρεμαω, θηραω, θλαω, τηταω). Like Νῑκαω, except in 216 cells: ind_act_3 pl, subj_act_3 pl, opt_act_1 sg, opt_act_1 pl, opt_act_2 sg, opt_act_2 du, ….
    conjugation!("ἐαω", 9, "~αω", like Νῑκαω;
    ind_act_3: _, _, "~ωσῐ|~ωσῐν";
    subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    opt_act_1: "~ῳην|~ῳμῐ", _, "~ῳμεν|~ῳημεν";
    opt_act_2: "~ῳης|~ῳς", "~ῳτον|~ῳητον", "~ῳτε|~ῳητε";
    opt_act_3: "~ῳη|~ῳ", "~ῳτην|~ῳητην", "~ῳεν|~ῳησᾰν";
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
    aorist_ind_act_1: "ἐ~ασᾰ", _, "ἐ~ασᾰμεν";
    aorist_ind_act_2: "ἐ~ασᾰς", "ἐ~ασᾰτον", "ἐ~ασᾰτε";
    aorist_ind_act_3: "ἐ~ασε", "ἐ~ασᾰτην", "ἐ~ασᾰν";
    aorist_ind_mid_1: "ἐ~ασᾰμην", _, "ἐ~ασᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ασω", "ἐ~ασᾰσθον", "ἐ~ασᾰσθε";
    aorist_ind_mid_3: "ἐ~ασᾰτο", "ἐ~ασᾰσθην", "ἐ~ασᾰντο";
    aorist_ind_pass_1: "ἐ~ασθην", _, "ἐ~ασθημεν";
    aorist_ind_pass_2: "ἐ~ασθης", "ἐ~ασθητον", "ἐ~ασθητε";
    aorist_ind_pass_3: "ἐ~ασθη", "ἐ~ασθητην", "ἐ~ασθησᾰν";
    aorist_subj_act_1: "~ασω", _, "~ασωμεν";
    aorist_subj_act_2: "~ασῃς", "~ασητον", "~ασητε";
    aorist_subj_act_3: "~ασῃ", _, "~ασωσῐ";
    aorist_subj_mid_1: "~ασωμαι", _, "~ασωμεθᾰ";
    aorist_subj_mid_2: "~ασῃ", "~ασησθον", "~ασησθε";
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
    perfect_ind_pass_1: _, _, "%ε~αμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~ασθε";
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
    perfect_imp_pass_2: "%ε~ασο", _, _;
    perfect_imp_pass_3: "%ε~ασθω", "%ε~ασθων", _;
    pluperfect_act_1: "ἐ%ε~ακειν", _, "ἐ%ε~ακεμεν";
    pluperfect_act_2: "ἐ%ε~ακεις", "ἐ%ε~ακετον", "ἐ%ε~ακετε";
    pluperfect_act_3: "ἐ%ε~ακει", "ἐ%ε~ακετην", "ἐ%ε~ακεσᾰν";
    pluperfect_mid_1: "ἐ%ε~αμην", _, "ἐ%ε~αμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ασο", "ἐ%ε~ασθον", "ἐ%ε~ασθε";
    pluperfect_mid_3: "ἐ%ε~ατο", "ἐ%ε~ασθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~αμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ασθε";
    pluperfect_pass_3: _, "ἐ%ε~ασθην", "ἐ%ε~αντο";
    infinitive_fut: "~ασειν", "~ασεσθαι", "~ασθησεσθαι";
    infinitive_aor: "~ᾱσαι", "~ασᾰσθαι", "~ασθηναι";
    infinitive_perf: "%ε~ακεναι", "%ε~ᾱσθαι", _;
    participle_fut: "~ασων", "~ασομενος", "~ασθησομενος";
    participle_aor: "~ασᾱς", "~ασᾰμενος", "~ασθεις";
    participle_perf: "%ε~ακως", "%ε~αμενος", _;
    ),
    // Κᾰτηγορεω: 9 words (κᾰταφρονεω, κᾰταργεω, κᾰτηγορεω, λοιπογρᾰφεω, τερᾰτολογεω, τερᾰτουργεω). Like Προσερχομαι, except in 299 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("κᾰτηγορεω", 9, "~ᾰ~εω", like Προσερχομαι;
    ind_act_1: "~α~ω", _, "~α~οῡμεν";
    ind_act_2: "~α~εῑς", _, _;
    ind_act_3: "~α~εῑ", _, "~α~οῡσῐ|~α~οῡσῐν";
    ind_mid_1: "~α~οῡμαι", _, "~α~ουμεθᾰ";
    ind_mid_2: "~α~εῑ|~α~ῃ", "~α~εῑσθον", "~α~εῑσθε";
    ind_mid_3: "~α~εῑται", _, "~α~οῡνται";
    ind_pass_1: _, _, "~α~ουμεθᾰ";
    ind_pass_2: "~α~εῑ|~α~ῃ", _, "~α~εῑσθε";
    ind_pass_3: "~α~εῑται", _, "~α~οῡνται";
    subj_act_1: "~α~ω", _, "~α~ωμεν";
    subj_act_2: "~α~ῃς", "~α~ητον", "~α~ητε";
    subj_act_3: "~α~ῃ", _, "~α~ωσῐ|~α~ωσῐν";
    subj_mid_1: "~α~ωμαι", _, "~α~ωμεθᾰ";
    subj_mid_2: "~α~ῃ", "~α~ησθον", "~α~ησθε";
    subj_mid_3: "~α~ηται", _, "~α~ωνται";
    subj_pass_1: _, _, "~α~ωμεθᾰ";
    subj_pass_2: _, _, "~α~ησθε";
    subj_pass_3: "~α~ηται", _, "~α~ωνται";
    opt_act_1: "~α~οιην|~α~οῑμῐ", _, "~α~οῑμεν|~α~οιημεν";
    opt_act_2: "~α~οιης|~α~οῑς", "~α~οῑτον|~α~οιητον", "~α~οῑτε|~α~οιητε";
    opt_act_3: "~α~οιη|~α~οῑ", "~α~οιτην|~α~οιητην", "~α~οῑεν|~α~οιησᾰν";
    opt_mid_1: "~α~οιμην", _, "~α~οιμεθᾰ";
    opt_mid_2: "~α~οῑο", "~α~οῑσθον", "~α~οῑσθε";
    opt_mid_3: "~α~οῑτο", "~α~οισθην", "~α~οῑντο";
    opt_pass_1: _, _, "~α~οιμεθᾰ";
    opt_pass_2: _, "~α~οῑσθον", "~α~οῑσθε";
    opt_pass_3: "~α~οῑτο", "~α~οισθην", "~α~οῑντο";
    imp_act_2: "~α~ει", "~α~εῑτον", "~α~εῑτε";
    imp_act_3: "~α~ειτω", "~α~ειτων", "~α~ουντων";
    imp_mid_2: "~α~οῡ", _, "~α~εῑσθε";
    imp_mid_3: "~α~εισθω", "~α~εισθων", "~α~εισθων";
    imp_pass_2: _, _, "~α~εῑσθε";
    imp_pass_3: _, "~α~εισθων", _;
    imperfect_act_1: "ἐ~α~ουν", _, "ἐ~α~οῡμεν";
    imperfect_act_2: "ἐ~α~εις", "ἐ~α~εῑτον", "ἐ~α~εῑτε";
    imperfect_act_3: "ἐ~α~ει", "ἐ~α~ειτην", "ἐ~α~ουν";
    imperfect_mid_1: "ἐ~α~ουμην", _, "ἐ~α~ουμεθᾰ";
    imperfect_mid_2: "ἐ~α~οῡ", "ἐ~α~εῑσθον", "ἐ~α~εῑσθε";
    imperfect_mid_3: "ἐ~α~εῑτο", "ἐ~α~εισθην", "ἐ~α~οῡντο";
    imperfect_pass_1: _, _, "ἐ~α~ουμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~α~εῑσθε";
    imperfect_pass_3: "ἐ~α~εῑτο", _, _;
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
    aorist_ind_act_1: "ἐ~α~ησᾰ", _, "ἐ~α~ησᾰμεν";
    aorist_ind_act_2: "ἐ~α~ησᾰς", "ἐ~α~ησᾰτον", "ἐ~α~ησᾰτε";
    aorist_ind_act_3: "ἐ~α~ησε|ἐ~α~ησεν", "ἐ~α~ησᾰτην", "ἐ~α~ησᾰν";
    aorist_ind_mid_1: "ἐ~α~ησᾰμην", _, "ἐ~α~ησᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~α~ησω", "ἐ~α~ησᾰσθον", "ἐ~α~ησᾰσθε";
    aorist_ind_mid_3: "ἐ~α~ησᾰτο", "ἐ~α~ησᾰσθην", "ἐ~α~ησᾰντο";
    aorist_ind_pass_1: "ἐ~α~ηθην", _, "ἐ~α~ηθημεν";
    aorist_ind_pass_2: "ἐ~α~ηθης", "ἐ~α~ηθητον", "ἐ~α~ηθητε";
    aorist_ind_pass_3: "ἐ~α~ηθη", "ἐ~α~ηθητην", "ἐ~α~ηθησᾰν";
    aorist_subj_act_1: "~α~ησω", _, "~α~ησωμεν";
    aorist_subj_act_2: "~α~ησῃς", "~α~ησητον", "~α~ησητε";
    aorist_subj_act_3: "~α~ησῃ", _, "~α~ησωσῐ|~α~ησωσῐν";
    aorist_subj_mid_1: "~α~ησωμαι", _, "~α~ησωμεθᾰ";
    aorist_subj_mid_2: "~α~ησῃ", "~α~ησησθον", "~α~ησησθε";
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
    perfect_ind_pass_1: _, _, "%ε~α~ημεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~α~ησθε";
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
    perfect_imp_pass_2: "%ε~α~ησο", _, _;
    perfect_imp_pass_3: "%ε~α~ησθω", "%ε~α~ησθων", _;
    pluperfect_act_1: "ἐ%ε~α~ηκειν", _, "ἐ%ε~α~ηκεμεν";
    pluperfect_act_2: "ἐ%ε~α~ηκεις", "ἐ%ε~α~ηκετον", "ἐ%ε~α~ηκετε";
    pluperfect_act_3: "ἐ%ε~α~ηκει", "ἐ%ε~α~ηκετην", "ἐ%ε~α~ηκεσᾰν";
    pluperfect_mid_1: "ἐ%ε~α~ημην", _, "ἐ%ε~α~ημεθᾰ";
    pluperfect_mid_2: "ἐ%ε~α~ησο", "ἐ%ε~α~ησθον", "ἐ%ε~α~ησθε";
    pluperfect_mid_3: "ἐ%ε~α~ητο", "ἐ%ε~α~ησθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~α~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~α~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~α~ησθην", "ἐ%ε~α~ηντο";
    infinitive_pres: "~α~εῑν", "~α~εῑσθαι", _;
    infinitive_fut: "~α~ησειν", "~α~ησεσθαι", "~α~ηθησεσθαι";
    infinitive_aor: "~α~ησαι", "~α~ησᾰσθαι", "~α~ηθηναι";
    infinitive_perf: "%ε~α~ηκεναι", "%ε~α~ησθαι", _;
    participle_pres: "~α~ων", "~α~ουμενος", _;
    participle_fut: "~α~ησων", "~α~ησομενος", "~α~ηθησομενος";
    participle_aor: "~α~ησᾱς", "~α~ησᾰμενος", "~α~ηθεις";
    participle_perf: "%ε~α~ηκως", "%ε~α~ημενος", _;
    ),
    // Πρεπω: 8 words (τρεπω, ῥεπω, πρεπω, διεπω, πᾰρεκτρεπω, ἐφεπω). Like Γῡροω, except in 298 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("πρεπω", 8, "~επω", like Γῡροω;
    ind_act_1: "~επω", _, "~επομεν";
    ind_act_2: "~επεις", _, _;
    ind_act_3: "~επει", _, "~επουσῐ";
    ind_mid_1: "~επομαι", _, "~επομεθᾰ";
    ind_mid_2: "~επῃ", "~επεσθον", "~επεσθε";
    ind_mid_3: "~επεται", _, "~επονται";
    ind_pass_1: _, _, "~επομεθᾰ";
    ind_pass_2: _, _, "~επεσθε";
    ind_pass_3: "~επεται", _, "~επονται";
    subj_act_1: "~επω", _, "~επωμεν";
    subj_act_2: "~επῃς", "~επητον", "~επητε";
    subj_act_3: "~επῃ", _, "~επωσῐ";
    subj_mid_1: "~επωμαι", _, "~επωμεθᾰ";
    subj_mid_2: "~επῃ", "~επησθον", "~επησθε";
    subj_mid_3: "~επηται", _, "~επωνται";
    subj_pass_1: _, _, "~επωμεθᾰ";
    subj_pass_2: _, _, "~επησθε";
    subj_pass_3: "~επηται", _, "~επωνται";
    opt_act_1: "~εποιμῐ", _, "~εποιμεν";
    opt_act_2: "~εποις", "~εποιτον", "~εποιτε";
    opt_act_3: "~εποι", "~εποιτην", "~εποιεν";
    opt_mid_1: "~εποιμην", _, "~εποιμεθᾰ";
    opt_mid_2: "~εποιο", "~εποισθον", "~εποισθε";
    opt_mid_3: "~εποιτο", "~εποισθην", "~εποιντο";
    opt_pass_1: _, _, "~εποιμεθᾰ";
    opt_pass_2: _, "~εποισθον", "~εποισθε";
    opt_pass_3: "~εποιτο", "~εποισθην", "~εποιντο";
    imp_act_2: "~επε", "~επετον", "~επετε";
    imp_act_3: "~επετω", "~επετων", "~εποντων";
    imp_mid_2: "~επου", _, "~επεσθε";
    imp_mid_3: "~επεσθω", "~επεσθων", "~επεσθων";
    imp_pass_2: _, _, "~επεσθε";
    imp_pass_3: _, "~επεσθων", _;
    imperfect_act_1: "ἐ~επον", _, "ἐ~επομεν";
    imperfect_act_2: "ἐ~επες", "ἐ~επετον", "ἐ~επετε";
    imperfect_act_3: "ἐ~επε", "ἐ~επετην", "ἐ~επον";
    imperfect_mid_1: "ἐ~επομην", _, "ἐ~επομεθᾰ";
    imperfect_mid_2: "ἐ~επου", "ἐ~επεσθον", "ἐ~επεσθε";
    imperfect_mid_3: "ἐ~επετο", "ἐ~επεσθην", "ἐ~εποντο";
    imperfect_pass_1: _, _, "ἐ~επομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~επεσθε";
    imperfect_pass_3: "ἐ~επετο", _, _;
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
    aorist_ind_act_1: "ἐ~εψᾰ", _, "ἐ~εψᾰμεν";
    aorist_ind_act_2: "ἐ~εψᾰς", "ἐ~εψᾰτον", "ἐ~εψᾰτε";
    aorist_ind_act_3: "ἐ~εψε", "ἐ~εψᾰτην", "ἐ~εψᾰν";
    aorist_ind_mid_1: "ἐ~εψᾰμην", _, "ἐ~εψᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~εψω", "ἐ~εψᾰσθον", "ἐ~εψᾰσθε";
    aorist_ind_mid_3: "ἐ~εψᾰτο", "ἐ~εψᾰσθην", "ἐ~εψᾰντο";
    aorist_ind_pass_1: "ἐ~εφθην", _, "ἐ~εφθημεν";
    aorist_ind_pass_2: "ἐ~εφθης", "ἐ~εφθητον", "ἐ~εφθητε";
    aorist_ind_pass_3: "ἐ~εφθη", "ἐ~εφθητην", "ἐ~εφθησᾰν";
    aorist_subj_act_1: "~εψω", _, "~εψωμεν";
    aorist_subj_act_2: "~εψῃς", "~εψητον", "~εψητε";
    aorist_subj_act_3: "~εψῃ", _, "~εψωσῐ";
    aorist_subj_mid_1: "~εψωμαι", _, "~εψωμεθᾰ";
    aorist_subj_mid_2: "~εψῃ", "~εψησθον", "~εψησθε";
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
    perfect_ind_pass_1: _, _, "%ε~αμμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~αφθε";
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
    perfect_imp_pass_2: "%ε~αψο", _, _;
    perfect_imp_pass_3: "%ε~αφθω", "%ε~αφθων", _;
    pluperfect_act_1: "ἐ%ε~οφειν", _, "ἐ%ε~οφεμεν";
    pluperfect_act_2: "ἐ%ε~οφεις", "ἐ%ε~οφετον", "ἐ%ε~οφετε";
    pluperfect_act_3: "ἐ%ε~οφει", "ἐ%ε~οφετην", "ἐ%ε~οφεσᾰν";
    pluperfect_mid_1: "ἐ%ε~αμμην", _, "ἐ%ε~αμμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~αψο", "ἐ%ε~αφθον", "ἐ%ε~αφθε";
    pluperfect_mid_3: "ἐ%ε~απτο", "ἐ%ε~αφθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~αμμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~αφθε";
    pluperfect_pass_3: _, "ἐ%ε~αφθην", "ἐ%ε~αφᾰτο";
    infinitive_pres: "~επειν", "~επεσθαι", _;
    infinitive_fut: "~εψειν", "~εψεσθαι", "~απησεσθαι";
    infinitive_aor: "~εψαι", "~εψᾰσθαι", "~εφθηναι";
    infinitive_perf: "%ε~οφεναι", "%ε~ᾱφθαι", _;
    participle_pres: "~επων", "~επομενος", _;
    participle_fut: "~εψων", "~εψομενος", "~απησομενος";
    participle_aor: "~εψᾱς", "~εψᾰμενος", "~εφθεις";
    participle_perf: "%ε~οφως", "%ε~αμμενος", _;
    ),
    // Σφᾰλλω: 8 words (βᾰλλω, θᾰλλω, πᾰλλω, βδᾰλλω, σφᾰλλω, ψᾰλλω). Like Προσερχομαι, except in 299 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("σφᾰλλω", 8, "~ᾰ~λω", like Προσερχομαι;
    ind_act_1: "~ᾰ~λω", _, "~ᾰ~λομεν";
    ind_act_2: "~ᾰ~λεις", _, _;
    ind_act_3: "~ᾰ~λει", _, "~ᾰ~λουσῐ|~ᾰ~λουσῐν";
    ind_mid_1: "~ᾰ~λομαι", _, "~ᾰ~λομεθᾰ";
    ind_mid_2: "~ᾰ~λῃ|~ᾰ~λει", "~ᾰ~λεσθον", "~ᾰ~λεσθε";
    ind_mid_3: "~ᾰ~λεται", _, "~ᾰ~λονται";
    ind_pass_1: _, _, "~ᾰ~λομεθᾰ";
    ind_pass_2: "~ᾰ~λῃ|~ᾰ~λει", _, "~ᾰ~λεσθε";
    ind_pass_3: "~ᾰ~λεται", _, "~ᾰ~λονται";
    subj_act_1: "~ᾰ~λω", _, "~ᾰ~λωμεν";
    subj_act_2: "~ᾰ~λῃς", "~ᾰ~λητον", "~ᾰ~λητε";
    subj_act_3: "~ᾰ~λῃ", _, "~ᾰ~λωσῐ|~ᾰ~λωσῐν";
    subj_mid_1: "~ᾰ~λωμαι", _, "~ᾰ~λωμεθᾰ";
    subj_mid_2: "~ᾰ~λῃ", "~ᾰ~λησθον", "~ᾰ~λησθε";
    subj_mid_3: "~ᾰ~ληται", _, "~ᾰ~λωνται";
    subj_pass_1: _, _, "~ᾰ~λωμεθᾰ";
    subj_pass_2: _, _, "~ᾰ~λησθε";
    subj_pass_3: "~ᾰ~ληται", _, "~ᾰ~λωνται";
    opt_act_1: "~ᾰ~λοιμῐ", _, "~ᾰ~λοιμεν";
    opt_act_2: "~ᾰ~λοις", "~ᾰ~λοιτον", "~ᾰ~λοιτε";
    opt_act_3: "~ᾰ~λοι", "~ᾰ~λοιτην", "~ᾰ~λοιεν";
    opt_mid_1: "~ᾰ~λοιμην", _, "~ᾰ~λοιμεθᾰ";
    opt_mid_2: "~ᾰ~λοιο", "~ᾰ~λοισθον", "~ᾰ~λοισθε";
    opt_mid_3: "~ᾰ~λοιτο", "~ᾰ~λοισθην", "~ᾰ~λοιντο";
    opt_pass_1: _, _, "~ᾰ~λοιμεθᾰ";
    opt_pass_2: _, "~ᾰ~λοισθον", "~ᾰ~λοισθε";
    opt_pass_3: "~ᾰ~λοιτο", "~ᾰ~λοισθην", "~ᾰ~λοιντο";
    imp_act_2: "~ᾰ~λε", "~ᾰ~λετον", "~ᾰ~λετε";
    imp_act_3: "~ᾰ~λετω", "~ᾰ~λετων", "~ᾰ~λοντων";
    imp_mid_2: "~ᾰ~λου", _, "~ᾰ~λεσθε";
    imp_mid_3: "~ᾰ~λεσθω", "~ᾰ~λεσθων", "~ᾰ~λεσθων";
    imp_pass_2: _, _, "~ᾰ~λεσθε";
    imp_pass_3: _, "~ᾰ~λεσθων", _;
    imperfect_act_1: "ἐ~ᾰ~λον", _, "ἐ~ᾰ~λομεν";
    imperfect_act_2: "ἐ~ᾰ~λες", "ἐ~ᾰ~λετον", "ἐ~ᾰ~λετε";
    imperfect_act_3: "ἐ~ᾰ~λε|ἐ~ᾰ~λεν", "ἐ~ᾰ~λετην", "ἐ~ᾰ~λον";
    imperfect_mid_1: "ἐ~ᾰ~λομην", _, "ἐ~ᾰ~λομεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~λου", "ἐ~ᾰ~λεσθον", "ἐ~ᾰ~λεσθε";
    imperfect_mid_3: "ἐ~ᾰ~λετο", "ἐ~ᾰ~λεσθην", "ἐ~ᾰ~λοντο";
    imperfect_pass_1: _, _, "ἐ~ᾰ~λομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ᾰ~λεσθε";
    imperfect_pass_3: "ἐ~ᾰ~λετο", _, _;
    future_ind_act_1: "~ᾰ~ω", _, "~ᾰ~οῡμεν";
    future_ind_act_2: "~ᾰ~εῑς", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    future_ind_act_3: "~ᾰ~εῑ", _, "~ᾰ~οῡσῐ";
    future_ind_mid_1: "~ᾰ~οῡμαι", _, "~ᾰ~ουμεθᾰ";
    future_ind_mid_2: "~ᾰ~εῑ", "~ᾰ~εῑσθον", "~ᾰ~εῑσθε";
    future_ind_mid_3: "~ᾰ~εῑται", _, "~ᾰ~οῡνται";
    future_ind_pass_1: "~ᾰ~ησησομαι", _, "~ᾰ~ησησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ησησῃ", "~ᾰ~ησησεσθον", "~ᾰ~ησησεσθε";
    future_ind_pass_3: "~ᾰ~ησησεται", _, "~ᾰ~ησησονται";
    future_opt_act_1: "~ᾰ~οιην", _, "~ᾰ~οῑμεν";
    future_opt_act_2: "~ᾰ~οιης", "~ᾰ~οῑτον", "~ᾰ~οῑτε";
    future_opt_act_3: "~ᾰ~οιη", "~ᾰ~οιτην", "~ᾰ~οῑεν";
    future_opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    future_opt_mid_2: "~ᾰ~οῑο", "~ᾰ~οῑσθον", "~ᾰ~οῑσθε";
    future_opt_mid_3: "~ᾰ~οῑτο", "~ᾰ~οισθην", "~ᾰ~οῑντο";
    future_opt_pass_1: "~ᾰ~ησησοιμην", _, "~ᾰ~ησησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ησησοιο", "~ᾰ~ησησοισθον", "~ᾰ~ησησοισθε";
    future_opt_pass_3: "~ᾰ~ησησοιτο", "~ᾰ~ησησοισθην", "~ᾰ~ησησοιντο";
    aorist_ind_act_1: "ἐ~η~ᾰ", _, "ἐ~η~ᾰμεν";
    aorist_ind_act_2: "ἐ~η~ᾰς", "ἐ~η~ᾰτον", "ἐ~η~ᾰτε";
    aorist_ind_act_3: "ἐ~η~ε", "ἐ~η~ᾰτην", "ἐ~η~ᾰν";
    aorist_ind_mid_1: "ἐ~η~ᾰμην", _, "ἐ~η~ᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~η~ω", "ἐ~η~ᾰσθον", "ἐ~η~ᾰσθε";
    aorist_ind_mid_3: "ἐ~η~ᾰτο", "ἐ~η~ᾰσθην", "ἐ~η~ᾰντο";
    aorist_ind_pass_1: "ἐ~ᾰ~ην", _, "ἐ~ᾰ~ημεν";
    aorist_ind_pass_2: "ἐ~ᾰ~ης", "ἐ~ᾰ~ητον", "ἐ~ᾰ~ητε";
    aorist_ind_pass_3: "ἐ~ᾰ~η", "ἐ~ᾰ~ητην", "ἐ~ᾰ~ησᾰν";
    aorist_subj_act_1: "~η~ω", _, "~η~ωμεν";
    aorist_subj_act_2: "~η~ῃς", "~η~ητον", "~η~ητε";
    aorist_subj_act_3: "~η~ῃ", _, "~η~ωσῐ|~η~ωσῐν";
    aorist_subj_mid_1: "~η~ωμαι", _, "~η~ωμεθᾰ";
    aorist_subj_mid_2: "~η~ῃ", "~η~ησθον", "~η~ησθε";
    aorist_subj_mid_3: "~η~ηται", _, "~η~ωνται";
    aorist_subj_pass_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    aorist_subj_pass_2: "~ᾰ~ῃς", "~ᾰ~ητον", "~ᾰ~ητε";
    aorist_subj_pass_3: "~ᾰ~ῃ", _, "~ᾰ~ωσῐ";
    aorist_opt_act_1: "~η~αιμῐ", _, "~η~αιμεν";
    aorist_opt_act_2: "~η~ειᾰς|~η~αις", "~η~αιτον", "~η~αιτε";
    aorist_opt_act_3: "~η~ειε|~η~ειεν|~η~αι", "~η~αιτην", "~η~ειᾰν|~η~αιεν";
    aorist_opt_mid_1: "~η~αιμην", _, "~η~αιμεθᾰ";
    aorist_opt_mid_2: "~η~αιο", "~η~αισθον", "~η~αισθε";
    aorist_opt_mid_3: "~η~αιτο", "~η~αισθην", "~η~αιντο";
    aorist_opt_pass_1: "~ᾰ~ειην", _, "~ᾰ~εῑμεν";
    aorist_opt_pass_2: "~ᾰ~ειης", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    aorist_opt_pass_3: "~ᾰ~ειη", "~ᾰ~ειτην", "~ᾰ~εῑεν";
    aorist_imp_act_2: "~η~ον", "~η~ᾰτον", "~η~ᾰτε";
    aorist_imp_act_3: "~η~ᾰτω", "~η~ᾰτων", "~η~ᾰντων";
    aorist_imp_mid_2: "~η~αι", "~η~ᾰσθον", "~η~ᾰσθε";
    aorist_imp_mid_3: "~η~ᾰσθω", "~η~ᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~ηθῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ητω", "~ᾰ~ητων", "~ᾰ~εντων";
    perfect_ind_act_1: "ἐ~ᾰ~κᾰ", _, "ἐ~ᾰ~κᾰμεν";
    perfect_ind_act_2: "ἐ~ᾰ~κᾰς", "ἐ~ᾰ~κᾰτον", "ἐ~ᾰ~κᾰτε";
    perfect_ind_act_3: "ἐ~ᾰ~κε", _, "ἐ~ᾰ~κᾱσῐ";
    perfect_ind_mid_1: "ἐ~ᾰ~μαι", _, "ἐ~ᾰ~μεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰ~σαι", "ἐ~ᾰ~θον", "ἐ~ᾰ~θε";
    perfect_ind_mid_3: "ἐ~ᾰ~ται", _, "ἐ~ᾰ~ᾰται";
    perfect_ind_pass_1: _, _, "ἐ~ᾰ~μεθᾰ";
    perfect_ind_pass_2: _, _, "ἐ~ᾰ~θε";
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
    perfect_imp_pass_2: "ἐ~ᾰ~σο", _, _;
    perfect_imp_pass_3: "ἐ~ᾰ~θω", "ἐ~ᾰ~θων", _;
    pluperfect_act_1: "ἐ~ᾰ~κειν", _, "ἐ~ᾰ~κεμεν";
    pluperfect_act_2: "ἐ~ᾰ~κεις", "ἐ~ᾰ~κετον", "ἐ~ᾰ~κετε";
    pluperfect_act_3: "ἐ~ᾰ~κει", "ἐ~ᾰ~κετην", "ἐ~ᾰ~κεσᾰν";
    pluperfect_mid_1: "ἐ~ᾰ~μην", _, "ἐ~ᾰ~μεθᾰ";
    pluperfect_mid_2: "ἐ~ᾰ~σο", "ἐ~ᾰ~θον", "ἐ~ᾰ~θε";
    pluperfect_mid_3: "ἐ~ᾰ~το", "ἐ~ᾰ~θην", _;
    pluperfect_pass_1: _, _, "ἐ~ᾰ~μεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ᾰ~θε";
    pluperfect_pass_3: _, "ἐ~ᾰ~θην", "ἐ~ᾰ~ᾰτο";
    infinitive_pres: "~ᾰ~λειν", "~ᾰ~λεσθαι", _;
    infinitive_fut: "~ᾰ~εῑν", "~ᾰ~εῑσθαι", "~ᾰ~ησησεσθαι";
    infinitive_aor: "~η~αι", "~η~ᾰσθαι", "~ᾰ~ηναι";
    infinitive_perf: "ἐ~ᾰ~κεναι", "ἐ~ᾰ~θαι", _;
    participle_pres: "~ᾰ~λων", "~ᾰ~λομενος", _;
    participle_fut: "~ᾰ~ων", "~ᾰ~ουμενος", "~ᾰ~ησησομενος";
    participle_aor: "~η~ᾱς", "~η~ᾰμενος", "~ᾰ~εις";
    participle_perf: "ἐ~ᾰ~κως", "ἐ~ᾰ~μενος", _;
    ),
    // Πᾰρᾰκᾰλεω: 8 words (πᾰρᾰκᾰλεω, στερεω, σῐδηροφορεω, κορεω, σῠγκᾰλεω, κᾰτᾰκᾰλεω). Like Δῐκεω, except in 91 cells: ind_mid_2 sg, ind_pass_2 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, ….
    conjugation!("πᾰρᾰκᾰλεω", 8, "~εω", like Δῐκεω;
    ind_mid_2: "~εῑ", _, _;
    ind_pass_2: derived, _, _;
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
    aorist_ind_act_3: "ἐ~ησε", _, _;
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
    pluperfect_act_1: "ἐ~ηκειν", _, _;
    pluperfect_act_2: "ἐ~ηκεις", _, _;
    pluperfect_act_3: "ἐ~ηκει", _, _;
    future_perfect_ind_mid_1: "ἐ~ησομαι", _, "ἐ~ησομεθᾰ";
    future_perfect_ind_mid_2: "ἐ~ησῃ", "ἐ~ησεσθον", "ἐ~ησεσθε";
    future_perfect_ind_mid_3: "ἐ~ησεται", _, "ἐ~ησονται";
    future_perfect_opt_mid_1: "ἐ~ησοιμην", _, "ἐ~ησοιμεθᾰ";
    future_perfect_opt_mid_2: "ἐ~ησοιο", "ἐ~ησοισθον", "ἐ~ησοισθε";
    future_perfect_opt_mid_3: "ἐ~ησοιτο", "ἐ~ησοισθην", "ἐ~ησοιντο";
    infinitive_fut: "~εῑν", "~εῑσθαι", "~ησεσθαι";
    infinitive_futp: _, "ἐ~ησεσθαι", _;
    participle_fut: "~ων", "~ουμενος", "~ησομενος";
    participle_futp: _, "ἐ~ησομενος", _;
    ),
    // Φᾰρμᾰσσω: 8 words (τᾰρᾰσσω, θωρησσω, κνωσσω, θρῡλῐσσω, παιφᾰσσω, φᾰρμᾰσσω). Like Τᾰσσω, except in 74 cells: ind_act_3 pl, ind_mid_2 sg, ind_pass_2 sg, subj_act_3 pl, imperfect_act_3 sg, future_ind_act_3 pl, ….
    conjugation!("φᾰρμᾰσσω", 8, "~σσω", like Τᾰσσω;
    ind_act_3: _, _, "~σσουσῐ";
    ind_mid_2: "~σσῃ", _, _;
    ind_pass_2: derived, _, _;
    subj_act_3: _, _, "~σσωσῐ";
    imperfect_act_3: "ἐ~σσε", _, _;
    future_ind_act_3: _, _, "~ξουσῐ";
    future_ind_mid_2: "~ξῃ", _, _;
    aorist_ind_act_1: "ἐ~ξον", _, "ἐ~ξομεν";
    aorist_ind_act_2: "ἐ~ξες", "ἐ~ξετον", "ἐ~ξετε";
    aorist_ind_act_3: "ἐ~ξε", "ἐ~ξετην", "ἐ~ξον";
    aorist_ind_mid_1: "ἐ~ξομην", _, "ἐ~ξομεθᾰ";
    aorist_ind_mid_2: "ἐ~ξου", "ἐ~ξεσθον", "ἐ~ξεσθε";
    aorist_ind_mid_3: "ἐ~ξετο", "ἐ~ξεσθην", "ἐ~ξοντο";
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
    pluperfect_pass_3: _, _, "ἐ%ε~χᾰτο";
    future_perfect_ind_mid_1: "%ε~ξομαι", _, "%ε~ξομεθᾰ";
    future_perfect_ind_mid_2: "%ε~ξῃ", "%ε~ξεσθον", "%ε~ξεσθε";
    future_perfect_ind_mid_3: "%ε~ξεται", _, "%ε~ξονται";
    future_perfect_opt_mid_1: "%ε~ξοιμην", _, "%ε~ξοιμεθᾰ";
    future_perfect_opt_mid_2: "%ε~ξοιο", "%ε~ξοισθον", "%ε~ξοισθε";
    future_perfect_opt_mid_3: "%ε~ξοιτο", "%ε~ξοισθην", "%ε~ξοιντο";
    infinitive_aor: "~ξεῑν", "~ξεσθαι", _;
    infinitive_futp: _, "%ε~ξεσθαι", _;
    participle_aor: "~ξων", "~ξομενος", _;
    participle_futp: _, "%ε~ξομενος", _;
    ),
    // Σῳζω: 7 words (σοφῐζω, σῳζω, κεᾰζω, στρουθῐζω, τῠκῐζω, μερῐζω). Like Ἀφορῐζω, except in 7 cells: imperfect_act_3 sg, aorist_ind_act_3 sg, perfect_ind_mid_3 pl, pluperfect_act_1 sg, pluperfect_act_2 sg, pluperfect_act_3 sg, ….
    conjugation!("σῳζω", 7, "~ζω", like Ἀφορῐζω;
    imperfect_act_3: "ἐ~ζε|ἐ~ζεν", _, _;
    aorist_ind_act_3: "ἐ~σε|ἐ~σεν", _, _;
    perfect_ind_mid_3: _, _, "%ε~σᾰται";
    pluperfect_act_1: "ἐ%ε~κειν|ἐ%ε~κη", _, _;
    pluperfect_act_2: "ἐ%ε~κεις|ἐ%ε~κης", _, _;
    pluperfect_act_3: "ἐ%ε~κει|ἐ%ε~κειν", _, _;
    pluperfect_pass_3: _, _, "ἐ%ε~σᾰτο";
    ),
    // Ἀπεῑπον: 7 words (προσεῑπον, ἐξεῑπον, προεῑπον, ἀπεῑπον, ὑπεῑπον, ἐπεῑπον). Like Πᾰρειμι, except in 223 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, subj_act_1 sg, ….
    conjugation!("ἀπεῑπον", 7, "~εῑπον", like Πᾰρειμι;
    ind_act_1: "~εῑπον", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, _, derived;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imp_act_3: derived, derived, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
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
    aorist_ind_act_1: "~εῑπον", _, "~ειπομεν";
    aorist_ind_act_2: "~εῑπες", "~ειπετον", "~ειπετε";
    aorist_ind_act_3: "~εῑπε", "~ειπετην", "~εῑπον";
    aorist_ind_mid_1: "~ειπομην", _, "~ειπομεθᾰ";
    aorist_ind_mid_2: "~ειπου", "~ειπεσθον", "~ειπεσθε";
    aorist_ind_mid_3: "~ειπετο", "~ειπεσθην", "~ειποντο";
    aorist_ind_pass_1: "~ερρηθην", _, "~ερρηθημεν";
    aorist_ind_pass_2: "~ερρηθης", "~ερρηθητον", "~ερρηθητε";
    aorist_ind_pass_3: "~ερρηθη", "~ερρηθητην", "~ερρηθησᾰν";
    aorist_subj_act_1: "~ειπω", _, "~ειπωμεν";
    aorist_subj_act_2: "~ειπῃς", "~ειπητον", "~ειπητε";
    aorist_subj_act_3: "~ειπῃ", _, "~ειπωσῐ";
    aorist_subj_mid_1: "~ειπωμαι", _, "~ειπωμεθᾰ";
    aorist_subj_mid_2: "~ειπῃ", "~ειπησθον", "~ειπησθε";
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
    perfect_ind_pass_1: _, _, "~ειρημεθᾰ";
    perfect_ind_pass_2: _, _, "~ειρησθε";
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
    perfect_imp_pass_2: "~ειρησο", _, _;
    perfect_imp_pass_3: "~ειρησθω", "~ειρησθων", _;
    infinitive_pres: derived, _, _;
    infinitive_fut: "~ερεῑν", "~ερεῑσθαι", "~ορρηθησεσθαι";
    infinitive_aor: "~ειπεῑν", "~ειπεσθαι", "~ορρηθηναι";
    infinitive_perf: "~ειρηκεναι", "~ειρησθαι", _;
    participle_pres: derived, _, _;
    participle_fut: "~ερων", "~ερουμενος", "~ορρηθησομενος";
    participle_aor: "~ειπων", "~ειπομενος", "~ορρηθεις";
    participle_perf: "~ειρηκως", "~ειρημενος", _;
    ),
    // Φειδομαι: 6 words (εἰδομαι, ἐλδομαι, μηδομαι, λῐνδομαι, φειδομαι, αἰδομαι). Like Προσερχομαι, except in 167 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("φειδομαι", 6, "~~δομαι", like Προσερχομαι;
    ind_act_1: "~~δομαι", _, _;
    ind_mid_1: "~~δομαι", _, "~~δομεθᾰ";
    ind_mid_2: "~~δῃ", "~~δεσθον", "~~δεσθε";
    ind_mid_3: "~~δεται", _, "~~δονται";
    ind_pass_1: _, _, "~~δομεθᾰ";
    ind_pass_2: _, _, "~~δεσθε";
    ind_pass_3: "~~δεται", _, "~~δονται";
    subj_mid_1: "~~δωμαι", _, "~~δωμεθᾰ";
    subj_mid_2: "~~δῃ", "~~δησθον", "~~δησθε";
    subj_mid_3: "~~δηται", _, "~~δωνται";
    subj_pass_1: _, _, "~~δωμεθᾰ";
    subj_pass_2: _, _, "~~δησθε";
    subj_pass_3: "~~δηται", _, "~~δωνται";
    opt_mid_1: "~~δοιμην", _, "~~δοιμεθᾰ";
    opt_mid_2: "~~δοιο", "~~δοισθον", "~~δοισθε";
    opt_mid_3: "~~δοιτο", "~~δοισθην", "~~δοιντο";
    opt_pass_1: _, _, "~~δοιμεθᾰ";
    opt_pass_2: _, "~~δοισθον", "~~δοισθε";
    opt_pass_3: "~~δοιτο", "~~δοισθην", "~~δοιντο";
    imp_mid_2: "~~δου", _, "~~δεσθε";
    imp_mid_3: "~~δεσθω", "~~δεσθων", "~~δεσθων";
    imp_pass_2: _, _, "~~δεσθε";
    imp_pass_3: _, "~~δεσθων", _;
    imperfect_mid_1: "~~δομην", _, "~~δομεθᾰ";
    imperfect_mid_2: "~~δου", "~~δεσθον", "~~δεσθε";
    imperfect_mid_3: "~~δετο", "~~δεσθην", "~~δοντο";
    imperfect_pass_1: _, _, "~~δομεθᾰ";
    imperfect_pass_2: _, _, "~~δεσθε";
    imperfect_pass_3: "~~δετο", _, _;
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
    aorist_ind_mid_1: "ἐ~~σᾰμην", _, "ἐ~~σᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~~σω", "ἐ~~σᾰσθον", "ἐ~~σᾰσθε";
    aorist_ind_mid_3: "ἐ~~σᾰτο", "ἐ~~σᾰσθην", "ἐ~~σᾰντο";
    aorist_ind_pass_1: derived, _, derived;
    aorist_ind_pass_2: derived, derived, derived;
    aorist_ind_pass_3: derived, derived, derived;
    aorist_subj_mid_1: "~~σωμαι", _, "~~σωμεθᾰ";
    aorist_subj_mid_2: "~~σῃ", "~~σησθον", "~~σησθε";
    aorist_subj_mid_3: "~~σηται", _, "~~σωνται";
    aorist_subj_pass_1: derived, _, derived;
    aorist_subj_pass_2: derived, derived, derived;
    aorist_subj_pass_3: derived, _, derived;
    aorist_opt_mid_1: "~~σαιμην", _, "~~σαιμεθᾰ";
    aorist_opt_mid_2: "~~σαιο", "~~σαισθον", "~~σαισθε";
    aorist_opt_mid_3: "~~σαιτο", "~~σαισθην", "~~σαιντο";
    aorist_opt_pass_1: derived, _, derived;
    aorist_opt_pass_2: derived, derived, derived;
    aorist_opt_pass_3: derived, derived, derived;
    aorist_imp_mid_2: "~ῑσα~", "~~σᾰσθον", "~~σᾰσθε";
    aorist_imp_mid_3: "~~σᾰσθω", "~~σᾰσθων", _;
    aorist_imp_pass_2: derived, _, _;
    aorist_imp_pass_3: derived, derived, derived;
    perfect_ind_mid_1: "%ε~~σμαι", _, "%ε~~σμεθᾰ";
    perfect_ind_mid_2: "%ε~~σαι", "%ε~~σθον", "%ε~~σθε";
    perfect_ind_mid_3: "%ε~~σται", _, "%ε~~δᾰται";
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_imp_mid_2: "%ε~~σο", _, _;
    perfect_imp_mid_3: "%ε~~σθω", "%ε~~σθων", _;
    perfect_imp_pass_2: derived, _, _;
    perfect_imp_pass_3: derived, derived, _;
    infinitive_pres: _, "~~δεσθαι", _;
    infinitive_fut: _, "~~σεσθαι", "~~σθησεσθαι";
    infinitive_aor: _, "~~σᾰσθαι", derived;
    infinitive_perf: _, derived, _;
    participle_pres: _, "~~δομενος", _;
    participle_fut: _, "~~σομενος", "~~σθησομενος";
    participle_aor: _, "~~σᾰμενος", derived;
    participle_perf: _, derived, _;
    ),
    // Ἀραομαι: 6 words (μναομαι, ἀραομαι, μᾰσαομαι, ἀλαομαι, μηχᾰναομαι, τεχναομαι). Like Οἰομαι, except in 128 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἀραομαι", 6, "~αομαι", like Οἰομαι;
    ind_act_1: "~αομαι", _, _;
    ind_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    ind_mid_2: "~ᾳ", "~ᾱσθον", "~ᾱσθε";
    ind_mid_3: "~ᾱται", _, "~ωνται";
    ind_pass_1: _, _, "~ωμεθᾰ";
    ind_pass_2: _, _, "~ᾱσθε";
    ind_pass_3: "~ᾱται", _, "~ωνται";
    subj_mid_2: "~ᾳ", "~ᾱσθον", "~ᾱσθε";
    subj_mid_3: "~ᾱται", _, _;
    subj_pass_2: _, _, "~ᾱσθε";
    subj_pass_3: "~ᾱται", _, _;
    opt_mid_1: "~ῳμην", _, "~ῳμεθᾰ";
    opt_mid_2: "~ῳο", "~ῳσθον", "~ῳσθε";
    opt_mid_3: "~ῳτο", "~ῳσθην", "~ῳντο";
    opt_pass_1: _, _, "~ῳμεθᾰ";
    opt_pass_2: _, "~ῳσθον", "~ῳσθε";
    opt_pass_3: "~ῳτο", "~ῳσθην", "~ῳντο";
    imp_mid_2: "~ω", _, "~ᾱσθε";
    imp_mid_3: "~ᾱσθω", "~ᾱσθων", "~ᾱσθων";
    imp_pass_2: _, _, "~ᾱσθε";
    imp_pass_3: _, "~ᾱσθων", _;
    imperfect_mid_1: "ἐ~ωμην", _, "ἐ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ω", "ἐ~ᾱσθον", "ἐ~ᾱσθε";
    imperfect_mid_3: "ἐ~ᾱτο", "ἐ~ᾱσθην", "ἐ~ωντο";
    imperfect_pass_1: _, _, "ἐ~ωμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ᾱσθε";
    imperfect_pass_3: "ἐ~ᾱτο", _, _;
    future_ind_pass_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_pass_2: "~ησῃ", "~ησεσθον", "~ησεσθε";
    future_ind_pass_3: "~ησεται", _, "~ησονται";
    future_opt_pass_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_pass_2: "~ησοιο", "~ησοισθον", "~ησοισθε";
    future_opt_pass_3: "~ησοιτο", "~ησοισθην", "~ησοιντο";
    aorist_ind_mid_1: "ἐ~ησᾰμην", _, "ἐ~ησᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ησω", "ἐ~ησᾰσθον", "ἐ~ησᾰσθε";
    aorist_ind_mid_3: "ἐ~ησᾰτο", "ἐ~ησᾰσθην", "ἐ~ησᾰντο";
    aorist_ind_pass_1: derived, _, derived;
    aorist_ind_pass_2: derived, derived, derived;
    aorist_ind_pass_3: derived, derived, derived;
    aorist_subj_mid_1: "~ησωμαι", _, "~ησωμεθᾰ";
    aorist_subj_mid_2: "~ησῃ", "~ησησθον", "~ησησθε";
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
    aorist_imp_mid_3: "~ησᾰσθω", "~ησᾰσθων", _;
    aorist_imp_pass_2: derived, _, _;
    aorist_imp_pass_3: derived, derived, derived;
    infinitive_pres: _, "~ᾱσθαι", _;
    infinitive_fut: _, _, "~ησεσθαι";
    infinitive_aor: _, "~ησᾰσθαι", derived;
    participle_pres: _, "~ωμενος", _;
    participle_fut: _, _, "~ησομενος";
    participle_aor: _, "~ησᾰμενος", derived;
    ),
    // Δῐωκω: 5 words (ἡκω, πᾰρηκω, προσηκω, τεθνηκω, κᾰθηκω). Like Ληγω, except in 208 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("δῐωκω", 5, "~κω", like Ληγω;
    ind_act_1: "~κω", _, "~κομεν";
    ind_act_2: "~κεις", _, _;
    ind_act_3: "~κει", _, "~κουσῐ";
    ind_mid_1: "~κομαι", _, "~κομεθᾰ";
    ind_mid_2: "~κῃ", "~κεσθον", "~κεσθε";
    ind_mid_3: "~κεται", _, "~κονται";
    ind_pass_1: _, _, "~κομεθᾰ";
    ind_pass_2: _, _, "~κεσθε";
    ind_pass_3: "~κεται", _, "~κονται";
    subj_act_1: "~κω", _, "~κωμεν";
    subj_act_2: "~κῃς", "~κητον", "~κητε";
    subj_act_3: "~κῃ", _, "~κωσῐ";
    subj_mid_1: "~κωμαι", _, "~κωμεθᾰ";
    subj_mid_2: "~κῃ", "~κησθον", "~κησθε";
    subj_mid_3: "~κηται", _, "~κωνται";
    subj_pass_1: _, _, "~κωμεθᾰ";
    subj_pass_2: _, _, "~κησθε";
    subj_pass_3: "~κηται", _, "~κωνται";
    opt_act_1: "~κοιμῐ", _, "~κοιμεν";
    opt_act_2: "~κοις", "~κοιτον", "~κοιτε";
    opt_act_3: "~κοι", "~κοιτην", "~κοιεν";
    opt_mid_1: "~κοιμην", _, "~κοιμεθᾰ";
    opt_mid_2: "~κοιο", "~κοισθον", "~κοισθε";
    opt_mid_3: "~κοιτο", "~κοισθην", "~κοιντο";
    opt_pass_1: _, _, "~κοιμεθᾰ";
    opt_pass_2: _, "~κοισθον", "~κοισθε";
    opt_pass_3: "~κοιτο", "~κοισθην", "~κοιντο";
    imp_act_2: "~κε", "~κετον", "~κετε";
    imp_act_3: "~κετω", "~κετων", "~κοντων";
    imp_mid_2: "~κου", _, "~κεσθε";
    imp_mid_3: "~κεσθω", "~κεσθων", "~κεσθων";
    imp_pass_2: _, _, "~κεσθε";
    imp_pass_3: _, "~κεσθων", _;
    imperfect_act_1: "~κον", _, "~κομεν";
    imperfect_act_2: "~κες", "~κετον", "~κετε";
    imperfect_act_3: "~κε", "~κετην", "~κον";
    imperfect_mid_1: "~κομην", _, "~κομεθᾰ";
    imperfect_mid_2: "~κου", "~κεσθον", "~κεσθε";
    imperfect_mid_3: "~κετο", "~κεσθην", "~κοντο";
    imperfect_pass_1: _, _, "~κομεθᾰ";
    imperfect_pass_2: _, _, "~κεσθε";
    imperfect_pass_3: "~κετο", _, _;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_act_1: "~ξᾰ", _, "~ξᾰμεν";
    aorist_ind_act_2: "~ξᾰς", "~ξᾰτον", "~ξᾰτε";
    aorist_ind_act_3: "~ξε", "~ξᾰτην", "~ξᾰν";
    aorist_ind_mid_1: "~ξᾰμην", _, "~ξᾰμεθᾰ";
    aorist_ind_mid_2: "~ξω", "~ξᾰσθον", "~ξᾰσθε";
    aorist_ind_mid_3: "~ξᾰτο", "~ξᾰσθην", "~ξᾰντο";
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
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: "~κᾰς", "~κᾰτον", "~κᾰτε";
    perfect_ind_act_3: "~κε", _, "~κᾱσῐ";
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, derived;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "~κω", _, "~κωμεν";
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ";
    perfect_opt_act_1: "~κοιμῐ", _, "~κοιμεν";
    perfect_opt_act_2: "~κοις", "~κοιτον", "~κοιτε";
    perfect_opt_act_3: "~κοι", "~κοιτην", "~κοιεν";
    perfect_imp_act_2: _, "~κετον", "~κετε";
    perfect_imp_act_3: "~κετω", "~κετων", "~κοντων";
    perfect_imp_mid_2: derived, _, _;
    perfect_imp_mid_3: derived, derived, _;
    perfect_imp_pass_2: derived, _, _;
    perfect_imp_pass_3: derived, derived, _;
    pluperfect_act_1: "~κειν", _, "~κεμεν";
    pluperfect_act_2: "~κεις", "~κετον", "~κετε";
    pluperfect_act_3: "~κει", "~κετην", "~κεσᾰν";
    infinitive_pres: "~κειν", "~κεσθαι", _;
    infinitive_fut: _, _, derived;
    infinitive_aor: _, _, derived;
    infinitive_perf: "~κεναι", derived, _;
    participle_pres: "~κων", "~κομενος", _;
    participle_fut: _, _, derived;
    participle_aor: _, _, derived;
    participle_perf: "~κως", derived, _;
    ),
    // Χραω: 5 words (χραω, σκῐαω, ἀπαγχοναω, πασχητῐαω, λουτῐαω). Like Δοκᾰω, except in 65 cells: future_ind_act_3 pl, future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, future_ind_pass_2 du, future_ind_pass_2 pl, ….
    conjugation!("χραω", 5, "~αω", like Δοκᾰω;
    future_ind_act_3: _, _, "~ησουσῐ";
    future_ind_pass_1: "~ησθησομαι", _, "~ησθησομεθᾰ";
    future_ind_pass_2: "~ησθησῃ", "~ησθησεσθον", "~ησθησεσθε";
    future_ind_pass_3: "~ησθησεται", _, "~ησθησονται";
    future_opt_pass_1: "~ησθησοιμην", _, "~ησθησοιμεθᾰ";
    future_opt_pass_2: "~ησθησοιο", "~ησθησοισθον", "~ησθησοισθε";
    future_opt_pass_3: "~ησθησοιτο", "~ησθησοισθην", "~ησθησοιντο";
    aorist_ind_act_3: "ἐ~ησε|ἐ~ησεν", _, _;
    aorist_ind_pass_1: "ἐ~ησθην", _, "ἐ~ησθημεν";
    aorist_ind_pass_2: "ἐ~ησθης", "ἐ~ησθητον", "ἐ~ησθητε";
    aorist_ind_pass_3: "ἐ~ησθη", "ἐ~ησθητην", "ἐ~ησθησᾰν";
    aorist_subj_pass_1: "~ησθω", _, "~ησθωμεν";
    aorist_subj_pass_2: "~ησθῃς", "~ησθητον", "~ησθητε";
    aorist_subj_pass_3: "~ησθῃ", _, "~ησθωσῐ";
    aorist_opt_pass_1: "~ησθειην", _, "~ησθεῑμεν";
    aorist_opt_pass_2: "~ησθειης", "~ησθεῑτον", "~ησθεῑτε";
    aorist_opt_pass_3: "~ησθειη", "~ησθειτην", "~ησθεῑεν";
    aorist_imp_pass_2: "~ησθητῐ", _, _;
    aorist_imp_pass_3: "~ησθητω", "~ησθητων", "~ησθεντων";
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_fut: _, _, "~ησθησεσθαι";
    infinitive_aor: _, _, "~ησθηναι";
    infinitive_futp: _, derived, _;
    participle_fut: _, _, "~ησθησομενος";
    participle_aor: _, _, "~ησθεις";
    participle_futp: _, derived, _;
    ),
    // Κᾰθιδρῡω: 5 words (κᾰθιδρῡω, ἀπερᾱω, ἐξερᾱω, ἐφηκω, ἀνηκω). Like Ἐλπω, except in 81 cells: future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, future_ind_pass_2 du, future_ind_pass_2 pl, future_ind_pass_3 sg, ….
    conjugation!("κᾰθιδρῡω", 5, "~ω", like Ἐλπω;
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
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: "~κᾰς", "~κᾰτον", "~κᾰτε";
    perfect_ind_act_3: "~κε", _, "~κᾱσῐ";
    perfect_ind_mid_1: "~μαι", _, "~μεθᾰ";
    perfect_ind_mid_3: "~ται", _, "~νται";
    perfect_ind_pass_1: _, _, "~μεθᾰ";
    perfect_subj_act_1: "~κω", _, "~κωμεν";
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ";
    perfect_opt_act_1: "~κοιμῐ", _, "~κοιμεν";
    perfect_opt_act_2: "~κοις", "~κοιτον", "~κοιτε";
    perfect_opt_act_3: "~κοι", "~κοιτην", "~κοιεν";
    perfect_imp_act_2: _, "~κετον", "~κετε";
    perfect_imp_act_3: "~κετω", "~κετων", "~κοντων";
    infinitive_fut: _, _, derived;
    infinitive_aor: _, _, derived;
    infinitive_perf: "~κεναι", _, _;
    participle_fut: _, _, derived;
    participle_aor: _, _, derived;
    participle_perf: "~κως", "~μενος", _;
    ),
    // Φρᾰζω: 5 words (φρᾰζω, θαυμᾰζω, θοᾰζω, θωᾰζω, χειμᾰζω). Like Πειρᾰζω, except in 19 cells: pluperfect_act_1 sg, pluperfect_act_1 pl, pluperfect_act_2 sg, pluperfect_act_2 du, pluperfect_act_2 pl, pluperfect_act_3 sg, ….
    conjugation!("φρᾰζω", 5, "~ζω", like Πειρᾰζω;
    pluperfect_act_1: "ἐ%ε~κειν|ἐ%ε~κη", _, "ἐ%ε~κεμεν";
    pluperfect_act_2: "ἐ%ε~κεις|ἐ%ε~κης", "ἐ%ε~κετον", "ἐ%ε~κετε";
    pluperfect_act_3: "ἐ%ε~κει|ἐ%ε~κειν", "ἐ%ε~κετην", "ἐ%ε~κεσᾰν";
    pluperfect_mid_1: "ἐ%ε~σμην", _, "ἐ%ε~σμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~σο", "ἐ%ε~σθον", "ἐ%ε~σθε";
    pluperfect_mid_3: "ἐ%ε~στο", "ἐ%ε~σθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~σμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~σθε";
    pluperfect_pass_3: _, "ἐ%ε~σθην", "ἐ%ε~δᾰτο";
    ),
    // Μᾰρτῠρεω: 5 words (μᾰρτῠρεω, λοιδορεω, μᾰραυγεω, δονεω, κερτομεω). Like Ποιεω, except in 61 cells: future_ind_act_3 pl, future_ind_mid_2 sg, future_ind_pass_2 sg, aorist_ind_act_3 sg, aorist_ind_pass_1 sg, aorist_ind_pass_1 pl, ….
    conjugation!("μᾰρτῠρεω", 5, "~εω", like Ποιεω;
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    future_ind_mid_2: "~ησῃ|~ησει", _, _;
    future_ind_pass_2: "~ηθησῃ|~ηθησει", _, _;
    aorist_ind_act_3: "ἐ~ησε|ἐ~ησεν", _, _;
    aorist_ind_pass_1: "~ηθην", _, "~ηθημεν";
    aorist_ind_pass_2: "~ηθης", "~ηθητον", "~ηθητε";
    aorist_ind_pass_3: "~ηθη", "~ηθητην", "~ηθησᾰν";
    aorist_subj_act_3: _, _, "~ησωσῐ|~ησωσῐν";
    aorist_subj_pass_1: "ἐ~ηθω", _, "ἐ~ηθωμεν";
    aorist_subj_pass_2: "ἐ~ηθῃς", "ἐ~ηθητον", "ἐ~ηθητε";
    aorist_subj_pass_3: "ἐ~ηθῃ", _, "ἐ~ηθωσῐ|ἐ~ηθωσῐν";
    aorist_opt_act_2: "~ησειᾰς|~ησαις", _, _;
    aorist_opt_act_3: "~ησειε|~ησειεν|~ησαι", _, "~ησειᾰν|~ησαιεν";
    aorist_opt_pass_1: "ἐ~ηθειην", _, "ἐ~ηθεῑμεν|ἐ~ηθειημεν";
    aorist_opt_pass_2: "ἐ~ηθειης", "ἐ~ηθεῑτον|ἐ~ηθειητον", "ἐ~ηθεῑτε|ἐ~ηθειητε";
    aorist_opt_pass_3: "ἐ~ηθειη", "ἐ~ηθειτην|ἐ~ηθειητην", "ἐ~ηθεῑεν|ἐ~ηθειησᾰν";
    aorist_imp_pass_2: "ἐ~ηθητῐ", _, _;
    aorist_imp_pass_3: "ἐ~ηθητω", "ἐ~ηθητων", "ἐ~ηθεντων";
    perfect_ind_act_3: "%ε~ηκε|%ε~ηκεν", _, "%ε~ηκᾱσῐ|%ε~ηκᾱσῐν";
    perfect_subj_act_3: _, _, "%ε~ηκωσῐ|%ε~ηκωσῐν";
    perfect_opt_act_1: "%ε~ηκοιμῐ|%ε~ηκοιην", _, _;
    perfect_opt_act_2: "%ε~ηκοις|%ε~ηκοιης", _, _;
    perfect_opt_act_3: "%ε~ηκοι|%ε~ηκοιη", _, _;
    perfect_imp_act_2: "%ε~ηκε", _, _;
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
    // Πειρᾰω: 5 words (κοπῐᾰω, ἀᾰω, πειρᾰω, μειδῐᾰω, νεᾰω). Like Ἐαω, except in 203 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("πειρᾰω", 5, "~ᾰω", like Ἐαω;
    future_ind_act_1: "~ᾱσω", _, "~ᾱσομεν";
    future_ind_act_2: "~ᾱσεις", "~ᾱσετον", "~ᾱσετε";
    future_ind_act_3: "~ᾱσει", _, "~ᾱσουσῐ";
    future_ind_mid_1: "~ᾱσομαι", _, "~ᾱσομεθᾰ";
    future_ind_mid_2: "~ᾱσῃ", "~ᾱσεσθον", "~ᾱσεσθε";
    future_ind_mid_3: "~ᾱσεται", _, "~ᾱσονται";
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_act_1: "~ᾱσοιμῐ", _, "~ᾱσοιμεν";
    future_opt_act_2: "~ᾱσοις", "~ᾱσοιτον", "~ᾱσοιτε";
    future_opt_act_3: "~ᾱσοι", "~ᾱσοιτην", "~ᾱσοιεν";
    future_opt_mid_1: "~ᾱσοιμην", _, "~ᾱσοιμεθᾰ";
    future_opt_mid_2: "~ᾱσοιο", "~ᾱσοισθον", "~ᾱσοισθε";
    future_opt_mid_3: "~ᾱσοιτο", "~ᾱσοισθην", "~ᾱσοιντο";
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_act_1: "ἐ~ᾱσᾰ", _, "ἐ~ᾱσᾰμεν";
    aorist_ind_act_2: "ἐ~ᾱσᾰς", "ἐ~ᾱσᾰτον", "ἐ~ᾱσᾰτε";
    aorist_ind_act_3: "ἐ~ᾱσε|ἐ~ᾱσεν", "ἐ~ᾱσᾰτην", "ἐ~ᾱσᾰν";
    aorist_ind_mid_1: "ἐ~ᾱσᾰμην", _, "ἐ~ᾱσᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ᾱσω", "ἐ~ᾱσᾰσθον", "ἐ~ᾱσᾰσθε";
    aorist_ind_mid_3: "ἐ~ᾱσᾰτο", "ἐ~ᾱσᾰσθην", "ἐ~ᾱσᾰντο";
    aorist_ind_pass_1: "ἐ~ᾱθην", _, "ἐ~ᾱθημεν";
    aorist_ind_pass_2: "ἐ~ᾱθης", "ἐ~ᾱθητον", "ἐ~ᾱθητε";
    aorist_ind_pass_3: "ἐ~ᾱθη", "ἐ~ᾱθητην", "ἐ~ᾱθησᾰν";
    aorist_subj_act_1: "~ᾱσω", _, "~ᾱσωμεν";
    aorist_subj_act_2: "~ᾱσῃς", "~ᾱσητον", "~ᾱσητε";
    aorist_subj_act_3: "~ᾱσῃ", _, "~ᾱσωσῐ|~ᾱσωσῐν";
    aorist_subj_mid_1: "~ᾱσωμαι", _, "~ᾱσωμεθᾰ";
    aorist_subj_mid_2: "~ᾱσῃ", "~ᾱσησθον", "~ᾱσησθε";
    aorist_subj_mid_3: "~ᾱσηται", _, "~ᾱσωνται";
    aorist_subj_pass_1: "~ᾱθω", _, "~ᾱθωμεν";
    aorist_subj_pass_2: "~ᾱθῃς", "~ᾱθητον", "~ᾱθητε";
    aorist_subj_pass_3: "~ᾱθῃ", _, "~ᾱθωσῐ";
    aorist_opt_act_1: "~ᾱσαιμῐ", _, "~ᾱσαιμεν";
    aorist_opt_act_2: "~ᾱσειᾰς|~ᾱσαις", "~ᾱσαιτον", "~ᾱσαιτε";
    aorist_opt_act_3: "~ᾱσειε|~ᾱσειεν|~ᾱσαι", "~ᾱσαιτην", "~ᾱσειᾰν|~ᾱσαιεν";
    aorist_opt_mid_1: "~ᾱσαιμην", _, "~ᾱσαιμεθᾰ";
    aorist_opt_mid_2: "~ᾱσαιο", "~ᾱσαισθον", "~ᾱσαισθε";
    aorist_opt_mid_3: "~ᾱσαιτο", "~ᾱσαισθην", "~ᾱσαιντο";
    aorist_opt_pass_1: "~ᾱθειην", _, "~ᾱθεῑμεν";
    aorist_opt_pass_2: "~ᾱθειης", "~ᾱθεῑτον", "~ᾱθεῑτε";
    aorist_opt_pass_3: "~ᾱθειη", "~ᾱθειτην", "~ᾱθεῑεν";
    aorist_imp_act_2: _, "~ᾱσᾰτον", "~ᾱσᾰτε";
    aorist_imp_act_3: "~ᾱσᾰτω", "~ᾱσᾰτων", "~ᾱσᾰντων";
    aorist_imp_mid_2: _, "~ᾱσᾰσθον", "~ᾱσᾰσθε";
    aorist_imp_mid_3: "~ᾱσᾰσθω", "~ᾱσᾰσθων", _;
    aorist_imp_pass_2: "~ᾱθητῐ", _, _;
    aorist_imp_pass_3: "~ᾱθητω", "~ᾱθητων", "~ᾱθεντων";
    perfect_ind_act_1: "%ε~ᾱκᾰ", _, "%ε~ᾱκᾰμεν";
    perfect_ind_act_2: "%ε~ᾱκᾰς", "%ε~ᾱκᾰτον", "%ε~ᾱκᾰτε";
    perfect_ind_act_3: "%ε~ᾱκε", _, "%ε~ᾱκᾱσῐ";
    perfect_ind_mid_1: "%ε~ᾱμαι", _, "%ε~ᾱμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾱσαι", "%ε~ᾱσθον", "%ε~ᾱσθε";
    perfect_ind_mid_3: "%ε~ᾱται", _, "%ε~ᾱνται";
    perfect_ind_pass_1: _, _, "%ε~ᾱμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~ᾱσθε";
    perfect_subj_act_1: "%ε~ᾱκω", _, "%ε~ᾱκωμεν";
    perfect_subj_act_2: "%ε~ᾱκῃς", "%ε~ᾱκητον", "%ε~ᾱκητε";
    perfect_subj_act_3: "%ε~ᾱκῃ", _, "%ε~ᾱκωσῐ";
    perfect_opt_act_1: "%ε~ᾱκοιμῐ", _, "%ε~ᾱκοιμεν";
    perfect_opt_act_2: "%ε~ᾱκοις", "%ε~ᾱκοιτον", "%ε~ᾱκοιτε";
    perfect_opt_act_3: "%ε~ᾱκοι", "%ε~ᾱκοιτην", "%ε~ᾱκοιεν";
    perfect_imp_act_2: _, "%ε~ᾱκετον", "%ε~ᾱκετε";
    perfect_imp_act_3: "%ε~ᾱκετω", "%ε~ᾱκετων", "%ε~ᾱκοντων";
    perfect_imp_mid_2: "%ε~ᾱσο", _, _;
    perfect_imp_mid_3: "%ε~ᾱσθω", "%ε~ᾱσθων", _;
    perfect_imp_pass_2: "%ε~ᾱσο", _, _;
    perfect_imp_pass_3: "%ε~ᾱσθω", "%ε~ᾱσθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: "ἐ%ε~ᾱμην", _, "ἐ%ε~ᾱμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ᾱσο", "ἐ%ε~ᾱσθον", "ἐ%ε~ᾱσθε";
    pluperfect_mid_3: "ἐ%ε~ᾱτο", "ἐ%ε~ᾱσθην", "ἐ%ε~ᾱντο";
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
    infinitive_fut: "~ᾱσειν", "~ᾱσεσθαι", derived;
    infinitive_aor: _, "~ᾱσᾰσθαι", "~ᾱθηναι";
    infinitive_perf: "%ε~ᾱκεναι", _, _;
    participle_fut: "~ᾱσων", "~ᾱσομενος", derived;
    participle_aor: "~ᾱσᾱς", "~ᾱσᾰμενος", "~ᾱθεις";
    participle_perf: "%ε~ᾱκως", "%ε~ᾱμενος", _;
    ),
    // Σῠνοικῐζω: 5 words (ἀνοικῐζω, κᾰτοικῐζω, σῠνοικῐζω, ξῠνοικῐζω, ἀποικῐζω). Like Προσερχομαι, except in 300 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("σῠνοικῐζω", 5, "~οι~ζω", like Προσερχομαι;
    ind_act_1: "~οι~ζω", _, "~οι~ζομεν";
    ind_act_2: "~οι~ζεις", _, _;
    ind_act_3: "~οι~ζει", _, "~οι~ζουσῐ|~οι~ζουσῐν";
    ind_mid_1: "~οι~ζομαι", _, "~οι~ζομεθᾰ";
    ind_mid_2: "~οι~ζῃ|~οι~ζει", "~οι~ζεσθον", "~οι~ζεσθε";
    ind_mid_3: "~οι~ζεται", _, "~οι~ζονται";
    ind_pass_1: _, _, "~οι~ζομεθᾰ";
    ind_pass_2: "~οι~ζῃ|~οι~ζει", _, "~οι~ζεσθε";
    ind_pass_3: "~οι~ζεται", _, "~οι~ζονται";
    subj_act_1: "~οι~ζω", _, "~οι~ζωμεν";
    subj_act_2: "~οι~ζῃς", "~οι~ζητον", "~οι~ζητε";
    subj_act_3: "~οι~ζῃ", _, "~οι~ζωσῐ|~οι~ζωσῐν";
    subj_mid_1: "~οι~ζωμαι", _, "~οι~ζωμεθᾰ";
    subj_mid_2: "~οι~ζῃ", "~οι~ζησθον", "~οι~ζησθε";
    subj_mid_3: "~οι~ζηται", _, "~οι~ζωνται";
    subj_pass_1: _, _, "~οι~ζωμεθᾰ";
    subj_pass_2: _, _, "~οι~ζησθε";
    subj_pass_3: "~οι~ζηται", _, "~οι~ζωνται";
    opt_act_1: "~οι~ζοιμῐ", _, "~οι~ζοιμεν";
    opt_act_2: "~οι~ζοις", "~οι~ζοιτον", "~οι~ζοιτε";
    opt_act_3: "~οι~ζοι", "~οι~ζοιτην", "~οι~ζοιεν";
    opt_mid_1: "~οι~ζοιμην", _, "~οι~ζοιμεθᾰ";
    opt_mid_2: "~οι~ζοιο", "~οι~ζοισθον", "~οι~ζοισθε";
    opt_mid_3: "~οι~ζοιτο", "~οι~ζοισθην", "~οι~ζοιντο";
    opt_pass_1: _, _, "~οι~ζοιμεθᾰ";
    opt_pass_2: _, "~οι~ζοισθον", "~οι~ζοισθε";
    opt_pass_3: "~οι~ζοιτο", "~οι~ζοισθην", "~οι~ζοιντο";
    imp_act_2: "~οι~ζε", "~οι~ζετον", "~οι~ζετε";
    imp_act_3: "~οι~ζετω", "~οι~ζετων", "~οι~ζοντων";
    imp_mid_2: "~οι~ζου", _, "~οι~ζεσθε";
    imp_mid_3: "~οι~ζεσθω", "~οι~ζεσθων", "~οι~ζεσθων";
    imp_pass_2: _, _, "~οι~ζεσθε";
    imp_pass_3: _, "~οι~ζεσθων", _;
    imperfect_act_1: "~ῳ~ζον", _, "~ῳ~ζομεν";
    imperfect_act_2: "~ῳ~ζες", "~ῳ~ζετον", "~ῳ~ζετε";
    imperfect_act_3: "~ῳ~ζε|~ῳ~ζεν", "~ῳ~ζετην", "~ῳ~ζον";
    imperfect_mid_1: "~ῳ~ζομην", _, "~ῳ~ζομεθᾰ";
    imperfect_mid_2: "~ῳ~ζου", "~ῳ~ζεσθον", "~ῳ~ζεσθε";
    imperfect_mid_3: "~ῳ~ζετο", "~ῳ~ζεσθην", "~ῳ~ζοντο";
    imperfect_pass_1: _, _, "~ῳ~ζομεθᾰ";
    imperfect_pass_2: _, _, "~ῳ~ζεσθε";
    imperfect_pass_3: "~ῳ~ζετο", _, _;
    future_ind_act_1: "~οι~ω", _, "~οι~οῡμεν";
    future_ind_act_2: "~οι~εῑς", "~οι~εῑτον", "~οι~εῑτε";
    future_ind_act_3: "~οι~εῑ", _, "~οι~οῡσῐ|~οι~οῡσῐν";
    future_ind_mid_1: "~οι~οῡμαι", _, "~οι~ουμεθᾰ";
    future_ind_mid_2: "~οι~εῑ|~οι~ῃ", "~οι~εῑσθον", "~οι~εῑσθε";
    future_ind_mid_3: "~οι~εῑται", _, "~οι~οῡνται";
    future_ind_pass_1: "~οι~σθησομαι", _, "~οι~σθησομεθᾰ";
    future_ind_pass_2: "~οι~σθησῃ|~οι~σθησει", "~οι~σθησεσθον", "~οι~σθησεσθε";
    future_ind_pass_3: "~οι~σθησεται", _, "~οι~σθησονται";
    future_opt_act_1: "~οι~οιην|~οι~οῑμῐ", _, "~οι~οῑμεν|~οι~οιημεν";
    future_opt_act_2: "~οι~οιης|~οι~οῑς", "~οι~οῑτον|~οι~οιητον", "~οι~οῑτε|~οι~οιητε";
    future_opt_act_3: "~οι~οιη|~οι~οῑ", "~οι~οιτην|~οι~οιητην", "~οι~οῑεν|~οι~οιησᾰν";
    future_opt_mid_1: "~οι~οιμην", _, "~οι~οιμεθᾰ";
    future_opt_mid_2: "~οι~οῑο", "~οι~οῑσθον", "~οι~οῑσθε";
    future_opt_mid_3: "~οι~οῑτο", "~οι~οισθην", "~οι~οῑντο";
    future_opt_pass_1: "~οι~σθησοιμην", _, "~οι~σθησοιμεθᾰ";
    future_opt_pass_2: "~οι~σθησοιο", "~οι~σθησοισθον", "~οι~σθησοισθε";
    future_opt_pass_3: "~οι~σθησοιτο", "~οι~σθησοισθην", "~οι~σθησοιντο";
    aorist_ind_act_1: "~ῳ~σᾰ", _, "~ῳ~σᾰμεν";
    aorist_ind_act_2: "~ῳ~σᾰς", "~ῳ~σᾰτον", "~ῳ~σᾰτε";
    aorist_ind_act_3: "~ῳ~σε|~ῳ~σεν", "~ῳ~σᾰτην", "~ῳ~σᾰν";
    aorist_ind_mid_1: "~ῳ~σᾰμην", _, "~ῳ~σᾰμεθᾰ";
    aorist_ind_mid_2: "~ῳ~σω", "~ῳ~σᾰσθον", "~ῳ~σᾰσθε";
    aorist_ind_mid_3: "~ῳ~σᾰτο", "~ῳ~σᾰσθην", "~ῳ~σᾰντο";
    aorist_ind_pass_1: "~ῳ~σθην", _, "~ῳ~σθημεν";
    aorist_ind_pass_2: "~ῳ~σθης", "~ῳ~σθητον", "~ῳ~σθητε";
    aorist_ind_pass_3: "~ῳ~σθη", "~ῳ~σθητην", "~ῳ~σθησᾰν";
    aorist_subj_act_1: "~οι~σω", _, "~οι~σωμεν";
    aorist_subj_act_2: "~οι~σῃς", "~οι~σητον", "~οι~σητε";
    aorist_subj_act_3: "~οι~σῃ", _, "~οι~σωσῐ|~οι~σωσῐν";
    aorist_subj_mid_1: "~οι~σωμαι", _, "~οι~σωμεθᾰ";
    aorist_subj_mid_2: "~οι~σῃ", "~οι~σησθον", "~οι~σησθε";
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
    perfect_ind_pass_1: _, _, "~ῳ~σμεθᾰ";
    perfect_ind_pass_2: _, _, "~ῳ~σθε";
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
    perfect_imp_pass_2: "~ῳ~σο", _, _;
    perfect_imp_pass_3: "~ῳ~σθω", "~ῳ~σθων", _;
    pluperfect_act_1: "~ῳ~κειν|~ῳ~κη", _, "~ῳ~κεμεν";
    pluperfect_act_2: "~ῳ~κεις|~ῳ~κης", "~ῳ~κετον", "~ῳ~κετε";
    pluperfect_act_3: "~ῳ~κει|~ῳ~κειν", "~ῳ~κετην", "~ῳ~κεσᾰν";
    pluperfect_mid_1: "~ῳ~σμην", _, "~ῳ~σμεθᾰ";
    pluperfect_mid_2: "~ῳ~σο", "~ῳ~σθον", "~ῳ~σθε";
    pluperfect_mid_3: "~ῳ~στο", "~ῳ~σθην", _;
    pluperfect_pass_1: _, _, "~ῳ~σμεθᾰ";
    pluperfect_pass_2: _, _, "~ῳ~σθε";
    pluperfect_pass_3: _, "~ῳ~σθην", "~ῳ~δᾰτο";
    infinitive_pres: "~οι~ζειν", "~οι~ζεσθαι", _;
    infinitive_fut: "~οι~εῑν", "~οι~εῑσθαι", "~οι~σθησεσθαι";
    infinitive_aor: "~οι~σαι", "~οι~σᾰσθαι", "~οι~σθηναι";
    infinitive_perf: "~ῳ~κεναι", "~ῳ~σθαι", _;
    participle_pres: "~οι~ζων", "~οι~ζομενος", _;
    participle_fut: "~οι~ων", "~οι~ουμενος", "~οι~σθησομενος";
    participle_aor: "~οι~σᾱς", "~οι~σᾰμενος", "~οι~σθεις";
    participle_perf: "~ῳ~κως", "~ῳ~σμενος", _;
    ),
    // Ἐξᾰκολουθεω: 5 words (πᾰρᾰμελεω, σῠνᾰδῐκεω, ξῠνᾰδῐκεω, ἀπᾰσχολεω, ἐξᾰκολουθεω). Like Ὑπᾰρχω, except in 276 cells: ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ind_mid_1 pl, ….
    conjugation!("ἐξᾰκολουθεω", 5, "~ᾰ~εω", like Ὑπᾰρχω;
    ind_act_1: _, _, "~ᾰ~οῡμεν";
    ind_act_2: "~ᾰ~εῑς", _, _;
    ind_act_3: "~ᾰ~εῑ", _, "~ᾰ~οῡσῐ|~ᾰ~οῡσῐν";
    ind_mid_1: "~ᾰ~οῡμαι", _, "~ᾰ~ουμεθᾰ";
    ind_mid_2: "~ᾰ~εῑ|~ᾰ~ῃ", "~ᾰ~εῑσθον", "~ᾰ~εῑσθε";
    ind_mid_3: "~ᾰ~εῑται", _, "~ᾰ~οῡνται";
    ind_pass_1: _, _, "~ᾰ~ουμεθᾰ";
    ind_pass_2: "~ᾰ~εῑ|~ᾰ~ῃ", _, "~ᾰ~εῑσθε";
    ind_pass_3: "~ᾰ~εῑται", _, "~ᾰ~οῡνται";
    subj_act_3: _, _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    opt_act_1: "~ᾰ~οιην|~ᾰ~οῑμῐ", _, "~ᾰ~οῑμεν|~ᾰ~οιημεν";
    opt_act_2: "~ᾰ~οιης|~ᾰ~οῑς", "~ᾰ~οῑτον|~ᾰ~οιητον", "~ᾰ~οῑτε|~ᾰ~οιητε";
    opt_act_3: "~ᾰ~οιη|~ᾰ~οῑ", "~ᾰ~οιτην|~ᾰ~οιητην", "~ᾰ~οῑεν|~ᾰ~οιησᾰν";
    opt_mid_2: "~ᾰ~οῑο", "~ᾰ~οῑσθον", "~ᾰ~οῑσθε";
    opt_mid_3: "~ᾰ~οῑτο", _, "~ᾰ~οῑντο";
    opt_pass_2: _, "~ᾰ~οῑσθον", "~ᾰ~οῑσθε";
    opt_pass_3: "~ᾰ~οῑτο", _, "~ᾰ~οῑντο";
    imp_act_2: "~ᾰ~ει", "~ᾰ~εῑτον", "~ᾰ~εῑτε";
    imp_act_3: "~ᾰ~ειτω", "~ᾰ~ειτων", "~ᾰ~ουντων";
    imp_mid_2: "~ᾰ~οῡ", _, "~ᾰ~εῑσθε";
    imp_mid_3: "~ᾰ~εισθω", "~ᾰ~εισθων", "~ᾰ~εισθων";
    imp_pass_2: _, _, "~ᾰ~εῑσθε";
    imp_pass_3: _, "~ᾰ~εισθων", _;
    imperfect_act_1: "~η~ουν", _, "~η~οῡμεν";
    imperfect_act_2: "~η~εις", "~η~εῑτον", "~η~εῑτε";
    imperfect_act_3: "~η~ει", "~η~ειτην", "~η~ουν";
    imperfect_mid_1: "~η~ουμην", _, "~η~ουμεθᾰ";
    imperfect_mid_2: "~η~οῡ", "~η~εῑσθον", "~η~εῑσθε";
    imperfect_mid_3: "~η~εῑτο", "~η~εισθην", "~η~οῡντο";
    imperfect_pass_1: _, _, "~η~ουμεθᾰ";
    imperfect_pass_2: _, _, "~η~εῑσθε";
    imperfect_pass_3: "~η~εῑτο", _, _;
    future_ind_act_1: "~ᾰ~ησω", _, "~ᾰ~ησομεν";
    future_ind_act_2: "~ᾰ~ησεις", "~ᾰ~ησετον", "~ᾰ~ησετε";
    future_ind_act_3: "~ᾰ~ησει", _, "~ᾰ~ησουσῐ|~ᾰ~ησουσῐν";
    future_ind_mid_1: "~ᾰ~ησομαι", _, "~ᾰ~ησομεθᾰ";
    future_ind_mid_2: "~ᾰ~ησῃ|~ᾰ~ησει", "~ᾰ~ησεσθον", "~ᾰ~ησεσθε";
    future_ind_mid_3: "~ᾰ~ησεται", _, "~ᾰ~ησονται";
    future_ind_pass_1: "~ᾰ~ηθησομαι", _, "~ᾰ~ηθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ηθησῃ", "~ᾰ~ηθησεσθον", "~ᾰ~ηθησεσθε";
    future_ind_pass_3: "~ᾰ~ηθησεται", _, "~ᾰ~ηθησονται";
    future_opt_act_1: "~ᾰ~ησοιμῐ", _, "~ᾰ~ησοιμεν";
    future_opt_act_2: "~ᾰ~ησοις", "~ᾰ~ησοιτον", "~ᾰ~ησοιτε";
    future_opt_act_3: "~ᾰ~ησοι", "~ᾰ~ησοιτην", "~ᾰ~ησοιεν";
    future_opt_mid_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ησοιο", "~ᾰ~ησοισθον", "~ᾰ~ησοισθε";
    future_opt_mid_3: "~ᾰ~ησοιτο", "~ᾰ~ησοισθην", "~ᾰ~ησοιντο";
    future_opt_pass_1: "~ᾰ~ηθησοιμην", _, "~ᾰ~ηθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ηθησοιο", "~ᾰ~ηθησοισθον", "~ᾰ~ηθησοισθε";
    future_opt_pass_3: "~ᾰ~ηθησοιτο", "~ᾰ~ηθησοισθην", "~ᾰ~ηθησοιντο";
    aorist_ind_act_1: "~η~ησᾰ", _, "~η~ησᾰμεν";
    aorist_ind_act_2: "~η~ησᾰς", "~η~ησᾰτον", "~η~ησᾰτε";
    aorist_ind_act_3: "~η~ησε|~η~ησεν", "~η~ησᾰτην", "~η~ησᾰν";
    aorist_ind_mid_1: "~η~ησᾰμην", _, "~η~ησᾰμεθᾰ";
    aorist_ind_mid_2: "~η~ησω", "~η~ησᾰσθον", "~η~ησᾰσθε";
    aorist_ind_mid_3: "~η~ησᾰτο", "~η~ησᾰσθην", "~η~ησᾰντο";
    aorist_ind_pass_1: "~η~ηθην", _, "~η~ηθημεν";
    aorist_ind_pass_2: "~η~ηθης", "~η~ηθητον", "~η~ηθητε";
    aorist_ind_pass_3: "~η~ηθη", "~η~ηθητην", "~η~ηθησᾰν";
    aorist_subj_act_1: "~ᾰ~ησω", _, "~ᾰ~ησωμεν";
    aorist_subj_act_2: "~ᾰ~ησῃς", "~ᾰ~ησητον", "~ᾰ~ησητε";
    aorist_subj_act_3: "~ᾰ~ησῃ", _, "~ᾰ~ησωσῐ|~ᾰ~ησωσῐν";
    aorist_subj_mid_1: "~ᾰ~ησωμαι", _, "~ᾰ~ησωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~ησῃ", "~ᾰ~ησησθον", "~ᾰ~ησησθε";
    aorist_subj_mid_3: "~ᾰ~ησηται", _, "~ᾰ~ησωνται";
    aorist_subj_pass_1: "~ᾰ~ηθω", _, "~ᾰ~ηθωμεν";
    aorist_subj_pass_2: "~ᾰ~ηθῃς", "~ᾰ~ηθητον", "~ᾰ~ηθητε";
    aorist_subj_pass_3: "~ᾰ~ηθῃ", _, "~ᾰ~ηθωσῐ";
    aorist_opt_act_1: "~ᾰ~ησαιμῐ", _, "~ᾰ~ησαιμεν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς|~ᾰ~ησαις", "~ᾰ~ησαιτον", "~ᾰ~ησαιτε";
    aorist_opt_act_3: "~ᾰ~ησειε|~ᾰ~ησειεν|~ᾰ~ησαι", "~ᾰ~ησαιτην", "~ᾰ~ησειᾰν|~ᾰ~ησαιεν";
    aorist_opt_mid_1: "~ᾰ~ησαιμην", _, "~ᾰ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~ησαιο", "~ᾰ~ησαισθον", "~ᾰ~ησαισθε";
    aorist_opt_mid_3: "~ᾰ~ησαιτο", "~ᾰ~ησαισθην", "~ᾰ~ησαιντο";
    aorist_opt_pass_1: "~ᾰ~ηθειην", _, "~ᾰ~ηθεῑμεν";
    aorist_opt_pass_2: "~ᾰ~ηθειης", "~ᾰ~ηθεῑτον", "~ᾰ~ηθεῑτε";
    aorist_opt_pass_3: "~ᾰ~ηθειη", "~ᾰ~ηθειτην", "~ᾰ~ηθεῑεν";
    aorist_imp_act_2: "~ᾰ~ησον", "~ᾰ~ησᾰτον", "~ᾰ~ησᾰτε";
    aorist_imp_act_3: "~ᾰ~ησᾰτω", "~ᾰ~ησᾰτων", "~ᾰ~ησᾰντων";
    aorist_imp_mid_2: "~ᾰ~ησαι", "~ᾰ~ησᾰσθον", "~ᾰ~ησᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ησᾰσθω", "~ᾰ~ησᾰσθων", _;
    aorist_imp_pass_2: "~ᾰ~ηθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ηθητω", "~ᾰ~ηθητων", "~ᾰ~ηθεντων";
    perfect_ind_act_1: "~η~ηκᾰ", _, "~η~ηκᾰμεν";
    perfect_ind_act_2: "~η~ηκᾰς", "~η~ηκᾰτον", "~η~ηκᾰτε";
    perfect_ind_act_3: "~η~ηκε|~η~ηκεν", _, "~η~ηκᾱσῐ|~η~ηκᾱσῐν";
    perfect_ind_mid_1: "~η~ημαι", _, "~η~ημεθᾰ";
    perfect_ind_mid_2: "~η~ησαι", "~η~ησθον", "~η~ησθε";
    perfect_ind_mid_3: "~η~ηται", _, "~η~ηνται";
    perfect_ind_pass_1: _, _, "~η~ημεθᾰ";
    perfect_ind_pass_2: _, _, "~η~ησθε";
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
    perfect_imp_pass_2: "~η~ησο", _, _;
    perfect_imp_pass_3: "~η~ησθω", "~η~ησθων", _;
    pluperfect_act_1: "~η~ηκειν|~η~ηκη", _, "~η~ηκεμεν";
    pluperfect_act_2: "~η~ηκεις|~η~ηκης", "~η~ηκετον", "~η~ηκετε";
    pluperfect_act_3: "~η~ηκει|~η~ηκειν", "~η~ηκετην", "~η~ηκεσᾰν";
    pluperfect_mid_1: "~η~ημην", _, "~η~ημεθᾰ";
    pluperfect_mid_2: "~η~ησο", "~η~ησθον", "~η~ησθε";
    pluperfect_mid_3: "~η~ητο", "~η~ησθην", _;
    pluperfect_pass_1: _, _, "~η~ημεθᾰ";
    pluperfect_pass_2: _, _, "~η~ησθε";
    pluperfect_pass_3: _, "~η~ησθην", "~η~ηντο";
    infinitive_pres: "~ᾰ~εῑν", "~ᾰ~εῑσθαι", _;
    infinitive_fut: "~ᾰ~ησειν", "~ᾰ~ησεσθαι", "~ᾰ~ηθησεσθαι";
    infinitive_aor: "~ᾰ~ησαι", "~ᾰ~ησᾰσθαι", "~ᾰ~ηθηναι";
    infinitive_perf: "~η~ηκεναι", "~η~ησθαι", _;
    participle_pres: _, "~ᾰ~ουμενος", _;
    participle_fut: "~ᾰ~ησων", "~ᾰ~ησομενος", "~ᾰ~ηθησομενος";
    participle_aor: "~ᾰ~ησᾱς", "~ᾰ~ησᾰμενος", "~ᾰ~ηθεις";
    participle_perf: "~η~ηκως", "~η~ημενος", _;
    ),
    // Ἱζω: 4 words (ἑλληνῐζω, ἀφῠπνῐζω, ῥωμᾱϊζω, ἐξῠβρῐζω). Like Κᾰτεργαζομαι, except in 116 cells: ind_act_3 pl, ind_mid_2 sg, ind_pass_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_1 pl, ….
    conjugation!("ἱζω", 4, "~ζω", like Κᾰτεργαζομαι;
    ind_act_3: _, _, "~ζουσῐ|~ζουσῐν";
    ind_mid_2: "~ζῃ|~ζει", _, _;
    ind_pass_2: "~ζῃ|~ζει", _, _;
    subj_act_3: _, _, "~ζωσῐ|~ζωσῐν";
    imperfect_act_1: "~ζον", _, "~ζομεν";
    imperfect_act_2: "~ζες", "~ζετον", "~ζετε";
    imperfect_act_3: "~ζε|~ζεν", "~ζετην", "~ζον";
    imperfect_mid_1: "~ζομην", _, "~ζομεθᾰ";
    imperfect_mid_2: "~ζου", "~ζεσθον", "~ζεσθε";
    imperfect_mid_3: "~ζετο", "~ζεσθην", "~ζοντο";
    imperfect_pass_1: _, _, "~ζομεθᾰ";
    imperfect_pass_2: _, _, "~ζεσθε";
    imperfect_pass_3: "~ζετο", _, _;
    aorist_ind_act_1: "~σᾰ", _, "~σᾰμεν";
    aorist_ind_act_2: "~σᾰς", "~σᾰτον", "~σᾰτε";
    aorist_ind_act_3: "~σε", "~σᾰτην", "~σᾰν";
    aorist_ind_mid_1: "~σᾰμην", _, "~σᾰμεθᾰ";
    aorist_ind_mid_2: "~σω", "~σᾰσθον", "~σᾰσθε";
    aorist_ind_mid_3: "~σᾰτο", "~σᾰσθην", "~σᾰντο";
    aorist_ind_pass_1: "~σθην", _, "~σθημεν";
    aorist_ind_pass_2: "~σθης", "~σθητον", "~σθητε";
    aorist_ind_pass_3: "~σθη", "~σθητην", "~σθησᾰν";
    aorist_subj_act_3: _, _, "~σωσῐ|~σωσῐν";
    aorist_opt_act_2: "~σειᾰς|~σαις", _, _;
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", _, "~σειᾰν|~σαιεν";
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: "~κᾰς", "~κᾰτον", "~κᾰτε";
    perfect_ind_act_3: "~κε", _, "~κᾱσῐ";
    perfect_ind_mid_1: "~σμαι", _, "~σμεθᾰ";
    perfect_ind_mid_2: "~σαι", "~σθον", "~σθε";
    perfect_ind_mid_3: "~σται", _, "~δᾰται";
    perfect_ind_pass_1: _, _, "~σμεθᾰ";
    perfect_ind_pass_2: _, _, "~σθε";
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
    perfect_imp_pass_2: "~σο", _, _;
    perfect_imp_pass_3: "~σθω", "~σθων", _;
    pluperfect_act_1: "~κειν", _, "~κεμεν";
    pluperfect_act_2: "~κεις", "~κετον", "~κετε";
    pluperfect_act_3: "~κει", "~κετην", "~κεσᾰν";
    pluperfect_mid_1: "~σμην", _, "~σμεθᾰ";
    pluperfect_mid_2: "~σο", "~σθον", "~σθε";
    pluperfect_mid_3: "~στο", "~σθην", _;
    pluperfect_pass_1: _, _, "~σμεθᾰ";
    pluperfect_pass_2: _, _, "~σθε";
    pluperfect_pass_3: _, "~σθην", "~δᾰτο";
    infinitive_perf: "~κεναι", "~σθαι", _;
    participle_perf: "~κως", "~σμενος", _;
    ),
    // Ἡγεομαι: 4 words (ἡγεομαι, νεομαι, ἐξηγεομαι, εὐνομεομαι). Like Ὠνεομαι, except in 109 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, subj_act_1 sg, ….
    conjugation!("ἡγεομαι", 4, "~εομαι", like Ὠνεομαι;
    ind_act_1: "~εομαι", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, _, derived;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imp_act_3: derived, derived, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    imperfect_mid_1: "~ουμην", _, "~ουμεθᾰ";
    imperfect_mid_2: "~οῡ", "~εῑσθον", "~εῑσθε";
    imperfect_mid_3: "~εῑτο", "~εισθην", "~οῡντο";
    imperfect_pass_1: _, _, "~ουμεθᾰ";
    imperfect_pass_2: _, _, "~εῑσθε";
    imperfect_pass_3: "~εῑτο", _, _;
    aorist_ind_mid_1: "~ησᾰμην", _, "~ησᾰμεθᾰ";
    aorist_ind_mid_2: "~ησω", "~ησᾰσθον", "~ησᾰσθε";
    aorist_ind_mid_3: "~ησᾰτο", "~ησᾰσθην", "~ησᾰντο";
    aorist_ind_pass_1: "~ηθην", _, "~ηθημεν";
    aorist_ind_pass_2: "~ηθης", "~ηθητον", "~ηθητε";
    aorist_ind_pass_3: "~ηθη", "~ηθητην", "~ηθησᾰν";
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: derived, derived, derived;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_mid_1: "~ημαι", _, "~ημεθᾰ";
    perfect_ind_mid_2: "~ησαι", "~ησθον", "~ησθε";
    perfect_ind_mid_3: "~ηται", _, "~ηνται";
    perfect_ind_pass_1: _, _, "~ημεθᾰ";
    perfect_ind_pass_2: _, _, "~ησθε";
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: _, derived, derived;
    perfect_imp_act_3: derived, derived, derived;
    perfect_imp_mid_2: "~ησο", _, _;
    perfect_imp_mid_3: "~ησθω", "~ησθων", _;
    perfect_imp_pass_2: "~ησο", _, _;
    perfect_imp_pass_3: "~ησθω", "~ησθων", _;
    infinitive_pres: derived, _, _;
    infinitive_perf: derived, "~ησθαι", _;
    participle_pres: derived, _, _;
    participle_perf: derived, "~ημενος", _;
    ),
    // Ἐπᾰγγελλω: 4 words (πᾰρᾰγγελλω, ἀπᾰγγελλω, ἐπᾰγγελλω, ἀνᾰγγελλω).
    conjugation!("ἐπᾰγγελλω", 4, "~ᾰ~~λω";
    ind_act_1: "~ᾰ~~λω", _, "~ᾰ~~λομεν";
    ind_act_2: "~ᾰ~~λεις", _, _;
    ind_act_3: "~ᾰ~~λει", _, "~ᾰ~~λουσῐ|~ᾰ~~λουσῐν";
    ind_mid_1: "~ᾰ~~λομαι", _, "~ᾰ~~λομεθᾰ";
    ind_mid_2: "~ᾰ~~λῃ|~ᾰ~~λει", "~ᾰ~~λεσθον", "~ᾰ~~λεσθε";
    ind_mid_3: "~ᾰ~~λεται", _, "~ᾰ~~λονται";
    ind_pass_1: _, _, "~ᾰ~~λομεθᾰ";
    ind_pass_2: "~ᾰ~~λῃ|~ᾰ~~λει", _, "~ᾰ~~λεσθε";
    ind_pass_3: "~ᾰ~~λεται", _, "~ᾰ~~λονται";
    subj_act_1: "~ᾰ~~λω", _, "~ᾰ~~λωμεν";
    subj_act_2: "~ᾰ~~λῃς", "~ᾰ~~λητον", "~ᾰ~~λητε";
    subj_act_3: "~ᾰ~~λῃ", _, "~ᾰ~~λωσῐ|~ᾰ~~λωσῐν";
    subj_mid_1: "~ᾰ~~λωμαι", _, "~ᾰ~~λωμεθᾰ";
    subj_mid_2: "~ᾰ~~λῃ", "~ᾰ~~λησθον", "~ᾰ~~λησθε";
    subj_mid_3: "~ᾰ~~ληται", _, "~ᾰ~~λωνται";
    subj_pass_1: _, _, "~ᾰ~~λωμεθᾰ";
    subj_pass_2: _, _, "~ᾰ~~λησθε";
    subj_pass_3: "~ᾰ~~ληται", _, "~ᾰ~~λωνται";
    opt_act_1: "~ᾰ~~λοιμῐ", _, "~ᾰ~~λοιμεν";
    opt_act_2: "~ᾰ~~λοις", "~ᾰ~~λοιτον", "~ᾰ~~λοιτε";
    opt_act_3: "~ᾰ~~λοι", "~ᾰ~~λοιτην", "~ᾰ~~λοιεν";
    opt_mid_1: "~ᾰ~~λοιμην", _, "~ᾰ~~λοιμεθᾰ";
    opt_mid_2: "~ᾰ~~λοιο", "~ᾰ~~λοισθον", "~ᾰ~~λοισθε";
    opt_mid_3: "~ᾰ~~λοιτο", "~ᾰ~~λοισθην", "~ᾰ~~λοιντο";
    opt_pass_1: _, _, "~ᾰ~~λοιμεθᾰ";
    opt_pass_2: _, "~ᾰ~~λοισθον", "~ᾰ~~λοισθε";
    opt_pass_3: "~ᾰ~~λοιτο", "~ᾰ~~λοισθην", "~ᾰ~~λοιντο";
    imp_act_2: "~ᾰ~~λε", "~ᾰ~~λετον", "~ᾰ~~λετε";
    imp_act_3: "~ᾰ~~λετω", "~ᾰ~~λετων", "~ᾰ~~λοντων";
    imp_mid_2: "~ᾰ~~λου", _, "~ᾰ~~λεσθε";
    imp_mid_3: "~ᾰ~~λεσθω", "~ᾰ~~λεσθων", "~ᾰ~~λεσθων";
    imp_pass_2: _, _, "~ᾰ~~λεσθε";
    imp_pass_3: _, "~ᾰ~~λεσθων", _;
    imperfect_act_1: "~η~~λον", _, "~η~~λομεν";
    imperfect_act_2: "~η~~λες", "~η~~λετον", "~η~~λετε";
    imperfect_act_3: "~η~~λε|~η~~λεν", "~η~~λετην", "~η~~λον";
    imperfect_mid_1: "~η~~λομην", _, "~η~~λομεθᾰ";
    imperfect_mid_2: "~η~~λου", "~η~~λεσθον", "~η~~λεσθε";
    imperfect_mid_3: "~η~~λετο", "~η~~λεσθην", "~η~~λοντο";
    imperfect_pass_1: _, _, "~η~~λομεθᾰ";
    imperfect_pass_2: _, _, "~η~~λεσθε";
    imperfect_pass_3: "~η~~λετο", _, _;
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
    aorist_ind_act_1: "~η~ι~ᾰ", _, "~η~ι~ᾰμεν";
    aorist_ind_act_2: "~η~ι~ᾰς", "~η~ι~ᾰτον", "~η~ι~ᾰτε";
    aorist_ind_act_3: "~η~ι~ε|~η~ι~εν", "~η~ι~ᾰτην", "~η~ι~ᾰν";
    aorist_ind_mid_1: "~η~ι~ᾰμην", _, "~η~ι~ᾰμεθᾰ";
    aorist_ind_mid_2: "~η~ι~ω", "~η~ι~ᾰσθον", "~η~ι~ᾰσθε";
    aorist_ind_mid_3: "~η~ι~ᾰτο", "~η~ι~ᾰσθην", "~η~ι~ᾰντο";
    aorist_ind_pass_1: "~η~~θην", _, "~η~~θημεν";
    aorist_ind_pass_2: "~η~~θης", "~η~~θητον", "~η~~θητε";
    aorist_ind_pass_3: "~η~~θη", "~η~~θητην", "~η~~θησᾰν";
    aorist_subj_act_1: "~ᾰ~ι~ω", _, "~ᾰ~ι~ωμεν";
    aorist_subj_act_2: "~ᾰ~ι~ῃς", "~ᾰ~ι~ητον", "~ᾰ~ι~ητε";
    aorist_subj_act_3: "~ᾰ~ι~ῃ", _, "~ᾰ~ι~ωσῐ|~ᾰ~ι~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ι~ωμαι", _, "~ᾰ~ι~ωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~ι~ῃ", "~ᾰ~ι~ησθον", "~ᾰ~ι~ησθε";
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
    perfect_ind_pass_1: _, _, "~η~~μεθᾰ";
    perfect_ind_pass_2: _, _, "~η~~θε";
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
    perfect_imp_pass_2: "~η~~σο", _, _;
    perfect_imp_pass_3: "~η~~θω", "~η~~θων", _;
    infinitive_pres: "~ᾰ~~λειν", "~ᾰ~~λεσθαι", _;
    infinitive_fut: "~ᾰ~~εῑν", "~ᾰ~~εῑσθαι", "~ᾰ~~θησεσθαι";
    infinitive_aor: "~ᾰ~ῑ~αι", "~ᾰ~ι~ᾰσθαι", "~ᾰ~~θηναι";
    infinitive_perf: "~η~~κεναι", "~η~~θαι", _;
    participle_pres: "~ᾰ~~λων", "~ᾰ~~λομενος", _;
    participle_fut: "~ᾰ~~ων", "~ᾰ~~ουμενος", "~ᾰ~~θησομενος";
    participle_aor: "~ᾰ~ι~ᾱς", "~ᾰ~ι~ᾰμενος", "~ᾰ~~θεις";
    participle_perf: "~η~~κως", "~η~~μενος", _;
    ),
    // Στεινω: 4 words (τεινω, θεινω, κτεινω, στεινω). Like Γῡροω, except in 298 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("στεινω", 4, "~εινω", like Γῡροω;
    ind_act_1: "~εινω", _, "~εινομεν";
    ind_act_2: "~εινεις", _, _;
    ind_act_3: "~εινει", _, "~εινουσῐ";
    ind_mid_1: "~εινομαι", _, "~εινομεθᾰ";
    ind_mid_2: "~εινῃ", "~εινεσθον", "~εινεσθε";
    ind_mid_3: "~εινεται", _, "~εινονται";
    ind_pass_1: _, _, "~εινομεθᾰ";
    ind_pass_2: _, _, "~εινεσθε";
    ind_pass_3: "~εινεται", _, "~εινονται";
    subj_act_1: "~εινω", _, "~εινωμεν";
    subj_act_2: "~εινῃς", "~εινητον", "~εινητε";
    subj_act_3: "~εινῃ", _, "~εινωσῐ";
    subj_mid_1: "~εινωμαι", _, "~εινωμεθᾰ";
    subj_mid_2: "~εινῃ", "~εινησθον", "~εινησθε";
    subj_mid_3: "~εινηται", _, "~εινωνται";
    subj_pass_1: _, _, "~εινωμεθᾰ";
    subj_pass_2: _, _, "~εινησθε";
    subj_pass_3: "~εινηται", _, "~εινωνται";
    opt_act_1: "~εινοιμῐ", _, "~εινοιμεν";
    opt_act_2: "~εινοις", "~εινοιτον", "~εινοιτε";
    opt_act_3: "~εινοι", "~εινοιτην", "~εινοιεν";
    opt_mid_1: "~εινοιμην", _, "~εινοιμεθᾰ";
    opt_mid_2: "~εινοιο", "~εινοισθον", "~εινοισθε";
    opt_mid_3: "~εινοιτο", "~εινοισθην", "~εινοιντο";
    opt_pass_1: _, _, "~εινοιμεθᾰ";
    opt_pass_2: _, "~εινοισθον", "~εινοισθε";
    opt_pass_3: "~εινοιτο", "~εινοισθην", "~εινοιντο";
    imp_act_2: "~εῑνε", "~εινετον", "~εινετε";
    imp_act_3: "~εινετω", "~εινετων", "~εινοντων";
    imp_mid_2: "~εινου", _, "~εινεσθε";
    imp_mid_3: "~εινεσθω", "~εινεσθων", "~εινεσθων";
    imp_pass_2: _, _, "~εινεσθε";
    imp_pass_3: _, "~εινεσθων", _;
    imperfect_act_1: "ἐ~εινον", _, "ἐ~εινομεν";
    imperfect_act_2: "ἐ~εινες", "ἐ~εινετον", "ἐ~εινετε";
    imperfect_act_3: "ἐ~εινε", "ἐ~εινετην", "ἐ~εινον";
    imperfect_mid_1: "ἐ~εινομην", _, "ἐ~εινομεθᾰ";
    imperfect_mid_2: "ἐ~εινου", "ἐ~εινεσθον", "ἐ~εινεσθε";
    imperfect_mid_3: "ἐ~εινετο", "ἐ~εινεσθην", "ἐ~εινοντο";
    imperfect_pass_1: _, _, "ἐ~εινομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~εινεσθε";
    imperfect_pass_3: "ἐ~εινετο", _, _;
    future_ind_act_1: "~ενω", _, "~ενοῡμεν";
    future_ind_act_2: "~ενεῑς", "~ενεῑτον", "~ενεῑτε";
    future_ind_act_3: "~ενεῑ", _, "~ενοῡσῐ";
    future_ind_mid_1: "~ενοῡμαι", _, "~ενουμεθᾰ";
    future_ind_mid_2: "~ενεῑ", "~ενεῑσθον", "~ενεῑσθε";
    future_ind_mid_3: "~ενεῑται", _, "~ενοῡνται";
    future_ind_pass_1: "~ᾰθησομαι", _, "~ᾰθησομεθᾰ";
    future_ind_pass_2: "~ᾰθησῃ", "~ᾰθησεσθον", "~ᾰθησεσθε";
    future_ind_pass_3: "~ᾰθησεται", _, "~ᾰθησονται";
    future_opt_act_1: "~ενοιην", _, "~ενοῑμεν";
    future_opt_act_2: "~ενοιης", "~ενοῑτον", "~ενοῑτε";
    future_opt_act_3: "~ενοιη", "~ενοιτην", "~ενοῑεν";
    future_opt_mid_1: "~ενοιμην", _, "~ενοιμεθᾰ";
    future_opt_mid_2: "~ενοῑο", "~ενοῑσθον", "~ενοῑσθε";
    future_opt_mid_3: "~ενοῑτο", "~ενοισθην", "~ενοῑντο";
    future_opt_pass_1: "~ᾰθησοιμην", _, "~ᾰθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰθησοιο", "~ᾰθησοισθον", "~ᾰθησοισθε";
    future_opt_pass_3: "~ᾰθησοιτο", "~ᾰθησοισθην", "~ᾰθησοιντο";
    aorist_ind_act_1: "ἐ~εινᾰ", _, "ἐ~εινᾰμεν";
    aorist_ind_act_2: "ἐ~εινᾰς", "ἐ~εινᾰτον", "ἐ~εινᾰτε";
    aorist_ind_act_3: "ἐ~εινε", "ἐ~εινᾰτην", "ἐ~εινᾰν";
    aorist_ind_mid_1: "ἐ~εινᾰμην", _, "ἐ~εινᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~εινω", "ἐ~εινᾰσθον", "ἐ~εινᾰσθε";
    aorist_ind_mid_3: "ἐ~εινᾰτο", "ἐ~εινᾰσθην", "ἐ~εινᾰντο";
    aorist_ind_pass_1: "ἐ~ᾰθην", _, "ἐ~ᾰθημεν";
    aorist_ind_pass_2: "ἐ~ᾰθης", "ἐ~ᾰθητον", "ἐ~ᾰθητε";
    aorist_ind_pass_3: "ἐ~ᾰθη", "ἐ~ᾰθητην", "ἐ~ᾰθησᾰν";
    aorist_subj_act_1: "~εινω", _, "~εινωμεν";
    aorist_subj_act_2: "~εινῃς", "~εινητον", "~εινητε";
    aorist_subj_act_3: "~εινῃ", _, "~εινωσῐ";
    aorist_subj_mid_1: "~εινωμαι", _, "~εινωμεθᾰ";
    aorist_subj_mid_2: "~εινῃ", "~εινησθον", "~εινησθε";
    aorist_subj_mid_3: "~εινηται", _, "~εινωνται";
    aorist_subj_pass_1: "~ᾰθω", _, "~ᾰθωμεν";
    aorist_subj_pass_2: "~ᾰθῃς", "~ᾰθητον", "~ᾰθητε";
    aorist_subj_pass_3: "~ᾰθῃ", _, "~ᾰθωσῐ";
    aorist_opt_act_1: "~ειναιμῐ", _, "~ειναιμεν";
    aorist_opt_act_2: "~εινειᾰς", "~ειναιτον", "~ειναιτε";
    aorist_opt_act_3: "~εινειε", "~ειναιτην", "~εινειᾰν";
    aorist_opt_mid_1: "~ειναιμην", _, "~ειναιμεθᾰ";
    aorist_opt_mid_2: "~ειναιο", "~ειναισθον", "~ειναισθε";
    aorist_opt_mid_3: "~ειναιτο", "~ειναισθην", "~ειναιντο";
    aorist_opt_pass_1: "~ᾰθειην", _, "~ᾰθεῑμεν";
    aorist_opt_pass_2: "~ᾰθειης", "~ᾰθεῑτον", "~ᾰθεῑτε";
    aorist_opt_pass_3: "~ᾰθειη", "~ᾰθειτην", "~ᾰθεῑεν";
    aorist_imp_act_2: "~εῑνον", "~εινᾰτον", "~εινᾰτε";
    aorist_imp_act_3: "~εινᾰτω", "~εινᾰτων", "~εινᾰντων";
    aorist_imp_mid_2: "~εῑναι", "~εινᾰσθον", "~εινᾰσθε";
    aorist_imp_mid_3: "~εινᾰσθω", "~εινᾰσθων", _;
    aorist_imp_pass_2: "~ᾰθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰθητω", "~ᾰθητων", "~ᾰθεντων";
    perfect_ind_act_1: "~ετᾰκᾰ", _, "~ετᾰκᾰμεν";
    perfect_ind_act_2: "~ετᾰκᾰς", "~ετᾰκᾰτον", "~ετᾰκᾰτε";
    perfect_ind_act_3: "~ετᾰκε", _, "~ετᾰκᾱσῐ";
    perfect_ind_mid_1: "~ετᾰμαι", _, "~ετᾰμεθᾰ";
    perfect_ind_mid_2: "~ετᾰσαι", "~ετᾰσθον", "~ετᾰσθε";
    perfect_ind_mid_3: "~ετᾰται", _, "~ετᾰνται";
    perfect_ind_pass_1: _, _, "~ετᾰμεθᾰ";
    perfect_ind_pass_2: _, _, "~ετᾰσθε";
    perfect_subj_act_1: "~ετᾰκω", _, "~ετᾰκωμεν";
    perfect_subj_act_2: "~ετᾰκῃς", "~ετᾰκητον", "~ετᾰκητε";
    perfect_subj_act_3: "~ετᾰκῃ", _, "~ετᾰκωσῐ";
    perfect_opt_act_1: "~ετᾰκοιμῐ", _, "~ετᾰκοιμεν";
    perfect_opt_act_2: "~ετᾰκοις", "~ετᾰκοιτον", "~ετᾰκοιτε";
    perfect_opt_act_3: "~ετᾰκοι", "~ετᾰκοιτην", "~ετᾰκοιεν";
    perfect_imp_act_2: _, "~ετᾰκετον", "~ετᾰκετε";
    perfect_imp_act_3: "~ετᾰκετω", "~ετᾰκετων", "~ετᾰκοντων";
    perfect_imp_mid_2: "~ετᾰσο", _, _;
    perfect_imp_mid_3: "~ετᾰσθω", "~ετᾰσθων", _;
    perfect_imp_pass_2: "~ετᾰσο", _, _;
    perfect_imp_pass_3: "~ετᾰσθω", "~ετᾰσθων", _;
    pluperfect_act_1: "~ετᾰκειν", _, "~ετᾰκεμεν";
    pluperfect_act_2: "~ετᾰκεις", "~ετᾰκετον", "~ετᾰκετε";
    pluperfect_act_3: "~ετᾰκει", "~ετᾰκετην", "~ετᾰκεσᾰν";
    pluperfect_mid_1: "~ετᾰμην", _, "~ετᾰμεθᾰ";
    pluperfect_mid_2: "~ετᾰσο", "~ετᾰσθον", "~ετᾰσθε";
    pluperfect_mid_3: "~ετᾰτο", "~ετᾰσθην", _;
    pluperfect_pass_1: _, _, "~ετᾰμεθᾰ";
    pluperfect_pass_2: _, _, "~ετᾰσθε";
    pluperfect_pass_3: _, "~ετᾰσθην", "~ετᾰντο";
    infinitive_pres: "~εινειν", "~εινεσθαι", _;
    infinitive_fut: "~ενεῑν", "~ενεῑσθαι", "~ᾰθησεσθαι";
    infinitive_aor: "~εῑναι", "~εινᾰσθαι", "~ᾰθηναι";
    infinitive_perf: "~ετᾰκεναι", "~ετᾰσθαι", _;
    participle_pres: "~εινων", "~εινομενος", _;
    participle_fut: "~ενων", "~ενουμενος", "~ᾰθησομενος";
    participle_aor: "~εινᾱς", "~εινᾰμενος", "~ᾰθεις";
    participle_perf: "~ετᾰκως", "~ετᾰμενος", _;
    ),
    // Ἀνᾰπτω: 4 words (ἀνᾰπτω, ἐξᾰπτω, καθᾰπτω, ἐπῐσῠνᾰπτω). Like Ἑπομαι, except in 262 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἀνᾰπτω", 4, "~ᾰπτω", like Ἑπομαι;
    ind_act_1: "~ᾰπτω", _, "~ᾰπτομεν";
    ind_act_2: "~ᾰπτεις", _, _;
    ind_act_3: "~ᾰπτει", _, "~ᾰπτουσῐ|~ᾰπτουσῐν";
    ind_mid_1: "~ᾰπτομαι", _, "~ᾰπτομεθᾰ";
    ind_mid_2: "~ᾰπτῃ|~ᾰπτει", "~ᾰπτεσθον", "~ᾰπτεσθε";
    ind_mid_3: "~ᾰπτεται", _, "~ᾰπτονται";
    ind_pass_1: _, _, "~ᾰπτομεθᾰ";
    ind_pass_2: "~ᾰπτῃ|~ᾰπτει", _, "~ᾰπτεσθε";
    ind_pass_3: "~ᾰπτεται", _, "~ᾰπτονται";
    subj_act_1: "~ᾰπτω", _, "~ᾰπτωμεν";
    subj_act_2: "~ᾰπτῃς", "~ᾰπτητον", "~ᾰπτητε";
    subj_act_3: "~ᾰπτῃ", _, "~ᾰπτωσῐ|~ᾰπτωσῐν";
    subj_mid_1: "~ᾰπτωμαι", _, "~ᾰπτωμεθᾰ";
    subj_mid_2: "~ᾰπτῃ", "~ᾰπτησθον", "~ᾰπτησθε";
    subj_mid_3: "~ᾰπτηται", _, "~ᾰπτωνται";
    subj_pass_1: _, _, "~ᾰπτωμεθᾰ";
    subj_pass_2: _, _, "~ᾰπτησθε";
    subj_pass_3: "~ᾰπτηται", _, "~ᾰπτωνται";
    opt_act_1: "~ᾰπτοιμῐ", _, "~ᾰπτοιμεν";
    opt_act_2: "~ᾰπτοις", "~ᾰπτοιτον", "~ᾰπτοιτε";
    opt_act_3: "~ᾰπτοι", "~ᾰπτοιτην", "~ᾰπτοιεν";
    opt_mid_1: "~ᾰπτοιμην", _, "~ᾰπτοιμεθᾰ";
    opt_mid_2: "~ᾰπτοιο", "~ᾰπτοισθον", "~ᾰπτοισθε";
    opt_mid_3: "~ᾰπτοιτο", "~ᾰπτοισθην", "~ᾰπτοιντο";
    opt_pass_1: _, _, "~ᾰπτοιμεθᾰ";
    opt_pass_2: _, "~ᾰπτοισθον", "~ᾰπτοισθε";
    opt_pass_3: "~ᾰπτοιτο", "~ᾰπτοισθην", "~ᾰπτοιντο";
    imp_act_2: "~ᾰπτε", "~ᾰπτετον", "~ᾰπτετε";
    imp_act_3: "~ᾰπτετω", "~ᾰπτετων", "~ᾰπτοντων";
    imp_mid_2: "~ᾰπτου", _, "~ᾰπτεσθε";
    imp_mid_3: "~ᾰπτεσθω", "~ᾰπτεσθων", "~ᾰπτεσθων";
    imp_pass_2: _, _, "~ᾰπτεσθε";
    imp_pass_3: _, "~ᾰπτεσθων", _;
    imperfect_act_1: "~ηπτον", _, "~ηπτομεν";
    imperfect_act_2: "~ηπτες", "~ηπτετον", "~ηπτετε";
    imperfect_act_3: "~ηπτε|~ηπτεν", "~ηπτετην", "~ηπτον";
    imperfect_mid_1: "~ηπτομην", _, "~ηπτομεθᾰ";
    imperfect_mid_2: "~ηπτου", "~ηπτεσθον", "~ηπτεσθε";
    imperfect_mid_3: "~ηπτετο", "~ηπτεσθην", "~ηπτοντο";
    imperfect_pass_1: _, _, "~ηπτομεθᾰ";
    imperfect_pass_2: _, _, "~ηπτεσθε";
    imperfect_pass_3: "~ηπτετο", _, _;
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
    aorist_ind_act_1: "~ηψᾰ", _, "~ηψᾰμεν";
    aorist_ind_act_2: "~ηψᾰς", "~ηψᾰτον", "~ηψᾰτε";
    aorist_ind_act_3: "~ηψε|~ηψεν", "~ηψᾰτην", "~ηψᾰν";
    aorist_ind_mid_1: "~ηψᾰμην", _, "~ηψᾰμεθᾰ";
    aorist_ind_mid_2: "~ηψω", "~ηψᾰσθον", "~ηψᾰσθε";
    aorist_ind_mid_3: "~ηψᾰτο", "~ηψᾰσθην", "~ηψᾰντο";
    aorist_ind_pass_1: "~ηφθην", _, "~ηφθημεν";
    aorist_ind_pass_2: "~ηφθης", "~ηφθητον", "~ηφθητε";
    aorist_ind_pass_3: "~ηφθη", "~ηφθητην", "~ηφθησᾰν";
    aorist_subj_act_1: "~ᾰψω", _, "~ᾰψωμεν";
    aorist_subj_act_2: "~ᾰψῃς", "~ᾰψητον", "~ᾰψητε";
    aorist_subj_act_3: "~ᾰψῃ", _, "~ᾰψωσῐ|~ᾰψωσῐν";
    aorist_subj_mid_1: "~ᾰψωμαι", _, "~ᾰψωμεθᾰ";
    aorist_subj_mid_2: "~ᾰψῃ", "~ᾰψησθον", "~ᾰψησθε";
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
    perfect_ind_pass_1: _, _, "~ημμεθᾰ";
    perfect_ind_pass_2: _, _, "~ηφθε";
    perfect_imp_mid_2: "~ηψο", _, _;
    perfect_imp_mid_3: "~ηφθω", "~ηφθων", _;
    perfect_imp_pass_2: "~ηψο", _, _;
    perfect_imp_pass_3: "~ηφθω", "~ηφθων", _;
    pluperfect_mid_1: "~ημμην", _, "~ημμεθᾰ";
    pluperfect_mid_2: "~ηψο", "~ηφθον", "~ηφθε";
    pluperfect_mid_3: "~ηπτο", "~ηφθην", _;
    pluperfect_pass_1: _, _, "~ημμεθᾰ";
    pluperfect_pass_2: _, _, "~ηφθε";
    pluperfect_pass_3: _, "~ηφθην", "~ηπᾰτο";
    infinitive_pres: "~ᾰπτειν", "~ᾰπτεσθαι", _;
    infinitive_fut: "~ᾰψειν", "~ᾰψεσθαι", "~ᾰφθησεσθαι";
    infinitive_aor: "~ᾰψαι", "~ᾰψᾰσθαι", "~ᾰφθηναι";
    infinitive_perf: _, "~ηφθαι", _;
    participle_pres: "~ᾰπτων", "~ᾰπτομενος", _;
    participle_fut: "~ᾰψων", "~ᾰψομενος", "~ᾰφθησομενος";
    participle_aor: "~ᾰψᾱς", "~ᾰψᾰμενος", "~ᾰφθεις";
    participle_perf: _, "~ημμενος", _;
    ),
    // Πᾰρᾰλλᾰσσω: 4 words (ἀπᾰλλᾰσσω, ὑπᾰλλᾰσσω, ἀντᾰλλᾰσσω, πᾰρᾰλλᾰσσω). Like Προσερχομαι, except in 284 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("πᾰρᾰλλᾰσσω", 4, "~ᾰ~σσω", like Προσερχομαι;
    ind_act_1: "~ᾰ~σσω", _, "~ᾰ~σσομεν";
    ind_act_2: "~ᾰ~σσεις", _, _;
    ind_act_3: "~ᾰ~σσει", _, "~ᾰ~σσουσῐ|~ᾰ~σσουσῐν";
    ind_mid_1: "~ᾰ~σσομαι", _, "~ᾰ~σσομεθᾰ";
    ind_mid_2: "~ᾰ~σσῃ|~ᾰ~σσει", "~ᾰ~σσεσθον", "~ᾰ~σσεσθε";
    ind_mid_3: "~ᾰ~σσεται", _, "~ᾰ~σσονται";
    ind_pass_1: _, _, "~ᾰ~σσομεθᾰ";
    ind_pass_2: "~ᾰ~σσῃ|~ᾰ~σσει", _, "~ᾰ~σσεσθε";
    ind_pass_3: "~ᾰ~σσεται", _, "~ᾰ~σσονται";
    subj_act_1: "~ᾰ~σσω", _, "~ᾰ~σσωμεν";
    subj_act_2: "~ᾰ~σσῃς", "~ᾰ~σσητον", "~ᾰ~σσητε";
    subj_act_3: "~ᾰ~σσῃ", _, "~ᾰ~σσωσῐ|~ᾰ~σσωσῐν";
    subj_mid_1: "~ᾰ~σσωμαι", _, "~ᾰ~σσωμεθᾰ";
    subj_mid_2: "~ᾰ~σσῃ", "~ᾰ~σσησθον", "~ᾰ~σσησθε";
    subj_mid_3: "~ᾰ~σσηται", _, "~ᾰ~σσωνται";
    subj_pass_1: _, _, "~ᾰ~σσωμεθᾰ";
    subj_pass_2: _, _, "~ᾰ~σσησθε";
    subj_pass_3: "~ᾰ~σσηται", _, "~ᾰ~σσωνται";
    opt_act_1: "~ᾰ~σσοιμῐ", _, "~ᾰ~σσοιμεν";
    opt_act_2: "~ᾰ~σσοις", "~ᾰ~σσοιτον", "~ᾰ~σσοιτε";
    opt_act_3: "~ᾰ~σσοι", "~ᾰ~σσοιτην", "~ᾰ~σσοιεν";
    opt_mid_1: "~ᾰ~σσοιμην", _, "~ᾰ~σσοιμεθᾰ";
    opt_mid_2: "~ᾰ~σσοιο", "~ᾰ~σσοισθον", "~ᾰ~σσοισθε";
    opt_mid_3: "~ᾰ~σσοιτο", "~ᾰ~σσοισθην", "~ᾰ~σσοιντο";
    opt_pass_1: _, _, "~ᾰ~σσοιμεθᾰ";
    opt_pass_2: _, "~ᾰ~σσοισθον", "~ᾰ~σσοισθε";
    opt_pass_3: "~ᾰ~σσοιτο", "~ᾰ~σσοισθην", "~ᾰ~σσοιντο";
    imp_act_2: "~ᾰ~σσε", "~ᾰ~σσετον", "~ᾰ~σσετε";
    imp_act_3: "~ᾰ~σσετω", "~ᾰ~σσετων", "~ᾰ~σσοντων";
    imp_mid_2: "~ᾰ~σσου", _, "~ᾰ~σσεσθε";
    imp_mid_3: "~ᾰ~σσεσθω", "~ᾰ~σσεσθων", "~ᾰ~σσεσθων";
    imp_pass_2: _, _, "~ᾰ~σσεσθε";
    imp_pass_3: _, "~ᾰ~σσεσθων", _;
    imperfect_act_1: "~η~σσον", _, "~η~σσομεν";
    imperfect_act_2: "~η~σσες", "~η~σσετον", "~η~σσετε";
    imperfect_act_3: "~η~σσε|~η~σσεν", "~η~σσετην", "~η~σσον";
    imperfect_mid_1: "~η~σσομην", _, "~η~σσομεθᾰ";
    imperfect_mid_2: "~η~σσου", "~η~σσεσθον", "~η~σσεσθε";
    imperfect_mid_3: "~η~σσετο", "~η~σσεσθην", "~η~σσοντο";
    imperfect_pass_1: _, _, "~η~σσομεθᾰ";
    imperfect_pass_2: _, _, "~η~σσεσθε";
    imperfect_pass_3: "~η~σσετο", _, _;
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
    aorist_ind_act_1: "~η~ξᾰ", _, "~η~ξᾰμεν";
    aorist_ind_act_2: "~η~ξᾰς", "~η~ξᾰτον", "~η~ξᾰτε";
    aorist_ind_act_3: "~η~ξε", "~η~ξᾰτην", "~η~ξᾰν";
    aorist_ind_mid_1: "~η~ξᾰμην", _, "~η~ξᾰμεθᾰ";
    aorist_ind_mid_2: "~η~ξω", "~η~ξᾰσθον", "~η~ξᾰσθε";
    aorist_ind_mid_3: "~η~ξᾰτο", "~η~ξᾰσθην", "~η~ξᾰντο";
    aorist_ind_pass_1: "~η~χθην|~η~γην", _, "~η~χθημεν|~η~γημεν";
    aorist_ind_pass_2: "~η~χθης|~η~γης", "~η~χθητον|~η~γητον", "~η~χθητε|~η~γητε";
    aorist_ind_pass_3: "~η~χθη|~η~γη", "~η~χθητην|~η~γητην", "~η~χθησᾰν|~η~γησᾰν";
    aorist_subj_act_1: "~ᾰ~ξω", _, "~ᾰ~ξωμεν";
    aorist_subj_act_2: "~ᾰ~ξῃς", "~ᾰ~ξητον", "~ᾰ~ξητε";
    aorist_subj_act_3: "~ᾰ~ξῃ", _, "~ᾰ~ξωσῐ";
    aorist_subj_mid_1: "~ᾰ~ξωμαι", _, "~ᾰ~ξωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~ξῃ", "~ᾰ~ξησθον", "~ᾰ~ξησθε";
    aorist_subj_mid_3: "~ᾰ~ξηται", _, "~ᾰ~ξωνται";
    aorist_subj_pass_1: "~ᾰ~χθω|~ᾰ~γω", _, "~ᾰ~χθωμεν|~ᾰ~γωμεν";
    aorist_subj_pass_2: "~ᾰ~χθῃς|~ᾰ~γῃς", "~ᾰ~χθητον|~ᾰ~γητον", "~ᾰ~χθητε|~ᾰ~γητε";
    aorist_subj_pass_3: "~ᾰ~χθῃ|~ᾰ~γῃ", "~ᾰ~χθητον|~ᾰ~γητον", "~ᾰ~χθωσῐ|~ᾰ~χθωσῐν|~ᾰ~γωσῐ|~ᾰ~γωσῐν";
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
    aorist_imp_pass_2: "~ᾰ~χθητῐ|~ᾰ~γηθῐ", "~ᾰ~χθητον|~ᾰ~γητον", "~ᾰ~χθητε|~ᾰ~γητε";
    aorist_imp_pass_3: "~ᾰ~χθητω|~ᾰ~γητω", "~ᾰ~χθητων|~ᾰ~γητων", "~ᾰ~χθεντων|~ᾰ~γεντων";
    perfect_ind_act_1: "~η~χᾰ", _, "~η~χᾰμεν";
    perfect_ind_act_2: "~η~χᾰς", "~η~χᾰτον", "~η~χᾰτε";
    perfect_ind_act_3: "~η~χε|~η~χεν", _, "~η~χᾱσῐ|~η~χᾱσῐν";
    perfect_ind_mid_1: "~η~γμαι", _, "~η~γμεθᾰ";
    perfect_ind_mid_2: "~η~ξαι", "~η~χθον", "~η~χθε";
    perfect_ind_mid_3: "~η~κται", _, "~η~χᾰται";
    perfect_ind_pass_1: _, _, "~η~γμεθᾰ";
    perfect_ind_pass_2: _, _, "~η~χθε";
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
    perfect_imp_pass_2: "~η~ξο", _, _;
    perfect_imp_pass_3: "~η~χθω", "~η~χθων", _;
    infinitive_pres: "~ᾰ~σσειν", "~ᾰ~σσεσθαι", _;
    infinitive_fut: "~ᾰ~ξειν", "~ᾰ~ξεσθαι", "~ᾰ~χθησεσθαι";
    infinitive_aor: "~ᾰ~ξαι", "~ᾰ~ξᾰσθαι", "~ᾰ~χθηναι|~ᾰ~γηναι";
    infinitive_perf: "~η~χεναι", "~η~χθαι", _;
    participle_pres: "~ᾰ~σσων", "~ᾰ~σσομενος", _;
    participle_fut: "~ᾰ~ξων", "~ᾰ~ξομενος", "~ᾰ~χθησομενος";
    participle_aor: "~ᾰ~ξᾱς", "~ᾰ~ξᾰμενος", "~ᾰ~χθεις|~ᾰ~γεις";
    participle_perf: "~η~χως", "~η~γμενος", _;
    ),
    // Κᾰτειργω: 4 words (ἐργω, εἰργω, ἀμεργω, κᾰτειργω). Like Ληγω, except in 77 cells: ind_act_3 pl, ind_mid_2 sg, ind_pass_2 sg, subj_act_3 pl, imperfect_act_1 sg, imperfect_act_1 pl, ….
    conjugation!("κᾰτειργω", 4, "~γω", like Ληγω;
    ind_act_3: _, _, "~γουσῐ|~γουσῐν";
    ind_mid_2: "~γῃ|~γει", _, _;
    ind_pass_2: "~γῃ|~γει", _, _;
    subj_act_3: _, _, "~γωσῐ|~γωσῐν";
    imperfect_act_1: "~γον", _, "~γομεν";
    imperfect_act_2: "~γες", "~γετον", "~γετε";
    imperfect_act_3: "~γε", "~γετην", "~γον";
    imperfect_mid_1: "~γομην", _, "~γομεθᾰ";
    imperfect_mid_2: "~γου", "~γεσθον", "~γεσθε";
    imperfect_mid_3: "~γετο", "~γεσθην", "~γοντο";
    imperfect_pass_1: _, _, "~γομεθᾰ";
    imperfect_pass_2: _, _, "~γεσθε";
    imperfect_pass_3: "~γετο", _, _;
    future_ind_act_3: _, _, "~ξουσῐ|~ξουσῐν";
    future_ind_mid_2: "~ξῃ|~ξει", _, _;
    aorist_ind_act_1: "~ξᾰ", _, "~ξᾰμεν";
    aorist_ind_act_2: "~ξᾰς", "~ξᾰτον", "~ξᾰτε";
    aorist_ind_act_3: "~ξε", "~ξᾰτην", "~ξᾰν";
    aorist_ind_mid_1: "~ξᾰμην", _, "~ξᾰμεθᾰ";
    aorist_ind_mid_2: "~ξω", "~ξᾰσθον", "~ξᾰσθε";
    aorist_ind_mid_3: "~ξᾰτο", "~ξᾰσθην", "~ξᾰντο";
    aorist_ind_pass_1: "~χθην", _, "~χθημεν";
    aorist_ind_pass_2: "~χθης", "~χθητον", "~χθητε";
    aorist_ind_pass_3: "~χθη", "~χθητην", "~χθησᾰν";
    perfect_ind_mid_1: "~γμαι", _, "~γμεθᾰ";
    perfect_ind_mid_2: "~ξαι", "~χθον", "~χθε";
    perfect_ind_mid_3: "~κται", _, "~χᾰται";
    perfect_ind_pass_1: _, _, "~γμεθᾰ";
    perfect_ind_pass_2: _, _, "~χθε";
    perfect_imp_mid_2: "~ξο", _, _;
    perfect_imp_mid_3: "~χθω", "~χθων", _;
    perfect_imp_pass_2: "~ξο", _, _;
    perfect_imp_pass_3: "~χθω", "~χθων", _;
    pluperfect_mid_1: "~γμην", _, "~γμεθᾰ";
    pluperfect_mid_2: "~ξο", "~χθον", "~χθε";
    pluperfect_mid_3: "~κτο", "~χθην", _;
    pluperfect_pass_1: _, _, "~γμεθᾰ";
    pluperfect_pass_2: _, _, "~χθε";
    pluperfect_pass_3: _, "~χθην", "~χᾰτο";
    infinitive_perf: _, "~χθαι", _;
    participle_perf: _, "~γμενος", _;
    ),
    // Πᾰροξῡνω: 4 words (παροτρῡνω, ἐποτρῡνω, πᾰροξῡνω, ἐξοτρῡνω). Like Προσερχομαι, except in 279 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("πᾰροξῡνω", 4, "~ο~ῡνω", like Προσερχομαι;
    ind_act_1: "~ο~ῡνω", _, "~ο~ῡνομεν";
    ind_act_2: "~ο~ῡνεις", _, _;
    ind_act_3: "~ο~ῡνει", _, "~ο~ῡνουσῐ";
    ind_mid_1: "~ο~ῡνομαι", _, "~ο~ῡνομεθᾰ";
    ind_mid_2: "~ο~ῡνῃ", "~ο~ῡνεσθον", "~ο~ῡνεσθε";
    ind_mid_3: "~ο~ῡνεται", _, "~ο~ῡνονται";
    ind_pass_1: _, _, "~ο~ῡνομεθᾰ";
    ind_pass_2: _, _, "~ο~ῡνεσθε";
    ind_pass_3: "~ο~ῡνεται", _, "~ο~ῡνονται";
    subj_act_1: "~ο~ῡνω", _, "~ο~ῡνωμεν";
    subj_act_2: "~ο~ῡνῃς", "~ο~ῡνητον", "~ο~ῡνητε";
    subj_act_3: "~ο~ῡνῃ", _, "~ο~ῡνωσῐ";
    subj_mid_1: "~ο~ῡνωμαι", _, "~ο~ῡνωμεθᾰ";
    subj_mid_2: "~ο~ῡνῃ", "~ο~ῡνησθον", "~ο~ῡνησθε";
    subj_mid_3: "~ο~ῡνηται", _, "~ο~ῡνωνται";
    subj_pass_1: _, _, "~ο~ῡνωμεθᾰ";
    subj_pass_2: _, _, "~ο~ῡνησθε";
    subj_pass_3: "~ο~ῡνηται", _, "~ο~ῡνωνται";
    opt_act_1: "~ο~ῡνοιμῐ", _, "~ο~ῡνοιμεν";
    opt_act_2: "~ο~ῡνοις", "~ο~ῡνοιτον", "~ο~ῡνοιτε";
    opt_act_3: "~ο~ῡνοι", "~ο~ῡνοιτην", "~ο~ῡνοιεν";
    opt_mid_1: "~ο~ῡνοιμην", _, "~ο~ῡνοιμεθᾰ";
    opt_mid_2: "~ο~ῡνοιο", "~ο~ῡνοισθον", "~ο~ῡνοισθε";
    opt_mid_3: "~ο~ῡνοιτο", "~ο~ῡνοισθην", "~ο~ῡνοιντο";
    opt_pass_1: _, _, "~ο~ῡνοιμεθᾰ";
    opt_pass_2: _, "~ο~ῡνοισθον", "~ο~ῡνοισθε";
    opt_pass_3: "~ο~ῡνοιτο", "~ο~ῡνοισθην", "~ο~ῡνοιντο";
    imp_act_2: "~ο~ῡνε", "~ο~ῡνετον", "~ο~ῡνετε";
    imp_act_3: "~ο~ῡνετω", "~ο~ῡνετων", "~ο~ῡνοντων";
    imp_mid_2: "~ο~ῡνου", _, "~ο~ῡνεσθε";
    imp_mid_3: "~ο~ῡνεσθω", "~ο~ῡνεσθων", "~ο~ῡνεσθων";
    imp_pass_2: _, _, "~ο~ῡνεσθε";
    imp_pass_3: _, "~ο~ῡνεσθων", _;
    imperfect_act_1: "~ω~ῡνον", _, "~ω~ῡνομεν";
    imperfect_act_2: "~ω~ῡνες", "~ω~ῡνετον", "~ω~ῡνετε";
    imperfect_act_3: "~ω~ῡνε", "~ω~ῡνετην", "~ω~ῡνον";
    imperfect_mid_1: "~ω~ῡνομην", _, "~ω~ῡνομεθᾰ";
    imperfect_mid_2: "~ω~ῡνου", "~ω~ῡνεσθον", "~ω~ῡνεσθε";
    imperfect_mid_3: "~ω~ῡνετο", "~ω~ῡνεσθην", "~ω~ῡνοντο";
    imperfect_pass_1: _, _, "~ω~ῡνομεθᾰ";
    imperfect_pass_2: _, _, "~ω~ῡνεσθε";
    imperfect_pass_3: "~ω~ῡνετο", _, _;
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
    aorist_ind_act_1: "~ω~ῡνᾰ", _, "~ω~ῡνᾰμεν";
    aorist_ind_act_2: "~ω~ῡνᾰς", "~ω~ῡνᾰτον", "~ω~ῡνᾰτε";
    aorist_ind_act_3: "~ω~ῡνε", "~ω~ῡνᾰτην", "~ω~ῡνᾰν";
    aorist_ind_mid_1: "~ω~ῡνᾰμην", _, "~ω~ῡνᾰμεθᾰ";
    aorist_ind_mid_2: "~ω~ῡνω", "~ω~ῡνᾰσθον", "~ω~ῡνᾰσθε";
    aorist_ind_mid_3: "~ω~ῡνᾰτο", "~ω~ῡνᾰσθην", "~ω~ῡνᾰντο";
    aorist_ind_pass_1: "~ω~ῠνθην", _, "~ω~ῠνθημεν";
    aorist_ind_pass_2: "~ω~ῠνθης", "~ω~ῠνθητον", "~ω~ῠνθητε";
    aorist_ind_pass_3: "~ω~ῠνθη", "~ω~ῠνθητην", "~ω~ῠνθησᾰν";
    aorist_subj_act_1: "~ο~ῡνω", _, "~ο~ῡνωμεν";
    aorist_subj_act_2: "~ο~ῡνῃς", "~ο~ῡνητον", "~ο~ῡνητε";
    aorist_subj_act_3: "~ο~ῡνῃ", _, "~ο~ῡνωσῐ";
    aorist_subj_mid_1: "~ο~ῡνωμαι", _, "~ο~ῡνωμεθᾰ";
    aorist_subj_mid_2: "~ο~ῡνῃ", "~ο~ῡνησθον", "~ο~ῡνησθε";
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
    perfect_ind_pass_1: _, _, "~ω~ῠμμεθᾰ";
    perfect_ind_pass_2: _, _, "~ω~ῠφθε";
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
    perfect_imp_pass_2: "~ω~ῠψο", _, _;
    perfect_imp_pass_3: "~ω~ῠφθω", "~ω~ῠφθων", _;
    infinitive_pres: "~ο~ῡνειν", "~ο~ῡνεσθαι", _;
    infinitive_fut: "~ο~ῠνεῑν", "~ο~ῠνεῑσθαι", "~ο~ῠνθησεσθαι";
    infinitive_aor: "~ο~ῡναι", "~ο~ῡνᾰσθαι", "~ο~ῠνθηναι";
    infinitive_perf: "~ω~ῠγκεναι", "~ω~ῠφθαι", _;
    participle_pres: "~ο~ῡνων", "~ο~ῡνομενος", _;
    participle_fut: "~ο~ῠνων", "~ο~ῠνουμενος", "~ο~ῠνθησομενος";
    participle_aor: "~ο~ῡνᾱς", "~ο~ῡνᾰμενος", "~ο~ῠνθεις";
    participle_perf: "~ω~ῠγκως", "~ω~ῠμμενος", _;
    ),
    // Λογεομαι: 4 words (δηλεομαι, μῑμεομαι, δῐηγεομαι, λογεομαι). Like Ἡγεομαι, except in 58 cells: ind_mid_2 sg, ind_pass_2 sg, imperfect_mid_1 sg, imperfect_mid_1 pl, imperfect_mid_2 sg, imperfect_mid_2 du, ….
    conjugation!("λογεομαι", 4, "~εομαι", like Ἡγεομαι;
    ind_mid_2: "~εῑ|~ῃ", _, _;
    ind_pass_2: "~εῑ", _, _;
    imperfect_mid_1: "ἐ~ουμην", _, "ἐ~ουμεθᾰ";
    imperfect_mid_2: "ἐ~οῡ", "ἐ~εῑσθον", "ἐ~εῑσθε";
    imperfect_mid_3: "ἐ~εῑτο", "ἐ~εισθην", "ἐ~οῡντο";
    imperfect_pass_1: _, _, "ἐ~ουμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~εῑσθε";
    imperfect_pass_3: "ἐ~εῑτο", _, _;
    future_ind_mid_2: "~ησῃ|~ησει", _, _;
    aorist_ind_mid_1: "ἐ~ησᾰμην", _, "ἐ~ησᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ησω", "ἐ~ησᾰσθον", "ἐ~ησᾰσθε";
    aorist_ind_mid_3: "ἐ~ησᾰτο", "ἐ~ησᾰσθην", "ἐ~ησᾰντο";
    aorist_ind_pass_1: "ἐ~ηθην", _, "ἐ~ηθημεν";
    aorist_ind_pass_2: "ἐ~ηθης", "ἐ~ηθητον", "ἐ~ηθητε";
    aorist_ind_pass_3: "ἐ~ηθη", "ἐ~ηθητην", "ἐ~ηθησᾰν";
    perfect_ind_mid_1: "%ε~ημαι", _, "%ε~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ησαι", "%ε~ησθον", "%ε~ησθε";
    perfect_ind_mid_3: "%ε~ηται", _, "%ε~ηνται";
    perfect_ind_pass_1: _, _, "%ε~ημεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~ησθε";
    perfect_imp_mid_2: "%ε~ησο", _, _;
    perfect_imp_mid_3: "%ε~ησθω", "%ε~ησθων", _;
    perfect_imp_pass_2: "%ε~ησο", _, _;
    perfect_imp_pass_3: "%ε~ησθω", "%ε~ησθων", _;
    pluperfect_mid_1: "ἐ%ε~ημην", _, "ἐ%ε~ημεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ησο", "ἐ%ε~ησθον", "ἐ%ε~ησθε";
    pluperfect_mid_3: "ἐ%ε~ητο", "ἐ%ε~ησθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~ησθην", "ἐ%ε~ηντο";
    infinitive_perf: _, "%ε~ησθαι", _;
    participle_perf: _, "%ε~ημενος", _;
    ),
    // Ἀκουω: 3 words (ἀγρευω, ἀμορβευω, ἀπῐστευω).
    conjugation!("ἀκουω", 3, "ἀ~υω";
    ind_act_1: "ἀ~υω", _, "ἀ~υομεν";
    ind_act_2: "ἀ~υεις", _, _;
    ind_act_3: "ἀ~υει", _, "ἀ~υουσῐ|ἀ~υουσῐν";
    ind_mid_1: "ἀ~υομαι", _, "ἀ~υομεθᾰ";
    ind_mid_2: "ἀ~υῃ|ἀ~υει", "ἀ~υεσθον", "ἀ~υεσθε";
    ind_mid_3: "ἀ~υεται", _, "ἀ~υονται";
    ind_pass_1: _, _, "ἀ~υομεθᾰ";
    ind_pass_2: "ἀ~υῃ|ἀ~υει", _, "ἀ~υεσθε";
    ind_pass_3: "ἀ~υεται", _, "ἀ~υονται";
    subj_act_1: "ἀ~υω", _, "ἀ~υωμεν";
    subj_act_2: "ἀ~υῃς", "ἀ~υητον", "ἀ~υητε";
    subj_act_3: "ἀ~υῃ", _, "ἀ~υωσῐ|ἀ~υωσῐν";
    subj_mid_1: "ἀ~υωμαι", _, "ἀ~υωμεθᾰ";
    subj_mid_2: "ἀ~υῃ", "ἀ~υησθον", "ἀ~υησθε";
    subj_mid_3: "ἀ~υηται", _, "ἀ~υωνται";
    subj_pass_1: _, _, "ἀ~υωμεθᾰ";
    subj_pass_2: _, _, "ἀ~υησθε";
    subj_pass_3: "ἀ~υηται", _, "ἀ~υωνται";
    opt_act_1: "ἀ~υοιμῐ", _, "ἀ~υοιμεν";
    opt_act_2: "ἀ~υοις", "ἀ~υοιτον", "ἀ~υοιτε";
    opt_act_3: "ἀ~υοι", "ἀ~υοιτην", "ἀ~υοιεν";
    opt_mid_1: "ἀ~υοιμην", _, "ἀ~υοιμεθᾰ";
    opt_mid_2: "ἀ~υοιο", "ἀ~υοισθον", "ἀ~υοισθε";
    opt_mid_3: "ἀ~υοιτο", "ἀ~υοισθην", "ἀ~υοιντο";
    opt_pass_1: _, _, "ἀ~υοιμεθᾰ";
    opt_pass_2: _, "ἀ~υοισθον", "ἀ~υοισθε";
    opt_pass_3: "ἀ~υοιτο", "ἀ~υοισθην", "ἀ~υοιντο";
    imp_act_2: "ἀ~υε", "ἀ~υετον", "ἀ~υετε";
    imp_act_3: "ἀ~υετω", "ἀ~υετων", "ἀ~υοντων";
    imp_mid_2: "ἀ~υου", _, "ἀ~υεσθε";
    imp_mid_3: "ἀ~υεσθω", "ἀ~υεσθων", "ἀ~υεσθων";
    imp_pass_2: _, _, "ἀ~υεσθε";
    imp_pass_3: _, "ἀ~υεσθων", _;
    imperfect_act_1: "ἠ~υον", _, "ἠ~υομεν";
    imperfect_act_2: "ἠ~υες", "ἠ~υετον", "ἠ~υετε";
    imperfect_act_3: "ἠ~υε|ἠ~υεν", "ἠ~υετην", "ἠ~υον";
    imperfect_mid_1: "ἠ~υομην", _, "ἠ~υομεθᾰ";
    imperfect_mid_2: "ἠ~υου", "ἠ~υεσθον", "ἠ~υεσθε";
    imperfect_mid_3: "ἠ~υετο", "ἠ~υεσθην", "ἠ~υοντο";
    imperfect_pass_1: _, _, "ἠ~υομεθᾰ";
    imperfect_pass_2: _, _, "ἠ~υεσθε";
    imperfect_pass_3: "ἠ~υετο", _, _;
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
    aorist_ind_act_1: "ἠ~υσᾰ", _, "ἠ~υσᾰμεν";
    aorist_ind_act_2: "ἠ~υσᾰς", "ἠ~υσᾰτον", "ἠ~υσᾰτε";
    aorist_ind_act_3: "ἠ~υσε|ἠ~υσεν", "ἠ~υσᾰτην", "ἠ~υσᾰν";
    aorist_ind_mid_1: "ἠ~υσᾰμην", _, "ἠ~υσᾰμεθᾰ";
    aorist_ind_mid_2: "ἠ~υσω", "ἠ~υσᾰσθον", "ἠ~υσᾰσθε";
    aorist_ind_mid_3: "ἠ~υσᾰτο", "ἠ~υσᾰσθην", "ἠ~υσᾰντο";
    aorist_ind_pass_1: "ἠ~υθην", _, "ἠ~υθημεν";
    aorist_ind_pass_2: "ἠ~υθης", "ἠ~υθητον", "ἠ~υθητε";
    aorist_ind_pass_3: "ἠ~υθη", "ἠ~υθητην", "ἠ~υθησᾰν";
    aorist_subj_act_1: "ἀ~υσω", _, "ἀ~υσωμεν";
    aorist_subj_act_2: "ἀ~υσῃς", "ἀ~υσητον", "ἀ~υσητε";
    aorist_subj_act_3: "ἀ~υσῃ", _, "ἀ~υσωσῐ|ἀ~υσωσῐν";
    aorist_subj_mid_1: "ἀ~υσωμαι", _, "ἀ~υσωμεθᾰ";
    aorist_subj_mid_2: "ἀ~υσῃ", "ἀ~υσησθον", "ἀ~υσησθε";
    aorist_subj_mid_3: "ἀ~υσηται", _, "ἀ~υσωνται";
    aorist_subj_pass_1: "ἀ~υθω", _, "ἀ~υθωμεν";
    aorist_subj_pass_2: "ἀ~υθῃς", "ἀ~υθητον", "ἀ~υθητε";
    aorist_subj_pass_3: "ἀ~υθῃ", _, "ἀ~υθωσῐ";
    aorist_opt_act_1: "ἀ~υσαιμῐ", _, "ἀ~υσαιμεν";
    aorist_opt_act_2: "ἀ~υσειᾰς|ἀ~υσαις", "ἀ~υσαιτον", "ἀ~υσαιτε";
    aorist_opt_act_3: "ἀ~υσειε|ἀ~υσειεν|ἀ~υσαι", "ἀ~υσαιτην", "ἀ~υσειᾰν|ἀ~υσαιεν";
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
    perfect_ind_pass_1: _, _, "ἠ~υμεθᾰ";
    perfect_ind_pass_2: _, _, "ἠ~υσθε";
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
    perfect_imp_pass_2: "ἠ~υσο", _, _;
    perfect_imp_pass_3: "ἠ~υσθω", "ἠ~υσθων", _;
    pluperfect_act_1: "ἠ~υκειν", _, "ἠ~υκεμεν";
    pluperfect_act_2: "ἠ~υκεις", "ἠ~υκετον", "ἠ~υκετε";
    pluperfect_act_3: "ἠ~υκει", "ἠ~υκετην", "ἠ~υκεσᾰν";
    pluperfect_mid_1: "ἠ~υμην", _, "ἠ~υμεθᾰ";
    pluperfect_mid_2: "ἠ~υσο", "ἠ~υσθον", "ἠ~υσθε";
    pluperfect_mid_3: "ἠ~υτο", "ἠ~υσθην", _;
    pluperfect_pass_1: _, _, "ἠ~υμεθᾰ";
    pluperfect_pass_2: _, _, "ἠ~υσθε";
    pluperfect_pass_3: _, "ἠ~υσθην", "ἠ~υντο";
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
    // Ἰᾱτρευω: 3 words (ἰᾱτρευω, ἱκετευω, ἱππευω). Like Βουλευω, except in 146 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("ἰᾱτρευω", 3, "~υω", like Βουλευω;
    imperfect_act_1: "~υον", _, "~υομεν";
    imperfect_act_2: "~υες", "~υετον", "~υετε";
    imperfect_act_3: "~υε|~υεν", "~υετην", "~υον";
    imperfect_mid_1: "~υομην", _, "~υομεθᾰ";
    imperfect_mid_2: "~υου", "~υεσθον", "~υεσθε";
    imperfect_mid_3: "~υετο", "~υεσθην", "~υοντο";
    imperfect_pass_1: _, _, "~υομεθᾰ";
    imperfect_pass_2: _, _, "~υεσθε";
    imperfect_pass_3: "~υετο", _, _;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_act_1: "~υσᾰ", _, "~υσᾰμεν";
    aorist_ind_act_2: "~υσᾰς", "~υσᾰτον", "~υσᾰτε";
    aorist_ind_act_3: "~υσε|~υσεν", "~υσᾰτην", "~υσᾰν";
    aorist_ind_mid_1: "~υσᾰμην", _, "~υσᾰμεθᾰ";
    aorist_ind_mid_2: "~υσω", "~υσᾰσθον", "~υσᾰσθε";
    aorist_ind_mid_3: "~υσᾰτο", "~υσᾰσθην", "~υσᾰντο";
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
    perfect_ind_pass_1: _, _, "~υμεθᾰ";
    perfect_ind_pass_2: _, _, "~υσθε";
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
    perfect_imp_pass_2: "~υσο", _, _;
    perfect_imp_pass_3: "~υσθω", "~υσθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
    infinitive_fut: _, _, derived;
    infinitive_aor: _, _, derived;
    infinitive_perf: "~υκεναι", "~ῡσθαι", _;
    participle_fut: _, _, derived;
    participle_aor: _, _, derived;
    participle_perf: "~υκως", "~υμενος", _;
    ),
    // Ἐπῐτῐθημῐ: 3 words (ἐπῐτῐθημῐ, συνεπῐτῐθημῐ, ξυνεπῐτῐθημῐ). Like Φειδομαι, except in 255 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἐπῐτῐθημῐ", 3, "~ῐτῐ~ημῐ", like Φειδομαι;
    ind_act_1: "~ῐ%ῐ~ημῐ", _, "~ῐ%ῐ~εμεν";
    ind_act_2: "~ῐ%ῐ~ης", _, _;
    ind_act_3: "~ῐ%ῐ~ησῐ|~ῐ%ῐ~ησῐν", _, "~ῐ%ῐ~εᾱσῐ|~ῐ%ῐ~εᾱσῐν";
    ind_mid_1: "~ῐ%ῐ~εμαι", _, "~ῐ%ῐ~εμεθᾰ";
    ind_mid_2: "~ῐ%ῐ~εσαι", "~ῐ%ῐ~εσθον", "~ῐ%ῐ~εσθε";
    ind_mid_3: "~ῐ%ῐ~εται", _, "~ῐ%ῐ~ενται";
    ind_pass_1: _, _, "~ῐ%ῐ~εμεθᾰ";
    ind_pass_2: _, _, "~ῐ%ῐ~εσθε";
    ind_pass_3: "~ῐ%ῐ~εται", _, "~ῐ%ῐ~ενται";
    subj_act_1: "~ῐ%ῐ~ω", _, "~ῐ%ῐ~ωμεν";
    subj_act_2: "~ῐ%ῐ~ῃς", "~ῐ%ῐ~ητον", "~ῐ%ῐ~ητε";
    subj_act_3: "~ῐ%ῐ~ῃ", _, "~ῐ%ῐ~ωσῐ|~ῐ%ῐ~ωσῐν";
    subj_mid_1: "~ῐ%ῐ~ωμαι", _, "~ῐ%ῐ~ωμεθᾰ";
    subj_mid_2: "~ῐ%ῐ~ῃ", "~ῐ%ῐ~ησθον", "~ῐ%ῐ~ησθε";
    subj_mid_3: "~ῐ%ῐ~ηται", _, "~ῐ%ῐ~ωνται";
    subj_pass_1: _, _, "~ῐ%ῐ~ωμεθᾰ";
    subj_pass_2: _, _, "~ῐ%ῐ~ησθε";
    subj_pass_3: "~ῐ%ῐ~ηται", _, "~ῐ%ῐ~ωνται";
    opt_act_1: "~ῐ%ῐ~ειην", _, "~ῐ%ῐ~εῑμεν|~ῐ%ῐ~ειημεν";
    opt_act_2: "~ῐ%ῐ~ειης", "~ῐ%ῐ~εῑτον|~ῐ%ῐ~ειητον", "~ῐ%ῐ~εῑτε|~ῐ%ῐ~ειητε";
    opt_act_3: "~ῐ%ῐ~ειη", "~ῐ%ῐ~ειτην|~ῐ%ῐ~ειητην", "~ῐ%ῐ~εῑεν|~ῐ%ῐ~ειησᾰν";
    opt_mid_1: "~ῐ%ῐ~ειμην", _, "~ῐ%ῐ~ειμεθᾰ|~ῐ%ῐ~οιμεθᾰ";
    opt_mid_2: "~ῐ%ῐ~εῑο", "~ῐ%ῐ~εῑσθον|~ῐ%ῐ~οῑσθον", "~ῐ%ῐ~εῑσθε|~ῐ%ῐ~οῑσθε";
    opt_mid_3: "~ῐ%ῐ~εῑτο|~ῐ%ῐ~οῑτο", "~ῐ%ῐ~εισθην|~ῐ%ῐ~οισθην", "~ῐ%ῐ~εῑντο|~ῐ%ῐ~οῑντο";
    opt_pass_1: _, _, "~ῐ%ῐ~ειμεθᾰ|~ῐ%ῐ~οιμεθᾰ";
    opt_pass_2: _, "~ῐ%ῐ~εῑσθον|~ῐ%ῐ~οῑσθον", "~ῐ%ῐ~εῑσθε|~ῐ%ῐ~οῑσθε";
    opt_pass_3: "~ῐ%ῐ~εῑτο|~ῐ%ῐ~οῑτο", "~ῐ%ῐ~εισθην|~ῐ%ῐ~οισθην", "~ῐ%ῐ~εῑντο|~ῐ%ῐ~οῑντο";
    imp_act_2: "~ῐ%ῐ~ει", "~ῐ%ῐ~ετον", "~ῐ%ῐ~ετε";
    imp_act_3: "~ῐ%ῐ~ετω", "~ῐ%ῐ~ετων", "~ῐ%ῐ~εντων";
    imp_mid_2: "~ῐ%ῐ~εσο|~ῐ%ῐ~ου", _, "~ῐ%ῐ~εσθε";
    imp_mid_3: "~ῐ%ῐ~εσθω", "~ῐ%ῐ~εσθων", "~ῐ%ῐ~εσθων";
    imp_pass_2: "~ῐ%ῐ~εσο|~ῐ%ῐ~ου", _, "~ῐ%ῐ~εσθε";
    imp_pass_3: _, "~ῐ%ῐ~εσθων", _;
    imperfect_act_1: "~ε%ῐ~ην", _, "~ε%ῐ~εμεν";
    imperfect_act_2: "~ε%ῐ~εις", "~ε%ῐ~ετον", "~ε%ῐ~ετε";
    imperfect_act_3: "~ε%ῐ~ει", "~ε%ῐ~ετην", "~ε%ῐ~εσᾰν";
    imperfect_mid_1: "~ε%ῐ~εμην", _, "~ε%ῐ~εμεθᾰ";
    imperfect_mid_2: "~ε%ῐ~εσο", "~ε%ῐ~εσθον", "~ε%ῐ~εσθε";
    imperfect_mid_3: "~ε%ῐ~ετο", "~ε%ῐ~εσθην", "~ε%ῐ~εντο";
    imperfect_pass_1: _, _, "~ε%ῐ~εμεθᾰ";
    imperfect_pass_2: _, _, "~ε%ῐ~εσθε";
    imperfect_pass_3: "~ε%ῐ~ετο", _, _;
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
    aorist_ind_act_1: "~ε~ηκᾰ", _, "~ε~ηκᾰμεν|~ε~εμεν";
    aorist_ind_act_2: "~ε~ηκᾰς", "~ε~ηκᾰτον|~ε~ετον", "~ε~ηκᾰτε|~ε~ετε";
    aorist_ind_act_3: "~ε~ηκε|~ε~ηκεν", "~ε~ηκᾰτην|~ε~ετην", "~ε~ηκᾰν|~ε~εσᾰν";
    aorist_ind_mid_1: "~ε~ηκᾰμην|~ε~εμην", _, "~ε~ηκᾰμεθᾰ|~ε~εμεθᾰ";
    aorist_ind_mid_2: "~ε~ηκω|~ε~ου", "~ε~ηκᾰσθον|~ε~εσθον", "~ε~ηκᾰσθε|~ε~εσθε";
    aorist_ind_mid_3: "~ε~ηκᾰτο|~ε~ετο", "~ε~ηκᾰσθην|~ε~εσθην", "~ε~ηκᾰντο|~ε~εντο";
    aorist_subj_act_1: "~ῐ~ηκω|~ῐ~ω", _, "~ῐ~ηκωμεν|~ῐ~ωμεν";
    aorist_subj_act_2: "~ῐ~ηκῃς|~ῐ~ῃς", "~ῐ~ηκητον|~ῐ~ητον", "~ῐ~ηκητε|~ῐ~ητε";
    aorist_subj_act_3: "~ῐ~ηκῃ|~ῐ~ῃ", "~ῐ~ηκητον|~ῐ~ητον", "~ῐ~ηκωσῐ|~ῐ~ηκωσῐν|~ῐ~ωσῐ|~ῐ~ωσῐν";
    aorist_subj_mid_1: "~ῐ~ηκωμαι|~ῐ~ωμαι", _, "~ῐ~ηκωμεθᾰ|~ῐ~ωμεθᾰ";
    aorist_subj_mid_2: "~ῐ~ηκῃ|~ῐ~ῃ", "~ῐ~ηκησθον|~ῐ~ησθον", "~ῐ~ηκησθε|~ῐ~ησθε";
    aorist_subj_mid_3: "~ῐ~ηκηται|~ῐ~ηται", "~ῐ~ηκησθον|~ῐ~ησθον", "~ῐ~ηκωνται|~ῐ~ωνται";
    aorist_opt_act_1: "~ῐ~ηκαιμῐ|~ῐ~ειην", _, "~ῐ~ηκαιμεν|~ῐ~εῑμεν|~ῐ~ειημεν";
    aorist_opt_act_2: "~ῐ~ηκειᾰς|~ῐ~ηκαις|~ῐ~ειης", "~ῐ~ηκαιτον|~ῐ~εῑτον|~ῐ~ειητον", "~ῐ~ηκαιτε|~ῐ~εῑτε|~ῐ~ειητε";
    aorist_opt_act_3: "~ῐ~ηκειε|~ῐ~ηκειεν|~ῐ~ηκαι|~ῐ~ειη", "~ῐ~ηκαιτην|~ῐ~ειτην|~ῐ~ειητην", "~ῐ~ηκειᾰν|~ῐ~ηκαιεν|~ῐ~εῑεν|~ῐ~ειησᾰν";
    aorist_opt_mid_1: "~ῐ~ηκαιμην|~ῐ~ειμην", _, "~ῐ~ηκαιμεθᾰ|~ῐ~ειμεθᾰ";
    aorist_opt_mid_2: "~ῐ~ηκαιο|~ῐ~εῑο", "~ῐ~ηκαισθον|~ῐ~εῑσθον", "~ῐ~ηκαισθε|~ῐ~εῑσθε";
    aorist_opt_mid_3: "~ῐ~ηκαιτο|~ῐ~εῑτο|~ῐ~οῑτο", "~ῐ~ηκαισθην|~ῐ~εισθην", "~ῐ~ηκαιντο|~ῐ~εῑντο";
    aorist_imp_act_2: "~ῐ~ηκον|~ῐ~ες", "~ῐ~ηκᾰτον|~ῐ~ετον", "~ῐ~ηκᾰτε|~ῐ~ετε";
    aorist_imp_act_3: "~ῐ~ηκᾰτω|~ῐ~ετω", "~ῐ~ηκᾰτων|~ῐ~ετων", "~ῐ~ηκᾰντων|~ῐ~εντων";
    aorist_imp_mid_2: "~ῐ~ηκαι|~ῐ~ου", "~ῐ~ηκᾰσθον|~ῐ~εσθον", "~ῐ~ηκᾰσθε|~ῐ~εσθε";
    aorist_imp_mid_3: "~ῐ~ηκᾰσθω|~ῐ~εσθω", "~ῐ~ηκᾰσθων|~ῐ~εσθων", "~ῐ~ηκᾰσθων|~ῐ~εσθων";
    perfect_ind_act_1: "~ῐ%ε~εικᾰ", _, "~ῐ%ε~εικᾰμεν";
    perfect_ind_act_2: "~ῐ%ε~εικᾰς", "~ῐ%ε~εικᾰτον", "~ῐ%ε~εικᾰτε";
    perfect_ind_act_3: "~ῐ%ε~εικε|~ῐ%ε~εικεν", _, "~ῐ%ε~εικᾱσῐ|~ῐ%ε~εικᾱσῐν";
    perfect_ind_mid_1: "~ῐ%ε~ειμαι", _, "~ῐ%ε~ειμεθᾰ";
    perfect_ind_mid_2: "~ῐ%ε~εισαι", "~ῐ%ε~εισθον", "~ῐ%ε~εισθε";
    perfect_ind_mid_3: "~ῐ%ε~ειται", _, "~ῐ%ε~εινται";
    perfect_ind_pass_1: _, _, "~ῐ%ε~ειμεθᾰ";
    perfect_ind_pass_2: _, _, "~ῐ%ε~εισθε";
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
    perfect_imp_pass_2: "~ῐ%ε~εισο", _, _;
    perfect_imp_pass_3: "~ῐ%ε~εισθω", "~ῐ%ε~εισθων", _;
    infinitive_pres: "~ῐ%ῐ~εναι", "~ῐ%ῐ~εσθαι", _;
    infinitive_fut: "~ῐ~ησειν", "~ῐ~ησεσθαι", "~ῐ%ε~ησεσθαι";
    infinitive_aor: "~ῐ~ηκαι|~ῐ~εῑναι", "~ῐ~ηκᾰσθαι|~ῐ~εσθαι", _;
    infinitive_perf: "~ῐ%ε~εικεναι", "~ῐ%ε~εῑσθαι", _;
    participle_pres: "~ῐ%ῐ~εις", "~ῐ%ῐ~εμενος", _;
    participle_fut: "~ῐ~ησων", "~ῐ~ησομενος", "~ῐ%ε~ησομενος";
    participle_aor: "~ῐ~ηκᾱς|~ῐ~εις", "~ῐ~ηκᾰμενος|~ῐ~εμενος", _;
    participle_perf: "~ῐ%ε~εικως", "~ῐ%ε~ειμενος", _;
    ),
    // Μνημονευω: 3 words (μνημονευω, στρᾰτευω, στρᾰτοπεδευω). Like Βουλευω, except in 73 cells: future_ind_pass_2 sg, aorist_subj_pass_3 pl, aorist_opt_pass_1 pl, aorist_opt_pass_2 du, aorist_opt_pass_2 pl, aorist_opt_pass_3 du, ….
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
    perfect_ind_pass_1: _, _, "ἐ~υμεθᾰ";
    perfect_ind_pass_2: _, _, "ἐ~υσθε";
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
    perfect_imp_pass_2: "ἐ~υσο", _, _;
    perfect_imp_pass_3: "ἐ~υσθω", "ἐ~υσθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
    infinitive_perf: "ἐ~υκεναι", "ἐ~ῡσθαι", _;
    participle_perf: "ἐ~υκως", "ἐ~υμενος", _;
    ),
    // Λιστραω: 3 words (μεριμναω, κῠβισταω, λιστραω). Like Ἐπῐτῐθημῐ, except in 223 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("λιστραω", 3, "~ι~αω", like Ἐπῐτῐθημῐ;
    ind_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    ind_act_2: "~ῐ~ᾳς", _, _;
    ind_act_3: "~ῐ~ᾳ", _, "~ῐ~ωσῐ|~ῐ~ωσῐν";
    ind_mid_1: "~ῐ~ωμαι", _, "~ῐ~ωμεθᾰ";
    ind_mid_2: "~ῐ~ᾳ", "~ῐ~ᾱσθον", "~ῐ~ᾱσθε";
    ind_mid_3: "~ῐ~ᾱται", _, "~ῐ~ωνται";
    ind_pass_1: _, _, "~ῐ~ωμεθᾰ";
    ind_pass_2: _, _, "~ῐ~ᾱσθε";
    ind_pass_3: "~ῐ~ᾱται", _, "~ῐ~ωνται";
    subj_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    subj_act_2: "~ῐ~ᾳς", "~ῐ~ᾱτον", "~ῐ~ᾱτε";
    subj_act_3: "~ῐ~ᾳ", _, "~ῐ~ωσῐ|~ῐ~ωσῐν";
    subj_mid_1: "~ῐ~ωμαι", _, "~ῐ~ωμεθᾰ";
    subj_mid_2: "~ῐ~ᾳ", "~ῐ~ᾱσθον", "~ῐ~ᾱσθε";
    subj_mid_3: "~ῐ~ᾱται", _, "~ῐ~ωνται";
    subj_pass_1: _, _, "~ῐ~ωμεθᾰ";
    subj_pass_2: _, _, "~ῐ~ᾱσθε";
    subj_pass_3: "~ῐ~ᾱται", _, "~ῐ~ωνται";
    opt_act_1: "~ῐ~ῳην|~ῐ~ῳμῐ", _, "~ῐ~ῳμεν|~ῐ~ῳημεν";
    opt_act_2: "~ῐ~ῳης|~ῐ~ῳς", "~ῐ~ῳτον|~ῐ~ῳητον", "~ῐ~ῳτε|~ῐ~ῳητε";
    opt_act_3: "~ῐ~ῳη|~ῐ~ῳ", "~ῐ~ῳτην|~ῐ~ῳητην", "~ῐ~ῳεν|~ῐ~ῳησᾰν";
    opt_mid_1: "~ῐ~ῳμην", _, "~ῐ~ῳμεθᾰ";
    opt_mid_2: "~ῐ~ῳο", "~ῐ~ῳσθον", "~ῐ~ῳσθε";
    opt_mid_3: "~ῐ~ῳτο", "~ῐ~ῳσθην", "~ῐ~ῳντο";
    opt_pass_1: _, _, "~ῐ~ῳμεθᾰ";
    opt_pass_2: _, "~ῐ~ῳσθον", "~ῐ~ῳσθε";
    opt_pass_3: "~ῐ~ῳτο", "~ῐ~ῳσθην", "~ῐ~ῳντο";
    imp_act_2: "~ῐ~ᾱ", "~ῐ~ᾱτον", "~ῐ~ᾱτε";
    imp_act_3: "~ῐ~ᾱτω", "~ῐ~ᾱτων", "~ῐ~ωντων";
    imp_mid_2: "~ῐ~ω", _, "~ῐ~ᾱσθε";
    imp_mid_3: "~ῐ~ᾱσθω", "~ῐ~ᾱσθων", "~ῐ~ᾱσθων";
    imp_pass_2: derived, _, "~ῐ~ᾱσθε";
    imp_pass_3: _, "~ῐ~ᾱσθων", _;
    imperfect_act_1: "ἐ~ῐ~ων", _, "ἐ~ῐ~ωμεν";
    imperfect_act_2: "ἐ~ῐ~ᾱς", "ἐ~ῐ~ᾱτον", "ἐ~ῐ~ᾱτε";
    imperfect_act_3: "ἐ~ῐ~ᾱ", "ἐ~ῐ~ᾱτην", "ἐ~ῐ~ων";
    imperfect_mid_1: "ἐ~ῐ~ωμην", _, "ἐ~ῐ~ωμεθᾰ";
    imperfect_mid_2: "ἐ~ῐ~ω", "ἐ~ῐ~ᾱσθον", "ἐ~ῐ~ᾱσθε";
    imperfect_mid_3: "ἐ~ῐ~ᾱτο", "ἐ~ῐ~ᾱσθην", "ἐ~ῐ~ωντο";
    imperfect_pass_1: _, _, "ἐ~ῐ~ωμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ῐ~ᾱσθε";
    imperfect_pass_3: "ἐ~ῐ~ᾱτο", _, _;
    future_ind_act_3: _, _, "~ῐ~ησουσῐ";
    future_ind_mid_2: "~ῐ~ησῃ", _, _;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_act_1: "ἐ~ῐ~ησᾰ", _, "ἐ~ῐ~ησᾰμεν";
    aorist_ind_act_2: "ἐ~ῐ~ησᾰς", "ἐ~ῐ~ησᾰτον", "ἐ~ῐ~ησᾰτε";
    aorist_ind_act_3: "ἐ~ῐ~ησε", "ἐ~ῐ~ησᾰτην", "ἐ~ῐ~ησᾰν";
    aorist_ind_mid_1: "ἐ~ῐ~ησᾰμην", _, "ἐ~ῐ~ησᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ῐ~ησω", "ἐ~ῐ~ησᾰσθον", "ἐ~ῐ~ησᾰσθε";
    aorist_ind_mid_3: "ἐ~ῐ~ησᾰτο", "ἐ~ῐ~ησᾰσθην", "ἐ~ῐ~ησᾰντο";
    aorist_subj_act_1: "~ῐ~ησω", _, "~ῐ~ησωμεν";
    aorist_subj_act_2: "~ῐ~ησῃς", "~ῐ~ησητον", "~ῐ~ησητε";
    aorist_subj_act_3: "~ῐ~ησῃ", derived, "~ῐ~ησωσῐ";
    aorist_subj_mid_1: "~ῐ~ησωμαι", _, "~ῐ~ησωμεθᾰ";
    aorist_subj_mid_2: "~ῐ~ησῃ", "~ῐ~ησησθον", "~ῐ~ησησθε";
    aorist_subj_mid_3: "~ῐ~ησηται", derived, "~ῐ~ησωνται";
    aorist_opt_act_1: "~ῐ~ησαιμῐ", _, "~ῐ~ησαιμεν";
    aorist_opt_act_2: "~ῐ~ησειᾰς", "~ῐ~ησαιτον", "~ῐ~ησαιτε";
    aorist_opt_act_3: "~ῐ~ησειε", "~ῐ~ησαιτην", "~ῐ~ησειᾰν";
    aorist_opt_mid_1: "~ῐ~ησαιμην", _, "~ῐ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ῐ~ησαιο", "~ῐ~ησαισθον", "~ῐ~ησαισθε";
    aorist_opt_mid_3: "~ῐ~ησαιτο", "~ῐ~ησαισθην", "~ῐ~ησαιντο";
    aorist_imp_act_2: "~ῐ~ησον", "~ῐ~ησᾰτον", "~ῐ~ησᾰτε";
    aorist_imp_act_3: "~ῐ~ησᾰτω", "~ῐ~ησᾰτων", "~ῐ~ησᾰντων";
    aorist_imp_mid_2: "~ῐ~ησαι", "~ῐ~ησᾰσθον", "~ῐ~ησᾰσθε";
    aorist_imp_mid_3: "~ῐ~ησᾰσθω", "~ῐ~ησᾰσθων", derived;
    perfect_ind_act_1: "%ε~ῐ~ηκᾰ", _, "%ε~ῐ~ηκᾰμεν";
    perfect_ind_act_2: "%ε~ῐ~ηκᾰς", "%ε~ῐ~ηκᾰτον", "%ε~ῐ~ηκᾰτε";
    perfect_ind_act_3: "%ε~ῐ~ηκε", _, "%ε~ῐ~ηκᾱσῐ";
    perfect_ind_mid_1: "%ε~ῐ~ημαι", _, "%ε~ῐ~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ῐ~ησαι", "%ε~ῐ~ησθον", "%ε~ῐ~ησθε";
    perfect_ind_mid_3: "%ε~ῐ~ηται", _, "%ε~ῐ~ηνται";
    perfect_ind_pass_1: _, _, "%ε~ῐ~ημεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~ῐ~ησθε";
    perfect_subj_act_1: "%ε~ῐ~ηκω", _, "%ε~ῐ~ηκωμεν";
    perfect_subj_act_2: "%ε~ῐ~ηκῃς", "%ε~ῐ~ηκητον", "%ε~ῐ~ηκητε";
    perfect_subj_act_3: "%ε~ῐ~ηκῃ", _, "%ε~ῐ~ηκωσῐ";
    perfect_opt_act_1: "%ε~ῐ~ηκοιμῐ", _, "%ε~ῐ~ηκοιμεν";
    perfect_opt_act_2: "%ε~ῐ~ηκοις", "%ε~ῐ~ηκοιτον", "%ε~ῐ~ηκοιτε";
    perfect_opt_act_3: "%ε~ῐ~ηκοι", "%ε~ῐ~ηκοιτην", "%ε~ῐ~ηκοιεν";
    perfect_imp_act_2: derived, "%ε~ῐ~ηκετον", "%ε~ῐ~ηκετε";
    perfect_imp_act_3: "%ε~ῐ~ηκετω", "%ε~ῐ~ηκετων", "%ε~ῐ~ηκοντων";
    perfect_imp_mid_2: "%ε~ῐ~ησο", _, _;
    perfect_imp_mid_3: "%ε~ῐ~ησθω", "%ε~ῐ~ησθων", _;
    perfect_imp_pass_2: "%ε~ῐ~ησο", _, _;
    perfect_imp_pass_3: "%ε~ῐ~ησθω", "%ε~ῐ~ησθων", _;
    infinitive_pres: "~ῐ~ᾱν", "~ῐ~ᾱσθαι", _;
    infinitive_fut: _, _, derived;
    infinitive_aor: "~ῐ~ησαι", "~ῐ~ησᾰσθαι", _;
    infinitive_perf: "%ε~ῐ~ηκεναι", "%ε~ῐ~ησθαι", _;
    participle_pres: "~ῐ~ων", "~ῐ~ωμενος", _;
    participle_fut: _, _, derived;
    participle_aor: "~ῐ~ησᾱς", "~ῐ~ησᾰμενος", _;
    participle_perf: "%ε~ῐ~ηκως", "%ε~ῐ~ημενος", _;
    ),
    // Ἐπευχομαι: 3 words (προσευχομαι, ἀπευχομαι, ἐπευχομαι). Like Ἑπομαι, except in 132 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἐπευχομαι", 3, "~ευχομαι", like Ἑπομαι;
    ind_act_1: "~ευχομαι", _, _;
    ind_mid_1: "~ευχομαι", _, "~ευχομεθᾰ";
    ind_mid_2: "~ευχῃ", "~ευχεσθον", "~ευχεσθε";
    ind_mid_3: "~ευχεται", _, "~ευχονται";
    ind_pass_1: _, _, "~ευχομεθᾰ";
    ind_pass_2: _, _, "~ευχεσθε";
    ind_pass_3: "~ευχεται", _, "~ευχονται";
    subj_mid_1: "~ευχωμαι", _, "~ευχωμεθᾰ";
    subj_mid_2: "~ευχῃ", "~ευχησθον", "~ευχησθε";
    subj_mid_3: "~ευχηται", _, "~ευχωνται";
    subj_pass_1: _, _, "~ευχωμεθᾰ";
    subj_pass_2: _, _, "~ευχησθε";
    subj_pass_3: "~ευχηται", _, "~ευχωνται";
    opt_mid_1: "~ευχοιμην", _, "~ευχοιμεθᾰ";
    opt_mid_2: "~ευχοιο", "~ευχοισθον", "~ευχοισθε";
    opt_mid_3: "~ευχοιτο", "~ευχοισθην", "~ευχοιντο";
    opt_pass_1: _, _, "~ευχοιμεθᾰ";
    opt_pass_2: _, "~ευχοισθον", "~ευχοισθε";
    opt_pass_3: "~ευχοιτο", "~ευχοισθην", "~ευχοιντο";
    imp_mid_2: "~ευχου", _, "~ευχεσθε";
    imp_mid_3: "~ευχεσθω", "~ευχεσθων", "~ευχεσθων";
    imp_pass_2: _, _, "~ευχεσθε";
    imp_pass_3: _, "~ευχεσθων", _;
    imperfect_mid_1: "~ηυχομην", _, "~ηυχομεθᾰ";
    imperfect_mid_2: "~ηυχου", "~ηυχεσθον", "~ηυχεσθε";
    imperfect_mid_3: "~ηυχετο", "~ηυχεσθην", "~ηυχοντο";
    imperfect_pass_1: _, _, "~ηυχομεθᾰ";
    imperfect_pass_2: _, _, "~ηυχεσθε";
    imperfect_pass_3: "~ηυχετο", _, _;
    future_ind_mid_1: "~ευξομαι", _, "~ευξομεθᾰ";
    future_ind_mid_2: "~ευξῃ", "~ευξεσθον", "~ευξεσθε";
    future_ind_mid_3: "~ευξεται", _, "~ευξονται";
    future_opt_mid_1: "~ευξοιμην", _, "~ευξοιμεθᾰ";
    future_opt_mid_2: "~ευξοιο", "~ευξοισθον", "~ευξοισθε";
    future_opt_mid_3: "~ευξοιτο", "~ευξοισθην", "~ευξοιντο";
    aorist_ind_mid_1: "~ηυξᾰμην", _, "~ηυξᾰμεθᾰ";
    aorist_ind_mid_2: "~ηυξω", "~ηυξᾰσθον", "~ηυξᾰσθε";
    aorist_ind_mid_3: "~ηυξᾰτο", "~ηυξᾰσθην", "~ηυξᾰντο";
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
    perfect_ind_pass_1: _, _, "~ηυγμεθᾰ";
    perfect_ind_pass_2: _, _, "~ηῡχθε";
    perfect_imp_mid_2: "~ηῡξο", _, _;
    perfect_imp_mid_3: "~ηυχθω", "~ηυχθων", _;
    perfect_imp_pass_2: "~ηῡξο", _, _;
    perfect_imp_pass_3: "~ηυχθω", "~ηυχθων", _;
    pluperfect_mid_1: "~ηυγμην", _, "~ηυγμεθᾰ";
    pluperfect_mid_2: "~ηῡξο", "~ηῡχθον", "~ηῡχθε";
    pluperfect_mid_3: "~ηῡκτο", "~ηυχθην", _;
    pluperfect_pass_1: _, _, "~ηυγμεθᾰ";
    pluperfect_pass_2: _, _, "~ηῡχθε";
    pluperfect_pass_3: _, "~ηυχθην", "~ηυχᾰτο";
    infinitive_pres: _, "~ευχεσθαι", _;
    infinitive_fut: _, "~ευξεσθαι", _;
    infinitive_aor: _, "~ευξᾰσθαι", _;
    infinitive_perf: _, "~ηῡχθαι", _;
    participle_pres: _, "~ευχομενος", _;
    participle_fut: _, "~ευξομενος", _;
    participle_aor: _, "~ευξᾰμενος", _;
    participle_perf: _, "~ηυγμενος", _;
    ),
    // Ἐγκειμαι: 3 words (συγκειμαι, ξυγκειμαι, ἐγκειμαι). Like Φειδομαι, except in 131 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἐγκειμαι", 3, "~γ~ειμαι", like Φειδομαι;
    ind_act_1: "~γ~ειμαι", _, _;
    ind_mid_1: "~γ~ειμαι", _, "~γ~ειμεθα";
    ind_mid_2: "~γ~εισαι", "~γ~εισθον", "~γ~εισθε";
    ind_mid_3: "~γ~ειται", _, "~γ~εινται|~γ~εᾰται|~γ~εονται|~γ~ειᾰται";
    ind_pass_1: _, _, derived;
    ind_pass_2: _, _, derived;
    ind_pass_3: derived, _, derived;
    subj_mid_1: "~γ~ωμαι", _, "~γ~ωμεθᾰ";
    subj_mid_2: "~γ~εῃ", "~γ~εησθον", "~γ~εησθε";
    subj_mid_3: "~γ~εηται|~γ~εῑται", _, "~γ~εωνται|~γ~ειωνται";
    subj_pass_1: _, _, derived;
    subj_pass_2: _, _, derived;
    subj_pass_3: derived, _, derived;
    opt_mid_1: "~γ~οιμην", _, "~γ~οιμεθᾰ";
    opt_mid_2: "~γ~οῑο", "~γ~οῑσθον", "~γ~οῑσθε";
    opt_mid_3: "~γ~οῑτο", "~γ~οισθην", "~γ~οῑντο";
    opt_pass_1: _, _, derived;
    opt_pass_2: _, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_mid_2: "~γ~εισο", _, "~γ~εισθον";
    imp_mid_3: "~γ~εισθω", "~γ~εισθων", "~γ~εισθων";
    imp_pass_2: _, _, derived;
    imp_pass_3: _, derived, _;
    imperfect_mid_1: "~νε~ειμην", _, "~νε~ειμεθα";
    imperfect_mid_2: "~νε~εισο", "~νε~εισθον", "~νε~εισθε";
    imperfect_mid_3: "~νε~ειτο", "~νε~εισθην", "~νε~ειντο|~νε~εατο";
    imperfect_pass_1: _, _, derived;
    imperfect_pass_2: _, _, derived;
    imperfect_pass_3: derived, _, _;
    future_ind_mid_1: "~γ~εισομαι", _, "~γ~εισομεθᾰ";
    future_ind_mid_2: "~γ~εισῃ|~γ~εισει", "~γ~εισεσθον", "~γ~εισεσθε";
    future_ind_mid_3: "~γ~εισεται", _, "~γ~εισονται";
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_mid_1: "~γ~εισοιμην", _, "~γ~εισοιμεθᾰ";
    future_opt_mid_2: "~γ~εισοιο", "~γ~εισοισθον", "~γ~εισοισθε";
    future_opt_mid_3: "~γ~εισοιτο", "~γ~εισοισθην", "~γ~εισοιντο";
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_mid_1: derived, _, derived;
    aorist_ind_mid_2: derived, derived, derived;
    aorist_ind_mid_3: derived, derived, derived;
    aorist_subj_mid_1: derived, _, derived;
    aorist_subj_mid_2: derived, derived, derived;
    aorist_subj_mid_3: derived, _, derived;
    aorist_opt_mid_1: derived, _, derived;
    aorist_opt_mid_2: derived, derived, derived;
    aorist_opt_mid_3: derived, derived, derived;
    aorist_imp_mid_2: derived, derived, derived;
    aorist_imp_mid_3: derived, derived, _;
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, derived;
    perfect_ind_mid_3: derived, _, derived;
    perfect_imp_mid_2: derived, _, _;
    perfect_imp_mid_3: derived, derived, _;
    infinitive_pres: _, derived, _;
    infinitive_fut: _, "~γ~εισεσθαι", derived;
    infinitive_aor: _, derived, _;
    participle_pres: _, derived, _;
    participle_fut: _, "~γ~εισομενος", derived;
    participle_aor: _, derived, _;
    ),
    // Δῐᾰλῡω: 3 words (ἀντᾰνᾰλῡω, δῐᾰλῡω, κᾰτᾰλῡω).
    conjugation!("δῐᾰλῡω", 3, "~ᾰ~ῡω";
    ind_act_1: "~ᾰ~ῡω", _, "~ᾰ~ῡομεν";
    ind_act_2: "~ᾰ~ῡεις", _, _;
    ind_act_3: "~ᾰ~ῡει", _, "~ᾰ~ῡουσῐ|~ᾰ~ῡουσῐν";
    ind_mid_1: "~ᾰ~ῡομαι", _, "~ᾰ~ῡομεθᾰ";
    ind_mid_2: "~ᾰ~ῡῃ|~ᾰ~ῡει", "~ᾰ~ῡεσθον", "~ᾰ~ῡεσθε";
    ind_mid_3: "~ᾰ~ῡεται", _, "~ᾰ~ῡονται";
    ind_pass_1: _, _, "~ᾰ~ῡομεθᾰ";
    ind_pass_2: "~ᾰ~ῡῃ|~ᾰ~ῡει", _, "~ᾰ~ῡεσθε";
    ind_pass_3: "~ᾰ~ῡεται", _, "~ᾰ~ῡονται";
    subj_act_1: "~ᾰ~ῡω", _, "~ᾰ~ῡωμεν";
    subj_act_2: "~ᾰ~ῡῃς", "~ᾰ~ῡητον", "~ᾰ~ῡητε";
    subj_act_3: "~ᾰ~ῡῃ", _, "~ᾰ~ῡωσῐ|~ᾰ~ῡωσῐν";
    subj_mid_1: "~ᾰ~ῡωμαι", _, "~ᾰ~ῡωμεθᾰ";
    subj_mid_2: "~ᾰ~ῡῃ", "~ᾰ~ῡησθον", "~ᾰ~ῡησθε";
    subj_mid_3: "~ᾰ~ῡηται", _, "~ᾰ~ῡωνται";
    subj_pass_1: _, _, "~ᾰ~ῡωμεθᾰ";
    subj_pass_2: _, _, "~ᾰ~ῡησθε";
    subj_pass_3: "~ᾰ~ῡηται", _, "~ᾰ~ῡωνται";
    opt_act_1: "~ᾰ~ῡοιμῐ", _, "~ᾰ~ῡοιμεν";
    opt_act_2: "~ᾰ~ῡοις", "~ᾰ~ῡοιτον", "~ᾰ~ῡοιτε";
    opt_act_3: "~ᾰ~ῡοι", "~ᾰ~ῡοιτην", "~ᾰ~ῡοιεν";
    opt_mid_1: "~ᾰ~ῡοιμην", _, "~ᾰ~ῡοιμεθᾰ";
    opt_mid_2: "~ᾰ~ῡοιο", "~ᾰ~ῡοισθον", "~ᾰ~ῡοισθε";
    opt_mid_3: "~ᾰ~ῡοιτο", "~ᾰ~ῡοισθην", "~ᾰ~ῡοιντο";
    opt_pass_1: _, _, "~ᾰ~ῡοιμεθᾰ";
    opt_pass_2: _, "~ᾰ~ῡοισθον", "~ᾰ~ῡοισθε";
    opt_pass_3: "~ᾰ~ῡοιτο", "~ᾰ~ῡοισθην", "~ᾰ~ῡοιντο";
    imp_act_2: "~ᾰ~ῡε", "~ᾰ~ῡετον", "~ᾰ~ῡετε";
    imp_act_3: "~ᾰ~ῡετω", "~ᾰ~ῡετων", "~ᾰ~ῡοντων";
    imp_mid_2: "~ᾰ~ῡου", _, "~ᾰ~ῡεσθε";
    imp_mid_3: "~ᾰ~ῡεσθω", "~ᾰ~ῡεσθων", "~ᾰ~ῡεσθων";
    imp_pass_2: _, _, "~ᾰ~ῡεσθε";
    imp_pass_3: _, "~ᾰ~ῡεσθων", _;
    imperfect_act_1: "~ε~ῡον", _, "~ε~ῡομεν";
    imperfect_act_2: "~ε~ῡες", "~ε~ῡετον", "~ε~ῡετε";
    imperfect_act_3: "~ε~ῡε|~ε~ῡεν", "~ε~ῡετην", "~ε~ῡον";
    imperfect_mid_1: "~ε~ῡομην", _, "~ε~ῡομεθᾰ";
    imperfect_mid_2: "~ε~ῡου", "~ε~ῡεσθον", "~ε~ῡεσθε";
    imperfect_mid_3: "~ε~ῡετο", "~ε~ῡεσθην", "~ε~ῡοντο";
    imperfect_pass_1: _, _, "~ε~ῡομεθᾰ";
    imperfect_pass_2: _, _, "~ε~ῡεσθε";
    imperfect_pass_3: "~ε~ῡετο", _, _;
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
    aorist_ind_act_1: "~ε~ῡσᾰ", _, "~ε~ῡσᾰμεν";
    aorist_ind_act_2: "~ε~ῡσᾰς", "~ε~ῡσᾰτον", "~ε~ῡσᾰτε";
    aorist_ind_act_3: "~ε~ῡσε|~ε~ῡσεν", "~ε~ῡσᾰτην", "~ε~ῡσᾰν";
    aorist_ind_mid_1: "~ε~ῡσᾰμην", _, "~ε~ῡσᾰμεθᾰ";
    aorist_ind_mid_2: "~ε~ῡσω", "~ε~ῡσᾰσθον", "~ε~ῡσᾰσθε";
    aorist_ind_mid_3: "~ε~ῡσᾰτο", "~ε~ῡσᾰσθην", "~ε~ῡσᾰντο";
    aorist_ind_pass_1: "~ε~ῠθην", _, "~ε~ῠθημεν";
    aorist_ind_pass_2: "~ε~ῠθης", "~ε~ῠθητον", "~ε~ῠθητε";
    aorist_ind_pass_3: "~ε~ῠθη", "~ε~ῠθητην", "~ε~ῠθησᾰν";
    aorist_subj_act_1: "~ᾰ~ῡσω", _, "~ᾰ~ῡσωμεν";
    aorist_subj_act_2: "~ᾰ~ῡσῃς", "~ᾰ~ῡσητον", "~ᾰ~ῡσητε";
    aorist_subj_act_3: "~ᾰ~ῡσῃ", _, "~ᾰ~ῡσωσῐ|~ᾰ~ῡσωσῐν";
    aorist_subj_mid_1: "~ᾰ~ῡσωμαι", _, "~ᾰ~ῡσωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~ῡσῃ", "~ᾰ~ῡσησθον", "~ᾰ~ῡσησθε";
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
    perfect_ind_pass_1: _, _, "~ᾰ~ελῠμεθᾰ";
    perfect_ind_pass_2: _, _, "~ᾰ~ελῠσθε";
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
    perfect_imp_pass_2: "~ᾰ~ελῠσο", _, _;
    perfect_imp_pass_3: "~ᾰ~ελῠσθω", "~ᾰ~ελῠσθων", _;
    pluperfect_act_1: "~ε~ελῠκειν|~ε~ελῠκη", _, "~ε~ελῠκεμεν";
    pluperfect_act_2: "~ε~ελῠκεις|~ε~ελῠκης", "~ε~ελῠκετον", "~ε~ελῠκετε";
    pluperfect_act_3: "~ε~ελῠκει|~ε~ελῠκειν", "~ε~ελῠκετην", "~ε~ελῠκεσᾰν";
    pluperfect_mid_1: "~ε~ελῠμην", _, "~ε~ελῠμεθᾰ";
    pluperfect_mid_2: "~ε~ελῠσο", "~ε~ελῠσθον", "~ε~ελῠσθε";
    pluperfect_mid_3: "~ε~ελῠτο", "~ε~ελῠσθην", _;
    pluperfect_pass_1: _, _, "~ε~ελῠμεθᾰ";
    pluperfect_pass_2: _, _, "~ε~ελῠσθε";
    pluperfect_pass_3: _, "~ε~ελῠσθην", "~ε~ελῠντο";
    future_perfect_ind_mid_1: "~ᾰ~ελῡσομαι", _, "~ᾰ~ελῡσομεθᾰ";
    future_perfect_ind_mid_2: "~ᾰ~ελῡσῃ|~ᾰ~ελῡσει", "~ᾰ~ελῡσεσθον", "~ᾰ~ελῡσεσθε";
    future_perfect_ind_mid_3: "~ᾰ~ελῡσεται", _, "~ᾰ~ελῡσονται";
    future_perfect_ind_pass_2: "~ᾰ~ελῡσῃ|~ᾰ~ελῡσει", _, _;
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
    // Δαμνημῐ: 2 words (δαμνημῐ, σκιδνημῐ). Like Θωᾰω, except in 76 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("δαμνημῐ", 2, "~ημῐ", like Θωᾰω;
    ind_act_1: "~ημῐ", _, "~ᾰμεν";
    ind_act_2: "~ης", _, _;
    ind_act_3: "~ησῐ|~ησῐν", _, "~ᾱσῐ|~ᾱσῐν";
    ind_mid_1: "~ᾰμαι", _, "~ᾰμεθᾰ";
    ind_mid_2: "~ᾰσαι", "~ᾰσθον", "~ᾰσθε";
    ind_mid_3: "~ᾰται", _, "~ᾰνται";
    ind_pass_1: _, _, "~ᾰμεθᾰ";
    ind_pass_2: _, _, "~ᾰσθε";
    ind_pass_3: "~ᾰται", _, "~ᾰνται";
    subj_mid_2: "~ῃ", _, _;
    opt_act_1: "~αιην", _, "~αῑμεν|~αιημεν";
    opt_act_2: "~αιης", "~αῑτον|~αιητον", "~αῑτε|~αιητε";
    opt_act_3: "~αιη", "~αιτην|~αιητην", "~αῑεν|~αιησᾰν";
    opt_mid_1: "~αιμην", _, "~αιμεθᾰ";
    opt_mid_2: "~αιο", "~αισθον", "~αισθε";
    opt_mid_3: "~αιτο", "~αισθην", "~αιντο";
    opt_pass_1: _, _, "~αιμεθᾰ";
    opt_pass_2: _, "~αισθον", "~αισθε";
    opt_pass_3: "~αιτο", "~αισθην", "~αιντο";
    imp_act_2: "~η", "~ᾰτον", "~ᾰτε";
    imp_act_3: "~ᾰτω", "~ᾰτων", "~ᾰντων";
    imp_mid_2: "~ᾰσο|~ω", _, "~ᾰσθε";
    imp_mid_3: "~ᾰσθω", "~ᾰσθων", "~ᾰσθων";
    imp_pass_2: "~ᾰσο|~ω", _, "~ᾰσθε";
    imp_pass_3: _, "~ᾰσθων", _;
    imperfect_act_1: "ἐ~ην", _, "ἐ~ᾰμεν";
    imperfect_act_2: "ἐ~ης", "ἐ~ᾰτον", "ἐ~ᾰτε";
    imperfect_act_3: "ἐ~η", "ἐ~ᾰτην", "ἐ~ᾰσᾰν";
    imperfect_mid_1: "ἐ~ᾰμην", _, "ἐ~ᾰμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰσο", "ἐ~ᾰσθον", "ἐ~ᾰσθε";
    imperfect_mid_3: "ἐ~ᾰτο", "ἐ~ᾰσθην", "ἐ~ᾰντο";
    imperfect_pass_1: _, _, "ἐ~ᾰμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ᾰσθε";
    imperfect_pass_3: "ἐ~ᾰτο", _, _;
    infinitive_pres: "~ᾰναι", "~ᾰσθαι", _;
    participle_pres: "~ᾱς", "~ᾰμενος", _;
    ),
    // Πῐμπλημῐ: 2 words (πῐμπλημῐ, πῐμπρημῐ). Like Χραω, except in 143 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("πῐμπλημῐ", 2, "πῐμ~ημῐ", like Χραω;
    ind_act_1: "πῐμ~ημῐ", _, "πῐμ~ᾰμεν";
    ind_act_2: "πῐμ~ης", _, _;
    ind_act_3: "πῐμ~ησῐ|πῐμ~ησῐν", _, "πῐμ~ᾱσῐ|πῐμ~ᾱσῐν";
    ind_mid_1: "πῐμ~ᾰμαι", _, "πῐμ~ᾰμεθᾰ";
    ind_mid_2: "πῐμ~ᾰσαι", "πῐμ~ᾰσθον", "πῐμ~ᾰσθε";
    ind_mid_3: "πῐμ~ᾰται", _, "πῐμ~ᾰνται";
    ind_pass_1: _, _, "πῐμ~ᾰμεθᾰ";
    ind_pass_2: _, _, "πῐμ~ᾰσθε";
    ind_pass_3: "πῐμ~ᾰται", _, "πῐμ~ᾰνται";
    subj_act_1: "πῐμ~ω", _, "πῐμ~ωμεν";
    subj_act_2: "πῐμ~ῃς", "πῐμ~ητον", "πῐμ~ητε";
    subj_act_3: "πῐμ~ῃ", _, "πῐμ~ωσῐ|πῐμ~ωσῐν";
    subj_mid_1: "πῐμ~ωμαι", _, "πῐμ~ωμεθᾰ";
    subj_mid_2: "πῐμ~ῃ", "πῐμ~ησθον", "πῐμ~ησθε";
    subj_mid_3: "πῐμ~ηται", _, "πῐμ~ωνται";
    subj_pass_1: _, _, "πῐμ~ωμεθᾰ";
    subj_pass_2: _, _, "πῐμ~ησθε";
    subj_pass_3: "πῐμ~ηται", _, "πῐμ~ωνται";
    opt_act_1: "πῐμ~αιην", _, "πῐμ~αῑμεν|πῐμ~αιημεν";
    opt_act_2: "πῐμ~αιης", "πῐμ~αῑτον|πῐμ~αιητον", "πῐμ~αῑτε|πῐμ~αιητε";
    opt_act_3: "πῐμ~αιη", "πῐμ~αιτην|πῐμ~αιητην", "πῐμ~αῑεν|πῐμ~αιησᾰν";
    opt_mid_1: "πῐμ~αιμην", _, "πῐμ~αιμεθᾰ";
    opt_mid_2: "πῐμ~αιο", "πῐμ~αισθον", "πῐμ~αισθε";
    opt_mid_3: "πῐμ~αιτο", "πῐμ~αισθην", "πῐμ~αιντο";
    opt_pass_1: _, _, "πῐμ~αιμεθᾰ";
    opt_pass_2: _, "πῐμ~αισθον", "πῐμ~αισθε";
    opt_pass_3: "πῐμ~αιτο", "πῐμ~αισθην", "πῐμ~αιντο";
    imp_act_2: "πῐμ~η", "πῐμ~ᾰτον", "πῐμ~ᾰτε";
    imp_act_3: "πῐμ~ᾰτω", "πῐμ~ᾰτων", "πῐμ~ᾰντων";
    imp_mid_2: "πῐμ~ᾰσο|πῐμ~ω", _, "πῐμ~ᾰσθε";
    imp_mid_3: "πῐμ~ᾰσθω", "πῐμ~ᾰσθων", "πῐμ~ᾰσθων";
    imp_pass_2: "πῐμ~ᾰσο|πῐμ~ω", _, "πῐμ~ᾰσθε";
    imp_pass_3: _, "πῐμ~ᾰσθων", _;
    imperfect_act_1: "ἐπῐμ~ην", _, "ἐπῐμ~ᾰμεν";
    imperfect_act_2: "ἐπῐμ~ης", "ἐπῐμ~ᾰτον", "ἐπῐμ~ᾰτε";
    imperfect_act_3: "ἐπῐμ~η", "ἐπῐμ~ᾰτην", "ἐπῐμ~ᾰσᾰν";
    imperfect_mid_1: "ἐπῐμ~ᾰμην", _, "ἐπῐμ~ᾰμεθᾰ";
    imperfect_mid_2: "ἐπῐμ~ᾰσο", "ἐπῐμ~ᾰσθον", "ἐπῐμ~ᾰσθε";
    imperfect_mid_3: "ἐπῐμ~ᾰτο", "ἐπῐμ~ᾰσθην", "ἐπῐμ~ᾰντο";
    imperfect_pass_1: _, _, "ἐπῐμ~ᾰμεθᾰ";
    imperfect_pass_2: _, _, "ἐπῐμ~ᾰσθε";
    imperfect_pass_3: "ἐπῐμ~ᾰτο", _, _;
    future_ind_act_3: _, _, "~ησουσῐ|~ησουσῐν";
    aorist_ind_act_3: "ἐ~ησε", _, _;
    aorist_ind_mid_1: "ἐ~ησᾰμην|ἐ~ημην", _, "ἐ~ησᾰμεθᾰ|ἐ~ημεθᾰ";
    aorist_ind_mid_2: "ἐ~ησω|ἐ~ηο", "ἐ~ησᾰσθον|ἐ~ησθον", "ἐ~ησᾰσθε|ἐ~ησθε";
    aorist_ind_mid_3: "ἐ~ησᾰτο|ἐ~ητο", "ἐ~ησᾰσθην|ἐ~ησθην", "ἐ~ησᾰντο|ἐ~ηντο";
    aorist_subj_pass_3: _, _, "~ησθωσῐ|~ησθωσῐν";
    aorist_opt_mid_1: "~ησαιμην|~ῃμην", _, "~ησαιμεθᾰ|~ῃμεθᾰ";
    aorist_opt_mid_2: "~ησαιο|~ῃο", "~ησαισθον|~ῃσθον", "~ησαισθε|~ῃσθε";
    aorist_opt_mid_3: "~ησαιτο|~ῃτο", "~ησαισθην|~ῃσθην", "~ησαιντο|~ῃντο";
    aorist_opt_pass_1: _, _, "~ησθεῑμεν|~ησθειημεν";
    aorist_opt_pass_2: _, "~ησθεῑτον|~ησθειητον", "~ησθεῑτε|~ησθειητε";
    aorist_opt_pass_3: _, "~ησθειτην|~ησθειητην", "~ησθεῑεν|~ησθειησᾰν";
    aorist_imp_mid_2: "~ησαι|~ηο", "~ησᾰσθον|~ησθον", "~ησᾰσθε|~ησθε";
    aorist_imp_mid_3: "~ησᾰσθω|~ησθω", "~ησᾰσθων|~ησθων", "~ησᾰσθων|~ησθων";
    perfect_ind_act_3: "%ε~ηκε|%ε~ηκεν", _, "%ε~ηκᾱσῐ|%ε~ηκᾱσῐν";
    perfect_ind_mid_1: "%ε~ησμαι", _, "%ε~ησμεθᾰ";
    perfect_ind_mid_3: "%ε~ησται", _, "%ε~ησᾰται";
    perfect_ind_pass_1: _, _, "%ε~ησμεθᾰ";
    perfect_subj_act_3: _, _, "%ε~ηκωσῐ|%ε~ηκωσῐν";
    perfect_opt_act_1: "%ε~ηκοιμῐ|%ε~ηκοιην", _, _;
    perfect_opt_act_2: "%ε~ηκοις|%ε~ηκοιης", _, _;
    perfect_opt_act_3: "%ε~ηκοι|%ε~ηκοιη", _, _;
    perfect_imp_act_2: "%ε~ηκε", _, _;
    pluperfect_mid_1: "ἐ%ε~ησμην", _, "ἐ%ε~ησμεθᾰ";
    pluperfect_mid_3: "ἐ%ε~ηστο", _, _;
    pluperfect_pass_1: _, _, "ἐ%ε~ησμεθᾰ";
    pluperfect_pass_3: _, _, "ἐ%ε~ησᾰτο";
    infinitive_pres: "πῐμ~ᾰναι", "πῐμ~ᾰσθαι", _;
    infinitive_aor: _, "~ησᾰσθαι|~ησθαι", _;
    participle_pres: "πῐμ~ᾱς", "πῐμ~ᾰμενος", _;
    participle_aor: _, "~ησᾰμενος|~ημενος", _;
    participle_perf: _, "%ε~ησμενος", _;
    ),
    // Πᾰρῐστημῐ: 2 words (πᾰρῐστημῐ, νῐστημῐ). Like Ἐξᾰκολουθεω, except in 306 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("πᾰρῐστημῐ", 2, "~ῐ~ημῐ", like Ἐξᾰκολουθεω;
    ind_act_1: "~ῐ~ημῐ", _, "~ῐ~ᾰμεν";
    ind_act_2: "~ῐ~ης", _, _;
    ind_act_3: "~ῐ~ησῐ|~ῐ~ησῐν", _, "~ῐ~ᾱσῐ|~ῐ~ᾱσῐν";
    ind_mid_1: "~ῐ~ᾰμαι", _, "~ῐ~ᾰμεθᾰ";
    ind_mid_2: "~ῐ~ᾰσαι", "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    ind_mid_3: "~ῐ~ᾰται", _, "~ῐ~ᾰνται";
    ind_pass_1: _, _, "~ῐ~ᾰμεθᾰ";
    ind_pass_2: derived, _, "~ῐ~ᾰσθε";
    ind_pass_3: "~ῐ~ᾰται", _, "~ῐ~ᾰνται";
    subj_act_1: "~ῐ~ω", _, "~ῐ~ωμεν";
    subj_act_2: "~ῐ~ῃς", "~ῐ~ητον", "~ῐ~ητε";
    subj_act_3: "~ῐ~ῃ", _, "~ῐ~ωσῐ|~ῐ~ωσῐν";
    subj_mid_1: "~ῐ~ωμαι", _, "~ῐ~ωμεθᾰ";
    subj_mid_2: "~ῐ~ῃ", "~ῐ~ησθον", "~ῐ~ησθε";
    subj_mid_3: "~ῐ~ηται", _, "~ῐ~ωνται";
    subj_pass_1: _, _, "~ῐ~ωμεθᾰ";
    subj_pass_2: _, _, "~ῐ~ησθε";
    subj_pass_3: "~ῐ~ηται", _, "~ῐ~ωνται";
    opt_act_1: "~ῐ~αιην", _, "~ῐ~αῑμεν|~ῐ~αιημεν";
    opt_act_2: "~ῐ~αιης", "~ῐ~αῑτον|~ῐ~αιητον", "~ῐ~αῑτε|~ῐ~αιητε";
    opt_act_3: "~ῐ~αιη", "~ῐ~αιτην|~ῐ~αιητην", "~ῐ~αῑεν|~ῐ~αιησᾰν";
    opt_mid_1: "~ῐ~αιμην", _, "~ῐ~αιμεθᾰ";
    opt_mid_2: "~ῐ~αιο", "~ῐ~αισθον", "~ῐ~αισθε";
    opt_mid_3: "~ῐ~αιτο", "~ῐ~αισθην", "~ῐ~αιντο";
    opt_pass_1: _, _, "~ῐ~αιμεθᾰ";
    opt_pass_2: _, "~ῐ~αισθον", "~ῐ~αισθε";
    opt_pass_3: "~ῐ~αιτο", "~ῐ~αισθην", "~ῐ~αιντο";
    imp_act_2: "~ῐ~η", "~ῐ~ᾰτον", "~ῐ~ᾰτε";
    imp_act_3: "~ῐ~ᾰτω", "~ῐ~ᾰτων", "~ῐ~ᾰντων";
    imp_mid_2: "~ῐ~ᾰσο|~ῐ~ω", _, "~ῐ~ᾰσθε";
    imp_mid_3: "~ῐ~ᾰσθω", "~ῐ~ᾰσθων", "~ῐ~ᾰσθων";
    imp_pass_2: "~ῐ~ᾰσο|~ῐ~ω", _, "~ῐ~ᾰσθε";
    imp_pass_3: _, "~ῐ~ᾰσθων", _;
    imperfect_act_1: "~ῑ~ην", _, "~ῑ~ᾰμεν";
    imperfect_act_2: "~ῑ~ης", "~ῑ~ᾰτον", "~ῑ~ᾰτε";
    imperfect_act_3: "~ῑ~η", "~ῑ~ᾰτην", "~ῑ~ᾰσᾰν";
    imperfect_mid_1: "~ῑ~ᾰμην", _, "~ῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~ῑ~ᾰσο", "~ῑ~ᾰσθον", "~ῑ~ᾰσθε";
    imperfect_mid_3: "~ῑ~ᾰτο", "~ῑ~ᾰσθην", "~ῑ~ᾰντο";
    imperfect_pass_1: _, _, "~ῑ~ᾰμεθᾰ";
    imperfect_pass_2: _, _, "~ῑ~ᾰσθε";
    imperfect_pass_3: "~ῑ~ᾰτο", _, _;
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ";
    future_ind_mid_2: "~ᾰ~ησῃ", _, _;
    future_ind_pass_1: "~ᾰ~ησομαι", _, "~ᾰ~ησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ησῃ", "~ᾰ~ησεσθον", "~ᾰ~ησεσθε";
    future_ind_pass_3: "~ᾰ~ησεται", _, "~ᾰ~ησονται";
    future_opt_pass_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ησοιο", "~ᾰ~ησοισθον", "~ᾰ~ησοισθε";
    future_opt_pass_3: "~ᾰ~ησοιτο", "~ᾰ~ησοισθην", "~ᾰ~ησοιντο";
    aorist_ind_act_1: "~ε~ησᾰ|~ε~ην", _, "~ε~ησᾰμεν|~ε~ημεν";
    aorist_ind_act_2: "~ε~ησᾰς|~ε~ης", "~ε~ησᾰτον|~ε~ητον", "~ε~ησᾰτε|~ε~ητε";
    aorist_ind_act_3: "~ε~ησε|~ε~ησεν|~ε~η", "~ε~ησᾰτην|~ε~ητην", "~ε~ησᾰν";
    aorist_ind_mid_1: "~ε~ησᾰμην|~ε~ᾰμην", _, "~ε~ησᾰμεθᾰ|~ε~ᾰμεθᾰ";
    aorist_ind_mid_2: "~ε~ησω|~ε~ω", "~ε~ησᾰσθον|~ε~ᾰσθον", "~ε~ησᾰσθε|~ε~ᾰσθε";
    aorist_ind_mid_3: "~ε~ησᾰτο|~ε~ᾰτο", "~ε~ησᾰσθην|~ε~ᾰσθην", "~ε~ησᾰντο|~ε~ᾰντο";
    aorist_ind_pass_1: "~ε~ᾰθην", _, "~ε~ᾰθημεν";
    aorist_ind_pass_2: "~ε~ᾰθης", "~ε~ᾰθητον", "~ε~ᾰθητε";
    aorist_ind_pass_3: "~ε~ᾰθη", "~ε~ᾰθητην", "~ε~ᾰθησᾰν";
    aorist_subj_act_1: "~ᾰ~ησω|~ᾰ~ω", _, "~ᾰ~ησωμεν|~ᾰ~ωμεν";
    aorist_subj_act_2: "~ᾰ~ησῃς|~ᾰ~ῃς", "~ᾰ~ησητον|~ᾰ~ητον", "~ᾰ~ησητε|~ᾰ~ητε";
    aorist_subj_act_3: "~ᾰ~ησῃ|~ᾰ~ῃ", "~ᾰ~ησητον|~ᾰ~ητον", "~ᾰ~ησωσῐ|~ᾰ~ησωσῐν|~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ησωμαι|~ᾰ~ωμαι", _, "~ᾰ~ησωμεθᾰ|~ᾰ~ωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~ησῃ|~ᾰ~ῃ", "~ᾰ~ησησθον|~ᾰ~ησθον", "~ᾰ~ησησθε|~ᾰ~ησθε";
    aorist_subj_mid_3: "~ᾰ~ησηται|~ᾰ~ηται", "~ᾰ~ησησθον|~ᾰ~ησθον", "~ᾰ~ησωνται|~ᾰ~ωνται";
    aorist_subj_pass_1: "~ᾰ~ᾰθω", _, "~ᾰ~ᾰθωμεν";
    aorist_subj_pass_2: "~ᾰ~ᾰθῃς", "~ᾰ~ᾰθητον", "~ᾰ~ᾰθητε";
    aorist_subj_pass_3: "~ᾰ~ᾰθῃ", _, "~ᾰ~ᾰθωσῐ";
    aorist_opt_act_1: "~ᾰ~ησαιμῐ|~ᾰ~αιην", _, "~ᾰ~ησαιμεν|~ᾰ~αῑμεν|~ᾰ~αιημεν";
    aorist_opt_act_2: "~ᾰ~ησειᾰς|~ᾰ~ησαις|~ᾰ~αιης", "~ᾰ~ησαιτον|~ᾰ~αῑτον|~ᾰ~αιητον", "~ᾰ~ησαιτε|~ᾰ~αῑτε|~ᾰ~αιητε";
    aorist_opt_act_3: "~ᾰ~ησειε|~ᾰ~ησειεν|~ᾰ~ησαι|~ᾰ~αιη", "~ᾰ~ησαιτην|~ᾰ~αιτην|~ᾰ~αιητην", "~ᾰ~ησειᾰν|~ᾰ~ησαιεν|~ᾰ~αῑεν|~ᾰ~αιησᾰν";
    aorist_opt_mid_1: "~ᾰ~ησαιμην|~ᾰ~αιμην", _, "~ᾰ~ησαιμεθᾰ|~ᾰ~αιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~ησαιο|~ᾰ~αῑο", "~ᾰ~ησαισθον|~ᾰ~αῑσθον", "~ᾰ~ησαισθε|~ᾰ~αῑσθε";
    aorist_opt_mid_3: "~ᾰ~ησαιτο|~ᾰ~αῑτο", "~ᾰ~ησαισθην|~ᾰ~αισθην", "~ᾰ~ησαιντο|~ᾰ~αῑντο";
    aorist_opt_pass_1: "~ᾰ~ᾰθειην", _, "~ᾰ~ᾰθεῑμεν";
    aorist_opt_pass_2: "~ᾰ~ᾰθειης", "~ᾰ~ᾰθεῑτον", "~ᾰ~ᾰθεῑτε";
    aorist_opt_pass_3: "~ᾰ~ᾰθειη", "~ᾰ~ᾰθειτην", "~ᾰ~ᾰθεῑεν";
    aorist_imp_act_2: "~ᾰ~ησον|~ᾰ~ηθῐ", "~ᾰ~ησᾰτον|~ᾰ~ητον", "~ᾰ~ησᾰτε|~ᾰ~ητε";
    aorist_imp_act_3: "~ᾰ~ησᾰτω|~ᾰ~ητω", "~ᾰ~ησᾰτων|~ᾰ~ητων", "~ᾰ~ησᾰντων|~ᾰ~ᾰντων";
    aorist_imp_mid_2: "~ᾰ~ησαι|~ᾰ~ω", "~ᾰ~ησᾰσθον|~ᾰ~ᾰσθον", "~ᾰ~ησᾰσθε|~ᾰ~ᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ησᾰσθω|~ᾰ~ᾰσθω", "~ᾰ~ησᾰσθων|~ᾰ~ᾰσθων", "~ᾰ~ησᾰσθων|~ᾰ~ᾰσθων";
    aorist_imp_pass_2: "~ᾰ~ᾰθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~ᾰθητω", "~ᾰ~ᾰθητων", "~ᾰ~ᾰθεντων";
    perfect_ind_act_1: "~ε~ᾰκᾰ", _, "~ε~ᾰκᾰμεν";
    perfect_ind_act_2: "~ε~ᾰκᾰς", "~ε~ᾰκᾰτον", "~ε~ᾰκᾰτε";
    perfect_ind_act_3: "~ε~ᾰκε", _, "~ε~ᾰκᾱσῐ";
    perfect_ind_mid_1: "~ε~ᾰμαι", _, "~ε~ᾰμεθᾰ";
    perfect_ind_mid_2: "~ε~ᾰσαι", "~ε~ᾰσθον", "~ε~ᾰσθε";
    perfect_ind_mid_3: "~ε~ᾰται", _, "~ε~ᾰνται";
    perfect_ind_pass_1: _, _, "~ε~ᾰμεθᾰ";
    perfect_ind_pass_2: _, _, "~ε~ᾰσθε";
    perfect_subj_act_1: "~ε~ᾰκω", _, "~ε~ᾰκωμεν";
    perfect_subj_act_2: "~ε~ᾰκῃς", "~ε~ᾰκητον", "~ε~ᾰκητε";
    perfect_subj_act_3: "~ε~ᾰκῃ", _, "~ε~ᾰκωσῐ";
    perfect_opt_act_1: "~ε~ᾰκοιμῐ", _, "~ε~ᾰκοιμεν";
    perfect_opt_act_2: "~ε~ᾰκοις", "~ε~ᾰκοιτον", "~ε~ᾰκοιτε";
    perfect_opt_act_3: "~ε~ᾰκοι", "~ε~ᾰκοιτην", "~ε~ᾰκοιεν";
    perfect_imp_act_2: derived, "~ε~ᾰκετον", "~ε~ᾰκετε";
    perfect_imp_act_3: "~ε~ᾰκετω", "~ε~ᾰκετων", "~ε~ᾰκοντων";
    perfect_imp_mid_2: "~ε~ᾰσο", _, _;
    perfect_imp_mid_3: "~ε~ᾰσθω", "~ε~ᾰσθων", _;
    perfect_imp_pass_2: "~ε~ᾰσο", _, _;
    perfect_imp_pass_3: "~ε~ᾰσθω", "~ε~ᾰσθων", _;
    pluperfect_act_1: "~ε~ᾰκειν", _, "~ε~ᾰκεμεν";
    pluperfect_act_2: "~ε~ᾰκεις", "~ε~ᾰκετον", "~ε~ᾰκετε";
    pluperfect_act_3: "~ε~ᾰκει", "~ε~ᾰκετην", "~ε~ᾰκεσᾰν";
    pluperfect_mid_1: "~ε~ᾰμην", _, "~ε~ᾰμεθᾰ";
    pluperfect_mid_2: "~ε~ᾰσο", "~ε~ᾰσθον", "~ε~ᾰσθε";
    pluperfect_mid_3: "~ε~ᾰτο", "~ε~ᾰσθην", _;
    pluperfect_pass_1: _, _, "~ε~ᾰμεθᾰ";
    pluperfect_pass_2: _, _, "~ε~ᾰσθε";
    pluperfect_pass_3: _, "~ε~ᾰσθην", "~ε~ᾰντο";
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
    infinitive_fut: _, _, "~ᾰ~ησεσθαι";
    infinitive_aor: "~ᾰ~ησαι|~ᾰ~ηναι", "~ᾰ~ησᾰσθαι|~ᾰ~ᾰσθαι", "~ᾰ~ᾰθηναι";
    infinitive_perf: "~ε~ᾰκεναι", "~ε~ᾰσθαι", _;
    infinitive_futp: "~ε~ηξειν", "~ε~ηξεσθαι", _;
    participle_pres: "~ῐ~ᾱς", "~ῐ~ᾰμενος", _;
    participle_fut: _, _, "~ᾰ~ησομενος";
    participle_aor: "~ᾰ~ησᾱς|~ᾰ~ᾱς", "~ᾰ~ησᾰμενος|~ᾰ~ᾰμενος", "~ᾰ~ᾰθεις";
    participle_perf: "~ε~ᾰκως", "~ε~ᾰμενος", _;
    participle_futp: "~ε~ηξων", "~ε~ηξομενος", _;
    ),
    // Μᾰρνᾰμαι: 2 words (μᾰρνᾰμαι, λῐνᾰμαι). Like Δαμνημῐ, except in 36 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, subj_act_1 sg, ….
    conjugation!("μᾰρνᾰμαι", 2, "~ᾰμαι", like Δαμνημῐ;
    ind_act_1: "~ᾰμαι", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, _, derived;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imp_act_3: derived, derived, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    infinitive_pres: derived, _, _;
    participle_pres: derived, _, _;
    ),
    // Σῠνομνῡμῐ: 2 words (σῠνομνῡμῐ, ξῠνομνῡμῐ). Like Ἐγκειμαι, except in 266 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("σῠνομνῡμῐ", 2, "~ο~νῡμῐ", like Ἐγκειμαι;
    ind_act_1: "~ο~νῡμῐ", _, "~ο~νῠμεν";
    ind_act_2: "~ο~νῡς", _, _;
    ind_act_3: "~ο~νῡσῐ|~ο~νῡσῐν", _, "~ο~νῠᾱσῐ|~ο~νῠᾱσῐν";
    ind_mid_1: "~ο~νῠμαι", _, "~ο~νῠμεθᾰ";
    ind_mid_2: "~ο~νῠσαι", "~ο~νῠσθον", "~ο~νῠσθε";
    ind_mid_3: "~ο~νῠται", _, "~ο~νῠνται";
    ind_pass_1: _, _, "~ο~νῠμεθᾰ";
    ind_pass_2: _, _, "~ο~νῠσθε";
    ind_pass_3: "~ο~νῠται", _, "~ο~νῠνται";
    subj_act_1: "~ο~νῠω", _, "~ο~νῠωμεν";
    subj_act_2: "~ο~νῠῃς", "~ο~νῠητον", "~ο~νῠητε";
    subj_act_3: "~ο~νῠῃ", _, "~ο~νῠωσῐ|~ο~νῠωσῐν";
    subj_mid_1: "~ο~νῠωμαι", _, "~ο~νῠωμεθᾰ";
    subj_mid_2: "~ο~νῠῃ", "~ο~νῠησθον", "~ο~νῠησθε";
    subj_mid_3: "~ο~νῠηται", _, "~ο~νῠωνται";
    subj_pass_1: _, _, "~ο~νῠωμεθᾰ";
    subj_pass_2: _, _, "~ο~νῠησθε";
    subj_pass_3: "~ο~νῠηται", _, "~ο~νῠωνται";
    opt_act_1: "~ο~νῠοιμῐ", _, "~ο~νῠοιμεν";
    opt_act_2: "~ο~νῠοις", "~ο~νῠοιτον", "~ο~νῠοιτε";
    opt_act_3: "~ο~νῠοι", "~ο~νῠοιτην", "~ο~νῠοιεν";
    opt_mid_1: "~ο~νῠοιμην", _, "~ο~νῠοιμεθᾰ";
    opt_mid_2: "~ο~νῠοιο", "~ο~νῠοισθον", "~ο~νῠοισθε";
    opt_mid_3: "~ο~νῠοιτο", "~ο~νῠοισθην", "~ο~νῠοιντο";
    opt_pass_1: _, _, "~ο~νῠοιμεθᾰ";
    opt_pass_2: _, "~ο~νῠοισθον", "~ο~νῠοισθε";
    opt_pass_3: "~ο~νῠοιτο", "~ο~νῠοισθην", "~ο~νῠοιντο";
    imp_act_2: "~ο~νῡ", "~ο~νῠτον", "~ο~νῠτε";
    imp_act_3: "~ο~νῠτω", "~ο~νῠτων", "~ο~νῠντων";
    imp_mid_2: "~ο~νῠσο", _, "~ο~νῠσθε";
    imp_mid_3: "~ο~νῠσθω", "~ο~νῠσθων", "~ο~νῠσθων";
    imp_pass_2: _, _, "~ο~νῠσθε";
    imp_pass_3: _, "~ο~νῠσθων", _;
    imperfect_act_1: "~ω~νῡν", _, "~ω~νῠμεν";
    imperfect_act_2: "~ω~νῡς", "~ω~νῠτον", "~ω~νῠτε";
    imperfect_act_3: "~ω~νῡ", "~ω~νῠτην", "~ω~νῠσᾰν";
    imperfect_mid_1: "~ω~νῠμην", _, "~ω~νῠμεθᾰ";
    imperfect_mid_2: "~ω~νῠσο", "~ω~νῠσθον", "~ω~νῠσθε";
    imperfect_mid_3: "~ω~νῠτο", "~ω~νῠσθην", "~ω~νῠντο";
    imperfect_pass_1: _, _, "~ω~νῠμεθᾰ";
    imperfect_pass_2: _, _, "~ω~νῠσθε";
    imperfect_pass_3: "~ω~νῠτο", _, _;
    future_ind_act_1: "~ο~ω", _, "~ο~οῡμεν";
    future_ind_act_2: "~ο~εῑς", "~ο~εῑτον", "~ο~εῑτε";
    future_ind_act_3: "~ο~εῑ", _, "~ο~οῡσῐ|~ο~οῡσῐν";
    future_ind_mid_1: "~ο~οῡμαι", _, "~ο~ουμεθᾰ";
    future_ind_mid_2: "~ο~εῑ|~ο~ῃ", "~ο~εῑσθον", "~ο~εῑσθε";
    future_ind_mid_3: "~ο~εῑται", _, "~ο~οῡνται";
    future_ind_pass_1: "~ο~οσθησομαι", _, "~ο~οσθησομεθᾰ";
    future_ind_pass_2: "~ο~οσθησῃ|~ο~οσθησει", "~ο~οσθησεσθον", "~ο~οσθησεσθε";
    future_ind_pass_3: "~ο~οσθησεται", _, "~ο~οσθησονται";
    future_opt_act_1: "~ο~οιην|~ο~οῑμῐ", _, "~ο~οῑμεν|~ο~οιημεν";
    future_opt_act_2: "~ο~οιης|~ο~οῑς", "~ο~οῑτον|~ο~οιητον", "~ο~οῑτε|~ο~οιητε";
    future_opt_act_3: "~ο~οιη|~ο~οῑ", "~ο~οιτην|~ο~οιητην", "~ο~οῑεν|~ο~οιησᾰν";
    future_opt_mid_1: "~ο~οιμην", _, "~ο~οιμεθᾰ";
    future_opt_mid_2: "~ο~οῑο", "~ο~οῑσθον", "~ο~οῑσθε";
    future_opt_mid_3: "~ο~οῑτο", "~ο~οισθην", "~ο~οῑντο";
    future_opt_pass_1: "~ο~οσθησοιμην", _, "~ο~οσθησοιμεθᾰ";
    future_opt_pass_2: "~ο~οσθησοιο", "~ο~οσθησοισθον", "~ο~οσθησοισθε";
    future_opt_pass_3: "~ο~οσθησοιτο", "~ο~οσθησοισθην", "~ο~οσθησοιντο";
    aorist_ind_act_1: "~ω~οσᾰ", _, "~ω~οσᾰμεν";
    aorist_ind_act_2: "~ω~οσᾰς", "~ω~οσᾰτον", "~ω~οσᾰτε";
    aorist_ind_act_3: "~ω~οσε|~ω~οσεν", "~ω~οσᾰτην", "~ω~οσᾰν";
    aorist_ind_mid_1: "~ω~οσᾰμην", _, "~ω~οσᾰμεθᾰ";
    aorist_ind_mid_2: "~ω~οσω", "~ω~οσᾰσθον", "~ω~οσᾰσθε";
    aorist_ind_mid_3: "~ω~οσᾰτο", "~ω~οσᾰσθην", "~ω~οσᾰντο";
    aorist_ind_pass_1: "~ω~οσθην|~ω~οθην", _, "~ω~οσθημεν|~ω~οθημεν";
    aorist_ind_pass_2: "~ω~οσθης|~ω~οθης", "~ω~οσθητον|~ω~οθητον", "~ω~οσθητε|~ω~οθητε";
    aorist_ind_pass_3: "~ω~οσθη|~ω~οθη", "~ω~οσθητην|~ω~οθητην", "~ω~οσθησᾰν|~ω~οθησᾰν";
    aorist_subj_act_1: "~ο~οσω", _, "~ο~οσωμεν";
    aorist_subj_act_2: "~ο~οσῃς", "~ο~οσητον", "~ο~οσητε";
    aorist_subj_act_3: "~ο~οσῃ", _, "~ο~οσωσῐ|~ο~οσωσῐν";
    aorist_subj_mid_1: "~ο~οσωμαι", _, "~ο~οσωμεθᾰ";
    aorist_subj_mid_2: "~ο~οσῃ", "~ο~οσησθον", "~ο~οσησθε";
    aorist_subj_mid_3: "~ο~οσηται", _, "~ο~οσωνται";
    aorist_subj_pass_1: "~ο~οσθω|~ο~οθω", _, "~ο~οσθωμεν|~ο~οθωμεν";
    aorist_subj_pass_2: "~ο~οσθῃς|~ο~οθῃς", "~ο~οσθητον|~ο~οθητον", "~ο~οσθητε|~ο~οθητε";
    aorist_subj_pass_3: "~ο~οσθῃ|~ο~οθῃ", "~ο~οσθητον|~ο~οθητον", "~ο~οσθωσῐ|~ο~οσθωσῐν|~ο~οθωσῐ|~ο~οθωσῐν";
    aorist_opt_act_1: "~ο~οσαιμῐ", _, "~ο~οσαιμεν";
    aorist_opt_act_2: "~ο~οσειᾰς|~ο~οσαις", "~ο~οσαιτον", "~ο~οσαιτε";
    aorist_opt_act_3: "~ο~οσειε|~ο~οσειεν|~ο~οσαι", "~ο~οσαιτην", "~ο~οσειᾰν|~ο~οσαιεν";
    aorist_opt_mid_1: "~ο~οσαιμην", _, "~ο~οσαιμεθᾰ";
    aorist_opt_mid_2: "~ο~οσαιο", "~ο~οσαισθον", "~ο~οσαισθε";
    aorist_opt_mid_3: "~ο~οσαιτο", "~ο~οσαισθην", "~ο~οσαιντο";
    aorist_opt_pass_1: "~ο~οσθειην|~ο~οθειην", _, "~ο~οσθεῑμεν|~ο~οσθειημεν|~ο~οθεῑμεν|~ο~οθειημεν";
    aorist_opt_pass_2: "~ο~οσθειης|~ο~οθειης", "~ο~οσθεῑτον|~ο~οσθειητον|~ο~οθεῑτον|~ο~οθειητον", "~ο~οσθεῑτε|~ο~οσθειητε|~ο~οθεῑτε|~ο~οθειητε";
    aorist_opt_pass_3: "~ο~οσθειη|~ο~οθειη", "~ο~οσθειτην|~ο~οσθειητην|~ο~οθειτην|~ο~οθειητην", "~ο~οσθεῑεν|~ο~οσθειησᾰν|~ο~οθεῑεν|~ο~οθειησᾰν";
    aorist_imp_act_2: "~ο~οσον", "~ο~οσᾰτον", "~ο~οσᾰτε";
    aorist_imp_act_3: "~ο~οσᾰτω", "~ο~οσᾰτων", "~ο~οσᾰντων";
    aorist_imp_mid_2: "~ο~οσαι", "~ο~οσᾰσθον", "~ο~οσᾰσθε";
    aorist_imp_mid_3: "~ο~οσᾰσθω", "~ο~οσᾰσθων", _;
    aorist_imp_pass_2: "~ο~οσθητῐ|~ο~οθητῐ", "~ο~οσθητον|~ο~οθητον", "~ο~οσθητε|~ο~οθητε";
    aorist_imp_pass_3: "~ο~οσθητω|~ο~οθητω", "~ο~οσθητων|~ο~οθητων", "~ο~οσθεντων|~ο~οθεντων";
    perfect_ind_act_1: "~ο~ωμοκᾰ", _, "~ο~ωμοκᾰμεν";
    perfect_ind_act_2: "~ο~ωμοκᾰς", "~ο~ωμοκᾰτον", "~ο~ωμοκᾰτε";
    perfect_ind_act_3: "~ο~ωμοκε|~ο~ωμοκεν", _, "~ο~ωμοκᾱσῐ|~ο~ωμοκᾱσῐν";
    perfect_subj_act_1: "~ο~ωμοκω", _, "~ο~ωμοκωμεν";
    perfect_subj_act_2: "~ο~ωμοκῃς", "~ο~ωμοκητον", "~ο~ωμοκητε";
    perfect_subj_act_3: "~ο~ωμοκῃ", _, "~ο~ωμοκωσῐ|~ο~ωμοκωσῐν";
    perfect_opt_act_1: "~ο~ωμοκοιμῐ|~ο~ωμοκοιην", _, "~ο~ωμοκοιμεν";
    perfect_opt_act_2: "~ο~ωμοκοις|~ο~ωμοκοιης", "~ο~ωμοκοιτον", "~ο~ωμοκοιτε";
    perfect_opt_act_3: "~ο~ωμοκοι|~ο~ωμοκοιη", "~ο~ωμοκοιτην", "~ο~ωμοκοιεν";
    perfect_imp_act_2: "~ο~ωμοκε", "~ο~ωμοκετον", "~ο~ωμοκετε";
    perfect_imp_act_3: "~ο~ωμοκετω", "~ο~ωμοκετων", "~ο~ωμοκοντων";
    infinitive_pres: "~ο~νῠναι", "~ο~νῠσθαι", _;
    infinitive_fut: "~ο~εῑν", "~ο~εῑσθαι", "~ο~οσθησεσθαι";
    infinitive_aor: "~ο~οσαι", "~ο~οσᾰσθαι", "~ο~οσθηναι|~ο~οθηναι";
    infinitive_perf: "~ο~ωμοκεναι", _, _;
    participle_pres: "~ο~νῡς", "~ο~νῠμενος", _;
    participle_fut: "~ο~ων", "~ο~ουμενος", "~ο~οσθησομενος";
    participle_aor: "~ο~οσᾱς", "~ο~οσᾰμενος", "~ο~οσθεις|~ο~οθεις";
    participle_perf: "~ο~ωμοκως", _, _;
    ),
    // Προσκειμαι: 2 words (προκειμαι, προσκειμαι). Like Ἐγκειμαι, except in 53 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("προσκειμαι", 2, "~~ειμαι", like Ἐγκειμαι;
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
    imp_mid_3: "~~εισθω", "~~εισθων", "~~εισθων";
    imperfect_mid_1: "~ε~ειμην", _, "~ε~ειμεθα";
    imperfect_mid_2: "~ε~εισο", "~ε~εισθον", "~ε~εισθε";
    imperfect_mid_3: "~ε~ειτο", "~ε~εισθην", "~ε~εατο";
    future_ind_mid_1: "~~εισομαι", _, "~~εισομεθᾰ";
    future_ind_mid_2: "~~εισῃ", "~~εισεσθον", "~~εισεσθε";
    future_ind_mid_3: "~~εισεται", _, "~~εισονται";
    future_opt_mid_1: "~~εισοιμην", _, "~~εισοιμεθᾰ";
    future_opt_mid_2: "~~εισοιο", "~~εισοισθον", "~~εισοισθε";
    future_opt_mid_3: "~~εισοιτο", "~~εισοισθην", "~~εισοιντο";
    infinitive_fut: _, "~~εισεσθαι", _;
    participle_fut: _, "~~εισομενος", _;
    ),
    // Ρρωννῡμῐ: 2 words (ῥωννῡμῐ, ρρωννῡμῐ). Like Κτῐζω, except in 142 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ρρωννῡμῐ", 2, "~ννῡμῐ", like Κτῐζω;
    ind_act_1: "~ννῡμῐ", _, "~ννῠμεν";
    ind_act_2: "~ννῡς", _, _;
    ind_act_3: "~ννῡσῐ|~ννῡσῐν", _, "~ννῠᾱσῐ|~ννῠᾱσῐν";
    ind_mid_1: "~ννῠμαι", _, "~ννῠμεθᾰ";
    ind_mid_2: "~ννῠσαι", "~ννῠσθον", "~ννῠσθε";
    ind_mid_3: "~ννῠται", _, "~ννῠνται";
    ind_pass_1: _, _, "~ννῠμεθᾰ";
    ind_pass_2: derived, _, "~ννῠσθε";
    ind_pass_3: "~ννῠται", _, "~ννῠνται";
    subj_act_1: "~ννῠω", _, "~ννῠωμεν";
    subj_act_2: "~ννῠῃς", "~ννῠητον", "~ννῠητε";
    subj_act_3: "~ννῠῃ", _, "~ννῠωσῐ|~ννῠωσῐν";
    subj_mid_1: "~ννῠωμαι", _, "~ννῠωμεθᾰ";
    subj_mid_2: "~ννῠῃ", "~ννῠησθον", "~ννῠησθε";
    subj_mid_3: "~ννῠηται", _, "~ννῠωνται";
    subj_pass_1: _, _, "~ννῠωμεθᾰ";
    subj_pass_2: _, _, "~ννῠησθε";
    subj_pass_3: "~ννῠηται", _, "~ννῠωνται";
    opt_act_1: "~ννῠοιμῐ", _, "~ννῠοιμεν";
    opt_act_2: "~ννῠοις", "~ννῠοιτον", "~ννῠοιτε";
    opt_act_3: "~ννῠοι", "~ννῠοιτην", "~ννῠοιεν";
    opt_mid_1: "~ννῠοιμην", _, "~ννῠοιμεθᾰ";
    opt_mid_2: "~ννῠοιο", "~ννῠοισθον", "~ννῠοισθε";
    opt_mid_3: "~ννῠοιτο", "~ννῠοισθην", "~ννῠοιντο";
    opt_pass_1: _, _, "~ννῠοιμεθᾰ";
    opt_pass_2: _, "~ννῠοισθον", "~ννῠοισθε";
    opt_pass_3: "~ννῠοιτο", "~ννῠοισθην", "~ννῠοιντο";
    imp_act_2: "~ννῡ", "~ννῠτον", "~ννῠτε";
    imp_act_3: "~ννῠτω", "~ννῠτων", "~ννῠντων";
    imp_mid_2: "~ννῠσο", _, "~ννῠσθε";
    imp_mid_3: "~ννῠσθω", "~ννῠσθων", "~ννῠσθων";
    imp_pass_2: _, _, "~ννῠσθε";
    imp_pass_3: _, "~ννῠσθων", _;
    imperfect_act_1: "ἐ~ννῡν", _, "ἐ~ννῠμεν";
    imperfect_act_2: "ἐ~ννῡς", "ἐ~ννῠτον", "ἐ~ννῠτε";
    imperfect_act_3: "ἐ~ννῡ", "ἐ~ννῠτην", "ἐ~ννῠσᾰν";
    imperfect_mid_1: "ἐ~ννῠμην", _, "ἐ~ννῠμεθᾰ";
    imperfect_mid_2: "ἐ~ννῠσο", "ἐ~ννῠσθον", "ἐ~ννῠσθε";
    imperfect_mid_3: "ἐ~ννῠτο", "ἐ~ννῠσθην", "ἐ~ννῠντο";
    imperfect_pass_1: _, _, "ἐ~ννῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ννῠσθε";
    imperfect_pass_3: "ἐ~ννῠτο", _, _;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: derived, derived, derived;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_mid_1: "ἐ~μαι", _, "ἐ~μεθᾰ";
    perfect_ind_mid_3: "ἐ~ται", _, "ἐ~νται";
    perfect_ind_pass_1: _, _, "ἐ~μεθᾰ";
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, derived;
    perfect_imp_act_3: derived, derived, derived;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: "ἐ~μην", _, "ἐ~μεθᾰ";
    pluperfect_mid_3: "ἐ~το", _, _;
    pluperfect_pass_1: _, _, "ἐ~μεθᾰ";
    pluperfect_pass_3: _, _, "ἐ~ντο";
    infinitive_pres: "~ννῠναι", "~ννῠσθαι", _;
    infinitive_perf: derived, _, _;
    participle_pres: "~ννῡς", "~ννῠμενος", _;
    participle_perf: derived, "ἐ~μενος", _;
    ),
    // Ῥηγνῡμῐ: 1 words (ῥηγνῡμῐ).
    conjugation!("ῥηγνῡμῐ", 1, "ῥηγνῡμῐ";
    ind_act_1: "ῥηγνῡμῐ|ῥηγνυω|ῥησσω|ῥηττω", _, "ῥηγνῠμεν|ῥηγνυομεν|ῥησσομεν|ῥηττομεν";
    ind_act_2: "ῥηγνῡς|ῥηγνυεις|ῥησσεις|ῥηττεις", "ῥηγνῠτον|ῥηγνυετον|ῥησσετον|ῥηττετον", "ῥηγνῠτε|ῥηγνυετε|ῥησσετε|ῥηττετε";
    ind_act_3: "ῥηγνῡσῐ|ῥηγνῡσῐν|ῥηγνυει|ῥησσει|ῥηττει", "ῥηγνῠτον|ῥηγνυετον|ῥησσετον|ῥηττετον", "ῥηγνῠᾱσῐ|ῥηγνῠᾱσῐν|ῥηγνυουσῐ|ῥηγνυουσῐν|ῥησσουσῐ|ῥησσουσῐν|ῥηττουσῐ|ῥηττουσῐν";
    ind_mid_1: "ῥηγνῠμαι|ῥηγνυομαι|ῥησσομαι|ῥηττομαι", _, "ῥηγνῠμεθᾰ|ῥηγνυομεθᾰ|ῥησσομεθᾰ|ῥηττομεθᾰ";
    ind_mid_2: "ῥηγνῠσαι|ῥηγνυῃ|ῥηγνυει|ῥησσῃ|ῥησσει|ῥηττῃ|ῥηττει", "ῥηγνῠσθον|ῥηγνυεσθον|ῥησσεσθον|ῥηττεσθον", "ῥηγνῠσθε|ῥηγνυεσθε|ῥησσεσθε|ῥηττεσθε";
    ind_mid_3: "ῥηγνῠται|ῥηγνυεται|ῥησσεται|ῥηττεται", "ῥηγνῠσθον|ῥηγνυεσθον|ῥησσεσθον|ῥηττεσθον", "ῥηγνῠνται|ῥηγνυονται|ῥησσονται|ῥηττονται";
    ind_pass_1: "ῥηγνῠμαι|ῥηγνυομαι|ῥησσομαι|ῥηττομαι", _, "ῥηγνῠμεθᾰ|ῥηγνυομεθᾰ|ῥησσομεθᾰ|ῥηττομεθᾰ";
    ind_pass_2: "ῥηγνῠσαι|ῥηγνυῃ|ῥηγνυει|ῥησσῃ|ῥησσει|ῥηττῃ|ῥηττει", "ῥηγνῠσθον|ῥηγνυεσθον|ῥησσεσθον|ῥηττεσθον", "ῥηγνῠσθε|ῥηγνυεσθε|ῥησσεσθε|ῥηττεσθε";
    ind_pass_3: "ῥηγνῠται|ῥηγνυεται|ῥησσεται|ῥηττεται", "ῥηγνῠσθον|ῥηγνυεσθον|ῥησσεσθον|ῥηττεσθον", "ῥηγνῠνται|ῥηγνυονται|ῥησσονται|ῥηττονται";
    subj_act_1: "ῥηγνῠω|ῥηγνυω|ῥησσω|ῥηττω", _, "ῥηγνῠωμεν|ῥηγνυωμεν|ῥησσωμεν|ῥηττωμεν";
    subj_act_2: "ῥηγνῠῃς|ῥηγνυῃς|ῥησσῃς|ῥηττῃς", "ῥηγνῠητον|ῥηγνυητον|ῥησσητον|ῥηττητον", "ῥηγνῠητε|ῥηγνυητε|ῥησσητε|ῥηττητε";
    subj_act_3: "ῥηγνῠῃ|ῥηγνυῃ|ῥησσῃ|ῥηττῃ", "ῥηγνῠητον|ῥηγνυητον|ῥησσητον|ῥηττητον", "ῥηγνῠωσῐ|ῥηγνῠωσῐν|ῥηγνυωσῐ|ῥηγνυωσῐν|ῥησσωσῐ|ῥησσωσῐν|ῥηττωσῐ|ῥηττωσῐν";
    subj_mid_1: "ῥηγνῠωμαι|ῥηγνυωμαι|ῥησσωμαι|ῥηττωμαι", _, "ῥηγνῠωμεθᾰ|ῥηγνυωμεθᾰ|ῥησσωμεθᾰ|ῥηττωμεθᾰ";
    subj_mid_2: "ῥηγνῠῃ|ῥηγνυῃ|ῥησσῃ|ῥηττῃ", "ῥηγνῠησθον|ῥηγνυησθον|ῥησσησθον|ῥηττησθον", "ῥηγνῠησθε|ῥηγνυησθε|ῥησσησθε|ῥηττησθε";
    subj_mid_3: "ῥηγνῠηται|ῥηγνυηται|ῥησσηται|ῥηττηται", "ῥηγνῠησθον|ῥηγνυησθον|ῥησσησθον|ῥηττησθον", "ῥηγνῠωνται|ῥηγνυωνται|ῥησσωνται|ῥηττωνται";
    subj_pass_1: "ῥηγνῠωμαι|ῥηγνυωμαι|ῥησσωμαι|ῥηττωμαι", _, "ῥηγνῠωμεθᾰ|ῥηγνυωμεθᾰ|ῥησσωμεθᾰ|ῥηττωμεθᾰ";
    subj_pass_2: "ῥηγνῠῃ|ῥηγνυῃ|ῥησσῃ|ῥηττῃ", "ῥηγνῠησθον|ῥηγνυησθον|ῥησσησθον|ῥηττησθον", "ῥηγνῠησθε|ῥηγνυησθε|ῥησσησθε|ῥηττησθε";
    subj_pass_3: "ῥηγνῠηται|ῥηγνυηται|ῥησσηται|ῥηττηται", "ῥηγνῠησθον|ῥηγνυησθον|ῥησσησθον|ῥηττησθον", "ῥηγνῠωνται|ῥηγνυωνται|ῥησσωνται|ῥηττωνται";
    opt_act_1: "ῥηγνῠοιμῐ|ῥηγνυοιμῐ|ῥησσοιμῐ|ῥηττοιμῐ", _, "ῥηγνῠοιμεν|ῥηγνυοιμεν|ῥησσοιμεν|ῥηττοιμεν";
    opt_act_2: "ῥηγνῠοις|ῥηγνυοις|ῥησσοις|ῥηττοις", "ῥηγνῠοιτον|ῥηγνυοιτον|ῥησσοιτον|ῥηττοιτον", "ῥηγνῠοιτε|ῥηγνυοιτε|ῥησσοιτε|ῥηττοιτε";
    opt_act_3: "ῥηγνῠοι|ῥηγνυοι|ῥησσοι|ῥηττοι", "ῥηγνῠοιτην|ῥηγνυοιτην|ῥησσοιτην|ῥηττοιτην", "ῥηγνῠοιεν|ῥηγνυοιεν|ῥησσοιεν|ῥηττοιεν";
    opt_mid_1: "ῥηγνῠοιμην|ῥηγνυοιμην|ῥησσοιμην|ῥηττοιμην", _, "ῥηγνῠοιμεθᾰ|ῥηγνυοιμεθᾰ|ῥησσοιμεθᾰ|ῥηττοιμεθᾰ";
    opt_mid_2: "ῥηγνῠοιο|ῥηγνυοιο|ῥησσοιο|ῥηττοιο", "ῥηγνῠοισθον|ῥηγνυοισθον|ῥησσοισθον|ῥηττοισθον", "ῥηγνῠοισθε|ῥηγνυοισθε|ῥησσοισθε|ῥηττοισθε";
    opt_mid_3: "ῥηγνῠοιτο|ῥηγνυοιτο|ῥησσοιτο|ῥηττοιτο", "ῥηγνῠοισθην|ῥηγνυοισθην|ῥησσοισθην|ῥηττοισθην", "ῥηγνῠοιντο|ῥηγνυοιντο|ῥησσοιντο|ῥηττοιντο";
    opt_pass_1: "ῥηγνῠοιμην|ῥηγνυοιμην|ῥησσοιμην|ῥηττοιμην", _, "ῥηγνῠοιμεθᾰ|ῥηγνυοιμεθᾰ|ῥησσοιμεθᾰ|ῥηττοιμεθᾰ";
    opt_pass_2: "ῥηγνῠοιο|ῥηγνυοιο|ῥησσοιο|ῥηττοιο", "ῥηγνῠοισθον|ῥηγνυοισθον|ῥησσοισθον|ῥηττοισθον", "ῥηγνῠοισθε|ῥηγνυοισθε|ῥησσοισθε|ῥηττοισθε";
    opt_pass_3: "ῥηγνῠοιτο|ῥηγνυοιτο|ῥησσοιτο|ῥηττοιτο", "ῥηγνῠοισθην|ῥηγνυοισθην|ῥησσοισθην|ῥηττοισθην", "ῥηγνῠοιντο|ῥηγνυοιντο|ῥησσοιντο|ῥηττοιντο";
    imp_act_2: "ῥηγνῡ|ῥηγνυε|ῥησσε|ῥηττε", "ῥηγνῠτον|ῥηγνυετον|ῥησσετον|ῥηττετον", "ῥηγνῠτε|ῥηγνυετε|ῥησσετε|ῥηττετε";
    imp_act_3: "ῥηγνῠτω|ῥηγνυετω|ῥησσετω|ῥηττετω", "ῥηγνῠτων|ῥηγνυετων|ῥησσετων|ῥηττετων", "ῥηγνῠντων|ῥηγνυοντων|ῥησσοντων|ῥηττοντων";
    imp_mid_2: "ῥηγνῠσο|ῥηγνυου|ῥησσου|ῥηττου", "ῥηγνῠσθον|ῥηγνυεσθον|ῥησσεσθον|ῥηττεσθον", "ῥηγνῠσθε|ῥηγνυεσθε|ῥησσεσθε|ῥηττεσθε";
    imp_mid_3: "ῥηγνῠσθω|ῥηγνυεσθω|ῥησσεσθω|ῥηττεσθω", "ῥηγνῠσθων|ῥηγνυεσθων|ῥησσεσθων|ῥηττεσθων", "ῥηγνῠσθων|ῥηγνυεσθων|ῥησσεσθων|ῥηττεσθων";
    imp_pass_2: "ῥηγνῠσο|ῥηγνυου|ῥησσου|ῥηττου", "ῥηγνῠσθον|ῥηγνυεσθον|ῥησσεσθον|ῥηττεσθον", "ῥηγνῠσθε|ῥηγνυεσθε|ῥησσεσθε|ῥηττεσθε";
    imp_pass_3: "ῥηγνῠσθω|ῥηγνυεσθω|ῥησσεσθω|ῥηττεσθω", "ῥηγνῠσθων|ῥηγνυεσθων|ῥησσεσθων|ῥηττεσθων", "ῥηγνῠσθων|ῥηγνυεσθων|ῥησσεσθων|ῥηττεσθων";
    imperfect_act_1: "ἐρρηγνῡν", _, "ἐρρηγνῠμεν";
    imperfect_act_2: "ἐρρηγνῡς", "ἐρρηγνῠτον", "ἐρρηγνῠτε";
    imperfect_act_3: "ἐρρηγνῡ", "ἐρρηγνῠτην", "ἐρρηγνῠσᾰν";
    imperfect_mid_1: "ἐρρηγνῠμην", _, "ἐρρηγνῠμεθᾰ";
    imperfect_mid_2: "ἐρρηγνῠσο", "ἐρρηγνῠσθον", "ἐρρηγνῠσθε";
    imperfect_mid_3: "ἐρρηγνῠτο", "ἐρρηγνῠσθην", "ἐρρηγνῠντο";
    imperfect_pass_1: _, _, "ἐρρηγνῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐρρηγνῠσθε";
    imperfect_pass_3: "ἐρρηγνῠτο", _, _;
    future_ind_act_1: "ῥηξω", _, "ῥηξομεν";
    future_ind_act_2: "ῥηξεις", "ῥηξετον", "ῥηξετε";
    future_ind_act_3: "ῥηξει", _, "ῥηξουσῐ|ῥηξουσῐν";
    future_ind_mid_1: "ῥηξομαι", _, "ῥηξομεθᾰ";
    future_ind_mid_2: "ῥηξῃ|ῥηξει", "ῥηξεσθον", "ῥηξεσθε";
    future_ind_mid_3: "ῥηξεται", _, "ῥηξονται";
    future_ind_pass_1: "ῥᾰγησομαι", _, "ῥᾰγησομεθᾰ";
    future_ind_pass_2: "ῥᾰγησῃ|ῥᾰγησει", "ῥᾰγησεσθον", "ῥᾰγησεσθε";
    future_ind_pass_3: "ῥᾰγησεται", _, "ῥᾰγησονται";
    future_opt_act_1: "ῥηξοιμῐ", _, "ῥηξοιμεν";
    future_opt_act_2: "ῥηξοις", "ῥηξοιτον", "ῥηξοιτε";
    future_opt_act_3: "ῥηξοι", "ῥηξοιτην", "ῥηξοιεν";
    future_opt_mid_1: "ῥηξοιμην", _, "ῥηξοιμεθᾰ";
    future_opt_mid_2: "ῥηξοιο", "ῥηξοισθον", "ῥηξοισθε";
    future_opt_mid_3: "ῥηξοιτο", "ῥηξοισθην", "ῥηξοιντο";
    future_opt_pass_1: "ῥᾰγησοιμην", _, "ῥᾰγησοιμεθᾰ";
    future_opt_pass_2: "ῥᾰγησοιο", "ῥᾰγησοισθον", "ῥᾰγησοισθε";
    future_opt_pass_3: "ῥᾰγησοιτο", "ῥᾰγησοισθην", "ῥᾰγησοιντο";
    aorist_ind_act_1: "ἐρρηξᾰ", _, "ἐρρηξᾰμεν";
    aorist_ind_act_2: "ἐρρηξᾰς", "ἐρρηξᾰτον", "ἐρρηξᾰτε";
    aorist_ind_act_3: "ἐρρηξε|ἐρρηξεν", "ἐρρηξᾰτην", "ἐρρηξᾰν";
    aorist_ind_mid_1: "ἐρρηξᾰμην", _, "ἐρρηξᾰμεθᾰ";
    aorist_ind_mid_2: "ἐρρηξω", "ἐρρηξᾰσθον", "ἐρρηξᾰσθε";
    aorist_ind_mid_3: "ἐρρηξᾰτο", "ἐρρηξᾰσθην", "ἐρρηξᾰντο";
    aorist_ind_pass_1: "ἐρρᾰγην|ἐρρηχθην", _, "ἐρρᾰγημεν|ἐρρηχθημεν";
    aorist_ind_pass_2: "ἐρρᾰγης|ἐρρηχθης", "ἐρρᾰγητον|ἐρρηχθητον", "ἐρρᾰγητε|ἐρρηχθητε";
    aorist_ind_pass_3: "ἐρρᾰγη|ἐρρηχθη", "ἐρρᾰγητην|ἐρρηχθητην", "ἐρρᾰγησᾰν|ἐρρηχθησᾰν";
    aorist_subj_act_1: "ῥηξω", _, "ῥηξωμεν";
    aorist_subj_act_2: "ῥηξῃς", "ῥηξητον", "ῥηξητε";
    aorist_subj_act_3: "ῥηξῃ", _, "ῥηξωσῐ|ῥηξωσῐν";
    aorist_subj_mid_1: "ῥηξωμαι", _, "ῥηξωμεθᾰ";
    aorist_subj_mid_2: "ῥηξῃ", "ῥηξησθον", "ῥηξησθε";
    aorist_subj_mid_3: "ῥηξηται", _, "ῥηξωνται";
    aorist_subj_pass_1: "ῥᾰγω|ῥηχθω", _, "ῥᾰγωμεν|ῥηχθωμεν";
    aorist_subj_pass_2: "ῥᾰγῃς|ῥηχθῃς", "ῥᾰγητον|ῥηχθητον", "ῥᾰγητε|ῥηχθητε";
    aorist_subj_pass_3: "ῥᾰγῃ|ῥηχθῃ", "ῥᾰγητον|ῥηχθητον", "ῥᾰγωσῐ|ῥᾰγωσῐν|ῥηχθωσῐ|ῥηχθωσῐν";
    aorist_opt_act_1: "ῥηξαιμῐ", _, "ῥηξαιμεν";
    aorist_opt_act_2: "ῥηξειᾰς|ῥηξαις", "ῥηξαιτον", "ῥηξαιτε";
    aorist_opt_act_3: "ῥηξειε|ῥηξειεν|ῥηξαι", "ῥηξαιτην", "ῥηξειᾰν|ῥηξαιεν";
    aorist_opt_mid_1: "ῥηξαιμην", _, "ῥηξαιμεθᾰ";
    aorist_opt_mid_2: "ῥηξαιο", "ῥηξαισθον", "ῥηξαισθε";
    aorist_opt_mid_3: "ῥηξαιτο", "ῥηξαισθην", "ῥηξαιντο";
    aorist_opt_pass_1: "ῥᾰγειην|ῥηχθειην", _, "ῥᾰγεῑμεν|ῥᾰγειημεν|ῥηχθεῑμεν|ῥηχθειημεν";
    aorist_opt_pass_2: "ῥᾰγειης|ῥηχθειης", "ῥᾰγεῑτον|ῥᾰγειητον|ῥηχθεῑτον|ῥηχθειητον", "ῥᾰγεῑτε|ῥᾰγειητε|ῥηχθεῑτε|ῥηχθειητε";
    aorist_opt_pass_3: "ῥᾰγειη|ῥηχθειη", "ῥᾰγειτην|ῥᾰγειητην|ῥηχθειτην|ῥηχθειητην", "ῥᾰγεῑεν|ῥᾰγειησᾰν|ῥηχθεῑεν|ῥηχθειησᾰν";
    aorist_imp_act_2: "ῥηξον", "ῥηξᾰτον", "ῥηξᾰτε";
    aorist_imp_act_3: "ῥηξᾰτω", "ῥηξᾰτων", "ῥηξᾰντων";
    aorist_imp_mid_2: "ῥηξαι", "ῥηξᾰσθον", "ῥηξᾰσθε";
    aorist_imp_mid_3: "ῥηξᾰσθω", "ῥηξᾰσθων", _;
    aorist_imp_pass_2: "ῥᾰγηθῐ|ῥηχθητῐ", "ῥᾰγητον|ῥηχθητον", "ῥᾰγητε|ῥηχθητε";
    aorist_imp_pass_3: "ῥᾰγητω|ῥηχθητω", "ῥᾰγητων|ῥηχθητων", "ῥᾰγεντων|ῥηχθεντων";
    perfect_ind_act_1: "ἐρρηχᾰ|ἐρρωγᾰ", _, "ἐρρηχᾰμεν|ἐρρωγᾰμεν";
    perfect_ind_act_2: "ἐρρηχᾰς|ἐρρωγᾰς", "ἐρρηχᾰτον|ἐρρωγᾰτον", "ἐρρηχᾰτε|ἐρρωγᾰτε";
    perfect_ind_act_3: "ἐρρηχε|ἐρρηχεν|ἐρρωγε|ἐρρωγεν", "ἐρρηχᾰτον|ἐρρωγᾰτον", "ἐρρηχᾱσῐ|ἐρρηχᾱσῐν|ἐρρωγᾱσῐ|ἐρρωγᾱσῐν";
    perfect_ind_mid_1: "ἐρρηγμαι", _, "ἐρρηγμεθᾰ";
    perfect_ind_mid_2: "ἐρρηξαι", "ἐρρηχθον", "ἐρρηχθε";
    perfect_ind_mid_3: "ἐρρηκται", _, "ἐρρηγᾰται";
    perfect_ind_pass_1: _, _, "ἐρρηγμεθᾰ";
    perfect_ind_pass_2: _, _, "ἐρρηχθε";
    perfect_subj_act_1: "ἐρρηχω|ἐρρωγω", _, "ἐρρηχωμεν|ἐρρωγωμεν";
    perfect_subj_act_2: "ἐρρηχῃς|ἐρρωγῃς", "ἐρρηχητον|ἐρρωγητον", "ἐρρηχητε|ἐρρωγητε";
    perfect_subj_act_3: "ἐρρηχῃ|ἐρρωγῃ", "ἐρρηχητον|ἐρρωγητον", "ἐρρηχωσῐ|ἐρρηχωσῐν|ἐρρωγωσῐ|ἐρρωγωσῐν";
    perfect_opt_act_1: "ἐρρηχοιμῐ|ἐρρηχοιην|ἐρρωγοιμῐ|ἐρρωγοιην", _, "ἐρρηχοιμεν|ἐρρωγοιμεν";
    perfect_opt_act_2: "ἐρρηχοις|ἐρρηχοιης|ἐρρωγοις|ἐρρωγοιης", "ἐρρηχοιτον|ἐρρωγοιτον", "ἐρρηχοιτε|ἐρρωγοιτε";
    perfect_opt_act_3: "ἐρρηχοι|ἐρρηχοιη|ἐρρωγοι|ἐρρωγοιη", "ἐρρηχοιτην|ἐρρωγοιτην", "ἐρρηχοιεν|ἐρρωγοιεν";
    perfect_imp_act_2: "ἐρρηχε|ἐρρωγε", "ἐρρηχετον|ἐρρωγετον", "ἐρρηχετε|ἐρρωγετε";
    perfect_imp_act_3: "ἐρρηχετω|ἐρρωγετω", "ἐρρηχετων|ἐρρωγετων", "ἐρρηχοντων|ἐρρωγοντων";
    perfect_imp_mid_2: "ἐρρηξο", _, _;
    perfect_imp_mid_3: "ἐρρηχθω", "ἐρρηχθων", _;
    perfect_imp_pass_2: "ἐρρηξο", _, _;
    perfect_imp_pass_3: "ἐρρηχθω", "ἐρρηχθων", _;
    pluperfect_act_1: "ἐρρωγειν|ἐρρωγη", _, "ἐρρωγεμεν";
    pluperfect_act_2: "ἐρρωγεις|ἐρρωγης", "ἐρρωγετον", "ἐρρωγετε";
    pluperfect_act_3: "ἐρρωγει|ἐρρωγειν", "ἐρρωγετην", "ἐρρωγεσᾰν";
    infinitive_pres: "ῥηγνῠναι|ῥηγνυειν|ῥησσειν|ῥηττειν", "ῥηγνῠσθαι|ῥηγνυεσθαι|ῥησσεσθαι|ῥηττεσθαι", "ῥηγνῠσθαι|ῥηγνυεσθαι|ῥησσεσθαι|ῥηττεσθαι";
    infinitive_fut: "ῥηξειν", "ῥηξεσθαι", "ῥᾰγησεσθαι";
    infinitive_aor: "ῥηξαι", "ῥηξᾰσθαι", "ῥᾰγηναι|ῥηχθηναι";
    infinitive_perf: "ἐρρηχεναι|ἐρρωγεναι", "ἐρρηχθαι", _;
    participle_pres: "ῥηγνῡς|ῥηγνυων|ῥησσων|ῥηττων", "ῥηγνῠμενος|ῥηγνυομενος|ῥησσομενος|ῥηττομενος", "ῥηγνῠμενος|ῥηγνυομενος|ῥησσομενος|ῥηττομενος";
    participle_fut: "ῥηξων", "ῥηξομενος", "ῥᾰγησομενος";
    participle_aor: "ῥηξᾱς", "ῥηξᾰμενος", "ῥᾰγεις|ῥηχθεις";
    participle_perf: "ἐρρηχως|ἐρρωγως", "ἐρρηγμενος", _;
    ),
    // Ἐξῐστημῐ: 1 words (ἐξῐστημῐ). Like Λιστραω, except in 307 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐξῐστημῐ", 1, "~ξῐ~ημῐ", like Λιστραω;
    ind_act_1: "~ξῐ~ημῐ|~ξε~ηκω", _, "~ξῐ~ᾰμεν|~ξε~ηκομεν";
    ind_act_2: "~ξῐ~ης|~ξε~ηκεις", "~ξῐ~ᾰτον|~ξε~ηκετον", "~ξῐ~ᾰτε|~ξε~ηκετε";
    ind_act_3: "~ξῐ~ησῐ|~ξῐ~ησῐν|~ξε~ηκει", "~ξῐ~ᾰτον|~ξε~ηκετον", "~ξῐ~ᾱσῐ|~ξῐ~ᾱσῐν|~ξε~ηκουσῐ|~ξε~ηκουσῐν";
    ind_mid_1: "~ξῐ~ᾰμαι|~ξε~ηκομαι", _, "~ξῐ~ᾰμεθᾰ|~ξε~ηκομεθᾰ";
    ind_mid_2: "~ξῐ~ᾰσαι|~ξε~ηκῃ|~ξε~ηκει", "~ξῐ~ᾰσθον|~ξε~ηκεσθον", "~ξῐ~ᾰσθε|~ξε~ηκεσθε";
    ind_mid_3: "~ξῐ~ᾰται|~ξε~ηκεται", "~ξῐ~ᾰσθον|~ξε~ηκεσθον", "~ξῐ~ᾰνται|~ξε~ηκονται";
    ind_pass_1: "~ξῐ~ᾰμαι|~ξε~ηκομαι", _, "~ξῐ~ᾰμεθᾰ|~ξε~ηκομεθᾰ";
    ind_pass_2: "~ξῐ~ᾰσαι|~ξε~ηκῃ|~ξε~ηκει", "~ξῐ~ᾰσθον|~ξε~ηκεσθον", "~ξῐ~ᾰσθε|~ξε~ηκεσθε";
    ind_pass_3: "~ξῐ~ᾰται|~ξε~ηκεται", "~ξῐ~ᾰσθον|~ξε~ηκεσθον", "~ξῐ~ᾰνται|~ξε~ηκονται";
    subj_act_1: "~ξῐ~ω|~ξε~ηκω", _, "~ξῐ~ωμεν|~ξε~ηκωμεν";
    subj_act_2: "~ξῐ~ῃς|~ξε~ηκῃς", "~ξῐ~ητον|~ξε~ηκητον", "~ξῐ~ητε|~ξε~ηκητε";
    subj_act_3: "~ξῐ~ῃ|~ξε~ηκῃ", "~ξῐ~ητον|~ξε~ηκητον", "~ξῐ~ωσῐ|~ξῐ~ωσῐν|~ξε~ηκωσῐ|~ξε~ηκωσῐν";
    subj_mid_1: "~ξῐ~ωμαι|~ξε~ηκωμαι", _, "~ξῐ~ωμεθᾰ|~ξε~ηκωμεθᾰ";
    subj_mid_2: "~ξῐ~ῃ|~ξε~ηκῃ", "~ξῐ~ησθον|~ξε~ηκησθον", "~ξῐ~ησθε|~ξε~ηκησθε";
    subj_mid_3: "~ξῐ~ηται|~ξε~ηκηται", "~ξῐ~ησθον|~ξε~ηκησθον", "~ξῐ~ωνται|~ξε~ηκωνται";
    subj_pass_1: "~ξῐ~ωμαι|~ξε~ηκωμαι", _, "~ξῐ~ωμεθᾰ|~ξε~ηκωμεθᾰ";
    subj_pass_2: "~ξῐ~ῃ|~ξε~ηκῃ", "~ξῐ~ησθον|~ξε~ηκησθον", "~ξῐ~ησθε|~ξε~ηκησθε";
    subj_pass_3: "~ξῐ~ηται|~ξε~ηκηται", "~ξῐ~ησθον|~ξε~ηκησθον", "~ξῐ~ωνται|~ξε~ηκωνται";
    opt_act_1: "~ξῐ~αιην|~ξε~ηκοιμῐ", _, "~ξῐ~αῑμεν|~ξῐ~αιημεν|~ξε~ηκοιμεν";
    opt_act_2: "~ξῐ~αιης|~ξε~ηκοις", "~ξῐ~αῑτον|~ξῐ~αιητον|~ξε~ηκοιτον", "~ξῐ~αῑτε|~ξῐ~αιητε|~ξε~ηκοιτε";
    opt_act_3: "~ξῐ~αιη|~ξε~ηκοι", "~ξῐ~αιτην|~ξῐ~αιητην|~ξε~ηκοιτην", "~ξῐ~αῑεν|~ξῐ~αιησᾰν|~ξε~ηκοιεν";
    opt_mid_1: "~ξῐ~αιμην|~ξε~ηκοιμην", _, "~ξῐ~αιμεθᾰ|~ξε~ηκοιμεθᾰ";
    opt_mid_2: "~ξῐ~αιο|~ξε~ηκοιο", "~ξῐ~αισθον|~ξε~ηκοισθον", "~ξῐ~αισθε|~ξε~ηκοισθε";
    opt_mid_3: "~ξῐ~αιτο|~ξε~ηκοιτο", "~ξῐ~αισθην|~ξε~ηκοισθην", "~ξῐ~αιντο|~ξε~ηκοιντο";
    opt_pass_1: "~ξῐ~αιμην|~ξε~ηκοιμην", _, "~ξῐ~αιμεθᾰ|~ξε~ηκοιμεθᾰ";
    opt_pass_2: "~ξῐ~αιο|~ξε~ηκοιο", "~ξῐ~αισθον|~ξε~ηκοισθον", "~ξῐ~αισθε|~ξε~ηκοισθε";
    opt_pass_3: "~ξῐ~αιτο|~ξε~ηκοιτο", "~ξῐ~αισθην|~ξε~ηκοισθην", "~ξῐ~αιντο|~ξε~ηκοιντο";
    imp_act_2: "~ξῐ~η|~ξε~ηκε", "~ξῐ~ᾰτον|~ξε~ηκετον", "~ξῐ~ᾰτε|~ξε~ηκετε";
    imp_act_3: "~ξῐ~ᾰτω|~ξε~ηκετω", "~ξῐ~ᾰτων|~ξε~ηκετων", "~ξῐ~ᾰντων|~ξε~ηκοντων";
    imp_mid_2: "~ξῐ~ᾰσο|~ξῐ~ω|~ξε~ηκου", "~ξῐ~ᾰσθον|~ξε~ηκεσθον", "~ξῐ~ᾰσθε|~ξε~ηκεσθε";
    imp_mid_3: "~ξῐ~ᾰσθω|~ξε~ηκεσθω", "~ξῐ~ᾰσθων|~ξε~ηκεσθων", "~ξῐ~ᾰσθων|~ξε~ηκεσθων";
    imp_pass_2: "~ξῐ~ᾰσο|~ξῐ~ω|~ξε~ηκου", "~ξῐ~ᾰσθον|~ξε~ηκεσθον", "~ξῐ~ᾰσθε|~ξε~ηκεσθε";
    imp_pass_3: "~ξῐ~ᾰσθω|~ξε~ηκεσθω", "~ξῐ~ᾰσθων|~ξε~ηκεσθων", "~ξῐ~ᾰσθων|~ξε~ηκεσθων";
    imperfect_act_1: "~ξῑ~ην|~ξι~ασκον", _, "~ξῑ~ᾰμεν|~ξι~ασκομεν";
    imperfect_act_2: "~ξῑ~ης|~ξι~ασκες", "~ξῑ~ᾰτον|~ξι~ασκετον", "~ξῑ~ᾰτε|~ξι~ασκετε";
    imperfect_act_3: "~ξῑ~η|~ξι~ασκε|~ξι~ασκεν", "~ξῑ~ᾰτην|~ξι~ασκετην", "~ξῑ~ᾰσᾰν|~ξι~ασκον";
    imperfect_mid_1: "~ξῑ~ᾰμην|~ξι~ασκομην", _, "~ξῑ~ᾰμεθᾰ|~ξι~ασκομεθᾰ";
    imperfect_mid_2: "~ξῑ~ᾰσο|~ξι~ασκου", "~ξῑ~ᾰσθον|~ξι~ασκεσθον", "~ξῑ~ᾰσθε|~ξι~ασκεσθε";
    imperfect_mid_3: "~ξῑ~ᾰτο|~ξι~ασκετο", "~ξῑ~ᾰσθην|~ξι~ασκεσθην", "~ξῑ~ᾰντο|~ξι~ασκοντο";
    imperfect_pass_1: "~ξῑ~ᾰμην|~ξι~ασκομην", _, "~ξῑ~ᾰμεθᾰ|~ξι~ασκομεθᾰ";
    imperfect_pass_2: "~ξῑ~ᾰσο|~ξι~ασκου", "~ξῑ~ᾰσθον|~ξι~ασκεσθον", "~ξῑ~ᾰσθε|~ξι~ασκεσθε";
    imperfect_pass_3: "~ξῑ~ᾰτο|~ξι~ασκετο", "~ξῑ~ᾰσθην|~ξι~ασκεσθην", "~ξῑ~ᾰντο|~ξι~ασκοντο";
    future_ind_act_1: "~κ~ᾱσεω", _, "~κ~ᾱσεομες";
    future_ind_act_2: "~κ~ᾱσεῑς", "~κ~ᾱσεῑτον", "~κ~ᾱσεῑτε";
    future_ind_act_3: "~κ~ᾱσεῑ", _, "~κ~ᾱσεοντῐ";
    future_ind_mid_1: "~κ~ᾱσεομαι", _, "~κ~ᾱσεομεθᾰ";
    future_ind_mid_2: "~κ~ᾱσῃ", "~κ~ᾱσεῑσθον", "~κ~ᾱσεῑσθε";
    future_ind_mid_3: "~κ~ᾱσεῑται", _, "~κ~ᾱσεονται";
    future_opt_act_1: "~κ~ᾱσεοιμῐ", _, "~κ~ᾱσεοιμες";
    future_opt_act_2: "~κ~ᾱσεοις", "~κ~ᾱσεοιτον", "~κ~ᾱσεοιτε";
    future_opt_act_3: "~κ~ᾱσεοι", "~κ~ᾱσεοιτᾱν", "~κ~ᾱσεοιεν";
    future_opt_mid_1: "~κ~ᾱσεοιμᾱν", _, "~κ~ᾱσεοιμεθᾰ";
    future_opt_mid_2: "~κ~ᾱσεοιο", "~κ~ᾱσεοισθον", "~κ~ᾱσεοισθε";
    future_opt_mid_3: "~κ~ᾱσεοιτο", "~κ~ᾱσεοισθᾱν", "~κ~ᾱσεοιντο";
    aorist_ind_act_1: "~ξε~ησᾰ|~ξε~ην|~κ~ᾰσκον", _, "~ξε~ησᾰμεν|~ξε~ημεν|~κ~ᾰσκομεν";
    aorist_ind_act_2: "~ξε~ησᾰς|~ξε~ης|~κ~ᾰσκες", "~ξε~ησᾰτον|~ξε~ητον|~κ~ᾰσκετον", "~ξε~ησᾰτε|~ξε~ητε|~κ~ᾰσκετε";
    aorist_ind_act_3: "~ξε~ησε|~ξε~ησεν|~ξε~η|~κ~ᾰσκε|~κ~ᾰσκεν", "~ξε~ησᾰτην|~ξε~ητην|~κ~ᾰσκετην", "~ξε~ησᾰν|~κ~ᾰσκον";
    aorist_ind_mid_1: "~ξε~ησᾰμην|~ξε~ᾰμην|~κ~ᾰσκομην", _, "~ξε~ησᾰμεθᾰ|~ξε~ᾰμεθᾰ|~κ~ᾰσκομεθᾰ";
    aorist_ind_mid_2: "~ξε~ησω|~ξε~ω|~κ~ᾰσκου", "~ξε~ησᾰσθον|~ξε~ᾰσθον|~κ~ᾰσκεσθον", "~ξε~ησᾰσθε|~ξε~ᾰσθε|~κ~ᾰσκεσθε";
    aorist_ind_mid_3: "~ξε~ησᾰτο|~ξε~ᾰτο|~κ~ᾰσκετο", "~ξε~ησᾰσθην|~ξε~ᾰσθην|~κ~ᾰσκεσθην", "~ξε~ησᾰντο|~ξε~ᾰντο|~κ~ᾰσκοντο";
    aorist_ind_pass_1: "~ξε~ᾰθην|~ξε~ην", _, "~ξε~ᾰθημεν|~ξε~ημεν";
    aorist_ind_pass_2: "~ξε~ᾰθης|~ξε~ης", "~ξε~ᾰθητον|~ξε~ητον", "~ξε~ᾰθητε|~ξε~ητε";
    aorist_ind_pass_3: "~ξε~ᾰθη|~ξε~η", "~ξε~ᾰθητην|~ξε~ητην", "~ξε~ᾰθησᾰν|~ξε~ησᾰν";
    aorist_subj_act_1: "~κ~ησω|~κ~ω|~κ~ᾰσκω", _, "~κ~ησωμεν|~κ~ωμεν|~κ~ᾰσκωμεν";
    aorist_subj_act_2: "~κ~ησῃς|~κ~ῃς|~κ~ᾰσκῃς", "~κ~ησητον|~κ~ητον|~κ~ᾰσκητον", "~κ~ησητε|~κ~ητε|~κ~ᾰσκητε";
    aorist_subj_act_3: "~κ~ησῃ|~κ~ῃ|~κ~ᾰσκῃ", "~κ~ησητον|~κ~ητον|~κ~ᾰσκητον", "~κ~ησωσῐ|~κ~ησωσῐν|~κ~ωσῐ|~κ~ωσῐν|~κ~ᾰσκωσῐ|~κ~ᾰσκωσῐν";
    aorist_subj_mid_1: "~κ~ησωμαι|~κ~ωμαι|~κ~ᾰσκωμαι", _, "~κ~ησωμεθᾰ|~κ~ωμεθᾰ|~κ~ᾰσκωμεθᾰ";
    aorist_subj_mid_2: "~κ~ησῃ|~κ~ῃ|~κ~ᾰσκῃ", "~κ~ησησθον|~κ~ησθον|~κ~ᾰσκησθον", "~κ~ησησθε|~κ~ησθε|~κ~ᾰσκησθε";
    aorist_subj_mid_3: "~κ~ησηται|~κ~ηται|~κ~ᾰσκηται", "~κ~ησησθον|~κ~ησθον|~κ~ᾰσκησθον", "~κ~ησωνται|~κ~ωνται|~κ~ᾰσκωνται";
    aorist_subj_pass_1: "~κ~ᾰθω|~κ~ω", _, "~κ~ᾰθωμεν|~κ~ωμεν";
    aorist_subj_pass_2: "~κ~ᾰθῃς|~κ~ῃς", "~κ~ᾰθητον|~κ~ητον", "~κ~ᾰθητε|~κ~ητε";
    aorist_subj_pass_3: "~κ~ᾰθῃ|~κ~ῃ", "~κ~ᾰθητον|~κ~ητον", "~κ~ᾰθωσῐ|~κ~ᾰθωσῐν|~κ~ωσῐ|~κ~ωσῐν";
    aorist_opt_act_1: "~κ~ησαιμῐ|~κ~αιην|~κ~ᾰσκοιμῐ", _, "~κ~ησαιμεν|~κ~αῑμεν|~κ~αιημεν|~κ~ᾰσκοιμεν";
    aorist_opt_act_2: "~κ~ησειᾰς|~κ~ησαις|~κ~αιης|~κ~ᾰσκοις", "~κ~ησαιτον|~κ~αῑτον|~κ~αιητον|~κ~ᾰσκοιτον", "~κ~ησαιτε|~κ~αῑτε|~κ~αιητε|~κ~ᾰσκοιτε";
    aorist_opt_act_3: "~κ~ησειε|~κ~ησειεν|~κ~ησαι|~κ~αιη|~κ~ᾰσκοι", "~κ~ησαιτην|~κ~αιτην|~κ~αιητην|~κ~ᾰσκοιτην", "~κ~ησειᾰν|~κ~ησαιεν|~κ~αῑεν|~κ~αιησᾰν|~κ~ᾰσκοιεν";
    aorist_opt_mid_1: "~κ~ησαιμην|~κ~αιμην|~κ~ᾰσκοιμην", _, "~κ~ησαιμεθᾰ|~κ~αιμεθᾰ|~κ~ᾰσκοιμεθᾰ";
    aorist_opt_mid_2: "~κ~ησαιο|~κ~αῑο|~κ~ᾰσκοιο", "~κ~ησαισθον|~κ~αῑσθον|~κ~ᾰσκοισθον", "~κ~ησαισθε|~κ~αῑσθε|~κ~ᾰσκοισθε";
    aorist_opt_mid_3: "~κ~ησαιτο|~κ~αῑτο|~κ~ᾰσκοιτο", "~κ~ησαισθην|~κ~αισθην|~κ~ᾰσκοισθην", "~κ~ησαιντο|~κ~αῑντο|~κ~ᾰσκοιντο";
    aorist_opt_pass_1: "~κ~ᾰθειην|~κ~ειην", _, "~κ~ᾰθεῑμεν|~κ~ᾰθειημεν|~κ~εῑμεν|~κ~ειημεν";
    aorist_opt_pass_2: "~κ~ᾰθειης|~κ~ειης", "~κ~ᾰθεῑτον|~κ~ᾰθειητον|~κ~εῑτον|~κ~ειητον", "~κ~ᾰθεῑτε|~κ~ᾰθειητε|~κ~εῑτε|~κ~ειητε";
    aorist_opt_pass_3: "~κ~ᾰθειη|~κ~ειη", "~κ~ᾰθειτην|~κ~ᾰθειητην|~κ~ειτην|~κ~ειητην", "~κ~ᾰθεῑεν|~κ~ᾰθειησᾰν|~κ~εῑεν|~κ~ειησᾰν";
    aorist_imp_act_2: "~κ~ησον|~κ~ηθῐ|~κ~ᾰσκε", "~κ~ησᾰτον|~κ~ητον|~κ~ᾰσκετον", "~κ~ησᾰτε|~κ~ητε|~κ~ᾰσκετε";
    aorist_imp_act_3: "~κ~ησᾰτω|~κ~ητω|~κ~ᾰσκετω", "~κ~ησᾰτων|~κ~ητων|~κ~ᾰσκετων", "~κ~ησᾰντων|~κ~ᾰντων|~κ~ᾰσκοντων";
    aorist_imp_mid_2: "~κ~ησαι|~κ~ω|~κ~ᾰσκοῡ", "~κ~ησᾰσθον|~κ~ᾰσθον|~κ~ᾰσκεσθον", "~κ~ησᾰσθε|~κ~ᾰσθε|~κ~ᾰσκεσθε";
    aorist_imp_mid_3: "~κ~ησᾰσθω|~κ~ᾰσθω|~κ~ᾰσκεσθω", "~κ~ησᾰσθων|~κ~ᾰσθων|~κ~ᾰσκεσθων", "~κ~ησᾰσθων|~κ~ᾰσθων|~κ~ᾰσκεσθων";
    aorist_imp_pass_2: "~κ~ᾰθητῐ|~κ~ηθῐ", "~κ~ᾰθητον|~κ~ητον", "~κ~ᾰθητε|~κ~ητε";
    aorist_imp_pass_3: "~κ~ᾰθητω|~κ~ητω", "~κ~ᾰθητων|~κ~ητων", "~κ~ᾰθεντων|~κ~εντων";
    perfect_ind_act_1: "~ξε~ᾰκᾰ|~ξε~ηκᾰ", _, "~ξε~ᾰκᾰμεν|~ξε~ηκᾰμεν|~ξε~ᾰμεν";
    perfect_ind_act_2: "~ξε~ᾰκᾰς|~ξε~ηκᾰς", "~ξε~ᾰκᾰτον|~ξε~ηκᾰτον|~ξε~ᾰτον", "~ξε~ᾰκᾰτε|~ξε~ηκᾰτε|~ξε~ᾰτε";
    perfect_ind_act_3: "~ξε~ᾰκε|~ξε~ᾰκεν|~ξε~ηκε|~ξε~ηκεν", "~ξε~ᾰκᾰτον|~ξε~ηκᾰτον|~ξε~ᾰτον", "~ξε~ᾰκᾱσῐ|~ξε~ᾰκᾱσῐν|~ξε~ηκᾱσῐ|~ξε~ηκᾱσῐν|~ξε~ᾱσῐ|~ξε~ᾱσῐν";
    perfect_ind_mid_1: "~ξε~ᾰμαι", _, "~ξε~ᾰμεθᾰ";
    perfect_ind_mid_2: "~ξε~ᾰσαι", "~ξε~ᾰσθον", "~ξε~ᾰσθε";
    perfect_ind_mid_3: "~ξε~ᾰται", _, "~ξε~ᾰνται";
    perfect_ind_pass_1: _, _, "~ξε~ᾰμεθᾰ";
    perfect_ind_pass_2: _, _, "~ξε~ᾰσθε";
    perfect_subj_act_1: "~ξε~ᾰκω|~ξε~ηκω|~ξε~ω", _, "~ξε~ᾰκωμεν|~ξε~ηκωμεν|~ξε~ωμεν";
    perfect_subj_act_2: "~ξε~ᾰκῃς|~ξε~ηκῃς|~ξε~ῃς", "~ξε~ᾰκητον|~ξε~ηκητον|~ξε~ητον", "~ξε~ᾰκητε|~ξε~ηκητε|~ξε~ητε";
    perfect_subj_act_3: "~ξε~ᾰκῃ|~ξε~ηκῃ|~ξε~ῃ", "~ξε~ᾰκητον|~ξε~ηκητον|~ξε~ητον", "~ξε~ᾰκωσῐ|~ξε~ᾰκωσῐν|~ξε~ηκωσῐ|~ξε~ηκωσῐν|~ξε~ωσῐ|~ξε~ωσῐν";
    perfect_opt_act_1: "~ξε~ᾰκοιμῐ|~ξε~ᾰκοιην|~ξε~ηκοιμῐ|~ξε~ηκοιην|~ξε~αιην", _, "~ξε~ᾰκοιμεν|~ξε~ηκοιμεν|~ξε~αῑμεν|~ξε~αιημεν";
    perfect_opt_act_2: "~ξε~ᾰκοις|~ξε~ᾰκοιης|~ξε~ηκοις|~ξε~ηκοιης|~ξε~αιης", "~ξε~ᾰκοιτον|~ξε~ηκοιτον|~ξε~αῑτον|~ξε~αιητον", "~ξε~ᾰκοιτε|~ξε~ηκοιτε|~ξε~αῑτε|~ξε~αιητε";
    perfect_opt_act_3: "~ξε~ᾰκοι|~ξε~ᾰκοιη|~ξε~ηκοι|~ξε~ηκοιη|~ξε~αιη", "~ξε~ᾰκοιτην|~ξε~ηκοιτην|~ξε~αιτην|~ξε~αιητην", "~ξε~ᾰκοιεν|~ξε~ηκοιεν|~ξε~αῑεν|~ξε~αιησᾰν";
    perfect_imp_act_2: "~ξε~ᾰκε|~ξε~ηκε|~ξε~ᾰθῐ", "~ξε~ᾰκετον|~ξε~ηκετον|~ξε~ᾰτον", "~ξε~ᾰκετε|~ξε~ηκετε|~ξε~ᾰτε";
    perfect_imp_act_3: "~ξε~ᾰκετω|~ξε~ηκετω|~ξε~ᾰτω", "~ξε~ᾰκετων|~ξε~ηκετων|~ξε~ᾰτων", "~ξε~ᾰκοντων|~ξε~ηκοντων|~ξε~ᾰντων";
    perfect_imp_mid_2: "~ξε~ᾰσο", _, _;
    perfect_imp_mid_3: "~ξε~ᾰσθω", "~ξε~ᾰσθων", _;
    perfect_imp_pass_2: "~ξε~ᾰσο", _, _;
    perfect_imp_pass_3: "~ξε~ᾰσθω", "~ξε~ᾰσθων", _;
    pluperfect_act_1: "~ξε~ηκειν|~ξε~ηκη|~ξει~ηκειν|~ξει~ηκη", _, "~ξε~ηκεμεν|~ξει~ηκεμεν|~ξε~ᾰμεν";
    pluperfect_act_2: "~ξε~ηκεις|~ξε~ηκης|~ξει~ηκεις|~ξει~ηκης", "~ξε~ηκετον|~ξει~ηκετον|~ξε~ᾰτον", "~ξε~ηκετε|~ξει~ηκετε|~ξε~ᾰτε";
    pluperfect_act_3: "~ξε~ηκει|~ξε~ηκειν|~ξει~ηκει|~ξει~ηκειν", "~ξε~ηκετην|~ξει~ηκετην|~ξε~ᾰτην", "~ξε~ηκεσᾰν|~ξει~ηκεσᾰν|~ξε~ᾰσᾰν";
    infinitive_pres: "~ξῐ~ᾰναι|~ξε~ηκειν", "~ξῐ~ᾰσθαι|~ξε~ηκεσθαι", "~ξῐ~ᾰσθαι|~ξε~ηκεσθαι";
    infinitive_fut: "~κ~ᾱσεῑν", "~κ~ᾱσεῑσθαι", _;
    infinitive_aor: "~κ~ησαι|~κ~ηναι|~κ~ᾰσκεῑν", "~κ~ησᾰσθαι|~κ~ᾰσθαι|~κ~ᾰσκεσθαι", "~κ~ᾰθηναι|~κ~ηναι";
    infinitive_perf: "~ξε~ᾰκεναι|~ξε~ηκεναι|~ξε~ᾰναι", "~ξε~ᾰσθαι", _;
    participle_pres: "~ξῐ~ᾱς|~ξε~ηκων", "~ξῐ~ᾰμενος|~ξε~ηκομενος", "~ξῐ~ᾰμενος|~ξε~ηκομενος";
    participle_fut: "~κ~ᾱσεων", "~κ~ᾱσεομενος", _;
    participle_aor: "~κ~ησᾱς|~κ~ᾱς|~κ~ᾰσκων", "~κ~ησᾰμενος|~κ~ᾰμενος|~κ~ᾰσκομενος", "~κ~ᾰθεις|~κ~εις";
    participle_perf: "~ξε~ᾰκως|~ξε~ηκως|~ξε~ως", "~ξε~ᾰμενος", _;
    ),
    // Ἱστημῐ: 1 words (ἱστημῐ). Like Ἀφῑημῐ, except in 295 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἱστημῐ", 1, "ἱ~ημῐ", like Ἀφῑημῐ;
    ind_act_1: "ἱ~ημῐ", _, "ἱ~ᾰμεν";
    ind_act_2: "ἱ~ης", _, _;
    ind_act_3: "ἱ~ησῐ|ἱ~ησῐν", _, "ἱ~ᾱσῐ|ἱ~ᾱσῐν";
    ind_mid_1: "ἱ~ᾰμαι", _, "ἱ~ᾰμεθᾰ";
    ind_mid_2: "ἱ~ᾰσαι", "ἱ~ᾰσθον", "ἱ~ᾰσθε";
    ind_mid_3: "ἱ~ᾰται", _, "ἱ~ᾰνται";
    ind_pass_1: _, _, "ἱ~ᾰμεθᾰ";
    ind_pass_2: _, _, "ἱ~ᾰσθε";
    ind_pass_3: "ἱ~ᾰται", _, "ἱ~ᾰνται";
    subj_act_1: "ἱ~ω", _, "ἱ~ωμεν";
    subj_act_2: "ἱ~ῃς", "ἱ~ητον", "ἱ~ητε";
    subj_act_3: "ἱ~ῃ", _, "ἱ~ωσῐ|ἱ~ωσῐν";
    subj_mid_1: "ἱ~ωμαι", _, "ἱ~ωμεθᾰ";
    subj_mid_2: "ἱ~ῃ", "ἱ~ησθον", "ἱ~ησθε";
    subj_mid_3: "ἱ~ηται", _, "ἱ~ωνται";
    subj_pass_1: _, _, "ἱ~ωμεθᾰ";
    subj_pass_2: _, _, "ἱ~ησθε";
    subj_pass_3: "ἱ~ηται", _, "ἱ~ωνται";
    opt_act_1: "ἱ~αιην", _, "ἱ~αῑμεν|ἱ~αιημεν";
    opt_act_2: "ἱ~αιης", "ἱ~αῑτον|ἱ~αιητον", "ἱ~αῑτε|ἱ~αιητε";
    opt_act_3: "ἱ~αιη", "ἱ~αιτην|ἱ~αιητην", "ἱ~αῑεν|ἱ~αιησᾰν";
    opt_mid_1: "ἱ~αιμην", _, "ἱ~αιμεθᾰ";
    opt_mid_2: "ἱ~αιο", "ἱ~αισθον", "ἱ~αισθε";
    opt_mid_3: "ἱ~αιτο", "ἱ~αισθην", "ἱ~αιντο";
    opt_pass_1: _, _, "ἱ~αιμεθᾰ";
    opt_pass_2: _, "ἱ~αισθον", "ἱ~αισθε";
    opt_pass_3: "ἱ~αιτο", "ἱ~αισθην", "ἱ~αιντο";
    imp_act_2: "ἱ~η", "ἱ~ᾰτον", "ἱ~ᾰτε";
    imp_act_3: "ἱ~ᾰτω", "ἱ~ᾰτων", "ἱ~ᾰντων";
    imp_mid_2: "ἱ~ᾰσο|ἱ~ω", _, "ἱ~ᾰσθε";
    imp_mid_3: "ἱ~ᾰσθω", "ἱ~ᾰσθων", "ἱ~ᾰσθων";
    imp_pass_2: "ἱ~ᾰσο|ἱ~ω", _, "ἱ~ᾰσθε";
    imp_pass_3: _, "ἱ~ᾰσθων", _;
    imperfect_act_1: "ἱ~ην", _, "ἱ~ᾰμεν";
    imperfect_act_2: "ἱ~ης", "ἱ~ᾰτον", "ἱ~ᾰτε";
    imperfect_act_3: "ἱ~η", "ἱ~ᾰτην", "ἱ~ᾰσᾰν";
    imperfect_mid_1: "ἱ~ᾰμην", _, "ἱ~ᾰμεθᾰ";
    imperfect_mid_2: "ἱ~ᾰσο", "ἱ~ᾰσθον", "ἱ~ᾰσθε";
    imperfect_mid_3: "ἱ~ᾰτο", "ἱ~ᾰσθην", "ἱ~ᾰντο";
    imperfect_pass_1: _, _, "ἱ~ᾰμεθᾰ";
    imperfect_pass_2: _, _, "ἱ~ᾰσθε";
    imperfect_pass_3: "ἱ~ᾰτο", _, _;
    future_ind_pass_1: "~ᾰθησομαι", _, "~ᾰθησομεθᾰ";
    future_ind_pass_2: "~ᾰθησῃ|~ᾰθησει", "~ᾰθησεσθον", "~ᾰθησεσθε";
    future_ind_pass_3: "~ᾰθησεται", _, "~ᾰθησονται";
    future_opt_pass_1: "~ᾰθησοιμην", _, "~ᾰθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰθησοιο", "~ᾰθησοισθον", "~ᾰθησοισθε";
    future_opt_pass_3: "~ᾰθησοιτο", "~ᾰθησοισθην", "~ᾰθησοιντο";
    aorist_ind_act_1: "ἐ~ησᾰ|ἐ~ην", _, "ἐ~ησᾰμεν|ἐ~ημεν";
    aorist_ind_act_2: "ἐ~ησᾰς|ἐ~ης", "ἐ~ησᾰτον|ἐ~ητον", "ἐ~ησᾰτε|ἐ~ητε";
    aorist_ind_act_3: "ἐ~ησε|ἐ~ησεν|ἐ~η", "ἐ~ησᾰτην|ἐ~ητην", "ἐ~ησᾰν";
    aorist_ind_mid_1: "ἐ~ησᾰμην|ἐ~ᾰμην", _, "ἐ~ησᾰμεθᾰ|ἐ~ᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ησω|ἐ~ω", "ἐ~ησᾰσθον|ἐ~ᾰσθον", "ἐ~ησᾰσθε|ἐ~ᾰσθε";
    aorist_ind_mid_3: "ἐ~ησᾰτο|ἐ~ᾰτο", "ἐ~ησᾰσθην|ἐ~ᾰσθην", "ἐ~ησᾰντο|ἐ~ᾰντο";
    aorist_ind_pass_1: "ἐ~ᾰθην", _, "ἐ~ᾰθημεν";
    aorist_ind_pass_2: "ἐ~ᾰθης", "ἐ~ᾰθητον", "ἐ~ᾰθητε";
    aorist_ind_pass_3: "ἐ~ᾰθη", "ἐ~ᾰθητην", "ἐ~ᾰθησᾰν";
    aorist_subj_act_1: "~ησω|~ω", _, "~ησωμεν|~ωμεν";
    aorist_subj_act_2: "~ησῃς|~ῃς", "~ησητον|~ητον", "~ησητε|~ητε";
    aorist_subj_act_3: "~ησῃ|~ῃ", "~ησητον|~ητον", "~ησωσῐ|~ησωσῐν|~ωσῐ|~ωσῐν";
    aorist_subj_mid_1: "~ησωμαι|~ωμαι", _, "~ησωμεθᾰ|~ωμεθᾰ";
    aorist_subj_mid_2: "~ησῃ|~ῃ", "~ησησθον|~ησθον", "~ησησθε|~ησθε";
    aorist_subj_mid_3: "~ησηται|~ηται", "~ησησθον|~ησθον", "~ησωνται|~ωνται";
    aorist_subj_pass_1: "~ᾰθω", _, "~ᾰθωμεν";
    aorist_subj_pass_2: "~ᾰθῃς", "~ᾰθητον", "~ᾰθητε";
    aorist_subj_pass_3: "~ᾰθῃ", _, "~ᾰθωσῐ|~ᾰθωσῐν";
    aorist_opt_act_1: "~ησαιμῐ|~αιην", _, "~ησαιμεν|~αῑμεν|~αιημεν";
    aorist_opt_act_2: "~ησειᾰς|~ησαις|~αιης", "~ησαιτον|~αῑτον|~αιητον", "~ησαιτε|~αῑτε|~αιητε";
    aorist_opt_act_3: "~ησειε|~ησειεν|~ησαι|~αιη", "~ησαιτην|~αιτην|~αιητην", "~ησειᾰν|~ησαιεν|~αῑεν|~αιησᾰν";
    aorist_opt_mid_1: "~ησαιμην|~αιμην", _, "~ησαιμεθᾰ|~αιμεθᾰ";
    aorist_opt_mid_2: "~ησαιο|~αῑο", "~ησαισθον|~αῑσθον", "~ησαισθε|~αῑσθε";
    aorist_opt_mid_3: "~ησαιτο|~αῑτο", "~ησαισθην|~αισθην", "~ησαιντο|~αῑντο";
    aorist_opt_pass_1: "~ᾰθειην", _, "~ᾰθεῑμεν|~ᾰθειημεν";
    aorist_opt_pass_2: "~ᾰθειης", "~ᾰθεῑτον|~ᾰθειητον", "~ᾰθεῑτε|~ᾰθειητε";
    aorist_opt_pass_3: "~ᾰθειη", "~ᾰθειτην|~ᾰθειητην", "~ᾰθεῑεν|~ᾰθειησᾰν";
    aorist_imp_act_2: "~ησον|~ηθῐ", "~ησᾰτον|~ητον", "~ησᾰτε|~ητε";
    aorist_imp_act_3: "~ησᾰτω|~ητω", "~ησᾰτων|~ητων", "~ησᾰντων|~ᾰντων";
    aorist_imp_mid_2: "~ησαι|~ω", "~ησᾰσθον|~ᾰσθον", "~ησᾰσθε|~ᾰσθε";
    aorist_imp_mid_3: "~ησᾰσθω|~ᾰσθω", "~ησᾰσθων|~ᾰσθων", "~ησᾰσθων|~ᾰσθων";
    aorist_imp_pass_2: "~ᾰθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰθητω", "~ᾰθητων", "~ᾰθεντων";
    perfect_ind_act_1: "ἑ~ᾰκᾰ|ἑ~ηκᾰ", _, "ἑ~ᾰκᾰμεν|ἑ~ηκᾰμεν|ἑ~ᾰμεν";
    perfect_ind_act_2: "ἑ~ᾰκᾰς|ἑ~ηκᾰς", "ἑ~ᾰκᾰτον|ἑ~ηκᾰτον|ἑ~ᾰτον", "ἑ~ᾰκᾰτε|ἑ~ηκᾰτε|ἑ~ᾰτε";
    perfect_ind_act_3: "ἑ~ᾰκε|ἑ~ᾰκεν|ἑ~ηκε|ἑ~ηκεν", "ἑ~ᾰκᾰτον|ἑ~ηκᾰτον|ἑ~ᾰτον", "ἑ~ᾰκᾱσῐ|ἑ~ᾰκᾱσῐν|ἑ~ηκᾱσῐ|ἑ~ηκᾱσῐν|ἑ~ᾱσῐ|ἑ~ᾱσῐν";
    perfect_ind_mid_1: "ἑ~ᾰμαι", _, "ἑ~ᾰμεθᾰ";
    perfect_ind_mid_2: "ἑ~ᾰσαι", "ἑ~ᾰσθον", "ἑ~ᾰσθε";
    perfect_ind_mid_3: "ἑ~ᾰται", _, "ἑ~ᾰνται";
    perfect_ind_pass_1: _, _, "ἑ~ᾰμεθᾰ";
    perfect_ind_pass_2: _, _, "ἑ~ᾰσθε";
    perfect_subj_act_1: "ἑ~ᾰκω|ἑ~ηκω|ἑ~ω", _, "ἑ~ᾰκωμεν|ἑ~ηκωμεν|ἑ~ωμεν";
    perfect_subj_act_2: "ἑ~ᾰκῃς|ἑ~ηκῃς|ἑ~ῃς", "ἑ~ᾰκητον|ἑ~ηκητον|ἑ~ητον", "ἑ~ᾰκητε|ἑ~ηκητε|ἑ~ητε";
    perfect_subj_act_3: "ἑ~ᾰκῃ|ἑ~ηκῃ|ἑ~ῃ", "ἑ~ᾰκητον|ἑ~ηκητον|ἑ~ητον", "ἑ~ᾰκωσῐ|ἑ~ᾰκωσῐν|ἑ~ηκωσῐ|ἑ~ηκωσῐν|ἑ~ωσῐ|ἑ~ωσῐν";
    perfect_opt_act_1: "ἑ~ᾰκοιμῐ|ἑ~ᾰκοιην|ἑ~ηκοιμῐ|ἑ~ηκοιην|ἑ~αιην", _, "ἑ~ᾰκοιμεν|ἑ~ηκοιμεν|ἑ~αῑμεν|ἑ~αιημεν";
    perfect_opt_act_2: "ἑ~ᾰκοις|ἑ~ᾰκοιης|ἑ~ηκοις|ἑ~ηκοιης|ἑ~αιης", "ἑ~ᾰκοιτον|ἑ~ηκοιτον|ἑ~αῑτον|ἑ~αιητον", "ἑ~ᾰκοιτε|ἑ~ηκοιτε|ἑ~αῑτε|ἑ~αιητε";
    perfect_opt_act_3: "ἑ~ᾰκοι|ἑ~ᾰκοιη|ἑ~ηκοι|ἑ~ηκοιη|ἑ~αιη", "ἑ~ᾰκοιτην|ἑ~ηκοιτην|ἑ~αιτην|ἑ~αιητην", "ἑ~ᾰκοιεν|ἑ~ηκοιεν|ἑ~αῑεν|ἑ~αιησᾰν";
    perfect_imp_act_2: "ἑ~ᾰκε|ἑ~ηκε|ἑ~ᾰθῐ", "ἑ~ᾰκετον|ἑ~ηκετον|ἑ~ᾰτον", "ἑ~ᾰκετε|ἑ~ηκετε|ἑ~ᾰτε";
    perfect_imp_act_3: "ἑ~ᾰκετω|ἑ~ηκετω|ἑ~ᾰτω", "ἑ~ᾰκετων|ἑ~ηκετων|ἑ~ᾰτων", "ἑ~ᾰκοντων|ἑ~ηκοντων|ἑ~ᾰντων";
    perfect_imp_mid_2: "ἑ~ᾰσο", _, _;
    perfect_imp_mid_3: "ἑ~ᾰσθω", "ἑ~ᾰσθων", _;
    perfect_imp_pass_2: "ἑ~ᾰσο", _, _;
    perfect_imp_pass_3: "ἑ~ᾰσθω", "ἑ~ᾰσθων", _;
    pluperfect_act_1: "εἱ~ηκειν|εἱ~ηκη", _, "εἱ~ηκεμεν|ἑ~ᾰμεν";
    pluperfect_act_2: "εἱ~ηκεις|εἱ~ηκης", "εἱ~ηκετον|ἑ~ᾰτον", "εἱ~ηκετε|ἑ~ᾰτε";
    pluperfect_act_3: "εἱ~ηκει|εἱ~ηκειν", "εἱ~ηκετην|ἑ~ᾰτην", "εἱ~ηκεσᾰν|ἑ~ᾰσᾰν";
    future_perfect_ind_act_1: "ἑ~ηξω", _, "ἑ~ηξομεν";
    future_perfect_ind_act_2: "ἑ~ηξεις", "ἑ~ηξετον", "ἑ~ηξετε";
    future_perfect_ind_act_3: "ἑ~ηξει", _, "ἑ~ηξουσῐ|ἑ~ηξουσῐν";
    future_perfect_ind_mid_1: "ἑ~ηξομαι", _, "ἑ~ηξομεθᾰ";
    future_perfect_ind_mid_2: "ἑ~ηξῃ|ἑ~ηξει", "ἑ~ηξεσθον", "ἑ~ηξεσθε";
    future_perfect_ind_mid_3: "ἑ~ηξεται", _, "ἑ~ηξονται";
    future_perfect_ind_pass_2: "ἑ~ηξῃ|ἑ~ηξει", _, _;
    future_perfect_opt_act_1: "ἑ~ηξοιμῐ", _, "ἑ~ηξοιμεν";
    future_perfect_opt_act_2: "ἑ~ηξοις", "ἑ~ηξοιτον", "ἑ~ηξοιτε";
    future_perfect_opt_act_3: "ἑ~ηξοι", "ἑ~ηξοιτην", "ἑ~ηξοιεν";
    future_perfect_opt_mid_1: "ἑ~ηξοιμην", _, "ἑ~ηξοιμεθᾰ";
    future_perfect_opt_mid_2: "ἑ~ηξοιο", "ἑ~ηξοισθον", "ἑ~ηξοισθε";
    future_perfect_opt_mid_3: "ἑ~ηξοιτο", "ἑ~ηξοισθην", "ἑ~ηξοιντο";
    infinitive_pres: "ἱ~ᾰναι", "ἱ~ᾰσθαι", _;
    infinitive_fut: _, _, "~ᾰθησεσθαι";
    infinitive_aor: "~ησαι|~ηναι", "~ησᾰσθαι|~ᾰσθαι", "~ᾰθηναι";
    infinitive_perf: "ἑ~ᾰκεναι|ἑ~ηκεναι|ἑ~ᾰναι", "ἑ~ᾰσθαι", _;
    infinitive_futp: "ἑ~ηξειν", "ἑ~ηξεσθαι", _;
    participle_pres: "ἱ~ᾱς", "ἱ~ᾰμενος", _;
    participle_fut: _, _, "~ᾰθησομενος";
    participle_aor: "~ησᾱς|~ᾱς", "~ησᾰμενος|~ᾰμενος", "~ᾰθεις";
    participle_perf: "ἑ~ᾰκως|ἑ~ηκως|ἑ~ως", "ἑ~ᾰμενος", _;
    participle_futp: "ἑ~ηξων", "ἑ~ηξομενος", _;
    ),
    // Εἰμῐ: 1 words (εἰμῐ).
    conjugation!("εἰμῐ", 1, "εἰμῐ";
    ind_act_1: "εἰμῐ", _, "ἐσμεν";
    ind_act_2: "εἰ", _, _;
    subj_act_1: "ὠ", _, "ὠμεν";
    subj_act_2: "ᾐς", "ἠτον", "ἠτε";
    subj_act_3: "ᾐ", _, "ὠσῐ|ὠσῐν";
    opt_act_1: "εἰην", _, "εἰμεν|εἰημεν";
    opt_act_2: "εἰης", "εἰτον|εἰητον", "εἰτε|εἰητε";
    opt_act_3: "εἰη", "εἰτην|εἰητην", "εἰεν|εἰησᾰν";
    imp_act_2: "ἰσθῐ", "ἐστον", "ἐστε";
    imp_act_3: "ἐστω|ἠτω", "ἐστων", "ἐστων|ἐστωσᾰν|ὀντων";
    imperfect_act_1: _, _, "ἠμεν";
    imperfect_act_2: "ἠσ|ἠσθᾰ", "ἠστον|ἠτον", "ἠτε";
    imperfect_act_3: "ἠν", "ἠστην|ἠτην", "ἠσᾰν";
    future_ind_mid_1: "ἐσομαι", _, "ἐσομεθᾰ";
    future_ind_mid_2: "ἐσῃ|ἐσει", "ἐσεσθον", "ἐσεσθε";
    future_ind_mid_3: "ἐσται|ἐσεται", _, "ἐσονται";
    future_opt_mid_1: "ἐσοιμην", _, "ἐσοιμεθᾰ";
    future_opt_mid_2: "ἐσοιο", "ἐσοισθον", "ἐσοισθε";
    future_opt_mid_3: "ἐσοιτο", "ἐσοισθην", "ἐσοιντο";
    infinitive_pres: "εἰναι", _, _;
    infinitive_fut: _, "ἐσεσθαι", _;
    participle_pres: "ὠν", _, _;
    participle_fut: _, "ἐσομενος", _;
    ),
    // Ἱημῐ: 1 words (ἱημῐ). Like Εἰμῐ, except in 309 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἱημῐ", 1, "ἱημῐ", like Εἰμῐ;
    ind_act_1: "ἱημῐ", _, "ἱεμεν";
    ind_act_2: "ἱης", _, _;
    ind_act_3: "ἱησῐ|ἱησῐν", _, "ἱᾱσι|ἱᾱσιν";
    ind_mid_1: "ἱεμαι", _, "ἱεμεθᾰ";
    ind_mid_2: "ἱεσαι", "ἱεσθον", "ἱεσθε";
    ind_mid_3: "ἱεται", _, "ἱενται";
    ind_pass_1: _, _, "ἱεμεθᾰ";
    ind_pass_2: _, _, "ἱεσθε";
    ind_pass_3: "ἱεται", _, "ἱενται";
    subj_act_1: "ἱω", _, "ἱωμεν";
    subj_act_2: "ἱῃς", "ἱητον", "ἱητε";
    subj_act_3: "ἱῃ", _, "ἱωσῐ|ἱωσῐν";
    subj_mid_1: "ἱωμαι", _, "ἱωμεθᾰ";
    subj_mid_2: "ἱῃ", "ἱησθον", "ἱησθε";
    subj_mid_3: "ἱηται", _, "ἱωνται";
    subj_pass_1: _, _, "ἱωμεθᾰ";
    subj_pass_2: _, _, "ἱησθε";
    subj_pass_3: "ἱηται", _, "ἱωνται";
    opt_act_1: "ἱειην", _, "ἱεῑμεν|ἱειημεν";
    opt_act_2: "ἱειης", "ἱεῑτον|ἱειητον", "ἱεῑτε|ἱειητε";
    opt_act_3: "ἱειη", "ἱειτην|ἱειητην", "ἱεῑεν|ἱειησᾰν";
    opt_mid_1: "ἱειμην", _, "ἱειμεθᾰ|ἱοιμεθᾰ";
    opt_mid_2: "ἱεῑο", "ἱεῑσθον|ἱοῑσθον", "ἱεῑσθε|ἱοῑσθε";
    opt_mid_3: "ἱεῑτο|ἱοῑτο", "ἱεισθην|ἱοισθην", "ἱεῑντο|ἱοῑντο";
    opt_pass_1: _, _, "ἱειμεθᾰ|ἱοιμεθᾰ";
    opt_pass_2: _, "ἱεῑσθον|ἱοῑσθον", "ἱεῑσθε|ἱοῑσθε";
    opt_pass_3: "ἱεῑτο|ἱοῑτο", "ἱεισθην|ἱοισθην", "ἱεῑντο|ἱοῑντο";
    imp_act_2: "ἱει", "ἱετον", "ἱετε";
    imp_act_3: "ἱετω", "ἱετων", "ἱεντων";
    imp_mid_2: "ἱεσο|ἱου", _, "ἱεσθε";
    imp_mid_3: "ἱεσθω", "ἱεσθων", "ἱεσθων";
    imp_pass_2: "ἱεσο|ἱου", _, "ἱεσθε";
    imp_pass_3: _, "ἱεσθων", _;
    imperfect_act_1: "ἱην", _, "ἱεμεν";
    imperfect_act_2: "ἱεις", "ἱετον", "ἱετε";
    imperfect_act_3: "ἱει|ἱειν", "ἱετην", "ἱεσᾰν";
    imperfect_mid_1: "ἱεμην", _, "ἱεμεθᾰ";
    imperfect_mid_2: "ἱεσο", "ἱεσθον", "ἱεσθε";
    imperfect_mid_3: "ἱετο", "ἱεσθην", "ἱεντο";
    imperfect_pass_1: _, _, "ἱεμεθᾰ";
    imperfect_pass_2: _, _, "ἱεσθε";
    imperfect_pass_3: "ἱετο", _, _;
    future_ind_act_1: "ἡσω", _, "ἡσομεν";
    future_ind_act_2: "ἡσεις", "ἡσετον", "ἡσετε";
    future_ind_act_3: "ἡσει", _, "ἡσουσῐ|ἡσουσῐν";
    future_ind_mid_1: "ἡσομαι", _, "ἡσομεθᾰ";
    future_ind_mid_2: "ἡσῃ|ἡσει", "ἡσεσθον", "ἡσεσθε";
    future_ind_mid_3: "ἡσεται", _, "ἡσονται";
    future_ind_pass_1: "ἑθησομαι", _, "ἑθησομεθᾰ";
    future_ind_pass_2: "ἑθησῃ|ἑθησει", "ἑθησεσθον", "ἑθησεσθε";
    future_ind_pass_3: "ἑθησεται", _, "ἑθησονται";
    future_opt_act_1: "ἡσοιμῐ", _, "ἡσοιμεν";
    future_opt_act_2: "ἡσοις", "ἡσοιτον", "ἡσοιτε";
    future_opt_act_3: "ἡσοι", "ἡσοιτην", "ἡσοιεν";
    future_opt_mid_1: "ἡσοιμην", _, "ἡσοιμεθᾰ";
    future_opt_mid_2: "ἡσοιο", "ἡσοισθον", "ἡσοισθε";
    future_opt_mid_3: "ἡσοιτο", "ἡσοισθην", "ἡσοιντο";
    future_opt_pass_1: "ἑθησοιμην", _, "ἑθησοιμεθᾰ";
    future_opt_pass_2: "ἑθησοιο", "ἑθησοισθον", "ἑθησοισθε";
    future_opt_pass_3: "ἑθησοιτο", "ἑθησοισθην", "ἑθησοιντο";
    aorist_ind_act_1: "ἡκᾰ", _, "εἱμεν|ἡκᾰμεν";
    aorist_ind_act_2: "ἡκᾰς", "εἱτον|ἡκᾰτον", "εἱτε|ἡκᾰτε";
    aorist_ind_act_3: "ἡκε|ἡκεν", "εἱτην|ἡκᾰτην", "εἱσᾰν|ἡκᾰν";
    aorist_ind_mid_1: "εἱμην|ἡκᾰμην", _, "εἱμεθᾰ|ἡκᾰμεθᾰ";
    aorist_ind_mid_2: "εἱσο|ἡκω", "εἱσθον|ἡκᾰσθον", "εἱσθε|ἡκᾰσθε";
    aorist_ind_mid_3: "εἱτο|ἡκᾰτο", "εἱσθην|ἡκᾰσθην", "εἱντο|ἡκᾰντο";
    aorist_ind_pass_1: "εἱθην", _, "εἱθημεν";
    aorist_ind_pass_2: "εἱθης", "εἱθητον", "εἱθητε";
    aorist_ind_pass_3: "εἱθη", "εἱθητην", "εἱθησᾰν";
    aorist_subj_act_1: "ὡ", _, "ὡμεν";
    aorist_subj_act_2: "ᾑς", "ἡτον", "ἡτε";
    aorist_subj_act_3: "ᾑ|ᾑσι|ᾑσιν", _, "ὡσῐ|ὡσῐν";
    aorist_subj_mid_1: "ὡμαι", _, "ὡμεθᾰ";
    aorist_subj_mid_2: "ᾑ", "ἡσθον", "ἡσθε";
    aorist_subj_mid_3: "ἡται", _, "ὡνται";
    aorist_subj_pass_1: "ἑθω", _, "ἑθωμεν";
    aorist_subj_pass_2: "ἑθῃς", "ἑθητον", "ἑθητε";
    aorist_subj_pass_3: "ἑθῃ", _, "ἑθωσῐ|ἑθωσῐν";
    aorist_opt_act_1: "εἱην", _, "εἱμεν|εἱημεν";
    aorist_opt_act_2: "εἱης", "εἱτον|εἱητον", "εἱτε|εἱητε";
    aorist_opt_act_3: "εἱη", "εἱτην|εἱητην", "εἱεν|εἱησᾰν";
    aorist_opt_mid_1: "εἱμην", _, "εἱμεθᾰ|οἱμεθᾰ";
    aorist_opt_mid_2: "εἱο", "εἱσθον", "εἱσθε|οἱσθε";
    aorist_opt_mid_3: "εἱτο|οἱτο", "εἱσθην", "εἱντο|οἱντο";
    aorist_opt_pass_1: "ἑθειην", _, "ἑθεῑμεν|ἑθειημεν";
    aorist_opt_pass_2: "ἑθειης", "ἑθεῑτον|ἑθειητον", "ἑθεῑτε|ἑθειητε";
    aorist_opt_pass_3: "ἑθειη", "ἑθειτην|ἑθειητην", "ἑθεῑεν|ἑθειησᾰν";
    aorist_imp_act_2: "ἑς", "ἑτον", "ἑτε";
    aorist_imp_act_3: "ἑτω", "ἑτων", "ἑντων";
    aorist_imp_mid_2: "οὑ", "ἑσθον", "ἑσθε";
    aorist_imp_mid_3: "ἑσθω", "ἑσθων", _;
    aorist_imp_pass_2: "ἑθητῐ", _, _;
    aorist_imp_pass_3: "ἑθητω", "ἑθητων", "ἑθεντων";
    perfect_ind_act_1: "εἱκᾰ|ἑωκᾰ", _, "εἱκᾰμεν|ἑωκᾰμεν";
    perfect_ind_act_2: "εἱκᾰς|ἑωκᾰς", "εἱκᾰτον|ἑωκᾰτον", "εἱκᾰτε|ἑωκᾰτε";
    perfect_ind_act_3: "εἱκε|εἱκεν|ἑωκε|ἑωκεν", "εἱκᾰτον|ἑωκᾰτον", "εἱκᾱσῐ|εἱκᾱσῐν|ἑωκᾱσῐ|ἑωκᾱσῐν";
    perfect_ind_mid_1: "εἱμαι|ἑωμαι", _, "εἱμεθᾰ|ἑωμεθᾰ";
    perfect_ind_mid_2: "εἱσαι|ἑωσαι", "εἱσθον|ἑωσθον", "εἱσθε|ἑωσθε";
    perfect_ind_mid_3: "εἱται|ἑωται", "εἱσθον|ἑωσθον", "εἱνται|ἑωνται";
    perfect_ind_pass_1: "εἱμαι|ἑωμαι", _, "εἱμεθᾰ|ἑωμεθᾰ";
    perfect_ind_pass_2: "εἱσαι|ἑωσαι", "εἱσθον|ἑωσθον", "εἱσθε|ἑωσθε";
    perfect_ind_pass_3: "εἱται|ἑωται", "εἱσθον|ἑωσθον", "εἱνται|ἑωνται";
    perfect_subj_act_1: "εἱκω|ἑωκω", _, "εἱκωμεν|ἑωκωμεν";
    perfect_subj_act_2: "εἱκῃς|ἑωκῃς", "εἱκητον|ἑωκητον", "εἱκητε|ἑωκητε";
    perfect_subj_act_3: "εἱκῃ|ἑωκῃ", "εἱκητον|ἑωκητον", "εἱκωσῐ|εἱκωσῐν|ἑωκωσῐ|ἑωκωσῐν";
    perfect_opt_act_1: "εἱκοιμῐ|εἱκοιην|ἑωκοιμῐ|ἑωκοιην", _, "εἱκοιμεν|ἑωκοιμεν";
    perfect_opt_act_2: "εἱκοις|εἱκοιης|ἑωκοις|ἑωκοιης", "εἱκοιτον|ἑωκοιτον", "εἱκοιτε|ἑωκοιτε";
    perfect_opt_act_3: "εἱκοι|εἱκοιη|ἑωκοι|ἑωκοιη", "εἱκοιτην|ἑωκοιτην", "εἱκοιεν|ἑωκοιεν";
    perfect_imp_act_2: "εἱκε|ἑωκε", "εἱκετον|ἑωκετον", "εἱκετε|ἑωκετε";
    perfect_imp_act_3: "εἱκετω|ἑωκετω", "εἱκετων|ἑωκετων", "εἱκοντων|ἑωκοντων";
    perfect_imp_mid_2: "εἱσο|ἑωσο", "εἱσθον|ἑωσθον", "εἱσθε|ἑωσθε";
    perfect_imp_mid_3: "εἱσθω|ἑωσθω", "εἱσθων|ἑωσθων", "εἱσθωσαν|εἱσθων|ἑωσθων";
    perfect_imp_pass_2: "εἱσο|ἑωσο", "εἱσθον|ἑωσθον", "εἱσθε|ἑωσθε";
    perfect_imp_pass_3: "εἱσθω|ἑωσθω", "εἱσθων|ἑωσθων", "εἱσθωσαν|εἱσθων|ἑωσθων";
    pluperfect_mid_1: "εἱμην", _, "εἱμεθᾰ";
    pluperfect_mid_2: "εἱσο", "εἱσθον", "εἱσθε";
    pluperfect_mid_3: "εἱτο", "εἱσθην", _;
    pluperfect_pass_1: _, _, "εἱμεθᾰ";
    pluperfect_pass_2: _, _, "εἱσθε";
    pluperfect_pass_3: _, "εἱσθην", "εἱντο";
    infinitive_pres: "ἱεναι", "ἱεσθαι", _;
    infinitive_fut: "ἡσειν", "ἡσεσθαι", "ἑθησεσθαι";
    infinitive_aor: "εἱναι", "ἑσθαι", "ἑθηναι";
    infinitive_perf: "εἱκεναι|ἑωκεναι", "εἱσθαι|ἑωσθαι", "εἱσθαι|ἑωσθαι";
    participle_pres: "ἱεις", "ἱεμενος", _;
    participle_fut: "ἡσων", "ἡσομενος", "ἑθησομενος";
    participle_aor: "εἱς", "ἑμενος", "ἑθεις";
    participle_perf: "εἱκως|ἑωκως", "εἱμενος|ἑωμενος", "εἱμενος|ἑωμενος";
    ),
    // Ἐπεντῐθημῐ: 1 words (ἐπεντῐθημῐ). Like Προσερχομαι, except in 271 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἐπεντῐθημῐ", 1, "~τῐ~ημῐ", like Προσερχομαι;
    ind_act_1: "~%ῐ~ημῐ", _, "~%ῐ~εμεν";
    ind_act_2: "~%ῐ~ης", _, _;
    ind_act_3: "~%ῐ~ησῐ|~%ῐ~ησῐν", _, "~%ῐ~εᾱσῐ|~%ῐ~εᾱσῐν";
    ind_mid_1: "~%ῐ~εμαι", _, "~%ῐ~εμεθᾰ";
    ind_mid_2: "~%ῐ~εσαι", "~%ῐ~εσθον", "~%ῐ~εσθε";
    ind_mid_3: "~%ῐ~εται", _, "~%ῐ~ενται";
    ind_pass_1: _, _, "~%ῐ~εμεθᾰ";
    ind_pass_2: _, _, "~%ῐ~εσθε";
    ind_pass_3: "~%ῐ~εται", _, "~%ῐ~ενται";
    subj_act_1: "~%ῐ~ω", _, "~%ῐ~ωμεν";
    subj_act_2: "~%ῐ~ῃς", "~%ῐ~ητον", "~%ῐ~ητε";
    subj_act_3: "~%ῐ~ῃ", _, "~%ῐ~ωσῐ|~%ῐ~ωσῐν";
    subj_mid_1: "~%ῐ~ωμαι", _, "~%ῐ~ωμεθᾰ";
    subj_mid_2: "~%ῐ~ῃ", "~%ῐ~ησθον", "~%ῐ~ησθε";
    subj_mid_3: "~%ῐ~ηται", _, "~%ῐ~ωνται";
    subj_pass_1: _, _, "~%ῐ~ωμεθᾰ";
    subj_pass_2: _, _, "~%ῐ~ησθε";
    subj_pass_3: "~%ῐ~ηται", _, "~%ῐ~ωνται";
    opt_act_1: "~%ῐ~ειην", _, "~%ῐ~εῑμεν|~%ῐ~ειημεν";
    opt_act_2: "~%ῐ~ειης", "~%ῐ~εῑτον|~%ῐ~ειητον", "~%ῐ~εῑτε|~%ῐ~ειητε";
    opt_act_3: "~%ῐ~ειη", "~%ῐ~ειτην|~%ῐ~ειητην", "~%ῐ~εῑεν|~%ῐ~ειησᾰν";
    opt_mid_1: "~%ῐ~ειμην", _, "~%ῐ~ειμεθᾰ|~%ῐ~οιμεθᾰ";
    opt_mid_2: "~%ῐ~εῑο", "~%ῐ~εῑσθον|~%ῐ~οῑσθον", "~%ῐ~εῑσθε|~%ῐ~οῑσθε";
    opt_mid_3: "~%ῐ~εῑτο|~%ῐ~οῑτο", "~%ῐ~εισθην|~%ῐ~οισθην", "~%ῐ~εῑντο|~%ῐ~οῑντο";
    opt_pass_1: _, _, "~%ῐ~ειμεθᾰ|~%ῐ~οιμεθᾰ";
    opt_pass_2: _, "~%ῐ~εῑσθον|~%ῐ~οῑσθον", "~%ῐ~εῑσθε|~%ῐ~οῑσθε";
    opt_pass_3: "~%ῐ~εῑτο|~%ῐ~οῑτο", "~%ῐ~εισθην|~%ῐ~οισθην", "~%ῐ~εῑντο|~%ῐ~οῑντο";
    imp_act_2: "~%ῐ~ει", "~%ῐ~ετον", "~%ῐ~ετε";
    imp_act_3: "~%ῐ~ετω", "~%ῐ~ετων", "~%ῐ~εντων";
    imp_mid_2: "~%ῐ~εσο|~%ῐ~ου", _, "~%ῐ~εσθε";
    imp_mid_3: "~%ῐ~εσθω", "~%ῐ~εσθων", "~%ῐ~εσθων";
    imp_pass_2: "~%ῐ~εσο|~%ῐ~ου", _, "~%ῐ~εσθε";
    imp_pass_3: _, "~%ῐ~εσθων", _;
    imperfect_act_1: "~ε%ῐ~ουν", _, "~ε%ῐ~οῡμεν";
    imperfect_act_2: "~ε%ῐ~εις", "~ε%ῐ~εῑτον", "~ε%ῐ~εῑτε";
    imperfect_act_3: "~ε%ῐ~ει", "~ε%ῐ~ειτην", "~ε%ῐ~ουν";
    imperfect_mid_1: "~ε%ῐ~ουμην", _, "~ε%ῐ~ουμεθᾰ";
    imperfect_mid_2: "~ε%ῐ~οῡ", "~ε%ῐ~εῑσθον", "~ε%ῐ~εῑσθε";
    imperfect_mid_3: "~ε%ῐ~εῑτο", "~ε%ῐ~εισθην", "~ε%ῐ~οῡντο";
    imperfect_pass_1: _, _, "~ε%ῐ~ουμεθᾰ";
    imperfect_pass_2: _, _, "~ε%ῐ~εῑσθε";
    imperfect_pass_3: "~ε%ῐ~εῑτο", _, _;
    future_ind_act_1: "~~ησω", _, "~~ησομεν";
    future_ind_act_2: "~~ησεις", "~~ησετον", "~~ησετε";
    future_ind_act_3: "~~ησει", _, "~~ησουσῐ|~~ησουσῐν";
    future_ind_mid_1: "~~ησομαι", _, "~~ησομεθᾰ";
    future_ind_mid_2: "~~ησῃ|~~ησει", "~~ησεσθον", "~~ησεσθε";
    future_ind_mid_3: "~~ησεται", _, "~~ησονται";
    future_ind_pass_1: "~%ε~ησομαι", _, "~%ε~ησομεθᾰ";
    future_ind_pass_2: "~%ε~ησῃ|~%ε~ησει", "~%ε~ησεσθον", "~%ε~ησεσθε";
    future_ind_pass_3: "~%ε~ησεται", _, "~%ε~ησονται";
    future_opt_act_1: "~~ησοιμῐ", _, "~~ησοιμεν";
    future_opt_act_2: "~~ησοις", "~~ησοιτον", "~~ησοιτε";
    future_opt_act_3: "~~ησοι", "~~ησοιτην", "~~ησοιεν";
    future_opt_mid_1: "~~ησοιμην", _, "~~ησοιμεθᾰ";
    future_opt_mid_2: "~~ησοιο", "~~ησοισθον", "~~ησοισθε";
    future_opt_mid_3: "~~ησοιτο", "~~ησοισθην", "~~ησοιντο";
    future_opt_pass_1: "~%ε~ησοιμην", _, "~%ε~ησοιμεθᾰ";
    future_opt_pass_2: "~%ε~ησοιο", "~%ε~ησοισθον", "~%ε~ησοισθε";
    future_opt_pass_3: "~%ε~ησοιτο", "~%ε~ησοισθην", "~%ε~ησοιντο";
    aorist_ind_act_1: "~ε~ηκᾰ", _, "~ε~εμεν|~ε~ηκᾰμεν";
    aorist_ind_act_2: "~ε~ηκᾰς", "~ε~ετον|~ε~ηκᾰτον", "~ε~ετε|~ε~ηκᾰτε";
    aorist_ind_act_3: "~ε~ηκε|~ε~ηκεν", "~ε~ετην|~ε~ηκᾰτην", "~ε~εσᾰν|~ε~ηκᾰν";
    aorist_ind_mid_1: "~ε~εμην|~ε~ηκᾰμην", _, "~ε~εμεθᾰ|~ε~ηκᾰμεθᾰ";
    aorist_ind_mid_2: "~ε~ου|~ε~ηκω", "~ε~εσθον|~ε~ηκᾰσθον", "~ε~εσθε|~ε~ηκᾰσθε";
    aorist_ind_mid_3: "~ε~ετο|~ε~ηκᾰτο", "~ε~εσθην|~ε~ηκᾰσθην", "~ε~εντο|~ε~ηκᾰντο";
    aorist_ind_pass_1: "~ετε~ην", _, "~ετε~ημεν";
    aorist_ind_pass_2: "~ετε~ης", "~ετε~ητον", "~ετε~ητε";
    aorist_ind_pass_3: "~ετε~η", "~ετε~ητην", "~ετε~ησᾰν";
    aorist_subj_act_1: "~~ω", _, "~~ωμεν";
    aorist_subj_act_2: "~~ῃς", "~~ητον", "~~ητε";
    aorist_subj_act_3: "~~ῃ", _, "~~ωσῐ|~~ωσῐν";
    aorist_subj_pass_1: "~τε~ω", _, "~τε~ωμεν";
    aorist_subj_pass_2: "~τε~ῃς", "~τε~ητον", "~τε~ητε";
    aorist_subj_pass_3: "~τε~ῃ", _, "~τε~ωσῐ|~τε~ωσῐν";
    aorist_opt_act_1: "~~ειην", _, "~~εῑμεν|~~ειημεν";
    aorist_opt_act_2: "~~ειης", "~~εῑτον|~~ειητον", "~~εῑτε|~~ειητε";
    aorist_opt_act_3: "~~ειη", "~~ειτην|~~ειητην", "~~εῑεν|~~ειησᾰν";
    aorist_opt_mid_1: "~~ειμην", _, "~~ειμεθᾰ";
    aorist_opt_mid_2: "~~εῑο", "~~εῑσθον", "~~εῑσθε";
    aorist_opt_mid_3: "~~εῑτο|~~οῑτο", "~~εισθην", "~~εῑντο";
    aorist_opt_pass_1: "~τε~ειην", _, "~τε~εῑμεν|~τε~ειημεν";
    aorist_opt_pass_2: "~τε~ειης", "~τε~εῑτον|~τε~ειητον", "~τε~εῑτε|~τε~ειητε";
    aorist_opt_pass_3: "~τε~ειη", "~τε~ειτην|~τε~ειητην", "~τε~εῑεν|~τε~ειησᾰν";
    aorist_imp_act_2: "~~ες", "~~ετον", "~~ετε";
    aorist_imp_act_3: "~~ετω", "~~ετων", "~~εντων";
    aorist_imp_mid_2: "~~ου", _, _;
    aorist_imp_pass_2: "~τε~ηθῐ", _, _;
    aorist_imp_pass_3: "~τε~ητω", "~τε~ητων", "~τε~εντων";
    perfect_ind_act_1: "~%ε~ηκᾰ|~%ε~εικᾰ", _, "~%ε~ηκᾰμεν|~%ε~εικᾰμεν";
    perfect_ind_act_2: "~%ε~ηκᾰς|~%ε~εικᾰς", "~%ε~ηκᾰτον|~%ε~εικᾰτον", "~%ε~ηκᾰτε|~%ε~εικᾰτε";
    perfect_ind_act_3: "~%ε~ηκε|~%ε~ηκεν|~%ε~εικε|~%ε~εικεν", "~%ε~ηκᾰτον|~%ε~εικᾰτον", "~%ε~ηκᾱσῐ|~%ε~ηκᾱσῐν|~%ε~εικᾱσῐ|~%ε~εικᾱσῐν";
    perfect_ind_mid_1: "~%ε~ειμαι", _, "~%ε~ειμεθᾰ";
    perfect_ind_mid_2: "~%ε~εισαι", "~%ε~εισθον", "~%ε~εισθε";
    perfect_ind_mid_3: "~%ε~ειται", _, "~%ε~εινται";
    perfect_ind_pass_1: _, _, "~%ε~ειμεθᾰ";
    perfect_ind_pass_2: _, _, "~%ε~εισθε";
    perfect_subj_act_1: "~%ε~ηκω|~%ε~εικω", _, "~%ε~ηκωμεν|~%ε~εικωμεν";
    perfect_subj_act_2: "~%ε~ηκῃς|~%ε~εικῃς", "~%ε~ηκητον|~%ε~εικητον", "~%ε~ηκητε|~%ε~εικητε";
    perfect_subj_act_3: "~%ε~ηκῃ|~%ε~εικῃ", "~%ε~ηκητον|~%ε~εικητον", "~%ε~ηκωσῐ|~%ε~ηκωσῐν|~%ε~εικωσῐ|~%ε~εικωσῐν";
    perfect_opt_act_1: "~%ε~ηκοιμῐ|~%ε~ηκοιην|~%ε~εικοιμῐ|~%ε~εικοιην", _, "~%ε~ηκοιμεν|~%ε~εικοιμεν";
    perfect_opt_act_2: "~%ε~ηκοις|~%ε~ηκοιης|~%ε~εικοις|~%ε~εικοιης", "~%ε~ηκοιτον|~%ε~εικοιτον", "~%ε~ηκοιτε|~%ε~εικοιτε";
    perfect_opt_act_3: "~%ε~ηκοι|~%ε~ηκοιη|~%ε~εικοι|~%ε~εικοιη", "~%ε~ηκοιτην|~%ε~εικοιτην", "~%ε~ηκοιεν|~%ε~εικοιεν";
    perfect_imp_act_2: "~%ε~ηκε|~%ε~εικε", "~%ε~ηκετον|~%ε~εικετον", "~%ε~ηκετε|~%ε~εικετε";
    perfect_imp_act_3: "~%ε~ηκετω|~%ε~εικετω", "~%ε~ηκετων|~%ε~εικετων", "~%ε~ηκοντων|~%ε~εικοντων";
    perfect_imp_mid_2: "~%ε~εισο", _, _;
    perfect_imp_mid_3: "~%ε~εισθω", "~%ε~εισθων", _;
    perfect_imp_pass_2: "~%ε~εισο", _, _;
    perfect_imp_pass_3: "~%ε~εισθω", "~%ε~εισθων", _;
    infinitive_pres: "~%ῐ~εναι", "~%ῐ~εσθαι", _;
    infinitive_fut: "~~ησειν", "~~ησεσθαι", "~%ε~ησεσθαι";
    infinitive_aor: "~~εῑναι", _, "~τε~ηναι";
    infinitive_perf: "~%ε~ηκεναι|~%ε~εικεναι", "~%ε~εῑσθαι", _;
    participle_pres: "~%ῐ~εις", "~%ῐ~εμενος", _;
    participle_fut: "~~ησων", "~~ησομενος", "~%ε~ησομενος";
    participle_aor: "~~εις", "~~εμενος", "~τε~εις";
    participle_perf: "~%ε~ηκως|~%ε~εικως", "~%ε~ειμενος", _;
    ),
    // Τῐθημῐ: 1 words (τῐθημῐ). Like Ἀφῑημῐ, except in 230 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("τῐθημῐ", 1, "τῐ~ημῐ", like Ἀφῑημῐ;
    ind_act_1: "%ῐ~ημῐ", _, "%ῐ~εμεν";
    ind_act_2: "%ῐ~ης", _, _;
    ind_act_3: "%ῐ~ησῐ|%ῐ~ησῐν", _, "%ῐ~εᾱσῐ|%ῐ~εᾱσῐν";
    ind_mid_1: "%ῐ~εμαι", _, "%ῐ~εμεθᾰ";
    ind_mid_2: "%ῐ~εσαι", "%ῐ~εσθον", "%ῐ~εσθε";
    ind_mid_3: "%ῐ~εται", _, "%ῐ~ενται";
    ind_pass_1: _, _, "%ῐ~εμεθᾰ";
    ind_pass_2: _, _, "%ῐ~εσθε";
    ind_pass_3: "%ῐ~εται", _, "%ῐ~ενται";
    subj_act_1: "%ῐ~ω", _, "%ῐ~ωμεν";
    subj_act_2: "%ῐ~ῃς", "%ῐ~ητον", "%ῐ~ητε";
    subj_act_3: "%ῐ~ῃ", _, "%ῐ~ωσῐ|%ῐ~ωσῐν";
    subj_mid_1: "%ῐ~ωμαι", _, "%ῐ~ωμεθᾰ";
    subj_mid_2: "%ῐ~ῃ", "%ῐ~ησθον", "%ῐ~ησθε";
    subj_mid_3: "%ῐ~ηται", _, "%ῐ~ωνται";
    subj_pass_1: _, _, "%ῐ~ωμεθᾰ";
    subj_pass_2: _, _, "%ῐ~ησθε";
    subj_pass_3: "%ῐ~ηται", _, "%ῐ~ωνται";
    opt_act_1: "%ῐ~ειην", _, "%ῐ~εῑμεν|%ῐ~ειημεν";
    opt_act_2: "%ῐ~ειης", "%ῐ~εῑτον|%ῐ~ειητον", "%ῐ~εῑτε|%ῐ~ειητε";
    opt_act_3: "%ῐ~ειη", "%ῐ~ειτην|%ῐ~ειητην", "%ῐ~εῑεν|%ῐ~ειησᾰν";
    opt_mid_1: "%ῐ~ειμην", _, "%ῐ~ειμεθᾰ|%ῐ~οιμεθᾰ";
    opt_mid_2: "%ῐ~εῑο", "%ῐ~εῑσθον|%ῐ~οῑσθον", "%ῐ~εῑσθε|%ῐ~οῑσθε";
    opt_mid_3: "%ῐ~εῑτο|%ῐ~οῑτο", "%ῐ~εισθην|%ῐ~οισθην", "%ῐ~εῑντο|%ῐ~οῑντο";
    opt_pass_1: _, _, "%ῐ~ειμεθᾰ|%ῐ~οιμεθᾰ";
    opt_pass_2: _, "%ῐ~εῑσθον|%ῐ~οῑσθον", "%ῐ~εῑσθε|%ῐ~οῑσθε";
    opt_pass_3: "%ῐ~εῑτο|%ῐ~οῑτο", "%ῐ~εισθην|%ῐ~οισθην", "%ῐ~εῑντο|%ῐ~οῑντο";
    imp_act_2: "%ῐ~ει", "%ῐ~ετον", "%ῐ~ετε";
    imp_act_3: "%ῐ~ετω", "%ῐ~ετων", "%ῐ~εντων";
    imp_mid_2: "%ῐ~εσο|%ῐ~ου", _, "%ῐ~εσθε";
    imp_mid_3: "%ῐ~εσθω", "%ῐ~εσθων", "%ῐ~εσθων";
    imp_pass_2: "%ῐ~εσο|%ῐ~ου", _, "%ῐ~εσθε";
    imp_pass_3: _, "%ῐ~εσθων", _;
    imperfect_act_1: "ἐ%ῐ~ουν", _, "ἐ%ῐ~οῡμεν";
    imperfect_act_2: "ἐ%ῐ~εις", "ἐ%ῐ~εῑτον", "ἐ%ῐ~εῑτε";
    imperfect_act_3: "ἐ%ῐ~ει", "ἐ%ῐ~ειτην", "ἐ%ῐ~ουν";
    imperfect_mid_1: "ἐ%ῐ~ουμην", _, "ἐ%ῐ~ουμεθᾰ";
    imperfect_mid_2: "ἐ%ῐ~οῡ", "ἐ%ῐ~εῑσθον", "ἐ%ῐ~εῑσθε";
    imperfect_mid_3: "ἐ%ῐ~εῑτο", "ἐ%ῐ~εισθην", "ἐ%ῐ~οῡντο";
    imperfect_pass_1: _, _, "ἐ%ῐ~ουμεθᾰ";
    imperfect_pass_2: _, _, "ἐ%ῐ~εῑσθε";
    imperfect_pass_3: "ἐ%ῐ~εῑτο", _, _;
    future_ind_pass_1: "%ε~ησομαι", _, "%ε~ησομεθᾰ";
    future_ind_pass_2: "%ε~ησῃ|%ε~ησει", "%ε~ησεσθον", "%ε~ησεσθε";
    future_ind_pass_3: "%ε~ησεται", _, "%ε~ησονται";
    future_opt_pass_1: "%ε~ησοιμην", _, "%ε~ησοιμεθᾰ";
    future_opt_pass_2: "%ε~ησοιο", "%ε~ησοισθον", "%ε~ησοισθε";
    future_opt_pass_3: "%ε~ησοιτο", "%ε~ησοισθην", "%ε~ησοιντο";
    aorist_ind_act_1: "ἐ~ηκᾰ", _, "ἐ~εμεν|ἐ~ηκᾰμεν";
    aorist_ind_act_2: "ἐ~ηκᾰς", "ἐ~ετον|ἐ~ηκᾰτον", "ἐ~ετε|ἐ~ηκᾰτε";
    aorist_ind_act_3: "ἐ~ηκε|ἐ~ηκεν", "ἐ~ετην|ἐ~ηκᾰτην", "ἐ~εσᾰν|ἐ~ηκᾰν";
    aorist_ind_mid_1: "ἐ~εμην|ἐ~ηκᾰμην", _, "ἐ~εμεθᾰ|ἐ~ηκᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ου|ἐ~ηκω", "ἐ~εσθον|ἐ~ηκᾰσθον", "ἐ~εσθε|ἐ~ηκᾰσθε";
    aorist_ind_mid_3: "ἐ~ετο|ἐ~ηκᾰτο", "ἐ~εσθην|ἐ~ηκᾰσθην", "ἐ~εντο|ἐ~ηκᾰντο";
    aorist_ind_pass_1: "ἐτε~ην", _, "ἐτε~ημεν";
    aorist_ind_pass_2: "ἐτε~ης", "ἐτε~ητον", "ἐτε~ητε";
    aorist_ind_pass_3: "ἐτε~η", "ἐτε~ητην", "ἐτε~ησᾰν";
    aorist_subj_act_3: _, _, "~ωσῐ|~ωσῐν";
    aorist_subj_pass_1: "τε~ω", _, "τε~ωμεν";
    aorist_subj_pass_2: "τε~ῃς", "τε~ητον", "τε~ητε";
    aorist_subj_pass_3: "τε~ῃ", _, "τε~ωσῐ|τε~ωσῐν";
    aorist_opt_act_1: _, _, "~εῑμεν|~ειημεν";
    aorist_opt_act_2: _, "~εῑτον|~ειητον", "~εῑτε|~ειητε";
    aorist_opt_act_3: _, "~ειτην|~ειητην", "~εῑεν|~ειησᾰν";
    aorist_opt_mid_3: "~εῑτο|~οῑτο", _, _;
    aorist_opt_pass_1: "τε~ειην", _, "τε~εῑμεν|τε~ειημεν";
    aorist_opt_pass_2: "τε~ειης", "τε~εῑτον|τε~ειητον", "τε~εῑτε|τε~ειητε";
    aorist_opt_pass_3: "τε~ειη", "τε~ειτην|τε~ειητην", "τε~εῑεν|τε~ειησᾰν";
    aorist_imp_pass_2: "τε~ηθῐ", _, _;
    aorist_imp_pass_3: "τε~ητω", "τε~ητων", "τε~εντων";
    perfect_ind_act_1: "%ε~ηκᾰ|%ε~εικᾰ", _, "%ε~ηκᾰμεν|%ε~εικᾰμεν";
    perfect_ind_act_2: "%ε~ηκᾰς|%ε~εικᾰς", "%ε~ηκᾰτον|%ε~εικᾰτον", "%ε~ηκᾰτε|%ε~εικᾰτε";
    perfect_ind_act_3: "%ε~ηκε|%ε~ηκεν|%ε~εικε|%ε~εικεν", "%ε~ηκᾰτον|%ε~εικᾰτον", "%ε~ηκᾱσῐ|%ε~ηκᾱσῐν|%ε~εικᾱσῐ|%ε~εικᾱσῐν";
    perfect_ind_mid_1: "%ε~ειμαι", _, "%ε~ειμεθᾰ";
    perfect_ind_mid_2: "%ε~εισαι", "%ε~εισθον", "%ε~εισθε";
    perfect_ind_mid_3: "%ε~ειται", _, "%ε~εινται";
    perfect_ind_pass_1: _, _, "%ε~ειμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~εισθε";
    perfect_subj_act_1: "%ε~ηκω|%ε~εικω", _, "%ε~ηκωμεν|%ε~εικωμεν";
    perfect_subj_act_2: "%ε~ηκῃς|%ε~εικῃς", "%ε~ηκητον|%ε~εικητον", "%ε~ηκητε|%ε~εικητε";
    perfect_subj_act_3: "%ε~ηκῃ|%ε~εικῃ", "%ε~ηκητον|%ε~εικητον", "%ε~ηκωσῐ|%ε~ηκωσῐν|%ε~εικωσῐ|%ε~εικωσῐν";
    perfect_opt_act_1: "%ε~ηκοιμῐ|%ε~ηκοιην|%ε~εικοιμῐ|%ε~εικοιην", _, "%ε~ηκοιμεν|%ε~εικοιμεν";
    perfect_opt_act_2: "%ε~ηκοις|%ε~ηκοιης|%ε~εικοις|%ε~εικοιης", "%ε~ηκοιτον|%ε~εικοιτον", "%ε~ηκοιτε|%ε~εικοιτε";
    perfect_opt_act_3: "%ε~ηκοι|%ε~ηκοιη|%ε~εικοι|%ε~εικοιη", "%ε~ηκοιτην|%ε~εικοιτην", "%ε~ηκοιεν|%ε~εικοιεν";
    perfect_imp_act_2: "%ε~ηκε|%ε~εικε", "%ε~ηκετον|%ε~εικετον", "%ε~ηκετε|%ε~εικετε";
    perfect_imp_act_3: "%ε~ηκετω|%ε~εικετω", "%ε~ηκετων|%ε~εικετων", "%ε~ηκοντων|%ε~εικοντων";
    perfect_imp_mid_2: "%ε~εισο", _, _;
    perfect_imp_mid_3: "%ε~εισθω", "%ε~εισθων", _;
    perfect_imp_pass_2: "%ε~εισο", _, _;
    perfect_imp_pass_3: "%ε~εισθω", "%ε~εισθων", _;
    pluperfect_act_1: "ἐ%ε~εικειν|ἐ%ε~εικη", _, "ἐ%ε~εικεμεν";
    pluperfect_act_2: "ἐ%ε~εικεις|ἐ%ε~εικης", "ἐ%ε~εικετον", "ἐ%ε~εικετε";
    pluperfect_act_3: "ἐ%ε~εικει|ἐ%ε~εικειν", "ἐ%ε~εικετην", "ἐ%ε~εικεσᾰν";
    pluperfect_mid_1: "ἐ%ε~ειμην", _, "ἐ%ε~ειμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~εισο", "ἐ%ε~εισθον", "ἐ%ε~εισθε";
    pluperfect_mid_3: "ἐ%ε~ειτο", "ἐ%ε~εισθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~ειμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~εισθε";
    pluperfect_pass_3: _, "ἐ%ε~εισθην", "ἐ%ε~ειντο";
    infinitive_pres: "%ῐ~εναι", "%ῐ~εσθαι", _;
    infinitive_fut: _, _, "%ε~ησεσθαι";
    infinitive_aor: _, _, "τε~ηναι";
    infinitive_perf: "%ε~ηκεναι|%ε~εικεναι", "%ε~εῑσθαι", _;
    participle_pres: "%ῐ~εις", "%ῐ~εμενος", _;
    participle_fut: _, _, "%ε~ησομενος";
    participle_aor: _, _, "τε~εις";
    participle_perf: "%ε~ηκως|%ε~εικως", "%ε~ειμενος", _;
    ),
    // Δῐδωμῐ: 1 words (δῐδωμῐ). Like Ἑπομαι, except in 312 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("δῐδωμῐ", 1, "~ῐδωμῐ", like Ἑπομαι;
    ind_act_1: "~ῐδωμῐ", _, "~ῐδομεν";
    ind_act_2: "~ῐδως", _, _;
    ind_act_3: "~ῐδωσῐ|~ῐδωσῐν", _, "~ῐδοᾱσῐ|~ῐδοᾱσῐν";
    ind_mid_1: "~ῐδομαι", _, "~ῐδομεθᾰ";
    ind_mid_2: "~ῐδοσαι", "~ῐδοσθον", "~ῐδοσθε";
    ind_mid_3: "~ῐδοται", _, "~ῐδονται";
    ind_pass_1: _, _, "~ῐδομεθᾰ";
    ind_pass_2: _, _, "~ῐδοσθε";
    ind_pass_3: "~ῐδοται", _, "~ῐδονται";
    subj_act_1: "~ῐδω", _, "~ῐδωμεν";
    subj_act_2: "~ῐδῳς", "~ῐδωτον", "~ῐδωτε";
    subj_act_3: "~ῐδῳ", _, "~ῐδωσῐ|~ῐδωσῐν";
    subj_mid_1: "~ῐδωμαι", _, "~ῐδωμεθᾰ";
    subj_mid_2: "~ῐδῳ", "~ῐδωσθον", "~ῐδωσθε";
    subj_mid_3: "~ῐδωται", _, "~ῐδωνται";
    subj_pass_1: _, _, "~ῐδωμεθᾰ";
    subj_pass_2: _, _, "~ῐδωσθε";
    subj_pass_3: "~ῐδωται", _, "~ῐδωνται";
    opt_act_1: "~ῐδοιην", _, "~ῐδοῑμεν|~ῐδοιημεν";
    opt_act_2: "~ῐδοιης", "~ῐδοῑτον|~ῐδοιητον", "~ῐδοῑτε|~ῐδοιητε";
    opt_act_3: "~ῐδοιη", "~ῐδοιτην|~ῐδοιητην", "~ῐδοῑεν|~ῐδοιησᾰν";
    opt_mid_1: "~ῐδοιμην", _, "~ῐδοιμεθᾰ";
    opt_mid_2: "~ῐδοῑο", "~ῐδοῑσθον", "~ῐδοῑσθε";
    opt_mid_3: "~ῐδοῑτο", "~ῐδοισθην", "~ῐδοῑντο";
    opt_pass_1: _, _, "~ῐδοιμεθᾰ";
    opt_pass_2: _, "~ῐδοῑσθον", "~ῐδοῑσθε";
    opt_pass_3: "~ῐδοῑτο", "~ῐδοισθην", "~ῐδοῑντο";
    imp_act_2: "~ῐδου", "~ῐδοτον", "~ῐδοτε";
    imp_act_3: "~ῐδοτω", "~ῐδοτων", "~ῐδοντων";
    imp_mid_2: "~ῐδοσο|~ῐδου", _, "~ῐδοσθε";
    imp_mid_3: "~ῐδοσθω", "~ῐδοσθων", "~ῐδοσθων";
    imp_pass_2: "~ῐδοσο|~ῐδου", _, "~ῐδοσθε";
    imp_pass_3: _, "~ῐδοσθων", _;
    imperfect_act_1: "ἐ~ῐδουν", _, "ἐ~ῐδομεν";
    imperfect_act_2: "ἐ~ῐδους", "ἐ~ῐδοτον", "ἐ~ῐδοτε";
    imperfect_act_3: "ἐ~ῐδου", "ἐ~ῐδοτην", "ἐ~ῐδοσᾰν|ἐ~ῐδουν";
    imperfect_mid_1: "ἐ~ῐδομην", _, "ἐ~ῐδομεθᾰ";
    imperfect_mid_2: "ἐ~ῐδοσο", "ἐ~ῐδοσθον", "ἐ~ῐδοσθε";
    imperfect_mid_3: "ἐ~ῐδοτο|ἐ~ῐδετο", "ἐ~ῐδοσθην", "ἐ~ῐδοντο";
    imperfect_pass_1: _, _, "ἐ~ῐδομεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ῐδοσθε";
    imperfect_pass_3: "ἐ~ῐδοτο|ἐ~ῐδετο", _, _;
    future_ind_act_1: "~ωσω|~ῐδωσω", _, "~ωσομεν|~ῐδωσομεν";
    future_ind_act_2: "~ωσεις|~ῐδωσεις", "~ωσετον|~ῐδωσετον", "~ωσετε|~ῐδωσετε";
    future_ind_act_3: "~ωσει|~ῐδωσει", "~ωσετον|~ῐδωσετον", "~ωσουσῐ|~ωσουσῐν|~ῐδωσουσῐ|~ῐδωσουσῐν";
    future_ind_mid_1: "~ωσομαι|~ῐδωσομαι", _, "~ωσομεθᾰ|~ῐδωσομεθᾰ";
    future_ind_mid_2: "~ωσῃ|~ωσει|~ῐδωσῃ|~ῐδωσει", "~ωσεσθον|~ῐδωσεσθον", "~ωσεσθε|~ῐδωσεσθε";
    future_ind_mid_3: "~ωσεται|~ῐδωσεται", "~ωσεσθον|~ῐδωσεσθον", "~ωσονται|~ῐδωσονται";
    future_ind_pass_1: "~οθησομαι", _, "~οθησομεθᾰ";
    future_ind_pass_2: "~οθησῃ|~οθησει", "~οθησεσθον", "~οθησεσθε";
    future_ind_pass_3: "~οθησεται", _, "~οθησονται";
    future_opt_act_1: "~ωσοιμῐ|~ῐδωσοιμῐ", _, "~ωσοιμεν|~ῐδωσοιμεν";
    future_opt_act_2: "~ωσοις|~ῐδωσοις", "~ωσοιτον|~ῐδωσοιτον", "~ωσοιτε|~ῐδωσοιτε";
    future_opt_act_3: "~ωσοι|~ῐδωσοι", "~ωσοιτην|~ῐδωσοιτην", "~ωσοιεν|~ῐδωσοιεν";
    future_opt_mid_1: "~ωσοιμην|~ῐδωσοιμην", _, "~ωσοιμεθᾰ|~ῐδωσοιμεθᾰ";
    future_opt_mid_2: "~ωσοιο|~ῐδωσοιο", "~ωσοισθον|~ῐδωσοισθον", "~ωσοισθε|~ῐδωσοισθε";
    future_opt_mid_3: "~ωσοιτο|~ῐδωσοιτο", "~ωσοισθην|~ῐδωσοισθην", "~ωσοιντο|~ῐδωσοιντο";
    future_opt_pass_1: "~οθησοιμην", _, "~οθησοιμεθᾰ";
    future_opt_pass_2: "~οθησοιο", "~οθησοισθον", "~οθησοισθε";
    future_opt_pass_3: "~οθησοιτο", "~οθησοισθην", "~οθησοιντο";
    aorist_ind_act_1: "ἐ~ωκᾰ", _, "ἐ~ομεν|ἐ~ωκᾰμεν";
    aorist_ind_act_2: "ἐ~ωκᾰς", "ἐ~οτον|ἐ~ωκᾰτον", "ἐ~οτε|ἐ~ωκᾰτε";
    aorist_ind_act_3: "ἐ~ωκε|ἐ~ωκεν", "ἐ~οτην|ἐ~ωκᾰτην", "ἐ~οσᾰν|ἐ~ωκᾰν";
    aorist_ind_mid_1: "ἐ~ομην|ἐ~ωκᾰμην", _, "ἐ~ομεθᾰ|ἐ~ωκᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ου|ἐ~ωκω", "ἐ~οσθον|ἐ~ωκᾰσθον", "ἐ~οσθε|ἐ~ωκᾰσθε";
    aorist_ind_mid_3: "ἐ~οτο|ἐ~ωκᾰτο", "ἐ~οσθην|ἐ~ωκᾰσθην", "ἐ~οντο|ἐ~ωκᾰντο";
    aorist_ind_pass_1: "ἐ~οθην", _, "ἐ~οθημεν";
    aorist_ind_pass_2: "ἐ~οθης", "ἐ~οθητον", "ἐ~οθητε";
    aorist_ind_pass_3: "ἐ~οθη", "ἐ~οθητην", "ἐ~οθησᾰν";
    aorist_subj_act_1: "~ω", _, "~ωμεν";
    aorist_subj_act_2: "~ῳς", "~ωτον", "~ωτε";
    aorist_subj_act_3: "~ῳ|~ωῃ|~οῑ", _, "~ωσῐ|~ωσῐν";
    aorist_subj_mid_2: "~ῳ", "~ωσθον", "~ωσθε";
    aorist_subj_mid_3: "~ωται", _, _;
    aorist_subj_pass_1: "~οθω", _, "~οθωμεν";
    aorist_subj_pass_2: "~οθῃς", "~οθητον", "~οθητε";
    aorist_subj_pass_3: "~οθῃ", _, "~οθωσῐ|~οθωσῐν";
    aorist_opt_act_1: "~οιην", _, "~οῑμεν|~οιημεν";
    aorist_opt_act_2: "~οιης|~ῳης", "~οῑτον|~οιητον", "~οῑτε|~οιητε";
    aorist_opt_act_3: "~οιη|~ῳη", "~οιτην|~οιητην", "~οῑεν|~οιησᾰν";
    aorist_opt_mid_2: "~οῑο", "~οῑσθον", "~οῑσθε";
    aorist_opt_mid_3: "~οῑτο", _, "~οῑντο";
    aorist_opt_pass_1: "~οθειην", _, "~οθεῑμεν|~οθειημεν";
    aorist_opt_pass_2: "~οθειης", "~οθεῑτον|~οθειητον", "~οθεῑτε|~οθειητε";
    aorist_opt_pass_3: "~οθειη", "~οθειτην|~οθειητην", "~οθεῑεν|~οθειησᾰν";
    aorist_imp_act_2: "~ος", "~οτον", "~οτε";
    aorist_imp_act_3: "~οτω", "~οτων", "~οντων";
    aorist_imp_mid_2: _, "~οσθον", "~οσθε";
    aorist_imp_mid_3: "~οσθω", "~οσθων", _;
    aorist_imp_pass_2: "~οθηθῐ", _, _;
    aorist_imp_pass_3: "~οθητω", "~οθητων", "~οθεντων";
    perfect_ind_act_1: "~εδωκᾰ", _, "~εδωκᾰμεν";
    perfect_ind_act_2: "~εδωκᾰς", "~εδωκᾰτον", "~εδωκᾰτε";
    perfect_ind_act_3: "~εδωκε|~εδωκεν", _, "~εδωκᾱσῐ|~εδωκᾱσῐν";
    perfect_ind_mid_1: "~εδομαι", _, "~εδομεθᾰ";
    perfect_ind_mid_2: "~εδοσαι", "~εδοσθον", "~εδοσθε";
    perfect_ind_mid_3: "~εδοται", _, "~εδονται";
    perfect_ind_pass_1: _, _, "~εδομεθᾰ";
    perfect_ind_pass_2: _, _, "~εδοσθε";
    perfect_subj_act_1: "~εδωκω", _, "~εδωκωμεν";
    perfect_subj_act_2: "~εδωκῃς", "~εδωκητον", "~εδωκητε";
    perfect_subj_act_3: "~εδωκῃ", _, "~εδωκωσῐ|~εδωκωσῐν";
    perfect_opt_act_1: "~εδωκοιμῐ|~εδωκοιην", _, "~εδωκοιμεν";
    perfect_opt_act_2: "~εδωκοις|~εδωκοιης", "~εδωκοιτον", "~εδωκοιτε";
    perfect_opt_act_3: "~εδωκοι|~εδωκοιη", "~εδωκοιτην", "~εδωκοιεν";
    perfect_imp_act_2: "~εδωκε", "~εδωκετον", "~εδωκετε";
    perfect_imp_act_3: "~εδωκετω", "~εδωκετων", "~εδωκοντων";
    perfect_imp_mid_2: "~εδοσο", _, _;
    perfect_imp_mid_3: "~εδοσθω", "~εδοσθων", _;
    perfect_imp_pass_2: "~εδοσο", _, _;
    perfect_imp_pass_3: "~εδοσθω", "~εδοσθων", _;
    pluperfect_act_1: "ἐ~εδωκειν|ἐ~εδωκη", _, "ἐ~εδωκεμεν";
    pluperfect_act_2: "ἐ~εδωκεις|ἐ~εδωκης", "ἐ~εδωκετον", "ἐ~εδωκετε";
    pluperfect_act_3: "ἐ~εδωκει|ἐ~εδωκειν", "ἐ~εδωκετην", "ἐ~εδωκεσᾰν";
    pluperfect_mid_1: "ἐ~εδομην", _, "ἐ~εδομεθᾰ";
    pluperfect_mid_2: "ἐ~εδοσο", "ἐ~εδοσθον", "ἐ~εδοσθε";
    pluperfect_mid_3: "ἐ~εδοτο", "ἐ~εδοσθην", _;
    pluperfect_pass_1: _, _, "ἐ~εδομεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~εδοσθε";
    pluperfect_pass_3: _, "ἐ~εδοσθην", "ἐ~εδοντο";
    future_perfect_ind_mid_1: "~εδωσομαι", _, "~εδωσομεθᾰ";
    future_perfect_ind_mid_2: "~εδωσῃ|~εδωσει", "~εδωσεσθον", "~εδωσεσθε";
    future_perfect_ind_mid_3: "~εδωσεται", _, "~εδωσονται";
    future_perfect_ind_pass_2: "~εδωσῃ|~εδωσει", _, _;
    future_perfect_opt_mid_1: "~εδωσοιμην", _, "~εδωσοιμεθᾰ";
    future_perfect_opt_mid_2: "~εδωσοιο", "~εδωσοισθον", "~εδωσοισθε";
    future_perfect_opt_mid_3: "~εδωσοιτο", "~εδωσοισθην", "~εδωσοιντο";
    infinitive_pres: "~ῐδοναι", "~ῐδοσθαι", _;
    infinitive_fut: "~ωσειν|~ῐδωσειν", "~ωσεσθαι|~ῐδωσεσθαι", "~οθησεσθαι";
    infinitive_aor: "~οῡναι", "~οσθαι", "~οθηναι";
    infinitive_perf: "~εδωκεναι", "~εδοσθαι", _;
    infinitive_futp: _, "~εδωσεσθαι", _;
    participle_pres: "~ῐδους", "~ῐδομενος", _;
    participle_fut: "~ωσων|~ῐδωσων", "~ωσομενος|~ῐδωσομενος", "~οθησομενος";
    participle_aor: "~ους", _, "~οθεις";
    participle_perf: "~εδωκως", "~εδομενος", _;
    participle_futp: _, "~εδωσομενος", _;
    ),
    // Εἰπον: 1 words (εἰπον). Like Ἐγκειμαι, except in 115 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("εἰπον", 1, "~ἰ~ον", like Ἐγκειμαι;
    ind_act_1: "~ἰ~ον", _, _;
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, derived, derived;
    ind_mid_3: derived, _, derived;
    subj_mid_1: derived, _, derived;
    subj_mid_2: derived, derived, derived;
    subj_mid_3: derived, _, derived;
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, derived;
    opt_mid_3: derived, derived, derived;
    imp_mid_2: derived, _, derived;
    imp_mid_3: derived, derived, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, derived;
    imperfect_mid_3: derived, derived, derived;
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, derived;
    future_ind_mid_3: derived, _, derived;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, derived, derived;
    future_opt_mid_3: derived, derived, derived;
    aorist_ind_act_1: "~ἰ~ον|~ἰ~ᾰ|ἐ~ι~ᾰ", _, "~ἰ~ομεν|~ἰ~ᾰμεν|ἐ~ι~ᾰμεν";
    aorist_ind_act_2: "~ἰ~ες|~ἰ~ᾰς|ἐ~ι~ᾰς", "~ἰ~ετον|~ἰ~ᾰτον|ἐ~ι~ᾰτον", "~ἰ~ετε|~ἰ~ᾰτε|ἐ~ι~ᾰτε";
    aorist_ind_act_3: "~ἰ~ε|~ἰ~εν|ἐ~ι~ε|ἐ~ι~εν", "~ἰ~ετην|~ἰ~ᾰτην|ἐ~ι~ᾰτην", "~ἰ~ον|~ἰ~ᾰν|ἐ~ι~ᾰν";
    aorist_ind_mid_1: "~ἰ~ομην|~ἰ~ᾰμην|ἐ~ι~ᾰμην", _, "~ἰ~ομεθᾰ|~ἰ~ᾰμεθᾰ|ἐ~ι~ᾰμεθᾰ";
    aorist_ind_mid_2: "~ἰ~ου|~ἰ~ω|ἐ~ι~ω", "~ἰ~εσθον|~ἰ~ᾰσθον|ἐ~ι~ᾰσθον", "~ἰ~εσθε|~ἰ~ᾰσθε|ἐ~ι~ᾰσθε";
    aorist_ind_mid_3: "~ἰ~ετο|~ἰ~ᾰτο|ἐ~ι~ᾰτο", "~ἰ~εσθην|~ἰ~ᾰσθην|ἐ~ι~ᾰσθην", "~ἰ~οντο|~ἰ~ᾰντο|ἐ~ι~ᾰντο";
    aorist_subj_act_1: "~ἰ~ω", _, "~ἰ~ωμεν";
    aorist_subj_act_2: "~ἰ~ῃς", "~ἰ~ητον", "~ἰ~ητε";
    aorist_subj_act_3: "~ἰ~ῃ", _, "~ἰ~ωσῐ|~ἰ~ωσῐν";
    aorist_subj_mid_1: "~ἰ~ωμαι", _, "~ἰ~ωμεθᾰ";
    aorist_subj_mid_2: "~ἰ~ῃ", "~ἰ~ησθον", "~ἰ~ησθε";
    aorist_subj_mid_3: "~ἰ~ηται", _, "~ἰ~ωνται";
    aorist_opt_act_1: "~ἰ~οιμῐ|~ἰ~αιμῐ", _, "~ἰ~οιμεν|~ἰ~αιμεν";
    aorist_opt_act_2: "~ἰ~οις|~ἰ~ειᾰς|~ἰ~αις", "~ἰ~οιτον|~ἰ~αιτον", "~ἰ~οιτε|~ἰ~αιτε";
    aorist_opt_act_3: "~ἰ~οι|~ἰ~ειε|~ἰ~ειεν|~ἰ~αι", "~ἰ~οιτην|~ἰ~αιτην", "~ἰ~οιεν|~ἰ~ειᾰν|~ἰ~αιεν";
    aorist_opt_mid_1: "~ἰ~οιμην|~ἰ~αιμην", _, "~ἰ~οιμεθᾰ|~ἰ~αιμεθᾰ";
    aorist_opt_mid_2: "~ἰ~οιο|~ἰ~αιο", "~ἰ~οισθον|~ἰ~αισθον", "~ἰ~οισθε|~ἰ~αισθε";
    aorist_opt_mid_3: "~ἰ~οιτο|~ἰ~αιτο", "~ἰ~οισθην|~ἰ~αισθην", "~ἰ~οιντο|~ἰ~αιντο";
    aorist_imp_act_2: "~ἰ~ε|~ἰ~ον", "~ἰ~ετον|~ἰ~ᾰτον", "~ἰ~ετε|~ἰ~ᾰτε";
    aorist_imp_act_3: "~ἰ~ετω|~ἰ~ᾰτω", "~ἰ~ετων|~ἰ~ᾰτων", "~ἰ~οντων|~ἰ~ᾰντων";
    aorist_imp_mid_2: "~ἰ~οῡ|~ἰ~αι", "~ἰ~εσθον|~ἰ~ᾰσθον", "~ἰ~εσθε|~ἰ~ᾰσθε";
    aorist_imp_mid_3: "~ἰ~εσθω|~ἰ~ᾰσθω", "~ἰ~εσθων|~ἰ~ᾰσθων", "~ἰ~εσθων|~ἰ~ᾰσθων";
    infinitive_fut: _, derived, _;
    infinitive_aor: "~ἰ~εῑν|~ἰ~αι", "~ἰ~εσθαι|~ἰ~ᾰσθαι", _;
    participle_fut: _, derived, _;
    participle_aor: "~ἰ~ων|~ἰ~ᾱς", "~ἰ~ομενος|~ἰ~ᾰμενος", _;
    ),
    // Δεικνῡμῐ: 1 words (δεικνῡμῐ).
    conjugation!("δεικνῡμῐ", 1, "~ικνῡμῐ";
    ind_act_1: "~ικνῡμῐ", _, "~ικνῠμεν";
    ind_act_2: "~ικνῡς", _, _;
    ind_act_3: "~ικνῡσῐ|~ικνῡσῐν", _, "~ικνῠᾱσῐ|~ικνῠᾱσῐν";
    ind_mid_1: "~ικνῠμαι", _, "~ικνῠμεθᾰ";
    ind_mid_2: "~ικνῠσαι", "~ικνῠσθον", "~ικνῠσθε";
    ind_mid_3: "~ικνῠται", _, "~ικνῠνται";
    ind_pass_1: _, _, "~ικνῠμεθᾰ";
    ind_pass_2: _, _, "~ικνῠσθε";
    ind_pass_3: "~ικνῠται", _, "~ικνῠνται";
    subj_act_1: "~ικνῠω", _, "~ικνῠωμεν";
    subj_act_2: "~ικνῠῃς", "~ικνῠητον", "~ικνῠητε";
    subj_act_3: "~ικνῠῃ", _, "~ικνῠωσῐ|~ικνῠωσῐν";
    subj_mid_1: "~ικνῠωμαι", _, "~ικνῠωμεθᾰ";
    subj_mid_2: "~ικνῠῃ", "~ικνῠησθον", "~ικνῠησθε";
    subj_mid_3: "~ικνῠηται", _, "~ικνῠωνται";
    subj_pass_1: _, _, "~ικνῠωμεθᾰ";
    subj_pass_2: _, _, "~ικνῠησθε";
    subj_pass_3: "~ικνῠηται", _, "~ικνῠωνται";
    opt_act_1: "~ικνῠοιμῐ", _, "~ικνῠοιμεν";
    opt_act_2: "~ικνῠοις", "~ικνῠοιτον", "~ικνῠοιτε";
    opt_act_3: "~ικνῠοι", "~ικνῠοιτην", "~ικνῠοιεν";
    opt_mid_1: "~ικνῠοιμην", _, "~ικνῠοιμεθᾰ";
    opt_mid_2: "~ικνῠοιο", "~ικνῠοισθον", "~ικνῠοισθε";
    opt_mid_3: "~ικνῠοιτο", "~ικνῠοισθην", "~ικνῠοιντο";
    opt_pass_1: _, _, "~ικνῠοιμεθᾰ";
    opt_pass_2: _, "~ικνῠοισθον", "~ικνῠοισθε";
    opt_pass_3: "~ικνῠοιτο", "~ικνῠοισθην", "~ικνῠοιντο";
    imp_act_2: "~ικνῡ", "~ικνῠτον", "~ικνῠτε";
    imp_act_3: "~ικνῠτω", "~ικνῠτων", "~ικνῠντων";
    imp_mid_2: "~ικνῠσο", _, "~ικνῠσθε";
    imp_mid_3: "~ικνῠσθω", "~ικνῠσθων", "~ικνῠσθων";
    imp_pass_2: _, _, "~ικνῠσθε";
    imp_pass_3: _, "~ικνῠσθων", _;
    imperfect_act_1: "ἐ~ικνῡν", _, "ἐ~ικνῠμεν";
    imperfect_act_2: "ἐ~ικνῡς", "ἐ~ικνῠτον", "ἐ~ικνῠτε";
    imperfect_act_3: "ἐ~ικνῡ", "ἐ~ικνῠτην", "ἐ~ικνῠσᾰν";
    imperfect_mid_1: "ἐ~ικνῠμην", _, "ἐ~ικνῠμεθᾰ";
    imperfect_mid_2: "ἐ~ικνῠσο", "ἐ~ικνῠσθον", "ἐ~ικνῠσθε";
    imperfect_mid_3: "ἐ~ικνῠτο", "ἐ~ικνῠσθην", "ἐ~ικνῠντο";
    imperfect_pass_1: _, _, "ἐ~ικνῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ικνῠσθε";
    imperfect_pass_3: "ἐ~ικνῠτο", _, _;
    future_ind_act_1: "~ιξω", _, "~ιξομεν";
    future_ind_act_2: "~ιξεις", "~ιξετον", "~ιξετε";
    future_ind_act_3: "~ιξει", _, "~ιξουσῐ|~ιξουσῐν";
    future_ind_mid_1: "~ιξομαι", _, "~ιξομεθᾰ";
    future_ind_mid_2: "~ιξῃ|~ιξει", "~ιξεσθον", "~ιξεσθε";
    future_ind_mid_3: "~ιξεται", _, "~ιξονται";
    future_ind_pass_1: "~ιχθησομαι", _, "~ιχθησομεθᾰ";
    future_ind_pass_2: "~ιχθησῃ|~ιχθησει", "~ιχθησεσθον", "~ιχθησεσθε";
    future_ind_pass_3: "~ιχθησεται", _, "~ιχθησονται";
    future_opt_act_1: "~ιξοιμῐ", _, "~ιξοιμεν";
    future_opt_act_2: "~ιξοις", "~ιξοιτον", "~ιξοιτε";
    future_opt_act_3: "~ιξοι", "~ιξοιτην", "~ιξοιεν";
    future_opt_mid_1: "~ιξοιμην", _, "~ιξοιμεθᾰ";
    future_opt_mid_2: "~ιξοιο", "~ιξοισθον", "~ιξοισθε";
    future_opt_mid_3: "~ιξοιτο", "~ιξοισθην", "~ιξοιντο";
    future_opt_pass_1: "~ιχθησοιμην", _, "~ιχθησοιμεθᾰ";
    future_opt_pass_2: "~ιχθησοιο", "~ιχθησοισθον", "~ιχθησοισθε";
    future_opt_pass_3: "~ιχθησοιτο", "~ιχθησοισθην", "~ιχθησοιντο";
    aorist_ind_act_1: "ἐ~ιξᾰ", _, "ἐ~ιξᾰμεν";
    aorist_ind_act_2: "ἐ~ιξᾰς", "ἐ~ιξᾰτον", "ἐ~ιξᾰτε";
    aorist_ind_act_3: "ἐ~ιξε|ἐ~ιξεν", "ἐ~ιξᾰτην", "ἐ~ιξᾰν";
    aorist_ind_mid_1: "ἐ~ιξᾰμην", _, "ἐ~ιξᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ιξω", "ἐ~ιξᾰσθον", "ἐ~ιξᾰσθε";
    aorist_ind_mid_3: "ἐ~ιξᾰτο", "ἐ~ιξᾰσθην", "ἐ~ιξᾰντο";
    aorist_ind_pass_1: "ἐ~ιχθην", _, "ἐ~ιχθημεν";
    aorist_ind_pass_2: "ἐ~ιχθης", "ἐ~ιχθητον", "ἐ~ιχθητε";
    aorist_ind_pass_3: "ἐ~ιχθη", "ἐ~ιχθητην", "ἐ~ιχθησᾰν";
    aorist_subj_act_1: "~ιξω", _, "~ιξωμεν";
    aorist_subj_act_2: "~ιξῃς", "~ιξητον", "~ιξητε";
    aorist_subj_act_3: "~ιξῃ", _, "~ιξωσῐ|~ιξωσῐν";
    aorist_subj_mid_1: "~ιξωμαι", _, "~ιξωμεθᾰ";
    aorist_subj_mid_2: "~ιξῃ", "~ιξησθον", "~ιξησθε";
    aorist_subj_mid_3: "~ιξηται", _, "~ιξωνται";
    aorist_subj_pass_1: "~ιχθω", _, "~ιχθωμεν";
    aorist_subj_pass_2: "~ιχθῃς", "~ιχθητον", "~ιχθητε";
    aorist_subj_pass_3: "~ιχθῃ", _, "~ιχθωσῐ|~ιχθωσῐν";
    aorist_opt_act_1: "~ιξαιμῐ", _, "~ιξαιμεν";
    aorist_opt_act_2: "~ιξειᾰς|~ιξαις", "~ιξαιτον", "~ιξαιτε";
    aorist_opt_act_3: "~ιξειε|~ιξειεν|~ιξαι", "~ιξαιτην", "~ιξειᾰν|~ιξαιεν";
    aorist_opt_mid_1: "~ιξαιμην", _, "~ιξαιμεθᾰ";
    aorist_opt_mid_2: "~ιξαιο", "~ιξαισθον", "~ιξαισθε";
    aorist_opt_mid_3: "~ιξαιτο", "~ιξαισθην", "~ιξαιντο";
    aorist_opt_pass_1: "~ιχθειην", _, "~ιχθεῑμεν|~ιχθειημεν";
    aorist_opt_pass_2: "~ιχθειης", "~ιχθεῑτον|~ιχθειητον", "~ιχθεῑτε|~ιχθειητε";
    aorist_opt_pass_3: "~ιχθειη", "~ιχθειτην|~ιχθειητην", "~ιχθεῑεν|~ιχθειησᾰν";
    aorist_imp_act_2: "~ῑξον", "~ιξᾰτον", "~ιξᾰτε";
    aorist_imp_act_3: "~ιξᾰτω", "~ιξᾰτων", "~ιξᾰντων";
    aorist_imp_mid_2: "~ῑξαι", "~ιξᾰσθον", "~ιξᾰσθε";
    aorist_imp_mid_3: "~ιξᾰσθω", "~ιξᾰσθων", _;
    aorist_imp_pass_2: "~ιχθητῐ", _, _;
    aorist_imp_pass_3: "~ιχθητω", "~ιχθητων", "~ιχθεντων";
    perfect_ind_act_1: "~δειχᾰ", _, "~δειχᾰμεν";
    perfect_ind_act_2: "~δειχᾰς", "~δειχᾰτον", "~δειχᾰτε";
    perfect_ind_act_3: "~δειχε|~δειχεν", _, "~δειχᾱσῐ|~δειχᾱσῐν";
    perfect_ind_mid_1: "~δειγμαι", _, "~δειγμεθᾰ";
    perfect_ind_mid_2: "~δειξαι", "~δειχθον", "~δειχθε";
    perfect_ind_mid_3: "~δεικται", _, "~δεικᾰται";
    perfect_ind_pass_1: _, _, "~δειγμεθᾰ";
    perfect_ind_pass_2: _, _, "~δειχθε";
    perfect_subj_act_1: "~δειχω", _, "~δειχωμεν";
    perfect_subj_act_2: "~δειχῃς", "~δειχητον", "~δειχητε";
    perfect_subj_act_3: "~δειχῃ", _, "~δειχωσῐ|~δειχωσῐν";
    perfect_opt_act_1: "~δειχοιμῐ|~δειχοιην", _, "~δειχοιμεν";
    perfect_opt_act_2: "~δειχοις|~δειχοιης", "~δειχοιτον", "~δειχοιτε";
    perfect_opt_act_3: "~δειχοι|~δειχοιη", "~δειχοιτην", "~δειχοιεν";
    perfect_imp_act_2: "~δειχε", "~δειχετον", "~δειχετε";
    perfect_imp_act_3: "~δειχετω", "~δειχετων", "~δειχοντων";
    perfect_imp_mid_2: "~δειξο", _, _;
    perfect_imp_mid_3: "~δειχθω", "~δειχθων", _;
    perfect_imp_pass_2: "~δειξο", _, _;
    perfect_imp_pass_3: "~δειχθω", "~δειχθων", _;
    pluperfect_act_1: "ἐ~δειχειν|ἐ~δειχη", _, "ἐ~δειχεμεν";
    pluperfect_act_2: "ἐ~δειχεις|ἐ~δειχης", "ἐ~δειχετον", "ἐ~δειχετε";
    pluperfect_act_3: "ἐ~δειχει|ἐ~δειχειν", "ἐ~δειχετην", "ἐ~δειχεσᾰν";
    pluperfect_mid_1: "ἐ~δειγμην", _, "ἐ~δειγμεθᾰ";
    pluperfect_mid_2: "ἐ~δειξο", "ἐ~δειχθον", "ἐ~δειχθε";
    pluperfect_mid_3: "ἐ~δεικτο", "ἐ~δειχθην", _;
    pluperfect_pass_1: _, _, "ἐ~δειγμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~δειχθε";
    pluperfect_pass_3: _, "ἐ~δειχθην", "ἐ~δεικᾰτο";
    future_perfect_ind_mid_1: "~δειξομαι", _, "~δειξομεθᾰ";
    future_perfect_ind_mid_2: "~δειξῃ|~δειξει", "~δειξεσθον", "~δειξεσθε";
    future_perfect_ind_mid_3: "~δειξεται", _, "~δειξονται";
    future_perfect_ind_pass_2: "~δειξῃ|~δειξει", _, _;
    future_perfect_opt_mid_1: "~δειξοιμην", _, "~δειξοιμεθᾰ";
    future_perfect_opt_mid_2: "~δειξοιο", "~δειξοισθον", "~δειξοισθε";
    future_perfect_opt_mid_3: "~δειξοιτο", "~δειξοισθην", "~δειξοιντο";
    infinitive_pres: "~ικνῠναι", "~ικνῠσθαι", _;
    infinitive_fut: "~ιξειν", "~ιξεσθαι", "~ιχθησεσθαι";
    infinitive_aor: "~ῑξαι", "~ιξᾰσθαι", "~ιχθηναι";
    infinitive_perf: "~δειχεναι", "~δεῑχθαι", _;
    infinitive_futp: _, "~δειξεσθαι", _;
    participle_pres: "~ικνῡς", "~ικνῠμενος", _;
    participle_fut: "~ιξων", "~ιξομενος", "~ιχθησομενος";
    participle_aor: "~ιξᾱς", "~ιξᾰμενος", "~ιχθεις";
    participle_perf: "~δειχως", "~δειγμενος", _;
    participle_futp: _, "~δειξομενος", _;
    ),
    // Ἐφᾰγον: 1 words (ἐφᾰγον). Like Πᾰρειμι, except in 67 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, subj_act_1 sg, ….
    conjugation!("ἐφᾰγον", 1, "ἐ~ον", like Πᾰρειμι;
    ind_act_1: "ἐ~ον", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, _, derived;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imp_act_3: derived, derived, derived;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    aorist_ind_act_1: "ἐ~ον", _, "ἐ~ομεν";
    aorist_ind_act_2: "ἐ~ες", "ἐ~ετον", "ἐ~ετε";
    aorist_ind_act_3: "ἐ~ε|ἐ~εν", "ἐ~ετην", "ἐ~ον";
    aorist_subj_act_1: "~ω", _, "~ωμεν";
    aorist_subj_act_2: "~ῃς", "~ητον", "~ητε";
    aorist_subj_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    aorist_opt_act_1: "~οιμῐ", _, "~οιμεν";
    aorist_opt_act_2: "~οις", "~οιτον", "~οιτε";
    aorist_opt_act_3: "~οι", "~οιτην", "~οιεν";
    aorist_imp_act_2: "~ε", "~ετον", "~ετε";
    aorist_imp_act_3: "~ετω", "~ετων", "~οντων";
    infinitive_pres: derived, _, _;
    infinitive_aor: "~εῑν", _, _;
    participle_pres: derived, _, _;
    participle_aor: "~ων", _, _;
    ),
    // Ἑννῡμῐ: 1 words (ἑννῡμῐ). Like Εἰμῐ, except in 232 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἑννῡμῐ", 1, "ἑννῡμῐ", like Εἰμῐ;
    ind_act_1: "ἑννῡμῐ|ἑννῠω", _, "ἑννῠμεν|ἑννῠομεν";
    ind_act_2: "ἑννῡς|ἑννῠεις", "ἑννῠτον|ἑννῠετον", "ἑννῠτε|ἑννῠετε";
    ind_act_3: "ἑννῡσῐ|ἑννῡσῐν|ἑννῠει", "ἑννῠτον|ἑννῠετον", "ἑννῠᾱσῐ|ἑννῠᾱσῐν|ἑννῠουσῐ|ἑννῠουσῐν";
    ind_mid_1: "ἑννῠμαι|ἑννῠομαι", _, "ἑννῠμεθᾰ|ἑννῠομεθᾰ";
    ind_mid_2: "ἑννῠσαι|ἑννῠῃ|ἑννῠει", "ἑννῠσθον|ἑννῠεσθον", "ἑννῠσθε|ἑννῠεσθε";
    ind_mid_3: "ἑννῠται|ἑννῠεται", "ἑννῠσθον|ἑννῠεσθον", "ἑννῠνται|ἑννῠονται";
    ind_pass_1: "ἑννῠμαι|ἑννῠομαι", _, "ἑννῠμεθᾰ|ἑννῠομεθᾰ";
    ind_pass_2: "ἑννῠσαι|ἑννῠῃ|ἑννῠει", "ἑννῠσθον|ἑννῠεσθον", "ἑννῠσθε|ἑννῠεσθε";
    ind_pass_3: "ἑννῠται|ἑννῠεται", "ἑννῠσθον|ἑννῠεσθον", "ἑννῠνται|ἑννῠονται";
    subj_act_1: "ἑννῠω", _, "ἑννῠωμεν";
    subj_act_2: "ἑννῠῃς", "ἑννῠητον", "ἑννῠητε";
    subj_act_3: "ἑννῠῃ", _, "ἑννῠωσῐ|ἑννῠωσῐν";
    subj_mid_1: "ἑννῠωμαι", _, "ἑννῠωμεθᾰ";
    subj_mid_2: "ἑννῠῃ", "ἑννῠησθον", "ἑννῠησθε";
    subj_mid_3: "ἑννῠηται", _, "ἑννῠωνται";
    subj_pass_1: _, _, "ἑννῠωμεθᾰ";
    subj_pass_2: _, _, "ἑννῠησθε";
    subj_pass_3: "ἑννῠηται", _, "ἑννῠωνται";
    opt_act_1: "ἑννῠοιμῐ", _, "ἑννῠοιμεν";
    opt_act_2: "ἑννῠοις", "ἑννῠοιτον", "ἑννῠοιτε";
    opt_act_3: "ἑννῠοι", "ἑννῠοιτην", "ἑννῠοιεν";
    opt_mid_1: "ἑννῠοιμην", _, "ἑννῠοιμεθᾰ";
    opt_mid_2: "ἑννῠοιο", "ἑννῠοισθον", "ἑννῠοισθε";
    opt_mid_3: "ἑννῠοιτο", "ἑννῠοισθην", "ἑννῠοιντο";
    opt_pass_1: _, _, "ἑννῠοιμεθᾰ";
    opt_pass_2: _, "ἑννῠοισθον", "ἑννῠοισθε";
    opt_pass_3: "ἑννῠοιτο", "ἑννῠοισθην", "ἑννῠοιντο";
    imp_act_2: "ἑννῡ|ἑννῠε", "ἑννῠτον|ἑννῠετον", "ἑννῠτε|ἑννῠετε";
    imp_act_3: "ἑννῠτω|ἑννῠετω", "ἑννῠτων|ἑννῠετων", "ἑννῠντων|ἑννῠοντων";
    imp_mid_2: "ἑννῠσο|ἑννῠου", "ἑννῠσθον|ἑννῠεσθον", "ἑννῠσθε|ἑννῠεσθε";
    imp_mid_3: "ἑννῠσθω|ἑννῠεσθω", "ἑννῠσθων|ἑννῠεσθων", "ἑννῠσθων|ἑννῠεσθων";
    imp_pass_2: "ἑννῠσο|ἑννῠου", "ἑννῠσθον|ἑννῠεσθον", "ἑννῠσθε|ἑννῠεσθε";
    imp_pass_3: "ἑννῠσθω|ἑννῠεσθω", "ἑννῠσθων|ἑννῠεσθων", "ἑννῠσθων|ἑννῠεσθων";
    imperfect_act_1: "ἑννῡν", _, "ἑννῠμεν";
    imperfect_act_2: "ἑννῡς", "ἑννῠτον", "ἑννῠτε";
    imperfect_act_3: "ἑννῡ", "ἑννῠτην", "ἑννῠσᾰν";
    imperfect_mid_1: "ἑννῠμην", _, "ἑννῠμεθᾰ";
    imperfect_mid_2: "ἑννῠσο", "ἑννῠσθον", "ἑννῠσθε";
    imperfect_mid_3: "ἑννῠτο", "ἑννῠσθην", "ἑννῠντο";
    imperfect_pass_1: _, _, "ἑννῠμεθᾰ";
    imperfect_pass_2: _, _, "ἑννῠσθε";
    imperfect_pass_3: "ἑννῠτο", _, _;
    future_ind_act_1: "ἑσω|ἑσσω", _, "ἑσομεν|ἑσσομεν";
    future_ind_act_2: "ἑσεις|ἑσσεις", "ἑσετον|ἑσσετον", "ἑσετε|ἑσσετε";
    future_ind_act_3: "ἑσει|ἑσσει", "ἑσετον|ἑσσετον", "ἑσουσῐ|ἑσουσῐν|ἑσσουσῐ|ἑσσουσῐν";
    future_ind_mid_1: "ἑσομαι|ἑσσομαι", _, "ἑσομεθᾰ|ἑσσομεθᾰ";
    future_ind_mid_2: "ἑσῃ|ἑσει|ἑσσῃ|ἑσσει", "ἑσεσθον|ἑσσεσθον", "ἑσεσθε|ἑσσεσθε";
    future_ind_mid_3: "ἑσεται|ἑσσεται", "ἑσεσθον|ἑσσεσθον", "ἑσονται|ἑσσονται";
    future_opt_act_1: "ἑσοιμῐ|ἑσσοιμῐ", _, "ἑσοιμεν|ἑσσοιμεν";
    future_opt_act_2: "ἑσοις|ἑσσοις", "ἑσοιτον|ἑσσοιτον", "ἑσοιτε|ἑσσοιτε";
    future_opt_act_3: "ἑσοι|ἑσσοι", "ἑσοιτην|ἑσσοιτην", "ἑσοιεν|ἑσσοιεν";
    future_opt_mid_1: "ἑσοιμην|ἑσσοιμην", _, "ἑσοιμεθᾰ|ἑσσοιμεθᾰ";
    future_opt_mid_2: "ἑσοιο|ἑσσοιο", "ἑσοισθον|ἑσσοισθον", "ἑσοισθε|ἑσσοισθε";
    future_opt_mid_3: "ἑσοιτο|ἑσσοιτο", "ἑσοισθην|ἑσσοισθην", "ἑσοιντο|ἑσσοιντο";
    aorist_ind_act_1: "ἑεσσᾰ|ἑσσᾰ", _, "ἑεσσᾰμεν|ἑσσᾰμεν";
    aorist_ind_act_2: "ἑεσσᾰς|ἑσσᾰς", "ἑεσσᾰτον|ἑσσᾰτον", "ἑεσσᾰτε|ἑσσᾰτε";
    aorist_ind_act_3: "ἑεσσε|ἑεσσεν|ἑσσε|ἑσσεν", "ἑεσσᾰτην|ἑσσᾰτην", "ἑεσσᾰν|ἑσσᾰν";
    aorist_ind_mid_1: "ἑεσσᾰμην|ἑσσᾰμην", _, "ἑεσσᾰμεθᾰ|ἑσσᾰμεθᾰ";
    aorist_ind_mid_2: "ἑεσσω|ἑσσω", "ἑεσσᾰσθον|ἑσσᾰσθον", "ἑεσσᾰσθε|ἑσσᾰσθε";
    aorist_ind_mid_3: "ἑεσσᾰτο|ἑσσᾰτο", "ἑεσσᾰσθην|ἑσσᾰσθην", "ἑεσσᾰντο|ἑσσᾰντο";
    aorist_subj_act_1: "ἑσσω", _, "ἑσσωμεν";
    aorist_subj_act_2: "ἑσσῃς", "ἑσσητον", "ἑσσητε";
    aorist_subj_act_3: "ἑσσῃ", _, "ἑσσωσῐ|ἑσσωσῐν";
    aorist_subj_mid_1: "ἑσσωμαι", _, "ἑσσωμεθᾰ";
    aorist_subj_mid_2: "ἑσσῃ", "ἑσσησθον", "ἑσσησθε";
    aorist_subj_mid_3: "ἑσσηται", _, "ἑσσωνται";
    aorist_opt_act_1: "ἑσσαιμῐ", _, "ἑσσαιμεν";
    aorist_opt_act_2: "ἑσσειᾰς|ἑσσαις", "ἑσσαιτον", "ἑσσαιτε";
    aorist_opt_act_3: "ἑσσειε|ἑσσειεν|ἑσσαι", "ἑσσαιτην", "ἑσσειᾰν|ἑσσαιεν";
    aorist_opt_mid_1: "ἑσσαιμην", _, "ἑσσαιμεθᾰ";
    aorist_opt_mid_2: "ἑσσαιο", "ἑσσαισθον", "ἑσσαισθε";
    aorist_opt_mid_3: "ἑσσαιτο", "ἑσσαισθην", "ἑσσαιντο";
    aorist_imp_act_2: "ἑσσον", "ἑσσᾰτον", "ἑσσᾰτε";
    aorist_imp_act_3: "ἑσσᾰτω", "ἑσσᾰτων", "ἑσσᾰντων";
    aorist_imp_mid_2: "ἑσσαι", "ἑσσᾰσθον", "ἑσσᾰσθε";
    aorist_imp_mid_3: "ἑσσᾰσθω", "ἑσσᾰσθων", _;
    perfect_ind_mid_1: "εἱμαι", _, "εἱμεθᾰ";
    perfect_ind_mid_2: "εἱσαι", "εἱσθον", "εἱσθε";
    perfect_ind_mid_3: "εἱται", _, "εἱνται";
    perfect_ind_pass_1: _, _, "εἱμεθᾰ";
    perfect_ind_pass_2: _, _, "εἱσθε";
    perfect_imp_mid_2: "εἱσο", _, _;
    perfect_imp_mid_3: "εἱσθω", "εἱσθων", _;
    perfect_imp_pass_2: "εἱσο", _, _;
    perfect_imp_pass_3: "εἱσθω", "εἱσθων", _;
    pluperfect_mid_1: "εἱμην", _, "εἱμεθᾰ";
    pluperfect_mid_2: "εἱσο", "εἱσθον", "εἱσθε";
    pluperfect_mid_3: "εἱτο", "εἱσθην", _;
    pluperfect_pass_1: _, _, "εἱμεθᾰ";
    pluperfect_pass_2: _, _, "εἱσθε";
    pluperfect_pass_3: _, "εἱσθην", "εἱντο";
    infinitive_pres: "ἑννῠναι|ἑννῠειν", "ἑννῠσθαι|ἑννῠεσθαι", "ἑννῠσθαι|ἑννῠεσθαι";
    infinitive_fut: "ἑσειν|ἑσσειν", "ἑσεσθαι|ἑσσεσθαι", _;
    infinitive_aor: "ἑσσαι", "ἑσσᾰσθαι", _;
    infinitive_perf: _, "εἱσθαι", _;
    participle_pres: "ἑννῡς|ἑννῠων", "ἑννῠμενος|ἑννῠομενος", "ἑννῠμενος|ἑννῠομενος";
    participle_fut: "ἑσων|ἑσσων", "ἑσομενος|ἑσσομενος", _;
    participle_aor: "ἑσσᾱς", "ἑσσᾰμενος", _;
    participle_perf: _, "εἱμενος", _;
    ),
    // Ὀρνῡμῐ: 1 words (ὀρνῡμῐ). Like Ἑπομαι, except in 254 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ὀρνῡμῐ", 1, "ὀ~νῡμῐ", like Ἑπομαι;
    ind_act_1: "ὀ~νῡμῐ", _, "ὀ~νῠμεν";
    ind_act_2: "ὀ~νῡς", _, _;
    ind_act_3: "ὀ~νῡσῐ|ὀ~νῡσῐν", _, "ὀ~νῠᾱσῐ|ὀ~νῠᾱσῐν";
    ind_mid_1: "ὀ~νῠμαι", _, "ὀ~νῠμεθᾰ";
    ind_mid_2: "ὀ~νῠσαι", "ὀ~νῠσθον", "ὀ~νῠσθε";
    ind_mid_3: "ὀ~νῠται", _, "ὀ~νῠνται";
    ind_pass_1: _, _, "ὀ~νῠμεθᾰ";
    ind_pass_2: _, _, "ὀ~νῠσθε";
    ind_pass_3: "ὀ~νῠται", _, "ὀ~νῠνται";
    subj_act_1: "ὀ~νῠω", _, "ὀ~νῠωμεν";
    subj_act_2: "ὀ~νῠῃς", "ὀ~νῠητον", "ὀ~νῠητε";
    subj_act_3: "ὀ~νῠῃ", _, "ὀ~νῠωσῐ|ὀ~νῠωσῐν";
    subj_mid_1: "ὀ~νῠωμαι", _, "ὀ~νῠωμεθᾰ";
    subj_mid_2: "ὀ~νῠῃ", "ὀ~νῠησθον", "ὀ~νῠησθε";
    subj_mid_3: "ὀ~νῠηται", _, "ὀ~νῠωνται";
    subj_pass_1: _, _, "ὀ~νῠωμεθᾰ";
    subj_pass_2: _, _, "ὀ~νῠησθε";
    subj_pass_3: "ὀ~νῠηται", _, "ὀ~νῠωνται";
    opt_act_1: "ὀ~νῠοιμῐ", _, "ὀ~νῠοιμεν";
    opt_act_2: "ὀ~νῠοις", "ὀ~νῠοιτον", "ὀ~νῠοιτε";
    opt_act_3: "ὀ~νῠοι", "ὀ~νῠοιτην", "ὀ~νῠοιεν";
    opt_mid_1: "ὀ~νῠοιμην", _, "ὀ~νῠοιμεθᾰ";
    opt_mid_2: "ὀ~νῠοιο", "ὀ~νῠοισθον", "ὀ~νῠοισθε";
    opt_mid_3: "ὀ~νῠοιτο", "ὀ~νῠοισθην", "ὀ~νῠοιντο";
    opt_pass_1: _, _, "ὀ~νῠοιμεθᾰ";
    opt_pass_2: _, "ὀ~νῠοισθον", "ὀ~νῠοισθε";
    opt_pass_3: "ὀ~νῠοιτο", "ὀ~νῠοισθην", "ὀ~νῠοιντο";
    imp_act_2: "ὀ~νῡ|ὀ~νῠθῐ", "ὀ~νῠτον", "ὀ~νῠτε";
    imp_act_3: "ὀ~νῠτω", "ὀ~νῠτων", "ὀ~νῠντων";
    imp_mid_2: "ὀ~νῠσο|ὀ~νυσθε", _, "ὀ~νῠσθε";
    imp_mid_3: "ὀ~νῠσθω", "ὀ~νῠσθων", "ὀ~νῠσθων";
    imp_pass_2: "ὀ~νῠσο|ὀ~νυσθε", _, "ὀ~νῠσθε";
    imp_pass_3: _, "ὀ~νῠσθων", _;
    imperfect_act_1: "ὠ~νῡν", _, "ὠ~νῠμεν";
    imperfect_act_2: "ὠ~νῡς", "ὠ~νῠτον", "ὠ~νῠτε";
    imperfect_act_3: "ὠ~νῡ", "ὠ~νῠτην", "ὠ~νῠσᾰν";
    imperfect_mid_1: "ὠ~νῠμην", _, "ὠ~νῠμεθᾰ";
    imperfect_mid_2: "ὠ~νῠσο", "ὠ~νῠσθον", "ὠ~νῠσθε";
    imperfect_mid_3: "ὠ~νῠτο", "ὠ~νῠσθην", "ὠ~νῠντο";
    imperfect_pass_1: _, _, "ὠ~νῠμεθᾰ";
    imperfect_pass_2: _, _, "ὠ~νῠσθε";
    imperfect_pass_3: "ὠ~νῠτο", _, _;
    future_ind_act_1: "ὀ~σω", _, "ὀ~σομεν";
    future_ind_act_2: "ὀ~σεις", "ὀ~σετον", "ὀ~σετε";
    future_ind_act_3: "ὀ~σει", _, "ὀ~σουσῐ|ὀ~σουσῐν";
    future_ind_mid_1: "ὀ~σομαι", _, "ὀ~σομεθᾰ";
    future_ind_mid_2: "ὀ~σῃ|ὀ~σει", "ὀ~σεσθον", "ὀ~σεσθε";
    future_ind_mid_3: "ὀ~σεται", _, "ὀ~σονται";
    future_opt_act_1: "ὀ~σοιμῐ", _, "ὀ~σοιμεν";
    future_opt_act_2: "ὀ~σοις", "ὀ~σοιτον", "ὀ~σοιτε";
    future_opt_act_3: "ὀ~σοι", "ὀ~σοιτην", "ὀ~σοιεν";
    future_opt_mid_1: "ὀ~σοιμην", _, "ὀ~σοιμεθᾰ";
    future_opt_mid_2: "ὀ~σοιο", "ὀ~σοισθον", "ὀ~σοισθε";
    future_opt_mid_3: "ὀ~σοιτο", "ὀ~σοισθην", "ὀ~σοιντο";
    aorist_ind_act_1: "ὠ~σᾰ", _, "ὠ~σᾰμεν";
    aorist_ind_act_2: "ὠ~σᾰς", "ὠ~σᾰτον", "ὠ~σᾰτε";
    aorist_ind_act_3: "ὠ~σε|ὠ~σεν", "ὠ~σᾰτην", "ὠ~σᾰν";
    aorist_ind_mid_1: "ὠ~σᾰμην", _, "ὠ~σᾰμεθᾰ";
    aorist_ind_mid_2: "ὠ~σω", "ὠ~σᾰσθον", "ὠ~σᾰσθε";
    aorist_ind_mid_3: "ὠ~σᾰτο", "ὠ~σᾰσθην", "ὠ~σᾰντο";
    aorist_subj_act_1: "ὀ~σω", _, "ὀ~σωμεν";
    aorist_subj_act_2: "ὀ~σῃς", "ὀ~σητον", "ὀ~σητε";
    aorist_subj_act_3: "ὀ~σῃ", _, "ὀ~σωσῐ|ὀ~σωσῐν";
    aorist_subj_mid_1: "ὀ~σωμαι", _, "ὀ~σωμεθᾰ";
    aorist_subj_mid_2: "ὀ~σῃ", "ὀ~σησθον", "ὀ~σησθε";
    aorist_subj_mid_3: "ὀ~σηται", _, "ὀ~σωνται";
    aorist_opt_act_1: "ὀ~σαιμῐ", _, "ὀ~σαιμεν";
    aorist_opt_act_2: "ὀ~σειᾰς|ὀ~σαις", "ὀ~σαιτον", "ὀ~σαιτε";
    aorist_opt_act_3: "ὀ~σειε|ὀ~σειεν|ὀ~σαι", "ὀ~σαιτην", "ὀ~σειᾰν|ὀ~σαιεν";
    aorist_opt_mid_1: "ὀ~σαιμην", _, "ὀ~σαιμεθᾰ";
    aorist_opt_mid_2: "ὀ~σαιο", "ὀ~σαισθον", "ὀ~σαισθε";
    aorist_opt_mid_3: "ὀ~σαιτο", "ὀ~σαισθην", "ὀ~σαιντο";
    aorist_imp_act_2: "ὀ~σον", "ὀ~σᾰτον", "ὀ~σᾰτε";
    aorist_imp_act_3: "ὀ~σᾰτω", "ὀ~σᾰτων", "ὀ~σᾰντων";
    aorist_imp_mid_2: "ὀ~σαι", "ὀ~σᾰσθον", "ὀ~σᾰσθε";
    aorist_imp_mid_3: "ὀ~σᾰσθω", "ὀ~σᾰσθων", _;
    perfect_ind_act_1: "ὀ~ωρᾰ", _, "ὀ~ωρᾰμεν";
    perfect_ind_act_2: "ὀ~ωρᾰς", "ὀ~ωρᾰτον", "ὀ~ωρᾰτε";
    perfect_ind_act_3: "ὀ~ωρε|ὀ~ωρεν", _, "ὀ~ωρᾱσῐ|ὀ~ωρᾱσῐν";
    perfect_ind_mid_1: "ὀ~ωρεμαι", _, "ὀ~ωρεμεθᾰ";
    perfect_ind_mid_2: "ὀ~ωρεσαι", "ὀ~ωρεσθον", "ὀ~ωρεσθε";
    perfect_ind_mid_3: "ὀ~ωρεται", _, "ὀ~ωρενται";
    perfect_ind_pass_1: _, _, "ὀ~ωρεμεθᾰ";
    perfect_ind_pass_2: _, _, "ὀ~ωρεσθε";
    perfect_subj_act_1: "ὀ~ωρω", _, "ὀ~ωρωμεν";
    perfect_subj_act_2: "ὀ~ωρῃς", "ὀ~ωρητον", "ὀ~ωρητε";
    perfect_subj_act_3: "ὀ~ωρῃ", _, "ὀ~ωρωσῐ|ὀ~ωρωσῐν";
    perfect_opt_act_1: "ὀ~ωροιμῐ|ὀ~ωροιην", _, "ὀ~ωροιμεν";
    perfect_opt_act_2: "ὀ~ωροις|ὀ~ωροιης", "ὀ~ωροιτον", "ὀ~ωροιτε";
    perfect_opt_act_3: "ὀ~ωροι|ὀ~ωροιη", "ὀ~ωροιτην", "ὀ~ωροιεν";
    perfect_imp_act_2: "ὀ~ωρε", "ὀ~ωρετον", "ὀ~ωρετε";
    perfect_imp_act_3: "ὀ~ωρετω", "ὀ~ωρετων", "ὀ~ωροντων";
    perfect_imp_mid_2: "ὀ~ωρεσο", _, _;
    perfect_imp_mid_3: "ὀ~ωρεσθω", "ὀ~ωρεσθων", _;
    perfect_imp_pass_2: "ὀ~ωρεσο", _, _;
    perfect_imp_pass_3: "ὀ~ωρεσθω", "ὀ~ωρεσθων", _;
    pluperfect_act_1: "ὀ~ωρειν|ὀ~ωρη", _, "ὀ~ωρεμεν";
    pluperfect_act_2: "ὀ~ωρεις|ὀ~ωρης", "ὀ~ωρετον", "ὀ~ωρετε";
    pluperfect_act_3: "ὀ~ωρει|ὀ~ωρειν", "ὀ~ωρετην", "ὀ~ωρεσᾰν";
    pluperfect_mid_1: "ὀ~ωρεμην", _, "ὀ~ωρεμεθᾰ";
    pluperfect_mid_2: "ὀ~ωρεσο", "ὀ~ωρεσθον", "ὀ~ωρεσθε";
    pluperfect_mid_3: "ὀ~ωρετο", "ὀ~ωρεσθην", _;
    pluperfect_pass_1: _, _, "ὀ~ωρεμεθᾰ";
    pluperfect_pass_2: _, _, "ὀ~ωρεσθε";
    pluperfect_pass_3: _, "ὀ~ωρεσθην", "ὀ~ωρεντο";
    infinitive_pres: "ὀ~νῠναι", "ὀ~νῠσθαι", _;
    infinitive_fut: "ὀ~σειν", "ὀ~σεσθαι", _;
    infinitive_aor: "ὀ~σαι", "ὀ~σᾰσθαι", _;
    infinitive_perf: "ὀ~ωρεναι", "ὀ~ωρεσθαι", _;
    participle_pres: "ὀ~νῡς", "ὀ~νῠμενος", _;
    participle_fut: "ὀ~σων", "ὀ~σομενος", _;
    participle_aor: "ὀ~σᾱς", "ὀ~σᾰμενος", _;
    participle_perf: "ὀ~ωρως", "ὀ~ωρεμενος", _;
    ),
    // Πηγνῡμῐ: 1 words (πηγνῡμῐ). Like Γῡροω, except in 304 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("πηγνῡμῐ", 1, "~ηγνῡμῐ", like Γῡροω;
    ind_act_1: "~ηγνῡμῐ", _, "~ηγνῠμεν";
    ind_act_2: "~ηγνῡς", _, _;
    ind_act_3: "~ηγνῡσῐ|~ηγνῡσῐν", _, "~ηγνῠᾱσῐ|~ηγνῠᾱσῐν";
    ind_mid_1: "~ηγνῠμαι", _, "~ηγνῠμεθᾰ";
    ind_mid_2: "~ηγνῠσαι", "~ηγνῠσθον", "~ηγνῠσθε";
    ind_mid_3: "~ηγνῠται", _, "~ηγνῠνται";
    ind_pass_1: _, _, "~ηγνῠμεθᾰ";
    ind_pass_2: _, _, "~ηγνῠσθε";
    ind_pass_3: "~ηγνῠται", _, "~ηγνῠνται";
    subj_act_1: "~ηγνῠω", _, "~ηγνῠωμεν";
    subj_act_2: "~ηγνῠῃς", "~ηγνῠητον", "~ηγνῠητε";
    subj_act_3: "~ηγνῠῃ", _, "~ηγνῠωσῐ|~ηγνῠωσῐν";
    subj_mid_1: "~ηγνῠωμαι", _, "~ηγνῠωμεθᾰ";
    subj_mid_2: "~ηγνῠῃ", "~ηγνῠησθον", "~ηγνῠησθε";
    subj_mid_3: "~ηγνῠηται", _, "~ηγνῠωνται";
    subj_pass_1: _, _, "~ηγνῠωμεθᾰ";
    subj_pass_2: _, _, "~ηγνῠησθε";
    subj_pass_3: "~ηγνῠηται", _, "~ηγνῠωνται";
    opt_act_1: "~ηγνῠοιμῐ", _, "~ηγνῠοιμεν";
    opt_act_2: "~ηγνῠοις", "~ηγνῠοιτον", "~ηγνῠοιτε";
    opt_act_3: "~ηγνῠοι", "~ηγνῠοιτην", "~ηγνῠοιεν";
    opt_mid_1: "~ηγνῠοιμην", _, "~ηγνῠοιμεθᾰ";
    opt_mid_2: "~ηγνῠοιο", "~ηγνῠοισθον", "~ηγνῠοισθε";
    opt_mid_3: "~ηγνῠοιτο", "~ηγνῠοισθην", "~ηγνῠοιντο";
    opt_pass_1: _, _, "~ηγνῠοιμεθᾰ";
    opt_pass_2: _, "~ηγνῠοισθον", "~ηγνῠοισθε";
    opt_pass_3: "~ηγνῠοιτο", "~ηγνῠοισθην", "~ηγνῠοιντο";
    imp_act_2: "~ηγνῡ", "~ηγνῠτον", "~ηγνῠτε";
    imp_act_3: "~ηγνῠτω", "~ηγνῠτων", "~ηγνῠντων";
    imp_mid_2: "~ηγνῠσο", _, "~ηγνῠσθε";
    imp_mid_3: "~ηγνῠσθω", "~ηγνῠσθων", "~ηγνῠσθων";
    imp_pass_2: _, _, "~ηγνῠσθε";
    imp_pass_3: _, "~ηγνῠσθων", _;
    imperfect_act_1: "~ηγνῡν", _, "~ηγνῠμεν";
    imperfect_act_2: "~ηγνῡς", "~ηγνῠτον", "~ηγνῠτε";
    imperfect_act_3: "~ηγνῡ", "~ηγνῠτην", "~ηγνῠσᾰν";
    imperfect_mid_1: "~ηγνῠμην", _, "~ηγνῠμεθᾰ";
    imperfect_mid_2: "~ηγνῠσο", "~ηγνῠσθον", "~ηγνῠσθε";
    imperfect_mid_3: "~ηγνῠτο", "~ηγνῠσθην", "~ηγνῠντο";
    imperfect_pass_1: _, _, "~ηγνῠμεθᾰ";
    imperfect_pass_2: _, _, "~ηγνῠσθε";
    imperfect_pass_3: "~ηγνῠτο", _, _;
    future_ind_act_1: "~ηξω", _, "~ηξομεν";
    future_ind_act_2: "~ηξεις", "~ηξετον", "~ηξετε";
    future_ind_act_3: "~ηξει", _, "~ηξουσῐ|~ηξουσῐν";
    future_ind_mid_1: "~ηξομαι", _, "~ηξομεθᾰ";
    future_ind_mid_2: "~ηξῃ|~ηξει", "~ηξεσθον", "~ηξεσθε";
    future_ind_mid_3: "~ηξεται", _, "~ηξονται";
    future_ind_pass_1: "~ᾰγησομαι", _, "~ᾰγησομεθᾰ";
    future_ind_pass_2: "~ᾰγησῃ|~ᾰγησει", "~ᾰγησεσθον", "~ᾰγησεσθε";
    future_ind_pass_3: "~ᾰγησεται", _, "~ᾰγησονται";
    future_opt_act_1: "~ηξοιμῐ", _, "~ηξοιμεν";
    future_opt_act_2: "~ηξοις", "~ηξοιτον", "~ηξοιτε";
    future_opt_act_3: "~ηξοι", "~ηξοιτην", "~ηξοιεν";
    future_opt_mid_1: "~ηξοιμην", _, "~ηξοιμεθᾰ";
    future_opt_mid_2: "~ηξοιο", "~ηξοισθον", "~ηξοισθε";
    future_opt_mid_3: "~ηξοιτο", "~ηξοισθην", "~ηξοιντο";
    future_opt_pass_1: "~ᾰγησοιμην", _, "~ᾰγησοιμεθᾰ";
    future_opt_pass_2: "~ᾰγησοιο", "~ᾰγησοισθον", "~ᾰγησοισθε";
    future_opt_pass_3: "~ᾰγησοιτο", "~ᾰγησοισθην", "~ᾰγησοιντο";
    aorist_ind_act_1: "ἐ~ηξᾰ", _, "ἐ~ηξᾰμεν";
    aorist_ind_act_2: "ἐ~ηξᾰς", "ἐ~ηξᾰτον", "ἐ~ηξᾰτε";
    aorist_ind_act_3: "ἐ~ηξε|ἐ~ηξεν", "ἐ~ηξᾰτην", "ἐ~ηξᾰν";
    aorist_ind_mid_1: "ἐ~ηξᾰμην", _, "ἐ~ηξᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ηξω", "ἐ~ηξᾰσθον", "ἐ~ηξᾰσθε";
    aorist_ind_mid_3: "ἐ~ηξᾰτο", "ἐ~ηξᾰσθην", "ἐ~ηξᾰντο";
    aorist_ind_pass_1: "ἐ~ηχθην|ἐ~ᾰγην", _, "ἐ~ηχθημεν|ἐ~ᾰγημεν";
    aorist_ind_pass_2: "ἐ~ηχθης|ἐ~ᾰγης", "ἐ~ηχθητον|ἐ~ᾰγητον", "ἐ~ηχθητε|ἐ~ᾰγητε";
    aorist_ind_pass_3: "ἐ~ηχθη|ἐ~ᾰγη", "ἐ~ηχθητην|ἐ~ᾰγητην", "ἐ~ηχθησᾰν|ἐ~ᾰγησᾰν";
    aorist_subj_act_1: "~ηξω", _, "~ηξωμεν";
    aorist_subj_act_2: "~ηξῃς", "~ηξητον", "~ηξητε";
    aorist_subj_act_3: "~ηξῃ", _, "~ηξωσῐ|~ηξωσῐν";
    aorist_subj_mid_1: "~ηξωμαι", _, "~ηξωμεθᾰ";
    aorist_subj_mid_2: "~ηξῃ", "~ηξησθον", "~ηξησθε";
    aorist_subj_mid_3: "~ηξηται", _, "~ηξωνται";
    aorist_subj_pass_1: "~ηχθω|~ᾰγω", _, "~ηχθωμεν|~ᾰγωμεν";
    aorist_subj_pass_2: "~ηχθῃς|~ᾰγῃς", "~ηχθητον|~ᾰγητον", "~ηχθητε|~ᾰγητε";
    aorist_subj_pass_3: "~ηχθῃ|~ᾰγῃ", "~ηχθητον|~ᾰγητον", "~ηχθωσῐ|~ηχθωσῐν|~ᾰγωσῐ|~ᾰγωσῐν";
    aorist_opt_act_1: "~ηξαιμῐ", _, "~ηξαιμεν";
    aorist_opt_act_2: "~ηξειᾰς|~ηξαις", "~ηξαιτον", "~ηξαιτε";
    aorist_opt_act_3: "~ηξειε|~ηξειεν|~ηξαι", "~ηξαιτην", "~ηξειᾰν|~ηξαιεν";
    aorist_opt_mid_1: "~ηξαιμην", _, "~ηξαιμεθᾰ";
    aorist_opt_mid_2: "~ηξαιο", "~ηξαισθον", "~ηξαισθε";
    aorist_opt_mid_3: "~ηξαιτο", "~ηξαισθην", "~ηξαιντο";
    aorist_opt_pass_1: "~ηχθειην|~ᾰγειην", _, "~ηχθεῑμεν|~ηχθειημεν|~ᾰγεῑμεν|~ᾰγειημεν";
    aorist_opt_pass_2: "~ηχθειης|~ᾰγειης", "~ηχθεῑτον|~ηχθειητον|~ᾰγεῑτον|~ᾰγειητον", "~ηχθεῑτε|~ηχθειητε|~ᾰγεῑτε|~ᾰγειητε";
    aorist_opt_pass_3: "~ηχθειη|~ᾰγειη", "~ηχθειτην|~ηχθειητην|~ᾰγειτην|~ᾰγειητην", "~ηχθεῑεν|~ηχθειησᾰν|~ᾰγεῑεν|~ᾰγειησᾰν";
    aorist_imp_act_2: "~ηξον", "~ηξᾰτον", "~ηξᾰτε";
    aorist_imp_act_3: "~ηξᾰτω", "~ηξᾰτων", "~ηξᾰντων";
    aorist_imp_mid_2: "~ηξαι", "~ηξᾰσθον", "~ηξᾰσθε";
    aorist_imp_mid_3: "~ηξᾰσθω", "~ηξᾰσθων", _;
    aorist_imp_pass_2: "~ηχθητῐ|~ᾰγηθῐ", "~ηχθητον|~ᾰγητον", "~ηχθητε|~ᾰγητε";
    aorist_imp_pass_3: "~ηχθητω|~ᾰγητω", "~ηχθητων|~ᾰγητων", "~ηχθεντων|~ᾰγεντων";
    perfect_ind_act_1: "~επηχᾰ|~επηγᾰ", _, "~επηχᾰμεν|~επηγᾰμεν";
    perfect_ind_act_2: "~επηχᾰς|~επηγᾰς", "~επηχᾰτον|~επηγᾰτον", "~επηχᾰτε|~επηγᾰτε";
    perfect_ind_act_3: "~επηχε|~επηχεν|~επηγε|~επηγεν", "~επηχᾰτον|~επηγᾰτον", "~επηχᾱσῐ|~επηχᾱσῐν|~επηγᾱσῐ|~επηγᾱσῐν";
    perfect_ind_mid_1: "~επηγμαι", _, "~επηγμεθᾰ";
    perfect_ind_mid_2: "~επηξαι", "~επηχθον", "~επηχθε";
    perfect_ind_mid_3: "~επηκται", _, "~επηγᾰται";
    perfect_ind_pass_1: _, _, "~επηγμεθᾰ";
    perfect_ind_pass_2: _, _, "~επηχθε";
    perfect_subj_act_1: "~επηχω|~επηγω", _, "~επηχωμεν|~επηγωμεν";
    perfect_subj_act_2: "~επηχῃς|~επηγῃς", "~επηχητον|~επηγητον", "~επηχητε|~επηγητε";
    perfect_subj_act_3: "~επηχῃ|~επηγῃ", "~επηχητον|~επηγητον", "~επηχωσῐ|~επηχωσῐν|~επηγωσῐ|~επηγωσῐν";
    perfect_opt_act_1: "~επηχοιμῐ|~επηχοιην|~επηγοιμῐ|~επηγοιην", _, "~επηχοιμεν|~επηγοιμεν";
    perfect_opt_act_2: "~επηχοις|~επηχοιης|~επηγοις|~επηγοιης", "~επηχοιτον|~επηγοιτον", "~επηχοιτε|~επηγοιτε";
    perfect_opt_act_3: "~επηχοι|~επηχοιη|~επηγοι|~επηγοιη", "~επηχοιτην|~επηγοιτην", "~επηχοιεν|~επηγοιεν";
    perfect_imp_act_2: "~επηχε|~επηγε", "~επηχετον|~επηγετον", "~επηχετε|~επηγετε";
    perfect_imp_act_3: "~επηχετω|~επηγετω", "~επηχετων|~επηγετων", "~επηχοντων|~επηγοντων";
    perfect_imp_mid_2: "~επηξο", _, _;
    perfect_imp_mid_3: "~επηχθω", "~επηχθων", _;
    perfect_imp_pass_2: "~επηξο", _, _;
    perfect_imp_pass_3: "~επηχθω", "~επηχθων", _;
    pluperfect_act_1: "ἐ~επηχειν|ἐ~επηχη|ἐ~επηγειν|ἐ~επηγη", _, "ἐ~επηχεμεν|ἐ~επηγεμεν";
    pluperfect_act_2: "ἐ~επηχεις|ἐ~επηχης|ἐ~επηγεις|ἐ~επηγης", "ἐ~επηχετον|ἐ~επηγετον", "ἐ~επηχετε|ἐ~επηγετε";
    pluperfect_act_3: "ἐ~επηχει|ἐ~επηχειν|ἐ~επηγει|ἐ~επηγειν", "ἐ~επηχετην|ἐ~επηγετην", "ἐ~επηχεσᾰν|ἐ~επηγεσᾰν";
    pluperfect_mid_1: "ἐ~επηγμην", _, "ἐ~επηγμεθᾰ";
    pluperfect_mid_2: "ἐ~επηξο", "ἐ~επηχθον", "ἐ~επηχθε";
    pluperfect_mid_3: "ἐ~επηκτο", "ἐ~επηχθην", _;
    pluperfect_pass_1: _, _, "ἐ~επηγμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~επηχθε";
    pluperfect_pass_3: _, "ἐ~επηχθην", "ἐ~επηγᾰτο";
    infinitive_pres: "~ηγνῠναι", "~ηγνῠσθαι", _;
    infinitive_fut: "~ηξειν", "~ηξεσθαι", "~ᾰγησεσθαι";
    infinitive_aor: "~ηξαι", "~ηξᾰσθαι", "~ηχθηναι|~ᾰγηναι";
    infinitive_perf: "~επηχεναι|~επηγεναι", "~επηχθαι", _;
    participle_pres: "~ηγνῡς", "~ηγνῠμενος", _;
    participle_fut: "~ηξων", "~ηξομενος", "~ᾰγησομενος";
    participle_aor: "~ηξᾱς", "~ηξᾰμενος", "~ηχθεις|~ᾰγεις";
    participle_perf: "~επηχως|~επηγως", "~επηγμενος", _;
    ),
    // Ἀντᾰποδῐδωμῐ: 1 words (ἀντᾰποδῐδωμῐ). Like Λιστραω, except in 267 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἀντᾰποδῐδωμῐ", 1, "~ο~ῐδωμῐ", like Λιστραω;
    ind_act_1: "~ο~ῐδωμῐ", _, "~ο~ῐδομεν";
    ind_act_2: "~ο~ῐδως", _, _;
    ind_act_3: "~ο~ῐδωσῐ|~ο~ῐδωσῐν", _, "~ο~ῐδοᾱσῐ|~ο~ῐδοᾱσῐν";
    ind_mid_1: "~ο~ῐδομαι", _, "~ο~ῐδομεθᾰ";
    ind_mid_2: "~ο~ῐδοσαι", "~ο~ῐδοσθον", "~ο~ῐδοσθε";
    ind_mid_3: "~ο~ῐδοται", _, "~ο~ῐδονται";
    ind_pass_1: _, _, "~ο~ῐδομεθᾰ";
    ind_pass_2: _, _, "~ο~ῐδοσθε";
    ind_pass_3: "~ο~ῐδοται", _, "~ο~ῐδονται";
    subj_act_1: "~ο~ῐδω", _, "~ο~ῐδωμεν";
    subj_act_2: "~ο~ῐδῳς", "~ο~ῐδωτον", "~ο~ῐδωτε";
    subj_act_3: "~ο~ῐδῳ", _, "~ο~ῐδωσῐ|~ο~ῐδωσῐν";
    subj_mid_1: "~ο~ῐδωμαι", _, "~ο~ῐδωμεθᾰ";
    subj_mid_2: "~ο~ῐδῳ", "~ο~ῐδωσθον", "~ο~ῐδωσθε";
    subj_mid_3: "~ο~ῐδωται", _, "~ο~ῐδωνται";
    subj_pass_1: _, _, "~ο~ῐδωμεθᾰ";
    subj_pass_2: _, _, "~ο~ῐδωσθε";
    subj_pass_3: "~ο~ῐδωται", _, "~ο~ῐδωνται";
    opt_act_1: "~ο~ῐδοιην", _, "~ο~ῐδοῑμεν|~ο~ῐδοιημεν";
    opt_act_2: "~ο~ῐδοιης", "~ο~ῐδοῑτον|~ο~ῐδοιητον", "~ο~ῐδοῑτε|~ο~ῐδοιητε";
    opt_act_3: "~ο~ῐδοιη", "~ο~ῐδοιτην|~ο~ῐδοιητην", "~ο~ῐδοῑεν|~ο~ῐδοιησᾰν";
    opt_mid_1: "~ο~ῐδοιμην", _, "~ο~ῐδοιμεθᾰ";
    opt_mid_2: "~ο~ῐδοῑο", "~ο~ῐδοῑσθον", "~ο~ῐδοῑσθε";
    opt_mid_3: "~ο~ῐδοῑτο", "~ο~ῐδοισθην", "~ο~ῐδοῑντο";
    opt_pass_1: _, _, "~ο~ῐδοιμεθᾰ";
    opt_pass_2: _, "~ο~ῐδοῑσθον", "~ο~ῐδοῑσθε";
    opt_pass_3: "~ο~ῐδοῑτο", "~ο~ῐδοισθην", "~ο~ῐδοῑντο";
    imp_act_2: "~ο~ῐδου", "~ο~ῐδοτον", "~ο~ῐδοτε";
    imp_act_3: "~ο~ῐδοτω", "~ο~ῐδοτων", "~ο~ῐδοντων";
    imp_mid_2: "~ο~ῐδοσο|~ο~ῐδου", _, "~ο~ῐδοσθε";
    imp_mid_3: "~ο~ῐδοσθω", "~ο~ῐδοσθων", "~ο~ῐδοσθων";
    imp_pass_2: "~ο~ῐδοσο|~ο~ῐδου", _, "~ο~ῐδοσθε";
    imp_pass_3: _, "~ο~ῐδοσθων", _;
    imperfect_act_1: "~ε~ῐδουν", _, "~ε~ῐδομεν";
    imperfect_act_2: "~ε~ῐδους", "~ε~ῐδοτον", "~ε~ῐδοτε";
    imperfect_act_3: "~ε~ῐδου", "~ε~ῐδοτην", "~ε~ῐδοσᾰν";
    imperfect_mid_1: "~ε~ῐδομην", _, "~ε~ῐδομεθᾰ";
    imperfect_mid_2: "~ε~ῐδοσο", "~ε~ῐδοσθον", "~ε~ῐδοσθε";
    imperfect_mid_3: "~ε~ῐδοτο", "~ε~ῐδοσθην", "~ε~ῐδοντο";
    imperfect_pass_1: _, _, "~ε~ῐδομεθᾰ";
    imperfect_pass_2: _, _, "~ε~ῐδοσθε";
    imperfect_pass_3: "~ε~ῐδοτο", _, _;
    future_ind_act_1: "~ο~ωσω", _, "~ο~ωσομεν";
    future_ind_act_2: "~ο~ωσεις", "~ο~ωσετον", "~ο~ωσετε";
    future_ind_act_3: "~ο~ωσει", _, "~ο~ωσουσῐ|~ο~ωσουσῐν";
    future_ind_mid_1: "~ο~ωσομαι", _, "~ο~ωσομεθᾰ";
    future_ind_mid_2: "~ο~ωσῃ|~ο~ωσει", "~ο~ωσεσθον", "~ο~ωσεσθε";
    future_ind_mid_3: "~ο~ωσεται", _, "~ο~ωσονται";
    future_opt_act_1: "~ο~ωσοιμῐ", _, "~ο~ωσοιμεν";
    future_opt_act_2: "~ο~ωσοις", "~ο~ωσοιτον", "~ο~ωσοιτε";
    future_opt_act_3: "~ο~ωσοι", "~ο~ωσοιτην", "~ο~ωσοιεν";
    future_opt_mid_1: "~ο~ωσοιμην", _, "~ο~ωσοιμεθᾰ";
    future_opt_mid_2: "~ο~ωσοιο", "~ο~ωσοισθον", "~ο~ωσοισθε";
    future_opt_mid_3: "~ο~ωσοιτο", "~ο~ωσοισθην", "~ο~ωσοιντο";
    aorist_ind_act_1: "~ε~ωκᾰ", _, "~ε~ομεν|~ε~ωκᾰμεν";
    aorist_ind_act_2: "~ε~ωκᾰς", "~ε~οτον|~ε~ωκᾰτον", "~ε~οτε|~ε~ωκᾰτε";
    aorist_ind_act_3: "~ε~ωκε|~ε~ωκεν", "~ε~οτην|~ε~ωκᾰτην", "~ε~οσᾰν|~ε~ωκᾰν";
    aorist_ind_mid_1: "~ε~ομην|~ε~ωκᾰμην", _, "~ε~ομεθᾰ|~ε~ωκᾰμεθᾰ";
    aorist_ind_mid_2: "~ε~ου|~ε~ωκω", "~ε~οσθον|~ε~ωκᾰσθον", "~ε~οσθε|~ε~ωκᾰσθε";
    aorist_ind_mid_3: "~ε~οτο|~ε~ωκᾰτο", "~ε~οσθην|~ε~ωκᾰσθην", "~ε~οντο|~ε~ωκᾰντο";
    aorist_ind_pass_1: "~ε~οθην", _, "~ε~οθημεν";
    aorist_ind_pass_2: "~ε~οθης", "~ε~οθητον", "~ε~οθητε";
    aorist_ind_pass_3: "~ε~οθη", "~ε~οθητην", "~ε~οθησᾰν";
    aorist_subj_act_1: "~ο~ω|~ο~ωκω", _, "~ο~ωμεν|~ο~ωκωμεν";
    aorist_subj_act_2: "~ο~ῳς|~ο~ωκῃς", "~ο~ωτον|~ο~ωκητον", "~ο~ωτε|~ο~ωκητε";
    aorist_subj_act_3: "~ο~ῳ|~ο~ωκῃ", "~ο~ωτον|~ο~ωκητον", "~ο~ωσῐ|~ο~ωσῐν|~ο~ωκωσῐ|~ο~ωκωσῐν";
    aorist_subj_mid_1: "~ο~ωμαι|~ο~ωκωμαι", _, "~ο~ωμεθᾰ|~ο~ωκωμεθᾰ";
    aorist_subj_mid_2: "~ο~ῳ|~ο~ωκῃ", "~ο~ωσθον|~ο~ωκησθον", "~ο~ωσθε|~ο~ωκησθε";
    aorist_subj_mid_3: "~ο~ωται|~ο~ωκηται", "~ο~ωσθον|~ο~ωκησθον", "~ο~ωνται|~ο~ωκωνται";
    aorist_subj_pass_1: "~ο~οθω", _, "~ο~οθωμεν";
    aorist_subj_pass_2: "~ο~οθῃς", "~ο~οθητον", "~ο~οθητε";
    aorist_subj_pass_3: "~ο~οθῃ", _, "~ο~οθωσῐ|~ο~οθωσῐν";
    aorist_opt_act_1: "~ο~οιην|~ο~ωκαιμῐ", _, "~ο~οῑμεν|~ο~οιημεν|~ο~ωκαιμεν";
    aorist_opt_act_2: "~ο~οιης|~ο~ωκειᾰς|~ο~ωκαις", "~ο~οῑτον|~ο~οιητον|~ο~ωκαιτον", "~ο~οῑτε|~ο~οιητε|~ο~ωκαιτε";
    aorist_opt_act_3: "~ο~οιη|~ο~ωκειε|~ο~ωκειεν|~ο~ωκαι", "~ο~οιτην|~ο~οιητην|~ο~ωκαιτην", "~ο~οῑεν|~ο~οιησᾰν|~ο~ωκειᾰν|~ο~ωκαιεν";
    aorist_opt_mid_1: "~ο~οιμην|~ο~ωκαιμην", _, "~ο~οιμεθᾰ|~ο~ωκαιμεθᾰ";
    aorist_opt_mid_2: "~ο~οῑο|~ο~ωκαιο", "~ο~οῑσθον|~ο~ωκαισθον", "~ο~οῑσθε|~ο~ωκαισθε";
    aorist_opt_mid_3: "~ο~οῑτο|~ο~ωκαιτο", "~ο~οισθην|~ο~ωκαισθην", "~ο~οῑντο|~ο~ωκαιντο";
    aorist_opt_pass_1: "~ο~οθειην", _, "~ο~οθεῑμεν|~ο~οθειημεν";
    aorist_opt_pass_2: "~ο~οθειης", "~ο~οθεῑτον|~ο~οθειητον", "~ο~οθεῑτε|~ο~οθειητε";
    aorist_opt_pass_3: "~ο~οθειη", "~ο~οθειτην|~ο~οθειητην", "~ο~οθεῑεν|~ο~οθειησᾰν";
    aorist_imp_act_2: "~ο~ος|~ο~ωκον", "~ο~οτον|~ο~ωκᾰτον", "~ο~οτε|~ο~ωκᾰτε";
    aorist_imp_act_3: "~ο~οτω|~ο~ωκᾰτω", "~ο~οτων|~ο~ωκᾰτων", "~ο~οντων|~ο~ωκᾰντων";
    aorist_imp_mid_2: "~ο~ου|~ο~ωκαι", "~ο~οσθον|~ο~ωκᾰσθον", "~ο~οσθε|~ο~ωκᾰσθε";
    aorist_imp_mid_3: "~ο~οσθω|~ο~ωκᾰσθω", "~ο~οσθων|~ο~ωκᾰσθων", "~ο~οσθων|~ο~ωκᾰσθων";
    aorist_imp_pass_2: "~ο~οθηθῐ", _, _;
    aorist_imp_pass_3: "~ο~οθητω", "~ο~οθητων", "~ο~οθεντων";
    perfect_ind_act_1: "~ο~εδωκᾰ", _, "~ο~εδωκᾰμεν";
    perfect_ind_act_2: "~ο~εδωκᾰς", "~ο~εδωκᾰτον", "~ο~εδωκᾰτε";
    perfect_ind_act_3: "~ο~εδωκε|~ο~εδωκεν", _, "~ο~εδωκᾱσῐ|~ο~εδωκᾱσῐν";
    perfect_ind_mid_1: "~ο~εδομαι", _, "~ο~εδομεθᾰ";
    perfect_ind_mid_2: "~ο~εδοσαι", "~ο~εδοσθον", "~ο~εδοσθε";
    perfect_ind_mid_3: "~ο~εδοται", _, "~ο~εδονται";
    perfect_ind_pass_1: _, _, "~ο~εδομεθᾰ";
    perfect_ind_pass_2: _, _, "~ο~εδοσθε";
    perfect_subj_act_1: "~ο~εδωκω", _, "~ο~εδωκωμεν";
    perfect_subj_act_2: "~ο~εδωκῃς", "~ο~εδωκητον", "~ο~εδωκητε";
    perfect_subj_act_3: "~ο~εδωκῃ", _, "~ο~εδωκωσῐ|~ο~εδωκωσῐν";
    perfect_opt_act_1: "~ο~εδωκοιμῐ|~ο~εδωκοιην", _, "~ο~εδωκοιμεν";
    perfect_opt_act_2: "~ο~εδωκοις|~ο~εδωκοιης", "~ο~εδωκοιτον", "~ο~εδωκοιτε";
    perfect_opt_act_3: "~ο~εδωκοι|~ο~εδωκοιη", "~ο~εδωκοιτην", "~ο~εδωκοιεν";
    perfect_imp_act_2: "~ο~εδωκε", "~ο~εδωκετον", "~ο~εδωκετε";
    perfect_imp_act_3: "~ο~εδωκετω", "~ο~εδωκετων", "~ο~εδωκοντων";
    perfect_imp_mid_2: "~ο~εδοσο", _, _;
    perfect_imp_mid_3: "~ο~εδοσθω", "~ο~εδοσθων", _;
    perfect_imp_pass_2: "~ο~εδοσο", _, _;
    perfect_imp_pass_3: "~ο~εδοσθω", "~ο~εδοσθων", _;
    infinitive_pres: "~ο~ῐδοναι", "~ο~ῐδοσθαι", _;
    infinitive_fut: "~ο~ωσειν", "~ο~ωσεσθαι", _;
    infinitive_aor: "~ο~οῡναι|~ο~ωκαι", "~ο~οσθαι|~ο~ωκᾰσθαι", "~ο~οθηναι";
    infinitive_perf: "~ο~εδωκεναι", "~ο~εδοσθαι", _;
    participle_pres: "~ο~ῐδους", "~ο~ῐδομενος", _;
    participle_fut: "~ο~ωσων", "~ο~ωσομενος", _;
    participle_aor: "~ο~ους|~ο~ωκᾱς", "~ο~ομενος|~ο~ωκᾰμενος", "~ο~οθεις";
    participle_perf: "~ο~εδωκως", "~ο~εδομενος", _;
    ),
    // Πετᾰννῡμῐ: 1 words (πετᾰννῡμῐ). Like Προσερχομαι, except in 301 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("πετᾰννῡμῐ", 1, "~~ᾰννῡμῐ", like Προσερχομαι;
    ind_act_1: "~~ᾰννῡμῐ", _, "~~ᾰννῠμεν";
    ind_act_2: "~~ᾰννῡς", _, _;
    ind_act_3: "~~ᾰννῡσῐ|~~ᾰννῡσῐν", _, "~~ᾰννῠᾱσῐ|~~ᾰννῠᾱσῐν";
    ind_mid_1: "~~ᾰννῠμαι", _, "~~ᾰννῠμεθᾰ";
    ind_mid_2: "~~ᾰννῠσαι", "~~ᾰννῠσθον", "~~ᾰννῠσθε";
    ind_mid_3: "~~ᾰννῠται", _, "~~ᾰννῠνται";
    ind_pass_1: _, _, "~~ᾰννῠμεθᾰ";
    ind_pass_2: _, _, "~~ᾰννῠσθε";
    ind_pass_3: "~~ᾰννῠται", _, "~~ᾰννῠνται";
    subj_act_1: "~~ᾰννῠω", _, "~~ᾰννῠωμεν";
    subj_act_2: "~~ᾰννῠῃς", "~~ᾰννῠητον", "~~ᾰννῠητε";
    subj_act_3: "~~ᾰννῠῃ", _, "~~ᾰννῠωσῐ|~~ᾰννῠωσῐν";
    subj_mid_1: "~~ᾰννῠωμαι", _, "~~ᾰννῠωμεθᾰ";
    subj_mid_2: "~~ᾰννῠῃ", "~~ᾰννῠησθον", "~~ᾰννῠησθε";
    subj_mid_3: "~~ᾰννῠηται", _, "~~ᾰννῠωνται";
    subj_pass_1: _, _, "~~ᾰννῠωμεθᾰ";
    subj_pass_2: _, _, "~~ᾰννῠησθε";
    subj_pass_3: "~~ᾰννῠηται", _, "~~ᾰννῠωνται";
    opt_act_1: "~~ᾰννῠοιμῐ", _, "~~ᾰννῠοιμεν";
    opt_act_2: "~~ᾰννῠοις", "~~ᾰννῠοιτον", "~~ᾰννῠοιτε";
    opt_act_3: "~~ᾰννῠοι", "~~ᾰννῠοιτην", "~~ᾰννῠοιεν";
    opt_mid_1: "~~ᾰννῠοιμην", _, "~~ᾰννῠοιμεθᾰ";
    opt_mid_2: "~~ᾰννῠοιο", "~~ᾰννῠοισθον", "~~ᾰννῠοισθε";
    opt_mid_3: "~~ᾰννῠοιτο", "~~ᾰννῠοισθην", "~~ᾰννῠοιντο";
    opt_pass_1: _, _, "~~ᾰννῠοιμεθᾰ";
    opt_pass_2: _, "~~ᾰννῠοισθον", "~~ᾰννῠοισθε";
    opt_pass_3: "~~ᾰννῠοιτο", "~~ᾰννῠοισθην", "~~ᾰννῠοιντο";
    imp_act_2: "~~ᾰννῡ", "~~ᾰννῠτον", "~~ᾰννῠτε";
    imp_act_3: "~~ᾰννῠτω", "~~ᾰννῠτων", "~~ᾰννῠντων";
    imp_mid_2: "~~ᾰννῠσο", _, "~~ᾰννῠσθε";
    imp_mid_3: "~~ᾰννῠσθω", "~~ᾰννῠσθων", "~~ᾰννῠσθων";
    imp_pass_2: _, _, "~~ᾰννῠσθε";
    imp_pass_3: _, "~~ᾰννῠσθων", _;
    imperfect_act_1: "ἐ~~ᾰννῡν", _, "ἐ~~ᾰννῠμεν";
    imperfect_act_2: "ἐ~~ᾰννῡς", "ἐ~~ᾰννῠτον", "ἐ~~ᾰννῠτε";
    imperfect_act_3: "ἐ~~ᾰννῡ", "ἐ~~ᾰννῠτην", "ἐ~~ᾰννῠσᾰν";
    imperfect_mid_1: "ἐ~~ᾰννῠμην", _, "ἐ~~ᾰννῠμεθᾰ";
    imperfect_mid_2: "ἐ~~ᾰννῠσο", "ἐ~~ᾰννῠσθον", "ἐ~~ᾰννῠσθε";
    imperfect_mid_3: "ἐ~~ᾰννῠτο", "ἐ~~ᾰννῠσθην", "ἐ~~ᾰννῠντο";
    imperfect_pass_1: _, _, "ἐ~~ᾰννῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~~ᾰννῠσθε";
    imperfect_pass_3: "ἐ~~ᾰννῠτο", _, _;
    future_ind_act_1: "~~ασω|~~εω|~~ω", _, "~~ασομεν|~~εομεν|~~οῡμεν";
    future_ind_act_2: "~~ασεις|~~εεις|~~εῑς", "~~ασετον|~~εετον|~~εῑτον", "~~ασετε|~~εετε|~~εῑτε";
    future_ind_act_3: "~~ασει|~~εει|~~εῑ", "~~ασετον|~~εετον|~~εῑτον", "~~ασουσῐ|~~ασουσῐν|~~εουσῐ|~~εουσῐν|~~οῡσῐ|~~οῡσῐν";
    future_ind_mid_1: "~~ασομαι|~~εομαι|~~οῡμαι", _, "~~ασομεθᾰ|~~εομεθᾰ|~~ουμεθᾰ";
    future_ind_mid_2: "~~ασῃ|~~ασει|~~εῃ|~~εει|~~εῑ|~~ῃ", "~~ασεσθον|~~εεσθον|~~εῑσθον", "~~ασεσθε|~~εεσθε|~~εῑσθε";
    future_ind_mid_3: "~~ασεται|~~εεται|~~εῑται", "~~ασεσθον|~~εεσθον|~~εῑσθον", "~~ασονται|~~εονται|~~οῡνται";
    future_ind_pass_1: "~~ᾰσθησομαι", _, "~~ᾰσθησομεθᾰ";
    future_ind_pass_2: "~~ᾰσθησῃ|~~ᾰσθησει", "~~ᾰσθησεσθον", "~~ᾰσθησεσθε";
    future_ind_pass_3: "~~ᾰσθησεται", _, "~~ᾰσθησονται";
    future_opt_act_1: "~~ασοιμῐ|~~εοιμῐ|~~οιην|~~οῑμῐ", _, "~~ασοιμεν|~~εοιμεν|~~οῑμεν|~~οιημεν";
    future_opt_act_2: "~~ασοις|~~εοις|~~οιης|~~οῑς", "~~ασοιτον|~~εοιτον|~~οῑτον|~~οιητον", "~~ασοιτε|~~εοιτε|~~οῑτε|~~οιητε";
    future_opt_act_3: "~~ασοι|~~εοι|~~οιη|~~οῑ", "~~ασοιτην|~~εοιτην|~~οιτην|~~οιητην", "~~ασοιεν|~~εοιεν|~~οῑεν|~~οιησᾰν";
    future_opt_mid_1: "~~ασοιμην|~~εοιμην|~~οιμην", _, "~~ασοιμεθᾰ|~~εοιμεθᾰ|~~οιμεθᾰ";
    future_opt_mid_2: "~~ασοιο|~~εοιο|~~οῑο", "~~ασοισθον|~~εοισθον|~~οῑσθον", "~~ασοισθε|~~εοισθε|~~οῑσθε";
    future_opt_mid_3: "~~ασοιτο|~~εοιτο|~~οῑτο", "~~ασοισθην|~~εοισθην|~~οισθην", "~~ασοιντο|~~εοιντο|~~οῑντο";
    future_opt_pass_1: "~~ᾰσθησοιμην", _, "~~ᾰσθησοιμεθᾰ";
    future_opt_pass_2: "~~ᾰσθησοιο", "~~ᾰσθησοισθον", "~~ᾰσθησοισθε";
    future_opt_pass_3: "~~ᾰσθησοιτο", "~~ᾰσθησοισθην", "~~ᾰσθησοιντο";
    aorist_ind_act_1: "ἐ~~ᾰσᾰ", _, "ἐ~~ᾰσᾰμεν";
    aorist_ind_act_2: "ἐ~~ᾰσᾰς", "ἐ~~ᾰσᾰτον", "ἐ~~ᾰσᾰτε";
    aorist_ind_act_3: "ἐ~~ᾰσε|ἐ~~ᾰσεν", "ἐ~~ᾰσᾰτην", "ἐ~~ᾰσᾰν";
    aorist_ind_mid_1: "ἐ~~ᾰσᾰμην", _, "ἐ~~ᾰσᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~~ᾰσω", "ἐ~~ᾰσᾰσθον", "ἐ~~ᾰσᾰσθε";
    aorist_ind_mid_3: "ἐ~~ᾰσᾰτο", "ἐ~~ᾰσᾰσθην", "ἐ~~ᾰσᾰντο";
    aorist_ind_pass_1: "ἐ~~ᾰσθην", _, "ἐ~~ᾰσθημεν";
    aorist_ind_pass_2: "ἐ~~ᾰσθης", "ἐ~~ᾰσθητον", "ἐ~~ᾰσθητε";
    aorist_ind_pass_3: "ἐ~~ᾰσθη", "ἐ~~ᾰσθητην", "ἐ~~ᾰσθησᾰν";
    aorist_subj_act_1: "~~ᾰσω", _, "~~ᾰσωμεν";
    aorist_subj_act_2: "~~ᾰσῃς", "~~ᾰσητον", "~~ᾰσητε";
    aorist_subj_act_3: "~~ᾰσῃ", _, "~~ᾰσωσῐ|~~ᾰσωσῐν";
    aorist_subj_mid_1: "~~ᾰσωμαι", _, "~~ᾰσωμεθᾰ";
    aorist_subj_mid_2: "~~ᾰσῃ", "~~ᾰσησθον", "~~ᾰσησθε";
    aorist_subj_mid_3: "~~ᾰσηται", _, "~~ᾰσωνται";
    aorist_subj_pass_1: "~~ᾰσθω", _, "~~ᾰσθωμεν";
    aorist_subj_pass_2: "~~ᾰσθῃς", "~~ᾰσθητον", "~~ᾰσθητε";
    aorist_subj_pass_3: "~~ᾰσθῃ", _, "~~ᾰσθωσῐ|~~ᾰσθωσῐν";
    aorist_opt_act_1: "~~ᾰσαιμῐ", _, "~~ᾰσαιμεν";
    aorist_opt_act_2: "~~ᾰσειᾰς|~~ᾰσαις", "~~ᾰσαιτον", "~~ᾰσαιτε";
    aorist_opt_act_3: "~~ᾰσειε|~~ᾰσειεν|~~ᾰσαι", "~~ᾰσαιτην", "~~ᾰσειᾰν|~~ᾰσαιεν";
    aorist_opt_mid_1: "~~ᾰσαιμην", _, "~~ᾰσαιμεθᾰ";
    aorist_opt_mid_2: "~~ᾰσαιο", "~~ᾰσαισθον", "~~ᾰσαισθε";
    aorist_opt_mid_3: "~~ᾰσαιτο", "~~ᾰσαισθην", "~~ᾰσαιντο";
    aorist_opt_pass_1: "~~ᾰσθειην", _, "~~ᾰσθεῑμεν|~~ᾰσθειημεν";
    aorist_opt_pass_2: "~~ᾰσθειης", "~~ᾰσθεῑτον|~~ᾰσθειητον", "~~ᾰσθεῑτε|~~ᾰσθειητε";
    aorist_opt_pass_3: "~~ᾰσθειη", "~~ᾰσθειτην|~~ᾰσθειητην", "~~ᾰσθεῑεν|~~ᾰσθειησᾰν";
    aorist_imp_act_2: "~~ᾰσον", "~~ᾰσᾰτον", "~~ᾰσᾰτε";
    aorist_imp_act_3: "~~ᾰσᾰτω", "~~ᾰσᾰτων", "~~ᾰσᾰντων";
    aorist_imp_mid_2: "~~ᾰσαι", "~~ᾰσᾰσθον", "~~ᾰσᾰσθε";
    aorist_imp_mid_3: "~~ᾰσᾰσθω", "~~ᾰσᾰσθων", _;
    aorist_imp_pass_2: "~~ᾰσθητῐ", _, _;
    aorist_imp_pass_3: "~~ᾰσθητω", "~~ᾰσθητων", "~~ᾰσθεντων";
    perfect_ind_act_1: "%ε~~ᾰκᾰ", _, "%ε~~ᾰκᾰμεν";
    perfect_ind_act_2: "%ε~~ᾰκᾰς", "%ε~~ᾰκᾰτον", "%ε~~ᾰκᾰτε";
    perfect_ind_act_3: "%ε~~ᾰκε|%ε~~ᾰκεν", _, "%ε~~ᾰκᾱσῐ|%ε~~ᾰκᾱσῐν";
    perfect_ind_mid_1: "~π~ᾰμαι", _, "~π~ᾰμεθᾰ";
    perfect_ind_mid_2: "~π~ᾰσαι", "~π~ᾰσθον", "~π~ᾰσθε";
    perfect_ind_mid_3: "~π~ᾰται", _, "~π~ᾰνται";
    perfect_ind_pass_1: _, _, "~π~ᾰμεθᾰ";
    perfect_ind_pass_2: _, _, "~π~ᾰσθε";
    perfect_subj_act_1: "%ε~~ᾰκω", _, "%ε~~ᾰκωμεν";
    perfect_subj_act_2: "%ε~~ᾰκῃς", "%ε~~ᾰκητον", "%ε~~ᾰκητε";
    perfect_subj_act_3: "%ε~~ᾰκῃ", _, "%ε~~ᾰκωσῐ|%ε~~ᾰκωσῐν";
    perfect_opt_act_1: "%ε~~ᾰκοιμῐ|%ε~~ᾰκοιην", _, "%ε~~ᾰκοιμεν";
    perfect_opt_act_2: "%ε~~ᾰκοις|%ε~~ᾰκοιης", "%ε~~ᾰκοιτον", "%ε~~ᾰκοιτε";
    perfect_opt_act_3: "%ε~~ᾰκοι|%ε~~ᾰκοιη", "%ε~~ᾰκοιτην", "%ε~~ᾰκοιεν";
    perfect_imp_act_2: "%ε~~ᾰκε", "%ε~~ᾰκετον", "%ε~~ᾰκετε";
    perfect_imp_act_3: "%ε~~ᾰκετω", "%ε~~ᾰκετων", "%ε~~ᾰκοντων";
    perfect_imp_mid_2: "~π~ᾰσο", _, _;
    perfect_imp_mid_3: "~π~ᾰσθω", "~π~ᾰσθων", _;
    perfect_imp_pass_2: "~π~ᾰσο", _, _;
    perfect_imp_pass_3: "~π~ᾰσθω", "~π~ᾰσθων", _;
    pluperfect_act_1: "ἐ%ε~~ᾰκειν|ἐ%ε~~ᾰκη", _, "ἐ%ε~~ᾰκεμεν";
    pluperfect_act_2: "ἐ%ε~~ᾰκεις|ἐ%ε~~ᾰκης", "ἐ%ε~~ᾰκετον", "ἐ%ε~~ᾰκετε";
    pluperfect_act_3: "ἐ%ε~~ᾰκει|ἐ%ε~~ᾰκειν", "ἐ%ε~~ᾰκετην", "ἐ%ε~~ᾰκεσᾰν";
    pluperfect_mid_1: "ἐ~π~ᾰμην", _, "ἐ~π~ᾰμεθᾰ";
    pluperfect_mid_2: "ἐ~π~ᾰσο", "ἐ~π~ᾰσθον", "ἐ~π~ᾰσθε";
    pluperfect_mid_3: "ἐ~π~ᾰτο", "ἐ~π~ᾰσθην", _;
    pluperfect_pass_1: _, _, "ἐ~π~ᾰμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~π~ᾰσθε";
    pluperfect_pass_3: _, "ἐ~π~ᾰσθην", "ἐ~π~ᾰντο";
    infinitive_pres: "~~ᾰννῠναι", "~~ᾰννῠσθαι", _;
    infinitive_fut: "~~ασειν|~~εειν|~~εῑν", "~~ασεσθαι|~~εεσθαι|~~εῑσθαι", "~~ᾰσθησεσθαι";
    infinitive_aor: "~~ᾰσαι", "~~ᾰσᾰσθαι", "~~ᾰσθηναι";
    infinitive_perf: "%ε~~ᾰκεναι", "~π~ᾰσθαι", _;
    participle_pres: "~~ᾰννῡς", "~~ᾰννῠμενος", _;
    participle_fut: "~~ασων|~~εων|~~ων", "~~ασομενος|~~εομενος|~~ουμενος", "~~ᾰσθησομενος";
    participle_aor: "~~ᾰσᾱς", "~~ᾰσᾰμενος", "~~ᾰσθεις";
    participle_perf: "%ε~~ᾰκως", "~π~ᾰμενος", _;
    ),
    // Ἐρᾰμαι: 1 words (ἐρᾰμαι). Like Μᾰρνᾰμαι, except in 121 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἐρᾰμαι", 1, "ἐ~ᾰμαι", like Μᾰρνᾰμαι;
    ind_act_1: "ἐ~ᾰμαι", _, _;
    ind_mid_1: "ἐ~ᾰμαι", _, "ἐ~ᾰμεθᾰ";
    ind_mid_2: "ἐ~ᾰσαι", "ἐ~ᾰσθον", "ἐ~ᾰσθε";
    ind_mid_3: "ἐ~ᾰται", _, "ἐ~ᾰνται";
    ind_pass_1: _, _, "ἐ~ᾰμεθᾰ";
    ind_pass_2: _, _, "ἐ~ᾰσθε";
    ind_pass_3: "ἐ~ᾰται", _, "ἐ~ᾰνται";
    subj_mid_1: "ἐ~ωμαι", _, "ἐ~ωμεθᾰ";
    subj_mid_2: "ἐ~ῃ", "ἐ~ησθον", "ἐ~ησθε";
    subj_mid_3: "ἐ~ηται", _, "ἐ~ωνται";
    subj_pass_1: _, _, "ἐ~ωμεθᾰ";
    subj_pass_2: _, _, "ἐ~ησθε";
    subj_pass_3: "ἐ~ηται", _, "ἐ~ωνται";
    opt_mid_1: "ἐ~αιμην", _, "ἐ~αιμεθᾰ";
    opt_mid_2: "ἐ~αιο", "ἐ~αισθον", "ἐ~αισθε";
    opt_mid_3: "ἐ~αιτο", "ἐ~αισθην", "ἐ~αιντο";
    opt_pass_1: _, _, "ἐ~αιμεθᾰ";
    opt_pass_2: _, "ἐ~αισθον", "ἐ~αισθε";
    opt_pass_3: "ἐ~αιτο", "ἐ~αισθην", "ἐ~αιντο";
    imp_mid_2: "ἐ~ᾰσο|ἐ~ω", _, "ἐ~ᾰσθε";
    imp_mid_3: "ἐ~ᾰσθω", "ἐ~ᾰσθων", "ἐ~ᾰσθων";
    imp_pass_2: "ἐ~ᾰσο|ἐ~ω", _, "ἐ~ᾰσθε";
    imp_pass_3: _, "ἐ~ᾰσθων", _;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, derived;
    imperfect_mid_3: derived, derived, derived;
    imperfect_pass_1: _, _, derived;
    imperfect_pass_2: _, _, derived;
    imperfect_pass_3: derived, _, _;
    future_ind_pass_1: "ἐ~ᾰσθησομαι", _, "ἐ~ᾰσθησομεθᾰ";
    future_ind_pass_2: "ἐ~ᾰσθησῃ|ἐ~ᾰσθησει", "ἐ~ᾰσθησεσθον", "ἐ~ᾰσθησεσθε";
    future_ind_pass_3: "ἐ~ᾰσθησεται", _, "ἐ~ᾰσθησονται";
    future_opt_pass_1: "ἐ~ᾰσθησοιμην", _, "ἐ~ᾰσθησοιμεθᾰ";
    future_opt_pass_2: "ἐ~ᾰσθησοιο", "ἐ~ᾰσθησοισθον", "ἐ~ᾰσθησοισθε";
    future_opt_pass_3: "ἐ~ᾰσθησοιτο", "ἐ~ᾰσθησοισθην", "ἐ~ᾰσθησοιντο";
    aorist_ind_pass_1: "ἠ~ᾰσθην", _, "ἠ~ᾰσθημεν";
    aorist_ind_pass_2: "ἠ~ᾰσθης", "ἠ~ᾰσθητον", "ἠ~ᾰσθητε";
    aorist_ind_pass_3: "ἠ~ᾰσθη", "ἠ~ᾰσθητην", "ἠ~ᾰσθησᾰν";
    aorist_subj_pass_1: "ἐ~ᾰσθω", _, "ἐ~ᾰσθωμεν";
    aorist_subj_pass_2: "ἐ~ᾰσθῃς", "ἐ~ᾰσθητον", "ἐ~ᾰσθητε";
    aorist_subj_pass_3: "ἐ~ᾰσθῃ", _, "ἐ~ᾰσθωσῐ|ἐ~ᾰσθωσῐν";
    aorist_opt_pass_1: "ἐ~ᾰσθειην", _, "ἐ~ᾰσθεῑμεν|ἐ~ᾰσθειημεν";
    aorist_opt_pass_2: "ἐ~ᾰσθειης", "ἐ~ᾰσθεῑτον|ἐ~ᾰσθειητον", "ἐ~ᾰσθεῑτε|ἐ~ᾰσθειητε";
    aorist_opt_pass_3: "ἐ~ᾰσθειη", "ἐ~ᾰσθειτην|ἐ~ᾰσθειητην", "ἐ~ᾰσθεῑεν|ἐ~ᾰσθειησᾰν";
    aorist_imp_pass_2: "ἐ~ᾰσθητῐ", _, _;
    aorist_imp_pass_3: "ἐ~ᾰσθητω", "ἐ~ᾰσθητων", "ἐ~ᾰσθεντων";
    perfect_ind_mid_1: "ἠ~ᾰσμαι", _, "ἠ~ᾰσμεθᾰ";
    perfect_ind_mid_2: "ἠ~ᾰσαι", "ἠ~ᾰσθον", "ἠ~ᾰσθε";
    perfect_ind_mid_3: "ἠ~ᾰσται", _, "ἠ~ᾰσᾰται";
    perfect_ind_pass_1: _, _, "ἠ~ᾰσμεθᾰ";
    perfect_ind_pass_2: _, _, "ἠ~ᾰσθε";
    perfect_imp_mid_2: "ἠ~ᾰσο", _, _;
    perfect_imp_mid_3: "ἠ~ᾰσθω", "ἠ~ᾰσθων", _;
    perfect_imp_pass_2: "ἠ~ᾰσο", _, _;
    perfect_imp_pass_3: "ἠ~ᾰσθω", "ἠ~ᾰσθων", _;
    infinitive_pres: _, "ἐ~ᾰσθαι", _;
    infinitive_fut: _, _, "ἐ~ᾰσθησεσθαι";
    infinitive_aor: _, _, "ἐ~ᾰσθηναι";
    infinitive_perf: _, "ἠ~ᾰσθαι", _;
    participle_pres: _, "ἐ~ᾰμενος", _;
    participle_fut: _, _, "ἐ~ᾰσθησομενος";
    participle_aor: _, _, "ἐ~ᾰσθεις";
    participle_perf: _, "ἠ~ᾰσμενος", _;
    ),
    // Σβεννῡμῐ: 1 words (σβεννῡμῐ). Like Δῐκεω, except in 275 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("σβεννῡμῐ", 1, "~εννῡμῐ", like Δῐκεω;
    ind_act_1: "~εννῡμῐ|~εννυω", _, "~εννῠμεν|~εννυομεν";
    ind_act_2: "~εννῡς|~εννυεις", "~εννῠτον|~εννυετον", "~εννῠτε|~εννυετε";
    ind_act_3: "~εννῡσῐ|~εννῡσῐν|~εννυει", "~εννῠτον|~εννυετον", "~εννῠᾱσῐ|~εννῠᾱσῐν|~εννυουσῐ|~εννυουσῐν";
    ind_mid_1: "~εννῠμαι|~εννυομαι", _, "~εννῠμεθᾰ|~εννυομεθᾰ";
    ind_mid_2: "~εννῠσαι|~εννυῃ|~εννυει", "~εννῠσθον|~εννυεσθον", "~εννῠσθε|~εννυεσθε";
    ind_mid_3: "~εννῠται|~εννυεται", "~εννῠσθον|~εννυεσθον", "~εννῠνται|~εννυονται";
    ind_pass_1: "~εννῠμαι|~εννυομαι", _, "~εννῠμεθᾰ|~εννυομεθᾰ";
    ind_pass_2: "~εννῠσαι|~εννυῃ|~εννυει", "~εννῠσθον|~εννυεσθον", "~εννῠσθε|~εννυεσθε";
    ind_pass_3: "~εννῠται|~εννυεται", "~εννῠσθον|~εννυεσθον", "~εννῠνται|~εννυονται";
    subj_act_1: "~εννῠω|~εννυω", _, "~εννῠωμεν|~εννυωμεν";
    subj_act_2: "~εννῠῃς|~εννυῃς", "~εννῠητον|~εννυητον", "~εννῠητε|~εννυητε";
    subj_act_3: "~εννῠῃ|~εννυῃ", "~εννῠητον|~εννυητον", "~εννῠωσῐ|~εννῠωσῐν|~εννυωσῐ|~εννυωσῐν";
    subj_mid_1: "~εννῠωμαι|~εννυωμαι", _, "~εννῠωμεθᾰ|~εννυωμεθᾰ";
    subj_mid_2: "~εννῠῃ|~εννυῃ", "~εννῠησθον|~εννυησθον", "~εννῠησθε|~εννυησθε";
    subj_mid_3: "~εννῠηται|~εννυηται", "~εννῠησθον|~εννυησθον", "~εννῠωνται|~εννυωνται";
    subj_pass_1: "~εννῠωμαι|~εννυωμαι", _, "~εννῠωμεθᾰ|~εννυωμεθᾰ";
    subj_pass_2: "~εννῠῃ|~εννυῃ", "~εννῠησθον|~εννυησθον", "~εννῠησθε|~εννυησθε";
    subj_pass_3: "~εννῠηται|~εννυηται", "~εννῠησθον|~εννυησθον", "~εννῠωνται|~εννυωνται";
    opt_act_1: "~εννῠοιμῐ|~εννυοιμῐ", _, "~εννῠοιμεν|~εννυοιμεν";
    opt_act_2: "~εννῠοις|~εννυοις", "~εννῠοιτον|~εννυοιτον", "~εννῠοιτε|~εννυοιτε";
    opt_act_3: "~εννῠοι|~εννυοι", "~εννῠοιτην|~εννυοιτην", "~εννῠοιεν|~εννυοιεν";
    opt_mid_1: "~εννῠοιμην|~εννυοιμην", _, "~εννῠοιμεθᾰ|~εννυοιμεθᾰ";
    opt_mid_2: "~εννῠοιο|~εννυοιο", "~εννῠοισθον|~εννυοισθον", "~εννῠοισθε|~εννυοισθε";
    opt_mid_3: "~εννῠοιτο|~εννυοιτο", "~εννῠοισθην|~εννυοισθην", "~εννῠοιντο|~εννυοιντο";
    opt_pass_1: "~εννῠοιμην|~εννυοιμην", _, "~εννῠοιμεθᾰ|~εννυοιμεθᾰ";
    opt_pass_2: "~εννῠοιο|~εννυοιο", "~εννῠοισθον|~εννυοισθον", "~εννῠοισθε|~εννυοισθε";
    opt_pass_3: "~εννῠοιτο|~εννυοιτο", "~εννῠοισθην|~εννυοισθην", "~εννῠοιντο|~εννυοιντο";
    imp_act_2: "~εννῡ|~εννυε", "~εννῠτον|~εννυετον", "~εννῠτε|~εννυετε";
    imp_act_3: "~εννῠτω|~εννυετω", "~εννῠτων|~εννυετων", "~εννῠντων|~εννυοντων";
    imp_mid_2: "~εννῠσο|~εννυου", "~εννῠσθον|~εννυεσθον", "~εννῠσθε|~εννυεσθε";
    imp_mid_3: "~εννῠσθω|~εννυεσθω", "~εννῠσθων|~εννυεσθων", "~εννῠσθων|~εννυεσθων";
    imp_pass_2: "~εννῠσο|~εννυου", "~εννῠσθον|~εννυεσθον", "~εννῠσθε|~εννυεσθε";
    imp_pass_3: "~εννῠσθω|~εννυεσθω", "~εννῠσθων|~εννυεσθων", "~εννῠσθων|~εννυεσθων";
    imperfect_act_1: "ἐ~εννῡν|ἐ~εννυον", _, "ἐ~εννῠμεν|ἐ~εννυομεν";
    imperfect_act_2: "ἐ~εννῡς|ἐ~εννυες", "ἐ~εννῠτον|ἐ~εννυετον", "ἐ~εννῠτε|ἐ~εννυετε";
    imperfect_act_3: "ἐ~εννῡ|ἐ~εννυε|ἐ~εννυεν", "ἐ~εννῠτην|ἐ~εννυετην", "ἐ~εννῠσᾰν|ἐ~εννυον";
    imperfect_mid_1: "ἐ~εννῠμην|ἐ~εννυομην", _, "ἐ~εννῠμεθᾰ|ἐ~εννυομεθᾰ";
    imperfect_mid_2: "ἐ~εννῠσο|ἐ~εννυου", "ἐ~εννῠσθον|ἐ~εννυεσθον", "ἐ~εννῠσθε|ἐ~εννυεσθε";
    imperfect_mid_3: "ἐ~εννῠτο|ἐ~εννυετο", "ἐ~εννῠσθην|ἐ~εννυεσθην", "ἐ~εννῠντο|ἐ~εννυοντο";
    imperfect_pass_1: "ἐ~εννῠμην|ἐ~εννυομην", _, "ἐ~εννῠμεθᾰ|ἐ~εννυομεθᾰ";
    imperfect_pass_2: "ἐ~εννῠσο|ἐ~εννυου", "ἐ~εννῠσθον|ἐ~εννυεσθον", "ἐ~εννῠσθε|ἐ~εννυεσθε";
    imperfect_pass_3: "ἐ~εννῠτο|ἐ~εννυετο", "ἐ~εννῠσθην|ἐ~εννυεσθην", "ἐ~εννῠντο|ἐ~εννυοντο";
    future_ind_act_1: "~εσω", _, "~εσομεν";
    future_ind_act_2: "~εσεις", "~εσετον", "~εσετε";
    future_ind_act_3: "~εσει", _, "~εσουσῐ|~εσουσῐν";
    future_ind_pass_1: "~εσθησομαι", _, "~εσθησομεθᾰ";
    future_ind_pass_2: "~εσθησῃ|~εσθησει", "~εσθησεσθον", "~εσθησεσθε";
    future_ind_pass_3: "~εσθησεται", _, "~εσθησονται";
    future_opt_act_1: "~εσοιμῐ", _, "~εσοιμεν";
    future_opt_act_2: "~εσοις", "~εσοιτον", "~εσοιτε";
    future_opt_act_3: "~εσοι", "~εσοιτην", "~εσοιεν";
    future_opt_pass_1: "~εσθησοιμην", _, "~εσθησοιμεθᾰ";
    future_opt_pass_2: "~εσθησοιο", "~εσθησοισθον", "~εσθησοισθε";
    future_opt_pass_3: "~εσθησοιτο", "~εσθησοισθην", "~εσθησοιντο";
    aorist_ind_act_1: "ἐ~εσᾰ", _, "ἐ~εσᾰμεν";
    aorist_ind_act_2: "ἐ~εσᾰς", "ἐ~εσᾰτον", "ἐ~εσᾰτε";
    aorist_ind_act_3: "ἐ~εσε|ἐ~εσεν", "ἐ~εσᾰτην", "ἐ~εσᾰν";
    aorist_ind_mid_1: "ἐ~εσᾰμην", _, "ἐ~εσᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~εσω", "ἐ~εσᾰσθον", "ἐ~εσᾰσθε";
    aorist_ind_mid_3: "ἐ~εσᾰτο", "ἐ~εσᾰσθην", "ἐ~εσᾰντο";
    aorist_ind_pass_1: "ἐ~εσθην|ἐ~ην", _, "ἐ~εσθημεν|ἐ~ημεν";
    aorist_ind_pass_2: "ἐ~εσθης|ἐ~ης", "ἐ~εσθητον|ἐ~ητον", "ἐ~εσθητε|ἐ~ητε";
    aorist_ind_pass_3: "ἐ~εσθη|ἐ~η", "ἐ~εσθητην|ἐ~ητην", "ἐ~εσθησᾰν|ἐ~ησᾰν";
    aorist_subj_act_1: "~εσω", _, "~εσωμεν";
    aorist_subj_act_2: "~εσῃς", "~εσητον", "~εσητε";
    aorist_subj_act_3: "~εσῃ", _, "~εσωσῐ|~εσωσῐν";
    aorist_subj_mid_1: "~εσωμαι", _, "~εσωμεθᾰ";
    aorist_subj_mid_2: "~εσῃ", "~εσησθον", "~εσησθε";
    aorist_subj_mid_3: "~εσηται", _, "~εσωνται";
    aorist_subj_pass_1: "~εσθω|~ω", _, "~εσθωμεν|~ωμεν";
    aorist_subj_pass_2: "~εσθῃς|~ῃς", "~εσθητον|~ητον", "~εσθητε|~ητε";
    aorist_subj_pass_3: "~εσθῃ|~ῃ", "~εσθητον|~ητον", "~εσθωσῐ|~εσθωσῐν|~ωσῐ|~ωσῐν";
    aorist_opt_act_1: "~εσαιμῐ", _, "~εσαιμεν";
    aorist_opt_act_2: "~εσειᾰς|~εσαις", "~εσαιτον", "~εσαιτε";
    aorist_opt_act_3: "~εσειε|~εσειεν|~εσαι", "~εσαιτην", "~εσειᾰν|~εσαιεν";
    aorist_opt_mid_1: "~εσαιμην", _, "~εσαιμεθᾰ";
    aorist_opt_mid_2: "~εσαιο", "~εσαισθον", "~εσαισθε";
    aorist_opt_mid_3: "~εσαιτο", "~εσαισθην", "~εσαιντο";
    aorist_opt_pass_1: "~εσθειην|~ειην", _, "~εσθεῑμεν|~εσθειημεν|~εῑμεν|~ειημεν";
    aorist_opt_pass_2: "~εσθειης|~ειης", "~εσθεῑτον|~εσθειητον|~εῑτον|~ειητον", "~εσθεῑτε|~εσθειητε|~εῑτε|~ειητε";
    aorist_opt_pass_3: "~εσθειη|~ειη", "~εσθειτην|~εσθειητην|~ειτην|~ειητην", "~εσθεῑεν|~εσθειησᾰν|~εῑεν|~ειησᾰν";
    aorist_imp_act_2: "~εσον", "~εσᾰτον", "~εσᾰτε";
    aorist_imp_act_3: "~εσᾰτω", "~εσᾰτων", "~εσᾰντων";
    aorist_imp_mid_2: "~εσαι", "~εσᾰσθον", "~εσᾰσθε";
    aorist_imp_mid_3: "~εσᾰσθω", "~εσᾰσθων", _;
    aorist_imp_pass_2: "~εσθητῐ|~ηθῐ", "~εσθητον|~ητον", "~εσθητε|~ητε";
    aorist_imp_pass_3: "~εσθητω|~ητω", "~εσθητων|~ητων", "~εσθεντων|~εντων";
    perfect_ind_mid_1: "ἐ~εσμαι", _, "ἐ~εσμεθᾰ";
    perfect_ind_mid_2: "ἐ~εσαι", "ἐ~εσθον", "ἐ~εσθε";
    perfect_ind_mid_3: "ἐ~εσται", _, "ἐ~εσᾰται";
    perfect_ind_pass_1: _, _, "ἐ~εσμεθᾰ";
    perfect_ind_pass_2: _, _, "ἐ~εσθε";
    perfect_imp_mid_2: "ἐ~εσο", _, _;
    perfect_imp_mid_3: "ἐ~εσθω", "ἐ~εσθων", _;
    perfect_imp_pass_2: "ἐ~εσο", _, _;
    perfect_imp_pass_3: "ἐ~εσθω", "ἐ~εσθων", _;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
    infinitive_pres: "~εννῠναι|~εννυειν", "~εννῠσθαι|~εννυεσθαι", "~εννῠσθαι|~εννυεσθαι";
    infinitive_fut: "~εσειν", _, "~εσθησεσθαι";
    infinitive_aor: "~εσαι", "~εσᾰσθαι", "~εσθηναι|~ηναι";
    infinitive_perf: _, "ἐ~εσθαι", _;
    participle_pres: "~εννῡς|~εννυων", "~εννῠμενος|~εννυομενος", "~εννῠμενος|~εννυομενος";
    participle_fut: "~εσων", _, "~εσθησομενος";
    participle_aor: "~εσᾱς", "~εσᾰμενος", "~εσθεις|~εις";
    participle_perf: _, "ἐ~εσμενος", _;
    ),
    // Φημῐ: 1 words (φημῐ). Like Δαμνημῐ, except in 103 cells: ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, opt_mid_2 sg, opt_mid_2 du, opt_mid_2 pl, ….
    conjugation!("φημῐ", 1, "~ημῐ", like Δαμνημῐ;
    ind_act_2: "~ῃς", _, _;
    ind_act_3: derived, _, derived;
    opt_mid_2: "~αῑο", "~αῑσθον", "~αῑσθε";
    opt_mid_3: "~αῑτο", _, "~αῑντο";
    opt_pass_2: _, "~αῑσθον", "~αῑσθε";
    opt_pass_3: "~αῑτο", _, "~αῑντο";
    imp_act_2: "~ᾰθῐ|~ᾰθῐ", _, _;
    imp_mid_2: "~ᾰσο", _, _;
    imp_pass_2: derived, _, _;
    imperfect_act_2: "ἐ~ης|ἐ~ησθᾰ", _, _;
    imperfect_act_3: "ἐ~η|ἐ~ην", _, "ἐ~ᾰσᾰν|ἐ~ᾰν";
    imperfect_pass_1: _, _, derived;
    imperfect_pass_2: _, _, derived;
    imperfect_pass_3: derived, _, _;
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", "~ησετε";
    future_ind_act_3: "~ησει", _, "~ησουσῐ|~ησουσῐν";
    future_ind_mid_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_mid_2: "~ησῃ|~ησει", "~ησεσθον", "~ησεσθε";
    future_ind_mid_3: "~ησεται", _, "~ησονται";
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: "~ησοις", "~ησοιτον", "~ησοιτε";
    future_opt_act_3: "~ησοι", "~ησοιτην", "~ησοιεν";
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_2: "~ησοιο", "~ησοισθον", "~ησοισθε";
    future_opt_mid_3: "~ησοιτο", "~ησοισθην", "~ησοιντο";
    aorist_ind_act_1: "ἐ~ησᾰ", _, "ἐ~ησᾰμεν";
    aorist_ind_act_2: "ἐ~ησᾰς", "ἐ~ησᾰτον", "ἐ~ησᾰτε";
    aorist_ind_act_3: "ἐ~ησε|ἐ~ησεν", "ἐ~ησᾰτην", "ἐ~ησᾰν";
    aorist_subj_act_1: "~ησω", _, "~ησωμεν";
    aorist_subj_act_2: "~ησῃς", "~ησητον", "~ησητε";
    aorist_subj_act_3: "~ησῃ", _, "~ησωσῐ|~ησωσῐν";
    aorist_opt_act_1: "~ησαιμῐ", _, "~ησαιμεν";
    aorist_opt_act_2: "~ησειᾰς|~ησαις", "~ησαιτον", "~ησαιτε";
    aorist_opt_act_3: "~ησειε|~ησειεν|~ησαι", "~ησαιτην", "~ησειᾰν|~ησαιεν";
    aorist_imp_act_2: "~ησον", "~ησᾰτον", "~ησᾰτε";
    aorist_imp_act_3: "~ησᾰτω", "~ησᾰτων", "~ησᾰντων";
    perfect_ind_mid_1: "%ε~ᾰμαι", _, "%ε~ᾰμεθᾰ";
    perfect_ind_mid_2: "%ε~ᾰσαι", "%ε~ᾰσθον", "%ε~ᾰσθε";
    perfect_ind_mid_3: "%ε~ᾰται", _, "%ε~ᾰνται";
    perfect_ind_pass_1: _, _, "%ε~ᾰμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~ᾰσθε";
    perfect_imp_mid_2: "%ε~ᾰσο", _, _;
    perfect_imp_mid_3: "%ε~ᾰσθω", "%ε~ᾰσθων", _;
    perfect_imp_pass_2: "%ε~ᾰσο", _, _;
    perfect_imp_pass_3: "%ε~ᾰσθω", "%ε~ᾰσθων", _;
    infinitive_fut: "~ησειν", "~ησεσθαι", _;
    infinitive_aor: "~ησαι", _, _;
    infinitive_perf: _, "%ε~ᾰσθαι", _;
    participle_fut: "~ησων", "~ησομενος", _;
    participle_aor: "~ησᾱς", _, _;
    participle_perf: _, "%ε~ᾰσμενος", _;
    ),
    // Ἐοικᾰ: 1 words (ἐοικᾰ). Like Εἰμῐ, except in 128 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, subj_act_1 sg, subj_act_1 pl, subj_act_2 sg, ….
    conjugation!("ἐοικᾰ", 1, "ἐοικᾰ", like Εἰμῐ;
    ind_act_1: "ἐοικᾰ", _, derived;
    ind_act_2: derived, _, _;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imp_act_3: derived, derived, derived;
    imperfect_act_1: _, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    future_ind_act_1: "εἰξω", _, "εἰξομεν";
    future_ind_act_2: "εἰξεις", "εἰξετον", "εἰξετε";
    future_ind_act_3: "εἰξει", _, "εἰξουσῐ|εἰξουσῐν";
    future_ind_mid_1: "εἰξομαι", _, "εἰξομεθᾰ";
    future_ind_mid_2: "εἰξῃ|εἰξει", "εἰξεσθον", "εἰξεσθε";
    future_ind_mid_3: "εἰξεται", _, "εἰξονται";
    future_opt_act_1: "εἰξοιμῐ", _, "εἰξοιμεν";
    future_opt_act_2: "εἰξοις", "εἰξοιτον", "εἰξοιτε";
    future_opt_act_3: "εἰξοι", "εἰξοιτην", "εἰξοιεν";
    future_opt_mid_1: "εἰξοιμην", _, "εἰξοιμεθᾰ";
    future_opt_mid_2: "εἰξοιο", "εἰξοισθον", "εἰξοισθε";
    future_opt_mid_3: "εἰξοιτο", "εἰξοισθην", "εἰξοιντο";
    aorist_ind_act_1: "εἰξᾰ", _, "εἰξᾰμεν";
    aorist_ind_act_2: "εἰξᾰς", "εἰξᾰτον", "εἰξᾰτε";
    aorist_ind_act_3: "εἰξε|εἰξεν", "εἰξᾰτην", "εἰξᾰν";
    aorist_ind_mid_1: "εἰξᾰμην", _, "εἰξᾰμεθᾰ";
    aorist_ind_mid_2: "εἰξω", "εἰξᾰσθον", "εἰξᾰσθε";
    aorist_ind_mid_3: "εἰξᾰτο", "εἰξᾰσθην", "εἰξᾰντο";
    aorist_subj_act_1: "εἰξω", _, "εἰξωμεν";
    aorist_subj_act_2: "εἰξῃς", "εἰξητον", "εἰξητε";
    aorist_subj_act_3: "εἰξῃ", _, "εἰξωσῐ|εἰξωσῐν";
    aorist_subj_mid_1: "εἰξωμαι", _, "εἰξωμεθᾰ";
    aorist_subj_mid_2: "εἰξῃ", "εἰξησθον", "εἰξησθε";
    aorist_subj_mid_3: "εἰξηται", _, "εἰξωνται";
    aorist_opt_act_1: "εἰξαιμῐ", _, "εἰξαιμεν";
    aorist_opt_act_2: "εἰξειᾰς|εἰξαις", "εἰξαιτον", "εἰξαιτε";
    aorist_opt_act_3: "εἰξειε|εἰξειεν|εἰξαι", "εἰξαιτην", "εἰξειᾰν|εἰξαιεν";
    aorist_opt_mid_1: "εἰξαιμην", _, "εἰξαιμεθᾰ";
    aorist_opt_mid_2: "εἰξαιο", "εἰξαισθον", "εἰξαισθε";
    aorist_opt_mid_3: "εἰξαιτο", "εἰξαισθην", "εἰξαιντο";
    aorist_imp_act_2: "εἰξον", "εἰξᾰτον", "εἰξᾰτε";
    aorist_imp_act_3: "εἰξᾰτω", "εἰξᾰτων", "εἰξᾰντων";
    aorist_imp_mid_2: "εἰξαι", "εἰξᾰσθον", "εἰξᾰσθε";
    aorist_imp_mid_3: "εἰξᾰσθω", "εἰξᾰσθων", _;
    infinitive_pres: derived, _, _;
    infinitive_fut: "εἰξειν", "εἰξεσθαι", _;
    infinitive_aor: "εἰξαι", "εἰξᾰσθαι", _;
    participle_pres: derived, _, _;
    participle_fut: "εἰξων", "εἰξομενος", _;
    participle_aor: "εἰξᾱς", "εἰξᾰμενος", _;
    ),
    // Κεῑμαι: 1 words (κεῑμαι). Like Μᾰρνᾰμαι, except in 71 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("κεῑμαι", 1, "~ῑμαι", like Μᾰρνᾰμαι;
    ind_act_1: "~ῑμαι", _, _;
    ind_mid_1: "~ῑμαι", _, "~ιμεθα";
    ind_mid_2: "~ῑσαι", "~ῑσθον", "~ῑσθε";
    ind_mid_3: "~ῑται", _, "~ῑνται|~ᾰται|~ονται|~ιᾰται";
    ind_pass_1: _, _, derived;
    ind_pass_2: _, _, derived;
    ind_pass_3: derived, _, derived;
    subj_mid_1: _, _, "~ωμεθα";
    subj_mid_3: "~ηται|~ῑται", _, "~ωνται|~ιωνται";
    subj_pass_1: _, _, derived;
    subj_pass_2: _, _, derived;
    subj_pass_3: derived, _, derived;
    opt_mid_1: "~οιμην", _, "~οιμεθα";
    opt_mid_2: "~οιο", derived, "~οισθε";
    opt_mid_3: "~οιτο", derived, "~οιντο";
    opt_pass_1: _, _, derived;
    opt_pass_2: _, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_mid_2: "~ῑσο", _, "~ῑσθε";
    imp_mid_3: "~ισθω", "~ισθων", "~ισθων";
    imp_pass_2: derived, _, derived;
    imp_pass_3: _, derived, _;
    imperfect_mid_1: "ἐ~ιμην", _, "ἐ~ιμεθα";
    imperfect_mid_2: "ἐ~ισο", "ἐ~ισθον", "ἐ~ισθε";
    imperfect_mid_3: "ἐ~ιτο", "ἐ~ισθην", "ἐ~ιντο|ἐ~ατο";
    imperfect_pass_1: _, _, derived;
    imperfect_pass_2: _, _, derived;
    imperfect_pass_3: derived, _, _;
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
    // Μῑγνῡμῐ: 1 words (μῑγνῡμῐ).
    conjugation!("μῑγνῡμῐ", 1, "~ῑγνῡμῐ";
    ind_act_1: "~ῑγνῡμῐ", _, "~ῑγνῠμεν";
    ind_act_2: "~ῑγνῡς", _, _;
    ind_act_3: "~ῑγνῡσῐ|~ῑγνῡσῐν", _, "~ῑγνῠᾱσῐ|~ῑγνῠᾱσῐν";
    ind_mid_1: "~ῑγνῠμαι", _, "~ῑγνῠμεθᾰ";
    ind_mid_2: "~ῑγνῠσαι", "~ῑγνῠσθον", "~ῑγνῠσθε";
    ind_mid_3: "~ῑγνῠται", _, "~ῑγνῠνται";
    ind_pass_1: _, _, "~ῑγνῠμεθᾰ";
    ind_pass_2: _, _, "~ῑγνῠσθε";
    ind_pass_3: "~ῑγνῠται", _, "~ῑγνῠνται";
    subj_act_1: "~ῑγνῠω", _, "~ῑγνῠωμεν";
    subj_act_2: "~ῑγνῠῃς", "~ῑγνῠητον", "~ῑγνῠητε";
    subj_act_3: "~ῑγνῠῃ", _, "~ῑγνῠωσῐ|~ῑγνῠωσῐν";
    subj_mid_1: "~ῑγνῠωμαι", _, "~ῑγνῠωμεθᾰ";
    subj_mid_2: "~ῑγνῠῃ", "~ῑγνῠησθον", "~ῑγνῠησθε";
    subj_mid_3: "~ῑγνῠηται", _, "~ῑγνῠωνται";
    subj_pass_1: _, _, "~ῑγνῠωμεθᾰ";
    subj_pass_2: _, _, "~ῑγνῠησθε";
    subj_pass_3: "~ῑγνῠηται", _, "~ῑγνῠωνται";
    opt_act_1: "~ῑγνῠοιμῐ", _, "~ῑγνῠοιμεν";
    opt_act_2: "~ῑγνῠοις", "~ῑγνῠοιτον", "~ῑγνῠοιτε";
    opt_act_3: "~ῑγνῠοι", "~ῑγνῠοιτην", "~ῑγνῠοιεν";
    opt_mid_1: "~ῑγνῠοιμην", _, "~ῑγνῠοιμεθᾰ";
    opt_mid_2: "~ῑγνῠοιο", "~ῑγνῠοισθον", "~ῑγνῠοισθε";
    opt_mid_3: "~ῑγνῠοιτο", "~ῑγνῠοισθην", "~ῑγνῠοιντο";
    opt_pass_1: _, _, "~ῑγνῠοιμεθᾰ";
    opt_pass_2: _, "~ῑγνῠοισθον", "~ῑγνῠοισθε";
    opt_pass_3: "~ῑγνῠοιτο", "~ῑγνῠοισθην", "~ῑγνῠοιντο";
    imp_act_2: "~ῑγνῡ", "~ῑγνῠτον", "~ῑγνῠτε";
    imp_act_3: "~ῑγνῠτω", "~ῑγνῠτων", "~ῑγνῠντων";
    imp_mid_2: "~ῑγνῠσο", _, "~ῑγνῠσθε";
    imp_mid_3: "~ῑγνῠσθω", "~ῑγνῠσθων", "~ῑγνῠσθων";
    imp_pass_2: _, _, "~ῑγνῠσθε";
    imp_pass_3: _, "~ῑγνῠσθων", _;
    imperfect_act_1: "ἐ~ῑγνῡν", _, "ἐ~ῑγνῠμεν";
    imperfect_act_2: "ἐ~ῑγνῡς", "ἐ~ῑγνῠτον", "ἐ~ῑγνῠτε";
    imperfect_act_3: "ἐ~ῑγνῡ", "ἐ~ῑγνῠτην", "ἐ~ῑγνῠσᾰν";
    imperfect_mid_1: "ἐ~ῑγνῠμην", _, "ἐ~ῑγνῠμεθᾰ";
    imperfect_mid_2: "ἐ~ῑγνῠσο", "ἐ~ῑγνῠσθον", "ἐ~ῑγνῠσθε";
    imperfect_mid_3: "ἐ~ῑγνῠτο", "ἐ~ῑγνῠσθην", "ἐ~ῑγνῠντο";
    imperfect_pass_1: _, _, "ἐ~ῑγνῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ῑγνῠσθε";
    imperfect_pass_3: "ἐ~ῑγνῠτο", _, _;
    future_ind_act_1: "~ῑξω", _, "~ῑξομεν";
    future_ind_act_2: "~ῑξεις", "~ῑξετον", "~ῑξετε";
    future_ind_act_3: "~ῑξει", _, "~ῑξουσῐ|~ῑξουσῐν";
    future_ind_mid_1: "~ῑξομαι", _, "~ῑξομεθᾰ";
    future_ind_mid_2: "~ῑξῃ|~ῑξει", "~ῑξεσθον", "~ῑξεσθε";
    future_ind_mid_3: "~ῑξεται", _, "~ῑξονται";
    future_ind_pass_1: "~ῑχθησομαι", _, "~ῑχθησομεθᾰ";
    future_ind_pass_2: "~ῑχθησῃ|~ῑχθησει", "~ῑχθησεσθον", "~ῑχθησεσθε";
    future_ind_pass_3: "~ῑχθησεται", _, "~ῑχθησονται";
    future_opt_act_1: "~ῑξοιμῐ", _, "~ῑξοιμεν";
    future_opt_act_2: "~ῑξοις", "~ῑξοιτον", "~ῑξοιτε";
    future_opt_act_3: "~ῑξοι", "~ῑξοιτην", "~ῑξοιεν";
    future_opt_mid_1: "~ῑξοιμην", _, "~ῑξοιμεθᾰ";
    future_opt_mid_2: "~ῑξοιο", "~ῑξοισθον", "~ῑξοισθε";
    future_opt_mid_3: "~ῑξοιτο", "~ῑξοισθην", "~ῑξοιντο";
    future_opt_pass_1: "~ῑχθησοιμην", _, "~ῑχθησοιμεθᾰ";
    future_opt_pass_2: "~ῑχθησοιο", "~ῑχθησοισθον", "~ῑχθησοισθε";
    future_opt_pass_3: "~ῑχθησοιτο", "~ῑχθησοισθην", "~ῑχθησοιντο";
    aorist_ind_act_1: "ἐ~ῑξᾰ", _, "ἐ~ῑξᾰμεν";
    aorist_ind_act_2: "ἐ~ῑξᾰς", "ἐ~ῑξᾰτον", "ἐ~ῑξᾰτε";
    aorist_ind_act_3: "ἐ~ῑξε|ἐ~ῑξεν", "ἐ~ῑξᾰτην", "ἐ~ῑξᾰν";
    aorist_ind_mid_1: "ἐ~ῑξᾰμην", _, "ἐ~ῑξᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ῑξω", "ἐ~ῑξᾰσθον", "ἐ~ῑξᾰσθε";
    aorist_ind_mid_3: "ἐ~ῑξᾰτο", "ἐ~ῑξᾰσθην", "ἐ~ῑξᾰντο";
    aorist_ind_pass_1: "ἐ~ῑχθην", _, "ἐ~ῑχθημεν";
    aorist_ind_pass_2: "ἐ~ῑχθης", "ἐ~ῑχθητον", "ἐ~ῑχθητε";
    aorist_ind_pass_3: "ἐ~ῑχθη", "ἐ~ῑχθητην", "ἐ~ῑχθησᾰν";
    aorist_subj_act_1: "~ῑξω", _, "~ῑξωμεν";
    aorist_subj_act_2: "~ῑξῃς", "~ῑξητον", "~ῑξητε";
    aorist_subj_act_3: "~ῑξῃ", _, "~ῑξωσῐ|~ῑξωσῐν";
    aorist_subj_mid_1: "~ῑξωμαι", _, "~ῑξωμεθᾰ";
    aorist_subj_mid_2: "~ῑξῃ", "~ῑξησθον", "~ῑξησθε";
    aorist_subj_mid_3: "~ῑξηται", _, "~ῑξωνται";
    aorist_subj_pass_1: "~ῑχθω", _, "~ῑχθωμεν";
    aorist_subj_pass_2: "~ῑχθῃς", "~ῑχθητον", "~ῑχθητε";
    aorist_subj_pass_3: "~ῑχθῃ", _, "~ῑχθωσῐ|~ῑχθωσῐν";
    aorist_opt_act_1: "~ῑξαιμῐ", _, "~ῑξαιμεν";
    aorist_opt_act_2: "~ῑξειᾰς|~ῑξαις", "~ῑξαιτον", "~ῑξαιτε";
    aorist_opt_act_3: "~ῑξειε|~ῑξειεν|~ῑξαι", "~ῑξαιτην", "~ῑξειᾰν|~ῑξαιεν";
    aorist_opt_mid_1: "~ῑξαιμην", _, "~ῑξαιμεθᾰ";
    aorist_opt_mid_2: "~ῑξαιο", "~ῑξαισθον", "~ῑξαισθε";
    aorist_opt_mid_3: "~ῑξαιτο", "~ῑξαισθην", "~ῑξαιντο";
    aorist_opt_pass_1: "~ῑχθειην", _, "~ῑχθεῑμεν|~ῑχθειημεν";
    aorist_opt_pass_2: "~ῑχθειης", "~ῑχθεῑτον|~ῑχθειητον", "~ῑχθεῑτε|~ῑχθειητε";
    aorist_opt_pass_3: "~ῑχθειη", "~ῑχθειτην|~ῑχθειητην", "~ῑχθεῑεν|~ῑχθειησᾰν";
    aorist_imp_act_2: "~ῑξον", "~ῑξᾰτον", "~ῑξᾰτε";
    aorist_imp_act_3: "~ῑξᾰτω", "~ῑξᾰτων", "~ῑξᾰντων";
    aorist_imp_mid_2: "~ῑξαι", "~ῑξᾰσθον", "~ῑξᾰσθε";
    aorist_imp_mid_3: "~ῑξᾰσθω", "~ῑξᾰσθων", _;
    aorist_imp_pass_2: "~ῑχθητῐ", _, _;
    aorist_imp_pass_3: "~ῑχθητω", "~ῑχθητων", "~ῑχθεντων";
    perfect_ind_act_1: "~εμῐχᾰ", _, "~εμῐχᾰμεν";
    perfect_ind_act_2: "~εμῐχᾰς", "~εμῐχᾰτον", "~εμῐχᾰτε";
    perfect_ind_act_3: "~εμῐχε|~εμῐχεν", _, "~εμῐχᾱσῐ|~εμῐχᾱσῐν";
    perfect_ind_mid_1: "~εμῐγμαι", _, "~εμῐγμεθᾰ";
    perfect_ind_mid_2: "~εμῐξαι", "~εμῐχθον", "~εμῐχθε";
    perfect_ind_mid_3: "~εμῐκται", _, "~εμῐγᾰται";
    perfect_ind_pass_1: _, _, "~εμῐγμεθᾰ";
    perfect_ind_pass_2: _, _, "~εμῐχθε";
    perfect_subj_act_1: "~εμῐχω", _, "~εμῐχωμεν";
    perfect_subj_act_2: "~εμῐχῃς", "~εμῐχητον", "~εμῐχητε";
    perfect_subj_act_3: "~εμῐχῃ", _, "~εμῐχωσῐ|~εμῐχωσῐν";
    perfect_opt_act_1: "~εμῐχοιμῐ|~εμῐχοιην", _, "~εμῐχοιμεν";
    perfect_opt_act_2: "~εμῐχοις|~εμῐχοιης", "~εμῐχοιτον", "~εμῐχοιτε";
    perfect_opt_act_3: "~εμῐχοι|~εμῐχοιη", "~εμῐχοιτην", "~εμῐχοιεν";
    perfect_imp_act_2: "~εμῐχε", "~εμῐχετον", "~εμῐχετε";
    perfect_imp_act_3: "~εμῐχετω", "~εμῐχετων", "~εμῐχοντων";
    perfect_imp_mid_2: "~εμῐξο", _, _;
    perfect_imp_mid_3: "~εμῐχθω", "~εμῐχθων", _;
    perfect_imp_pass_2: "~εμῐξο", _, _;
    perfect_imp_pass_3: "~εμῐχθω", "~εμῐχθων", _;
    pluperfect_act_1: "ἐ~εμῐχειν|ἐ~εμῐχη", _, "ἐ~εμῐχεμεν";
    pluperfect_act_2: "ἐ~εμῐχεις|ἐ~εμῐχης", "ἐ~εμῐχετον", "ἐ~εμῐχετε";
    pluperfect_act_3: "ἐ~εμῐχει|ἐ~εμῐχειν", "ἐ~εμῐχετην", "ἐ~εμῐχεσᾰν";
    pluperfect_mid_1: "ἐ~εμῐγμην", _, "ἐ~εμῐγμεθᾰ";
    pluperfect_mid_2: "ἐ~εμῐξο", "ἐ~εμῐχθον", "ἐ~εμῐχθε";
    pluperfect_mid_3: "ἐ~εμῐκτο", "ἐ~εμῐχθην", _;
    pluperfect_pass_1: _, _, "ἐ~εμῐγμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~εμῐχθε";
    pluperfect_pass_3: _, "ἐ~εμῐχθην", "ἐ~εμῐγᾰτο";
    future_perfect_ind_mid_1: "~εμῑξομαι", _, "~εμῑξομεθᾰ";
    future_perfect_ind_mid_2: "~εμῑξῃ|~εμῑξει", "~εμῑξεσθον", "~εμῑξεσθε";
    future_perfect_ind_mid_3: "~εμῑξεται", _, "~εμῑξονται";
    future_perfect_ind_pass_2: "~εμῑξῃ|~εμῑξει", _, _;
    future_perfect_opt_mid_1: "~εμῑξοιμην", _, "~εμῑξοιμεθᾰ";
    future_perfect_opt_mid_2: "~εμῑξοιο", "~εμῑξοισθον", "~εμῑξοισθε";
    future_perfect_opt_mid_3: "~εμῑξοιτο", "~εμῑξοισθην", "~εμῑξοιντο";
    infinitive_pres: "~ῑγνῠναι", "~ῑγνῠσθαι", _;
    infinitive_fut: "~ῑξειν", "~ῑξεσθαι", "~ῑχθησεσθαι";
    infinitive_aor: "~ῑξαι", "~ῑξᾰσθαι", "~ῑχθηναι";
    infinitive_perf: "~εμῐχεναι", "~εμῐχθαι", _;
    infinitive_futp: _, "~εμῑξεσθαι", _;
    participle_pres: "~ῑγνῡς", "~ῑγνῠμενος", _;
    participle_fut: "~ῑξων", "~ῑξομενος", "~ῑχθησομενος";
    participle_aor: "~ῑξᾱς", "~ῑξᾰμενος", "~ῑχθεις";
    participle_perf: "~εμῐχως", "~εμῐγμενος", _;
    participle_futp: _, "~εμῑξομενος", _;
    ),
    // Διατιθημῐ: 1 words (διατιθημῐ). Like Ἐγκειμαι, except in 144 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("διατιθημῐ", 1, "~ατι~ημῐ", like Ἐγκειμαι;
    ind_act_1: "~ατι~ημῐ", _, "~ατι~εμεν";
    ind_act_2: "~ατι~ης", _, _;
    ind_act_3: "~ατι~ησῐ|~ατι~ησῐν", _, "~ατι~εᾱσῐ|~ατι~εᾱσῐν";
    ind_mid_1: "~ατι~εμαι", _, "~ατι~εμεθᾰ";
    ind_mid_2: "~ατι~εσαι", "~ατι~εσθον", "~ατι~εσθε";
    ind_mid_3: "~ατι~εται", _, "~ατι~ενται";
    ind_pass_1: _, _, "~ατι~εμεθᾰ";
    ind_pass_2: _, _, "~ατι~εσθε";
    ind_pass_3: "~ατι~εται", _, "~ατι~ενται";
    subj_act_1: "~ατι~ω", _, "~ατι~ωμεν";
    subj_act_2: "~ατι~ῃς", "~ατι~ητον", "~ατι~ητε";
    subj_act_3: "~ατι~ῃ", _, "~ατι~ωσῐ|~ατι~ωσῐν";
    subj_mid_1: "~ατι~ωμαι", _, "~ατι~ωμεθᾰ";
    subj_mid_2: "~ατι~ῃ", "~ατι~ησθον", "~ατι~ησθε";
    subj_mid_3: "~ατι~ηται", _, "~ατι~ωνται";
    subj_pass_1: _, _, "~ατι~ωμεθᾰ";
    subj_pass_2: _, _, "~ατι~ησθε";
    subj_pass_3: "~ατι~ηται", _, "~ατι~ωνται";
    opt_act_1: "~ατι~ειην", _, "~ατι~εῑμεν|~ατι~ειημεν";
    opt_act_2: "~ατι~ειης", "~ατι~εῑτον|~ατι~ειητον", "~ατι~εῑτε|~ατι~ειητε";
    opt_act_3: "~ατι~ειη", "~ατι~ειτην|~ατι~ειητην", "~ατι~εῑεν|~ατι~ειησᾰν";
    opt_mid_1: "~ατι~ειμην", _, "~ατι~ειμεθᾰ|~ατι~οιμεθᾰ";
    opt_mid_2: "~ατι~εῑο", "~ατι~εῑσθον|~ατι~οῑσθον", "~ατι~εῑσθε|~ατι~οῑσθε";
    opt_mid_3: "~ατι~εῑτο|~ατι~οῑτο", "~ατι~εισθην|~ατι~οισθην", "~ατι~εῑντο|~ατι~οῑντο";
    opt_pass_1: _, _, "~ατι~ειμεθᾰ|~ατι~οιμεθᾰ";
    opt_pass_2: _, "~ατι~εῑσθον|~ατι~οῑσθον", "~ατι~εῑσθε|~ατι~οῑσθε";
    opt_pass_3: "~ατι~εῑτο|~ατι~οῑτο", "~ατι~εισθην|~ατι~οισθην", "~ατι~εῑντο|~ατι~οῑντο";
    imp_act_2: "~ατι~ει", "~ατι~ετον", "~ατι~ετε";
    imp_act_3: "~ατι~ετω", "~ατι~ετων", "~ατι~εντων";
    imp_mid_2: "~ατι~εσο|~ατι~ου", _, "~ατι~εσθε";
    imp_mid_3: "~ατι~εσθω", "~ατι~εσθων", "~ατι~εσθων";
    imp_pass_2: "~ατι~εσο|~ατι~ου", _, "~ατι~εσθε";
    imp_pass_3: _, "~ατι~εσθων", _;
    imperfect_act_1: "~ετι~ην", _, "~ετι~εμεν";
    imperfect_act_2: "~ετι~εις", "~ετι~ετον", "~ετι~ετε";
    imperfect_act_3: "~ετι~ει", "~ετι~ετην", "~ετι~εσᾰν";
    imperfect_mid_1: "~ετι~εμην", _, "~ετι~εμεθᾰ";
    imperfect_mid_2: "~ετι~εσο", "~ετι~εσθον", "~ετι~εσθε";
    imperfect_mid_3: "~ετι~ετο", "~ετι~εσθην", "~ετι~εντο";
    imperfect_pass_1: _, _, "~ετι~εμεθᾰ";
    imperfect_pass_2: _, _, "~ετι~εσθε";
    imperfect_pass_3: "~ετι~ετο", _, _;
    future_ind_act_1: "~α~ησω", _, "~α~ησομεν";
    future_ind_act_2: "~α~ησεις", "~α~ησετον", "~α~ησετε";
    future_ind_act_3: "~α~ησει", _, "~α~ησουσῐ|~α~ησουσῐν";
    future_ind_mid_1: "~α~ησομαι", _, "~α~ησομεθᾰ";
    future_ind_mid_2: "~α~ησῃ|~α~ησει", "~α~ησεσθον", "~α~ησεσθε";
    future_ind_mid_3: "~α~ησεται", _, "~α~ησονται";
    future_ind_pass_1: "~α%ε~ησομαι", _, "~α%ε~ησομεθᾰ";
    future_ind_pass_2: "~α%ε~ησῃ|~α%ε~ησει", "~α%ε~ησεσθον", "~α%ε~ησεσθε";
    future_ind_pass_3: "~α%ε~ησεται", _, "~α%ε~ησονται";
    future_opt_act_1: "~α~ησοιμῐ", _, "~α~ησοιμεν";
    future_opt_act_2: "~α~ησοις", "~α~ησοιτον", "~α~ησοιτε";
    future_opt_act_3: "~α~ησοι", "~α~ησοιτην", "~α~ησοιεν";
    future_opt_mid_1: "~α~ησοιμην", _, "~α~ησοιμεθᾰ";
    future_opt_mid_2: "~α~ησοιο", "~α~ησοισθον", "~α~ησοισθε";
    future_opt_mid_3: "~α~ησοιτο", "~α~ησοισθην", "~α~ησοιντο";
    future_opt_pass_1: "~α%ε~ησοιμην", _, "~α%ε~ησοιμεθᾰ";
    future_opt_pass_2: "~α%ε~ησοιο", "~α%ε~ησοισθον", "~α%ε~ησοισθε";
    future_opt_pass_3: "~α%ε~ησοιτο", "~α%ε~ησοισθην", "~α%ε~ησοιντο";
    infinitive_pres: "~ατι~εναι", "~ατι~εσθαι", _;
    infinitive_fut: "~α~ησειν", "~α~ησεσθαι", "~α%ε~ησεσθαι";
    participle_pres: "~ατι~εις", "~ατι~εμενος", _;
    participle_fut: "~α~ησων", "~α~ησομενος", "~α%ε~ησομενος";
    ),
    // Κερᾰννῡμῐ: 1 words (κερᾰννῡμῐ). Like Προσερχομαι, except in 302 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("κερᾰννῡμῐ", 1, "~ε~ᾰννῡμῐ", like Προσερχομαι;
    ind_act_1: "~ε~ᾰννῡμῐ|~ε~ᾰννῡω", _, "~ε~ᾰννῠμεν|~ε~ᾰννῡομεν";
    ind_act_2: "~ε~ᾰννῡς|~ε~ᾰννῡεις", "~ε~ᾰννῠτον|~ε~ᾰννῡετον", "~ε~ᾰννῠτε|~ε~ᾰννῡετε";
    ind_act_3: "~ε~ᾰννῡσῐ|~ε~ᾰννῡσῐν|~ε~ᾰννῡει", "~ε~ᾰννῠτον|~ε~ᾰννῡετον", "~ε~ᾰννῠᾱσῐ|~ε~ᾰννῠᾱσῐν|~ε~ᾰννῡουσῐ|~ε~ᾰννῡουσῐν";
    ind_mid_1: "~ε~ᾰννῠμαι|~ε~ᾰννῡομαι", _, "~ε~ᾰννῠμεθᾰ|~ε~ᾰννῡομεθᾰ";
    ind_mid_2: "~ε~ᾰννῠσαι|~ε~ᾰννῡῃ|~ε~ᾰννῡει", "~ε~ᾰννῠσθον|~ε~ᾰννῡεσθον", "~ε~ᾰννῠσθε|~ε~ᾰννῡεσθε";
    ind_mid_3: "~ε~ᾰννῠται|~ε~ᾰννῡεται", "~ε~ᾰννῠσθον|~ε~ᾰννῡεσθον", "~ε~ᾰννῠνται|~ε~ᾰννῡονται";
    ind_pass_1: "~ε~ᾰννῠμαι|~ε~ᾰννῡομαι", _, "~ε~ᾰννῠμεθᾰ|~ε~ᾰννῡομεθᾰ";
    ind_pass_2: "~ε~ᾰννῠσαι|~ε~ᾰννῡῃ|~ε~ᾰννῡει", "~ε~ᾰννῠσθον|~ε~ᾰννῡεσθον", "~ε~ᾰννῠσθε|~ε~ᾰννῡεσθε";
    ind_pass_3: "~ε~ᾰννῠται|~ε~ᾰννῡεται", "~ε~ᾰννῠσθον|~ε~ᾰννῡεσθον", "~ε~ᾰννῠνται|~ε~ᾰννῡονται";
    subj_act_1: "~ε~ᾰννῠω|~ε~ᾰννῡω", _, "~ε~ᾰννῠωμεν|~ε~ᾰννῡωμεν";
    subj_act_2: "~ε~ᾰννῠῃς|~ε~ᾰννῡῃς", "~ε~ᾰννῠητον|~ε~ᾰννῡητον", "~ε~ᾰννῠητε|~ε~ᾰννῡητε";
    subj_act_3: "~ε~ᾰννῠῃ|~ε~ᾰννῡῃ", "~ε~ᾰννῠητον|~ε~ᾰννῡητον", "~ε~ᾰννῠωσῐ|~ε~ᾰννῠωσῐν|~ε~ᾰννῡωσῐ|~ε~ᾰννῡωσῐν";
    subj_mid_1: "~ε~ᾰννῠωμαι|~ε~ᾰννῡωμαι", _, "~ε~ᾰννῠωμεθᾰ|~ε~ᾰννῡωμεθᾰ";
    subj_mid_2: "~ε~ᾰννῠῃ|~ε~ᾰννῡῃ", "~ε~ᾰννῠησθον|~ε~ᾰννῡησθον", "~ε~ᾰννῠησθε|~ε~ᾰννῡησθε";
    subj_mid_3: "~ε~ᾰννῠηται|~ε~ᾰννῡηται", "~ε~ᾰννῠησθον|~ε~ᾰννῡησθον", "~ε~ᾰννῠωνται|~ε~ᾰννῡωνται";
    subj_pass_1: "~ε~ᾰννῠωμαι|~ε~ᾰννῡωμαι", _, "~ε~ᾰννῠωμεθᾰ|~ε~ᾰννῡωμεθᾰ";
    subj_pass_2: "~ε~ᾰννῠῃ|~ε~ᾰννῡῃ", "~ε~ᾰννῠησθον|~ε~ᾰννῡησθον", "~ε~ᾰννῠησθε|~ε~ᾰννῡησθε";
    subj_pass_3: "~ε~ᾰννῠηται|~ε~ᾰννῡηται", "~ε~ᾰννῠησθον|~ε~ᾰννῡησθον", "~ε~ᾰννῠωνται|~ε~ᾰννῡωνται";
    opt_act_1: "~ε~ᾰννῠοιμῐ|~ε~ᾰννῡοιμῐ", _, "~ε~ᾰννῠοιμεν|~ε~ᾰννῡοιμεν";
    opt_act_2: "~ε~ᾰννῠοις|~ε~ᾰννῡοις", "~ε~ᾰννῠοιτον|~ε~ᾰννῡοιτον", "~ε~ᾰννῠοιτε|~ε~ᾰννῡοιτε";
    opt_act_3: "~ε~ᾰννῠοι|~ε~ᾰννῡοι", "~ε~ᾰννῠοιτην|~ε~ᾰννῡοιτην", "~ε~ᾰννῠοιεν|~ε~ᾰννῡοιεν";
    opt_mid_1: "~ε~ᾰννῠοιμην|~ε~ᾰννῡοιμην", _, "~ε~ᾰννῠοιμεθᾰ|~ε~ᾰννῡοιμεθᾰ";
    opt_mid_2: "~ε~ᾰννῠοιο|~ε~ᾰννῡοιο", "~ε~ᾰννῠοισθον|~ε~ᾰννῡοισθον", "~ε~ᾰννῠοισθε|~ε~ᾰννῡοισθε";
    opt_mid_3: "~ε~ᾰννῠοιτο|~ε~ᾰννῡοιτο", "~ε~ᾰννῠοισθην|~ε~ᾰννῡοισθην", "~ε~ᾰννῠοιντο|~ε~ᾰννῡοιντο";
    opt_pass_1: "~ε~ᾰννῠοιμην|~ε~ᾰννῡοιμην", _, "~ε~ᾰννῠοιμεθᾰ|~ε~ᾰννῡοιμεθᾰ";
    opt_pass_2: "~ε~ᾰννῠοιο|~ε~ᾰννῡοιο", "~ε~ᾰννῠοισθον|~ε~ᾰννῡοισθον", "~ε~ᾰννῠοισθε|~ε~ᾰννῡοισθε";
    opt_pass_3: "~ε~ᾰννῠοιτο|~ε~ᾰννῡοιτο", "~ε~ᾰννῠοισθην|~ε~ᾰννῡοισθην", "~ε~ᾰννῠοιντο|~ε~ᾰννῡοιντο";
    imp_act_2: "~ε~ᾰννῡ|~ε~ᾰννῡε", "~ε~ᾰννῠτον|~ε~ᾰννῡετον", "~ε~ᾰννῠτε|~ε~ᾰννῡετε";
    imp_act_3: "~ε~ᾰννῠτω|~ε~ᾰννῡετω", "~ε~ᾰννῠτων|~ε~ᾰννῡετων", "~ε~ᾰννῠντων|~ε~ᾰννῡοντων";
    imp_mid_2: "~ε~ᾰννῠσο|~ε~ᾰννῡου", "~ε~ᾰννῠσθον|~ε~ᾰννῡεσθον", "~ε~ᾰννῠσθε|~ε~ᾰννῡεσθε";
    imp_mid_3: "~ε~ᾰννῠσθω|~ε~ᾰννῡεσθω", "~ε~ᾰννῠσθων|~ε~ᾰννῡεσθων", "~ε~ᾰννῠσθων|~ε~ᾰννῡεσθων";
    imp_pass_2: "~ε~ᾰννῠσο|~ε~ᾰννῡου", "~ε~ᾰννῠσθον|~ε~ᾰννῡεσθον", "~ε~ᾰννῠσθε|~ε~ᾰννῡεσθε";
    imp_pass_3: "~ε~ᾰννῠσθω|~ε~ᾰννῡεσθω", "~ε~ᾰννῠσθων|~ε~ᾰννῡεσθων", "~ε~ᾰννῠσθων|~ε~ᾰννῡεσθων";
    imperfect_act_1: "ἐ~ε~ᾰννῡν", _, "ἐ~ε~ᾰννῠμεν";
    imperfect_act_2: "ἐ~ε~ᾰννῡς", "ἐ~ε~ᾰννῠτον", "ἐ~ε~ᾰννῠτε";
    imperfect_act_3: "ἐ~ε~ᾰννῡ", "ἐ~ε~ᾰννῠτην", "ἐ~ε~ᾰννῠσᾰν";
    imperfect_mid_1: "ἐ~ε~ᾰννῠμην", _, "ἐ~ε~ᾰννῠμεθᾰ";
    imperfect_mid_2: "ἐ~ε~ᾰννῠσο", "ἐ~ε~ᾰννῠσθον", "ἐ~ε~ᾰννῠσθε";
    imperfect_mid_3: "ἐ~ε~ᾰννῠτο", "ἐ~ε~ᾰννῠσθην", "ἐ~ε~ᾰννῠντο";
    imperfect_pass_1: _, _, "ἐ~ε~ᾰννῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ε~ᾰννῠσθε";
    imperfect_pass_3: "ἐ~ε~ᾰννῠτο", _, _;
    future_ind_act_1: "~ε~ᾰσω", _, "~ε~ᾰσομεν";
    future_ind_act_2: "~ε~ᾰσεις", "~ε~ᾰσετον", "~ε~ᾰσετε";
    future_ind_act_3: "~ε~ᾰσει", _, "~ε~ᾰσουσῐ|~ε~ᾰσουσῐν";
    future_ind_mid_1: "~ε~ᾰσομαι", _, "~ε~ᾰσομεθᾰ";
    future_ind_mid_2: "~ε~ᾰσῃ|~ε~ᾰσει", "~ε~ᾰσεσθον", "~ε~ᾰσεσθε";
    future_ind_mid_3: "~ε~ᾰσεται", _, "~ε~ᾰσονται";
    future_ind_pass_1: "~~ᾱθησομαι", _, "~~ᾱθησομεθᾰ";
    future_ind_pass_2: "~~ᾱθησῃ|~~ᾱθησει", "~~ᾱθησεσθον", "~~ᾱθησεσθε";
    future_ind_pass_3: "~~ᾱθησεται", _, "~~ᾱθησονται";
    future_opt_act_1: "~ε~ᾰσοιμῐ", _, "~ε~ᾰσοιμεν";
    future_opt_act_2: "~ε~ᾰσοις", "~ε~ᾰσοιτον", "~ε~ᾰσοιτε";
    future_opt_act_3: "~ε~ᾰσοι", "~ε~ᾰσοιτην", "~ε~ᾰσοιεν";
    future_opt_mid_1: "~ε~ᾰσοιμην", _, "~ε~ᾰσοιμεθᾰ";
    future_opt_mid_2: "~ε~ᾰσοιο", "~ε~ᾰσοισθον", "~ε~ᾰσοισθε";
    future_opt_mid_3: "~ε~ᾰσοιτο", "~ε~ᾰσοισθην", "~ε~ᾰσοιντο";
    future_opt_pass_1: "~~ᾱθησοιμην", _, "~~ᾱθησοιμεθᾰ";
    future_opt_pass_2: "~~ᾱθησοιο", "~~ᾱθησοισθον", "~~ᾱθησοισθε";
    future_opt_pass_3: "~~ᾱθησοιτο", "~~ᾱθησοισθην", "~~ᾱθησοιντο";
    aorist_ind_act_1: "ἐ~ε~ᾰσᾰ", _, "ἐ~ε~ᾰσᾰμεν";
    aorist_ind_act_2: "ἐ~ε~ᾰσᾰς", "ἐ~ε~ᾰσᾰτον", "ἐ~ε~ᾰσᾰτε";
    aorist_ind_act_3: "ἐ~ε~ᾰσε|ἐ~ε~ᾰσεν", "ἐ~ε~ᾰσᾰτην", "ἐ~ε~ᾰσᾰν";
    aorist_ind_mid_1: "ἐ~ε~ᾰσᾰμην", _, "ἐ~ε~ᾰσᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ε~ᾰσω", "ἐ~ε~ᾰσᾰσθον", "ἐ~ε~ᾰσᾰσθε";
    aorist_ind_mid_3: "ἐ~ε~ᾰσᾰτο", "ἐ~ε~ᾰσᾰσθην", "ἐ~ε~ᾰσᾰντο";
    aorist_ind_pass_1: "ἐ~~ᾱθην", _, "ἐ~~ᾱθημεν";
    aorist_ind_pass_2: "ἐ~~ᾱθης", "ἐ~~ᾱθητον", "ἐ~~ᾱθητε";
    aorist_ind_pass_3: "ἐ~~ᾱθη", "ἐ~~ᾱθητην", "ἐ~~ᾱθησᾰν";
    aorist_subj_act_1: "~ε~ᾰσω", _, "~ε~ᾰσωμεν";
    aorist_subj_act_2: "~ε~ᾰσῃς", "~ε~ᾰσητον", "~ε~ᾰσητε";
    aorist_subj_act_3: "~ε~ᾰσῃ", _, "~ε~ᾰσωσῐ|~ε~ᾰσωσῐν";
    aorist_subj_mid_1: "~ε~ᾰσωμαι", _, "~ε~ᾰσωμεθᾰ";
    aorist_subj_mid_2: "~ε~ᾰσῃ", "~ε~ᾰσησθον", "~ε~ᾰσησθε";
    aorist_subj_mid_3: "~ε~ᾰσηται", _, "~ε~ᾰσωνται";
    aorist_subj_pass_1: "~~ᾱθω", _, "~~ᾱθωμεν";
    aorist_subj_pass_2: "~~ᾱθῃς", "~~ᾱθητον", "~~ᾱθητε";
    aorist_subj_pass_3: "~~ᾱθῃ", _, "~~ᾱθωσῐ|~~ᾱθωσῐν";
    aorist_opt_act_1: "~ε~ᾰσαιμῐ", _, "~ε~ᾰσαιμεν";
    aorist_opt_act_2: "~ε~ᾰσειᾰς|~ε~ᾰσαις", "~ε~ᾰσαιτον", "~ε~ᾰσαιτε";
    aorist_opt_act_3: "~ε~ᾰσειε|~ε~ᾰσειεν|~ε~ᾰσαι", "~ε~ᾰσαιτην", "~ε~ᾰσειᾰν|~ε~ᾰσαιεν";
    aorist_opt_mid_1: "~ε~ᾰσαιμην", _, "~ε~ᾰσαιμεθᾰ";
    aorist_opt_mid_2: "~ε~ᾰσαιο", "~ε~ᾰσαισθον", "~ε~ᾰσαισθε";
    aorist_opt_mid_3: "~ε~ᾰσαιτο", "~ε~ᾰσαισθην", "~ε~ᾰσαιντο";
    aorist_opt_pass_1: "~~ᾱθειην", _, "~~ᾱθεῑμεν|~~ᾱθειημεν";
    aorist_opt_pass_2: "~~ᾱθειης", "~~ᾱθεῑτον|~~ᾱθειητον", "~~ᾱθεῑτε|~~ᾱθειητε";
    aorist_opt_pass_3: "~~ᾱθειη", "~~ᾱθειτην|~~ᾱθειητην", "~~ᾱθεῑεν|~~ᾱθειησᾰν";
    aorist_imp_act_2: "~ε~ᾰσον", "~ε~ᾰσᾰτον", "~ε~ᾰσᾰτε";
    aorist_imp_act_3: "~ε~ᾰσᾰτω", "~ε~ᾰσᾰτων", "~ε~ᾰσᾰντων";
    aorist_imp_mid_2: "~ε~ᾰσαι", "~ε~ᾰσᾰσθον", "~ε~ᾰσᾰσθε";
    aorist_imp_mid_3: "~ε~ᾰσᾰσθω", "~ε~ᾰσᾰσθων", _;
    aorist_imp_pass_2: "~~ᾱθητῐ", _, _;
    aorist_imp_pass_3: "~~ᾱθητω", "~~ᾱθητων", "~~ᾱθεντων";
    perfect_ind_mid_1: "%ε~~ᾱμαι", _, "%ε~~ᾱμεθᾰ";
    perfect_ind_mid_2: "%ε~~ᾱσαι", "%ε~~ᾱσθον", "%ε~~ᾱσθε";
    perfect_ind_mid_3: "%ε~~ᾱται", _, "%ε~~ᾱνται";
    perfect_ind_pass_1: _, _, "%ε~~ᾱμεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~~ᾱσθε";
    perfect_imp_mid_2: "%ε~~ᾱσο", _, _;
    perfect_imp_mid_3: "%ε~~ᾱσθω", "%ε~~ᾱσθων", _;
    perfect_imp_pass_2: "%ε~~ᾱσο", _, _;
    perfect_imp_pass_3: "%ε~~ᾱσθω", "%ε~~ᾱσθων", _;
    pluperfect_mid_1: "ἐ%ε~~ᾱμην", _, "ἐ%ε~~ᾱμεθᾰ";
    pluperfect_mid_2: "ἐ%ε~~ᾱσο", "ἐ%ε~~ᾱσθον", "ἐ%ε~~ᾱσθε";
    pluperfect_mid_3: "ἐ%ε~~ᾱτο", "ἐ%ε~~ᾱσθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~~ᾱμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~~ᾱσθε";
    pluperfect_pass_3: _, "ἐ%ε~~ᾱσθην", "ἐ%ε~~ᾱντο";
    future_perfect_ind_mid_1: "%ε~~ᾱσομαι", _, "%ε~~ᾱσομεθᾰ";
    future_perfect_ind_mid_2: "%ε~~ᾱσῃ|%ε~~ᾱσει", "%ε~~ᾱσεσθον", "%ε~~ᾱσεσθε";
    future_perfect_ind_mid_3: "%ε~~ᾱσεται", _, "%ε~~ᾱσονται";
    future_perfect_ind_pass_2: "%ε~~ᾱσῃ|%ε~~ᾱσει", _, _;
    future_perfect_opt_mid_1: "%ε~~ᾱσοιμην", _, "%ε~~ᾱσοιμεθᾰ";
    future_perfect_opt_mid_2: "%ε~~ᾱσοιο", "%ε~~ᾱσοισθον", "%ε~~ᾱσοισθε";
    future_perfect_opt_mid_3: "%ε~~ᾱσοιτο", "%ε~~ᾱσοισθην", "%ε~~ᾱσοιντο";
    infinitive_pres: "~ε~ᾰννῠναι|~ε~ᾰννῡειν", "~ε~ᾰννῠσθαι|~ε~ᾰννῡεσθαι", "~ε~ᾰννῠσθαι|~ε~ᾰννῡεσθαι";
    infinitive_fut: "~ε~ᾰσειν", "~ε~ᾰσεσθαι", "~~ᾱθησεσθαι";
    infinitive_aor: "~ε~ᾰσαι", "~ε~ᾰσᾰσθαι", "~~ᾱθηναι";
    infinitive_perf: _, "%ε~~ᾱσθαι", _;
    infinitive_futp: _, "%ε~~ᾱσεσθαι", _;
    participle_pres: "~ε~ᾰννῡς|~ε~ᾰννῡων", "~ε~ᾰννῠμενος|~ε~ᾰννῡομενος", "~ε~ᾰννῠμενος|~ε~ᾰννῡομενος";
    participle_fut: "~ε~ᾰσων", "~ε~ᾰσομενος", "~~ᾱθησομενος";
    participle_aor: "~ε~ᾰσᾱς", "~ε~ᾰσᾰμενος", "~~ᾱθεις";
    participle_perf: _, "%ε~~ᾱμενος", _;
    participle_futp: _, "%ε~~ᾱσομενος", _;
    ),
    // Δυναμαι: 1 words (δυναμαι). Like Προσερχομαι, except in 188 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("δυναμαι", 1, "~υ~αμαι", like Προσερχομαι;
    ind_act_1: "~υ~αμαι", _, _;
    ind_mid_1: "~ῠ~ᾰμαι", _, "~ῠ~ᾰμεθᾰ";
    ind_mid_2: "~ῠ~ᾰσαι", "~ῠ~ᾰσθον", "~ῠ~ᾰσθε";
    ind_mid_3: "~ῠ~ᾰται", _, "~ῠ~ᾰνται";
    ind_pass_1: _, _, "~ῠ~ᾰμεθᾰ";
    ind_pass_2: _, _, "~ῠ~ᾰσθε";
    ind_pass_3: "~ῠ~ᾰται", _, "~ῠ~ᾰνται";
    subj_mid_1: "~ῠ~ωμαι", _, "~ῠ~ωμεθᾰ";
    subj_mid_2: "~ῠ~ῃ", "~ῠ~ησθον", "~ῠ~ησθε";
    subj_mid_3: "~ῠ~ηται", _, "~ῠ~ωνται";
    subj_pass_1: _, _, "~ῠ~ωμεθᾰ";
    subj_pass_2: _, _, "~ῠ~ησθε";
    subj_pass_3: "~ῠ~ηται", _, "~ῠ~ωνται";
    opt_mid_1: "~ῠ~αιμην", _, "~ῠ~αιμεθᾰ";
    opt_mid_2: "~ῠ~αιο", "~ῠ~αισθον", "~ῠ~αισθε";
    opt_mid_3: "~ῠ~αιτο", "~ῠ~αισθην", "~ῠ~αιντο";
    opt_pass_1: _, _, "~ῠ~αιμεθᾰ";
    opt_pass_2: _, "~ῠ~αισθον", "~ῠ~αισθε";
    opt_pass_3: "~ῠ~αιτο", "~ῠ~αισθην", "~ῠ~αιντο";
    imp_mid_2: "~ῠ~ᾰσο|~ῠ~ω", _, "~ῠ~ᾰσθε";
    imp_mid_3: "~ῠ~ᾰσθω", "~ῠ~ᾰσθων", "~ῠ~ᾰσθων";
    imp_pass_2: "~ῠ~ᾰσο|~ῠ~ω", _, "~ῠ~ᾰσθε";
    imp_pass_3: _, "~ῠ~ᾰσθων", _;
    imperfect_mid_1: "ἐ~ῠ~ᾰμην|ἠ~ῠ~ᾰμην", _, "ἐ~ῠ~ᾰμεθᾰ|ἠ~ῠ~ᾰμεθᾰ";
    imperfect_mid_2: "ἐ~ῠ~ω|ἐ~ῠ~ασο|ἠ~ῠ~ω|ἠ~ῠ~ασο", "ἐ~ῠ~ᾰσθον|ἠ~ῠ~ᾰσθον", "ἐ~ῠ~ᾰσθε|ἠ~ῠ~ᾰσθε";
    imperfect_mid_3: "ἐ~ῠ~ᾰτο|ἠ~ῠ~ᾰτο", "ἐ~ῠ~ᾰσθην|ἠ~ῠ~ᾰσθην", "ἐ~ῠ~ᾰντο|ἠ~ῠ~ᾰντο";
    imperfect_pass_1: "ἐ~ῠ~ᾰμην|ἠ~ῠ~ᾰμην", _, "ἐ~ῠ~ᾰμεθᾰ|ἠ~ῠ~ᾰμεθᾰ";
    imperfect_pass_2: "ἐ~ῠ~ω|ἐ~ῠ~ασο|ἠ~ῠ~ω|ἠ~ῠ~ασο", "ἐ~ῠ~ᾰσθον|ἠ~ῠ~ᾰσθον", "ἐ~ῠ~ᾰσθε|ἠ~ῠ~ᾰσθε";
    imperfect_pass_3: "ἐ~ῠ~ᾰτο|ἠ~ῠ~ᾰτο", "ἐ~ῠ~ᾰσθην|ἠ~ῠ~ᾰσθην", "ἐ~ῠ~ᾰντο|ἠ~ῠ~ᾰντο";
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, derived;
    future_ind_mid_3: derived, _, derived;
    future_ind_pass_1: "~ῠ~ησομαι|~ῠ~ηθησομαι", _, "~ῠ~ησομεθᾰ|~ῠ~ηθησομεθᾰ";
    future_ind_pass_2: "~ῠ~ησῃ|~ῠ~ησει|~ῠ~ηθησῃ|~ῠ~ηθησει", "~ῠ~ησεσθον|~ῠ~ηθησεσθον", "~ῠ~ησεσθε|~ῠ~ηθησεσθε";
    future_ind_pass_3: "~ῠ~ησεται|~ῠ~ηθησεται", "~ῠ~ησεσθον|~ῠ~ηθησεσθον", "~ῠ~ησονται|~ῠ~ηθησονται";
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, derived, derived;
    future_opt_mid_3: derived, derived, derived;
    future_opt_pass_1: "~ῠ~ησοιμην|~ῠ~ηθησοιμην", _, "~ῠ~ησοιμεθᾰ|~ῠ~ηθησοιμεθᾰ";
    future_opt_pass_2: "~ῠ~ησοιο|~ῠ~ηθησοιο", "~ῠ~ησοισθον|~ῠ~ηθησοισθον", "~ῠ~ησοισθε|~ῠ~ηθησοισθε";
    future_opt_pass_3: "~ῠ~ησοιτο|~ῠ~ηθησοιτο", "~ῠ~ησοισθην|~ῠ~ηθησοισθην", "~ῠ~ησοιντο|~ῠ~ηθησοιντο";
    aorist_ind_mid_1: "ἐ~ῠ~ησᾰμην", _, "ἐ~ῠ~ησᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ῠ~ησω", "ἐ~ῠ~ησᾰσθον", "ἐ~ῠ~ησᾰσθε";
    aorist_ind_mid_3: "ἐ~ῠ~ησᾰτο", "ἐ~ῠ~ησᾰσθην", "ἐ~ῠ~ησᾰντο";
    aorist_ind_pass_1: "ἐ~ῠ~ασθην|ἐ~ῠ~ηθην|ἠ~ῠ~ηθην|ἠ~ῠ~ασθην", _, "ἐ~ῠ~ασθημεν|ἐ~ῠ~ηθημεν|ἠ~ῠ~ηθημεν|ἠ~ῠ~ασθημεν";
    aorist_ind_pass_2: "ἐ~ῠ~ασθης|ἐ~ῠ~ηθης|ἠ~ῠ~ηθης|ἠ~ῠ~ασθης", "ἐ~ῠ~ασθητον|ἐ~ῠ~ηθητον|ἠ~ῠ~ηθητον|ἠ~ῠ~ασθητον", "ἐ~ῠ~ασθητε|ἐ~ῠ~ηθητε|ἠ~ῠ~ηθητε|ἠ~ῠ~ασθητε";
    aorist_ind_pass_3: "ἐ~ῠ~ασθη|ἐ~ῠ~ηθη|ἠ~ῠ~ηθη|ἠ~ῠ~ασθη", "ἐ~ῠ~ασθητην|ἐ~ῠ~ηθητην|ἠ~ῠ~ηθητην|ἠ~ῠ~ασθητην", "ἐ~ῠ~ασθησᾰν|ἐ~ῠ~ηθησᾰν|ἠ~ῠ~ηθησᾰν|ἠ~ῠ~ασθησᾰν";
    aorist_subj_mid_1: "~ῠ~ησωμαι", _, "~ῠ~ησωμεθᾰ";
    aorist_subj_mid_2: "~ῠ~ησῃ", "~ῠ~ησησθον", "~ῠ~ησησθε";
    aorist_subj_mid_3: "~ῠ~ησηται", _, "~ῠ~ησωνται";
    aorist_subj_pass_1: "~ῠ~ασθω|~ῠ~ηθω", _, "~ῠ~ασθωμεν|~ῠ~ηθωμεν";
    aorist_subj_pass_2: "~ῠ~ασθῃς|~ῠ~ηθῃς", "~ῠ~ασθητον|~ῠ~ηθητον", "~ῠ~ασθητε|~ῠ~ηθητε";
    aorist_subj_pass_3: "~ῠ~ασθῃ|~ῠ~ηθῃ", "~ῠ~ασθητον|~ῠ~ηθητον", "~ῠ~ασθωσῐ|~ῠ~ασθωσῐν|~ῠ~ηθωσῐ|~ῠ~ηθωσῐν";
    aorist_opt_mid_1: "~ῠ~ησαιμην", _, "~ῠ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ῠ~ησαιο", "~ῠ~ησαισθον", "~ῠ~ησαισθε";
    aorist_opt_mid_3: "~ῠ~ησαιτο", "~ῠ~ησαισθην", "~ῠ~ησαιντο";
    aorist_opt_pass_1: "~ῠ~ασθειην|~ῠ~ηθειην", _, "~ῠ~ασθεῑμεν|~ῠ~ασθειημεν|~ῠ~ηθεῑμεν|~ῠ~ηθειημεν";
    aorist_opt_pass_2: "~ῠ~ασθειης|~ῠ~ηθειης", "~ῠ~ασθεῑτον|~ῠ~ασθειητον|~ῠ~ηθεῑτον|~ῠ~ηθειητον", "~ῠ~ασθεῑτε|~ῠ~ασθειητε|~ῠ~ηθεῑτε|~ῠ~ηθειητε";
    aorist_opt_pass_3: "~ῠ~ασθειη|~ῠ~ηθειη", "~ῠ~ασθειτην|~ῠ~ασθειητην|~ῠ~ηθειτην|~ῠ~ηθειητην", "~ῠ~ασθεῑεν|~ῠ~ασθειησᾰν|~ῠ~ηθεῑεν|~ῠ~ηθειησᾰν";
    aorist_imp_mid_2: "~ῠ~ησαι", "~ῠ~ησᾰσθον", "~ῠ~ησᾰσθε";
    aorist_imp_mid_3: "~ῠ~ησᾰσθω", "~ῠ~ησᾰσθων", _;
    aorist_imp_pass_2: "~ῠ~ασθητῐ|~ῠ~ηθητῐ", "~ῠ~ασθητον|~ῠ~ηθητον", "~ῠ~ασθητε|~ῠ~ηθητε";
    aorist_imp_pass_3: "~ῠ~ασθητω|~ῠ~ηθητω", "~ῠ~ασθητων|~ῠ~ηθητων", "~ῠ~ασθεντων|~ῠ~ηθεντων";
    perfect_ind_mid_1: "~εδῠ~ημαι", _, "~εδῠ~ημεθᾰ";
    perfect_ind_mid_2: "~εδῠ~ησαι", "~εδῠ~ησθον", "~εδῠ~ησθε";
    perfect_ind_mid_3: "~εδῠ~ηται", _, "~εδῠ~ηνται";
    perfect_ind_pass_1: _, _, "~εδῠ~ημεθᾰ";
    perfect_ind_pass_2: _, _, "~εδῠ~ησθε";
    perfect_imp_mid_2: "~εδῠ~ησο", _, _;
    perfect_imp_mid_3: "~εδῠ~ησθω", "~εδῠ~ησθων", _;
    perfect_imp_pass_2: "~εδῠ~ησο", _, _;
    perfect_imp_pass_3: "~εδῠ~ησθω", "~εδῠ~ησθων", _;
    pluperfect_mid_1: "ἐ~εδῠ~ημην", _, "ἐ~εδῠ~ημεθᾰ";
    pluperfect_mid_2: "ἐ~εδῠ~ησο", "ἐ~εδῠ~ησθον", "ἐ~εδῠ~ησθε";
    pluperfect_mid_3: "ἐ~εδῠ~ητο", "ἐ~εδῠ~ησθην", _;
    pluperfect_pass_1: _, _, "ἐ~εδῠ~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~εδῠ~ησθε";
    pluperfect_pass_3: _, "ἐ~εδῠ~ησθην", "ἐ~εδῠ~ηντο";
    infinitive_pres: _, "~ῠ~ᾰσθαι", _;
    infinitive_fut: _, derived, "~ῠ~ησεσθαι|~ῠ~ηθησεσθαι";
    infinitive_aor: _, "~ῠ~ησᾰσθαι", "~ῠ~ασθηναι|~ῠ~ηθηναι";
    infinitive_perf: _, "~εδῠ~ησθαι", _;
    participle_pres: _, "~ῠ~ᾰμενος", _;
    participle_fut: _, derived, "~ῠ~ησομενος|~ῠ~ηθησομενος";
    participle_aor: _, "~ῠ~ησᾰμενος", "~ῠ~ασθεις|~ῠ~ηθεις";
    participle_perf: _, "~εδῠ~ημενος", _;
    ),
    // Οἰδᾰ: 1 words (οἰδᾰ). Like Ἐφᾰγον, except in 49 cells: ind_act_1 sg, future_ind_mid_1 sg, future_ind_mid_1 pl, future_ind_mid_2 sg, future_ind_mid_2 du, future_ind_mid_2 pl, ….
    conjugation!("οἰδᾰ", 1, "ο~δᾰ", like Ἐφᾰγον;
    ind_act_1: "ο~δᾰ", _, _;
    future_ind_mid_1: "ε~σομαι", _, "ε~σομεθᾰ";
    future_ind_mid_2: "ε~σῃ|ε~σει", "ε~σεσθον", "ε~σεσθε";
    future_ind_mid_3: "ε~σεται", _, "ε~σονται";
    future_opt_mid_1: "ε~σοιμην", _, "ε~σοιμεθᾰ";
    future_opt_mid_2: "ε~σοιο", "ε~σοισθον", "ε~σοισθε";
    future_opt_mid_3: "ε~σοιτο", "ε~σοισθην", "ε~σοιντο";
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_subj_act_1: derived, _, derived;
    aorist_subj_act_2: derived, derived, derived;
    aorist_subj_act_3: derived, _, derived;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_imp_act_2: derived, derived, derived;
    aorist_imp_act_3: derived, derived, derived;
    infinitive_fut: _, "ε~σεσθαι", _;
    infinitive_aor: derived, _, _;
    participle_fut: _, "ε~σομενος", _;
    participle_aor: derived, _, _;
    ),
    // Πᾰρᾰδιδωμῐ: 1 words (πᾰρᾰδιδωμῐ). Like Εἰπον, except in 216 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("πᾰρᾰδιδωμῐ", 1, "~ᾰ~ιδωμῐ", like Εἰπον;
    ind_act_1: "~ᾰ~ιδωμῐ", _, "~ᾰ~ιδομεν";
    ind_act_2: "~ᾰ~ιδως", _, _;
    ind_act_3: "~ᾰ~ιδωσῐ|~ᾰ~ιδωσῐν", _, "~ᾰ~ιδοᾱσῐ|~ᾰ~ιδοᾱσῐν";
    ind_mid_1: "~ᾰ~ιδομαι", _, "~ᾰ~ιδομεθᾰ";
    ind_mid_2: "~ᾰ~ιδοσαι", "~ᾰ~ιδοσθον", "~ᾰ~ιδοσθε";
    ind_mid_3: "~ᾰ~ιδοται", _, "~ᾰ~ιδονται";
    ind_pass_1: _, _, "~ᾰ~ιδομεθᾰ";
    ind_pass_2: _, _, "~ᾰ~ιδοσθε";
    ind_pass_3: "~ᾰ~ιδοται", _, "~ᾰ~ιδονται";
    subj_act_1: "~ᾰ~ιδω", _, "~ᾰ~ιδωμεν";
    subj_act_2: "~ᾰ~ιδῳς", "~ᾰ~ιδωτον", "~ᾰ~ιδωτε";
    subj_act_3: "~ᾰ~ιδῳ", _, "~ᾰ~ιδωσῐ|~ᾰ~ιδωσῐν";
    subj_mid_1: "~ᾰ~ιδωμαι", _, "~ᾰ~ιδωμεθᾰ";
    subj_mid_2: "~ᾰ~ιδῳ", "~ᾰ~ιδωσθον", "~ᾰ~ιδωσθε";
    subj_mid_3: "~ᾰ~ιδωται", _, "~ᾰ~ιδωνται";
    subj_pass_1: _, _, "~ᾰ~ιδωμεθᾰ";
    subj_pass_2: _, _, "~ᾰ~ιδωσθε";
    subj_pass_3: "~ᾰ~ιδωται", _, "~ᾰ~ιδωνται";
    opt_act_1: "~ᾰ~ιδοιην", _, "~ᾰ~ιδοῑμεν|~ᾰ~ιδοιημεν";
    opt_act_2: "~ᾰ~ιδοιης", "~ᾰ~ιδοῑτον|~ᾰ~ιδοιητον", "~ᾰ~ιδοῑτε|~ᾰ~ιδοιητε";
    opt_act_3: "~ᾰ~ιδοιη", "~ᾰ~ιδοιτην|~ᾰ~ιδοιητην", "~ᾰ~ιδοῑεν|~ᾰ~ιδοιησᾰν";
    opt_mid_1: "~ᾰ~ιδοιμην", _, "~ᾰ~ιδοιμεθᾰ";
    opt_mid_2: "~ᾰ~ιδοῑο", "~ᾰ~ιδοῑσθον", "~ᾰ~ιδοῑσθε";
    opt_mid_3: "~ᾰ~ιδοῑτο", "~ᾰ~ιδοισθην", "~ᾰ~ιδοῑντο";
    opt_pass_1: _, _, "~ᾰ~ιδοιμεθᾰ";
    opt_pass_2: _, "~ᾰ~ιδοῑσθον", "~ᾰ~ιδοῑσθε";
    opt_pass_3: "~ᾰ~ιδοῑτο", "~ᾰ~ιδοισθην", "~ᾰ~ιδοῑντο";
    imp_act_2: "~ᾰ~ιδου", "~ᾰ~ιδοτον", "~ᾰ~ιδοτε";
    imp_act_3: "~ᾰ~ιδοτω", "~ᾰ~ιδοτων", "~ᾰ~ιδοντων";
    imp_mid_2: "~ᾰ~ιδοσο|~ᾰ~ιδου", _, "~ᾰ~ιδοσθε";
    imp_mid_3: "~ᾰ~ιδοσθω", "~ᾰ~ιδοσθων", "~ᾰ~ιδοσθων";
    imp_pass_2: "~ᾰ~ιδοσο|~ᾰ~ιδου", _, "~ᾰ~ιδοσθε";
    imp_pass_3: _, "~ᾰ~ιδοσθων", _;
    future_ind_act_1: "~ᾰ~ωσω", _, "~ᾰ~ωσομεν";
    future_ind_act_2: "~ᾰ~ωσεις", "~ᾰ~ωσετον", "~ᾰ~ωσετε";
    future_ind_act_3: "~ᾰ~ωσει", _, "~ᾰ~ωσουσῐ|~ᾰ~ωσουσῐν";
    future_ind_mid_1: "~ᾰ~ωσομαι", _, "~ᾰ~ωσομεθᾰ";
    future_ind_mid_2: "~ᾰ~ωσῃ|~ᾰ~ωσει", "~ᾰ~ωσεσθον", "~ᾰ~ωσεσθε";
    future_ind_mid_3: "~ᾰ~ωσεται", _, "~ᾰ~ωσονται";
    future_ind_pass_1: "~ᾰ~οθησομαι", _, "~ᾰ~οθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~οθησῃ|~ᾰ~οθησει", "~ᾰ~οθησεσθον", "~ᾰ~οθησεσθε";
    future_ind_pass_3: "~ᾰ~οθησεται", _, "~ᾰ~οθησονται";
    future_opt_act_1: "~ᾰ~ωσοιμῐ", _, "~ᾰ~ωσοιμεν";
    future_opt_act_2: "~ᾰ~ωσοις", "~ᾰ~ωσοιτον", "~ᾰ~ωσοιτε";
    future_opt_act_3: "~ᾰ~ωσοι", "~ᾰ~ωσοιτην", "~ᾰ~ωσοιεν";
    future_opt_mid_1: "~ᾰ~ωσοιμην", _, "~ᾰ~ωσοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ωσοιο", "~ᾰ~ωσοισθον", "~ᾰ~ωσοισθε";
    future_opt_mid_3: "~ᾰ~ωσοιτο", "~ᾰ~ωσοισθην", "~ᾰ~ωσοιντο";
    future_opt_pass_1: "~ᾰ~οθησοιμην", _, "~ᾰ~οθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~οθησοιο", "~ᾰ~οθησοισθον", "~ᾰ~οθησοισθε";
    future_opt_pass_3: "~ᾰ~οθησοιτο", "~ᾰ~οθησοισθην", "~ᾰ~οθησοιντο";
    aorist_ind_act_1: derived, _, "~ε~ομεν";
    aorist_ind_act_2: derived, "~ε~οτον", "~ε~οτε";
    aorist_ind_act_3: derived, "~ε~οτην", "~ε~οσᾰν";
    aorist_ind_mid_1: "~ε~ομην", _, "~ε~ομεθᾰ";
    aorist_ind_mid_2: "~ε~ου", "~ε~οσθον", "~ε~οσθε";
    aorist_ind_mid_3: "~ε~οτο", "~ε~οσθην", "~ε~οντο";
    aorist_ind_pass_1: "~ε~οθην", _, "~ε~οθημεν";
    aorist_ind_pass_2: "~ε~οθης", "~ε~οθητον", "~ε~οθητε";
    aorist_ind_pass_3: "~ε~οθη", "~ε~οθητην", "~ε~οθησᾰν";
    aorist_subj_act_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    aorist_subj_act_2: "~ᾰ~ῳς", "~ᾰ~ωτον", "~ᾰ~ωτε";
    aorist_subj_act_3: "~ᾰ~ῳ", _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~ῳ", "~ᾰ~ωσθον", "~ᾰ~ωσθε";
    aorist_subj_mid_3: "~ᾰ~ωται", _, "~ᾰ~ωνται";
    aorist_subj_pass_1: "~ᾰ~οθω", _, "~ᾰ~οθωμεν";
    aorist_subj_pass_2: "~ᾰ~οθῃς", "~ᾰ~οθητον", "~ᾰ~οθητε";
    aorist_subj_pass_3: "~ᾰ~οθῃ", _, "~ᾰ~οθωσῐ|~ᾰ~οθωσῐν";
    aorist_opt_act_1: "~ᾰ~οιην", _, "~ᾰ~οῑμεν|~ᾰ~οιημεν";
    aorist_opt_act_2: "~ᾰ~οιης", "~ᾰ~οῑτον|~ᾰ~οιητον", "~ᾰ~οῑτε|~ᾰ~οιητε";
    aorist_opt_act_3: "~ᾰ~οιη", "~ᾰ~οιτην|~ᾰ~οιητην", "~ᾰ~οῑεν|~ᾰ~οιησᾰν";
    aorist_opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~οῑο", "~ᾰ~οῑσθον", "~ᾰ~οῑσθε";
    aorist_opt_mid_3: "~ᾰ~οῑτο", "~ᾰ~οισθην", "~ᾰ~οῑντο";
    aorist_opt_pass_1: "~ᾰ~οθειην", _, "~ᾰ~οθεῑμεν|~ᾰ~οθειημεν";
    aorist_opt_pass_2: "~ᾰ~οθειης", "~ᾰ~οθεῑτον|~ᾰ~οθειητον", "~ᾰ~οθεῑτε|~ᾰ~οθειητε";
    aorist_opt_pass_3: "~ᾰ~οθειη", "~ᾰ~οθειτην|~ᾰ~οθειητην", "~ᾰ~οθεῑεν|~ᾰ~οθειησᾰν";
    aorist_imp_act_2: "~ᾰ~ος", "~ᾰ~οτον", "~ᾰ~οτε";
    aorist_imp_act_3: "~ᾰ~οτω", "~ᾰ~οτων", "~ᾰ~οντων";
    aorist_imp_mid_2: "~ᾰ~ου", "~ᾰ~οσθον", "~ᾰ~οσθε";
    aorist_imp_mid_3: "~ᾰ~οσθω", "~ᾰ~οσθων", derived;
    aorist_imp_pass_2: "~ᾰ~οθηθῐ", _, _;
    aorist_imp_pass_3: "~ᾰ~οθητω", "~ᾰ~οθητων", "~ᾰ~οθεντων";
    infinitive_pres: "~ᾰ~ιδοναι", "~ᾰ~ιδοσθαι", _;
    infinitive_fut: "~ᾰ~ωσειν", "~ᾰ~ωσεσθαι", "~ᾰ~οθησεσθαι";
    infinitive_aor: "~ᾰ~οῡναι", "~ᾰ~οσθαι", "~ᾰ~οθηναι";
    participle_pres: "~ᾰ~ιδους", "~ᾰ~ιδομενος", _;
    participle_fut: "~ᾰ~ωσων", "~ᾰ~ωσομενος", "~ᾰ~οθησομενος";
    participle_aor: "~ᾰ~ους", "~ᾰ~ομενος", "~ᾰ~οθεις";
    ),
    // Ἀνῐστημῐ: 1 words (ἀνῐστημῐ). Like Πᾰρῐστημῐ, except in 127 cells: future_ind_act_3 pl, future_ind_mid_2 sg, future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, future_ind_pass_2 du, ….
    conjugation!("ἀνῐστημῐ", 1, "~ῐ~ημῐ", like Πᾰρῐστημῐ;
    future_ind_act_3: _, _, "~ᾰ~ησουσῐ|~ᾰ~ησουσῐν";
    future_ind_mid_2: "~ᾰ~ησῃ|~ᾰ~ησει", _, _;
    future_ind_pass_1: "~ᾰ~ᾰθησομαι", _, "~ᾰ~ᾰθησομεθᾰ";
    future_ind_pass_2: "~ᾰ~ᾰθησῃ|~ᾰ~ᾰθησει", "~ᾰ~ᾰθησεσθον", "~ᾰ~ᾰθησεσθε";
    future_ind_pass_3: "~ᾰ~ᾰθησεται", _, "~ᾰ~ᾰθησονται";
    future_opt_pass_1: "~ᾰ~ᾰθησοιμην", _, "~ᾰ~ᾰθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰ~ᾰθησοιο", "~ᾰ~ᾰθησοισθον", "~ᾰ~ᾰθησοισθε";
    future_opt_pass_3: "~ᾰ~ᾰθησοιτο", "~ᾰ~ᾰθησοισθην", "~ᾰ~ᾰθησοιντο";
    aorist_subj_pass_3: _, _, "~ᾰ~ᾰθωσῐ|~ᾰ~ᾰθωσῐν";
    aorist_opt_pass_1: _, _, "~ᾰ~ᾰθεῑμεν|~ᾰ~ᾰθειημεν";
    aorist_opt_pass_2: _, "~ᾰ~ᾰθεῑτον|~ᾰ~ᾰθειητον", "~ᾰ~ᾰθεῑτε|~ᾰ~ᾰθειητε";
    aorist_opt_pass_3: _, "~ᾰ~ᾰθειτην|~ᾰ~ᾰθειητην", "~ᾰ~ᾰθεῑεν|~ᾰ~ᾰθειησᾰν";
    perfect_ind_act_1: "~ε~ᾰκᾰ|~ε~ηκᾰ", _, "~ε~ᾰκᾰμεν|~ε~ηκᾰμεν";
    perfect_ind_act_2: "~ε~ᾰκᾰς|~ε~ηκᾰς", "~ε~ᾰκᾰτον|~ε~ηκᾰτον", "~ε~ᾰκᾰτε|~ε~ηκᾰτε";
    perfect_ind_act_3: "~ε~ᾰκε|~ε~ᾰκεν|~ε~ηκε|~ε~ηκεν", "~ε~ᾰκᾰτον|~ε~ηκᾰτον", "~ε~ᾰκᾱσῐ|~ε~ᾰκᾱσῐν|~ε~ηκᾱσῐ|~ε~ηκᾱσῐν|~ε~εᾱσῐ";
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, derived;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "~ε~ᾰκω|~ε~ηκω", _, "~ε~ᾰκωμεν|~ε~ηκωμεν";
    perfect_subj_act_2: "~ε~ᾰκῃς|~ε~ηκῃς", "~ε~ᾰκητον|~ε~ηκητον", "~ε~ᾰκητε|~ε~ηκητε";
    perfect_subj_act_3: "~ε~ᾰκῃ|~ε~ηκῃ", "~ε~ᾰκητον|~ε~ηκητον", "~ε~ᾰκωσῐ|~ε~ᾰκωσῐν|~ε~ηκωσῐ|~ε~ηκωσῐν";
    perfect_opt_act_1: "~ε~ᾰκοιμῐ|~ε~ᾰκοιην|~ε~ηκοιμῐ|~ε~ηκοιην", _, "~ε~ᾰκοιμεν|~ε~ηκοιμεν";
    perfect_opt_act_2: "~ε~ᾰκοις|~ε~ᾰκοιης|~ε~ηκοις|~ε~ηκοιης", "~ε~ᾰκοιτον|~ε~ηκοιτον", "~ε~ᾰκοιτε|~ε~ηκοιτε";
    perfect_opt_act_3: "~ε~ᾰκοι|~ε~ᾰκοιη|~ε~ηκοι|~ε~ηκοιη", "~ε~ᾰκοιτην|~ε~ηκοιτην", "~ε~ᾰκοιεν|~ε~ηκοιεν";
    perfect_imp_act_2: "~ε~ᾰκε|~ε~ηκε", "~ε~ᾰκετον|~ε~ηκετον", "~ε~ᾰκετε|~ε~ηκετε";
    perfect_imp_act_3: "~ε~ᾰκετω|~ε~ηκετω", "~ε~ᾰκετων|~ε~ηκετων", "~ε~ᾰκοντων|~ε~ηκοντων";
    perfect_imp_mid_2: derived, _, _;
    perfect_imp_mid_3: derived, derived, _;
    perfect_imp_pass_2: derived, _, _;
    perfect_imp_pass_3: derived, derived, _;
    pluperfect_act_1: "~ε~ηκειν|~ε~ηκη", _, "~ε~ηκεμεν";
    pluperfect_act_2: "~ε~ηκεις|~ε~ηκης", "~ε~ηκετον", "~ε~ηκετε";
    pluperfect_act_3: "~ε~ηκει|~ε~ηκειν", "~ε~ηκετην", "~ε~ηκεσᾰν";
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
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
    infinitive_fut: _, _, "~ᾰ~ᾰθησεσθαι";
    infinitive_perf: "~ε~ᾰκεναι|~ε~ηκεναι", derived, _;
    infinitive_futp: derived, derived, _;
    participle_fut: _, _, "~ᾰ~ᾰθησομενος";
    participle_perf: "~ε~ᾰκως|~ε~ηκως", derived, _;
    participle_futp: derived, derived, _;
    ),
    // Δεῑ: 1 words (δεῑ). Like Οἰδᾰ, except in 31 cells: ind_act_1 sg, ind_act_3 sg, subj_act_3 sg, opt_act_3 sg, imperfect_act_3 sg, future_ind_act_3 sg, ….
    conjugation!("δεῑ", 1, "~εῑ", like Οἰδᾰ;
    ind_act_1: "~εῑ", _, _;
    ind_act_3: "~εει|~εῑ", _, _;
    subj_act_3: "~εῃ|~ῃ", _, _;
    opt_act_3: "~εοι", _, _;
    imperfect_act_3: "ἐ~εε|ἐ~εεν|ἐ~ει", _, _;
    future_ind_act_3: "~εησει", _, _;
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, derived;
    future_ind_mid_3: derived, _, derived;
    future_opt_act_3: "~εησοι", _, _;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, derived, derived;
    future_opt_mid_3: derived, derived, derived;
    aorist_ind_act_3: "ἐ~εησε|ἐ~εησεν", _, _;
    aorist_subj_act_3: "~εησῃ", _, _;
    aorist_opt_act_3: "~εησειε|~εησειεν", _, _;
    perfect_ind_act_3: "~εδεηκε|~εδεηκεν", _, _;
    infinitive_pres: "~εειν|~εῑν", _, _;
    infinitive_fut: "~εησειν", derived, _;
    infinitive_aor: "~εησαι", _, _;
    participle_fut: _, derived, _;
    ),
    // Ζωννῡμῐ: 1 words (ζωννῡμῐ). Like Ρρωννῡμῐ, except in 79 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("ζωννῡμῐ", 1, "~ννῡμῐ", like Ρρωννῡμῐ;
    imperfect_act_1: "ἐ~ννῡν|ἐ~ννυον", _, "ἐ~ννῠμεν|ἐ~ννυομεν";
    imperfect_act_2: "ἐ~ννῡς|ἐ~ννυες", "ἐ~ννῠτον|ἐ~ννυετον", "ἐ~ννῠτε|ἐ~ννυετε";
    imperfect_act_3: "ἐ~ννῡ|ἐ~ννυε|ἐ~ννυεν", "ἐ~ννῠτην|ἐ~ννυετην", "ἐ~ννῠσᾰν|ἐ~ννυον";
    imperfect_mid_1: "ἐ~ννῠμην|ἐ~ννυομην", _, "ἐ~ννῠμεθᾰ|ἐ~ννυομεθᾰ";
    imperfect_mid_2: "ἐ~ννῠσο|ἐ~ννυου", "ἐ~ννῠσθον|ἐ~ννυεσθον", "ἐ~ννῠσθε|ἐ~ννυεσθε";
    imperfect_mid_3: "ἐ~ννῠτο|ἐ~ννυετο", "ἐ~ννῠσθην|ἐ~ννυεσθην", "ἐ~ννῠντο|ἐ~ννυοντο";
    imperfect_pass_1: "ἐ~ννῠμην|ἐ~ννυομην", _, "ἐ~ννῠμεθᾰ|ἐ~ννυομεθᾰ";
    imperfect_pass_2: "ἐ~ννῠσο|ἐ~ννυου", "ἐ~ννῠσθον|ἐ~ννυεσθον", "ἐ~ννῠσθε|ἐ~ννυεσθε";
    imperfect_pass_3: "ἐ~ννῠτο|ἐ~ννυετο", "ἐ~ννῠσθην|ἐ~ννυεσθην", "ἐ~ννῠντο|ἐ~ννυοντο";
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    perfect_ind_act_1: "ἐ~κᾰ", _, "ἐ~κᾰμεν";
    perfect_ind_act_2: "ἐ~κᾰς", "ἐ~κᾰτον", "ἐ~κᾰτε";
    perfect_ind_act_3: "ἐ~κε|ἐ~κεν", _, "ἐ~κᾱσῐ|ἐ~κᾱσῐν";
    perfect_subj_act_1: "ἐ~κω", _, "ἐ~κωμεν";
    perfect_subj_act_2: "ἐ~κῃς", "ἐ~κητον", "ἐ~κητε";
    perfect_subj_act_3: "ἐ~κῃ", _, "ἐ~κωσῐ|ἐ~κωσῐν";
    perfect_opt_act_1: "ἐ~κοιμῐ|ἐ~κοιην", _, "ἐ~κοιμεν";
    perfect_opt_act_2: "ἐ~κοις|ἐ~κοιης", "ἐ~κοιτον", "ἐ~κοιτε";
    perfect_opt_act_3: "ἐ~κοι|ἐ~κοιη", "ἐ~κοιτην", "ἐ~κοιεν";
    perfect_imp_act_2: "ἐ~κε", "ἐ~κετον", "ἐ~κετε";
    perfect_imp_act_3: "ἐ~κετω", "ἐ~κετων", "ἐ~κοντων";
    pluperfect_act_1: "ἐ~κειν|ἐ~κη", _, "ἐ~κεμεν";
    pluperfect_act_2: "ἐ~κεις|ἐ~κης", "ἐ~κετον", "ἐ~κετε";
    pluperfect_act_3: "ἐ~κει|ἐ~κειν", "ἐ~κετην", "ἐ~κεσᾰν";
    infinitive_fut: _, _, derived;
    infinitive_perf: "ἐ~κεναι", _, _;
    participle_fut: _, _, derived;
    participle_perf: "ἐ~κως", _, _;
    ),
    // Κᾰθημαι: 1 words (κᾰθημαι). Like Ἐγκειμαι, except in 69 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("κᾰθημαι", 1, "~ᾰ~μαι", like Ἐγκειμαι;
    ind_act_1: "~ᾰ~μαι", _, _;
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, derived, derived;
    ind_mid_3: derived, _, derived;
    subj_mid_1: derived, _, derived;
    subj_mid_2: derived, derived, derived;
    subj_mid_3: derived, _, derived;
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, derived;
    opt_mid_3: derived, derived, derived;
    imp_mid_2: derived, _, derived;
    imp_mid_3: derived, derived, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, derived;
    imperfect_mid_3: derived, derived, derived;
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, derived;
    future_ind_mid_3: derived, _, derived;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, derived, derived;
    future_opt_mid_3: derived, derived, derived;
    pluperfect_mid_1: "ἐ~α~μην|~α~μην", _, "ἐ~α~μεθᾰ|~α~μεθᾰ";
    pluperfect_mid_2: "ἐ~α~σο|~α~σο", "ἐ~α~σθον|~α~σθον", "ἐ~α~σθε|~α~σθε";
    pluperfect_mid_3: "ἐ~α~το|~α~στο|~α~το", "ἐ~α~σθην|~α~σθην", "ἐ~α~ντο|~α~ντο";
    pluperfect_pass_1: "ἐ~α~μην|~α~μην", _, "ἐ~α~μεθᾰ|~α~μεθᾰ";
    pluperfect_pass_2: "ἐ~α~σο|~α~σο", "ἐ~α~σθον|~α~σθον", "ἐ~α~σθε|~α~σθε";
    pluperfect_pass_3: "ἐ~α~το|~α~στο|~α~το", "ἐ~α~σθην|~α~σθην", "ἐ~α~ντο|~α~ντο";
    infinitive_fut: _, derived, _;
    participle_fut: _, derived, _;
    ),
    // Ὀμνῡμῐ: 1 words (ὀμνῡμῐ). Like Ὀρνῡμῐ, except in 210 cells: imp_act_2 sg, imp_mid_2 sg, imp_pass_2 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, ….
    conjugation!("ὀμνῡμῐ", 1, "ὀ~νῡμῐ", like Ὀρνῡμῐ;
    imp_act_2: "ὀ~νῡ", _, _;
    imp_mid_2: "ὀ~νῠσο", _, _;
    imp_pass_2: derived, _, _;
    future_ind_act_1: "ὀ~ω", _, "ὀ~οῡμεν";
    future_ind_act_2: "ὀ~εῑς", "ὀ~εῑτον", "ὀ~εῑτε";
    future_ind_act_3: "ὀ~εῑ", _, "ὀ~οῡσῐ|ὀ~οῡσῐν";
    future_ind_mid_1: "ὀ~οῡμαι", _, "ὀ~ουμεθᾰ";
    future_ind_mid_2: "ὀ~εῑ|ὀ~ῃ", "ὀ~εῑσθον", "ὀ~εῑσθε";
    future_ind_mid_3: "ὀ~εῑται", _, "ὀ~οῡνται";
    future_ind_pass_1: "ὀ~οσθησομαι", _, "ὀ~οσθησομεθᾰ";
    future_ind_pass_2: "ὀ~οσθησῃ|ὀ~οσθησει", "ὀ~οσθησεσθον", "ὀ~οσθησεσθε";
    future_ind_pass_3: "ὀ~οσθησεται", _, "ὀ~οσθησονται";
    future_opt_act_1: "ὀ~οιην|ὀ~οῑμῐ", _, "ὀ~οῑμεν|ὀ~οιημεν";
    future_opt_act_2: "ὀ~οιης|ὀ~οῑς", "ὀ~οῑτον|ὀ~οιητον", "ὀ~οῑτε|ὀ~οιητε";
    future_opt_act_3: "ὀ~οιη|ὀ~οῑ", "ὀ~οιτην|ὀ~οιητην", "ὀ~οῑεν|ὀ~οιησᾰν";
    future_opt_mid_1: "ὀ~οιμην", _, "ὀ~οιμεθᾰ";
    future_opt_mid_2: "ὀ~οῑο", "ὀ~οῑσθον", "ὀ~οῑσθε";
    future_opt_mid_3: "ὀ~οῑτο", "ὀ~οισθην", "ὀ~οῑντο";
    future_opt_pass_1: "ὀ~οσθησοιμην", _, "ὀ~οσθησοιμεθᾰ";
    future_opt_pass_2: "ὀ~οσθησοιο", "ὀ~οσθησοισθον", "ὀ~οσθησοισθε";
    future_opt_pass_3: "ὀ~οσθησοιτο", "ὀ~οσθησοισθην", "ὀ~οσθησοιντο";
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_ind_mid_1: derived, _, derived;
    aorist_ind_mid_2: derived, derived, derived;
    aorist_ind_mid_3: derived, derived, derived;
    aorist_ind_pass_1: "ὠ~οσθην", _, "ὠ~οσθημεν";
    aorist_ind_pass_2: "ὠ~οσθης", "ὠ~οσθητον", "ὠ~οσθητε";
    aorist_ind_pass_3: "ὠ~οσθη", "ὠ~οσθητην", "ὠ~οσθησᾰν";
    aorist_subj_act_1: derived, _, derived;
    aorist_subj_act_2: derived, derived, derived;
    aorist_subj_act_3: derived, _, derived;
    aorist_subj_mid_1: derived, _, derived;
    aorist_subj_mid_2: derived, derived, derived;
    aorist_subj_mid_3: derived, _, derived;
    aorist_subj_pass_1: "ὀ~οσθω", _, "ὀ~οσθωμεν";
    aorist_subj_pass_2: "ὀ~οσθῃς", "ὀ~οσθητον", "ὀ~οσθητε";
    aorist_subj_pass_3: "ὀ~οσθῃ", _, "ὀ~οσθωσῐ|ὀ~οσθωσῐν";
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_mid_1: derived, _, derived;
    aorist_opt_mid_2: derived, derived, derived;
    aorist_opt_mid_3: derived, derived, derived;
    aorist_opt_pass_1: "ὀ~οσθειην", _, "ὀ~οσθεῑμεν|ὀ~οσθειημεν";
    aorist_opt_pass_2: "ὀ~οσθειης", "ὀ~οσθεῑτον|ὀ~οσθειητον", "ὀ~οσθεῑτε|ὀ~οσθειητε";
    aorist_opt_pass_3: "ὀ~οσθειη", "ὀ~οσθειτην|ὀ~οσθειητην", "ὀ~οσθεῑεν|ὀ~οσθειησᾰν";
    aorist_imp_act_2: derived, derived, derived;
    aorist_imp_act_3: derived, derived, derived;
    aorist_imp_mid_2: derived, derived, derived;
    aorist_imp_mid_3: derived, derived, _;
    aorist_imp_pass_2: "ὀ~οσθητῐ", _, _;
    aorist_imp_pass_3: "ὀ~οσθητω", "ὀ~οσθητων", "ὀ~οσθεντων";
    perfect_ind_act_1: "ὀ~ωμοκᾰ", _, "ὀ~ωμοκᾰμεν";
    perfect_ind_act_2: "ὀ~ωμοκᾰς", "ὀ~ωμοκᾰτον", "ὀ~ωμοκᾰτε";
    perfect_ind_act_3: "ὀ~ωμοκε|ὀ~ωμοκεν", _, "ὀ~ωμοκᾱσῐ|ὀ~ωμοκᾱσῐν";
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, derived;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "ὀ~ωμοκω", _, "ὀ~ωμοκωμεν";
    perfect_subj_act_2: "ὀ~ωμοκῃς", "ὀ~ωμοκητον", "ὀ~ωμοκητε";
    perfect_subj_act_3: "ὀ~ωμοκῃ", _, "ὀ~ωμοκωσῐ|ὀ~ωμοκωσῐν";
    perfect_opt_act_1: "ὀ~ωμοκοιμῐ|ὀ~ωμοκοιην", _, "ὀ~ωμοκοιμεν";
    perfect_opt_act_2: "ὀ~ωμοκοις|ὀ~ωμοκοιης", "ὀ~ωμοκοιτον", "ὀ~ωμοκοιτε";
    perfect_opt_act_3: "ὀ~ωμοκοι|ὀ~ωμοκοιη", "ὀ~ωμοκοιτην", "ὀ~ωμοκοιεν";
    perfect_imp_act_2: "ὀ~ωμοκε", "ὀ~ωμοκετον", "ὀ~ωμοκετε";
    perfect_imp_act_3: "ὀ~ωμοκετω", "ὀ~ωμοκετων", "ὀ~ωμοκοντων";
    perfect_imp_mid_2: derived, _, _;
    perfect_imp_mid_3: derived, derived, _;
    perfect_imp_pass_2: derived, _, _;
    perfect_imp_pass_3: derived, derived, _;
    pluperfect_act_1: "ὠ~ωμοκειν|ὠ~ωμοκη|ὀ~ωμοκειν|ὀ~ωμοκη", _, "ὠ~ωμοκεμεν|ὀ~ωμοκεμεν";
    pluperfect_act_2: "ὠ~ωμοκεις|ὠ~ωμοκης|ὀ~ωμοκεις|ὀ~ωμοκης", "ὠ~ωμοκετον|ὀ~ωμοκετον", "ὠ~ωμοκετε|ὀ~ωμοκετε";
    pluperfect_act_3: "ὠ~ωμοκει|ὠ~ωμοκειν|ὀ~ωμοκει|ὀ~ωμοκειν", "ὠ~ωμοκετην|ὀ~ωμοκετην", "ὠ~ωμοκεσᾰν|ὀ~ωμοκεσᾰν";
    pluperfect_mid_1: "ὀ~ωμοσμην", _, "ὀ~ωμοσμεθᾰ";
    pluperfect_mid_2: "ὀ~ωμοσο", "ὀ~ωμοσθον", "ὀ~ωμοσθε";
    pluperfect_mid_3: "ὀ~ωμοστο", "ὀ~ωμοσθην", _;
    pluperfect_pass_1: _, _, "ὀ~ωμοσμεθᾰ";
    pluperfect_pass_2: _, _, "ὀ~ωμοσθε";
    pluperfect_pass_3: _, "ὀ~ωμοσθην", "ὀ~ωμοσᾰτο";
    infinitive_fut: "ὀ~εῑν", "ὀ~εῑσθαι", "ὀ~οσθησεσθαι";
    infinitive_aor: derived, derived, "ὀ~οσθηναι";
    infinitive_perf: "ὀ~ωμοκεναι", derived, _;
    participle_fut: "ὀ~ων", "ὀ~ουμενος", "ὀ~οσθησομενος";
    participle_aor: derived, derived, "ὀ~οσθεις";
    participle_perf: "ὀ~ωμοκως", derived, _;
    ),
    // Ἐφῐστημῐ: 1 words (ἐφῐστημῐ).
    conjugation!("ἐφῐστημῐ", 1, "~φῐ~ημῐ";
    ind_act_1: "~φῐ~ημῐ|~φε~ηκω", _, "~φῐ~ᾰμεν|~φε~ηκομεν";
    ind_act_2: "~φῐ~ης|~φε~ηκεις", "~φῐ~ᾰτον|~φε~ηκετον", "~φῐ~ᾰτε|~φε~ηκετε";
    ind_act_3: "~φῐ~ησῐ|~φῐ~ησῐν|~φε~ηκει", "~φῐ~ᾰτον|~φε~ηκετον", "~φῐ~ᾱσῐ|~φῐ~ᾱσῐν|~φε~ηκουσῐ|~φε~ηκουσῐν";
    ind_mid_1: "~φῐ~ᾰμαι|~φε~ηκομαι", _, "~φῐ~ᾰμεθᾰ|~φε~ηκομεθᾰ";
    ind_mid_2: "~φῐ~ᾰσαι|~φε~ηκῃ|~φε~ηκει", "~φῐ~ᾰσθον|~φε~ηκεσθον", "~φῐ~ᾰσθε|~φε~ηκεσθε";
    ind_mid_3: "~φῐ~ᾰται|~φε~ηκεται", "~φῐ~ᾰσθον|~φε~ηκεσθον", "~φῐ~ᾰνται|~φε~ηκονται";
    ind_pass_1: "~φῐ~ᾰμαι|~φε~ηκομαι", _, "~φῐ~ᾰμεθᾰ|~φε~ηκομεθᾰ";
    ind_pass_2: "~φῐ~ᾰσαι|~φε~ηκῃ|~φε~ηκει", "~φῐ~ᾰσθον|~φε~ηκεσθον", "~φῐ~ᾰσθε|~φε~ηκεσθε";
    ind_pass_3: "~φῐ~ᾰται|~φε~ηκεται", "~φῐ~ᾰσθον|~φε~ηκεσθον", "~φῐ~ᾰνται|~φε~ηκονται";
    subj_act_1: "~φῐ~ω|~φε~ηκω", _, "~φῐ~ωμεν|~φε~ηκωμεν";
    subj_act_2: "~φῐ~ῃς|~φε~ηκῃς", "~φῐ~ητον|~φε~ηκητον", "~φῐ~ητε|~φε~ηκητε";
    subj_act_3: "~φῐ~ῃ|~φε~ηκῃ", "~φῐ~ητον|~φε~ηκητον", "~φῐ~ωσῐ|~φῐ~ωσῐν|~φε~ηκωσῐ|~φε~ηκωσῐν";
    subj_mid_1: "~φῐ~ωμαι|~φε~ηκωμαι", _, "~φῐ~ωμεθᾰ|~φε~ηκωμεθᾰ";
    subj_mid_2: "~φῐ~ῃ|~φε~ηκῃ", "~φῐ~ησθον|~φε~ηκησθον", "~φῐ~ησθε|~φε~ηκησθε";
    subj_mid_3: "~φῐ~ηται|~φε~ηκηται", "~φῐ~ησθον|~φε~ηκησθον", "~φῐ~ωνται|~φε~ηκωνται";
    subj_pass_1: "~φῐ~ωμαι|~φε~ηκωμαι", _, "~φῐ~ωμεθᾰ|~φε~ηκωμεθᾰ";
    subj_pass_2: "~φῐ~ῃ|~φε~ηκῃ", "~φῐ~ησθον|~φε~ηκησθον", "~φῐ~ησθε|~φε~ηκησθε";
    subj_pass_3: "~φῐ~ηται|~φε~ηκηται", "~φῐ~ησθον|~φε~ηκησθον", "~φῐ~ωνται|~φε~ηκωνται";
    opt_act_1: "~φῐ~αιην|~φε~ηκοιμῐ", _, "~φῐ~αῑμεν|~φῐ~αιημεν|~φε~ηκοιμεν";
    opt_act_2: "~φῐ~αιης|~φε~ηκοις", "~φῐ~αῑτον|~φῐ~αιητον|~φε~ηκοιτον", "~φῐ~αῑτε|~φῐ~αιητε|~φε~ηκοιτε";
    opt_act_3: "~φῐ~αιη|~φε~ηκοι", "~φῐ~αιτην|~φῐ~αιητην|~φε~ηκοιτην", "~φῐ~αῑεν|~φῐ~αιησᾰν|~φε~ηκοιεν";
    opt_mid_1: "~φῐ~αιμην|~φε~ηκοιμην", _, "~φῐ~αιμεθᾰ|~φε~ηκοιμεθᾰ";
    opt_mid_2: "~φῐ~αιο|~φε~ηκοιο", "~φῐ~αισθον|~φε~ηκοισθον", "~φῐ~αισθε|~φε~ηκοισθε";
    opt_mid_3: "~φῐ~αιτο|~φε~ηκοιτο", "~φῐ~αισθην|~φε~ηκοισθην", "~φῐ~αιντο|~φε~ηκοιντο";
    opt_pass_1: "~φῐ~αιμην|~φε~ηκοιμην", _, "~φῐ~αιμεθᾰ|~φε~ηκοιμεθᾰ";
    opt_pass_2: "~φῐ~αιο|~φε~ηκοιο", "~φῐ~αισθον|~φε~ηκοισθον", "~φῐ~αισθε|~φε~ηκοισθε";
    opt_pass_3: "~φῐ~αιτο|~φε~ηκοιτο", "~φῐ~αισθην|~φε~ηκοισθην", "~φῐ~αιντο|~φε~ηκοιντο";
    imp_act_2: "~φῐ~η|~φε~ηκε", "~φῐ~ᾰτον|~φε~ηκετον", "~φῐ~ᾰτε|~φε~ηκετε";
    imp_act_3: "~φῐ~ᾰτω|~φε~ηκετω", "~φῐ~ᾰτων|~φε~ηκετων", "~φῐ~ᾰντων|~φε~ηκοντων";
    imp_mid_2: "~φῐ~ᾰσο|~φῐ~ω|~φε~ηκου", "~φῐ~ᾰσθον|~φε~ηκεσθον", "~φῐ~ᾰσθε|~φε~ηκεσθε";
    imp_mid_3: "~φῐ~ᾰσθω|~φε~ηκεσθω", "~φῐ~ᾰσθων|~φε~ηκεσθων", "~φῐ~ᾰσθων|~φε~ηκεσθων";
    imp_pass_2: "~φῐ~ᾰσο|~φῐ~ω|~φε~ηκου", "~φῐ~ᾰσθον|~φε~ηκεσθον", "~φῐ~ᾰσθε|~φε~ηκεσθε";
    imp_pass_3: "~φῐ~ᾰσθω|~φε~ηκεσθω", "~φῐ~ᾰσθων|~φε~ηκεσθων", "~φῐ~ᾰσθων|~φε~ηκεσθων";
    imperfect_act_1: "~φῑ~ην|~φι~ασκον", _, "~φῑ~ᾰμεν|~φι~ασκομεν";
    imperfect_act_2: "~φῑ~ης|~φι~ασκες", "~φῑ~ᾰτον|~φι~ασκετον", "~φῑ~ᾰτε|~φι~ασκετε";
    imperfect_act_3: "~φῑ~η|~φι~ασκε|~φι~ασκεν", "~φῑ~ᾰτην|~φι~ασκετην", "~φῑ~ᾰσᾰν|~φι~ασκον";
    imperfect_mid_1: "~φῑ~ᾰμην|~φι~ασκομην", _, "~φῑ~ᾰμεθᾰ|~φι~ασκομεθᾰ";
    imperfect_mid_2: "~φῑ~ᾰσο|~φι~ασκου", "~φῑ~ᾰσθον|~φι~ασκεσθον", "~φῑ~ᾰσθε|~φι~ασκεσθε";
    imperfect_mid_3: "~φῑ~ᾰτο|~φι~ασκετο", "~φῑ~ᾰσθην|~φι~ασκεσθην", "~φῑ~ᾰντο|~φι~ασκοντο";
    imperfect_pass_1: "~φῑ~ᾰμην|~φι~ασκομην", _, "~φῑ~ᾰμεθᾰ|~φι~ασκομεθᾰ";
    imperfect_pass_2: "~φῑ~ᾰσο|~φι~ασκου", "~φῑ~ᾰσθον|~φι~ασκεσθον", "~φῑ~ᾰσθε|~φι~ασκεσθε";
    imperfect_pass_3: "~φῑ~ᾰτο|~φι~ασκετο", "~φῑ~ᾰσθην|~φι~ασκεσθην", "~φῑ~ᾰντο|~φι~ασκοντο";
    future_ind_act_1: "~πῐ~ᾱσεω", _, "~πῐ~ᾱσεομες";
    future_ind_act_2: "~πῐ~ᾱσεῑς", "~πῐ~ᾱσεῑτον", "~πῐ~ᾱσεῑτε";
    future_ind_act_3: "~πῐ~ᾱσεῑ", _, "~πῐ~ᾱσεοντῐ";
    future_ind_mid_1: "~πῐ~ᾱσεομαι", _, "~πῐ~ᾱσεομεθᾰ";
    future_ind_mid_2: "~πῐ~ᾱσῃ", "~πῐ~ᾱσεῑσθον", "~πῐ~ᾱσεῑσθε";
    future_ind_mid_3: "~πῐ~ᾱσεῑται", _, "~πῐ~ᾱσεονται";
    future_opt_act_1: "~πῐ~ᾱσεοιμῐ", _, "~πῐ~ᾱσεοιμες";
    future_opt_act_2: "~πῐ~ᾱσεοις", "~πῐ~ᾱσεοιτον", "~πῐ~ᾱσεοιτε";
    future_opt_act_3: "~πῐ~ᾱσεοι", "~πῐ~ᾱσεοιτᾱν", "~πῐ~ᾱσεοιεν";
    future_opt_mid_1: "~πῐ~ᾱσεοιμᾱν", _, "~πῐ~ᾱσεοιμεθᾰ";
    future_opt_mid_2: "~πῐ~ᾱσεοιο", "~πῐ~ᾱσεοισθον", "~πῐ~ᾱσεοισθε";
    future_opt_mid_3: "~πῐ~ᾱσεοιτο", "~πῐ~ᾱσεοισθᾱν", "~πῐ~ᾱσεοιντο";
    aorist_ind_act_1: "~πε~ησᾰ|~πε~ην|~πῐ~ᾰσκον", _, "~πε~ησᾰμεν|~πε~ημεν|~πῐ~ᾰσκομεν";
    aorist_ind_act_2: "~πε~ησᾰς|~πε~ης|~πῐ~ᾰσκες", "~πε~ησᾰτον|~πε~ητον|~πῐ~ᾰσκετον", "~πε~ησᾰτε|~πε~ητε|~πῐ~ᾰσκετε";
    aorist_ind_act_3: "~πε~ησε|~πε~ησεν|~πε~η|~πῐ~ᾰσκε|~πῐ~ᾰσκεν", "~πε~ησᾰτην|~πε~ητην|~πῐ~ᾰσκετην", "~πε~ησᾰν|~πῐ~ᾰσκον";
    aorist_ind_mid_1: "~πε~ησᾰμην|~πε~ᾰμην|~πῐ~ᾰσκομην", _, "~πε~ησᾰμεθᾰ|~πε~ᾰμεθᾰ|~πῐ~ᾰσκομεθᾰ";
    aorist_ind_mid_2: "~πε~ησω|~πε~ω|~πῐ~ᾰσκου", "~πε~ησᾰσθον|~πε~ᾰσθον|~πῐ~ᾰσκεσθον", "~πε~ησᾰσθε|~πε~ᾰσθε|~πῐ~ᾰσκεσθε";
    aorist_ind_mid_3: "~πε~ησᾰτο|~πε~ᾰτο|~πῐ~ᾰσκετο", "~πε~ησᾰσθην|~πε~ᾰσθην|~πῐ~ᾰσκεσθην", "~πε~ησᾰντο|~πε~ᾰντο|~πῐ~ᾰσκοντο";
    aorist_ind_pass_1: "~πε~ᾰθην|~πε~ην", _, "~πε~ᾰθημεν|~πε~ημεν";
    aorist_ind_pass_2: "~πε~ᾰθης|~πε~ης", "~πε~ᾰθητον|~πε~ητον", "~πε~ᾰθητε|~πε~ητε";
    aorist_ind_pass_3: "~πε~ᾰθη|~πε~η", "~πε~ᾰθητην|~πε~ητην", "~πε~ᾰθησᾰν|~πε~ησᾰν";
    aorist_subj_act_1: "~πῐ~ησω|~πῐ~ω|~πῐ~ᾰσκω", _, "~πῐ~ησωμεν|~πῐ~ωμεν|~πῐ~ᾰσκωμεν";
    aorist_subj_act_2: "~πῐ~ησῃς|~πῐ~ῃς|~πῐ~ᾰσκῃς", "~πῐ~ησητον|~πῐ~ητον|~πῐ~ᾰσκητον", "~πῐ~ησητε|~πῐ~ητε|~πῐ~ᾰσκητε";
    aorist_subj_act_3: "~πῐ~ησῃ|~πῐ~ῃ|~πῐ~ᾰσκῃ", "~πῐ~ησητον|~πῐ~ητον|~πῐ~ᾰσκητον", "~πῐ~ησωσῐ|~πῐ~ησωσῐν|~πῐ~ωσῐ|~πῐ~ωσῐν|~πῐ~ᾰσκωσῐ|~πῐ~ᾰσκωσῐν";
    aorist_subj_mid_1: "~πῐ~ησωμαι|~πῐ~ωμαι|~πῐ~ᾰσκωμαι", _, "~πῐ~ησωμεθᾰ|~πῐ~ωμεθᾰ|~πῐ~ᾰσκωμεθᾰ";
    aorist_subj_mid_2: "~πῐ~ησῃ|~πῐ~ῃ|~πῐ~ᾰσκῃ", "~πῐ~ησησθον|~πῐ~ησθον|~πῐ~ᾰσκησθον", "~πῐ~ησησθε|~πῐ~ησθε|~πῐ~ᾰσκησθε";
    aorist_subj_mid_3: "~πῐ~ησηται|~πῐ~ηται|~πῐ~ᾰσκηται", "~πῐ~ησησθον|~πῐ~ησθον|~πῐ~ᾰσκησθον", "~πῐ~ησωνται|~πῐ~ωνται|~πῐ~ᾰσκωνται";
    aorist_subj_pass_1: "~πῐ~ᾰθω|~πῐ~ω", _, "~πῐ~ᾰθωμεν|~πῐ~ωμεν";
    aorist_subj_pass_2: "~πῐ~ᾰθῃς|~πῐ~ῃς", "~πῐ~ᾰθητον|~πῐ~ητον", "~πῐ~ᾰθητε|~πῐ~ητε";
    aorist_subj_pass_3: "~πῐ~ᾰθῃ|~πῐ~ῃ", "~πῐ~ᾰθητον|~πῐ~ητον", "~πῐ~ᾰθωσῐ|~πῐ~ᾰθωσῐν|~πῐ~ωσῐ|~πῐ~ωσῐν";
    aorist_opt_act_1: "~πῐ~ησαιμῐ|~πῐ~αιην|~πῐ~ᾰσκοιμῐ", _, "~πῐ~ησαιμεν|~πῐ~αῑμεν|~πῐ~αιημεν|~πῐ~ᾰσκοιμεν";
    aorist_opt_act_2: "~πῐ~ησειᾰς|~πῐ~ησαις|~πῐ~αιης|~πῐ~ᾰσκοις", "~πῐ~ησαιτον|~πῐ~αῑτον|~πῐ~αιητον|~πῐ~ᾰσκοιτον", "~πῐ~ησαιτε|~πῐ~αῑτε|~πῐ~αιητε|~πῐ~ᾰσκοιτε";
    aorist_opt_act_3: "~πῐ~ησειε|~πῐ~ησειεν|~πῐ~ησαι|~πῐ~αιη|~πῐ~ᾰσκοι", "~πῐ~ησαιτην|~πῐ~αιτην|~πῐ~αιητην|~πῐ~ᾰσκοιτην", "~πῐ~ησειᾰν|~πῐ~ησαιεν|~πῐ~αῑεν|~πῐ~αιησᾰν|~πῐ~ᾰσκοιεν";
    aorist_opt_mid_1: "~πῐ~ησαιμην|~πῐ~αιμην|~πῐ~ᾰσκοιμην", _, "~πῐ~ησαιμεθᾰ|~πῐ~αιμεθᾰ|~πῐ~ᾰσκοιμεθᾰ";
    aorist_opt_mid_2: "~πῐ~ησαιο|~πῐ~αῑο|~πῐ~ᾰσκοιο", "~πῐ~ησαισθον|~πῐ~αῑσθον|~πῐ~ᾰσκοισθον", "~πῐ~ησαισθε|~πῐ~αῑσθε|~πῐ~ᾰσκοισθε";
    aorist_opt_mid_3: "~πῐ~ησαιτο|~πῐ~αῑτο|~πῐ~ᾰσκοιτο", "~πῐ~ησαισθην|~πῐ~αισθην|~πῐ~ᾰσκοισθην", "~πῐ~ησαιντο|~πῐ~αῑντο|~πῐ~ᾰσκοιντο";
    aorist_opt_pass_1: "~πῐ~ᾰθειην|~πῐ~ειην", _, "~πῐ~ᾰθεῑμεν|~πῐ~ᾰθειημεν|~πῐ~εῑμεν|~πῐ~ειημεν";
    aorist_opt_pass_2: "~πῐ~ᾰθειης|~πῐ~ειης", "~πῐ~ᾰθεῑτον|~πῐ~ᾰθειητον|~πῐ~εῑτον|~πῐ~ειητον", "~πῐ~ᾰθεῑτε|~πῐ~ᾰθειητε|~πῐ~εῑτε|~πῐ~ειητε";
    aorist_opt_pass_3: "~πῐ~ᾰθειη|~πῐ~ειη", "~πῐ~ᾰθειτην|~πῐ~ᾰθειητην|~πῐ~ειτην|~πῐ~ειητην", "~πῐ~ᾰθεῑεν|~πῐ~ᾰθειησᾰν|~πῐ~εῑεν|~πῐ~ειησᾰν";
    aorist_imp_act_2: "~πῐ~ησον|~πῐ~ηθῐ|~πῐ~ᾰσκε", "~πῐ~ησᾰτον|~πῐ~ητον|~πῐ~ᾰσκετον", "~πῐ~ησᾰτε|~πῐ~ητε|~πῐ~ᾰσκετε";
    aorist_imp_act_3: "~πῐ~ησᾰτω|~πῐ~ητω|~πῐ~ᾰσκετω", "~πῐ~ησᾰτων|~πῐ~ητων|~πῐ~ᾰσκετων", "~πῐ~ησᾰντων|~πῐ~ᾰντων|~πῐ~ᾰσκοντων";
    aorist_imp_mid_2: "~πῐ~ησαι|~πῐ~ω|~πῐ~ᾰσκοῡ", "~πῐ~ησᾰσθον|~πῐ~ᾰσθον|~πῐ~ᾰσκεσθον", "~πῐ~ησᾰσθε|~πῐ~ᾰσθε|~πῐ~ᾰσκεσθε";
    aorist_imp_mid_3: "~πῐ~ησᾰσθω|~πῐ~ᾰσθω|~πῐ~ᾰσκεσθω", "~πῐ~ησᾰσθων|~πῐ~ᾰσθων|~πῐ~ᾰσκεσθων", "~πῐ~ησᾰσθων|~πῐ~ᾰσθων|~πῐ~ᾰσκεσθων";
    aorist_imp_pass_2: "~πῐ~ᾰθητῐ|~πῐ~ηθῐ", "~πῐ~ᾰθητον|~πῐ~ητον", "~πῐ~ᾰθητε|~πῐ~ητε";
    aorist_imp_pass_3: "~πῐ~ᾰθητω|~πῐ~ητω", "~πῐ~ᾰθητων|~πῐ~ητων", "~πῐ~ᾰθεντων|~πῐ~εντων";
    perfect_ind_act_1: "~φε~ᾰκᾰ|~φε~ηκᾰ", _, "~φε~ᾰκᾰμεν|~φε~ηκᾰμεν|~φε~ᾰμεν";
    perfect_ind_act_2: "~φε~ᾰκᾰς|~φε~ηκᾰς", "~φε~ᾰκᾰτον|~φε~ηκᾰτον|~φε~ᾰτον", "~φε~ᾰκᾰτε|~φε~ηκᾰτε|~φε~ᾰτε";
    perfect_ind_act_3: "~φε~ᾰκε|~φε~ᾰκεν|~φε~ηκε|~φε~ηκεν", "~φε~ᾰκᾰτον|~φε~ηκᾰτον|~φε~ᾰτον", "~φε~ᾰκᾱσῐ|~φε~ᾰκᾱσῐν|~φε~ηκᾱσῐ|~φε~ηκᾱσῐν|~φε~ᾱσῐ|~φε~ᾱσῐν";
    perfect_ind_mid_1: "~φε~ᾰμαι", _, "~φε~ᾰμεθᾰ";
    perfect_ind_mid_2: "~φε~ᾰσαι", "~φε~ᾰσθον", "~φε~ᾰσθε";
    perfect_ind_mid_3: "~φε~ᾰται", _, "~φε~ᾰνται";
    perfect_ind_pass_1: _, _, "~φε~ᾰμεθᾰ";
    perfect_ind_pass_2: _, _, "~φε~ᾰσθε";
    perfect_subj_act_1: "~φε~ᾰκω|~φε~ηκω|~φε~ω", _, "~φε~ᾰκωμεν|~φε~ηκωμεν|~φε~ωμεν";
    perfect_subj_act_2: "~φε~ᾰκῃς|~φε~ηκῃς|~φε~ῃς", "~φε~ᾰκητον|~φε~ηκητον|~φε~ητον", "~φε~ᾰκητε|~φε~ηκητε|~φε~ητε";
    perfect_subj_act_3: "~φε~ᾰκῃ|~φε~ηκῃ|~φε~ῃ", "~φε~ᾰκητον|~φε~ηκητον|~φε~ητον", "~φε~ᾰκωσῐ|~φε~ᾰκωσῐν|~φε~ηκωσῐ|~φε~ηκωσῐν|~φε~ωσῐ|~φε~ωσῐν";
    perfect_opt_act_1: "~φε~ᾰκοιμῐ|~φε~ᾰκοιην|~φε~ηκοιμῐ|~φε~ηκοιην|~φε~αιην", _, "~φε~ᾰκοιμεν|~φε~ηκοιμεν|~φε~αῑμεν|~φε~αιημεν";
    perfect_opt_act_2: "~φε~ᾰκοις|~φε~ᾰκοιης|~φε~ηκοις|~φε~ηκοιης|~φε~αιης", "~φε~ᾰκοιτον|~φε~ηκοιτον|~φε~αῑτον|~φε~αιητον", "~φε~ᾰκοιτε|~φε~ηκοιτε|~φε~αῑτε|~φε~αιητε";
    perfect_opt_act_3: "~φε~ᾰκοι|~φε~ᾰκοιη|~φε~ηκοι|~φε~ηκοιη|~φε~αιη", "~φε~ᾰκοιτην|~φε~ηκοιτην|~φε~αιτην|~φε~αιητην", "~φε~ᾰκοιεν|~φε~ηκοιεν|~φε~αῑεν|~φε~αιησᾰν";
    perfect_imp_act_2: "~φε~ᾰκε|~φε~ηκε|~φε~ᾰθῐ", "~φε~ᾰκετον|~φε~ηκετον|~φε~ᾰτον", "~φε~ᾰκετε|~φε~ηκετε|~φε~ᾰτε";
    perfect_imp_act_3: "~φε~ᾰκετω|~φε~ηκετω|~φε~ᾰτω", "~φε~ᾰκετων|~φε~ηκετων|~φε~ᾰτων", "~φε~ᾰκοντων|~φε~ηκοντων|~φε~ᾰντων";
    perfect_imp_mid_2: "~φε~ᾰσο", _, _;
    perfect_imp_mid_3: "~φε~ᾰσθω", "~φε~ᾰσθων", _;
    perfect_imp_pass_2: "~φε~ᾰσο", _, _;
    perfect_imp_pass_3: "~φε~ᾰσθω", "~φε~ᾰσθων", _;
    pluperfect_act_1: "~φε~ᾰκειν|~φε~ᾰκη|~φε~ηκειν|~φε~ηκη|~φει~ηκειν|~φει~ηκη", _, "~φε~ᾰκεμεν|~φε~ηκεμεν|~φει~ηκεμεν|~φε~ᾰμεν";
    pluperfect_act_2: "~φε~ᾰκεις|~φε~ᾰκης|~φε~ηκεις|~φε~ηκης|~φει~ηκεις|~φει~ηκης", "~φε~ᾰκετον|~φε~ηκετον|~φει~ηκετον|~φε~ᾰτον", "~φε~ᾰκετε|~φε~ηκετε|~φει~ηκετε|~φε~ᾰτε";
    pluperfect_act_3: "~φε~ᾰκει|~φε~ᾰκειν|~φε~ηκει|~φε~ηκειν|~φει~ηκει|~φει~ηκειν", "~φε~ᾰκετην|~φε~ηκετην|~φει~ηκετην|~φε~ᾰτην", "~φε~ᾰκεσᾰν|~φε~ηκεσᾰν|~φει~ηκεσᾰν|~φε~ᾰσᾰν";
    future_perfect_ind_act_1: "~φε~ηξω", _, "~φε~ηξομεν";
    future_perfect_ind_act_2: "~φε~ηξεις", "~φε~ηξετον", "~φε~ηξετε";
    future_perfect_ind_act_3: "~φε~ηξει", _, "~φε~ηξουσῐ|~φε~ηξουσῐν";
    future_perfect_ind_mid_1: "~φε~ηξομαι", _, "~φε~ηξομεθᾰ";
    future_perfect_ind_mid_2: "~φε~ηξῃ|~φε~ηξει", "~φε~ηξεσθον", "~φε~ηξεσθε";
    future_perfect_ind_mid_3: "~φε~ηξεται", _, "~φε~ηξονται";
    future_perfect_ind_pass_2: "~φε~ηξῃ|~φε~ηξει", _, _;
    future_perfect_opt_act_1: "~φε~ηξοιμῐ", _, "~φε~ηξοιμεν";
    future_perfect_opt_act_2: "~φε~ηξοις", "~φε~ηξοιτον", "~φε~ηξοιτε";
    future_perfect_opt_act_3: "~φε~ηξοι", "~φε~ηξοιτην", "~φε~ηξοιεν";
    future_perfect_opt_mid_1: "~φε~ηξοιμην", _, "~φε~ηξοιμεθᾰ";
    future_perfect_opt_mid_2: "~φε~ηξοιο", "~φε~ηξοισθον", "~φε~ηξοισθε";
    future_perfect_opt_mid_3: "~φε~ηξοιτο", "~φε~ηξοισθην", "~φε~ηξοιντο";
    infinitive_pres: "~φῐ~ᾰναι|~φε~ηκειν", "~φῐ~ᾰσθαι|~φε~ηκεσθαι", "~φῐ~ᾰσθαι|~φε~ηκεσθαι";
    infinitive_fut: "~πῐ~ᾱσεῑν", "~πῐ~ᾱσεῑσθαι", _;
    infinitive_aor: "~πῐ~ησαι|~πῐ~ηναι|~πῐ~ᾰσκεῑν", "~πῐ~ησᾰσθαι|~πῐ~ᾰσθαι|~πῐ~ᾰσκεσθαι", "~πῐ~ᾰθηναι|~πῐ~ηναι";
    infinitive_perf: "~φε~ᾰκεναι|~φε~ηκεναι|~φε~ᾰναι", "~φε~ᾰσθαι", _;
    infinitive_futp: "~φε~ηξειν", "~φε~ηξεσθαι", _;
    participle_pres: "~φῐ~ᾱς|~φε~ηκων", "~φῐ~ᾰμενος|~φε~ηκομενος", "~φῐ~ᾰμενος|~φε~ηκομενος";
    participle_fut: "~πῐ~ᾱσεων", "~πῐ~ᾱσεομενος", _;
    participle_aor: "~πῐ~ησᾱς|~πῐ~ᾱς|~πῐ~ᾰσκων", "~πῐ~ησᾰμενος|~πῐ~ᾰμενος|~πῐ~ᾰσκομενος", "~πῐ~ᾰθεις|~πῐ~εις";
    participle_perf: "~φε~ᾰκως|~φε~ηκως|~φε~ως", "~φε~ᾰμενος", _;
    participle_futp: "~φε~ηξων", "~φε~ηξομενος", _;
    ),
    // Κᾰθῐστημῐ: 1 words (κᾰθῐστημῐ).
    conjugation!("κᾰθῐστημῐ", 1, "~θῐ~ημῐ";
    ind_act_1: "~θῐ~ημῐ|~θε~ηκω", _, "~θῐ~ᾰμεν|~θε~ηκομεν";
    ind_act_2: "~θῐ~ης|~θε~ηκεις", "~θῐ~ᾰτον|~θε~ηκετον", "~θῐ~ᾰτε|~θε~ηκετε";
    ind_act_3: "~θῐ~ησῐ|~θῐ~ησῐν|~θε~ηκει", "~θῐ~ᾰτον|~θε~ηκετον", "~θῐ~ᾱσῐ|~θῐ~ᾱσῐν|~θε~ηκουσῐ|~θε~ηκουσῐν";
    ind_mid_1: "~θῐ~ᾰμαι|~θε~ηκομαι", _, "~θῐ~ᾰμεθᾰ|~θε~ηκομεθᾰ";
    ind_mid_2: "~θῐ~ᾰσαι|~θε~ηκῃ|~θε~ηκει", "~θῐ~ᾰσθον|~θε~ηκεσθον", "~θῐ~ᾰσθε|~θε~ηκεσθε";
    ind_mid_3: "~θῐ~ᾰται|~θε~ηκεται", "~θῐ~ᾰσθον|~θε~ηκεσθον", "~θῐ~ᾰνται|~θε~ηκονται";
    ind_pass_1: "~θῐ~ᾰμαι|~θε~ηκομαι", _, "~θῐ~ᾰμεθᾰ|~θε~ηκομεθᾰ";
    ind_pass_2: "~θῐ~ᾰσαι|~θε~ηκῃ|~θε~ηκει", "~θῐ~ᾰσθον|~θε~ηκεσθον", "~θῐ~ᾰσθε|~θε~ηκεσθε";
    ind_pass_3: "~θῐ~ᾰται|~θε~ηκεται", "~θῐ~ᾰσθον|~θε~ηκεσθον", "~θῐ~ᾰνται|~θε~ηκονται";
    subj_act_1: "~θῐ~ω|~θε~ηκω", _, "~θῐ~ωμεν|~θε~ηκωμεν";
    subj_act_2: "~θῐ~ῃς|~θε~ηκῃς", "~θῐ~ητον|~θε~ηκητον", "~θῐ~ητε|~θε~ηκητε";
    subj_act_3: "~θῐ~ῃ|~θε~ηκῃ", "~θῐ~ητον|~θε~ηκητον", "~θῐ~ωσῐ|~θῐ~ωσῐν|~θε~ηκωσῐ|~θε~ηκωσῐν";
    subj_mid_1: "~θῐ~ωμαι|~θε~ηκωμαι", _, "~θῐ~ωμεθᾰ|~θε~ηκωμεθᾰ";
    subj_mid_2: "~θῐ~ῃ|~θε~ηκῃ", "~θῐ~ησθον|~θε~ηκησθον", "~θῐ~ησθε|~θε~ηκησθε";
    subj_mid_3: "~θῐ~ηται|~θε~ηκηται", "~θῐ~ησθον|~θε~ηκησθον", "~θῐ~ωνται|~θε~ηκωνται";
    subj_pass_1: "~θῐ~ωμαι|~θε~ηκωμαι", _, "~θῐ~ωμεθᾰ|~θε~ηκωμεθᾰ";
    subj_pass_2: "~θῐ~ῃ|~θε~ηκῃ", "~θῐ~ησθον|~θε~ηκησθον", "~θῐ~ησθε|~θε~ηκησθε";
    subj_pass_3: "~θῐ~ηται|~θε~ηκηται", "~θῐ~ησθον|~θε~ηκησθον", "~θῐ~ωνται|~θε~ηκωνται";
    opt_act_1: "~θῐ~αιην|~θε~ηκοιμῐ", _, "~θῐ~αῑμεν|~θῐ~αιημεν|~θε~ηκοιμεν";
    opt_act_2: "~θῐ~αιης|~θε~ηκοις", "~θῐ~αῑτον|~θῐ~αιητον|~θε~ηκοιτον", "~θῐ~αῑτε|~θῐ~αιητε|~θε~ηκοιτε";
    opt_act_3: "~θῐ~αιη|~θε~ηκοι", "~θῐ~αιτην|~θῐ~αιητην|~θε~ηκοιτην", "~θῐ~αῑεν|~θῐ~αιησᾰν|~θε~ηκοιεν";
    opt_mid_1: "~θῐ~αιμην|~θε~ηκοιμην", _, "~θῐ~αιμεθᾰ|~θε~ηκοιμεθᾰ";
    opt_mid_2: "~θῐ~αιο|~θε~ηκοιο", "~θῐ~αισθον|~θε~ηκοισθον", "~θῐ~αισθε|~θε~ηκοισθε";
    opt_mid_3: "~θῐ~αιτο|~θε~ηκοιτο", "~θῐ~αισθην|~θε~ηκοισθην", "~θῐ~αιντο|~θε~ηκοιντο";
    opt_pass_1: "~θῐ~αιμην|~θε~ηκοιμην", _, "~θῐ~αιμεθᾰ|~θε~ηκοιμεθᾰ";
    opt_pass_2: "~θῐ~αιο|~θε~ηκοιο", "~θῐ~αισθον|~θε~ηκοισθον", "~θῐ~αισθε|~θε~ηκοισθε";
    opt_pass_3: "~θῐ~αιτο|~θε~ηκοιτο", "~θῐ~αισθην|~θε~ηκοισθην", "~θῐ~αιντο|~θε~ηκοιντο";
    imp_act_2: "~θῐ~η|~θε~ηκε", "~θῐ~ᾰτον|~θε~ηκετον", "~θῐ~ᾰτε|~θε~ηκετε";
    imp_act_3: "~θῐ~ᾰτω|~θε~ηκετω", "~θῐ~ᾰτων|~θε~ηκετων", "~θῐ~ᾰντων|~θε~ηκοντων";
    imp_mid_2: "~θῐ~ᾰσο|~θῐ~ω|~θε~ηκου", "~θῐ~ᾰσθον|~θε~ηκεσθον", "~θῐ~ᾰσθε|~θε~ηκεσθε";
    imp_mid_3: "~θῐ~ᾰσθω|~θε~ηκεσθω", "~θῐ~ᾰσθων|~θε~ηκεσθων", "~θῐ~ᾰσθων|~θε~ηκεσθων";
    imp_pass_2: "~θῐ~ᾰσο|~θῐ~ω|~θε~ηκου", "~θῐ~ᾰσθον|~θε~ηκεσθον", "~θῐ~ᾰσθε|~θε~ηκεσθε";
    imp_pass_3: "~θῐ~ᾰσθω|~θε~ηκεσθω", "~θῐ~ᾰσθων|~θε~ηκεσθων", "~θῐ~ᾰσθων|~θε~ηκεσθων";
    imperfect_act_1: "~θῑ~ην|~θι~ασκον", _, "~θῑ~ᾰμεν|~θι~ασκομεν";
    imperfect_act_2: "~θῑ~ης|~θι~ασκες", "~θῑ~ᾰτον|~θι~ασκετον", "~θῑ~ᾰτε|~θι~ασκετε";
    imperfect_act_3: "~θῑ~η|~θι~ασκε|~θι~ασκεν", "~θῑ~ᾰτην|~θι~ασκετην", "~θῑ~ᾰσᾰν|~θι~ασκον";
    imperfect_mid_1: "~θῑ~ᾰμην|~θι~ασκομην", _, "~θῑ~ᾰμεθᾰ|~θι~ασκομεθᾰ";
    imperfect_mid_2: "~θῑ~ᾰσο|~θι~ασκου", "~θῑ~ᾰσθον|~θι~ασκεσθον", "~θῑ~ᾰσθε|~θι~ασκεσθε";
    imperfect_mid_3: "~θῑ~ᾰτο|~θι~ασκετο", "~θῑ~ᾰσθην|~θι~ασκεσθην", "~θῑ~ᾰντο|~θι~ασκοντο";
    imperfect_pass_1: "~θῑ~ᾰμην|~θι~ασκομην", _, "~θῑ~ᾰμεθᾰ|~θι~ασκομεθᾰ";
    imperfect_pass_2: "~θῑ~ᾰσο|~θι~ασκου", "~θῑ~ᾰσθον|~θι~ασκεσθον", "~θῑ~ᾰσθε|~θι~ασκεσθε";
    imperfect_pass_3: "~θῑ~ᾰτο|~θι~ασκετο", "~θῑ~ᾰσθην|~θι~ασκεσθην", "~θῑ~ᾰντο|~θι~ασκοντο";
    future_ind_act_1: "~τᾰ~ᾱσεω", _, "~τᾰ~ᾱσεομες";
    future_ind_act_2: "~τᾰ~ᾱσεῑς", "~τᾰ~ᾱσεῑτον", "~τᾰ~ᾱσεῑτε";
    future_ind_act_3: "~τᾰ~ᾱσεῑ", _, "~τᾰ~ᾱσεοντῐ";
    future_ind_mid_1: "~τᾰ~ᾱσεομαι", _, "~τᾰ~ᾱσεομεθᾰ";
    future_ind_mid_2: "~τᾰ~ᾱσῃ", "~τᾰ~ᾱσεῑσθον", "~τᾰ~ᾱσεῑσθε";
    future_ind_mid_3: "~τᾰ~ᾱσεῑται", _, "~τᾰ~ᾱσεονται";
    future_opt_act_1: "~τᾰ~ᾱσεοιμῐ", _, "~τᾰ~ᾱσεοιμες";
    future_opt_act_2: "~τᾰ~ᾱσεοις", "~τᾰ~ᾱσεοιτον", "~τᾰ~ᾱσεοιτε";
    future_opt_act_3: "~τᾰ~ᾱσεοι", "~τᾰ~ᾱσεοιτᾱν", "~τᾰ~ᾱσεοιεν";
    future_opt_mid_1: "~τᾰ~ᾱσεοιμᾱν", _, "~τᾰ~ᾱσεοιμεθᾰ";
    future_opt_mid_2: "~τᾰ~ᾱσεοιο", "~τᾰ~ᾱσεοισθον", "~τᾰ~ᾱσεοισθε";
    future_opt_mid_3: "~τᾰ~ᾱσεοιτο", "~τᾰ~ᾱσεοισθᾱν", "~τᾰ~ᾱσεοιντο";
    aorist_ind_act_1: "~τε~ησᾰ|~τε~ην|~τᾰ~ᾰσκον", _, "~τε~ησᾰμεν|~τε~ημεν|~τᾰ~ᾰσκομεν";
    aorist_ind_act_2: "~τε~ησᾰς|~τε~ης|~τᾰ~ᾰσκες", "~τε~ησᾰτον|~τε~ητον|~τᾰ~ᾰσκετον", "~τε~ησᾰτε|~τε~ητε|~τᾰ~ᾰσκετε";
    aorist_ind_act_3: "~τε~ησε|~τε~ησεν|~τε~η|~τᾰ~ᾰσκε|~τᾰ~ᾰσκεν", "~τε~ησᾰτην|~τε~ητην|~τᾰ~ᾰσκετην", "~τε~ησᾰν|~τᾰ~ᾰσκον";
    aorist_ind_mid_1: "~τε~ησᾰμην|~τε~ᾰμην|~τᾰ~ᾰσκομην", _, "~τε~ησᾰμεθᾰ|~τε~ᾰμεθᾰ|~τᾰ~ᾰσκομεθᾰ";
    aorist_ind_mid_2: "~τε~ησω|~τε~ω|~τᾰ~ᾰσκου", "~τε~ησᾰσθον|~τε~ᾰσθον|~τᾰ~ᾰσκεσθον", "~τε~ησᾰσθε|~τε~ᾰσθε|~τᾰ~ᾰσκεσθε";
    aorist_ind_mid_3: "~τε~ησᾰτο|~τε~ᾰτο|~τᾰ~ᾰσκετο", "~τε~ησᾰσθην|~τε~ᾰσθην|~τᾰ~ᾰσκεσθην", "~τε~ησᾰντο|~τε~ᾰντο|~τᾰ~ᾰσκοντο";
    aorist_ind_pass_1: "~τε~ᾰθην|~τε~ην", _, "~τε~ᾰθημεν|~τε~ημεν";
    aorist_ind_pass_2: "~τε~ᾰθης|~τε~ης", "~τε~ᾰθητον|~τε~ητον", "~τε~ᾰθητε|~τε~ητε";
    aorist_ind_pass_3: "~τε~ᾰθη|~τε~η", "~τε~ᾰθητην|~τε~ητην", "~τε~ᾰθησᾰν|~τε~ησᾰν";
    aorist_subj_act_1: "~τᾰ~ησω|~τᾰ~ω|~τᾰ~ᾰσκω", _, "~τᾰ~ησωμεν|~τᾰ~ωμεν|~τᾰ~ᾰσκωμεν";
    aorist_subj_act_2: "~τᾰ~ησῃς|~τᾰ~ῃς|~τᾰ~ᾰσκῃς", "~τᾰ~ησητον|~τᾰ~ητον|~τᾰ~ᾰσκητον", "~τᾰ~ησητε|~τᾰ~ητε|~τᾰ~ᾰσκητε";
    aorist_subj_act_3: "~τᾰ~ησῃ|~τᾰ~ῃ|~τᾰ~ᾰσκῃ", "~τᾰ~ησητον|~τᾰ~ητον|~τᾰ~ᾰσκητον", "~τᾰ~ησωσῐ|~τᾰ~ησωσῐν|~τᾰ~ωσῐ|~τᾰ~ωσῐν|~τᾰ~ᾰσκωσῐ|~τᾰ~ᾰσκωσῐν";
    aorist_subj_mid_1: "~τᾰ~ησωμαι|~τᾰ~ωμαι|~τᾰ~ᾰσκωμαι", _, "~τᾰ~ησωμεθᾰ|~τᾰ~ωμεθᾰ|~τᾰ~ᾰσκωμεθᾰ";
    aorist_subj_mid_2: "~τᾰ~ησῃ|~τᾰ~ῃ|~τᾰ~ᾰσκῃ", "~τᾰ~ησησθον|~τᾰ~ησθον|~τᾰ~ᾰσκησθον", "~τᾰ~ησησθε|~τᾰ~ησθε|~τᾰ~ᾰσκησθε";
    aorist_subj_mid_3: "~τᾰ~ησηται|~τᾰ~ηται|~τᾰ~ᾰσκηται", "~τᾰ~ησησθον|~τᾰ~ησθον|~τᾰ~ᾰσκησθον", "~τᾰ~ησωνται|~τᾰ~ωνται|~τᾰ~ᾰσκωνται";
    aorist_subj_pass_1: "~τᾰ~ᾰθω|~τᾰ~ω", _, "~τᾰ~ᾰθωμεν|~τᾰ~ωμεν";
    aorist_subj_pass_2: "~τᾰ~ᾰθῃς|~τᾰ~ῃς", "~τᾰ~ᾰθητον|~τᾰ~ητον", "~τᾰ~ᾰθητε|~τᾰ~ητε";
    aorist_subj_pass_3: "~τᾰ~ᾰθῃ|~τᾰ~ῃ", "~τᾰ~ᾰθητον|~τᾰ~ητον", "~τᾰ~ᾰθωσῐ|~τᾰ~ᾰθωσῐν|~τᾰ~ωσῐ|~τᾰ~ωσῐν";
    aorist_opt_act_1: "~τᾰ~ησαιμῐ|~τᾰ~αιην|~τᾰ~ᾰσκοιμῐ", _, "~τᾰ~ησαιμεν|~τᾰ~αῑμεν|~τᾰ~αιημεν|~τᾰ~ᾰσκοιμεν";
    aorist_opt_act_2: "~τᾰ~ησειᾰς|~τᾰ~ησαις|~τᾰ~αιης|~τᾰ~ᾰσκοις", "~τᾰ~ησαιτον|~τᾰ~αῑτον|~τᾰ~αιητον|~τᾰ~ᾰσκοιτον", "~τᾰ~ησαιτε|~τᾰ~αῑτε|~τᾰ~αιητε|~τᾰ~ᾰσκοιτε";
    aorist_opt_act_3: "~τᾰ~ησειε|~τᾰ~ησειεν|~τᾰ~ησαι|~τᾰ~αιη|~τᾰ~ᾰσκοι", "~τᾰ~ησαιτην|~τᾰ~αιτην|~τᾰ~αιητην|~τᾰ~ᾰσκοιτην", "~τᾰ~ησειᾰν|~τᾰ~ησαιεν|~τᾰ~αῑεν|~τᾰ~αιησᾰν|~τᾰ~ᾰσκοιεν";
    aorist_opt_mid_1: "~τᾰ~ησαιμην|~τᾰ~αιμην|~τᾰ~ᾰσκοιμην", _, "~τᾰ~ησαιμεθᾰ|~τᾰ~αιμεθᾰ|~τᾰ~ᾰσκοιμεθᾰ";
    aorist_opt_mid_2: "~τᾰ~ησαιο|~τᾰ~αῑο|~τᾰ~ᾰσκοιο", "~τᾰ~ησαισθον|~τᾰ~αῑσθον|~τᾰ~ᾰσκοισθον", "~τᾰ~ησαισθε|~τᾰ~αῑσθε|~τᾰ~ᾰσκοισθε";
    aorist_opt_mid_3: "~τᾰ~ησαιτο|~τᾰ~αῑτο|~τᾰ~ᾰσκοιτο", "~τᾰ~ησαισθην|~τᾰ~αισθην|~τᾰ~ᾰσκοισθην", "~τᾰ~ησαιντο|~τᾰ~αῑντο|~τᾰ~ᾰσκοιντο";
    aorist_opt_pass_1: "~τᾰ~ᾰθειην|~τᾰ~ειην", _, "~τᾰ~ᾰθεῑμεν|~τᾰ~ᾰθειημεν|~τᾰ~εῑμεν|~τᾰ~ειημεν";
    aorist_opt_pass_2: "~τᾰ~ᾰθειης|~τᾰ~ειης", "~τᾰ~ᾰθεῑτον|~τᾰ~ᾰθειητον|~τᾰ~εῑτον|~τᾰ~ειητον", "~τᾰ~ᾰθεῑτε|~τᾰ~ᾰθειητε|~τᾰ~εῑτε|~τᾰ~ειητε";
    aorist_opt_pass_3: "~τᾰ~ᾰθειη|~τᾰ~ειη", "~τᾰ~ᾰθειτην|~τᾰ~ᾰθειητην|~τᾰ~ειτην|~τᾰ~ειητην", "~τᾰ~ᾰθεῑεν|~τᾰ~ᾰθειησᾰν|~τᾰ~εῑεν|~τᾰ~ειησᾰν";
    aorist_imp_act_2: "~τᾰ~ησον|~τᾰ~ηθῐ|~τᾰ~ᾰσκε", "~τᾰ~ησᾰτον|~τᾰ~ητον|~τᾰ~ᾰσκετον", "~τᾰ~ησᾰτε|~τᾰ~ητε|~τᾰ~ᾰσκετε";
    aorist_imp_act_3: "~τᾰ~ησᾰτω|~τᾰ~ητω|~τᾰ~ᾰσκετω", "~τᾰ~ησᾰτων|~τᾰ~ητων|~τᾰ~ᾰσκετων", "~τᾰ~ησᾰντων|~τᾰ~ᾰντων|~τᾰ~ᾰσκοντων";
    aorist_imp_mid_2: "~τᾰ~ησαι|~τᾰ~ω|~τᾰ~ᾰσκοῡ", "~τᾰ~ησᾰσθον|~τᾰ~ᾰσθον|~τᾰ~ᾰσκεσθον", "~τᾰ~ησᾰσθε|~τᾰ~ᾰσθε|~τᾰ~ᾰσκεσθε";
    aorist_imp_mid_3: "~τᾰ~ησᾰσθω|~τᾰ~ᾰσθω|~τᾰ~ᾰσκεσθω", "~τᾰ~ησᾰσθων|~τᾰ~ᾰσθων|~τᾰ~ᾰσκεσθων", "~τᾰ~ησᾰσθων|~τᾰ~ᾰσθων|~τᾰ~ᾰσκεσθων";
    aorist_imp_pass_2: "~τᾰ~ᾰθητῐ|~τᾰ~ηθῐ", "~τᾰ~ᾰθητον|~τᾰ~ητον", "~τᾰ~ᾰθητε|~τᾰ~ητε";
    aorist_imp_pass_3: "~τᾰ~ᾰθητω|~τᾰ~ητω", "~τᾰ~ᾰθητων|~τᾰ~ητων", "~τᾰ~ᾰθεντων|~τᾰ~εντων";
    perfect_ind_act_1: "~θε~ᾰκᾰ|~θε~ηκᾰ", _, "~θε~ᾰκᾰμεν|~θε~ηκᾰμεν|~θε~ᾰμεν";
    perfect_ind_act_2: "~θε~ᾰκᾰς|~θε~ηκᾰς", "~θε~ᾰκᾰτον|~θε~ηκᾰτον|~θε~ᾰτον", "~θε~ᾰκᾰτε|~θε~ηκᾰτε|~θε~ᾰτε";
    perfect_ind_act_3: "~θε~ᾰκε|~θε~ᾰκεν|~θε~ηκε|~θε~ηκεν", "~θε~ᾰκᾰτον|~θε~ηκᾰτον|~θε~ᾰτον", "~θε~ᾰκᾱσῐ|~θε~ᾰκᾱσῐν|~θε~ηκᾱσῐ|~θε~ηκᾱσῐν|~θε~ᾱσῐ|~θε~ᾱσῐν";
    perfect_ind_mid_1: "~θε~ᾰμαι", _, "~θε~ᾰμεθᾰ";
    perfect_ind_mid_2: "~θε~ᾰσαι", "~θε~ᾰσθον", "~θε~ᾰσθε";
    perfect_ind_mid_3: "~θε~ᾰται", _, "~θε~ᾰνται";
    perfect_ind_pass_1: _, _, "~θε~ᾰμεθᾰ";
    perfect_ind_pass_2: _, _, "~θε~ᾰσθε";
    perfect_subj_act_1: "~θε~ᾰκω|~θε~ηκω|~θε~ω", _, "~θε~ᾰκωμεν|~θε~ηκωμεν|~θε~ωμεν";
    perfect_subj_act_2: "~θε~ᾰκῃς|~θε~ηκῃς|~θε~ῃς", "~θε~ᾰκητον|~θε~ηκητον|~θε~ητον", "~θε~ᾰκητε|~θε~ηκητε|~θε~ητε";
    perfect_subj_act_3: "~θε~ᾰκῃ|~θε~ηκῃ|~θε~ῃ", "~θε~ᾰκητον|~θε~ηκητον|~θε~ητον", "~θε~ᾰκωσῐ|~θε~ᾰκωσῐν|~θε~ηκωσῐ|~θε~ηκωσῐν|~θε~ωσῐ|~θε~ωσῐν";
    perfect_opt_act_1: "~θε~ᾰκοιμῐ|~θε~ᾰκοιην|~θε~ηκοιμῐ|~θε~ηκοιην|~θε~αιην", _, "~θε~ᾰκοιμεν|~θε~ηκοιμεν|~θε~αῑμεν|~θε~αιημεν";
    perfect_opt_act_2: "~θε~ᾰκοις|~θε~ᾰκοιης|~θε~ηκοις|~θε~ηκοιης|~θε~αιης", "~θε~ᾰκοιτον|~θε~ηκοιτον|~θε~αῑτον|~θε~αιητον", "~θε~ᾰκοιτε|~θε~ηκοιτε|~θε~αῑτε|~θε~αιητε";
    perfect_opt_act_3: "~θε~ᾰκοι|~θε~ᾰκοιη|~θε~ηκοι|~θε~ηκοιη|~θε~αιη", "~θε~ᾰκοιτην|~θε~ηκοιτην|~θε~αιτην|~θε~αιητην", "~θε~ᾰκοιεν|~θε~ηκοιεν|~θε~αῑεν|~θε~αιησᾰν";
    perfect_imp_act_2: "~θε~ᾰκε|~θε~ηκε|~θε~ᾰθῐ", "~θε~ᾰκετον|~θε~ηκετον|~θε~ᾰτον", "~θε~ᾰκετε|~θε~ηκετε|~θε~ᾰτε";
    perfect_imp_act_3: "~θε~ᾰκετω|~θε~ηκετω|~θε~ᾰτω", "~θε~ᾰκετων|~θε~ηκετων|~θε~ᾰτων", "~θε~ᾰκοντων|~θε~ηκοντων|~θε~ᾰντων";
    perfect_imp_mid_2: "~θε~ᾰσο", _, _;
    perfect_imp_mid_3: "~θε~ᾰσθω", "~θε~ᾰσθων", _;
    perfect_imp_pass_2: "~θε~ᾰσο", _, _;
    perfect_imp_pass_3: "~θε~ᾰσθω", "~θε~ᾰσθων", _;
    pluperfect_act_1: "~θε~ᾰκειν|~θε~ᾰκη|~θε~ηκειν|~θε~ηκη|~θει~ηκειν|~θει~ηκη", _, "~θε~ᾰκεμεν|~θε~ηκεμεν|~θει~ηκεμεν|~θε~ᾰμεν";
    pluperfect_act_2: "~θε~ᾰκεις|~θε~ᾰκης|~θε~ηκεις|~θε~ηκης|~θει~ηκεις|~θει~ηκης", "~θε~ᾰκετον|~θε~ηκετον|~θει~ηκετον|~θε~ᾰτον", "~θε~ᾰκετε|~θε~ηκετε|~θει~ηκετε|~θε~ᾰτε";
    pluperfect_act_3: "~θε~ᾰκει|~θε~ᾰκειν|~θε~ηκει|~θε~ηκειν|~θει~ηκει|~θει~ηκειν", "~θε~ᾰκετην|~θε~ηκετην|~θει~ηκετην|~θε~ᾰτην", "~θε~ᾰκεσᾰν|~θε~ηκεσᾰν|~θει~ηκεσᾰν|~θε~ᾰσᾰν";
    pluperfect_mid_1: "~θε~ᾰμην", _, "~θε~ᾰμεθᾰ";
    pluperfect_mid_2: "~θε~ᾰσο", "~θε~ᾰσθον", "~θε~ᾰσθε";
    pluperfect_mid_3: "~θε~ᾰτο", "~θε~ᾰσθην", _;
    pluperfect_pass_1: _, _, "~θε~ᾰμεθᾰ";
    pluperfect_pass_2: _, _, "~θε~ᾰσθε";
    pluperfect_pass_3: _, "~θε~ᾰσθην", "~θε~ᾰντο";
    future_perfect_ind_act_1: "~θε~ηξω", _, "~θε~ηξομεν";
    future_perfect_ind_act_2: "~θε~ηξεις", "~θε~ηξετον", "~θε~ηξετε";
    future_perfect_ind_act_3: "~θε~ηξει", _, "~θε~ηξουσῐ|~θε~ηξουσῐν";
    future_perfect_ind_mid_1: "~θε~ηξομαι", _, "~θε~ηξομεθᾰ";
    future_perfect_ind_mid_2: "~θε~ηξῃ|~θε~ηξει", "~θε~ηξεσθον", "~θε~ηξεσθε";
    future_perfect_ind_mid_3: "~θε~ηξεται", _, "~θε~ηξονται";
    future_perfect_ind_pass_2: "~θε~ηξῃ|~θε~ηξει", _, _;
    future_perfect_opt_act_1: "~θε~ηξοιμῐ", _, "~θε~ηξοιμεν";
    future_perfect_opt_act_2: "~θε~ηξοις", "~θε~ηξοιτον", "~θε~ηξοιτε";
    future_perfect_opt_act_3: "~θε~ηξοι", "~θε~ηξοιτην", "~θε~ηξοιεν";
    future_perfect_opt_mid_1: "~θε~ηξοιμην", _, "~θε~ηξοιμεθᾰ";
    future_perfect_opt_mid_2: "~θε~ηξοιο", "~θε~ηξοισθον", "~θε~ηξοισθε";
    future_perfect_opt_mid_3: "~θε~ηξοιτο", "~θε~ηξοισθην", "~θε~ηξοιντο";
    infinitive_pres: "~θῐ~ᾰναι|~θε~ηκειν", "~θῐ~ᾰσθαι|~θε~ηκεσθαι", "~θῐ~ᾰσθαι|~θε~ηκεσθαι";
    infinitive_fut: "~τᾰ~ᾱσεῑν", "~τᾰ~ᾱσεῑσθαι", _;
    infinitive_aor: "~τᾰ~ησαι|~τᾰ~ηναι|~τᾰ~ᾰσκεῑν", "~τᾰ~ησᾰσθαι|~τᾰ~ᾰσθαι|~τᾰ~ᾰσκεσθαι", "~τᾰ~ᾰθηναι|~τᾰ~ηναι";
    infinitive_perf: "~θε~ᾰκεναι|~θε~ηκεναι|~θε~ᾰναι", "~θε~ᾰσθαι", _;
    infinitive_futp: "~θε~ηξειν", "~θε~ηξεσθαι", _;
    participle_pres: "~θῐ~ᾱς|~θε~ηκων", "~θῐ~ᾰμενος|~θε~ηκομενος", "~θῐ~ᾰμενος|~θε~ηκομενος";
    participle_fut: "~τᾰ~ᾱσεων", "~τᾰ~ᾱσεομενος", _;
    participle_aor: "~τᾰ~ησᾱς|~τᾰ~ᾱς|~τᾰ~ᾰσκων", "~τᾰ~ησᾰμενος|~τᾰ~ᾰμενος|~τᾰ~ᾰσκομενος", "~τᾰ~ᾰθεις|~τᾰ~εις";
    participle_perf: "~θε~ᾰκως|~θε~ηκως|~θε~ως", "~θε~ᾰμενος", _;
    participle_futp: "~θε~ηξων", "~θε~ηξομενος", _;
    ),
    // Ἠμῐ: 1 words (ἠμῐ). Like Δεῑ, except in 15 cells: ind_act_1 sg, ind_act_3 sg, subj_act_3 sg, opt_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, ….
    conjugation!("ἠμῐ", 1, "~μῐ", like Δεῑ;
    ind_act_1: "~μῐ|~μῐ", _, _;
    ind_act_3: "~σῐ|~σῐ", _, _;
    subj_act_3: derived, _, _;
    opt_act_3: derived, _, _;
    imperfect_act_1: "~ν", _, _;
    imperfect_act_3: "~", _, _;
    future_ind_act_3: derived, _, _;
    future_opt_act_3: derived, _, _;
    aorist_ind_act_3: derived, _, _;
    aorist_subj_act_3: derived, _, _;
    aorist_opt_act_3: derived, _, _;
    perfect_ind_act_3: derived, _, _;
    infinitive_pres: derived, _, _;
    infinitive_fut: derived, _, _;
    infinitive_aor: derived, _, _;
    ),
    // Ἐτλην: 1 words (ἐτλην). Like Κᾰθημαι, except in 96 cells: ind_act_1 sg, future_ind_mid_1 sg, future_ind_mid_1 pl, future_ind_mid_2 sg, future_ind_mid_2 du, future_ind_mid_2 pl, ….
    conjugation!("ἐτλην", 1, "ἐ~~ην", like Κᾰθημαι;
    ind_act_1: "ἐ~~ην", _, _;
    future_ind_mid_1: "~~ησομαι|~~ᾰσομαι|~ᾰ~ᾰσσομαι", _, "~~ησομεθᾰ|~~ᾰσομεθᾰ|~ᾰ~ᾰσσομεθᾰ";
    future_ind_mid_2: "~~ησῃ|~~ησει|~~ᾰσῃ|~~ᾰσει|~ᾰ~ᾰσσῃ|~ᾰ~ᾰσσει", "~~ησεσθον|~~ᾰσεσθον|~ᾰ~ᾰσσεσθον", "~~ησεσθε|~~ᾰσεσθε|~ᾰ~ᾰσσεσθε";
    future_ind_mid_3: "~~ησεται|~~ᾰσεται|~ᾰ~ᾰσσεται", "~~ησεσθον|~~ᾰσεσθον|~ᾰ~ᾰσσεσθον", "~~ησονται|~~ᾰσονται|~ᾰ~ᾰσσονται";
    future_opt_mid_1: "~~ησοιμην|~~ᾰσοιμην|~ᾰ~ᾰσσοιμην", _, "~~ησοιμεθᾰ|~~ᾰσοιμεθᾰ|~ᾰ~ᾰσσοιμεθᾰ";
    future_opt_mid_2: "~~ησοιο|~~ᾰσοιο|~ᾰ~ᾰσσοιο", "~~ησοισθον|~~ᾰσοισθον|~ᾰ~ᾰσσοισθον", "~~ησοισθε|~~ᾰσοισθε|~ᾰ~ᾰσσοισθε";
    future_opt_mid_3: "~~ησοιτο|~~ᾰσοιτο|~ᾰ~ᾰσσοιτο", "~~ησοισθην|~~ᾰσοισθην|~ᾰ~ᾰσσοισθην", "~~ησοιντο|~~ᾰσοιντο|~ᾰ~ᾰσσοιντο";
    aorist_ind_act_1: "ἐ~~ην|ἐ~~ᾰν", _, "ἐ~~ημεν";
    aorist_ind_act_2: "ἐ~~ης|ἐ~~ᾰς", "ἐ~~ητον", "ἐ~~ητε";
    aorist_ind_act_3: "ἐ~~η|ἐ~~ᾰ", "ἐ~~ητην", "ἐ~~ησᾰν|ἐ~~ᾰσᾰν";
    aorist_subj_act_1: "~~ω", _, "~~ωμεν";
    aorist_subj_act_2: "~~ῃς", "~~ητον", "~~ητε";
    aorist_subj_act_3: "~~ῃ", _, "~~ωσῐ|~~ωσῐν";
    aorist_opt_act_1: "~~αιην", _, "~~αῑμεν|~~αιημεν";
    aorist_opt_act_2: "~~αιης", "~~αῑτον|~~αιητον", "~~αῑτε|~~αιητε";
    aorist_opt_act_3: "~~αιη", "~~αιτην|~~αιητην", "~~αῑεν|~~αιησᾰν";
    aorist_imp_act_2: "~~ηθῐ", "~~ητον", "~~ητε";
    aorist_imp_act_3: "~~ητω", "~~ητων", "~~ᾰντων";
    perfect_ind_act_1: "%ε~~ηκᾰ", _, "%ε~~ηκᾰμεν";
    perfect_ind_act_2: "%ε~~ηκᾰς", "%ε~~ηκᾰτον", "%ε~~ηκᾰτε";
    perfect_ind_act_3: "%ε~~ηκε|%ε~~ηκεν", _, "%ε~~ηκᾱσῐ|%ε~~ηκᾱσῐν";
    perfect_subj_act_1: "%ε~~ηκω", _, "%ε~~ηκωμεν";
    perfect_subj_act_2: "%ε~~ηκῃς", "%ε~~ηκητον", "%ε~~ηκητε";
    perfect_subj_act_3: "%ε~~ηκῃ", _, "%ε~~ηκωσῐ|%ε~~ηκωσῐν";
    perfect_opt_act_1: "%ε~~ηκοιμῐ|%ε~~ηκοιην", _, "%ε~~ηκοιμεν";
    perfect_opt_act_2: "%ε~~ηκοις|%ε~~ηκοιης", "%ε~~ηκοιτον", "%ε~~ηκοιτε";
    perfect_opt_act_3: "%ε~~ηκοι|%ε~~ηκοιη", "%ε~~ηκοιτην", "%ε~~ηκοιεν";
    perfect_imp_act_2: "%ε~~ηκε", "%ε~~ηκετον", "%ε~~ηκετε";
    perfect_imp_act_3: "%ε~~ηκετω", "%ε~~ηκετων", "%ε~~ηκοντων";
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    pluperfect_pass_1: derived, _, derived;
    pluperfect_pass_2: derived, derived, derived;
    pluperfect_pass_3: derived, derived, derived;
    infinitive_fut: _, "~~ησεσθαι|~~ᾰσεσθαι|~ᾰ~ᾰσσεσθαι", _;
    infinitive_aor: "~~ηναι|~~ημεν|~~ημεναι", _, _;
    infinitive_perf: "%ε~~ηκεναι", _, _;
    participle_fut: _, "~~ησομενος|~~ᾰσομενος|~ᾰ~ᾰσσομενος", _;
    participle_aor: "~~ᾱς", _, _;
    participle_perf: "%ε~~ηκως", _, _;
    ),
    // Χρη: 1 words (χρη). Like Ἠμῐ, except in 12 cells: ind_act_1 sg, ind_act_3 sg, subj_act_3 sg, opt_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, ….
    conjugation!("χρη", 1, "~η", like Ἠμῐ;
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
    // Ἠλθον: 1 words (ἠλθον). Like Ἐφᾰγον, except in 32 cells: ind_act_1 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_2 pl, ….
    conjugation!("ἠλθον", 1, "ἠ~ον", like Ἐφᾰγον;
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
    // Ζευγνῡμῐ: 1 words (ζευγνῡμῐ). Like Ἐπευχομαι, except in 276 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ζευγνῡμῐ", 1, "~ευγνῡμῐ", like Ἐπευχομαι;
    ind_act_1: "~ευγνῡμῐ|~ευγνυω", _, "~ευγνῠμεν|~ευγνυομεν";
    ind_act_2: "~ευγνῡς|~ευγνυεις", "~ευγνῠτον|~ευγνυετον", "~ευγνῠτε|~ευγνυετε";
    ind_act_3: "~ευγνῡσῐ|~ευγνῡσῐν|~ευγνυει", "~ευγνῠτον|~ευγνυετον", "~ευγνῠᾱσῐ|~ευγνῠᾱσῐν|~ευγνυουσῐ|~ευγνυουσῐν";
    ind_mid_1: "~ευγνῠμαι|~ευγνυομαι", _, "~ευγνῠμεθᾰ|~ευγνυομεθᾰ";
    ind_mid_2: "~ευγνῠσαι|~ευγνυῃ|~ευγνυει", "~ευγνῠσθον|~ευγνυεσθον", "~ευγνῠσθε|~ευγνυεσθε";
    ind_mid_3: "~ευγνῠται|~ευγνυεται", "~ευγνῠσθον|~ευγνυεσθον", "~ευγνῠνται|~ευγνυονται";
    ind_pass_1: "~ευγνῠμαι|~ευγνυομαι", _, "~ευγνῠμεθᾰ|~ευγνυομεθᾰ";
    ind_pass_2: "~ευγνῠσαι|~ευγνυῃ|~ευγνυει", "~ευγνῠσθον|~ευγνυεσθον", "~ευγνῠσθε|~ευγνυεσθε";
    ind_pass_3: "~ευγνῠται|~ευγνυεται", "~ευγνῠσθον|~ευγνυεσθον", "~ευγνῠνται|~ευγνυονται";
    subj_act_1: "~ευγνῠω|~ευγνυω", _, "~ευγνῠωμεν|~ευγνυωμεν";
    subj_act_2: "~ευγνῠῃς|~ευγνυῃς", "~ευγνῠητον|~ευγνυητον", "~ευγνῠητε|~ευγνυητε";
    subj_act_3: "~ευγνῠῃ|~ευγνυῃ", "~ευγνῠητον|~ευγνυητον", "~ευγνῠωσῐ|~ευγνῠωσῐν|~ευγνυωσῐ|~ευγνυωσῐν";
    subj_mid_1: "~ευγνῠωμαι|~ευγνυωμαι", _, "~ευγνῠωμεθᾰ|~ευγνυωμεθᾰ";
    subj_mid_2: "~ευγνῠῃ|~ευγνυῃ", "~ευγνῠησθον|~ευγνυησθον", "~ευγνῠησθε|~ευγνυησθε";
    subj_mid_3: "~ευγνῠηται|~ευγνυηται", "~ευγνῠησθον|~ευγνυησθον", "~ευγνῠωνται|~ευγνυωνται";
    subj_pass_1: "~ευγνῠωμαι|~ευγνυωμαι", _, "~ευγνῠωμεθᾰ|~ευγνυωμεθᾰ";
    subj_pass_2: "~ευγνῠῃ|~ευγνυῃ", "~ευγνῠησθον|~ευγνυησθον", "~ευγνῠησθε|~ευγνυησθε";
    subj_pass_3: "~ευγνῠηται|~ευγνυηται", "~ευγνῠησθον|~ευγνυησθον", "~ευγνῠωνται|~ευγνυωνται";
    opt_act_1: "~ευγνῠοιμῐ|~ευγνυοιμῐ", _, "~ευγνῠοιμεν|~ευγνυοιμεν";
    opt_act_2: "~ευγνῠοις|~ευγνυοις", "~ευγνῠοιτον|~ευγνυοιτον", "~ευγνῠοιτε|~ευγνυοιτε";
    opt_act_3: "~ευγνῠοι|~ευγνυοι", "~ευγνῠοιτην|~ευγνυοιτην", "~ευγνῠοιεν|~ευγνυοιεν";
    opt_mid_1: "~ευγνῠοιμην|~ευγνυοιμην", _, "~ευγνῠοιμεθᾰ|~ευγνυοιμεθᾰ";
    opt_mid_2: "~ευγνῠοιο|~ευγνυοιο", "~ευγνῠοισθον|~ευγνυοισθον", "~ευγνῠοισθε|~ευγνυοισθε";
    opt_mid_3: "~ευγνῠοιτο|~ευγνυοιτο", "~ευγνῠοισθην|~ευγνυοισθην", "~ευγνῠοιντο|~ευγνυοιντο";
    opt_pass_1: "~ευγνῠοιμην|~ευγνυοιμην", _, "~ευγνῠοιμεθᾰ|~ευγνυοιμεθᾰ";
    opt_pass_2: "~ευγνῠοιο|~ευγνυοιο", "~ευγνῠοισθον|~ευγνυοισθον", "~ευγνῠοισθε|~ευγνυοισθε";
    opt_pass_3: "~ευγνῠοιτο|~ευγνυοιτο", "~ευγνῠοισθην|~ευγνυοισθην", "~ευγνῠοιντο|~ευγνυοιντο";
    imp_act_2: "~ευγνῡ|~ευγνυε", "~ευγνῠτον|~ευγνυετον", "~ευγνῠτε|~ευγνυετε";
    imp_act_3: "~ευγνῠτω|~ευγνυετω", "~ευγνῠτων|~ευγνυετων", "~ευγνῠντων|~ευγνυοντων";
    imp_mid_2: "~ευγνῠσο|~ευγνυου", "~ευγνῠσθον|~ευγνυεσθον", "~ευγνῠσθε|~ευγνυεσθε";
    imp_mid_3: "~ευγνῠσθω|~ευγνυεσθω", "~ευγνῠσθων|~ευγνυεσθων", "~ευγνῠσθων|~ευγνυεσθων";
    imp_pass_2: "~ευγνῠσο|~ευγνυου", "~ευγνῠσθον|~ευγνυεσθον", "~ευγνῠσθε|~ευγνυεσθε";
    imp_pass_3: "~ευγνῠσθω|~ευγνυεσθω", "~ευγνῠσθων|~ευγνυεσθων", "~ευγνῠσθων|~ευγνυεσθων";
    imperfect_act_1: "ἐ~ευγνῡν|ἐ~ευγνυον", _, "ἐ~ευγνῠμεν|ἐ~ευγνυομεν";
    imperfect_act_2: "ἐ~ευγνῡς|ἐ~ευγνυες", "ἐ~ευγνῠτον|ἐ~ευγνυετον", "ἐ~ευγνῠτε|ἐ~ευγνυετε";
    imperfect_act_3: "ἐ~ευγνῡ|ἐ~ευγνυε|ἐ~ευγνυεν", "ἐ~ευγνῠτην|ἐ~ευγνυετην", "ἐ~ευγνῠσᾰν|ἐ~ευγνυον";
    imperfect_mid_1: "ἐ~ευγνῠμην|ἐ~ευγνυομην", _, "ἐ~ευγνῠμεθᾰ|ἐ~ευγνυομεθᾰ";
    imperfect_mid_2: "ἐ~ευγνῠσο|ἐ~ευγνυου", "ἐ~ευγνῠσθον|ἐ~ευγνυεσθον", "ἐ~ευγνῠσθε|ἐ~ευγνυεσθε";
    imperfect_mid_3: "ἐ~ευγνῠτο|ἐ~ευγνυετο", "ἐ~ευγνῠσθην|ἐ~ευγνυεσθην", "ἐ~ευγνῠντο|ἐ~ευγνυοντο";
    imperfect_pass_1: "ἐ~ευγνῠμην|ἐ~ευγνυομην", _, "ἐ~ευγνῠμεθᾰ|ἐ~ευγνυομεθᾰ";
    imperfect_pass_2: "ἐ~ευγνῠσο|ἐ~ευγνυου", "ἐ~ευγνῠσθον|ἐ~ευγνυεσθον", "ἐ~ευγνῠσθε|ἐ~ευγνυεσθε";
    imperfect_pass_3: "ἐ~ευγνῠτο|ἐ~ευγνυετο", "ἐ~ευγνῠσθην|ἐ~ευγνυεσθην", "ἐ~ευγνῠντο|ἐ~ευγνυοντο";
    future_ind_act_1: "~ευξω", _, "~ευξομεν";
    future_ind_act_2: "~ευξεις", "~ευξετον", "~ευξετε";
    future_ind_act_3: "~ευξει", _, "~ευξουσῐ|~ευξουσῐν";
    future_ind_mid_2: "~ευξῃ|~ευξει", _, _;
    future_opt_act_1: "~ευξοιμῐ", _, "~ευξοιμεν";
    future_opt_act_2: "~ευξοις", "~ευξοιτον", "~ευξοιτε";
    future_opt_act_3: "~ευξοι", "~ευξοιτην", "~ευξοιεν";
    aorist_ind_act_1: "ἐ~ευξᾰ", _, "ἐ~ευξᾰμεν";
    aorist_ind_act_2: "ἐ~ευξᾰς", "ἐ~ευξᾰτον", "ἐ~ευξᾰτε";
    aorist_ind_act_3: "ἐ~ευξε|ἐ~ευξεν", "ἐ~ευξᾰτην", "ἐ~ευξᾰν";
    aorist_ind_mid_1: "ἐ~ευξᾰμην", _, "ἐ~ευξᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ευξω", "ἐ~ευξᾰσθον", "ἐ~ευξᾰσθε";
    aorist_ind_mid_3: "ἐ~ευξᾰτο", "ἐ~ευξᾰσθην", "ἐ~ευξᾰντο";
    aorist_ind_pass_1: "ἐ~ευχθην|ἐ~υγην", _, "ἐ~ευχθημεν|ἐ~υγημεν";
    aorist_ind_pass_2: "ἐ~ευχθης|ἐ~υγης", "ἐ~ευχθητον|ἐ~υγητον", "ἐ~ευχθητε|ἐ~υγητε";
    aorist_ind_pass_3: "ἐ~ευχθη|ἐ~υγη", "ἐ~ευχθητην|ἐ~υγητην", "ἐ~ευχθησᾰν|ἐ~υγησᾰν";
    aorist_subj_act_1: "~ευξω", _, "~ευξωμεν";
    aorist_subj_act_2: "~ευξῃς", "~ευξητον", "~ευξητε";
    aorist_subj_act_3: "~ευξῃ", _, "~ευξωσῐ|~ευξωσῐν";
    aorist_subj_pass_1: "~ευχθω|~υγω", _, "~ευχθωμεν|~υγωμεν";
    aorist_subj_pass_2: "~ευχθῃς|~υγῃς", "~ευχθητον|~υγητον", "~ευχθητε|~υγητε";
    aorist_subj_pass_3: "~ευχθῃ|~υγῃ", "~ευχθητον|~υγητον", "~ευχθωσῐ|~ευχθωσῐν|~υγωσῐ|~υγωσῐν";
    aorist_opt_act_1: "~ευξαιμῐ", _, "~ευξαιμεν";
    aorist_opt_act_2: "~ευξειᾰς|~ευξαις", "~ευξαιτον", "~ευξαιτε";
    aorist_opt_act_3: "~ευξειε|~ευξειεν|~ευξαι", "~ευξαιτην", "~ευξειᾰν|~ευξαιεν";
    aorist_opt_pass_1: "~ευχθειην|~υγειην", _, "~ευχθεῑμεν|~ευχθειημεν|~υγεῑμεν|~υγειημεν";
    aorist_opt_pass_2: "~ευχθειης|~υγειης", "~ευχθεῑτον|~ευχθειητον|~υγεῑτον|~υγειητον", "~ευχθεῑτε|~ευχθειητε|~υγεῑτε|~υγειητε";
    aorist_opt_pass_3: "~ευχθειη|~υγειη", "~ευχθειτην|~ευχθειητην|~υγειτην|~υγειητην", "~ευχθεῑεν|~ευχθειησᾰν|~υγεῑεν|~υγειησᾰν";
    aorist_imp_act_2: "~εῡξον", "~ευξᾰτον", "~ευξᾰτε";
    aorist_imp_act_3: "~ευξᾰτω", "~ευξᾰτων", "~ευξᾰντων";
    aorist_imp_mid_2: "~εῡξαι", _, _;
    aorist_imp_pass_2: "~ευχθητῐ|~υγηθῐ", "~ευχθητον|~υγητον", "~ευχθητε|~υγητε";
    aorist_imp_pass_3: "~ευχθητω|~υγητω", "~ευχθητων|~υγητων", "~ευχθεντων|~υγεντων";
    perfect_ind_act_1: "ἐ~ευχᾰ", _, "ἐ~ευχᾰμεν";
    perfect_ind_act_2: "ἐ~ευχᾰς", "ἐ~ευχᾰτον", "ἐ~ευχᾰτε";
    perfect_ind_act_3: "ἐ~ευχε|ἐ~ευχεν", _, "ἐ~ευχᾱσῐ|ἐ~ευχᾱσῐν";
    perfect_ind_mid_1: "ἐ~ευγμαι", _, "ἐ~ευγμεθᾰ";
    perfect_ind_mid_2: "ἐ~ευξαι", "ἐ~ευχθον", "ἐ~ευχθε";
    perfect_ind_mid_3: "ἐ~ευκται", _, "ἐ~ευγᾰται";
    perfect_ind_pass_1: _, _, "ἐ~ευγμεθᾰ";
    perfect_ind_pass_2: _, _, "ἐ~ευχθε";
    perfect_subj_act_1: "ἐ~ευχω", _, "ἐ~ευχωμεν";
    perfect_subj_act_2: "ἐ~ευχῃς", "ἐ~ευχητον", "ἐ~ευχητε";
    perfect_subj_act_3: "ἐ~ευχῃ", _, "ἐ~ευχωσῐ|ἐ~ευχωσῐν";
    perfect_opt_act_1: "ἐ~ευχοιμῐ|ἐ~ευχοιην", _, "ἐ~ευχοιμεν";
    perfect_opt_act_2: "ἐ~ευχοις|ἐ~ευχοιης", "ἐ~ευχοιτον", "ἐ~ευχοιτε";
    perfect_opt_act_3: "ἐ~ευχοι|ἐ~ευχοιη", "ἐ~ευχοιτην", "ἐ~ευχοιεν";
    perfect_imp_act_2: "ἐ~ευχε", "ἐ~ευχετον", "ἐ~ευχετε";
    perfect_imp_act_3: "ἐ~ευχετω", "ἐ~ευχετων", "ἐ~ευχοντων";
    perfect_imp_mid_2: "ἐ~ευξο", _, _;
    perfect_imp_mid_3: "ἐ~ευχθω", "ἐ~ευχθων", _;
    perfect_imp_pass_2: "ἐ~ευξο", _, _;
    perfect_imp_pass_3: "ἐ~ευχθω", "ἐ~ευχθων", _;
    pluperfect_act_1: "ἐ~ευχειν|ἐ~ευχη", _, "ἐ~ευχεμεν";
    pluperfect_act_2: "ἐ~ευχεις|ἐ~ευχης", "ἐ~ευχετον", "ἐ~ευχετε";
    pluperfect_act_3: "ἐ~ευχει|ἐ~ευχειν", "ἐ~ευχετην", "ἐ~ευχεσᾰν";
    pluperfect_mid_1: "ἐ~ευγμην", _, "ἐ~ευγμεθᾰ";
    pluperfect_mid_2: "ἐ~ευξο", "ἐ~ευχθον", "ἐ~ευχθε";
    pluperfect_mid_3: "ἐ~ευκτο", "ἐ~ευχθην", _;
    pluperfect_pass_1: _, _, "ἐ~ευγμεθᾰ";
    pluperfect_pass_2: _, _, "ἐ~ευχθε";
    pluperfect_pass_3: _, "ἐ~ευχθην", "ἐ~ευγᾰτο";
    infinitive_pres: "~ευγνῠναι|~ευγνυειν", "~ευγνῠσθαι|~ευγνυεσθαι", "~ευγνῠσθαι|~ευγνυεσθαι";
    infinitive_fut: "~ευξειν", _, _;
    infinitive_aor: "~εῡξαι", _, "~ευχθηναι|~υγηναι";
    infinitive_perf: "ἐ~ευχεναι", "ἐ~εῡχθαι", _;
    participle_pres: "~ευγνῡς|~ευγνυων", "~ευγνῠμενος|~ευγνυομενος", "~ευγνῠμενος|~ευγνυομενος";
    participle_fut: "~ευξων", _, _;
    participle_aor: "~ευξᾱς", _, "~ευχθεις|~υγεις";
    participle_perf: "ἐ~ευχως", "ἐ~ευγμενος", _;
    ),
    // Ἀρνυμαι: 1 words (ἀρνυμαι). Like Ἑπομαι, except in 57 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἀρνυμαι", 1, "~νυμαι", like Ἑπομαι;
    ind_act_1: "~νυμαι", _, _;
    ind_mid_1: "~νῠμαι", _, "~νῠμεθᾰ";
    ind_mid_2: "~νῠσαι", "~νῠσθον", "~νῠσθε";
    ind_mid_3: "~νῠται", _, "~νῠνται";
    ind_pass_1: _, _, "~νῠμεθᾰ";
    ind_pass_2: _, _, "~νῠσθε";
    ind_pass_3: "~νῠται", _, "~νῠνται";
    subj_mid_1: "~νῠωμαι", _, "~νῠωμεθᾰ";
    subj_mid_2: "~νῠῃ", "~νῠησθον", "~νῠησθε";
    subj_mid_3: "~νῠηται", _, "~νῠωνται";
    subj_pass_1: _, _, "~νῠωμεθᾰ";
    subj_pass_2: _, _, "~νῠησθε";
    subj_pass_3: "~νῠηται", _, "~νῠωνται";
    opt_mid_1: "~νῠοιμην", _, "~νῠοιμεθᾰ";
    opt_mid_2: "~νῠοιο", "~νῠοισθον", "~νῠοισθε";
    opt_mid_3: "~νῠοιτο", "~νῠοισθην", "~νῠοιντο";
    opt_pass_1: _, _, "~νῠοιμεθᾰ";
    opt_pass_2: _, "~νῠοισθον", "~νῠοισθε";
    opt_pass_3: "~νῠοιτο", "~νῠοισθην", "~νῠοιντο";
    imp_mid_2: "~νῠσο", _, "~νῠσθε";
    imp_mid_3: "~νῠσθω", "~νῠσθων", "~νῠσθων";
    imp_pass_2: _, _, "~νῠσθε";
    imp_pass_3: _, "~νῠσθων", _;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, derived;
    imperfect_mid_3: derived, derived, derived;
    imperfect_pass_1: _, _, derived;
    imperfect_pass_2: _, _, derived;
    imperfect_pass_3: derived, _, _;
    infinitive_pres: _, "~νῠσθαι", _;
    participle_pres: _, "~νῠμενος", _;
    ),
    // Εἰδον: 1 words (εἰδον). Like Ἐφᾰγον, except in 74 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("εἰδον", 1, "ε~ον", like Ἐφᾰγον;
    ind_act_1: "ε~ον", _, _;
    future_ind_act_1: "~ησω", _, "~ησομεν";
    future_ind_act_2: "~ησεις", "~ησετον", "~ησετε";
    future_ind_act_3: "~ησει", _, "~ησουσῐ|~ησουσῐν";
    future_ind_mid_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_mid_2: "~ησῃ|~ησει", "~ησεσθον", "~ησεσθε";
    future_ind_mid_3: "~ησεται", _, "~ησονται";
    future_opt_act_1: "~ησοιμῐ", _, "~ησοιμεν";
    future_opt_act_2: "~ησοις", "~ησοιτον", "~ησοιτε";
    future_opt_act_3: "~ησοι", "~ησοιτην", "~ησοιεν";
    future_opt_mid_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_mid_2: "~ησοιο", "~ησοισθον", "~ησοισθε";
    future_opt_mid_3: "~ησοιτο", "~ησοισθην", "~ησοιντο";
    aorist_ind_act_1: "ε~ον", _, "ε~ομεν";
    aorist_ind_act_2: "ε~ες", "ε~ετον", "ε~ετε";
    aorist_ind_act_3: "ε~ε|ε~εν", "ε~ετην", "ε~ον";
    aorist_ind_mid_1: "ε~ομην", _, "ε~ομεθᾰ";
    aorist_ind_mid_2: "ε~ου", "ε~εσθον", "ε~εσθε";
    aorist_ind_mid_3: "ε~ετο", "ε~εσθην", "ε~οντο";
    aorist_subj_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    aorist_subj_mid_2: "~ῃ", "~ησθον", "~ησθε";
    aorist_subj_mid_3: "~ηται", _, "~ωνται";
    aorist_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    aorist_opt_mid_2: "~οιο", "~οισθον", "~οισθε";
    aorist_opt_mid_3: "~οιτο", "~οισθην", "~οιντο";
    aorist_imp_act_2: "~ε|~ε", _, _;
    aorist_imp_mid_2: "~οῡ", "~εσθον", "~εσθε";
    aorist_imp_mid_3: "~εσθω", "~εσθων", _;
    infinitive_fut: "~ησειν", "~ησεσθαι", _;
    infinitive_aor: _, "~εσθαι", _;
    participle_fut: "~ησων", "~ησομενος", _;
    participle_aor: _, "~ομενος", _;
    ),
    // Ἐπεφνον: 1 words (ἐπεφνον). Like Ἐφᾰγον, except in 66 cells: ind_act_1 sg, future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, future_ind_pass_2 du, future_ind_pass_2 pl, ….
    conjugation!("ἐπεφνον", 1, "ἐ~νον", like Ἐφᾰγον;
    ind_act_1: "ἐ~νον", _, _;
    future_ind_pass_1: "~ησομαι", _, "~ησομεθᾰ";
    future_ind_pass_2: "~ησῃ|~ησει", "~ησεσθον", "~ησεσθε";
    future_ind_pass_3: "~ησεται", _, "~ησονται";
    future_opt_pass_1: "~ησοιμην", _, "~ησοιμεθᾰ";
    future_opt_pass_2: "~ησοιο", "~ησοισθον", "~ησοισθε";
    future_opt_pass_3: "~ησοιτο", "~ησοισθην", "~ησοιντο";
    aorist_ind_act_1: "ἐ~νον", _, "ἐ~νομεν";
    aorist_ind_act_2: "ἐ~νες", "ἐ~νετον", "ἐ~νετε";
    aorist_ind_act_3: "ἐ~νε|ἐ~νεν", "ἐ~νετην", "ἐ~νον";
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
    perfect_ind_pass_1: _, _, "~αμεθᾰ";
    perfect_ind_pass_2: _, _, "~ασθε";
    perfect_imp_mid_2: "~ασο", _, _;
    perfect_imp_mid_3: "~ασθω", "~ασθων", _;
    perfect_imp_pass_2: "~ασο", _, _;
    perfect_imp_pass_3: "~ασθω", "~ασθων", _;
    infinitive_fut: _, _, "~ησεσθαι";
    infinitive_aor: "~νεῑν", _, _;
    infinitive_perf: _, "~ᾱσθαι", _;
    participle_fut: _, _, "~ησομενος";
    participle_aor: "~νων", _, _;
    participle_perf: _, "~αμενος", _;
    ),
    // Μετεκῑᾰθον: 1 words (μετεκῑᾰθον). Like Κᾰθημαι, except in 48 cells: ind_act_1 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_2 pl, ….
    conjugation!("μετεκῑᾰθον", 1, "~ε~ον", like Κᾰθημαι;
    ind_act_1: "~ε~ον", _, _;
    aorist_ind_act_1: "~ε~ον", _, "~ε~ομεν";
    aorist_ind_act_2: "~ε~ες", "~ε~ετον", "~ε~ετε";
    aorist_ind_act_3: "~ε~ε|~ε~εν", "~ε~ετην", "~ε~ον";
    aorist_subj_act_1: "~α~ω", _, "~α~ωμεν";
    aorist_subj_act_2: "~α~ῃς", "~α~ητον", "~α~ητε";
    aorist_subj_act_3: "~α~ῃ", _, "~α~ωσῐ|~α~ωσῐν";
    aorist_opt_act_1: "~α~οιμῐ", _, "~α~οιμεν";
    aorist_opt_act_2: "~α~οις", "~α~οιτον", "~α~οιτε";
    aorist_opt_act_3: "~α~οι", "~α~οιτην", "~α~οιεν";
    aorist_imp_act_2: "~α~ε", "~α~ετον", "~α~ετε";
    aorist_imp_act_3: "~α~ετω", "~α~ετων", "~α~οντων";
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    pluperfect_pass_1: derived, _, derived;
    pluperfect_pass_2: derived, derived, derived;
    pluperfect_pass_3: derived, derived, derived;
    infinitive_aor: "~α~εῑν", _, _;
    participle_aor: "~α~ων", _, _;
    ),
    // Ὀλλῡμῐ: 1 words (ὀλλῡμῐ). Like Ἑπομαι, except in 217 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ὀλλῡμῐ", 1, "ὀ~λῡμῐ", like Ἑπομαι;
    ind_act_1: "ὀ~λῡμῐ", _, "ὀ~λῠμεν";
    ind_act_2: "ὀ~λῡς", _, _;
    ind_act_3: "ὀ~λῡσῐ|ὀ~λῡσῐν", _, "ὀ~λῠᾱσῐ|ὀ~λῠᾱσῐν";
    ind_mid_1: "ὀ~λῠμαι", _, "ὀ~λῠμεθᾰ";
    ind_mid_2: "ὀ~λῠσαι", "ὀ~λῠσθον", "ὀ~λῠσθε";
    ind_mid_3: "ὀ~λῠται", _, "ὀ~λῠνται";
    ind_pass_1: _, _, "ὀ~λῠμεθᾰ";
    ind_pass_2: _, _, "ὀ~λῠσθε";
    ind_pass_3: "ὀ~λῠται", _, "ὀ~λῠνται";
    subj_act_1: "ὀ~λῠω", _, "ὀ~λῠωμεν";
    subj_act_2: "ὀ~λῠῃς", "ὀ~λῠητον", "ὀ~λῠητε";
    subj_act_3: "ὀ~λῠῃ", _, "ὀ~λῠωσῐ|ὀ~λῠωσῐν";
    subj_mid_1: "ὀ~λῠωμαι", _, "ὀ~λῠωμεθᾰ";
    subj_mid_2: "ὀ~λῠῃ", "ὀ~λῠησθον", "ὀ~λῠησθε";
    subj_mid_3: "ὀ~λῠηται", _, "ὀ~λῠωνται";
    subj_pass_1: _, _, "ὀ~λῠωμεθᾰ";
    subj_pass_2: _, _, "ὀ~λῠησθε";
    subj_pass_3: "ὀ~λῠηται", _, "ὀ~λῠωνται";
    opt_act_1: "ὀ~λῠοιμῐ", _, "ὀ~λῠοιμεν";
    opt_act_2: "ὀ~λῠοις", "ὀ~λῠοιτον", "ὀ~λῠοιτε";
    opt_act_3: "ὀ~λῠοι", "ὀ~λῠοιτην", "ὀ~λῠοιεν";
    opt_mid_1: "ὀ~λῠοιμην", _, "ὀ~λῠοιμεθᾰ";
    opt_mid_2: "ὀ~λῠοιο", "ὀ~λῠοισθον", "ὀ~λῠοισθε";
    opt_mid_3: "ὀ~λῠοιτο", "ὀ~λῠοισθην", "ὀ~λῠοιντο";
    opt_pass_1: _, _, "ὀ~λῠοιμεθᾰ";
    opt_pass_2: _, "ὀ~λῠοισθον", "ὀ~λῠοισθε";
    opt_pass_3: "ὀ~λῠοιτο", "ὀ~λῠοισθην", "ὀ~λῠοιντο";
    imp_act_2: "ὀ~λῡ", "ὀ~λῠτον", "ὀ~λῠτε";
    imp_act_3: "ὀ~λῠτω", "ὀ~λῠτων", "ὀ~λῠντων";
    imp_mid_2: "ὀ~λῠσο", _, "ὀ~λῠσθε";
    imp_mid_3: "ὀ~λῠσθω", "ὀ~λῠσθων", "ὀ~λῠσθων";
    imp_pass_2: _, _, "ὀ~λῠσθε";
    imp_pass_3: _, "ὀ~λῠσθων", _;
    imperfect_act_1: "ὠ~λῡν", _, "ὠ~λῠμεν";
    imperfect_act_2: "ὠ~λῡς", "ὠ~λῠτον", "ὠ~λῠτε";
    imperfect_act_3: "ὠ~λῡ", "ὠ~λῠτην", "ὠ~λῠσᾰν";
    imperfect_mid_1: "ὠ~λῠμην", _, "ὠ~λῠμεθᾰ";
    imperfect_mid_2: "ὠ~λῠσο", "ὠ~λῠσθον", "ὠ~λῠσθε";
    imperfect_mid_3: "ὠ~λῠτο", "ὠ~λῠσθην", "ὠ~λῠντο";
    imperfect_pass_1: _, _, "ὠ~λῠμεθᾰ";
    imperfect_pass_2: _, _, "ὠ~λῠσθε";
    imperfect_pass_3: "ὠ~λῠτο", _, _;
    future_ind_act_1: "ὀ~ω", _, "ὀ~οῡμεν";
    future_ind_act_2: "ὀ~εῑς", "ὀ~εῑτον", "ὀ~εῑτε";
    future_ind_act_3: "ὀ~εῑ", _, "ὀ~οῡσῐ|ὀ~οῡσῐν";
    future_ind_mid_1: "ὀ~οῡμαι", _, "ὀ~ουμεθᾰ";
    future_ind_mid_2: "ὀ~εῑ|ὀ~ῃ", "ὀ~εῑσθον", "ὀ~εῑσθε";
    future_ind_mid_3: "ὀ~εῑται", _, "ὀ~οῡνται";
    future_opt_act_1: "ὀ~οιην|ὀ~οῑμῐ", _, "ὀ~οῑμεν|ὀ~οιημεν";
    future_opt_act_2: "ὀ~οιης|ὀ~οῑς", "ὀ~οῑτον|ὀ~οιητον", "ὀ~οῑτε|ὀ~οιητε";
    future_opt_act_3: "ὀ~οιη|ὀ~οῑ", "ὀ~οιτην|ὀ~οιητην", "ὀ~οῑεν|ὀ~οιησᾰν";
    future_opt_mid_1: "ὀ~οιμην", _, "ὀ~οιμεθᾰ";
    future_opt_mid_2: "ὀ~οῑο", "ὀ~οῑσθον", "ὀ~οῑσθε";
    future_opt_mid_3: "ὀ~οῑτο", "ὀ~οισθην", "ὀ~οῑντο";
    aorist_ind_act_1: "ὠ~εσᾰ", _, "ὠ~εσᾰμεν";
    aorist_ind_act_2: "ὠ~εσᾰς", "ὠ~εσᾰτον", "ὠ~εσᾰτε";
    aorist_ind_act_3: "ὠ~εσε|ὠ~εσεν", "ὠ~εσᾰτην", "ὠ~εσᾰν";
    aorist_ind_mid_1: "ὠ~ομην|ὀ~ομην", _, "ὠ~ομεθᾰ|ὀ~ομεθᾰ|ὀ~ομεσθᾰ";
    aorist_ind_mid_2: "ὠ~ου|ὀ~εο|ὀ~ευ", "ὠ~εσθον|ὀ~εσθον", "ὠ~εσθε|ὀ~εσθε";
    aorist_ind_mid_3: "ὠ~ετο|ὀ~ετο", "ὠ~εσθην|ὀ~εσθην", "ὠ~οντο|ὀ~οντο";
    aorist_subj_act_1: "ὀ~εσω", _, "ὀ~εσωμεν";
    aorist_subj_act_2: "ὀ~εσῃς", "ὀ~εσητον", "ὀ~εσητε";
    aorist_subj_act_3: "ὀ~εσῃ", _, "ὀ~εσωσῐ|ὀ~εσωσῐν";
    aorist_subj_mid_1: "ὀ~ωμαι", _, "ὀ~ωμεθᾰ";
    aorist_subj_mid_2: "ὀ~ῃ", "ὀ~ησθον", "ὀ~ησθε";
    aorist_subj_mid_3: "ὀ~ηται", _, "ὀ~ωνται";
    aorist_opt_act_1: "ὀ~εσαιμῐ", _, "ὀ~εσαιμεν";
    aorist_opt_act_2: "ὀ~εσειᾰς|ὀ~εσαις", "ὀ~εσαιτον", "ὀ~εσαιτε";
    aorist_opt_act_3: "ὀ~εσειε|ὀ~εσειεν|ὀ~εσαι", "ὀ~εσαιτην", "ὀ~εσειᾰν|ὀ~εσαιεν";
    aorist_opt_mid_1: "ὀ~οιμην", _, "ὀ~οιμεθᾰ";
    aorist_opt_mid_2: "ὀ~οιο", "ὀ~οισθον", "ὀ~οισθε";
    aorist_opt_mid_3: "ὀ~οιτο", "ὀ~οισθην", "ὀ~οιντο";
    aorist_imp_act_2: "ὀ~εσον", "ὀ~εσᾰτον", "ὀ~εσᾰτε";
    aorist_imp_act_3: "ὀ~εσᾰτω", "ὀ~εσᾰτων", "ὀ~εσᾰντων";
    aorist_imp_mid_2: "ὀ~οῡ", "ὀ~εσθον", "ὀ~εσθε";
    aorist_imp_mid_3: "ὀ~εσθω", "ὀ~εσθων", _;
    perfect_ind_act_1: "ὀ~ωλεκᾰ", _, "ὀ~ωλεκᾰμεν";
    perfect_ind_act_2: "ὀ~ωλεκᾰς", "ὀ~ωλεκᾰτον", "ὀ~ωλεκᾰτε";
    perfect_ind_act_3: "ὀ~ωλεκε|ὀ~ωλεκεν", _, "ὀ~ωλεκᾱσῐ|ὀ~ωλεκᾱσῐν";
    perfect_subj_act_1: "ὀ~ωλεκω", _, "ὀ~ωλεκωμεν";
    perfect_subj_act_2: "ὀ~ωλεκῃς", "ὀ~ωλεκητον", "ὀ~ωλεκητε";
    perfect_subj_act_3: "ὀ~ωλεκῃ", _, "ὀ~ωλεκωσῐ|ὀ~ωλεκωσῐν";
    perfect_opt_act_1: "ὀ~ωλεκοιμῐ|ὀ~ωλεκοιην", _, "ὀ~ωλεκοιμεν";
    perfect_opt_act_2: "ὀ~ωλεκοις|ὀ~ωλεκοιης", "ὀ~ωλεκοιτον", "ὀ~ωλεκοιτε";
    perfect_opt_act_3: "ὀ~ωλεκοι|ὀ~ωλεκοιη", "ὀ~ωλεκοιτην", "ὀ~ωλεκοιεν";
    perfect_imp_act_2: "ὀ~ωλεκε", "ὀ~ωλεκετον", "ὀ~ωλεκετε";
    perfect_imp_act_3: "ὀ~ωλεκετω", "ὀ~ωλεκετων", "ὀ~ωλεκοντων";
    infinitive_pres: "ὀ~λῠναι", "ὀ~λῠσθαι", _;
    infinitive_fut: "ὀ~εῑν", "ὀ~εῑσθαι", _;
    infinitive_aor: "ὀ~εσαι", "ὀ~εσθαι", _;
    infinitive_perf: "ὀ~ωλεκεναι", _, _;
    participle_pres: "ὀ~λῡς", "ὀ~λῠμενος", _;
    participle_fut: "ὀ~ων", "ὀ~ουμενος", _;
    participle_aor: "ὀ~εσᾱς", "ὀ~ομενος", _;
    participle_perf: "ὀ~ωλεκως", _, _;
    ),
    // Ἀνοιγνῡμῐ: 1 words (ἀνοιγνῡμῐ). Like Κᾰθημαι, except in 108 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἀνοιγνῡμῐ", 1, "~οι~ῡμῐ", like Κᾰθημαι;
    ind_act_1: "~οι~ῡμῐ", _, "~οι~ῠμεν";
    ind_act_2: "~οι~ῡς", _, _;
    ind_act_3: "~οι~ῡσῐ|~οι~ῡσῐν", _, "~οι~ῠᾱσῐ|~οι~ῠᾱσῐν";
    ind_mid_1: "~οι~ῠμαι", _, "~οι~ῠμεθᾰ";
    ind_mid_2: "~οι~ῠσαι", "~οι~ῠσθον", "~οι~ῠσθε";
    ind_mid_3: "~οι~ῠται", _, "~οι~ῠνται";
    ind_pass_1: _, _, "~οι~ῠμεθᾰ";
    ind_pass_2: _, _, "~οι~ῠσθε";
    ind_pass_3: "~οι~ῠται", _, "~οι~ῠνται";
    subj_act_1: "~οι~ῠω", _, "~οι~ῠωμεν";
    subj_act_2: "~οι~ῠῃς", "~οι~ῠητον", "~οι~ῠητε";
    subj_act_3: "~οι~ῠῃ", _, "~οι~ῠωσῐ|~οι~ῠωσῐν";
    subj_mid_1: "~οι~ῠωμαι", _, "~οι~ῠωμεθᾰ";
    subj_mid_2: "~οι~ῠῃ", "~οι~ῠησθον", "~οι~ῠησθε";
    subj_mid_3: "~οι~ῠηται", _, "~οι~ῠωνται";
    subj_pass_1: _, _, "~οι~ῠωμεθᾰ";
    subj_pass_2: _, _, "~οι~ῠησθε";
    subj_pass_3: "~οι~ῠηται", _, "~οι~ῠωνται";
    opt_act_1: "~οι~ῠοιμῐ", _, "~οι~ῠοιμεν";
    opt_act_2: "~οι~ῠοις", "~οι~ῠοιτον", "~οι~ῠοιτε";
    opt_act_3: "~οι~ῠοι", "~οι~ῠοιτην", "~οι~ῠοιεν";
    opt_mid_1: "~οι~ῠοιμην", _, "~οι~ῠοιμεθᾰ";
    opt_mid_2: "~οι~ῠοιο", "~οι~ῠοισθον", "~οι~ῠοισθε";
    opt_mid_3: "~οι~ῠοιτο", "~οι~ῠοισθην", "~οι~ῠοιντο";
    opt_pass_1: _, _, "~οι~ῠοιμεθᾰ";
    opt_pass_2: _, "~οι~ῠοισθον", "~οι~ῠοισθε";
    opt_pass_3: "~οι~ῠοιτο", "~οι~ῠοισθην", "~οι~ῠοιντο";
    imp_act_2: "~οι~ῡ", "~οι~ῠτον", "~οι~ῠτε";
    imp_act_3: "~οι~ῠτω", "~οι~ῠτων", "~οι~ῠντων";
    imp_mid_2: "~οι~ῠσο", _, "~οι~ῠσθε";
    imp_mid_3: "~οι~ῠσθω", "~οι~ῠσθων", "~οι~ῠσθων";
    imp_pass_2: _, _, "~οι~ῠσθε";
    imp_pass_3: _, "~οι~ῠσθων", _;
    imperfect_act_1: "~εῳ~ῡν", _, "~εῳ~ῠμεν";
    imperfect_act_2: "~εῳ~ῡς", "~εῳ~ῠτον", "~εῳ~ῠτε";
    imperfect_act_3: "~εῳ~ῡ", "~εῳ~ῠτην", "~εῳ~ῠσᾰν";
    imperfect_mid_1: "~εῳ~ῠμην", _, "~εῳ~ῠμεθᾰ";
    imperfect_mid_2: "~εῳ~ῠσο", "~εῳ~ῠσθον", "~εῳ~ῠσθε";
    imperfect_mid_3: "~εῳ~ῠτο", "~εῳ~ῠσθην", "~εῳ~ῠντο";
    imperfect_pass_1: _, _, "~εῳ~ῠμεθᾰ";
    imperfect_pass_2: _, _, "~εῳ~ῠσθε";
    imperfect_pass_3: "~εῳ~ῠτο", _, _;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    pluperfect_pass_1: derived, _, derived;
    pluperfect_pass_2: derived, derived, derived;
    pluperfect_pass_3: derived, derived, derived;
    infinitive_pres: "~οι~ῠναι", "~οι~ῠσθαι", _;
    participle_pres: "~οι~ῡς", "~οι~ῠμενος", _;
    ),
    // Οἰγνῡμῐ: 1 words (οἰγνῡμῐ). Like Θωᾰω, except in 92 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("οἰγνῡμῐ", 1, "οἰ~ῡμῐ", like Θωᾰω;
    ind_act_1: "οἰ~ῡμῐ", _, "οἰ~ῠμεν";
    ind_act_2: "οἰ~ῡς", _, _;
    ind_act_3: "οἰ~ῡσῐ|οἰ~ῡσῐν", _, "οἰ~ῠᾱσῐ|οἰ~ῠᾱσῐν";
    ind_mid_1: "οἰ~ῠμαι", _, "οἰ~ῠμεθᾰ";
    ind_mid_2: "οἰ~ῠσαι", "οἰ~ῠσθον", "οἰ~ῠσθε";
    ind_mid_3: "οἰ~ῠται", _, "οἰ~ῠνται";
    ind_pass_1: _, _, "οἰ~ῠμεθᾰ";
    ind_pass_2: _, _, "οἰ~ῠσθε";
    ind_pass_3: "οἰ~ῠται", _, "οἰ~ῠνται";
    subj_act_1: "οἰ~ῠω", _, "οἰ~ῠωμεν";
    subj_act_2: "οἰ~ῠῃς", "οἰ~ῠητον", "οἰ~ῠητε";
    subj_act_3: "οἰ~ῠῃ", _, "οἰ~ῠωσῐ|οἰ~ῠωσῐν";
    subj_mid_1: "οἰ~ῠωμαι", _, "οἰ~ῠωμεθᾰ";
    subj_mid_2: "οἰ~ῠῃ", "οἰ~ῠησθον", "οἰ~ῠησθε";
    subj_mid_3: "οἰ~ῠηται", _, "οἰ~ῠωνται";
    subj_pass_1: _, _, "οἰ~ῠωμεθᾰ";
    subj_pass_2: _, _, "οἰ~ῠησθε";
    subj_pass_3: "οἰ~ῠηται", _, "οἰ~ῠωνται";
    opt_act_1: "οἰ~ῠοιμῐ", _, "οἰ~ῠοιμεν";
    opt_act_2: "οἰ~ῠοις", "οἰ~ῠοιτον", "οἰ~ῠοιτε";
    opt_act_3: "οἰ~ῠοι", "οἰ~ῠοιτην", "οἰ~ῠοιεν";
    opt_mid_1: "οἰ~ῠοιμην", _, "οἰ~ῠοιμεθᾰ";
    opt_mid_2: "οἰ~ῠοιο", "οἰ~ῠοισθον", "οἰ~ῠοισθε";
    opt_mid_3: "οἰ~ῠοιτο", "οἰ~ῠοισθην", "οἰ~ῠοιντο";
    opt_pass_1: _, _, "οἰ~ῠοιμεθᾰ";
    opt_pass_2: _, "οἰ~ῠοισθον", "οἰ~ῠοισθε";
    opt_pass_3: "οἰ~ῠοιτο", "οἰ~ῠοισθην", "οἰ~ῠοιντο";
    imp_act_2: "οἰ~ῡ", "οἰ~ῠτον", "οἰ~ῠτε";
    imp_act_3: "οἰ~ῠτω", "οἰ~ῠτων", "οἰ~ῠντων";
    imp_mid_2: "οἰ~ῠσο", _, "οἰ~ῠσθε";
    imp_mid_3: "οἰ~ῠσθω", "οἰ~ῠσθων", "οἰ~ῠσθων";
    imp_pass_2: _, _, "οἰ~ῠσθε";
    imp_pass_3: _, "οἰ~ῠσθων", _;
    imperfect_act_1: "ᾠ~ῡν", _, "ᾠ~ῠμεν";
    imperfect_act_2: "ᾠ~ῡς", "ᾠ~ῠτον", "ᾠ~ῠτε";
    imperfect_act_3: "ᾠ~ῡ", "ᾠ~ῠτην", "ᾠ~ῠσᾰν";
    imperfect_mid_1: "ᾠ~ῠμην", _, "ᾠ~ῠμεθᾰ";
    imperfect_mid_2: "ᾠ~ῠσο", "ᾠ~ῠσθον", "ᾠ~ῠσθε";
    imperfect_mid_3: "ᾠ~ῠτο", "ᾠ~ῠσθην", "ᾠ~ῠντο";
    imperfect_pass_1: _, _, "ᾠ~ῠμεθᾰ";
    imperfect_pass_2: _, _, "ᾠ~ῠσθε";
    imperfect_pass_3: "ᾠ~ῠτο", _, _;
    infinitive_pres: "οἰ~ῠναι", "οἰ~ῠσθαι", _;
    participle_pres: "οἰ~ῡς", "οἰ~ῠμενος", _;
    ),
    // Πεπνῡμαι: 1 words (πεπνῡμαι). Like Ἠμῐ, except in 33 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, aorist_ind_pass_1 sg, aorist_ind_pass_1 pl, ….
    conjugation!("πεπνῡμαι", 1, "πε~μαι", like Ἠμῐ;
    ind_act_1: "πε~μαι", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    aorist_ind_pass_1: "ἐ~θην", _, "ἐ~θημεν";
    aorist_ind_pass_2: "ἐ~θης", "ἐ~θητον", "ἐ~θητε";
    aorist_ind_pass_3: "ἐ~θη", "ἐ~θητην", "ἐ~θησᾰν";
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
    // Περνημῐ: 1 words (περνημῐ). Like Οἰδᾰ, except in 52 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("περνημῐ", 1, "~νημῐ", like Οἰδᾰ;
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
    // Κρεμᾰννῡμῐ: 1 words (κρεμᾰννῡμῐ). Like Φημῐ, except in 308 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("κρεμᾰννῡμῐ", 1, "~ᾰννῡμῐ", like Φημῐ;
    ind_act_1: "~ᾰννῡμῐ", _, "~ᾰννῠμεν";
    ind_act_2: "~ᾰννῡς", _, _;
    ind_act_3: "~ᾰννῡσῐ|~ᾰννῡσῐν", _, "~ᾰννῠᾱσῐ|~ᾰννῠᾱσῐν";
    ind_mid_1: "~ᾰννῠμαι", _, "~ᾰννῠμεθᾰ";
    ind_mid_2: "~ᾰννῠσαι", "~ᾰννῠσθον", "~ᾰννῠσθε";
    ind_mid_3: "~ᾰννῠται", _, "~ᾰννῠνται";
    ind_pass_1: "~ᾰννῠμαι|~ᾰμαι", _, "~ᾰννῠμεθᾰ|~ᾰμεθᾰ";
    ind_pass_2: "~ᾰννῠσαι|~ᾰσαι", "~ᾰννῠσθον|~ᾰσθον", "~ᾰννῠσθε|~ᾰσθε";
    ind_pass_3: "~ᾰννῠται|~ᾰται", "~ᾰννῠσθον|~ᾰσθον", "~ᾰννῠνται|~ᾰνται";
    subj_act_1: "~ᾰννῠω", _, "~ᾰννῠωμεν";
    subj_act_2: "~ᾰννῠῃς", "~ᾰννῠητον", "~ᾰννῠητε";
    subj_act_3: "~ᾰννῠῃ", _, "~ᾰννῠωσῐ|~ᾰννῠωσῐν";
    subj_mid_1: "~ᾰννῠωμαι", _, "~ᾰννῠωμεθᾰ";
    subj_mid_2: "~ᾰννῠῃ", "~ᾰννῠησθον", "~ᾰννῠησθε";
    subj_mid_3: "~ᾰννῠηται", _, "~ᾰννῠωνται";
    subj_pass_1: "~ᾰννῠωμαι|~ωμαι", _, "~ᾰννῠωμεθᾰ|~ωμεθᾰ";
    subj_pass_2: "~ᾰννῠῃ|~ῃ", "~ᾰννῠησθον|~ησθον", "~ᾰννῠησθε|~ησθε";
    subj_pass_3: "~ᾰννῠηται|~ηται", "~ᾰννῠησθον|~ησθον", "~ᾰννῠωνται|~ωνται";
    opt_act_1: "~ᾰννῠοιμῐ", _, "~ᾰννῠοιμεν";
    opt_act_2: "~ᾰννῠοις", "~ᾰννῠοιτον", "~ᾰννῠοιτε";
    opt_act_3: "~ᾰννῠοι", "~ᾰννῠοιτην", "~ᾰννῠοιεν";
    opt_mid_1: "~ᾰννῠοιμην", _, "~ᾰννῠοιμεθᾰ";
    opt_mid_2: "~ᾰννῠοιο", "~ᾰννῠοισθον", "~ᾰννῠοισθε";
    opt_mid_3: "~ᾰννῠοιτο", "~ᾰννῠοισθην", "~ᾰννῠοιντο";
    opt_pass_1: "~ᾰννῠοιμην|~αιμην", _, "~ᾰννῠοιμεθᾰ|~αιμεθᾰ";
    opt_pass_2: "~ᾰννῠοιο|~αιο", "~ᾰννῠοισθον|~αισθον", "~ᾰννῠοισθε|~αισθε";
    opt_pass_3: "~ᾰννῠοιτο|~αιτο", "~ᾰννῠοισθην|~αισθην", "~ᾰννῠοιντο|~αιντο";
    imp_act_2: "~ᾰννῡ", "~ᾰννῠτον", "~ᾰννῠτε";
    imp_act_3: "~ᾰννῠτω", "~ᾰννῠτων", "~ᾰννῠντων";
    imp_mid_2: "~ᾰννῠσο", _, "~ᾰννῠσθε";
    imp_mid_3: "~ᾰννῠσθω", "~ᾰννῠσθων", "~ᾰννῠσθων";
    imp_pass_2: "~ᾰννῠσο|~ᾰσο|~ω", "~ᾰννῠσθον|~ᾰσθον", "~ᾰννῠσθε|~ᾰσθε";
    imp_pass_3: "~ᾰννῠσθω|~ᾰσθω", "~ᾰννῠσθων|~ᾰσθων", "~ᾰννῠσθων|~ᾰσθων";
    imperfect_act_1: "ἐ~ᾰννῡν", _, "ἐ~ᾰννῠμεν";
    imperfect_act_2: "ἐ~ᾰννῡς", "ἐ~ᾰννῠτον", "ἐ~ᾰννῠτε";
    imperfect_act_3: "ἐ~ᾰννῡ", "ἐ~ᾰννῠτην", "ἐ~ᾰννῠσᾰν";
    imperfect_mid_1: "ἐ~ᾰννῠμην", _, "ἐ~ᾰννῠμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰννῠσο", "ἐ~ᾰννῠσθον", "ἐ~ᾰννῠσθε";
    imperfect_mid_3: "ἐ~ᾰννῠτο", "ἐ~ᾰννῠσθην", "ἐ~ᾰννῠντο";
    imperfect_pass_1: "ἐ~ᾰννῠμην|ἐ~ᾰμην", _, "ἐ~ᾰννῠμεθᾰ|ἐ~ᾰμεθᾰ";
    imperfect_pass_2: "ἐ~ᾰννῠσο|ἐ~ᾰσο", "ἐ~ᾰννῠσθον|ἐ~ᾰσθον", "ἐ~ᾰννῠσθε|ἐ~ᾰσθε";
    imperfect_pass_3: "ἐ~ᾰννῠτο|ἐ~ᾰτο", "ἐ~ᾰννῠσθην|ἐ~ᾰσθην", "ἐ~ᾰννῠντο|ἐ~ᾰντο";
    future_ind_act_1: "~ᾰσω|~ησω", _, "~ᾰσομεν|~ησομεν";
    future_ind_act_2: "~ᾰσεις|~ησεις", "~ᾰσετον|~ησετον", "~ᾰσετε|~ησετε";
    future_ind_act_3: "~ᾰσει|~ησει", "~ᾰσετον|~ησετον", "~ᾰσουσῐ|~ᾰσουσῐν|~ησουσῐ|~ησουσῐν";
    future_ind_mid_1: "~ᾰσομαι|~ησομαι", _, "~ᾰσομεθᾰ|~ησομεθᾰ";
    future_ind_mid_2: "~ᾰσῃ|~ᾰσει|~ησῃ|~ησει", "~ᾰσεσθον|~ησεσθον", "~ᾰσεσθε|~ησεσθε";
    future_ind_mid_3: "~ᾰσεται|~ησεται", "~ᾰσεσθον|~ησεσθον", "~ᾰσονται|~ησονται";
    future_ind_pass_1: "~ᾰσθησομαι", _, "~ᾰσθησομεθᾰ";
    future_ind_pass_2: "~ᾰσθησῃ|~ᾰσθησει", "~ᾰσθησεσθον", "~ᾰσθησεσθε";
    future_ind_pass_3: "~ᾰσθησεται", _, "~ᾰσθησονται";
    future_opt_act_1: "~ᾰσοιμῐ|~ησοιμῐ", _, "~ᾰσοιμεν|~ησοιμεν";
    future_opt_act_2: "~ᾰσοις|~ησοις", "~ᾰσοιτον|~ησοιτον", "~ᾰσοιτε|~ησοιτε";
    future_opt_act_3: "~ᾰσοι|~ησοι", "~ᾰσοιτην|~ησοιτην", "~ᾰσοιεν|~ησοιεν";
    future_opt_mid_1: "~ᾰσοιμην|~ησοιμην", _, "~ᾰσοιμεθᾰ|~ησοιμεθᾰ";
    future_opt_mid_2: "~ᾰσοιο|~ησοιο", "~ᾰσοισθον|~ησοισθον", "~ᾰσοισθε|~ησοισθε";
    future_opt_mid_3: "~ᾰσοιτο|~ησοιτο", "~ᾰσοισθην|~ησοισθην", "~ᾰσοιντο|~ησοιντο";
    future_opt_pass_1: "~ᾰσθησοιμην", _, "~ᾰσθησοιμεθᾰ";
    future_opt_pass_2: "~ᾰσθησοιο", "~ᾰσθησοισθον", "~ᾰσθησοισθε";
    future_opt_pass_3: "~ᾰσθησοιτο", "~ᾰσθησοισθην", "~ᾰσθησοιντο";
    aorist_ind_act_1: "ἐ~ᾰσᾰ", _, "ἐ~ᾰσᾰμεν";
    aorist_ind_act_2: "ἐ~ᾰσᾰς", "ἐ~ᾰσᾰτον", "ἐ~ᾰσᾰτε";
    aorist_ind_act_3: "ἐ~ᾰσε|ἐ~ᾰσεν", "ἐ~ᾰσᾰτην", "ἐ~ᾰσᾰν";
    aorist_ind_mid_1: "ἐ~ᾰσᾰμην", _, "ἐ~ᾰσᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ᾰσω", "ἐ~ᾰσᾰσθον", "ἐ~ᾰσᾰσθε";
    aorist_ind_mid_3: "ἐ~ᾰσᾰτο", "ἐ~ᾰσᾰσθην", "ἐ~ᾰσᾰντο";
    aorist_ind_pass_1: "ἐ~ᾰσθην", _, "ἐ~ᾰσθημεν";
    aorist_ind_pass_2: "ἐ~ᾰσθης", "ἐ~ᾰσθητον", "ἐ~ᾰσθητε";
    aorist_ind_pass_3: "ἐ~ᾰσθη", "ἐ~ᾰσθητην", "ἐ~ᾰσθησᾰν";
    aorist_subj_act_1: "~ᾰσω", _, "~ᾰσωμεν";
    aorist_subj_act_2: "~ᾰσῃς", "~ᾰσητον", "~ᾰσητε";
    aorist_subj_act_3: "~ᾰσῃ", _, "~ᾰσωσῐ|~ᾰσωσῐν";
    aorist_subj_mid_1: "~ᾰσωμαι", _, "~ᾰσωμεθᾰ";
    aorist_subj_mid_2: "~ᾰσῃ", "~ᾰσησθον", "~ᾰσησθε";
    aorist_subj_mid_3: "~ᾰσηται", _, "~ᾰσωνται";
    aorist_subj_pass_1: "~ᾰσθω", _, "~ᾰσθωμεν";
    aorist_subj_pass_2: "~ᾰσθῃς", "~ᾰσθητον", "~ᾰσθητε";
    aorist_subj_pass_3: "~ᾰσθῃ", _, "~ᾰσθωσῐ|~ᾰσθωσῐν";
    aorist_opt_act_1: "~ᾰσαιμῐ", _, "~ᾰσαιμεν";
    aorist_opt_act_2: "~ᾰσειᾰς|~ᾰσαις", "~ᾰσαιτον", "~ᾰσαιτε";
    aorist_opt_act_3: "~ᾰσειε|~ᾰσειεν|~ᾰσαι", "~ᾰσαιτην", "~ᾰσειᾰν|~ᾰσαιεν";
    aorist_opt_mid_1: "~ᾰσαιμην", _, "~ᾰσαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰσαιο", "~ᾰσαισθον", "~ᾰσαισθε";
    aorist_opt_mid_3: "~ᾰσαιτο", "~ᾰσαισθην", "~ᾰσαιντο";
    aorist_opt_pass_1: "~ᾰσθειην", _, "~ᾰσθεῑμεν|~ᾰσθειημεν";
    aorist_opt_pass_2: "~ᾰσθειης", "~ᾰσθεῑτον|~ᾰσθειητον", "~ᾰσθεῑτε|~ᾰσθειητε";
    aorist_opt_pass_3: "~ᾰσθειη", "~ᾰσθειτην|~ᾰσθειητην", "~ᾰσθεῑεν|~ᾰσθειησᾰν";
    aorist_imp_act_2: "~ᾰσον", "~ᾰσᾰτον", "~ᾰσᾰτε";
    aorist_imp_act_3: "~ᾰσᾰτω", "~ᾰσᾰτων", "~ᾰσᾰντων";
    aorist_imp_mid_2: "~ᾰσαι", "~ᾰσᾰσθον", "~ᾰσᾰσθε";
    aorist_imp_mid_3: "~ᾰσᾰσθω", "~ᾰσᾰσθων", _;
    aorist_imp_pass_2: "~ᾰσθητῐ", _, _;
    aorist_imp_pass_3: "~ᾰσθητω", "~ᾰσθητων", "~ᾰσθεντων";
    perfect_ind_act_1: "%ε~ᾰκᾰ", _, "%ε~ᾰκᾰμεν";
    perfect_ind_act_2: "%ε~ᾰκᾰς", "%ε~ᾰκᾰτον", "%ε~ᾰκᾰτε";
    perfect_ind_act_3: "%ε~ᾰκε|%ε~ᾰκεν", _, "%ε~ᾰκᾱσῐ|%ε~ᾰκᾱσῐν";
    perfect_ind_mid_1: "%ε~ᾰσμαι", _, "%ε~ᾰσμεθᾰ";
    perfect_ind_mid_3: "%ε~ᾰσται", _, "%ε~ᾰσᾰται";
    perfect_ind_pass_1: _, _, "%ε~ᾰσμεθᾰ";
    perfect_subj_act_1: "%ε~ᾰκω", _, "%ε~ᾰκωμεν";
    perfect_subj_act_2: "%ε~ᾰκῃς", "%ε~ᾰκητον", "%ε~ᾰκητε";
    perfect_subj_act_3: "%ε~ᾰκῃ", _, "%ε~ᾰκωσῐ|%ε~ᾰκωσῐν";
    perfect_opt_act_1: "%ε~ᾰκοιμῐ|%ε~ᾰκοιην", _, "%ε~ᾰκοιμεν";
    perfect_opt_act_2: "%ε~ᾰκοις|%ε~ᾰκοιης", "%ε~ᾰκοιτον", "%ε~ᾰκοιτε";
    perfect_opt_act_3: "%ε~ᾰκοι|%ε~ᾰκοιη", "%ε~ᾰκοιτην", "%ε~ᾰκοιεν";
    perfect_imp_act_2: "%ε~ᾰκε", "%ε~ᾰκετον", "%ε~ᾰκετε";
    perfect_imp_act_3: "%ε~ᾰκετω", "%ε~ᾰκετων", "%ε~ᾰκοντων";
    pluperfect_act_1: "%ε~ᾰκειν|%ε~ᾰκη", _, "%ε~ᾰκεμεν";
    pluperfect_act_2: "%ε~ᾰκεις|%ε~ᾰκης", "%ε~ᾰκετον", "%ε~ᾰκετε";
    pluperfect_act_3: "%ε~ᾰκει|%ε~ᾰκειν", "%ε~ᾰκετην", "%ε~ᾰκεσᾰν";
    pluperfect_mid_1: "%ε~ᾰσμην", _, "%ε~ᾰσμεθᾰ";
    pluperfect_mid_2: "%ε~ᾰσο", "%ε~ᾰσθον", "%ε~ᾰσθε";
    pluperfect_mid_3: "%ε~ᾰστο", "%ε~ᾰσθην", _;
    pluperfect_pass_1: _, _, "%ε~ᾰσμεθᾰ";
    pluperfect_pass_2: _, _, "%ε~ᾰσθε";
    pluperfect_pass_3: _, "%ε~ᾰσθην", "%ε~ᾰσᾰτο";
    infinitive_pres: "~ᾰννῠναι", "~ᾰννῠσθαι", _;
    infinitive_fut: "~ᾰσειν|~ησειν", "~ᾰσεσθαι|~ησεσθαι", "~ᾰσθησεσθαι";
    infinitive_aor: "~ᾰσαι", "~ᾰσᾰσθαι", "~ᾰσθηναι";
    infinitive_perf: "%ε~ᾰκεναι", _, _;
    participle_pres: "~ᾰννῡς", "~ᾰννῠμενος", _;
    participle_fut: "~ᾰσων|~ησων", "~ᾰσομενος|~ησομενος", "~ᾰσθησομενος";
    participle_aor: "~ᾰσᾱς", "~ᾰσᾰμενος", "~ᾰσθεις";
    participle_perf: "%ε~ᾰκως", _, _;
    ),
    // Ἐπῐστᾰμαι: 1 words (ἐπῐστᾰμαι). Like Ἐγκειμαι, except in 82 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἐπῐστᾰμαι", 1, "ἐ~ῐ~ᾰμαι", like Ἐγκειμαι;
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
    imp_mid_2: "ἐ~ι~ᾰσο|ἐ~ι~ω", _, "ἐ~ι~ᾰσθε";
    imp_mid_3: "ἐ~ι~ᾰσθω", "ἐ~ι~ᾰσθων", "ἐ~ι~ᾰσθων";
    imperfect_mid_1: "ἠ~ι~ᾰμην", _, "ἠ~ι~ᾰμεθᾰ";
    imperfect_mid_2: "ἠ~ῐ~ω|ἠ~ῐ~ᾰσο", "ἠ~ι~ᾰσθον", "ἠ~ι~ᾰσθε";
    imperfect_mid_3: "ἠ~ι~ᾰτο", "ἠ~ι~ᾰσθην", "ἠ~ι~ᾰντο";
    future_ind_mid_1: "ἐ~ι~ησομαι", _, "ἐ~ι~ησομεθᾰ";
    future_ind_mid_2: "ἐ~ι~ησῃ|ἐ~ι~ησει", "ἐ~ι~ησεσθον", "ἐ~ι~ησεσθε";
    future_ind_mid_3: "ἐ~ι~ησεται", _, "ἐ~ι~ησονται";
    future_opt_mid_1: "ἐ~ι~ησοιμην", _, "ἐ~ι~ησοιμεθᾰ";
    future_opt_mid_2: "ἐ~ι~ησοιο", "ἐ~ι~ησοισθον", "ἐ~ι~ησοισθε";
    future_opt_mid_3: "ἐ~ι~ησοιτο", "ἐ~ι~ησοισθην", "ἐ~ι~ησοιντο";
    aorist_ind_pass_1: "ἠ~ι~ηθην", _, "ἠ~ι~ηθημεν";
    aorist_ind_pass_2: "ἠ~ι~ηθης", "ἠ~ι~ηθητον", "ἠ~ι~ηθητε";
    aorist_ind_pass_3: "ἠ~ι~ηθη", "ἠ~ι~ηθητην", "ἠ~ι~ηθησᾰν";
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
    // Πτᾰρνῠμαι: 1 words (πτᾰρνῠμαι). Like Ἐφᾰγον, except in 46 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("πτᾰρνῠμαι", 1, "~νῠμαι", like Ἐφᾰγον;
    ind_act_1: "~νῠμαι", _, _;
    ind_mid_1: "~νῠμαι", _, "~νῠμεθᾰ";
    ind_mid_2: "~νῠσαι", "~νῠσθον", "~νῠσθε";
    ind_mid_3: "~νῠται", _, "~νῠνται";
    ind_pass_1: _, _, "~νῠμεθᾰ";
    ind_pass_2: _, _, "~νῠσθε";
    ind_pass_3: "~νῠται", _, "~νῠνται";
    subj_mid_1: "~νῠωμαι", _, "~νῠωμεθᾰ";
    subj_mid_2: "~νῠῃ", "~νῠησθον", "~νῠησθε";
    subj_mid_3: "~νῠηται", _, "~νῠωνται";
    subj_pass_1: _, _, "~νῠωμεθᾰ";
    subj_pass_2: _, _, "~νῠησθε";
    subj_pass_3: "~νῠηται", _, "~νῠωνται";
    opt_mid_1: "~νῠοιμην", _, "~νῠοιμεθᾰ";
    opt_mid_2: "~νῠοιο", "~νῠοισθον", "~νῠοισθε";
    opt_mid_3: "~νῠοιτο", "~νῠοισθην", "~νῠοιντο";
    opt_pass_1: _, _, "~νῠοιμεθᾰ";
    opt_pass_2: _, "~νῠοισθον", "~νῠοισθε";
    opt_pass_3: "~νῠοιτο", "~νῠοισθην", "~νῠοιντο";
    imp_mid_2: "~νῠσο", _, "~νῠσθε";
    imp_mid_3: "~νῠσθω", "~νῠσθων", "~νῠσθων";
    imp_pass_2: _, _, "~νῠσθε";
    imp_pass_3: _, "~νῠσθων", _;
    infinitive_pres: _, "~νῠσθαι", _;
    participle_pres: _, "~νῠμενος", _;
    ),
    // Ἐπορον: 1 words (ἐπορον). Like Μετεκῑᾰθον, except in 50 cells: ind_act_1 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_2 pl, ….
    conjugation!("ἐπορον", 1, "ἐ~ο~ον", like Μετεκῑᾰθον;
    ind_act_1: "ἐ~ο~ον", _, _;
    aorist_ind_act_1: "ἐ~ο~ον|~ο~ον", _, "ἐ~ο~ομεν|~ο~ομεν";
    aorist_ind_act_2: "ἐ~ο~ες|~ο~ες", "ἐ~ο~ετον|~ο~ετον", "ἐ~ο~ετε|~ο~ετε";
    aorist_ind_act_3: "ἐ~ο~ε|ἐ~ο~εν|~ο~ε|~ο~εν", "ἐ~ο~ετην|~ο~ετην", "ἐ~ο~ον|~ο~ον";
    aorist_subj_act_1: "~ο~ω", _, "~ο~ωμεν";
    aorist_subj_act_2: "~ο~ῃς", "~ο~ητον", "~ο~ητε";
    aorist_subj_act_3: "~ο~ῃ", _, "~ο~ωσῐ|~ο~ωσῐν";
    aorist_opt_act_1: "~ο~οιμῐ", _, "~ο~οιμεν";
    aorist_opt_act_2: "~ο~οις", "~ο~οιτον", "~ο~οιτε";
    aorist_opt_act_3: "~ο~οι", "~ο~οιτην", "~ο~οιεν";
    aorist_imp_act_2: "~ο~ε", "~ο~ετον", "~ο~ετε";
    aorist_imp_act_3: "~ο~ετω", "~ο~ετων", "~ο~οντων";
    perfect_ind_pass_1: "%ε~~ωμαι", _, "%ε~~ωμεθᾰ";
    perfect_ind_pass_2: "%ε~~ωσαι", "%ε~~ωσθον", "%ε~~ωσθε";
    perfect_ind_pass_3: "%ε~~ωται", _, "%ε~~ωνται";
    perfect_imp_pass_2: "%ε~~ωσο", _, _;
    perfect_imp_pass_3: "%ε~~ωσθω", "%ε~~ωσθων", _;
    pluperfect_pass_1: "%ε~~ωμην", _, "%ε~~ωμεθᾰ";
    pluperfect_pass_2: "%ε~~ωσο", "%ε~~ωσθον", "%ε~~ωσθε";
    pluperfect_pass_3: "%ε~~ωτο", "%ε~~ωσθην", "%ε~~ωντο";
    infinitive_aor: "~ο~εῑν", _, _;
    participle_aor: "~ο~ων", _, _;
    ),
    // Δαινῡμῐ: 1 words (δαινῡμῐ). Like Κεῑμαι, except in 170 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("δαινῡμῐ", 1, "~ινῡμῐ", like Κεῑμαι;
    ind_act_1: "~ινῡμῐ", _, "~ινῠμεν";
    ind_act_2: "~ινῡς", _, _;
    ind_act_3: "~ινῡσῐ|~ινῡσῐν", _, "~ινῠᾱσῐ|~ινῠᾱσῐν";
    ind_mid_1: "~ινῠμαι", _, "~ινῠμεθᾰ";
    ind_mid_2: "~ινῠσαι", "~ινῠσθον", "~ινῠσθε";
    ind_mid_3: "~ινῠται", _, "~ινῠνται";
    ind_pass_1: _, _, "~ινῠμεθᾰ";
    ind_pass_2: _, _, "~ινῠσθε";
    ind_pass_3: "~ινῠται", _, "~ινῠνται";
    subj_act_1: "~ινῠω", _, "~ινῠωμεν";
    subj_act_2: "~ινῠῃς", "~ινῠητον", "~ινῠητε";
    subj_act_3: "~ινῠῃ", _, "~ινῠωσῐ|~ινῠωσῐν";
    subj_mid_1: "~ινῠωμαι", _, "~ινῠωμεθᾰ";
    subj_mid_2: "~ινῠῃ", "~ινῠησθον", "~ινῠησθε";
    subj_mid_3: "~ινῠηται", _, "~ινῠωνται";
    subj_pass_1: _, _, "~ινῠωμεθᾰ";
    subj_pass_2: _, _, "~ινῠησθε";
    subj_pass_3: "~ινῠηται", _, "~ινῠωνται";
    opt_act_1: "~ινῠοιμῐ", _, "~ινῠοιμεν";
    opt_act_2: "~ινῠοις", "~ινῠοιτον", "~ινῠοιτε";
    opt_act_3: "~ινῠοι", "~ινῠοιτην", "~ινῠοιεν";
    opt_mid_1: "~ινῠοιμην", _, "~ινῠοιμεθᾰ";
    opt_mid_2: "~ινῠοιο", "~ινῠοισθον", "~ινῠοισθε";
    opt_mid_3: "~ινῠοιτο|~ινῡτο", "~ινῠοισθην", "~ινῠοιντο|~ινυατο";
    opt_pass_1: _, _, "~ινῠοιμεθᾰ";
    opt_pass_2: _, "~ινῠοισθον", "~ινῠοισθε";
    opt_pass_3: "~ινῠοιτο|~ινῡτο", "~ινῠοισθην", "~ινῠοιντο|~ινυατο";
    imp_act_2: "~ινῡ", "~ινῠτον", "~ινῠτε";
    imp_act_3: "~ινῠτω", "~ινῠτων", "~ινῠντων";
    imp_mid_2: "~ινῠσο", _, "~ινῠσθε";
    imp_mid_3: "~ινῠσθω", "~ινῠσθων", "~ινῠσθων";
    imp_pass_2: _, _, "~ινῠσθε";
    imp_pass_3: _, "~ινῠσθων", _;
    imperfect_act_1: "ἐ~ινῡν", _, "ἐ~ινῠμεν";
    imperfect_act_2: "ἐ~ινῡς", "ἐ~ινῠτον", "ἐ~ινῠτε";
    imperfect_act_3: "ἐ~ινῡ", "ἐ~ινῠτην", "ἐ~ινῠσᾰν";
    imperfect_mid_1: "ἐ~ινῠμην", _, "ἐ~ινῠμεθᾰ";
    imperfect_mid_2: "ἐ~ινῠσο", "ἐ~ινῠσθον", "ἐ~ινῠσθε";
    imperfect_mid_3: "ἐ~ινῠτο", "ἐ~ινῠσθην", "ἐ~ινῠντο";
    imperfect_pass_1: _, _, "ἐ~ινῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ινῠσθε";
    imperfect_pass_3: "ἐ~ινῠτο", _, _;
    future_ind_act_1: "~ισω", _, "~ισομεν";
    future_ind_act_2: "~ισεις", "~ισετον", "~ισετε";
    future_ind_act_3: "~ισει", _, "~ισουσῐ|~ισουσῐν";
    future_opt_act_1: "~ισοιμῐ", _, "~ισοιμεν";
    future_opt_act_2: "~ισοις", "~ισοιτον", "~ισοιτε";
    future_opt_act_3: "~ισοι", "~ισοιτην", "~ισοιεν";
    aorist_ind_act_1: "ἐ~ισᾰ", _, "ἐ~ισᾰμεν";
    aorist_ind_act_2: "ἐ~ισᾰς", "ἐ~ισᾰτον", "ἐ~ισᾰτε";
    aorist_ind_act_3: "ἐ~ισε|ἐ~ισεν", "ἐ~ισᾰτην", "ἐ~ισᾰν";
    aorist_ind_mid_1: "ἐ~ισᾰμην", _, "ἐ~ισᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ισω", "ἐ~ισᾰσθον", "ἐ~ισᾰσθε";
    aorist_ind_mid_3: "ἐ~ισᾰτο", "ἐ~ισᾰσθην", "ἐ~ισᾰντο";
    aorist_subj_act_1: "~ισω", _, "~ισωμεν";
    aorist_subj_act_2: "~ισῃς", "~ισητον", "~ισητε";
    aorist_subj_act_3: "~ισῃ", _, "~ισωσῐ|~ισωσῐν";
    aorist_subj_mid_1: "~ισωμαι", _, "~ισωμεθᾰ";
    aorist_subj_mid_2: "~ισῃ", "~ισησθον", "~ισησθε";
    aorist_subj_mid_3: "~ισηται", _, "~ισωνται";
    aorist_opt_act_1: "~ισαιμῐ", _, "~ισαιμεν";
    aorist_opt_act_2: "~ισειᾰς|~ισαις", "~ισαιτον", "~ισαιτε";
    aorist_opt_act_3: "~ισειε|~ισειεν|~ισαι", "~ισαιτην", "~ισειᾰν|~ισαιεν";
    aorist_opt_mid_1: "~ισαιμην", _, "~ισαιμεθᾰ";
    aorist_opt_mid_2: "~ισαιο", "~ισαισθον", "~ισαισθε";
    aorist_opt_mid_3: "~ισαιτο", "~ισαισθην", "~ισαιντο";
    aorist_imp_act_2: "~ῑσον", "~ισᾰτον", "~ισᾰτε";
    aorist_imp_act_3: "~ισᾰτω", "~ισᾰτων", "~ισᾰντων";
    aorist_imp_mid_2: "~ῑσαι", "~ισᾰσθον", "~ισᾰσθε";
    aorist_imp_mid_3: "~ισᾰσθω", "~ισᾰσθων", _;
    infinitive_pres: "~ινῠναι", "~ινῠσθαι", _;
    infinitive_fut: "~ισειν", _, _;
    infinitive_aor: "~ῑσαι", "~ισᾰσθαι", _;
    participle_pres: "~ινῡς", "~ινῠμενος", _;
    participle_fut: "~ισων", _, _;
    participle_aor: "~ισᾱς", "~ισᾰμενος", _;
    ),
    // Ἐνῐστημῐ: 1 words (ἐνῐστημῐ). Like Λιστραω, except in 307 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἐνῐστημῐ", 1, "~ῐ~ημῐ", like Λιστραω;
    ind_act_1: "~ῐ~ημῐ|~ε~ηκω", _, "~ῐ~ᾰμεν|~ε~ηκομεν";
    ind_act_2: "~ῐ~ης|~ε~ηκεις", "~ῐ~ᾰτον|~ε~ηκετον", "~ῐ~ᾰτε|~ε~ηκετε";
    ind_act_3: "~ῐ~ησῐ|~ῐ~ησῐν|~ε~ηκει", "~ῐ~ᾰτον|~ε~ηκετον", "~ῐ~ᾱσῐ|~ῐ~ᾱσῐν|~ε~ηκουσῐ|~ε~ηκουσῐν";
    ind_mid_1: "~ῐ~ᾰμαι|~ε~ηκομαι", _, "~ῐ~ᾰμεθᾰ|~ε~ηκομεθᾰ";
    ind_mid_2: "~ῐ~ᾰσαι|~ε~ηκῃ|~ε~ηκει", "~ῐ~ᾰσθον|~ε~ηκεσθον", "~ῐ~ᾰσθε|~ε~ηκεσθε";
    ind_mid_3: "~ῐ~ᾰται|~ε~ηκεται", "~ῐ~ᾰσθον|~ε~ηκεσθον", "~ῐ~ᾰνται|~ε~ηκονται";
    ind_pass_1: "~ῐ~ᾰμαι|~ε~ηκομαι", _, "~ῐ~ᾰμεθᾰ|~ε~ηκομεθᾰ";
    ind_pass_2: "~ῐ~ᾰσαι|~ε~ηκῃ|~ε~ηκει", "~ῐ~ᾰσθον|~ε~ηκεσθον", "~ῐ~ᾰσθε|~ε~ηκεσθε";
    ind_pass_3: "~ῐ~ᾰται|~ε~ηκεται", "~ῐ~ᾰσθον|~ε~ηκεσθον", "~ῐ~ᾰνται|~ε~ηκονται";
    subj_act_1: "~ῐ~ω|~ε~ηκω", _, "~ῐ~ωμεν|~ε~ηκωμεν";
    subj_act_2: "~ῐ~ῃς|~ε~ηκῃς", "~ῐ~ητον|~ε~ηκητον", "~ῐ~ητε|~ε~ηκητε";
    subj_act_3: "~ῐ~ῃ|~ε~ηκῃ", "~ῐ~ητον|~ε~ηκητον", "~ῐ~ωσῐ|~ῐ~ωσῐν|~ε~ηκωσῐ|~ε~ηκωσῐν";
    subj_mid_1: "~ῐ~ωμαι|~ε~ηκωμαι", _, "~ῐ~ωμεθᾰ|~ε~ηκωμεθᾰ";
    subj_mid_2: "~ῐ~ῃ|~ε~ηκῃ", "~ῐ~ησθον|~ε~ηκησθον", "~ῐ~ησθε|~ε~ηκησθε";
    subj_mid_3: "~ῐ~ηται|~ε~ηκηται", "~ῐ~ησθον|~ε~ηκησθον", "~ῐ~ωνται|~ε~ηκωνται";
    subj_pass_1: "~ῐ~ωμαι|~ε~ηκωμαι", _, "~ῐ~ωμεθᾰ|~ε~ηκωμεθᾰ";
    subj_pass_2: "~ῐ~ῃ|~ε~ηκῃ", "~ῐ~ησθον|~ε~ηκησθον", "~ῐ~ησθε|~ε~ηκησθε";
    subj_pass_3: "~ῐ~ηται|~ε~ηκηται", "~ῐ~ησθον|~ε~ηκησθον", "~ῐ~ωνται|~ε~ηκωνται";
    opt_act_1: "~ῐ~αιην|~ε~ηκοιμῐ", _, "~ῐ~αῑμεν|~ῐ~αιημεν|~ε~ηκοιμεν";
    opt_act_2: "~ῐ~αιης|~ε~ηκοις", "~ῐ~αῑτον|~ῐ~αιητον|~ε~ηκοιτον", "~ῐ~αῑτε|~ῐ~αιητε|~ε~ηκοιτε";
    opt_act_3: "~ῐ~αιη|~ε~ηκοι", "~ῐ~αιτην|~ῐ~αιητην|~ε~ηκοιτην", "~ῐ~αῑεν|~ῐ~αιησᾰν|~ε~ηκοιεν";
    opt_mid_1: "~ῐ~αιμην|~ε~ηκοιμην", _, "~ῐ~αιμεθᾰ|~ε~ηκοιμεθᾰ";
    opt_mid_2: "~ῐ~αιο|~ε~ηκοιο", "~ῐ~αισθον|~ε~ηκοισθον", "~ῐ~αισθε|~ε~ηκοισθε";
    opt_mid_3: "~ῐ~αιτο|~ε~ηκοιτο", "~ῐ~αισθην|~ε~ηκοισθην", "~ῐ~αιντο|~ε~ηκοιντο";
    opt_pass_1: "~ῐ~αιμην|~ε~ηκοιμην", _, "~ῐ~αιμεθᾰ|~ε~ηκοιμεθᾰ";
    opt_pass_2: "~ῐ~αιο|~ε~ηκοιο", "~ῐ~αισθον|~ε~ηκοισθον", "~ῐ~αισθε|~ε~ηκοισθε";
    opt_pass_3: "~ῐ~αιτο|~ε~ηκοιτο", "~ῐ~αισθην|~ε~ηκοισθην", "~ῐ~αιντο|~ε~ηκοιντο";
    imp_act_2: "~ῐ~η|~ε~ηκε", "~ῐ~ᾰτον|~ε~ηκετον", "~ῐ~ᾰτε|~ε~ηκετε";
    imp_act_3: "~ῐ~ᾰτω|~ε~ηκετω", "~ῐ~ᾰτων|~ε~ηκετων", "~ῐ~ᾰντων|~ε~ηκοντων";
    imp_mid_2: "~ῐ~ᾰσο|~ῐ~ω|~ε~ηκου", "~ῐ~ᾰσθον|~ε~ηκεσθον", "~ῐ~ᾰσθε|~ε~ηκεσθε";
    imp_mid_3: "~ῐ~ᾰσθω|~ε~ηκεσθω", "~ῐ~ᾰσθων|~ε~ηκεσθων", "~ῐ~ᾰσθων|~ε~ηκεσθων";
    imp_pass_2: "~ῐ~ᾰσο|~ῐ~ω|~ε~ηκου", "~ῐ~ᾰσθον|~ε~ηκεσθον", "~ῐ~ᾰσθε|~ε~ηκεσθε";
    imp_pass_3: "~ῐ~ᾰσθω|~ε~ηκεσθω", "~ῐ~ᾰσθων|~ε~ηκεσθων", "~ῐ~ᾰσθων|~ε~ηκεσθων";
    imperfect_act_1: "~ῑ~ην|~ι~ασκον", _, "~ῑ~ᾰμεν|~ι~ασκομεν";
    imperfect_act_2: "~ῑ~ης|~ι~ασκες", "~ῑ~ᾰτον|~ι~ασκετον", "~ῑ~ᾰτε|~ι~ασκετε";
    imperfect_act_3: "~ῑ~η|~ι~ασκε|~ι~ασκεν", "~ῑ~ᾰτην|~ι~ασκετην", "~ῑ~ᾰσᾰν|~ι~ασκον";
    imperfect_mid_1: "~ῑ~ᾰμην|~ι~ασκομην", _, "~ῑ~ᾰμεθᾰ|~ι~ασκομεθᾰ";
    imperfect_mid_2: "~ῑ~ᾰσο|~ι~ασκου", "~ῑ~ᾰσθον|~ι~ασκεσθον", "~ῑ~ᾰσθε|~ι~ασκεσθε";
    imperfect_mid_3: "~ῑ~ᾰτο|~ι~ασκετο", "~ῑ~ᾰσθην|~ι~ασκεσθην", "~ῑ~ᾰντο|~ι~ασκοντο";
    imperfect_pass_1: "~ῑ~ᾰμην|~ι~ασκομην", _, "~ῑ~ᾰμεθᾰ|~ι~ασκομεθᾰ";
    imperfect_pass_2: "~ῑ~ᾰσο|~ι~ασκου", "~ῑ~ᾰσθον|~ι~ασκεσθον", "~ῑ~ᾰσθε|~ι~ασκεσθε";
    imperfect_pass_3: "~ῑ~ᾰτο|~ι~ασκετο", "~ῑ~ᾰσθην|~ι~ασκεσθην", "~ῑ~ᾰντο|~ι~ασκοντο";
    future_ind_act_1: "~~ᾱσεω", _, "~~ᾱσεομες";
    future_ind_act_2: "~~ᾱσεῑς", "~~ᾱσεῑτον", "~~ᾱσεῑτε";
    future_ind_act_3: "~~ᾱσεῑ", _, "~~ᾱσεοντῐ";
    future_ind_mid_1: "~~ᾱσεομαι", _, "~~ᾱσεομεθᾰ";
    future_ind_mid_2: "~~ᾱσῃ", "~~ᾱσεῑσθον", "~~ᾱσεῑσθε";
    future_ind_mid_3: "~~ᾱσεῑται", _, "~~ᾱσεονται";
    future_opt_act_1: "~~ᾱσεοιμῐ", _, "~~ᾱσεοιμες";
    future_opt_act_2: "~~ᾱσεοις", "~~ᾱσεοιτον", "~~ᾱσεοιτε";
    future_opt_act_3: "~~ᾱσεοι", "~~ᾱσεοιτᾱν", "~~ᾱσεοιεν";
    future_opt_mid_1: "~~ᾱσεοιμᾱν", _, "~~ᾱσεοιμεθᾰ";
    future_opt_mid_2: "~~ᾱσεοιο", "~~ᾱσεοισθον", "~~ᾱσεοισθε";
    future_opt_mid_3: "~~ᾱσεοιτο", "~~ᾱσεοισθᾱν", "~~ᾱσεοιντο";
    aorist_ind_act_1: "~ε~ησᾰ|~ε~ην|~~ᾰσκον", _, "~ε~ησᾰμεν|~ε~ημεν|~~ᾰσκομεν";
    aorist_ind_act_2: "~ε~ησᾰς|~ε~ης|~~ᾰσκες", "~ε~ησᾰτον|~ε~ητον|~~ᾰσκετον", "~ε~ησᾰτε|~ε~ητε|~~ᾰσκετε";
    aorist_ind_act_3: "~ε~ησε|~ε~ησεν|~ε~η|~~ᾰσκε|~~ᾰσκεν", "~ε~ησᾰτην|~ε~ητην|~~ᾰσκετην", "~ε~ησᾰν|~~ᾰσκον";
    aorist_ind_mid_1: "~ε~ησᾰμην|~ε~ᾰμην|~~ᾰσκομην", _, "~ε~ησᾰμεθᾰ|~ε~ᾰμεθᾰ|~~ᾰσκομεθᾰ";
    aorist_ind_mid_2: "~ε~ησω|~ε~ω|~~ᾰσκου", "~ε~ησᾰσθον|~ε~ᾰσθον|~~ᾰσκεσθον", "~ε~ησᾰσθε|~ε~ᾰσθε|~~ᾰσκεσθε";
    aorist_ind_mid_3: "~ε~ησᾰτο|~ε~ᾰτο|~~ᾰσκετο", "~ε~ησᾰσθην|~ε~ᾰσθην|~~ᾰσκεσθην", "~ε~ησᾰντο|~ε~ᾰντο|~~ᾰσκοντο";
    aorist_ind_pass_1: "~ε~ᾰθην|~ε~ην", _, "~ε~ᾰθημεν|~ε~ημεν";
    aorist_ind_pass_2: "~ε~ᾰθης|~ε~ης", "~ε~ᾰθητον|~ε~ητον", "~ε~ᾰθητε|~ε~ητε";
    aorist_ind_pass_3: "~ε~ᾰθη|~ε~η", "~ε~ᾰθητην|~ε~ητην", "~ε~ᾰθησᾰν|~ε~ησᾰν";
    aorist_subj_act_1: "~~ησω|~~ω|~~ᾰσκω", _, "~~ησωμεν|~~ωμεν|~~ᾰσκωμεν";
    aorist_subj_act_2: "~~ησῃς|~~ῃς|~~ᾰσκῃς", "~~ησητον|~~ητον|~~ᾰσκητον", "~~ησητε|~~ητε|~~ᾰσκητε";
    aorist_subj_act_3: "~~ησῃ|~~ῃ|~~ᾰσκῃ", "~~ησητον|~~ητον|~~ᾰσκητον", "~~ησωσῐ|~~ησωσῐν|~~ωσῐ|~~ωσῐν|~~ᾰσκωσῐ|~~ᾰσκωσῐν";
    aorist_subj_mid_1: "~~ησωμαι|~~ωμαι|~~ᾰσκωμαι", _, "~~ησωμεθᾰ|~~ωμεθᾰ|~~ᾰσκωμεθᾰ";
    aorist_subj_mid_2: "~~ησῃ|~~ῃ|~~ᾰσκῃ", "~~ησησθον|~~ησθον|~~ᾰσκησθον", "~~ησησθε|~~ησθε|~~ᾰσκησθε";
    aorist_subj_mid_3: "~~ησηται|~~ηται|~~ᾰσκηται", "~~ησησθον|~~ησθον|~~ᾰσκησθον", "~~ησωνται|~~ωνται|~~ᾰσκωνται";
    aorist_subj_pass_1: "~~ᾰθω|~~ω", _, "~~ᾰθωμεν|~~ωμεν";
    aorist_subj_pass_2: "~~ᾰθῃς|~~ῃς", "~~ᾰθητον|~~ητον", "~~ᾰθητε|~~ητε";
    aorist_subj_pass_3: "~~ᾰθῃ|~~ῃ", "~~ᾰθητον|~~ητον", "~~ᾰθωσῐ|~~ᾰθωσῐν|~~ωσῐ|~~ωσῐν";
    aorist_opt_act_1: "~~ησαιμῐ|~~αιην|~~ᾰσκοιμῐ", _, "~~ησαιμεν|~~αῑμεν|~~αιημεν|~~ᾰσκοιμεν";
    aorist_opt_act_2: "~~ησειᾰς|~~ησαις|~~αιης|~~ᾰσκοις", "~~ησαιτον|~~αῑτον|~~αιητον|~~ᾰσκοιτον", "~~ησαιτε|~~αῑτε|~~αιητε|~~ᾰσκοιτε";
    aorist_opt_act_3: "~~ησειε|~~ησειεν|~~ησαι|~~αιη|~~ᾰσκοι", "~~ησαιτην|~~αιτην|~~αιητην|~~ᾰσκοιτην", "~~ησειᾰν|~~ησαιεν|~~αῑεν|~~αιησᾰν|~~ᾰσκοιεν";
    aorist_opt_mid_1: "~~ησαιμην|~~αιμην|~~ᾰσκοιμην", _, "~~ησαιμεθᾰ|~~αιμεθᾰ|~~ᾰσκοιμεθᾰ";
    aorist_opt_mid_2: "~~ησαιο|~~αῑο|~~ᾰσκοιο", "~~ησαισθον|~~αῑσθον|~~ᾰσκοισθον", "~~ησαισθε|~~αῑσθε|~~ᾰσκοισθε";
    aorist_opt_mid_3: "~~ησαιτο|~~αῑτο|~~ᾰσκοιτο", "~~ησαισθην|~~αισθην|~~ᾰσκοισθην", "~~ησαιντο|~~αῑντο|~~ᾰσκοιντο";
    aorist_opt_pass_1: "~~ᾰθειην|~~ειην", _, "~~ᾰθεῑμεν|~~ᾰθειημεν|~~εῑμεν|~~ειημεν";
    aorist_opt_pass_2: "~~ᾰθειης|~~ειης", "~~ᾰθεῑτον|~~ᾰθειητον|~~εῑτον|~~ειητον", "~~ᾰθεῑτε|~~ᾰθειητε|~~εῑτε|~~ειητε";
    aorist_opt_pass_3: "~~ᾰθειη|~~ειη", "~~ᾰθειτην|~~ᾰθειητην|~~ειτην|~~ειητην", "~~ᾰθεῑεν|~~ᾰθειησᾰν|~~εῑεν|~~ειησᾰν";
    aorist_imp_act_2: "~~ησον|~~ηθῐ|~~ᾰσκε", "~~ησᾰτον|~~ητον|~~ᾰσκετον", "~~ησᾰτε|~~ητε|~~ᾰσκετε";
    aorist_imp_act_3: "~~ησᾰτω|~~ητω|~~ᾰσκετω", "~~ησᾰτων|~~ητων|~~ᾰσκετων", "~~ησᾰντων|~~ᾰντων|~~ᾰσκοντων";
    aorist_imp_mid_2: "~~ησαι|~~ω|~~ᾰσκοῡ", "~~ησᾰσθον|~~ᾰσθον|~~ᾰσκεσθον", "~~ησᾰσθε|~~ᾰσθε|~~ᾰσκεσθε";
    aorist_imp_mid_3: "~~ησᾰσθω|~~ᾰσθω|~~ᾰσκεσθω", "~~ησᾰσθων|~~ᾰσθων|~~ᾰσκεσθων", "~~ησᾰσθων|~~ᾰσθων|~~ᾰσκεσθων";
    aorist_imp_pass_2: "~~ᾰθητῐ|~~ηθῐ", "~~ᾰθητον|~~ητον", "~~ᾰθητε|~~ητε";
    aorist_imp_pass_3: "~~ᾰθητω|~~ητω", "~~ᾰθητων|~~ητων", "~~ᾰθεντων|~~εντων";
    perfect_ind_act_1: "~ε~ᾰκᾰ|~ε~ηκᾰ", _, "~ε~ᾰκᾰμεν|~ε~ηκᾰμεν|~ε~ᾰμεν";
    perfect_ind_act_2: "~ε~ᾰκᾰς|~ε~ηκᾰς", "~ε~ᾰκᾰτον|~ε~ηκᾰτον|~ε~ᾰτον", "~ε~ᾰκᾰτε|~ε~ηκᾰτε|~ε~ᾰτε";
    perfect_ind_act_3: "~ε~ᾰκε|~ε~ᾰκεν|~ε~ηκε|~ε~ηκεν", "~ε~ᾰκᾰτον|~ε~ηκᾰτον|~ε~ᾰτον", "~ε~ᾰκᾱσῐ|~ε~ᾰκᾱσῐν|~ε~ηκᾱσῐ|~ε~ηκᾱσῐν|~ε~ᾱσῐ|~ε~ᾱσῐν";
    perfect_ind_mid_1: "~ε~ᾰμαι", _, "~ε~ᾰμεθᾰ";
    perfect_ind_mid_2: "~ε~ᾰσαι", "~ε~ᾰσθον", "~ε~ᾰσθε";
    perfect_ind_mid_3: "~ε~ᾰται", _, "~ε~ᾰνται";
    perfect_ind_pass_1: _, _, "~ε~ᾰμεθᾰ";
    perfect_ind_pass_2: _, _, "~ε~ᾰσθε";
    perfect_subj_act_1: "~ε~ᾰκω|~ε~ηκω|~ε~ω", _, "~ε~ᾰκωμεν|~ε~ηκωμεν|~ε~ωμεν";
    perfect_subj_act_2: "~ε~ᾰκῃς|~ε~ηκῃς|~ε~ῃς", "~ε~ᾰκητον|~ε~ηκητον|~ε~ητον", "~ε~ᾰκητε|~ε~ηκητε|~ε~ητε";
    perfect_subj_act_3: "~ε~ᾰκῃ|~ε~ηκῃ|~ε~ῃ", "~ε~ᾰκητον|~ε~ηκητον|~ε~ητον", "~ε~ᾰκωσῐ|~ε~ᾰκωσῐν|~ε~ηκωσῐ|~ε~ηκωσῐν|~ε~ωσῐ|~ε~ωσῐν";
    perfect_opt_act_1: "~ε~ᾰκοιμῐ|~ε~ᾰκοιην|~ε~ηκοιμῐ|~ε~ηκοιην|~ε~αιην", _, "~ε~ᾰκοιμεν|~ε~ηκοιμεν|~ε~αῑμεν|~ε~αιημεν";
    perfect_opt_act_2: "~ε~ᾰκοις|~ε~ᾰκοιης|~ε~ηκοις|~ε~ηκοιης|~ε~αιης", "~ε~ᾰκοιτον|~ε~ηκοιτον|~ε~αῑτον|~ε~αιητον", "~ε~ᾰκοιτε|~ε~ηκοιτε|~ε~αῑτε|~ε~αιητε";
    perfect_opt_act_3: "~ε~ᾰκοι|~ε~ᾰκοιη|~ε~ηκοι|~ε~ηκοιη|~ε~αιη", "~ε~ᾰκοιτην|~ε~ηκοιτην|~ε~αιτην|~ε~αιητην", "~ε~ᾰκοιεν|~ε~ηκοιεν|~ε~αῑεν|~ε~αιησᾰν";
    perfect_imp_act_2: "~ε~ᾰκε|~ε~ηκε|~ε~ᾰθῐ", "~ε~ᾰκετον|~ε~ηκετον|~ε~ᾰτον", "~ε~ᾰκετε|~ε~ηκετε|~ε~ᾰτε";
    perfect_imp_act_3: "~ε~ᾰκετω|~ε~ηκετω|~ε~ᾰτω", "~ε~ᾰκετων|~ε~ηκετων|~ε~ᾰτων", "~ε~ᾰκοντων|~ε~ηκοντων|~ε~ᾰντων";
    perfect_imp_mid_2: "~ε~ᾰσο", _, _;
    perfect_imp_mid_3: "~ε~ᾰσθω", "~ε~ᾰσθων", _;
    perfect_imp_pass_2: "~ε~ᾰσο", _, _;
    perfect_imp_pass_3: "~ε~ᾰσθω", "~ε~ᾰσθων", _;
    pluperfect_act_1: "~ε~ηκειν|~ε~ηκη|~ει~ηκειν|~ει~ηκη", _, "~ε~ηκεμεν|~ει~ηκεμεν|~ε~ᾰμεν";
    pluperfect_act_2: "~ε~ηκεις|~ε~ηκης|~ει~ηκεις|~ει~ηκης", "~ε~ηκετον|~ει~ηκετον|~ε~ᾰτον", "~ε~ηκετε|~ει~ηκετε|~ε~ᾰτε";
    pluperfect_act_3: "~ε~ηκει|~ε~ηκειν|~ει~ηκει|~ει~ηκειν", "~ε~ηκετην|~ει~ηκετην|~ε~ᾰτην", "~ε~ηκεσᾰν|~ει~ηκεσᾰν|~ε~ᾰσᾰν";
    infinitive_pres: "~ῐ~ᾰναι|~ε~ηκειν", "~ῐ~ᾰσθαι|~ε~ηκεσθαι", "~ῐ~ᾰσθαι|~ε~ηκεσθαι";
    infinitive_fut: "~~ᾱσεῑν", "~~ᾱσεῑσθαι", _;
    infinitive_aor: "~~ησαι|~~ηναι|~~ᾰσκεῑν", "~~ησᾰσθαι|~~ᾰσθαι|~~ᾰσκεσθαι", "~~ᾰθηναι|~~ηναι";
    infinitive_perf: "~ε~ᾰκεναι|~ε~ηκεναι|~ε~ᾰναι", "~ε~ᾰσθαι", _;
    participle_pres: "~ῐ~ᾱς|~ε~ηκων", "~ῐ~ᾰμενος|~ε~ηκομενος", "~ῐ~ᾰμενος|~ε~ηκομενος";
    participle_fut: "~~ᾱσεων", "~~ᾱσεομενος", _;
    participle_aor: "~~ησᾱς|~~ᾱς|~~ᾰσκων", "~~ησᾰμενος|~~ᾰμενος|~~ᾰσκομενος", "~~ᾰθεις|~~εις";
    participle_perf: "~ε~ᾰκως|~ε~ηκως|~ε~ως", "~ε~ᾰμενος", _;
    ),
    // Πᾰρᾰτῐθημῐ: 1 words (πᾰρᾰτῐθημῐ). Like Μετεκῑᾰθον, except in 235 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("πᾰρᾰτῐθημῐ", 1, "~ᾰτῐ~ημῐ", like Μετεκῑᾰθον;
    ind_act_1: "~ᾰ%ῐ~ημῐ", _, "~ᾰ%ῐ~εμεν";
    ind_act_2: "~ᾰ%ῐ~ης", _, _;
    ind_act_3: "~ᾰ%ῐ~ησῐ|~ᾰ%ῐ~ησῐν", _, "~ᾰ%ῐ~εᾱσῐ|~ᾰ%ῐ~εᾱσῐν";
    ind_mid_1: "~ᾰ%ῐ~εμαι", _, "~ᾰ%ῐ~εμεθᾰ";
    ind_mid_2: "~ᾰ%ῐ~εσαι", "~ᾰ%ῐ~εσθον", "~ᾰ%ῐ~εσθε";
    ind_mid_3: "~ᾰ%ῐ~εται", _, "~ᾰ%ῐ~ενται";
    ind_pass_1: _, _, "~ᾰ%ῐ~εμεθᾰ";
    ind_pass_2: _, _, "~ᾰ%ῐ~εσθε";
    ind_pass_3: "~ᾰ%ῐ~εται", _, "~ᾰ%ῐ~ενται";
    subj_act_1: "~ᾰ%ῐ~ω", _, "~ᾰ%ῐ~ωμεν";
    subj_act_2: "~ᾰ%ῐ~ῃς", "~ᾰ%ῐ~ητον", "~ᾰ%ῐ~ητε";
    subj_act_3: "~ᾰ%ῐ~ῃ", _, "~ᾰ%ῐ~ωσῐ|~ᾰ%ῐ~ωσῐν";
    subj_mid_1: "~ᾰ%ῐ~ωμαι", _, "~ᾰ%ῐ~ωμεθᾰ";
    subj_mid_2: "~ᾰ%ῐ~ῃ", "~ᾰ%ῐ~ησθον", "~ᾰ%ῐ~ησθε";
    subj_mid_3: "~ᾰ%ῐ~ηται", _, "~ᾰ%ῐ~ωνται";
    subj_pass_1: _, _, "~ᾰ%ῐ~ωμεθᾰ";
    subj_pass_2: _, _, "~ᾰ%ῐ~ησθε";
    subj_pass_3: "~ᾰ%ῐ~ηται", _, "~ᾰ%ῐ~ωνται";
    opt_act_1: "~ᾰ%ῐ~ειην", _, "~ᾰ%ῐ~εῑμεν|~ᾰ%ῐ~ειημεν";
    opt_act_2: "~ᾰ%ῐ~ειης", "~ᾰ%ῐ~εῑτον|~ᾰ%ῐ~ειητον", "~ᾰ%ῐ~εῑτε|~ᾰ%ῐ~ειητε";
    opt_act_3: "~ᾰ%ῐ~ειη", "~ᾰ%ῐ~ειτην|~ᾰ%ῐ~ειητην", "~ᾰ%ῐ~εῑεν|~ᾰ%ῐ~ειησᾰν";
    opt_mid_1: "~ᾰ%ῐ~ειμην", _, "~ᾰ%ῐ~ειμεθᾰ|~ᾰ%ῐ~οιμεθᾰ";
    opt_mid_2: "~ᾰ%ῐ~εῑο", "~ᾰ%ῐ~εῑσθον|~ᾰ%ῐ~οῑσθον", "~ᾰ%ῐ~εῑσθε|~ᾰ%ῐ~οῑσθε";
    opt_mid_3: "~ᾰ%ῐ~εῑτο|~ᾰ%ῐ~οῑτο", "~ᾰ%ῐ~εισθην|~ᾰ%ῐ~οισθην", "~ᾰ%ῐ~εῑντο|~ᾰ%ῐ~οῑντο";
    opt_pass_1: _, _, "~ᾰ%ῐ~ειμεθᾰ|~ᾰ%ῐ~οιμεθᾰ";
    opt_pass_2: _, "~ᾰ%ῐ~εῑσθον|~ᾰ%ῐ~οῑσθον", "~ᾰ%ῐ~εῑσθε|~ᾰ%ῐ~οῑσθε";
    opt_pass_3: "~ᾰ%ῐ~εῑτο|~ᾰ%ῐ~οῑτο", "~ᾰ%ῐ~εισθην|~ᾰ%ῐ~οισθην", "~ᾰ%ῐ~εῑντο|~ᾰ%ῐ~οῑντο";
    imp_act_2: "~ᾰ%ῐ~ει", "~ᾰ%ῐ~ετον", "~ᾰ%ῐ~ετε";
    imp_act_3: "~ᾰ%ῐ~ετω", "~ᾰ%ῐ~ετων", "~ᾰ%ῐ~εντων";
    imp_mid_2: "~ᾰ%ῐ~εσο|~ᾰ%ῐ~ου", _, "~ᾰ%ῐ~εσθε";
    imp_mid_3: "~ᾰ%ῐ~εσθω", "~ᾰ%ῐ~εσθων", "~ᾰ%ῐ~εσθων";
    imp_pass_2: "~ᾰ%ῐ~εσο|~ᾰ%ῐ~ου", _, "~ᾰ%ῐ~εσθε";
    imp_pass_3: _, "~ᾰ%ῐ~εσθων", _;
    future_ind_act_1: "~ᾰ~ησω", _, "~ᾰ~ησομεν";
    future_ind_act_2: "~ᾰ~ησεις", "~ᾰ~ησετον", "~ᾰ~ησετε";
    future_ind_act_3: "~ᾰ~ησει", _, "~ᾰ~ησουσῐ|~ᾰ~ησουσῐν";
    future_ind_mid_1: "~ᾰ~ησομαι", _, "~ᾰ~ησομεθᾰ";
    future_ind_mid_2: "~ᾰ~ησῃ|~ᾰ~ησει", "~ᾰ~ησεσθον", "~ᾰ~ησεσθε";
    future_ind_mid_3: "~ᾰ~ησεται", _, "~ᾰ~ησονται";
    future_opt_act_1: "~ᾰ~ησοιμῐ", _, "~ᾰ~ησοιμεν";
    future_opt_act_2: "~ᾰ~ησοις", "~ᾰ~ησοιτον", "~ᾰ~ησοιτε";
    future_opt_act_3: "~ᾰ~ησοι", "~ᾰ~ησοιτην", "~ᾰ~ησοιεν";
    future_opt_mid_1: "~ᾰ~ησοιμην", _, "~ᾰ~ησοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~ησοιο", "~ᾰ~ησοισθον", "~ᾰ~ησοισθε";
    future_opt_mid_3: "~ᾰ~ησοιτο", "~ᾰ~ησοισθην", "~ᾰ~ησοιντο";
    aorist_ind_act_1: "~ε~ηκᾰ", _, "~ε~ηκᾰμεν";
    aorist_ind_act_2: "~ε~ηκᾰς", "~ε~ηκᾰτον", "~ε~ηκᾰτε";
    aorist_ind_act_3: "~ε~ηκε|~ε~ηκεν", "~ε~ηκᾰτην", "~ε~ηκᾰν";
    aorist_ind_mid_1: "~ε~ηκᾰμην", _, "~ε~ηκᾰμεθᾰ";
    aorist_ind_mid_2: "~ε~ηκω", "~ε~ηκᾰσθον", "~ε~ηκᾰσθε";
    aorist_ind_mid_3: "~ε~ηκᾰτο", "~ε~ηκᾰσθην", "~ε~ηκᾰντο";
    aorist_subj_act_1: "~ᾰ~ηκω", _, "~ᾰ~ηκωμεν";
    aorist_subj_act_2: "~ᾰ~ηκῃς", "~ᾰ~ηκητον", "~ᾰ~ηκητε";
    aorist_subj_act_3: "~ᾰ~ηκῃ", _, "~ᾰ~ηκωσῐ|~ᾰ~ηκωσῐν";
    aorist_subj_mid_1: "~ᾰ~ηκωμαι", _, "~ᾰ~ηκωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~ηκῃ", "~ᾰ~ηκησθον", "~ᾰ~ηκησθε";
    aorist_subj_mid_3: "~ᾰ~ηκηται", _, "~ᾰ~ηκωνται";
    aorist_opt_act_1: "~ᾰ~ηκαιμῐ", _, "~ᾰ~ηκαιμεν";
    aorist_opt_act_2: "~ᾰ~ηκειᾰς|~ᾰ~ηκαις", "~ᾰ~ηκαιτον", "~ᾰ~ηκαιτε";
    aorist_opt_act_3: "~ᾰ~ηκειε|~ᾰ~ηκειεν|~ᾰ~ηκαι", "~ᾰ~ηκαιτην", "~ᾰ~ηκειᾰν|~ᾰ~ηκαιεν";
    aorist_opt_mid_1: "~ᾰ~ηκαιμην", _, "~ᾰ~ηκαιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~ηκαιο", "~ᾰ~ηκαισθον", "~ᾰ~ηκαισθε";
    aorist_opt_mid_3: "~ᾰ~ηκαιτο", "~ᾰ~ηκαισθην", "~ᾰ~ηκαιντο";
    aorist_imp_act_2: "~ᾰ~ηκον", "~ᾰ~ηκᾰτον", "~ᾰ~ηκᾰτε";
    aorist_imp_act_3: "~ᾰ~ηκᾰτω", "~ᾰ~ηκᾰτων", "~ᾰ~ηκᾰντων";
    aorist_imp_mid_2: "~ᾰ~ηκαι", "~ᾰ~ηκᾰσθον", "~ᾰ~ηκᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ηκᾰσθω", "~ᾰ~ηκᾰσθων", _;
    perfect_ind_act_1: "~ᾰ%ε~εικᾰ", _, "~ᾰ%ε~εικᾰμεν";
    perfect_ind_act_2: "~ᾰ%ε~εικᾰς", "~ᾰ%ε~εικᾰτον", "~ᾰ%ε~εικᾰτε";
    perfect_ind_act_3: "~ᾰ%ε~εικε|~ᾰ%ε~εικεν", _, "~ᾰ%ε~εικᾱσῐ|~ᾰ%ε~εικᾱσῐν";
    perfect_ind_mid_1: "~ᾰ%ε~ειμαι", _, "~ᾰ%ε~ειμεθᾰ";
    perfect_ind_mid_2: "~ᾰ%ε~εισαι", "~ᾰ%ε~εισθον", "~ᾰ%ε~εισθε";
    perfect_ind_mid_3: "~ᾰ%ε~ειται", _, "~ᾰ%ε~εινται";
    perfect_ind_pass_1: _, _, "~ᾰ%ε~ειμεθᾰ";
    perfect_ind_pass_2: _, _, "~ᾰ%ε~εισθε";
    perfect_subj_act_1: "~ᾰ%ε~εικω", _, "~ᾰ%ε~εικωμεν";
    perfect_subj_act_2: "~ᾰ%ε~εικῃς", "~ᾰ%ε~εικητον", "~ᾰ%ε~εικητε";
    perfect_subj_act_3: "~ᾰ%ε~εικῃ", _, "~ᾰ%ε~εικωσῐ|~ᾰ%ε~εικωσῐν";
    perfect_opt_act_1: "~ᾰ%ε~εικοιμῐ|~ᾰ%ε~εικοιην", _, "~ᾰ%ε~εικοιμεν";
    perfect_opt_act_2: "~ᾰ%ε~εικοις|~ᾰ%ε~εικοιης", "~ᾰ%ε~εικοιτον", "~ᾰ%ε~εικοιτε";
    perfect_opt_act_3: "~ᾰ%ε~εικοι|~ᾰ%ε~εικοιη", "~ᾰ%ε~εικοιτην", "~ᾰ%ε~εικοιεν";
    perfect_imp_act_2: "~ᾰ%ε~εικε", "~ᾰ%ε~εικετον", "~ᾰ%ε~εικετε";
    perfect_imp_act_3: "~ᾰ%ε~εικετω", "~ᾰ%ε~εικετων", "~ᾰ%ε~εικοντων";
    perfect_imp_mid_2: "~ᾰ%ε~εισο", _, _;
    perfect_imp_mid_3: "~ᾰ%ε~εισθω", "~ᾰ%ε~εισθων", _;
    perfect_imp_pass_2: "~ᾰ%ε~εισο", _, _;
    perfect_imp_pass_3: "~ᾰ%ε~εισθω", "~ᾰ%ε~εισθων", _;
    pluperfect_act_1: "~ε%ε~εικειν|~ε%ε~εικη", _, "~ε%ε~εικεμεν";
    pluperfect_act_2: "~ε%ε~εικεις|~ε%ε~εικης", "~ε%ε~εικετον", "~ε%ε~εικετε";
    pluperfect_act_3: "~ε%ε~εικει|~ε%ε~εικειν", "~ε%ε~εικετην", "~ε%ε~εικεσᾰν";
    pluperfect_mid_1: "~ᾰ%ε~ειμην", _, "~ᾰ%ε~ειμεθᾰ";
    pluperfect_mid_2: "~ᾰ%ε~εισο", "~ᾰ%ε~εισθον", "~ᾰ%ε~εισθε";
    pluperfect_mid_3: "~ᾰ%ε~ειτο", "~ᾰ%ε~εισθην", _;
    pluperfect_pass_1: _, _, "~ᾰ%ε~ειμεθᾰ";
    pluperfect_pass_2: _, _, "~ᾰ%ε~εισθε";
    pluperfect_pass_3: _, "~ᾰ%ε~εισθην", "~ᾰ%ε~ειντο";
    infinitive_pres: "~ᾰ%ῐ~εναι", "~ᾰ%ῐ~εσθαι", _;
    infinitive_fut: "~ᾰ~ησειν", "~ᾰ~ησεσθαι", _;
    infinitive_aor: "~ᾰ~ηκαι", "~ᾰ~ηκᾰσθαι", _;
    infinitive_perf: "~ᾰ%ε~εικεναι", "~ᾰ%ε~εῑσθαι", _;
    participle_pres: "~ᾰ%ῐ~εις", "~ᾰ%ῐ~εμενος", _;
    participle_fut: "~ᾰ~ησων", "~ᾰ~ησομενος", _;
    participle_aor: "~ᾰ~ηκᾱς", "~ᾰ~ηκᾰμενος", _;
    participle_perf: "~ᾰ%ε~εικως", "~ᾰ%ε~ειμενος", _;
    ),
    // Κατατῐθημῐ: 1 words (κατατῐθημῐ). Like Διατιθημῐ, except in 157 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("κατατῐθημῐ", 1, "~ατῐ~ημῐ", like Διατιθημῐ;
    ind_act_1: "~α%ῐ~ημῐ", _, "~α%ῐ~εμεν";
    ind_act_2: "~α%ῐ~ης", _, _;
    ind_act_3: "~α%ῐ~ησῐ|~α%ῐ~ησῐν", _, "~α%ῐ~εᾱσῐ|~α%ῐ~εᾱσῐν";
    ind_mid_1: "~α%ῐ~εμαι", _, "~α%ῐ~εμεθᾰ";
    ind_mid_2: "~α%ῐ~εσαι", "~α%ῐ~εσθον", "~α%ῐ~εσθε";
    ind_mid_3: "~α%ῐ~εται", _, "~α%ῐ~ενται";
    ind_pass_1: _, _, "~α%ῐ~εμεθᾰ";
    ind_pass_2: _, _, "~α%ῐ~εσθε";
    ind_pass_3: "~α%ῐ~εται", _, "~α%ῐ~ενται";
    subj_act_1: "~α%ῐ~ω", _, "~α%ῐ~ωμεν";
    subj_act_2: "~α%ῐ~ῃς", "~α%ῐ~ητον", "~α%ῐ~ητε";
    subj_act_3: "~α%ῐ~ῃ", _, "~α%ῐ~ωσῐ|~α%ῐ~ωσῐν";
    subj_mid_1: "~α%ῐ~ωμαι", _, "~α%ῐ~ωμεθᾰ";
    subj_mid_2: "~α%ῐ~ῃ", "~α%ῐ~ησθον", "~α%ῐ~ησθε";
    subj_mid_3: "~α%ῐ~ηται", _, "~α%ῐ~ωνται";
    subj_pass_1: _, _, "~α%ῐ~ωμεθᾰ";
    subj_pass_2: _, _, "~α%ῐ~ησθε";
    subj_pass_3: "~α%ῐ~ηται", _, "~α%ῐ~ωνται";
    opt_act_1: "~α%ῐ~ειην", _, "~α%ῐ~εῑμεν|~α%ῐ~ειημεν";
    opt_act_2: "~α%ῐ~ειης", "~α%ῐ~εῑτον|~α%ῐ~ειητον", "~α%ῐ~εῑτε|~α%ῐ~ειητε";
    opt_act_3: "~α%ῐ~ειη", "~α%ῐ~ειτην|~α%ῐ~ειητην", "~α%ῐ~εῑεν|~α%ῐ~ειησᾰν";
    opt_mid_1: "~α%ῐ~ειμην", _, "~α%ῐ~ειμεθᾰ|~α%ῐ~οιμεθᾰ";
    opt_mid_2: "~α%ῐ~εῑο", "~α%ῐ~εῑσθον|~α%ῐ~οῑσθον", "~α%ῐ~εῑσθε|~α%ῐ~οῑσθε";
    opt_mid_3: "~α%ῐ~εῑτο|~α%ῐ~οῑτο", "~α%ῐ~εισθην|~α%ῐ~οισθην", "~α%ῐ~εῑντο|~α%ῐ~οῑντο";
    opt_pass_1: _, _, "~α%ῐ~ειμεθᾰ|~α%ῐ~οιμεθᾰ";
    opt_pass_2: _, "~α%ῐ~εῑσθον|~α%ῐ~οῑσθον", "~α%ῐ~εῑσθε|~α%ῐ~οῑσθε";
    opt_pass_3: "~α%ῐ~εῑτο|~α%ῐ~οῑτο", "~α%ῐ~εισθην|~α%ῐ~οισθην", "~α%ῐ~εῑντο|~α%ῐ~οῑντο";
    imp_act_2: "~α%ῐ~ει", "~α%ῐ~ετον", "~α%ῐ~ετε";
    imp_act_3: "~α%ῐ~ετω", "~α%ῐ~ετων", "~α%ῐ~εντων";
    imp_mid_2: "~α%ῐ~εσο|~α%ῐ~ου", _, "~α%ῐ~εσθε";
    imp_mid_3: "~α%ῐ~εσθω", "~α%ῐ~εσθων", "~α%ῐ~εσθων";
    imp_pass_2: "~α%ῐ~εσο|~α%ῐ~ου", _, "~α%ῐ~εσθε";
    imp_pass_3: _, "~α%ῐ~εσθων", _;
    imperfect_act_1: "~ε%ῐ~ην", _, "~ε%ῐ~εμεν";
    imperfect_act_2: "~ε%ῐ~εις", "~ε%ῐ~ετον", "~ε%ῐ~ετε";
    imperfect_act_3: "~ε%ῐ~ει", "~ε%ῐ~ετην", "~ε%ῐ~εσᾰν";
    imperfect_mid_1: "~ε%ῐ~εμην", _, "~ε%ῐ~εμεθᾰ";
    imperfect_mid_2: "~ε%ῐ~εσο", "~ε%ῐ~εσθον", "~ε%ῐ~εσθε";
    imperfect_mid_3: "~ε%ῐ~ετο", "~ε%ῐ~εσθην", "~ε%ῐ~εντο";
    imperfect_pass_1: _, _, "~ε%ῐ~εμεθᾰ";
    imperfect_pass_2: _, _, "~ε%ῐ~εσθε";
    imperfect_pass_3: "~ε%ῐ~ετο", _, _;
    aorist_ind_act_1: "~ε~ηκᾰ", _, "~ε~ηκᾰμεν|~ε~εμεν";
    aorist_ind_act_2: "~ε~ηκᾰς", "~ε~ηκᾰτον|~ε~ετον", "~ε~ηκᾰτε|~ε~ετε";
    aorist_ind_act_3: "~ε~ηκε|~ε~ηκεν", "~ε~ηκᾰτην|~ε~ετην", "~ε~ηκᾰν|~ε~εσᾰν";
    aorist_ind_mid_1: "~ε~ηκᾰμην|~ε~εμην", _, "~ε~ηκᾰμεθᾰ|~ε~εμεθᾰ";
    aorist_ind_mid_2: "~ε~ηκω|~ε~ου", "~ε~ηκᾰσθον|~ε~εσθον", "~ε~ηκᾰσθε|~ε~εσθε";
    aorist_ind_mid_3: "~ε~ηκᾰτο|~ε~ετο", "~ε~ηκᾰσθην|~ε~εσθην", "~ε~ηκᾰντο|~ε~εντο";
    aorist_subj_act_1: "~α~ηκω|~α~ω", _, "~α~ηκωμεν|~α~ωμεν";
    aorist_subj_act_2: "~α~ηκῃς|~α~ῃς", "~α~ηκητον|~α~ητον", "~α~ηκητε|~α~ητε";
    aorist_subj_act_3: "~α~ηκῃ|~α~ῃ", "~α~ηκητον|~α~ητον", "~α~ηκωσῐ|~α~ηκωσῐν|~α~ωσῐ|~α~ωσῐν";
    aorist_subj_mid_1: "~α~ηκωμαι|~α~ωμαι", _, "~α~ηκωμεθᾰ|~α~ωμεθᾰ";
    aorist_subj_mid_2: "~α~ηκῃ|~α~ῃ", "~α~ηκησθον|~α~ησθον", "~α~ηκησθε|~α~ησθε";
    aorist_subj_mid_3: "~α~ηκηται|~α~ηται", "~α~ηκησθον|~α~ησθον", "~α~ηκωνται|~α~ωνται";
    aorist_opt_act_1: "~α~ηκαιμῐ|~α~ειην", _, "~α~ηκαιμεν|~α~εῑμεν|~α~ειημεν";
    aorist_opt_act_2: "~α~ηκειᾰς|~α~ηκαις|~α~ειης", "~α~ηκαιτον|~α~εῑτον|~α~ειητον", "~α~ηκαιτε|~α~εῑτε|~α~ειητε";
    aorist_opt_act_3: "~α~ηκειε|~α~ηκειεν|~α~ηκαι|~α~ειη", "~α~ηκαιτην|~α~ειτην|~α~ειητην", "~α~ηκειᾰν|~α~ηκαιεν|~α~εῑεν|~α~ειησᾰν";
    aorist_opt_mid_1: "~α~ηκαιμην|~α~ειμην", _, "~α~ηκαιμεθᾰ|~α~ειμεθᾰ";
    aorist_opt_mid_2: "~α~ηκαιο|~α~εῑο", "~α~ηκαισθον|~α~εῑσθον", "~α~ηκαισθε|~α~εῑσθε";
    aorist_opt_mid_3: "~α~ηκαιτο|~α~εῑτο|~α~οῑτο", "~α~ηκαισθην|~α~εισθην", "~α~ηκαιντο|~α~εῑντο";
    aorist_imp_act_2: "~α~ηκον|~α~ες", "~α~ηκᾰτον|~α~ετον", "~α~ηκᾰτε|~α~ετε";
    aorist_imp_act_3: "~α~ηκᾰτω|~α~ετω", "~α~ηκᾰτων|~α~ετων", "~α~ηκᾰντων|~α~εντων";
    aorist_imp_mid_2: "~α~ηκαι|~α~ου", "~α~ηκᾰσθον|~α~εσθον", "~α~ηκᾰσθε|~α~εσθε";
    aorist_imp_mid_3: "~α~ηκᾰσθω|~α~εσθω", "~α~ηκᾰσθων|~α~εσθων", "~α~ηκᾰσθων|~α~εσθων";
    infinitive_pres: "~α%ῐ~εναι", "~α%ῐ~εσθαι", _;
    infinitive_aor: "~α~ηκαι|~α~εῑναι", "~α~ηκᾰσθαι|~α~εσθαι", _;
    participle_pres: "~α%ῐ~εις", "~α%ῐ~εμενος", _;
    participle_aor: "~α~ηκᾱς|~α~εις", "~α~ηκᾰμενος|~α~εμενος", _;
    ),
    // Κεκᾰδον: 1 words (κεκᾰδον). Like Μετεκῑᾰθον, except in 69 cells: ind_act_1 sg, future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, ….
    conjugation!("κεκᾰδον", 1, "~ᾰ~ον", like Μετεκῑᾰθον;
    ind_act_1: "~ᾰ~ον", _, _;
    future_ind_act_1: "~α~ησω", _, "~α~ησομεν";
    future_ind_act_2: "~α~ησεις", "~α~ησετον", "~α~ησετε";
    future_ind_act_3: "~α~ησει", _, "~α~ησουσῐ|~α~ησουσῐν";
    aorist_ind_act_1: "~ᾰ~ον", _, "~ᾰ~ομεν";
    aorist_ind_act_2: "~ᾰ~ες", "~ᾰ~ετον", "~ᾰ~ετε";
    aorist_ind_act_3: "~ᾰ~ε|~ᾰ~εν", "~ᾰ~ετην", "~ᾰ~ον";
    aorist_ind_mid_1: "~ᾰ~ομην", _, "~ᾰ~ομεθᾰ";
    aorist_ind_mid_2: "~ᾰ~ου", "~ᾰ~εσθον", "~ᾰ~εσθε";
    aorist_ind_mid_3: "~ᾰ~ετο", "~ᾰ~εσθην", "~ᾰ~οντο";
    aorist_subj_act_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    aorist_subj_act_2: "~ᾰ~ῃς", "~ᾰ~ητον", "~ᾰ~ητε";
    aorist_subj_act_3: "~ᾰ~ῃ", _, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~ῃ", "~ᾰ~ησθον", "~ᾰ~ησθε";
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
    // Ἀγᾰμαι: 1 words (ἀγᾰμαι). Like Κεῑμαι, except in 129 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ἀγᾰμαι", 1, "ἀ~ᾰμαι", like Κεῑμαι;
    ind_act_1: "ἀ~ᾰμαι", _, _;
    ind_mid_1: "ἀ~ᾰμαι", _, "ἀ~ᾰμεθᾰ";
    ind_mid_2: "ἀ~ᾰσαι", "ἀ~ᾰσθον", "ἀ~ᾰσθε";
    ind_mid_3: "ἀ~ᾰται", _, "ἀ~ᾰνται";
    subj_mid_1: "ἀ~ωμαι", _, "ἀ~ωμεθᾰ";
    subj_mid_2: "ἀ~ῃ", "ἀ~ησθον", "ἀ~ησθε";
    subj_mid_3: "ἀ~ηται", _, "ἀ~ωνται";
    opt_mid_1: "ἀ~αιμην", _, "ἀ~αιμεθᾰ";
    opt_mid_2: "ἀ~αιο", "ἀ~αισθον", "ἀ~αισθε";
    opt_mid_3: "ἀ~αιτο", "ἀ~αισθην", "ἀ~αιντο";
    imp_mid_2: "ἀ~ᾰσο|ἀ~ω", _, "ἀ~ᾰσθε";
    imp_mid_3: "ἀ~ᾰσθω", "ἀ~ᾰσθων", "ἀ~ᾰσθων";
    imperfect_mid_1: "ἠ~ᾰμην", _, "ἠ~ᾰμεθᾰ";
    imperfect_mid_2: "ἠ~ᾰσο", "ἠ~ᾰσθον", "ἠ~ᾰσθε";
    imperfect_mid_3: "ἠ~ᾰτο", "ἠ~ᾰσθην", "ἠ~ᾰντο";
    future_ind_mid_1: "ἀ~ᾰσσομαι", _, "ἀ~ᾰσσομεθᾰ";
    future_ind_mid_2: "ἀ~ᾰσσῃ|ἀ~ᾰσσει", "ἀ~ᾰσσεσθον", "ἀ~ᾰσσεσθε";
    future_ind_mid_3: "ἀ~ᾰσσεται", _, "ἀ~ᾰσσονται";
    future_ind_pass_1: "ἀ~ᾰσθησομαι", _, "ἀ~ᾰσθησομεθᾰ";
    future_ind_pass_2: "ἀ~ᾰσθησῃ|ἀ~ᾰσθησει", "ἀ~ᾰσθησεσθον", "ἀ~ᾰσθησεσθε";
    future_ind_pass_3: "ἀ~ᾰσθησεται", _, "ἀ~ᾰσθησονται";
    future_opt_mid_1: "ἀ~ᾰσσοιμην", _, "ἀ~ᾰσσοιμεθᾰ";
    future_opt_mid_2: "ἀ~ᾰσσοιο", "ἀ~ᾰσσοισθον", "ἀ~ᾰσσοισθε";
    future_opt_mid_3: "ἀ~ᾰσσοιτο", "ἀ~ᾰσσοισθην", "ἀ~ᾰσσοιντο";
    future_opt_pass_1: "ἀ~ᾰσθησοιμην", _, "ἀ~ᾰσθησοιμεθᾰ";
    future_opt_pass_2: "ἀ~ᾰσθησοιο", "ἀ~ᾰσθησοισθον", "ἀ~ᾰσθησοισθε";
    future_opt_pass_3: "ἀ~ᾰσθησοιτο", "ἀ~ᾰσθησοισθην", "ἀ~ᾰσθησοιντο";
    aorist_ind_mid_1: "ἠ~ᾰσᾰμην", _, "ἠ~ᾰσᾰμεθᾰ";
    aorist_ind_mid_2: "ἠ~ᾰσω", "ἠ~ᾰσᾰσθον", "ἠ~ᾰσᾰσθε";
    aorist_ind_mid_3: "ἠ~ᾰσᾰτο", "ἠ~ᾰσᾰσθην", "ἠ~ᾰσᾰντο";
    aorist_ind_pass_1: "ἠ~ᾰσθην", _, "ἠ~ᾰσθημεν";
    aorist_ind_pass_2: "ἠ~ᾰσθης", "ἠ~ᾰσθητον", "ἠ~ᾰσθητε";
    aorist_ind_pass_3: "ἠ~ᾰσθη", "ἠ~ᾰσθητην", "ἠ~ᾰσθησᾰν";
    aorist_subj_mid_1: "ἀ~ᾰσωμαι", _, "ἀ~ᾰσωμεθᾰ";
    aorist_subj_mid_2: "ἀ~ᾰσῃ", "ἀ~ᾰσησθον", "ἀ~ᾰσησθε";
    aorist_subj_mid_3: "ἀ~ᾰσηται", _, "ἀ~ᾰσωνται";
    aorist_subj_pass_1: "ἀ~ᾰσθω", _, "ἀ~ᾰσθωμεν";
    aorist_subj_pass_2: "ἀ~ᾰσθῃς", "ἀ~ᾰσθητον", "ἀ~ᾰσθητε";
    aorist_subj_pass_3: "ἀ~ᾰσθῃ", _, "ἀ~ᾰσθωσῐ|ἀ~ᾰσθωσῐν";
    aorist_opt_mid_1: "ἀ~ᾰσαιμην", _, "ἀ~ᾰσαιμεθᾰ";
    aorist_opt_mid_2: "ἀ~ᾰσαιο", "ἀ~ᾰσαισθον", "ἀ~ᾰσαισθε";
    aorist_opt_mid_3: "ἀ~ᾰσαιτο", "ἀ~ᾰσαισθην", "ἀ~ᾰσαιντο";
    aorist_opt_pass_1: "ἀ~ᾰσθειην", _, "ἀ~ᾰσθεῑμεν|ἀ~ᾰσθειημεν";
    aorist_opt_pass_2: "ἀ~ᾰσθειης", "ἀ~ᾰσθεῑτον|ἀ~ᾰσθειητον", "ἀ~ᾰσθεῑτε|ἀ~ᾰσθειητε";
    aorist_opt_pass_3: "ἀ~ᾰσθειη", "ἀ~ᾰσθειτην|ἀ~ᾰσθειητην", "ἀ~ᾰσθεῑεν|ἀ~ᾰσθειησᾰν";
    aorist_imp_mid_2: "ἀ~ᾰσαι", "ἀ~ᾰσᾰσθον", "ἀ~ᾰσᾰσθε";
    aorist_imp_mid_3: "ἀ~ᾰσᾰσθω", "ἀ~ᾰσᾰσθων", _;
    aorist_imp_pass_2: "ἀ~ᾰσθητῐ", _, _;
    aorist_imp_pass_3: "ἀ~ᾰσθητω", "ἀ~ᾰσθητων", "ἀ~ᾰσθεντων";
    infinitive_fut: _, "ἀ~ᾰσσεσθαι", "ἀ~ᾰσθησεσθαι";
    infinitive_aor: _, "ἀ~ᾰσᾰσθαι", "ἀ~ᾰσθηναι";
    participle_fut: _, "ἀ~ᾰσσομενος", "ἀ~ᾰσθησομενος";
    participle_aor: _, "ἀ~ᾰσᾰμενος", "ἀ~ᾰσθεις";
    ),
    // Σῠνιστημῐ: 1 words (σῠνιστημῐ). Like Ἐπεντῐθημῐ, except in 263 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("σῠνιστημῐ", 1, "~νι~ημῐ", like Ἐπεντῐθημῐ;
    ind_act_1: "~νι~ημῐ", _, "~νι~ᾰμεν";
    ind_act_2: "~νι~ης", _, _;
    ind_act_3: "~νι~ησῐ|~νι~ησῐν", _, "~νι~ᾱσῐ|~νι~ᾱσῐν";
    ind_mid_1: "~νι~ᾰμαι", _, "~νι~ᾰμεθᾰ";
    ind_mid_2: "~νι~ᾰσαι", "~νι~ᾰσθον", "~νι~ᾰσθε";
    ind_mid_3: "~νι~ᾰται", _, "~νι~ᾰνται";
    ind_pass_1: _, _, "~νι~ᾰμεθᾰ";
    ind_pass_2: _, _, "~νι~ᾰσθε";
    ind_pass_3: "~νι~ᾰται", _, "~νι~ᾰνται";
    subj_act_1: "~νι~ω", _, "~νι~ωμεν";
    subj_act_2: "~νι~ῃς", "~νι~ητον", "~νι~ητε";
    subj_act_3: "~νι~ῃ", _, "~νι~ωσῐ|~νι~ωσῐν";
    subj_mid_1: "~νι~ωμαι", _, "~νι~ωμεθᾰ";
    subj_mid_2: "~νι~ῃ", "~νι~ησθον", "~νι~ησθε";
    subj_mid_3: "~νι~ηται", _, "~νι~ωνται";
    subj_pass_1: _, _, "~νι~ωμεθᾰ";
    subj_pass_2: _, _, "~νι~ησθε";
    subj_pass_3: "~νι~ηται", _, "~νι~ωνται";
    opt_act_1: "~νι~αιην", _, "~νι~αῑμεν|~νι~αιημεν";
    opt_act_2: "~νι~αιης", "~νι~αῑτον|~νι~αιητον", "~νι~αῑτε|~νι~αιητε";
    opt_act_3: "~νι~αιη", "~νι~αιτην|~νι~αιητην", "~νι~αῑεν|~νι~αιησᾰν";
    opt_mid_1: "~νι~αιμην", _, "~νι~αιμεθᾰ";
    opt_mid_2: "~νι~αιο", "~νι~αισθον", "~νι~αισθε";
    opt_mid_3: "~νι~αιτο", "~νι~αισθην", "~νι~αιντο";
    opt_pass_1: _, _, "~νι~αιμεθᾰ";
    opt_pass_2: _, "~νι~αισθον", "~νι~αισθε";
    opt_pass_3: "~νι~αιτο", "~νι~αισθην", "~νι~αιντο";
    imp_act_2: "~νι~η", "~νι~ᾰτον", "~νι~ᾰτε";
    imp_act_3: "~νι~ᾰτω", "~νι~ᾰτων", "~νι~ᾰντων";
    imp_mid_2: "~νι~ᾰσο|~νι~ω", _, "~νι~ᾰσθε";
    imp_mid_3: "~νι~ᾰσθω", "~νι~ᾰσθων", "~νι~ᾰσθων";
    imp_pass_2: "~νι~ᾰσο|~νι~ω", _, "~νι~ᾰσθε";
    imp_pass_3: _, "~νι~ᾰσθων", _;
    imperfect_act_1: "~νῑ~ην", _, "~νῑ~ᾰμεν";
    imperfect_act_2: "~νῑ~ης", "~νῑ~ᾰτον", "~νῑ~ᾰτε";
    imperfect_act_3: "~νῑ~η", "~νῑ~ᾰτην", "~νῑ~ᾰσᾰν";
    imperfect_mid_1: "~νῑ~ᾰμην", _, "~νῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~νῑ~ᾰσο", "~νῑ~ᾰσθον", "~νῑ~ᾰσθε";
    imperfect_mid_3: "~νῑ~ᾰτο", "~νῑ~ᾰσθην", "~νῑ~ᾰντο";
    imperfect_pass_1: _, _, "~νῑ~ᾰμεθᾰ";
    imperfect_pass_2: _, _, "~νῑ~ᾰσθε";
    imperfect_pass_3: "~νῑ~ᾰτο", _, _;
    future_ind_pass_1: "~~ᾰθησομαι", _, "~~ᾰθησομεθᾰ";
    future_ind_pass_2: "~~ᾰθησῃ|~~ᾰθησει", "~~ᾰθησεσθον", "~~ᾰθησεσθε";
    future_ind_pass_3: "~~ᾰθησεται", _, "~~ᾰθησονται";
    future_opt_pass_1: "~~ᾰθησοιμην", _, "~~ᾰθησοιμεθᾰ";
    future_opt_pass_2: "~~ᾰθησοιο", "~~ᾰθησοισθον", "~~ᾰθησοισθε";
    future_opt_pass_3: "~~ᾰθησοιτο", "~~ᾰθησοισθην", "~~ᾰθησοιντο";
    aorist_ind_act_1: "~νε~ησᾰ|~νε~ην", _, "~νε~ησᾰμεν|~νε~ημεν";
    aorist_ind_act_2: "~νε~ησᾰς|~νε~ης", "~νε~ησᾰτον|~νε~ητον", "~νε~ησᾰτε|~νε~ητε";
    aorist_ind_act_3: "~νε~ησε|~νε~ησεν|~νε~η", "~νε~ησᾰτην|~νε~ητην", "~νε~ησᾰν";
    aorist_ind_mid_1: "~νε~ησᾰμην|~νε~ᾰμην", _, "~νε~ησᾰμεθᾰ|~νε~ᾰμεθᾰ";
    aorist_ind_mid_2: "~νε~ησω|~νε~ω", "~νε~ησᾰσθον|~νε~ᾰσθον", "~νε~ησᾰσθε|~νε~ᾰσθε";
    aorist_ind_mid_3: "~νε~ησᾰτο|~νε~ᾰτο", "~νε~ησᾰσθην|~νε~ᾰσθην", "~νε~ησᾰντο|~νε~ᾰντο";
    aorist_ind_pass_1: "~νε~ᾰθην|~νε~ην", _, "~νε~ᾰθημεν|~νε~ημεν";
    aorist_ind_pass_2: "~νε~ᾰθης|~νε~ης", "~νε~ᾰθητον|~νε~ητον", "~νε~ᾰθητε|~νε~ητε";
    aorist_ind_pass_3: "~νε~ᾰθη|~νε~η", "~νε~ᾰθητην|~νε~ητην", "~νε~ᾰθησᾰν|~νε~ησᾰν";
    aorist_subj_act_1: "~~ησω|~~ω", _, "~~ησωμεν|~~ωμεν";
    aorist_subj_act_2: "~~ησῃς|~~ῃς", "~~ησητον|~~ητον", "~~ησητε|~~ητε";
    aorist_subj_act_3: "~~ησῃ|~~ῃ", "~~ησητον|~~ητον", "~~ησωσῐ|~~ησωσῐν|~~ωσῐ|~~ωσῐν";
    aorist_subj_mid_1: "~~ησωμαι|~~ωμαι", _, "~~ησωμεθᾰ|~~ωμεθᾰ";
    aorist_subj_mid_2: "~~ησῃ|~~ῃ", "~~ησησθον|~~ησθον", "~~ησησθε|~~ησθε";
    aorist_subj_mid_3: "~~ησηται|~~ηται", "~~ησησθον|~~ησθον", "~~ησωνται|~~ωνται";
    aorist_subj_pass_1: "~~ᾰθω|~~ω", _, "~~ᾰθωμεν|~~ωμεν";
    aorist_subj_pass_2: "~~ᾰθῃς|~~ῃς", "~~ᾰθητον|~~ητον", "~~ᾰθητε|~~ητε";
    aorist_subj_pass_3: "~~ᾰθῃ|~~ῃ", "~~ᾰθητον|~~ητον", "~~ᾰθωσῐ|~~ᾰθωσῐν|~~ωσῐ|~~ωσῐν";
    aorist_opt_act_1: "~~ησαιμῐ|~~αιην", _, "~~ησαιμεν|~~αῑμεν|~~αιημεν";
    aorist_opt_act_2: "~~ησειᾰς|~~ησαις|~~αιης", "~~ησαιτον|~~αῑτον|~~αιητον", "~~ησαιτε|~~αῑτε|~~αιητε";
    aorist_opt_act_3: "~~ησειε|~~ησειεν|~~ησαι|~~αιη", "~~ησαιτην|~~αιτην|~~αιητην", "~~ησειᾰν|~~ησαιεν|~~αῑεν|~~αιησᾰν";
    aorist_opt_mid_1: "~~ησαιμην|~~αιμην", _, "~~ησαιμεθᾰ|~~αιμεθᾰ";
    aorist_opt_mid_2: "~~ησαιο|~~αῑο", "~~ησαισθον|~~αῑσθον", "~~ησαισθε|~~αῑσθε";
    aorist_opt_mid_3: "~~ησαιτο|~~αῑτο", "~~ησαισθην|~~αισθην", "~~ησαιντο|~~αῑντο";
    aorist_opt_pass_1: "~~ᾰθειην|~~ειην", _, "~~ᾰθεῑμεν|~~ᾰθειημεν|~~εῑμεν|~~ειημεν";
    aorist_opt_pass_2: "~~ᾰθειης|~~ειης", "~~ᾰθεῑτον|~~ᾰθειητον|~~εῑτον|~~ειητον", "~~ᾰθεῑτε|~~ᾰθειητε|~~εῑτε|~~ειητε";
    aorist_opt_pass_3: "~~ᾰθειη|~~ειη", "~~ᾰθειτην|~~ᾰθειητην|~~ειτην|~~ειητην", "~~ᾰθεῑεν|~~ᾰθειησᾰν|~~εῑεν|~~ειησᾰν";
    aorist_imp_act_2: "~~ησον|~~ηθῐ", "~~ησᾰτον|~~ητον", "~~ησᾰτε|~~ητε";
    aorist_imp_act_3: "~~ησᾰτω|~~ητω", "~~ησᾰτων|~~ητων", "~~ησᾰντων|~~ᾰντων";
    aorist_imp_mid_2: "~~ησαι|~~ω", "~~ησᾰσθον|~~ᾰσθον", "~~ησᾰσθε|~~ᾰσθε";
    aorist_imp_mid_3: "~~ησᾰσθω|~~ᾰσθω", "~~ησᾰσθων|~~ᾰσθων", "~~ησᾰσθων|~~ᾰσθων";
    aorist_imp_pass_2: "~~ᾰθητῐ|~~ηθῐ", "~~ᾰθητον|~~ητον", "~~ᾰθητε|~~ητε";
    aorist_imp_pass_3: "~~ᾰθητω|~~ητω", "~~ᾰθητων|~~ητων", "~~ᾰθεντων|~~εντων";
    perfect_ind_act_1: "~νε~ηκᾰ|~νε~ᾰκᾰ", _, "~νε~ηκᾰμεν|~νε~ᾰκᾰμεν|~νε~ᾰμεν";
    perfect_ind_act_2: "~νε~ηκᾰς|~νε~ᾰκᾰς", "~νε~ηκᾰτον|~νε~ᾰκᾰτον|~νε~ᾰτον", "~νε~ηκᾰτε|~νε~ᾰκᾰτε|~νε~ᾰτε";
    perfect_ind_act_3: "~νε~ηκε|~νε~ηκεν|~νε~ᾰκε|~νε~ᾰκεν", "~νε~ηκᾰτον|~νε~ᾰκᾰτον|~νε~ᾰτον", "~νε~ηκᾱσῐ|~νε~ηκᾱσῐν|~νε~ᾰκᾱσῐ|~νε~ᾰκᾱσῐν|~νε~ᾱσῐ|~νε~ᾱσῐν";
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, derived;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "~νε~ηκω|~νε~ᾰκω|~νε~ω", _, "~νε~ηκωμεν|~νε~ᾰκωμεν|~νε~ωμεν";
    perfect_subj_act_2: "~νε~ηκῃς|~νε~ᾰκῃς|~νε~ῃς", "~νε~ηκητον|~νε~ᾰκητον|~νε~ητον", "~νε~ηκητε|~νε~ᾰκητε|~νε~ητε";
    perfect_subj_act_3: "~νε~ηκῃ|~νε~ᾰκῃ|~νε~ῃ", "~νε~ηκητον|~νε~ᾰκητον|~νε~ητον", "~νε~ηκωσῐ|~νε~ηκωσῐν|~νε~ᾰκωσῐ|~νε~ᾰκωσῐν|~νε~ωσῐ|~νε~ωσῐν";
    perfect_opt_act_1: "~νε~ηκοιμῐ|~νε~ηκοιην|~νε~ᾰκοιμῐ|~νε~ᾰκοιην|~νε~αιην", _, "~νε~ηκοιμεν|~νε~ᾰκοιμεν|~νε~αῑμεν|~νε~αιημεν";
    perfect_opt_act_2: "~νε~ηκοις|~νε~ηκοιης|~νε~ᾰκοις|~νε~ᾰκοιης|~νε~αιης", "~νε~ηκοιτον|~νε~ᾰκοιτον|~νε~αῑτον|~νε~αιητον", "~νε~ηκοιτε|~νε~ᾰκοιτε|~νε~αῑτε|~νε~αιητε";
    perfect_opt_act_3: "~νε~ηκοι|~νε~ηκοιη|~νε~ᾰκοι|~νε~ᾰκοιη|~νε~αιη", "~νε~ηκοιτην|~νε~ᾰκοιτην|~νε~αιτην|~νε~αιητην", "~νε~ηκοιεν|~νε~ᾰκοιεν|~νε~αῑεν|~νε~αιησᾰν";
    perfect_imp_act_2: "~νε~ηκε|~νε~ᾰκε|~νε~ᾰθῐ", "~νε~ηκετον|~νε~ᾰκετον|~νε~ᾰτον", "~νε~ηκετε|~νε~ᾰκετε|~νε~ᾰτε";
    perfect_imp_act_3: "~νε~ηκετω|~νε~ᾰκετω|~νε~ᾰτω", "~νε~ηκετων|~νε~ᾰκετων|~νε~ᾰτων", "~νε~ηκοντων|~νε~ᾰκοντων|~νε~ᾰντων";
    perfect_imp_mid_2: derived, _, _;
    perfect_imp_mid_3: derived, derived, _;
    perfect_imp_pass_2: derived, _, _;
    perfect_imp_pass_3: derived, derived, _;
    pluperfect_act_1: "~νει~ηκειν|~νει~ηκη", _, "~νει~ηκεμεν";
    pluperfect_act_2: "~νει~ηκεις|~νει~ηκης", "~νει~ηκετον", "~νει~ηκετε";
    pluperfect_act_3: "~νει~ηκει|~νει~ηκειν", "~νει~ηκετην", "~νει~ηκεσᾰν";
    infinitive_pres: "~νι~ᾰναι", "~νι~ᾰσθαι", _;
    infinitive_fut: _, _, "~~ᾰθησεσθαι";
    infinitive_aor: "~~ησαι|~~ηναι", "~~ησᾰσθαι|~~ᾰσθαι", "~~ᾰθηναι|~~ηναι";
    infinitive_perf: "~νε~ηκεναι|~νε~ᾰκεναι|~νε~ᾰναι", derived, _;
    participle_pres: "~νι~ᾱς", "~νι~ᾰμενος", _;
    participle_fut: _, _, "~~ᾰθησομενος";
    participle_aor: "~~ησᾱς|~~ᾱς", "~~ησᾰμενος|~~ᾰμενος", "~~ᾰθεις|~~εις";
    participle_perf: "~νε~ηκως|~νε~ᾰκως|~νε~ως|~νε~ᾰως|~νε~εως", derived, _;
    ),
    // Ξῠνιστημῐ: 1 words (ξῠνιστημῐ). Like Σῠνιστημῐ, except in 144 cells: future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, future_ind_pass_2 du, future_ind_pass_2 pl, future_ind_pass_3 sg, ….
    conjugation!("ξῠνιστημῐ", 1, "~νι~ημῐ", like Σῠνιστημῐ;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_act_1: "~νε~ην", _, "~νε~ημεν";
    aorist_ind_act_2: "~νε~ης", "~νε~ητον", "~νε~ητε";
    aorist_ind_act_3: "~νε~η", "~νε~ητην", _;
    aorist_ind_mid_1: "~νε~ᾰμην", _, "~νε~ᾰμεθᾰ";
    aorist_ind_mid_2: "~νε~ω", "~νε~ᾰσθον", "~νε~ᾰσθε";
    aorist_ind_mid_3: "~νε~ᾰτο", "~νε~ᾰσθην", "~νε~ᾰντο";
    aorist_ind_pass_1: "~νε~ᾰθην", _, "~νε~ᾰθημεν";
    aorist_ind_pass_2: "~νε~ᾰθης", "~νε~ᾰθητον", "~νε~ᾰθητε";
    aorist_ind_pass_3: "~νε~ᾰθη", "~νε~ᾰθητην", "~νε~ᾰθησᾰν";
    aorist_subj_act_1: "~~ω", _, "~~ωμεν";
    aorist_subj_act_2: "~~ῃς", "~~ητον", "~~ητε";
    aorist_subj_act_3: "~~ῃ", derived, "~~ωσῐ|~~ωσῐν";
    aorist_subj_mid_1: "~~ωμαι", _, "~~ωμεθᾰ";
    aorist_subj_mid_2: "~~ῃ", "~~ησθον", "~~ησθε";
    aorist_subj_mid_3: "~~ηται", derived, "~~ωνται";
    aorist_subj_pass_1: "~~ᾰθω", _, "~~ᾰθωμεν";
    aorist_subj_pass_2: "~~ᾰθῃς", "~~ᾰθητον", "~~ᾰθητε";
    aorist_subj_pass_3: "~~ᾰθῃ", derived, "~~ᾰθωσῐ|~~ᾰθωσῐν";
    aorist_opt_act_1: "~~αιην", _, "~~αῑμεν|~~αιημεν";
    aorist_opt_act_2: "~~αιης", "~~αῑτον|~~αιητον", "~~αῑτε|~~αιητε";
    aorist_opt_act_3: "~~αιη", "~~αιτην|~~αιητην", "~~αῑεν|~~αιησᾰν";
    aorist_opt_mid_1: "~~αιμην", _, "~~αιμεθᾰ";
    aorist_opt_mid_2: "~~αῑο", "~~αῑσθον", "~~αῑσθε";
    aorist_opt_mid_3: "~~αῑτο", "~~αισθην", "~~αῑντο";
    aorist_opt_pass_1: "~~ᾰθειην", _, "~~ᾰθεῑμεν|~~ᾰθειημεν";
    aorist_opt_pass_2: "~~ᾰθειης", "~~ᾰθεῑτον|~~ᾰθειητον", "~~ᾰθεῑτε|~~ᾰθειητε";
    aorist_opt_pass_3: "~~ᾰθειη", "~~ᾰθειτην|~~ᾰθειητην", "~~ᾰθεῑεν|~~ᾰθειησᾰν";
    aorist_imp_act_2: "~~ηθῐ", "~~ητον", "~~ητε";
    aorist_imp_act_3: "~~ητω", "~~ητων", "~~ᾰντων";
    aorist_imp_mid_2: "~~ω", "~~ᾰσθον", "~~ᾰσθε";
    aorist_imp_mid_3: "~~ᾰσθω", "~~ᾰσθων", derived;
    aorist_imp_pass_2: "~~ᾰθηθῐ", derived, derived;
    aorist_imp_pass_3: "~~ᾰθητω", "~~ᾰθητων", "~~ᾰθεντων";
    perfect_ind_act_1: "~νε~ηκᾰ", _, "~νε~ηκᾰμεν|~νε~ᾰμεν";
    perfect_ind_act_2: "~νε~ηκᾰς|~νε~ᾰς", "~νε~ηκᾰτον|~νε~ᾰτον", "~νε~ηκᾰτε|~νε~ᾰτε";
    perfect_ind_act_3: "~νε~ηκε|~νε~ηκεν", "~νε~ηκᾰτον|~νε~ᾰτον", "~νε~ηκᾱσῐ|~νε~ηκᾱσῐν|~νε~ᾱσῐ|~νε~ᾱσῐν";
    perfect_subj_act_1: "~νε~ηκω|~νε~ω", _, "~νε~ηκωμεν|~νε~ωμεν";
    perfect_subj_act_2: "~νε~ηκῃς|~νε~ῃς", "~νε~ηκητον|~νε~ητον", "~νε~ηκητε|~νε~ητε";
    perfect_subj_act_3: "~νε~ηκῃ|~νε~ῃ", "~νε~ηκητον|~νε~ητον", "~νε~ηκωσῐ|~νε~ηκωσῐν|~νε~ωσῐ|~νε~ωσῐν";
    perfect_opt_act_1: "~νε~ηκοιμῐ|~νε~ηκοιην|~νε~αιην", _, "~νε~ηκοιμεν|~νε~αῑμεν|~νε~αιημεν";
    perfect_opt_act_2: "~νε~ηκοις|~νε~ηκοιης|~νε~αιης", "~νε~ηκοιτον|~νε~αῑτον|~νε~αιητον", "~νε~ηκοιτε|~νε~αῑτε|~νε~αιητε";
    perfect_opt_act_3: "~νε~ηκοι|~νε~ηκοιη|~νε~αιη", "~νε~ηκοιτην|~νε~αιτην|~νε~αιητην", "~νε~ηκοιεν|~νε~αῑεν|~νε~αιησᾰν";
    perfect_imp_act_2: "~νε~ηκε|~νε~ᾰθῐ", "~νε~ηκετον|~νε~ᾰτον", "~νε~ηκετε|~νε~ᾰτε";
    perfect_imp_act_3: "~νε~ηκετω|~νε~ᾰτω", "~νε~ηκετων|~νε~ᾰτων", "~νε~ηκοντων|~νε~ᾰντων";
    infinitive_fut: _, _, derived;
    infinitive_aor: "~~ηναι", "~~ᾰσθαι", "~~ᾰθηναι";
    infinitive_perf: "~νε~ηκεναι|~νε~ᾰναι", _, _;
    participle_fut: _, _, derived;
    participle_aor: "~~ᾱς", "~~ᾰμενος", "~~ᾰθεις";
    participle_perf: "~νε~ηκως|~νε~ως", _, _;
    ),
    // Ἀπᾰνῐστημῐ: 1 words (ἀπᾰνῐστημῐ). Like Ἀνῐστημῐ, except in 132 cells: aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_2 pl, aorist_ind_act_3 sg, ….
    conjugation!("ἀπᾰνῐστημῐ", 1, "~ῐ~ημῐ", like Ἀνῐστημῐ;
    aorist_ind_act_1: "~ε~ην", _, "~ε~ημεν";
    aorist_ind_act_2: "~ε~ης", "~ε~ητον", "~ε~ητε";
    aorist_ind_act_3: "~ε~η", "~ε~ητην", _;
    aorist_ind_mid_1: "~ε~ᾰμην", _, "~ε~ᾰμεθᾰ";
    aorist_ind_mid_2: "~ε~ω", "~ε~ᾰσθον", "~ε~ᾰσθε";
    aorist_ind_mid_3: "~ε~ᾰτο", "~ε~ᾰσθην", "~ε~ᾰντο";
    aorist_ind_pass_1: derived, _, derived;
    aorist_ind_pass_2: derived, derived, derived;
    aorist_ind_pass_3: derived, derived, derived;
    aorist_subj_act_1: "~ᾰ~ω", _, "~ᾰ~ωμεν";
    aorist_subj_act_2: "~ᾰ~ῃς", "~ᾰ~ητον", "~ᾰ~ητε";
    aorist_subj_act_3: "~ᾰ~ῃ", derived, "~ᾰ~ωσῐ|~ᾰ~ωσῐν";
    aorist_subj_mid_1: "~ᾰ~ωμαι", _, "~ᾰ~ωμεθᾰ";
    aorist_subj_mid_2: "~ᾰ~ῃ", "~ᾰ~ησθον", "~ᾰ~ησθε";
    aorist_subj_mid_3: "~ᾰ~ηται", derived, "~ᾰ~ωνται";
    aorist_subj_pass_1: derived, _, derived;
    aorist_subj_pass_2: derived, derived, derived;
    aorist_subj_pass_3: derived, _, derived;
    aorist_opt_act_1: "~ᾰ~αιην", _, "~ᾰ~αῑμεν|~ᾰ~αιημεν";
    aorist_opt_act_2: "~ᾰ~αιης", "~ᾰ~αῑτον|~ᾰ~αιητον", "~ᾰ~αῑτε|~ᾰ~αιητε";
    aorist_opt_act_3: "~ᾰ~αιη", "~ᾰ~αιτην|~ᾰ~αιητην", "~ᾰ~αῑεν|~ᾰ~αιησᾰν";
    aorist_opt_mid_1: "~ᾰ~αιμην", _, "~ᾰ~αιμεθᾰ";
    aorist_opt_mid_2: "~ᾰ~αῑο", "~ᾰ~αῑσθον", "~ᾰ~αῑσθε";
    aorist_opt_mid_3: "~ᾰ~αῑτο", "~ᾰ~αισθην", "~ᾰ~αῑντο";
    aorist_opt_pass_1: derived, _, derived;
    aorist_opt_pass_2: derived, derived, derived;
    aorist_opt_pass_3: derived, derived, derived;
    aorist_imp_act_2: "~ᾰ~ηθῐ", "~ᾰ~ητον", "~ᾰ~ητε";
    aorist_imp_act_3: "~ᾰ~ητω", "~ᾰ~ητων", "~ᾰ~ᾰντων";
    aorist_imp_mid_2: "~ᾰ~ω", "~ᾰ~ᾰσθον", "~ᾰ~ᾰσθε";
    aorist_imp_mid_3: "~ᾰ~ᾰσθω", "~ᾰ~ᾰσθων", derived;
    aorist_imp_pass_2: derived, _, _;
    aorist_imp_pass_3: derived, derived, derived;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: derived, derived, derived;
    perfect_ind_act_3: derived, derived, derived;
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, derived, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, derived;
    perfect_imp_act_3: derived, derived, derived;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    infinitive_aor: "~ᾰ~ηναι", "~ᾰ~ᾰσθαι", derived;
    infinitive_perf: derived, _, _;
    participle_aor: "~ᾰ~ᾱς", "~ᾰ~ᾰμενος", derived;
    participle_perf: derived, _, _;
    ),
    // Καινῠμαι: 1 words (καινῠμαι). Like Μᾰρνᾰμαι, except in 92 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("καινῠμαι", 1, "~αινῠμαι", like Μᾰρνᾰμαι;
    ind_act_1: "~αινῠμαι", _, _;
    ind_mid_1: "~αινῠμαι", _, "~αινῠμεθᾰ";
    ind_mid_2: "~αινῠσαι", "~αινῠσθον", "~αινῠσθε";
    ind_mid_3: "~αινῠται", _, "~αινῠνται";
    ind_pass_1: _, _, "~αινῠμεθᾰ";
    ind_pass_2: _, _, "~αινῠσθε";
    ind_pass_3: "~αινῠται", _, "~αινῠνται";
    subj_mid_1: "~αινῠωμαι", _, "~αινῠωμεθᾰ";
    subj_mid_2: "~αινῠῃ", "~αινῠησθον", "~αινῠησθε";
    subj_mid_3: "~αινῠηται", _, "~αινῠωνται";
    subj_pass_1: _, _, "~αινῠωμεθᾰ";
    subj_pass_2: _, _, "~αινῠησθε";
    subj_pass_3: "~αινῠηται", _, "~αινῠωνται";
    opt_mid_1: "~αινῠοιμην", _, "~αινῠοιμεθᾰ";
    opt_mid_2: "~αινῠοιο", "~αινῠοισθον", "~αινῠοισθε";
    opt_mid_3: "~αινῠοιτο", "~αινῠοισθην", "~αινῠοιντο";
    opt_pass_1: _, _, "~αινῠοιμεθᾰ";
    opt_pass_2: _, "~αινῠοισθον", "~αινῠοισθε";
    opt_pass_3: "~αινῠοιτο", "~αινῠοισθην", "~αινῠοιντο";
    imp_mid_2: "~αινῠσο", _, "~αινῠσθε";
    imp_mid_3: "~αινῠσθω", "~αινῠσθων", "~αινῠσθων";
    imp_pass_2: derived, _, "~αινῠσθε";
    imp_pass_3: _, "~αινῠσθων", _;
    imperfect_mid_1: "ἐ~αινῠμην", _, "ἐ~αινῠμεθᾰ";
    imperfect_mid_2: "ἐ~αινῠσο", "ἐ~αινῠσθον", "ἐ~αινῠσθε";
    imperfect_mid_3: "ἐ~αινῠτο", "ἐ~αινῠσθην", "ἐ~αινῠντο";
    imperfect_pass_1: _, _, "ἐ~αινῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~αινῠσθε";
    imperfect_pass_3: "ἐ~αινῠτο", _, _;
    perfect_ind_mid_1: "~εκᾰσμαι", _, "~εκᾰσμεθᾰ";
    perfect_ind_mid_2: "~εκασαι|~εκασσαι", "~εκᾰσθον", "~εκᾰσθε";
    perfect_ind_mid_3: "~εκᾰσται", _, "~εκᾰδᾰται";
    perfect_ind_pass_1: _, _, "~εκᾰσμεθᾰ";
    perfect_ind_pass_2: "~εκασαι|~εκασσαι", _, "~εκᾰσθε";
    perfect_imp_mid_2: "~εκᾰσο", _, _;
    perfect_imp_mid_3: "~εκᾰσθω", "~εκᾰσθων", _;
    perfect_imp_pass_2: "~εκᾰσο", _, _;
    perfect_imp_pass_3: "~εκᾰσθω", "~εκᾰσθων", _;
    pluperfect_mid_1: "ἐ~εκᾰσμην|~εκᾰσμην", _, "ἐ~εκᾰσμεθᾰ|~εκᾰσμεθᾰ";
    pluperfect_mid_2: "ἐ~εκᾰσο|~εκᾰσο", "ἐ~εκᾰσθον|~εκᾰσθον", "ἐ~εκᾰσθε|~εκᾰσθε";
    pluperfect_mid_3: "ἐ~εκᾰστο|~εκᾰστο", "ἐ~εκᾰσθην|~εκᾰσθην", "ἐ~εκᾰδᾰτο|~εκᾰδᾰτο";
    pluperfect_pass_1: "ἐ~εκᾰσμην|~εκᾰσμην", _, "ἐ~εκᾰσμεθᾰ|~εκᾰσμεθᾰ";
    pluperfect_pass_2: "ἐ~εκᾰσο|~εκᾰσο", "ἐ~εκᾰσθον|~εκᾰσθον", "ἐ~εκᾰσθε|~εκᾰσθε";
    pluperfect_pass_3: "ἐ~εκᾰστο|~εκᾰστο", "ἐ~εκᾰσθην|~εκᾰσθην", "ἐ~εκᾰδᾰτο|~εκᾰδᾰτο";
    infinitive_pres: _, "~αινῠσθαι", _;
    infinitive_perf: _, "~εκᾰσθαι", _;
    participle_pres: _, "~αινῠμενος", _;
    participle_perf: _, "~εκᾰσμενος", _;
    ),
    // Ἐγκᾰθῐστημῐ: 1 words (ἐγκᾰθῐστημῐ). Like Ἐγκειμαι, except in 261 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἐγκᾰθῐστημῐ", 1, "~θῐ~ημῐ", like Ἐγκειμαι;
    ind_act_1: "~θῐ~ημῐ", _, "~θῐ~ᾰμεν";
    ind_act_2: "~θῐ~ης", _, _;
    ind_act_3: "~θῐ~ησῐ|~θῐ~ησῐν", _, "~θῐ~ᾱσῐ|~θῐ~ᾱσῐν";
    ind_mid_1: "~θῐ~ᾰμαι", _, "~θῐ~ᾰμεθᾰ";
    ind_mid_2: "~θῐ~ᾰσαι", "~θῐ~ᾰσθον", "~θῐ~ᾰσθε";
    ind_mid_3: "~θῐ~ᾰται", _, "~θῐ~ᾰνται";
    ind_pass_1: _, _, "~θῐ~ᾰμεθᾰ";
    ind_pass_2: _, _, "~θῐ~ᾰσθε";
    ind_pass_3: "~θῐ~ᾰται", _, "~θῐ~ᾰνται";
    subj_act_1: "~θῐ~ω", _, "~θῐ~ωμεν";
    subj_act_2: "~θῐ~ῃς", "~θῐ~ητον", "~θῐ~ητε";
    subj_act_3: "~θῐ~ῃ", _, "~θῐ~ωσῐ|~θῐ~ωσῐν";
    subj_mid_1: "~θῐ~ωμαι", _, "~θῐ~ωμεθᾰ";
    subj_mid_2: "~θῐ~ῃ", "~θῐ~ησθον", "~θῐ~ησθε";
    subj_mid_3: "~θῐ~ηται", _, "~θῐ~ωνται";
    subj_pass_1: _, _, "~θῐ~ωμεθᾰ";
    subj_pass_2: _, _, "~θῐ~ησθε";
    subj_pass_3: "~θῐ~ηται", _, "~θῐ~ωνται";
    opt_act_1: "~θῐ~αιην", _, "~θῐ~αῑμεν|~θῐ~αιημεν";
    opt_act_2: "~θῐ~αιης", "~θῐ~αῑτον|~θῐ~αιητον", "~θῐ~αῑτε|~θῐ~αιητε";
    opt_act_3: "~θῐ~αιη", "~θῐ~αιτην|~θῐ~αιητην", "~θῐ~αῑεν|~θῐ~αιησᾰν";
    opt_mid_1: "~θῐ~αιμην", _, "~θῐ~αιμεθᾰ";
    opt_mid_2: "~θῐ~αιο", "~θῐ~αισθον", "~θῐ~αισθε";
    opt_mid_3: "~θῐ~αιτο", "~θῐ~αισθην", "~θῐ~αιντο";
    opt_pass_1: _, _, "~θῐ~αιμεθᾰ";
    opt_pass_2: _, "~θῐ~αισθον", "~θῐ~αισθε";
    opt_pass_3: "~θῐ~αιτο", "~θῐ~αισθην", "~θῐ~αιντο";
    imp_act_2: "~θῐ~η", "~θῐ~ᾰτον", "~θῐ~ᾰτε";
    imp_act_3: "~θῐ~ᾰτω", "~θῐ~ᾰτων", "~θῐ~ᾰντων";
    imp_mid_2: "~θῐ~ᾰσο|~θῐ~ω", _, "~θῐ~ᾰσθε";
    imp_mid_3: "~θῐ~ᾰσθω", "~θῐ~ᾰσθων", "~θῐ~ᾰσθων";
    imp_pass_2: "~θῐ~ᾰσο|~θῐ~ω", _, "~θῐ~ᾰσθε";
    imp_pass_3: _, "~θῐ~ᾰσθων", _;
    imperfect_act_1: "~θῑ~ην", _, "~θῑ~ᾰμεν";
    imperfect_act_2: "~θῑ~ης", "~θῑ~ᾰτον", "~θῑ~ᾰτε";
    imperfect_act_3: "~θῑ~η", "~θῑ~ᾰτην", "~θῑ~ᾰσᾰν";
    imperfect_mid_1: "~θῑ~ᾰμην", _, "~θῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~θῑ~ᾰσο", "~θῑ~ᾰσθον", "~θῑ~ᾰσθε";
    imperfect_mid_3: "~θῑ~ᾰτο", "~θῑ~ᾰσθην", "~θῑ~ᾰντο";
    imperfect_pass_1: _, _, "~θῑ~ᾰμεθᾰ";
    imperfect_pass_2: _, _, "~θῑ~ᾰσθε";
    imperfect_pass_3: "~θῑ~ᾰτο", _, _;
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
    aorist_ind_act_1: "~τε~ησᾰ", _, "~τε~ησᾰμεν";
    aorist_ind_act_2: "~τε~ησᾰς", "~τε~ησᾰτον", "~τε~ησᾰτε";
    aorist_ind_act_3: "~τε~ησε|~τε~ησεν", "~τε~ησᾰτην", "~τε~ησᾰν";
    aorist_ind_mid_1: "~τε~ησᾰμην", _, "~τε~ησᾰμεθᾰ";
    aorist_ind_mid_2: "~τε~ησω", "~τε~ησᾰσθον", "~τε~ησᾰσθε";
    aorist_ind_mid_3: "~τε~ησᾰτο", "~τε~ησᾰσθην", "~τε~ησᾰντο";
    aorist_ind_pass_1: "~τε~ᾰθην", _, "~τε~ᾰθημεν";
    aorist_ind_pass_2: "~τε~ᾰθης", "~τε~ᾰθητον", "~τε~ᾰθητε";
    aorist_ind_pass_3: "~τε~ᾰθη", "~τε~ᾰθητην", "~τε~ᾰθησᾰν";
    aorist_subj_act_1: "~τᾰ~ησω", _, "~τᾰ~ησωμεν";
    aorist_subj_act_2: "~τᾰ~ησῃς", "~τᾰ~ησητον", "~τᾰ~ησητε";
    aorist_subj_act_3: "~τᾰ~ησῃ", _, "~τᾰ~ησωσῐ|~τᾰ~ησωσῐν";
    aorist_subj_mid_1: "~τᾰ~ησωμαι", _, "~τᾰ~ησωμεθᾰ";
    aorist_subj_mid_2: "~τᾰ~ησῃ", "~τᾰ~ησησθον", "~τᾰ~ησησθε";
    aorist_subj_mid_3: "~τᾰ~ησηται", _, "~τᾰ~ησωνται";
    aorist_subj_pass_1: "~τᾰ~ᾰθω", _, "~τᾰ~ᾰθωμεν";
    aorist_subj_pass_2: "~τᾰ~ᾰθῃς", "~τᾰ~ᾰθητον", "~τᾰ~ᾰθητε";
    aorist_subj_pass_3: "~τᾰ~ᾰθῃ", _, "~τᾰ~ᾰθωσῐ|~τᾰ~ᾰθωσῐν";
    aorist_opt_act_1: "~τᾰ~ησαιμῐ", _, "~τᾰ~ησαιμεν";
    aorist_opt_act_2: "~τᾰ~ησειᾰς|~τᾰ~ησαις", "~τᾰ~ησαιτον", "~τᾰ~ησαιτε";
    aorist_opt_act_3: "~τᾰ~ησειε|~τᾰ~ησειεν|~τᾰ~ησαι", "~τᾰ~ησαιτην", "~τᾰ~ησειᾰν|~τᾰ~ησαιεν";
    aorist_opt_mid_1: "~τᾰ~ησαιμην", _, "~τᾰ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~τᾰ~ησαιο", "~τᾰ~ησαισθον", "~τᾰ~ησαισθε";
    aorist_opt_mid_3: "~τᾰ~ησαιτο", "~τᾰ~ησαισθην", "~τᾰ~ησαιντο";
    aorist_opt_pass_1: "~τᾰ~ᾰθειην", _, "~τᾰ~ᾰθεῑμεν|~τᾰ~ᾰθειημεν";
    aorist_opt_pass_2: "~τᾰ~ᾰθειης", "~τᾰ~ᾰθεῑτον|~τᾰ~ᾰθειητον", "~τᾰ~ᾰθεῑτε|~τᾰ~ᾰθειητε";
    aorist_opt_pass_3: "~τᾰ~ᾰθειη", "~τᾰ~ᾰθειτην|~τᾰ~ᾰθειητην", "~τᾰ~ᾰθεῑεν|~τᾰ~ᾰθειησᾰν";
    aorist_imp_act_2: "~τᾰ~ησον", "~τᾰ~ησᾰτον", "~τᾰ~ησᾰτε";
    aorist_imp_act_3: "~τᾰ~ησᾰτω", "~τᾰ~ησᾰτων", "~τᾰ~ησᾰντων";
    aorist_imp_mid_2: "~τᾰ~ησαι", "~τᾰ~ησᾰσθον", "~τᾰ~ησᾰσθε";
    aorist_imp_mid_3: "~τᾰ~ησᾰσθω", "~τᾰ~ησᾰσθων", _;
    aorist_imp_pass_2: "~τᾰ~ᾰθητῐ", _, _;
    aorist_imp_pass_3: "~τᾰ~ᾰθητω", "~τᾰ~ᾰθητων", "~τᾰ~ᾰθεντων";
    perfect_ind_act_1: _, _, "~θε~ᾰμεν";
    perfect_ind_act_2: _, "~θε~ᾰτον", "~θε~ᾰτε";
    perfect_ind_act_3: _, _, "~θε~ᾱσῐ|~θε~ᾱσῐν";
    perfect_subj_act_1: "~θε~ω", _, "~θε~ωμεν";
    perfect_subj_act_2: "~θε~ῃς", "~θε~ητον", "~θε~ητε";
    perfect_subj_act_3: "~θε~ῃ", _, "~θε~ωσῐ|~θε~ωσῐν";
    perfect_opt_act_1: "~θε~αιην", _, "~θε~αῑμεν|~θε~αιημεν";
    perfect_opt_act_2: "~θε~αιης", "~θε~αῑτον|~θε~αιητον", "~θε~αῑτε|~θε~αιητε";
    perfect_opt_act_3: "~θε~αιη", "~θε~αιτην|~θε~αιητην", "~θε~αῑεν|~θε~αιησᾰν";
    perfect_imp_act_2: "~θε~ᾰθῐ", "~θε~ᾰτον", "~θε~ᾰτε";
    perfect_imp_act_3: "~θε~ᾰτω", "~θε~ᾰτων", "~θε~ᾰντων";
    infinitive_pres: "~θῐ~ᾰναι", "~θῐ~ᾰσθαι", _;
    infinitive_fut: "~τᾰ~ησειν", "~τᾰ~ησεσθαι", "~τᾰ~ησεσθαι";
    infinitive_aor: "~τᾰ~ησαι", "~τᾰ~ησᾰσθαι", "~τᾰ~ᾰθηναι";
    infinitive_perf: "~θε~ᾰναι", _, _;
    participle_pres: "~θῐ~ᾱς", "~θῐ~ᾰμενος", _;
    participle_fut: "~τᾰ~ησων", "~τᾰ~ησομενος", "~τᾰ~ησομενος";
    participle_aor: "~τᾰ~ησᾱς", "~τᾰ~ησᾰμενος", "~τᾰ~ᾰθεις";
    participle_perf: "~θε~ως", _, _;
    ),
    // Μεθῐστημῐ: 1 words (μεθῐστημῐ). Like Ἐγκᾰθῐστημῐ, except in 173 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("μεθῐστημῐ", 1, "~θῐ~ημῐ", like Ἐγκᾰθῐστημῐ;
    future_ind_act_1: "~τα~ησω", _, "~τα~ησομεν";
    future_ind_act_2: "~τα~ησεις", "~τα~ησετον", "~τα~ησετε";
    future_ind_act_3: "~τα~ησει", _, "~τα~ησουσῐ|~τα~ησουσῐν";
    future_ind_mid_1: "~τα~ησομαι", _, "~τα~ησομεθᾰ";
    future_ind_mid_2: "~τα~ησῃ|~τα~ησει", "~τα~ησεσθον", "~τα~ησεσθε";
    future_ind_mid_3: "~τα~ησεται", _, "~τα~ησονται";
    future_ind_pass_1: "~τα~ησομαι", _, "~τα~ησομεθᾰ";
    future_ind_pass_2: "~τα~ησῃ|~τα~ησει", "~τα~ησεσθον", "~τα~ησεσθε";
    future_ind_pass_3: "~τα~ησεται", _, "~τα~ησονται";
    future_opt_act_1: "~τα~ησοιμῐ", _, "~τα~ησοιμεν";
    future_opt_act_2: "~τα~ησοις", "~τα~ησοιτον", "~τα~ησοιτε";
    future_opt_act_3: "~τα~ησοι", "~τα~ησοιτην", "~τα~ησοιεν";
    future_opt_mid_1: "~τα~ησοιμην", _, "~τα~ησοιμεθᾰ";
    future_opt_mid_2: "~τα~ησοιο", "~τα~ησοισθον", "~τα~ησοισθε";
    future_opt_mid_3: "~τα~ησοιτο", "~τα~ησοισθην", "~τα~ησοιντο";
    future_opt_pass_1: "~τα~ησοιμην", _, "~τα~ησοιμεθᾰ";
    future_opt_pass_2: "~τα~ησοιο", "~τα~ησοισθον", "~τα~ησοισθε";
    future_opt_pass_3: "~τα~ησοιτο", "~τα~ησοισθην", "~τα~ησοιντο";
    aorist_ind_act_1: "~τε~ησᾰ|~τε~ην", _, "~τε~ησᾰμεν|~τε~ημεν";
    aorist_ind_act_2: "~τε~ησᾰς|~τε~ης", "~τε~ησᾰτον|~τε~ητον", "~τε~ησᾰτε|~τε~ητε";
    aorist_ind_act_3: "~τε~ησε|~τε~ησεν|~τε~η", "~τε~ησᾰτην|~τε~ητην", _;
    aorist_ind_mid_1: "~τε~ησᾰμην|~τε~ᾰμην", _, "~τε~ησᾰμεθᾰ|~τε~ᾰμεθᾰ";
    aorist_ind_mid_2: "~τε~ησω|~τε~ω", "~τε~ησᾰσθον|~τε~ᾰσθον", "~τε~ησᾰσθε|~τε~ᾰσθε";
    aorist_ind_mid_3: "~τε~ησᾰτο|~τε~ᾰτο", "~τε~ησᾰσθην|~τε~ᾰσθην", "~τε~ησᾰντο|~τε~ᾰντο";
    aorist_subj_act_1: "~τα~ησω|~τα~ω", _, "~τα~ησωμεν|~τα~ωμεν";
    aorist_subj_act_2: "~τα~ησῃς|~τα~ῃς", "~τα~ησητον|~τα~ητον", "~τα~ησητε|~τα~ητε";
    aorist_subj_act_3: "~τα~ησῃ|~τα~ῃ", "~τα~ησητον|~τα~ητον", "~τα~ησωσῐ|~τα~ησωσῐν|~τα~ωσῐ|~τα~ωσῐν";
    aorist_subj_mid_1: "~τα~ησωμαι|~τα~ωμαι", _, "~τα~ησωμεθᾰ|~τα~ωμεθᾰ";
    aorist_subj_mid_2: "~τα~ησῃ|~τα~ῃ", "~τα~ησησθον|~τα~ησθον", "~τα~ησησθε|~τα~ησθε";
    aorist_subj_mid_3: "~τα~ησηται|~τα~ηται", "~τα~ησησθον|~τα~ησθον", "~τα~ησωνται|~τα~ωνται";
    aorist_subj_pass_1: "~τα~ᾰθω", _, "~τα~ᾰθωμεν";
    aorist_subj_pass_2: "~τα~ᾰθῃς", "~τα~ᾰθητον", "~τα~ᾰθητε";
    aorist_subj_pass_3: "~τα~ᾰθῃ", _, "~τα~ᾰθωσῐ|~τα~ᾰθωσῐν";
    aorist_opt_act_1: "~τα~ησαιμῐ|~τα~αιην", _, "~τα~ησαιμεν|~τα~αῑμεν|~τα~αιημεν";
    aorist_opt_act_2: "~τα~ησειᾰς|~τα~ησαις|~τα~αιης", "~τα~ησαιτον|~τα~αῑτον|~τα~αιητον", "~τα~ησαιτε|~τα~αῑτε|~τα~αιητε";
    aorist_opt_act_3: "~τα~ησειε|~τα~ησειεν|~τα~ησαι|~τα~αιη", "~τα~ησαιτην|~τα~αιτην|~τα~αιητην", "~τα~ησειᾰν|~τα~ησαιεν|~τα~αῑεν|~τα~αιησᾰν";
    aorist_opt_mid_1: "~τα~ησαιμην|~τα~αιμην", _, "~τα~ησαιμεθᾰ|~τα~αιμεθᾰ";
    aorist_opt_mid_2: "~τα~ησαιο|~τα~αῑο", "~τα~ησαισθον|~τα~αῑσθον", "~τα~ησαισθε|~τα~αῑσθε";
    aorist_opt_mid_3: "~τα~ησαιτο|~τα~αῑτο", "~τα~ησαισθην|~τα~αισθην", "~τα~ησαιντο|~τα~αῑντο";
    aorist_opt_pass_1: "~τα~ᾰθειην", _, "~τα~ᾰθεῑμεν|~τα~ᾰθειημεν";
    aorist_opt_pass_2: "~τα~ᾰθειης", "~τα~ᾰθεῑτον|~τα~ᾰθειητον", "~τα~ᾰθεῑτε|~τα~ᾰθειητε";
    aorist_opt_pass_3: "~τα~ᾰθειη", "~τα~ᾰθειτην|~τα~ᾰθειητην", "~τα~ᾰθεῑεν|~τα~ᾰθειησᾰν";
    aorist_imp_act_2: "~τα~ησον|~τα~ηθῐ", "~τα~ησᾰτον|~τα~ητον", "~τα~ησᾰτε|~τα~ητε";
    aorist_imp_act_3: "~τα~ησᾰτω|~τα~ητω", "~τα~ησᾰτων|~τα~ητων", "~τα~ησᾰντων|~τα~ᾰντων";
    aorist_imp_mid_2: "~τα~ησαι|~τα~ω", "~τα~ησᾰσθον|~τα~ᾰσθον", "~τα~ησᾰσθε|~τα~ᾰσθε";
    aorist_imp_mid_3: "~τα~ησᾰσθω|~τα~ᾰσθω", "~τα~ησᾰσθων|~τα~ᾰσθων", "~τα~ησᾰσθων|~τα~ᾰσθων";
    aorist_imp_pass_2: "~τα~ᾰθητῐ", _, _;
    aorist_imp_pass_3: "~τα~ᾰθητω", "~τα~ᾰθητων", "~τα~ᾰθεντων";
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
    pluperfect_act_1: "~θε~ηκειν|~θε~ηκη", _, "~θε~ηκεμεν";
    pluperfect_act_2: "~θε~ηκεις|~θε~ηκης", "~θε~ηκετον", "~θε~ηκετε";
    pluperfect_act_3: "~θε~ηκει|~θε~ηκειν", "~θε~ηκετην", "~θε~ηκεσᾰν";
    infinitive_fut: "~τα~ησειν", "~τα~ησεσθαι", "~τα~ησεσθαι";
    infinitive_aor: "~τα~ησαι|~τα~ηναι", "~τα~ησᾰσθαι|~τα~ᾰσθαι", "~τα~ᾰθηναι";
    infinitive_perf: "~θε~ηκεναι", _, _;
    participle_fut: "~τα~ησων", "~τα~ησομενος", "~τα~ησομενος";
    participle_aor: "~τα~ησᾱς|~τα~ᾱς", "~τα~ησᾰμενος|~τα~ᾰμενος", "~τα~ᾰθεις";
    participle_perf: "~θε~ηκως", _, _;
    ),
    // Διαζωννῡμῐ: 1 words (διαζωννῡμῐ). Like Λιστραω, except in 263 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("διαζωννῡμῐ", 1, "~α~ννῡμῐ", like Λιστραω;
    ind_act_1: "~α~ννῡμῐ", _, "~α~ννῠμεν";
    ind_act_2: "~α~ννῡς", _, _;
    ind_act_3: "~α~ννῡσῐ|~α~ννῡσῐν", _, "~α~ννῠᾱσῐ|~α~ννῠᾱσῐν";
    ind_mid_1: "~α~ννῠμαι", _, "~α~ννῠμεθᾰ";
    ind_mid_2: "~α~ννῠσαι", "~α~ννῠσθον", "~α~ννῠσθε";
    ind_mid_3: "~α~ννῠται", _, "~α~ννῠνται";
    ind_pass_1: _, _, "~α~ννῠμεθᾰ";
    ind_pass_2: _, _, "~α~ννῠσθε";
    ind_pass_3: "~α~ννῠται", _, "~α~ννῠνται";
    subj_act_1: "~α~ννῠω", _, "~α~ννῠωμεν";
    subj_act_2: "~α~ννῠῃς", "~α~ννῠητον", "~α~ννῠητε";
    subj_act_3: "~α~ννῠῃ", _, "~α~ννῠωσῐ|~α~ννῠωσῐν";
    subj_mid_1: "~α~ννῠωμαι", _, "~α~ννῠωμεθᾰ";
    subj_mid_2: "~α~ννῠῃ", "~α~ννῠησθον", "~α~ννῠησθε";
    subj_mid_3: "~α~ννῠηται", _, "~α~ννῠωνται";
    subj_pass_1: _, _, "~α~ννῠωμεθᾰ";
    subj_pass_2: _, _, "~α~ννῠησθε";
    subj_pass_3: "~α~ννῠηται", _, "~α~ννῠωνται";
    opt_act_1: "~α~ννῠοιμῐ", _, "~α~ννῠοιμεν";
    opt_act_2: "~α~ννῠοις", "~α~ννῠοιτον", "~α~ννῠοιτε";
    opt_act_3: "~α~ννῠοι", "~α~ννῠοιτην", "~α~ννῠοιεν";
    opt_mid_1: "~α~ννῠοιμην", _, "~α~ννῠοιμεθᾰ";
    opt_mid_2: "~α~ννῠοιο", "~α~ννῠοισθον", "~α~ννῠοισθε";
    opt_mid_3: "~α~ννῠοιτο", "~α~ννῠοισθην", "~α~ννῠοιντο";
    opt_pass_1: _, _, "~α~ννῠοιμεθᾰ";
    opt_pass_2: _, "~α~ννῠοισθον", "~α~ννῠοισθε";
    opt_pass_3: "~α~ννῠοιτο", "~α~ννῠοισθην", "~α~ννῠοιντο";
    imp_act_2: "~α~ννῡ", "~α~ννῠτον", "~α~ννῠτε";
    imp_act_3: "~α~ννῠτω", "~α~ννῠτων", "~α~ννῠντων";
    imp_mid_2: "~α~ννῠσο", _, "~α~ννῠσθε";
    imp_mid_3: "~α~ννῠσθω", "~α~ννῠσθων", "~α~ννῠσθων";
    imp_pass_2: _, _, "~α~ννῠσθε";
    imp_pass_3: _, "~α~ννῠσθων", _;
    imperfect_act_1: "~ε~ννῡν", _, "~ε~ννῠμεν";
    imperfect_act_2: "~ε~ννῡς", "~ε~ννῠτον", "~ε~ννῠτε";
    imperfect_act_3: "~ε~ννῡ", "~ε~ννῠτην", "~ε~ννῠσᾰν";
    imperfect_mid_1: "~ε~ννῠμην", _, "~ε~ννῠμεθᾰ";
    imperfect_mid_2: "~ε~ννῠσο", "~ε~ννῠσθον", "~ε~ννῠσθε";
    imperfect_mid_3: "~ε~ννῠτο", "~ε~ννῠσθην", "~ε~ννῠντο";
    imperfect_pass_1: _, _, "~ε~ννῠμεθᾰ";
    imperfect_pass_2: _, _, "~ε~ννῠσθε";
    imperfect_pass_3: "~ε~ννῠτο", _, _;
    future_ind_act_1: "~α~σω", _, "~α~σομεν";
    future_ind_act_2: "~α~σεις", "~α~σετον", "~α~σετε";
    future_ind_act_3: "~α~σει", _, "~α~σουσῐ|~α~σουσῐν";
    future_ind_mid_1: "~α~σομαι", _, "~α~σομεθᾰ";
    future_ind_mid_2: "~α~σῃ|~α~σει", "~α~σεσθον", "~α~σεσθε";
    future_ind_mid_3: "~α~σεται", _, "~α~σονται";
    future_opt_act_1: "~α~σοιμῐ", _, "~α~σοιμεν";
    future_opt_act_2: "~α~σοις", "~α~σοιτον", "~α~σοιτε";
    future_opt_act_3: "~α~σοι", "~α~σοιτην", "~α~σοιεν";
    future_opt_mid_1: "~α~σοιμην", _, "~α~σοιμεθᾰ";
    future_opt_mid_2: "~α~σοιο", "~α~σοισθον", "~α~σοισθε";
    future_opt_mid_3: "~α~σοιτο", "~α~σοισθην", "~α~σοιντο";
    aorist_ind_act_1: "~α~σᾰ", _, "~α~σᾰμεν";
    aorist_ind_act_2: "~α~σᾰς", "~α~σᾰτον", "~α~σᾰτε";
    aorist_ind_act_3: "~α~σε|~α~σεν", "~α~σᾰτην", "~α~σᾰν";
    aorist_ind_mid_1: "~α~σᾰμην", _, "~α~σᾰμεθᾰ";
    aorist_ind_mid_2: "~α~σω", "~α~σᾰσθον", "~α~σᾰσθε";
    aorist_ind_mid_3: "~α~σᾰτο", "~α~σᾰσθην", "~α~σᾰντο";
    aorist_ind_pass_1: "~α~σθην", _, "~α~σθημεν";
    aorist_ind_pass_2: "~α~σθης", "~α~σθητον", "~α~σθητε";
    aorist_ind_pass_3: "~α~σθη", "~α~σθητην", "~α~σθησᾰν";
    aorist_subj_act_1: "~α~σω", _, "~α~σωμεν";
    aorist_subj_act_2: "~α~σῃς", "~α~σητον", "~α~σητε";
    aorist_subj_act_3: "~α~σῃ", _, "~α~σωσῐ|~α~σωσῐν";
    aorist_subj_mid_1: "~α~σωμαι", _, "~α~σωμεθᾰ";
    aorist_subj_mid_2: "~α~σῃ", "~α~σησθον", "~α~σησθε";
    aorist_subj_mid_3: "~α~σηται", _, "~α~σωνται";
    aorist_subj_pass_1: "~α~σθω", _, "~α~σθωμεν";
    aorist_subj_pass_2: "~α~σθῃς", "~α~σθητον", "~α~σθητε";
    aorist_subj_pass_3: "~α~σθῃ", _, "~α~σθωσῐ|~α~σθωσῐν";
    aorist_opt_act_1: "~α~σαιμῐ", _, "~α~σαιμεν";
    aorist_opt_act_2: "~α~σειᾰς|~α~σαις", "~α~σαιτον", "~α~σαιτε";
    aorist_opt_act_3: "~α~σειε|~α~σειεν|~α~σαι", "~α~σαιτην", "~α~σειᾰν|~α~σαιεν";
    aorist_opt_mid_1: "~α~σαιμην", _, "~α~σαιμεθᾰ";
    aorist_opt_mid_2: "~α~σαιο", "~α~σαισθον", "~α~σαισθε";
    aorist_opt_mid_3: "~α~σαιτο", "~α~σαισθην", "~α~σαιντο";
    aorist_opt_pass_1: "~α~σθειην", _, "~α~σθεῑμεν|~α~σθειημεν";
    aorist_opt_pass_2: "~α~σθειης", "~α~σθεῑτον|~α~σθειητον", "~α~σθεῑτε|~α~σθειητε";
    aorist_opt_pass_3: "~α~σθειη", "~α~σθειτην|~α~σθειητην", "~α~σθεῑεν|~α~σθειησᾰν";
    aorist_imp_act_2: "~α~σον", "~α~σᾰτον", "~α~σᾰτε";
    aorist_imp_act_3: "~α~σᾰτω", "~α~σᾰτων", "~α~σᾰντων";
    aorist_imp_mid_2: "~α~σαι", "~α~σᾰσθον", "~α~σᾰσθε";
    aorist_imp_mid_3: "~α~σᾰσθω", "~α~σᾰσθων", _;
    aorist_imp_pass_2: "~α~σθητῐ", _, _;
    aorist_imp_pass_3: "~α~σθητω", "~α~σθητων", "~α~σθεντων";
    perfect_ind_act_1: "~ε~κᾰ", _, "~ε~κᾰμεν";
    perfect_ind_act_2: "~ε~κᾰς", "~ε~κᾰτον", "~ε~κᾰτε";
    perfect_ind_act_3: "~ε~κε|~ε~κεν", _, "~ε~κᾱσῐ|~ε~κᾱσῐν";
    perfect_ind_mid_1: "~ε~μαι", _, "~ε~μεθᾰ";
    perfect_ind_mid_2: "~ε~σαι", "~ε~σθον", "~ε~σθε";
    perfect_ind_mid_3: "~ε~ται", _, "~ε~νται";
    perfect_ind_pass_1: _, _, "~ε~μεθᾰ";
    perfect_ind_pass_2: _, _, "~ε~σθε";
    perfect_subj_act_1: "~ε~κω", _, "~ε~κωμεν";
    perfect_subj_act_2: "~ε~κῃς", "~ε~κητον", "~ε~κητε";
    perfect_subj_act_3: "~ε~κῃ", _, "~ε~κωσῐ|~ε~κωσῐν";
    perfect_opt_act_1: "~ε~κοιμῐ|~ε~κοιην", _, "~ε~κοιμεν";
    perfect_opt_act_2: "~ε~κοις|~ε~κοιης", "~ε~κοιτον", "~ε~κοιτε";
    perfect_opt_act_3: "~ε~κοι|~ε~κοιη", "~ε~κοιτην", "~ε~κοιεν";
    perfect_imp_act_2: "~ε~κε", "~ε~κετον", "~ε~κετε";
    perfect_imp_act_3: "~ε~κετω", "~ε~κετων", "~ε~κοντων";
    perfect_imp_mid_2: "~ε~σο", _, _;
    perfect_imp_mid_3: "~ε~σθω", "~ε~σθων", _;
    perfect_imp_pass_2: "~ε~σο", _, _;
    perfect_imp_pass_3: "~ε~σθω", "~ε~σθων", _;
    infinitive_pres: "~α~ννῠναι", "~α~ννῠσθαι", _;
    infinitive_fut: "~α~σειν", "~α~σεσθαι", _;
    infinitive_aor: "~α~σαι", "~α~σᾰσθαι", "~α~σθηναι";
    infinitive_perf: "~ε~κεναι", "~ε~σθαι", _;
    participle_pres: "~α~ννῡς", "~α~ννῠμενος", _;
    participle_fut: "~α~σων", "~α~σομενος", _;
    participle_aor: "~α~σᾱς", "~α~σᾰμενος", "~α~σθεις";
    participle_perf: "~ε~κως", "~ε~μενος", _;
    ),
    // Σκεδᾰννῡμῐ: 1 words (σκεδᾰννῡμῐ). Like Κρεμᾰννῡμῐ, except in 173 cells: ind_pass_1 sg, ind_pass_1 pl, ind_pass_2 sg, ind_pass_2 du, ind_pass_2 pl, ind_pass_3 sg, ….
    conjugation!("σκεδᾰννῡμῐ", 1, "~ᾰννῡμῐ", like Κρεμᾰννῡμῐ;
    ind_pass_1: derived, _, "~ᾰννῠμεθᾰ";
    ind_pass_2: derived, derived, "~ᾰννῠσθε";
    ind_pass_3: "~ᾰννῠται", derived, "~ᾰννῠνται";
    subj_pass_1: derived, _, "~ᾰννῠωμεθᾰ";
    subj_pass_2: derived, derived, "~ᾰννῠησθε";
    subj_pass_3: "~ᾰννῠηται", derived, "~ᾰννῠωνται";
    opt_pass_1: derived, _, "~ᾰννῠοιμεθᾰ";
    opt_pass_2: derived, "~ᾰννῠοισθον", "~ᾰννῠοισθε";
    opt_pass_3: "~ᾰννῠοιτο", "~ᾰννῠοισθην", "~ᾰννῠοιντο";
    imp_pass_2: derived, derived, "~ᾰννῠσθε";
    imp_pass_3: derived, "~ᾰννῠσθων", derived;
    imperfect_act_1: "~ᾰννῡν", _, "~ᾰννῠμεν";
    imperfect_act_2: "~ᾰννῡς", "~ᾰννῠτον", "~ᾰννῠτε";
    imperfect_act_3: "~ᾰννῡ", "~ᾰννῠτην", "~ᾰννῠσᾰν";
    imperfect_mid_1: "~ᾰννῠμην", _, "~ᾰννῠμεθᾰ";
    imperfect_mid_2: "~ᾰννῠσο", "~ᾰννῠσθον", "~ᾰννῠσθε";
    imperfect_mid_3: "~ᾰννῠτο", "~ᾰννῠσθην", "~ᾰννῠντο";
    imperfect_pass_1: derived, _, "~ᾰννῠμεθᾰ";
    imperfect_pass_2: derived, derived, "~ᾰννῠσθε";
    imperfect_pass_3: "~ᾰννῠτο", derived, derived;
    future_ind_act_1: "~ω", _, "~ωμεν";
    future_ind_act_2: "~ᾳς", "~ᾱτον", "~ᾱτε";
    future_ind_act_3: "~ᾳ", derived, "~ωσῐ|~ωσῐν";
    future_ind_mid_1: "~ωμαι", _, "~ωμεθᾰ";
    future_ind_mid_2: "~ᾳ", "~ᾱσθον", "~ᾱσθε";
    future_ind_mid_3: "~ᾱται", derived, "~ωνται";
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_act_1: "~ῳην|~ῳμῐ", _, "~ῳμεν|~ῳημεν";
    future_opt_act_2: "~ῳης|~ῳς", "~ῳτον|~ῳητον", "~ῳτε|~ῳητε";
    future_opt_act_3: "~ῳη|~ῳ", "~ῳτην|~ῳητην", "~ῳεν|~ῳησᾰν";
    future_opt_mid_1: "~ῳμην", _, "~ῳμεθᾰ";
    future_opt_mid_2: "~ῳο", "~ῳσθον", "~ῳσθε";
    future_opt_mid_3: "~ῳτο", "~ῳσθην", "~ῳντο";
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: derived, derived, derived;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_mid_1: "ἐ~ᾰσμαι", _, "ἐ~ᾰσμεθᾰ";
    perfect_ind_mid_2: "ἐ~ᾰσαι", "ἐ~ᾰσθον", "ἐ~ᾰσθε";
    perfect_ind_mid_3: "ἐ~ᾰσται", _, "ἐ~ᾰσᾰται";
    perfect_ind_pass_1: _, _, "ἐ~ᾰσμεθᾰ";
    perfect_ind_pass_2: _, _, "ἐ~ᾰσθε";
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, derived;
    perfect_imp_act_3: derived, derived, derived;
    perfect_imp_mid_2: "ἐ~ᾰσο", _, _;
    perfect_imp_mid_3: "ἐ~ᾰσθω", "ἐ~ᾰσθων", _;
    perfect_imp_pass_2: "ἐ~ᾰσο", _, _;
    perfect_imp_pass_3: "ἐ~ᾰσθω", "ἐ~ᾰσθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
    infinitive_fut: "~ᾱν", "~ᾱσθαι", derived;
    infinitive_perf: derived, "ἐ~ᾰσθαι", _;
    participle_fut: "~ων", "~ωμενος", derived;
    participle_perf: derived, "ἐ~ᾰσμενος", _;
    ),
    // Κορεννῡμῐ: 1 words (κορεννῡμῐ). Like Ἑπομαι, except in 235 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("κορεννῡμῐ", 1, "~εννῡμῐ", like Ἑπομαι;
    ind_act_1: "~εννῡμῐ|~εννῠω", _, "~εννῠμεν|~εννῠομεν";
    ind_act_2: "~εννῡς|~εννῠεις", "~εννῠτον|~εννῠετον", "~εννῠτε|~εννῠετε";
    ind_act_3: "~εννῡσῐ|~εννῡσῐν|~εννῠει", "~εννῠτον|~εννῠετον", "~εννῠᾱσῐ|~εννῠᾱσῐν|~εννῠουσῐ|~εννῠουσῐν";
    ind_mid_1: "~εννῠμαι|~εννῠομαι", _, "~εννῠμεθᾰ|~εννῠομεθᾰ";
    ind_mid_2: "~εννῠσαι|~εννῠῃ|~εννῠει", "~εννῠσθον|~εννῠεσθον", "~εννῠσθε|~εννῠεσθε";
    ind_mid_3: "~εννῠται|~εννῠεται", "~εννῠσθον|~εννῠεσθον", "~εννῠνται|~εννῠονται";
    ind_pass_1: "~εννῠμαι|~εννῠομαι", _, "~εννῠμεθᾰ|~εννῠομεθᾰ";
    ind_pass_2: "~εννῠσαι|~εννῠῃ|~εννῠει", "~εννῠσθον|~εννῠεσθον", "~εννῠσθε|~εννῠεσθε";
    ind_pass_3: "~εννῠται|~εννῠεται", "~εννῠσθον|~εννῠεσθον", "~εννῠνται|~εννῠονται";
    subj_act_1: "~εννῠω", _, "~εννῠωμεν";
    subj_act_2: "~εννῠῃς", "~εννῠητον", "~εννῠητε";
    subj_act_3: "~εννῠῃ", _, "~εννῠωσῐ|~εννῠωσῐν";
    subj_mid_1: "~εννῠωμαι", _, "~εννῠωμεθᾰ";
    subj_mid_2: "~εννῠῃ", "~εννῠησθον", "~εννῠησθε";
    subj_mid_3: "~εννῠηται", _, "~εννῠωνται";
    subj_pass_1: _, _, "~εννῠωμεθᾰ";
    subj_pass_2: _, _, "~εννῠησθε";
    subj_pass_3: "~εννῠηται", _, "~εννῠωνται";
    opt_act_1: "~εννῠοιμῐ", _, "~εννῠοιμεν";
    opt_act_2: "~εννῠοις", "~εννῠοιτον", "~εννῠοιτε";
    opt_act_3: "~εννῠοι", "~εννῠοιτην", "~εννῠοιεν";
    opt_mid_1: "~εννῠοιμην", _, "~εννῠοιμεθᾰ";
    opt_mid_2: "~εννῠοιο", "~εννῠοισθον", "~εννῠοισθε";
    opt_mid_3: "~εννῠοιτο", "~εννῠοισθην", "~εννῠοιντο";
    opt_pass_1: _, _, "~εννῠοιμεθᾰ";
    opt_pass_2: _, "~εννῠοισθον", "~εννῠοισθε";
    opt_pass_3: "~εννῠοιτο", "~εννῠοισθην", "~εννῠοιντο";
    imp_act_2: "~εννῡ|~εννῠε", "~εννῠτον|~εννῠετον", "~εννῠτε|~εννῠετε";
    imp_act_3: "~εννῠτω|~εννῠετω", "~εννῠτων|~εννῠετων", "~εννῠντων|~εννῠοντων";
    imp_mid_2: "~εννῠσο|~εννῠου", "~εννῠσθον|~εννῠεσθον", "~εννῠσθε|~εννῠεσθε";
    imp_mid_3: "~εννῠσθω|~εννῠεσθω", "~εννῠσθων|~εννῠεσθων", "~εννῠσθων|~εννῠεσθων";
    imp_pass_2: "~εννῠσο|~εννῠου", "~εννῠσθον|~εννῠεσθον", "~εννῠσθε|~εννῠεσθε";
    imp_pass_3: "~εννῠσθω|~εννῠεσθω", "~εννῠσθων|~εννῠεσθων", "~εννῠσθων|~εννῠεσθων";
    imperfect_act_1: "ἐ~εννῡν", _, "ἐ~εννῠμεν";
    imperfect_act_2: "ἐ~εννῡς", "ἐ~εννῠτον", "ἐ~εννῠτε";
    imperfect_act_3: "ἐ~εννῡ", "ἐ~εννῠτην", "ἐ~εννῠσᾰν";
    imperfect_mid_1: "ἐ~εννῠμην", _, "ἐ~εννῠμεθᾰ";
    imperfect_mid_2: "ἐ~εννῠσο", "ἐ~εννῠσθον", "ἐ~εννῠσθε";
    imperfect_mid_3: "ἐ~εννῠτο", "ἐ~εννῠσθην", "ἐ~εννῠντο";
    imperfect_pass_1: _, _, "ἐ~εννῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~εννῠσθε";
    imperfect_pass_3: "ἐ~εννῠτο", _, _;
    future_ind_act_1: "~εσω|~εσσω", _, "~εσομεν|~εσσομεν";
    future_ind_act_2: "~εσεις|~εσσεις", "~εσετον|~εσσετον", "~εσετε|~εσσετε";
    future_ind_act_3: "~εσει|~εσσει", "~εσετον|~εσσετον", "~εσουσῐ|~εσουσῐν|~εσσουσῐ|~εσσουσῐν";
    future_ind_mid_1: "~εσομαι|~εσσομαι", _, "~εσομεθᾰ|~εσσομεθᾰ";
    future_ind_mid_2: "~εσῃ|~εσει|~εσσῃ|~εσσει", "~εσεσθον|~εσσεσθον", "~εσεσθε|~εσσεσθε";
    future_ind_mid_3: "~εσεται|~εσσεται", "~εσεσθον|~εσσεσθον", "~εσονται|~εσσονται";
    future_opt_act_1: "~εσοιμῐ|~εσσοιμῐ", _, "~εσοιμεν|~εσσοιμεν";
    future_opt_act_2: "~εσοις|~εσσοις", "~εσοιτον|~εσσοιτον", "~εσοιτε|~εσσοιτε";
    future_opt_act_3: "~εσοι|~εσσοι", "~εσοιτην|~εσσοιτην", "~εσοιεν|~εσσοιεν";
    future_opt_mid_1: "~εσοιμην|~εσσοιμην", _, "~εσοιμεθᾰ|~εσσοιμεθᾰ";
    future_opt_mid_2: "~εσοιο|~εσσοιο", "~εσοισθον|~εσσοισθον", "~εσοισθε|~εσσοισθε";
    future_opt_mid_3: "~εσοιτο|~εσσοιτο", "~εσοισθην|~εσσοισθην", "~εσοιντο|~εσσοιντο";
    aorist_ind_act_1: "ἐ~εσᾰ|ἐ~εσσᾰ", _, "ἐ~εσᾰμεν|ἐ~εσσᾰμεν";
    aorist_ind_act_2: "ἐ~εσᾰς|ἐ~εσσᾰς", "ἐ~εσᾰτον|ἐ~εσσᾰτον", "ἐ~εσᾰτε|ἐ~εσσᾰτε";
    aorist_ind_act_3: "ἐ~εσε|ἐ~εσεν|ἐ~εσσε|ἐ~εσσεν", "ἐ~εσᾰτην|ἐ~εσσᾰτην", "ἐ~εσᾰν|ἐ~εσσᾰν";
    aorist_ind_mid_1: "ἐ~εσᾰμην|ἐ~εσσᾰμην", _, "ἐ~εσᾰμεθᾰ|ἐ~εσσᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~εσω|ἐ~εσσω", "ἐ~εσᾰσθον|ἐ~εσσᾰσθον", "ἐ~εσᾰσθε|ἐ~εσσᾰσθε";
    aorist_ind_mid_3: "ἐ~εσᾰτο|ἐ~εσσᾰτο", "ἐ~εσᾰσθην|ἐ~εσσᾰσθην", "ἐ~εσᾰντο|ἐ~εσσᾰντο";
    aorist_subj_act_1: "~εσω|~εσσω", _, "~εσωμεν|~εσσωμεν";
    aorist_subj_act_2: "~εσῃς|~εσσῃς", "~εσητον|~εσσητον", "~εσητε|~εσσητε";
    aorist_subj_act_3: "~εσῃ|~εσσῃ", "~εσητον|~εσσητον", "~εσωσῐ|~εσωσῐν|~εσσωσῐ|~εσσωσῐν";
    aorist_subj_mid_1: "~εσωμαι|~εσσωμαι", _, "~εσωμεθᾰ|~εσσωμεθᾰ";
    aorist_subj_mid_2: "~εσῃ|~εσσῃ", "~εσησθον|~εσσησθον", "~εσησθε|~εσσησθε";
    aorist_subj_mid_3: "~εσηται|~εσσηται", "~εσησθον|~εσσησθον", "~εσωνται|~εσσωνται";
    aorist_opt_act_1: "~εσαιμῐ|~εσσαιμῐ", _, "~εσαιμεν|~εσσαιμεν";
    aorist_opt_act_2: "~εσειᾰς|~εσαις|~εσσειᾰς|~εσσαις", "~εσαιτον|~εσσαιτον", "~εσαιτε|~εσσαιτε";
    aorist_opt_act_3: "~εσειε|~εσειεν|~εσαι|~εσσειε|~εσσειεν|~εσσαι", "~εσαιτην|~εσσαιτην", "~εσειᾰν|~εσαιεν|~εσσειᾰν|~εσσαιεν";
    aorist_opt_mid_1: "~εσαιμην|~εσσαιμην", _, "~εσαιμεθᾰ|~εσσαιμεθᾰ";
    aorist_opt_mid_2: "~εσαιο|~εσσαιο", "~εσαισθον|~εσσαισθον", "~εσαισθε|~εσσαισθε";
    aorist_opt_mid_3: "~εσαιτο|~εσσαιτο", "~εσαισθην|~εσσαισθην", "~εσαιντο|~εσσαιντο";
    aorist_imp_act_2: "~εσον|~εσσον", "~εσᾰτον|~εσσᾰτον", "~εσᾰτε|~εσσᾰτε";
    aorist_imp_act_3: "~εσᾰτω|~εσσᾰτω", "~εσᾰτων|~εσσᾰτων", "~εσᾰντων|~εσσᾰντων";
    aorist_imp_mid_2: "~εσαι|~εσσαι", "~εσᾰσθον|~εσσᾰσθον", "~εσᾰσθε|~εσσᾰσθε";
    aorist_imp_mid_3: "~εσᾰσθω|~εσσᾰσθω", "~εσᾰσθων|~εσσᾰσθων", "~εσᾰσθων|~εσσᾰσθων";
    perfect_ind_mid_1: "%ε~ημαι", _, "%ε~ημεθᾰ";
    perfect_ind_mid_2: "%ε~ησαι", "%ε~ησθον", "%ε~ησθε";
    perfect_ind_mid_3: "%ε~ηται", _, "%ε~ηνται";
    perfect_ind_pass_1: _, _, "%ε~ημεθᾰ";
    perfect_ind_pass_2: _, _, "%ε~ησθε";
    perfect_imp_mid_2: "%ε~ησο", _, _;
    perfect_imp_mid_3: "%ε~ησθω", "%ε~ησθων", _;
    perfect_imp_pass_2: "%ε~ησο", _, _;
    perfect_imp_pass_3: "%ε~ησθω", "%ε~ησθων", _;
    pluperfect_mid_1: "ἐ%ε~ημην", _, "ἐ%ε~ημεθᾰ";
    pluperfect_mid_2: "ἐ%ε~ησο", "ἐ%ε~ησθον", "ἐ%ε~ησθε";
    pluperfect_mid_3: "ἐ%ε~ητο", "ἐ%ε~ησθην", _;
    pluperfect_pass_1: _, _, "ἐ%ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "ἐ%ε~ησθε";
    pluperfect_pass_3: _, "ἐ%ε~ησθην", "ἐ%ε~ηντο";
    infinitive_pres: "~εννῠναι|~εννῠειν", "~εννῠσθαι|~εννῠεσθαι", "~εννῠσθαι|~εννῠεσθαι";
    infinitive_fut: "~εσειν|~εσσειν", "~εσεσθαι|~εσσεσθαι", _;
    infinitive_aor: "~εσαι|~εσσαι", "~εσᾰσθαι|~εσσᾰσθαι", _;
    infinitive_perf: _, "%ε~ησθαι", _;
    participle_pres: "~εννῡς|~εννῠων", "~εννῠμενος|~εννῠομενος", "~εννῠμενος|~εννῠομενος";
    participle_fut: "~εσων|~εσσων", "~εσομενος|~εσσομενος", _;
    participle_aor: "~εσᾱς|~εσσᾱς", "~εσᾰμενος|~εσσᾰμενος", _;
    participle_perf: _, "%ε~ημενος", _;
    ),
    // Ὀμοργνῡμῐ: 1 words (ὀμοργνῡμῐ). Like Γῡροω, except in 299 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ὀμοργνῡμῐ", 1, "ὀ~γνῡμῐ", like Γῡροω;
    ind_act_1: "ὀ~γνῡμῐ", _, "ὀ~γνῠμεν";
    ind_act_2: "ὀ~γνῡς", _, _;
    ind_act_3: "ὀ~γνῡσῐ|ὀ~γνῡσῐν", _, "ὀ~γνῠᾱσῐ|ὀ~γνῠᾱσῐν";
    ind_mid_1: "ὀ~γνῠμαι", _, "ὀ~γνῠμεθᾰ";
    ind_mid_2: "ὀ~γνῠσαι", "ὀ~γνῠσθον", "ὀ~γνῠσθε";
    ind_mid_3: "ὀ~γνῠται", _, "ὀ~γνῠνται";
    ind_pass_1: _, _, "ὀ~γνῠμεθᾰ";
    ind_pass_2: _, _, "ὀ~γνῠσθε";
    ind_pass_3: "ὀ~γνῠται", _, "ὀ~γνῠνται";
    subj_act_1: "ὀ~γνῠω", _, "ὀ~γνῠωμεν";
    subj_act_2: "ὀ~γνῠῃς", "ὀ~γνῠητον", "ὀ~γνῠητε";
    subj_act_3: "ὀ~γνῠῃ", _, "ὀ~γνῠωσῐ|ὀ~γνῠωσῐν";
    subj_mid_1: "ὀ~γνῠωμαι", _, "ὀ~γνῠωμεθᾰ";
    subj_mid_2: "ὀ~γνῠῃ", "ὀ~γνῠησθον", "ὀ~γνῠησθε";
    subj_mid_3: "ὀ~γνῠηται", _, "ὀ~γνῠωνται";
    subj_pass_1: _, _, "ὀ~γνῠωμεθᾰ";
    subj_pass_2: _, _, "ὀ~γνῠησθε";
    subj_pass_3: "ὀ~γνῠηται", _, "ὀ~γνῠωνται";
    opt_act_1: "ὀ~γνῠοιμῐ", _, "ὀ~γνῠοιμεν";
    opt_act_2: "ὀ~γνῠοις", "ὀ~γνῠοιτον", "ὀ~γνῠοιτε";
    opt_act_3: "ὀ~γνῠοι", "ὀ~γνῠοιτην", "ὀ~γνῠοιεν";
    opt_mid_1: "ὀ~γνῠοιμην", _, "ὀ~γνῠοιμεθᾰ";
    opt_mid_2: "ὀ~γνῠοιο", "ὀ~γνῠοισθον", "ὀ~γνῠοισθε";
    opt_mid_3: "ὀ~γνῠοιτο", "ὀ~γνῠοισθην", "ὀ~γνῠοιντο";
    opt_pass_1: _, _, "ὀ~γνῠοιμεθᾰ";
    opt_pass_2: _, "ὀ~γνῠοισθον", "ὀ~γνῠοισθε";
    opt_pass_3: "ὀ~γνῠοιτο", "ὀ~γνῠοισθην", "ὀ~γνῠοιντο";
    imp_act_2: "ὀ~γνῡ", "ὀ~γνῠτον", "ὀ~γνῠτε";
    imp_act_3: "ὀ~γνῠτω", "ὀ~γνῠτων", "ὀ~γνῠντων";
    imp_mid_2: "ὀ~γνῠσο", _, "ὀ~γνῠσθε";
    imp_mid_3: "ὀ~γνῠσθω", "ὀ~γνῠσθων", "ὀ~γνῠσθων";
    imp_pass_2: _, _, "ὀ~γνῠσθε";
    imp_pass_3: _, "ὀ~γνῠσθων", _;
    imperfect_act_1: "ὠ~γνῡν", _, "ὠ~γνῠμεν";
    imperfect_act_2: "ὠ~γνῡς", "ὠ~γνῠτον", "ὠ~γνῠτε";
    imperfect_act_3: "ὠ~γνῡ", "ὠ~γνῠτην", "ὠ~γνῠσᾰν";
    imperfect_mid_1: "ὠ~γνῠμην", _, "ὠ~γνῠμεθᾰ";
    imperfect_mid_2: "ὠ~γνῠσο", "ὠ~γνῠσθον", "ὠ~γνῠσθε";
    imperfect_mid_3: "ὠ~γνῠτο", "ὠ~γνῠσθην", "ὠ~γνῠντο";
    imperfect_pass_1: _, _, "ὠ~γνῠμεθᾰ";
    imperfect_pass_2: _, _, "ὠ~γνῠσθε";
    imperfect_pass_3: "ὠ~γνῠτο", _, _;
    future_ind_act_1: "ὀ~ξω", _, "ὀ~ξομεν";
    future_ind_act_2: "ὀ~ξεις", "ὀ~ξετον", "ὀ~ξετε";
    future_ind_act_3: "ὀ~ξει", _, "ὀ~ξουσῐ|ὀ~ξουσῐν";
    future_ind_mid_1: "ὀ~ξομαι", _, "ὀ~ξομεθᾰ";
    future_ind_mid_2: "ὀ~ξῃ|ὀ~ξει", "ὀ~ξεσθον", "ὀ~ξεσθε";
    future_ind_mid_3: "ὀ~ξεται", _, "ὀ~ξονται";
    future_ind_pass_1: "ὀ~χθησομαι", _, "ὀ~χθησομεθᾰ";
    future_ind_pass_2: "ὀ~χθησῃ|ὀ~χθησει", "ὀ~χθησεσθον", "ὀ~χθησεσθε";
    future_ind_pass_3: "ὀ~χθησεται", _, "ὀ~χθησονται";
    future_opt_act_1: "ὀ~ξοιμῐ", _, "ὀ~ξοιμεν";
    future_opt_act_2: "ὀ~ξοις", "ὀ~ξοιτον", "ὀ~ξοιτε";
    future_opt_act_3: "ὀ~ξοι", "ὀ~ξοιτην", "ὀ~ξοιεν";
    future_opt_mid_1: "ὀ~ξοιμην", _, "ὀ~ξοιμεθᾰ";
    future_opt_mid_2: "ὀ~ξοιο", "ὀ~ξοισθον", "ὀ~ξοισθε";
    future_opt_mid_3: "ὀ~ξοιτο", "ὀ~ξοισθην", "ὀ~ξοιντο";
    future_opt_pass_1: "ὀ~χθησοιμην", _, "ὀ~χθησοιμεθᾰ";
    future_opt_pass_2: "ὀ~χθησοιο", "ὀ~χθησοισθον", "ὀ~χθησοισθε";
    future_opt_pass_3: "ὀ~χθησοιτο", "ὀ~χθησοισθην", "ὀ~χθησοιντο";
    aorist_ind_act_1: "ὠ~ξᾰ", _, "ὠ~ξᾰμεν";
    aorist_ind_act_2: "ὠ~ξᾰς", "ὠ~ξᾰτον", "ὠ~ξᾰτε";
    aorist_ind_act_3: "ὠ~ξε|ὠ~ξεν", "ὠ~ξᾰτην", "ὠ~ξᾰν";
    aorist_ind_mid_1: "ὠ~ξᾰμην", _, "ὠ~ξᾰμεθᾰ";
    aorist_ind_mid_2: "ὠ~ξω", "ὠ~ξᾰσθον", "ὠ~ξᾰσθε";
    aorist_ind_mid_3: "ὠ~ξᾰτο", "ὠ~ξᾰσθην", "ὠ~ξᾰντο";
    aorist_ind_pass_1: "ὠ~χθην", _, "ὠ~χθημεν";
    aorist_ind_pass_2: "ὠ~χθης", "ὠ~χθητον", "ὠ~χθητε";
    aorist_ind_pass_3: "ὠ~χθη", "ὠ~χθητην", "ὠ~χθησᾰν";
    aorist_subj_act_1: "ὀ~ξω", _, "ὀ~ξωμεν";
    aorist_subj_act_2: "ὀ~ξῃς", "ὀ~ξητον", "ὀ~ξητε";
    aorist_subj_act_3: "ὀ~ξῃ", _, "ὀ~ξωσῐ|ὀ~ξωσῐν";
    aorist_subj_mid_1: "ὀ~ξωμαι", _, "ὀ~ξωμεθᾰ";
    aorist_subj_mid_2: "ὀ~ξῃ", "ὀ~ξησθον", "ὀ~ξησθε";
    aorist_subj_mid_3: "ὀ~ξηται", _, "ὀ~ξωνται";
    aorist_subj_pass_1: "ὀ~χθω", _, "ὀ~χθωμεν";
    aorist_subj_pass_2: "ὀ~χθῃς", "ὀ~χθητον", "ὀ~χθητε";
    aorist_subj_pass_3: "ὀ~χθῃ", _, "ὀ~χθωσῐ|ὀ~χθωσῐν";
    aorist_opt_act_1: "ὀ~ξαιμῐ", _, "ὀ~ξαιμεν";
    aorist_opt_act_2: "ὀ~ξειᾰς|ὀ~ξαις", "ὀ~ξαιτον", "ὀ~ξαιτε";
    aorist_opt_act_3: "ὀ~ξειε|ὀ~ξειεν|ὀ~ξαι", "ὀ~ξαιτην", "ὀ~ξειᾰν|ὀ~ξαιεν";
    aorist_opt_mid_1: "ὀ~ξαιμην", _, "ὀ~ξαιμεθᾰ";
    aorist_opt_mid_2: "ὀ~ξαιο", "ὀ~ξαισθον", "ὀ~ξαισθε";
    aorist_opt_mid_3: "ὀ~ξαιτο", "ὀ~ξαισθην", "ὀ~ξαιντο";
    aorist_opt_pass_1: "ὀ~χθειην", _, "ὀ~χθεῑμεν|ὀ~χθειημεν";
    aorist_opt_pass_2: "ὀ~χθειης", "ὀ~χθεῑτον|ὀ~χθειητον", "ὀ~χθεῑτε|ὀ~χθειητε";
    aorist_opt_pass_3: "ὀ~χθειη", "ὀ~χθειτην|ὀ~χθειητην", "ὀ~χθεῑεν|ὀ~χθειησᾰν";
    aorist_imp_act_2: "ὀ~ξον", "ὀ~ξᾰτον", "ὀ~ξᾰτε";
    aorist_imp_act_3: "ὀ~ξᾰτω", "ὀ~ξᾰτων", "ὀ~ξᾰντων";
    aorist_imp_mid_2: "ὀ~ξαι", "ὀ~ξᾰσθον", "ὀ~ξᾰσθε";
    aorist_imp_mid_3: "ὀ~ξᾰσθω", "ὀ~ξᾰσθων", _;
    aorist_imp_pass_2: "ὀ~χθητῐ", _, _;
    aorist_imp_pass_3: "ὀ~χθητω", "ὀ~χθητων", "ὀ~χθεντων";
    perfect_ind_act_1: "ὠ~χᾰ", _, "ὠ~χᾰμεν";
    perfect_ind_act_2: "ὠ~χᾰς", "ὠ~χᾰτον", "ὠ~χᾰτε";
    perfect_ind_act_3: "ὠ~χε|ὠ~χεν", _, "ὠ~χᾱσῐ|ὠ~χᾱσῐν";
    perfect_ind_mid_1: "ὠ~γμαι", _, "ὠ~γμεθᾰ";
    perfect_ind_mid_2: "ὠ~ξαι", "ὠ~χθον", "ὠ~χθε";
    perfect_ind_mid_3: "ὠ~κται", _, "ὠ~γᾰται";
    perfect_ind_pass_1: _, _, "ὠ~γμεθᾰ";
    perfect_ind_pass_2: _, _, "ὠ~χθε";
    perfect_subj_act_1: "ὠ~χω", _, "ὠ~χωμεν";
    perfect_subj_act_2: "ὠ~χῃς", "ὠ~χητον", "ὠ~χητε";
    perfect_subj_act_3: "ὠ~χῃ", _, "ὠ~χωσῐ|ὠ~χωσῐν";
    perfect_opt_act_1: "ὠ~χοιμῐ|ὠ~χοιην", _, "ὠ~χοιμεν";
    perfect_opt_act_2: "ὠ~χοις|ὠ~χοιης", "ὠ~χοιτον", "ὠ~χοιτε";
    perfect_opt_act_3: "ὠ~χοι|ὠ~χοιη", "ὠ~χοιτην", "ὠ~χοιεν";
    perfect_imp_act_2: "ὠ~χε", "ὠ~χετον", "ὠ~χετε";
    perfect_imp_act_3: "ὠ~χετω", "ὠ~χετων", "ὠ~χοντων";
    perfect_imp_mid_2: "ὠ~ξο", _, _;
    perfect_imp_mid_3: "ὠ~χθω", "ὠ~χθων", _;
    perfect_imp_pass_2: "ὠ~ξο", _, _;
    perfect_imp_pass_3: "ὠ~χθω", "ὠ~χθων", _;
    pluperfect_act_1: "ὠ~χειν|ὠ~χη", _, "ὠ~χεμεν";
    pluperfect_act_2: "ὠ~χεις|ὠ~χης", "ὠ~χετον", "ὠ~χετε";
    pluperfect_act_3: "ὠ~χει|ὠ~χειν", "ὠ~χετην", "ὠ~χεσᾰν";
    pluperfect_mid_1: "ὠ~γμην", _, "ὠ~γμεθᾰ";
    pluperfect_mid_2: "ὠ~ξο", "ὠ~χθον", "ὠ~χθε";
    pluperfect_mid_3: "ὠ~κτο", "ὠ~χθην", _;
    pluperfect_pass_1: _, _, "ὠ~γμεθᾰ";
    pluperfect_pass_2: _, _, "ὠ~χθε";
    pluperfect_pass_3: _, "ὠ~χθην", "ὠ~γᾰτο";
    infinitive_pres: "ὀ~γνῠναι", "ὀ~γνῠσθαι", _;
    infinitive_fut: "ὀ~ξειν", "ὀ~ξεσθαι", "ὀ~χθησεσθαι";
    infinitive_aor: "ὀ~ξαι", "ὀ~ξᾰσθαι", "ὀ~χθηναι";
    infinitive_perf: "ὠ~χεναι", "ὠ~χθαι", _;
    participle_pres: "ὀ~γνῡς", "ὀ~γνῠμενος", _;
    participle_fut: "ὀ~ξων", "ὀ~ξομενος", "ὀ~χθησομενος";
    participle_aor: "ὀ~ξᾱς", "ὀ~ξᾰμενος", "ὀ~χθεις";
    participle_perf: "ὠ~χως", "ὠ~γμενος", _;
    ),
    // Δῐϊστημῐ: 1 words (δῐϊστημῐ). Like Ἀνῐστημῐ, except in 126 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("δῐϊστημῐ", 1, "~ϊ~ημῐ", like Ἀνῐστημῐ;
    ind_act_1: "~ϊ~ημῐ", _, "~ϊ~ᾰμεν";
    ind_act_2: "~ϊ~ης", _, _;
    ind_act_3: "~ϊ~ησῐ|~ϊ~ησῐν", _, "~ϊ~ᾱσῐ|~ϊ~ᾱσῐν";
    ind_mid_1: "~ϊ~ᾰμαι", _, "~ϊ~ᾰμεθᾰ";
    ind_mid_2: "~ϊ~ᾰσαι", "~ϊ~ᾰσθον", "~ϊ~ᾰσθε";
    ind_mid_3: "~ϊ~ᾰται", _, "~ϊ~ᾰνται";
    ind_pass_1: _, _, "~ϊ~ᾰμεθᾰ";
    ind_pass_2: _, _, "~ϊ~ᾰσθε";
    ind_pass_3: "~ϊ~ᾰται", _, "~ϊ~ᾰνται";
    subj_act_1: "~ϊ~ω", _, "~ϊ~ωμεν";
    subj_act_2: "~ϊ~ῃς", "~ϊ~ητον", "~ϊ~ητε";
    subj_act_3: "~ϊ~ῃ", _, "~ϊ~ωσῐ|~ϊ~ωσῐν";
    subj_mid_1: "~ϊ~ωμαι", _, "~ϊ~ωμεθᾰ";
    subj_mid_2: "~ϊ~ῃ", "~ϊ~ησθον", "~ϊ~ησθε";
    subj_mid_3: "~ϊ~ηται", _, "~ϊ~ωνται";
    subj_pass_1: _, _, "~ϊ~ωμεθᾰ";
    subj_pass_2: _, _, "~ϊ~ησθε";
    subj_pass_3: "~ϊ~ηται", _, "~ϊ~ωνται";
    opt_act_1: "~ϊ~αιην", _, "~ϊ~αῑμεν|~ϊ~αιημεν";
    opt_act_2: "~ϊ~αιης", "~ϊ~αῑτον|~ϊ~αιητον", "~ϊ~αῑτε|~ϊ~αιητε";
    opt_act_3: "~ϊ~αιη", "~ϊ~αιτην|~ϊ~αιητην", "~ϊ~αῑεν|~ϊ~αιησᾰν";
    opt_mid_1: "~ϊ~αιμην", _, "~ϊ~αιμεθᾰ";
    opt_mid_2: "~ϊ~αιο", "~ϊ~αισθον", "~ϊ~αισθε";
    opt_mid_3: "~ϊ~αιτο", "~ϊ~αισθην", "~ϊ~αιντο";
    opt_pass_1: _, _, "~ϊ~αιμεθᾰ";
    opt_pass_2: _, "~ϊ~αισθον", "~ϊ~αισθε";
    opt_pass_3: "~ϊ~αιτο", "~ϊ~αισθην", "~ϊ~αιντο";
    imp_act_2: "~ϊ~η", "~ϊ~ᾰτον", "~ϊ~ᾰτε";
    imp_act_3: "~ϊ~ᾰτω", "~ϊ~ᾰτων", "~ϊ~ᾰντων";
    imp_mid_2: "~ϊ~ᾰσο|~ϊ~ω", _, "~ϊ~ᾰσθε";
    imp_mid_3: "~ϊ~ᾰσθω", "~ϊ~ᾰσθων", "~ϊ~ᾰσθων";
    imp_pass_2: "~ϊ~ᾰσο|~ϊ~ω", _, "~ϊ~ᾰσθε";
    imp_pass_3: _, "~ϊ~ᾰσθων", _;
    imperfect_act_1: "~ϊ~ην", _, "~ϊ~ᾰμεν";
    imperfect_act_2: "~ϊ~ης", "~ϊ~ᾰτον", "~ϊ~ᾰτε";
    imperfect_act_3: "~ϊ~η", "~ϊ~ᾰτην", "~ϊ~ᾰσᾰν";
    imperfect_mid_1: "~ϊ~ᾰμην", _, "~ϊ~ᾰμεθᾰ";
    imperfect_mid_2: "~ϊ~ᾰσο", "~ϊ~ᾰσθον", "~ϊ~ᾰσθε";
    imperfect_mid_3: "~ϊ~ᾰτο", "~ϊ~ᾰσθην", "~ϊ~ᾰντο";
    imperfect_pass_1: _, _, "~ϊ~ᾰμεθᾰ";
    imperfect_pass_2: _, _, "~ϊ~ᾰσθε";
    imperfect_pass_3: "~ϊ~ᾰτο", _, _;
    aorist_ind_act_3: _, _, "~ε~ησᾰν|~ε~ᾰν";
    perfect_ind_act_1: "~ε~ηκᾰ", _, "~ε~ηκᾰμεν";
    perfect_ind_act_2: "~ε~ηκᾰς", "~ε~ηκᾰτον", "~ε~ηκᾰτε";
    perfect_ind_act_3: "~ε~ηκε|~ε~ηκεν", derived, "~ε~ηκᾱσῐ|~ε~ηκᾱσῐν";
    perfect_subj_act_1: "~ε~ηκω", _, "~ε~ηκωμεν";
    perfect_subj_act_2: "~ε~ηκῃς", "~ε~ηκητον", "~ε~ηκητε";
    perfect_subj_act_3: "~ε~ηκῃ", derived, "~ε~ηκωσῐ|~ε~ηκωσῐν";
    perfect_opt_act_1: "~ε~ηκοιμῐ|~ε~ηκοιην", _, "~ε~ηκοιμεν";
    perfect_opt_act_2: "~ε~ηκοις|~ε~ηκοιης", "~ε~ηκοιτον", "~ε~ηκοιτε";
    perfect_opt_act_3: "~ε~ηκοι|~ε~ηκοιη", "~ε~ηκοιτην", "~ε~ηκοιεν";
    perfect_imp_act_2: "~ε~ηκε", "~ε~ηκετον", "~ε~ηκετε";
    perfect_imp_act_3: "~ε~ηκετω", "~ε~ηκετων", "~ε~ηκοντων";
    infinitive_pres: "~ϊ~ᾰναι", "~ϊ~ᾰσθαι", _;
    infinitive_perf: "~ε~ηκεναι", _, _;
    participle_pres: "~ϊ~ᾱς", "~ϊ~ᾰμενος", _;
    participle_perf: "~ε~ηκως", _, _;
    ),
    // Στορνῡμῐ: 1 words (στορνῡμῐ). Like Σῠνομνῡμῐ, except in 229 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("στορνῡμῐ", 1, "~ο~νῡμῐ", like Σῠνομνῡμῐ;
    imperfect_act_1: "ἐ~ο~νῡν", _, "ἐ~ο~νῠμεν";
    imperfect_act_2: "ἐ~ο~νῡς", "ἐ~ο~νῠτον", "ἐ~ο~νῠτε";
    imperfect_act_3: "ἐ~ο~νῡ", "ἐ~ο~νῠτην", "ἐ~ο~νῠσᾰν";
    imperfect_mid_1: "ἐ~ο~νῠμην", _, "ἐ~ο~νῠμεθᾰ";
    imperfect_mid_2: "ἐ~ο~νῠσο", "ἐ~ο~νῠσθον", "ἐ~ο~νῠσθε";
    imperfect_mid_3: "ἐ~ο~νῠτο", "ἐ~ο~νῠσθην", "ἐ~ο~νῠντο";
    imperfect_pass_1: _, _, "ἐ~ο~νῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ο~νῠσθε";
    imperfect_pass_3: "ἐ~ο~νῠτο", _, _;
    future_ind_mid_1: derived, _, derived;
    future_ind_mid_2: derived, derived, derived;
    future_ind_mid_3: derived, _, derived;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_mid_1: derived, _, derived;
    future_opt_mid_2: derived, derived, derived;
    future_opt_mid_3: derived, derived, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_act_1: "ἐ~ο~εσᾰ|ἐ~~ωσᾰ", _, "ἐ~ο~εσᾰμεν|ἐ~~ωσᾰμεν";
    aorist_ind_act_2: "ἐ~ο~εσᾰς|ἐ~~ωσᾰς", "ἐ~ο~εσᾰτον|ἐ~~ωσᾰτον", "ἐ~ο~εσᾰτε|ἐ~~ωσᾰτε";
    aorist_ind_act_3: "ἐ~ο~εσε|ἐ~ο~εσεν|ἐ~~ωσε|ἐ~~ωσεν", "ἐ~ο~εσᾰτην|ἐ~~ωσᾰτην", "ἐ~ο~εσᾰν|ἐ~~ωσᾰν";
    aorist_ind_mid_1: "ἐ~ο~εσᾰμην|ἐ~~ωσᾰμην", _, "ἐ~ο~εσᾰμεθᾰ|ἐ~~ωσᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~ο~εσω|ἐ~~ωσω", "ἐ~ο~εσᾰσθον|ἐ~~ωσᾰσθον", "ἐ~ο~εσᾰσθε|ἐ~~ωσᾰσθε";
    aorist_ind_mid_3: "ἐ~ο~εσᾰτο|ἐ~~ωσᾰτο", "ἐ~ο~εσᾰσθην|ἐ~~ωσᾰσθην", "ἐ~ο~εσᾰντο|ἐ~~ωσᾰντο";
    aorist_ind_pass_1: "ἐ~ο~εσθην|ἐ~~ωθην|ἐ~ο~ηθην", _, "ἐ~ο~εσθημεν|ἐ~~ωθημεν|ἐ~ο~ηθημεν";
    aorist_ind_pass_2: "ἐ~ο~εσθης|ἐ~~ωθης|ἐ~ο~ηθης", "ἐ~ο~εσθητον|ἐ~~ωθητον|ἐ~ο~ηθητον", "ἐ~ο~εσθητε|ἐ~~ωθητε|ἐ~ο~ηθητε";
    aorist_ind_pass_3: "ἐ~ο~εσθη|ἐ~~ωθη|ἐ~ο~ηθη", "ἐ~ο~εσθητην|ἐ~~ωθητην|ἐ~ο~ηθητην", "ἐ~ο~εσθησᾰν|ἐ~~ωθησᾰν|ἐ~ο~ηθησᾰν";
    aorist_subj_act_1: "~ο~εσω|~~ωσω", _, "~ο~εσωμεν|~~ωσωμεν";
    aorist_subj_act_2: "~ο~εσῃς|~~ωσῃς", "~ο~εσητον|~~ωσητον", "~ο~εσητε|~~ωσητε";
    aorist_subj_act_3: "~ο~εσῃ|~~ωσῃ", "~ο~εσητον|~~ωσητον", "~ο~εσωσῐ|~ο~εσωσῐν|~~ωσωσῐ|~~ωσωσῐν";
    aorist_subj_mid_1: "~ο~εσωμαι|~~ωσωμαι", _, "~ο~εσωμεθᾰ|~~ωσωμεθᾰ";
    aorist_subj_mid_2: "~ο~εσῃ|~~ωσῃ", "~ο~εσησθον|~~ωσησθον", "~ο~εσησθε|~~ωσησθε";
    aorist_subj_mid_3: "~ο~εσηται|~~ωσηται", "~ο~εσησθον|~~ωσησθον", "~ο~εσωνται|~~ωσωνται";
    aorist_subj_pass_1: "~ο~εσθω|~~ωθω|~ο~ηθω", _, "~ο~εσθωμεν|~~ωθωμεν|~ο~ηθωμεν";
    aorist_subj_pass_2: "~ο~εσθῃς|~~ωθῃς|~ο~ηθῃς", "~ο~εσθητον|~~ωθητον|~ο~ηθητον", "~ο~εσθητε|~~ωθητε|~ο~ηθητε";
    aorist_subj_pass_3: "~ο~εσθῃ|~~ωθῃ|~ο~ηθῃ", "~ο~εσθητον|~~ωθητον|~ο~ηθητον", "~ο~εσθωσῐ|~ο~εσθωσῐν|~~ωθωσῐ|~~ωθωσῐν|~ο~ηθωσῐ|~ο~ηθωσῐν";
    aorist_opt_act_1: "~ο~εσαιμῐ|~~ωσαιμῐ", _, "~ο~εσαιμεν|~~ωσαιμεν";
    aorist_opt_act_2: "~ο~εσειᾰς|~ο~εσαις|~~ωσειᾰς|~~ωσαις", "~ο~εσαιτον|~~ωσαιτον", "~ο~εσαιτε|~~ωσαιτε";
    aorist_opt_act_3: "~ο~εσειε|~ο~εσειεν|~ο~εσαι|~~ωσειε|~~ωσειεν|~~ωσαι", "~ο~εσαιτην|~~ωσαιτην", "~ο~εσειᾰν|~ο~εσαιεν|~~ωσειᾰν|~~ωσαιεν";
    aorist_opt_mid_1: "~ο~εσαιμην|~~ωσαιμην", _, "~ο~εσαιμεθᾰ|~~ωσαιμεθᾰ";
    aorist_opt_mid_2: "~ο~εσαιο|~~ωσαιο", "~ο~εσαισθον|~~ωσαισθον", "~ο~εσαισθε|~~ωσαισθε";
    aorist_opt_mid_3: "~ο~εσαιτο|~~ωσαιτο", "~ο~εσαισθην|~~ωσαισθην", "~ο~εσαιντο|~~ωσαιντο";
    aorist_opt_pass_1: "~ο~εσθειην|~~ωθειην|~ο~ηθειην", _, "~ο~εσθεῑμεν|~ο~εσθειημεν|~~ωθεῑμεν|~~ωθειημεν|~ο~ηθεῑμεν|~ο~ηθειημεν";
    aorist_opt_pass_2: "~ο~εσθειης|~~ωθειης|~ο~ηθειης", "~ο~εσθεῑτον|~ο~εσθειητον|~~ωθεῑτον|~~ωθειητον|~ο~ηθεῑτον|~ο~ηθειητον", "~ο~εσθεῑτε|~ο~εσθειητε|~~ωθεῑτε|~~ωθειητε|~ο~ηθεῑτε|~ο~ηθειητε";
    aorist_opt_pass_3: "~ο~εσθειη|~~ωθειη|~ο~ηθειη", "~ο~εσθειτην|~ο~εσθειητην|~~ωθειτην|~~ωθειητην|~ο~ηθειτην|~ο~ηθειητην", "~ο~εσθεῑεν|~ο~εσθειησᾰν|~~ωθεῑεν|~~ωθειησᾰν|~ο~ηθεῑεν|~ο~ηθειησᾰν";
    aorist_imp_act_2: "~ο~εσον|~~ωσον", "~ο~εσᾰτον|~~ωσᾰτον", "~ο~εσᾰτε|~~ωσᾰτε";
    aorist_imp_act_3: "~ο~εσᾰτω|~~ωσᾰτω", "~ο~εσᾰτων|~~ωσᾰτων", "~ο~εσᾰντων|~~ωσᾰντων";
    aorist_imp_mid_2: "~ο~εσαι|~~ωσαι", "~ο~εσᾰσθον|~~ωσᾰσθον", "~ο~εσᾰσθε|~~ωσᾰσθε";
    aorist_imp_mid_3: "~ο~εσᾰσθω|~~ωσᾰσθω", "~ο~εσᾰσθων|~~ωσᾰσθων", "~ο~εσᾰσθων|~~ωσᾰσθων";
    aorist_imp_pass_2: "~ο~εσθητῐ|~~ωθητῐ|~ο~ηθητῐ", "~ο~εσθητον|~~ωθητον|~ο~ηθητον", "~ο~εσθητε|~~ωθητε|~ο~ηθητε";
    aorist_imp_pass_3: "~ο~εσθητω|~~ωθητω|~ο~ηθητω", "~ο~εσθητων|~~ωθητων|~ο~ηθητων", "~ο~εσθεντων|~~ωθεντων|~ο~ηθεντων";
    perfect_ind_act_1: "ἐ~~ωκᾰ", _, "ἐ~~ωκᾰμεν";
    perfect_ind_act_2: "ἐ~~ωκᾰς", "ἐ~~ωκᾰτον", "ἐ~~ωκᾰτε";
    perfect_ind_act_3: "ἐ~~ωκε|ἐ~~ωκεν", _, "ἐ~~ωκᾱσῐ|ἐ~~ωκᾱσῐν";
    perfect_ind_mid_1: "ἐ~~ωμαι", _, "ἐ~~ωμεθᾰ";
    perfect_ind_mid_2: "ἐ~~ωσαι", "ἐ~~ωσθον", "ἐ~~ωσθε";
    perfect_ind_mid_3: "ἐ~~ωται", _, "ἐ~~ωνται";
    perfect_ind_pass_1: "ἐ~~ωμαι|ἐ~ο~εσμαι", _, "ἐ~~ωμεθᾰ|ἐ~ο~εσμεθᾰ";
    perfect_ind_pass_2: "ἐ~~ωσαι|ἐ~ο~εσαι", "ἐ~~ωσθον|ἐ~ο~εσθον", "ἐ~~ωσθε|ἐ~ο~εσθε";
    perfect_ind_pass_3: "ἐ~~ωται|ἐ~ο~εσται", "ἐ~~ωσθον|ἐ~ο~εσθον", "ἐ~~ωνται|ἐ~ο~εσᾰται";
    perfect_subj_act_1: "ἐ~~ωκω", _, "ἐ~~ωκωμεν";
    perfect_subj_act_2: "ἐ~~ωκῃς", "ἐ~~ωκητον", "ἐ~~ωκητε";
    perfect_subj_act_3: "ἐ~~ωκῃ", _, "ἐ~~ωκωσῐ|ἐ~~ωκωσῐν";
    perfect_opt_act_1: "ἐ~~ωκοιμῐ|ἐ~~ωκοιην", _, "ἐ~~ωκοιμεν";
    perfect_opt_act_2: "ἐ~~ωκοις|ἐ~~ωκοιης", "ἐ~~ωκοιτον", "ἐ~~ωκοιτε";
    perfect_opt_act_3: "ἐ~~ωκοι|ἐ~~ωκοιη", "ἐ~~ωκοιτην", "ἐ~~ωκοιεν";
    perfect_imp_act_2: "ἐ~~ωκε", "ἐ~~ωκετον", "ἐ~~ωκετε";
    perfect_imp_act_3: "ἐ~~ωκετω", "ἐ~~ωκετων", "ἐ~~ωκοντων";
    perfect_imp_mid_2: "ἐ~~ωσο", _, _;
    perfect_imp_mid_3: "ἐ~~ωσθω", "ἐ~~ωσθων", _;
    perfect_imp_pass_2: "ἐ~~ωσο|ἐ~ο~εσο", "ἐ~~ωσθον|ἐ~ο~εσθον", "ἐ~~ωσθε|ἐ~ο~εσθε";
    perfect_imp_pass_3: "ἐ~~ωσθω|ἐ~ο~εσθω", "ἐ~~ωσθων|ἐ~ο~εσθων", "ἐ~~ωσθων|ἐ~ο~εσθων";
    pluperfect_act_1: "ἐ~~ωκειν|ἐ~~ωκη", _, "ἐ~~ωκεμεν";
    pluperfect_act_2: "ἐ~~ωκεις|ἐ~~ωκης", "ἐ~~ωκετον", "ἐ~~ωκετε";
    pluperfect_act_3: "ἐ~~ωκει|ἐ~~ωκειν", "ἐ~~ωκετην", "ἐ~~ωκεσᾰν";
    pluperfect_mid_1: "ἐ~~ωμην", _, "ἐ~~ωμεθᾰ";
    pluperfect_mid_2: "ἐ~~ωσο", "ἐ~~ωσθον", "ἐ~~ωσθε";
    pluperfect_mid_3: "ἐ~~ωτο", "ἐ~~ωσθην", "ἐ~~ωντο";
    pluperfect_pass_1: "ἐ~~ωμην|ἐ~ο~εσμην", _, "ἐ~~ωμεθᾰ|ἐ~ο~εσμεθᾰ";
    pluperfect_pass_2: "ἐ~~ωσο|ἐ~ο~εσο", "ἐ~~ωσθον|ἐ~ο~εσθον", "ἐ~~ωσθε|ἐ~ο~εσθε";
    pluperfect_pass_3: "ἐ~~ωτο|ἐ~ο~εστο", "ἐ~~ωσθην|ἐ~ο~εσθην", "ἐ~~ωντο|ἐ~ο~εσᾰτο";
    infinitive_fut: _, derived, derived;
    infinitive_aor: "~ο~εσαι|~~ωσαι", "~ο~εσᾰσθαι|~~ωσᾰσθαι", "~ο~εσθηναι|~~ωθηναι|~ο~ηθηναι";
    infinitive_perf: "ἐ~~ωκεναι", "ἐ~~ωσθαι", _;
    participle_fut: _, derived, derived;
    participle_aor: "~ο~εσᾱς|~~ωσᾱς", "~ο~εσᾰμενος|~~ωσᾰμενος", "~ο~εσθεις|~~ωθεις|~ο~ηθεις";
    participle_perf: "ἐ~~ωκως", "ἐ~~ωμενος", _;
    ),
    // Ἀφῐστημῐ: 1 words (ἀφῐστημῐ). Like Ἐφῐστημῐ, except in 304 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 du, ind_act_2 pl, ind_act_3 sg, ….
    conjugation!("ἀφῐστημῐ", 1, "~φῐ~ημῐ", like Ἐφῐστημῐ;
    ind_act_1: "~φῐ~ημῐ", _, "~φῐ~ᾰμεν";
    ind_act_2: "~φῐ~ης", derived, derived;
    ind_act_3: "~φῐ~ησῐ|~φῐ~ησῐν", derived, "~φῐ~ᾱσῐ|~φῐ~ᾱσῐν";
    ind_mid_1: "~φῐ~ᾰμαι", _, "~φῐ~ᾰμεθᾰ";
    ind_mid_2: "~φῐ~ᾰσαι", "~φῐ~ᾰσθον", "~φῐ~ᾰσθε";
    ind_mid_3: "~φῐ~ᾰται", derived, "~φῐ~ᾰνται";
    ind_pass_1: derived, _, "~φῐ~ᾰμεθᾰ";
    ind_pass_2: derived, derived, "~φῐ~ᾰσθε";
    ind_pass_3: "~φῐ~ᾰται", derived, "~φῐ~ᾰνται";
    subj_act_1: "~φῐ~ω", _, "~φῐ~ωμεν";
    subj_act_2: "~φῐ~ῃς", "~φῐ~ητον", "~φῐ~ητε";
    subj_act_3: "~φῐ~ῃ", derived, "~φῐ~ωσῐ|~φῐ~ωσῐν";
    subj_mid_1: "~φῐ~ωμαι", _, "~φῐ~ωμεθᾰ";
    subj_mid_2: "~φῐ~ῃ", "~φῐ~ησθον", "~φῐ~ησθε";
    subj_mid_3: "~φῐ~ηται", derived, "~φῐ~ωνται";
    subj_pass_1: derived, _, "~φῐ~ωμεθᾰ";
    subj_pass_2: derived, derived, "~φῐ~ησθε";
    subj_pass_3: "~φῐ~ηται", derived, "~φῐ~ωνται";
    opt_act_1: "~φῐ~αιην", _, "~φῐ~αῑμεν|~φῐ~αιημεν";
    opt_act_2: "~φῐ~αιης", "~φῐ~αῑτον|~φῐ~αιητον", "~φῐ~αῑτε|~φῐ~αιητε";
    opt_act_3: "~φῐ~αιη", "~φῐ~αιτην|~φῐ~αιητην", "~φῐ~αῑεν|~φῐ~αιησᾰν";
    opt_mid_1: "~φῐ~αιμην", _, "~φῐ~αιμεθᾰ";
    opt_mid_2: "~φῐ~αιο", "~φῐ~αισθον", "~φῐ~αισθε";
    opt_mid_3: "~φῐ~αιτο", "~φῐ~αισθην", "~φῐ~αιντο";
    opt_pass_1: derived, _, "~φῐ~αιμεθᾰ";
    opt_pass_2: derived, "~φῐ~αισθον", "~φῐ~αισθε";
    opt_pass_3: "~φῐ~αιτο", "~φῐ~αισθην", "~φῐ~αιντο";
    imp_act_2: "~φῐ~η", "~φῐ~ᾰτον", "~φῐ~ᾰτε";
    imp_act_3: "~φῐ~ᾰτω", "~φῐ~ᾰτων", "~φῐ~ᾰντων";
    imp_mid_2: "~φῐ~ᾰσο|~φῐ~ω", derived, "~φῐ~ᾰσθε";
    imp_mid_3: "~φῐ~ᾰσθω", "~φῐ~ᾰσθων", "~φῐ~ᾰσθων";
    imp_pass_2: "~φῐ~ᾰσο|~φῐ~ω", derived, "~φῐ~ᾰσθε";
    imp_pass_3: derived, "~φῐ~ᾰσθων", derived;
    imperfect_act_1: "~φῑ~ην", _, "~φῑ~ᾰμεν";
    imperfect_act_2: "~φῑ~ης", "~φῑ~ᾰτον", "~φῑ~ᾰτε";
    imperfect_act_3: "~φῑ~η", "~φῑ~ᾰτην", "~φῑ~ᾰσᾰν";
    imperfect_mid_1: "~φῑ~ᾰμην", _, "~φῑ~ᾰμεθᾰ";
    imperfect_mid_2: "~φῑ~ᾰσο", "~φῑ~ᾰσθον", "~φῑ~ᾰσθε";
    imperfect_mid_3: "~φῑ~ᾰτο", "~φῑ~ᾰσθην", "~φῑ~ᾰντο";
    imperfect_pass_1: derived, _, "~φῑ~ᾰμεθᾰ";
    imperfect_pass_2: derived, derived, "~φῑ~ᾰσθε";
    imperfect_pass_3: "~φῑ~ᾰτο", derived, derived;
    future_ind_act_1: "~πο~ησω", _, "~πο~ησομεν";
    future_ind_act_2: "~πο~ησεις", "~πο~ησετον", "~πο~ησετε";
    future_ind_act_3: "~πο~ησει", _, "~πο~ησουσῐ|~πο~ησουσῐν";
    future_ind_mid_1: "~πο~ησομαι", _, "~πο~ησομεθᾰ";
    future_ind_mid_2: "~πο~ησῃ|~πο~ησει", "~πο~ησεσθον", "~πο~ησεσθε";
    future_ind_mid_3: "~πο~ησεται", _, "~πο~ησονται";
    future_ind_pass_1: "~πο~ησομαι", _, "~πο~ησομεθᾰ";
    future_ind_pass_2: "~πο~ησῃ|~πο~ησει", "~πο~ησεσθον", "~πο~ησεσθε";
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
    aorist_ind_act_1: "~πε~ησᾰ|~πε~ην", _, "~πε~ησᾰμεν|~πε~ημεν";
    aorist_ind_act_2: "~πε~ησᾰς|~πε~ης", "~πε~ησᾰτον|~πε~ητον", "~πε~ησᾰτε|~πε~ητε";
    aorist_ind_act_3: "~πε~ησε|~πε~ησεν|~πε~η", "~πε~ησᾰτην|~πε~ητην", "~πε~ησᾰν|~πε~ᾰν";
    aorist_ind_mid_1: "~πε~ησᾰμην|~πε~ᾰμην", _, "~πε~ησᾰμεθᾰ|~πε~ᾰμεθᾰ";
    aorist_ind_mid_2: "~πε~ησω|~πε~ω", "~πε~ησᾰσθον|~πε~ᾰσθον", "~πε~ησᾰσθε|~πε~ᾰσθε";
    aorist_ind_mid_3: "~πε~ησᾰτο|~πε~ᾰτο", "~πε~ησᾰσθην|~πε~ᾰσθην", "~πε~ησᾰντο|~πε~ᾰντο";
    aorist_ind_pass_1: "~πε~ᾰθην", _, "~πε~ᾰθημεν";
    aorist_ind_pass_2: "~πε~ᾰθης", "~πε~ᾰθητον", "~πε~ᾰθητε";
    aorist_ind_pass_3: "~πε~ᾰθη", "~πε~ᾰθητην", "~πε~ᾰθησᾰν";
    aorist_subj_act_1: "~πο~ησω|~πο~ω", _, "~πο~ησωμεν|~πο~ωμεν";
    aorist_subj_act_2: "~πο~ησῃς|~πο~ῃς", "~πο~ησητον|~πο~ητον", "~πο~ησητε|~πο~ητε";
    aorist_subj_act_3: "~πο~ησῃ|~πο~ῃ", "~πο~ησητον|~πο~ητον", "~πο~ησωσῐ|~πο~ησωσῐν|~πο~ωσῐ|~πο~ωσῐν";
    aorist_subj_mid_1: "~πο~ησωμαι|~πο~ωμαι", _, "~πο~ησωμεθᾰ|~πο~ωμεθᾰ";
    aorist_subj_mid_2: "~πο~ησῃ|~πο~ῃ", "~πο~ησησθον|~πο~ησθον", "~πο~ησησθε|~πο~ησθε";
    aorist_subj_mid_3: "~πο~ησηται|~πο~ηται", "~πο~ησησθον|~πο~ησθον", "~πο~ησωνται|~πο~ωνται";
    aorist_subj_pass_1: "~πο~ᾰθω", _, "~πο~ᾰθωμεν";
    aorist_subj_pass_2: "~πο~ᾰθῃς", "~πο~ᾰθητον", "~πο~ᾰθητε";
    aorist_subj_pass_3: "~πο~ᾰθῃ", derived, "~πο~ᾰθωσῐ|~πο~ᾰθωσῐν";
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
    aorist_imp_mid_3: "~πο~ησᾰσθω|~πο~ᾰσθω", "~πο~ησᾰσθων|~πο~ᾰσθων", "~πο~ησᾰσθων|~πο~ᾰσθων";
    aorist_imp_pass_2: "~πο~ᾰθητῐ", derived, derived;
    aorist_imp_pass_3: "~πο~ᾰθητω", "~πο~ᾰθητων", "~πο~ᾰθεντων";
    perfect_ind_act_1: _, _, "~φε~ᾰκᾰμεν|~φε~ηκᾰμεν";
    perfect_ind_act_2: _, "~φε~ᾰκᾰτον|~φε~ηκᾰτον", "~φε~ᾰκᾰτε|~φε~ηκᾰτε";
    perfect_ind_act_3: _, "~φε~ᾰκᾰτον|~φε~ηκᾰτον", "~φε~ᾰκᾱσῐ|~φε~ᾰκᾱσῐν|~φε~ηκᾱσῐ|~φε~ηκᾱσῐν";
    perfect_subj_act_1: "~φε~ᾰκω|~φε~ηκω", _, "~φε~ᾰκωμεν|~φε~ηκωμεν";
    perfect_subj_act_2: "~φε~ᾰκῃς|~φε~ηκῃς", "~φε~ᾰκητον|~φε~ηκητον", "~φε~ᾰκητε|~φε~ηκητε";
    perfect_subj_act_3: "~φε~ᾰκῃ|~φε~ηκῃ", "~φε~ᾰκητον|~φε~ηκητον", "~φε~ᾰκωσῐ|~φε~ᾰκωσῐν|~φε~ηκωσῐ|~φε~ηκωσῐν";
    perfect_opt_act_1: "~φε~ᾰκοιμῐ|~φε~ᾰκοιην|~φε~ηκοιμῐ|~φε~ηκοιην", _, "~φε~ᾰκοιμεν|~φε~ηκοιμεν";
    perfect_opt_act_2: "~φε~ᾰκοις|~φε~ᾰκοιης|~φε~ηκοις|~φε~ηκοιης", "~φε~ᾰκοιτον|~φε~ηκοιτον", "~φε~ᾰκοιτε|~φε~ηκοιτε";
    perfect_opt_act_3: "~φε~ᾰκοι|~φε~ᾰκοιη|~φε~ηκοι|~φε~ηκοιη", "~φε~ᾰκοιτην|~φε~ηκοιτην", "~φε~ᾰκοιεν|~φε~ηκοιεν";
    perfect_imp_act_2: "~φε~ᾰκε|~φε~ηκε", "~φε~ᾰκετον|~φε~ηκετον", "~φε~ᾰκετε|~φε~ηκετε";
    perfect_imp_act_3: "~φε~ᾰκετω|~φε~ηκετω", "~φε~ᾰκετων|~φε~ηκετων", "~φε~ᾰκοντων|~φε~ηκοντων";
    pluperfect_act_1: "~φε~ηκειν|~φε~ηκη|~φεε~ηκειν|~φεε~ηκη", _, "~φε~ηκεμεν|~φεε~ηκεμεν|~φε~ᾰμεν";
    pluperfect_act_2: "~φε~ηκεις|~φε~ηκης|~φεε~ηκεις|~φεε~ηκης", "~φε~ηκετον|~φεε~ηκετον|~φε~ᾰτον", "~φε~ηκετε|~φεε~ηκετε|~φε~ᾰτε";
    pluperfect_act_3: "~φε~ηκει|~φε~ηκειν|~φεε~ηκει|~φεε~ηκειν", "~φε~ηκετην|~φεε~ηκετην|~φε~ᾰτην", "~φε~ηκεσᾰν|~φεε~ηκεσᾰν|~φε~ᾰσᾰν";
    infinitive_pres: "~φῐ~ᾰναι", "~φῐ~ᾰσθαι", derived;
    infinitive_fut: "~πο~ησειν", "~πο~ησεσθαι", "~πο~ησεσθαι";
    infinitive_aor: "~πο~ησαι|~πο~ηναι", "~πο~ησᾰσθαι|~πο~ᾰσθαι", "~πο~ᾰθηναι";
    infinitive_perf: "~φε~ᾰκεναι|~φε~ηκεναι", _, _;
    participle_pres: "~φῐ~ᾱς", "~φῐ~ᾰμενος", derived;
    participle_fut: "~πο~ησων", "~πο~ησομενος", "~πο~ησομενος";
    participle_aor: "~πο~ησᾱς|~πο~ᾱς", "~πο~ησᾰμενος|~πο~ᾰμενος", "~πο~ᾰθεις";
    participle_perf: "~φε~ᾰκως|~φε~ηκως", _, _;
    ),
    // Κᾰτακειμαι: 1 words (κᾰτακειμαι). Like Προσκειμαι, except in 46 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("κᾰτακειμαι", 1, "~α~ειμαι", like Προσκειμαι;
    ind_act_1: "~α~ειμαι", _, _;
    ind_mid_1: "~ᾰ~ειμαι", _, "~ᾰ~ειμεθα";
    ind_mid_2: "~ᾰ~εισαι", "~ᾰ~εισθον", "~ᾰ~εισθε";
    ind_mid_3: "~ᾰ~ειται", _, "~ᾰ~εινται|~ᾰ~εᾰται|~ᾰ~εονται|~ᾰ~ειᾰται";
    subj_mid_1: "~ᾰ~ωμαι|~ᾰ~εωμαι", _, "~ᾰ~ωμεθᾰ";
    subj_mid_2: "~ᾰ~εῃ", "~ᾰ~εησθον", "~ᾰ~εησθε";
    subj_mid_3: "~ᾰ~εηται|~ᾰ~ειται", _, "~ᾰ~εωνται|~ᾰ~ειωνται";
    opt_mid_1: "~ᾰ~οιμην", _, "~ᾰ~οιμεθᾰ";
    opt_mid_2: "~ᾰ~οῑο", "~ᾰ~οῑσθον", "~ᾰ~οῑσθε";
    opt_mid_3: "~ᾰ~οῑτο", "~ᾰ~οισθην", "~ᾰ~οῑντο";
    imp_mid_2: "~ᾰ~εισο", _, "~ᾰ~εισθον";
    imp_mid_3: "~ᾰ~εισθω", "~ᾰ~εισθων", "~ᾰ~εισθων";
    imperfect_mid_3: _, _, "~ε~ειντο|~ε~εατο";
    future_ind_mid_1: "~ᾰ~εισομαι", _, "~ᾰ~εισομεθᾰ";
    future_ind_mid_2: "~ᾰ~εισῃ|~ᾰ~εισει", "~ᾰ~εισεσθον", "~ᾰ~εισεσθε";
    future_ind_mid_3: "~ᾰ~εισεται", _, "~ᾰ~εισονται";
    future_opt_mid_1: "~ᾰ~εισοιμην", _, "~ᾰ~εισοιμεθᾰ";
    future_opt_mid_2: "~ᾰ~εισοιο", "~ᾰ~εισοισθον", "~ᾰ~εισοισθε";
    future_opt_mid_3: "~ᾰ~εισοιτο", "~ᾰ~εισοισθην", "~ᾰ~εισοιντο";
    infinitive_fut: _, "~ᾰ~εισεσθαι", _;
    participle_fut: _, "~ᾰ~εισομενος", _;
    ),
    // Συνοιδᾰ: 1 words (συνοιδᾰ). Like Ἐπᾰγγελλω, except in 281 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("συνοιδᾰ", 1, "~υ~ο~δᾰ", like Ἐπᾰγγελλω;
    ind_act_1: "~υ~ο~δᾰ", _, derived;
    ind_act_2: derived, _, _;
    ind_act_3: derived, _, derived;
    ind_mid_1: derived, _, derived;
    ind_mid_2: derived, derived, derived;
    ind_mid_3: derived, _, derived;
    ind_pass_1: _, _, derived;
    ind_pass_2: derived, _, derived;
    ind_pass_3: derived, _, derived;
    subj_act_1: derived, _, derived;
    subj_act_2: derived, derived, derived;
    subj_act_3: derived, _, derived;
    subj_mid_1: derived, _, derived;
    subj_mid_2: derived, derived, derived;
    subj_mid_3: derived, _, derived;
    subj_pass_1: _, _, derived;
    subj_pass_2: _, _, derived;
    subj_pass_3: derived, _, derived;
    opt_act_1: derived, _, derived;
    opt_act_2: derived, derived, derived;
    opt_act_3: derived, derived, derived;
    opt_mid_1: derived, _, derived;
    opt_mid_2: derived, derived, derived;
    opt_mid_3: derived, derived, derived;
    opt_pass_1: _, _, derived;
    opt_pass_2: _, derived, derived;
    opt_pass_3: derived, derived, derived;
    imp_act_2: derived, derived, derived;
    imp_act_3: derived, derived, derived;
    imp_mid_2: derived, _, derived;
    imp_mid_3: derived, derived, derived;
    imp_pass_2: _, _, derived;
    imp_pass_3: _, derived, _;
    imperfect_act_1: derived, _, derived;
    imperfect_act_2: derived, derived, derived;
    imperfect_act_3: derived, derived, derived;
    imperfect_mid_1: derived, _, derived;
    imperfect_mid_2: derived, derived, derived;
    imperfect_mid_3: derived, derived, derived;
    imperfect_pass_1: _, _, derived;
    imperfect_pass_2: _, _, derived;
    imperfect_pass_3: derived, _, _;
    future_ind_act_1: derived, _, derived;
    future_ind_act_2: derived, derived, derived;
    future_ind_act_3: derived, _, derived;
    future_ind_mid_1: "~ῠ~ε~σομαι", _, "~ῠ~ε~σομεθᾰ";
    future_ind_mid_2: "~ῠ~ε~σῃ|~ῠ~ε~σει", "~ῠ~ε~σεσθον", "~ῠ~ε~σεσθε";
    future_ind_mid_3: "~ῠ~ε~σεται", _, "~ῠ~ε~σονται";
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_act_1: derived, _, derived;
    future_opt_act_2: derived, derived, derived;
    future_opt_act_3: derived, derived, derived;
    future_opt_mid_1: "~ῠ~ε~σοιμην", _, "~ῠ~ε~σοιμεθᾰ";
    future_opt_mid_2: "~ῠ~ε~σοιο", "~ῠ~ε~σοισθον", "~ῠ~ε~σοισθε";
    future_opt_mid_3: "~ῠ~ε~σοιτο", "~ῠ~ε~σοισθην", "~ῠ~ε~σοιντο";
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_act_1: derived, _, derived;
    aorist_ind_act_2: derived, derived, derived;
    aorist_ind_act_3: derived, derived, derived;
    aorist_ind_mid_1: derived, _, derived;
    aorist_ind_mid_2: derived, derived, derived;
    aorist_ind_mid_3: derived, derived, derived;
    aorist_ind_pass_1: derived, _, derived;
    aorist_ind_pass_2: derived, derived, derived;
    aorist_ind_pass_3: derived, derived, derived;
    aorist_subj_act_1: derived, _, derived;
    aorist_subj_act_2: derived, derived, derived;
    aorist_subj_act_3: derived, _, derived;
    aorist_subj_mid_1: derived, _, derived;
    aorist_subj_mid_2: derived, derived, derived;
    aorist_subj_mid_3: derived, _, derived;
    aorist_subj_pass_1: derived, _, derived;
    aorist_subj_pass_2: derived, derived, derived;
    aorist_subj_pass_3: derived, _, derived;
    aorist_opt_act_1: derived, _, derived;
    aorist_opt_act_2: derived, derived, derived;
    aorist_opt_act_3: derived, derived, derived;
    aorist_opt_mid_1: derived, _, derived;
    aorist_opt_mid_2: derived, derived, derived;
    aorist_opt_mid_3: derived, derived, derived;
    aorist_opt_pass_1: derived, _, derived;
    aorist_opt_pass_2: derived, derived, derived;
    aorist_opt_pass_3: derived, derived, derived;
    aorist_imp_act_2: derived, derived, derived;
    aorist_imp_act_3: derived, derived, derived;
    aorist_imp_mid_2: derived, derived, derived;
    aorist_imp_mid_3: derived, derived, _;
    aorist_imp_pass_2: derived, _, _;
    aorist_imp_pass_3: derived, derived, derived;
    perfect_ind_act_1: derived, _, derived;
    perfect_ind_act_2: derived, derived, derived;
    perfect_ind_act_3: derived, _, derived;
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, derived;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: derived, _, derived;
    perfect_subj_act_2: derived, derived, derived;
    perfect_subj_act_3: derived, _, derived;
    perfect_opt_act_1: derived, _, derived;
    perfect_opt_act_2: derived, derived, derived;
    perfect_opt_act_3: derived, derived, derived;
    perfect_imp_act_2: derived, derived, derived;
    perfect_imp_act_3: derived, derived, derived;
    perfect_imp_mid_2: derived, _, _;
    perfect_imp_mid_3: derived, derived, _;
    perfect_imp_pass_2: derived, _, _;
    perfect_imp_pass_3: derived, derived, _;
    infinitive_pres: derived, derived, _;
    infinitive_fut: derived, "~ῠ~ε~σεσθαι", derived;
    infinitive_aor: derived, derived, derived;
    infinitive_perf: derived, derived, _;
    participle_pres: derived, derived, _;
    participle_fut: derived, "~ῠ~ε~σομενος", derived;
    participle_aor: derived, derived, derived;
    participle_perf: derived, derived, _;
    ),
    // Ξῠνοιδᾰ: 1 words (ξῠνοιδᾰ). Like Πᾰρειμι, except in 54 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_2 pl, ind_act_3 sg, ind_act_3 pl, ….
    conjugation!("ξῠνοιδᾰ", 1, "~οιδᾰ", like Πᾰρειμι;
    ind_act_1: "~οιδᾰ", _, "~ισμεν|~οιδᾰμεν";
    ind_act_2: "~οισθᾰ|~οιδᾰς", _, "~ιστε|~οιδᾰτε";
    ind_act_3: "~οιδε", _, "~ισᾱσῐ|~ισᾱσῐν";
    subj_act_1: "~ειδω", _, "~ειδωμεν";
    subj_act_2: "~ειδῃς", "~ειδητον", "~ειδητε";
    subj_act_3: "~ειδῃ", _, "~ειδωσῐ|~ειδωσῐν";
    opt_act_1: "~ειδειην", _, "~ειδεῑμεν|~ειδειημεν";
    opt_act_2: "~ειδειης", "~ειδεῑτον", "~ειδεῑτε|~ειδειητε";
    opt_act_3: "~ειδειη", "~ειδειτην", "~ειδεῑεν|~ειδειησᾰν";
    imp_act_2: "~ισθῐ", "~ιστον", "~ιστε";
    imp_act_3: "~ιστω", "~ιστων", "~ιστων";
    imperfect_act_1: "~ῃδη|~ῃδειν", _, "~ῃσμεν|~ῃδεμεν|~ῃδειμεν";
    imperfect_act_2: "~ῃδησθᾰ|~ῃδεις", "~ῃστον", "~ῃστε|~ῃδειτε|~ῃδετε";
    imperfect_act_3: "~ῃδει|~ῃδειν", "~ῃστην", "~ῃσᾰν|~ῃδεσᾰν";
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
    // Προοιδᾰ: 1 words (προοιδᾰ). Like Ξῠνοιδᾰ, except in 9 cells: ind_act_1 pl, ind_act_2 pl, ind_act_3 pl, imp_act_2 sg, imp_act_2 du, imp_act_2 pl, ….
    conjugation!("προοιδᾰ", 1, "~οιδᾰ", like Ξῠνοιδᾰ;
    ind_act_1: _, _, "~ϊσμεν|~οιδᾰμεν";
    ind_act_2: _, _, "~ϊστε|~οιδᾰτε";
    ind_act_3: _, _, "~ϊσᾱσῐ|~ϊσᾱσῐν";
    imp_act_2: "~ϊσθῐ", "~ϊστον", "~ϊστε";
    imp_act_3: "~ϊστω", "~ϊστων", "~ϊστων";
    ),
    // Ὀνινημῐ: 1 words (ὀνινημῐ). Like Εἰμῐ, except in 197 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ὀνινημῐ", 1, "ὀνινημῐ", like Εἰμῐ;
    ind_act_1: "ὀνινημῐ", _, "ὀνινᾰμεν";
    ind_act_2: "ὀνινης", _, _;
    ind_act_3: "ὀνινησῐ|ὀνινησῐν", _, "ὀνινᾱσῐ|ὀνινᾱσῐν";
    ind_mid_1: "ὀνινᾰμαι", _, "ὀνινᾰμεθᾰ";
    ind_mid_2: "ὀνινᾰσαι", "ὀνινᾰσθον", "ὀνινᾰσθε";
    ind_mid_3: "ὀνινᾰται", _, "ὀνινᾰνται";
    subj_act_1: "ὀνινω", _, "ὀνινωμεν";
    subj_act_2: "ὀνινῃς", "ὀνινητον", "ὀνινητε";
    subj_act_3: "ὀνινῃ", _, "ὀνινωσῐ|ὀνινωσῐν";
    subj_mid_1: "ὀνινωμαι", _, "ὀνινωμεθᾰ";
    subj_mid_2: "ὀνινῃ", "ὀνινησθον", "ὀνινησθε";
    subj_mid_3: "ὀνινηται", _, "ὀνινωνται";
    opt_act_1: "ὀνιναιην", _, "ὀνιναῑμεν|ὀνιναιημεν";
    opt_act_2: "ὀνιναιης", "ὀνιναῑτον|ὀνιναιητον", "ὀνιναῑτε|ὀνιναιητε";
    opt_act_3: "ὀνιναιη", "ὀνιναιτην|ὀνιναιητην", "ὀνιναῑεν|ὀνιναιησᾰν";
    opt_mid_1: "ὀνιναιμην", _, "ὀνιναιμεθᾰ";
    opt_mid_2: "ὀνιναιο", "ὀνιναισθον", "ὀνιναισθε";
    opt_mid_3: "ὀνιναιτο", "ὀνιναισθην", "ὀνιναιντο";
    imp_act_2: "ὀνινη", "ὀνινᾰτον", "ὀνινᾰτε";
    imp_act_3: "ὀνινᾰτω", "ὀνινᾰτων", "ὀνινᾰντων";
    imp_mid_2: "ὀνινᾰσο|ὀνινω", _, "ὀνινᾰσθε";
    imp_mid_3: "ὀνινᾰσθω", "ὀνινᾰσθων", "ὀνινᾰσθων";
    imperfect_act_1: "ὠφελουν", _, "ὠφελοῡμεν";
    imperfect_act_2: "ὠφελεις", "ὠφελεῑτον", "ὠφελεῑτε";
    imperfect_act_3: "ὠφελει", "ὠφελειτην", "ὠφελουν";
    imperfect_mid_1: "ὠφελουμην", _, "ὠφελουμεθᾰ";
    imperfect_mid_2: "ὠφελοῡ", "ὠφελεῑσθον", "ὠφελεῑσθε";
    imperfect_mid_3: "ὠφελεῑτο", "ὠφελεισθην", "ὠφελοῡντο";
    future_ind_act_1: "ὀνησω", _, "ὀνησομεν";
    future_ind_act_2: "ὀνησεις", "ὀνησετον", "ὀνησετε";
    future_ind_act_3: "ὀνησει", _, "ὀνησουσῐ|ὀνησουσῐν";
    future_ind_mid_1: "ὀνησομαι", _, "ὀνησομεθᾰ";
    future_ind_mid_2: "ὀνησῃ|ὀνησει", "ὀνησεσθον", "ὀνησεσθε";
    future_ind_mid_3: "ὀνησεται", _, "ὀνησονται";
    future_opt_act_1: "ὀνησοιμῐ", _, "ὀνησοιμεν";
    future_opt_act_2: "ὀνησοις", "ὀνησοιτον", "ὀνησοιτε";
    future_opt_act_3: "ὀνησοι", "ὀνησοιτην", "ὀνησοιεν";
    future_opt_mid_1: "ὀνησοιμην", _, "ὀνησοιμεθᾰ";
    future_opt_mid_2: "ὀνησοιο", "ὀνησοισθον", "ὀνησοισθε";
    future_opt_mid_3: "ὀνησοιτο", "ὀνησοισθην", "ὀνησοιντο";
    aorist_ind_act_1: "ὠνησᾰ", _, "ὠνησᾰμεν";
    aorist_ind_act_2: "ὠνησᾰς", "ὠνησᾰτον", "ὠνησᾰτε";
    aorist_ind_act_3: "ὠνησε|ὠνησεν", "ὠνησᾰτην", "ὠνησᾰν";
    aorist_ind_mid_1: "ὠνησᾰμην|ὠνᾰμην|ὠνημην", _, "ὠνησᾰμεθᾰ|ὠνᾰμεθᾰ|ὠνημεθᾰ";
    aorist_ind_mid_2: "ὠνησω|ὠνω|ὠνηο", "ὠνησᾰσθον|ὠνᾰσθον|ὠνησθον", "ὠνησᾰσθε|ὠνᾰσθε|ὠνησθε";
    aorist_ind_mid_3: "ὠνησᾰτο|ὠνᾰτο|ὠνητο", "ὠνησᾰσθην|ὠνᾰσθην|ὠνησθην", "ὠνησᾰντο|ὠνᾰντο|ὠνηντο";
    aorist_ind_pass_1: "ὠνηθην", _, "ὠνηθημεν";
    aorist_ind_pass_2: "ὠνηθης", "ὠνηθητον", "ὠνηθητε";
    aorist_ind_pass_3: "ὠνηθη", "ὠνηθητην", "ὠνηθησᾰν";
    aorist_subj_act_1: "ὀνησω", _, "ὀνησωμεν";
    aorist_subj_act_2: "ὀνησῃς", "ὀνησητον", "ὀνησητε";
    aorist_subj_act_3: "ὀνησῃ", _, "ὀνησωσῐ|ὀνησωσῐν";
    aorist_subj_mid_1: "ὀνησωμαι|ὀνωμαι", _, "ὀνησωμεθᾰ|ὀνωμεθᾰ";
    aorist_subj_mid_2: "ὀνησῃ|ὀνῃ", "ὀνησησθον|ὀνησθον", "ὀνησησθε|ὀνησθε";
    aorist_subj_mid_3: "ὀνησηται|ὀνηται", "ὀνησησθον|ὀνησθον", "ὀνησωνται|ὀνωνται";
    aorist_subj_pass_1: "ὀνηθω", _, "ὀνηθωμεν";
    aorist_subj_pass_2: "ὀνηθῃς", "ὀνηθητον", "ὀνηθητε";
    aorist_subj_pass_3: "ὀνηθῃ", _, "ὀνηθωσῐ|ὀνηθωσῐν";
    aorist_opt_act_1: "ὀνησαιμῐ", _, "ὀνησαιμεν";
    aorist_opt_act_2: "ὀνησειᾰς|ὀνησαις", "ὀνησαιτον", "ὀνησαιτε";
    aorist_opt_act_3: "ὀνησειε|ὀνησειεν|ὀνησαι", "ὀνησαιτην", "ὀνησειᾰν|ὀνησαιεν";
    aorist_opt_mid_1: "ὀνησαιμην|ὀναιμην|ὀνῃμην", _, "ὀνησαιμεθᾰ|ὀναιμεθᾰ|ὀνῃμεθᾰ";
    aorist_opt_mid_2: "ὀνησαιο|ὀναῑο|ὀνῃο", "ὀνησαισθον|ὀναῑσθον|ὀνῃσθον", "ὀνησαισθε|ὀναῑσθε|ὀνῃσθε";
    aorist_opt_mid_3: "ὀνησαιτο|ὀναῑτο|ὀνῃτο", "ὀνησαισθην|ὀναισθην|ὀνῃσθην", "ὀνησαιντο|ὀναῑντο|ὀνῃντο";
    aorist_opt_pass_1: "ὀνηθειην", _, "ὀνηθεῑμεν|ὀνηθειημεν";
    aorist_opt_pass_2: "ὀνηθειης", "ὀνηθεῑτον|ὀνηθειητον", "ὀνηθεῑτε|ὀνηθειητε";
    aorist_opt_pass_3: "ὀνηθειη", "ὀνηθειτην|ὀνηθειητην", "ὀνηθεῑεν|ὀνηθειησᾰν";
    aorist_imp_act_2: "ὀνησον", "ὀνησᾰτον", "ὀνησᾰτε";
    aorist_imp_act_3: "ὀνησᾰτω", "ὀνησᾰτων", "ὀνησᾰντων";
    aorist_imp_mid_2: "ὀνησαι|ὀνω|ὀνησο", "ὀνησᾰσθον|ὀνᾰσθον|ὀνησθον", "ὀνησᾰσθε|ὀνᾰσθε|ὀνησθε";
    aorist_imp_mid_3: "ὀνησᾰσθω|ὀνᾰσθω|ὀνησθω", "ὀνησᾰσθων|ὀνᾰσθων|ὀνησθων", "ὀνησᾰσθων|ὀνᾰσθων|ὀνησθων";
    aorist_imp_pass_2: "ὀνηθητῐ", _, _;
    aorist_imp_pass_3: "ὀνηθητω", "ὀνηθητων", "ὀνηθεντων";
    infinitive_pres: "ὀνινᾰναι", _, _;
    infinitive_fut: "ὀνησειν", "ὀνησεσθαι", _;
    infinitive_aor: "ὀνησαι", "ὀνησᾰσθαι|ὀνᾰσθαι|ὀνησθαι", "ὀνηθηναι";
    participle_pres: "ὀνινᾱς", _, _;
    participle_fut: "ὀνησων", "ὀνησομενος", _;
    participle_aor: "ὀνησᾱς", "ὀνησᾰμενος|ὀνᾰμενος|ὀνημενος", "ὀνηθεις";
    ),
    // Ἐμπιμπρημι: 1 words (ἐμπιμπρημι). Like Προσερχομαι, except in 258 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἐμπιμπρημι", 1, "~μπιμ~ημι", like Προσερχομαι;
    ind_act_1: "~μπῐμ~ω", _, "~μπῐμ~ωμεν";
    ind_act_2: "~μπῐμ~ᾳς", _, _;
    ind_act_3: "~μπῐμ~ᾳ", _, "~μπῐμ~ωσῐ|~μπῐμ~ωσῐν";
    ind_mid_1: "~μπῐμ~ωμαι", _, "~μπῐμ~ωμεθᾰ";
    ind_mid_2: "~μπῐμ~ᾳ", "~μπῐμ~ᾱσθον", "~μπῐμ~ᾱσθε";
    ind_mid_3: "~μπῐμ~ᾱται", _, "~μπῐμ~ωνται";
    ind_pass_1: _, _, "~μπῐμ~ωμεθᾰ";
    ind_pass_2: _, _, "~μπῐμ~ᾱσθε";
    ind_pass_3: "~μπῐμ~ᾱται", _, "~μπῐμ~ωνται";
    subj_act_1: "~μπῐμ~ω", _, "~μπῐμ~ωμεν";
    subj_act_2: "~μπῐμ~ᾳς", "~μπῐμ~ᾱτον", "~μπῐμ~ᾱτε";
    subj_act_3: "~μπῐμ~ᾳ", _, "~μπῐμ~ωσῐ|~μπῐμ~ωσῐν";
    subj_mid_1: "~μπῐμ~ωμαι", _, "~μπῐμ~ωμεθᾰ";
    subj_mid_2: "~μπῐμ~ᾳ", "~μπῐμ~ᾱσθον", "~μπῐμ~ᾱσθε";
    subj_mid_3: "~μπῐμ~ᾱται", _, "~μπῐμ~ωνται";
    subj_pass_1: _, _, "~μπῐμ~ωμεθᾰ";
    subj_pass_2: _, _, "~μπῐμ~ᾱσθε";
    subj_pass_3: "~μπῐμ~ᾱται", _, "~μπῐμ~ωνται";
    opt_act_1: "~μπῐμ~ῳην|~μπῐμ~ῳμῐ", _, "~μπῐμ~ῳμεν|~μπῐμ~ῳημεν";
    opt_act_2: "~μπῐμ~ῳης|~μπῐμ~ῳς", "~μπῐμ~ῳτον|~μπῐμ~ῳητον", "~μπῐμ~ῳτε|~μπῐμ~ῳητε";
    opt_act_3: "~μπῐμ~ῳη|~μπῐμ~ῳ", "~μπῐμ~ῳτην|~μπῐμ~ῳητην", "~μπῐμ~ῳεν|~μπῐμ~ῳησᾰν";
    opt_mid_1: "~μπῐμ~ῳμην", _, "~μπῐμ~ῳμεθᾰ";
    opt_mid_2: "~μπῐμ~ῳο", "~μπῐμ~ῳσθον", "~μπῐμ~ῳσθε";
    opt_mid_3: "~μπῐμ~ῳτο", "~μπῐμ~ῳσθην", "~μπῐμ~ῳντο";
    opt_pass_1: _, _, "~μπῐμ~ῳμεθᾰ";
    opt_pass_2: _, "~μπῐμ~ῳσθον", "~μπῐμ~ῳσθε";
    opt_pass_3: "~μπῐμ~ῳτο", "~μπῐμ~ῳσθην", "~μπῐμ~ῳντο";
    imp_act_2: "~μπῐμ~ᾱ", "~μπῐμ~ᾱτον", "~μπῐμ~ᾱτε";
    imp_act_3: "~μπῐμ~ᾱτω", "~μπῐμ~ᾱτων", "~μπῐμ~ωντων";
    imp_mid_2: "~μπῐμ~ω", _, "~μπῐμ~ᾱσθε";
    imp_mid_3: "~μπῐμ~ᾱσθω", "~μπῐμ~ᾱσθων", "~μπῐμ~ᾱσθων";
    imp_pass_2: _, _, "~μπῐμ~ᾱσθε";
    imp_pass_3: _, "~μπῐμ~ᾱσθων", _;
    imperfect_act_1: "~νεπῐμ~ων", _, "~νεπῐμ~ωμεν";
    imperfect_act_2: "~νεπῐμ~ᾱς", "~νεπῐμ~ᾱτον", "~νεπῐμ~ᾱτε";
    imperfect_act_3: "~νεπῐμ~ᾱ", "~νεπῐμ~ᾱτην", "~νεπῐμ~ων";
    imperfect_mid_1: "~νεπῐμ~ωμην", _, "~νεπῐμ~ωμεθᾰ";
    imperfect_mid_2: "~νεπῐμ~ω", "~νεπῐμ~ᾱσθον", "~νεπῐμ~ᾱσθε";
    imperfect_mid_3: "~νεπῐμ~ᾱτο", "~νεπῐμ~ᾱσθην", "~νεπῐμ~ωντο";
    imperfect_pass_1: _, _, "~νεπῐμ~ωμεθᾰ";
    imperfect_pass_2: _, _, "~νεπῐμ~ᾱσθε";
    imperfect_pass_3: "~νεπῐμ~ᾱτο", _, _;
    future_ind_act_1: "~μ~ησω|~νῐ~ησω", _, "~μ~ησομεν|~νῐ~ησομεν";
    future_ind_act_2: "~μ~ησεις|~νῐ~ησεις", "~μ~ησετον|~νῐ~ησετον", "~μ~ησετε|~νῐ~ησετε";
    future_ind_act_3: "~μ~ησει|~νῐ~ησει", "~μ~ησετον|~νῐ~ησετον", "~μ~ησουσῐ|~μ~ησουσῐν|~νῐ~ησουσῐ|~νῐ~ησουσῐν";
    future_ind_mid_1: "~μ~ησομαι|~νῐ~ησομαι", _, "~μ~ησομεθᾰ|~νῐ~ησομεθᾰ";
    future_ind_mid_2: "~μ~ησῃ|~μ~ησει|~νῐ~ησῃ|~νῐ~ησει", "~μ~ησεσθον|~νῐ~ησεσθον", "~μ~ησεσθε|~νῐ~ησεσθε";
    future_ind_mid_3: "~μ~ησεται|~νῐ~ησεται", "~μ~ησεσθον|~νῐ~ησεσθον", "~μ~ησονται|~νῐ~ησονται";
    future_ind_pass_1: "~μ%ε~ησομαι", _, "~μ%ε~ησομεθᾰ";
    future_ind_pass_2: "~μ%ε~ησῃ|~μ%ε~ησει", "~μ%ε~ησεσθον", "~μ%ε~ησεσθε";
    future_ind_pass_3: "~μ%ε~ησεται", _, "~μ%ε~ησονται";
    future_opt_act_1: "~μ~ησοιμῐ|~νῐ~ησοιμῐ", _, "~μ~ησοιμεν|~νῐ~ησοιμεν";
    future_opt_act_2: "~μ~ησοις|~νῐ~ησοις", "~μ~ησοιτον|~νῐ~ησοιτον", "~μ~ησοιτε|~νῐ~ησοιτε";
    future_opt_act_3: "~μ~ησοι|~νῐ~ησοι", "~μ~ησοιτην|~νῐ~ησοιτην", "~μ~ησοιεν|~νῐ~ησοιεν";
    future_opt_mid_1: "~μ~ησοιμην|~νῐ~ησοιμην", _, "~μ~ησοιμεθᾰ|~νῐ~ησοιμεθᾰ";
    future_opt_mid_2: "~μ~ησοιο|~νῐ~ησοιο", "~μ~ησοισθον|~νῐ~ησοισθον", "~μ~ησοισθε|~νῐ~ησοισθε";
    future_opt_mid_3: "~μ~ησοιτο|~νῐ~ησοιτο", "~μ~ησοισθην|~νῐ~ησοισθην", "~μ~ησοιντο|~νῐ~ησοιντο";
    future_opt_pass_1: "~μ%ε~ησοιμην", _, "~μ%ε~ησοιμεθᾰ";
    future_opt_pass_2: "~μ%ε~ησοιο", "~μ%ε~ησοισθον", "~μ%ε~ησοισθε";
    future_opt_pass_3: "~μ%ε~ησοιτο", "~μ%ε~ησοισθην", "~μ%ε~ησοιντο";
    aorist_ind_act_1: "~νε~ησᾰ", _, "~νε~ησᾰμεν";
    aorist_ind_act_2: "~νε~ησᾰς", "~νε~ησᾰτον", "~νε~ησᾰτε";
    aorist_ind_act_3: "~νε~ησε|~νε~ησεν", "~νε~ησᾰτην", "~νε~ησᾰν";
    aorist_ind_mid_1: "~νε~ησᾰμην", _, "~νε~ησᾰμεθᾰ";
    aorist_ind_mid_2: "~νε~ησω", "~νε~ησᾰσθον", "~νε~ησᾰσθε";
    aorist_ind_mid_3: "~νε~ησᾰτο", "~νε~ησᾰσθην", "~νε~ησᾰντο";
    aorist_ind_pass_1: "~νε~ησθην", _, "~νε~ησθημεν";
    aorist_ind_pass_2: "~νε~ησθης", "~νε~ησθητον", "~νε~ησθητε";
    aorist_ind_pass_3: "~νε~ησθη", "~νε~ησθητην", "~νε~ησθησᾰν";
    aorist_subj_act_1: "~μ~ησω|~νῐ~ησω", _, "~μ~ησωμεν|~νῐ~ησωμεν";
    aorist_subj_act_2: "~μ~ησῃς|~νῐ~ησῃς", "~μ~ησητον|~νῐ~ησητον", "~μ~ησητε|~νῐ~ησητε";
    aorist_subj_act_3: "~μ~ησῃ|~νῐ~ησῃ", "~μ~ησητον|~νῐ~ησητον", "~μ~ησωσῐ|~μ~ησωσῐν|~νῐ~ησωσῐ|~νῐ~ησωσῐν";
    aorist_subj_mid_1: "~μ~ησωμαι|~νῐ~ησωμαι", _, "~μ~ησωμεθᾰ|~νῐ~ησωμεθᾰ";
    aorist_subj_mid_2: "~μ~ησῃ|~νῐ~ησῃ", "~μ~ησησθον|~νῐ~ησησθον", "~μ~ησησθε|~νῐ~ησησθε";
    aorist_subj_mid_3: "~μ~ησηται|~νῐ~ησηται", "~μ~ησησθον|~νῐ~ησησθον", "~μ~ησωνται|~νῐ~ησωνται";
    aorist_subj_pass_1: "~μ~ησθω|~νῐ~ησθω", _, "~μ~ησθωμεν|~νῐ~ησθωμεν";
    aorist_subj_pass_2: "~μ~ησθῃς|~νῐ~ησθῃς", "~μ~ησθητον|~νῐ~ησθητον", "~μ~ησθητε|~νῐ~ησθητε";
    aorist_subj_pass_3: "~μ~ησθῃ|~νῐ~ησθῃ", "~μ~ησθητον|~νῐ~ησθητον", "~μ~ησθωσῐ|~μ~ησθωσῐν|~νῐ~ησθωσῐ|~νῐ~ησθωσῐν";
    aorist_opt_act_1: "~μ~ησαιμῐ|~νῐ~ησαιμῐ", _, "~μ~ησαιμεν|~νῐ~ησαιμεν";
    aorist_opt_act_2: "~μ~ησειᾰς|~μ~ησαις|~νῐ~ησειᾰς|~νῐ~ησαις", "~μ~ησαιτον|~νῐ~ησαιτον", "~μ~ησαιτε|~νῐ~ησαιτε";
    aorist_opt_act_3: "~μ~ησειε|~μ~ησειεν|~μ~ησαι|~νῐ~ησειε|~νῐ~ησειεν|~νῐ~ησαι", "~μ~ησαιτην|~νῐ~ησαιτην", "~μ~ησειᾰν|~μ~ησαιεν|~νῐ~ησειᾰν|~νῐ~ησαιεν";
    aorist_opt_mid_1: "~μ~ησαιμην|~νῐ~ησαιμην", _, "~μ~ησαιμεθᾰ|~νῐ~ησαιμεθᾰ";
    aorist_opt_mid_2: "~μ~ησαιο|~νῐ~ησαιο", "~μ~ησαισθον|~νῐ~ησαισθον", "~μ~ησαισθε|~νῐ~ησαισθε";
    aorist_opt_mid_3: "~μ~ησαιτο|~νῐ~ησαιτο", "~μ~ησαισθην|~νῐ~ησαισθην", "~μ~ησαιντο|~νῐ~ησαιντο";
    aorist_opt_pass_1: "~μ~ησθειην|~νῐ~ησθειην", _, "~μ~ησθεῑμεν|~μ~ησθειημεν|~νῐ~ησθεῑμεν|~νῐ~ησθειημεν";
    aorist_opt_pass_2: "~μ~ησθειης|~νῐ~ησθειης", "~μ~ησθεῑτον|~μ~ησθειητον|~νῐ~ησθεῑτον|~νῐ~ησθειητον", "~μ~ησθεῑτε|~μ~ησθειητε|~νῐ~ησθεῑτε|~νῐ~ησθειητε";
    aorist_opt_pass_3: "~μ~ησθειη|~νῐ~ησθειη", "~μ~ησθειτην|~μ~ησθειητην|~νῐ~ησθειτην|~νῐ~ησθειητην", "~μ~ησθεῑεν|~μ~ησθειησᾰν|~νῐ~ησθεῑεν|~νῐ~ησθειησᾰν";
    aorist_imp_act_2: "~μ~ησον|~νῐ~ησον", "~μ~ησᾰτον|~νῐ~ησᾰτον", "~μ~ησᾰτε|~νῐ~ησᾰτε";
    aorist_imp_act_3: "~μ~ησᾰτω|~νῐ~ησᾰτω", "~μ~ησᾰτων|~νῐ~ησᾰτων", "~μ~ησᾰντων|~νῐ~ησᾰντων";
    aorist_imp_mid_2: "~μ~ησαι|~νῐ~ησαι", "~μ~ησᾰσθον|~νῐ~ησᾰσθον", "~μ~ησᾰσθε|~νῐ~ησᾰσθε";
    aorist_imp_mid_3: "~μ~ησᾰσθω|~νῐ~ησᾰσθω", "~μ~ησᾰσθων|~νῐ~ησᾰσθων", "~μ~ησᾰσθων|~νῐ~ησᾰσθων";
    aorist_imp_pass_2: "~μ~ησθητῐ|~νῐ~ησθητῐ", "~μ~ησθητον|~νῐ~ησθητον", "~μ~ησθητε|~νῐ~ησθητε";
    aorist_imp_pass_3: "~μ~ησθητω|~νῐ~ησθητω", "~μ~ησθητων|~νῐ~ησθητων", "~μ~ησθεντων|~νῐ~ησθεντων";
    perfect_ind_mid_1: "~μ%ε~ησμαι", _, "~μ%ε~ησμεθᾰ";
    perfect_ind_mid_2: "~μ%ε~ησαι", "~μ%ε~ησθον", "~μ%ε~ησθε";
    perfect_ind_mid_3: "~μ%ε~ησται", _, "~μ%ε~ηδᾰται";
    perfect_ind_pass_1: _, _, "~μ%ε~ησμεθᾰ";
    perfect_ind_pass_2: _, _, "~μ%ε~ησθε";
    perfect_imp_mid_2: "~μ%ε~ησο", _, _;
    perfect_imp_mid_3: "~μ%ε~ησθω", "~μ%ε~ησθων", _;
    perfect_imp_pass_2: "~μ%ε~ησο", _, _;
    perfect_imp_pass_3: "~μ%ε~ησθω", "~μ%ε~ησθων", _;
    infinitive_pres: "~μπῐμ~ᾱν", "~μπῐμ~ᾱσθαι", _;
    infinitive_fut: "~μ~ησειν|~νῐ~ησειν", "~μ~ησεσθαι|~νῐ~ησεσθαι", "~μ%ε~ησεσθαι";
    infinitive_aor: "~μ~ησαι|~νῐ~ησαι", "~μ~ησᾰσθαι|~νῐ~ησᾰσθαι", "~μ~ησθηναι|~νῐ~ησθηναι";
    infinitive_perf: _, "~μ%ε~ησθαι", _;
    participle_pres: "~μπῐμ~ων", "~μπῐμ~ωμενος", _;
    participle_fut: "~μ~ησων|~νῐ~ησων", "~μ~ησομενος|~νῐ~ησομενος", "~μ%ε~ησομενος";
    participle_aor: "~μ~ησᾱς|~νῐ~ησᾱς", "~μ~ησᾰμενος|~νῐ~ησᾰμενος", "~μ~ησθεις|~νῐ~ησθεις";
    participle_perf: _, "~μ%ε~ησμενος", _;
    ),
    // Ἀνθῐστημῐ: 1 words (ἀνθῐστημῐ). Like Μεθῐστημῐ, except in 158 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("ἀνθῐστημῐ", 1, "~θῐ~ημῐ", like Μεθῐστημῐ;
    future_ind_act_1: "~τῐ~ησω", _, "~τῐ~ησομεν";
    future_ind_act_2: "~τῐ~ησεις", "~τῐ~ησετον", "~τῐ~ησετε";
    future_ind_act_3: "~τῐ~ησει", _, "~τῐ~ησουσῐ|~τῐ~ησουσῐν";
    future_ind_mid_1: "~τῐ~ησομαι", _, "~τῐ~ησομεθᾰ";
    future_ind_mid_2: "~τῐ~ησῃ|~τῐ~ησει", "~τῐ~ησεσθον", "~τῐ~ησεσθε";
    future_ind_mid_3: "~τῐ~ησεται", _, "~τῐ~ησονται";
    future_ind_pass_1: "~τῐ~ᾰθησομαι", _, "~τῐ~ᾰθησομεθᾰ";
    future_ind_pass_2: "~τῐ~ᾰθησῃ|~τῐ~ᾰθησει", "~τῐ~ᾰθησεσθον", "~τῐ~ᾰθησεσθε";
    future_ind_pass_3: "~τῐ~ᾰθησεται", _, "~τῐ~ᾰθησονται";
    future_opt_act_1: "~τῐ~ησοιμῐ", _, "~τῐ~ησοιμεν";
    future_opt_act_2: "~τῐ~ησοις", "~τῐ~ησοιτον", "~τῐ~ησοιτε";
    future_opt_act_3: "~τῐ~ησοι", "~τῐ~ησοιτην", "~τῐ~ησοιεν";
    future_opt_mid_1: "~τῐ~ησοιμην", _, "~τῐ~ησοιμεθᾰ";
    future_opt_mid_2: "~τῐ~ησοιο", "~τῐ~ησοισθον", "~τῐ~ησοισθε";
    future_opt_mid_3: "~τῐ~ησοιτο", "~τῐ~ησοισθην", "~τῐ~ησοιντο";
    future_opt_pass_1: "~τῐ~ᾰθησοιμην", _, "~τῐ~ᾰθησοιμεθᾰ";
    future_opt_pass_2: "~τῐ~ᾰθησοιο", "~τῐ~ᾰθησοισθον", "~τῐ~ᾰθησοισθε";
    future_opt_pass_3: "~τῐ~ᾰθησοιτο", "~τῐ~ᾰθησοισθην", "~τῐ~ᾰθησοιντο";
    aorist_subj_act_1: "~τῐ~ησω|~τῐ~ω", _, "~τῐ~ησωμεν|~τῐ~ωμεν";
    aorist_subj_act_2: "~τῐ~ησῃς|~τῐ~ῃς", "~τῐ~ησητον|~τῐ~ητον", "~τῐ~ησητε|~τῐ~ητε";
    aorist_subj_act_3: "~τῐ~ησῃ|~τῐ~ῃ", "~τῐ~ησητον|~τῐ~ητον", "~τῐ~ησωσῐ|~τῐ~ησωσῐν|~τῐ~ωσῐ|~τῐ~ωσῐν";
    aorist_subj_mid_1: "~τῐ~ησωμαι|~τῐ~ωμαι", _, "~τῐ~ησωμεθᾰ|~τῐ~ωμεθᾰ";
    aorist_subj_mid_2: "~τῐ~ησῃ|~τῐ~ῃ", "~τῐ~ησησθον|~τῐ~ησθον", "~τῐ~ησησθε|~τῐ~ησθε";
    aorist_subj_mid_3: "~τῐ~ησηται|~τῐ~ηται", "~τῐ~ησησθον|~τῐ~ησθον", "~τῐ~ησωνται|~τῐ~ωνται";
    aorist_subj_pass_1: "~τῐ~ᾰθω", _, "~τῐ~ᾰθωμεν";
    aorist_subj_pass_2: "~τῐ~ᾰθῃς", "~τῐ~ᾰθητον", "~τῐ~ᾰθητε";
    aorist_subj_pass_3: "~τῐ~ᾰθῃ", _, "~τῐ~ᾰθωσῐ|~τῐ~ᾰθωσῐν";
    aorist_opt_act_1: "~τῐ~ησαιμῐ|~τῐ~αιην", _, "~τῐ~ησαιμεν|~τῐ~αῑμεν|~τῐ~αιημεν";
    aorist_opt_act_2: "~τῐ~ησειᾰς|~τῐ~ησαις|~τῐ~αιης", "~τῐ~ησαιτον|~τῐ~αῑτον|~τῐ~αιητον", "~τῐ~ησαιτε|~τῐ~αῑτε|~τῐ~αιητε";
    aorist_opt_act_3: "~τῐ~ησειε|~τῐ~ησειεν|~τῐ~ησαι|~τῐ~αιη", "~τῐ~ησαιτην|~τῐ~αιτην|~τῐ~αιητην", "~τῐ~ησειᾰν|~τῐ~ησαιεν|~τῐ~αῑεν|~τῐ~αιησᾰν";
    aorist_opt_mid_1: "~τῐ~ησαιμην|~τῐ~αιμην", _, "~τῐ~ησαιμεθᾰ|~τῐ~αιμεθᾰ";
    aorist_opt_mid_2: "~τῐ~ησαιο|~τῐ~αῑο", "~τῐ~ησαισθον|~τῐ~αῑσθον", "~τῐ~ησαισθε|~τῐ~αῑσθε";
    aorist_opt_mid_3: "~τῐ~ησαιτο|~τῐ~αῑτο", "~τῐ~ησαισθην|~τῐ~αισθην", "~τῐ~ησαιντο|~τῐ~αῑντο";
    aorist_opt_pass_1: "~τῐ~ᾰθειην", _, "~τῐ~ᾰθεῑμεν|~τῐ~ᾰθειημεν";
    aorist_opt_pass_2: "~τῐ~ᾰθειης", "~τῐ~ᾰθεῑτον|~τῐ~ᾰθειητον", "~τῐ~ᾰθεῑτε|~τῐ~ᾰθειητε";
    aorist_opt_pass_3: "~τῐ~ᾰθειη", "~τῐ~ᾰθειτην|~τῐ~ᾰθειητην", "~τῐ~ᾰθεῑεν|~τῐ~ᾰθειησᾰν";
    aorist_imp_act_2: "~τῐ~ησον|~τῐ~ηθῐ", "~τῐ~ησᾰτον|~τῐ~ητον", "~τῐ~ησᾰτε|~τῐ~ητε";
    aorist_imp_act_3: "~τῐ~ησᾰτω|~τῐ~ητω", "~τῐ~ησᾰτων|~τῐ~ητων", "~τῐ~ησᾰντων|~τῐ~ᾰντων";
    aorist_imp_mid_2: "~τῐ~ησαι|~τῐ~ω", "~τῐ~ησᾰσθον|~τῐ~ᾰσθον", "~τῐ~ησᾰσθε|~τῐ~ᾰσθε";
    aorist_imp_mid_3: "~τῐ~ησᾰσθω|~τῐ~ᾰσθω", "~τῐ~ησᾰσθων|~τῐ~ᾰσθων", "~τῐ~ησᾰσθων|~τῐ~ᾰσθων";
    aorist_imp_pass_2: "~τῐ~ᾰθητῐ", _, _;
    aorist_imp_pass_3: "~τῐ~ᾰθητω", "~τῐ~ᾰθητων", "~τῐ~ᾰθεντων";
    perfect_ind_act_1: "~θε~ᾰκᾰ", _, "~θε~ᾰκᾰμεν";
    perfect_ind_act_2: "~θε~ᾰκᾰς", "~θε~ᾰκᾰτον", "~θε~ᾰκᾰτε";
    perfect_ind_act_3: "~θε~ᾰκε|~θε~ᾰκεν", _, "~θε~ᾰκᾱσῐ|~θε~ᾰκᾱσῐν";
    perfect_subj_act_1: "~θε~ᾰκω", _, "~θε~ᾰκωμεν";
    perfect_subj_act_2: "~θε~ᾰκῃς", "~θε~ᾰκητον", "~θε~ᾰκητε";
    perfect_subj_act_3: "~θε~ᾰκῃ", _, "~θε~ᾰκωσῐ|~θε~ᾰκωσῐν";
    perfect_opt_act_1: "~θε~ᾰκοιμῐ|~θε~ᾰκοιην", _, "~θε~ᾰκοιμεν";
    perfect_opt_act_2: "~θε~ᾰκοις|~θε~ᾰκοιης", "~θε~ᾰκοιτον", "~θε~ᾰκοιτε";
    perfect_opt_act_3: "~θε~ᾰκοι|~θε~ᾰκοιη", "~θε~ᾰκοιτην", "~θε~ᾰκοιεν";
    perfect_imp_act_2: "~θε~ᾰκε", "~θε~ᾰκετον", "~θε~ᾰκετε";
    perfect_imp_act_3: "~θε~ᾰκετω", "~θε~ᾰκετων", "~θε~ᾰκοντων";
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    infinitive_fut: "~τῐ~ησειν", "~τῐ~ησεσθαι", "~τῐ~ᾰθησεσθαι";
    infinitive_aor: "~τῐ~ησαι|~τῐ~ηναι", "~τῐ~ησᾰσθαι|~τῐ~ᾰσθαι", "~τῐ~ᾰθηναι";
    infinitive_perf: "~θε~ᾰκεναι", _, _;
    participle_fut: "~τῐ~ησων", "~τῐ~ησομενος", "~τῐ~ᾰθησομενος";
    participle_aor: "~τῐ~ησᾱς|~τῐ~ᾱς", "~τῐ~ησᾰμενος|~τῐ~ᾰμενος", "~τῐ~ᾰθεις";
    participle_perf: "~θε~ᾰκως", _, _;
    ),
    // Περῐῐστημῐ: 1 words (περῐῐστημῐ). Like Ἀνῐστημῐ, except in 195 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("περῐῐστημῐ", 1, "~ῐ~ημῐ", like Ἀνῐστημῐ;
    future_ind_act_1: "~~ησω", _, "~~ησομεν";
    future_ind_act_2: "~~ησεις", "~~ησετον", "~~ησετε";
    future_ind_act_3: "~~ησει", _, "~~ησουσῐ|~~ησουσῐν";
    future_ind_mid_1: "~~ησομαι", _, "~~ησομεθᾰ";
    future_ind_mid_2: "~~ησῃ|~~ησει", "~~ησεσθον", "~~ησεσθε";
    future_ind_mid_3: "~~ησεται", _, "~~ησονται";
    future_ind_pass_1: "~~ησομαι", _, "~~ησομεθᾰ";
    future_ind_pass_2: "~~ησῃ|~~ησει", "~~ησεσθον", "~~ησεσθε";
    future_ind_pass_3: "~~ησεται", _, "~~ησονται";
    future_opt_act_1: "~~ησοιμῐ", _, "~~ησοιμεν";
    future_opt_act_2: "~~ησοις", "~~ησοιτον", "~~ησοιτε";
    future_opt_act_3: "~~ησοι", "~~ησοιτην", "~~ησοιεν";
    future_opt_mid_1: "~~ησοιμην", _, "~~ησοιμεθᾰ";
    future_opt_mid_2: "~~ησοιο", "~~ησοισθον", "~~ησοισθε";
    future_opt_mid_3: "~~ησοιτο", "~~ησοισθην", "~~ησοιντο";
    future_opt_pass_1: "~~ησοιμην", _, "~~ησοιμεθᾰ";
    future_opt_pass_2: "~~ησοιο", "~~ησοισθον", "~~ησοισθε";
    future_opt_pass_3: "~~ησοιτο", "~~ησοισθην", "~~ησοιντο";
    aorist_subj_act_1: "~~ησω|~~ω", _, "~~ησωμεν|~~ωμεν";
    aorist_subj_act_2: "~~ησῃς|~~ῃς", "~~ησητον|~~ητον", "~~ησητε|~~ητε";
    aorist_subj_act_3: "~~ησῃ|~~ῃ", "~~ησητον|~~ητον", "~~ησωσῐ|~~ησωσῐν|~~ωσῐ|~~ωσῐν";
    aorist_subj_mid_1: "~~ησωμαι|~~ωμαι", _, "~~ησωμεθᾰ|~~ωμεθᾰ";
    aorist_subj_mid_2: "~~ησῃ|~~ῃ", "~~ησησθον|~~ησθον", "~~ησησθε|~~ησθε";
    aorist_subj_mid_3: "~~ησηται|~~ηται", "~~ησησθον|~~ησθον", "~~ησωνται|~~ωνται";
    aorist_subj_pass_1: "~~ᾰθω", _, "~~ᾰθωμεν";
    aorist_subj_pass_2: "~~ᾰθῃς", "~~ᾰθητον", "~~ᾰθητε";
    aorist_subj_pass_3: "~~ᾰθῃ", _, "~~ᾰθωσῐ|~~ᾰθωσῐν";
    aorist_opt_act_1: "~~ησαιμῐ|~~αιην", _, "~~ησαιμεν|~~αῑμεν|~~αιημεν";
    aorist_opt_act_2: "~~ησειᾰς|~~ησαις|~~αιης", "~~ησαιτον|~~αῑτον|~~αιητον", "~~ησαιτε|~~αῑτε|~~αιητε";
    aorist_opt_act_3: "~~ησειε|~~ησειεν|~~ησαι|~~αιη", "~~ησαιτην|~~αιτην|~~αιητην", "~~ησειᾰν|~~ησαιεν|~~αῑεν|~~αιησᾰν";
    aorist_opt_mid_1: "~~ησαιμην|~~αιμην", _, "~~ησαιμεθᾰ|~~αιμεθᾰ";
    aorist_opt_mid_2: "~~ησαιο|~~αῑο", "~~ησαισθον|~~αῑσθον", "~~ησαισθε|~~αῑσθε";
    aorist_opt_mid_3: "~~ησαιτο|~~αῑτο", "~~ησαισθην|~~αισθην", "~~ησαιντο|~~αῑντο";
    aorist_opt_pass_1: "~~ᾰθειην", _, "~~ᾰθεῑμεν|~~ᾰθειημεν";
    aorist_opt_pass_2: "~~ᾰθειης", "~~ᾰθεῑτον|~~ᾰθειητον", "~~ᾰθεῑτε|~~ᾰθειητε";
    aorist_opt_pass_3: "~~ᾰθειη", "~~ᾰθειτην|~~ᾰθειητην", "~~ᾰθεῑεν|~~ᾰθειησᾰν";
    aorist_imp_act_2: "~~ησον|~~ηθῐ", "~~ησᾰτον|~~ητον", "~~ησᾰτε|~~ητε";
    aorist_imp_act_3: "~~ησᾰτω|~~ητω", "~~ησᾰτων|~~ητων", "~~ησᾰντων|~~ᾰντων";
    aorist_imp_mid_2: "~~ησαι|~~ω", "~~ησᾰσθον|~~ᾰσθον", "~~ησᾰσθε|~~ᾰσθε";
    aorist_imp_mid_3: "~~ησᾰσθω|~~ᾰσθω", "~~ησᾰσθων|~~ᾰσθων", "~~ησᾰσθων|~~ᾰσθων";
    aorist_imp_pass_2: "~~ᾰθητῐ", _, _;
    aorist_imp_pass_3: "~~ᾰθητω", "~~ᾰθητων", "~~ᾰθεντων";
    perfect_ind_act_1: "~ε~ηκᾰ|~ε~ᾰκᾰ", _, "~ε~ηκᾰμεν|~ε~ᾰκᾰμεν";
    perfect_ind_act_2: "~ε~ηκᾰς|~ε~ᾰκᾰς", "~ε~ηκᾰτον|~ε~ᾰκᾰτον", "~ε~ηκᾰτε|~ε~ᾰκᾰτε";
    perfect_ind_act_3: "~ε~ηκε|~ε~ηκεν|~ε~ᾰκε|~ε~ᾰκεν", "~ε~ηκᾰτον|~ε~ᾰκᾰτον", "~ε~ηκᾱσῐ|~ε~ηκᾱσῐν|~ε~ᾰκᾱσῐ|~ε~ᾰκᾱσῐν";
    perfect_ind_mid_1: "~ε~ημαι|~ε~ᾰμαι", _, "~ε~ημεθᾰ|~ε~ᾰμεθᾰ";
    perfect_ind_mid_2: "~ε~ησαι|~ε~ᾰσαι", "~ε~ησθον|~ε~ᾰσθον", "~ε~ησθε|~ε~ᾰσθε";
    perfect_ind_mid_3: "~ε~ηται|~ε~ᾰται", "~ε~ησθον|~ε~ᾰσθον", "~ε~ηνται|~ε~ᾰνται";
    perfect_ind_pass_1: "~ε~ημαι|~ε~ᾰμαι", _, "~ε~ημεθᾰ|~ε~ᾰμεθᾰ";
    perfect_ind_pass_2: "~ε~ησαι|~ε~ᾰσαι", "~ε~ησθον|~ε~ᾰσθον", "~ε~ησθε|~ε~ᾰσθε";
    perfect_ind_pass_3: "~ε~ηται|~ε~ᾰται", "~ε~ησθον|~ε~ᾰσθον", "~ε~ηνται|~ε~ᾰνται";
    perfect_subj_act_1: "~ε~ηκω|~ε~ᾰκω", _, "~ε~ηκωμεν|~ε~ᾰκωμεν";
    perfect_subj_act_2: "~ε~ηκῃς|~ε~ᾰκῃς", "~ε~ηκητον|~ε~ᾰκητον", "~ε~ηκητε|~ε~ᾰκητε";
    perfect_subj_act_3: "~ε~ηκῃ|~ε~ᾰκῃ", "~ε~ηκητον|~ε~ᾰκητον", "~ε~ηκωσῐ|~ε~ηκωσῐν|~ε~ᾰκωσῐ|~ε~ᾰκωσῐν";
    perfect_opt_act_1: "~ε~ηκοιμῐ|~ε~ηκοιην|~ε~ᾰκοιμῐ|~ε~ᾰκοιην", _, "~ε~ηκοιμεν|~ε~ᾰκοιμεν";
    perfect_opt_act_2: "~ε~ηκοις|~ε~ηκοιης|~ε~ᾰκοις|~ε~ᾰκοιης", "~ε~ηκοιτον|~ε~ᾰκοιτον", "~ε~ηκοιτε|~ε~ᾰκοιτε";
    perfect_opt_act_3: "~ε~ηκοι|~ε~ηκοιη|~ε~ᾰκοι|~ε~ᾰκοιη", "~ε~ηκοιτην|~ε~ᾰκοιτην", "~ε~ηκοιεν|~ε~ᾰκοιεν";
    perfect_imp_act_2: "~ε~ηκε|~ε~ᾰκε", "~ε~ηκετον|~ε~ᾰκετον", "~ε~ηκετε|~ε~ᾰκετε";
    perfect_imp_act_3: "~ε~ηκετω|~ε~ᾰκετω", "~ε~ηκετων|~ε~ᾰκετων", "~ε~ηκοντων|~ε~ᾰκοντων";
    perfect_imp_mid_2: "~ε~ησο|~ε~ᾰσο", "~ε~ησθον|~ε~ᾰσθον", "~ε~ησθε|~ε~ᾰσθε";
    perfect_imp_mid_3: "~ε~ησθω|~ε~ᾰσθω", "~ε~ησθων|~ε~ᾰσθων", "~ε~ησθων|~ε~ᾰσθων";
    perfect_imp_pass_2: "~ε~ησο|~ε~ᾰσο", "~ε~ησθον|~ε~ᾰσθον", "~ε~ησθε|~ε~ᾰσθε";
    perfect_imp_pass_3: "~ε~ησθω|~ε~ᾰσθω", "~ε~ησθων|~ε~ᾰσθων", "~ε~ησθων|~ε~ᾰσθων";
    pluperfect_mid_1: "~ε~ημην", _, "~ε~ημεθᾰ";
    pluperfect_mid_2: "~ε~ησο", "~ε~ησθον", "~ε~ησθε";
    pluperfect_mid_3: "~ε~ητο", "~ε~ησθην", _;
    pluperfect_pass_1: _, _, "~ε~ημεθᾰ";
    pluperfect_pass_2: _, _, "~ε~ησθε";
    pluperfect_pass_3: _, "~ε~ησθην", "~ε~ηντο";
    infinitive_fut: "~~ησειν", "~~ησεσθαι", "~~ησεσθαι";
    infinitive_aor: "~~ησαι|~~ηναι", "~~ησᾰσθαι|~~ᾰσθαι", "~~ᾰθηναι";
    infinitive_perf: "~ε~ηκεναι|~ε~ᾰκεναι", "~ε~ησθαι|~ε~ᾰσθαι", "~ε~ησθαι|~ε~ᾰσθαι";
    participle_fut: "~~ησων", "~~ησομενος", "~~ησομενος";
    participle_aor: "~~ησᾱς|~~ᾱς", "~~ησᾰμενος|~~ᾰμενος", "~~ᾰθεις";
    participle_perf: "~ε~ηκως|~ε~ᾰκως", "~ε~ημενος|~ε~ᾰμενος", "~ε~ημενος|~ε~ᾰμενος";
    ),
    // Ἐπορνῡμῐ: 1 words (ἐπορνῡμῐ). Like Σῠνομνῡμῐ, except in 176 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("ἐπορνῡμῐ", 1, "~ο~νῡμῐ", like Σῠνομνῡμῐ;
    future_ind_act_1: "~ο~σω", _, "~ο~σομεν";
    future_ind_act_2: "~ο~σεις", "~ο~σετον", "~ο~σετε";
    future_ind_act_3: "~ο~σει", _, "~ο~σουσῐ|~ο~σουσῐν";
    future_ind_mid_1: "~ο~σομαι", _, "~ο~σομεθᾰ";
    future_ind_mid_2: "~ο~σῃ|~ο~σει", "~ο~σεσθον", "~ο~σεσθε";
    future_ind_mid_3: "~ο~σεται", _, "~ο~σονται";
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_act_1: "~ο~σοιμῐ", _, "~ο~σοιμεν";
    future_opt_act_2: "~ο~σοις", "~ο~σοιτον", "~ο~σοιτε";
    future_opt_act_3: "~ο~σοι", "~ο~σοιτην", "~ο~σοιεν";
    future_opt_mid_1: "~ο~σοιμην", _, "~ο~σοιμεθᾰ";
    future_opt_mid_2: "~ο~σοιο", "~ο~σοισθον", "~ο~σοισθε";
    future_opt_mid_3: "~ο~σοιτο", "~ο~σοισθην", "~ο~σοιντο";
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_act_1: "~ω~σᾰ", _, "~ω~σᾰμεν";
    aorist_ind_act_2: "~ω~σᾰς", "~ω~σᾰτον", "~ω~σᾰτε";
    aorist_ind_act_3: "~ω~σε|~ω~σεν", "~ω~σᾰτην", "~ω~σᾰν";
    aorist_ind_mid_1: "~ω~σᾰμην|~ω~ομην", _, "~ω~σᾰμεθᾰ|~ω~ομεθᾰ";
    aorist_ind_mid_2: "~ω~σω|~ω~ου", "~ω~σᾰσθον|~ω~εσθον", "~ω~σᾰσθε|~ω~εσθε";
    aorist_ind_mid_3: "~ω~σᾰτο|~ω~το", "~ω~σᾰσθην|~ω~εσθην", "~ω~σᾰντο|~ω~οντο";
    aorist_ind_pass_1: derived, _, derived;
    aorist_ind_pass_2: derived, derived, derived;
    aorist_ind_pass_3: derived, derived, derived;
    aorist_subj_act_1: "~ο~σω", _, "~ο~σωμεν";
    aorist_subj_act_2: "~ο~σῃς", "~ο~σητον", "~ο~σητε";
    aorist_subj_act_3: "~ο~σῃ", _, "~ο~σωσῐ|~ο~σωσῐν";
    aorist_subj_mid_1: "~ο~σωμαι|~ο~ωμαι", _, "~ο~σωμεθᾰ|~ο~ωμεθᾰ";
    aorist_subj_mid_2: "~ο~σῃ|~ο~ῃ", "~ο~σησθον|~ο~ησθον", "~ο~σησθε|~ο~ησθε";
    aorist_subj_mid_3: "~ο~σηται|~ο~ηται", "~ο~σησθον|~ο~ησθον", "~ο~σωνται|~ο~ωνται";
    aorist_subj_pass_1: derived, _, derived;
    aorist_subj_pass_2: derived, derived, derived;
    aorist_subj_pass_3: derived, derived, derived;
    aorist_opt_act_1: "~ο~σαιμῐ", _, "~ο~σαιμεν";
    aorist_opt_act_2: "~ο~σειᾰς|~ο~σαις", "~ο~σαιτον", "~ο~σαιτε";
    aorist_opt_act_3: "~ο~σειε|~ο~σειεν|~ο~σαι", "~ο~σαιτην", "~ο~σειᾰν|~ο~σαιεν";
    aorist_opt_mid_1: "~ο~σαιμην|~ο~οιμην", _, "~ο~σαιμεθᾰ|~ο~οιμεθᾰ";
    aorist_opt_mid_2: "~ο~σαιο|~ο~οιο", "~ο~σαισθον|~ο~οισθον", "~ο~σαισθε|~ο~οισθε";
    aorist_opt_mid_3: "~ο~σαιτο|~ο~οιτο", "~ο~σαισθην|~ο~οισθην", "~ο~σαιντο|~ο~οιντο";
    aorist_opt_pass_1: derived, _, derived;
    aorist_opt_pass_2: derived, derived, derived;
    aorist_opt_pass_3: derived, derived, derived;
    aorist_imp_act_2: "~ο~σον", "~ο~σᾰτον", "~ο~σᾰτε";
    aorist_imp_act_3: "~ο~σᾰτω", "~ο~σᾰτων", "~ο~σᾰντων";
    aorist_imp_mid_2: "~ο~σαι|~ο~οῡ", "~ο~σᾰσθον|~ο~εσθον", "~ο~σᾰσθε|~ο~εσθε";
    aorist_imp_mid_3: "~ο~σᾰσθω|~ο~εσθω", "~ο~σᾰσθων|~ο~εσθων", "~ο~σᾰσθων|~ο~εσθων";
    aorist_imp_pass_2: derived, derived, derived;
    aorist_imp_pass_3: derived, derived, derived;
    perfect_ind_act_1: "~ο~ωρᾰ", _, "~ο~ωρᾰμεν";
    perfect_ind_act_2: "~ο~ωρᾰς", "~ο~ωρᾰτον", "~ο~ωρᾰτε";
    perfect_ind_act_3: "~ο~ωρε|~ο~ωρεν", _, "~ο~ωρᾱσῐ|~ο~ωρᾱσῐν";
    perfect_subj_act_1: "~ο~ωρω", _, "~ο~ωρωμεν";
    perfect_subj_act_2: "~ο~ωρῃς", "~ο~ωρητον", "~ο~ωρητε";
    perfect_subj_act_3: "~ο~ωρῃ", _, "~ο~ωρωσῐ|~ο~ωρωσῐν";
    perfect_opt_act_1: "~ο~ωροιμῐ|~ο~ωροιην", _, "~ο~ωροιμεν";
    perfect_opt_act_2: "~ο~ωροις|~ο~ωροιης", "~ο~ωροιτον", "~ο~ωροιτε";
    perfect_opt_act_3: "~ο~ωροι|~ο~ωροιη", "~ο~ωροιτην", "~ο~ωροιεν";
    perfect_imp_act_2: "~ο~ωρε", "~ο~ωρετον", "~ο~ωρετε";
    perfect_imp_act_3: "~ο~ωρετω", "~ο~ωρετων", "~ο~ωροντων";
    infinitive_fut: "~ο~σειν", "~ο~σεσθαι", derived;
    infinitive_aor: "~ο~σαι", "~ο~σᾰσθαι|~ο~εσθαι", derived;
    infinitive_perf: "~ο~ωρεναι", _, _;
    participle_fut: "~ο~σων", "~ο~σομενος", derived;
    participle_aor: "~ο~σᾱς", "~ο~σᾰμενος|~ο~ομενος", derived;
    participle_perf: "~ο~ωρως", _, _;
    ),
    // Ἐξεστῐ: 1 words (ἐξεστῐ). Like Ἠμῐ, except in 8 cells: ind_act_1 sg, ind_act_3 sg, subj_act_3 sg, opt_act_3 sg, imp_act_3 sg, imperfect_act_1 sg, ….
    conjugation!("ἐξεστῐ", 1, "~εστῐ", like Ἠμῐ;
    ind_act_1: "~εστῐ", _, _;
    ind_act_3: "~εστῐ|~εστῐν", _, _;
    subj_act_3: "~ῃ", _, _;
    opt_act_3: "~ειη", _, _;
    imp_act_3: "~εστω", _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    infinitive_pres: "~εῑναι", _, _;
    ),
    // Ὑφῐστημῐ: 1 words (ὑφῐστημῐ). Like Ἀφῐστημῐ, except in 110 cells: future_ind_pass_1 sg, future_ind_pass_1 pl, future_ind_pass_2 sg, future_ind_pass_2 du, future_ind_pass_2 pl, future_ind_pass_3 sg, ….
    conjugation!("ὑφῐστημῐ", 1, "~φῐ~ημῐ", like Ἀφῐστημῐ;
    future_ind_pass_1: "~πο~ᾰθησομαι", _, "~πο~ᾰθησομεθᾰ";
    future_ind_pass_2: "~πο~ᾰθησῃ|~πο~ᾰθησει", "~πο~ᾰθησεσθον", "~πο~ᾰθησεσθε";
    future_ind_pass_3: "~πο~ᾰθησεται", _, "~πο~ᾰθησονται";
    future_opt_pass_1: "~πο~ᾰθησοιμην", _, "~πο~ᾰθησοιμεθᾰ";
    future_opt_pass_2: "~πο~ᾰθησοιο", "~πο~ᾰθησοισθον", "~πο~ᾰθησοισθε";
    future_opt_pass_3: "~πο~ᾰθησοιτο", "~πο~ᾰθησοισθην", "~πο~ᾰθησοιντο";
    aorist_ind_act_3: _, _, "~πε~ησᾰν";
    perfect_ind_act_1: "~φε~ᾰκᾰ", _, "~φε~ᾰκᾰμεν";
    perfect_ind_act_2: "~φε~ᾰκᾰς", "~φε~ᾰκᾰτον", "~φε~ᾰκᾰτε";
    perfect_ind_act_3: "~φε~ᾰκε|~φε~ᾰκεν", derived, "~φε~ᾰκᾱσῐ|~φε~ᾰκᾱσῐν";
    perfect_ind_mid_1: derived, _, derived;
    perfect_ind_mid_2: derived, derived, derived;
    perfect_ind_mid_3: derived, _, derived;
    perfect_ind_pass_1: _, _, derived;
    perfect_ind_pass_2: _, _, derived;
    perfect_subj_act_1: "~φε~ᾰκω", _, "~φε~ᾰκωμεν";
    perfect_subj_act_2: "~φε~ᾰκῃς", "~φε~ᾰκητον", "~φε~ᾰκητε";
    perfect_subj_act_3: "~φε~ᾰκῃ", derived, "~φε~ᾰκωσῐ|~φε~ᾰκωσῐν";
    perfect_opt_act_1: "~φε~ᾰκοιμῐ|~φε~ᾰκοιην", _, "~φε~ᾰκοιμεν";
    perfect_opt_act_2: "~φε~ᾰκοις|~φε~ᾰκοιης", "~φε~ᾰκοιτον", "~φε~ᾰκοιτε";
    perfect_opt_act_3: "~φε~ᾰκοι|~φε~ᾰκοιη", "~φε~ᾰκοιτην", "~φε~ᾰκοιεν";
    perfect_imp_act_2: "~φε~ᾰκε", "~φε~ᾰκετον", "~φε~ᾰκετε";
    perfect_imp_act_3: "~φε~ᾰκετω", "~φε~ᾰκετων", "~φε~ᾰκοντων";
    perfect_imp_mid_2: derived, _, _;
    perfect_imp_mid_3: derived, derived, _;
    perfect_imp_pass_2: derived, _, _;
    perfect_imp_pass_3: derived, derived, _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    future_perfect_ind_act_1: derived, _, derived;
    future_perfect_ind_act_2: derived, derived, derived;
    future_perfect_ind_act_3: derived, _, derived;
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_ind_pass_2: derived, _, _;
    future_perfect_opt_act_1: derived, _, derived;
    future_perfect_opt_act_2: derived, derived, derived;
    future_perfect_opt_act_3: derived, derived, derived;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_fut: _, _, "~πο~ᾰθησεσθαι";
    infinitive_perf: "~φε~ᾰκεναι", derived, _;
    infinitive_futp: derived, derived, _;
    participle_fut: _, _, "~πο~ᾰθησομενος";
    participle_perf: "~φε~ᾰκως", derived, _;
    participle_futp: derived, derived, _;
    ),
    // Γεγονοι: 1 words (γεγονοι). Like Κᾰθημαι, except in 64 cells: ind_act_1 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, perfect_ind_act_2 sg, perfect_ind_act_2 du, perfect_ind_act_2 pl, ….
    conjugation!("γεγονοι", 1, "~ο~οι", like Κᾰθημαι;
    ind_act_1: "~ο~οι", _, _;
    perfect_ind_act_1: "~ο~ᾰ", _, "~ο~ᾰμεν";
    perfect_ind_act_2: "~ο~ᾰς", "~ο~ᾰτον", "~ο~ᾰτε";
    perfect_ind_act_3: "~ο~ε|~ο~εν", _, "~ο~ᾱσῐ|~ο~ᾱσῐν";
    perfect_ind_mid_1: "~ε~ημαι", _, "~ε~ημεθᾰ";
    perfect_ind_mid_2: "~ε~ησαι", "~ε~ησθον", "~ε~ησθε";
    perfect_ind_mid_3: "~ε~ηται", _, "~ε~ηνται";
    perfect_ind_pass_1: _, _, "~ε~ημεθᾰ";
    perfect_ind_pass_2: _, _, "~ε~ησθε";
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
    perfect_imp_pass_2: "~ε~ησο", _, _;
    perfect_imp_pass_3: "~ε~ησθω", "~ε~ησθων", _;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    pluperfect_pass_1: derived, _, derived;
    pluperfect_pass_2: derived, derived, derived;
    pluperfect_pass_3: derived, derived, derived;
    infinitive_perf: "~ο~εναι", "~ε~ησθαι", _;
    participle_perf: "~ο~ως", "~ε~ημενος", _;
    ),
    // Γεγενησαι: 1 words (γεγενησαι). Like Γεγονοι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησαι", 1, "~ε~ησαι", like Γεγονοι;
    ind_act_1: "~ε~ησαι", _, _;
    ),
    // Γεγενηται: 1 words (γεγενηται). Like Γεγονοι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενηται", 1, "~ε~ηται", like Γεγονοι;
    ind_act_1: "~ε~ηται", _, _;
    ),
    // Γεγενησο: 1 words (γεγενησο). Like Ἠμῐ, except in 51 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, ….
    conjugation!("γεγενησο", 1, "~ενησο", like Ἠμῐ;
    ind_act_1: "~ενησο", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    perfect_ind_act_1: "~ονᾰ", _, "~ονᾰμεν";
    perfect_ind_act_2: "~ονᾰς", "~ονᾰτον", "~ονᾰτε";
    perfect_ind_act_3: "~ονε|~ονεν", _, "~ονᾱσῐ|~ονᾱσῐν";
    perfect_ind_mid_1: "~ενημαι", _, "~ενημεθᾰ";
    perfect_ind_mid_2: "~ενησαι", "~ενησθον", "~ενησθε";
    perfect_ind_mid_3: "~ενηται", _, "~ενηνται";
    perfect_ind_pass_1: _, _, "~ενημεθᾰ";
    perfect_ind_pass_2: _, _, "~ενησθε";
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
    perfect_imp_pass_2: "~ενησο", _, _;
    perfect_imp_pass_3: "~ενησθω", "~ενησθων", _;
    infinitive_perf: "~ονεναι", "~ενησθαι", _;
    participle_perf: "~ονως", "~ενημενος", _;
    ),
    // Γεγενησθον: 1 words (γεγενησθον). Like Γεγονοι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησθον", 1, "~ε~ησθον", like Γεγονοι;
    ind_act_1: "~ε~ησθον", _, _;
    ),
    // Γεγενησθων: 1 words (γεγενησθων). Like Γεγονοι, except in 1 cell: ind_act_1 sg.
    conjugation!("γεγενησθων", 1, "~ε~ησθων", like Γεγονοι;
    ind_act_1: "~ε~ησθων", _, _;
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
    // Τεθνηκας: 1 words (τεθνηκας). Like Ἠμῐ, except in 34 cells: ind_act_1 sg, ind_act_3 sg, imperfect_act_1 sg, imperfect_act_3 sg, perfect_ind_act_1 sg, perfect_ind_act_1 pl, ….
    conjugation!("τεθνηκας", 1, "~ας", like Ἠμῐ;
    ind_act_1: "~ας", _, _;
    ind_act_3: derived, _, _;
    imperfect_act_1: derived, _, _;
    imperfect_act_3: derived, _, _;
    perfect_ind_act_1: "~ᾰ", _, "~ᾰμεν";
    perfect_ind_act_2: "~ᾰς", "~ᾰτον", "~ᾰτε";
    perfect_ind_act_3: "~ε|~εν", _, "~ᾱσῐ|~ᾱσῐν";
    perfect_subj_act_1: "~ω", _, "~ωμεν";
    perfect_subj_act_2: "~ῃς", "~ητον", "~ητε";
    perfect_subj_act_3: "~ῃ", _, "~ωσῐ|~ωσῐν";
    perfect_opt_act_1: "~οιμῐ|~οιην", _, "~οιμεν";
    perfect_opt_act_2: "~οις|~οιης", "~οιτον", "~οιτε";
    perfect_opt_act_3: "~οι|~οιη", "~οιτην", "~οιεν";
    perfect_imp_act_2: "~ε", "~ετον", "~ετε";
    perfect_imp_act_3: "~ετω", "~ετων", "~οντων";
    infinitive_perf: "~εναι", _, _;
    participle_perf: "~ως", _, _;
    ),
    // Τεθνηκῃ: 1 words (τεθνηκῃ). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκῃ", 1, "~ῃ", like Τεθνηκας;
    ind_act_1: "~ῃ", _, _;
    ),
    // Τεθνηκοι: 1 words (τεθνηκοι). Like Τεθνηκας, except in 1 cell: ind_act_1 sg.
    conjugation!("τεθνηκοι", 1, "~οι", like Τεθνηκας;
    ind_act_1: "~οι", _, _;
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
    // Προσμειγνῡμῐ: 1 words (προσμειγνῡμῐ). Like Συνοιδᾰ, except in 187 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("προσμειγνῡμῐ", 1, "~~~γνῡμῐ", like Συνοιδᾰ;
    ind_act_1: "~~~γνῡμῐ", _, "~~~γνῠμεν";
    ind_act_2: "~~~γνῡς", _, _;
    ind_act_3: "~~~γνῡσῐ|~~~γνῡσῐν", _, "~~~γνῠᾱσῐ|~~~γνῠᾱσῐν";
    ind_mid_1: "~~~γνῠμαι", _, "~~~γνῠμεθᾰ";
    ind_mid_2: "~~~γνῠσαι", "~~~γνῠσθον", "~~~γνῠσθε";
    ind_mid_3: "~~~γνῠται", _, "~~~γνῠνται";
    ind_pass_1: _, _, "~~~γνῠμεθᾰ";
    ind_pass_2: _, _, "~~~γνῠσθε";
    ind_pass_3: "~~~γνῠται", _, "~~~γνῠνται";
    subj_act_1: "~~~γνῠω", _, "~~~γνῠωμεν";
    subj_act_2: "~~~γνῠῃς", "~~~γνῠητον", "~~~γνῠητε";
    subj_act_3: "~~~γνῠῃ", _, "~~~γνῠωσῐ|~~~γνῠωσῐν";
    subj_mid_1: "~~~γνῠωμαι", _, "~~~γνῠωμεθᾰ";
    subj_mid_2: "~~~γνῠῃ", "~~~γνῠησθον", "~~~γνῠησθε";
    subj_mid_3: "~~~γνῠηται", _, "~~~γνῠωνται";
    subj_pass_1: _, _, "~~~γνῠωμεθᾰ";
    subj_pass_2: _, _, "~~~γνῠησθε";
    subj_pass_3: "~~~γνῠηται", _, "~~~γνῠωνται";
    opt_act_1: "~~~γνῠοιμῐ", _, "~~~γνῠοιμεν";
    opt_act_2: "~~~γνῠοις", "~~~γνῠοιτον", "~~~γνῠοιτε";
    opt_act_3: "~~~γνῠοι", "~~~γνῠοιτην", "~~~γνῠοιεν";
    opt_mid_1: "~~~γνῠοιμην", _, "~~~γνῠοιμεθᾰ";
    opt_mid_2: "~~~γνῠοιο", "~~~γνῠοισθον", "~~~γνῠοισθε";
    opt_mid_3: "~~~γνῠοιτο", "~~~γνῠοισθην", "~~~γνῠοιντο";
    opt_pass_1: _, _, "~~~γνῠοιμεθᾰ";
    opt_pass_2: _, "~~~γνῠοισθον", "~~~γνῠοισθε";
    opt_pass_3: "~~~γνῠοιτο", "~~~γνῠοισθην", "~~~γνῠοιντο";
    imp_act_2: "~~~γνῡ", "~~~γνῠτον", "~~~γνῠτε";
    imp_act_3: "~~~γνῠτω", "~~~γνῠτων", "~~~γνῠντων";
    imp_mid_2: "~~~γνῠσο", _, "~~~γνῠσθε";
    imp_mid_3: "~~~γνῠσθω", "~~~γνῠσθων", "~~~γνῠσθων";
    imp_pass_2: _, _, "~~~γνῠσθε";
    imp_pass_3: _, "~~~γνῠσθων", _;
    imperfect_act_1: "~ε~~γνῡν", _, "~ε~~γνῠμεν";
    imperfect_act_2: "~ε~~γνῡς", "~ε~~γνῠτον", "~ε~~γνῠτε";
    imperfect_act_3: "~ε~~γνῡ", "~ε~~γνῠτην", "~ε~~γνῠσᾰν";
    imperfect_mid_1: "~ε~~γνῠμην", _, "~ε~~γνῠμεθᾰ";
    imperfect_mid_2: "~ε~~γνῠσο", "~ε~~γνῠσθον", "~ε~~γνῠσθε";
    imperfect_mid_3: "~ε~~γνῠτο", "~ε~~γνῠσθην", "~ε~~γνῠντο";
    imperfect_pass_1: _, _, "~ε~~γνῠμεθᾰ";
    imperfect_pass_2: _, _, "~ε~~γνῠσθε";
    imperfect_pass_3: "~ε~~γνῠτο", _, _;
    future_ind_act_1: "~~~ξω", _, "~~~ξομεν";
    future_ind_act_2: "~~~ξεις", "~~~ξετον", "~~~ξετε";
    future_ind_act_3: "~~~ξει", _, "~~~ξουσῐ|~~~ξουσῐν";
    future_ind_mid_1: "~~~ξομαι", _, "~~~ξομεθᾰ";
    future_ind_mid_2: "~~~ξῃ|~~~ξει", "~~~ξεσθον", "~~~ξεσθε";
    future_ind_mid_3: "~~~ξεται", _, "~~~ξονται";
    future_opt_act_1: "~~~ξοιμῐ", _, "~~~ξοιμεν";
    future_opt_act_2: "~~~ξοις", "~~~ξοιτον", "~~~ξοιτε";
    future_opt_act_3: "~~~ξοι", "~~~ξοιτην", "~~~ξοιεν";
    future_opt_mid_1: "~~~ξοιμην", _, "~~~ξοιμεθᾰ";
    future_opt_mid_2: "~~~ξοιο", "~~~ξοισθον", "~~~ξοισθε";
    future_opt_mid_3: "~~~ξοιτο", "~~~ξοισθην", "~~~ξοιντο";
    aorist_ind_act_1: "~ε~~ξᾰ", _, "~ε~~ξᾰμεν";
    aorist_ind_act_2: "~ε~~ξᾰς", "~ε~~ξᾰτον", "~ε~~ξᾰτε";
    aorist_ind_act_3: "~ε~~ξε|~ε~~ξεν", "~ε~~ξᾰτην", "~ε~~ξᾰν";
    aorist_ind_mid_1: "~ε~~ξᾰμην", _, "~ε~~ξᾰμεθᾰ";
    aorist_ind_mid_2: "~ε~~ξω", "~ε~~ξᾰσθον", "~ε~~ξᾰσθε";
    aorist_ind_mid_3: "~ε~~ξᾰτο", "~ε~~ξᾰσθην", "~ε~~ξᾰντο";
    aorist_subj_act_1: "~ῐ~~ξω", _, "~ῐ~~ξωμεν";
    aorist_subj_act_2: "~ῐ~~ξῃς", "~ῐ~~ξητον", "~ῐ~~ξητε";
    aorist_subj_act_3: "~ῐ~~ξῃ", _, "~ῐ~~ξωσῐ|~ῐ~~ξωσῐν";
    aorist_subj_mid_1: "~ῐ~~ξωμαι", _, "~ῐ~~ξωμεθᾰ";
    aorist_subj_mid_2: "~ῐ~~ξῃ", "~ῐ~~ξησθον", "~ῐ~~ξησθε";
    aorist_subj_mid_3: "~ῐ~~ξηται", _, "~ῐ~~ξωνται";
    aorist_opt_act_1: "~ῐ~~ξαιμῐ", _, "~ῐ~~ξαιμεν";
    aorist_opt_act_2: "~ῐ~~ξειᾰς|~ῐ~~ξαις", "~ῐ~~ξαιτον", "~ῐ~~ξαιτε";
    aorist_opt_act_3: "~ῐ~~ξειε|~ῐ~~ξειεν|~ῐ~~ξαι", "~ῐ~~ξαιτην", "~ῐ~~ξειᾰν|~ῐ~~ξαιεν";
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
    // Κιδνᾰμαι: 1 words (κιδνᾰμαι). Like Κᾰθημαι, except in 74 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("κιδνᾰμαι", 1, "~ι~ᾰμαι", like Κᾰθημαι;
    ind_act_1: "~ι~ᾰμαι", _, _;
    ind_mid_1: "~ῐ~ᾰμαι", _, "~ῐ~ᾰμεθᾰ";
    ind_mid_2: "~ῐ~ᾰσαι", "~ῐ~ᾰσθον", "~ῐ~ᾰσθε";
    ind_mid_3: "~ῐ~ᾰται", _, "~ῐ~ᾰνται";
    ind_pass_1: _, _, "~ῐ~ᾰμεθᾰ";
    ind_pass_2: _, _, "~ῐ~ᾰσθε";
    ind_pass_3: "~ῐ~ᾰται", _, "~ῐ~ᾰνται";
    subj_mid_1: "~ῐ~ωμαι", _, "~ῐ~ωμεθᾰ";
    subj_mid_2: "~ῐ~ῃ", "~ῐ~ησθον", "~ῐ~ησθε";
    subj_mid_3: "~ῐ~ηται", _, "~ῐ~ωνται";
    subj_pass_1: _, _, "~ῐ~ωμεθᾰ";
    subj_pass_2: _, _, "~ῐ~ησθε";
    subj_pass_3: "~ῐ~ηται", _, "~ῐ~ωνται";
    opt_mid_1: "~ῐ~αιμην", _, "~ῐ~αιμεθᾰ";
    opt_mid_2: "~ῐ~αιο", "~ῐ~αισθον", "~ῐ~αισθε";
    opt_mid_3: "~ῐ~αιτο", "~ῐ~αισθην", "~ῐ~αιντο";
    opt_pass_1: _, _, "~ῐ~αιμεθᾰ";
    opt_pass_2: _, "~ῐ~αισθον", "~ῐ~αισθε";
    opt_pass_3: "~ῐ~αιτο", "~ῐ~αισθην", "~ῐ~αιντο";
    imp_mid_2: "~ῐ~ᾰσο|~ῐ~ω", _, "~ῐ~ᾰσθε";
    imp_mid_3: "~ῐ~ᾰσθω", "~ῐ~ᾰσθων", "~ῐ~ᾰσθων";
    imp_pass_2: "~ῐ~ᾰσο|~ῐ~ω", _, "~ῐ~ᾰσθε";
    imp_pass_3: _, "~ῐ~ᾰσθων", _;
    imperfect_mid_1: "ἐ~ῐ~ᾰμην", _, "ἐ~ῐ~ᾰμεθᾰ";
    imperfect_mid_2: "ἐ~ῐ~ᾰσο", "ἐ~ῐ~ᾰσθον", "ἐ~ῐ~ᾰσθε";
    imperfect_mid_3: "ἐ~ῐ~ᾰτο", "ἐ~ῐ~ᾰσθην", "ἐ~ῐ~ᾰντο";
    imperfect_pass_1: _, _, "ἐ~ῐ~ᾰμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ῐ~ᾰσθε";
    imperfect_pass_3: "ἐ~ῐ~ᾰτο", _, _;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, derived;
    pluperfect_pass_1: derived, _, derived;
    pluperfect_pass_2: derived, derived, derived;
    pluperfect_pass_3: derived, derived, derived;
    infinitive_pres: _, "~ῐ~ᾰσθαι", _;
    participle_pres: _, "~ῐ~ᾰμενος", _;
    ),
    // Τεθηπᾰ: 1 words (τεθηπᾰ). Like Ἐφᾰγον, except in 62 cells: ind_act_1 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_2 pl, ….
    conjugation!("τεθηπᾰ", 1, "~εθηπᾰ", like Ἐφᾰγον;
    ind_act_1: "~εθηπᾰ", _, _;
    aorist_ind_act_1: "ἐ~ᾰφον", _, "ἐ~ᾰφομεν";
    aorist_ind_act_2: "ἐ~ᾰφες", "ἐ~ᾰφετον", "ἐ~ᾰφετε";
    aorist_ind_act_3: "ἐ~ᾰφε|ἐ~ᾰφεν", "ἐ~ᾰφετην", "ἐ~ᾰφον";
    aorist_subj_act_1: "~ᾰφω", _, "~ᾰφωμεν";
    aorist_subj_act_2: "~ᾰφῃς", "~ᾰφητον", "~ᾰφητε";
    aorist_subj_act_3: "~ᾰφῃ", _, "~ᾰφωσῐ|~ᾰφωσῐν";
    aorist_opt_act_1: "~ᾰφοιμῐ", _, "~ᾰφοιμεν";
    aorist_opt_act_2: "~ᾰφοις", "~ᾰφοιτον", "~ᾰφοιτε";
    aorist_opt_act_3: "~ᾰφοι", "~ᾰφοιτην", "~ᾰφοιεν";
    aorist_imp_act_2: "~ᾰφε", "~ᾰφετον", "~ᾰφετε";
    aorist_imp_act_3: "~ᾰφετω", "~ᾰφετων", "~ᾰφοντων";
    perfect_ind_act_1: "~εθηπᾰ", _, "~εθηπᾰμεν";
    perfect_ind_act_2: "~εθηπᾰς", "~εθηπᾰτον", "~εθηπᾰτε";
    perfect_ind_act_3: "~εθηπε|~εθηπεν", _, "~εθηπᾱσῐ|~εθηπᾱσῐν";
    perfect_subj_act_1: "~εθηπω", _, "~εθηπωμεν";
    perfect_subj_act_2: "~εθηπῃς", "~εθηπητον", "~εθηπητε";
    perfect_subj_act_3: "~εθηπῃ", _, "~εθηπωσῐ|~εθηπωσῐν";
    perfect_opt_act_1: "~εθηποιμῐ|~εθηποιην", _, "~εθηποιμεν";
    perfect_opt_act_2: "~εθηποις|~εθηποιης", "~εθηποιτον", "~εθηποιτε";
    perfect_opt_act_3: "~εθηποι|~εθηποιη", "~εθηποιτην", "~εθηποιεν";
    perfect_imp_act_2: "~εθηπε", "~εθηπετον", "~εθηπετε";
    perfect_imp_act_3: "~εθηπετω", "~εθηπετων", "~εθηποντων";
    infinitive_aor: "~ᾰφεῑν", _, _;
    infinitive_perf: "~εθηπεναι", _, _;
    participle_aor: "~ᾰφων", _, _;
    participle_perf: "~εθηπως", _, _;
    ),
    // Ἀντῐκᾰθῐστημῐ: 1 words (ἀντῐκᾰθῐστημῐ). Like Ἐγκᾰθῐστημῐ, except in 125 cells: aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_2 pl, aorist_ind_act_3 sg, ….
    conjugation!("ἀντῐκᾰθῐστημῐ", 1, "~θῐ~ημῐ", like Ἐγκᾰθῐστημῐ;
    aorist_ind_act_1: "~τε~ησᾰ|~τε~ην", _, "~τε~ησᾰμεν|~τε~ημεν";
    aorist_ind_act_2: "~τε~ησᾰς|~τε~ης", "~τε~ησᾰτον|~τε~ητον", "~τε~ησᾰτε|~τε~ητε";
    aorist_ind_act_3: "~τε~ησε|~τε~ησεν|~τε~η", "~τε~ησᾰτην|~τε~ητην", _;
    aorist_ind_mid_1: "~τε~ησᾰμην|~τε~ᾰμην", _, "~τε~ησᾰμεθᾰ|~τε~ᾰμεθᾰ";
    aorist_ind_mid_2: "~τε~ησω|~τε~ω", "~τε~ησᾰσθον|~τε~ᾰσθον", "~τε~ησᾰσθε|~τε~ᾰσθε";
    aorist_ind_mid_3: "~τε~ησᾰτο|~τε~ᾰτο", "~τε~ησᾰσθην|~τε~ᾰσθην", "~τε~ησᾰντο|~τε~ᾰντο";
    aorist_ind_pass_1: "~τε~ᾰθην|~τε~ην", _, "~τε~ᾰθημεν|~τε~ημεν";
    aorist_ind_pass_2: "~τε~ᾰθης|~τε~ης", "~τε~ᾰθητον|~τε~ητον", "~τε~ᾰθητε|~τε~ητε";
    aorist_ind_pass_3: "~τε~ᾰθη|~τε~η", "~τε~ᾰθητην|~τε~ητην", "~τε~ᾰθησᾰν|~τε~ησᾰν";
    aorist_subj_act_1: "~τᾰ~ησω|~τᾰ~ω", _, "~τᾰ~ησωμεν|~τᾰ~ωμεν";
    aorist_subj_act_2: "~τᾰ~ησῃς|~τᾰ~ῃς", "~τᾰ~ησητον|~τᾰ~ητον", "~τᾰ~ησητε|~τᾰ~ητε";
    aorist_subj_act_3: "~τᾰ~ησῃ|~τᾰ~ῃ", "~τᾰ~ησητον|~τᾰ~ητον", "~τᾰ~ησωσῐ|~τᾰ~ησωσῐν|~τᾰ~ωσῐ|~τᾰ~ωσῐν";
    aorist_subj_mid_1: "~τᾰ~ησωμαι|~τᾰ~ωμαι", _, "~τᾰ~ησωμεθᾰ|~τᾰ~ωμεθᾰ";
    aorist_subj_mid_2: "~τᾰ~ησῃ|~τᾰ~ῃ", "~τᾰ~ησησθον|~τᾰ~ησθον", "~τᾰ~ησησθε|~τᾰ~ησθε";
    aorist_subj_mid_3: "~τᾰ~ησηται|~τᾰ~ηται", "~τᾰ~ησησθον|~τᾰ~ησθον", "~τᾰ~ησωνται|~τᾰ~ωνται";
    aorist_subj_pass_1: "~τᾰ~ᾰθω|~τᾰ~ω", _, "~τᾰ~ᾰθωμεν|~τᾰ~ωμεν";
    aorist_subj_pass_2: "~τᾰ~ᾰθῃς|~τᾰ~ῃς", "~τᾰ~ᾰθητον|~τᾰ~ητον", "~τᾰ~ᾰθητε|~τᾰ~ητε";
    aorist_subj_pass_3: "~τᾰ~ᾰθῃ|~τᾰ~ῃ", "~τᾰ~ᾰθητον|~τᾰ~ητον", "~τᾰ~ᾰθωσῐ|~τᾰ~ᾰθωσῐν|~τᾰ~ωσῐ|~τᾰ~ωσῐν";
    aorist_opt_act_1: "~τᾰ~ησαιμῐ|~τᾰ~αιην", _, "~τᾰ~ησαιμεν|~τᾰ~αῑμεν|~τᾰ~αιημεν";
    aorist_opt_act_2: "~τᾰ~ησειᾰς|~τᾰ~ησαις|~τᾰ~αιης", "~τᾰ~ησαιτον|~τᾰ~αῑτον|~τᾰ~αιητον", "~τᾰ~ησαιτε|~τᾰ~αῑτε|~τᾰ~αιητε";
    aorist_opt_act_3: "~τᾰ~ησειε|~τᾰ~ησειεν|~τᾰ~ησαι|~τᾰ~αιη", "~τᾰ~ησαιτην|~τᾰ~αιτην|~τᾰ~αιητην", "~τᾰ~ησειᾰν|~τᾰ~ησαιεν|~τᾰ~αῑεν|~τᾰ~αιησᾰν";
    aorist_opt_mid_1: "~τᾰ~ησαιμην|~τᾰ~αιμην", _, "~τᾰ~ησαιμεθᾰ|~τᾰ~αιμεθᾰ";
    aorist_opt_mid_2: "~τᾰ~ησαιο|~τᾰ~αῑο", "~τᾰ~ησαισθον|~τᾰ~αῑσθον", "~τᾰ~ησαισθε|~τᾰ~αῑσθε";
    aorist_opt_mid_3: "~τᾰ~ησαιτο|~τᾰ~αῑτο", "~τᾰ~ησαισθην|~τᾰ~αισθην", "~τᾰ~ησαιντο|~τᾰ~αῑντο";
    aorist_opt_pass_1: "~τᾰ~ᾰθειην|~τᾰ~ειην", _, "~τᾰ~ᾰθεῑμεν|~τᾰ~ᾰθειημεν|~τᾰ~εῑμεν|~τᾰ~ειημεν";
    aorist_opt_pass_2: "~τᾰ~ᾰθειης|~τᾰ~ειης", "~τᾰ~ᾰθεῑτον|~τᾰ~ᾰθειητον|~τᾰ~εῑτον|~τᾰ~ειητον", "~τᾰ~ᾰθεῑτε|~τᾰ~ᾰθειητε|~τᾰ~εῑτε|~τᾰ~ειητε";
    aorist_opt_pass_3: "~τᾰ~ᾰθειη|~τᾰ~ειη", "~τᾰ~ᾰθειτην|~τᾰ~ᾰθειητην|~τᾰ~ειτην|~τᾰ~ειητην", "~τᾰ~ᾰθεῑεν|~τᾰ~ᾰθειησᾰν|~τᾰ~εῑεν|~τᾰ~ειησᾰν";
    aorist_imp_act_2: "~τᾰ~ησον|~τᾰ~ηθῐ", "~τᾰ~ησᾰτον|~τᾰ~ητον", "~τᾰ~ησᾰτε|~τᾰ~ητε";
    aorist_imp_act_3: "~τᾰ~ησᾰτω|~τᾰ~ητω", "~τᾰ~ησᾰτων|~τᾰ~ητων", "~τᾰ~ησᾰντων|~τᾰ~ᾰντων";
    aorist_imp_mid_2: "~τᾰ~ησαι|~τᾰ~ω", "~τᾰ~ησᾰσθον|~τᾰ~ᾰσθον", "~τᾰ~ησᾰσθε|~τᾰ~ᾰσθε";
    aorist_imp_mid_3: "~τᾰ~ησᾰσθω|~τᾰ~ᾰσθω", "~τᾰ~ησᾰσθων|~τᾰ~ᾰσθων", "~τᾰ~ησᾰσθων|~τᾰ~ᾰσθων";
    aorist_imp_pass_2: "~τᾰ~ᾰθητῐ|~τᾰ~ηθῐ", "~τᾰ~ᾰθητον|~τᾰ~ητον", "~τᾰ~ᾰθητε|~τᾰ~ητε";
    aorist_imp_pass_3: "~τᾰ~ᾰθητω|~τᾰ~ητω", "~τᾰ~ᾰθητων|~τᾰ~ητων", "~τᾰ~ᾰθεντων|~τᾰ~εντων";
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
    infinitive_aor: "~τᾰ~ησαι|~τᾰ~ηναι", "~τᾰ~ησᾰσθαι|~τᾰ~ᾰσθαι", "~τᾰ~ᾰθηναι|~τᾰ~ηναι";
    infinitive_perf: "~θε~ηκεναι", _, _;
    participle_aor: "~τᾰ~ησᾱς|~τᾰ~ᾱς", "~τᾰ~ησᾰμενος|~τᾰ~ᾰμενος", "~τᾰ~ᾰθεις|~τᾰ~εις";
    participle_perf: "~θε~ηκως", _, _;
    ),
    // Στρωννῡμῐ: 1 words (στρωννῡμῐ). Like Ζωννῡμῐ, except in 89 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("στρωννῡμῐ", 1, "~ννῡμῐ", like Ζωννῡμῐ;
    imperfect_act_1: "ἐ~ννῡν", _, "ἐ~ννῠμεν";
    imperfect_act_2: "ἐ~ννῡς", "ἐ~ννῠτον", "ἐ~ννῠτε";
    imperfect_act_3: "ἐ~ννῡ", "ἐ~ννῠτην", "ἐ~ννῠσᾰν";
    imperfect_mid_1: "ἐ~ννῠμην", _, "ἐ~ννῠμεθᾰ";
    imperfect_mid_2: "ἐ~ννῠσο", "ἐ~ννῠσθον", "ἐ~ννῠσθε";
    imperfect_mid_3: "ἐ~ννῠτο", "ἐ~ννῠσθην", "ἐ~ννῠντο";
    imperfect_pass_1: derived, _, "ἐ~ννῠμεθᾰ";
    imperfect_pass_2: derived, derived, "ἐ~ννῠσθε";
    imperfect_pass_3: "ἐ~ννῠτο", derived, derived;
    future_ind_act_1: "~σω|~ννῡσω", _, "~σομεν|~ννῡσομεν";
    future_ind_act_2: "~σεις|~ννῡσεις", "~σετον|~ννῡσετον", "~σετε|~ννῡσετε";
    future_ind_act_3: "~σει|~ννῡσει", "~σετον|~ννῡσετον", "~σουσῐ|~σουσῐν|~ννῡσουσῐ|~ννῡσουσῐν";
    future_ind_mid_1: "~σομαι|~ννῡσομαι", _, "~σομεθᾰ|~ννῡσομεθᾰ";
    future_ind_mid_2: "~σῃ|~σει|~ννῡσῃ|~ννῡσει", "~σεσθον|~ννῡσεσθον", "~σεσθε|~ννῡσεσθε";
    future_ind_mid_3: "~σεται|~ννῡσεται", "~σεσθον|~ννῡσεσθον", "~σονται|~ννῡσονται";
    future_opt_act_1: "~σοιμῐ|~ννῡσοιμῐ", _, "~σοιμεν|~ννῡσοιμεν";
    future_opt_act_2: "~σοις|~ννῡσοις", "~σοιτον|~ννῡσοιτον", "~σοιτε|~ννῡσοιτε";
    future_opt_act_3: "~σοι|~ννῡσοι", "~σοιτην|~ννῡσοιτην", "~σοιεν|~ννῡσοιεν";
    future_opt_mid_1: "~σοιμην|~ννῡσοιμην", _, "~σοιμεθᾰ|~ννῡσοιμεθᾰ";
    future_opt_mid_2: "~σοιο|~ννῡσοιο", "~σοισθον|~ννῡσοισθον", "~σοισθε|~ννῡσοισθε";
    future_opt_mid_3: "~σοιτο|~ννῡσοιτο", "~σοισθην|~ννῡσοισθην", "~σοιντο|~ννῡσοιντο";
    aorist_ind_pass_1: "ἐ~θην", _, "ἐ~θημεν";
    aorist_ind_pass_2: "ἐ~θης", "ἐ~θητον", "ἐ~θητε";
    aorist_ind_pass_3: "ἐ~θη", "ἐ~θητην", "ἐ~θησᾰν";
    aorist_subj_pass_1: "~θω", _, "~θωμεν";
    aorist_subj_pass_2: "~θῃς", "~θητον", "~θητε";
    aorist_subj_pass_3: "~θῃ", _, "~θωσῐ|~θωσῐν";
    aorist_opt_pass_1: "~θειην", _, "~θεῑμεν|~θειημεν";
    aorist_opt_pass_2: "~θειης", "~θεῑτον|~θειητον", "~θεῑτε|~θειητε";
    aorist_opt_pass_3: "~θειη", "~θειτην|~θειητην", "~θεῑεν|~θειησᾰν";
    aorist_imp_pass_2: "~θητῐ", _, _;
    aorist_imp_pass_3: "~θητω", "~θητων", "~θεντων";
    infinitive_fut: "~σειν|~ννῡσειν", "~σεσθαι|~ννῡσεσθαι", _;
    infinitive_aor: _, _, "~θηναι";
    participle_fut: "~σων|~ννῡσων", "~σομενος|~ννῡσομενος", _;
    participle_aor: _, _, "~θεις";
    ),
    // Δῐκεῑν: 1 words (δῐκεῑν). Like Ἐφᾰγον, except in 1 cell: ind_act_1 sg.
    conjugation!("δῐκεῑν", 1, "~εῑν", like Ἐφᾰγον;
    ind_act_1: "~εῑν", _, _;
    ),
    // Ξῠρρηγνῡμῐ: 1 words (ξῠρρηγνῡμῐ). Like Λιστραω, except in 282 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ξῠρρηγνῡμῐ", 1, "~~ηγνῡμῐ", like Λιστραω;
    ind_act_1: "~~ηγνῡμῐ", _, "~~ηγνῠμεν";
    ind_act_2: "~~ηγνῡς", _, _;
    ind_act_3: "~~ηγνῡσῐ|~~ηγνῡσῐν", _, "~~ηγνῠᾱσῐ|~~ηγνῠᾱσῐν";
    ind_mid_1: "~~ηγνῠμαι", _, "~~ηγνῠμεθᾰ";
    ind_mid_2: "~~ηγνῠσαι", "~~ηγνῠσθον", "~~ηγνῠσθε";
    ind_mid_3: "~~ηγνῠται", _, "~~ηγνῠνται";
    ind_pass_1: _, _, "~~ηγνῠμεθᾰ";
    ind_pass_2: _, _, "~~ηγνῠσθε";
    ind_pass_3: "~~ηγνῠται", _, "~~ηγνῠνται";
    subj_act_1: "~~ηγνῠω", _, "~~ηγνῠωμεν";
    subj_act_2: "~~ηγνῠῃς", "~~ηγνῠητον", "~~ηγνῠητε";
    subj_act_3: "~~ηγνῠῃ", _, "~~ηγνῠωσῐ|~~ηγνῠωσῐν";
    subj_mid_1: "~~ηγνῠωμαι", _, "~~ηγνῠωμεθᾰ";
    subj_mid_2: "~~ηγνῠῃ", "~~ηγνῠησθον", "~~ηγνῠησθε";
    subj_mid_3: "~~ηγνῠηται", _, "~~ηγνῠωνται";
    subj_pass_1: _, _, "~~ηγνῠωμεθᾰ";
    subj_pass_2: _, _, "~~ηγνῠησθε";
    subj_pass_3: "~~ηγνῠηται", _, "~~ηγνῠωνται";
    opt_act_1: "~~ηγνῠοιμῐ", _, "~~ηγνῠοιμεν";
    opt_act_2: "~~ηγνῠοις", "~~ηγνῠοιτον", "~~ηγνῠοιτε";
    opt_act_3: "~~ηγνῠοι", "~~ηγνῠοιτην", "~~ηγνῠοιεν";
    opt_mid_1: "~~ηγνῠοιμην", _, "~~ηγνῠοιμεθᾰ";
    opt_mid_2: "~~ηγνῠοιο", "~~ηγνῠοισθον", "~~ηγνῠοισθε";
    opt_mid_3: "~~ηγνῠοιτο", "~~ηγνῠοισθην", "~~ηγνῠοιντο";
    opt_pass_1: _, _, "~~ηγνῠοιμεθᾰ";
    opt_pass_2: _, "~~ηγνῠοισθον", "~~ηγνῠοισθε";
    opt_pass_3: "~~ηγνῠοιτο", "~~ηγνῠοισθην", "~~ηγνῠοιντο";
    imp_act_2: "~~ηγνῡ", "~~ηγνῠτον", "~~ηγνῠτε";
    imp_act_3: "~~ηγνῠτω", "~~ηγνῠτων", "~~ηγνῠντων";
    imp_mid_2: "~~ηγνῠσο", _, "~~ηγνῠσθε";
    imp_mid_3: "~~ηγνῠσθω", "~~ηγνῠσθων", "~~ηγνῠσθων";
    imp_pass_2: _, _, "~~ηγνῠσθε";
    imp_pass_3: _, "~~ηγνῠσθων", _;
    imperfect_act_1: "~νε~ηγνῡν", _, "~νε~ηγνῠμεν";
    imperfect_act_2: "~νε~ηγνῡς", "~νε~ηγνῠτον", "~νε~ηγνῠτε";
    imperfect_act_3: "~νε~ηγνῡ", "~νε~ηγνῠτην", "~νε~ηγνῠσᾰν";
    imperfect_mid_1: "~νε~ηγνῠμην", _, "~νε~ηγνῠμεθᾰ";
    imperfect_mid_2: "~νε~ηγνῠσο", "~νε~ηγνῠσθον", "~νε~ηγνῠσθε";
    imperfect_mid_3: "~νε~ηγνῠτο", "~νε~ηγνῠσθην", "~νε~ηγνῠντο";
    imperfect_pass_1: _, _, "~νε~ηγνῠμεθᾰ";
    imperfect_pass_2: _, _, "~νε~ηγνῠσθε";
    imperfect_pass_3: "~νε~ηγνῠτο", _, _;
    future_ind_act_1: "~~ηξω", _, "~~ηξομεν";
    future_ind_act_2: "~~ηξεις", "~~ηξετον", "~~ηξετε";
    future_ind_act_3: "~~ηξει", _, "~~ηξουσῐ|~~ηξουσῐν";
    future_ind_mid_1: "~~ηξομαι", _, "~~ηξομεθᾰ";
    future_ind_mid_2: "~~ηξῃ|~~ηξει", "~~ηξεσθον", "~~ηξεσθε";
    future_ind_mid_3: "~~ηξεται", _, "~~ηξονται";
    future_opt_act_1: "~~ηξοιμῐ", _, "~~ηξοιμεν";
    future_opt_act_2: "~~ηξοις", "~~ηξοιτον", "~~ηξοιτε";
    future_opt_act_3: "~~ηξοι", "~~ηξοιτην", "~~ηξοιεν";
    future_opt_mid_1: "~~ηξοιμην", _, "~~ηξοιμεθᾰ";
    future_opt_mid_2: "~~ηξοιο", "~~ηξοισθον", "~~ηξοισθε";
    future_opt_mid_3: "~~ηξοιτο", "~~ηξοισθην", "~~ηξοιντο";
    aorist_ind_act_1: "~νε~ηξᾰ", _, "~νε~ηξᾰμεν";
    aorist_ind_act_2: "~νε~ηξᾰς", "~νε~ηξᾰτον", "~νε~ηξᾰτε";
    aorist_ind_act_3: "~νε~ηξε|~νε~ηξεν", "~νε~ηξᾰτην", "~νε~ηξᾰν";
    aorist_ind_mid_1: "~νε~ηξᾰμην", _, "~νε~ηξᾰμεθᾰ";
    aorist_ind_mid_2: "~νε~ηξω", "~νε~ηξᾰσθον", "~νε~ηξᾰσθε";
    aorist_ind_mid_3: "~νε~ηξᾰτο", "~νε~ηξᾰσθην", "~νε~ηξᾰντο";
    aorist_ind_pass_1: "~νε~ᾰγην", _, "~νε~ᾰγημεν";
    aorist_ind_pass_2: "~νε~ᾰγης", "~νε~ᾰγητον", "~νε~ᾰγητε";
    aorist_ind_pass_3: "~νε~ᾰγη", "~νε~ᾰγητην", "~νε~ᾰγησᾰν";
    aorist_subj_act_1: "~~ηξω", _, "~~ηξωμεν";
    aorist_subj_act_2: "~~ηξῃς", "~~ηξητον", "~~ηξητε";
    aorist_subj_act_3: "~~ηξῃ", _, "~~ηξωσῐ|~~ηξωσῐν";
    aorist_subj_mid_1: "~~ηξωμαι", _, "~~ηξωμεθᾰ";
    aorist_subj_mid_2: "~~ηξῃ", "~~ηξησθον", "~~ηξησθε";
    aorist_subj_mid_3: "~~ηξηται", _, "~~ηξωνται";
    aorist_subj_pass_1: "~~ᾰγω", _, "~~ᾰγωμεν";
    aorist_subj_pass_2: "~~ᾰγῃς", "~~ᾰγητον", "~~ᾰγητε";
    aorist_subj_pass_3: "~~ᾰγῃ", _, "~~ᾰγωσῐ|~~ᾰγωσῐν";
    aorist_opt_act_1: "~~ηξαιμῐ", _, "~~ηξαιμεν";
    aorist_opt_act_2: "~~ηξειᾰς|~~ηξαις", "~~ηξαιτον", "~~ηξαιτε";
    aorist_opt_act_3: "~~ηξειε|~~ηξειεν|~~ηξαι", "~~ηξαιτην", "~~ηξειᾰν|~~ηξαιεν";
    aorist_opt_mid_1: "~~ηξαιμην", _, "~~ηξαιμεθᾰ";
    aorist_opt_mid_2: "~~ηξαιο", "~~ηξαισθον", "~~ηξαισθε";
    aorist_opt_mid_3: "~~ηξαιτο", "~~ηξαισθην", "~~ηξαιντο";
    aorist_opt_pass_1: "~~ᾰγειην", _, "~~ᾰγεῑμεν|~~ᾰγειημεν";
    aorist_opt_pass_2: "~~ᾰγειης", "~~ᾰγεῑτον|~~ᾰγειητον", "~~ᾰγεῑτε|~~ᾰγειητε";
    aorist_opt_pass_3: "~~ᾰγειη", "~~ᾰγειτην|~~ᾰγειητην", "~~ᾰγεῑεν|~~ᾰγειησᾰν";
    aorist_imp_act_2: "~~ηξον", "~~ηξᾰτον", "~~ηξᾰτε";
    aorist_imp_act_3: "~~ηξᾰτω", "~~ηξᾰτων", "~~ηξᾰντων";
    aorist_imp_mid_2: "~~ηξαι", "~~ηξᾰσθον", "~~ηξᾰσθε";
    aorist_imp_mid_3: "~~ηξᾰσθω", "~~ηξᾰσθων", _;
    aorist_imp_pass_2: "~~ᾰγηθῐ", _, _;
    aorist_imp_pass_3: "~~ᾰγητω", "~~ᾰγητων", "~~ᾰγεντων";
    perfect_ind_act_1: "~νε~ωγᾰ", _, "~νε~ωγᾰμεν";
    perfect_ind_act_2: "~νε~ωγᾰς", "~νε~ωγᾰτον", "~νε~ωγᾰτε";
    perfect_ind_act_3: "~νε~ωγε|~νε~ωγεν", _, "~νε~ωγᾱσῐ|~νε~ωγᾱσῐν";
    perfect_ind_mid_1: "~~ηρρηγμαι", _, "~~ηρρηγμεθᾰ";
    perfect_ind_mid_2: "~~ηρρηξαι", "~~ηρρηχθον", "~~ηρρηχθε";
    perfect_ind_mid_3: "~~ηρρηκται", _, "~~ηρρηγᾰται";
    perfect_ind_pass_1: _, _, "~~ηρρηγμεθᾰ";
    perfect_ind_pass_2: _, _, "~~ηρρηχθε";
    perfect_subj_act_1: "~νε~ωγω", _, "~νε~ωγωμεν";
    perfect_subj_act_2: "~νε~ωγῃς", "~νε~ωγητον", "~νε~ωγητε";
    perfect_subj_act_3: "~νε~ωγῃ", _, "~νε~ωγωσῐ|~νε~ωγωσῐν";
    perfect_opt_act_1: "~νε~ωγοιμῐ|~νε~ωγοιην", _, "~νε~ωγοιμεν";
    perfect_opt_act_2: "~νε~ωγοις|~νε~ωγοιης", "~νε~ωγοιτον", "~νε~ωγοιτε";
    perfect_opt_act_3: "~νε~ωγοι|~νε~ωγοιη", "~νε~ωγοιτην", "~νε~ωγοιεν";
    perfect_imp_act_2: "~νε~ωγε", "~νε~ωγετον", "~νε~ωγετε";
    perfect_imp_act_3: "~νε~ωγετω", "~νε~ωγετων", "~νε~ωγοντων";
    perfect_imp_mid_2: "~~ηρρηξο", _, _;
    perfect_imp_mid_3: "~~ηρρηχθω", "~~ηρρηχθων", _;
    perfect_imp_pass_2: "~~ηρρηξο", _, _;
    perfect_imp_pass_3: "~~ηρρηχθω", "~~ηρρηχθων", _;
    pluperfect_act_1: "~νε~ωγειν|~νε~ωγη", _, "~νε~ωγεμεν";
    pluperfect_act_2: "~νε~ωγεις|~νε~ωγης", "~νε~ωγετον", "~νε~ωγετε";
    pluperfect_act_3: "~νε~ωγει|~νε~ωγειν", "~νε~ωγετην", "~νε~ωγεσᾰν";
    pluperfect_mid_1: "~~ηρρηγμην", _, "~~ηρρηγμεθᾰ";
    pluperfect_mid_2: "~~ηρρηξο", "~~ηρρηχθον", "~~ηρρηχθε";
    pluperfect_mid_3: "~~ηρρηκτο", "~~ηρρηχθην", _;
    pluperfect_pass_1: _, _, "~~ηρρηγμεθᾰ";
    pluperfect_pass_2: _, _, "~~ηρρηχθε";
    pluperfect_pass_3: _, "~~ηρρηχθην", "~~ηρρηγᾰτο";
    infinitive_pres: "~~ηγνῠναι", "~~ηγνῠσθαι", _;
    infinitive_fut: "~~ηξειν", "~~ηξεσθαι", _;
    infinitive_aor: "~~ηξαι", "~~ηξᾰσθαι", "~~ᾰγηναι";
    infinitive_perf: "~νε~ωγεναι", "~~ηρρηχθαι", _;
    participle_pres: "~~ηγνῡς", "~~ηγνῠμενος", _;
    participle_fut: "~~ηξων", "~~ηξομενος", _;
    participle_aor: "~~ηξᾱς", "~~ηξᾰμενος", "~~ᾰγεις";
    participle_perf: "~νε~ωγως", "~~ηρρηγμενος", _;
    ),
    // Ἀμφῑστημῐ: 1 words (ἀμφῑστημῐ). Like Ἀπᾰνῐστημῐ, except in 269 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἀμφῑστημῐ", 1, "~ῑ~ημῐ", like Ἀπᾰνῐστημῐ;
    ind_act_1: "~ῑ~ημῐ", _, "~ῑ~ᾰμεν";
    ind_act_2: "~ῑ~ης", _, _;
    ind_act_3: "~ῑ~ησῐ|~ῑ~ησῐν", _, "~ῑ~ᾱσῐ|~ῑ~ᾱσῐν";
    ind_mid_1: "~ῑ~ᾰμαι", _, "~ῑ~ᾰμεθᾰ";
    ind_mid_2: "~ῑ~ᾰσαι", "~ῑ~ᾰσθον", "~ῑ~ᾰσθε";
    ind_mid_3: "~ῑ~ᾰται", _, "~ῑ~ᾰνται";
    ind_pass_1: _, _, "~ῑ~ᾰμεθᾰ";
    ind_pass_2: _, _, "~ῑ~ᾰσθε";
    ind_pass_3: "~ῑ~ᾰται", _, "~ῑ~ᾰνται";
    subj_act_1: "~ῑ~ω", _, "~ῑ~ωμεν";
    subj_act_2: "~ῑ~ῃς", "~ῑ~ητον", "~ῑ~ητε";
    subj_act_3: "~ῑ~ῃ", _, "~ῑ~ωσῐ|~ῑ~ωσῐν";
    subj_mid_1: "~ῑ~ωμαι", _, "~ῑ~ωμεθᾰ";
    subj_mid_2: "~ῑ~ῃ", "~ῑ~ησθον", "~ῑ~ησθε";
    subj_mid_3: "~ῑ~ηται", _, "~ῑ~ωνται";
    subj_pass_1: _, _, "~ῑ~ωμεθᾰ";
    subj_pass_2: _, _, "~ῑ~ησθε";
    subj_pass_3: "~ῑ~ηται", _, "~ῑ~ωνται";
    opt_act_1: "~ῑ~αιην", _, "~ῑ~αῑμεν|~ῑ~αιημεν";
    opt_act_2: "~ῑ~αιης", "~ῑ~αῑτον|~ῑ~αιητον", "~ῑ~αῑτε|~ῑ~αιητε";
    opt_act_3: "~ῑ~αιη", "~ῑ~αιτην|~ῑ~αιητην", "~ῑ~αῑεν|~ῑ~αιησᾰν";
    opt_mid_1: "~ῑ~αιμην", _, "~ῑ~αιμεθᾰ";
    opt_mid_2: "~ῑ~αιο", "~ῑ~αισθον", "~ῑ~αισθε";
    opt_mid_3: "~ῑ~αιτο", "~ῑ~αισθην", "~ῑ~αιντο";
    opt_pass_1: _, _, "~ῑ~αιμεθᾰ";
    opt_pass_2: _, "~ῑ~αισθον", "~ῑ~αισθε";
    opt_pass_3: "~ῑ~αιτο", "~ῑ~αισθην", "~ῑ~αιντο";
    imp_act_2: "~ῑ~η", "~ῑ~ᾰτον", "~ῑ~ᾰτε";
    imp_act_3: "~ῑ~ᾰτω", "~ῑ~ᾰτων", "~ῑ~ᾰντων";
    imp_mid_2: "~ῑ~ᾰσο|~ῑ~ω", _, "~ῑ~ᾰσθε";
    imp_mid_3: "~ῑ~ᾰσθω", "~ῑ~ᾰσθων", "~ῑ~ᾰσθων";
    imp_pass_2: "~ῑ~ᾰσο|~ῑ~ω", _, "~ῑ~ᾰσθε";
    imp_pass_3: _, "~ῑ~ᾰσθων", _;
    future_ind_act_1: "~ι~ησω", _, "~ι~ησομεν";
    future_ind_act_2: "~ι~ησεις", "~ι~ησετον", "~ι~ησετε";
    future_ind_act_3: "~ι~ησει", _, "~ι~ησουσῐ|~ι~ησουσῐν";
    future_ind_mid_1: "~ι~ησομαι", _, "~ι~ησομεθᾰ";
    future_ind_mid_2: "~ι~ησῃ|~ι~ησει", "~ι~ησεσθον", "~ι~ησεσθε";
    future_ind_mid_3: "~ι~ησεται", _, "~ι~ησονται";
    future_ind_pass_1: "~ῑ~ᾰθησομαι", _, "~ῑ~ᾰθησομεθᾰ";
    future_ind_pass_2: "~ῑ~ᾰθησῃ|~ῑ~ᾰθησει", "~ῑ~ᾰθησεσθον", "~ῑ~ᾰθησεσθε";
    future_ind_pass_3: "~ῑ~ᾰθησεται", _, "~ῑ~ᾰθησονται";
    future_opt_act_1: "~ι~ησοιμῐ", _, "~ι~ησοιμεν";
    future_opt_act_2: "~ι~ησοις", "~ι~ησοιτον", "~ι~ησοιτε";
    future_opt_act_3: "~ι~ησοι", "~ι~ησοιτην", "~ι~ησοιεν";
    future_opt_mid_1: "~ι~ησοιμην", _, "~ι~ησοιμεθᾰ";
    future_opt_mid_2: "~ι~ησοιο", "~ι~ησοισθον", "~ι~ησοισθε";
    future_opt_mid_3: "~ι~ησοιτο", "~ι~ησοισθην", "~ι~ησοιντο";
    future_opt_pass_1: "~ῑ~ᾰθησοιμην", _, "~ῑ~ᾰθησοιμεθᾰ";
    future_opt_pass_2: "~ῑ~ᾰθησοιο", "~ῑ~ᾰθησοισθον", "~ῑ~ᾰθησοισθε";
    future_opt_pass_3: "~ῑ~ᾰθησοιτο", "~ῑ~ᾰθησοισθην", "~ῑ~ᾰθησοιντο";
    aorist_ind_pass_1: "~ε~ᾰθην", _, "~ε~ᾰθημεν";
    aorist_ind_pass_2: "~ε~ᾰθης", "~ε~ᾰθητον", "~ε~ᾰθητε";
    aorist_ind_pass_3: "~ε~ᾰθη", "~ε~ᾰθητην", "~ε~ᾰθησᾰν";
    aorist_subj_act_1: "~ι~ησω", _, "~ι~ησωμεν";
    aorist_subj_act_2: "~ι~ησῃς", "~ι~ησητον", "~ι~ησητε";
    aorist_subj_act_3: "~ι~ησῃ", _, "~ι~ησωσῐ|~ι~ησωσῐν";
    aorist_subj_mid_1: "~ι~ησωμαι", _, "~ι~ησωμεθᾰ";
    aorist_subj_mid_2: "~ι~ησῃ", "~ι~ησησθον", "~ι~ησησθε";
    aorist_subj_mid_3: "~ι~ησηται", _, "~ι~ησωνται";
    aorist_subj_pass_1: "~ι~ᾰθω", _, "~ι~ᾰθωμεν";
    aorist_subj_pass_2: "~ι~ᾰθῃς", "~ι~ᾰθητον", "~ι~ᾰθητε";
    aorist_subj_pass_3: "~ι~ᾰθῃ", _, "~ι~ᾰθωσῐ|~ι~ᾰθωσῐν";
    aorist_opt_act_1: "~ι~ησαιην", _, "~ι~ησαῑμεν|~ι~ησαιημεν";
    aorist_opt_act_2: "~ι~ησαιης", "~ι~ησαῑτον|~ι~ησαιητον", "~ι~ησαῑτε|~ι~ησαιητε";
    aorist_opt_act_3: "~ι~ησαιη", "~ι~ησαιτην|~ι~ησαιητην", "~ι~ησαῑεν|~ι~ησαιησᾰν";
    aorist_opt_mid_1: "~ι~ησαιμην", _, "~ι~ησαιμεθᾰ";
    aorist_opt_mid_2: "~ι~ησαῑο", "~ι~ησαῑσθον", "~ι~ησαῑσθε";
    aorist_opt_mid_3: "~ι~ησαῑτο", "~ι~ησαισθην", "~ι~ησαῑντο";
    aorist_opt_pass_1: "~ι~ᾰθειην", _, "~ι~ᾰθεῑμεν|~ι~ᾰθειημεν";
    aorist_opt_pass_2: "~ι~ᾰθειης", "~ι~ᾰθεῑτον|~ι~ᾰθειητον", "~ι~ᾰθεῑτε|~ι~ᾰθειητε";
    aorist_opt_pass_3: "~ι~ᾰθειη", "~ι~ᾰθειτην|~ι~ᾰθειητην", "~ι~ᾰθεῑεν|~ι~ᾰθειησᾰν";
    aorist_imp_act_2: "~ι~ησηθῐ", "~ι~ησητον", "~ι~ησητε";
    aorist_imp_act_3: "~ι~ησητω", "~ι~ησητων", "~ι~ησᾰντων";
    aorist_imp_mid_2: "~ι~ησω", "~ι~ησᾰσθον", "~ι~ησᾰσθε";
    aorist_imp_mid_3: "~ι~ησᾰσθω", "~ι~ησᾰσθων", _;
    aorist_imp_pass_2: "~ι~ᾰθηθῐ", _, _;
    aorist_imp_pass_3: "~ι~ᾰθητω", "~ι~ᾰθητων", "~ι~ᾰθεντων";
    perfect_ind_act_1: _, _, "~ε~ᾰμεν";
    perfect_ind_act_2: _, "~ε~ᾰτον", "~ε~ᾰτε";
    perfect_ind_act_3: _, _, "~ε~ᾱσῐ|~ε~ᾱσῐν";
    perfect_subj_act_1: "~ε~ω", _, "~ε~ωμεν";
    perfect_subj_act_2: "~ε~ῃς", "~ε~ητον", "~ε~ητε";
    perfect_subj_act_3: "~ε~ῃ", _, "~ε~ωσῐ|~ε~ωσῐν";
    perfect_opt_act_1: "~ε~αιην", _, "~ε~αῑμεν|~ε~αιημεν";
    perfect_opt_act_2: "~ε~αιης", "~ε~αῑτον|~ε~αιητον", "~ε~αῑτε|~ε~αιητε";
    perfect_opt_act_3: "~ε~αιη", "~ε~αιτην|~ε~αιητην", "~ε~αῑεν|~ε~αιησᾰν";
    perfect_imp_act_2: "~ε~ᾰθῐ", "~ε~ᾰτον", "~ε~ᾰτε";
    perfect_imp_act_3: "~ε~ᾰτω", "~ε~ᾰτων", "~ε~ᾰντων";
    pluperfect_act_1: "~ει~ηκειν|~ει~ηκη", _, "~ει~ηκεμεν|~ε~ᾰμεν";
    pluperfect_act_2: "~ει~ηκεις|~ει~ηκης", "~ει~ηκετον|~ε~ᾰτον", "~ει~ηκετε|~ε~ᾰτε";
    pluperfect_act_3: "~ει~ηκει|~ει~ηκειν", "~ει~ηκετην|~ε~ᾰτην", "~ει~ηκεσᾰν|~ε~ᾰσᾰν";
    future_perfect_ind_act_1: "~ε~ηξω", _, "~ε~ηξομεν";
    future_perfect_ind_act_2: "~ε~ηξεις", "~ε~ηξετον", "~ε~ηξετε";
    future_perfect_ind_act_3: "~ε~ηξει", _, "~ε~ηξουσῐ|~ε~ηξουσῐν";
    future_perfect_ind_mid_1: "~ε~ηξομαι", _, "~ε~ηξομεθᾰ";
    future_perfect_ind_mid_2: "~ε~ηξῃ|~ε~ηξει", "~ε~ηξεσθον", "~ε~ηξεσθε";
    future_perfect_ind_mid_3: "~ε~ηξεται", _, "~ε~ηξονται";
    future_perfect_ind_pass_2: "~ε~ηξῃ|~ε~ηξει", _, _;
    future_perfect_opt_act_1: "~ε~ηξοιμῐ", _, "~ε~ηξοιμεν";
    future_perfect_opt_act_2: "~ε~ηξοις", "~ε~ηξοιτον", "~ε~ηξοιτε";
    future_perfect_opt_act_3: "~ε~ηξοι", "~ε~ηξοιτην", "~ε~ηξοιεν";
    future_perfect_opt_mid_1: "~ε~ηξοιμην", _, "~ε~ηξοιμεθᾰ";
    future_perfect_opt_mid_2: "~ε~ηξοιο", "~ε~ηξοισθον", "~ε~ηξοισθε";
    future_perfect_opt_mid_3: "~ε~ηξοιτο", "~ε~ηξοισθην", "~ε~ηξοιντο";
    infinitive_pres: "~ῑ~ᾰναι", "~ῑ~ᾰσθαι", _;
    infinitive_fut: "~ι~ησειν", "~ι~ησεσθαι", "~ῑ~ᾰθησεσθαι";
    infinitive_aor: "~ι~ησηναι", "~ι~ησᾰσθαι", "~ι~ᾰθηναι";
    infinitive_perf: "~ε~ᾰναι", _, _;
    infinitive_futp: "~ε~ηξειν", "~ε~ηξεσθαι", _;
    participle_pres: "~ῑ~ᾱς", "~ῑ~ᾰμενος", _;
    participle_fut: "~ι~ησων", "~ι~ησομενος", "~ῑ~ᾰθησομενος";
    participle_aor: "~ι~ησᾱς", "~ι~ησᾰμενος", "~ι~ᾰθεις";
    participle_perf: "~ε~ως", _, _;
    participle_futp: "~ε~ηξων", "~ε~ηξομενος", _;
    ),
    // Γανῠμαι: 1 words (γανῠμαι). Like Κᾰθημαι, except in 62 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("γανῠμαι", 1, "~α~ῠμαι", like Κᾰθημαι;
    ind_act_1: "~α~ῠμαι", _, _;
    ind_mid_1: "~ᾰ~ῠμαι", _, "~ᾰ~ῠμεθᾰ";
    ind_mid_2: "~ᾰ~ῠσαι", "~ᾰ~ῠσθον", "~ᾰ~ῠσθε";
    ind_mid_3: "~ᾰ~ῠται", _, "~ᾰ~ῠνται";
    subj_mid_1: "~ᾰ~ῠωμαι", _, "~ᾰ~ῠωμεθᾰ";
    subj_mid_2: "~ᾰ~ῠῃ", "~ᾰ~ῠησθον", "~ᾰ~ῠησθε";
    subj_mid_3: "~ᾰ~ῠηται", _, "~ᾰ~ῠωνται";
    opt_mid_1: "~ᾰ~ῠοιμην", _, "~ᾰ~ῠοιμεθᾰ";
    opt_mid_2: "~ᾰ~ῠοιο", "~ᾰ~ῠοισθον", "~ᾰ~ῠοισθε";
    opt_mid_3: "~ᾰ~ῠοιτο", "~ᾰ~ῠοισθην", "~ᾰ~ῠοιντο";
    imp_mid_2: "~ᾰ~ῠσο", _, "~ᾰ~ῠσθε";
    imp_mid_3: "~ᾰ~ῠσθω", "~ᾰ~ῠσθων", "~ᾰ~ῠσθων";
    imperfect_mid_1: "ἐ~ᾰ~ῠμην", _, "ἐ~ᾰ~ῠμεθᾰ";
    imperfect_mid_2: "ἐ~ᾰ~ῠσο", "ἐ~ᾰ~ῠσθον", "ἐ~ᾰ~ῠσθε";
    imperfect_mid_3: "ἐ~ᾰ~ῠτο", "ἐ~ᾰ~ῠσθην", "ἐ~ᾰ~ῠντο";
    perfect_ind_mid_1: "~εγᾰ~ῡμαι", _, "~εγᾰ~ῡμεθᾰ";
    perfect_ind_mid_2: "~εγᾰ~ῡσαι", "~εγᾰ~ῡσθον", "~εγᾰ~ῡσθε";
    perfect_ind_mid_3: "~εγᾰ~ῡται", _, "~εγᾰ~ῡνται";
    perfect_imp_mid_2: "~εγᾰ~ῡσο", _, _;
    perfect_imp_mid_3: "~εγᾰ~ῡσθω", "~εγᾰ~ῡσθων", _;
    pluperfect_mid_1: "ἐ~εγᾰ~ῡμην", _, "ἐ~εγᾰ~ῡμεθᾰ";
    pluperfect_mid_2: "ἐ~εγᾰ~ῡσο", "ἐ~εγᾰ~ῡσθον", "ἐ~εγᾰ~ῡσθε";
    pluperfect_mid_3: "ἐ~εγᾰ~ῡτο", "ἐ~εγᾰ~ῡσθην", "ἐ~εγᾰ~ῡντο";
    pluperfect_pass_1: derived, _, derived;
    pluperfect_pass_2: derived, derived, derived;
    pluperfect_pass_3: derived, derived, derived;
    ),
    // Δῐακειμαι: 1 words (δῐακειμαι). Like Οἰδᾰ, except in 43 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("δῐακειμαι", 1, "δῐα~ιμαι", like Οἰδᾰ;
    ind_act_1: "δῐα~ιμαι", _, _;
    ind_mid_1: "δῐα~ιμαι", _, "δῐᾰ~ιμεθα";
    ind_mid_2: "δῐα~ισαι", "δῐα~ισθον", "δῐα~ισθε";
    ind_mid_3: "δῐα~ιται", _, "δῐα~ινται";
    subj_mid_2: "δῐᾰ~ῃ", "δῐᾰ~ησθον", "δῐᾰ~ησθε";
    subj_mid_3: "δῐᾰ~ηται", _, "δῐᾰ~ωνται";
    imp_mid_2: "δῐα~ισο", _, "δῐα~ισθε";
    imp_mid_3: "~ισθω", "~ισθων", "~ισθων";
    imperfect_mid_1: "δῐε~ιμην", _, "δῐε~ιμεθα";
    imperfect_mid_2: "δῐε~ισο", "δῐε~ισθον", "δῐε~ισθε";
    imperfect_mid_3: "δῐε~ιτο", "δῐε~ισθην", "δῐε~ιντο";
    future_ind_mid_1: "δῐᾰ~ισομαι", _, "δῐᾰ~ισομεθᾰ";
    future_ind_mid_2: "δῐᾰ~ισῃ|δῐᾰ~ισει", "δῐᾰ~ισεσθον", "δῐᾰ~ισεσθε";
    future_ind_mid_3: "δῐᾰ~ισεται", _, "δῐᾰ~ισονται";
    future_opt_mid_1: "δῐᾰ~ισοιμην", _, "δῐᾰ~ισοιμεθᾰ";
    future_opt_mid_2: "δῐᾰ~ισοιο", "δῐᾰ~ισοισθον", "δῐᾰ~ισοισθε";
    future_opt_mid_3: "δῐᾰ~ισοιτο", "δῐᾰ~ισοισθην", "δῐᾰ~ισοιντο";
    infinitive_fut: _, "δῐᾰ~ισεσθαι", _;
    participle_fut: _, "δῐᾰ~ισομενος", _;
    ),
    // Ἀδησᾰ: 1 words (ἀδησᾰ). Like Ἐφᾰγον, except in 62 cells: ind_act_1 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_2 pl, ….
    conjugation!("ἀδησᾰ", 1, "~σᾰ", like Ἐφᾰγον;
    ind_act_1: "~σᾰ", _, _;
    aorist_ind_act_1: "~σᾰ", _, "~σᾰμεν";
    aorist_ind_act_2: "~σᾰς", "~σᾰτον", "~σᾰτε";
    aorist_ind_act_3: "~σε|~σεν", "~σᾰτην", "~σᾰν";
    aorist_subj_act_1: "~σω", _, "~σωμεν";
    aorist_subj_act_2: "~σῃς", "~σητον", "~σητε";
    aorist_subj_act_3: "~σῃ", _, "~σωσῐ|~σωσῐν";
    aorist_opt_act_1: "~σαιμῐ", _, "~σαιμεν";
    aorist_opt_act_2: "~σειᾰς|~σαις", "~σαιτον", "~σαιτε";
    aorist_opt_act_3: "~σειε|~σειεν|~σαι", "~σαιτην", "~σειᾰν|~σαιεν";
    aorist_imp_act_2: "~σον", "~σᾰτον", "~σᾰτε";
    aorist_imp_act_3: "~σᾰτω", "~σᾰτων", "~σᾰντων";
    perfect_ind_act_1: "~κᾰ", _, "~κᾰμεν";
    perfect_ind_act_2: "~κᾰς", "~κᾰτον", "~κᾰτε";
    perfect_ind_act_3: "~κε|~κεν", _, "~κᾱσῐ|~κᾱσῐν";
    perfect_subj_act_1: "~κω", _, "~κωμεν";
    perfect_subj_act_2: "~κῃς", "~κητον", "~κητε";
    perfect_subj_act_3: "~κῃ", _, "~κωσῐ|~κωσῐν";
    perfect_opt_act_1: "~κοιμῐ|~κοιην", _, "~κοιμεν";
    perfect_opt_act_2: "~κοις|~κοιης", "~κοιτον", "~κοιτε";
    perfect_opt_act_3: "~κοι|~κοιη", "~κοιτην", "~κοιεν";
    perfect_imp_act_2: "~κε", "~κετον", "~κετε";
    perfect_imp_act_3: "~κετω", "~κετων", "~κοντων";
    infinitive_aor: "~σαι", _, _;
    infinitive_perf: "~κεναι", _, _;
    participle_aor: "~σᾱς", _, _;
    participle_perf: "~κως", _, _;
    ),
    // Μειγνῡμῐ: 1 words (μειγνῡμῐ). Like Δεικνῡμῐ, except in 176 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("μειγνῡμῐ", 1, "~ιγνῡμῐ", like Δεικνῡμῐ;
    ind_act_1: "~ιγνῡμῐ", _, "~ιγνῠμεν";
    ind_act_2: "~ιγνῡς", _, _;
    ind_act_3: "~ιγνῡσῐ|~ιγνῡσῐν", _, "~ιγνῠᾱσῐ|~ιγνῠᾱσῐν";
    ind_mid_1: "~ιγνῠμαι", _, "~ιγνῠμεθᾰ";
    ind_mid_2: "~ιγνῠσαι", "~ιγνῠσθον", "~ιγνῠσθε";
    ind_mid_3: "~ιγνῠται", _, "~ιγνῠνται";
    ind_pass_1: _, _, "~ιγνῠμεθᾰ";
    ind_pass_2: _, _, "~ιγνῠσθε";
    ind_pass_3: "~ιγνῠται", _, "~ιγνῠνται";
    subj_act_1: "~ιγνῠω", _, "~ιγνῠωμεν";
    subj_act_2: "~ιγνῠῃς", "~ιγνῠητον", "~ιγνῠητε";
    subj_act_3: "~ιγνῠῃ", _, "~ιγνῠωσῐ|~ιγνῠωσῐν";
    subj_mid_1: "~ιγνῠωμαι", _, "~ιγνῠωμεθᾰ";
    subj_mid_2: "~ιγνῠῃ", "~ιγνῠησθον", "~ιγνῠησθε";
    subj_mid_3: "~ιγνῠηται", _, "~ιγνῠωνται";
    subj_pass_1: _, _, "~ιγνῠωμεθᾰ";
    subj_pass_2: _, _, "~ιγνῠησθε";
    subj_pass_3: "~ιγνῠηται", _, "~ιγνῠωνται";
    opt_act_1: "~ιγνῠοιμῐ", _, "~ιγνῠοιμεν";
    opt_act_2: "~ιγνῠοις", "~ιγνῠοιτον", "~ιγνῠοιτε";
    opt_act_3: "~ιγνῠοι", "~ιγνῠοιτην", "~ιγνῠοιεν";
    opt_mid_1: "~ιγνῠοιμην", _, "~ιγνῠοιμεθᾰ";
    opt_mid_2: "~ιγνῠοιο", "~ιγνῠοισθον", "~ιγνῠοισθε";
    opt_mid_3: "~ιγνῠοιτο", "~ιγνῠοισθην", "~ιγνῠοιντο";
    opt_pass_1: _, _, "~ιγνῠοιμεθᾰ";
    opt_pass_2: _, "~ιγνῠοισθον", "~ιγνῠοισθε";
    opt_pass_3: "~ιγνῠοιτο", "~ιγνῠοισθην", "~ιγνῠοιντο";
    imp_act_2: "~ιγνῡ", "~ιγνῠτον", "~ιγνῠτε";
    imp_act_3: "~ιγνῠτω", "~ιγνῠτων", "~ιγνῠντων";
    imp_mid_2: "~ιγνῠσο", _, "~ιγνῠσθε";
    imp_mid_3: "~ιγνῠσθω", "~ιγνῠσθων", "~ιγνῠσθων";
    imp_pass_2: _, _, "~ιγνῠσθε";
    imp_pass_3: _, "~ιγνῠσθων", _;
    imperfect_act_1: "ἐ~ιγνῡν", _, "ἐ~ιγνῠμεν";
    imperfect_act_2: "ἐ~ιγνῡς", "ἐ~ιγνῠτον", "ἐ~ιγνῠτε";
    imperfect_act_3: "ἐ~ιγνῡ", "ἐ~ιγνῠτην", "ἐ~ιγνῠσᾰν";
    imperfect_mid_1: "ἐ~ιγνῠμην", _, "ἐ~ιγνῠμεθᾰ";
    imperfect_mid_2: "ἐ~ιγνῠσο", "ἐ~ιγνῠσθον", "ἐ~ιγνῠσθε";
    imperfect_mid_3: "ἐ~ιγνῠτο", "ἐ~ιγνῠσθην", "ἐ~ιγνῠντο";
    imperfect_pass_1: _, _, "ἐ~ιγνῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ιγνῠσθε";
    imperfect_pass_3: "ἐ~ιγνῠτο", _, _;
    perfect_ind_act_1: "~μειχᾰ", _, "~μειχᾰμεν";
    perfect_ind_act_2: "~μειχᾰς", "~μειχᾰτον", "~μειχᾰτε";
    perfect_ind_act_3: "~μειχε|~μειχεν", _, "~μειχᾱσῐ|~μειχᾱσῐν";
    perfect_ind_mid_1: "~μειγμαι", _, "~μειγμεθᾰ";
    perfect_ind_mid_2: "~μειξαι", "~μειχθον", "~μειχθε";
    perfect_ind_mid_3: "~μεικται", _, "~μειγᾰται";
    perfect_ind_pass_1: _, _, "~μειγμεθᾰ";
    perfect_ind_pass_2: _, _, "~μειχθε";
    perfect_subj_act_1: "~μειχω", _, "~μειχωμεν";
    perfect_subj_act_2: "~μειχῃς", "~μειχητον", "~μειχητε";
    perfect_subj_act_3: "~μειχῃ", _, "~μειχωσῐ|~μειχωσῐν";
    perfect_opt_act_1: "~μειχοιμῐ|~μειχοιην", _, "~μειχοιμεν";
    perfect_opt_act_2: "~μειχοις|~μειχοιης", "~μειχοιτον", "~μειχοιτε";
    perfect_opt_act_3: "~μειχοι|~μειχοιη", "~μειχοιτην", "~μειχοιεν";
    perfect_imp_act_2: "~μειχε", "~μειχετον", "~μειχετε";
    perfect_imp_act_3: "~μειχετω", "~μειχετων", "~μειχοντων";
    perfect_imp_mid_2: "~μειξο", _, _;
    perfect_imp_mid_3: "~μειχθω", "~μειχθων", _;
    perfect_imp_pass_2: "~μειξο", _, _;
    perfect_imp_pass_3: "~μειχθω", "~μειχθων", _;
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
    future_perfect_ind_mid_1: derived, _, derived;
    future_perfect_ind_mid_2: derived, derived, derived;
    future_perfect_ind_mid_3: derived, _, derived;
    future_perfect_ind_pass_2: derived, _, _;
    future_perfect_opt_mid_1: derived, _, derived;
    future_perfect_opt_mid_2: derived, derived, derived;
    future_perfect_opt_mid_3: derived, derived, derived;
    infinitive_pres: "~ιγνῠναι", "~ιγνῠσθαι", _;
    infinitive_perf: "~μειχεναι", "~μεῑχθαι", _;
    infinitive_futp: _, derived, _;
    participle_pres: "~ιγνῡς", "~ιγνῠμενος", _;
    participle_perf: "~μειχως", "~μειγμενος", _;
    participle_futp: _, derived, _;
    ),
    // Ἐμπηγνῡμῐ: 1 words (ἐμπηγνῡμῐ). Like Ξῠρρηγνῡμῐ, except in 270 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἐμπηγνῡμῐ", 1, "~μ~ηγνῡμῐ", like Ξῠρρηγνῡμῐ;
    ind_act_1: "~μ~ηγνῡμῐ", _, "~μ~ηγνῠμεν";
    ind_act_2: "~μ~ηγνῡς", _, _;
    ind_act_3: "~μ~ηγνῡσῐ|~μ~ηγνῡσῐν", _, "~μ~ηγνῠᾱσῐ|~μ~ηγνῠᾱσῐν";
    ind_mid_1: "~μ~ηγνῠμαι", _, "~μ~ηγνῠμεθᾰ";
    ind_mid_2: "~μ~ηγνῠσαι", "~μ~ηγνῠσθον", "~μ~ηγνῠσθε";
    ind_mid_3: "~μ~ηγνῠται", _, "~μ~ηγνῠνται";
    ind_pass_1: _, _, "~μ~ηγνῠμεθᾰ";
    ind_pass_2: _, _, "~μ~ηγνῠσθε";
    ind_pass_3: "~μ~ηγνῠται", _, "~μ~ηγνῠνται";
    subj_act_1: "~μ~ηγνῠω", _, "~μ~ηγνῠωμεν";
    subj_act_2: "~μ~ηγνῠῃς", "~μ~ηγνῠητον", "~μ~ηγνῠητε";
    subj_act_3: "~μ~ηγνῠῃ", _, "~μ~ηγνῠωσῐ|~μ~ηγνῠωσῐν";
    subj_mid_1: "~μ~ηγνῠωμαι", _, "~μ~ηγνῠωμεθᾰ";
    subj_mid_2: "~μ~ηγνῠῃ", "~μ~ηγνῠησθον", "~μ~ηγνῠησθε";
    subj_mid_3: "~μ~ηγνῠηται", _, "~μ~ηγνῠωνται";
    subj_pass_1: _, _, "~μ~ηγνῠωμεθᾰ";
    subj_pass_2: _, _, "~μ~ηγνῠησθε";
    subj_pass_3: "~μ~ηγνῠηται", _, "~μ~ηγνῠωνται";
    opt_act_1: "~μ~ηγνῠοιμῐ", _, "~μ~ηγνῠοιμεν";
    opt_act_2: "~μ~ηγνῠοις", "~μ~ηγνῠοιτον", "~μ~ηγνῠοιτε";
    opt_act_3: "~μ~ηγνῠοι", "~μ~ηγνῠοιτην", "~μ~ηγνῠοιεν";
    opt_mid_1: "~μ~ηγνῠοιμην", _, "~μ~ηγνῠοιμεθᾰ";
    opt_mid_2: "~μ~ηγνῠοιο", "~μ~ηγνῠοισθον", "~μ~ηγνῠοισθε";
    opt_mid_3: "~μ~ηγνῠοιτο", "~μ~ηγνῠοισθην", "~μ~ηγνῠοιντο";
    opt_pass_1: _, _, "~μ~ηγνῠοιμεθᾰ";
    opt_pass_2: _, "~μ~ηγνῠοισθον", "~μ~ηγνῠοισθε";
    opt_pass_3: "~μ~ηγνῠοιτο", "~μ~ηγνῠοισθην", "~μ~ηγνῠοιντο";
    imp_act_2: "~μ~ηγνῡ", "~μ~ηγνῠτον", "~μ~ηγνῠτε";
    imp_act_3: "~μ~ηγνῠτω", "~μ~ηγνῠτων", "~μ~ηγνῠντων";
    imp_mid_2: "~μ~ηγνῠσο", _, "~μ~ηγνῠσθε";
    imp_mid_3: "~μ~ηγνῠσθω", "~μ~ηγνῠσθων", "~μ~ηγνῠσθων";
    imp_pass_2: _, _, "~μ~ηγνῠσθε";
    imp_pass_3: _, "~μ~ηγνῠσθων", _;
    future_ind_act_1: "~μ~ηξω", _, "~μ~ηξομεν";
    future_ind_act_2: "~μ~ηξεις", "~μ~ηξετον", "~μ~ηξετε";
    future_ind_act_3: "~μ~ηξει", _, "~μ~ηξουσῐ|~μ~ηξουσῐν";
    future_ind_mid_1: "~μ~ηξομαι", _, "~μ~ηξομεθᾰ";
    future_ind_mid_2: "~μ~ηξῃ|~μ~ηξει", "~μ~ηξεσθον", "~μ~ηξεσθε";
    future_ind_mid_3: "~μ~ηξεται", _, "~μ~ηξονται";
    future_opt_act_1: "~μ~ηξοιμῐ", _, "~μ~ηξοιμεν";
    future_opt_act_2: "~μ~ηξοις", "~μ~ηξοιτον", "~μ~ηξοιτε";
    future_opt_act_3: "~μ~ηξοι", "~μ~ηξοιτην", "~μ~ηξοιεν";
    future_opt_mid_1: "~μ~ηξοιμην", _, "~μ~ηξοιμεθᾰ";
    future_opt_mid_2: "~μ~ηξοιο", "~μ~ηξοισθον", "~μ~ηξοισθε";
    future_opt_mid_3: "~μ~ηξοιτο", "~μ~ηξοισθην", "~μ~ηξοιντο";
    aorist_ind_pass_1: "~νε~αγην|~νε~ηχθην", _, "~νε~αγημεν|~νε~ηχθημεν";
    aorist_ind_pass_2: "~νε~αγης|~νε~ηχθης", "~νε~αγητον|~νε~ηχθητον", "~νε~αγητε|~νε~ηχθητε";
    aorist_ind_pass_3: "~νε~αγη|~νε~ηχθη", "~νε~αγητην|~νε~ηχθητην", "~νε~αγησᾰν|~νε~ηχθησᾰν";
    aorist_subj_act_1: "~νε~ηξω", _, "~νε~ηξωμεν";
    aorist_subj_act_2: "~νε~ηξῃς", "~νε~ηξητον", "~νε~ηξητε";
    aorist_subj_act_3: "~νε~ηξῃ", _, "~νε~ηξωσῐ|~νε~ηξωσῐν";
    aorist_subj_mid_1: "~νε~ηξωμαι", _, "~νε~ηξωμεθᾰ";
    aorist_subj_mid_2: "~νε~ηξῃ", "~νε~ηξησθον", "~νε~ηξησθε";
    aorist_subj_mid_3: "~νε~ηξηται", _, "~νε~ηξωνται";
    aorist_subj_pass_1: "~μ~αγω|~μ~ηχθω", _, "~μ~αγωμεν|~μ~ηχθωμεν";
    aorist_subj_pass_2: "~μ~αγῃς|~μ~ηχθῃς", "~μ~αγητον|~μ~ηχθητον", "~μ~αγητε|~μ~ηχθητε";
    aorist_subj_pass_3: "~μ~αγῃ|~μ~ηχθῃ", "~μ~αγητον|~μ~ηχθητον", "~μ~αγωσῐ|~μ~αγωσῐν|~μ~ηχθωσῐ|~μ~ηχθωσῐν";
    aorist_opt_act_1: "~νε~ηξαιμῐ", _, "~νε~ηξαιμεν";
    aorist_opt_act_2: "~νε~ηξειᾰς|~νε~ηξαις", "~νε~ηξαιτον", "~νε~ηξαιτε";
    aorist_opt_act_3: "~νε~ηξειε|~νε~ηξειεν|~νε~ηξαι", "~νε~ηξαιτην", "~νε~ηξειᾰν|~νε~ηξαιεν";
    aorist_opt_mid_1: "~νε~ηξαιμην", _, "~νε~ηξαιμεθᾰ";
    aorist_opt_mid_2: "~νε~ηξαιο", "~νε~ηξαισθον", "~νε~ηξαισθε";
    aorist_opt_mid_3: "~νε~ηξαιτο", "~νε~ηξαισθην", "~νε~ηξαιντο";
    aorist_opt_pass_1: "~μ~αγειην|~μ~ηχθειην", _, "~μ~αγεῑμεν|~μ~αγειημεν|~μ~ηχθεῑμεν|~μ~ηχθειημεν";
    aorist_opt_pass_2: "~μ~αγειης|~μ~ηχθειης", "~μ~αγεῑτον|~μ~αγειητον|~μ~ηχθεῑτον|~μ~ηχθειητον", "~μ~αγεῑτε|~μ~αγειητε|~μ~ηχθεῑτε|~μ~ηχθειητε";
    aorist_opt_pass_3: "~μ~αγειη|~μ~ηχθειη", "~μ~αγειτην|~μ~αγειητην|~μ~ηχθειτην|~μ~ηχθειητην", "~μ~αγεῑεν|~μ~αγειησᾰν|~μ~ηχθεῑεν|~μ~ηχθειησᾰν";
    aorist_imp_act_2: "~νε~ηξον", "~νε~ηξᾰτον", "~νε~ηξᾰτε";
    aorist_imp_act_3: "~νε~ηξᾰτω", "~νε~ηξᾰτων", "~νε~ηξᾰντων";
    aorist_imp_mid_2: "~νε~ηξαι", "~νε~ηξᾰσθον", "~νε~ηξᾰσθε";
    aorist_imp_mid_3: "~νε~ηξᾰσθω", "~νε~ηξᾰσθων", _;
    aorist_imp_pass_2: "~μ~αγηθῐ|~μ~ηχθητῐ", "~μ~αγητον|~μ~ηχθητον", "~μ~αγητε|~μ~ηχθητε";
    aorist_imp_pass_3: "~μ~αγητω|~μ~ηχθητω", "~μ~αγητων|~μ~ηχθητων", "~μ~αγεντων|~μ~ηχθεντων";
    perfect_ind_act_1: "~μ~επηγᾰ|~μ~επηχᾰ", _, "~μ~επηγᾰμεν|~μ~επηχᾰμεν";
    perfect_ind_act_2: "~μ~επηγᾰς|~μ~επηχᾰς", "~μ~επηγᾰτον|~μ~επηχᾰτον", "~μ~επηγᾰτε|~μ~επηχᾰτε";
    perfect_ind_act_3: "~μ~επηγε|~μ~επηγεν|~μ~επηχε|~μ~επηχεν", "~μ~επηγᾰτον|~μ~επηχᾰτον", "~μ~επηγᾱσῐ|~μ~επηγᾱσῐν|~μ~επηχᾱσῐ|~μ~επηχᾱσῐν";
    perfect_ind_mid_1: "~μ~επηγμαι", _, "~μ~επηγμεθᾰ";
    perfect_ind_mid_2: "~μ~επηξαι", "~μ~επηχθον", "~μ~επηχθε";
    perfect_ind_mid_3: "~μ~επηκται", _, "~μ~επηγᾰται";
    perfect_ind_pass_1: _, _, "~μ~επηγμεθᾰ";
    perfect_ind_pass_2: _, _, "~μ~επηχθε";
    perfect_subj_act_1: "~μ~επηγω|~μ~επηχω", _, "~μ~επηγωμεν|~μ~επηχωμεν";
    perfect_subj_act_2: "~μ~επηγῃς|~μ~επηχῃς", "~μ~επηγητον|~μ~επηχητον", "~μ~επηγητε|~μ~επηχητε";
    perfect_subj_act_3: "~μ~επηγῃ|~μ~επηχῃ", "~μ~επηγητον|~μ~επηχητον", "~μ~επηγωσῐ|~μ~επηγωσῐν|~μ~επηχωσῐ|~μ~επηχωσῐν";
    perfect_opt_act_1: "~μ~επηγοιμῐ|~μ~επηγοιην|~μ~επηχοιμῐ|~μ~επηχοιην", _, "~μ~επηγοιμεν|~μ~επηχοιμεν";
    perfect_opt_act_2: "~μ~επηγοις|~μ~επηγοιης|~μ~επηχοις|~μ~επηχοιης", "~μ~επηγοιτον|~μ~επηχοιτον", "~μ~επηγοιτε|~μ~επηχοιτε";
    perfect_opt_act_3: "~μ~επηγοι|~μ~επηγοιη|~μ~επηχοι|~μ~επηχοιη", "~μ~επηγοιτην|~μ~επηχοιτην", "~μ~επηγοιεν|~μ~επηχοιεν";
    perfect_imp_act_2: "~μ~επηγε|~μ~επηχε", "~μ~επηγετον|~μ~επηχετον", "~μ~επηγετε|~μ~επηχετε";
    perfect_imp_act_3: "~μ~επηγετω|~μ~επηχετω", "~μ~επηγετων|~μ~επηχετων", "~μ~επηγοντων|~μ~επηχοντων";
    perfect_imp_mid_2: "~μ~επηξο", _, _;
    perfect_imp_mid_3: "~μ~επηχθω", "~μ~επηχθων", _;
    perfect_imp_pass_2: "~μ~επηξο", _, _;
    perfect_imp_pass_3: "~μ~επηχθω", "~μ~επηχθων", _;
    pluperfect_act_1: "~νε~επηγειν|~νε~επηγη|~νε~επηχειν|~νε~επηχη", _, "~νε~επηγεμεν|~νε~επηχεμεν";
    pluperfect_act_2: "~νε~επηγεις|~νε~επηγης|~νε~επηχεις|~νε~επηχης", "~νε~επηγετον|~νε~επηχετον", "~νε~επηγετε|~νε~επηχετε";
    pluperfect_act_3: "~νε~επηγει|~νε~επηγειν|~νε~επηχει|~νε~επηχειν", "~νε~επηγετην|~νε~επηχετην", "~νε~επηγεσᾰν|~νε~επηχεσᾰν";
    pluperfect_mid_1: derived, _, derived;
    pluperfect_mid_2: derived, derived, derived;
    pluperfect_mid_3: derived, derived, _;
    pluperfect_pass_1: _, _, derived;
    pluperfect_pass_2: _, _, derived;
    pluperfect_pass_3: _, derived, derived;
    future_perfect_ind_mid_1: "~μ~αγησομαι", _, "~μ~αγησομεθᾰ";
    future_perfect_ind_mid_2: "~μ~αγησῃ|~μ~αγησει", "~μ~αγησεσθον", "~μ~αγησεσθε";
    future_perfect_ind_mid_3: "~μ~αγησεται", _, "~μ~αγησονται";
    future_perfect_ind_pass_2: "~μ~αγησῃ|~μ~αγησει", _, _;
    future_perfect_opt_mid_1: "~μ~αγησοιμην", _, "~μ~αγησοιμεθᾰ";
    future_perfect_opt_mid_2: "~μ~αγησοιο", "~μ~αγησοισθον", "~μ~αγησοισθε";
    future_perfect_opt_mid_3: "~μ~αγησοιτο", "~μ~αγησοισθην", "~μ~αγησοιντο";
    infinitive_pres: "~μ~ηγνῠναι", "~μ~ηγνῠσθαι", _;
    infinitive_fut: "~μ~ηξειν", "~μ~ηξεσθαι", _;
    infinitive_aor: "~νε~ηξαι", "~νε~ηξᾰσθαι", "~μ~αγηναι|~μ~ηχθηναι";
    infinitive_perf: "~μ~επηγεναι|~μ~επηχεναι", "~μ~επηχθαι", _;
    infinitive_futp: _, "~μ~αγησεσθαι", _;
    participle_pres: "~μ~ηγνῡς", "~μ~ηγνῠμενος", _;
    participle_fut: "~μ~ηξων", "~μ~ηξομενος", _;
    participle_aor: "~νε~ηξᾱς", "~νε~ηξᾰμενος", "~μ~αγεις|~μ~ηχθεις";
    participle_perf: "~μ~επηγως|~μ~επηχως", "~μ~επηγμενος", _;
    participle_futp: _, "~μ~αγησομενος", _;
    ),
    // Ὑπεκτιθεμαι: 1 words (ὑπεκτιθεμαι). Like Προσερχομαι, except in 168 cells: ind_act_1 sg, ind_mid_1 sg, ind_mid_1 pl, ind_mid_2 sg, ind_mid_2 du, ind_mid_2 pl, ….
    conjugation!("ὑπεκτιθεμαι", 1, "~κτι~εμαι", like Προσερχομαι;
    ind_act_1: "~κτι~εμαι", _, _;
    ind_mid_1: "~κ%ῐ~εμαι", _, "~κ%ῐ~εμεθᾰ";
    ind_mid_2: "~κ%ῐ~εσαι", "~κ%ῐ~εσθον", "~κ%ῐ~εσθε";
    ind_mid_3: "~κ%ῐ~εται", _, "~κ%ῐ~ενται";
    ind_pass_1: _, _, "~κ%ῐ~εμεθᾰ";
    ind_pass_2: _, _, "~κ%ῐ~εσθε";
    ind_pass_3: "~κ%ῐ~εται", _, "~κ%ῐ~ενται";
    subj_mid_1: "~κ%ῐ~ωμαι", _, "~κ%ῐ~ωμεθᾰ";
    subj_mid_2: "~κ%ῐ~ῃ", "~κ%ῐ~ησθον", "~κ%ῐ~ησθε";
    subj_mid_3: "~κ%ῐ~ηται", _, "~κ%ῐ~ωνται";
    subj_pass_1: _, _, "~κ%ῐ~ωμεθᾰ";
    subj_pass_2: _, _, "~κ%ῐ~ησθε";
    subj_pass_3: "~κ%ῐ~ηται", _, "~κ%ῐ~ωνται";
    opt_mid_1: "~κ%ῐ~ειμην", _, "~κ%ῐ~ειμεθᾰ|~κ%ῐ~οιμεθᾰ";
    opt_mid_2: "~κ%ῐ~εῑο", "~κ%ῐ~εῑσθον|~κ%ῐ~οῑσθον", "~κ%ῐ~εῑσθε|~κ%ῐ~οῑσθε";
    opt_mid_3: "~κ%ῐ~εῑτο|~κ%ῐ~οῑτο", "~κ%ῐ~εισθην|~κ%ῐ~οισθην", "~κ%ῐ~εῑντο|~κ%ῐ~οῑντο";
    opt_pass_1: _, _, "~κ%ῐ~ειμεθᾰ|~κ%ῐ~οιμεθᾰ";
    opt_pass_2: _, "~κ%ῐ~εῑσθον|~κ%ῐ~οῑσθον", "~κ%ῐ~εῑσθε|~κ%ῐ~οῑσθε";
    opt_pass_3: "~κ%ῐ~εῑτο|~κ%ῐ~οῑτο", "~κ%ῐ~εισθην|~κ%ῐ~οισθην", "~κ%ῐ~εῑντο|~κ%ῐ~οῑντο";
    imp_mid_2: "~κ%ῐ~εσο|~κ%ῐ~ου", _, "~κ%ῐ~εσθε";
    imp_mid_3: "~κ%ῐ~εσθω", "~κ%ῐ~εσθων", "~κ%ῐ~εσθων";
    imp_pass_2: "~κ%ῐ~εσο|~κ%ῐ~ου", _, "~κ%ῐ~εσθε";
    imp_pass_3: _, "~κ%ῐ~εσθων", _;
    imperfect_mid_1: "~ξε%ῐ~εμην", _, "~ξε%ῐ~εμεθᾰ";
    imperfect_mid_2: "~ξε%ῐ~εσο", "~ξε%ῐ~εσθον", "~ξε%ῐ~εσθε";
    imperfect_mid_3: "~ξε%ῐ~ετο", "~ξε%ῐ~εσθην", "~ξε%ῐ~εντο";
    imperfect_pass_1: _, _, "~ξε%ῐ~εμεθᾰ";
    imperfect_pass_2: _, _, "~ξε%ῐ~εσθε";
    imperfect_pass_3: "~ξε%ῐ~ετο", _, _;
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
    aorist_ind_mid_1: "~ξε~εμην", _, "~ξε~εμεθᾰ";
    aorist_ind_mid_2: "~ξε~ου", "~ξε~εσθον", "~ξε~εσθε";
    aorist_ind_mid_3: "~ξε~ετο", "~ξε~εσθην", "~ξε~εντο";
    aorist_ind_pass_1: "~ξετε~ην", _, "~ξετε~ημεν";
    aorist_ind_pass_2: "~ξετε~ης", "~ξετε~ητον", "~ξετε~ητε";
    aorist_ind_pass_3: "~ξετε~η", "~ξετε~ητην", "~ξετε~ησᾰν";
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
    perfect_ind_pass_1: _, _, "~κ%ε~ειμεθᾰ";
    perfect_ind_pass_2: _, _, "~κ%ε~εισθε";
    perfect_imp_mid_2: "~κ%ε~εισο", _, _;
    perfect_imp_mid_3: "~κ%ε~εισθω", "~κ%ε~εισθων", _;
    perfect_imp_pass_2: "~κ%ε~εισο", _, _;
    perfect_imp_pass_3: "~κ%ε~εισθω", "~κ%ε~εισθων", _;
    infinitive_pres: _, "~κ%ῐ~εσθαι", _;
    infinitive_fut: _, "~κ~ησεσθαι", "~κ%ε~ησεσθαι";
    infinitive_aor: _, "~κ~εσθαι", "~κτε~ηναι";
    infinitive_perf: _, "~κ%ε~εῑσθαι", _;
    participle_pres: _, "~κ%ῐ~εμενος", _;
    participle_fut: _, "~κ~ησομενος", "~κ%ε~ησομενος";
    participle_aor: _, "~κ~εμενος", "~κτε~εις";
    participle_perf: _, "~κ%ε~ειμενος", _;
    ),
    // Ἐκτιθημῐ: 1 words (ἐκτιθημῐ). Like Ὑπεκτιθεμαι, except in 187 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("ἐκτιθημῐ", 1, "~κτι~ημῐ", like Ὑπεκτιθεμαι;
    ind_act_1: "~κτι~ημῐ", _, "~κτι~εμεν";
    ind_act_2: "~κτι~ης", _, _;
    ind_act_3: "~κτι~ησῐ|~κτι~ησῐν", _, "~κτι~εᾱσῐ|~κτι~εᾱσῐν";
    ind_mid_1: "~κτι~εμαι", _, "~κτι~εμεθᾰ";
    ind_mid_2: "~κτι~εσαι", "~κτι~εσθον", "~κτι~εσθε";
    ind_mid_3: "~κτι~εται", _, "~κτι~ενται";
    ind_pass_1: _, _, "~κτι~εμεθᾰ";
    ind_pass_2: _, _, "~κτι~εσθε";
    ind_pass_3: "~κτι~εται", _, "~κτι~ενται";
    subj_act_1: "~κτι~ω", _, "~κτι~ωμεν";
    subj_act_2: "~κτι~ῃς", "~κτι~ητον", "~κτι~ητε";
    subj_act_3: "~κτι~ῃ", _, "~κτι~ωσῐ|~κτι~ωσῐν";
    subj_mid_1: "~κτι~ωμαι", _, "~κτι~ωμεθᾰ";
    subj_mid_2: "~κτι~ῃ", "~κτι~ησθον", "~κτι~ησθε";
    subj_mid_3: "~κτι~ηται", _, "~κτι~ωνται";
    subj_pass_1: _, _, "~κτι~ωμεθᾰ";
    subj_pass_2: _, _, "~κτι~ησθε";
    subj_pass_3: "~κτι~ηται", _, "~κτι~ωνται";
    opt_act_1: "~κτι~ειην", _, "~κτι~εῑμεν|~κτι~ειημεν";
    opt_act_2: "~κτι~ειης", "~κτι~εῑτον|~κτι~ειητον", "~κτι~εῑτε|~κτι~ειητε";
    opt_act_3: "~κτι~ειη", "~κτι~ειτην|~κτι~ειητην", "~κτι~εῑεν|~κτι~ειησᾰν";
    opt_mid_1: "~κτι~ειμην", _, "~κτι~ειμεθᾰ|~κτι~οιμεθᾰ";
    opt_mid_2: "~κτι~εῑο", "~κτι~εῑσθον|~κτι~οῑσθον", "~κτι~εῑσθε|~κτι~οῑσθε";
    opt_mid_3: "~κτι~εῑτο|~κτι~οῑτο", "~κτι~εισθην|~κτι~οισθην", "~κτι~εῑντο|~κτι~οῑντο";
    opt_pass_1: _, _, "~κτι~ειμεθᾰ|~κτι~οιμεθᾰ";
    opt_pass_2: _, "~κτι~εῑσθον|~κτι~οῑσθον", "~κτι~εῑσθε|~κτι~οῑσθε";
    opt_pass_3: "~κτι~εῑτο|~κτι~οῑτο", "~κτι~εισθην|~κτι~οισθην", "~κτι~εῑντο|~κτι~οῑντο";
    imp_act_2: "~κτι~ει", "~κτι~ετον", "~κτι~ετε";
    imp_act_3: "~κτι~ετω", "~κτι~ετων", "~κτι~εντων";
    imp_mid_2: "~κτι~εσο|~κτι~ου", _, "~κτι~εσθε";
    imp_mid_3: "~κτι~εσθω", "~κτι~εσθων", "~κτι~εσθων";
    imp_pass_2: "~κτι~εσο|~κτι~ου", _, "~κτι~εσθε";
    imp_pass_3: _, "~κτι~εσθων", _;
    imperfect_act_1: "~ξετι~ην", _, "~ξετι~εμεν";
    imperfect_act_2: "~ξετι~εις", "~ξετι~ετον", "~ξετι~ετε";
    imperfect_act_3: "~ξετι~ει", "~ξετι~ετην", "~ξετι~εσᾰν";
    imperfect_mid_1: "~ξετι~εμην", _, "~ξετι~εμεθᾰ";
    imperfect_mid_2: "~ξετι~εσο", "~ξετι~εσθον", "~ξετι~εσθε";
    imperfect_mid_3: "~ξετι~ετο", "~ξετι~εσθην", "~ξετι~εντο";
    imperfect_pass_1: _, _, "~ξετι~εμεθᾰ";
    imperfect_pass_2: _, _, "~ξετι~εσθε";
    imperfect_pass_3: "~ξετι~ετο", _, _;
    future_ind_act_1: "~κ~ησω", _, "~κ~ησομεν";
    future_ind_act_2: "~κ~ησεις", "~κ~ησετον", "~κ~ησετε";
    future_ind_act_3: "~κ~ησει", _, "~κ~ησουσῐ|~κ~ησουσῐν";
    future_opt_act_1: "~κ~ησοιμῐ", _, "~κ~ησοιμεν";
    future_opt_act_2: "~κ~ησοις", "~κ~ησοιτον", "~κ~ησοιτε";
    future_opt_act_3: "~κ~ησοι", "~κ~ησοιτην", "~κ~ησοιεν";
    aorist_ind_act_1: _, _, "~ξε~εμεν";
    aorist_ind_act_2: _, "~ξε~ετον", "~ξε~ετε";
    aorist_ind_act_3: _, "~ξε~ετην", "~ξε~εσᾰν";
    aorist_subj_act_1: "~κ~ω", _, "~κ~ωμεν";
    aorist_subj_act_2: "~κ~ῃς", "~κ~ητον", "~κ~ητε";
    aorist_subj_act_3: "~κ~ῃ", _, "~κ~ωσῐ|~κ~ωσῐν";
    aorist_opt_act_1: "~κ~ειην", _, "~κ~εῑμεν|~κ~ειημεν";
    aorist_opt_act_2: "~κ~ειης", "~κ~εῑτον|~κ~ειητον", "~κ~εῑτε|~κ~ειητε";
    aorist_opt_act_3: "~κ~ειη", "~κ~ειτην|~κ~ειητην", "~κ~εῑεν|~κ~ειησᾰν";
    aorist_imp_act_2: "~κ~ες", "~κ~ετον", "~κ~ετε";
    aorist_imp_act_3: "~κ~ετω", "~κ~ετων", "~κ~εντων";
    perfect_ind_act_1: "~κ%ε~εικᾰ", _, "~κ%ε~εικᾰμεν";
    perfect_ind_act_2: "~κ%ε~εικᾰς", "~κ%ε~εικᾰτον", "~κ%ε~εικᾰτε";
    perfect_ind_act_3: "~κ%ε~εικε|~κ%ε~εικεν", _, "~κ%ε~εικᾱσῐ|~κ%ε~εικᾱσῐν";
    perfect_subj_act_1: "~κ%ε~εικω", _, "~κ%ε~εικωμεν";
    perfect_subj_act_2: "~κ%ε~εικῃς", "~κ%ε~εικητον", "~κ%ε~εικητε";
    perfect_subj_act_3: "~κ%ε~εικῃ", _, "~κ%ε~εικωσῐ|~κ%ε~εικωσῐν";
    perfect_opt_act_1: "~κ%ε~εικοιμῐ|~κ%ε~εικοιην", _, "~κ%ε~εικοιμεν";
    perfect_opt_act_2: "~κ%ε~εικοις|~κ%ε~εικοιης", "~κ%ε~εικοιτον", "~κ%ε~εικοιτε";
    perfect_opt_act_3: "~κ%ε~εικοι|~κ%ε~εικοιη", "~κ%ε~εικοιτην", "~κ%ε~εικοιεν";
    perfect_imp_act_2: "~κ%ε~εικε", "~κ%ε~εικετον", "~κ%ε~εικετε";
    perfect_imp_act_3: "~κ%ε~εικετω", "~κ%ε~εικετων", "~κ%ε~εικοντων";
    pluperfect_act_1: "~ξε%ε~εικειν|~ξε%ε~εικη", _, "~ξε%ε~εικεμεν";
    pluperfect_act_2: "~ξε%ε~εικεις|~ξε%ε~εικης", "~ξε%ε~εικετον", "~ξε%ε~εικετε";
    pluperfect_act_3: "~ξε%ε~εικει|~ξε%ε~εικειν", "~ξε%ε~εικετην", "~ξε%ε~εικεσᾰν";
    pluperfect_mid_1: "~ξε%ε~ειμην", _, "~ξε%ε~ειμεθᾰ";
    pluperfect_mid_2: "~ξε%ε~εισο", "~ξε%ε~εισθον", "~ξε%ε~εισθε";
    pluperfect_mid_3: "~ξε%ε~ειτο", "~ξε%ε~εισθην", _;
    pluperfect_pass_1: _, _, "~ξε%ε~ειμεθᾰ";
    pluperfect_pass_2: _, _, "~ξε%ε~εισθε";
    pluperfect_pass_3: _, "~ξε%ε~εισθην", "~ξε%ε~ειντο";
    infinitive_pres: "~κτι~εναι", "~κτι~εσθαι", _;
    infinitive_fut: "~κ~ησειν", _, _;
    infinitive_aor: "~κ~εῑναι", _, _;
    infinitive_perf: "~κ%ε~εικεναι", _, _;
    participle_pres: "~κτι~εις", "~κτι~εμενος", _;
    participle_fut: "~κ~ησων", _, _;
    participle_aor: "~κ~εις", _, _;
    participle_perf: "~κ%ε~εικως", _, _;
    ),
    // Προϊστημῐ: 1 words (προϊστημῐ). Like Δῐϊστημῐ, except in 152 cells: future_ind_act_1 sg, future_ind_act_1 pl, future_ind_act_2 sg, future_ind_act_2 du, future_ind_act_2 pl, future_ind_act_3 sg, ….
    conjugation!("προϊστημῐ", 1, "~ϊ~ημῐ", like Δῐϊστημῐ;
    future_ind_act_1: "~~ησω", _, "~~ησομεν";
    future_ind_act_2: "~~ησεις", "~~ησετον", "~~ησετε";
    future_ind_act_3: "~~ησει", _, "~~ησουσῐ|~~ησουσῐν";
    future_ind_mid_1: "~~ησομαι", _, "~~ησομεθᾰ";
    future_ind_mid_2: "~~ησῃ|~~ησει", "~~ησεσθον", "~~ησεσθε";
    future_ind_mid_3: "~~ησεται", _, "~~ησονται";
    future_ind_pass_1: "~~ᾰθησομαι", _, "~~ᾰθησομεθᾰ";
    future_ind_pass_2: "~~ᾰθησῃ|~~ᾰθησει", "~~ᾰθησεσθον", "~~ᾰθησεσθε";
    future_ind_pass_3: "~~ᾰθησεται", _, "~~ᾰθησονται";
    future_opt_act_1: "~~ησοιμῐ", _, "~~ησοιμεν";
    future_opt_act_2: "~~ησοις", "~~ησοιτον", "~~ησοιτε";
    future_opt_act_3: "~~ησοι", "~~ησοιτην", "~~ησοιεν";
    future_opt_mid_1: "~~ησοιμην", _, "~~ησοιμεθᾰ";
    future_opt_mid_2: "~~ησοιο", "~~ησοισθον", "~~ησοισθε";
    future_opt_mid_3: "~~ησοιτο", "~~ησοισθην", "~~ησοιντο";
    future_opt_pass_1: "~~ᾰθησοιμην", _, "~~ᾰθησοιμεθᾰ";
    future_opt_pass_2: "~~ᾰθησοιο", "~~ᾰθησοισθον", "~~ᾰθησοισθε";
    future_opt_pass_3: "~~ᾰθησοιτο", "~~ᾰθησοισθην", "~~ᾰθησοιντο";
    aorist_ind_act_1: "~υ~ησᾰ|~ε~ησᾰ|~υ~ην|~ε~ην", _, "~υ~ησᾰμεν|~ε~ησᾰμεν|~υ~ημεν|~ε~ημεν";
    aorist_ind_act_2: "~υ~ησᾰς|~ε~ησᾰς|~υ~ης|~ε~ης", "~υ~ησᾰτον|~ε~ησᾰτον|~υ~ητον|~ε~ητον", "~υ~ησᾰτε|~ε~ησᾰτε|~υ~ητε|~ε~ητε";
    aorist_ind_act_3: "~υ~ησε|~υ~ησεν|~ε~ησε|~ε~ησεν|~υ~η|~ε~η", "~υ~ησᾰτην|~ε~ησᾰτην|~υ~ητην|~ε~ητην", "~υ~ησᾰν|~ε~ησᾰν";
    aorist_ind_mid_1: "~υ~ησᾰμην|~ε~ησᾰμην|~υ~ᾰμην|~ε~ᾰμην", _, "~υ~ησᾰμεθᾰ|~ε~ησᾰμεθᾰ|~υ~ᾰμεθᾰ|~ε~ᾰμεθᾰ";
    aorist_ind_mid_2: "~υ~ησω|~ε~ησω|~υ~ω|~ε~ω", "~υ~ησᾰσθον|~ε~ησᾰσθον|~υ~ᾰσθον|~ε~ᾰσθον", "~υ~ησᾰσθε|~ε~ησᾰσθε|~υ~ᾰσθε|~ε~ᾰσθε";
    aorist_ind_mid_3: "~υ~ησᾰτο|~ε~ησᾰτο|~υ~ᾰτο|~ε~ᾰτο", "~υ~ησᾰσθην|~ε~ησᾰσθην|~υ~ᾰσθην|~ε~ᾰσθην", "~υ~ησᾰντο|~ε~ησᾰντο|~υ~ᾰντο|~ε~ᾰντο";
    aorist_ind_pass_1: "~υ~ᾰθην|~ε~ᾰθην", _, "~υ~ᾰθημεν|~ε~ᾰθημεν";
    aorist_ind_pass_2: "~υ~ᾰθης|~ε~ᾰθης", "~υ~ᾰθητον|~ε~ᾰθητον", "~υ~ᾰθητε|~ε~ᾰθητε";
    aorist_ind_pass_3: "~υ~ᾰθη|~ε~ᾰθη", "~υ~ᾰθητην|~ε~ᾰθητην", "~υ~ᾰθησᾰν|~ε~ᾰθησᾰν";
    aorist_subj_act_1: "~~ησω|~~ω", _, "~~ησωμεν|~~ωμεν";
    aorist_subj_act_2: "~~ησῃς|~~ῃς", "~~ησητον|~~ητον", "~~ησητε|~~ητε";
    aorist_subj_act_3: "~~ησῃ|~~ῃ", "~~ησητον|~~ητον", "~~ησωσῐ|~~ησωσῐν|~~ωσῐ|~~ωσῐν";
    aorist_subj_mid_1: "~~ησωμαι|~~ωμαι", _, "~~ησωμεθᾰ|~~ωμεθᾰ";
    aorist_subj_mid_2: "~~ησῃ|~~ῃ", "~~ησησθον|~~ησθον", "~~ησησθε|~~ησθε";
    aorist_subj_mid_3: "~~ησηται|~~ηται", "~~ησησθον|~~ησθον", "~~ησωνται|~~ωνται";
    aorist_subj_pass_1: "~~ᾰθω", _, "~~ᾰθωμεν";
    aorist_subj_pass_2: "~~ᾰθῃς", "~~ᾰθητον", "~~ᾰθητε";
    aorist_subj_pass_3: "~~ᾰθῃ", _, "~~ᾰθωσῐ|~~ᾰθωσῐν";
    aorist_opt_act_1: "~~ησαιμῐ|~~αιην", _, "~~ησαιμεν|~~αῑμεν|~~αιημεν";
    aorist_opt_act_2: "~~ησειᾰς|~~ησαις|~~αιης", "~~ησαιτον|~~αῑτον|~~αιητον", "~~ησαιτε|~~αῑτε|~~αιητε";
    aorist_opt_act_3: "~~ησειε|~~ησειεν|~~ησαι|~~αιη", "~~ησαιτην|~~αιτην|~~αιητην", "~~ησειᾰν|~~ησαιεν|~~αῑεν|~~αιησᾰν";
    aorist_opt_mid_1: "~~ησαιμην|~~αιμην", _, "~~ησαιμεθᾰ|~~αιμεθᾰ";
    aorist_opt_mid_2: "~~ησαιο|~~αῑο", "~~ησαισθον|~~αῑσθον", "~~ησαισθε|~~αῑσθε";
    aorist_opt_mid_3: "~~ησαιτο|~~αῑτο", "~~ησαισθην|~~αισθην", "~~ησαιντο|~~αῑντο";
    aorist_opt_pass_1: "~~ᾰθειην", _, "~~ᾰθεῑμεν|~~ᾰθειημεν";
    aorist_opt_pass_2: "~~ᾰθειης", "~~ᾰθεῑτον|~~ᾰθειητον", "~~ᾰθεῑτε|~~ᾰθειητε";
    aorist_opt_pass_3: "~~ᾰθειη", "~~ᾰθειτην|~~ᾰθειητην", "~~ᾰθεῑεν|~~ᾰθειησᾰν";
    aorist_imp_act_2: "~~ησον|~~ηθῐ", "~~ησᾰτον|~~ητον", "~~ησᾰτε|~~ητε";
    aorist_imp_act_3: "~~ησᾰτω|~~ητω", "~~ησᾰτων|~~ητων", "~~ησᾰντων|~~ᾰντων";
    aorist_imp_mid_2: "~~ησαι|~~ω", "~~ησᾰσθον|~~ᾰσθον", "~~ησᾰσθε|~~ᾰσθε";
    aorist_imp_mid_3: "~~ησᾰσθω|~~ᾰσθω", "~~ησᾰσθων|~~ᾰσθων", "~~ησᾰσθων|~~ᾰσθων";
    aorist_imp_pass_2: "~~ᾰθητῐ", _, _;
    aorist_imp_pass_3: "~~ᾰθητω", "~~ᾰθητων", "~~ᾰθεντων";
    pluperfect_act_1: derived, _, derived;
    pluperfect_act_2: derived, derived, derived;
    pluperfect_act_3: derived, derived, derived;
    infinitive_fut: "~~ησειν", "~~ησεσθαι", "~~ᾰθησεσθαι";
    infinitive_aor: "~~ησαι|~~ηναι", "~~ησᾰσθαι|~~ᾰσθαι", "~~ᾰθηναι";
    participle_fut: "~~ησων", "~~ησομενος", "~~ᾰθησομενος";
    participle_aor: "~~ησᾱς|~~ᾱς", "~~ησᾰμενος|~~ᾰμενος", "~~ᾰθεις";
    ),
    // Λλῡμῐ: 1 words (λλῡμῐ). Like Ἀρνυμαι, except in 170 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("λλῡμῐ", 1, "~λῡμῐ", like Ἀρνυμαι;
    ind_act_1: "~λῡμῐ", _, "~λῠμεν";
    ind_act_2: "~λῡς", _, _;
    ind_act_3: "~λῡσῐ|~λῡσῐν", _, "~λῠᾱσῐ|~λῠᾱσῐν";
    ind_mid_1: "~λῠμαι", _, "~λῠμεθᾰ";
    ind_mid_2: "~λῠσαι", "~λῠσθον", "~λῠσθε";
    ind_mid_3: "~λῠται", _, "~λῠνται";
    ind_pass_1: _, _, "~λῠμεθᾰ";
    ind_pass_2: _, _, "~λῠσθε";
    ind_pass_3: "~λῠται", _, "~λῠνται";
    subj_act_1: "~λῠω", _, "~λῠωμεν";
    subj_act_2: "~λῠῃς", "~λῠητον", "~λῠητε";
    subj_act_3: "~λῠῃ", _, "~λῠωσῐ|~λῠωσῐν";
    subj_mid_1: "~λῠωμαι", _, "~λῠωμεθᾰ";
    subj_mid_2: "~λῠῃ", "~λῠησθον", "~λῠησθε";
    subj_mid_3: "~λῠηται", _, "~λῠωνται";
    subj_pass_1: _, _, "~λῠωμεθᾰ";
    subj_pass_2: _, _, "~λῠησθε";
    subj_pass_3: "~λῠηται", _, "~λῠωνται";
    opt_act_1: "~λῠοιμῐ", _, "~λῠοιμεν";
    opt_act_2: "~λῠοις", "~λῠοιτον", "~λῠοιτε";
    opt_act_3: "~λῠοι", "~λῠοιτην", "~λῠοιεν";
    opt_mid_1: "~λῠοιμην", _, "~λῠοιμεθᾰ";
    opt_mid_2: "~λῠοιο", "~λῠοισθον", "~λῠοισθε";
    opt_mid_3: "~λῠοιτο", "~λῠοισθην", "~λῠοιντο";
    opt_pass_1: _, _, "~λῠοιμεθᾰ";
    opt_pass_2: _, "~λῠοισθον", "~λῠοισθε";
    opt_pass_3: "~λῠοιτο", "~λῠοισθην", "~λῠοιντο";
    imp_act_2: "~λῡ", "~λῠτον", "~λῠτε";
    imp_act_3: "~λῠτω", "~λῠτων", "~λῠντων";
    imp_mid_2: "~λῠσο", _, "~λῠσθε";
    imp_mid_3: "~λῠσθω", "~λῠσθων", "~λῠσθων";
    imp_pass_2: _, _, "~λῠσθε";
    imp_pass_3: _, "~λῠσθων", _;
    future_ind_act_1: "~ω", _, "~οῡμεν";
    future_ind_act_2: "~εῑς", "~εῑτον", "~εῑτε";
    future_ind_act_3: "~εῑ", _, "~οῡσῐ|~οῡσῐν";
    future_ind_mid_1: "~οῡμαι", _, "~ουμεθᾰ";
    future_ind_mid_2: "~εῑ|~ῃ", "~εῑσθον", "~εῑσθε";
    future_ind_mid_3: "~εῑται", _, "~οῡνται";
    future_opt_act_1: "~οιην|~οῑμῐ", _, "~οῑμεν|~οιημεν";
    future_opt_act_2: "~οιης|~οῑς", "~οῑτον|~οιητον", "~οῑτε|~οιητε";
    future_opt_act_3: "~οιη|~οῑ", "~οιτην|~οιητην", "~οῑεν|~οιησᾰν";
    future_opt_mid_1: "~οιμην", _, "~οιμεθᾰ";
    future_opt_mid_2: "~οῑο", "~οῑσθον", "~οῑσθε";
    future_opt_mid_3: "~οῑτο", "~οισθην", "~οῑντο";
    aorist_ind_mid_1: derived, _, derived;
    aorist_ind_mid_2: derived, derived, derived;
    aorist_ind_mid_3: derived, derived, derived;
    aorist_subj_act_1: "~εσω", _, "~εσωμεν";
    aorist_subj_act_2: "~εσῃς", "~εσητον", "~εσητε";
    aorist_subj_act_3: "~εσῃ", _, "~εσωσῐ|~εσωσῐν";
    aorist_opt_act_1: "~εσαιμῐ", _, "~εσαιμεν";
    aorist_opt_act_2: "~εσειᾰς|~εσαις", "~εσαιτον", "~εσαιτε";
    aorist_opt_act_3: "~εσειε|~εσειεν|~εσαι", "~εσαιτην", "~εσειᾰν|~εσαιεν";
    aorist_imp_act_2: "~εσον", "~εσᾰτον", "~εσᾰτε";
    aorist_imp_act_3: "~εσᾰτω", "~εσᾰτων", "~εσᾰντων";
    perfect_ind_act_1: "~ωλεκᾰ|~ωλᾰ", _, "~ωλεκᾰμεν|~ωλᾰμεν";
    perfect_ind_act_2: "~ωλεκᾰς|~ωλᾰς", "~ωλεκᾰτον|~ωλᾰτον", "~ωλεκᾰτε|~ωλᾰτε";
    perfect_ind_act_3: "~ωλεκε|~ωλεκεν|~ωλε|~ωλεν", "~ωλεκᾰτον|~ωλᾰτον", "~ωλεκᾱσῐ|~ωλεκᾱσῐν|~ωλᾱσῐ|~ωλᾱσῐν";
    perfect_subj_act_1: "~ωλεκω|~ωλω", _, "~ωλεκωμεν|~ωλωμεν";
    perfect_subj_act_2: "~ωλεκῃς|~ωλῃς", "~ωλεκητον|~ωλητον", "~ωλεκητε|~ωλητε";
    perfect_subj_act_3: "~ωλεκῃ|~ωλῃ", "~ωλεκητον|~ωλητον", "~ωλεκωσῐ|~ωλεκωσῐν|~ωλωσῐ|~ωλωσῐν";
    perfect_opt_act_1: "~ωλεκοιμῐ|~ωλεκοιην|~ωλοιμῐ|~ωλοιην", _, "~ωλεκοιμεν|~ωλοιμεν";
    perfect_opt_act_2: "~ωλεκοις|~ωλεκοιης|~ωλοις|~ωλοιης", "~ωλεκοιτον|~ωλοιτον", "~ωλεκοιτε|~ωλοιτε";
    perfect_opt_act_3: "~ωλεκοι|~ωλεκοιη|~ωλοι|~ωλοιη", "~ωλεκοιτην|~ωλοιτην", "~ωλεκοιεν|~ωλοιεν";
    perfect_imp_act_2: "~ωλεκε|~ωλε", "~ωλεκετον|~ωλετον", "~ωλεκετε|~ωλετε";
    perfect_imp_act_3: "~ωλεκετω|~ωλετω", "~ωλεκετων|~ωλετων", "~ωλεκοντων|~ωλοντων";
    infinitive_pres: "~λῠναι", "~λῠσθαι", _;
    infinitive_fut: "~εῑν", "~εῑσθαι", _;
    infinitive_aor: "~εσαι", _, _;
    infinitive_perf: "~ωλεκεναι|~ωλεναι", _, _;
    participle_pres: "~λῡς", "~λῠμενος", _;
    participle_fut: "~ων", "~ουμενος", _;
    participle_aor: "~εσᾱς", _, _;
    participle_perf: "~ωλεκως|~ωλως", _, _;
    ),
    // Γνῡμῐ: 1 words (γνῡμῐ). Like Εἰμῐ, except in 249 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("γνῡμῐ", 1, "γνῡμῐ", like Εἰμῐ;
    ind_act_1: "γνῡμῐ", _, "γνῠμεν";
    ind_act_2: "γνῡς", _, _;
    ind_act_3: "γνῡσῐ|γνῡσῐν", _, "γνῠᾱσῐ|γνῠᾱσῐν";
    ind_mid_1: "γνῠμαι", _, "γνῠμεθᾰ";
    ind_mid_2: "γνῠσαι", "γνῠσθον", "γνῠσθε";
    ind_mid_3: "γνῠται", _, "γνῠνται";
    ind_pass_1: _, _, "γνῠμεθᾰ";
    ind_pass_2: _, _, "γνῠσθε";
    ind_pass_3: "γνῠται", _, "γνῠνται";
    subj_act_1: "γνῠω", _, "γνῠωμεν";
    subj_act_2: "γνῠῃς", "γνῠητον", "γνῠητε";
    subj_act_3: "γνῠῃ", _, "γνῠωσῐ|γνῠωσῐν";
    subj_mid_1: "γνῠωμαι", _, "γνῠωμεθᾰ";
    subj_mid_2: "γνῠῃ", "γνῠησθον", "γνῠησθε";
    subj_mid_3: "γνῠηται", _, "γνῠωνται";
    subj_pass_1: _, _, "γνῠωμεθᾰ";
    subj_pass_2: _, _, "γνῠησθε";
    subj_pass_3: "γνῠηται", _, "γνῠωνται";
    opt_act_1: "γνῠοιμῐ", _, "γνῠοιμεν";
    opt_act_2: "γνῠοις", "γνῠοιτον", "γνῠοιτε";
    opt_act_3: "γνῠοι", "γνῠοιτην", "γνῠοιεν";
    opt_mid_1: "γνῠοιμην", _, "γνῠοιμεθᾰ";
    opt_mid_2: "γνῠοιο", "γνῠοισθον", "γνῠοισθε";
    opt_mid_3: "γνῠοιτο", "γνῠοισθην", "γνῠοιντο";
    opt_pass_1: _, _, "γνῠοιμεθᾰ";
    opt_pass_2: _, "γνῠοισθον", "γνῠοισθε";
    opt_pass_3: "γνῠοιτο", "γνῠοισθην", "γνῠοιντο";
    imp_act_2: "γνῡ", "γνῠτον", "γνῠτε";
    imp_act_3: "γνῠτω", "γνῠτων", "γνῠντων";
    imp_mid_2: "γνῠσο", _, "γνῠσθε";
    imp_mid_3: "γνῠσθω", "γνῠσθων", "γνῠσθων";
    imp_pass_2: _, _, "γνῠσθε";
    imp_pass_3: _, "γνῠσθων", _;
    imperfect_act_1: "ἐᾰγνῡν", _, "ἐᾰγνῠμεν";
    imperfect_act_2: "ἐᾰγνῡς", "ἐᾰγνῠτον", "ἐᾰγνῠτε";
    imperfect_act_3: "ἐᾰγνῡ", "ἐᾰγνῠτην", "ἐᾰγνῠσᾰν";
    imperfect_mid_1: "ἐᾰγνῠμην", _, "ἐᾰγνῠμεθᾰ";
    imperfect_mid_2: "ἐᾰγνῠσο", "ἐᾰγνῠσθον", "ἐᾰγνῠσθε";
    imperfect_mid_3: "ἐᾰγνῠτο", "ἐᾰγνῠσθην", "ἐᾰγνῠντο";
    imperfect_pass_1: _, _, "ἐᾰγνῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐᾰγνῠσθε";
    imperfect_pass_3: "ἐᾰγνῠτο", _, _;
    future_ind_act_1: "ξω", _, "ξομεν";
    future_ind_act_2: "ξεις", "ξετον", "ξετε";
    future_ind_act_3: "ξει", _, "ξουσῐ|ξουσῐν";
    future_ind_mid_1: "ξομαι", _, "ξομεθᾰ";
    future_ind_mid_2: "ξῃ|ξει", "ξεσθον", "ξεσθε";
    future_ind_mid_3: "ξεται", _, "ξονται";
    future_opt_act_1: "ξοιμῐ", _, "ξοιμεν";
    future_opt_act_2: "ξοις", "ξοιτον", "ξοιτε";
    future_opt_act_3: "ξοι", "ξοιτην", "ξοιεν";
    future_opt_mid_1: "ξοιμην", _, "ξοιμεθᾰ";
    future_opt_mid_2: "ξοιο", "ξοισθον", "ξοισθε";
    future_opt_mid_3: "ξοιτο", "ξοισθην", "ξοιντο";
    aorist_ind_act_1: "ἐᾰξᾰ", _, "ἐᾰξᾰμεν";
    aorist_ind_act_2: "ἐᾰξᾰς", "ἐᾰξᾰτον", "ἐᾰξᾰτε";
    aorist_ind_act_3: "ἐᾰξε|ἐᾰξεν", "ἐᾰξᾰτην", "ἐᾰξᾰν";
    aorist_ind_mid_1: "ἐᾰξᾰμην", _, "ἐᾰξᾰμεθᾰ";
    aorist_ind_mid_2: "ἐᾰξω", "ἐᾰξᾰσθον", "ἐᾰξᾰσθε";
    aorist_ind_mid_3: "ἐᾰξᾰτο", "ἐᾰξᾰσθην", "ἐᾰξᾰντο";
    aorist_ind_pass_1: "ἐαχθην|ἐᾰγην", _, "ἐαχθημεν|ἐᾰγημεν";
    aorist_ind_pass_2: "ἐαχθης|ἐᾰγης", "ἐαχθητον|ἐᾰγητον", "ἐαχθητε|ἐᾰγητε";
    aorist_ind_pass_3: "ἐαχθη|ἐᾰγη", "ἐαχθητην|ἐᾰγητην", "ἐαχθησᾰν|ἐᾰγησᾰν";
    aorist_subj_act_1: "ξω", _, "ξωμεν";
    aorist_subj_act_2: "ξῃς", "ξητον", "ξητε";
    aorist_subj_act_3: "ξῃ", _, "ξωσῐ|ξωσῐν";
    aorist_subj_mid_1: "ξωμαι", _, "ξωμεθᾰ";
    aorist_subj_mid_2: "ξῃ", "ξησθον", "ξησθε";
    aorist_subj_mid_3: "ξηται", _, "ξωνται";
    aorist_subj_pass_1: "χθω|γω", _, "χθωμεν|γωμεν";
    aorist_subj_pass_2: "χθῃς|γῃς", "χθητον|γητον", "χθητε|γητε";
    aorist_subj_pass_3: "χθῃ|γῃ", "χθητον|γητον", "χθωσῐ|χθωσῐν|γωσῐ|γωσῐν";
    aorist_opt_act_1: "ξαιμῐ", _, "ξαιμεν";
    aorist_opt_act_2: "ξειᾰς|ξαις", "ξαιτον", "ξαιτε";
    aorist_opt_act_3: "ξειε|ξειεν|ξαι", "ξαιτην", "ξειᾰν|ξαιεν";
    aorist_opt_mid_1: "ξαιμην", _, "ξαιμεθᾰ";
    aorist_opt_mid_2: "ξαιο", "ξαισθον", "ξαισθε";
    aorist_opt_mid_3: "ξαιτο", "ξαισθην", "ξαιντο";
    aorist_opt_pass_1: "χθειην|γειην", _, "χθεῑμεν|χθειημεν|γεῑμεν|γειημεν";
    aorist_opt_pass_2: "χθειης|γειης", "χθεῑτον|χθειητον|γεῑτον|γειητον", "χθεῑτε|χθειητε|γεῑτε|γειητε";
    aorist_opt_pass_3: "χθειη|γειη", "χθειτην|χθειητην|γειτην|γειητην", "χθεῑεν|χθειησᾰν|γεῑεν|γειησᾰν";
    aorist_imp_act_2: "ξον", "ξᾰτον", "ξᾰτε";
    aorist_imp_act_3: "ξᾰτω", "ξᾰτων", "ξᾰντων";
    aorist_imp_mid_2: "ξαι", "ξᾰσθον", "ξᾰσθε";
    aorist_imp_mid_3: "ξᾰσθω", "ξᾰσθων", _;
    aorist_imp_pass_2: "χθητῐ|γηθῐ", "χθητον|γητον", "χθητε|γητε";
    aorist_imp_pass_3: "χθητω|γητω", "χθητων|γητων", "χθεντων|γεντων";
    perfect_ind_act_1: "ἐᾱγᾰ", _, "ἐᾱγᾰμεν";
    perfect_ind_act_2: "ἐᾱγᾰς", "ἐᾱγᾰτον", "ἐᾱγᾰτε";
    perfect_ind_act_3: "ἐᾱγε|ἐᾱγεν", _, "ἐᾱγᾱσῐ|ἐᾱγᾱσῐν";
    perfect_subj_act_1: "ἐᾱγω", _, "ἐᾱγωμεν";
    perfect_subj_act_2: "ἐᾱγῃς", "ἐᾱγητον", "ἐᾱγητε";
    perfect_subj_act_3: "ἐᾱγῃ", _, "ἐᾱγωσῐ|ἐᾱγωσῐν";
    perfect_opt_act_1: "ἐᾱγοιμῐ|ἐᾱγοιην", _, "ἐᾱγοιμεν";
    perfect_opt_act_2: "ἐᾱγοις|ἐᾱγοιης", "ἐᾱγοιτον", "ἐᾱγοιτε";
    perfect_opt_act_3: "ἐᾱγοι|ἐᾱγοιη", "ἐᾱγοιτην", "ἐᾱγοιεν";
    perfect_imp_act_2: "ἐᾱγε", "ἐᾱγετον", "ἐᾱγετε";
    perfect_imp_act_3: "ἐᾱγετω", "ἐᾱγετων", "ἐᾱγοντων";
    infinitive_pres: "γνῠναι", "γνῠσθαι", _;
    infinitive_fut: "ξειν", "ξεσθαι", _;
    infinitive_aor: "ξαι", "ξᾰσθαι", "χθηναι|γηναι";
    infinitive_perf: "ἐᾱγεναι", _, _;
    participle_pres: "γνῡς", "γνῠμενος", _;
    participle_fut: "ξων", "ξομενος", _;
    participle_aor: "ξᾱς", "ξᾰμενος", "χθεις|γεις";
    participle_perf: "ἐᾱγως", _, _;
    ),
    // Ρρηγνῡμῐ: 1 words (ρρηγνῡμῐ). Like Πηγνῡμῐ, except in 136 cells: imperfect_act_1 sg, imperfect_act_1 pl, imperfect_act_2 sg, imperfect_act_2 du, imperfect_act_2 pl, imperfect_act_3 sg, ….
    conjugation!("ρρηγνῡμῐ", 1, "~ηγνῡμῐ", like Πηγνῡμῐ;
    imperfect_act_1: "ἐ~ηγνῡν", _, "ἐ~ηγνῠμεν";
    imperfect_act_2: "ἐ~ηγνῡς", "ἐ~ηγνῠτον", "ἐ~ηγνῠτε";
    imperfect_act_3: "ἐ~ηγνῡ", "ἐ~ηγνῠτην", "ἐ~ηγνῠσᾰν";
    imperfect_mid_1: "ἐ~ηγνῠμην", _, "ἐ~ηγνῠμεθᾰ";
    imperfect_mid_2: "ἐ~ηγνῠσο", "ἐ~ηγνῠσθον", "ἐ~ηγνῠσθε";
    imperfect_mid_3: "ἐ~ηγνῠτο", "ἐ~ηγνῠσθην", "ἐ~ηγνῠντο";
    imperfect_pass_1: _, _, "ἐ~ηγνῠμεθᾰ";
    imperfect_pass_2: _, _, "ἐ~ηγνῠσθε";
    imperfect_pass_3: "ἐ~ηγνῠτο", _, _;
    future_ind_pass_1: derived, _, derived;
    future_ind_pass_2: derived, derived, derived;
    future_ind_pass_3: derived, _, derived;
    future_opt_pass_1: derived, _, derived;
    future_opt_pass_2: derived, derived, derived;
    future_opt_pass_3: derived, derived, derived;
    aorist_ind_pass_1: "ἐ~ᾰγην", _, "ἐ~ᾰγημεν";
    aorist_ind_pass_2: "ἐ~ᾰγης", "ἐ~ᾰγητον", "ἐ~ᾰγητε";
    aorist_ind_pass_3: "ἐ~ᾰγη", "ἐ~ᾰγητην", "ἐ~ᾰγησᾰν";
    aorist_subj_pass_1: "~ᾰγω", _, "~ᾰγωμεν";
    aorist_subj_pass_2: "~ᾰγῃς", "~ᾰγητον", "~ᾰγητε";
    aorist_subj_pass_3: "~ᾰγῃ", derived, "~ᾰγωσῐ|~ᾰγωσῐν";
    aorist_opt_pass_1: "~ᾰγειην", _, "~ᾰγεῑμεν|~ᾰγειημεν";
    aorist_opt_pass_2: "~ᾰγειης", "~ᾰγεῑτον|~ᾰγειητον", "~ᾰγεῑτε|~ᾰγειητε";
    aorist_opt_pass_3: "~ᾰγειη", "~ᾰγειτην|~ᾰγειητην", "~ᾰγεῑεν|~ᾰγειησᾰν";
    aorist_imp_pass_2: "~ᾰγηθῐ", derived, derived;
    aorist_imp_pass_3: "~ᾰγητω", "~ᾰγητων", "~ᾰγεντων";
    perfect_ind_act_1: "ἐ~ωγᾰ", _, "ἐ~ωγᾰμεν";
    perfect_ind_act_2: "ἐ~ωγᾰς", "ἐ~ωγᾰτον", "ἐ~ωγᾰτε";
    perfect_ind_act_3: "ἐ~ωγε|ἐ~ωγεν", derived, "ἐ~ωγᾱσῐ|ἐ~ωγᾱσῐν";
    perfect_ind_mid_1: "~ηρρηγμαι", _, "~ηρρηγμεθᾰ";
    perfect_ind_mid_2: "~ηρρηξαι", "~ηρρηχθον", "~ηρρηχθε";
    perfect_ind_mid_3: "~ηρρηκται", _, "~ηρρηγᾰται";
    perfect_ind_pass_1: _, _, "~ηρρηγμεθᾰ";
    perfect_ind_pass_2: _, _, "~ηρρηχθε";
    perfect_subj_act_1: "ἐ~ωγω", _, "ἐ~ωγωμεν";
    perfect_subj_act_2: "ἐ~ωγῃς", "ἐ~ωγητον", "ἐ~ωγητε";
    perfect_subj_act_3: "ἐ~ωγῃ", derived, "ἐ~ωγωσῐ|ἐ~ωγωσῐν";
    perfect_opt_act_1: "ἐ~ωγοιμῐ|ἐ~ωγοιην", _, "ἐ~ωγοιμεν";
    perfect_opt_act_2: "ἐ~ωγοις|ἐ~ωγοιης", "ἐ~ωγοιτον", "ἐ~ωγοιτε";
    perfect_opt_act_3: "ἐ~ωγοι|ἐ~ωγοιη", "ἐ~ωγοιτην", "ἐ~ωγοιεν";
    perfect_imp_act_2: "ἐ~ωγε", "ἐ~ωγετον", "ἐ~ωγετε";
    perfect_imp_act_3: "ἐ~ωγετω", "ἐ~ωγετων", "ἐ~ωγοντων";
    perfect_imp_mid_2: "~ηρρηξο", _, _;
    perfect_imp_mid_3: "~ηρρηχθω", "~ηρρηχθων", _;
    perfect_imp_pass_2: "~ηρρηξο", _, _;
    perfect_imp_pass_3: "~ηρρηχθω", "~ηρρηχθων", _;
    pluperfect_act_1: "ἐ~ωγειν|ἐ~ωγη", _, "ἐ~ωγεμεν";
    pluperfect_act_2: "ἐ~ωγεις|ἐ~ωγης", "ἐ~ωγετον", "ἐ~ωγετε";
    pluperfect_act_3: "ἐ~ωγει|ἐ~ωγειν", "ἐ~ωγετην", "ἐ~ωγεσᾰν";
    pluperfect_mid_1: "~ηρρηγμην", _, "~ηρρηγμεθᾰ";
    pluperfect_mid_2: "~ηρρηξο", "~ηρρηχθον", "~ηρρηχθε";
    pluperfect_mid_3: "~ηρρηκτο", "~ηρρηχθην", _;
    pluperfect_pass_1: _, _, "~ηρρηγμεθᾰ";
    pluperfect_pass_2: _, _, "~ηρρηχθε";
    pluperfect_pass_3: _, "~ηρρηχθην", "~ηρρηγᾰτο";
    infinitive_fut: _, _, derived;
    infinitive_aor: _, _, "~ᾰγηναι";
    infinitive_perf: "ἐ~ωγεναι", "~ηρρηχθαι", _;
    participle_fut: _, _, derived;
    participle_aor: _, _, "~ᾰγεις";
    participle_perf: "ἐ~ωγως", "~ηρρηγμενος", _;
    ),
    // Δειδῐᾰ: 1 words (δειδῐᾰ). Like Μετεκῑᾰθον, except in 92 cells: ind_act_1 sg, aorist_ind_act_1 sg, aorist_ind_act_1 pl, aorist_ind_act_2 sg, aorist_ind_act_2 du, aorist_ind_act_2 pl, ….
    conjugation!("δειδῐᾰ", 1, "~~δῐᾰ", like Μετεκῑᾰθον;
    ind_act_1: "~~δῐᾰ", _, _;
    aorist_ind_act_1: "ἐ~~σᾰ", _, "ἐ~~σᾰμεν";
    aorist_ind_act_2: "ἐ~~σᾰς", "ἐ~~σᾰτον", "ἐ~~σᾰτε";
    aorist_ind_act_3: "ἐ~~σε|ἐ~~σεν", "ἐ~~σᾰτην", "ἐ~~σᾰν";
    aorist_ind_mid_1: "ἐ~~σᾰμην", _, "ἐ~~σᾰμεθᾰ";
    aorist_ind_mid_2: "ἐ~~σω", "ἐ~~σᾰσθον", "ἐ~~σᾰσθε";
    aorist_ind_mid_3: "ἐ~~σᾰτο", "ἐ~~σᾰσθην", "ἐ~~σᾰντο";
    aorist_subj_act_1: "~~σω", _, "~~σωμεν";
    aorist_subj_act_2: "~~σῃς", "~~σητον", "~~σητε";
    aorist_subj_act_3: "~~σῃ", _, "~~σωσῐ|~~σωσῐν";
    aorist_subj_mid_1: "~~σωμαι", _, "~~σωμεθᾰ";
    aorist_subj_mid_2: "~~σῃ", "~~σησθον", "~~σησθε";
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
    // Κᾰτᾰμῑγνῡμῐ: 1 words (κᾰτᾰμῑγνῡμῐ). Like Πᾰρᾰλλᾰσσω, except in 192 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("κᾰτᾰμῑγνῡμῐ", 1, "~ᾰ~γνῡμῐ", like Πᾰρᾰλλᾰσσω;
    ind_act_1: "~ᾰ~γνῡμῐ", _, "~ᾰ~γνῠμεν";
    ind_act_2: "~ᾰ~γνῡς", _, _;
    ind_act_3: "~ᾰ~γνῡσῐ|~ᾰ~γνῡσῐν", _, "~ᾰ~γνῠᾱσῐ|~ᾰ~γνῠᾱσῐν";
    ind_mid_1: "~ᾰ~γνῠμαι", _, "~ᾰ~γνῠμεθᾰ";
    ind_mid_2: "~ᾰ~γνῠσαι", "~ᾰ~γνῠσθον", "~ᾰ~γνῠσθε";
    ind_mid_3: "~ᾰ~γνῠται", _, "~ᾰ~γνῠνται";
    ind_pass_1: _, _, "~ᾰ~γνῠμεθᾰ";
    ind_pass_2: derived, _, "~ᾰ~γνῠσθε";
    ind_pass_3: "~ᾰ~γνῠται", _, "~ᾰ~γνῠνται";
    subj_act_1: "~ᾰ~γνῠω", _, "~ᾰ~γνῠωμεν";
    subj_act_2: "~ᾰ~γνῠῃς", "~ᾰ~γνῠητον", "~ᾰ~γνῠητε";
    subj_act_3: "~ᾰ~γνῠῃ", _, "~ᾰ~γνῠωσῐ|~ᾰ~γνῠωσῐν";
    subj_mid_1: "~ᾰ~γνῠωμαι", _, "~ᾰ~γνῠωμεθᾰ";
    subj_mid_2: "~ᾰ~γνῠῃ", "~ᾰ~γνῠησθον", "~ᾰ~γνῠησθε";
    subj_mid_3: "~ᾰ~γνῠηται", _, "~ᾰ~γνῠωνται";
    subj_pass_1: _, _, "~ᾰ~γνῠωμεθᾰ";
    subj_pass_2: _, _, "~ᾰ~γνῠησθε";
    subj_pass_3: "~ᾰ~γνῠηται", _, "~ᾰ~γνῠωνται";
    opt_act_1: "~ᾰ~γνῠοιμῐ", _, "~ᾰ~γνῠοιμεν";
    opt_act_2: "~ᾰ~γνῠοις", "~ᾰ~γνῠοιτον", "~ᾰ~γνῠοιτε";
    opt_act_3: "~ᾰ~γνῠοι", "~ᾰ~γνῠοιτην", "~ᾰ~γνῠοιεν";
    opt_mid_1: "~ᾰ~γνῠοιμην", _, "~ᾰ~γνῠοιμεθᾰ";
    opt_mid_2: "~ᾰ~γνῠοιο", "~ᾰ~γνῠοισθον", "~ᾰ~γνῠοισθε";
    opt_mid_3: "~ᾰ~γνῠοιτο", "~ᾰ~γνῠοισθην", "~ᾰ~γνῠοιντο";
    opt_pass_1: _, _, "~ᾰ~γνῠοιμεθᾰ";
    opt_pass_2: _, "~ᾰ~γνῠοισθον", "~ᾰ~γνῠοισθε";
    opt_pass_3: "~ᾰ~γνῠοιτο", "~ᾰ~γνῠοισθην", "~ᾰ~γνῠοιντο";
    imp_act_2: "~ᾰ~γνῡ", "~ᾰ~γνῠτον", "~ᾰ~γνῠτε";
    imp_act_3: "~ᾰ~γνῠτω", "~ᾰ~γνῠτων", "~ᾰ~γνῠντων";
    imp_mid_2: "~ᾰ~γνῠσο", _, "~ᾰ~γνῠσθε";
    imp_mid_3: "~ᾰ~γνῠσθω", "~ᾰ~γνῠσθων", "~ᾰ~γνῠσθων";
    imp_pass_2: _, _, "~ᾰ~γνῠσθε";
    imp_pass_3: _, "~ᾰ~γνῠσθων", _;
    imperfect_act_1: "~ε~γνῡν", _, "~ε~γνῠμεν";
    imperfect_act_2: "~ε~γνῡς", "~ε~γνῠτον", "~ε~γνῠτε";
    imperfect_act_3: "~ε~γνῡ", "~ε~γνῠτην", "~ε~γνῠσᾰν";
    imperfect_mid_1: "~ε~γνῠμην", _, "~ε~γνῠμεθᾰ";
    imperfect_mid_2: "~ε~γνῠσο", "~ε~γνῠσθον", "~ε~γνῠσθε";
    imperfect_mid_3: "~ε~γνῠτο", "~ε~γνῠσθην", "~ε~γνῠντο";
    imperfect_pass_1: _, _, "~ε~γνῠμεθᾰ";
    imperfect_pass_2: _, _, "~ε~γνῠσθε";
    imperfect_pass_3: "~ε~γνῠτο", _, _;
    aorist_ind_act_1: "~ε~ξᾰ", _, "~ε~ξᾰμεν";
    aorist_ind_act_2: "~ε~ξᾰς", "~ε~ξᾰτον", "~ε~ξᾰτε";
    aorist_ind_act_3: "~ε~ξε|~ε~ξεν", "~ε~ξᾰτην", "~ε~ξᾰν";
    aorist_ind_mid_1: "~ε~ξᾰμην", _, "~ε~ξᾰμεθᾰ";
    aorist_ind_mid_2: "~ε~ξω", "~ε~ξᾰσθον", "~ε~ξᾰσθε";
    aorist_ind_mid_3: "~ε~ξᾰτο", "~ε~ξᾰσθην", "~ε~ξᾰντο";
    aorist_ind_pass_1: "~ε~χθην", _, "~ε~χθημεν";
    aorist_ind_pass_2: "~ε~χθης", "~ε~χθητον", "~ε~χθητε";
    aorist_ind_pass_3: "~ε~χθη", "~ε~χθητην", "~ε~χθησᾰν";
    aorist_subj_act_3: _, _, "~ᾰ~ξωσῐ|~ᾰ~ξωσῐν";
    aorist_subj_pass_1: "~ᾰ~χθω", _, "~ᾰ~χθωμεν";
    aorist_subj_pass_2: "~ᾰ~χθῃς", "~ᾰ~χθητον", "~ᾰ~χθητε";
    aorist_subj_pass_3: "~ᾰ~χθῃ", derived, "~ᾰ~χθωσῐ|~ᾰ~χθωσῐν";
    aorist_opt_act_2: "~ᾰ~ξειᾰς|~ᾰ~ξαις", _, _;
    aorist_opt_act_3: "~ᾰ~ξειε|~ᾰ~ξειεν|~ᾰ~ξαι", _, "~ᾰ~ξειᾰν|~ᾰ~ξαιεν";
    aorist_opt_pass_1: "~ᾰ~χθειην", _, "~ᾰ~χθεῑμεν|~ᾰ~χθειημεν";
    aorist_opt_pass_2: "~ᾰ~χθειης", "~ᾰ~χθεῑτον|~ᾰ~χθειητον", "~ᾰ~χθεῑτε|~ᾰ~χθειητε";
    aorist_opt_pass_3: "~ᾰ~χθειη", "~ᾰ~χθειτην|~ᾰ~χθειητην", "~ᾰ~χθεῑεν|~ᾰ~χθειησᾰν";
    aorist_imp_pass_2: "~ᾰ~χθητῐ", derived, derived;
    aorist_imp_pass_3: "~ᾰ~χθητω", "~ᾰ~χθητων", "~ᾰ~χθεντων";
    perfect_ind_act_1: "~ᾰ%ε~χᾰ", _, "~ᾰ%ε~χᾰμεν";
    perfect_ind_act_2: "~ᾰ%ε~χᾰς", "~ᾰ%ε~χᾰτον", "~ᾰ%ε~χᾰτε";
    perfect_ind_act_3: "~ᾰ%ε~χε|~ᾰ%ε~χεν", _, "~ᾰ%ε~χᾱσῐ|~ᾰ%ε~χᾱσῐν";
    perfect_ind_mid_1: "~ᾰ%ε~γμαι", _, "~ᾰ%ε~γμεθᾰ";
    perfect_ind_mid_2: "~ᾰ%ε~ξαι", "~ᾰ%ε~χθον", "~ᾰ%ε~χθε";
    perfect_ind_mid_3: "~ᾰ%ε~κται", _, "~ᾰ%ε~γᾰται";
    perfect_ind_pass_1: _, _, "~ᾰ%ε~γμεθᾰ";
    perfect_ind_pass_2: _, _, "~ᾰ%ε~χθε";
    perfect_subj_act_1: "~ᾰ%ε~χω", _, "~ᾰ%ε~χωμεν";
    perfect_subj_act_2: "~ᾰ%ε~χῃς", "~ᾰ%ε~χητον", "~ᾰ%ε~χητε";
    perfect_subj_act_3: "~ᾰ%ε~χῃ", _, "~ᾰ%ε~χωσῐ|~ᾰ%ε~χωσῐν";
    perfect_opt_act_1: "~ᾰ%ε~χοιμῐ|~ᾰ%ε~χοιην", _, "~ᾰ%ε~χοιμεν";
    perfect_opt_act_2: "~ᾰ%ε~χοις|~ᾰ%ε~χοιης", "~ᾰ%ε~χοιτον", "~ᾰ%ε~χοιτε";
    perfect_opt_act_3: "~ᾰ%ε~χοι|~ᾰ%ε~χοιη", "~ᾰ%ε~χοιτην", "~ᾰ%ε~χοιεν";
    perfect_imp_act_2: "~ᾰ%ε~χε", "~ᾰ%ε~χετον", "~ᾰ%ε~χετε";
    perfect_imp_act_3: "~ᾰ%ε~χετω", "~ᾰ%ε~χετων", "~ᾰ%ε~χοντων";
    perfect_imp_mid_2: "~ᾰ%ε~ξο", _, _;
    perfect_imp_mid_3: "~ᾰ%ε~χθω", "~ᾰ%ε~χθων", _;
    perfect_imp_pass_2: "~ᾰ%ε~ξο", _, _;
    perfect_imp_pass_3: "~ᾰ%ε~χθω", "~ᾰ%ε~χθων", _;
    infinitive_pres: "~ᾰ~γνῠναι", "~ᾰ~γνῠσθαι", _;
    infinitive_aor: _, _, "~ᾰ~χθηναι";
    infinitive_perf: "~ᾰ%ε~χεναι", "~ᾰ%ε~χθαι", _;
    participle_pres: "~ᾰ~γνῡς", "~ᾰ~γνῠμενος", _;
    participle_aor: _, _, "~ᾰ~χθεις";
    participle_perf: "~ᾰ%ε~χως", "~ᾰ%ε~γμενος", _;
    ),
    // Προδῐδωμῐ: 1 words (προδῐδωμῐ). Like Ἐγκειμαι, except in 231 cells: ind_act_1 sg, ind_act_1 pl, ind_act_2 sg, ind_act_3 sg, ind_act_3 pl, ind_mid_1 sg, ….
    conjugation!("προδῐδωμῐ", 1, "~~ῐδωμῐ", like Ἐγκειμαι;
    ind_act_1: "~~ῐδωμῐ", _, "~~ῐδομεν";
    ind_act_2: "~~ῐδως", _, _;
    ind_act_3: "~~ῐδωσῐ|~~ῐδωσῐν", _, "~~ῐδοᾱσῐ|~~ῐδοᾱσῐν";
    ind_mid_1: "~~ῐδομαι", _, "~~ῐδομεθᾰ";
    ind_mid_2: "~~ῐδοσαι", "~~ῐδοσθον", "~~ῐδοσθε";
    ind_mid_3: "~~ῐδοται", _, "~~ῐδονται";
    ind_pass_1: _, _, "~~ῐδομεθᾰ";
    ind_pass_2: _, _, "~~ῐδοσθε";
    ind_pass_3: "~~ῐδοται", _, "~~ῐδονται";
    subj_act_1: "~~ῐδω", _, "~~ῐδωμεν";
    subj_act_2: "~~ῐδῳς", "~~ῐδωτον", "~~ῐδωτε";
    subj_act_3: "~~ῐδῳ", _, "~~ῐδωσῐ|~~ῐδωσῐν";
    subj_mid_1: "~~ῐδωμαι", _, "~~ῐδωμεθᾰ";
    subj_mid_2: "~~ῐδῳ", "~~ῐδωσθον", "~~ῐδωσθε";
    subj_mid_3: "~~ῐδωται", _, "~~ῐδωνται";
    subj_pass_1: _, _, "~~ῐδωμεθᾰ";
    subj_pass_2: _, _, "~~ῐδωσθε";
    subj_pass_3: "~~ῐδωται", _, "~~ῐδωνται";
    opt_act_1: "~~ῐδοιην", _, "~~ῐδοῑμεν|~~ῐδοιημεν";
    opt_act_2: "~~ῐδοιης", "~~ῐδοῑτον|~~ῐδοιητον", "~~ῐδοῑτε|~~ῐδοιητε";
    opt_act_3: "~~ῐδοιη", "~~ῐδοιτην|~~ῐδοιητην", "~~ῐδοῑεν|~~ῐδοιησᾰν";
    opt_mid_1: "~~ῐδοιμην", _, "~~ῐδοιμεθᾰ";
    opt_mid_2: "~~ῐδοῑο", "~~ῐδοῑσθον", "~~ῐδοῑσθε";
    opt_mid_3: "~~ῐδοῑτο", "~~ῐδοισθην", "~~ῐδοῑντο";
    opt_pass_1: _, _, "~~ῐδοιμεθᾰ";
    opt_pass_2: _, "~~ῐδοῑσθον", "~~ῐδοῑσθε";
    opt_pass_3: "~~ῐδοῑτο", "~~ῐδοισθην", "~~ῐδοῑντο";
    imp_act_2: "~~ῐδου", "~~ῐδοτον", "~~ῐδοτε";
    imp_act_3: "~~ῐδοτω", "~~ῐδοτων", "~~ῐδοντων";
    imp_mid_2: "~~ῐδοσο|~~ῐδου", _, "~~ῐδοσθε";
    imp_mid_3: "~~ῐδοσθω", "~~ῐδοσθων", "~~ῐδοσθων";
    imp_pass_2: "~~ῐδοσο|~~ῐδου", _, "~~ῐδοσθε";
    imp_pass_3: _, "~~ῐδοσθων", _;
    imperfect_act_1: "~υ~ῐδουν", _, "~υ~ῐδομεν";
    imperfect_act_2: "~υ~ῐδους", "~υ~ῐδοτον", "~υ~ῐδοτε";
    imperfect_act_3: "~υ~ῐδου", "~υ~ῐδοτην", "~υ~ῐδοσᾰν";
    imperfect_mid_1: "~υ~ῐδομην", _, "~υ~ῐδομεθᾰ";
    imperfect_mid_2: "~υ~ῐδοσο", "~υ~ῐδοσθον", "~υ~ῐδοσθε";
    imperfect_mid_3: "~υ~ῐδοτο", "~υ~ῐδοσθην", "~υ~ῐδοντο";
    imperfect_pass_1: _, _, "~υ~ῐδομεθᾰ";
    imperfect_pass_2: _, _, "~υ~ῐδοσθε";
    imperfect_pass_3: "~υ~ῐδοτο", _, _;
    future_ind_act_1: "~~ωσω", _, "~~ωσομεν";
    future_ind_act_2: "~~ωσεις", "~~ωσετον", "~~ωσετε";
    future_ind_act_3: "~~ωσει", _, "~~ωσουσῐ|~~ωσουσῐν";
    future_ind_mid_1: "~~ωσομαι", _, "~~ωσομεθᾰ";
    future_ind_mid_2: "~~ωσῃ|~~ωσει", "~~ωσεσθον", "~~ωσεσθε";
    future_ind_mid_3: "~~ωσεται", _, "~~ωσονται";
    future_ind_pass_1: "~~οθησομαι", _, "~~οθησομεθᾰ";
    future_ind_pass_2: "~~οθησῃ|~~οθησει", "~~οθησεσθον", "~~οθησεσθε";
    future_ind_pass_3: "~~οθησεται", _, "~~οθησονται";
    future_opt_act_1: "~~ωσοιμῐ", _, "~~ωσοιμεν";
    future_opt_act_2: "~~ωσοις", "~~ωσοιτον", "~~ωσοιτε";
    future_opt_act_3: "~~ωσοι", "~~ωσοιτην", "~~ωσοιεν";
    future_opt_mid_1: "~~ωσοιμην", _, "~~ωσοιμεθᾰ";
    future_opt_mid_2: "~~ωσοιο", "~~ωσοισθον", "~~ωσοισθε";
    future_opt_mid_3: "~~ωσοιτο", "~~ωσοισθην", "~~ωσοιντο";
    future_opt_pass_1: "~~οθησοιμην", _, "~~οθησοιμεθᾰ";
    future_opt_pass_2: "~~οθησοιο", "~~οθησοισθον", "~~οθησοισθε";
    future_opt_pass_3: "~~οθησοιτο", "~~οθησοισθην", "~~οθησοιντο";
    aorist_ind_act_1: _, _, "~υ~ομεν";
    aorist_ind_act_2: _, "~υ~οτον", "~υ~οτε";
    aorist_ind_act_3: _, "~υ~οτην", "~υ~οσᾰν";
    aorist_ind_mid_1: "~υ~ομην", _, "~υ~ομεθᾰ";
    aorist_ind_mid_2: "~υ~ου", "~υ~οσθον", "~υ~οσθε";
    aorist_ind_mid_3: "~υ~οτο", "~υ~οσθην", "~υ~οντο";
    aorist_ind_pass_1: "~υ~οθην", _, "~υ~οθημεν";
    aorist_ind_pass_2: "~υ~οθης", "~υ~οθητον", "~υ~οθητε";
    aorist_ind_pass_3: "~υ~οθη", "~υ~οθητην", "~υ~οθησᾰν";
    aorist_subj_act_1: "~~ω", _, "~~ωμεν";
    aorist_subj_act_2: "~~ῳς", "~~ωτον", "~~ωτε";
    aorist_subj_act_3: "~~ῳ", _, "~~ωσῐ|~~ωσῐν";
    aorist_subj_mid_1: "~~ωμαι", _, "~~ωμεθᾰ";
    aorist_subj_mid_2: "~~ῳ", "~~ωσθον", "~~ωσθε";
    aorist_subj_mid_3: "~~ωται", _, "~~ωνται";
    aorist_subj_pass_1: "~~οθω", _, "~~οθωμεν";
    aorist_subj_pass_2: "~~οθῃς", "~~οθητον", "~~οθητε";
    aorist_subj_pass_3: "~~οθῃ", _, "~~οθωσῐ|~~οθωσῐν";
    aorist_opt_act_1: "~~οιην", _, "~~οῑμεν|~~οιημεν";
    aorist_opt_act_2: "~~οιης", "~~οῑτον|~~οιητον", "~~οῑτε|~~οιητε";
    aorist_opt_act_3: "~~οιη", "~~οιτην|~~οιητην", "~~οῑεν|~~οιησᾰν";
    aorist_opt_mid_1: "~~οιμην", _, "~~οιμεθᾰ";
    aorist_opt_mid_2: "~~οῑο", "~~οῑσθον", "~~οῑσθε";
    aorist_opt_mid_3: "~~οῑτο", "~~οισθην", "~~οῑντο";
    aorist_opt_pass_1: "~~οθειην", _, "~~οθεῑμεν|~~οθειημεν";
    aorist_opt_pass_2: "~~οθειης", "~~οθεῑτον|~~οθειητον", "~~οθεῑτε|~~οθειητε";
    aorist_opt_pass_3: "~~οθειη", "~~οθειτην|~~οθειητην", "~~οθεῑεν|~~οθειησᾰν";
    aorist_imp_act_2: "~~ος", "~~οτον", "~~οτε";
    aorist_imp_act_3: "~~οτω", "~~οτων", "~~οντων";
    aorist_imp_mid_2: "~~ου", "~~οσθον", "~~οσθε";
    aorist_imp_mid_3: "~~οσθω", "~~οσθων", _;
    aorist_imp_pass_2: "~~οθηθῐ", _, _;
    aorist_imp_pass_3: "~~οθητω", "~~οθητων", "~~οθεντων";
    infinitive_pres: "~~ῐδοναι", "~~ῐδοσθαι", _;
    infinitive_fut: "~~ωσειν", "~~ωσεσθαι", "~~οθησεσθαι";
    infinitive_aor: "~~οῡναι", "~~οσθαι", "~~οθηναι";
    participle_pres: "~~ῐδους", "~~ῐδομενος", _;
    participle_fut: "~~ωσων", "~~ωσομενος", "~~οθησομενος";
    participle_aor: "~~ους", "~~ομενος", "~~οθεις";
    ),
];
