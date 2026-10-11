use flexion::Path;

/// Words ending -мишься.
pub(crate) fn n2736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2689);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7617);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14513);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21324);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14514);
					match s {
						[] => true,
						['х', '\u{1}', s @ ..] => {
							super::push(out, 21325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14515);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 21326);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26881);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26882);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 14516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7618);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14517);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 21328);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21329);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14518);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14519);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 21331);
							s.is_empty()
						}
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26884);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 26885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 14520);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7619);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14521);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 14522);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21334);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26886);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26887);
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
		['й', s @ ..] => {
			super::push(out, 7620);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21335);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'л', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21337);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7621);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14523);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26888);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26889);
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
		['м', s @ ..] => {
			super::push(out, 7622);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7623);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14525);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21339);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26890);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21340);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 26891);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14526);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21341);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26892);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30622);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30623);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 30624);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30625);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 26893);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30626);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32608);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 32609);
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
				['л', s @ ..] => {
					super::push(out, 14527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21342);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21343);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 21344);
							s.is_empty()
						}
						['в', s @ ..] => super::t9::n22593(s, out),
						['д', s @ ..] => super::t9::n22594(s, out),
						['е', s @ ..] => super::t9::n22595(s, out),
						['з', s @ ..] => super::t9::n22596(s, out),
						['о', s @ ..] => super::t9::n22597(s, out),
						['с', s @ ..] => super::t9::n22598(s, out),
						['т', s @ ..] => super::t9::n22599(s, out),
						['ы', s @ ..] => super::t9::n22600(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t9::n15341(s, out),
				['р', s @ ..] => super::t9::n15342(s, out),
				['т', s @ ..] => super::t9::n15343(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t9::n7968(s, out),
		['т', s @ ..] => super::t9::n7969(s, out),
		['у', s @ ..] => super::t9::n7970(s, out),
		['ы', s @ ..] => super::t9::n7971(s, out),
		['ь', s @ ..] => super::t9::n7972(s, out),
		['ю', s @ ..] => super::t9::n7973(s, out),
		['я', s @ ..] => super::t9::n7974(s, out),
		_ => false,
	}
}

/// Words ending -нишься.
pub(crate) fn n2737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2690);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7631);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21376);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21377);
							match s {
								[] => true,
								['р', 'а', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32613);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 32614);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 32615);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 32616);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 30635);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21378);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26919);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 14544);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 21379);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26920);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 14545);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 21380);
							match s {
								[] => true,
								['л', 'у', 'х', 'с', 'и', s @ ..] => {
									super::push(out, 26921);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30636);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21381);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 26922);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 21382);
							match s {
								[] => true,
								['а', 'в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30637);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 30638);
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
				['з', s @ ..] => {
					super::push(out, 14546);
					match s {
						[] => true,
						['у', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26923);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14547);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14548);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21384);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 21385);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30639);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30640);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 30641);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 30642);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32617);
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
						['и', s @ ..] => {
							super::push(out, 21386);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 21387);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30643);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 30644);
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
				['л', s @ ..] => {
					super::push(out, 14549);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t9::n15364(s, out),
				['р', s @ ..] => super::t9::n15365(s, out),
				['с', s @ ..] => super::t9::n15366(s, out),
				['т', s @ ..] => super::t9::n15367(s, out),
				['ф', s @ ..] => super::t9::n15368(s, out),
				['щ', s @ ..] => super::t9::n15369(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t9::n7976(s, out),
		['в', s @ ..] => super::t9::n7977(s, out),
		['г', s @ ..] => super::t9::n7978(s, out),
		['д', s @ ..] => super::t9::n7979(s, out),
		['е', s @ ..] => super::t9::n7980(s, out),
		['ж', s @ ..] => super::t9::n7981(s, out),
		['з', s @ ..] => super::t9::n7982(s, out),
		['и', s @ ..] => super::t9::n7983(s, out),
		['л', s @ ..] => super::t9::n7984(s, out),
		['м', s @ ..] => super::t9::n7985(s, out),
		['о', s @ ..] => super::t9::n7986(s, out),
		['п', s @ ..] => super::t9::n7987(s, out),
		['р', s @ ..] => super::t9::n7988(s, out),
		['с', s @ ..] => super::t9::n7989(s, out),
		['т', s @ ..] => super::t9::n7990(s, out),
		['у', s @ ..] => super::t9::n7991(s, out),
		['ч', s @ ..] => super::t9::n7992(s, out),
		['ш', s @ ..] => super::t9::n7993(s, out),
		['ы', s @ ..] => super::t9::n7994(s, out),
		['ю', s @ ..] => super::t9::n7995(s, out),
		['я', s @ ..] => super::t9::n7996(s, out),
		_ => false,
	}
}

/// Words ending -оишься.
pub(crate) fn n2738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2691);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7653);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14646);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14647);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7654);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14649);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21573);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21574);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27104);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 21575);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27105);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 27106);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21576);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 21577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14650);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 14651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7655);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14652);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 21578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 14653);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 7656);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14654);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27107);
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['е', 'б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32660);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 32661);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 32662);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 30762);
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
		['л', s @ ..] => {
			super::push(out, 7657);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 14655);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21580);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 27108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14656);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21581);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7658);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7659);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14658);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14659);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 14660);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 14661);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 14662);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 14663);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7660);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14664);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14665);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 14666);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21585);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27109);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 21586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14667);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21587);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 21588);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 21589);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 21590);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21591);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21592);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14668);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 14669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21593);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21594);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21595);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27111);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 27112);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30763);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 30764);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 27113);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27114);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 27115);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 27116);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30765);
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
				['ы', s @ ..] => {
					super::push(out, 14670);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21596);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7661);
			match s {
				[] => true,
				['с', s @ ..] => super::t9::n15485(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пишься.
pub(crate) fn n2739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2692);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7662);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14672);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 14673);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21605);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27123);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 27124);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 21606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7663);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14674);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21607);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21608);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 21609);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21610);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27125);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21611);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27127);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14675);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21612);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27128);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 27129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 14676);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 21613);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27130);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 14677);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7664);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14678);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 14679);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 14680);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21615);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 14681);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21617);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 21618);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27131);
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
		['й', s @ ..] => {
			super::push(out, 7665);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7666);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14683);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27132);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 27133);
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
		['о', s @ ..] => {
			super::push(out, 7667);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14684);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21619);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27134);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 14685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21620);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21621);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 21622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21623);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21624);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21625);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27135);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14687);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 21626);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27136);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30767);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 30768);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 21627);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27137);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21628);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 27139);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30769);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30770);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30771);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21629);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27140);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14688);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21630);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21631);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21632);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27141);
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
		['р', s @ ..] => {
			super::push(out, 7668);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14689);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27142);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 27143);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 27144);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30772);
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
		['с', s @ ..] => super::t9::n8013(s, out),
		['у', s @ ..] => super::t9::n8014(s, out),
		['ы', s @ ..] => {
			super::push(out, 7671);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t9::n8016(s, out),
		_ => false,
	}
}

/// Words ending -ришься.
pub(crate) fn n2740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2693);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7673);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21657);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21658);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 21659);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 21660);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 21661);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27158);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30778);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 27159);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 21662);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 21663);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 21664);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 21665);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 27161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21666);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 27162);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 27163);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30779);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 27164);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 27165);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30780);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 27166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21667);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27167);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 21668);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 21669);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21670);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 14701);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 14702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21671);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21672);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 27169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 21673);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 21674);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27170);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 21675);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27171);
									match s {
										[] => true,
										['и', 'к', 'с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32664);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 32665);
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
						['о', s @ ..] => super::t9::n22937(s, out),
						['т', s @ ..] => {
							super::push(out, 21677);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 21678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => super::t9::n15517(s, out),
				['з', s @ ..] => super::t9::n15518(s, out),
				['и', s @ ..] => super::t9::n15519(s, out),
				['м', s @ ..] => super::t9::n15520(s, out),
				['п', s @ ..] => super::t9::n15521(s, out),
				['т', s @ ..] => super::t9::n15522(s, out),
				['у', s @ ..] => super::t9::n15523(s, out),
				['х', s @ ..] => super::t9::n15524(s, out),
				['ц', s @ ..] => super::t9::n15525(s, out),
				['ш', s @ ..] => super::t9::n15526(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t9::n8018(s, out),
		['в', s @ ..] => super::t9::n8019(s, out),
		['г', s @ ..] => super::t9::n8020(s, out),
		['д', s @ ..] => super::t9::n8021(s, out),
		['е', s @ ..] => super::t9::n8022(s, out),
		['з', s @ ..] => super::t9::n8023(s, out),
		['и', s @ ..] => super::t9::n8024(s, out),
		['к', s @ ..] => super::t9::n8025(s, out),
		['о', s @ ..] => super::t9::n8026(s, out),
		['т', s @ ..] => super::t9::n8027(s, out),
		['у', s @ ..] => super::t9::n8028(s, out),
		['ф', s @ ..] => super::t9::n8029(s, out),
		['х', s @ ..] => super::t9::n8030(s, out),
		['щ', s @ ..] => super::t9::n8031(s, out),
		['ы', s @ ..] => super::t9::n8032(s, out),
		['ю', s @ ..] => super::t9::n8033(s, out),
		['я', s @ ..] => super::t9::n8034(s, out),
		_ => false,
	}
}

/// Words ending -сишься.
pub(crate) fn n2741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2694);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7691);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14799);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 21881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 14800);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21882);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27336);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27337);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 27338);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30887);
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
				['г', s @ ..] => {
					super::push(out, 14801);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14802);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21884);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27339);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14803);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21885);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27340);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 21886);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27341);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14804);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 14805);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21887);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27343);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27344);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 27345);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 27346);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 27347);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30888);
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
		['е', s @ ..] => {
			super::push(out, 7692);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14806);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21888);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21889);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 21890);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 14807);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21892);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21893);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21894);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21895);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14808);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 21896);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27349);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 21897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14809);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21898);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21899);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21900);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14810);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21901);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27351);
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
		['и', s @ ..] => {
			super::push(out, 7693);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 7694);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14812);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21902);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27352);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 27353);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14813);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27354);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 27355);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30889);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 30890);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 27356);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30891);
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
		['о', s @ ..] => super::t10::n8039(s, out),
		['р', s @ ..] => {
			super::push(out, 7696);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14819);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14820);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21931);
							s.is_empty()
						}
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27385);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27386);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 27387);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 27388);
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
		['у', s @ ..] => super::t10::n8041(s, out),
		['ы', s @ ..] => {
			super::push(out, 7698);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14823);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21940);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 21941);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t10::n15638(s, out),
				_ => false,
			}
		}
		['э', s @ ..] => super::t10::n8043(s, out),
		_ => false,
	}
}

/// Words ending -тишься.
pub(crate) fn n2742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2695);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7700);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14826);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21944);
							match s {
								[] => true,
								['о', 'г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30912);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30913);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30914);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 30915);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', 'з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27401);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 14827);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21945);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 27403);
									match s {
										[] => true,
										['и', 'в', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32696);
													s.is_empty()
												}
												['б', s @ ..] => {
													super::push(out, 32697);
													match s {
														[] => true,
														['о', s @ ..] => super::t10::n35324(s, out),
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
						['х', s @ ..] => {
							super::push(out, 21946);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 27404);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30916);
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
				['г', s @ ..] => {
					super::push(out, 14828);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21947);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21948);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27405);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30917);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30918);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 27406);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14829);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21949);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21950);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21951);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21952);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21953);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14830);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 21954);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 27408);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 21955);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27409);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 21956);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27411);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27412);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 27413);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 27414);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30919);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 21957);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27415);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 30920);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21958);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t10::n15645(s, out),
				['н', s @ ..] => {
					super::push(out, 14832);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 21963);
							match s {
								[] => true,
								['о', 'м', s @ ..] => super::t10::n29069(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t10::n15647(s, out),
				['р', s @ ..] => super::t10::n15648(s, out),
				['х', s @ ..] => super::t10::n15649(s, out),
				['ч', s @ ..] => super::t10::n15650(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t10::n8045(s, out),
		['е', s @ ..] => super::t10::n8046(s, out),
		['и', s @ ..] => super::t10::n8047(s, out),
		['й', s @ ..] => super::t10::n8048(s, out),
		['л', s @ ..] => super::t10::n8049(s, out),
		['н', s @ ..] => super::t10::n8050(s, out),
		['о', s @ ..] => super::t10::n8051(s, out),
		['п', s @ ..] => super::t10::n8052(s, out),
		['р', s @ ..] => super::t10::n8053(s, out),
		['с', s @ ..] => super::t10::n8054(s, out),
		['у', s @ ..] => super::t10::n8055(s, out),
		['х', s @ ..] => super::t10::n8056(s, out),
		['ч', s @ ..] => super::t10::n8057(s, out),
		['ы', s @ ..] => super::t10::n8058(s, out),
		['э', s @ ..] => super::t10::n8059(s, out),
		['ю', s @ ..] => super::t10::n8060(s, out),
		['я', s @ ..] => super::t10::n8061(s, out),
		_ => false,
	}
}

/// Words ending -уишься.
pub(crate) fn n2743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2696);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 7718);
			match s {
				[] => true,
				['т', 'с', 'а', s @ ..] => {
					super::push(out, 14902);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22084);
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

/// Words ending -фишься.
pub(crate) fn n2744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2697);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7719);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14903);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 22085);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27575);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 27576);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 31033);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7720);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14904);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27577);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27578);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7721);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14906);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27579);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 27580);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31034);
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

/// Words ending -чишься.
pub(crate) fn n2745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2698);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7722);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14907);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 22087);
							s.is_empty()
						}
						['о', 'с', '\u{1}', s @ ..] => {
							super::push(out, 22088);
							s.is_empty()
						}
						['ы', 'д', s @ ..] => {
							super::push(out, 22089);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 14908);
					match s {
						[] => true,
						['л', 'у', 'б', '\u{1}', s @ ..] => {
							super::push(out, 22090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 14909);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14910);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22091);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27582);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 22092);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22093);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 27583);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14911);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27584);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 27585);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27586);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 27587);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31035);
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
				['р', s @ ..] => {
					super::push(out, 14912);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 22094);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27588);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27589);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22095);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22096);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31036);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31037);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31038);
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
				['с', s @ ..] => {
					super::push(out, 14913);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 22097);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14914);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27591);
									s.is_empty()
								}
								['а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31039);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31040);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31041);
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
		['в', s @ ..] => {
			super::push(out, 7723);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14915);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							super::push(out, 22098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7724);
			match s {
				[] => true,
				['е', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22099);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 22100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 14916);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27592);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 27593);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', '\u{1}', s @ ..] => {
							super::push(out, 22101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7725);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14917);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22102);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22103);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27594);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14918);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22104);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22105);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27595);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 27596);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31042);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31043);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 22106);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22107);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22108);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27597);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22109);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22110);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14919);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14920);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t10::n8070(s, out),
		['л', s @ ..] => {
			super::push(out, 7727);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t10::n8072(s, out),
		['н', s @ ..] => super::t10::n8073(s, out),
		['о', s @ ..] => super::t10::n8074(s, out),
		['р', s @ ..] => super::t10::n8075(s, out),
		['т', s @ ..] => {
			super::push(out, 7732);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t10::n8077(s, out),
		['ы', s @ ..] => super::t10::n8078(s, out),
		['ь', s @ ..] => super::t10::n8079(s, out),
		['ю', s @ ..] => super::t10::n8080(s, out),
		['я', s @ ..] => super::t10::n8081(s, out),
		_ => false,
	}
}

/// Words ending -шишься.
pub(crate) fn n2746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2699);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7738);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14984);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27696);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 27697);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14985);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22248);
							match s {
								[] => true,
								['ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31102);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31103);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 22249);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 22250);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27698);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 31104);
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
		['д', s @ ..] => {
			super::push(out, 7739);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 14986);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 22251);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7740);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14987);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 14988);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 22252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14989);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 14990);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14991);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22254);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									super::push(out, 27699);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 22255);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27700);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22256);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22257);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22258);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22259);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 22260);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27701);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31105);
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
		['и', s @ ..] => {
			super::push(out, 7741);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7742);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14994);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7743);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14996);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14997);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22262);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 27702);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22263);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 27703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14998);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27704);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 27705);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14999);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22264);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 27706);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22265);
							match s {
								[] => true,
								['о', 'т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31106);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 31107);
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
				['н', s @ ..] => {
					super::push(out, 15000);
					match s {
						[] => true,
						['о', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27707);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 27708);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 27709);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15001);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22266);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27710);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22267);
							match s {
								[] => true,
								['к', s @ ..] => super::t10::n29376(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t10::n15817(s, out),
				['т', s @ ..] => super::t10::n15818(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t10::n8088(s, out),
		['у', s @ ..] => super::t10::n8089(s, out),
		['ч', s @ ..] => super::t10::n8090(s, out),
		['ы', s @ ..] => super::t10::n8091(s, out),
		['ь', s @ ..] => super::t10::n8092(s, out),
		['э', s @ ..] => super::t10::n8093(s, out),
		['я', s @ ..] => super::t10::n8094(s, out),
		_ => false,
	}
}

/// Words ending -щишься.
pub(crate) fn n2747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2700);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7751);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15021);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22321);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31138);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31139);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 32748);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 31140);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32749);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 31141);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32750);
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
				['т', s @ ..] => {
					super::push(out, 15022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22322);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22323);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22324);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22325);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27750);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 22326);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22327);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27751);
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
		['б', s @ ..] => {
			super::push(out, 7752);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7753);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15024);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 22328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15025);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22329);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 27752);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31142);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31143);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 32751);
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 31144);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31145);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7754);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7755);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7756);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15028);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22331);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22332);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 27753);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 22333);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27754);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22334);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22335);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22336);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 22337);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27755);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 27756);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 22338);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15030);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7757);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15031);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 22340);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27758);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											super::push(out, 31146);
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
				['о', s @ ..] => {
					super::push(out, 15032);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 22341);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27759);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 27760);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 27761);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27762);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 31147);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 27763);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31148);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => super::t10::n23633(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7758);
			match s {
				[] => true,
				['о', s @ ..] => super::t10::n15849(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t10::n8103(s, out),
		['ы', s @ ..] => super::t10::n8104(s, out),
		['ю', s @ ..] => super::t10::n8105(s, out),
		_ => false,
	}
}

/// Words ending -ьишься.
pub(crate) fn n2748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2701);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 7762);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 15037);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -меваешь.
pub(crate) fn n2784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2736);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 7823);
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22371);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22372);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15104);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22373);
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

/// Words ending -неваешь.
pub(crate) fn n2785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2737);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7824);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 7825);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15105);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 15106);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7826);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15108);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 15109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7827);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22376);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22377);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27772);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'а', 'к', 'о', s @ ..] => {
					super::push(out, 15110);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15111);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15112);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 15113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7828);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15114);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7829);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15115);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15116);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 22381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 7830);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оеваешь.
pub(crate) fn n2786(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 7831);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 7832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -певаешь.
pub(crate) fn n2787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2738);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7833);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7834);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 7835);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7836);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7837);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7838);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 7839);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15117);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22382);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 22383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 15118);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 15119);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15120);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22385);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 22386);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 22387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 15121);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 15122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7840);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 7841);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -реваешь.
pub(crate) fn n2788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2739);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7842);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7843);
			match s {
				[] => true,
				['о', 'д', s @ ..] => {
					super::push(out, 15124);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22388);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7844);
			s.is_empty()
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22390);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 22391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 15125);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 15126);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 15127);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22392);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27773);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27774);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27775);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31153);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22393);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27776);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					super::push(out, 15128);
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					super::push(out, 15129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7845);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 7846);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 7847);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15130);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15131);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15133);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 15134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7848);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -теваешь.
pub(crate) fn n2790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2741);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7849);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7850);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 7851);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уеваешь.
pub(crate) fn n2791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2742);
	s.is_empty()
}

/// Words ending -чеваешь.
pub(crate) fn n2792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2743);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 7852);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7853);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15137);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шеваешь.
pub(crate) fn n2793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2744);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 7854);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15138);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щеваешь.
pub(crate) fn n2794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2745);
	s.is_empty()
}

/// Words ending -осбиваешь.
pub(crate) fn n15987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15160);
	s.is_empty()
}

/// Words ending -оотбиваешь.
pub(crate) fn n8228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7871);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 15161);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оубиваешь.
pub(crate) fn n8229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7872);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 15162);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овыбиваешь.
pub(crate) fn n8230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7873);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 15163);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -виваешь.
pub(crate) fn n2797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2748);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 7874);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7875);
			match s {
				[] => true,
				['р', 'п', 'о', s @ ..] => {
					super::push(out, 15164);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22415);
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

/// Words ending -гиваешь.
pub(crate) fn n2798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2749);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7876);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15165);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22417);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', 'т', s @ ..] => {
					super::push(out, 15166);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7877);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 7878);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15167);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27786);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 27787);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31156);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 27788);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 15168);
					match s {
						[] => true,
						['р', 'б', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27789);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27790);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31157);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 31158);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32755);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 27791);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31159);
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
				['я', s @ ..] => {
					super::push(out, 15169);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 22420);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7879);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22421);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27792);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27793);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 27794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22423);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7880);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22425);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22426);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27795);
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
		['р', s @ ..] => {
			super::push(out, 7881);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15170);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 27796);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31160);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 27797);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31161);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 31162);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 27798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 15171);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27799);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 27800);
									match s {
										[] => true,
										['в', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 32756);
													match s {
														[] => true,
														['н', s @ ..] => super::t10::n35335(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 32757);
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
		['у', s @ ..] => {
			super::push(out, 7882);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 7883);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15172);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27801);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 27802);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27803);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 27804);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t10::n16004(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t10::n8241(s, out),
		_ => false,
	}
}

/// Words ending -еиваешь.
pub(crate) fn n2799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2750);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7885);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15176);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22434);
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

/// Words ending -живаешь.
pub(crate) fn n2800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2751);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7886);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15177);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 15178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22435);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 22436);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 15179);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15180);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 15181);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15182);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15183);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22438);
							match s {
								[] => true,
								['о', 'д', s @ ..] => {
									super::push(out, 27817);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31168);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22439);
							match s {
								[] => true,
								['м', 'б', s @ ..] => {
									super::push(out, 27818);
									match s {
										[] => true,
										['о', 'о', s @ ..] => {
											super::push(out, 31169);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32760);
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
				['с', s @ ..] => {
					super::push(out, 15184);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 15185);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22441);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 22442);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 22443);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22444);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27819);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 22445);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 27820);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 27821);
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 27822);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 27823);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 27824);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22446);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22447);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27825);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 27826);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31170);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 27827);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31171);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 22448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7887);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7888);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7889);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15187);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7890);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15188);
					match s {
						[] => true,
						['о', 'д', s @ ..] => {
							super::push(out, 22450);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27828);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'о', 'п', s @ ..] => {
							super::push(out, 22451);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15189);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22452);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22453);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27830);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 27831);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 27832);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31172);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 32761);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32762);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 32763);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 31173);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32764);
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
						['з', s @ ..] => {
							super::push(out, 22454);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 15190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 7891);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15191);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 15192);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22456);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27833);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31174);
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
				['и', s @ ..] => super::t10::n16025(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t10::n8249(s, out),
		['л', s @ ..] => super::t10::n8250(s, out),
		['о', s @ ..] => super::t10::n8251(s, out),
		['р', s @ ..] => super::t10::n8252(s, out),
		['с', s @ ..] => super::t10::n8253(s, out),
		['т', s @ ..] => super::t10::n8254(s, out),
		['у', s @ ..] => super::t10::n8255(s, out),
		['ы', s @ ..] => super::t10::n8256(s, out),
		['ю', s @ ..] => super::t10::n8257(s, out),
		['я', s @ ..] => super::t10::n8258(s, out),
		_ => false,
	}
}

/// Words ending -зиваешь.
pub(crate) fn n2801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2752);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7902);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15214);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -киваешь.
pub(crate) fn n2802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2753);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7903);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7904);
			match s {
				[] => true,
				['д', 'д', s @ ..] => {
					super::push(out, 15215);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 15216);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15217);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22469);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27844);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31176);
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 27845);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 27846);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 31177);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 32765);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 31178);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32766);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 27847);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31179);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 27848);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31180);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 27849);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27850);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 27851);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31181);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 27852);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 27853);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31182);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 32767);
													match s {
														[] => true,
														['п', s @ ..] => super::t10::n35336(s, out),
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
						['ы', s @ ..] => {
							super::push(out, 22470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15218);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22471);
							match s {
								[] => true,
								['в', 'а', s @ ..] => {
									super::push(out, 27854);
									match s {
										[] => true,
										['з', 'о', s @ ..] => {
											super::push(out, 31183);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32768);
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
						['п', s @ ..] => {
							super::push(out, 22472);
							match s {
								[] => true,
								['о', 'п', s @ ..] => {
									super::push(out, 27855);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31184);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 27856);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31185);
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
				['м', s @ ..] => {
					super::push(out, 15219);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15220);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22473);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22474);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 22475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7905);
			match s {
				[] => true,
				['я', 'т', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22476);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27857);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7906);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15221);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15222);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15223);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15224);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27858);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 27859);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 27860);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31186);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 31187);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 27861);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31188);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 27862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 15225);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 15226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t10::n8264(s, out),
		['м', s @ ..] => {
			super::push(out, 7908);
			match s {
				[] => true,
				['у', 'р', s @ ..] => {
					super::push(out, 15227);
					match s {
						[] => true,
						['х', 'с', s @ ..] => {
							super::push(out, 22484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t10::n8266(s, out),
		['р', s @ ..] => super::t10::n8267(s, out),
		['с', s @ ..] => super::t10::n8268(s, out),
		['у', s @ ..] => super::t10::n8269(s, out),
		['ч', s @ ..] => {
			super::push(out, 7913);
			s.is_empty()
		}
		['ы', s @ ..] => super::t10::n8271(s, out),
		['ь', s @ ..] => super::t10::n8272(s, out),
		['ю', s @ ..] => super::t10::n8273(s, out),
		['я', s @ ..] => super::t10::n8274(s, out),
		_ => false,
	}
}

/// Words ending -ливаешь.
pub(crate) fn n2803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2754);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7916);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7917);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15254);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22514);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27898);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22515);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 15255);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22516);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 15256);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 15257);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 15258);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15259);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 15260);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15261);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 15262);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22519);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27900);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31202);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 15263);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22521);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7918);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15264);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 15265);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15266);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22523);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 22524);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27901);
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
		['в', s @ ..] => {
			super::push(out, 7919);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15267);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22525);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 22526);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 27902);
									match s {
										[] => true,
										['в', 'о', s @ ..] => {
											super::push(out, 31203);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32777);
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
						['н', s @ ..] => {
							super::push(out, 22527);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 22528);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27903);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 31204);
											match s {
												[] => true,
												['з', s @ ..] => {
													match s {
														[] => true,
														['о', s @ ..] => super::t10::n35338(s, out),
														['ы', s @ ..] => super::t10::n35339(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', 'с', '\u{1}', s @ ..] => {
											super::push(out, 31205);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 27904);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 27905);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15268);
					match s {
						[] => true,
						['о', 'м', 'и', s @ ..] => {
							super::push(out, 22530);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 15269);
					match s {
						[] => true,
						['н', 'ю', 'л', 'с', 'с', s @ ..] => {
							super::push(out, 22531);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27907);
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
		['д', s @ ..] => {
			super::push(out, 7920);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15270);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							super::push(out, 22532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7921);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22533);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27908);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 7922);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15271);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 15272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7923);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15273);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22535);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22536);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22537);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 22538);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 15274);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 15275);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15276);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 15277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7924);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7925);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15278);
					s.is_empty()
				}
				['д', s @ ..] => super::t10::n16120(s, out),
				['з', s @ ..] => {
					super::push(out, 15280);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 15281);
					s.is_empty()
				}
				['р', s @ ..] => super::t10::n16123(s, out),
				['с', s @ ..] => super::t10::n16124(s, out),
				['х', s @ ..] => {
					super::push(out, 15284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t10::n8285(s, out),
		['р', s @ ..] => super::t10::n8286(s, out),
		['с', s @ ..] => super::t10::n8287(s, out),
		['т', s @ ..] => super::t10::n8288(s, out),
		['у', s @ ..] => super::t10::n8289(s, out),
		['ш', s @ ..] => super::t10::n8290(s, out),
		['ы', s @ ..] => super::t10::n8291(s, out),
		['ю', s @ ..] => super::t10::n8292(s, out),
		['я', s @ ..] => super::t10::n8293(s, out),
		_ => false,
	}
}

/// Words ending -ниваешь.
pub(crate) fn n2804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2755);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7935);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15304);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 15305);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27929);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31217);
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 27930);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27931);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 31218);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 31219);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 31220);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 31221);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22557);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', '\u{1}', s @ ..] => {
					super::push(out, 15306);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15307);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 15308);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7936);
			match s {
				[] => true,
				['а', 'р', 'с', s @ ..] => {
					super::push(out, 15310);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22558);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27933);
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
		['г', s @ ..] => {
			super::push(out, 7937);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15311);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22559);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 22560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 15312);
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22561);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 27934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 15313);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 15314);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15315);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22565);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 22566);
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 22567);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 15316);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15317);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 15318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7938);
			match s {
				[] => true,
				['м', 'е', s @ ..] => {
					super::push(out, 15319);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22569);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27935);
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
		['з', s @ ..] => {
			super::push(out, 7939);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7940);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15320);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22570);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27936);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31222);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 22571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7941);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15321);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22572);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27937);
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
		['у', s @ ..] => {
			super::push(out, 7942);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22573);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7943);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15322);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 7944);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 7945);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оиваешь.
pub(crate) fn n2805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2756);
	s.is_empty()
}

/// Words ending -пиваешь.
pub(crate) fn n2806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2757);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7946);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7947);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15324);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7948);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 7949);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -риваешь.
pub(crate) fn n2807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2758);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7950);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15325);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22576);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31223);
											match s {
												[] => true,
												['а', 'р', 'о', s @ ..] => {
													super::push(out, 32781);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', 'п', s @ ..] => {
											super::push(out, 31224);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32782);
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
				['д', s @ ..] => {
					super::push(out, 15326);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 22577);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27939);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31225);
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
				['ж', s @ ..] => {
					super::push(out, 15327);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22578);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27940);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15328);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 15329);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15330);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15331);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 15332);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 22579);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 15333);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 15334);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22581);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7951);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15335);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22582);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 15336);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', s @ ..] => {
					super::push(out, 15337);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 15338);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15339);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27942);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 27943);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 27944);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 15340);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15341);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 15342);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 15343);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7952);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7953);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 7954);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7955);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7956);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15344);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 15345);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 15346);
					match s {
						[] => true,
						['н', 'а', 'п', 'ш', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27945);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31226);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 27946);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 15347);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 22589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15348);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22590);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 27947);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22591);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27948);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27949);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 22592);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31227);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 31228);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 31229);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 22593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22594);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22595);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 27950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15349);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 15350);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22596);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7957);
			match s {
				[] => true,
				['а', 'м', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22597);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27951);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => super::t10::n23913(s, out),
						['о', 'п', s @ ..] => {
							super::push(out, 22599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t10::n16198(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t10::n8317(s, out),
		['х', s @ ..] => super::t10::n8318(s, out),
		['ы', s @ ..] => super::t10::n8319(s, out),
		['ю', s @ ..] => super::t10::n8320(s, out),
		['я', s @ ..] => super::t10::n8321(s, out),
		_ => false,
	}
}

/// Words ending -хиваешь.
pub(crate) fn n2808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2759);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15362);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22613);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22614);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27965);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											super::push(out, 31235);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', '\u{1}', s @ ..] => {
									super::push(out, 27966);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', s @ ..] => {
							super::push(out, 22615);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 22616);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22618);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 22619);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27967);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31236);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 22620);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 22621);
							s.is_empty()
						}
						['о', 'п', s @ ..] => {
							super::push(out, 22622);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7963);
			match s {
				[] => true,
				['п', 'а', s @ ..] => {
					super::push(out, 15363);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22624);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27969);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31237);
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
		['о', 'п', s @ ..] => {
			super::push(out, 7964);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 15364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7965);
			match s {
				[] => true,
				['а', 'п', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22625);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22626);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22627);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22628);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27970);
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
		['у', s @ ..] => {
			super::push(out, 7966);
			match s {
				[] => true,
				['б', 'д', s @ ..] => {
					super::push(out, 15365);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15366);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27971);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27972);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31238);
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
		['ы', s @ ..] => {
			super::push(out, 7967);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15367);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'о', 'р', 'г', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22631);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27973);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 27974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15368);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22632);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22633);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 22634);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27975);
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
		['ю', s @ ..] => {
			super::push(out, 7968);
			match s {
				[] => true,
				['н', 'о', s @ ..] => {
					super::push(out, 15369);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27976);
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
		['я', s @ ..] => {
			super::push(out, 7969);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22636);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22637);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22638);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22639);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 27977);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31239);
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

/// Words ending -чиваешь.
pub(crate) fn n2809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2760);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7970);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22640);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 22641);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 27978);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31240);
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
				['п', s @ ..] => {
					super::push(out, 15370);
					match s {
						[] => true,
						['о', 'н', 'о', 'к', 'о', s @ ..] => {
							super::push(out, 22642);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 27979);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31241);
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
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22643);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27980);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31242);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 22644);
							match s {
								[] => true,
								['о', 'р', 'ю', 'б', 'о', s @ ..] => {
									super::push(out, 27981);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31243);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7971);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15371);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22646);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22647);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27982);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 27983);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27984);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31244);
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 31245);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 27985);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31246);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 27986);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 22648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15372);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 15373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7972);
			match s {
				[] => true,
				['н', 'н', s @ ..] => {
					super::push(out, 15374);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22649);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 27987);
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
		['л', s @ ..] => {
			super::push(out, 7973);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22650);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 22651);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27988);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31247);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31248);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 31249);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 27989);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 27990);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 27991);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7974);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15376);
					match s {
						[] => true,
						['к', 'а', s @ ..] => {
							super::push(out, 22652);
							match s {
								[] => true,
								['з', 'о', s @ ..] => {
									super::push(out, 27992);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31250);
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
				['и', s @ ..] => {
					super::push(out, 15377);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 22653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7975);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15378);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 15379);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 22654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15380);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 15381);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22655);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27993);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15382);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15383);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15384);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22657);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 27994);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22658);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22659);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 15385);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22660);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27996);
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
		['п', s @ ..] => {
			super::push(out, 7976);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7977);
			s.is_empty()
		}
		['у', s @ ..] => super::t10::n8338(s, out),
		['ю', s @ ..] => {
			super::push(out, 7979);
			s.is_empty()
		}
		['я', 'п', 'с', s @ ..] => {
			super::push(out, 7980);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15392);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шиваешь.
pub(crate) fn n2810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2761);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7981);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15393);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 15394);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 15395);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15396);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15397);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 22668);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 22669);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28002);
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 31251);
											match s {
												[] => true,
												['а', 'р', 'о', s @ ..] => {
													super::push(out, 32783);
													match s {
														[] => true,
														['п', s @ ..] => super::t10::n35340(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 31252);
											match s {
												[] => true,
												['в', 'о', s @ ..] => {
													super::push(out, 32784);
													match s {
														[] => true,
														['п', s @ ..] => super::t10::n35341(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 28003);
									match s {
										[] => true,
										['в', 'о', s @ ..] => {
											super::push(out, 31253);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32785);
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
		['б', s @ ..] => {
			super::push(out, 7982);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7983);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7984);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7985);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15400);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15401);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22670);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28004);
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
		['и', s @ ..] => {
			super::push(out, 7986);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15402);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7987);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15403);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 15404);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15405);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22672);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22674);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22675);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 22676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7988);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15406);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 15407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7989);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7990);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15409);
					match s {
						[] => true,
						['с', 'и', s @ ..] => {
							super::push(out, 22677);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28005);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31254);
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
				['с', s @ ..] => {
					super::push(out, 15410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7991);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15411);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 15412);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щиваешь.
pub(crate) fn n2811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2762);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7992);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15413);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 22678);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28006);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 15414);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 22679);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15415);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 15416);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15417);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22680);
							match s {
								[] => true,
								['т', 'ы', s @ ..] => {
									super::push(out, 28008);
									match s {
										[] => true,
										['в', 'о', s @ ..] => {
											super::push(out, 31255);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32786);
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
		['е', s @ ..] => {
			super::push(out, 7993);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15418);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22681);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28009);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15419);
					match s {
						[] => true,
						['к', 'с', 'и', s @ ..] => {
							super::push(out, 22682);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28010);
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
		['о', s @ ..] => {
			super::push(out, 7994);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15420);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7995);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15422);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22684);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28011);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7996);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 7997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вываешь.
pub(crate) fn n2820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2771);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8022);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8023);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15446);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8024);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 15447);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							super::push(out, 22698);
							match s {
								[] => true,
								['е', 'п', 'о', s @ ..] => {
									super::push(out, 28016);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31258);
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
				['з', s @ ..] => {
					super::push(out, 15448);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28017);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 28018);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15449);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 22699);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 28019);
									match s {
										[] => true,
										['в', 'о', s @ ..] => {
											super::push(out, 31259);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32787);
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
						['п', s @ ..] => {
							super::push(out, 22700);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28020);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31260);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32788);
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
				['о', 'в', 'о', s @ ..] => {
					super::push(out, 15450);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22701);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15451);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22702);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 15452);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22703);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 31261);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 32789);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 31262);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 31263);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 32790);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', 'а', s @ ..] => {
									super::push(out, 28023);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31264);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22704);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 15453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 8025);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8026);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15455);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8027);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15456);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 22706);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28025);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 15457);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22707);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 22708);
							match s {
								[] => true,
								['о', 'т', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31265);
											s.is_empty()
										}
										['е', s @ ..] => super::t10::n33148(s, out),
										['и', s @ ..] => super::t10::n33149(s, out),
										['о', s @ ..] => super::t10::n33150(s, out),
										['с', s @ ..] => super::t10::n33151(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t10::n16315(s, out),
				['з', s @ ..] => super::t10::n16316(s, out),
				['к', s @ ..] => super::t10::n16317(s, out),
				['л', s @ ..] => super::t10::n16318(s, out),
				['н', 'е', s @ ..] => super::t10::n16319(s, out),
				['п', s @ ..] => super::t10::n16320(s, out),
				['р', s @ ..] => super::t10::n16321(s, out),
				['с', s @ ..] => super::t10::n16322(s, out),
				['т', s @ ..] => super::t10::n16323(s, out),
				['х', s @ ..] => super::t10::n16324(s, out),
				['ц', s @ ..] => super::t10::n16325(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t10::n8388(s, out),
		_ => false,
	}
}

/// Words ending -складываешь.
pub(crate) fn n24044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22724);
	match s {
		[] => true,
		['а', 'р', 'о', s @ ..] => {
			super::push(out, 28036);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 31283);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -падываешь.
pub(crate) fn n16329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15471);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 22725);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28037);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едываешь.
pub(crate) fn n8390(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15472);
			match s {
				[] => true,
				['о', 'о', 'д', s @ ..] => {
					super::push(out, 22726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 15473);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28038);
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

/// Words ending -здываешь.
pub(crate) fn n8391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8030);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15474);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22728);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 22729);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28039);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28040);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31284);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15475);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -идываешь.
pub(crate) fn n8392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8031);
	match s {
		[] => true,
		['к', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22730);
					s.is_empty()
				}
				['с', 'о', s @ ..] => {
					super::push(out, 22731);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22732);
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 28043);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31285);
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

/// Words ending -рдываешь.
pub(crate) fn n8393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8032);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 15476);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22733);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыдываешь.
pub(crate) fn n8394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8033);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 15477);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22734);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядываешь.
pub(crate) fn n8395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8034);
	match s {
		[] => true,
		['л', 'г', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22735);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22736);
					match s {
						[] => true,
						['з', 'о', s @ ..] => {
							super::push(out, 28044);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 22737);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22738);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'п', s @ ..] => {
					super::push(out, 22739);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22740);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 28047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15478);
			match s {
				[] => true,
				['п', 'с', s @ ..] => {
					super::push(out, 22741);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28048);
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

/// Words ending -зываешь.
pub(crate) fn n2822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2773);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8035);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15479);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15480);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22742);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22743);
							match s {
								[] => true,
								['д', 'е', s @ ..] => {
									super::push(out, 28049);
									match s {
										[] => true,
										['р', 'п', 'о', s @ ..] => {
											super::push(out, 31287);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32797);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 28050);
									match s {
										[] => true,
										['а', 'р', 'а', s @ ..] => {
											super::push(out, 31288);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32798);
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
				['м', s @ ..] => {
					super::push(out, 15481);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22744);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15482);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22745);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28052);
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
		['б', s @ ..] => {
			super::push(out, 8036);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8037);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 8038);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15484);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8039);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22747);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28053);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 22748);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 28054);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31289);
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
		['и', s @ ..] => {
			super::push(out, 8040);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15485);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15486);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15487);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 8041);
			match s {
				[] => true,
				['а', 'п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22750);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 22751);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 22752);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28055);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'р', s @ ..] => {
							super::push(out, 22753);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28056);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22754);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22755);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28057);
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
		['о', s @ ..] => {
			super::push(out, 8042);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15488);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8043);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8044);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 8045);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 8046);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15491);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 8047);
			match s {
				[] => true,
				['л', 'а', 'к', 'с', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22756);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 22757);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22758);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 28058);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 22759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 8048);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15493);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 15494);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лываешь.
pub(crate) fn n2823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2774);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8049);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15495);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22760);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 22761);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28060);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31290);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28061);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31291);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22762);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22763);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22764);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 28062);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31292);
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
				['м', s @ ..] => {
					super::push(out, 15496);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 15497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8050);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8051);
			match s {
				[] => true,
				['д', 'з', s @ ..] => {
					super::push(out, 15498);
					match s {
						[] => true,
						['а', 'р', 'о', s @ ..] => {
							super::push(out, 22765);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28063);
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
		['п', s @ ..] => {
			super::push(out, 8052);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15499);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 22766);
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							super::push(out, 22767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 15500);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 15501);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15502);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22769);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 22770);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 22771);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28064);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'в', 'и', s @ ..] => {
					super::push(out, 15504);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15505);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 15506);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 15507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8053);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15508);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мываешь.
pub(crate) fn n2824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2775);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8054);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15509);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 15510);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22773);
							match s {
								[] => true,
								['о', 'о', s @ ..] => {
									super::push(out, 28065);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31293);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 22774);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 28066);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31294);
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
				['н', s @ ..] => {
					super::push(out, 15511);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15512);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28067);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31295);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 28068);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31296);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 28069);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28070);
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
		['б', s @ ..] => {
			super::push(out, 8055);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15513);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8056);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 8057);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8058);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15515);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22775);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22776);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28071);
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
		['з', s @ ..] => {
			super::push(out, 8059);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15516);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 15517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8060);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8061);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8062);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15519);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8063);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8064);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22779);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22780);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 22781);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22782);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22783);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 22784);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31297);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32799);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 31298);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22785);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 28073);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 28074);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 28075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 22786);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31299);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31300);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32800);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 31301);
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
		['ы', s @ ..] => {
			super::push(out, 8065);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15522);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нываешь.
pub(crate) fn n2825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2776);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8066);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15523);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15524);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 15525);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15526);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 28076);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28077);
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
		['д', s @ ..] => {
			super::push(out, 8067);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8068);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 8069);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8070);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8071);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8072);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 8073);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пываешь.
pub(crate) fn n2826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2777);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8074);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15531);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22787);
							match s {
								[] => true,
								['з', 'о', s @ ..] => {
									super::push(out, 28078);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31302);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 22788);
							match s {
								[] => true,
								['в', 'а', s @ ..] => {
									super::push(out, 28079);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31303);
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
				['р', s @ ..] => {
					super::push(out, 15532);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22789);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 22790);
							match s {
								[] => true,
								['а', 'з', s @ ..] => {
									super::push(out, 28080);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 22791);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 28082);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28083);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28084);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 15533);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22792);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22793);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28086);
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
		['е', s @ ..] => {
			super::push(out, 8075);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8076);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15534);
					match s {
						[] => true,
						['к', 'с', 'о', 'п', 'а', s @ ..] => {
							super::push(out, 22794);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 22795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 15535);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 22796);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 28087);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31304);
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
		['о', s @ ..] => {
			super::push(out, 8077);
			match s {
				[] => true,
				['л', 'х', 'о', 'д', s @ ..] => {
					super::push(out, 15536);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22798);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28088);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31305);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22799);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28089);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 22800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8078);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 8079);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 8080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рываешь.
pub(crate) fn n2827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2778);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8081);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15537);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22801);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 28090);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 15538);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28091);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31306);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'п', s @ ..] => {
									super::push(out, 28092);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 15539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 15540);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22804);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15541);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15542);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 15543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8082);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15544);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22805);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28094);
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
		['в', s @ ..] => {
			super::push(out, 8083);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 8084);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15545);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28095);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31307);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32801);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 28096);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 28097);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 28098);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31308);
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
				['ы', s @ ..] => {
					super::push(out, 15546);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22806);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22807);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28099);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 28100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8085);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8086);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 8087);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8088);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15549);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8089);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15551);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22810);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28101);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28102);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 22811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15552);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15553);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15554);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28103);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31309);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', 'р', 'п', 'о', s @ ..] => {
									super::push(out, 28104);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31310);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28105);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31311);
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
				['у', s @ ..] => {
					super::push(out, 15555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8090);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8091);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 8092);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15556);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22812);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28106);
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
		['у', s @ ..] => {
			super::push(out, 8093);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 8094);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15557);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22813);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28107);
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

/// Words ending -сываешь.
pub(crate) fn n2828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2779);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8095);
			match s {
				[] => true,
				['р', 'б', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22814);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28108);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31312);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 22815);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 28109);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31313);
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
				['с', s @ ..] => {
					super::push(out, 15558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8096);
			match s {
				[] => true,
				['т', 'в', s @ ..] => {
					super::push(out, 15559);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 15560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8097);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22817);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22818);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31314);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32802);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 31315);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32803);
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
		['с', s @ ..] => {
			super::push(out, 8098);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15561);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15562);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8099);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22821);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22822);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 28110);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31316);
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
		['я', s @ ..] => {
			super::push(out, 8100);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15563);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 28111);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31317);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 28112);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31318);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28113);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28114);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 28115);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31319);
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
				['о', s @ ..] => {
					super::push(out, 15564);
					match s {
						[] => true,
						['п', 'д', s @ ..] => {
							super::push(out, 22823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15565);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тываешь.
pub(crate) fn n2829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2780);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8101);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15566);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28116);
									match s {
										[] => true,
										['з', 'а', s @ ..] => {
											super::push(out, 31320);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32804);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 28117);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28118);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 15567);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15568);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 22824);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 28120);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31321);
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
				['м', 'з', s @ ..] => {
					super::push(out, 15569);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22825);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28121);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 15570);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28122);
									s.is_empty()
								}
								['х', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 31322);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31323);
											s.is_empty()
										}
										['с', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 32805);
													s.is_empty()
												}
												['в', s @ ..] => {
													super::push(out, 32806);
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
				['ч', s @ ..] => {
					super::push(out, 15571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8102);
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					super::push(out, 15572);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22826);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28123);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15573);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 15574);
					match s {
						[] => true,
						['е', 'л', s @ ..] => {
							super::push(out, 22827);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 28124);
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
		['и', s @ ..] => {
			super::push(out, 8103);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15575);
					match s {
						[] => true,
						['к', 'т', s @ ..] => {
							super::push(out, 22828);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28125);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15576);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22829);
							match s {
								[] => true,
								['о', 'в', 'о', s @ ..] => {
									super::push(out, 28126);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31324);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 22830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 15577);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8104);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15578);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 22832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 8105);
			match s {
				[] => true,
				['а', 'б', '\u{1}', s @ ..] => {
					super::push(out, 15579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8106);
			match s {
				[] => true,
				['б', 'а', 'р', '\u{1}', s @ ..] => {
					super::push(out, 15580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 8107);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15581);
					s.is_empty()
				}
				['е', 'ш', 'о', s @ ..] => {
					super::push(out, 15582);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15583);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 22834);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 28127);
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
		['р', s @ ..] => {
			super::push(out, 8108);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8109);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15584);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22835);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28128);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 22836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 15585);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22837);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 15586);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 22838);
							match s {
								[] => true,
								['х', 'у', s @ ..] => {
									super::push(out, 28130);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22839);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28131);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31326);
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
				['и', s @ ..] => super::t10::n16454(s, out),
				['о', s @ ..] => super::t10::n16455(s, out),
				['р', s @ ..] => {
					super::push(out, 15589);
					s.is_empty()
				}
				['у', s @ ..] => super::t10::n16457(s, out),
				['ы', s @ ..] => super::t10::n16458(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8110);
			s.is_empty()
		}
		['х', s @ ..] => super::t10::n8472(s, out),
		['ы', s @ ..] => super::t10::n8473(s, out),
		['я', s @ ..] => {
			super::push(out, 8113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цываешь.
pub(crate) fn n2830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2781);
	match s {
		[] => true,
		['а', 'л', s @ ..] => {
			super::push(out, 8114);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15594);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гуркаешь.
pub(crate) fn n8928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8537);
	s.is_empty()
}

/// Words ending -туркаешь.
pub(crate) fn n8929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8538);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15981);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15982);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 23007);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шуркаешь.
pub(crate) fn n8930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8539);
	s.is_empty()
}

/// Words ending -ыркаешь.
pub(crate) fn n3051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2990);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 8540);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 15983);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28172);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31334);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28173);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31335);
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
				['ш', s @ ..] => {
					super::push(out, 15984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 8541);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 8542);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15985);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15986);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 8543);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23009);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 23010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15987);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15988);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 15989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 8544);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юркаешь.
pub(crate) fn n3052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2991);
	s.is_empty()
}

/// Words ending -ытыкаешь.
pub(crate) fn n9029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8634);
	s.is_empty()
}

/// Words ending -фыкаешь.
pub(crate) fn n3098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3037);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8635);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хыкаешь.
pub(crate) fn n3099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3038);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8636);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цыкаешь.
pub(crate) fn n3100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3039);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8637);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8638);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -листаешь.
pub(crate) fn n9429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9014);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16363);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 16364);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23200);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16365);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ристаешь.
pub(crate) fn n9430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9015);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16366);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 16367);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 16368);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23203);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23204);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28213);
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

/// Words ending -остаешь.
pub(crate) fn n3294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3230);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 9016);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16369);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 9017);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9018);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16371);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 16372);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23206);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 23207);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28214);
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

/// Words ending -рстаешь.
pub(crate) fn n3295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3231);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9019);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23208);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23209);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 23210);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28215);
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

/// Words ending -тстаешь.
pub(crate) fn n3297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3233);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9021);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -устаешь.
pub(crate) fn n3298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3234);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 9022);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 16373);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыстаешь.
pub(crate) fn n3299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3235);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9023);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 9024);
			match s {
				[] => true,
				['х', 'т', s @ ..] => {
					super::push(out, 16374);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юстаешь.
pub(crate) fn n3300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3236);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9025);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 16375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23211);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23212);
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

/// Words ending -ястаешь.
pub(crate) fn n3301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3237);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9026);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9027);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ничаешь.
pub(crate) fn n3390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3326);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9178);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 16481);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9179);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16482);
					match s {
						[] => true,
						['х', 'о', 'п', 'с', s @ ..] => {
							super::push(out, 23254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 16483);
					match s {
						[] => true,
						['л', 'х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28226);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 16484);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16485);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23255);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 9180);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16486);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23256);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 23257);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28228);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 16487);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23258);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 28229);
									s.is_empty()
								}
								['ч', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31343);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 31344);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 23259);
							s.is_empty()
						}
						['у', 'х', 'а', s @ ..] => {
							super::push(out, 23260);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28230);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 23261);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28231);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 28232);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31345);
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
				['и', s @ ..] => {
					super::push(out, 16488);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23262);
							s.is_empty()
						}
						['т', 'к', 'а', 'а', s @ ..] => {
							super::push(out, 23263);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28233);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16489);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 23264);
							match s {
								[] => true,
								['о', 'п', 'с', s @ ..] => {
									super::push(out, 28234);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 31346);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 23265);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28235);
									match s {
										[] => true,
										['д', 'о', s @ ..] => {
											super::push(out, 31347);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32808);
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
				['р', s @ ..] => {
					super::push(out, 16490);
					match s {
						[] => true,
						['е', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28236);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28237);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 28238);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28239);
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
		['д', s @ ..] => {
			super::push(out, 9181);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16491);
					match s {
						[] => true,
						['ж', s @ ..] => super::t10::n24605(s, out),
						['р', s @ ..] => {
							super::push(out, 23267);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28244);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23268);
							match s {
								[] => true,
								['о', s @ ..] => super::t10::n29933(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t10::n17403(s, out),
				['з', s @ ..] => super::t10::n17404(s, out),
				['и', s @ ..] => super::t10::n17405(s, out),
				['л', s @ ..] => super::t10::n17406(s, out),
				['о', s @ ..] => super::t10::n17407(s, out),
				['р', s @ ..] => super::t10::n17408(s, out),
				['у', s @ ..] => super::t10::n17409(s, out),
				['ы', s @ ..] => super::t10::n17410(s, out),
				['ю', s @ ..] => super::t10::n17411(s, out),
				['я', s @ ..] => super::t10::n17412(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t10::n9608(s, out),
		['ж', s @ ..] => super::t10::n9609(s, out),
		['з', s @ ..] => super::t10::n9610(s, out),
		['й', s @ ..] => super::t10::n9611(s, out),
		['м', s @ ..] => super::t10::n9612(s, out),
		['н', s @ ..] => super::t10::n9613(s, out),
		['р', s @ ..] => super::t10::n9614(s, out),
		['с', s @ ..] => super::t10::n9615(s, out),
		['т', s @ ..] => super::t10::n9616(s, out),
		['у', s @ ..] => super::t10::n9617(s, out),
		['ч', s @ ..] => super::t10::n9618(s, out),
		['ш', s @ ..] => super::t10::n9619(s, out),
		['щ', s @ ..] => super::t10::n9620(s, out),
		['ь', s @ ..] => super::t10::n9621(s, out),
		_ => false,
	}
}

/// Words ending -нцовеешь.
pub(crate) fn n9843(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 16717);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16718);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28429);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28430);
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

/// Words ending -ствуешь.
pub(crate) fn n4360(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10078);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17147);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28442);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31494);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'п', s @ ..] => {
									super::push(out, 28443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 17148);
					match s {
						[] => true,
						['у', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28444);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31495);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 31496);
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
		['б', s @ ..] => {
			super::push(out, 10079);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 23615);
							match s {
								[] => true,
								['о', 'м', 'о', s @ ..] => {
									super::push(out, 28445);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31497);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 23616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['л', 'з', 'о', s @ ..] => {
							super::push(out, 23617);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 23618);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28447);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 31498);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23619);
							match s {
								[] => true,
								['о', 'п', 'с', 'о', s @ ..] => {
									super::push(out, 28448);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31499);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 23620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10080);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17149);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 23621);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 23622);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', 'з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31500);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31501);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 32854);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 28450);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17150);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17151);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17152);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23625);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28451);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 23626);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28452);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 23627);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28453);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28454);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 23628);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 23629);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28456);
									match s {
										[] => true,
										['и', 'е', 'н', 'а', s @ ..] => {
											super::push(out, 31502);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 32855);
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
				['у', s @ ..] => {
					super::push(out, 17153);
					match s {
						[] => true,
						['ч', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28457);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 28458);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31503);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 28459);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28460);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31504);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 31505);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 31506);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32856);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 32857);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => super::t10::n30171(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => super::t10::n18099(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t10::n10607(s, out),
		['д', s @ ..] => super::t10::n10608(s, out),
		['е', s @ ..] => super::t10::n10609(s, out),
		['и', s @ ..] => super::t10::n10610(s, out),
		['й', s @ ..] => super::t10::n10611(s, out),
		['м', s @ ..] => super::t10::n10612(s, out),
		['н', s @ ..] => super::t10::n10613(s, out),
		['п', s @ ..] => super::t10::n10614(s, out),
		['р', s @ ..] => super::t10::n10615(s, out),
		['т', s @ ..] => super::t10::n10616(s, out),
		['ф', s @ ..] => super::t10::n10617(s, out),
		['ь', s @ ..] => super::t10::n10618(s, out),
		_ => false,
	}
}

/// Words ending -ивируешь.
pub(crate) fn n10905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10339);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 17411);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23917);
					s.is_empty()
				}
				['о', 'м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28714);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 28715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 23918);
					match s {
						[] => true,
						['л', 'у', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31682);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31683);
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
		['х', s @ ..] => {
			super::push(out, 17412);
			match s {
				[] => true,
				['р', 'а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28716);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28717);
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

/// Words ending -рвируешь.
pub(crate) fn n10907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10341);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 23919);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31684);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31685);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32947);
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
				['н', s @ ..] => {
					super::push(out, 23920);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28718);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 28719);
							match s {
								[] => true,
								['о', 'к', 'а', s @ ..] => {
									super::push(out, 31686);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32948);
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

/// Words ending -ьвируешь.
pub(crate) fn n10908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10342);
	s.is_empty()
}

/// Words ending -гируешь.
pub(crate) fn n4591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4428);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10343);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17414);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23921);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28720);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28721);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28722);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17415);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10344);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10345);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 10346);
			match s {
				[] => true,
				['е', 'л', 'е', 'п', 'а', s @ ..] => {
					super::push(out, 17417);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10347);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10348);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 10349);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17419);
					match s {
						[] => true,
						['а', 'т', 'с', 'о', 'н', 'о', s @ ..] => {
							super::push(out, 23924);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28723);
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
		['ю', s @ ..] => {
			super::push(out, 10350);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дируешь.
pub(crate) fn n4592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4429);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10351);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17420);
					match s {
						[] => true,
						['и', 'р', 'р', 'а', 'б', 'а', s @ ..] => {
							super::push(out, 23925);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28724);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17421);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17422);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23927);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28725);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', 'е', s @ ..] => {
							super::push(out, 23928);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 28726);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 23929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10352);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17423);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 17424);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17425);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 23930);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23931);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28727);
									match s {
										[] => true,
										['н', 'о', 'к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32949);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 32950);
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
				['м', s @ ..] => {
					super::push(out, 17426);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17427);
					match s {
						[] => true,
						['б', 'у', 'с', 'о', s @ ..] => {
							super::push(out, 23932);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28728);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10353);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23933);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 23934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10354);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17429);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 23935);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28729);
									match s {
										[] => true,
										['п', 'о', 'р', 'п', 'с', s @ ..] => {
											super::push(out, 31687);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 32951);
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
						['к', s @ ..] => {
							super::push(out, 23936);
							match s {
								[] => true,
								['с', 'о', s @ ..] => {
									super::push(out, 28730);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31688);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 23937);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28731);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											super::push(out, 31689);
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
				['е', s @ ..] => {
					super::push(out, 17430);
					match s {
						[] => true,
						['г', 'е', 'л', 'а', s @ ..] => {
							super::push(out, 23938);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 23939);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 23940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17431);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23941);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28732);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 23942);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28733);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 23943);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 28734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 23944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17432);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23945);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 23946);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 23947);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 28737);
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
		['о', s @ ..] => {
			super::push(out, 10355);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17433);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23948);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17434);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 23950);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31690);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31691);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31692);
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
				['н', s @ ..] => {
					super::push(out, 17435);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17436);
					match s {
						[] => true,
						['а', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28738);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 28739);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 28740);
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
		['р', s @ ..] => {
			super::push(out, 10356);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17437);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23951);
							s.is_empty()
						}
						['ш', s @ ..] => super::t10::n25341(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t10::n10923(s, out),
		['ю', s @ ..] => super::t10::n10924(s, out),
		_ => false,
	}
}

/// Words ending -жируешь.
pub(crate) fn n4593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4430);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10359);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10360);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17441);
					match s {
						[] => true,
						['н', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28743);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17442);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17443);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17444);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17445);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23954);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 23955);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28745);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 28746);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31693);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 28747);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 23956);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 23957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17446);
					match s {
						[] => true,
						['с', 'а', 'м', 'о', 'п', s @ ..] => {
							super::push(out, 23958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17447);
					match s {
						[] => true,
						['н', 'а', 'ш', 'о', s @ ..] => {
							super::push(out, 23959);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28748);
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
		['д', s @ ..] => {
			super::push(out, 10361);
			match s {
				[] => true,
				['е', 'х', s @ ..] => {
					super::push(out, 17448);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23960);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28749);
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
		['е', s @ ..] => {
			super::push(out, 10362);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10363);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17449);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'и', 'д', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23962);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28750);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 28751);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 23963);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17450);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 23964);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10364);
			match s {
				[] => true,
				['а', 'р', 'а', 'е', s @ ..] => {
					super::push(out, 17451);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10365);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10366);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 10367);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10368);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 10369);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17454);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зируешь.
pub(crate) fn n4594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4431);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10370);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23966);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23967);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28753);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 17455);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17456);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 23968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17457);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23969);
							match s {
								[] => true,
								['а', 'ф', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 31694);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31695);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31696);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23970);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'т', 'н', 'и', 'с', 'е', s @ ..] => {
			super::push(out, 10371);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10372);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17459);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 17460);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 23971);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28754);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 23972);
							s.is_empty()
						}
						['о', 'р', s @ ..] => {
							match s {
								[] => true,
								['п', 'м', s @ ..] => {
									match s {
										[] => true,
										['и', 'а', s @ ..] => {
											super::push(out, 31697);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32952);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 31698);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 32953);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['я', 'е', s @ ..] => {
									super::push(out, 28755);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31699);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 17461);
					match s {
						[] => true,
						['о', 'л', 'о', 'е', s @ ..] => {
							super::push(out, 23974);
							match s {
								[] => true,
								['д', 'и', 'а', s @ ..] => {
									super::push(out, 28756);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31700);
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
				['л', s @ ..] => {
					super::push(out, 17462);
					match s {
						[] => true,
						['а', s @ ..] => super::t10::n25367(s, out),
						['и', 'б', 'а', s @ ..] => {
							super::push(out, 23975);
							match s {
								[] => true,
								['т', 'с', 'а', s @ ..] => {
									super::push(out, 28762);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31708);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => super::t10::n25369(s, out),
						['о', s @ ..] => super::t10::n25370(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t10::n18445(s, out),
				['н', s @ ..] => super::t10::n18446(s, out),
				['р', s @ ..] => super::t10::n18447(s, out),
				['т', s @ ..] => super::t10::n18448(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t10::n10939(s, out),
		['о', s @ ..] => super::t10::n10940(s, out),
		['ю', s @ ..] => super::t10::n10941(s, out),
		_ => false,
	}
}

/// Words ending -иируешь.
pub(crate) fn n4595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4432);
	match s {
		[] => true,
		['в', 'е', 'р', 'б', 'б', 'а', 'о', s @ ..] => {
			super::push(out, 10376);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', 'а', 'л', 'п', 'с', s @ ..] => {
			super::push(out, 10377);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10378);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 10379);
			match s {
				[] => true,
				['п', 'о', 'р', 'п', 'а', s @ ..] => {
					super::push(out, 17477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', 'и', 'н', 'ы', s @ ..] => {
			super::push(out, 10380);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17478);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

