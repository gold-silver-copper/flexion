use flexion::Path;

/// Words ending -цевый.
pub(crate) fn n980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 955);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2453);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 2454);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6229);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12667);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 12668);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19947);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 12669);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 19948);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 25908);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12670);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 12671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6230);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 6231);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2455);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2456);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чевый.
pub(crate) fn n981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 956);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2457);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2458);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6232);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'а', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6233);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6234);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2459);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2460);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12676);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 19949);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', 'е', s @ ..] => {
							super::push(out, 12677);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19950);
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 25909);
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
				['и', 'р', s @ ..] => {
					super::push(out, 6236);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шевый.
pub(crate) fn n982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 957);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2461);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6238);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2462);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2463);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					super::push(out, 6240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2464);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2465);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2466);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2467);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 2468);
			match s {
				[] => true,
				['л', 'п', '\u{1}', s @ ..] => {
					super::push(out, 6244);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щевый.
pub(crate) fn n983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 958);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2469);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6245);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьевый.
pub(crate) fn n984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 959);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 2470);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6246);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2471);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6248);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рыстолюбивый.
pub(crate) fn n31140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29678);
	s.is_empty()
}

/// Words ending -ливый.
pub(crate) fn n990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 965);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2480);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 6257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2481);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6258);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2482);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2483);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2484);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6262);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19955);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6263);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12689);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 19957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 12690);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12691);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 12692);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12693);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 12694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 6264);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6265);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12696);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6266);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12698);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 12699);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19959);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 19960);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 12700);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 19961);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12701);
							match s {
								[] => true,
								['у', '\u{1}', s @ ..] => {
									super::push(out, 19962);
									s.is_empty()
								}
								['ю', s @ ..] => {
									super::push(out, 19963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6267);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12702);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 19964);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 12703);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 12704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6268);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 12705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2485);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2486);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12706);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 19965);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12707);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 19966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6270);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6271);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2487);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6272);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12709);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 12710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6273);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6274);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 12711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 6275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2488);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2489);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2490);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2491);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 6277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2492);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2493);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6278);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2494);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2495);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6280);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12714);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 12715);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 12716);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19970);
									match s {
										[] => true,
										['р', 'п', 'е', s @ ..] => {
											super::push(out, 25915);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 29679);
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
							super::push(out, 12717);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 19971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6281);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6282);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2496);
			match s {
				[] => true,
				['а', s @ ..] => super::t6::n6554(s, out),
				['о', s @ ..] => {
					super::push(out, 6284);
					s.is_empty()
				}
				['у', s @ ..] => super::t6::n6556(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t6::n2592(s, out),
		['х', s @ ..] => {
			super::push(out, 2498);
			s.is_empty()
		}
		['ч', s @ ..] => super::t6::n2594(s, out),
		['ш', s @ ..] => super::t6::n2595(s, out),
		_ => false,
	}
}

/// Words ending -злочестивый.
pub(crate) fn n31147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29685);
	s.is_empty()
}

/// Words ending -лостивый.
pub(crate) fn n13388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12762);
	s.is_empty()
}

/// Words ending -ьстивый.
pub(crate) fn n6589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6315);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 12763);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чтивый.
pub(crate) fn n2607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2511);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 6316);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чивый.
pub(crate) fn n995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 970);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2512);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 2513);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 2514);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6317);
					match s {
						[] => true,
						['ж', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 20006);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 25937);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 20007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2515);
			match s {
				[] => true,
				['о', 'х', 'т', s @ ..] => {
					super::push(out, 6319);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2516);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6320);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 20008);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 25938);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 20009);
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
			super::push(out, 2517);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 2518);
			match s {
				[] => true,
				['о', 'т', 'с', 'у', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12765);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12766);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 20010);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 20011);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 25939);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', 'п', s @ ..] => {
									super::push(out, 20012);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 25940);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 20013);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 25941);
											s.is_empty()
										}
										['б', s @ ..] => {
											super::push(out, 25942);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 25943);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 20014);
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
			super::push(out, 2519);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6321);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6322);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 12767);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 20015);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2520);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					super::push(out, 6324);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2521);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6326);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 25944);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 25945);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 29686);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 20016);
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
			super::push(out, 2522);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2523);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 2524);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2525);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6327);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2526);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шивый.
pub(crate) fn n996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 971);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2527);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2528);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2529);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 2530);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word фаговый.
pub(crate) fn n13404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12775);
	s.is_empty()
}

/// Words ending -чаговый.
pub(crate) fn n6624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6349);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 12776);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шаговый.
pub(crate) fn n6625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6350);
	s.is_empty()
}

/// Words ending -еговый.
pub(crate) fn n2641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2545);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6351);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 6352);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иговый.
pub(crate) fn n2642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2546);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 6353);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6354);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 6355);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12777);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6356);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6357);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 6358);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12778);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лговый.
pub(crate) fn n2643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2547);
	s.is_empty()
}

/// Words ending -нговый.
pub(crate) fn n2644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2548);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6359);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12779);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12780);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20020);
							match s {
								[] => true,
								['н', 'д', 'о', '\u{1}', s @ ..] => {
									super::push(out, 25946);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6360);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12782);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6361);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12784);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 20021);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 25947);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 12785);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 12786);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20023);
							s.is_empty()
						}
						['н', 'е', s @ ..] => {
							super::push(out, 20024);
							match s {
								[] => true,
								['р', 'б', 'о', s @ ..] => {
									super::push(out, 25948);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 29687);
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
					super::push(out, 12787);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 12788);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12789);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 20025);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25949);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 29688);
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
					super::push(out, 12790);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20026);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 20027);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 20028);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25951);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12791);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 12792);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12793);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20029);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 25952);
									match s {
										[] => true,
										['у', 'ш', '\u{1}', s @ ..] => {
											super::push(out, 29689);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 25953);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 25954);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 20030);
							match s {
								[] => true,
								['л', 'к', '-', 'о', 'н', 'т', 'е', 'ч', 'с', 'а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 25955);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'т', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 25956);
									s.is_empty()
								}
								['к', 'о', s @ ..] => {
									super::push(out, 25957);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29690);
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
					super::push(out, 12794);
					match s {
						[] => true,
						['р', 'о', s @ ..] => {
							super::push(out, 20031);
							match s {
								[] => true,
								['с', 'т', s @ ..] => {
									super::push(out, 25958);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 29691);
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
				['т', s @ ..] => {
					super::push(out, 12795);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20032);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 25959);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29692);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 20033);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 25960);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 20034);
							match s {
								[] => true,
								['е', 'р', '\u{1}', s @ ..] => {
									super::push(out, 25961);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 20035);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20036);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25962);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 20037);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25963);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25964);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 29693);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 20038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 12796);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20039);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 25965);
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
			super::push(out, 6362);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12797);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 20040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12798);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 20041);
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

/// Words ending -оговый.
pub(crate) fn n2645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2549);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 6363);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12799);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6364);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20042);
							match s {
								[] => true,
								['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 25966);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 20043);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25967);
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
			super::push(out, 6365);
			match s {
				[] => true,
				['и', 'м', '\u{1}', s @ ..] => {
					super::push(out, 12800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6366);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12801);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12802);
					s.is_empty()
				}
				['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 12803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6367);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рговый.
pub(crate) fn n2646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2550);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6368);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12804);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 12805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6369);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 12806);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12807);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20047);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25968);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29694);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20048);
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

/// Words ending -уговый.
pub(crate) fn n2647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2551);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 6370);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьговый.
pub(crate) fn n2648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2552);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6371);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12808);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яговый.
pub(crate) fn n2649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2553);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 6372);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -довый.
pub(crate) fn n1001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 976);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2554);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6373);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6374);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6375);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6376);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2555);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 2556);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2557);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6377);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12810);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6379);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 12812);
							match s {
								[] => true,
								['о', 'н', 'м', 'е', 'т', '\u{1}', s @ ..] => {
									super::push(out, 20049);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6380);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 20050);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 20051);
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
			super::push(out, 2558);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6381);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2559);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6382);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12814);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 20052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2560);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2561);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6385);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6386);
					match s {
						[] => true,
						['р', 'б', s @ ..] => {
							super::push(out, 12816);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20053);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 25969);
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
					super::push(out, 6387);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6388);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12817);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 12818);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 12819);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 12820);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 20055);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 25970);
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
					super::push(out, 6389);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12821);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 20056);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 25971);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 29695);
													s.is_empty()
												}
												_ => false,
											}
										}
										['х', s @ ..] => {
											super::push(out, 25972);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 29696);
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
							super::push(out, 12822);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20057);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 20058);
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
			super::push(out, 2562);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 6390);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6391);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6392);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12823);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 20059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 12824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6393);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12825);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6394);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6395);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 6396);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12827);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12828);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 20060);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 12829);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 12830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2563);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6397);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12831);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 12832);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6398);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12833);
							match s {
								[] => true,
								['-', 'о', 'н', 'м', 'е', 'т', '\u{1}', s @ ..] => {
									super::push(out, 20062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 12834);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20063);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 25973);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 12835);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 12836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t6::n2660(s, out),
		['ь', s @ ..] => {
			super::push(out, 2565);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 2566);
			s.is_empty()
		}
		['я', s @ ..] => super::t6::n2663(s, out),
		_ => false,
	}
}

/// Words ending -жовый.
pub(crate) fn n1002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 977);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2568);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2569);
			s.is_empty()
		}
		['е', '\u{1}', s @ ..] => {
			super::push(out, 2570);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2571);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2572);
			s.is_empty()
		}
		['н', 'и', 'р', 'к', '\u{1}', s @ ..] => {
			super::push(out, 2573);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2574);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6402);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зовый.
pub(crate) fn n1003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 978);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2575);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6403);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 6404);
					s.is_empty()
				}
				['к', '\u{1}', s @ ..] => {
					super::push(out, 6405);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6406);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12845);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12846);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12847);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 20068);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 25976);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', 'д', 'о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 25977);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 25978);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', 'х', s @ ..] => {
							super::push(out, 12848);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 20069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 12849);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 25979);
											s.is_empty()
										}
										['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 25980);
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
				['ф', '\u{1}', s @ ..] => {
					super::push(out, 6407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2576);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6408);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6409);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2577);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6411);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 6412);
					match s {
						[] => true,
						['с', 'б', 'а', '\u{1}', s @ ..] => {
							super::push(out, 12852);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2578);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2579);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 2580);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6413);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6414);
					match s {
						[] => true,
						['р', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20070);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 20071);
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
			super::push(out, 2581);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12853);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 12854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2582);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2583);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 2584);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 2585);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6415);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6416);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6417);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12855);
							match s {
								[] => true,
								['б', '-', s @ ..] => {
									super::push(out, 20072);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 25981);
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

/// Words ending -ковый.
pub(crate) fn n1004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 979);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2586);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6418);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6419);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 12857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12858);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12859);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 12860);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 12861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6421);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 12862);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 20073);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6422);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12863);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20074);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 20075);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 25982);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 20076);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 25983);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 12864);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 20077);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6423);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12865);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 12866);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20078);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12867);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 12868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12869);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6425);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 12871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 6426);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2587);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6427);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6428);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12874);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12875);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 20080);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 25984);
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
					super::push(out, 6429);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2588);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 12877);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 20081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 12878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6430);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12879);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 20082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6431);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2589);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6432);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20083);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 20084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6433);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6434);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 6435);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12883);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									super::push(out, 20085);
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
			super::push(out, 2590);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12884);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6437);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6438);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12886);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20086);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 6439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2591);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6440);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6441);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12888);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 12889);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 20087);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 20088);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 25985);
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
		['з', s @ ..] => {
			super::push(out, 2592);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6442);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t6::n2689(s, out),
		['й', s @ ..] => super::t6::n2690(s, out),
		['к', s @ ..] => super::t6::n2691(s, out),
		['л', s @ ..] => super::t6::n2692(s, out),
		['м', s @ ..] => super::t6::n2693(s, out),
		['н', s @ ..] => super::t6::n2694(s, out),
		['о', s @ ..] => super::t6::n2695(s, out),
		['п', s @ ..] => super::t6::n2696(s, out),
		['р', s @ ..] => super::t6::n2697(s, out),
		['с', s @ ..] => super::t6::n2698(s, out),
		['т', s @ ..] => super::t6::n2699(s, out),
		['у', s @ ..] => super::t6::n2700(s, out),
		['ч', s @ ..] => super::t6::n2701(s, out),
		['ш', s @ ..] => super::t6::n2702(s, out),
		['ы', s @ ..] => super::t6::n2703(s, out),
		['ь', s @ ..] => super::t6::n2704(s, out),
		['э', s @ ..] => {
			super::push(out, 2609);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 2610);
			s.is_empty()
		}
		['я', s @ ..] => super::t6::n2707(s, out),
		_ => false,
	}
}

/// Words ending -ловый.
pub(crate) fn n1005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 980);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2612);
			match s {
				[] => true,
				['и', 'в', s @ ..] => {
					super::push(out, 6525);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'о', '\u{1}', s @ ..] => {
					super::push(out, 6526);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13030);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 13031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2613);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6527);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6528);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2614);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 6529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2615);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6530);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 6531);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2616);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2617);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6532);
					match s {
						[] => true,
						['о', 'к', 'о', 'р', 'к', '\u{1}', s @ ..] => {
							super::push(out, 13034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6533);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 13035);
							match s {
								[] => true,
								['о', 'н', 'м', 'е', 'т', '\u{1}', s @ ..] => {
									super::push(out, 20211);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 13036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6534);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13037);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 20212);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 20213);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 26038);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 13038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6535);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6536);
					match s {
						[] => true,
						['э', 'о', s @ ..] => {
							super::push(out, 13039);
							match s {
								[] => true,
								['н', 'о', 'т', s @ ..] => {
									super::push(out, 20214);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26039);
											match s {
												[] => true,
												['ц', 'а', '\u{1}', s @ ..] => {
													super::push(out, 29721);
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
		['й', s @ ..] => {
			super::push(out, 2618);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2619);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6537);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2620);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6539);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 13041);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 13042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6540);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 13043);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 20215);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 6541);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 20216);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 26040);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ц', s @ ..] => {
									super::push(out, 20217);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 26041);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 20218);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 20219);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['к', 'п', s @ ..] => {
									super::push(out, 20220);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26042);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', 'г', s @ ..] => {
									super::push(out, 20221);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 26043);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', 'п', s @ ..] => {
									super::push(out, 20222);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 26044);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 20223);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 26045);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'ы', s @ ..] => {
									super::push(out, 20224);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 26046);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 20225);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 13045);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 20226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 13046);
							match s {
								[] => true,
								['е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 20227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6543);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 13048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6544);
					match s {
						[] => true,
						['а', s @ ..] => super::t6::n13688(s, out),
						['и', s @ ..] => super::t6::n13689(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t6::n6828(s, out),
				['с', s @ ..] => super::t6::n6829(s, out),
				['т', s @ ..] => super::t6::n6830(s, out),
				['э', s @ ..] => super::t6::n6831(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t6::n2717(s, out),
		['с', s @ ..] => super::t6::n2718(s, out),
		['т', s @ ..] => super::t6::n2719(s, out),
		['у', s @ ..] => super::t6::n2720(s, out),
		['э', s @ ..] => super::t6::n2721(s, out),
		['я', s @ ..] => super::t6::n2722(s, out),
		_ => false,
	}
}

/// Words ending -мовый.
pub(crate) fn n1006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 981);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2627);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6556);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13061);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 20234);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 13062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2628);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6558);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 13063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2629);
			match s {
				[] => true,
				['д', 'о', 'е', 'н', '\u{1}', s @ ..] => {
					super::push(out, 6559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2630);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6560);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13064);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 13065);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 20235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6561);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 6562);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13066);
							match s {
								[] => true,
								['х', 'е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 26048);
											s.is_empty()
										}
										['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 26049);
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
			super::push(out, 2631);
			match s {
				[] => true,
				['а', 'р', 'г', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 13067);
							s.is_empty()
						}
						['о', 'л', 'и', 'к', 'х', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 20236);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 26050);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 20237);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 26051);
											match s {
												[] => true,
												['е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 29722);
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
						['т', s @ ..] => {
							super::push(out, 13068);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20238);
									match s {
										[] => true,
										['с', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
											super::push(out, 26052);
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
					super::push(out, 6563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2632);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6564);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13070);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20239);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 13071);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20240);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2633);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2634);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6566);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6567);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2635);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 2636);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6569);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6570);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13074);
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

/// Words ending -новый.
pub(crate) fn n1007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 982);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2637);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2638);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6571);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13075);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 20241);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', 'а', 'л', 'к', 'а', 'б', '\u{1}', s @ ..] => {
					super::push(out, 6572);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13076);
							s.is_empty()
						}
						['р', 'у', s @ ..] => {
							super::push(out, 13077);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 20242);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13078);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 13079);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 20243);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6573);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13080);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13081);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 20244);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 20245);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 20246);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 26053);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ж', s @ ..] => {
									super::push(out, 20247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20248);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 20249);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 20250);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 20251);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 26054);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 29723);
													s.is_empty()
												}
												_ => false,
											}
										}
										['г', s @ ..] => {
											super::push(out, 26055);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 26056);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 26057);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 29724);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 20252);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 20253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6574);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13082);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13083);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20254);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6575);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', 'с', s @ ..] => {
							super::push(out, 13084);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13085);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 20256);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 13086);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 13087);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 20257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6576);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13088);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 20258);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 13089);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13090);
							s.is_empty()
						}
						['к', 'о', '\u{1}', s @ ..] => {
							super::push(out, 13091);
							s.is_empty()
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 20259);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 20260);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 26058);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'о', s @ ..] => {
							super::push(out, 13092);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 20261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', 'а', 'к', '-', s @ ..] => {
							super::push(out, 13093);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20262);
									match s {
										[] => true,
										['н', 'м', 'е', 'т', '\u{1}', s @ ..] => {
											super::push(out, 26059);
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
					super::push(out, 6577);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 13094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 6578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2639);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 6579);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t6::n2736(s, out),
		['и', s @ ..] => super::t6::n2737(s, out),
		['й', s @ ..] => {
			super::push(out, 2642);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2643);
			match s {
				[] => true,
				['о', s @ ..] => super::t6::n6895(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2644);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6605);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t6::n2741(s, out),
		['р', s @ ..] => super::t6::n2742(s, out),
		['с', s @ ..] => super::t6::n2743(s, out),
		['х', s @ ..] => super::t6::n2744(s, out),
		['ь', s @ ..] => super::t6::n2745(s, out),
		['я', s @ ..] => {
			super::push(out, 2650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -повый.
pub(crate) fn n1008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 983);
	match s {
		[] => true,
		['а', 'р', 'к', '\u{1}', s @ ..] => {
			super::push(out, 2651);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2652);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2653);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6621);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6622);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13152);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20299);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 6623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2654);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6624);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 13154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6625);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2655);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2656);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6627);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13155);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 13156);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 20300);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6628);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 13157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 6629);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2657);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ровый.
pub(crate) fn n1009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 984);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2658);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6630);
					match s {
						[] => true,
						['а', 'г', s @ ..] => {
							super::push(out, 13159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6631);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6632);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13160);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20301);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 13161);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 20302);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 13162);
							match s {
								[] => true,
								['о', 'д', 'и', s @ ..] => {
									super::push(out, 20303);
									match s {
										[] => true,
										['т', 'я', 'с', s @ ..] => {
											super::push(out, 26078);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 29731);
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
					super::push(out, 6633);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 6634);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6635);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6636);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13163);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 13164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 6637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2659);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6638);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6639);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 13165);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 20304);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6640);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6641);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6642);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13168);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 20305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6643);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13169);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 13170);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 13171);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20306);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 20307);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 13172);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 20308);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26079);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 20309);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 20310);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13173);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 20311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6644);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 6645);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2660);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6646);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13176);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13177);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 13178);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20313);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13179);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 20314);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 26080);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 20315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6647);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6648);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2661);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6649);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6650);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 13183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6651);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2662);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6652);
					match s {
						[] => true,
						['к', 'о', s @ ..] => {
							super::push(out, 13185);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 20316);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 26081);
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
					super::push(out, 6653);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13186);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 13187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6654);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6655);
					match s {
						[] => true,
						['а', s @ ..] => super::t6::n13837(s, out),
						['и', s @ ..] => super::t6::n13838(s, out),
						['у', s @ ..] => super::t6::n13839(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t6::n2759(s, out),
		['и', s @ ..] => super::t6::n2760(s, out),
		['й', s @ ..] => super::t6::n2761(s, out),
		['к', s @ ..] => super::t6::n2762(s, out),
		['н', s @ ..] => super::t6::n2763(s, out),
		['о', s @ ..] => super::t6::n2764(s, out),
		['т', s @ ..] => super::t6::n2765(s, out),
		['у', s @ ..] => super::t6::n2766(s, out),
		['х', s @ ..] => super::t6::n2767(s, out),
		['ю', s @ ..] => super::t6::n2768(s, out),
		['я', s @ ..] => super::t6::n2769(s, out),
		_ => false,
	}
}

/// Words ending -совый.
pub(crate) fn n1010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 985);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2674);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6695);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13222);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 20340);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 20341);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26098);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 13223);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2675);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6697);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6698);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13225);
							match s {
								[] => true,
								['к', 'р', 'е', 'г', '\u{1}', s @ ..] => {
									super::push(out, 20343);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6699);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6700);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6701);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13228);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 20344);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 13229);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 20345);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2676);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6703);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 6704);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6705);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6706);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13231);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13232);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20346);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6707);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 6708);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2677);
			match s {
				[] => true,
				['е', 'р', 'д', s @ ..] => {
					super::push(out, 6709);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2678);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6710);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6711);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13236);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20347);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6712);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 13238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 6713);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2679);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2680);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6715);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13240);
							match s {
								[] => true,
								['а', 'з', s @ ..] => {
									super::push(out, 20348);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26099);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 13241);
							s.is_empty()
						}
						['н', 'и', 'ф', 'е', s @ ..] => {
							super::push(out, 13242);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2681);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6717);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13243);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13244);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 20350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6718);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6719);
					match s {
						[] => true,
						['и', 'р', 'е', 'м', '\u{1}', s @ ..] => {
							super::push(out, 13246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6720);
					match s {
						[] => true,
						['б', 'т', s @ ..] => {
							super::push(out, 13247);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20351);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 13248);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20352);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6721);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13250);
							match s {
								[] => true,
								['л', '\u{1}', s @ ..] => {
									super::push(out, 20353);
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
			super::push(out, 2682);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6722);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6723);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6724);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6725);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13253);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20354);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 20355);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 20356);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t6::n13903(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t6::n2779(s, out),
		['с', s @ ..] => super::t6::n2780(s, out),
		['у', s @ ..] => super::t6::n2781(s, out),
		['ь', s @ ..] => super::t6::n2782(s, out),
		['ю', s @ ..] => {
			super::push(out, 2687);
			s.is_empty()
		}
		['я', s @ ..] => super::t6::n2784(s, out),
		_ => false,
	}
}

/// Words ending -товый.
pub(crate) fn n1011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 986);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2689);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6738);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6739);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 13269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'а', 'с', '\u{1}', s @ ..] => {
					super::push(out, 6740);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6741);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6742);
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					super::push(out, 6743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2690);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6744);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2691);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6745);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13272);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 20365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6746);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 6747);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6748);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 13273);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 13274);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 20366);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6749);
					match s {
						[] => true,
						['о', 'и', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20367);
									s.is_empty()
								}
								['-', 'о', 'н', s @ ..] => {
									super::push(out, 20368);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 26102);
											match s {
												[] => true,
												['е', 'т', '\u{1}', s @ ..] => {
													super::push(out, 29739);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['а', s @ ..] => {
									super::push(out, 20369);
									s.is_empty()
								}
								['о', 'н', s @ ..] => {
									super::push(out, 20370);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 26103);
											match s {
												[] => true,
												['е', 'т', '\u{1}', s @ ..] => {
													super::push(out, 29740);
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
				['н', s @ ..] => {
					super::push(out, 6750);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 13275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2692);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6751);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 13276);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 20371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 13277);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 20372);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13278);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 20373);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26104);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 20374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6752);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6753);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 6754);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6755);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13279);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6756);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13281);
							s.is_empty()
						}
						['е', 'п', s @ ..] => {
							super::push(out, 13282);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13283);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13284);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 20375);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 20376);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 20377);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 20378);
									s.is_empty()
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => super::t6::n27397(s, out),
										['е', s @ ..] => {
											super::push(out, 26105);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => super::t6::n21409(s, out),
								['т', s @ ..] => super::t6::n21410(s, out),
								_ => false,
							}
						}
						['у', s @ ..] => super::t6::n13937(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t6::n7051(s, out),
				['н', s @ ..] => super::t6::n7052(s, out),
				['р', s @ ..] => super::t6::n7053(s, out),
				['с', s @ ..] => super::t6::n7054(s, out),
				['т', s @ ..] => super::t6::n7055(s, out),
				['ф', s @ ..] => super::t6::n7056(s, out),
				['х', s @ ..] => super::t6::n7057(s, out),
				['ц', 'ь', s @ ..] => super::t6::n7058(s, out),
				['ш', s @ ..] => super::t6::n7059(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t6::n2789(s, out),
		['к', s @ ..] => super::t6::n2790(s, out),
		['л', s @ ..] => super::t6::n2791(s, out),
		['н', s @ ..] => super::t6::n2792(s, out),
		['о', s @ ..] => super::t6::n2793(s, out),
		['п', s @ ..] => super::t6::n2794(s, out),
		['р', s @ ..] => super::t6::n2795(s, out),
		['с', s @ ..] => super::t6::n2796(s, out),
		['у', s @ ..] => super::t6::n2797(s, out),
		['ф', s @ ..] => super::t6::n2798(s, out),
		['х', s @ ..] => super::t6::n2799(s, out),
		['ч', s @ ..] => super::t6::n2800(s, out),
		['ь', s @ ..] => super::t6::n2801(s, out),
		_ => false,
	}
}

/// Words ending -фовый.
pub(crate) fn n1012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 987);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2706);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2707);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6808);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 13359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2708);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 2709);
			match s {
				[] => true,
				['л', 'о', 'г', '\u{1}', s @ ..] => {
					super::push(out, 6809);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховый.
pub(crate) fn n1013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 988);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2710);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6810);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2711);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6812);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13360);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 20437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13361);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20438);
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
			super::push(out, 2712);
			match s {
				[] => true,
				['п', 'е', 'л', 'б', 'о', '\u{1}', s @ ..] => {
					super::push(out, 6813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2713);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2714);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2715);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2716);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 6816);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6817);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6818);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2717);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 2718);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 2719);
			match s {
				[] => true,
				['з', 'е', 'б', '\u{1}', s @ ..] => {
					super::push(out, 6821);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цовый.
pub(crate) fn n1014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 989);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6822);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2720);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6824);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6825);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 6826);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', 'б', s @ ..] => {
					super::push(out, 6828);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 13366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2721);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6830);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 13367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 6831);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2722);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2723);
			match s {
				[] => true,
				['и', 'в', 'с', '\u{1}', s @ ..] => {
					super::push(out, 6833);
					s.is_empty()
				}
				['у', 'п', s @ ..] => {
					super::push(out, 6834);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2724);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6835);
					s.is_empty()
				}
				['о', 'т', s @ ..] => {
					super::push(out, 6836);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2725);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -човый.
pub(crate) fn n1015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 990);
	s.is_empty()
}

/// Words ending -шовый.
pub(crate) fn n1016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 991);
	s.is_empty()
}

/// Words ending -щовый.
pub(crate) fn n1017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 992);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2726);
			match s {
				[] => true,
				['о', 'х', '\u{1}', s @ ..] => {
					super::push(out, 6837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2727);
			match s {
				[] => true,
				['в', 'х', '\u{1}', s @ ..] => {
					super::push(out, 6838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2728);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word зелено-белый.
pub(crate) fn n31216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29748);
	s.is_empty()
}

/// The word черно-желто-белый.
pub(crate) fn n31217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29749);
	s.is_empty()
}

/// The word заплесневелый.
pub(crate) fn n31218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29750);
	s.is_empty()
}

/// Words ending -енелый.
pub(crate) fn n2908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2804);
	match s {
		[] => true,
		['д', 'е', s @ ..] => {
			super::push(out, 6894);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20465);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6895);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13429);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 13430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6896);
			match s {
				[] => true,
				['а', 'к', 'о', '\u{1}', s @ ..] => {
					super::push(out, 13431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6897);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнелый.
pub(crate) fn n2910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2806);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -снелый.
pub(crate) fn n2911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2807);
	s.is_empty()
}

/// Words ending -янелый.
pub(crate) fn n2912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2808);
	s.is_empty()
}

/// Words ending -пелый.
pub(crate) fn n1072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1042);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2809);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2810);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6901);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6902);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13432);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 13433);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6903);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13434);
							match s {
								[] => true,
								['о', 'к', 'с', '\u{1}', s @ ..] => {
									super::push(out, 20469);
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

/// Words ending -релый.
pub(crate) fn n1073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1043);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2811);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13435);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 13436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2812);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 2813);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6904);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6905);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13437);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 13438);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20470);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2814);
			match s {
				[] => true,
				['ж', 'о', '\u{1}', s @ ..] => {
					super::push(out, 6907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2815);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13439);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 20471);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 26133);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 20472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 13440);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 20473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 13441);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13442);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13443);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 13444);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20474);
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
			super::push(out, 2816);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6908);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2817);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6910);
					s.is_empty()
				}
				['д', 'о', '\u{1}', s @ ..] => {
					super::push(out, 6911);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -селый.
pub(crate) fn n1074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1044);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2818);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13446);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 13447);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20475);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 13448);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 20476);
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
			super::push(out, 2819);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2820);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -телый.
pub(crate) fn n1075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1045);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2821);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2822);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6914);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13449);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13450);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 20477);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 13451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13452);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 20478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 13453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6915);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6916);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13454);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 20479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13455);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 13456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2823);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 6917);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -целый.
pub(crate) fn n1076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1046);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2824);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2825);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6919);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шелый.
pub(crate) fn n1077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1047);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 2826);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					super::push(out, 6920);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лочнокислый.
pub(crate) fn n31219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29751);
	s.is_empty()
}

/// Words ending -аемый.
pub(crate) fn n1121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1090);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2881);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6974);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6975);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6976);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13486);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 20495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6977);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6978);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 6979);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13488);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 20496);
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
		['в', s @ ..] => super::t6::n2988(s, out),
		['г', s @ ..] => {
			super::push(out, 2883);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6985);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 20579);
									match s {
										[] => true,
										['о', 'о', s @ ..] => {
											super::push(out, 26225);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 29783);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', 'а', s @ ..] => {
									super::push(out, 20580);
									match s {
										[] => true,
										['р', 'о', s @ ..] => {
											super::push(out, 26226);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 29784);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 20581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6986);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 20582);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26227);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 29785);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 20583);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6987);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 13519);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 20584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6988);
					match s {
						[] => true,
						['в', 'д', '\u{1}', s @ ..] => {
							super::push(out, 13520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6989);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13521);
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 20585);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6990);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 20586);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 20587);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6991);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 20588);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 20589);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 26228);
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
					super::push(out, 6992);
					match s {
						[] => true,
						['р', 'п', 'с', s @ ..] => {
							super::push(out, 13522);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20590);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 26229);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2884);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6993);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13524);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20591);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 13525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 6994);
					match s {
						[] => true,
						['а', 'л', s @ ..] => {
							super::push(out, 13526);
							match s {
								[] => true,
								['х', 'о', 'е', s @ ..] => {
									super::push(out, 20592);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 26230);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13527);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 13528);
							match s {
								[] => true,
								['е', 'в', 'т', 'о', s @ ..] => {
									super::push(out, 20593);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26231);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 29786);
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
							super::push(out, 13529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'ж', s @ ..] => {
					super::push(out, 6995);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 6996);
					match s {
						[] => true,
						['л', 'б', 'а', 'н', 'е', s @ ..] => {
							super::push(out, 13530);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20594);
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
			super::push(out, 2885);
			match s {
				[] => true,
				['а', s @ ..] => super::t6::n7318(s, out),
				['з', s @ ..] => {
					super::push(out, 6998);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6999);
					s.is_empty()
				}
				['о', 'т', s @ ..] => {
					super::push(out, 7000);
					match s {
						[] => true,
						['ч', 'и', 'н', 'у', 'е', s @ ..] => {
							super::push(out, 13533);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20596);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t6::n7322(s, out),
				['я', s @ ..] => super::t6::n7323(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t6::n2992(s, out),
		['к', s @ ..] => super::t6::n2993(s, out),
		['л', s @ ..] => super::t6::n2994(s, out),
		['м', s @ ..] => super::t6::n2995(s, out),
		['н', s @ ..] => super::t6::n2996(s, out),
		['п', s @ ..] => super::t6::n2997(s, out),
		['р', s @ ..] => super::t6::n2998(s, out),
		['с', s @ ..] => super::t6::n2999(s, out),
		['т', s @ ..] => super::t6::n3000(s, out),
		['х', s @ ..] => super::t6::n3001(s, out),
		['ц', s @ ..] => super::t7::n3002(s, out),
		['ч', s @ ..] => super::t7::n3003(s, out),
		['ш', s @ ..] => super::t7::n3004(s, out),
		['щ', s @ ..] => super::t7::n3005(s, out),
		_ => false,
	}
}

/// Words ending -вуемый.
pub(crate) fn n3011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2904);
	match s {
		[] => true,
		['т', 'с', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13592);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 13593);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20625);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 20626);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 20627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13594);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 13595);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20628);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26245);
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

/// Words ending -дуемый.
pub(crate) fn n3013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2906);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7059);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13596);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 13597);
					match s {
						[] => true,
						['с', 'с', 'и', s @ ..] => {
							super::push(out, 20629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13598);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20630);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 26246);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 20631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7060);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7061);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 7062);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зуемый.
pub(crate) fn n3015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2908);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7064);
			match s {
				[] => true,
				['к', 'а', s @ ..] => {
					super::push(out, 13600);
					match s {
						[] => true,
						['н', 'е', s @ ..] => {
							super::push(out, 20632);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26247);
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
			super::push(out, 7065);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 7066);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13601);
					match s {
						[] => true,
						['о', 'п', 'с', 'и', 'о', s @ ..] => {
							super::push(out, 20633);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26248);
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

/// Words ending -луемый.
pub(crate) fn n3017(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					super::push(out, 13602);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 13603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7067);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 13604);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нуемый.
pub(crate) fn n3019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2911);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 7069);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7070);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -руемый.
pub(crate) fn n3020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2912);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7072);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7073);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7074);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13608);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20634);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20636);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20637);
							match s {
								[] => true,
								['е', 'с', '\u{1}', s @ ..] => {
									super::push(out, 26249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 13609);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20638);
							s.is_empty()
						}
						['н', 'а', 'м', s @ ..] => {
							super::push(out, 20639);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20640);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20641);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26251);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 29792);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 26252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 13610);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 13611);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20642);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 26253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26254);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 26255);
									match s {
										[] => true,
										['и', 'т', s @ ..] => {
											super::push(out, 29793);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 31768);
													match s {
														[] => true,
														['г', s @ ..] => super::t7::n34426(s, out),
														_ => false,
													}
												}
												['п', s @ ..] => {
													super::push(out, 31769);
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
				['к', s @ ..] => {
					super::push(out, 13612);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20643);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26256);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20644);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20645);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26257);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 20646);
							match s {
								[] => true,
								['а', 'м', 'е', s @ ..] => {
									super::push(out, 26258);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29794);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 29795);
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
					super::push(out, 13613);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20647);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 26259);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 29796);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20648);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 26260);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 29797);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 20649);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20650);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26261);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29798);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 26262);
									match s {
										[] => true,
										['т', 'н', 'о', 'к', 'е', s @ ..] => {
											super::push(out, 29799);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => super::t7::n21692(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t7::n14284(s, out),
				['н', s @ ..] => super::t7::n14285(s, out),
				['п', s @ ..] => super::t7::n14286(s, out),
				['р', s @ ..] => {
					super::push(out, 13617);
					s.is_empty()
				}
				['с', s @ ..] => super::t7::n14288(s, out),
				['т', s @ ..] => super::t7::n14289(s, out),
				['ц', s @ ..] => super::t7::n14290(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t7::n7400(s, out),
		['т', s @ ..] => super::t7::n7401(s, out),
		['у', s @ ..] => super::t7::n7402(s, out),
		['ф', s @ ..] => super::t7::n7403(s, out),
		_ => false,
	}
}

/// Words ending -суемый.
pub(crate) fn n3021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2913);
	s.is_empty()
}

/// Words ending -туемый.
pub(crate) fn n3022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2914);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7079);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 7080);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 7081);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 7082);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20670);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26273);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7083);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 7084);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13626);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20672);
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

/// Words ending -фуемый.
pub(crate) fn n3023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2915);
	s.is_empty()
}

/// Words ending -хуемый.
pub(crate) fn n3024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2916);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7085);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7086);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13628);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цуемый.
pub(crate) fn n3025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2917);
	s.is_empty()
}

/// Words ending -чуемый.
pub(crate) fn n3026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2918);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7087);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 7088);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7089);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 7090);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13629);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шуемый.
pub(crate) fn n3027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2919);
	s.is_empty()
}

/// Words ending -чемый.
pub(crate) fn n1127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1096);
	s.is_empty()
}

/// Words ending -шемый.
pub(crate) fn n1128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1097);
	s.is_empty()
}

/// Words ending -щемый.
pub(crate) fn n1129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1098);
	s.is_empty()
}

/// Words ending -юемый.
pub(crate) fn n1130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1099);
	s.is_empty()
}

/// Words ending -яемый.
pub(crate) fn n1131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1100);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2920);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7091);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 20673);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 20674);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 29810);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 29811);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 29812);
													match s {
														[] => true,
														['о', s @ ..] => super::t7::n33339(s, out),
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
									super::push(out, 20675);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7092);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 7093);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13631);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13632);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 20676);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26274);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29813);
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
		['н', s @ ..] => {
			super::push(out, 2921);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7094);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['а', 'л', 'п', 'с', 'о', 'в', 'о', s @ ..] => {
									super::push(out, 20677);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 26275);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 29814);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 20678);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 20679);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 20680);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26276);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 29815);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 20681);
									match s {
										[] => true,
										['о', 'е', s @ ..] => {
											super::push(out, 26277);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 29816);
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
				['и', 'в', s @ ..] => {
					super::push(out, 7095);
					s.is_empty()
				}
				['л', 'о', 'п', 'с', s @ ..] => {
					super::push(out, 7096);
					match s {
						[] => true,
						['и', 'о', s @ ..] => {
							super::push(out, 13633);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 20682);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13634);
							s.is_empty()
						}
						['л', 'к', 'с', s @ ..] => {
							super::push(out, 13635);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20683);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 26278);
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
					super::push(out, 7097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2922);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7098);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							super::push(out, 13636);
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

/// Words ending -еимый.
pub(crate) fn n1136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1105);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2945);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жимый.
pub(crate) fn n1137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1106);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2946);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2947);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13657);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13659);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2948);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7127);
					match s {
						[] => true,
						['д', 'е', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20691);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 20692);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26279);
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
				['т', s @ ..] => {
					super::push(out, 7128);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2949);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7129);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 7130);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7131);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7132);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7133);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 13664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2950);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7134);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13665);
							match s {
								[] => true,
								['о', 'р', 'п', 'о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 26280);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 26281);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 13666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2951);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13667);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 13668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2952);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 2953);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7136);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зимый.
pub(crate) fn n1138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1107);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7137);
					match s {
						[] => true,
						['о', 'о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 20694);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26282);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 20695);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 26283);
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
				['т', s @ ..] => {
					super::push(out, 7138);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7139);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20696);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 20697);
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
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7140);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 7141);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 7142);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13669);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 20698);
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
			super::push(out, 2954);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 7143);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13670);
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

/// Words ending -лимый.
pub(crate) fn n1139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1108);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2955);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7144);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 7145);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 7146);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13671);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 7147);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7148);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 7149);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7150);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 7151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', 'у', 'г', s @ ..] => {
			super::push(out, 2956);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2957);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2958);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7153);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 7154);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13673);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20699);
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
			super::push(out, 2959);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 7156);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 7157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2960);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 7158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2961);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7159);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7160);
					match s {
						[] => true,
						['о', 'е', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20700);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 20701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 7161);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7162);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13674);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 20702);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 7163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2962);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7164);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 7165);
					match s {
						[] => true,
						['ч', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20703);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 20704);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7166);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 13676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2963);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7168);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7169);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 2964);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 2965);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мимый.
pub(crate) fn n1140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1109);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2966);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2967);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7171);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 7172);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7173);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2968);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7175);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нимый.
pub(crate) fn n1141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1110);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2969);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7176);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 7177);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 7178);
					s.is_empty()
				}
				['м', 'у', s @ ..] => {
					super::push(out, 7179);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13681);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 13682);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 13683);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13684);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 13685);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 20705);
									match s {
										[] => true,
										['у', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29817);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 29818);
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
							super::push(out, 13686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2970);
			match s {
				[] => true,
				['а', 'р', 'с', 'е', s @ ..] => {
					super::push(out, 7180);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2971);
			match s {
				[] => true,
				['а', 'с', '\u{1}', s @ ..] => {
					super::push(out, 7181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2972);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7182);
					match s {
						[] => true,
						['а', 'з', 'о', s @ ..] => {
							super::push(out, 13688);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 20706);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', 'е', 'щ', s @ ..] => {
							super::push(out, 13689);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 20707);
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
			super::push(out, 2973);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2974);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 7183);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2975);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13691);
							match s {
								[] => true,
								['и', 'о', s @ ..] => {
									super::push(out, 20708);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 26284);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'в', '\u{1}', s @ ..] => {
									super::push(out, 20709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 13692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2976);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7184);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 13693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7185);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2977);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7186);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2978);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2979);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оимый.
pub(crate) fn n1142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1111);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 2980);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2981);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2982);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7188);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7189);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13696);
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

/// Words ending -пимый.
pub(crate) fn n1143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1112);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2983);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2984);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7190);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -римый.
pub(crate) fn n1144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1113);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2985);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7191);
					match s {
						[] => true,
						['о', 'б', 'о', 'д', 'у', 'е', s @ ..] => {
							super::push(out, 13697);
							s.is_empty()
						}
						['с', 'о', 'б', 'о', 'д', 'у', 'е', 'н', '\u{1}', s @ ..] => {
							super::push(out, 13698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7192);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 7193);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 7194);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7195);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2986);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 2987);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2988);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13700);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 7197);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2989);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7198);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7199);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7201);
					match s {
						[] => true,
						['и', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20710);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 20711);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 7202);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13703);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2990);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7203);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13704);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 20712);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 20713);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 26285);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13705);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20714);
									s.is_empty()
								}
								['с', 'а', 'р', 'о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 26286);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 26287);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 29819);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', 'т', s @ ..] => {
											super::push(out, 26288);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 29820);
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
				['г', s @ ..] => {
					super::push(out, 7204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7205);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 7206);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13708);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 20715);
									match s {
										[] => true,
										['с', '\u{1}', s @ ..] => {
											super::push(out, 26289);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13709);
							match s {
								[] => true,
								['о', 'е', s @ ..] => {
									super::push(out, 20716);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 13710);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 20717);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7207);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7208);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13712);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 20718);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 13713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2991);
			match s {
				[] => true,
				['о', 'м', 'с', 'о', s @ ..] => {
					super::push(out, 7209);
					match s {
						[] => true,
						['б', 'о', 'д', 'у', 'е', s @ ..] => {
							super::push(out, 13714);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20719);
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
			super::push(out, 2992);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7210);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 7211);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 7212);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 7213);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7214);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 7215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2993);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 2994);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 7217);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 13715);
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

/// Words ending -симый.
pub(crate) fn n1145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1114);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2995);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7218);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 7219);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 7220);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7221);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2996);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2997);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7222);
					match s {
						[] => true,
						['а', 'з', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 20720);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 20721);
									match s {
										[] => true,
										['н', 'у', s @ ..] => {
											super::push(out, 26290);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 29821);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 20722);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 26291);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 26292);
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
			super::push(out, 2998);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7223);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7224);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13720);
							s.is_empty()
						}
						['з', 'и', s @ ..] => {
							super::push(out, 13721);
							match s {
								[] => true,
								['о', 'р', 'п', 'е', s @ ..] => {
									super::push(out, 20723);
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
			super::push(out, 2999);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 7225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3000);
			match s {
				[] => true,
				['о', 'ф', s @ ..] => {
					super::push(out, 7226);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тимый.
pub(crate) fn n1146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1115);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3001);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7227);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 13722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7228);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 7229);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7230);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13725);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 13726);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 13727);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20724);
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
			super::push(out, 3002);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3003);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7232);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 7233);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13729);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 20725);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 13730);
							s.is_empty()
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
				['е', s @ ..] => {
					super::push(out, 7234);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7235);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3004);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7236);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['в', 'о', 'с', '-', s @ ..] => {
									super::push(out, 20726);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26293);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 20727);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26294);
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
					super::push(out, 7237);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 13732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3005);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 3006);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7239);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3007);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					super::push(out, 7241);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чимый.
pub(crate) fn n1147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1116);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3008);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7242);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7243);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13734);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 26295);
											s.is_empty()
										}
										['щ', s @ ..] => {
											super::push(out, 26296);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 20728);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 26297);
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
					super::push(out, 7244);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 13735);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 13736);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 20729);
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
			super::push(out, 3009);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3010);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7245);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 7246);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13738);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 20730);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 13739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3011);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7247);
					match s {
						[] => true,
						['з', 'а', 'р', 'о', s @ ..] => {
							super::push(out, 13740);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 20731);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20732);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 20733);
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
			super::push(out, 3012);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3013);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 7248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3014);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7249);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 7250);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 7251);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13741);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13742);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 13743);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 20734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7252);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3015);
			match s {
				[] => true,
				['е', 'п', '\u{1}', s @ ..] => {
					super::push(out, 7254);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3016);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7256);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 7257);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 7258);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 3017);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3018);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7261);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13744);
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

/// Words ending -шимый.
pub(crate) fn n1148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1117);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3019);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7262);
					match s {
						[] => true,
						['з', 'а', 'р', '-', s @ ..] => {
							super::push(out, 13745);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20735);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7263);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3020);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3021);
			match s {
				[] => true,
				['р', 'е', s @ ..] => {
					super::push(out, 7265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3022);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13748);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13749);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13750);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 20736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', 'о', s @ ..] => {
							super::push(out, 13751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 3023);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7266);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щимый.
pub(crate) fn n1149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1118);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7267);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3024);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3025);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 3026);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -добный.
pub(crate) fn n3190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3072);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7311);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7312);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7313);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20747);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 26300);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 26301);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 29822);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 31780);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', 'п', s @ ..] => {
									super::push(out, 26302);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 26303);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29823);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 26304);
									match s {
										[] => true,
										['е', 'м', 'а', '\u{1}', s @ ..] => {
											super::push(out, 29824);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 26305);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 26306);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 26307);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 29825);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 31781);
													s.is_empty()
												}
												['м', s @ ..] => {
													super::push(out, 31782);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 29826);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 26308);
									match s {
										[] => true,
										['о', s @ ..] => {
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 31783);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 31784);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 29827);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 26309);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 29828);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 31785);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 31786);
													match s {
														[] => true,
														['е', s @ ..] => super::t7::n34431(s, out),
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
									super::push(out, 26310);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 29829);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 31787);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', 'в', 'е', 'н', '\u{1}', s @ ..] => {
											super::push(out, 29830);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 29831);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 26311);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 29832);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 31788);
													match s {
														[] => true,
														['л', s @ ..] => super::t7::n34432(s, out),
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
									super::push(out, 26312);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29833);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 31789);
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
			super::push(out, 7314);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дробный.
pub(crate) fn n7658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7319);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13781);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кробный.
pub(crate) fn n7659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7320);
	s.is_empty()
}

/// Words ending -пробный.
pub(crate) fn n7660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7321);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 13782);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тробный.
pub(crate) fn n7661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7322);
	match s {
		[] => true,
		['у', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20748);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20749);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20750);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 29834);
									s.is_empty()
								}
								['и', 'д', 'е', '\u{1}', s @ ..] => {
									super::push(out, 29835);
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

/// Words ending -эробный.
pub(crate) fn n7662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7323);
	s.is_empty()
}

/// Words ending -собный.
pub(crate) fn n3193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3075);
	match s {
		[] => true,
		['о', 'п', 'с', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13783);
					match s {
						[] => true,
						['ж', 'е', 'т', 'а', 'л', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26313);
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									super::push(out, 26314);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 20751);
							match s {
								[] => true,
								['з', 'и', 'ж', 'о', s @ ..] => {
									super::push(out, 26315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13784);
					match s {
						[] => true,
						['к', 'ж', 'е', 'л', 'е', 'н', '\u{1}', s @ ..] => {
							super::push(out, 20752);
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

/// Words ending -фобный.
pub(crate) fn n3194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3076);
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13785);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 13786);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 13787);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20754);
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

/// Words ending -щобный.
pub(crate) fn n3195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3077);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 7324);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13788);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбный.
pub(crate) fn n1177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1146);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3078);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3079);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -убный.
pub(crate) fn n1178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1147);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 3080);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7326);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7327);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7328);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7329);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3081);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 7330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3082);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7331);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20755);
									s.is_empty()
								}
								['х', 'е', s @ ..] => {
									super::push(out, 20756);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 29836);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t7::n33350(s, out),
														_ => false,
													}
												}
												['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 29837);
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
				['к', s @ ..] => {
					super::push(out, 7332);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13792);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3083);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7333);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7334);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 13795);
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

/// Words ending -ыбный.
pub(crate) fn n1179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1148);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7335);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7336);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13796);
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

/// Words ending -юбный.
pub(crate) fn n1180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1149);
	s.is_empty()
}

/// Words ending -дневный.
pub(crate) fn n7695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7350);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13820);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 13821);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 20760);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20761);
					match s {
						[] => true,
						['в', 'о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 26317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20762);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26318);
							match s {
								[] => true,
								['ц', 'д', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31791);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 31792);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 32809);
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
						['я', 'п', s @ ..] => {
							super::push(out, 26319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13822);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20763);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13823);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 13824);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 26321);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 26322);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 29838);
									match s {
										[] => true,
										['е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 31793);
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
					super::push(out, 20764);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -запевный.
pub(crate) fn n14512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13825);
	s.is_empty()
}

/// Words ending -ревный.
pub(crate) fn n3214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3095);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7352);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7353);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 7354);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13827);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 13828);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чевный.
pub(crate) fn n3215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3096);
	s.is_empty()
}

/// Words ending -шевный.
pub(crate) fn n3216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3097);
	s.is_empty()
}

/// Words ending -ивный.
pub(crate) fn n1183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1152);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3098);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7355);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 13829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3099);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7356);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13830);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 13831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 7357);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 20765);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 20766);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 26323);
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
			super::push(out, 3100);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7358);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7359);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 7360);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13834);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 26324);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 26325);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 26326);
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
					super::push(out, 7361);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7362);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13835);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20767);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 26327);
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
		['п', s @ ..] => {
			super::push(out, 3101);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 3102);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7363);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 7364);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7365);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13836);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 20768);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 20769);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 26328);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 20770);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 26329);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 20771);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 26330);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 29839);
													match s {
														[] => true,
														['р', 'о', s @ ..] => super::t7::n33354(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['д', s @ ..] => {
											super::push(out, 26331);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 29840);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', 'е', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													match s {
														[] => true,
														['и', s @ ..] => super::t7::n33355(s, out),
														['о', s @ ..] => super::t7::n33356(s, out),
														_ => false,
													}
												}
												['р', '-', s @ ..] => {
													super::push(out, 29841);
													match s {
														[] => true,
														['о', s @ ..] => super::t7::n33357(s, out),
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
									super::push(out, 20772);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 26332);
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
				['ь', 'л', 'у', 'п', s @ ..] => {
					super::push(out, 7366);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 20773);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 20774);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 26333);
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
		['т', s @ ..] => super::t7::n3222(s, out),
		['х', s @ ..] => {
			super::push(out, 3104);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 3105);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7376);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 13860);
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

/// Words ending -овный.
pub(crate) fn n1184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1153);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3106);
			match s {
				[] => true,
				['ю', 'л', '-', s @ ..] => {
					super::push(out, 7377);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3107);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13862);
							match s {
								[] => true,
								['ц', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26372);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 26373);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 29864);
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
							super::push(out, 13863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3108);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7379);
					s.is_empty()
				}
				['с', 'о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13864);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20812);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3109);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7380);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13866);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3110);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3111);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7381);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7382);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							super::push(out, 13867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7383);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 20813);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 26374);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29865);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 20814);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 26375);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 26376);
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
					super::push(out, 7384);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13868);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 20815);
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
		['т', s @ ..] => {
			super::push(out, 3112);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7385);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7386);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7387);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 3113);
			match s {
				[] => true,
				['у', 'д', '-', s @ ..] => {
					super::push(out, 7388);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13871);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20816);
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
			super::push(out, 3114);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7389);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рвный.
pub(crate) fn n1185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1154);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7390);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7391);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13872);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13873);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 20817);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7392);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -увный.
pub(crate) fn n1186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1155);
	s.is_empty()
}

/// Words ending -ывный.
pub(crate) fn n1187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1156);
	match s {
		[] => true,
		['з', 'т', s @ ..] => {
			super::push(out, 3115);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3116);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -явный.
pub(crate) fn n1188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1157);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3117);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7393);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word северо-западный.
pub(crate) fn n31341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29866);
	s.is_empty()
}

/// Words ending --северо-западный.
pub(crate) fn n31342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29867);
	s.is_empty()
}

/// Words ending -радный.
pub(crate) fn n3249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3129);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7413);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', 'о', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13888);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13889);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 13890);
					match s {
						[] => true,
						['р', 'е', s @ ..] => {
							super::push(out, 20824);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26380);
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
			super::push(out, 7414);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 13891);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7415);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7416);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13892);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13893);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 20826);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 20827);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26381);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 29868);
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

/// The word чадный.
pub(crate) fn n7769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7419);
	s.is_empty()
}

/// Words ending -шадный.
pub(crate) fn n3253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3133);
	match s {
		[] => true,
		['о', 'л', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13896);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13897);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щадный.
pub(crate) fn n3254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3134);
	s.is_empty()
}

/// Words ending -едный.
pub(crate) fn n1191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1160);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3135);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7420);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7421);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7422);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13898);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 13899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 7423);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3136);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7424);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7425);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7426);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13902);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 13903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3137);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3138);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7427);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13904);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 13905);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20832);
									match s {
										[] => true,
										['т', 'л', 'е', 'ж', '\u{1}', s @ ..] => {
											super::push(out, 26382);
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
					super::push(out, 7428);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13906);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 20833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7429);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3139);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7431);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13908);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20834);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13909);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 20835);
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
			super::push(out, 3140);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3141);
			match s {
				[] => true,
				['и', 'с', 'о', 'л', 'е', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13910);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13911);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 20836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7432);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							super::push(out, 13912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'о', 'т', 'о', s @ ..] => {
					super::push(out, 7433);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3142);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7434);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 7435);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13915);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 13916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 7436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13917);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13918);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13919);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7437);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -здный.
pub(crate) fn n1192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1161);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3143);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7439);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20837);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 20838);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 20839);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 26383);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ж', s @ ..] => {
									super::push(out, 20840);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26384);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 20841);
									match s {
										[] => true,
										['е', 'б', '\u{1}', s @ ..] => {
											super::push(out, 26385);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 26386);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 26387);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 29869);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 26388);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 26389);
											s.is_empty()
										}
										['н', s @ ..] => {
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29870);
													match s {
														[] => true,
														['у', s @ ..] => super::t7::n33372(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 29871);
													match s {
														[] => true,
														['а', 'р', 'к', '\u{1}', s @ ..] => super::t7::n33373(s, out),
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
									super::push(out, 20842);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 20843);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26390);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 29872);
													match s {
														[] => true,
														['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => super::t7::n33374(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 26391);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 29873);
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
					super::push(out, 7440);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 7441);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7442);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 7443);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13923);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13924);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 20844);
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

/// Words ending -идный.
pub(crate) fn n1193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1162);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3144);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 3145);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7444);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7445);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7446);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 13925);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 20845);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26392);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 13926);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 20846);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 26393);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 13927);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 13928);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 13929);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 13930);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 20847);
									match s {
										[] => true,
										['р', 'е', 'с', '\u{1}', s @ ..] => {
											super::push(out, 26394);
											s.is_empty()
										}
										_ => false,
									}
								}
								['й', s @ ..] => {
									super::push(out, 20848);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 26395);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 20849);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26396);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ь', s @ ..] => {
									super::push(out, 20850);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 26397);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 13931);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 13932);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 20851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 13933);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 20852);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 26398);
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
					super::push(out, 7447);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7448);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13934);
							match s {
								[] => true,
								['е', 'м', 'а', '\u{1}', s @ ..] => {
									super::push(out, 20853);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 20854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 13935);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 13936);
							match s {
								[] => true,
								['а', 'з', s @ ..] => {
									super::push(out, 20855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 13937);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 20856);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26399);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 13938);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20857);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 20858);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26400);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 29874);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 20859);
									match s {
										[] => true,
										['н', s @ ..] => super::t7::n27706(s, out),
										['т', s @ ..] => super::t7::n27707(s, out),
										_ => false,
									}
								}
								['л', s @ ..] => super::t7::n21918(s, out),
								['о', s @ ..] => super::t7::n21919(s, out),
								['т', s @ ..] => super::t7::n21920(s, out),
								['ч', s @ ..] => super::t7::n21921(s, out),
								['ш', s @ ..] => super::t7::n21922(s, out),
								_ => false,
							}
						}
						['л', s @ ..] => super::t7::n14636(s, out),
						['м', 'е', 'з', 'о', 'н', 'р', 'е', 'ч', '\u{1}', s @ ..] => super::t7::n14637(s, out),
						['н', s @ ..] => super::t7::n14638(s, out),
						['р', s @ ..] => super::t7::n14639(s, out),
						['с', s @ ..] => super::t7::n14640(s, out),
						['т', s @ ..] => super::t7::n14641(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t7::n3267(s, out),
		['д', s @ ..] => super::t7::n3268(s, out),
		['е', s @ ..] => super::t7::n3269(s, out),
		['з', s @ ..] => super::t7::n3270(s, out),
		['л', s @ ..] => super::t7::n3271(s, out),
		['м', s @ ..] => super::t7::n3272(s, out),
		['н', s @ ..] => super::t7::n3273(s, out),
		['о', s @ ..] => super::t7::n3274(s, out),
		['п', s @ ..] => super::t7::n3275(s, out),
		['р', s @ ..] => super::t7::n3276(s, out),
		['с', s @ ..] => super::t7::n3277(s, out),
		['т', s @ ..] => super::t7::n3278(s, out),
		['ф', s @ ..] => super::t7::n3279(s, out),
		['х', s @ ..] => super::t7::n3280(s, out),
		['ц', s @ ..] => super::t7::n3281(s, out),
		_ => false,
	}
}

/// Words ending -ндный.
pub(crate) fn n1194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1163);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3161);
			match s {
				[] => true,
				['б', 'а', 'р', 'т', 'н', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13980);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13981);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13982);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 20897);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 7483);
					match s {
						[] => true,
						['а', 'л', '\u{1}', s @ ..] => {
							super::push(out, 13983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 7484);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 13984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 7485);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7486);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13986);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3162);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3163);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7488);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13987);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 13988);
							match s {
								[] => true,
								['г', 'р', s @ ..] => {
									super::push(out, 20898);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26424);
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
					super::push(out, 7489);
					match s {
						[] => true,
						['е', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20899);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 20900);
									s.is_empty()
								}
								['е', 'ж', s @ ..] => {
									super::push(out, 20901);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26425);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 26426);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 29881);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 26427);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 20902);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 20903);
									match s {
										[] => true,
										['е', 'р', s @ ..] => {
											match s {
												[] => true,
												['т', '\u{1}', s @ ..] => {
													super::push(out, 29882);
													s.is_empty()
												}
												['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 29883);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ю', s @ ..] => {
									super::push(out, 20904);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 26428);
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
			super::push(out, 3164);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7490);
					match s {
						[] => true,
						['р', 'и', 'г', '\u{1}', s @ ..] => {
							super::push(out, 13989);
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

/// Words ending -одный.
pub(crate) fn n1195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1164);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3165);
			match s {
				[] => true,
				['о', 'в', 'с', 'е', s @ ..] => {
					super::push(out, 7491);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3166);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7492);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7493);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7494);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13991);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 13992);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 20905);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13993);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20906);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13994);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 20907);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 13995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7495);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13996);
							match s {
								[] => true,
								['р', 'о', s @ ..] => {
									super::push(out, 20908);
									match s {
										[] => true,
										['к', 'а', 'р', 'б', '\u{1}', s @ ..] => {
											super::push(out, 26429);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							super::push(out, 13997);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13998);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7496);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14000);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 20909);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7497);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14001);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 20910);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 14002);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 14003);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 14004);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 14005);
							match s {
								[] => true,
								['м', 'е', 'з', s @ ..] => {
									super::push(out, 20911);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14006);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 29884);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 29885);
													match s {
														[] => true,
														['ф', s @ ..] => super::t7::n33376(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 29886);
													match s {
														[] => true,
														['о', s @ ..] => super::t7::n33377(s, out),
														_ => false,
													}
												}
												['л', s @ ..] => {
													super::push(out, 29887);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 29888);
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
						['т', s @ ..] => {
							super::push(out, 14007);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20912);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 26430);
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
					super::push(out, 7498);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7499);
					match s {
						[] => true,
						['о', 'о', 'р', s @ ..] => {
							super::push(out, 14008);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 20913);
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
			super::push(out, 3167);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7500);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7501);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 14009);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20914);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 14010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7502);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7503);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7504);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7505);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14013);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 20915);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 26431);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 29889);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 20916);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 7506);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14014);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 20917);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 26432);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 20918);
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
			super::push(out, 3168);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7507);
					s.is_empty()
				}
				['р', s @ ..] => super::t7::n7863(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t7::n3290(s, out),
		['л', s @ ..] => super::t7::n3291(s, out),
		['м', s @ ..] => super::t7::n3292(s, out),
		['н', s @ ..] => super::t7::n3293(s, out),
		['п', s @ ..] => super::t7::n3294(s, out),
		['р', s @ ..] => super::t7::n3295(s, out),
		['т', s @ ..] => super::t7::n3296(s, out),
		['х', s @ ..] => super::t7::n3297(s, out),
		_ => false,
	}
}

/// Words ending -рдный.
pub(crate) fn n1196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1165);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3177);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7529);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 7530);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14043);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 7531);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 7532);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 14044);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 20954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3178);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 3179);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 3180);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удный.
pub(crate) fn n1197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1166);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3181);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 3182);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14045);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3183);
			match s {
				[] => true,
				['б', 'и', s @ ..] => {
					super::push(out, 7534);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3184);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7535);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7536);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3185);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7537);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14048);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 14049);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14050);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 20955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 7538);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7539);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7540);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14051);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 14052);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20956);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 26459);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14053);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 14054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3186);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7541);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7542);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14055);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 14056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14057);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3187);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 3188);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7544);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 3189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыдный.
pub(crate) fn n1198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1167);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 3190);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14060);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14061);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 14062);
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

/// Words ending -ьдный.
pub(crate) fn n1199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1168);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3191);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7545);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдный.
pub(crate) fn n1200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1169);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3192);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3193);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7547);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 14063);
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

/// Words ending -ядный.
pub(crate) fn n1201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1170);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3194);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3195);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7548);
					match s {
						[] => true,
						['о', 'з', s @ ..] => {
							super::push(out, 14064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 7549);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 14065);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 20957);
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
			super::push(out, 3196);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3197);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7550);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14066);
							match s {
								[] => true,
								['о', 'н', s @ ..] => {
									super::push(out, 20958);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 26460);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 29904);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 20959);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 14067);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 20960);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 26461);
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
					super::push(out, 7551);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20961);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 26462);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 29905);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', 'е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 26463);
											s.is_empty()
										}
										['ы', 'т', 'е', 'ч', 'и', 'т', 'я', 'с', 'е', 'д', 'и', 'т', 'с', 'е', 'ш', '\u{1}', s @ ..] => {
											super::push(out, 26464);
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
				['т', s @ ..] => {
					super::push(out, 7552);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 7553);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мленый.
pub(crate) fn n3337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3215);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7573);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 14088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7574);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14089);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оленый.
pub(crate) fn n3338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3216);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 7575);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14090);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 14091);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20966);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29906);
											s.is_empty()
										}
										['ь', s @ ..] => {
											super::push(out, 29907);
											match s {
												[] => true,
												['р', 'о', 'г', '\u{1}', s @ ..] => {
													super::push(out, 31820);
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
					super::push(out, 14092);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20967);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 7576);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14093);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пленый.
pub(crate) fn n3339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3217);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7577);
			match s {
				[] => true,
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20968);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7578);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фленый.
pub(crate) fn n3341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3219);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7580);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шленый.
pub(crate) fn n3342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3220);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 7582);
			match s {
				[] => true,
				['м', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20970);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20971);
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

/// Words ending -яленый.
pub(crate) fn n3343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3221);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7583);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14095);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -меный.
pub(crate) fn n1207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1176);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 3222);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -неный.
pub(crate) fn n1208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1177);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3223);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14096);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 14097);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3224);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3225);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оеный.
pub(crate) fn n1209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1178);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3226);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7586);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3227);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -реный.
pub(crate) fn n1210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1179);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3228);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7588);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7589);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 7590);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 7591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3229);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3230);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 7593);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 14099);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 14100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 7594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3231);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3232);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3233);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 3234);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7598);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -теный.
pub(crate) fn n1211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1180);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3235);
			match s {
				[] => true,
				['л', 'п', '\u{1}', s @ ..] => {
					super::push(out, 7599);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ченый.
pub(crate) fn n1212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1181);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3236);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3237);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7600);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 7601);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3238);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3239);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7602);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 14103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3240);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7603);
					s.is_empty()
				}
				['р', 'т', s @ ..] => {
					super::push(out, 7604);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', '\u{1}', s @ ..] => {
					super::push(out, 7605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3241);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 14105);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20972);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 14106);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 20973);
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
			super::push(out, 3242);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7606);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7607);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7608);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3243);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7609);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7610);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 7611);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7612);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 20974);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 20975);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 26467);
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
					super::push(out, 7613);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 7614);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 14109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3244);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7615);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шеный.
pub(crate) fn n1213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1182);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3245);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7616);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 14110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 7617);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7618);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 14112);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 20976);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 26468);
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
			super::push(out, 3246);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3247);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7620);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7621);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3248);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7623);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7624);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щеный.
pub(crate) fn n1214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1183);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3249);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3250);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7625);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3251);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 7626);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14114);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3252);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7627);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 7628);
					s.is_empty()
				}
				['м', 'е', s @ ..] => {
					super::push(out, 7629);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7630);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3253);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7631);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ражный.
pub(crate) fn n3385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3263);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7649);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14133);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7650);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7651);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 7652);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14134);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 14135);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14136);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20987);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26471);
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
			super::push(out, 7653);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 14137);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20988);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26472);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29908);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 29909);
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

/// Words ending -сажный.
pub(crate) fn n3386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3264);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7654);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 14138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7655);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20989);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 20990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'а', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 14139);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 14140);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тажный.
pub(crate) fn n3387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3265);
	match s {
		[] => true,
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14141);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20991);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 20992);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26473);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14142);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14144);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 14145);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14146);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14147);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14148);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 20996);
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26474);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 29910);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 26475);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29911);
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
					super::push(out, 14149);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 14150);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20997);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 29912);
											s.is_empty()
										}
										['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 29913);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 20998);
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

/// Words ending -джный.
pub(crate) fn n1216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1185);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3267);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3268);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7656);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ежный.
pub(crate) fn n1217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1186);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3269);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 3270);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7657);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 7658);
					match s {
						[] => true,
						['р', 'а', s @ ..] => {
							super::push(out, 14151);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 26476);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 26477);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 29914);
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
		['д', s @ ..] => {
			super::push(out, 3271);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7659);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 20999);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21000);
									match s {
										[] => true,
										['г', 'а', 'л', 'б', 'е', s @ ..] => {
											super::push(out, 26478);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21001);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 26479);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 21002);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 26480);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 14152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7660);
					match s {
						[] => true,
						['л', 'о', 'м', '-', s @ ..] => {
							super::push(out, 14153);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21003);
									match s {
										[] => true,
										['к', 'с', 'ь', 'л', 'о', 'м', 'о', 'с', 'м', 'о', 'к', '\u{1}', s @ ..] => {
											super::push(out, 26481);
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
			super::push(out, 3272);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7661);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14154);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21004);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 21005);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 26482);
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
					super::push(out, 7662);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 7663);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 7664);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 7665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3273);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3274);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7666);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7667);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 14155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7668);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14156);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 21006);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 21007);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 21008);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 21009);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 26483);
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
					super::push(out, 7669);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3275);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7670);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14158);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 14159);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 14160);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7671);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7672);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21011);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21012);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 26484);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 21013);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21014);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 26485);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 29915);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 29916);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 26486);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 26487);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 21015);
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
		['т', s @ ..] => {
			super::push(out, 3276);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7673);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 7674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 3277);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7675);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 7676);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ижный.
pub(crate) fn n1218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1187);
	s.is_empty()
}

/// Words ending -лжный.
pub(crate) fn n1219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1188);
	s.is_empty()
}

