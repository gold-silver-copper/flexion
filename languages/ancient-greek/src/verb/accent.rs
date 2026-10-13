//! Accent. Each word has a class: an accent pattern, which says cell by cell
//! which rule places the accent, and the syllable its stem is accented on,
//! counted from the lemma's end. `Stem`: where the lemma has it (persistent);
//! `Ending`: on the ending's first syllable; `Recessive`: as far from the end
//! as the law of limitation allows. The law then moves it where it must, and
//! chooses acute or circumflex by the syllables' length.

use super::cell::*;
use flexion::{AccentPattern, Choice, StressClass};
use std::borrow::Cow;

/// Writes one accent pattern: the word it is named after, how many words
/// follow it, the pattern it is like (by its place, if any), then a row of
/// places per line, as the paradigm tables are laid out (`_`, or a row left
/// out: the place of the pattern it is like).
macro_rules! pattern {
    ($name:literal, $words:literal $(, like $parent:literal)?;
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
        AccentPattern {
            name: Cow::Borrowed($name),
            words: $words,
            parent: like!($($parent)?),
            places: Cow::Borrowed(concat!(
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

/// The pattern an accent pattern is like, by its place.
macro_rules! like {
    () => {
        None
    };
    ($parent:literal) => {
        Some($parent)
    };
}

/// The accent patterns, commonest first.
pub static PATTERNS: [AccentPattern<'static>; 36] = [
    // 0, `ἐχω`: 1,202 words.
    pattern!("ἐχω", 1202;
    ind_act_1: Stem, Stem, Stem;
    ind_act_2: Stem, Stem, Stem;
    ind_act_3: Stem, Stem, Stem;
    ind_mid_1: Stem, Stem, Recessive;
    ind_mid_2: Stem, Stem, Stem;
    ind_mid_3: Stem, Stem, Stem;
    ind_pass_1: Stem, Stem, Recessive;
    ind_pass_2: Stem, Stem, Stem;
    ind_pass_3: Stem, Stem, Stem;
    subj_act_1: Stem, Stem, Stem;
    subj_act_2: Stem, Stem, Stem;
    subj_act_3: Stem, Stem, Stem;
    subj_mid_1: Stem, Stem, Recessive;
    subj_mid_2: Stem, Stem, Stem;
    subj_mid_3: Stem, Stem, Stem;
    subj_pass_1: Stem, Stem, Recessive;
    subj_pass_2: Stem, Stem, Stem;
    subj_pass_3: Stem, Stem, Stem;
    opt_act_1: Recessive, Stem, Stem;
    opt_act_2: Stem, Stem, Stem;
    opt_act_3: Recessive, Recessive, Stem;
    opt_mid_1: Recessive, Stem, Recessive;
    opt_mid_2: Stem, Stem, Stem;
    opt_mid_3: Stem, Recessive, Stem;
    opt_pass_1: Recessive, Stem, Recessive;
    opt_pass_2: Stem, Stem, Stem;
    opt_pass_3: Stem, Recessive, Stem;
    imp_act_1: Stem, Stem, Stem;
    imp_act_2: Recessive, Stem, Stem;
    imp_act_3: Recessive, Recessive, Recessive;
    imp_mid_1: Stem, Stem, Stem;
    imp_mid_2: Stem, Stem, Stem;
    imp_mid_3: Recessive, Recessive, Recessive;
    imp_pass_1: Stem, Stem, Stem;
    imp_pass_2: Stem, Stem, Stem;
    imp_pass_3: Recessive, Recessive, Recessive;
    imperfect_act_1: Recessive, Stem, Stem;
    imperfect_act_2: Recessive, Stem, Stem;
    imperfect_act_3: Recessive, Recessive, Recessive;
    imperfect_mid_1: Recessive, Stem, Recessive;
    imperfect_mid_2: Stem, Stem, Stem;
    imperfect_mid_3: Stem, Recessive, Stem;
    imperfect_pass_1: Recessive, Stem, Recessive;
    imperfect_pass_2: Stem, Stem, Stem;
    imperfect_pass_3: Stem, Recessive, Stem;
    future_ind_act_1: Recessive, Stem, Stem;
    future_ind_act_2: Recessive, Stem, Stem;
    future_ind_act_3: Recessive, Stem, Stem;
    future_ind_mid_1: Recessive, Stem, Recessive;
    future_ind_mid_2: Recessive, Recessive, Recessive;
    future_ind_mid_3: Recessive, Recessive, Recessive;
    future_ind_pass_1: Recessive, Stem, Recessive;
    future_ind_pass_2: Recessive, Recessive, Recessive;
    future_ind_pass_3: Recessive, Recessive, Recessive;
    future_subj_act_1: Stem, Stem, Stem;
    future_subj_act_2: Stem, Stem, Stem;
    future_subj_act_3: Stem, Stem, Stem;
    future_subj_mid_1: Stem, Stem, Stem;
    future_subj_mid_2: Stem, Stem, Stem;
    future_subj_mid_3: Stem, Stem, Stem;
    future_subj_pass_1: Stem, Stem, Stem;
    future_subj_pass_2: Stem, Stem, Stem;
    future_subj_pass_3: Stem, Stem, Stem;
    future_opt_act_1: Recessive, Stem, Stem;
    future_opt_act_2: Recessive, Stem, Stem;
    future_opt_act_3: Recessive, Recessive, Stem;
    future_opt_mid_1: Recessive, Stem, Recessive;
    future_opt_mid_2: Recessive, Recessive, Recessive;
    future_opt_mid_3: Recessive, Recessive, Recessive;
    future_opt_pass_1: Recessive, Stem, Recessive;
    future_opt_pass_2: Recessive, Recessive, Recessive;
    future_opt_pass_3: Recessive, Recessive, Recessive;
    future_imp_act_1: Stem, Stem, Stem;
    future_imp_act_2: Stem, Stem, Stem;
    future_imp_act_3: Stem, Stem, Stem;
    future_imp_mid_1: Stem, Stem, Stem;
    future_imp_mid_2: Stem, Stem, Stem;
    future_imp_mid_3: Stem, Stem, Stem;
    future_imp_pass_1: Stem, Stem, Stem;
    future_imp_pass_2: Stem, Stem, Stem;
    future_imp_pass_3: Stem, Stem, Stem;
    aorist_ind_act_1: Recessive, Stem, Recessive;
    aorist_ind_act_2: Recessive, Recessive, Recessive;
    aorist_ind_act_3: Recessive, Recessive, Recessive;
    aorist_ind_mid_1: Recessive, Stem, Recessive;
    aorist_ind_mid_2: Recessive, Recessive, Recessive;
    aorist_ind_mid_3: Recessive, Recessive, Recessive;
    aorist_ind_pass_1: Recessive, Stem, Recessive;
    aorist_ind_pass_2: Recessive, Recessive, Recessive;
    aorist_ind_pass_3: Recessive, Stem, Recessive;
    aorist_subj_act_1: Recessive, Stem, Recessive;
    aorist_subj_act_2: Recessive, Recessive, Recessive;
    aorist_subj_act_3: Recessive, Recessive, Recessive;
    aorist_subj_mid_1: Recessive, Stem, Recessive;
    aorist_subj_mid_2: Recessive, Recessive, Recessive;
    aorist_subj_mid_3: Recessive, Recessive, Recessive;
    aorist_subj_pass_1: Ending, Stem, Ending;
    aorist_subj_pass_2: Ending, Ending, Ending;
    aorist_subj_pass_3: Ending, Ending, Ending;
    aorist_opt_act_1: Recessive, Stem, Recessive;
    aorist_opt_act_2: Recessive, Recessive, Recessive;
    aorist_opt_act_3: Recessive, Recessive, Recessive;
    aorist_opt_mid_1: Recessive, Stem, Recessive;
    aorist_opt_mid_2: Recessive, Recessive, Recessive;
    aorist_opt_mid_3: Recessive, Recessive, Recessive;
    aorist_opt_pass_1: Stem, Stem, Ending;
    aorist_opt_pass_2: Stem, Ending, Ending;
    aorist_opt_pass_3: Stem, Stem, Ending;
    aorist_imp_act_1: Stem, Stem, Stem;
    aorist_imp_act_2: Recessive, Recessive, Recessive;
    aorist_imp_act_3: Recessive, Recessive, Recessive;
    aorist_imp_mid_1: Stem, Stem, Stem;
    aorist_imp_mid_2: Recessive, Recessive, Recessive;
    aorist_imp_mid_3: Recessive, Recessive, Recessive;
    aorist_imp_pass_1: Stem, Stem, Stem;
    aorist_imp_pass_2: Recessive, Recessive, Recessive;
    aorist_imp_pass_3: Stem, Stem, Stem;
    perfect_ind_act_1: Recessive, Stem, Recessive;
    perfect_ind_act_2: Recessive, Recessive, Recessive;
    perfect_ind_act_3: Recessive, Recessive, Recessive;
    perfect_ind_mid_1: Recessive, Stem, Recessive;
    perfect_ind_mid_2: Recessive, Recessive, Recessive;
    perfect_ind_mid_3: Recessive, Recessive, Recessive;
    perfect_ind_pass_1: Recessive, Stem, Recessive;
    perfect_ind_pass_2: Recessive, Recessive, Recessive;
    perfect_ind_pass_3: Recessive, Recessive, Recessive;
    perfect_subj_act_1: Recessive, Stem, Recessive;
    perfect_subj_act_2: Recessive, Recessive, Recessive;
    perfect_subj_act_3: Recessive, Recessive, Recessive;
    perfect_subj_mid_1: Stem, Stem, Stem;
    perfect_subj_mid_2: Stem, Stem, Stem;
    perfect_subj_mid_3: Stem, Stem, Stem;
    perfect_subj_pass_1: Stem, Stem, Stem;
    perfect_subj_pass_2: Stem, Stem, Stem;
    perfect_subj_pass_3: Stem, Stem, Stem;
    perfect_opt_act_1: Recessive, Stem, Recessive;
    perfect_opt_act_2: Recessive, Recessive, Recessive;
    perfect_opt_act_3: Recessive, Stem, Recessive;
    perfect_opt_mid_1: Stem, Stem, Stem;
    perfect_opt_mid_2: Stem, Stem, Stem;
    perfect_opt_mid_3: Stem, Stem, Stem;
    perfect_opt_pass_1: Stem, Stem, Stem;
    perfect_opt_pass_2: Stem, Stem, Stem;
    perfect_opt_pass_3: Stem, Stem, Stem;
    perfect_imp_act_1: Stem, Stem, Stem;
    perfect_imp_act_2: Recessive, Recessive, Recessive;
    perfect_imp_act_3: Stem, Stem, Stem;
    perfect_imp_mid_1: Stem, Stem, Stem;
    perfect_imp_mid_2: Recessive, Recessive, Recessive;
    perfect_imp_mid_3: Recessive, Recessive, Recessive;
    perfect_imp_pass_1: Stem, Stem, Stem;
    perfect_imp_pass_2: Recessive, Recessive, Recessive;
    perfect_imp_pass_3: Recessive, Recessive, Recessive;
    pluperfect_act_1: Recessive, Stem, Recessive;
    pluperfect_act_2: Recessive, Recessive, Recessive;
    pluperfect_act_3: Recessive, Stem, Recessive;
    pluperfect_mid_1: Recessive, Stem, Recessive;
    pluperfect_mid_2: Recessive, Recessive, Recessive;
    pluperfect_mid_3: Recessive, Recessive, Recessive;
    pluperfect_pass_1: Recessive, Stem, Recessive;
    pluperfect_pass_2: Recessive, Recessive, Recessive;
    pluperfect_pass_3: Recessive, Recessive, Recessive;
    future_perfect_ind_act_1: Stem, Stem, Stem;
    future_perfect_ind_act_2: Stem, Stem, Stem;
    future_perfect_ind_act_3: Stem, Stem, Stem;
    future_perfect_ind_mid_1: Stem, Stem, Stem;
    future_perfect_ind_mid_2: Stem, Stem, Stem;
    future_perfect_ind_mid_3: Stem, Stem, Stem;
    future_perfect_ind_pass_1: Stem, Stem, Stem;
    future_perfect_ind_pass_2: Stem, Stem, Stem;
    future_perfect_ind_pass_3: Stem, Stem, Stem;
    future_perfect_subj_act_1: Stem, Stem, Stem;
    future_perfect_subj_act_2: Stem, Stem, Stem;
    future_perfect_subj_act_3: Stem, Stem, Stem;
    future_perfect_subj_mid_1: Stem, Stem, Stem;
    future_perfect_subj_mid_2: Stem, Stem, Stem;
    future_perfect_subj_mid_3: Stem, Stem, Stem;
    future_perfect_subj_pass_1: Stem, Stem, Stem;
    future_perfect_subj_pass_2: Stem, Stem, Stem;
    future_perfect_subj_pass_3: Stem, Stem, Stem;
    future_perfect_opt_act_1: Stem, Stem, Stem;
    future_perfect_opt_act_2: Stem, Stem, Stem;
    future_perfect_opt_act_3: Stem, Stem, Stem;
    future_perfect_opt_mid_1: Stem, Stem, Stem;
    future_perfect_opt_mid_2: Stem, Stem, Stem;
    future_perfect_opt_mid_3: Stem, Stem, Stem;
    future_perfect_opt_pass_1: Stem, Stem, Stem;
    future_perfect_opt_pass_2: Stem, Stem, Stem;
    future_perfect_opt_pass_3: Stem, Stem, Stem;
    future_perfect_imp_act_1: Stem, Stem, Stem;
    future_perfect_imp_act_2: Stem, Stem, Stem;
    future_perfect_imp_act_3: Stem, Stem, Stem;
    future_perfect_imp_mid_1: Stem, Stem, Stem;
    future_perfect_imp_mid_2: Stem, Stem, Stem;
    future_perfect_imp_mid_3: Stem, Stem, Stem;
    future_perfect_imp_pass_1: Stem, Stem, Stem;
    future_perfect_imp_pass_2: Stem, Stem, Stem;
    future_perfect_imp_pass_3: Stem, Stem, Stem;
    infinitive_pres: Stem, Stem, Stem;
    infinitive_fut: Recessive, Recessive, Recessive;
    infinitive_aor: Stem, Stem, Ending;
    infinitive_perf: Stem, Ending, Ending;
    infinitive_futp: Stem, Stem, Stem;
    participle_pres: Stem, Recessive, Recessive;
    participle_fut: Recessive, Recessive, Recessive;
    participle_aor: Stem, Recessive, Ending;
    participle_perf: Ending, Ending, Ending;
    participle_futp: Stem, Stem, Stem;
    ),
    // 1, `ποιεω`: 259 words. Like 0 (`ἐχω`), except in 30 cells.
    pattern!("ποιεω", 259, like 0;
    imperfect_act_1: _, _, Ending;
    imperfect_act_2: _, Ending, Ending;
    imperfect_mid_2: Ending, Ending, Ending;
    imperfect_mid_3: Ending, _, Ending;
    imperfect_pass_2: Ending, Ending, Ending;
    imperfect_pass_3: Ending, _, Ending;
    aorist_subj_pass_1: Stem, _, Stem;
    aorist_subj_pass_2: Stem, Stem, Stem;
    aorist_subj_pass_3: Stem, Stem, Stem;
    aorist_opt_pass_1: _, _, Stem;
    aorist_opt_pass_2: _, Stem, Stem;
    aorist_opt_pass_3: _, _, Stem;
    infinitive_aor: _, _, Stem;
    participle_aor: _, _, Stem;
    participle_perf: Stem, Stem, Stem;
    ),
    // 2, `ἀναιρεω`: 163 words. Like 0 (`ἐχω`), except in 2 cells.
    pattern!("ἀναιρεω", 163, like 0;
    infinitive_perf: _, Stem, Stem;
    ),
    // 3, `καθιζω`: 156 words. Like 2 (`ἀναιρεω`), except in 27 cells.
    pattern!("καθιζω", 156, like 2;
    future_ind_act_1: Ending, _, Ending;
    future_ind_act_2: Ending, Ending, Ending;
    future_ind_act_3: Ending, Ending, Ending;
    future_ind_mid_1: Ending, _, _;
    future_ind_mid_2: Ending, Ending, Ending;
    future_ind_mid_3: Ending, Ending, Ending;
    future_opt_act_1: _, _, Ending;
    future_opt_act_2: _, Ending, Ending;
    future_opt_act_3: _, _, Ending;
    future_opt_mid_2: Ending, Ending, Ending;
    future_opt_mid_3: Ending, _, Ending;
    infinitive_fut: Ending, Ending, _;
    participle_fut: Ending, _, _;
    ),
    // 4, `προβαλλω`: 36 words. Like 2 (`ἀναιρεω`), except in 4 cells.
    pattern!("προβαλλω", 36, like 2;
    aorist_imp_mid_2: Ending, _, _;
    infinitive_aor: Ending, Ending, _;
    participle_aor: Ending, _, _;
    ),
    // 5, `ἀνιστημι`: 36 words. Like 0 (`ἐχω`), except in 44 cells.
    pattern!("ἀνιστημι", 36, like 0;
    ind_act_3: _, _, Ending;
    subj_act_1: Ending, _, Ending;
    subj_act_2: Ending, Ending, Ending;
    subj_act_3: Ending, Ending, Ending;
    subj_mid_1: Ending, _, _;
    subj_mid_2: Ending, Ending, Ending;
    subj_mid_3: Ending, Ending, Ending;
    subj_pass_1: Ending, _, _;
    subj_pass_2: Ending, Ending, Ending;
    subj_pass_3: Ending, Ending, Ending;
    opt_act_1: _, _, Ending;
    opt_act_2: _, Ending, Ending;
    opt_act_3: _, _, Ending;
    aorist_subj_pass_1: Stem, _, Stem;
    aorist_subj_pass_2: Stem, Stem, Stem;
    aorist_subj_pass_3: Stem, Stem, Stem;
    aorist_opt_pass_1: _, _, Stem;
    aorist_opt_pass_2: _, Stem, Stem;
    aorist_opt_pass_3: _, _, Stem;
    infinitive_aor: Ending, _, Stem;
    participle_pres: Ending, _, _;
    participle_aor: _, _, Stem;
    participle_perf: Stem, _, _;
    ),
    // 6, `ἀφῑημῐ`: 25 words. Like 0 (`ἐχω`), except in 5 cells.
    pattern!("ἀφῑημῐ", 25, like 0;
    aorist_ind_act_1: Ending, _, _;
    aorist_ind_act_2: Ending, _, _;
    aorist_ind_act_3: Ending, _, Ending;
    participle_perf: Stem, _, _;
    ),
    // 7, `ὀφειλω`: 24 words. Like 0 (`ἐχω`), except in 1 cell.
    pattern!("ὀφειλω", 24, like 0;
    infinitive_aor: Ending, _, _;
    ),
    // 8, `ἐπῐλᾰμβᾰνω`: 22 words. Like 2 (`ἀναιρεω`), except in 27 cells.
    pattern!("ἐπῐλᾰμβᾰνω", 22, like 2;
    future_ind_act_1: _, _, Recessive;
    future_ind_act_2: _, Recessive, Recessive;
    future_ind_act_3: _, Recessive, Recessive;
    future_opt_act_1: _, _, Recessive;
    future_opt_act_2: _, Recessive, Recessive;
    future_opt_act_3: _, _, Recessive;
    aorist_subj_pass_1: Stem, _, Stem;
    aorist_subj_pass_2: Stem, Stem, Stem;
    aorist_subj_pass_3: Stem, Stem, Stem;
    aorist_opt_pass_1: _, _, Stem;
    aorist_opt_pass_2: _, Stem, Stem;
    aorist_opt_pass_3: _, _, Stem;
    infinitive_aor: Recessive, Recessive, Stem;
    participle_aor: Recessive, _, _;
    participle_perf: _, Stem, Stem;
    ),
    // 9, `ὑπαρχω`: 18 words. Like 0 (`ἐχω`), except in 8 cells.
    pattern!("ὑπαρχω", 18, like 0;
    imperfect_act_1: Stem, _, _;
    imperfect_act_2: Stem, _, _;
    imperfect_act_3: Stem, _, Stem;
    aorist_ind_act_1: Stem, _, _;
    aorist_ind_act_2: Stem, _, _;
    aorist_ind_act_3: Stem, _, Stem;
    ),
    // 10, `τῐθημῐ`: 17 words. Like 5 (`ἀνιστημι`), except in 28 cells.
    pattern!("τῐθημῐ", 17, like 5;
    ind_act_3: _, _, Stem;
    opt_mid_2: Ending, Ending, Ending;
    opt_mid_3: Ending, _, Ending;
    opt_pass_2: Ending, Ending, Ending;
    opt_pass_3: Ending, _, Ending;
    aorist_subj_pass_1: Ending, _, Ending;
    aorist_subj_pass_2: Ending, Ending, Ending;
    aorist_subj_pass_3: Ending, Ending, Ending;
    aorist_opt_pass_1: _, _, Ending;
    aorist_opt_pass_2: _, Ending, Ending;
    aorist_opt_pass_3: _, _, Ending;
    infinitive_aor: Stem, _, Ending;
    participle_aor: _, _, Ending;
    participle_perf: _, Stem, Stem;
    ),
    // 11, `ἐκλυτροω`: 14 words. Like 0 (`ἐχω`), except in 13 cells.
    pattern!("ἐκλυτροω", 14, like 0;
    imperfect_act_1: _, _, Ending;
    imperfect_act_2: _, Ending, Ending;
    imperfect_mid_2: Ending, Ending, Ending;
    imperfect_mid_3: Ending, _, Ending;
    imperfect_pass_2: Ending, Ending, Ending;
    imperfect_pass_3: Ending, _, Ending;
    ),
    // 12, `ἡγεομαι`: 14 words. Like 1 (`ποιεω`), except in 50 cells.
    pattern!("ἡγεομαι", 14, like 1;
    ind_mid_1: Ending, _, _;
    ind_mid_2: Ending, Ending, Ending;
    ind_mid_3: Ending, Ending, Ending;
    ind_pass_1: Ending, _, _;
    ind_pass_2: Ending, Ending, Ending;
    ind_pass_3: Ending, Ending, Ending;
    subj_mid_1: Ending, _, _;
    subj_mid_2: Ending, Ending, Ending;
    subj_mid_3: Ending, Ending, Ending;
    subj_pass_1: Ending, _, _;
    subj_pass_2: Ending, Ending, Ending;
    subj_pass_3: Ending, Ending, Ending;
    opt_mid_2: Ending, Ending, Ending;
    opt_mid_3: Ending, _, Ending;
    opt_pass_2: Ending, Ending, Ending;
    opt_pass_3: Ending, _, Ending;
    imp_mid_2: Ending, Ending, Ending;
    imp_pass_2: Ending, Ending, Ending;
    imperfect_act_1: _, _, Stem;
    imperfect_act_2: _, Stem, Stem;
    infinitive_pres: _, Ending, Ending;
    participle_perf: Ending, _, _;
    ),
    // 13, `τελεω`: 11 words. Like 0 (`ἐχω`), except in 18 cells.
    pattern!("τελεω", 11, like 0;
    future_ind_act_1: Stem, _, _;
    future_ind_act_2: Stem, _, _;
    future_ind_act_3: Stem, _, _;
    future_ind_mid_1: Stem, _, _;
    future_ind_mid_2: Stem, Stem, Stem;
    future_ind_mid_3: Stem, Stem, Stem;
    future_opt_mid_2: Stem, Stem, Stem;
    future_opt_mid_3: Stem, _, Stem;
    infinitive_fut: Stem, Stem, _;
    participle_fut: Stem, _, _;
    ),
    // 14, `προσευχομαι`: 10 words. Like 12 (`ἡγεομαι`), except in 26 cells.
    pattern!("προσευχομαι", 10, like 12;
    imperfect_mid_2: Stem, Stem, Stem;
    imperfect_mid_3: Stem, _, Stem;
    imperfect_pass_2: Stem, Stem, Stem;
    imperfect_pass_3: Stem, _, Stem;
    aorist_subj_pass_1: Ending, _, Ending;
    aorist_subj_pass_2: Ending, Ending, Ending;
    aorist_subj_pass_3: Ending, Ending, Ending;
    aorist_opt_pass_1: _, _, Ending;
    aorist_opt_pass_2: _, Ending, Ending;
    aorist_opt_pass_3: _, _, Ending;
    infinitive_aor: _, _, Ending;
    participle_aor: _, _, Ending;
    participle_perf: _, Ending, Ending;
    ),
    // 15, `κᾰλεω`: 8 words. Like 2 (`ἀναιρεω`), except in 14 cells.
    pattern!("κᾰλεω", 8, like 2;
    aorist_subj_pass_1: Stem, _, Stem;
    aorist_subj_pass_2: Stem, Stem, Stem;
    aorist_subj_pass_3: Stem, Stem, Stem;
    aorist_opt_pass_1: _, _, Stem;
    aorist_opt_pass_2: _, Stem, Stem;
    aorist_opt_pass_3: _, _, Stem;
    infinitive_aor: _, _, Stem;
    participle_aor: _, _, Stem;
    ),
    // 16, `ὑπερεχω`: 8 words. Like 0 (`ἐχω`), except in 1 cell.
    pattern!("ὑπερεχω", 8, like 0;
    aorist_imp_mid_2: Stem, _, _;
    ),
    // 17, `ὑπολαμβανω`: 7 words. Like 15 (`κᾰλεω`), except in 5 cells.
    pattern!("ὑπολαμβανω", 7, like 15;
    infinitive_perf: _, Ending, Ending;
    participle_perf: Stem, Stem, Stem;
    ),
    // 18, `ἀρνυμαι`: 7 words. Like 0 (`ἐχω`), except in 2 cells.
    pattern!("ἀρνυμαι", 7, like 0;
    aorist_imp_mid_2: Ending, _, _;
    infinitive_aor: _, Ending, _;
    ),
    // 19, `ἀναγω`: 7 words. Like 0 (`ἐχω`), except in 4 cells.
    pattern!("ἀναγω", 7, like 0;
    imperfect_act_1: Stem, _, _;
    imperfect_act_2: Stem, _, _;
    imperfect_act_3: Stem, _, Stem;
    ),
    // 20, `ζωννῡμῐ`: 7 words. Like 2 (`ἀναιρεω`), except in 1 cell.
    pattern!("ζωννῡμῐ", 7, like 2;
    participle_pres: Ending, _, _;
    ),
    // 21, `ἀειρω`: 6 words. Like 13 (`τελεω`), except in 2 cells.
    pattern!("ἀειρω", 6, like 13;
    infinitive_perf: _, Stem, Stem;
    ),
    // 22, `προσκειμαι`: 6 words. Like 0 (`ἐχω`), except in 6 cells.
    pattern!("προσκειμαι", 6, like 0;
    subj_mid_1: Ending, _, _;
    opt_mid_2: Ending, Ending, Ending;
    opt_mid_3: Ending, _, Ending;
    ),
    // 23, `λᾰμβᾰνω`: 4 words. Like 15 (`κᾰλεω`), except in 5 cells.
    pattern!("λᾰμβᾰνω", 4, like 15;
    aorist_imp_act_2: Stem, _, _;
    aorist_imp_mid_2: Stem, _, _;
    participle_aor: _, _, Ending;
    participle_perf: _, Stem, Stem;
    ),
    // 24, `προσᾰγω`: 3 words. Like 16 (`ὑπερεχω`), except in 4 cells.
    pattern!("προσᾰγω", 3, like 16;
    aorist_ind_act_1: Stem, _, _;
    aorist_ind_act_2: Stem, _, _;
    aorist_ind_act_3: Stem, _, Stem;
    ),
    // 25, `ὑποφερω`: 3 words. Like 0 (`ἐχω`), except in 9 cells.
    pattern!("ὑποφερω", 3, like 0;
    future_ind_act_1: _, _, Recessive;
    future_ind_act_2: _, Recessive, Recessive;
    future_ind_act_3: _, Recessive, Recessive;
    future_opt_act_1: _, _, Recessive;
    future_opt_act_2: _, Recessive, Recessive;
    future_opt_act_3: _, _, Recessive;
    ),
    // 26, `νεμω`: 2 words. Like 3 (`καθιζω`), except in 2 cells.
    pattern!("νεμω", 2, like 3;
    infinitive_perf: _, Ending, Ending;
    ),
    // 27, `ἀνιημῐ`: 2 words. Like 7 (`ὀφειλω`), except in 1 cell.
    pattern!("ἀνιημῐ", 2, like 7;
    participle_aor: Ending, _, _;
    ),
    // 28, `ἐφεπω`: 2 words. Like 15 (`κᾰλεω`), except in 5 cells.
    pattern!("ἐφεπω", 2, like 15;
    aorist_ind_act_1: Stem, _, _;
    aorist_ind_act_2: Stem, _, _;
    aorist_ind_act_3: Stem, _, Stem;
    aorist_imp_mid_2: Stem, _, _;
    ),
    // 29, `ἀντιλεγω`: 2 words. Like 21 (`ἀειρω`), except in 2 cells.
    pattern!("ἀντιλεγω", 2, like 21;
    aorist_imp_mid_2: Stem, _, _;
    participle_aor: Ending, _, _;
    ),
    // 30, `ὠνεομαι`: 2 words. Like 0 (`ἐχω`), except in 1 cell.
    pattern!("ὠνεομαι", 2, like 0;
    infinitive_aor: _, Recessive, _;
    ),
    // 31, `δῐᾰφερω`: 2 words. Like 25 (`ὑποφερω`), except in 2 cells.
    pattern!("δῐᾰφερω", 2, like 25;
    infinitive_perf: _, Stem, Stem;
    ),
    // 32, `ἀειδω`: 1 words. Like 2 (`ἀναιρεω`), except in 2 cells.
    pattern!("ἀειδω", 1, like 2;
    participle_perf: _, Stem, Stem;
    ),
    // 33, `γῐγνωσκω`: 1 words. Like 21 (`ἀειρω`), except in 14 cells.
    pattern!("γῐγνωσκω", 1, like 21;
    aorist_subj_pass_1: Stem, _, Stem;
    aorist_subj_pass_2: Stem, Stem, Stem;
    aorist_subj_pass_3: Stem, Stem, Stem;
    aorist_opt_pass_1: _, _, Stem;
    aorist_opt_pass_2: _, Stem, Stem;
    aorist_opt_pass_3: _, _, Stem;
    infinitive_aor: _, _, Stem;
    participle_aor: _, _, Stem;
    ),
    // 34, `δοκεω`: 1 words. Like 1 (`ποιεω`), except in 5 cells.
    pattern!("δοκεω", 1, like 1;
    infinitive_perf: _, Stem, Stem;
    participle_perf: Ending, Ending, Ending;
    ),
    // 35, `κᾰθιημῐ`: 1 words. Like 6 (`ἀφῑημῐ`), except in 3 cells.
    pattern!("κᾰθιημῐ", 1, like 6;
    aorist_imp_act_2: Stem, _, _;
    aorist_imp_mid_2: Stem, _, _;
    participle_perf: Ending, _, _;
    ),
];

/// Every accent class: a pattern and the stem's accented syllable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccentClass {
    /// Pattern `ἐχω`, the stem accented on its next-to-last: 1,023 words.
    Ἐχω,
    /// Pattern `ποιεω`, the stem accented on its next-to-last: 245 words.
    Ποιεω,
    /// Pattern `ἐχω`, the stem accented on its syllable 3 from the end: 161 words.
    Οἰομαι,
    /// Pattern `ἀναιρεω`, the stem accented on its next-to-last: 141 words.
    Ἀναιρεω,
    /// Pattern `καθιζω`, the stem accented on its next-to-last: 140 words.
    Καθιζω,
    /// Pattern `ἀνιστημι`, the stem accented on its syllable 3 from the end: 34 words.
    Ἀνιστημι,
    /// Pattern `προβαλλω`, the stem accented on its next-to-last: 33 words.
    Προβαλλω,
    /// Pattern `ὀφειλω`, the stem accented on its next-to-last: 22 words.
    Ὀφειλω,
    /// Pattern `ἀναιρεω`, the stem accented on its syllable 3 from the end: 21 words.
    Κᾰτεργαζομαι,
    /// Pattern `ἐπῐλᾰμβᾰνω`, the stem accented on its next-to-last: 19 words.
    Ἐπῐλᾰμβᾰνω,
    /// Pattern `τῐθημῐ`, the stem accented on its syllable 3 from the end: 17 words.
    Τῐθημῐ,
    /// Pattern `ὑπαρχω`, the stem accented on its next-to-last: 16 words.
    Ὑπαρχω,
    /// Pattern `καθιζω`, the stem accented on its syllable 3 from the end: 15 words.
    Λογῐζομαι,
    /// Pattern `ποιεω`, the stem accented on its syllable 3 from the end: 14 words.
    Περιειμι,
    /// Pattern `ἐχω`, the stem accented on the lemma's last syllable: 11 words.
    Δεῑ,
    /// Pattern `ἀφῑημῐ`, the stem accented on its syllable 3 from the end: 11 words.
    Ἀφῑημῐ,
    /// Pattern `ἀφῑημῐ`, the stem accented on its syllable 4 from the end: 11 words.
    Πᾰρερχομαι,
    /// Pattern `ἐκλυτροω`, the stem accented on its next-to-last: 11 words.
    Ἐκλυτροω,
    /// Pattern `τελεω`, the stem accented on its next-to-last: 10 words.
    Τελεω,
    /// Pattern `ἡγεομαι`, the stem accented on its syllable 4 from the end: 9 words.
    Ἡγεομαι,
    /// Pattern `προσευχομαι`, never on the stem: 8 words.
    Προσευχομαι,
    /// Pattern `κᾰλεω`, the stem accented on its next-to-last: 7 words.
    Κᾰλεω,
    /// Pattern `ὑπολαμβανω`, the stem accented on its next-to-last: 7 words.
    Ὑπολαμβανω,
    /// Pattern `ἀναγω`, the stem accented on its next-to-last: 7 words.
    Ἀναγω,
    /// Pattern `ζωννῡμῐ`, the stem accented on its syllable 3 from the end: 7 words.
    Ζωννῡμῐ,
    /// Pattern `ὑπερεχω`, the stem accented on its next-to-last: 6 words.
    Ὑπερεχω,
    /// Pattern `ἀρνυμαι`, the stem accented on its syllable 3 from the end: 6 words.
    Ἀρνυμαι,
    /// Pattern `ἀειρω`, the stem accented on its next-to-last: 6 words.
    Ἀειρω,
    /// Pattern `προσκειμαι`, the stem accented on its syllable 3 from the end: 6 words.
    Προσκειμαι,
    /// Pattern `ἐχω`, the stem accented on its syllable 4 from the end: 5 words.
    Ἐπευχομαι,
    /// Pattern `ἡγεομαι`, never on the stem: 4 words.
    Νεομαι,
    /// Pattern `λᾰμβᾰνω`, the stem accented on its next-to-last: 4 words.
    Λᾰμβᾰνω,
    /// Pattern `προβαλλω`, the stem accented on its syllable 3 from the end: 3 words.
    Ἐπορον,
    /// Pattern `ἀφῑημῐ`, never on the stem: 3 words.
    Προεῑπον,
    /// Pattern `προσᾰγω`, the stem accented on its next-to-last: 3 words.
    Προσᾰγω,
    /// Pattern `ὑποφερω`, the stem accented on its next-to-last: 3 words.
    Ὑποφερω,
    /// Pattern `ἐχω`, never on the stem: 2 words.
    Ὀττευομαι,
    /// Pattern `ὀφειλω`, the stem accented on its syllable 3 from the end: 2 words.
    Ἐμπηγνῡμῐ,
    /// Pattern `ὑπαρχω`, the stem accented on its syllable 3 from the end: 2 words.
    Ἐπορνῡμῐ,
    /// Pattern `ἐκλυτροω`, the stem accented on its syllable 3 from the end: 2 words.
    Δημοκρᾰτεομαι,
    /// Pattern `προσευχομαι`, the stem accented on its syllable 4 from the end: 2 words.
    Δᾰτεομαι,
    /// Pattern `ὑπερεχω`, the stem accented on its syllable 3 from the end: 2 words.
    Συνεπομαι,
    /// Pattern `ἐφεπω`, the stem accented on its next-to-last: 2 words.
    Ἐφεπω,
    /// Pattern `ἀντιλεγω`, the stem accented on its next-to-last: 2 words.
    Ἀντιλεγω,
    /// Pattern `δῐᾰφερω`, the stem accented on its next-to-last: 2 words.
    Δῐᾰφερω,
    /// Pattern `ἀναιρεω`, never on the stem: 1 words.
    Ἐδω,
    /// Pattern `καθιζω`, never on the stem: 1 words.
    Περνημῐ,
    /// Pattern `ἀνιστημι`, never on the stem: 1 words.
    Χρη,
    /// Pattern `ἀνιστημι`, the stem accented on the lemma's last syllable: 1 words.
    Εἰμῐ,
    /// Pattern `ἐπῐλᾰμβᾰνω`, never on the stem: 1 words.
    Ἐοικᾰ,
    /// Pattern `ἐπῐλᾰμβᾰνω`, the stem accented on the lemma's last syllable: 1 words.
    Δρᾱω,
    /// Pattern `ἐπῐλᾰμβᾰνω`, the stem accented on its syllable 3 from the end: 1 words.
    Ἀραρισκω,
    /// Pattern `ἐκλυτροω`, the stem accented on the lemma's last syllable: 1 words.
    Ζω,
    /// Pattern `ἡγεομαι`, the stem accented on the lemma's last syllable: 1 words.
    Φημῐ,
    /// Pattern `τελεω`, the stem accented on its syllable 3 from the end: 1 words.
    Σκεδᾰννῡμῐ,
    /// Pattern `κᾰλεω`, the stem accented on its syllable 3 from the end: 1 words.
    Ἐπιμιμνησκομαι,
    /// Pattern `ἀρνυμαι`, the stem accented on its next-to-last: 1 words.
    Περθω,
    /// Pattern `νεμω`, the stem accented on its next-to-last: 1 words.
    Νεμω,
    /// Pattern `νεμω`, the stem accented on its syllable 3 from the end: 1 words.
    Στορνῡμῐ,
    /// Pattern `ἀνιημῐ`, the stem accented on its syllable 3 from the end: 1 words.
    Ἐπεφνον,
    /// Pattern `ἀνιημῐ`, the stem accented on its syllable 4 from the end: 1 words.
    Ἀνιημῐ,
    /// Pattern `ὠνεομαι`, the stem accented on its next-to-last: 1 words.
    Μεθῠσκω,
    /// Pattern `ὠνεομαι`, the stem accented on its syllable 3 from the end: 1 words.
    Ὠνεομαι,
    /// Pattern `ἀειδω`, the stem accented on its next-to-last: 1 words.
    Ἀειδω,
    /// Pattern `γῐγνωσκω`, the stem accented on its next-to-last: 1 words.
    Γῐγνωσκω,
    /// Pattern `δοκεω`, the stem accented on its next-to-last: 1 words.
    Δοκεω,
    /// Pattern `κᾰθιημῐ`, the stem accented on its syllable 3 from the end: 1 words.
    Κᾰθιημῐ,
}

impl AccentClass {
    /// Every class, in order.
    pub const ALL: [AccentClass; 67] = [
        AccentClass::Ἐχω,
        AccentClass::Ποιεω,
        AccentClass::Οἰομαι,
        AccentClass::Ἀναιρεω,
        AccentClass::Καθιζω,
        AccentClass::Ἀνιστημι,
        AccentClass::Προβαλλω,
        AccentClass::Ὀφειλω,
        AccentClass::Κᾰτεργαζομαι,
        AccentClass::Ἐπῐλᾰμβᾰνω,
        AccentClass::Τῐθημῐ,
        AccentClass::Ὑπαρχω,
        AccentClass::Λογῐζομαι,
        AccentClass::Περιειμι,
        AccentClass::Δεῑ,
        AccentClass::Ἀφῑημῐ,
        AccentClass::Πᾰρερχομαι,
        AccentClass::Ἐκλυτροω,
        AccentClass::Τελεω,
        AccentClass::Ἡγεομαι,
        AccentClass::Προσευχομαι,
        AccentClass::Κᾰλεω,
        AccentClass::Ὑπολαμβανω,
        AccentClass::Ἀναγω,
        AccentClass::Ζωννῡμῐ,
        AccentClass::Ὑπερεχω,
        AccentClass::Ἀρνυμαι,
        AccentClass::Ἀειρω,
        AccentClass::Προσκειμαι,
        AccentClass::Ἐπευχομαι,
        AccentClass::Νεομαι,
        AccentClass::Λᾰμβᾰνω,
        AccentClass::Ἐπορον,
        AccentClass::Προεῑπον,
        AccentClass::Προσᾰγω,
        AccentClass::Ὑποφερω,
        AccentClass::Ὀττευομαι,
        AccentClass::Ἐμπηγνῡμῐ,
        AccentClass::Ἐπορνῡμῐ,
        AccentClass::Δημοκρᾰτεομαι,
        AccentClass::Δᾰτεομαι,
        AccentClass::Συνεπομαι,
        AccentClass::Ἐφεπω,
        AccentClass::Ἀντιλεγω,
        AccentClass::Δῐᾰφερω,
        AccentClass::Ἐδω,
        AccentClass::Περνημῐ,
        AccentClass::Χρη,
        AccentClass::Εἰμῐ,
        AccentClass::Ἐοικᾰ,
        AccentClass::Δρᾱω,
        AccentClass::Ἀραρισκω,
        AccentClass::Ζω,
        AccentClass::Φημῐ,
        AccentClass::Σκεδᾰννῡμῐ,
        AccentClass::Ἐπιμιμνησκομαι,
        AccentClass::Περθω,
        AccentClass::Νεμω,
        AccentClass::Στορνῡμῐ,
        AccentClass::Ἐπεφνον,
        AccentClass::Ἀνιημῐ,
        AccentClass::Μεθῠσκω,
        AccentClass::Ὠνεομαι,
        AccentClass::Ἀειδω,
        AccentClass::Γῐγνωσκω,
        AccentClass::Δοκεω,
        AccentClass::Κᾰθιημῐ,
    ];
}

impl flexion::Label for AccentClass {
    fn index(self) -> usize {
        self as usize
    }
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

/// The classes' patterns and stem syllables.
pub static CLASSES: [StressClass<'static>; 67] = [
    StressClass {
        name: Cow::Borrowed("ἐχω"),
        pattern: 0, // ἐχω
        vowel: Some(1),
        words: 1023,
    },
    StressClass {
        name: Cow::Borrowed("ποιεω"),
        pattern: 1, // ποιεω
        vowel: Some(1),
        words: 245,
    },
    StressClass {
        name: Cow::Borrowed("οἰομαι"),
        pattern: 0, // ἐχω
        vowel: Some(2),
        words: 161,
    },
    StressClass {
        name: Cow::Borrowed("ἀναιρεω"),
        pattern: 2, // ἀναιρεω
        vowel: Some(1),
        words: 141,
    },
    StressClass {
        name: Cow::Borrowed("καθιζω"),
        pattern: 3, // καθιζω
        vowel: Some(1),
        words: 140,
    },
    StressClass {
        name: Cow::Borrowed("ἀνιστημι"),
        pattern: 5, // ἀνιστημι
        vowel: Some(2),
        words: 34,
    },
    StressClass {
        name: Cow::Borrowed("προβαλλω"),
        pattern: 4, // προβαλλω
        vowel: Some(1),
        words: 33,
    },
    StressClass {
        name: Cow::Borrowed("ὀφειλω"),
        pattern: 7, // ὀφειλω
        vowel: Some(1),
        words: 22,
    },
    StressClass {
        name: Cow::Borrowed("κᾰτεργαζομαι"),
        pattern: 2, // ἀναιρεω
        vowel: Some(2),
        words: 21,
    },
    StressClass {
        name: Cow::Borrowed("ἐπῐλᾰμβᾰνω"),
        pattern: 8, // ἐπῐλᾰμβᾰνω
        vowel: Some(1),
        words: 19,
    },
    StressClass {
        name: Cow::Borrowed("τῐθημῐ"),
        pattern: 10, // τῐθημῐ
        vowel: Some(2),
        words: 17,
    },
    StressClass {
        name: Cow::Borrowed("ὑπαρχω"),
        pattern: 9, // ὑπαρχω
        vowel: Some(1),
        words: 16,
    },
    StressClass {
        name: Cow::Borrowed("λογῐζομαι"),
        pattern: 3, // καθιζω
        vowel: Some(2),
        words: 15,
    },
    StressClass {
        name: Cow::Borrowed("περιειμι"),
        pattern: 1, // ποιεω
        vowel: Some(2),
        words: 14,
    },
    StressClass {
        name: Cow::Borrowed("δεῑ"),
        pattern: 0, // ἐχω
        vowel: Some(0),
        words: 11,
    },
    StressClass {
        name: Cow::Borrowed("ἀφῑημῐ"),
        pattern: 6, // ἀφῑημῐ
        vowel: Some(2),
        words: 11,
    },
    StressClass {
        name: Cow::Borrowed("πᾰρερχομαι"),
        pattern: 6, // ἀφῑημῐ
        vowel: Some(3),
        words: 11,
    },
    StressClass {
        name: Cow::Borrowed("ἐκλυτροω"),
        pattern: 11, // ἐκλυτροω
        vowel: Some(1),
        words: 11,
    },
    StressClass {
        name: Cow::Borrowed("τελεω"),
        pattern: 13, // τελεω
        vowel: Some(1),
        words: 10,
    },
    StressClass {
        name: Cow::Borrowed("ἡγεομαι"),
        pattern: 12, // ἡγεομαι
        vowel: Some(3),
        words: 9,
    },
    StressClass {
        name: Cow::Borrowed("προσευχομαι"),
        pattern: 14, // προσευχομαι
        vowel: None,
        words: 8,
    },
    StressClass {
        name: Cow::Borrowed("κᾰλεω"),
        pattern: 15, // κᾰλεω
        vowel: Some(1),
        words: 7,
    },
    StressClass {
        name: Cow::Borrowed("ὑπολαμβανω"),
        pattern: 17, // ὑπολαμβανω
        vowel: Some(1),
        words: 7,
    },
    StressClass {
        name: Cow::Borrowed("ἀναγω"),
        pattern: 19, // ἀναγω
        vowel: Some(1),
        words: 7,
    },
    StressClass {
        name: Cow::Borrowed("ζωννῡμῐ"),
        pattern: 20, // ζωννῡμῐ
        vowel: Some(2),
        words: 7,
    },
    StressClass {
        name: Cow::Borrowed("ὑπερεχω"),
        pattern: 16, // ὑπερεχω
        vowel: Some(1),
        words: 6,
    },
    StressClass {
        name: Cow::Borrowed("ἀρνυμαι"),
        pattern: 18, // ἀρνυμαι
        vowel: Some(2),
        words: 6,
    },
    StressClass {
        name: Cow::Borrowed("ἀειρω"),
        pattern: 21, // ἀειρω
        vowel: Some(1),
        words: 6,
    },
    StressClass {
        name: Cow::Borrowed("προσκειμαι"),
        pattern: 22, // προσκειμαι
        vowel: Some(2),
        words: 6,
    },
    StressClass {
        name: Cow::Borrowed("ἐπευχομαι"),
        pattern: 0, // ἐχω
        vowel: Some(3),
        words: 5,
    },
    StressClass {
        name: Cow::Borrowed("νεομαι"),
        pattern: 12, // ἡγεομαι
        vowel: None,
        words: 4,
    },
    StressClass {
        name: Cow::Borrowed("λᾰμβᾰνω"),
        pattern: 23, // λᾰμβᾰνω
        vowel: Some(1),
        words: 4,
    },
    StressClass {
        name: Cow::Borrowed("ἐπορον"),
        pattern: 4, // προβαλλω
        vowel: Some(2),
        words: 3,
    },
    StressClass {
        name: Cow::Borrowed("προεῑπον"),
        pattern: 6, // ἀφῑημῐ
        vowel: None,
        words: 3,
    },
    StressClass {
        name: Cow::Borrowed("προσᾰγω"),
        pattern: 24, // προσᾰγω
        vowel: Some(1),
        words: 3,
    },
    StressClass {
        name: Cow::Borrowed("ὑποφερω"),
        pattern: 25, // ὑποφερω
        vowel: Some(1),
        words: 3,
    },
    StressClass {
        name: Cow::Borrowed("ὀττευομαι"),
        pattern: 0, // ἐχω
        vowel: None,
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("ἐμπηγνῡμῐ"),
        pattern: 7, // ὀφειλω
        vowel: Some(2),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("ἐπορνῡμῐ"),
        pattern: 9, // ὑπαρχω
        vowel: Some(2),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("δημοκρᾰτεομαι"),
        pattern: 11, // ἐκλυτροω
        vowel: Some(2),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("δᾰτεομαι"),
        pattern: 14, // προσευχομαι
        vowel: Some(3),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("συνεπομαι"),
        pattern: 16, // ὑπερεχω
        vowel: Some(2),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("ἐφεπω"),
        pattern: 28, // ἐφεπω
        vowel: Some(1),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("ἀντιλεγω"),
        pattern: 29, // ἀντιλεγω
        vowel: Some(1),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("δῐᾰφερω"),
        pattern: 31, // δῐᾰφερω
        vowel: Some(1),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("ἐδω"),
        pattern: 2, // ἀναιρεω
        vowel: None,
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("περνημῐ"),
        pattern: 3, // καθιζω
        vowel: None,
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("χρη"),
        pattern: 5, // ἀνιστημι
        vowel: None,
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("εἰμῐ"),
        pattern: 5, // ἀνιστημι
        vowel: Some(0),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἐοικᾰ"),
        pattern: 8, // ἐπῐλᾰμβᾰνω
        vowel: None,
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("δρᾱω"),
        pattern: 8, // ἐπῐλᾰμβᾰνω
        vowel: Some(0),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἀραρισκω"),
        pattern: 8, // ἐπῐλᾰμβᾰνω
        vowel: Some(2),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ζω"),
        pattern: 11, // ἐκλυτροω
        vowel: Some(0),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("φημῐ"),
        pattern: 12, // ἡγεομαι
        vowel: Some(0),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("σκεδᾰννῡμῐ"),
        pattern: 13, // τελεω
        vowel: Some(2),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἐπιμιμνησκομαι"),
        pattern: 15, // κᾰλεω
        vowel: Some(2),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("περθω"),
        pattern: 18, // ἀρνυμαι
        vowel: Some(1),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("νεμω"),
        pattern: 26, // νεμω
        vowel: Some(1),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("στορνῡμῐ"),
        pattern: 26, // νεμω
        vowel: Some(2),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἐπεφνον"),
        pattern: 27, // ἀνιημῐ
        vowel: Some(2),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἀνιημῐ"),
        pattern: 27, // ἀνιημῐ
        vowel: Some(3),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("μεθῠσκω"),
        pattern: 30, // ὠνεομαι
        vowel: Some(1),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ὠνεομαι"),
        pattern: 30, // ὠνεομαι
        vowel: Some(2),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἀειδω"),
        pattern: 32, // ἀειδω
        vowel: Some(1),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("γῐγνωσκω"),
        pattern: 33, // γῐγνωσκω
        vowel: Some(1),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("δοκεω"),
        pattern: 34, // δοκεω
        vowel: Some(1),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("κᾰθιημῐ"),
        pattern: 35, // κᾰθιημῐ
        vowel: Some(2),
        words: 1,
    },
];

use AccentClass::*;

/// Which class a word is of: 186 rules and 1,764 listed words.
pub const CLASS: Choice<'static, AccentClass> = flexion::choice! {
    rules {
        "" => [Ἐχω 1023, Ποιεω 245, Οἰομαι 161, Ἀναιρεω 141, Καθιζω 140, Ἀνιστημι 34, Προβαλλω 33, Ὀφειλω 22, Κᾰτεργαζομαι 21, Ἐπῐλᾰμβᾰνω 19, Τῐθημῐ 17, Ὑπαρχω 16, Λογῐζομαι 15, Περιειμι 14, Δεῑ 11, Ἀφῑημῐ 11, Πᾰρερχομαι 11, Ἐκλυτροω 11, Τελεω 10, Ἡγεομαι 9, Προσευχομαι 8, Κᾰλεω 7, Ὑπολαμβανω 7, Ἀναγω 7, Ζωννῡμῐ 7, Ὑπερεχω 6, Ἀρνυμαι 6, Ἀειρω 6, Προσκειμαι 6, Ἐπευχομαι 5, Νεομαι 4, Λᾰμβᾰνω 4, Ἐπορον 3, Προεῑπον 3, Προσᾰγω 3, Ὑποφερω 3, Ὀττευομαι 2, Ἐμπηγνῡμῐ 2, Ἐπορνῡμῐ 2, Δημοκρᾰτεομαι 2, Δᾰτεομαι 2, Συνεπομαι 2, Ἐφεπω 2, Ἀντιλεγω 2, Δῐᾰφερω 2, Ἐδω 1, Περνημῐ 1, Χρη 1, Εἰμῐ 1, Ἐοικᾰ 1, Δρᾱω 1, Ἀραρισκω 1, Ζω 1, Φημῐ 1, Σκεδᾰννῡμῐ 1, Ἐπιμιμνησκομαι 1, Περθω 1, Νεμω 1, Στορνῡμῐ 1, Ἐπεφνον 1, Ἀνιημῐ 1, Μεθῠσκω 1, Ὠνεομαι 1, Ἀειδω 1, Γῐγνωσκω 1, Δοκεω 1, Κᾰθιημῐ 1],
        "δα" => [Ἐχω 2, Οἰομαι 2],
        "ε" => [Οἰομαι 5],
        "τε" => [Οἰομαι 4],
        "ι" => [Οἰομαι 132, Ἀνιστημι 34, Κᾰτεργαζομαι 21, Τῐθημῐ 17, Λογῐζομαι 15, Περιειμι 13, Πᾰρερχομαι 11, Ἡγεομαι 9, Ἐχω 8, Ἀφῑημῐ 7, Προσευχομαι 7, Ζωννῡμῐ 7, Ἀρνυμαι 6, Προσκειμαι 6, Νεομαι 4, Ἐπευχομαι 3, Δεῑ 2, Ὀττευομαι 2, Ἐπορνῡμῐ 2, Δημοκρᾰτεομαι 2, Δᾰτεομαι 2, Συνεπομαι 2, Ποιεω 1, Ἀναιρεω 1, Ἐπορον 1, Ἐμπηγνῡμῐ 1, Περνημῐ 1, Εἰμῐ 1, Φημῐ 1, Σκεδᾰννῡμῐ 1, Ἐπιμιμνησκομαι 1, Στορνῡμῐ 1, Ἀνιημῐ 1, Ὠνεομαι 1, Κᾰθιημῐ 1],
        "αι" => [Οἰομαι 110, Κᾰτεργαζομαι 21, Λογῐζομαι 11, Πᾰρερχομαι 11, Ἡγεομαι 9, Περιειμι 8, Προσευχομαι 7, Ἐχω 6, Ἀνιστημι 6, Ἀρνυμαι 6, Προσκειμαι 6, Νεομαι 4, Ἐπευχομαι 3, Ὀττευομαι 2, Δημοκρᾰτεομαι 2, Δᾰτεομαι 2, Συνεπομαι 2, Ἀναιρεω 1, Τῐθημῐ 1, Ἐπορον 1, Ἐπιμιμνησκομαι 1, Ὠνεομαι 1],
        "αμαι" => [Ἀνιστημι 6, Οἰομαι 1],
        "ιμαι" => [Προσκειμαι 6, Ἐχω 1, Οἰομαι 1],
        "γκειμαι" => [Προσκειμαι 3],
        "αομαι" => [Οἰομαι 9, Κᾰτεργαζομαι 6, Περιειμι 2, Νεομαι 1, Δᾰτεομαι 1],
        "ναομαι" => [Περιειμι 2, Νεομαι 1],
        "βομαι" => [Οἰομαι 3],
        "δομαι" => [Οἰομαι 10],
        "εομαι" => [Ἡγεομαι 9, Οἰομαι 4, Κᾰτεργαζομαι 4, Περιειμι 3, Προσευχομαι 3, Δημοκρᾰτεομαι 2, Νεομαι 1, Δᾰτεομαι 1, Ὠνεομαι 1],
        "κνεομαι" => [Ἡγεομαι 3],
        "ρνεομαι" => [Κᾰτεργαζομαι 2],
        "ατεομαι" => [Δημοκρᾰτεομαι 2, Ἡγεομαι 1, Δᾰτεομαι 1],
        "ρατεομαι" => [Δημοκρᾰτεομαι 2],
        "ζομαι" => [Οἰομαι 15, Λογῐζομαι 6, Κᾰτεργαζομαι 5, Ἀρνυμαι 2],
        "αζομαι" => [Οἰομαι 9, Κᾰτεργαζομαι 5],
        "γαζομαι" => [Κᾰτεργαζομαι 5],
        "ργαζομαι" => [Κᾰτεργαζομαι 5],
        "εζομαι" => [Ἀρνυμαι 2],
        "ιζομαι" => [Λογῐζομαι 6, Οἰομαι 4],
        "νιζομαι" => [Λογῐζομαι 3],
        "θομαι" => [Οἰομαι 3],
        "λομαι" => [Οἰομαι 7],
        "νομαι" => [Οἰομαι 9, Κᾰτεργαζομαι 3, Ἐχω 2, Λογῐζομαι 2],
        "ανομαι" => [Κᾰτεργαζομαι 3, Λογῐζομαι 1],
        "γνομαι" => [Ἐχω 2, Οἰομαι 1],
        "ινομαι" => [Οἰομαι 7, Λογῐζομαι 1],
        "οομαι" => [Οἰομαι 5, Περιειμι 3, Προσευχομαι 3, Κᾰτεργαζομαι 1],
        "πομαι" => [Συνεπομαι 2, Οἰομαι 1],
        "ρομαι" => [Οἰομαι 5, Ἀρνυμαι 3, Λογῐζομαι 2],
        "σομαι" => [Οἰομαι 8, Ἀναιρεω 1],
        "υομαι" => [Οἰομαι 7, Ὀττευομαι 2],
        "χομαι" => [Πᾰρερχομαι 11, Οἰομαι 8, Ἐπευχομαι 3, Κᾰτεργαζομαι 1, Λογῐζομαι 1, Προσευχομαι 1, Νεομαι 1],
        "πευχομαι" => [Ἐπευχομαι 2],
        "υμαι" => [Ἐχω 2, Ἀρνυμαι 1, Νεομαι 1, Ἐπορον 1],
        "μι" => [Ἀνιστημι 27, Οἰομαι 20, Τῐθημῐ 16, Ἀφῑημῐ 7, Ζωννῡμῐ 7, Περιειμι 5, Λογῐζομαι 4, Ἐπορνῡμῐ 2, Ποιεω 1, Δεῑ 1, Ἐμπηγνῡμῐ 1, Περνημῐ 1, Εἰμῐ 1, Φημῐ 1, Σκεδᾰννῡμῐ 1, Στορνῡμῐ 1, Ἀνιημῐ 1, Κᾰθιημῐ 1],
        "ημι" => [Ἀνιστημι 24, Τῐθημῐ 9, Ἀφῑημῐ 7, Οἰομαι 3, Ποιεω 1, Δεῑ 1, Περνημῐ 1, Φημῐ 1, Ἀνιημῐ 1, Κᾰθιημῐ 1],
        "θημι" => [Τῐθημῐ 9],
        "ιθημι" => [Τῐθημῐ 9],
        "ιημι" => [Ἀφῑημῐ 7, Οἰομαι 2, Ἀνιημῐ 1, Κᾰθιημῐ 1],
        "τημι" => [Ἀνιστημι 20],
        "στημι" => [Ἀνιστημι 20],
        "ιμι" => [Τῐθημῐ 5, Οἰομαι 4, Περιειμι 4, Εἰμῐ 1],
        "ξειμι" => [Περιειμι 2],
        "υμι" => [Οἰομαι 10, Ζωννῡμῐ 7, Λογῐζομαι 4, Ἀνιστημι 3, Τῐθημῐ 2, Ἐπορνῡμῐ 2, Περιειμι 1, Ἐμπηγνῡμῐ 1, Σκεδᾰννῡμῐ 1, Στορνῡμῐ 1],
        "ιγνυμι" => [Ἀνιστημι 3, Οἰομαι 1, Τῐθημῐ 1],
        "μνυμι" => [Λογῐζομαι 3],
        "ομνυμι" => [Λογῐζομαι 3],
        "εννυμι" => [Οἰομαι 3],
        "ωννυμι" => [Ζωννῡμῐ 4, Ἐπορνῡμῐ 1],
        "ρωννυμι" => [Ζωννῡμῐ 3],
        "ωμι" => [Οἰομαι 3],
        "οι" => [Ἐχω 2],
        "ν" => [Οἰομαι 16, Προβαλλω 6, Ἀφῑημῐ 4, Προεῑπον 3, Ἐπορον 2, Ἐχω 1, Ἐπεφνον 1],
        "εν" => [Οἰομαι 4],
        "δον" => [Προβαλλω 2],
        "πον" => [Ἀφῑημῐ 4, Προεῑπον 3, Προβαλλω 1],
        "πειπον" => [Ἀφῑημῐ 3],
        "τον" => [Οἰομαι 4],
        "ων" => [Οἰομαι 3],
        "ω" => [Ἐχω 1007, Ποιεω 244, Ἀναιρεω 140, Καθιζω 140, Προβαλλω 27, Ὀφειλω 22, Ἐπῐλᾰμβᾰνω 19, Ὑπαρχω 16, Ἐκλυτροω 11, Τελεω 10, Δεῑ 9, Κᾰλεω 7, Ὑπολαμβανω 7, Ἀναγω 7, Ὑπερεχω 6, Ἀειρω 6, Λᾰμβᾰνω 4, Οἰομαι 3, Προσᾰγω 3, Ὑποφερω 3, Ἐφεπω 2, Ἀντιλεγω 2, Δῐᾰφερω 2, Περιειμι 1, Προσευχομαι 1, Ἐμπηγνῡμῐ 1, Ἐδω 1, Δρᾱω 1, Ἀραρισκω 1, Ζω 1, Περθω 1, Νεμω 1, Μεθῠσκω 1, Ἀειδω 1, Γῐγνωσκω 1, Δοκεω 1],
        "αω" => [Ἐχω 58, Ποιεω 50, Ἀναιρεω 9, Ὑπερεχω 4, Ἐκλυτροω 3, Ἐπῐλᾰμβᾰνω 2, Δεῑ 1, Λᾰμβᾰνω 1, Δρᾱω 1],
        "καω" => [Ποιεω 4, Ἐχω 1],
        "λαω" => [Ἐχω 8, Ποιεω 2],
        "ρμαω" => [Ἀναιρεω 2],
        "ραω" => [Ἐχω 7, Ποιεω 4, Ὑπερεχω 4, Ἐπῐλᾰμβᾰνω 2, Λᾰμβᾰνω 1, Δρᾱω 1],
        "εραω" => [Ἐχω 4, Ποιεω 1],
        "χαω" => [Ποιεω 3],
        "ψαω" => [Ἐκλυτροω 2],
        "βω" => [Ἐχω 11],
        "γω" => [Ἐχω 28, Ἀναιρεω 11, Ἀναγω 3, Προβαλλω 2, Καθιζω 1, Ὑπαρχω 1, Ὑπερεχω 1, Προσᾰγω 1, Ἀντιλεγω 1],
        "ναγω" => [Ἀναιρεω 2, Ἀναγω 2],
        "αναγω" => [Ἀναγω 2],
        "αλεγω" => [Ἀναιρεω 3, Ἐχω 1],
        "ιγω" => [Ἐχω 5],
        "δω" => [Ἐχω 11, Ἀναιρεω 3, Ὀφειλω 3, Ἐπῐλᾰμβᾰνω 1, Ἐδω 1, Ἀειδω 1],
        "εω" => [Ἐχω 177, Ποιεω 147, Ἀναιρεω 34, Καθιζω 8, Ἐκλυτροω 6, Ἐπῐλᾰμβᾰνω 4, Τελεω 4, Ἀειρω 3, Κᾰλεω 1, Ἐμπηγνῡμῐ 1, Ἀντιλεγω 1, Δοκεω 1],
        "γεω" => [Ποιεω 26, Ἐχω 10, Ἀναιρεω 3, Ἐκλυτροω 1],
        "αγεω" => [Ποιεω 5],
        "υθεω" => [Ἐχω 3],
        "ωθεω" => [Ἐπῐλᾰμβᾰνω 2, Ἀναιρεω 1],
        "ικεω" => [Ἐχω 10, Ἀναιρεω 2],
        "ρκεω" => [Ἀναιρεω 2, Ποιεω 1],
        "αλεω" => [Ἀναιρεω 5, Καθιζω 3, Ἐχω 2, Ποιεω 1, Κᾰλεω 1, Ἀειρω 1],
        "μελεω" => [Ἐχω 3],
        "λεμεω" => [Ποιεω 3],
        "δομεω" => [Ἐχω 6, Ποιεω 1],
        "οδομεω" => [Ἐχω 6],
        "ρομεω" => [Ποιεω 3],
        "ενεω" => [Ἐχω 4],
        "αινεω" => [Ἀναιρεω 2, Ἐχω 1],
        "ονεω" => [Ποιεω 12, Ἐχω 6, Ἀναιρεω 1],
        "πεω" => [Ποιεω 4, Ἐχω 3, Ἐπῐλᾰμβᾰνω 2, Ἐκλυτροω 1],
        "ιρεω" => [Ἀναιρεω 7, Ἀειρω 2, Ἐχω 1, Τελεω 1],
        "ναιρεω" => [Ἀειρω 2, Ἀναιρεω 1],
        "ροφορεω" => [Ποιεω 3],
        "ωρεω" => [Ποιεω 6, Ἐχω 4],
        "αχωρεω" => [Ἐχω 3],
        "τεω" => [Ἐχω 22, Ποιεω 11, Ἀναιρεω 4],
        "ιτεω" => [Ἀναιρεω 3, Ἐχω 1],
        "φεω" => [Ἐχω 6, Ποιεω 5, Ἐκλυτροω 2],
        "υμαχεω" => [Ποιεω 3],
        "ζω" => [Ἐχω 143, Καθιζω 100, Ἀναιρεω 27, Τελεω 4, Ὀφειλω 2, Δεῑ 2, Ἀειρω 2, Ζω 1],
        "αζω" => [Ἐχω 77, Ἀναιρεω 16, Τελεω 4, Ἀειρω 2, Ὀφειλω 1, Δεῑ 1],
        "ιαζω" => [Ἐχω 18, Ἀναιρεω 1],
        "γκαζω" => [Ἀναιρεω 2],
        "ομαζω" => [Ἀναιρεω 2],
        "παζω" => [Ἀναιρεω 4, Ἐχω 3],
        "σταζω" => [Ἐχω 5],
        "ιζω" => [Καθιζω 99, Ἐχω 49, Ἀναιρεω 10],
        "αιζω" => [Ἐχω 6, Καθιζω 1],
        "κιζω" => [Καθιζω 18, Ἐχω 2],
        "ριζω" => [Καθιζω 18, Ἐχω 5, Ἀναιρεω 5],
        "εριζω" => [Καθιζω 5],
        "υζω" => [Ἐχω 9],
        "ωζω" => [Ἐχω 5],
        "υθω" => [Ἐχω 5],
        "ιω" => [Ἐχω 29, Καθιζω 1, Δεῑ 1, Τελεω 1],
        "κω" => [Ἐχω 36, Ἐπῐλᾰμβᾰνω 7, Ἀναιρεω 3, Προβαλλω 3, Ὀφειλω 3, Ὑπαρχω 3, Δεῑ 1, Ἀναγω 1, Λᾰμβᾰνω 1, Ἀραρισκω 1, Μεθῠσκω 1, Γῐγνωσκω 1],
        "εκω" => [Ἐχω 4],
        "ηκω" => [Ἐχω 6, Ὑπαρχω 3],
        "λκω" => [Ὀφειλω 2, Ἐχω 1, Ἀναγω 1],
        "φελκω" => [Ὀφειλω 2],
        "ησκω" => [Ἐπῐλᾰμβᾰνω 3, Ἐχω 1, Προβαλλω 1],
        "υρισκω" => [Ἀναιρεω 2, Λᾰμβᾰνω 1],
        "ρωσκω" => [Ἐπῐλᾰμβᾰνω 2, Προβαλλω 1],
        "λω" => [Ἐχω 18, Καθιζω 7, Ὀφειλω 7, Προβαλλω 3, Ἀναιρεω 2, Κᾰλεω 1],
        "ελω" => [Ὀφειλω 3, Ἐχω 1],
        "ιλω" => [Ὀφειλω 3, Ἐχω 1],
        "φειλω" => [Ὀφειλω 3],
        "αλλω" => [Ἐχω 8, Καθιζω 4, Προβαλλω 3],
        "βαλλω" => [Προβαλλω 3, Καθιζω 1],
        "γελλω" => [Καθιζω 3, Ἀναιρεω 2],
        "ιλλω" => [Ἐχω 3],
        "νω" => [Ἐχω 99, Καθιζω 17, Ὑπολαμβανω 7, Ἀναιρεω 6, Προβαλλω 6, Κᾰλεω 5, Ἐπῐλᾰμβᾰνω 2, Δεῑ 1, Τελεω 1, Λᾰμβᾰνω 1, Δῐᾰφερω 1],
        "ανω" => [Ἐχω 20, Ὑπολαμβανω 7, Κᾰλεω 5, Ἀναιρεω 2, Ἐπῐλᾰμβᾰνω 2, Λᾰμβᾰνω 1, Δῐᾰφερω 1],
        "βανω" => [Ὑπολαμβανω 6, Κᾰλεω 3, Ἐχω 1, Ἐπῐλᾰμβᾰνω 1, Λᾰμβᾰνω 1],
        "νδανω" => [Ἐχω 3],
        "στανω" => [Ἐχω 3],
        "μενω" => [Καθιζω 2, Ἐχω 1],
        "ινω" => [Ἐχω 49, Ἀναιρεω 3, Καθιζω 3, Προβαλλω 3, Δεῑ 1],
        "μαινω" => [Ἐχω 9],
        "πινω" => [Προβαλλω 2],
        "μνω" => [Προβαλλω 2, Ἐχω 1],
        "υνω" => [Ἐχω 27, Καθιζω 12, Ἀναιρεω 1, Τελεω 1],
        "λυνω" => [Ἐχω 5],
        "τρυνω" => [Ἐχω 4],
        "ατυνω" => [Καθιζω 2],
        "ξω" => [Ὀφειλω 2, Ἐχω 1],
        "οω" => [Ποιεω 47, Ἐχω 41, Ἀναιρεω 8, Ἐκλυτροω 2, Οἰομαι 1, Δεῑ 1, Προσευχομαι 1],
        "αιοω" => [Ποιεω 3],
        "ξιοω" => [Ἀναιρεω 2],
        "μοω" => [Ποιεω 5, Ἐχω 1],
        "υροω" => [Ἐχω 4, Ποιεω 1],
        "τοω" => [Ποιεω 5, Ἐχω 1],
        "πω" => [Ἐχω 21, Προβαλλω 3, Ἀναγω 1, Ἐφεπω 1],
        "ιπω" => [Προβαλλω 3, Ἐχω 1],
        "λειπω" => [Προβαλλω 3],
        "ρω" => [Ἐχω 31, Ἀναιρεω 4, Ὑποφερω 3, Καθιζω 2, Ὀφειλω 2, Προσᾰγω 2, Προβαλλω 1, Ἐπῐλᾰμβᾰνω 1, Ὑπαρχω 1, Ἀειρω 1, Δῐᾰφερω 1],
        "ταιρω" => [Προσᾰγω 2],
        "γειρω" => [Ἐχω 4],
        "σω" => [Ἐχω 50, Ἀναιρεω 6, Προβαλλω 2, Ἐπῐλᾰμβᾰνω 2, Ὀφειλω 1],
        "λασσω" => [Ἀναιρεω 5, Ἐχω 4],
        "λλασσω" => [Ἀναιρεω 5],
        "ισσω" => [Ἐχω 10],
        "τω" => [Ἐχω 56, Ἀναιρεω 4, Προβαλλω 2, Οἰομαι 1, Καθιζω 1, Ὑπαρχω 1],
        "ναπτω" => [Ἐχω 2, Ἀναιρεω 2],
        "ττω" => [Ἐχω 17],
        "υω" => [Ἐχω 142, Ἀναιρεω 17, Προβαλλω 1, Ὀφειλω 1, Περιειμι 1, Λᾰμβᾰνω 1],
        "λευω" => [Ἐχω 12],
        "ανυω" => [Ἀναιρεω 2, Ἐχω 1],
        "ουω" => [Ἐχω 9],
        "ρυω" => [Ἐχω 4, Ἀναιρεω 3],
        "ρτυω" => [Ἀναιρεω 2],
        "φω" => [Ἐχω 14, Ἀναιρεω 3],
        "ρεφω" => [Ἀναιρεω 3, Ἐχω 2],
        "χω" => [Ἐχω 17, Ὑπαρχω 10, Ἀναιρεω 3, Καθιζω 3, Προβαλλω 3, Ἀναγω 2, Ὑπερεχω 1, Ἐφεπω 1],
        "γχω" => [Ἐχω 3],
        "πεχω" => [Ὑπαρχω 2],
        "τρεχω" => [Καθιζω 3, Ἐχω 1, Προβαλλω 1],
        "τεχω" => [Ὑπαρχω 3],
        "αρχω" => [Ἀναιρεω 2, Ὑπαρχω 2],
    }
    listed {
        "ααω" => Ἐχω,
        "αγαθοποιεω" => Ἀναιρεω,
        "αγαλλιαω" => Ἐχω,
        "αγαλλομαι" => Οἰομαι,
        "αγαμαι" => Ἀνιστημι,
        "αγανακτεω" => Ἐχω,
        "αγαπαω" => Ἐχω,
        "αγγελλω" => Ἀναιρεω,
        "αγειρω" => Ἐχω,
        "αγιζω" => Ἐχω,
        "αγκυλοω" => Ἐχω,
        "αγνευω" => Ἀναιρεω,
        "αγνιζω" => Ἀναιρεω,
        "αγνοεω" => Ἐχω,
        "αγοραζω" => Ἀειρω,
        "αγορανομεω" => Ἐχω,
        "αγορευω" => Ἀναιρεω,
        "αγραυλεω" => Ἐχω,
        "αγρευω" => Ἀναιρεω,
        "αγρεω" => Ἐχω,
        "αγρυπνεω" => Ἐχω,
        "αγχονιζω" => Ἐχω,
        "αγχω" => Ἐχω,
        "αγω" => Ἀναιρεω,
        "αγωνιζομαι" => Λογῐζομαι,
        "αδελφοκτονεω" => Ἐχω,
        "αδηλοω" => Ἐχω,
        "αδημονεω" => Ἐχω,
        "αδησα" => Ἐχω,
        "αδικεω" => Ἀναιρεω,
        "αδω" => Ἀναιρεω,
        "αειδω" => Ἀειδω,
        "αειρω" => Ἀειρω,
        "αεξω" => Ὀφειλω,
        "αζομαι" => Οἰομαι,
        "αθανατιζω" => Ἐχω,
        "αθετεω" => Ἐχω,
        "αθλεω" => Ἐχω,
        "αθροιζω" => Καθιζω,
        "αθυμεω" => Ἐχω,
        "αθυρω" => Ἐχω,
        "αιδεομαι" => Κᾰτεργαζομαι,
        "αιδομαι" => Οἰομαι,
        "αιθω" => Ἐχω,
        "αικιζω" => Καθιζω,
        "αιρετιζω" => Καθιζω,
        "αιρεω" => Ἀναιρεω,
        "αιρω" => Ἀναιρεω,
        "αισθανομαι" => Κᾰτεργαζομαι,
        "αισσω" => Ἐχω,
        "αισχοω" => Ἐχω,
        "αισχρουργεω" => Ἐχω,
        "αισχυνω" => Ἐχω,
        "αιτεω" => Ἀναιρεω,
        "αιτιαομαι" => Οἰομαι,
        "αιτιζω" => Ἐχω,
        "αιχμαζω" => Ἀναιρεω,
        "αιχμαλοω" => Ἐχω,
        "αιχμαλωτιζω" => Καθιζω,
        "ακηδεω" => Ἐχω,
        "ακμαζω" => Ἐχω,
        "ακολουθεω" => Ἐχω,
        "ακοναω" => Ἐχω,
        "ακοντιζω" => Ἐχω,
        "ακουω" => Ἐχω,
        "ακριβοω" => Ἐχω,
        "ακροαομαι" => Κᾰτεργαζομαι,
        "αλαομαι" => Κᾰτεργαζομαι,
        "αλγεω" => Ἐχω,
        "αλεγω" => Ἐχω,
        "αλειφω" => Ἐχω,
        "αλεξω" => Ἐχω,
        "αλεομαι" => Προσευχομαι,
        "αλεω" => Ἐχω,
        "αλθαινω" => Ἐχω,
        "αλθεω" => Ἐχω,
        "αλθησκω" => Ἐχω,
        "αλιευω" => Ἀναιρεω,
        "αλιζω" => Ἐχω,
        "αλισκομαι" => Οἰομαι,
        "αλλασσω" => Ἀναιρεω,
        "αλλαττω" => Ἐχω,
        "αλλοιοω" => Ἐχω,
        "αλλομαι" => Οἰομαι,
        "αλλοτριοω" => Ἐχω,
        "αλυω" => Ἐχω,
        "αλφανω" => Ἐχω,
        "αμαλδυνω" => Ἐχω,
        "αμαρτανω" => Ἀναιρεω,
        "αμαρυσσω" => Ἐχω,
        "αμαυροω" => Ἐχω,
        "αμαω" => Ἐχω,
        "αμβλυνω" => Ἐχω,
        "αμειβω" => Ἐχω,
        "αμελγω" => Ἐχω,
        "αμελεω" => Ἐχω,
        "αμερδω" => Ἐπῐλᾰμβᾰνω,
        "αμηχανοποιεομαι" => Προσευχομαι,
        "αμυνω" => Καθιζω,
        "αμυσσω" => Ἐπῐλᾰμβᾰνω,
        "αμφικαλυπτω" => Ἐχω,
        "αμφικρυπτω" => Ἐχω,
        "αμφιστημι" => Ἀνιστημι,
        "αναβλαστανω" => Ἐχω,
        "αναβοαω" => Ἐχω,
        "αναγγελλω" => Καθιζω,
        "αναγιγνωσκω" => Ἐπῐλᾰμβᾰνω,
        "αναγκαζω" => Ἀναιρεω,
        "αναγνωριζω" => Ἐχω,
        "αναγω" => Ἀναγω,
        "αναδιδασκω" => Ἐχω,
        "αναειρω" => Ἐχω,
        "αναζω" => Δεῑ,
        "αναινομαι" => Οἰομαι,
        "αναιρεω" => Ἀναιρεω,
        "αναισσω" => Ἐχω,
        "ανακαλεω" => Ἀναιρεω,
        "ανακρυπτω" => Ἐχω,
        "αναλαμβανω" => Ὑπολαμβανω,
        "αναλισκω" => Ἀναιρεω,
        "αναμενω" => Καθιζω,
        "αναμιμνησκω" => Ἐπῐλᾰμβᾰνω,
        "αναποδιζω" => Ἐχω,
        "αναπτυσσω" => Ἀναιρεω,
        "αναπτω" => Ἀναιρεω,
        "ανασκαπτω" => Ἀναιρεω,
        "ανατασσω" => Ἐχω,
        "ανατρεχω" => Καθιζω,
        "αναφερω" => Ὑποφερω,
        "αναχωρεω" => Ἐχω,
        "αναψυχω" => Ἀναιρεω,
        "ανδανω" => Ἐχω,
        "ανδραποδιζω" => Ἐχω,
        "ανεγειρω" => Ἐχω,
        "ανειρομαι" => Ἀρνυμαι,
        "ανερχομαι" => Ἐπευχομαι,
        "ανεχω" => Ἀναγω,
        "ανηκουστεω" => Ἐχω,
        "ανηκω" => Ὑπαρχω,
        "ανθεω" => Ἐχω,
        "ανθιζω" => Ἐχω,
        "ανθιστημι" => Ἀνιστημι,
        "ανιημι" => Ἀνιημῐ,
        "ανιστημι" => Ἀνιστημι,
        "ανοιγω" => Ἐχω,
        "ανοικοδομεω" => Ἐχω,
        "ανορουω" => Ἐχω,
        "ανταγωνιζομαι" => Λογῐζομαι,
        "ανταιρω" => Προσᾰγω,
        "ανταναγω" => Ἀναγω,
        "ανταναλυω" => Ἐχω,
        "ανταποδιδωμι" => Οἰομαι,
        "ανταποκρινομαι" => Λογῐζομαι,
        "αντεγειρω" => Ἐχω,
        "αντελπιζω" => Ἀναιρεω,
        "αντεπιβουλευω" => Ἐχω,
        "αντεπιπλεω" => Ἐχω,
        "αντεραω" => Ἐχω,
        "αντεχω" => Ὑπαρχω,
        "αντιαζω" => Ἀναιρεω,
        "αντιαω" => Ἐχω,
        "αντικαθιστημι" => Ἀνιστημι,
        "αντιλεγω" => Ἀντιλεγω,
        "αντιπαρατασσομαι" => Οἰομαι,
        "ανυω" => Ἀναιρεω,
        "αξιοω" => Ἀναιρεω,
        "απαγγελλω" => Ἀναιρεω,
        "απαγχοναω" => Ἀναιρεω,
        "απαγχονεω" => Ἀναιρεω,
        "απαγχονιζω" => Καθιζω,
        "απαγχω" => Ἐχω,
        "απαγω" => Ἀναιρεω,
        "απαιτεω" => Ἀναιρεω,
        "απακριβοω" => Ἀναιρεω,
        "απαλθαινομαι" => Οἰομαι,
        "απαλλαξειω" => Ἐχω,
        "απαμειβομαι" => Οἰομαι,
        "απανιστημι" => Ἀνιστημι,
        "απανταω" => Ἐχω,
        "απαξιοω" => Ἀναιρεω,
        "απαρεσκω" => Ἐχω,
        "απαρνεομαι" => Κᾰτεργαζομαι,
        "απασχολεω" => Ἐχω,
        "απαταω" => Ἐχω,
        "απαυραω" => Ἐπῐλᾰμβᾰνω,
        "απειλεω" => Ἐχω,
        "απειμι" => Περιειμι,
        "απειπον" => Ἀφῑημῐ,
        "απελευθεροω" => Ἐχω,
        "απεραω" => Ἐχω,
        "απερχομαι" => Πᾰρερχομαι,
        "απευχομαι" => Ἐπευχομαι,
        "απεχθανομαι" => Λογῐζομαι,
        "απεχω" => Ὑπαρχω,
        "απιστεω" => Ἐχω,
        "απιχθυοομαι" => Προσευχομαι,
        "απλοω" => Ἀναιρεω,
        "αποδυω" => Ἐχω,
        "αποζω" => Δεῑ,
        "αποθραυω" => Ἐχω,
        "αποκαλυπτω" => Ἐχω,
        "αποκρυπτω" => Ἐχω,
        "αποκωλυω" => Ἐχω,
        "απολαμβανω" => Ὑπολαμβανω,
        "απολεγω" => Ὑπερεχω,
        "απολειπω" => Προβαλλω,
        "απολυτροω" => Ἐχω,
        "απονευροομαι" => Οἰομαι,
        "αποπερδομαι" => Οἰομαι,
        "απορεω" => Ἐχω,
        "αποστατεω" => Ἐχω,
        "απτω" => Ἀναιρεω,
        "απωθεω" => Ἐπῐλᾰμβᾰνω,
        "αραομαι" => Κᾰτεργαζομαι,
        "αραρισκω" => Ἀραρισκω,
        "αρασσω" => Ἐχω,
        "αργεω" => Ἐχω,
        "αρδω" => Ἐχω,
        "αρεσκω" => Ἐχω,
        "αρηγω" => Ἐχω,
        "αριθμεω" => Ἐχω,
        "αρκεω" => Ἀναιρεω,
        "αρμοζω" => Ἀναιρεω,
        "αρνεομαι" => Κᾰτεργαζομαι,
        "αρνυμαι" => Ἀρνυμαι,
        "αροω" => Ἐχω,
        "αρπαζω" => Ἀναιρεω,
        "αρρητουργεω" => Ἐχω,
        "αρταω" => Ἐχω,
        "αρτυω" => Ἀναιρεω,
        "αρχαιζω" => Καθιζω,
        "αρχω" => Ἀναιρεω,
        "ασαω" => Ἐχω,
        "ασθενεω" => Ἐχω,
        "ασκεω" => Ἐχω,
        "ασπαζομαι" => Οἰομαι,
        "ασπαιρω" => Ἐχω,
        "αστειευομαι" => Οἰομαι,
        "αστραπτω" => Ἐχω,
        "ασφαλιζω" => Ἐχω,
        "ασχολεω" => Ἐχω,
        "ασωτευω" => Ἀναιρεω,
        "αταλλω" => Ἐχω,
        "ατενιζω" => Ἐχω,
        "ατιμαζω" => Ἀναιρεω,
        "ατιταλλω" => Ἐχω,
        "αττικιζω" => Καθιζω,
        "ατυχεω" => Ἐχω,
        "αυδαω" => Ἀναιρεω,
        "αυθεντεω" => Ἐχω,
        "αυλεω" => Ἐχω,
        "αυξανω" => Ἀναιρεω,
        "αυξω" => Ὀφειλω,
        "αυω" => Ἐχω,
        "αφαιρεω" => Ἀναιρεω,
        "αφαμαρτανω" => Ἐχω,
        "αφανιζω" => Ἐχω,
        "αφελκω" => Ὀφειλω,
        "αφιημι" => Ἀφῑημῐ,
        "αφικνεομαι" => Ἡγεομαι,
        "αφιστημι" => Ἀνιστημι,
        "αφοριζω" => Ἀναιρεω,
        "αφριζω" => Ἐχω,
        "αφυπνιζω" => Ἐχω,
        "αφυσσω" => Ἐπῐλᾰμβᾰνω,
        "αχθομαι" => Οἰομαι,
        "βαθυνω" => Καθιζω,
        "βαινω" => Ἐχω,
        "βαλλιζω" => Καθιζω,
        "βαλλω" => Καθιζω,
        "βαπτιζω" => Ἀναιρεω,
        "βαπτω" => Ἐχω,
        "βαρβαροομαι" => Οἰομαι,
        "βαρυνω" => Καθιζω,
        "βασανιζω" => Καθιζω,
        "βασταζω" => Ἐχω,
        "βατταλογεω" => Ποιεω,
        "βαττολογεω" => Ποιεω,
        "βδεω" => Ἐχω,
        "βελτιοω" => Ποιεω,
        "βιαζομαι" => Οἰομαι,
        "βιβαζω" => Ἐχω,
        "βιβρωσκω" => Ἐπῐλᾰμβᾰνω,
        "βινεω" => Ἐχω,
        "βιοω" => Ποιεω,
        "βλαπτω" => Ἐχω,
        "βλαστανω" => Ἐχω,
        "βλασφημεω" => Ποιεω,
        "βλεπω" => Ἐχω,
        "βλισσω" => Ἐχω,
        "βλιττω" => Ἐχω,
        "βλυζω" => Ἐχω,
        "βλωσκω" => Ἐχω,
        "βοαω" => Ποιεω,
        "βοηθεω" => Ποιεω,
        "βοσκω" => Ὀφειλω,
        "βουλευω" => Ἐχω,
        "βουλομαι" => Οἰομαι,
        "βραδυνω" => Καθιζω,
        "βρασσω" => Ἐχω,
        "βραχυνω" => Καθιζω,
        "βρεχω" => Ἐχω,
        "βρυαζω" => Ἐχω,
        "βρυκω" => Ἐχω,
        "βρυχω" => Ἐχω,
        "γαμεω" => Τελεω,
        "γαμιζω" => Ἐχω,
        "γαναω" => Ἐχω,
        "γανυμαι" => Ἐχω,
        "γεγενημεθα" => Ἐπευχομαι,
        "γεγενηνται" => Οἰομαι,
        "γεγενησαι" => Οἰομαι,
        "γεγενησθε" => Οἰομαι,
        "γεγενησθον" => Οἰομαι,
        "γεγενησθω" => Οἰομαι,
        "γεγενησθων" => Οἰομαι,
        "γεγενησο" => Ἐπευχομαι,
        "γεγενηται" => Οἰομαι,
        "γεγονοι" => Ἐχω,
        "γεινομαι" => Οἰομαι,
        "γελαω" => Ἐχω,
        "γεμιζω" => Καθιζω,
        "γενειαω" => Ποιεω,
        "γενναω" => Ποιεω,
        "γευω" => Ἐχω,
        "γηθεω" => Ἐκλυτροω,
        "γηρασκω" => Ἐχω,
        "γιγνομαι" => Ἐχω,
        "γιγνωσκω" => Γῐγνωσκω,
        "γινωσκω" => Δεῑ,
        "γλυφω" => Ἐχω,
        "γναμπτω" => Ἐχω,
        "γνωριζω" => Καθιζω,
        "γοαω" => Ἐχω,
        "γραφω" => Ἐχω,
        "γραω" => Ἐχω,
        "γρυλιζω" => Ἐχω,
        "γυμναζω" => Ἐχω,
        "γυμνοω" => Ποιεω,
        "γυροω" => Ἐχω,
        "δαιζω" => Ἐχω,
        "δαινυμι" => Ζωννῡμῐ,
        "δακνω" => Προβαλλω,
        "δακρυω" => Ἐχω,
        "δαμαζω" => Τελεω,
        "δαμναω" => Ποιεω,
        "δαπαναω" => Ποιεω,
        "δαρθανω" => Ἐχω,
        "δατεομαι" => Δᾰτεομαι,
        "δει" => Δεῑ,
        "δειγματιζω" => Ἐχω,
        "δειδω" => Ἐχω,
        "δεικνυμι" => Τῐθημῐ,
        "δειλομαι" => Οἰομαι,
        "δειματοω" => Ἐχω,
        "δειπνεω" => Ποιεω,
        "δεμω" => Ἐχω,
        "δεξιοομαι" => Οἰομαι,
        "δερκομαι" => Οἰομαι,
        "δερω" => Καθιζω,
        "δεχομαι" => Λογῐζομαι,
        "δεω" => Ποιεω,
        "δηιοω" => Ἐχω,
        "δηλεομαι" => Περιειμι,
        "δηλοω" => Ποιεω,
        "δημιουργεω" => Ποιεω,
        "δημοκρατεομαι" => Δημοκρᾰτεομαι,
        "δημοσιευω" => Ἐχω,
        "δηοω" => Ἐχω,
        "δηω" => Δεῑ,
        "διαβαινω" => Ἐχω,
        "διαβαλλω" => Προβαλλω,
        "διαγωνιζομαι" => Λογῐζομαι,
        "διαζωννυμι" => Ἐπορνῡμῐ,
        "διαιρεω" => Ἀναιρεω,
        "διαιρω" => Ὑπαρχω,
        "διαιταω" => Ἐχω,
        "διακειμαι" => Οἰομαι,
        "διακονεω" => Ποιεω,
        "διαλεγω" => Ἀναιρεω,
        "διαμηνυω" => Ἐχω,
        "διανυω" => Ἀναιρεω,
        "διαπερθω" => Ἐχω,
        "διαπριω" => Ἐχω,
        "διαπυνθανομαι" => Κᾰτεργαζομαι,
        "διαρκεω" => Ἀναιρεω,
        "διαρπαζω" => Ἀναιρεω,
        "διασευομαι" => Οἰομαι,
        "διασκευαζω" => Ἀναιρεω,
        "διασκοπεω" => Ἐπῐλᾰμβᾰνω,
        "διασωζω" => Ἐχω,
        "διαφερω" => Δῐᾰφερω,
        "διαφορεω" => Ἐχω,
        "διαχεω" => Καθιζω,
        "διαχωριζω" => Καθιζω,
        "διδασκω" => Ἐχω,
        "διδρασκω" => Ἐχω,
        "διδωμι" => Οἰομαι,
        "διεπω" => Ἀναγω,
        "διερχομαι" => Πᾰρερχομαι,
        "διευθυνω" => Καθιζω,
        "διηγεομαι" => Περιειμι,
        "διιστημι" => Ἀνιστημι,
        "δικαζω" => Ἐχω,
        "δικαιοω" => Ποιεω,
        "δικειν" => Προβαλλω,
        "δινεω" => Ποιεω,
        "διοικεω" => Ἐχω,
        "διπλασιοομαι" => Προσευχομαι,
        "διπλοω" => Δεῑ,
        "δισταζω" => Ἐχω,
        "διψαω" => Ἐκλυτροω,
        "διωκω" => Ἐχω,
        "δογματιζω" => Καθιζω,
        "δογματογραφεω" => Ποιεω,
        "δογματοποιεω" => Ποιεω,
        "δοκεω" => Δοκεω,
        "δοκιμαζω" => Ἐχω,
        "δοκοω" => Ποιεω,
        "δολιζω" => Καθιζω,
        "δολιοω" => Ποιεω,
        "δολοφονεω" => Ποιεω,
        "δολοω" => Ποιεω,
        "δομεω" => Ποιεω,
        "δομοω" => Ποιεω,
        "δονεω" => Ποιεω,
        "δοξαζω" => Ἐχω,
        "δουλευω" => Ἐχω,
        "δουλοω" => Ποιεω,
        "δουπεω" => Ἐκλυτροω,
        "δραπετευω" => Ἐχω,
        "δρασσομαι" => Οἰομαι,
        "δραω" => Δρᾱω,
        "δρυπτω" => Ἐχω,
        "δυναμαι" => Οἰομαι,
        "δυπτω" => Ἐχω,
        "δυστυχεω" => Ποιεω,
        "δυω" => Ἐχω,
        "δωρεω" => Ποιεω,
        "εαω" => Ἀναιρεω,
        "εγγιζω" => Καθιζω,
        "εγειρω" => Ἐχω,
        "εγκαθιστημι" => Ἀνιστημι,
        "εγκαλεω" => Ἀναιρεω,
        "εγκαταλεγω" => Ἀναιρεω,
        "εδω" => Ἐδω,
        "εεργω" => Ἐχω,
        "εζομαι" => Ἀρνυμαι,
        "εθελω" => Ὀφειλω,
        "εθιζω" => Καθιζω,
        "ειδομαι" => Οἰομαι,
        "ειδον" => Προβαλλω,
        "εικαζω" => Ἀναιρεω,
        "εικω" => Ἐχω,
        "ειλεω" => Ἐχω,
        "ειλυω" => Ἐχω,
        "ειλω" => Ἐχω,
        "ειμι" => Εἰμῐ,
        "ειπον" => Προβαλλω,
        "ειργω" => Ἀναιρεω,
        "ειρω" => Ἀναιρεω,
        "ειρωνευομαι" => Ὀττευομαι,
        "εισαγω" => Ἐχω,
        "εισακουω" => Ἐχω,
        "εισαναβαινω" => Ἐχω,
        "εισερχομαι" => Πᾰρερχομαι,
        "εισοραω" => Ὑπερεχω,
        "εκγαμιζω" => Ἐχω,
        "εκδιωκω" => Ἐχω,
        "εκδυω" => Ἐχω,
        "εκκαλεω" => Ἀναιρεω,
        "εκλεγω" => Ἐχω,
        "εκλυτροω" => Ἐκλυτροω,
        "εκρεω" => Καθιζω,
        "εκτιθημι" => Τῐθημῐ,
        "εκφερω" => Προβαλλω,
        "εκχεω" => Ἐχω,
        "ελασσοω" => Ἐχω,
        "ελαυνω" => Ἐχω,
        "ελαω" => Ἐχω,
        "ελεγχω" => Ἐχω,
        "ελεεω" => Ἀναιρεω,
        "ελελιζω" => Ἀναιρεω,
        "ελευθεροω" => Ἐχω,
        "ελισσω" => Ἐχω,
        "ελκυω" => Ἀναιρεω,
        "ελκω" => Ἐχω,
        "ελλειπω" => Προβαλλω,
        "ελληνιζω" => Καθιζω,
        "ελπιζω" => Καθιζω,
        "ελπω" => Ἐχω,
        "εμβαλλω" => Προβαλλω,
        "εμεω" => Ἐχω,
        "εμπαιζω" => Ἐχω,
        "εμπηγνυμι" => Ἐμπηγνῡμῐ,
        "εμπιμπρημι" => Ποιεω,
        "εμπολαω" => Ἐχω,
        "εναιρω" => Ἐχω,
        "εναντιοομαι" => Κᾰτεργαζομαι,
        "εναριζω" => Ἀναιρεω,
        "ενδεω" => Ἐχω,
        "ενδυω" => Ἐχω,
        "ενειμι" => Τῐθημῐ,
        "ενεχω" => Ἐφεπω,
        "ενιημι" => Οἰομαι,
        "ενιστημι" => Ἀνιστημι,
        "εννυμι" => Οἰομαι,
        "ενοικεω" => Ἐχω,
        "ενοραω" => Ἐχω,
        "εντυγχανω" => Ἐχω,
        "ενυπνιαζω" => Ἐχω,
        "εξαγρεω" => Ἐχω,
        "εξαγω" => Ἀναιρεω,
        "εξαιρεω" => Ἀναιρεω,
        "εξαιρω" => Ἀναιρεω,
        "εξαιτεω" => Ἀναιρεω,
        "εξακολουθεω" => Ἐχω,
        "εξαναλυω" => Ἐχω,
        "εξαπτω" => Ὑπαρχω,
        "εξαρπαζω" => Ἐχω,
        "εξαρταω" => Ἀναιρεω,
        "εξαρτυω" => Ἀναιρεω,
        "εξαρχω" => Ὑπαρχω,
        "εξασθενεω" => Ἐχω,
        "εξαττικιζω" => Καθιζω,
        "εξαφανιζω" => Καθιζω,
        "εξειμι" => Περιειμι,
        "εξειπον" => Ἀφῑημῐ,
        "εξελαυνω" => Τελεω,
        "εξεραω" => Ἐχω,
        "εξεργαζομαι" => Κᾰτεργαζομαι,
        "εξερομαι" => Ἀρνυμαι,
        "εξερχομαι" => Πᾰρερχομαι,
        "εξεστι" => Ἀνιστημι,
        "εξεταζω" => Ἀναιρεω,
        "εξευρισκω" => Ἀναιρεω,
        "εξηγεομαι" => Ἡγεομαι,
        "εξιημι" => Ἀφῑημῐ,
        "εξικνεομαι" => Ἡγεομαι,
        "εξισοω" => Ἐχω,
        "εξιστημι" => Ἀνιστημι,
        "εξομολογεω" => Ἀναιρεω,
        "εξορκιζω" => Καθιζω,
        "εξοτρυνω" => Ἐχω,
        "εξυβριζω" => Καθιζω,
        "εξυπηρετεω" => Ἐχω,
        "εξυπνιζω" => Ἀναιρεω,
        "εοικα" => Ἐοικᾰ,
        "εορταζω" => Ἀναιρεω,
        "επαγγελλω" => Καθιζω,
        "επαγω" => Ἀναγω,
        "επαινεω" => Ἀναιρεω,
        "επαιρω" => Ἀναιρεω,
        "επαισσω" => Ἐχω,
        "επαιτεω" => Ἐχω,
        "επαιω" => Ἐχω,
        "επακουω" => Ἐχω,
        "επαλθαινω" => Ἐχω,
        "επαμυνω" => Ἐχω,
        "επαναλαμβανω" => Ὑπολαμβανω,
        "επαναλυω" => Ἐχω,
        "επαναχωρεω" => Ἐχω,
        "επανερχομαι" => Οἰομαι,
        "επανιημι" => Ἀφῑημῐ,
        "επαπολυω" => Ἐχω,
        "επαρχω" => Ἀναιρεω,
        "επειγω" => Ἐχω,
        "επειμι" => Τῐθημῐ,
        "επειπον" => Ἀφῑημῐ,
        "επεκβαινω" => Ἐχω,
        "επενδυνω" => Ἐχω,
        "επεντιθημι" => Τῐθημῐ,
        "επεξειμι" => Περιειμι,
        "επεξερχομαι" => Πᾰρερχομαι,
        "επεξηγεομαι" => Προσευχομαι,
        "επερομαι" => Ἀρνυμαι,
        "επερχομαι" => Πᾰρερχομαι,
        "επευφημεω" => Ἀναιρεω,
        "επεφνον" => Ἐπεφνον,
        "επεχω" => Ὑπαρχω,
        "επιγιγνωσκω" => Ἐχω,
        "επικαλεω" => Ἀειρω,
        "επιλαμβανω" => Ἐπῐλᾰμβᾰνω,
        "επιληθω" => Ἐχω,
        "επιμειδαω" => Ἐχω,
        "επιμελεομαι" => Οἰομαι,
        "επιμιμνησκομαι" => Ἐπιμιμνησκομαι,
        "επιπειθομαι" => Οἰομαι,
        "επισευω" => Περιειμι,
        "επισκεπτομαι" => Κᾰτεργαζομαι,
        "επισκευαζω" => Τελεω,
        "επισταμαι" => Ἀνιστημι,
        "επισυναπτω" => Ἐχω,
        "επιτιθημι" => Τῐθημῐ,
        "επιτρεχω" => Ἐχω,
        "επιφαινω" => Καθιζω,
        "επιφερω" => Ὑποφερω,
        "επιχειρεω" => Ἀναιρεω,
        "επιχεω" => Καθιζω,
        "εποιχομαι" => Οἰομαι,
        "επομαι" => Οἰομαι,
        "επονομαζω" => Ἀναιρεω,
        "επορνυμι" => Ἐπορνῡμῐ,
        "επορον" => Ἐπορον,
        "επουλοω" => Ἐχω,
        "επωμιζομαι" => Οἰομαι,
        "εραμαι" => Ἀνιστημι,
        "εραω" => Ἐχω,
        "εργαζομαι" => Κᾰτεργαζομαι,
        "εργω" => Ἀναιρεω,
        "ερδω" => Ἀναιρεω,
        "ερεθιζω" => Καθιζω,
        "ερειδω" => Ἀναιρεω,
        "ερειπω" => Ἐχω,
        "ερεσσω" => Ἐχω,
        "ερευγομαι" => Οἰομαι,
        "ερευθω" => Ἐχω,
        "ερευναω" => Ἐχω,
        "ερεφω" => Ἀναιρεω,
        "ερημοω" => Ἐχω,
        "εριδαινω" => Ἐχω,
        "εριζω" => Καθιζω,
        "ερομαι" => Οἰομαι,
        "ερπυζω" => Ἐχω,
        "ερπω" => Ἐχω,
        "ερρω" => Ὀφειλω,
        "ερυθραινω" => Ἀναιρεω,
        "ερυκω" => Ἐχω,
        "ερυω" => Ἐχω,
        "ερχομαι" => Οἰομαι,
        "ερωταω" => Ἐχω,
        "εσθιω" => Ἐχω,
        "ετεροζυγεω" => Ἐχω,
        "ετλην" => Ἐχω,
        "ετοιμαζω" => Ἀειρω,
        "ευαγγελιζω" => Καθιζω,
        "ευδω" => Ὀφειλω,
        "ευθενεω" => Ἐχω,
        "ευθηνεω" => Ἐχω,
        "ευθυνω" => Ἐχω,
        "ευλαβεομαι" => Οἰομαι,
        "ευλογεω" => Ἐχω,
        "ευναω" => Ἐχω,
        "ευνομεομαι" => Ἡγεομαι,
        "ευρισκω" => Λᾰμβᾰνω,
        "ευρυνω" => Καθιζω,
        "ευτρεπιζω" => Καθιζω,
        "ευτυχεω" => Ἀναιρεω,
        "ευφημιζω" => Καθιζω,
        "ευφραινω" => Ἐχω,
        "ευχαριστεω" => Ἀναιρεω,
        "ευχομαι" => Κᾰτεργαζομαι,
        "εφαγον" => Ἐπορον,
        "εφαλλομαι" => Οἰομαι,
        "εφανδανω" => Ἐχω,
        "εφελκω" => Ὀφειλω,
        "εφεπω" => Ἐφεπω,
        "εφευρισκω" => Ἀναιρεω,
        "εφηκω" => Ὑπαρχω,
        "εφιημι" => Ἀφῑημῐ,
        "εφιστημι" => Ἀνιστημι,
        "εφοραω" => Ἐπῐλᾰμβᾰνω,
        "εφορμαω" => Ἀναιρεω,
        "εφορμεω" => Ἀναιρεω,
        "εχθω" => Ἐχω,
        "εχω" => Ἐχω,
        "εψω" => Ὀφειλω,
        "ζευγνυμι" => Οἰομαι,
        "ζεω" => Ἐχω,
        "ζηλοω" => Ποιεω,
        "ζητεω" => Ποιεω,
        "ζυγοω" => Ποιεω,
        "ζω" => Ζω,
        "ζωγραφεω" => Ἐκλυτροω,
        "ζωγρεω" => Ποιεω,
        "ζωννυμι" => Ζωννῡμῐ,
        "ζωογονεω" => Ποιεω,
        "ζωοποιεω" => Ἐχω,
        "ηβαω" => Ἀναιρεω,
        "ηγεομαι" => Ἡγεομαι,
        "ηδομαι" => Οἰομαι,
        "ηδυνω" => Ἐχω,
        "ηθεω" => Ἐχω,
        "ηθολογεω" => Ἐχω,
        "ηλθον" => Προβαλλω,
        "ηλοω" => Ἐχω,
        "ημι" => Δεῑ,
        "ηρεμεω" => Ἐχω,
        "ησσαομαι" => Κᾰτεργαζομαι,
        "ησυχαζω" => Ἐχω,
        "ηχεω" => Ἐχω,
        "θαλλω" => Ἐχω,
        "θαλπω" => Ἐχω,
        "θαπτω" => Ἐχω,
        "θαρρεω" => Ἐχω,
        "θαρσεω" => Ποιεω,
        "θαυμαζω" => Ἐχω,
        "θεαομαι" => Οἰομαι,
        "θειαζω" => Ἐχω,
        "θεινω" => Ἐχω,
        "θειοω" => Ποιεω,
        "θελω" => Ὀφειλω,
        "θεοπροπεω" => Ποιεω,
        "θεραπευω" => Ἐχω,
        "θεω" => Ἐχω,
        "θεωρεω" => Ποιεω,
        "θηραω" => Ποιεω,
        "θιγγανω" => Ὑπολαμβανω,
        "θλαω" => Ποιεω,
        "θλιβω" => Ἐχω,
        "θναισκω" => Ἐχω,
        "θνησκω" => Προβαλλω,
        "θοαζω" => Ἐχω,
        "θρασσω" => Ὀφειλω,
        "θραττω" => Ἐχω,
        "θραυω" => Ἐχω,
        "θρυπτω" => Ἐχω,
        "θρωσκω" => Προβαλλω,
        "θυμιαω" => Ποιεω,
        "θυω" => Ἐχω,
        "θωαζω" => Ἐχω,
        "θωεω" => Ἐχω,
        "θωρακιζω" => Καθιζω,
        "θωρησσω" => Προβαλλω,
        "ιαινω" => Ἐχω,
        "ιαλλω" => Καθιζω,
        "ιαομαι" => Κᾰτεργαζομαι,
        "ιαπτω" => Ἐχω,
        "ιατρευω" => Ἀναιρεω,
        "ιδροω" => Ἐχω,
        "ιδρυω" => Ἀναιρεω,
        "ιζω" => Ἐχω,
        "ιημι" => Οἰομαι,
        "ιθυνω" => Ἐχω,
        "ικετευω" => Ἐχω,
        "ικνεομαι" => Ἡγεομαι,
        "ικω" => Ἐχω,
        "ιλασκομαι" => Οἰομαι,
        "ιμασσω" => Ἐχω,
        "ιναω" => Ἐχω,
        "ινεω" => Ἐχω,
        "ιππευω" => Ἀναιρεω,
        "ισοθεοω" => Ἀναιρεω,
        "ισορροπεω" => Ἐχω,
        "ιστημι" => Ἀνιστημι,
        "ισχυω" => Ἐχω,
        "ισχω" => Ἐχω,
        "ιχθυαζομαι" => Οἰομαι,
        "ιχθυαω" => Ἐχω,
        "ιχθυολογεω" => Ἐχω,
        "καθαιρεω" => Τελεω,
        "καθαιρω" => Ἐχω,
        "καθαπτω" => Ἐχω,
        "καθαρευω" => Ἐχω,
        "καθαριζω" => Καθιζω,
        "καθεζομαι" => Ἀρνυμαι,
        "καθελκω" => Ἀναγω,
        "καθευδω" => Ὀφειλω,
        "καθημαι" => Ἐχω,
        "καθιδρυω" => Ἀναιρεω,
        "καθιζανω" => Ἐχω,
        "καθιζω" => Καθιζω,
        "καθιημι" => Κᾰθιημῐ,
        "καθιστημι" => Ἀνιστημι,
        "καθοραω" => Ὑπερεχω,
        "καινοω" => Ποιεω,
        "καινυμαι" => Νεομαι,
        "καινω" => Προβαλλω,
        "καιω" => Ἐχω,
        "κακοπαθεω" => Ποιεω,
        "κακουργεω" => Ποιεω,
        "κακοω" => Ἐχω,
        "καλεω" => Κᾰλεω,
        "καλλιγραφεω" => Ἐχω,
        "καλυπτω" => Ἐχω,
        "καμμυω" => Ἐχω,
        "καμνω" => Προβαλλω,
        "καμπτω" => Ἐχω,
        "καναχεω" => Ποιεω,
        "κανονιζω" => Καθιζω,
        "καπνιζω" => Καθιζω,
        "καπτω" => Ἐχω,
        "καροω" => Ποιεω,
        "κασσυω" => Ἐχω,
        "καταβιβαζω" => Τελεω,
        "καταβοαω" => Ἐχω,
        "καταγω" => Ἐχω,
        "καταδιωκω" => Ἐχω,
        "καταδυω" => Ἐχω,
        "καταθνησκω" => Ἐπῐλᾰμβᾰνω,
        "καταικιζω" => Καθιζω,
        "καταιρω" => Προσᾰγω,
        "κατακαλεω" => Καθιζω,
        "κατακλαω" => Ἐχω,
        "κατακλυζω" => Ἐχω,
        "καταλαμβανω" => Ὑπολαμβανω,
        "καταλεγω" => Ἀναιρεω,
        "καταμυω" => Ἐχω,
        "καταναγκαζω" => Ἀναιρεω,
        "καταπαιδεραστεω" => Ἐχω,
        "καταπετροω" => Ἀναιρεω,
        "καταργεω" => Ἀναιρεω,
        "κατασκηνοω" => Ἐχω,
        "καταστρεφω" => Ἀναιρεω,
        "κατατρεχω" => Καθιζω,
        "καταφρονεω" => Ἐχω,
        "καταχραομαι" => Οἰομαι,
        "κατειμι" => Οἰομαι,
        "κατειργω" => Ἐχω,
        "κατεπειγω" => Ἐχω,
        "κατεργαζομαι" => Κᾰτεργαζομαι,
        "κατερυω" => Ἀναιρεω,
        "κατερχομαι" => Νεομαι,
        "κατευθυνω" => Καθιζω,
        "κατεχω" => Ὑπαρχω,
        "κατηγορεω" => Ἀναιρεω,
        "κατηχεω" => Ἐχω,
        "κατηχιζω" => Καθιζω,
        "κατισχυω" => Ἐχω,
        "κατοικειοομαι" => Οἰομαι,
        "κατοικεω" => Ἀναιρεω,
        "κατοικοδομεω" => Ἐχω,
        "κατοικονομεω" => Ἐχω,
        "κατοικοφθορεω" => Ἐχω,
        "καυχαομαι" => Οἰομαι,
        "κεαζω" => Ἐχω,
        "κειμαι" => Ἐχω,
        "κειρω" => Ἐχω,
        "κελαδεω" => Ποιεω,
        "κελευω" => Ἐχω,
        "κελλω" => Ἐχω,
        "κελομαι" => Οἰομαι,
        "κεντεω" => Ποιεω,
        "κεραιζω" => Ἐχω,
        "κεραννυμι" => Περιειμι,
        "κεραυνοω" => Ποιεω,
        "κερδαινω" => Ἐχω,
        "κερτομεω" => Ποιεω,
        "κευθω" => Ἐχω,
        "κεφαλοτομεω" => Ποιεω,
        "κηδευω" => Ἀναιρεω,
        "κηδω" => Ὀφειλω,
        "κηλεω" => Ποιεω,
        "κηρυσσω" => Ἐχω,
        "κινδυνευω" => Ἐχω,
        "κινεω" => Ποιεω,
        "κιω" => Ἐχω,
        "κλαζω" => Ἐχω,
        "κλαιω" => Καθιζω,
        "κλαω" => Ἐχω,
        "κλειδοω" => Ἐχω,
        "κλειω" => Ἐχω,
        "κλεπτω" => Ἐχω,
        "κλεω" => Ποιεω,
        "κληρονομεω" => Ποιεω,
        "κλινω" => Καθιζω,
        "κλονεω" => Ποιεω,
        "κλυζω" => Ἐχω,
        "κλυω" => Προβαλλω,
        "κλωθω" => Ἐχω,
        "κναπτω" => Ἐχω,
        "κναω" => Ποιεω,
        "κνιζω" => Ἐχω,
        "κοεω" => Ποιεω,
        "κοιμαω" => Ποιεω,
        "κοινοω" => Ἐχω,
        "κοινωνεω" => Ποιεω,
        "κοιρανεω" => Ποιεω,
        "κολαζω" => Ἐχω,
        "κολακευω" => Ἐχω,
        "κολαπτω" => Ἐχω,
        "κολλαω" => Ποιεω,
        "κολοβοω" => Ἐχω,
        "κολουω" => Ἐχω,
        "κολυμβαω" => Ἐχω,
        "κομαω" => Ποιεω,
        "κομβοω" => Ποιεω,
        "κομιζω" => Καθιζω,
        "κομπεω" => Ποιεω,
        "κομπολακεω" => Ποιεω,
        "κονιω" => Ἐχω,
        "κοπιαω" => Ἐχω,
        "κοπτω" => Ἐχω,
        "κορεννυμι" => Οἰομαι,
        "κοριζομαι" => Οἰομαι,
        "κορινθιαζομαι" => Οἰομαι,
        "κορυσσω" => Ἐχω,
        "κοσμεω" => Ποιεω,
        "κουρευομαι" => Οἰομαι,
        "κουροτροφεω" => Ποιεω,
        "κουφιζω" => Καθιζω,
        "κουφολογεω" => Ποιεω,
        "κραζω" => Ἐχω,
        "κραινω" => Ἐχω,
        "κρατευω" => Ἐχω,
        "κρατεω" => Ποιεω,
        "κρατυνω" => Καθιζω,
        "κραυγαζω" => Ἐχω,
        "κρεκω" => Ἐχω,
        "κρεμαννυμι" => Οἰομαι,
        "κρεμαω" => Ποιεω,
        "κρετεω" => Ποιεω,
        "κρητιζω" => Ἐχω,
        "κρινω" => Καθιζω,
        "κροτεω" => Ποιεω,
        "κρουω" => Ἐχω,
        "κρυπτω" => Ἐχω,
        "κταομαι" => Οἰομαι,
        "κτεινω" => Ἐχω,
        "κτενιζω" => Καθιζω,
        "κτιζω" => Ἐχω,
        "κτυπεω" => Ποιεω,
        "κυβερναω" => Ποιεω,
        "κυεω" => Ποιεω,
        "κυκαω" => Ποιεω,
        "κυκλοω" => Ἐχω,
        "κυλινδω" => Ἐχω,
        "κυμαινω" => Ἐχω,
        "κυνεω" => Ποιεω,
        "κυπτω" => Καθιζω,
        "κυροω" => Ποιεω,
        "κυω" => Ἐχω,
        "κωλυω" => Ἐχω,
        "λαβρηγορεω" => Ἐχω,
        "λαγαιω" => Ἐχω,
        "λαγγαζω" => Ἐχω,
        "λαγοθηρεω" => Ἐχω,
        "λαγχανω" => Κᾰλεω,
        "λαθραιοπραγεω" => Ποιεω,
        "λαθω" => Ἐχω,
        "λαικαζω" => Ἐχω,
        "λαιφασσω" => Ἐχω,
        "λακτιζω" => Καθιζω,
        "λακωνιζω" => Καθιζω,
        "λακωνομανεω" => Ἐχω,
        "λαλαγεω" => Ποιεω,
        "λαλαχευομαι" => Οἰομαι,
        "λαλεω" => Ποιεω,
        "λαμβανω" => Λᾰμβᾰνω,
        "λαμπαδαρχεω" => Ποιεω,
        "λαμπαδευω" => Ἐχω,
        "λαμπαδηφορεω" => Ποιεω,
        "λαμπαδιζω" => Καθιζω,
        "λαμπαδοδρομεω" => Ποιεω,
        "λαμπαδουχεω" => Ποιεω,
        "λαμπαδοφεγγεω" => Ποιεω,
        "λαμπω" => Ἐχω,
        "λανθανω" => Δῐᾰφερω,
        "λαξευω" => Ἐχω,
        "λαπασσω" => Ἐχω,
        "λαπτω" => Ἐχω,
        "λαρκοφορεω" => Ἐκλυτροω,
        "λασκω" => Ἐχω,
        "λατρειω" => Ἐχω,
        "λαφυσσω" => Ἐχω,
        "λαχαινω" => Ἐχω,
        "λαχανευω" => Ἐχω,
        "λαχνοομαι" => Περιειμι,
        "λεγω" => Ἐχω,
        "λειβω" => Ἐχω,
        "λειπω" => Προβαλλω,
        "λειτουργεω" => Ποιεω,
        "λεπτυνω" => Ἐχω,
        "λευκαινω" => Ἐχω,
        "λευκογραφεω" => Ποιεω,
        "λευσσω" => Ἐχω,
        "λεχομαι" => Οἰομαι,
        "ληζομαι" => Οἰομαι,
        "ληθω" => Προβαλλω,
        "ληιζομαι" => Οἰομαι,
        "ληκεω" => Ποιεω,
        "λιαζομαι" => Οἰομαι,
        "λιζω" => Ἐχω,
        "λικμαω" => Ποιεω,
        "λιμαινω" => Ἐχω,
        "λιμναζω" => Ἐχω,
        "λιμοκτονεω" => Ποιεω,
        "λιμφευω" => Ἐχω,
        "λινοστατεω" => Ἐχω,
        "λιπαρεω" => Ποιεω,
        "λισσομαι" => Οἰομαι,
        "λογιζομαι" => Λογῐζομαι,
        "λογοθετεω" => Ποιεω,
        "λογομαχεω" => Ποιεω,
        "λογχευω" => Ἐχω,
        "λογχιαζω" => Ἐχω,
        "λογχοβολεω" => Ποιεω,
        "λογχοω" => Ποιεω,
        "λοιδορεω" => Ποιεω,
        "λοιπογραφεω" => Ποιεω,
        "λοξοω" => Ποιεω,
        "λοπιζω" => Καθιζω,
        "λορδοω" => Ποιεω,
        "λουτιαω" => Ποιεω,
        "λουτροομαι" => Περιειμι,
        "λουτροφορεω" => Ποιεω,
        "λουω" => Ἐχω,
        "λυζω" => Ἐχω,
        "λυπεω" => Ἐχω,
        "λυριζω" => Καθιζω,
        "λυσιτελεω" => Ποιεω,
        "λυτροω" => Ποιεω,
        "λυω" => Ἐχω,
        "λωγαω" => Ποιεω,
        "μαδαω" => Ποιεω,
        "μαθητευω" => Ἐχω,
        "μαινομαι" => Οἰομαι,
        "μακαριζω" => Καθιζω,
        "μακρηγορεω" => Ποιεω,
        "μανθανω" => Ἐχω,
        "μαντευομαι" => Οἰομαι,
        "μαραινω" => Ἐχω,
        "μαραυγεω" => Ποιεω,
        "μαρμαιρω" => Ἐχω,
        "μαρτυρεω" => Ποιεω,
        "μασαομαι" => Δᾰτεομαι,
        "μασσω" => Ἐχω,
        "μασταζω" => Ἐχω,
        "μαστιγοω" => Οἰομαι,
        "μαστιζω" => Ἐχω,
        "μαστιχαω" => Ποιεω,
        "μαχομαι" => Οἰομαι,
        "μεθαλλομαι" => Οἰομαι,
        "μεθιστημι" => Ἀνιστημι,
        "μεθυσκω" => Μεθῠσκω,
        "μεθυω" => Ἐχω,
        "μειγνυμι" => Τῐθημῐ,
        "μειδαω" => Δεῑ,
        "μειδιαω" => Ποιεω,
        "μειλισσω" => Ἐχω,
        "μειοω" => Ποιεω,
        "μειρομαι" => Οἰομαι,
        "μελαινω" => Ἐχω,
        "μελεταω" => Ποιεω,
        "μελλω" => Ὀφειλω,
        "μελπω" => Ἐχω,
        "μελω" => Ὀφειλω,
        "μεμφομαι" => Οἰομαι,
        "μενω" => Ἐχω,
        "μεριζω" => Καθιζω,
        "μεριμναω" => Ποιεω,
        "μεστοω" => Ποιεω,
        "μετακαλεω" => Ἐχω,
        "μεταλαγχανω" => Ἐχω,
        "μεταλαμβανω" => Ὑπολαμβανω,
        "μεταφερω" => Ἐπῐλᾰμβᾰνω,
        "μετειμι" => Τῐθημῐ,
        "μετεκιαθον" => Προβαλλω,
        "μετερχομαι" => Πᾰρερχομαι,
        "μετεχω" => Ὑπαρχω,
        "μετρεω" => Ποιεω,
        "μετριαζω" => Ἐχω,
        "μηδιζω" => Ἐχω,
        "μηδομαι" => Οἰομαι,
        "μηκαομαι" => Οἰομαι,
        "μηκυνω" => Ἐχω,
        "μηνιω" => Τελεω,
        "μηνυω" => Ἐχω,
        "μηχαναομαι" => Περιειμι,
        "μηχαναω" => Ποιεω,
        "μηχανορραφεω" => Ἐχω,
        "μιαινω" => Ἐχω,
        "μιγνυμι" => Οἰομαι,
        "μικρογραφεω" => Ἐχω,
        "μιμεομαι" => Περιειμι,
        "μιμνησκω" => Ἐπῐλᾰμβᾰνω,
        "μινυριζω" => Καθιζω,
        "μισεω" => Ποιεω,
        "μισθοω" => Ποιεω,
        "μναομαι" => Νεομαι,
        "μνημονευω" => Ἐχω,
        "μνηστευω" => Ἐχω,
        "μολυνω" => Ἐχω,
        "μοναζω" => Ἐχω,
        "μοναρχεω" => Ἐχω,
        "μορφοω" => Ποιεω,
        "μοσχοποιεω" => Ποιεω,
        "μοχθεω" => Ποιεω,
        "μυεω" => Ποιεω,
        "μυζαω" => Ποιεω,
        "μυθολογεω" => Ποιεω,
        "μυκαομαι" => Κᾰτεργαζομαι,
        "μυκτηριζω" => Ἐχω,
        "μυριζω" => Καθιζω,
        "μυσσομαι" => Οἰομαι,
        "μυω" => Ἐχω,
        "μωκαομαι" => Οἰομαι,
        "μωλεω" => Ἐχω,
        "μωμαομαι" => Οἰομαι,
        "μωραινω" => Ἐχω,
        "ναιω" => Ἐχω,
        "ναρκοω" => Ποιεω,
        "νασσω" => Ἐχω,
        "ναυαγεω" => Ποιεω,
        "ναυκληρεω" => Ποιεω,
        "ναυπηγεω" => Ποιεω,
        "ναω" => Ποιεω,
        "νεαω" => Ποιεω,
        "νεικεω" => Ἐχω,
        "νεκραγωγεω" => Ἀναιρεω,
        "νεμεσαω" => Ποιεω,
        "νεμω" => Νεμω,
        "νεομαι" => Νεομαι,
        "νευω" => Ἐχω,
        "νεω" => Καθιζω,
        "νεωτεριζω" => Καθιζω,
        "νηστευω" => Ἐχω,
        "νιζω" => Ἐχω,
        "νικαω" => Ποιεω,
        "νισσομαι" => Οἰομαι,
        "νοεω" => Ποιεω,
        "νομιζω" => Καθιζω,
        "νοσεω" => Ποιεω,
        "νοσοκομεω" => Ἀναιρεω,
        "νοστεω" => Ποιεω,
        "νουθετεω" => Ἐχω,
        "νυσσω" => Ἐχω,
        "νωμαω" => Ποιεω,
        "ξαινω" => Ἐχω,
        "ξενιζω" => Καθιζω,
        "ξενοκτονεω" => Ἐχω,
        "ξεω" => Ἐχω,
        "ξηραινω" => Ἐχω,
        "ξυγκινδυνευω" => Ἐχω,
        "ξυλλαμβανω" => Κᾰλεω,
        "ξυμβουλευω" => Ἐχω,
        "ξυμμενω" => Καθιζω,
        "ξυμπολεμεω" => Ποιεω,
        "ξυμφερω" => Ἐχω,
        "ξυναγω" => Ἀναιρεω,
        "ξυναδικεω" => Ἐχω,
        "ξυναιρεω" => Ἀειρω,
        "ξυνεπιτιθημι" => Τῐθημῐ,
        "ξυνεπομαι" => Συνεπομαι,
        "ξυνεργαζομαι" => Κᾰτεργαζομαι,
        "ξυνερχομαι" => Πᾰρερχομαι,
        "ξυνιημι" => Ἀφῑημῐ,
        "ξυνιστημι" => Ἀνιστημι,
        "ξυνναυμαχεω" => Ποιεω,
        "ξυνοιδα" => Οἰομαι,
        "ξυνοικιζω" => Καθιζω,
        "ξυνοικοδομεω" => Ἐχω,
        "ξυνομνυμι" => Λογῐζομαι,
        "ξυρεω" => Ποιεω,
        "ξυρρηγνυμι" => Οἰομαι,
        "ξυσκοταζω" => Ἐχω,
        "ξυσσωζω" => Ἐχω,
        "ξυστρατευω" => Ἐχω,
        "οδηγεω" => Ἐχω,
        "οδυρομαι" => Λογῐζομαι,
        "οδυσσομαι" => Ἀναιρεω,
        "οζω" => Ὀφειλω,
        "οιγω" => Ἐχω,
        "οιδανω" => Ἐπῐλᾰμβᾰνω,
        "οιδεω" => Ἐχω,
        "οικειοομαι" => Οἰομαι,
        "οικειοω" => Ἀναιρεω,
        "οικεω" => Ἐχω,
        "οικιζω" => Καθιζω,
        "οικοδεσποτεω" => Ἐχω,
        "οικοδομεω" => Ἐχω,
        "οικονομεω" => Ἐχω,
        "οικοφθορεω" => Ἐχω,
        "οιμωζω" => Ἐχω,
        "οιομαι" => Οἰομαι,
        "οιφω" => Ἐχω,
        "οιχομαι" => Οἰομαι,
        "ολιγαρχεω" => Ἐχω,
        "ολισθανω" => Ἐχω,
        "ολλυμι" => Λογῐζομαι,
        "ολολυζω" => Ἐχω,
        "ολοφυρομαι" => Λογῐζομαι,
        "ομαδεω" => Ἐμπηγνῡμῐ,
        "ομαρτεω" => Ἐχω,
        "ομειχω" => Ἐχω,
        "ομιλεω" => Ἐχω,
        "ομνυμι" => Λογῐζομαι,
        "ομοιοω" => Ἀναιρεω,
        "ομολογεω" => Ἐχω,
        "ομοργνυμι" => Ζωννῡμῐ,
        "ομφυνω" => Καθιζω,
        "ονινημι" => Οἰομαι,
        "ονομαζω" => Ἀναιρεω,
        "ονομαινω" => Ἐχω,
        "οξυνω" => Ἀναιρεω,
        "οπαζω" => Ἀναιρεω,
        "οπιπευω" => Ἐχω,
        "οπλιζω" => Καθιζω,
        "οπταω" => Ἀναιρεω,
        "οραω" => Λᾰμβᾰνω,
        "οργαω" => Ἀναιρεω,
        "ορεγω" => Ἀναιρεω,
        "ορθοω" => Ἐχω,
        "οριζω" => Ἀναιρεω,
        "ορμαινω" => Ἐχω,
        "ορμαω" => Ἀναιρεω,
        "ορμεω" => Ἐχω,
        "ορμιζω" => Καθιζω,
        "ορνυμι" => Ζωννῡμῐ,
        "ορουω" => Ἐχω,
        "ορυσσω" => Ἐχω,
        "ορχεομαι" => Κᾰτεργαζομαι,
        "οσφραινομαι" => Οἰομαι,
        "οττευομαι" => Ὀττευομαι,
        "ουρεω" => Ἐχω,
        "ουταω" => Ἐχω,
        "οφειλω" => Ὀφειλω,
        "οχεω" => Ἐχω,
        "οψωνεω" => Ἐχω,
        "παγιδευω" => Ὀφειλω,
        "παιανιζω" => Καθιζω,
        "παιδεραστεω" => Ποιεω,
        "παιδευω" => Ἐχω,
        "παιζω" => Ἐχω,
        "παιω" => Ἐχω,
        "παλλω" => Ἐχω,
        "πανηγυριαρχεω" => Ἐχω,
        "παραγγελλω" => Καθιζω,
        "παραγω" => Ἐχω,
        "παραδιδωμι" => Οἰομαι,
        "παραινεω" => Ἀναιρεω,
        "παραιτεομαι" => Οἰομαι,
        "παρακαλεω" => Καθιζω,
        "παραλαμβανω" => Ἐχω,
        "παραμελεω" => Ἐχω,
        "παρατιθημι" => Τῐθημῐ,
        "παρατυγχανω" => Ἐχω,
        "παρειμι" => Τῐθημῐ,
        "παρειπον" => Προεῑπον,
        "παρεκτρεπω" => Ἐχω,
        "παρερχομαι" => Πᾰρερχομαι,
        "παρεχω" => Ὑπαρχω,
        "παρηγορεω" => Ποιεω,
        "παριστημι" => Ἀνιστημι,
        "παροικεω" => Ἐχω,
        "παροξυνω" => Ἐχω,
        "παροραω" => Ἐχω,
        "παροτρυνω" => Ἐχω,
        "παρουσιαζω" => Ἐχω,
        "πασσω" => Ἐχω,
        "παστιλλοω" => Ποιεω,
        "παστοω" => Ποιεω,
        "πασχητιαω" => Ποιεω,
        "πασχω" => Προβαλλω,
        "παταγεω" => Ποιεω,
        "πατασσω" => Ἐχω,
        "πατεομαι" => Ἡγεομαι,
        "πατεριζω" => Καθιζω,
        "πατεω" => Ποιεω,
        "πατριαζω" => Ἐχω,
        "πατροκτονεω" => Ποιεω,
        "παυω" => Ἐχω,
        "παχυνω" => Ἐχω,
        "πεδαω" => Ἐχω,
        "πεζοπορεω" => Ποιεω,
        "πειθω" => Ἐχω,
        "πειναω" => Ποιεω,
        "πειραζω" => Ἐχω,
        "πειραω" => Ποιεω,
        "πειρω" => Ἐχω,
        "πεκω" => Ἐχω,
        "πελαζω" => Τελεω,
        "πελω" => Ἐχω,
        "πεμπω" => Ἐχω,
        "πενθεω" => Ποιεω,
        "πεπνυμαι" => Ἐχω,
        "περαινω" => Ἐχω,
        "περαιοω" => Ποιεω,
        "περαω" => Ποιεω,
        "περδομαι" => Οἰομαι,
        "περθω" => Περθω,
        "περιειμι" => Περιειμι,
        "περιεχω" => Ὑπαρχω,
        "περιιστημι" => Ἀνιστημι,
        "περικλειω" => Ἐχω,
        "περιλυω" => Ἐχω,
        "περιοραω" => Ὑπερεχω,
        "περισκοπεω" => Ἐπῐλᾰμβᾰνω,
        "περιτειχιζω" => Καθιζω,
        "περιτρεχω" => Καθιζω,
        "περιφερω" => Ἐχω,
        "περιφρονεω" => Ἐχω,
        "περνημι" => Περνημῐ,
        "πεσσω" => Ἐχω,
        "πεταννυμι" => Οἰομαι,
        "πετομαι" => Οἰομαι,
        "πηγνυμι" => Οἰομαι,
        "πηδαω" => Ποιεω,
        "πημαινω" => Ἐχω,
        "πιαζω" => Ἐχω,
        "πιαινω" => Ἐχω,
        "πιδαω" => Ποιεω,
        "πιεζω" => Ἐχω,
        "πιλεω" => Ποιεω,
        "πιμπλημι" => Ἀνιστημι,
        "πιμπρημι" => Ἀνιστημι,
        "πινοω" => Ποιεω,
        "πινω" => Προβαλλω,
        "πιπρασκω" => Ἐπῐλᾰμβᾰνω,
        "πιπτω" => Προβαλλω,
        "πιστευω" => Ἐχω,
        "πλαζω" => Ἐχω,
        "πλαναω" => Ποιεω,
        "πλασσω" => Ἐχω,
        "πλατυνω" => Καθιζω,
        "πλειστηριζομαι" => Οἰομαι,
        "πλεκω" => Ἐχω,
        "πλεοναζω" => Ἐχω,
        "πλεονεκτεω" => Ποιεω,
        "πλεω" => Ἐχω,
        "πληθυνω" => Ἐχω,
        "πληθω" => Ἐχω,
        "πληροφορεω" => Ποιεω,
        "πληροω" => Ποιεω,
        "πλησιαζω" => Ἐχω,
        "πλησσω" => Ἐχω,
        "πληττω" => Ἐχω,
        "πλισσομαι" => Οἰομαι,
        "πλουτοκρατεομαι" => Δημοκρᾰτεομαι,
        "πλυνω" => Ἐχω,
        "πλωιζω" => Καθιζω,
        "πλωω" => Ἐχω,
        "πνεω" => Καθιζω,
        "πνιγω" => Ἐχω,
        "ποδιζω" => Καθιζω,
        "ποδοκτυπεω" => Ἐχω,
        "ποεω" => Ποιεω,
        "ποθεω" => Ἐχω,
        "ποιεω" => Ποιεω,
        "ποιμαινω" => Ἐχω,
        "πολεμεω" => Ποιεω,
        "πολεμοω" => Ποιεω,
        "πολιορκεω" => Ποιεω,
        "πολιοφυλακεω" => Ποιεω,
        "πολιτευω" => Ἐχω,
        "πονεω" => Ποιεω,
        "πορευω" => Ἐχω,
        "πορθεω" => Ποιεω,
        "ποριζω" => Καθιζω,
        "πορνευω" => Ἐχω,
        "πορσυνω" => Ἐχω,
        "ποτιζω" => Καθιζω,
        "πρασσω" => Ἐχω,
        "πραττω" => Ἐχω,
        "πραυνω" => Καθιζω,
        "πρεσβευω" => Ἐχω,
        "πρηθω" => Ἐχω,
        "προαγω" => Ὑπαρχω,
        "προαπολυω" => Ἐχω,
        "προβαλλω" => Προβαλλω,
        "προειμι" => Οἰομαι,
        "προειπον" => Προεῑπον,
        "προερχομαι" => Πᾰρερχομαι,
        "προετοιμαζω" => Ἀναιρεω,
        "προιημι" => Ἀφῑημῐ,
        "προιστημι" => Ἀνιστημι,
        "προισχω" => Ἀναγω,
        "προκαλεω" => Ἀναιρεω,
        "προκινδυνευω" => Ἐχω,
        "προλαμβανω" => Κᾰλεω,
        "προμηνυω" => Ἐχω,
        "προνοεω" => Ἐχω,
        "προοιδα" => Οἰομαι,
        "προοραω" => Ὑπερεχω,
        "προοριζω" => Καθιζω,
        "προοφειλω" => Ὀφειλω,
        "προπινω" => Προβαλλω,
        "προπονεω" => Ἐχω,
        "προσαγορευω" => Λᾰμβᾰνω,
        "προσαγω" => Προσᾰγω,
        "προσαυδαω" => Ἐχω,
        "προσειμι" => Οἰομαι,
        "προσειπον" => Προεῑπον,
        "προσερχομαι" => Οἰομαι,
        "προσευχομαι" => Προσευχομαι,
        "προσεχω" => Ὑπαρχω,
        "προσηκω" => Ὑπαρχω,
        "προσκαλεω" => Ἀναιρεω,
        "προσκυνεω" => Ποιεω,
        "προσλαμβανω" => Κᾰλεω,
        "προσμειγνυμι" => Ἀνιστημι,
        "προσοικεω" => Ἐχω,
        "προσφερω" => Ἐχω,
        "προσφωνεω" => Ἀντιλεγω,
        "προτερεω" => Ποιεω,
        "προτιμωρεω" => Ἐχω,
        "προυφειλω" => Ὀφειλω,
        "προφερω" => Ἐχω,
        "προωθεω" => Ἀναιρεω,
        "πταιω" => Ἐχω,
        "πταρνυμαι" => Ἐπορον,
        "πτεροω" => Ποιεω,
        "πτησσω" => Ἐχω,
        "πτισσω" => Ἐχω,
        "πτοεω" => Ἐχω,
        "πτυσσω" => Ἐχω,
        "πτυω" => Ἐχω,
        "πυθω" => Ἐχω,
        "πυκτευω" => Ἐχω,
        "πυνθανομαι" => Κᾰτεργαζομαι,
        "πυρεσσω" => Ἐχω,
        "πυροω" => Ἐχω,
        "πυρραζω" => Ἐχω,
        "πυρσαινω" => Δεῑ,
        "πωλεω" => Ἐχω,
        "ραινω" => Ἐχω,
        "ραντιζω" => Καθιζω,
        "ραπιζω" => Ἐχω,
        "ραπτω" => Ἐχω,
        "ρασσω" => Ἐχω,
        "ραφανιδοω" => Ἐκλυτροω,
        "ρεζω" => Ἐχω,
        "ρεμβω" => Ἐχω,
        "ρεπω" => Ἐχω,
        "ρεω" => Ἐχω,
        "ρηγνυμι" => Οἰομαι,
        "ριγεω" => Ἐκλυτροω,
        "ριπταζω" => Ἐχω,
        "ριπτω" => Ἐχω,
        "ροδιζω" => Ἐχω,
        "ροφεω" => Ἐκλυτροω,
        "ρυπαινω" => Ἐχω,
        "ρωμαιζω" => Ἐχω,
        "ρωννυμι" => Ζωννῡμῐ,
        "ρωομαι" => Οἰομαι,
        "σαβαζω" => Ὀφειλω,
        "σαγηνευω" => Ἐχω,
        "σαινω" => Ἐχω,
        "σαιρω" => Ἐχω,
        "σαμαινω" => Ἐχω,
        "σαοω" => Ἐχω,
        "σαρκαζω" => Ἐχω,
        "σαττω" => Ἐχω,
        "σβεννυμι" => Οἰομαι,
        "σεβομαι" => Οἰομαι,
        "σειρεω" => Ἐχω,
        "σειω" => Ἐχω,
        "σευω" => Ἐχω,
        "σηθω" => Ἐχω,
        "σηκοω" => Ἐχω,
        "σημαινω" => Ἐχω,
        "σημειοω" => Ἐχω,
        "σηπω" => Ἐχω,
        "σιγαω" => Ποιεω,
        "σιζω" => Ἐχω,
        "σιωπαω" => Ποιεω,
        "σκανδαλιζω" => Ἐχω,
        "σκαπτω" => Ἐχω,
        "σκεδαννυμι" => Σκεδᾰννῡμῐ,
        "σκελλω" => Κᾰλεω,
        "σκεπαζω" => Ἐχω,
        "σκεπτομαι" => Οἰομαι,
        "σκευαζω" => Ἐχω,
        "σκηνοω" => Ποιεω,
        "σκηπτω" => Ἐχω,
        "σκιαζω" => Ἐχω,
        "σκιρταω" => Ποιεω,
        "σκοπεω" => Ποιεω,
        "σκορπιζω" => Καθιζω,
        "σκοτοω" => Ποιεω,
        "σκυλευω" => Ἐχω,
        "σκυλλω" => Ἐχω,
        "σκυλοδεψεω" => Ποιεω,
        "σκωπτω" => Ἐχω,
        "σμαραγεω" => Ποιεω,
        "σμαω" => Ἐκλυτροω,
        "σμηχω" => Ἐχω,
        "σοβεω" => Ποιεω,
        "σοεω" => Ἐχω,
        "σολοικιζω" => Καθιζω,
        "σοφιζω" => Καθιζω,
        "σπανιζω" => Καθιζω,
        "σπαω" => Ἐχω,
        "σπειρω" => Ἐχω,
        "σπενδω" => Ἐχω,
        "σπερχω" => Ἐχω,
        "σπευδω" => Ἐχω,
        "σπιζω" => Ἐχω,
        "σπουδαζω" => Ἐχω,
        "σταζω" => Ἐχω,
        "σταθμαω" => Ἐχω,
        "σταλασσω" => Ἐχω,
        "στασιαζω" => Ἐχω,
        "στειβω" => Ἐχω,
        "στειχω" => Προβαλλω,
        "στελλω" => Ἐχω,
        "στεναζω" => Ἐχω,
        "στενοχωρεω" => Ποιεω,
        "στενοω" => Ποιεω,
        "στεργω" => Ἐχω,
        "στερεω" => Τελεω,
        "στεφανοω" => Ἐχω,
        "στεφω" => Ἐχω,
        "στηριζω" => Καθιζω,
        "στιγματιζω" => Ἐχω,
        "στιζω" => Ἐχω,
        "στιλβω" => Ἐχω,
        "στοιχειοω" => Ποιεω,
        "στορνυμι" => Στορνῡμῐ,
        "στοχαζομαι" => Οἰομαι,
        "στραβιζω" => Ἐχω,
        "στραγγαλοομαι" => Προσευχομαι,
        "στρατευω" => Ἐχω,
        "στρατηγεω" => Ποιεω,
        "στρατοπεδευω" => Ἐχω,
        "στρεφω" => Ἐχω,
        "στρουθιζω" => Καθιζω,
        "στρωννυμι" => Ζωννῡμῐ,
        "στυγεω" => Ποιεω,
        "στυω" => Ἐχω,
        "συγγιγνομαι" => Ἐχω,
        "συγκακουργεω" => Ποιεω,
        "συγκαλεω" => Καθιζω,
        "συγκινδυνευω" => Ἐχω,
        "συγκρυπτω" => Ἐχω,
        "συγχρονιζω" => Καθιζω,
        "συμβουλευω" => Ἐχω,
        "συμμαχεω" => Ἐχω,
        "συμμαχομαι" => Οἰομαι,
        "συμπολεμεω" => Ποιεω,
        "συμφερω" => Ἐχω,
        "συμφωνεω" => Ἐχω,
        "συναγω" => Ἀναιρεω,
        "συναδικεω" => Ἐχω,
        "συναιρεω" => Ἀειρω,
        "συναλιζω" => Καθιζω,
        "συναναστρεφω" => Ἀναιρεω,
        "συναπτω" => Ἀναιρεω,
        "συναρπαζω" => Ἀναιρεω,
        "συνδεω" => Ἐχω,
        "συνειμι" => Τῐθημῐ,
        "συνεπιτιθημι" => Τῐθημῐ,
        "συνεπομαι" => Συνεπομαι,
        "συνεργαζομαι" => Κᾰτεργαζομαι,
        "συνερχομαι" => Πᾰρερχομαι,
        "συνιημι" => Ἀφῑημῐ,
        "συνιστημι" => Ἀνιστημι,
        "συνναυμαχεω" => Ποιεω,
        "συνοιδα" => Ἐχω,
        "συνοικεω" => Ἐχω,
        "συνοικιζω" => Καθιζω,
        "συνοικοδομεω" => Ἐχω,
        "συνομνυμι" => Λογῐζομαι,
        "συντειχιζω" => Ἐχω,
        "συντυγχανω" => Ἐχω,
        "συρω" => Καθιζω,
        "σφαλλω" => Καθιζω,
        "σφετεριζω" => Καθιζω,
        "σφιγγω" => Ἐχω,
        "σφυζω" => Ἐχω,
        "σχαζω" => Ἐχω,
        "σχεδιαζω" => Ἐχω,
        "σχηματιζω" => Καθιζω,
        "σχιζω" => Ἐχω,
        "σχολαζω" => Ἐχω,
        "σωζω" => Ἐχω,
        "σωφρονεω" => Ποιεω,
        "ταλαιπωρεω" => Ποιεω,
        "ταμιευω" => Ἐχω,
        "ταρασσω" => Προβαλλω,
        "ταρβεω" => Ἐχω,
        "ταριχευω" => Ἐχω,
        "ταρχυω" => Ἐχω,
        "τασσω" => Ἐχω,
        "τεθηπα" => Οἰομαι,
        "τεθνηκετω" => Οἰομαι,
        "τεθνηκοι" => Ἐχω,
        "τεθνηκοιη" => Οἰομαι,
        "τεθνηκοιησ" => Οἰομαι,
        "τεθνηκοιμι" => Οἰομαι,
        "τεινω" => Ἐχω,
        "τειχιζω" => Καθιζω,
        "τειχοφυλακεω" => Ποιεω,
        "τεκμαιρομαι" => Οἰομαι,
        "τεκμηριοω" => Ποιεω,
        "τεκταινομαι" => Οἰομαι,
        "τελεθω" => Δεῑ,
        "τελειοω" => Ἐχω,
        "τελεοδρομεω" => Ποιεω,
        "τελευταω" => Ποιεω,
        "τελεω" => Τελεω,
        "τελλω" => Ἐχω,
        "τεμνω" => Προβαλλω,
        "τερατολογεω" => Ποιεω,
        "τερατοτοκεω" => Ἐχω,
        "τερατουργεω" => Ποιεω,
        "τερσομαι" => Οἰομαι,
        "τετραζω" => Ἐχω,
        "τετραινω" => Ἀναιρεω,
        "τευταζω" => Ἐχω,
        "τευχω" => Ἐχω,
        "τεχναζω" => Ἐχω,
        "τεχναομαι" => Περιειμι,
        "τεχνολογεω" => Ποιεω,
        "τηκω" => Ἐχω,
        "τηρεω" => Ποιεω,
        "τιθασευω" => Ἐχω,
        "τιθημι" => Τῐθημῐ,
        "τικτω" => Προβαλλω,
        "τιλλω" => Ἐχω,
        "τιμαω" => Ποιεω,
        "τιμωρεω" => Ποιεω,
        "τινασσω" => Ἐχω,
        "τινω" => Ἐχω,
        "τιτρωσκω" => Ἐπῐλᾰμβᾰνω,
        "τιττυβιζω" => Καθιζω,
        "τιω" => Ἐχω,
        "τολμαω" => Ποιεω,
        "τοποθετεω" => Ποιεω,
        "τραγωδεω" => Ποιεω,
        "τρεμω" => Ἐχω,
        "τρεπω" => Ἐχω,
        "τρεφω" => Ἐχω,
        "τρεχω" => Προβαλλω,
        "τριαζω" => Ἐχω,
        "τριβω" => Ἐχω,
        "τριζω" => Ἐχω,
        "τρομεω" => Ποιεω,
        "τρυγαω" => Ἐχω,
        "τρυπαω" => Ἐχω,
        "τρυφαω" => Ἐχω,
        "τρυχω" => Ἐχω,
        "τρυω" => Ἐχω,
        "τρωγλοδυτεω" => Ἐχω,
        "τρωγω" => Προβαλλω,
        "τυγχανω" => Κᾰλεω,
        "τυκιζω" => Καθιζω,
        "τυμβευω" => Ἀναιρεω,
        "τυπτω" => Ἐχω,
        "τυραννευω" => Ἐχω,
        "τυραννεω" => Ποιεω,
        "τυραννοκτονεω" => Ποιεω,
        "τυφω" => Ἐχω,
        "υαλιζω" => Καθιζω,
        "υβριζω" => Ἀναιρεω,
        "υγιαινω" => Ἐχω,
        "υδραινω" => Ἐχω,
        "υμνεω" => Ἐχω,
        "υπαγω" => Ἐχω,
        "υπακουω" => Ἐχω,
        "υπαναχωρεω" => Ἐχω,
        "υπαρχω" => Ὑπαρχω,
        "υπειπον" => Ἀφῑημῐ,
        "υπεκτιθεμαι" => Τῐθημῐ,
        "υπεκφευγω" => Καθιζω,
        "υπεραττικιζω" => Ἐχω,
        "υπερεχω" => Ὑπερεχω,
        "υπερνικαω" => Ἐχω,
        "υπερφερω" => Ἐχω,
        "υπηρετεω" => Ἐχω,
        "υπισχνεομαι" => Ἡγεομαι,
        "υπνοω" => Ἐχω,
        "υπογιγνομαι" => Οἰομαι,
        "υποκοριζομαι" => Λογῐζομαι,
        "υπολαμβανω" => Ὑπολαμβανω,
        "υπονοεω" => Ἐχω,
        "υποστηριζω" => Ἀναιρεω,
        "υποφερω" => Ὑποφερω,
        "υστερεω" => Ἐχω,
        "υφαινω" => Ἀναιρεω,
        "υφαιρεω" => Ἀναιρεω,
        "υφηγεομαι" => Ἡγεομαι,
        "υφιστημι" => Ἀνιστημι,
        "υψοω" => Ἐχω,
        "υω" => Ἐχω,
        "φαινω" => Ἐχω,
        "φανεροω" => Ἐχω,
        "φανταζω" => Ἐχω,
        "φαρω" => Ἐχω,
        "φενακιζω" => Καθιζω,
        "φερβω" => Ἐχω,
        "φερω" => Ἐχω,
        "φευγω" => Προβαλλω,
        "φημι" => Φημῐ,
        "φθανω" => Ἐχω,
        "φθεγγομαι" => Οἰομαι,
        "φθειρω" => Ἐχω,
        "φθιω" => Ἐχω,
        "φθονεω" => Ποιεω,
        "φιλεω" => Ποιεω,
        "φιλομαθεω" => Ποιεω,
        "φιλοσοφεω" => Ποιεω,
        "φιμοω" => Ποιεω,
        "φλιβω" => Ἐχω,
        "φλυαρεω" => Ποιεω,
        "φλυω" => Ἐχω,
        "φοβεω" => Ποιεω,
        "φοινικιζω" => Ἐχω,
        "φοιταω" => Ποιεω,
        "φονευω" => Ἐχω,
        "φορεω" => Ποιεω,
        "φορολογεω" => Ποιεω,
        "φορτιζω" => Καθιζω,
        "φορτοω" => Ποιεω,
        "φραζω" => Ἐχω,
        "φρασσω" => Ἐχω,
        "φρισσω" => Ἐχω,
        "φρονεω" => Ποιεω,
        "φροντιζω" => Καθιζω,
        "φρουρεω" => Ποιεω,
        "φρυγω" => Ἐχω,
        "φυγαδευω" => Ἐχω,
        "φυλακεω" => Ἐχω,
        "φυλακιζω" => Καθιζω,
        "φυλασσω" => Ἐχω,
        "φυλαττω" => Ἐχω,
        "φυρω" => Ἐχω,
        "φυσαω" => Ποιεω,
        "φυτευω" => Ἐχω,
        "φυω" => Ἐχω,
        "φωγω" => Ἐχω,
        "φωλευω" => Ἐχω,
        "φωνεω" => Ποιεω,
        "φωνομαχεω" => Ἐχω,
        "χαζομαι" => Οἰομαι,
        "χαιρω" => Ὀφειλω,
        "χαλαροω" => Προσευχομαι,
        "χαλαω" => Ἐχω,
        "χαλεπαινω" => Ἐχω,
        "χαλεπτω" => Ἐχω,
        "χανδανω" => Ἐχω,
        "χαοω" => Ἐχω,
        "χαρακτηριζω" => Καθιζω,
        "χαρασσω" => Ἐχω,
        "χαριζομαι" => Λογῐζομαι,
        "χαριζω" => Ἐχω,
        "χαριτοω" => Ποιεω,
        "χασκω" => Προβαλλω,
        "χατιζω" => Ἐχω,
        "χεζω" => Καθιζω,
        "χειροομαι" => Περιειμι,
        "χειρουργεω" => Ποιεω,
        "χεω" => Ἐχω,
        "χλευαζω" => Ἐχω,
        "χλιαινω" => Ἐχω,
        "χολοω" => Ἐχω,
        "χορευω" => Ἐχω,
        "χορηγεω" => Ποιεω,
        "χορταζω" => Ἐχω,
        "χοω" => Ποιεω,
        "χραομαι" => Οἰομαι,
        "χραω" => Ποιεω,
        "χρεμετιζω" => Καθιζω,
        "χρη" => Χρη,
        "χρηματιζω" => Καθιζω,
        "χρησιμευω" => Ἐχω,
        "χριω" => Ἐχω,
        "χρονιζω" => Καθιζω,
        "χωρεω" => Ποιεω,
        "χωριζω" => Καθιζω,
        "ψαλλω" => Καθιζω,
        "ψαω" => Ἐκλυτροω,
        "ψεγω" => Ἐχω,
        "ψευδω" => Ἐχω,
        "ψηλαφαω" => Ἐχω,
        "ψηφιζω" => Καθιζω,
        "ψιλοω" => Ποιεω,
        "ψυχω" => Ἐχω,
        "ωθεω" => Ἐπῐλᾰμβᾰνω,
        "ωμοθετεω" => Ἐχω,
        "ωνεομαι" => Ὠνεομαι,
        "ωρυομαι" => Οἰομαι,
        "ωφελεω" => Ἐχω,
        "ωχριω" => Δεῑ,
    }
};

/// Per conjugation, the cells whose ending takes the circumflex when accented on
/// its last syllable (`θεοῦ`).
pub static CIRCUMFLEX: [Cow<'static, [u16]>; 235] = [
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_ACT_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_PRES_ACT,
        INFINITIVE_FUT_ACT,
        INFINITIVE_AOR_ACT,
        PARTICIPLE_PRES_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ἐχω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_PRES_ACT,
        INFINITIVE_FUT_ACT,
        INFINITIVE_AOR_ACT,
        PARTICIPLE_PRES_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ποιεω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ὁρῐζω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        OPT_ACT_2_SG,
        OPT_ACT_3_SG,
        IMP_ACT_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Γῡροω
    Cow::Borrowed(&[
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_FUT_ACT,
        INFINITIVE_AOR_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ὑπᾰκουω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Χρημᾰτῐζω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Βουλευω
    Cow::Borrowed(&[
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Οἰομαι
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_ACT_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Δοκᾰω
    Cow::Borrowed(&[
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
    ]), // Προσερχομαι
    Cow::Borrowed(&[
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Σπᾰω
    Cow::Borrowed(&[
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_AOR_ACT,
    ]), // Σῠνᾰγω
    Cow::Borrowed(&[
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
    ]), // Κᾰτεργαζομαι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Τᾰσσω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ἁπτω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Εὐλογεω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Σῠνᾰρπᾰζω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἀσκεω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_PRES_ACT,
        INFINITIVE_AOR_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Νῑκαω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ὠνεομαι
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Πρεσβευω
    Cow::Borrowed(&[]), // Θωᾰω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Κτῐζω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Πειρᾰζω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Δῐκεω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_AOR_ACT,
    ]), // Πεμπω
    Cow::Borrowed(&[]), // Πᾰρερχομαι
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_ACT_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἀναιρεω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Παυω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Δῐοικεω
    Cow::Borrowed(&[
        IND_ACT_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        IMPERFECT_ACT_1_SG,
        IMPERFECT_ACT_3_SG,
    ]), // Πᾰρειμι
    Cow::Borrowed(&[]), // Εὐθῡνω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ψηφῐζω
    Cow::Borrowed(&[]), // Λῡω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ψῑλοω
    Cow::Borrowed(&[
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἐνᾰντῐοομαι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Θρῠπτω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_ACT_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἐαω
    Cow::Borrowed(&[AORIST_SUBJ_ACT_1_SG, AORIST_SUBJ_ACT_2_SG]), // Ἀφῑημῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Μᾰντευομαι
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Καθιζω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Κᾰτηγορεω
    Cow::Borrowed(&[]), // Πρεπω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_FUT_ACT,
        INFINITIVE_AOR_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Σφᾰλλω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_PRES_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Πᾰρᾰκᾰλεω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_AOR_ACT,
    ]), // Φᾰρμᾰσσω
    Cow::Borrowed(&[AORIST_SUBJ_MID_2_SG]), // Αἱρεω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ἀφᾰνῐζω
    Cow::Borrowed(&[
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
    ]), // Ἀραομαι
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Σῳζω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἀκουω
    Cow::Borrowed(&[]), // Ἀπεῑπον
    Cow::Borrowed(&[]), // Ὀνομαινω
    Cow::Borrowed(&[
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
    ]), // Ἡγεομαι
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἀγαπαω
    Cow::Borrowed(&[]), // Φειδομαι
    Cow::Borrowed(&[]), // Δῐωκω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Χραω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Μᾰρτῠρεω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Οἰκεω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ὀχεω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ὀνομᾰζω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Σῠνοικῐζω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἐξᾰκολουθεω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἀροω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Κᾰκοω
    Cow::Borrowed(&[]), // Στεινω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ὁμολογεω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Πειρᾰω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Πᾰρᾰλλᾰσσω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Κᾰτειργω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Δῐᾰλῡω
    Cow::Borrowed(&[]), // Πᾰροξῡνω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἐλᾰσσοω
    Cow::Borrowed(&[
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
    ]), // Λογεομαι
    Cow::Borrowed(&[]), // Ἰαυω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἐρωτᾰω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Πλᾰναω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ἐπᾰγγελλω
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Ἐπῐτῐθημῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Μνημονευω
    Cow::Borrowed(&[]), // Ἐξᾰπτω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Λιστραω
    Cow::Borrowed(&[]), // Ἐπευχομαι
    Cow::Borrowed(&[]), // Ὑστερεω
    Cow::Borrowed(&[]), // Ἐγκειμαι
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Δαμνημῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Πῐμπλημῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Πᾰρῐστημῐ
    Cow::Borrowed(&[SUBJ_MID_2_SG, SUBJ_PASS_2_SG]), // Μᾰρνᾰμαι
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Σῠνομνῡμῐ
    Cow::Borrowed(&[]), // Προσκειμαι
    Cow::Borrowed(&[]), // Ῥηγνῡμῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Ἐξῐστημῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Ἱστημι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ῥωννῡμῐ
    Cow::Borrowed(&[
        IND_ACT_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        IMPERFECT_ACT_2_SG,
        IMPERFECT_ACT_3_SG,
    ]), // Εἰμῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
        AORIST_SUBJ_MID_2_SG,
        AORIST_IMP_MID_2_SG,
    ]), // Ἱημι
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
        AORIST_SUBJ_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἐπεντῐθημῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
        AORIST_SUBJ_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
    ]), // Τῐθημῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
        AORIST_SUBJ_MID_2_SG,
        AORIST_IMP_MID_2_SG,
    ]), // Δῐδωμῐ
    Cow::Borrowed(&[AORIST_IMP_MID_2_SG, INFINITIVE_AOR_ACT]), // Εἰπον
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Δεικνῡμῐ
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Ἐφᾰγον
    Cow::Borrowed(&[]), // Ἑννῡμῐ
    Cow::Borrowed(&[]), // Ὀρνῡμῐ
    Cow::Borrowed(&[]), // Πηγνῡμῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
        AORIST_SUBJ_MID_2_SG,
    ]), // Ἀντᾰποδῐδωμῐ
    Cow::Borrowed(&[]), // Πετᾰννῡμῐ
    Cow::Borrowed(&[SUBJ_MID_2_SG, SUBJ_PASS_2_SG]), // Ἐρᾰμαι
    Cow::Borrowed(&[]), // Σβεννῡμῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Φημῐ
    Cow::Borrowed(&[]), // Ἐοικᾰ
    Cow::Borrowed(&[]), // Κεῑμαι
    Cow::Borrowed(&[]), // Μῑγνῡμῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Διατιθημῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Κερᾰννῡμῐ
    Cow::Borrowed(&[]), // Δυναμαι
    Cow::Borrowed(&[]), // Οἰδᾰ
    Cow::Borrowed(&[
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
        AORIST_SUBJ_MID_2_SG,
    ]), // Πᾰρᾰδιδωμῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Ἀνιστημι
    Cow::Borrowed(&[]), // Δεῑ
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ζωννῡμῐ
    Cow::Borrowed(&[]), // Κᾰθημαι
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ὀμνῡμῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Ἐφῐστημῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Κᾰθῐστημῐ
    Cow::Borrowed(&[]), // Ἠμῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
    ]), // Ἐτλην
    Cow::Borrowed(&[SUBJ_ACT_3_SG]), // Χρη
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Ἠλθον
    Cow::Borrowed(&[]), // Ζευγνῡμῐ
    Cow::Borrowed(&[AORIST_IMP_MID_2_SG]), // Ἀρνυμαι
    Cow::Borrowed(&[AORIST_IMP_MID_2_SG, INFINITIVE_AOR_ACT]), // Εἰδον
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Ἐπεφνον
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Μετεκῑᾰθον
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ὀλλῡμῐ
    Cow::Borrowed(&[]), // Ἀνοιγνυμι
    Cow::Borrowed(&[]), // Οἰγνῡμῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Πεπνῡμαι
    Cow::Borrowed(&[]), // Περνημῐ
    Cow::Borrowed(&[]), // Κρεμᾰννῡμῐ
    Cow::Borrowed(&[SUBJ_MID_2_SG]), // Ἐπῐστᾰμαι
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Πτᾰρνῠμαι
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Ἐπορον
    Cow::Borrowed(&[]), // Δαινῡμῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Ἐνῐστημῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Πᾰρᾰτῐθημῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Κατατῐθημῐ
    Cow::Borrowed(&[AORIST_IMP_MID_2_SG, INFINITIVE_AOR_ACT]), // Κεκᾰδον
    Cow::Borrowed(&[SUBJ_MID_2_SG]), // Ἀγαμαι
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Σῠνιστημῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
        AORIST_SUBJ_MID_2_SG,
    ]), // Ξῠνιστημῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
        AORIST_SUBJ_MID_2_SG,
    ]), // Ἀπᾰνῐστημῐ
    Cow::Borrowed(&[]), // Καινῠμαι
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        PERFECT_SUBJ_ACT_1_SG,
        PERFECT_SUBJ_ACT_2_SG,
        PERFECT_SUBJ_ACT_3_SG,
    ]), // Ἐγκᾰθῐστημῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Μεθῐστημῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Διαζωννῡμῐ
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Σκεδᾰννῡμῐ
    Cow::Borrowed(&[]), // Κορεννῡμῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ὀμοργνῡμῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Διϊστημι
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Στορνῡμῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Ἀφῐστημῐ
    Cow::Borrowed(&[]), // Κᾰτακειμαι
    Cow::Borrowed(&[]), // Συνοιδᾰ
    Cow::Borrowed(&[]), // Ξῠνοιδᾰ
    Cow::Borrowed(&[]), // Προοιδᾰ
    Cow::Borrowed(&[]), // Ὀνινημῐ
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἐμπιμπρημι
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Ἀνθιστημι
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Περῐῐστημῐ
    Cow::Borrowed(&[]), // Ἐπορνῡμῐ
    Cow::Borrowed(&[SUBJ_ACT_3_SG]), // Ἐξεστῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Ὑφιστημι
    Cow::Borrowed(&[]), // Γεγονοι
    Cow::Borrowed(&[]), // Γεγενησαι
    Cow::Borrowed(&[]), // Γεγενηται
    Cow::Borrowed(&[]), // Γεγενησο
    Cow::Borrowed(&[]), // Γεγενησθον
    Cow::Borrowed(&[]), // Γεγενησθων
    Cow::Borrowed(&[]), // Γεγενηνται
    Cow::Borrowed(&[]), // Γεγενησθε
    Cow::Borrowed(&[]), // Γεγενημεθα
    Cow::Borrowed(&[]), // Τεθνηκας
    Cow::Borrowed(&[]), // Τεθνηκῃ
    Cow::Borrowed(&[]), // Τεθνηκοι
    Cow::Borrowed(&[]), // Τεθνηκοιη
    Cow::Borrowed(&[]), // Τεθνηκοιμι
    Cow::Borrowed(&[]), // Τεθνηκοιην
    Cow::Borrowed(&[]), // Τεθνηκοις
    Cow::Borrowed(&[]), // Τεθνηκοιης
    Cow::Borrowed(&[]), // Τεθνηκῃς
    Cow::Borrowed(&[]), // Τεθνηκαμεν
    Cow::Borrowed(&[]), // Τεθνηκωμεν
    Cow::Borrowed(&[]), // Τεθνηκοιμεν
    Cow::Borrowed(&[]), // Τεθνηκατε
    Cow::Borrowed(&[]), // Τεθνηκητε
    Cow::Borrowed(&[]), // Τεθνηκοιτε
    Cow::Borrowed(&[]), // Τεθνηκετε
    Cow::Borrowed(&[]), // Τεθνηκασι
    Cow::Borrowed(&[]), // Τεθνηκασιν
    Cow::Borrowed(&[]), // Τεθνηκωσι
    Cow::Borrowed(&[]), // Τεθνηκωσιν
    Cow::Borrowed(&[]), // Τεθνηκοιεν
    Cow::Borrowed(&[]), // Τεθνηκοντων
    Cow::Borrowed(&[]), // Τεθνηκατον
    Cow::Borrowed(&[]), // Τεθνηκητον
    Cow::Borrowed(&[]), // Τεθνηκοιτον
    Cow::Borrowed(&[]), // Τεθνηκοιτην
    Cow::Borrowed(&[]), // Τεθνηκετον
    Cow::Borrowed(&[]), // Τεθνηκετων
    Cow::Borrowed(&[]), // Προσμειγνῡμῐ
    Cow::Borrowed(&[SUBJ_MID_2_SG, SUBJ_PASS_2_SG]), // Κιδνᾰμαι
    Cow::Borrowed(&[]), // Τεθηπᾰ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Ἀντῐκᾰθῐστημῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Στρωννῡμῐ
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Δῐκεῑν
    Cow::Borrowed(&[]), // Ξῠρρηγνῡμῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        PERFECT_SUBJ_ACT_1_SG,
        PERFECT_SUBJ_ACT_2_SG,
        PERFECT_SUBJ_ACT_3_SG,
    ]), // Ἀμφιστημι
    Cow::Borrowed(&[]), // Γανῠμαι
    Cow::Borrowed(&[]), // Δῐακειμαι
    Cow::Borrowed(&[]), // Ἀδησα
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Μειγνῡμῐ
    Cow::Borrowed(&[]), // Ἐμπηγνῡμῐ
    Cow::Borrowed(&[
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        AORIST_SUBJ_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ὑπεκτιθεμαι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἀναρρωννυμι
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
        AORIST_SUBJ_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἐκτιθημῐ
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Προϊστημῐ
    Cow::Borrowed(&[]), // Λλυμι
    Cow::Borrowed(&[]), // Γνῡμῐ
    Cow::Borrowed(&[]), // Ρρηγνῡμῐ
    Cow::Borrowed(&[]), // Δειδῐᾰ
    Cow::Borrowed(&[]), // Κᾰτᾰμῑγνῡμῐ
    Cow::Borrowed(&[]), // Προδῐδωμῐ
];

/// The cells whose final `-αι`, `-οι` counts as long.
pub static LONG_FINALS: [u16; 7] = [
    OPT_ACT_3_SG,
    FUTURE_OPT_ACT_3_SG,
    AORIST_OPT_ACT_3_SG,
    PERFECT_OPT_ACT_3_SG,
    FUTURE_PERFECT_OPT_ACT_3_SG,
    INFINITIVE_PRES_ACT,
    INFINITIVE_PERF_ACT,
];
