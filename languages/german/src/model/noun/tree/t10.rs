use flexion::Path;

/// Words ending -der.
pub(crate) fn n2638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2634);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9250);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19357);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 19358);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 19359);
					match s {
						[] => true,
						['g', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 29079);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 29080);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 29081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 19360);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 29082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 19361);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36126);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 36127);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 19362);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 19363);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 19364);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 19365);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 29085);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 36128);
									s.is_empty()
								}
								['e', 'u', s @ ..] => {
									super::push(out, 36129);
									match s {
										[] => true,
										['q', '\u{1}', s @ ..] => {
											super::push(out, 40587);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'e', 's', s @ ..] => {
					super::push(out, 19366);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 29086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 19367);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29087);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 29088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19368);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 19369);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 19370);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 19371);
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36130);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 36131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 19372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9251);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 19373);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 19374);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 29089);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 29090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9252);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19375);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 19376);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 29091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 19377);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 19378);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 29092);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 29093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19379);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 29094);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36133);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 36134);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 29095);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 29096);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 29097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 19380);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 19381);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							super::push(out, 29098);
							match s {
								[] => true,
								['t', 'h', s @ ..] => {
									super::push(out, 36135);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 19382);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 19383);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19384);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 19385);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 19386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9253);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 19387);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 19388);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 19389);
					match s {
						[] => true,
						['l', 'a', 'r', 't', 's', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 29100);
							s.is_empty()
						}
						['n', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 29101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 19390);
					match s {
						[] => true,
						['o', 'r', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 29102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'g', s @ ..] => {
					super::push(out, 19391);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 29103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 19392);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 19393);
					match s {
						[] => true,
						['p', 'o', 'r', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 29104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9254);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 19394);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 29105);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 36136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19395);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 29106);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 29107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 19396);
					s.is_empty()
				}
				['u', 'm', s @ ..] => {
					super::push(out, 19397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 19398);
					match s {
						[] => true,
						['w', 'r', 'e', 't', 'n', s @ ..] => {
							super::push(out, 29109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9255);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 19399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t14::n9365(s, out),
		['o', s @ ..] => {
			super::push(out, 9257);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19410);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 19411);
					match s {
						[] => true,
						['r', 'e', 'd', 'e', 'w', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 29139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t14::n19965(s, out),
				['m', s @ ..] => {
					super::push(out, 19413);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t14::n9367(s, out),
		['u', s @ ..] => super::t14::n9368(s, out),
		['ä', s @ ..] => super::t14::n9369(s, out),
		['ö', s @ ..] => super::t14::n9370(s, out),
		['ü', s @ ..] => super::t14::n9371(s, out),
		_ => false,
	}
}

/// Words ending -eer.
pub(crate) fn n2639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2635);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 9263);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19430);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19431);
					match s {
						[] => true,
						['o', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36183);
									s.is_empty()
								}
								['g', 'r', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 36184);
									s.is_empty()
								}
								['h', 'c', 's', 'r', 'i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 36185);
									s.is_empty()
								}
								['s', 's', 'u', 'h', 'c', 's', 'r', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 36186);
									s.is_empty()
								}
								['ß', 'u', 'h', 'c', 's', 'r', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 36187);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9264);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 19432);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 29156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 19433);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29157);
							match s {
								[] => true,
								['g', 'i', 'l', 'l', 'i', 'w', 'i', 'e', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 36188);
									s.is_empty()
								}
								['s', 'o', 'l', 's', 't', 'i', 'e', 'b', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 36189);
									s.is_empty()
								}
								['t', 's', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 40595);
											s.is_empty()
										}
										['r', 'ü', 'f', '\u{1}', s @ ..] => {
											super::push(out, 40596);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 29158);
							match s {
								[] => true,
								['i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 36190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'u', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 29159);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 29160);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 36191);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 19434);
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							super::push(out, 29161);
							match s {
								[] => true,
								['r', 'ü', 'b', '\u{1}', s @ ..] => {
									super::push(out, 36192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9265);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 19436);
					match s {
						[] => true,
						['n', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 29162);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 29163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 19437);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 29164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['b', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 29165);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 29166);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['m', 'a', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 36193);
									s.is_empty()
								}
								['u', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 36194);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 29167);
							match s {
								[] => true,
								['n', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 36195);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 36196);
									match s {
										[] => true,
										['e', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 40597);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 29168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 19438);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 29169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 19439);
					match s {
						[] => true,
						['e', 't', 'h', 'c', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 29170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19440);
					s.is_empty()
				}
				['t', 'l', s @ ..] => {
					super::push(out, 19441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9266);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19442);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 19443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 9267);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19444);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 19445);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29171);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 29172);
							s.is_empty()
						}
						['l', 'a', 'a', '\u{1}', s @ ..] => {
							super::push(out, 29173);
							s.is_empty()
						}
						['n', 'e', 's', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 29174);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 29175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9268);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9269);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 19446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 9270);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 19447);
					match s {
						[] => true,
						['l', 'k', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 29176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 19448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 9271);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 9272);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 19449);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fer.
pub(crate) fn n2640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2636);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9273);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 19450);
					match s {
						[] => true,
						['-', 't', 'a', s @ ..] => {
							super::push(out, 29177);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 29178);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 29179);
							s.is_empty()
						}
						['t', 'a', s @ ..] => {
							super::push(out, 29180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 19451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9274);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 29181);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 36197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 29182);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 36198);
									match s {
										[] => true,
										['e', 'h', 'c', 's', 'i', s @ ..] => {
											super::push(out, 40598);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 36199);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 40599);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 29183);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 29184);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36200);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 9275);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19452);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 29185);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36201);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 29186);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 29187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 19453);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 29188);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 40600);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 40601);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 29189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19454);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 29190);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 29191);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 29192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 19455);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 19456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 9276);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 19457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9277);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 19458);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 29193);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 29194);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 29195);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36203);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 36204);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 29196);
							s.is_empty()
						}
						['n', 'r', s @ ..] => {
							super::push(out, 29197);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36205);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 36206);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 36207);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 29198);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 29199);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 36208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 19459);
					match s {
						[] => true,
						['q', 'a', '\u{1}', s @ ..] => {
							super::push(out, 29200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 19460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9278);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9279);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9280);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 19461);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 9281);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 19462);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29202);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 36209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 29203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 19463);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 29204);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 29205);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36210);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 36211);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 36212);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 36213);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 36214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 29206);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36215);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 36216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 29207);
							match s {
								[] => true,
								['s', s @ ..] => super::t14::n38232(s, out),
								_ => false,
							}
						}
						['t', 's', s @ ..] => super::t14::n30493(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t14::n20019(s, out),
				['ö', s @ ..] => super::t14::n20020(s, out),
				['ü', s @ ..] => super::t14::n20021(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t14::n9391(s, out),
		['s', s @ ..] => super::t14::n9392(s, out),
		['u', s @ ..] => super::t14::n9393(s, out),
		['ä', s @ ..] => super::t14::n9394(s, out),
		['ü', s @ ..] => super::t14::n9395(s, out),
		_ => false,
	}
}

/// Words ending -ger.
pub(crate) fn n2641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2637);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9287);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 9288);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 19484);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 19485);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 29218);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									super::push(out, 36220);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 40602);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 29219);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36221);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 36222);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 29220);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 29221);
							match s {
								[] => true,
								['n', 'o', 'i', 't', 'i', s @ ..] => {
									super::push(out, 36223);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'e', 'i', s @ ..] => {
							super::push(out, 29222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 19486);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 19487);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19488);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 19489);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 19490);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 29223);
							match s {
								[] => true,
								['c', 's', 'r', 'e', 'g', 'e', 'i', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 36224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9289);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 19491);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 29224);
							match s {
								[] => true,
								['h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40603);
											s.is_empty()
										}
										['n', 'e', 'g', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 40604);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 19492);
					match s {
						[] => true,
						['f', 'p', 'n', 'e', 'l', s @ ..] => {
							super::push(out, 29225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 9290);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9291);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 29226);
							s.is_empty()
						}
						['u', 'ä', 'l', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36225);
									s.is_empty()
								}
								['d', 'n', 'a', 'f', 'p', '\u{1}', s @ ..] => {
									super::push(out, 36226);
									s.is_empty()
								}
								['e', 'h', 'i', 'e', 'l', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 36227);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 36228);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 19493);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29227);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 29228);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 29229);
							match s {
								[] => true,
								['u', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 36229);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 29230);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 36230);
									s.is_empty()
								}
								['ä', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 40605);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 40606);
											match s {
												[] => true,
												['s', 'r', 'e', 'v', 't', s @ ..] => {
													super::push(out, 43095);
													match s {
														[] => true,
														['s', 'n', 'u', 'k', '\u{1}', s @ ..] => super::t14::n47220(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ü', s @ ..] => {
									super::push(out, 36231);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 29231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 19494);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 29232);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 29233);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 29234);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 29235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 19495);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 19496);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 19497);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 19498);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 19499);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 19500);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 29237);
							s.is_empty()
						}
						['n', s @ ..] => super::t14::n30525(s, out),
						['u', s @ ..] => super::t14::n30526(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t14::n20057(s, out),
				['s', s @ ..] => super::t14::n20058(s, out),
				['t', s @ ..] => super::t14::n20059(s, out),
				['w', s @ ..] => super::t14::n20060(s, out),
				['z', s @ ..] => super::t14::n20061(s, out),
				['ß', s @ ..] => super::t14::n20062(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t14::n9401(s, out),
		['n', s @ ..] => super::t14::n9402(s, out),
		['r', s @ ..] => super::t14::n9403(s, out),
		['u', s @ ..] => super::t14::n9404(s, out),
		['ä', s @ ..] => super::t14::n9405(s, out),
		['ü', s @ ..] => super::t14::n9406(s, out),
		_ => false,
	}
}

/// Words ending -her.
pub(crate) fn n2642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2638);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 9298);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19519);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 29271);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 29272);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									super::push(out, 36249);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 40616);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 29273);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 29274);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 36250);
									match s {
										[] => true,
										['c', 's', '\u{1}', s @ ..] => {
											super::push(out, 40617);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 19520);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 19521);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29275);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 36251);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 36252);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40618);
											s.is_empty()
										}
										['a', 'r', 't', 'l', 'u', '\u{1}', s @ ..] => {
											super::push(out, 40619);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 40620);
											match s {
												[] => true,
												['l', 'o', 'f', 'r', 'e', '\u{1}', s @ ..] => {
													super::push(out, 43102);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', s @ ..] => {
											match s {
												[] => true,
												['p', s @ ..] => {
													super::push(out, 43103);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 43104);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 40621);
											match s {
												[] => true,
												['s', 'u', 'l', 'f', 'n', 'i', 'e', '\u{1}', s @ ..] => {
													super::push(out, 43105);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 40622);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 40623);
											match s {
												[] => true,
												['e', 'n', '\u{1}', s @ ..] => {
													super::push(out, 43106);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 29276);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 29277);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 36254);
									s.is_empty()
								}
								['ü', s @ ..] => {
									super::push(out, 36255);
									match s {
										[] => true,
										['j', '\u{1}', s @ ..] => {
											super::push(out, 40624);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 19522);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 29278);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 29279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19523);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29280);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 36256);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 29281);
							match s {
								[] => true,
								['ü', 'h', '\u{1}', s @ ..] => {
									super::push(out, 36257);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 29282);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 29283);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 36258);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t14::n30572(s, out),
						['l', s @ ..] => super::t14::n30573(s, out),
						['m', s @ ..] => super::t14::n30574(s, out),
						['n', s @ ..] => super::t14::n30575(s, out),
						['r', s @ ..] => super::t14::n30576(s, out),
						['t', s @ ..] => super::t14::n30577(s, out),
						['u', s @ ..] => super::t14::n30578(s, out),
						['ä', s @ ..] => super::t14::n30579(s, out),
						['ö', s @ ..] => super::t14::n30580(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t14::n20080(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t14::n9408(s, out),
		['i', s @ ..] => super::t14::n9409(s, out),
		['p', s @ ..] => super::t14::n9410(s, out),
		['t', s @ ..] => super::t14::n9411(s, out),
		['ä', s @ ..] => super::t14::n9412(s, out),
		_ => false,
	}
}

/// Words ending -ier.
pub(crate) fn n2643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2639);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9304);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 19537);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 19538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 9305);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19539);
					s.is_empty()
				}
				['d', 'n', 'e', 'b', 'a', 'r', 'e', 'i', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 19540);
					s.is_empty()
				}
				['e', 'g', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 19541);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 19542);
					match s {
						[] => true,
						['e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 29306);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 36281);
									s.is_empty()
								}
								['n', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 36282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', 'u', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 29307);
							s.is_empty()
						}
						['ü', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 29308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19543);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29309);
							match s {
								[] => true,
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 36283);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 29310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 19544);
					match s {
						[] => true,
						['c', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 29311);
							s.is_empty()
						}
						['r', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 29312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 19545);
					match s {
						[] => true,
						['e', 'd', 'n', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 29313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 19546);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29314);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 29315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 19547);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 36284);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 36285);
									match s {
										[] => true,
										['s', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 40640);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 19548);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 29316);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40641);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 40642);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 19549);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29317);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 36286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 29318);
							match s {
								[] => true,
								['t', 'a', s @ ..] => {
									super::push(out, 36287);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19550);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29319);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 36288);
									s.is_empty()
								}
								['m', 'r', 'i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 36289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'n', 'i', 'l', s @ ..] => {
							match s {
								[] => true,
								['b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 36290);
									s.is_empty()
								}
								['h', 'ü', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 36291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 36292);
									s.is_empty()
								}
								['i', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 36293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 19551);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 19552);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 19553);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 19554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 9306);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 19555);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 19556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9307);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 29320);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 36294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 29321);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 36295);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 36296);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 40643);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29322);
							match s {
								[] => true,
								['b', 'e', 'l', 'l', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 36297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 29323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9308);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 19557);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 19558);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 29324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 19559);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29325);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 36298);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 36299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 19560);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 29326);
							s.is_empty()
						}
						['r', s @ ..] => super::t14::n30621(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 19561);
					s.is_empty()
				}
				['p', s @ ..] => super::t14::n20121(s, out),
				['r', s @ ..] => super::t14::n20122(s, out),
				['s', s @ ..] => super::t14::n20123(s, out),
				['w', s @ ..] => super::t14::n20124(s, out),
				['z', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => super::t14::n20125(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t14::n9418(s, out),
		['g', s @ ..] => super::t14::n9419(s, out),
		['h', s @ ..] => super::t14::n9420(s, out),
		['k', s @ ..] => super::t14::n9421(s, out),
		['l', s @ ..] => super::t14::n9422(s, out),
		['m', s @ ..] => super::t14::n9423(s, out),
		['n', s @ ..] => super::t14::n9424(s, out),
		['o', s @ ..] => super::t14::n9425(s, out),
		['p', s @ ..] => super::t14::n9426(s, out),
		['r', s @ ..] => super::t14::n9427(s, out),
		['s', s @ ..] => super::t14::n9428(s, out),
		['t', s @ ..] => super::t14::n9429(s, out),
		['v', s @ ..] => super::t14::n9430(s, out),
		['w', s @ ..] => super::t14::n9431(s, out),
		['x', s @ ..] => super::t14::n9432(s, out),
		['z', s @ ..] => super::t14::n9433(s, out),
		_ => false,
	}
}

/// Words ending -jer.
pub(crate) fn n2644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2640);
	s.is_empty()
}

/// Words ending -ker.
pub(crate) fn n2645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2641);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9325);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 19648);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19649);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 19650);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 29443);
							match s {
								[] => true,
								['a', 'm', '-', s @ ..] => {
									super::push(out, 36384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 9326);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29444);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 29445);
							match s {
								[] => true,
								['o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 36385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 29446);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36386);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 40689);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 29447);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 29448);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 36387);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 29449);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36388);
									match s {
										[] => true,
										['s', 'e', 'i', 'r', '\u{1}', s @ ..] => {
											super::push(out, 40690);
											s.is_empty()
										}
										['t', 'o', 't', '\u{1}', s @ ..] => {
											super::push(out, 40691);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 29450);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 29451);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 19652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29452);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 29453);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 29454);
							match s {
								[] => true,
								['c', 'u', s @ ..] => {
									super::push(out, 36390);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 29455);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36391);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 40692);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 36392);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 29456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19653);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 29457);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36393);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 29458);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 29459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 19654);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 29460);
							s.is_empty()
						}
						['h', 'f', s @ ..] => {
							super::push(out, 29461);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 36394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 19655);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 29462);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 29463);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 29464);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 29465);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 29466);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 29467);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36395);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 36396);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 36397);
									s.is_empty()
								}
								['l', 'e', 'f', 'f', s @ ..] => {
									super::push(out, 36398);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 43134);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 43135);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 40693);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 36399);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', 'i', s @ ..] => {
											super::push(out, 40694);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 40695);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', 'l', 'a', s @ ..] => {
									super::push(out, 36400);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 19656);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 19657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9327);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9328);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 19658);
					match s {
						[] => true,
						['o', 'n', s @ ..] => {
							super::push(out, 29468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9329);
			match s {
				[] => true,
				['ö', s @ ..] => {
					super::push(out, 19659);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 29469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t14::n9439(s, out),
		['o', s @ ..] => {
			super::push(out, 9331);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 19664);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 19665);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 29479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => super::t14::n20226(s, out),
				['r', s @ ..] => {
					super::push(out, 19667);
					s.is_empty()
				}
				['w', s @ ..] => super::t14::n20228(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t14::n9441(s, out),
		['s', s @ ..] => {
			super::push(out, 9333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ler.
pub(crate) fn n2646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2642);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9334);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 19671);
					match s {
						[] => true,
						['o', 'n', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 29486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 19672);
					s.is_empty()
				}
				['g', 'e', 'l', s @ ..] => {
					super::push(out, 19673);
					match s {
						[] => true,
						['l', 'i', '\u{1}', s @ ..] => {
							super::push(out, 29487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 19674);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 19675);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 29488);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 29489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 19676);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 29490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 19677);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 29491);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 36405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', 'm', 'i', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 29492);
							s.is_empty()
						}
						['o', 'i', s @ ..] => {
							super::push(out, 29493);
							match s {
								[] => true,
								['t', 'a', 'n', 'h', 'c', 's', 't', 'u', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 36406);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 19678);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 19679);
					s.is_empty()
				}
				['u', 'x', 'e', 's', 'o', 'm', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 19680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 9335);
			match s {
				[] => true,
				['b', 'o', s @ ..] => {
					super::push(out, 19681);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 29495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9336);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19682);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 29496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9337);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 9338);
			match s {
				[] => true,
				['e', 'i', 'r', s @ ..] => {
					super::push(out, 19683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9339);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9340);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 19684);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 19685);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 19686);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 29498);
							match s {
								[] => true,
								['o', 'n', s @ ..] => {
									super::push(out, 36407);
									match s {
										[] => true,
										['i', 'v', '\u{1}', s @ ..] => {
											super::push(out, 40697);
											s.is_empty()
										}
										['ö', '\u{1}', s @ ..] => {
											super::push(out, 40698);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 9341);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 9342);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19687);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 29499);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 36408);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 40699);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 19688);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 29500);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 36409);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 29501);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 29502);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 36410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 29503);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 29504);
							match s {
								[] => true,
								['q', s @ ..] => {
									super::push(out, 36411);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 36412);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 29505);
							match s {
								[] => true,
								['a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 36413);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 36414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 29506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 29507);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 29508);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 29509);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 36415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 19689);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 29510);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 36416);
									s.is_empty()
								}
								['r', 'e', 't', s @ ..] => {
									super::push(out, 36417);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 29511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 19690);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 29512);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 29513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 19691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9343);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9344);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 9345);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9346);
			match s {
				[] => true,
				['s', 'a', s @ ..] => {
					super::push(out, 19692);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 29514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 9347);
			match s {
				[] => true,
				['s', 'n', s @ ..] => {
					super::push(out, 19693);
					match s {
						[] => true,
						['ü', 'k', 'n', 'e', s @ ..] => {
							super::push(out, 29515);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 36419);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9348);
			match s {
				[] => true,
				['a', 'f', s @ ..] => {
					super::push(out, 19694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'm', s @ ..] => {
					super::push(out, 19695);
					match s {
						[] => true,
						['a', 'n', 'm', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 29517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 19696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 9349);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 9350);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mer.
pub(crate) fn n2647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2643);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9351);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 19697);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 29518);
							match s {
								[] => true,
								['e', 't', '\u{1}', s @ ..] => {
									super::push(out, 36420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19698);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 29519);
							match s {
								[] => true,
								['i', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 36421);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9352);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 19699);
					match s {
						[] => true,
						['c', 'o', 'h', s @ ..] => {
							super::push(out, 29520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9353);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19700);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 29521);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 19701);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 19702);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 19703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9354);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 19704);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 19705);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36423);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 40700);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43136);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 36424);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 40701);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 43137);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 19706);
					match s {
						[] => true,
						['p', 'h', s @ ..] => {
							super::push(out, 29523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', s @ ..] => {
					super::push(out, 19707);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 29524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9355);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 9356);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19708);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 29525);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 29526);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 36425);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40702);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'c', 'u', 'a', s @ ..] => {
									super::push(out, 36426);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 40703);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 36427);
									match s {
										[] => true,
										['e', 'x', s @ ..] => {
											super::push(out, 40704);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', s @ ..] => {
									super::push(out, 36428);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 40705);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 40706);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 36429);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 29527);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 36430);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 36431);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 36432);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 29528);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 36433);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 36434);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 40707);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 29529);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 29530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 19709);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 29531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19710);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 29532);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 29533);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 36435);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 29534);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 36436);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 40708);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 29535);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 29536);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 36437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 29537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t14::n20273(s, out),
				['u', s @ ..] => super::t14::n20274(s, out),
				['ä', s @ ..] => super::t14::n20275(s, out),
				['ü', s @ ..] => super::t14::n20276(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t14::n9466(s, out),
		['r', s @ ..] => super::t14::n9467(s, out),
		['u', s @ ..] => super::t14::n9468(s, out),
		['y', s @ ..] => super::t14::n9469(s, out),
		['ä', s @ ..] => {
			super::push(out, 9361);
			s.is_empty()
		}
		['ö', s @ ..] => super::t14::n9471(s, out),
		['ü', s @ ..] => super::t14::n9472(s, out),
		_ => false,
	}
}

/// Words ending -ner.
pub(crate) fn n2648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2644);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9364);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 9365);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 9366);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9367);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 19729);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', 'r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 36447);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 36448);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 40709);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 40710);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 36449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 29563);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 29564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 19730);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 29565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 36450);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 36451);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 29566);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 29567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29568);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 29569);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 29570);
							s.is_empty()
						}
						['r', 'o', s @ ..] => {
							super::push(out, 29571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 19731);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29572);
							s.is_empty()
						}
						['e', 'l', 'r', s @ ..] => {
							super::push(out, 29573);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 29574);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 29575);
							s.is_empty()
						}
						['o', 'r', 't', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 29576);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 29577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 19732);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 36452);
									match s {
										[] => true,
										['l', 'b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 40711);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 36453);
									match s {
										[] => true,
										['r', 'p', 's', 'e', 'g', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 40712);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 36454);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 29578);
							match s {
								[] => true,
								['l', 'f', 't', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 36455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19733);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 29579);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 29580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 19734);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 29581);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 29582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['h', 'o', s @ ..] => {
							super::push(out, 29583);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 29584);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 40713);
											s.is_empty()
										}
										['g', '-', 't', 'k', 'n', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 40714);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'm', s @ ..] => {
					super::push(out, 19735);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 19736);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19737);
					match s {
						[] => true,
						['i', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 29585);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 29586);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 29587);
							match s {
								[] => true,
								['m', 'l', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 36456);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 29588);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 36457);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19738);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 29589);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 29590);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 29591);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 29592);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 36458);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 36459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 29593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 19739);
					match s {
						[] => true,
						['n', 'a', 'x', '\u{1}', s @ ..] => {
							super::push(out, 29594);
							s.is_empty()
						}
						['p', 'm', 'e', 'k', '\u{1}', s @ ..] => {
							super::push(out, 29595);
							s.is_empty()
						}
						['r', 'a', 'g', 'n', 'i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 29596);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 29597);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 36460);
									s.is_empty()
								}
								['ü', s @ ..] => {
									super::push(out, 36461);
									match s {
										[] => true,
										['h', 's', 'a', 'l', 'g', '\u{1}', s @ ..] => {
											super::push(out, 40715);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 19740);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 19741);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 19742);
					match s {
						[] => true,
						['e', 'i', 'g', '\u{1}', s @ ..] => super::t14::n30903(s, out),
						['o', s @ ..] => super::t14::n30904(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t14::n9477(s, out),
		['g', s @ ..] => super::t14::n9478(s, out),
		['h', s @ ..] => super::t14::n9479(s, out),
		['i', s @ ..] => super::t14::n9480(s, out),
		['k', s @ ..] => super::t14::n9481(s, out),
		['l', s @ ..] => super::t14::n9482(s, out),
		['n', s @ ..] => super::t14::n9483(s, out),
		['o', s @ ..] => super::t14::n9484(s, out),
		['r', s @ ..] => super::t14::n9485(s, out),
		['s', s @ ..] => super::t14::n9486(s, out),
		['t', s @ ..] => super::t14::n9487(s, out),
		['u', s @ ..] => super::t14::n9488(s, out),
		['ö', 'h', s @ ..] => super::t14::n9489(s, out),
		['ü', s @ ..] => super::t14::n9490(s, out),
		_ => false,
	}
}

/// Words ending -oer.
pub(crate) fn n2649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2645);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 9381);
			match s {
				[] => true,
				['k', 'o', 't', '\u{1}', s @ ..] => {
					super::push(out, 19765);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -per.
pub(crate) fn n2650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2646);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9382);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 19766);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 19767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9383);
			match s {
				[] => true,
				['i', 'j', s @ ..] => {
					super::push(out, 19768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9384);
			match s {
				[] => true,
				['n', 's', '\u{1}', s @ ..] => {
					super::push(out, 19769);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 19770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9385);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19771);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 19772);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 29620);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 29621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 19773);
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 29622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9386);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 19774);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 29623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 19775);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 29624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 9387);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19776);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 29625);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40718);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 40719);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 40720);
											s.is_empty()
										}
										['n', 'r', 'a', 'w', '\u{1}', s @ ..] => {
											super::push(out, 40721);
											s.is_empty()
										}
										['s', 'u', 'r', 'a', 'z', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 40722);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 36475);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 29626);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 29627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 19777);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 29628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19778);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 19779);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 19780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9388);
			match s {
				[] => true,
				['ö', 'k', 'l', s @ ..] => {
					super::push(out, 19781);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29629);
							match s {
								[] => true,
								['r', 't', 's', 'a', '\u{1}', s @ ..] => {
									super::push(out, 36476);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 9389);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19782);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 19783);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 19784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9390);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 19785);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 19786);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rer.
pub(crate) fn n2651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2647);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9391);
			match s {
				[] => true,
				['b', 't', s @ ..] => {
					super::push(out, 19787);
					match s {
						[] => true,
						['h', 'c', 'i', 's', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 29630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 19788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9392);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 19789);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 29631);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36477);
									s.is_empty()
								}
								['t', 'd', 'a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 36478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 19790);
					match s {
						[] => true,
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 29632);
							s.is_empty()
						}
						['r', 'o', s @ ..] => {
							super::push(out, 29633);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 36479);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 36480);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 40723);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', 'p', s @ ..] => {
					super::push(out, 19791);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', s @ ..] => {
					super::push(out, 19792);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 29635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19793);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 19794);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 29636);
							match s {
								[] => true,
								['ä', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 36482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19795);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 19796);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29637);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 36483);
									match s {
										[] => true,
										['l', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 40724);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 29638);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 36484);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9393);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9394);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19797);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 19798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9395);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ser.
pub(crate) fn n2652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2648);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9396);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19799);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 19800);
					s.is_empty()
				}
				['e', 't', '\u{1}', s @ ..] => {
					super::push(out, 19801);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 19802);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 19803);
					s.is_empty()
				}
				['h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 19804);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 19805);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19806);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 19807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 9397);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9398);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 9399);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9400);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19808);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 29639);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 29640);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36486);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 19809);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36487);
									s.is_empty()
								}
								['s', 't', 'f', 'a', 'h', 'c', 's', 't', 'r', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 36488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 9401);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 9402);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 9403);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9404);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 19810);
					match s {
						[] => true,
						['l', 'l', s @ ..] => {
							super::push(out, 29641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 19811);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 29642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9405);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 19812);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 19813);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 19814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29645);
							s.is_empty()
						}
						['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 29646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 9406);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9407);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					super::push(out, 19815);
					match s {
						[] => true,
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 29647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 9408);
			match s {
				[] => true,
				['a', s @ ..] => super::t14::n20381(s, out),
				['e', s @ ..] => super::t14::n20382(s, out),
				['i', s @ ..] => {
					super::push(out, 19818);
					match s {
						[] => true,
						['e', 'w', s @ ..] => super::t14::n30969(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t14::n20384(s, out),
				['u', s @ ..] => super::t14::n20385(s, out),
				['ä', s @ ..] => super::t14::n20386(s, out),
				['ö', s @ ..] => super::t14::n20387(s, out),
				['ü', s @ ..] => super::t14::n20388(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t14::n9519(s, out),
		['w', s @ ..] => super::t14::n9520(s, out),
		['y', s @ ..] => super::t14::n9521(s, out),
		['ä', s @ ..] => super::t14::n9522(s, out),
		['ö', s @ ..] => super::t14::n9523(s, out),
		_ => false,
	}
}

/// Words ending -ter.
pub(crate) fn n2653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2649);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9414);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 19830);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 29671);
							match s {
								[] => true,
								['e', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 36511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 29672);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 29673);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 29674);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 36512);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 19831);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19832);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 19833);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 19834);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19835);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29676);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 29677);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 29678);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 36513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 19836);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 29679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 19837);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 29680);
							s.is_empty()
						}
						['t', 't', s @ ..] => {
							super::push(out, 29681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 19838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 9415);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 9416);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19839);
					match s {
						[] => true,
						['r', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 29682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9417);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 19840);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 19841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9418);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19842);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 29684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 19843);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 19844);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 19845);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 29685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19846);
					s.is_empty()
				}
				['m', s @ ..] => super::t14::n20412(s, out),
				['n', s @ ..] => {
					super::push(out, 19848);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 29698);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 29699);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 29700);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 29701);
							match s {
								[] => true,
								['u', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 36540);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 19849);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 36541);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 36542);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 29702);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 36543);
									match s {
										[] => true,
										['s', 'n', s @ ..] => {
											super::push(out, 40792);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 36544);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 29703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 19850);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 29704);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 29705);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36545);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 19851);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 19852);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t14::n9529(s, out),
		['g', s @ ..] => super::t14::n9530(s, out),
		['h', s @ ..] => super::t14::n9531(s, out),
		['i', s @ ..] => super::t14::n9532(s, out),
		['k', s @ ..] => super::t14::n9533(s, out),
		['l', s @ ..] => super::t14::n9534(s, out),
		['m', s @ ..] => super::t14::n9535(s, out),
		['n', s @ ..] => super::t14::n9536(s, out),
		['o', s @ ..] => super::t14::n9537(s, out),
		['p', s @ ..] => super::t14::n9538(s, out),
		['r', s @ ..] => super::t14::n9539(s, out),
		['s', s @ ..] => super::t14::n9540(s, out),
		['t', s @ ..] => super::t14::n9541(s, out),
		['u', s @ ..] => super::t14::n9542(s, out),
		['x', s @ ..] => {
			super::push(out, 9433);
			s.is_empty()
		}
		['z', s @ ..] => super::t14::n9544(s, out),
		['ä', s @ ..] => super::t14::n9545(s, out),
		['ö', s @ ..] => {
			super::push(out, 9436);
			s.is_empty()
		}
		['ü', s @ ..] => super::t14::n9547(s, out),
		_ => false,
	}
}

/// Words ending -uer.
pub(crate) fn n2654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2650);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9438);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 19940);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29914);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 29915);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 29916);
							match s {
								[] => true,
								['l', 's', 'u', 'ä', 'h', '\u{1}', s @ ..] => {
									super::push(out, 36727);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['a', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 36728);
									s.is_empty()
								}
								['e', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 36729);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'u', 'e', 'z', 'g', 'u', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 29917);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 40885);
											s.is_empty()
										}
										['l', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 40886);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 36730);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 36731);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 29918);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 29919);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 36732);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 40887);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 36733);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 40888);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 40889);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 43216);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 43217);
													s.is_empty()
												}
												['ü', s @ ..] => {
													super::push(out, 43218);
													s.is_empty()
												}
												_ => false,
											}
										}
										['p', s @ ..] => {
											super::push(out, 40890);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'u', 'ä', 'h', '\u{1}', s @ ..] => {
									super::push(out, 36734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => super::t14::n31236(s, out),
						['n', s @ ..] => super::t14::n31237(s, out),
						['o', s @ ..] => super::t14::n31238(s, out),
						['r', s @ ..] => super::t14::n31239(s, out),
						['s', s @ ..] => super::t14::n31240(s, out),
						['t', s @ ..] => super::t14::n31241(s, out),
						['u', 'e', 'n', '\u{1}', s @ ..] => super::t14::n31242(s, out),
						['ß', s @ ..] => super::t14::n31243(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t14::n20507(s, out),
				['g', s @ ..] => super::t14::n20508(s, out),
				['h', s @ ..] => super::t14::n20509(s, out),
				['k', s @ ..] => super::t14::n20510(s, out),
				['l', s @ ..] => super::t14::n20511(s, out),
				['m', s @ ..] => super::t14::n20512(s, out),
				['n', s @ ..] => super::t14::n20513(s, out),
				['r', s @ ..] => super::t14::n20514(s, out),
				['s', s @ ..] => super::t14::n20515(s, out),
				['t', s @ ..] => super::t14::n20516(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t14::n9549(s, out),
		['l', s @ ..] => super::t14::n9550(s, out),
		['q', s @ ..] => super::t14::n9551(s, out),
		['r', s @ ..] => super::t14::n9552(s, out),
		['t', s @ ..] => super::t14::n9553(s, out),
		['ä', s @ ..] => super::t14::n9554(s, out),
		_ => false,
	}
}

/// Words ending -ver.
pub(crate) fn n2655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2651);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9445);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 19958);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 19959);
					match s {
						[] => true,
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 29950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 19960);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9446);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 19961);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 19962);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19963);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 29952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19964);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 19965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9447);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 19966);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 29953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 19967);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 36768);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 36769);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 36770);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 40916);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9448);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19968);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 19969);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 29954);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 29955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 19970);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29956);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 29957);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 29958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 19971);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 29959);
							match s {
								[] => true,
								['i', 's', 's', 'o', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 36771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 19972);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 36772);
									match s {
										[] => true,
										['p', 's', '\u{1}', s @ ..] => {
											super::push(out, 40917);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 36773);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 19973);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 29960);
							match s {
								[] => true,
								['u', 't', '\u{1}', s @ ..] => {
									super::push(out, 36774);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 19974);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 19975);
					match s {
						[] => true,
						['s', 'o', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 29961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9449);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 9450);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 19976);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wer.
pub(crate) fn n2656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2652);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9451);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 19977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9452);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 19978);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 19979);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19980);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 29962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 19981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xer.
pub(crate) fn n2657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2653);
	match s {
		[] => true,
		['o', 'd', s @ ..] => {
			super::push(out, 9453);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 19982);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yer.
pub(crate) fn n2658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2654);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9454);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 19983);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 36775);
									s.is_empty()
								}
								['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 36776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 19984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9455);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9456);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 19985);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19986);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 29964);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zer.
pub(crate) fn n2659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2655);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 9457);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19987);
					match s {
						[] => true,
						['w', 'u', s @ ..] => {
							super::push(out, 29965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 19988);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 29966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9458);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9459);
			match s {
				[] => true,
				['a', 'w', s @ ..] => {
					super::push(out, 19989);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 29967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 29968);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 29969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 9460);
			match s {
				[] => true,
				['i', 'l', s @ ..] => {
					super::push(out, 19990);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 29970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 19991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßer.
pub(crate) fn n2660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2656);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9461);
			s.is_empty()
		}
		['i', 'e', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 19992);
					match s {
						[] => true,
						['h', 'c', 's', 'e', s @ ..] => {
							super::push(out, 29971);
							match s {
								[] => true,
								['n', 'i', 'e', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 36777);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 19993);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 19994);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29972);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 29973);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 29974);
							s.is_empty()
						}
						['t', 'h', 'c', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 29975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'r', s @ ..] => {
			super::push(out, 9462);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 19995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 19996);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 19997);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äer.
pub(crate) fn n2661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2657);
	s.is_empty()
}

/// Words ending -ehr.
pub(crate) fn n2663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2659);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9467);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 9468);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 9469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20011);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 20012);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 29985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 20013);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 29986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 20014);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 29987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 20015);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 29988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t14::n20584(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9470);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 9471);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20017);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 20018);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 36789);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 36790);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 20019);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 20020);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 36791);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 36792);
									match s {
										[] => true,
										['i', 'r', 'e', 't', 'n', 'a', 'f', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 40938);
											s.is_empty()
										}
										['s', 'o', 'k', 'r', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 40939);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 36793);
									s.is_empty()
								}
								['n', 'e', 'n', 'i', 'h', 'c', 's', 'a', 'm', '-', s @ ..] => {
									super::push(out, 36794);
									s.is_empty()
								}
								['s', 'n', s @ ..] => {
									super::push(out, 36795);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'a', 'a', '\u{1}', s @ ..] => {
					super::push(out, 20021);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20022);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 20023);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 29990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 20024);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 20025);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 29991);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 29992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 20026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 9472);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20027);
					match s {
						[] => true,
						['e', 'v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36796);
									s.is_empty()
								}
								['n', 'e', 'g', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 36797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wuhr.
pub(crate) fn n9598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9486);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20041);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ähr.
pub(crate) fn n2666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2662);
	match s {
		[] => true,
		['w', s @ ..] => {
			super::push(out, 9487);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öhr.
pub(crate) fn n2667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2663);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9488);
			s.is_empty()
		}
		['l', 'e', 'd', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 9489);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ühr.
pub(crate) fn n2668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2664);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 9490);
			match s {
				[] => true,
				['e', 'g', 'n', s @ ..] => {
					super::push(out, 20042);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 29999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nhumor.
pub(crate) fn n31350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30020);
	s.is_empty()
}

/// Words ending -rumor.
pub(crate) fn n20644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20076);
	s.is_empty()
}

/// Words ending -tumor.
pub(crate) fn n20645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20077);
	match s {
		[] => true,
		['n', 'e', s @ ..] => {
			super::push(out, 30021);
			match s {
				[] => true,
				['h', 'c', 'o', 'n', 'k', '\u{1}', s @ ..] => {
					super::push(out, 36803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', 'p', s @ ..] => {
			super::push(out, 30022);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nor.
pub(crate) fn n2694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2690);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9539);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9540);
			match s {
				[] => true,
				['t', 'n', s @ ..] => {
					super::push(out, 20078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 9541);
			match s {
				[] => true,
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 20079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9542);
			match s {
				[] => true,
				['d', 'n', s @ ..] => {
					super::push(out, 20080);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oor.
pub(crate) fn n2695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2691);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 9543);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 20081);
					match s {
						[] => true,
						['c', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 30023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9544);
			match s {
				[] => true,
				['f', 'e', 'c', 'n', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 20083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9545);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 20084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 9546);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20085);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -por.
pub(crate) fn n2696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2692);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9547);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9548);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20086);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 20087);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ror.
pub(crate) fn n2697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2693);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9549);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 20088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9550);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9551);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 20089);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sor.
pub(crate) fn n2698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2694);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9552);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20090);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 36804);
									s.is_empty()
								}
								['k', 'n', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 36805);
									s.is_empty()
								}
								['l', 'h', 'a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 36806);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['d', 'o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 40940);
											s.is_empty()
										}
										['f', 'f', 'a', 'w', '\u{1}', s @ ..] => {
											super::push(out, 40941);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9553);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9554);
			match s {
				[] => true,
				['u', 'c', s @ ..] => {
					super::push(out, 20091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 9555);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tor.
pub(crate) fn n2699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2695);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9556);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 9557);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20092);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 20093);
					match s {
						[] => true,
						['e', 'l', 'b', 'm', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 30024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 9558);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20094);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 30025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 20095);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 30026);
							match s {
								[] => true,
								['a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 36807);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 20096);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 30027);
							match s {
								[] => true,
								['s', 'i', '\u{1}', s @ ..] => {
									super::push(out, 36808);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 20097);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 30028);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 36809);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 40942);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 20098);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 30029);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 36810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 20099);
					match s {
						[] => true,
						['i', 'n', 'o', 'i', 't', s @ ..] => {
							super::push(out, 30030);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 36811);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20100);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30031);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 36812);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'r', s @ ..] => {
							super::push(out, 30032);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 36813);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											super::push(out, 40943);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 20101);
					match s {
						[] => true,
						['q', 'ä', 'd', s @ ..] => {
							super::push(out, 30033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9559);
			match s {
				[] => true,
				['ü', s @ ..] => {
					super::push(out, 20102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9560);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 20103);
					match s {
						[] => true,
						['n', 'o', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 30034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 9561);
			match s {
				[] => true,
				['o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 20104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 9562);
			match s {
				[] => true,
				['e', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 20105);
					s.is_empty()
				}
				['r', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 20106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9563);
			match s {
				[] => true,
				['c', 'u', 'b', 'r', 'e', 'd', 'l', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 20107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9564);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 20108);
					match s {
						[] => true,
						['e', '-', 'l', 'm', 't', 'h', '\u{1}', s @ ..] => {
							super::push(out, 30035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 20109);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 30036);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									super::push(out, 36814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 20110);
					match s {
						[] => true,
						['o', 'm', s @ ..] => {
							match s {
								[] => true,
								['h', 'e', 's', 'n', 'r', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 36815);
									s.is_empty()
								}
								['n', 'ü', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 36816);
									s.is_empty()
								}
								['s', 'g', 'n', 'u', 'h', 'c', 'a', 'w', 'r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
									super::push(out, 36817);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 20111);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 20112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 9565);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20113);
					match s {
						[] => true,
						['e', 'r', 'm', s @ ..] => {
							super::push(out, 30037);
							match s {
								[] => true,
								['o', 't', 'a', '\u{1}', s @ ..] => {
									super::push(out, 36818);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 36819);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 36820);
									match s {
										[] => true,
										['g', 'n', 'u', 'r', 'e', 'i', s @ ..] => {
											super::push(out, 40944);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 43248);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'o', 'i', 't', 'a', s @ ..] => {
											super::push(out, 40945);
											match s {
												[] => true,
												['d', 'r', 'a', 't', 'e', 'r', '\u{1}', s @ ..] => {
													super::push(out, 43249);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 36821);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 40946);
											match s {
												[] => true,
												['o', 'd', 'n', 'a', 't', 's', '\u{1}', s @ ..] => {
													super::push(out, 43250);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 20114);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 30038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 20115);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 30039);
							s.is_empty()
						}
						['s', s @ ..] => super::t14::n31373(s, out),
						['v', s @ ..] => {
							super::push(out, 30041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20116);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20117);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30042);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 36827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'd', 'n', s @ ..] => super::t14::n20686(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t14::n9678(s, out),
		['m', s @ ..] => super::t14::n9679(s, out),
		['n', s @ ..] => super::t14::n9680(s, out),
		['o', s @ ..] => super::t14::n9681(s, out),
		['p', s @ ..] => super::t14::n9682(s, out),
		['r', s @ ..] => super::t14::n9683(s, out),
		['s', s @ ..] => super::t14::n9684(s, out),
		['t', s @ ..] => super::t14::n9685(s, out),
		['u', s @ ..] => super::t14::n9686(s, out),
		['y', s @ ..] => super::t14::n9687(s, out),
		['z', s @ ..] => super::t14::n9688(s, out),
		['ß', s @ ..] => super::t14::n9689(s, out),
		_ => false,
	}
}

/// Words ending -uor.
pub(crate) fn n2700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2696);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 9578);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 9579);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vor.
pub(crate) fn n2701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2697);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9580);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lgur.
pub(crate) fn n9726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9613);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 20181);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 20182);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 30091);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -augur.
pub(crate) fn n20758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20183);
	s.is_empty()
}

/// Words ending -hur.
pub(crate) fn n2717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2713);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9615);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 20184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 9616);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 9617);
			match s {
				[] => true,
				['m', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 20185);
					s.is_empty()
				}
				['n', 'e', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 20186);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kur.
pub(crate) fn n2718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2714);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 9618);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					super::push(out, 20187);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lur.
pub(crate) fn n2719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2715);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 9619);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20188);
					s.is_empty()
				}
				['d', 'l', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 20189);
					s.is_empty()
				}
				['f', 'r', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 20190);
					s.is_empty()
				}
				['h', 'c', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 20191);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 20192);
					s.is_empty()
				}
				['n', 'n', 'a', 'w', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 20193);
					s.is_empty()
				}
				['r', 'e', 'd', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 20194);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 20195);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 20196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9620);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 9621);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20197);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mur.
pub(crate) fn n2720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2716);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9622);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9623);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 20198);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 20199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9624);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nur.
pub(crate) fn n2721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2717);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 9625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -our.
pub(crate) fn n2722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2718);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 9626);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 20200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'r', s @ ..] => {
			super::push(out, 9627);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 9628);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20201);
					match s {
						[] => true,
						['b', 'u', 'o', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 30093);
							s.is_empty()
						}
						['p', 'm', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 30094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9629);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 9630);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 20202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 9631);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9632);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 9633);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pur.
pub(crate) fn n2723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2719);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 9634);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 20205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9635);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 20206);
					match s {
						[] => true,
						['p', 'd', s @ ..] => {
							super::push(out, 30095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 9636);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sur.
pub(crate) fn n2724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2720);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9637);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9638);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9639);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 30096);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 36846);
									s.is_empty()
								}
								['t', 's', 'b', s @ ..] => {
									super::push(out, 36847);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20207);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tur.
pub(crate) fn n2725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2721);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9640);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 20208);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 20209);
					match s {
						[] => true,
						['u', 'k', 's', 'u', 'm', 'l', s @ ..] => {
							super::push(out, 30097);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 36848);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 20210);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 30098);
							match s {
								[] => true,
								['r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
									super::push(out, 36849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 30099);
							s.is_empty()
						}
						['r', 'e', 'd', s @ ..] => {
							super::push(out, 30100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 20211);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 36850);
									match s {
										[] => true,
										['m', 'e', 't', '-', s @ ..] => {
											super::push(out, 40955);
											match s {
												[] => true,
												['k', s @ ..] => {
													super::push(out, 43251);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 36851);
									match s {
										[] => true,
										['i', 'l', s @ ..] => {
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 43252);
													s.is_empty()
												}
												['c', s @ ..] => {
													super::push(out, 43253);
													s.is_empty()
												}
												['e', 'c', s @ ..] => {
													super::push(out, 43254);
													s.is_empty()
												}
												['l', 'a', 'i', 'n', s @ ..] => {
													super::push(out, 43255);
													s.is_empty()
												}
												['n', 'e', 's', s @ ..] => {
													super::push(out, 43256);
													s.is_empty()
												}
												['r', s @ ..] => {
													match s {
														[] => true,
														['e', s @ ..] => super::t14::n47290(s, out),
														['o', s @ ..] => super::t14::n47291(s, out),
														_ => false,
													}
												}
												['s', 'g', s @ ..] => {
													super::push(out, 43257);
													s.is_empty()
												}
												['t', 'l', s @ ..] => {
													super::push(out, 43258);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20212);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 30101);
							match s {
								[] => true,
								['i', 'd', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 40956);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 40957);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 30102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9641);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 20213);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 20214);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20215);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 20216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 9642);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					super::push(out, 20217);
					match s {
						[] => true,
						['i', 'h', 'c', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'i', s @ ..] => {
									super::push(out, 36852);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 36853);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 40958);
											s.is_empty()
										}
										['u', 'a', 'h', 'u', s @ ..] => {
											super::push(out, 40959);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 20218);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 30104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 't', 'l', s @ ..] => {
					super::push(out, 20219);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 't', 's', 'a', 'r', 'f', 'n', 'i', 'k', s @ ..] => {
					super::push(out, 20220);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 30106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9643);
			match s {
				[] => true,
				['u', 'k', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 30107);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36854);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 36855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['a', 'm', s @ ..] => {
							super::push(out, 30108);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 30109);
							s.is_empty()
						}
						['d', 'n', 'a', 'b', s @ ..] => {
							super::push(out, 30110);
							s.is_empty()
						}
						['h', 'c', 's', s @ ..] => {
							super::push(out, 30111);
							s.is_empty()
						}
						['k', 'i', 'm', s @ ..] => {
							super::push(out, 30112);
							s.is_empty()
						}
						['l', 'e', 'c', s @ ..] => {
							super::push(out, 30113);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 30114);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 30115);
							match s {
								[] => true,
								['e', 'p', 'r', 'ö', 'k', 'i', s @ ..] => {
									super::push(out, 36856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => super::t14::n31455(s, out),
						['t', 'k', s @ ..] => super::t14::n31456(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t14::n9757(s, out),
		['n', s @ ..] => super::t14::n9758(s, out),
		['p', s @ ..] => super::t14::n9759(s, out),
		['s', s @ ..] => super::t14::n9760(s, out),
		['u', s @ ..] => super::t14::n9761(s, out),
		_ => false,
	}
}

/// Words ending -vur.
pub(crate) fn n2726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2722);
	s.is_empty()
}

/// Words ending -wur.
pub(crate) fn n2727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2723);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 9649);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zur.
pub(crate) fn n2728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2724);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -œur.
pub(crate) fn n2729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2725);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 9651);
			match s {
				[] => true,
				['-', 'e', 'h', 'c', 'o', 'r', 'c', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 20225);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word romanes.
pub(crate) fn n20918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20334);
	s.is_empty()
}

/// Words ending -enes.
pub(crate) fn n9906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9787);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 20335);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 20336);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 20337);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word marines.
pub(crate) fn n9907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9788);
	s.is_empty()
}

/// Words ending -nnes.
pub(crate) fn n9908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9789);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 20338);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnes.
pub(crate) fn n9909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9790);
	s.is_empty()
}

/// Words ending -pes.
pub(crate) fn n2799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2795);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 9791);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9792);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -res.
pub(crate) fn n2800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2796);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9793);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 20339);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 20340);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 20341);
					match s {
						[] => true,
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 30157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9794);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 20342);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 20343);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							super::push(out, 30159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 20344);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 30160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 20345);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							match s {
								[] => true,
								['n', 'r', s @ ..] => {
									super::push(out, 36876);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 36877);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 40964);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 20346);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9795);
			match s {
				[] => true,
				['e', 's', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 20347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9796);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 20348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'ö', 'l', '\u{1}', s @ ..] => {
			super::push(out, 9797);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 9798);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 20349);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ses.
pub(crate) fn n2801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2797);
	match s {
		[] => true,
		['n', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 9799);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9800);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 9801);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 9802);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tes.
pub(crate) fn n2802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2798);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9803);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 20350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9804);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 20351);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 20352);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20353);
					match s {
						[] => true,
						['h', 'c', 'i', 'e', 'z', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 30162);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9805);
			match s {
				[] => true,
				['c', 'i', s @ ..] => {
					super::push(out, 20354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 9806);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 9807);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9808);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9809);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 20357);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 20358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9810);
			match s {
				[] => true,
				['e', 'i', 'z', s @ ..] => {
					super::push(out, 20359);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 20360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 9811);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 9812);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9813);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 9814);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ues.
pub(crate) fn n2803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2799);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9815);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 20363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9816);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 20364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 9817);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ves.
pub(crate) fn n2804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2800);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9818);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zes.
pub(crate) fn n2805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2801);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 9819);
			match s {
				[] => true,
				['o', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 20365);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßes.
pub(crate) fn n2806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2802);
	s.is_empty()
}

/// Words ending -lenzuwachs.
pub(crate) fn n46103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43259);
	s.is_empty()
}

/// Words ending -hnzuwachs.
pub(crate) fn n46104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43260);
	s.is_empty()
}

/// Words ending -ebreis.
pub(crate) fn n31525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30182);
	s.is_empty()
}

/// Words ending -dreis.
pub(crate) fn n20989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20405);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 30183);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ereis.
pub(crate) fn n20990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20406);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 30184);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 30185);
			match s {
				[] => true,
				['c', 's', 't', 'e', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 36880);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -freis.
pub(crate) fn n20991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20407);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 30186);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hreis.
pub(crate) fn n20992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20408);
	match s {
		[] => true,
		['c', 'u', s @ ..] => {
			super::push(out, 30187);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kreis.
pub(crate) fn n20993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20409);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 30188);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 36881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'r', 'o', s @ ..] => {
			super::push(out, 30189);
			s.is_empty()
		}
		['r', 'e', 'i', s @ ..] => {
			super::push(out, 30190);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 36882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 36883);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 40966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 36884);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nreis.
pub(crate) fn n20994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20410);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 30191);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 36885);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['b', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 40967);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 40968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -preis.
pub(crate) fn n20995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20411);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 30192);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					super::push(out, 36886);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 40969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rreis.
pub(crate) fn n20996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20412);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 36887);
					match s {
						[] => true,
						['s', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 40970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 36888);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sreis.
pub(crate) fn n20997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20413);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 30193);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -treis.
pub(crate) fn n20998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20414);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 30194);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -teis.
pub(crate) fn n9969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9849);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 20415);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 20416);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -weis.
pub(crate) fn n9970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9850);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 20417);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 20418);
			match s {
				[] => true,
				['c', 'a', 'n', 'l', s @ ..] => {
					super::push(out, 30195);
					match s {
						[] => true,
						['e', 'z', 'n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 36889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zeis.
pub(crate) fn n9971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9851);
	s.is_empty()
}

/// Words ending -fis.
pub(crate) fn n2817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2813);
	s.is_empty()
}

/// Words ending -gis.
pub(crate) fn n2818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2814);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 9852);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9853);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -his.
pub(crate) fn n2819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2815);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 9854);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 9855);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kis.
pub(crate) fn n2820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2816);
	match s {
		[] => true,
		['c', 'u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 9856);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9857);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lis.
pub(crate) fn n2821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2817);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9858);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20419);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20420);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 20421);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 20422);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 20423);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 20424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 9859);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9860);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 9861);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20426);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 20427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9862);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30196);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 30197);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 36890);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 36891);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 40971);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 40972);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mis.
pub(crate) fn n2822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2818);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9863);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9864);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9865);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9866);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nis.
pub(crate) fn n2823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2819);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9867);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20430);
					s.is_empty()
				}
				['n', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 20431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 9868);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20432);
					match s {
						[] => true,
						['l', 'r', 'e', 'z', s @ ..] => {
							super::push(out, 30198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 20433);
					match s {
						[] => true,
						['e', 'd', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 30199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 20434);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 20435);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 20436);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9869);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 20437);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 36892);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 36893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 20438);
					match s {
						[] => true,
						['ä', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36894);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 36895);
									match s {
										[] => true,
										['e', 'v', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 43261);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 43262);
													s.is_empty()
												}
												['s', 's', s @ ..] => {
													super::push(out, 43263);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 30201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 20439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9870);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 20440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 9871);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 9872);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20441);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20442);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30202);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 36896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 36897);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 40973);
											match s {
												[] => true,
												['g', '-', s @ ..] => {
													super::push(out, 43264);
													match s {
														[] => true,
														['n', s @ ..] => super::t14::n47292(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['p', s @ ..] => {
											super::push(out, 40974);
											match s {
												[] => true,
												['m', s @ ..] => {
													super::push(out, 43265);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 36898);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 20443);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 30203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 20444);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30204);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 30205);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9873);
			match s {
				[] => true,
				['c', 'i', 'e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 30206);
							match s {
								[] => true,
								['g', 'n', s @ ..] => {
									super::push(out, 36900);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 30207);
							match s {
								[] => true,
								['r', 'e', 'v', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 40975);
											s.is_empty()
										}
										['l', 'e', 'h', s @ ..] => {
											super::push(out, 40976);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 40977);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 43266);
													match s {
														[] => true,
														['e', 'm', 'h', s @ ..] => super::t14::n47293(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9874);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 20445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9875);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20446);
					match s {
						[] => true,
						['d', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 30208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 20447);
					match s {
						[] => true,
						['e', 'h', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 36901);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 36902);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 36903);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 20448);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30209);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 30210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 20449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9876);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9877);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 20451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t14::n9998(s, out),
		['t', s @ ..] => super::t14::n9999(s, out),
		['u', 'e', 'r', 'f', 'r', 'e', '\u{1}', s @ ..] => {
			super::push(out, 9880);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ois.
pub(crate) fn n2824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2820);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9881);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 9882);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9883);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 9884);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9885);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 9886);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pis.
pub(crate) fn n2825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2821);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9887);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9888);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9889);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9890);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ris.
pub(crate) fn n2826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2822);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9891);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 20462);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 9892);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20464);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 20465);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 20466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9893);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9894);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 20467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 9895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9896);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9897);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sis.
pub(crate) fn n2827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2823);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9898);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 20469);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30220);
							s.is_empty()
						}
						['l', 'ö', s @ ..] => {
							super::push(out, 30221);
							s.is_empty()
						}
						['n', 'e', 't', 'n', s @ ..] => {
							super::push(out, 30222);
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
							super::push(out, 30223);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 30224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 20470);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 30225);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 36914);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 36915);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 36916);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 30226);
							match s {
								[] => true,
								['n', 'a', 'h', 'p', 'e', 'l', 'e', 's', s @ ..] => {
									super::push(out, 36917);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 30227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 20471);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20472);
					s.is_empty()
				}
				['t', 's', 'a', s @ ..] => {
					super::push(out, 20473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9899);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 20474);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 20475);
					match s {
						[] => true,
						['o', 'p', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 30228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'r', s @ ..] => {
					super::push(out, 20476);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					super::push(out, 20477);
					match s {
						[] => true,
						['g', 'o', s @ ..] => {
							super::push(out, 30229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9900);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 20479);
					s.is_empty()
				}
				['r', 'c', s @ ..] => {
					super::push(out, 20480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9901);
			match s {
				[] => true,
				['e', 'u', s @ ..] => {
					super::push(out, 20481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9902);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 20482);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 30230);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 36918);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 30231);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36919);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 20483);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 30232);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 30233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 20484);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20485);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 30234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 9903);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20486);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 20487);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 30235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9904);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9905);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20489);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 30236);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 30237);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 30238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 20490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'k', s @ ..] => {
			super::push(out, 9906);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 9907);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 20491);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tis.
pub(crate) fn n2828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2824);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9908);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9909);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 20492);
					match s {
						[] => true,
						['i', 'o', 'e', 'r', 'y', 'h', 't', 'n', s @ ..] => {
							super::push(out, 30239);
							match s {
								[] => true,
								['u', 'm', 'm', 'i', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 36920);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 20493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 20494);
					match s {
						[] => true,
						['n', 'i', 'n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 30241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 20495);
					match s {
						[] => true,
						['e', 'y', 'm', s @ ..] => {
							super::push(out, 30242);
							s.is_empty()
						}
						['u', 's', s @ ..] => {
							super::push(out, 30243);
							match s {
								[] => true,
								['p', 'a', 'c', s @ ..] => {
									super::push(out, 36921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 20496);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 30244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', s @ ..] => {
					super::push(out, 20497);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 30245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 20498);
					match s {
						[] => true,
						['t', 's', 'a', 'g', 'x', s @ ..] => {
							super::push(out, 30246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'o', 'y', s @ ..] => {
					super::push(out, 20499);
					match s {
						[] => true,
						['m', 'o', s @ ..] => {
							super::push(out, 30247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20500);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 30248);
							match s {
								[] => true,
								['y', 'z', '\u{1}', s @ ..] => {
									super::push(out, 36922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 20501);
					match s {
						[] => true,
						['i', 't', 'c', 'n', 'u', 'j', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 30249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 9910);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9911);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9912);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9913);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20504);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 20505);
					s.is_empty()
				}
				['y', 'z', '\u{1}', s @ ..] => {
					super::push(out, 20506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 9914);
			match s {
				[] => true,
				['o', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 20507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9915);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 9916);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 20509);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uis.
pub(crate) fn n2829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2825);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 9917);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 9918);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9919);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 9920);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 20510);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 20511);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20512);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vis.
pub(crate) fn n2830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2826);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 9921);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 9922);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20513);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 30250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 20514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9923);
			match s {
				[] => true,
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 20515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9924);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 20516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9925);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 20517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['à', s @ ..] => {
			super::push(out, 9926);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xis.
pub(crate) fn n2831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2827);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9927);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20518);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 20519);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20520);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 36923);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 40984);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 36924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30251);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['m', 'e', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 36925);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 36926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 30252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9928);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 9929);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 20522);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zis.
pub(crate) fn n2832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2828);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9930);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zins.
pub(crate) fn n10103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9982);
	match s {
		[] => true,
		['l', 'a', s @ ..] => {
			super::push(out, 20578);
			match s {
				[] => true,
				['t', 'i', 'p', 'a', 'k', 'd', 'm', 'e', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 30272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 20579);
			s.is_empty()
		}
		['s', 'g', 'n', 'u', 'r', s @ ..] => {
			super::push(out, 20580);
			match s {
				[] => true,
				['e', 'i', 's', 'i', 'l', 'a', 't', 'i', 'p', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 30273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 20581);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 30274);
					s.is_empty()
				}
				['i', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 30275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 20582);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 30276);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rns.
pub(crate) fn n2854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2850);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 9985);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uns.
pub(crate) fn n2855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2851);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 9986);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öns.
pub(crate) fn n2856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2852);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 9987);
			match s {
				[] => true,
				['e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 20583);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ess.
pub(crate) fn n2899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2895);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 10089);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 20666);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 30304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10090);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 10091);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10092);
			match s {
				[] => true,
				['t', 'h', 'c', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 20669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10093);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20670);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 30305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 20671);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20672);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30306);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 30307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20673);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 30308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 20674);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 20675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10094);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 20676);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 30309);
							match s {
								[] => true,
								['u', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 36944);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 30310);
							match s {
								[] => true,
								['r', 'o', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 36945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 20677);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 20678);
					match s {
						[] => true,
						['x', 'e', '-', 't', s @ ..] => {
							super::push(out, 30311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20679);
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 30312);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 36946);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 40985);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 36947);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 36948);
									s.is_empty()
								}
								['s', 't', s @ ..] => {
									super::push(out, 36949);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10095);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 10096);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 20680);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 20681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 10097);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 20682);
					match s {
						[] => true,
						['i', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 30313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20683);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 36950);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 40986);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 36951);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 40987);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 40988);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 40989);
											match s {
												[] => true,
												['e', 'd', 'e', 'i', 'r', 'f', 't', s @ ..] => {
													super::push(out, 43270);
													s.is_empty()
												}
												['o', 'i', 't', 'a', 'r', s @ ..] => {
													super::push(out, 43271);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 20684);
					match s {
						[] => true,
						['b', 'a', 'r', s @ ..] => {
							super::push(out, 30314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iss.
pub(crate) fn n2900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2896);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 10098);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20685);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 30315);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 30316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 20686);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 20687);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10099);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10100);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 20689);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 20690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10101);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30317);
							s.is_empty()
						}
						['n', 'n', s @ ..] => {
							super::push(out, 30318);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 30319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10102);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 10103);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20691);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 20692);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 20693);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 30320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20694);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 20695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10104);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20696);
					s.is_empty()
				}
				['t', 'n', 'n', 'e', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30321);
							s.is_empty()
						}
						['r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 30322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10105);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 20697);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 20698);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 20699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 10106);
			match s {
				[] => true,
				['r', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 20700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oss.
pub(crate) fn n2901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2897);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10107);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 10108);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 10109);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10110);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30323);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 30324);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 30325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10111);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 20702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 20703);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 20704);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 30327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10112);
			match s {
				[] => true,
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 20705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10113);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 20706);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 20707);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30329);
							s.is_empty()
						}
						['e', 'n', 'r', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 30330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 20708);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20709);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 30332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 20710);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 30333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20711);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 30334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10114);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 20712);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 30335);
							s.is_empty()
						}
						['s', 'u', s @ ..] => {
							super::push(out, 30336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uss.
pub(crate) fn n2902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2898);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10115);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10116);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 20714);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 30337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10117);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30338);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 'g', s @ ..] => {
											super::push(out, 40990);
											s.is_empty()
										}
										['r', 'e', 's', s @ ..] => {
											super::push(out, 40991);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'e', 'x', s @ ..] => {
							super::push(out, 30339);
							s.is_empty()
						}
						['r', 'o', 'v', 's', s @ ..] => {
							super::push(out, 30340);
							s.is_empty()
						}
						['s', 'u', 'a', 's', 'b', s @ ..] => {
							super::push(out, 30341);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 36952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10118);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 10119);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 20715);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 30342);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 30343);
							s.is_empty()
						}
						['e', 'd', s @ ..] => {
							super::push(out, 30344);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 30345);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 30346);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 30347);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36953);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 36954);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 30348);
							match s {
								[] => true,
								['e', 'h', s @ ..] => {
									super::push(out, 36955);
									s.is_empty()
								}
								['i', 'e', 'l', s @ ..] => {
									super::push(out, 36956);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 30349);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 30350);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 40992);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 40993);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 36957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 36958);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 36959);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 36960);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 36961);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 36962);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 30351);
							s.is_empty()
						}
						['z', 'r', s @ ..] => {
							super::push(out, 30352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 20716);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 36963);
									s.is_empty()
								}
								['e', 'b', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40994);
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 43272);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 40995);
											match s {
												[] => true,
												['g', 'n', 'u', 'd', s @ ..] => {
													super::push(out, 43273);
													match s {
														[] => true,
														['n', s @ ..] => super::t14::n47296(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', s @ ..] => {
											super::push(out, 40996);
											match s {
												[] => true,
												['e', 'e', s @ ..] => {
													super::push(out, 43274);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 40997);
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 43275);
													match s {
														[] => true,
														['r', 'ö', 'b', '\u{1}', s @ ..] => super::t14::n47297(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 36964);
									match s {
										[] => true,
										['e', 't', s @ ..] => {
											super::push(out, 40998);
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 43276);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 36965);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 36966);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 40999);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10120);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 10121);
			match s {
				[] => true,
				['a', s @ ..] => super::t14::n21309(s, out),
				['e', s @ ..] => super::t14::n21310(s, out),
				['h', s @ ..] => super::t14::n21311(s, out),
				['l', s @ ..] => super::t14::n21312(s, out),
				['n', s @ ..] => super::t14::n21313(s, out),
				['r', s @ ..] => super::t14::n21314(s, out),
				['s', s @ ..] => super::t14::n21315(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t14::n10243(s, out),
		['t', s @ ..] => super::t14::n10244(s, out),
		_ => false,
	}
}

/// Words ending -bus.
pub(crate) fn n2914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2910);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 10142);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 10143);
			match s {
				[] => true,
				['k', 'i', 'n', 'o', 'm', 'r', 'a', 'h', 'h', 'e', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 20758);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 20759);
					match s {
						[] => true,
						['l', 'y', 's', 'y', 'h', 'c', 'a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 30369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 10144);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 10145);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10146);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10147);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 20762);
					match s {
						[] => true,
						['i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 30370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 20763);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20764);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 30371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'f', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 20765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10148);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20766);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30372);
							s.is_empty()
						}
						['n', 'o', 'i', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 30373);
							s.is_empty()
						}
						['r', 'e', 'd', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 30374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 20767);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 30375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10149);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20768);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 30376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36980);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 30377);
							match s {
								[] => true,
								['t', '\u{1}', s @ ..] => {
									super::push(out, 36981);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 30378);
							s.is_empty()
						}
						['j', s @ ..] => {
							super::push(out, 30379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 20769);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 30380);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 30381);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 36982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10150);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10151);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 20771);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30382);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 30383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 20772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10152);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20773);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 20774);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 30384);
							s.is_empty()
						}
						['t', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 30385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10153);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 20775);
					match s {
						[] => true,
						['n', 'u', 't', 'i', 'e', 'l', 'r', 'e', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 30386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10154);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10155);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 20776);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 20777);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dus.
pub(crate) fn n2916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2912);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 10159);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 20781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10160);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 20782);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 30388);
							s.is_empty()
						}
						['g', 'u', 'e', s @ ..] => {
							super::push(out, 30389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 20783);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eus.
pub(crate) fn n2917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2913);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 10161);
			match s {
				[] => true,
				['a', 'i', 'p', 'e', 'l', 'k', 's', 'a', '\u{1}', s @ ..] => {
					super::push(out, 20784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10162);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 20785);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 20786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10163);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 10164);
			match s {
				[] => true,
				['o', 'd', s @ ..] => {
					super::push(out, 20787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10165);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 20788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10166);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20789);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 20790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10167);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 10168);
			match s {
				[] => true,
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 20791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 10169);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fus.
pub(crate) fn n2918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2914);
	match s {
		[] => true,
		['n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 10170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gus.
pub(crate) fn n2919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2915);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10171);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10172);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 20792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10173);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10174);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hus.
pub(crate) fn n2920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2916);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 10175);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 20793);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 20794);
					match s {
						[] => true,
						['o', 'm', 'l', s @ ..] => {
							super::push(out, 30391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10176);
			match s {
				[] => true,
				['y', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 30392);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 30393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10177);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 20795);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 20796);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ius.
pub(crate) fn n2921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2917);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 10178);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 30394);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 36983);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 30395);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 10179);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 30396);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 30397);
							match s {
								[] => true,
								['h', 'r', 'r', 'y', 'p', '\u{1}', s @ ..] => {
									super::push(out, 36984);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'i', 'f', '\u{1}', s @ ..] => {
			super::push(out, 10180);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 10181);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20797);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 30398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10182);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20798);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 30399);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 30400);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 30401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 20799);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 20800);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 20801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10183);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20802);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 20803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10184);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20804);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 10185);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 10186);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jus.
pub(crate) fn n2922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2918);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 10187);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kus.
pub(crate) fn n2923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2919);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10188);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 20806);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 20807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10189);
			match s {
				[] => true,
				['f', 'f', 'i', 'f', 'p', '\u{1}', s @ ..] => {
					super::push(out, 20808);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 20809);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20810);
					s.is_empty()
				}
				['r', 'y', 'g', 'e', 'n', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 20811);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 30402);
							s.is_empty()
						}
						['y', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 30403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20812);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30404);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['g', 'e', 'l', 'h', 'p', '\u{1}', s @ ..] => {
											super::push(out, 41010);
											s.is_empty()
										}
										['h', 'c', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 41011);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 30405);
							s.is_empty()
						}
						['f', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 30406);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 30407);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 30408);
							match s {
								[] => true,
								['o', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41012);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 41013);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10190);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20813);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 20814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10191);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 20815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10192);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 20816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10193);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20817);
					match s {
						[] => true,
						['z', 'i', 't', 's', 'y', 'z', '\u{1}', s @ ..] => {
							super::push(out, 30409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 20818);
					match s {
						[] => true,
						['z', 'n', 'e', 'f', 'f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 30410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10194);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20820);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 20821);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 30411);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36985);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 20822);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 30412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lus.
pub(crate) fn n2924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2920);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10195);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 20823);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 20824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 10196);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10197);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 20825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 10198);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10199);
			match s {
				[] => true,
				['t', 'u', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 20826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10200);
			match s {
				[] => true,
				['y', 'z', s @ ..] => {
					match s {
						[] => true,
						['r', 'u', s @ ..] => {
							super::push(out, 30413);
							match s {
								[] => true,
								['t', 'k', 'n', 'u', 'j', 'n', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 36986);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'u', 'r', s @ ..] => {
									super::push(out, 36987);
									match s {
										[] => true,
										['e', 'i', 'z', 'n', 'a', 'n', 'i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 41014);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 'b', 'e', 'l', 't', s @ ..] => {
									super::push(out, 36988);
									match s {
										[] => true,
										['k', 'u', 'd', 'o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 41015);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10201);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20827);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 30414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 20828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10202);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 10203);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20829);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20830);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 20831);
					match s {
						[] => true,
						['t', 'a', 's', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 30415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10204);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 10205);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 30416);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 30417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ï', s @ ..] => {
			super::push(out, 10206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mus.
pub(crate) fn n2925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2921);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10207);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 10208);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 20832);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 20833);
					match s {
						[] => true,
						['o', 'p', 'o', 'p', 'p', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 30418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'y', s @ ..] => {
			super::push(out, 10209);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 10210);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 20834);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 30419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20835);
					match s {
						[] => true,
						['y', 'h', 'r', 'o', s @ ..] => {
							super::push(out, 30420);
							match s {
								[] => true,
								['i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 36989);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10211);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 20836);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20837);
					match s {
						[] => true,
						['p', 'n', 'e', 'h', 'c', 'n', 'a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 30421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10212);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 10213);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20839);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 30422);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 30423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 20840);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 30424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10214);
			match s {
				[] => true,
				['a', 's', s @ ..] => {
					super::push(out, 20841);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 30425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10215);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10216);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 20843);
					s.is_empty()
				}
				['r', 'd', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 20844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10217);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20845);
					match s {
						[] => true,
						['h', 'c', 'r', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 30426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t14::n10339(s, out),
		['t', s @ ..] => {
			super::push(out, 10219);
			match s {
				[] => true,
				['h', 'c', 'u', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 20851);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nus.
pub(crate) fn n2926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2922);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10220);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 20852);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10221);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 20853);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 30451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 20854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10222);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 20855);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30452);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 30453);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 30454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 20856);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 20857);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 20858);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10223);
			match s {
				[] => true,
				['u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 20859);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 20860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10224);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10225);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20861);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 20862);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 30455);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'u', 't', s @ ..] => {
									match s {
										[] => true,
										['f', 'ü', 'l', '\u{1}', s @ ..] => {
											super::push(out, 41099);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 41100);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 37022);
									s.is_empty()
								}
								['t', 'h', 'c', 'a', 'n', 'h', 'i', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 37023);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'l', 'e', 'w', 'm', 'u', '\u{1}', s @ ..] => {
							super::push(out, 30456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 20863);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 30457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20864);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 30458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10226);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 20865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10227);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10228);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ous.
pub(crate) fn n2927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2923);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 10229);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10230);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 10231);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pus.
pub(crate) fn n2928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2924);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10232);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 10233);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 10234);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20866);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 30459);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37024);
									s.is_empty()
								}
								['e', 'g', 'e', 'l', 'l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 37025);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 37026);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 30460);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 30461);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 30462);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 37027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 20867);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 30463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10235);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 20868);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20869);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 30464);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 30465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10236);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10237);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10238);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 10239);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 20872);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 30466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rus.
pub(crate) fn n2929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2925);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10240);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10241);
			match s {
				[] => true,
				['b', 'r', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 20873);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 20874);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 20875);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 30467);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 30468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10242);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10243);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 20876);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30469);
							s.is_empty()
						}
						['e', 'l', s @ ..] => {
							super::push(out, 30470);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 37028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 30471);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 37029);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 37030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', s @ ..] => {
							super::push(out, 30472);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 30473);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 37031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10244);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 20877);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 20878);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 20879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10245);
			match s {
				[] => true,
				['i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 20880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10246);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10247);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20881);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 30474);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 30475);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 37033);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 41101);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 37034);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 30476);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 37035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20882);
					match s {
						[] => true,
						['i', 'z', 's', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 30477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 10248);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sus.
pub(crate) fn n2930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2926);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10249);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 20883);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 20884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10250);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10251);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10252);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10253);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20886);
					s.is_empty()
				}
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 20887);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 20888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10254);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20889);
					match s {
						[] => true,
						['p', 'o', s @ ..] => {
							super::push(out, 30478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 20890);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 20891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10255);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 10256);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20892);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tus.
pub(crate) fn n2931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2927);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10257);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 20893);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 30479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 20894);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 30480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 20895);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 't', s @ ..] => {
									super::push(out, 37036);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 41102);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 41103);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 41104);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 41105);
											s.is_empty()
										}
										['t', 'l', s @ ..] => {
											super::push(out, 41106);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 37037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 10258);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 20896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10259);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 20897);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 20898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10260);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 20899);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 20900);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 20901);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20902);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 20903);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20904);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 30481);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 30482);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 30483);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 30484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 20905);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 20906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10261);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20907);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 30485);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 30486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 20908);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 20909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10262);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20910);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 20911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10263);
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 20912);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 20913);
					match s {
						[] => true,
						['l', 'a', 'k', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 30487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10264);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10265);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 20915);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 30488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 10266);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 20916);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vus.
pub(crate) fn n2932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2928);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10267);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 10268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xus.
pub(crate) fn n2933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2929);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10269);
			match s {
				[] => true,
				['l', 'p', 'r', s @ ..] => {
					super::push(out, 20917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10270);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äus.
pub(crate) fn n2934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2930);
	s.is_empty()
}

/// Words ending -udenstaat.
pub(crate) fn n46191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43335);
	s.is_empty()
}

/// The word parteienstaat.
pub(crate) fn n46192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43336);
	s.is_empty()
}

/// The word binnenstaat.
pub(crate) fn n46193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43337);
	s.is_empty()
}

/// Words ending -itenstaat.
pub(crate) fn n46194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43338);
	match s {
		[] => true,
		['l', 'l', 'e', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 44344);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ttenstaat.
pub(crate) fn n46195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43339);
	match s {
		[] => true,
		['e', 'n', 'o', 'i', 'r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 44345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cat.
pub(crate) fn n2949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2945);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 10283);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 20928);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 20929);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37048);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 37049);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 37050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'v', 'd', 'a', '\u{1}', s @ ..] => {
			super::push(out, 10284);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10285);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20930);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dat.
pub(crate) fn n2950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2946);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10286);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 20931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 10287);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 20932);
					match s {
						[] => true,
						['l', 'o', 'm', 'm', s @ ..] => {
							super::push(out, 30499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10288);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 10289);
			match s {
				[] => true,
				['o', 's', 'z', s @ ..] => {
					super::push(out, 20933);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 30500);
							match s {
								[] => true,
								['l', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 37051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10290);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20934);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'a', 'h', 'r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
									super::push(out, 37052);
									s.is_empty()
								}
								['s', 'h', s @ ..] => {
									super::push(out, 37053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10291);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20935);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 30501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10292);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 30502);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 30503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10293);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10294);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 10295);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20937);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eat.
pub(crate) fn n2951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2947);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 10296);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 20938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10297);
			match s {
				[] => true,
				['i', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 20939);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 10298);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 10299);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10300);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 20941);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37054);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 37055);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10301);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 20942);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 20943);
					match s {
						[] => true,
						['m', 'u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 30504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10302);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fat.
pub(crate) fn n2952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2948);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 10303);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 20944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10304);
			match s {
				[] => true,
				['u', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30505);
							s.is_empty()
						}
						['i', 'd', s @ ..] => {
							super::push(out, 30506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gat.
pub(crate) fn n2953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2949);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10305);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 20945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10306);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30507);
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 30508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 20946);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10307);
			match s {
				[] => true,
				['d', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 20948);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 20949);
					match s {
						[] => true,
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 30509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 20950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10308);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10309);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hat.
pub(crate) fn n2954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2950);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 10310);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20951);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 20952);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30510);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 30511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', 't', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 20953);
					s.is_empty()
				}
				['n', 'e', 'p', 'p', 'u', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 20954);
					s.is_empty()
				}
				['o', 'e', 'd', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 20955);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20956);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 37056);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 37057);
									match s {
										[] => true,
										['r', 't', 'a', s @ ..] => {
											match s {
												[] => true,
												['m', '\u{1}', s @ ..] => {
													super::push(out, 43340);
													s.is_empty()
												}
												['p', '\u{1}', s @ ..] => {
													super::push(out, 43341);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 37058);
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 37059);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'a', 'v', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 20957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10311);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10312);
			match s {
				[] => true,
				['s', 'o', 'h', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30512);
							s.is_empty()
						}
						['a', 't', 'e', 'm', 'm', s @ ..] => {
							super::push(out, 30513);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 30514);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 37060);
									s.is_empty()
								}
								['r', 't', 'n', s @ ..] => {
									super::push(out, 37061);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 30515);
							match s {
								[] => true,
								['y', 'n', s @ ..] => {
									super::push(out, 37062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 30516);
							match s {
								[] => true,
								['u', 'i', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 41118);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 41119);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 41120);
											match s {
												[] => true,
												['t', 'a', 'n', '\u{1}', s @ ..] => {
													super::push(out, 43342);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 30517);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 30518);
							match s {
								[] => true,
								['d', 'b', 'y', 'l', 'o', 'm', 'm', s @ ..] => {
									super::push(out, 37063);
									s.is_empty()
								}
								['h', 't', 'r', 'o', 'm', s @ ..] => {
									super::push(out, 37064);
									s.is_empty()
								}
								['r', 'o', s @ ..] => {
									super::push(out, 37065);
									match s {
										[] => true,
										['u', 'l', 'f', 'a', 'x', 'e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 41121);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 30519);
							s.is_empty()
						}
						['y', 'l', 'o', 'p', 'm', s @ ..] => {
							super::push(out, 30520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10313);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20959);
					s.is_empty()
				}
				['n', 'a', 'x', '\u{1}', s @ ..] => {
					super::push(out, 20960);
					s.is_empty()
				}
				['u', 'z', '\u{1}', s @ ..] => {
					super::push(out, 20961);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 20962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 10314);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 20963);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iat.
pub(crate) fn n2955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2951);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 10315);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 20964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10316);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 10317);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 10318);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 10319);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10320);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 30521);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 30522);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 30523);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 30524);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 30525);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 37066);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 41122);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 41123);
											match s {
												[] => true,
												['k', 'e', 's', '\u{1}', s @ ..] => {
													super::push(out, 43343);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 37067);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 37068);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 30526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 20966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10321);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 20967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10322);
			match s {
				[] => true,
				['r', 'a', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 20968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 10323);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kat.
pub(crate) fn n2956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2952);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10324);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 10325);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 20969);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 20970);
					match s {
						[] => true,
						['p', 'e', s @ ..] => {
							super::push(out, 30527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 20971);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 20972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10326);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['n', 'y', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37069);
									s.is_empty()
								}
								['n', 'e', 'g', 'o', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 37070);
									s.is_empty()
								}
								['r', 'e', 'h', 'c', 'e', 'r', 'b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 37071);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', 'r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37072);
									s.is_empty()
								}
								['s', 'l', 'e', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 37073);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 20973);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 37074);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 41124);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 41125);
											match s {
												[] => true,
												['e', 'z', 's', 't', 'i', 'e', 'h', 't', 'h', 'c', 'e', '\u{1}', s @ ..] => {
													super::push(out, 43344);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 20974);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37075);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 37076);
									match s {
										[] => true,
										['u', 'i', 'c', s @ ..] => {
											super::push(out, 41126);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 37077);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['m', 'u', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 41127);
											s.is_empty()
										}
										['r', 'o', 'b', '\u{1}', s @ ..] => {
											super::push(out, 41128);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 30529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 20975);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 20976);
					s.is_empty()
				}
				['s', 'y', 'h', 'p', '\u{1}', s @ ..] => {
					super::push(out, 20977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10327);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 10328);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20978);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 20979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10329);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 20980);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 20981);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 37078);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 37079);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10330);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 20982);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lat.
pub(crate) fn n2957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2953);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10331);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 10332);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 20983);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 30530);
							match s {
								[] => true,
								['h', 'p', '\u{1}', s @ ..] => {
									super::push(out, 37080);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 20984);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 20985);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30531);
							s.is_empty()
						}
						['a', 'z', s @ ..] => {
							super::push(out, 30532);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 30533);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 30534);
							match s {
								[] => true,
								['l', 's', 's', 'ü', 'n', '\u{1}', s @ ..] => {
									super::push(out, 37081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'e', 'g', s @ ..] => {
							super::push(out, 30535);
							s.is_empty()
						}
						['n', 'e', 'k', 'r', 'u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 30536);
							s.is_empty()
						}
						['s', 'g', 'n', 'i', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 30537);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'u', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 37082);
									s.is_empty()
								}
								['s', 'b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 37083);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 20986);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 30538);
							match s {
								[] => true,
								['m', 'u', 'i', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 41129);
											s.is_empty()
										}
										['n', 'o', 'm', 'm', 'a', '\u{1}', s @ ..] => {
											super::push(out, 41130);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10333);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 20987);
					match s {
						[] => true,
						['a', 'r', 'm', s @ ..] => {
							super::push(out, 30539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 20988);
					match s {
						[] => true,
						['r', 'e', 'c', '\u{1}', s @ ..] => {
							super::push(out, 30540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 10334);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20989);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 10335);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 20991);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 20992);
					match s {
						[] => true,
						['z', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 30541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 10336);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 20993);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 20994);
					match s {
						[] => true,
						['t', 's', 'e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37084);
									s.is_empty()
								}
								['n', 'i', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 37085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 20995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10337);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10338);
			match s {
				[] => true,
				['k', 'a', s @ ..] => {
					super::push(out, 20996);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 20997);
					match s {
						[] => true,
						['a', 'r', 'g', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', 'f', 'f', 'o', 't', 's', 't', 's', 'n', 'u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 37086);
									s.is_empty()
								}
								['f', 'f', 'o', 't', 's', 't', 's', 'n', 'u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 37087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 10339);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 20998);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37088);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 37089);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 20999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 10340);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 21000);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mat.
pub(crate) fn n2958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2954);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10341);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 21001);
					s.is_empty()
				}
				['r', 'f', 'l', 'o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 21002);
					s.is_empty()
				}
				['t', 'u', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 21003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10342);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 21004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10343);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 21005);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 21006);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21007);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37090);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 37091);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10344);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 21009);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 21010);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							super::push(out, 30542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 21011);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 21012);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 30543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 21013);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30544);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37092);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 37093);
									match s {
										[] => true,
										['o', 'r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 41131);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 37094);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 37095);
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 30545);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 37096);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 30546);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 41132);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 41133);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43345);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 41134);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													super::push(out, 43346);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 41135);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 41136);
											match s {
												[] => true,
												['p', 'a', '\u{1}', s @ ..] => {
													super::push(out, 43347);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 30547);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 37097);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21014);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 30548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10345);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 21015);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37098);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 37099);
									match s {
										[] => true,
										['e', 't', s @ ..] => {
											super::push(out, 41137);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nat.
pub(crate) fn n2959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2955);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10347);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 10348);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 21016);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 21017);
					match s {
						[] => true,
						['n', 'a', 'm', 'r', 'e', 'p', 'm', s @ ..] => {
							super::push(out, 30550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 21018);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 30551);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 30552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 21019);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 30553);
							match s {
								[] => true,
								['n', 'a', 'h', 'c', 's', 'd', 'i', 'a', 'b', 'r', 'e', 's', 'a', '\u{1}', s @ ..] => {
									super::push(out, 37100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 30554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 21020);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 21021);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 30555);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 37101);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 37102);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21022);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 30556);
							match s {
								[] => true,
								['t', 'm', s @ ..] => {
									super::push(out, 37103);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 30557);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 37104);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 21023);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 30558);
							match s {
								[] => true,
								['o', 'i', 'h', 't', 'm', s @ ..] => {
									super::push(out, 37105);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 30559);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10349);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 21024);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 21025);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 21026);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 21027);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 30560);
							match s {
								[] => true,
								['a', 'n', 'i', 'l', 'p', 'i', 'z', 's', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 37106);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 21028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 10350);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21029);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 21030);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 21031);
					match s {
						[] => true,
						['s', 'e', s @ ..] => {
							super::push(out, 30561);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 37107);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10351);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 21032);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 21033);
					s.is_empty()
				}
				['g', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 21034);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 21035);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 21036);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 30562);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 30563);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 37108);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'u', 'f', 'r', s @ ..] => {
							super::push(out, 30564);
							match s {
								[] => true,
								['e', 'b', 'l', 'i', 's', 'k', 'c', 'e', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 37109);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 30565);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['a', 'm', s @ ..] => {
											super::push(out, 41138);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 41139);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													super::push(out, 43348);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 21037);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 30566);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37110);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 37111);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 37112);
									s.is_empty()
								}
								['r', 'e', 's', 's', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 37113);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 37114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 21038);
					match s {
						[] => true,
						['a', 'l', 'p', 'o', 'r', 'o', 'l', 'h', 'c', 'a', 'x', 'e', 'h', 'm', s @ ..] => {
							super::push(out, 30567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10352);
			match s {
				[] => true,
				['u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 21039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10353);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21040);
					match s {
						[] => true,
						['t', 's', 'm', s @ ..] => {
							super::push(out, 30568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 21041);
					match s {
						[] => true,
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 30569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t14::n10477(s, out),
		['r', s @ ..] => {
			super::push(out, 10355);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 21051);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 21052);
					match s {
						[] => true,
						['t', 'n', 'i', 'e', s @ ..] => {
							super::push(out, 30586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 21053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10356);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oat.
pub(crate) fn n2960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2956);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 10357);
			match s {
				[] => true,
				['r', 'e', 'v', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 21054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 10358);
			match s {
				[] => true,
				['n', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 21055);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

