//! Accent. Each word has a class: an accent pattern, which says cell by cell
//! which rule places the accent, and the syllable its stem is accented on,
//! counted from the lemma's end. `Stem`: where the lemma has it (persistent);
//! `Ending`: on the ending's first syllable; `Recessive`: as far from the end
//! as the law of limitation allows; `Last`, `NextToLast`: on that syllable of
//! the word. The law then moves it where it must, and
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
pub static PATTERNS: [AccentPattern<'static>; 32] = [
    // 0, `ἐχω`: 1,188 words.
    pattern!("ἐχω", 1188;
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
    opt_act_1: Stem, Stem, Stem;
    opt_act_2: Stem, Stem, Stem;
    opt_act_3: Stem, Recessive, Stem;
    opt_mid_1: Recessive, Stem, Recessive;
    opt_mid_2: Stem, Stem, Stem;
    opt_mid_3: Stem, Recessive, Stem;
    opt_pass_1: Recessive, Stem, Recessive;
    opt_pass_2: Stem, Stem, Stem;
    opt_pass_3: Stem, Recessive, Stem;
    imp_act_1: Stem, Stem, Stem;
    imp_act_2: Recessive, Stem, Stem;
    imp_act_3: Recessive, Recessive, NextToLast;
    imp_mid_1: Stem, Stem, Stem;
    imp_mid_2: Stem, Stem, Stem;
    imp_mid_3: Recessive, Recessive, Recessive;
    imp_pass_1: Stem, Stem, Stem;
    imp_pass_2: Stem, Stem, Stem;
    imp_pass_3: Recessive, Recessive, NextToLast;
    imperfect_act_1: Recessive, Stem, Stem;
    imperfect_act_2: Recessive, Stem, Stem;
    imperfect_act_3: Recessive, Stem, Recessive;
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
    aorist_ind_pass_3: Recessive, Recessive, Recessive;
    aorist_subj_act_1: Recessive, Stem, Recessive;
    aorist_subj_act_2: Recessive, Recessive, Recessive;
    aorist_subj_act_3: Recessive, Recessive, Recessive;
    aorist_subj_mid_1: Recessive, Stem, Recessive;
    aorist_subj_mid_2: Recessive, Recessive, Recessive;
    aorist_subj_mid_3: Recessive, Recessive, Recessive;
    aorist_subj_pass_1: Last, Stem, NextToLast;
    aorist_subj_pass_2: Last, NextToLast, NextToLast;
    aorist_subj_pass_3: Last, NextToLast, NextToLast;
    aorist_opt_act_1: Recessive, Stem, Recessive;
    aorist_opt_act_2: Recessive, Recessive, Recessive;
    aorist_opt_act_3: Recessive, Recessive, Recessive;
    aorist_opt_mid_1: Recessive, Stem, Recessive;
    aorist_opt_mid_2: Recessive, Recessive, Recessive;
    aorist_opt_mid_3: Recessive, Recessive, Recessive;
    aorist_opt_pass_1: Recessive, Stem, NextToLast;
    aorist_opt_pass_2: Recessive, NextToLast, NextToLast;
    aorist_opt_pass_3: Recessive, Recessive, NextToLast;
    aorist_imp_act_1: Stem, Stem, Stem;
    aorist_imp_act_2: Recessive, Recessive, Recessive;
    aorist_imp_act_3: Recessive, Recessive, Recessive;
    aorist_imp_mid_1: Stem, Stem, Stem;
    aorist_imp_mid_2: Recessive, Recessive, Recessive;
    aorist_imp_mid_3: Recessive, Recessive, Recessive;
    aorist_imp_pass_1: Stem, Stem, Stem;
    aorist_imp_pass_2: Recessive, Recessive, Recessive;
    aorist_imp_pass_3: Recessive, Recessive, Recessive;
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
    perfect_subj_mid_1: NextToLast, Stem, NextToLast;
    perfect_subj_mid_2: NextToLast, NextToLast, NextToLast;
    perfect_subj_mid_3: NextToLast, NextToLast, NextToLast;
    perfect_subj_pass_1: NextToLast, Stem, NextToLast;
    perfect_subj_pass_2: NextToLast, NextToLast, NextToLast;
    perfect_subj_pass_3: NextToLast, NextToLast, NextToLast;
    perfect_opt_act_1: Recessive, Stem, Recessive;
    perfect_opt_act_2: Recessive, Recessive, Recessive;
    perfect_opt_act_3: Recessive, Stem, Recessive;
    perfect_opt_mid_1: NextToLast, Stem, NextToLast;
    perfect_opt_mid_2: NextToLast, NextToLast, NextToLast;
    perfect_opt_mid_3: NextToLast, NextToLast, NextToLast;
    perfect_opt_pass_1: NextToLast, Stem, NextToLast;
    perfect_opt_pass_2: NextToLast, NextToLast, NextToLast;
    perfect_opt_pass_3: NextToLast, NextToLast, NextToLast;
    perfect_imp_act_1: Stem, Stem, Stem;
    perfect_imp_act_2: Recessive, Recessive, Recessive;
    perfect_imp_act_3: Stem, Stem, NextToLast;
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
    infinitive_aor: NextToLast, Stem, NextToLast;
    infinitive_perf: NextToLast, NextToLast, NextToLast;
    infinitive_futp: Stem, Stem, Stem;
    participle_pres: Stem, Recessive, Recessive;
    participle_fut: Recessive, Recessive, Recessive;
    participle_aor: Stem, Recessive, Last;
    participle_perf: Last, NextToLast, NextToLast;
    participle_futp: Stem, Stem, Stem;
    ),
    // 1, `ποιεω`: 299 words. Like 0 (`ἐχω`), except in 13 cells.
    pattern!("ποιεω", 299, like 0;
    imperfect_act_1: _, _, Ending;
    imperfect_act_2: _, Ending, Ending;
    imperfect_mid_2: Ending, Ending, Ending;
    imperfect_mid_3: Ending, _, Ending;
    imperfect_pass_2: Ending, Ending, Ending;
    imperfect_pass_3: Ending, _, Ending;
    ),
    // 2, `καθιζω`: 176 words. Like 0 (`ἐχω`), except in 27 cells.
    pattern!("καθιζω", 176, like 0;
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
    // 3, `ὁρῐζω`: 85 words. Like 0 (`ἐχω`), except in 27 cells.
    pattern!("ὁρῐζω", 85, like 0;
    future_ind_act_1: Last, _, NextToLast;
    future_ind_act_2: Last, NextToLast, NextToLast;
    future_ind_act_3: Last, NextToLast, NextToLast;
    future_ind_mid_1: NextToLast, _, _;
    future_ind_mid_2: Last, NextToLast, NextToLast;
    future_ind_mid_3: NextToLast, NextToLast, NextToLast;
    future_opt_act_1: _, _, NextToLast;
    future_opt_act_2: _, NextToLast, NextToLast;
    future_opt_act_3: _, _, NextToLast;
    future_opt_mid_2: NextToLast, NextToLast, NextToLast;
    future_opt_mid_3: NextToLast, _, NextToLast;
    infinitive_fut: Last, NextToLast, _;
    participle_fut: Last, _, _;
    ),
    // 4, `προσερχομαι`: 36 words. Like 1 (`ποιεω`), except in 49 cells.
    pattern!("προσερχομαι", 36, like 1;
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
    ),
    // 5, `ἀνῐστημῐ`: 31 words. Like 0 (`ἐχω`), except in 29 cells.
    pattern!("ἀνῐστημῐ", 31, like 0;
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
    infinitive_pres: NextToLast, _, _;
    participle_pres: Ending, _, _;
    ),
    // 6, `ζωννῡμῐ`: 23 words. Like 0 (`ἐχω`), except in 2 cells.
    pattern!("ζωννῡμῐ", 23, like 0;
    infinitive_pres: NextToLast, _, _;
    participle_pres: Ending, _, _;
    ),
    // 7, `χεω`: 23 words. Like 0 (`ἐχω`), except in 11 cells.
    pattern!("χεω", 23, like 0;
    imperfect_act_1: _, _, NextToLast;
    imperfect_act_2: _, NextToLast, NextToLast;
    imperfect_mid_2: _, NextToLast, NextToLast;
    imperfect_mid_3: NextToLast, _, NextToLast;
    imperfect_pass_2: _, NextToLast, NextToLast;
    imperfect_pass_3: NextToLast, _, NextToLast;
    ),
    // 8, `τῐθημῐ`: 20 words. Like 5 (`ἀνῐστημῐ`), except in 11 cells.
    pattern!("τῐθημῐ", 20, like 5;
    ind_act_3: _, _, Stem;
    opt_mid_2: Ending, Ending, Ending;
    opt_mid_3: Ending, _, Ending;
    opt_pass_2: Ending, Ending, Ending;
    opt_pass_3: Ending, _, Ending;
    ),
    // 9, `ἐξαιρεω`: 16 words. Like 0 (`ἐχω`), except in 2 cells.
    pattern!("ἐξαιρεω", 16, like 0;
    aorist_imp_mid_2: Stem, _, _;
    infinitive_aor: Stem, _, _;
    ),
    // 10, `πᾰρερχομαι`: 16 words. Like 0 (`ἐχω`), except in 6 cells.
    pattern!("πᾰρερχομαι", 16, like 0;
    aorist_ind_act_1: NextToLast, _, _;
    aorist_ind_act_2: NextToLast, _, _;
    aorist_ind_act_3: NextToLast, _, NextToLast;
    infinitive_aor: Last, _, _;
    participle_aor: Last, _, _;
    ),
    // 11, `στειχω`: 15 words. Like 0 (`ἐχω`), except in 4 cells.
    pattern!("στειχω", 15, like 0;
    aorist_imp_mid_2: Last, _, _;
    infinitive_aor: Ending, Ending, _;
    participle_aor: Ending, _, _;
    ),
    // 12, `ὑπᾰγω`: 13 words. Like 11 (`στειχω`), except in 3 cells.
    pattern!("ὑπᾰγω", 13, like 11;
    infinitive_aor: Last, NextToLast, _;
    participle_aor: Last, _, _;
    ),
    // 13, `πᾰσχω`: 12 words. Like 0 (`ἐχω`), except in 2 cells.
    pattern!("πᾰσχω", 12, like 0;
    infinitive_aor: Ending, _, _;
    participle_aor: Ending, _, _;
    ),
    // 14, `πᾰρεχω`: 12 words. Like 12 (`ὑπᾰγω`), except in 8 cells.
    pattern!("πᾰρεχω", 12, like 12;
    imperfect_act_1: Stem, _, _;
    imperfect_act_2: Stem, _, _;
    imperfect_act_3: Stem, _, Stem;
    aorist_ind_act_1: Stem, _, _;
    aorist_ind_act_2: Stem, _, _;
    aorist_ind_act_3: Stem, _, Stem;
    ),
    // 15, `ἀφῑημῐ`: 11 words. Like 0 (`ἐχω`), except in 12 cells.
    pattern!("ἀφῑημῐ", 11, like 0;
    aorist_subj_act_1: Stem, _, Stem;
    aorist_subj_act_2: Stem, Stem, Stem;
    aorist_subj_act_3: Stem, Stem, Stem;
    aorist_opt_act_1: _, _, Stem;
    aorist_opt_act_2: _, Stem, Stem;
    aorist_opt_act_3: _, _, Stem;
    ),
    // 16, `ἐπῐλᾰμβᾰνω`: 9 words. Like 0 (`ἐχω`), except in 11 cells.
    pattern!("ἐπῐλᾰμβᾰνω", 9, like 0;
    future_ind_act_1: _, _, Recessive;
    future_ind_act_2: _, Recessive, Recessive;
    future_ind_act_3: _, Recessive, Recessive;
    future_opt_act_1: _, _, Recessive;
    future_opt_act_2: _, Recessive, Recessive;
    future_opt_act_3: _, _, Recessive;
    infinitive_aor: _, Recessive, _;
    participle_aor: Recessive, _, _;
    ),
    // 17, `ἀναιρεω`: 8 words. Like 10 (`πᾰρερχομαι`), except in 3 cells.
    pattern!("ἀναιρεω", 8, like 10;
    aorist_imp_mid_2: Stem, _, _;
    infinitive_aor: Stem, _, _;
    participle_aor: Stem, _, _;
    ),
    // 18, `ἐνῑημῐ`: 8 words. Like 6 (`ζωννῡμῐ`), except in 1 cell.
    pattern!("ἐνῑημῐ", 8, like 6;
    participle_pres: Last, _, _;
    ),
    // 19, `ὑπᾰρχω`: 8 words. Like 14 (`πᾰρεχω`), except in 4 cells.
    pattern!("ὑπᾰρχω", 8, like 14;
    aorist_imp_mid_2: Recessive, _, _;
    infinitive_aor: NextToLast, Stem, _;
    participle_aor: Stem, _, _;
    ),
    // 20, `ἀπειπον`: 8 words. Like 3 (`ὁρῐζω`), except in 8 cells.
    pattern!("ἀπειπον", 8, like 3;
    aorist_ind_act_1: NextToLast, _, _;
    aorist_ind_act_2: NextToLast, _, _;
    aorist_ind_act_3: NextToLast, _, NextToLast;
    aorist_imp_mid_2: Last, _, _;
    infinitive_aor: Last, NextToLast, _;
    participle_aor: Last, _, _;
    ),
    // 21, `εἰδον`: 7 words. Like 10 (`πᾰρερχομαι`), except in 3 cells.
    pattern!("εἰδον", 7, like 10;
    aorist_imp_act_2: Last, _, _;
    aorist_imp_mid_2: Last, _, _;
    infinitive_aor: _, NextToLast, _;
    ),
    // 22, `ὠνεομαι`: 5 words. Like 0 (`ἐχω`), except in 1 cell.
    pattern!("ὠνεομαι", 5, like 0;
    infinitive_aor: _, Recessive, _;
    ),
    // 23, `δῐᾰφερω`: 5 words. Like 16 (`ἐπῐλᾰμβᾰνω`), except in 2 cells.
    pattern!("δῐᾰφερω", 5, like 16;
    infinitive_aor: _, Stem, _;
    participle_aor: Stem, _, _;
    ),
    // 24, `προβαλλω`: 4 words. Like 2 (`καθιζω`), except in 4 cells.
    pattern!("προβαλλω", 4, like 2;
    aorist_imp_mid_2: Last, _, _;
    infinitive_aor: Ending, Ending, _;
    participle_aor: Ending, _, _;
    ),
    // 25, `λᾰμβᾰνω`: 4 words. Like 9 (`ἐξαιρεω`), except in 4 cells.
    pattern!("λᾰμβᾰνω", 4, like 9;
    aorist_subj_pass_1: Stem, _, _;
    aorist_subj_pass_2: Stem, _, _;
    aorist_subj_pass_3: Stem, _, _;
    aorist_imp_act_2: Stem, _, _;
    ),
    // 26, `ὀμνῡμῐ`: 4 words. Like 2 (`καθιζω`), except in 2 cells.
    pattern!("ὀμνῡμῐ", 4, like 2;
    infinitive_pres: NextToLast, _, _;
    participle_pres: Ending, _, _;
    ),
    // 27, `τελεω`: 4 words. Like 1 (`ποιεω`), except in 18 cells.
    pattern!("τελεω", 4, like 1;
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
    // 28, `προϊσχω`: 3 words. Like 0 (`ἐχω`), except in 4 cells.
    pattern!("προϊσχω", 3, like 0;
    imperfect_act_1: Stem, _, _;
    imperfect_act_2: Stem, _, _;
    imperfect_act_3: Stem, _, Stem;
    ),
    // 29, `δᾰμαζω`: 2 words. Like 27 (`τελεω`), except in 13 cells.
    pattern!("δᾰμαζω", 2, like 27;
    imperfect_act_1: _, _, Stem;
    imperfect_act_2: _, Stem, Stem;
    imperfect_mid_2: Stem, Stem, Stem;
    imperfect_mid_3: Stem, _, Stem;
    imperfect_pass_2: Stem, Stem, Stem;
    imperfect_pass_3: Stem, _, Stem;
    ),
    // 30, `περθω`: 2 words. Like 0 (`ἐχω`), except in 2 cells.
    pattern!("περθω", 2, like 0;
    aorist_imp_mid_2: Last, _, _;
    infinitive_aor: _, Ending, _;
    ),
    // 31, `ἀμῠσσω`: 2 words. Like 16 (`ἐπῐλᾰμβᾰνω`), except in 3 cells.
    pattern!("ἀμῠσσω", 2, like 16;
    aorist_subj_pass_1: Stem, _, _;
    aorist_subj_pass_2: Stem, _, _;
    aorist_subj_pass_3: Stem, _, _;
    ),
];

/// Every accent class: a pattern and the stem's accented syllable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccentClass {
    /// Pattern `ἐχω`, the stem accented on its next-to-last: 1,033 words.
    Ἐχω,
    /// Pattern `ποιεω`, the stem accented on its next-to-last: 279 words.
    Ποιεω,
    /// Pattern `καθιζω`, the stem accented on its next-to-last: 165 words.
    Καθιζω,
    /// Pattern `ἐχω`, the stem accented on its syllable 3 from the end: 140 words.
    Οἰομαι,
    /// Pattern `ὁρῐζω`, the stem accented on its next-to-last: 74 words.
    Ὁρῐζω,
    /// Pattern `ἀνῐστημῐ`, the stem accented on its syllable 3 from the end: 30 words.
    Ἀνῐστημῐ,
    /// Pattern `ζωννῡμῐ`, the stem accented on its syllable 3 from the end: 22 words.
    Ζωννῡμῐ,
    /// Pattern `τῐθημῐ`, the stem accented on its syllable 3 from the end: 20 words.
    Τῐθημῐ,
    /// Pattern `χεω`, the stem accented on its next-to-last: 19 words.
    Χεω,
    /// Pattern `προσερχομαι`, the stem accented on its syllable 4 from the end: 18 words.
    Ἡγεομαι,
    /// Pattern `ποιεω`, the stem accented on its syllable 3 from the end: 17 words.
    Χρᾰομαι,
    /// Pattern `προσερχομαι`, never on the stem: 17 words.
    Προσερχομαι,
    /// Pattern `ἐξαιρεω`, the stem accented on its next-to-last: 16 words.
    Ἐξαιρεω,
    /// Pattern `πᾰρεχω`, the stem accented on its next-to-last: 12 words.
    Πᾰρεχω,
    /// Pattern `πᾰρερχομαι`, the stem accented on its syllable 4 from the end: 11 words.
    Πᾰρερχομαι,
    /// Pattern `ὑπᾰγω`, the stem accented on its next-to-last: 11 words.
    Ὑπᾰγω,
    /// Pattern `ἐχω`, the stem accented on the lemma's last syllable: 10 words.
    Δει,
    /// Pattern `καθιζω`, the stem accented on its syllable 3 from the end: 10 words.
    Λογῐζομαι,
    /// Pattern `ὁρῐζω`, the stem accented on its syllable 3 from the end: 9 words.
    Ὑποκοριζομαι,
    /// Pattern `στειχω`, the stem accented on its next-to-last: 8 words.
    Στειχω,
    /// Pattern `ἐνῑημῐ`, the stem accented on its syllable 3 from the end: 8 words.
    Ἐνῑημῐ,
    /// Pattern `ἀφῑημῐ`, the stem accented on its syllable 3 from the end: 7 words.
    Ἀφῑημῐ,
    /// Pattern `ἐπῐλᾰμβᾰνω`, the stem accented on its next-to-last: 7 words.
    Ἐπῐλᾰμβᾰνω,
    /// Pattern `ἀναιρεω`, the stem accented on its next-to-last: 7 words.
    Ἀναιρεω,
    /// Pattern `στειχω`, the stem accented on its syllable 3 from the end: 6 words.
    Ἀρνυμαι,
    /// Pattern `πᾰσχω`, the stem accented on its next-to-last: 6 words.
    Πᾰσχω,
    /// Pattern `ὑπᾰρχω`, the stem accented on its next-to-last: 6 words.
    Ὑπᾰρχω,
    /// Pattern `ἐχω`, the stem accented on its syllable 4 from the end: 5 words.
    Θεᾰομαι,
    /// Pattern `πᾰσχω`, the stem accented on its syllable 3 from the end: 5 words.
    Ἐπορον,
    /// Pattern `δῐᾰφερω`, the stem accented on its next-to-last: 5 words.
    Δῐᾰφερω,
    /// Pattern `πᾰρερχομαι`, the stem accented on its next-to-last: 4 words.
    Ἐξευρῐσκω,
    /// Pattern `ἀφῑημῐ`, the stem accented on its next-to-last: 4 words.
    Κᾰτᾰβαινω,
    /// Pattern `ἀπειπον`, the stem accented on its next-to-last: 4 words.
    Ἀντιλεγω,
    /// Pattern `ἀπειπον`, the stem accented on its syllable 3 from the end: 4 words.
    Ἀπειπον,
    /// Pattern `εἰδον`, the stem accented on its next-to-last: 4 words.
    Εἰδον,
    /// Pattern `προβαλλω`, the stem accented on its next-to-last: 4 words.
    Προβαλλω,
    /// Pattern `λᾰμβᾰνω`, the stem accented on its next-to-last: 4 words.
    Λᾰμβᾰνω,
    /// Pattern `ὀμνῡμῐ`, the stem accented on its syllable 3 from the end: 4 words.
    Ὀμνῡμῐ,
    /// Pattern `τελεω`, the stem accented on its next-to-last: 4 words.
    Τελεω,
    /// Pattern `εἰδον`, never on the stem: 3 words.
    Προειπον,
    /// Pattern `ὠνεομαι`, the stem accented on its next-to-last: 3 words.
    Ἐπισκοπεω,
    /// Pattern `προϊσχω`, the stem accented on its next-to-last: 3 words.
    Προϊσχω,
    /// Pattern `ὁρῐζω`, the stem accented on its syllable 4 from the end: 2 words.
    Ἀγωνιζομαι,
    /// Pattern `χεω`, the stem accented on the lemma's last syllable: 2 words.
    Ὠχρῐω,
    /// Pattern `ὑπᾰρχω`, the stem accented on its syllable 3 from the end: 2 words.
    Ἐπορνῡμῐ,
    /// Pattern `ὠνεομαι`, the stem accented on its syllable 3 from the end: 2 words.
    Ὠνεομαι,
    /// Pattern `δᾰμαζω`, the stem accented on its next-to-last: 2 words.
    Δᾰμαζω,
    /// Pattern `περθω`, the stem accented on its next-to-last: 2 words.
    Περθω,
    /// Pattern `ἀμῠσσω`, the stem accented on its next-to-last: 2 words.
    Ἀμῠσσω,
    /// Pattern `ποιεω`, never on the stem: 1 words.
    Ἀλαομαι,
    /// Pattern `ποιεω`, the stem accented on the lemma's last syllable: 1 words.
    Ζω,
    /// Pattern `ποιεω`, the stem accented on its syllable 4 from the end: 1 words.
    Μωμᾰομαι,
    /// Pattern `καθιζω`, never on the stem: 1 words.
    Περνημῐ,
    /// Pattern `προσερχομαι`, the stem accented on the lemma's last syllable: 1 words.
    Φημῐ,
    /// Pattern `ἀνῐστημῐ`, never on the stem: 1 words.
    Χρη,
    /// Pattern `ζωννῡμῐ`, the stem accented on its next-to-last: 1 words.
    Κῐω,
    /// Pattern `χεω`, the stem accented on its syllable 3 from the end: 1 words.
    Δεξῐοομαι,
    /// Pattern `χεω`, the stem accented on its syllable 5 from the end: 1 words.
    Ἐξικνεομαι,
    /// Pattern `πᾰρερχομαι`, the stem accented on its syllable 3 from the end: 1 words.
    Ἐξῑημῐ,
    /// Pattern `στειχω`, the stem accented on its syllable 4 from the end: 1 words.
    Αἰσθᾰνομαι,
    /// Pattern `ὑπᾰγω`, the stem accented on its syllable 3 from the end: 1 words.
    Ἀνειρομαι,
    /// Pattern `ὑπᾰγω`, the stem accented on its syllable 4 from the end: 1 words.
    Ἀνερχομαι,
    /// Pattern `πᾰσχω`, the stem accented on its syllable 4 from the end: 1 words.
    Ἀνιημῐ,
    /// Pattern `ἐπῐλᾰμβᾰνω`, the stem accented on the lemma's last syllable: 1 words.
    Δρᾱω,
    /// Pattern `ἐπῐλᾰμβᾰνω`, the stem accented on its syllable 3 from the end: 1 words.
    Ἀρᾰρῐσκω,
    /// Pattern `ἀναιρεω`, the stem accented on its syllable 3 from the end: 1 words.
    Ἐφῑημῐ,
}

impl AccentClass {
    /// Every class, in order.
    pub const ALL: [AccentClass; 66] = [
        AccentClass::Ἐχω,
        AccentClass::Ποιεω,
        AccentClass::Καθιζω,
        AccentClass::Οἰομαι,
        AccentClass::Ὁρῐζω,
        AccentClass::Ἀνῐστημῐ,
        AccentClass::Ζωννῡμῐ,
        AccentClass::Τῐθημῐ,
        AccentClass::Χεω,
        AccentClass::Ἡγεομαι,
        AccentClass::Χρᾰομαι,
        AccentClass::Προσερχομαι,
        AccentClass::Ἐξαιρεω,
        AccentClass::Πᾰρεχω,
        AccentClass::Πᾰρερχομαι,
        AccentClass::Ὑπᾰγω,
        AccentClass::Δει,
        AccentClass::Λογῐζομαι,
        AccentClass::Ὑποκοριζομαι,
        AccentClass::Στειχω,
        AccentClass::Ἐνῑημῐ,
        AccentClass::Ἀφῑημῐ,
        AccentClass::Ἐπῐλᾰμβᾰνω,
        AccentClass::Ἀναιρεω,
        AccentClass::Ἀρνυμαι,
        AccentClass::Πᾰσχω,
        AccentClass::Ὑπᾰρχω,
        AccentClass::Θεᾰομαι,
        AccentClass::Ἐπορον,
        AccentClass::Δῐᾰφερω,
        AccentClass::Ἐξευρῐσκω,
        AccentClass::Κᾰτᾰβαινω,
        AccentClass::Ἀντιλεγω,
        AccentClass::Ἀπειπον,
        AccentClass::Εἰδον,
        AccentClass::Προβαλλω,
        AccentClass::Λᾰμβᾰνω,
        AccentClass::Ὀμνῡμῐ,
        AccentClass::Τελεω,
        AccentClass::Προειπον,
        AccentClass::Ἐπισκοπεω,
        AccentClass::Προϊσχω,
        AccentClass::Ἀγωνιζομαι,
        AccentClass::Ὠχρῐω,
        AccentClass::Ἐπορνῡμῐ,
        AccentClass::Ὠνεομαι,
        AccentClass::Δᾰμαζω,
        AccentClass::Περθω,
        AccentClass::Ἀμῠσσω,
        AccentClass::Ἀλαομαι,
        AccentClass::Ζω,
        AccentClass::Μωμᾰομαι,
        AccentClass::Περνημῐ,
        AccentClass::Φημῐ,
        AccentClass::Χρη,
        AccentClass::Κῐω,
        AccentClass::Δεξῐοομαι,
        AccentClass::Ἐξικνεομαι,
        AccentClass::Ἐξῑημῐ,
        AccentClass::Αἰσθᾰνομαι,
        AccentClass::Ἀνειρομαι,
        AccentClass::Ἀνερχομαι,
        AccentClass::Ἀνιημῐ,
        AccentClass::Δρᾱω,
        AccentClass::Ἀρᾰρῐσκω,
        AccentClass::Ἐφῑημῐ,
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
pub static CLASSES: [StressClass<'static>; 66] = [
    StressClass {
        name: Cow::Borrowed("ἐχω"),
        pattern: 0, // ἐχω
        vowel: Some(1),
        words: 1033,
    },
    StressClass {
        name: Cow::Borrowed("ποιεω"),
        pattern: 1, // ποιεω
        vowel: Some(1),
        words: 279,
    },
    StressClass {
        name: Cow::Borrowed("καθιζω"),
        pattern: 2, // καθιζω
        vowel: Some(1),
        words: 165,
    },
    StressClass {
        name: Cow::Borrowed("οἰομαι"),
        pattern: 0, // ἐχω
        vowel: Some(2),
        words: 140,
    },
    StressClass {
        name: Cow::Borrowed("ὁρῐζω"),
        pattern: 3, // ὁρῐζω
        vowel: Some(1),
        words: 74,
    },
    StressClass {
        name: Cow::Borrowed("ἀνῐστημῐ"),
        pattern: 5, // ἀνῐστημῐ
        vowel: Some(2),
        words: 30,
    },
    StressClass {
        name: Cow::Borrowed("ζωννῡμῐ"),
        pattern: 6, // ζωννῡμῐ
        vowel: Some(2),
        words: 22,
    },
    StressClass {
        name: Cow::Borrowed("τῐθημῐ"),
        pattern: 8, // τῐθημῐ
        vowel: Some(2),
        words: 20,
    },
    StressClass {
        name: Cow::Borrowed("χεω"),
        pattern: 7, // χεω
        vowel: Some(1),
        words: 19,
    },
    StressClass {
        name: Cow::Borrowed("ἡγεομαι"),
        pattern: 4, // προσερχομαι
        vowel: Some(3),
        words: 18,
    },
    StressClass {
        name: Cow::Borrowed("χρᾰομαι"),
        pattern: 1, // ποιεω
        vowel: Some(2),
        words: 17,
    },
    StressClass {
        name: Cow::Borrowed("προσερχομαι"),
        pattern: 4, // προσερχομαι
        vowel: None,
        words: 17,
    },
    StressClass {
        name: Cow::Borrowed("ἐξαιρεω"),
        pattern: 9, // ἐξαιρεω
        vowel: Some(1),
        words: 16,
    },
    StressClass {
        name: Cow::Borrowed("πᾰρεχω"),
        pattern: 14, // πᾰρεχω
        vowel: Some(1),
        words: 12,
    },
    StressClass {
        name: Cow::Borrowed("πᾰρερχομαι"),
        pattern: 10, // πᾰρερχομαι
        vowel: Some(3),
        words: 11,
    },
    StressClass {
        name: Cow::Borrowed("ὑπᾰγω"),
        pattern: 12, // ὑπᾰγω
        vowel: Some(1),
        words: 11,
    },
    StressClass {
        name: Cow::Borrowed("δει"),
        pattern: 0, // ἐχω
        vowel: Some(0),
        words: 10,
    },
    StressClass {
        name: Cow::Borrowed("λογῐζομαι"),
        pattern: 2, // καθιζω
        vowel: Some(2),
        words: 10,
    },
    StressClass {
        name: Cow::Borrowed("ὑποκοριζομαι"),
        pattern: 3, // ὁρῐζω
        vowel: Some(2),
        words: 9,
    },
    StressClass {
        name: Cow::Borrowed("στειχω"),
        pattern: 11, // στειχω
        vowel: Some(1),
        words: 8,
    },
    StressClass {
        name: Cow::Borrowed("ἐνῑημῐ"),
        pattern: 18, // ἐνῑημῐ
        vowel: Some(2),
        words: 8,
    },
    StressClass {
        name: Cow::Borrowed("ἀφῑημῐ"),
        pattern: 15, // ἀφῑημῐ
        vowel: Some(2),
        words: 7,
    },
    StressClass {
        name: Cow::Borrowed("ἐπῐλᾰμβᾰνω"),
        pattern: 16, // ἐπῐλᾰμβᾰνω
        vowel: Some(1),
        words: 7,
    },
    StressClass {
        name: Cow::Borrowed("ἀναιρεω"),
        pattern: 17, // ἀναιρεω
        vowel: Some(1),
        words: 7,
    },
    StressClass {
        name: Cow::Borrowed("ἀρνυμαι"),
        pattern: 11, // στειχω
        vowel: Some(2),
        words: 6,
    },
    StressClass {
        name: Cow::Borrowed("πᾰσχω"),
        pattern: 13, // πᾰσχω
        vowel: Some(1),
        words: 6,
    },
    StressClass {
        name: Cow::Borrowed("ὑπᾰρχω"),
        pattern: 19, // ὑπᾰρχω
        vowel: Some(1),
        words: 6,
    },
    StressClass {
        name: Cow::Borrowed("θεᾰομαι"),
        pattern: 0, // ἐχω
        vowel: Some(3),
        words: 5,
    },
    StressClass {
        name: Cow::Borrowed("ἐπορον"),
        pattern: 13, // πᾰσχω
        vowel: Some(2),
        words: 5,
    },
    StressClass {
        name: Cow::Borrowed("δῐᾰφερω"),
        pattern: 23, // δῐᾰφερω
        vowel: Some(1),
        words: 5,
    },
    StressClass {
        name: Cow::Borrowed("ἐξευρῐσκω"),
        pattern: 10, // πᾰρερχομαι
        vowel: Some(1),
        words: 4,
    },
    StressClass {
        name: Cow::Borrowed("κᾰτᾰβαινω"),
        pattern: 15, // ἀφῑημῐ
        vowel: Some(1),
        words: 4,
    },
    StressClass {
        name: Cow::Borrowed("ἀντιλεγω"),
        pattern: 20, // ἀπειπον
        vowel: Some(1),
        words: 4,
    },
    StressClass {
        name: Cow::Borrowed("ἀπειπον"),
        pattern: 20, // ἀπειπον
        vowel: Some(2),
        words: 4,
    },
    StressClass {
        name: Cow::Borrowed("εἰδον"),
        pattern: 21, // εἰδον
        vowel: Some(1),
        words: 4,
    },
    StressClass {
        name: Cow::Borrowed("προβαλλω"),
        pattern: 24, // προβαλλω
        vowel: Some(1),
        words: 4,
    },
    StressClass {
        name: Cow::Borrowed("λᾰμβᾰνω"),
        pattern: 25, // λᾰμβᾰνω
        vowel: Some(1),
        words: 4,
    },
    StressClass {
        name: Cow::Borrowed("ὀμνῡμῐ"),
        pattern: 26, // ὀμνῡμῐ
        vowel: Some(2),
        words: 4,
    },
    StressClass {
        name: Cow::Borrowed("τελεω"),
        pattern: 27, // τελεω
        vowel: Some(1),
        words: 4,
    },
    StressClass {
        name: Cow::Borrowed("προειπον"),
        pattern: 21, // εἰδον
        vowel: None,
        words: 3,
    },
    StressClass {
        name: Cow::Borrowed("ἐπισκοπεω"),
        pattern: 22, // ὠνεομαι
        vowel: Some(1),
        words: 3,
    },
    StressClass {
        name: Cow::Borrowed("προϊσχω"),
        pattern: 28, // προϊσχω
        vowel: Some(1),
        words: 3,
    },
    StressClass {
        name: Cow::Borrowed("ἀγωνιζομαι"),
        pattern: 3, // ὁρῐζω
        vowel: Some(3),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("ὠχρῐω"),
        pattern: 7, // χεω
        vowel: Some(0),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("ἐπορνῡμῐ"),
        pattern: 19, // ὑπᾰρχω
        vowel: Some(2),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("ὠνεομαι"),
        pattern: 22, // ὠνεομαι
        vowel: Some(2),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("δᾰμαζω"),
        pattern: 29, // δᾰμαζω
        vowel: Some(1),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("περθω"),
        pattern: 30, // περθω
        vowel: Some(1),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("ἀμῠσσω"),
        pattern: 31, // ἀμῠσσω
        vowel: Some(1),
        words: 2,
    },
    StressClass {
        name: Cow::Borrowed("ἀλαομαι"),
        pattern: 1, // ποιεω
        vowel: None,
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ζω"),
        pattern: 1, // ποιεω
        vowel: Some(0),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("μωμᾰομαι"),
        pattern: 1, // ποιεω
        vowel: Some(3),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("περνημῐ"),
        pattern: 2, // καθιζω
        vowel: None,
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("φημῐ"),
        pattern: 4, // προσερχομαι
        vowel: Some(0),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("χρη"),
        pattern: 5, // ἀνῐστημῐ
        vowel: None,
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("κῐω"),
        pattern: 6, // ζωννῡμῐ
        vowel: Some(1),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("δεξῐοομαι"),
        pattern: 7, // χεω
        vowel: Some(2),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἐξικνεομαι"),
        pattern: 7, // χεω
        vowel: Some(4),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἐξῑημῐ"),
        pattern: 10, // πᾰρερχομαι
        vowel: Some(2),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("αἰσθᾰνομαι"),
        pattern: 11, // στειχω
        vowel: Some(3),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἀνειρομαι"),
        pattern: 12, // ὑπᾰγω
        vowel: Some(2),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἀνερχομαι"),
        pattern: 12, // ὑπᾰγω
        vowel: Some(3),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἀνιημῐ"),
        pattern: 13, // πᾰσχω
        vowel: Some(3),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("δρᾱω"),
        pattern: 16, // ἐπῐλᾰμβᾰνω
        vowel: Some(0),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἀρᾰρῐσκω"),
        pattern: 16, // ἐπῐλᾰμβᾰνω
        vowel: Some(2),
        words: 1,
    },
    StressClass {
        name: Cow::Borrowed("ἐφῑημῐ"),
        pattern: 17, // ἀναιρεω
        vowel: Some(2),
        words: 1,
    },
];

use AccentClass::*;

/// Which class a word is of: 64 rules and 1,285 listed words.
pub const CLASS: Choice<'static, AccentClass> = flexion::choice! {
    rules {
        "" => [Ἐχω 1033, Ποιεω 279, Καθιζω 165, Οἰομαι 140, Ὁρῐζω 74, Ἀνῐστημῐ 30, Ζωννῡμῐ 22, Τῐθημῐ 20, Χεω 19, Ἡγεομαι 18, Χρᾰομαι 17, Προσερχομαι 17, Ἐξαιρεω 16, Πᾰρεχω 12, Πᾰρερχομαι 11, Ὑπᾰγω 11, Δει 10, Λογῐζομαι 10, Ὑποκοριζομαι 9, Στειχω 8, Ἐνῑημῐ 8, Ἀφῑημῐ 7, Ἐπῐλᾰμβᾰνω 7, Ἀναιρεω 7, Ἀρνυμαι 6, Πᾰσχω 6, Ὑπᾰρχω 6, Θεᾰομαι 5, Ἐπορον 5, Δῐᾰφερω 5, Ἐξευρῐσκω 4, Κᾰτᾰβαινω 4, Ἀντιλεγω 4, Ἀπειπον 4, Εἰδον 4, Προβαλλω 4, Λᾰμβᾰνω 4, Ὀμνῡμῐ 4, Τελεω 4, Προειπον 3, Ἐπισκοπεω 3, Προϊσχω 3, Ἀγωνιζομαι 2, Ὠχρῐω 2, Ἐπορνῡμῐ 2, Ὠνεομαι 2, Δᾰμαζω 2, Περθω 2, Ἀμῠσσω 2, Ἀλαομαι 1, Ζω 1, Μωμᾰομαι 1, Περνημῐ 1, Φημῐ 1, Χρη 1, Κῐω 1, Δεξῐοομαι 1, Ἐξικνεομαι 1, Ἐξῑημῐ 1, Αἰσθᾰνομαι 1, Ἀνειρομαι 1, Ἀνερχομαι 1, Ἀνιημῐ 1, Δρᾱω 1, Ἀρᾰρῐσκω 1, Ἐφῑημῐ 1],
        "α" => [Ἐνῑημῐ 2, Ἐχω 1, Θεᾰομαι 1],
        "ε" => [Οἰομαι 5],
        "ι" => [Οἰομαι 111, Ἀνῐστημῐ 30, Ζωννῡμῐ 22, Τῐθημῐ 20, Ἡγεομαι 18, Χρᾰομαι 16, Προσερχομαι 16, Πᾰρερχομαι 11, Λογῐζομαι 10, Ὑποκοριζομαι 9, Ἐχω 8, Ἀφῑημῐ 7, Ἐνῑημῐ 6, Ἀρνυμαι 6, Ὀμνῡμῐ 4, Δει 3, Θεᾰομαι 3, Ἐπορον 2, Ἀγωνιζομαι 2, Ἐπορνῡμῐ 2, Ὠνεομαι 2, Ποιεω 1, Ἐξαιρεω 1, Ἀλαομαι 1, Μωμᾰομαι 1, Περνημῐ 1, Φημῐ 1, Δεξῐοομαι 1, Ἐξικνεομαι 1, Ἐξῑημῐ 1, Αἰσθᾰνομαι 1, Ἀνειρομαι 1, Ἀνερχομαι 1, Ἀνιημῐ 1, Ἐφῑημῐ 1],
        "αι" => [Οἰομαι 104, Ἡγεομαι 18, Προσερχομαι 16, Χρᾰομαι 12, Πᾰρερχομαι 11, Λογῐζομαι 10, Ὑποκοριζομαι 8, Ἀνῐστημῐ 6, Ἀρνυμαι 6, Ἐχω 5, Τῐθημῐ 5, Θεᾰομαι 3, Ἐπορον 2, Ἀγωνιζομαι 2, Ὠνεομαι 2, Ἐξαιρεω 1, Ἀφῑημῐ 1, Ἀλαομαι 1, Μωμᾰομαι 1, Δεξῐοομαι 1, Ἐξικνεομαι 1, Αἰσθᾰνομαι 1, Ἀνειρομαι 1, Ἀνερχομαι 1],
        "αμαι" => [Ἀνῐστημῐ 6, Οἰομαι 1],
        "ιμαι" => [Τῐθημῐ 4, Ἐχω 1, Οἰομαι 1],
        "δομαι" => [Οἰομαι 10],
        "εομαι" => [Ἡγεομαι 12, Προσερχομαι 6, Χρᾰομαι 5, Οἰομαι 3, Ὠνεομαι 1, Ἐξικνεομαι 1],
        "ζομαι" => [Οἰομαι 20, Λογῐζομαι 5, Ἀρνυμαι 2, Ἀγωνιζομαι 2, Ὑποκοριζομαι 1],
        "λομαι" => [Οἰομαι 4, Ὑποκοριζομαι 3],
        "ινομαι" => [Οἰομαι 5, Ὑποκοριζομαι 3],
        "οομαι" => [Προσερχομαι 4, Οἰομαι 3, Χρᾰομαι 3, Δεξῐοομαι 1],
        "ερομαι" => [Ἀρνυμαι 3, Οἰομαι 1],
        "χομαι" => [Πᾰρερχομαι 11, Οἰομαι 9, Προσερχομαι 2, Λογῐζομαι 2, Ἐπορον 1, Ἀνερχομαι 1],
        "μι" => [Ἀνῐστημῐ 24, Ζωννῡμῐ 22, Τῐθημῐ 15, Ἐνῑημῐ 6, Ἀφῑημῐ 6, Οἰομαι 5, Χρᾰομαι 4, Ὀμνῡμῐ 4, Δει 2, Ἐπορνῡμῐ 2, Ποιεω 1, Ὑποκοριζομαι 1, Περνημῐ 1, Φημῐ 1, Ἐξῑημῐ 1, Ἀνιημῐ 1, Ἐφῑημῐ 1],
        "ημι" => [Ἀνῐστημῐ 24, Τῐθημῐ 10, Ἀφῑημῐ 6, Ποιεω 1, Ζωννῡμῐ 1, Δει 1, Ἐνῑημῐ 1, Περνημῐ 1, Φημῐ 1, Ἐξῑημῐ 1, Ἀνιημῐ 1, Ἐφῑημῐ 1],
        "θημι" => [Τῐθημῐ 9],
        "τημι" => [Ἀνῐστημῐ 19, Ζωννῡμῐ 1],
        "ιμι" => [Τῐθημῐ 5, Χρᾰομαι 4, Ἐνῑημῐ 3, Οἰομαι 1, Δει 1],
        "υμι" => [Ζωννῡμῐ 21, Ὀμνῡμῐ 4, Ἐνῑημῐ 2, Ἐπορνῡμῐ 2, Οἰομαι 1, Ὑποκοριζομαι 1],
        "ωμι" => [Οἰομαι 3],
        "οι" => [Ἐχω 2],
        "ν" => [Οἰομαι 16, Ἀπειπον 4, Ἐπορον 3, Εἰδον 3, Προειπον 3, Πᾰσχω 2, Στειχω 1],
        "πον" => [Ἀπειπον 4, Προειπον 3, Εἰδον 1],
        "ω" => [Ἐχω 1020, Ποιεω 278, Καθιζω 165, Ὁρῐζω 74, Χεω 19, Ἐξαιρεω 15, Πᾰρεχω 12, Ὑπᾰγω 11, Δει 7, Στειχω 7, Ἐπῐλᾰμβᾰνω 7, Ἀναιρεω 7, Οἰομαι 6, Ὑπᾰρχω 6, Δῐᾰφερω 5, Πᾰσχω 4, Ἐξευρῐσκω 4, Κᾰτᾰβαινω 4, Ἀντιλεγω 4, Προβαλλω 4, Λᾰμβᾰνω 4, Τελεω 4, Ἐπισκοπεω 3, Προϊσχω 3, Ὠχρῐω 2, Δᾰμαζω 2, Περθω 2, Ἀμῠσσω 2, Χρᾰομαι 1, Προσερχομαι 1, Εἰδον 1, Ζω 1, Κῐω 1, Δρᾱω 1, Ἀρᾰρῐσκω 1],
        "αω" => [Ἐχω 59, Ποιεω 58, Ἀναιρεω 2, Λᾰμβᾰνω 2, Οἰομαι 1, Χεω 1, Χρᾰομαι 1, Ἐξαιρεω 1, Ἐξευρῐσκω 1, Ἐπισκοπεω 1, Ὠχρῐω 1, Δρᾱω 1],
        "οραω" => [Ἀναιρεω 2, Λᾰμβᾰνω 2, Χεω 1, Ἐξαιρεω 1, Ἐξευρῐσκω 1, Ἐπισκοπεω 1],
        "βω" => [Ἐχω 11],
        "γω" => [Ἐχω 32, Ὑπᾰγω 8, Πᾰρεχω 3, Στειχω 1, Ἀναιρεω 1, Πᾰσχω 1, Ἐξευρῐσκω 1, Ἀντιλεγω 1],
        "αγω" => [Ὑπᾰγω 7, Πᾰρεχω 3, Ἐχω 2, Ἀναιρεω 1],
        "δω" => [Ἐχω 19, Ἐπῐλᾰμβᾰνω 1],
        "εω" => [Ἐχω 177, Ποιεω 169, Χεω 14, Καθιζω 5, Τελεω 4, Ὁρῐζω 3, Ἀναιρεω 3, Ἐξαιρεω 2, Ἀντιλεγω 2, Ἐπισκοπεω 2, Οἰομαι 1],
        "γεω" => [Ποιεω 27, Ἐχω 13],
        "ικεω" => [Ἐχω 11, Ποιεω 1],
        "πεω" => [Ποιεω 5, Ἐχω 2, Χεω 2, Ἐπισκοπεω 2],
        "ιρεω" => [Ἐχω 3, Ἀναιρεω 3, Ὁρῐζω 2, Ἐξαιρεω 2, Ἀντιλεγω 1],
        "τεω" => [Ἐχω 24, Ποιεω 13],
        "ζω" => [Ἐχω 150, Καθιζω 115, Ὁρῐζω 8, Δει 2, Δᾰμαζω 1, Ζω 1],
        "αζω" => [Ἐχω 94, Ὁρῐζω 4, Δει 1, Δᾰμαζω 1],
        "ιζω" => [Καθιζω 114, Ἐχω 39, Ὁρῐζω 3],
        "υζω" => [Ἐχω 8],
        "ειω" => [Ἐχω 9],
        "κω" => [Ἐχω 38, Ὁρῐζω 6, Ἐπῐλᾰμβᾰνω 5, Ὑπᾰρχω 3, Ἐξευρῐσκω 2, Δει 1, Λᾰμβᾰνω 1, Προϊσχω 1, Ἀρᾰρῐσκω 1],
        "ηκω" => [Ἐχω 6, Ὑπᾰρχω 3],
        "λω" => [Ἐχω 22, Καθιζω 12, Προβαλλω 4],
        "βαλλω" => [Προβαλλω 4],
        "ελλω" => [Καθιζω 8, Ἐχω 3],
        "νω" => [Ἐχω 58, Ὁρῐζω 51, Καθιζω 15, Ἐξαιρεω 12, Κᾰτᾰβαινω 4, Ὑπᾰγω 2, Δει 1, Ἐπῐλᾰμβᾰνω 1, Ἀναιρεω 1, Δῐᾰφερω 1, Λᾰμβᾰνω 1, Δᾰμαζω 1],
        "ανω" => [Ἐχω 22, Ἐξαιρεω 12, Ὑπᾰγω 1, Ἐπῐλᾰμβᾰνω 1, Ἀναιρεω 1, Δῐᾰφερω 1, Λᾰμβᾰνω 1],
        "ινω" => [Ὁρῐζω 28, Ἐχω 20, Καθιζω 8, Κᾰτᾰβαινω 4, Δει 1],
        "βαινω" => [Κᾰτᾰβαινω 4, Ἐχω 3],
        "οω" => [Ποιεω 51, Ἐχω 43, Χεω 4, Οἰομαι 1, Προσερχομαι 1, Δει 1],
        "τοω" => [Ποιεω 6],
        "πω" => [Ἐχω 22, Στειχω 3, Εἰδον 1, Προϊσχω 1],
        "ιπω" => [Στειχω 3, Ἐχω 1],
        "ρω" => [Ἐχω 28, Καθιζω 15, Δῐᾰφερω 4, Ὁρῐζω 1],
        "ερω" => [Ἐχω 9, Δῐᾰφερω 4, Καθιζω 1],
        "σω" => [Ἐχω 55, Ἀμῠσσω 2, Στειχω 1, Περθω 1],
        "τω" => [Ἐχω 57, Καθιζω 1, Οἰομαι 1, Ὁρῐζω 1, Ὑπᾰγω 1, Ὑπᾰρχω 1],
        "υω" => [Ἐχω 157, Οἰομαι 1, Πᾰσχω 1, Ἀντιλεγω 1],
        "φω" => [Ἐχω 17],
        "χω" => [Ἐχω 20, Πᾰρεχω 9, Ὁρῐζω 3, Πᾰσχω 2, Ὑπᾰρχω 2, Στειχω 1, Προϊσχω 1],
        "εχω" => [Πᾰρεχω 9, Ἐχω 3, Ὁρῐζω 3, Πᾰσχω 1],
    }
    listed {
        "ααω" => Οἰομαι,
        "αγαλλιαω" => Ἐχω,
        "αγαμαι" => Ἀνῐστημῐ,
        "αγαπαω" => Ἐχω,
        "αγγελλω" => Καθιζω,
        "αγειρω" => Καθιζω,
        "αγκυλοω" => Ἐχω,
        "αγνευω" => Ἐχω,
        "αγνιζω" => Καθιζω,
        "αγνοεω" => Ἐχω,
        "αγοραζω" => Ὁρῐζω,
        "αγραυλεω" => Ἐχω,
        "αγρεω" => Ἐχω,
        "αγω" => Ὑπᾰγω,
        "αγωνιζομαι" => Ἀγωνιζομαι,
        "αδηλοω" => Ἐχω,
        "αδικεω" => Ἐχω,
        "αδω" => Ἐχω,
        "αειδω" => Ἐχω,
        "αειρω" => Ἐχω,
        "αεξω" => Ἐχω,
        "αθανατιζω" => Ἐχω,
        "αθροιζω" => Καθιζω,
        "αιδεομαι" => Ἡγεομαι,
        "αιδομαι" => Οἰομαι,
        "αιρετιζω" => Καθιζω,
        "αιρεω" => Ἐχω,
        "αιρω" => Ὁρῐζω,
        "αισθανομαι" => Αἰσθᾰνομαι,
        "αισσω" => Ἐχω,
        "αισχοω" => Ἐχω,
        "αισχρουργεω" => Ἐχω,
        "αισχυνω" => Ὁρῐζω,
        "αιτεω" => Ἐχω,
        "αιτιαομαι" => Οἰομαι,
        "αιτιζω" => Ἐχω,
        "αιχμαζω" => Ἐχω,
        "αιχμαλοω" => Ἐχω,
        "ακοντιζω" => Ἐχω,
        "ακουω" => Ἐχω,
        "ακριβοω" => Ἐχω,
        "ακροαομαι" => Ἡγεομαι,
        "αλαομαι" => Ἀλαομαι,
        "αλγεω" => Ἐχω,
        "αλειφω" => Ἐχω,
        "αλεξω" => Ἐχω,
        "αλεομαι" => Προσερχομαι,
        "αλεω" => Ἐχω,
        "αλθαινω" => Ὁρῐζω,
        "αλιευω" => Ἐχω,
        "αλιζω" => Ἐχω,
        "αλινω" => Ἐχω,
        "αλισκομαι" => Ἀφῑημῐ,
        "αλλοιοω" => Ἐχω,
        "αλλομαι" => Ὑποκοριζομαι,
        "αλλοτριοω" => Ἐχω,
        "αλφανω" => Ἐχω,
        "αμαλδυνω" => Ὁρῐζω,
        "αμαρτανω" => Ἐξαιρεω,
        "αμαυροω" => Ἐχω,
        "αμβλυνω" => Ὁρῐζω,
        "αμειβω" => Ἐχω,
        "αμερδω" => Ἐπῐλᾰμβᾰνω,
        "αμηχανοποιεομαι" => Προσερχομαι,
        "αμυνω" => Ὁρῐζω,
        "αμυσσω" => Ἀμῠσσω,
        "αμφικαλυπτω" => Ἐχω,
        "αμφιστημι" => Ἀνῐστημῐ,
        "αναβλαστανω" => Ἐξαιρεω,
        "αναβοαω" => Ἐχω,
        "αναγκαζω" => Ἐχω,
        "αναγω" => Πᾰρεχω,
        "αναζω" => Δει,
        "αναινομαι" => Οἰομαι,
        "αναιρεω" => Ἀναιρεω,
        "αναισσω" => Ἐχω,
        "ανακαλεω" => Ἐχω,
        "αναλαμβανω" => Ἐξαιρεω,
        "αναλισκω" => Ἐχω,
        "αναποδιζω" => Καθιζω,
        "ανατρεχω" => Ὁρῐζω,
        "αναχρονιζομαι" => Λογῐζομαι,
        "αναψυχω" => Ἐχω,
        "ανδανω" => Ἐχω,
        "ανεγειρω" => Καθιζω,
        "ανειρομαι" => Ἀνειρομαι,
        "ανερχομαι" => Ἀνερχομαι,
        "ανεχω" => Πᾰρεχω,
        "ανηκω" => Ὑπᾰρχω,
        "ανθιστημι" => Ἀνῐστημῐ,
        "ανιημι" => Ἀνιημῐ,
        "ανιστημι" => Ἀνῐστημῐ,
        "ανοιγνυμι" => Ζωννῡμῐ,
        "ανοιγω" => Ἐχω,
        "ανορουω" => Ἐχω,
        "ανταγωνιζομαι" => Ἀγωνιζομαι,
        "ανταιρω" => Καθιζω,
        "ανταναγω" => Πᾰρεχω,
        "ανταναλυω" => Ἐχω,
        "ανταποδιδωμι" => Οἰομαι,
        "ανταποκρινομαι" => Ὑποκοριζομαι,
        "αντεγειρω" => Καθιζω,
        "αντεπιβουλευω" => Ἐχω,
        "αντεπιπλεω" => Ἐχω,
        "αντεραω" => Ἐχω,
        "αντεχω" => Πᾰρεχω,
        "αντιαω" => Ἐχω,
        "αντιλεγω" => Ἀντιλεγω,
        "αντιπαρατασσομαι" => Θεᾰομαι,
        "ανυω" => Ἐχω,
        "αξιοω" => Ἐχω,
        "απαγχοναω" => Ἐχω,
        "απαγχονεω" => Ἐχω,
        "απαγχονιζω" => Καθιζω,
        "απαγχω" => Ἐχω,
        "απαγω" => Ὑπᾰγω,
        "απακριβοω" => Ἐχω,
        "απαλθαινομαι" => Οἰομαι,
        "απαμειβομαι" => Οἰομαι,
        "απανιστημι" => Ἀνῐστημῐ,
        "απανταω" => Ἐχω,
        "απαξιοω" => Ἐχω,
        "απαρεσκω" => Ἐχω,
        "απαρνεομαι" => Ἡγεομαι,
        "απαυραω" => Χρᾰομαι,
        "απειμι" => Χρᾰομαι,
        "απειπον" => Ἀπειπον,
        "απελευθεροω" => Ἐχω,
        "απεραω" => Ἐχω,
        "απερχομαι" => Πᾰρερχομαι,
        "απευχομαι" => Οἰομαι,
        "απεχθανομαι" => Λογῐζομαι,
        "απεχω" => Πᾰρεχω,
        "απιχθυοομαι" => Προσερχομαι,
        "απλοω" => Ἐχω,
        "αποδυω" => Ἐχω,
        "αποζω" => Δει,
        "αποθραυω" => Ἐχω,
        "αποκαλυπτω" => Ἐχω,
        "απολαμβανω" => Ἀναιρεω,
        "απολεγω" => Ἐξευρῐσκω,
        "απολυτροω" => Ἐχω,
        "αποπερδομαι" => Οἰομαι,
        "απορεω" => Ἐχω,
        "απτω" => Ἐχω,
        "απωθεω" => Ποιεω,
        "αραομαι" => Ἡγεομαι,
        "αραρισκω" => Ἀρᾰρῐσκω,
        "αργεω" => Ἐχω,
        "αρδω" => Ἐχω,
        "αρεσκω" => Ἐχω,
        "αριθμεω" => Ἐχω,
        "αρκεω" => Ἐχω,
        "αρμοζω" => Ἐχω,
        "αρνυμαι" => Ἀρνυμαι,
        "αροω" => Ἐχω,
        "αρπαζω" => Ἐχω,
        "αρρητουργεω" => Ἐχω,
        "αρχαιζω" => Καθιζω,
        "αρχω" => Ἐχω,
        "ασαω" => Ἐχω,
        "ασπαζομαι" => Οἰομαι,
        "αστραπτω" => Ἐχω,
        "ατιταλλω" => Ἐχω,
        "αττικιζω" => Καθιζω,
        "αυδαω" => Ἐχω,
        "αυθεντεω" => Ἐχω,
        "αυλεω" => Ἐχω,
        "αυξανω" => Ἐχω,
        "αυξω" => Ἐχω,
        "αυω" => Ἐχω,
        "αφαιρεω" => Ἀναιρεω,
        "αφαμαρτανω" => Ἐξαιρεω,
        "αφιημι" => Ἀφῑημῐ,
        "αφικνεομαι" => Ἡγεομαι,
        "αφοριζω" => Καθιζω,
        "αφραινω" => Ἐχω,
        "αφυπνιζω" => Ἐχω,
        "αφυσσω" => Ἀμῠσσω,
        "αχθομαι" => Οἰομαι,
        "βαθυνω" => Ὁρῐζω,
        "βαινω" => Ἐχω,
        "βαλλω" => Προβαλλω,
        "βαμβαινω" => Ἐχω,
        "βαπτιζω" => Ἐχω,
        "βαρυνω" => Ὁρῐζω,
        "βασταζω" => Ἐχω,
        "βιβαζω" => Ἐχω,
        "βιβρωσκω" => Ἐπῐλᾰμβᾰνω,
        "βιοω" => Ποιεω,
        "βλαπτω" => Ἐχω,
        "βλαστανω" => Ἐχω,
        "βλασφημεω" => Ποιεω,
        "βλεμεαινω" => Ἐχω,
        "βλεπω" => Ἐχω,
        "βλιττω" => Ἐχω,
        "βλυζω" => Ἐχω,
        "βλωσκω" => Ὁρῐζω,
        "βοαω" => Ποιεω,
        "βοηθεω" => Ποιεω,
        "βραδυνω" => Ὁρῐζω,
        "βρασσω" => Ἐχω,
        "βραχυνω" => Ὁρῐζω,
        "βρεχω" => Ἐχω,
        "βρυαζω" => Ἐχω,
        "βρυκω" => Ἐχω,
        "βρυχω" => Ἐχω,
        "γαμεω" => Τελεω,
        "γαμιζω" => Ἐχω,
        "γανυμαι" => Ἐχω,
        "γεγενημεθα" => Θεᾰομαι,
        "γεγενησθω" => Οἰομαι,
        "γεγενησο" => Θεᾰομαι,
        "γελαω" => Ποιεω,
        "γενειαω" => Ποιεω,
        "γενναω" => Ποιεω,
        "γηθεω" => Ποιεω,
        "γηρασκω" => Ἐχω,
        "γιγνομαι" => Ἐχω,
        "γιγνωσκω" => Ὁρῐζω,
        "γινωσκω" => Δει,
        "γλυφω" => Ἐχω,
        "γναμπτω" => Ἐχω,
        "γοαω" => Ἐχω,
        "γραω" => Ἐχω,
        "γρυλιζω" => Ἐχω,
        "γυμνοω" => Ποιεω,
        "γυροω" => Χεω,
        "δαιζω" => Ἐχω,
        "δακνω" => Ὑπᾰγω,
        "δαμαζω" => Δᾰμαζω,
        "δαμναω" => Ποιεω,
        "δαπαναω" => Ποιεω,
        "δαρθανω" => Ἐχω,
        "δατεομαι" => Ἡγεομαι,
        "δει" => Δει,
        "δειγματιζω" => Ἐχω,
        "δειδω" => Ἐχω,
        "δεικνυμι" => Ζωννῡμῐ,
        "δειλομαι" => Οἰομαι,
        "δειπνεω" => Ποιεω,
        "δεμω" => Ἐχω,
        "δεξιοομαι" => Δεξῐοομαι,
        "δερκομαι" => Οἰομαι,
        "δερω" => Καθιζω,
        "δεχομαι" => Λογῐζομαι,
        "δεω" => Ποιεω,
        "δηιοω" => Ἐχω,
        "δηλεομαι" => Χρᾰομαι,
        "δημοκρατεομαι" => Χρᾰομαι,
        "δηοω" => Ἐχω,
        "δηω" => Δει,
        "διαβαινω" => Ἐχω,
        "διαβαλλω" => Προβαλλω,
        "διαγωνιζομαι" => Λογῐζομαι,
        "διαζωννυμι" => Ἐπορνῡμῐ,
        "διαιρεω" => Ἀναιρεω,
        "διαιρω" => Καθιζω,
        "διαιταω" => Ἐχω,
        "διακειμαι" => Οἰομαι,
        "διακονεω" => Ποιεω,
        "διαλεγω" => Ἐχω,
        "διαλειπω" => Στειχω,
        "διαμηνυω" => Ἐχω,
        "διαπερθω" => Ἐχω,
        "διαπυνθανομαι" => Οἰομαι,
        "διαρκεω" => Ἐχω,
        "διαρπαζω" => Ἐχω,
        "διασευομαι" => Θεᾰομαι,
        "διασκευαζω" => Ἐχω,
        "διασκοπεω" => Ἐπισκοπεω,
        "διασωζω" => Ἐχω,
        "διατιθημι" => Τῐθημῐ,
        "διαφερω" => Δῐᾰφερω,
        "διαχεω" => Ἐχω,
        "διδασκω" => Ἐχω,
        "διδρασκω" => Ἐχω,
        "διδωμι" => Οἰομαι,
        "διεπω" => Προϊσχω,
        "διερχομαι" => Πᾰρερχομαι,
        "διευθυνω" => Καθιζω,
        "διηγεομαι" => Χρᾰομαι,
        "διιστημι" => Ζωννῡμῐ,
        "δικειν" => Πᾰσχω,
        "δινεω" => Ποιεω,
        "διπλασιοομαι" => Προσερχομαι,
        "διπλοω" => Δει,
        "διψαω" => Ποιεω,
        "δογματογραφεω" => Ποιεω,
        "δογματοποιεω" => Ποιεω,
        "δοκεω" => Ποιεω,
        "δολοφονεω" => Ποιεω,
        "δομεω" => Ποιεω,
        "δονεω" => Ποιεω,
        "δοξαζω" => Ἐχω,
        "δουπεω" => Ποιεω,
        "δραπετευω" => Ἐχω,
        "δραω" => Δρᾱω,
        "δυναμαι" => Οἰομαι,
        "δυπτω" => Ἐχω,
        "δυστυχεω" => Ποιεω,
        "δυω" => Ἐχω,
        "δωρεω" => Ποιεω,
        "εαω" => Ἐχω,
        "εγειρω" => Καθιζω,
        "εγκαθιστημι" => Ἀνῐστημῐ,
        "εγκαλεω" => Ποιεω,
        "εγκαταλεγω" => Ἐχω,
        "εεργω" => Ὑπᾰγω,
        "εζομαι" => Ἀρνυμαι,
        "εθελω" => Ἐχω,
        "εθιζω" => Καθιζω,
        "ειδον" => Εἰδον,
        "εικαζω" => Ἐχω,
        "εικω" => Ἐχω,
        "ειλω" => Ἐχω,
        "ειμι" => Δει,
        "ειπον" => Εἰδον,
        "εισαγω" => Ὑπᾰγω,
        "εισακουω" => Ἐχω,
        "εισαναβαινω" => Κᾰτᾰβαινω,
        "εισερχομαι" => Πᾰρερχομαι,
        "εισοραω" => Ἀναιρεω,
        "εκγαμιζω" => Ἐχω,
        "εκδιωκω" => Ἐχω,
        "εκδυω" => Ἐχω,
        "εκκαλεω" => Ποιεω,
        "εκκρυπτω" => Ἐχω,
        "εκλεγω" => Ἐχω,
        "εκλυτροω" => Χεω,
        "εκρεω" => Ὁρῐζω,
        "εκτεινω" => Ὁρῐζω,
        "εκτιθημι" => Τῐθημῐ,
        "εκφερω" => Ἐχω,
        "εκχεω" => Ποιεω,
        "ελασσοω" => Ἐχω,
        "ελαυνω" => Ἐχω,
        "ελαω" => Ἐχω,
        "ελεγχω" => Ἐχω,
        "ελεεω" => Ἐχω,
        "ελελιζω" => Ἐχω,
        "ελευθεροω" => Ἐχω,
        "ελισσω" => Ἐχω,
        "ελκυω" => Ἐχω,
        "ελκω" => Ἐχω,
        "ελλειπω" => Στειχω,
        "ελληνιζω" => Καθιζω,
        "ελπιζω" => Καθιζω,
        "ελπω" => Ἐχω,
        "εμβαλλω" => Προβαλλω,
        "εμεω" => Ἐχω,
        "εμπαιζω" => Ἐχω,
        "εμπηγνυμι" => Ζωννῡμῐ,
        "εμπιμπρημι" => Ποιεω,
        "εναντιοομαι" => Οἰομαι,
        "εναριζω" => Ἐχω,
        "ενδεω" => Χεω,
        "ενδυω" => Ἐχω,
        "ενειμι" => Τῐθημῐ,
        "ενιημι" => Ἐνῑημῐ,
        "ενιστημι" => Ἀνῐστημῐ,
        "εννυμι" => Ζωννῡμῐ,
        "ενοραω" => Ἐξευρῐσκω,
        "εντυγχανω" => Ἐχω,
        "εξαγρεω" => Ἐχω,
        "εξαιρεω" => Ἐξαιρεω,
        "εξαιρω" => Καθιζω,
        "εξαναλυω" => Ἐχω,
        "εξαπτω" => Ὑπᾰρχω,
        "εξαρπαζω" => Ἐχω,
        "εξαρταω" => Ἐχω,
        "εξαρτυω" => Ἐχω,
        "εξαρχω" => Ὑπᾰρχω,
        "εξασθενεω" => Ἐχω,
        "εξαφανιζω" => Καθιζω,
        "εξειμι" => Χρᾰομαι,
        "εξειπον" => Ἀπειπον,
        "εξελαυνω" => Δᾰμαζω,
        "εξεραω" => Ἐχω,
        "εξεργαζομαι" => Οἰομαι,
        "εξερομαι" => Ἀρνυμαι,
        "εξερχομαι" => Πᾰρερχομαι,
        "εξεστι" => Ἐχω,
        "εξεταζω" => Ἐχω,
        "εξευρισκω" => Ἐξευρῐσκω,
        "εξιημι" => Ἐξῑημῐ,
        "εξικνεομαι" => Ἐξικνεομαι,
        "εξισοω" => Ἐχω,
        "εξιστημι" => Ἀνῐστημῐ,
        "εξομολογεω" => Ἐχω,
        "εξοτρυνω" => Ὁρῐζω,
        "εξυπνιζω" => Ἐχω,
        "εορταζω" => Ἐχω,
        "επαγω" => Πᾰρεχω,
        "επαινεω" => Ἐχω,
        "επαιρω" => Καθιζω,
        "επαισσω" => Ἐχω,
        "επαλθαινω" => Ἐχω,
        "επαμυνω" => Ὁρῐζω,
        "επαναλαμβανω" => Ἐξαιρεω,
        "επαναλυω" => Ἐχω,
        "επαναχωρεω" => Ἐχω,
        "επανερχομαι" => Ἐπορον,
        "επανιημι" => Ἀφῑημῐ,
        "επαπολυω" => Ἐχω,
        "επαρχω" => Ἐχω,
        "επειγω" => Ἐχω,
        "επειμι" => Τῐθημῐ,
        "επεκβαινω" => Κᾰτᾰβαινω,
        "επενδυνω" => Ἐχω,
        "επεντιθημι" => Τῐθημῐ,
        "επεξειμι" => Χρᾰομαι,
        "επεξερχομαι" => Πᾰρερχομαι,
        "επεξηγεομαι" => Προσερχομαι,
        "επερομαι" => Ἀρνυμαι,
        "επερχομαι" => Πᾰρερχομαι,
        "επευφημεω" => Ἐχω,
        "επεφνον" => Ἐπορον,
        "επεχω" => Πᾰρεχω,
        "επιγιγνωσκω" => Ἐχω,
        "επικαλεω" => Καθιζω,
        "επιλαμβανω" => Ἐπῐλᾰμβᾰνω,
        "επιληθω" => Ἐχω,
        "επιμειδαω" => Ἐχω,
        "επιμελεομαι" => Οἰομαι,
        "επιμιμνησκομαι" => Ὠνεομαι,
        "επισευω" => Οἰομαι,
        "επισκευαζω" => Ὁρῐζω,
        "επισκοπεω" => Ἐπισκοπεω,
        "επιτρεχω" => Ἐχω,
        "επιφαινω" => Καθιζω,
        "επιφερω" => Δῐᾰφερω,
        "επιχειρεω" => Ἐχω,
        "εποιχομαι" => Οἰομαι,
        "επομαι" => Οἰομαι,
        "επονομαζω" => Ἐχω,
        "επορνυμι" => Ἐπορνῡμῐ,
        "επορον" => Ἐπορον,
        "εποτρυνω" => Ἐχω,
        "επουλοω" => Ἐχω,
        "εραμαι" => Ἀνῐστημῐ,
        "εραω" => Ἐχω,
        "εργαζομαι" => Οἰομαι,
        "ερδω" => Ἐχω,
        "ερεεινω" => Ἐχω,
        "ερεθιζω" => Καθιζω,
        "ερειδω" => Ἐχω,
        "ερειπω" => Ἐχω,
        "ερεσσω" => Ἐχω,
        "ερευγομαι" => Οἰομαι,
        "ερευθω" => Ἐχω,
        "ερεφω" => Ἐχω,
        "ερημοω" => Ἐχω,
        "εριδαινω" => Ὁρῐζω,
        "ερομαι" => Ἀρνυμαι,
        "ερπυζω" => Ἐχω,
        "ερπω" => Ἐχω,
        "ερρω" => Ἐχω,
        "ερυθραινω" => Ἐχω,
        "ερυκω" => Ἐχω,
        "ερυω" => Ἐχω,
        "ερχομαι" => Οἰομαι,
        "εσθιω" => Ἐχω,
        "ετεροζυγεω" => Ἐχω,
        "ετοιμαζω" => Ὁρῐζω,
        "ευαγγελιζω" => Καθιζω,
        "ευδω" => Ἐχω,
        "ευθυνω" => Ἐχω,
        "ευλαβεομαι" => Προσερχομαι,
        "ευλογεω" => Ἐχω,
        "ευναω" => Ἐχω,
        "ευνομεομαι" => Ἡγεομαι,
        "ευρισκω" => Λᾰμβᾰνω,
        "ευρυνω" => Καθιζω,
        "ευτρεπιζω" => Καθιζω,
        "ευτυχεω" => Ἐχω,
        "ευφημιζω" => Καθιζω,
        "ευφραινω" => Ὁρῐζω,
        "ευχαριστεω" => Ἐχω,
        "ευχομαι" => Οἰομαι,
        "εφαγον" => Ἐπορον,
        "εφαλλομαι" => Ὑποκοριζομαι,
        "εφανδανω" => Ἐχω,
        "εφελκω" => Ἐχω,
        "εφεπω" => Εἰδον,
        "εφευρισκω" => Ἐξευρῐσκω,
        "εφηκω" => Ὑπᾰρχω,
        "εφιημι" => Ἐφῑημῐ,
        "εφιστημι" => Ἀνῐστημῐ,
        "εφοραω" => Ἐπισκοπεω,
        "εφορμαω" => Ἐχω,
        "εφορμεω" => Ἐχω,
        "εχθω" => Ἐχω,
        "εχω" => Ἐχω,
        "εψω" => Ἐχω,
        "ζεω" => Χεω,
        "ζητεω" => Ποιεω,
        "ζω" => Ζω,
        "ζωγραφεω" => Ποιεω,
        "ζωγρεω" => Ποιεω,
        "ζωννυμι" => Ζωννῡμῐ,
        "ζωογονεω" => Ποιεω,
        "ηβαω" => Ἐχω,
        "ηδομαι" => Οἰομαι,
        "ηδυνω" => Ἐχω,
        "ηθολογεω" => Ἐχω,
        "ηλθον" => Εἰδον,
        "ηλοω" => Ἐχω,
        "ημι" => Δει,
        "ησσαομαι" => Ἡγεομαι,
        "ησυχαζω" => Ἐχω,
        "ηχεω" => Ἐχω,
        "θαλλω" => Ἐχω,
        "θαπτω" => Ἐχω,
        "θαρσεω" => Ποιεω,
        "θεαομαι" => Θεᾰομαι,
        "θεινω" => Ὁρῐζω,
        "θεοπροπεω" => Ποιεω,
        "θεω" => Χεω,
        "θεωρεω" => Ποιεω,
        "θηραω" => Ποιεω,
        "θιγγανω" => Ἐχω,
        "θλαω" => Ποιεω,
        "θναισκω" => Ἐχω,
        "θνησκω" => Ὁρῐζω,
        "θρασσω" => Ἐχω,
        "θραττω" => Ἐχω,
        "θραυω" => Ἐχω,
        "θρωσκω" => Ὁρῐζω,
        "θυμιαω" => Ποιεω,
        "θυω" => Ἐχω,
        "θωαδδω" => Ἐχω,
        "θωεω" => Ἐχω,
        "θωρησσω" => Περθω,
        "ιαινω" => Ἐχω,
        "ιαλλω" => Καθιζω,
        "ιαομαι" => Ἡγεομαι,
        "ιδροω" => Ἐχω,
        "ιζω" => Ἐχω,
        "ιημι" => Τῐθημῐ,
        "ικνεομαι" => Ἡγεομαι,
        "ικω" => Ἐχω,
        "ιλασκομαι" => Οἰομαι,
        "ιναω" => Ἐχω,
        "ινεω" => Ἐχω,
        "ιουδαιζω" => Ἐχω,
        "ισοθεοω" => Ἐχω,
        "ισορροπεω" => Ἐχω,
        "ιστημι" => Ἀνῐστημῐ,
        "ιχθυολογεω" => Ἐχω,
        "καθαιρεω" => Ἀντιλεγω,
        "καθαιρω" => Καθιζω,
        "καθεζομαι" => Ἀρνυμαι,
        "καθελκω" => Προϊσχω,
        "καθευδω" => Ἐχω,
        "καθημαι" => Ἐχω,
        "καθιζανω" => Ἐχω,
        "καθιζω" => Καθιζω,
        "καθιημι" => Ἀφῑημῐ,
        "καθιστημι" => Ἀνῐστημῐ,
        "καθοραω" => Ἀναιρεω,
        "καινυμαι" => Οἰομαι,
        "καινω" => Ὁρῐζω,
        "καιω" => Ἐχω,
        "κακκαω" => Ποιεω,
        "κακοπαθεω" => Ποιεω,
        "κακοω" => Ἐχω,
        "καλεω" => Ἐχω,
        "καλλιγραφεω" => Χεω,
        "καλυπτω" => Ἐχω,
        "καμνω" => Ὁρῐζω,
        "καμπτω" => Ἐχω,
        "καναχεω" => Ποιεω,
        "καπνιζω" => Καθιζω,
        "καπτω" => Ἐχω,
        "καταβαινω" => Κᾰτᾰβαινω,
        "καταβοαω" => Ἐχω,
        "καταγω" => Ὑπᾰγω,
        "καταδιωκω" => Ἐχω,
        "καταδυω" => Ἐχω,
        "καταθνησκω" => Ὁρῐζω,
        "καταικιζω" => Καθιζω,
        "καταιρω" => Καθιζω,
        "κατακαλεω" => Καθιζω,
        "καταλαμβανω" => Ἐξαιρεω,
        "καταλεγω" => Ἐχω,
        "καταναγκαζω" => Ἐχω,
        "καταπαιδεραστεω" => Ἐχω,
        "καταπετροω" => Ἐχω,
        "καταργεω" => Ἐχω,
        "κατασκηνοω" => Ἐχω,
        "καταστρεφω" => Ἐχω,
        "κατατιθημι" => Τῐθημῐ,
        "κατατρεχω" => Ὁρῐζω,
        "καταφρονεω" => Ἐχω,
        "καταχραομαι" => Οἰομαι,
        "κατειμι" => Ἐνῑημῐ,
        "κατειργω" => Ἐχω,
        "κατεπειγω" => Ἐχω,
        "κατεργαζομαι" => Οἰομαι,
        "κατερυω" => Ἐχω,
        "κατερχομαι" => Προσερχομαι,
        "κατευθυνω" => Καθιζω,
        "κατηγορεω" => Ἐχω,
        "κατισχυω" => Ἐχω,
        "κατοικειοομαι" => Οἰομαι,
        "κατοικεω" => Ἐχω,
        "κεαζω" => Ἐχω,
        "κειμαι" => Ἐχω,
        "κειρω" => Καθιζω,
        "κεκαδον" => Στειχω,
        "κελαδεω" => Ποιεω,
        "κελλω" => Ἐχω,
        "κεντεω" => Ποιεω,
        "κεραννυμι" => Ζωννῡμῐ,
        "κερδαινω" => Ὁρῐζω,
        "κερτομεω" => Ποιεω,
        "κευθω" => Ἐχω,
        "κεφαλοτομεω" => Ποιεω,
        "κηδω" => Ἐχω,
        "κηλεω" => Ποιεω,
        "κινεω" => Ποιεω,
        "κιω" => Κῐω,
        "κλαζω" => Ἐχω,
        "κλαιω" => Καθιζω,
        "κλαω" => Ποιεω,
        "κλειδοω" => Ἐχω,
        "κλεπτω" => Ἐχω,
        "κλεω" => Ποιεω,
        "κληρονομεω" => Ποιεω,
        "κλινω" => Καθιζω,
        "κλονεω" => Ποιεω,
        "κλυω" => Πᾰσχω,
        "κναπτω" => Ἐχω,
        "κναω" => Ποιεω,
        "κνιζω" => Ἐχω,
        "κοεω" => Ποιεω,
        "κοιμαω" => Ποιεω,
        "κοινοω" => Χεω,
        "κοινωνεω" => Ποιεω,
        "κοιρανεω" => Ποιεω,
        "κολλαω" => Ποιεω,
        "κολοβοω" => Ἐχω,
        "κολυμβαω" => Ποιεω,
        "κομαω" => Ποιεω,
        "κομπεω" => Ποιεω,
        "κομπολακεω" => Ποιεω,
        "κονιω" => Ἐχω,
        "κορεννυμι" => Ζωννῡμῐ,
        "κορυσσω" => Ἐχω,
        "κοσμεω" => Ποιεω,
        "κουροτροφεω" => Ποιεω,
        "κραζω" => Ἐχω,
        "κραινω" => Ὁρῐζω,
        "κρατευω" => Ἐχω,
        "κρατεω" => Ποιεω,
        "κρατυνω" => Καθιζω,
        "κρεμαννυμι" => Ζωννῡμῐ,
        "κρεμαω" => Ποιεω,
        "κρετεω" => Ποιεω,
        "κρητιζω" => Ἐχω,
        "κρινω" => Καθιζω,
        "κροτεω" => Ποιεω,
        "κρωζω" => Ἐχω,
        "κταομαι" => Οἰομαι,
        "κτεινω" => Καθιζω,
        "κτιζω" => Ἐχω,
        "κτυπεω" => Ποιεω,
        "κυβερναω" => Ποιεω,
        "κυεω" => Ποιεω,
        "κυκαω" => Ποιεω,
        "κυλινδω" => Ἐχω,
        "κυμαινω" => Ὁρῐζω,
        "κυνεω" => Ποιεω,
        "κυπτω" => Καθιζω,
        "λαβρηγορεω" => Ποιεω,
        "λαγαιω" => Ἐχω,
        "λαγοθηρεω" => Ποιεω,
        "λαγχανω" => Ἐχω,
        "λαθω" => Ἐχω,
        "λαιφασσω" => Ἐχω,
        "λακωνομανεω" => Χεω,
        "λαλεω" => Ποιεω,
        "λαμβανω" => Λᾰμβᾰνω,
        "λαμπαδαρχεω" => Ποιεω,
        "λαμπαδευω" => Ἐχω,
        "λαμπαδηφορεω" => Ποιεω,
        "λαμπαδοδρομεω" => Ποιεω,
        "λαμπαδουχεω" => Ποιεω,
        "λαμπω" => Ἐχω,
        "λανθανω" => Δῐᾰφερω,
        "λαπτω" => Ἐχω,
        "λαρκοφορεω" => Ποιεω,
        "λασκω" => Ἐχω,
        "λατρειω" => Ἐχω,
        "λαφυσσω" => Ἐχω,
        "λαχαινω" => Ἐχω,
        "λαχνοομαι" => Χρᾰομαι,
        "λεγω" => Ἐχω,
        "λειπω" => Στειχω,
        "λεπτυνω" => Ὁρῐζω,
        "λευκαινω" => Ὁρῐζω,
        "λευκογραφεω" => Ποιεω,
        "λεχομαι" => Οἰομαι,
        "ληζομαι" => Οἰομαι,
        "ληθω" => Στειχω,
        "ληκαω" => Ποιεω,
        "ληκεω" => Ποιεω,
        "λιαζομαι" => Οἰομαι,
        "λιζω" => Ἐχω,
        "λικμαω" => Ποιεω,
        "λιμαινω" => Ἐχω,
        "λιμοκτονεω" => Ποιεω,
        "λινοστατεω" => Ποιεω,
        "λιπαρεω" => Ποιεω,
        "λισσομαι" => Οἰομαι,
        "λογιζομαι" => Λογῐζομαι,
        "λογοθετεω" => Ποιεω,
        "λογομαχεω" => Ποιεω,
        "λογχοβολεω" => Ποιεω,
        "λοιδορεω" => Ποιεω,
        "λοιπογραφεω" => Ποιεω,
        "λουτιαω" => Ποιεω,
        "λουτροομαι" => Χρᾰομαι,
        "λουτροφορεω" => Ποιεω,
        "λουω" => Ἐχω,
        "λοχαω" => Ποιεω,
        "λυζω" => Ἐχω,
        "λυπεω" => Ἐχω,
        "λυριζω" => Καθιζω,
        "λυσιτελεω" => Ποιεω,
        "λυτροω" => Ποιεω,
        "λυω" => Ἐχω,
        "λωγαω" => Ποιεω,
        "μαδαω" => Ποιεω,
        "μαινομαι" => Ὑποκοριζομαι,
        "μακαριζω" => Καθιζω,
        "μακρηγορεω" => Ποιεω,
        "μανθανω" => Ἐχω,
        "μαντευομαι" => Οἰομαι,
        "μαραινω" => Ὁρῐζω,
        "μαρμαιρω" => Ἐχω,
        "μαρτυρεω" => Ποιεω,
        "μασαομαι" => Ἡγεομαι,
        "μαστιγοω" => Οἰομαι,
        "μαστιζω" => Ἐχω,
        "μαστιχαω" => Ποιεω,
        "μεθαλλομαι" => Ὑποκοριζομαι,
        "μεθιστημι" => Ἀνῐστημῐ,
        "μεθυσκω" => Ἐπῐλᾰμβᾰνω,
        "μεθυω" => Ἐχω,
        "μειδαω" => Ὠχρῐω,
        "μειδιαω" => Ποιεω,
        "μειλισσω" => Ἐχω,
        "μειρομαι" => Οἰομαι,
        "μελαινω" => Ὁρῐζω,
        "μελεταω" => Ποιεω,
        "μελλω" => Ἐχω,
        "μελπω" => Ἐχω,
        "μελω" => Ἐχω,
        "μεμφομαι" => Οἰομαι,
        "μενω" => Ἐχω,
        "μεριζω" => Καθιζω,
        "μεριμναω" => Ποιεω,
        "μεταλαγχανω" => Ἐχω,
        "μεταλαμβανω" => Ἐξαιρεω,
        "μεταφερω" => Δῐᾰφερω,
        "μετειμι" => Τῐθημῐ,
        "μετεκιαθον" => Πᾰσχω,
        "μετερχομαι" => Πᾰρερχομαι,
        "μετρεω" => Ποιεω,
        "μηδιζω" => Ἐχω,
        "μηδομαι" => Οἰομαι,
        "μηκαομαι" => Προσερχομαι,
        "μηκυνω" => Ὁρῐζω,
        "μηνιω" => Ὁρῐζω,
        "μηχαναομαι" => Χρᾰομαι,
        "μηχαναω" => Ποιεω,
        "μιαινω" => Ὁρῐζω,
        "μιγνυμι" => Ζωννῡμῐ,
        "μιμεομαι" => Χρᾰομαι,
        "μιμνησκω" => Ἐπῐλᾰμβᾰνω,
        "μισεω" => Ποιεω,
        "μναομαι" => Προσερχομαι,
        "μολυνω" => Ὁρῐζω,
        "μοσχοποιεω" => Ποιεω,
        "μοχθεω" => Ποιεω,
        "μυεω" => Ποιεω,
        "μυζαω" => Ποιεω,
        "μυκαομαι" => Ἡγεομαι,
        "μυκτηριζω" => Ἐχω,
        "μωκαομαι" => Χρᾰομαι,
        "μωλεω" => Ἐχω,
        "μωμαομαι" => Μωμᾰομαι,
        "μωραινω" => Ὁρῐζω,
        "ναιω" => Ἐχω,
        "νασσω" => Ἐχω,
        "ναυκληρεω" => Ποιεω,
        "ναυμαχεω" => Ποιεω,
        "ναω" => Ποιεω,
        "νεαω" => Ποιεω,
        "νεικεω" => Ποιεω,
        "νεκραγωγεω" => Ἐχω,
        "νεμεσαω" => Ποιεω,
        "νεμω" => Καθιζω,
        "νεομαι" => Προσερχομαι,
        "νεω" => Καθιζω,
        "νηστευω" => Ἐχω,
        "νιζω" => Ἐχω,
        "νικαω" => Ποιεω,
        "νισσομαι" => Οἰομαι,
        "νοεω" => Ποιεω,
        "νοσεω" => Ποιεω,
        "νοσοκομεω" => Ἐχω,
        "νοστεω" => Ποιεω,
        "νουθετεω" => Ἐχω,
        "νωμαω" => Ποιεω,
        "ξαινω" => Ὁρῐζω,
        "ξεω" => Ἐχω,
        "ξηραινω" => Καθιζω,
        "ξυγκινδυνευω" => Ἐχω,
        "ξυλλαμβανω" => Ἐξαιρεω,
        "ξυμμενω" => Καθιζω,
        "ξυμπολεμεω" => Ποιεω,
        "ξυμφερω" => Ἐχω,
        "ξυναγω" => Ἐχω,
        "ξυναδικεω" => Ἐχω,
        "ξυναιρεω" => Ὁρῐζω,
        "ξυνεπιτιθημι" => Τῐθημῐ,
        "ξυνεπομαι" => Προσερχομαι,
        "ξυνεργαζομαι" => Οἰομαι,
        "ξυνερχομαι" => Πᾰρερχομαι,
        "ξυνιημι" => Ἀφῑημῐ,
        "ξυνιστημι" => Ἀνῐστημῐ,
        "ξυνναυμαχεω" => Ποιεω,
        "ξυνοιδα" => Ἐνῑημῐ,
        "ξυνοικιζω" => Καθιζω,
        "ξυνοικοδομεω" => Ἐχω,
        "ξυνομνυμι" => Ὀμνῡμῐ,
        "ξυρεω" => Ποιεω,
        "ξυρρηγνυμι" => Ἐνῑημῐ,
        "ξυσκοταζω" => Ἐχω,
        "ξυσσωζω" => Ἐχω,
        "ξυστρατευω" => Ἐχω,
        "οδηγεω" => Ἐχω,
        "οδυρομαι" => Λογῐζομαι,
        "οδυσσομαι" => Οἰομαι,
        "οιδανω" => Ὑπᾰγω,
        "οικειοομαι" => Προσερχομαι,
        "οικειοω" => Ἐχω,
        "οικιζω" => Καθιζω,
        "οιμωζω" => Ἐχω,
        "οιομαι" => Οἰομαι,
        "οιχομαι" => Οἰομαι,
        "ολισθανω" => Ἐχω,
        "ολλυμι" => Οἰομαι,
        "ολοφυρομαι" => Λογῐζομαι,
        "ομαδεω" => Οἰομαι,
        "ομνυμι" => Ὀμνῡμῐ,
        "ομοιοω" => Ἐχω,
        "ομολογεω" => Ἐχω,
        "ομφυνω" => Καθιζω,
        "ονινημι" => Ἀνῐστημῐ,
        "ονομαζω" => Ἐχω,
        "ονομαινω" => Ὁρῐζω,
        "οξυνω" => Ὁρῐζω,
        "οπλιζω" => Καθιζω,
        "οραω" => Λᾰμβᾰνω,
        "ορεγω" => Ἐχω,
        "ορθοω" => Ἐχω,
        "οριζω" => Ὁρῐζω,
        "ορμαινω" => Ἐχω,
        "ορμαω" => Ἐχω,
        "ορνυμι" => Ζωννῡμῐ,
        "ορυσσω" => Ἐχω,
        "οσφραινομαι" => Οἰομαι,
        "ουρεω" => Χεω,
        "ουταω" => Ἐχω,
        "οφειλω" => Ἐχω,
        "οφελλω" => Ἐχω,
        "οχεω" => Ἐχω,
        "παγιδευω" => Ἐχω,
        "παιδεραστεω" => Ποιεω,
        "παιζω" => Ἐχω,
        "παλλω" => Ἐχω,
        "παραβαινω" => Κᾰτᾰβαινω,
        "παραγω" => Ὑπᾰγω,
        "παραδιδωμι" => Οἰομαι,
        "παραινεω" => Ἐχω,
        "παραιτεομαι" => Οἰομαι,
        "παρακαλεω" => Καθιζω,
        "παραλαμβανω" => Ἐχω,
        "παρατιθημι" => Τῐθημῐ,
        "παρατυγχανω" => Ἐχω,
        "παρειμι" => Τῐθημῐ,
        "παρειπον" => Προειπον,
        "παρεκτρεπω" => Ἐχω,
        "παρερχομαι" => Πᾰρερχομαι,
        "παρεχω" => Πᾰρεχω,
        "παρηγορεω" => Ποιεω,
        "παροξυνω" => Ὁρῐζω,
        "παροραω" => Λᾰμβᾰνω,
        "παροτρυνω" => Ὁρῐζω,
        "πασχητιαω" => Ποιεω,
        "πασχω" => Πᾰσχω,
        "παταγεω" => Ποιεω,
        "πατασσω" => Ἐχω,
        "πατεομαι" => Ἡγεομαι,
        "πατεω" => Ποιεω,
        "πατροκτονεω" => Ποιεω,
        "παχυνω" => Ὁρῐζω,
        "πεζοπορεω" => Ποιεω,
        "πειθω" => Ἐχω,
        "πειναω" => Ποιεω,
        "πειραω" => Ποιεω,
        "πειρω" => Ἐχω,
        "πεκω" => Ἐχω,
        "πελαζω" => Ὁρῐζω,
        "πενθεω" => Ποιεω,
        "πεπνυμαι" => Ἐχω,
        "περαινω" => Ὁρῐζω,
        "περαω" => Ποιεω,
        "περδομαι" => Οἰομαι,
        "περθω" => Περθω,
        "περιειμι" => Χρᾰομαι,
        "περιεχω" => Πᾰρεχω,
        "περιθαλπω" => Ἐχω,
        "περιιστημι" => Ἀνῐστημῐ,
        "περικλειω" => Ἐχω,
        "περιοραω" => Χεω,
        "περισκοπεω" => Χεω,
        "περιτειχιζω" => Καθιζω,
        "περιτρεχω" => Ὁρῐζω,
        "περιφερω" => Ἐχω,
        "περνημι" => Περνημῐ,
        "πεσσω" => Ἐχω,
        "πεταννυμι" => Ζωννῡμῐ,
        "πετομαι" => Οἰομαι,
        "πηγνυμι" => Ζωννῡμῐ,
        "πηδαω" => Ποιεω,
        "πημαινω" => Ὁρῐζω,
        "πιαινω" => Ὁρῐζω,
        "πιδαω" => Ποιεω,
        "πιεζω" => Ἐχω,
        "πιλεω" => Ποιεω,
        "πιμπλημι" => Ἀνῐστημῐ,
        "πιμπρημι" => Ἀνῐστημῐ,
        "πινω" => Ὁρῐζω,
        "πιπρασκω" => Ἐπῐλᾰμβᾰνω,
        "πιπτω" => Ὁρῐζω,
        "πλαζω" => Ἐχω,
        "πλαναω" => Ποιεω,
        "πλατυνω" => Ὁρῐζω,
        "πλεκω" => Ἐχω,
        "πλεονεκτεω" => Ποιεω,
        "πλεω" => Χεω,
        "πληθυνω" => Ὁρῐζω,
        "πληθω" => Ἐχω,
        "πληροφορεω" => Ποιεω,
        "πλησσω" => Ἐχω,
        "πλισσομαι" => Οἰομαι,
        "πλουτοκρατεομαι" => Χρᾰομαι,
        "πλυνω" => Ὁρῐζω,
        "πλωω" => Ἐχω,
        "πνεω" => Καθιζω,
        "πνιγω" => Ἐχω,
        "ποδιζω" => Καθιζω,
        "ποδοκτυπεω" => Χεω,
        "ποεω" => Ποιεω,
        "ποθεω" => Ἐχω,
        "ποιεω" => Ποιεω,
        "ποιμαινω" => Καθιζω,
        "πολεμεω" => Ποιεω,
        "πολιορκεω" => Ποιεω,
        "πολιοφυλακεω" => Ποιεω,
        "πονεω" => Ποιεω,
        "πορθεω" => Ποιεω,
        "πορσυνω" => Ὁρῐζω,
        "πραδδω" => Ἐχω,
        "πρασσω" => Ἐχω,
        "πραττω" => Ἐχω,
        "πραυνω" => Καθιζω,
        "πρηθω" => Ἐχω,
        "προειμι" => Ἐνῑημῐ,
        "προειπον" => Προειπον,
        "προερχομαι" => Πᾰρερχομαι,
        "προετοιμαζω" => Ἐχω,
        "προιημι" => Ἀφῑημῐ,
        "προιστημι" => Ἀνῐστημῐ,
        "προισχω" => Προϊσχω,
        "προκαταλαμβανω" => Ἐξαιρεω,
        "προλαμβανω" => Ἐξαιρεω,
        "προμηνυω" => Ἐχω,
        "προοιδα" => Ἐνῑημῐ,
        "προοραω" => Ἐξαιρεω,
        "προοριζω" => Καθιζω,
        "προοφειλω" => Ἐχω,
        "προσαγορευω" => Ἀντιλεγω,
        "προσαγω" => Ἀναιρεω,
        "προσαυδαω" => Ἐχω,
        "προσειμι" => Ἐνῑημῐ,
        "προσειπον" => Προειπον,
        "προσερχομαι" => Προσερχομαι,
        "προσεχω" => Πᾰρεχω,
        "προσηκω" => Ὑπᾰρχω,
        "προσκαλεω" => Ποιεω,
        "προσκυνεω" => Ποιεω,
        "προσλαμβανω" => Ἐξαιρεω,
        "προσμειγνυμι" => Ζωννῡμῐ,
        "προσφερω" => Ἐχω,
        "προσφωνεω" => Ἀντιλεγω,
        "προσχωρεω" => Χεω,
        "προτερεω" => Ποιεω,
        "προυφειλω" => Ἐχω,
        "προωθεω" => Χεω,
        "πταρνυμαι" => Ἐπορον,
        "πτησσω" => Ἐχω,
        "πτισσω" => Ἐχω,
        "πτοεω" => Ἐχω,
        "πτυσσω" => Ἐχω,
        "πτυω" => Ἐχω,
        "πυνθανομαι" => Οἰομαι,
        "πυροω" => Χεω,
        "πυρσαινω" => Δει,
        "πωλεω" => Ἐχω,
        "ραινω" => Ὁρῐζω,
        "ραντιζω" => Καθιζω,
        "ραπιζω" => Καθιζω,
        "ραπτω" => Ἐχω,
        "ραφανιδοω" => Ποιεω,
        "ρεζω" => Ἐχω,
        "ρεμβω" => Ἐχω,
        "ρεπω" => Ἐχω,
        "ρεω" => Ποιεω,
        "ρηγνυμι" => Ἐνῑημῐ,
        "ριγεω" => Ποιεω,
        "ριπταζω" => Ἐχω,
        "ριπτω" => Ἐχω,
        "ροδιζω" => Ὁρῐζω,
        "ροφεω" => Ποιεω,
        "ρυπαινω" => Καθιζω,
        "ρωμαιζω" => Ἐχω,
        "σαβαζω" => Ἐχω,
        "σαινω" => Ἐχω,
        "σαιρω" => Ἐχω,
        "σαμαινω" => Ἐχω,
        "σαοω" => Ἐχω,
        "σαττω" => Ἐχω,
        "σβεννυμι" => Ζωννῡμῐ,
        "σεβομαι" => Οἰομαι,
        "σευω" => Ἐχω,
        "σηθω" => Ἐχω,
        "σηκοω" => Ἐχω,
        "σημαινω" => Ὁρῐζω,
        "σημειοω" => Ἐχω,
        "σηπω" => Ἐχω,
        "σιγαω" => Ποιεω,
        "σιδηροφορεω" => Ποιεω,
        "σιζω" => Ἐχω,
        "σιωπαω" => Ποιεω,
        "σκανδαλιζω" => Ἐχω,
        "σκαπτω" => Ἐχω,
        "σκεδαννυμι" => Ὑποκοριζομαι,
        "σκελλω" => Καθιζω,
        "σκεπτομαι" => Οἰομαι,
        "σκηπτω" => Ἐχω,
        "σκιαζω" => Ἐχω,
        "σκιρταω" => Ποιεω,
        "σκοπεω" => Ποιεω,
        "σκυδμαινω" => Ἐχω,
        "σκυλευω" => Ἐχω,
        "σκυλλω" => Ἐχω,
        "σκυλοδεψεω" => Ποιεω,
        "σμαραγεω" => Ποιεω,
        "σμαω" => Ποιεω,
        "σμηχω" => Ἐχω,
        "σοβεω" => Ποιεω,
        "σοεω" => Ἐχω,
        "σολοικιζω" => Καθιζω,
        "σπαω" => Ποιεω,
        "σπειρω" => Καθιζω,
        "σπενδω" => Ἐχω,
        "σπερχω" => Ἐχω,
        "σπευδω" => Ἐχω,
        "σπιζω" => Ἐχω,
        "σταζω" => Ἐχω,
        "σταυροω" => Ἐχω,
        "στεινω" => Ἐχω,
        "στειχω" => Στειχω,
        "στελλω" => Καθιζω,
        "στενοχωρεω" => Ποιεω,
        "στεργω" => Ἐχω,
        "στερεω" => Τελεω,
        "στερομαι" => Οἰομαι,
        "στεφω" => Ἐχω,
        "στηριζω" => Καθιζω,
        "στιγματιζω" => Ἐχω,
        "στιζω" => Ἐχω,
        "στιλβω" => Ἐχω,
        "στορνυμι" => Ὀμνῡμῐ,
        "στοχαζομαι" => Οἰομαι,
        "στραβιζω" => Ἐχω,
        "στραγγαλοομαι" => Προσερχομαι,
        "στρεφω" => Ἐχω,
        "στρωννυμι" => Ζωννῡμῐ,
        "συγκαλεω" => Τελεω,
        "συγκινδυνευω" => Ἐχω,
        "συγκρυπτω" => Ἐχω,
        "συγχρονιζω" => Καθιζω,
        "συμβουλευω" => Ἐχω,
        "συμμαχεω" => Ποιεω,
        "συμμαχομαι" => Λογῐζομαι,
        "συμπολεμεω" => Ποιεω,
        "συμφερω" => Ἐχω,
        "συναγω" => Ἐχω,
        "συναδικεω" => Ἐχω,
        "συναιρεω" => Ὁρῐζω,
        "συναλιζω" => Καθιζω,
        "συναναστρεφω" => Ἐχω,
        "συναπτω" => Ἐχω,
        "συναρπαζω" => Ἐχω,
        "συνδεω" => Ποιεω,
        "συνειμι" => Τῐθημῐ,
        "συνεπομαι" => Προσερχομαι,
        "συνεργαζομαι" => Οἰομαι,
        "συνερχομαι" => Πᾰρερχομαι,
        "συνθραυω" => Ἐχω,
        "συνιημι" => Ἀφῑημῐ,
        "συνιστημι" => Ἀνῐστημῐ,
        "συνναυμαχεω" => Ποιεω,
        "συνοικιζω" => Καθιζω,
        "συνοικοδομεω" => Ἐχω,
        "συνομνυμι" => Ὀμνῡμῐ,
        "συντειχιζω" => Ἐχω,
        "συντυγχανω" => Ἐχω,
        "συρω" => Καθιζω,
        "σφαλλω" => Καθιζω,
        "σφιγγω" => Ἐχω,
        "σφυζω" => Ἐχω,
        "σχαζω" => Ἐχω,
        "σχαω" => Ποιεω,
        "σχιζω" => Ἐχω,
        "σωζω" => Ὁρῐζω,
        "σωφρονεω" => Ποιεω,
        "ταλαιπωρεω" => Ποιεω,
        "ταρασσω" => Στειχω,
        "ταρχυω" => Ἐχω,
        "τεθνηκετω" => Οἰομαι,
        "τεθνηκοι" => Ἐχω,
        "τεθνηκοιη" => Οἰομαι,
        "τεθνηκοιησ" => Οἰομαι,
        "τεθνηκοιμι" => Οἰομαι,
        "τεινω" => Ὁρῐζω,
        "τειχοφυλακεω" => Ποιεω,
        "τεκμαιρομαι" => Ὑποκοριζομαι,
        "τεκταινομαι" => Ὑποκοριζομαι,
        "τελεθω" => Δει,
        "τελειοω" => Ἐχω,
        "τελεοδρομεω" => Ποιεω,
        "τελευταω" => Ποιεω,
        "τελεω" => Τελεω,
        "τελλω" => Καθιζω,
        "τεμνω" => Ὁρῐζω,
        "τερατοτοκεω" => Ποιεω,
        "τερσομαι" => Οἰομαι,
        "τετραινω" => Ὁρῐζω,
        "τευχω" => Ἐχω,
        "τεχναομαι" => Χρᾰομαι,
        "τηκω" => Ἐχω,
        "τηρεω" => Ποιεω,
        "τιθημι" => Τῐθημῐ,
        "τικτω" => Ὑπᾰγω,
        "τιλλω" => Καθιζω,
        "τιμαω" => Ποιεω,
        "τιμωρεω" => Ποιεω,
        "τινασσω" => Ἐχω,
        "τινω" => Ἐχω,
        "τιτρωσκω" => Ἐπῐλᾰμβᾰνω,
        "τολμαω" => Ποιεω,
        "τοποθετεω" => Ποιεω,
        "τραγωδεω" => Ποιεω,
        "τρεφω" => Ἐχω,
        "τρεχω" => Πᾰσχω,
        "τριαζω" => Ἐχω,
        "τριβω" => Ἐχω,
        "τριζω" => Ἐχω,
        "τρομεω" => Ποιεω,
        "τρυγαω" => Ἐχω,
        "τρυπαω" => Ποιεω,
        "τρυφαω" => Ἐχω,
        "τρωγλοδυτεω" => Ποιεω,
        "τρωγω" => Στειχω,
        "τυγχανω" => Ἐχω,
        "τυπτω" => Ἐχω,
        "τυραννεω" => Ποιεω,
        "τυραννοκτονεω" => Ποιεω,
        "τυφω" => Ἐχω,
        "υγιαινω" => Ὁρῐζω,
        "υδραινω" => Ὁρῐζω,
        "υπαγω" => Ὑπᾰγω,
        "υπακουω" => Ἐχω,
        "υπαρχω" => Ὑπᾰρχω,
        "υπειπον" => Ἀπειπον,
        "υπεκτιθεμαι" => Τῐθημῐ,
        "υπεκφευγω" => Ἐχω,
        "υπεραττικιζω" => Ἐχω,
        "υπερνικαω" => Ἐχω,
        "υπισχνεομαι" => Οἰομαι,
        "υπνοω" => Ἐχω,
        "υπογιγνομαι" => Ἐξαιρεω,
        "υποκοριζομαι" => Ὑποκοριζομαι,
        "υπολαμβανω" => Ἐξαιρεω,
        "υπολογιζομαι" => Λογῐζομαι,
        "υποστηριζω" => Καθιζω,
        "υποφερω" => Δῐᾰφερω,
        "υφαινω" => Ὁρῐζω,
        "υφαιρεω" => Ἐξαιρεω,
        "υφιστημι" => Ἀνῐστημῐ,
        "υψοω" => Ἐχω,
        "υω" => Ἐχω,
        "φαινω" => Ἐχω,
        "φανεροω" => Ἐχω,
        "φαρω" => Ἐχω,
        "φερβω" => Ἐχω,
        "φερω" => Ἐχω,
        "φευγω" => Πᾰσχω,
        "φημι" => Φημῐ,
        "φθανω" => Ἐχω,
        "φθεγγομαι" => Οἰομαι,
        "φθειρω" => Καθιζω,
        "φθονεω" => Ποιεω,
        "φιλεω" => Ποιεω,
        "φιλομαθεω" => Ποιεω,
        "φιλοσοφεω" => Ποιεω,
        "φλιβω" => Ἐχω,
        "φλυαρεω" => Ποιεω,
        "φοβεω" => Ποιεω,
        "φοινικιζω" => Ἐχω,
        "φοιταω" => Ποιεω,
        "φορεω" => Ποιεω,
        "φρασσω" => Ἐχω,
        "φρισσω" => Ἐχω,
        "φρονεω" => Ποιεω,
        "φροντιζω" => Καθιζω,
        "φρουρεω" => Ποιεω,
        "φρυγω" => Ἐχω,
        "φυλακεω" => Χεω,
        "φυλασσω" => Ἐχω,
        "φυσαω" => Ποιεω,
        "φυω" => Ἐχω,
        "φωνεω" => Ποιεω,
        "φωνομαχεω" => Ποιεω,
        "χαζομαι" => Οἰομαι,
        "χαινω" => Ἐχω,
        "χαιρω" => Ἐχω,
        "χαλαροω" => Προσερχομαι,
        "χαλαω" => Ἐχω,
        "χαλεπαινω" => Ὁρῐζω,
        "χαλεπτω" => Ἐχω,
        "χανδανω" => Ἐχω,
        "χαοω" => Ἐχω,
        "χαριζομαι" => Λογῐζομαι,
        "χαριζω" => Ὁρῐζω,
        "χαριτοω" => Ποιεω,
        "χασκω" => Ὁρῐζω,
        "χατιζω" => Ἐχω,
        "χεζω" => Καθιζω,
        "χειροομαι" => Χρᾰομαι,
        "χεω" => Χεω,
        "χλιαινω" => Καθιζω,
        "χολοω" => Ἐχω,
        "χοω" => Ποιεω,
        "χραομαι" => Χρᾰομαι,
        "χραω" => Ποιεω,
        "χρη" => Χρη,
        "χρησιμευω" => Ἐχω,
        "χωρεω" => Ποιεω,
        "ψαλλω" => Καθιζω,
        "ψαω" => Ποιεω,
        "ψεγω" => Ἐχω,
        "ψυχω" => Ἐχω,
        "ωθεω" => Χεω,
        "ωνεομαι" => Ὠνεομαι,
        "ωρυομαι" => Οἰομαι,
        "ωχριω" => Ὠχρῐω,
    }
};

/// Per conjugation, the cells whose ending takes the circumflex when accented on
/// its last syllable (`θεοῦ`).
pub static CIRCUMFLEX: [Cow<'static, [u16]>; 286] = [
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
        PARTICIPLE_FUT_ACT,
    ]), // Μελλω
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
    ]), // Ποιεω
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
    ]), // Βουλευω
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
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἀξῐοω
    Cow::Borrowed(&[
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Κρῑνομαι
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
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Δῐκᾰζω
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
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_PRES_ACT,
        INFINITIVE_AOR_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Φοιτᾰω
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
    ]), // Σκοπεω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Τᾰσσω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_AOR_ACT,
    ]), // Ληγω
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
    ]), // Στῐζω
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
    ]), // Ποικιλλω
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
    ]), // Κᾰτηχεω
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
    ]), // Φᾰντᾰζω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Δᾰπτω
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
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_AOR_ACT,
    ]), // Πεμπω
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
    ]), // Ὀρχεομαι
    Cow::Borrowed(&[]), // Θωᾰω
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
    ]), // Λογῐζομαι
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
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
        AORIST_SUBJ_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        INFINITIVE_FUT_ACT,
    ]), // Πᾰρῐστημῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Σκᾰνδᾰλῐζω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Μνημονευω
    Cow::Borrowed(&[
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
    ]), // Τεχναομαι
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
    ]), // Δῐοικεω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ῥωννῡμῐ
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
    ]), // Πορσῡνω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἐπᾰϊω
    Cow::Borrowed(&[]), // Πῠρσαινω
    Cow::Borrowed(&[]), // Κᾰθιδρῡω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        INFINITIVE_AOR_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Σαινω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_AOR_ACT,
    ]), // Ἀνᾰγω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_AOR_ACT,
    ]), // Πῡθω
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Δαμνημῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ζευγνῡμῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ὁμᾰρτεω
    Cow::Borrowed(&[
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Χεω
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
        AORIST_IMP_MID_2_SG,
    ]), // Ἀφῑημῐ
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
    ]), // Στραγγᾰλοομαι
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
    ]), // Ζωγρᾰφεω
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
    ]), // Φθειρω
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Ἐξερχομαι
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Δαινῡμῐ
    Cow::Borrowed(&[AORIST_IMP_MID_2_SG]), // Σῑνομαι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἀθῡρω
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
    ]), // Ψῑλοω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_AOR_ACT,
    ]), // Φᾰρμᾰσσω
    Cow::Borrowed(&[]), // Δῐωκω
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
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Κῠω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ψευδω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Πλᾰσσω
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
    ]), // Κερδαινω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Λῡω
    Cow::Borrowed(&[FUTURE_IND_MID_2_SG]), // Μαχομαι
    Cow::Borrowed(&[]), // Λλῡμῐ
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
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Πρεσβευω
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
    ]), // Σοφῐζω
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
    ]), // Βᾰδιζω
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
    ]), // Ἀπειπον
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
        INFINITIVE_PRES_ACT,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_PRES_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Τελεω
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
    ]), // Χραω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Βουλομαι
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
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Θλαω
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
    ]), // Λᾰγοθηρεω
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Εἰρω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_AOR_ACT,
    ]), // Λλᾰμβᾰνω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Θρῠπτω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Σειω
    Cow::Borrowed(&[
        AORIST_SUBJ_ACT_1_SG,
        AORIST_SUBJ_ACT_2_SG,
        AORIST_SUBJ_ACT_3_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Βαινω
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
    ]), // Μηκᾰομαι
    Cow::Borrowed(&[]), // Ἐπεξειμι
    Cow::Borrowed(&[AORIST_IMP_MID_2_SG, INFINITIVE_AOR_ACT]), // Πᾰρεχω
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
    ]), // Θηραω
    Cow::Borrowed(&[
        IND_ACT_2_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        IMPERFECT_ACT_1_SG,
        IMPERFECT_ACT_3_SG,
    ]), // Περιειμι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἀποκωλῡω
    Cow::Borrowed(&[]), // Ἡκω
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
    ]), // Ἱζω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Φρᾰζω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Πρεπω
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
    ]), // Στερεω
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
    ]), // Μᾰρτῠρεω
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
    ]), // Ἐγγῐζω
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
    ]), // Ἀναιρεω
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
    ]), // Ἀνοικῐζω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_AOR_ACT,
    ]), // Κᾰτᾰλᾰμβᾰνω
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Μετερχομαι
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Τελλω
    Cow::Borrowed(&[]), // Φειδομαι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Σκηπτω
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
    ]), // Ποιμαινω
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
    ]), // Βᾰθῠνω
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
    ]), // Προκᾰλεω
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
    ]), // Περαινω
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
    ]), // Ἡγεομαι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Κοπτω
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
    Cow::Borrowed(&[]), // Λειβω
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
    ]), // Ἐξᾰττῐκῐζω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Προφερω
    Cow::Borrowed(&[FUTURE_IND_MID_2_SG]), // Περῐτρεχω
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
    ]), // Δερω
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
    ]), // Γελᾰω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Βῐᾰζομαι
    Cow::Borrowed(&[SUBJ_MID_2_SG, SUBJ_PASS_2_SG]), // Μᾰρνᾰμαι
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
    ]), // Νεκραγωγεω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἀνᾰπτω
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
    ]), // Ἀμεργω
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
    ]), // Πᾰροξῡνω
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
    ]), // Ἐλᾰσσοω
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
    ]), // Ἐρωτᾰω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
    ]), // Πᾰρᾰγῐγνομαι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Κρουω
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
    ]), // Πλουτοκρᾰτεομαι
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
    ]), // Κυκλοω
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
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ψᾱω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Πληττω
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
    ]), // Ἐπαιρω
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
    ]), // Ἀπαιτεω
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
    ]), // Ἐξορκῐζω
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
    ]), // Ὑπᾰνᾰχωρεω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Γρᾰφω
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
    ]), // Ἡλοω
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
    ]), // Πῑαινω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Θερᾰπευω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        IMP_ACT_2_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἀνᾰζω
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
    ]), // Τεινω
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
    ]), // Πλᾰναω
    Cow::Borrowed(&[]), // Ὀμνῠω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_FUT_ACT,
        INFINITIVE_AOR_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Προβαλλω
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
    ]), // Ἐπῐτῐθημῐ
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
    ]), // Κῠβισταω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Φλῡω
    Cow::Borrowed(&[]), // Ἐπευχομαι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Κᾰθελκω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Θλῑβω
    Cow::Borrowed(&[AORIST_IMP_MID_2_SG, INFINITIVE_AOR_ACT]), // Κᾰτεχω
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
    ]), // Σῠνιστημῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἰαινω
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
    ]), // Ἀφῐστημῐ
    Cow::Borrowed(&[]), // Δρᾰσσομαι
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
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ἐξαιρω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἀντᾰνᾰλῡω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_AOR_ACT,
    ]), // Ἁμᾰρτᾰνω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Κολᾰπτω
    Cow::Borrowed(&[
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        PERFECT_SUBJ_ACT_1_SG,
        PERFECT_SUBJ_ACT_2_SG,
        PERFECT_SUBJ_ACT_3_SG,
    ]), // Ἀντῐκᾰθῐστημῐ
    Cow::Borrowed(&[]), // Ἐκγᾰμῐζω
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
    ]), // Ἀντελπῐζω
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
        AORIST_IMP_MID_2_SG,
        INFINITIVE_PRES_ACT,
        INFINITIVE_FUT_ACT,
        INFINITIVE_AOR_ACT,
        PARTICIPLE_PRES_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Σῠναιρεω
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
    ]), // Φῡρω
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
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
        INFINITIVE_PRES_ACT,
        INFINITIVE_AOR_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Σταθμαω
    Cow::Borrowed(&[
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Κνᾰω
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
    ]), // Δηλεομαι
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
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Τρωγλοδῠτεω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Κηδευω
    Cow::Borrowed(&[]), // Λεγομαι
    Cow::Borrowed(&[
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        IMP_ACT_2_SG,
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Ρρεω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Σσῳζω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ρρηγνῡμῐ
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Θρεομαι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Κηρῡσσω
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Ἐφᾰγον
    Cow::Borrowed(&[AORIST_IMP_MID_2_SG, INFINITIVE_AOR_ACT]), // Εἰδον
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Πτᾰρνῠμαι
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ξυμπολεμεω
    Cow::Borrowed(&[]), // Κᾰτακειμαι
    Cow::Borrowed(&[SUBJ_ACT_1_SG, SUBJ_ACT_2_SG, SUBJ_ACT_3_SG]), // Ξῠνοιδᾰ
    Cow::Borrowed(&[]), // Συγκειμαι
    Cow::Borrowed(&[]), // Κορινθιαζομαι
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
    ]), // Στεφανοω
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
    ]), // Ἐπῐχειρεω
    Cow::Borrowed(&[AORIST_IMP_MID_2_SG]), // Συνεπομαι
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
    ]), // Λαλαγεω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ξυνναυμᾰχεω
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
    ]), // Μετᾰκᾰλεω
    Cow::Borrowed(&[]), // Επῐβουλευω
    Cow::Borrowed(&[]), // Ερεω
    Cow::Borrowed(&[]), // Ἐδω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
        AORIST_IMP_MID_2_SG,
    ]), // Αἰσθᾰνομαι
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
    ]), // Τρυπαω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Μειδᾰω
    Cow::Borrowed(&[
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἀνᾰχρονῐζομαι
    Cow::Borrowed(&[FUTURE_IND_MID_2_SG]), // Σῠμμᾰχομαι
    Cow::Borrowed(&[]), // Κᾰθημαι
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
    ]), // Ἐπῐκᾰλεω
    Cow::Borrowed(&[SUBJ_ACT_3_SG]), // Χρη
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Ἠλθον
    Cow::Borrowed(&[AORIST_IMP_MID_2_SG]), // Ἀρνυμαι
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Ἐπεφνον
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Μετεκῑᾰθον
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Πεπνῡμαι
    Cow::Borrowed(&[]), // Περνημῐ
    Cow::Borrowed(&[
        SUBJ_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἐπῐστᾰμαι
    Cow::Borrowed(&[IMPERFECT_MID_2_SG, IMPERFECT_PASS_2_SG]), // Ἀπαυραω
    Cow::Borrowed(&[]), // Προκειμαι
    Cow::Borrowed(&[]), // Ἐπῐπειθομαι
    Cow::Borrowed(&[]), // Ἐπισευω
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
    ]), // Δειμᾰτοω
    Cow::Borrowed(&[]), // Κορυσσω
    Cow::Borrowed(&[AORIST_IMP_MID_2_SG, INFINITIVE_AOR_ACT]), // Κεκᾰδον
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
    ]), // Κᾰτᾰκλαω
    Cow::Borrowed(&[FUTURE_IND_MID_2_SG]), // Ἀγωνιζομαι
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
    ]), // Ἀπιστεω
    Cow::Borrowed(&[
        FUTURE_IND_ACT_1_SG,
        FUTURE_IND_ACT_2_SG,
        FUTURE_IND_ACT_3_SG,
        INFINITIVE_FUT_ACT,
        PARTICIPLE_FUT_ACT,
    ]), // Χᾰριζω
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
    ]), // Διαφορεω
    Cow::Borrowed(&[]), // Τελεθω
    Cow::Borrowed(&[]), // Δῐᾰσευομαι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Φθιω
    Cow::Borrowed(&[
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
    ]), // Οικειοομαι
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
    ]), // Συγκᾰκουργεω
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἀποστατεω
    Cow::Borrowed(&[]), // Γεγονοι
    Cow::Borrowed(&[]), // Γεγενησαι
    Cow::Borrowed(&[]), // Γεγενηται
    Cow::Borrowed(&[]), // Γεγενησθω
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
    Cow::Borrowed(&[]), // Τεθνηκετω
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
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἀντιπαρατασσομαι
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Ἐπᾰνερχομαι
    Cow::Borrowed(&[SUBJ_MID_2_SG, SUBJ_PASS_2_SG]), // Κιδνᾰμαι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἐπισκεπτομαι
    Cow::Borrowed(&[
        IND_ACT_1_SG,
        IND_ACT_2_SG,
        IND_ACT_3_SG,
        SUBJ_ACT_1_SG,
        SUBJ_ACT_2_SG,
        SUBJ_ACT_3_SG,
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Κολυμβαω
    Cow::Borrowed(&[INFINITIVE_AOR_ACT]), // Δῐκειν
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
    ]), // Ὑπονοεω
    Cow::Borrowed(&[
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἀντᾰγωνιζομαι
    Cow::Borrowed(&[
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
    ]), // Διπλᾰσῐοομαι
    Cow::Borrowed(&[
        IND_MID_2_SG,
        IND_PASS_2_SG,
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        IMP_MID_2_SG,
        IMP_PASS_2_SG,
        IMPERFECT_MID_2_SG,
        IMPERFECT_PASS_2_SG,
        AORIST_IMP_MID_2_SG,
    ]), // Ἐξικνεομαι
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
    ]), // Καταπετροω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Λαλαχευομαι
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
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Μικρογρᾰφεω
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
    ]), // Λῐνοστᾰτεω
    Cow::Borrowed(&[]), // Ἀδησᾰ
    Cow::Borrowed(&[FUTURE_IND_MID_2_SG]), // Ὑποκοριζομαι
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
        INFINITIVE_PRES_ACT,
        PARTICIPLE_PRES_ACT,
    ]), // Ἰχᾰνᾰω
    Cow::Borrowed(&[
        FUTURE_IND_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ᾰντᾰποκρῑνομαι
    Cow::Borrowed(&[
        SUBJ_MID_2_SG,
        SUBJ_PASS_2_SG,
        AORIST_SUBJ_MID_2_SG,
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ὑπεκτιθεμαι
    Cow::Borrowed(&[]), // Μᾰστῐγοω
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Βᾰρβᾰροομαι
    Cow::Borrowed(&[
        AORIST_SUBJ_PASS_1_SG,
        AORIST_SUBJ_PASS_2_SG,
        AORIST_SUBJ_PASS_3_SG,
    ]), // Ἐπιμιμνησκομαι
    Cow::Borrowed(&[]), // Εργεω
    Cow::Borrowed(&[]), // Ᾰνθρωπεω
    Cow::Borrowed(&[]), // Εισκᾰτοικεω
    Cow::Borrowed(&[]), // Εκπλεω
    Cow::Borrowed(&[]), // Δειδῐᾰ
    Cow::Borrowed(&[]), // Λανθανομαι
    Cow::Borrowed(&[]), // Τρᾰπω
];

/// The cells whose final `-αι`, `-οι` counts as long.
pub static LONG_FINALS: [u16; 5] = [
    OPT_ACT_3_SG,
    FUTURE_OPT_ACT_3_SG,
    AORIST_OPT_ACT_3_SG,
    PERFECT_OPT_ACT_3_SG,
    FUTURE_PERFECT_OPT_ACT_3_SG,
];
