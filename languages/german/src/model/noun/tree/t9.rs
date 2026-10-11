use flexion::Path;

/// Words ending -fen.
pub(crate) fn n2210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2206);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8018);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 17680);
					match s {
						[] => true,
						['s', 'l', 'r', s @ ..] => {
							super::push(out, 27625);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 35280);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 27626);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17681);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17682);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 27628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8019);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 8020);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17683);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27629);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27630);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 27631);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 35282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 17684);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', s @ ..] => {
									super::push(out, 35283);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 40210);
											s.is_empty()
										}
										['t', 'n', s @ ..] => {
											super::push(out, 40211);
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
					super::push(out, 17685);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17686);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 17687);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 27632);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 27633);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 17688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8021);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 27634);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 35285);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 27635);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 35286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 27636);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27637);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 35287);
									match s {
										[] => true,
										['u', 'e', s @ ..] => {
											super::push(out, 40212);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 35288);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 35289);
									match s {
										[] => true,
										['s', 'b', s @ ..] => {
											super::push(out, 40213);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 42922);
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
							super::push(out, 27638);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27639);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 27640);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 35290);
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
			super::push(out, 8022);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8023);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 17689);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17690);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 27641);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 27642);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 35291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 27643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8024);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17691);
					match s {
						[] => true,
						['z', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 35292);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 35293);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 17692);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17693);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 27644);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 35295);
									match s {
										[] => true,
										['z', 'r', 'e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 40214);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 27645);
							match s {
								[] => true,
								['t', 'n', s @ ..] => {
									super::push(out, 35296);
									match s {
										[] => true,
										['e', 'g', 'u', s @ ..] => {
											super::push(out, 40215);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 27646);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 35297);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17694);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 17695);
					match s {
						[] => true,
						['n', 'h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 35298);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 35299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 17696);
					match s {
						[] => true,
						['h', 'c', 's', 'r', s @ ..] => {
							super::push(out, 27647);
							match s {
								[] => true,
								['e', 's', s @ ..] => {
									super::push(out, 35300);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 17697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8025);
			match s {
				[] => true,
				['e', 'w', 'n', s @ ..] => {
					super::push(out, 17698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t13::n8123(s, out),
		['ü', s @ ..] => {
			super::push(out, 8027);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gen.
pub(crate) fn n2211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2207);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8028);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8029);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 17703);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 27654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 17704);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 17705);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27655);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 27656);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 35301);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 27657);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35302);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 17706);
					match s {
						[] => true,
						['n', 'e', 't', 'r', s @ ..] => {
							super::push(out, 27658);
							match s {
								[] => true,
								['a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 35303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 17707);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17708);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 27659);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 27660);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 27661);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27662);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 17709);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27663);
							match s {
								[] => true,
								['a', 'g', s @ ..] => {
									super::push(out, 35305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', s @ ..] => {
							super::push(out, 27664);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 35306);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', 'n', s @ ..] => {
							super::push(out, 27665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17710);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 27666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 17711);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 27667);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 40216);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 42923);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 's', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 40217);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'o', s @ ..] => {
							super::push(out, 27668);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 27669);
							match s {
								[] => true,
								['g', 'n', 'u', 't', 't', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 35307);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 35308);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', s @ ..] => {
							super::push(out, 27670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 17712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8030);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 17713);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17714);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 27671);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 35309);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 35310);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40218);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 27672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17715);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17716);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27673);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35311);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 35312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 27674);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 27675);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 27676);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 27677);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 27678);
							s.is_empty()
						}
						['r', 'e', 'b', s @ ..] => {
							super::push(out, 27679);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 27680);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 35313);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', s @ ..] => {
							super::push(out, 27681);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 27682);
							s.is_empty()
						}
						['z', 'r', 'ä', s @ ..] => {
							super::push(out, 27683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 17717);
					match s {
						[] => true,
						['n', 'i', 't', s @ ..] => {
							super::push(out, 27684);
							s.is_empty()
						}
						['s', 'u', s @ ..] => {
							super::push(out, 27685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 17718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 8031);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17719);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17720);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 27686);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8032);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 17721);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17722);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27688);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27689);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 35314);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 27690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17723);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 27691);
							match s {
								[] => true,
								['e', 'h', 'r', 'e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 35315);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 27692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t13::n18207(s, out),
				['t', s @ ..] => super::t13::n18208(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t13::n8130(s, out),
		['n', s @ ..] => super::t13::n8131(s, out),
		['o', s @ ..] => super::t13::n8132(s, out),
		['r', s @ ..] => super::t13::n8133(s, out),
		['s', s @ ..] => super::t13::n8134(s, out),
		['t', s @ ..] => super::t13::n8135(s, out),
		['u', s @ ..] => super::t13::n8136(s, out),
		['y', s @ ..] => super::t13::n8137(s, out),
		['ä', s @ ..] => super::t13::n8138(s, out),
		['ö', s @ ..] => super::t13::n8139(s, out),
		['ü', s @ ..] => super::t13::n8140(s, out),
		_ => false,
	}
}

/// Words ending -hen.
pub(crate) fn n2212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2208);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8044);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 8045);
			match s {
				[] => true,
				['a', s @ ..] => super::t13::n18236(s, out),
				['b', s @ ..] => {
					super::push(out, 17753);
					match s {
						[] => true,
						['i', 'e', 'w', s @ ..] => {
							super::push(out, 27734);
							match s {
								[] => true,
								['n', 'e', 'k', 'c', s @ ..] => {
									super::push(out, 35346);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 17754);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 27735);
							match s {
								[] => true,
								['h', s @ ..] => super::t13::n37307(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => super::t13::n28925(s, out),
						['ä', s @ ..] => super::t13::n28926(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t13::n18239(s, out),
				['f', s @ ..] => super::t13::n18240(s, out),
				['g', s @ ..] => super::t13::n18241(s, out),
				['h', s @ ..] => super::t13::n18242(s, out),
				['i', s @ ..] => super::t13::n18243(s, out),
				['k', s @ ..] => super::t13::n18244(s, out),
				['l', s @ ..] => super::t13::n18245(s, out),
				['m', s @ ..] => super::t13::n18246(s, out),
				['n', s @ ..] => super::t13::n18247(s, out),
				['o', s @ ..] => super::t13::n18248(s, out),
				['p', s @ ..] => super::t13::n18249(s, out),
				['r', s @ ..] => super::t13::n18250(s, out),
				['s', s @ ..] => super::t13::n18251(s, out),
				['t', s @ ..] => super::t13::n18252(s, out),
				['u', s @ ..] => super::t13::n18253(s, out),
				['z', s @ ..] => super::t13::n18254(s, out),
				['ß', s @ ..] => super::t13::n18255(s, out),
				['ä', s @ ..] => super::t13::n18256(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t13::n8143(s, out),
		['g', s @ ..] => super::t13::n8144(s, out),
		['i', s @ ..] => super::t13::n8145(s, out),
		['o', s @ ..] => super::t13::n8146(s, out),
		['p', s @ ..] => super::t13::n8147(s, out),
		['t', s @ ..] => super::t13::n8148(s, out),
		['u', s @ ..] => super::t13::n8149(s, out),
		['ä', s @ ..] => super::t13::n8150(s, out),
		['ö', s @ ..] => super::t13::n8151(s, out),
		['ü', s @ ..] => super::t13::n8152(s, out),
		_ => false,
	}
}

/// Words ending -ien.
pub(crate) fn n2213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2209);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8056);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 17788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 8057);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 8058);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8059);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17790);
					match s {
						[] => true,
						['k', 'l', s @ ..] => {
							super::push(out, 27807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 17791);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8060);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 27808);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 27809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 8061);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8062);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17794);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27810);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 27811);
							s.is_empty()
						}
						['k', 'i', s @ ..] => {
							super::push(out, 27812);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 27813);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 17795);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27815);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 27816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17796);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8063);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17798);
					s.is_empty()
				}
				['é', 'h', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 17799);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8064);
			match s {
				[] => true,
				['i', 'l', s @ ..] => {
					super::push(out, 17800);
					s.is_empty()
				}
				['o', 'r', s @ ..] => {
					super::push(out, 17801);
					s.is_empty()
				}
				['é', 'l', 'a', 'd', 'g', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 17802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8065);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8066);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17803);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 27817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 17804);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 27818);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 27819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17805);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 27820);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 27821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17806);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 27822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8067);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17807);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8068);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8069);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 8070);
			match s {
				[] => true,
				['n', 'e', 'd', s @ ..] => {
					super::push(out, 17809);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ken.
pub(crate) fn n2214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2210);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8071);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8072);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17810);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 17811);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27823);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17812);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17813);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 27824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 17814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 8073);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17815);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27825);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 27826);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 27827);
							s.is_empty()
						}
						['l', 'p', s @ ..] => {
							super::push(out, 27828);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 27829);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 17816);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27831);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35423);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 40265);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 27832);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 27833);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 35424);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 40266);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 27834);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 27835);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 27836);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27837);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27838);
							match s {
								[] => true,
								['s', 'i', s @ ..] => {
									super::push(out, 35425);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 27839);
							match s {
								[] => true,
								['n', 'e', s @ ..] => {
									super::push(out, 35426);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 27840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 17817);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 27841);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 27842);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 27843);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 35427);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 35428);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 35429);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 40267);
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
					super::push(out, 17818);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27844);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 27845);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 35430);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 27846);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 27847);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27848);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 27849);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27850);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 27851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17819);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 17820);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 27852);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 27853);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 35431);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 35432);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 40268);
											s.is_empty()
										}
										['n', 'a', 'h', s @ ..] => {
											super::push(out, 40269);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'd', s @ ..] => {
									super::push(out, 35433);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 35434);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 35435);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 35436);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 35437);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 35438);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40270);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 40271);
											match s {
												[] => true,
												['e', 'w', 'h', 'c', 's', 'g', s @ ..] => {
													super::push(out, 42938);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 35439);
									match s {
										[] => true,
										['e', 'v', s @ ..] => {
											super::push(out, 40272);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 40273);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'l', s @ ..] => {
									super::push(out, 35440);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 27854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8074);
			match s {
				[] => true,
				['ä', 'p', '\u{1}', s @ ..] => {
					super::push(out, 17821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t13::n8172(s, out),
		['i', s @ ..] => super::t13::n8173(s, out),
		['k', s @ ..] => super::t13::n8174(s, out),
		['l', s @ ..] => super::t13::n8175(s, out),
		['m', s @ ..] => super::t13::n8176(s, out),
		['n', s @ ..] => super::t13::n8177(s, out),
		['o', s @ ..] => super::t13::n8178(s, out),
		['r', s @ ..] => super::t13::n8179(s, out),
		['s', s @ ..] => super::t13::n8180(s, out),
		['u', s @ ..] => super::t13::n8181(s, out),
		['ö', s @ ..] => {
			super::push(out, 8085);
			s.is_empty()
		}
		['ü', s @ ..] => super::t13::n8183(s, out),
		_ => false,
	}
}

/// Words ending -len.
pub(crate) fn n2215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2211);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8087);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17850);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 17851);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 17852);
					match s {
						[] => true,
						['c', 's', 'n', s @ ..] => {
							super::push(out, 27880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 17853);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17854);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 27881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 17855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8088);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17856);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 27882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 17857);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 17858);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 27883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8089);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17859);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 27884);
							s.is_empty()
						}
						['z', 's', s @ ..] => {
							super::push(out, 27885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 17860);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17861);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 27886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 17862);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 17863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8090);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17864);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 17865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8091);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17866);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27887);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 35451);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 27888);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35452);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 27889);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 27890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 17867);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27891);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 27892);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35453);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 27893);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 27894);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 35454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 17868);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 27895);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27896);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 27897);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 35455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 17869);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27898);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 27899);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27900);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27901);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 35456);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 35457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 27902);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 27903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 17870);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 17871);
					match s {
						[] => true,
						['f', 'l', 'e', s @ ..] => {
							super::push(out, 27904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8092);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8093);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 17872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 8094);
			match s {
				[] => true,
				['h', 't', s @ ..] => {
					match s {
						[] => true,
						['e', 'y', s @ ..] => {
							super::push(out, 27905);
							s.is_empty()
						}
						['ä', 'y', s @ ..] => {
							super::push(out, 27906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 17873);
					s.is_empty()
				}
				['p', 'o', 'r', 'p', 'y', s @ ..] => {
					super::push(out, 17874);
					s.is_empty()
				}
				['t', 'e', s @ ..] => {
					super::push(out, 17875);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -men.
pub(crate) fn n2216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2212);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8095);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17876);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 17877);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 17878);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17879);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 17880);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 27907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 17881);
					match s {
						[] => true,
						['a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 27908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 17882);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 27909);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35458);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['s', 'u', 'l', 'h', 'c', 's', 'b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 40276);
											s.is_empty()
										}
										['t', 'a', 'a', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 40277);
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
					super::push(out, 17883);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8096);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8097);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 17884);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 17885);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17886);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 27911);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 35459);
									match s {
										[] => true,
										['e', 'h', 'c', 's', 't', 'i', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 40278);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 27912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8098);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17887);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 27913);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 35460);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 27914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 17888);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 35461);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 35462);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 35463);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 35464);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 35465);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 35466);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35467);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 42939);
													s.is_empty()
												}
												['d', s @ ..] => {
													super::push(out, 42940);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 42941);
													s.is_empty()
												}
												['v', s @ ..] => {
													super::push(out, 42942);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 40279);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 35468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 17889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8099);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8100);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17891);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17892);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8101);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17894);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17895);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17896);
					match s {
						[] => true,
						['k', s @ ..] => super::t13::n29110(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17897);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 17898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8102);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 17899);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 27915);
							s.is_empty()
						}
						['l', 'e', 'z', 'r', 'u', 'w', '\u{1}', s @ ..] => super::t13::n29112(s, out),
						['o', s @ ..] => super::t13::n29113(s, out),
						['s', s @ ..] => super::t13::n29114(s, out),
						['ä', s @ ..] => super::t13::n29115(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t13::n18389(s, out),
				['r', s @ ..] => super::t13::n18390(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t13::n8200(s, out),
		['s', s @ ..] => super::t13::n8201(s, out),
		['t', s @ ..] => super::t13::n8202(s, out),
		['u', s @ ..] => super::t14::n8203(s, out),
		['y', s @ ..] => super::t14::n8204(s, out),
		['ä', s @ ..] => super::t14::n8205(s, out),
		['ö', s @ ..] => super::t14::n8206(s, out),
		_ => false,
	}
}

/// Words ending -nen.
pub(crate) fn n2217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2213);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8110);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 17912);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 17913);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 17914);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17915);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 27921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8111);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8112);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 8113);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 8114);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8115);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27922);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 27923);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 27924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 8116);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8117);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17916);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17917);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 27925);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 35488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 27926);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 27927);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 27928);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27929);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 35489);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40290);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 40291);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 40292);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 35490);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 35491);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 35492);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 35493);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 35494);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40293);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 40294);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', s @ ..] => {
									super::push(out, 35495);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 40295);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 40296);
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
					super::push(out, 17918);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 27930);
							s.is_empty()
						}
						['s', 'n', s @ ..] => {
							super::push(out, 27931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17919);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 27932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 17920);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 17921);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 27933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8118);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 17922);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17923);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 17924);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8119);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8120);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 8121);
			match s {
				[] => true,
				['r', 't', 's', s @ ..] => {
					super::push(out, 17926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 8122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pen.
pub(crate) fn n2218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2214);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8123);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 27934);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 35496);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 27935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8124);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 17927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8125);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17928);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							super::push(out, 27936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17929);
					match s {
						[] => true,
						['l', 'k', 'r', 'e', 'v', s @ ..] => {
							super::push(out, 27937);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 27938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8126);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 17930);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 27939);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8127);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17931);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27941);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 27942);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 35497);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 35498);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 35499);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 27943);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 27944);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35500);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 27945);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27946);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 27947);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 27948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 17932);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17933);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17934);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 35501);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 35502);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 27949);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 27950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 17935);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 27951);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 27952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8128);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17936);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17937);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 27953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8129);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 17938);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ren.
pub(crate) fn n2219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2215);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8130);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8131);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17939);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 17940);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 17941);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 17942);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 17943);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 27954);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 27955);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8132);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 17944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8133);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17945);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 17946);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 27956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 17947);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 27957);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35503);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 40297);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 35504);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 40298);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 42948);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 35505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 27958);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 27959);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 35506);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 17948);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 27960);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 27961);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 35507);
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
			super::push(out, 8134);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17949);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 35508);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 35509);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 35510);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40299);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 35511);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 35512);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 35513);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 35514);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 35515);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 35516);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 35517);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35518);
									match s {
										[] => true,
										['e', s @ ..] => super::t14::n42815(s, out),
										['o', s @ ..] => {
											super::push(out, 40300);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 35519);
									s.is_empty()
								}
								['t', 'i', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 40301);
											s.is_empty()
										}
										['m', s @ ..] => super::t14::n42818(s, out),
										_ => false,
									}
								}
								['z', s @ ..] => super::t14::n37489(s, out),
								_ => false,
							}
						}
						['w', s @ ..] => super::t14::n29161(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t14::n18441(s, out),
				['o', s @ ..] => super::t14::n18442(s, out),
				['ä', s @ ..] => super::t14::n18443(s, out),
				['ü', s @ ..] => super::t14::n18444(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t14::n8232(s, out),
		['k', s @ ..] => super::t14::n8233(s, out),
		['o', s @ ..] => super::t14::n8234(s, out),
		['r', s @ ..] => super::t14::n8235(s, out),
		['u', s @ ..] => super::t14::n8236(s, out),
		['ä', s @ ..] => super::t14::n8237(s, out),
		['ö', s @ ..] => super::t14::n8238(s, out),
		['ü', s @ ..] => super::t14::n8239(s, out),
		_ => false,
	}
}

/// Words ending -sen.
pub(crate) fn n2220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2216);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8143);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8144);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 17970);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 17971);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 17972);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17973);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 27979);
							s.is_empty()
						}
						['z', 'r', s @ ..] => {
							super::push(out, 27980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 17974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 8145);
			match s {
				[] => true,
				['ü', s @ ..] => {
					super::push(out, 17975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8146);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17976);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17977);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 17978);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 17979);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 27981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17980);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 27982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 17981);
					match s {
						[] => true,
						['d', 'n', s @ ..] => {
							super::push(out, 27983);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 27984);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 35526);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 35527);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 35528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 's', s @ ..] => {
							super::push(out, 27985);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 27986);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', 'i', s @ ..] => {
									super::push(out, 35529);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 35530);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 40304);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 40305);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 35531);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 35532);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 27987);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35533);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 40306);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 40307);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 40308);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 40309);
											s.is_empty()
										}
										['x', s @ ..] => {
											super::push(out, 40310);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'a', 'm', s @ ..] => {
									super::push(out, 35534);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', 't', s @ ..] => {
							super::push(out, 27988);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 27989);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35535);
									s.is_empty()
								}
								['g', 'n', 'u', 't', 't', s @ ..] => {
									super::push(out, 35536);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40311);
											s.is_empty()
										}
										['h', 'c', 'i', 'r', 'e', 'b', s @ ..] => {
											super::push(out, 40312);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'a', 'h', 'n', s @ ..] => {
									super::push(out, 35537);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', 'o', s @ ..] => {
							super::push(out, 27990);
							s.is_empty()
						}
						['z', 'i', s @ ..] => {
							super::push(out, 27991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8147);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 27992);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 27993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8148);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17982);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 27994);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 27995);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 27996);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 27997);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 27998);
							s.is_empty()
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 35538);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 35539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 27999);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 28000);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 28001);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 28002);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 35540);
									match s {
										[] => true,
										['y', s @ ..] => {
											super::push(out, 40313);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 35541);
									s.is_empty()
								}
								['ü', s @ ..] => super::t14::n37513(s, out),
								_ => false,
							}
						}
						['s', s @ ..] => super::t14::n29205(s, out),
						['t', s @ ..] => super::t14::n29206(s, out),
						['w', s @ ..] => super::t14::n29207(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t14::n18475(s, out),
				_ => false,
			}
		}
		['k', s @ ..] => super::t14::n8246(s, out),
		['l', s @ ..] => super::t14::n8247(s, out),
		['m', s @ ..] => super::t14::n8248(s, out),
		['n', s @ ..] => super::t14::n8249(s, out),
		['o', s @ ..] => super::t14::n8250(s, out),
		['p', s @ ..] => super::t14::n8251(s, out),
		['r', s @ ..] => super::t14::n8252(s, out),
		['s', s @ ..] => super::t14::n8253(s, out),
		['u', s @ ..] => super::t14::n8254(s, out),
		['ö', s @ ..] => super::t14::n8255(s, out),
		_ => false,
	}
}

/// Words ending -ten.
pub(crate) fn n2221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2217);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8159);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18002);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 18003);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 28027);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 35555);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 28028);
							match s {
								[] => true,
								['g', 'n', 'u', s @ ..] => {
									super::push(out, 35556);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 40316);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 35557);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 18004);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 18005);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18006);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 28029);
							match s {
								[] => true,
								['h', 'c', s @ ..] => {
									super::push(out, 35558);
									s.is_empty()
								}
								['n', 'a', s @ ..] => {
									super::push(out, 35559);
									s.is_empty()
								}
								['r', 'e', 'v', s @ ..] => {
									super::push(out, 35560);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 35561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 28030);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 28031);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35562);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 28032);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 28033);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 28034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 18007);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 18008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 8160);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 18009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8161);
			match s {
				[] => true,
				['i', 'b', 'r', s @ ..] => {
					super::push(out, 18010);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 28035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18011);
					s.is_empty()
				}
				['r', 't', 'n', s @ ..] => {
					super::push(out, 18012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 8162);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 18013);
					match s {
						[] => true,
						['c', 's', 'n', s @ ..] => {
							super::push(out, 28036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8163);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28037);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 35564);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 35565);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 40317);
											match s {
												[] => true,
												['i', 'e', 'w', '\u{1}', s @ ..] => {
													super::push(out, 42952);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 40318);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 35566);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35567);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 35568);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 28038);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 28039);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 35569);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35570);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 40319);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 28040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8164);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18014);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'r', 'a', 'u', s @ ..] => {
							super::push(out, 28041);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35571);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 28042);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 28043);
							match s {
								[] => true,
								['b', 's', s @ ..] => {
									super::push(out, 35572);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 40320);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 28044);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 35573);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 28045);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 35574);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 35575);
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
			super::push(out, 8165);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18015);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 28046);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 28047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8166);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18016);
					match s {
						[] => true,
						['h', 'r', s @ ..] => {
							super::push(out, 28048);
							match s {
								[] => true,
								['e', 'v', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 40321);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['g', 'n', 'u', 'm', s @ ..] => super::t14::n45769(s, out),
												['k', s @ ..] => super::t14::n45770(s, out),
												['n', s @ ..] => super::t14::n45771(s, out),
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
				['e', s @ ..] => super::t14::n18511(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t14::n8264(s, out),
		['o', s @ ..] => super::t14::n8265(s, out),
		['p', s @ ..] => super::t14::n8266(s, out),
		['r', s @ ..] => super::t14::n8267(s, out),
		['s', s @ ..] => super::t14::n8268(s, out),
		['t', s @ ..] => super::t14::n8269(s, out),
		['u', s @ ..] => super::t14::n8270(s, out),
		['ä', s @ ..] => super::t14::n8271(s, out),
		['ö', s @ ..] => super::t14::n8272(s, out),
		['ü', s @ ..] => super::t14::n8273(s, out),
		_ => false,
	}
}

/// Words ending -uen.
pub(crate) fn n2222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2218);
	s.is_empty()
}

/// Words ending -ven.
pub(crate) fn n2223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2219);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8177);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8178);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8179);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 18056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8180);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wen.
pub(crate) fn n2224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2220);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 8181);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 8182);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8183);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xen.
pub(crate) fn n2225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2221);
	match s {
		[] => true,
		['a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 8184);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8185);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8186);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 18057);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18058);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yen.
pub(crate) fn n2226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2222);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8187);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8188);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zen.
pub(crate) fn n2227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2223);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8189);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8190);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 18059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8191);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 18060);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8192);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8193);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18062);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 28103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 18063);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 28104);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 28105);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18064);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28106);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 35594);
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
			super::push(out, 8194);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18065);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 18066);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18067);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 28107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18068);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 28108);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 28109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8195);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18069);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28110);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 28111);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 18070);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 28113);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 35595);
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
			super::push(out, 8196);
			match s {
				[] => true,
				['e', 'm', 'h', 'c', 's', 's', s @ ..] => {
					super::push(out, 18071);
					s.is_empty()
				}
				['u', 'w', s @ ..] => {
					super::push(out, 18072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28114);
							s.is_empty()
						}
						['r', 'e', 'd', 'i', 'w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 28115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8197);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18073);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 28116);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 28117);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 28118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 18074);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 28119);
							match s {
								[] => true,
								['r', 'e', 'z', s @ ..] => {
									super::push(out, 35596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 28120);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 28121);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 28122);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 35597);
									s.is_empty()
								}
								['r', 'e', 'v', s @ ..] => {
									super::push(out, 35598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 18075);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 18076);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 18077);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 28123);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 28124);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 35599);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 35600);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 40335);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 28125);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 18078);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 18079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8198);
			s.is_empty()
		}
		['z', 'a', 'm', s @ ..] => {
			super::push(out, 8199);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 8200);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 18080);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßen.
pub(crate) fn n2228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2224);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8201);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 28127);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 'g', s @ ..] => {
											super::push(out, 40336);
											s.is_empty()
										}
										['r', 'e', 't', s @ ..] => {
											super::push(out, 40337);
											s.is_empty()
										}
										['t', 'h', s @ ..] => {
											super::push(out, 40338);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 28128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8202);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8203);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8204);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18081);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 18082);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äen.
pub(crate) fn n2229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2225);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 8205);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -éen.
pub(crate) fn n2230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2226);
	match s {
		[] => true,
		['l', 'u', 'e', 'h', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 8207);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word souterrain.
pub(crate) fn n45784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42967);
	s.is_empty()
}

/// Words ending -rubin.
pub(crate) fn n18658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18159);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 28170);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 28171);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ybin.
pub(crate) fn n8343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8246);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 18161);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 18162);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cin.
pub(crate) fn n2241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2237);
	match s {
		[] => true,
		['i', 'd', s @ ..] => {
			super::push(out, 8247);
			match s {
				[] => true,
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 18163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8248);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8249);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 8250);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -din.
pub(crate) fn n2242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2238);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8251);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 18164);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 28172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18165);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28173);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 28174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8252);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 18167);
					match s {
						[] => true,
						['a', 'h', 'c', 's', 'd', 'u', 'm', 's', 'k', 'l', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 28175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8253);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18168);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'v', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 28176);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 28177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'd', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 18169);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18170);
					match s {
						[] => true,
						['y', 'p', 'i', s @ ..] => {
							super::push(out, 28178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18171);
					s.is_empty()
				}
				['u', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 18172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8254);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8255);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18173);
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 28179);
							match s {
								[] => true,
								['l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 35625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 28180);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 35626);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 18174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8256);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18175);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 18176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18177);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 18178);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 28181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8257);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 18179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 8258);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ein.
pub(crate) fn n2243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2239);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 8259);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 18180);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 18181);
					match s {
						[] => true,
						['c', 's', 'i', 'f', s @ ..] => {
							super::push(out, 28182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 'f', 'l', 'e', 's', s @ ..] => {
							super::push(out, 28183);
							match s {
								[] => true,
								['s', 'o', 'r', 'l', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 35627);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 28184);
							match s {
								[] => true,
								['a', 'z', s @ ..] => {
									super::push(out, 35628);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 18182);
					match s {
						[] => true,
						['i', 'e', 'h', s @ ..] => {
							super::push(out, 28185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8260);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 18183);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18184);
					match s {
						[] => true,
						['a', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35629);
									s.is_empty()
								}
								['z', 'n', 'ü', 'm', '\u{1}', s @ ..] => {
									super::push(out, 35630);
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
			super::push(out, 8261);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 8262);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 18185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8263);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 18186);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28186);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 35631);
									match s {
										[] => true,
										['l', 'l', 'e', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 40345);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 28187);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 35632);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 35633);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 40346);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 40347);
											match s {
												[] => true,
												['e', 'b', 'a', '\u{1}', s @ ..] => {
													super::push(out, 42968);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 35634);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 40348);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 40349);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 35635);
									s.is_empty()
								}
								['l', 'e', s @ ..] => {
									super::push(out, 35636);
									match s {
										[] => true,
										['g', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 40350);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 40351);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 35637);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40352);
											s.is_empty()
										}
										['e', s @ ..] => super::t14::n42873(s, out),
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 35638);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35639);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 40353);
											match s {
												[] => true,
												['m', s @ ..] => super::t14::n45793(s, out),
												['r', s @ ..] => super::t14::n45794(s, out),
												['u', s @ ..] => super::t14::n45795(s, out),
												_ => false,
											}
										}
										['o', s @ ..] => super::t14::n42875(s, out),
										_ => false,
									}
								}
								['s', s @ ..] => super::t14::n37616(s, out),
								['t', s @ ..] => super::t14::n37617(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t14::n18688(s, out),
				['t', s @ ..] => super::t14::n18689(s, out),
				_ => false,
			}
		}
		['j', s @ ..] => super::t14::n8362(s, out),
		['l', s @ ..] => super::t14::n8363(s, out),
		['m', s @ ..] => super::t14::n8364(s, out),
		['n', s @ ..] => super::t14::n8365(s, out),
		['p', s @ ..] => super::t14::n8366(s, out),
		['r', s @ ..] => super::t14::n8367(s, out),
		['s', s @ ..] => super::t14::n8368(s, out),
		['t', s @ ..] => super::t14::n8369(s, out),
		['w', s @ ..] => super::t14::n8370(s, out),
		_ => false,
	}
}

/// Words ending -fin.
pub(crate) fn n2244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2240);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8273);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8274);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 18232);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18233);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 28229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 8275);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18234);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 28230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 18235);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 18236);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 18237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8276);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18238);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28231);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 28232);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 28233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 18239);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gin.
pub(crate) fn n2245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2241);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8277);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8278);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 18240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8279);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 18241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8280);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 18242);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 18243);
					match s {
						[] => true,
						['o', 't', 'a', s @ ..] => {
							super::push(out, 28235);
							match s {
								[] => true,
								['n', 'a', s @ ..] => {
									super::push(out, 35683);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 18244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8281);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 18245);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 28237);
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

/// Words ending -hin.
pub(crate) fn n2246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2242);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8282);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8283);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 18246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 8284);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18247);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 28239);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 35684);
									match s {
										[] => true,
										['a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 40393);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 28240);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18248);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28241);
							match s {
								[] => true,
								['l', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 35686);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 28242);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 35687);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 28243);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35688);
									match s {
										[] => true,
										['k', '\u{1}', s @ ..] => {
											super::push(out, 40394);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 28244);
							match s {
								[] => true,
								['j', s @ ..] => {
									super::push(out, 35689);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 18249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8285);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18250);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18251);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18252);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18253);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 28246);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 35690);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 35691);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18254);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 28247);
							match s {
								[] => true,
								['h', 'p', 'l', s @ ..] => {
									super::push(out, 35692);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 18255);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8286);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 18256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8287);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18257);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 18258);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18259);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 18260);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 35693);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 35694);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 'l', 'o', 'k', 'a', '\u{1}', s @ ..] => {
					super::push(out, 18261);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 18262);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iin.
pub(crate) fn n2247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2243);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 8288);
			match s {
				[] => true,
				['l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 18263);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jin.
pub(crate) fn n2248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2244);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8289);
			s.is_empty()
		}
		['i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 8290);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kin.
pub(crate) fn n2249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2245);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8291);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 18264);
					match s {
						[] => true,
						['o', 't', '\u{1}', s @ ..] => {
							super::push(out, 28249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8292);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 18265);
					match s {
						[] => true,
						['r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 28250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8293);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 8294);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8295);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18267);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 28251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18268);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 28252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 18269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8296);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 18270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8297);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8298);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18271);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 18272);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 18273);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lin.
pub(crate) fn n2250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2246);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8299);
			match s {
				[] => true,
				['d', 'g', 'y', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 18274);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 18275);
					s.is_empty()
				}
				['k', 's', s @ ..] => {
					super::push(out, 18276);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 18277);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 35695);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 35696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18278);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18279);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28253);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 28254);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 35697);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 18280);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 18281);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 18282);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28255);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 28256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 't', 'p', '\u{1}', s @ ..] => {
					super::push(out, 18283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8300);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 18284);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28257);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 28258);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 35698);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 35699);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 18285);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18286);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 18287);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 28259);
							match s {
								[] => true,
								['i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 35700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 28260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 18288);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 28261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 18289);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 18290);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28262);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 28263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18291);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 18292);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 28264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 18293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8301);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8302);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 18294);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 18295);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 18296);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 18297);
					match s {
						[] => true,
						['q', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 28265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 8303);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 8304);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18298);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 18299);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 28266);
							match s {
								[] => true,
								['i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 35701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 18300);
					match s {
						[] => true,
						['c', 'i', 'n', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 28267);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 28268);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 28269);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 28270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18301);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 18302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8305);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18303);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 18304);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 18305);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 18306);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 35702);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 35703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 28271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 18307);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28272);
							match s {
								[] => true,
								['a', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 35704);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18308);
					match s {
						[] => true,
						['i', 'h', 'c', 'y', s @ ..] => {
							super::push(out, 28273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 18309);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28274);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 28275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18310);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 28276);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 28277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8306);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 18311);
					match s {
						[] => true,
						['z', 's', 'i', 'd', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 35705);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40395);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 40396);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 40397);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 40398);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 43002);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 43003);
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
		['r', s @ ..] => super::t14::n8405(s, out),
		['u', s @ ..] => super::t14::n8406(s, out),
		['y', s @ ..] => super::t14::n8407(s, out),
		['ä', s @ ..] => {
			super::push(out, 8310);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -min.
pub(crate) fn n2251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2247);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8311);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18320);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 18321);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 18322);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28283);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 35708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 18323);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 28284);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 28285);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 28286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 18324);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 28287);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 28288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 18325);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 28289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18326);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18327);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							super::push(out, 28290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18328);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18329);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 35710);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 35711);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 35712);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 28291);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 28292);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 28293);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 28294);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 28295);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35713);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 35714);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 28296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 18330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8312);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 18331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8313);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 18332);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18333);
					match s {
						[] => true,
						['e', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 28297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8314);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8315);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 8316);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 8317);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8318);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 18334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8319);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18335);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 28298);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 28299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 18336);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 28300);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 35715);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 28301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8320);
			match s {
				[] => true,
				['a', 'j', s @ ..] => {
					match s {
						[] => true,
						['n', 'r', 'e', 'u', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 28302);
							s.is_empty()
						}
						['r', 'e', 'm', 'm', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 28303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8321);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 18338);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 8322);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 18340);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 18341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 8323);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nin.
pub(crate) fn n2252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2248);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8324);
			match s {
				[] => true,
				['g', 'n', 'a', s @ ..] => {
					super::push(out, 18342);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 18343);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28304);
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 28305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 18344);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35716);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 35717);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 35718);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35719);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 35720);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 18345);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18346);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 18347);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 28306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 18348);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 28307);
							match s {
								[] => true,
								['o', 'm', s @ ..] => {
									super::push(out, 35721);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 18349);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 28308);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8325);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18350);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18351);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 8326);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 18352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8327);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18353);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 18354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8328);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 18355);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 28311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 18356);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 28312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8329);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18358);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 28313);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 18359);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 28315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 18360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8330);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18361);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 28316);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 28317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 18362);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 18363);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18364);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28318);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 28319);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28320);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 28321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18365);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28322);
							s.is_empty()
						}
						['e', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 28323);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', 'l', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 35722);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 35723);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 28324);
							s.is_empty()
						}
						['u', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 28325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 8331);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oin.
pub(crate) fn n2253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2249);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 8332);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 18366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8333);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8334);
			s.is_empty()
		}
		['s', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 8335);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 8336);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pin.
pub(crate) fn n2254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2250);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8337);
			s.is_empty()
		}
		['a', 'r', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 8338);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8339);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 18367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8340);
			match s {
				[] => true,
				['c', 'y', 'l', '\u{1}', s @ ..] => {
					super::push(out, 18368);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 28326);
							s.is_empty()
						}
						['y', 'l', '\u{1}', s @ ..] => {
							super::push(out, 28327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 28328);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 28329);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8341);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8342);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8343);
			s.is_empty()
		}
		['u', 'k', 'l', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 8344);
			s.is_empty()
		}
		['y', 't', '\u{1}', s @ ..] => {
			super::push(out, 8345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rin.
pub(crate) fn n2255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2251);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8346);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 18369);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 18370);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35724);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 35725);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 35726);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 18371);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 28331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 18372);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 18373);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 18374);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28332);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 28333);
							match s {
								[] => true,
								['a', 'y', 'm', '\u{1}', s @ ..] => {
									super::push(out, 35727);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 28334);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 35728);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 28335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28336);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 28337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18376);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 18377);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 8347);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 18378);
					s.is_empty()
				}
				['e', 'f', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 18379);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8348);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18381);
					match s {
						[] => true,
						['h', 'n', s @ ..] => {
							super::push(out, 28339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 18382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8349);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 18383);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 28340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 18384);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28341);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 35729);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 40399);
											match s {
												[] => true,
												['m', 'ä', 'h', '\u{1}', s @ ..] => {
													super::push(out, 43004);
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
				['g', s @ ..] => {
					super::push(out, 18385);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 18386);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18387);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 28342);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									super::push(out, 35730);
									match s {
										[] => true,
										['r', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 40400);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 28343);
							match s {
								[] => true,
								['a', 't', 'e', 'g', s @ ..] => {
									super::push(out, 35731);
									match s {
										[] => true,
										['e', 'v', '-', s @ ..] => {
											super::push(out, 40401);
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
					super::push(out, 18388);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18389);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 18390);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 28344);
							match s {
								[] => true,
								['o', 'w', 'e', 'b', 'n', s @ ..] => {
									super::push(out, 35732);
									match s {
										[] => true,
										['e', 'k', s @ ..] => {
											super::push(out, 40402);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 28345);
							match s {
								[] => true,
								['i', 'w', 'h', s @ ..] => {
									super::push(out, 35733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18391);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35734);
									match s {
										[] => true,
										['f', 'h', s @ ..] => {
											super::push(out, 40403);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 35735);
									match s {
										[] => true,
										['l', s @ ..] => {
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 43005);
													match s {
														[] => true,
														['l', s @ ..] => super::t14::n47186(s, out),
														_ => false,
													}
												}
												['l', 'l', s @ ..] => {
													super::push(out, 43006);
													match s {
														[] => true,
														['a', s @ ..] => super::t14::n47187(s, out),
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
				['s', s @ ..] => {
					super::push(out, 18392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18393);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 28347);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35736);
									match s {
										[] => true,
										['b', 'o', 't', 'o', 'z', 'a', '\u{1}', s @ ..] => {
											super::push(out, 40404);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 28348);
							match s {
								[] => true,
								['o', s @ ..] => super::t14::n37717(s, out),
								_ => false,
							}
						}
						['s', 'e', s @ ..] => super::t14::n29576(s, out),
						_ => false,
					}
				}
				['v', s @ ..] => super::t14::n18900(s, out),
				['z', s @ ..] => super::t14::n18901(s, out),
				['ß', s @ ..] => super::t14::n18902(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t14::n8448(s, out),
		['h', s @ ..] => super::t14::n8449(s, out),
		['i', s @ ..] => super::t14::n8450(s, out),
		['o', s @ ..] => super::t14::n8451(s, out),
		['r', s @ ..] => super::t14::n8452(s, out),
		['t', s @ ..] => super::t14::n8453(s, out),
		['u', s @ ..] => super::t14::n8454(s, out),
		['y', s @ ..] => super::t14::n8455(s, out),
		_ => false,
	}
}

/// Words ending -sin.
pub(crate) fn n2256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2252);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8358);
			s.is_empty()
		}
		['a', 'h', s @ ..] => {
			super::push(out, 8359);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 18415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8360);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18416);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8361);
			match s {
				[] => true,
				['c', 'u', s @ ..] => {
					super::push(out, 18417);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 28371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8362);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 18418);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 18419);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18420);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 28372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 18421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8363);
			match s {
				[] => true,
				['a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 18422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8364);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18423);
					match s {
						[] => true,
						['t', 'o', 'i', 'g', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 28373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8365);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18424);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 18425);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18426);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28374);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18427);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 18428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8366);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 28376);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 28377);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 28378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8367);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28379);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 28380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8368);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18430);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 28381);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 28382);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 28383);
							match s {
								[] => true,
								['a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 35751);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 35752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 28384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'd', s @ ..] => {
					super::push(out, 18431);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8369);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18433);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 18434);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 28386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 18435);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 28387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 8370);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 18436);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 28388);
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

/// Words ending -tin.
pub(crate) fn n2257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2253);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8371);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18437);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18438);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28389);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 28390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 18439);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 35753);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35754);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18440);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28391);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 35755);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40408);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 28392);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 28393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18441);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18442);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35756);
									s.is_empty()
								}
								['o', 'y', s @ ..] => {
									super::push(out, 35757);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 35758);
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
		['d', s @ ..] => {
			super::push(out, 8372);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 18443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8373);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 18444);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18445);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 28395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 18446);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 28396);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 28397);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 35759);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18447);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35760);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 35761);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 35762);
									match s {
										[] => true,
										['r', 'h', 't', 'y', 'r', 'e', '\u{1}', s @ ..] => {
											super::push(out, 40409);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 40410);
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
					super::push(out, 18448);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 28398);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 28399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8374);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 18449);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 28400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18450);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28401);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 28402);
							match s {
								[] => true,
								['r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 35763);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 28403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18451);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 28404);
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 35764);
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
			super::push(out, 8375);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 18452);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 28405);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 28406);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35766);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 35767);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 28407);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8376);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8377);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8378);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18454);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 28409);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35768);
									match s {
										[] => true,
										['d', 'a', '\u{1}', s @ ..] => {
											super::push(out, 40411);
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
					super::push(out, 18455);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 28410);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35769);
									s.is_empty()
								}
								['u', 't', 's', '-', s @ ..] => {
									super::push(out, 35770);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 40412);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 28411);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 35771);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 43007);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 43008);
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
							super::push(out, 28412);
							match s {
								[] => true,
								['q', '\u{1}', s @ ..] => {
									super::push(out, 35772);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18456);
					match s {
						[] => true,
						['c', s @ ..] => super::t14::n29643(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t14::n8477(s, out),
		['p', s @ ..] => super::t14::n8478(s, out),
		['r', s @ ..] => super::t14::n8479(s, out),
		['s', s @ ..] => super::t14::n8480(s, out),
		['t', s @ ..] => super::t14::n8481(s, out),
		['u', s @ ..] => super::t14::n8482(s, out),
		['z', s @ ..] => super::t14::n8483(s, out),
		['ä', s @ ..] => super::t14::n8484(s, out),
		_ => false,
	}
}

/// Words ending -uin.
pub(crate) fn n2258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2254);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 8387);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 8388);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18475);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 18476);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35780);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 35781);
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
			super::push(out, 8389);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vin.
pub(crate) fn n2259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2255);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8390);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8391);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 18477);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 28435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8392);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18479);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 18480);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -win.
pub(crate) fn n2260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2256);
	s.is_empty()
}

/// Words ending -xin.
pub(crate) fn n2261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2257);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8393);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 18481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8394);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 18482);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 18483);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18484);
					match s {
						[] => true,
						['a', 'f', s @ ..] => {
							super::push(out, 28436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8395);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18485);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 28437);
							match s {
								[] => true,
								['n', 'e', 'r', 'd', 'a', '\u{1}', s @ ..] => {
									super::push(out, 35782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18486);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18487);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 35783);
									match s {
										[] => true,
										['r', 'e', '\u{1}', s @ ..] => {
											super::push(out, 40415);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 35784);
									match s {
										[] => true,
										['t', 'n', 'a', 'x', '\u{1}', s @ ..] => {
											super::push(out, 40416);
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

/// Words ending -yin.
pub(crate) fn n2262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2258);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 8396);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18488);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zin.
pub(crate) fn n2263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2259);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8397);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 18489);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35785);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 35786);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 40417);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 35787);
									match s {
										[] => true,
										['n', 'a', 'l', 'g', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 40418);
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
				['m', s @ ..] => {
					super::push(out, 18490);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 28438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18491);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 28439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8398);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18492);
					match s {
						[] => true,
						['e', 'm', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 35788);
									s.is_empty()
								}
								['o', 'n', 'a', s @ ..] => {
									super::push(out, 35789);
									s.is_empty()
								}
								['r', 'o', s @ ..] => {
									super::push(out, 35790);
									s.is_empty()
								}
								['s', 'b', s @ ..] => {
									super::push(out, 35791);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 35792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 18493);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18494);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 28440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18496);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 18497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 8399);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 8400);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8401);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28442);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 28443);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 28444);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35793);
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
			super::push(out, 8402);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 18498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'e', 'r', 't', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 8403);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 8404);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18499);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 28445);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 28446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 18500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'm', '\u{1}', s @ ..] => {
			super::push(out, 8405);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 8406);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18501);
					match s {
						[] => true,
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 28447);
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

/// Words ending -ßin.
pub(crate) fn n2264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2260);
	s.is_empty()
}

/// Words ending -mann.
pub(crate) fn n8518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8420);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 18511);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 28455);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 35794);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 35795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 28456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 18512);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 18513);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 18514);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 28457);
					match s {
						[] => true,
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 35796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 18515);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 28458);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 35797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 28459);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 35798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', 'u', 'a', 'k', 'n', s @ ..] => {
			super::push(out, 18516);
			match s {
				[] => true,
				['r', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 28460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 18517);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 18518);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 35799);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 40421);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 43010);
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 44212);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 43011);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 40422);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 43012);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 40423);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 35800);
							match s {
								[] => true,
								['u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 40424);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 35801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 18519);
			match s {
				[] => true,
				['a', 'o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 28461);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 35802);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 35803);
							match s {
								[] => true,
								['r', 'o', 'h', 'e', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 40425);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 35804);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 35805);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 43013);
											s.is_empty()
										}
										['ü', 'm', '\u{1}', s @ ..] => {
											super::push(out, 43014);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 35806);
							s.is_empty()
						}
						['s', 'u', s @ ..] => {
							super::push(out, 35807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 28462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 18520);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 28463);
					match s {
						[] => true,
						['a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 35808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 18521);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 35809);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 35810);
							s.is_empty()
						}
						['t', 's', 'a', 'k', s @ ..] => {
							super::push(out, 35811);
							match s {
								[] => true,
								['r', 'e', 'i', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 40426);
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
			super::push(out, 18522);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 28464);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 35812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 28465);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 35813);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 40427);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 35814);
							s.is_empty()
						}
						['k', 'c', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 35815);
							s.is_empty()
						}
						['l', 'l', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 35816);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 35817);
							match s {
								[] => true,
								['m', 'i', 'z', '\u{1}', s @ ..] => {
									super::push(out, 40428);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 35818);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 35819);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 40429);
									match s {
										[] => true,
										['h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 43015);
											s.is_empty()
										}
										['m', '\u{1}', s @ ..] => {
											super::push(out, 43016);
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
					super::push(out, 28466);
					match s {
						[] => true,
						['e', 'w', 'r', 'e', 'u', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 35820);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 35821);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 40430);
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
			super::push(out, 18523);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 28467);
					match s {
						[] => true,
						['n', s @ ..] => super::t14::n37803(s, out),
						['u', s @ ..] => {
							super::push(out, 35823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', s @ ..] => super::t14::n29705(s, out),
				['l', s @ ..] => super::t14::n29706(s, out),
				['n', s @ ..] => super::t14::n29707(s, out),
				['p', s @ ..] => super::t14::n29708(s, out),
				['r', s @ ..] => super::t14::n29709(s, out),
				['t', s @ ..] => super::t14::n29710(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t14::n19037(s, out),
		['z', s @ ..] => super::t14::n19038(s, out),
		_ => false,
	}
}

/// Words ending -ergespann.
pub(crate) fn n45842(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 44215);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 44216);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 44217);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -inn.
pub(crate) fn n2269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2265);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 8430);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18531);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 35832);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 40446);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 43021);
													match s {
														[] => true,
														['p', 's', '\u{1}', s @ ..] => super::t14::n47194(s, out),
														_ => false,
													}
												}
												['k', s @ ..] => {
													super::push(out, 43022);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 40447);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 35833);
									s.is_empty()
								}
								['n', 'a', s @ ..] => {
									super::push(out, 35834);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35835);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 40448);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 40449);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 43023);
													s.is_empty()
												}
												['n', 'u', s @ ..] => {
													match s {
														[] => true,
														['d', 'l', 'i', 'b', 's', s @ ..] => super::t14::n47195(s, out),
														['k', s @ ..] => super::t14::n47196(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 40450);
											s.is_empty()
										}
										['r', 'h', s @ ..] => {
											super::push(out, 40451);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 40452);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 35836);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 35837);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40453);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 40454);
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
					super::push(out, 18532);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 28484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8431);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 18533);
					match s {
						[] => true,
						['s', 'd', s @ ..] => {
							super::push(out, 28485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 8432);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8433);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 18534);
					match s {
						[] => true,
						['ü', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 28486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 18535);
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28487);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 28488);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 28489);
							match s {
								[] => true,
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 35838);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 18536);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 18537);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28490);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 35839);
									s.is_empty()
								}
								['g', 'i', s @ ..] => {
									super::push(out, 35840);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 28491);
							match s {
								[] => true,
								['a', 'w', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40455);
											s.is_empty()
										}
										['r', 'e', 'd', 'n', 'i', 'r', '\u{1}', s @ ..] => {
											super::push(out, 40456);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 28492);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 40457);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 40458);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 28493);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 28494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18538);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28495);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 28496);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 28497);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 28498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18539);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 28499);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 28500);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 28501);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28502);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 35841);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8434);
			match s {
				[] => true,
				['z', 'r', 'ä', '\u{1}', s @ ..] => {
					super::push(out, 18541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => super::t14::n8533(s, out),
		['z', s @ ..] => {
			super::push(out, 8436);
			match s {
				[] => true,
				['l', s @ ..] => super::t14::n19058(s, out),
				['t', s @ ..] => super::t14::n19059(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gon.
pub(crate) fn n2277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2273);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8464);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8465);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18569);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 18570);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18571);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28518);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28519);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35851);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 35852);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18572);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 18573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 18574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 8466);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8467);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 18576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8468);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8469);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18578);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18579);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18580);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18581);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8470);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18582);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 28524);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 35853);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35854);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 35855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 8471);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hon.
pub(crate) fn n2278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2274);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8472);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 18584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 8473);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18585);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 28525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 18586);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28526);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18587);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18588);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 18589);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 28528);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28529);
							match s {
								[] => true,
								['p', 'i', 'r', 't', 's', 't', 'h', 'c', 'i', 'r', 'e', 'g', 't', 'l', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 35856);
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
			super::push(out, 8474);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18590);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18591);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 28530);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28531);
							match s {
								[] => true,
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 35857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 18592);
					match s {
						[] => true,
						['e', 'k', 'c', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 28532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18593);
					match s {
						[] => true,
						['l', 'o', s @ ..] => {
							super::push(out, 28533);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 35858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18594);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 28534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 18595);
					match s {
						[] => true,
						['b', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 28535);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 28536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8475);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18596);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 18597);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28537);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 28538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 18598);
					match s {
						[] => true,
						['p', 'z', s @ ..] => {
							super::push(out, 28539);
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

/// Words ending -ion.
pub(crate) fn n2279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2275);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8476);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8477);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 18599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 8478);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 18600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8479);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18601);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18602);
					match s {
						[] => true,
						['m', 'a', s @ ..] => {
							super::push(out, 28540);
							match s {
								[] => true,
								['r', 'y', 'p', '\u{1}', s @ ..] => {
									super::push(out, 35859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 28541);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 35860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 28542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18603);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 18604);
					match s {
						[] => true,
						['p', 'o', 'h', 't', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 28543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8480);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 18605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 8481);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 18606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 8482);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 28544);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 35861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 28545);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 35862);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 40461);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'a', s @ ..] => {
									super::push(out, 35863);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 40462);
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
				['i', 'l', 'e', 'r', 'h', s @ ..] => {
					super::push(out, 18607);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 28546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18608);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 28547);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8483);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 18609);
					match s {
						[] => true,
						['l', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 28548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8484);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 18611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 8485);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 18612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8486);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18613);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 28549);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35864);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 28550);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 28551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 18614);
					match s {
						[] => true,
						['h', 'c', s @ ..] => {
							super::push(out, 28552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 18615);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 18616);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28554);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 35865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 28555);
							match s {
								[] => true,
								['b', 'e', s @ ..] => {
									super::push(out, 35866);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 35867);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40463);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 28556);
							match s {
								[] => true,
								['t', 's', s @ ..] => {
									super::push(out, 35868);
									match s {
										[] => true,
										['a', 't', 'n', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 40464);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 40465);
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
					super::push(out, 18617);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 28557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 18618);
					match s {
						[] => true,
						['t', 's', 'i', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 28558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8487);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18619);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 28559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 18620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8488);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18621);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28560);
							s.is_empty()
						}
						['d', 'i', 'n', 'o', 'z', 'o', '\u{1}', s @ ..] => {
							super::push(out, 28561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 18622);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 28562);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 28563);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 28564);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 28565);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 28566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 18623);
					match s {
						[] => true,
						['m', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 28567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 18624);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28568);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 28569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18625);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 28570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 18626);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28571);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 28572);
							match s {
								[] => true,
								['m', 'o', 'k', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'a', s @ ..] => {
											super::push(out, 40466);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 40467);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 28573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t14::n8587(s, out),
		['r', s @ ..] => super::t14::n8588(s, out),
		['s', s @ ..] => super::t14::n8589(s, out),
		['t', s @ ..] => super::t14::n8590(s, out),
		['x', s @ ..] => {
			super::push(out, 8493);
			match s {
				[] => true,
				['e', 'l', 'p', s @ ..] => super::t14::n19175(s, out),
				_ => false,
			}
		}
		['z', s @ ..] => super::t14::n8592(s, out),
		_ => false,
	}
}

/// Words ending -kon.
pub(crate) fn n2281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2277);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8498);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8499);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 18658);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 28636);
							match s {
								[] => true,
								['o', 't', 'o', 'r', 'p', 'l', s @ ..] => {
									super::push(out, 35951);
									match s {
										[] => true,
										['a', 'n', 'i', 'd', 'r', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 40534);
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
					super::push(out, 18659);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 28637);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 35952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 18660);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8500);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18662);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18663);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28638);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 28639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18664);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18665);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28640);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 28641);
							match s {
								[] => true,
								['i', 'd', 'i', '\u{1}', s @ ..] => {
									super::push(out, 35953);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 28642);
							match s {
								[] => true,
								['o', 'i', 'k', 's', '\u{1}', s @ ..] => {
									super::push(out, 35954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 18666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8501);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18667);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35955);
									s.is_empty()
								}
								['l', 'e', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 40535);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 40536);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 35956);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 35957);
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
			super::push(out, 8502);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 18668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8503);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 18669);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lon.
pub(crate) fn n2282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2278);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8504);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18670);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 18671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28644);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 28645);
							match s {
								[] => true,
								['e', 'c', 'n', 'e', s @ ..] => {
									match s {
										[] => true,
										['i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 40537);
											s.is_empty()
										}
										['l', 'e', s @ ..] => {
											match s {
												[] => true,
												['t', '\u{1}', s @ ..] => {
													super::push(out, 43073);
													s.is_empty()
												}
												['y', 'm', '\u{1}', s @ ..] => {
													super::push(out, 43074);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 40538);
											s.is_empty()
										}
										['t', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 40539);
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
					super::push(out, 18672);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18673);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8505);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 18675);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 18676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 8506);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18677);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 18678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8507);
			match s {
				[] => true,
				['t', 'a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28647);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 35958);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 35959);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 35960);
									match s {
										[] => true,
										['t', 'r', s @ ..] => {
											super::push(out, 40540);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 28648);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8508);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 8509);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 8510);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18679);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['l', 'e', 's', 's', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 35961);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 35962);
									match s {
										[] => true,
										['e', 'h', 'ö', 'h', '\u{1}', s @ ..] => {
											super::push(out, 40541);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 35963);
									s.is_empty()
								}
								['t', 'f', 'u', 'l', 'ß', 'i', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 35964);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 18680);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28650);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 35965);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 35966);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 40542);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 28651);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28652);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 35967);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 35968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 28653);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 35969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 28654);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 35970);
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
			super::push(out, 8511);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 18681);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 18682);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 28655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 18683);
					match s {
						[] => true,
						['i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 28656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18684);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8512);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 8513);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18687);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 18688);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 18689);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28659);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 28660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18690);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mon.
pub(crate) fn n2283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2279);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8514);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 18691);
					match s {
						[] => true,
						['e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 28661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 18692);
					match s {
						[] => true,
						['r', 'e', 't', 't', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 28662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8515);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 18693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8516);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 28663);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 28664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8517);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 18694);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18695);
					match s {
						[] => true,
						['e', 'h', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35971);
									s.is_empty()
								}
								['l', 'a', 'u', 'x', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 35972);
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
			super::push(out, 8518);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18696);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 28665);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8519);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 18698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 8520);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 18699);
					s.is_empty()
				}
				['z', 'a', '\u{1}', s @ ..] => {
					super::push(out, 18700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 8521);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 8522);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 18702);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -non.
pub(crate) fn n2284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2280);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8523);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8524);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 18703);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 18704);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 18705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28666);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 28667);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 28668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18706);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 18707);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 28669);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 28670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8525);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 18709);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 28671);
							s.is_empty()
						}
						['t', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 28672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 18710);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 18711);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 28673);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 28674);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 35974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 18712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 8526);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 18713);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8527);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 18715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8528);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 8529);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pon.
pub(crate) fn n2286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2282);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 8537);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 8538);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 18718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8539);
			match s {
				[] => true,
				['t', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 18719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8540);
			match s {
				[] => true,
				['o', 'r', s @ ..] => {
					super::push(out, 18720);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ron.
pub(crate) fn n2287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2283);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8541);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 18721);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28675);
							s.is_empty()
						}
						['l', 'l', 'o', 'w', 'm', 'u', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 28676);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 28677);
							s.is_empty()
						}
						['r', 'e', 'd', 'n', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 28678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 18722);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 18723);
					match s {
						[] => true,
						['e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 28680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 18724);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 28681);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 28682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 18725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 8542);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 8543);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18726);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 28683);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 28684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18727);
					match s {
						[] => true,
						['e', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35975);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 35976);
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
			super::push(out, 8544);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 18728);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 18729);
					match s {
						[] => true,
						['c', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 28685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 18730);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18731);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28686);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 28687);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 28688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 8545);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 8546);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 18732);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18733);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28689);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35977);
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
			super::push(out, 8547);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 8548);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18735);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8549);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8550);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18737);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 28690);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 28691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8551);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18738);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 28692);
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 35978);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 28693);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 35979);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 28694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 18739);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 18740);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 28696);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 28697);
							match s {
								[] => true,
								['-', 'a', 'd', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 35980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 18741);
					match s {
						[] => true,
						['e', 'l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 35981);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 35982);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40543);
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
					super::push(out, 18742);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 28698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18743);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 18744);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28699);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 35983);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 35984);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 35985);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 28700);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 28701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 18745);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 18746);
					match s {
						[] => true,
						['l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 28702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8552);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 28703);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 28704);
							match s {
								[] => true,
								['l', 'e', 'g', 'e', 'i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 35986);
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
			super::push(out, 8553);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 18747);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 18748);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -son.
pub(crate) fn n2288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2284);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8554);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18749);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8555);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 18751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8556);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18752);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28705);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 28706);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 35987);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 35988);
									match s {
										[] => true,
										['e', 'f', s @ ..] => {
											super::push(out, 40544);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'o', s @ ..] => {
									super::push(out, 35989);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											super::push(out, 40545);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'p', s @ ..] => {
									super::push(out, 35990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 18753);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 18754);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 18755);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 28708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18756);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 28709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18757);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 28710);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 28711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18758);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 28712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8557);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8558);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18760);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18761);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 28713);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 28714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8559);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 18762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8560);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 18763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8561);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18764);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 18765);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28715);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8562);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18766);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 18767);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8563);
			match s {
				[] => true,
				['e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 18769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 8564);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 18770);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ton.
pub(crate) fn n2289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2285);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8565);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 8566);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 18771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 8567);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 18772);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18773);
					match s {
						[] => true,
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 28716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18774);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18775);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 18776);
					match s {
						[] => true,
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 28717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18777);
					match s {
						[] => true,
						['n', 'e', 'g', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 28718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 8568);
			match s {
				[] => true,
				['l', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 18778);
					s.is_empty()
				}
				['r', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 18779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 8569);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 8570);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8571);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 18780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28719);
							s.is_empty()
						}
						['g', 'n', 'i', 'l', 'c', 'y', 'c', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 28720);
							s.is_empty()
						}
						['h', 'c', 's', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 28721);
							s.is_empty()
						}
						['l', 'h', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 28722);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 28723);
							s.is_empty()
						}
						['r', 'e', 'g', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 28724);
							s.is_empty()
						}
						['t', 'h', 'c', 'i', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 28725);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 28726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 18781);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 18782);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 18783);
					match s {
						[] => true,
						['a', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 28728);
							s.is_empty()
						}
						['n', 'a', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 28729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 18784);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 28730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 18785);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 18786);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 18787);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 28731);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 28732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 18788);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28733);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 28734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 18789);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28735);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 28736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 8572);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 18790);
					match s {
						[] => true,
						['e', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 28737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8573);
			match s {
				[] => true,
				['ä', s @ ..] => {
					super::push(out, 18791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8574);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18792);
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 28738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 18793);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18794);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18795);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28739);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18796);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 18797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 8575);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 18798);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 18799);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8576);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 18800);
					match s {
						[] => true,
						['g', 'n', 'i', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 28741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18801);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 28742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8577);
			match s {
				[] => true,
				['m', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 28743);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 28744);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8578);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18802);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 28745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 18803);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18804);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 28746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18805);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 28747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 18806);
					s.is_empty()
				}
				['u', 'a', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 18807);
					s.is_empty()
				}
				['ü', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 18808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8579);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18809);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28748);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 28749);
							match s {
								[] => true,
								['a', 'm', 'm', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 35991);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 28750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 18810);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18811);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18812);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 18813);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8580);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28752);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 35992);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 28753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 18815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8581);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18816);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 18817);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 28754);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 28755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18818);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 28756);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t14::n8680(s, out),
		['t', s @ ..] => super::t14::n8681(s, out),
		['u', s @ ..] => {
			super::push(out, 8584);
			match s {
				[] => true,
				['a', s @ ..] => super::t14::n19359(s, out),
				['f', s @ ..] => {
					super::push(out, 18834);
					s.is_empty()
				}
				['m', s @ ..] => super::t14::n19361(s, out),
				_ => false,
			}
		}
		['w', s @ ..] => super::t14::n8683(s, out),
		['y', s @ ..] => super::t14::n8684(s, out),
		['z', s @ ..] => super::t14::n8685(s, out),
		['ß', s @ ..] => super::t14::n8686(s, out),
		['û', s @ ..] => super::t14::n8687(s, out),
		_ => false,
	}
}

/// Words ending -von.
pub(crate) fn n2291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2287);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8590);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -won.
pub(crate) fn n2292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2288);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8591);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xon.
pub(crate) fn n2293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2289);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8592);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18841);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8593);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8594);
			match s {
				[] => true,
				['d', 'a', 'r', 'a', 'p', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 28777);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'a', s @ ..] => {
									super::push(out, 35994);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 35995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 18844);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yon.
pub(crate) fn n2294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2290);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8595);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 8596);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 8597);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 8598);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8599);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28778);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 28779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8600);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8601);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zon.
pub(crate) fn n2295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2291);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8602);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8603);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8604);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -çon.
pub(crate) fn n2297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2293);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8605);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8606);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word bleikern.
pub(crate) fn n30042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28784);
	s.is_empty()
}

/// The word antikern.
pub(crate) fn n30043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28785);
	s.is_empty()
}

/// The word grafikkern.
pub(crate) fn n19389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18859);
	s.is_empty()
}

/// Words ending -lkern.
pub(crate) fn n19390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18860);
	match s {
		[] => true,
		['ö', s @ ..] => {
			super::push(out, 28786);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mkern.
pub(crate) fn n19391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18861);
	match s {
		[] => true,
		['u', 'i', 'l', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 28787);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nkern.
pub(crate) fn n19392(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 28788);
			match s {
				[] => true,
				['r', 'u', s @ ..] => {
					super::push(out, 35998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 28789);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 35999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 28790);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 36000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'g', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 28791);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 28792);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 28793);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 36001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rkern.
pub(crate) fn n19393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18862);
	match s {
		[] => true,
		['e', 'd', 'e', 'f', '\u{1}', s @ ..] => {
			super::push(out, 28794);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 28795);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 28796);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -skern.
pub(crate) fn n19394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18863);
	s.is_empty()
}

/// Words ending -tkern.
pub(crate) fn n19395(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 28797);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 28798);
			s.is_empty()
		}
		['s', 'e', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 28799);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nußkern.
pub(crate) fn n19396(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 28800);
			s.is_empty()
		}
		['d', 'r', 'e', '\u{1}', s @ ..] => {
			super::push(out, 28801);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 36002);
					s.is_empty()
				}
				['e', 's', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 36003);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ökern.
pub(crate) fn n19397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18864);
	s.is_empty()
}

/// Words ending -lern.
pub(crate) fn n8718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8620);
	s.is_empty()
}

/// Words ending -mern.
pub(crate) fn n8719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8621);
	s.is_empty()
}

/// Words ending -nern.
pub(crate) fn n8720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8622);
	s.is_empty()
}

/// Words ending -pern.
pub(crate) fn n8721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8623);
	s.is_empty()
}

/// Words ending -sern.
pub(crate) fn n8722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8624);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 18865);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 28802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 18866);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tern.
pub(crate) fn n8723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8625);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 18867);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 18868);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 18869);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 18870);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 18871);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 28803);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 28804);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 36004);
							s.is_empty()
						}
						['ß', 'o', 'r', 'g', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 40546);
									s.is_empty()
								}
								['r', 'u', 'n', s @ ..] => {
									super::push(out, 40547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 28805);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 28806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 18872);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 36005);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 36006);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 18873);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 18874);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28807);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 28808);
					match s {
						[] => true,
						['m', 'e', 'l', 'a', 's', 'u', 'h', 't', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 36007);
							s.is_empty()
						}
						['r', 'e', 'd', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 36008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['a', s @ ..] => {
					super::push(out, 28809);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 28810);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 28811);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36009);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 40548);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 36010);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 36011);
							match s {
								[] => true,
								['-', s @ ..] => {
									match s {
										[] => true,
										['d', 'o', 'o', 'w', 'y', 'l', 'l', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 43075);
											s.is_empty()
										}
										['e', 'n', 'y', 'a', 'w', '-', 'n', 'h', 'o', 'j', '\u{1}', s @ ..] => {
											super::push(out, 43076);
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
					super::push(out, 28812);
					s.is_empty()
				}
				['g', 'r', 'e', 'w', 'z', '\u{1}', s @ ..] => {
					super::push(out, 28813);
					s.is_empty()
				}
				['h', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 28814);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 28815);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 28816);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 28817);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 36012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 28818);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 36013);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 40549);
									match s {
										[] => true,
										['i', 'e', 'r', 't', 'p', 'u', 'a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 43077);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'e', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 40550);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 36014);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 36015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 28819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36016);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 36017);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 36018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 28820);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 28821);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 36019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 28822);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 28823);
					match s {
						[] => true,
						['a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 36020);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 36021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 28824);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 28825);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 36022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 18875);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 28826);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 36023);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 40551);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 28827);
					match s {
						[] => true,
						['z', 'r', s @ ..] => {
							super::push(out, 36024);
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

/// Words ending -uern.
pub(crate) fn n8724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8626);
	s.is_empty()
}

/// Words ending -zern.
pub(crate) fn n8725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8627);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 18876);
			match s {
				[] => true,
				['o', 'k', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 36025);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									super::push(out, 40552);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'g', 'u', s @ ..] => {
							super::push(out, 36026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 18877);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 28828);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßern.
pub(crate) fn n8726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8628);
	s.is_empty()
}

/// Words ending -irn.
pub(crate) fn n2300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2296);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8629);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 18878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8630);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 8631);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 8632);
			match s {
				[] => true,
				['ß', 'i', s @ ..] => {
					super::push(out, 18880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8633);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28829);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 28830);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['l', 'a', 'r', 't', 'n', 'e', 'z', '\u{1}', s @ ..] => {
											super::push(out, 40553);
											s.is_empty()
										}
										['s', 'e', 'g', 'a', 't', '\u{1}', s @ ..] => {
											super::push(out, 40554);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 28831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 8634);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 18881);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -orn.
pub(crate) fn n2301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2297);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 8635);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 8636);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 18882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 8637);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18883);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 18884);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 18885);
					match s {
						[] => true,
						['g', 'e', 'w', 'r', s @ ..] => {
							super::push(out, 28832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 18886);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 18887);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18888);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['m', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 36027);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 36028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'a', 'z', 'a', '\u{1}', s @ ..] => {
							super::push(out, 28834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18889);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28835);
							s.is_empty()
						}
						['e', 't', 'r', 'y', 'm', '\u{1}', s @ ..] => {
							super::push(out, 28836);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 28837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18890);
					match s {
						[] => true,
						['e', 'u', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 28838);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 28839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18891);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18892);
					match s {
						[] => true,
						['s', 'i', 'r', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 28840);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 18893);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 18894);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 8638);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 18895);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28843);
							s.is_empty()
						}
						['g', 'r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 28844);
							s.is_empty()
						}
						['l', 'l', 'a', 'b', 'e', 'e', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 28845);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 28846);
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									super::push(out, 36029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 18896);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 18897);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 18898);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 28848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 18899);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 28849);
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 36030);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18900);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28850);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36031);
									match s {
										[] => true,
										['r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 40555);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'e', 'i', 'z', '\u{1}', s @ ..] => {
									super::push(out, 36032);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 36033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36034);
									s.is_empty()
								}
								['w', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 36035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 't', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', 'l', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 36036);
									s.is_empty()
								}
								['i', 'r', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 36037);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'h', 's', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 28851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 18901);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 18902);
					match s {
						[] => true,
						['e', 'v', 'l', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 28852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18903);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 28853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18904);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28854);
							s.is_empty()
						}
						['f', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 28855);
							s.is_empty()
						}
						['o', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 28856);
							s.is_empty()
						}
						['s', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 28857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 8639);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18905);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 28858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 18906);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 18907);
					s.is_empty()
				}
				['i', 'e', 'w', 'z', '\u{1}', s @ ..] => {
					super::push(out, 18908);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 18909);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28859);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 36038);
									match s {
										[] => true,
										['p', 'a', '\u{1}', s @ ..] => {
											super::push(out, 40556);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 36039);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 36040);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 28860);
							match s {
								[] => true,
								['o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 36041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 18910);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28861);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 28862);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 36042);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t14::n19445(s, out),
				['s', s @ ..] => {
					super::push(out, 18912);
					match s {
						[] => true,
						['i', 'e', 'r', 'k', s @ ..] => super::t14::n30128(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t14::n19447(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8640);
			s.is_empty()
		}
		['p', s @ ..] => super::t14::n8739(s, out),
		['z', s @ ..] => super::t14::n8740(s, out),
		_ => false,
	}
}

/// Words ending -rrn.
pub(crate) fn n2302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2298);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8643);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urn.
pub(crate) fn n2303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2299);
	match s {
		[] => true,
		['g', 's', 's', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 8644);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 8645);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28870);
							s.is_empty()
						}
						['o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 28871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8646);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -örn.
pub(crate) fn n2304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2300);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 8647);
			match s {
				[] => true,
				['e', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28872);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 28873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8648);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word gefährdungsrisiko.
pub(crate) fn n45906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43078);
	s.is_empty()
}

/// Words ending -kungsrisiko.
pub(crate) fn n45907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43079);
	s.is_empty()
}

/// Words ending -rungsrisiko.
pub(crate) fn n45908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43080);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 44241);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 44242);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tungsrisiko.
pub(crate) fn n45909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43081);
	s.is_empty()
}

/// The word verletzungsrisiko.
pub(crate) fn n45910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43082);
	s.is_empty()
}

/// Words ending -asko.
pub(crate) fn n8938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8836);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 19035);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 19036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -esko.
pub(crate) fn n8939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8837);
	s.is_empty()
}

/// Words ending -isko.
pub(crate) fn n8940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8838);
	s.is_empty()
}

/// Words ending -uko.
pub(crate) fn n2428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2424);
	s.is_empty()
}

/// Words ending -öko.
pub(crate) fn n2429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2425);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 8839);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word zahlungskonto.
pub(crate) fn n45911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43083);
	s.is_empty()
}

/// Words ending -fporto.
pub(crate) fn n30222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28950);
	s.is_empty()
}

/// Words ending -sporto.
pub(crate) fn n30223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28951);
	s.is_empty()
}

/// Words ending -sto.
pub(crate) fn n2503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2499);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8985);
			match s {
				[] => true,
				['p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 19132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8986);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8987);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tto.
pub(crate) fn n2504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2500);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8988);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 19133);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 19134);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 19135);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 28952);
							match s {
								[] => true,
								['n', 'a', 'i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 36082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'a', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 28953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 19136);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 19137);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19138);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 28954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 19139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8989);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 8990);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 19140);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 28955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 19141);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19142);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 28956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19143);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 28957);
							match s {
								[] => true,
								['r', 's', s @ ..] => {
									super::push(out, 36083);
									match s {
										[] => true,
										['h', 'c', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 40567);
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
		['u', s @ ..] => {
			super::push(out, 8991);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 19144);
					match s {
						[] => true,
						['c', 's', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 28958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 19145);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 19146);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uto.
pub(crate) fn n2505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2501);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8992);
			match s {
				[] => true,
				['t', 'l', 'e', 'w', 'm', 'u', '\u{1}', s @ ..] => {
					super::push(out, 19147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 8993);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 19148);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28959);
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

/// Words ending -äto.
pub(crate) fn n2506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2502);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 8994);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word elektronenmikroskop.
pub(crate) fn n45914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43086);
	s.is_empty()
}

/// The word rasterelektronenmikroskop.
pub(crate) fn n45915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43087);
	s.is_empty()
}

/// The word transmissionselektronenmikroskop.
pub(crate) fn n45916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43088);
	s.is_empty()
}

/// Words ending -ktop.
pub(crate) fn n9182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9074);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 19181);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 19182);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -otop.
pub(crate) fn n9183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9075);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 19183);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 19184);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ptop.
pub(crate) fn n9184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9076);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19185);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stop.
pub(crate) fn n9185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9077);
	s.is_empty()
}

/// Words ending -ttop.
pub(crate) fn n9186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9078);
	s.is_empty()
}

/// Words ending -wop.
pub(crate) fn n2576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2572);
	match s {
		[] => true,
		['g', '\u{1}', s @ ..] => {
			super::push(out, 9079);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -har.
pub(crate) fn n2616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2612);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9166);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 19246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 9167);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19247);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 19248);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 29010);
							match s {
								[] => true,
								['n', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 36096);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 29011);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 29012);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 36097);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 29013);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 29014);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 36098);
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
		['o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 9168);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 9169);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9170);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 19250);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iar.
pub(crate) fn n2617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2613);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 9171);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					super::push(out, 19251);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 9172);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9173);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 9174);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 9175);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jar.
pub(crate) fn n2618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2614);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9176);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 9177);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19253);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 19254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 9178);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 19255);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 19256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9179);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9180);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kar.
pub(crate) fn n2619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2615);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9181);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 9182);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 19257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9183);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9184);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9185);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 19258);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lar.
pub(crate) fn n2620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2616);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9186);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 9187);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9188);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 19259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9189);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 9190);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 19260);
					s.is_empty()
				}
				['r', 'e', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 19261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9191);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 19262);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 29016);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 36099);
									match s {
										[] => true,
										['s', 'u', '\u{1}', s @ ..] => {
											super::push(out, 40579);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'e', 't', 'r', 'e', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 36100);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 36101);
									s.is_empty()
								}
								['r', 'e', 'b', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 36102);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 29017);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 29018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 19263);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 29019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9192);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 19264);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 19265);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 19266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 9193);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 19267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 9194);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 19268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9195);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 19269);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 19270);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 19271);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 19272);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 19273);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 19274);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 29020);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 36103);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19275);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 19276);
					match s {
						[] => true,
						['i', 'p', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 29021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 19277);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mar.
pub(crate) fn n2621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2617);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 9196);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nar.
pub(crate) fn n2622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2618);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9197);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 19278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 19279);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 19280);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 29023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9198);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 19281);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 19282);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 29024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9199);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 19283);
					match s {
						[] => true,
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 29025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 19284);
					match s {
						[] => true,
						['p', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 29026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19285);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 29027);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 29028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19286);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -par.
pub(crate) fn n2624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2620);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9200);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rar.
pub(crate) fn n2625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2621);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 9201);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 9202);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sar.
pub(crate) fn n2626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2622);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9204);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 19287);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 19288);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 29029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9205);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9206);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 19290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 9207);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19291);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 29030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 19292);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 19293);
					match s {
						[] => true,
						['m', 'm', 'o', 'k', 's', 'g', s @ ..] => {
							super::push(out, 29031);
							match s {
								[] => true,
								['n', 'u', s @ ..] => {
									super::push(out, 36104);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 19294);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 29032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9208);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 19295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 9209);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 19296);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tar.
pub(crate) fn n2627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2623);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9210);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 19297);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 19298);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 19299);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 19300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9211);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 19301);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 19302);
					match s {
						[] => true,
						['i', 'k', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 36105);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 36106);
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
			super::push(out, 9212);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 19303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9213);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					super::push(out, 19304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9214);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 19305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29033);
							s.is_empty()
						}
						['o', 'p', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 29034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 9215);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 19306);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 29035);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 29036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9216);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19307);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 29037);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 29038);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									super::push(out, 36107);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 36108);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 29039);
							match s {
								[] => true,
								['e', 'f', 'p', 'o', 'd', 'n', 'a', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 36109);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'l', s @ ..] => {
							super::push(out, 29040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9217);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 19308);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 29041);
							match s {
								[] => true,
								['m', 'o', 'k', s @ ..] => {
									match s {
										[] => true,
										['l', 'e', 'b', 'i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 40580);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 40581);
											s.is_empty()
										}
										['o', 'i', 'd', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 40582);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['e', 'z', 't', 'e', 's', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 43089);
													s.is_empty()
												}
												['g', s @ ..] => {
													super::push(out, 43090);
													s.is_empty()
												}
												['r', 'e', 'v', '\u{1}', s @ ..] => {
													super::push(out, 43091);
													s.is_empty()
												}
												['s', 'a', 'h', '\u{1}', s @ ..] => {
													super::push(out, 43092);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 's', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 40583);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 29042);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 36110);
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
			super::push(out, 9218);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 19309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 9219);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19310);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 19311);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 19312);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 29043);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 36111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 19313);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 29044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 19314);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 19315);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 29045);
							match s {
								[] => true,
								['a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 36112);
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
		['ä', s @ ..] => {
			super::push(out, 9220);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uar.
pub(crate) fn n2628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2624);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 9221);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 9222);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 19316);
					match s {
						[] => true,
						['n', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 29046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 19317);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -var.
pub(crate) fn n2629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2625);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9223);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9224);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 19318);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 19319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 9225);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9226);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 19320);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -war.
pub(crate) fn n2630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2626);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 9227);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yar.
pub(crate) fn n2631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2627);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 9228);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zar.
pub(crate) fn n2632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2628);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9229);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 9230);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 19321);
					match s {
						[] => true,
						['a', 'l', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 29047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 19322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 9231);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9232);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9233);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ilber.
pub(crate) fn n19895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19342);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 36122);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 40584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 36123);
					match s {
						[] => true,
						['c', 'i', s @ ..] => {
							super::push(out, 40585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 36124);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mber.
pub(crate) fn n9348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9239);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19343);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 19344);
			match s {
				[] => true,
				['t', 'a', s @ ..] => {
					super::push(out, 29071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 19345);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 29072);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 29073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 19346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -phober.
pub(crate) fn n19900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19347);
	match s {
		[] => true,
		['o', 'i', 'l', 'b', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 29074);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tober.
pub(crate) fn n19901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19348);
	s.is_empty()
}

/// Words ending -rber.
pub(crate) fn n9350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9241);
	s.is_empty()
}

/// Words ending -uber.
pub(crate) fn n9351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9242);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 19349);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 29075);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 29076);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 29077);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 36125);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									super::push(out, 40586);
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
		['t', s @ ..] => {
			super::push(out, 19350);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 19351);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äber.
pub(crate) fn n9352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9243);
	s.is_empty()
}

/// Words ending -öber.
pub(crate) fn n9353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9244);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 19352);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -über.
pub(crate) fn n9354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9245);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 19353);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 19354);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 19355);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cer.
pub(crate) fn n2637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2633);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9246);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9247);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9248);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9249);
			match s {
				[] => true,
				['m', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 19356);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

